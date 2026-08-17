//! Editing an instance's fields in place, from the inspector.
//!
//! An inline edit is committed against the design, not against the widget that
//! shows it: the field reads its value from the component every frame, and a
//! commit is refused rather than coerced when the text does not parse to the
//! field's type. That is why the parse and the commit are separate steps here
//! — a coerced value would persist a number the operator never typed.

use super::*;

pub(super) fn field_value(component: &Component, field: &InlineEditField) -> String {
    match field {
        InlineEditField::Instance => component.name.clone(),
        InlineEditField::Value => component.value.clone(),
        InlineEditField::Parameters => component.params.clone(),
        InlineEditField::Parameter(key) => crate::state::parse_params_string(&component.params)
            .get(key)
            .cloned()
            .unwrap_or_default(),
    }
}

/// Why `candidate` cannot be applied to `field`, if it cannot.
///
/// Reference designators obey SPICE identity rules. Declared instance
/// parameters reuse their property-sheet type, quantity, enum, and range
/// contract; unknown extension parameters remain losslessly editable.
pub(super) fn field_rejection(
    state: &AppState,
    component: &Component,
    field: &InlineEditField,
    candidate: &str,
) -> Option<String> {
    if let InlineEditField::Parameter(key) = field {
        match crate::workbench::app::authoritative_component_property_sheet(state, component) {
            Ok(Some(sheet)) => {
                if let Some(definition) = sheet.get(key) {
                    return parameter_source_rejection(state, definition, candidate);
                }
            }
            Ok(None) => {}
            Err(error) => return Some(error),
        }
    }
    let InlineEditField::Instance = field else {
        return None;
    };
    let candidate = candidate.trim();
    if candidate.is_empty() {
        return Some("Enter a non-empty instance name.".to_owned());
    }
    if candidate.eq_ignore_ascii_case(&component.name) {
        return None;
    }
    if let Err(error) = component.validate_reference_designator(candidate) {
        return Some(error);
    }
    state
        .schematic
        .components
        .iter()
        .any(|other| other.id != component.id && other.name.eq_ignore_ascii_case(candidate))
        .then(|| {
            format!(
                "A component named `{candidate}` already exists; SPICE designators are case-insensitively unique."
            )
        })
}

pub(super) fn parameter_source_rejection(
    state: &AppState,
    definition: &PropertyDefinition,
    candidate: &str,
) -> Option<String> {
    let candidate = candidate.trim();
    // An empty secondary parameter means "inherit the model/property-sheet
    // default"; it is not the same transaction as authoring an empty value.
    if candidate.is_empty() {
        return definition
            .required
            .then(|| format!("{} is required.", definition.display_name));
    }
    let value = match definition.prop_type {
        PropertyType::Number | PropertyType::Expression => {
            return crate::properties::tabbed_dialog::parse_expression_source(
                definition,
                candidate,
                state.ui.preferences.quantity_presentation_policy(),
                state.ui.number_locale,
            )
            .err();
        }
        PropertyType::String => PropertyValue::String(candidate.to_owned()),
        PropertyType::Boolean => match candidate.to_ascii_lowercase().as_str() {
            "true" | "1" | "yes" | "on" => PropertyValue::Boolean(true),
            "false" | "0" | "no" | "off" => PropertyValue::Boolean(false),
            _ => {
                return Some(format!(
                    "{} must be yes/no, true/false, on/off, or 1/0",
                    definition.display_name
                ));
            }
        },
        PropertyType::Enum => {
            let PropertyValue::Enum { options, .. } = &definition.default_value else {
                return Some(format!(
                    "{} has an invalid enumerated property contract",
                    definition.display_name
                ));
            };
            let Some(selected) = options
                .iter()
                .find(|option| option.eq_ignore_ascii_case(candidate))
            else {
                return Some(format!(
                    "{} must be one of: {}",
                    definition.display_name,
                    options.join(", ")
                ));
            };
            PropertyValue::Enum {
                selected: selected.clone(),
                options: options.clone(),
            }
        }
    };
    definition.validate(&value).err()
}

/// Write `candidate` into `field` on the live design.
///
/// Returns `true` when the design actually changed, so a session that only
/// regained and lost focus never manufactures an undo entry.
pub(super) fn apply_field(
    state: &mut AppState,
    id: u64,
    field: &InlineEditField,
    candidate: &str,
) -> bool {
    let Some(component) = state
        .schematic
        .components
        .iter_mut()
        .find(|component| component.id == id)
    else {
        return false;
    };
    let changed = match field {
        InlineEditField::Instance => {
            let candidate = candidate.trim();
            if component.name == candidate {
                false
            } else {
                component.name = candidate.to_owned();
                true
            }
        }
        InlineEditField::Value => {
            if component.value == candidate {
                false
            } else {
                component.value = candidate.to_owned();
                true
            }
        }
        InlineEditField::Parameters => {
            let updated = candidate.trim().to_owned();
            if component.params == updated {
                false
            } else {
                component.params = updated;
                true
            }
        }
        InlineEditField::Parameter(key) => {
            let updated = write_param(&component.params, key, candidate);
            if component.params == updated {
                false
            } else {
                component.params = updated;
                true
            }
        }
    };
    if changed {
        state.schematic.is_dirty = true;
        state.schematic.bump_topology_version();
    }
    changed
}

/// Set `key` to `value` in a `key=value key=value` parameter string,
/// preserving the order of the other entries. An empty value removes the
/// entry, returning the instance to whatever it inherits.
pub(super) fn write_param(params: &str, key: &str, value: &str) -> String {
    let value = value.trim();
    let mut parts: Vec<String> = Vec::new();
    let mut replaced = false;
    for entry in params.split_whitespace() {
        let entry_key = entry.split_once('=').map_or(entry, |(name, _)| name);
        if entry_key.eq_ignore_ascii_case(key) {
            replaced = true;
            if !value.is_empty() {
                parts.push(format!("{key}={value}"));
            }
        } else {
            parts.push(entry.to_owned());
        }
    }
    if !replaced && !value.is_empty() {
        parts.push(format!("{key}={value}"));
    }
    parts.join(" ")
}

/// Open an edit session on `field`, seeded with `current`.
pub(super) fn begin_edit(
    app: &mut RSpiceApp,
    component: &Component,
    field: InlineEditField,
    current: String,
) {
    if app.state.schematic_edit_read_only() {
        return;
    }
    let before = crate::state::SchematicSnapshot::capture(&app.state.schematic);
    app.state
        .workbench
        .inline_edit
        .begin(component.id, field, &current, before);
}

/// End the open session, folding everything typed into it into one undo
/// entry described by `description`.
pub(super) fn commit_edit(app: &mut RSpiceApp, description: &str) {
    if let Some(before) = app.state.workbench.inline_edit.end() {
        app.state.schematic.commit_undo_from(before, description);
    }
}

pub(super) fn edit_description(field: &InlineEditField) -> String {
    match field {
        InlineEditField::Instance => "rename instance".to_owned(),
        InlineEditField::Value => "edit instance value".to_owned(),
        InlineEditField::Parameters => "edit instance parameters".to_owned(),
        InlineEditField::Parameter(key) => format!("edit {key}"),
    }
}

pub(super) fn tunable_value_quantity(
    kind: ComponentType,
) -> Option<crate::state::DesignVariableQuantity> {
    use crate::state::DesignVariableQuantity as Quantity;

    match kind {
        ComponentType::Resistor | ComponentType::Ccvs => Some(Quantity::Resistance),
        ComponentType::Capacitor => Some(Quantity::Capacitance),
        ComponentType::VoltageSource | ComponentType::VoltageSourceAc => Some(Quantity::Voltage),
        ComponentType::CurrentSource | ComponentType::CurrentSourceAc => Some(Quantity::Current),
        ComponentType::Vcvs | ComponentType::Cccs | ComponentType::OpAmp => {
            Some(Quantity::Dimensionless)
        }
        _ => None,
    }
}

/// Component value fields carry their quantity through the owning device, while
/// design variables are deliberately self-describing. Preserve an explicit
/// unit when one is already present and otherwise add the unit implied by the
/// selected component before constructing the typed variable.
pub(super) fn typed_tuning_expression(
    value: &str,
    quantity: crate::state::DesignVariableQuantity,
) -> String {
    use crate::state::DesignVariableQuantity as Quantity;

    let value = value.trim();
    let has_ascii_suffix = |suffix: &str| {
        value
            .get(value.len().saturating_sub(suffix.len())..)
            .is_some_and(|candidate| candidate.eq_ignore_ascii_case(suffix))
    };
    match quantity {
        Quantity::Resistance if has_ascii_suffix("ohm") || value.ends_with('Ω') => {
            value.to_owned()
        }
        Quantity::Resistance => format!("{value} ohm"),
        Quantity::Capacitance if value.ends_with('F') => value.to_owned(),
        Quantity::Capacitance => format!("{value}F"),
        Quantity::Voltage if value.ends_with('V') => value.to_owned(),
        Quantity::Voltage => format!("{value}V"),
        Quantity::Current if value.ends_with('A') => value.to_owned(),
        Quantity::Current => format!("{value}A"),
        Quantity::Temperature | Quantity::Dimensionless => value.to_owned(),
    }
}

pub(super) fn is_parameter_identifier(value: &str) -> bool {
    let mut characters = value.chars();
    let Some(first) = characters.next() else {
        return false;
    };
    (first.is_ascii_alphabetic() || first == '_')
        && characters.all(|character| character.is_ascii_alphanumeric() || character == '_')
}

pub(super) fn simple_parameter_reference(value: &str) -> Option<&str> {
    let value = value.trim();
    let candidate = value
        .strip_prefix('{')
        .and_then(|body| body.strip_suffix('}'))
        .map(str::trim)
        .unwrap_or(value);
    is_parameter_identifier(candidate).then_some(candidate)
}

pub(super) fn proposed_tuning_variable_name(
    component: &Component,
    variables: &[crate::state::DesignVariable],
) -> String {
    let mut stem = component
        .name
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || character == '_' {
                character.to_ascii_uppercase()
            } else {
                '_'
            }
        })
        .collect::<String>();
    if !stem
        .chars()
        .next()
        .is_some_and(|character| character.is_ascii_alphabetic() || character == '_')
    {
        stem.insert_str(0, "P_");
    }
    stem.truncate(116);
    let base = format!("{stem}_VALUE");
    if !variables
        .iter()
        .any(|variable| variable.name.eq_ignore_ascii_case(&base))
    {
        return base;
    }
    for suffix in 2_u32.. {
        let suffix = format!("_{suffix}");
        let keep = 128_usize.saturating_sub(suffix.len());
        let mut candidate = base.clone();
        candidate.truncate(keep);
        candidate.push_str(&suffix);
        if !variables
            .iter()
            .any(|variable| variable.name.eq_ignore_ascii_case(&candidate))
        {
            return candidate;
        }
    }
    unreachable!("the finite variable set cannot exhaust every numeric suffix")
}

pub(super) enum ComponentTuningPreparation {
    ExistingBinding {
        variable_id: crate::product::DesignVariableId,
    },
    StageBinding {
        /// Boxed: a staged variable is an order of magnitude larger than the
        /// existing-binding case's identifier.
        variable: Box<crate::state::DesignVariable>,
        creates_variable: bool,
    },
}

pub(super) struct PreparedComponentTuning {
    plan_id: crate::product::SimulationPlanId,
    plan_revision: crate::product::ObjectRevision,
    variables: Vec<crate::state::DesignVariable>,
    source_view: CellViewRef,
    source_topology_version: u64,
    active_plan_run: Option<crate::product::RunId>,
    preparation: ComponentTuningPreparation,
}

/// Resolve the exact Tune transaction without mutating the sandbox or either
/// authoritative document. The inspector button and the click handler both
/// use this preflight so a control cannot advertise a workflow that staging
/// will later reject.
pub(super) fn prepare_component_tuning(
    app: &RSpiceApp,
    component: &Component,
) -> Result<PreparedComponentTuning, String> {
    if app.state.schematic_edit_read_only() {
        return Err("the active schematic is read-only".to_owned());
    }
    let quantity = tunable_value_quantity(component.kind).ok_or_else(|| {
        format!(
            "{} values do not have a truthful typed design-variable mapping",
            component.kind.display_name()
        )
    })?;
    let (plan_id, plan_revision) = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .map(|plan| (plan.id(), plan.revision()))
        .map_err(|error| format!("the active simulation plan is unavailable: {error}"))?;
    let variables = app
        .state
        .workspace
        .plan_data(plan_id)
        .map(|payload| payload.design_variables.clone())
        .ok_or_else(|| "the active simulation plan has no configuration payload".to_owned())?;
    let source_view = app.state.workspace.active_schematic_reference();
    let source_topology_version = app.state.schematic.topology_version();
    let active_plan_run = app
        .state
        .simulation
        .active_run()
        .filter(|run| {
            run.prepared_receipt()
                .and_then(|receipt| receipt.simulation_plan_id())
                == Some(plan_id)
        })
        .map(|run| run.run_id);

    let session = &app.state.workbench.verification;
    let current_session = session.tuning_plan_id == Some(plan_id)
        && session.tuning_plan_revision == Some(plan_revision);
    if current_session && let Some(pending) = session.tuning_instance_binding.as_ref() {
        if pending.component_id == component.id
            && pending.source_view == source_view
            && pending.source_topology_version == source_topology_version
            && pending.source_value == component.value
        {
            return Ok(PreparedComponentTuning {
                plan_id,
                plan_revision,
                variables,
                source_view,
                source_topology_version,
                active_plan_run,
                preparation: ComponentTuningPreparation::ExistingBinding {
                    variable_id: pending.variable.id,
                },
            });
        }
        if pending.creates_variable || pending.requires_schematic_edit() {
            return Err(format!(
                "{} already has an uncommitted Value binding; commit or revert it before tuning another instance",
                pending.component_name
            ));
        }
    }

    let (variable, creates_variable) = if let Some(reference) =
        simple_parameter_reference(&component.value)
    {
        let variable = variables
            .iter()
            .find(|variable| variable.name.eq_ignore_ascii_case(reference))
            .cloned()
            .ok_or_else(|| {
                format!(
                    "the Value row references parameter '{reference}', but the active plan does not define it"
                )
            })?;
        if variable.quantity != quantity {
            return Err(format!(
                "parameter '{}' is typed as {}, but {} requires {}",
                variable.name,
                variable.quantity.label(),
                component.kind.display_name(),
                quantity.label()
            ));
        }
        (variable, false)
    } else {
        let name = proposed_tuning_variable_name(component, &variables);
        let typed_expression = typed_tuning_expression(&component.value, quantity);
        let variable = crate::state::DesignVariable::new(
            &name,
            &typed_expression,
            quantity,
            // Generated run decks emit project/testbench parameters at
            // the configured root. The explicit instance binding remains
            // the sole consumer, while project scope keeps a child-cell
            // edit executable from its parent simulation root.
            crate::state::DesignVariableScope::Project,
            format!(
                "Value of {} in {}",
                component.name,
                source_view.display_path()
            ),
            None,
            crate::state::DesignVariableSweepEligibility::NestedSweepAndOptimization,
            crate::state::DesignVariableOverridePolicy::ExplicitTestLocalOverride,
        )
        .map_err(|error| {
            format!(
                "the current Value '{}' cannot become a typed {} variable: {error}",
                component.value,
                quantity.label()
            )
        })?;
        (variable, true)
    };

    Ok(PreparedComponentTuning {
        plan_id,
        plan_revision,
        variables,
        source_view,
        source_topology_version,
        active_plan_run,
        preparation: ComponentTuningPreparation::StageBinding {
            variable: Box::new(variable),
            creates_variable,
        },
    })
}

pub(super) fn component_tuning_action_block_reason(
    app: &RSpiceApp,
    component: &Component,
) -> Option<String> {
    match Command::VerificationPage(VerificationPage::Tuning).availability(app) {
        CommandAvailability::Available => prepare_component_tuning(app, component).err(),
        CommandAvailability::Disabled(reason) => Some(reason.to_owned()),
        CommandAvailability::Hidden => {
            Some("parameter tuning is unavailable in this context".to_owned())
        }
    }
}

/// Bind the selected Value row to the non-destructive parameter sandbox.
///
/// This function writes only runtime proposal state. The schematic and active
/// plan remain byte-for-byte authoritative until the existing review dialog
/// commits the complete transaction.
pub(super) fn stage_component_tuning(app: &mut RSpiceApp, component_id: u64) -> Result<(), String> {
    let component = app
        .state
        .schematic
        .components
        .iter()
        .find(|component| component.id == component_id)
        .cloned()
        .ok_or_else(|| "the selected instance no longer exists".to_owned())?;
    let prepared = prepare_component_tuning(app, &component)?;

    let session = &mut app.state.workbench.verification;
    if session.tuning_plan_id != Some(prepared.plan_id)
        || session.tuning_plan_revision != Some(prepared.plan_revision)
    {
        session.tuning_plan_id = Some(prepared.plan_id);
        session.tuning_plan_revision = Some(prepared.plan_revision);
        session.tuning_variables = prepared
            .variables
            .iter()
            .map(|variable| crate::workbench::state::TuningVariableDraft {
                variable_id: variable.id,
                baseline_expression: variable.expression.clone(),
                candidate_expression: variable.expression.clone(),
                validation_error: None,
                proposed: false,
            })
            .collect();
        session.tuning_instance_binding = None;
        session.tuning_selected_variable = None;
        session.tuning_focus_variable = None;
        session.tuning_baseline_run = prepared.active_plan_run;
        session.tuning_review_open = false;
    }

    let (variable, creates_variable) = match prepared.preparation {
        ComponentTuningPreparation::ExistingBinding { variable_id } => {
            session.tuning_selected_variable = Some(variable_id);
            session.tuning_focus_variable = Some(variable_id);
            return Ok(());
        }
        ComponentTuningPreparation::StageBinding {
            variable,
            creates_variable,
        } => (*variable, creates_variable),
    };
    if creates_variable {
        session.tuning_variables.retain(|draft| !draft.proposed);
        session
            .tuning_variables
            .push(crate::workbench::state::TuningVariableDraft {
                variable_id: variable.id,
                baseline_expression: variable.expression.clone(),
                candidate_expression: variable.expression.clone(),
                validation_error: None,
                proposed: true,
            });
    }

    let binding_expression = format!("{{{}}}", variable.name);
    session.tuning_instance_binding = Some(crate::workbench::state::TuningInstanceBindingDraft {
        component_id,
        component_name: component.name.clone(),
        source_view: prepared.source_view,
        source_topology_version: prepared.source_topology_version,
        source_value: component.value.clone(),
        binding_expression,
        variable: variable.clone(),
        creates_variable,
    });
    session.tuning_selected_variable = Some(variable.id);
    session.tuning_focus_variable = Some(variable.id);
    session.action_receipt = if creates_variable {
        format!(
            "{} is staged as a new typed {} variable for {}; the schematic and plan are unchanged.",
            variable.name,
            variable.quantity.label(),
            component.name
        )
    } else {
        format!(
            "{} is selected for {}; edits remain sandboxed until explicit review and commit.",
            variable.name, component.name
        )
    };
    Ok(())
}

/// One editable instance row.
///
/// The row applies each keystroke to the design so the canvas, connectivity,
/// and netlist track the edit as it is typed, but the undo history records a
/// single entry when the field loses focus. Illegal text is held in the
/// session buffer, outlined and explained, and never written to the design.
pub(super) fn edit_row(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    component: &Component,
    field: InlineEditField,
    label: &str,
) -> Option<String> {
    edit_row_with_hint(ui, app, component, field, label, "")
}

/// An editable row whose empty authoritative value presents inherited/default
/// copy. The hint is never written into the design.
pub(super) fn edit_row_with_hint(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    component: &Component,
    field: InlineEditField,
    label: &str,
    hint: &str,
) -> Option<String> {
    let editable = !app.state.schematic_edit_read_only();
    if !editable {
        let value = field_value(component, &field);
        property_row(ui, label, if value.is_empty() { hint } else { &value });
        return None;
    }

    let mut buffer = app
        .state
        .workbench
        .inline_edit
        .buffer_for(component.id, &field)
        .map_or_else(|| field_value(component, &field), str::to_owned);
    let rejection = app
        .state
        .workbench
        .inline_edit
        .error_for(component.id, &field)
        .map(str::to_owned);

    let tuning = Command::VerificationPage(VerificationPage::Tuning);
    let (response, tuning_response) = if matches!(field, InlineEditField::Value) {
        let tuning_block_reason = rejection.as_ref().map_or_else(
            || component_tuning_action_block_reason(app, component),
            |reason| {
                Some(format!(
                    "resolve the Value validation error before tuning: {reason}"
                ))
            },
        );
        let (edit, action) = property_row_input_action(
            ui,
            label,
            &mut buffer,
            rejection.is_some(),
            WorkbenchIcon::Sliders,
            &format!("Scrub-tune {} in the parameter sandbox", component.name),
            tuning_block_reason.is_none(),
            tuning_block_reason.as_deref(),
        );
        (edit, Some(action))
    } else {
        (
            property_row_input_with_hint(ui, label, &mut buffer, hint, rejection.is_some()),
            None,
        )
    };
    if response.gained_focus() {
        begin_edit(app, component, field.clone(), buffer.clone());
    }
    if response.changed() {
        begin_edit(app, component, field.clone(), buffer.clone());
        app.state.workbench.inline_edit.set_buffer(buffer.clone());
        match field_rejection(&app.state, component, &field, &buffer) {
            Some(reason) => app.state.workbench.inline_edit.set_error(Some(reason)),
            None => {
                app.state.workbench.inline_edit.set_error(None);
                apply_field(&mut app.state, component.id, &field, &buffer);
            }
        }
    }
    if response.lost_focus() {
        commit_edit(app, &edit_description(&field));
    }
    if tuning_response.is_some_and(|response| response.clicked()) {
        match stage_component_tuning(app, component.id) {
            Ok(()) => tuning.execute(app),
            Err(error) => {
                app.state.workbench.verification.action_receipt = format!("Tune blocked: {error}");
                app.state
                    .push_user_message(crate::diagnostics::ConsoleMessage::warning(format!(
                        "Could not tune {}: {error}",
                        component.name
                    )));
            }
        }
    }

    let rejection = app
        .state
        .workbench
        .inline_edit
        .error_for(component.id, &field)
        .map(str::to_owned);
    if let Some(reason) = rejection.as_deref() {
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_description(reason);
            node.set_invalid(egui::accesskit::Invalid::True);
        });
    }
    rejection
}

pub(super) const INLINE_VALIDATION_SLOT_H: f32 = 18.0;

/// Stable validation slot for an editable property group.
///
/// The slot belongs to the open edit session: while one of the group's fields
/// holds focus it owns the same height whether or not the typed text was
/// rejected, so intermediate invalid input cannot push the terminal,
/// parameter, or operating-point sections up and down as it is typed. With no
/// session open there is nothing to report, and reserving the strip would only
/// leave every editable section framed by more space below its last row than
/// above its first.
pub(super) fn rejection_slot(ui: &mut Ui, editing: bool, reason: Option<&str>) -> egui::Rect {
    let t = Tokens::get(ui.ctx());
    if !editing && reason.is_none() {
        return egui::Rect::from_min_size(ui.cursor().min, egui::Vec2::ZERO);
    }
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), INLINE_VALIDATION_SLOT_H),
        egui::Sense::hover(),
    );
    if let Some(reason) = reason {
        let label_rect = rect.shrink2(egui::vec2(10.0, 0.0));
        let label = ui.put(
            label_rect,
            egui::Label::new(
                egui::RichText::new(reason)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.err),
            )
            .truncate(),
        );
        let _ = label.on_hover_text(reason);
        response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), reason)
        });
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_role(egui::accesskit::Role::Status);
            node.set_label(reason);
        });
    }
    rect
}
pub(super) fn net_class_tone(ui: &Ui, class: NetClass) -> Color32 {
    let t = Tokens::get(ui.ctx());
    match class {
        NetClass::Ground => t.color.text_dim,
        NetClass::Supply => t.color.accent,
        NetClass::Signal => t.color.ok,
    }
}

pub(super) fn net_panel(ui: &mut Ui, app: &mut RSpiceApp, name: &str, nets: &[DesignNet]) {
    let Some(net) = nets.iter().find(|net| net.name.eq_ignore_ascii_case(name)) else {
        // The conductor resolved to a net connectivity no longer reports.
        // Fall back to the sheet rather than narrate a phantom object.
        sheet_panel(ui, app, nets);
        return;
    };
    let class = net.class;
    let port = net.port;
    let net_name = net.name.clone();
    let segment_count = net.wire_ids.len();
    let terminals = net.terminals.clone();
    let scope = match port {
        Some(direction) => format!("interface port · {}", direction.keyword()),
        None if class == NetClass::Ground => "global reference".to_owned(),
        None => "sheet-local".to_owned(),
    };

    hero(
        ui,
        app,
        Hero {
            preview: HeroPreview::Icon(net_icon(class)),
            eyebrow: format!(
                "NET · SHEET {}",
                app.state.workspace.active_view.cell.to_ascii_uppercase()
            ),
            title: net_name.clone(),
            subtitle: scope.clone(),
            statuses: vec![(class.keyword().to_owned(), net_class_tone(ui, class))],
            open_properties: None,
        },
    );

    section_header(ui, "Net identity", None);
    property_row(ui, "Class", class.keyword());
    property_row(ui, "Scope", &scope);
    property_row(
        ui,
        "Conductors",
        &match segment_count {
            0 => "no drawn segments".to_owned(),
            1 => "1 wire".to_owned(),
            count => format!("{count} wires"),
        },
    );
    property_row(
        ui,
        "Terminals",
        &if terminals.is_empty() {
            "unwired".to_owned()
        } else {
            terminals.len().to_string()
        },
    );

    section_header(
        ui,
        "Connected terminals",
        Some(&terminals.len().to_string()),
    );
    if terminals.is_empty() {
        muted_inspector_copy(
            ui,
            "No bound terminals. A wire binds when it ends on an instance pin.",
        );
    } else {
        let mut select: Option<u64> = None;
        for terminal in &terminals {
            let value = app
                .state
                .schematic
                .components
                .iter()
                .find(|component| component.id == terminal.component_id)
                .map_or_else(String::new, |component| component.value.clone());
            let label = format!("{}.{}", terminal.reference, terminal.pin);
            let row = TreeRow::new(&label).mono().indent(1).meta(&value).show(ui);
            if row.response.clicked() {
                select = Some(terminal.component_id);
            }
            row.response
                .on_hover_text(format!("Select {} on the sheet", terminal.reference));
        }
        if let Some(id) = select {
            select_component(app, id);
        }
    }

    match net_operating_point(&app.state, &net_name) {
        Some(annotation) => {
            section_header(
                ui,
                &format!("Operating point · Run {}", annotation.run_id),
                Some(if annotation.current {
                    "current"
                } else {
                    "stale"
                }),
            );
            if !annotation.current {
                property_row(
                    ui,
                    "Provenance",
                    "Historical evidence · rerun for current schematic",
                );
            }
            property_row(
                ui,
                "Node voltage",
                &format!(
                    "{} {}",
                    crate::quantity::format_engineering_value(annotation.voltage),
                    annotation.unit
                ),
            );
            property_row(
                ui,
                "Detail",
                &format!(
                    "V({net_name}) · {} node solution",
                    if annotation.current {
                        "current"
                    } else {
                        "historical"
                    }
                ),
            );
            property_row(ui, "Analysis", &annotation.analysis);
            property_row(
                ui,
                "Temperature",
                &format!(
                    "{:.1} °C",
                    app.state.sim_setup.reference_pvt.temperature_celsius
                ),
            );
        }
        None => {
            section_header(ui, "Operating point", Some("no evidence"));
            muted_inspector_copy(
                ui,
                "No retained operating point names this conductor. Run a DC operating point to annotate it.",
            );
        }
    }

    let current = checks_current(&app.state);
    let findings = current.then(|| {
        app.state
            .dialogs
            .drc_results
            .as_ref()
            .map(|result| {
                result
                    .violations()
                    .iter()
                    .filter(|violation| violation_targets_net(violation, net))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    });
    let collision_count = current.then(|| net_name_collision_count(&app.state, net));
    section_header(ui, "Checks", Some(&checks_status(&app.state)));
    let t = Tokens::get(ui.ctx());
    let (connectivity, connectivity_tone, connectivity_mark) = match findings.as_ref() {
        None => (
            "pending recheck".to_owned(),
            t.color.warn,
            StatusMark::Warning,
        ),
        Some(findings) if findings.is_empty() => (
            if terminals.len() >= 2 {
                "conductor closed".to_owned()
            } else {
                "declared".to_owned()
            },
            t.color.ok,
            StatusMark::Success,
        ),
        Some(findings) => {
            let severity = findings
                .iter()
                .map(|finding| finding.severity)
                .max()
                .unwrap_or(DrcSeverity::Info);
            (
                format!(
                    "{} finding{}",
                    findings.len(),
                    if findings.len() == 1 { "" } else { "s" }
                ),
                if severity >= DrcSeverity::Error {
                    t.color.err
                } else {
                    t.color.warn
                },
                if severity >= DrcSeverity::Error {
                    StatusMark::Failure
                } else {
                    StatusMark::Warning
                },
            )
        }
    };
    property_row_status(
        ui,
        "Connectivity",
        &connectivity,
        connectivity_tone,
        connectivity_mark,
    );
    let (collision_label, collision_tone, collision_mark) = match collision_count {
        None => (
            "pending recheck".to_owned(),
            t.color.warn,
            StatusMark::Warning,
        ),
        Some(0) => (
            "unique on sheet".to_owned(),
            t.color.ok,
            StatusMark::Success,
        ),
        Some(count) => (
            format!(
                "{count} conflicting name{}",
                if count == 1 { "" } else { "s" }
            ),
            t.color.err,
            StatusMark::Failure,
        ),
    };
    property_row_status(
        ui,
        "Name collisions",
        &collision_label,
        collision_tone,
        collision_mark,
    );

    let plottable = class != NetClass::Ground;
    let connected: Vec<u64> = terminals
        .iter()
        .map(|terminal| terminal.component_id)
        .collect();
    if plottable || !connected.is_empty() {
        action_stack(ui, |ui| {
            if plottable {
                let display = format!("V({net_name})");
                let label = format!("Plot {display}");
                if Button::new(&label).icon(Icon::Results).show(ui).clicked() {
                    let configuration_changed = crate::schematic::view::toggle_probe_with_feedback(
                        ui,
                        &mut app.state,
                        &net_name,
                        &display,
                    );
                    if configuration_changed {
                        app.invalidate_simulation_preflight();
                    }
                }
            }
            if !connected.is_empty()
                && Button::new("Select connected instances")
                    .ghost()
                    .show(ui)
                    .clicked()
            {
                app.state.schematic.selection.clear();
                app.state.schematic.net_highlight.clear();
                for id in &connected {
                    app.state.schematic.selection.select_component(*id);
                }
            }
        });
    }
}

pub(super) struct NetAnnotation {
    run_id: u64,
    analysis: String,
    voltage: f64,
    unit: String,
    current: bool,
}

/// The selected run's DC node voltage for a net. Historical values remain
/// inspectable but carry an explicit stale provenance label.
pub(super) fn net_operating_point(state: &AppState, net: &str) -> Option<NetAnnotation> {
    let bare = net.to_ascii_lowercase();
    let wrapped = format!("v({bare})");
    let current = active_run_matches_design(state);
    state.simulation.active_run().and_then(|run| {
        run.analyses.iter().find_map(|analysis| {
            analysis.dc_op.as_ref().and_then(|op| {
                op.node_voltages
                    .iter()
                    .find(|value| {
                        let name = value.name.to_ascii_lowercase();
                        name == wrapped || name == bare
                    })
                    .map(|value| NetAnnotation {
                        run_id: run.id,
                        analysis: analysis.label.clone(),
                        voltage: value.value,
                        unit: value.unit.clone(),
                        current,
                    })
            })
        })
    })
}

pub(super) fn active_run_matches_design(state: &AppState) -> bool {
    let Some(run) = state.simulation.active_run() else {
        return false;
    };
    run.prepared_receipt()
        .is_some_and(|receipt| receipt.project_revision() == state.workspace.project.revision())
        && state.simulation.cross_probe.is_current_for(
            &state.workspace.active_view,
            state.schematic.topology_version(),
        )
}

pub(super) fn violation_targets_net(violation: &DrcViolation, net: &DesignNet) -> bool {
    use crate::services::drc::DrcViolationType;

    if !matches!(
        violation.violation_type,
        DrcViolationType::FloatingNode
            | DrcViolationType::UnconnectedPin
            | DrcViolationType::OrphanNetLabel
            | DrcViolationType::DanglingWire
            | DrcViolationType::ShortedOutputs
            | DrcViolationType::ShortCircuit
            | DrcViolationType::SourceToSource
            | DrcViolationType::InvalidName
    ) {
        return false;
    }
    match &violation.location {
        DrcLocation::Node { net_name } | DrcLocation::NetLabel { name: net_name } => {
            net_name.eq_ignore_ascii_case(&net.name)
        }
        DrcLocation::Wire { id } => net.wire_ids.contains(id),
        DrcLocation::Component { id, .. } => {
            violation.violation_type != DrcViolationType::UnconnectedPin
                && net
                    .terminals
                    .iter()
                    .any(|terminal| terminal.component_id == *id)
                && violation.related_items.iter().any(|item| {
                    item.eq_ignore_ascii_case(&net.name)
                        || net
                            .terminals
                            .iter()
                            .any(|terminal| item.eq_ignore_ascii_case(&terminal.reference))
                })
        }
        DrcLocation::Point { .. }
        | DrcLocation::Bus { .. }
        | DrcLocation::BusTap { .. }
        | DrcLocation::Global
        | DrcLocation::SymbolPin { .. } => violation
            .related_items
            .iter()
            .any(|item| item.eq_ignore_ascii_case(&net.name)),
    }
}

pub(super) fn net_name_collision_count(state: &AppState, net: &DesignNet) -> usize {
    let graph = NetGraph::build(&state.schematic.wires, &state.schematic.junctions);
    let wire_ids = net
        .wire_ids
        .iter()
        .copied()
        .collect::<std::collections::HashSet<_>>();
    let mut names = state
        .schematic
        .net_labels
        .iter()
        .filter(|label| {
            let connected = graph.find_connected_wires(label.pos);
            (!wire_ids.is_empty() && connected.iter().any(|id| wire_ids.contains(id)))
                || (wire_ids.is_empty() && label.name.eq_ignore_ascii_case(&net.name))
        })
        .map(|label| normalized_net_name(&label.name, state.schematic.document_policy.net_naming))
        .collect::<std::collections::HashSet<_>>();
    for terminal in &net.terminals {
        if let Some(port) = state
            .schematic
            .components
            .iter()
            .find(|component| component.id == terminal.component_id)
            .and_then(Component::port_spec)
        {
            names.insert(normalized_net_name(
                &port.name,
                state.schematic.document_policy.net_naming,
            ));
        }
    }
    names.insert(normalized_net_name(
        &net.name,
        state.schematic.document_policy.net_naming,
    ));
    names.len().saturating_sub(1)
}

pub(super) fn normalized_net_name(name: &str, policy: NetNamingPolicy) -> String {
    match policy {
        NetNamingPolicy::StrictCaseSensitive => name.to_owned(),
        NetNamingPolicy::SpiceCompatibleRelaxed => name.to_ascii_lowercase(),
    }
}

pub(super) fn select_component(app: &mut RSpiceApp, id: u64) {
    let position = app
        .state
        .schematic
        .components
        .iter()
        .find(|component| component.id == id)
        .map(|component| component.pos);
    app.state.schematic.selection.select_only_component(id);
    app.state.schematic.net_highlight.clear();
    app.state.schematic.center_request = position;
}

/// Select every conductor of a net and highlight it — the same transaction
/// the navigator's net rows commit.
pub(super) fn select_net(app: &mut RSpiceApp, net: &DesignNet) {
    app.state.schematic.selection.clear();
    for wire in &net.wire_ids {
        app.state.schematic.selection.select_wire(*wire);
    }
    if net.wire_ids.is_empty() {
        for component_id in net.terminals.iter().map(|terminal| terminal.component_id) {
            app.state.schematic.selection.select_component(component_id);
        }
    }
    app.state
        .schematic
        .net_highlight
        .highlight_named_wires(&net.name, net.wire_ids.iter().copied().collect());
}

// =============================================================================
