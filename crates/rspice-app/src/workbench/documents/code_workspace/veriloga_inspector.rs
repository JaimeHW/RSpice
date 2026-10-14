//! What the compiler knows about the Verilog-A source in front of the reader.
//!
//! The Design workspace's inspector dock draws this body, and the editor reads
//! its compile status and diagnostics from here too, so it sits below both the
//! docks and the surfaces. Everything here is derived from the exact compile
//! receipt or failure whose token matches the selected bundle: a receipt from
//! another cell view, or from an earlier revision of this one, never reads as
//! current.

use std::sync::Arc;

use egui::{Align, Color32, Layout, Sense, Stroke, Ui, Vec2};

use crate::product::ProjectId;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::Button;
use crate::workbench::design_system::{
    Problem, StatusMark, WorkbenchIcon, code_inspector_property_list, code_inspector_section,
    property_row, property_row_control_columns, property_row_height, property_row_status,
    property_row_toned,
};
use crate::workbench::{MessageCatalog, MessageId};

use rspice_design::library::metadata::{CellParameter, ParameterValueType};

use super::page::VerilogAWorkbenchState;
use super::veriloga_interface::{
    VerilogACellSymbol, VerilogAInterfaceOutcome, VerilogASymbolDrift, compiled_parameters,
};
use super::{
    CodeDiagnosticCollection, CodeEditorDiagnostic, CodeEditorSeverity, SelectedVerilogASource,
    TargetQualification, VerilogACompileReceipt,
};

/// The compile receipt for exactly this source, if the last compile was of it.
pub(crate) fn veriloga_receipt<'a>(
    state: &'a VerilogAWorkbenchState,
    project_id: ProjectId,
    selected: &SelectedVerilogASource,
) -> Option<&'a VerilogACompileReceipt> {
    state
        .receipt
        .as_ref()
        .filter(|receipt| selected.matches_token(project_id, receipt.compilation.token()))
}

/// The diagnostics the last compile of exactly this source produced: the
/// receipt's advisories after a pass, the failure's findings after a refusal.
pub(crate) fn veriloga_diagnostics(
    state: &VerilogAWorkbenchState,
    project_id: ProjectId,
    selected: &SelectedVerilogASource,
) -> Arc<CodeDiagnosticCollection> {
    veriloga_receipt(state, project_id, selected)
        .map(|receipt| Arc::clone(&receipt.diagnostics))
        .unwrap_or_else(|| {
            state
                .last_failure_token
                .filter(|token| selected.matches_token(project_id, *token))
                .map_or_else(
                    || Arc::new(CodeDiagnosticCollection::default()),
                    |_| Arc::clone(&state.last_failure),
                )
        })
}

/// One phrase and tone for where the source stands with the compiler.
///
/// The editor's status strip and the inspector's status row both read this,
/// so the two can never disagree about whether the visible text compiled.
/// A pass carrying advisories is still a pass: the words say so, and the
/// tone says Diagnostics still has something to read.
pub(crate) fn veriloga_compile_status(
    state: &VerilogAWorkbenchState,
    project_id: ProjectId,
    selected: &SelectedVerilogASource,
    messages: MessageCatalog,
    t: &Tokens,
) -> (String, Color32) {
    if state.pending.is_some() {
        return (
            messages.text(MessageId::VerilogAStatusCompiling),
            t.color.info,
        );
    }
    let failed = state
        .last_failure_token
        .is_some_and(|token| selected.matches_token(project_id, token));
    match veriloga_receipt(state, project_id, selected) {
        None if failed => (messages.text(MessageId::VerilogAStatusFailed), t.color.err),
        None => (
            messages.text(MessageId::VerilogAStatusModified),
            t.color.warn,
        ),
        // The module compiled, but it is not the cell its schematic defines.
        Some(VerilogACompileReceipt {
            interface: Some(VerilogAInterfaceOutcome::SchematicMismatch { .. }),
            ..
        }) => (
            messages.text(MessageId::VerilogAStatusPortsDiffer),
            t.color.err,
        ),
        Some(receipt) => {
            let summary = receipt.diagnostics.summary();
            match summary.total().saturating_sub(summary.errors) {
                0 => (messages.text(MessageId::VerilogAStatusPass), t.color.ok),
                advisories => {
                    let count = advisories.to_string();
                    (
                        messages.format(
                            if advisories == 1 {
                                MessageId::VerilogAStatusPassAdvisorySingular
                            } else {
                                MessageId::VerilogAStatusPassAdvisories
                            },
                            &[("count", &count)],
                        ),
                        t.color.warn,
                    )
                }
            }
        }
    }
}

/// What the reader asked the inspector to do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VerilogAInspectorAction {
    /// Reconcile the cell's hand-edited symbol with the module's ports.
    UpdateSymbol,
}

/// The inspector body: module identity, build targets, diagnostics, and the
/// ABI contract the module exposes to a netlist.
pub(crate) fn show_veriloga_inspector(
    ui: &mut Ui,
    state: &VerilogAWorkbenchState,
    project_id: ProjectId,
    selected: &SelectedVerilogASource,
    messages: MessageCatalog,
) -> Option<VerilogAInspectorAction> {
    let t = Tokens::get(ui.ctx());
    ui.set_width(ui.available_width());
    let receipt = veriloga_receipt(state, project_id, selected);
    let action = module_section(ui, state, project_id, selected, receipt, messages, &t);
    build_targets_section(ui, receipt, messages, &t);
    // Diagnostics exist only for a source the compiler has read. With neither
    // a receipt nor a refusal for this exact revision, an empty list is not a
    // clean bill: nothing has looked yet.
    let compiled = receipt.is_some()
        || state
            .last_failure_token
            .is_some_and(|token| selected.matches_token(project_id, token));
    diagnostics_section(
        ui,
        compiled.then(|| veriloga_diagnostics(state, project_id, selected)),
        messages,
    );
    parameters_section(ui, receipt, messages, &t);
    abi_section(ui, receipt, messages);
    action
}

/// Height of one row of the parameter table, and of its column captions.
const PARAMETER_ROW_H: f32 = 22.0;
const PARAMETER_CAPTION_H: f32 = 20.0;
/// Gap between two columns of the parameter table.
const PARAMETER_COLUMN_GAP: f32 = 8.0;
/// Inset of the table from the section's edges, matching the property rows'.
const PARAMETER_TABLE_PAD: f32 = 10.0;
/// Space above the captions and below the last row.
const PARAMETER_TABLE_INSET_Y: f32 = 6.0;

/// The parameters an instance of the compiled module takes, one dense row
/// each: name, type, default, range and units, with the module's description
/// of the parameter on hover. These are exactly what the compile writes as the
/// view's parameter contract, so the table and an instance's property sheet
/// cannot disagree.
fn parameters_section(
    ui: &mut Ui,
    receipt: Option<&VerilogACompileReceipt>,
    messages: MessageCatalog,
    t: &Tokens,
) {
    let parameters = receipt.map(|receipt| compiled_parameters(&receipt.compilation.report().abi));
    let required = messages.text(MessageId::VerilogACompileRequired);
    let count = parameters
        .as_ref()
        .map(|parameters| parameters.len().to_string());
    let (status, problem) = match count.as_deref() {
        Some(count) => (count, None),
        None => (required.as_str(), Some(Problem::Warning)),
    };
    code_inspector_section(
        ui,
        &messages.text(MessageId::VerilogAParameters),
        Some(status),
        problem,
        |ui| match parameters.as_deref() {
            None => muted_row(ui, &messages.text(MessageId::VerilogAParametersNotCompiled)),
            Some([]) => muted_row(ui, &messages.text(MessageId::VerilogANoParameters)),
            Some(parameters) => parameter_table(ui, parameters, messages, t),
        },
    );
}

/// One parameter's cells, as the table shows them.
struct ParameterCells {
    name: String,
    kind: String,
    default: String,
    range: String,
    units: String,
    description: Option<String>,
}

impl ParameterCells {
    fn of(parameter: &CellParameter, messages: MessageCatalog) -> Self {
        let declaration = parameter.declaration.as_ref();
        let kind = match declaration.map(|declaration| declaration.value_type) {
            Some(ParameterValueType::Integer) => MessageId::VerilogAParameterTypeInteger,
            Some(ParameterValueType::String) => MessageId::VerilogAParameterTypeString,
            _ => MessageId::VerilogAParameterTypeReal,
        };
        // In a narrow column the interval reads without its `from`.
        let range = declaration
            .and_then(|declaration| declaration.range.as_ref())
            .map(|range| {
                let text = range.source_text();
                text.strip_prefix("from ")
                    .map_or(text.clone(), str::to_owned)
            })
            .unwrap_or_else(|| messages.text(MessageId::VerilogAParameterAnyValue));
        Self {
            name: parameter.name.clone(),
            kind: messages.text(kind),
            default: parameter
                .default_value
                .clone()
                .unwrap_or_else(|| messages.text(MessageId::VerilogAParameterComputed)),
            range,
            units: declaration
                .and_then(|declaration| declaration.units.clone())
                .unwrap_or_default(),
            description: declaration
                .and_then(|declaration| declaration.description.clone())
                .filter(|description| !description.trim().is_empty()),
        }
    }

    fn columns(&self) -> [&str; 5] {
        [
            &self.name,
            &self.kind,
            &self.default,
            &self.range,
            &self.units,
        ]
    }
}

/// Narrowest a column is narrowed to before the next one gives way.
const PARAMETER_COLUMN_MIN_W: f32 = 36.0;

/// Column widths for a table `available` wide: each column as wide as its
/// widest cell and the spare width to the range. When the cells do not fit,
/// the range gives way first, since its full text is on hover; then the
/// default, the name, and the type, each no narrower than a few characters.
fn parameter_column_widths(natural: [f32; 5], available: f32) -> [f32; 5] {
    let room = (available - PARAMETER_COLUMN_GAP * 4.0).max(0.0);
    let mut widths = natural;
    let total: f32 = natural.iter().sum();
    if total <= room {
        widths[3] += room - total;
        return widths;
    }
    let mut excess = total - room;
    for column in [3, 2, 0, 1] {
        let give = (widths[column] - PARAMETER_COLUMN_MIN_W)
            .max(0.0)
            .min(excess);
        widths[column] -= give;
        excess -= give;
    }
    widths
}

fn parameter_table(
    ui: &mut Ui,
    parameters: &[CellParameter],
    messages: MessageCatalog,
    t: &Tokens,
) {
    let mono = theme::mono(tokens::FS_0, FontWeight::Regular);
    let sans = theme::sans(tokens::FS_0, FontWeight::Regular);
    let caption = theme::sans(tokens::FS_MICRO, FontWeight::Medium);
    let cells = parameters
        .iter()
        .map(|parameter| ParameterCells::of(parameter, messages))
        .collect::<Vec<_>>();
    let captions = [
        MessageId::VerilogAParameterName,
        MessageId::VerilogAParameterType,
        MessageId::VerilogAParameterDefault,
        MessageId::VerilogAParameterRange,
        MessageId::VerilogAParameterUnits,
    ]
    .map(|id| messages.text(id).to_uppercase());
    // Units are painted in the sans face: the mono face has no `Ω`.
    let font_of = |column: usize| if column == 4 { &sans } else { &mono };
    let measure = |text: &str, font: &egui::FontId| {
        ui.painter()
            .layout_no_wrap(text.to_owned(), font.clone(), t.color.text)
            .size()
            .x
    };
    let mut natural = [0.0_f32; 5];
    for (column, width) in natural.iter_mut().enumerate() {
        *width = cells
            .iter()
            .map(|row| measure(row.columns()[column], font_of(column)))
            .fold(measure(&captions[column], &caption), f32::max);
    }
    let width = ui.available_width();
    let widths = parameter_column_widths(natural, width - 2.0 * PARAMETER_TABLE_PAD);
    let lefts = {
        let mut left = PARAMETER_TABLE_PAD;
        widths.map(|width| {
            let start = left;
            left += width + PARAMETER_COLUMN_GAP;
            start
        })
    };
    let paint_cells = |ui: &Ui,
                       rect: egui::Rect,
                       texts: [&str; 5],
                       fonts: [&egui::FontId; 5],
                       colors: [Color32; 5]| {
        for column in 0..5 {
            let cell = egui::Rect::from_min_size(
                egui::pos2(rect.left() + lefts[column], rect.top()),
                Vec2::new(widths[column], rect.height()),
            );
            let shown = crate::workbench::design_system::elide_text(
                ui,
                texts[column],
                fonts[column],
                cell.width(),
            );
            ui.painter().with_clip_rect(cell).text(
                egui::pos2(cell.left(), cell.center().y),
                egui::Align2::LEFT_CENTER,
                shown,
                fonts[column].clone(),
                colors[column],
            );
        }
    };

    ui.add_space(PARAMETER_TABLE_INSET_Y);
    let (header, _) = ui.allocate_exact_size(Vec2::new(width, PARAMETER_CAPTION_H), Sense::hover());
    paint_cells(
        ui,
        header,
        [
            &captions[0],
            &captions[1],
            &captions[2],
            &captions[3],
            &captions[4],
        ],
        [&caption; 5],
        [t.color.text_faint; 5],
    );
    for row in &cells {
        let (rect, response) =
            ui.allocate_exact_size(Vec2::new(width, PARAMETER_ROW_H), Sense::hover());
        if response.hovered() {
            ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
        }
        paint_cells(
            ui,
            rect,
            row.columns(),
            [&mono, &mono, &mono, &mono, &sans],
            [
                t.color.text,
                t.color.text_dim,
                t.color.text_dim,
                t.color.text_dim,
                t.color.text_dim,
            ],
        );
        let summary = [
            row.kind.as_str(),
            row.default.as_str(),
            row.range.as_str(),
            row.units.as_str(),
        ]
        .into_iter()
        .filter(|cell| !cell.is_empty())
        .collect::<Vec<_>>()
        .join(" · ");
        let spoken = format!("{}: {summary}", row.name);
        response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::Label, true, spoken.clone())
        });
        let _ = response.on_hover_text(match row.description.as_deref() {
            Some(description) => format!("{spoken}\n{description}"),
            None => spoken.clone(),
        });
    }
    ui.add_space(PARAMETER_TABLE_INSET_Y);
}

fn module_section(
    ui: &mut Ui,
    state: &VerilogAWorkbenchState,
    project_id: ProjectId,
    selected: &SelectedVerilogASource,
    receipt: Option<&VerilogACompileReceipt>,
    messages: MessageCatalog,
    t: &Tokens,
) -> Option<VerilogAInspectorAction> {
    let mut action = None;
    // The module a compile actually elaborated outranks the one the bundle
    // asks for: with no explicit entry the compiler chooses, and the receipt
    // is the record of what it chose.
    let module = receipt
        .map(|receipt| receipt.compilation.report().abi.module_name.to_string())
        .or_else(|| selected.selected_module().map(str::to_owned))
        .unwrap_or_else(|| messages.text(MessageId::VerilogACompileReviewAutomaticModule));
    let (status, tone) = veriloga_compile_status(state, project_id, selected, messages, t);
    code_inspector_section(
        ui,
        &messages.text(MessageId::VerilogAModuleSection),
        None,
        None,
        |ui| {
            code_inspector_property_list(ui, |ui| {
                property_row(
                    ui,
                    &messages.text(MessageId::VerilogACompileReviewEntryModule),
                    &module,
                );
                if let crate::state::ProjectSourceOwner::CellView { reference } =
                    selected.bundle().owner()
                {
                    property_row(
                        ui,
                        &messages.text(MessageId::VerilogACellView),
                        &reference.display_path(),
                    );
                }
                property_row(
                    ui,
                    &messages.text(MessageId::VerilogARootSource),
                    selected.document().logical_path(),
                );
                property_row_toned(
                    ui,
                    &messages.text(MessageId::VerilogACompileStatus),
                    &status,
                    tone,
                );
                if let crate::state::ProjectSourceOwner::CellView { reference } =
                    selected.bundle().owner()
                    && interface_rows(ui, state, receipt, reference, messages, t)
                {
                    action = Some(VerilogAInspectorAction::UpdateSymbol);
                }
            });
        },
    );
    action
}

/// Which view defines the cell's ports, and where its symbol stands against
/// them. Returns whether Update symbol was pressed.
fn interface_rows(
    ui: &mut Ui,
    state: &VerilogAWorkbenchState,
    receipt: Option<&VerilogACompileReceipt>,
    reference: &crate::state::CellViewRef,
    messages: MessageCatalog,
    t: &Tokens,
) -> bool {
    let symbol = messages.text(MessageId::VerilogASymbolRow);
    match state.symbol_standing.of(reference) {
        None => false,
        Some(VerilogACellSymbol::SchematicDefined) => {
            let differs = receipt.is_some_and(|receipt| {
                matches!(
                    receipt.interface,
                    Some(VerilogAInterfaceOutcome::SchematicMismatch { .. })
                )
            });
            let (value, tone) = if differs {
                (MessageId::VerilogAPortsDifferFromSchematic, t.color.err)
            } else {
                (MessageId::VerilogAPortsFromSchematic, t.color.text)
            };
            property_row_toned(
                ui,
                &messages.text(MessageId::VerilogAPortsRow),
                &messages.text(value),
                tone,
            );
            false
        }
        Some(VerilogACellSymbol::None) => {
            property_row(ui, &symbol, &messages.text(MessageId::VerilogASymbolNone));
            false
        }
        Some(VerilogACellSymbol::Generated) => {
            property_row(
                ui,
                &symbol,
                &messages.text(MessageId::VerilogASymbolGenerated),
            );
            false
        }
        Some(VerilogACellSymbol::InStep) => {
            property_row(ui, &symbol, &messages.text(MessageId::VerilogASymbolInStep));
            false
        }
        Some(VerilogACellSymbol::Drifted(drift)) => {
            property_row_toned(
                ui,
                &symbol,
                &symbol_drift_text(drift, messages),
                t.color.warn,
            );
            value_column_button(ui, &messages.text(MessageId::VerilogAUpdateSymbol))
        }
    }
}

/// "2 pins missing, 1 extra", and how many pins turned round when any did.
pub(crate) fn symbol_drift_text(drift: &VerilogASymbolDrift, messages: MessageCatalog) -> String {
    let count = |items: &[String]| items.len().to_string();
    let missing = count(&drift.missing);
    let mut text = messages.format(
        if drift.missing.len() == 1 {
            MessageId::VerilogASymbolPinMissing
        } else {
            MessageId::VerilogASymbolPinsMissing
        },
        &[("missing", &missing), ("extra", &count(&drift.extra))],
    );
    if !drift.turned.is_empty() {
        text.push_str(&messages.format(
            MessageId::VerilogASymbolPinsTurned,
            &[("count", &count(&drift.turned))],
        ));
    }
    text
}

/// A button in the value column, under the fact it acts on, so the action
/// stays on the inspector's label/value grid.
fn value_column_button(ui: &mut Ui, label: &str) -> bool {
    let width = ui.available_width();
    let (_, value_left) = property_row_control_columns(width);
    let height = property_row_height(ui) + 6.0;
    ui.allocate_ui_with_layout(
        Vec2::new(width, height),
        Layout::left_to_right(Align::Center),
        |ui| {
            ui.set_min_size(Vec2::new(width, height));
            ui.add_space(value_left);
            Button::new(label).show(ui).clicked()
        },
    )
    .inner
}

fn build_targets_section(
    ui: &mut Ui,
    receipt: Option<&VerilogACompileReceipt>,
    messages: MessageCatalog,
    t: &Tokens,
) {
    code_inspector_section(
        ui,
        &messages.text(MessageId::VerilogABuildTargets),
        None,
        None,
        |ui| {
            code_inspector_property_list(ui, |ui| {
                let Some(receipt) = receipt else {
                    let required = messages.text(MessageId::VerilogACompileRequired);
                    for label in [
                        MessageId::VerilogASemanticIr,
                        MessageId::VerilogABytecodeVm,
                        MessageId::VerilogANativeJit,
                        MessageId::VerilogAWasmInterpreter,
                        MessageId::VerilogAGeneratedRust,
                    ] {
                        property_row(ui, &messages.text(label), &required);
                    }
                    return;
                };
                property_row_status(
                    ui,
                    &messages.text(MessageId::VerilogASemanticIr),
                    &messages.text(MessageId::VerilogACanonical),
                    t.color.ok,
                    StatusMark::Success,
                );
                if receipt.bytecode_available {
                    property_row_status(
                        ui,
                        &messages.text(MessageId::VerilogABytecodeVm),
                        &messages.text(MessageId::VerilogAAvailable),
                        t.color.ok,
                        StatusMark::Success,
                    );
                } else {
                    property_row_status(
                        ui,
                        &messages.text(MessageId::VerilogABytecodeVm),
                        &messages.text(MessageId::VerilogAUnavailable),
                        t.color.err,
                        StatusMark::Failure,
                    );
                }
                target_row(
                    ui,
                    &messages.text(MessageId::VerilogANativeJit),
                    &receipt.native_jit,
                    messages,
                );
                target_row(
                    ui,
                    &messages.text(MessageId::VerilogAWasmInterpreter),
                    &receipt.wasm_interpreter,
                    messages,
                );
                target_row(
                    ui,
                    &messages.text(MessageId::VerilogAGeneratedRust),
                    &receipt.generated_rust,
                    messages,
                );
            });
        },
    );
}

fn diagnostics_section(
    ui: &mut Ui,
    diagnostics: Option<Arc<CodeDiagnosticCollection>>,
    messages: MessageCatalog,
) {
    const INSPECTOR_DIAGNOSTIC_PREVIEW: usize = 50;

    let Some(diagnostics) = diagnostics else {
        code_inspector_section(
            ui,
            &messages.text(MessageId::VerilogADiagnostics),
            Some(&messages.text(MessageId::VerilogACompileRequired)),
            Some(Problem::Warning),
            |ui| {
                muted_row(
                    ui,
                    &messages.text(MessageId::VerilogADiagnosticsNotCompiled),
                )
            },
        );
        return;
    };
    let summary = diagnostics.summary();
    let error_count = summary.errors;
    let advisory_count = summary.total().saturating_sub(error_count);
    let (status, problem) = if error_count > 0 {
        let count = error_count.to_string();
        (
            messages.format(
                if error_count == 1 {
                    MessageId::VerilogAErrorSingular
                } else {
                    MessageId::VerilogAErrors
                },
                &[("count", &count)],
            ),
            Some(Problem::Failure),
        )
    } else if advisory_count > 0 {
        let count = advisory_count.to_string();
        (
            messages.format(
                if advisory_count == 1 {
                    MessageId::VerilogAAdvisorySingular
                } else {
                    MessageId::VerilogAAdvisories
                },
                &[("count", &count)],
            ),
            Some(Problem::Warning),
        )
    } else {
        (messages.text(MessageId::VerilogAClean), None)
    };
    code_inspector_section(
        ui,
        &messages.text(MessageId::VerilogADiagnostics),
        Some(&status),
        problem,
        |ui| {
            if diagnostics.is_empty() {
                muted_row(ui, &messages.text(MessageId::VerilogANoDiagnostics));
                return;
            }
            for diagnostic in diagnostics.iter().take(INSPECTOR_DIAGNOSTIC_PREVIEW) {
                diagnostic_row(ui, diagnostic);
            }
            if diagnostics.len() > INSPECTOR_DIAGNOSTIC_PREVIEW {
                let remaining = (diagnostics.len() - INSPECTOR_DIAGNOSTIC_PREVIEW).to_string();
                muted_row(
                    ui,
                    &messages.format(MessageId::VerilogADiagnosticsMore, &[("count", &remaining)]),
                );
            }
        },
    );
}

fn abi_section(ui: &mut Ui, receipt: Option<&VerilogACompileReceipt>, messages: MessageCatalog) {
    code_inspector_section(
        ui,
        &messages.text(MessageId::VerilogAAbiContract),
        None,
        None,
        |ui| {
            code_inspector_property_list(ui, |ui| {
                // Counts exist only for a module that compiled. Before that a
                // dash, not a zero: an uncompiled module has no contract yet,
                // which is different from a contract with nothing in it.
                // The parameters have a section of their own above.
                let counts = receipt.map(|receipt| {
                    [
                        receipt.analog_ports,
                        receipt.noise_sources,
                        receipt.state_variables,
                    ]
                });
                for (index, label) in [
                    MessageId::VerilogAAnalogPorts,
                    MessageId::VerilogANoiseSources,
                    MessageId::VerilogAStateVariables,
                ]
                .into_iter()
                .enumerate()
                {
                    let value = counts
                        .map_or_else(|| "\u{2014}".to_owned(), |counts| counts[index].to_string());
                    property_row(ui, &messages.text(label), &value);
                }
            });
        },
    );
}

fn target_row(ui: &mut Ui, label: &str, target: &TargetQualification, messages: MessageCatalog) {
    let t = Tokens::get(ui.ctx());
    let (value, tone, mark) = match target {
        TargetQualification::Available => (
            messages.text(MessageId::VerilogAAvailable),
            t.color.ok,
            Some(StatusMark::Success),
        ),
        TargetQualification::Preview => (
            messages.text(MessageId::VerilogAPreview),
            t.color.warn,
            Some(StatusMark::Warning),
        ),
        TargetQualification::QualificationOnly => (
            messages.text(MessageId::VerilogAQualificationOnly),
            t.color.text,
            None,
        ),
        TargetQualification::Unsupported(reason) => (
            messages.format(MessageId::VerilogAUnsupported, &[("reason", reason)]),
            t.color.err,
            Some(StatusMark::Failure),
        ),
        TargetQualification::Failed(reason) => (
            messages.format(MessageId::VerilogAFailed, &[("reason", reason)]),
            t.color.err,
            Some(StatusMark::Failure),
        ),
    };
    if let Some(mark) = mark {
        property_row_status(ui, label, &value, tone, mark);
    } else {
        property_row_toned(ui, label, &value, tone);
    }
}

fn muted_row(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .inner_margin(egui::Margin::symmetric(9, 8))
        .show(ui, |ui| {
            ui.add(
                egui::Label::new(
                    egui::RichText::new(text)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                )
                .wrap(),
            );
        });
}

fn diagnostic_row(ui: &mut Ui, diagnostic: &CodeEditorDiagnostic) {
    let t = Tokens::get(ui.ctx());
    let tone = match diagnostic.severity {
        CodeEditorSeverity::Hint | CodeEditorSeverity::Info => t.color.info,
        CodeEditorSeverity::Warning => t.color.warn,
        CodeEditorSeverity::Error => t.color.err,
    };
    let shown = egui::Frame::new()
        .inner_margin(egui::Margin::symmetric(9, 8))
        .show(ui, |ui| {
            ui.horizontal_top(|ui| {
                ui.spacing_mut().item_spacing.x = 7.0;
                let (icon, _) = ui.allocate_exact_size(Vec2::splat(14.0), Sense::hover());
                WorkbenchIcon::Warning.paint(ui.painter(), icon, tone);
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = 3.0;
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(diagnostic.message.as_ref())
                                .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                                .color(t.color.text),
                        )
                        .wrap(),
                    );
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(diagnostic.detail.as_ref())
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.text_faint),
                        )
                        .wrap(),
                    );
                    // The producer's code closes the row on its own baseline.
                    // A code is identity, not prose: sharing a line with the
                    // message or the detail would squeeze whichever of them
                    // the reader actually came for.
                    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                        ui.label(
                            egui::RichText::new(diagnostic.code())
                                .font(theme::mono(tokens::FS_MICRO, FontWeight::Regular))
                                .color(t.color.text_faint),
                        );
                    });
                });
            });
        });
    ui.painter().hline(
        shown.response.rect.x_range(),
        shown.response.rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A table that fits gives its spare width to the range; one that does
    /// not narrows the range first, and a parameter's name last but one.
    #[test]
    fn the_range_gives_way_before_the_name_does() {
        let natural = [40.0, 46.0, 53.0, 105.0, 28.0];
        let gaps = PARAMETER_COLUMN_GAP * 4.0;
        let roomy = parameter_column_widths(natural, 300.0 + gaps);
        assert_eq!(roomy, [40.0, 46.0, 53.0, 133.0, 28.0]);

        let tight = parameter_column_widths(natural, 258.0 + gaps);
        assert_eq!(tight, [40.0, 46.0, 53.0, 91.0, 28.0]);

        // Range, default and name each down to a few characters, and the
        // last two points from the type.
        let cramped = parameter_column_widths(natural, 180.0 + gaps);
        assert_eq!(cramped, [36.0, 44.0, 36.0, 36.0, 28.0]);
    }
}
