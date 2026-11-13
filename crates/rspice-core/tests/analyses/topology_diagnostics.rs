//! Singular-system attribution, floating-node detection, and the reference
//! node of a deck that names no node 0.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;
use std::collections::BTreeMap;

#[test]
fn voltage_source_loop_reports_singularity_guidance() {
    // Two ideal voltage sources forcing different values on one node pair:
    // the DC system is singular and the error must say more than "singular".
    let deck = "\
* conflicting sources
v1 a 0 dc 1
v2 a 0 dc 2
r1 a 0 1k
.op
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    if let Err(err) = engine.run_dc_op(&netlist) {
        let message = err.to_string().to_lowercase();
        assert!(
            message.contains("singular"),
            "error should describe the singularity: {message}"
        );
        assert!(
            message.contains("voltage source") || message.contains("constrain"),
            "error should point at the cause or the unconstrained unknown: {message}"
        );
    }
    // Some continuation ladders regularize conflicting sources into a
    // solvable compromise; that is a solver-policy outcome, not a diagnostics
    // regression, so only the error path is asserted.
}

#[test]
fn capacitor_only_node_still_builds_and_solves() {
    // The floating node is reported via a warning, not an error: the gmin
    // keeps the system solvable and existing decks must keep running.
    let deck = "\
* floating cap node
v1 in 0 dc 1
r1 in out 1k
c1 out float 1n
c2 float 0 1n
.op
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    engine
        .run_dc_op(&netlist)
        .expect("cap-only node must stay runnable (warned, not fatal)");
}

/// Every number the operating point and a transient report, by name, with
/// each value as its bits: node voltages and branch currents, the time axis,
/// and every digital event. The reference net, `0` or `ref`, is the one name
/// left out.
fn report_bits(deck: &str, tstop: f64, tstep: f64) -> Result<BTreeMap<String, String>, String> {
    let netlist = Netlist::parse(deck).map_err(|err| format!("parse: {err}"))?;
    let engine = Engine::new(SimulationConfig::default());
    let kept = |name: &str| !name.eq_ignore_ascii_case("0") && !name.eq_ignore_ascii_case("ref");
    let bits = |value: f64| format!("{:016x}", value.to_bits());
    let mut report = BTreeMap::new();
    let op = engine
        .run_dc_op(&netlist)
        .map_err(|err| format!("operating point: {err}"))?;
    for (name, value) in op.node_names.iter().zip(&op.node_voltages) {
        if kept(name) {
            report.insert(format!("op v({name})"), bits(*value));
        }
    }
    for (name, value) in op.branch_names.iter().zip(&op.branch_currents) {
        report.insert(format!("op i({name})"), bits(*value));
    }
    let tran = engine
        .run_tran(&netlist, tstop, tstep)
        .map_err(|err| format!("transient: {err}"))?;
    report.insert("tran points".to_owned(), tran.time.len().to_string());
    for (point, time) in tran.time.iter().enumerate() {
        report.insert(format!("tran[{point:04}] t"), bits(*time));
        for (name, series) in tran.node_names.iter().zip(&tran.voltages) {
            // An event-only net keeps no voltage column.
            if let Some(value) = series.get(point).filter(|_| kept(name)) {
                report.insert(format!("tran[{point:04}] v({name})"), bits(*value));
            }
        }
        for (name, series) in tran.branch_names.iter().zip(&tran.branch_currents) {
            if let Some(value) = series.get(point) {
                report.insert(format!("tran[{point:04}] i({name})"), bits(*value));
            }
        }
    }
    for trace in &tran.digital_traces {
        for (index, event) in trace.points.iter().enumerate() {
            report.insert(
                format!("tran d({})[{index:04}]", trace.node_name),
                format!("{} {:?}", bits(event.time), event.value),
            );
        }
    }
    Ok(report)
}

/// One deck run twice: once naming its reference `0`, once naming it `ref`,
/// which the build must then choose as the first voltage source's negative
/// terminal. Both must report the same numbers, bit for bit.
fn assert_reference_choice_is_invisible(label: &str, deck: &str, tstop: f64, tstep: f64) {
    let grounded = report_bits(&deck.replace("{ref}", "0"), tstop, tstep)
        .unwrap_or_else(|err| panic!("{label} with node 0: {err}"));
    let chosen = report_bits(&deck.replace("{ref}", "ref"), tstop, tstep)
        .unwrap_or_else(|err| panic!("{label} without node 0: {err}"));
    let differences: Vec<String> = grounded
        .keys()
        .chain(chosen.keys().filter(|key| !grounded.contains_key(*key)))
        .filter(|key| grounded.get(*key) != chosen.get(*key))
        .map(|key| {
            let show = |value: Option<&String>| {
                value.map(|value| match u64::from_str_radix(value, 16) {
                    Ok(bits) if value.len() == 16 => format!("{:e}", f64::from_bits(bits)),
                    _ => value.clone(),
                })
            };
            format!(
                "{key}: {:?} with node 0, {:?} without",
                show(grounded.get(key)),
                show(chosen.get(key))
            )
        })
        .collect();
    assert!(
        differences.is_empty(),
        "{label}: {} of {} observations differ without node 0:\n{}",
        differences.len(),
        grounded.len(),
        differences
            .iter()
            .take(12)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[test]
fn a_deck_without_node_zero_matches_it_for_a_plain_rc() {
    assert_reference_choice_is_invisible(
        "RC",
        "* rc\n\
         v1 in {ref} pulse(0 1 1n 1n 1n 5n 10n)\n\
         r1 in out 1k\n\
         c1 out {ref} 1p\n\
         .end\n",
        20.0e-9,
        0.1e-9,
    );
}

#[test]
fn a_deck_without_node_zero_matches_it_for_a_gummel_poon_bjt_with_internal_resistances() {
    assert_reference_choice_is_invisible(
        "Gummel-Poon",
        "* gummel-poon with internal resistances\n\
         vcc vcc {ref} 5\n\
         vb in {ref} pulse(0.6 0.9 1n 1n 1n 5n 10n)\n\
         rb in b 1k\n\
         rc vcc c 1k\n\
         q1 c b {ref} qgp\n\
         .model qgp npn (is=1e-15 bf=100 rb=100 rc=10 re=1 cje=1p cjc=0.5p tf=0.1n)\n\
         .end\n",
        20.0e-9,
        0.1e-9,
    );
}

#[test]
fn a_deck_without_node_zero_matches_it_for_a_vbic_bjt() {
    assert_reference_choice_is_invisible(
        "VBIC",
        "* vbic with internal resistances\n\
         vcc vcc {ref} 5\n\
         vb in {ref} pulse(0.6 0.9 1n 1n 1n 5n 10n)\n\
         rb in b 1k\n\
         rc vcc c 1k\n\
         q1 c b {ref} qv\n\
         .model qv npn (level=4 is=1e-14 ibei=1e-16 ibci=1e-16 rcx=10 rci=20 rbx=10 \
         rbi=40 re=1 cje=1p cjc=0.5p tf=0.1n)\n\
         .end\n",
        20.0e-9,
        0.1e-9,
    );
}

#[test]
fn a_deck_without_node_zero_matches_it_for_a_b3soi_mosfet() {
    assert_reference_choice_is_invisible(
        "B3SOI",
        "* b3soi with sheet resistance\n\
         vd d {ref} 5\n\
         vg g {ref} pulse(0 1.2 1n 1n 1n 5n 10n)\n\
         m1 d g {ref} {ref} nmos w=4u l=1u\n\
         .model nmos nmos level=10 soimod=0 rsh=10 capmod=2\n\
         .end\n",
        20.0e-9,
        0.1e-9,
    );
}

/// The bridges sense and drive against the reference net: a single-ended
/// D/A output would connect node 0, and a deck that calls its reference
/// `ref` would keep node 0 rather than take `ref`.
#[test]
fn a_deck_without_node_zero_matches_it_for_an_xspice_gate() {
    assert_reference_choice_is_invisible(
        "XSPICE",
        "* xspice bridge and buffer\n\
         vin in {ref} pulse(0 1 1n 0.1n 0.1n 4n 8n)\n\
         rin in {ref} 1k\n\
         aadc [%vd(in {ref})] [d] adc\n\
         .model adc adc_bridge (in_low=0.4 in_high=0.6)\n\
         abuf d e buf\n\
         .model buf d_buffer (rise_delay=1n fall_delay=1n input_load=1p)\n\
         adac [e] [%vd(out {ref})] dac\n\
         .model dac dac_bridge (out_low=0 out_high=1)\n\
         rout out {ref} 1k\n\
         .end\n",
        20.0e-9,
        0.1e-9,
    );
}

/// The node a deck's `ref` net became.
fn reference_node(deck: &str) -> Option<usize> {
    let netlist = Netlist::parse(deck).expect("deck parses");
    Engine::new(SimulationConfig::default())
        .build_circuit(&netlist)
        .expect("deck builds")
        .get_node_by_name("ref")
}

/// A code model's single-ended output drives its net against node 0, a
/// branch incident on it, so a deck that names no node 0 but holds one is
/// connected to it all the same: the build keeps node 0 as the reference and
/// `ref` stays a net.
#[test]
fn a_single_ended_code_model_drive_keeps_node_zero_as_the_reference() {
    for (label, card) in [
        ("analog gain", "again in out gain"),
        (
            "D/A output",
            "aadc [%vd(in ref)] [d] adc\nadac [d] [out] dac",
        ),
    ] {
        let deck = format!(
            "* {label}\n\
             vin in ref pulse(0 1 1n 0.1n 0.1n 4n 8n)\n\
             rin in ref 1k\n\
             {card}\n\
             .model adc adc_bridge (in_low=0.4 in_high=0.6)\n\
             .model dac dac_bridge (out_low=0 out_high=1)\n\
             .model gain gain (gain=2)\n\
             rout out ref 1k\n\
             .end\n"
        );
        assert!(
            reference_node(&deck).is_some_and(|node| node != 0),
            "{label}: `ref` became the reference"
        );
    }
}

/// A code model's voltage input, an A/D converter's among them, only senses
/// its net: it stamps nothing in node 0's row or column, as a single-ended
/// read in an equation stamps nothing there. A deck that names no node 0 and
/// reaches it only through such inputs would float as written, so it takes
/// its first voltage source's negative terminal as the reference, and runs
/// bit for bit as the same deck naming that net 0.
#[test]
fn a_deck_that_only_senses_node_zero_takes_the_automatic_reference() {
    for (label, cards) in [
        (
            "A/D input",
            "aadc [in] [d] adc\n\
             .model adc adc_bridge (in_low=0.4 in_high=0.6)\n\
             abuf d e buf\n\
             .model buf d_buffer (rise_delay=1n fall_delay=1n input_load=1p)\n",
        ),
        (
            "analog gain input",
            "again in %vd(out {ref}) gain\n\
             .model gain gain (gain=2)\n\
             rout out {ref} 1k\n",
        ),
    ] {
        let deck = format!(
            "* {label}\n\
             vin in {{ref}} pulse(0 1 1n 0.1n 0.1n 4n 8n)\n\
             rin in {{ref}} 1k\n\
             {cards}\
             .end\n"
        );
        assert_eq!(
            reference_node(&deck.replace("{ref}", "ref")),
            Some(0),
            "{label}: the input only senses node 0, so `ref` is the reference"
        );
        assert_reference_choice_is_invisible(label, &deck, 20.0e-9, 0.1e-9);
    }
}
