//! The hardcopy feature: the schematic and publication exports, Print and
//! Export active view, and the report editor.
//!
//! Its commands are answered and carried out in a [`Cx`] over the feature's
//! own slot, the model, the session and the platform services' state, and
//! read what the results workspace shows through its port. The schematic and
//! publication exports write through the export IO the context lends, and
//! Print and Export active view open their workflow in the same context.
//!
//! The hardcopy dialog is the feature's dialog step, drawn in the same
//! context. The shell opens its workflow, in the same context, for the
//! requests other parts raise: a result document's figure export, and the
//! print another feature offers as its dialog's next step.
//!
//! The report route is the feature's too: it draws the route's stage in the
//! same context, and declares the tools the route's toolbar shows, which
//! the shell lays out and dispatches like any other.

pub(crate) mod commands;
pub(crate) mod exports;
pub(crate) mod report;
pub(crate) mod report_authoring;

use crate::ui::panels::WorkbenchIcon;
use crate::workbench::app::{
    HardcopyParts, HardcopyWorkflow, open_hardcopy_workflow, render_hardcopy_dialog,
};
use crate::workbench::framework::command_registry::{CommandAvailability, CommandSpec, ids};
use crate::workbench::framework::dialogs::DialogKey;
use crate::workbench::framework::feature::{
    CommandSet, Cx, Feature, FeatureDialog, FeatureRegistry, FeatureSurface, Reads, ToolbarTool,
};
use crate::workbench::hardcopy_adapters::sources::active_app_hardcopy_source_available;
use crate::workbench::slots::HardcopyState;
use crate::workbench::state::WorkbenchSession;

use commands::HardcopyCommand;

/// The hardcopy feature.
pub struct HardcopyFeature;

impl Feature for HardcopyFeature {
    type State = HardcopyState;
    type Session = WorkbenchSession;
    type Handles = crate::workbench::framework::feature::NoHandles;

    fn register(registry: &mut FeatureRegistry<Self>) {
        registry.commands::<HardcopyCommand>();
        registry.dialogs(DIALOGS);
        registry.surface(FeatureSurface {
            show: report_authoring::show,
        });
        registry.toolbar(REPORT_TOOLS);
    }
}

/// The dialog step the feature draws, under the key the shell's dialog order
/// lists it by: Page Setup, Print and Export. It runs every frame, open or
/// not: closed, it lets go of the work the dialog started.
const DIALOGS: &[FeatureDialog<HardcopyFeature>] = &[FeatureDialog {
    key: DialogKey("hardcopy.hardcopy"),
    is_open: |hardcopy| hardcopy.hardcopy.open,
    render: |cx, ctx| render_hardcopy_dialog(&mut workflow_parts(cx), ctx),
}];

/// What the hardcopy workflow reads and writes, borrowed from the feature's
/// context.
fn workflow_parts<'a>(cx: &'a mut Cx<'_, HardcopyFeature>) -> HardcopyParts<'a> {
    let hardcopy = &mut *cx.state;
    HardcopyParts {
        dialog: &mut hardcopy.hardcopy,
        report_authoring: &hardcopy.report_authoring,
        model: cx.model,
        workbench: cx.wb,
        results: cx.peers.results(),
        export_io: cx.handles.export_io,
    }
}

/// Open `workflow` on the active view, in the feature's context: for Print
/// and Export active view, and, as the shell asks, for a request another part
/// raised, the export workflow for a result document's figure or the print
/// workflow another feature offers as its dialog's next step.
pub fn open_workflow(cx: &mut Cx<'_, HardcopyFeature>, workflow: HardcopyWorkflow) {
    open_hardcopy_workflow(&mut workflow_parts(cx), workflow);
}

/// The report route's toolbar: its document actions, in the order the
/// toolbar lays them out. The mockup's Build review draft action stays off
/// it until exact plot artwork publication and the release handoff complete
/// the full contract: the route never offers an artifact it cannot produce.
const REPORT_TOOLS: &[ToolbarTool] = &[
    ToolbarTool {
        key: HardcopyCommand::SaveReportDocument.key(),
        icon: WorkbenchIcon::Save,
        label: "Save report document",
    },
    ToolbarTool {
        key: HardcopyCommand::AddReportPage.key(),
        icon: WorkbenchIcon::Add,
        label: "Add report page",
    },
    ToolbarTool {
        key: HardcopyCommand::ReportPageProperties.key(),
        icon: WorkbenchIcon::Sliders,
        label: "Page properties",
    },
];

impl CommandSet for HardcopyCommand {
    type Feature = HardcopyFeature;
    const REGISTERED: &'static [Self] = &Self::LISTED;
    const UNLISTED: &'static [Self] = &Self::REPORT_TOOLBAR;

    fn spec(self) -> CommandSpec {
        self.declared_spec()
    }

    /// The exports and the report editor run whenever a project is open,
    /// which the shell's guard has already asked. Print and Export active
    /// view need a view to resolve, and Print a host that prints. The report
    /// toolbar's actions need the report route and a report the project lets
    /// change.
    fn availability(self, reads: &Reads<'_, HardcopyFeature>) -> CommandAvailability {
        let editor = &reads.state.report_authoring;
        let available = match self {
            Self::ExportSchematicSvg | Self::ExportPublicationSnapshot | Self::ReportAuthoring => {
                true
            }
            Self::PrintHardcopy => {
                cfg!(any(target_os = "windows", target_arch = "wasm32"))
                    && active_app_hardcopy_source_available(
                        &reads.state.report_authoring,
                        reads.model,
                        reads.wb,
                        reads.results,
                    )
            }
            Self::ExportActiveView => active_app_hardcopy_source_available(
                &reads.state.report_authoring,
                reads.model,
                reads.wb,
                reads.results,
            ),
            Self::SaveReportDocument => {
                report::can_save_document(editor, reads.model, reads.wb, reads.platform)
            }
            Self::AddReportPage => {
                report::can_add_page(editor, reads.model, reads.wb, reads.platform)
            }
            Self::ReportPageProperties => {
                report::can_edit_page_properties(editor, reads.model, reads.wb, reads.platform)
            }
        };
        if available {
            CommandAvailability::Available
        } else {
            CommandAvailability::Disabled("command is unavailable in this context")
        }
    }

    fn execute(self, cx: &mut Cx<'_, HardcopyFeature>) {
        match self {
            Self::ExportSchematicSvg => {
                exports::action_export_svg_with_io(cx.model, cx.handles.export_io);
            }
            Self::ExportPublicationSnapshot => {
                exports::action_export_publication_snapshot_with_io(cx)
            }
            Self::PrintHardcopy => open_workflow(cx, HardcopyWorkflow::Print),
            Self::ExportActiveView => open_workflow(cx, HardcopyWorkflow::Export),
            Self::ReportAuthoring => report::open(cx.wb),
            // Saving the report is saving the project, behind its own
            // prologue; nothing follows it here.
            Self::SaveReportDocument => {
                if report::ready_to_save(cx.model, cx.wb, cx.platform) {
                    cx.execute(ids::SAVE);
                }
            }
            Self::AddReportPage => {
                report::open_add_page(&mut cx.state.report_authoring, cx.model, cx.platform);
            }
            Self::ReportPageProperties => {
                report::open_page_properties(&mut cx.state.report_authoring, cx.model, cx.platform);
            }
        }
    }
}
