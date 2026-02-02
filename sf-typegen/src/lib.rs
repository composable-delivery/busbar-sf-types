pub mod categories;
pub mod error;
pub mod modular_generator;
pub mod traits_gen;

// Build utilities were historically used for WSDL-based generation.
// Those modules have been removed/renamed as this crate is now a TypeScript → Rust type generator.
// Keep this module behind an opt-in feature only once it's been updated to the new architecture.
//#[cfg(feature = "build-utils")]
//pub mod build_utils;

/// Re-export error types.
///
/// NOTE: the crate is now a type generator (not WSDL-specific), but the error type
/// name is kept for compatibility with existing internal code.
pub use error::{Result, WsdlError};

pub use categories::{CATEGORIES, TypeCategory, find_category};

pub use modular_generator::{
    FieldDef, GenerationResult, ModularGenerator, ModularGeneratorConfig, TypeDefinitions,
};
