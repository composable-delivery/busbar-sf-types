pub mod error;
pub mod categories;
pub mod modular_generator;

#[cfg(feature = "build-utils")]
pub mod build_utils;

pub use error::{Result, WsdlError};
pub use categories::{find_category, TypeCategory, CATEGORIES};
pub use modular_generator::{
    FieldDef, GenerationResult, ModularGenerator, ModularGeneratorConfig, TypeDefinitions,
};
