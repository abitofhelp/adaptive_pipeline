-- Structured Database Schema - No JSON serialization
-- Uses proper database columns for all Pipeline fields

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

-- Create processing_metrics table
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
    FOREIGN KEY (pipeline_id) REFERENCES pipelines(id)
);

-- Create processing_sessions table  
CREATE TABLE IF NOT EXISTS processing_sessions (
    id TEXT PRIMARY KEY,
    pipeline_name TEXT NOT NULL,
    status INTEGER NOT NULL,
    compression_enabled BOOLEAN NOT NULL,
    encryption_enabled BOOLEAN NOT NULL,
    error_count INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL
);

-- Create file_chunks table
CREATE TABLE IF NOT EXISTS file_chunks (
    id TEXT PRIMARY KEY,
    session_id TEXT NOT NULL,
    chunk_index INTEGER NOT NULL,
    status INTEGER NOT NULL,
    compression_ratio REAL,
    input_size INTEGER NOT NULL,
    output_size INTEGER NOT NULL,
    FOREIGN KEY (session_id) REFERENCES processing_sessions(id)
);

-- Create security_contexts table
CREATE TABLE IF NOT EXISTS security_contexts (
    id TEXT PRIMARY KEY,
    context_type TEXT NOT NULL,
    encryption_key TEXT NOT NULL,
    created_at TEXT NOT NULL
);

-- Insert sample pipeline data using structured approach
INSERT INTO pipelines (id, name, archived, created_at, updated_at) VALUES
('01HN8X9K2M5P7Q8R9S0T1U2V4A', 'secure-backup', false, '2025-01-10T14:20:00Z', '2025-01-12T09:15:00Z'),
('01HN8X9K2M5P7Q8R9S0T1U2V3W', 'image-processing', false, '2025-01-15T10:30:00Z', '2025-01-15T10:30:00Z');

-- Insert pipeline configuration
INSERT INTO pipeline_configuration (pipeline_id, key, value) VALUES
('01HN8X9K2M5P7Q8R9S0T1U2V4A', 'encryption_algorithm', 'AES256'),
('01HN8X9K2M5P7Q8R9S0T1U2V4A', 'key_rotation_days', '30'),
('01HN8X9K2M5P7Q8R9S0T1U2V4A', 'backup_enabled', 'true'),
('01HN8X9K2M5P7Q8R9S0T1U2V3W', 'max_file_size', '100MB'),
('01HN8X9K2M5P7Q8R9S0T1U2V3W', 'output_format', 'JPEG'),
('01HN8X9K2M5P7Q8R9S0T1U2V3W', 'quality', '85');

-- Insert pipeline stages
INSERT INTO pipeline_stages (id, pipeline_id, name, stage_type, enabled, stage_order, algorithm, parallel_processing, chunk_size, created_at, updated_at) VALUES
('01HN8X9K2M5P7Q8R9S0T1U2V4B', '01HN8X9K2M5P7Q8R9S0T1U2V4A', 'input_checksum', 'Integrity', true, 1, 'SHA256', false, null, '2025-01-10T14:20:00Z', '2025-01-10T14:20:00Z'),
('01HN8X9K2M5P7Q8R9S0T1U2V4C', '01HN8X9K2M5P7Q8R9S0T1U2V4A', 'compression', 'Compression', true, 2, 'brotli', false, null, '2025-01-10T14:20:00Z', '2025-01-10T14:20:00Z'),
('01HN8X9K2M5P7Q8R9S0T1U2V4D', '01HN8X9K2M5P7Q8R9S0T1U2V4A', 'encryption', 'Encryption', true, 3, 'aes256gcm', false, null, '2025-01-10T14:20:00Z', '2025-01-10T14:20:00Z'),
('01HN8X9K2M5P7Q8R9S0T1U2V4E', '01HN8X9K2M5P7Q8R9S0T1U2V4A', 'output_checksum', 'Integrity', true, 4, 'SHA256', false, null, '2025-01-10T14:20:00Z', '2025-01-10T14:20:00Z'),
('01HN8X9K2M5P7Q8R9S0T1U2V3X', '01HN8X9K2M5P7Q8R9S0T1U2V3W', 'input_validation', 'Integrity', true, 1, 'SHA256', false, null, '2025-01-15T10:30:00Z', '2025-01-15T10:30:00Z'),
('01HN8X9K2M5P7Q8R9S0T1U2V3Y', '01HN8X9K2M5P7Q8R9S0T1U2V3W', 'image_compression', 'Compression', true, 2, 'jpeg', false, null, '2025-01-15T10:30:00Z', '2025-01-15T10:30:00Z');

-- Insert stage parameters
INSERT INTO stage_parameters (stage_id, key, value) VALUES
('01HN8X9K2M5P7Q8R9S0T1U2V4C', 'level', '6'),
('01HN8X9K2M5P7Q8R9S0T1U2V3Y', 'compression_level', '6');

-- Insert processing metrics (initialized to zero)
INSERT INTO processing_metrics (pipeline_id) VALUES
('01HN8X9K2M5P7Q8R9S0T1U2V4A'),
('01HN8X9K2M5P7Q8R9S0T1U2V3W');

-- Insert sample processing sessions
INSERT INTO processing_sessions (id, pipeline_name, status, compression_enabled, encryption_enabled, error_count, created_at) VALUES
('01HN8X9K2M5P7Q8R9S0T1U2V3W', 'image-processing', 1, 1, 0, 0, '2025-01-15T11:00:00Z'),
('01HN8X9K2M5P7Q8R9S0T1U2V4A', 'secure-backup', 1, 0, 1, 0, '2025-01-15T12:30:00Z');

-- Insert sample file chunks
INSERT INTO file_chunks (id, session_id, chunk_index, status, compression_ratio, input_size, output_size) VALUES
('01HN8XA1B2C3D4E5F6G7H8I9J0', '01HN8X9K2M5P7Q8R9S0T1U2V3W', 2, 2, 100.0, 15728640, 15728640),
('01HN8XB2C3D4E5F6G7H8I9J0K1', '01HN8X9K2M5P7Q8R9S0T1U2V4A', 1, 1, 75.5, 1048576, 1048576);

-- Insert sample security contexts
INSERT INTO security_contexts (id, context_type, encryption_key, created_at) VALUES
('01HN8XC3D4E5F6G7H8I9J0K1L2', 'AES256', 'sample_key_data_here', '2025-01-15T09:00:00Z'),
('01HN8XD4E5F6G7H8I9J0K1L2M3', 'ChaCha20', 'another_sample_key', '2025-01-15T10:15:00Z');

-- Display table counts for verification
SELECT 'pipelines' as table_name, COUNT(*) as count FROM pipelines
UNION ALL
SELECT 'pipeline_configuration', COUNT(*) FROM pipeline_configuration
UNION ALL
SELECT 'pipeline_stages', COUNT(*) FROM pipeline_stages  
UNION ALL
SELECT 'stage_parameters', COUNT(*) FROM stage_parameters
UNION ALL
SELECT 'processing_metrics', COUNT(*) FROM processing_metrics
UNION ALL
SELECT 'processing_sessions', COUNT(*) FROM processing_sessions
UNION ALL
SELECT 'file_chunks', COUNT(*) FROM file_chunks
UNION ALL
SELECT 'security_contexts', COUNT(*) FROM security_contexts;

-- Display pipeline names for verification
SELECT id, name, created_at, updated_at FROM pipelines ORDER BY name;
