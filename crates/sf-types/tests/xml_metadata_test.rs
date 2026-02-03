//! Tests for XML serialization with metadata types
//!
//! This test file verifies that XML serialization works correctly
//! with non-settings metadata types and collections.

use busbar_sf_types::metadata::apex::ApexTestSuite;
use busbar_sf_types::traits::XmlSerializable;

#[test]
fn test_metadata_type_with_vec_serialization() {
    let test_suite = ApexTestSuite {
        test_class_name: vec![
            "TestClass1".to_string(),
            "TestClass2".to_string(),
            "TestClass3".to_string(),
        ],
    };

    let xml = test_suite.to_metadata_xml().expect("Failed to serialize");

    // Verify XML structure
    assert!(xml.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
    assert!(xml.contains("xmlns=\"http://soap.sforce.com/2006/04/metadata\""));
    assert!(xml.contains("<ApexTestSuite"));
    assert!(xml.contains("</ApexTestSuite>"));

    // Verify array elements are present
    assert!(xml.contains("<testClassName>TestClass1</testClassName>"));
    assert!(xml.contains("<testClassName>TestClass2</testClassName>"));
    assert!(xml.contains("<testClassName>TestClass3</testClassName>"));
}

#[test]
fn test_metadata_type_with_vec_deserialization() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<ApexTestSuite xmlns="http://soap.sforce.com/2006/04/metadata">
    <testClassName>TestClass1</testClassName>
    <testClassName>TestClass2</testClassName>
    <testClassName>TestClass3</testClassName>
</ApexTestSuite>"#;

    let test_suite = ApexTestSuite::from_metadata_xml(xml).expect("Failed to deserialize");

    assert_eq!(test_suite.test_class_name.len(), 3);
    assert_eq!(test_suite.test_class_name[0], "TestClass1");
    assert_eq!(test_suite.test_class_name[1], "TestClass2");
    assert_eq!(test_suite.test_class_name[2], "TestClass3");
}

#[test]
fn test_metadata_type_with_vec_roundtrip() {
    let original = ApexTestSuite {
        test_class_name: vec!["MyTestClass".to_string(), "AnotherTestClass".to_string()],
    };

    let xml = original.to_metadata_xml().expect("Failed to serialize");
    let deserialized = ApexTestSuite::from_metadata_xml(&xml).expect("Failed to deserialize");

    assert_eq!(original.test_class_name, deserialized.test_class_name);
}

#[test]
fn test_metadata_type_with_empty_vec() {
    let test_suite = ApexTestSuite {
        test_class_name: vec![],
    };

    let xml = test_suite.to_metadata_xml().expect("Failed to serialize");

    // Empty vec should still produce valid XML
    assert!(xml.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
    assert!(xml.contains("xmlns=\"http://soap.sforce.com/2006/04/metadata\""));
    assert!(xml.contains("ApexTestSuite"));

    // Deserialize back
    let deserialized = ApexTestSuite::from_metadata_xml(&xml).expect("Failed to deserialize");

    assert_eq!(deserialized.test_class_name.len(), 0);
}
