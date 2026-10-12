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
    StatusMark, WorkbenchIcon, code_inspector_property_list, code_inspector_section, property_row,
    property_row_control_columns, property_row_height, property_row_status, property_row_toned,
};
use crate::workbench::{MessageCatalog, MessageId};

use super::page::VerilogAWorkbenchState;
use super::veriloga_interface::{
    VerilogACellSymbol, VerilogAInterfaceOutcome, VerilogASymbolDrift,
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
        &t,
    );
    abi_section(ui, receipt, messages);
    action
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
    t: &Tokens,
) {
    const INSPECTOR_DIAGNOSTIC_PREVIEW: usize = 50;

    let Some(diagnostics) = diagnostics else {
        code_inspector_section(
            ui,
            &messages.text(MessageId::VerilogADiagnostics),
            Some((
                &messages.text(MessageId::VerilogACompileRequired),
                t.color.warn,
            )),
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
    let (status, tone) = if error_count > 0 {
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
            t.color.err,
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
            t.color.warn,
        )
    } else {
        (messages.text(MessageId::VerilogAClean), t.color.ok)
    };
    code_inspector_section(
        ui,
        &messages.text(MessageId::VerilogADiagnostics),
        Some((&status, tone)),
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
        |ui| {
            code_inspector_property_list(ui, |ui| {
                // Counts exist only for a module that compiled. Before that a
                // dash, not a zero: an uncompiled module has no contract yet,
                // which is different from a contract with nothing in it.
                let counts = receipt.map(|receipt| {
                    [
                        receipt.analog_ports,
                        receipt.compilation.report().abi.parameters.len(),
                        receipt.noise_sources,
                        receipt.state_variables,
                    ]
                });
                for (index, label) in [
                    MessageId::VerilogAAnalogPorts,
                    MessageId::VerilogAParameters,
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
