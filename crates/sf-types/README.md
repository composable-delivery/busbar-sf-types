# sf-generated-types

Auto-generated Salesforce metadata types with modular feature flags.

This crate provides Rust types generated from Salesforce's official [`@salesforce/types`](https://github.com/forcedotcom/wsdl) TypeScript definitions, organized into feature-gated modules for selective compilation.

## Features

Types are organized by domain, allowing you to include only what you need:

### Core Features

| Feature | Description | ~Types |
|---------|-------------|--------|
| `common` | Core enums used across types (default) | 45 |
| `settings` | Org settings for scratch org definitions | 261 |
| `full` | All types (enables everything) | 2680+ |

### Metadata Domain Features (Phase 1)

| Feature | Description | ~Types |
|---------|-------------|--------|
| `objects` | Custom objects, fields, schema types | 29 |
| `layouts` | Page layouts and UI configuration | 20 |
| `permissions` | Permission sets, profiles, sharing rules | 52 |
| `flows` | Flow definitions and process automation | 70 |
| `apex` | Apex classes, triggers, components | 4 |
| `lwc` | Lightning Web Components and Aura | 8 |
| `automation` | Workflow rules, approval processes | 27 |
| `experience` | Experience Cloud and communities | 39 |
| `reports` | Reports and dashboards | 63 |
| `packaging` | 1GP and 2GP package types | 4 |
| `email` | Email templates and configuration | 15 |
| `bots` | Einstein Bots | 37 |
| `integration` | Connected apps, external services | 26 |
| `analytics` | Wave Analytics, CRM Analytics, Tableau CRM | 48 |
| `omnistudio` | OmniStudio, FlexiPages, OmniScripts | 49 |
| `ai` | AI/ML, Einstein, Prompt Templates | 85 |
| `decisions` | Decision Tables, Expression Sets, Business Rules | 60 |
| `datacloud` | Data Cloud, CDP, Data Sources | 52 |
| `servicecloud` | Service Cloud, Cases, Knowledge, Embedded Service | 65 |
| `servicecatalog` | Service Catalog items and configurations | 35 |
| `messaging` | Messaging, Chat, Conversations | 97 |
| `loyalty` | Loyalty Cloud, Programs, Benefits | 25 |
| `identity` | Identity, SAML, OAuth, Authentication | 35 |
| `scheduling` | Scheduling, Field Service, Appointments | 23 |
| `batch` | Batch Jobs, Async Processing, Queues | 45 |
| `quickactions` | Quick Actions, Global Actions, Action Links | 26 |
| `custommetadata` | Custom Metadata, Custom Labels, Custom Tabs | 22 |
| `recordactions` | Record Actions, Record Aggregations | 23 |
| `externalapps` | External Client Apps, Mobile Apps | 46 |
| `discovery` | Einstein Discovery, Predictive Analytics | 29 |
| `marketing` | Marketing Cloud Connect, Campaigns | 34 |
| `search` | Search customization and configuration | 17 |
| `platformevents` | Platform Events, Event Subscriptions | 32 |
| `useraccess` | User Access Policies, User Provisioning | 15 |
| `activation` | Context Definitions, Activation Platform | 55 |
| `industries` | Industry Cloud (Health, Insurance, Manufacturing) | 22 |
| `commerce` | Commerce Cloud, Web Stores, Products | 13 |
| `fieldmappings` | Field Mappings, Field Restrictions | 19 |

### Extended Domain Features (Phase 2)

| Feature | Description | ~Types |
|---------|-------------|--------|
| `copilot` | Einstein Copilot, Agentforce, Assistants | 14 |
| `forecasting` | Sales Forecasting, Advanced Account Forecasting | 29 |
| `nba` | Next Best Action, Recommendations, Strategy Builder | 16 |
| `omnichannel` | Omni-Channel routing, Presence, Skills | 12 |
| `mobilesecurity` | Mobile Security Policies, Mobile Apps | 8 |
| `documents` | Document Templates, Content Assets, OCR | 26 |
| `stages` | Sales Stages, Path Assistant | 17 |
| `cpq` | Configure-Price-Quote, Pricing Rules | 14 |
| `applications` | Applications, App Menus, App Branding | 19 |
| `callcenter` | Call Center, CTI, Telephony Integration | 7 |
| `businessrules` | Business Processes, Rule Definitions | 22 |
| `assessments` | Assessments, Surveys, Question Sets | 8 |
| `visualization` | Einstein Analytics Visualizations | 8 |
| `telemetry` | Telemetry Definitions, Usage Tracking | 8 |
| `navigation` | Navigation Menus, Navigation Links | 5 |
| `transactionsecurity` | Transaction Security Policies | 5 |
| `invocable` | Invocable Actions, Action Extensions | 6 |
| `channels` | Channel Definitions, Channel Layouts | 7 |
| `folders` | Folders, Folder Sharing, Public Access | 11 |
| `reputation` | Community Reputation, Points, Levels | 6 |
| `digitalexperience` | Digital Experience Configuration | 6 |
| `briefcase` | Field Service Briefcase, Offline Data | 6 |
| `lifesciences` | Life Sciences Cloud, Clinical | 8 |
| `translations` | Translations, Localization | 3 |
| `managedcontent` | Managed Content, CMS | 7 |
| `objectmappings` | Object Mappings, Object Relationships | 14 |
| `components` | Component Instances, Configuration | 6 |
| `feed` | Feed Items, Feed Layouts | 7 |
| `codecoverage` | Apex Code Coverage, Test Results | 8 |
| `slack` | Slack Integration, Record Layouts | 2 |
| `explainability` | AI Explainability, Message Templates | 5 |
| `portals` | Portal Configuration, Portal Roles | 4 |
| `picklists` | Picklist Values, Translations | 4 |
| `relatedrecords` | Related Records, Related Content | 8 |
| `homepage` | Home Page Components, Layouts | 2 |
| `uiconfig` | UI Configuration, Formulas, Formats | 7 |
| `conditions` | Condition Types, Condition Filters | 5 |
| `policies` | Security Policies, Policy Rules | 8 |
| `features` | Feature Parameters, Configuration | 5 |
| `mappings` | General Mapping Types, Operations | 5 |
| `deploy` | Deploy Options, Details, Problems | 5 |
| `lightning` | Lightning Bolt, Framework Types | 7 |
| `workspace` | Workspace Mappings, Console Config | 2 |
| `standard` | Standard Permission Sets, Fields | 1 |

### Use Case Bundles

Convenience feature combinations for common use cases:

| Bundle | Features Included |
|--------|-------------------|
| `org-shape` | Org settings capture (`settings`) |
| `deployments` | Deploy operations (`objects`, `layouts`, `permissions`, `apex`, `flows`, `automation`) |
| `metadata-etl` | Full metadata access (`full`) |
| `package-dev` | Package development (`packaging`, `objects`, `permissions`, `apex`) |
| `experience-dev` | Experience Cloud (`experience`, `objects`, `layouts`, `permissions`) |
| `data-operations` | Data Cloud (`datacloud`, `batch`, `analytics`) |
| `service-operations` | Service Cloud (`servicecloud`, `servicecatalog`, `messaging`, `quickactions`) |
| `ai-dev` | AI/ML development (`ai`, `bots`, `discovery`) |
| `industry-dev` | Industry Cloud (`industries`, `servicecloud`, `commerce`, `lifesciences`) |
| `sales-operations` | Sales ops (`forecasting`, `cpq`, `stages`, `nba`) |
| `content-management` | Content mgmt (`documents`, `managedcontent`, `feed`) |

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
# For org settings only (minimal footprint, ~5s compile)
sf-generated-types = { version = "0.1", features = ["settings"] }

# For deployment operations (~20s compile)
sf-generated-types = { version = "0.1", features = ["deployments"] }

# For AI/ML development
sf-generated-types = { version = "0.1", features = ["ai-dev"] }

# For sales operations
sf-generated-types = { version = "0.1", features = ["sales-operations"] }

# For all types (~1.5min compile)
sf-generated-types = { version = "0.1", features = ["full"] }
```

## Usage

### Basic Types

```rust
use sf_generated_types::{ApexSettings, SecuritySettings};

let settings = ApexSettings {
    enable_compile_on_deploy: Some(true),
    enable_debug_logs_during_deployment: Some(true),
    ..Default::default()
};

// Serialize to JSON (for scratch org definitions)
let json = serde_json::to_string_pretty(&settings)?;
```

### Common Enums

Common enums are always available:

```rust
use sf_generated_types::{FieldType, DeployStatus, SharingModel};

let field_type = FieldType::Text;
let status = DeployStatus::Succeeded;
```

### Domain-Specific Types

Access types from specific Salesforce products:

```rust
// AI/ML types
use sf_generated_types::metadata::ai::{Prompt, MlModelType};

// OmniStudio types
use sf_generated_types::metadata::omnistudio::{OmniScript, FlexiPage};

// Data Cloud types
use sf_generated_types::metadata::datacloud::{DataSource, DataConnector};

// Service Cloud types
use sf_generated_types::metadata::servicecloud::{EmbeddedServiceFlowConfig};

// Einstein Copilot types
use sf_generated_types::metadata::copilot::{AssistantDefinition};

// Sales Forecasting types
use sf_generated_types::metadata::forecasting::{ForecastingDateType};

// Next Best Action types
use sf_generated_types::metadata::nba::{RecommendationStrategy};
```

### Traits for Type-Safe APIs

The `traits` module provides traits for building type-safe API clients:

```rust
use sf_generated_types::traits::{MetadataType, SettingsType};

// Generic function that works with any metadata type
fn get_type_name<T: MetadataType>() -> &'static str {
    T::METADATA_TYPE_NAME
}

// Settings types can be used in scratch org definitions
fn to_scratch_def<T: SettingsType>(setting: &T) -> serde_json::Value {
    serde_json::json!({
        T::SCRATCH_DEF_KEY: setting
    })
}
```

### With busbar-sf-api

This crate is designed to work with [busbar-sf-api](https://crates.io/crates/busbar-sf-api) for type-safe Salesforce API operations:

```rust
use busbar_sf_rest::SalesforceRestClient;
use sf_generated_types::CustomObject;

// Query with typed results
let objects: Vec<CustomObject> = client
    .query("SELECT Id, FullName FROM CustomObject")
    .await?;
```

## Module Structure

```
sf-generated-types/
├── common/              # Core enums (FieldType, DeployStatus, etc.)
├── metadata/            # Core metadata types (80+ modules)
│   ├── objects.rs       # CustomObject, CustomField, etc.
│   ├── layouts.rs       # Layout, LayoutSection, etc.
│   ├── permissions.rs   # PermissionSet, Profile, etc.
│   ├── flows.rs         # Flow, FlowVariable, etc.
│   ├── apex.rs          # ApexClass, ApexTrigger, etc.
│   ├── lwc.rs           # LightningComponentBundle, etc.
│   ├── ai.rs            # Prompt, MlModel*, AiEvaluation*, etc.
│   ├── copilot.rs       # AssistantDefinition, AssistantSkill*, etc.
│   ├── omnistudio.rs    # OmniScript, FlexiPage, etc.
│   ├── forecasting.rs   # Forecasting*, AdvAcct*, etc.
│   ├── nba.rs           # Recommendation*, Strategy*, etc.
│   ├── datacloud.rs     # DataSource, DataConnector, etc.
│   ├── servicecloud.rs  # EmbeddedService*, Case*, etc.
│   ├── ... (70+ more domain modules)
│   └── mod.rs           # Module exports
├── settings/            # Org settings types (~261 types)
├── packaging/           # Package types (Package2, etc.)
├── traits.rs            # MetadataType, SettingsType traits
└── uncategorized.rs     # Types not yet categorized (~627 types)
```

## Categorization Coverage

| Metric | Phase 0 | Phase 1 | Phase 2 (Current) |
|--------|---------|---------|-------------------|
| Categories | 15 | 39 | 83 |
| Categorized Types | ~838 (31%) | ~1,671 (62%) | **~2,054 (77%)** |
| Uncategorized Types | ~1,843 (69%) | ~1,010 (38%) | **~627 (23%)** |
| `uncategorized.rs` Lines | 35,658 | 22,479 | **15,855** |

The remaining uncategorized types are mostly:
- Small groups (1-4 types) that don't form clear categories
- Internal test types (`Ftest*`)
- Edge cases and deprecated types

## Regenerating Types

Types are generated from `@salesforce/types` using the `sf-wsdl` crate:

```bash
cd crates/sf-wsdl
cargo run --bin generate_from_typescript
```

This fetches the latest TypeScript definitions from GitHub and generates:
- Modular Rust files organized by category
- A monolithic file for backward compatibility
- Trait implementations for type-safe API usage

### Adding New Categories

To improve categorization, edit `crates/sf-wsdl/src/categories.rs`:

```rust
TypeCategory {
    name: "your_category",
    feature: "yourcategory",
    module_path: "metadata/yourcategory.rs",
    description: "Description of types in this category",
    patterns: &["YourPrefix*", "*YourSuffix"],
    explicit_types: &["ExactTypeName1", "ExactTypeName2"],
},
```

Then regenerate:

```bash
cargo run --bin generate_from_typescript
```

### Version Tracking

The generator fetches from:
- URL: `https://raw.githubusercontent.com/forcedotcom/wsdl/main/src/metadata.ts`
- Repository: [forcedotcom/wsdl](https://github.com/forcedotcom/wsdl)

## Serialization

All types implement `Serialize` and `Deserialize` from serde, with:

- `camelCase` field naming (matching Salesforce JSON)
- `#[serde(default)]` for optional fields
- `#[serde(skip_serializing_if = "Option::is_none")]` to omit null values

### JSON (Scratch Org Definitions)

```rust
use sf_generated_types::SecuritySettings;

let settings = SecuritySettings::default();
let json = serde_json::to_value(&settings)?;

// Use in scratch org definition
let scratch_def = serde_json::json!({
    "orgName": "My Scratch Org",
    "edition": "Developer",
    "settings": {
        "securitySettings": json
    }
});
```

### XML (Metadata API)

For XML serialization (Metadata API deploy/retrieve), use the `quick-xml` crate:

```rust
use quick_xml::se::to_string;

let xml = to_string(&settings)?;
```

## Build Times

| Feature | Approx. Build Time |
|---------|-------------------|
| `settings` only | ~5 seconds |
| `deployments` | ~20 seconds |
| `ai-dev` | ~15 seconds |
| `sales-operations` | ~15 seconds |
| `service-operations` | ~20 seconds |
| `full` | ~1.5 minutes |

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
## XML Serialization

All Salesforce metadata types now support bi-directional XML serialization through the `XmlSerializable` trait. This allows you to convert between Rust structs and Salesforce Metadata API XML format.

### Usage

```rust
use busbar_sf_types::settings::org_settings::AccountPlanSettings;
use busbar_sf_types::traits::XmlSerializable;

// Create a settings object
let settings = AccountPlanSettings {
    enable_account_plan: true,
};

// Serialize to XML
let xml = settings.to_metadata_xml()?;
// Output:
// <?xml version="1.0" encoding="UTF-8"?>
// <AccountPlanSettings xmlns="http://soap.sforce.com/2006/04/metadata">
//     <enableAccountPlan>true</enableAccountPlan>
// </AccountPlanSettings>

// Deserialize from XML
let deserialized = AccountPlanSettings::from_metadata_xml(&xml)?;
```

### Features

- **Automatic namespace handling**: The Salesforce namespace `xmlns="http://soap.sforce.com/2006/04/metadata"` is automatically added during serialization
- **XML declaration**: Proper XML declaration is included in all serialized output
- **Roundtrip safe**: Serialization and deserialization preserve all data
- **Error handling**: Clear error messages for serialization/deserialization failures
- **Works with all types**: Any type implementing `MetadataType` automatically supports XML serialization

### Examples

See `examples/xml_demo.rs` for a complete working example:

```bash
cargo run --example xml_demo --features settings
```

