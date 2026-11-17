//! The SPICE deck File ▸ Export ▸ Netlist writes for use outside the
//! application, with its Verilog-A include paths resolved so the deck still
//! finds its sources.

use std::collections::{HashMap, HashSet};

use rspice_simulation::veriloga::ModelLibraryVerilogAFiles;

/// A Verilog-A file an exported deck needs beside it.
pub struct ExportedSource {
    /// Relative to the folder the deck is written to, with `/` separators.
    pub relative_path: String,
    pub content: String,
}

/// A deck for another tool and the files to write beside it.
pub struct NetlistExport {
    pub netlist: String,
    pub sources: Vec<ExportedSource>,
}

/// The deck as another tool reads it, in SPICE.
///
/// The deck the run executes names its Verilog-A by the keys of the sources
/// the run sealed, which exist nowhere outside the project. Each is copied
/// into a folder named after the deck, and the deck names it by that
/// relative path, so the folder can be moved whole. See
/// [`relocated_sources`].
pub fn spice_export(
    spice_netlist: &str,
    workspace: &crate::state::ProjectWorkspace,
    libraries: &[ModelLibraryVerilogAFiles],
    deck_stem: &str,
) -> NetlistExport {
    let (relocated, sources) = relocated_sources(spice_netlist, workspace, libraries, deck_stem);
    let mut netlist = spice_netlist
        .lines()
        .map(|line| relocated_veriloga_line(line, &relocated))
        .collect::<Vec<_>>()
        .join("\n");
    if spice_netlist.ends_with('\n') {
        netlist.push('\n');
    }
    NetlistExport { netlist, sources }
}

/// Where each Verilog-A source the deck names by deck key is written, by its
/// path relative to the deck, and the files themselves.
///
/// A project-owned source exists nowhere outside the project, and a model
/// library's source is the sealed copy the run compiled, so both are copied:
/// a project's into `<deck>_veriloga/<cell>`, a library's whole source set
/// into `<deck>_veriloga/<root>` so an `` `include `` between its files still
/// resolves. A path the deck names that is no key is a file already on disk
/// and keeps its path, as it would in a Virtuoso netlist.
fn relocated_sources(
    spice_netlist: &str,
    workspace: &crate::state::ProjectWorkspace,
    libraries: &[ModelLibraryVerilogAFiles],
    deck_stem: &str,
) -> (HashMap<String, String>, Vec<ExportedSource>) {
    let project = workspace.content.project.id();
    let registry = &workspace.content.project_sources;
    let mut relocated = HashMap::new();
    let mut folders = HashSet::new();
    let mut written_libraries = HashMap::new();
    let mut sources = Vec::new();
    for line in spice_netlist.lines() {
        let Some(key) = parse_veriloga_include_path(line) else {
            continue;
        };
        if relocated.contains_key(&key) {
            continue;
        }
        if let Some(bundle) = registry.veriloga_bundle_for_source_key(project, &key) {
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
            continue;
        }
        let Some((index, library)) = libraries
            .iter()
            .enumerate()
            .find(|(_, library)| key.starts_with(&library.key_prefix))
        else {
            continue;
        };
        let folder = written_libraries.entry(index).or_insert_with(|| {
            let owner = library
                .root
                .rsplit('/')
                .next()
                .and_then(|name| name.split('.').next())
                .unwrap_or("library");
            let folder = unique_folder(
                format!(
                    "{}_veriloga/{}",
                    portable_name(deck_stem),
                    portable_name(owner)
                ),
                &mut folders,
            );
            sources.extend(library.files.iter().map(|(path, content)| ExportedSource {
                relative_path: format!("{folder}/{path}"),
                content: content.clone(),
            }));
            folder
        });
        relocated.insert(key, format!("{folder}/{}", library.root));
    }
    (relocated, sources)
}

/// A `.veriloga` card naming a relocated source by its new path, keeping
/// the module name the card gives it.
fn relocated_veriloga_line(line: &str, relocated: &HashMap<String, String>) -> String {
    let Some((key, path)) = parse_veriloga_include_path(line)
        .and_then(|key| relocated.get(&key).map(|path| (key, path)))
    else {
        return line.to_owned();
    };
    let quoted = quote_netlist_path(&key);
    if line.contains(&quoted) {
        line.replacen(&quoted, &quote_netlist_path(path), 1)
    } else {
        line.replacen(&key, &quote_netlist_path(path), 1)
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
    unique_folder(
        format!(
            "{}_veriloga/{}",
            portable_name(deck_stem),
            portable_name(&owner)
        ),
        taken,
    )
}

/// `base`, or `base_2`, `base_3`, ... when another source took it.
fn unique_folder(base: String, taken: &mut HashSet<String>) -> String {
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
