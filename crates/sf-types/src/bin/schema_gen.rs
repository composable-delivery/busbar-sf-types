//! Temporary schema generator stub.
//!
//! This crate intends to support schema generation via `schemars`, but the generated
//! Salesforce types do not yet reliably derive `schemars::JsonSchema` across all
//! modules/features. CI runs clippy with `-D warnings`, so this binary must compile
//! cleanly under the feature set that enables it.
//!
//! Once `JsonSchema` derives (and any required overlays) are fully supported, replace
//! this stub with a real schema generator that emits JSON Schemas for selected types.

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

    // Keep behavior predictable for CI/artifacts: ensure the directory exists and
    // write a small marker file so users can tell this ran.
    fs::create_dir_all(&output_dir)?;

    let marker_path = output_dir.join("SCHEMA_GEN_STUB.txt");
    let marker_contents = "\
schema-gen is currently a stub.

Reason:
- busbar-sf-types does not yet guarantee `schemars::JsonSchema` derives for all generated types.

Next steps:
- implement consistent `#[cfg_attr(feature = \"schemars\", derive(schemars::JsonSchema))]`
  on generated structs/enums (including settings + all metadata modules)
- add any required overlays for types that need special schemars handling
- then replace this stub with real schema generation output.
";
    fs::write(marker_path, marker_contents.as_bytes())?;

    println!(
        "schema-gen stub: wrote marker file under: {}",
        output_dir.display()
    );

    Ok(())
}
