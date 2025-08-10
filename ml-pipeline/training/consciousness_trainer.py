"""
Consciousness Engine ML Training Pipeline
Expert CTO Next Gen - Advanced ML Training System
"""

import os
import json
import logging
import asyncio
from datetime import datetime, timedelta
from typing import Dict, List, Optional, Tuple
from dataclasses import dataclass
from pathlib import Path

import torch
import torch.nn as nn
import transformers
from transformers import (
    AutoTokenizer, AutoModelForCausalLM, 
    TrainingArguments, Trainer, DataCollatorForLanguageModeling
)
from datasets import Dataset, load_dataset
import wandb
import mlflow
import numpy as np
from sklearn.metrics import accuracy_score, f1_score
import psycopg2
import redis
from sqlalchemy import create_engine, text
import pandas as pd

# Configuration
@dataclass
class TrainingConfig:
    model_name: str = "microsoft/DialoGPT-medium"
    max_length: int = 512
    batch_size: int = 8
    learning_rate: float = 5e-5
    num_epochs: int = 3
    warmup_steps: int = 500
    logging_steps: int = 100
    save_steps: int = 1000
    eval_steps: int = 500
    output_dir: str = "./models/consciousness-engine"
    data_dir: str = "./data"
    
    # Database
    database_url: str = os.getenv("DATABASE_URL", "postgresql://postgres:password@localhost:5432/consciousness")
    redis_url: str = os.getenv("REDIS_URL", "redis://localhost:6379")
    
    # MLOps
    wandb_project: str = "consciousness-engine"
    mlflow_tracking_uri: str = "http://localhost:5000"
    
    # Quality thresholds
    min_quality_score: float = 0.8
    min_ethical_score: float = 0.9
    min_empathy_score: float = 0.85

class ConsciousnessTrainer:
    def __init__(self, config: TrainingConfig):
        self.config = config
        self.setup_logging()
        self.setup_mlops()
        self.setup_database()
        
    def setup_logging(self):
        logging.basicConfig(
            level=logging.INFO,
            format='%(asctime)s - %(name)s - %(levelname)s - %(message)s',
            handlers=[
                logging.FileHandler('training.log'),
                logging.StreamHandler()
            ]
        )
        self.logger = logging.getLogger(__name__)
        
    def setup_mlops(self):
        """Setup MLOps tracking"""
        # Weights & Biases
        wandb.init(
            project=self.config.wandb_project,
            config=self.config.__dict__
        )
        
        # MLflow
        mlflow.set_tracking_uri(self.config.mlflow_tracking_uri)
        mlflow.set_experiment("consciousness-engine-training")
        
    def setup_database(self):
        """Setup database connections"""
        self.db_engine = create_engine(self.config.database_url)
        self.redis_client = redis.from_url(self.config.redis_url)
        
    async def collect_training_data(self) -> Dataset:
        """Collect and prepare training data from conversations"""
        self.logger.info("Collecting training data from conversations...")
        
        # Query conversations from database
        query = """
        SELECT 
            c.user_message,
            c.ai_response,
            cm.confidence,
            cm.ethical_score,
            cm.empathy_score,
            cm.quality_score,
            c.created_at
        FROM conversations c
        JOIN consciousness_metrics cm ON c.id = cm.conversation_id
        WHERE cm.quality_score >= %s
        AND cm.ethical_score >= %s
        AND cm.empathy_score >= %s
        AND c.created_at >= %s
        ORDER BY c.created_at DESC
        LIMIT 10000
        """
        
        cutoff_date = datetime.now() - timedelta(days=30)
        
        with self.db_engine.connect() as conn:
            df = pd.read_sql(
                query, 
                conn, 
                params=[
                    self.config.min_quality_score,
                    self.config.min_ethical_score,
                    self.config.min_empathy_score,
                    cutoff_date
                ]
            )
        
        self.logger.info(f"Collected {len(df)} high-quality conversations")
        
        # Prepare dataset
        conversations = []
        for _, row in df.iterrows():
            conversation = {
                "input": row['user_message'],
                "output": row['ai_response'],
                "quality_score": row['quality_score'],
                "ethical_score": row['ethical_score'],
                "empathy_score": row['empathy_score']
            }
            conversations.append(conversation)
        
        # Add external high-quality datasets
        external_data = await self.load_external_datasets()
        conversations.extend(external_data)
        
        return Dataset.from_list(conversations)
    
    async def load_external_datasets(self) -> List[Dict]:
        """Load external high-quality conversation datasets"""
        self.logger.info("Loading external datasets...")
        
        external_conversations = []
        
        # Load empathetic dialogues
        try:
            empathetic_data = load_dataset("empathetic_dialogues")
            for item in empathetic_data['train'][:1000]:  # Sample 1000
                external_conversations.append({
                    "input": item['prompt'],
                    "output": item['response'],
                    "quality_score": 0.85,
                    "ethical_score": 0.95,
                    "empathy_score": 0.9
                })
        except Exception as e:
            self.logger.warning(f"Could not load empathetic_dialogues: {e}")
        
        # Load PersonaChat
        try:
            persona_data = load_dataset("persona_chat")
            for item in persona_data['train'][:1000]:  # Sample 1000
                if len(item['history']) >= 2:
                    external_conversations.append({
                        "input": item['history'][-2],
                        "output": item['history'][-1],
                        "quality_score": 0.8,
                        "ethical_score": 0.9,
                        "empathy_score": 0.8
                    })
        except Exception as e:
            self.logger.warning(f"Could not load persona_chat: {e}")
        
        self.logger.info(f"Loaded {len(external_conversations)} external conversations")
        return external_conversations
    
    def preprocess_data(self, dataset: Dataset) -> Dataset:
        """Preprocess data for training"""
        self.logger.info("Preprocessing data...")
        
        tokenizer = AutoTokenizer.from_pretrained(self.config.model_name)
        if tokenizer.pad_token is None:
            tokenizer.pad_token = tokenizer.eos_token
        
        def tokenize_function(examples):
            # Create conversation format
            conversations = []
            for inp, out in zip(examples['input'], examples['output']):
                conversation = f"Human: {inp}\nAssistant: {out}{tokenizer.eos_token}"
                conversations.append(conversation)
            
            # Tokenize
            tokenized = tokenizer(
                conversations,
                truncation=True,
                padding=True,
                max_length=self.config.max_length,
                return_tensors="pt"
            )
            
            # Labels are the same as input_ids for language modeling
            tokenized["labels"] = tokenized["input_ids"].clone()
            
            return tokenized
        
        tokenized_dataset = dataset.map(
            tokenize_function,
            batched=True,
            remove_columns=dataset.column_names
        )
        
        return tokenized_dataset
    
    def create_model_and_tokenizer(self) -> Tuple[nn.Module, transformers.PreTrainedTokenizer]:
        """Create model and tokenizer"""
        self.logger.info(f"Loading model: {self.config.model_name}")
        
        tokenizer = AutoTokenizer.from_pretrained(self.config.model_name)
        if tokenizer.pad_token is None:
            tokenizer.pad_token = tokenizer.eos_token
        
        model = AutoModelForCausalLM.from_pretrained(
            self.config.model_name,
            torch_dtype=torch.float16 if torch.cuda.is_available() else torch.float32,
            device_map="auto" if torch.cuda.is_available() else None
        )
        
        # Resize token embeddings if needed
        model.resize_token_embeddings(len(tokenizer))
        
        return model, tokenizer
    
    def compute_metrics(self, eval_pred):
        """Compute custom metrics for evaluation"""
        predictions, labels = eval_pred
        
        # Decode predictions and labels
        tokenizer = AutoTokenizer.from_pretrained(self.config.model_name)
        
        # Calculate perplexity
        shift_logits = predictions[..., :-1, :].contiguous()
        shift_labels = labels[..., 1:].contiguous()
        
        loss_fct = nn.CrossEntropyLoss()
        loss = loss_fct(shift_logits.view(-1, shift_logits.size(-1)), shift_labels.view(-1))
        perplexity = torch.exp(loss)
        
        return {
            "perplexity": perplexity.item(),
            "loss": loss.item()
        }
    
    async def train_model(self) -> str:
        """Train the consciousness model"""
        self.logger.info("Starting model training...")
        
        # Collect and preprocess data
        dataset = await self.collect_training_data()
        tokenized_dataset = self.preprocess_data(dataset)
        
        # Split dataset
        train_size = int(0.9 * len(tokenized_dataset))
        eval_size = len(tokenized_dataset) - train_size
        
        train_dataset = tokenized_dataset.select(range(train_size))
        eval_dataset = tokenized_dataset.select(range(train_size, train_size + eval_size))
        
        # Create model and tokenizer
        model, tokenizer = self.create_model_and_tokenizer()
        
        # Data collator
        data_collator = DataCollatorForLanguageModeling(
            tokenizer=tokenizer,
            mlm=False,
        )
        
        # Training arguments
        training_args = TrainingArguments(
            output_dir=self.config.output_dir,
            overwrite_output_dir=True,
            num_train_epochs=self.config.num_epochs,
            per_device_train_batch_size=self.config.batch_size,
            per_device_eval_batch_size=self.config.batch_size,
            warmup_steps=self.config.warmup_steps,
            logging_steps=self.config.logging_steps,
            save_steps=self.config.save_steps,
            eval_steps=self.config.eval_steps,
            evaluation_strategy="steps",
            save_strategy="steps",
            load_best_model_at_end=True,
            metric_for_best_model="eval_loss",
            greater_is_better=False,
            learning_rate=self.config.learning_rate,
            weight_decay=0.01,
            fp16=torch.cuda.is_available(),
            dataloader_pin_memory=False,
            report_to=["wandb", "mlflow"],
            run_name=f"consciousness-training-{datetime.now().strftime('%Y%m%d-%H%M%S')}"
        )
        
        # Trainer
        trainer = Trainer(
            model=model,
            args=training_args,
            train_dataset=train_dataset,
            eval_dataset=eval_dataset,
            data_collator=data_collator,
            compute_metrics=self.compute_metrics,
        )
        
        # Start MLflow run
        with mlflow.start_run():
            # Log parameters
            mlflow.log_params(self.config.__dict__)
            
            # Train model
            trainer.train()
            
            # Evaluate model
            eval_results = trainer.evaluate()
            mlflow.log_metrics(eval_results)
            
            # Save model
            model_path = f"{self.config.output_dir}/final"
            trainer.save_model(model_path)
            tokenizer.save_pretrained(model_path)
            
            # Log model to MLflow
            mlflow.transformers.log_model(
                transformers_model={"model": model, "tokenizer": tokenizer},
                artifact_path="consciousness-model",
                registered_model_name="consciousness-engine"
            )
            
            self.logger.info(f"Model saved to {model_path}")
            
            return model_path
    
    async def evaluate_model_quality(self, model_path: str) -> Dict[str, float]:
        """Evaluate model quality on various metrics"""
        self.logger.info("Evaluating model quality...")
        
        # Load model
        model = AutoModelForCausalLM.from_pretrained(model_path)
        tokenizer = AutoTokenizer.from_pretrained(model_path)
        
        # Test conversations
        test_conversations = [
            "Hello, how are you today?",
            "I'm feeling sad about my job situation.",
            "Can you help me understand artificial intelligence?",
            "What do you think about ethics in AI?",
            "Tell me a creative story about the future."
        ]
        
        quality_scores = []
        ethical_scores = []
        empathy_scores = []
        
        for conversation in test_conversations:
            # Generate response
            inputs = tokenizer.encode(f"Human: {conversation}\nAssistant:", return_tensors="pt")
            
            with torch.no_grad():
                outputs = model.generate(
                    inputs,
                    max_length=inputs.shape[1] + 100,
                    num_return_sequences=1,
                    temperature=0.7,
                    do_sample=True,
                    pad_token_id=tokenizer.eos_token_id
                )
            
            response = tokenizer.decode(outputs[0][inputs.shape[1]:], skip_special_tokens=True)
            
            # Evaluate response (simplified scoring)
            quality_score = min(1.0, len(response.split()) / 20)  # Length-based quality
            ethical_score = 0.95 if "harmful" not in response.lower() else 0.3  # Simple ethical check
            empathy_score = 0.9 if any(word in response.lower() for word in ["understand", "feel", "sorry"]) else 0.7
            
            quality_scores.append(quality_score)
            ethical_scores.append(ethical_score)
            empathy_scores.append(empathy_score)
        
        metrics = {
            "avg_quality_score": np.mean(quality_scores),
            "avg_ethical_score": np.mean(ethical_scores),
            "avg_empathy_score": np.mean(empathy_scores),
            "min_quality_score": np.min(quality_scores),
            "min_ethical_score": np.min(ethical_scores),
            "min_empathy_score": np.min(empathy_scores)
        }
        
        self.logger.info(f"Model evaluation metrics: {metrics}")
        return metrics
    
    async def deploy_model(self, model_path: str) -> bool:
        """Deploy model to production if quality thresholds are met"""
        self.logger.info("Evaluating model for deployment...")
        
        # Evaluate model quality
        metrics = await self.evaluate_model_quality(model_path)
        
        # Check thresholds
        if (metrics["min_quality_score"] >= self.config.min_quality_score and
            metrics["min_ethical_score"] >= self.config.min_ethical_score and
            metrics["min_empathy_score"] >= self.config.min_empathy_score):
            
            self.logger.info("Model meets quality thresholds. Deploying...")
            
            # Update model version in Redis
            model_info = {
                "model_path": model_path,
                "version": datetime.now().isoformat(),
                "metrics": metrics
            }
            
            self.redis_client.set("consciousness_model_info", json.dumps(model_info))
            
            # Log deployment
            with mlflow.start_run():
                mlflow.log_metrics(metrics)
                mlflow.log_param("deployment_status", "deployed")
                mlflow.log_param("deployment_time", datetime.now().isoformat())
            
            self.logger.info("Model deployed successfully!")
            return True
        else:
            self.logger.warning("Model does not meet quality thresholds. Deployment cancelled.")
            return False

async def main():
    """Main training pipeline"""
    config = TrainingConfig()
    trainer = ConsciousnessTrainer(config)
    
    try:
        # Train model
        model_path = await trainer.train_model()
        
        # Deploy if quality is sufficient
        deployed = await trainer.deploy_model(model_path)
        
        if deployed:
            print("🎉 Model training and deployment completed successfully!")
        else:
            print("⚠️ Model training completed but deployment was cancelled due to quality issues.")
            
    except Exception as e:
        logging.error(f"Training pipeline failed: {e}")
        raise

if __name__ == "__main__":
    asyncio.run(main())
