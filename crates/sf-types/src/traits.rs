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
    /// Serialize to Salesforce Metadata API XML format with proper namespace.
    fn to_metadata_xml(&self) -> Result<String, XmlError> {
        // Serialize the struct to XML string using quick-xml
        let xml = quick_xml::se::to_string(self).map_err(|e| XmlError::Serialize(e.to_string()))?;

        // Inject namespace into the root tag
        // Match the opening root tag specifically: <TagName> or <TagName />
        // This regex ensures we only match the first occurrence of the opening tag
        let root_pattern = format!(
            r"^(<{}\s*/?>|<{}\s+[^>]*>)",
            Self::XML_ROOT_ELEMENT,
            Self::XML_ROOT_ELEMENT
        );

        // For safety, we'll use a more precise approach:
        // Find the first occurrence of "<{XML_ROOT_ELEMENT}" that's followed by either '>' or whitespace
        let search_pattern = format!("<{}", Self::XML_ROOT_ELEMENT);
        if let Some(pos) = xml.find(&search_pattern) {
            // Find the end of the opening tag (either '>' or '/>')
            if let Some(end_pos) = xml[pos..].find('>') {
                let tag_end = pos + end_pos;
                let opening_tag = &xml[pos..=tag_end];

                // Check if it's a self-closing tag
                let is_self_closing = opening_tag.ends_with("/>");

                // Create replacement with namespace
                let replacement = if is_self_closing {
                    format!(
                        "<{} xmlns=\"http://soap.sforce.com/2006/04/metadata\"/>",
                        Self::XML_ROOT_ELEMENT
                    )
                } else {
                    format!(
                        "<{} xmlns=\"http://soap.sforce.com/2006/04/metadata\">",
                        Self::XML_ROOT_ELEMENT
                    )
                };

                // Replace the opening tag
                let xml_with_ns = format!("{}{}{}", &xml[..pos], replacement, &xml[tag_end + 1..]);

                // Add XML declaration
                return Ok(format!(
                    "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n{}",
                    xml_with_ns
                ));
            }
        }

        // Fallback: if we couldn't find/parse the tag properly, return an error
        Err(XmlError::Serialize(format!(
            "Could not find root element '{}' in serialized XML",
            Self::XML_ROOT_ELEMENT
        )))
    }

    /// Deserialize from Salesforce Metadata API XML format.
    fn from_metadata_xml(xml: &str) -> Result<Self, XmlError> {
        quick_xml::de::from_str(xml).map_err(|e| XmlError::Deserialize(e.to_string()))
    }
}

/// Blanket implementation: all MetadataType types automatically support XML serialization
impl<T: MetadataType> XmlSerializable for T {}

/// Trait for types that represent package components.
pub trait PackageComponent: MetadataType {
    fn package_member(&self) -> Option<String> {
        self.full_name()
    }
}

impl<T: MetadataType> PackageComponent for T {}
