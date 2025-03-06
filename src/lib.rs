//! Kraken API client library
//! 
//! This crate provides a Rust interface to the Kraken cryptocurrency exchange API.
//! It supports both public and private API endpoints, as well as WebSocket connections
//! for real-time data.

pub mod client;
pub mod error;
pub mod models;
pub mod api;
pub mod auth;
pub mod config;
pub mod utils;

// Re-export commonly used types
pub use client::KrakenClient;
pub use error::Error;
pub use config::Config;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
