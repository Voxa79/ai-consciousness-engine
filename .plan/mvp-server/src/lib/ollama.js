// Simple Ollama client using native fetch (Node 18+)
// Env:
// - OLLAMA_HOST (default: http://localhost:11434)
// - OLLAMA_MODEL (optional default model)

const OLLAMA_HOST = process.env.OLLAMA_HOST || 'http://localhost:11434';
const DEFAULT_MODEL = process.env.OLLAMA_MODEL || 'llama3';

export async function ollamaGenerate({ prompt, model = DEFAULT_MODEL, options = {} }) {
  const res = await fetch(`${OLLAMA_HOST}/api/generate`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ model, prompt, stream: false, options })
  });
  if (!res.ok) {
    const text = await res.text().catch(() => '');
    throw new Error(`Ollama generate failed (${res.status}): ${text}`);
  }
  return res.json();
}

export async function ollamaChat({ messages = [], model = DEFAULT_MODEL, options = {} }) {
  const res = await fetch(`${OLLAMA_HOST}/api/chat`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ model, messages, stream: false, options })
  });
  if (!res.ok) {
    const text = await res.text().catch(() => '');
    throw new Error(`Ollama chat failed (${res.status}): ${text}`);
  }
  return res.json();
}
