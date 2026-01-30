//! Auto-generated Salesforce metadata types from @salesforce/types.
//!
//! This crate contains Rust types generated from the Salesforce Metadata API.
//! Types are organized into feature-gated modules for selective compilation.
//!
//! # Features
//!
//! - `common` - Core enums used across types (default)
//! - `settings` - Org settings types (~227 types) for scratch org definitions
//! - `objects` - Custom objects, fields, and schema types
//! - `layouts` - Page layouts and UI configuration
//! - `permissions` - Permission sets, profiles, and sharing rules
//! - `flows` - Flow definitions and process automation
//! - `apex` - Apex classes, triggers, and components
//! - `lwc` - Lightning Web Components and Aura bundles
//! - `automation` - Workflow rules and approval processes
//! - `experience` - Experience Cloud and communities
//! - `reports` - Reports, dashboards, and analytics
//! - `packaging` - 1GP and 2GP package types
//! - `email` - Email templates and configuration
//! - `bots` - Einstein Bots and conversational AI
//! - `integration` - Connected apps and external services
//! - `full` - All types (enables all features)
//!
//! # Usage
//!
//! For org settings only:
//! ```toml
//! sf-generated-types = { version = "0.1", features = ["settings"] }
//! ```
//!
//! For deployment operations:
//! ```toml
//! sf-generated-types = { version = "0.1", features = ["deployments"] }
//! ```
//!
//! For all types:
//! ```toml
//! sf-generated-types = { version = "0.1", features = ["full"] }
//! ```

#![allow(non_camel_case_types)]

// Common types (always available)
pub mod common;
pub use common::*;

// Core traits for type-safe API usage
pub mod traits;

// Feature-gated modules

#[cfg(any(feature = "activation", feature = "ai", feature = "analytics", feature = "apex", feature = "applications", feature = "assessments", feature = "automation", feature = "batch", feature = "bots", feature = "briefcase", feature = "businessrules", feature = "callcenter", feature = "channels", feature = "codecoverage", feature = "commerce", feature = "components", feature = "conditions", feature = "copilot", feature = "cpq", feature = "custommetadata", feature = "datacloud", feature = "decisions", feature = "deploy", feature = "digitalexperience", feature = "discovery", feature = "documents", feature = "email", feature = "experience", feature = "explainability", feature = "externalapps", feature = "features", feature = "feed", feature = "fieldmappings", feature = "flows", feature = "folders", feature = "forecasting", feature = "homepage", feature = "identity", feature = "industries", feature = "integration", feature = "invocable", feature = "layouts", feature = "lifesciences", feature = "lightning", feature = "loyalty", feature = "lwc", feature = "managedcontent", feature = "mappings", feature = "marketing", feature = "messaging", feature = "mobilesecurity", feature = "navigation", feature = "nba", feature = "objectmappings", feature = "objects", feature = "omnichannel", feature = "omnistudio", feature = "permissions", feature = "picklists", feature = "platformevents", feature = "policies", feature = "portals", feature = "quickactions", feature = "recordactions", feature = "relatedrecords", feature = "reports", feature = "reputation", feature = "scheduling", feature = "search", feature = "servicecatalog", feature = "servicecloud", feature = "slack", feature = "stages", feature = "standard", feature = "telemetry", feature = "transactionsecurity", feature = "translations", feature = "uiconfig", feature = "useraccess", feature = "visualization", feature = "workspace"))]
pub mod metadata;
#[cfg(any(feature = "activation", feature = "ai", feature = "analytics", feature = "apex", feature = "applications", feature = "assessments", feature = "automation", feature = "batch", feature = "bots", feature = "briefcase", feature = "businessrules", feature = "callcenter", feature = "channels", feature = "codecoverage", feature = "commerce", feature = "components", feature = "conditions", feature = "copilot", feature = "cpq", feature = "custommetadata", feature = "datacloud", feature = "decisions", feature = "deploy", feature = "digitalexperience", feature = "discovery", feature = "documents", feature = "email", feature = "experience", feature = "explainability", feature = "externalapps", feature = "features", feature = "feed", feature = "fieldmappings", feature = "flows", feature = "folders", feature = "forecasting", feature = "homepage", feature = "identity", feature = "industries", feature = "integration", feature = "invocable", feature = "layouts", feature = "lifesciences", feature = "lightning", feature = "loyalty", feature = "lwc", feature = "managedcontent", feature = "mappings", feature = "marketing", feature = "messaging", feature = "mobilesecurity", feature = "navigation", feature = "nba", feature = "objectmappings", feature = "objects", feature = "omnichannel", feature = "omnistudio", feature = "permissions", feature = "picklists", feature = "platformevents", feature = "policies", feature = "portals", feature = "quickactions", feature = "recordactions", feature = "relatedrecords", feature = "reports", feature = "reputation", feature = "scheduling", feature = "search", feature = "servicecatalog", feature = "servicecloud", feature = "slack", feature = "stages", feature = "standard", feature = "telemetry", feature = "transactionsecurity", feature = "translations", feature = "uiconfig", feature = "useraccess", feature = "visualization", feature = "workspace"))]
pub use metadata::*;

#[cfg(feature = "packaging")]
pub mod packaging;
#[cfg(feature = "packaging")]
pub use packaging::*;

#[cfg(feature = "settings")]
pub mod settings;
#[cfg(feature = "settings")]
pub use settings::*;

// Uncategorized types (available with full feature)
#[cfg(feature = "full")]
pub mod uncategorized;
#[cfg(feature = "full")]
pub use uncategorized::*;

// Trait implementations (requires full feature)
#[cfg(feature = "full")]
mod trait_impls;

// Backward compatibility: monolithic generated file
#[cfg(feature = "full")]
#[doc(hidden)]
pub mod generated_salesforce_types;
