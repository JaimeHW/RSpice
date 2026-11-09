//! SPICE-to-AHDL netlist compatibility.
//!
//! Rewrites the constructs an AHDL-flavoured engine spells differently, and
//! resolves Verilog-A include paths so a deck exported for another tool
//! still finds its sources.

use std::collections::{HashMap, HashSet};

/// A project-owned Verilog-A file an exported deck needs beside it.
pub(super) struct ExportedSource {
    /// Relative to the folder the deck is written to, with `/` separators.
    pub(super) relative_path: String,
    pub(super) content: String,
}

/// An AHDL deck and the files to write beside it.
pub(super) struct AhdlExport {
    pub(super) netlist: String,
    pub(super) sources: Vec<ExportedSource>,
}

/// The deck as another tool reads it.
///
/// A project-owned source exists nowhere outside the project, so it is copied
/// into a folder named after the deck and the deck names it by relative path.
/// A source already on disk keeps its path, as it would in a Virtuoso netlist.
pub(super) fn spice_to_ahdl_export(
    spice_netlist: &str,
    workspace: &crate::state::ProjectWorkspace,
    deck_stem: &str,
) -> AhdlExport {
    let project = workspace.content.project.id();
    let registry = &workspace.content.project_sources;
    let mut relocated = HashMap::new();
    let mut folders = HashSet::new();
    let mut sources = Vec::new();
    for line in spice_netlist.lines() {
        let Some(key) = parse_veriloga_include_path(line) else {
            continue;
        };
        if relocated.contains_key(&key) {
            continue;
        }
        let Some(bundle) = registry.veriloga_bundle_for_source_key(project, &key) else {
            continue;
        };
        let folder = source_folder(deck_stem, bundle, &mut folders);
        let root = format!("{folder}/{}", bundle.root().logical_path());
        sources.push(ExportedSource {
            relative_path: root.clone(),
            content: bundle.root().content().to_owned(),
        });
        sources.extend(bundle.files().iter().map(|file| ExportedSource {
            relative_path: format!("{folder}/{}", file.logical_path()),
            content: file.content().to_owned(),
        }));
        relocated.insert(key, root);
    }
    AhdlExport {
        netlist: ahdl_netlist(spice_netlist, &relocated),
        sources,
    }
}

/// `<deck>_veriloga/<cell>`, the way a library keeps one folder per cell.
fn source_folder(
    deck_stem: &str,
    bundle: &crate::state::ProjectSourceBundle,
    taken: &mut HashSet<String>,
) -> String {
    let owner = match bundle.owner() {
        crate::state::ProjectSourceOwner::CellView { reference } => reference.cell.clone(),
        crate::state::ProjectSourceOwner::CodeWorkspace { .. } => bundle
            .root()
            .logical_path()
            .rsplit('/')
            .next()
            .and_then(|name| name.split('.').next())
            .unwrap_or("source")
            .to_owned(),
    };
    let base = format!(
        "{}_veriloga/{}",
        portable_name(deck_stem),
        portable_name(&owner)
    );
    let mut folder = base.clone();
    let mut suffix = 2_usize;
    while !taken.insert(folder.clone()) {
        folder = format!("{base}_{suffix}");
        suffix += 1;
    }
    folder
}

fn portable_name(name: &str) -> String {
    name.chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '_' | '-' | '.') {
                character
            } else {
                '_'
            }
        })
        .collect()
}

pub(super) fn spice_to_ahdl_compatible_netlist(spice_netlist: &str) -> String {
    ahdl_netlist(spice_netlist, &HashMap::new())
}

/// One `ahdl_include` and the comment naming the design object it serves.
struct AhdlInclude {
    path: String,
    origin: Option<String>,
}

fn ahdl_netlist(spice_netlist: &str, relocated: &HashMap<String, String>) -> String {
    let mut includes: Vec<AhdlInclude> = Vec::new();
    let mut seen_paths: HashSet<String> = HashSet::new();
    let mut retained_lines: Vec<&str> = Vec::new();
    // What the deck calls each module, lower-cased, against its source's name.
    let mut modules: HashMap<String, String> = HashMap::new();

    for line in spice_netlist.lines() {
        let Some(path) = parse_veriloga_include_path(line) else {
            retained_lines.push(line);
            continue;
        };
        let origin = retained_lines
            .last()
            .copied()
            .filter(|previous| veriloga_origin_module(previous).is_some());
        if let Some(origin) = origin {
            retained_lines.pop();
            if let (Some(module), Some(alias)) =
                (veriloga_origin_module(origin), parse_veriloga_alias(line))
            {
                modules.insert(alias.to_ascii_lowercase(), module.to_owned());
            }
        }
        let path = relocated.get(&path).cloned().unwrap_or(path);
        if seen_paths.insert(path.clone()) {
            includes.push(AhdlInclude {
                path,
                origin: origin.map(|comment| comment.trim_start_matches("* ").to_owned()),
            });
        }
    }

    if includes.is_empty() {
        return retained_lines.join("\n");
    }

    // Where the generator headed its sources, or else ahead of the first card.
    let heading = retained_lines
        .iter()
        .position(|line| line.trim() == "* Verilog-A sources");
    let mut output: Vec<String> = Vec::with_capacity(retained_lines.len() + includes.len() * 2 + 3);
    let mut inserted_prefix = false;
    for (index, line) in retained_lines.into_iter().enumerate() {
        let trimmed = line.trim();
        let at_first_card = heading.is_none() && !trimmed.is_empty() && !trimmed.starts_with('*');
        if !inserted_prefix && at_first_card {
            push_ahdl_prefix(&mut output, &includes);
            inserted_prefix = true;
        }
        output.push(with_source_module_names(line, &modules));
        if heading == Some(index) {
            push_ahdl_prefix(&mut output, &includes);
            inserted_prefix = true;
        }
    }

    if !inserted_prefix {
        push_ahdl_prefix(&mut output, &includes);
    }

    output.join("\n")
}

/// The source's module name on an instance line where the deck used an alias:
/// the other tool compiles the module under the name the source gives it.
fn with_source_module_names(line: &str, modules: &HashMap<String, String>) -> String {
    let is_instance = line
        .trim_start()
        .chars()
        .next()
        .is_some_and(|first| first.eq_ignore_ascii_case(&'x'));
    if !is_instance || modules.is_empty() {
        return line.to_owned();
    }
    line.split(' ')
        .map(|token| {
            modules
                .get(&token.to_ascii_lowercase())
                .map_or(token, String::as_str)
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn push_ahdl_prefix(output: &mut Vec<String>, includes: &[AhdlInclude]) {
    output.push("simulator lang=spectre".to_string());
    for include in includes {
        if let Some(origin) = &include.origin {
            output.push(format!("// {origin}"));
        }
        output.push(format!(
            "ahdl_include {}",
            quote_netlist_path(&include.path)
        ));
    }
    output.push("simulator lang=spice".to_string());
}

/// The module named by a `* lib/cell/view (module name)` comment.
fn veriloga_origin_module(line: &str) -> Option<&str> {
    line.trim_start()
        .strip_prefix("* ")?
        .split_once(" (module ")?
        .1
        .strip_suffix(')')
}

/// The name a `.veriloga "path" name` card gives the module, when it gives one.
fn parse_veriloga_alias(line: &str) -> Option<&str> {
    let rest = line.trim_start().get(9..)?.trim_start();
    let after_path = match rest.strip_prefix('"') {
        Some(quoted) => &quoted[quoted.find('"')? + 1..],
        None => rest.split_once(char::is_whitespace)?.1,
    };
    after_path.split_whitespace().next()
}

pub(super) fn parse_veriloga_include_path(line: &str) -> Option<String> {
    let trimmed = line.trim_start();
    let prefix = trimmed.get(..9)?;
    if !prefix.eq_ignore_ascii_case(".veriloga") {
        return None;
    }

    let rest = trimmed.get(9..)?.trim_start();
    if rest.is_empty() {
        return None;
    }

    if let Some(quoted) = rest.strip_prefix('"') {
        let end = quoted.find('"')?;
        return Some(quoted[..end].to_string());
    }

    rest.split_whitespace().next().map(ToString::to_string)
}

pub(super) fn quote_netlist_path(path: &str) -> String {
    let escaped = path.replace('"', "\\\"");
    format!("\"{}\"", escaped)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DECK: &str = "* RSpice Netlist\n\n* Verilog-A sources\n* work/ota/veriloga (module ota_core)\n.VERILOGA \"key/ota.va\" ota_core_1a2b3c4d\n\n* Circuit netlist\nX1 a b ota_core_1a2b3c4d gain=5\nR1 a b 1k\n.end";

    #[test]
    fn an_exported_deck_names_its_sources_by_their_relocated_path_and_their_modules() {
        let relocated =
            HashMap::from([("key/ota.va".to_owned(), "d_veriloga/ota/ota.va".to_owned())]);

        assert_eq!(
            ahdl_netlist(DECK, &relocated),
            "* RSpice Netlist\n\n* Verilog-A sources\nsimulator lang=spectre\n// work/ota/veriloga (module ota_core)\nahdl_include \"d_veriloga/ota/ota.va\"\nsimulator lang=spice\n\n* Circuit netlist\nX1 a b ota_core gain=5\nR1 a b 1k\n.end"
        );
    }

    #[test]
    fn a_deck_without_the_generators_comment_keeps_its_names_and_its_order() {
        assert_eq!(
            spice_to_ahdl_compatible_netlist("* t\n.veriloga \"a.va\" m\nX1 a m\n.end"),
            "* t\nsimulator lang=spectre\nahdl_include \"a.va\"\nsimulator lang=spice\nX1 a m\n.end"
        );
    }
}
