-- ============================================================================
-- SQLite Test Database Creation Script
-- ============================================================================
-- 
-- Purpose: Creates a comprehensive test database for the optimized adaptive 
--          pipeline system with realistic sample data
-- 
-- Usage: sqlite3 test_pipeline.db < create_test_database.sql
-- 
-- Author: Generated for Phase 3 SQLite Integration
-- Date: 2025-07-07
-- ============================================================================

-- Enable foreign key constraints
PRAGMA foreign_keys = ON;

-- ============================================================================
-- TABLE CREATION
-- ============================================================================

-- Pipelines table (main entity)
CREATE TABLE IF NOT EXISTS pipelines (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    data TEXT NOT NULL,  -- JSON serialized Pipeline entity
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    archived BOOLEAN NOT NULL DEFAULT false
);

-- Pipeline stages table (for relational queries)
CREATE TABLE IF NOT EXISTS pipeline_stages (
    id TEXT PRIMARY KEY,
    pipeline_id TEXT NOT NULL,
    name TEXT NOT NULL,
    stage_order INTEGER NOT NULL,
    algorithm TEXT NOT NULL,
    data TEXT NOT NULL,  -- JSON serialized PipelineStage entity
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    archived BOOLEAN NOT NULL DEFAULT false,
    FOREIGN KEY (pipeline_id) REFERENCES pipelines(id) ON DELETE CASCADE
);

-- Processing sessions table
CREATE TABLE IF NOT EXISTS processing_sessions (
    id TEXT PRIMARY KEY,
    pipeline_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    status TEXT NOT NULL,
    data TEXT NOT NULL,  -- JSON serialized ProcessingSession entity
    started_at TEXT NOT NULL,
    completed_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    archived BOOLEAN NOT NULL DEFAULT false,
    FOREIGN KEY (pipeline_id) REFERENCES pipelines(id) ON DELETE CASCADE
);

-- File chunks table (for file processing tracking)
CREATE TABLE IF NOT EXISTS file_chunks (
    id TEXT PRIMARY KEY,
    session_id TEXT NOT NULL,
    chunk_number INTEGER NOT NULL,
    file_path TEXT NOT NULL,
    size_bytes INTEGER NOT NULL,
    checksum TEXT,
    data TEXT NOT NULL,  -- JSON serialized FileChunk entity
    processed_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    archived BOOLEAN NOT NULL DEFAULT false,
    FOREIGN KEY (session_id) REFERENCES processing_sessions(id) ON DELETE CASCADE
);

-- Security contexts table
CREATE TABLE IF NOT EXISTS security_contexts (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    encryption_key_id TEXT NOT NULL,
    permissions TEXT NOT NULL,  -- JSON array of permissions
    data TEXT NOT NULL,  -- JSON serialized SecurityContext entity
    expires_at TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    archived BOOLEAN NOT NULL DEFAULT false
);

-- ============================================================================
-- INDEXES FOR PERFORMANCE
-- ============================================================================

-- Pipeline indexes
CREATE INDEX IF NOT EXISTS idx_pipelines_name ON pipelines(name);
CREATE INDEX IF NOT EXISTS idx_pipelines_created_at ON pipelines(created_at);
CREATE INDEX IF NOT EXISTS idx_pipelines_archived ON pipelines(archived);

-- Pipeline stage indexes
CREATE INDEX IF NOT EXISTS idx_pipeline_stages_pipeline_id ON pipeline_stages(pipeline_id);
CREATE INDEX IF NOT EXISTS idx_pipeline_stages_order ON pipeline_stages(pipeline_id, stage_order);

-- Processing session indexes
CREATE INDEX IF NOT EXISTS idx_processing_sessions_pipeline_id ON processing_sessions(pipeline_id);
CREATE INDEX IF NOT EXISTS idx_processing_sessions_user_id ON processing_sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_processing_sessions_status ON processing_sessions(status);
CREATE INDEX IF NOT EXISTS idx_processing_sessions_started_at ON processing_sessions(started_at);

-- File chunk indexes
CREATE INDEX IF NOT EXISTS idx_file_chunks_session_id ON file_chunks(session_id);
CREATE INDEX IF NOT EXISTS idx_file_chunks_chunk_number ON file_chunks(session_id, chunk_number);
CREATE INDEX IF NOT EXISTS idx_file_chunks_processed_at ON file_chunks(processed_at);

-- Security context indexes
CREATE INDEX IF NOT EXISTS idx_security_contexts_user_id ON security_contexts(user_id);
CREATE INDEX IF NOT EXISTS idx_security_contexts_expires_at ON security_contexts(expires_at);

-- ============================================================================
-- VIEWS FOR COMMON QUERIES
-- ============================================================================

-- Active pipelines view
CREATE VIEW IF NOT EXISTS active_pipelines AS
SELECT 
    id,
    name,
    created_at,
    updated_at,
    json_extract(data, '$.stages') as stage_count
FROM pipelines 
WHERE archived = false;

-- Pipeline processing summary view
CREATE VIEW IF NOT EXISTS pipeline_processing_summary AS
SELECT 
    p.id as pipeline_id,
    p.name as pipeline_name,
    COUNT(ps.id) as total_sessions,
    COUNT(CASE WHEN ps.status = 'completed' THEN 1 END) as completed_sessions,
    COUNT(CASE WHEN ps.status = 'running' THEN 1 END) as running_sessions,
    COUNT(CASE WHEN ps.status = 'failed' THEN 1 END) as failed_sessions,
    MAX(ps.started_at) as last_run
FROM pipelines p
LEFT JOIN processing_sessions ps ON p.id = ps.pipeline_id
WHERE p.archived = false
GROUP BY p.id, p.name;

-- File processing progress view
CREATE VIEW IF NOT EXISTS file_processing_progress AS
SELECT 
    ps.id as session_id,
    ps.pipeline_id,
    COUNT(fc.id) as total_chunks,
    COUNT(CASE WHEN fc.processed_at IS NOT NULL THEN 1 END) as processed_chunks,
    ROUND(
        (COUNT(CASE WHEN fc.processed_at IS NOT NULL THEN 1 END) * 100.0) / COUNT(fc.id), 
        2
    ) as progress_percentage,
    SUM(fc.size_bytes) as total_bytes,
    SUM(CASE WHEN fc.processed_at IS NOT NULL THEN fc.size_bytes ELSE 0 END) as processed_bytes
FROM processing_sessions ps
LEFT JOIN file_chunks fc ON ps.id = fc.session_id
WHERE ps.archived = false
GROUP BY ps.id, ps.pipeline_id;

-- ============================================================================
-- SAMPLE DATA INSERTION
-- ============================================================================

-- Insert sample pipelines
INSERT OR IGNORE INTO pipelines (id, name, data, created_at, updated_at, archived) VALUES
(
    '01HN8X9K2M5P7Q8R9S0T1U2V3W',
    'image-processing',
    '{"id":"01HN8X9K2M5P7Q8R9S0T1U2V3W","name":"image-processing","configuration":{"max_file_size":"100MB","output_format":"JPEG","quality":"85"},"metrics":{"total_processed":1250,"total_errors":3,"avg_processing_time_ms":2500},"stages":[{"id":"01HN8X9K2M5P7Q8R9S0T1U2V3X","name":"input_validation","algorithm":"Validation","order":1,"parameters":{}},{"id":"01HN8X9K2M5P7Q8R9S0T1U2V3Y","name":"image_compression","algorithm":"Compression","order":2,"parameters":{"compression_level":"6"}},{"id":"01HN8X9K2M5P7Q8R9S0T1U2V3Z","name":"metadata_extraction","algorithm":"Metadata","order":3,"parameters":{"extract_exif":"true"}}],"created_at":"2025-01-15T10:30:00Z","updated_at":"2025-01-15T10:30:00Z"}',
    '2025-01-15T10:30:00Z',
    '2025-01-15T10:30:00Z',
    false
),
(
    '01HN8X9K2M5P7Q8R9S0T1U2V4A',
    'secure-backup',
    '{"id":"01HN8X9K2M5P7Q8R9S0T1U2V4A","name":"secure-backup","configuration":{"encryption_algorithm":"AES256","key_rotation_days":"30","backup_enabled":"true"},"metrics":{"total_processed":850,"total_errors":1,"avg_processing_time_ms":1200},"stages":["stage4","stage5","stage6","stage7"],"created_at":"2025-01-10T14:20:00Z","updated_at":"2025-01-12T09:15:00Z"}',
    '2025-01-10T14:20:00Z',
    '2025-01-12T09:15:00Z',
    false
),
(
    '01HN8X9K2M5P7Q8R9S0T1U2V5E',
    'data-analytics',
    '{"id":"01HN8X9K2M5P7Q8R9S0T1U2V5E","name":"data-analytics","configuration":{"batch_size":"1000","output_format":"Parquet","compression":"snappy"},"metrics":{"total_processed":5000,"total_errors":12,"avg_processing_time_ms":800},"stages":["stage8","stage9","stage10"],"created_at":"2025-01-05T08:45:00Z","updated_at":"2025-01-07T16:30:00Z"}',
    '2025-01-05T08:45:00Z',
    '2025-01-07T16:30:00Z',
    false
),
(
    '01HN8X9K2M5P7Q8R9S0T1U2V6I',
    'legacy-archive',
    '{"id":"01HN8X9K2M5P7Q8R9S0T1U2V6I","name":"legacy-archive","configuration":{"archive_format":"TAR.GZ","retention_days":"2555"},"metrics":{"total_processed":200,"total_errors":0,"avg_processing_time_ms":5000},"stages":[{"id":"01HN8X9K2M5P7Q8R9S0T1U2V6J","name":"file_collection","algorithm":"Collection","order":1,"parameters":{}},{"id":"01HN8X9K2M5P7Q8R9S0T1U2V6K","name":"compression","algorithm":"Compression","order":2,"parameters":{"level":"9"}},{"id":"01HN8X9K2M5P7Q8R9S0T1U2V6L","name":"archive_storage","algorithm":"Archive","order":3,"parameters":{"location":"cold_storage"}}],"created_at":"2024-12-01T12:00:00Z","updated_at":"2024-12-01T12:00:00Z"}',
    '2024-12-01T12:00:00Z',
    '2024-12-01T12:00:00Z',
    true
);

-- Insert sample processing sessions
INSERT OR IGNORE INTO processing_sessions (id, pipeline_id, user_id, status, data, started_at, completed_at, created_at, updated_at, archived) VALUES
(
    '01HN8XA1B2C3D4E5F6G7H8I9J0',
    '01HN8X9K2M5P7Q8R9S0T1U2V3W',
    'user123@example.com',
    'completed',
    '{"id":"01HN8XA1B2C3D4E5F6G7H8I9J0","pipeline_id":"01HN8X9K2M5P7Q8R9S0T1U2V3W","user_id":"user123@example.com","status":"completed","input_files":["/data/images/photo1.jpg","/data/images/photo2.jpg"],"output_files":["/data/processed/photo1_compressed.jpg","/data/processed/photo2_compressed.jpg"],"metrics":{"processing_time_ms":4500,"files_processed":2,"bytes_processed":15728640}}',
    '2025-01-15T11:00:00Z',
    '2025-01-15T11:04:30Z',
    '2025-01-15T11:00:00Z',
    '2025-01-15T11:04:30Z',
    false
),
(
    '01HN8XA1B2C3D4E5F6G7H8I9K1',
    '01HN8X9K2M5P7Q8R9S0T1U2V4A',
    'admin@company.com',
    'running',
    '{"id":"01HN8XA1B2C3D4E5F6G7H8I9K1","pipeline_id":"01HN8X9K2M5P7Q8R9S0T1U2V4A","user_id":"admin@company.com","status":"running","input_files":["/data/documents/contract.pdf"],"output_files":[],"metrics":{"processing_time_ms":2100,"files_processed":0,"bytes_processed":0}}',
    '2025-01-15T12:30:00Z',
    NULL,
    '2025-01-15T12:30:00Z',
    '2025-01-15T12:32:00Z',
    false
),
(
    '01HN8XA1B2C3D4E5F6G7H8I9L2',
    '01HN8X9K2M5P7Q8R9S0T1U2V5E',
    'analyst@company.com',
    'failed',
    '{"id":"01HN8XA1B2C3D4E5F6G7H8I9L2","pipeline_id":"01HN8X9K2M5P7Q8R9S0T1U2V5E","user_id":"analyst@company.com","status":"failed","input_files":["/data/analytics/dataset.csv"],"output_files":[],"metrics":{"processing_time_ms":1200,"files_processed":0,"bytes_processed":0},"error":"Invalid CSV format in row 1500"}',
    '2025-01-14T09:15:00Z',
    '2025-01-14T09:16:12Z',
    '2025-01-14T09:15:00Z',
    '2025-01-14T09:16:12Z',
    false
);

-- Insert sample file chunks
INSERT OR IGNORE INTO file_chunks (id, session_id, chunk_number, file_path, size_bytes, checksum, data, processed_at, created_at, updated_at, archived) VALUES
(
    '01HN8XB1C2D3E4F5G6H7I8J9K0',
    '01HN8XA1B2C3D4E5F6G7H8I9J0',
    1,
    '/data/images/photo1.jpg',
    7864320,
    'sha256:a1b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef123456',
    '{"id":"01HN8XB1C2D3E4F5G6H7I8J9K0","session_id":"01HN8XA1B2C3D4E5F6G7H8I9J0","chunk_number":1,"file_path":"/data/images/photo1.jpg","size_bytes":7864320,"checksum":"sha256:a1b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef123456","metadata":{"original_format":"JPEG","dimensions":"1920x1080","color_depth":24}}',
    '2025-01-15T11:02:15Z',
    '2025-01-15T11:00:30Z',
    '2025-01-15T11:02:15Z',
    false
),
(
    '01HN8XB1C2D3E4F5G6H7I8J9L1',
    '01HN8XA1B2C3D4E5F6G7H8I9J0',
    2,
    '/data/images/photo2.jpg',
    7864320,
    'sha256:b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef1234567',
    '{"id":"01HN8XB1C2D3E4F5G6H7I8J9L1","session_id":"01HN8XA1B2C3D4E5F6G7H8I9J0","chunk_number":2,"file_path":"/data/images/photo2.jpg","size_bytes":7864320,"checksum":"sha256:b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef1234567","metadata":{"original_format":"JPEG","dimensions":"2560x1440","color_depth":24}}',
    '2025-01-15T11:03:45Z',
    '2025-01-15T11:01:00Z',
    '2025-01-15T11:03:45Z',
    false
),
(
    '01HN8XB1C2D3E4F5G6H7I8J9M2',
    '01HN8XA1B2C3D4E5F6G7H8I9K1',
    1,
    '/data/documents/contract.pdf',
    2097152,
    'sha256:c3d4e5f6789012345678901234567890abcdef1234567890abcdef12345678',
    '{"id":"01HN8XB1C2D3E4F5G6H7I8J9M2","session_id":"01HN8XA1B2C3D4E5F6G7H8I9K1","chunk_number":1,"file_path":"/data/documents/contract.pdf","size_bytes":2097152,"checksum":"sha256:c3d4e5f6789012345678901234567890abcdef1234567890abcdef12345678","metadata":{"original_format":"PDF","pages":25,"encrypted":false}}',
    NULL,
    '2025-01-15T12:30:30Z',
    '2025-01-15T12:30:30Z',
    false
);

-- Insert sample security contexts
INSERT OR IGNORE INTO security_contexts (id, user_id, encryption_key_id, permissions, data, expires_at, created_at, updated_at, archived) VALUES
(
    '01HN8XC1D2E3F4G5H6I7J8K9L0',
    'user123@example.com',
    'key_aes256_001',
    '["read","write","process"]',
    '{"id":"01HN8XC1D2E3F4G5H6I7J8K9L0","user_id":"user123@example.com","encryption_key_id":"key_aes256_001","permissions":["read","write","process"],"session_data":{"login_time":"2025-01-15T10:00:00Z","ip_address":"192.168.1.100","user_agent":"PipelineClient/1.0"}}',
    '2025-02-14T10:00:00Z',
    '2025-01-15T10:00:00Z',
    '2025-01-15T10:00:00Z',
    false
),
(
    '01HN8XC1D2E3F4G5H6I7J8K9M1',
    'admin@company.com',
    'key_aes256_admin',
    '["read","write","process","admin","delete"]',
    '{"id":"01HN8XC1D2E3F4G5H6I7J8K9M1","user_id":"admin@company.com","encryption_key_id":"key_aes256_admin","permissions":["read","write","process","admin","delete"],"session_data":{"login_time":"2025-01-15T08:00:00Z","ip_address":"192.168.1.10","user_agent":"AdminConsole/2.1"}}',
    '2025-01-16T08:00:00Z',
    '2025-01-15T08:00:00Z',
    '2025-01-15T08:00:00Z',
    false
);

-- ============================================================================
-- VERIFICATION QUERIES
-- ============================================================================

-- Show database statistics
SELECT 'pipelines' as table_name, COUNT(*) as record_count FROM pipelines
UNION ALL
SELECT 'processing_sessions' as table_name, COUNT(*) as record_count FROM processing_sessions
UNION ALL
SELECT 'file_chunks' as table_name, COUNT(*) as record_count FROM file_chunks
UNION ALL
SELECT 'security_contexts' as table_name, COUNT(*) as record_count FROM security_contexts;

-- Show active pipelines summary
SELECT * FROM active_pipelines;

-- Show processing summary
SELECT * FROM pipeline_processing_summary;

-- Show file processing progress
SELECT * FROM file_processing_progress;

-- ============================================================================
-- END OF SCRIPT
-- ============================================================================
