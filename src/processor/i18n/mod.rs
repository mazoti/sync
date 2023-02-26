//! Contains all strings in translated language

#[cfg(feature = "br")]
pub mod messages_br;
#[cfg(feature = "br")]
pub use messages_br as msgs;

#[cfg(feature = "en")]
pub mod messages;
#[cfg(feature = "en")]
pub use messages as msgs;
