use super::*;

const RB: f64 = 25.0;
const RC: f64 = 5.0;
const RE: f64 = 2.0;

// Commensurate 500/700 MHz trajectories have a 100 MHz common fundamental.
// This independent Fourier construction supplies native current sources;
// no behavioral-source or numerical-device evaluator generates the forcing.
const PERIOD: f64 = 1e-8;
const HARMONICS: usize = 96;
const SAMPLES: usize = 2048;
const LOAD: [f64; 3] = [100.0, 200.0, 50.0];
const RESISTANCE: [f64; 3] = [RC, RB, RE];

struct Forcing {
    initial: [f64; 3],
    cosine: Vec<[f64; 3]>,
    sine: Vec<[f64; 3]>,
    delayed: Vec<f64>,
}

fn undelayed_forcing(regime: Regime, time: f64, dialect: SpiceDialect) -> ([f64; 3], f64) {
    let state = regime.state(time, dialect);
    let (qbe, qbc) = ChargeLaw::BIASED.rates(&state);
    let ic = -state.reverse / state.charge_factor - state.reverse / BR - qbc;
    let ib = state.forward / BF + state.reverse / BR + qbe + qbc;
    let currents = [ic, ib, -ic - ib];
    let intrinsic = [state.collector, state.base, 0.0];
    (
        std::array::from_fn(|i| {
            (1.0 + RESISTANCE[i] / LOAD[i]) * currents[i] + intrinsic[i] / LOAD[i]
        }),
        state.forward_transport,
    )
}

impl Forcing {
    fn new(regime: Regime, dialect: SpiceDialect) -> Self {
        let (initial, phase) = undelayed_forcing(regime, 0.0, dialect);
        let mut result = Self {
            initial: std::array::from_fn(|i| initial[i] + Self::phase_weight(i) * phase),
            cosine: vec![[0.0; 3]; HARMONICS],
            sine: vec![[0.0; 3]; HARMONICS],
            delayed: vec![0.0; HARMONICS],
        };
        for sample in 0..SAMPLES {
            let theta = std::f64::consts::TAU * sample as f64 / SAMPLES as f64;
            let (current, forward) =
                undelayed_forcing(regime, PERIOD * sample as f64 / SAMPLES as f64, dialect);
            for harmonic in 0..HARMONICS {
                let (sin, cos) = ((harmonic + 1) as f64 * theta).sin_cos();
                for (i, &value) in current.iter().enumerate() {
                    result.cosine[harmonic][i] += 2.0 / SAMPLES as f64 * value * cos;
                    result.sine[harmonic][i] += 2.0 / SAMPLES as f64 * value * sin;
                }
                result.delayed[harmonic] += 2.0 / SAMPLES as f64 * forward * cos;
            }
        }
        // Remove summation noise, with the total discarded amplitude far
        // below the independently checked forcing accuracy below.
        for coefficient in result
            .cosine
            .iter_mut()
            .flatten()
            .chain(result.sine.iter_mut().flatten())
            .chain(&mut result.delayed)
        {
            if coefficient.abs() < 1e-16 {
                *coefficient = 0.0;
            }
        }
        // Off-grid checks also cover the constant delayed prehistory and its
        // onset. This tolerance is orders tighter than the circuit accuracy
        // gate; a truncated/aliased forcing cannot masquerade as solver error.
        for sample in 0..1025 {
            let time = STOP * sample as f64 / 1024.0;
            let (current, _) = undelayed_forcing(regime, time, dialect);
            let phase = regime.state(time - DELAY, dialect).forward_transport;
            for (i, actual) in result.at(time).into_iter().enumerate() {
                let expected = current[i] + Self::phase_weight(i) * phase;
                assert!(
                    (actual - expected).abs() < 1e-12,
                    "{regime:?}/{dialect:?}/{time}/{i}: forcing {actual:e} != {expected:e}"
                );
            }
        }
        result
    }

    fn phase_weight(terminal: usize) -> f64 {
        [1.0, 0.0, -1.0][terminal] * (1.0 + RESISTANCE[terminal] / LOAD[terminal])
    }

    fn at(&self, time: f64) -> [f64; 3] {
        let mut result = self.initial;
        for harmonic in 0..HARMONICS {
            let omega = std::f64::consts::TAU * (harmonic + 1) as f64 / PERIOD;
            let (sin, cos) = (omega * time).sin_cos();
            let delayed = self.delayed[harmonic] * ((omega * (time - DELAY).max(0.0)).cos() - 1.0);
            for (i, value) in result.iter_mut().enumerate() {
                *value += self.cosine[harmonic][i] * (cos - 1.0)
                    + self.sine[harmonic][i] * sin
                    + Self::phase_weight(i) * delayed;
            }
        }
        result
    }
}

/// Actual model RB/RC/RE and external resistive loads leave all intrinsic and
/// external voltages as unknowns. Native injected currents manufacture the
/// target orbit through feedback; the test does not clamp a BJT terminal.
fn deck(regime: Regime, polarity: f64, dialect: SpiceDialect) -> Netlist {
    use std::fmt::Write;
    let kind = if polarity > 0.0 { "NPN" } else { "PNP" };
    let forcing = Forcing::new(regime, dialect);
    let mut text = format!(
        "Manufactured private GP {}\nQ1 c b e qm\n.model qm {kind} IS={IS} BF={BF} BR={BR} IKF={IKF} IKR={IKR} VAF={VAF} VAR={VAR} TF={TF} PTF=57.29577951308232 TNOM=27 RB={RB} RBM={RB} RC={RC} RE={RE} {}\n.options TEMP=27 GMIN=0 RELTOL=1e-5 ABSTOL=1e-15 VNTOL=1e-10 METHOD=TRAP\n",
        regime.name,
        ChargeLaw::BIASED.parameters()
    );
    for (i, name) in ["c", "b", "e"].into_iter().enumerate() {
        writeln!(text, "V{name} {name} drive_{name} 0\nR{name} drive_{name} 0 {}\nI{name}dc 0 drive_{name} {:.17e}", LOAD[i], polarity * forcing.initial[i]).unwrap();
        for harmonic in 0..HARMONICS {
            for (suffix, coefficient, delay, cosine) in [
                ("cos", forcing.cosine[harmonic][i], 0.0, true),
                ("sin", forcing.sine[harmonic][i], 0.0, false),
                (
                    "delay",
                    Forcing::phase_weight(i) * forcing.delayed[harmonic],
                    DELAY,
                    true,
                ),
            ] {
                if coefficient == 0.0 {
                    continue;
                }
                let amplitude = polarity * coefficient;
                let offset = if cosine { -amplitude } else { 0.0 };
                let phase = if cosine { 90 } else { 0 };
                writeln!(text, "I{name}{suffix}{harmonic} 0 drive_{name} DC 0 SIN({offset:.17e} {amplitude:.17e} {:.17e} {delay:.17e} 0 {phase})", (harmonic + 1) as f64 / PERIOD).unwrap();
            }
        }
    }
    text.push_str(".end\n");
    Netlist::parse(&text).unwrap()
}

#[derive(Debug)]
struct PrivateError {
    current: [f64; 3],
    peak: [f64; 3],
    voltage: [f64; 3],
}

fn run_private(regime: Regime, polarity: f64, dialect: SpiceDialect, step: f64) -> PrivateError {
    let mut config = SimulationConfig {
        gp_transient_phase_model: GpTransientPhaseModel::ExactDelay,
        integration_method: IntegrationMethod::Trapezoidal,
        ..SimulationConfig::default().with_spice_dialect(dialect)
    };
    config.convergence_config.gmin_target = 0.0;
    let result = Engine::new(config)
        .run_tran(&deck(regime, polarity, dialect), STOP, step)
        .unwrap_or_else(|error| panic!("{regime:?}/{dialect:?}/{polarity}: {error}"));
    let currents =
        ["vc", "vb", "ve"].map(|name| result.try_branch_current_waveform_named(name).unwrap());
    let voltages = ["c", "b", "e"].map(|name| result.try_voltage_waveform_named(name).unwrap());
    let mut error = PrivateError {
        current: [0.0; 3],
        peak: [0.0; 3],
        voltage: [0.0; 3],
    };
    assert_eq!(result.time.last(), Some(&STOP));
    for (index, &time) in result.time.iter().enumerate() {
        let state = regime.state(time, dialect);
        let phase = regime.state(time - DELAY, dialect).forward_transport;
        let (qbe_rate, qbc_rate) = ChargeLaw::BIASED.rates(&state);
        let ic = phase - state.reverse / state.charge_factor - state.reverse / BR - qbc_rate;
        let ib = state.forward / BF + state.reverse / BR + qbe_rate + qbc_rate;
        let expected = [ic, ib, -ic - ib];
        let intrinsic = [state.collector, state.base, 0.0];
        let resistance = [RC, RB, RE];
        let mut actual_intrinsic = [0.0; 3];
        for terminal in 0..3 {
            let current = -polarity * currents[terminal][index];
            error.current[terminal] =
                error.current[terminal].max((current - expected[terminal]).abs());
            error.peak[terminal] = error.peak[terminal].max(expected[terminal].abs());
            // Ohm's law reconstructs private voltages from public terminals;
            // it does not use the core's private-node or device helpers.
            actual_intrinsic[terminal] =
                polarity * voltages[terminal][index] - resistance[terminal] * current;
        }
        for terminal in 0..3 {
            let observed = if terminal == 2 {
                actual_intrinsic[terminal]
            } else {
                actual_intrinsic[terminal] - actual_intrinsic[2]
            };
            error.voltage[terminal] =
                error.voltage[terminal].max((observed - intrinsic[terminal]).abs());
        }
        let kcl = (currents[0][index] + currents[1][index] + currents[2][index]).abs();
        let scale: f64 = currents.iter().map(|lead| lead[index].abs()).sum();
        let roundoff =
            ChargeLaw::BIASED.companion_roundoff(&state, dialect, result.step_sizes[index]);
        assert!(
            kcl < 1e-14 + 1e-9 * scale + roundoff,
            "terminal KCL {kcl:e}"
        );
    }
    error
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[ignore = "C03c.2: high-injection private charge recovery still fails; required before qualification closes"]
fn gp_exact_delay_solves_biased_charge_through_private_terminal_resistances() {
    for dialect in [
        SpiceDialect::Ngspice,
        SpiceDialect::Xyce,
        SpiceDialect::BestAvailable,
    ] {
        for polarity in [1.0, -1.0] {
            for regime in REGIMES {
                let coarse = run_private(regime, polarity, dialect, 4e-11);
                let fine = run_private(regime, polarity, dialect, 4e-12);
                eprintln!(
                    "private nodes {}/{dialect:?}/{polarity}: coarse={coarse:?}, fine={fine:?}",
                    regime.name
                );
                for terminal in 0..3 {
                    assert!(
                        fine.current[terminal] < 1e-14 + 0.02 * fine.peak[terminal],
                        "{regime:?}/{dialect:?}/{polarity}/{terminal}: {fine:?}"
                    );
                    assert!(
                        fine.voltage[terminal] < 2e-4,
                        "{regime:?}/{dialect:?}/{polarity}/{terminal}: {fine:?}"
                    );
                    assert!(
                        fine.current[terminal] < 1e-14 + 0.7 * coarse.current[terminal],
                        "{regime:?}/{dialect:?}/{polarity}/{terminal}: coarse={coarse:?}, fine={fine:?}"
                    );
                    assert!(
                        fine.voltage[terminal] < 1e-12 + 0.7 * coarse.voltage[terminal],
                        "{regime:?}/{dialect:?}/{polarity}/{terminal}: coarse={coarse:?}, fine={fine:?}"
                    );
                }
            }
        }
    }
}
