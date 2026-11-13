//! Switch thermal noise: a switch is a resistor, and `.NOISE` adds `4kT·G` of
//! its present conductance across its terminals, as ngspice does for its `S`
//! and `W` devices (`src/spicelib/devices/sw/swnoise.c`, `csw/cswnoise.c`).
//!
//! The reference numbers are ngspice-46 output (`print onoise_*` with
//! `numdgt=15`, in V/√Hz) for a 1 kΩ source resistor into a switch to ground,
//! observed at the switch node with the source shorted. An OFF switch is the
//! switch deck itself. An ON switch is its held-resistor twin, the switch
//! replaced by a 10 Ω resistor: ngspice-46 evaluates every switch at its OFF
//! conductance in `.AC` and `.NOISE` after an operating point, because its
//! small-signal load reads the switch state from `CKTstate1`, which a DC
//! solve never writes (`swload.c`, `cswload.c` under `MODEINITSMSIG`). The
//! same deck run through ngspice-46 reports `v(2) = 0.999` in `.AC` with the
//! switch ON at the operating point.

use rspice_core::analysis::NoiseResult;
use rspice_core::engine::{Engine, SimulationConfig, SpiceDialect};
use rspice_core::netlist::Netlist;

/// ngspice-46, OFF switch (`ron=10 roff=1meg`): total and switch onoise.
const OFF_TOTAL: f64 = 4.069_337_369_215_718e-9;
const OFF_SWITCH: f64 = 1.286_194_528_865_469e-10;
/// ngspice-46, the ON switch's 10 Ω held twin: total and twin-resistor onoise.
const ON_TOTAL: f64 = 4.051_166_087_002_019e-10;
const ON_SWITCH: f64 = 4.031_060_920_284_485e-10;

fn voltage_switch_deck(control: f64) -> String {
    format!(
        "voltage-controlled switch noise\n\
         v1 1 0 dc 0 ac 1\n\
         r1 1 2 1k\n\
         s1 2 0 c 0 sw1\n\
         vc c 0 dc {control}\n\
         .model sw1 sw vt=1 vh=0.2 ron=10 roff=1meg\n\
         .end\n"
    )
}

fn current_switch_deck(control: f64) -> String {
    format!(
        "current-controlled switch noise\n\
         v1 1 0 dc 0 ac 1\n\
         r1 1 2 1k\n\
         w1 2 0 vsense csw1\n\
         isense 0 x dc {control}\n\
         vsense x 0 dc 0\n\
         .model csw1 csw it=1m ih=0.2m ron=10 roff=1meg\n\
         .end\n"
    )
}

/// The Xyce spelling of the same switch: a `VSWITCH` card, which the Xyce
/// dialect evaluates as its expression-controlled `S` device.
fn xyce_switch_deck(control: f64) -> String {
    format!(
        "xyce switch noise\n\
         v1 1 0 dc 0 ac 1\n\
         r1 1 2 1k\n\
         s1 2 0 c 0 sw1\n\
         vc c 0 dc {control}\n\
         .model sw1 vswitch (von=1 voff=0 ron=10 roff=1e6)\n\
         .end\n"
    )
}

fn noise_at_1khz(deck: &str, dialect: SpiceDialect) -> NoiseResult {
    let netlist = Netlist::parse(deck).expect("the switch noise deck parses");
    Engine::new(SimulationConfig {
        spice_dialect: dialect,
        ..Default::default()
    })
    .run_noise_named_with_input_source(&netlist, "2", None, "v1", &[1.0e3], 300.15)
    .expect("the switch noise analysis runs")
    .into_iter()
    .next()
    .expect("one frequency point")
}

fn assert_close(label: &str, actual: f64, expected: f64) {
    let relative = (actual - expected).abs() / expected;
    assert!(
        relative <= 1.0e-4,
        "{label}: RSpice {actual:.9e} against the reference {expected:.9e} (rel {relative:.3e})"
    );
}

fn switch_contribution(result: &NoiseResult, switch: &str) -> f64 {
    let matching: Vec<_> = result
        .contributions
        .iter()
        .filter(|contribution| contribution.identity.device.eq_ignore_ascii_case(switch))
        .collect();
    assert_eq!(
        matching.len(),
        1,
        "switch '{switch}' contributes exactly one thermal source: {:?}",
        result.contributions
    );
    assert!(
        matching[0].identity.mechanism.is_none(),
        "a switch's single source is the device itself, as ngspice's onoise_{switch} is"
    );
    matching[0].output_contribution
}

#[test]
fn switch_thermal_noise_matches_ngspice46_in_each_state() {
    for dialect in [SpiceDialect::Ngspice, SpiceDialect::BestAvailable] {
        for (label, deck, switch, total, own) in [
            (
                "S off",
                voltage_switch_deck(0.0),
                "s1",
                OFF_TOTAL,
                OFF_SWITCH,
            ),
            ("S on", voltage_switch_deck(2.0), "s1", ON_TOTAL, ON_SWITCH),
            (
                "W off",
                current_switch_deck(-50.0e-3),
                "w1",
                OFF_TOTAL,
                OFF_SWITCH,
            ),
            (
                "W on",
                current_switch_deck(50.0e-3),
                "w1",
                ON_TOTAL,
                ON_SWITCH,
            ),
        ] {
            let label = format!("{dialect:?} {label}");
            let result = noise_at_1khz(&deck, dialect);
            assert_close(
                &format!("{label} switch onoise (V^2/Hz)"),
                switch_contribution(&result, switch),
                own * own,
            );
            assert_close(
                &format!("{label} total onoise (V^2/Hz)"),
                result.output_noise_density,
                total * total,
            );
        }
    }
}

/// Xyce gives its switch no noise source at all (`N_DEV_SW.C` declares no
/// `getNumNoiseSources`). A resistance without Johnson noise is not physical,
/// so the Xyce dialect's switch carries the same `4kT·G` the ngspice twin
/// measures, as the native MOS6 flicker law extends ngspice.
#[test]
fn xyce_switch_carries_the_thermal_noise_of_its_present_conductance() {
    for (label, control, total, own) in [
        ("off", 0.0, OFF_TOTAL, OFF_SWITCH),
        ("on", 2.0, ON_TOTAL, ON_SWITCH),
    ] {
        let result = noise_at_1khz(&xyce_switch_deck(control), SpiceDialect::Xyce);
        // Xyce's own Boltzmann constant differs from ngspice's by 1.9e-5.
        assert_close(
            &format!("Xyce {label} switch onoise (V^2/Hz)"),
            switch_contribution(&result, "s1"),
            own * own,
        );
        assert_close(
            &format!("Xyce {label} total onoise (V^2/Hz)"),
            result.output_noise_density,
            total * total,
        );
    }
}
