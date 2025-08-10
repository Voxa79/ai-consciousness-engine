// Simple API Gateway - Sans dépendances problématiques
// Expert CTO Next Gen - Version ultra-simple

const express = require('express');
const cors = require('cors');
const app = express();

app.use(cors());
app.use(express.json());

const CONSCIOUSNESS_ENGINE_URL = process.env.CONSCIOUSNESS_ENGINE_URL || 'http://localhost:8080';

// Middleware de logging
app.use((req, res, next) => {
  console.log(`${new Date().toISOString()} - ${req.method} ${req.path}`);
  next();
});

// Health check du gateway
app.get('/health', (req, res) => {
  res.json({
    status: 'healthy',
    timestamp: new Date().toISOString(),
    version: '1.0.0-simple-gateway',
    service: 'api-gateway',
    upstream_services: {
      consciousness_engine: CONSCIOUSNESS_ENGINE_URL
    }
  });
});

// Proxy simple pour consciousness/process
app.post('/api/v1/consciousness/process', async (req, res) => {
  try {
    const targetUrl = `${CONSCIOUSNESS_ENGINE_URL}/consciousness/process`;
    console.log(`Proxying POST to ${targetUrl}`);
    
    const response = await fetch(targetUrl, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(req.body)
    });
    
    const data = await response.json();
    res.status(response.status).json(data);
  } catch (error) {
    console.error('Proxy error:', error.message);
    res.status(502).json({
      error: 'Bad Gateway',
      message: 'Consciousness Engine unavailable',
      timestamp: new Date().toISOString()
    });
  }
});

// Proxy simple pour consciousness/state
app.get('/api/v1/consciousness/state', async (req, res) => {
  try {
    const targetUrl = `${CONSCIOUSNESS_ENGINE_URL}/consciousness/state`;
    console.log(`Proxying GET to ${targetUrl}`);
    
    const response = await fetch(targetUrl);
    const data = await response.json();
    res.status(response.status).json(data);
  } catch (error) {
    console.error('Proxy error:', error.message);
    res.status(502).json({
      error: 'Bad Gateway',
      message: 'Consciousness Engine unavailable',
      timestamp: new Date().toISOString()
    });
  }
});

// Mock endpoints pour autres services
app.get('/api/v1/agents', (req, res) => {
  res.json({
    agents: [
      {
        id: 'agent-001',
        name: 'Consciousness Analyzer',
        type: 'consciousness',
        status: 'active',
        capabilities: ['self-awareness', 'ethical-reasoning', 'emotional-intelligence'],
        last_activity: new Date().toISOString()
      }
    ],
    total_count: 1,
    active_count: 1,
    timestamp: new Date().toISOString()
  });
});

app.get('/api/v1/governance/policies', (req, res) => {
  res.json({
    policies: [
      {
        id: 'policy-001',
        name: 'Ethical AI Guidelines',
        category: 'ethics',
        status: 'active',
        compliance_rate: 0.98,
        last_updated: new Date(Date.now() - 86400000).toISOString()
      }
    ],
    total_policies: 1,
    active_policies: 1,
    overall_compliance: 0.98,
    timestamp: new Date().toISOString()
  });
});

app.get('/api/v1/metrics', (req, res) => {
  res.json({
    system_metrics: {
      total_requests: Math.floor(Math.random() * 10000) + 5000,
      requests_per_minute: Math.floor(Math.random() * 100) + 20,
      average_response_time: Math.floor(Math.random() * 200) + 50,
      error_rate: Math.random() * 0.05,
      uptime_percentage: 99.5 + Math.random() * 0.49
    },
    consciousness_metrics: {
      average_consciousness_level: 0.85 + Math.random() * 0.14,
      total_interactions: Math.floor(Math.random() * 1000) + 500,
      user_satisfaction: 0.88 + Math.random() * 0.11,
      ethical_compliance: 0.95 + Math.random() * 0.04
    },
    timestamp: new Date().toISOString()
  });
});

// Error handling
app.use((err, req, res, next) => {
  console.error('Gateway Error:', err);
  res.status(500).json({
    error: 'Internal server error',
    message: err.message,
    service: 'simple-gateway',
    timestamp: new Date().toISOString()
  });
});

// 404 handler
app.use('*', (req, res) => {
  res.status(404).json({
    error: 'Not Found',
    message: `Endpoint ${req.method} ${req.originalUrl} not found`,
    available_endpoints: [
      'GET /health',
      'POST /api/v1/consciousness/process',
      'GET /api/v1/consciousness/state',
      'GET /api/v1/agents',
      'GET /api/v1/governance/policies',
      'GET /api/v1/metrics'
    ],
    timestamp: new Date().toISOString()
  });
});

const port = process.env.PORT || 3000;
app.listen(port, () => {
  console.log(`🌐 Simple API Gateway running on port ${port}`);
  console.log(`📊 Health check: http://localhost:${port}/health`);
  console.log(`🔗 Proxying consciousness requests to: ${CONSCIOUSNESS_ENGINE_URL}`);
  console.log(`📋 Available endpoints:`);
  console.log(`   - POST /api/v1/consciousness/process`);
  console.log(`   - GET  /api/v1/consciousness/state`);
  console.log(`   - GET  /api/v1/agents`);
  console.log(`   - GET  /api/v1/governance/policies`);
  console.log(`   - GET  /api/v1/metrics`);
});
