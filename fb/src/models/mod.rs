pub mod exception;
pub mod user;

/// Marker trait for types that can be stored in the Redis cache via [`crate::cache::Cache`].
/// Implement with `#[cache(key = "prefix", field = "field_name")]`, which generates
/// `cache_key()` and satisfies the `Serialize` supertrait requirement.
pub trait Cacheable: serde::Serialize {
    fn cache_key(&self) -> String;
}
