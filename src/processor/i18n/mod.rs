//! Internationalization (i18n) module for managing translated strings
//!
//! This module provides a flexible system for handling multiple language translations
//! with compile-time language selection via feature flags.

#[cfg(feature = "br")]
pub mod messages_br;

#[cfg(feature = "br")]
pub use messages_br as message;

#[cfg(feature = "en")]
pub mod messages;

#[cfg(feature = "en")]
pub use messages as message;
