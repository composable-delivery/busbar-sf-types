use schemars::schema_for;
use sf_generated_types as sf;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let output_dir = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        PathBuf::from("schemas")
    };

    println!("Generating JSON Schemas to: {}", output_dir.display());
    fs::create_dir_all(&output_dir)?;

    // Generate schema for Settings (Org Shape)
    // We use a Map<String, Value> wrapper approach often for org shape, but here we can
    // generate schemas for specific top-level settings types.

    // Example: SecuritySettings
    let schema = schema_for!(sf::settings::SecuritySettings);
    let path = output_dir.join("SecuritySettings.json");
    fs::write(path, serde_json::to_string_pretty(&schema)?)?;

    // Example: ApexSettings
    let schema = schema_for!(sf::settings::ApexSettings);
    let path = output_dir.join("ApexSettings.json");
    fs::write(path, serde_json::to_string_pretty(&schema)?)?;

    // Metadata Types

    // CustomObject
    #[cfg(feature = "objects")]
    {
        let schema = schema_for!(sf::metadata::objects::CustomObject);
        let path = output_dir.join("CustomObject.json");
        fs::write(path, serde_json::to_string_pretty(&schema)?)?;
    }

    // PermissionSet
    #[cfg(feature = "permissions")]
    {
        let schema = schema_for!(sf::metadata::permissions::PermissionSet);
        let path = output_dir.join("PermissionSet.json");
        fs::write(path, serde_json::to_string_pretty(&schema)?)?;
    }

    // Layout
    #[cfg(feature = "layouts")]
    {
        let schema = schema_for!(sf::metadata::layouts::Layout);
        let path = output_dir.join("Layout.json");
        fs::write(path, serde_json::to_string_pretty(&schema)?)?;
    }

    // Flow
    #[cfg(feature = "flows")]
    {
        let schema = schema_for!(sf::metadata::flows::Flow);
        let path = output_dir.join("Flow.json");
        fs::write(path, serde_json::to_string_pretty(&schema)?)?;
    }

    println!("✅ Schema generation complete.");
    Ok(())
}
