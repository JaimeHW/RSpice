//! Physical-source and projected-noise oracles for the authenticated QPSS catalog.
use super::*;
use crate::{
    abort_signal::CountingAbort,
    analysis::quasi_periodic::{
        QuasiPeriodicNoiseConfig, QuasiPeriodicNoisePoint, QuasiPeriodicSampling,
    },
};

fn config() -> QpssConfig {
    let mut config = QpssConfig::new(vec![1000.0, std::f64::consts::SQRT_2 * 1000.0], vec![1, 1]);
    config.grid.sampling = QuasiPeriodicSampling::Exact(vec![8, 10]);
    config.initial_state = QpssInitialState::DcOperatingPoint;
    config.solver.relative_tolerance = 1e-10;
    config.solver.current_absolute_tolerance = 1e-15;
    config.solver.voltage_absolute_tolerance = 1e-12;
    config
}
fn noise(
    engine: &Engine,
    netlist: &Netlist,
    point: &QpssOperatingPoint,
    sources: &[Source],
    observations: &[(&str, bool)],
    frequencies: Vec<Value>,
) -> Vec<QuasiPeriodicNoisePoint> {
    let engine = engine.resolved_for_netlist(netlist);
    let grid = engine
        .validate_qpss_operating_point_with_abort(netlist, point, &NoAbort)
        .unwrap();
    let circuit = engine.build_circuit_with_abort(netlist, &NoAbort).unwrap();
    let mut solver = engine.qpss_circuit_solver(&circuit, &grid, true).unwrap();
    let outputs: Vec<_> = observations
        .iter()
        .map(|(name, branch)| {
            let row = if *branch {
                point.node_names().len()
                    + point
                        .branch_names()
                        .iter()
                        .position(|n| n.eq_ignore_ascii_case(name))
                        .unwrap()
            } else {
                point
                    .node_names()
                    .iter()
                    .position(|n| n.eq_ignore_ascii_case(name))
                    .unwrap()
            };
            let mut observation = vec![vec![Complex64::ZERO; grid.len()]; point.spectra().len()];
            observation[row][grid.dc_index()] = Complex64::ONE;
            observation
        })
        .collect();
    let request = QuasiPeriodicNoiseConfig {
        frequencies_hz: frequencies,
        frequency_lattice: vec![0, 0],
        input_lattices: grid.indices().to_vec(),
        linear: Default::default(),
    };
    let mut points = Vec::new();
    solver
        .visit_quasi_periodic_noise_with_abort(
            grid,
            &request,
            point.spectra(),
            &outputs,
            sources,
            &engine.config.resource_limits,
            &NoAbort,
            |_, point| {
                points.push(point);
                Ok(())
            },
        )
        .unwrap();
    points
}
fn close(actual: Value, expected: Value) {
    assert!(
        (actual / expected - 1.0).abs() < 2e-7,
        "{actual:e} != {expected:e}"
    );
}

#[test]
fn qpnoise_resistor_catalog_preserves_temperature_ac_resistance_flicker_and_own_current() {
    let mut nodal_voltage = None;
    for tolerance in [0, 2000] {
        let deck = format!(
            "noise catalog\nI1 0 out 1m\nR1 out 0 RM 1k AC=2k TEMP=50\nRquiet out 0 4k NOISY=0\n.model RM R(KF=1e-12 AF=2 EF=1)\n.options device zeroresistancetol={tolerance}\n.end\n"
        );
        let netlist = Netlist::parse(&deck).unwrap();
        let engine = Engine::new(Default::default());
        let point = engine.run_qpss(&netlist, config()).unwrap();
        let sources = engine
            .qpss_noise_sources_with_abort(&netlist, &point, &NoAbort)
            .unwrap();
        assert_eq!(sources.len(), 2);
        assert_eq!(sources[0].name, "R1 thermal");
        assert_eq!(sources[1].name, "R1 flicker");
        let branch = tolerance > 0;
        assert_eq!(
            sources[0].injections[0].0 >= point.node_names().len(),
            branch
        );
        let outputs = if branch {
            vec![("out", false), ("R1", true)]
        } else {
            vec![("out", false)]
        };
        let points = noise(&engine, &netlist, &point, &sources, &outputs, vec![100.0]);
        let z = 1.0 / (1.0 / 2000.0 + 1.0 / 4000.0);
        let kb =
            pnoise_physical_constants(engine.resolved_for_netlist(&netlist).config.spice_dialect)
                .boltzmann;
        // The existing ngspice-compatible resistor law uses TEMP_K + model TNOM_C.
        let white_current = 4.0 * kb * (323.15 + 27.0) / 2000.0;
        let dc_current = 0.001 * 4000.0 / 5000.0;
        let flicker_current = 1e-12 * dc_current * dc_current / 100.0;
        for (index, q) in [white_current, flicker_current].into_iter().enumerate() {
            let covariance = &points[0].source_covariances[index];
            close(covariance.values[0].re, z * z * q);
            if branch {
                close(covariance.values[1].re, z * (z - 2000.0) / 2000.0 * q);
                close(
                    covariance.values[3].re,
                    (z - 2000.0).powi(2) / 2000.0_f64.powi(2) * q,
                );
                assert!(
                    covariance.values[1].re < 0.0,
                    "intrinsic series noise must appear in the branch current"
                );
            }
        }
        if let Some(expected) = nodal_voltage {
            close(points[0].source_covariances[0].values[0].re, expected);
        }
        nodal_voltage = Some(points[0].source_covariances[0].values[0].re);
        let changed = Netlist::parse(&deck.replace("AC=2k", "AC=3k")).unwrap();
        assert!(
            engine
                .qpss_noise_sources_with_abort(&changed, &point, &NoAbort)
                .is_err()
        );
    }
}

#[test]
fn qpnoise_native_bjt_matches_stationary_noise_and_retains_nonlinear_modulation_modes() {
    for driven in [false, true] {
        let amplitude = if driven { ".01" } else { "0" };
        let deck = format!(
            "native BJT noise\nVcc vcc 0 2\nRL vcc collector 2k NOISY=0\nVb1 base1 0 DC .6 AC 1 SIN(.6 {amplitude} 1k)\nVb2 base base1 SIN(0 {amplitude} 1414.2135623730951)\nQ1 collector base 0 NMOD\n.model NMOD NPN(IS=1e-15 BF=100 TF=1n CJE=1p CJC=.5p KF=1e-12 AF=1.2 EF=.8)\n.options GMIN=0 RELTOL=1e-9 ABSTOL=1e-15 VNTOL=1e-12\n.end\n"
        );
        let netlist = Netlist::parse(&deck).unwrap();
        let mut simulation = crate::config::SimulationConfig::default();
        simulation.convergence_config.gmin_target = 0.0;
        simulation.convergence_config.junction_gmin_target = 0.0;
        let engine = Engine::new(simulation);
        let point = engine.run_qpss(&netlist, config()).unwrap();
        let sources = engine
            .qpss_noise_sources_with_abort(&netlist, &point, &NoAbort)
            .unwrap();
        assert!(sources.len() >= 3, "{sources:?}");
        assert!(sources.iter().all(|s| s.name.starts_with("Q1")));
        let colored = sources
            .iter()
            .find_map(|source| {
                if let Spectrum::PowerLaw {
                    modulation,
                    modulation_lattices: Some(tuples),
                    ..
                } = &source.spectrum
                {
                    Some((modulation, tuples))
                } else {
                    None
                }
            })
            .unwrap();
        let grid = engine
            .validate_qpss_operating_point_with_abort(&netlist, &point, &NoAbort)
            .unwrap();
        assert!(colored.1.len() > grid.len());
        if driven {
            let maximum = colored.0.iter().map(|v| v.norm()).fold(0.0_f64, f64::max);
            assert!(
                colored
                    .1
                    .iter()
                    .zip(colored.0)
                    .any(|(tuple, a)| grid.index_of(tuple).is_none() && a.norm() > maximum * 1e-6)
            );
            assert!(sources.iter().any(|source|matches!(&source.spectrum,Spectrum::White {density,..} if density.iter().any(|v|(*v-density[0]).abs()>1e-4))));
        } else {
            let points = noise(
                &engine,
                &netlist,
                &point,
                &sources,
                &[("collector", false)],
                vec![10.0, 100.0],
            );
            let reference = engine
                .run_noise_named_with_input_source_and_abort(
                    &netlist,
                    "collector",
                    None,
                    "Vb1",
                    &[10.0, 100.0],
                    engine.config.temperature,
                    &NoAbort,
                )
                .unwrap();
            for (point, reference) in points.iter().zip(reference) {
                let total = point
                    .source_covariances
                    .iter()
                    .map(|covariance| covariance.values[0].re)
                    .sum();
                close(total, reference.output_noise_density);
            }
        }
    }
}

#[test]
fn bsim3_periodic_noise_qpss_preserves_physical_flicker_and_bias_modulation() {
    const MODELS: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/device/mosfet/bsim3v3/testdata/models018.lib"
    ));
    let engine = Engine::default();
    for driven in [false, true] {
        let amplitude = if driven { 0.03 } else { 0.0 };
        let netlist = Netlist::parse(&format!(
            "BSIM3 QPNOISE\n\
            vdd supply 0 dc 1.8\nrd supply out 3k noisy=0\n\
            vin in mid dc 0.95 ac 1 sin(0.95 {amplitude} 1000)\n\
            vsecond mid 0 dc 0 sin(0 {amplitude} 1414.2135623730951)\n\
            m1 out in 0 0 n018 w=1u l=0.18u m=2\n\
            {}\n.end\n",
            MODELS.replace("level=49", "level=49 noimod=2 ef=0.9")
        ))
        .unwrap();
        let point = engine.run_qpss(&netlist, config()).unwrap();
        let sources = engine
            .qpss_noise_sources_with_abort(&netlist, &point, &NoAbort)
            .unwrap();
        assert_eq!(sources.len(), 2, "{sources:?}");
        assert!(sources.iter().all(|s| s.name.starts_with("M1:")));
        let (modulation, tuples) = sources
            .iter()
            .find_map(|source| {
                if let Spectrum::PowerLaw {
                    exponent,
                    modulation,
                    modulation_lattices: Some(tuples),
                    ..
                } = &source.spectrum
                {
                    assert_eq!(*exponent, 0.9);
                    Some((modulation, tuples))
                } else {
                    None
                }
            })
            .unwrap();
        let grid = engine
            .validate_qpss_operating_point_with_abort(&netlist, &point, &NoAbort)
            .unwrap();
        assert!(tuples.len() > grid.len());
        if driven {
            let maximum = modulation
                .iter()
                .map(|v| v.norm())
                .fold(0.0_f64, Value::max);
            assert!(
                tuples
                    .iter()
                    .zip(modulation)
                    .any(|(tuple, value)| grid.index_of(tuple).is_none()
                        && value.norm() > maximum * 1e-6)
            );
            assert!(sources.iter().any(|source| matches!(&source.spectrum,
                Spectrum::White { density, .. } if density.iter().any(|v| (*v - density[0]).abs() > 1e-4))));
            // A driven orbit must also project to finite, positive output noise.
            let projected = noise(
                &engine,
                &netlist,
                &point,
                &sources,
                &[("out", false)],
                vec![10.0],
            );
            assert!(
                projected[0]
                    .source_covariances
                    .iter()
                    .all(|c| c.values[0].re.is_finite() && c.values[0].re > 0.0)
            );
        } else {
            let projected = noise(
                &engine,
                &netlist,
                &point,
                &sources,
                &[("out", false)],
                vec![10.0, 100.0],
            );
            let reference = engine
                .run_noise_named_with_input_source_and_abort(
                    &netlist,
                    "out",
                    None,
                    "vin",
                    &[10.0, 100.0],
                    engine.config.temperature,
                    &NoAbort,
                )
                .unwrap();
            for (point, reference) in projected.iter().zip(reference) {
                let total: Value = point
                    .source_covariances
                    .iter()
                    .map(|c| c.values[0].re)
                    .sum();
                close(total, reference.output_noise_density);
            }
        }
    }
}

#[test]
fn qpnoise_catalog_includes_physical_shunts_and_observes_cancellation() {
    let netlist = Netlist::parse("physical shunt\nV1 out 0 1\n.options RSHUNT=1k\n.end\n").unwrap();
    let engine = Engine::new(Default::default());
    let point = engine.run_qpss(&netlist, config()).unwrap();
    let sources = engine
        .qpss_noise_sources_with_abort(&netlist, &point, &NoAbort)
        .unwrap();
    assert_eq!(sources.len(), 1);
    assert!(sources[0].name.starts_with("RSHUNT:"));
    let Spectrum::White {
        density,
        binary_scale_exponent,
    } = &sources[0].spectrum
    else {
        unreachable!()
    };
    let resolved = engine.resolved_for_netlist(&netlist);
    let constants = pnoise_physical_constants(resolved.config.spice_dialect);
    close(
        libm::scalbn(density[0], *binary_scale_exponent),
        4.0 * constants.boltzmann * resolved.config.temperature / 1000.0,
    );
    let abort = CountingAbort::new(20);
    assert!(matches!(
        engine.qpss_noise_sources_with_abort(&netlist, &point, &abort),
        Err(SimulationError::Aborted)
    ));
    assert_eq!(abort.count(), 21);
}
