//! # arb_poc
//!
//! A proof-of-concept arbitrage trading system for cryptocurrency exchanges.
//!
//! This crate provides abstractions for interacting with multiple cryptocurrency
//! exchanges and implementing arbitrage strategies.
//!
//! ## Architecture
//!
//! The crate is organized into the following modules:
//!
//! - [`exchange`]: Common traits and types for exchange abstraction
//! - [`exchanges`]: Implementations for specific exchanges (e.g., Upbit)
//! - [`config`]: Configuration management

pub mod config;
pub mod exchange;
pub mod exchanges;
