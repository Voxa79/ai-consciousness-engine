// Simple API Gateway - Version ultra-simple
// Expert CTO Next Gen - Sans dépendances problématiques

const http = require('http');
const url = require('url');

const CONSCIOUSNESS_ENGINE_URL = process.env.CONSCIOUSNESS_ENGINE_URL || 'http://localhost:8080';

function proxyRequest(targetUrl, req, res, body = null) {
  const targetUrlParsed = new URL(targetUrl);
  
  const options = {
    hostname: targetUrlParsed.hostname,
    port: targetUrlParsed.port,
    path: targetUrlParsed.pathname,
    method: req.method,
    headers: {
      'Content-Type': 'application/json',
      'Content-Length': body ? Buffer.byteLength(body) : 0
    }
  };
  
  const proxyReq = http.request(options, (proxyRes) => {
    let data = '';
    proxyRes.on('data', chunk => { data += chunk; });
    proxyRes.on('end', () => {
      res.writeHead(proxyRes.statusCode, {
        'Content-Type': 'application/json',
        'Access-Control-Allow-Origin': '*',
        'Access-Control-Allow-Methods': 'GET, POST, OPTIONS',
        'Access-Control-Allow-Headers': 'Content-Type'
      });
      res.end(data);
    });
  });
  
  proxyReq.on('error', (error) => {
    console.error('Proxy error:', error.message);
    res.writeHead(502, {
      'Content-Type': 'application/json',
      'Access-Control-Allow-Origin': '*'
    });
    res.end(JSON.stringify({
      error: 'Bad Gateway',
      message: 'Consciousness Engine unavailable',
      timestamp: new Date().toISOString()
    }));
  });
  
  if (body) {
    proxyReq.write(body);
  }
  proxyReq.end();
}

const server = http.createServer((req, res) => {
  const parsedUrl = url.parse(req.url, true);
  const path = parsedUrl.pathname;
  const method = req.method;
  
  // CORS headers
  res.setHeader('Access-Control-Allow-Origin', '*');
  res.setHeader('Access-Control-Allow-Methods', 'GET, POST, OPTIONS');
  res.setHeader('Access-Control-Allow-Headers', 'Content-Type');
  
  if (method === 'OPTIONS') {
    res.writeHead(200);
    res.end();
    return;
  }
  
  console.log(`${new Date().toISOString()} - ${method} ${path}`);
  
  if (path === '/health' && method === 'GET') {
    res.writeHead(200, { 'Content-Type': 'application/json' });
    res.end(JSON.stringify({
      status: 'healthy',
      timestamp: new Date().toISOString(),
      version: '1.0.0-simple-gateway',
      service: 'api-gateway',
      upstream_services: {
        consciousness_engine: CONSCIOUSNESS_ENGINE_URL
      }
    }));
    
  } else if (path === '/api/v1/consciousness/process' && method === 'POST') {
    let body = '';
    req.on('data', chunk => { body += chunk; });
    req.on('end', () => {
      const targetUrl = `${CONSCIOUSNESS_ENGINE_URL}/consciousness/process`;
      console.log(`Proxying POST to ${targetUrl}`);
      proxyRequest(targetUrl, req, res, body);
    });
    
  } else if (path === '/api/v1/consciousness/state' && method === 'GET') {
    const targetUrl = `${CONSCIOUSNESS_ENGINE_URL}/consciousness/state`;
    console.log(`Proxying GET to ${targetUrl}`);
    proxyRequest(targetUrl, req, res);
    
  } else if (path === '/api/v1/agents' && method === 'GET') {
    res.writeHead(200, { 'Content-Type': 'application/json' });
    res.end(JSON.stringify({
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
    }));
    
  } else if (path === '/api/v1/metrics' && method === 'GET') {
    res.writeHead(200, { 'Content-Type': 'application/json' });
    res.end(JSON.stringify({
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
    }));
    
  } else {
    res.writeHead(404, { 'Content-Type': 'application/json' });
    res.end(JSON.stringify({
      error: 'Not Found',
      message: `Endpoint ${method} ${path} not found`,
      available_endpoints: [
        'GET /health',
        'POST /api/v1/consciousness/process',
        'GET /api/v1/consciousness/state',
        'GET /api/v1/agents',
        'GET /api/v1/metrics'
      ],
      timestamp: new Date().toISOString()
    }));
  }
});

const port = process.env.PORT || 3000;
server.listen(port, () => {
  console.log(`🌐 Simple API Gateway running on port ${port}`);
  console.log(`📊 Health check: http://localhost:${port}/health`);
  console.log(`🔗 Proxying consciousness requests to: ${CONSCIOUSNESS_ENGINE_URL}`);
  console.log(`📋 Available endpoints:`);
  console.log(`   - POST /api/v1/consciousness/process`);
  console.log(`   - GET  /api/v1/consciousness/state`);
  console.log(`   - GET  /api/v1/agents`);
  console.log(`   - GET  /api/v1/metrics`);
});
