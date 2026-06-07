//! JSON Schema generator for Salesforce metadata types using schemars.

use busbar_sf_types::schema_registry::{all_schema_types, schema_for_type};
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let output_dir = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        // Assume running from workspace root
        PathBuf::from("site/public/schemas")
    };

    fs::create_dir_all(&output_dir)?;

    let mut generated = Vec::new();

    for type_name in all_schema_types() {
        if let Some(json) = schema_for_type(type_name) {
            let path = output_dir.join(format!("{}.json", type_name));
            fs::write(&path, serde_json::to_string_pretty(&json)?)?;
            generated.push(type_name.to_string());
        }
    }

    let index_path = output_dir.join("index.json");
    fs::write(&index_path, serde_json::to_string_pretty(&generated)?)?;

    println!(
        "schema-gen: wrote {} schemas to {}",
        generated.len(),
        output_dir.display()
    );

    Ok(())
}
