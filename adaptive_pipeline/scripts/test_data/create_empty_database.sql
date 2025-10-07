-- ============================================================================
-- SQLite Empty Database Creation Script
-- ============================================================================
-- 
-- Purpose: Creates an empty database schema for the optimized adaptive 
--          pipeline system with new GenericId format
-- 
-- Usage: sqlite3 test_pipeline.db < create_empty_database.sql
-- 
-- Author: Generated for Phase 3 SQLite Integration
-- Date: 2025-07-07
-- ============================================================================

-- Enable foreign key constraints
PRAGMA foreign_keys = ON;

-- ============================================================================
-- TABLE CREATION (SCHEMA ONLY - NO DATA)
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
    input_file_path TEXT,
    output_file_path TEXT,
    progress_percentage REAL NOT NULL DEFAULT 0.0,
    bytes_processed INTEGER NOT NULL DEFAULT 0,
    total_bytes INTEGER NOT NULL DEFAULT 0,
    error_message TEXT,
    started_at TEXT NOT NULL,
    completed_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    archived BOOLEAN NOT NULL DEFAULT false,
    FOREIGN KEY (pipeline_id) REFERENCES pipelines(id) ON DELETE CASCADE
);

-- File chunks table (for chunk-level tracking)
CREATE TABLE IF NOT EXISTS file_chunks (
    id TEXT PRIMARY KEY,
    session_id TEXT NOT NULL,
    chunk_number INTEGER NOT NULL,
    chunk_size INTEGER NOT NULL,
    checksum TEXT,
    processing_status TEXT NOT NULL,
    error_message TEXT,
    started_at TEXT,
    completed_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    archived BOOLEAN NOT NULL DEFAULT false,
    FOREIGN KEY (session_id) REFERENCES processing_sessions(id) ON DELETE CASCADE
);

-- Security contexts table
CREATE TABLE IF NOT EXISTS security_contexts (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    permissions TEXT NOT NULL,  -- JSON array of permissions
    encryption_key_id TEXT,
    access_level TEXT NOT NULL,
    expires_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    archived BOOLEAN NOT NULL DEFAULT false
);

-- ============================================================================
-- INDEXES FOR PERFORMANCE
-- ============================================================================

-- Pipeline indexes
CREATE INDEX IF NOT EXISTS idx_pipelines_name ON pipelines(name);
CREATE INDEX IF NOT EXISTS idx_pipelines_archived ON pipelines(archived);
CREATE INDEX IF NOT EXISTS idx_pipelines_created_at ON pipelines(created_at);

-- Pipeline stages indexes
CREATE INDEX IF NOT EXISTS idx_pipeline_stages_pipeline_id ON pipeline_stages(pipeline_id);
CREATE INDEX IF NOT EXISTS idx_pipeline_stages_order ON pipeline_stages(pipeline_id, stage_order);
CREATE INDEX IF NOT EXISTS idx_pipeline_stages_archived ON pipeline_stages(archived);

-- Processing sessions indexes
CREATE INDEX IF NOT EXISTS idx_processing_sessions_pipeline_id ON processing_sessions(pipeline_id);
CREATE INDEX IF NOT EXISTS idx_processing_sessions_user_id ON processing_sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_processing_sessions_status ON processing_sessions(status);
CREATE INDEX IF NOT EXISTS idx_processing_sessions_started_at ON processing_sessions(started_at);
CREATE INDEX IF NOT EXISTS idx_processing_sessions_archived ON processing_sessions(archived);

-- File chunks indexes
CREATE INDEX IF NOT EXISTS idx_file_chunks_session_id ON file_chunks(session_id);
CREATE INDEX IF NOT EXISTS idx_file_chunks_chunk_number ON file_chunks(session_id, chunk_number);
CREATE INDEX IF NOT EXISTS idx_file_chunks_status ON file_chunks(processing_status);
CREATE INDEX IF NOT EXISTS idx_file_chunks_archived ON file_chunks(archived);

-- Security contexts indexes
CREATE INDEX IF NOT EXISTS idx_security_contexts_user_id ON security_contexts(user_id);
CREATE INDEX IF NOT EXISTS idx_security_contexts_access_level ON security_contexts(access_level);
CREATE INDEX IF NOT EXISTS idx_security_contexts_expires_at ON security_contexts(expires_at);
CREATE INDEX IF NOT EXISTS idx_security_contexts_archived ON security_contexts(archived);

-- ============================================================================
-- VERIFICATION QUERIES
-- ============================================================================

-- Show table count (should be 5 tables)
SELECT 'Table count:' as info, COUNT(*) as count 
FROM sqlite_master 
WHERE type='table' AND name NOT LIKE 'sqlite_%';

-- Show index count
SELECT 'Index count:' as info, COUNT(*) as count 
FROM sqlite_master 
WHERE type='index' AND name NOT LIKE 'sqlite_%';

-- Show all tables
SELECT 'Tables created:' as info, name as table_name 
FROM sqlite_master 
WHERE type='table' AND name NOT LIKE 'sqlite_%'
ORDER BY name;
