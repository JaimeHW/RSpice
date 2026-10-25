//! The sheets feature: the drawing sheet a schematic is drawn on, its
//! properties, formats and sizes, and the sheets of one drawing.
//!
//! Its commands and its dialogs run in a [`Cx`] over the feature's own dialog
//! state, the model and the session. Whatever the open drawing is, the model
//! answers; design's review markers, which the format manager's previews and
//! the overflow review show, are read through the
//! [`DesignPort`](crate::workbench::framework::feature::DesignPort).

pub(crate) mod commands;
#[cfg(test)]
mod tests;

use egui::Context;

use crate::state::ViewType;
use crate::workbench::app::dialogs::delete_sheet::render_delete_sheet_confirmation;
use crate::workbench::app::dialogs::drawing_sheet_defaults::render_drawing_sheet_defaults_dialog;
use crate::workbench::app::dialogs::drawing_sheet_overflow::{
    OverflowFollowUp, render_drawing_sheet_overflow_dialog,
};
use crate::workbench::app::dialogs::drawing_sheet_presets::PresetDialogs;
use crate::workbench::app::dialogs::drawing_sheet_support::render_drawing_sheet_support_dialogs;
use crate::workbench::app::drawing_sheet_edit::drawing_sheet_editable;
use crate::workbench::app::sheets as sheet_actions;
use crate::workbench::app::{
    open_custom_sheet_size_library, open_drawing_sheet_defaults, open_sheet_format_manager,
};
use crate::workbench::app_state::Model;
use crate::workbench::docks::inspector::design::sheet::{
    SheetInspector, commit_pending_sheet_text, sheet_panel,
};
use crate::workbench::framework::command_registry::{CommandAvailability, CommandSpec, ids};
use crate::workbench::framework::dialogs::DialogKey;
use crate::workbench::framework::feature::{
    CommandSet, Cx, Feature, FeatureDialog, FeatureInspector, FeatureRegistry, Reads, Session,
};
use crate::workbench::framework::requests::PrintActiveView;
use crate::workbench::slots::SheetsState;
use crate::workbench::state::{WorkbenchSession, Workspace};

use commands::SheetsCommand;

/// The sheets feature.
pub(crate) struct SheetsFeature;

impl Feature for SheetsFeature {
    type State = SheetsState;
    type Session = WorkbenchSession;

    fn register(registry: &mut FeatureRegistry<Self>) {
        registry.commands::<SheetsCommand>();
        registry.dialogs(DIALOGS);
        registry.inspector(FeatureInspector {
            show: |cx, ui| {
                let review_markers = cx.peers.design().review_markers();
                sheet_panel(
                    ui,
                    SheetInspector {
                        model: cx.model,
                        workbench: cx.wb,
                        reveal_sheet_section: &mut cx.state.reveal_sheet_section,
                        overflow_review_open: &mut cx.state.drawing_sheet_overflow_open,
                        review_markers,
                    },
                );
            },
            commit_pending_edit: |cx, ctx| commit_pending_sheet_text(ctx, cx.model, cx.wb),
        });
    }
}

/// The dialog steps the feature draws, each under the key the shell's dialog
/// order lists it by.
const DIALOGS: &[FeatureDialog<SheetsFeature>] = &[
    FeatureDialog {
        key: DialogKey("sheets.drawing_sheet_overflow"),
        is_open: |sheets| sheets.drawing_sheet_overflow_open,
        render: render_overflow_review,
    },
    FeatureDialog {
        key: DialogKey("sheets.delete_sheet"),
        is_open: |sheets| sheets.delete_sheet.is_some(),
        render: |cx, ctx| {
            render_delete_sheet_confirmation(ctx, &mut cx.state.delete_sheet, cx.model, cx.wb);
        },
    },
    FeatureDialog {
        key: DialogKey("sheets.drawing_sheet_defaults"),
        is_open: |sheets| sheets.drawing_sheet_defaults.open,
        render: |cx, ctx| {
            render_drawing_sheet_defaults_dialog(
                ctx,
                &mut cx.state.drawing_sheet_defaults,
                &mut cx.state.drawing_sheet_presets,
                cx.model,
                cx.wb,
            );
        },
    },
    FeatureDialog {
        key: DialogKey("sheets.drawing_sheet_presets"),
        is_open: |sheets| sheets.drawing_sheet_presets.any_open(),
        render: |cx, ctx| {
            PresetDialogs {
                dialogs: &mut cx.state.drawing_sheet_presets,
                model: cx.model,
                workbench: cx.wb,
                reveal_sheet_section: &mut cx.state.reveal_sheet_section,
            }
            .render(ctx);
        },
    },
    FeatureDialog {
        key: DialogKey("sheets.drawing_sheet_support"),
        is_open: |sheets| sheets.drawing_sheet_support.any_open(),
        render: |cx, ctx| {
            render_drawing_sheet_support_dialogs(
                ctx,
                &mut cx.state.drawing_sheet_support,
                cx.model,
                cx.wb,
                &mut cx.state.reveal_sheet_section,
            );
        },
    },
];

/// The overflow review, and the step it hands on: the sheet's properties, or
/// the print workflow, which opens straight after the review is drawn.
fn render_overflow_review(cx: &mut Cx<'_, SheetsFeature>, ctx: &Context) {
    let review_markers = cx.peers.design().review_markers();
    match render_drawing_sheet_overflow_dialog(
        ctx,
        &mut cx.state.drawing_sheet_overflow_open,
        cx.model,
        review_markers,
    ) {
        Some(OverflowFollowUp::SheetProperties) => reveal_sheet_properties(cx),
        Some(OverflowFollowUp::Print) => cx.wb.outbox().push(PrintActiveView),
        None => {}
    }
}

/// Show the open sheet's properties in the inspector.
fn reveal_sheet_properties(cx: &mut Cx<'_, SheetsFeature>) {
    sheet_actions::reveal_sheet_properties_on(cx.model, cx.wb, &mut cx.state.reveal_sheet_section);
}

/// Whether the design workspace shows a schematic or testbench editor: where
/// the sheets of a drawing are paged and authored.
fn schematic_editor_open(workbench: &WorkbenchSession, model: &Model) -> bool {
    workbench.workspace == Workspace::Design
        && matches!(
            model.workspace.content.active_view_type(),
            ViewType::Schematic | ViewType::Testbench
        )
}

/// How many sheets the open drawing has. A drawing with no sheet catalog is
/// one sheet drawn without one, and counts none here.
fn sheet_count(model: &Model) -> usize {
    model
        .active_sheet_catalog()
        .map_or(0, |catalog| catalog.sheets().len())
}

/// The refusal a command gives when nothing more specific applies.
const UNAVAILABLE: CommandAvailability =
    CommandAvailability::Disabled("command is unavailable in this context");

/// `Available` when `enabled` holds, `refusal` otherwise.
fn available_or(enabled: bool, refusal: CommandAvailability) -> CommandAvailability {
    if enabled {
        CommandAvailability::Available
    } else {
        refusal
    }
}

impl CommandSet for SheetsCommand {
    type Feature = SheetsFeature;
    const REGISTERED: &'static [Self] = &Self::ALL;

    fn spec(self) -> CommandSpec {
        self.declared_spec()
    }

    fn availability(self, reads: &Reads<'_, SheetsFeature>) -> CommandAvailability {
        let (state, model, workbench) = (reads.state, reads.model, reads.wb);
        let read_only = model.schematic_edit_read_only();
        let editor = schematic_editor_open(workbench, model);
        let editable = drawing_sheet_editable(workbench, model);
        match self {
            Self::SheetProperties => available_or(
                editable,
                if read_only {
                    CommandAvailability::Disabled("the active schematic is read-only")
                } else {
                    CommandAvailability::Disabled(
                        "open an editable schematic or testbench drawing sheet",
                    )
                },
            ),
            Self::SheetFormatManager => {
                let has_active_sheet = model
                    .workspace
                    .content
                    .design_management
                    .sheet_catalog(&model.workspace.content.active_key())
                    .is_some_and(|catalog| catalog.active().is_some());
                let manager_open = state.drawing_sheet_support.manager.open;
                available_or(
                    editable && !read_only && !manager_open && has_active_sheet,
                    if read_only {
                        CommandAvailability::Disabled("the active schematic is read-only")
                    } else if !editable {
                        CommandAvailability::Disabled(
                            "open an editable schematic or testbench drawing sheet",
                        )
                    } else if manager_open {
                        CommandAvailability::Disabled("Sheet Format Manager is already open")
                    } else if !has_active_sheet {
                        CommandAvailability::Disabled(
                            "give the open sheet a property of its own in Sheet properties first",
                        )
                    } else {
                        UNAVAILABLE
                    },
                )
            }
            Self::CustomSheetSizes => available_or(
                editable && !state.drawing_sheet_presets.any_open(),
                UNAVAILABLE,
            ),
            Self::SheetDefaults => {
                available_or(editable && !state.drawing_sheet_defaults.open, UNAVAILABLE)
            }
            Self::DrawingSheetLayers => available_or(editor, UNAVAILABLE),
            // Sheet navigation is offered only where there is a second sheet
            // to reach. A drawing with no sheet catalog is one sheet, and a
            // new sheet there creates the catalog.
            Self::NextSheet | Self::PreviousSheet => available_or(
                editor && sheet_count(model) > 1,
                if editor {
                    CommandAvailability::Disabled("the active cell view has a single sheet")
                } else {
                    CommandAvailability::Disabled("open a schematic or testbench")
                },
            ),
            Self::NewSheet | Self::DeleteSheet => {
                let enabled =
                    editor && !read_only && (self == Self::NewSheet || sheet_count(model) > 1);
                available_or(
                    enabled,
                    if !editor {
                        CommandAvailability::Disabled("open an editable schematic or testbench")
                    } else if read_only {
                        CommandAvailability::Disabled("the active schematic is read-only")
                    } else {
                        CommandAvailability::Disabled("the active cell view has a single sheet")
                    },
                )
            }
        }
    }

    fn execute(self, cx: &mut Cx<'_, SheetsFeature>) {
        match self {
            // Sheet properties and custom sheet sizes are disabled whenever
            // they would refuse, so a refusal here needs no words.
            Self::SheetProperties => reveal_sheet_properties(cx),
            Self::SheetFormatManager => {
                let review_markers = cx.peers.design().review_markers();
                if let Err(error) = open_sheet_format_manager(
                    &mut cx.state.drawing_sheet_support,
                    cx.model,
                    cx.wb,
                    review_markers,
                ) {
                    cx.wb.prompt(error);
                }
            }
            Self::CustomSheetSizes => {
                open_custom_sheet_size_library(&mut cx.state.drawing_sheet_presets, cx.model);
            }
            Self::SheetDefaults => {
                open_drawing_sheet_defaults(&mut cx.state.drawing_sheet_defaults, cx.model);
            }
            // The sheet's layers are part of the drawing's one display
            // options dialog, which is design's.
            Self::DrawingSheetLayers => cx.execute(ids::VISIBILITY_OPTIONS),
            // Each of these reports its own outcome, including the catalog's
            // refusal text, so the result is never discarded.
            Self::NextSheet => {
                let _ = sheet_actions::next_sheet(cx.model, cx.wb);
            }
            Self::PreviousSheet => {
                let _ = sheet_actions::previous_sheet(cx.model, cx.wb);
            }
            Self::NewSheet => {
                let _ = sheet_actions::new_sheet(cx.model, cx.wb);
            }
            Self::DeleteSheet => {
                if let Some(id) = cx.model.active_sheet_id() {
                    let _ = sheet_actions::delete_sheet(
                        &mut cx.state.delete_sheet,
                        cx.model,
                        cx.wb,
                        id,
                    );
                }
            }
        }
    }
}
