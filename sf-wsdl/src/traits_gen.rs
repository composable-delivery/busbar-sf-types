//! Stub module for trait generation
//!
//! This module provides stub implementations for trait generation functionality.
//! The traits.rs and trait_impls.rs files are currently maintained manually.

use std::collections::HashMap;

/// Configuration for trait generation
#[derive(Debug, Clone, Default)]
pub struct TraitGenConfig {
    // Placeholder for future configuration options
}

/// Generate the traits module content
///
/// Currently returns empty string as traits.rs is maintained manually
pub fn generate_traits_module() -> String {
    // Traits are maintained manually in sf-generated-types/src/traits.rs
    String::new()
}

/// Generate trait implementations for types
///
/// Currently returns empty string as trait_impls.rs is maintained manually
pub fn generate_all_trait_impls(
    _struct_names: &[String],
    _fields_by_type: &HashMap<String, Vec<String>>,
    _config: &TraitGenConfig,
) -> String {
    // Trait implementations are maintained manually in sf-generated-types/src/trait_impls.rs
    String::new()
}
