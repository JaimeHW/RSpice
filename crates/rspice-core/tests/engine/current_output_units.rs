use rspice_core::abort_signal::NoAbort;
use rspice_core::analysis::measure_signals::{
    evaluate_tran_four_output_requests_with_abort, evaluate_tran_output_requests_with_abort,
};
use rspice_core::engine::Engine;
use rspice_core::execution::{SignalUnit, transient_fft_output_unit};
use rspice_core::netlist::{Netlist, SaveSignal, parse_save_probe};
use rspice_core::resource::ResourceLimits;

fn deck(device: &str, construction: &str, probes: &[&str]) -> Netlist {
    let mut source = format!(
        "* current unit provenance\nVc c 0 1\nVb b 0 .6\nVm marker:IC 0 2\n.model qm NPN(IS=1e-15 BF=100)\n{construction}\n.tran .0078125 1\n.print tran {}\n.four 1 {}\n",
        probes.join(" "),
        probes.join(" "),
    );
    for probe in probes {
        source.push_str(&format!(".fft {probe} np=64 format=unorm\n"));
    }
    source.push_str(&format!(
        ".print tran @{device}[gm] V(marker:IC) {{2*IC({device})}} N(marker:IC) {{N(marker:IC)}}\n.end\n"
    ));
    Netlist::parse(&source).unwrap()
}

fn assert_current_units(netlist: &Netlist, current_count: usize) {
    let result = Engine::new(Default::default())
        .run_tran_with_abort(netlist, 1.0, 1.0 / 128.0, &NoAbort)
        .unwrap();
    let columns = evaluate_tran_output_requests_with_abort(
        netlist,
        &result,
        ResourceLimits::default(),
        &NoAbort,
    )
    .unwrap();
    assert!(columns[0].2.iter().any(|value| *value != 0.0));
    for column in &columns[..current_count] {
        assert_eq!(column.1, "current", "{}", column.0);
        assert_eq!(column.2, columns[0].2, "{}", column.0);
    }
    // Undeclared expression dimensions and non-current device parameters do
    // not acquire amperes just because a current probe occurs nearby.
    assert_eq!(columns[current_count].1, "parameter");
    assert_eq!(columns[current_count + 1].1, "voltage");
    assert_eq!(columns[current_count + 2].1, "parameter");
    for column in &columns[current_count + 3..] {
        assert_eq!(column.1, "voltage", "{}", column.0);
        assert!(column.2.iter().all(|value| *value == 2.0));
    }
    let fourier = evaluate_tran_four_output_requests_with_abort(
        netlist,
        &result,
        0,
        ResourceLimits::default(),
        &NoAbort,
    )
    .unwrap();
    assert_eq!(fourier.len(), current_count);
    for column in fourier {
        assert_eq!(column.1, "current", "{}", column.0);
    }
    assert_eq!(result.fft_results.len(), current_count);
    for fft in &result.fft_results {
        assert_eq!(fft.physical_type, "current", "{}", fft.output_name);
        assert_eq!(
            transient_fft_output_unit(fft.physical_type, fft.format).unwrap(),
            SignalUnit::Ampere
        );
    }
}

#[test]
fn current_output_units_are_preserved_for_typed_device_probes() {
    let netlist = deck("Q1", "Q1 c b 0 qm", &["@Q1[ic]", "N(Q1:ic)", "IC(Q1)"]);
    assert_current_units(&netlist, 3);
}

#[test]
fn current_output_units_survive_hierarchy_and_braced_aliases() {
    let netlist = deck(
        "X1:Q1",
        ".subckt cell c b e\nQ1 c b e qm\n.ends cell\nX1 c b 0 cell",
        &[
            "@x1:q1[IC]",
            "N(X1:Q1:ic)",
            "IC(X1:Q1)",
            "{N(X1:Q1:ic)}",
            "{@X1:Q1[ic]}",
            "{IC(X1:Q1)}",
        ],
    );
    assert_current_units(&netlist, 6);
}

#[test]
fn current_output_units_parse_the_last_hierarchy_separator_as_the_parameter() {
    assert_eq!(
        parse_save_probe("N(X1:X2:Q1:ic)"),
        Some(SaveSignal::DeviceParam {
            device: "X1:X2:Q1".into(),
            param: "ic".into(),
        })
    );
}

#[test]
fn current_output_probes_preserve_builtins_and_reject_incomplete_parameters() {
    use rspice_core::netlist::expr::{eval_simple, parse_expression};

    assert_eq!(eval_simple("IF(1, INT(2.5), 9)").unwrap(), 2.0);
    for malformed in ["@Q1[ic", "@Q1[]", "@[ic]", "@X1:Q1[ic]]"] {
        assert!(parse_expression(malformed).is_err(), "{malformed}");
    }
    for malformed in ["@Q1[ic", "@Q1[]", "@[ic]"] {
        let source = format!("bad Fourier probe\nV1 1 0 1\n.four 1 {malformed}\n.end\n");
        assert!(Netlist::parse(&source).is_err(), "{malformed}");
    }
}
