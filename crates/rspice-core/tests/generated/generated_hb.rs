//! Generated compact models through harmonic balance.
//!
//! The generator decides per model whether harmonic balance can sample its
//! residual as `F(x) + dQ(x)/dt`; an admitted model's harmonics must be those
//! of its native twin where one exists, and those of RSpice's own settled
//! transient everywhere.

use num_complex::Complex64;
use rspice_core::analysis::harmonic_balance::HbConfig;
use rspice_core::engine::{Engine, HbAnalysisResult, SimulationConfig, TransientStartupMode};
use rspice_core::netlist::Netlist;
use std::f64::consts::TAU;

const F0: f64 = 1.0e6;
const HARMONICS: usize = 8;

fn engine() -> Engine {
    Engine::new(SimulationConfig::default())
}

#[cfg(feature = "veriloga-model-r2-cmc")]
fn run_hb(deck: &str) -> HbAnalysisResult {
    let netlist = Netlist::parse(deck).expect("parse");
    engine()
        .run_hb(
            &netlist,
            HbConfig::new(F0)
                .with_harmonics(HARMONICS)
                .with_tolerance(1e-9),
        )
        .unwrap_or_else(|error| panic!("HB must run: {error}\n{deck}"))
}

fn spectrum(result: &HbAnalysisResult, name: &str) -> Vec<Complex64> {
    result
        .result
        .spectral_voltages
        .iter()
        .find(|voltage| voltage.node_name.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("node {name} present"))
        .coefficients
        .clone()
}

/// A resistor in a diode clipper: the generated R2_CMC at its default
/// coefficients is the native resistor of the same value, so every harmonic
/// the diode makes must be the same at every node, to the Newton tolerance.
#[cfg(feature = "veriloga-model-r2-cmc")]
#[test]
fn a_generated_resistor_gives_its_native_twins_harmonics() {
    let deck = |resistor: &str| {
        format!(
            "* resistor twin in a clipper\nV1 in 0 SIN(0.4 0.6 1meg)\n{resistor}\n\
             D1 a 0 dm\nC1 a 0 50p\n.model dm D IS=1e-14 CJO=2p\n.end\n"
        )
    };
    let native = run_hb(&deck("R1 in a 1k"));
    let generated = run_hb(&deck("XR1 in a r2_cmc r=1k"));
    for name in ["in", "a"] {
        let reference = spectrum(&native, name);
        let twin = spectrum(&generated, name);
        let scale = reference[1].norm();
        for (k, (native_value, twin_value)) in reference.iter().zip(&twin).enumerate() {
            assert!(
                (native_value - twin_value).norm() <= 1e-7 * scale,
                "V({name}) harmonic {k}: generated {twin_value} against native {native_value}"
            );
        }
    }
    assert!(
        spectrum(&native, "a")[2].norm() > 1e-3,
        "the clipper must make harmonics for the comparison to mean anything"
    );
}

/// The one-sided Fourier coefficients of `waveform` over the period ending
/// at `end`, sampled at `samples` uniform points by linear interpolation:
/// `c_0` the mean and `c_k = (2/N) sum v(t_n) e^{-jk w t_n}`, the convention
/// harmonic balance reports.
fn period_spectrum(time: &[f64], waveform: &[f64], end: f64, samples: usize) -> Vec<Complex64> {
    let period = F0.recip();
    let start = end - period;
    let at = |t: f64| {
        let index = time
            .partition_point(|&sample| sample < t)
            .clamp(1, time.len() - 1);
        let (t0, t1) = (time[index - 1], time[index]);
        let (v0, v1) = (waveform[index - 1], waveform[index]);
        v0 + (v1 - v0) * (t - t0) / (t1 - t0)
    };
    (0..=HARMONICS)
        .map(|k| {
            let sum: Complex64 = (0..samples)
                .map(|n| {
                    let phase = n as f64 / samples as f64;
                    at(start + phase * period) * Complex64::from_polar(1.0, -TAU * k as f64 * phase)
                })
                .sum();
            let scale = if k == 0 { 1.0 } else { 2.0 };
            sum * scale / samples as f64
        })
        .collect()
}

/// Each admitted family's fixture: a model driven by one tone through a
/// resistor. Its harmonic-balance spectrum must be the spectrum of RSpice's
/// own transient once that has settled. A thermal terminal is a node of its
/// own, which the model's thermal network defines: grounding it would add a
/// second constraint where the model already holds it at ambient.
/// The larger MOSFET models need `RUST_MIN_STACK=67108864` in a debug build.
///
/// Settle test: the transient starts from rest (UIC), so no DC solve decides
/// the fixture, and runs `PERIODS` periods at a step of at most T/512; the
/// DC term and first three harmonics of its last period must differ from
/// those of the period before by at most 1e-5 of the fundamental.
/// Allowance, fixed before the first comparison: each of those four
/// coefficients of the harmonic-balance spectrum is within 1e-3 of the
/// fundamental's amplitude of the transient's, which bounds the transient's
/// own truncation error at 512 steps a period with a wide margin.
#[test]
fn generated_models_reach_the_settled_transients_steady_state() {
    const PERIODS: f64 = 24.0;
    const SAMPLES: usize = 512;
    let fixtures: Vec<(&str, &str, &str)> = vec![
        #[cfg(feature = "veriloga-model-r2-cmc")]
        (
            "resistor (R2_CMC)",
            "V1 in 0 SIN(0.5 1 1meg)\nR1 in a 1k\nXR a 0 r2_cmc r=2k\nC1 a 0 50p\n",
            "a",
        ),
        #[cfg(feature = "veriloga-model-diode-cmc")]
        (
            "diode (DIODE_CMC)",
            "V1 in 0 SIN(0.3 0.6 1meg)\nR1 in a 1k\nXD a 0 diode_cmc\n",
            "a",
        ),
        #[cfg(feature = "veriloga-model-mosvar")]
        (
            "MOS varactor (MOSVAR)",
            "V1 in 0 SIN(0 1 1meg)\nR1 in g 10k\nXV g 0 0 mosvar\n",
            "g",
        ),
        #[cfg(feature = "veriloga-model-r2-et-cmc")]
        (
            "electrothermal resistor (R2_ET_CMC)",
            "V1 in 0 SIN(0.5 1 1meg)\nR1 in a 1k\nXR a 0 0 r2_et_cmc r=2k\nC1 a 0 50p\n",
            "a",
        ),
        #[cfg(feature = "veriloga-model-r3-cmc")]
        (
            "three-terminal resistor (R3_CMC)",
            "V1 in 0 SIN(0.5 1 1meg)\nR1 in a 1k\nXR a 0 0 0 r3_cmc\nC1 a 0 50p\n",
            "a",
        ),
        #[cfg(feature = "veriloga-model-hicuml0va")]
        (
            "bipolar transistor (HICUM/L0)",
            "VCC vcc 0 3\nV1 in 0 SIN(0.8 0.02 1meg)\nRB in b 10k\nRC vcc c 1k\n\
             XQ c b 0 0 t hicuml0va\n",
            "c",
        ),
        #[cfg(feature = "veriloga-model-angelov")]
        (
            "GaAs FET (Angelov)",
            "VDD vdd 0 3\nV1 in 0 SIN(-0.5 0.3 1meg)\nRG in g 1k\nRD vdd d 100\n\
             XA d g 0 angelov\n",
            "d",
        ),
        #[cfg(feature = "veriloga-model-bsimcmg110")]
        (
            "FinFET (BSIM-CMG)",
            "VDD vdd 0 1\nV1 in 0 SIN(0.6 0.1 1meg)\nRG in g 1k\nRD vdd d 1k\n\
             XM d g 0 0 bsimcmg110\n",
            "d",
        ),
        #[cfg(feature = "veriloga-model-hisimsotb-va")]
        (
            "thin-BOX SOI MOSFET (HiSIM-SOTB)",
            "VDD vdd 0 1\nV1 in 0 SIN(0.6 0.1 1meg)\nRG in g 1k\nRD vdd d 1k\n\
             XM d g 0 0 hisimsotb_va\n",
            "d",
        ),
        #[cfg(feature = "veriloga-model-bsimimg")]
        (
            "MOSFET (BSIM-IMG)",
            "VDD vdd 0 1\nV1 in 0 SIN(0.6 0.1 1meg)\nRG in g 1k\nRD vdd d 1k\n\
             XM d g 0 0 t bsimimg\n",
            "d",
        ),
    ];
    let mut failures = Vec::new();
    for (family, circuit, node) in fixtures {
        let deck = format!("* {family} periodic steady state\n{circuit}.end\n");
        let netlist = Netlist::parse(&deck).expect("parse");
        let hb = match engine().run_hb(
            &netlist,
            HbConfig::new(F0)
                .with_harmonics(HARMONICS)
                .with_tolerance(1e-9),
        ) {
            Ok(result) => spectrum(&result, node),
            Err(error) => {
                failures.push(format!("{family}: harmonic balance must run: {error}"));
                continue;
            }
        };
        let tstop = PERIODS / F0;
        let tran = match engine().run_tran_with_startup_mode_and_abort(
            &netlist,
            tstop,
            1.0 / (F0 * SAMPLES as f64),
            TransientStartupMode::Uic,
            &rspice_core::NoAbort,
        ) {
            Ok(tran) => tran,
            Err(error) => {
                failures.push(format!(
                    "{family}: transient must run: {error}; HB V({node}) {:?}",
                    &hb[..4]
                ));
                continue;
            }
        };
        let waveform = tran
            .try_voltage_waveform_named(node)
            .unwrap_or_else(|| panic!("{family}: V({node}) present"));
        let last = period_spectrum(&tran.time, waveform, tstop, SAMPLES);
        let before = period_spectrum(&tran.time, waveform, tstop - 1.0 / F0, SAMPLES);
        let fundamental = last[1].norm();
        for k in 0..=3 {
            if (last[k] - before[k]).norm() > 1e-5 * fundamental {
                failures.push(format!(
                    "{family}: V({node}) harmonic {k} has not settled: {} then {}",
                    before[k], last[k]
                ));
            }
            if (hb[k] - last[k]).norm() > 1e-3 * fundamental {
                failures.push(format!(
                    "{family}: V({node}) harmonic {k}: harmonic balance {} against the settled \
                     transient {} (fundamental {fundamental})",
                    hb[k], last[k]
                ));
            }
        }
        eprintln!(
            "{family}: V({node}) HB {:?} transient {:?}",
            &hb[..4],
            &last[..4]
        );
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// A generated model whose generator could not prove its residual
/// `F(x) + dQ(x)/dt` is refused before solving, by instance and module, with
/// the generator's reason: a variable read back after the body assigned it
/// (EKV 2.6), or a `ddt` scaled by a quantity that varies with bias (the
/// ASM-ESD diode).
#[test]
fn harmonic_balance_refuses_a_generated_model_by_its_reason() {
    use rspice_core::identity::AnalysisKind;
    use rspice_core::{SimulationError, UnsupportedCapabilityError};
    let cases: Vec<(&str, &str, &str, &str)> = vec![
        #[cfg(feature = "veriloga-model-ekv-va")]
        (
            "XE d 0 0 0 ekv_va",
            "XE",
            "ekv_va",
            "`E0_Q_1` is read before it is assigned, so it keeps its value from the last \
             accepted time point, which no harmonic-balance spectrum carries",
        ),
        #[cfg(feature = "veriloga-model-asmesd-dio")]
        (
            "XA d 0 asmesd_dio",
            "XA",
            "asmesd_dio",
            "it scales a `ddt` by a quantity that varies with bias, so its residual is not \
             F(x) + dQ(x)/dt",
        ),
    ];
    for (card, instance, module, reason) in cases {
        let deck =
            format!("* refused generated model\nV1 in 0 SIN(0 1 1meg)\nR1 in d 1k\n{card}\n.end\n");
        let netlist = Netlist::parse(&deck).expect("parse");
        let error = engine()
            .run_hb(&netlist, HbConfig::new(F0).with_harmonics(2))
            .map(drop)
            .expect_err("HB must refuse the model");
        let SimulationError::UnsupportedCapability(refusal) = &error else {
            panic!("{module}: expected a capability refusal, got {error}");
        };
        let UnsupportedCapabilityError {
            capability,
            analysis_kind,
            instance: refused_instance,
            module: refused_module,
            detail,
            ..
        } = refusal.as_ref();
        assert_eq!(*capability, "analysis.hb.veriloga_operator", "{error}");
        assert_eq!(
            *analysis_kind,
            Some(AnalysisKind::HarmonicBalance),
            "{error}"
        );
        assert_eq!(refused_instance.as_deref(), Some(instance), "{error}");
        assert!(
            refused_module
                .as_deref()
                .is_some_and(|name| name.eq_ignore_ascii_case(module)),
            "{error}"
        );
        assert!(
            detail.contains("harmonic-balance analysis") && detail.ends_with(reason),
            "{module}: the refusal must name the analysis and the generator's reason: {detail}"
        );
    }
}

/// Periodic AC from the small-signal source `vrf` into sidebands -2..=2 of
/// `V(a)` at three offsets, and the periodic transfer function from it into
/// the upper first sideband, on the deck's harmonic-balance carrier: once
/// solved by periodic AC itself and once retained from `.hb`.
#[cfg(feature = "veriloga-model-r2-cmc")]
fn hb_carrier_transfers(deck: &str) -> Vec<Complex64> {
    use rspice_core::NoAbort;
    use rspice_core::analysis::pac::{PacConfig, PacSweepType};
    use rspice_core::netlist::{FreqVariation, PeriodicSourceSelector, PeriodicSweep, PxfCard};
    let (start, stop) = (0.13 * F0, 0.37 * F0);
    let netlist = Netlist::parse(deck).expect("parse");
    let config = PacConfig::new()
        .with_fundamental(F0)
        .with_sweep(start, stop, 3)
        .with_sweep_type(PacSweepType::Linear)
        .with_sidebands(-2, 2)
        .with_input_source("vrf")
        .with_output_node("a");
    let card = PxfCard {
        sweep: PeriodicSweep {
            variation: FreqVariation::Lin,
            points: 3,
            start_freq: start,
            stop_freq: stop,
        },
        input_source: "vrf".into(),
        input_sideband: 0,
        output_node: "a".into(),
        output_ref: None,
        output_sideband: 1,
        max_sideband: 2,
        reltol: 1e-9,
        abstol: 1e-15,
        source: PeriodicSourceSelector::Preceding,
    };
    let hb = run_hb(deck);
    let fresh = engine()
        .run_pac(&netlist, config.clone())
        .unwrap_or_else(|error| panic!("PAC must run: {error}\n{deck}"));
    let retained = engine()
        .run_pac_from_hb_with_abort(&netlist, config, &hb.operating_point, &NoAbort)
        .unwrap_or_else(|error| panic!("PAC on the HB carrier must run: {error}\n{deck}"));
    let pxf = engine()
        .run_pxf_card_from_hb_with_abort(&netlist, &card, &hb.operating_point, &NoAbort)
        .unwrap_or_else(|error| panic!("PXF on the HB carrier must run: {error}\n{deck}"));
    let mut transfers = Vec::new();
    for pac in [&fresh, &retained] {
        for offset in 0..3 {
            for sideband in -2..=2 {
                transfers.push(
                    pac.result
                        .conversion_matrix
                        .get(offset, sideband, 0)
                        .expect("sideband present"),
                );
            }
        }
    }
    transfers.extend(pxf.points.iter().map(|point| point.transfer));
    assert_eq!(transfers.len(), 2 * 3 * 5 + 3);
    transfers
}

/// A pumped diode mixer whose load resistor is native or the generated
/// R2_CMC.
#[cfg(feature = "veriloga-model-r2-cmc")]
fn pumped_mixer(resistor: &str) -> String {
    format!(
        "* pumped diode mixer\nVLO lo 0 SIN(0.6 0.4 1meg)\nR1 lo a 1k\nVRF rf 0 DC 0\n\
         R2 rf a 10k\n{resistor}\nD1 a 0 dm\nC1 a 0 50p\n.model dm D IS=1e-14 CJO=2p\n.end\n"
    )
}

/// Periodic AC and periodic transfer functions on a harmonic-balance carrier
/// linearize harmonic balance's own samples of a generated model, so the
/// generated R2_CMC in a pumped mixer gives every sideband transfer its native
/// twin gives. Allowance: 1e-6 of the largest native transfer, at each of
/// three offsets, on a carrier periodic AC solves itself and on one retained
/// from `.hb`.
#[cfg(feature = "veriloga-model-r2-cmc")]
#[test]
fn a_generated_resistor_mixes_its_native_twins_sidebands() {
    let native = hb_carrier_transfers(&pumped_mixer("R3 a 0 2k"));
    let twin = hb_carrier_transfers(&pumped_mixer("XR3 a 0 r2_cmc r=2k"));
    let scale = native.iter().map(|value| value.norm()).fold(0.0, f64::max);
    for (index, (twin_value, native_value)) in twin.iter().zip(&native).enumerate() {
        assert!(
            (twin_value - native_value).norm() <= 1e-6 * scale,
            "transfer {index}: generated {twin_value} against native {native_value}"
        );
    }
    assert!(
        native[1].norm() > 1e-3 * scale && native[3].norm() > 1e-3 * scale,
        "the pumped diode must convert into the first sidebands: {native:?}"
    );
}

/// A shooting carrier's period map carries no generated Verilog-A state, so
/// the shooting route refuses a generated model by instance and module before
/// periodic AC or a periodic transfer function could linearize around it.
#[cfg(feature = "veriloga-model-r2-cmc")]
#[test]
fn a_shooting_carrier_refuses_a_generated_model_by_name() {
    use rspice_core::analysis::PssConfig;
    use rspice_core::identity::AnalysisKind;
    use rspice_core::{NoAbort, SimulationError, UnsupportedCapabilityError};
    let netlist = Netlist::parse(&pumped_mixer("XR3 a 0 r2_cmc r=2k")).expect("parse");
    let error = engine()
        .run_pss_operating_point_with_abort(
            &netlist,
            PssConfig::new(F0).with_harmonics(4),
            &NoAbort,
        )
        .map(drop)
        .expect_err("the shooting carrier must refuse the model");
    let SimulationError::UnsupportedCapability(refusal) = &error else {
        panic!("expected a capability refusal, got {error}");
    };
    let UnsupportedCapabilityError {
        capability,
        analysis_kind,
        instance,
        module,
        detail,
        ..
    } = refusal.as_ref();
    assert_eq!(*capability, "analysis.periodic.veriloga", "{error}");
    assert_eq!(*analysis_kind, Some(AnalysisKind::Pss), "{error}");
    assert_eq!(instance.as_deref(), Some("XR3"), "{error}");
    assert!(
        module
            .as_deref()
            .is_some_and(|name| name.eq_ignore_ascii_case("r2_cmc")),
        "{error}"
    );
    assert!(detail.contains("PSS analysis"), "{detail}");
}
