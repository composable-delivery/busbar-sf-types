//! Tests for XML serialization and deserialization
//!
//! This test file verifies that the XmlSerializable trait correctly:
//! 1. Serializes structs to XML with proper Salesforce namespace
//! 2. Deserializes XML back to structs (roundtrip)
//! 3. Handles the XML declaration properly

use busbar_sf_types::settings::org_settings::AccountPlanSettings;
use busbar_sf_types::traits::XmlSerializable;

#[test]
fn test_to_metadata_xml_includes_xml_declaration() {
    let settings = AccountPlanSettings {
        enable_account_plan: true,
    };

    let xml = settings.to_metadata_xml().expect("Failed to serialize to XML");

    // Check that XML declaration is present
    assert!(
        xml.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"),
        "XML should start with declaration"
    );
}

#[test]
fn test_to_metadata_xml_includes_namespace() {
    let settings = AccountPlanSettings {
        enable_account_plan: true,
    };

    let xml = settings.to_metadata_xml().expect("Failed to serialize to XML");

    // Check that namespace is present in the root element
    assert!(
        xml.contains("xmlns=\"http://soap.sforce.com/2006/04/metadata\""),
        "XML should include Salesforce metadata namespace"
    );
}

#[test]
fn test_to_metadata_xml_uses_correct_root_element() {
    let settings = AccountPlanSettings {
        enable_account_plan: true,
    };

    let xml = settings.to_metadata_xml().expect("Failed to serialize to XML");

    // Check that the correct root element is used
    assert!(
        xml.contains("<AccountPlanSettings"),
        "XML should use AccountPlanSettings as root element"
    );
    assert!(
        xml.contains("</AccountPlanSettings>"),
        "XML should close AccountPlanSettings root element"
    );
}

#[test]
fn test_to_metadata_xml_serializes_fields() {
    let settings = AccountPlanSettings {
        enable_account_plan: true,
    };

    let xml = settings.to_metadata_xml().expect("Failed to serialize to XML");

    // Check that fields are serialized with camelCase
    assert!(
        xml.contains("<enableAccountPlan>"),
        "XML should contain enableAccountPlan field"
    );
    assert!(
        xml.contains("true"),
        "XML should contain the field value"
    );
}

#[test]
fn test_from_metadata_xml_deserializes_correctly() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<AccountPlanSettings xmlns="http://soap.sforce.com/2006/04/metadata">
    <enableAccountPlan>true</enableAccountPlan>
</AccountPlanSettings>"#;

    let settings = AccountPlanSettings::from_metadata_xml(xml)
        .expect("Failed to deserialize from XML");

    assert_eq!(settings.enable_account_plan, true);
}

#[test]
fn test_from_metadata_xml_handles_false_value() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<AccountPlanSettings xmlns="http://soap.sforce.com/2006/04/metadata">
    <enableAccountPlan>false</enableAccountPlan>
</AccountPlanSettings>"#;

    let settings = AccountPlanSettings::from_metadata_xml(xml)
        .expect("Failed to deserialize from XML");

    assert_eq!(settings.enable_account_plan, false);
}

#[test]
fn test_roundtrip_serialization() {
    // Create a struct
    let original = AccountPlanSettings {
        enable_account_plan: true,
    };

    // Serialize to XML
    let xml = original.to_metadata_xml().expect("Failed to serialize");

    // Deserialize back
    let deserialized = AccountPlanSettings::from_metadata_xml(&xml)
        .expect("Failed to deserialize");

    // Check that values match
    assert_eq!(original.enable_account_plan, deserialized.enable_account_plan);
}

#[test]
fn test_from_metadata_xml_without_namespace() {
    // quick-xml should handle XML without namespace gracefully
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<AccountPlanSettings>
    <enableAccountPlan>true</enableAccountPlan>
</AccountPlanSettings>"#;

    let settings = AccountPlanSettings::from_metadata_xml(xml)
        .expect("Failed to deserialize XML without namespace");

    assert_eq!(settings.enable_account_plan, true);
}

#[test]
fn test_from_metadata_xml_handles_default_values() {
    // When field is missing, it should use the default value
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<AccountPlanSettings xmlns="http://soap.sforce.com/2006/04/metadata">
</AccountPlanSettings>"#;

    let settings = AccountPlanSettings::from_metadata_xml(xml)
        .expect("Failed to deserialize XML with missing fields");

    // Default value for bool is false
    assert_eq!(settings.enable_account_plan, false);
}

#[test]
fn test_xml_error_display() {
    use busbar_sf_types::traits::XmlError;

    let error = XmlError::Serialize("test error".to_string());
    assert_eq!(
        format!("{}", error),
        "XML serialization error: test error"
    );

    let error = XmlError::Deserialize("test error".to_string());
    assert_eq!(
        format!("{}", error),
        "XML deserialization error: test error"
    );
}
