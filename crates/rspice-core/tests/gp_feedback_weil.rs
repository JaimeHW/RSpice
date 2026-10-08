//! Independent ngspice 46 feedback oracles and accepted-state continuation.
//!
//! Intrinsic RB/RC/RE, collector feedback and emitter degeneration leave every
//! transistor terminal unknown. The large-signal case adds nonlinear RB,
//! Early/high-injection terms and bias-dependent diffusion charge. Weil is a
//! discrete filter, so its numerical oracle must use the reference time grid.
//! Provenance and the independently checked PNP symmetry live beside the data.
use rspice_core::engine::{TransientCheckpoint, TransientCheckpointEncoding, TransientStartupMode};
use rspice_core::numerics::integration::IntegrationMethod;
use rspice_core::{Engine, GpTransientPhaseModel, Netlist, SimulationConfig, SpiceDialect};
use std::sync::Arc;

struct Case {
    name: &'static str,
    source: &'static str,
    reference: &'static str,
    startup: TransientStartupMode,
}

const CASES: [Case; 5] = [
    Case {
        name: "linear PTF21",
        source: include_str!("testdata/gp_feedback_linear_p21_ngspice46.cir"),
        reference: include_str!("testdata/gp_feedback_linear_p21_ngspice46.tsv"),
        startup: TransientStartupMode::OperatingPoint,
    },
    Case {
        name: "linear PTF90",
        source: include_str!("testdata/gp_feedback_linear_p90_ngspice46.cir"),
        reference: include_str!("testdata/gp_feedback_linear_p90_ngspice46.tsv"),
        startup: TransientStartupMode::OperatingPoint,
    },
    Case {
        name: "nonlinear PTF21",
        source: include_str!("testdata/gp_feedback_nonlinear_p21_ngspice46.cir"),
        reference: include_str!("testdata/gp_feedback_nonlinear_p21_ngspice46.tsv"),
        startup: TransientStartupMode::OperatingPoint,
    },
    Case {
        name: "nonlinear PTF90",
        source: include_str!("testdata/gp_feedback_nonlinear_p90_ngspice46.cir"),
        reference: include_str!("testdata/gp_feedback_nonlinear_p90_ngspice46.tsv"),
        startup: TransientStartupMode::OperatingPoint,
    },
    Case {
        name: "nonlinear PTF90 UIC",
        source: include_str!("testdata/gp_feedback_nonlinear_p90_uic_ngspice46.cir"),
        reference: include_str!("testdata/gp_feedback_nonlinear_p90_uic_ngspice46.tsv"),
        startup: TransientStartupMode::Uic,
    },
];

impl Case {
    fn netlist(&self, polarity: f64) -> Netlist {
        // The control block belongs to the reference simulator's capture;
        // these tests exercise the public core analysis and checkpoint APIs.
        let mut source = self.source.split(".control").next().unwrap().to_owned() + ".end\n";
        if polarity < 0.0 {
            source = source
                .replace("NPN(", "PNP(")
                .replace("VS supply 0 ", "VS supply 0 -")
                .replace("DC 0.78 SIN(0.78 ", "DC -0.78 SIN(-0.78 -");
        }
        Netlist::parse(&source).unwrap()
    }

    fn rows(&self) -> Vec<[f64; 6]> {
        self.reference
            .lines()
            .skip(1)
            .map(|line| {
                line.split_whitespace()
                    .map(|s| s.parse::<f64>().unwrap())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect()
    }
}

fn config(model: GpTransientPhaseModel) -> SimulationConfig {
    let mut config = SimulationConfig {
        gp_transient_phase_model: model,
        integration_method: IntegrationMethod::Trapezoidal,
        ..SimulationConfig::default().with_spice_dialect(SpiceDialect::Ngspice)
    };
    // Remove the separate numerical nodal shunt as well as authored GMIN.
    config.convergence_config.gmin_target = 0.0;
    config
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn feedback_tracks_recorded_ngspice_without_clamping_a_transistor_terminal() {
    for case in &CASES {
        let rows = case.rows();
        // ngspice UIC does not publish a time-zero row. Preserve the core's
        // explicit initial state and compare every captured positive-time row.
        let offset = usize::from(case.startup.is_uic());
        assert_eq!(rows.len(), if offset == 0 { 608 } else { 710 });
        let mut times = Vec::with_capacity(rows.len() + offset);
        if offset != 0 {
            assert!(rows[0][0] > 0.0);
            times.push(0.0);
        }
        times.extend(rows.iter().map(|row| row[0]));
        let grid = Arc::new(times);
        let stop = *grid.last().unwrap();
        for polarity in [1.0, -1.0] {
            let mut policy = config(GpTransientPhaseModel::NgspiceWeil);
            policy.locked_time_grid = Some(grid.clone());
            let result = Engine::new(policy)
                .run_tran_with_startup_mode(&case.netlist(polarity), stop, 8e-12, case.startup)
                .unwrap_or_else(|e| panic!("{}/{polarity}: {e}", case.name));
            assert_eq!(result.time, *grid);
            let outputs = [
                result.try_voltage_waveform_named("c").unwrap(),
                result.try_voltage_waveform_named("b").unwrap(),
                result.try_voltage_waveform_named("e").unwrap(),
                result.try_branch_current_waveform_named("VS").unwrap(),
                result.try_branch_current_waveform_named("VIN").unwrap(),
            ];
            for (column, actual) in outputs.into_iter().enumerate() {
                assert_eq!(actual.len(), rows.len() + offset);
                assert!(actual.iter().all(|value| value.is_finite()));
                let actual = &actual[offset..];
                let low = rows
                    .iter()
                    .map(|r| r[column + 1])
                    .fold(f64::INFINITY, f64::min);
                let high = rows
                    .iter()
                    .map(|r| r[column + 1])
                    .fold(f64::NEG_INFINITY, f64::max);
                let error = rows
                    .iter()
                    .zip(actual)
                    .map(|(r, a)| (r[column + 1] - polarity * a).abs())
                    .fold(0.0, f64::max);
                // One ppm of signal swing plus 2 nV / 2 pA; include startup
                // and DC without subtracting a fitted offset or phase shift.
                let budget = if column < 3 { 2e-9 } else { 2e-12 } + 1e-6 * (high - low);
                assert!(
                    error < budget,
                    "{}/{polarity}/{column}: {error:e}>{budget:e}",
                    case.name
                );
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn adaptive_feedback_checkpoints_preserve_the_accepted_trajectory() {
    for case in &CASES {
        for polarity in [1.0, -1.0] {
            let netlist = case.netlist(polarity);
            for model in [
                GpTransientPhaseModel::NgspiceWeil,
                GpTransientPhaseModel::ExactDelay,
            ] {
                let engine = Engine::new(config(model));
                // An off-grid checkpoint splits an adaptive interval. Resume
                // must retain the controller, nonlinear storage and phase state.
                let (full, checkpoints) = engine
                    .run_tran_checkpoint_schedule_with_startup_mode(
                        &netlist,
                        4.8e-9,
                        8e-12,
                        case.startup,
                        &[1.911e-9],
                    )
                    .unwrap_or_else(|e| panic!("{}/{polarity}/{model:?}: {e}", case.name));
                let checkpoint = TransientCheckpoint::from_bytes(
                    &checkpoints[0]
                        .checkpoint
                        .to_bytes(TransientCheckpointEncoding::Packed)
                        .unwrap(),
                )
                .unwrap();
                let (resumed, _) = engine
                    .run_tran_resume(&netlist, &checkpoint, 4.8e-9, 8e-12)
                    .unwrap_or_else(|e| panic!("resume {}/{polarity}/{model:?}: {e}", case.name));
                let offset = full
                    .time
                    .iter()
                    .position(|t| t.to_bits() == checkpoint.time.to_bits())
                    .unwrap();
                assert_eq!(
                    resumed.time,
                    full.time[offset..],
                    "grid {}/{polarity}/{model:?}",
                    case.name
                );
                assert_eq!(resumed.step_sizes[1..], full.step_sizes[offset + 1..]);
                assert_eq!(resumed.voltages.len(), full.voltages.len());
                assert_eq!(resumed.branch_currents.len(), full.branch_currents.len());
                for (a, b) in resumed
                    .voltages
                    .iter()
                    .chain(&resumed.branch_currents)
                    .zip(full.voltages.iter().chain(&full.branch_currents))
                {
                    assert_eq!(a, &b[offset..], "state {}/{polarity}/{model:?}", case.name);
                }
            }
        }
    }
}
