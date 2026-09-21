use super::*;
use crate::analysis::quasi_periodic::{
    QuasiPeriodicGridConfig, QuasiPeriodicLinearMethod, QuasiPeriodicNoiseSpectrum,
    QuasiPeriodicSampling,
};
use crate::device::behavioral::{
    BehavioralBranchResolution, BehavioralCurrentSource, BehavioralSources, BehavioralVoltageSource,
};

fn close(actual: Complex64, expected: Complex64, scale: Value) {
    assert!(
        (actual - expected).norm() < 2e-8 * scale,
        "{actual} vs {expected}"
    );
}

#[test]
fn quasi_periodic_integrals_preserve_phase_lift_nested_states_and_conversion() {
    for (rate, method) in [
        (1e3, QuasiPeriodicLinearMethod::Direct),
        (1e9, QuasiPeriodicLinearMethod::Krylov),
    ] {
        let f2 = rate * std::f64::consts::SQRT_2;
        let mut grid_config = QuasiPeriodicGridConfig::new(vec![rate, f2], vec![1, 1]);
        grid_config.sampling = QuasiPeriodicSampling::Exact(vec![8, 8]);
        let limits = ResourceLimits::default();
        let grid =
            Arc::new(QuasiPeriodicGrid::new_with_abort(grid_config, &limits, &NoAbort).unwrap());
        // Deliberately unrelated HB frequency: no common period participates.
        let mut solver = HbSolver::try_new(HbConfig::new(17.0).with_harmonics(1), 4).unwrap();
        solver
            .try_add_periodic_voltage_source_branch(1, 0, 0, 1, "VIN")
            .unwrap();
        for (node, ordinal, name) in [(2, 2, "BV"), (4, 3, "BN")] {
            solver
                .try_add_periodic_constitutive_port_branch(node, 0, ordinal, name)
                .unwrap();
            solver
                .try_add_exact_mna_static_entry(4 + ordinal - 1, node - 1, 1.0, name)
                .unwrap();
        }
        for node in [1, 2, 3] {
            solver.add_resistor(node, 4, 1e3);
        }
        let node = |name: &str| match name {
            "in" => Some(1),
            "out" => Some(2),
            "sink" => Some(3),
            "nested" => Some(4),
            _ => None,
        };
        let mut bv = BehavioralVoltageSource::new(
            "BV".into(),
            2,
            0,
            2,
            &format!(".2+{rate}*sdt(v(in)-v(out)+.1*sin(2*pi*{rate}*time)*cos(2*pi*{f2}*time))"),
        )
        .unwrap();
        bv.bind_references(node, |_| BehavioralBranchResolution::MissingDevice)
            .unwrap();
        let mut nested = BehavioralVoltageSource::new(
            "BN".into(),
            4,
            0,
            3,
            &format!("{rate}*sdt(v(in)-v(nested)-{rate}*sdt(v(nested)))"),
        )
        .unwrap();
        nested
            .bind_references(node, |_| BehavioralBranchResolution::MissingDevice)
            .unwrap();
        let mut bi = BehavioralCurrentSource::new(
            "BI".into(),
            3,
            0,
            &format!("-{rate}*sdt(-i(bv)-v(sink)/1k)"),
        )
        .unwrap();
        bi.bind_references(node, |name| {
            if name.eq_ignore_ascii_case("bv") {
                BehavioralBranchResolution::Branch(5)
            } else {
                BehavioralBranchResolution::MissingDevice
            }
        })
        .unwrap();
        let sources = BehavioralSources {
            voltage_sources: vec![bv, nested],
            current_sources: vec![bi],
        };
        solver
            .set_quasi_periodic_behavioral_sources(&sources, &grid)
            .unwrap();
        assert_eq!(solver.physical_branch_count(), 3);
        assert_eq!(solver.unknowns(), 11);
        assert_eq!(
            solver.try_periodic_mna_branch_names().unwrap()[3..],
            ["B:BV:sdt:0", "B:BN:sdt:0", "B:BN:sdt:1", "B:BI:sdt:0"]
        );
        let mut drive = vec![vec![Complex64::ZERO; grid.len()]; 11];
        drive[4][grid.dc_index()] = Complex64::new(0.7, 0.0);
        for (tuple, coefficient) in [(vec![1, 0], 0.4), (vec![0, 1], 0.15)] {
            let k = grid.index_of(&tuple).unwrap();
            drive[4][k] = Complex64::new(coefficient, 0.0);
            drive[4][grid.len() - 1 - k] = drive[4][k].conj();
        }
        let mut config = QuasiPeriodicSolveConfig::default();
        config.linear.method = method;
        config.relative_tolerance = 1e-9;
        config.current_absolute_tolerance = 1e-14;
        let orbit = solver
            .solve_quasi_periodic_with_abort(grid.clone(), &config, &drive, None, &limits, &NoAbort)
            .unwrap();
        let h = |frequency| rate / Complex64::new(rate, std::f64::consts::TAU * frequency);
        let bandpass = |frequency| {
            let s = Complex64::new(0.0, std::f64::consts::TAU * frequency / rate);
            s / (s * s + s + 1.0)
        };
        for (k, tuple) in grid.indices().iter().enumerate() {
            let f = grid.frequencies_hz()[k];
            let forcing = if tuple[0].abs() == 1 && tuple[1].abs() == 1 {
                Complex64::new(0.0, -0.025 * tuple[0] as Value)
            } else {
                Complex64::ZERO
            };
            let out = h(f) * (drive[4][k] + forcing);
            close(orbit.spectra()[1][k], out, 1.0);
            close(orbit.spectra()[2][k], h(f) * out, 1.0);
            close(orbit.spectra()[3][k], bandpass(f) * drive[4][k], 1.0);
        }
        let dc = grid.dc_index();
        for (row, expected) in [(7, 0.5), (8, 0.7), (9, 0.0), (10, 0.7e-3)] {
            close(
                orbit.spectra()[row][dc] * rate,
                Complex64::new(expected, 0.0),
                expected.abs().max(1e-3),
            );
        }
        let mut excitation = vec![vec![Complex64::ZERO; grid.len()]; 11];
        let excited = grid.index_of(&[1, -1]).unwrap();
        excitation[4][excited] = Complex64::ONE;
        let settings = QuasiPeriodicAcConfig {
            linear: config.linear.clone(),
            ..Default::default()
        };
        let offsets = [0.0, 0.13 * rate];
        let ac = solver
            .solve_quasi_periodic_ac_with_abort(
                grid.clone(),
                &settings,
                orbit.spectra(),
                &offsets,
                &excitation,
                &limits,
                &NoAbort,
            )
            .unwrap();
        for (point, offset) in ac.iter().zip(offsets) {
            let frequency = offset + grid.frequencies_hz()[excited];
            close(point.spectra[1][excited], h(frequency), 1.0);
            close(point.spectra[2][excited], h(frequency) * h(frequency), 1.0);
            close(point.spectra[3][excited], bandpass(frequency), 1.0);
            for k in 0..grid.len() {
                if k != excited {
                    close(point.spectra[2][k], Complex64::ZERO, 1.0);
                }
            }
        }
        let mut observation = vec![vec![Complex64::ZERO; grid.len()]; 11];
        observation[2][dc] = Complex64::ONE;
        let noise = QuasiPeriodicNoiseSource {
            name: "input voltage noise".into(),
            injections: vec![(4, Complex64::ONE)],
            spectrum: QuasiPeriodicNoiseSpectrum::White {
                density: vec![1e-18],
                binary_scale_exponent: 0,
            },
        };
        let config = QuasiPeriodicNoiseConfig {
            frequencies_hz: vec![rate * 0.13],
            frequency_lattice: vec![0, 0],
            input_lattices: vec![vec![0, 0]],
            linear: config.linear,
        };
        solver
            .visit_quasi_periodic_noise_with_abort(
                grid,
                &config,
                orbit.spectra(),
                &[observation],
                &[noise],
                &limits,
                &NoAbort,
                |_, point| {
                    let expected = 1e-18 * h(rate * 0.13).norm_sqr().powi(2);
                    close(
                        point.source_covariances[0].values[0],
                        Complex64::new(expected, 0.0),
                        expected,
                    );
                    Ok(())
                },
            )
            .unwrap();
    }
}
