//! Independent GP transport/charge laws under prescribed terminal voltages.
//!
//! The forward phase operator acts on I_BE / Q_B, including Early effect and
//! both high-injection terms. Reverse transport remains instantaneous. These
//! oracles use algebra and the published ngspice Weil recurrence, never core
//! device, charge, phase or interpolation helpers.
//!
//! Local equation reference: ngspice-46 src/spicelib/devices/bjt/bjtload.c,
//! base charge, excess phase and forward diffusion charge sections. The exact
//! delay oracle evaluates the prescribed input at t - delay analytically;
//! ngspice's discrete recurrence is a separate phase law.

use rspice_core::engine::{TransientCheckpoint, TransientCheckpointEncoding, TransientResult};
use rspice_core::numerics::integration::IntegrationMethod;
use rspice_core::{Engine, GpTransientPhaseModel, Netlist, SimulationConfig, SpiceDialect};

#[path = "gp_phase_regimes/behavioral.rs"]
mod behavioral;
#[path = "gp_phase_regimes/charge.rs"]
mod charge;
#[path = "gp_phase_regimes/capacitor_ic.rs"]
mod capacitor_ic;
#[path = "gp_phase_regimes/controlled.rs"]
mod controlled;
#[path = "gp_phase_regimes/coverage_gaps.rs"]
mod coverage_gaps;
#[path = "gp_phase_regimes/current_controlled.rs"]
mod current_controlled;
#[path = "gp_phase_regimes/magnetic.rs"]
mod magnetic;
#[path = "gp_phase_regimes/diodes.rs"]
mod diodes;
#[path = "gp_phase_regimes/nodal_voltage.rs"]
mod nodal_voltage;
#[path = "gp_phase_regimes/private_nodes.rs"]
mod private_nodes;
#[path = "gp_phase_regimes/static_private.rs"]
mod static_private;
use charge::ChargeLaw;

const IS: f64 = 1e-16;
const BF: f64 = 80.0;
const BR: f64 = 5.0;
const IKF: f64 = 1e-3;
const IKR: f64 = 2e-4;
const VAF: f64 = 15.0;
const VAR: f64 = 12.0;
const TF: f64 = 1e-9;
const DELAY: f64 = 1e-9;
const STOP: f64 = 5.37e-9;

#[derive(Clone, Copy, Debug)]
struct Regime {
    name: &'static str,
    base: f64,
    base_amplitude: f64,
    collector: f64,
    collector_amplitude: f64,
}

const REGIMES: [Regime; 3] = [
    Regime {
        name: "saturation",
        base: 0.70,
        base_amplitude: 0.015,
        collector: 0.02,
        collector_amplitude: 0.01,
    },
    Regime {
        name: "reverse_active",
        base: 0.04,
        base_amplitude: 0.01,
        collector: -0.72,
        collector_amplitude: 0.015,
    },
    Regime {
        name: "high_injection",
        base: 0.84,
        base_amplitude: 0.006,
        collector: 2.0,
        collector_amplitude: 0.1,
    },
];

struct State {
    base: f64,
    collector: f64,
    base_rate: f64,
    collector_rate: f64,
    forward: f64,
    reverse: f64,
    forward_rate: f64,
    reverse_rate: f64,
    charge_factor: f64,
    forward_transport: f64,
    forward_transport_rate: f64,
}

fn thermal_voltage(dialect: SpiceDialect) -> f64 {
    match dialect {
        SpiceDialect::Ngspice => 1.380_648_52e-23 * 300.15 / 1.602_176_620_8e-19,
        SpiceDialect::Xyce => 1.380_622_6e-23 * 300.15 / 1.602_191_8e-19,
        SpiceDialect::BestAvailable => 1.380_649e-23 * 300.15 / 1.602_176_634e-19,
    }
}

/// Value and derivative with respect to junction voltage. Ngspice's reverse
/// continuation is cubic below -3 VT; Xyce retains the diode exponential.
fn diode(voltage: f64, vt: f64, dialect: SpiceDialect) -> (f64, f64) {
    if dialect != SpiceDialect::Xyce && voltage < -3.0 * vt {
        let a = 3.0 * vt / std::f64::consts::E;
        (
            -IS * (1.0 + (a / voltage).powi(3)),
            3.0 * IS * a.powi(3) / voltage.powi(4),
        )
    } else {
        (IS * (voltage / vt).exp_m1(), IS * (voltage / vt).exp() / vt)
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_clamped_uic_preserves_tight_charge_tolerance() {
    use rspice_core::engine::TransientStartupMode;
    for dialect in [
        SpiceDialect::Ngspice,
        SpiceDialect::Xyce,
        SpiceDialect::BestAvailable,
    ] {
        for polarity in [1.0, -1.0] {
            let kind = if polarity > 0.0 { "NPN" } else { "PNP" };
            let deck = Netlist::parse(&format!(
                "clamped GP UIC\nVC c 0 {}\nVB b 0 DC {} SIN({} {} 1G)\nQ1 c b 0 qm\n.model qm {kind}(IS=1e-16 BF=100 BR=1 TF=1n PTF=57.29577951308232)\n.options GMIN=0 RELTOL=1e-7 ABSTOL=1e-16 VNTOL=1e-10 CHGTOL=1e-26\n.save v(b) i(vc) i(vb)\n.end\n",
                2.0 * polarity, 0.7 * polarity, 0.7 * polarity, 1e-6 * polarity,
            )).unwrap();
            let mut config = SimulationConfig {
                gp_transient_phase_model: GpTransientPhaseModel::ExactDelay,
                ..SimulationConfig::default().with_spice_dialect(dialect)
            };
            config.convergence_config.gmin_target = 0.0;
            let engine = Engine::new(config);
            let result = engine
                .run_tran_with_startup_mode(&deck, 2.5e-9, 4e-12, TransientStartupMode::Uic)
                .unwrap_or_else(|error| panic!("{dialect:?}/{polarity}: {error}"));
            behavioral::check(&result, dialect, polarity, None, "vb", true);
            let impulse = result
                .current_impulses
                .as_ref()
                .unwrap()
                .iter()
                .find(|trace| {
                    matches!(&trace.owner, rspice_core::CurrentImpulseOwner::Branch { branch_name }
                    if branch_name.eq_ignore_ascii_case("vb"))
                })
                .unwrap();
            let startup = impulse
                .points
                .iter()
                .find(|point| point.time == 0.0)
                .unwrap();
            let expected = -polarity * TF * diode(0.7, thermal_voltage(dialect), dialect).0;
            assert!((startup.charge_coulombs - expected).abs() < 1e-26 + 1e-10 * expected.abs());
            assert_eq!(engine.convergence_quality().force_accepted_points, 0);
        }
    }
}

impl Regime {
    fn waveform(bias: f64, amplitude: f64, frequency: f64, time: f64) -> (f64, f64) {
        let omega = std::f64::consts::TAU * frequency;
        let theta = omega * time.max(0.0);
        (
            bias + amplitude * (1.0 - theta.cos()),
            amplitude * omega * theta.sin(),
        )
    }

    fn state(self, time: f64, dialect: SpiceDialect) -> State {
        let (base, db) = Self::waveform(self.base, self.base_amplitude, 5e8, time);
        let (collector, dc) = Self::waveform(self.collector, self.collector_amplitude, 7e8, time);
        let (forward, gf) = diode(base, thermal_voltage(dialect), dialect);
        let (reverse, gr) = diode(base - collector, thermal_voltage(dialect), dialect);
        let df = gf * db;
        let dr = gr * (db - dc);
        let q1 = 1.0 / (1.0 - (base - collector) / VAF - base / VAR);
        let dq1 = q1 * q1 * ((db - dc) / VAF + db / VAR);
        let root = (1.0 + 4.0 * (forward / IKF + reverse / IKR)).sqrt();
        let qb = 0.5 * q1 * (1.0 + root);
        let dqb = 0.5 * dq1 * (1.0 + root) + q1 * (df / IKF + dr / IKR) / root;
        State {
            base,
            collector,
            base_rate: db,
            collector_rate: dc,
            forward,
            reverse,
            forward_rate: df,
            reverse_rate: dr,
            charge_factor: qb,
            forward_transport: forward / qb,
            forward_transport_rate: (df * qb - forward * dqb) / (qb * qb),
        }
    }

    fn deck(self, polarity: f64, charge: ChargeLaw) -> Netlist {
        let kind = if polarity > 0.0 { "NPN" } else { "PNP" };
        let charge_parameters = charge.parameters();
        Netlist::parse(&format!(
            "Independent GP {}\nVC c 0 DC {} SIN({} {} 700MEG 0 0 90)\nVB b 0 DC {} SIN({} {} 500MEG 0 0 90)\nVE e 0 0\nQ1 c b e qm\n.model qm {kind} IS={IS} BF={BF} BR={BR} IKF={IKF} IKR={IKR} VAF={VAF} VAR={VAR} TF={TF} PTF=57.29577951308232 TNOM=27 {charge_parameters}\n.options TEMP=27 GMIN=0 RELTOL=.01 ABSTOL=1e-15 VNTOL=1e-10 METHOD=TRAP MAXORD=2\n.end\n",
            self.name,
            polarity * self.collector,
            polarity * (self.collector + self.collector_amplitude),
            -polarity * self.collector_amplitude,
            polarity * self.base,
            polarity * (self.base + self.base_amplitude),
            -polarity * self.base_amplitude,
        )).unwrap()
    }
}

#[derive(Debug)]
struct Error {
    collector: f64,
    base: f64,
    collector_peak: f64,
    base_peak: f64,
    kcl: f64,
    kcl_budget_fraction: f64,
}

fn compare(
    result: &TransientResult,
    regime: Regime,
    polarity: f64,
    dialect: SpiceDialect,
    model: GpTransientPhaseModel,
    charge: ChargeLaw,
) -> Error {
    let collector = result.try_branch_current_waveform_named("vc").unwrap();
    let base = result.try_branch_current_waveform_named("vb").unwrap();
    let emitter = result.try_branch_current_waveform_named("ve").unwrap();
    let mut error = Error {
        collector: 0.0,
        base: 0.0,
        collector_peak: 0.0,
        base_peak: 0.0,
        kcl: 0.0,
        kcl_budget_fraction: 0.0,
    };
    let mut previous = regime.state(0.0, dialect).forward_transport;
    let mut older = previous;
    let mut previous_step = 0.0;
    assert_eq!(result.time.last(), Some(&STOP));
    for (index, &time) in result.time.iter().enumerate() {
        let state = regime.state(time, dialect);
        let delayed = if model == GpTransientPhaseModel::ExactDelay {
            regime
                .state((time - DELAY).max(0.0), dialect)
                .forward_transport
        } else if index == 0 {
            previous
        } else {
            // ngspice-46 bjtload.c: the recurrence acts on the complete
            // nonlinear I_BE/Q_B input, on this actual accepted interval.
            let step = result.step_sizes[index];
            let ratio = if previous_step == 0.0 {
                1.0
            } else {
                step / previous_step
            };
            let a = step / DELAY;
            let next = (previous * (1.0 + ratio + 3.0 * a) - older * ratio
                + 3.0 * a * a * state.forward_transport)
                / (1.0 + 3.0 * a + 3.0 * a * a);
            older = previous;
            previous = next;
            previous_step = step;
            next
        };
        // Depletion capacitances are zero. Forward diffusion charge resides
        // between B and E; reverse transit charge resides between B and C.
        let (forward_rate, reverse_rate) = charge.rates(&state);
        let expected_collector = -polarity
            * (delayed - state.reverse / state.charge_factor - state.reverse / BR - reverse_rate);
        let expected_base =
            -polarity * (state.forward / BF + state.reverse / BR + forward_rate + reverse_rate);
        error.collector = error
            .collector
            .max((collector[index] - expected_collector).abs());
        error.base = error.base.max((base[index] - expected_base).abs());
        error.collector_peak = error.collector_peak.max(expected_collector.abs());
        error.base_peak = error.base_peak.max(expected_base.abs());
        let scale = collector[index].abs() + base[index].abs() + emitter[index].abs();
        let roundoff = charge.companion_roundoff(&state, dialect, result.step_sizes[index]);
        let kcl = (collector[index] + base[index] + emitter[index]).abs();
        let kcl_budget = 1e-14 + 1e-9 * scale + roundoff;
        error.kcl = error.kcl.max(kcl);
        error.kcl_budget_fraction = error.kcl_budget_fraction.max(kcl / kcl_budget);
        assert!(
            kcl < kcl_budget,
            "{regime:?}/{dialect:?}/{model:?}/polarity={polarity}/t={time}/dt={}: terminal KCL {}, {}, {}; expected C={expected_collector}, B={expected_base}",
            result.step_sizes[index],
            collector[index],
            base[index],
            emitter[index]
        );
    }
    error
}

fn run(
    regime: Regime,
    polarity: f64,
    dialect: SpiceDialect,
    model: GpTransientPhaseModel,
    step: f64,
) -> Error {
    run_with_charge(
        regime,
        polarity,
        dialect,
        model,
        step,
        ChargeLaw::UNMODULATED,
    )
}

fn run_with_charge(
    regime: Regime,
    polarity: f64,
    dialect: SpiceDialect,
    model: GpTransientPhaseModel,
    step: f64,
    charge: ChargeLaw,
) -> Error {
    let mut config = SimulationConfig {
        gp_transient_phase_model: model,
        integration_method: IntegrationMethod::Trapezoidal,
        ..SimulationConfig::default().with_spice_dialect(dialect)
    };
    config.convergence_config.gmin_target = 0.0;
    let result = Engine::new(config)
        .run_tran(&regime.deck(polarity, charge), STOP, step)
        .unwrap_or_else(|error| panic!("{regime:?}/{dialect:?}/{model:?}/{polarity}: {error}"));
    compare(&result, regime, polarity, dialect, model, charge)
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_exact_phase_tracks_nonlinear_transport_and_instantaneous_reverse_current() {
    for dialect in [
        SpiceDialect::Ngspice,
        SpiceDialect::Xyce,
        SpiceDialect::BestAvailable,
    ] {
        for polarity in [1.0, -1.0] {
            for regime in REGIMES {
                let coarse = run(
                    regime,
                    polarity,
                    dialect,
                    GpTransientPhaseModel::ExactDelay,
                    4e-11,
                );
                let fine = run(
                    regime,
                    polarity,
                    dialect,
                    GpTransientPhaseModel::ExactDelay,
                    4e-12,
                );
                eprintln!(
                    "{}/{dialect:?}/{polarity}: coarse={coarse:?}, fine={fine:?}",
                    regime.name
                );
                assert!(
                    fine.collector < 1e-14 + 4e-4 * fine.collector_peak,
                    "{regime:?}/{dialect:?}/{polarity}: {fine:?}"
                );
                // Include the first-order startup/restart charge-current
                // samples. Their full-waveform error must stay within twice
                // the requested 1% relative tolerance and refine with dt.
                assert!(
                    fine.base < 1e-14 + 0.02 * fine.base_peak,
                    "{regime:?}/{dialect:?}/{polarity}: {fine:?}"
                );
                // In this reverse-active case forward charge/transport is
                // negligible; both errors are already at roundoff and need
                // not improve on a finer grid.
                if regime.name != "reverse_active" {
                    assert!(fine.collector < 0.7 * coarse.collector);
                    assert!(fine.base < 0.7 * coarse.base);
                }
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_weil_phase_tracks_the_nonlinear_recurrence_across_operating_regimes() {
    for dialect in [
        SpiceDialect::Ngspice,
        SpiceDialect::Xyce,
        SpiceDialect::BestAvailable,
    ] {
        for polarity in [1.0, -1.0] {
            for regime in REGIMES {
                let error = run(
                    regime,
                    polarity,
                    dialect,
                    GpTransientPhaseModel::NgspiceWeil,
                    4e-12,
                );
                eprintln!("{}/{dialect:?}/{polarity}: {error:?}", regime.name);
                assert!(
                    error.collector < 1e-14 + 1e-8 * error.collector_peak,
                    "{regime:?}/{dialect:?}/{polarity}: {error:?}"
                );
                assert!(
                    error.base < 1e-14 + 0.02 * error.base_peak,
                    "{regime:?}/{dialect:?}/{polarity}: {error:?}"
                );
            }
        }
    }
}

fn exact_restart(
    engine: &Engine,
    deck: &Netlist,
    original: &TransientResult,
    checkpoint: &TransientCheckpoint,
    stop: f64,
    step: f64,
) {
    let checkpoint = TransientCheckpoint::from_bytes(
        &checkpoint
            .to_bytes(TransientCheckpointEncoding::Packed)
            .unwrap(),
    )
    .unwrap();
    let (resumed, _) = engine
        .run_tran_resume(deck, &checkpoint, stop, step)
        .unwrap();
    let seam = original
        .time
        .iter()
        .position(|time| *time == resumed.time[0])
        .unwrap();
    assert_eq!(resumed.time, original.time[seam..]);
    for (actual, expected) in resumed
        .voltages
        .iter()
        .chain(&resumed.branch_currents)
        .zip(original.voltages.iter().chain(&original.branch_currents))
    {
        if expected.is_empty() {
            assert!(actual.is_empty());
        } else {
            assert_eq!(actual.len(), expected.len() - seam);
            for (index, (&actual, &expected)) in actual.iter().zip(&expected[seam..]).enumerate() {
                assert_eq!(
                    actual.to_bits(),
                    expected.to_bits(),
                    "restart sample {index} at {:e}: {actual:e} != {expected:e}",
                    resumed.time[index]
                );
            }
        }
    }
}
