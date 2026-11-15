//! Generated compact models through harmonic balance.
//!
//! The generator decides per model whether harmonic balance can sample its
//! residual as `F(x) + dQ(x)/dt`; an admitted model's harmonics must be those
//! of its native twin where one exists, and those of RSpice's own settled
//! transient everywhere one settles, or of shooting's orbit where none does.

use num_complex::Complex64;
use rspice_core::analysis::PssConfig;
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

/// VBIC 4T et against RSpice's native VBIC on one card, in a common-emitter
/// stage: once with the thermal node grounded, and once self-heating through
/// a thermal resistance and capacitance. Both are VBIC's equations; the
/// generated model takes k and q from CODATA 2010 (`constants.vams`) where
/// the native one takes ngspice's CODATA 2014 values, a 2.4e-7 difference in
/// the thermal voltage that moves the collector current by about 30 times
/// that at this bias. Allowance, fixed from that before the comparison: each
/// of the DC term and harmonics 1-3 at the collector, the base and the
/// thermal node within 1e-4 of the native collector fundamental.
#[cfg(feature = "veriloga-model-vbic-4t-et-cf")]
#[test]
fn vbic_4t_et_gives_the_native_vbic_harmonics() {
    let resistances = "RCX=10 RCI=10 RBX=10 RBI=10 RE=1 RBP=10 RS=10";
    let stage = |card: &str| {
        format!(
            "* VBIC common emitter\nVCC vcc 0 3\nV1 in 0 SIN(0.8 0.02 1meg)\nRB in b 10k\n\
             RC vcc c 1k\n{card}\n.end\n"
        )
    };
    let run = |deck: String| {
        let netlist = Netlist::parse(&deck).expect("parse");
        engine()
            .run_hb(
                &netlist,
                HbConfig::new(F0)
                    .with_harmonics(HARMONICS)
                    .with_tolerance(1e-9),
            )
            .unwrap_or_else(|error| panic!("HB must run: {error}\n{deck}"))
    };
    for (generated, native, nodes) in [
        (
            format!("Q1 c b 0 0 0 qv\n.model qv VBIC_4T_ET_CF {resistances}"),
            format!("Q1 c b 0 0 qn\n.model qn NPN LEVEL=4 {resistances}"),
            &["c", "b"][..],
        ),
        (
            format!("Q1 c b 0 0 t qv\n.model qv VBIC_4T_ET_CF {resistances} RTH=300 CTH=1n"),
            format!("Q1 c b 0 0 t qn\n.model qn NPN LEVEL=4 {resistances} RTH=300 CTH=1n"),
            &["c", "b", "t"][..],
        ),
    ] {
        let generated = run(stage(&generated));
        let native = run(stage(&native));
        let scale = spectrum(&native, "c")[1].norm();
        for node in nodes {
            let (twin, reference) = (spectrum(&generated, node), spectrum(&native, node));
            for k in 0..=3 {
                assert!(
                    (twin[k] - reference[k]).norm() <= 1e-4 * scale,
                    "V({node}) harmonic {k}: VBIC 4T et {} against native VBIC {}",
                    twin[k],
                    reference[k]
                );
            }
        }
        assert!(
            spectrum(&native, "c")[2].norm() > 1e-3,
            "the stage must make harmonics for the comparison to mean anything"
        );
    }
}

/// VBIC 4T et's default card gives one answer in every analysis.
/// `vbic_4T_et_cf.va` reads its zero default series resistances as open
/// branches, so `cx`, `ci`, `bx`, `bi`, `ei`, `bp` and `si` reach only one
/// another, and the default `RTH = 0` leaves the net on `dt` without an
/// equation. Spectre installs GMIN on such nodes at setup; the operating
/// point, the transient and harmonic balance all hold them there, and the
/// transistor conducts nothing: the collector sits at the supply and the
/// base at the drive. Before, the operating point and the transient failed
/// on the singular system while harmonic balance answered, and shooting,
/// whose steps took no floor, failed on every step's singular matrix.
/// Harmonic balance's own GMIN leaves `c` within 3e-9 V of the supply.
#[cfg(feature = "veriloga-model-vbic-4t-et-cf")]
#[test]
fn vbic_4t_et_default_card_gives_one_answer_in_every_analysis() {
    let deck = "* VBIC 4T et default card\nVCC vcc 0 3\nV1 in 0 SIN(0.8 0.02 1meg)\n\
                RB in b 10k\nRC vcc c 1k\nQ1 c b 0 0 t qv\n.model qv VBIC_4T_ET_CF\n.end\n";
    let netlist = Netlist::parse(deck).expect("parse");
    let op = engine()
        .run_dc_op(&netlist)
        .unwrap_or_else(|error| panic!("the operating point holds the open card: {error}"));
    for (name, expected) in [("c", 3.0), ("b", 0.8), ("t", 0.0)] {
        let value = op.try_voltage_named(name).expect("node voltage");
        assert!((value - expected).abs() <= 1e-9, "V({name}) = {value}");
    }
    let tran = engine()
        .run_tran(&netlist, 2.0 / F0, 1.0 / (256.0 * F0))
        .unwrap_or_else(|error| panic!("the transient holds the open card: {error}"));
    let collector = tran.try_voltage_waveform_named("c").expect("V(c)");
    assert!(collector.iter().all(|&value| (value - 3.0).abs() <= 1e-9));
    let hb = engine()
        .run_hb(
            &netlist,
            HbConfig::new(F0)
                .with_harmonics(HARMONICS)
                .with_tolerance(1e-9),
        )
        .unwrap_or_else(|error| panic!("HB must run: {error}"));
    let c = spectrum(&hb, "c");
    assert!((c[0].re - 3.0).abs() <= 1e-8, "{c:?}");
    assert!(
        c[1..].iter().all(|harmonic| harmonic.norm() <= 1e-9),
        "{c:?}"
    );
    let pss = engine()
        .run_pss(
            &netlist,
            PssConfig::new(F0)
                .with_points_per_period(512)
                .with_tolerance(1e-9),
        )
        .unwrap_or_else(|error| panic!("shooting holds the open card: {error}"))
        .result;
    let output = pss
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("c"))
        .expect("V(c)");
    assert!(
        pss.waveforms[output]
            .values
            .iter()
            .all(|&value| (value - 3.0).abs() <= 1e-9),
        "{:?}",
        &pss.waveforms[output].values[..4]
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

/// Each admitted family's fixture: the family, its circuit and the node read.
fn settled_fixtures() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
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
        // The VBIC 4T et source opens a zero series resistance rather than
        // shorting it, so its default card leaves the terminals unconnected:
        // the card gives the resistances, and a thermal resistance for the
        // thermal node.
        #[cfg(feature = "veriloga-model-vbic-4t-et-cf")]
        (
            "self-heating bipolar transistor (VBIC 4T et)",
            "VCC vcc 0 3\nV1 in 0 SIN(0.8 0.02 1meg)\nRB in b 10k\nRC vcc c 1k\n\
                 Q1 c b 0 0 t qv\n.model qv VBIC_4T_ET_CF RCX=10 RCI=10 RBX=10 RBI=10 RE=1 RBP=10 \
                 RS=10 RTH=300 CTH=1n\n",
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
        #[cfg(feature = "veriloga-model-bsimcmg108")]
        (
            "FinFET (BSIM-CMG 108)",
            "VDD vdd 0 1\nV1 in 0 SIN(0.6 0.1 1meg)\nRG in g 1k\nRD vdd d 1k\n\
                 XM d g 0 0 bsimcmg108\n",
            "d",
        ),
        #[cfg(feature = "veriloga-model-bsimcmg107")]
        (
            "FinFET (BSIM-CMG 107)",
            "VDD vdd 0 1\nV1 in 0 SIN(0.6 0.1 1meg)\nRG in g 1k\nRD vdd d 1k\n\
                 XM d g 0 0 bsimcmg107\n",
            "d",
        ),
        #[cfg(feature = "veriloga-model-l-utsoi-832ce87d")]
        (
            "UTBB FD-SOI MOSFET (L-UTSOI)",
            "VDD vdd 0 1\nV1 in 0 SIN(0.6 0.1 1meg)\nRG in g 1k\nRD vdd d 1k\n\
                 XM d g 0 0 t l_utsoi__832ce87d\n",
            "d",
        ),
        #[cfg(feature = "veriloga-model-l-utsoi-485e0ac9")]
        (
            "UTBB FD-SOI MOSFET, NQS build (L-UTSOI)",
            "VDD vdd 0 1\nV1 in 0 SIN(0.6 0.1 1meg)\nRG in g 1k\nRD vdd d 1k\n\
                 XM d g 0 0 t l_utsoi__485e0ac9\n",
            "d",
        ),
        #[cfg(feature = "veriloga-model-epfl-hemt-10a")]
        (
            "GaN HEMT (EPFL HEMT)",
            "VDD vdd 0 10\nV1 in 0 SIN(-2 0.5 1meg)\nRG in g 1k\nRD vdd d 10\n\
                 XH d g 0 0 t EPFL_HEMT_10a\n",
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
        #[cfg(feature = "veriloga-model-bjt504va")]
        (
            "bipolar transistor (Mextram 504)",
            "VCC vcc 0 3\nV1 in 0 SIN(0.8 0.02 1meg)\nRB in b 10k\nRC vcc c 1k\n\
                 XQ c b 0 0 bjt504va\n",
            "c",
        ),
        #[cfg(feature = "veriloga-model-bjt504tva")]
        (
            "bipolar transistor (Mextram 504, self-heating)",
            "VCC vcc 0 3\nV1 in 0 SIN(0.8 0.02 1meg)\nRB in b 10k\nRC vcc c 1k\n\
                 XQ c b 0 0 t bjt504tva\n",
            "c",
        ),
        #[cfg(feature = "veriloga-model-bjtd504va")]
        (
            "bipolar transistor (Mextram 504, no substrate)",
            "VCC vcc 0 3\nV1 in 0 SIN(0.8 0.02 1meg)\nRB in b 10k\nRC vcc c 1k\n\
                 XQ c b 0 bjtd504va\n",
            "c",
        ),
        #[cfg(feature = "veriloga-model-bjtd504tva")]
        (
            "bipolar transistor (Mextram 504, no substrate, self-heating)",
            "VCC vcc 0 3\nV1 in 0 SIN(0.8 0.02 1meg)\nRB in b 10k\nRC vcc c 1k\n\
                 XQ c b 0 t bjtd504tva\n",
            "c",
        ),
        #[cfg(feature = "veriloga-model-bjt505-va")]
        (
            "bipolar transistor (Mextram 505)",
            "VCC vcc 0 3\nV1 in 0 SIN(0.8 0.02 1meg)\nRB in b 10k\nRC vcc c 1k\n\
                 XQ c b 0 0 bjt505_va\n",
            "c",
        ),
        #[cfg(feature = "veriloga-model-bjt505t-va")]
        (
            "bipolar transistor (Mextram 505, self-heating)",
            "VCC vcc 0 3\nV1 in 0 SIN(0.8 0.02 1meg)\nRB in b 10k\nRC vcc c 1k\n\
                 XQ c b 0 0 t bjt505t_va\n",
            "c",
        ),
        #[cfg(feature = "veriloga-model-bjtd505-va")]
        (
            "bipolar transistor (Mextram 505, no substrate)",
            "VCC vcc 0 3\nV1 in 0 SIN(0.8 0.02 1meg)\nRB in b 10k\nRC vcc c 1k\n\
                 XQ c b 0 bjtd505_va\n",
            "c",
        ),
        #[cfg(feature = "veriloga-model-bjtd505t-va")]
        (
            "bipolar transistor (Mextram 505, no substrate, self-heating)",
            "VCC vcc 0 3\nV1 in 0 SIN(0.8 0.02 1meg)\nRB in b 10k\nRC vcc c 1k\n\
                 XQ c b 0 t bjtd505t_va\n",
            "c",
        ),
        #[cfg(feature = "veriloga-model-hisimhv-n4-va")]
        (
            "high-voltage MOSFET (HiSIM-HV, 4 terminals)",
            "VDD vdd 0 5\nV1 in 0 SIN(1.5 0.2 1meg)\nRG in g 1k\nRD vdd d 1k\n\
                 XM d g 0 0 hisimhv_n4_va\n",
            "d",
        ),
        #[cfg(feature = "veriloga-model-hisimhv-n5-va")]
        (
            "high-voltage MOSFET (HiSIM-HV, 5 terminals)",
            "VDD vdd 0 5\nV1 in 0 SIN(1.5 0.2 1meg)\nRG in g 1k\nRD vdd d 1k\n\
                 XM d g 0 0 0 hisimhv_n5_va\n",
            "d",
        ),
        #[cfg(feature = "veriloga-model-hisimhv-va")]
        (
            "high-voltage MOSFET (HiSIM-HV, self-heating)",
            "VDD vdd 0 5\nV1 in 0 SIN(1.5 0.2 1meg)\nRG in g 1k\nRD vdd d 1k\n\
                 XM d g 0 0 0 t hisimhv_va\n",
            "d",
        ),
        #[cfg(feature = "veriloga-model-asmesd-dio")]
        (
            "ESD diode (ASM-ESD)",
            "V1 in 0 SIN(0.3 0.6 1meg)\nR1 in a 1k\nXD a 0 t asmesd_dio\n",
            "a",
        ),
        #[cfg(feature = "veriloga-model-asmesd")]
        (
            "ESD bipolar clamp (ASM-ESD)",
            "VCC vcc 0 3\nV1 in 0 SIN(0.8 0.02 1meg)\nRB in b 10k\nRC vcc c 1k\n\
                 XQ c b 0 t asmesd\n",
            "c",
        ),
        #[cfg(feature = "veriloga-model-ekv3-rf")]
        (
            "MOSFET (EKV3 RF)",
            "VDD vdd 0 1\nV1 in 0 SIN(0.6 0.1 1meg)\nRG in g 1k\nRD vdd d 1k\n\
                 XM d g 0 0 ekv3_rf\n",
            "d",
        ),
        #[cfg(feature = "veriloga-model-angelov-gan")]
        (
            "GaN FET (Angelov-GaN)",
            "VDD vdd 0 3\nV1 in 0 SIN(-0.5 0.3 1meg)\nRG in g 1k\nRD vdd d 100\n\
                 XA d g 0 t r angelov_gan\n",
            "d",
        ),
        #[cfg(feature = "veriloga-model-asmhemt")]
        (
            "GaN HEMT (ASM-HEMT)",
            "VDD vdd 0 10\nV1 in 0 SIN(-1 0.5 1meg)\nRG in g 1k\nRD vdd d 100\n\
                 XH d g 0 0 t asmhemt\n",
            "d",
        ),
        // MVSG's thermal network takes rth times cth, 2.5 ms by default, to
        // settle, which no 24-period transient reaches. A 1 mK/W heat sink at
        // its thermal node (RT) settles it in a tenth of a period and holds
        // the device within a millikelvin of ambient.
        #[cfg(feature = "veriloga-model-mvsg-cmc")]
        (
            "GaN HEMT (MVSG)",
            "VDD vdd 0 10\nV1 in 0 SIN(-1 0.5 1meg)\nRG in g 1k\nRD vdd d 100\n\
             XH d g 0 0 t mvsg_cmc\nRT t 0 1m\n",
            "d",
        ),
        #[cfg(feature = "veriloga-model-bsimsoi-18c250bc")]
        (
            "SOI MOSFET (BSIM-SOI 4.7)",
            "VDD vdd 0 1\nV1 in 0 SIN(1 0.1 1meg)\nRG in g 1k\nRD vdd d 1k\n\
                 XM d g 0 0 0 b t bsimsoi__18c250bc\n",
            "d",
        ),
        #[cfg(feature = "veriloga-model-vbic13")]
        (
            "bipolar transistor (VBIC 1.3)",
            "VCC vcc 0 3\nV1 in 0 SIN(0.8 0.02 1meg)\nRB in b 10k\nRC vcc c 1k\n\
                 XQ c b 0 vbic13\n",
            "c",
        ),
        #[cfg(feature = "veriloga-model-vbic13-3t-et")]
        (
            "self-heating bipolar transistor (VBIC 1.3 3T et)",
            "VCC vcc 0 3\nV1 in 0 SIN(0.8 0.02 1meg)\nRB in b 10k\nRC vcc c 1k\n\
                 XQ c b 0 t vbic13_3t_et\n",
            "c",
        ),
        #[cfg(feature = "veriloga-model-vbic13-4t")]
        (
            "bipolar transistor with substrate (VBIC 1.3 4T)",
            "VCC vcc 0 3\nV1 in 0 SIN(0.8 0.02 1meg)\nRB in b 10k\nRC vcc c 1k\n\
                 XQ c b 0 0 vbic13_4t\n",
            "c",
        ),
        #[cfg(feature = "veriloga-model-hicuml2va")]
        (
            "bipolar transistor (HICUM/L2)",
            "VCC vcc 0 3\nV1 in 0 SIN(0.8 0.02 1meg)\nRB in b 10k\nRC vcc c 1k\n\
                 XQ c b 0 0 t hicuml2va\n",
            "c",
        ),
        #[cfg(feature = "veriloga-model-juncap200")]
        (
            "junction diode (JUNCAP200)",
            "V1 in 0 SIN(0.3 0.6 1meg)\nR1 in a 1k\nXD a 0 juncap200\n",
            "a",
        ),
        // EKV 2.6's and PSP 102's drain charge barely depends on the drain
        // voltage on these cards, so the drain is an algebraic node a gate
        // charge drives. The trapezoidal rule carries the inconsistent
        // derivative a start from rest (UIC) gives that charge as an
        // undamped point-to-point oscillation of about 20 mV, which the
        // shortened last step before tstop turns into a 1e-2 shift of the
        // last period's spectrum. Gear's rule damps it in a few steps.
        #[cfg(feature = "veriloga-model-ekv-va")]
        (
            "MOSFET (EKV 2.6)",
            "VDD vdd 0 1\nV1 in 0 SIN(0.6 0.1 1meg)\nRG in g 1k\nRD vdd d 1k\n\
                 XM d g 0 0 ekv_va\n.options method=gear\n",
            "d",
        ),
        #[cfg(feature = "veriloga-model-bsimbulk")]
        (
            "MOSFET (BSIM-BULK)",
            "VDD vdd 0 1\nV1 in 0 SIN(0.6 0.1 1meg)\nRG in g 1k\nRD vdd d 1k\n\
                 XM d g 0 0 t bsimbulk\n",
            "d",
        ),
        #[cfg(feature = "veriloga-model-bsimcmg111")]
        (
            "FinFET (BSIM-CMG 111)",
            "VDD vdd 0 1\nV1 in 0 SIN(0.6 0.1 1meg)\nRG in g 1k\nRD vdd d 1k\n\
                 XM d g 0 0 t bsimcmg111\n",
            "d",
        ),
        #[cfg(feature = "veriloga-model-bsimcmg-va")]
        (
            "FinFET (BSIM-CMG 112)",
            "VDD vdd 0 1\nV1 in 0 SIN(0.6 0.1 1meg)\nRG in g 1k\nRD vdd d 1k\n\
                 XM d g 0 0 t bsimcmg_va\n",
            "d",
        ),
        #[cfg(feature = "veriloga-model-hisimsoi-va-242bc21d")]
        (
            "SOI MOSFET (HiSIM-SOI, four terminals)",
            "VDD vdd 0 1\nV1 in 0 SIN(0.6 0.1 1meg)\nRG in g 1k\nRD vdd d 1k\n\
                 XM d g 0 0 hisimsoi_va__242bc21d\n",
            "d",
        ),
        #[cfg(feature = "veriloga-model-hisimsoi-va-38074d06")]
        (
            "SOI MOSFET (HiSIM-SOI, body contact)",
            "VDD vdd 0 1\nV1 in 0 SIN(0.6 0.1 1meg)\nRG in g 1k\nRD vdd d 1k\n\
                 XM d g 0 0 0 hisimsoi_va__38074d06\n",
            "d",
        ),
        #[cfg(feature = "veriloga-model-hisimsoi-va-5be18005")]
        (
            "SOI MOSFET (HiSIM-SOI, body contact and thermal node)",
            "VDD vdd 0 1\nV1 in 0 SIN(0.6 0.1 1meg)\nRG in g 1k\nRD vdd d 1k\n\
                 XM d g 0 0 0 t hisimsoi_va__5be18005\n",
            "d",
        ),
        #[cfg(feature = "veriloga-model-psp102va")]
        (
            "MOSFET (PSP 102)",
            "VDD vdd 0 1\nV1 in 0 SIN(0.6 0.1 1meg)\nRG in g 1k\nRD vdd d 1k\n\
                 XM d g 0 0 psp102va\n.options method=gear\n",
            "d",
        ),
        #[cfg(feature = "veriloga-model-psp102bva")]
        (
            "MOSFET (PSP 102, binned)",
            "VDD vdd 0 1\nV1 in 0 SIN(0.6 0.1 1meg)\nRG in g 1k\nRD vdd d 1k\n\
                 XM d g 0 0 psp102bva\n",
            "d",
        ),
        #[cfg(feature = "veriloga-model-psp102eva")]
        (
            "MOSFET (PSP 102, electrical)",
            "VDD vdd 0 1\nV1 in 0 SIN(0.6 0.1 1meg)\nRG in g 1k\nRD vdd d 1k\n\
                 XM d g 0 0 psp102eva\n",
            "d",
        ),
        // Gear's rule for PSP 103 and 103t, chosen before the first
        // comparison: a start from rest gives the charges an inconsistent
        // derivative, which it damps in a few steps, as for PSP 102 above.
        #[cfg(feature = "veriloga-model-psp103va")]
        (
            "MOSFET (PSP 103)",
            "VDD vdd 0 1\nV1 in 0 SIN(0.6 0.1 1meg)\nRG in g 1k\nRD vdd d 1k\n\
                 XM d g 0 0 psp103va\n.options method=gear\n",
            "d",
        ),
        #[cfg(feature = "veriloga-model-psp103tva")]
        (
            "self-heating MOSFET (PSP 103)",
            "VDD vdd 0 1\nV1 in 0 SIN(0.6 0.1 1meg)\nRG in g 1k\nRD vdd d 1k\n\
                 XM d g 0 0 t psp103tva\n.options method=gear\n",
            "d",
        ),
        #[cfg(feature = "veriloga-model-psp104va")]
        (
            "MOSFET (PSP 104)",
            "VDD vdd 0 1\nV1 in 0 SIN(0.6 0.1 1meg)\nRG in g 1k\nRD vdd d 1k\n\
                 XM d g 0 0 psp104va\n",
            "d",
        ),
        #[cfg(feature = "veriloga-model-psp104tva")]
        (
            "self-heating MOSFET (PSP 104)",
            "VDD vdd 0 1\nV1 in 0 SIN(0.6 0.1 1meg)\nRG in g 1k\nRD vdd d 1k\n\
                 XM d g 0 0 t psp104tva\n",
            "d",
        ),
        // PSP's NQS builds carry each collocation point's channel charge as
        // an `idt` whose initial condition is the quasi-static charge at the
        // present bias; harmonic balance holds each integrand's mean at zero
        // instead, from a DC seed that holds each charge at that condition
        // (SWNQS=3 turns three of them on). Gear's rule, as for PSP 102
        // above.
        #[cfg(feature = "veriloga-model-pspnqs102va")]
        (
            "non-quasi-static MOSFET (PSP 102)",
            "VDD vdd 0 1\nV1 in 0 SIN(0.6 0.1 1meg)\nRG in g 1k\nRD vdd d 1k\n\
                 M1 d g 0 0 nq\n.model nq PSPNQS102VA SWNQS=3\n.options method=gear\n",
            "d",
        ),
        #[cfg(feature = "veriloga-model-pspnqs102bva")]
        (
            "non-quasi-static MOSFET (PSP 102, binned)",
            "VDD vdd 0 1\nV1 in 0 SIN(0.6 0.1 1meg)\nRG in g 1k\nRD vdd d 1k\n\
                 M1 d g 0 0 nq\n.model nq PSPNQS102BVA SWNQS=3\n.options method=gear\n",
            "d",
        ),
        #[cfg(feature = "veriloga-model-pspnqs102eva")]
        (
            "non-quasi-static MOSFET (PSP 102, electrical)",
            "VDD vdd 0 1\nV1 in 0 SIN(0.6 0.1 1meg)\nRG in g 1k\nRD vdd d 1k\n\
                 M1 d g 0 0 nq\n.model nq PSPNQS102EVA SWNQS=3\n.options method=gear\n",
            "d",
        ),
        #[cfg(feature = "veriloga-model-pspnqs103va")]
        (
            "non-quasi-static MOSFET (PSP 103)",
            "VDD vdd 0 1\nV1 in 0 SIN(0.6 0.1 1meg)\nRG in g 1k\nRD vdd d 1k\n\
                 M1 d g 0 0 nq\n.model nq PSPNQS103VA SWNQS=3\n.options method=gear\n",
            "d",
        ),
        #[cfg(feature = "veriloga-model-pspnqs104va")]
        (
            "non-quasi-static MOSFET (PSP 104)",
            "VDD vdd 0 1\nV1 in 0 SIN(0.6 0.1 1meg)\nRG in g 1k\nRD vdd d 1k\n\
                 M1 d g 0 0 nq\n.model nq PSPNQS104VA SWNQS=3\n.options method=gear\n",
            "d",
        ),
    ]
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
    let mut failures = Vec::new();
    for (family, circuit, node) in settled_fixtures() {
        check_fixture(family, circuit, node, &mut failures);
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// Run one fixture through harmonic balance and compare its spectrum with
/// the settled transient's, under the allowance
/// [`generated_models_reach_the_settled_transients_steady_state`] states.
fn check_fixture(family: &str, circuit: &str, node: &str, failures: &mut Vec<String>) {
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
            return;
        }
    };
    compare_with_settled_transient(&netlist, family, node, "harmonic balance", &hb, failures);
}

/// PSTB on a fixture's shooting orbit, read at a probe loop beside it, an RL
/// whose period maps a current deviation by exp(−T·R/L) = exp(−1): PSTB
/// must run where shooting solves, read that multiplier, and find every
/// multiplier inside the unit circle, the fixture's settled transient being
/// stable. A model shooting refuses before solving is its own refusal.
fn check_pstb(family: &str, circuit: &str, failures: &mut Vec<String>) {
    let deck = format!(
        "* {family} periodic stability\n{circuit}VPSTB pstb 0 SIN(0 1 1meg)\n\
         RPSTB pstb qpstb 100\nLPSTB qpstb 0 100u\n.end\n"
    );
    let netlist = Netlist::parse(&deck).expect("parse");
    let point = match engine().run_pss_operating_point_with_abort(
        &netlist,
        PssConfig::new(F0)
            .with_points_per_period(512)
            .with_tolerance(1e-9),
        &rspice_core::NoAbort,
    ) {
        Ok(point) => point,
        Err(rspice_core::SimulationError::UnsupportedCapability(refusal))
            if refusal.detail.ends_with(CARRIED_SITE) || refusal.detail.ends_with(FLUX) =>
        {
            return;
        }
        Err(error) => {
            failures.push(format!(
                "{family}: PSTB's shooting orbit must solve: {error}"
            ));
            return;
        }
    };
    let card = rspice_core::netlist::PstbCard {
        probe_instance: "LPSTB".to_owned(),
        max_harmonics: 4,
        stability_threshold: 1.0 + 1e-6,
        detect_subharmonics: true,
    };
    let multipliers: Vec<Complex64> = match engine().run_pstb_card_from_pss_with_abort(
        &netlist,
        &card,
        &point,
        &rspice_core::NoAbort,
    ) {
        Ok(stability) => stability
            .result
            .multipliers
            .iter()
            .map(|mode| mode.value)
            .collect(),
        Err(error) => {
            failures.push(format!("{family}: PSTB must run: {error}"));
            return;
        }
    };
    let probe = (-1.0_f64).exp();
    if !multipliers
        .iter()
        .any(|value| (value - probe).norm() <= 1.0e-3 * probe)
    {
        failures.push(format!(
            "{family}: PSTB must read the probe loop's exp(-1): {multipliers:?}"
        ));
    }
    if let Some(unstable) = multipliers.iter().find(|value| value.norm() >= 1.0) {
        failures.push(format!(
            "{family}: PSTB finds {unstable} outside the unit circle of a settled orbit: \
             {multipliers:?}"
        ));
    }
}

/// Compare `periodic`, a steady-state spectrum of V(`node`), with the
/// spectrum of RSpice's own transient of `netlist` once it has settled, under
/// the allowance [`generated_models_reach_the_settled_transients_steady_state`]
/// states.
fn compare_with_settled_transient(
    netlist: &Netlist,
    family: &str,
    node: &str,
    method: &str,
    periodic: &[Complex64],
    failures: &mut Vec<String>,
) {
    const PERIODS: f64 = 24.0;
    const SAMPLES: usize = 512;
    let tstop = PERIODS / F0;
    let tran = match engine().run_tran_with_startup_mode_and_abort(
        netlist,
        tstop,
        1.0 / (F0 * SAMPLES as f64),
        TransientStartupMode::Uic,
        &rspice_core::NoAbort,
    ) {
        Ok(tran) => tran,
        Err(error) => {
            failures.push(format!(
                "{family}: transient must run: {error}; {method} V({node}) {:?}",
                &periodic[..4]
            ));
            return;
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
        if (periodic[k] - last[k]).norm() > 1e-3 * fundamental {
            failures.push(format!(
                "{family}: V({node}) harmonic {k}: {method} {} against the settled transient {} \
                 (fundamental {fundamental})",
                periodic[k], last[k]
            ));
        }
    }
    let worst = (0..=3)
        .map(|k| (periodic[k] - last[k]).norm() / fundamental)
        .fold(0.0, f64::max);
    eprintln!(
        "{family}: V({node}) {method} worst {worst:.3e} of the fundamental; {method} {:?} \
         transient {:?}",
        &periodic[..4],
        &last[..4]
    );
}

/// Each admitted family whose transient cannot settle within a fixture's
/// run, with shooting as its oracle. BSIM-SOI e2aff994's floating body
/// charges only as 1/t: after 384 periods its fundamental still moves by
/// 1.4e-4 of itself each period, so no settled transient exists to compare
/// with. Shooting solves the period map's fixed point directly and needs
/// none. The gate swings about 1 V so that the device conducts; about the
/// 0.6 V of the other MOSFET fixtures it is nearly off, and its drain moves
/// by 6 µV. About 0.8 V the body's Floquet multiplier is 0.99996, so the
/// period map's step noise, magnified 3e4 times in the body, stalled
/// shooting short of its tolerance until its steps settled to rounding.
///
/// Allowance, fixed before the first comparison: the standing one, each of
/// the DC term and first three harmonics of the harmonic-balance spectrum
/// within 1e-3 of the fundamental's amplitude of shooting's.
fn shooting_oracle_fixtures() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        #[cfg(feature = "veriloga-model-bsimsoi-e2aff994")]
        (
            "SOI MOSFET (BSIM-SOI e2aff994)",
            "VDD vdd 0 1\nV1 in 0 SIN(1.0 0.1 1meg)\nRG in g 1k\nRD vdd d 1k\n\
                 XM d g 0 0 0 t bsimsoi__e2aff994\n",
            "d",
        ),
        #[cfg(feature = "veriloga-model-bsimsoi-e2aff994")]
        (
            "SOI MOSFET (BSIM-SOI e2aff994, 0.8 V gate)",
            "VDD vdd 0 1\nV1 in 0 SIN(0.8 0.1 1meg)\nRG in g 1k\nRD vdd d 1k\n\
                 XM d g 0 0 0 t bsimsoi__e2aff994\n",
            "d",
        ),
    ]
}

#[test]
fn generated_models_whose_transient_cannot_settle_reach_shootings_orbit() {
    let mut failures = Vec::new();
    for (family, circuit, node) in shooting_oracle_fixtures() {
        check_against_shooting(family, circuit, node, &mut failures);
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// Shooting carries each of PSP's non-quasi-static `idt` charges as a
/// coordinate of its own, harmonic balance as an unknown of its own; both
/// reach one orbit, under the shooting oracle's allowance.
#[test]
fn shooting_and_harmonic_balance_agree_on_a_generated_models_integrals() {
    let mut failures = Vec::new();
    for (family, circuit, node) in settled_fixtures() {
        if family.starts_with("non-quasi-static MOSFET (PSP") {
            check_against_shooting(family, circuit, node, &mut failures);
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// Run one shooting-oracle fixture through harmonic balance and shooting and
/// compare their spectra under the allowance [`shooting_oracle_fixtures`]
/// states.
fn check_against_shooting(family: &str, circuit: &str, node: &str, failures: &mut Vec<String>) {
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
            return;
        }
    };
    let pss = match engine().run_pss(
        &netlist,
        PssConfig::new(F0)
            .with_points_per_period(512)
            .with_tolerance(1e-9),
    ) {
        Ok(result) => result.result,
        Err(error) => {
            failures.push(format!("{family}: shooting must run: {error}"));
            return;
        }
    };
    let output = pss
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("{family}: V({node}) present"));
    let end = *pss.time.last().expect("a periodic waveform");
    let orbit = period_spectrum(&pss.time, &pss.waveforms[output].values, end, 512);
    let fundamental = orbit[1].norm();
    for k in 0..=3 {
        if (hb[k] - orbit[k]).norm() > 1e-3 * fundamental {
            failures.push(format!(
                "{family}: V({node}) harmonic {k}: harmonic balance {} against shooting {} \
                 (fundamental {fundamental})",
                hb[k], orbit[k]
            ));
        }
    }
    let worst = (0..=3)
        .map(|k| (hb[k] - orbit[k]).norm() / fundamental)
        .fold(0.0, f64::max);
    eprintln!(
        "{family}: V({node}) harmonic balance worst {worst:.3e} of shooting's fundamental; \
         harmonic balance {:?} shooting {:?}",
        &hb[..4],
        &orbit[..4]
    );
}

/// Why shooting refuses a generated model whose source branch stores flux.
const FLUX: &str = "a voltage contribution takes the time derivative of its branch current (an \
                    inductance), and the shooting state fixes node voltages, not branch currents";

/// Why periodic AC, periodic transfer functions and periodic noise refuse a
/// generated model with an `idt` carried as an unknown of its own.
#[cfg(feature = "veriloga-model-pspnqs102va")]
const SMALL_SIGNAL_INTEGRAL: &str = "it carries an `idt` as an unknown of its own, and the \
     periodic small-signal sideband solve does not certify with one";

/// Why shooting refuses a generated model that carries both a `ddt` and an
/// `idt` as unknowns of their own.
const CARRIED_SITE: &str = "harmonic balance carries both a `ddt` and an `idt` of it as \
                            unknowns of their own, and a shooting period starts its integrals \
                            from a hold that would not seed the `ddt`'s charge";

/// Shooting reaches the same settled transient for every fixture harmonic
/// balance does, under the same allowance, or refuses before solving a model
/// whose source branch stores flux: the period map integrates each generated
/// model's charges, its `ddt` operands among them, from the state the period
/// starts at, with the nodes its own source branches tie taken as one, and
/// carries each `idt` as a coordinate of its own (PSP's non-quasi-static
/// charges).
#[test]
fn generated_models_reach_the_settled_transients_steady_state_by_shooting() {
    let mut failures = Vec::new();
    for (family, circuit, node) in settled_fixtures() {
        let deck = format!("* {family} periodic steady state\n{circuit}.end\n");
        let netlist = Netlist::parse(&deck).expect("parse");
        let pss = match engine().run_pss(
            &netlist,
            PssConfig::new(F0)
                .with_points_per_period(512)
                .with_tolerance(1e-9),
        ) {
            Ok(result) => result.result,
            // A model with an `idt` site harmonic balance carries as an
            // unknown of its own, or one whose source branch stores flux, is
            // refused before solving, by name and reason (see
            // `shooting_refuses_a_generated_model_by_its_reason`).
            Err(rspice_core::SimulationError::UnsupportedCapability(refusal))
                if refusal.detail.ends_with(CARRIED_SITE) || refusal.detail.ends_with(FLUX) =>
            {
                eprintln!("{family}: shooting refuses it: {}", refusal.detail);
                continue;
            }
            Err(error) => {
                failures.push(format!("{family}: shooting must run: {error}"));
                continue;
            }
        };
        let output = pss
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case(node))
            .unwrap_or_else(|| panic!("{family}: V({node}) present"));
        let end = *pss.time.last().expect("a periodic waveform");
        let spectrum = period_spectrum(&pss.time, &pss.waveforms[output].values, end, 512);
        compare_with_settled_transient(
            &netlist,
            family,
            node,
            "shooting",
            &spectrum,
            &mut failures,
        );
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// Shooting carries a self-heating generated model's temperature as a state
/// of the period: ASM-HEMT's thermal node closes over the orbit. Read as the
/// tie its disabled-self-heating arm (`Temp(dt) <+ 0`) makes, the node was
/// solved afresh at each period start, 0.242 K above ambient against the
/// period end's 0.261 K.
#[cfg(feature = "veriloga-model-asmhemt")]
#[test]
fn shooting_carries_a_self_heating_models_temperature_through_the_period() {
    let deck = "* ASM-HEMT self-heating orbit\nVDD vdd 0 10\nV1 in 0 SIN(-1 0.5 1meg)\n\
                RG in g 1k\nRD vdd d 100\nXH d g 0 0 t asmhemt\n.end\n";
    let netlist = Netlist::parse(deck).expect("parse");
    let pss = engine()
        .run_pss(
            &netlist,
            PssConfig::new(F0)
                .with_points_per_period(512)
                .with_tolerance(1e-9),
        )
        .expect("shooting solves ASM-HEMT")
        .result;
    let node = pss
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("t"))
        .expect("the thermal node");
    let temperature = &pss.waveforms[node].values;
    let (start, end) = (temperature[0], temperature[temperature.len() - 1]);
    assert!(
        end > 0.25 && (end - start).abs() <= 1e-6 * end,
        "the thermal node closes over the period: {start} K at its start, {end} K at its end"
    );
}

/// A generated model whose harmonics neither a settled transient nor shooting
/// has confirmed is refused before solving, by instance and module, with
/// what is missing (BSIM-SOI 4.6.1), as is one whose generator could not
/// prove its residual `F(x) + dQ(x)/dt`, with the generator's reason; the
/// generator proves every shipped model's.
#[test]
fn harmonic_balance_refuses_a_generated_model_by_its_reason() {
    let failures: Vec<String> = refused_models()
        .into_iter()
        .filter_map(|(card, instance, module, reason)| {
            check_refusal(card, instance, module, reason)
        })
        .collect();
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// Every model harmonic balance refuses: a card that instantiates it from
/// the driven node `d`, its instance and module, and the reason its refusal
/// ends with.
fn refused_models() -> Vec<(&'static str, &'static str, &'static str, &'static str)> {
    vec![
        #[cfg(feature = "veriloga-model-bsimsoi-va")]
        (
            "XM d d 0 0 bsimsoi_va",
            "XM",
            "bsimsoi_va",
            "shooting, which needs no settled transient, cannot place the body: its Floquet \
             multiplier is within 1e-8 of one, so the period map's steps, which settle to 1e-11 \
             V, leave it 1e-3 V apart",
        ),
    ]
}

/// Run harmonic balance on `card` driven through a resistor; what is wrong
/// unless it refuses `instance` of `module` before solving, naming the
/// analysis and ending with `reason`.
fn check_refusal(card: &str, instance: &str, module: &str, reason: &str) -> Option<String> {
    use rspice_core::identity::AnalysisKind;
    use rspice_core::{SimulationError, UnsupportedCapabilityError};
    let deck =
        format!("* refused generated model\nV1 in 0 SIN(0 1 1meg)\nR1 in d 1k\n{card}\n.end\n");
    let netlist = Netlist::parse(&deck).expect("parse");
    let error = match engine().run_hb(&netlist, HbConfig::new(F0).with_harmonics(2)) {
        Ok(_) => return Some(format!("{module}: harmonic balance must refuse it")),
        Err(error) => error,
    };
    let SimulationError::UnsupportedCapability(refusal) = &error else {
        return Some(format!(
            "{module}: expected a capability refusal, got {error}"
        ));
    };
    let UnsupportedCapabilityError {
        capability,
        analysis_kind,
        instance: refused_instance,
        module: refused_module,
        detail,
        ..
    } = refusal.as_ref();
    let named = *capability == "analysis.hb.veriloga_operator"
        && *analysis_kind == Some(AnalysisKind::HarmonicBalance)
        && refused_instance.as_deref() == Some(instance)
        && refused_module
            .as_deref()
            .is_some_and(|name| name.eq_ignore_ascii_case(module))
        && detail.contains("harmonic-balance analysis")
        && detail.ends_with(reason);
    (!named).then(|| {
        format!(
            "{module}: the refusal must name the analysis, instance, module and reason: {error}"
        )
    })
}

/// Every shipped model this build compiles, walked by name: an admitted one
/// must reach its fixture's settled transient, or shooting's orbit where no
/// transient settles, PSTB must read a fixture's shooting orbit (see
/// [`check_pstb`]), a refused one must give its
/// named reason, and one with neither a fixture nor a named refusal fails,
/// so a model the generator newly admits cannot go unchecked. A fixture or
/// refusal names its model as a word of its deck. It needs every model
/// feature, which only the nightly builds, so ordinary runs skip it.
#[test]
#[ignore = "nightly: needs every model feature"]
fn every_shipped_model_reaches_its_fixture_or_names_its_refusal() {
    use rspice_core::device::veriloga_builtins::builtins;
    let names_model = |deck: &str, name: &str| {
        deck.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
            .any(|word| word.eq_ignore_ascii_case(name))
    };
    let fixtures = settled_fixtures();
    let oracle_fixtures = shooting_oracle_fixtures();
    let refused = refused_models();
    let mut failures = Vec::new();
    for &name in builtins::builtin_names() {
        if let Some(&(card, instance, module, reason)) =
            refused.iter().find(|(card, ..)| names_model(card, name))
        {
            failures.extend(check_refusal(card, instance, module, reason));
        } else if let Some(&(family, circuit, node)) = fixtures
            .iter()
            .find(|(_, circuit, _)| names_model(circuit, name))
        {
            check_fixture(family, circuit, node, &mut failures);
            check_pstb(family, circuit, &mut failures);
        } else if let Some(&(family, circuit, node)) = oracle_fixtures
            .iter()
            .find(|(_, circuit, _)| names_model(circuit, name))
        {
            check_against_shooting(family, circuit, node, &mut failures);
        } else {
            failures.push(format!(
                "{name}: harmonic balance admits it, but it has no settled-transient fixture"
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// Shooting refuses before solving, by instance and module, a generated
/// model with an `idt` site harmonic balance carries as an unknown of its own
/// (PSP 102's non-quasi-static charges) and one whose source branch stores
/// flux at its parameters, each with its reason and shooting's own token.
#[test]
fn shooting_refuses_a_generated_model_by_its_reason() {
    use rspice_core::identity::AnalysisKind;
    use rspice_core::{SimulationError, UnsupportedCapabilityError};
    let cases: Vec<(&str, &str, &str, &str)> = vec![
        // A 1 mH drain inductance, whose current the period map would miss:
        // admitted, shooting stops short of the settled transient.
        #[cfg(feature = "veriloga-model-angelov")]
        (
            "MA d d 0 am\n.model am angelov ld=1m",
            "MA",
            "angelov",
            FLUX,
        ),
    ];
    for (card, instance, module, reason) in cases {
        let deck =
            format!("* refused generated model\nV1 in 0 SIN(0 1 1meg)\nR1 in d 1k\n{card}\n.end\n");
        let netlist = Netlist::parse(&deck).expect("parse");
        let error = engine()
            .run_pss(&netlist, PssConfig::new(F0))
            .map(drop)
            .expect_err("shooting must refuse the model");
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
        assert_eq!(*capability, "analysis.pss.veriloga_operator", "{error}");
        assert_eq!(*analysis_kind, Some(AnalysisKind::Pss), "{error}");
        assert_eq!(refused_instance.as_deref(), Some(instance), "{error}");
        assert!(
            refused_module
                .as_deref()
                .is_some_and(|name| name.eq_ignore_ascii_case(module)),
            "{error}"
        );
        assert!(
            detail.contains("PSS analysis") && detail.ends_with(reason),
            "{module}: the refusal must name the analysis and its reason: {detail}"
        );
    }
}

/// Periodic AC from the small-signal source `vrf` into sidebands -2..=2 of
/// `V(a)` at three offsets, and the periodic transfer function from it into
/// the upper first sideband, on the deck's harmonic-balance carrier: once
/// solved by periodic AC itself and once retained from `.hb`.
#[cfg(any(feature = "veriloga-model-r2-cmc", feature = "veriloga-model-bjt504va"))]
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

/// Periodic noise of `V(a)` at output sidebands -2..=2 and two offsets on the
/// deck's harmonic-balance carrier: each sideband's total at each offset,
/// then the part the sources whose labels name `device` contribute.
#[cfg(all(feature = "veriloga-model-r2-cmc", feature = "veriloga-builtins-noise"))]
fn pnoise_by_sideband(deck: &str, device: &str) -> Vec<(Vec<f64>, Vec<f64>)> {
    use rspice_core::NoAbort;
    use rspice_core::engine::{PeriodicNoiseRequest, PeriodicNoiseSidebands};
    let netlist = Netlist::parse(deck).expect("parse");
    let hb = run_hb(deck);
    let offsets = [0.13 * F0, 0.37 * F0];
    (-2..=2)
        .map(|output| {
            let result = engine()
                .run_pnoise_from_hb_request_with_abort(
                    &netlist,
                    &PeriodicNoiseRequest {
                        sampling: None,
                        offsets: &offsets,
                        output_node: "a",
                        output_ref: None,
                        input_source: None,
                        max_sideband: 4,
                        sidebands: PeriodicNoiseSidebands { input: 0, output },
                    },
                    &hb.operating_point,
                    &NoAbort,
                )
                .unwrap_or_else(|error| panic!("pnoise must run: {error}\n{deck}"));
            let mut part = vec![0.0; offsets.len()];
            for (label, values) in &result.contributors {
                let label = label.to_ascii_lowercase();
                if label
                    .strip_prefix(device)
                    .is_some_and(|rest| rest.is_empty() || rest.starts_with([' ', ':']))
                {
                    for (sum, value) in part.iter_mut().zip(values) {
                        *sum += value;
                    }
                }
            }
            (result.output_noise, part)
        })
        .collect()
}

/// Periodic noise samples a generated model's noise processes over the orbit
/// and folds them as the native twin's, so the generated R2_CMC in a pumped
/// mixer gives each output sideband's noise, and its own part of it, that
/// the native resistor gives. Allowance: 2e-6 of the largest native total at
/// that sideband: R2_CMC's thermal noise uses its NIST 2004 Boltzmann
/// constant, 1.38065050e-23, which is 1.09e-6 above the circuit's CODATA
/// 2018 one.
#[cfg(all(feature = "veriloga-model-r2-cmc", feature = "veriloga-builtins-noise"))]
#[test]
fn a_generated_resistors_noise_folds_as_its_native_twins() {
    let native = pnoise_by_sideband(&pumped_mixer("R3 a 0 2k"), "r3");
    let twin = pnoise_by_sideband(&pumped_mixer("XR3 a 0 r2_cmc r=2k"), "xr3");
    for (sideband, ((native_total, native_part), (twin_total, twin_part))) in
        (-2..=2).zip(native.iter().zip(&twin))
    {
        let scale = native_total.iter().copied().fold(0.0, f64::max);
        for (index, (native_value, twin_value)) in native_total
            .iter()
            .chain(native_part)
            .zip(twin_total.iter().chain(twin_part))
            .enumerate()
        {
            assert!(
                (twin_value - native_value).abs() <= 2.0e-6 * scale,
                "sideband {sideband} value {index}: generated {twin_value:e} against native \
                 {native_value:e}"
            );
        }
        assert!(
            native_part.iter().all(|&value| value > 1.0e-3 * scale),
            "R3 must contribute at sideband {sideband}: {native_part:?} of {native_total:?}"
        );
    }
}

/// Periodic AC from `vrf` into sidebands -2..=2 of `V(a)` at three offsets,
/// and the periodic transfer function into the upper first sideband, on the
/// deck's shooting carrier.
#[cfg(any(feature = "veriloga-model-r2-cmc", feature = "veriloga-model-bjt504va"))]
fn shooting_carrier_transfers(deck: &str) -> Vec<Complex64> {
    use rspice_core::NoAbort;
    use rspice_core::analysis::pac::{PacConfig, PacSweepType};
    use rspice_core::netlist::{FreqVariation, PeriodicSourceSelector, PeriodicSweep, PxfCard};
    let (start, stop) = (0.13 * F0, 0.37 * F0);
    let netlist = Netlist::parse(deck).expect("parse");
    let point = engine()
        .run_pss_operating_point_with_abort(
            &netlist,
            PssConfig::new(F0)
                .with_harmonics(HARMONICS)
                .with_points_per_period(256)
                .with_tolerance(1e-9),
            &NoAbort,
        )
        .unwrap_or_else(|error| panic!("shooting must run: {error}\n{deck}"));
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
    let pac = engine()
        .run_pac_from_pss_with_abort(&netlist, config, &point, &NoAbort)
        .unwrap_or_else(|error| panic!("PAC on the shooting carrier must run: {error}\n{deck}"));
    let pxf = engine()
        .run_pxf_card_from_pss_with_abort(&netlist, &card, &point, &NoAbort)
        .unwrap_or_else(|error| panic!("PXF on the shooting carrier must run: {error}\n{deck}"));
    let mut transfers = Vec::new();
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
    transfers.extend(pxf.points.iter().map(|point| point.transfer));
    assert_eq!(transfers.len(), 3 * 5 + 3);
    transfers
}

/// A shooting carrier now carries a generated model's charges, and periodic
/// AC and periodic transfer functions linearize its projected orbit through
/// harmonic balance's own samples, so the generated R2_CMC in a pumped mixer
/// gives every sideband transfer its native twin gives on that carrier.
/// Allowance: 1e-6 of the largest native transfer, as on the HB carrier.
#[cfg(feature = "veriloga-model-r2-cmc")]
#[test]
fn a_generated_resistor_mixes_its_native_twins_sidebands_on_a_shooting_carrier() {
    let native = shooting_carrier_transfers(&pumped_mixer("R3 a 0 2k"));
    let twin = shooting_carrier_transfers(&pumped_mixer("XR3 a 0 r2_cmc r=2k"));
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

/// Periodic AC, periodic transfer functions and periodic noise refuse PSP
/// 102's non-quasi-static integrals by name on a shooting orbit that carries
/// them, as on harmonic balance's carrier, each with the label of the
/// analysis it runs (a periodic transfer function runs as periodic AC).
#[cfg(feature = "veriloga-model-pspnqs102va")]
#[test]
fn periodic_small_signal_refuses_a_generated_models_integrals_by_name() {
    use rspice_core::NoAbort;
    use rspice_core::analysis::pac::{PacConfig, PacSweepType};
    use rspice_core::engine::{PeriodicNoiseRequest, PeriodicNoiseSidebands};
    use rspice_core::netlist::{FreqVariation, PeriodicSourceSelector, PeriodicSweep, PxfCard};
    use rspice_core::{SimulationError, UnsupportedCapabilityError};
    let deck = "* pumped non-quasi-static stage\nVDD vdd 0 1\nV1 in 0 SIN(0.6 0.1 1meg)\n\
                RG in g 1k\nVRF rf 0 DC 0\nR2 rf g 10k\nRD vdd a 1k\nM1 a g 0 0 nq\n\
                .model nq PSPNQS102VA SWNQS=3\n.options method=gear\n.end\n";
    let netlist = Netlist::parse(deck).expect("parse");
    let point = engine()
        .run_pss_operating_point_with_abort(&netlist, PssConfig::new(F0), &NoAbort)
        .unwrap_or_else(|error| panic!("shooting must carry the integrals: {error}"));
    let pac = PacConfig::new()
        .with_fundamental(F0)
        .with_sweep(0.13 * F0, 0.37 * F0, 3)
        .with_sweep_type(PacSweepType::Linear)
        .with_sidebands(-2, 2)
        .with_input_source("vrf")
        .with_output_node("a");
    let card = PxfCard {
        sweep: PeriodicSweep {
            variation: FreqVariation::Lin,
            points: 3,
            start_freq: 0.13 * F0,
            stop_freq: 0.37 * F0,
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
    let refusals = [
        (
            "PAC",
            "PAC analysis",
            engine()
                .run_pac_from_pss_with_abort(&netlist, pac, &point, &NoAbort)
                .map(drop),
        ),
        (
            "PXF",
            "PAC analysis",
            engine()
                .run_pxf_card_from_pss_with_abort(&netlist, &card, &point, &NoAbort)
                .map(drop),
        ),
        (
            "pnoise",
            "pnoise analysis",
            engine()
                .run_pnoise_from_pss_request_with_abort(
                    &netlist,
                    &PeriodicNoiseRequest {
                        sampling: None,
                        offsets: &[0.13 * F0],
                        output_node: "a",
                        output_ref: None,
                        input_source: None,
                        max_sideband: 2,
                        sidebands: PeriodicNoiseSidebands {
                            input: 0,
                            output: 0,
                        },
                    },
                    &point,
                    &NoAbort,
                )
                .map(drop),
        ),
    ];
    for (analysis, label, outcome) in refusals {
        let error = outcome.expect_err("the integrals must be refused");
        let SimulationError::UnsupportedCapability(refusal) = &error else {
            panic!("{analysis}: expected a capability refusal, got {error}");
        };
        let UnsupportedCapabilityError {
            instance, detail, ..
        } = refusal.as_ref();
        assert_eq!(instance.as_deref(), Some("M1"), "{error}");
        assert!(
            detail.starts_with(label) && detail.ends_with(SMALL_SIGNAL_INTEGRAL),
            "{analysis}: the refusal must name the analysis and its reason: {detail}"
        );
    }
}

/// Periodic AC and periodic transfer functions on a shooting orbit read a
/// generated model's carried `ddt` (Mextram 504's) off the model over the
/// orbit, and give in a pumped common-emitter stage every sideband transfer
/// they give on harmonic balance's carrier. Allowance: 1e-4 of the largest
/// transfer.
#[cfg(feature = "veriloga-model-bjt504va")]
#[test]
fn periodic_small_signal_on_a_shooting_orbit_reads_a_generated_models_carried_derivative() {
    let deck = "* pumped common-emitter stage\nVCC vcc 0 3\nV1 in 0 SIN(0.8 0.02 1meg)\n\
                RB in b 10k\nVRF rf 0 DC 0\nR2 rf b 10k\nRC vcc a 1k\nXQ a b 0 0 bjt504va\n.end\n";
    let harmonic_balance = hb_carrier_transfers(deck);
    let shooting = shooting_carrier_transfers(deck);
    let retained = &harmonic_balance[15..];
    let scale = retained
        .iter()
        .map(|value| value.norm())
        .fold(0.0, f64::max);
    let worst = shooting
        .iter()
        .zip(retained)
        .map(|(shot, balanced)| (shot - balanced).norm())
        .fold(0.0, f64::max);
    eprintln!(
        "Mextram 504: shooting carrier worst {:.3e} of the largest transfer",
        worst / scale
    );
    for (index, (shot, balanced)) in shooting.iter().zip(retained).enumerate() {
        assert!(
            (shot - balanced).norm() <= 1e-4 * scale,
            "transfer {index}: shooting carrier {shot} against harmonic balance's {balanced}"
        );
    }
}

/// The PSTB multipliers of `deck`'s orbit read at inductor `probe`, largest
/// first.
#[cfg(any(
    feature = "veriloga-model-r2-cmc",
    feature = "veriloga-model-diode-cmc"
))]
fn pstb_multipliers(deck: &str, config: PssConfig, probe: &str) -> Vec<Complex64> {
    let netlist = Netlist::parse(deck).expect("parse");
    let point = engine()
        .run_pss_operating_point_with_abort(&netlist, config, &rspice_core::NoAbort)
        .unwrap_or_else(|error| panic!("PSS must solve: {error}\n{deck}"));
    let card = rspice_core::netlist::PstbCard {
        probe_instance: probe.to_owned(),
        max_harmonics: 4,
        stability_threshold: 1.0 + 1e-6,
        detect_subharmonics: true,
    };
    let stability = engine()
        .run_pstb_card_from_pss_with_abort(&netlist, &card, &point, &rspice_core::NoAbort)
        .unwrap_or_else(|error| panic!("PSTB must run: {error}\n{deck}"));
    let mut multipliers: Vec<Complex64> = stability
        .result
        .multipliers
        .iter()
        .map(|mode| mode.value)
        .collect();
    multipliers.sort_by(|a, b| b.norm().total_cmp(&a.norm()));
    assert!(
        !point
            .shooting_state_basis()
            .first()
            .is_some_and(|name| name.starts_with("event:")),
        "native shooting solves {deck}"
    );
    multipliers
}

/// PSTB reads a generated compact model's charges where shooting carries
/// them. The generated R2_CMC is the native resistor of its value, so an RL
/// through either maps a current deviation over the period by
/// exp(−T·R/L) = exp(−1) at R = 100 Ω, L = 100 µH.
#[cfg(feature = "veriloga-model-r2-cmc")]
#[test]
fn pstb_reads_a_generated_resistors_rl_as_its_native_twins() {
    let deck = |resistor: &str| {
        format!("* pstb of an RL\nV1 in 0 SIN(0 1 1meg)\n{resistor}\nL1 a 0 100u\n.end\n")
    };
    let config = || {
        PssConfig::new(F0)
            .with_points_per_period(512)
            .with_tolerance(1e-9)
    };
    let native = pstb_multipliers(&deck("R1 in a 100"), config(), "L1");
    let generated = pstb_multipliers(&deck("XR1 in a r2_cmc r=100"), config(), "L1");
    let expected = (-1.0_f64).exp();
    eprintln!("RL: R2_CMC {generated:?}, native {native:?}, exp(-1) = {expected}");
    assert_eq!(generated.len(), 1, "{generated:?}");
    assert!(
        (generated[0].re / expected - 1.0).abs() <= 1.0e-4,
        "{generated:?}"
    );
    for (a, b) in generated.iter().zip(&native) {
        assert!((a - b).norm() <= 1.0e-9, "R2_CMC {a} against native {b}");
    }
}

/// PSTB reads the generated DIODE_CMC's junction charge where native
/// shooting carries it. Held in reverse bias (V(b) between −1.5 and −0.5 V)
/// behind R1 = 10 kΩ, the junction is its depletion capacitance
/// `CJORBOT·AB·(1 − V/VBIRBOT)^(−PBOT)`, 1 nF at zero bias with a 1 mm²
/// bottom area, no edges and the reference temperature the circuit's; its
/// currents there are below a nanoampere against the milliamperes that
/// charge it. The native twin is the SPICE diode with that capacitance law,
/// CJO = 1 nF, VJ = 1 V, M = 0.5, and the same saturation current, whose
/// multiplier exp(∮∂f/∂v dt) over the RC's orbit is 0.868124; a probe loop
/// beside them adds the RL's exp(−1). DIODE_CMC's junction reads 0.869087
/// there, 1.1e-3 above its twin: its charge function is not the SPICE law to
/// a part in a thousand. The spectra agree to 2e-3.
#[cfg(feature = "veriloga-model-diode-cmc")]
#[test]
fn pstb_reads_a_generated_diodes_junction_as_its_native_twins() {
    let deck = |junction: &str| {
        format!(
            "* pstb of a reverse-biased junction\nV1 in 0 SIN(-1 0.5 1meg)\nR1 in b 10k\n\
             {junction}\nV2 p 0 SIN(0 1 1meg)\nR2 p q 100\nL2 q 0 100u\n.end\n"
        )
    };
    let config = || {
        PssConfig::new(F0)
            .with_points_per_period(512)
            .with_tolerance(1e-9)
    };
    let native = pstb_multipliers(
        &deck("D1 b 0 dj\n.model dj d is=1e-18 cjo=1n vj=1 m=0.5"),
        config(),
        "L2",
    );
    let generated = pstb_multipliers(
        &deck("D9 b 0 dcm ab=1e-6 ls=0 lg=0\n.model dcm diode_cmc trj=27"),
        config(),
        "L2",
    );
    eprintln!("junction: DIODE_CMC {generated:?}, native {native:?}");
    assert_eq!(generated.len(), 2, "{generated:?}");
    assert_eq!(native.len(), 2, "{native:?}");
    assert!(
        (generated[1].re / (-1.0_f64).exp() - 1.0).abs() <= 1.0e-4,
        "{generated:?}"
    );
    for (a, b) in generated.iter().zip(&native) {
        assert!(
            (a - b).norm() <= 2.0e-3 * b.norm(),
            "DIODE_CMC {a} against native {b}"
        );
    }
}
