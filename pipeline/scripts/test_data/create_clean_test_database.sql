-- Clean Test Database Creation Script
-- All IDs use simple string ULID format for consistent serialization

-- Create pipelines table
CREATE TABLE IF NOT EXISTS pipelines (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    data TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    archived BOOLEAN NOT NULL DEFAULT FALSE
);

-- Create processing_sessions table  
CREATE TABLE IF NOT EXISTS processing_sessions (
    id TEXT PRIMARY KEY,
    pipeline_name TEXT NOT NULL,
    status INTEGER NOT NULL,
    compression_enabled BOOLEAN NOT NULL,
    encryption_enabled BOOLEAN NOT NULL,
    error_count INTEGER NOT NULL,
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
    output_size INTEGER NOT NULL
);

-- Create security_contexts table
CREATE TABLE IF NOT EXISTS security_contexts (
    id TEXT PRIMARY KEY,
    context_type TEXT NOT NULL,
    encryption_key TEXT NOT NULL,
    created_at TEXT NOT NULL
);

-- Insert clean pipeline data with simple string IDs
INSERT INTO pipelines (id, name, data, created_at, updated_at, archived) VALUES
(
    '01HN8X9K2M5P7Q8R9S0T1U2V4A',
    'secure-backup',
    '{"id":"01HN8X9K2M5P7Q8R9S0T1U2V4A","name":"secure-backup","archived":false,"configuration":{"encryption_algorithm":"AES256","key_rotation_days":"30","backup_enabled":"true"},"metrics":{"bytes_processed":0,"bytes_total":0,"chunks_processed":0,"chunks_total":0,"start_time_rfc3339":null,"end_time_rfc3339":null,"processing_duration":null,"throughput_bytes_per_second":0.0,"compression_ratio":null,"error_count":0,"warning_count":0,"input_file_size_bytes":0,"output_file_size_bytes":0,"input_file_checksum":null,"output_file_checksum":null,"stage_metrics":{}},"stages":[{"id":"01HN8X9K2M5P7Q8R9S0T1U2V4B","name":"input_checksum","stage_type":"Integrity","configuration":{"algorithm":"SHA256","parameters":{},"parallel_processing":false,"chunk_size":null},"enabled":true,"order":1,"created_at":"2025-01-10T14:20:00Z","updated_at":"2025-01-10T14:20:00Z"},{"id":"01HN8X9K2M5P7Q8R9S0T1U2V4C","name":"compression","stage_type":"Compression","configuration":{"algorithm":"brotli","parameters":{"level":"6"},"parallel_processing":false,"chunk_size":null},"enabled":true,"order":2,"created_at":"2025-01-10T14:20:00Z","updated_at":"2025-01-10T14:20:00Z"},{"id":"01HN8X9K2M5P7Q8R9S0T1U2V4D","name":"encryption","stage_type":"Encryption","configuration":{"algorithm":"aes256gcm","parameters":{},"parallel_processing":false,"chunk_size":null},"enabled":true,"order":3,"created_at":"2025-01-10T14:20:00Z","updated_at":"2025-01-10T14:20:00Z"},{"id":"01HN8X9K2M5P7Q8R9S0T1U2V4E","name":"output_checksum","stage_type":"Integrity","configuration":{"algorithm":"SHA256","parameters":{},"parallel_processing":false,"chunk_size":null},"enabled":true,"order":4,"created_at":"2025-01-10T14:20:00Z","updated_at":"2025-01-10T14:20:00Z"}],"created_at":"2025-01-10T14:20:00Z","updated_at":"2025-01-12T09:15:00Z"}',
    '2025-01-10T14:20:00Z',
    '2025-01-12T09:15:00Z',
    false
),
(
    '01HN8X9K2M5P7Q8R9S0T1U2V3W',
    'image-processing',
    '{"id":"01HN8X9K2M5P7Q8R9S0T1U2V3W","name":"image-processing","archived":false,"configuration":{"max_file_size":"100MB","output_format":"JPEG","quality":"85"},"metrics":{"bytes_processed":0,"bytes_total":0,"chunks_processed":0,"chunks_total":0,"start_time_rfc3339":null,"end_time_rfc3339":null,"processing_duration":null,"throughput_bytes_per_second":0.0,"compression_ratio":null,"error_count":0,"warning_count":0,"input_file_size_bytes":0,"output_file_size_bytes":0,"input_file_checksum":null,"output_file_checksum":null,"stage_metrics":{}},"stages":[{"id":"01HN8X9K2M5P7Q8R9S0T1U2V3X","name":"input_validation","stage_type":"Integrity","configuration":{"algorithm":"SHA256","parameters":{},"parallel_processing":false,"chunk_size":null},"enabled":true,"order":1,"created_at":"2025-01-15T10:30:00Z","updated_at":"2025-01-15T10:30:00Z"},{"id":"01HN8X9K2M5P7Q8R9S0T1U2V3Y","name":"image_compression","stage_type":"Compression","configuration":{"algorithm":"jpeg","parameters":{"compression_level":"6"},"parallel_processing":false,"chunk_size":null},"enabled":true,"order":2,"created_at":"2025-01-15T10:30:00Z","updated_at":"2025-01-15T10:30:00Z"}],"created_at":"2025-01-15T10:30:00Z","updated_at":"2025-01-15T10:30:00Z"}',
    '2025-01-15T10:30:00Z',
    '2025-01-15T10:30:00Z',
    false
);

-- Insert sample processing sessions
INSERT INTO processing_sessions (id, pipeline_name, status, compression_enabled, encryption_enabled, error_count, created_at) VALUES
('01HN8X9K2M5P7Q8R9S0T1U2V3W', 'image-processing', 1, 1, 0, 0, '2025-01-15T11:00:00Z'),
('01HN8X9K2M5P7Q8R9S0T1U2V4A', 'secure-backup', 1, 0, 1, 0, '2025-01-15T12:30:00Z');

-- Insert sample file chunks
INSERT INTO file_chunks (id, session_id, chunk_index, status, compression_ratio, input_size, output_size) VALUES
('01HN8XA1B2C3D4E5F6G7H8I9J0', '01HN8X9K2M5P7Q8R9S0T1U2V3W', 2, 2, 100.0, 15728640, 15728640),
('01HN8XA1B2C3D4E5F6G7H8I9K1', '01HN8X9K2M5P7Q8R9S0T1U2V4A', 1, 0, 0.0, 2097152, 0);

-- Insert sample security contexts
INSERT INTO security_contexts (id, context_type, encryption_key, created_at) VALUES
('01HN8XB1C2D3E4F5G6H7I8J9K0', 'Internal', 'test_key_internal_001', '2025-01-15T10:00:00Z'),
('01HN8XB1C2D3E4F5G6H7I8J9L1', 'External', 'test_key_external_002', '2025-01-15T11:00:00Z');

-- Display table counts for verification
SELECT 'pipelines' as table_name, COUNT(*) as count FROM pipelines
UNION ALL
SELECT 'processing_sessions', COUNT(*) FROM processing_sessions  
UNION ALL
SELECT 'file_chunks', COUNT(*) FROM file_chunks
UNION ALL
SELECT 'security_contexts', COUNT(*) FROM security_contexts;

-- Display pipeline names for verification
SELECT id, name, created_at, updated_at FROM pipelines ORDER BY name;
