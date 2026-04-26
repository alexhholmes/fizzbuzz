pub mod exception;
pub mod user;

/// Marker trait for models that may be stored in the Redis cache.
/// Implement via `#[derive(Cacheable)]`.
pub trait Cacheable {}
