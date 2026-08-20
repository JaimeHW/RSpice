//! The symbol registry rows the Symbols page paints.
//!
//! A row states what a symbol cellview's typed contract declares and whether
//! the artwork agrees with it, and both halves live in the view's metadata as
//! JSON. Deriving the rows therefore deserialized every symbol cellview in
//! every design library twice — the model-bound definition and the symbol
//! document — before the table virtualized down to the dozen rows on screen. A
//! technology library ships a symbol for every device it offers, so that is a
//! four-figure number of deserializations on every frame the page paints.
//!
//! The deserializations are cached behind [`symbol_corpus_revision`], which
//! hashes exactly what they read. See that function for why the key is content
//! rather than the catalog's revision counter alone: a stale row here does not
//! merely look old, it prints a pin-mismatch verdict about artwork that is no
//! longer there, which is worse than the cost it saves.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash as _, Hasher as _};

use super::*;

#[derive(Clone)]
pub(super) struct SymbolRow {
    pub(super) reference: CellViewRef,
    pub(super) authority: SymbolRowAuthority,
    pub(super) family: String,
    pub(super) pins: Vec<String>,
    pub(super) form: String,
    pub(super) template: String,
    pub(super) status: String,
    pub(super) definition: Option<ModelBoundSymbolDefinition>,
    pub(super) diagnostics: Vec<String>,
}

#[derive(Clone)]
pub(super) enum SymbolRowAuthority {
    DesignLibrary {
        read_only: bool,
    },
    SignedTechnology {
        technology_name: String,
        revision: String,
        manifest_digest: crate::product::ContentDigest,
        archive_digest: crate::product::ContentDigest,
    },
}

impl SymbolRow {
    pub(super) fn read_only(&self) -> bool {
        match &self.authority {
            SymbolRowAuthority::DesignLibrary { read_only } => *read_only,
            SymbolRowAuthority::SignedTechnology { .. } => true,
        }
    }
}

/// One design-library symbol cellview, as its metadata deserialized.
///
/// This is the entire product of the two parses and the only thing the cache
/// holds. `family` is deliberately absent: a symbol that declares no binding
/// falls back to a model of the same name in the model corpus, and the corpus
/// is not part of the key.
#[derive(Clone)]
struct ParsedSymbolView {
    reference: CellViewRef,
    read_only: bool,
    /// The family the symbol itself declares, from its typed netlist binding or
    /// from its cell's metadata. `None` hands the question to the corpus.
    declared_family: Option<String>,
    pins: Vec<String>,
    form: String,
    template: String,
    status: String,
    definition: Option<ModelBoundSymbolDefinition>,
    diagnostics: Vec<String>,
}

#[derive(Clone)]
struct SymbolContractCache {
    corpus_revision: u64,
    views: Vec<ParsedSymbolView>,
}

/// One symbol cellview, with the identity the parse needs around it.
struct SymbolViewSite<'a> {
    library: &'a str,
    read_only: bool,
    cell: &'a crate::state::Cell,
    view: &'a crate::state::View,
}

pub(super) fn symbol_rows(ui: &Ui, app: &ManagerRenderContext<'_>) -> Vec<SymbolRow> {
    let mut rows = design_library_rows(ui, app);
    if let Ok(Some(package)) = app.state.project_signed_technology_package() {
        for definition in package.symbol_definitions() {
            let pins = definition
                .pins
                .iter()
                .map(|pin| pin.name.clone())
                .collect::<Vec<_>>();
            let family = definition
                .netlist
                .model
                .as_ref()
                .map(|model| format!("{}/{}", model.library, model.model))
                .unwrap_or_else(|| "invalid signed binding".to_owned());
            rows.push(SymbolRow {
                reference: CellViewRef::new(
                    &definition.identity.library,
                    &definition.identity.cell,
                    "symbol",
                ),
                authority: SymbolRowAuthority::SignedTechnology {
                    technology_name: package.manifest().technology_name.clone(),
                    revision: package.manifest().revision.clone(),
                    manifest_digest: package.manifest_digest(),
                    archive_digest: package.archive_digest(),
                },
                family,
                pins,
                form: super::super::symbol_parameter_form_label(definition),
                template: definition.netlist.template.clone(),
                status: "read-only".to_owned(),
                definition: Some(definition.clone()),
                diagnostics: Vec::new(),
            });
        }
    }
    rows.sort_by(|left, right| {
        left.reference
            .library
            .to_ascii_lowercase()
            .cmp(&right.reference.library.to_ascii_lowercase())
            .then_with(|| {
                left.reference
                    .cell
                    .to_ascii_lowercase()
                    .cmp(&right.reference.cell.to_ascii_lowercase())
            })
    });
    rows
}

/// The rows the project's own design libraries contribute.
///
/// The corpus fallback is resolved here rather than in the cached projection,
/// and only for the symbols that need it — a library whose symbols all declare
/// their binding never touches the model corpus at all.
fn design_library_rows(ui: &Ui, app: &ManagerRenderContext<'_>) -> Vec<SymbolRow> {
    let parsed = parsed_symbol_views(ui, app);
    let mut models_by_cell = None;
    let mut rows = Vec::with_capacity(parsed.len());
    for view in parsed {
        let family = match view.declared_family {
            Some(family) => family,
            None => models_by_cell
                .get_or_insert_with(|| model_names_by_cell(app))
                .get(&view.reference.cell.to_ascii_lowercase())
                .cloned()
                .unwrap_or_else(|| "unbound".to_owned()),
        };
        rows.push(SymbolRow {
            reference: view.reference,
            authority: SymbolRowAuthority::DesignLibrary {
                read_only: view.read_only,
            },
            family,
            pins: view.pins,
            form: view.form,
            template: view.template,
            status: view.status,
            definition: view.definition,
            diagnostics: view.diagnostics,
        });
    }
    rows
}

/// Every design-library symbol cellview's contract, deserialized at most once
/// per change to the symbol corpus.
fn parsed_symbol_views(ui: &Ui, app: &ManagerRenderContext<'_>) -> Vec<ParsedSymbolView> {
    let mut hasher = DefaultHasher::new();
    let mut views = Vec::new();
    symbol_corpus_revision(app, &mut hasher, &mut views);
    let corpus_revision = hasher.finish();

    let cache_id = egui::Id::new("models-symbol-contracts");
    if let Some(cached) = ui
        .ctx()
        .data(|data| data.get_temp::<SymbolContractCache>(cache_id))
        && cached.corpus_revision == corpus_revision
    {
        return cached.views;
    }

    let parsed = views.iter().map(parse_symbol_view).collect::<Vec<_>>();
    ui.ctx().data_mut(|data| {
        data.insert_temp(
            cache_id,
            SymbolContractCache {
                corpus_revision,
                views: parsed.clone(),
            },
        );
    });
    parsed
}

/// Hash everything the parses read, and collect the views they will read it
/// from, in one walk of the design libraries.
///
/// The key is content, not [`crate::state::LibraryManager::revision`] alone.
/// That counter advances on every catalog mutation within a session, but the
/// whole `LibraryManager` is *replaced* wholesale when a project is opened,
/// when a recovery comparison is accepted, and when design history restores a
/// candidate — and the replacement carries whatever counter it was serialized
/// with, which may be one this session has already seen. Keying on the counter
/// alone would then paint the previous project's pin-mismatch verdicts against
/// this project's artwork. The counter is folded in anyway, so an ordinary edit
/// changes the key even where content hashing could not tell two states apart.
///
/// `show_read_only` is in the key because it decides which libraries
/// `libraries_sorted` returns at all, and it is a plain field that no mutation
/// counter watches.
fn symbol_corpus_revision<'a>(
    app: &'a ManagerRenderContext<'_>,
    hasher: &mut DefaultHasher,
    views: &mut Vec<SymbolViewSite<'a>>,
) {
    app.state.library_manager.revision().hash(hasher);
    app.state.library_manager.show_read_only.hash(hasher);
    for library in app.state.library_manager.libraries_sorted() {
        library.name.hash(hasher);
        library.read_only.hash(hasher);
        for cell in library.cells_sorted() {
            cell.name.hash(hasher);
            // The cell's metadata carries the declared family a symbol with no
            // typed binding falls back to, so it is an input to the projection.
            hash_metadata(&cell.metadata, hasher);
            for view in cell
                .views_sorted()
                .into_iter()
                .filter(|view| view.view_type == ViewType::Symbol)
            {
                view.name.hash(hasher);
                hash_metadata(&view.metadata, hasher);
                views.push(SymbolViewSite {
                    library: &library.name,
                    read_only: library.read_only,
                    cell,
                    view,
                });
            }
        }
    }
}

/// Fold a metadata map into the hash without depending on its iteration order.
///
/// A `HashMap` iterates in an order that belongs to the instance rather than to
/// the content, so hashing its entries in iteration order would produce a
/// different key for an unchanged corpus and defeat the cache entirely.
fn hash_metadata(metadata: &std::collections::HashMap<String, String>, hasher: &mut DefaultHasher) {
    let mut folded_xor = 0_u64;
    let mut folded_sum = 0_u64;
    for (key, value) in metadata {
        let mut entry = DefaultHasher::new();
        key.hash(&mut entry);
        value.hash(&mut entry);
        let entry = entry.finish();
        folded_xor ^= entry;
        folded_sum = folded_sum.wrapping_add(entry.rotate_left(17));
    }
    metadata.len().hash(hasher);
    folded_xor.hash(hasher);
    folded_sum.hash(hasher);
}

/// Deserialize one symbol cellview's typed contract and its artwork, and say
/// where the two disagree.
fn parse_symbol_view(site: &SymbolViewSite<'_>) -> ParsedSymbolView {
    let &SymbolViewSite {
        library,
        read_only,
        cell,
        view,
    } = site;
    let definition_result = ModelBoundSymbolDefinition::load_from_view(view);
    let document_result = SymbolDocument::load_from_view(view);
    let definition = definition_result.as_ref().ok().and_then(Clone::clone);
    let pins = document_result
        .as_ref()
        .map(|document| {
            document
                .pins
                .iter()
                .map(|pin| pin.name.clone())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let declared_family = definition
        .as_ref()
        .and_then(|definition| definition.netlist.model.as_ref())
        .map(|model| format!("{}/{}", model.library, model.model))
        .or_else(|| {
            super::super::metadata_value(
                [&cell.metadata],
                &["model.family", "model_family", "model", "model.name"],
            )
        });
    let form = definition
        .as_ref()
        .map(super::super::symbol_parameter_form_label)
        .unwrap_or_else(|| "legacy / none".to_owned());
    let template = definition
        .as_ref()
        .map(|definition| definition.netlist.template.clone())
        .filter(|template| !template.trim().is_empty())
        .unwrap_or_else(|| "not defined".to_owned());
    let mut diagnostics = Vec::new();
    if let Err(error) = definition_result {
        diagnostics.push(format!("Definition metadata: {error}"));
    }
    if let Err(error) = &document_result {
        diagnostics.push(format!("Symbol document: {error}"));
    }
    if let (Some(definition), Ok(document)) = (&definition, &document_result) {
        let expected = definition
            .pins
            .iter()
            .map(|pin| pin.name.to_ascii_lowercase())
            .collect::<Vec<_>>();
        let observed = document
            .pins
            .iter()
            .map(|pin| pin.name.to_ascii_lowercase())
            .collect::<Vec<_>>();
        if expected != observed {
            diagnostics.push(format!(
                "Blocking pin mismatch: provider {:?}, symbol {:?}",
                expected, observed
            ));
        }
        if let Err(error) = definition.validate() {
            diagnostics.push(format!("Executable contract: {error}"));
        }
    } else if definition.is_none() {
        diagnostics.push("Legacy symbol has no typed model/netlist/form contract.".to_owned());
    }
    let status = if diagnostics
        .iter()
        .any(|diagnostic| diagnostic.contains("Blocking"))
    {
        "pin mismatch"
    } else if !diagnostics.is_empty() {
        "review"
    } else if read_only {
        "read-only"
    } else {
        "bound"
    }
    .to_owned();
    ParsedSymbolView {
        reference: CellViewRef::new(library, &cell.name, &view.name),
        read_only,
        declared_family,
        pins,
        form,
        template,
        status,
        definition,
        diagnostics,
    }
}

/// Every model name in the corpus, folded for lookup by a symbol's cell name.
///
/// A symbol with no declared binding falls back to a model that shares its
/// cell name, which used to mean walking the whole corpus per symbol — the
/// product of the symbol registry and the model corpus, on every frame.
/// Insertion follows library order, so the model that wins a duplicated name
/// is the same one the linear search found.
fn model_names_by_cell(app: &ManagerRenderContext<'_>) -> BTreeMap<String, String> {
    let mut names = BTreeMap::new();
    for library in app.state.model_library_manager.libraries_sorted() {
        for model in library.models.values() {
            names
                .entry(model.name.to_ascii_lowercase())
                .or_insert_with(|| model.name.clone());
        }
    }
    names
}
