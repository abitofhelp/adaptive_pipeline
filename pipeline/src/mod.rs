//! Root module for the pipeline application
//! 
//! This application follows a hybrid architecture combining:
//! - Domain-Driven Design (DDD) for business modeling
//! - Clean Architecture for layer separation
//! - Hexagonal Architecture (Ports & Adapters) for dependency inversion

pub mod core;
pub mod infrastructure;
pub mod interface;
