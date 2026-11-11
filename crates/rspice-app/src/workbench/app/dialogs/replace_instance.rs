//! Replace instance: swap the selected instance's master for another one,
//! reading first which pins and authored values the new master carries.

use std::collections::HashSet;

use egui::Context;

use crate::state::{
    Component, ComponentType, Point, Rotation, SchematicReplacementAuthority,
    SchematicReplacementPreview, SchematicReplacementSourceSpec, SchematicReplacementTargetSpec,
    SchematicReplacementTerminal, SymbolResolver,
};
use crate::ui::widgets::{CommandForm, DialogChoice, TableColumn};
use rspice_design::hierarchy::HierarchySource;
use rspice_simulation::netlist_gen::generate_netlist_hierarchical;

use crate::properties::PropertyEditorSchema;
use crate::workbench::app::SchematicEditAuthority;
use crate::workbench::app::actions::workspace::sync_active_schematic;
use crate::workbench::app::schematic::instance_catalog::{
    InstanceCatalogEntry, instance_catalog, resolve_instance_catalog_entry,
};
use crate::workbench::app_state::{AppState, Model};
use crate::workbench::state::WorkbenchSession;

/// Isolated authority and draft for the Replace instance form. Current is a
/// resolved fact; the replacement master is the only choice, picked from
/// `candidates`. The selected component's stable identifier is retained so
/// the commit cannot drift to a later selection.
#[derive(Debug, Clone, Default)]
pub(crate) struct ReplaceInstanceDialogState {
    pub(crate) open: bool,
    pub(crate) source_component_id: u64,
    pub(crate) current: String,
    pub(crate) replacement: String,
    /// The masters the instance can be replaced with, the likeliest first.
    pub(crate) candidates: Vec<String>,
    pub(crate) authority: Option<SchematicEditAuthority>,
    pub(crate) replacement_authority: Option<crate::state::SchematicReplacementAuthority>,
    pub(crate) initial_target_identity: String,
    pub(crate) initial_target_spec: Option<crate::state::SchematicReplacementTargetSpec>,
    pub(crate) initial_replacement: String,
    pub(crate) preview_error: Option<String>,
    pub(crate) dirty: bool,
}

pub(crate) struct ReplaceInstanceOpen {
    pub(crate) authority: SchematicEditAuthority,
    pub(crate) replacement_authority: crate::state::SchematicReplacementAuthority,
    pub(crate) source_component_id: u64,
    pub(crate) current: String,
    pub(crate) replacement: String,
    pub(crate) initial_target_identity: String,
    pub(crate) initial_target_spec: crate::state::SchematicReplacementTargetSpec,
    pub(crate) candidates: Vec<String>,
}

impl ReplaceInstanceDialogState {
    pub(crate) fn open(&mut self, request: ReplaceInstanceOpen) {
        let initial_replacement = request.replacement.clone();
        *self = Self {
            open: true,
            source_component_id: request.source_component_id,
            current: request.current,
            replacement: request.replacement,
            candidates: request.candidates,
            authority: Some(request.authority),
            replacement_authority: Some(request.replacement_authority),
            initial_target_identity: request.initial_target_identity,
            initial_target_spec: Some(request.initial_target_spec),
            initial_replacement,
            preview_error: None,
            dirty: false,
        };
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }

    pub(crate) fn mark_edited(&mut self) {
        self.dirty = !self
            .replacement
            .trim()
            .eq_ignore_ascii_case(self.initial_replacement.trim());
        self.preview_error = None;
    }
}

const TITLE: &str = "Replace instance";
const PRIMARY: &str = "Replace";
/// The sentence assistive technology reads for the whole form.
const DESCRIPTION: &str = "Replace the selected instance's master, keeping the pins and authored values the new master can carry.";
/// The mapping table's columns: what the current master has, which of the
/// two it is, and what the replacement makes of it.
const MAPPING_COLUMNS: [TableColumn<'static>; 3] = [
    TableColumn {
        header: "Name",
        mono: true,
    },
    TableColumn {
        header: "Kind",
        mono: false,
    },
    TableColumn {
        header: "Replacement",
        mono: false,
    },
];

#[derive(Debug, Clone)]
struct ResolvedReplacement {
    identity: String,
    display: String,
    target: SchematicReplacementTargetSpec,
    preview: SchematicReplacementPreview,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ValidationCacheKey {
    source_component_id: u64,
    selected_component_id: Option<u64>,
    query: String,
    dirty: bool,
    preview_error: Option<String>,
    schematic_edit_read_only: bool,
    library_revision: u64,
    project_revision: u64,
    design_execution_epoch: u64,
    active_schematic_epoch: u64,
    topology_version: u64,
}

#[derive(Debug, Clone)]
struct CachedReplacementValidation {
    key: ValidationCacheKey,
    validation: ReplacementValidation,
}

#[derive(Debug, Clone)]
enum ReplacementValidation {
    Invalid(String),
    Valid(Box<ResolvedReplacement>),
}

/// One line of the mapping table: a pin, or a parameter the instance sets,
/// and what the replacement makes of it.
#[derive(Debug, Clone, PartialEq, Eq)]
struct MappingLine {
    name: String,
    kind: &'static str,
    outcome: String,
}

/// What the reader confirms against: every pin and every authored value of
/// the current master with what becomes of it, and how many authored values
/// the replacement discards.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ReplacementEvidence {
    lines: Vec<MappingLine>,
    dropped_parameters: usize,
}

impl ReplacementEvidence {
    fn table(&self) -> Vec<Vec<String>> {
        self.lines
            .iter()
            .map(|line| {
                vec![
                    line.name.clone(),
                    line.kind.to_owned(),
                    line.outcome.clone(),
                ]
            })
            .collect()
    }
}

/// Project the reviewed preview into what the form shows.
///
/// The preview is the authority — the same value commit applies — so the
/// mapping on screen is the mapping that will be made rather than a second
/// reading of the two contracts. A replacement that does not resolve has no
/// mapping to show; the footer says why. A parameter the instance never set
/// loses nothing whichever master it lands on, so only authored ones are
/// listed.
fn replacement_evidence(validation: &ReplacementValidation) -> Option<ReplacementEvidence> {
    let ReplacementValidation::Valid(resolved) = validation else {
        return None;
    };
    let compatibility = &resolved.preview.compatibility;
    let pins = compatibility
        .terminal_mappings
        .iter()
        .map(|mapping| MappingLine {
            name: mapping.source.clone(),
            kind: "Pin",
            outcome: carried_outcome(&mapping.source, mapping.target.as_deref()).unwrap_or_else(
                || {
                    if mapping.connected {
                        "Not carried, wired".to_owned()
                    } else {
                        "Not carried".to_owned()
                    }
                },
            ),
        });
    let parameters = compatibility
        .parameter_mappings
        .iter()
        .filter(|mapping| mapping.has_authored_value)
        .map(|mapping| MappingLine {
            name: mapping.source.clone(),
            kind: "Parameter",
            outcome: carried_outcome(&mapping.source, mapping.target.as_deref())
                .unwrap_or_else(|| "Value dropped".to_owned()),
        });
    Some(ReplacementEvidence {
        lines: pins.chain(parameters).collect(),
        dropped_parameters: resolved.preview.impact.dropped_parameters,
    })
}

/// "Carried", or "Carried as <name>" when the new master names it otherwise.
fn carried_outcome(source: &str, target: Option<&str>) -> Option<String> {
    let target = target?;
    Some(if target.eq_ignore_ascii_case(source) {
        "Carried".to_owned()
    } else {
        format!("Carried as {target}")
    })
}

/// What the replacement costs, stated in the footer before Replace is
/// pressed: nothing, or the authored values it discards.
fn dropped_parameter_consequence(dropped: usize) -> Option<String> {
    match dropped {
        0 => None,
        1 => Some("1 authored parameter value is discarded.".to_owned()),
        count => Some(format!("{count} authored parameter values are discarded.")),
    }
}

impl ReplacementValidation {
    fn can_commit(&self) -> bool {
        matches!(self, Self::Valid(_))
    }

    fn error(&self) -> Option<&str> {
        match self {
            Self::Invalid(message) => Some(message),
            Self::Valid(_) => None,
        }
    }
}

pub(crate) fn replace_instance_available(state: &AppState) -> bool {
    if state.schematic_edit_read_only() {
        return false;
    }
    let Some(component_id) = state
        .model
        .schematic
        .session
        .editor
        .selection
        .single_component()
    else {
        return false;
    };
    let Some(source) = state
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == component_id)
        .filter(|component| !matches!(component.kind, ComponentType::Ground | ComponentType::Port))
    else {
        return false;
    };
    instance_catalog(&state.design.property_registry, &state.model)
        .into_iter()
        .any(|entry| !entry.is_same_master(source))
}

pub(crate) fn open_replace_instance_dialog(state: &mut AppState) {
    // Replace instance is greyed on a read-only schematic.
    if state.schematic_edit_read_only() {
        return;
    }
    let replacement_authority =
        match replacement_authority(&state.design.property_registry, &state.model) {
            Ok(authority) => authority,
            Err(message) => {
                state.prompt(message);
                return;
            }
        };
    let ranked = ranked_replacements(
        &state.design.property_registry,
        &state.model,
        &replacement_authority,
    );
    let Some(resolved) = best_replacement(&state.model, &replacement_authority, &ranked) else {
        state.prompt("No other ready master keeps every connection of the selected instance.");
        return;
    };
    let current = current_display(replacement_authority.source_component());
    // The proposed master leads the picks; the rest follow in rank order.
    let candidates = std::iter::once(resolved.display.clone())
        .chain(
            ranked
                .into_iter()
                .map(|candidate| candidate.entry.display)
                .filter(|display| *display != resolved.display),
        )
        .collect();
    state.design.replace_instance.open(ReplaceInstanceOpen {
        authority: SchematicEditAuthority::capture(state),
        replacement_authority: replacement_authority.clone(),
        source_component_id: replacement_authority.component_id(),
        current,
        replacement: resolved.display,
        initial_target_identity: resolved.identity,
        initial_target_spec: resolved.target,
        candidates,
    });
}

/// What Replace instance draws over and changes: its draft, the property
/// sheets the instance catalog reads, the model and the session.
pub(in crate::workbench) struct ReplaceInstance<'a> {
    pub(in crate::workbench) dialog: &'a mut ReplaceInstanceDialogState,
    pub(in crate::workbench) registry: &'a PropertyEditorSchema,
    pub(in crate::workbench) model: &'a mut Model,
    pub(in crate::workbench) workbench: &'a mut WorkbenchSession,
}

impl ReplaceInstance<'_> {
    pub(in crate::workbench) fn render_replace_instance_dialog(&mut self, ctx: &Context) {
        if !self.dialog.open {
            return;
        }
        let validation = validation_for_repaint(ctx, self.dialog, self.registry, self.model);
        let evidence = replacement_evidence(&validation);
        let consequence = evidence
            .as_ref()
            .and_then(|evidence| dropped_parameter_consequence(evidence.dropped_parameters));
        let table = evidence.as_ref().map(ReplacementEvidence::table);
        let dialog = &*self.dialog;
        let mut picked = None;
        let choice = CommandForm::new(TITLE, PRIMARY)
            .wide()
            .describe(DESCRIPTION)
            .primary_enabled(validation.can_commit())
            .status(validation.error().map_or(Ok(None), Err))
            .consequence(consequence.as_deref())
            .show(ctx, |rows| {
                rows.value("Current", &dialog.current);
                let replace_with = rows.select_with_response(
                    "Replace with",
                    "replace-instance-master",
                    &dialog.replacement,
                    &dialog.candidates,
                );
                picked = replace_with.picked;
                if let Some(table) = table.as_ref() {
                    rows.table(&MAPPING_COLUMNS, table);
                }
                Some(replace_with.response.id)
            });
        if let Some(index) = picked {
            let dialog = &mut *self.dialog;
            if let Some(display) = dialog.candidates.get(index).cloned() {
                dialog.replacement = display;
                dialog.mark_edited();
            }
        }
        match choice {
            DialogChoice::Primary => self.commit_instance_replacement(),
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.dialog.close();
            }
            DialogChoice::None | DialogChoice::Secondary => {}
        }
    }

    fn commit_instance_replacement(&mut self) {
        let validation = validate_draft(self.dialog, self.registry, self.model);
        let ReplacementValidation::Valid(resolved) = validation else {
            return;
        };
        let Some(authority) = self.dialog.replacement_authority.clone() else {
            return;
        };
        match self
            .model
            .schematic
            .replace_selected_instance(&authority, &resolved.target)
        {
            Ok(impact) => {
                sync_active_schematic(self.model, self.workbench);
                let replaced = self
                    .model
                    .schematic
                    .document()
                    .components
                    .iter()
                    .find(|component| component.id == impact.component_id)
                    .map_or_else(
                        || "the instance".to_owned(),
                        |component| component.name.clone(),
                    );
                self.workbench.prompt(format!(
                    "Replaced {replaced} with {}; {} connections and {} parameter values kept.",
                    resolved.identity, impact.preserved_connections, impact.preserved_parameters,
                ));
                self.dialog.close();
            }
            Err(error) => {
                self.dialog.preview_error = Some(error.to_string());
            }
        }
    }
}

fn validate_draft(
    draft: &ReplaceInstanceDialogState,
    registry: &PropertyEditorSchema,
    model: &Model,
) -> ReplacementValidation {
    if let Some(error) = draft.preview_error.as_ref() {
        return ReplacementValidation::Invalid(error.clone());
    }
    let Some(edit_authority) = draft.authority.as_ref() else {
        return ReplacementValidation::Invalid(
            "The retained schematic authority is unavailable. Close and reopen Replace instance."
                .to_owned(),
        );
    };
    if let Err(error) = edit_authority.validate(model, TITLE) {
        return ReplacementValidation::Invalid(error);
    }
    let Some(authority) = draft.replacement_authority.as_ref() else {
        return ReplacementValidation::Invalid(
            "The retained replacement authority is unavailable. Close and reopen Replace instance."
                .to_owned(),
        );
    };
    if authority.component_id() != draft.source_component_id {
        return ReplacementValidation::Invalid(
            "The retained source-instance identity is inconsistent. Close and reopen Replace instance."
                .to_owned(),
        );
    }
    let validation = resolve_replacement(registry, model, authority, &draft.replacement);
    if !draft.dirty
        && let ReplacementValidation::Valid(resolved) = &validation
    {
        let same_identity = resolved.identity == draft.initial_target_identity;
        let same_spec = draft
            .initial_target_spec
            .as_ref()
            .is_some_and(|initial| initial == &resolved.target);
        if !same_identity || !same_spec {
            return ReplacementValidation::Invalid(
                "The initially reviewed replacement master changed. Close and reopen Replace instance."
                    .to_owned(),
            );
        }
    }
    validation
}

fn validation_for_repaint(
    ctx: &Context,
    draft: &ReplaceInstanceDialogState,
    registry: &PropertyEditorSchema,
    model: &Model,
) -> ReplacementValidation {
    let key = ValidationCacheKey {
        source_component_id: draft.source_component_id,
        selected_component_id: model.schematic.session.editor.selection.single_component(),
        query: draft.replacement.trim().to_ascii_lowercase(),
        dirty: draft.dirty,
        preview_error: draft.preview_error.clone(),
        schematic_edit_read_only: model.schematic_edit_read_only(),
        library_revision: model.library_manager.revision(),
        project_revision: model.workspace.content.project.revision().get(),
        design_execution_epoch: model.design_execution_epoch,
        active_schematic_epoch: model.active_schematic_epoch,
        topology_version: model.schematic.topology_version(),
    };
    let cache_id = egui::Id::new("replace-instance-validation-cache");
    if let Some(cached) = ctx.data(|data| data.get_temp::<CachedReplacementValidation>(cache_id))
        && cached.key == key
    {
        return cached.validation;
    }
    let validation = validate_draft(draft, registry, model);
    ctx.data_mut(|data| {
        data.insert_temp(
            cache_id,
            CachedReplacementValidation {
                key,
                validation: validation.clone(),
            },
        );
    });
    validation
}

fn resolve_replacement(
    registry: &PropertyEditorSchema,
    model: &Model,
    authority: &SchematicReplacementAuthority,
    authored: &str,
) -> ReplacementValidation {
    let catalog = instance_catalog(registry, model);
    let entry = match resolve_instance_catalog_entry(&catalog, authored) {
        Ok(entry) => entry,
        Err(error) => return ReplacementValidation::Invalid(error),
    };
    if entry.is_same_master(authority.source_component()) {
        return ReplacementValidation::Invalid(
            "Replacement must resolve to a different primitive or library/cell/view master."
                .to_owned(),
        );
    }
    let target = match entry.target_spec(model) {
        Ok(target) => target,
        Err(error) => return ReplacementValidation::Invalid(error),
    };
    let preview = match model
        .schematic
        .preview_instance_replacement(authority, &target)
    {
        Ok(preview) => preview,
        Err(error) => return ReplacementValidation::Invalid(error.to_string()),
    };
    if let Err(error) = validate_downstream(model, authority, &target) {
        return ReplacementValidation::Invalid(error);
    }
    ReplacementValidation::Valid(Box::new(ResolvedReplacement {
        identity: entry.identity.clone(),
        display: entry.display.clone(),
        target,
        preview,
    }))
}

fn replacement_authority(
    registry: &PropertyEditorSchema,
    model: &Model,
) -> Result<SchematicReplacementAuthority, String> {
    let component_id = model
        .schematic
        .session
        .editor
        .selection
        .single_component()
        .ok_or_else(|| "Select exactly one complete component instance to replace.".to_owned())?;
    let component = model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == component_id)
        .ok_or_else(|| "The selected component no longer exists.".to_owned())?;
    if matches!(component.kind, ComponentType::Ground | ComponentType::Port) {
        return Err("Ground and interface-port objects are not replaceable instances.".to_owned());
    }
    let source_spec = authored_source_spec(registry, model, component)?;
    model
        .schematic
        .replacement_authority_with_spec(source_spec)
        .map_err(|error| error.to_string())
}

fn authored_source_spec(
    registry: &PropertyEditorSchema,
    model: &Model,
    component: &Component,
) -> Result<SchematicReplacementSourceSpec, String> {
    let mut template = component.clone();
    template.pos = Point::origin();
    template.rotation = Rotation::R0;
    template.mirror_h = false;
    template.mirror_v = false;
    let resolved = template.library_cell.as_ref().and_then(|binding| {
        SymbolResolver::new(
            &model.library_manager,
            &model.workspace.content.schematic_buffers,
        )
        .resolve_binding(binding)
    });
    let positions = template.terminal_positions_resolved(resolved.as_ref());
    let terminals = positions
        .into_iter()
        .map(|(name, offset)| {
            let direction = template.library_cell.as_ref().and_then(|binding| {
                binding
                    .terminal_order
                    .iter()
                    .position(|candidate| candidate.eq_ignore_ascii_case(&name))
                    .and_then(|index| binding.terminal_dirs.get(index))
                    .copied()
            });
            let terminal = SchematicReplacementTerminal::new(name, offset);
            direction.map_or(terminal.clone(), |direction| {
                terminal.with_direction(direction)
            })
        })
        .collect();
    let spec = SchematicReplacementSourceSpec::from_component(component)
        .map_err(|error| error.to_string())?;
    let mut parameter_keys = spec.parameter_keys().to_vec();
    parameter_keys.extend(
        instance_catalog(registry, model)
            .into_iter()
            .filter(|entry| entry.is_same_master(component))
            .flat_map(|entry| {
                entry
                    .parameters
                    .into_iter()
                    .map(|parameter| parameter.name().to_owned())
            }),
    );
    parameter_keys.sort_by_key(|name| name.to_ascii_lowercase());
    parameter_keys.dedup_by(|left, right| left.eq_ignore_ascii_case(right));
    Ok(spec
        .with_terminals(terminals)
        .with_parameter_keys(parameter_keys))
}

/// One master that can stand in for the selected instance, with the
/// replacement it would make.
struct RankedReplacement {
    entry: InstanceCatalogEntry,
    target: SchematicReplacementTargetSpec,
    preview: SchematicReplacementPreview,
}

/// The first of `ranked` whose replacement also passes the downstream
/// checks: the form's opening choice.
fn best_replacement(
    model: &Model,
    authority: &SchematicReplacementAuthority,
    ranked: &[RankedReplacement],
) -> Option<ResolvedReplacement> {
    ranked.iter().find_map(|candidate| {
        validate_downstream(model, authority, &candidate.target).ok()?;
        Some(ResolvedReplacement {
            identity: candidate.entry.identity.clone(),
            display: candidate.entry.display.clone(),
            target: candidate.target.clone(),
            preview: candidate.preview.clone(),
        })
    })
}

/// Every other master the selected instance can be replaced with, the
/// likeliest first: same device prefix, same pin count, most pins and then
/// parameters carried, same library. These are the form's choices.
fn ranked_replacements(
    registry: &PropertyEditorSchema,
    model: &Model,
    authority: &SchematicReplacementAuthority,
) -> Vec<RankedReplacement> {
    let source = authority.source_component();
    let source_library = source
        .library_cell
        .as_ref()
        .map(|binding| binding.library.to_ascii_lowercase());
    let mut candidates = instance_catalog(registry, model)
        .into_iter()
        .filter(|entry| !entry.is_same_master(source))
        .filter_map(|entry| {
            let target = entry.target_spec(model).ok()?;
            let preview = model
                .schematic
                .preview_instance_replacement(authority, &target)
                .ok()?;
            let compatibility = &preview.compatibility;
            let same_library = source_library.as_ref().is_some_and(|source_library| {
                entry
                    .template
                    .library_cell
                    .as_ref()
                    .is_some_and(|binding| binding.library.eq_ignore_ascii_case(source_library))
            });
            let rank = (
                usize::from(entry.template.kind.spice_prefix() == source.kind.spice_prefix()),
                usize::from(
                    compatibility.source_terminal_count == compatibility.target_terminal_count,
                ),
                compatibility.mapped_terminal_count,
                compatibility.mapped_parameter_count,
                usize::from(same_library),
            );
            Some((rank, entry, target, preview))
        })
        .collect::<Vec<_>>();
    candidates.sort_by(|left, right| {
        right
            .0
            .cmp(&left.0)
            .then_with(|| left.1.identity.cmp(&right.1.identity))
    });
    candidates
        .into_iter()
        .map(|(_, entry, target, preview)| RankedReplacement {
            entry,
            target,
            preview,
        })
        .collect()
}

fn validate_downstream(
    model: &Model,
    authority: &SchematicReplacementAuthority,
    target: &SchematicReplacementTargetSpec,
) -> Result<(), String> {
    let mut candidate = model.schematic.clone();
    candidate
        .replace_selected_instance(authority, target)
        .map_err(|error| error.to_string())?;

    let baseline_hierarchy = model.workspace.resolve_hierarchy_with_active(
        &model.library_manager,
        &model.workspace.content.active_view,
        &model.schematic,
    );
    let candidate_hierarchy = model.workspace.resolve_hierarchy_with_active(
        &model.library_manager,
        &model.workspace.content.active_view,
        &candidate,
    );
    let baseline_failures = hierarchy_failures(&baseline_hierarchy);
    let new_failures = hierarchy_failures(&candidate_hierarchy)
        .difference(&baseline_failures)
        .cloned()
        .collect::<Vec<_>>();
    if !new_failures.is_empty() {
        return Err(format!(
            "Replacement introduces unresolved hierarchy: {}.",
            new_failures.join("; ")
        ));
    }

    let baseline_generation = generate_candidate_netlist(model, &model.schematic)?;
    let candidate_generation = generate_candidate_netlist(model, &candidate)?;
    let baseline_errors = baseline_generation
        .errors
        .into_iter()
        .map(|error| error.trim().to_owned())
        .collect::<HashSet<_>>();
    let new_errors = candidate_generation
        .errors
        .into_iter()
        .filter(|error| !baseline_errors.contains(error.trim()))
        .collect::<Vec<_>>();
    if !new_errors.is_empty() {
        return Err(format!(
            "Replacement is not netlist-compatible: {}",
            new_errors.join(" ")
        ));
    }
    Ok(())
}

fn hierarchy_failures(
    resolution: &crate::state::workspace::HierarchyResolution,
) -> HashSet<String> {
    resolution
        .bindings
        .iter()
        .filter(|binding| !binding.status.is_resolved())
        .map(|binding| {
            format!(
                "{} ({}){}",
                binding.reference.display_path(),
                binding.status.label(),
                binding
                    .diagnostic
                    .as_deref()
                    .map(|diagnostic| format!(": {diagnostic}"))
                    .unwrap_or_default()
            )
        })
        .collect()
}

/// Netlist one candidate sheet against the configured design.
///
/// The candidate is the subject; the masters it instantiates come from the
/// design projection. A candidate netlisted against the live editor buffers
/// would be compared for compatibility with a hierarchy the run does not use.
fn generate_candidate_netlist(
    model: &Model,
    schematic: &crate::state::SchematicState,
) -> Result<rspice_simulation::netlist_gen::NetlistResult, String> {
    let projection = model
        .workspace
        .design_projection(
            &model.library_manager,
            &model.workspace.content.active_view,
            &model.schematic,
        )
        .map_err(|error| error.to_string())?;
    let hierarchy =
        HierarchySource::from_design_projection(model.library_manager.catalog(), &projection);
    Ok(generate_netlist_hierarchical(
        schematic,
        &[],
        &hierarchy,
        &rspice_simulation::netlist_gen::NetlistSourceData::new(
            &crate::simulation::table_route::SourceFiles,
        ),
    ))
}

fn current_display(component: &Component) -> String {
    let master = component
        .library_cell
        .as_ref()
        .map(|binding| binding.cell.as_str())
        .unwrap_or_else(|| component.kind.display_name());
    format!("{} \u{00b7} {master}", component.name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Component, ComponentType};
    use crate::workbench::app::RSpiceApp;

    fn placed_source(params: &str) -> AppState {
        let mut state = AppState::default();
        let id = state
            .model
            .schematic
            .add_component(ComponentType::VoltageSource, Point::origin());
        if !params.is_empty() {
            state
                .model
                .schematic
                .document_mut_for_test()
                .components
                .iter_mut()
                .find(|component| component.id == id)
                .expect("the placed source is retained")
                .params = params.to_owned();
        }
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_component(id);
        state
    }

    /// Replace instance is a wide command form: Current, a pick list of the
    /// masters that can stand in (not a free-text field), and the mapping
    /// table. None of the wizard it was drawn as survives: no step tabs,
    /// purpose line or "Context" side table.
    #[test]
    fn replace_instance_is_a_command_form_with_a_pick_list_and_a_mapping_table() {
        let mut app = RSpiceApp::test_instance();
        app.state = placed_source("");
        open_replace_instance_dialog(&mut app.state);
        assert!(app.state.design.replace_instance.open);
        let candidates = &app.state.design.replace_instance.candidates;
        assert!(
            candidates.contains(&app.state.design.replace_instance.replacement),
            "the opening choice is one of the picks"
        );
        assert_eq!(
            candidates.first(),
            Some(&app.state.design.replace_instance.replacement),
            "the proposed master leads the picks"
        );

        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut nodes = Vec::new();
        for _ in 0..2 {
            nodes = ctx
                .run_ui(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(1440.0, 900.0),
                        )),
                        ..Default::default()
                    },
                    |ui| {
                        ReplaceInstance {
                            dialog: &mut app.state.design.replace_instance,
                            registry: &app.state.design.property_registry,
                            model: &mut app.state.model,
                            workbench: &mut app.state.workbench,
                        }
                        .render_replace_instance_dialog(ui.ctx());
                    },
                )
                .platform_output
                .accesskit_update
                .expect("AccessKit tree update")
                .nodes;
        }
        let texts = nodes
            .iter()
            .flat_map(|(_, node)| [node.label(), node.value()])
            .flatten()
            .collect::<Vec<_>>();
        assert!(
            nodes.iter().any(|(_, node)| {
                node.role() == egui::accesskit::Role::ComboBox
                    && node.label() == Some("Replace with")
            }),
            "Replace with is a pick list"
        );
        assert!(
            texts.iter().any(|text| text.starts_with("Name: ")),
            "the mapping table is published"
        );
        let wizard = texts
            .iter()
            .filter(|text| {
                text.contains("Scope") || text.contains("Context") || text.contains("Mapping")
            })
            .collect::<Vec<_>>();
        assert!(wizard.is_empty(), "the wizard is back: {wizard:?}");
    }

    #[test]
    fn replacement_is_unavailable_without_exactly_one_instance() {
        assert!(!replace_instance_available(&AppState::default()));
    }

    #[test]
    fn current_display_uses_stable_reference_and_master() {
        let component = Component::new(9, ComponentType::VoltageSource, Point::origin())
            .with_name_value("VREF", "1.2");
        assert_eq!(current_display(&component), "VREF \u{00b7} V DC");
    }

    #[test]
    fn voltage_source_variants_offer_a_real_compatible_replacement() {
        let mut state = AppState::default();
        let id = state
            .model
            .schematic
            .add_component(ComponentType::VoltageSource, Point::origin());
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_component(id);

        assert!(replace_instance_available(&state));
        open_replace_instance_dialog(&mut state);

        assert!(state.design.replace_instance.open);
        assert_eq!(state.design.replace_instance.source_component_id, id);
        assert!(!state.design.replace_instance.replacement.is_empty());
        assert!(
            validate_draft(
                &state.design.replace_instance,
                &state.design.property_registry,
                &state.model,
            )
            .can_commit()
        );
    }

    #[test]
    fn ambiguous_authored_master_fails_closed() {
        let mut state = AppState::default();
        let id = state
            .model
            .schematic
            .add_component(ComponentType::VoltageSource, Point::origin());
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_component(id);
        open_replace_instance_dialog(&mut state);
        assert!(state.design.replace_instance.open);

        state.design.replace_instance.replacement = "voltage".to_owned();
        state.design.replace_instance.mark_edited();

        let validation = validate_draft(
            &state.design.replace_instance,
            &state.design.property_registry,
            &state.model,
        );
        assert!(replacement_evidence(&validation).is_none());
        assert!(matches!(
            validation,
            ReplacementValidation::Invalid(message) if message.contains("ambiguous")
        ));
    }

    #[test]
    fn unrelated_library_revision_does_not_invalidate_the_reviewed_master() {
        let mut state = AppState::default();
        let id = state
            .model
            .schematic
            .add_component(ComponentType::VoltageSource, Point::origin());
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_component(id);
        open_replace_instance_dialog(&mut state);
        assert!(
            validate_draft(
                &state.design.replace_instance,
                &state.design.property_registry,
                &state.model,
            )
            .can_commit()
        );

        state.model.library_manager.clear();

        assert!(
            validate_draft(
                &state.design.replace_instance,
                &state.design.property_registry,
                &state.model,
            )
            .can_commit()
        );
    }

    /// The table names what is carried across and what is not. A summary
    /// count cannot say which parameter loses its authored value, and that is
    /// exactly the fact a reader needs before confirming; the footer states
    /// the cost.
    #[test]
    fn the_evidence_names_a_parameter_the_new_master_cannot_carry() {
        let mut state = placed_source("legacy_tail=3n");
        open_replace_instance_dialog(&mut state);
        assert!(state.design.replace_instance.open);

        let validation = validate_draft(
            &state.design.replace_instance,
            &state.design.property_registry,
            &state.model,
        );
        assert!(
            validation.can_commit(),
            "the fixture resolves a replacement"
        );
        let evidence = replacement_evidence(&validation).expect("a valid replacement has evidence");

        let dropped = evidence
            .lines
            .iter()
            .find(|line| line.name == "legacy_tail")
            .expect("the authored parameter is listed by name");
        assert_eq!(dropped.kind, "Parameter");
        assert_eq!(dropped.outcome, "Value dropped");
        assert!(evidence.dropped_parameters >= 1);
        assert!(
            dropped_parameter_consequence(evidence.dropped_parameters)
                .is_some_and(|consequence| consequence.contains("discarded"))
        );
        assert_eq!(dropped_parameter_consequence(0), None);
        assert!(
            evidence
                .lines
                .iter()
                .any(|line| line.kind == "Pin" && line.outcome.starts_with("Carried")),
            "a compatible replacement carries at least one pin across"
        );

        assert!(
            replacement_evidence(&ReplacementValidation::Invalid("no".to_owned())).is_none(),
            "an unresolved replacement has no mapping to show"
        );
    }

    #[test]
    fn validation_cache_does_not_hide_a_commit_failure() {
        let mut state = AppState::default();
        let id = state
            .model
            .schematic
            .add_component(ComponentType::VoltageSource, Point::origin());
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_component(id);
        open_replace_instance_dialog(&mut state);
        let context = Context::default();
        assert!(
            validation_for_repaint(
                &context,
                &state.design.replace_instance,
                &state.design.property_registry,
                &state.model,
            )
            .can_commit()
        );

        state.design.replace_instance.preview_error =
            Some("The validated replacement could not be committed.".to_owned());
        assert!(matches!(
            validation_for_repaint(
            &context,
            &state.design.replace_instance,
            &state.design.property_registry,
            &state.model,
        ),
            ReplacementValidation::Invalid(message)
                if message == "The validated replacement could not be committed."
        ));
    }
}
