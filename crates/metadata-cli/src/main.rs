use anyhow::Result;
use clap::Parser;
use quick_xml::de as qx_de;
use roxmltree::Document;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Parse Salesforce metadata XML files and summarize types"
)]
struct Args {
    /// Directory containing metadata to parse
    dir: PathBuf,

    /// Write summary JSON to this file (otherwise prints to stdout)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Pretty-print JSON
    #[arg(long)]
    pretty: bool,
}

#[derive(Serialize)]
struct TypeInfo {
    count: usize,
    examples: Vec<String>,
}

#[derive(Serialize)]
struct Summary {
    total_files: usize,
    parsed: usize,
    errors: usize,
    types: HashMap<String, TypeInfo>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut total_files = 0usize;
    let mut parsed = 0usize;
    let mut errors = 0usize;
    let mut types: HashMap<String, TypeInfo> = HashMap::new();

    visit_dir(&args.dir, &mut |path: &Path| {
        total_files += 1;
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            if ext.eq_ignore_ascii_case("xml") {
                match fs::read_to_string(path) {
                    Ok(text) => match Document::parse(&text) {
                        Ok(doc) => {
                            // Use root element name as metadata type
                            let root = doc.root_element();
                            let t = root.tag_name().name().to_string();
                            // try a serde-based deserialization into a generic value to validate
                            match qx_de::from_str::<serde_json::Value>(&text) {
                                Ok(_) => {
                                    let info = types.entry(t.clone()).or_insert(TypeInfo {
                                        count: 0,
                                        examples: Vec::new(),
                                    });
                                    info.count += 1;
                                    if info.examples.len() < 5 {
                                        if let Some(p) = path.to_str() {
                                            info.examples.push(p.to_string());
                                        }
                                    }
                                    parsed += 1;
                                }
                                Err(e) => {
                                    errors += 1;
                                    let info = types.entry(t.clone()).or_insert(TypeInfo {
                                        count: 0,
                                        examples: Vec::new(),
                                    });
                                    if info.examples.len() < 3 {
                                        if let Some(p) = path.to_str() {
                                            info.examples.push(format!("ERROR:{} -> {}", p, e));
                                        }
                                    }
                                }
                            }
                        }
                        Err(_) => {
                            errors += 1;
                        }
                    },
                    Err(_) => errors += 1,
                }
            }
        }
    })?;

    let summary = Summary {
        total_files,
        parsed,
        errors,
        types,
    };

    let out = if args.pretty {
        serde_json::to_string_pretty(&summary)?
    } else {
        serde_json::to_string(&summary)?
    };

    if let Some(out_path) = args.output {
        fs::write(out_path, out)?;
    } else {
        println!("{}", out);
    }

    Ok(())
}

fn types_map_entry<'a>(
    map: &'a mut HashMap<String, TypeInfo>,
    type_name: &str,
    path: &Path,
) -> &'a mut TypeInfo {
    let info = map.entry(type_name.to_string()).or_insert(TypeInfo {
        count: 0,
        examples: Vec::new(),
    });
    if info.examples.len() < 5 {
        if let Some(p) = path.to_str() {
            info.examples.push(p.to_string());
        }
    }
    info
}

fn visit_dir<F>(dir: &Path, cb: &mut F) -> Result<()>
where
    F: FnMut(&Path),
{
    if !dir.exists() {
        return Ok(());
    }
    let mut stack = vec![dir.to_path_buf()];
    while let Some(p) = stack.pop() {
        let read = match fs::read_dir(&p) {
            Ok(rd) => rd,
            Err(_) => continue,
        };
        for entry in read.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.is_file() {
                cb(&path);
            }
        }
    }
    Ok(())
}
