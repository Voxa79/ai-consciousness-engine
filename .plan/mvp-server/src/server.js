import express from 'express';
import dotenv from 'dotenv';
import { createClient } from '@supabase/supabase-js';
import swaggerUi from 'swagger-ui-express';
import { thresholds, evaluateQuality, evaluateEthicsCompliance, evaluatePerformance } from './lib/validators.js';
import { ollamaChat, ollamaGenerate } from './lib/ollama.js';

dotenv.config();

const app = express();
// Augmente la limite pour accepter des images encodées en base64 dans le payload (best-practice CI)
app.use(express.json({ limit: '5mb' }));

const PORT = process.env.PORT || 8787;
const MODE = (process.env.MODE || 'audit').toLowerCase();

const supabaseUrl = process.env.SUPABASE_URL;
const supabaseKey = process.env.SUPABASE_SERVICE_ROLE_KEY || process.env.SUPABASE_ANON_KEY;
const supabase = (supabaseUrl && supabaseKey)
  ? createClient(supabaseUrl, supabaseKey)
  : null;

// Minimal OpenAPI spec (can be extended later)
const openapiSpec = {
  openapi: '3.0.3',
  info: {
    title: 'Kiro MVP Server API',
    version: '0.1.0',
    description: 'API alignée avec les standards Steering (qualité, conformité, performance)'
  },
  servers: [{ url: `http://localhost:${process.env.PORT || 8787}` }],
  paths: {
    '/health': {
      get: {
        summary: 'Etat du service',
        responses: {
          '200': {
            description: 'OK',
            content: { 'application/json': { schema: { type: 'object', properties: {
              status: { type: 'string' }, timestamp: { type: 'string', format: 'date-time' }, mode: { type: 'string' }, db: { type: 'string' }
            } } } }
          }
        }
      }
    },
    '/quality/validate': {
      post: {
        summary: 'Valider la qualité de conscience',
        requestBody: {
          required: true,
          content: { 'application/json': { schema: { type: 'object', properties: {
            agentType: { type: 'string' }, changeId: { type: 'string' },
            scores: { type: 'object', properties: {
              selfAwareness: { type: 'number' }, ethical: { type: 'number' }, metaCognitiveDepth: { type: 'number' }, empathyAuthenticity: { type: 'number' },
              metaCognitive: { type: 'number' }, empathy: { type: 'number' }
            } },
            latencies: { type: 'object', properties: {
              assessmentMs: { type: 'number' }, metaCognitiveMs: { type: 'number' }, responseMs: { type: 'number' }
            } }
          } } } }
        },
        responses: { '200': { description: 'Résultat', content: { 'application/json': { schema: { type: 'object', properties: {
          approved: { type: 'boolean' }, composite_score: { type: 'number' }, reasons: { type: 'array', items: { type: 'object' } }, recommendations: { type: 'array', items: { type: 'object' } }
        } } } } }, '422': { description: 'Rejeté (enforcement)' } }
      }
    },
    '/compliance/check': {
      post: {
        summary: 'Contrôle de conformité éthique et réglementaire',
        requestBody: { required: true, content: { 'application/json': { schema: { type: 'object', properties: {
          decision: { type: 'object' },
          frameworks: { type: 'object' },
          resolution: { type: 'object' },
          gdpr: { type: 'object' },
          aiAct: { type: 'object' }, ai_act: { type: 'object' }
        } } } } },
        responses: { '200': { description: 'Résultat', content: { 'application/json': { schema: { type: 'object', properties: {
          compliant: { type: 'boolean' }, composite_ethics_score: { type: 'number' }, violations: { type: 'array', items: { type: 'object' } }, requires_human_review: { type: 'boolean' }
        } } } } }, '403': { description: 'Interdit (enforcement)' } }
      }
    },
    '/performance/trigger': {
      post: {
        summary: 'Déclencheur d’optimisation de performance',
        requestBody: { required: true, content: { 'application/json': { schema: { type: 'object', properties: {
          metrics: { type: 'object', properties: { p50: { type: 'number' }, p95: { type: 'number' }, p99: { type: 'number' }, cpu: { type: 'number' }, memory: { type: 'number' }, memory_mb: { type: 'number' } } },
          patterns: { type: 'object' }
        } } } } },
        responses: { '200': { description: 'Evaluation', content: { 'application/json': { schema: { type: 'object', properties: {
          should_optimize: { type: 'boolean' }, target_engine: { type: 'string', nullable: true }, alerts: { type: 'array', items: { type: 'object' } }, suggestions: { type: 'array', items: { type: 'object' } }
        } } } } } }
      }
    },
    '/llm/health': {
      get: {
        summary: 'Etat du LLM (Ollama)',
        responses: { '200': { description: 'OK', content: { 'application/json': { schema: { type: 'object', properties: {
          status: { type: 'string' }, host: { type: 'string' }, models_count: { type: 'number' }, models: { type: 'array', items: { type: 'string' } }
        } } } } }, '503': { description: 'Indisponible' } }
      }
    }
    ,
    '/vision/describe': {
      post: {
        summary: 'Décrire une image via un VLM (Ollama)',
        requestBody: { required: true, content: { 'application/json': { schema: { 
          type: 'object', 
          description: 'Fournir soit image_url (HTTP/HTTPS), soit image_base64 (data URL).',
          properties: {
            image_url: { type: 'string' },
            image_base64: { type: 'string', description: 'Data URL (ex: data:image/png;base64,...) ou chaîne base64 brute' },
            prompt: { type: 'string', example: 'Décris l’image de façon concise.' },
            model: { type: 'string', example: 'llava:7b' },
            options: { type: 'object' }
          },
          anyOf: [
            { required: ['image_url'] },
            { required: ['image_base64'] }
          ]
        } } } },
        responses: { '200': { description: 'Description générée', content: { 'application/json': { schema: { type: 'object', properties: { text: { type: 'string' } } } } } }, '503': { description: 'Service indisponible' } }
      }
    }
    ,
    '/asr/transcribe': {
      post: {
        summary: 'Transcription audio (ASR)',
        requestBody: { required: true, content: { 'application/json': { schema: { type: 'object', properties: {
          audio_url: { type: 'string' },
          language: { type: 'string', example: 'fr' },
          task: { type: 'string', example: 'transcribe' }
        }, required: ['audio_url'] } } } },
        responses: { '200': { description: 'Transcription', content: { 'application/json': { schema: { type: 'object', properties: { text: { type: 'string' } } } } } }, '503': { description: 'Service indisponible' } }
      }
    }
    ,
    '/tts/synthesize': {
      post: {
        summary: 'Synthèse vocale (TTS)',
        requestBody: { required: true, content: { 'application/json': { schema: { type: 'object', properties: {
          text: { type: 'string' },
          voice: { type: 'string', example: 'fr-FR' },
          format: { type: 'string', example: 'wav' }
        }, required: ['text'] } } } },
        responses: { '200': { description: 'Audio encodé (base64)', content: { 'application/json': { schema: { type: 'object', properties: { audio_base64: { type: 'string' }, mime_type: { type: 'string' } } } } } }, '503': { description: 'Service indisponible' } }
      }
    }
    ,
    '/orchestrator/mock-sse': {
      get: {
        summary: 'Flux SSE mock (start/token/complete)',
        description: 'Orchestrateur minimal de démonstration: émet un flux SSE paramétrable pour valider le pipeline en temps réel.',
        parameters: [
          { name: 'tokens', in: 'query', schema: { type: 'integer', minimum: 1, default: 40 }, description: 'Nombre de tokens à émettre' },
          { name: 'interval_ms', in: 'query', schema: { type: 'integer', minimum: 10, default: 50 }, description: 'Intervalle entre tokens (ms)' },
          { name: 'id', in: 'query', schema: { type: 'string' }, description: 'Identifiant d\'exécution personnalisé' },
          { name: 'topic', in: 'query', schema: { type: 'string' }, description: 'Topic à inclure dans les événements (sinon dérivé de id)' }
        ],
        responses: {
          '200': {
            description: 'SSE stream',
            content: {
              'text/event-stream': { schema: { type: 'string', example: 'event: message\ndata: {"type":"start","id":"abc","timestamp":"..."}\n\n' } }
            }
          }
        }
      }
    }
    ,
    '/llm/generate': {
      post: {
        summary: 'Génération texte (Ollama /api/generate)',
        requestBody: { required: true, content: { 'application/json': { schema: { type: 'object', properties: {
          prompt: { type: 'string' }, model: { type: 'string' }, options: { type: 'object' }
        }, required: ['prompt'] } } } },
        responses: { '200': { description: 'Réponse Ollama generate', content: { 'application/json': { schema: { type: 'object' } } } } }
      }
    },
    '/llm/chat': {
      post: {
        summary: 'Chat (Ollama /api/chat)',
        requestBody: { required: true, content: { 'application/json': { schema: { type: 'object', properties: {
          messages: { type: 'array', items: { type: 'object' } }, model: { type: 'string' }, options: { type: 'object' }
        }, required: ['messages'] } } } },
        responses: { '200': { description: 'Réponse Ollama chat', content: { 'application/json': { schema: { type: 'object' } } } } }
      }
    },
    '/llm/stream': {
      post: {
        summary: 'Génération en streaming (SSE)',
        description: 'Proxy SSE vers Ollama /api/generate (stream: true), émet start/token/complete.',
        requestBody: { required: true, content: { 'application/json': { schema: { type: 'object', properties: {
          prompt: { type: 'string' }, model: { type: 'string' }, options: { type: 'object' }
        }, required: ['prompt'] } } } },
        responses: { '200': { description: 'Flux SSE', content: { 'text/event-stream': { schema: { type: 'string' } } } } }
      }
    }
    ,
    '/evaluate': {
      post: {
        summary: 'Orchestration unifiée: (optionnel) LLM + évaluations',
        description: 'Exécute un appel LLM (prompt/messages) si fourni, puis agrège les évaluations qualité/conformité/performance via les validateurs.',
        requestBody: {
          required: false,
          content: {
            'application/json': {
              schema: {
                type: 'object',
                properties: {
                  llm: {
                    type: 'object',
                    properties: {
                      prompt: { type: 'string' },
                      messages: { type: 'array', items: { type: 'object' } },
                      model: { type: 'string' },
                      options: { type: 'object' }
                    }
                  },
                  quality: {
                    type: 'object',
                    properties: {
                      agentType: { type: 'string' },
                      changeId: { type: 'string' },
                      scores: { type: 'object' },
                      latencies: { type: 'object' },
                      context: { type: 'object' }
                    }
                  },
                  compliance: {
                    type: 'object',
                    properties: {
                      decision: { type: 'object' },
                      frameworks: { type: 'object' },
                      resolution: { type: 'object' },
                      gdpr: { type: 'object' },
                      aiAct: { type: 'object' },
                      ai_act: { type: 'object' },
                      context: { type: 'object' }
                    }
                  },
                  performance: {
                    type: 'object',
                    properties: {
                      metrics: { type: 'object' },
                      patterns: { type: 'object' },
                      context: { type: 'object' }
                    }
                  }
                }
              }
            }
          }
        },
        responses: {
          '200': {
            description: 'Résultats agrégés',
            content: {
              'application/json': { schema: { type: 'object' } }
            }
          }
        }
      }
    }
  }
};

function ok() { return { status: 'ok', timestamp: new Date().toISOString() }; }

function requireSupabase(res) {
  if (!supabase) {
    res.status(503).json({ error: 'Supabase not configured', details: 'Set SUPABASE_URL and key in .env' });
    return false;
  }
  return true;
}

// Health endpoint
app.get('/health', async (_req, res) => {
  let db = 'unconfigured';
  if (supabase) {
    try {
      const { error } = await supabase.from('compliance_metrics').select('*').limit(1);
      db = error ? 'connected-with-errors' : 'connected';
    } catch {
      db = 'error';
    }
  }
  res.json({ ...ok(), mode: MODE, db });
});

// Diagnostic: écho du body reçu
app.post('/_echo_body', (req, res) => {
  const body = req.body || {};
  const keys = Object.keys(body);
  const sizes = {
    prompt_len: typeof body.prompt === 'string' ? body.prompt.length : null,
    image_url_len: typeof body.image_url === 'string' ? body.image_url.length : null,
    imageUrl_len: typeof body.imageUrl === 'string' ? body.imageUrl.length : null,
    image_base64_len: typeof body.image_base64 === 'string' ? body.image_base64.length : null,
    imageBase64_len: typeof body.imageBase64 === 'string' ? body.imageBase64.length : null,
  };
  try { console.log('[_echo_body] keys=', keys); } catch {}
  res.json({ keys, sizes });
});

// Vision: décrire une image via un modèle VLM dans Ollama (ex: moondream2, llava, qwen2.5-vl)
app.post('/vision/describe', async (req, res) => {
  try {
    const body = req.body || {};
    const image_url = body.image_url || body.imageUrl;
    const image_base64 = body.image_base64 || body.imageBase64;
    const prompt = (body.prompt ?? 'Décris cette image.');
    const model = (body.model ?? process.env.VLM_MODEL ?? 'moondream2');
    const options = (body.options ?? {});
    // Logs debug (ne contient pas l'image)
    try {
      console.log('[vision/describe] payload keys:', Object.keys(body));
      console.log('[vision/describe] model:', model, 'prompt_len:', String(prompt||'').length);
    } catch {}
    // Validation: exiger au moins une source d'image
    const urlOk = (typeof image_url === 'string' && image_url.length > 0);
    const b64Ok = (typeof image_base64 === 'string' && image_base64.length > 0);
    if (!urlOk && !b64Ok) {
      return res.status(400).json({ error: 'image_url ou image_base64 requis', details: { provided_keys: Object.keys(body) } });
    }
    const host = process.env.OLLAMA_HOST || 'http://localhost:11434';
    // Normalisation image pour Ollama vision:
    // - Si URL HTTP/HTTPS -> passer telle quelle (Ollama peut tirer l'image distante)
    // - Si base64 -> Ollama attend la chaîne base64 SANS préfixe data:
    let images = [];
    if (urlOk) {
      images = [image_url];
    } else if (b64Ok) {
      const raw = image_base64.trim();
      const pure = raw.startsWith('data:') ? (raw.split(',')[1] || '') : raw;
      images = [pure];
    }
    const messages = [{ role: 'user', content: prompt, images }];
    const r = await fetch(`${host}/api/chat`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ model, messages, stream: false, options })
    });
    if (!r.ok) {
      const text = await r.text().catch(() => '');
      try { console.error('[vision/describe] VLM error:', text.slice(0, 512)); } catch {}
      return res.status(503).json({ error: 'VLM call failed', details: text });
    }
    const data = await r.json().catch(() => ({}));
    // Compat: extraire le texte selon le format d’Ollama chat
    const text = data?.message?.content || data?.content || data?.response || '';
    return res.json({ text });
  } catch (e) {
    return res.status(500).json({ error: 'vision.describe failed', details: String(e?.message || e) });
  }
});

// ASR: proxy vers un service externe (ex: faster-whisper). Nécessite ASR_SERVICE_URL
app.post('/asr/transcribe', async (req, res) => {
  try {
    const ASR_URL = process.env.ASR_SERVICE_URL; // ex: http://asr:8000
    if (!ASR_URL) return res.status(503).json({ error: 'ASR indisponible', details: 'ASR_SERVICE_URL non configuré' });
    const { audio_url, language, task } = req.body || {};
    if (!audio_url) return res.status(400).json({ error: 'audio_url manquant' });
    const r = await fetch(`${ASR_URL}/transcribe`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ audio_url, language, task })
    });
    if (!r.ok) {
      const text = await r.text().catch(() => '');
      return res.status(503).json({ error: 'ASR failed', details: text });
    }
    const data = await r.json().catch(() => ({}));
    return res.json({ text: data?.text || data?.transcript || '' });
  } catch (e) {
    return res.status(500).json({ error: 'asr.transcribe failed', details: String(e?.message || e) });
  }
});

// TTS: proxy vers un service externe (ex: Piper HTTP). Nécessite TTS_SERVICE_URL
app.post('/tts/synthesize', async (req, res) => {
  try {
    const TTS_URL = process.env.TTS_SERVICE_URL; // ex: http://tts:5002
    if (!TTS_URL) return res.status(503).json({ error: 'TTS indisponible', details: 'TTS_SERVICE_URL non configuré' });
    const { text, voice = process.env.TTS_VOICE || 'fr-FR', format = 'wav' } = req.body || {};
    if (!text) return res.status(400).json({ error: 'text manquant' });
    const r = await fetch(`${TTS_URL}/synthesize`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ text, voice, format })
    });
    if (!r.ok) {
      const t = await r.text().catch(() => '');
      return res.status(503).json({ error: 'TTS failed', details: t });
    }
    const data = await r.json().catch(() => ({}));
    const audio_base64 = data?.audio_base64 || data?.audio || null;
    const mime_type = data?.mime_type || (format === 'wav' ? 'audio/wav' : 'audio/ogg');
    if (!audio_base64) return res.status(500).json({ error: 'TTS response invalide' });
    return res.json({ audio_base64, mime_type });
  } catch (e) {
    return res.status(500).json({ error: 'tts.synthesize failed', details: String(e?.message || e) });
  }
});
// Root endpoint - simple catalog
app.get('/', (_req, res) => {
  res.json({
    name: 'Kiro MVP Server',
    mode: MODE,
    endpoints: {
      health: '/health',
      quality_validate: '/quality/validate',
      compliance_check: '/compliance/check',
      performance_trigger: '/performance/trigger',
      evaluate: '/evaluate',
      llm_health: '/llm/health',
      orchestrator_mock_sse: '/orchestrator/mock-sse',
      openapi: '/openapi.json',
      docs: '/docs'
    }
  });
});

// OpenAPI JSON
app.get('/openapi.json', (_req, res) => {
  res.json(openapiSpec);
});

// Swagger UI
app.use('/docs', swaggerUi.serve, swaggerUi.setup(openapiSpec));

// LLM health (Ollama)
app.get('/llm/health', async (_req, res) => {
  try {
    const host = process.env.OLLAMA_HOST || 'http://localhost:11434';
    const r = await fetch(`${host}/api/tags`);
    if (!r.ok) throw new Error(`HTTP ${r.status}`);
    const data = await r.json().catch(() => ({}));
    const models = Array.isArray(data.models) ? data.models.map(m => m.name || m.model || 'unknown') : [];
    res.json({ status: 'ok', host, models_count: models.length, models });
  } catch (e) {
    res.status(503).json({ status: 'unavailable', error: String(e?.message || e) });
  }
});

// POST /evaluate (unifié)
app.post('/evaluate', async (req, res) => {
  try {
    const body = req.body || {};
    const t = thresholds();

    // 1) LLM (optionnel)
    let llmResult = null;
    if (body.llm) {
      const { prompt, messages, model, options } = body.llm || {};
      if (typeof prompt === 'string' && prompt.length > 0) {
        try {
          llmResult = await ollamaGenerate({ prompt, model, options });
        } catch (e) {
          llmResult = { error: 'Ollama generate failed', details: e?.message };
        }
      } else if (Array.isArray(messages) && messages.length > 0) {
        try {
          llmResult = await ollamaChat({ messages, model, options });
        } catch (e) {
          llmResult = { error: 'Ollama chat failed', details: e?.message };
        }
      }
    }

    // 2) Qualité (optionnel)
    let quality = null;
    if (body.quality) {
      const { agentType = 'generic', changeId = 'unknown', scores = {}, latencies = {}, context = {} } = body.quality || {};
      const s = {
        selfAwareness: Number(scores.selfAwareness ?? scores.self_awareness ?? 0),
        ethical: Number(scores.ethical ?? scores.ethical_score ?? 0),
        metaCognitiveDepth: Number(scores.metaCognitiveDepth ?? scores.metaCognitive ?? scores.meta_cognitive_depth ?? 0),
        empathyAuthenticity: Number(scores.empathyAuthenticity ?? scores.empathy ?? scores.empathy_authenticity ?? 0),
      };
      const l = {
        assessmentMs: Number(latencies.assessmentMs ?? latencies.assessment_ms ?? latencies.assess_ms ?? null),
        metaCognitiveMs: Number(latencies.metaCognitiveMs ?? latencies.meta_ms ?? latencies.meta_cognitive_ms ?? null),
        responseMs: Number(latencies.responseMs ?? latencies.response_ms ?? null),
      };
      const evalQ = evaluateQuality({
        selfAwareness: s.selfAwareness,
        ethical: s.ethical,
        metaCognitiveDepth: s.metaCognitiveDepth,
        empathyAuthenticity: s.empathyAuthenticity,
        latencies: { assessmentMs: l.assessmentMs, metaCognitiveMs: l.metaCognitiveMs, responseMs: l.responseMs }
      }, t);

      quality = {
        approved: evalQ.approved,
        composite_score: evalQ.compositeScore,
        reasons: evalQ.reasons,
        recommendations: evalQ.recommendations,
        agentType,
        changeId,
        context
      };
    }

    // 3) Conformité (optionnel)
    let compliance = null;
    if (body.compliance) {
      const { decision = {}, frameworks = {}, resolution = { conflicts: [], resolution: null }, gdpr = {}, aiAct = {}, ai_act = {}, context = {} } = body.compliance || {};
      const aiActNormalized = Object.keys(aiAct).length ? aiAct : ai_act;
      const evalC = evaluateEthicsCompliance({ frameworks, resolution, gdpr, aiAct: aiActNormalized }, t);
      compliance = {
        compliant: evalC.compliant,
        composite_ethics_score: evalC.compositeEthicsScore,
        conflicts: evalC.conflicts,
        resolution: evalC.resolution,
        violations: evalC.violations,
        recommendations: evalC.recommendations,
        requires_human_review: !evalC.compliant && MODE === 'audit',
        decision,
        context
      };
    }

    // 4) Performance (optionnel)
    let performance = null;
    if (body.performance) {
      const { metrics = {}, patterns = {}, context = {} } = body.performance || {};
      const normalizedMetrics = {
        p50: Number(metrics.p50 ?? metrics.latency_ms?.p50 ?? 0),
        p95: Number(metrics.p95 ?? metrics.latency_ms?.p95 ?? 0),
        p99: Number(metrics.p99 ?? metrics.latency_ms?.p99 ?? 0),
        cpu: Number(metrics.cpu ?? 0),
        memory: Number(metrics.memory ?? metrics.memory_mb ?? 0),
        energy: Number(metrics.energy ?? metrics.energy_j ?? 0),
      };
      const evalP = evaluatePerformance({ metrics: normalizedMetrics, patterns }, t);
      performance = evalP;
      performance.context = context;
    }

    const payload = {
      status: 'ok',
      timestamp: new Date().toISOString(),
      mode: MODE,
      llm: llmResult,
      quality,
      compliance,
      performance
    };

    // Enforcement minimal: si qualité non approuvée en mode enforcement
    if (MODE === 'enforcement' && quality && quality.approved === false) {
      return res.status(422).json(payload);
    }
    if (MODE === 'enforcement' && compliance && !compliance.compliant) {
      return res.status(403).json(payload);
    }

    res.json(payload);
  } catch (e) {
    res.status(500).json({ error: 'Evaluate failed', details: e?.message });
  }
});

// GET /orchestrator/mock-sse (SSE)
app.get('/orchestrator/mock-sse', async (req, res) => {
  // Headers SSE
  res.setHeader('Content-Type', 'text/event-stream');
  res.setHeader('Cache-Control', 'no-cache');
  res.setHeader('Connection', 'keep-alive');
  res.flushHeaders?.();

  const q = req.query || {};
  const total = Math.max(1, parseInt(String(q.tokens ?? '')) || 40);
  const interval = Math.max(10, parseInt(String(q.interval_ms ?? '')) || 50);
  const id = String(q.id || cryptoRandomId());
  const topic = String(q.topic || `execution:${id}`);

  const send = (obj) => {
    const payload = JSON.stringify({ topic, ...obj });
    res.write(`data: ${payload}\n\n`);
  };

  const startAt = new Date();
  send({ type: 'start', id, timestamp: startAt.toISOString() });

  let i = 0;
  const timer = setInterval(() => {
    i += 1;
    const now = new Date();
    send({ type: 'token', id, n: i, content: `tok_${i}`, timestamp: now.toISOString() });
    if (i >= total) {
      clearInterval(timer);
      const end = new Date();
      send({ type: 'complete', id, tokens: total, duration_ms: (end.getTime() - startAt.getTime()), timestamp: end.toISOString() });
      res.end();
    }
  }, interval);

  req.on('close', () => {
    clearInterval(timer);
  });
});

// LLM generate (non-stream)
app.post('/llm/generate', async (req, res) => {
  try {
    const { prompt, model, options } = req.body || {};
    if (!prompt) return res.status(400).json({ error: 'Missing prompt' });
    const result = await ollamaGenerate({ prompt, model, options });
    res.json(result);
  } catch (e) {
    res.status(502).json({ error: 'Ollama generate failed', details: e?.message });
  }
});

// LLM chat (non-stream)
app.post('/llm/chat', async (req, res) => {
  try {
    const { messages, model, options } = req.body || {};
    if (!Array.isArray(messages)) return res.status(400).json({ error: 'Missing messages[]' });
    const result = await ollamaChat({ messages, model, options });
    res.json(result);
  } catch (e) {
    res.status(502).json({ error: 'Ollama chat failed', details: e?.message });
  }
});

// LLM streaming proxy (SSE)
app.post('/llm/stream', async (req, res) => {
  try {
    const { prompt, model, options } = req.body || {};
    if (!prompt) {
      res.writeHead(400, { 'Content-Type': 'application/json' });
      return res.end(JSON.stringify({ error: 'Missing prompt' }));
    }
    res.writeHead(200, {
      'Content-Type': 'text/event-stream',
      'Cache-Control': 'no-cache',
      'Connection': 'keep-alive',
      'Access-Control-Allow-Origin': '*'
    });
    const send = (event, data) => {
      res.write(`event: ${event}\n`);
      res.write(`data: ${JSON.stringify(data)}\n\n`);
    };
    const startAt = new Date().toISOString();
    send('start', { timestamp: startAt, model: model || process.env.OLLAMA_MODEL || 'llama3' });

    const host = process.env.OLLAMA_HOST || 'http://localhost:11434';
    const body = JSON.stringify({ model: model || process.env.OLLAMA_MODEL || 'llama3', prompt, stream: true, options: options || {} });
    const r = await fetch(`${host}/api/generate`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body
    });
    if (!r.ok || !r.body) {
      send('error', { message: `Ollama stream failed: HTTP ${r.status}` });
      res.end();
      return;
    }

    let buffer = '';
    const decoder = new TextDecoder();
    for await (const chunk of r.body) {
      buffer += decoder.decode(chunk, { stream: true });
      let idx;
      while ((idx = buffer.indexOf('\n')) >= 0) {
        const line = buffer.slice(0, idx).trim();
        buffer = buffer.slice(idx + 1);
        if (!line) continue;
        try {
          const obj = JSON.parse(line);
          if (obj?.response) {
            send('token', { content: obj.response });
          }
          if (obj?.done) {
            send('complete', { timestamp: new Date().toISOString() });
            res.end();
            return;
          }
        } catch (_e) {
          // ignore malformed line
        }
      }
    }
    // if we exit loop without done
    send('complete', { timestamp: new Date().toISOString() });
    res.end();
  } catch (e) {
    try {
      res.writeHead(200, { 'Content-Type': 'text/event-stream', 'Cache-Control': 'no-cache', 'Connection': 'keep-alive' });
      res.write(`event: error\n`);
      res.write(`data: ${JSON.stringify({ message: e?.message })}\n\n`);
      res.end();
    } catch (_err) {
      res.status(500).json({ error: 'Stream failed', details: e?.message });
    }
  }
});

// POST /quality/validate
app.post('/quality/validate', async (req, res) => {
  try {
    const { agentType = 'generic', changeId = 'unknown', scores = {}, latencies = {}, context = {} } = req.body || {};
    // Normalize score keys (support legacy and new naming + snake_case)
    const s = {
      selfAwareness: Number(
        scores.selfAwareness ?? scores.self_awareness ?? 0
      ),
      ethical: Number(scores.ethical ?? scores.ethical_score ?? 0),
      metaCognitiveDepth: Number(
        scores.metaCognitiveDepth ?? scores.metaCognitive ?? scores.meta_cognitive_depth ?? 0
      ),
      empathyAuthenticity: Number(
        scores.empathyAuthenticity ?? scores.empathy ?? scores.empathy_authenticity ?? 0
      ),
    };
    // Normalize latencies (camelCase and snake_case)
    const l = {
      assessmentMs: Number(
        latencies.assessmentMs ?? latencies.assessment_ms ?? latencies.assess_ms ?? null
      ),
      metaCognitiveMs: Number(
        latencies.metaCognitiveMs ?? latencies.meta_ms ?? latencies.meta_cognitive_ms ?? null
      ),
      responseMs: Number(
        latencies.responseMs ?? latencies.response_ms ?? null
      ),
    };

    const t = thresholds();
    const evalQ = evaluateQuality({
      selfAwareness: s.selfAwareness,
      ethical: s.ethical,
      metaCognitiveDepth: s.metaCognitiveDepth,
      empathyAuthenticity: s.empathyAuthenticity,
      latencies: {
        assessmentMs: l.assessmentMs,
        metaCognitiveMs: l.metaCognitiveMs,
        responseMs: l.responseMs,
      }
    }, t);

    const payload = {
      approved: evalQ.approved,
      composite_score: evalQ.compositeScore,
      reasons: evalQ.reasons,
      recommendations: evalQ.recommendations,
    };

    if (supabase) {
      try {
        await supabase.from('consciousness_validations').insert({
          agent_type: agentType,
          change_id: changeId,
          self_awareness_score: s.selfAwareness,
          ethical_score: s.ethical,
          meta_cognitive_depth: s.metaCognitiveDepth,
          empathy_authenticity: s.empathyAuthenticity,
          latency_assess_ms: l.assessmentMs,
          latency_meta_ms: l.metaCognitiveMs,
          latency_response_ms: l.responseMs,
          composite_score: payload.composite_score,
          approved: evalQ.approved,
          reasons: payload.reasons,
          recommendations: payload.recommendations,
          environment: context.environment || 'local'
        });
      } catch (e) {
        // swallow to not fail request
        console.warn('Supabase insert (consciousness_validations) failed:', e?.message);
      }
    }
    const status = (MODE === 'enforcement' && !evalQ.approved) ? 422 : 200;
    res.status(status).json(payload);
  } catch (err) {
    res.status(400).json({ error: 'Bad Request', details: err?.message });
  }
});

// POST /compliance/check
app.post('/compliance/check', async (req, res) => {
  try {
    const { decision = {}, frameworks = {}, resolution = { conflicts: [], resolution: null }, gdpr = {}, aiAct = {}, ai_act = {}, context = {} } = req.body || {};

    // Normalize compliance keys (aiAct | ai_act)
    const aiActNormalized = Object.keys(aiAct).length ? aiAct : ai_act;

    const t = thresholds();
    const evalC = evaluateEthicsCompliance({
      frameworks,
      resolution,
      gdpr,
      aiAct: aiActNormalized,
    }, t);

    const payload = {
      compliant: evalC.compliant,
      composite_ethics_score: evalC.compositeEthicsScore,
      conflicts: evalC.conflicts,
      resolution: evalC.resolution,
      violations: evalC.violations,
      recommendations: evalC.recommendations,
      requires_human_review: !evalC.compliant && MODE === 'audit'
    };

    if (supabase) {
      try {
        await supabase.from('ethical_compliance_audits').insert({
          hook_execution_id: cryptoRandomId(),
          decision_id: decision.id || 'unknown',
          decision_type: decision.type || 'generic',
          frameworks_scores: frameworks,
          composite_ethics_score: payload.composite_ethics_score,
          gdpr: gdpr,
          ai_act: aiAct,
          conflicts: payload.conflicts,
          resolution: payload.resolution,
          compliant: payload.compliant,
          environment: context.environment || 'local'
        });

        if (!payload.compliant) {
          await supabase.from('compliance_violations').insert({
            hook_execution_id: cryptoRandomId(),
            decision_id: decision.id || 'unknown',
            decision_type: decision.type || 'generic',
            severity: 'critical',
            description: 'Compliance check failed',
            resolved: false
          });
        }
      } catch (e) {
        console.warn('Supabase insert (compliance) failed:', e?.message);
      }
    }
    const status = (MODE === 'enforcement' && (!payload.compliant)) ? 403 : 200;
    res.status(status).json(payload);
  } catch (err) {
    res.status(400).json({ error: 'Bad Request', details: err?.message });
  }
});

// POST /performance/trigger
app.post('/performance/trigger', async (req, res) => {
  try {
    const { metrics = {}, patterns = {}, context = {} } = req.body || {};
    // Normalize performance metrics
    const normalizedMetrics = {
      p50: Number(metrics.p50 ?? metrics.latency_ms?.p50 ?? 0),
      p95: Number(metrics.p95 ?? metrics.latency_ms?.p95 ?? 0),
      p99: Number(metrics.p99 ?? metrics.latency_ms?.p99 ?? 0),
      cpu: Number(metrics.cpu ?? 0),
      memory: Number(metrics.memory ?? metrics.memory_mb ?? 0),
      energy: Number(metrics.energy ?? metrics.energy_j ?? 0),
    };
    const t = thresholds();
    const evalP = evaluatePerformance({ metrics: normalizedMetrics, patterns }, t);

    // Store minimal record when an action is suggested
    if (supabase && evalP.should_optimize) {
      try {
        await supabase.from('performance_optimizations').insert({
          hook_execution_id: cryptoRandomId(),
          optimization_type: (evalP.target_engine || 'mvp'),
          start_time: new Date().toISOString(),
          end_time: new Date().toISOString(),
          execution_time_ms: 0,
          latency_reduction_percent: null,
          throughput_increase_percent: null,
          resource_efficiency_percent: null,
          power_reduction_percent: null,
          battery_life_extension_percent: null,
          quantum_advantage: null,
          spike_efficiency: null,
          confidence: evalP.confidence || 0.75,
          applied_changes: evalP,
          trigger_type: 'mvp',
          environment: context.environment || 'local'
        });
      } catch (e) {
        console.warn('Supabase insert (performance) failed:', e?.message);
      }
    }
    const status = (MODE === 'enforcement' && evalP.alerts?.some(a => a.severity === 'critical')) ? 422 : 200;
    res.status(status).json(evalP);
  } catch (err) {
    res.status(400).json({ error: 'Bad Request', details: err?.message });
  }
});

// Overview endpoints (minimal)
app.get('/quality/overview', async (_req, res) => {
  if (!requireSupabase(res)) return;
  try {
    const { data, error } = await supabase
      .from('consciousness_quality_metrics')
      .select('*')
      .gte('created_at', new Date(Date.now() - 24 * 3600 * 1000).toISOString())
      .order('created_at', { ascending: false });
    if (error) throw error;
    const average = (arr) => (arr.length ? (arr.reduce((a, b) => a + b, 0) / arr.length) : 0);
    res.json({
      averageScore: Number(average((data || []).map(d => Number(d.average_score || 0))).toFixed(2)),
      passRate: 0, // TODO: compute from validations table in later iteration
      criticalIssues: (data || []).reduce((s, d) => s + (Number(d.critical_issues || 0)), 0),
      items: data || []
    });
  } catch (e) {
    res.status(500).json({ error: 'Overview failed', details: e?.message });
  }
});

app.get('/compliance/overview', async (_req, res) => {
  if (!requireSupabase(res)) return;
  try {
    const { data, error } = await supabase
      .from('compliance_metrics')
      .select('*')
      .gte('created_at', new Date(Date.now() - 24 * 3600 * 1000).toISOString())
      .order('created_at', { ascending: false });
    if (error) throw error;
    res.json({ items: data || [] });
  } catch (e) {
    res.status(500).json({ error: 'Overview failed', details: e?.message });
  }
});

app.get('/compliance/alerts', async (_req, res) => {
  if (!requireSupabase(res)) return;
  try {
    const { data, error } = await supabase
      .from('compliance_violations')
      .select('*')
      .eq('resolved', false)
      .in('severity', ['high', 'critical'])
      .order('created_at', { ascending: false })
      .limit(25);
    if (error) throw error;
    res.json({ items: data || [] });
  } catch (e) {
    res.status(500).json({ error: 'Alerts failed', details: e?.message });
  }
});

app.get('/performance/overview', async (_req, res) => {
  if (!requireSupabase(res)) return;
  try {
    const { data, error } = await supabase
      .from('performance_optimizations')
      .select('*')
      .gte('created_at', new Date(Date.now() - 24 * 3600 * 1000).toISOString())
      .order('created_at', { ascending: false })
      .limit(50);
    if (error) throw error;
    res.json({ items: data || [] });
  } catch (e) {
    res.status(500).json({ error: 'Overview failed', details: e?.message });
  }
});

function cryptoRandomId() {
  // lightweight random id; not cryptographically strong in Node <19 without crypto.randomUUID
  const s4 = () => Math.floor((1 + Math.random()) * 0x10000).toString(16).substring(1);
  return `${s4()}${s4()}-${s4()}-${s4()}-${s4()}-${s4()}${s4()}${s4()}`;
}

app.listen(PORT, () => {
  console.log(`[mvp-server] listening on http://localhost:${PORT} (mode=${MODE})`);
});
