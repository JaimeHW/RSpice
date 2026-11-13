//! Xyce DIG input decode at the operating point, through the engine.
//!
//! Xyce's `N_DEV_Digital.C` seeds every DCOP input state from `S0VHI` alone,
//! and its two-threshold hysteresis step cannot move that seed.  An input
//! parked between the thresholds therefore starts LOW when `S0VHI` is the
//! upper threshold (the shipped card) and HIGH when the card puts `S0VHI`
//! below `S1VLO`.  These decks hold that parity at the engine boundary for a
//! combinational gate and for the T flip-flop, so the model-level rule in
//! `xspice/models/digital_output.rs` cannot drift from what a netlist sees.

use rspice_core::engine::Engine;
use rspice_core::netlist::Netlist;

/// `(s0vhi, s1vlo)` and the level a 1.2 V input decodes to at DCOP.
const IN_BAND_DECODES: [((f64, f64), bool); 2] = [((1.7, 0.9), false), ((0.8, 2.0), true)];

/// Corpus-style output network: 5 Ω to the driven rail and 200 Ω to the other,
/// into a 100 kΩ load.  LOW settles near 0.07 V and HIGH near 2.93 V.
fn dig_card(model: &str, s0vhi: f64, s1vlo: f64) -> String {
    format!(
        ".model dmod {model} (clo=1p chi=1p cload=1p rload=1k \
         s0rlo=5 s0rhi=200 s0tsw=5n s0vlo=-1 s0vhi={s0vhi} \
         s1rlo=200 s1rhi=5 s1tsw=5n s1vlo={s1vlo} s1vhi=3 delay=20n)"
    )
}

fn op_voltage(deck: &str, node: &str) -> f64 {
    let netlist = Netlist::parse(deck).unwrap_or_else(|err| panic!("deck parses: {err}"));
    let op = Engine::default()
        .run_dc_op(&netlist)
        .expect("operating point solves");
    op.try_voltage_named(node)
        .unwrap_or_else(|| panic!("node {node} missing from {:?}", op.node_names))
}

fn logic_level(voltage: f64) -> bool {
    assert!(
        !(0.5..=2.5).contains(&voltage),
        "output at {voltage} V is not a settled DIG level"
    );
    voltage > 2.5
}

#[test]
fn buffer_dcop_input_between_thresholds_seeds_from_s0vhi() {
    for ((s0vhi, s1vlo), expected) in IN_BAND_DECODES {
        let deck = format!(
            "* Xyce DIG buffer: DCOP decode of an in-band input\n\
             vdpwr dpwr 0 3\n\
             vin in 0 1.2\n\
             abuf dpwr 0 [in] out dmod\n\
             rload out 0 100k\n\
             {}\n\
             .op\n\
             .end\n",
            dig_card("xyce_d_buffer", s0vhi, s1vlo)
        );
        assert_eq!(
            logic_level(op_voltage(&deck, "out")),
            expected,
            "buffer output with s0vhi={s0vhi} s1vlo={s1vlo}"
        );
    }
}

#[test]
fn tff_dcop_toggle_between_thresholds_seeds_q_from_s0vhi() {
    for ((s0vhi, s1vlo), expected) in IN_BAND_DECODES {
        let deck = format!(
            "* Xyce DIG TFF: DCOP decode of an in-band toggle input\n\
             vdpwr dpwr 0 3\n\
             vt t 0 1.2\n\
             vclk clk 0 0\n\
             atff dpwr 0 t clk q qbar dmod\n\
             rq q 0 100k\n\
             rqbar qbar 0 100k\n\
             {}\n\
             .op\n\
             .end\n",
            dig_card("xyce_d_tff", s0vhi, s1vlo)
        );
        assert_eq!(
            logic_level(op_voltage(&deck, "q")),
            expected,
            "Q with s0vhi={s0vhi} s1vlo={s1vlo}"
        );
        assert_eq!(
            logic_level(op_voltage(&deck, "qbar")),
            !expected,
            "QBAR with s0vhi={s0vhi} s1vlo={s1vlo}"
        );
    }
}
