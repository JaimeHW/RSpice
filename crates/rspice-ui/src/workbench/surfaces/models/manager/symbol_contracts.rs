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
//!
//! # What the detail cards are derived from
//!
//! The registry row is only half of what this file states. The other half is
//! the tables the selected symbol's detail draws — its pins against the
//! provider's ports, its instance-line template token by token, and the
//! instances placed from it — and each is derived here, from the stored
//! definition, rather than assembled at the painter. A painter that assembled
//! them would be the one place a reader could not check them against the rule
//! that actually runs on save.

/// Re-exported so the Symbols page's own tests can seed a registry from the
/// same fixture this file's tests check the derivations against; two fixtures
/// for one registry is two registries.
///
/// The module itself stays private and stays at the bottom of the file:
/// `source_guard::production_half` locates the shipped half of a self-scanning
/// source by that exact declaration, and the control ratchet reads this file
/// through it. A `pub(super) mod tests` panics the guard, and one declared at
/// the top leaves the ratchet scanning the module doc comment and nothing
/// else — a guard that scans nothing passes forever.
#[cfg(test)]
pub(super) use tests::seed_symbol_registry;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash as _, Hasher as _};

use crate::state::{PortSpec, SymbolPinDefinition, SymbolSourceContract};

use super::*;

#[derive(Clone)]
pub(super) struct SymbolRow {
    pub(super) reference: CellViewRef,
    pub(super) authority: SymbolRowAuthority,
    pub(super) family: String,
    pub(super) pins: Vec<String>,
    pub(super) form: String,
    pub(super) template: String,
    pub(super) status: SymbolStatus,
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

/// What a registry row's STATUS column says, and how loudly it says it.
///
/// The tone belongs to the verdict, not to the painter. A pin mismatch is a
/// symbol whose artwork and provider disagree — placing it emits a netlist
/// line with the terminals in the wrong order — and a review is a contract
/// that would not parse or would not validate; both are work. `bound` and
/// `read-only` are settled states with nothing to do about them. The painter
/// used to pick the colour by string-matching the label, which is why only
/// `pin mismatch` was ever coloured: nothing there said `review` was a finding
/// too.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum SymbolStatus {
    /// The provider's terminals and the drawn artwork disagree.
    PinMismatch,
    /// The typed contract did not parse, or did not validate.
    Review,
    /// Bound, executable, and writable from this project.
    Bound,
    /// Bound, and owned by a library this project may not write.
    ReadOnly,
}

impl SymbolStatus {
    pub(super) const fn label(self) -> &'static str {
        match self {
            Self::PinMismatch => "pin mismatch",
            Self::Review => "review",
            Self::Bound => "bound",
            Self::ReadOnly => "read-only",
        }
    }

    /// Whether the row is one a reader still has work to do about — the count
    /// the registry footer reports.
    pub(super) const fn needs_attention(self) -> bool {
        matches!(self, Self::PinMismatch | Self::Review)
    }
}

/// Which of the two corpora a row belongs to.
///
/// The registry browses both: the project's own symbol libraries, which it may
/// write, and the signed technology package's, which it may not. They are one
/// table because a reader compares across them, and grouped within it because
/// the difference decides whether any action on the row is available at all.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub(super) enum SymbolCorpus {
    Project,
    SignedTechnology,
}

impl SymbolRow {
    pub(super) fn read_only(&self) -> bool {
        match &self.authority {
            SymbolRowAuthority::DesignLibrary { read_only } => *read_only,
            SymbolRowAuthority::SignedTechnology { .. } => true,
        }
    }

    pub(super) fn corpus(&self) -> SymbolCorpus {
        match &self.authority {
            SymbolRowAuthority::DesignLibrary { .. } => SymbolCorpus::Project,
            SymbolRowAuthority::SignedTechnology { .. } => SymbolCorpus::SignedTechnology,
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
    status: SymbolStatus,
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
                status: SymbolStatus::ReadOnly,
                definition: Some(definition.clone()),
                diagnostics: Vec::new(),
            });
        }
    }
    // Corpus first, so the table is the project's own forms followed by the
    // technology's — the order the group band in the registry announces. The
    // sort is stable and its comparator is total on (corpus, library, cell),
    // so a row's position within its group does not move between frames.
    rows.sort_by(|left, right| {
        left.corpus()
            .cmp(&right.corpus())
            .then_with(|| {
                left.reference
                    .library
                    .to_ascii_lowercase()
                    .cmp(&right.reference.library.to_ascii_lowercase())
            })
            .then_with(|| {
                left.reference
                    .cell
                    .to_ascii_lowercase()
                    .cmp(&right.reference.cell.to_ascii_lowercase())
            })
    });
    rows
}

/// How many leading rows belong to the project's own libraries.
///
/// [`symbol_rows`] sorts by corpus first, so the project's rows are a prefix
/// and this one number is the whole grouping the registry needs.
pub(super) fn project_row_count(rows: &[SymbolRow]) -> usize {
    rows.iter()
        .take_while(|row| row.corpus() == SymbolCorpus::Project)
        .count()
}

/// The band that separates the two corpora, when there are two to separate.
///
/// It names the technology, the revision the project pinned, how many symbols
/// it contributes and that none of them may be edited here — every fact taken
/// from the signed manifest the rows under it were read from.
pub(super) fn technology_group_band(rows: &[SymbolRow]) -> Option<String> {
    let project = project_row_count(rows);
    let technology = rows.len() - project;
    if project == 0 || technology == 0 {
        return None;
    }
    let SymbolRowAuthority::SignedTechnology {
        technology_name,
        revision,
        ..
    } = &rows[project].authority
    else {
        return None;
    };
    Some(format!(
        "{technology_name} {revision} · {technology} TECHNOLOGY SYMBOL{} · READ-ONLY",
        if technology == 1 { "" } else { "S" }
    ))
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
            crate::state::workspace::metadata_value(
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
        SymbolStatus::PinMismatch
    } else if !diagnostics.is_empty() {
        SymbolStatus::Review
    } else if read_only {
        SymbolStatus::ReadOnly
    } else {
        SymbolStatus::Bound
    };
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

/// One line of the pin-contract table: what the symbol declares at a position
/// in the netlist order, and what the provider declares at the same position.
#[derive(Clone, PartialEq, Eq, Debug)]
pub(super) struct PinContractRow {
    /// One-based positional netlist order — the position itself, which is what
    /// a SPICE card is indexed by.
    pub(super) order: usize,
    /// The symbol's pin at this position, when it declares one.
    pub(super) pin: Option<String>,
    /// That pin's declared electrical class and direction. Empty where the
    /// symbol declares no pin here.
    pub(super) electrical: String,
    /// The provider's port at this position, when the provider declares one.
    pub(super) provider_port: Option<String>,
    pub(super) check: PinCheck,
}

/// Whether one position of the pin contract agrees with the provider.
#[derive(Clone, PartialEq, Eq, Debug)]
pub(super) enum PinCheck {
    /// Same name, same direction — the only state the save path accepts.
    Aligned,
    /// The two disagree, spelled the way the save path spells the refusal.
    Mismatched(String),
    /// One side declares a terminal at this position and the other does not.
    Absent(&'static str),
}

impl PinCheck {
    pub(super) fn label(&self) -> &str {
        match self {
            Self::Aligned => "aligned",
            Self::Mismatched(reason) => reason,
            Self::Absent(reason) => reason,
        }
    }

    pub(super) const fn is_aligned(&self) -> bool {
        matches!(self, Self::Aligned)
    }
}

/// What the pin-contract card can say about a symbol.
pub(super) enum PinContract {
    /// The provider declares ports, so every position has a verdict.
    Checked(Vec<PinContractRow>),
    /// There is a typed definition, but nothing to check its pins against, and
    /// the sentence saying why.
    NoProviderPorts(&'static str),
    /// A legacy symbol: drawn pin names, and no typed contract at all.
    Legacy(Vec<String>),
}

/// The selected symbol's pins, checked against the provider's ports.
///
/// The rule is the one `validate_source` runs on every save: the provider's
/// ports zipped with the pins in netlist order, agreeing when the names match
/// case-insensitively and the directions are equal, and both sides declaring
/// the same number of terminals. It is re-expressed here rather than called
/// because the save path's comparison is private to the symbol-definition
/// module and refuses a *definition* rather than a position — a card that
/// could only say "this symbol is wrong somewhere" is the thing this replaces.
/// What keeps the two from drifting is a test:
/// `the_pin_card_and_the_save_path_agree_on_every_mutation` mutates a valid
/// definition every way the rule rejects and requires this card's verdict and
/// `ModelBoundSymbolDefinition::validate` to reach the same conclusion.
pub(super) fn pin_contract(row: &SymbolRow) -> PinContract {
    let Some(definition) = &row.definition else {
        return PinContract::Legacy(row.pins.clone());
    };
    let ports: &[PortSpec] = match &definition.source {
        SymbolSourceContract::Model { ports, .. }
        | SymbolSourceContract::ExistingSchematicPins { ports, .. } => ports,
        SymbolSourceContract::BlankExplicitContract => {
            return PinContract::NoProviderPorts(
                "This definition is an explicitly unbound review contract. It names no \
                 provider, so its pins have no ports to be checked against.",
            );
        }
    };
    let mut ordered = definition.pins.iter().collect::<Vec<_>>();
    ordered.sort_by_key(|pin| pin.order);
    // Every position either side declares gets a line, so a provider with one
    // terminal more than the symbol is a visible extra row rather than a
    // silently dropped one.
    let positions = ports.len().max(ordered.len());
    let rows = (0..positions)
        .map(|index| {
            let port = ports.get(index);
            let pin = ordered.get(index).copied();
            PinContractRow {
                order: index + 1,
                pin: pin.map(|pin| pin.name.clone()),
                electrical: pin.map(pin_electrical).unwrap_or_default(),
                provider_port: port
                    .map(|port| format!("{} {}", port.name, port.direction.keyword())),
                check: pin_check(index, port, pin),
            }
        })
        .collect();
    PinContract::Checked(rows)
}

/// A pin's declared class and direction, which RSpice carries separately:
/// `analog` says what kind of terminal it is, `in` says which way it drives.
fn pin_electrical(pin: &SymbolPinDefinition) -> String {
    format!(
        "{} · {}",
        pin.electrical_type.label(),
        pin.direction.keyword()
    )
}

/// One position's verdict, in the words `compare_source_ports` refuses with.
fn pin_check(index: usize, port: Option<&PortSpec>, pin: Option<&SymbolPinDefinition>) -> PinCheck {
    match (port, pin) {
        (Some(port), Some(pin)) => {
            if port.name.eq_ignore_ascii_case(&pin.name) && port.direction == pin.direction {
                PinCheck::Aligned
            } else {
                PinCheck::Mismatched(format!(
                    "terminal {} is `{} {}` in the source and `{} {}` in the symbol",
                    index + 1,
                    port.name,
                    port.direction.keyword(),
                    pin.name,
                    pin.direction.keyword()
                ))
            }
        }
        (Some(_), None) => PinCheck::Absent("the symbol declares no pin here"),
        (None, Some(_)) => PinCheck::Absent("the provider declares no port here"),
        // Unreachable: the row count is the larger of the two sides.
        (None, None) => PinCheck::Absent("neither side declares a terminal here"),
    }
}

/// One token of the validated instance-line template, and where its value
/// comes from when the netlist is written.
#[derive(Clone, PartialEq, Eq, Debug)]
pub(super) struct TemplateToken {
    /// The token exactly as the template spells it.
    pub(super) token: String,
    /// What supplies the substitution.
    pub(super) source: &'static str,
    /// The value itself where the contract fixes it, and what kind of value it
    /// is where the placed instance decides.
    pub(super) value: String,
    /// Who may change it.
    pub(super) owner: &'static str,
    /// A token nothing resolves blocks netlisting rather than emitting an
    /// empty field, so the card says which one.
    pub(super) resolves: bool,
}

/// What the netlist-template card can say about a symbol.
pub(super) enum NetlistTemplate {
    /// The stored template parses as the instance-line grammar; these are its
    /// tokens, in emission order.
    Tokens(Vec<TemplateToken>),
    /// The stored template is not an instance line, with the refusal the save
    /// path gives for it.
    Invalid(String),
}

/// The selected symbol's template, token by token.
///
/// The grammar is [`crate::state::validate_library_netlist_template`]'s, and it
/// admits exactly four token kinds — a bound reference, the ordered nodes, the
/// model, and an optional parameter tail — so a token table is a complete
/// account of the emitted line rather than a sample of it.
///
/// The template is re-validated here rather than trusted. This page shows
/// definitions that failed to validate, on purpose, and a token table derived
/// from a template the netlist writer would refuse describes a line that never
/// gets emitted.
pub(super) fn netlist_template(definition: &ModelBoundSymbolDefinition) -> NetlistTemplate {
    let netlist = &definition.netlist;
    if let Err(reason) = crate::state::validate_library_netlist_template(&netlist.template) {
        return NetlistTemplate::Invalid(reason);
    }
    let mut ordered = definition.pins.iter().collect::<Vec<_>>();
    ordered.sort_by_key(|pin| pin.order);
    let nodes = ordered
        .iter()
        .map(|pin| pin.name.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    let tokens = netlist
        .template
        .split_ascii_whitespace()
        .map(|token| match token {
            "{nodes}" => TemplateToken {
                token: token.to_owned(),
                source: "pin contract",
                value: if nodes.is_empty() {
                    "no pins declared".to_owned()
                } else {
                    nodes.clone()
                },
                // The order is the symbol's; the nets substituted into it
                // belong to the sheet the instance is placed on.
                owner: "schematic",
                resolves: !ordered.is_empty(),
            },
            "{model}" => TemplateToken {
                token: token.to_owned(),
                source: "source contract",
                value: netlist
                    .model
                    .as_ref()
                    .map_or_else(|| "unbound".to_owned(), |model| model.model.clone()),
                // `validate_netlist` requires the executable model to equal the
                // source contract's exactly, so no instance can override it —
                // which is why this owner is not "instance" as it would be in a
                // product where the model is a per-instance property.
                owner: "symbol definition",
                resolves: netlist.model.is_some(),
            },
            "{params}" => TemplateToken {
                token: token.to_owned(),
                source: "parameter form",
                value: if netlist.parameter_order.is_empty() {
                    "no emitted fields".to_owned()
                } else {
                    netlist.parameter_order.join(" ")
                },
                owner: "parameter form",
                // An empty parameter tail is the validated behaviour of a form
                // with no emitted fields, not a token that resolves to nothing.
                resolves: true,
            },
            // The first token, which the grammar requires to be `{ref}` or an
            // ASCII device prefix followed by `{name}`.
            _ => TemplateToken {
                token: token.to_owned(),
                source: "instance",
                value: format!("reference designator · prefix {}", netlist.device_prefix),
                owner: "instance",
                resolves: !netlist.device_prefix.trim().is_empty(),
            },
        })
        .collect();
    NetlistTemplate::Tokens(tokens)
}

/// One placed instance of the selected symbol.
#[derive(Clone, PartialEq, Eq, Debug)]
pub(super) struct PlacedInstance {
    pub(super) designator: String,
    /// The sheet the instance sits on, and where on it.
    pub(super) location: String,
    pub(super) binding: InstanceBinding,
}

/// How a placed instance is attached to the symbol.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum InstanceBinding {
    /// The instance names this library and cell outright, which is the binding
    /// a model-bound symbol places with.
    CellView,
    /// The instance names the model this symbol binds to without naming the
    /// symbol: placing a primitive device and typing the model name reaches
    /// the same provider by a different route.
    Model,
}

impl InstanceBinding {
    pub(super) const fn label(self) -> &'static str {
        match self {
            Self::CellView => "cellview binding",
            Self::Model => "model name",
        }
    }
}

/// The instances placed from the selected symbol, and how many there are.
#[derive(Clone, PartialEq, Eq, Debug)]
pub(super) struct PlacedInstances {
    pub(super) total: usize,
    /// The first `USAGE_ROWS` of them, which is what the card lists.
    pub(super) shown: Vec<PlacedInstance>,
}

#[derive(Clone)]
struct PlacedInstanceCache {
    key: u64,
    instances: PlacedInstances,
}

/// Which instances on the active sheet were placed from this symbol.
///
/// Derived for the *selected* symbol only. The same question asked of every
/// row is [`ConsumerIndex`], which resolves each instance's model against the
/// whole catalog — the product of the schematic and the corpus — and this
/// registry has six hundred rows at production scale. One symbol's answer is
/// one pass over the sheet, and that pass is cached behind a key made of
/// exactly what it reads, the way [`parsed_symbol_views`] is: the key hashes
/// borrowed fields and nothing else, while the miss allocates a parameter map
/// per component, so the key is strictly cheaper than the work it guards.
///
/// The active sheet is the whole scope. The card states that rather than
/// implying a hierarchy-wide count it did not compute.
pub(super) fn placed_instances(
    ui: &Ui,
    app: &ManagerRenderContext<'_>,
    row: &SymbolRow,
) -> PlacedInstances {
    let Some(schematic) = app.state.workspace.active_schematic() else {
        return PlacedInstances {
            total: 0,
            shown: Vec::new(),
        };
    };
    // `family` is `library/model`; an instance names the model alone.
    let family = row.family.rsplit('/').next().unwrap_or_default().to_owned();
    let sheet = app.state.workspace.active_view.display_path();
    let mut hasher = DefaultHasher::new();
    row.reference.library.hash(&mut hasher);
    row.reference.cell.hash(&mut hasher);
    family.hash(&mut hasher);
    sheet.hash(&mut hasher);
    for component in &schematic.components {
        component.id.hash(&mut hasher);
        component.name.hash(&mut hasher);
        component.value.hash(&mut hasher);
        component.params.hash(&mut hasher);
        component.pos.hash(&mut hasher);
        if let Some(cell) = &component.library_cell {
            cell.library.hash(&mut hasher);
            cell.cell.hash(&mut hasher);
        }
    }
    let key = hasher.finish();

    let cache_id = egui::Id::new("models-symbol-placed-instances");
    if let Some(cached) = ui
        .ctx()
        .data(|data| data.get_temp::<PlacedInstanceCache>(cache_id))
        && cached.key == key
    {
        return cached.instances;
    }

    let mut total = 0;
    let mut shown = Vec::new();
    for component in &schematic.components {
        let Some(binding) = instance_binding(component, row, &family) else {
            continue;
        };
        total += 1;
        if shown.len() < USAGE_ROWS {
            shown.push(PlacedInstance {
                designator: component.name.clone(),
                location: format!("{sheet} · ({}, {})", component.pos.x, component.pos.y),
                binding,
            });
        }
    }
    let instances = PlacedInstances { total, shown };
    ui.ctx().data_mut(|data| {
        data.insert_temp(
            cache_id,
            PlacedInstanceCache {
                key,
                instances: instances.clone(),
            },
        );
    });
    instances
}

/// Whether one placed component came from this symbol, and by which route.
///
/// The cellview binding is checked first and wins: an instance that names the
/// library and cell was placed from the symbol whatever its value field holds.
fn instance_binding(
    component: &crate::state::Component,
    row: &SymbolRow,
    family: &str,
) -> Option<InstanceBinding> {
    if let Some(cell) = &component.library_cell
        && cell.library.eq_ignore_ascii_case(&row.reference.library)
        && cell.cell.eq_ignore_ascii_case(&row.reference.cell)
    {
        return Some(InstanceBinding::CellView);
    }
    if family.is_empty() {
        return None;
    }
    explicit_component_model(component)
        .filter(|model| model.eq_ignore_ascii_case(family))
        .map(|_| InstanceBinding::Model)
}

#[cfg(test)]
mod tests;
