//! Core traits for type-safe Salesforce API interactions.
//!
//! These traits can be used as bounds in API client generics to enable
//! type-safe metadata operations.

use serde::{de::DeserializeOwned, Serialize};

/// Error type for XML serialization operations
#[derive(Debug, Clone)]
pub enum XmlError {
    /// Error during serialization
    Serialize(String),
    /// Error during deserialization
    Deserialize(String),
}

impl std::fmt::Display for XmlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            XmlError::Serialize(msg) => write!(f, "XML serialization error: {}", msg),
            XmlError::Deserialize(msg) => write!(f, "XML deserialization error: {}", msg),
        }
    }
}

impl std::error::Error for XmlError {}

/// Core trait for any Salesforce metadata type that can be deployed/retrieved.
pub trait MetadataType: Serialize + DeserializeOwned + Send + Sync + Clone + 'static {
    /// The Metadata API type name (e.g., "CustomObject", "ApexClass").
    const METADATA_TYPE_NAME: &'static str;

    /// The XML root element name for this type.
    const XML_ROOT_ELEMENT: &'static str;

    /// Whether this type is contained in a folder.
    const IS_FOLDER_TYPE: bool = false;

    /// Whether this type supports wildcard (*) in package.xml.
    const SUPPORTS_WILDCARD: bool = true;

    /// Get the API name (fullName) of this metadata component.
    fn api_name(&self) -> Option<&str>;

    /// Get the folder path if this is a folder-based type.
    fn folder_path(&self) -> Option<&str> {
        None
    }

    /// Get the full path for package.xml.
    fn full_name(&self) -> Option<String> {
        match (self.folder_path(), self.api_name()) {
            (Some(folder), Some(name)) => Some(format!("{}/{}", folder, name)),
            (None, Some(name)) => Some(name.to_string()),
            _ => None,
        }
    }
}

/// Trait for types that support JSON serialization for scratch org definitions.
pub trait JsonSerializable: MetadataType {
    fn to_scratch_def_json(&self) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::to_value(self)
    }

    fn from_scratch_def_json(value: serde_json::Value) -> Result<Self, serde_json::Error> {
        serde_json::from_value(value)
    }
}

/// Trait specifically for org Settings types.
pub trait SettingsType: JsonSerializable {
    /// The settings key name in scratch org definitions (e.g., "securitySettings").
    const SCRATCH_DEF_KEY: &'static str;
}

/// Trait for types that support XML serialization (Metadata API format).
pub trait XmlSerializable: MetadataType {
    fn to_metadata_xml(&self) -> Result<String, XmlError>;
    fn from_metadata_xml(xml: &str) -> Result<Self, XmlError>;
}

/// Trait for types that represent package components.
pub trait PackageComponent: MetadataType {
    fn package_member(&self) -> Option<String> {
        self.full_name()
    }
}

impl<T: MetadataType> PackageComponent for T {}
