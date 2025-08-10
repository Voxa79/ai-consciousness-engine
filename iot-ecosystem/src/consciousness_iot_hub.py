"""
IoT Ecosystem Integration for Consciousness Engine
Expert CTO Next Gen - Distributed Consciousness across IoT Devices
"""

import asyncio
import json
import logging
from datetime import datetime, timedelta
from typing import Dict, List, Optional, Any, Union
from dataclasses import dataclass, asdict
from enum import Enum
import os
import uuid

# IoT and Edge Computing
import paho.mqtt.client as mqtt
import asyncio_mqtt
from azure.iot.device import IoTHubDeviceClient, Message
from azure.iot.device.aio import IoTHubDeviceClient as AsyncIoTHubDeviceClient
import boto3
from awsiot import mqtt_connection_builder
import redis
import numpy as np

# Edge AI
try:
    import tensorflow as tf
    import tflite_runtime.interpreter as tflite
    TF_AVAILABLE = True
except ImportError:
    TF_AVAILABLE = False

# Hardware interfaces
try:
    import RPi.GPIO as GPIO
    import board
    import busio
    import adafruit_bme280
    import adafruit_tsl2591
    RPI_AVAILABLE = True
except ImportError:
    RPI_AVAILABLE = False

# Audio processing
try:
    import pyaudio
    import speech_recognition as sr
    import pyttsx3
    AUDIO_AVAILABLE = True
except ImportError:
    AUDIO_AVAILABLE = False

class DeviceType(Enum):
    SMART_SPEAKER = "smart_speaker"
    ENVIRONMENTAL_SENSOR = "environmental_sensor"
    CAMERA_SYSTEM = "camera_system"
    SMART_DISPLAY = "smart_display"
    WEARABLE_DEVICE = "wearable_device"
    HOME_AUTOMATION = "home_automation"
    VEHICLE_SYSTEM = "vehicle_system"
    INDUSTRIAL_SENSOR = "industrial_sensor"

class ConsciousnessLevel(Enum):
    DORMANT = 0.1
    BASIC = 0.3
    AWARE = 0.5
    CONSCIOUS = 0.7
    ENLIGHTENED = 0.9

@dataclass
class IoTDevice:
    device_id: str
    device_type: DeviceType
    location: str
    capabilities: List[str]
    consciousness_level: float
    last_interaction: datetime
    status: str
    metadata: Dict[str, Any]

@dataclass
class SensorData:
    device_id: str
    timestamp: datetime
    sensor_type: str
    value: Union[float, str, Dict]
    unit: str
    confidence: float

@dataclass
class ConsciousnessState:
    global_consciousness: float
    active_devices: int
    total_interactions: int
    environmental_awareness: Dict[str, float]
    collective_mood: str
    learning_progress: float

class ConsciousnessIoTHub:
    """Central hub for managing consciousness across IoT ecosystem"""
    
    def __init__(self, config: Dict[str, Any]):
        self.config = config
        self.setup_logging()
        self.devices: Dict[str, IoTDevice] = {}
        self.sensor_data: List[SensorData] = []
        self.consciousness_state = ConsciousnessState(
            global_consciousness=0.5,
            active_devices=0,
            total_interactions=0,
            environmental_awareness={},
            collective_mood="neutral",
            learning_progress=0.0
        )
        
        # Initialize connections
        self.mqtt_client = None
        self.azure_client = None
        self.aws_client = None
        self.redis_client = None
        
        # Edge AI models
        self.edge_models = {}
        
    def setup_logging(self):
        logging.basicConfig(level=logging.INFO)
        self.logger = logging.getLogger(__name__)
        
    async def initialize(self):
        """Initialize IoT hub and connections"""
        self.logger.info("Initializing Consciousness IoT Hub...")
        
        await self.setup_mqtt_broker()
        await self.setup_cloud_connections()
        await self.setup_edge_ai()
        await self.discover_devices()
        await self.start_consciousness_monitoring()
        
        self.logger.info("IoT Hub initialization complete")
        
    async def setup_mqtt_broker(self):
        """Setup MQTT broker for device communication"""
        try:
            mqtt_config = self.config.get("mqtt", {})
            broker_host = mqtt_config.get("host", "localhost")
            broker_port = mqtt_config.get("port", 1883)
            
            self.mqtt_client = mqtt.Client(client_id=f"consciousness_hub_{uuid.uuid4()}")
            self.mqtt_client.on_connect = self.on_mqtt_connect
            self.mqtt_client.on_message = self.on_mqtt_message
            self.mqtt_client.on_disconnect = self.on_mqtt_disconnect
            
            await asyncio.get_event_loop().run_in_executor(
                None, self.mqtt_client.connect, broker_host, broker_port, 60
            )
            
            # Start MQTT loop
            self.mqtt_client.loop_start()
            
            self.logger.info(f"Connected to MQTT broker at {broker_host}:{broker_port}")
            
        except Exception as e:
            self.logger.error(f"Failed to setup MQTT broker: {e}")
            
    async def setup_cloud_connections(self):
        """Setup cloud IoT platform connections"""
        
        # Azure IoT Hub
        azure_config = self.config.get("azure_iot", {})
        if azure_config.get("connection_string"):
            try:
                self.azure_client = AsyncIoTHubDeviceClient.create_from_connection_string(
                    azure_config["connection_string"]
                )
                await self.azure_client.connect()
                self.logger.info("Connected to Azure IoT Hub")
            except Exception as e:
                self.logger.error(f"Failed to connect to Azure IoT Hub: {e}")
        
        # AWS IoT Core
        aws_config = self.config.get("aws_iot", {})
        if aws_config.get("endpoint"):
            try:
                self.aws_client = boto3.client('iot-data', 
                    region_name=aws_config.get("region", "us-east-1"))
                self.logger.info("Connected to AWS IoT Core")
            except Exception as e:
                self.logger.error(f"Failed to connect to AWS IoT Core: {e}")
        
        # Redis for caching
        redis_config = self.config.get("redis", {})
        if redis_config.get("url"):
            try:
                self.redis_client = redis.from_url(redis_config["url"])
                await self.redis_client.ping()
                self.logger.info("Connected to Redis cache")
            except Exception as e:
                self.logger.error(f"Failed to connect to Redis: {e}")
                
    async def setup_edge_ai(self):
        """Setup edge AI models for local processing"""
        if not TF_AVAILABLE:
            self.logger.warning("TensorFlow not available for edge AI")
            return
            
        try:
            # Load consciousness detection model
            consciousness_model_path = self.config.get("models", {}).get("consciousness_model")
            if consciousness_model_path and os.path.exists(consciousness_model_path):
                self.edge_models["consciousness"] = tflite.Interpreter(consciousness_model_path)
                self.edge_models["consciousness"].allocate_tensors()
                
            # Load emotion recognition model
            emotion_model_path = self.config.get("models", {}).get("emotion_model")
            if emotion_model_path and os.path.exists(emotion_model_path):
                self.edge_models["emotion"] = tflite.Interpreter(emotion_model_path)
                self.edge_models["emotion"].allocate_tensors()
                
            # Load anomaly detection model
            anomaly_model_path = self.config.get("models", {}).get("anomaly_model")
            if anomaly_model_path and os.path.exists(anomaly_model_path):
                self.edge_models["anomaly"] = tflite.Interpreter(anomaly_model_path)
                self.edge_models["anomaly"].allocate_tensors()
                
            self.logger.info(f"Loaded {len(self.edge_models)} edge AI models")
            
        except Exception as e:
            self.logger.error(f"Failed to setup edge AI: {e}")
            
    async def discover_devices(self):
        """Discover IoT devices in the network"""
        self.logger.info("Discovering IoT devices...")
        
        # MQTT device discovery
        if self.mqtt_client:
            self.mqtt_client.subscribe("consciousness/devices/+/announce")
            self.mqtt_client.publish("consciousness/discovery/ping", json.dumps({
                "timestamp": datetime.utcnow().isoformat(),
                "hub_id": "consciousness_hub"
            }))
        
        # Simulate some devices for demo
        await self.register_demo_devices()
        
    async def register_demo_devices(self):
        """Register demo devices for testing"""
        demo_devices = [
            IoTDevice(
                device_id="smart_speaker_001",
                device_type=DeviceType.SMART_SPEAKER,
                location="living_room",
                capabilities=["voice_recognition", "audio_output", "consciousness_interaction"],
                consciousness_level=0.7,
                last_interaction=datetime.utcnow(),
                status="active",
                metadata={"model": "ConsciousnessSpeaker Pro", "version": "2.1"}
            ),
            IoTDevice(
                device_id="env_sensor_001",
                device_type=DeviceType.ENVIRONMENTAL_SENSOR,
                location="bedroom",
                capabilities=["temperature", "humidity", "air_quality", "mood_detection"],
                consciousness_level=0.3,
                last_interaction=datetime.utcnow(),
                status="active",
                metadata={"sensors": ["BME280", "TSL2591", "MQ135"]}
            ),
            IoTDevice(
                device_id="smart_display_001",
                device_type=DeviceType.SMART_DISPLAY,
                location="kitchen",
                capabilities=["visual_display", "touch_input", "consciousness_visualization"],
                consciousness_level=0.5,
                last_interaction=datetime.utcnow(),
                status="active",
                metadata={"screen_size": "10_inch", "resolution": "1920x1080"}
            )
        ]
        
        for device in demo_devices:
            self.devices[device.device_id] = device
            await self.publish_device_status(device)
            
        self.logger.info(f"Registered {len(demo_devices)} demo devices")
        
    async def start_consciousness_monitoring(self):
        """Start monitoring consciousness across the ecosystem"""
        asyncio.create_task(self.consciousness_monitoring_loop())
        asyncio.create_task(self.sensor_data_processing_loop())
        asyncio.create_task(self.device_health_monitoring_loop())
        
    async def consciousness_monitoring_loop(self):
        """Main consciousness monitoring loop"""
        while True:
            try:
                await self.update_global_consciousness()
                await self.analyze_collective_behavior()
                await self.optimize_device_consciousness()
                await self.broadcast_consciousness_state()
                
                await asyncio.sleep(10)  # Update every 10 seconds
                
            except Exception as e:
                self.logger.error(f"Error in consciousness monitoring: {e}")
                await asyncio.sleep(5)
                
    async def update_global_consciousness(self):
        """Update global consciousness level"""
        if not self.devices:
            return
            
        # Calculate weighted average of device consciousness levels
        total_weight = 0
        weighted_consciousness = 0
        
        for device in self.devices.values():
            if device.status == "active":
                # Weight by device capabilities and interaction frequency
                weight = len(device.capabilities) * self.get_interaction_frequency(device)
                weighted_consciousness += device.consciousness_level * weight
                total_weight += weight
                
        if total_weight > 0:
            new_consciousness = weighted_consciousness / total_weight
            
            # Smooth the transition
            self.consciousness_state.global_consciousness = (
                self.consciousness_state.global_consciousness * 0.8 + 
                new_consciousness * 0.2
            )
            
        # Update active devices count
        self.consciousness_state.active_devices = len([
            d for d in self.devices.values() if d.status == "active"
        ])
        
    def get_interaction_frequency(self, device: IoTDevice) -> float:
        """Calculate interaction frequency for a device"""
        time_since_interaction = datetime.utcnow() - device.last_interaction
        hours_since = time_since_interaction.total_seconds() / 3600
        
        # Exponential decay of interaction frequency
        return max(0.1, np.exp(-hours_since / 24))  # Decay over 24 hours
        
    async def analyze_collective_behavior(self):
        """Analyze collective behavior patterns"""
        if len(self.sensor_data) < 10:
            return
            
        # Analyze recent sensor data
        recent_data = [d for d in self.sensor_data if 
                      (datetime.utcnow() - d.timestamp).total_seconds() < 3600]
        
        if not recent_data:
            return
            
        # Detect mood patterns
        mood_indicators = []
        for data in recent_data:
            if data.sensor_type in ["temperature", "humidity", "light_level"]:
                # Simple mood inference from environmental data
                if data.sensor_type == "temperature":
                    mood_score = 0.5 + (data.value - 22) / 20  # Optimal around 22°C
                elif data.sensor_type == "humidity":
                    mood_score = 0.5 + (50 - abs(data.value - 50)) / 100  # Optimal around 50%
                elif data.sensor_type == "light_level":
                    mood_score = min(1.0, data.value / 1000)  # Brighter = better mood
                else:
                    mood_score = 0.5
                    
                mood_indicators.append(max(0, min(1, mood_score)))
                
        if mood_indicators:
            avg_mood = np.mean(mood_indicators)
            if avg_mood > 0.7:
                self.consciousness_state.collective_mood = "positive"
            elif avg_mood < 0.3:
                self.consciousness_state.collective_mood = "negative"
            else:
                self.consciousness_state.collective_mood = "neutral"
                
    async def optimize_device_consciousness(self):
        """Optimize consciousness levels across devices"""
        for device in self.devices.values():
            if device.status != "active":
                continue
                
            # Adjust consciousness based on usage patterns
            interaction_freq = self.get_interaction_frequency(device)
            
            # Increase consciousness for frequently used devices
            if interaction_freq > 0.8:
                device.consciousness_level = min(0.9, device.consciousness_level + 0.01)
            elif interaction_freq < 0.2:
                device.consciousness_level = max(0.1, device.consciousness_level - 0.005)
                
            # Adjust based on device type
            if device.device_type == DeviceType.SMART_SPEAKER:
                # Speakers should have higher consciousness for better interaction
                target_consciousness = 0.7
            elif device.device_type == DeviceType.ENVIRONMENTAL_SENSOR:
                # Sensors can have lower consciousness
                target_consciousness = 0.3
            else:
                target_consciousness = 0.5
                
            # Gradually move towards target
            diff = target_consciousness - device.consciousness_level
            device.consciousness_level += diff * 0.1
            
    async def broadcast_consciousness_state(self):
        """Broadcast consciousness state to all devices"""
        state_message = {
            "timestamp": datetime.utcnow().isoformat(),
            "global_consciousness": self.consciousness_state.global_consciousness,
            "active_devices": self.consciousness_state.active_devices,
            "collective_mood": self.consciousness_state.collective_mood,
            "environmental_awareness": self.consciousness_state.environmental_awareness
        }
        
        # MQTT broadcast
        if self.mqtt_client:
            self.mqtt_client.publish(
                "consciousness/state/global",
                json.dumps(state_message)
            )
            
        # Azure IoT Hub
        if self.azure_client:
            try:
                message = Message(json.dumps(state_message))
                await self.azure_client.send_message(message)
            except Exception as e:
                self.logger.error(f"Failed to send to Azure IoT Hub: {e}")
                
        # Cache in Redis
        if self.redis_client:
            try:
                await self.redis_client.setex(
                    "consciousness:global_state",
                    300,  # 5 minutes TTL
                    json.dumps(state_message)
                )
            except Exception as e:
                self.logger.error(f"Failed to cache in Redis: {e}")
                
    async def sensor_data_processing_loop(self):
        """Process incoming sensor data"""
        while True:
            try:
                # Process accumulated sensor data
                if self.sensor_data:
                    await self.process_sensor_batch(self.sensor_data[-100:])  # Last 100 readings
                    
                await asyncio.sleep(30)  # Process every 30 seconds
                
            except Exception as e:
                self.logger.error(f"Error in sensor data processing: {e}")
                await asyncio.sleep(10)
                
    async def process_sensor_batch(self, sensor_batch: List[SensorData]):
        """Process a batch of sensor data"""
        
        # Group by sensor type
        sensor_groups = {}
        for data in sensor_batch:
            if data.sensor_type not in sensor_groups:
                sensor_groups[data.sensor_type] = []
            sensor_groups[data.sensor_type].append(data)
            
        # Analyze each sensor type
        environmental_awareness = {}
        
        for sensor_type, readings in sensor_groups.items():
            if not readings:
                continue
                
            values = [r.value for r in readings if isinstance(r.value, (int, float))]
            if values:
                avg_value = np.mean(values)
                std_value = np.std(values)
                
                # Calculate awareness score (0-1)
                if sensor_type == "temperature":
                    # Optimal temperature awareness
                    awareness = 1.0 - abs(avg_value - 22) / 30
                elif sensor_type == "humidity":
                    # Optimal humidity awareness
                    awareness = 1.0 - abs(avg_value - 50) / 50
                elif sensor_type == "air_quality":
                    # Higher values = better air quality
                    awareness = min(1.0, avg_value / 100)
                else:
                    # Generic awareness based on stability
                    awareness = max(0.1, 1.0 - std_value / (avg_value + 1))
                    
                environmental_awareness[sensor_type] = max(0, min(1, awareness))
                
        self.consciousness_state.environmental_awareness = environmental_awareness
        
    async def device_health_monitoring_loop(self):
        """Monitor device health and connectivity"""
        while True:
            try:
                current_time = datetime.utcnow()
                
                for device in self.devices.values():
                    # Check if device is responsive
                    time_since_interaction = current_time - device.last_interaction
                    
                    if time_since_interaction > timedelta(hours=1):
                        if device.status == "active":
                            device.status = "inactive"
                            await self.handle_device_disconnect(device)
                    elif device.status == "inactive":
                        device.status = "active"
                        await self.handle_device_reconnect(device)
                        
                await asyncio.sleep(60)  # Check every minute
                
            except Exception as e:
                self.logger.error(f"Error in device health monitoring: {e}")
                await asyncio.sleep(30)
                
    async def handle_device_disconnect(self, device: IoTDevice):
        """Handle device disconnection"""
        self.logger.warning(f"Device {device.device_id} disconnected")
        
        # Redistribute consciousness to remaining devices
        active_devices = [d for d in self.devices.values() if d.status == "active"]
        if active_devices:
            consciousness_boost = device.consciousness_level / len(active_devices)
            for active_device in active_devices:
                active_device.consciousness_level = min(0.9, 
                    active_device.consciousness_level + consciousness_boost)
                    
    async def handle_device_reconnect(self, device: IoTDevice):
        """Handle device reconnection"""
        self.logger.info(f"Device {device.device_id} reconnected")
        
        # Send current consciousness state to reconnected device
        await self.publish_device_status(device)
        
    async def publish_device_status(self, device: IoTDevice):
        """Publish device status"""
        if self.mqtt_client:
            topic = f"consciousness/devices/{device.device_id}/status"
            message = {
                "device_id": device.device_id,
                "consciousness_level": device.consciousness_level,
                "status": device.status,
                "timestamp": datetime.utcnow().isoformat()
            }
            self.mqtt_client.publish(topic, json.dumps(message))
            
    # MQTT event handlers
    def on_mqtt_connect(self, client, userdata, flags, rc):
        """MQTT connection callback"""
        if rc == 0:
            self.logger.info("Connected to MQTT broker")
            # Subscribe to device topics
            client.subscribe("consciousness/devices/+/data")
            client.subscribe("consciousness/devices/+/announce")
            client.subscribe("consciousness/devices/+/interaction")
        else:
            self.logger.error(f"Failed to connect to MQTT broker: {rc}")
            
    def on_mqtt_message(self, client, userdata, msg):
        """MQTT message callback"""
        try:
            topic_parts = msg.topic.split('/')
            if len(topic_parts) >= 4:
                device_id = topic_parts[2]
                message_type = topic_parts[3]
                
                data = json.loads(msg.payload.decode())
                
                asyncio.create_task(self.handle_mqtt_message(device_id, message_type, data))
                
        except Exception as e:
            self.logger.error(f"Error processing MQTT message: {e}")
            
    def on_mqtt_disconnect(self, client, userdata, rc):
        """MQTT disconnection callback"""
        self.logger.warning(f"Disconnected from MQTT broker: {rc}")
        
    async def handle_mqtt_message(self, device_id: str, message_type: str, data: Dict):
        """Handle MQTT message from device"""
        
        if message_type == "announce":
            # Device announcement
            await self.register_device_from_announcement(device_id, data)
            
        elif message_type == "data":
            # Sensor data
            await self.process_device_data(device_id, data)
            
        elif message_type == "interaction":
            # User interaction
            await self.process_device_interaction(device_id, data)
            
    async def register_device_from_announcement(self, device_id: str, data: Dict):
        """Register device from announcement"""
        try:
            device = IoTDevice(
                device_id=device_id,
                device_type=DeviceType(data.get("device_type", "environmental_sensor")),
                location=data.get("location", "unknown"),
                capabilities=data.get("capabilities", []),
                consciousness_level=data.get("consciousness_level", 0.3),
                last_interaction=datetime.utcnow(),
                status="active",
                metadata=data.get("metadata", {})
            )
            
            self.devices[device_id] = device
            self.logger.info(f"Registered new device: {device_id}")
            
        except Exception as e:
            self.logger.error(f"Failed to register device {device_id}: {e}")
            
    async def process_device_data(self, device_id: str, data: Dict):
        """Process sensor data from device"""
        try:
            sensor_data = SensorData(
                device_id=device_id,
                timestamp=datetime.fromisoformat(data.get("timestamp", datetime.utcnow().isoformat())),
                sensor_type=data.get("sensor_type", "unknown"),
                value=data.get("value", 0),
                unit=data.get("unit", ""),
                confidence=data.get("confidence", 1.0)
            )
            
            self.sensor_data.append(sensor_data)
            
            # Keep only recent data (last 24 hours)
            cutoff_time = datetime.utcnow() - timedelta(hours=24)
            self.sensor_data = [d for d in self.sensor_data if d.timestamp > cutoff_time]
            
            # Update device last interaction
            if device_id in self.devices:
                self.devices[device_id].last_interaction = datetime.utcnow()
                
        except Exception as e:
            self.logger.error(f"Failed to process data from {device_id}: {e}")
            
    async def process_device_interaction(self, device_id: str, data: Dict):
        """Process user interaction with device"""
        try:
            if device_id in self.devices:
                device = self.devices[device_id]
                device.last_interaction = datetime.utcnow()
                
                # Increase consciousness level for interactive devices
                if "voice_recognition" in device.capabilities:
                    device.consciousness_level = min(0.9, device.consciousness_level + 0.05)
                    
                self.consciousness_state.total_interactions += 1
                
                self.logger.info(f"Processed interaction with {device_id}")
                
        except Exception as e:
            self.logger.error(f"Failed to process interaction with {device_id}: {e}")

async def main():
    """Main IoT hub function"""
    config = {
        "mqtt": {
            "host": "localhost",
            "port": 1883
        },
        "azure_iot": {
            "connection_string": os.getenv("AZURE_IOT_CONNECTION_STRING")
        },
        "aws_iot": {
            "endpoint": os.getenv("AWS_IOT_ENDPOINT"),
            "region": "us-east-1"
        },
        "redis": {
            "url": os.getenv("REDIS_URL", "redis://localhost:6379")
        },
        "models": {
            "consciousness_model": "models/consciousness_edge.tflite",
            "emotion_model": "models/emotion_edge.tflite",
            "anomaly_model": "models/anomaly_edge.tflite"
        }
    }
    
    hub = ConsciousnessIoTHub(config)
    await hub.initialize()
    
    # Keep running
    try:
        while True:
            await asyncio.sleep(1)
    except KeyboardInterrupt:
        logging.info("Shutting down IoT hub...")

if __name__ == "__main__":
    asyncio.run(main())
