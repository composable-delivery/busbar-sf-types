//! Example demonstrating XML serialization and deserialization
//! 
//! This example shows how to use the XmlSerializable trait to convert
//! Salesforce metadata types to/from XML format with proper namespace.

use busbar_sf_types::settings::org_settings::AccountPlanSettings;
use busbar_sf_types::traits::XmlSerializable;

fn main() {
    println!("=== XML Serialization Demo ===\n");

    // Create a settings object
    let settings = AccountPlanSettings {
        enable_account_plan: true,
    };

    println!("1. Original struct:");
    println!("   {:?}\n", settings);

    // Serialize to XML
    let xml = settings.to_metadata_xml()
        .expect("Failed to serialize to XML");

    println!("2. Serialized to XML:");
    println!("{}\n", xml);

    // Deserialize from XML
    let deserialized = AccountPlanSettings::from_metadata_xml(&xml)
        .expect("Failed to deserialize from XML");

    println!("3. Deserialized back to struct:");
    println!("   {:?}\n", deserialized);

    // Verify roundtrip
    println!("4. Roundtrip verification:");
    println!("   enable_account_plan: {} -> {} ✓",
        settings.enable_account_plan,
        deserialized.enable_account_plan
    );

    println!("\n=== Success! ===");
}
