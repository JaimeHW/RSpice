//! What a run's checks reported beside its result.

use rspice_core::ModelFinish;
use rspice_core::analysis::AssertionReport;
use rspice_core::config::{HdlOutputRecord, HdlStream};
use rspice_results::analysis_result::{HdlDiagnostic, RunChecks};

/// What a run's checks reported beside an operating point, a DC sweep or a
/// transient: the HDL's severity-task records and the deck's `.ASSERT`
/// reports. Both are observations of the run; neither changes its result.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SimulationChecks {
    /// The HDL's `$info`, `$warning`, `$error` and `$fatal` records, in
    /// emission order, each with the call's file, line and time.
    pub hdl: Vec<HdlOutputRecord>,
    /// The deck's `.ASSERT` reports for this analysis, one per assertion and
    /// checked instance, as the evaluator published them.
    pub assertions: Vec<AssertionReport>,
    /// The `$finish`, `$stop` or `$fatal` that ended the run before its
    /// stop point, with its source line, its time and a `$fatal`'s message.
    /// `None` for a run that reached its end.
    pub finish: Option<ModelFinish>,
}

impl SimulationChecks {
    /// The checks of one analysis: the severity-task records among the HDL
    /// output it accepted, and its assertion reports.
    pub fn new(hdl_output: &[HdlOutputRecord], assertions: Vec<AssertionReport>) -> Self {
        Self {
            hdl: hdl_output
                .iter()
                .filter(|record| matches!(record.stream, HdlStream::Diagnostic { .. }))
                .cloned()
                .collect(),
            assertions,
            finish: None,
        }
    }

    /// These checks, with the request that ended the run, if one did.
    #[must_use]
    pub fn finished_by(mut self, finish: Option<ModelFinish>) -> Self {
        self.finish = finish;
        self
    }

    pub fn is_empty(&self) -> bool {
        self.hdl.is_empty() && self.assertions.is_empty() && self.finish.is_none()
    }

    /// The checks as a retained analysis result keeps them.
    pub fn into_run_checks(self) -> RunChecks {
        RunChecks {
            hdl: self
                .hdl
                .into_iter()
                .filter_map(|record| match record.stream {
                    HdlStream::Diagnostic { severity } => Some(HdlDiagnostic {
                        severity,
                        text: record.text,
                        point: record.point,
                        origin: record.origin,
                    }),
                    _ => None,
                })
                .collect(),
            assertions: self.assertions,
            finish: self.finish,
        }
    }
}
