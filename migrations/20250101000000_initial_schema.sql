-- Initial database schema for Optimized Adaptive Pipeline
-- This migration creates all necessary tables for pipeline management

-- Pipelines table: stores pipeline configurations
CREATE TABLE IF NOT EXISTS pipelines (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    archived BOOLEAN NOT NULL DEFAULT false,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Pipeline configuration: key-value pairs for pipeline settings
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

-- Pipeline stages: defines processing stages in a pipeline
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

-- Stage parameters: configuration for individual stages
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

-- Processing metrics: tracks pipeline execution metrics
CREATE TABLE IF NOT EXISTS processing_metrics (
    pipeline_id TEXT PRIMARY KEY,
    bytes_processed INTEGER NOT NULL DEFAULT 0,
    bytes_total INTEGER NOT NULL DEFAULT 0,
    chunks_processed INTEGER NOT NULL DEFAULT 0,
    chunks_total INTEGER NOT NULL DEFAULT 0,
    start_time_rfc3339 TEXT,
    end_time_rfc3339 TEXT,
    processing_duration_ms INTEGER,
    throughput_bytes_per_second REAL NOT NULL DEFAULT 0.0,
    compression_ratio REAL,
    error_count INTEGER NOT NULL DEFAULT 0,
    warning_count INTEGER NOT NULL DEFAULT 0,
    input_file_size_bytes INTEGER NOT NULL DEFAULT 0,
    output_file_size_bytes INTEGER NOT NULL DEFAULT 0,
    input_file_checksum TEXT,
    output_file_checksum TEXT,
    FOREIGN KEY (pipeline_id) REFERENCES pipelines(id) ON DELETE CASCADE
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_pipeline_stages_pipeline_id ON pipeline_stages(pipeline_id);
CREATE INDEX IF NOT EXISTS idx_pipeline_stages_order ON pipeline_stages(pipeline_id, stage_order);
CREATE INDEX IF NOT EXISTS idx_pipeline_configuration_pipeline_id ON pipeline_configuration(pipeline_id);
CREATE INDEX IF NOT EXISTS idx_stage_parameters_stage_id ON stage_parameters(stage_id);
CREATE INDEX IF NOT EXISTS idx_pipelines_name ON pipelines(name) WHERE archived = false;
