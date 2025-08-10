import express from 'express';

const app = express();
app.use(express.json({ limit: '2mb' }));

app.get('/health', (_req, res) => res.json({ status: 'ok' }));

// Simple mock: returns base64 of a small WAV header + silence
function generateSilentWavBase64(seconds = 1, sampleRate = 16000) {
  const numSamples = seconds * sampleRate;
  const byteRate = sampleRate * 2; // 16-bit mono
  const blockAlign = 2;
  const dataSize = numSamples * 2;
  const buffer = Buffer.alloc(44 + dataSize);
  buffer.write('RIFF', 0);
  buffer.writeUInt32LE(36 + dataSize, 4);
  buffer.write('WAVE', 8);
  buffer.write('fmt ', 12);
  buffer.writeUInt32LE(16, 16); // PCM chunk size
  buffer.writeUInt16LE(1, 20); // PCM format
  buffer.writeUInt16LE(1, 22); // mono
  buffer.writeUInt32LE(sampleRate, 24);
  buffer.writeUInt32LE(byteRate, 28);
  buffer.writeUInt16LE(blockAlign, 32);
  buffer.writeUInt16LE(16, 34); // bits per sample
  buffer.write('data', 36);
  buffer.writeUInt32LE(dataSize, 40);
  // samples are zeros (silence)
  return buffer.toString('base64');
}

app.post('/synthesize', async (req, res) => {
  try {
    const { text, voice = 'fr-FR', format = 'wav' } = req.body || {};
    if (!text) return res.status(400).json({ error: 'text required' });
    if (format !== 'wav') return res.status(400).json({ error: 'only wav format supported in mock' });
    const audio_base64 = generateSilentWavBase64(1, 16000);
    return res.json({ audio_base64, mime_type: 'audio/wav', voice });
  } catch (e) {
    return res.status(500).json({ error: String(e?.message || e) });
  }
});

const PORT = process.env.PORT || 5002;
app.listen(PORT, () => console.log(`[tts-mock] listening on ${PORT}`));
