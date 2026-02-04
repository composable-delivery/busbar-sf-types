use regex::Regex;
use std::{fs, path::Path};

fn main() {
    let src_path = Path::new("src/trait_impls.rs");
    let out_path = Path::new("src/generated_registry.rs");

    let src = match fs::read_to_string(src_path) {
        Ok(s) => s,
        Err(e) => {
            panic!("Failed to read {}: {}", src_path.display(), e);
        }
    };

    let re_impl =
        Regex::new(r"impl\s+crate::traits::MetadataType\s+for\s+([A-Za-z0-9_]+)\s*\{").unwrap();
    let re_root =
        Regex::new(r#"const\s+XML_ROOT_ELEMENT:\s*&'static\s+str\s*=\s*\"([^\"]+)\";"#).unwrap();
    let re_cfg = Regex::new(r"^\s*#\s*\[cfg[^(]*\]").unwrap();

    let mut entries = Vec::new();

    for m in re_impl.captures_iter(&src) {
        let type_name = m.get(1).unwrap().as_str().to_string();
        let impl_pos = m.get(0).unwrap().start();

        // search forward from impl for XML_ROOT_ELEMENT within a reasonable window
        let tail = &src[impl_pos..std::cmp::min(src.len(), impl_pos + 2000)];
        if let Some(rc) = re_root.captures(tail) {
            let root = rc.get(1).unwrap().as_str().to_string();

            // collect cfg attributes immediately preceding the impl (up to 8 lines)
            let prefix = &src[..impl_pos];
            let mut cfg_lines = Vec::new();
            for line in prefix.lines().rev().take(8) {
                if re_cfg.is_match(line) {
                    cfg_lines.push(line.trim().to_string());
                } else if line.trim().is_empty() {
                    continue;
                } else {
                    break;
                }
            }
            cfg_lines.reverse();

            entries.push((type_name, root, cfg_lines));
        }
    }

    // Generate registry source
    let mut out = String::new();
    out.push_str("// Auto-generated registry mapping XML root element -> concrete parser\n");
    out.push_str("use crate::prelude::*;\nuse crate::traits::XmlError;\nuse crate::traits::XmlSerializable;\nuse serde_json::Value;\n\n");
    out.push_str("pub fn parse_by_root(root: &str, xml: &str) -> Result<Value, XmlError> {\n    match root {\n");

    for (type_name, root, cfg_lines) in entries {
        for cfg in &cfg_lines {
            out.push_str(&format!("{}\n", cfg));
        }

        out.push_str(&format!("        \"{}\" => {{\n", root));
        out.push_str(&format!("            // try to parse as {}\n", type_name));
        out.push_str(&format!(
            "            let parsed: Result<{}, XmlError> = {}::from_metadata_xml(xml);\n",
            type_name, type_name
        ));
        out.push_str("            match parsed {\n");
        out.push_str("                Ok(v) => return Ok(serde_json::to_value(&v).map_err(|e| XmlError::Serialize(e.to_string()))?),\n");
        out.push_str("                Err(e) => return Err(e),\n");
        out.push_str("            }\n");
        out.push_str("        }\n");
    }

    out.push_str(
        "        _ => Err(XmlError::Deserialize(format!(\"Unknown root: {}\", root))),\n    }\n}\n",
    );

    fs::write(out_path, out).expect("Failed to write generated_registry.rs");
}
