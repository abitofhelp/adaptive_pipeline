-- Complete Structured Database Schema - Matches Repository Code Expectations
-- Uses proper database columns for all Pipeline fields with full metrics support

-- Create pipelines table with structured columns
CREATE TABLE IF NOT EXISTS pipelines (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Create pipeline_configuration table
CREATE TABLE IF NOT EXISTS pipeline_configuration (
    pipeline_id TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    PRIMARY KEY (pipeline_id, key),
    FOREIGN KEY (pipeline_id) REFERENCES pipelines(id)
);

-- Create pipeline_stages table
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
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (pipeline_id) REFERENCES pipelines(id)
);

-- Create stage_parameters table
CREATE TABLE IF NOT EXISTS stage_parameters (
    stage_id TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    PRIMARY KEY (stage_id, key),
    FOREIGN KEY (stage_id) REFERENCES pipeline_stages(id)
);

-- Create comprehensive processing_metrics table matching repository expectations
CREATE TABLE IF NOT EXISTS processing_metrics (
    pipeline_id TEXT PRIMARY KEY,
    bytes_processed INTEGER NOT NULL DEFAULT 0,
    bytes_total INTEGER NOT NULL DEFAULT 0,
    chunks_processed INTEGER NOT NULL DEFAULT 0,
    chunks_total INTEGER NOT NULL DEFAULT 0,
    processing_duration_ms INTEGER,
    compression_ratio REAL NOT NULL DEFAULT 0.0,
    encryption_overhead REAL,
    throughput_mbps REAL,
    error_count INTEGER NOT NULL DEFAULT 0,
    warning_count INTEGER NOT NULL DEFAULT 0,
    stage_metrics_json TEXT NOT NULL DEFAULT '{}',
    input_file_path TEXT,
    output_file_path TEXT,
    input_file_checksum TEXT,
    output_file_checksum TEXT,
    FOREIGN KEY (pipeline_id) REFERENCES pipelines(id)
);

-- Create processing_sessions table (for tracking pipeline executions)
CREATE TABLE IF NOT EXISTS processing_sessions (
    id TEXT PRIMARY KEY,
    pipeline_id TEXT NOT NULL,
    status INTEGER NOT NULL,
    compression_enabled BOOLEAN NOT NULL,
    encryption_enabled BOOLEAN NOT NULL,
    error_count INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    FOREIGN KEY (pipeline_id) REFERENCES pipelines(id)
);

-- Create file_chunks table (for tracking processed file chunks)
CREATE TABLE IF NOT EXISTS file_chunks (
    id TEXT PRIMARY KEY,
    session_id TEXT NOT NULL,
    chunk_index INTEGER NOT NULL,
    status INTEGER NOT NULL,
    checksum TEXT,
    size_bytes INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    FOREIGN KEY (session_id) REFERENCES processing_sessions(id)
);

-- Create security_contexts table (for encryption/security metadata)
CREATE TABLE IF NOT EXISTS security_contexts (
    id TEXT PRIMARY KEY,
    pipeline_id TEXT NOT NULL,
    encryption_algorithm TEXT,
    key_derivation_algorithm TEXT,
    security_level INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    FOREIGN KEY (pipeline_id) REFERENCES pipelines(id)
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_pipelines_name ON pipelines(name);
CREATE INDEX IF NOT EXISTS idx_pipelines_archived ON pipelines(archived);
CREATE INDEX IF NOT EXISTS idx_pipeline_stages_pipeline_id ON pipeline_stages(pipeline_id);
CREATE INDEX IF NOT EXISTS idx_pipeline_stages_order ON pipeline_stages(stage_order);
CREATE INDEX IF NOT EXISTS idx_processing_sessions_pipeline_id ON processing_sessions(pipeline_id);
CREATE INDEX IF NOT EXISTS idx_file_chunks_session_id ON file_chunks(session_id);
