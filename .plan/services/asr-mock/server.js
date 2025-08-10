import express from 'express';

const app = express();
app.use(express.json({ limit: '2mb' }));

app.get('/health', (_req, res) => res.json({ status: 'ok' }));

app.post('/transcribe', async (req, res) => {
  try {
    const { audio_url, language = 'fr', task = 'transcribe' } = req.body || {};
    if (!audio_url) return res.status(400).json({ error: 'audio_url required' });
    // Mock transcription content
    const demo = language.startsWith('fr') ? 'Ceci est une transcription de démonstration.' : 'This is a demo transcription.';
    return res.json({ text: demo, language, task });
  } catch (e) {
    return res.status(500).json({ error: String(e?.message || e) });
  }
});

const PORT = process.env.PORT || 8000;
app.listen(PORT, () => console.log(`[asr-mock] listening on ${PORT}`));
