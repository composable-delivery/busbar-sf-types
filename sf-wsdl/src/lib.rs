pub mod categories;
pub mod error;
pub mod modular_generator;
pub mod traits_gen;

#[cfg(feature = "build-utils")]
pub mod build_utils;

pub use categories::{find_category, TypeCategory, CATEGORIES};
pub use error::{Result, WsdlError};
pub use modular_generator::{
    FieldDef, GenerationResult, ModularGenerator, ModularGeneratorConfig, TypeDefinitions,
};
