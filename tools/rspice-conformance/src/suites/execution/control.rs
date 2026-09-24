//! Ordered control execution with an independent reference for each occurrence.
//!
//! A `!control` contract covers circuit changes, every named analysis, and
//! resolution of presentation vectors. It does not claim GUI plot rendering.

use super::*;
use crate::suites::ngspice::{TestRunner as OracleRunner, TestRunnerConfig};
use rspice_core::engine::{
    ControlAnalysisResult, ControlCircuit, ControlCommandEffect, ControlExecutionError,
    ControlPresentationKind,
};
use rspice_core::execution::control::{ControlErrorKind, ControlLimits, ControlProgram};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fmt::Write;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ControlContract {
    pub version: u32,
    /// Reference interpreter precision; 17 retains binary64 scalar arguments.
    pub ngspice_csnumprec: u8,
    pub runs: Vec<ControlRunContract>,
    /// A recorded trajectory without a reproducible numerical startup target.
    /// Such runs count only as execution coverage, never as oracle comparisons.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub characterization: Option<ControlCharacterization>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ControlCharacterization {
    pub reason: String,
    /// Corpus-relative companion with explicitly defined numerical conditions.
    pub qualification_case: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ControlRunContract {
    pub line: usize,
    pub dataset: String,
    pub probes: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ControlOracle {
    pub version: u32,
    pub producer: String,
    pub source_fingerprint: String,
    pub contract_fingerprint: String,
    pub runs: Vec<ControlRunReference>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ControlRunReference {
    pub line: usize,
    pub dataset: String,
    pub axis: String,
    pub coordinates: Vec<f64>,
    /// One complete column per contract probe, in the declared order.
    pub values: Vec<Vec<f64>>,
}

impl ControlContract {
    pub(super) fn load(path: &Path, source: &str) -> Result<Self, String> {
        let text = std::fs::read_to_string(path.with_extension("control.json"))
            .map_err(|error| format!("control contract: {error}"))?;
        let contract: Self =
            serde_json::from_str(&text).map_err(|error| format!("control contract: {error}"))?;
        if contract.version != 1 || contract.ngspice_csnumprec != 17 || contract.runs.is_empty() {
            return Err(
                "control contract requires version 1, CSNUMPREC=17 and at least one run".into(),
            );
        }
        let lines: Vec<_> = source.lines().collect();
        let mut names = BTreeSet::new();
        for run in &contract.runs {
            let mut command = run
                .line
                .checked_sub(1)
                .and_then(|line| lines.get(line))
                .and_then(|line| line.split_whitespace().next())
                .unwrap_or_default()
                .to_ascii_lowercase();
            if command == "run" {
                command = declarative_run_kind(source)?.to_string();
            }
            if !matches!(command.as_str(), "op" | "ac" | "tran")
                || !run.dataset.strip_prefix(&command).is_some_and(|ordinal| {
                    ordinal.parse::<usize>().is_ok_and(|ordinal| ordinal > 0)
                })
                || !names.insert(&run.dataset)
                || run.probes.is_empty()
                || run
                    .probes
                    .iter()
                    .any(|probe| probe.is_empty() || probe.chars().any(char::is_whitespace))
                || run.probes.iter().collect::<BTreeSet<_>>().len() != run.probes.len()
                || (command == "op"
                    && run.probes.iter().any(|probe| {
                        !(probe.starts_with("v(") || probe.starts_with("i("))
                            || !probe.ends_with(')')
                    }))
            {
                return Err(format!(
                    "invalid control run contract at line {} ({})",
                    run.line, run.dataset
                ));
            }
        }
        if let Some(characterization) = &contract.characterization {
            if characterization.reason.trim().is_empty()
                || characterization.qualification_case.is_empty()
                || Path::new(&characterization.qualification_case)
                    .components()
                    .any(|part| !matches!(part, std::path::Component::Normal(_)))
                || contract.runs.len() != 1
                || !contract.runs[0].dataset.starts_with("tran")
            {
                return Err("characterization requires a reason, a corpus-relative qualification case and one transient run".into());
            }
        }
        Ok(contract)
    }

    pub(super) fn fingerprint(&self) -> String {
        blake3::hash(
            serde_json::to_string(self)
                .expect("finite structural contract")
                .as_bytes(),
        )
        .to_hex()
        .to_string()
    }
}

/// The capture format currently records one plot per command occurrence.
/// Refuse multi-analysis RUN rather than capturing only its final plot.
pub(super) fn declarative_run_kind(source: &str) -> Result<&'static str, String> {
    let program = ControlProgram::parse_deck_with_abort(
        source,
        ControlLimits::default(),
        &rspice_core::NoAbort,
    )
    .map_err(|error| error.to_string())?;
    let netlist =
        Netlist::parse(program.declarative_source()).map_err(|error| error.to_string())?;
    match netlist.analyses.as_slice() {
        [AnalysisCommand::Op] => Ok("op"),
        [AnalysisCommand::Ac { .. }] => Ok("ac"),
        [AnalysisCommand::Tran { .. }] => Ok("tran"),
        _ => Err(
            "ordered RUN capture requires exactly one declarative OP, AC or transient analysis"
                .into(),
        ),
    }
}

impl ControlOracle {
    pub(super) fn validate(&self, contract: &ControlContract, source: &str) -> Result<(), String> {
        if self.version != 1
            || self.producer.is_empty()
            || self.source_fingerprint != super::oracle_bundle::source_fingerprint(source)
            || self.contract_fingerprint != contract.fingerprint()
            || self.runs.len() != contract.runs.len()
        {
            return Err(
                "control oracle version, producer, source, contract or run coverage differs".into(),
            );
        }
        for (run, expected) in self.runs.iter().zip(&contract.runs) {
            let axis = if expected.dataset.starts_with("ac") {
                "frequency"
            } else if expected.dataset.starts_with("tran") {
                "time"
            } else {
                "op"
            };
            if run.line != expected.line
                || run.dataset != expected.dataset
                || run.axis != axis
                || run.coordinates.is_empty()
                || (axis == "op" && run.coordinates.as_slice() != [0.0])
                || run.coordinates.iter().any(|value| !value.is_finite())
                || run.coordinates.windows(2).any(|pair| pair[1] <= pair[0])
                || run.values.len() != expected.probes.len()
                || run.values.iter().any(|column| {
                    column.len() != run.coordinates.len()
                        || column.iter().any(|value| !value.is_finite())
                })
            {
                return Err(format!(
                    "incomplete or mismatched control reference for {}",
                    expected.dataset
                ));
            }
        }
        Ok(())
    }
}

impl ControlRunReference {
    fn table(&self, probes: &[String]) -> String {
        if self.axis == "op" {
            let mut table = String::new();
            for (prefix, title) in [("v(", "Node Voltage"), ("i(", "Source Current")] {
                writeln!(table, "{title}").unwrap();
                for (probe, values) in probes
                    .iter()
                    .zip(&self.values)
                    .filter(|(probe, _)| probe.starts_with(prefix))
                {
                    writeln!(table, "{probe} {:.17e}", values[0]).unwrap();
                }
            }
            return table;
        }
        let mut table = format!("Index {} {}\n", self.axis, probes.join(" "));
        for (index, coordinate) in self.coordinates.iter().enumerate() {
            write!(table, "{index} {coordinate:.17e}").unwrap();
            for column in &self.values {
                write!(table, " {:.17e}", column[index]).unwrap();
            }
            table.push('\n');
        }
        table
    }
}

impl ExecutionRunner {
    pub(super) fn execute_control(
        &self,
        key: &str,
        path: &Path,
        source: &str,
        start: Instant,
    ) -> (ExecutionOutcome, Vec<String>, bool) {
        let mut labels = Vec::new();
        let mut compared = false;
        let execution = (|| -> Result<(), ExecutionOutcome> {
            let mismatch = |diagnostic| ExecutionOutcome::ReferenceMismatch { diagnostic };
            let measures = self.is_measures(key);
            if measures != path.with_extension("gates.tsv").is_file() {
                return Err(mismatch(
                    "ordered control !measures and its gates sidecar must agree".into(),
                ));
            }
            let contract = ControlContract::load(path, source).map_err(mismatch)?;
            if let Some(characterization) = &contract.characterization {
                let companion = &characterization.qualification_case;
                if measures
                    || companion == key
                    || !self.is_control(companion)
                    || !self.is_measures(companion)
                    || self.contract_for(companion) != ExecutionContract::Executes
                    || !self.root.join(companion).is_file()
                    || !self
                        .root
                        .join(companion)
                        .with_extension("gates.tsv")
                        .is_file()
                {
                    return Err(mismatch("characterization must name a separate executing control case with numerical measure gates".into()));
                }
            }
            if measures
                && (contract.runs.len() != 1 || !contract.runs[0].dataset.starts_with("tran"))
            {
                return Err(mismatch("ordered measure sidecars require exactly one transient run; multiple runs need distinct measure identities".into()));
            }
            let reference = std::fs::read_to_string(path.with_extension("control.oracle.json"))
                .map_err(|error| mismatch(format!("control oracle: {error}")))?;
            let oracle: ControlOracle = serde_json::from_str(&reference)
                .map_err(|error| mismatch(format!("control oracle: {error}")))?;
            oracle.validate(&contract, source).map_err(mismatch)?;
            let engine = self.engine(None);
            let abort = DeadlineAbort::new(start, self.config.max_time_per_deck_ms);
            let resources = engine.config().resource_limits;
            let program = ControlProgram::parse_deck_with_abort(
                source,
                ControlLimits {
                    max_source_bytes: resources.max_expanded_source_bytes,
                    max_source_lines: resources.max_netlist_lines,
                    max_loop_values: resources.max_batch_runs,
                    ..ControlLimits::default()
                },
                &abort,
            )
            .map_err(|error| control_error(error.into()))?;
            let netlist =
                Netlist::parse_with_path(program.declarative_source(), path).map_err(|error| {
                    ExecutionOutcome::Rejected {
                        diagnostic: error.to_string(),
                    }
                })?;
            let mut session = program.start(netlist.params.clone());
            let mut circuit = ControlCircuit::new(netlist).map_err(control_error)?;
            let mut completed = 0usize;
            while let Some(command) = session
                .next_command(&mut circuit, &abort)
                .map_err(|error| control_error(error.into()))?
            {
                let previous = circuit.datasets().len();
                match circuit
                    .execute(&engine, &command, session.variables(), &abort)
                    .map_err(control_error)?
                {
                    ControlCommandEffect::CircuitChanged => {}
                    ControlCommandEffect::Analyses(_) => {
                        for dataset in &circuit.datasets()[previous..] {
                            let label = format!("{} (control line {})", dataset.name, command.line);
                            labels.push(label.clone());
                            let expected = contract
                                .runs
                                .get(completed)
                                .ok_or_else(|| mismatch(format!("unexpected {label}")))?;
                            if expected.line != command.line || expected.dataset != dataset.name {
                                return Err(mismatch(format!(
                                    "{label} differs from expected {} at line {}",
                                    expected.dataset, expected.line
                                )));
                            }
                            let reference = &oracle.runs[completed];
                            let runner = OracleRunner::new_checked_in_oracle(
                                &self.root,
                                TestRunnerConfig {
                                    relative_tolerance: 0.05,
                                    absolute_tolerance: 1e-6,
                                    max_mismatches: 10,
                                    ..TestRunnerConfig::default()
                                },
                            )
                            .with_checked_in_reference_content(
                                path,
                                &reference.table(&expected.probes),
                            );
                            let comparison = match &dataset.result {
                                ControlAnalysisResult::OperatingPoint(result) => {
                                    if !result
                                        .node_voltages
                                        .iter()
                                        .chain(&result.branch_currents)
                                        .all(|v| v.is_finite())
                                    {
                                        return Err(mismatch(format!("{label}: nonfinite result")));
                                    }
                                    runner.compare_dc_op_reference(path, result)
                                }
                                ControlAnalysisResult::Ac(points) => {
                                    if points.is_empty()
                                        || points.iter().any(|p| {
                                            !p.frequency.is_finite()
                                                || p.voltages
                                                    .iter()
                                                    .chain(&p.currents)
                                                    .any(|v| !v.re.is_finite() || !v.im.is_finite())
                                        })
                                    {
                                        return Err(mismatch(format!(
                                            "{label}: empty or nonfinite result"
                                        )));
                                    }
                                    if points.len() != reference.coordinates.len()
                                        || points.iter().zip(&reference.coordinates).any(
                                            |(point, expected)| {
                                                (point.frequency - expected).abs()
                                                    > 1e-12 * expected.abs()
                                            },
                                        )
                                    {
                                        return Err(mismatch(format!(
                                            "{label}: AC frequency grid differs"
                                        )));
                                    }
                                    runner.compare_ac_reference(path, circuit.netlist(), points)
                                }
                                ControlAnalysisResult::Transient(result) => {
                                    if result.time.is_empty()
                                        || result.time.iter().any(|v| !v.is_finite())
                                        || result
                                            .voltages
                                            .iter()
                                            .chain(&result.branch_currents)
                                            .flatten()
                                            .any(|v| !v.is_finite())
                                    {
                                        return Err(mismatch(format!(
                                            "{label}: empty or nonfinite result"
                                        )));
                                    }
                                    let actual_stop = *result.time.last().unwrap();
                                    let expected_stop = *reference.coordinates.last().unwrap();
                                    if (actual_stop - expected_stop).abs()
                                        > 1e-12 * expected_stop.abs().max(f64::MIN_POSITIVE)
                                    {
                                        return Err(mismatch(format!(
                                            "{label}: transient interval is incomplete"
                                        )));
                                    }
                                    if contract.characterization.is_some() {
                                        for probe in &expected.probes {
                                            let probe = probe.to_ascii_lowercase();
                                            let column = if let Some(name) = probe
                                                .strip_prefix("v(")
                                                .and_then(|name| name.strip_suffix(')'))
                                            {
                                                result.node_names.iter().position(|node| node.eq_ignore_ascii_case(name))
                                                    .and_then(|index| result.voltages.get(index))
                                            } else if let Some(name) = probe
                                                .strip_prefix("i(")
                                                .and_then(|name| name.strip_suffix(')'))
                                            {
                                                result.branch_names.iter().position(|branch| branch.eq_ignore_ascii_case(name))
                                                    .and_then(|index| result.branch_currents.get(index))
                                            } else {
                                                None
                                            };
                                            if column.is_none_or(|values| values.len() != result.time.len()) {
                                                return Err(mismatch(format!("{label}: incomplete characterization probe {probe}")));
                                            }
                                        }
                                        Ok(Vec::new())
                                    } else if measures {
                                        runner.compare_transient_measures(path, result)
                                    } else {
                                        runner.compare_transient_reference(
                                            path,
                                            circuit.netlist(),
                                            result,
                                        )
                                    }
                                }
                            }
                            .map_err(mismatch)?;
                            compared |= contract.characterization.is_none();
                            if !comparison.is_empty() {
                                return Err(mismatch(format!(
                                    "{label}: {:?}",
                                    &comparison[..comparison.len().min(3)]
                                )));
                            }
                            completed += 1;
                        }
                    }
                    ControlCommandEffect::Presentation(request) => {
                        let traces = match &request.kind {
                            ControlPresentationKind::Plot { traces, .. }
                            | ControlPresentationKind::Print(traces) => traces.as_slice(),
                            ControlPresentationKind::UnitsChanged { .. } => &[],
                        };
                        if traces.iter().any(|trace| {
                            trace.x.samples.is_empty()
                                || trace.x.samples.len() != trace.y.samples.len()
                                || trace
                                    .x
                                    .samples
                                    .iter()
                                    .chain(&trace.y.samples)
                                    .any(|v| !v.re.is_finite() || !v.im.is_finite())
                        }) {
                            return Err(mismatch(format!(
                                "control line {}: incomplete/nonfinite presentation",
                                command.line
                            )));
                        }
                    }
                }
            }
            if completed != contract.runs.len() {
                return Err(mismatch(format!(
                    "control script completed {completed} of {} required runs",
                    contract.runs.len()
                )));
            }
            Ok(())
        })();
        (
            execution.map_or_else(|error| error, |_| ExecutionOutcome::Completed),
            labels,
            compared,
        )
    }
}

fn control_error(error: ControlExecutionError) -> ExecutionOutcome {
    match &error {
        ControlExecutionError::Command(error) if error.kind == ControlErrorKind::Aborted => {
            ExecutionOutcome::TimedOut {
                analysis: "control script".into(),
            }
        }
        ControlExecutionError::Simulation {
            source: rspice_core::SimulationError::Aborted,
            ..
        } => ExecutionOutcome::TimedOut {
            analysis: "control script".into(),
        },
        ControlExecutionError::Command(_) => ExecutionOutcome::Unsupported {
            directive: error.to_string(),
        },
        _ => ExecutionOutcome::Refused {
            analysis: "control script".into(),
            diagnostic: error.to_string(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordered_control_runs_compare_each_altered_bias_and_reject_corrupt_coverage() {
        let temporary = tempfile::tempdir().unwrap();
        let root = temporary.path().join("paranoia");
        std::fs::create_dir(&root).unwrap();
        let path = root.join("ordered.sp");
        let source = "ordered divider\nV1 in 0 0\nR1 in out 1k\nR2 out 0 1k\n.op\n.control\nforeach level 1 2\nalter V1 $level\nrun\nend\nplot op1.out op2.out\n.endc\n.end\n";
        std::fs::write(&path, source).unwrap();
        std::fs::write(
            root.join("execution-manifest.tsv"),
            "ordered.sp\texecutes\t!control\n",
        )
        .unwrap();
        let contract = ControlContract {
            version: 1,
            ngspice_csnumprec: 17,
            characterization: None,
            runs: (1..=2)
                .map(|ordinal| ControlRunContract {
                    line: 9,
                    dataset: format!("op{ordinal}"),
                    probes: vec!["v(out)".into()],
                })
                .collect(),
        };
        std::fs::write(
            path.with_extension("control.json"),
            serde_json::to_string(&contract).unwrap(),
        )
        .unwrap();
        let expanded = Netlist::preprocess_includes(source, &path).unwrap();
        let mut oracle = ControlOracle {
            version: 1,
            producer: "independent equal-resistor divider".into(),
            source_fingerprint: super::super::oracle_bundle::source_fingerprint(&expanded),
            contract_fingerprint: contract.fingerprint(),
            runs: (1..=2)
                .map(|ordinal| ControlRunReference {
                    line: 9,
                    dataset: format!("op{ordinal}"),
                    axis: "op".into(),
                    coordinates: vec![0.0],
                    values: vec![vec![ordinal as f64 / 2.0]],
                })
                .collect(),
        };
        let runner = ExecutionRunner::new(
            ExecutionCorpus::Paranoia,
            temporary.path(),
            ExecutionConfig::default(),
        );
        let write = |oracle: &ControlOracle| {
            std::fs::write(
                path.with_extension("control.oracle.json"),
                serde_json::to_string(oracle).unwrap(),
            )
            .unwrap()
        };
        write(&oracle);
        let result = runner.run_deck("ordered.sp");
        assert!(result.passed && result.oracle_compared, "{result:?}");
        assert_eq!(result.analyses.len(), 2);
        oracle.runs[1].values[0][0] = 0.5;
        write(&oracle);
        assert!(matches!(
            runner.run_deck("ordered.sp").outcome,
            ExecutionOutcome::ReferenceMismatch { .. }
        ));
        oracle.runs[1].values[0][0] = 1.0;
        oracle.runs.swap(0, 1);
        write(&oracle);
        assert!(matches!(
            runner.run_deck("ordered.sp").outcome,
            ExecutionOutcome::ReferenceMismatch { .. }
        ));
        oracle.runs.swap(0, 1);
        oracle.runs.pop();
        write(&oracle);
        assert!(matches!(
            runner.run_deck("ordered.sp").outcome,
            ExecutionOutcome::ReferenceMismatch { .. }
        ));
    }

    #[test]
    fn original_bjt_control_compares_all_six_bias_runs() {
        original_script("control_structs/foreach_bjt_ft.sp", 6);
    }

    #[test]
    fn original_memristor_control_compares_all_three_frequency_runs() {
        original_script("memristor/memristor.sp", 3);
    }

    #[test]
    fn ring_explicit_startup_and_settled_cycles_meet_measure_gates() {
        original_script("various/ro_17_4_startup.cir", 1);
    }

    #[test]
    fn original_ring_is_complete_execution_without_numerical_credit() {
        let tests = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests");
        let mut runner = ExecutionRunner::new(
            ExecutionCorpus::Paranoia,
            &tests,
            ExecutionConfig::default(),
        );
        let result = runner.run_deck("various/ro_17_4.cir");
        assert!(result.passed && !result.oracle_compared, "{result:?}");
        assert_eq!(result.analyses.len(), 1);
        runner.manifest.remove("various/ro_17_4_startup.cir");
        let missing = runner.run_deck("various/ro_17_4.cir");
        assert!(
            matches!(missing.outcome, ExecutionOutcome::ReferenceMismatch { .. }),
            "{missing:?}"
        );
        assert!(missing.analyses.is_empty());
    }

    #[test]
    fn ring_qualification_preserves_the_original_circuit_and_model() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/paranoia/various");
        let original = std::fs::read_to_string(root.join("ro_17_4.cir"))
            .unwrap()
            .replace("\r\n", "\n");
        let startup = std::fs::read_to_string(root.join("ro_17_4_startup.cir"))
            .unwrap()
            .replace("\r\n", "\n");
        assert_eq!(
            startup,
            original.replace(".tran .1ns 5n", ".ic v(18)=1\n.tran .1ns 30n 0 20p"),
        );
        assert!(
            declarative_run_kind(&original.replace(".tran .1ns 5n", ".op\n.tran .1ns 5n")).is_err()
        );
    }

    fn original_script(key: &str, count: usize) {
        let tests = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests");
        let runner = ExecutionRunner::new(
            ExecutionCorpus::Paranoia,
            &tests,
            ExecutionConfig::default(),
        );
        let result = runner.run_deck(key);
        assert!(result.passed && result.oracle_compared, "{result:?}");
        assert_eq!(result.analyses.len(), count, "{key}");
    }
}
