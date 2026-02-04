use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::Result;
use clap::Parser;

/// Integration runner that parses metadata XML files into concrete generated types
#[derive(Parser)]
struct Args {
    /// Input directory containing metadata XML files
    input: PathBuf,

    /// Output directory to write parsed JSON files
    #[clap(long, default_value = "output")]
    output: PathBuf,
}

fn visit_dir(root: &Path) -> Vec<PathBuf> {
    let mut stack = vec![root.to_path_buf()];
    let mut files = Vec::new();

    while let Some(path) = stack.pop() {
        if let Ok(meta) = fs::metadata(&path) {
            if meta.is_dir() {
                if let Ok(entries) = fs::read_dir(&path) {
                    for e in entries.flatten() {
                        stack.push(e.path());
                    }
                }
            } else if meta.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "xml" {
                        files.push(path);
                    }
                }
            }
        }
    }

    files
}

fn ensure_parent(out: &Path) -> Result<()> {
    if let Some(p) = out.parent() {
        fs::create_dir_all(p)?;
    }
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    let files = visit_dir(&args.input);
    println!("Found {} XML files", files.len());

    for path in files {
        let xml = match fs::read_to_string(&path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to read {}: {}", path.display(), e);
                continue;
            }
        };

        // quick root detection
        let root = match roxmltree::Document::parse(&xml) {
            Ok(doc) => doc.root_element().tag_name().name().to_string(),
            Err(e) => {
                eprintln!("Failed to parse root for {}: {}", path.display(), e);
                continue;
            }
        };

        // attempt concrete parse using registry in busbar-sf-types
        match busbar_sf_types::parse_metadata_by_root(&root, &xml) {
            Ok(value) => {
                // write JSON to output/<relative_path>.json
                let rel = path.strip_prefix(&args.input).unwrap_or(&path);
                let mut out_path = args.output.join(rel);
                out_path.set_extension("json");
                ensure_parent(&out_path)?;
                let mut f = fs::File::create(&out_path)?;
                let j = serde_json::to_string_pretty(&value)?;
                f.write_all(j.as_bytes())?;
            }
            Err(e) => {
                eprintln!("Failed to parse {} as {}: {}", path.display(), root, e);
                // write an error file next to output
                let rel = path.strip_prefix(&args.input).unwrap_or(&path);
                let mut out_path = args.output.join(rel);
                out_path.set_extension("error");
                ensure_parent(&out_path)?;
                let mut f = fs::File::create(&out_path)?;
                writeln!(f, "{}", e)?;
            }
        }
    }

    Ok(())
}
