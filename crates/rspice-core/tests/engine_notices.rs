//! A run's advisories are recorded where the engine raises them, against the
//! element and the card it was binding, beside the log line that says so.

use rspice_core::notice::{EngineNotice, NoticeCollector, ParameterReplacement, ParameterScope};
use rspice_core::{Engine, Netlist, NoAbort, SimulationConfig};

/// Two geometries of one card whose `pbs` is below BSIM4's floor and whose
/// `eu` is negative.
const DECK: &str = "BSIM4 clamps\nVD d 0 1\nVG g 0 1\n\
M1 d g 0 0 nch L=1u W=10u\nM2 d g 0 0 nch L=2u W=10u\n\
.model nch nmos level=54 version=4.8 pbs=0.05 eu=-1\n.op\n.end\n";

fn replacements(collector: &NoticeCollector) -> Vec<(ParameterReplacement, usize)> {
    collector
        .take()
        .into_iter()
        .filter_map(|recorded| match recorded.notice {
            EngineNotice::ParameterReplaced(replacement) => {
                Some((replacement, recorded.occurrences))
            }
            _ => None,
        })
        .collect()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn a_clamp_is_recorded_against_the_card_and_the_element_that_bound_it() {
    let collector = NoticeCollector::new();
    let _installed = collector.install();
    let netlist = Netlist::parse(DECK).expect("the deck parses");
    Engine::default()
        .run_dc_op_with_abort(&netlist, &NoAbort)
        .expect("a clamped card still solves");
    let replaced = replacements(&collector);

    // The card's own value: once, against the card, naming the element that
    // bound it first.
    let pbs: Vec<_> = replaced
        .iter()
        .filter(|(replacement, _)| replacement.parameter == "pbs")
        .collect();
    assert_eq!(pbs.len(), 1, "{replaced:#?}");
    let (pbs, _) = pbs[0];
    assert_eq!(pbs.family, "BSIM4");
    assert_eq!(pbs.scope, ParameterScope::Model);
    assert!(
        pbs.model
            .as_deref()
            .is_some_and(|model| model.eq_ignore_ascii_case("nch")),
        "{pbs:?}"
    );
    assert!(
        pbs.element
            .as_deref()
            .is_some_and(|element| element.eq_ignore_ascii_case("M1")),
        "{pbs:?}"
    );
    assert_eq!(
        (pbs.requested, pbs.applied, pbs.limit),
        (0.05, 0.1, Some(0.1))
    );

    // A size-dependent value: one per geometry, against the element.
    let mut eu: Vec<String> = replaced
        .iter()
        .filter(|(replacement, _)| replacement.parameter == "eu")
        .map(|(replacement, _)| {
            assert_eq!(replacement.scope, ParameterScope::Instance);
            assert_eq!(replacement.applied, 0.0);
            replacement
                .element
                .clone()
                .unwrap_or_default()
                .to_ascii_uppercase()
        })
        .collect();
    eu.sort();
    assert_eq!(eu, ["M1", "M2"], "{replaced:#?}");
}

/// What a WPE clamp clamps is the mobility factor `1 + KU0WE * SCEFF`, which
/// `KU0WE = -0.5` at `SCA = 4` drives to -1 and the model holds at zero. The
/// notice states the parameter the card wrote, -0.5, and the value of it that
/// zeroes the factor at this instance's SCEFF, -1/4: never the factor, which
/// no card spells.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn a_wpe_mobility_clamp_states_the_ku0we_the_card_wrote() {
    let collector = NoticeCollector::new();
    let _installed = collector.install();
    let netlist = Netlist::parse(
        "WPE clamp\nVD d 0 1\nVG g 0 1\nM1 d g 0 0 nch L=1u W=10u sca=4\n\
         .model nch nmos level=54 version=4.8 wpemod=1 ku0we=-0.5\n.op\n.end\n",
    )
    .expect("the deck parses");
    // A channel with no mobility conducts nothing, so the deck is built, which
    // binds the instance, and not solved.
    Engine::default()
        .build_circuit_with_abort(&netlist, &NoAbort)
        .expect("a clamped instance still binds");

    let replaced = replacements(&collector);
    let ku0we: Vec<_> = replaced
        .iter()
        .filter(|(replacement, _)| replacement.parameter == "ku0we")
        .collect();
    assert_eq!(ku0we.len(), 1, "{replaced:#?}");
    let (ku0we, _) = ku0we[0];
    assert_eq!(ku0we.scope, ParameterScope::Instance);
    assert_eq!(
        (ku0we.requested, ku0we.applied, ku0we.limit),
        (-0.5, -0.25, Some(-0.25))
    );
}

/// Recording is an account of the solve, not a part of it: the clamped deck
/// solves to the same bits with a collector installed as without one.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn recording_notices_changes_no_solved_value() {
    let netlist = Netlist::parse(DECK).expect("the deck parses");
    let solve = || {
        let solved = Engine::default()
            .run_dc_op_with_abort(&netlist, &NoAbort)
            .expect("a clamped card still solves");
        solved
            .node_voltages
            .iter()
            .chain(&solved.branch_currents)
            .map(|value| value.to_bits())
            .collect::<Vec<u64>>()
    };
    let unrecorded = solve();
    let collector = NoticeCollector::new();
    let recorded = {
        let _installed = collector.install();
        solve()
    };
    assert!(
        !replacements(&collector).is_empty(),
        "the recorded solve raised its clamps"
    );
    assert_eq!(recorded, unrecorded);
}

/// An element inside a subcircuit is reported by the name the engine solves
/// it under, the written instance names above it joined by dots: the name a
/// host maps back to the occurrence that draws it.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn a_clamp_inside_a_subcircuit_names_the_element_by_its_hierarchical_name() {
    let deck = "BSIM4 clamp in a subcircuit\nVD d 0 1\nVG g 0 1\n\
                .subckt amp d g\nM1 d g 0 0 nch L=1u W=10u\n.ends amp\n\
                XAMP d g amp\n\
                .model nch nmos level=54 version=4.8 eu=-1\n.op\n.end\n";
    let collector = NoticeCollector::new();
    let _installed = collector.install();
    let netlist = Netlist::parse(deck).expect("the deck parses");
    Engine::default()
        .run_dc_op_with_abort(&netlist, &NoAbort)
        .expect("a clamped card still solves");
    let elements: Vec<String> = replacements(&collector)
        .into_iter()
        .filter(|(replacement, _)| replacement.parameter == "eu")
        .filter_map(|(replacement, _)| replacement.element)
        .map(|element| element.to_ascii_uppercase())
        .collect();
    assert_eq!(elements, ["XAMP.M1"]);
}

/// A Monte Carlo run binds the card once per sample, on whichever thread
/// solves it. The clamp reaches the run's collector from the workers as from
/// the run's own thread: one notice, counted, naming the samples the way the
/// run's log lines do.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn a_clamp_every_monte_carlo_sample_raises_is_one_notice_listing_the_samples() {
    const SAMPLES: usize = 12;
    let netlist = Netlist::parse(
        "BSIM4 clamp under Monte Carlo\n.param wid=10u\nVD d 0 1\nVG g 0 1\n\
         M1 d g 0 0 nch L=1u W={wid}\n\
         .model nch nmos level=54 version=4.8 pbs=0.05\n.op\n.end\n",
    )
    .expect("the deck parses");
    for workers in [1, 4] {
        let mut config = SimulationConfig::default();
        config.resource_limits.max_parallel_workers = workers;
        let collector = NoticeCollector::new();
        let result = {
            let _installed = collector.install();
            Engine::new(config)
                .run_monte_carlo(&netlist, SAMPLES, 7)
                .expect("a clamped card still solves")
        };
        assert_eq!(result.num_failures, 0, "{workers} workers");

        let recorded = collector.take();
        let pbs: Vec<_> = recorded
            .iter()
            .filter(|recorded| {
                matches!(
                    &recorded.notice,
                    EngineNotice::ParameterReplaced(replacement) if replacement.parameter == "pbs"
                )
            })
            .collect();
        assert_eq!(pbs.len(), 1, "{workers} workers: {recorded:#?}");
        let pbs = pbs[0];
        assert!(pbs.occurrences >= SAMPLES, "{workers} workers: {pbs:#?}");
        assert_eq!(
            pbs.points.len() + pbs.unnamed_points,
            SAMPLES,
            "{workers} workers: every sample, each once: {pbs:#?}"
        );
        let mut samples: Vec<usize> = pbs
            .points
            .iter()
            .map(|point| {
                point
                    .strip_prefix("sample ")
                    .and_then(|number| number.parse().ok())
                    .unwrap_or_else(|| panic!("a sample label: {point:?}"))
            })
            .collect();
        samples.sort_unstable();
        samples.dedup();
        assert_eq!(samples.len(), pbs.points.len(), "{pbs:#?}");
        assert!(
            samples.iter().all(|sample| (1..=SAMPLES).contains(sample)),
            "{pbs:#?}"
        );
    }
}
