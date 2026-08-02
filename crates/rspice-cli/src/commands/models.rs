//! `rspice models` — inspect the shipped SPICE model packs.

use rspice_core::library::{CatalogEntry, MODELS_DIR_ENV, SpiceLibraryIndex};

use crate::cli::{CliError, ModelsArgs};

fn invalid(message: impl Into<String>, suggestion: Option<&str>) -> CliError {
    CliError::InvalidArgument {
        message: message.into(),
        suggestion: suggestion.map(str::to_string),
    }
}

/// Cap on rows printed for a browse query, so a bare `--device diode` does not
/// spool 81,000 lines at the terminal.
const BROWSE_LIMIT: usize = 200;

pub fn models(args: ModelsArgs, _verbose: bool, quiet: bool) -> Result<(), CliError> {
    let index = match &args.models_dir {
        Some(dir) => Some(SpiceLibraryIndex::open(dir).map_err(|source| {
            invalid(
                format!("cannot open model tree at {}: {source}", dir.display()),
                Some("expected a directory containing PACKS.tsv and CATALOG.tsv"),
            )
        })?),
        None => SpiceLibraryIndex::discover().map_err(|source| {
            invalid(
                format!("cannot read the shipped model tree: {source}"),
                None,
            )
        })?,
    };

    let Some(index) = index else {
        // Not fatal to the product: the built-in library is compiled in, so
        // RSpice simulates fine without the packs. Only this command needs them.
        return Err(invalid(
            "no SPICE model tree found; the built-in library is compiled in, but \
             the packs ship as data beside the binary",
            Some(&format!(
                "set {MODELS_DIR_ENV}, or pass --models-dir pointing at a \
                 models/spice directory"
            )),
        ));
    };

    if let Some(part) = &args.part {
        return show_part(&index, part, quiet);
    }
    if let Some(device) = &args.device {
        let hits = index
            .parts_by_device(device, BROWSE_LIMIT)
            .map_err(catalog_err)?;
        return browse(hits, quiet);
    }
    if let Some(search) = &args.search {
        let hits = index
            .search_parts(search, BROWSE_LIMIT)
            .map_err(catalog_err)?;
        return browse(hits, quiet);
    }

    list_packs(&index, args.shippable_only, quiet)
}

fn list_packs(
    index: &SpiceLibraryIndex,
    shippable_only: bool,
    quiet: bool,
) -> Result<(), CliError> {
    if !quiet {
        println!("Model tree: {}", index.root().display());
    }

    let packs: Vec<_> = if shippable_only {
        index.redistributable_packs().collect()
    } else {
        index.packs().iter().collect()
    };

    println!(
        "{:<22} {:<10} {:<14} {:>9} {:>9}  NAME",
        "PACK", "CATEGORY", "LICENCE", "MODELS", "SUBCKTS"
    );
    for pack in &packs {
        let licence = if pack.redistributable {
            pack.tier.display_name().to_string()
        } else {
            format!("{}*", pack.tier.display_name())
        };
        println!(
            "{:<22} {:<10} {:<14} {:>9} {:>9}  {}",
            pack.id, pack.category, licence, pack.models, pack.subcircuits, pack.name
        );
    }

    if !quiet {
        let models: usize = packs.iter().map(|p| p.models).sum();
        let subckts: usize = packs.iter().map(|p| p.subcircuits).sum();
        let bytes: u64 = packs.iter().map(|p| p.bytes).sum();
        println!(
            "\n{} packs, {models} models, {subckts} subcircuits, {:.1} MB",
            packs.len(),
            bytes as f64 / 1_048_576.0
        );
        if !shippable_only && packs.iter().any(|pack| !pack.redistributable) {
            println!(
                "* redistribution not established; excluded by --shippable-only. \
                 See models/spice/LICENSE-AUDIT.tsv."
            );
        }
    }
    Ok(())
}

fn show_part(index: &SpiceLibraryIndex, part: &str, quiet: bool) -> Result<(), CliError> {
    let matches = index.find_part(part).map_err(catalog_err)?;
    if matches.is_empty() {
        return Err(invalid(
            format!("no definition named '{part}' in any pack"),
            Some("try --search for a substring match"),
        ));
    }

    if !quiet && matches.len() > 1 {
        // Not a warning to be dismissed: the same part number carries different
        // parameter fits in different packs, so the deck must say which it means.
        println!(
            "'{part}' is defined in {} packs. Reference one explicitly with \
             .include or .lib; the fits are not interchangeable.\n",
            matches
                .iter()
                .map(|m| m.pack.as_str())
                .collect::<std::collections::BTreeSet<_>>()
                .len()
        );
    }

    for entry in &matches {
        println!(
            "{:<24} {:<8} {:<16} {}{}",
            entry.name,
            entry.kind,
            entry.device,
            entry.pack,
            if entry.restricted {
                "  [restricted]"
            } else {
                ""
            }
        );
        if let Some(path) = entry.source_path(index) {
            println!("    {}:{}", path.display(), entry.line);
        }
    }
    Ok(())
}

fn browse(entries: Vec<CatalogEntry>, quiet: bool) -> Result<(), CliError> {
    if entries.is_empty() {
        return Err(invalid("no definitions matched", None));
    }
    for entry in &entries {
        println!(
            "{:<28} {:<8} {:<16} {}{}",
            entry.name,
            entry.kind,
            entry.device,
            entry.pack,
            if entry.restricted {
                "  [restricted]"
            } else {
                ""
            }
        );
    }
    if !quiet && entries.len() >= BROWSE_LIMIT {
        println!("\n(truncated at {BROWSE_LIMIT}; narrow the query)");
    }
    Ok(())
}

fn catalog_err(source: std::io::Error) -> CliError {
    invalid(
        format!("cannot read the part catalog: {source}"),
        Some("regenerate it with tools/models/build_manifest.py"),
    )
}
