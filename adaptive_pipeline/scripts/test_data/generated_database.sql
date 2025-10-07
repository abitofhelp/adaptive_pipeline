
-- Generated using our ID value objects - SOURCE OF TRUTH
-- Delete old database and create fresh one
DROP TABLE IF EXISTS stage_parameters;
DROP TABLE IF EXISTS pipeline_stages;
DROP TABLE IF EXISTS pipeline_configuration;
DROP TABLE IF EXISTS processing_metrics;
DROP TABLE IF EXISTS processing_sessions;
DROP TABLE IF EXISTS file_chunks;
DROP TABLE IF EXISTS security_contexts;
DROP TABLE IF EXISTS pipelines;

-- Create tables
CREATE TABLE pipelines (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    archived BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE pipeline_configuration (
    pipeline_id TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    PRIMARY KEY (pipeline_id, key),
    FOREIGN KEY (pipeline_id) REFERENCES pipelines(id)
);

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
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (pipeline_id) REFERENCES pipelines(id)
);

CREATE TABLE stage_parameters (
    stage_id TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    PRIMARY KEY (stage_id, key),
    FOREIGN KEY (stage_id) REFERENCES pipeline_stages(id)
);

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
    FOREIGN KEY (pipeline_id) REFERENCES pipelines(id)
);

-- Insert data with proper ULID format from our value objects
INSERT INTO pipelines (id, name, archived, created_at, updated_at) VALUES
('01JZSW3DPHGZTWM26G62NF4NAK', 'test-multi-stage', false, '2025-07-09T19:25:00Z', '2025-07-09T19:25:00Z'),
('01JZSW3DPHTRNFR1DX9SCJEBQ3', 'image-processing', false, '2025-07-09T19:25:00Z', '2025-07-09T19:25:00Z');

-- Insert pipeline configuration
INSERT INTO pipeline_configuration (pipeline_id, key, value) VALUES
('01JZSW3DPHGZTWM26G62NF4NAK', 'encryption_algorithm', 'aes256gcm'),
('01JZSW3DPHGZTWM26G62NF4NAK', 'compression_algorithm', 'brotli'),
('01JZSW3DPHGZTWM26G62NF4NAK', 'chunk_size_mb', '1');

-- Insert pipeline stages for test-multi-stage (4 stages total)
INSERT INTO pipeline_stages (id, pipeline_id, name, stage_type, enabled, stage_order, algorithm, parallel_processing, chunk_size, created_at, updated_at) VALUES
('01JZSW3DPHD3RG2XKRT3YYVRTY', '01JZSW3DPHGZTWM26G62NF4NAK', 'input_checksum', 'Custom', true, 0, 'sha256', false, null, '2025-07-09T19:25:00Z', '2025-07-09T19:25:00Z'),
('01JZSW3DPHBT1V9P79DWBRTXRC', '01JZSW3DPHGZTWM26G62NF4NAK', 'compression', 'Custom', true, 1, 'brotli', false, null, '2025-07-09T19:25:00Z', '2025-07-09T19:25:00Z'),
('01JZSW3DPH45CWPXHEWQTZ6NDW', '01JZSW3DPHGZTWM26G62NF4NAK', 'encryption', 'Custom', true, 2, 'aes256gcm', false, null, '2025-07-09T19:25:00Z', '2025-07-09T19:25:00Z'),
('01JZSW3DPHEV9RNYRA2PPEB976', '01JZSW3DPHGZTWM26G62NF4NAK', 'output_checksum', 'Custom', true, 3, 'sha256', false, null, '2025-07-09T19:25:00Z', '2025-07-09T19:25:00Z'),
('01JZSW3DPHVNZSS9SGQTVK9QHM', '01JZSW3DPHTRNFR1DX9SCJEBQ3', 'input_validation', 'Custom', true, 0, 'sha256', false, null, '2025-07-09T19:25:00Z', '2025-07-09T19:25:00Z'),
('01JZSW3DPHCB939C4AM370AKDE', '01JZSW3DPHTRNFR1DX9SCJEBQ3', 'image_compression', 'Custom', true, 1, 'jpeg', false, null, '2025-07-09T19:25:00Z', '2025-07-09T19:25:00Z');

-- Insert processing metrics
INSERT INTO processing_metrics (pipeline_id) VALUES
('01JZSW3DPHGZTWM26G62NF4NAK'),
('01JZSW3DPHTRNFR1DX9SCJEBQ3');

-- Verify data
SELECT 'pipelines' as table_name, COUNT(*) as count FROM pipelines
UNION ALL
SELECT 'pipeline_configuration', COUNT(*) FROM pipeline_configuration
UNION ALL
SELECT 'pipeline_stages', COUNT(*) FROM pipeline_stages
UNION ALL
SELECT 'processing_metrics', COUNT(*) FROM processing_metrics;
