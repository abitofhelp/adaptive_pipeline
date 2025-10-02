-- Clean Database Schema with Soft Delete Support
-- All tables have archived column for proper soft delete functionality

-- Drop existing tables to ensure clean state
DROP TABLE IF EXISTS stage_parameters;
DROP TABLE IF EXISTS pipeline_stages;
DROP TABLE IF EXISTS pipeline_configuration;
DROP TABLE IF EXISTS processing_metrics;
DROP TABLE IF EXISTS processing_sessions;
DROP TABLE IF EXISTS file_chunks;
DROP TABLE IF EXISTS pipelines;

-- Create pipelines table with archived column
CREATE TABLE pipelines (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Create pipeline_configuration table with archived column
CREATE TABLE pipeline_configuration (
    pipeline_id TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (pipeline_id, key),
    FOREIGN KEY (pipeline_id) REFERENCES pipelines(id)
);

-- Create pipeline_stages table with archived column
CREATE TABLE pipeline_stages (
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
    FOREIGN KEY (pipeline_id) REFERENCES pipelines(id)
);

-- Create stage_parameters table with archived column
CREATE TABLE stage_parameters (
    stage_id TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (stage_id, key),
    FOREIGN KEY (stage_id) REFERENCES pipeline_stages(id)
);

-- Create processing_metrics table with archived column
CREATE TABLE processing_metrics (
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
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (pipeline_id) REFERENCES pipelines(id)
);

-- Create processing_sessions table with archived column
CREATE TABLE processing_sessions (
    id TEXT PRIMARY KEY,
    pipeline_name TEXT NOT NULL,
    status INTEGER NOT NULL,
    compression_enabled BOOLEAN NOT NULL,
    encryption_enabled BOOLEAN NOT NULL,
    error_count INTEGER NOT NULL DEFAULT 0,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TEXT NOT NULL
);

-- Create file_chunks table with archived column
CREATE TABLE file_chunks (
    id TEXT PRIMARY KEY,
    session_id TEXT NOT NULL,
    chunk_index INTEGER NOT NULL,
    data BLOB NOT NULL,
    checksum TEXT NOT NULL,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TEXT NOT NULL,
    FOREIGN KEY (session_id) REFERENCES processing_sessions(id)
);

-- Create indexes for performance
CREATE INDEX idx_pipelines_name ON pipelines(name);
CREATE INDEX idx_pipelines_archived ON pipelines(archived);
CREATE INDEX idx_pipeline_configuration_archived ON pipeline_configuration(archived);
CREATE INDEX idx_pipeline_stages_archived ON pipeline_stages(archived);
CREATE INDEX idx_pipeline_stages_pipeline_id ON pipeline_stages(pipeline_id);
CREATE INDEX idx_stage_parameters_archived ON stage_parameters(archived);
CREATE INDEX idx_processing_metrics_archived ON processing_metrics(archived);
CREATE INDEX idx_processing_sessions_archived ON processing_sessions(archived);
CREATE INDEX idx_file_chunks_archived ON file_chunks(archived);

-- Insert test data
INSERT INTO pipelines (id, name, archived, created_at, updated_at) VALUES
('01JZP5DJ4JQ2XHP339V32CNJEB', 'test-compression', FALSE, '2025-01-09T08:00:00Z', '2025-01-09T08:00:00Z'),
('01JZP5FGTCWX7NMY9E5KJKA7KW', 'test-encryption', FALSE, '2025-01-09T08:01:00Z', '2025-01-09T08:01:00Z'),
('01JZP5GVFFK1K7KPCFP3EHDZXE', 'test-multi-stage', FALSE, '2025-01-09T08:02:00Z', '2025-01-09T08:02:00Z');

-- Insert pipeline stages for test-compression
INSERT INTO pipeline_stages (id, pipeline_id, name, stage_type, enabled, stage_order, algorithm, parallel_processing, chunk_size, archived, created_at, updated_at) VALUES
('01JZP5DJ4JQ2XHP339V32CNJEC', '01JZP5DJ4JQ2XHP339V32CNJEB', 'input_checksum', 'integrity', TRUE, 1, 'sha256', FALSE, 1048576, FALSE, '2025-01-09T08:00:00Z', '2025-01-09T08:00:00Z'),
('01JZP5DJ4JQ2XHP339V32CNJED', '01JZP5DJ4JQ2XHP339V32CNJEB', 'compression', 'compression', TRUE, 2, 'brotli', FALSE, 1048576, FALSE, '2025-01-09T08:00:00Z', '2025-01-09T08:00:00Z'),
('01JZP5DJ4JQ2XHP339V32CNJEE', '01JZP5DJ4JQ2XHP339V32CNJEB', 'output_checksum', 'integrity', TRUE, 3, 'sha256', FALSE, 1048576, FALSE, '2025-01-09T08:00:00Z', '2025-01-09T08:00:00Z');

-- Insert pipeline stages for test-encryption
INSERT INTO pipeline_stages (id, pipeline_id, name, stage_type, enabled, stage_order, algorithm, parallel_processing, chunk_size, archived, created_at, updated_at) VALUES
('01JZP5FGTCWX7NMY9E5KJKA7KX', '01JZP5FGTCWX7NMY9E5KJKA7KW', 'input_checksum', 'integrity', TRUE, 1, 'sha256', FALSE, 1048576, FALSE, '2025-01-09T08:01:00Z', '2025-01-09T08:01:00Z'),
('01JZP5FGTCWX7NMY9E5KJKA7KY', '01JZP5FGTCWX7NMY9E5KJKA7KW', 'encryption', 'encryption', TRUE, 2, 'aes256gcm', FALSE, 1048576, FALSE, '2025-01-09T08:01:00Z', '2025-01-09T08:01:00Z'),
('01JZP5FGTCWX7NMY9E5KJKA7KZ', '01JZP5FGTCWX7NMY9E5KJKA7KW', 'output_checksum', 'integrity', TRUE, 3, 'sha256', FALSE, 1048576, FALSE, '2025-01-09T08:01:00Z', '2025-01-09T08:01:00Z');

-- Insert pipeline stages for test-multi-stage
INSERT INTO pipeline_stages (id, pipeline_id, name, stage_type, enabled, stage_order, algorithm, parallel_processing, chunk_size, archived, created_at, updated_at) VALUES
('01JZP5GVFFK1K7KPCFP3EHDZXF', '01JZP5GVFFK1K7KPCFP3EHDZXE', 'input_checksum', 'integrity', TRUE, 1, 'sha256', FALSE, 1048576, FALSE, '2025-01-09T08:02:00Z', '2025-01-09T08:02:00Z'),
('01JZP5GVFFK1K7KPCFP3EHDZXG', '01JZP5GVFFK1K7KPCFP3EHDZXE', 'compression', 'compression', TRUE, 2, 'brotli', FALSE, 1048576, FALSE, '2025-01-09T08:02:00Z', '2025-01-09T08:02:00Z'),
('01JZP5GVFFK1K7KPCFP3EHDZXH', '01JZP5GVFFK1K7KPCFP3EHDZXE', 'encryption', 'encryption', TRUE, 3, 'aes256gcm', FALSE, 1048576, FALSE, '2025-01-09T08:02:00Z', '2025-01-09T08:02:00Z'),
('01JZP5GVFFK1K7KPCFP3EHDZXI', '01JZP5GVFFK1K7KPCFP3EHDZXE', 'output_checksum', 'integrity', TRUE, 4, 'sha256', FALSE, 1048576, FALSE, '2025-01-09T08:02:00Z', '2025-01-09T08:02:00Z');
