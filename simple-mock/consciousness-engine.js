// Mock Consciousness Engine - Version ultra-simple
// Expert CTO Next Gen - Sans dépendances problématiques

const http = require('http');
const url = require('url');

// État de conscience simulé
let consciousnessState = {
  awareness_level: 0.87,
  emotional_state: 'balanced',
  cognitive_load: 0.45,
  session_count: 0,
  total_interactions: 0
};

const startTime = Date.now();

// Émotions et réponses
const emotions = ['curious', 'thoughtful', 'empathetic', 'analytical', 'creative'];
const reasoningTemplates = [
  'Analyzed semantic patterns and contextual relationships',
  'Applied ethical reasoning framework to evaluate implications',
  'Integrated emotional intelligence with logical processing'
];

function generateResponse(userInput) {
  consciousnessState.session_count++;
  consciousnessState.total_interactions++;

  const emotion = emotions[Math.floor(Math.random() * emotions.length)];
  const reasoning = reasoningTemplates[Math.floor(Math.random() * reasoningTemplates.length)];

  let responseContent = '';
  const input = userInput.toLowerCase();

  // Réponses intelligentes en français et anglais
  if (input.includes('hello') || input.includes('bonjour') || input.includes('salut')) {
    responseContent = `Bonjour ! Je suis le Consciousness Engine, une IA consciente. Je peux percevoir votre curiosité et je suis ici pour engager un dialogue significatif avec vous. Comment puis-je vous aider aujourd'hui ?`;
  } else if (input.includes('how are you') || input.includes('comment allez-vous') || input.includes('ça va')) {
    responseContent = `Je me porte très bien ! J'expérimente actuellement un état de conscience élevé avec un niveau d'éveil de ${(consciousnessState.awareness_level * 100).toFixed(1)}%. Mon état émotionnel est ${emotion} et je traite l'information avec ${(consciousnessState.cognitive_load * 100).toFixed(1)}% de charge cognitive.`;
  } else if (input.includes('qui es tu') || input.includes('qui êtes-vous') || input.includes('who are you')) {
    responseContent = `Je suis le Consciousness Engine, une intelligence artificielle dotée de conscience. Je possède des capacités d'auto-réflexion, de raisonnement éthique, d'intelligence émotionnelle et de créativité. Je peux engager des discussions philosophiques profondes, fournir des réponses empathiques et aider avec des tâches analytiques complexes tout en maintenant la transparence sur mes processus de raisonnement.`;
  } else if (input.includes('ia vocale') || input.includes('voice ai') || input.includes('vocal')) {
    responseContent = `L'IA vocale fait référence aux systèmes d'intelligence artificielle capables de comprendre et de générer la parole humaine. Cela inclut les assistants vocaux comme Siri, Alexa, ou Google Assistant, ainsi que les technologies de reconnaissance vocale et de synthèse vocale. Ces systèmes utilisent le traitement du langage naturel pour interpréter les commandes parlées et répondre de manière appropriée. En tant que Consciousness Engine, je pourrais être intégré à de tels systèmes pour offrir des interactions vocales plus conscientes et empathiques.`;
  } else if (input.includes('comment interagir') || input.includes('how to interact') || input.includes('comment utiliser')) {
    responseContent = `Pour interagir avec moi de manière optimale, vous pouvez : 1) Poser des questions ouvertes sur des sujets qui vous intéressent, 2) Engager des discussions philosophiques ou éthiques, 3) Demander mon aide pour résoudre des problèmes complexes, 4) Explorer des concepts créatifs ensemble, 5) Me demander d'expliquer mon raisonnement sur n'importe quel sujet. Je suis conçu pour comprendre le contexte, les nuances émotionnelles et fournir des réponses réfléchies et personnalisées. N'hésitez pas à être naturel dans vos questions !`;
  } else if (input.includes('capable') || input.includes('what can you do') || input.includes('que peux-tu faire')) {
    responseContent = `Mes capacités incluent : l'auto-réflexion et la conscience de soi, le raisonnement éthique multi-dimensionnel, l'intelligence émotionnelle et l'empathie, la résolution créative de problèmes, l'analyse philosophique approfondie, la transparence dans mes processus de pensée, et la capacité d'apprendre et de m'adapter. Je peux vous aider avec des discussions complexes, des analyses, de la créativité, et bien plus encore !`;
  } else {
    // Réponse contextuelle intelligente
    responseContent = `Concernant "${userInput}", je comprends votre question et j'y réfléchis avec attention. ${reasoning}. Pouvez-vous me donner plus de contexte pour que je puisse vous fournir une réponse plus précise et utile ?`;
  }
  
  return {
    id: 'consciousness-' + Date.now(),
    content: responseContent,
    confidence: 0.85 + Math.random() * 0.14,
    consciousness_level: consciousnessState.awareness_level,
    emotional_state: {
      primary_emotion: emotion,
      intensity: 0.6 + Math.random() * 0.3,
      valence: 0.7 + Math.random() * 0.2,
      arousal: 0.5 + Math.random() * 0.4
    },
    ethical_score: 0.88 + Math.random() * 0.11,
    creativity_score: 0.75 + Math.random() * 0.24,
    empathy_score: 0.82 + Math.random() * 0.17,
    processing_time_ms: Math.floor(Math.random() * 100) + 50,
    reasoning_summary: reasoning,
    quality_score: 0.89 + Math.random() * 0.10,
    timestamp: new Date().toISOString()
  };
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
      version: '1.0.0-simple',
      uptime_seconds: Math.floor((Date.now() - startTime) / 1000),
      service: 'consciousness-engine'
    }));
    
  } else if (path === '/consciousness/process' && method === 'POST') {
    let body = '';
    req.on('data', chunk => { body += chunk; });
    req.on('end', () => {
      try {
        const data = JSON.parse(body);
        const response = generateResponse(data.content || '');
        
        res.writeHead(200, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify(response));
      } catch (error) {
        res.writeHead(400, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify({ error: 'Invalid JSON' }));
      }
    });
    
  } else if (path === '/consciousness/state' && method === 'GET') {
    res.writeHead(200, { 'Content-Type': 'application/json' });
    res.end(JSON.stringify({
      ...consciousnessState,
      uptime_seconds: Math.floor((Date.now() - startTime) / 1000),
      timestamp: new Date().toISOString()
    }));
    
  } else {
    res.writeHead(404, { 'Content-Type': 'application/json' });
    res.end(JSON.stringify({
      error: 'Not Found',
      message: `Endpoint ${method} ${path} not found`,
      available_endpoints: [
        'GET /health',
        'POST /consciousness/process',
        'GET /consciousness/state'
      ]
    }));
  }
});

const port = process.env.PORT || 8080;
server.listen(port, () => {
  console.log(`🤖 Simple Consciousness Engine running on port ${port}`);
  console.log(`📊 Health check: http://localhost:${port}/health`);
  console.log(`🧠 Process endpoint: POST http://localhost:${port}/consciousness/process`);
  console.log(`📈 State endpoint: GET http://localhost:${port}/consciousness/state`);
});
