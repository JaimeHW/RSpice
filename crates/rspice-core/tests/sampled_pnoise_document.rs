//! Public PNOISE result-document and sampling-geometry integration.

use rspice_core::analysis::harmonic_balance::HbConfig;
use rspice_core::engine::{Engine, PeriodicNoiseResult};
use rspice_core::execution::{
    AnalysisInstanceId, AnalysisKind, AnalysisResultDocument, SignalUnit,
};
use rspice_core::{Netlist, NoAbort};
use std::f64::consts::TAU;

#[test]
fn sampled_pnoise_card_runs_and_publishes_timing_units_and_geometry() {
    let netlist = Netlist::parse(
        "Sampled card\nI1 0 out SIN(0 1m 1k) AC 1\nR1 out 0 1k\n.options temp=27\n\
         .pnoise lin 3 100 250 out=out input=I1 maxsideband=1 sampling=edge\n\
         + threshold=0 direction=rising occurrence=1 phasetol=1n minslew=1k integratednoise=yes\n.end\n",
    ).unwrap();
    let rspice_core::netlist::AnalysisCommand::Pnoise(card) = &netlist.analyses[0] else {
        panic!()
    };
    let engine = Engine::default();
    let carrier = engine
        .run_hb_with_abort(&netlist, HbConfig::new(1000.0).with_harmonics(4), &NoAbort)
        .unwrap();
    let result = engine
        .run_pnoise_card_from_hb_with_abort(&netlist, card, &carrier.operating_point, &NoAbort)
        .unwrap();
    let PeriodicNoiseResult::Driven { result: exact, .. } = &result else {
        panic!()
    };
    let thermal = 4.0 * rspice_core::constants::K_BOLTZMANN * 300.15 * 1000.0;
    let density = thermal * 3.0 / (TAU * 1000.0).powi(2);
    assert!(
        (exact.integrated_output_noise.unwrap().powi(2) / (density * 150.0) - 1.0).abs() < 1e-8
    );
    exact.sampling.as_ref().unwrap().validate().unwrap();
    let document = AnalysisResultDocument::from_pnoise(
        AnalysisInstanceId::new(AnalysisKind::PNoise, 0),
        &result,
    )
    .unwrap()
    .build()
    .unwrap();
    let output = document
        .signals()
        .iter()
        .find(|signal| signal.descriptor().canonical_name() == "output_noise")
        .unwrap();
    assert_eq!(
        output.descriptor().unit(),
        &SignalUnit::Custom("s^2/Hz".into())
    );
    assert!(
        document
            .scalars()
            .iter()
            .any(|scalar| scalar.unit() == Some(&SignalUnit::Second))
    );
    let json = document.to_json().unwrap();
    assert_eq!(AnalysisResultDocument::from_json(&json).unwrap(), document);
    let older = json.replace("\"schemaVersion\":9", "\"schemaVersion\":8");
    assert!(AnalysisResultDocument::from_json(&older).is_err());
    let mut malformed = exact.clone();
    malformed
        .sampling
        .as_mut()
        .unwrap()
        .output
        .slew_volts_per_second *= -1.0;
    assert!(
        AnalysisResultDocument::from_pnoise(
            AnalysisInstanceId::new(AnalysisKind::PNoise, 0),
            &PeriodicNoiseResult::Driven {
                output: "V(out)".into(),
                result: malformed
            }
        )
        .is_err()
    );
}
