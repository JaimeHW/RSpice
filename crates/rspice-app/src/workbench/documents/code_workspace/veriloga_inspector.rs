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
    Problem, WorkbenchIcon, code_inspector_section, paint_problem_mark, panel_bleed, panel_inset,
    problem_line, property_row, property_row_control_columns, property_row_height,
    property_row_problem,
};
use crate::workbench::{MessageCatalog, MessageId};

use rspice_design::library::metadata::{CellParameter, ParameterValueType};

use super::page::VerilogAWorkbenchState;
use super::veriloga_interface::{
    VerilogACellSymbol, VerilogAInterfaceOutcome, VerilogASymbolDrift, compiled_parameters,
};
use super::{
    CodeDiagnosticCollection, CodeEditorDiagnostic, CodeEditorSeverity, SelectedVerilogASource,
    VerilogACompileReceipt,
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

/// One phrase for where the source stands with the compiler, and whether the
/// reader has to act on it.
///
/// The editor's status strip and the inspector's status row both read this,
/// so the two can never disagree about whether the visible text compiled.
/// Compiling and edited-since-the-last-compile are where the source is, not
/// something wrong with it, so they carry no mark; a failure, ports that
/// disagree with the schematic, and advisories left in Diagnostics do.
pub(crate) fn veriloga_compile_status(
    state: &VerilogAWorkbenchState,
    project_id: ProjectId,
    selected: &SelectedVerilogASource,
    messages: MessageCatalog,
) -> Option<(String, Option<Problem>)> {
    if state.pending.is_some() {
        return Some((messages.text(MessageId::VerilogAStatusCompiling), None));
    }
    let failed = state
        .last_failure_token
        .is_some_and(|token| selected.matches_token(project_id, token));
    match veriloga_receipt(state, project_id, selected) {
        None if failed => Some((
            messages.text(MessageId::VerilogAStatusFailed),
            Some(Problem::Failure),
        )),
        None => Some((messages.text(MessageId::VerilogAStatusModified), None)),
        // The module compiled, but it is not the cell its schematic defines.
        Some(VerilogACompileReceipt {
            interface: Some(VerilogAInterfaceOutcome::SchematicMismatch { .. }),
            ..
        }) => Some((
            messages.text(MessageId::VerilogAStatusPortsDiffer),
            Some(Problem::Failure),
        )),
        Some(receipt) => {
            let summary = receipt.diagnostics.summary();
            match summary.total().saturating_sub(summary.errors) {
                // A clean compile is the state the reader expects. Saying so
                // is a success report, so the strip says nothing instead.
                0 => None,
                advisories => {
                    let count = advisories.to_string();
                    Some((
                        messages.format(
                            if advisories == 1 {
                                MessageId::VerilogAStatusPassAdvisorySingular
                            } else {
                                MessageId::VerilogAStatusPassAdvisories
                            },
                            &[("count", &count)],
                        ),
                        Some(Problem::Warning),
                    ))
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

/// The inspector body: the module and where its ports stand, what the
/// compiler found in it, and the parameters an instance of it takes. Where
/// the source stands with the compiler is the editor strip's to say.
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
    let action = module_section(ui, state, selected, receipt, messages);
    // Diagnostics exist only for a source the compiler has read. With neither
    // a receipt nor a refusal for this exact revision there is nothing to
    // list, and the strip already says the source needs a compile.
    let compiled = receipt.is_some()
        || state
            .last_failure_token
            .is_some_and(|token| selected.matches_token(project_id, token));
    if compiled {
        diagnostics_section(
            ui,
            &veriloga_diagnostics(state, project_id, selected),
            messages,
        );
    }
    parameters_section(ui, receipt, messages, &t);
    action
}

/// Gap between two columns of the parameter table. The table's rows are panel
/// rows: one row tall, on the panel inset.
const PARAMETER_COLUMN_GAP: f32 = 8.0;

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
    // A module that takes no parameters has no section to show it.
    if parameters
        .as_deref()
        .is_some_and(<[CellParameter]>::is_empty)
    {
        return;
    }
    let required = messages.text(MessageId::VerilogACompileRequired);
    let count = parameters
        .as_ref()
        .map(|parameters| parameters.len().to_string());
    // Not compiled yet is where the module is, not a fault in it.
    let status = count.as_deref().unwrap_or(required.as_str());
    code_inspector_section(
        ui,
        &messages.text(MessageId::VerilogAParameters),
        Some(status),
        None,
        |ui| match parameters.as_deref() {
            None => muted_row(ui, &messages.text(MessageId::VerilogAParametersNotCompiled)),
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
    .map(|id| messages.text(id));
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
    let pad = panel_inset(ui);
    let row_height = property_row_height(ui);
    let widths = parameter_column_widths(natural, width - 2.0 * pad);
    let lefts = {
        let mut left = pad;
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

    let (header, _) = ui.allocate_exact_size(Vec2::new(width, row_height), Sense::hover());
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
        let (rect, response) = ui.allocate_exact_size(Vec2::new(width, row_height), Sense::hover());
        if response.hovered() {
            ui.painter()
                .rect_filled(panel_bleed(ui, rect), 0.0, t.color.bg_hover);
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
}

fn module_section(
    ui: &mut Ui,
    state: &VerilogAWorkbenchState,
    selected: &SelectedVerilogASource,
    receipt: Option<&VerilogACompileReceipt>,
    messages: MessageCatalog,
) -> Option<VerilogAInspectorAction> {
    let mut action = None;
    // The module a compile actually elaborated outranks the one the bundle
    // asks for: with no explicit entry the compiler chooses, and the receipt
    // is the record of what it chose.
    let module = receipt
        .map(|receipt| receipt.compilation.report().abi.module_name.to_string())
        .or_else(|| selected.selected_module().map(str::to_owned))
        .unwrap_or_else(|| messages.text(MessageId::VerilogAModuleAuto));
    code_inspector_section(
        ui,
        &messages.text(MessageId::VerilogAModuleSection),
        None,
        None,
        |ui| {
            property_row(ui, &messages.text(MessageId::VerilogAModuleRow), &module);
            // A compile that produced no bytecode leaves nothing a run can
            // execute; every other build target is the engine's concern.
            if receipt.is_some_and(|receipt| !receipt.bytecode_available) {
                problem_line(
                    ui,
                    Problem::Failure,
                    &messages.text(MessageId::VerilogANoBytecode),
                );
            }
            if let crate::state::ProjectSourceOwner::CellView { reference } =
                selected.bundle().owner()
                && interface_rows(ui, state, receipt, reference, messages)
            {
                action = Some(VerilogAInspectorAction::UpdateSymbol);
            }
        },
    );
    action
}

/// Where the module's ports stand against the cell's: a row only when they
/// disagree with the schematic that defines them, or when a hand-edited
/// symbol has fallen behind them. Returns whether Update symbol was pressed.
fn interface_rows(
    ui: &mut Ui,
    state: &VerilogAWorkbenchState,
    receipt: Option<&VerilogACompileReceipt>,
    reference: &crate::state::CellViewRef,
    messages: MessageCatalog,
) -> bool {
    match state.symbol_standing.of(reference) {
        Some(VerilogACellSymbol::SchematicDefined) => {
            let differs = receipt.is_some_and(|receipt| {
                matches!(
                    receipt.interface,
                    Some(VerilogAInterfaceOutcome::SchematicMismatch { .. })
                )
            });
            if differs {
                property_row_problem(
                    ui,
                    &messages.text(MessageId::VerilogAPortsRow),
                    &messages.text(MessageId::VerilogAPortsDifferFromSchematic),
                    Some(Problem::Failure),
                );
            }
            false
        }
        Some(VerilogACellSymbol::Drifted(drift)) => {
            property_row(
                ui,
                &messages.text(MessageId::VerilogASymbolRow),
                &symbol_drift_text(drift, messages),
            );
            value_column_button(ui, &messages.text(MessageId::VerilogAUpdateSymbol))
        }
        None
        | Some(
            VerilogACellSymbol::None | VerilogACellSymbol::Generated | VerilogACellSymbol::InStep,
        ) => false,
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
    let (_, value_left) = property_row_control_columns(ui, width);
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

/// What the last compile of this source found. A clean compile has nothing to
/// list, so it has no section.
fn diagnostics_section(
    ui: &mut Ui,
    diagnostics: &CodeDiagnosticCollection,
    messages: MessageCatalog,
) {
    const INSPECTOR_DIAGNOSTIC_PREVIEW: usize = 50;

    if diagnostics.is_empty() {
        return;
    }
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
    } else {
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
    };
    code_inspector_section(
        ui,
        &messages.text(MessageId::VerilogADiagnostics),
        Some(&status),
        problem,
        |ui| {
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

fn muted_row(ui: &mut Ui, text: &str) {
    crate::workbench::design_system::inspector::muted_inspector_copy(ui, text);
}

/// Space above and below a diagnostic's two lines, inside the rule under it.
const DIAGNOSTIC_PAD_Y: i8 = 8;

fn diagnostic_row(ui: &mut Ui, diagnostic: &CodeEditorDiagnostic) {
    /// The side of the kit's problem mark, so a row's mark matches the one in
    /// the section header above it.
    use crate::ui::widgets::PROBLEM_MARK_SIDE as MARK_SIDE;

    let t = Tokens::get(ui.ctx());
    // On the panel inset: the body has set it already, so the row adds none.
    let inset = panel_inset(ui) as i8;
    let shown = egui::Frame::new()
        .inner_margin(egui::Margin::symmetric(inset, DIAGNOSTIC_PAD_Y))
        .show(ui, |ui| {
            ui.horizontal_top(|ui| {
                ui.spacing_mut().item_spacing.x = 7.0;
                let (icon, _) = ui.allocate_exact_size(Vec2::splat(14.0), Sense::hover());
                let mark = egui::Rect::from_center_size(icon.center(), Vec2::splat(MARK_SIDE));
                match diagnostic.severity {
                    CodeEditorSeverity::Error => {
                        paint_problem_mark(ui.painter(), mark, Problem::Failure);
                    }
                    CodeEditorSeverity::Warning => {
                        paint_problem_mark(ui.painter(), mark, Problem::Warning);
                    }
                    CodeEditorSeverity::Hint | CodeEditorSeverity::Info => {
                        WorkbenchIcon::Warning.paint(ui.painter(), icon, t.color.text_faint);
                    }
                }
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
