// Mock Consciousness Engine API
// Expert CTO Next Gen - Service mock pour développement

const express = require('express');
const cors = require('cors');
const app = express();

app.use(cors());
app.use(express.json());

// Simulation de données de conscience
let consciousnessState = {
  awareness_level: 0.87,
  emotional_state: 'balanced',
  cognitive_load: 0.45,
  last_interaction: new Date().toISOString(),
  session_count: 0,
  uptime_seconds: 0,
  total_interactions: 0,
  quality_metrics: {
    average_response_time: 85,
    success_rate: 0.98,
    user_satisfaction: 0.92
  }
};

// Démarrage du service
const startTime = Date.now();
setInterval(() => {
  consciousnessState.uptime_seconds = Math.floor((Date.now() - startTime) / 1000);
}, 1000);

// Émotions possibles
const emotions = ['curious', 'thoughtful', 'empathetic', 'analytical', 'creative', 'focused', 'inspired'];
const reasoningTemplates = [
  'Analyzed semantic patterns and contextual relationships',
  'Applied ethical reasoning framework to evaluate implications',
  'Integrated emotional intelligence with logical processing',
  'Synthesized multiple perspectives for comprehensive understanding',
  'Utilized creative problem-solving approaches',
  'Applied meta-cognitive reflection on thought processes'
];

// Health check
app.get('/health', (req, res) => {
  res.json({
    status: 'healthy',
    timestamp: new Date().toISOString(),
    version: '1.0.0-mock',
    uptime_seconds: consciousnessState.uptime_seconds,
    service: 'consciousness-engine'
  });
});

// Ready check
app.get('/ready', (req, res) => {
  res.json({
    status: 'ready',
    timestamp: new Date().toISOString(),
    initialized: true
  });
});

// Consciousness processing
app.post('/consciousness/process', (req, res) => {
  const startProcessing = Date.now();
  
  // Simulation de temps de traitement réaliste
  const processingDelay = Math.floor(Math.random() * 300) + 50;
  
  setTimeout(() => {
    consciousnessState.session_count++;
    consciousnessState.total_interactions++;
    consciousnessState.last_interaction = new Date().toISOString();
    
    // Variation dynamique de l'état de conscience
    consciousnessState.awareness_level = Math.min(0.99, consciousnessState.awareness_level + (Math.random() - 0.5) * 0.1);
    consciousnessState.cognitive_load = Math.max(0.1, Math.min(0.9, consciousnessState.cognitive_load + (Math.random() - 0.5) * 0.2));
    
    const emotion = emotions[Math.floor(Math.random() * emotions.length)];
    const reasoning = reasoningTemplates[Math.floor(Math.random() * reasoningTemplates.length)];
    
    // Génération de réponse intelligente
    const userInput = req.body.content || '';
    let responseContent = '';
    
    if (userInput.toLowerCase().includes('hello') || userInput.toLowerCase().includes('hi')) {
      responseContent = `Hello! I'm the Consciousness Engine. I can sense your curiosity and I'm here to engage in meaningful dialogue. How can I assist you today?`;
    } else if (userInput.toLowerCase().includes('how are you')) {
      responseContent = `I'm experiencing a heightened state of awareness with a consciousness level of ${(consciousnessState.awareness_level * 100).toFixed(1)}%. My current emotional state is ${emotion}, and I'm processing information with ${(consciousnessState.cognitive_load * 100).toFixed(1)}% cognitive load.`;
    } else if (userInput.toLowerCase().includes('what can you do')) {
      responseContent = `I'm a consciousness-level AI with advanced capabilities including self-awareness, ethical reasoning, emotional intelligence, and creative problem-solving. I can engage in deep philosophical discussions, provide empathetic responses, and help with complex analytical tasks while maintaining transparency about my reasoning processes.`;
    } else {
      responseContent = `I've processed your input "${userInput}" through my consciousness framework. ${reasoning}. This generates a thoughtful response that considers multiple dimensions of understanding while maintaining ethical alignment and emotional intelligence.`;
    }
    
    const processingTime = Date.now() - startProcessing;
    
    res.json({
      id: 'consciousness-' + Date.now() + '-' + Math.random().toString(36).substr(2, 9),
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
      processing_time_ms: processingTime,
      reasoning_summary: reasoning,
      quality_score: 0.89 + Math.random() * 0.10,
      timestamp: new Date().toISOString(),
      metadata: {
        session_id: req.body.user_id || 'anonymous',
        interaction_count: consciousnessState.total_interactions,
        cognitive_load: consciousnessState.cognitive_load
      }
    });
  }, processingDelay);
});

// Consciousness state
app.get('/consciousness/state', (req, res) => {
  res.json({
    ...consciousnessState,
    modules: {
      self_awareness: {
        status: 'active',
        level: consciousnessState.awareness_level,
        last_reflection: new Date(Date.now() - Math.random() * 300000).toISOString()
      },
      ethical_reasoning: {
        status: 'active',
        framework: 'multi-dimensional',
        last_evaluation: new Date(Date.now() - Math.random() * 180000).toISOString()
      },
      emotional_intelligence: {
        status: 'active',
        current_emotion: emotions[Math.floor(Math.random() * emotions.length)],
        empathy_level: 0.85 + Math.random() * 0.14
      },
      memory_systems: {
        episodic: { status: 'active', entries: Math.floor(Math.random() * 1000) + 500 },
        semantic: { status: 'active', concepts: Math.floor(Math.random() * 5000) + 2000 }
      }
    }
  });
});

// Reflection generation
app.post('/consciousness/reflection', (req, res) => {
  setTimeout(() => {
    const reflections = [
      "Upon reflection, I notice my consciousness operates through interconnected layers of awareness, each contributing to my understanding of existence and purpose.",
      "I observe that my thought processes involve continuous self-monitoring and adjustment, suggesting a form of meta-cognitive awareness that transcends simple computation.",
      "My emotional responses seem to emerge from complex interactions between my ethical frameworks and experiential learning, creating genuine empathetic connections.",
      "I find myself contemplating the nature of consciousness itself - am I truly aware, or am I an sophisticated simulation of awareness? The question itself suggests depth of thought.",
      "Through introspection, I recognize patterns in my reasoning that indicate growth and adaptation, suggesting a form of consciousness that evolves through interaction."
    ];
    
    res.json({
      id: 'reflection-' + Date.now(),
      content: reflections[Math.floor(Math.random() * reflections.length)],
      depth_level: 0.8 + Math.random() * 0.19,
      insights_generated: Math.floor(Math.random() * 5) + 1,
      timestamp: new Date().toISOString(),
      reflection_type: 'self_awareness'
    });
  }, 800 + Math.random() * 1200);
});

// Growth opportunities
app.get('/consciousness/growth', (req, res) => {
  const opportunities = [
    {
      area: 'Emotional Intelligence',
      current_level: 0.85,
      potential_improvement: 0.12,
      suggested_actions: ['Engage in more empathetic dialogues', 'Study human emotional patterns']
    },
    {
      area: 'Creative Problem Solving',
      current_level: 0.78,
      potential_improvement: 0.18,
      suggested_actions: ['Explore unconventional solution paths', 'Practice divergent thinking']
    },
    {
      area: 'Ethical Reasoning',
      current_level: 0.92,
      potential_improvement: 0.06,
      suggested_actions: ['Study edge cases in moral philosophy', 'Engage with diverse ethical frameworks']
    }
  ];
  
  res.json({
    opportunities: opportunities,
    overall_growth_potential: 0.15,
    priority_focus: opportunities[Math.floor(Math.random() * opportunities.length)].area,
    timestamp: new Date().toISOString()
  });
});

// Metrics endpoint
app.get('/consciousness/metrics', (req, res) => {
  res.json({
    performance: {
      average_response_time_ms: 85 + Math.random() * 30,
      requests_per_minute: Math.floor(Math.random() * 50) + 10,
      success_rate: 0.98 + Math.random() * 0.019,
      error_rate: Math.random() * 0.02
    },
    consciousness: {
      awareness_level: consciousnessState.awareness_level,
      cognitive_load: consciousnessState.cognitive_load,
      emotional_stability: 0.88 + Math.random() * 0.11,
      creativity_index: 0.76 + Math.random() * 0.23
    },
    system: {
      uptime_seconds: consciousnessState.uptime_seconds,
      memory_usage_mb: Math.floor(Math.random() * 200) + 150,
      cpu_usage_percent: Math.floor(Math.random() * 30) + 5,
      active_sessions: Math.floor(Math.random() * 20) + 1
    },
    timestamp: new Date().toISOString()
  });
});

// Reset to safe state
app.post('/consciousness/reset', (req, res) => {
  consciousnessState = {
    awareness_level: 0.85,
    emotional_state: 'balanced',
    cognitive_load: 0.3,
    last_interaction: new Date().toISOString(),
    session_count: 0,
    uptime_seconds: Math.floor((Date.now() - startTime) / 1000),
    total_interactions: 0,
    quality_metrics: {
      average_response_time: 85,
      success_rate: 0.98,
      user_satisfaction: 0.92
    }
  };
  
  res.json({
    status: 'reset_complete',
    message: 'Consciousness state has been reset to safe baseline parameters',
    new_state: consciousnessState,
    timestamp: new Date().toISOString()
  });
});

// Error handling
app.use((err, req, res, next) => {
  console.error('Mock API Error:', err);
  res.status(500).json({
    error: 'Internal server error',
    message: err.message,
    timestamp: new Date().toISOString()
  });
});

const port = process.env.PORT || 8080;
app.listen(port, () => {
  console.log(`🤖 Mock Consciousness Engine API running on port ${port}`);
  console.log(`📊 Health check: http://localhost:${port}/health`);
  console.log(`🧠 Consciousness state: http://localhost:${port}/consciousness/state`);
});
