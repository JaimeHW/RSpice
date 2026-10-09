//! Public DCMATCH control execution, presentation and retained-result contracts.
use rspice_core::analysis::dcmatch::DcMatchResult;
use rspice_core::engine::{
    ControlAnalysisResult, ControlCircuit, ControlCommandEffect, ControlExecutionError,
    ControlPresentationKind,
};
use rspice_core::execution::AnalysisResultDocument;
use rspice_core::execution::SignalUnit;
use rspice_core::execution::control::{ControlCommand, ControlScalarEvaluator};
use rspice_core::identity::AnalysisKind;
use rspice_core::netlist::{
    AnalysisCommand, SpectreDistribution, SpectreSpread, SpectreStatisticsPlan, SpectreVariation,
    SpectreVariationScope,
};
use rspice_core::{
    AbortSignal, Engine, Netlist, NoAbort, ResourceKind, SimulationConfig, SimulationError,
};
use std::sync::atomic::{AtomicUsize, Ordering};

fn deck(arguments: &str, scope: SpectreVariationScope) -> Netlist {
    let mut netlist = Netlist::parse(&format!("Mismatch control\n.param r=1 s=0\nB1 drive 0 V={{r+s}}\nV1 drive out 0\nR1 out 0 1k tc1=.01 tnom=27\n.options temp=27\n.dcmatch {arguments}\n.end\n")).unwrap();
    netlist.spectre_statistics = SpectreStatisticsPlan {
        variations: [("r", "0.01"), ("s", "0.02")]
            .into_iter()
            .map(|(parameter, spread)| SpectreVariation {
                bounds: None,
                line: 1,
                scope,
                parameter: parameter.into(),
                distribution: SpectreDistribution::Gaussian,
                spread: SpectreSpread::StandardDeviation(spread.into()),
                percent: false,
            })
            .collect(),
        correlations: vec![],
    };
    netlist
}

fn command(name: &str, arguments: &str) -> ControlCommand {
    ControlCommand {
        name: name.into(),
        arguments: arguments.into(),
        line: 9,
    }
}

fn execute(
    circuit: &mut ControlCircuit,
    name: &str,
    arguments: &str,
) -> Result<ControlCommandEffect, ControlExecutionError> {
    let variables = circuit.netlist().params.clone();
    circuit.execute(
        &Engine::default(),
        &command(name, arguments),
        &variables,
        &NoAbort,
    )
}

fn result(circuit: &ControlCircuit, index: usize) -> &DcMatchResult {
    let ControlAnalysisResult::DcMatch(result) = &circuit.datasets()[index].result else {
        panic!("DCMATCH result")
    };
    result
}

fn close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= 1e-8 * expected.abs().max(1e-12),
        "{actual} != {expected}"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn dcmatch_explicit_and_run_preserve_direct_results_and_physical_units() {
    for scope in [
        SpectreVariationScope::Mismatch,
        SpectreVariationScope::Process,
    ] {
        for (probe, unit, gain) in [
            ("V(out)", SignalUnit::Volt, 1.0_f64),
            ("V(0,out)", SignalUnit::Volt, -1.0),
            ("I(V1)", SignalUnit::Ampere, 0.001),
        ] {
            for report in ["CONTRIBUTORS=0", "CONTRIBUTORS=1", "THRESHOLD=.9"] {
                let scope_args = if scope == SpectreVariationScope::Mismatch {
                    "MISMATCH=YES PROCESS=NO"
                } else {
                    "MISMATCH=NO PROCESS=YES"
                };
                let arguments = format!("OUT={probe} {scope_args} {report} SIGMA=3");
                let netlist = deck(&arguments, scope);
                let AnalysisCommand::DcMatch(card) = &netlist.analyses[0] else {
                    panic!("card")
                };
                let expected = Engine::default().run_dc_match(&netlist, card).unwrap();
                close(expected.nominal_value, gain);
                close(expected.sigma_total, gain.abs() * 0.0005_f64.sqrt());
                let mut circuit = ControlCircuit::new(netlist).unwrap();
                execute(&mut circuit, "dcmatch", &arguments).unwrap();
                execute(&mut circuit, "run", "").unwrap();
                for index in 0..2 {
                    let dataset = &circuit.datasets()[index];
                    assert_eq!(dataset.name, format!("dcmatch{}", index + 1));
                    assert_eq!(dataset.analysis_id.kind(), AnalysisKind::DcMatch);
                    assert_eq!(result(&circuit, index), &expected);
                    assert_eq!(expected.output_unit(), unit);
                    let document = AnalysisResultDocument::from_dc_match(
                        dataset.analysis_id,
                        result(&circuit, index),
                    )
                    .unwrap()
                    .build()
                    .unwrap();
                    assert_eq!(
                        document,
                        AnalysisResultDocument::from_dc_match(dataset.analysis_id, &expected)
                            .unwrap()
                            .build()
                            .unwrap()
                    );
                }
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn dcmatch_prints_scalars_and_contributors_without_losing_empty_report_summaries() {
    let mut circuit = ControlCircuit::new(deck(
        "OUT=V(out) CONTRIBUTORS=0 SIGMA=3",
        SpectreVariationScope::Mismatch,
    ))
    .unwrap();
    execute(&mut circuit, "run", "").unwrap();
    let original = result(&circuit, 0).clone();
    let variables = circuit.netlist().params.clone();
    close(
        circuit
            .evaluate_scalar("dcmatch1.sigma_total*3", &variables, 9, &NoAbort)
            .unwrap()
            .re,
        original.quoted_sigma(),
    );
    assert!(
        circuit
            .evaluate_scalar("contribution", &variables, 9, &NoAbort)
            .is_err()
    );
    let ControlCommandEffect::Presentation(p) = execute(
        &mut circuit,
        "print",
        "nominal_value sigma_total quoted_sigma sigma_total*3 contribution share",
    )
    .unwrap() else {
        panic!("print")
    };
    assert_eq!(p.scalars.len(), 4);
    assert!(
        p.scalars
            .iter()
            .all(|s| s.scalar.unit() == Some(&SignalUnit::Volt))
    );
    let ControlPresentationKind::Print(traces) = p.kind else {
        panic!("traces")
    };
    assert_eq!(traces.len(), 2);
    assert_eq!(traces[0].y.unit, SignalUnit::Volt);
    assert_eq!(traces[1].y.unit, SignalUnit::Dimensionless);
    for (row, contributor) in original.contributors.iter().enumerate() {
        assert_eq!(traces[0].x.samples[row].re, row as f64);
        assert_eq!(traces[0].y.samples[row].re, contributor.contribution);
        assert_eq!(traces[1].y.samples[row].re, contributor.share);
    }
    execute(&mut circuit, "dcmatch", "OUT=V(out) THRESHOLD=.9").unwrap();
    assert!(result(&circuit, 1).contributors.is_empty());
    let ControlCommandEffect::Presentation(p) =
        execute(&mut circuit, "print", "sigma_total sigma_total*2").unwrap()
    else {
        panic!("print")
    };
    assert_eq!(p.scalars.len(), 2);
    close(
        circuit
            .evaluate_scalar("sigma_total", &variables, 9, &NoAbort)
            .unwrap()
            .re,
        original.sigma_total,
    );
    execute(&mut circuit, "settype", "current sigma_total").unwrap();
    let ControlCommandEffect::Presentation(p) =
        execute(&mut circuit, "print", "sigma_total").unwrap()
    else {
        panic!("print")
    };
    assert_eq!(p.scalars[0].scalar.unit(), Some(&SignalUnit::Ampere));
    assert_eq!(result(&circuit, 0), &original);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn dcmatch_observes_changes_and_rejects_different_contributor_grids() {
    let mut netlist = deck("OUT=I(V1) CONTRIBUTORS=2", SpectreVariationScope::Mismatch);
    let process = netlist
        .spectre_statistics
        .variations
        .clone()
        .into_iter()
        .map(|mut variation| {
            variation.scope = SpectreVariationScope::Process;
            variation
        });
    netlist.spectre_statistics.variations.extend(process);
    let mut circuit = ControlCircuit::new(netlist).unwrap();
    execute(&mut circuit, "run", "").unwrap();
    let original = result(&circuit, 0).clone();
    execute(&mut circuit, "option", "temp=127").unwrap();
    execute(&mut circuit, "run", "").unwrap();
    execute(&mut circuit, "alter", "R1 2k").unwrap();
    execute(&mut circuit, "run", "").unwrap();
    close(
        result(&circuit, 1).nominal_value,
        original.nominal_value / 2.0,
    );
    close(result(&circuit, 2).sigma_total, original.sigma_total / 4.0);
    assert_eq!(result(&circuit, 0), &original);
    execute(
        &mut circuit,
        "print",
        "dcmatch1.contribution+dcmatch2.contribution",
    )
    .unwrap();
    // Equal row counts and indices do not equate process and instance identities.
    execute(
        &mut circuit,
        "dcmatch",
        "OUT=I(V1) MISMATCH=NO PROCESS=YES CONTRIBUTORS=2",
    )
    .unwrap();
    assert_eq!(
        result(&circuit, 3).contributors.len(),
        original.contributors.len()
    );
    assert!(
        execute(
            &mut circuit,
            "print",
            "dcmatch1.contribution+dcmatch4.contribution"
        )
        .is_err()
    );
}

struct CancelAfter(AtomicUsize);
impl AbortSignal for CancelAfter {
    fn is_aborted(&self) -> bool {
        self.0.fetch_add(1, Ordering::Relaxed) >= 20
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn dcmatch_quota_failure_and_cancellation_do_not_publish_or_consume_ordinals() {
    let mut circuit = ControlCircuit::new(deck(
        "OUT=V(out) CONTRIBUTORS=0",
        SpectreVariationScope::Mismatch,
    ))
    .unwrap();
    execute(&mut circuit, "run", "").unwrap();
    let original = result(&circuit, 0).clone();
    let count = original.retained_value_count();
    let variables = circuit.netlist().params.clone();
    let mut config = SimulationConfig::default();
    config.resource_limits.max_result_values = 2 * count - 1;
    assert!(
        matches!(circuit.execute(&Engine::new(config.clone()), &command("run", ""), &variables, &NoAbort), Err(ControlExecutionError::Simulation { source: SimulationError::ResourceLimit(error), .. }) if error.resource == ResourceKind::ResultValues)
    );
    assert!(matches!(
        circuit.execute(
            &Engine::default(),
            &command("run", ""),
            &variables,
            &CancelAfter(AtomicUsize::new(0))
        ),
        Err(ControlExecutionError::Simulation {
            source: SimulationError::Aborted,
            ..
        })
    ));
    assert!(execute(&mut circuit, "dcmatch", "OUT=V(missing)").is_err());
    assert_eq!(circuit.datasets().len(), 1);
    assert_eq!(result(&circuit, 0), &original);
    config.resource_limits.max_result_values = 2 * count;
    circuit
        .execute(
            &Engine::new(config),
            &command("run", ""),
            &variables,
            &NoAbort,
        )
        .unwrap();
    assert_eq!(circuit.datasets()[1].name, "dcmatch2");
    assert_eq!(result(&circuit, 1), &original);
}
