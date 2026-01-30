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
///
/// This trait provides the fundamental metadata about a type needed for
/// API operations like deploy and retrieve.
///
/// # Example
///
/// ```rust,ignore
/// use sf_generated_types::traits::MetadataType;
/// use sf_generated_types::CustomObject;
///
/// // Get metadata type info
/// println!("Type: {}", CustomObject::METADATA_TYPE_NAME);
///
/// // Use in generic API calls
/// async fn deploy<T: MetadataType>(client: &MetadataClient, items: &[T]) {
///     let type_name = T::METADATA_TYPE_NAME;
///     // ...
/// }
/// ```
pub trait MetadataType: Serialize + DeserializeOwned + Send + Sync + Clone + 'static {
    /// The Metadata API type name (e.g., "CustomObject", "ApexClass").
    /// This matches the `<types>` element in package.xml.
    const METADATA_TYPE_NAME: &'static str;

    /// The XML root element name for this type.
    /// Usually the same as METADATA_TYPE_NAME but may differ for some types.
    const XML_ROOT_ELEMENT: &'static str;

    /// Whether this type is contained in a folder (e.g., reports, email templates).
    const IS_FOLDER_TYPE: bool = false;

    /// Whether this type supports wildcard (*) in package.xml.
    const SUPPORTS_WILDCARD: bool = true;

    /// Get the API name (fullName) of this metadata component.
    fn api_name(&self) -> Option<&str>;

    /// Get the folder path if this is a folder-based type.
    fn folder_path(&self) -> Option<&str> {
        None
    }

    /// Get the full path for package.xml (folder/name for folder types, just name otherwise).
    fn full_name(&self) -> Option<String> {
        match (self.folder_path(), self.api_name()) {
            (Some(folder), Some(name)) => Some(format!("{}/{}", folder, name)),
            (None, Some(name)) => Some(name.to_string()),
            _ => None,
        }
    }
}

/// Trait for types that support JSON serialization for scratch org definitions.
///
/// Settings types in Salesforce can be specified in scratch org definition files
/// as JSON, which then gets converted to XML for deployment.
///
/// # Example
///
/// ```rust,ignore
/// use sf_generated_types::traits::JsonSerializable;
/// use sf_generated_types::SecuritySettings;
///
/// let settings = SecuritySettings::default();
/// let json = settings.to_scratch_def_json()?;
/// ```
pub trait JsonSerializable: MetadataType {
    /// Convert to scratch org definition JSON format.
    fn to_scratch_def_json(&self) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::to_value(self)
    }

    /// Parse from scratch org definition JSON.
    fn from_scratch_def_json(value: serde_json::Value) -> Result<Self, serde_json::Error> {
        serde_json::from_value(value)
    }
}

/// Trait specifically for org Settings types.
///
/// Settings types have a special key name used in scratch org definition files
/// under the `settings` object.
///
/// # Example
///
/// ```rust,ignore
/// use sf_generated_types::traits::SettingsType;
/// use sf_generated_types::SecuritySettings;
///
/// // Build scratch org definition
/// let mut settings = serde_json::Map::new();
/// settings.insert(
///     SecuritySettings::SCRATCH_DEF_KEY.to_string(),
///     security_settings.to_scratch_def_json()?
/// );
/// ```
pub trait SettingsType: JsonSerializable {
    /// The settings key name in scratch org definitions (e.g., "securitySettings").
    /// This is typically the camelCase version of the type name without "Settings" suffix.
    const SCRATCH_DEF_KEY: &'static str;
}

/// Trait for types that support XML serialization (Metadata API format).
///
/// This trait enables conversion to/from the XML format used by the
/// Salesforce Metadata API for deploy and retrieve operations.
///
/// # Example
///
/// ```rust,ignore
/// use sf_generated_types::traits::XmlSerializable;
/// use sf_generated_types::CustomObject;
///
/// let obj = CustomObject { /* ... */ };
/// let xml = obj.to_metadata_xml()?;
/// ```
pub trait XmlSerializable: MetadataType {
    /// Serialize to Salesforce metadata XML format.
    fn to_metadata_xml(&self) -> Result<String, XmlError>;

    /// Parse from Salesforce metadata XML format.
    fn from_metadata_xml(xml: &str) -> Result<Self, XmlError>;
}

/// Trait for SObject types that can be queried via SOQL.
///
/// This is separate from MetadataType because SObjects represent
/// data records, not metadata configuration.
///
/// # Example
///
/// ```rust,ignore
/// use sf_generated_types::traits::SObjectType;
///
/// async fn query<T: SObjectType>(client: &RestClient, filter: &str) -> Vec<T> {
///     let soql = format!("SELECT {} FROM {} WHERE {}",
///         T::default_fields().join(", "),
///         T::SOBJECT_NAME,
///         filter
///     );
///     client.query(&soql).await
/// }
/// ```
pub trait SObjectType: Serialize + DeserializeOwned + Send + Sync + Clone + 'static {
    /// The SObject API name (e.g., "Account", "Contact", "CustomObject__c").
    const SOBJECT_NAME: &'static str;

    /// Whether this is a custom object (name ends with __c).
    const IS_CUSTOM: bool = false;

    /// Get the record ID if present.
    fn id(&self) -> Option<&str>;

    /// Get the default fields to query for this SObject.
    fn default_fields() -> &'static [&'static str] {
        &["Id"]
    }
}

/// Trait for types that represent package components.
///
/// These types can be added to a Package (package.xml) for deployment.
pub trait PackageComponent: MetadataType {
    /// Get the package.xml type member entry for this component.
    fn package_member(&self) -> Option<String> {
        self.full_name()
    }
}

// Blanket implementation: all MetadataTypes are PackageComponents
impl<T: MetadataType> PackageComponent for T {}
