//! Oracle tests for transient device noise.
//!
//! Every assertion here is analytic: Johnson-Nyquist power in a band,
//! equipartition on a capacitor, the device model's own `KF*I^AF/f^EF` law,
//! and the square-root bias dependence of shot noise. None of them is a
//! recorded waveform, so none of them can agree with a defect.

use super::*;
use crate::abort_signal::NoAbort;
use crate::analysis::transient::TransientResult;
use crate::netlist::Netlist;

/// The analysis temperature every fixture here runs at.
const T: Value = crate::constants::TEMP_REFERENCE;

/// Boltzmann's constant as the native dialect's noise physics uses it.
fn boltzmann() -> Value {
    crate::analysis::noise::NoisePhysicalConstants::MODERN.boltzmann
}

fn engine() -> Engine {
    Engine::default()
}

/// Run one deck as a transient with the sample grid as the step ceiling.
fn run(deck: &str, tstop: Value, max_step: Value) -> TransientResult {
    let netlist = Netlist::parse(deck).expect("the transient-noise deck parses");
    engine()
        .run_tran(&netlist, tstop, max_step)
        .expect("the transient-noise run converges")
}

/// The exact time average of `w^2` over `(0, tstop]`.
///
/// Weighting by each accepted interval is what makes this the integral
/// the oracle is stated as, rather than an average over however the
/// controller happened to distribute its points.
fn time_averaged_square(result: &TransientResult, waveform: &[Value]) -> Value {
    let mut weighted = 0.0;
    let mut span = 0.0;
    for index in 1..waveform.len() {
        let dt = result.step_sizes[index];
        weighted += waveform[index] * waveform[index] * dt;
        span += dt;
    }
    assert!(span > 0.0, "the run recorded no accepted interval");
    weighted / span
}

/// Build a circuit, solve its operating point, and install the injection
/// plan, returning everything the caller needs to inspect one train.
fn installed(
    deck: &str,
    config: TransientNoiseConfig,
    tstop: Value,
) -> (CircuitData, TransientNoiseRuntime, Vec<Value>) {
    let engine = engine();
    let netlist = Netlist::parse(deck).expect("the fixture deck parses");
    let mut circuit = engine
        .build_circuit(&netlist)
        .expect("the fixture circuit builds");
    let mut matrix = engine
        .build_matrix(&circuit)
        .expect("the fixture matrix builds");
    circuit.link_indices(&matrix);
    let solution = engine
        .solve_dc_operating_point(&netlist, &mut circuit, &mut matrix)
        .expect("the fixture operating point converges");
    if circuit.has_nonlinear_devices() {
        circuit.update_nonlinear(&solution);
    }
    let runtime = engine
        .install_transient_device_noise(&mut circuit, &solution, config, tstop, None, &NoAbort)
        .expect("the injection plan installs");
    (circuit, runtime, solution)
}

/// One injected source's complete sample train, read from the plan.
fn injected_train(circuit: &CircuitData, source: usize) -> Vec<Value> {
    let plan = circuit
        .transient_device_noise()
        .expect("the plan is installed");
    let nt = plan.sample_interval();
    (0..plan.sample_count())
        .map(|index| plan.source_current(source, (index as Value + 0.5) * nt))
        .collect()
}

/// Johnson-Nyquist power in the noise band: the mean square of a
/// resistor's own node voltage is `4kTRf`, with nothing else in the
/// circuit to shape it.
#[test]
fn a_resistor_at_300k_produces_4ktr_noise_power_in_band() {
    const RESISTANCE: Value = 1.0e3;
    const FMAX: Value = 1.0e9;
    const SAMPLES: usize = 100_000;
    let nt = 1.0 / (2.0 * FMAX);
    let tstop = SAMPLES as Value * nt;
    let result = run(
        &format!(
            "johnson nyquist in band\n\
             R1 out 0 {RESISTANCE}\n\
             .TRAN {nt:e} {tstop:e} NOISEFMAX={FMAX:e} NOISESEED=20260916\n\
             .END\n"
        ),
        tstop,
        nt,
    );
    let waveform = result
        .try_voltage_waveform_named("out")
        .expect("the resistor's node is retained");
    assert_eq!(waveform[0], 0.0, "t=0 carries the deterministic point");

    let measured = time_averaged_square(&result, waveform);
    let expected = 4.0 * boltzmann() * T * RESISTANCE * FMAX;
    // The estimator's relative standard error is sqrt(2/N) = 0.45% at
    // 100000 independent held samples, so 3% is 6.7 standard errors.
    let error = (measured / expected - 1.0).abs();
    assert!(
        error < 0.03,
        "measured {measured:e} V^2 against 4kTRf = {expected:e} V^2 ({:.2}% off)",
        error * 100.0
    );
}

/// Equipartition: whatever the resistance and whatever the noise
/// bandwidth, a capacitor in parallel with a resistor holds `kT/C` of
/// mean-square voltage. This is the strongest oracle available, because
/// the resistance cancels out of it entirely.
///
/// The deck carries a bank of independent sections rather than one, for
/// precision alone: the estimate's error falls with the number of
/// independent correlation times observed, and a bank collects them
/// across sections instead of across a run 64 times as long.
#[test]
fn a_resistor_capacitor_pair_settles_to_kt_over_c() {
    const SECTIONS: usize = 64;
    const CAPACITANCE: Value = 1.0e-12;
    const RESISTANCE: Value = 1.0e3;
    const SAMPLES: usize = 20_000;
    // 100 times the RC corner, which leaves the sample-and-hold roll-off
    // at the corner at pi/300 = 1.0% and the rest of the band far above
    // where the lowpass has any weight.
    let corner = 1.0 / (2.0 * std::f64::consts::PI * RESISTANCE * CAPACITANCE);
    let fmax = 100.0 * corner;
    let nt = 1.0 / (2.0 * fmax);
    let tstop = SAMPLES as Value * nt;

    let mut deck = String::from("equipartition bank\n");
    for section in 1..=SECTIONS {
        deck.push_str(&format!(
            "R{section} n{section} 0 {RESISTANCE}\nC{section} n{section} 0 {CAPACITANCE:e}\n"
        ));
    }
    deck.push_str(&format!(
        ".TRAN {nt:e} {tstop:e} NOISEFMAX={fmax:e} NOISESEED=20260916\n.END\n"
    ));
    let result = run(&deck, tstop, nt);

    let expected = boltzmann() * T / CAPACITANCE;
    let mut total = 0.0;
    for section in 1..=SECTIONS {
        let waveform = result
            .try_voltage_waveform_named(&format!("n{section}"))
            .unwrap_or_else(|| panic!("section {section} is retained"));
        total += time_averaged_square(&result, waveform);
    }
    let measured = total / SECTIONS as Value;
    let error = (measured / expected - 1.0).abs();
    assert!(
        error < 0.03,
        "measured {measured:e} V^2 against kT/C = {expected:e} V^2 ({:.2}% off)",
        error * 100.0
    );
}

/// Determinism: the same seed replays the same realization exactly, a
/// different seed does not, and a device that carries no noise model does
/// not disturb another device's stream.
#[test]
fn a_seeded_transient_noise_run_is_bit_identical() {
    const FMAX: Value = 1.0e9;
    let nt = 1.0 / (2.0 * FMAX);
    let tstop = 256.0 * nt;
    let deck = |seed: u64, extra: &str| {
        format!(
            "seeded transient noise\n\
             R1 out 0 1k\n\
             {extra}\
             .TRAN {nt:e} {tstop:e} NOISEFMAX={FMAX:e} NOISESEED={seed}\n\
             .END\n"
        )
    };

    let first = run(&deck(7, ""), tstop, nt);
    let again = run(&deck(7, ""), tstop, nt);
    let first_wave = first.try_voltage_waveform_named("out").expect("retained");
    let again_wave = again.try_voltage_waveform_named("out").expect("retained");
    assert_eq!(
        first.time, again.time,
        "the same seed replays the same grid"
    );
    assert_eq!(
        first_wave, again_wave,
        "the same seed replays the same waveform bit for bit"
    );
    assert!(
        first_wave.iter().any(|value| *value != 0.0),
        "the run must actually inject noise"
    );

    let other = run(&deck(8, ""), tstop, nt);
    let other_wave = other.try_voltage_waveform_named("out").expect("retained");
    assert_ne!(
        first_wave, other_wave,
        "a different seed must draw a different realization"
    );

    // A zero-gain VCVS carries no noise model and draws no current from
    // `out`, so it renumbers the system without changing this circuit.
    let with_inert = run(&deck(7, "E9 n9 0 out 0 0\n"), tstop, nt);
    let inert_wave = with_inert
        .try_voltage_waveform_named("out")
        .expect("retained");
    assert_eq!(
        first_wave, inert_wave,
        "adding a noiseless device must leave R1's train alone"
    );
}

/// `NOISESCALE` multiplies every injected amplitude, and zero reproduces
/// the deterministic solution exactly.
#[test]
fn noisescale_scales_the_injected_rms_linearly() {
    const FMAX: Value = 1.0e9;
    let nt = 1.0 / (2.0 * FMAX);
    let tstop = 4096.0 * nt;
    let deck = |scale: &str| {
        format!(
            "noise scale\n\
             V1 in 0 1\n\
             R1 in out 1k\n\
             R2 out 0 1k\n\
             .TRAN {nt:e} {tstop:e} NOISEFMAX={FMAX:e} NOISESEED=11 {scale}\n\
             .END\n"
        )
    };

    let unity = run(&deck("NOISESCALE=1"), tstop, nt);
    let doubled = run(&deck("NOISESCALE=2"), tstop, nt);
    let unity_wave = unity.try_voltage_waveform_named("out").expect("retained");
    let doubled_wave = doubled.try_voltage_waveform_named("out").expect("retained");
    // The deterministic divider output is exactly 0.5; the fluctuation
    // about it is what the scale multiplies.
    let unity_power = unity_wave
        .iter()
        .skip(1)
        .map(|value| (value - 0.5) * (value - 0.5))
        .sum::<Value>();
    let doubled_power = doubled_wave
        .iter()
        .skip(1)
        .map(|value| (value - 0.5) * (value - 0.5))
        .sum::<Value>();
    assert!(unity_power > 0.0, "the unity-scale run must inject noise");
    let ratio = (doubled_power / unity_power).sqrt();
    assert!(
        (ratio - 2.0).abs() < 1.0e-9,
        "NOISESCALE=2 must double the injected RMS, found {ratio}"
    );

    // NOISESCALE=0 leaves the assembly with no injected current at all,
    // so a time-invariant linear deck holds its operating point exactly.
    let silent = run(&deck("NOISESCALE=0"), tstop, nt);
    let silent_wave = silent.try_voltage_waveform_named("out").expect("retained");
    let operating_point = silent_wave[0];
    assert!(
        (operating_point - 0.5).abs() < 1.0e-9,
        "the divider's operating point is {operating_point}, not 0.5"
    );
    for (index, value) in silent_wave.iter().enumerate() {
        assert_eq!(
            *value, operating_point,
            "NOISESCALE=0 must hold the deterministic solution at point {index}"
        );
    }
}

/// The injected flicker current follows the device model's own
/// `KF·I^AF/f^EF` law, in magnitude as well as slope.
#[test]
fn flicker_noise_follows_the_devices_af_ef_law_in_time() {
    const FMAX: Value = 1.0e5;
    const SAMPLES: usize = 1 << 17;
    let nt = 1.0 / (2.0 * FMAX);
    let tstop = SAMPLES as Value * nt;
    let config = TransientNoiseConfig {
        fmax: FMAX,
        fmin: None,
        seed: Some(3),
        scale: 1.0,
    };
    // A LEVEL=1 MOSFET at NLEV=0 takes both exponents from its own card
    // — `KF·Id^AF/(Leff²·Cox·f^EF)` — so the law under test is the
    // model's and not a fixed 1/f.
    let (circuit, runtime, solution) = installed(
        "level one flicker law\n\
         VG g 0 2\n\
         VD d 0 1\n\
         M1 d g 0 0 NM W=10u L=1u\n\
         .MODEL NM NMOS LEVEL=1 VTO=0.5 KP=100u NLEV=0 KF=1e-22 AF=1.2 EF=1.1\n\
         .END\n",
        config,
        tstop,
    );

    let position = runtime
        .entries
        .iter()
        .position(|entry| {
            entry.law == AmplitudeLaw::PowerLaw && entry.identity.device.eq_ignore_ascii_case("M1")
        })
        .expect("the MOSFET exports a flicker mechanism");
    let entry = &runtime.entries[position];
    assert_eq!(entry.identity.mechanism.as_deref(), Some("FN"));

    let train = injected_train(&circuit, position);
    let estimate = welch_one_sided(&train, nt, 1 << 13);

    // The oracle is the mechanism's own density, evaluated through the
    // same public accessor `.NOISE` uses.
    let collected = Engine::try_collect_noise_sources(
        &circuit,
        &solution,
        crate::config::SpiceDialect::BestAvailable,
    )
    .expect("the catalog collects")
    .elementary;
    let source = collected
        .iter()
        .find(|source| {
            source.identity.device.eq_ignore_ascii_case("M1")
                && source.identity.mechanism.as_deref() == Some("FN")
        })
        .expect("the flicker mechanism is in the catalog");
    let ef = source.ef;
    assert!(
        (ef - 1.1).abs() < 1.0e-12,
        "the card's EF must reach the model, found {ef}"
    );

    let fmin = 1.0 / tstop;
    let (slope, _) = log_log_fit(&estimate, 10.0 * fmin, FMAX / 10.0);
    assert!(
        (slope + ef).abs() < 0.1,
        "the injected spectrum's slope is {slope}, not -{ef}"
    );

    // Compare the estimate over a band around 1 kHz against the model
    // evaluated on the same bins. A periodogram bin is an exponential
    // random variable, so a band is what makes the comparison precise;
    // averaging the model over the same bins is what keeps it a
    // comparison of densities rather than of a density against a
    // band mean of a power law, which differ by several percent.
    const BAND_LOW: Value = 500.0;
    const BAND_HIGH: Value = 2000.0;
    let measured = band_average(&estimate, BAND_LOW, BAND_HIGH);
    let expected = band_average(
        &estimate
            .iter()
            .map(|(frequency, _)| (*frequency, source.spectral_density(*frequency, T)))
            .collect::<Vec<_>>(),
        BAND_LOW,
        BAND_HIGH,
    );
    assert!(
        (source.spectral_density(1.0e3, T) / expected - 1.0).abs() < 0.2,
        "the band must sit on the 1 kHz decade"
    );
    let error = (measured / expected - 1.0).abs();
    assert!(
        error < 0.10,
        "the injected density over {BAND_LOW}-{BAND_HIGH} Hz is {measured:e} A^2/Hz against \
         the model's {expected:e} A^2/Hz ({:.1}% off)",
        error * 100.0
    );
}

/// Spectre practice: the injected density follows the instantaneous bias.
/// A diode's shot noise is `2qI`, so its injected amplitude must scale as
/// the square root of the bias current — measured here against the
/// current the deterministic solution carries, which the noise code has
/// no part in computing.
#[test]
fn transient_noise_tracks_the_instantaneous_bias() {
    const FMAX: Value = 1.0e9;
    const RESISTANCE: Value = 1.0e3;
    let nt = 1.0 / (2.0 * FMAX);
    let tstop = 1024.0 * nt;
    let config = TransientNoiseConfig {
        fmax: FMAX,
        fmin: None,
        seed: Some(5),
        scale: 1.0,
    };
    let deck = |drive: Value| {
        format!(
            "shot noise follows the bias\n\
             V1 in 0 {drive}\n\
             R1 in a {RESISTANCE}\n\
             D1 a 0 DM\n\
             .MODEL DM D IS=1e-14 N=1\n\
             .END\n"
        )
    };

    let mut samples = Vec::new();
    for drive in [0.68, 20.0] {
        let (circuit, runtime, solution) = installed(&deck(drive), config, tstop);
        let position = runtime
            .entries
            .iter()
            .position(|entry| entry.identity.device.eq_ignore_ascii_case("D1"))
            .expect("the diode exports a shot-noise mechanism");
        assert_eq!(runtime.entries[position].law, AmplitudeLaw::Flat);
        let node = circuit
            .get_node_by_name("a")
            .expect("the diode's anode exists");
        let bias = (drive - solution[node - 1]) / RESISTANCE;
        assert!(bias > 0.0, "the diode must be forward biased at {drive} V");
        let amplitude = circuit
            .transient_device_noise()
            .expect("installed")
            .amplitude(position);
        samples.push((bias, amplitude));
    }

    let (early_current, early_amplitude) = samples[0];
    let (late_current, late_amplitude) = samples[1];
    assert!(
        late_current / early_current > 100.0,
        "the two bias points must be far apart: {early_current:e} A then {late_current:e} A"
    );
    let measured = late_amplitude / early_amplitude;
    let expected = (late_current / early_current).sqrt();
    let error = (measured / expected - 1.0).abs();
    assert!(
        error < 0.10,
        "the shot-noise amplitude scaled by {measured} where sqrt(I_late/I_early) is \
         {expected} ({:.2}% off)",
        error * 100.0
    );
}

/// A mechanism whose spectrum the time domain cannot represent is refused
/// by name rather than approximated.
///
/// The classification is tested directly because the two refused shapes
/// reach a deck by different routes: a tabulated density arrives from any
/// EKV3 card and from Verilog-A `noise_table`, while the Lorentzian is a
/// density `NoiseSource::burst` can carry that no shipping device model
/// currently exports — so the refusal has to be proven on the source, not
/// on a deck that happens to produce one today.
#[test]
fn an_unrenderable_noise_mechanism_is_refused_by_name() {
    let tabulated = NoiseSource::tabulated(
        "X1".to_string(),
        1,
        0,
        1.0,
        vec![(1.0, 1.0e-20), (1.0e6, 1.0e-22)],
        true,
    );
    let message = rendering_law(&tabulated).expect_err("a table cannot be rendered");
    assert!(
        message.contains("X1") && message.contains("tabulated"),
        "{message}"
    );

    let burst = NoiseSource::burst("Q1".to_string(), 1, 0, 1.0e-12, 2.0, 1.0e-6, 1.0e3)
        .with_identity(NoiseSourceIdentity::mechanism("Q1", "BN"));
    let message = rendering_law(&burst).expect_err("a Lorentzian cannot be rendered");
    assert!(
        message.contains("Q1:BN") && message.contains("random-telegraph"),
        "{message}"
    );

    let steep = NoiseSource::flicker_with_frequency_exponent(
        "M1".to_string(),
        1,
        0,
        1.0e-24,
        1.0,
        2.5,
        1.0e-3,
    );
    let message = rendering_law(&steep).expect_err("a 1/f^2.5 law cannot be rendered");
    assert!(
        message.contains("M1") && message.contains("2.5"),
        "{message}"
    );

    // The renderable shapes classify without complaint, and an EF of zero
    // is a flat density rather than a degenerate power law.
    assert_eq!(
        rendering_law(&NoiseSource::thermal("R1".to_string(), 1, 0, 1.0e3)),
        Ok(Some(AmplitudeLaw::Flat))
    );
    assert_eq!(
        rendering_law(&NoiseSource::shot("D1".to_string(), 1, 0, 1.0e-6)),
        Ok(Some(AmplitudeLaw::Flat))
    );
    assert_eq!(
        rendering_law(&NoiseSource::flicker_with_frequency_exponent(
            "M2".to_string(),
            1,
            0,
            1.0e-24,
            1.0,
            0.0,
            1.0e-3,
        )),
        Ok(Some(AmplitudeLaw::Flat))
    );
    assert_eq!(
        rendering_law(&NoiseSource::flicker(
            "M3".to_string(),
            1,
            0,
            1.0e-24,
            1.0,
            1.0e-3
        )),
        Ok(Some(AmplitudeLaw::PowerLaw))
    );
}

/// A deck that asks for transient noise but has no noise mechanism runs
/// as an ordinary deterministic transient.
#[test]
fn a_deck_without_noise_mechanisms_runs_deterministically() {
    let nt = 1.0e-9;
    let tstop = 64.0e-9;
    let config = TransientNoiseConfig {
        fmax: 1.0 / (2.0 * nt),
        fmin: None,
        seed: Some(1),
        scale: 1.0,
    };
    let (circuit, runtime, _) = installed(
        "no noise mechanisms\n\
         V1 in 0 1\n\
         E1 out 0 in 0 2\n\
         .END\n",
        config,
        tstop,
    );
    assert_eq!(runtime.source_count(), 0);
    let mut rhs = vec![0.0; circuit.matrix_size()];
    circuit.stamp_transient_device_noise(&mut rhs, tstop / 2.0);
    assert!(rhs.iter().all(|value| *value == 0.0));
}

/// A one-sided Welch periodogram of a sampled sequence, in units of the
/// sequence's square per hertz.
fn welch_one_sided(samples: &[Value], nt: Value, segment: usize) -> Vec<(Value, Value)> {
    use rustfft::{FftPlanner, num_complex::Complex};
    assert!(samples.len() >= segment, "the record must cover a segment");
    // Remove the record mean so the window's leakage from bin zero cannot
    // be mistaken for power at the lowest frequencies the fit uses.
    let mean = samples.iter().sum::<Value>() / samples.len() as Value;
    let samples: Vec<Value> = samples.iter().map(|value| value - mean).collect();
    let window: Vec<Value> = (0..segment)
        .map(|index| {
            let phase = std::f64::consts::TAU * index as Value / (segment as Value - 1.0);
            0.5 - 0.5 * phase.cos()
        })
        .collect();
    let normalization: Value = window.iter().map(|value| value * value).sum();
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(segment);
    let bins = segment / 2 + 1;
    let mut accumulated = vec![0.0; bins];
    let mut segments = 0usize;
    let mut start = 0usize;
    while start + segment <= samples.len() {
        let mut buffer: Vec<Complex<Value>> = samples[start..start + segment]
            .iter()
            .zip(&window)
            .map(|(value, weight)| Complex::new(value * weight, 0.0))
            .collect();
        fft.process(&mut buffer);
        for (bin, slot) in accumulated.iter_mut().enumerate() {
            let scale = if bin == 0 || bin == segment / 2 {
                1.0
            } else {
                2.0
            };
            *slot += scale * nt * buffer[bin].norm_sqr() / normalization;
        }
        segments += 1;
        start += segment / 2;
    }
    assert!(segments > 0);
    let resolution = 1.0 / (segment as Value * nt);
    accumulated
        .into_iter()
        .enumerate()
        .map(|(bin, total)| (bin as Value * resolution, total / segments as Value))
        .collect()
}

/// Least-squares slope and intercept of `log10(density)` against
/// `log10(frequency)` over a band.
fn log_log_fit(estimate: &[(Value, Value)], low: Value, high: Value) -> (Value, Value) {
    let points: Vec<(Value, Value)> = estimate
        .iter()
        .filter(|(frequency, density)| *frequency >= low && *frequency <= high && *density > 0.0)
        .map(|(frequency, density)| (frequency.log10(), density.log10()))
        .collect();
    assert!(points.len() >= 8, "the band must hold enough bins");
    let count = points.len() as Value;
    let mean_x = points.iter().map(|(x, _)| x).sum::<Value>() / count;
    let mean_y = points.iter().map(|(_, y)| y).sum::<Value>() / count;
    let covariance: Value = points
        .iter()
        .map(|(x, y)| (x - mean_x) * (y - mean_y))
        .sum();
    let variance: Value = points
        .iter()
        .map(|(x, _)| (x - mean_x) * (x - mean_x))
        .sum();
    let slope = covariance / variance;
    (slope, mean_y - slope * mean_x)
}

/// Mean estimated density over a band.
fn band_average(estimate: &[(Value, Value)], low: Value, high: Value) -> Value {
    let mut total = 0.0;
    let mut count = 0usize;
    for (frequency, density) in estimate {
        if *frequency >= low && *frequency <= high {
            total += density;
            count += 1;
        }
    }
    assert!(count > 0, "the band must hold at least one bin");
    total / count as Value
}
