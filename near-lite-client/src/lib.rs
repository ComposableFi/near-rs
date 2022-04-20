//! # Near lite client
//!
//! The main purpose of the lite client is to keep track of a small subset
//! of the chain's state while still being able to:
//! 1. verify the chain's state transitions and keep a subset of the state
//! 2. verify that a transaction belongs to a vald block

mod block_validation;
pub mod checkpoint;
pub mod client;
mod error;
mod merkle_tree;
mod signature;
mod storage;
pub mod types;
mod verifier;
