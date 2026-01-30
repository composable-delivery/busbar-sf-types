//! Error types for WSDL processing

use thiserror::Error;

/// Result type alias for WSDL operations
pub type Result<T> = std::result::Result<T, WsdlError>;

/// Errors that can occur during WSDL processing
#[derive(Error, Debug)]
pub enum WsdlError {
    /// IO error when reading WSDL files
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Error parsing WSDL/XSD content
    #[error("Parse error: {0}")]
    Parse(String),

    /// Error during code generation
    #[error("Code generation error: {0}")]
    CodeGeneration(String),

    /// Error during post-processing
    #[error("Post-processing error: {0}")]
    PostProcessing(String),

    /// XSD parser error
    #[error("XSD parser error: {0}")]
    XsdParser(String),

    /// Regex error
    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// Missing required file or resource
    #[error("Missing resource: {0}")]
    MissingResource(String),

    /// Invalid WSDL structure
    #[error("Invalid WSDL structure: {0}")]
    InvalidWsdl(String),

    /// Network error during WSDL fetching
    #[error("Network error: {0}")]
    Network(String),

    /// Authentication error
    #[error("Authentication error: {0}")]
    Authentication(String),

    /// Resource not found
    #[error("Not found: {0}")]
    NotFound(String),

    /// Build script error
    #[error("Build script error: {0}")]
    BuildScript(String),
}
