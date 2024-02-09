//! # Common Functionality
//! The vast majority of functionality is in this library crate. The client and server executable crates are really just wiring up bits and pieces from this library.

pub mod message_parser;

mod byte;
pub mod processor;
pub mod message;
pub mod net;
mod string;
pub mod config;
pub mod crypto;

pub use config::DomainConfig;