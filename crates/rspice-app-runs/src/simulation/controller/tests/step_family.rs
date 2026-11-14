//! A `.STEP` of a swept analysis runs through the controller as one retained
//! analysis per point, each keeping its whole sweep: the family LTspice and
//! QSPICE draw. The oracle values are LTspice 17.2.4's, read from its `.raw`
//! and `.log` for the same decks.
use crate::state::{AnalysisResult, AnalysisType, SimulationRun};
use rspice_results::provenance::AnalysisResultFamilyMember;

fn signal<'a>(analysis: &'a AnalysisResult, name: &str) -> &'a crate::state::RetainedTrace {
    analysis
        .waveforms
        .iter()
        .find(|waveform| waveform.name.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| {
            panic!(
                "{name} in {}: {:?}",
                analysis.label,
                analysis
                    .waveforms
                    .iter()
                    .map(|waveform| &waveform.name)
                    .collect::<Vec<_>>()
            )
        })
}

/// The signal's sample at the sweep value `x`, which must be a grid point.
fn at(analysis: &AnalysisResult, name: &str, x: f64) -> f64 {
    let waveform = signal(analysis, name);
    let index = waveform
        .x
        .iter()
        .position(|value| (value - x).abs() <= 1e-9 * x.abs().max(1.0))
        .unwrap_or_else(|| panic!("{x} on the sweep of {}", analysis.label));
    waveform.y[index]
}

fn coordinate(analysis: &AnalysisResult) -> Vec<(String, f64)> {
    match analysis
        .provenance()
        .and_then(|provenance| provenance.family_member())
    {
        Some(AnalysisResultFamilyMember::StepPoint { coordinate, .. }) => coordinate
            .iter()
            .map(|axis| (axis.name.clone(), axis.value))
            .collect(),
        other => panic!("{} names no .STEP point: {other:?}", analysis.label),
    }
}

fn measurement(analysis: &AnalysisResult, name: &str) -> f64 {
    analysis
        .measurements
        .iter()
        .find(|measurement| measurement.name.eq_ignore_ascii_case(name))
        .and_then(|measurement| measurement.value)
        .unwrap_or_else(|| panic!("{name} in {}: {:?}", analysis.label, analysis.measurements))
}

fn members(run: &SimulationRun, kind: AnalysisType, count: usize) -> Vec<&AnalysisResult> {
    let labels: Vec<_> = run
        .analyses
        .iter()
        .map(|analysis| &analysis.label)
        .collect();
    assert_eq!(run.analyses.len(), count, "{labels:?}");
    assert!(
        run.analyses
            .iter()
            .all(|analysis| analysis.success && analysis.analysis_type == kind),
        "{labels:?}"
    );
    run.analyses.iter().collect()
}

const NMOS_FAMILY: &str = "nmos family\n\
    .param vgs=1\n\
    VG g 0 {vgs}\n\
    VD d 0 0\n\
    M1 d g 0 0 NM W=10u L=1u\n\
    .model NM NMOS (LEVEL=1 VTO=0.7 KP=100u LAMBDA=0.02)\n\
    .step param vgs list 1.5 2 2.5\n\
    .dc VD 0 3 0.05\n\
    .meas DC idsat FIND I(VD) AT 3\n\
    .end\n";

/// The level-1 square law with channel-length modulation.
fn square_law(vgs: f64, vds: f64) -> f64 {
    let (kp, w_over_l, vto, lambda) = (100e-6, 10.0, 0.7, 0.02);
    let overdrive = vgs - vto;
    let channel = if vds >= overdrive {
        overdrive * overdrive / 2.0
    } else {
        overdrive * vds - vds * vds / 2.0
    };
    kp * w_over_l * channel * (1.0 + lambda * vds)
}

#[test]
fn a_stepped_dc_sweep_keeps_one_id_vds_curve_per_gate_voltage() {
    let run = super::super::test_execution::run_manual_batch(NMOS_FAMILY);
    let curves = members(&run, AnalysisType::DcSweep, 3);
    // LTspice: I(Vd) at Vd = 1 V and 3 V, and the log's idsat per step.
    let ltspice = [
        (1.5, -3.264_000_020_1e-4, -3.392_000_060_1e-4, -0.0003392),
        (2.0, -8.160_000_020_1e-4, -8.957_000_060_1e-4, -0.0008957),
        (2.5, -1.326_000_002_01e-3, -1.717_200_006_01e-3, -0.0017172),
    ];
    for (curve, (vgs, at_1v, at_3v, idsat)) in curves.iter().zip(ltspice) {
        assert_eq!(coordinate(curve), [("vgs".to_owned(), vgs)]);
        let drain = signal(curve, "I(VD)");
        assert_eq!(drain.x.len(), 61, "{}: the whole sweep", curve.label);
        for (vds, id) in drain.x.iter().zip(drain.y.iter()) {
            // gmin across the drain junction is the only departure.
            assert!(
                (-id - square_law(vgs, *vds)).abs() < 1e-11,
                "{} at Vd = {vds}: {id:e}",
                curve.label
            );
        }
        for (vds, expected) in [(1.0, at_1v), (3.0, at_3v)] {
            let id = at(curve, "I(VD)", vds);
            assert!(
                (id - expected).abs() < 1e-15,
                "{} at Vd = {vds}: {id:e}, LTspice {expected:e}",
                curve.label
            );
        }
        let measured = measurement(curve, "idsat");
        assert!(
            (measured - idsat).abs() < 5e-8,
            "{}: idsat {measured:e}, LTspice {idsat:e}",
            curve.label
        );
    }
    assert_eq!(curves[1].label, "DC Sweep · vgs = 2");
}

const RC_FAMILY: &str = "rc low-pass family\n\
    .param c=1n\n\
    V1 in 0 AC 1\n\
    R1 in out 1k\n\
    C1 out 0 {c}\n\
    .step param c list 1n 10n 100n\n\
    .ac dec 20 1 1meg\n\
    .meas AC f3db WHEN mag(V(out))=0.7071067811865476\n\
    .end\n";

/// Where |V(out)| crosses 1/√2, interpolated in log frequency between the
/// grid points either side.
fn minus_3db(analysis: &AnalysisResult) -> f64 {
    let magnitude = signal(analysis, "|V(out)|");
    let threshold = std::f64::consts::FRAC_1_SQRT_2;
    let at = magnitude
        .y
        .windows(2)
        .position(|pair| pair[0] >= threshold && pair[1] < threshold)
        .expect("the response crosses -3 dB");
    let (f0, f1) = (magnitude.x[at].log10(), magnitude.x[at + 1].log10());
    let (m0, m1) = (magnitude.y[at], magnitude.y[at + 1]);
    10f64.powf(f0 + (m0 - threshold) / (m0 - m1) * (f1 - f0))
}

#[test]
fn a_stepped_ac_sweep_keeps_one_bode_curve_per_capacitor() {
    let run = super::super::test_execution::run_manual_batch(RC_FAMILY);
    let curves = members(&run, AnalysisType::Ac, 3);
    // LTspice: the same log-frequency interpolation of its own curves, and
    // its log's f3db per step.
    let ltspice = [
        (1e-9, 159_138.556_427_825_1, 159_176.0),
        (1e-8, 15_913.855_642_782_51, 15_917.6),
        (1e-7, 1_591.385_564_278_251, 1_591.76),
    ];
    for (curve, (c, interpolated, logged)) in curves.iter().zip(ltspice) {
        assert_eq!(coordinate(curve), [("c".to_owned(), c)]);
        assert_eq!(signal(curve, "|V(out)|").x.len(), 121, "{}", curve.label);
        let corner = 1.0 / (2.0 * std::f64::consts::PI * 1e3 * c);
        let crossing = minus_3db(curve);
        // Twenty points a decade put the grid's interpolation 1e-4 off the
        // closed form, as LTspice's own curve is.
        assert!(
            (crossing / corner - 1.0).abs() < 2e-4,
            "{}: {crossing} against 1/(2πRC) = {corner}",
            curve.label
        );
        assert!(
            (crossing / interpolated - 1.0).abs() < 1e-9,
            "{}: {crossing} against LTspice {interpolated}",
            curve.label
        );
        let measured = measurement(curve, "f3db");
        assert!(
            (measured / logged - 1.0).abs() < 5e-6,
            "{}: f3db {measured} against LTspice {logged}",
            curve.label
        );
    }
}

const DIODE_TEMPERATURES: &str = "diode temperature family\n\
    V1 in 0 0\n\
    R1 in a 100\n\
    D1 a 0 DMOD\n\
    .model DMOD D(IS=1e-14 N=1.05 RS=0.5)\n\
    .step temp list -40 27 125\n\
    .dc V1 0 2 0.02\n\
    .meas DC va FIND V(a) AT 1\n\
    .end\n";

/// V(a) of the diode deck at V1 = 1 V and `celsius`, solved in closed form:
/// the SPICE diode's saturation current at temperature (EG = 1.11 eV,
/// XTI = 3) with the CODATA 2018 Boltzmann constant and charge, and gmin
/// across the junction.
fn diode_anode(celsius: f64) -> f64 {
    let (is, n, rs, r, gmin) = (1e-14, 1.05, 0.5, 100.0, 1e-12);
    let kelvin = celsius + 273.15;
    let ratio = kelvin / 300.15;
    let vt = 1.380_649e-23 * kelvin / 1.602_176_634e-19;
    let saturation = is * ratio.powf(3.0 / n) * ((ratio - 1.0) * 1.11 / (n * vt)).exp();
    let (mut low, mut high) = (0.0, 1.0 / r);
    for _ in 0..200 {
        let current = 0.5 * (low + high);
        let junction = n * vt * (current / saturation).ln_1p();
        if (1.0 - junction - current * rs) / r - current - gmin * junction > 0.0 {
            low = current;
        } else {
            high = current;
        }
    }
    let current = 0.5 * (low + high);
    n * vt * (current / saturation).ln_1p() + current * rs
}

#[test]
fn a_temperature_step_keeps_one_forward_curve_per_temperature() {
    let run = super::super::test_execution::run_manual_batch(DIODE_TEMPERATURES);
    let curves = members(&run, AnalysisType::DcSweep, 3);
    // LTspice: V(a) at V1 = 1 V, and its log's va per step. LTspice's
    // Boltzmann constant and charge are the older CODATA ones, whose thermal
    // voltage is 29 nV above this engine's at 27 °C; across the junction's
    // 27 thermal voltages that is the 0.8 to 1.6 µV its curves sit above.
    let ltspice = [
        (-40.0, 0.811_629_162_008_294_6, 0.811629),
        (27.0, 0.717_492_288_293_681_4, 0.717492),
        (125.0, 0.575_199_958_883_672_7, 0.5752),
    ];
    for (curve, (temp, anode, logged)) in curves.iter().zip(ltspice) {
        assert_eq!(coordinate(curve), [("temp".to_owned(), temp)]);
        assert_eq!(signal(curve, "V(a)").x.len(), 101, "{}", curve.label);
        let va = at(curve, "V(a)", 1.0);
        let closed_form = diode_anode(temp);
        assert!(
            (va - closed_form).abs() < 5e-7,
            "{}: V(a) {va} against the closed form {closed_form}",
            curve.label
        );
        assert!(
            (va - anode).abs() < 3e-6,
            "{}: V(a) {va} against LTspice {anode}",
            curve.label
        );
        let measured = measurement(curve, "va");
        assert!(
            (measured - logged).abs() < 3e-6,
            "{}: va {measured} against LTspice {logged}",
            curve.label
        );
    }
}

/// A temperature point is a PVT point as well as a member of the family: the
/// run's reference process, no supply axis, its own temperature, and nominal
/// only at the run's reference temperature (27 °C), as a temperature step's
/// point is. The family stays as it is drawn, one curve named per temperature.
#[test]
fn a_temperature_point_is_nominal_only_at_the_reference_temperature() {
    let run = super::super::test_execution::run_manual_batch(DIODE_TEMPERATURES);
    let curves = members(&run, AnalysisType::DcSweep, 3);
    for (curve, (temp, nominal)) in
        curves
            .iter()
            .zip([(-40.0, false), (27.0, true), (125.0, false)])
    {
        assert_eq!(coordinate(curve), [("temp".to_owned(), temp)]);
        let point = curve
            .provenance()
            .and_then(|provenance| provenance.pvt_point())
            .unwrap_or_else(|| panic!("{} is attributed to no PVT point", curve.label));
        assert_eq!(point.temperature_celsius(), temp, "{}", curve.label);
        assert_eq!(point.is_nominal(), nominal, "{}", curve.label);
        assert_eq!(point.process(), "TT", "{}", curve.label);
        assert_eq!(point.supply_voltage(), None, "{}", curve.label);
        assert_eq!(point.corner_contract(), None, "{}", curve.label);
    }

    // Off the reference temperature, no point is the nominal one.
    let run = super::super::test_execution::run_manual_batch(
        &DIODE_TEMPERATURES.replace("list -40 27 125", "list 0 50"),
    );
    let curves = members(&run, AnalysisType::DcSweep, 2);
    assert!(curves.iter().all(|curve| {
        curve
            .provenance()
            .and_then(|provenance| provenance.pvt_point())
            .is_some_and(|point| !point.is_nominal())
    }));
}

#[test]
fn nested_steps_are_the_cross_product_of_their_points() {
    let deck = NMOS_FAMILY
        .replace("W=10u", "W={w}")
        .replace(".param vgs=1", ".param vgs=1 w=10u")
        .replace(
            ".step param vgs list 1.5 2 2.5",
            ".step param vgs list 1.5 2.5\n.step param w list 10u 20u",
        );
    let run = super::super::test_execution::run_manual_batch(&deck);
    let curves = members(&run, AnalysisType::DcSweep, 4);
    // The first card varies fastest, as LTspice orders its steps.
    for (curve, (vgs, w)) in
        curves
            .iter()
            .zip([(1.5, 10e-6), (2.5, 10e-6), (1.5, 20e-6), (2.5, 20e-6)])
    {
        assert_eq!(
            coordinate(curve),
            [("vgs".to_owned(), vgs), ("w".to_owned(), w)]
        );
        let scale = w / 10e-6;
        let id = -at(curve, "I(VD)", 3.0);
        assert!(
            (id - scale * square_law(vgs, 3.0)).abs() < 1e-11,
            "{}: {id:e}",
            curve.label
        );
    }
}

#[test]
fn each_temperature_point_solves_the_circuit_its_conditionals_select() {
    let deck = "conditional family\n\
        VDD vdd 0 DC 1.8\n\
        R1 vdd out 1k\n\
        R2 out 0 1k\n\
        .if (TEMPER < 50)\n\
        R3 vdd extra 1k\n\
        R4 extra 0 1k\n\
        .endif\n\
        .step temp list 27 85\n\
        .dc VDD 0 1.8 0.9\n\
        .end\n";
    let run = super::super::test_execution::run_manual_batch(deck);
    let curves = members(&run, AnalysisType::DcSweep, 2);
    let names = |curve: &AnalysisResult| {
        curve
            .waveforms
            .iter()
            .map(|waveform| waveform.name.to_ascii_lowercase())
            .collect::<Vec<_>>()
    };
    assert!(
        names(curves[0]).contains(&"v(extra)".to_owned()),
        "{:?}",
        names(curves[0])
    );
    assert!(
        !names(curves[1]).contains(&"v(extra)".to_owned()),
        "{:?}",
        names(curves[1])
    );
    assert!((at(curves[1], "V(out)", 1.8) - 0.9).abs() < 1e-9);
}
