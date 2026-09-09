//! The `#[wasm_bindgen]` export shims.
//!
//! Every export opens one [`ExecutionScope`], which decodes the options
//! object, installs the cancellation control, and starts the deadline, then
//! calls the corresponding Rust entry point with that scope's policy and
//! abort source. No export contains analysis logic and none of them can
//! reach a non-abort core entrypoint.
//!
//! Every analysis export returns a [`WasmResultHandle`]. There is no export
//! that copies a whole result into ordinary JavaScript arrays.

use wasm_bindgen::prelude::*;

use crate::DetailedWasmResult;
use crate::abort::ExecutionScope;
use crate::errors::wasm_error_to_js;
use crate::handles::WasmResultHandle;
use crate::js_interop::serialize_to_js;
use crate::options::WasmResourceLimits;
use crate::runners::deck::{
    DeckExecution, run_authored_deck_document_with_options_and_abort_detailed,
};
use crate::runners::direct::{
    health_check_with_options_and_abort_detailed, run_ac_document_with_options_and_abort_detailed,
    run_dc_sweep_document_with_options_and_abort_detailed,
    run_noise_document_with_options_and_abort_detailed,
    run_operating_point_document_with_options_and_abort_detailed,
    run_transient_document_with_options_and_abort_detailed,
    summarize_netlist_with_options_and_abort_detailed,
};

/// Open the shared per-call scope, projecting any failure into `RSpiceError`.
fn scope(options: JsValue) -> Result<ExecutionScope, JsValue> {
    ExecutionScope::open(options).map_err(|error| wasm_error_to_js(*error))
}

/// Project a typed runner failure into the thrown `RSpiceError`.
fn thrown<T>(outcome: DetailedWasmResult<T>) -> Result<T, JsValue> {
    outcome.map_err(|error| wasm_error_to_js(*error))
}

/// Retain one execution's results behind the shared bounded handle.
fn retain(scope: &ExecutionScope, execution: DeckExecution) -> Result<WasmResultHandle, JsValue> {
    thrown(WasmResultHandle::new(
        &execution.plan,
        execution.coordinates,
        execution.results,
        scope.resource_limits(),
    ))
}

#[wasm_bindgen(js_name = defaultResourceLimits)]
pub fn default_resource_limits_js() -> Result<JsValue, JsValue> {
    serialize_to_js(&WasmResourceLimits::default())
}

#[wasm_bindgen(js_name = healthCheck)]
pub fn health_check_js(options: JsValue) -> Result<JsValue, JsValue> {
    let scope = scope(options)?;
    let report = thrown(health_check_with_options_and_abort_detailed(
        scope.options(),
        &scope.abort(),
    ))?;
    serialize_to_js(&report)
}

#[wasm_bindgen(js_name = summarizeNetlist)]
pub fn summarize_netlist_js(source: &str, options: JsValue) -> Result<JsValue, JsValue> {
    let scope = scope(options)?;
    let summary = thrown(summarize_netlist_with_options_and_abort_detailed(
        source,
        scope.options(),
        &scope.abort(),
    ))?;
    serialize_to_js(&summary)
}

#[wasm_bindgen(js_name = runOperatingPointDocument)]
pub fn run_operating_point_document_js(
    source: &str,
    options: JsValue,
) -> Result<WasmResultHandle, JsValue> {
    let scope = scope(options)?;
    let execution = thrown(
        run_operating_point_document_with_options_and_abort_detailed(
            source,
            scope.options(),
            &scope.abort(),
        ),
    )?;
    retain(&scope, execution)
}

#[wasm_bindgen(js_name = runDcSweepDocument)]
pub fn run_dc_sweep_document_js(
    source: &str,
    source_name: &str,
    start: f64,
    stop: f64,
    step: f64,
    options: JsValue,
) -> Result<WasmResultHandle, JsValue> {
    let scope = scope(options)?;
    let execution = thrown(run_dc_sweep_document_with_options_and_abort_detailed(
        source,
        source_name,
        start,
        stop,
        step,
        scope.options(),
        &scope.abort(),
    ))?;
    retain(&scope, execution)
}

#[wasm_bindgen(js_name = runAcAnalysisDocument)]
pub fn run_ac_document_js(
    source: &str,
    frequencies: Vec<f64>,
    options: JsValue,
) -> Result<WasmResultHandle, JsValue> {
    let scope = scope(options)?;
    let execution = thrown(run_ac_document_with_options_and_abort_detailed(
        source,
        &frequencies,
        scope.options(),
        &scope.abort(),
    ))?;
    retain(&scope, execution)
}

#[wasm_bindgen(js_name = runTransientAnalysisDocument)]
pub fn run_transient_document_js(
    source: &str,
    tstop: f64,
    max_step: f64,
    options: JsValue,
) -> Result<WasmResultHandle, JsValue> {
    let scope = scope(options)?;
    let execution = thrown(run_transient_document_with_options_and_abort_detailed(
        source,
        tstop,
        max_step,
        scope.options(),
        &scope.abort(),
    ))?;
    retain(&scope, execution)
}

#[wasm_bindgen(js_name = runNoiseAnalysisDocument)]
pub fn run_noise_document_js(
    source: &str,
    output_node: &str,
    reference_node: Option<String>,
    input_source: &str,
    frequencies: Vec<f64>,
    options: JsValue,
) -> Result<WasmResultHandle, JsValue> {
    let scope = scope(options)?;
    let execution = thrown(run_noise_document_with_options_and_abort_detailed(
        source,
        output_node,
        reference_node.as_deref(),
        input_source,
        &frequencies,
        scope.options(),
        &scope.abort(),
    ))?;
    retain(&scope, execution)
}

/// Execute a complete authored analog deck, including its canonical
/// DATA/STEP/TEMP axes, and retain every coordinate-local result behind
/// bounded windows.
#[wasm_bindgen(js_name = runAuthoredDeckDocument)]
pub fn run_authored_deck_document_js(
    source: &str,
    options: JsValue,
) -> Result<WasmResultHandle, JsValue> {
    let scope = scope(options)?;
    let execution = thrown(run_authored_deck_document_with_options_and_abort_detailed(
        source,
        scope.options(),
        &scope.abort(),
    ))?;
    retain(&scope, execution)
}

/// Tests that exercise the real JavaScript boundary.
///
/// These run under `wasm-bindgen-test` on Node. Everything they assert is a
/// property of the boundary itself -- typed-array columns, explicit `null`,
/// the thrown error's shape, and the shared control word -- which is exactly
/// what a native test cannot observe.
#[cfg(all(test, target_arch = "wasm32"))]
// Clippy's allow-*-in-tests policy recognizes #[test], but these functions
// are registered by wasm-bindgen-test instead. Panics are test failures here.
#[allow(
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic,
    clippy::unwrap_used
)]
mod wasm_tests {
    use wasm_bindgen::JsCast;
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::*;
    use crate::js_interop::{js_array_property, js_property};

    #[wasm_bindgen_test]
    fn stationary_bjt_bias_leaves_startup_in_wasm() {
        let engine = rspice_core::Engine::default();
        let abort = rspice_core::abort_signal::NoAbort;
        for (kind, isat) in [
            ("NPN", 1e-14),
            ("PNP", 1e-14),
            ("NPN", 1.0),
            ("PNP", 1.0),
            ("NPN", 1e20),
            ("PNP", 1e20),
        ] {
            let netlist = rspice_core::Netlist::parse(&format!(
                "stationary BJT bias\nI1 0 out DC 1 AC 1\nR1 out 0 1\nQ1 0 0 0 qm\n.model qm {kind}(IS={isat})\n.end\n"
            )).unwrap();
            let dc = engine.run_dc_op_with_abort(&netlist, &abort).unwrap();
            assert!((dc.try_voltage_named("out").unwrap() - 1.0).abs() < 1e-12);
            let ac = engine.run_ac_with_abort(&netlist, &[1e6], &abort).unwrap();
            assert!((ac[0].voltages[0] - rspice_core::Complex64::new(1.0, 0.0)).norm() < 1e-12);
            let tran = engine
                .run_tran_with_abort(&netlist, 2e-9, 1e-9, &abort)
                .unwrap();
            assert_eq!(tran.time.last().copied(), Some(2e-9));
            assert!(
                tran.try_voltage_waveform_named("out")
                    .unwrap()
                    .iter()
                    .all(|v| (v - 1.0).abs() < 1e-12)
            );
        }
    }

    #[wasm_bindgen_test]
    fn reverse_mos_preserves_small_output_conductance_in_wasm() {
        let engine = rspice_core::Engine::default();
        let abort = rspice_core::abort_signal::NoAbort;
        for (kind, polarity) in [("NMOS", 1), ("PMOS", -1)] {
            let netlist = rspice_core::Netlist::parse(&format!(
                "reverse MOS output slope\nV1 out 0 DC {} AC 1\nM1 0 0 out 0 mm W=1 L=1\n.model mm {kind}(LEVEL=1 KP=1e20 VTO={} LAMBDA=1e-20 IS=0)\n.end\n",
                2 * polarity, -polarity,
            )).unwrap();
            let ac = engine.run_ac_with_abort(&netlist, &[1e6], &abort).unwrap();
            let branch = ac[0]
                .branch_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("V1"))
                .unwrap();
            assert!(
                (ac[0].currents[branch] + rspice_core::Complex64::new(0.5, 0.0)).norm() < 1e-10
            );
        }
    }

    #[wasm_bindgen_test]
    fn classic_jfet_charge_cycle_is_conservative_in_wasm() {
        use rspice_core::engine::{SimulationConfig, SpiceDialect};
        use rspice_core::numerics::integration::IntegrationMethod;
        let engine = rspice_core::Engine::new(SimulationConfig {
            spice_dialect: SpiceDialect::Ngspice,
            integration_method: IntegrationMethod::BackwardEuler,
            locked_time_grid: Some(std::sync::Arc::new(vec![0.0, 1e-6, 2e-6])),
            ..Default::default()
        });
        // Analytic ngspice depletion charge between -2 V and +0.75 V,
        // crossing the FC=0.5 continuation knee (CGS+CGD = 3 nF).
        let nominal_charge = 6e-9 * (3.0_f64.sqrt() - 0.5_f64.sqrt())
            + 3e-9 / 0.5_f64.powf(1.5) * (0.25 * 0.25 + (0.75 * 0.75 - 0.25) / 4.0);
        // Warm charge anchor from ngspice-46, TEMP=100 C and TNOM=50 C.
        for (low, model_options, options, expected) in [
            (-2.0, "", "", nominal_charge),
            (
                -1.0,
                "TNOM=50",
                ".options TEMP=100\n",
                5.642_174_551_784_039e-9,
            ),
        ] {
            for (kind, polarity) in [("NJF", 1.0), ("PJF", -1.0)] {
                let netlist = rspice_core::Netlist::parse(&format!(
                "JFET charge cycle\nVg gate 0 DC {} PWL(0 {} 1u {} 2u {})\nJ1 0 gate 0 jm\n.model jm {kind}(BETA=1m VTO=-2 IS=0 CGS=1n CGD=2n {model_options})\n{options}.end\n",
                low * polarity, low * polarity, 0.75 * polarity, low * polarity,
            )).unwrap();
                let result = engine
                    .run_tran_with_abort(&netlist, 2e-6, 1e-6, &rspice_core::abort_signal::NoAbort)
                    .unwrap();
                let current = result.try_branch_current_waveform_named("Vg").unwrap();
                let mut charge = 0.0;
                let mut reached_peak = false;
                for (time, current) in result.time.windows(2).zip(&current[1..]) {
                    charge -= current * (time[1] - time[0]);
                    if time[1] == 1e-6 {
                        assert!((charge - polarity * expected).abs() < 2e-16);
                        reached_peak = true;
                    }
                }
                assert!(reached_peak);
                assert!(charge.abs() < 1e-15);
                assert_eq!(engine.convergence_quality().force_accepted_points, 0);
            }
        }
    }

    #[wasm_bindgen_test]
    fn classic_jfet_charge_pss_matches_analytic_rc_in_wasm() {
        for (kind, polarity) in [("NJF", 1.0), ("PJF", -1.0)] {
            let netlist = rspice_core::Netlist::parse(&format!(
                "JFET PSS in WASM\nV1 in 0 DC {} SIN({} {} 1meg)\nR1 in out 1k\nJ1 0 out 0 jm 2 M=3\n.model jm {kind}(IS=0 CGS=100p CGD=50p M=0)\n.end\n",
                -polarity, -polarity, 0.01 * polarity,
            )).unwrap();
            let point = rspice_core::Engine::default()
                .run_pss_operating_point_with_abort(
                    &netlist,
                    rspice_core::analysis::PssConfig::new(1e6)
                        .with_points_per_period(256)
                        .with_tstab_periods(0)
                        .with_tolerance(1e-10),
                    &rspice_core::abort_signal::NoAbort,
                )
                .unwrap();
            assert_eq!(point.shooting_state_basis(), ["J:J1:qgs"]);
            let result = &point.analysis().result;
            let out = result
                .node_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("out"))
                .unwrap();
            let wc = std::f64::consts::TAU * 1e6 * 1e3 * 900e-12;
            for (&time, &actual) in result.time.iter().zip(&result.waveforms[out].values) {
                let phase = std::f64::consts::TAU * 1e6 * time;
                let expected = -polarity
                    + polarity * 0.01 * (phase.sin() - wc * phase.cos()) / (1.0 + wc * wc);
                assert!((actual - expected).abs() < 4e-6);
            }
        }
    }

    #[wasm_bindgen_test]
    fn newton_tracker_rejects_nonfinite_and_stale_success_in_wasm() {
        use rspice_core::solver::{NewtonConfig, NewtonSolver};
        let finite = [1.0; 17];
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let mut solver = NewtonSolver::new(NewtonConfig::default());
            solver.init(finite.to_vec());
            let mut invalid = finite;
            invalid[16] = value;
            assert!(!solver.check_convergence(&invalid));
            assert!(solver.result().is_err());
            assert!(!solver.check_convergence(&finite));
            assert!(solver.check_convergence(&finite));
            assert!(!solver.check_convergence(&[1.0]));
            assert!(solver.result().is_err());
        }
        let mut solver = NewtonSolver::new(NewtonConfig::default());
        solver.init(vec![1.0]);
        assert!(!solver.check_convergence(&[0.0]));
        for (rel_tol, expected) in [(0.5, false), (1.0, true)] {
            let mut solver = NewtonSolver::new(NewtonConfig {
                abs_tol: f64::MAX,
                rel_tol,
                ..NewtonConfig::default()
            });
            solver.init(vec![-f64::MAX; 17]);
            assert_eq!(solver.check_convergence(&[f64::MAX; 17]), expected);
        }
    }

    #[wasm_bindgen_test]
    fn transient_gmin_rescue_observes_cancellation_in_wasm() {
        let netlist = rspice_core::Netlist::parse(
            "cubic continuation rescue\nI1 0 n PULSE(0 -2 0 1f 1f 10n 20n)\nB1 n 0 I={V(n)*V(n)*V(n)-2*V(n)}\n.tran 0 1f uic\n.end\n",
        ).unwrap();
        let engine = rspice_core::Engine::new(rspice_core::engine::SimulationConfig {
            spice_dialect: rspice_core::engine::SpiceDialect::Xyce,
            transient_nonlinear_max_iterations: Some(8),
            convergence_config: rspice_core::ConvergenceConfig {
                gmin_initial: 10.0,
                gmin_target: 1e-15,
                junction_gmin_target: 0.0,
                ..Default::default()
            },
            locked_time_grid: Some(std::sync::Arc::new(vec![0.0, 1e-15])),
            ..Default::default()
        });
        let abort = rspice_core::abort_signal::CountingAbort::new(64);
        let error = engine
            .run_tran_with_abort(&netlist, 1e-15, 1e-15, &abort)
            .unwrap_err();
        assert!(matches!(
            error,
            rspice_core::engine::SimulationError::Aborted
        ));
        assert_eq!(abort.observed_at(), Some(65));
        assert_eq!(abort.polls_after_abort(), 0);
        let completed = engine
            .run_tran_with_abort(&netlist, 1e-15, 1e-15, &rspice_core::abort_signal::NoAbort)
            .unwrap();
        let endpoint = *completed
            .try_voltage_waveform_named("n")
            .unwrap()
            .last()
            .unwrap();
        assert!((endpoint + 1.769_292_354_238_631_4).abs() < 1e-6);
    }

    #[wasm_bindgen_test]
    fn tied_admittance_preserves_dc_ac_and_transient_in_wasm() {
        let engine = rspice_core::Engine::default();
        let abort = rspice_core::abort_signal::NoAbort;
        let expected = rspice_core::Complex64::new(1.0, 0.0)
            / rspice_core::Complex64::new(1.0, std::f64::consts::TAU * 1e6 * 1e-9);
        for device in [
            "R2 out out 1e-20",
            "C2 out out 1e6",
            "D2 out out dm\n.model dm D(IS=1e20 CJO=1e20)",
            "Q2 out out out out qm\n.model qm NPN(IS=1e20 CJE=1e20 CJC=1e20 CJS=1e20)",
            "Q2 out out out out qm\n.model qm PNP(IS=1e20 CJE=1e20 CJC=1e20 CJS=1e20)",
            "J2 out out out jm\n.model jm NJF(BETA=1e20 VTO=-1 IS=1e20 CGS=1e20 CGD=1e20)",
            "M2 out out out out mm W=1 L=1\n.model mm NMOS(LEVEL=1 KP=1e20 VTO=-1 IS=1e20 CGSO=1e20 CGDO=1e20 CGBO=1e20 CBD=1e20 CBS=1e20)",
        ] {
            let netlist = rspice_core::Netlist::parse(&format!(
                "tied admittance\nI1 0 out DC 1 AC 1\nR1 out 0 1\nC1 out 0 1n\n{device}\n.end\n"
            ))
            .unwrap();
            let dc = engine.run_dc_op_with_abort(&netlist, &abort).unwrap();
            assert!((dc.try_voltage_named("out").unwrap() - 1.0).abs() < 1e-12);
            let ac = engine.run_ac_with_abort(&netlist, &[1e6], &abort).unwrap();
            assert!((ac[0].voltages[0] - expected).norm() < 1e-14);
            let tran = engine
                .run_tran_with_abort(&netlist, 2e-9, 1e-9, &abort)
                .unwrap();
            assert_eq!(tran.time.last().copied(), Some(2e-9));
            assert!(
                tran.try_voltage_waveform_named("out")
                    .unwrap()
                    .iter()
                    .all(|v| (v - 1.0).abs() < 1e-12)
            );
            assert_eq!(engine.convergence_quality().force_accepted_points, 0);
        }
    }

    #[wasm_bindgen_test]
    fn current_excitation_scale_and_tied_terminals_are_physical_in_wasm() {
        let engine = rspice_core::Engine::default();
        let abort = rspice_core::abort_signal::NoAbort;
        let expected = rspice_core::Complex64::from_polar(1.0, 37.0_f64.to_radians());
        for magnitude in [1e-15, 1e-16, 1e-300] {
            let netlist = rspice_core::Netlist::parse(&format!(
                "small AC current\nI1 0 out DC 0 AC {magnitude} 37\nR1 out 0 1\n.end\n"
            ))
            .unwrap();
            let result = engine.run_ac_with_abort(&netlist, &[1e3], &abort).unwrap();
            assert!((result[0].voltages[0] / magnitude - expected).norm() < 1e-14);
        }
        for terminals in ["out out", "0 0"] {
            let netlist = rspice_core::Netlist::parse(&format!(
                "tied current\nI1 0 out DC 1 AC 1 37\nI2 {terminals} DC 1e100 PWL(0 1e100 1n -1e100 2n -1e100) AC 1e100 37\nR1 out 0 1\n.end\n"
            )).unwrap();
            let dc = engine.run_dc_op_with_abort(&netlist, &abort).unwrap();
            assert!((dc.try_voltage_named("out").unwrap() - 1.0).abs() < 1e-12);
            let ac = engine.run_ac_with_abort(&netlist, &[1e3], &abort).unwrap();
            assert!((ac[0].voltages[0] - expected).norm() < 1e-14);
            let tran = engine
                .run_tran_with_abort(&netlist, 2e-9, 1e-9, &abort)
                .unwrap();
            assert_eq!(tran.time.last().copied(), Some(2e-9));
            assert!(
                tran.try_voltage_waveform_named("out")
                    .unwrap()
                    .iter()
                    .all(|v| (v - 1.0).abs() < 1e-12)
            );
            assert_eq!(engine.convergence_quality().force_accepted_points, 0);
        }
    }

    #[wasm_bindgen_test]
    fn centered_poisson_fluctuations_survive_large_bias_cancellation_in_wasm() {
        let grid: Vec<_> = (0..=256).map(|index| f64::from(index) * 1e-9).collect();
        let engine = rspice_core::Engine::new(rspice_core::engine::SimulationConfig {
            locked_time_grid: Some(std::sync::Arc::new(grid.clone())),
            ..Default::default()
        });
        for mean in [1e20_f64, 1e40, 1e100] {
            let mut reference = None;
            for (bias, dc) in [(-mean, 0.0), (0.0, -mean), (-mean / 2.0, -mean / 2.0)] {
                let netlist = rspice_core::Netlist::parse(&format!(
                    "centered Poisson\nI1 0 out DC {dc} TRRANDOM(4 1n 0 {mean} {bias}) AC 1 DISTOF1 1\nR1 out 0 {}\n.end\n", 1.0/mean.sqrt()
                )).unwrap();
                let result = engine
                    .run_tran_with_abort(
                        &netlist,
                        grid[256],
                        1e-9,
                        &rspice_core::abort_signal::NoAbort,
                    )
                    .unwrap();
                assert_eq!(result.time, grid);
                let output = result.try_voltage_waveform_named("out").unwrap();
                let average = output[1..].iter().sum::<f64>() / 256.0;
                let variance = output[1..]
                    .iter()
                    .map(|v| (v - average).powi(2))
                    .sum::<f64>()
                    / 256.0;
                assert!(average.abs() < 0.5 && (variance - 1.0).abs() < 0.6);
                if let Some(expected) = &reference {
                    assert_eq!(output, expected);
                } else {
                    reference = Some(output.to_vec());
                }
                assert_eq!(engine.convergence_quality().force_accepted_points, 0);
            }
        }
    }

    #[wasm_bindgen_test]
    fn transient_current_waveform_is_independent_of_its_dc_specification_in_wasm() {
        let netlist = rspice_core::Netlist::parse(
            "independent DC and transient current\nI1 0 out DC 1e100 PWL(0 1 1n 2 2n 2)\nI2 0 out 3\nR1 out 0 1\n.end\n"
        ).unwrap();
        let engine = rspice_core::Engine::default();
        let result = engine
            .run_tran_with_abort(&netlist, 2e-9, 1e-9, &rspice_core::abort_signal::NoAbort)
            .unwrap();
        assert_eq!(result.time.last().copied(), Some(2e-9));
        for (time, actual) in result
            .time
            .iter()
            .zip(result.try_voltage_waveform_named("out").unwrap())
        {
            assert!((actual - 4.0 - (*time / 1e-9).min(1.0)).abs() < 1e-10);
        }
        assert_eq!(engine.convergence_quality().force_accepted_points, 0);
    }

    #[wasm_bindgen_test]
    fn nonlinear_voltage_and_branch_states_have_no_global_rail_in_wasm() {
        let engine = rspice_core::Engine::default();
        for (bias, resistance) in [(5000.0_f64, 1000.0), (-5000.0, 1000.0), (5.0, 1e-14)] {
            let diode_nodes = if bias > 0.0 { "0 out" } else { "out 0" };
            let netlist = rspice_core::Netlist::parse(&format!(
                "reverse diode divider\nV1 in 0 PWL(0 {bias} 1n {end} 2n {end})\nR1 in out {resistance}\nR2 out 0 {resistance}\nD1 {diode_nodes} dm\n.model dm D(IS=1e-14)\n.end\n",
                end = 2.0 * bias,
            )).unwrap();
            let result = engine
                .run_tran_with_abort(&netlist, 2e-9, 1e-9, &rspice_core::abort_signal::NoAbort)
                .unwrap();
            assert_eq!(result.time.last().copied(), Some(2e-9));
            for (time, actual) in result
                .time
                .iter()
                .zip(result.try_voltage_waveform_named("out").unwrap())
            {
                let expected = 0.5 * (1.0 + (*time / 1e-9).min(1.0));
                assert!(
                    (actual / bias - expected).abs() < 1e-8,
                    "at {time}: {actual}"
                );
            }
        }
    }

    #[wasm_bindgen_test]
    fn exact_pulse_corner_and_gmin_recovery_retain_the_physical_root_in_wasm() {
        let netlist = rspice_core::Netlist::parse(
            "cubic continuation\n.options gmin=0\nI1 0 n PULSE(0 -2 0 1f 1f 10n 20n)\nB1 n 0 I={V(n)*V(n)*V(n)-2*V(n)}\n.tran 0 1f uic\n.end\n"
        ).unwrap();
        let engine = rspice_core::Engine::new(rspice_core::engine::SimulationConfig {
            spice_dialect: rspice_core::engine::SpiceDialect::Xyce,
            transient_nonlinear_max_iterations: Some(8),
            convergence_config: rspice_core::ConvergenceConfig {
                gmin_initial: 10.0,
                gmin_target: 1e-15,
                junction_gmin_target: 0.0,
                ..Default::default()
            },
            locked_time_grid: Some(std::sync::Arc::new(vec![0.0, 1e-15])),
            ..Default::default()
        });
        let result = engine
            .run_tran_with_abort(&netlist, 1e-15, 1e-15, &rspice_core::abort_signal::NoAbort)
            .unwrap();
        let voltage = *result
            .try_voltage_waveform_named("n")
            .unwrap()
            .last()
            .unwrap();
        assert!(
            (voltage + 1.769_292_354_238_631_4).abs() < 1e-6,
            "cubic root: {voltage}"
        );
        assert_eq!(engine.convergence_quality().force_accepted_points, 0);
    }

    #[wasm_bindgen_test]
    fn poisson_sources_and_canceling_currents_converge_in_wasm() {
        let engine = rspice_core::Engine::default();
        for mean in [64.0_f64, 1e6, 1e20, 1e100] {
            let netlist = rspice_core::Netlist::parse(&format!(
                "Poisson shunt\nI1 0 out TRRANDOM(4 1n 0 {mean} 0)\nR1 out 0 {}\n.end\n",
                1.0 / mean
            ))
            .unwrap();
            let result = engine
                .run_tran_with_abort(&netlist, 4e-9, 1e-9, &rspice_core::abort_signal::NoAbort)
                .unwrap();
            let output = result.try_voltage_waveform_named("out").unwrap();
            assert_eq!(output[0], 0.0);
            assert_eq!(result.time.last().copied(), Some(4e-9));
            assert!(
                output
                    .iter()
                    .skip(1)
                    .all(|value| (value - 1.0).abs() < 12.0 / mean.sqrt() + 1e-12)
            );
        }
        let netlist = rspice_core::Netlist::parse(
            "canceling sources\nI1 0 out PWL(0 0 1n 1 2n 1 3n 0)\nI2 out 0 PWL(0 0 1n 1 2n 1 3n 0)\nR1 out 0 1\n.end\n"
        ).unwrap();
        let result = engine
            .run_tran_with_abort(&netlist, 4e-9, 1e-9, &rspice_core::abort_signal::NoAbort)
            .unwrap();
        assert_eq!(result.time.last().copied(), Some(4e-9));
        assert!(
            result
                .try_voltage_waveform_named("out")
                .unwrap()
                .iter()
                .all(|value| value.abs() < 1e-14)
        );
    }

    #[wasm_bindgen_test]
    fn noise_zero_origin_and_exact_prefixes_survive_horizon_extension_in_wasm() {
        use std::sync::Arc;
        let grid: Vec<_> = (0..=129).map(|index| f64::from(index) * 0.3e-9).collect();
        let engine = rspice_core::Engine::new(rspice_core::engine::SimulationConfig {
            locked_time_grid: Some(Arc::new(grid.clone())),
            ..Default::default()
        });
        for waveform in [
            "TRNOISE(0 1n 1 1)",
            "TRNOISE(1 1n 1 1 1 .7n .9n)",
            "TRRANDOM(2 1n .3n 1 0)",
        ] {
            for dc in [0.0, 0.25] {
                let netlist = rspice_core::Netlist::parse(&format!(
                    "noise horizon\nV1 out 0 DC {dc} {waveform}\nR1 out 0 1\n.end\n"
                ))
                .unwrap();
                let run = |stop| {
                    engine
                        .run_tran_with_abort(
                            &netlist,
                            stop,
                            1e-9,
                            &rspice_core::abort_signal::NoAbort,
                        )
                        .unwrap()
                };
                let full = run(grid[129]);
                let short = run(grid[57]);
                assert_eq!(short.try_voltage_waveform_named("out").unwrap()[0], dc);
                assert_eq!(short.time, full.time[..short.time.len()]);
                for (actual, expected) in short
                    .voltages
                    .iter()
                    .zip(&full.voltages)
                    .chain(short.branch_currents.iter().zip(&full.branch_currents))
                {
                    for (a, b) in actual.iter().zip(expected) {
                        assert_eq!(a.to_bits(), b.to_bits(), "{waveform}");
                    }
                }
            }
        }
    }

    #[wasm_bindgen_test]
    fn hierarchical_noise_sources_preserve_scoped_parameters_in_wasm() {
        let engine = rspice_core::Engine::default();
        for waveform in ["TRNOISE({amp} 1n 0 0)", "TRRANDOM(2 1n 0 {amp} 0)"] {
            let netlist = rspice_core::Netlist::parse(&format!(
                "hierarchical noise\n.subckt cell p params:amp=0\nV1 p 0 {waveform} AC 2 DISTOF1 1\n.ends cell\nX1 one cell amp=.01\nX2 two cell amp=.02\nR1 one 0 1\nR2 two 0 1\n.options seed=42\n.end\n"
            )).unwrap();
            let run = || {
                engine
                    .run_tran_with_abort(&netlist, 10e-9, 1e-9, &rspice_core::abort_signal::NoAbort)
                    .unwrap()
            };
            let first = run();
            let second = run();
            assert_eq!(first.time, second.time);
            assert_eq!(first.voltages, second.voltages);
            let traces = ["one", "two"].map(|name| {
                let index = first
                    .node_names
                    .iter()
                    .position(|node| node.eq_ignore_ascii_case(name))
                    .unwrap();
                let values = &first.voltages[index];
                assert!(values.iter().any(|value| value.abs() > 0.003));
                values
            });
            assert!(traces[0].iter().zip(traces[1]).any(|(a, b)| *a != b / 2.0));
        }
    }

    #[wasm_bindgen_test]
    fn distortion_annotations_preserve_transient_noise_in_wasm() {
        let engine = rspice_core::Engine::default();
        for waveform in ["TRNOISE(1 1n 0 0)", "TRRANDOM(2 1n 0 1 0)"] {
            for source in ["V1 out 0", "I1 0 out"] {
                let run = |annotation: &str| {
                    let netlist = rspice_core::Netlist::parse(&format!(
                        "noise annotation\n{source} {waveform} AC 2 {annotation}\nR1 out 0 1\n.end\n"
                    )).unwrap();
                    engine
                        .run_tran_with_abort(
                            &netlist,
                            10e-9,
                            1e-9,
                            &rspice_core::abort_signal::NoAbort,
                        )
                        .unwrap()
                };
                let baseline = run("");
                let annotated = run("DISTOF1 1 DISTOF2 .5 90");
                let index = baseline
                    .node_names
                    .iter()
                    .position(|name| name.eq_ignore_ascii_case("out"))
                    .unwrap();
                assert!(
                    baseline.voltages[index]
                        .iter()
                        .any(|value| value.abs() > 0.3)
                );
                assert_eq!(annotated.time, baseline.time);
                assert_eq!(annotated.voltages, baseline.voltages);
            }
        }
    }

    #[wasm_bindgen_test]
    fn waveform_ac_terms_preserve_dc_bias_in_wasm() {
        let engine = rspice_core::Engine::default();
        for waveform in [
            "SIN(.65 .05 1meg)",
            "PULSE(.65 .7 0 1n 1n 5n 10n)",
            "PWL(0 .65 20n .7)",
        ] {
            for source in ["V1 out 0", "I1 0 out"] {
                for (dc, expected) in [("", 0.65), ("DC 0", 0.0)] {
                    let netlist = rspice_core::Netlist::parse(&format!(
                        "waveform bias\n{source} {waveform} AC 2 90 {dc}\nR1 out 0 1\n.end\n"
                    ))
                    .unwrap();
                    let op = engine
                        .run_dc_op_with_abort(&netlist, &rspice_core::abort_signal::NoAbort)
                        .unwrap();
                    let index = op
                        .node_names
                        .iter()
                        .position(|name| name.eq_ignore_ascii_case("out"))
                        .unwrap();
                    assert!((op.node_voltages[index] - expected).abs() < 1e-12);
                    let ac = engine
                        .run_ac_with_abort(&netlist, &[1e3], &rspice_core::abort_signal::NoAbort)
                        .unwrap();
                    let index = ac[0]
                        .node_names
                        .iter()
                        .position(|name| name.eq_ignore_ascii_case("out"))
                        .unwrap();
                    assert!(
                        (ac[0].voltages[index] - rspice_core::Complex64::new(0.0, 2.0)).norm()
                            < 1e-12
                    );
                }
            }
        }
    }

    #[wasm_bindgen_test]
    fn legacy_itf_scaling_preserves_input_charge_in_wasm() {
        let vt = 300.15 * 1.380649e-23 / 1.602176634e-19_f64;
        let forward = 1e-16 * (0.65 / vt).exp_m1();
        let conductance = 1e-16 / vt * (0.65 / vt).exp();
        let fraction = forward / (forward + 1e-4);
        let extra = 3.0 * ((0.65 - 0.72) / 14.4_f64).exp() * fraction.powi(2);
        let capacitance =
            1e-9 * (conductance * (1.0 + extra * (3.0 - 2.0 * fraction)) + forward * extra / 14.4);
        let expected = rspice_core::Complex64::new(
            -3.0 * (conductance / 100.0 + 1e-16 / vt * ((0.65 - 0.72) / vt).exp()),
            -3.0 * std::f64::consts::TAU * 1e8 * capacitance,
        );
        let mut config = rspice_core::SimulationConfig::default();
        config.convergence_config.gmin_target = 0.0;
        let engine = rspice_core::Engine::new(config);
        for (kind, p) in [("NPN", 1.0), ("PNP", -1.0)] {
            for instance in ["M=3", "AREA=3", "AREA=1.5 M=2"] {
                let netlist = rspice_core::Netlist::parse(&format!(
                    "* Legacy ITF instance scaling\nVc c 0 {}\nVb b 0 PWL(0 {} 20n {}) AC 1 DC {}\nQ1 c b 0 qm {instance}\n\
                     .model qm {kind}(LEVEL=1 IS=1e-16 BF=100 TF=1n XTF=3 VTF=10 ITF=100u)\n.options gmin=0\n.end\n",
                    p*0.72,p*0.65,p*0.7,p*0.65
                )).unwrap();
                let ac = engine
                    .run_ac_with_abort(&netlist, &[1e8], &rspice_core::abort_signal::NoAbort)
                    .unwrap();
                let slot = ac[0]
                    .branch_names
                    .iter()
                    .position(|name| name.eq_ignore_ascii_case("Vb"))
                    .unwrap();
                assert!((ac[0].currents[slot] - expected).norm() < 1e-11 * expected.norm());
            }
        }
    }

    #[wasm_bindgen_test]
    fn vbic_nonpositive_activation_energies_and_signed_sources_match_xyce_in_wasm() {
        for (energy, dc, ac) in [
            (
                0.0,
                [
                    -1.8759381199128295e-05,
                    -5.974425036580707e-06,
                    5.759162012220747e-06,
                ],
                [
                    rspice_core::Complex64::new(7.475710679968615e-07, -7.2352565019688054e-06),
                    rspice_core::Complex64::new(5.679107136074598e-07, 1.1568126579812277e-05),
                    rspice_core::Complex64::new(-3.0678023251495346e-07, -5.895892422355515e-07),
                ],
            ),
            (
                -0.1,
                [
                    -1.3203041813337315e-05,
                    -4.205786531991028e-06,
                    4.052895950577105e-06,
                ],
                [
                    rspice_core::Complex64::new(5.723237566711971e-07, -7.6979104375324e-06),
                    rspice_core::Complex64::new(5.244002563240605e-07, 1.2426635278632646e-05),
                    rspice_core::Complex64::new(-2.559863313225597e-07, -6.704596659558985e-07),
                ],
            ),
        ] {
            let netlist = rspice_core::Netlist::parse(&format!("* VBIC activation energies\nVc c 0 .1\nVb b 0 +.7\nVs s 0 -.4\nVth th 0 DC 30 AC 1\nQ1 c b 0 s th vm SW_ET=0 M=3\n.model vm NPN(LEVEL=12 IS=1e-16 IBEI=1e-18 IBEN=1e-14 IBCI=1e-18 IBCN=1e-14 ISP=1e-15 IBEIP=1e-18 IBENP=1e-14 IBCIP=1e-16 IBCNP=1e-14 RCX=1 RCI=1 RBX=1 RBI=5 RE=1 RBP=5 RS=1 RTH=1000 CTH=1p CJE=1p CJC=2p CJEP=1p CJCP=1p TF=1n TR=2n TD=1n GMIN=0 TNOM=27 EA={energy} EAIE={energy} EAIC={energy} EAIS={energy} EANE={energy} EANC={energy} EANS={energy} EAP={energy})\n.temp 27\n.options gmin=0\n.end\n")).unwrap();
            let mut config = rspice_core::SimulationConfig::default();
            config.convergence_config.gmin_target = 0.0;
            let engine = rspice_core::Engine::new(config);
            let operating = engine
                .run_dc_op_with_abort(&netlist, &rspice_core::abort_signal::NoAbort)
                .unwrap();
            let small_signal = engine
                .run_ac_with_abort(&netlist, &[1e8], &rspice_core::abort_signal::NoAbort)
                .unwrap();
            for (index, name) in ["Vc", "Vb", "Vs"].into_iter().enumerate() {
                let actual = operating.branch_current_named(name).unwrap();
                assert!((actual - dc[index]).abs() < 2e-6 * dc[index].abs().max(1e-12));
                let column = small_signal[0]
                    .branch_names
                    .iter()
                    .position(|branch| branch.eq_ignore_ascii_case(name))
                    .unwrap();
                assert!(
                    (small_signal[0].currents[column] - ac[index]).norm()
                        < 2e-6 * ac[index].norm().max(1e-12)
                );
            }
        }
    }

    #[wasm_bindgen_test]
    fn vbic13_extrinsic_flicker_matches_xyce_in_wasm() {
        let netlist = rspice_core::Netlist::parse(
            "VBIC extrinsic flicker in WASM\nVcc vcc 0 3\nRc vcc c 1k\nVb drive 0 DC 0.7 AC 1\nRb drive b 1k\nVth th 0 0\nQ1 c b 0 th vm SW_ET=0 M=3\n\
             .model vm NPN(LEVEL=11 IS=1e-16 IBEI=1e-18 IBCI=1e-18 RCX=10 RCI=2 RBX=5 RBI=3 RE=1 RBP=0 RS=0 GMIN=0 IBEIP=0 ISP=0 TNOM=27 WBE=0 KFN=1e-8 AFN=1.5 BFN=0.8)\n.temp 27\n.options gmin=0\n.end\n",
        ).unwrap();
        let points = rspice_core::Engine::default()
            .run_noise_named_with_input_source_and_abort(
                &netlist,
                "c",
                None,
                "Vb",
                &[1.0],
                300.15,
                &rspice_core::abort_signal::NoAbort,
            )
            .unwrap();
        let expected = 3.929629740476495e-10;
        assert!((points[0].output_noise_density - expected).abs() < 2e-7 * expected);
        assert!(points[0].contributions.iter().any(|source| {
            source.identity.device.eq_ignore_ascii_case("Q1")
                && source.identity.mechanism.as_deref() == Some("FN_BEX")
                && source.output_contribution > 0.0
        }));
    }

    #[wasm_bindgen_test]
    fn vbic13_near_early_cutoff_and_transit_current_scaling_in_wasm() {
        let netlist = rspice_core::Netlist::parse(
            "VBIC positive Early cutoff in WASM\nVc c 0 0.6\nVb b 0 0.7\nVth th 0 DC 20 AC 1\nQ1 c b 0 th vm SW_ET=1 M=3\n\
             .model vm NPN(LEVEL=11 IS=1e-16 IBEI=1e-18 IBCI=1e-18 ISP=0 IBEIP=0 VEF=5 VER=3 TCVEF=-0.049999995000000005 TCVER=-0.02 RCX=1 RCI=1 RBX=1 RBI=1 RE=1 RBP=0 RS=0 GMIN=0 TNOM=27 RTH=1000 TMAXCLIP=100 CTH=1p CJE=1p CJC=1p TF=1n TR=2n QTF=0.3 XTF=2 VTF=2 ITF=1e-4 TD=1n QBM=0 NKF=0.4 IKF=1e-4 IKR=2e-4 AVC1=0.05 AVC2=0.3 TAVC=0.01)\n.temp 27\n.options gmin=0\n.end\n",
        ).unwrap();
        let points = rspice_core::Engine::default()
            .run_ac_with_abort(&netlist, &[1e8], &rspice_core::abort_signal::NoAbort)
            .unwrap();
        let point = &points[0];
        for (branch, expected) in [
            (
                "vc",
                rspice_core::Complex64::new(0.001067718922411774, -0.0007753494662417362),
            ),
            (
                "vb",
                rspice_core::Complex64::new(-5.900647010104256e-6, 0.0016378819083698172),
            ),
            (
                "vth",
                rspice_core::Complex64::new(-0.0036276377668730667, -0.0014305243645007673),
            ),
        ] {
            let index = point
                .branch_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case(branch))
                .unwrap();
            assert!((point.currents[index] - expected).norm() < 2e-6 * expected.norm());
        }
    }

    #[wasm_bindgen_test]
    fn vbic13_extreme_thermal_slope_retains_physical_currents_in_wasm() {
        for (vef, coefficient, collector, base) in [
            (
                1e15,
                -0.049_999_999_999_999_99,
                rspice_core::Complex64::new(-11073210790.749592, 8091064247.664278),
                rspice_core::Complex64::new(-91165705.28041226, -9220857782.629837),
            ),
            (
                5.0,
                -0.0499999999995,
                rspice_core::Complex64::new(-0.0010482816638976908, 0.0007608625817640124),
                rspice_core::Complex64::new(-4.054942295825502e-6, -0.000859797326861502),
            ),
            (
                5.0,
                -0.049_999_999_999_999_99,
                rspice_core::Complex64::new(-0.0010482816655048935, 0.0007608625829294785),
                rspice_core::Complex64::new(-4.0549423005197475e-6, -0.0008597973281960521),
            ),
        ] {
            let netlist = rspice_core::Netlist::parse(&format!(
            "VBIC direct residual in WASM\nVc c 0 -0.6\nVb b 0 -0.7\nVth th 0 DC 20 AC 1\nQ1 c b 0 0 th vm SW_ET=0 M=3\n\
             .model vm PNP(LEVEL=12 IS=1e-16 IBEI=1e-18 IBCI=1e-18 ISP=0 IBEIP=0 VEF={vef} VER=3 TCVEF={coefficient:.18} TCVER=-0.02 RCX=1 RCI=1 RBX=1 RBI=1 RE=1 RBP=0 RS=0 GMIN=0 TNOM=27 RTH=1000 CTH=1p CJE=1p CJC=1p TF=1n TR=2n QTF=0.3 TD=1n)\n.temp 27\n.options gmin=0\n.end\n",
        )).unwrap();
            let mut config = rspice_core::SimulationConfig::default();
            config.convergence_config.gmin_target = 0.0;
            let points = rspice_core::Engine::new(config)
                .run_ac_with_abort(&netlist, &[1e8], &rspice_core::abort_signal::NoAbort)
                .unwrap();
            for (branch, expected) in [
                ("vc", collector),
                ("vb", base),
                (
                    "vth",
                    rspice_core::Complex64::new(-0.003, -0.0018849555921538759),
                ),
            ] {
                let index = points[0]
                    .branch_names
                    .iter()
                    .position(|name| name.eq_ignore_ascii_case(branch))
                    .unwrap();
                assert!((points[0].currents[index] - expected).norm() < 2e-6 * expected.norm());
            }
        }
    }

    #[wasm_bindgen_test]
    fn vbic13_early_voltage_cutoff_matches_xyce_in_wasm() {
        let netlist = rspice_core::Netlist::parse(
            "VBIC Early-voltage cutoff in WASM\nVc c 0 1.8\nVb b 0 0.7\nVth th 0 DC 20 AC 1\nQ1 c b 0 th vm SW_ET=1\n\
             .model vm NPN(LEVEL=11 VEF=5 VER=3 TCVEF=-0.05 TCVER=-0.05 IS=1e-16 IBEI=1e-18 IBCI=1e-18 RCX=10 RCI=2 RBX=5 RBI=3 RE=1 RBP=0 RS=0 AVC1=0.05 AVC2=0.3 TAVC=0.01 TD=1n RTH=1000 TCRTH=0.005 TMAXCLIP=100 CTH=1p GMIN=1u TNOM=27)\n.temp 27\n.end\n",
        ).unwrap();
        let points = rspice_core::Engine::default()
            .run_ac_with_abort(&netlist, &[1e8], &rspice_core::abort_signal::NoAbort)
            .unwrap();
        let point = &points[0];
        for (branch, expected) in [
            (
                "vc",
                rspice_core::Complex64::new(-8.663038807350953e-6, 6.275760707091533e-6),
            ),
            (
                "vb",
                rspice_core::Complex64::new(4.317831416641038e-7, -4.178850145275066e-7),
            ),
            (
                "vth",
                rspice_core::Complex64::new(-0.0008111544184809163, -0.000639322846682267),
            ),
        ] {
            let index = point
                .branch_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case(branch))
                .unwrap();
            assert!((point.currents[index] - expected).norm() < 2e-7 * expected.norm());
        }
    }

    #[wasm_bindgen_test]
    fn vbic13_resistance_floor_preserves_small_currents_in_wasm() {
        let netlist = rspice_core::Netlist::parse(
            "VBIC resistance floor precision in WASM\nVc c 0 -1.8\nVb b 0 -0.65\nVth th 0 20\nQ1 c b 0 0 th vm SW_ET=1 M=3 TRISE=20\n\
             .model vm PNP(LEVEL=12 IS=1e-16 NF=1.1 NR=1.2 ISRR=0.7 TNF=0.001 PNJMAXI=1n XISR=1.8 DEAR=0.1 IBEI=1e-18 IBCI=1e-18 IBEIP=0 ISP=0 RCX=0 RCI=0 RBX=0 RBI=0 RE=0 RBP=0 RS=0 GMIN=0 TNOM=27 RTH=1000 CTH=1p TD=1n TF=1n TR=2n)\n.temp 27\n.options gmin=0\n.end\n",
        ).unwrap();
        let mut config = rspice_core::SimulationConfig::default();
        config.convergence_config.gmin_target = 0.0;
        let result = rspice_core::Engine::new(config)
            .run_dc_op_with_abort(&netlist, &rspice_core::abort_signal::NoAbort)
            .unwrap();
        for (branch, expected) in [
            ("Vc", 7.045_220_149_350_812e-8),
            ("Vb", 8.783310826230572e-8),
        ] {
            let actual = result.branch_current_named(branch).unwrap();
            assert!((actual - expected).abs() < 1e-7 * expected);
        }
    }

    #[wasm_bindgen_test]
    fn vbic13_pnjmaxi_global_option_matches_xyce_in_wasm() {
        let netlist = rspice_core::Netlist::parse(
            "VBIC PNJMAXI in WASM\nVc c 0 -1\nVb b 0 -0.8\nQ1 c b 0 0 vm SW_ET=0\n\
             .model vm PNP(LEVEL=12 IS=1e-16 IBEI=0 IBCI=0 IBEIP=0 ISP=0 RCX=1 RCI=1 RBX=1 RBI=1 RE=1 RBP=0 RS=0 GMIN=0 TNOM=27)\n.temp 27\n.options DEVICE PNJMAXI=1u\n.end\n",
        ).unwrap();
        let result = rspice_core::Engine::default()
            .run_dc_op_with_abort(&netlist, &rspice_core::abort_signal::NoAbort)
            .unwrap();
        let current = result.branch_current_named("Vc").unwrap();
        assert!((current - 8.903669004624935e-6).abs() < 1e-12);
    }

    #[wasm_bindgen_test]
    fn vbic_tnf_thermal_derivative_matches_xyce_in_wasm() {
        let netlist = rspice_core::Netlist::parse(
            "VBIC TNF thermal derivative in WASM\nVc c 0 -1.8\nVb b 0 -0.7\nVth th 0 DC 20 AC 1\nQ1 c b 0 0 th vm SW_ET=1 M=3 TRISE=20\n\
             .model vm PNP(LEVEL=12 IS=1e-16 NF=1.1 NR=1.2 ISRR=0.7 TNF=0.001 XISR=1.8 DEAR=0.1 IBEI=1e-18 IBCI=1e-18 IBEIP=0 ISP=0 RCX=10 RCI=2 RBX=5 RBI=3 RE=1 RBP=0 RS=0 GMIN=0 TNOM=27 RTH=1000 CTH=1p TD=1n TF=1n TR=2n)\n.temp 27\n.options gmin=0\n.end\n"
        ).unwrap();
        let points = rspice_core::Engine::default()
            .run_ac_with_abort(&netlist, &[1e8], &rspice_core::abort_signal::NoAbort)
            .unwrap();
        let point = &points[0];
        for (branch, expected) in [
            (
                "vc",
                rspice_core::Complex64::new(1.084013990699429e-6, -7.885275583603683e-7),
            ),
            (
                "vb",
                rspice_core::Complex64::new(8.501427197947536e-7, 9.013856542434545e-7),
            ),
            (
                "vth",
                rspice_core::Complex64::new(-0.0029974552824991404, -0.0018863758535638618),
            ),
        ] {
            let index = point
                .branch_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case(branch))
                .unwrap();
            assert!((point.currents[index] - expected).norm() < 2e-6 * expected.norm());
        }
    }

    #[wasm_bindgen_test]
    fn vbic13_signed_reverse_charge_and_delay_matches_xyce_in_wasm() {
        let netlist = rspice_core::Netlist::parse(
            "Signed VBIC reverse charge and delay in WASM\nVc c 0 -1\nVb b 0 DC 0.1 AC 1\nVth th 0 20\nQ1 c b 0 0 th vm SW_ET=0 M=3\n\
             .model vm PNP(LEVEL=12 IS=1e-8 ISRR=0.7 IBEI=0 IBCI=0 IBEIP=0 ISP=0 RCX=1 RCI=1 RBX=1 RBI=1 RE=1 RBP=10 RS=10 GMIN=0 TNOM=27 IKF=1e-10 IKR=1e-10 QBM=1 NKF=0.4 VEF=3 VER=4 TF=2n TR=1n TD=1n QTF=0.5 XTF=10 ITF=1e-3)\n.temp 27\n.options gmin=0\n.end\n"
        ).unwrap();
        let points = rspice_core::Engine::default()
            .run_ac_with_abort(&netlist, &[1e8], &rspice_core::abort_signal::NoAbort)
            .unwrap();
        let point = &points[0];
        for (branch, expected) in [
            (
                "vc",
                rspice_core::Complex64::new(-1.2339551736312433e-6, 1.4936538192949223e-6),
            ),
            (
                "vb",
                rspice_core::Complex64::new(-2.595701431573616e-11, -4.092821900053193e-6),
            ),
        ] {
            let index = point
                .branch_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case(branch))
                .unwrap();
            assert!((point.currents[index] - expected).norm() < 2e-6 * expected.norm());
        }
    }

    #[wasm_bindgen_test]
    fn vbic13_delayed_avalanche_matches_xyce_in_wasm() {
        for (kind, polarity) in [("NPN", 1.0), ("PNP", -1.0)] {
            let netlist = rspice_core::Netlist::parse(&format!(
                "Delayed VBIC avalanche in WASM\nVc c 0 {}\nVb b 0 DC {} AC 1\nQ1 c b 0 vm SW_ET=0\n\
                 .model vm {kind}(LEVEL=11 IS=1e-16 IBEI=1e-18 IBCI=1e-18 RCX=10 RCI=2 RBX=5 RBI=3 RE=1 RBP=0 RS=0 AVC1=0.05 AVC2=0.3 TD=1n GMIN=1u TNOM=27)\n.temp 27\n.end\n",
                polarity * 1.8, polarity * 0.7,
            )).unwrap();
            let points = rspice_core::Engine::default()
                .run_ac_with_abort(&netlist, &[1e8], &rspice_core::abort_signal::NoAbort)
                .unwrap();
            let point = &points[0];
            for (branch, expected) in [
                (
                    "vc",
                    rspice_core::Complex64::new(-0.001774651889771782, 0.0012868112960925493),
                ),
                (
                    "vb",
                    rspice_core::Complex64::new(9.590581606050419e-5, -9.066589478545957e-5),
                ),
            ] {
                let index = point
                    .branch_names
                    .iter()
                    .position(|name| name.eq_ignore_ascii_case(branch))
                    .unwrap();
                assert!((point.currents[index] - expected).norm() < 2e-7 * expected.norm());
            }
        }
    }

    #[wasm_bindgen_test]
    fn vbic13_extrinsic_avalanche_matches_xyce_in_wasm() {
        for (level, kind, polarity, expected) in [
            (
                11,
                "NPN",
                1.0,
                [-6.251124606622771e-5, 3.931202464614644e-6],
            ),
            (
                12,
                "PNP",
                -1.0,
                [5.919472910615865e-5, 1.1288947081972223e-6],
            ),
        ] {
            let substrate = if level == 12 { " 0" } else { "" };
            let netlist = rspice_core::Netlist::parse(&format!(
                "VBIC13 avalanche in WASM\nVc c 0 {}\nVb b 0 {}\nQ1 c b 0{substrate} vm SW_ET=0\n\
                 .model vm {kind}(LEVEL={level} IS=1e-16 IBEI=1e-18 IBCI=1e-18 RCX=10 RCI=2 RBX=5 RBI=3 RE=1 RBP=0 RS=0 AVCX1=0.05 AVCX2=0.3 GMIN=1e-6 TNOM=27)\n.temp 27\n.end\n",
                polarity * 1.8, polarity * 0.7,
            )).unwrap();
            let result = rspice_core::Engine::default()
                .run_dc_op_with_abort(&netlist, &rspice_core::abort_signal::NoAbort)
                .unwrap();
            for (branch, expected) in ["vc", "vb"].into_iter().zip(expected) {
                assert!(
                    (result.branch_current_named(branch).unwrap() - expected).abs()
                        < 2e-7 * expected.abs()
                );
            }
        }
    }

    #[wasm_bindgen_test]
    fn vbic_self_heating_switch_matches_xyce_in_wasm() {
        for (control, current) in [
            ("", -5.69505259e-5),
            ("SW_ET=1", -5.69505259e-5),
            ("SW_ET=0", -5.67002151e-5),
        ] {
            let netlist = rspice_core::Netlist::parse(&format!("* VBIC thermal switch\nVc c 0 1.2\nVb b 0 0.7\nQ1 c b 0 vm {control}\n.model vm NPN(LEVEL=11 IS=1e-16 IBEI=1e-18 IBCI=1e-18 RCI=0 RBI=0 RTH=1000 TNOM=27)\n.temp 27\n.end\n")).unwrap();
            let result = rspice_core::Engine::default()
                .run_dc_op_with_abort(&netlist, &rspice_core::abort_signal::NoAbort)
                .unwrap();
            assert!(
                (result.branch_current_named("vc").unwrap() - current).abs() < 1e-5 * current.abs()
            );
        }
    }

    #[wasm_bindgen_test]
    fn vbic13_clipped_thermal_ac_port_in_wasm() {
        // The thermal current differentiates both theta and R(T(theta)).
        // An omitted CTH must contribute exactly zero imaginary admittance.
        for level in [11, 12] {
            let substrate = if level == 12 { " 0" } else { "" };
            let netlist = rspice_core::Netlist::parse(&format!("* VBIC13 thermal AC\nVth th 0 DC 74 AC 1\nQ1 0 0 0{substrate} th vm SW_ET=0\n.model vm NPN(LEVEL={level} RTH=1000 TCRTH=0.005 TMAXCLIP=100 TNOM=27)\n.temp 27\n.end\n")).unwrap();
            let points = rspice_core::Engine::default()
                .run_ac_with_abort(&netlist, &[1e3], &rspice_core::abort_signal::NoAbort)
                .unwrap();
            let point = &points[0];
            let index = point
                .branch_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("Vth"))
                .unwrap();
            let tail = (-2.0_f64).exp();
            let resistance = 1000.0 * (1.0 + 0.005 * (100.0 - tail - 27.0));
            let expected = -(1.0 / resistance - 74.0 * 5.0 * tail / resistance.powi(2));
            assert!((point.currents[index].re - expected).abs() < 1e-12);
            assert_eq!(point.currents[index].im, 0.0);
        }
    }

    #[wasm_bindgen_test]
    fn vbic_charge_pss_matches_analytic_rc_in_wasm() {
        for (polarity, level) in [("NPN", 4), ("PNP", 4), ("NPN", 11), ("PNP", 11)] {
            let netlist = rspice_core::Netlist::parse(&format!("* VBIC charge PSS\nV1 in 0 SIN(0 0.1 1meg)\nR1 in out 1k\nQ1 0 out 0 0 vm\n.model vm {polarity}(LEVEL={level} IS=1e-40 IBEI=0 IBCI=0 CBEO=159p RCX=0 RCI=0 RBX=0 RBI=0 RBP=0)\n.end\n")).unwrap();
            let analysis = rspice_core::Engine::default()
                .run_pss_with_abort(
                    &netlist,
                    rspice_core::analysis::PssConfig::new(1e6)
                        .with_points_per_period(64)
                        .with_tstab_periods(0),
                    &rspice_core::abort_signal::NoAbort,
                )
                .unwrap();
            let result = &analysis.result;
            let output = result
                .node_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("out"))
                .unwrap();
            let wc = std::f64::consts::TAU * 1e6 * 1e3 * 159e-12;
            for (&time, &value) in result.time.iter().zip(&result.waveforms[output].values) {
                let phase = std::f64::consts::TAU * 1e6 * time;
                let expected = 0.1 * (phase.sin() - wc * phase.cos()) / (1.0 + wc * wc);
                assert!(
                    (value - expected).abs() < 5e-5,
                    "{polarity}: t={time:e}, {value} != {expected}"
                );
            }
        }
    }

    #[wasm_bindgen_test]
    fn jfet_checkpoints_preserve_startup_and_accepted_trajectories_in_wasm() {
        use rspice_core::engine::{
            TransientCheckpoint, TransientCheckpointEncoding, TransientStartupMode,
        };
        let abort = rspice_core::abort_signal::NoAbort;
        for level in [1, 2] {
            let netlist = rspice_core::Netlist::parse(&format!(
                "JFET checkpoint in WASM\nVDD supply 0 5\nVIN in 0 DC -1 PULSE(-1 -0.2 20n 5n 5n 70n 150n)\nRG in gate 1k\nRD supply drain 1k\nJ1 drain gate 0 jm IC=0.1,-0.2\n.model jm NJF(LEVEL={level} BETA=1e-4 VTO=-2 IS=1e-30 CGS=100p CGD=50p TAUG=40n TAUD=60n LFGAM=0.03 DELTA=0.01)\n.end\n",
            )).unwrap();
            let engine = rspice_core::Engine::default();
            for startup in [
                TransientStartupMode::OperatingPoint,
                TransientStartupMode::Uic,
            ] {
                let (full, scheduled) = engine
                    .run_tran_checkpoint_schedule_with_startup_mode_and_abort(
                        &netlist,
                        150e-9,
                        2e-9,
                        startup,
                        &[0.0, 77.3e-9],
                        &abort,
                    )
                    .unwrap();
                assert_eq!(scheduled.len(), 2);
                for captured in scheduled {
                    let checkpoint = TransientCheckpoint::from_bytes(
                        &captured
                            .checkpoint
                            .to_bytes_with_abort(TransientCheckpointEncoding::Packed, &abort)
                            .unwrap(),
                    )
                    .unwrap();
                    let (resumed, _) = engine
                        .run_tran_resume_with_abort(&netlist, &checkpoint, 150e-9, 2e-9, &abort)
                        .unwrap();
                    let offset = full
                        .time
                        .iter()
                        .position(|time| time.to_bits() == checkpoint.time.to_bits())
                        .unwrap();
                    assert_eq!(resumed.time, full.time[offset..]);
                    assert_eq!(resumed.node_names, full.node_names);
                    assert_eq!(resumed.branch_names, full.branch_names);
                    for (actual, expected) in resumed
                        .voltages
                        .iter()
                        .zip(&full.voltages)
                        .chain(resumed.branch_currents.iter().zip(&full.branch_currents))
                    {
                        assert_eq!(actual.len(), expected.len() - offset);
                        for (actual, expected) in actual.iter().zip(&expected[offset..]) {
                            assert_eq!(actual.to_bits(), expected.to_bits());
                        }
                    }
                }
            }
        }
    }

    #[wasm_bindgen_test]
    fn promoted_vbic_checkpoint_continues_exactly_in_wasm() {
        use rspice_core::engine::{
            TransientCheckpoint, TransientCheckpointEncoding, TransientStartupMode,
        };
        let abort = rspice_core::abort_signal::NoAbort;
        for (kind, polarity) in [("NPN", 1.0), ("PNP", -1.0)] {
            let netlist = rspice_core::Netlist::parse(&format!(
            "* VBIC checkpoint in WASM\nVCC supply 0 {}\nVIN base 0 DC {} SIN({} {} 1G)\nRC supply out 1k\nRE emitter 0 100\nQ1 out base emitter 0 active\n.model active {kind} LEVEL=4 IS=1e-16 IBEI=1e-18\n+ RCX=10 RCI=60 RBX=10 RBI=40 RE=2 RS=20 RBP=40\n+ CJE=100f CJC=20f CJEP=100f CJCP=400f TF=10p TR=100p\n+ TD=20p SELFT=1 RTH=300 CTH=1p\n.end\n",
            polarity * 3.3, polarity * 0.8, polarity * 0.8, polarity * 0.05,
        )).unwrap();
            let engine = rspice_core::Engine::default();
            let (full, scheduled) = engine
                .run_tran_checkpoint_schedule_with_startup_mode_and_abort(
                    &netlist,
                    0.5e-9,
                    1e-11,
                    TransientStartupMode::OperatingPoint,
                    &[0.237e-9],
                    &abort,
                )
                .unwrap();
            let checkpoint = TransientCheckpoint::from_bytes(
                &scheduled[0]
                    .checkpoint
                    .to_bytes_with_abort(TransientCheckpointEncoding::Packed, &abort)
                    .unwrap(),
            )
            .unwrap();
            let (resumed, _) = engine
                .run_tran_resume_with_abort(&netlist, &checkpoint, 0.5e-9, 1e-11, &abort)
                .unwrap();
            let offset = full
                .time
                .iter()
                .position(|time| time.to_bits() == checkpoint.time.to_bits())
                .unwrap();
            assert_eq!(resumed.time, full.time[offset..]);
            assert_eq!(resumed.node_names, full.node_names);
            for (actual, expected) in resumed
                .voltages
                .iter()
                .zip(&full.voltages)
                .chain(resumed.branch_currents.iter().zip(&full.branch_currents))
            {
                assert_eq!(actual.len(), expected.len() - offset);
                for (actual, expected) in actual.iter().zip(&expected[offset..]) {
                    assert_eq!(actual.to_bits(), expected.to_bits());
                }
            }
            for state in ["rth", "xf1", "xf2"] {
                let name = format!("Q1.__{state}.internal");
                let column = full
                    .node_names
                    .iter()
                    .position(|node| node.eq_ignore_ascii_case(&name))
                    .unwrap();
                assert!(full.voltages[column].iter().any(|value| value.abs() > 1e-8));
            }
        }
    }

    #[wasm_bindgen_test]
    fn periodic_waveform_precision_survives_wasm_time_and_amplitude_scales() {
        for frequency in [1e-300, 1e300, 1e308] {
            let period = 1.0 / frequency;
            for amplitude in [1e-300, 1e300] {
                let mut result = rspice_core::analysis::PssResult::new(period, 1, 129);
                result.time = (0..=128)
                    .map(|index| (index as f64 / 128.0) * period)
                    .collect();
                result.waveforms[0].values = (0..=128)
                    .map(|index| {
                        amplitude * (0.25 + (std::f64::consts::TAU * index as f64 / 128.0).sin())
                    })
                    .collect();
                let expected = 0.25 + 0.5 * (std::f64::consts::TAU / 128.0).sin();
                assert!(
                    (result.voltage_at(1, period / 256.0) / amplitude - expected).abs() < 2e-14
                );
                let harmonics = result.harmonics(1, 1);
                assert_eq!(harmonics.len(), 2);
                assert!((harmonics[0].magnitude / amplitude - 0.25).abs() < 2e-14);
                assert!((harmonics[1].magnitude / amplitude - 1.0).abs() < 2e-14);
                assert!((harmonics[1].phase + 90.0).abs() < 2e-12);
            }
        }
    }

    #[wasm_bindgen_test]
    fn discontinuous_rlc_orbit_preserves_winding_flux_in_wasm() {
        let netlist = rspice_core::Netlist::parse("WASM discontinuous RLC orbit\nB1 in 0 V=if(sin(2*pi*1meg*time+0.1)>0,1,0)\nR1 in out 1k\nC1 out 0 159p\nL1 out load 10u\nR2 load 0 2k\n.options RELTOL=1e-6 VNTOL=1e-8\n.end\n").unwrap();
        let analysis = rspice_core::Engine::default()
            .run_pss_with_abort(
                &netlist,
                rspice_core::analysis::PssConfig::new(1e6)
                    .with_tstab_periods(0)
                    .with_points_per_period(256),
                &rspice_core::abort_signal::NoAbort,
            )
            .unwrap();
        let result = &analysis.result;
        // Initial values from the exact two-state exp(A*t) periodic solution;
        // the native and Python regressions compare the complete waveform.
        for (name, initial) in [("out", 0.09862296652380287), ("load", 0.07147347879650891)] {
            let node = result
                .node_names
                .iter()
                .position(|n| n.eq_ignore_ascii_case(name))
                .unwrap();
            let waveform = &result.waveforms[node];
            assert!((waveform.values[0] - initial).abs() < 1e-5);
            assert!((waveform.values.last().unwrap() - initial).abs() < 1e-5);
            assert!((waveform.dc(&result.time, result.period) - 1.0 / 3.0).abs() < 1e-5);
        }
    }

    #[wasm_bindgen_test]
    fn small_signal_shooting_closes_the_period_in_wasm() {
        let netlist = rspice_core::Netlist::parse(
            "WASM small-signal shooting\nV1 in 0 SIN(0 1u 1meg)\nR1 in out 1k\nC1 out 0 159.154943091895p\n.end\n",
        )
        .unwrap();
        let result = rspice_core::Engine::default()
            .run_pss_with_abort(
                &netlist,
                rspice_core::analysis::PssConfig::new(1e6)
                    .with_tstab_periods(0)
                    .with_points_per_period(512),
                &rspice_core::abort_signal::NoAbort,
            )
            .unwrap();
        let node = result
            .result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .unwrap();
        let voltage = &result.result.waveforms[node].values;
        assert!(result.iterations > 0);
        assert!((voltage[0] + 0.5e-6).abs() < 1e-12);
        assert!((voltage.last().unwrap() - voltage[0]).abs() < 1e-14);
    }

    #[wasm_bindgen_test]
    fn pss_source_periodicity_rejects_endpoint_aliases_in_wasm() {
        for source in [
            "V1 out 0 SIN(0 1 1.5meg)",
            "B1 out 0 V=sin(2*pi*1.5meg*time)",
            "B1 out 0 V=spice_sin(0,1,1.5meg)",
            "B1 out 0 V=1meg*time",
        ] {
            let netlist = rspice_core::Netlist::parse(&format!(
                "WASM nonperiodic forcing\n{source}\nC1 out 0 1p\n.end\n"
            ))
            .unwrap();
            let error = rspice_core::Engine::default()
                .run_pss_with_abort(
                    &netlist,
                    rspice_core::analysis::PssConfig::new(1e6).with_tstab_periods(0),
                    &rspice_core::abort_signal::NoAbort,
                )
                .unwrap_err();
            assert!(
                error
                    .to_string()
                    .contains("analysis.pss.driven_source_waveform"),
                "{source}: {error}"
            );
        }
    }

    #[wasm_bindgen_test]
    fn pss_source_sampling_refines_aliased_harmonics_in_wasm() {
        for (source, dc) in [
            ("V1 in 0 SIN(0 1 128meg)", 0.0),
            ("B1 in 0 V=sin(2*pi*64meg*time)^4", 0.375),
            (
                "B1 in 0 V=if(cos(2*pi*64meg*time+0.1)>0.9999,1,0)",
                0.004501619094809,
            ),
            (
                "B1 in 0 V=exp(-10000*(1-cos(2*pi*64meg*time+0.1)))",
                0.003989472674605,
            ),
            (
                "B1 in 0 V=exp(-1000000*(cos(2*pi*64meg*time+0.1)+0.5*cos(2*pi*128meg*time+0.2)-0.25)^2)",
                0.0003257350825830,
            ),
            (
                "B1 in 0 V=exp(-1000000*((cos(2*pi*64meg*time+0.1)+0.5*cos(2*(2*pi*64meg*time+0.1)))/(sqr(sin(2*pi*64meg*time+0.1))+sqr(cos(2*pi*64meg*time+0.1)))-0.25)^2)",
                0.0003257350825830,
            ),
            (
                "B1 in 0 V=exp(-1000000*((1e-310*(cos(2*pi*64meg*time+0.1)+0.5*cos(2*(2*pi*64meg*time+0.1))))/(1e-310*(sin(2*pi*64meg*time+0.1)^2+cos(2*pi*64meg*time+0.1)^2))-0.25)^2)",
                0.0003257350825830,
            ),
            (
                "B1 in 0 V=exp(-1000000*(exp(cos(2*pi*64meg*time+0.1))+0.5*exp(cos(2*(2*pi*64meg*time+0.1)))-1.5)^2)",
                0.0008530607516274274,
            ),
            (
                "B1 in 0 V=exp(-1000000*(ln(2+cos(2*pi*64meg*time+0.1))+0.5*ln(2+cos(2*(2*pi*64meg*time+0.1)))-1.5)^2)",
                0.001042689086571716,
            ),
            (
                "B1 in 0 V=exp(-1000000*(2.302585092994046*(log10(2+cos(2*pi*64meg*time+0.1))+0.5*log10(2+cos(2*(2*pi*64meg*time+0.1))))-1.5)^2)",
                0.001042689086571716,
            ),
            (
                "B1 in 0 V=exp(-1000000*(sqrt(2+cos(2*pi*64meg*time+0.1))+0.5*sqrt(2+cos(2*(2*pi*64meg*time+0.1)))-2.4)^2)",
                0.001069489245389935,
            ),
            (
                "B1 in 0 V=exp(-1000000*(abs(cos(2*pi*64meg*time+0.1))+0.5*abs(cos(2*(2*pi*64meg*time+0.1)))-0.75)^2)",
                0.04282819034132224,
            ),
            (
                "B1 in 0 V=exp(-1000000*(max(cos(2*pi*64meg*time+0.1),-cos(2*pi*64meg*time+0.1))+0.5*max(cos(2*(2*pi*64meg*time+0.1)),-cos(2*(2*pi*64meg*time+0.1)))-0.75)^2)",
                0.04282819034132224,
            ),
            (
                "B1 in 0 V=exp(-1000000*(min(cos(2*pi*64meg*time+0.1),-cos(2*pi*64meg*time+0.1),1)+0.5*min(cos(2*(2*pi*64meg*time+0.1)),-cos(2*(2*pi*64meg*time+0.1)),1)+0.75)^2)",
                0.04282819034132224,
            ),
            (
                "B1 in 0 V=abs(cos(2*pi*64meg*time+0.1)+0.5*cos(2*pi*128meg*time+0.2)-0.25)<0.001",
                0.000367552653101734,
            ),
            (
                "B1 in 0 V=0.5*(1-pwrs(abs(cos(2*pi*64meg*time+0.1)+0.5*cos(2*pi*128meg*time+0.2)-0.25)-0.001,0))",
                0.000367552653101734,
            ),
            (
                "B1 in 0 V=exp(-1000000*(atan(cos(2*pi*64meg*time+0.1))+0.5*atan(cos(2*(2*pi*64meg*time+0.1)))-0.25)^2)",
                0.00040382551583931654,
            ),
            (
                "B1 in 0 V=exp(-1000000*(sinh(cos(2*pi*64meg*time+0.1))+0.5*sinh(cos(2*(2*pi*64meg*time+0.1)))-0.25)^2)",
                0.00028901630323141536,
            ),
            (
                "B1 in 0 V=exp(-1000000*(cosh(cos(2*pi*64meg*time+0.1))+0.5*cosh(cos(2*(2*pi*64meg*time+0.1)))-1.7)^2)",
                0.01350698003904257,
            ),
            (
                "B1 in 0 V=exp(-1000000*(tanh(cos(2*pi*64meg*time+0.1))+0.5*tanh(cos(2*(2*pi*64meg*time+0.1)))-0.25)^2)",
                0.00041029391299753326,
            ),
            (
                "B1 in 0 V=exp(-1000000*(asinh(cos(2*pi*64meg*time+0.1))+0.5*asinh(cos(2*(2*pi*64meg*time+0.1)))-0.25)^2)",
                0.00036317795605477475,
            ),
            (
                "B1 in 0 V=exp(-1000000*(acosh(2+cos(2*pi*64meg*time+0.1))+0.5*acosh(2+cos(2*(2*pi*64meg*time+0.1)))-1.5)^2)",
                0.00037163118229903647,
            ),
            (
                "B1 in 0 V=exp(-1000000*(atanh(0.5*cos(2*pi*64meg*time+0.1))+0.5*atanh(0.5*cos(2*(2*pi*64meg*time+0.1)))-0.25)^2)",
                0.000607763343389148,
            ),
            (
                "B1 in 0 V=exp(-1000000*(asin(0.5*cos(2*pi*64meg*time+0.1))+0.5*asin(0.5*cos(2*(2*pi*64meg*time+0.1)))-0.25)^2)",
                0.0006247583059893382,
            ),
            (
                "B1 in 0 V=exp(-1000000*(acos(0.5*cos(2*pi*64meg*time+0.1))+0.5*acos(0.5*cos(2*(2*pi*64meg*time+0.1)))-2.1)^2)",
                0.0006251590597194105,
            ),
            (
                "B1 in 0 V=exp(-1000000*(tan(0.5*cos(2*pi*64meg*time+0.1))+0.5*tan(0.5*cos(2*(2*pi*64meg*time+0.1)))-0.25)^2)",
                0.000608698982941285,
            ),
            (
                "B1 in 0 V=exp(-1000000*atan2(cos(2*pi*64meg*time+0.1)+0.5*cos(2*(2*pi*64meg*time+0.1))-0.25,2)^2)",
                0.0006514709387906895,
            ),
            ("B1 in 0 V=atan2(0*sin(2*pi*64meg*time+0.1),-1)", 0.0),
            ("B1 in 0 V=atan(tan(2*pi*64meg*time+0.1))", 0.0),
            ("B1 in 0 V=tanh(tan(2*pi*64meg*time+0.1))", 0.0),
            (
                "B1 in 0 V=atan(sin(2*pi*64meg*time+0.1)/cos(2*pi*64meg*time+0.1))",
                0.0,
            ),
            ("B1 in 0 V=atan(1/cos(2*pi*64meg*time+0.1))", 0.0),
            (
                "B1 in 0 V=tanh(tan(2*pi*64meg*time+0.1)^2)",
                0.6084407392048392,
            ),
            (
                "B1 in 0 V=exp(-1/cos(2*pi*64meg*time+0.1)^2)",
                0.15729920705028186,
            ),
            (
                "B1 in 0 V=exp(-sqr(1/cos(2*pi*64meg*time+0.1)))",
                0.15729920705028186,
            ),
            ("V1 in 0 PULSE(0 1 400p 10p 10p 100p 1u)", 0.00011),
            ("B1 in 0 V=spice_pulse(0,1,400p,10p,10p,100p,1u)", 0.00011),
            (
                "B1 in 0 V=table(mod(time*1e12,1e6),0,0,400,0,410,1,510,1,520,0,1e6,0)",
                0.00011,
            ),
            (
                "V1 in 0 PWL(0 0 400p 0 410p 1 510p 1 520p 0 1u 0) R=0",
                0.00011,
            ),
            (
                "B1 in 0 V=table(time%1u,0,0,400p,0,500p,1,1.5n,1,1.6n,0,1u,0)",
                0.0011,
            ),
        ] {
            let polar_square = source.contains("atan2(0*sin");
            let squared_tangent = source.contains("tanh(tan(") && source.ends_with("^2)");
            let reciprocal_exponential =
                source.contains("exp(-1/cos(") || source.contains("exp(-sqr(1/cos(");
            let bounded_composition = reciprocal_exponential
                || ["atan(tan(", "tanh(tan(", "atan(sin(", "atan(1/cos("]
                    .iter()
                    .any(|prefix| source.contains(prefix));
            let options = if squared_tangent {
                ".options reltol=1e-5 vntol=1e-8\n"
            } else if bounded_composition {
                ".options reltol=1e-4 vntol=1e-8\n"
            } else if polar_square {
                ".options reltol=1e-4\n"
            } else {
                ""
            };
            let netlist = rspice_core::Netlist::parse(&format!(
                "WASM aliased forcing\n{options}{source}\nR1 in out 1k\nC1 out 0 159.154943091895p\n.end\n"
            ))
            .unwrap();
            let analysis = rspice_core::Engine::default()
                .run_pss_with_abort(
                    &netlist,
                    rspice_core::analysis::PssConfig::new(1e6).with_tstab_periods(0),
                    &rspice_core::abort_signal::NoAbort,
                )
                .unwrap();
            let result = &analysis.result;
            let steps = result.time.len() - 1;
            assert!(steps > 256, "{source}");
            let output = result
                .node_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("out"))
                .unwrap();
            let mean = result.waveforms[output].dc(&result.time, result.period);
            if bounded_composition {
                // Analytic sawtooth extrema / independent RC convolution.
                let (low, high) = if reciprocal_exponential {
                    (0.15566623240897726, 0.158933481927689)
                } else if squared_tangent {
                    (0.6038757155372827, 0.6129976417956825)
                } else if source.contains("atan(1/cos(") {
                    (-0.025340899420651316, 0.025340899420651316)
                } else if source.contains("atan(") {
                    (-0.006425394680575902, 0.012850531334919424)
                } else {
                    (-0.00597775083683904, 0.010_400_683_782_896_1)
                };
                let values = &result.waveforms[output].values;
                let tolerance = if reciprocal_exponential { 1e-5 } else { 1e-6 };
                assert!(
                    (values.iter().copied().fold(f64::NEG_INFINITY, f64::max) - high).abs()
                        < tolerance
                );
                assert!(
                    (values.iter().copied().fold(f64::INFINITY, f64::min) - low).abs() < tolerance
                );
            }
            if polar_square {
                let high = std::f64::consts::PI
                    * ((1.0 / 64e6) / (4.0 * 1000.0 * 159.154943091895e-12_f64)).tanh();
                let values = &result.waveforms[output].values;
                assert!(
                    (values.iter().copied().fold(f64::NEG_INFINITY, f64::max) - high).abs() < 1e-5
                );
                assert!((values.iter().copied().fold(f64::INFINITY, f64::min) + high).abs() < 1e-5);
                assert!(steps < 8192);
            }
            let tolerance = if bounded_composition {
                if reciprocal_exponential { 1e-5 } else { 1e-6 }
            } else if dc == 0.00011 || dc == 0.0011 {
                assert!(steps < 1024, "the local source mesh must stay bounded");
                1e-7
            } else if dc < 0.001 {
                1e-6
            } else if dc < 0.01 {
                1e-5
            } else {
                1e-4
            };
            assert!((mean - dc).abs() < tolerance, "{source}: DC {mean}");
        }
    }

    #[wasm_bindgen_test]
    fn pss_preserves_rounded_period_boundaries_in_wasm() {
        let netlist = rspice_core::Netlist::parse(
            "WASM rounded clock\nB1 in 0 V=spice_pulse(0,1,0,1n,1n,40u,100u)\nR1 in out 1k\nC1 out 0 15.9154943091895n\n.end\n",
        )
        .unwrap();
        let analysis = rspice_core::Engine::default()
            .run_pss_with_abort(
                &netlist,
                rspice_core::analysis::PssConfig::new(1e4).with_tstab_periods(0),
                &rspice_core::abort_signal::NoAbort,
            )
            .unwrap();
        let result = &analysis.result;
        let seam = &result.time[result.time.len() - 2..];
        assert_eq!(seam[0], 100.0 * 1e-6);
        assert_eq!(seam[1], 1.0 / 1e4);
        assert_eq!(seam[0].next_up(), seam[1]);
        let output = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .unwrap();
        assert!((result.waveforms[output].dc(&result.time, result.period) - 0.40001).abs() < 1e-4);
    }

    #[wasm_bindgen_test]
    fn pwl_waveform_time_scaling_and_event_enumeration_work_in_wasm() {
        for scale in [1e-18, 1e-30] {
            let waveform = rspice_core::device::pwl_file::PwlWaveform::new(vec![
                (0.0, 0.0),
                (1.0, 1.0),
                (2.0, 0.0),
            ])
            .unwrap()
            .with_scaling(scale, 1.0, 0.0, 0.0);
            assert!((waveform.value_at_repeating(4.5 * scale, Some(0.0)) - 0.5).abs() < 1e-14);
            let netlist = rspice_core::Netlist::parse(&format!(
                "WASM PWL events\nI1 0 out PWL(0 0 {scale:e} 1 {:e} 0) R=0\nR1 out 0 1\n.end\n",
                2.0 * scale,
            ))
            .unwrap();
            let events = rspice_core::Engine::default()
                .transient_source_event_times_with_abort(
                    &netlist,
                    6.5 * scale,
                    scale / 16.0,
                    &[],
                    &rspice_core::abort_signal::NoAbort,
                )
                .unwrap();
            for knot in 0..=6 {
                assert!(
                    events
                        .iter()
                        .any(|time| (time / scale - f64::from(knot)).abs() < 1e-14)
                );
            }
        }
    }

    #[wasm_bindgen_test]
    fn monte_carlo_host_entropy_is_available_and_replayable_in_wasm() {
        use rspice_core::analysis::{MonteCarloConfig, MonteCarloRunner, Tolerance};
        let run = |config| {
            let mut runner = MonteCarloRunner::new(config);
            runner.add_component("R1", 1000.0, Tolerance::uniform(5.0));
            runner
                .run(|variation| Ok::<_, ()>(variation.values.clone()))
                .expect("WASM obtains host entropy without a native clock")
        };
        let original = run(MonteCarloConfig::new(4));
        let seed = original.sampling.expect("resolved seed is retained").seed;
        let replay = run(MonteCarloConfig::new(4).with_seed(seed));
        assert_eq!(
            original.variables["R1"].samples,
            replay.variables["R1"].samples
        );
    }

    const DECK: &str = "browser boundary deck\n\
V1 in 0 PULSE(0 1 0 1n 1n 20n 40n) AC 1\n\
R1 in out 1k\n\
C1 out 0 1p\n\
.TRAN 1n 20n\n\
.END\n";

    /// An options object carrying only a `sharedInt32` control word.
    fn shared_cancellation_options(cancelled: bool) -> JsValue {
        let buffer = js_sys::SharedArrayBuffer::new(4);
        let view = js_sys::Int32Array::new(buffer.as_ref());
        js_sys::Atomics::store(&view, 0, i32::from(cancelled))
            .expect("Node supports Atomics.store on SharedArrayBuffer");

        let cancellation = js_sys::Object::new();
        js_sys::Reflect::set(
            &cancellation,
            &JsValue::from_str("mechanism"),
            &JsValue::from_str("sharedInt32"),
        )
        .expect("set cancellation mechanism");
        js_sys::Reflect::set(&cancellation, &JsValue::from_str("view"), &view)
            .expect("set cancellation view");

        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &JsValue::from_str("cancellation"), &cancellation)
            .expect("set cancellation policy");
        options.into()
    }

    fn error_field(error: &JsValue, name: &str) -> Option<String> {
        js_property(error, name)
            .expect("RSpiceError exposes its structured fields")
            .as_string()
    }

    /// Window columns cross the boundary as typed arrays, and every signal
    /// carries a `Uint8Array` validity mask beside its numbers.
    #[wasm_bindgen_test]
    fn window_columns_cross_the_boundary_as_typed_arrays() {
        let handle = run_authored_deck_document_js(DECK, JsValue::UNDEFINED)
            .expect("the authored deck runs through the JavaScript export");
        let window = handle
            .read_window_js(0, 0, 4)
            .expect("a four-point window is inside the transient");

        let axes = js_array_property(&window, "axes").expect("the window publishes its axes");
        let axis = axes.get(0);
        assert!(
            js_property(&axis, "values")
                .expect("an axis publishes its coordinates")
                .is_instance_of::<js_sys::Float64Array>(),
            "axis coordinates must be a Float64Array"
        );

        let signals =
            js_array_property(&window, "signals").expect("the window publishes its signals");
        let values =
            js_property(&signals.get(0), "values").expect("a signal publishes its samples");
        assert!(
            js_property(&values, "validity")
                .expect("a signal publishes its validity mask")
                .is_instance_of::<js_sys::Uint8Array>(),
            "the validity mask must be a Uint8Array"
        );
        let representation = js_property(&values, "representation")
            .expect("a signal declares its representation")
            .as_string();
        match representation.as_deref() {
            Some("real") => assert!(
                js_property(&values, "values")
                    .expect("a real signal publishes its samples")
                    .is_instance_of::<js_sys::Float64Array>()
            ),
            Some("complex") => {
                for column in ["real", "imaginary"] {
                    assert!(
                        js_property(&values, column)
                            .expect("a complex signal publishes both columns")
                            .is_instance_of::<js_sys::Float64Array>()
                    );
                }
            }
            other => panic!("unexpected sample representation {other:?}"),
        }
    }

    /// Optional metadata is published as explicit `null`, never omitted, so a
    /// consumer can tell absence from a field this build forgot to write.
    #[wasm_bindgen_test]
    fn absent_metadata_is_explicit_null() {
        let handle = run_authored_deck_document_js(DECK, JsValue::UNDEFINED)
            .expect("the authored deck runs through the JavaScript export");
        let metadata = handle
            .result_metadata_js(0)
            .expect("result metadata crosses the boundary");
        assert!(
            js_property(&metadata, "parentAnalysis")
                .expect("the metadata declares its parent analysis")
                .is_null(),
            "a transient has no parent analysis and must say so explicitly"
        );
        let payload = js_property(&metadata, "payload").expect("the metadata declares its payload");
        assert!(
            js_property(&payload, "compression")
                .expect("the payload declares its compression certificate")
                .is_null(),
            "an uncompressed run must publish a null certificate, not omit it"
        );
    }

    /// A control word that is already set cancels the export before it
    /// publishes anything, and the thrown error is the documented one.
    #[wasm_bindgen_test]
    fn a_pre_set_control_word_cancels_the_export() {
        let error = run_authored_deck_document_js(DECK, shared_cancellation_options(true))
            .expect_err("a pre-set control word must cancel the run");
        assert_eq!(error_field(&error, "code").as_deref(), Some("aborted"));
        assert_eq!(
            error_field(&error, "category").as_deref(),
            Some("cancellation")
        );
        assert_eq!(
            js_property(&error, "retryable")
                .expect("RSpiceError declares its retry policy")
                .as_bool(),
            Some(true)
        );
    }

    /// A clear control word does not cancel anything.
    #[wasm_bindgen_test]
    fn a_clear_control_word_runs_to_completion() {
        let handle = run_authored_deck_document_js(DECK, shared_cancellation_options(false))
            .expect("a clear control word must not cancel the run");
        assert!(handle.result_count() > 0);
    }

    /// Cancellation controls this build cannot honour are rejected before any
    /// work, rather than accepted and then ignored.
    #[wasm_bindgen_test]
    fn unhonourable_cancellation_controls_are_rejected() {
        let abort_signal = js_sys::Object::new();
        js_sys::Reflect::set(
            &abort_signal,
            &JsValue::from_str("mechanism"),
            &JsValue::from_str("abortSignal"),
        )
        .expect("set cancellation mechanism");
        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &JsValue::from_str("cancellation"), &abort_signal)
            .expect("set cancellation policy");
        let error = run_authored_deck_document_js(DECK, options.into())
            .expect_err("a DOM AbortSignal cannot interrupt a synchronous call");
        assert_eq!(
            error_field(&error, "code").as_deref(),
            Some("unsupported_cancellation")
        );

        // An ordinary ArrayBuffer is not shared memory, so the caller could
        // never signal through it.
        let view = js_sys::Int32Array::new(&js_sys::ArrayBuffer::new(4).into());
        let cancellation = js_sys::Object::new();
        js_sys::Reflect::set(
            &cancellation,
            &JsValue::from_str("mechanism"),
            &JsValue::from_str("sharedInt32"),
        )
        .expect("set cancellation mechanism");
        js_sys::Reflect::set(&cancellation, &JsValue::from_str("view"), &view)
            .expect("set cancellation view");
        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &JsValue::from_str("cancellation"), &cancellation)
            .expect("set cancellation policy");
        let error = run_authored_deck_document_js(DECK, options.into())
            .expect_err("an ordinary ArrayBuffer must be rejected");
        assert_eq!(
            error_field(&error, "code").as_deref(),
            Some("invalid_argument")
        );
    }

    /// The direct transient route cannot assign identities to attached FFT
    /// results. Its typed refusal directs the caller to the authored route.
    #[wasm_bindgen_test]
    fn a_direct_transient_with_fft_throws_the_typed_refusal() {
        let source = "browser refusal deck\n\
V1 in 0 SIN(0 1 1G)\n\
R1 in out 1k\n\
C1 out 0 1p\n\
.options fft\n\
.FFT v(out) np=16 format=norm\n\
.END\n";
        let error = run_transient_document_js(source, 1.6e-8, 5.0e-11, JsValue::UNDEFINED)
            .expect_err("the direct route must refuse attached FFT results");
        assert_eq!(
            error_field(&error, "code").as_deref(),
            Some("unsupported_deck_analysis")
        );
        assert_eq!(
            error_field(&error, "category").as_deref(),
            Some("unsupported_feature")
        );
        let message = error_field(&error, "message").unwrap_or_default();
        assert!(
            message.contains(".FFT") && message.contains("runAuthoredDeckDocument"),
            "the refusal names the attached card and its supported route: {message}"
        );
    }
}
