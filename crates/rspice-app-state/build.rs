//! Embed the built-in Verilog-A model sources: the text the Library page shows
//! read-only and a copy of a model starts from.

use std::path::{Path, PathBuf};
use std::{env, fs};

fn main() {
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("set by cargo"));
    let root = manifest
        .join("..")
        .join("..")
        .join("models")
        .join("veriloga");
    println!("cargo:rerun-if-changed={}", root.display());
    println!("cargo:rerun-if-changed=build.rs");

    let mut found = Vec::new();
    collect_sources(&root, &mut found);
    let mut sources = found
        .into_iter()
        .map(|path| {
            let relative = path
                .strip_prefix(&root)
                .expect("found under the root")
                .to_string_lossy()
                .replace('\\', "/");
            (relative, path)
        })
        .collect::<Vec<_>>();
    sources.sort();

    let mut generated = String::from("pub(crate) static VERILOGA_SOURCES: &[(&str, &str)] = &[\n");
    for (relative, path) in &sources {
        generated.push_str(&format!(
            "    ({relative:?}, include_str!({:?})),\n",
            path.display().to_string()
        ));
    }
    generated.push_str("];\n");
    let out = PathBuf::from(env::var("OUT_DIR").expect("set by cargo"));
    fs::write(out.join("veriloga_sources.rs"), generated).expect("OUT_DIR is writable");
}

fn collect_sources(folder: &Path, found: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(folder).expect("the Verilog-A model tree exists") {
        let path = entry.expect("a readable entry").path();
        if path.is_dir() {
            collect_sources(&path, found);
        } else if path
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| matches!(extension, "va" | "include" | "inc" | "vams" | "h"))
        {
            found.push(path);
        }
    }
}
