-- Pipeline Database Schema
-- This schema supports the structured SQLite repository implementation

-- Main pipelines table
CREATE TABLE IF NOT EXISTS pipelines (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    archived BOOLEAN NOT NULL DEFAULT FALSE
);

-- Processing sessions table
CREATE TABLE IF NOT EXISTS processing_sessions (
    id TEXT PRIMARY KEY,
    pipeline_name TEXT NOT NULL,
    status INTEGER NOT NULL,
    compression_enabled BOOLEAN NOT NULL,
    encryption_enabled BOOLEAN NOT NULL,
    error_count INTEGER NOT NULL,
    created_at TEXT NOT NULL
);

-- File chunks table
CREATE TABLE IF NOT EXISTS file_chunks (
    id TEXT PRIMARY KEY,
    session_id TEXT NOT NULL,
    chunk_index INTEGER NOT NULL,
    status INTEGER NOT NULL,
    compression_ratio REAL,
    input_size INTEGER NOT NULL,
    output_size INTEGER NOT NULL
);

-- Security contexts table
CREATE TABLE IF NOT EXISTS security_contexts (
    id TEXT PRIMARY KEY,
    context_type TEXT NOT NULL,
    encryption_key TEXT NOT NULL,
    created_at TEXT NOT NULL
);

-- Additional tables that may be needed by the repository
-- (These may be created by the repository if they don't exist)

-- Pipeline configuration table (for key-value pairs)
CREATE TABLE IF NOT EXISTS pipeline_configuration (
    pipeline_id TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    PRIMARY KEY (pipeline_id, key),
    FOREIGN KEY (pipeline_id) REFERENCES pipelines(id) ON DELETE CASCADE
);

-- Pipeline stages table (for stage details)
CREATE TABLE IF NOT EXISTS pipeline_stages (
    id TEXT PRIMARY KEY,
    pipeline_id TEXT NOT NULL,
    name TEXT NOT NULL,
    stage_type TEXT NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    stage_order INTEGER NOT NULL,
    algorithm TEXT NOT NULL,
    parallel_processing BOOLEAN NOT NULL DEFAULT FALSE,
    chunk_size INTEGER,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (pipeline_id) REFERENCES pipelines(id) ON DELETE CASCADE
);

-- Stage configuration parameters table
CREATE TABLE IF NOT EXISTS stage_parameters (
    stage_id TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    PRIMARY KEY (stage_id, key),
    FOREIGN KEY (stage_id) REFERENCES pipeline_stages(id) ON DELETE CASCADE
);

-- NOTE: Metrics are handled by Prometheus (per SRS requirements)
-- No processing_metrics table needed - observability is external

-- Indexes for better performance
CREATE INDEX IF NOT EXISTS idx_pipelines_name ON pipelines(name);
CREATE INDEX IF NOT EXISTS idx_pipelines_archived ON pipelines(archived);
CREATE INDEX IF NOT EXISTS idx_pipeline_configuration_archived ON pipeline_configuration(archived);
CREATE INDEX IF NOT EXISTS idx_pipeline_stages_archived ON pipeline_stages(archived);
CREATE INDEX IF NOT EXISTS idx_stage_parameters_archived ON stage_parameters(archived);
CREATE INDEX IF NOT EXISTS idx_pipeline_stages_pipeline_id ON pipeline_stages(pipeline_id);
CREATE INDEX IF NOT EXISTS idx_pipeline_stages_order ON pipeline_stages(pipeline_id, stage_order);
CREATE INDEX IF NOT EXISTS idx_processing_sessions_pipeline ON processing_sessions(pipeline_name);
CREATE INDEX IF NOT EXISTS idx_file_chunks_session ON file_chunks(session_id);
