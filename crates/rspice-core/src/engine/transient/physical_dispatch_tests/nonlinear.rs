use super::*;
use std::fmt::Write;

const PERIOD: Value = 4e-6;
const DELAY: Value = 1e-6;
const TF: Value = 1e-6;
const IS: Value = 1e-14;
const BF: Value = 100.0;
const RB: Value = 10_000.0;
const RC: Value = 1000.0;
const CB: Value = 1e-9;
const CC: Value = 1e-9;
const BASE: Value = 0.6;
const COLLECTOR: Value = 2.0;
const BASE_AMPLITUDE: Value = 0.01;
const COLLECTOR_AMPLITUDE: Value = 0.1;
const HARMONICS: usize = 12;

fn prescribed(time: Value) -> (Value, Value, Value, Value) {
    let theta = std::f64::consts::TAU * time.max(0.0) / PERIOD;
    let speed = if time > 0.0 {
        std::f64::consts::TAU / PERIOD * theta.sin()
    } else {
        0.0
    };
    (
        BASE + BASE_AMPLITUDE * (1.0 - theta.cos()),
        COLLECTOR + COLLECTOR_AMPLITUDE * (1.0 - theta.cos()),
        BASE_AMPLITUDE * speed,
        COLLECTOR_AMPLITUDE * speed,
    )
}

/// Independent GP equations for NF=1, VAF=1, no high-injection rolloff and
/// no depletion/reverse transit charge. Both voltages are unknowns in the
/// actual circuit: finite RB/RC and CB/CC, no ideal voltage clamp at B or C.
fn prescribed_transport(time: Value, vt: Value) -> (Value, Value, Value) {
    let (base, collector, db, dc) = prescribed(time);
    let forward = IS * (base / vt).exp_m1();
    let early = 1.0 - base + collector;
    (
        forward,
        forward * early,
        IS * (base / vt).exp() / vt * db * early + forward * (dc - db),
    )
}

/// Positive-term power series for I_n(x). Here x < .4, so twelve harmonics
/// leave transport and its first derivative well below floating-point/source
/// tolerances. This constructs forcing from equations, not solver samples.
fn bessel_i(n: usize, x: Value) -> Value {
    let mut term = 1.0;
    for k in 1..=n {
        term *= x / (2.0 * k as Value);
    }
    let mut sum = term;
    for k in 1..=24 {
        term *= x * x / (4.0 * k as Value * (n + k) as Value);
        sum += term;
    }
    sum
}

fn harmonic_source(
    deck: &mut String,
    name: &str,
    node: &str,
    n: usize,
    amplitude: Value,
    delay: Value,
    cosine: bool,
) {
    let offset = if cosine { -amplitude } else { 0.0 };
    let phase = if cosine { 90 } else { 0 };
    // DC 0 preserves the initial bias; transient cos(theta)-1 and sin(theta)
    // also vanish before their activation. SIN retains its authored phase.
    writeln!(
        deck,
        "I{name}{n} 0 {node} DC 0 SIN({offset:.17e} {amplitude:.17e} {:.17e} {delay:.17e} 0 {phase})",
        n as Value / PERIOD
    )
    .unwrap();
}

fn nonlinear_deck(vt: Value) -> String {
    let z = BASE_AMPLITUDE / vt;
    let scale = IS * ((BASE + BASE_AMPLITUDE) / vt).exp();
    let exponential: Vec<_> = (0..HARMONICS + 2)
        .map(|n| {
            let multiplicity = if n == 0 { 1.0 } else { 2.0 };
            let sign = if n % 2 == 0 { 1.0 } else { -1.0 };
            multiplicity * sign * bessel_i(n, z)
        })
        .collect();
    let g = 1.0 - BASE + COLLECTOR + COLLECTOR_AMPLITUDE - BASE_AMPLITUDE;
    let h = COLLECTOR_AMPLITUDE - BASE_AMPLITUDE;
    let mut current = Vec::new();
    let mut transport = Vec::new();
    for n in 0..=HARMONICS {
        current.push(scale * exponential[n] - if n == 0 { IS } else { 0.0 });
        let product = match n {
            0 => exponential[1] / 2.0,
            1 => exponential[0] + exponential[2] / 2.0,
            _ => (exponential[n - 1] + exponential[n + 1]) / 2.0,
        };
        transport.push(
            scale * (g * exponential[n] - h * product) - if n == 0 { IS * g } else { 0.0 }
                + if n == 1 { IS * h } else { 0.0 },
        );
    }
    // Check the manufactured forcing against its closed form before giving
    // it to the engine. The reference never calls a device or delay method.
    for sample in 0..257 {
        let time = PERIOD * sample as Value / 257.0;
        let theta = std::f64::consts::TAU * time / PERIOD;
        let (expected_if, expected_f, expected_df) = prescribed_transport(time, vt);
        let mut actual_if = current[0];
        let mut actual_f = transport[0];
        let mut actual_df = 0.0;
        for n in 1..=HARMONICS {
            actual_if += current[n] * (n as Value * theta).cos();
            actual_f += transport[n] * (n as Value * theta).cos();
            actual_df -= transport[n] * n as Value * std::f64::consts::TAU / PERIOD
                * (n as Value * theta).sin();
        }
        assert!((actual_if - expected_if).abs() < 1e-16);
        assert!((actual_f - expected_f).abs() < 1e-16);
        assert!(TF * (actual_df - expected_df).abs() < 1e-16);
    }
    let (initial_if, initial_f, _) = prescribed_transport(0.0, vt);
    let mut deck = format!(
        "Manufactured nonlinear GP delay feedback\nVs s 0 3\nRb b 0 {RB}\nRc c s {RC}\nCb b 0 {CB:.17e}\nCc c 0 {CC:.17e}\nIbiasb 0 b {:.17e}\nIbiasc 0 c {:.17e}\nQ1 c b 0 qm\n.model qm NPN(IS=1e-14 BF=100 BR=1 VAF=1 TF=1u PTF=57.29577951308232 TNOM=27)\n.options gmin=0 temp=27 reltol=1e-7 abstol=1e-14 vntol=1e-10\n",
        BASE / RB + initial_if / BF,
        (COLLECTOR - 3.0) / RC + initial_f,
    );
    let omega = std::f64::consts::TAU / PERIOD;
    for n in 1..=HARMONICS {
        let base_cos = current[n] / BF - if n == 1 { BASE_AMPLITUDE / RB } else { 0.0 };
        let base_sin = -TF * n as Value * omega * transport[n]
            + if n == 1 {
                CB * BASE_AMPLITUDE * omega
            } else {
                0.0
            };
        harmonic_source(&mut deck, "bcos", "b", n, base_cos, 0.0, true);
        harmonic_source(&mut deck, "bsin", "b", n, base_sin, 0.0, false);
        harmonic_source(&mut deck, "delay", "c", n, transport[n], DELAY, true);
    }
    harmonic_source(
        &mut deck,
        "ccos",
        "c",
        1,
        -COLLECTOR_AMPLITUDE / RC,
        0.0,
        true,
    );
    harmonic_source(
        &mut deck,
        "csin",
        "c",
        1,
        CC * COLLECTOR_AMPLITUDE * omega,
        0.0,
        false,
    );
    deck.push_str(".tran 5n 12u\n.end\n");
    deck
}

fn check_nonlinear_feedback(dialect: SpiceDialect, method: IntegrationMethod) {
    let vt = match dialect {
        SpiceDialect::Xyce => 1.380_622_6e-23 * 300.15 / 1.602_191_8e-19,
        _ => 1.380_649e-23 * 300.15 / 1.602_176_634e-19,
    };
    let deck = nonlinear_deck(vt);
    let config = SimulationConfig {
        integration_method: method,
        ..SimulationConfig::default().with_spice_dialect(dialect)
    };
    let (result, _) = run_with_configuration(&deck, 12e-6, 5e-9, None, &[], config);
    let base = result.try_voltage_waveform_named("b").unwrap();
    let collector = result.try_voltage_waveform_named("c").unwrap();
    let mut base_error = 0.0_f64;
    let mut collector_error = 0.0_f64;
    let mut delay_contribution = 0.0_f64;
    let mut charge_contribution = 0.0_f64;
    for ((&time, &base), &collector) in result.time.iter().zip(base).zip(collector) {
        let (expected_base, expected_collector, _, _) = prescribed(time);
        base_error = base_error.max((base - expected_base).abs());
        collector_error = collector_error.max((collector - expected_collector).abs());
        let (_, forward, rate) = prescribed_transport(time, vt);
        let (_, delayed, _) = prescribed_transport(time - DELAY, vt);
        delay_contribution = delay_contribution.max((forward - delayed).abs());
        charge_contribution = charge_contribution.max((TF * rate).abs());
    }
    eprintln!(
        "GP nonlinear {dialect:?}/{method:?}: points={}, base_error={base_error:e}, collector_error={collector_error:e}",
        result.time.len()
    );
    // Reverse junction leakage omitted from the forcing is <4e-14 A, far
    // below these voltage budgets even through RB. Both nonlinear forward
    // transport and TF diffusion charge change substantially along the orbit.
    assert!(delay_contribution > 1e-4);
    assert!(charge_contribution > 1e-4);
    assert!(base_error < 2e-6, "base error {base_error:e}");
    assert!(
        collector_error < 2e-5,
        "collector error {collector_error:e}"
    );
}

#[test]
fn physical_dispatch_manufactured_nonlinear_ngspice() {
    check_nonlinear_feedback(SpiceDialect::Ngspice, IntegrationMethod::Trapezoidal);
}

#[test]
fn physical_dispatch_manufactured_nonlinear_xyce() {
    check_nonlinear_feedback(SpiceDialect::Xyce, IntegrationMethod::TrapGear);
}
