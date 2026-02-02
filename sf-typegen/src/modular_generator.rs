//! Modular code generator for Salesforce types.
//!
//! This module generates Rust code organized into categorized modules with
//! proper feature flags, enabling consumers to pull in only the types they need.
//!
//! NOTE: We intentionally avoid generating `settings/settings.rs` because it forces
//! `settings/mod.rs` to contain `pub mod settings;`, which triggers Clippy's
//! `module_inception` lint under `-D warnings`. Instead, the settings category
//! output file is renamed to `settings/org_settings.rs` and re-exported from
//! `settings/mod.rs`.
//!
//! Implementation details:
//! - The settings category module file is renamed from `settings` -> `org_settings`.
//! - Any category whose module_path would naturally generate a `settings/settings.rs`
//!   will instead generate `settings/org_settings.rs`.

use crate::categories::{
    find_category, group_by_category, CategorizedType, TypeCategory, CATEGORIES,
};
use crate::traits_gen::{generate_all_trait_impls, generate_traits_module, TraitGenConfig};
use convert_case::{Case, Casing};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::path::Path;

/// Rust reserved keywords that need escaping with r#
const RESERVED_WORDS: &[&str] = &[
    "type", "fn", "let", "const", "mut", "ref", "if", "else", "match", "loop", "while", "for",
    "in", "return", "break", "continue", "struct", "enum", "trait", "impl", "pub", "priv", "use",
    "mod", "crate", "self", "Self", "super", "unsafe", "async", "await", "move", "where", "as",
    "dyn", "abstract", "become", "box", "do", "final", "macro", "override", "try",
];

/// The domain/category feature flags that enable generated type modules.
///
/// This list is used to:
/// - generate a feature-gated `prelude.rs` that re-exports enabled types
/// - gate compilation of `trait_impls.rs` when any type-bearing feature is enabled
const DOMAIN_FEATURES: &[&str] = &[
    "settings",
    "objects",
    "layouts",
    "permissions",
    "flows",
    "apex",
    "lwc",
    "automation",
    "experience",
    "reports",
    "packaging",
    "email",
    "bots",
    "integration",
    "analytics",
    "omnistudio",
    "ai",
    "decisions",
    "datacloud",
    "servicecloud",
    "servicecatalog",
    "messaging",
    "loyalty",
    "identity",
    "scheduling",
    "batch",
    "quickactions",
    "custommetadata",
    "recordactions",
    "externalapps",
    "discovery",
    "marketing",
    "search",
    "platformevents",
    "useraccess",
    "activation",
    "industries",
    "commerce",
    "fieldmappings",
    "copilot",
    "forecasting",
    "nba",
    "omnichannel",
    "mobilesecurity",
    "documents",
    "stages",
    "cpq",
    "applications",
    "callcenter",
    "businessrules",
    "assessments",
    "visualization",
    "telemetry",
    "navigation",
    "transactionsecurity",
    "invocable",
    "channels",
    "folders",
    "reputation",
    "digitalexperience",
    "briefcase",
    "lifesciences",
    "translations",
    "managedcontent",
    "objectmappings",
    "components",
    "feed",
    "codecoverage",
    "slack",
    "explainability",
    "portals",
    "picklists",
    "relatedrecords",
    "homepage",
    "uiconfig",
    "conditions",
    "policies",
    "features",
    "mappings",
    "deploy",
    "lightning",
    "workspace",
    "standard",
];

/// Field definition extracted from TypeScript
#[derive(Debug, Clone)]
pub struct FieldDef {
    pub name: String,
    pub type_ref: String,
    pub optional: bool,
    pub is_array: bool,
    pub description: Option<String>,
}

/// Container for extracted type definitions
#[derive(Debug, Default)]
pub struct TypeDefinitions {
    /// Union types (become Rust enums)
    pub union_types: HashMap<String, Vec<String>>,
    /// Interface types (become Rust structs)
    pub interface_types: HashMap<String, Vec<FieldDef>>,
    pub descriptions: HashMap<String, String>,
}

/// Configuration for the modular generator
#[derive(Debug, Clone)]
pub struct ModularGeneratorConfig {
    /// Base output directory for generated files
    pub output_dir: String,
    /// Whether to generate a single monolithic file as well
    pub generate_monolithic: bool,
    /// Whether to generate trait implementations
    pub generate_traits: bool,
    /// Trait generation configuration
    pub trait_config: TraitGenConfig,
    /// Whether to generate the lib.rs with feature gates
    pub generate_lib_rs: bool,
    /// Whether to generate a feature-gated prelude module that re-exports enabled types.
    ///
    /// This is used to allow trait impl generation to refer to types without depending
    /// on a monolithic output, while still preserving slim builds via feature gates.
    pub generate_prelude_rs: bool,
}

impl Default for ModularGeneratorConfig {
    fn default() -> Self {
        Self {
            output_dir: "../sf-generated-types/src".to_string(),
            generate_monolithic: false,
            generate_traits: true,
            trait_config: TraitGenConfig::default(),
            generate_lib_rs: true,
            generate_prelude_rs: true,
        }
    }
}

/// Tracks submodules that need to be aggregated in mod.rs files
#[derive(Debug, Default)]
struct ModuleTracker {
    /// Map of directory path -> list of (submodule_name, description, feature)
    submodules: HashMap<String, Vec<(String, String, String)>>,
}

impl ModuleTracker {
    fn add_submodule(
        &mut self,
        dir_path: &str,
        module_name: &str,
        description: &str,
        feature: &str,
    ) {
        // Don't add "mod" as a module name - it's a reserved keyword
        if module_name == "mod" {
            return;
        }
        self.submodules
            .entry(dir_path.to_string())
            .or_default()
            .push((
                module_name.to_string(),
                description.to_string(),
                feature.to_string(),
            ));
    }
}

/// Modular generator for Salesforce types
pub struct ModularGenerator {
    config: ModularGeneratorConfig,
}

impl ModularGenerator {
    /// Create a new modular generator with the given configuration
    pub fn new(config: ModularGeneratorConfig) -> Self {
        Self { config }
    }

    /// Generate all modular output from type definitions
    pub fn generate(&self, defs: &TypeDefinitions) -> anyhow::Result<GenerationResult> {
        let output_dir = Path::new(&self.config.output_dir);
        fs::create_dir_all(output_dir)?;

        let mut result = GenerationResult::default();
        let mut module_tracker = ModuleTracker::default();

        // Categorize all types
        let enum_names: Vec<String> = defs.union_types.keys().cloned().collect();
        let struct_names: Vec<String> = defs.interface_types.keys().cloned().collect();

        let categorized_enums: Vec<CategorizedType> = enum_names
            .iter()
            .map(|name| CategorizedType {
                name: name.clone(),
                category: find_category(name),
                is_enum: true,
            })
            .collect();

        let categorized_structs: Vec<CategorizedType> = struct_names
            .iter()
            .map(|name| CategorizedType {
                name: name.clone(),
                category: find_category(name),
                is_enum: false,
            })
            .collect();

        // Combine for grouping
        let all_types: Vec<CategorizedType> = categorized_enums
            .iter()
            .chain(categorized_structs.iter())
            .cloned()
            .collect();

        let groups = group_by_category(&all_types);

        // Build a set of types in each category for cross-reference resolution
        let mut types_by_category: HashMap<&str, HashSet<&str>> = HashMap::new();
        for ct in &all_types {
            let cat_name = ct.category.map(|c| c.name).unwrap_or("uncategorized");
            types_by_category
                .entry(cat_name)
                .or_default()
                .insert(&ct.name);
        }

        // Generate traits module
        if self.config.generate_traits {
            let traits_content = generate_traits_module();
            let traits_path = output_dir.join("traits.rs");
            fs::write(&traits_path, &traits_content)?;
            result.files_written.push(traits_path.display().to_string());
        }

        // Generate common enums module
        self.generate_common_module(output_dir, defs, &groups, &mut result)?;

        // Generate category modules (first pass - generate the .rs files and track modules)
        for category in CATEGORIES {
            if category.name != "common_enums" {
                self.generate_category_module(
                    output_dir,
                    category,
                    defs,
                    &groups,
                    &mut result,
                    &mut module_tracker,
                    &types_by_category,
                )?;
            }
        }

        // Generate uncategorized module (catch-all)
        self.generate_uncategorized_module(
            output_dir,
            defs,
            &groups,
            &mut result,
            &types_by_category,
        )?;

        // Generate aggregated mod.rs files for each directory
        self.generate_aggregated_mod_files(output_dir, &module_tracker, &mut result)?;

        // Generate trait implementations
        if self.config.generate_traits {
            self.generate_trait_impls(output_dir, defs, &struct_names, &mut result)?;
        }

        // Generate prelude.rs (feature-gated re-exports)
        if self.config.generate_prelude_rs {
            self.generate_prelude_rs(output_dir, &mut result)?;
        }

        // Generate lib.rs with feature gates
        if self.config.generate_lib_rs {
            self.generate_lib_rs(output_dir, &groups, &mut result)?;
        }

        // Generate monolithic file only if explicitly enabled (modular-only by default)
        if self.config.generate_monolithic {
            self.generate_monolithic(output_dir, defs, &mut result)?;
        }

        Ok(result)
    }

    /// Generate the common enums module
    fn generate_common_module(
        &self,
        output_dir: &Path,
        defs: &TypeDefinitions,
        groups: &HashMap<&'static str, Vec<&CategorizedType>>,
        result: &mut GenerationResult,
    ) -> anyhow::Result<()> {
        let common_dir = output_dir.join("common");
        fs::create_dir_all(&common_dir)?;

        let mut content = String::new();
        content.push_str(&self.file_header());
        content.push_str("//! Common enums used across Salesforce metadata types.\n\n");
        content.push_str("use serde::{Deserialize, Serialize};\n\n");

        // Find the common_enums category and sort them for deterministic output
        if let Some(common_types) = groups.get("common_enums") {
            let mut enum_list: Vec<_> = common_types.iter().filter(|ct| ct.is_enum).collect();
            enum_list.sort_by(|a, b| a.name.cmp(&b.name));

            for ct in enum_list {
                if let Some(variants) = defs.union_types.get(&ct.name) {
                    content.push_str(&generate_enum(&ct.name, variants));
                    content.push('\n');
                    result.types_generated += 1;
                }
            }
        }

        let enums_path = common_dir.join("enums.rs");
        fs::write(&enums_path, &content)?;
        result.files_written.push(enums_path.display().to_string());

        // Generate mod.rs for common
        let mod_content = format!(
            "{}//! Common types used across Salesforce metadata.\n\n\
             pub mod enums;\n\
             pub use enums::*;\n",
            self.file_header()
        );
        let mod_path = common_dir.join("mod.rs");
        fs::write(&mod_path, mod_content)?;
        result.files_written.push(mod_path.display().to_string());

        Ok(())
    }

    /// Generate a category module
    #[allow(clippy::too_many_arguments)]
    fn generate_category_module(
        &self,
        output_dir: &Path,
        category: &TypeCategory,
        defs: &TypeDefinitions,
        groups: &HashMap<&'static str, Vec<&CategorizedType>>,
        result: &mut GenerationResult,
        module_tracker: &mut ModuleTracker,
        types_by_category: &HashMap<&str, HashSet<&str>>,
    ) -> anyhow::Result<()> {
        let types = match groups.get(category.name) {
            Some(t) if !t.is_empty() => t,
            _ => return Ok(()), // No types in this category
        };

        // Determine the module directory structure
        let module_parts: Vec<&str> = category.module_path.split('/').collect();
        let (module_dir, file_name) = if module_parts.len() > 1 {
            let dir = output_dir.join(module_parts[0]);
            let file = module_parts[1].replace(".rs", "");

            // Avoid using "mod" as a file name
            let mut file = if file == "mod" {
                category.name.to_string()
            } else {
                file
            };

            // Special case: avoid module inception for `settings/mod.rs` by ensuring the
            // child module name is not identical to the parent module name.
            //
            // With `categories.rs` using `settings/org_settings.rs`, this normally won't trigger,
            // but keeping the guard makes the generator robust if config regresses.
            if category.name == "settings" && file == "settings" {
                file = "org_settings".to_string();
            }

            (dir, file)
        } else {
            (output_dir.to_path_buf(), category.name.to_string())
        };

        // Skip if file_name is "mod" (reserved)
        if file_name == "mod" {
            return Ok(());
        }

        fs::create_dir_all(&module_dir)?;

        let mut content = String::new();
        content.push_str(&self.file_header());
        content.push_str(&format!("//! {}\n\n", category.description));

        // Add imports
        content.push_str("use serde::{Deserialize, Serialize};\n\n");

        // Get the set of types in this category for reference resolution
        let local_types: HashSet<&str> = types_by_category
            .get(category.name)
            .cloned()
            .unwrap_or_default();

        // Generate enums (sorted by name for deterministic output)
        let mut enum_types: Vec<_> = types.iter().filter(|t| t.is_enum).collect();
        enum_types.sort_by(|a, b| a.name.cmp(&b.name));
        for ct in enum_types {
            if let Some(variants) = defs.union_types.get(&ct.name) {
                content.push_str(&generate_enum(&ct.name, variants));
                content.push('\n');
                result.types_generated += 1;
            }
        }

        // Generate structs (sorted by name for deterministic output)
        let mut struct_types: Vec<_> = types.iter().filter(|t| !t.is_enum).collect();
        struct_types.sort_by(|a, b| a.name.cmp(&b.name));
        for ct in struct_types {
            if let Some(fields) = defs.interface_types.get(&ct.name) {
                content.push_str(&generate_struct_for_module(
                    &ct.name,
                    fields,
                    defs,
                    &local_types,
                ));
                content.push('\n');
                result.types_generated += 1;
            }
        }

        let file_path = module_dir.join(format!("{}.rs", file_name));
        fs::write(&file_path, &content)?;
        result.files_written.push(file_path.display().to_string());

        // Track this submodule for aggregation
        //
        // IMPORTANT: `file_name` is the *final* module name after any special-casing
        // (e.g. `settings` -> `org_settings`). The `module_tracker` must use that final
        // module name so the generated `settings/mod.rs` references the file we actually wrote.
        let dir_key = module_dir.display().to_string();
        module_tracker.add_submodule(&dir_key, &file_name, category.description, category.feature);

        Ok(())
    }

    /// Generate aggregated mod.rs files for directories with multiple submodules
    fn generate_aggregated_mod_files(
        &self,
        _output_dir: &Path,
        module_tracker: &ModuleTracker,
        result: &mut GenerationResult,
    ) -> anyhow::Result<()> {
        for (dir_path, submodules) in &module_tracker.submodules {
            let dir = Path::new(dir_path);

            // Skip if no submodules
            if submodules.is_empty() {
                continue;
            }

            let mut content = String::new();
            content.push_str(&self.file_header());

            // Determine the directory name for the description
            let dir_name = dir.file_name().and_then(|n| n.to_str()).unwrap_or("types");

            content.push_str(&format!("//! {} types.\n\n", capitalize_first(dir_name)));

            // Sort submodules for consistent output
            let mut sorted_submodules = submodules.clone();
            sorted_submodules.sort_by(|a, b| a.0.cmp(&b.0));

            // Deduplicate by module name
            let mut seen_modules = HashSet::new();
            let unique_submodules: Vec<_> = sorted_submodules
                .into_iter()
                .filter(|(name, _, _)| seen_modules.insert(name.clone()))
                .collect();

            // Generate module declarations
            for (module_name, description, _feature) in &unique_submodules {
                content.push_str(&format!("/// {}\n", description));
                content.push_str(&format!("pub mod {};\n", module_name));
            }

            content.push('\n');

            // Generate re-exports
            for (module_name, _, _) in &unique_submodules {
                content.push_str(&format!("pub use {}::*;\n", module_name));
            }

            let mod_path = dir.join("mod.rs");
            fs::write(&mod_path, &content)?;
            result.files_written.push(mod_path.display().to_string());
        }

        Ok(())
    }

    /// Generate the uncategorized types module
    fn generate_uncategorized_module(
        &self,
        output_dir: &Path,
        defs: &TypeDefinitions,
        groups: &HashMap<&'static str, Vec<&CategorizedType>>,
        result: &mut GenerationResult,
        types_by_category: &HashMap<&str, HashSet<&str>>,
    ) -> anyhow::Result<()> {
        let uncategorized = match groups.get("uncategorized") {
            Some(t) if !t.is_empty() => t,
            _ => return Ok(()),
        };

        let mut content = String::new();
        content.push_str(&self.file_header());
        content.push_str("//! Uncategorized Salesforce metadata types.\n");
        content.push_str("//!\n");
        content.push_str("//! These types haven't been assigned to a specific category yet.\n\n");
        content.push_str("use serde::{Deserialize, Serialize};\n\n");

        // Get the set of types in uncategorized for reference resolution
        let local_types: HashSet<&str> = types_by_category
            .get("uncategorized")
            .cloned()
            .unwrap_or_default();

        // Generate enums
        for ct in uncategorized.iter().filter(|t| t.is_enum) {
            if let Some(variants) = defs.union_types.get(&ct.name) {
                content.push_str(&generate_enum(&ct.name, variants));
                content.push('\n');
                result.types_generated += 1;
            }
        }

        // Generate structs
        for ct in uncategorized.iter().filter(|t| !t.is_enum) {
            if let Some(fields) = defs.interface_types.get(&ct.name) {
                content.push_str(&generate_struct_for_module(
                    &ct.name,
                    fields,
                    defs,
                    &local_types,
                ));
                content.push('\n');
                result.types_generated += 1;
            }
        }

        let path = output_dir.join("uncategorized.rs");
        fs::write(&path, &content)?;
        result.files_written.push(path.display().to_string());

        Ok(())
    }

    /// Generate trait implementations module
    fn generate_trait_impls(
        &self,
        output_dir: &Path,
        defs: &TypeDefinitions,
        struct_names: &[String],
        result: &mut GenerationResult,
    ) -> anyhow::Result<()> {
        // Build a map of type name -> field names for checking full_name
        let fields_by_type: HashMap<String, Vec<String>> = defs
            .interface_types
            .iter()
            .map(|(name, fields)| {
                (
                    name.clone(),
                    fields.iter().map(|f| f.name.clone()).collect(),
                )
            })
            .collect();

        // Filter out structs categorized into common_enums: that category only generates
        // enums, so any struct placed there is never emitted as a Rust type.  Generating a
        // trait impl for a non-existent type causes a compilation error.
        let generated_structs: Vec<String> = struct_names
            .iter()
            .filter(|name| find_category(name).map(|c| c.name) != Some("common_enums"))
            .cloned()
            .collect();

        // Build a map of type name -> feature flag name so we can cfg-gate each impl
        // to match the module feature gates (keeps builds slim).
        let mut feature_by_type: HashMap<String, String> = HashMap::new();
        for name in &generated_structs {
            if let Some(cat) = find_category(name) {
                feature_by_type.insert(name.clone(), cat.feature.to_string());
            } else {
                // Uncategorized types are only available with `full`
                feature_by_type.insert(name.clone(), "full".to_string());
            }
        }

        let impl_code = generate_all_trait_impls(
            &generated_structs,
            &fields_by_type,
            &self.config.trait_config,
            Some(&feature_by_type),
        );

        let path = output_dir.join("trait_impls.rs");
        let content = format!(
            "{}//! Auto-generated trait implementations for Salesforce types.\n//!\n//! NOTE: This module is compiled when *any* domain feature is enabled, and each impl is\n//! feature-gated so builds remain slim.\n\n#![allow(unused_imports)]\n\nuse crate::prelude::*;\n\n{}\n",
            self.file_header(),
            impl_code
        );
        fs::write(&path, &content)?;
        result.files_written.push(path.display().to_string());

        Ok(())
    }

    /// Generate lib.rs with feature-gated re-exports
    fn generate_lib_rs(
        &self,
        output_dir: &Path,
        groups: &HashMap<&'static str, Vec<&CategorizedType>>,
        result: &mut GenerationResult,
    ) -> anyhow::Result<()> {
        let mut content = String::new();

        content.push_str(
            r#"//! Auto-generated Salesforce metadata types from @salesforce/types.
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

"#,
        );

        // Common module (always included)
        content.push_str("// Common types (always available)\n");
        content.push_str("pub mod common;\n");
        content.push_str("pub use common::*;\n\n");

        // Traits module
        content.push_str("// Core traits for type-safe API usage\n");
        content.push_str("pub mod traits;\n\n");

        // Feature-gated modules
        content.push_str("// Feature-gated modules\n\n");

        // Build a map of module -> set of features that enable it
        let mut module_features: BTreeMap<&str, HashSet<&str>> = BTreeMap::new();
        for category in CATEGORIES {
            if category.feature != "common" {
                let module_name = category
                    .module_path
                    .split('/')
                    .next()
                    .unwrap_or(category.name);
                // Skip if module name equals "mod"
                if module_name != "mod" {
                    module_features
                        .entry(module_name)
                        .or_default()
                        .insert(category.feature);
                }
            }
        }

        // Generate feature-gated module declarations with any() for multiple features
        for (module, features) in &module_features {
            let mut features_vec: Vec<&str> = features.iter().copied().collect();
            features_vec.sort(); // Sort for consistent output

            if features_vec.len() == 1 {
                // Single feature
                content.push_str(&format!("#[cfg(feature = \"{}\")]\n", features_vec[0]));
            } else {
                // Multiple features - use any()
                let feature_list: Vec<String> = features_vec
                    .iter()
                    .map(|f| format!("feature = \"{}\"", f))
                    .collect();
                content.push_str(&format!("#[cfg(any({}))]\n", feature_list.join(", ")));
            }
            content.push_str(&format!("pub mod {};\n", module));

            if features_vec.len() == 1 {
                content.push_str(&format!("#[cfg(feature = \"{}\")]\n", features_vec[0]));
            } else {
                let feature_list: Vec<String> = features_vec
                    .iter()
                    .map(|f| format!("feature = \"{}\"", f))
                    .collect();
                content.push_str(&format!("#[cfg(any({}))]\n", feature_list.join(", ")));
            }
            content.push_str(&format!("pub use {}::*;\n\n", module));
        }

        // Uncategorized module (always available with full feature)
        if groups
            .get("uncategorized")
            .map(|v| !v.is_empty())
            .unwrap_or(false)
        {
            content.push_str("// Uncategorized types (available with full feature)\n");
            content.push_str("#[cfg(feature = \"full\")]\n");
            content.push_str("pub mod uncategorized;\n");
            content.push_str("#[cfg(feature = \"full\")]\n");
            content.push_str("pub use uncategorized::*;\n\n");
        }

        // Prelude module (feature-gated re-exports used by trait impls and consumers)
        content.push_str("// Prelude: feature-gated re-exports of enabled types\n");
        content.push_str("pub mod prelude;\n\n");

        // Trait implementations: compile when any type-bearing module feature is enabled.
        // Each impl is feature-gated, so consumers only compile impls for enabled types.
        content.push_str("// Trait implementations (feature-gated per impl)\n");
        content.push_str("#[cfg(any(\n");
        for feat in DOMAIN_FEATURES {
            content.push_str(&format!("    feature = \"{}\",\n", feat));
        }
        // Uncategorized types are only available with `full`, so allow trait impls under `full` as well.
        content.push_str("    feature = \"full\"\n");
        content.push_str("))]\n");
        content.push_str("mod trait_impls;\n");

        let path = output_dir.join("lib.rs");
        fs::write(&path, &content)?;
        result.files_written.push(path.display().to_string());

        Ok(())
    }

    ////// Generate monolithic file for backward compatibility
    ///
    /// NOTE: The project now defaults to modular-only output. This function remains
    /// available for explicitly-enabled legacy workflows, but should not be used by
    /// the generated `lib.rs`.
    /// Generate monolithic file for backward compatibility
    ///
    /// NOTE: The project now defaults to modular-only output. This function remains
    /// available for explicitly-enabled legacy workflows, but should not be used by
    /// the generated `lib.rs`.
    fn generate_monolithic(
        &self,
        output_dir: &Path,
        defs: &TypeDefinitions,
        result: &mut GenerationResult,
    ) -> anyhow::Result<()> {
        let mut content = String::new();
        content.push_str(&self.file_header());
        content.push_str("use serde::{Deserialize, Serialize};\n\n");

        // Priority types first
        let priority_enums = [
            "FieldType",
            "SharingModel",
            "DeploymentStatus",
            "DeployStatus",
            "Gender",
        ];
        let priority_structs = [
            "CustomObject",
            "CustomField",
            "Layout",
            "PermissionSet",
            "ValidationRule",
            "WorkflowRule",
        ];

        let mut generated: HashSet<String> = HashSet::new();

        // Generate priority enums
        for name in priority_enums {
            if let Some(variants) = defs.union_types.get(name) {
                content.push_str(&generate_enum(name, variants));
                content.push('\n');
                generated.insert(name.to_string());
            }
        }

        // Generate remaining enums (sorted for deterministic output)
        let mut remaining_enums: Vec<_> = defs
            .union_types
            .iter()
            .filter(|(name, _)| !generated.contains(*name))
            .collect();
        remaining_enums.sort_by(|a, b| a.0.cmp(b.0));

        for (name, variants) in remaining_enums {
            content.push_str(&generate_enum(name, variants));
            content.push('\n');
            generated.insert(name.clone());
        }

        // Generate priority structs
        for name in priority_structs {
            if let Some(fields) = defs.interface_types.get(name) {
                content.push_str(&generate_struct(name, fields, defs));
                content.push('\n');
                generated.insert(name.to_string());
            }
        }

        // Generate remaining structs (sorted for deterministic output)
        let mut remaining_structs: Vec<_> = defs
            .interface_types
            .iter()
            .filter(|(name, _)| !generated.contains(*name))
            .collect();
        remaining_structs.sort_by(|a, b| a.0.cmp(b.0));

        for (name, fields) in remaining_structs {
            content.push_str(&generate_struct(name, fields, defs));
            content.push('\n');
        }

        let path = output_dir.join("generated_salesforce_types.rs");
        fs::write(&path, &content)?;
        result.files_written.push(path.display().to_string());

        Ok(())
    }

    /// Generate a feature-gated prelude module (`prelude.rs`) that re-exports all enabled types.
    ///
    /// This module is used by `trait_impls.rs` so it can refer to type names without
    /// depending on a monolithic output file.
    fn generate_prelude_rs(
        &self,
        output_dir: &Path,
        result: &mut GenerationResult,
    ) -> anyhow::Result<()> {
        // Build a map from feature name -> top-level module (first path segment of module_path).
        // Categories whose module_path starts with "metadata/" live under `crate::metadata::<child>`,
        // while others (e.g. "settings/…", "packaging/…") are top-level: `crate::<module>`.
        let feature_to_top_module: HashMap<&str, &str> = CATEGORIES
            .iter()
            .map(|c| {
                let top = c.module_path.split('/').next().unwrap_or(c.name);
                (c.feature, top)
            })
            .collect();

        let mut content = String::new();
        content.push_str(&self.file_header());
        content.push_str("//! Prelude: feature-gated re-exports of enabled Salesforce types.\n");
        content.push_str("//!\n");
        content.push_str("//! This module is generated to provide a consistent import surface:\n");
        content.push_str("//! `use sf_generated_types::prelude::*;`\n");
        content.push_str("//!\n");
        content.push_str(
            "//! Each re-export is behind the same feature gate as the module that defines it,\n",
        );
        content.push_str("//! so enabling a slim set of features results in a slim compile.\n\n");

        // Common is always available
        content.push_str("pub use crate::common::*;\n\n");

        // Re-export each domain feature behind its feature gate.
        // The crate path depends on whether the category lives under `metadata/` or at the top level.
        for feat in DOMAIN_FEATURES {
            let top_module = feature_to_top_module
                .get(feat)
                .copied()
                .unwrap_or("metadata");

            content.push_str(&format!("#[cfg(feature = \"{}\")]\n", feat));
            if top_module == "metadata" {
                content.push_str(&format!("pub use crate::metadata::{}::*;\n\n", feat));
            } else {
                // Top-level module (e.g. settings, packaging)
                content.push_str(&format!("pub use crate::{}::*;\n\n", top_module));
            }
        }

        // Uncategorized types are only available under `full`
        content.push_str("#[cfg(feature = \"full\")]\n");
        content.push_str("pub use crate::uncategorized::*;\n");

        let path = output_dir.join("prelude.rs");
        fs::write(&path, &content)?;
        result.files_written.push(path.display().to_string());

        Ok(())
    }

    /// Generate the file header
    fn file_header(&self) -> String {
        r#"#![allow(non_camel_case_types)]
//! Auto-generated Salesforce metadata types from @salesforce/types
//! Source: <https://github.com/forcedotcom/wsdl>
//!
//! DO NOT EDIT - This file is automatically generated

"#
        .to_string()
    }
}

/// Capitalize the first letter of a string
fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// Result of code generation
#[derive(Debug, Default)]
pub struct GenerationResult {
    /// List of files written
    pub files_written: Vec<String>,
    /// Number of types generated
    pub types_generated: usize,
    /// Any warnings during generation
    pub warnings: Vec<String>,
}

/// Generate a Rust enum from TypeScript union type
pub fn generate_enum(name: &str, variants: &[String]) -> String {
    let mut output = format!(
        r#"#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum {} {{
    #[default]
"#,
        name
    );

    for variant in variants {
        // Clean variant name
        let mut rust_variant = variant
            .replace(['-', ' ', '.', '/', ':'], "_")
            .replace(['(', ')'], "");

        if rust_variant
            .chars()
            .next()
            .map(|c| c.is_numeric())
            .unwrap_or(true)
        {
            rust_variant = format!("_{}", rust_variant);
        }

        // Handle special cases
        if rust_variant == "Self" {
            rust_variant = "SelfValue".to_string();
        } else if RESERVED_WORDS.contains(&rust_variant.as_str()) {
            rust_variant = format!("r#{}", rust_variant);
        }

        // Add serde rename if needed
        if rust_variant != *variant {
            output.push_str(&format!("    #[serde(rename = \"{}\")]\n", variant));
        }

        output.push_str(&format!("    {},\n", rust_variant));
    }

    output.push_str("}\n");
    output
}

/// Generate a Rust struct from TypeScript interface (for monolithic file - uses all types)
pub fn generate_struct(name: &str, fields: &[FieldDef], defs: &TypeDefinitions) -> String {
    let description = defs
        .descriptions
        .get(name)
        .map(|d| format!("/// {}\n", d.replace('\n', "\n/// ")))
        .unwrap_or_default();

    let mut output = format!(
        r#"{}#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct {} {{
"#,
        description, name
    );

    // Track seen field names to avoid duplicates
    let mut seen_names: HashSet<String> = HashSet::new();

    for field in fields {
        // Add doc comment if description exists
        if let Some(desc) = &field.description {
            output.push_str(&format!("    /// {}\n", desc.replace('\n', "\n    /// ")));
        }

        let rust_field_name = to_snake_case(&field.name);

        // Skip duplicates
        if seen_names.contains(&rust_field_name) {
            continue;
        }
        seen_names.insert(rust_field_name.clone());

        // Build serde attributes
        let mut serde_attrs = Vec::new();
        if rust_field_name != field.name {
            serde_attrs.push(format!("rename = \"{}\"", field.name));
        }
        serde_attrs.push("default".to_string());
        if field.optional {
            serde_attrs.push("skip_serializing_if = \"Option::is_none\"".to_string());
        }

        if !serde_attrs.is_empty() {
            output.push_str(&format!("    #[serde({})]\n", serde_attrs.join(", ")));
        }

        // Resolve the field type - for monolithic, we can reference all types
        // Check for self-reference (field type matches struct name)
        let resolved_type = if field.type_ref == name {
            format!("Box<{}>", name)
        } else {
            resolve_type_monolithic(&field.type_ref, defs, name)
        };
        let field_type = if field.is_array {
            format!("Vec<{}>", resolved_type)
        } else {
            resolved_type
        };

        let field_type = if field.optional {
            format!("Option<{}>", field_type)
        } else {
            field_type
        };

        output.push_str(&format!("    pub {}: {},\n", rust_field_name, field_type));
    }

    output.push_str("}\n");
    output
}

/// Generate a Rust struct for a specific module (uses serde_json::Value for cross-module refs)
pub fn generate_struct_for_module(
    name: &str,
    fields: &[FieldDef],
    defs: &TypeDefinitions,
    local_types: &HashSet<&str>,
) -> String {
    let description = defs
        .descriptions
        .get(name)
        .map(|d| format!("/// {}\n", d.replace('\n', "\n/// ")))
        .unwrap_or_default();

    let mut output = format!(
        r#"{}#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct {} {{
"#,
        description, name
    );

    // Track seen field names to avoid duplicates
    let mut seen_names: HashSet<String> = HashSet::new();

    for field in fields {
        // Add doc comment if description exists
        if let Some(desc) = &field.description {
            output.push_str(&format!("    /// {}\n", desc.replace('\n', "\n    /// ")));
        }

        let rust_field_name = to_snake_case(&field.name);

        // Skip duplicates
        if seen_names.contains(&rust_field_name) {
            continue;
        }
        seen_names.insert(rust_field_name.clone());

        // Build serde attributes
        let mut serde_attrs = Vec::new();
        if rust_field_name != field.name {
            serde_attrs.push(format!("rename = \"{}\"", field.name));
        }
        serde_attrs.push("default".to_string());
        if field.optional {
            serde_attrs.push("skip_serializing_if = \"Option::is_none\"".to_string());
        }

        if !serde_attrs.is_empty() {
            output.push_str(&format!("    #[serde({})]\n", serde_attrs.join(", ")));
        }

        // Resolve the field type - use serde_json::Value for cross-module refs
        // Check for self-reference (field type matches struct name)
        let resolved_type = if field.type_ref == name {
            format!("Box<{}>", name)
        } else {
            resolve_type_for_module(&field.type_ref, defs, local_types, name)
        };
        let field_type = if field.is_array {
            format!("Vec<{}>", resolved_type)
        } else {
            resolved_type
        };

        let field_type = if field.optional {
            format!("Option<{}>", field_type)
        } else {
            field_type
        };

        output.push_str(&format!("    pub {}: {},\n", rust_field_name, field_type));
    }

    output.push_str("}\n");
    output
}

/// Convert a name to snake_case, handling Rust keywords
fn to_snake_case(name: &str) -> String {
    let snake = name.to_case(Case::Snake);
    if RESERVED_WORDS.contains(&snake.as_str()) {
        format!("r#{}", snake)
    } else {
        snake
    }
}

/// Resolve a TypeScript type reference to a Rust type (for monolithic file)
fn resolve_type_monolithic(type_ref: &str, defs: &TypeDefinitions, current_struct: &str) -> String {
    match type_ref {
        "String" | "string" => "String".to_string(),
        "bool" | "boolean" => "bool".to_string(),
        "f64" | "number" => "f64".to_string(),
        "i32" | "i64" => type_ref.to_string(),
        _ => {
            // Check if it's a known enum or struct
            if defs.union_types.contains_key(type_ref)
                || defs.interface_types.contains_key(type_ref)
            {
                // Use Box for self-referential or known recursive types to avoid infinite size
                if type_ref == current_struct || is_potentially_recursive(type_ref) {
                    format!("Box<{}>", type_ref)
                } else {
                    type_ref.to_string()
                }
            } else {
                // Fall back to serde_json::Value for unknown types
                "serde_json::Value".to_string()
            }
        }
    }
}

/// Resolve a TypeScript type reference to a Rust type (for modular files)
/// Uses serde_json::Value for types not in the local module
fn resolve_type_for_module(
    type_ref: &str,
    defs: &TypeDefinitions,
    local_types: &HashSet<&str>,
    current_struct: &str,
) -> String {
    match type_ref {
        "String" | "string" => "String".to_string(),
        "bool" | "boolean" => "bool".to_string(),
        "f64" | "number" => "f64".to_string(),
        "i32" | "i64" => type_ref.to_string(),
        _ => {
            // Check if it's a local type (in this module)
            if local_types.contains(type_ref) {
                // Use Box for self-referential or known recursive types
                if type_ref == current_struct || is_potentially_recursive(type_ref) {
                    format!("Box<{}>", type_ref)
                } else {
                    type_ref.to_string()
                }
            } else if defs.union_types.contains_key(type_ref)
                || defs.interface_types.contains_key(type_ref)
            {
                // Type exists but is in another module - use serde_json::Value
                "serde_json::Value".to_string()
            } else {
                // Unknown type
                "serde_json::Value".to_string()
            }
        }
    }
}

/// Check if a type might be recursive (simplified heuristic)
fn is_potentially_recursive(type_name: &str) -> bool {
    // These types are known to have recursive structures in Salesforce metadata
    matches!(
        type_name,
        "FlowElement"
            | "FlowNode"
            | "FlowConnector"
            | "LayoutSection"
            | "LayoutColumn"
            | "ObjectRelationship"
            | "AIPredictionField"
            | "MLFilter"
            | "MLField"
            | "MLModelElement"
            | "RecordAction"
            | "RecordActionDeployment"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_enum() {
        let enum_code = generate_enum("FieldType", &["Text".to_string(), "Number".to_string()]);
        assert!(enum_code.contains("pub enum FieldType"));
        assert!(enum_code.contains("Text"));
        assert!(enum_code.contains("Number"));
        assert!(enum_code.contains("#[default]"));
    }

    #[test]
    fn test_generate_struct() {
        let fields = vec![
            FieldDef {
                name: "fullName".to_string(),
                type_ref: "String".to_string(),
                optional: true,
                is_array: false,
                description: None,
            },
            FieldDef {
                name: "fields".to_string(),
                type_ref: "CustomField".to_string(),
                optional: true,
                is_array: true,
                description: None,
            },
        ];

        let defs = TypeDefinitions::default();
        let struct_code = generate_struct("CustomObject", &fields, &defs);

        assert!(struct_code.contains("pub struct CustomObject"));
        assert!(struct_code.contains("full_name"));
        assert!(struct_code.contains("Option<String>"));
        assert!(struct_code.contains("Option<Vec<"));
    }

    #[test]
    fn test_generate_struct_for_module() {
        let fields = vec![
            FieldDef {
                name: "fullName".to_string(),
                type_ref: "String".to_string(),
                optional: true,
                is_array: false,
                description: None,
            },
            FieldDef {
                name: "otherType".to_string(),
                type_ref: "SomeOtherType".to_string(),
                optional: true,
                is_array: false,
                description: None,
            },
        ];

        let mut defs = TypeDefinitions::default();
        defs.interface_types
            .insert("SomeOtherType".to_string(), vec![]);

        let mut local_types = HashSet::new();
        local_types.insert("CustomObject");
        // SomeOtherType is NOT in local_types

        let struct_code = generate_struct_for_module("CustomObject", &fields, &defs, &local_types);

        assert!(struct_code.contains("pub struct CustomObject"));
        // SomeOtherType should be serde_json::Value since it's not local
        assert!(struct_code.contains("serde_json::Value"));
    }

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("fullName"), "full_name");
        assert_eq!(to_snake_case("type"), "r#type");
        assert_eq!(to_snake_case("CustomObject"), "custom_object");
    }

    #[test]
    fn test_resolve_type_monolithic() {
        let mut defs = TypeDefinitions::default();
        defs.union_types.insert("FieldType".to_string(), vec![]);

        assert_eq!(
            resolve_type_monolithic("String", &defs, "MyStruct"),
            "String"
        );
        assert_eq!(resolve_type_monolithic("bool", &defs, "MyStruct"), "bool");
        assert_eq!(
            resolve_type_monolithic("FieldType", &defs, "MyStruct"),
            "FieldType"
        );
        assert_eq!(
            resolve_type_monolithic("UnknownType", &defs, "MyStruct"),
            "serde_json::Value"
        );
        // Self-reference should use Box
        assert_eq!(
            resolve_type_monolithic("MyStruct", &defs, "MyStruct"),
            "serde_json::Value" // MyStruct not in defs, so falls through
        );
    }

    #[test]
    fn test_resolve_type_for_module() {
        let mut defs = TypeDefinitions::default();
        defs.union_types.insert("FieldType".to_string(), vec![]);
        defs.union_types.insert("OtherEnum".to_string(), vec![]);

        let mut local_types = HashSet::new();
        local_types.insert("FieldType");
        // OtherEnum is NOT in local_types

        assert_eq!(
            resolve_type_for_module("String", &defs, &local_types, "MyStruct"),
            "String"
        );
        assert_eq!(
            resolve_type_for_module("FieldType", &defs, &local_types, "MyStruct"),
            "FieldType"
        );
        // OtherEnum exists but is not local, so should be Value
        assert_eq!(
            resolve_type_for_module("OtherEnum", &defs, &local_types, "MyStruct"),
            "serde_json::Value"
        );
    }

    #[test]
    fn test_module_tracker() {
        let mut tracker = ModuleTracker::default();
        tracker.add_submodule("/path/to/metadata", "objects", "Custom objects", "objects");
        tracker.add_submodule("/path/to/metadata", "flows", "Flow definitions", "flows");
        // This should be ignored
        tracker.add_submodule("/path/to/metadata", "mod", "Reserved", "reserved");

        let submodules = tracker.submodules.get("/path/to/metadata").unwrap();
        assert_eq!(submodules.len(), 2);
        assert_eq!(submodules[0].0, "objects");
        assert_eq!(submodules[1].0, "flows");
    }

    #[test]
    fn test_capitalize_first() {
        assert_eq!(capitalize_first("metadata"), "Metadata");
        assert_eq!(capitalize_first("CAPS"), "CAPS");
        assert_eq!(capitalize_first(""), "");
    }
}
