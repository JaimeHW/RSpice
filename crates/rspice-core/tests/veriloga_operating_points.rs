//! The operating point a runtime-compiled Verilog-A instance reports: its
//! terminal currents and the module variables its source describes.
#![cfg(feature = "veriloga")]

use rspice_core::{Engine, Netlist};
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

static MODEL_SEQUENCE: AtomicU64 = AtomicU64::new(0);

fn write_model(name: &str, source: &str) -> String {
    let sequence = MODEL_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let mut path: PathBuf = std::env::temp_dir();
    path.push(format!(
        "rspice_module_op_{name}_{}_{sequence}.va",
        std::process::id()
    ));
    let mut file = std::fs::File::create(&path).expect("create model file");
    file.write_all(source.as_bytes()).expect("write model");
    path.display().to_string().replace('\\', "/")
}

/// A conductance between `a` and `b`, with two described outputs: `g` in
/// siemens, and `itot`, which an instance's multiplicity scales.
const CONDUCTANCE: &str = r#"`include "disciplines.vams"
module rmod(a, b);
  inout a, b;
  electrical a, b;
  (* desc = "Conductance", units = "S" *) real g;
  (* units = "A", multiplicity = "multiply" *) real itot;
  real hidden;
  analog begin
    g = 1.0e-3;
    hidden = 2.0 * g;
    I(a, b) <+ g * V(a, b);
    itot = g * V(a, b);
  end
endmodule
"#;

fn module_report(deck: &str, instance: &str) -> rspice_core::circuit::ModuleOpReport {
    let netlist = Netlist::parse_validated(deck).expect("deck parses");
    let (_, report) = Engine::default()
        .run_dc_op_with_report(&netlist)
        .expect("operating point solves");
    report
        .entries
        .into_iter()
        .find(|entry| entry.name.eq_ignore_ascii_case(instance))
        .unwrap_or_else(|| panic!("{instance} has no operating-point entry"))
        .module
        .expect("a Verilog-A instance is reported under its module's names")
}

#[test]
fn an_instance_reports_each_terminal_current_and_each_described_variable() {
    let model = write_model("conductance", CONDUCTANCE);
    // Both terminals sit on ground-referenced nets, one of them ground: each
    // still reports its own current.
    let report = module_report(
        &format!("* module op\nV1 in 0 DC 5\nX1 in 0 rmod m=2\n.va \"{model}\" rmod\n.op\n.end\n"),
        "X1",
    );
    assert_eq!(report.module, "rmod");
    let value = |name: &str| {
        report
            .quantity(name)
            .unwrap_or_else(|| panic!("{name} is reported: {report:?}"))
            .value
    };
    // Two in parallel: 2 x 5 V x 1 mS into `a`, out of `b`.
    assert!((value("ia") - 10.0e-3).abs() < 1e-12, "{}", value("ia"));
    assert!((value("ib") + 10.0e-3).abs() < 1e-12, "{}", value("ib"));
    assert_eq!(value("g"), 1.0e-3);
    assert!((value("itot") - 10.0e-3).abs() < 1e-12, "{}", value("itot"));
    let g = report.quantity("g").unwrap();
    assert_eq!((g.units.as_str(), g.description.as_str()), ("S", "Conductance"));
    assert_eq!(report.quantity("ia").unwrap().units, "A");
    assert!(
        report.quantity("hidden").is_none(),
        "a variable its source does not describe is not an output"
    );
}

/// `@x1[g]` reads a described variable as `@x1[ia]` reads a terminal
/// current, and a name the instance does not report is refused with the
/// names it does.
#[test]
fn a_described_variable_is_read_by_name_and_an_unknown_name_is_refused() {
    let model = write_model("conductance_named", CONDUCTANCE);
    let deck = |saves: &str| {
        format!(
            "* module op by name\nV1 in 0 DC 5\nX1 in 0 rmod m=2\n{saves}\n.va \"{model}\" rmod\n\
             .op\n.end\n"
        )
    };
    let netlist = Netlist::parse_validated(&deck(".save @x1[g] @x1[itot] @x1[ia]"))
        .expect("deck parses");
    let result = Engine::default()
        .run_dc_op(&netlist)
        .expect("a described variable is a quantity the instance reports");
    let observable = |name: &str| {
        result
            .try_dc_observable_named(name)
            .unwrap_or_else(|| panic!("{name} is published; have {:?}", result.dc_observables))
    };
    assert_eq!(observable("x1:g"), 1.0e-3);
    assert!((observable("x1:itot") - 10.0e-3).abs() < 1e-12);
    assert!((observable("x1:ia") - 10.0e-3).abs() < 1e-12);

    let netlist = Netlist::parse_validated(&deck(".save @x1[hidden]")).expect("deck parses");
    let error = Engine::default()
        .run_dc_op(&netlist)
        .expect_err("an undescribed variable is not reported");
    let rspice_core::SimulationError::UnsupportedCapability(refusal) = &error else {
        panic!("expected a typed refusal, got {error:?}");
    };
    let detail = refusal.detail.to_ascii_lowercase();
    for form in ["@x1[hidden]", "@x1[ia]", "@x1[ib]", "@x1[g]", "@x1[itot]"] {
        assert!(detail.contains(form), "{form}: {error}");
    }
}
