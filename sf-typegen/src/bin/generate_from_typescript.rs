//! Generate Rust types from @salesforce/types TypeScript definitions
//!
//! This parses Salesforce's official metadata.ts file using oxc_parser
//! and generates Rust structs/enums organized into categorized modules
//! with proper feature flags.
//!
//! Usage:
//!   cargo run -p sf-typegen --bin generate_from_typescript
//!
//! Options:
//!   --modular       Generate only the modular files (default)
//!   --output-dir    Output directory (default: ../sf-generated-types/src)
//!
//! Advanced / legacy (not recommended):
//!   --monolithic    Generate only the monolithic file (legacy mode)
//!   --all           Generate both monolithic and modular files

use anyhow::{Context, Result};
use convert_case::{Case, Casing};
use oxc_allocator::Allocator;
use oxc_ast::ast::*;
use oxc_parser::{Parser, ParserReturn};
use oxc_span::SourceType;
use regex::Regex;
use sf_typegen::categories::find_category;
use sf_typegen::modular_generator::{
    FieldDef, ModularGenerator, ModularGeneratorConfig, TypeDefinitions,
};
use std::collections::{BTreeMap, HashMap};
use std::env;
use std::fs;
use std::path::Path;

use serde::Deserialize;

/// Rust reserved keywords that need escaping with r#
const RESERVED_WORDS: &[&str] = &[
    "type", "fn", "let", "const", "mut", "ref", "if", "else", "match", "loop", "while", "for",
    "in", "return", "break", "continue", "struct", "enum", "trait", "impl", "pub", "priv", "use",
    "mod", "crate", "self", "Self", "super", "unsafe", "async", "await", "move", "where", "as",
    "dyn", "abstract", "become", "box", "do", "final", "macro", "override", "try",
];

#[derive(Debug, Clone, Copy, PartialEq)]
enum GenerationMode {
    Monolithic,
    Modular,
    All,
}

fn main() -> Result<()> {
    println!("🔧 Salesforce TypeScript → Rust Type Generator");
    println!("================================================\n");

    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let mode = parse_generation_mode(&args);
    let output_dir = parse_output_dir(&args);
    let input_file = parse_input_file(&args);

    println!("📋 Generation mode: {:?}", mode);
    println!("📂 Output directory: {}", output_dir);

    let typescript_source = if let Some(path) = input_file {
        println!("\n📂 Reading @salesforce/types from local file: {}", path);
        std::fs::read_to_string(&path).context("Failed to read input file")?
    } else {
        println!("\n📥 Fetching @salesforce/types from GitHub...");
        fetch_salesforce_types()?
    };
    println!("📊 Source size: {} bytes", typescript_source.len());

    // Parse TypeScript AST using oxc
    println!("🔍 Parsing TypeScript AST with oxc...");
    let mut type_definitions = parse_and_extract(&typescript_source)?;

    println!(
        "✅ Found {} union types and {} interface types",
        type_definitions.union_types.len(),
        type_definitions.interface_types.len()
    );

    // Fallback: use regex to extract struct fields if oxc didn't get them
    if type_definitions.interface_types.is_empty() {
        println!("📝 Falling back to regex extraction for struct fields...");
        let regex_structs = extract_structs_with_regex(&typescript_source)?;
        for (name, fields) in regex_structs {
            type_definitions.interface_types.insert(name, fields);
        }
        println!(
            "✅ Regex extraction found {} structs",
            type_definitions.interface_types.len()
        );
    }

    // Print categorization statistics
    print_categorization_stats(&type_definitions);

    // Check for missing overlays
    report_missing_overlays(&type_definitions);

    // Generate based on mode
    match mode {
        GenerationMode::Modular | GenerationMode::All => {
            println!("\n🦀 Generating modular Rust code...");
            generate_modular(&type_definitions, &output_dir)?;
        }
        _ => {}
    }

    match mode {
        GenerationMode::Monolithic | GenerationMode::All => {
            println!("\n🦀 Generating monolithic Rust code...");
            generate_monolithic(&type_definitions, &output_dir)?;
        }
        _ => {}
    }

    println!("\n✨ Successfully generated Rust types from @salesforce/types!");
    Ok(())
}

fn parse_generation_mode(args: &[String]) -> GenerationMode {
    for arg in args {
        match arg.as_str() {
            "--monolithic" => return GenerationMode::Monolithic,
            "--modular" => return GenerationMode::Modular,
            "--all" => return GenerationMode::All,
            _ => {}
        }
    }
    GenerationMode::Modular // Default to modular-only output
}

fn parse_output_dir(args: &[String]) -> String {
    let mut iter = args.iter().peekable();
    while let Some(arg) = iter.next() {
        if arg == "--output-dir" {
            if let Some(dir) = iter.next() {
                return dir.clone();
            }
        }
    }
    "../crates/sf-types/src".to_string()
}

fn parse_input_file(args: &[String]) -> Option<String> {
    let mut iter = args.iter().peekable();
    while let Some(arg) = iter.next() {
        if arg == "--input-file" {
            if let Some(file) = iter.next() {
                return Some(file.clone());
            }
        }
    }
    None
}

/// Generate modular output using the new generator
fn generate_modular(defs: &TypeDefinitions, output_dir: &str) -> Result<()> {
    let config = ModularGeneratorConfig {
        output_dir: output_dir.to_string(),
        generate_monolithic: false, // We'll handle this separately
        generate_traits: true,
        generate_lib_rs: true,
        ..Default::default()
    };

    let generator = ModularGenerator::new(config);
    let result = generator.generate(defs)?;

    println!("📝 Generated {} types", result.types_generated);
    println!("📂 Written {} files:", result.files_written.len());
    for file in &result.files_written {
        println!("   - {}", file);
    }

    if !result.warnings.is_empty() {
        println!("⚠️  {} warnings:", result.warnings.len());
        for warning in &result.warnings {
            println!("   - {}", warning);
        }
    }

    Ok(())
}

/// Generate monolithic output (backward compatibility)
fn generate_monolithic(defs: &TypeDefinitions, output_dir: &str) -> Result<()> {
    let rust_code = generate_rust_code(defs)?;

    // Write to sf-generated-types crate
    let output_path = format!("{}/generated_salesforce_types.rs", output_dir);
    std::fs::create_dir_all(output_dir)?;
    std::fs::write(&output_path, &rust_code)?;
    println!("📝 Written monolithic file to {}", output_path);

    // Also write to metadata-etl crate for downstream consumers
    let output_path_etl = "../metadata-etl/src/generated_salesforce_types.rs";
    if std::path::Path::new("../metadata-etl/src").exists() {
        std::fs::write(output_path_etl, &rust_code)?;
        println!("📝 Written to {}", output_path_etl);
    }

    Ok(())
}

/// Print statistics about type categorization
///
/// Also writes a machine-readable report for CI automation:
/// - `missing_categories.json` (JSON array of type names that did not match any category)
fn print_categorization_stats(defs: &TypeDefinitions) {
    let mut categorized_enums = 0;
    let mut categorized_structs = 0;
    let mut category_counts: HashMap<&str, usize> = HashMap::new();

    let mut missing_categories: Vec<String> = Vec::new();

    for name in defs.union_types.keys() {
        if let Some(cat) = find_category(name) {
            categorized_enums += 1;
            *category_counts.entry(cat.name).or_insert(0) += 1;
        } else {
            missing_categories.push(name.clone());
        }
    }

    for name in defs.interface_types.keys() {
        if let Some(cat) = find_category(name) {
            categorized_structs += 1;
            *category_counts.entry(cat.name).or_insert(0) += 1;
        } else {
            missing_categories.push(name.clone());
        }
    }

    let total_enums = defs.union_types.len();
    let total_structs = defs.interface_types.len();
    let uncategorized_enums = total_enums - categorized_enums;
    let uncategorized_structs = total_structs - categorized_structs;

    println!("\n📊 Categorization Statistics:");
    println!(
        "   Enums:   {} categorized, {} uncategorized",
        categorized_enums, uncategorized_enums
    );
    println!(
        "   Structs: {} categorized, {} uncategorized",
        categorized_structs, uncategorized_structs
    );
    println!("\n   By category:");

    let mut sorted_categories: Vec<_> = category_counts.iter().collect();
    sorted_categories.sort_by(|a, b| b.1.cmp(a.1));

    for (cat_name, count) in sorted_categories {
        println!("   - {}: {} types", cat_name, count);
    }

    // Write machine-readable report for CI automation / rollup issues
    if !missing_categories.is_empty() {
        missing_categories.sort();
        missing_categories.dedup();

        println!(
            "\n⚠️  Found {} types without a category assignment.",
            missing_categories.len()
        );

        let report_path = "missing_categories.json";
        if let Ok(json) = serde_json::to_string_pretty(&missing_categories) {
            if let Err(e) = std::fs::write(report_path, json) {
                println!("   Failed to write {}: {}", report_path, e);
            } else {
                println!("   Written missing categories to {}", report_path);
            }
        }
    } else {
        println!("\n✅ All types have category assignments!");
    }
}

/// Fetch the latest metadata.ts from Salesforce's GitHub
fn fetch_salesforce_types() -> Result<String> {
    let url = "https://raw.githubusercontent.com/forcedotcom/wsdl/main/src/metadata.ts";
    let response = reqwest::blocking::get(url)
        .context("Failed to fetch from GitHub")?
        .text()
        .context("Failed to read response")?;
    Ok(response)
}

/// Parse TypeScript and extract type definitions using oxc
fn parse_and_extract(source: &str) -> Result<TypeDefinitions> {
    let allocator = Allocator::default();
    let source_type = SourceType::from_path("metadata.ts").unwrap();

    let ParserReturn {
        program, errors, ..
    } = Parser::new(&allocator, source, source_type).parse();

    if !errors.is_empty() {
        println!("⚠️  Parser warnings: {} (continuing anyway)", errors.len());
    }

    let mut union_types = HashMap::new();
    let mut interface_types = HashMap::new();
    let mut descriptions = HashMap::new();

    // Walk through all statements in the program
    for stmt in &program.body {
        if let Statement::ExportNamedDeclaration(export) = stmt {
            if let Some(Declaration::TSTypeAliasDeclaration(type_alias)) = &export.declaration {
                let type_name = type_alias.id.name.to_string();

                // Check if it's a single string literal type (e.g., export type ElementType = 'Float')
                if let TSType::TSLiteralType(lit) = &type_alias.type_annotation {
                    if let TSLiteral::StringLiteral(s) = &lit.literal {
                        union_types.insert(type_name.clone(), vec![s.value.to_string()]);
                    }
                }

                // Check if it's a union type (enum) - ONLY string literal unions
                if let TSType::TSUnionType(union) = &type_alias.type_annotation {
                    let mut variants = Vec::new();
                    let mut is_string_union = true;

                    for t in &union.types {
                        if let TSType::TSLiteralType(lit) = t {
                            if let TSLiteral::StringLiteral(s) = &lit.literal {
                                variants.push(s.value.to_string());
                            } else {
                                is_string_union = false;
                                break;
                            }
                        } else {
                            is_string_union = false;
                            break;
                        }
                    }

                    if is_string_union && !variants.is_empty() {
                        union_types.insert(type_name.clone(), variants);
                    }
                }

                // Check if it's an intersection type (struct extending Metadata)
                if let TSType::TSIntersectionType(intersection) = &type_alias.type_annotation {
                    for t in &intersection.types {
                        if let TSType::TSTypeLiteral(type_lit) = t {
                            let fields = extract_fields_from_type_lit(type_lit);
                            if !fields.is_empty() {
                                interface_types.insert(type_name.clone(), fields);
                            }
                        }
                    }
                }

                // Check for plain type literals (structs not extending Metadata)
                if let TSType::TSTypeLiteral(type_lit) = &type_alias.type_annotation {
                    let fields = extract_fields_from_type_lit(type_lit);
                    if !fields.is_empty() {
                        interface_types.insert(type_name.clone(), fields);
                    }
                }
            }
        }
    }

    println!("   - Union types (enums): {}", union_types.len());
    println!("   - Interface types (structs): {}", interface_types.len());

    // Show some important types we found
    if union_types.contains_key("FieldType") {
        println!("   ✓ Found FieldType enum");
    }
    if interface_types.contains_key("CustomObject") {
        let cf = &interface_types["CustomObject"];
        println!("   ✓ Found CustomObject struct ({} fields)", cf.len());
    }
    if interface_types.contains_key("CustomField") {
        let cf = &interface_types["CustomField"];
        println!("   ✓ Found CustomField struct ({} fields)", cf.len());
    }
    if interface_types.contains_key("Layout") {
        let cf = &interface_types["Layout"];
        println!("   ✓ Found Layout struct ({} fields)", cf.len());
    }

    // Apply manual documentation overlays
    apply_overlays(&mut interface_types, &mut descriptions);

    Ok(TypeDefinitions {
        union_types,
        interface_types,
        descriptions,
    })
}

/// Extract fields from a TypeScript type literal
fn extract_fields_from_type_lit(type_lit: &TSTypeLiteral) -> Vec<FieldDef> {
    let mut fields = Vec::new();

    for member in &type_lit.members {
        if let TSSignature::TSPropertySignature(prop) = member {
            extract_property_signature(prop, &mut fields);
        }
    }

    fields
}

/// Extract a single property signature into a field definition
fn extract_property_signature(prop: &TSPropertySignature, fields: &mut Vec<FieldDef>) {
    if let PropertyKey::Identifier(ident) = &prop.key {
        let field_name = ident.name.to_string();
        let optional = prop.optional;

        let (type_ref, is_array) = if let Some(type_ann) = &prop.type_annotation {
            extract_type_info(&type_ann.type_annotation)
        } else {
            ("String".to_string(), false)
        };

        fields.push(FieldDef {
            name: field_name,
            type_ref,
            optional,
            is_array,
            description: None,
        });
    }
}

/// Extract type information from a TypeScript type
fn extract_type_info(ts_type: &TSType) -> (String, bool) {
    match ts_type {
        TSType::TSStringKeyword(_) => ("String".to_string(), false),
        TSType::TSBooleanKeyword(_) => ("bool".to_string(), false),
        TSType::TSNumberKeyword(_) => ("f64".to_string(), false),
        TSType::TSArrayType(array) => {
            let (inner_type, _) = extract_type_info(&array.element_type);
            (inner_type, true)
        }
        TSType::TSTypeReference(type_ref) => {
            if let TSTypeName::IdentifierReference(ident) = &type_ref.type_name {
                (ident.name.to_string(), false)
            } else {
                ("serde_json::Value".to_string(), false)
            }
        }
        _ => ("serde_json::Value".to_string(), false),
    }
}

/// Convert TypeScript primitive types to Rust equivalents
fn convert_typescript_type_to_rust(ts_type: &str) -> String {
    match ts_type.trim().to_lowercase().as_str() {
        "string" => "String".to_string(),
        "boolean" => "bool".to_string(),
        "number" => "f64".to_string(),
        "date" => "String".to_string(),
        "any" | "object" => "serde_json::Value".to_string(),
        _ => ts_type.trim().to_string(),
    }
}

/// Fallback: extract structs using regex when oxc AST extraction fails
fn extract_structs_with_regex(source: &str) -> Result<HashMap<String, Vec<FieldDef>>> {
    let mut structs = HashMap::new();

    let type_pattern =
        Regex::new(r"(?s)export type (\w+)\s*=\s*(?:Metadata\s*&\s*)?\s*\{([^}]+)\}")?;
    let field_pattern = Regex::new(r"(\w+)\??\s*:\s*([^;,}]+)")?;

    for cap in type_pattern.captures_iter(source) {
        let type_name = cap.get(1).unwrap().as_str().to_string();
        let type_body = cap.get(2).unwrap().as_str();

        let mut fields = Vec::new();

        for field_cap in field_pattern.captures_iter(type_body) {
            let mut field_name = field_cap.get(1).unwrap().as_str().to_string();
            let type_str = field_cap.get(2).unwrap().as_str().trim();

            if RESERVED_WORDS.contains(&field_name.as_str()) {
                field_name = format!("r#{}", field_name);
            }

            let optional = type_body[..field_cap.get(0).unwrap().start()].ends_with('?');
            let is_array = type_str.contains("[]");
            let raw_type = type_str
                .replace("[]", "")
                .split('[')
                .next()
                .unwrap_or("String")
                .trim()
                .to_string();

            let type_ref = convert_typescript_type_to_rust(&raw_type);

            fields.push(FieldDef {
                name: field_name,
                type_ref,
                optional,
                is_array,
                description: None,
            });
        }

        if !fields.is_empty() && !type_name.chars().next().unwrap().is_lowercase() {
            structs.insert(type_name, fields);
        }
    }

    Ok(structs)
}

/// Generate Rust code from extracted type definitions (legacy monolithic)
fn generate_rust_code(defs: &TypeDefinitions) -> Result<String> {
    let mut output = String::new();

    output.push_str(
        r#"#![allow(non_camel_case_types)]
//! Auto-generated Salesforce metadata types from @salesforce/types
//! Source: <https://github.com/forcedotcom/wsdl>
//!
//! DO NOT EDIT - This file is automatically generated

use crate::traits::{JsonSerializable, MetadataType, SettingsType};
use serde::{Deserialize, Serialize};
#[cfg(feature = "schemars")]
use schemars::JsonSchema;

"#,
    );

    let mut generated_types = std::collections::HashSet::new();

    // Priority enums
    let priority_types = [
        "FieldType",
        "SharingModel",
        "DeploymentStatus",
        "Gender",
        "DeployStatus",
    ];

    for type_name in priority_types {
        if let Some(variants) = defs.union_types.get(type_name) {
            output.push_str(&generate_enum(type_name, variants));
            output.push('\n');
            generated_types.insert(type_name.to_string());
        }
    }

    // All remaining enums
    for (type_name, variants) in &defs.union_types {
        if !generated_types.contains(type_name) {
            output.push_str(&generate_enum(type_name, variants));
            output.push('\n');
            generated_types.insert(type_name.clone());
        }
    }

    // Priority structs
    let priority_structs = [
        "CustomObject",
        "CustomField",
        "Layout",
        "PermissionSet",
        "ValidationRule",
        "WorkflowRule",
    ];

    for struct_name in priority_structs {
        if let Some(fields) = defs.interface_types.get(struct_name) {
            output.push_str(&generate_struct(struct_name, fields));
            output.push('\n');
            generated_types.insert(struct_name.to_string());
        }
    }

    // All remaining structs
    for (type_name, fields) in &defs.interface_types {
        if !generated_types.contains(type_name) {
            output.push_str(&generate_struct(type_name, fields));
            output.push('\n');
        }
    }

    // Generate trait implementations
    output.push_str("\n// Trait Implementations\n");
    for type_name in defs.interface_types.keys() {
        if type_name == "Metadata" {
            continue;
        }

        // Determine if this is a Settings type
        let is_settings = type_name.ends_with("Settings");

        // Determine api_name logic
        let api_name_impl = if let Some(fields) = defs.interface_types.get(type_name) {
            if fields.iter().any(|f| f.name == "fullName") {
                r#"Some(&self.full_name)"#
            } else {
                r#"None"#
            }
        } else {
            r#"None"#
        };

        // Implement MetadataType
        output.push_str(&format!(
            r#"
impl MetadataType for {} {{
    const METADATA_TYPE_NAME: &'static str = "{}";
    const XML_ROOT_ELEMENT: &'static str = "{}";

    fn api_name(&self) -> Option<&str> {{
        {}
    }}
}}
"#,
            type_name, type_name, type_name, api_name_impl
        ));

        // Implement JsonSerializable for all MetadataTypes
        output.push_str(&format!(
            r#"
impl JsonSerializable for {} {{}}
"#,
            type_name
        ));

        // Implement SettingsType if applicable
        if is_settings {
            let key = type_name.to_case(Case::Camel);
            let key = key.strip_suffix("Settings").unwrap_or(&key);
            output.push_str(&format!(
                r#"
impl SettingsType for {} {{
    const SCRATCH_DEF_KEY: &'static str = "{}";
}}
"#,
                type_name, key
            ));
        }
    }

    Ok(output)
}

/// Generate a Rust enum from TypeScript union type
fn generate_enum(name: &str, variants: &[String]) -> String {
    let mut output = format!(
        r#"#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub enum {} {{
    #[default]
"#,
        name
    );

    for variant in variants.iter() {
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

        if rust_variant == "Self" {
            rust_variant = "SelfValue".to_string();
        } else if RESERVED_WORDS.contains(&rust_variant.as_str()) {
            rust_variant = format!("r#{}", rust_variant);
        }

        if rust_variant != *variant {
            output.push_str(&format!("    #[serde(rename = \"{}\")]\n", variant));
        }
        output.push_str(&format!("    {},\n", rust_variant));
    }

    output.push_str("}\n");
    output
}

/// Generate a Rust struct from TypeScript interface
fn generate_struct(name: &str, fields: &[FieldDef]) -> String {
    let mut output = format!(
        r#"#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct {} {{
"#,
        name
    );

    let mut seen_rust_names = std::collections::HashSet::new();

    for field in fields {
        let rust_field_name = field.name.to_case(Case::Snake);

        if seen_rust_names.contains(&rust_field_name) {
            continue;
        }
        seen_rust_names.insert(rust_field_name.clone());

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

        let resolved_type = match field.type_ref.as_str() {
            "String" | "bool" | "f64" => field.type_ref.clone(),
            t if t.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) => {
                "serde_json::Value".to_string()
            }
            _ => "serde_json::Value".to_string(),
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

#[derive(Debug, Deserialize)]
struct TypeOverlay {
    description: Option<String>,
    #[serde(default)]
    fields: BTreeMap<String, String>,
}

fn apply_overlays(
    interface_types: &mut HashMap<String, Vec<FieldDef>>,
    descriptions: &mut HashMap<String, String>,
) {
    let overlays = get_overlays();

    for (type_name, overlay) in overlays {
        // Update type description
        if let Some(desc) = overlay.description {
            descriptions.insert(type_name.clone(), desc);
        }

        // Update field descriptions
        if let Some(fields) = interface_types.get_mut(&type_name) {
            for field in fields {
                if let Some(desc) = overlay.fields.get(&field.name) {
                    field.description = Some(desc.clone());
                }
            }
        }
    }
}

fn get_overlays() -> HashMap<String, TypeOverlay> {
    // Try multiple candidate paths: the file lives next to the sf-typegen Cargo.toml,
    // but the generator is typically invoked from the repo root via scripts/generate.sh.
    let candidates = [
        // When run from repo root (the usual CI / generate.sh path)
        Path::new("sf-typegen/overlays.toml"),
        // When run from inside the sf-typegen directory
        Path::new("overlays.toml"),
    ];

    let overlay_path = match candidates.iter().find(|p| p.exists()) {
        Some(p) => *p,
        None => {
            println!("Warning: overlays.toml not found, proceeding without overlays.");
            return HashMap::new();
        }
    };

    let content = fs::read_to_string(overlay_path).expect("Failed to read overlays.toml");
    let overlays: HashMap<String, TypeOverlay> =
        toml::from_str(&content).expect("Failed to parse overlays.toml");

    overlays
}

fn report_missing_overlays(defs: &TypeDefinitions) {
    let overlays = get_overlays();
    let mut missing = Vec::new();

    for type_name in defs.interface_types.keys() {
        // Skip some internal types if needed, or types that don't need docs
        if !overlays.contains_key(type_name) {
            missing.push(type_name.clone());
        }
    }

    if !missing.is_empty() {
        println!(
            "\n⚠️  Found {} types without overlays (documentation injection).",
            missing.len()
        );
        missing.sort();

        let report_path = "missing_overlays.json";
        if let Ok(json) = serde_json::to_string_pretty(&missing) {
            if let Err(e) = std::fs::write(report_path, json) {
                println!("   Failed to write {}: {}", report_path, e);
            } else {
                println!("   Written missing types to {}", report_path);
            }
        }
    } else {
        println!("\n✅ All types have overlay definitions!");
    }
}
