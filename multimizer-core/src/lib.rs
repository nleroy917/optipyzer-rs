//!
//! # Multimizer Core
//!
//! Multimizer Core is a library that provides the core functionality of the Multimizer project -- a codon optimization toolkit.
//!
//!
pub mod consts;
pub mod models;
pub mod optimizations;
pub mod utils;

#[cfg(feature = "sqlite")]
pub mod db;
