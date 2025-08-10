# Integration Roadmap — Hooks → Runtime

This roadmap turns the hooks and steering standards into a minimal runnable MVP (without heavy infra changes).

## 1) Endpoints (MVP)
- POST /quality/validate
  - Input: decision/change context, scores
  - Output: { passed, composite_score, failed_validations[], recommendations[] }
- POST /compliance/check
  - Input: decision + context (user, data categories, risk, transparency), ethical framework scores
  - Output: { compliance_passed, composite_compliance_score, critical_violations, violations[], requires_human_review }
- POST /performance/trigger
  - Input: metrics (latency, cpu, mem, energy), workload patterns, context
  - Output: { should_optimize, optimization_type, target_engine, priority, reason }

Notes:
- All endpoints sync, stateless; enforcement (block/pass) is decided by caller using response.
- Add `mode: "audit" | "enforcement"` in server config to switch behaviors.

## 2) Persistence (Supabase)
- Create the following tables (SQL taken from hooks):
  - ethical_compliance_audits, ethical_reasoning_chains, compliance_metrics, compliance_violations
  - consciousness_validations, consciousness_quality_metrics
  - performance_optimizations, performance_metrics_history
- Minimal writes:
  - /quality/validate → insert into consciousness_validations and consciousness_quality_metrics (aggregate)
  - /compliance/check → insert into ethical_compliance_audits (+ violations when any) and compliance_metrics
  - /performance/trigger → when should_optimize=true, insert into performance_optimizations (result placeholder)

## 3) Configuration
- .env
  - SUPABASE_URL, SUPABASE_ANON_KEY
  - MODE=audit|enforcement
  - THRESHOLDS: QUALITY_MIN=85, ETHICAL_MIN=95, LATENCY_MAX=50, CPU_MAX=70
  - ALERTS: WEBHOOK_URL(optional), EMAIL_TO(optional)
- Feature flags
  - ENABLE_REMEDIATION=true|false, ENABLE_DASHBOARD=true|false

## 4) Security/Privacy
- PII minimization: hash identifiers, avoid payload raw data in audits
- Role separation: service key for writes, anon key for reads (dashboards)
- Retention policies: TTL or scheduled purge for raw logs

## 5) Dashboards (minimal)
- GET /quality/overview → aggregates from consciousness_quality_metrics
- GET /compliance/overview + GET /compliance/alerts → aggregates + unresolved high/critical
- GET /performance/overview → aggregates from performance_optimizations

## 6) Testing & Ops
- Postman collection with 6 requests (3 POST, 3 GET)
- CI: quality gate job (provided in hook) blocking on QUALITY_MIN and compliance_passed
- Health: GET /health returns version + supabase connectivity status

## 7) Work Plan (checklist)
- [ ] Create HTTP server skeleton (Node/Express or Edge/Fastify)
- [ ] Implement 3 POST endpoints with validation
- [ ] Implement 3 GET overview endpoints
- [ ] Supabase client + env config + safe insert helpers
- [ ] Create DB tables via SQL
- [ ] Add minimal logging with PII stripping
- [ ] Add health endpoint and basic rate limit
- [ ] Provide sample payloads and Postman collection

## 8) Timeline
- Day 1: endpoints + Supabase writes + health + sample payloads
- Day 2: overview endpoints + alerts + CI wiring (quality gate) + docs

## 9) Open Questions
- Where to deploy MVP (local only vs edge function vs small VM)?
- Do we enforce blocking at API layer or only signal to callers?
- Slack/Email integration for alerts?
