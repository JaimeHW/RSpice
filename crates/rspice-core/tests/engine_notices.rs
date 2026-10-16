//! A run's advisories are recorded where the engine raises them, against the
//! element and the card it was binding, beside the log line that says so.

use rspice_core::notice::{EngineNotice, NoticeCollector, ParameterReplacement, ParameterScope};
use rspice_core::{Engine, Netlist, NoAbort};

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
