//! Generator for Salesforce trait implementations.
//!
//! This module handles generating the Rust code that implements core traits
//! like `MetadataType` and `SettingsType` for the generated structs.
//!
//! Goals:
//! - No monolithic `generated_salesforce_types` dependency
//! - Trait impls "go along with the types": compile under the same feature flags
//! - Use a generated `prelude` module so impl generation doesn't need module paths

use convert_case::{Case, Casing};
use std::collections::HashMap;

/// Configuration for trait generation
#[derive(Debug, Clone, Default)]
pub struct TraitGenConfig {
    /// Whether to generate JsonSerializable implementations
    pub generate_json_serializable: bool,
    /// Whether to generate SettingsType implementations
    pub generate_settings_type: bool,
    /// Custom overrides for XML root elements
    pub xml_root_overrides: HashMap<String, String>,
}

/// Generates the content for the `traits.rs` module.
///
/// This returns the static content of the traits module which defines
/// the `MetadataType`, `JsonSerializable`, etc. traits.
pub fn generate_traits_module() -> String {
    // This is essentially a copy of the content found in sf-generated-types/src/traits.rs
    // We return it here so the generator can write it out if needed.
    r#"//! Core traits for type-safe Salesforce API interactions.
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
"#
    .to_string()
}

/// Generates trait implementations for a list of structs, with an optional feature-gate per type.
///
/// - `feature_by_type`: map `TypeName -> Some("feature")` to wrap impls in `#[cfg(feature = "...")]`.
///   If `None` or missing, impls are emitted without a cfg guard.
///
/// Note: The generated `trait_impls.rs` is expected to `use crate::prelude::*;` so that
/// type names can be referenced directly without needing to know their module paths.
pub fn generate_all_trait_impls(
    struct_names: &[String],
    fields_by_type: &HashMap<String, Vec<String>>,
    config: &TraitGenConfig,
    feature_by_type: Option<&HashMap<String, String>>,
) -> String {
    let mut output = String::new();

    for name in struct_names {
        let feature_guard = feature_by_type
            .and_then(|m| m.get(name))
            .map(|s| s.as_str());

        // Determine if it has a 'fullName' field, which usually maps to api_name
        let has_full_name = fields_by_type
            .get(name)
            .map(|fields| fields.contains(&"fullName".to_string()))
            .unwrap_or(false);

        output.push_str(&generate_metadata_type_impl(
            name,
            has_full_name,
            config,
            feature_guard,
        ));

        // SettingsType & JsonSerializable impl if it looks like a settings type
        if name.ends_with("Settings") {
            output.push_str(&generate_settings_type_impl(name, feature_guard));
        }
    }

    output
}

fn generate_metadata_type_impl(
    name: &str,
    has_full_name: bool,
    config: &TraitGenConfig,
    feature_guard: Option<&str>,
) -> String {
    let xml_root = config
        .xml_root_overrides
        .get(name)
        .map(|s| s.as_str())
        .unwrap_or(name);

    // Heuristic: most top-level metadata types support wildcard.
    let supports_wildcard = true;

    let api_name_impl = if has_full_name {
        r#"        self.full_name.as_deref()"#
    } else {
        r#"        None"#
    };

    let cfg_line = feature_guard
        .map(|f| format!("#[cfg(feature = \"{}\")]\n", f))
        .unwrap_or_default();

    format!(
        r#"
{cfg_line}impl crate::traits::MetadataType for {name} {{
    const METADATA_TYPE_NAME: &'static str = "{name}";
    const XML_ROOT_ELEMENT: &'static str = "{xml_root}";
    const IS_FOLDER_TYPE: bool = false;
    const SUPPORTS_WILDCARD: bool = {supports_wildcard};

    fn api_name(&self) -> Option<&str> {{
{api_name_impl}
    }}
}}
"#
    )
}

fn generate_settings_type_impl(name: &str, feature_guard: Option<&str>) -> String {
    // e.g., SecuritySettings -> securitySettings
    // This is a simple heuristic: camelCase the type name.
    let scratch_key = name.to_case(Case::Camel);

    let cfg_line = feature_guard
        .map(|f| format!("#[cfg(feature = \"{}\")]\n", f))
        .unwrap_or_default();

    format!(
        r#"
{cfg_line}impl crate::traits::JsonSerializable for {name} {{}}

{cfg_line}impl crate::traits::SettingsType for {name} {{
    const SCRATCH_DEF_KEY: &'static str = "{scratch_key}";
}}
"#
    )
}
