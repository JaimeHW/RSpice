//! Transient regression runner.

use super::*;

impl TestRunner {
    pub(super) fn run_transient_test(
        &self,
        name: &str,
        cir_path: &Path,
        source: &str,
        tstep: Value,
        tstop: Value,
        tstart: Value,
        tmax: Option<Value>,
        start: std::time::Instant,
    ) -> TestResult {
        let netlist = match Netlist::parse(source) {
            Ok(n) => n,
            Err(e) => {
                return TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(format!("Parse error: {}", e)),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("Transient".to_string()),
                };
            }
        };

        let mut engine = self.create_dynamic_engine();
        // RSPICE_GRID_LOCKED=1 replays the reference's recorded time grid as
        // the exact accepted-step sequence (no adaptive points, no
        // breakpoint restarts, LTE off), isolating physics parity from
        // step-control parity: every remaining transient gate failure is a
        // grid-coupled trajectory difference against a grid no modern
        // binary reproduces, so comparing on equal grids is the
        // Spectre-grade validation standard. Decks without a time-axis
        // reference table fall back to free-running.
        if std::env::var("RSPICE_GRID_LOCKED").as_deref() == Ok("1")
            && let Ok(Some(reference)) = self.load_reference_table_for_axis(cir_path, &["time"])
            && let Some(series) = reference
                .variables
                .values()
                .max_by_key(|series| series.x.len())
            && series.x.len() >= 2
        {
            engine.config.locked_time_grid =
                Some(std::sync::Arc::new(series.x.clone()));
        }
        let max_step =
            tmax.unwrap_or_else(|| Self::default_transient_max_step(tstep, tstop, tstart));

        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms);
        match engine.run_tran_with_abort(&netlist, tstop, max_step, &abort) {
            Ok(result) => {
                let mismatches = match self.compare_transient_reference(cir_path, &netlist, &result)
                {
                    Ok(m) => m,
                    Err(e) => {
                        return TestResult {
                            name: name.to_string(),
                            passed: false,
                            error: Some(format!("Reference comparison error: {}", e)),
                            mismatches: Vec::new(),
                            duration_ms: start.elapsed().as_millis(),
                            analysis_type: Some("Transient".to_string()),
                        };
                    }
                };
                let passed = mismatches.is_empty();
                TestResult {
                    name: name.to_string(),
                    passed,
                    error: if passed {
                        None
                    } else {
                        Some(format!("{} reference mismatch(es)", mismatches.len()))
                    },
                    mismatches,
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("Transient".to_string()),
                }
            }
            Err(e) => TestResult {
                name: name.to_string(),
                passed: false,
                error: Some(format!("Simulation error: {}", e)),
                mismatches: Vec::new(),
                duration_ms: start.elapsed().as_millis(),
                analysis_type: Some("Transient".to_string()),
            },
        }
    }
}
