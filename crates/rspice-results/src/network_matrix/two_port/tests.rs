//! Oracles: worked examples published with their S-parameters and results,
//! checked to the digits they print, and the defining property of every
//! circle checked on the circle itself.

use super::*;
use crate::analysis_result::AnalysisResult;
use crate::family_metadata::AnalysisResultFamilyMetadata;

fn polar(magnitude: f64, degrees: f64) -> Complex64 {
    Complex64::from_polar(magnitude, degrees.to_radians())
}

fn degrees(value: Complex64) -> f64 {
    value.arg().to_degrees()
}

/// `value` rounds to `printed` at the printed number of decimals.
fn prints_as(what: &str, value: f64, printed: f64, decimals: i32) {
    let half_unit = 0.5 * 10f64.powi(-decimals);
    assert!(
        (value - printed).abs() <= half_unit,
        "{what}: {value} does not print as {printed} at {decimals} decimals"
    );
}

/// 2SC5226A at VCE = 5 V, IC = 7 mA, 100 MHz, Z0 = 50 Ω (onsemi datasheet),
/// as worked in S. Arar, "Learn About Unconditional Stability and Potential
/// Instability in RF Amplifier Design", All About Circuits.
fn arar_100_mhz() -> TwoPortScattering {
    TwoPortScattering {
        s11: polar(0.720, -46.0),
        s12: polar(0.030, 68.5),
        s21: polar(17.973, 148.5),
        s22: polar(0.880, -23.6),
    }
}

/// The same part at 1.4 GHz as worked in Arar, "Using the Operating Power
/// Gain to Design a Bilateral RF Amplifier" and "Using the Available Power
/// Gain to Design Bilateral Low-Noise Amplifiers", All About Circuits.
fn arar_1400_mhz() -> TwoPortScattering {
    TwoPortScattering {
        s11: polar(0.533, 176.6),
        s12: polar(0.06, 58.4),
        s21: polar(2.800, 64.5),
        s22: polar(0.604, -58.3),
    }
}

fn decibels(value: f64) -> f64 {
    10f64.powf(value / 10.0)
}

#[test]
fn the_stability_circles_match_the_published_example() {
    let device = arar_100_mhz();
    // Printed |Δ| = 0.71 and K = 0.19. |Δ| is 0.70496, which the article
    // rounds up; it is held to one unit of the printed digit.
    assert!((device.determinant().norm() - 0.71).abs() <= 0.01);
    prints_as("K", device.rollett_k(), 0.19, 2);

    // Printed cS = 25.37 ∠ 102.76°, rS = 25.17, stable outside.
    let source = device.source_stability_circle().expect("source circle");
    prints_as("|cS|", source.circle.center.norm(), 25.37, 2);
    prints_as("∠cS", degrees(source.circle.center), 102.76, 2);
    prints_as("rS", source.circle.radius, 25.17, 2);
    assert!(
        !source.stable_inside,
        "the article reads the outside as stable"
    );

    // Printed cL = 2.35 ∠ 58.43°, rL = 1.94; the chart centre is stable.
    let load = device.load_stability_circle().expect("load circle");
    prints_as("|cL|", load.circle.center.norm(), 2.35, 2);
    prints_as("∠cL", degrees(load.circle.center), 58.43, 2);
    prints_as("rL", load.circle.radius, 1.94, 2);
    assert!(load.is_stable(Complex64::new(0.0, 0.0)));
    assert!(!device.is_unconditionally_stable());
}

#[test]
fn the_power_gain_circle_matches_the_published_example() {
    let device = arar_1400_mhz();
    // Printed K = 1.12, Δ = 0.16 ∠ 113.32°, C2 = 0.52 ∠ −57.51°.
    prints_as("K", device.rollett_k(), 1.12, 2);
    prints_as("|Δ|", device.determinant().norm(), 0.16, 2);
    prints_as("∠Δ", degrees(device.determinant()), 113.32, 2);
    let c2 = device.s22 - device.determinant() * device.s11.conj();
    prints_as("|C2|", c2.norm(), 0.52, 2);
    prints_as("∠C2", degrees(c2), -57.51, 2);

    // GP = 12.59 (11 dB): printed gP = 1.61, CP = 0.54 ∠ 57.51°, rP = 0.44.
    let gain = 12.59;
    prints_as("gP", gain / device.s21.norm_sqr(), 1.61, 2);
    let circle = device.power_gain_circle(gain).expect("power-gain circle");
    prints_as("|CP|", circle.center.norm(), 0.54, 2);
    prints_as("∠CP", degrees(circle.center), 57.51, 2);
    prints_as("rP", circle.radius, 0.44, 2);
}

#[test]
fn the_available_gain_circle_matches_the_published_example() {
    let device = arar_1400_mhz();
    // Printed C1 = 0.44 ∠ 177.66°.
    let c1 = device.s11 - device.determinant() * device.s22.conj();
    prints_as("|C1|", c1.norm(), 0.44, 2);
    prints_as("∠C1", degrees(c1), 177.66, 2);

    // GA = 13 dB: printed gA = 2.55, CA = 0.67 ∠ −177.66°, rA = 0.29. The
    // exact gA is 2.54498, which the article rounds up; it is held to one
    // unit of the printed digit.
    let gain = decibels(13.0);
    assert!((gain / device.s21.norm_sqr() - 2.55).abs() <= 0.01);
    let circle = device
        .available_gain_circle(gain)
        .expect("available-gain circle");
    prints_as("|CA|", circle.center.norm(), 0.67, 2);
    prints_as("∠CA", degrees(circle.center), -177.66, 2);
    prints_as("rA", circle.radius, 0.29, 2);
}

#[test]
fn the_maximum_gain_is_the_available_gain_of_a_stable_device() {
    let device = arar_1400_mhz();
    assert!(device.is_unconditionally_stable());
    let Some(MaximumGain::Available(gain)) = device.maximum_gain() else {
        panic!("an unconditionally stable device has an available maximum");
    };
    // The article prints GT,max = 28.73 (14.58 dB) from its rounded K = 1.12;
    // the same formula with that K reproduces it, and the exact K gives
    // 28.93.
    let rounded = 2.800 / 0.06 * (1.12 - (1.12f64 * 1.12 - 1.0).sqrt());
    prints_as("GT,max at K = 1.12", rounded, 28.73, 2);
    let k = device.rollett_k();
    let exact = device.s21.norm() / device.s12.norm() * (k - (k * k - 1.0).sqrt());
    assert!((gain - exact).abs() <= 1.0e-12 * exact);
    prints_as("MAG", gain, 28.93, 2);
    // The available gain at the simultaneous conjugate match is the maximum:
    // no source on a dense grid of the chart beats it.
    let mut best = 0.0f64;
    for ring in 0..200 {
        for spoke in 0..360 {
            let source =
                Complex64::from_polar(f64::from(ring) / 200.0, f64::from(spoke).to_radians());
            best = best.max(device.available_gain(source));
        }
    }
    assert!(best <= gain * (1.0 + 1.0e-12) && best >= gain * 0.999);

    let Some(MaximumGain::Stable(stable)) = arar_100_mhz().maximum_gain() else {
        panic!("a potentially unstable device has a stable maximum");
    };
    assert!((stable / (17.973 / 0.030) - 1.0).abs() < 1.0e-12);
}

#[test]
fn the_noise_circle_matches_the_published_example() {
    // Fmin = 1.6 dB, Γopt = 0.62 ∠ 100°, RN = 20 Ω, Z0 = 50 Ω; the 3 dB
    // circle with F taken as 2: printed N = 0.4, CF = 0.44 ∠ 100°, rF = 0.46.
    let noise = TwoPortNoiseParameters {
        minimum_noise_factor: decibels(1.6),
        noise_resistance_ohm: 20.0,
        optimum_source_reflection: polar(0.62, 100.0),
        reference_impedance_ohm: 50.0,
    };
    let circle = noise.noise_circle(2.0).expect("noise circle");
    let n = (2.0 - noise.minimum_noise_factor) / (4.0 * 20.0 / 50.0)
        * (1.0 + noise.optimum_source_reflection).norm_sqr();
    prints_as("N", n, 0.4, 1);
    prints_as("|CF|", circle.center.norm(), 0.44, 2);
    prints_as("∠CF", degrees(circle.center), 100.0, 2);
    prints_as("rF", circle.radius, 0.46, 2);
    // At the minimum the circle is the optimum point itself; below it there
    // is none.
    let at_minimum = noise
        .noise_circle(noise.minimum_noise_factor)
        .expect("degenerate circle");
    assert_eq!(at_minimum.radius, 0.0);
    assert!((at_minimum.center - noise.optimum_source_reflection).norm() < 1.0e-15);
    assert!(noise.noise_circle(1.2).is_none());
}

/// Points around a circle.
fn around(circle: SmithCircle) -> impl Iterator<Item = Complex64> {
    (0..72).map(move |step| {
        circle.center + Complex64::from_polar(circle.radius, f64::from(step) * 5f64.to_radians())
    })
}

#[test]
fn every_circle_is_the_locus_its_definition_names() {
    for device in [arar_100_mhz(), arar_1400_mhz()] {
        let source = device.source_stability_circle().unwrap();
        let load = device.load_stability_circle().unwrap();
        for gamma in around(source.circle) {
            assert!((device.output_reflection(gamma).norm() - 1.0).abs() < 1.0e-9);
        }
        for gamma in around(load.circle) {
            assert!((device.input_reflection(gamma).norm() - 1.0).abs() < 1.0e-9);
        }
        // The stated stable side is the side where the opposite reflection
        // is below one, tested a little inside and outside the boundary.
        let output = |gamma: Complex64| device.output_reflection(gamma);
        let input = |gamma: Complex64| device.input_reflection(gamma);
        let sides: [(StabilityCircle, &dyn Fn(Complex64) -> Complex64); 2] =
            [(source, &output), (load, &input)];
        for (circle, reflection) in sides {
            let c = circle.circle;
            for (scale, inside) in [(0.97, true), (1.03, false)] {
                let gamma = c.center + Complex64::new(c.radius * scale, 0.0);
                assert_eq!(
                    reflection(gamma).norm() < 1.0,
                    inside == circle.stable_inside,
                    "stable side at {gamma}"
                );
            }
        }
        for decibel in [6.0, 9.0, 12.0] {
            let gain = decibels(decibel);
            if let Some(circle) = device.power_gain_circle(gain) {
                for gamma in around(circle) {
                    assert!((device.power_gain(gamma) / gain - 1.0).abs() < 1.0e-9);
                }
            }
            if let Some(circle) = device.available_gain_circle(gain) {
                for gamma in around(circle) {
                    assert!((device.available_gain(gamma) / gain - 1.0).abs() < 1.0e-9);
                }
            }
        }
    }
    let noise = TwoPortNoiseParameters {
        minimum_noise_factor: 1.4,
        noise_resistance_ohm: 12.0,
        optimum_source_reflection: polar(0.5, 140.0),
        reference_impedance_ohm: 50.0,
    };
    for factor in [1.5, 2.0, 3.0] {
        let circle = noise.noise_circle(factor).unwrap();
        for gamma in around(circle) {
            assert!((noise.noise_factor(gamma) / factor - 1.0).abs() < 1.0e-9);
        }
    }
}

#[test]
fn mu_is_the_distance_to_the_nearest_unstable_termination() {
    // Edwards and Sinsky define μ geometrically: the distance from the chart
    // centre to the nearest load (μ) or source (μ′) for which the opposite
    // port's reflection reaches one. Search for it directly.
    for device in [arar_100_mhz(), arar_1400_mhz()] {
        let input = |gamma: Complex64| device.input_reflection(gamma);
        let output = |gamma: Complex64| device.output_reflection(gamma);
        let planes: [(f64, &dyn Fn(Complex64) -> Complex64); 2] =
            [(device.mu_load(), &input), (device.mu_source(), &output)];
        for (mu, reflection) in planes {
            let mut nearest = f64::INFINITY;
            for spoke in 0..3600 {
                let angle = f64::from(spoke) * 0.1f64.to_radians();
                // The first radius along this spoke where the reflection
                // reaches one, bisected.
                let unstable =
                    |radius: f64| reflection(Complex64::from_polar(radius, angle)).norm() >= 1.0;
                let mut outer = 0.0;
                while outer < 3.0 && !unstable(outer) {
                    outer += 0.01;
                }
                if outer >= 3.0 {
                    continue;
                }
                let mut inner = (outer - 0.01f64).max(0.0);
                for _ in 0..60 {
                    let middle = 0.5 * (inner + outer);
                    if unstable(middle) {
                        outer = middle;
                    } else {
                        inner = middle;
                    }
                }
                nearest = nearest.min(outer);
            }
            assert!(
                (nearest - mu).abs() < 1.0e-4,
                "μ = {mu}, searched {nearest}"
            );
        }
    }
}

#[test]
fn a_unilateral_device_has_no_reverse_terms_to_divide_by() {
    let device = TwoPortScattering {
        s11: polar(0.75, -120.0),
        s12: Complex64::new(0.0, 0.0),
        s21: polar(2.5, 80.0),
        s22: polar(0.6, -70.0),
    };
    assert!(device.rollett_k().is_infinite());
    assert!(device.is_unconditionally_stable());
    let Some(MaximumGain::Available(gain)) = device.maximum_gain() else {
        panic!("a stable unilateral device has an available maximum");
    };
    let expected = 2.5f64.powi(2) / ((1.0 - 0.75f64.powi(2)) * (1.0 - 0.6f64.powi(2)));
    assert!((gain / expected - 1.0).abs() < 1.0e-12);
    // A gain above the maximum has no circle.
    assert!(device.power_gain_circle(gain * 1.01).is_none());
    assert!(device.power_gain_circle(gain * 0.5).is_some());
}

fn complex_waveform(
    label: String,
    name: &str,
    frequencies: &[f64],
    values: &[Complex64],
) -> RetainedWaveform {
    RetainedWaveform::new(
        label,
        frequencies.to_vec(),
        values.iter().map(|value| value.norm()).collect::<Vec<_>>(),
    )
    .with_complex_components(
        name,
        values.iter().map(|value| value.re).collect::<Vec<_>>(),
        values.iter().map(|value| value.im).collect::<Vec<_>>(),
    )
}

/// Retained two-port noise columns: `Rn` (Ω), `Fmin` (linear) and the
/// complex `Sopt`, one entry per frequency, at a reference temperature.
struct RetainedNoise {
    temperature_kelvin: f64,
    resistance_ohm: Vec<f64>,
    minimum_factor: Vec<f64>,
    optimum: Vec<Complex64>,
}

/// A two-port sweep in the shape the runner retains it: complex `S11` to
/// `S22` columns and, with noise, `Rn`, `Fmin` and complex `Sopt`.
fn retained_sweep(
    frequencies: &[f64],
    points: &[TwoPortScattering],
    references_ohm: [f64; 2],
    noise: Option<RetainedNoise>,
) -> AnalysisResult {
    let mut waveforms = Vec::new();
    for (name, pick) in [("S11", 0usize), ("S12", 1), ("S21", 2), ("S22", 3)] {
        let values = points
            .iter()
            .map(|point| [point.s11, point.s12, point.s21, point.s22][pick])
            .collect::<Vec<_>>();
        waveforms.push(complex_waveform(
            format!("|{name}|"),
            name,
            frequencies,
            &values,
        ));
    }
    let temperature = noise.as_ref().map(|noise| noise.temperature_kelvin);
    if let Some(noise) = noise {
        waveforms.push(RetainedWaveform::new(
            "Rn",
            frequencies.to_vec(),
            noise.resistance_ohm,
        ));
        waveforms.push(RetainedWaveform::new(
            "Fmin",
            frequencies.to_vec(),
            noise.minimum_factor,
        ));
        waveforms.push(complex_waveform(
            "Sopt".to_owned(),
            "Sopt",
            frequencies,
            &noise.optimum,
        ));
    }
    AnalysisResult::new(1, AnalysisType::SParameter, "sp", 0.0)
        .with_family_metadata(AnalysisResultFamilyMetadata::SParameter {
            reference_impedances_ohm: references_ohm.to_vec(),
            noise_reference_temperature_kelvin: temperature,
        })
        .with_waveforms(waveforms)
}

fn sweep_analysis(with_noise: bool) -> AnalysisResult {
    let noise = with_noise.then(|| RetainedNoise {
        temperature_kelvin: 290.0,
        resistance_ohm: vec![20.0, 18.0],
        minimum_factor: vec![1.3, 1.45],
        optimum: vec![Complex64::new(0.0, 0.5), Complex64::new(-0.1077, 0.6106)],
    });
    retained_sweep(
        &[1.0e8, 1.4e9],
        &[arar_100_mhz(), arar_1400_mhz()],
        [50.0, 75.0],
        noise,
    )
}

#[test]
fn a_retained_two_port_sweep_resolves_with_its_noise_parameters() {
    let sweep = TwoPortSweep::resolve(&sweep_analysis(true)).expect("two-port sweep");
    assert_eq!(sweep.frequencies, vec![1.0e8, 1.4e9]);
    assert_eq!(sweep.reference_impedances_ohm, [50.0, 75.0]);
    let expected = [arar_100_mhz(), arar_1400_mhz()];
    for (point, expected) in sweep.scattering.iter().zip(expected) {
        for (actual, wanted) in [
            (point.s11, expected.s11),
            (point.s12, expected.s12),
            (point.s21, expected.s21),
            (point.s22, expected.s22),
        ] {
            assert!((actual - wanted).norm() < 1.0e-12);
        }
    }
    let noise = sweep.noise.as_ref().expect("retained noise parameters");
    assert_eq!(noise[1].noise_resistance_ohm, 18.0);
    assert_eq!(noise[1].minimum_noise_factor, 1.45);
    assert_eq!(noise[1].reference_impedance_ohm, 50.0);
    assert_eq!(sweep.nearest_index(1.0e9), Some(1));

    let quiet = TwoPortSweep::resolve(&sweep_analysis(false)).expect("two-port sweep");
    assert!(quiet.noise.is_none(), "no noise parameters are invented");

    let mut one_port = sweep_analysis(false);
    one_port.family_metadata = Some(AnalysisResultFamilyMetadata::SParameter {
        reference_impedances_ohm: vec![50.0],
        noise_reference_temperature_kelvin: None,
    });
    assert!(TwoPortSweep::resolve(&one_port).is_none());

    let mut partial = sweep_analysis(false);
    partial.waveforms.remove(1);
    assert!(
        TwoPortSweep::resolve(&partial).is_none(),
        "a matrix without S12 cannot place a circle"
    );
}

/// A common-emitter 2N3904 stage (the public Fairchild Gummel-Poon card),
/// biased through chokes and coupled through capacitors to two 50 Ω ports.
#[cfg(feature = "engine-evidence")]
const COMMON_EMITTER_STAGE: &str = "Common-emitter 2N3904 stage\n\
     VCC vcc 0 DC 9\n\
     RBIAS vcc b 430k\n\
     LC vcc c 1m\n\
     CIN p1 b 100n\n\
     COUT c p2 100n\n\
     Q1 c b 0 Q2N3904\n\
     V1 p1 0 DC 0 AC 1 portnum=1 z0=50\n\
     V2 p2 0 DC 0 AC 0 portnum=2 z0=50\n\
     .model Q2N3904 NPN(IS=6.734f XTI=3 EG=1.11 VAF=74.03 BF=416.4 NE=1.259\n\
     + ISE=6.734f IKF=66.78m XTB=1.5 BR=.7371 NC=2 ISC=0 IKR=0 RC=1 CJC=3.638p\n\
     + MJC=.3085 VJC=.75 FC=.5 CJE=4.493p MJE=.2593 VJE=.75 TR=239.5n\n\
     + TF=301.2p ITF=.4 VTF=4 XTF=2 RB=10)\n\
     .end\n";

/// Relative agreement to `tolerance`, with an absolute floor for values that
/// are zero in exact arithmetic.
#[cfg(feature = "engine-evidence")]
fn agrees(what: &str, actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() <= tolerance * expected.abs().max(1.0e-12),
        "{what}: {actual} against {expected}"
    );
}

#[cfg(feature = "engine-evidence")]
fn agrees_complex(what: &str, actual: Complex64, expected: Complex64, tolerance: f64) {
    assert!(
        (actual - expected).norm() <= tolerance * expected.norm().max(1.0e-12),
        "{what}: {actual} against {expected}"
    );
}

/// The circles a simulated transistor stage gets on the Smith pane are the
/// engine's own stability analysis of the same S-matrix, and the textbook
/// formulas (Gonzalez sections 3.3, 3.6 and 4.3) evaluated here from the raw
/// coefficients. The noise circle is checked against the engine's noise
/// factor at the port's own 50 Ω, which the engine derives separately from
/// the correlation matrix.
#[cfg(feature = "engine-evidence")]
#[test]
fn a_simulated_transistor_stage_gets_the_engines_circles() {
    use rspice_core::analysis::s_param::{GainAnalysis, StabilityAnalysis};
    use rspice_core::{Engine, Netlist, NoAbort};

    let netlist = Netlist::parse(COMMON_EMITTER_STAGE).expect("deck parses");
    let frequencies = [1.0e6, 1.0e7, 3.0e7, 1.0e8, 3.0e8, 1.0e9];
    let run = Engine::default()
        .run_sp_over_grid_with_abort(&netlist, &frequencies, true, &NoAbort)
        .expect(".SP runs");
    let engine = &run.scattering.data;
    let noise = run.port_noise.as_ref().expect("noise was requested");
    let engine_noise = noise
        .two_port
        .as_ref()
        .expect("a two-port has noise parameters");
    let points = engine
        .iter()
        .map(|point| TwoPortScattering {
            s11: point.s11(),
            s12: point.s12(),
            s21: point.s21(),
            s22: point.s22(),
        })
        .collect::<Vec<_>>();
    let references = [run.scattering.ports[0].z0, run.scattering.ports[1].z0];
    let analysis = retained_sweep(
        &frequencies,
        &points,
        references,
        Some(RetainedNoise {
            temperature_kelvin: noise.reference_temperature_kelvin,
            resistance_ohm: engine_noise.iter().map(|p| p.noise_resistance).collect(),
            minimum_factor: engine_noise
                .iter()
                .map(|p| p.minimum_noise_factor)
                .collect(),
            optimum: engine_noise
                .iter()
                .map(|p| p.optimum_source_reflection)
                .collect(),
        }),
    );
    let sweep = TwoPortSweep::resolve(&analysis).expect("the run resolves as a two-port");
    assert_eq!(sweep.reference_impedances_ohm, [50.0, 50.0]);
    let sweep_noise = sweep.noise.as_ref().expect("noise parameters are attached");

    let tolerance = 1.0e-9;
    let mut potentially_unstable = 0;
    for (index, device) in sweep.scattering.iter().enumerate() {
        let f = frequencies[index];
        let core = StabilityAnalysis::from_s_matrix(&engine[index]);
        agrees(
            &format!("K at {f}"),
            device.rollett_k(),
            core.k_factor,
            tolerance,
        );
        agrees(
            &format!("mu at {f}"),
            device.mu_load(),
            core.mu_factor,
            tolerance,
        );
        agrees(
            &format!("mu' at {f}"),
            device.mu_source(),
            core.mu_prime,
            tolerance,
        );
        assert_eq!(
            device.is_unconditionally_stable(),
            core.unconditionally_stable,
            "stability verdict at {f}"
        );
        let source = device.source_stability_circle().expect("source circle");
        let load = device.load_stability_circle().expect("load circle");
        agrees_complex(
            &format!("source centre at {f}"),
            source.circle.center,
            core.input_stability_center,
            tolerance,
        );
        agrees(
            &format!("source radius at {f}"),
            source.circle.radius,
            core.input_stability_radius,
            tolerance,
        );
        agrees_complex(
            &format!("load centre at {f}"),
            load.circle.center,
            core.output_stability_center,
            tolerance,
        );
        agrees(
            &format!("load radius at {f}"),
            load.circle.radius,
            core.output_stability_radius,
            tolerance,
        );
        assert_eq!(
            source.stable_inside, core.input_stable_inside,
            "source side at {f}"
        );
        assert_eq!(
            load.stable_inside, core.output_stable_inside,
            "load side at {f}"
        );

        // The same figures from the textbook formulas, evaluated here.
        let (s11, s12, s21, s22) = (device.s11, device.s12, device.s21, device.s22);
        let delta = s11 * s22 - s12 * s21;
        let loop_gain = (s12 * s21).norm();
        let k = (1.0 - s11.norm_sqr() - s22.norm_sqr() + delta.norm_sqr()) / (2.0 * loop_gain);
        agrees(
            &format!("textbook K at {f}"),
            device.rollett_k(),
            k,
            tolerance,
        );
        let source_denominator = s11.norm_sqr() - delta.norm_sqr();
        let load_denominator = s22.norm_sqr() - delta.norm_sqr();
        agrees_complex(
            &format!("textbook source centre at {f}"),
            source.circle.center,
            (s11 - delta * s22.conj()).conj() / source_denominator,
            tolerance,
        );
        agrees(
            &format!("textbook source radius at {f}"),
            source.circle.radius,
            loop_gain / source_denominator.abs(),
            tolerance,
        );
        agrees_complex(
            &format!("textbook load centre at {f}"),
            load.circle.center,
            (s22 - delta * s11.conj()).conj() / load_denominator,
            tolerance,
        );
        agrees(
            &format!("textbook load radius at {f}"),
            load.circle.radius,
            loop_gain / load_denominator.abs(),
            tolerance,
        );

        // Gain circles 3 dB below the device's maximum gain.
        let gains = GainAnalysis::from_s_matrix(&engine[index]);
        let maximum = device.maximum_gain().expect("a maximum gain");
        match maximum {
            MaximumGain::Available(gain) => {
                agrees(
                    &format!("MAG at {f}"),
                    10.0 * gain.log10(),
                    gains.mag_db,
                    tolerance,
                );
            }
            MaximumGain::Stable(gain) => {
                potentially_unstable += 1;
                agrees(
                    &format!("MSG at {f}"),
                    10.0 * gain.log10(),
                    gains.msg_db,
                    tolerance,
                );
            }
        }
        let target = 0.5 * maximum.linear();
        let gp = target / s21.norm_sqr();
        let c2 = s22 - delta * s11.conj();
        let power_scale = 1.0 + gp * load_denominator;
        let power = device.power_gain_circle(target).expect("power-gain circle");
        agrees_complex(
            &format!("textbook power-gain centre at {f}"),
            power.center,
            gp * c2.conj() / power_scale,
            tolerance,
        );
        agrees(
            &format!("textbook power-gain radius at {f}"),
            power.radius,
            (1.0 - 2.0 * k * loop_gain * gp + (loop_gain * gp).powi(2)).sqrt() / power_scale.abs(),
            tolerance,
        );
        let ga = target / s21.norm_sqr();
        let c1 = s11 - delta * s22.conj();
        let available_scale = 1.0 + ga * source_denominator;
        let available = device
            .available_gain_circle(target)
            .expect("available-gain circle");
        agrees_complex(
            &format!("textbook available-gain centre at {f}"),
            available.center,
            ga * c1.conj() / available_scale,
            tolerance,
        );
        agrees(
            &format!("textbook available-gain radius at {f}"),
            available.radius,
            (1.0 - 2.0 * k * loop_gain * ga + (loop_gain * ga).powi(2)).sqrt()
                / available_scale.abs(),
            tolerance,
        );

        // Noise: the factor at Γs = 0 is the engine's separately derived F,
        // and the circle is the textbook one.
        let parameters = sweep_noise[index];
        agrees(
            &format!("F at 50 Ω at {f}"),
            parameters.noise_factor(Complex64::new(0.0, 0.0)),
            engine_noise[index].noise_factor,
            1.0e-6,
        );
        let factor = 1.25 * parameters.minimum_noise_factor;
        let optimum = parameters.optimum_source_reflection;
        let n = (factor - parameters.minimum_noise_factor)
            / (4.0 * parameters.noise_resistance_ohm / references[0])
            * (1.0 + optimum).norm_sqr();
        let circle = parameters.noise_circle(factor).expect("noise circle");
        agrees_complex(
            &format!("textbook noise centre at {f}"),
            circle.center,
            optimum / (1.0 + n),
            tolerance,
        );
        agrees(
            &format!("textbook noise radius at {f}"),
            circle.radius,
            (n * n + n * (1.0 - optimum.norm_sqr())).sqrt() / (1.0 + n),
            tolerance,
        );
    }
    assert!(
        potentially_unstable > 0,
        "the stage is potentially unstable somewhere"
    );
    assert!(
        potentially_unstable < frequencies.len(),
        "and stable somewhere"
    );
}
