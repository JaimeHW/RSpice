//! Editing an instance's fields in place, from the inspector.
//!
//! An inline edit is committed against the design, not against the widget that
//! shows it: the field reads its value from the component every frame, and a
//! commit is refused rather than coerced when the text does not parse to the
//! field's type. That is why the parse and the commit are separate steps here
//! — a coerced value would persist a number the operator never typed.

use super::*;
use crate::workbench::framework::command_registry::ids;
use crate::workbench::framework::requests::StageComponentTuning;
use rspice_design::schematic::bom::BomField;
use rspice_vocabulary::property::{deck_number_spelling, list_elements};

#[cfg(test)]
mod tests;

pub(super) fn field_value(component: &Component, field: &InlineEditField) -> String {
    match field {
        InlineEditField::Instance => component.name.clone(),
        InlineEditField::Value => component.value.clone(),
        InlineEditField::Parameters => component.params.clone(),
        // A cell instance's `m` is its typed multiplicity, never text.
        InlineEditField::Parameter(key)
            if crate::workbench::app_state::model::is_instance_multiplicity(component, key) =>
        {
            component
                .multiplicity
                .map(|multiplicity| multiplicity.to_string())
                .unwrap_or_default()
        }
        // Parameter text is keyed case-insensitively, and stored lower-case.
        // A purchasing field is read from the typed BOM record instead.
        InlineEditField::Parameter(key) => match BomField::from_key(key) {
            Some(bom) => component.bom.get(bom).to_owned(),
            None => crate::state::parse_params_string(&component.params)
                .get(&key.to_ascii_lowercase())
                .cloned()
                .unwrap_or_default(),
        },
    }
}

/// Why `candidate` cannot be applied to `field`, if it cannot.
///
/// Reference designators obey SPICE identity rules. Declared instance
/// parameters reuse their property-sheet type, quantity, enum, and range
/// contract; unknown extension parameters remain losslessly editable.
///
/// A field of an independent source is then put through
/// [`source_commit_refusal`](crate::properties::property_bridge::source_commit_refusal)
/// — the same gate the modal component editor commits through — because a
/// value the engine refuses is refused wherever it was typed. Until this ran
/// here, `TD=-1n` was turned away by the editor and accepted by the inline
/// field beside it.
#[cfg(test)]
pub(super) fn field_rejection(
    design: &DesignState,
    model: &Model,
    wb: &WorkbenchSession,
    component: &Component,
    field: &InlineEditField,
    candidate: &str,
) -> Option<String> {
    prepare_field_edit(design, model, wb, component, field, candidate).err()
}

fn prepare_field_edit(
    design: &DesignState,
    model: &Model,
    wb: &WorkbenchSession,
    component: &Component,
    field: &InlineEditField,
    source: &str,
) -> Result<Component, String> {
    let mut authored = source.to_owned();
    let mut typed_value = None;
    if let InlineEditField::Parameter(key) = field
        && let Some(sheet) = crate::workbench::app::authoritative_component_property_sheet(
            &design.property_registry,
            model,
            component,
        )?
        && let Some(definition) = sheet.get(key)
    {
        let source = if definition.verilog_numbers {
            deck_number_spelling(source)
        } else {
            source.to_owned()
        };
        typed_value = parameter_source_value(model, wb, definition, &source)?;
        authored = typed_value
            .as_ref()
            .map(|value| parameter_source_text(definition, &source, value))
            .unwrap_or_default();
    }
    let candidate = edited_component(component, field, &authored)?;
    if let Some(refusal) =
        source_contract_rejection(design, &candidate, field, typed_value.as_ref())
    {
        return Err(refusal);
    }
    let InlineEditField::Instance = field else {
        return Ok(candidate);
    };
    if candidate.name.is_empty() {
        return Err("Enter a non-empty instance name.".to_owned());
    }
    if candidate.name.eq_ignore_ascii_case(&component.name) {
        return Ok(candidate);
    }
    candidate.validate_reference_designator(&candidate.name)?;
    if model
        .schematic
        .document()
        .components
        .iter()
        .any(|other| other.id != component.id && other.name.eq_ignore_ascii_case(&candidate.name))
    {
        return Err(format!(
            "A component named `{}` already exists; SPICE designators are case-insensitively unique.",
            candidate.name
        ));
    }
    Ok(candidate)
}

/// Why committing `candidate` into `field` would break this source's waveform
/// contract, if it would.
///
/// The candidate is written onto a copy of the instance first, so the gate
/// reads the fields as they will stand *after* the commit rather than as they
/// stand now: an inline edit moves exactly one field, and a rule that spans two
/// of them has to see the new value in place. The copy is built by the same
/// `edited_component` preparation the commit uses, so the gate can never judge a string the
/// commit would not have written.
fn source_contract_rejection(
    design: &DesignState,
    component: &Component,
    field: &InlineEditField,
    typed_value: Option<&PropertyValue>,
) -> Option<String> {
    let sheet = design.property_registry.get(component.kind)?;
    if matches!(field, InlineEditField::Instance) {
        // The reference designator is not a waveform field, and re-reading the
        // contract on a rename would refuse a rename for a value the rename
        // did not touch.
        return None;
    }
    let mut values = crate::properties::property_bridge::collect_properties_from_component(
        component,
        &design.property_registry,
    );
    // The edited field is presented as the *typed* value it commits as, which
    // is what the modal editor's own commit hands the gate. Without this the
    // gate saw the raw parameter text, and the waveform rules read that with a
    // bare engineering parser: `td=-1ns` carries its unit into the parameter
    // string, fails that parse, and became a field the contract could not see.
    if let InlineEditField::Parameter(key) = field
        && let Some(value) = typed_value
    {
        values.insert(key.clone(), value.clone());
    }
    crate::properties::property_bridge::source_commit_refusal(component, &values, sheet)
}

/// The text a committed value is stored as. A module's number stays as it
/// was typed, so long as the deck reads the same value from that text.
fn parameter_source_text(
    definition: &PropertyDefinition,
    typed: &str,
    value: &PropertyValue,
) -> String {
    if definition.verilog_numbers
        && let PropertyValue::Number { value: number, .. } = value
        && rspice_vocabulary::quantity::engineering::parse_engineering_value(typed.trim())
            .is_ok_and(|read| (read - number).abs() <= 1e-12 * number.abs())
    {
        return typed.trim().to_owned();
    }
    crate::properties::property_bridge::property_value_to_string(value)
}

/// The typed value `candidate` commits as, or why it cannot commit.
///
/// One parse, two readers: the field's own type contract refuses from here, and
/// the source-contract gate is handed the value this produced rather than
/// re-reading the raw text. `Ok(None)` is an empty secondary parameter, which
/// means "inherit the model or property-sheet default" and is not the same
/// transaction as authoring an empty value.
fn parameter_source_value(
    model: &Model,
    wb: &WorkbenchSession,
    definition: &PropertyDefinition,
    candidate: &str,
) -> Result<Option<PropertyValue>, String> {
    let candidate = candidate.trim();
    if candidate.is_empty() {
        return if definition.required {
            Err(format!("{} is required.", definition.display_name))
        } else {
            Ok(None)
        };
    }
    let value = match definition.prop_type {
        PropertyType::Number | PropertyType::Expression => {
            return rspice_design::properties::value::parse_expression_source(
                definition,
                candidate,
                model.preferences.quantity_presentation_policy(),
                wb.number_locale,
            )
            .map(Some);
        }
        // A list is stored as one spelling however it was typed.
        PropertyType::String if definition.value_list.is_some() => {
            PropertyValue::String(list_elements(candidate).map_or_else(
                || candidate.to_owned(),
                |elements| {
                    let spelled: Vec<String> = elements
                        .iter()
                        .map(|element| deck_number_spelling(element))
                        .collect();
                    format!("[{}]", spelled.join(" "))
                },
            ))
        }
        PropertyType::String => PropertyValue::String(candidate.to_owned()),
        PropertyType::Boolean => match candidate.to_ascii_lowercase().as_str() {
            "true" | "1" | "yes" | "on" => PropertyValue::Boolean(true),
            "false" | "0" | "no" | "off" => PropertyValue::Boolean(false),
            _ => {
                return Err(format!(
                    "{} must be yes/no, true/false, on/off, or 1/0",
                    definition.display_name
                ));
            }
        },
        PropertyType::Enum => {
            let PropertyValue::Enum { options, .. } = &definition.default_value else {
                return Err(format!(
                    "{} has an invalid enumerated property contract",
                    definition.display_name
                ));
            };
            let Some(selected) = options
                .iter()
                .find(|option| option.eq_ignore_ascii_case(candidate))
            else {
                return Err(format!(
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
    definition.validate(&value)?;
    Ok(Some(value))
}

/// Build the complete component candidate without publishing draft text.
fn edited_component(
    component: &Component,
    field: &InlineEditField,
    candidate: &str,
) -> Result<Component, String> {
    let mut component = component.clone();
    match field {
        InlineEditField::Instance => component.name = candidate.trim().to_owned(),
        InlineEditField::Value => component.value = candidate.to_owned(),
        InlineEditField::Parameters => {
            crate::state::params_string::validate_parameter_text(candidate)?;
            component.params = candidate.trim().to_owned();
        }
        // The multiplicity goes back to the typed field it is read from, and
        // the plain 1 that means "stands for itself" is no multiplier at all,
        // as the component editor stores it.
        InlineEditField::Parameter(key)
            if crate::workbench::app_state::model::is_instance_multiplicity(&component, key) =>
        {
            component.multiplicity = if candidate.trim().is_empty() {
                None
            } else {
                Some(crate::state::InstanceMultiplicity::parse(candidate)?)
                    .filter(|multiplicity| multiplicity.value() != 1.0)
            };
        }
        InlineEditField::Parameter(key) => match BomField::from_key(key) {
            Some(bom) => candidate.trim().clone_into(component.bom.get_mut(bom)),
            None => {
                component.params = crate::state::params_string::set_parameter_value(
                    &component.params,
                    key,
                    candidate,
                )?;
                // Choosing a functional part's preset fills its parameters in.
                if let Some(descriptor) = component
                    .library_cell
                    .as_ref()
                    .filter(|binding| binding.is_builtin_functional())
                    .and_then(|binding| {
                        crate::state::validate_builtin_functional_binding(binding).ok()
                    })
                    .filter(|descriptor| {
                        descriptor
                            .preset
                            .is_some_and(|preset| preset.name.eq_ignore_ascii_case(key))
                    })
                {
                    component.params = crate::state::apply_functional_preset(
                        descriptor,
                        &component.params,
                        candidate.trim(),
                    )?;
                }
            }
        },
    }
    // A gate is redrawn with the input count its parameters now set.
    crate::state::conform_functional_inputs(&mut component)?;
    Ok(component)
}
/// Finish the previous field before opening a draft against the live component.
pub(super) fn begin_edit(
    cx: &mut Cx<'_, DesignFeature>,
    component_id: u64,
    field: InlineEditField,
) -> bool {
    if cx.model.schematic_edit_read_only() {
        return false;
    }
    if cx.wb.inline_edit.buffer_for(component_id, &field).is_some() {
        return true;
    }
    if commit_inline_component_edit(cx.model, cx.wb).is_err() {
        return false;
    }
    let Some(component) = cx
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == component_id)
        .cloned()
    else {
        return false;
    };
    let session = crate::workbench::state::InlineEditSession {
        buffer: field_value(&component, &field),
        candidate: Some(component.clone()),
        expected: component,
        description: edit_description(&field),
        field,
        authority: inline_edit_authority(cx.model),
        error: None,
        widget: None,
    };
    cx.wb.inline_edit.begin(session);
    true
}

pub(super) fn update_edit(cx: &mut Cx<'_, DesignFeature>, text: String) {
    let Some(session) = cx.wb.inline_edit.session() else {
        return;
    };
    let candidate = prepare_field_edit(
        cx.state,
        cx.model,
        cx.wb,
        &session.expected,
        &session.field,
        &text,
    );
    cx.wb.inline_edit.set_draft(text, candidate);
}

fn restore_edit_focus(ctx: &egui::Context, cx: &Cx<'_, DesignFeature>) {
    if let Some(widget) = cx
        .wb
        .inline_edit
        .session()
        .and_then(|session| session.widget)
    {
        ctx.memory_mut(|memory| memory.request_focus(widget));
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

pub(super) fn component_tuning_action_block_reason(
    cx: &Cx<'_, DesignFeature>,
    component: &Component,
) -> Option<String> {
    match cx.foreign_availability(ids::TUNING) {
        CommandAvailability::Available => cx
            .peers
            .simulate()
            .component_tuning_block_reason(cx.model, component),
        CommandAvailability::Disabled(reason) => Some(reason.to_owned()),
        CommandAvailability::Hidden => {
            Some("parameter tuning is unavailable in this context".to_owned())
        }
    }
}

/// One editable instance row.
///
/// The row applies each keystroke to the design so the canvas, connectivity,
/// and netlist track the edit as it is typed, but the undo history records a
/// single entry when the field loses focus. Illegal text is held in the
/// session buffer, outlined and explained, and never written to the design.
pub(super) fn edit_row(
    ui: &mut Ui,
    cx: &mut Cx<'_, DesignFeature>,
    component: &Component,
    field: InlineEditField,
    label: &str,
) -> Option<String> {
    edit_row_with_hint(
        ui,
        cx,
        component,
        field,
        label,
        "",
        PropertyRowCaption::default(),
    )
}

/// An editable row whose empty authoritative value presents inherited/default
/// copy. The hint is never written into the design.
///
/// A parameter whose sheet states its unit or what it is passes them as
/// `caption`: the unit sits inside the field's trailing edge and the
/// description on the label's hover, editable or not.
pub(super) fn edit_row_with_hint(
    ui: &mut Ui,
    cx: &mut Cx<'_, DesignFeature>,
    component: &Component,
    field: InlineEditField,
    label: &str,
    hint: &str,
    caption: PropertyRowCaption<'_>,
) -> Option<String> {
    let captioned = caption.unit.is_some() || caption.description.is_some();
    let editable = !cx.model.schematic_edit_read_only();
    if !editable {
        let value = field_value(component, &field);
        let shown = if value.is_empty() { hint } else { &value };
        if captioned {
            property_row_captioned(ui, label, shown, caption);
        } else {
            property_row(ui, label, shown);
        }
        return None;
    }

    let mut buffer = cx
        .wb
        .inline_edit
        .buffer_for(component.id, &field)
        .map_or_else(|| field_value(component, &field), str::to_owned);
    let rejection = cx
        .wb
        .inline_edit
        .error_for(component.id, &field)
        .map(str::to_owned);
    let cancel = cx.wb.inline_edit.session().is_some_and(|session| {
        session.expected.id == component.id
            && session.field == field
            && session.widget.is_some_and(|widget| {
                ui.memory(|memory| memory.has_focus(widget) || memory.had_focus_last_frame(widget))
            })
    }) && ui.input(|input| input.key_pressed(egui::Key::Escape));

    let (response, tuning_response) = ui
        .push_id((component.id, &field), |ui| {
            if matches!(field, InlineEditField::Value) {
                let tuning_block_reason = rejection.as_ref().map_or_else(
                    || component_tuning_action_block_reason(cx, component),
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
                    crate::workbench::design_system::PropertyRowAction {
                        icon: WorkbenchIcon::Sliders,
                        label: &format!("Scrub-tune {} in the parameter sandbox", component.name),
                        enabled: tuning_block_reason.is_none(),
                        disabled_reason: tuning_block_reason.as_deref(),
                    },
                );
                (edit, Some(action))
            } else if captioned {
                (
                    property_row_input_captioned(
                        ui,
                        label,
                        &mut buffer,
                        hint,
                        rejection.is_some(),
                        caption,
                    ),
                    None,
                )
            } else {
                (
                    property_row_input_with_hint(ui, label, &mut buffer, hint, rejection.is_some()),
                    None,
                )
            }
        })
        .inner;
    // A resistor, capacitor or inductor's value steps along the project's
    // series from the keyboard and the wheel, as typing it would.
    let stepped = matches!(field, InlineEditField::Value)
        && crate::schematic::view::standard_value_snap::takes_standard_value(component.kind)
        && rspice_schematic_editor::property_values::step_standard_value(
            ui,
            response.id,
            cx.model.workspace.content.standard_value_series,
            &mut buffer,
        );
    if cancel {
        cx.wb.inline_edit.end();
        ui.input_mut(|input| {
            input.consume_key(egui::Modifiers::NONE, egui::Key::Escape);
        });
        response.surrender_focus();
        return None;
    }
    if response.gained_focus() || response.changed() || stepped {
        if !begin_edit(cx, component.id, field.clone()) {
            restore_edit_focus(ui.ctx(), cx);
            return None;
        }
        cx.wb.inline_edit.set_widget(response.id);
        if response.changed() || stepped {
            update_edit(cx, buffer);
        }
    }
    if response.lost_focus()
        && cx.wb.inline_edit.buffer_for(component.id, &field).is_some()
        && commit_inline_component_edit(cx.model, cx.wb).is_err()
    {
        restore_edit_focus(ui.ctx(), cx);
    }
    if tuning_response.is_some_and(|response| response.clicked()) {
        if commit_inline_component_edit(cx.model, cx.wb).is_err() {
            restore_edit_focus(ui.ctx(), cx);
            return cx
                .wb
                .inline_edit
                .error_for(component.id, &field)
                .map(str::to_owned);
        }
        cx.wb.outbox.push(StageComponentTuning {
            component_id: component.id,
        });
    }

    let rejection = cx
        .wb
        .inline_edit
        .error_for(component.id, &field)
        .map(str::to_owned);
    if let Some(reason) = rejection.as_deref() {
        // The module's own account of the parameter stays; the error joins it.
        let description = match caption.description.filter(|text| !text.is_empty()) {
            Some(description) => format!("{description} {reason}"),
            None => reason.to_owned(),
        };
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_description(description);
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
    let inset = crate::workbench::design_system::panel_inset(ui);
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), INLINE_VALIDATION_SLOT_H),
        egui::Sense::hover(),
    );
    if let Some(reason) = reason {
        let label_rect = rect.shrink2(egui::vec2(inset, 0.0));
        // `Ui::put` centres what it places; a message reads from the panel's
        // left edge, under the row labels.
        let mut strip = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(label_rect)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
        );
        let label = strip.add(
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
            node.set_live(egui::accesskit::Live::Polite);
        });
    }
    rect
}

/// What a net's off-sheet connectors declare, for the identity section.
///
/// The canvas paints a connector's direction but never says where its partner
/// is. It cannot: the sheet catalog is not reachable from the painter, and a
/// name whose painted width varied with the catalog would break the one
/// intrinsic bounds function that marquee selection and fit-to-content share.
/// The inspector reads the model, so the partner sheets are stated here.
pub(super) struct OffSheetDeclaration {
    pub(super) direction: String,
    pub(super) partners: String,
}

/// `None` for a net no off-sheet connector names, which is why its rows are
/// absent rather than filled with "not applicable".
pub(super) fn off_sheet_declaration(model: &Model, name: &str) -> Option<OffSheetDeclaration> {
    // Net names are not case-sensitive under either naming policy: the
    // extraction joins `Out` and `out`, so their connectors are one net's.
    let same_name = |candidate: &str| candidate.eq_ignore_ascii_case(name);
    let connectors: Vec<&crate::state::NetLabel> = model
        .schematic
        .document()
        .net_labels
        .iter()
        .filter(|label| label.kind.off_sheet_direction().is_some() && same_name(&label.name))
        .collect();
    if connectors.is_empty() {
        return None;
    }

    // Declared in the product's canonical order, so two connectors that
    // disagree read the same way whichever was drawn first.
    let direction = crate::state::NetLabelKind::DIRECTIONS
        .into_iter()
        .filter(|direction| {
            connectors
                .iter()
                .any(|label| label.kind.off_sheet_direction() == Some(*direction))
        })
        .map(crate::state::NetLabelKind::direction_label)
        .collect::<Vec<_>>()
        .join(", ");

    let key = model.workspace.content.active_schematic_reference().key();
    let catalog = model
        .workspace
        .content
        .design_management
        .sheet_catalog(&key);
    let active_sheet = catalog.and_then(|catalog| catalog.active_sheet_id());
    let mut pages: Vec<u32> = Vec::new();
    if let Some(catalog) = catalog {
        for label in &connectors {
            let Some(sheet) = model
                .workspace
                .content
                .design_management
                .sheet_for_object_or_active(&key, label.id)
            else {
                continue;
            };
            if Some(sheet) == active_sheet {
                continue;
            }
            if let Some((page, _)) = catalog.page_number_and_count(sheet)
                && !pages.contains(&page)
            {
                pages.push(page);
            }
        }
    }
    pages.sort_unstable();

    Some(OffSheetDeclaration {
        direction,
        partners: if pages.is_empty() {
            "none \u{2014} no partner on another sheet".to_owned()
        } else {
            pages
                .iter()
                .map(|page| format!("p{page}"))
                .collect::<Vec<_>>()
                .join(", ")
        },
    })
}

pub(super) fn net_panel(
    ui: &mut Ui,
    cx: &mut Cx<'_, DesignFeature>,
    name: &str,
    nets: &[DesignNet],
) {
    let Some(net) = nets.iter().find(|net| net.name.eq_ignore_ascii_case(name)) else {
        // The conductor resolved to a net connectivity no longer reports.
        // Fall back to the sheet rather than narrate a phantom object.
        cx.state.inspector_yields_to_sheet = true;
        return;
    };
    let class = net.class;
    let port = net.port;
    let net_name = net.name.clone();
    let terminals = net.terminals.clone();
    let scope = match port {
        Some(direction) => format!("interface port · {}", direction.keyword()),
        None if class == NetClass::Ground => "global reference".to_owned(),
        None => "sheet-local".to_owned(),
    };

    object_title(ui, &net_name, Some("Net"), None);

    let off_sheet = off_sheet_declaration(cx.model, &net_name);

    section_header(ui, "Net identity", None);
    property_row(ui, "Scope", &scope);
    if let Some(declaration) = &off_sheet {
        property_row(
            ui,
            "Direction",
            &format!("Off-sheet · {}", declaration.direction),
        );
        property_row(ui, "Partners", &declaration.partners);
    }

    section_header(
        ui,
        "Connected terminals",
        Some(&terminals.len().to_string()),
    );
    if terminals.is_empty() {
        muted_inspector_copy(ui, "No pins on this net.");
    } else {
        let mut select: Option<u64> = None;
        for terminal in &terminals {
            let value = cx
                .model
                .schematic
                .document()
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
            select_component(cx, id);
        }
    }

    // The node voltage the active run solved for this net, when it solved
    // one; with none the section is absent.
    if let Some(annotation) = net_operating_point(cx.model, &net_name) {
        if annotation.current {
            section_header(ui, "Operating point", None);
        } else {
            section_header_explained(
                ui,
                "Operating point",
                "stale, rerun",
                &format!(
                    "Run {} solved an earlier version of this schematic.",
                    annotation.run_id
                ),
            );
        }
        property_row(
            ui,
            "Node voltage",
            &crate::workbench::docks::inspector::compact_readout(
                annotation.voltage,
                &annotation.unit,
            ),
        );
    }

    // Only this net's own findings, and only from a check of the drawing as it
    // stands. A clean or unchecked net is the normal state.
    let findings = current_findings(cx.model, |violation| violation_targets_net(violation, net));
    let collisions = if checks_current(cx.model) {
        net_name_collision_count(cx.model, net)
    } else {
        0
    };
    if !findings.is_empty() || collisions > 0 {
        section_header(ui, "Checks", None);
        if !findings.is_empty() {
            property_row_problem(
                ui,
                "Connectivity",
                &count_label(findings.len(), "finding"),
                Some(findings_problem(&findings)),
            );
        }
        if collisions > 0 {
            property_row_problem(
                ui,
                "Name collisions",
                &count_label(collisions, "conflicting name"),
                Some(Problem::Failure),
            );
        }
    }

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
                        &mut crate::schematic::view::canvas_parts_in(cx),
                        &net_name,
                        &display,
                    );
                    if configuration_changed {
                        rspice_app_runs::workbench::preflight::invalidate(
                            &mut cx.runs.preflight,
                            &mut *cx.handles.run_controller,
                        );
                    }
                }
            }
            if !connected.is_empty()
                && Button::new("Select connected instances")
                    .ghost()
                    .show(ui)
                    .clicked()
            {
                cx.model.schematic.session.editor.selection.clear();
                cx.model.schematic.session.editor.net_highlight.clear();
                for id in &connected {
                    cx.model
                        .schematic
                        .session
                        .editor
                        .selection
                        .select_component(*id);
                }
            }
        });
    }
}

pub(super) struct NetAnnotation {
    run_id: u64,
    voltage: f64,
    unit: String,
    current: bool,
}

/// The engine's name for net `name` at the occurrence being edited: the bare
/// name on the top sheet, the dotted instance path below it. The run names
/// every node that way, so a bare name below the root would read a node of
/// the top sheet.
pub(super) fn net_engine_name(model: &Model, name: &str) -> Option<String> {
    model
        .workspace
        .content
        .occurrence_path()
        .engine_name_for(name)
        .ok()
}

/// The selected run's DC node voltage for a net. A run of an earlier
/// drawing is still read, and says it is stale.
pub(super) fn net_operating_point(model: &Model, net: &str) -> Option<NetAnnotation> {
    let bare = net_engine_name(model, net)?;
    let wrapped = format!("v({bare})");
    let current = active_run_matches_design(model);
    model.simulation.active_run().and_then(|run| {
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
                        voltage: value.value,
                        unit: value.unit.clone(),
                        current,
                    })
            })
        })
    })
}

pub(super) fn active_run_matches_design(model: &Model) -> bool {
    let Some(run) = model.simulation.active_run() else {
        return false;
    };
    run.prepared_receipt().is_some_and(|receipt| {
        receipt.project_revision() == model.workspace.content.project.revision()
    }) && model.simulation.source.cross_probe.is_current_for(
        &model.workspace.content.active_view,
        model.schematic.topology_version(),
    )
}

pub(super) fn violation_targets_net(violation: &DrcViolation, net: &DesignNet) -> bool {
    use rspice_design::drc::DrcViolationType;

    if !matches!(
        violation.violation_type,
        // Every unattached-terminal finding is listed, so the family stays
        // complete when one of them is located somewhere a net can claim.
        DrcViolationType::FloatingNode
            | DrcViolationType::UnconnectedPin
            | DrcViolationType::UndrivenInput
            | DrcViolationType::UnloadedOutput
            | DrcViolationType::OrphanNetLabel
            | DrcViolationType::DanglingWire
            | DrcViolationType::ShortedOutputs
            | DrcViolationType::SourceToSource
            | DrcViolationType::InvalidName
    ) {
        return false;
    }
    match &violation.location {
        DrcLocation::Node { net_name, .. } | DrcLocation::NetLabel { name: net_name } => {
            net_name.eq_ignore_ascii_case(&net.name)
        }
        DrcLocation::Wire { id } => net.wire_ids.contains(id),
        DrcLocation::Component { id, .. } => {
            net.terminals
                .iter()
                .any(|terminal| terminal.component_id == *id)
                && violation.related_items().iter().any(|item| {
                    item.eq_ignore_ascii_case(&net.name)
                        || net
                            .terminals
                            .iter()
                            .any(|terminal| item.eq_ignore_ascii_case(&terminal.reference))
                })
        }
        // An unattached terminal is, by definition, on no net: the component
        // that owns it may well stand on this one through its other pins, and
        // that is not what the finding is about. This arm is why the component
        // arm above no longer has to exclude an open-pin finding — such a
        // finding is never located on a component any more.
        DrcLocation::Terminal { .. } => false,
        // A finding inside an instantiated cellview belongs to that cellview's
        // nets, not to this one's.
        DrcLocation::InsideInstance { .. } | DrcLocation::CellView { .. } => false,
        DrcLocation::Point { .. }
        | DrcLocation::Bus { .. }
        | DrcLocation::BusTap { .. }
        | DrcLocation::Global
        | DrcLocation::SymbolPin { .. } => violation
            .related_items()
            .iter()
            .any(|item| item.eq_ignore_ascii_case(&net.name)),
    }
}

pub(super) fn net_name_collision_count(model: &Model, net: &DesignNet) -> usize {
    // Which labels stand on this net is the one extraction's answer, so the
    // collision count describes the node the deck emits.
    let connectivity = extract(&model.schematic, None);
    let wire_ids = net
        .wire_ids
        .iter()
        .copied()
        .collect::<std::collections::HashSet<_>>();
    let mut names = model
        .schematic
        .document()
        .net_labels
        .iter()
        .filter(|label| {
            if wire_ids.is_empty() {
                return label.name.eq_ignore_ascii_case(&net.name);
            }
            connectivity
                .net_at(label.pos)
                .is_some_and(|extracted| extracted.wires.iter().any(|id| wire_ids.contains(id)))
        })
        .map(|label| normalized_net_name(&label.name))
        .collect::<std::collections::HashSet<_>>();
    for terminal in &net.terminals {
        if let Some(port) = model
            .schematic
            .document()
            .components
            .iter()
            .find(|component| component.id == terminal.component_id)
            .and_then(Component::port_spec)
        {
            names.insert(normalized_net_name(&port.name));
        }
    }
    names.insert(normalized_net_name(&net.name));
    names.len().saturating_sub(1)
}

/// The spelling two names share when they are one net. Connectivity folds
/// case under both naming policies (the policy only limits which characters
/// a name may use), so this does too.
pub(super) fn normalized_net_name(name: &str) -> String {
    name.to_ascii_lowercase()
}

pub(super) fn select_component(cx: &mut Cx<'_, DesignFeature>, id: u64) {
    let position = cx
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == id)
        .map(|component| component.pos);
    cx.model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(id);
    cx.model.schematic.session.editor.net_highlight.clear();
    cx.model.schematic.session.editor.center_request = position;
}

/// Select every conductor of a net and highlight it — the same transaction
/// the navigator's net rows commit.
pub(super) fn select_net(cx: &mut Cx<'_, DesignFeature>, net: &DesignNet) {
    cx.model.schematic.session.editor.selection.clear();
    for wire in &net.wire_ids {
        cx.model
            .schematic
            .session
            .editor
            .selection
            .select_wire(*wire);
    }
    if net.wire_ids.is_empty() {
        for component_id in net.terminals.iter().map(|terminal| terminal.component_id) {
            cx.model
                .schematic
                .session
                .editor
                .selection
                .select_component(component_id);
        }
    }
    cx.model
        .schematic
        .session
        .editor
        .net_highlight
        .highlight_named_wires(&net.name, net.wire_ids.iter().copied().collect());
}

// =============================================================================
