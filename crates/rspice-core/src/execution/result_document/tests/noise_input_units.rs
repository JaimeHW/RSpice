//! Input-referred noise follows the resolved source quantity through publication.

use crate::analysis::noise::NoiseInputQuantity;
use crate::constants::K_BOLTZMANN;
use crate::engine::{Engine, PeriodicNoiseResult, SimulationConfig};
use crate::execution::{AnalysisInstanceId, AnalysisKind, AnalysisResultDocument, SignalUnit};
use crate::netlist::Netlist;

#[test]
fn noise_input_units_follow_resolved_hierarchical_current_sources() {
    let netlist = Netlist::parse(
        "current-referred resistor noise\n\
         .subckt load out\nIref 0 out dc 0 ac 1\nR1 out 0 1k\n.ends\n\
         X1 out load\n.end\n",
    )
    .unwrap();
    let engine = Engine::new(SimulationConfig::default());
    let input_name = engine
        .build_circuit(&netlist)
        .unwrap()
        .current_sources
        .names[0]
        .clone();
    let frequencies = [1e3, 1e4];
    let ordinary = engine
        .run_noise_named_with_input_source(&netlist, "out", None, &input_name, &frequencies, 300.15)
        .unwrap();
    let expected_current = 4.0 * K_BOLTZMANN * 300.15 / 1e3;
    for point in &ordinary {
        assert_eq!(point.input_quantity, Some(NoiseInputQuantity::Current));
        assert!((point.input_referred_density / expected_current - 1.0).abs() < 1e-10);
        assert!((point.output_noise_density / (expected_current * 1e6) - 1.0).abs() < 1e-10);
    }
    let document = AnalysisResultDocument::from_noise(
        AnalysisInstanceId::new(AnalysisKind::Noise, 0),
        &ordinary,
    )
    .unwrap()
    .build()
    .unwrap();
    assert!(
        document
            .signals()
            .iter()
            .any(|signal| { signal.descriptor().unit() == &SignalUnit::Custom("A^2/Hz".into()) })
    );
    assert!(
        document
            .signals()
            .iter()
            .any(|signal| { signal.descriptor().unit() == &SignalUnit::Custom("ohm^2".into()) })
    );
    let mut periodic = engine
        .run_pnoise(
            &netlist,
            1e6,
            &frequencies,
            "out",
            None,
            Some(&input_name),
            0,
        )
        .unwrap();
    assert_eq!(periodic.input_quantity, Some(NoiseInputQuantity::Current));
    for density in periodic.input_noise.as_ref().unwrap() {
        assert!((density / expected_current - 1.0).abs() < 1e-10);
    }
    periodic.integrated_input_noise = Some((expected_current * 9e3).sqrt());
    let document = AnalysisResultDocument::from_pnoise(
        AnalysisInstanceId::new(AnalysisKind::PNoise, 0),
        &PeriodicNoiseResult::Driven {
            output: "V(out)".into(),
            result: periodic,
        },
    )
    .unwrap()
    .build()
    .unwrap();
    assert!(
        document
            .signals()
            .iter()
            .any(|signal| { signal.descriptor().unit() == &SignalUnit::Custom("A^2/Hz".into()) })
    );
    assert!(
        document
            .scalars()
            .iter()
            .any(|scalar| scalar.unit() == Some(&SignalUnit::Ampere))
    );

    let mut inconsistent = ordinary;
    inconsistent[1].input_quantity = Some(NoiseInputQuantity::Voltage);
    assert!(
        AnalysisResultDocument::from_noise(
            AnalysisInstanceId::new(AnalysisKind::Noise, 0),
            &inconsistent,
        )
        .is_err()
    );
}
