import express from 'express';
import http from 'http';
import { WebSocketServer } from 'ws';
import { createParser } from 'eventsource-parser';

const PORT = parseInt(process.env.PORT || '8787', 10);
const JWT_SECRET = process.env.JWT_SECRET || 'dev-secret-key';
const ASR_SERVICE_URL = process.env.ASR_SERVICE_URL || 'http://localhost:8000';
const TTS_SERVICE_URL = process.env.TTS_SERVICE_URL || 'http://localhost:5002';
const GATEWAY_URL = process.env.GATEWAY_URL || 'http://localhost:3000';

const app = express();
app.use(express.json());

app.get('/health', (_req, res) => {
  res.json({ status: 'ok', service: 'ultravox-adapter', port: PORT });
});

const server = http.createServer(app);
const wss = new WebSocketServer({ server, path: '/realtime' });

function log(...args) {
  console.log('[ultravox-adapter]', ...args);
}

// DEV: validation JWT simple (optionnelle). En prod, valider la signature.
function authOk(req) {
  try {
    const url = new URL(req.url, 'http://localhost');
    const token = req.headers['sec-websocket-protocol'] || url.searchParams.get('token') || '';
    // Accepter en dev si vide
    if (!token) return true;
    // Placeholder: en prod, vérifier JWT (exp, iss, aud, signature avec JWT_SECRET)
    return true;
  } catch {
    return false;
  }
}

wss.on('connection', async (ws, req) => {
  if (!authOk(req)) {
    ws.close(1008, 'Unauthorized');
    return;
  }
  log('client connected');

  const state = { closed: false };
  ws.on('close', () => {
    state.closed = true;
    log('client disconnected');
  });

  ws.on('message', async (raw) => {
    if (state.closed) return;
    let msg;
    try { msg = JSON.parse(raw.toString()); } catch {
      ws.send(JSON.stringify({ type: 'error', error: 'invalid_json' }));
      return;
    }

    // Types attendus au PoC:
    // - { type: 'text', text: '...' , model?: 'tinyllama' | 'qwen2.5:3b-instruct' | 'llama3.2:3b-instruct' }
    // - { type: 'audio_chunk', data: '<base64-pcm16-le-16k>' } (ACK seulement pour ce PoC)
    // - { type: 'end' } fin de requête courante

    if (msg.type === 'audio_chunk') {
      // PoC: pas d'ASR streaming encore, on ACK seulement
      ws.send(JSON.stringify({ type: 'audio_ack' }));
      return;
    }

    if (msg.type === 'text') {
      const model = msg.model || 'tinyllama';
      const prompt = String(msg.text || '').trim();
      if (!prompt) {
        ws.send(JSON.stringify({ type: 'error', error: 'empty_prompt' }));
        return;
      }
      try {
        await proxyLlmStream({ ws, model, prompt });
      } catch (e) {
        log('llm stream error', e?.message || e);
        if (!state.closed) ws.send(JSON.stringify({ type: 'error', error: 'llm_stream_failed' }));
      }
      return;
    }

    if (msg.type === 'end') {
      try { ws.close(1000, 'done'); } catch {}
      return;
    }

    ws.send(JSON.stringify({ type: 'error', error: 'unknown_message_type' }));
  });
});

async function proxyLlmStream({ ws, model, prompt }) {
  // Appelle la Gateway /api/v1/llm/stream (SSE) et relaie les tokens au client WS
  const url = `${GATEWAY_URL}/api/v1/llm/stream`;
  const body = JSON.stringify({ model, prompt, stream: true });
  const r = await fetch(url, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json', 'Accept': 'text/event-stream' },
    body,
  });
  if (!r.ok || !r.body) {
    const t = await r.text().catch(() => '');
    throw new Error(`gateway_error: ${r.status} ${t}`);
  }

  const reader = r.body.getReader();
  const decoder = new TextDecoder();
  const parser = createParser((event) => {
    if (event.type !== 'event') return;
    const data = event.data || '';
    if (data === '[DONE]') {
      try { ws.send(JSON.stringify({ type: 'llm_complete' })); } catch {}
      return;
    }
    try {
      const obj = JSON.parse(data);
      if (obj?.token) {
        ws.send(JSON.stringify({ type: 'llm_token', token: obj.token }));
      } else if (obj?.message) {
        ws.send(JSON.stringify({ type: 'llm_message', message: obj.message }));
      } else {
        ws.send(JSON.stringify({ type: 'llm_event', data: obj }));
      }
    } catch {
      // Pas JSON : envoyer brut
      ws.send(JSON.stringify({ type: 'llm_raw', data }));
    }
  });

  while (true) {
    const { value, done } = await reader.read();
    if (done) break;
    parser.feed(decoder.decode(value, { stream: true }));
  }
}

server.listen(PORT, () => {
  log(`listening on http://localhost:${PORT}`);
});
