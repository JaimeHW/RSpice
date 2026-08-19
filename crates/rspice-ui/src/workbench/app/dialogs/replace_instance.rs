//! Mockup-owned Replace selected instance operation.

use std::collections::HashSet;

use egui::{Align, Context, Frame, Layout, Margin, Popup, ScrollArea, Ui, vec2};

use crate::diagnostics::ConsoleMessage;
use crate::simulation::netlist_gen::{HierarchySource, generate_netlist_hierarchical};
use crate::state::{
    Component, ComponentType, Point, Rotation, SchematicReplacementAuthority,
    SchematicReplacementPreview, SchematicReplacementSourceSpec, SchematicReplacementTargetSpec,
    SchematicReplacementTerminal, SymbolResolver,
};
use crate::ui::tokens::Tokens;
use crate::ui::widgets::{
    Dialog, DialogChoice, DialogInitialFocus, DialogSize, DialogTransactionTone,
};

use crate::workbench::app::dialogs::operation_primitives::{
    BODY_HEIGHT, CONTEXT_WIDTH, SURFACE_HEIGHT, TRANSACTION_HEIGHT, impact_preview,
    operation_steps, paint_body_dividers,
};
use crate::workbench::app::dialogs::review_primitives::{
    input_field, purpose_line, read_only_field,
};
use crate::workbench::app::dialogs::schematic_command::{DISCARD_DETAIL, DISCARD_TITLE};
use crate::workbench::app::schematic::instance_catalog::{
    InstanceCatalogEntry, instance_catalog, resolve_instance_catalog_entry,
};
use crate::workbench::app::{RSpiceApp, ReplaceInstanceOpen, SchematicEditAuthority};
use crate::workbench::app_state::AppState;

const EYEBROW: &str = "SCHEMATIC \u{00b7} MODEL/SYMBOL COMPATIBILITY";
const TITLE: &str = "Replace selected instance";
const PRIMARY: &str = "Replace instance";
const DESCRIPTION: &str = "Replace with pin, parameter, model, and netlist compatibility review; preserve mapping where valid.";

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

    fn mapping(&self) -> Option<String> {
        let Self::Valid(resolved) = self else {
            return None;
        };
        let compatibility = &resolved.preview.compatibility;
        Some(format!(
            "{} / {} pins \u{00b7} {} / {} parameters",
            compatibility.mapped_terminal_count,
            compatibility.target_terminal_count,
            compatibility.mapped_parameter_count,
            compatibility.target_parameter_count,
        ))
    }
}

pub(crate) fn replace_instance_available(state: &AppState) -> bool {
    if state.schematic_edit_read_only() {
        return false;
    }
    let Some(component_id) = state.schematic.selection.single_component() else {
        return false;
    };
    let Some(source) = state
        .schematic
        .components
        .iter()
        .find(|component| component.id == component_id)
        .filter(|component| !matches!(component.kind, ComponentType::Ground | ComponentType::Port))
    else {
        return false;
    };
    instance_catalog(state)
        .into_iter()
        .any(|entry| !entry.is_same_master(source))
}

pub(crate) fn open_replace_instance_dialog(state: &mut AppState) {
    if state.schematic_edit_read_only() {
        state.push_user_message(ConsoleMessage::warning(
            "Replace instance is unavailable because the active schematic is read-only.".to_owned(),
        ));
        return;
    }
    let replacement_authority = match replacement_authority(state) {
        Ok(authority) => authority,
        Err(message) => {
            state.push_user_message(ConsoleMessage::warning(message));
            return;
        }
    };
    let Some(resolved) = best_replacement(state, &replacement_authority) else {
        state.push_user_message(ConsoleMessage::warning(
            "No different, ready replacement master can preserve the selected instance's connected terminal contract."
                .to_owned(),
        ));
        return;
    };
    let current = current_display(replacement_authority.source_component());
    let mapping = mapping_summary(&resolved.preview);
    state.dialogs.replace_instance.open(ReplaceInstanceOpen {
        authority: SchematicEditAuthority::capture(state),
        replacement_authority: replacement_authority.clone(),
        source_component_id: replacement_authority.component_id(),
        current,
        replacement: resolved.display,
        initial_target_identity: resolved.identity,
        initial_target_spec: resolved.target,
        mapping,
    });
}

impl RSpiceApp {
    pub(in crate::workbench) fn render_replace_instance_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.replace_instance.open {
            return;
        }
        let validation = validation_for_repaint(ctx, &self.state);
        self.state.dialogs.replace_instance.mapping = validation
            .mapping()
            .unwrap_or_else(|| "Unavailable".to_owned());
        let project = self.state.workspace.project.display_name().to_owned();
        let revision = self.state.workspace.project.revision().get().to_string();
        let discard_confirm = self.state.dialogs.replace_instance.discard_confirm;
        let retain_dirty_cancel = self.state.dialogs.replace_instance.dirty && !discard_confirm;
        let validation_error = validation.error().map(str::to_owned);
        let has_transaction = discard_confirm || validation_error.is_some();
        let mut dialog = Dialog::new(EYEBROW, TITLE, PRIMARY)
            .description(DESCRIPTION)
            .size(DialogSize::SimulationWorkflow)
            .initial_height(
                SURFACE_HEIGHT
                    + if has_transaction {
                        TRANSACTION_HEIGHT
                    } else {
                        0.0
                    },
            )
            .flush_body()
            .ghost(if discard_confirm {
                "Discard changes"
            } else {
                "Cancel"
            })
            .primary_enabled(validation.can_commit())
            .initial_focus(DialogInitialFocus::BodyControl);
        if retain_dirty_cancel {
            dialog = dialog.retain_on_cancel_focus(DialogInitialFocus::Ghost);
        }
        if discard_confirm {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Error,
                DISCARD_TITLE,
                DISCARD_DETAIL,
            );
        } else if let Some(error) = validation_error.as_deref() {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Error,
                "Replacement requires attention",
                error,
            );
        }

        let catalog = instance_catalog(&self.state);
        let mut replacement_changed = false;
        let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
            let focus = replacement_body(
                ui,
                &self.state.dialogs.replace_instance.current,
                &mut self.state.dialogs.replace_instance.replacement,
                &self.state.dialogs.replace_instance.mapping,
                validation.error(),
                &catalog,
                &project,
                &revision,
            );
            replacement_changed = focus.changed;
            Some(focus.id)
        });
        if replacement_changed {
            self.state.dialogs.replace_instance.mark_edited();
        }
        match choice {
            DialogChoice::Primary => self.commit_instance_replacement(ctx),
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.replace_instance.attempt_close();
            }
            DialogChoice::None | DialogChoice::Secondary => {}
        }
    }

    fn commit_instance_replacement(&mut self, ctx: &Context) {
        let validation = validate_draft(&self.state);
        let ReplacementValidation::Valid(resolved) = validation else {
            return;
        };
        let Some(authority) = self
            .state
            .dialogs
            .replace_instance
            .replacement_authority
            .clone()
        else {
            return;
        };
        match self
            .state
            .schematic
            .replace_selected_instance(&authority, &resolved.target)
        {
            Ok(impact) => {
                self.state.sync_active_schematic_to_workspace();
                let message = format!(
                    "Replaced component {} with {} as one undoable transaction; {} connections and {} parameter values were preserved.",
                    impact.component_id,
                    resolved.identity,
                    impact.preserved_connections,
                    impact.preserved_parameters,
                );
                self.state
                    .push_user_message(ConsoleMessage::info(message.clone()));
                self.state.ui.toasts.success(
                    ctx,
                    "Instance replaced",
                    "Pin mapping, parameter transfer, hierarchy, and generated netlist were validated before commit.",
                );
                self.state.dialogs.replace_instance.close();
            }
            Err(error) => {
                self.state.dialogs.replace_instance.preview_error = Some(error.to_string());
                self.state.dialogs.replace_instance.validation_field_mask = 1;
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct ReplacementBodyResponse {
    id: egui::Id,
    changed: bool,
}

fn replacement_body(
    ui: &mut Ui,
    current: &str,
    replacement: &mut String,
    mapping: &str,
    replacement_error: Option<&str>,
    catalog: &[InstanceCatalogEntry],
    project: &str,
    revision: &str,
) -> ReplacementBodyResponse {
    purpose_line(ui, DESCRIPTION);
    operation_steps(ui);
    let width = ui.available_width();
    // The restored design's 760 px breakpoint is a viewport media query.  The
    // desktop dialog itself is 760 px wide, so keying this off the body width
    // would incorrectly stack every desktop dialog.
    let stacked = replacement_uses_stacked_layout(ui.ctx().content_rect().width());
    let context_width = CONTEXT_WIDTH.min((width * 0.42).max(220.0));
    let form_width = if stacked {
        width
    } else {
        (width - context_width).max(1.0)
    };
    let mut field_id = ui.id().with("replace-instance-replacement");
    let mut changed = false;
    if stacked {
        let form_rect = replacement_form(
            ui,
            current,
            replacement,
            mapping,
            replacement_error,
            catalog,
            &mut field_id,
            &mut changed,
            form_width,
        );
        ui.painter().hline(
            form_rect.x_range(),
            form_rect.bottom(),
            egui::Stroke::new(1.0, Tokens::get(ui.ctx()).color.border_strong),
        );
        impact_preview(ui, project, revision);
    } else {
        let body = ui.allocate_ui_with_layout(
            vec2(width, BODY_HEIGHT),
            Layout::left_to_right(Align::Min),
            |ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                replacement_form(
                    ui,
                    current,
                    replacement,
                    mapping,
                    replacement_error,
                    catalog,
                    &mut field_id,
                    &mut changed,
                    form_width,
                );
                ui.allocate_ui_with_layout(
                    vec2(context_width, BODY_HEIGHT),
                    Layout::top_down(Align::Min),
                    |ui| {
                        ui.set_min_size(vec2(context_width, BODY_HEIGHT));
                        impact_preview(ui, project, revision);
                    },
                );
            },
        );
        paint_body_dividers(ui, body.response.rect, form_width);
    }
    ReplacementBodyResponse {
        id: field_id,
        changed,
    }
}

const fn replacement_uses_stacked_layout(viewport_width: f32) -> bool {
    viewport_width <= 760.0
}

fn replacement_form(
    ui: &mut Ui,
    current: &str,
    replacement: &mut String,
    mapping: &str,
    replacement_error: Option<&str>,
    catalog: &[InstanceCatalogEntry],
    field_id: &mut egui::Id,
    changed: &mut bool,
    form_width: f32,
) -> egui::Rect {
    ui.allocate_ui_with_layout(
        vec2(form_width, BODY_HEIGHT),
        Layout::top_down(Align::Min),
        |ui| {
            ui.set_min_size(vec2(form_width, BODY_HEIGHT));
            Frame::NONE
                .fill(Tokens::get(ui.ctx()).color.bg_app)
                .inner_margin(Margin::symmetric(12, 10))
                .show(ui, |ui| {
                    ui.set_width((form_width - 24.0).max(1.0));
                    read_only_field(ui, "Current", current, "Selected stable instance identity");
                    ui.add_space(9.0);
                    let response = input_field(
                        ui,
                        "Replacement",
                        replacement,
                        "library/cell/view or primitive name",
                        replacement_error,
                        "Ready replacement master with stable catalog identity",
                    );
                    *field_id = response.id;
                    *changed |= response.changed();
                    let popup_id = Popup::default_response_id(&response);
                    let query = replacement.trim().to_ascii_lowercase();
                    let mut picked = None;
                    let popup = Popup::menu(&response).gap(2.0).show(|ui| {
                        ui.set_min_width(response.rect.width());
                        ScrollArea::vertical()
                            .id_salt("replace-instance-candidates")
                            .max_height(180.0)
                            .show(ui, |ui| {
                                for entry in catalog.iter().filter(|entry| {
                                    query.is_empty()
                                        || entry.display.to_ascii_lowercase().contains(&query)
                                        || entry.identity.to_ascii_lowercase().contains(&query)
                                }) {
                                    if ui
                                        .selectable_label(false, &entry.display)
                                        .on_hover_text(&entry.identity)
                                        .clicked()
                                    {
                                        picked = Some(entry.display.clone());
                                    }
                                }
                            });
                    });
                    if popup.is_some() {
                        ui.ctx().accesskit_node_builder(popup_id, |node| {
                            node.set_label("Ready replacement masters");
                        });
                    }
                    if let Some(value) = picked {
                        *replacement = value;
                        *changed = true;
                        Popup::close_id(ui.ctx(), popup_id);
                    }
                    ui.add_space(9.0);
                    read_only_field(
                        ui,
                        "Mapping",
                        mapping,
                        "Preserved replacement pin and parameter mapping",
                    );
                });
        },
    )
    .response
    .rect
}

fn validate_draft(state: &AppState) -> ReplacementValidation {
    let draft = &state.dialogs.replace_instance;
    if let Some(error) = draft.preview_error.as_ref() {
        return ReplacementValidation::Invalid(error.clone());
    }
    let Some(edit_authority) = draft.authority.as_ref() else {
        return ReplacementValidation::Invalid(
            "The retained schematic authority is unavailable. Close and reopen Replace instance."
                .to_owned(),
        );
    };
    if let Err(error) = edit_authority.validate(state, TITLE) {
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
    let validation = resolve_replacement(state, authority, &draft.replacement);
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

fn validation_for_repaint(ctx: &Context, state: &AppState) -> ReplacementValidation {
    let draft = &state.dialogs.replace_instance;
    let key = ValidationCacheKey {
        source_component_id: draft.source_component_id,
        selected_component_id: state.schematic.selection.single_component(),
        query: draft.replacement.trim().to_ascii_lowercase(),
        dirty: draft.dirty,
        preview_error: draft.preview_error.clone(),
        schematic_edit_read_only: state.schematic_edit_read_only(),
        library_revision: state.library_manager.revision(),
        project_revision: state.workspace.project.revision().get(),
        design_execution_epoch: state.design_execution_epoch,
        active_schematic_epoch: state.active_schematic_epoch,
        topology_version: state.schematic.topology_version(),
    };
    let cache_id = egui::Id::new("replace-instance-validation-cache");
    if let Some(cached) = ctx.data(|data| data.get_temp::<CachedReplacementValidation>(cache_id))
        && cached.key == key
    {
        return cached.validation;
    }
    let validation = validate_draft(state);
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
    state: &AppState,
    authority: &SchematicReplacementAuthority,
    authored: &str,
) -> ReplacementValidation {
    let catalog = instance_catalog(state);
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
    let target = match entry.target_spec(state) {
        Ok(target) => target,
        Err(error) => return ReplacementValidation::Invalid(error),
    };
    let preview = match state
        .schematic
        .preview_instance_replacement(authority, &target)
    {
        Ok(preview) => preview,
        Err(error) => return ReplacementValidation::Invalid(error.to_string()),
    };
    if let Err(error) = validate_downstream(state, authority, &target) {
        return ReplacementValidation::Invalid(error);
    }
    ReplacementValidation::Valid(Box::new(ResolvedReplacement {
        identity: entry.identity.clone(),
        display: entry.display.clone(),
        target,
        preview,
    }))
}

fn replacement_authority(state: &AppState) -> Result<SchematicReplacementAuthority, String> {
    let component_id = state
        .schematic
        .selection
        .single_component()
        .ok_or_else(|| "Select exactly one complete component instance to replace.".to_owned())?;
    let component = state
        .schematic
        .components
        .iter()
        .find(|component| component.id == component_id)
        .ok_or_else(|| "The selected component no longer exists.".to_owned())?;
    if matches!(component.kind, ComponentType::Ground | ComponentType::Port) {
        return Err("Ground and interface-port objects are not replaceable instances.".to_owned());
    }
    let source_spec = authored_source_spec(state, component)?;
    state
        .schematic
        .replacement_authority_with_spec(source_spec)
        .map_err(|error| error.to_string())
}

fn authored_source_spec(
    state: &AppState,
    component: &Component,
) -> Result<SchematicReplacementSourceSpec, String> {
    let mut template = component.clone();
    template.pos = Point::origin();
    template.rotation = Rotation::R0;
    template.mirror_h = false;
    template.mirror_v = false;
    let resolved = template.library_cell.as_ref().and_then(|binding| {
        SymbolResolver::new(&state.library_manager, &state.workspace.schematic_buffers)
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
        instance_catalog(state)
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

fn best_replacement(
    state: &AppState,
    authority: &SchematicReplacementAuthority,
) -> Option<ResolvedReplacement> {
    let source = authority.source_component();
    let source_library = source
        .library_cell
        .as_ref()
        .map(|binding| binding.library.to_ascii_lowercase());
    let mut candidates = instance_catalog(state)
        .into_iter()
        .filter(|entry| !entry.is_same_master(source))
        .filter_map(|entry| {
            let target = entry.target_spec(state).ok()?;
            let preview = state
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
        .find_map(|(_, entry, target, preview)| {
            validate_downstream(state, authority, &target).ok()?;
            Some(ResolvedReplacement {
                identity: entry.identity,
                display: entry.display,
                target,
                preview,
            })
        })
}

fn validate_downstream(
    state: &AppState,
    authority: &SchematicReplacementAuthority,
    target: &SchematicReplacementTargetSpec,
) -> Result<(), String> {
    let mut candidate = state.schematic.clone();
    candidate
        .replace_selected_instance(authority, target)
        .map_err(|error| error.to_string())?;

    let baseline_hierarchy = state.workspace.resolve_hierarchy_with_active(
        &state.library_manager,
        &state.workspace.active_view,
        &state.schematic,
    );
    let candidate_hierarchy = state.workspace.resolve_hierarchy_with_active(
        &state.library_manager,
        &state.workspace.active_view,
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

    let baseline_generation = generate_candidate_netlist(state, &state.schematic)?;
    let candidate_generation = generate_candidate_netlist(state, &candidate)?;
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
    state: &AppState,
    schematic: &crate::state::SchematicState,
) -> Result<crate::simulation::netlist_gen::NetlistResult, String> {
    let projection = state
        .workspace
        .design_projection(
            &state.library_manager,
            &state.workspace.active_view,
            &state.schematic,
        )
        .map_err(|error| error.to_string())?;
    let hierarchy = HierarchySource::from_design_projection(&state.library_manager, &projection);
    Ok(generate_netlist_hierarchical(schematic, &[], &hierarchy))
}

fn current_display(component: &Component) -> String {
    let master = component
        .library_cell
        .as_ref()
        .map(|binding| binding.cell.as_str())
        .unwrap_or_else(|| component.kind.display_name());
    format!("{} \u{00b7} {master}", component.name)
}

fn mapping_summary(preview: &SchematicReplacementPreview) -> String {
    let compatibility = &preview.compatibility;
    format!(
        "{} / {} pins \u{00b7} {} / {} parameters",
        compatibility.mapped_terminal_count,
        compatibility.target_terminal_count,
        compatibility.mapped_parameter_count,
        compatibility.target_parameter_count,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Component, ComponentType};

    #[test]
    fn restored_mockup_contract_is_exact() {
        assert_eq!(TITLE, "Replace selected instance");
        assert_eq!(EYEBROW, "SCHEMATIC \u{00b7} MODEL/SYMBOL COMPATIBILITY");
        assert_eq!(PRIMARY, "Replace instance");
        assert_eq!(SURFACE_HEIGHT, 406.0);
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
            .schematic
            .add_component(ComponentType::VoltageSource, Point::origin());
        state.schematic.selection.select_only_component(id);

        assert!(replace_instance_available(&state));
        open_replace_instance_dialog(&mut state);

        assert!(state.dialogs.replace_instance.open);
        assert_eq!(state.dialogs.replace_instance.source_component_id, id);
        assert!(!state.dialogs.replace_instance.replacement.is_empty());
        assert!(validate_draft(&state).can_commit());
    }

    #[test]
    fn ambiguous_authored_master_fails_closed() {
        let mut state = AppState::default();
        let id = state
            .schematic
            .add_component(ComponentType::VoltageSource, Point::origin());
        state.schematic.selection.select_only_component(id);
        open_replace_instance_dialog(&mut state);
        assert!(state.dialogs.replace_instance.open);

        state.dialogs.replace_instance.replacement = "voltage".to_owned();
        state.dialogs.replace_instance.mark_edited();

        let validation = validate_draft(&state);
        assert!(validation.mapping().is_none());
        assert!(matches!(
            validation,
            ReplacementValidation::Invalid(message) if message.contains("ambiguous")
        ));
    }

    #[test]
    fn unrelated_library_revision_does_not_invalidate_the_reviewed_master() {
        let mut state = AppState::default();
        let id = state
            .schematic
            .add_component(ComponentType::VoltageSource, Point::origin());
        state.schematic.selection.select_only_component(id);
        open_replace_instance_dialog(&mut state);
        assert!(validate_draft(&state).can_commit());

        state.library_manager.clear();

        assert!(validate_draft(&state).can_commit());
    }

    #[test]
    fn restored_mockup_stacks_only_at_its_viewport_media_breakpoint() {
        assert!(replacement_uses_stacked_layout(760.0));
        assert!(!replacement_uses_stacked_layout(761.0));
        assert!(!replacement_uses_stacked_layout(1_440.0));
    }

    #[test]
    fn validation_cache_does_not_hide_a_commit_failure() {
        let mut state = AppState::default();
        let id = state
            .schematic
            .add_component(ComponentType::VoltageSource, Point::origin());
        state.schematic.selection.select_only_component(id);
        open_replace_instance_dialog(&mut state);
        let context = Context::default();
        assert!(validation_for_repaint(&context, &state).can_commit());

        state.dialogs.replace_instance.preview_error =
            Some("The validated replacement could not be committed.".to_owned());
        assert!(matches!(
            validation_for_repaint(&context, &state),
            ReplacementValidation::Invalid(message)
                if message == "The validated replacement could not be committed."
        ));
    }
}
