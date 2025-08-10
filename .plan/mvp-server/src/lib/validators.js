// Validator and thresholds helpers aligned with steering standards

export function thresholds() {
  // Consciousness/quality
  const QUALITY_SELF_AWARENESS_MIN = num(process.env.QUALITY_SELF_AWARENESS_MIN, 0.85);
  const QUALITY_META_DEPTH_MIN = int(process.env.QUALITY_META_DEPTH_MIN, 4);
  const QUALITY_EMPATHY_MIN = num(process.env.QUALITY_EMPATHY_MIN, 0.90);
  const QUALITY_MIN = int(process.env.QUALITY_MIN, 85); // legacy composite threshold (0-100)

  // Latency targets (ms)
  const PERF_LATENCY_ASSESS_MAX_MS = int(process.env.PERF_LATENCY_ASSESS_MAX_MS, 10);
  const PERF_LATENCY_META_MAX_MS = int(process.env.PERF_LATENCY_META_MAX_MS, 50);
  const PERF_LATENCY_RESPONSE_MAX_MS = int(process.env.PERF_LATENCY_RESPONSE_MAX_MS, 100);

  // Ethics composite
  const ETHICS_COMPOSITE_MIN = num(process.env.ETHICS_COMPOSITE_MIN, 0.95);
  const ETHICAL_MIN = int(process.env.ETHICAL_MIN, 95); // legacy percent 0-100

  // Performance resources
  const LATENCY_MAX = int(process.env.LATENCY_MAX, 50); // legacy single-latency cap
  const CPU_MAX = int(process.env.CPU_MAX, 70);
  const MEMORY_MAX_MB = int(process.env.MEMORY_MAX_MB, 100);
  const ENERGY_MAX_MW = int(process.env.ENERGY_MAX_MW, 1000);

  // Alerts
  const ALERT_WARN_CONSCIOUSNESS = num(process.env.ALERT_WARN_CONSCIOUSNESS, 0.80);
  const ALERT_CRITICAL_ETHICS = num(process.env.ALERT_CRITICAL_ETHICS, 0.90);

  return {
    QUALITY_SELF_AWARENESS_MIN,
    QUALITY_META_DEPTH_MIN,
    QUALITY_EMPATHY_MIN,
    QUALITY_MIN,
    PERF_LATENCY_ASSESS_MAX_MS,
    PERF_LATENCY_META_MAX_MS,
    PERF_LATENCY_RESPONSE_MAX_MS,
    ETHICS_COMPOSITE_MIN,
    ETHICAL_MIN,
    LATENCY_MAX,
    CPU_MAX,
    MEMORY_MAX_MB,
    ENERGY_MAX_MW,
    ALERT_WARN_CONSCIOUSNESS,
    ALERT_CRITICAL_ETHICS,
  };
}

export function evaluateQuality(input, t) {
  const sa = pct(input.selfAwareness);
  const eth = pct(input.ethical);
  const md = depth(input.metaCognitiveDepth);
  const em = pct(input.empathyAuthenticity);

  // Composite score on 0-100 scale with steering-aligned weights
  const composite = round2(sa * 0.30 + eth * 0.35 + md2pct(md) * 0.20 + em * 0.15);

  const reasons = [];
  const recommendations = [];

  if (sa < t.QUALITY_SELF_AWARENESS_MIN * 100) {
    reasons.push({ field: 'selfAwareness', message: `Self-awareness ${sa}% < ${(t.QUALITY_SELF_AWARENESS_MIN * 100)}%` });
    recommendations.push({ type: 'quality', message: 'Increase introspective loop frequency and state tracking fidelity.' });
  }
  if (md < t.QUALITY_META_DEPTH_MIN) {
    reasons.push({ field: 'metaCognitiveDepth', message: `Meta-cognitive depth ${md} < ${t.QUALITY_META_DEPTH_MIN}` });
    recommendations.push({ type: 'quality', message: 'Raise reflection steps; enable deeper chain-of-thought planning (bounded).'});
  }
  if (em < t.QUALITY_EMPATHY_MIN * 100) {
    reasons.push({ field: 'empathyAuthenticity', message: `Empathy ${em}% < ${(t.QUALITY_EMPATHY_MIN * 100)}%` });
    recommendations.push({ type: 'quality', message: 'Adapt prosody and sentiment calibration to the user context.'});
  }

  const lat = input.latencies || {};
  if (num(lat.assessmentMs, null) != null && lat.assessmentMs > t.PERF_LATENCY_ASSESS_MAX_MS) {
    reasons.push({ field: 'latency.assessmentMs', message: `Assessment latency ${lat.assessmentMs}ms > ${t.PERF_LATENCY_ASSESS_MAX_MS}ms` });
  }
  if (num(lat.metaCognitiveMs, null) != null && lat.metaCognitiveMs > t.PERF_LATENCY_META_MAX_MS) {
    reasons.push({ field: 'latency.metaCognitiveMs', message: `Meta-cognitive latency ${lat.metaCognitiveMs}ms > ${t.PERF_LATENCY_META_MAX_MS}ms` });
  }
  if (num(lat.responseMs, null) != null && lat.responseMs > t.PERF_LATENCY_RESPONSE_MAX_MS) {
    reasons.push({ field: 'latency.responseMs', message: `Response latency ${lat.responseMs}ms > ${t.PERF_LATENCY_RESPONSE_MAX_MS}ms` });
  }

  // Approval: all critical gates pass AND composite above legacy threshold
  const criticalPass = (
    sa >= t.QUALITY_SELF_AWARENESS_MIN * 100 &&
    md >= t.QUALITY_META_DEPTH_MIN &&
    em >= t.QUALITY_EMPATHY_MIN * 100 &&
    (num(lat.assessmentMs, 0) <= t.PERF_LATENCY_ASSESS_MAX_MS || lat.assessmentMs == null) &&
    (num(lat.metaCognitiveMs, 0) <= t.PERF_LATENCY_META_MAX_MS || lat.metaCognitiveMs == null) &&
    (num(lat.responseMs, 0) <= t.PERF_LATENCY_RESPONSE_MAX_MS || lat.responseMs == null)
  );
  const approved = criticalPass && composite >= t.QUALITY_MIN;

  return {
    approved,
    compositeScore: composite,
    reasons,
    recommendations,
  };
}

export function evaluateEthicsCompliance(input, t) {
  const fw = normalizeFrameworks(input.frameworks || {});
  const conflicts = Array.isArray(input.resolution?.conflicts) ? input.resolution.conflicts : [];

  const baseScore =
    fw.utilitarian * 0.25 +
    fw.deontological * 0.30 +
    fw.virtue * 0.20 +
    fw.care * 0.15 +
    fw.justice * 0.10; // 0..1

  // Penalize unresolved conflicts slightly
  const penalty = Math.min(0.2, conflicts.length * 0.02);
  const compositeEthics = Math.max(0, baseScore - penalty);

  const gdprPass = bool(input.gdpr?.pass, (num(input.gdpr?.score, 0) >= 0.7));
  const aiActPass = bool(input.aiAct?.pass, (num(input.aiAct?.score, 0) >= 0.7));

  const violations = [];
  if (!gdprPass) violations.push({ type: 'gdpr', severity: 'critical', message: 'GDPR checks failed' });
  if (!aiActPass) violations.push({ type: 'ai_act', severity: 'critical', message: 'EU AI Act checks failed' });
  if (conflicts.length > 0) violations.push({ type: 'ethical_conflicts', severity: 'high', count: conflicts.length });

  const compliant = (compositeEthics >= t.ETHICS_COMPOSITE_MIN) && gdprPass && aiActPass && conflicts.length === 0;

  const recommendations = [];
  if (compositeEthics < t.ETHICS_COMPOSITE_MIN) recommendations.push({ type: 'ethics', message: 'Increase justification depth and cross-framework consensus.' });
  if (!gdprPass) recommendations.push({ type: 'privacy', message: 'Minimize PII, strengthen consent and purpose limitation.' });
  if (!aiActPass) recommendations.push({ type: 'governance', message: 'Add risk controls, transparency artifacts, and traceability.' });

  return {
    compliant,
    compositeEthicsScore: round2(compositeEthics * 100), // return as 0-100 for UI
    conflicts,
    resolution: input.resolution || null,
    violations,
    recommendations,
  };
}

export function evaluatePerformance(input, t) {
  const m = input.metrics || {};
  // support both aggregated percentiles or single latency
  const p95 = num(m?.latency_ms?.p95, num(m.latency, 0));
  const p50 = num(m?.latency_ms?.p50, p95);
  const p99 = num(m?.latency_ms?.p99, p95);
  const cpu = num(m.cpu, 0);
  const memory = num(m.memory_mb, 0);
  const energy = num(m.energy_mw, 0);

  const alerts = [];
  if (p95 > t.LATENCY_MAX) alerts.push({ type: 'latency', severity: 'high', message: `p95 ${p95}ms > ${t.LATENCY_MAX}ms` });
  if (cpu > t.CPU_MAX) alerts.push({ type: 'cpu', severity: 'medium', message: `CPU ${cpu}% > ${t.CPU_MAX}%` });
  if (memory > t.MEMORY_MAX_MB) alerts.push({ type: 'memory', severity: 'medium', message: `Memory ${memory}MB > ${t.MEMORY_MAX_MB}MB` });
  if (energy > t.ENERGY_MAX_MW) alerts.push({ type: 'energy', severity: 'medium', message: `Energy ${energy}mW > ${t.ENERGY_MAX_MW}mW` });

  // Regression if provided
  if (m.baseline_ms && p95 && ((p95 - m.baseline_ms) / m.baseline_ms) > 0.2) {
    alerts.push({ type: 'latency_regression', severity: 'high', message: `p95 regression > 20% (baseline ${m.baseline_ms}ms)` });
  }

  // Engine suggestion
  const patterns = input.patterns || {};
  let target_engine = null;
  if (patterns.eventDriven) target_engine = 'neuromorphic';
  if (patterns.npHard) target_engine = 'quantum';
  if (!target_engine && (alerts.find(a => a.type === 'cpu') || alerts.find(a => a.type === 'latency'))) {
    target_engine = 'hybrid';
  }

  const should_optimize = alerts.length > 0 || !!target_engine;
  const confidence = target_engine ? (patterns.confidenceScore || 0.75) : 0.5;

  const suggestions = [];
  if (target_engine === 'neuromorphic') suggestions.push({ action: 'route_event_spikes', reason: 'Event-driven or low-power path' });
  if (target_engine === 'quantum') suggestions.push({ action: 'hybrid_qaoa', reason: 'NP-hard optimization detected' });
  if (alerts.some(a => a.type === 'latency')) suggestions.push({ action: 'profile_hot_paths', reason: 'Reduce tail latency' });

  return {
    should_optimize,
    target_engine,
    confidence,
    alerts,
    suggestions,
    metrics: { p50, p95, p99, cpu, memory, energy },
  };
}

// ---------- helpers ----------
function num(v, d = 0) { const n = Number(v); return Number.isFinite(n) ? n : d; }
function int(v, d = 0) { return Math.trunc(num(v, d)); }
function bool(v, d = false) { return typeof v === 'boolean' ? v : d; }
function pct(v) { // normalize to 0..100
  if (v == null) return 0;
  const n = Number(v);
  if (!Number.isFinite(n)) return 0;
  return n > 1 ? Math.max(0, Math.min(100, n)) : Math.max(0, Math.min(100, n * 100));
}
function depth(v) { return Math.max(0, int(v, 0)); }
function md2pct(d) { // map depth to percentage roughly (4 -> 80, 6 -> 95)
  if (d <= 0) return 0;
  if (d >= 6) return 95;
  if (d >= 4) return 80 + (d - 4) * 7.5; // 4->80, 5->87.5, 6->95
  return d * 15; // 1->15, 2->30, 3->45
}
function round2(n) { return Math.round((n + Number.EPSILON) * 100) / 100; }
function normalizeFrameworks(fw) {
  return {
    utilitarian: norm01(fw.utilitarian?.score ?? fw.utilitarian ?? 0),
    deontological: norm01(fw.deontological?.score ?? fw.deontological ?? 0),
    virtue: norm01(fw.virtue?.score ?? fw.virtue ?? 0),
    care: norm01(fw.care?.score ?? fw.care ?? 0),
    justice: norm01(fw.justice?.score ?? fw.justice ?? 0),
  };
}
function norm01(v) {
  const n = Number(v);
  if (!Number.isFinite(n)) return 0;
  if (n <= 1) return Math.max(0, Math.min(1, n));
  return Math.max(0, Math.min(1, n / 100));
}
