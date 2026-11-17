//! What each library the project depends on stands at: one status per
//! library, read by every place that names one.
//!
//! A design library is read from the project file, so it is in the project
//! for as long as the catalogue holds it. A model library is built in, or it
//! executes from source files whose bytes the project pinned when they were
//! attached; whether those still hold is the retained drift report's answer,
//! which is decided on events and never on a frame. Nothing here reads the
//! disk or hashes a byte, so a status is as cheap to ask for as the report
//! and the catalogues are to read.
//!
//! The rail, the table and the inspector beside the Dependencies page read
//! the rows built here, once per change of what a status reads, so the three
//! list one set of libraries in one order under one filter and say the same
//! thing about each.

use std::collections::BTreeSet;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::Path;
use std::sync::Arc;

use crate::state::model_library::{ModelLibrary, ModelSourceAuthority};
use crate::workbench::app_state::Model;
use crate::workbench::app_state::model::ModelSourceDriftScope;
use crate::workbench::surfaces::project::derived::derived;
use crate::workbench::surfaces::project::overview::Tone;

/// Painted in a column the library on that row has no value for.
pub(crate) const UNSET: &str = "\u{2014}";

/// Which catalogue a library is in. A design library and a model library may
/// share a name, so a library is named by both.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DependencyKind {
    Design,
    Model,
}

impl DependencyKind {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Design => "Design library",
            Self::Model => "Model library",
        }
    }

    /// The word for the catalogue under a "Type" heading, which says the
    /// rest.
    pub const fn kind_word(self) -> &'static str {
        match self {
            Self::Design => "Design",
            Self::Model => "Model",
        }
    }
}

/// One library the project depends on, by catalogue and name.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DependencyLibrary {
    pub kind: DependencyKind,
    pub name: String,
}

impl DependencyLibrary {
    pub fn design(name: impl Into<String>) -> Self {
        Self {
            kind: DependencyKind::Design,
            name: name.into(),
        }
    }

    pub fn model(name: impl Into<String>) -> Self {
        Self {
            kind: DependencyKind::Model,
            name: name.into(),
        }
    }
}

/// Where one library stands: the mark it earns, the word for it, and what
/// the reader needs to act on it when there is more to say.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LibraryStatus {
    pub mark: Tone,
    pub label: &'static str,
    pub detail: Option<String>,
}

impl LibraryStatus {
    fn neutral(label: &'static str, detail: Option<String>) -> Self {
        Self {
            mark: Tone::Neutral,
            label,
            detail,
        }
    }
}

/// Where `library` stands in `model`.
pub fn library_status(model: &Model, library: &DependencyLibrary) -> LibraryStatus {
    match library.kind {
        // The library's cells are read from the project file with the rest
        // of the project; a path it records is not where they come from.
        DependencyKind::Design => match model.library_manager.get_library(&library.name) {
            Some(_) => LibraryStatus::neutral("In project", None),
            None => not_in_project(),
        },
        DependencyKind::Model => model
            .model_library_manager
            .get_library(&library.name)
            .map_or_else(not_in_project, |found| model_library_status(model, found)),
    }
}

fn not_in_project() -> LibraryStatus {
    LibraryStatus::neutral("Not in project", None)
}

fn model_library_status(model: &Model, library: &ModelLibrary) -> LibraryStatus {
    let authority = library.source_authority;
    if !authority.has_execution_source() {
        // Its models ship with RSpice and simulate from there: there is no
        // file to find, so there is nothing that can be missing.
        return LibraryStatus::neutral("Built in", Some("Ships with RSpice".to_owned()));
    }
    let files = library.source_closure.len();
    if library.root_path.is_none() || files == 0 {
        return LibraryStatus {
            mark: Tone::Error,
            label: "No model files",
            detail: Some("No model file has been read for this library".to_owned()),
        };
    }
    if authority.uses_retained_bytes() {
        let kept: BTreeSet<_> = library
            .source_contents
            .iter()
            .map(|content| &content.path)
            .collect();
        let missing = library
            .source_closure
            .iter()
            .filter(|pin| !kept.contains(&pin.path))
            .count();
        if missing > 0 {
            return LibraryStatus {
                mark: Tone::Error,
                label: "Incomplete",
                detail: Some(format!(
                    "{missing} of {} not kept in the project",
                    source_files(files)
                )),
            };
        }
    }
    let findings = model.source_drift_findings(&library.name);
    let more = |count: usize| match count {
        0 | 1 => String::new(),
        count => format!(" \u{00b7} and {} more", count - 1),
    };
    let unread: Vec<_> = findings
        .iter()
        .filter(|finding| finding.on_disk.is_none())
        .collect();
    if let Some(first) = unread.first() {
        return LibraryStatus {
            mark: Tone::Error,
            label: "Not found",
            detail: Some(format!(
                "{} cannot be read{}",
                file_name(&first.path),
                more(unread.len())
            )),
        };
    }
    if let Some(first) = findings.first() {
        return LibraryStatus {
            mark: Tone::Warn,
            label: "Changed",
            detail: Some(format!(
                "{} \u{00b7} attached {} \u{00b7} now {}{}",
                file_name(&first.path),
                first.pinned,
                first.on_disk.as_deref().unwrap_or(UNSET),
                more(findings.len())
            )),
        };
    }
    let label = if matches!(authority, ModelSourceAuthority::External) {
        "Found"
    } else {
        "In project"
    };
    LibraryStatus::neutral(label, Some(source_files(files)))
}

/// The file a finding is about, by name: the library's source row beside it
/// gives the whole path.
fn file_name(path: &Path) -> String {
    path.file_name().map_or_else(
        || shown_path(path),
        |name| name.to_string_lossy().into_owned(),
    )
}

/// A path as the reader knows it. Canonicalising a Windows drive path adds
/// the verbatim `\\?\` prefix, which is no part of the name they gave it.
fn shown_path(path: &Path) -> String {
    let text = path.display().to_string();
    let Some(rest) = text.strip_prefix(r"\\?\") else {
        return text;
    };
    if let Some(share) = rest.strip_prefix(r"UNC\") {
        return format!(r"\\{share}");
    }
    let drive = rest.as_bytes();
    if drive.first().is_some_and(u8::is_ascii_alphabetic) && drive.get(1) == Some(&b':') {
        return rest.to_owned();
    }
    text
}

fn source_files(count: usize) -> String {
    format!("{count} source file{}", if count == 1 { "" } else { "s" })
}

/// What every status reads: the two catalogues and the drift report. Each is
/// read by its stamp or by what it found, never by walking a catalogue, so
/// asking for the key costs an idle frame nothing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyStatusKey {
    design: u64,
    models: [u64; 3],
    drift: (Option<ModelSourceDriftScope>, u64),
}

pub fn dependency_status_key(model: &Model) -> DependencyStatusKey {
    // The report holds only the libraries that drifted, so hashing what it
    // found is as cheap as the report is short, and catches a rescan that
    // found something new under the same scope.
    let mut findings = DefaultHasher::new();
    for (library, found) in &model.source_drift.libraries {
        library.hash(&mut findings);
        for finding in found {
            finding.path.hash(&mut findings);
            finding.pinned.hash(&mut findings);
            finding.on_disk.hash(&mut findings);
        }
    }
    DependencyStatusKey {
        design: model.library_manager.content_stamp(),
        models: model.model_library_manager.content_stamp(),
        drift: (model.source_drift.scanned, findings.finish()),
    }
}

/// One row of the libraries the project depends on, as the rail, the table
/// and the inspector show it.
#[derive(Debug, Clone)]
pub struct DependencyRow {
    pub library: DependencyLibrary,
    pub version: String,
    pub source: String,
    pub digest: String,
    pub status: LibraryStatus,
}

impl DependencyRow {
    /// Whether the reader's filter, trimmed and lowercased, finds this row
    /// in anything it shows.
    pub fn matches(&self, filter: &str) -> bool {
        filter.is_empty()
            || [
                self.library.name.as_str(),
                self.library.kind.label(),
                self.version.as_str(),
                self.source.as_str(),
                self.digest.as_str(),
                self.status.label,
            ]
            .iter()
            .any(|value| value.to_lowercase().contains(filter))
    }
}

/// Every library the project depends on, design libraries first, each kind
/// by name without regard to case. Built when a status input changes, so an
/// idle frame sorts nothing.
pub fn dependency_rows(ctx: &egui::Context, model: &Model) -> Arc<[DependencyRow]> {
    derived(
        ctx,
        "dependencies.rows",
        dependency_status_key(model),
        || build_rows(model).into(),
    )
}

/// The rows `filter` lets through, by index into [`dependency_rows`], chosen
/// when the filter or the rows change. The rail and the table both read
/// this, so they list the same libraries.
pub fn visible_dependency_rows(
    ctx: &egui::Context,
    model: &Model,
    filter: &str,
) -> (Arc<[DependencyRow]>, Arc<[usize]>) {
    let rows = dependency_rows(ctx, model);
    let filter = filter.trim().to_lowercase();
    let visible = derived(
        ctx,
        "dependencies.visible",
        (dependency_status_key(model), filter.clone()),
        || {
            rows.iter()
                .enumerate()
                .filter(|(_, row)| row.matches(&filter))
                .map(|(index, _)| index)
                .collect::<Arc<[usize]>>()
        },
    );
    (rows, visible)
}

fn build_rows(model: &Model) -> Vec<DependencyRow> {
    let design = model
        .library_manager
        .catalog()
        .libraries_sorted()
        .into_iter()
        .map(|library| {
            let key = DependencyLibrary::design(&library.name);
            DependencyRow {
                status: library_status(model, &key),
                library: key,
                version: nonempty_or_unset(&library.technology),
                source: library
                    .path
                    .as_deref()
                    .map_or_else(|| "Project file".to_owned(), shown_path),
                digest: UNSET.to_owned(),
            }
        });
    let models = model
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .map(|library| {
            let key = DependencyLibrary::model(&library.name);
            DependencyRow {
                status: library_status(model, &key),
                library: key,
                version: nonempty_or_unset(&library.version),
                source: library
                    .root_path
                    .as_deref()
                    .map_or_else(|| UNSET.to_owned(), shown_path),
                digest: library
                    .source_closure
                    .iter()
                    .find(|pin| library.root_path.as_ref() == Some(&pin.path))
                    .or_else(|| library.source_closure.first())
                    .map_or_else(
                        || UNSET.to_owned(),
                        |pin| pin.digest.to_string().chars().take(8).collect(),
                    ),
            }
        });
    let mut rows: Vec<_> = design.chain(models).collect();
    rows.sort_by(|left, right| {
        left.library
            .kind
            .cmp(&right.library.kind)
            .then_with(|| {
                left.library
                    .name
                    .to_lowercase()
                    .cmp(&right.library.name.to_lowercase())
            })
            .then_with(|| left.library.name.cmp(&right.library.name))
    });
    rows
}

fn nonempty_or_unset(value: &str) -> String {
    if value.trim().is_empty() {
        UNSET.to_owned()
    } else {
        value.to_owned()
    }
}

/// Bring the drift report up to what the model catalogue holds now, when
/// either it or the project's revision has moved since it was last asked.
/// Scanning rehashes every pinned source, which is why it is asked here, on
/// a change, and never inside a status.
pub fn refresh_source_drift(ctx: &egui::Context, model: &mut Model) {
    let key = (
        model.model_library_manager.content_stamp(),
        model.workspace.content.project.revision().get(),
    );
    derived(ctx, "dependencies.drift", key, || {
        if model.source_drift_needs_scan() {
            model.scan_source_drift();
            // Whatever drew a status before this frame's scan draws it again.
            ctx.request_repaint();
        }
    });
}

/// What the Dependencies page's rail and inspector have selected: kept while
/// it names a library of the project, else the first row the filter shows.
pub fn settle_selection(
    rows: &[DependencyRow],
    visible: &[usize],
    selection: &mut Option<DependencyLibrary>,
) {
    let names_a_row = selection
        .as_ref()
        .is_some_and(|selected| rows.iter().any(|row| &row.library == selected));
    if !names_a_row {
        *selection = visible.first().map(|&index| rows[index].library.clone());
    }
}

/// The technology the project's runs take their model sources from, as the
/// Dependencies page and its inspector state it. A project with none is
/// ordinary unless the simulation setup needs one, which is when a run would
/// be refused.
pub fn technology_status(model: &Model) -> LibraryStatus {
    if model
        .workspace
        .content
        .project
        .technology_binding()
        .is_none()
    {
        return match model.technology_inputs().technology_demand().block_reason() {
            Some(reason) => LibraryStatus {
                mark: Tone::Error,
                label: "Not attached",
                detail: Some(reason),
            },
            None => LibraryStatus::neutral("Not attached", None),
        };
    }
    // As last proven: proving it hashes the attached library's sources. A
    // binding that no longer resolves refuses every run.
    match model.project_technology_verdict() {
        Ok(()) => LibraryStatus::neutral("Attached", None),
        Err(error) => LibraryStatus {
            mark: Tone::Error,
            label: "Not resolved",
            detail: Some(error),
        },
    }
}

#[cfg(test)]
mod tests;
