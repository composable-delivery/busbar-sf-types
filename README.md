# busbar-sf-types

Modular, feature-flagged Rust types for Salesforce metadata, **automatically generated** from [`@salesforce/types`](https://www.npmjs.com/package/@salesforce/types).

This is a **fully codegen-built crate**—no manual type definitions. All Rust types are automatically generated from Salesforce metadata definitions via the [`@salesforce/types`](https://github.com/forcedotcom/wsdl) npm package.

📊 **[Explore the interactive type dependency graph →](https://composable-delivery.github.io/busbar-sf-types/)**

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

The types are generated using the `sf-typegen` crate included in this repository.

### Prerequisites

- Rust toolchain (1.91.0 or later)
- Node.js and npm (for `@salesforce/types`)

### How to Regenerate

```bash
bash scripts/generate.sh
```

This command will:
1. Fetch the latest `metadata.ts` from `@salesforce/types` npm package
2. Parse the TypeScript AST using oxc parser
3. **Build a type dependency graph** to analyze relationships between types
4. **Perform graph-based categorization** to automatically assign types to modules
5. Apply **manual documentation overlays** (descriptions for key types)
6. Apply categorization rules defined in `sf-typegen/src/categories.rs`
7. Generate modular Rust files in `crates/sf-types/src/metadata/`
8. Export type graph to `assets/type-graph.json` and `assets/type-graph.dot`
9. **Copy type graph to `docs/type-graph.json` for GitHub Pages deployment**

### Type Dependency Graph

The generator now includes a sophisticated type dependency graph that:

- **Analyzes** TypeScript AST to build a directed graph of type relationships
- **Categorizes** types automatically based on their dependencies
- **Detects** shared types used by multiple categories (assigned to "common" module)
- **Exports** to JSON and DOT formats for analysis and visualization

The graph contains:
- **2682 nodes** (types from Salesforce metadata)
- **4906 edges** (dependency relationships)
- **267 shared types** automatically identified and moved to common module

See [`assets/README.md`](assets/README.md) for detailed information on using the type graph.

#### Interactive Type Explorer

Explore the type dependency graph interactively in your browser:

**🔗 [Launch Salesforce Types Explorer](https://composable-delivery.github.io/busbar-sf-types/)**

The interactive explorer provides:
- 🔍 **Search** - Find types by name across 2600+ types
- 📊 **Category Browsing** - Browse types organized into 80+ categories
- 🎯 **Dependency Visualization** - See type relationships with different edge types:
  - **Contains** - Type A has a field of Type B
  - **Extends** - Type A extends/inherits from Type B  
  - **Generic** - Type A is a generic instantiation of Type B
- 📈 **Statistics** - View graph metrics and category distributions
- 🎨 **Color-coded** - Types colored by category for easy identification

To visualize the graph locally with Graphviz:
```bash
dot -Tpng assets/type-graph.dot -o type-graph.png
```

### Memory Requirements

**Note:** Building with `--all-features` requires >8GB of memory and may fail in memory-constrained environments (CI runners, devcontainers). This is due to the large number of feature flags (80+).

For development and CI, we recommend:
- Use specific feature subsets: `cargo build --features "objects,flows,apex"`
- Or use default features: `cargo build` (includes common types only)
- Full feature compilation works on machines with 16GB+ RAM

### Documentation Overlays

Since the source `@salesforce/types` definitions do not include comments, we support a manual overlay system to inject documentation into the generated Rust code (and resulting JSON schemas).

These are defined in `sf-typegen/src/bin/generate_from_typescript.rs` in the `apply_overlays()` function.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.