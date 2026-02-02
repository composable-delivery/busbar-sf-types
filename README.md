# busbar-sf-types

Modular, feature-flagged Rust types for Salesforce metadata, **automatically generated** from [`@salesforce/types`](https://www.npmjs.com/package/@salesforce/types).

This is a **fully codegen-built crate**—no manual type definitions. All Rust types are automatically generated from Salesforce metadata definitions via the [`@salesforce/types`](https://github.com/forcedotcom/wsdl) npm package.

## About Generated Types

This repository contains:
- **Generated types** in `crates/sf-types/src/metadata/` (derived from Salesforce metadata)
- **Codegen tooling** in `sf-typegen/` (Rust tool that performs the generation)

Every version of this crate corresponds to a version of `@salesforce/types`. The crate version is aligned with the npm package version:
- `@salesforce/types@1.0.0` → crate `1.0.0`
- `@salesforce/types@1.1.0` → crate `1.1.0`  
- `@salesforce/types@1.1.0` (codegen fix) → crate `1.1.1`

See [ATTRIBUTION.md](ATTRIBUTION.md) for licensing and attribution requirements.

## Reproducibility

The generated nature of this crate is explicit and reproducible:

```bash
# To regenerate from scratch:
npm install  # Fetches @salesforce/types
bash scripts/generate.sh
# Output in crates/sf-types/src/ will exactly match what's committed
```

## Components

This workspace consists of two main crates:

1. **`busbar-sf-types`**: The library crate containing auto-generated Rust structs and enums
2. **`sf-typegen`**: The codegen toolchain that parses `@salesforce/types` and produces modular Rust code

---

## busbar-sf-types Library

The core library provides Rust types generated from Salesforce's [`@salesforce/types`](https://github.com/forcedotcom/wsdl) metadata definitions.

### Features

Types are organized by domain, allowing you to include only what you need to keep compile times fast.

#### Core Features

| Feature | Description | ~Types |
|---------|-------------|--------|
| `common` | Core enums used across types (default) | 45 |
| `settings` | Org settings for scratch org definitions | 261 |
| `full` | All types (enables everything) | 2680+ |

#### Popular Domain Features

| Feature | Description |
|---------|-------------|
| `objects` | Custom objects, fields, schema types |
| `permissions` | Permission sets, profiles, sharing rules |
| `flows` | Flow definitions and process automation |
| `apex` | Apex classes, triggers, components |
| `lwc` | Lightning Web Components and Aura |
| `ai` | AI/ML, Einstein, Prompt Templates |
| `datacloud` | Data Cloud, CDP, Data Sources |
| `servicecloud` | Service Cloud, Cases, Knowledge |

*(See `crates/sf-types/Cargo.toml` for the full list of 80+ supported categories)*

### Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
# For org settings only (minimal footprint, ~5s compile)
sf-generated-types = { git = "https://github.com/kantext-dev/busbar-sf-types", features = ["settings"] }

# For deployment operations (~20s compile)
sf-generated-types = { git = "https://github.com/kantext-dev/busbar-sf-types", features = ["deployments"] }

# For JSON Schema support
sf-generated-types = { git = "https://github.com/kantext-dev/busbar-sf-types", features = ["full", "schemars"] }
```

#### Example Code

```rust
use sf_generated_types::{ApexSettings, SecuritySettings, FieldType};
use sf_generated_types::metadata::ai::Prompt;

fn main() {
    let settings = ApexSettings {
        enable_compile_on_deploy: Some(true),
        enable_debug_logs_during_deployment: Some(true),
        ..Default::default()
    };
    
    // Core enum
    let field_type = FieldType::Text;
    
    // Domain-specific type
    let prompt = Prompt {
        master_label: "My Prompt".to_string(),
        ..Default::default()
    };
}
```

## JSON Schemas

This crate supports generating JSON Schemas for Salesforce metadata types via the `schemars` feature.

### Generating Schemas

You can run the included schema generator to produce JSON schema files for types like `CustomObject`, `PermissionSet`, etc.:

```bash
cargo run -p sf-generated-types --bin schema-gen --features "full schemars" -- schemas/
```

This will output files like `schemas/CustomObject.json` and `schemas/SecuritySettings.json`.

---

## Generating Types

The types are generated using the `sf-wsdl` crate included in this repository.

### Prerequisites

- Rust toolchain (stable)

### How to Regenerate

1. Navigate to the generator directory:
   ```bash
   cd sf-wsdl
   ```

2. Run the generator:
   ```bash
   cargo run --bin generate_from_typescript -- --output-dir ../sf-generated-types/src
   ```

This command will:
1. Fetch the latest `metadata.ts` from `forcedotcom/wsdl` on GitHub.
2. Parse the TypeScript AST.
3. Apply **manual documentation overlays** (descriptions injected for key types like `CustomObject`).
4. Apply categorization rules defined in `sf-wsdl/src/categories.rs`.
5. Generate modular Rust files in `sf-generated-types/src/metadata/`.

### Documentation Overlays

Since the source `@salesforce/types` definitions do not include comments, we support a manual overlay system to inject documentation into the generated Rust code (and resulting JSON schemas).

These are defined in `sf-wsdl/src/bin/generate_from_typescript.rs` in the `get_overlays()` function.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.