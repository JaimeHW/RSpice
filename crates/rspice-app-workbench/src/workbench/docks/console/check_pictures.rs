//! The review pictures of a run's checks on the Problems page and in the
//! Console: a testbench's `$error`, `$warning` and `$fatal`, a violated
//! assertion card and the task that ended the run, each one row; and the
//! HDL line whose source line opens the cell view's Verilog-A in its editor.

use std::sync::Arc;

use rspice_core::analysis::{AssertViolation, AssertionReport};
use rspice_core::config::{HdlOrigin, HdlPoint, HdlSeverity};
use rspice_core::netlist::{AssertAnalysis, AssertLevel, NetlistSourceLocation};
use rspice_core::{ModelFinish, ModelFinishPoint, ModelFinishSource, ModelFinishTask};
use rspice_results::analysis_result::{HdlDiagnostic, RunChecks};
use rspice_vocabulary::run_log::{RunLogLine, RunLogSource};

use crate::diagnostics::{LogAnchor, LogSeverity, LogSource};
use crate::state::code_diagnostics::CodeDiagnosticCollection;
use crate::state::netlist_diagnostics::NetlistDiagnosticCollection;
use crate::state::{
    AnalysisResult, AnalysisType, CellViewRef, ProjectSourceBundle, ProjectSourceDependency,
    ProjectSourceFile, ProjectSourceLanguage, ProjectSourceOwner, SimulationRunLifecycle,
};
use crate::workbench::app_state::Model;
use crate::workbench::app_state::diagnostic_store::SourceCollections;
use crate::workbench::app_state::diagnostic_store::listing::{ListingInputs, ListingSource};
use crate::workbench::app_state::waivers::WaiverTarget;
use crate::workbench::framework::host::ProblemsHost;
use crate::workbench::framework::workbench_host::ConsoleHost;
use crate::workbench::state::{ConsoleChannel, WorkbenchSession};
use rspice_app_state::workbench::app_state::model::SlotFacts;

/// The shell's side of the two pages, over a model and a session alone.
struct PictureHost {
    model: Model,
    workbench: WorkbenchSession,
    netlist: Arc<NetlistDiagnosticCollection>,
    code: Arc<CodeDiagnosticCollection>,
}

impl ProblemsHost for PictureHost {
    fn apply_quick_fix(&mut self, _: uuid::Uuid) -> Result<String, String> {
        Err("no quick fix in a picture".into())
    }
}

impl ListingSource for PictureHost {
    fn listing_inputs(&self) -> ListingInputs<'_> {
        ListingInputs {
            model: &self.model,
            sources: SourceCollections {
                netlist: &self.netlist,
                netlist_revision: 0,
                netlist_validated: None,
                veriloga: None,
                veriloga_token: None,
                veriloga_failure: &self.code,
                veriloga_failure_token: None,
                automation: &self.code,
                automation_validated: false,
                automation_token: None,
            },
            facts: facts(),
        }
    }
}

impl ConsoleHost for PictureHost {
    fn model(&self) -> &Model {
        &self.model
    }

    fn model_mut(&mut self) -> &mut Model {
        &mut self.model
    }

    fn workbench(&self) -> &WorkbenchSession {
        &self.workbench
    }

    fn workbench_mut(&mut self) -> &mut WorkbenchSession {
        &mut self.workbench
    }

    fn jump_to_log_anchor(&mut self, _: LogAnchor) {}

    fn log_anchor_refusal(&self, _: &LogAnchor) -> Option<String> {
        None
    }

    fn waive_finding(&mut self, _: &WaiverTarget, _: &str) -> Result<(), String> {
        Err("no waivers in a picture".into())
    }

    fn remove_finding_waiver(&mut self, _: &WaiverTarget) -> Result<(), String> {
        Err("no waivers in a picture".into())
    }
}

fn facts() -> SlotFacts {
    SlotFacts {
        preflight_report_retained: false,
        review_markers: Default::default(),
    }
}

fn hdl(severity: HdlSeverity, time: f64, line: u32, text: &str) -> HdlDiagnostic {
    HdlDiagnostic {
        severity,
        text: text.into(),
        point: HdlPoint::Transient { time },
        origin: HdlOrigin {
            scope: "xtb".into(),
            file: "tb/tb.va".into(),
            line,
        },
    }
}

/// A project whose cell view `lib.tb` holds `tb/tb.va`, and one completed
/// run of it whose testbench reported four checks and wrote them to its log.
fn checked_run() -> PictureHost {
    let mut model = Model::session_start(
        crate::state::pdk_config::PdkConfig::default(),
        None,
        rspice_app_state::workbench::workspace::restore_model_library_manager,
    );
    let bundle = ProjectSourceBundle::try_new(
        ProjectSourceOwner::cell_view(CellViewRef::new("lib", "tb", "veriloga")),
        ProjectSourceLanguage::VerilogA,
        "tb/tb.va",
        "module tb; endmodule\n",
        Vec::<ProjectSourceFile>::new(),
        Vec::<ProjectSourceDependency>::new(),
    )
    .expect("a valid Verilog-A bundle");
    model
        .workspace
        .content
        .project_sources
        .insert_bundle(bundle)
        .expect("attach the cell view's source");
    let checks = RunChecks {
        hdl: vec![
            hdl(HdlSeverity::Error, 2.0e-9, 9, "out stuck low"),
            hdl(HdlSeverity::Warning, 3.0e-9, 11, "slow edge"),
            hdl(HdlSeverity::Fatal, 5.0e-9, 14, "giving up"),
        ],
        assertions: vec![AssertionReport {
            name: "vout_floor".into(),
            analysis: AssertAnalysis::Tran,
            target: "V(out) > 1.1".into(),
            instance: None,
            level: AssertLevel::Error,
            message: None,
            origin: NetlistSourceLocation::in_memory(5),
            violations: vec![AssertViolation {
                start: Some(1.0e-9),
                end: None,
                extreme: Some(1.0),
                at: Some(1.0e-9),
            }],
            unknown_stretches: 0,
            error: None,
        }],
        finish: Some(ModelFinish {
            instance: "xtb".into(),
            model: "tb".into(),
            site: 0,
            point: ModelFinishPoint::Transient { time: 5.0e-9 },
            diagnostic_level: 1,
            task: ModelFinishTask::Fatal,
            source: Some(ModelFinishSource {
                file: "tb/tb.va".into(),
                line: 14,
            }),
            message: Some("giving up".into()),
        }),
    };
    let run = model.simulation.start_run();
    let sequence = run.id;
    let mut analysis = AnalysisResult::new(1, AnalysisType::Transient, "TRAN");
    analysis.checks = checks.clone();
    run.add_analysis(analysis);
    run.lifecycle = SimulationRunLifecycle::Completed;
    for (severity, line, text) in [
        (LogSeverity::Error, 9, "out stuck low"),
        (LogSeverity::Warning, 11, "slow edge"),
        (LogSeverity::Error, 14, "giving up"),
    ] {
        run.log.push_sourced(
            RunLogLine {
                unix_ms: 0,
                severity,
                source: LogSource::Hdl,
                message: format!("{text} \u{2014} xtb (tb/tb.va:{line})"),
            },
            Some(RunLogSource {
                file: "tb/tb.va".into(),
                line,
            }),
        );
    }
    model.publish_run_checks("TRAN", &checks, sequence, facts());
    let mut workbench = WorkbenchSession::default();
    workbench.console_view.open(ConsoleChannel::Run(sequence));
    PictureHost {
        model,
        workbench,
        netlist: Arc::default(),
        code: Arc::default(),
    }
}

fn write(directory: &str, name: &str, size: egui::Vec2, mut pass: impl FnMut(&mut egui::Ui)) {
    let canvas = rspice_ui_kit::raster::render(size, |ui, _| pass(ui));
    std::fs::write(format!("{directory}/{name}"), canvas.png(canvas.height()))
        .expect("the picture is written");
}

#[test]
#[ignore = "writes the review pictures"]
fn write_the_run_check_pictures() {
    let Ok(directory) = std::env::var("RSPICE_PICTURES") else {
        return;
    };
    let mut state = checked_run();
    write(
        &directory,
        "problems-run-checks.png",
        egui::vec2(1100.0, 240.0),
        |ui| {
            super::problems::problems(ui, &mut state);
        },
    );
    write(
        &directory,
        "console-hdl-source-link.png",
        egui::vec2(1100.0, 140.0),
        |ui| super::transcript::console(ui, &mut state),
    );
}
