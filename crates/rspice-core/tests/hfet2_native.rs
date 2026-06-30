//! Native HFET2 (`Z`/`NHFET LEVEL=6`) validation against ngspice 46.

use rspice_core::Value;
use rspice_core::engine::Engine;
use rspice_core::netlist::Netlist;

fn engine() -> Engine {
    Engine::default()
}

fn hfet2_op_deck() -> &'static str {
    "\
* HFET2 NHFET DC operating point
Vds drain 0 DC 1.0
Vgs gate 0 DC 0.3
Z1 drain gate 0 HMOD L=1u W=10u
.model HMOD nhfet level=6 rd=60 rs=60 m=2.57 lambda=0.17
+ vs=1.5e5 mu=0.385 vto=0.13 eta=1.28 sigma0=0.04
+ vsigma=0.1 vsigmat=0.3 nmax=6e15 d1=0.03e-6 d2=0.2e-6
+ di=0.04e-6 delta=3.0 deltad=4.5e-9 gamma=3.0 n=5.0
.op
.end
"
}

fn nmf_level6_op_deck() -> String {
    hfet2_op_deck().replace(".model HMOD nhfet", ".model HMOD nmf")
}

fn hfet2_ac_deck() -> &'static str {
    "\
* HFET2 NHFET common-source AC
Vdd vdd 0 DC 2.0
Rd vdd drain 200
Vgs gate 0 DC 0.3 AC 1
Z1 drain gate 0 HMOD L=1u W=10u
.model HMOD nhfet level=6 rd=60 rs=60 m=2.57 lambda=0.17
+ vs=1.5e5 mu=0.385 vto=0.13 eta=1.28 sigma0=0.04
+ vsigma=0.1 vsigmat=0.3 nmax=6e15 d1=0.03e-6 d2=0.2e-6
+ di=0.04e-6 delta=3.0 deltad=4.5e-9 gamma=3.0 n=5.0
.end
"
}

fn hfet2_eta2_density_branch_op_deck() -> &'static str {
    "\
* HFET2 LEVEL=6 ETA2/D2/VT2 density branch
Vds drain 0 DC 1.0
Vgs gate 0 DC 0.3
Z1 drain gate 0 HMOD L=1u W=10u
.model HMOD nhfet level=6 rd=60 rs=60 m=2.57 lambda=0.17
+ vs=1.5e5 mu=0.385 vto=0.13 eta=1.28 sigma0=0.04
+ vsigma=0.1 vsigmat=0.3 nmax=6e15 di=0.04e-6
+ d1=0.03e-6 eta1=2 d2=0.2e-6 eta2=2 vt2=0.16
+ delta=3.0 deltad=4.5e-9 gamma=3.0 n=5.0
.op
.end
"
}

fn hfet2_eta2_density_branch_ac_deck() -> &'static str {
    "\
* HFET2 LEVEL=6 ETA2/D2/VT2 density branch AC
Vdd vdd 0 DC 2.0
Rd vdd drain 200
Vgs gate 0 DC 0.3 AC 1
Z1 drain gate 0 HMOD L=1u W=10u
.model HMOD nhfet level=6 rd=60 rs=60 m=2.57 lambda=0.17
+ vs=1.5e5 mu=0.385 vto=0.13 eta=1.28 sigma0=0.04
+ vsigma=0.1 vsigmat=0.3 nmax=6e15 di=0.04e-6
+ d1=0.03e-6 eta1=2 d2=0.2e-6 eta2=2 vt2=0.16
+ delta=3.0 deltad=4.5e-9 gamma=3.0 n=5.0
.end
"
}

fn hfet2_temperature_coefficient_op_deck() -> &'static str {
    "\
* HFET2 LEVEL=6 temperature coefficient OP
.option temp=127 tnom=27
Vds drain 0 DC 1.0
Vgs gate 0 DC 0.3
Z1 drain gate 0 HMOD L=1u W=10u
.model HMOD nhfet level=6 rd=60 rs=60 m=2.57 lambda=0.17
+ klambda=0.001 kmu=0.001 knmax=1e13 kvto=0.001
+ vs=1.5e5 mu=0.385 vto=0.13 eta=1.28 sigma0=0.04
+ vsigma=0.1 vsigmat=0.3 nmax=6e15 di=0.04e-6
+ d1=0.03e-6 eta1=2 d2=0.2e-6 eta2=2 vt2=0.16
+ delta=3.0 deltad=4.5e-9 gamma=3.0 n=5.0
.op
.end
"
}

fn hfet2_temperature_coefficient_ac_deck() -> &'static str {
    "\
* HFET2 LEVEL=6 temperature coefficient AC
.option temp=127 tnom=27
Vdd vdd 0 DC 2.0
Rd vdd drain 200
Vgs gate 0 DC 0.3 AC 1
Z1 drain gate 0 HMOD L=1u W=10u
.model HMOD nhfet level=6 rd=60 rs=60 m=2.57 lambda=0.17
+ klambda=0.001 kmu=0.001 knmax=1e13 kvto=0.001
+ vs=1.5e5 mu=0.385 vto=0.13 eta=1.28 sigma0=0.04
+ vsigma=0.1 vsigmat=0.3 nmax=6e15 di=0.04e-6
+ d1=0.03e-6 eta1=2 d2=0.2e-6 eta2=2 vt2=0.16
+ delta=3.0 deltad=4.5e-9 gamma=3.0 n=5.0
.end
"
}

fn hfet2_gate_current_op_deck() -> &'static str {
    "\
* HFET2 LEVEL=6 JS/GGR gate-current OP
.option gmin=0
Vds drain 0 DC 0.2
Vgs gate 0 DC 0.45
Z1 drain gate 0 HMOD L=1u W=10u
.model HMOD nhfet level=6 rd=0 rs=0 m=2.57 lambda=0.17
+ js=1e-4 ggr=40 del=0.04 n=5
+ vs=1.5e5 mu=0.385 vto=0.13 eta=1.28 sigma0=0.04
+ vsigma=0.1 vsigmat=0.3 nmax=6e15 di=0.04e-6
+ d1=0.03e-6 eta1=2 d2=0.2e-6 eta2=2 vt2=0.16
+ delta=3.0 deltad=4.5e-9 gamma=3.0
.op
.end
"
}

fn hfet2_transient_pulse_deck() -> &'static str {
    "\
* HFET2 NHFET transient pulse
Vdd vdd 0 DC 2.0
Rd vdd drain 200
Vgs gate 0 PULSE(0 0.4 0 0.1n 0.1n 1n 2n)
Z1 drain gate 0 HMOD L=1u W=10u
.model HMOD nhfet level=6 rd=60 rs=60 m=2.57 lambda=0.17
+ vs=1.5e5 mu=0.385 vto=0.13 eta=1.28 sigma0=0.04
+ vsigma=0.1 vsigmat=0.3 nmax=6e15 d1=0.03e-6 d2=0.2e-6
+ di=0.04e-6 delta=3.0 deltad=4.5e-9 gamma=3.0 n=5.0
.end
"
}

fn phfet2_op_deck() -> &'static str {
    "\
* HFET2 PHFET DC operating point
Vds drain 0 DC -1.0
Vgs gate 0 DC -0.3
Z1 drain gate 0 HMOD L=1u W=10u
.model HMOD phfet level=6 rd=60 rs=60 m=2.57 lambda=0.17
+ vs=0.8e5 mu=0.03 vto=-0.13 eta=1.4 sigma0=0.04
+ vsigma=0.1 vsigmat=0.3 nmax=6e15 di=0.04e-6
+ d1=0.03e-6 eta1=2 d2=0.2e-6 eta2=2 vt2=-0.16
+ delta=3.0 deltad=4.5e-9 gamma=3.0 n=5.0
.op
.end
"
}

fn assert_close(label: &str, got: Value, expected: Value, rel_tol: Value, abs_tol: Value) {
    let abs = (got - expected).abs();
    let tol = abs_tol.max(rel_tol * got.abs().max(expected.abs()).max(1.0e-30));
    assert!(
        abs <= tol,
        "{label}: rspice={got:.12e} ngspice46={expected:.12e} abs={abs:.3e} tol={tol:.3e}"
    );
}

fn node_series<'a>(names: &[String], voltages: &'a [Vec<Value>], want: &str) -> &'a [Value] {
    let idx = names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(want))
        .unwrap_or_else(|| panic!("missing {want} node in {:?}", names));
    &voltages[idx]
}

fn interpolate(time: &[Value], values: &[Value], target: Value) -> Value {
    assert_eq!(time.len(), values.len(), "time and value vectors align");
    if target <= time[0] {
        return values[0];
    }
    for index in 1..time.len() {
        if time[index] >= target {
            let t0 = time[index - 1];
            let t1 = time[index];
            let y0 = values[index - 1];
            let y1 = values[index];
            let frac = if t1 == t0 {
                0.0
            } else {
                (target - t0) / (t1 - t0)
            };
            return y0 + frac * (y1 - y0);
        }
    }
    *values.last().expect("non-empty value vector")
}

#[test]
fn phfet2_level6_op_matches_ngspice46_polarity() {
    let netlist = Netlist::parse(phfet2_op_deck()).expect("PHFET2 OP deck parses");
    let (result, report) = engine()
        .run_dc_op_with_report(&netlist)
        .expect("PHFET2 OP converges");

    let z1 = report
        .entries
        .iter()
        .find(|entry| entry.name.eq_ignore_ascii_case("z1"))
        .expect("z1 OP entry");
    assert_eq!(z1.device_kind, "HFET2");

    assert_close(
        "PHFET2 OP I(Vds)",
        result
            .branch_current_named("vds")
            .expect("Vds branch current"),
        1.742_091_247_822_555e-5,
        1.0e-5,
        1.0e-11,
    );
}

#[test]
fn hfet2_level6_transient_pulse_matches_ngspice46() {
    let netlist = Netlist::parse(hfet2_transient_pulse_deck()).expect("HFET2 tran deck parses");
    let result = engine()
        .run_tran(&netlist, 4.0e-9, 0.01e-9)
        .expect("HFET2 transient runs");
    let drain = node_series(&result.node_names, &result.voltages, "drain");

    // ngspice-46 `tran 0.01n 4n 0 0.01n`, `linearize v(drain)`.
    let reference = [
        (0.0, 1.999_612_461_991_581),
        (0.05e-9, 1.977_878_882_549_536),
        (0.1e-9, 1.912_755_886_985_636),
        (0.5e-9, 1.909_878_303_736_017),
        (1.0e-9, 1.909_878_303_751_528),
        (1.2e-9, 1.999_583_872_254_267),
        (2.0e-9, 1.999_612_456_817_702),
        (2.5e-9, 1.909_878_304_453_585),
        (3.0e-9, 1.909_878_304_428_148),
        (4.0e-9, 1.999_612_461_728_685),
    ];

    for &(time, v_ref) in &reference {
        let v = interpolate(&result.time, drain, time);
        let delta = (v - v_ref).abs();
        assert!(
            delta < 8.0e-3,
            "HFET2 transient at {time:.3e}s: rspice={v:.9e} ngspice={v_ref:.9e} delta={delta:.3e}"
        );
    }
}

#[test]
fn hfet2_level6_gate_current_matches_ngspice46() {
    let netlist = Netlist::parse(hfet2_gate_current_op_deck()).expect("HFET2 gate OP deck parses");
    let result = engine()
        .run_dc_op(&netlist)
        .expect("HFET2 gate OP converges");

    assert_close(
        "HFET2 JS/GGR OP I(Vds)",
        result
            .branch_current_named("vds")
            .expect("Vds branch current"),
        -3.056_951_993_773_827e-4,
        1.0e-5,
        1.0e-11,
    );
    assert_close(
        "HFET2 JS/GGR OP I(Vgs)",
        result
            .branch_current_named("vgs")
            .expect("Vgs branch current"),
        -7.886_124_525_662_594e-11,
        1.0e-5,
        1.0e-14,
    );
}

#[test]
fn hfet2_level6_temperature_coefficients_ac_matches_ngspice46() {
    let netlist =
        Netlist::parse(hfet2_temperature_coefficient_ac_deck()).expect("HFET2 temp AC deck parses");
    let result = engine()
        .run_ac(&netlist, &[1.0e9])
        .expect("HFET2 temp AC runs")
        .remove(0);
    let drain = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("drain"))
        .unwrap_or_else(|| panic!("missing drain node in {:?}", result.node_names));
    let v = result.voltages[drain];

    assert_close(
        "HFET2 KLAMBDA/KMU/KNMAX/KVTO AC V(drain).re",
        v.re,
        -2.524_407_141_869_349e-1,
        1.0e-5,
        1.0e-8,
    );
    assert_close(
        "HFET2 KLAMBDA/KMU/KNMAX/KVTO AC V(drain).im",
        v.im,
        3.260_119_427_881_715e-3,
        1.0e-5,
        1.0e-8,
    );
}

#[test]
fn hfet2_level6_temperature_coefficients_match_ngspice46() {
    let netlist =
        Netlist::parse(hfet2_temperature_coefficient_op_deck()).expect("HFET2 temp deck parses");
    let result = engine()
        .run_dc_op(&netlist)
        .expect("HFET2 temp OP converges");

    assert_close(
        "HFET2 KLAMBDA/KMU/KNMAX/KVTO OP I(Vds)",
        result
            .branch_current_named("vds")
            .expect("Vds branch current"),
        -7.820_015_253_063_445e-5,
        1.0e-5,
        1.0e-11,
    );
}

#[test]
fn hfet2_level6_eta2_density_branch_ac_matches_ngspice46() {
    let netlist =
        Netlist::parse(hfet2_eta2_density_branch_ac_deck()).expect("HFET2 ETA2 AC deck parses");
    let result = engine()
        .run_ac(&netlist, &[1.0e9])
        .expect("HFET2 ETA2 AC runs")
        .remove(0);
    let drain = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("drain"))
        .unwrap_or_else(|| panic!("missing drain node in {:?}", result.node_names));
    let v = result.voltages[drain];

    assert_close(
        "HFET2 ETA2/D2/VT2 AC V(drain).re",
        v.re,
        -3.449_178_104_276_041e-1,
        1.0e-5,
        1.0e-8,
    );
    assert_close(
        "HFET2 ETA2/D2/VT2 AC V(drain).im",
        v.im,
        5.047_432_768_388_053e-3,
        1.0e-5,
        1.0e-8,
    );
}

#[test]
fn hfet2_level6_eta2_density_branch_matches_ngspice46() {
    let netlist =
        Netlist::parse(hfet2_eta2_density_branch_op_deck()).expect("HFET2 ETA2 OP deck parses");
    let result = engine()
        .run_dc_op(&netlist)
        .expect("HFET2 ETA2 OP converges");

    assert_close(
        "HFET2 ETA2/D2/VT2 OP I(Vds)",
        result
            .branch_current_named("vds")
            .expect("Vds branch current"),
        -9.094_865_075_118_6e-5,
        1.0e-5,
        1.0e-11,
    );
}

#[test]
fn hfet2_level6_op_matches_ngspice46_and_reports_native_kind() {
    let netlist = Netlist::parse(hfet2_op_deck()).expect("HFET2 OP deck parses");
    let (result, report) = engine()
        .run_dc_op_with_report(&netlist)
        .expect("HFET2 OP converges");

    let z1 = report
        .entries
        .iter()
        .find(|entry| entry.name.eq_ignore_ascii_case("z1"))
        .expect("z1 OP entry");
    assert_eq!(
        z1.device_kind, "HFET2",
        "NHFET LEVEL=6 must be identified as the native HFET2 path"
    );

    assert_close(
        "HFET2 OP I(Vds)",
        result
            .branch_current_named("vds")
            .expect("Vds branch current"),
        -1.905_464_71e-4,
        1.0e-5,
        1.0e-11,
    );
}

#[test]
fn nmf_level6_uses_hfet2_route_like_ngspice46() {
    let deck = nmf_level6_op_deck();
    let netlist = Netlist::parse(&deck).expect("NMF LEVEL=6 OP deck parses");
    let (result, report) = engine()
        .run_dc_op_with_report(&netlist)
        .expect("NMF LEVEL=6 OP converges");

    let z1 = report
        .entries
        .iter()
        .find(|entry| entry.name.eq_ignore_ascii_case("z1"))
        .expect("z1 OP entry");
    assert_eq!(
        z1.device_kind, "HFET2",
        "NMF LEVEL=6 must route to the native HFET2-compatible path"
    );

    assert_close(
        "NMF LEVEL=6 OP I(Vds)",
        result
            .branch_current_named("vds")
            .expect("Vds branch current"),
        -1.905_464_71e-4,
        1.0e-5,
        1.0e-11,
    );
}

#[test]
fn hfet2_level6_common_source_ac_matches_ngspice46() {
    let netlist = Netlist::parse(hfet2_ac_deck()).expect("HFET2 AC deck parses");
    let result = engine()
        .run_ac(&netlist, &[1.0e9])
        .expect("HFET2 AC runs")
        .remove(0);
    let drain = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("drain"))
        .unwrap_or_else(|| panic!("missing drain node in {:?}", result.node_names));
    let v = result.voltages[drain];

    assert_close(
        "HFET2 AC V(drain).re",
        v.re,
        -3.366_156_11e-1,
        1.0e-5,
        1.0e-8,
    );
    assert_close(
        "HFET2 AC V(drain).im",
        v.im,
        4.367_344_92e-3,
        1.0e-5,
        1.0e-8,
    );
}
