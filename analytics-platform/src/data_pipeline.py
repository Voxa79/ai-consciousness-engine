"""
Advanced Analytics and BI Platform for Consciousness Engine
Expert CTO Next Gen - Real-time Analytics Pipeline
"""

import asyncio
import json
import logging
from datetime import datetime, timedelta
from typing import Dict, List, Optional, Any
from dataclasses import dataclass, asdict
from pathlib import Path
import os

import pandas as pd
import numpy as np
from sqlalchemy import create_engine, text
import redis
import clickhouse_connect
from kafka import KafkaProducer, KafkaConsumer
import apache_beam as beam
from apache_beam.options.pipeline_options import PipelineOptions
import plotly.graph_objects as go
import plotly.express as px
from plotly.subplots import make_subplots
import dash
from dash import dcc, html, Input, Output, callback
import dash_bootstrap_components as dbc

# ML Libraries
from sklearn.cluster import KMeans
from sklearn.preprocessing import StandardScaler
from sklearn.decomposition import PCA
from sklearn.ensemble import IsolationForest
import tensorflow as tf
from transformers import pipeline

# Configuration
@dataclass
class AnalyticsConfig:
    # Databases
    postgres_url: str = os.getenv("DATABASE_URL", "postgresql://postgres:password@localhost:5432/consciousness")
    clickhouse_host: str = os.getenv("CLICKHOUSE_HOST", "localhost")
    clickhouse_port: int = int(os.getenv("CLICKHOUSE_PORT", "9000"))
    redis_url: str = os.getenv("REDIS_URL", "redis://localhost:6379")
    
    # Kafka
    kafka_bootstrap_servers: str = os.getenv("KAFKA_BOOTSTRAP_SERVERS", "localhost:9092")
    
    # Analytics
    batch_size: int = 1000
    window_size_minutes: int = 5
    anomaly_threshold: float = 0.1
    
    # ML Models
    sentiment_model: str = "cardiffnlp/twitter-roberta-base-sentiment-latest"
    emotion_model: str = "j-hartmann/emotion-english-distilroberta-base"

class ConsciousnessAnalytics:
    def __init__(self, config: AnalyticsConfig):
        self.config = config
        self.setup_logging()
        self.setup_connections()
        self.setup_ml_models()
        
    def setup_logging(self):
        logging.basicConfig(
            level=logging.INFO,
            format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
        )
        self.logger = logging.getLogger(__name__)
        
    def setup_connections(self):
        """Setup database and messaging connections"""
        # PostgreSQL
        self.pg_engine = create_engine(self.config.postgres_url)
        
        # ClickHouse for analytics
        self.ch_client = clickhouse_connect.get_client(
            host=self.config.clickhouse_host,
            port=self.config.clickhouse_port
        )
        
        # Redis for caching
        self.redis_client = redis.from_url(self.config.redis_url)
        
        # Kafka for streaming
        self.kafka_producer = KafkaProducer(
            bootstrap_servers=[self.config.kafka_bootstrap_servers],
            value_serializer=lambda x: json.dumps(x).encode('utf-8')
        )
        
    def setup_ml_models(self):
        """Setup ML models for analysis"""
        self.sentiment_analyzer = pipeline(
            "sentiment-analysis",
            model=self.config.sentiment_model,
            return_all_scores=True
        )
        
        self.emotion_analyzer = pipeline(
            "text-classification",
            model=self.config.emotion_model,
            return_all_scores=True
        )
        
    async def process_conversation_stream(self):
        """Process real-time conversation stream"""
        consumer = KafkaConsumer(
            'consciousness-conversations',
            bootstrap_servers=[self.config.kafka_bootstrap_servers],
            value_deserializer=lambda m: json.loads(m.decode('utf-8'))
        )
        
        batch = []
        
        for message in consumer:
            conversation = message.value
            
            # Enrich conversation with analytics
            enriched = await self.enrich_conversation(conversation)
            batch.append(enriched)
            
            # Process batch
            if len(batch) >= self.config.batch_size:
                await self.process_batch(batch)
                batch = []
                
    async def enrich_conversation(self, conversation: Dict) -> Dict:
        """Enrich conversation with analytics data"""
        user_message = conversation.get('user_message', '')
        ai_response = conversation.get('ai_response', '')
        
        # Sentiment analysis
        user_sentiment = self.analyze_sentiment(user_message)
        ai_sentiment = self.analyze_sentiment(ai_response)
        
        # Emotion analysis
        user_emotions = self.analyze_emotions(user_message)
        ai_emotions = self.analyze_emotions(ai_response)
        
        # Text metrics
        user_metrics = self.calculate_text_metrics(user_message)
        ai_metrics = self.calculate_text_metrics(ai_response)
        
        # Conversation quality score
        quality_score = self.calculate_quality_score(conversation, user_sentiment, ai_sentiment)
        
        enriched = {
            **conversation,
            'analytics': {
                'user_sentiment': user_sentiment,
                'ai_sentiment': ai_sentiment,
                'user_emotions': user_emotions,
                'ai_emotions': ai_emotions,
                'user_text_metrics': user_metrics,
                'ai_text_metrics': ai_metrics,
                'quality_score': quality_score,
                'processed_at': datetime.utcnow().isoformat()
            }
        }
        
        return enriched
        
    def analyze_sentiment(self, text: str) -> Dict:
        """Analyze sentiment of text"""
        if not text.strip():
            return {'label': 'NEUTRAL', 'score': 0.5}
            
        results = self.sentiment_analyzer(text)
        return {
            'label': results[0]['label'],
            'score': results[0]['score'],
            'all_scores': results
        }
        
    def analyze_emotions(self, text: str) -> Dict:
        """Analyze emotions in text"""
        if not text.strip():
            return {'dominant_emotion': 'neutral', 'scores': {}}
            
        results = self.emotion_analyzer(text)
        emotions = {result['label']: result['score'] for result in results}
        dominant = max(emotions, key=emotions.get)
        
        return {
            'dominant_emotion': dominant,
            'scores': emotions
        }
        
    def calculate_text_metrics(self, text: str) -> Dict:
        """Calculate text metrics"""
        words = text.split()
        sentences = text.split('.')
        
        return {
            'word_count': len(words),
            'sentence_count': len(sentences),
            'avg_word_length': np.mean([len(word) for word in words]) if words else 0,
            'character_count': len(text),
            'complexity_score': len(set(words)) / len(words) if words else 0
        }
        
    def calculate_quality_score(self, conversation: Dict, user_sentiment: Dict, ai_sentiment: Dict) -> float:
        """Calculate conversation quality score"""
        base_score = conversation.get('consciousness_metrics', {}).get('quality_score', 0.5)
        
        # Adjust based on sentiment alignment
        sentiment_alignment = 1 - abs(user_sentiment['score'] - ai_sentiment['score'])
        
        # Adjust based on response appropriateness
        response_length = len(conversation.get('ai_response', ''))
        length_score = min(1.0, response_length / 100)  # Optimal around 100 chars
        
        # Combine scores
        quality_score = (base_score * 0.6 + sentiment_alignment * 0.2 + length_score * 0.2)
        
        return min(1.0, max(0.0, quality_score))
        
    async def process_batch(self, batch: List[Dict]):
        """Process batch of enriched conversations"""
        # Store in ClickHouse for analytics
        await self.store_analytics_batch(batch)
        
        # Update real-time metrics
        await self.update_realtime_metrics(batch)
        
        # Detect anomalies
        anomalies = await self.detect_anomalies(batch)
        if anomalies:
            await self.handle_anomalies(anomalies)
            
        # Update user segments
        await self.update_user_segments(batch)
        
    async def store_analytics_batch(self, batch: List[Dict]):
        """Store analytics data in ClickHouse"""
        rows = []
        
        for conversation in batch:
            analytics = conversation.get('analytics', {})
            
            row = {
                'conversation_id': conversation.get('id'),
                'user_id': conversation.get('user_id'),
                'timestamp': conversation.get('created_at'),
                'user_sentiment_label': analytics.get('user_sentiment', {}).get('label'),
                'user_sentiment_score': analytics.get('user_sentiment', {}).get('score'),
                'ai_sentiment_label': analytics.get('ai_sentiment', {}).get('label'),
                'ai_sentiment_score': analytics.get('ai_sentiment', {}).get('score'),
                'dominant_user_emotion': analytics.get('user_emotions', {}).get('dominant_emotion'),
                'dominant_ai_emotion': analytics.get('ai_emotions', {}).get('dominant_emotion'),
                'quality_score': analytics.get('quality_score'),
                'user_word_count': analytics.get('user_text_metrics', {}).get('word_count'),
                'ai_word_count': analytics.get('ai_text_metrics', {}).get('word_count'),
                'response_time_ms': conversation.get('response_time_ms'),
                'consciousness_level': conversation.get('consciousness_metrics', {}).get('consciousness_level'),
                'ethical_score': conversation.get('consciousness_metrics', {}).get('ethical_score'),
                'empathy_score': conversation.get('consciousness_metrics', {}).get('empathy_score')
            }
            
            rows.append(row)
        
        # Insert into ClickHouse
        self.ch_client.insert('conversation_analytics', rows)
        
    async def update_realtime_metrics(self, batch: List[Dict]):
        """Update real-time metrics in Redis"""
        current_time = datetime.utcnow()
        window_key = f"metrics:{current_time.strftime('%Y-%m-%d:%H:%M')}"
        
        metrics = {
            'total_conversations': len(batch),
            'avg_quality_score': np.mean([conv.get('analytics', {}).get('quality_score', 0) for conv in batch]),
            'avg_response_time': np.mean([conv.get('response_time_ms', 0) for conv in batch]),
            'sentiment_distribution': self.calculate_sentiment_distribution(batch),
            'emotion_distribution': self.calculate_emotion_distribution(batch),
            'user_activity': len(set(conv.get('user_id') for conv in batch))
        }
        
        # Store in Redis with expiration
        self.redis_client.setex(window_key, 3600, json.dumps(metrics))
        
    def calculate_sentiment_distribution(self, batch: List[Dict]) -> Dict:
        """Calculate sentiment distribution"""
        sentiments = [conv.get('analytics', {}).get('user_sentiment', {}).get('label', 'NEUTRAL') for conv in batch]
        distribution = {}
        for sentiment in sentiments:
            distribution[sentiment] = distribution.get(sentiment, 0) + 1
        return distribution
        
    def calculate_emotion_distribution(self, batch: List[Dict]) -> Dict:
        """Calculate emotion distribution"""
        emotions = [conv.get('analytics', {}).get('user_emotions', {}).get('dominant_emotion', 'neutral') for conv in batch]
        distribution = {}
        for emotion in emotions:
            distribution[emotion] = distribution.get(emotion, 0) + 1
        return distribution
        
    async def detect_anomalies(self, batch: List[Dict]) -> List[Dict]:
        """Detect anomalies in conversation patterns"""
        # Extract features for anomaly detection
        features = []
        for conv in batch:
            analytics = conv.get('analytics', {})
            feature_vector = [
                analytics.get('quality_score', 0),
                analytics.get('user_sentiment', {}).get('score', 0.5),
                analytics.get('ai_sentiment', {}).get('score', 0.5),
                conv.get('response_time_ms', 0) / 1000,  # Convert to seconds
                analytics.get('user_text_metrics', {}).get('word_count', 0),
                analytics.get('ai_text_metrics', {}).get('word_count', 0)
            ]
            features.append(feature_vector)
        
        if len(features) < 10:  # Need minimum samples
            return []
            
        # Use Isolation Forest for anomaly detection
        detector = IsolationForest(contamination=self.config.anomaly_threshold)
        anomaly_labels = detector.fit_predict(features)
        
        anomalies = []
        for i, label in enumerate(anomaly_labels):
            if label == -1:  # Anomaly detected
                anomalies.append({
                    'conversation': batch[i],
                    'anomaly_score': detector.decision_function([features[i]])[0],
                    'detected_at': datetime.utcnow().isoformat()
                })
                
        return anomalies
        
    async def handle_anomalies(self, anomalies: List[Dict]):
        """Handle detected anomalies"""
        for anomaly in anomalies:
            # Log anomaly
            self.logger.warning(f"Anomaly detected: {anomaly['anomaly_score']}")
            
            # Store in database for investigation
            anomaly_record = {
                'conversation_id': anomaly['conversation']['id'],
                'anomaly_score': anomaly['anomaly_score'],
                'detected_at': anomaly['detected_at'],
                'investigation_status': 'pending'
            }
            
            # Send to Kafka for real-time alerts
            self.kafka_producer.send('consciousness-anomalies', anomaly_record)
            
    async def update_user_segments(self, batch: List[Dict]):
        """Update user segments based on behavior"""
        user_behaviors = {}
        
        for conv in batch:
            user_id = conv.get('user_id')
            if not user_id:
                continue
                
            if user_id not in user_behaviors:
                user_behaviors[user_id] = {
                    'conversations': 0,
                    'avg_quality': 0,
                    'dominant_sentiment': 'NEUTRAL',
                    'dominant_emotion': 'neutral',
                    'avg_response_time': 0
                }
            
            analytics = conv.get('analytics', {})
            behavior = user_behaviors[user_id]
            
            behavior['conversations'] += 1
            behavior['avg_quality'] = (behavior['avg_quality'] + analytics.get('quality_score', 0)) / 2
            behavior['avg_response_time'] = (behavior['avg_response_time'] + conv.get('response_time_ms', 0)) / 2
            
        # Update user segments in database
        for user_id, behavior in user_behaviors.items():
            segment = self.determine_user_segment(behavior)
            
            # Update in Redis
            self.redis_client.setex(
                f"user_segment:{user_id}",
                86400,  # 24 hours
                json.dumps({
                    'segment': segment,
                    'behavior': behavior,
                    'updated_at': datetime.utcnow().isoformat()
                })
            )
            
    def determine_user_segment(self, behavior: Dict) -> str:
        """Determine user segment based on behavior"""
        conversations = behavior['conversations']
        avg_quality = behavior['avg_quality']
        
        if conversations >= 50 and avg_quality >= 0.8:
            return 'power_user'
        elif conversations >= 20 and avg_quality >= 0.6:
            return 'regular_user'
        elif conversations >= 5:
            return 'casual_user'
        else:
            return 'new_user'
            
    async def generate_insights_report(self, time_range: str = '24h') -> Dict:
        """Generate insights report"""
        # Query analytics data
        query = f"""
        SELECT 
            COUNT(*) as total_conversations,
            AVG(quality_score) as avg_quality,
            AVG(response_time_ms) as avg_response_time,
            COUNT(DISTINCT user_id) as unique_users,
            user_sentiment_label,
            dominant_user_emotion,
            toHour(timestamp) as hour
        FROM conversation_analytics 
        WHERE timestamp >= now() - INTERVAL {time_range}
        GROUP BY user_sentiment_label, dominant_user_emotion, hour
        ORDER BY hour
        """
        
        results = self.ch_client.query(query)
        
        # Process results into insights
        insights = {
            'summary': {
                'total_conversations': sum(row[0] for row in results.result_rows),
                'avg_quality_score': np.mean([row[1] for row in results.result_rows]),
                'avg_response_time_ms': np.mean([row[2] for row in results.result_rows]),
                'unique_users': len(set(row[3] for row in results.result_rows))
            },
            'trends': {
                'hourly_activity': {},
                'sentiment_trends': {},
                'emotion_trends': {}
            },
            'recommendations': self.generate_recommendations(results.result_rows)
        }
        
        return insights
        
    def generate_recommendations(self, data: List) -> List[str]:
        """Generate actionable recommendations"""
        recommendations = []
        
        # Analyze patterns and generate recommendations
        avg_quality = np.mean([row[1] for row in data])
        if avg_quality < 0.7:
            recommendations.append("Consider improving AI response quality through additional training")
            
        avg_response_time = np.mean([row[2] for row in data])
        if avg_response_time > 2000:  # 2 seconds
            recommendations.append("Optimize response time - current average is above 2 seconds")
            
        return recommendations

# Dash Dashboard
def create_dashboard(analytics: ConsciousnessAnalytics):
    """Create interactive dashboard"""
    app = dash.Dash(__name__, external_stylesheets=[dbc.themes.BOOTSTRAP])
    
    app.layout = dbc.Container([
        dbc.Row([
            dbc.Col([
                html.H1("Consciousness Engine Analytics", className="text-center mb-4"),
                html.Hr()
            ])
        ]),
        
        dbc.Row([
            dbc.Col([
                dcc.Graph(id="conversation-volume-chart")
            ], width=6),
            dbc.Col([
                dcc.Graph(id="quality-score-chart")
            ], width=6)
        ]),
        
        dbc.Row([
            dbc.Col([
                dcc.Graph(id="sentiment-distribution-chart")
            ], width=6),
            dbc.Col([
                dcc.Graph(id="emotion-distribution-chart")
            ], width=6)
        ]),
        
        dcc.Interval(
            id='interval-component',
            interval=30*1000,  # Update every 30 seconds
            n_intervals=0
        )
    ])
    
    @app.callback(
        [Output('conversation-volume-chart', 'figure'),
         Output('quality-score-chart', 'figure'),
         Output('sentiment-distribution-chart', 'figure'),
         Output('emotion-distribution-chart', 'figure')],
        [Input('interval-component', 'n_intervals')]
    )
    def update_charts(n):
        # Fetch real-time data and update charts
        # Implementation would fetch data from ClickHouse/Redis
        
        # Placeholder charts
        volume_fig = px.line(title="Conversation Volume")
        quality_fig = px.line(title="Quality Score Trend")
        sentiment_fig = px.pie(title="Sentiment Distribution")
        emotion_fig = px.bar(title="Emotion Distribution")
        
        return volume_fig, quality_fig, sentiment_fig, emotion_fig
    
    return app

async def main():
    """Main analytics pipeline"""
    config = AnalyticsConfig()
    analytics = ConsciousnessAnalytics(config)
    
    # Start dashboard
    dashboard = create_dashboard(analytics)
    
    # Start analytics pipeline
    await analytics.process_conversation_stream()

if __name__ == "__main__":
    asyncio.run(main())
