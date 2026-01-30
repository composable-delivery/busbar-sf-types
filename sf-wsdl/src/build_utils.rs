//! Build script utilities for WSDL processing

pub use crate::error::{Result, WsdlError};
pub use crate::generator::WsdlGenerator;
pub use crate::processor::{ProcessorConfig, WsdlProcessor};

/// Convenience function for simple WSDL generation in build scripts
///
/// # Example
///
/// ```rust,ignore
/// // In build.rs
/// use kantext_sf_wsdl::build_utils::generate_wsdl_types;
///
/// fn main() {
///     generate_wsdl_types(&[
///         ("partner", "wsdl/partner.wsdl"),
///         ("metadata", "wsdl/metadata.wsdl"),
///     ]).expect("Failed to generate WSDL types");
/// }
/// ```
pub fn generate_wsdl_types(wsdl_files: &[(&str, &str)]) -> Result<()> {
    let mut generator = WsdlGenerator::new();

    for (name, path) in wsdl_files {
        generator = generator.add_wsdl(name, path);
    }

    generator.generate()
}

/// Generate WSDL types with custom configuration
///
/// # Example
///
/// ```rust,ignore
/// // In build.rs
/// use kantext_sf_wsdl::build_utils::{generate_wsdl_types_with_config, ProcessorConfig};
///
/// fn main() {
///     let config = ProcessorConfig {
///         derive_traits: vec!["Debug".to_string(), "Clone".to_string()],
///         apply_fixes: true,
///         remove_duplicates: true,
///         flatten_wrappers: false,
///     };
///     
///     generate_wsdl_types_with_config(
///         &[("partner", "wsdl/partner.wsdl")],
///         config
///     ).expect("Failed to generate WSDL types");
/// }
/// ```
pub fn generate_wsdl_types_with_config(
    wsdl_files: &[(&str, &str)],
    config: ProcessorConfig,
) -> Result<()> {
    let mut generator = WsdlGenerator::new().with_config(config);

    for (name, path) in wsdl_files {
        generator = generator.add_wsdl(name, path);
    }

    generator.generate()
}
