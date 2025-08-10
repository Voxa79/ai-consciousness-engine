-- Initial schema for Consciousness Engine
-- This migration creates the foundational tables for the consciousness platform

-- Enable required extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";
CREATE EXTENSION IF NOT EXISTS "pg_trgm";

-- Users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    roles TEXT[] NOT NULL DEFAULT '{}',
    consciousness_tier VARCHAR(20) NOT NULL DEFAULT 'Basic',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_login TIMESTAMPTZ,
    is_active BOOLEAN NOT NULL DEFAULT true,
    
    CONSTRAINT valid_consciousness_tier CHECK (consciousness_tier IN ('Basic', 'Advanced', 'Enterprise', 'Research')),
    CONSTRAINT valid_email CHECK (email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$')
);

-- Create indexes for users
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_consciousness_tier ON users(consciousness_tier);
CREATE INDEX idx_users_created_at ON users(created_at);
CREATE INDEX idx_users_is_active ON users(is_active);

-- Agents table
CREATE TABLE agents (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(100) NOT NULL,
    description TEXT,
    agent_type VARCHAR(50) NOT NULL,
    owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    configuration JSONB NOT NULL,
    consciousness_state JSONB,
    status VARCHAR(20) NOT NULL DEFAULT 'inactive',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_activity TIMESTAMPTZ,
    
    CONSTRAINT valid_agent_type CHECK (agent_type IN ('consciousness', 'analytical', 'creative', 'ethical', 'specialized')),
    CONSTRAINT valid_status CHECK (status IN ('active', 'inactive', 'training', 'error', 'maintenance'))
);

-- Create indexes for agents
CREATE INDEX idx_agents_owner_id ON agents(owner_id);
CREATE INDEX idx_agents_type ON agents(agent_type);
CREATE INDEX idx_agents_status ON agents(status);
CREATE INDEX idx_agents_created_at ON agents(created_at);
CREATE INDEX idx_agents_last_activity ON agents(last_activity);

-- Consciousness sessions table
CREATE TABLE consciousness_sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    agent_id UUID REFERENCES agents(id) ON DELETE SET NULL,
    request_data JSONB NOT NULL,
    response_data JSONB NOT NULL,
    consciousness_state JSONB NOT NULL,
    processing_metrics JSONB NOT NULL,
    ethical_evaluation JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Add GIN indexes for JSONB columns for fast queries
    CONSTRAINT valid_request_data CHECK (jsonb_typeof(request_data) = 'object'),
    CONSTRAINT valid_response_data CHECK (jsonb_typeof(response_data) = 'object'),
    CONSTRAINT valid_consciousness_state CHECK (jsonb_typeof(consciousness_state) = 'object'),
    CONSTRAINT valid_processing_metrics CHECK (jsonb_typeof(processing_metrics) = 'object'),
    CONSTRAINT valid_ethical_evaluation CHECK (jsonb_typeof(ethical_evaluation) = 'object')
);

-- Create indexes for consciousness sessions
CREATE INDEX idx_consciousness_sessions_user_id ON consciousness_sessions(user_id);
CREATE INDEX idx_consciousness_sessions_agent_id ON consciousness_sessions(agent_id);
CREATE INDEX idx_consciousness_sessions_created_at ON consciousness_sessions(created_at);

-- GIN indexes for JSONB columns
CREATE INDEX idx_consciousness_sessions_request_data ON consciousness_sessions USING GIN (request_data);
CREATE INDEX idx_consciousness_sessions_response_data ON consciousness_sessions USING GIN (response_data);
CREATE INDEX idx_consciousness_sessions_consciousness_state ON consciousness_sessions USING GIN (consciousness_state);
CREATE INDEX idx_consciousness_sessions_processing_metrics ON consciousness_sessions USING GIN (processing_metrics);
CREATE INDEX idx_consciousness_sessions_ethical_evaluation ON consciousness_sessions USING GIN (ethical_evaluation);

-- Ethical violations table
CREATE TABLE ethical_violations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    session_id UUID NOT NULL REFERENCES consciousness_sessions(id) ON DELETE CASCADE,
    violation_type VARCHAR(50) NOT NULL,
    severity VARCHAR(20) NOT NULL,
    description TEXT NOT NULL,
    framework VARCHAR(50) NOT NULL,
    confidence REAL NOT NULL CHECK (confidence >= 0.0 AND confidence <= 1.0),
    resolved BOOLEAN NOT NULL DEFAULT false,
    resolution_notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    resolved_at TIMESTAMPTZ,
    
    CONSTRAINT valid_violation_type CHECK (violation_type IN ('harm', 'bias', 'privacy', 'deception', 'manipulation', 'discrimination', 'unfairness')),
    CONSTRAINT valid_severity CHECK (severity IN ('low', 'medium', 'high', 'critical')),
    CONSTRAINT valid_framework CHECK (framework IN ('utilitarian', 'deontological', 'virtue_ethics', 'care_ethics', 'justice'))
);

-- Create indexes for ethical violations
CREATE INDEX idx_ethical_violations_session_id ON ethical_violations(session_id);
CREATE INDEX idx_ethical_violations_type ON ethical_violations(violation_type);
CREATE INDEX idx_ethical_violations_severity ON ethical_violations(severity);
CREATE INDEX idx_ethical_violations_framework ON ethical_violations(framework);
CREATE INDEX idx_ethical_violations_resolved ON ethical_violations(resolved);
CREATE INDEX idx_ethical_violations_created_at ON ethical_violations(created_at);

-- Analytics metrics table (partitioned by date for performance)
CREATE TABLE analytics_metrics (
    id UUID DEFAULT uuid_generate_v4(),
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    agent_id UUID REFERENCES agents(id) ON DELETE SET NULL,
    request_id UUID NOT NULL,
    
    -- Performance metrics
    processing_time_ms BIGINT NOT NULL CHECK (processing_time_ms >= 0),
    queue_time_ms BIGINT NOT NULL CHECK (queue_time_ms >= 0),
    total_time_ms BIGINT NOT NULL CHECK (total_time_ms >= 0),
    
    -- Consciousness metrics
    awareness_level REAL NOT NULL CHECK (awareness_level >= 0.0 AND awareness_level <= 1.0),
    ethical_score REAL NOT NULL CHECK (ethical_score >= 0.0 AND ethical_score <= 1.0),
    creativity_score REAL NOT NULL CHECK (creativity_score >= 0.0 AND creativity_score <= 1.0),
    meta_cognitive_depth SMALLINT NOT NULL CHECK (meta_cognitive_depth >= 0),
    
    -- System metrics
    cpu_usage REAL NOT NULL CHECK (cpu_usage >= 0.0 AND cpu_usage <= 1.0),
    memory_usage REAL NOT NULL CHECK (memory_usage >= 0.0 AND memory_usage <= 1.0),
    gpu_usage REAL CHECK (gpu_usage >= 0.0 AND gpu_usage <= 1.0),
    quantum_coherence REAL CHECK (quantum_coherence >= 0.0 AND quantum_coherence <= 1.0),
    
    PRIMARY KEY (id, timestamp)
) PARTITION BY RANGE (timestamp);

-- Create monthly partitions for analytics metrics
CREATE TABLE analytics_metrics_2024_01 PARTITION OF analytics_metrics
    FOR VALUES FROM ('2024-01-01') TO ('2024-02-01');
CREATE TABLE analytics_metrics_2024_02 PARTITION OF analytics_metrics
    FOR VALUES FROM ('2024-02-01') TO ('2024-03-01');
CREATE TABLE analytics_metrics_2024_03 PARTITION OF analytics_metrics
    FOR VALUES FROM ('2024-03-01') TO ('2024-04-01');
CREATE TABLE analytics_metrics_2024_04 PARTITION OF analytics_metrics
    FOR VALUES FROM ('2024-04-01') TO ('2024-05-01');
CREATE TABLE analytics_metrics_2024_05 PARTITION OF analytics_metrics
    FOR VALUES FROM ('2024-05-01') TO ('2024-06-01');
CREATE TABLE analytics_metrics_2024_06 PARTITION OF analytics_metrics
    FOR VALUES FROM ('2024-06-01') TO ('2024-07-01');
CREATE TABLE analytics_metrics_2024_07 PARTITION OF analytics_metrics
    FOR VALUES FROM ('2024-07-01') TO ('2024-08-01');
CREATE TABLE analytics_metrics_2024_08 PARTITION OF analytics_metrics
    FOR VALUES FROM ('2024-08-01') TO ('2024-09-01');
CREATE TABLE analytics_metrics_2024_09 PARTITION OF analytics_metrics
    FOR VALUES FROM ('2024-09-01') TO ('2024-10-01');
CREATE TABLE analytics_metrics_2024_10 PARTITION OF analytics_metrics
    FOR VALUES FROM ('2024-10-01') TO ('2024-11-01');
CREATE TABLE analytics_metrics_2024_11 PARTITION OF analytics_metrics
    FOR VALUES FROM ('2024-11-01') TO ('2024-12-01');
CREATE TABLE analytics_metrics_2024_12 PARTITION OF analytics_metrics
    FOR VALUES FROM ('2024-12-01') TO ('2025-01-01');

-- Create indexes for analytics metrics
CREATE INDEX idx_analytics_metrics_user_id ON analytics_metrics(user_id);
CREATE INDEX idx_analytics_metrics_agent_id ON analytics_metrics(agent_id);
CREATE INDEX idx_analytics_metrics_request_id ON analytics_metrics(request_id);
CREATE INDEX idx_analytics_metrics_timestamp ON analytics_metrics(timestamp);
CREATE INDEX idx_analytics_metrics_awareness_level ON analytics_metrics(awareness_level);
CREATE INDEX idx_analytics_metrics_ethical_score ON analytics_metrics(ethical_score);

-- API keys table for authentication
CREATE TABLE api_keys (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    key_hash VARCHAR(255) NOT NULL UNIQUE,
    permissions TEXT[] NOT NULL DEFAULT '{}',
    rate_limit_per_hour INTEGER NOT NULL DEFAULT 1000,
    expires_at TIMESTAMPTZ,
    last_used_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    is_active BOOLEAN NOT NULL DEFAULT true
);

-- Create indexes for API keys
CREATE INDEX idx_api_keys_user_id ON api_keys(user_id);
CREATE INDEX idx_api_keys_key_hash ON api_keys(key_hash);
CREATE INDEX idx_api_keys_is_active ON api_keys(is_active);
CREATE INDEX idx_api_keys_expires_at ON api_keys(expires_at);

-- Rate limiting table
CREATE TABLE rate_limits (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    identifier VARCHAR(255) NOT NULL, -- user_id, api_key, or IP address
    identifier_type VARCHAR(20) NOT NULL CHECK (identifier_type IN ('user', 'api_key', 'ip')),
    endpoint VARCHAR(255) NOT NULL,
    requests_count INTEGER NOT NULL DEFAULT 0,
    window_start TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    window_duration_seconds INTEGER NOT NULL DEFAULT 3600,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(identifier, identifier_type, endpoint, window_start)
);

-- Create indexes for rate limiting
CREATE INDEX idx_rate_limits_identifier ON rate_limits(identifier, identifier_type);
CREATE INDEX idx_rate_limits_endpoint ON rate_limits(endpoint);
CREATE INDEX idx_rate_limits_window_start ON rate_limits(window_start);

-- Audit log table
CREATE TABLE audit_logs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    action VARCHAR(100) NOT NULL,
    resource_type VARCHAR(50) NOT NULL,
    resource_id UUID,
    details JSONB,
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for audit logs
CREATE INDEX idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX idx_audit_logs_action ON audit_logs(action);
CREATE INDEX idx_audit_logs_resource_type ON audit_logs(resource_type);
CREATE INDEX idx_audit_logs_resource_id ON audit_logs(resource_id);
CREATE INDEX idx_audit_logs_created_at ON audit_logs(created_at);
CREATE INDEX idx_audit_logs_details ON audit_logs USING GIN (details);

-- System configuration table
CREATE TABLE system_config (
    key VARCHAR(100) PRIMARY KEY,
    value JSONB NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Insert default system configuration
INSERT INTO system_config (key, value, description) VALUES
('consciousness.default_quality_threshold', '0.85', 'Default quality threshold for consciousness processing'),
('consciousness.default_ethical_strictness', '0.95', 'Default ethical strictness level'),
('consciousness.max_processing_time_ms', '30000', 'Maximum processing time in milliseconds'),
('rate_limiting.default_requests_per_hour', '1000', 'Default rate limit per hour for users'),
('rate_limiting.premium_requests_per_hour', '10000', 'Premium rate limit per hour for advanced users'),
('analytics.retention_days', '365', 'Number of days to retain analytics data'),
('security.session_timeout_minutes', '60', 'Session timeout in minutes'),
('security.max_failed_login_attempts', '5', 'Maximum failed login attempts before lockout');

-- Functions for updated_at timestamps
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Create triggers for updated_at columns
CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_agents_updated_at BEFORE UPDATE ON agents
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_rate_limits_updated_at BEFORE UPDATE ON rate_limits
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_system_config_updated_at BEFORE UPDATE ON system_config
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Function to create new monthly partition for analytics_metrics
CREATE OR REPLACE FUNCTION create_monthly_partition(table_name text, start_date date)
RETURNS void AS $$
DECLARE
    partition_name text;
    end_date date;
BEGIN
    partition_name := table_name || '_' || to_char(start_date, 'YYYY_MM');
    end_date := start_date + interval '1 month';
    
    EXECUTE format('CREATE TABLE IF NOT EXISTS %I PARTITION OF %I FOR VALUES FROM (%L) TO (%L)',
                   partition_name, table_name, start_date, end_date);
END;
$$ LANGUAGE plpgsql;

-- Function to clean up old analytics data
CREATE OR REPLACE FUNCTION cleanup_old_analytics_data(retention_days integer DEFAULT 365)
RETURNS integer AS $$
DECLARE
    cutoff_date timestamptz;
    deleted_count integer;
BEGIN
    cutoff_date := NOW() - (retention_days || ' days')::interval;
    
    DELETE FROM analytics_metrics WHERE timestamp < cutoff_date;
    GET DIAGNOSTICS deleted_count = ROW_COUNT;
    
    RETURN deleted_count;
END;
$$ LANGUAGE plpgsql;

-- Create a view for user statistics
CREATE VIEW user_stats AS
SELECT 
    u.id,
    u.username,
    u.consciousness_tier,
    COUNT(cs.id) as total_sessions,
    AVG((cs.consciousness_state->>'awareness_level')::float) as avg_awareness_level,
    AVG((cs.ethical_evaluation->>'overall_score')::float) as avg_ethical_score,
    MAX(cs.created_at) as last_session_at,
    COUNT(CASE WHEN cs.created_at > NOW() - interval '24 hours' THEN 1 END) as sessions_last_24h,
    COUNT(CASE WHEN cs.created_at > NOW() - interval '7 days' THEN 1 END) as sessions_last_7d
FROM users u
LEFT JOIN consciousness_sessions cs ON u.id = cs.user_id
WHERE u.is_active = true
GROUP BY u.id, u.username, u.consciousness_tier;

-- Create a view for agent performance
CREATE VIEW agent_performance AS
SELECT 
    a.id,
    a.name,
    a.agent_type,
    a.status,
    COUNT(cs.id) as total_sessions,
    AVG(EXTRACT(EPOCH FROM (cs.response_data->>'processing_time_ms')::bigint)) as avg_processing_time_ms,
    AVG((cs.consciousness_state->>'awareness_level')::float) as avg_awareness_level,
    AVG((cs.ethical_evaluation->>'overall_score')::float) as avg_ethical_score,
    COUNT(ev.id) as total_violations,
    MAX(cs.created_at) as last_used_at
FROM agents a
LEFT JOIN consciousness_sessions cs ON a.id = cs.agent_id
LEFT JOIN ethical_violations ev ON cs.id = ev.session_id
GROUP BY a.id, a.name, a.agent_type, a.status;

-- Grant permissions (these would be customized based on your user roles)
-- For now, we'll create basic roles

-- Create database roles
DO $$
BEGIN
    IF NOT EXISTS (SELECT FROM pg_catalog.pg_roles WHERE rolname = 'consciousness_app') THEN
        CREATE ROLE consciousness_app LOGIN;
    END IF;
    
    IF NOT EXISTS (SELECT FROM pg_catalog.pg_roles WHERE rolname = 'consciousness_readonly') THEN
        CREATE ROLE consciousness_readonly LOGIN;
    END IF;
END
$$;

-- Grant permissions to application role
GRANT USAGE ON SCHEMA public TO consciousness_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA public TO consciousness_app;
GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA public TO consciousness_app;
GRANT EXECUTE ON ALL FUNCTIONS IN SCHEMA public TO consciousness_app;

-- Grant read-only permissions
GRANT USAGE ON SCHEMA public TO consciousness_readonly;
GRANT SELECT ON ALL TABLES IN SCHEMA public TO consciousness_readonly;
GRANT SELECT ON ALL SEQUENCES IN SCHEMA public TO consciousness_readonly;

-- Set default privileges for future objects
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO consciousness_app;
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT USAGE, SELECT ON SEQUENCES TO consciousness_app;
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT EXECUTE ON FUNCTIONS TO consciousness_app;

ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT SELECT ON TABLES TO consciousness_readonly;
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT SELECT ON SEQUENCES TO consciousness_readonly;