//! Authored-card accounting and parser-boundary contracts for PSpice
//! CHEBYSHEV controlled sources.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::{ElementKind, Netlist};
use std::path::{Path, PathBuf};

struct TempDirectory(PathBuf);

impl TempDirectory {
    fn new(label: &str) -> Self {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock follows Unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "rspice-chebyshev-{label}-{}-{nonce}",
            std::process::id()
        ));
        std::fs::create_dir(&path).expect("create isolated CHEBYSHEV test directory");
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDirectory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn authored_count_excludes_comments_inactive_branches_and_post_end_text() {
    let source = "CHEBYSHEV metadata\n\
        * ECOMMENT out 0 CHEBYSHEV {V(in)} = LP (800Hz 1.2kHz) .1dB 50dB\n\
        .if 0\n\
        EINACTIVE out 0 CHEBYSHEV {V(in)} = LP (800Hz 1.2kHz) .1dB 50dB\n\
        .endif\n\
        .subckt FILTER in out\n\
        ESUB out 0 CHEBYSHEV {V(in)} = LP 800 Hz 1.2k Hz .1 dB 50 dB\n\
        .ends FILTER\n\
        GTOP 0 gout CHEBYSHEV {V(in)} = HP 1.2kHz 800Hz .1dB 50dB M=2\n\
        .end\n\
        EAFTER out 0 CHEBYSHEV {V(in)} = LP (800Hz 1.2kHz) .1dB 50dB\n";

    let netlist = Netlist::parse(source).expect("active CHEBYSHEV cards parse");
    assert_eq!(netlist.pspice_chebyshev_source_count(), 2);

    let terminal = netlist
        .elements
        .iter()
        .find(|element| element.name.eq_ignore_ascii_case("GTOP"))
        .expect("G-source terminal realization exists");
    match &terminal.kind {
        ElementKind::BehavioralCurrent { multiplicity, .. } => {
            assert!(multiplicity.given);
            assert_eq!(multiplicity.value, 2.0);
        }
        other => panic!("GTOP must lower to a behavioral current source, found {other:?}"),
    }
}

#[test]
fn default_and_ordinary_netlists_report_zero_authored_chebyshev_cards() {
    assert_eq!(Netlist::default().pspice_chebyshev_source_count(), 0);
    let ordinary =
        Netlist::parse("ordinary\nV1 1 0 1\nR1 1 0 1k\n.end\n").expect("ordinary deck parses");
    assert_eq!(ordinary.pspice_chebyshev_source_count(), 0);
}

#[test]
fn expanded_include_cards_contribute_to_authored_count() {
    let directory = TempDirectory::new("include-count");
    let child = directory.path().join("filter.inc");
    let root = directory.path().join("root.cir");
    std::fs::write(
        &child,
        "EINC out 0 CHEBYSHEV {V(in)} = LP (800Hz 1.2kHz) .1dB 50dB\n",
    )
    .expect("write included CHEBYSHEV card");
    std::fs::write(&root, "include metadata\n.include filter.inc\n.end\n")
        .expect("write root deck");

    let netlist = Netlist::parse_file(&root).expect("included CHEBYSHEV deck parses");
    assert_eq!(netlist.pspice_chebyshev_source_count(), 1);
}

#[test]
fn addresistors_materialization_preserves_authored_count() {
    let source = "CHEBYSHEV materialization metadata\n\
        .preprocess addresistors oneterminal 1meg\n\
        EFILTER out 0 CHEBYSHEV {V(0)} = LP (800Hz 1.2kHz) .1dB 50dB\n\
        R1 1 2 1k\n\
        C1 2 0 1u\n\
        .end\n";
    let netlist = Netlist::parse(source).expect("CHEBYSHEV ADDRESISTORS deck parses");
    assert_eq!(netlist.pspice_chebyshev_source_count(), 1);

    let materialized = netlist
        .materialize_xyce_add_resistors()
        .expect("ADDRESISTORS derived netlist materializes");
    assert_eq!(
        materialized.netlist.pspice_chebyshev_source_count(),
        netlist.pspice_chebyshev_source_count()
    );
}

fn ac_voltage(netlist: &Netlist, node: &str, frequency_hz: f64) -> f64 {
    let result = Engine::new(SimulationConfig::default())
        .run_ac(netlist, &[frequency_hz])
        .expect("CHEBYSHEV AC analysis solves");
    let index = result[0]
        .node_names
        .iter()
        .position(|candidate| candidate.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("AC result is missing node {node}"));
    result[0].voltages[index].norm()
}

#[test]
fn canonical_no_equals_syntax_is_accepted() {
    let netlist = Netlist::parse(
        "canonical Cadence PSpice CHEBYSHEV syntax\n\
         vin in 0 ac 1\n\
         efilter out 0 chebyshev V(in) lp (800Hz 1.2kHz) .1dB 50dB\n\
         rload out 0 1k\n\
         .end\n",
    )
    .expect("canonical no-equals CHEBYSHEV card parses");

    assert_eq!(netlist.pspice_chebyshev_source_count(), 1);
    assert!(ac_voltage(&netlist, "out", 1_200.0) <= 10.0f64.powf(-50.0 / 20.0) * 1.01);
}

#[test]
fn attached_hertz_units_preserve_contextual_engineering_scaling() {
    for (pass, stop, pass_hz, stop_hz) in [
        ("800Hz", "1.2kHz", 800.0, 1_200.0),
        ("1kHz", "1.5kHz", 1.0e3, 1.5e3),
        ("1MHz", "1.5MHz", 1.0e6, 1.5e6),
        ("1MEGHz", "1.5MEGHz", 1.0e6, 1.5e6),
        ("1uHz", "1.5uHz", 1.0e-6, 1.5e-6),
    ] {
        let deck = format!(
            "contextual hertz scaling\nvin in 0 ac 1\nefilter out 0 CHEBYSHEV {{V(in)}} LP ({pass} {stop}) .1dB 50dB\nrload out 0 1k\n.end\n"
        );
        let netlist = Netlist::parse(&deck)
            .unwrap_or_else(|error| panic!("{pass}/{stop} cutoff deck must parse: {error}"));

        assert!(ac_voltage(&netlist, "out", pass_hz) >= 10.0f64.powf(-0.1 / 20.0) * 0.999);
        assert!(ac_voltage(&netlist, "out", stop_hz) <= 10.0f64.powf(-50.0 / 20.0) * 1.01);
    }
}

#[test]
fn subcircuit_instances_design_from_their_own_parameter_overrides() {
    let netlist = Netlist::parse(
        "per-instance CHEBYSHEV parameter resolution\n\
         .subckt FILTER in out params: FP=800 FS=1.2k\n\
         efilter out 0 CHEBYSHEV {V(in)} LP (FP FS) .1dB 50dB\n\
         .ends FILTER\n\
         vin in 0 ac 1\n\
         xslow in slow FILTER FP=800 FS=1.2k\n\
         xfast in fast FILTER FP=8k FS=12k\n\
         rslow slow 0 1k\n\
         rfast fast 0 1k\n\
         .end\n",
    )
    .expect("parameterized subcircuit CHEBYSHEV deck parses");

    let slow_stop = ac_voltage(&netlist, "slow", 1_200.0);
    let fast_pass = ac_voltage(&netlist, "fast", 1_200.0);
    assert!(slow_stop <= 10.0f64.powf(-50.0 / 20.0) * 1.01);
    assert!(fast_pass >= 10.0f64.powf(-0.1 / 20.0) * 0.999);
}

#[test]
fn forward_top_level_parameters_resolve_before_design() {
    let netlist = Netlist::parse(
        "forward CHEBYSHEV parameters\n\
         vin in 0 ac 1\n\
         efilter out 0 CHEBYSHEV {V(in)} LP ({FP} {FS}) {RP+0} {RS}\n\
         rload out 0 1k\n\
         .param FP=800 FS=1.2k RP=.1 RS=50\n\
         .end\n",
    )
    .expect("forward parameter references resolve after the complete deck is parsed");

    assert!(ac_voltage(&netlist, "out", 1_200.0) <= 10.0f64.powf(-50.0 / 20.0) * 1.01);
}

#[test]
fn forward_top_level_current_source_multiplicity_resolves_before_lowering() {
    let netlist = Netlist::parse(
        "forward CHEBYSHEV current-source multiplicity\n\
         vin in 0 ac 1\n\
         gunit 0 unit CHEBYSHEV {V(in)} LP (800Hz 1.2kHz) .1dB 50dB M=1\n\
         gscaled 0 scaled CHEBYSHEV {V(in)} LP (800Hz 1.2kHz) .1dB 50dB M={SCALE}\n\
         runit unit 0 1k\n\
         rscaled scaled 0 1k\n\
         .param SCALE=2\n\
         .end\n",
    )
    .expect("forward G-source multiplicity resolves after the complete deck is parsed");

    let unit = ac_voltage(&netlist, "unit", 800.0);
    let scaled = ac_voltage(&netlist, "scaled", 800.0);
    assert!(
        (scaled / unit - 2.0).abs() <= 1.0e-12,
        "M={{SCALE}} must produce exactly twice the M=1 transfer: unit={unit}, scaled={scaled}"
    );
}

#[test]
fn incompatible_explicit_units_fail_closed() {
    for filter in [
        "LP (800ms 1.2kHz) .1dB 50dB",
        "LP (800Hz 1.2kHz) .1V 50dB",
        "LP (800Hz 1.2kHz) .1dB 50A",
    ] {
        let deck = format!(
            "incompatible CHEBYSHEV units\nvin in 0 ac 1\nefilter out 0 CHEBYSHEV {{V(in)}} {filter}\nrload out 0 1k\n.end\n"
        );
        let error = Netlist::parse(&deck).expect_err("incompatible units must be rejected");
        assert!(error.to_string().contains("line 3"));
    }
}

#[test]
fn finite_but_unrepresentable_cutoff_range_fails_closed() {
    let error = Netlist::parse(
        "unrepresentable CHEBYSHEV coefficients\n\
         vin in 0 ac 1\n\
         efilter out 0 CHEBYSHEV {V(in)} LP (1e-200 Hz 2e-200 Hz) .1dB 50dB\n\
         rload out 0 1k\n\
         .end\n",
    )
    .expect_err("coefficient underflow must not silently change the filter");
    let message = error.to_string();
    assert!(message.contains("line 3"), "missing source line: {message}");
    assert!(
        message.contains("cannot be represented") || message.contains("lost a required nonzero"),
        "unexpected range diagnostic: {message}"
    );
}

#[test]
fn deferred_generated_element_name_collisions_fail_closed() {
    let netlist = Netlist::parse(
        "deferred CHEBYSHEV helper collision\n\
         .subckt FILTER in out params: FP=800 FS=1.2k\n\
         efilter out 0 CHEBYSHEV {V(in)} LP (FP FS) .1dB 50dB\n\
         efilter.__dx1 spare 0 in 0 1\n\
         .ends FILTER\n\
         vin in 0 ac 1\n\
         x1 in out FILTER\n\
         rload out 0 1k\n\
         .end\n",
    )
    .expect("deferred helper ownership is checked during flattening");

    let error = Engine::new(SimulationConfig::default())
        .run_ac(&netlist, &[1_000.0])
        .expect_err("generated and authored element names must not alias")
        .to_string();
    assert!(
        error.contains("collides"),
        "unexpected collision error: {error}"
    );
    assert!(
        error.to_ascii_uppercase().contains("EFILTER.__DX1"),
        "collision must name the conflicting helper: {error}"
    );
}

#[test]
fn later_top_level_node_cannot_alias_generated_dynamic_state_case_insensitively() {
    let netlist = Netlist::parse(
        "top-level CHEBYSHEV state-node collision\n\
         vin in 0 ac 1\n\
         efilter out 0 CHEBYSHEV {V(in)} LP (800Hz 1.2kHz) .1dB 50dB\n\
         rclamp EFILTER.__x1 0 1k\n\
         rload out 0 1k\n\
         .end\n",
    )
    .expect("node ownership is validated after final flattening");

    let error = Engine::new(SimulationConfig::default())
        .run_ac(&netlist, &[1_000.0])
        .expect_err("authored and generated state nodes must not alias")
        .to_string();
    assert!(
        error.contains("generated internal node"),
        "unexpected collision error: {error}"
    );
    assert!(
        error.to_ascii_uppercase().contains("EFILTER.__X1"),
        "collision must name the private state node: {error}"
    );
}

#[test]
fn hierarchical_node_cannot_alias_generated_cascade_output_case_insensitively() {
    let netlist = Netlist::parse(
        "hierarchical CHEBYSHEV cascade-node collision\n\
         .subckt FILTER in out params: FP=800 FS=1.2k\n\
         efilter out 0 CHEBYSHEV {V(in)} LP (FP FS) .1dB 50dB\n\
         rclamp eFiLtEr.__y1 0 1k\n\
         .ends FILTER\n\
         vin in 0 ac 1\n\
         x1 in out FILTER\n\
         rload out 0 1k\n\
         .end\n",
    )
    .expect("hierarchical node ownership is validated after remapping");

    let error = Engine::new(SimulationConfig::default())
        .run_ac(&netlist, &[1_000.0])
        .expect_err("authored and generated cascade nodes must not alias")
        .to_string();
    assert!(
        error.contains("generated internal node"),
        "unexpected collision error: {error}"
    );
    assert!(
        error.to_ascii_uppercase().contains("X1.EFILTER.__Y1"),
        "collision must name the remapped private cascade node: {error}"
    );
}

#[test]
fn subcircuit_formal_cannot_capture_a_generated_private_node() {
    let netlist = Netlist::parse(
        "CHEBYSHEV private node versus formal port\n\
         .subckt FILTER in out efilter.__X1 params: FP=800 FS=1.2k\n\
         efilter out 0 CHEBYSHEV {V(in)} LP (FP FS) .1dB 50dB\n\
         .ends FILTER\n\
         vin in 0 ac 1\n\
         x1 in out 0 FILTER\n\
         rload out 0 1k\n\
         .end\n",
    )
    .expect("private-node capture is validated after hierarchy expansion");

    let error = Engine::new(SimulationConfig::default())
        .run_ac(&netlist, &[1_000.0])
        .expect_err("a formal port must not capture a generated private node")
        .to_string();
    assert!(
        error.contains("internal-node metadata") || error.contains("generated internal node"),
        "unexpected capture error: {error}"
    );
    assert!(
        error.to_ascii_uppercase().contains("EFILTER.__CX1"),
        "capture diagnostic must identify the affected helper: {error}"
    );
}
