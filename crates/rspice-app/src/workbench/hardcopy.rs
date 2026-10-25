//! The hardcopy feature: the schematic and publication exports, Print and
//! Export active view, and the report editor.
//!
//! Its commands are answered and carried out in a [`Cx`] over the feature's
//! own slot, the model, the session and the platform services' state, and
//! read what the results workspace shows through its port. The exports and
//! the print and export workflows still run in the shell, on the request a
//! command raises, straight after the command in the same dispatch.

pub(crate) mod commands;
pub(crate) mod report;
pub(crate) mod report_authoring;

use crate::workbench::framework::command_registry::{CommandAvailability, CommandSpec, ids};
use crate::workbench::framework::feature::{
    CommandSet, Cx, Feature, FeatureRegistry, Reads, Session,
};
use crate::workbench::framework::requests::{
    ExportActiveView, ExportPublicationSnapshot, ExportSchematicSvg, PrintActiveView,
};
use crate::workbench::hardcopy_adapters::sources::active_app_hardcopy_source_available;
use crate::workbench::slots::HardcopyState;
use crate::workbench::state::WorkbenchSession;

use commands::HardcopyCommand;

/// The hardcopy feature.
pub(crate) struct HardcopyFeature;

impl Feature for HardcopyFeature {
    type State = HardcopyState;
    type Session = WorkbenchSession;

    fn register(registry: &mut FeatureRegistry<Self>) {
        registry.commands::<HardcopyCommand>();
    }
}

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
            Self::ExportSchematicSvg => cx.wb.outbox().push(ExportSchematicSvg),
            Self::ExportPublicationSnapshot => cx.wb.outbox().push(ExportPublicationSnapshot),
            Self::PrintHardcopy => cx.wb.outbox().push(PrintActiveView),
            Self::ExportActiveView => cx.wb.outbox().push(ExportActiveView),
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
