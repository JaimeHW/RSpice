//! `.save` / `.probe` / `.print` / `.plot` output-selection parsing and
//! matching semantics.

use rspice_core::netlist::{Netlist, SaveSignal};

fn parse(deck: &str) -> Netlist {
    Netlist::parse(deck).expect("deck parses")
}

#[test]
fn save_directive_parses_probe_forms() {
    let netlist = parse(
        "\
* save forms
v1 in 0 dc 1
r1 in out 1k
r2 out 0 1k
.save v(out) i(v1) all
.op
.end
",
    );

    assert!(netlist.saves.signals.contains(&SaveSignal::Voltage("out".into())));
    assert!(netlist.saves.signals.contains(&SaveSignal::Current("v1".into())));
    assert!(netlist.saves.signals.contains(&SaveSignal::All));
}

#[test]
fn print_directive_skips_analysis_type_and_collects_probes() {
    let netlist = parse(
        "\
* print form
v1 in 0 dc 1
r1 in out 1k
r2 out 0 1k
.print tran v(out) v(in,out)
.tran 1n 10n
.end
",
    );

    assert!(netlist.saves.signals.contains(&SaveSignal::Voltage("out".into())));
    assert!(
        netlist
            .saves
            .signals
            .contains(&SaveSignal::VoltageDiff("in".into(), "out".into()))
    );
    // The leading analysis keyword must not be treated as a probe.
    assert!(!netlist.saves.signals.iter().any(
        |signal| matches!(signal, SaveSignal::Raw(name) if name.eq_ignore_ascii_case("tran"))
    ));
}

#[test]
fn device_param_probe_parses() {
    let netlist = parse(
        "\
* device param probe
v1 in 0 dc 1
r1 in 0 1k
.save @r1[i]
.op
.end
",
    );

    assert!(netlist.saves.signals.iter().any(|signal| matches!(
        signal,
        SaveSignal::DeviceParam { device, param }
            if device.eq_ignore_ascii_case("r1") && param.eq_ignore_ascii_case("i")
    )));
}

#[test]
fn selection_matches_raw_variable_conventions() {
    let netlist = parse(
        "\
* selection semantics
v1 in 0 dc 1
r1 in out 1k
r2 out 0 1k
.save v(out) i(v1)
.op
.end
",
    );
    let saves = &netlist.saves;

    // Selected signals, any case, wrapped or branch-suffixed.
    assert!(saves.selects("v(out)"));
    assert!(saves.selects("V(OUT)"));
    assert!(saves.selects("i(v1)"));
    assert!(saves.selects("I(V1)"));
    assert!(saves.selects("v1#branch"));

    // Scale vectors always survive.
    assert!(saves.selects("time"));
    assert!(saves.selects("frequency"));

    // Unselected node is filtered.
    assert!(!saves.selects("v(in)"));
    assert!(!saves.selects("V(IN)"));
}

#[test]
fn empty_save_set_keeps_everything() {
    let netlist = parse(
        "\
* no save card
v1 in 0 dc 1
r1 in 0 1k
.op
.end
",
    );
    assert!(netlist.saves.keeps_everything());
    assert!(netlist.saves.selects("v(anything)"));
}

#[test]
fn bare_names_are_voltage_shorthand() {
    let netlist = parse(
        "\
* bare-name save
v1 in 0 dc 1
r1 in out 1k
r2 out 0 1k
.save out
.op
.end
",
    );
    assert!(netlist.saves.selects("v(out)"));
    assert!(!netlist.saves.selects("v(in)"));
}
