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
