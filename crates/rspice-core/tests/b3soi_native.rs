//! Engine-level validation of native BSIM3-SOI routing.
//!
//! Xyce exposes its unified BSIMSOI3 front as MOS LEVEL=10. RSpice keeps the
//! ngspice-derived SOI ports internally as distinct native devices (55/56/57);
//! these tests ensure the Xyce-compatible level routes to a native SOI port
//! without opting into the simplified bulk-MOS approximation.

#![allow(clippy::excessive_precision)]

use rspice_core::analysis::advanced::stb::StbConfig;
use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;
use rspice_core::{
    Value,
    device::mosfet::b3soi::{
        common::CHARGE_Q,
        pd::{
            eval::{self, B3SoiPdBias, ModelConsts},
            params::B3SoiPdModel,
            temp::{B3SoiPdGeometry, B3SoiPdSized},
        },
    },
};
use std::collections::HashMap;

fn engine() -> Engine {
    Engine::new(SimulationConfig::default())
}

fn xyce_level10_default_gain_stage() -> &'static str {
    "* Xyce ACtests/bsim3soi/gain-stagesoi_default.cir\n\
     m1 3 2 0 0 nmos w=4u l=1u\n\
     rsource 1 2 100k\n\
     rload 3 vdd 25k\n\
     vdd1 vdd 0 5\n\
     vin 1 0 1.44 ac .1\n\
     .model nmos nmos level=10\n\
     .end\n"
}

fn b3soidd_n1_model_card() -> &'static str {
    include_str!("../../../tests/bsim3soidd/nmosdd.mod")
}

fn xyce_bsim3soi_self_heating_model_card() -> &'static str {
    include_str!("testdata/xyce_bsim3soi_nmos_3_2.mod")
}

fn xyce_bsim3soi_self_heating_model_card_with_cth(cth0: Value) -> String {
    let replacement = format!("cth0    = {cth0:.6e}");
    let updated = xyce_bsim3soi_self_heating_model_card().replace("cth0    = 1e-005", &replacement);
    assert_ne!(
        updated,
        xyce_bsim3soi_self_heating_model_card(),
        "expected test model to contain the default CTH0 token"
    );
    updated
}

fn level10_soimod3_auto_select_deck(selector_params: &str) -> String {
    format!(
        "\
        * Xyce/ngspice LEVEL=10 SOIMOD=3 auto-selects a native SOI family\n\
        m1 d g s 0 nmos w=4u l=1u\n\
        vd d 0 5\n\
        vg g 0 1.2\n\
        vs s 0 0\n\
        .model nmos nmos level=10 soimod=3 capmod=2 {selector_params}\n\
        .op\n\
        .end\n"
    )
}

#[test]
fn xyce_level10_soimod3_auto_selects_native_soi_family() {
    for (case, selector_params, expected_kind) in [
        ("pd", "", "B3SOIPD"),
        ("dd", "vbs0pd=-1 vbs0fd=0.5", "B3SOIDD"),
        ("fd", "vbs0fd=-1", "B3SOIFD"),
    ] {
        let deck = level10_soimod3_auto_select_deck(selector_params);
        let netlist = Netlist::parse(&deck).expect("deck parses");
        let (_, report) = engine()
            .run_dc_op_with_report(&netlist)
            .unwrap_or_else(|err| panic!("SOIMOD=3 {case} auto-select deck must run: {err}"));
        let entry = report
            .entries
            .iter()
            .find(|entry| entry.name.eq_ignore_ascii_case("m1"))
            .expect("m1 OP entry");

        assert_eq!(
            entry.device_kind, expected_kind,
            "SOIMOD=3 {case} selector should route to {expected_kind}"
        );
    }
}

#[test]
fn xyce_level10_soimod3_auto_selection_is_instance_length_aware() {
    let deck = "\
        * Xyce/ngspice SOIMOD=3 selector is per instance because Vbs0t depends on L\n\
        mshort d g 0 0 nmos w=4u l=0.2u\n\
        mlong d g 0 0 nmos w=4u l=5u\n\
        vd d 0 5\n\
        vg g 0 1.2\n\
        .model nmos nmos level=10 soimod=3 capmod=2 vbs0pd=-1 vbs0fd=0.5\n\
        .op\n\
        .end\n";

    let netlist = Netlist::parse(deck).expect("deck parses");
    let (_, report) = engine()
        .run_dc_op_with_report(&netlist)
        .expect("SOIMOD=3 instance-length deck runs");
    let device_kind = |name: &str| {
        report
            .entries
            .iter()
            .find(|entry| entry.name.eq_ignore_ascii_case(name))
            .map(|entry| entry.device_kind)
            .unwrap_or_else(|| panic!("missing OP entry for {name}"))
    };

    assert_eq!(device_kind("mshort"), "B3SOIPD");
    assert_eq!(device_kind("mlong"), "B3SOIDD");
}

#[test]
fn xyce_level10_rsh_default_squares_matches_xyce710_ngspice46_dc_op() {
    let deck = "\
        * native SOI RSH defaults NRD/NRS to one square\n\
        m1 d g s 0 nmos w=4u l=1u\n\
        vd d 0 5\n\
        vg g 0 1.2\n\
        vs s 0 0\n\
        .model nmos nmos level=10 soimod=0 rsh=10 capmod=2\n\
        .op\n\
        .end\n";

    let netlist = Netlist::parse(deck).expect("deck parses");
    let (op, report) = engine()
        .run_dc_op_with_report(&netlist)
        .expect("RSH operating point runs");

    let voltage = |name: &str| {
        op.try_voltage_named(name)
            .unwrap_or_else(|| panic!("missing node {name}"))
    };
    let branch = |name: &str| {
        op.branch_current_named(name)
            .unwrap_or_else(|| panic!("missing branch {name}"))
    };

    // Xyce 7.10 LEVEL=10 and ngspice-46 LEVEL=57 agree on this PD/RSH DC
    // point. The omitted NRD/NRS instance parameters default to one square in
    // both engines, so the 10 ohm sheet resistance creates visible prime-node
    // drops of about 45.7 uV on each terminal.
    assert_rel("I(VD)", branch("vd"), -4.56969270e-6, 2.0e-5);
    assert_rel("I(VS)", branch("vs"), 4.56969270e-6, 2.0e-5);
    assert_rel("V(M1.__dint)", voltage("M1.__dint"), 4.999954303073, 2.0e-8);
    assert_rel("V(M1.__sint)", voltage("M1.__sint"), 4.56969270e-5, 2.0e-5);

    let entry = report
        .entries
        .iter()
        .find(|entry| entry.name.eq_ignore_ascii_case("m1"))
        .expect("m1 OP entry");
    assert_eq!(entry.device_kind, "B3SOIPD");
    assert_eq!(entry.region, Some("saturation"));
    let param = |name: &str| {
        entry
            .params
            .iter()
            .find(|(key, _)| *key == name)
            .map(|(_, value)| *value)
            .unwrap_or_else(|| panic!("missing OP param {name}"))
    };
    assert_rel("id", param("id"), 4.56969e-6, 2.0e-5);
    assert_rel("vgs", param("vgs"), 1.19995, 2.0e-5);
    assert_rel("vds", param("vds"), 4.99991, 2.0e-5);
    assert_rel("vbs", param("vbs"), 3.56303e-2, 2.0e-5);
    assert_rel("gm", param("gm"), 5.87320e-5, 2.0e-5);
    assert_rel("gds", param("gds"), 2.83329e-7, 2.0e-5);
    assert_rel("gmb", param("gmb"), 2.79859e-5, 2.0e-5);
    assert_rel("vth", param("vth"), 1.08731, 2.0e-5);
    assert_rel("vdsat", param("vdsat"), 9.72944e-2, 2.0e-5);
}

#[test]
fn xyce_level10_capmod3_rsh_dc_op_matches_xyce710_ngspice46_dc_path() {
    let deck = "\
        * Xyce BSIMSOI3 dcSweep-style CAPMOD=3 remains valid for DC\n\
        m1 d g s 0 nmos w=4u l=1u\n\
        vd d 0 5\n\
        vg g 0 1.2\n\
        vs s 0 0\n\
        .model nmos nmos level=10 soimod=0 rsh=10 capmod=3\n\
        .op\n\
        .end\n";

    let netlist = Netlist::parse(deck).expect("deck parses");
    let op = engine().run_dc_op(&netlist).expect("CAPMOD=3 DC OP runs");

    assert_rel(
        "I(VD)",
        op.branch_current_named("vd").expect("VD branch current"),
        -4.56969270e-6,
        2.0e-5,
    );
    assert_rel(
        "V(M1.__dint)",
        op.try_voltage_named("M1.__dint")
            .expect("internal drain prime"),
        4.999954303073,
        2.0e-8,
    );
}

#[test]
fn xyce_level10_capmod3_rsh_dc_sweep_uses_native_pd_dc_path() {
    let deck = "\
        * Xyce BSIMSOI3 dcSweep-style CAPMOD=3 remains valid for DC sweep\n\
        m1 d g s 0 nmos w=4u l=1u\n\
        vd d 0 5\n\
        vg g 0 0\n\
        vs s 0 0\n\
        .model nmos nmos level=10 soimod=0 rsh=10 capmod=3\n\
        .end\n";

    let netlist = Netlist::parse(deck).expect("deck parses");
    let results = engine()
        .run_dc_sweep(&netlist, "vg", 0.8, 1.2, 0.2)
        .expect("CAPMOD=3 DC sweep runs");

    assert_eq!(results.len(), 3);
    let (_, final_point) = results
        .iter()
        .find(|(sweep, _)| (sweep - 1.2).abs() < 1.0e-12)
        .expect("VG=1.2 point present");
    assert_rel(
        "I(VD) at VG=1.2",
        final_point
            .branch_current_named("vd")
            .expect("VD branch current"),
        -4.56969270e-6,
        2.0e-5,
    );
    assert_rel(
        "V(M1.__dint) at VG=1.2",
        final_point
            .try_voltage_named("M1.__dint")
            .expect("internal drain prime"),
        4.999954303073,
        2.0e-8,
    );
}

#[test]
fn xyce_level10_capmod3_pd_ac_matches_ngspice46_level57_oracle() {
    let deck = "\
        * Xyce LEVEL=10 SOIMOD=0 CAPMOD=3 routes to native PD charge model\n\
        m1 out in 0 0 nmos w=4u l=1u\n\
        rload out vdd 25k\n\
        vdd vdd 0 5\n\
        vin in 0 1.2 ac 1\n\
        .model nmos nmos level=10 soimod=0 capmod=3\n\
        .end\n";

    let netlist = Netlist::parse(deck).expect("deck parses");
    let ngspice_reference: [(f64, f64, f64); 4] = [
        (1.0e6, -1.71490204316328, 1.113655509533947e-3),
        (1.0e7, -1.71488061804644, 1.113652229046714e-2),
        (1.0e8, -1.71273891507125, 1.113236042102483e-1),
        (1.0e9, -1.50636142781689, 1.073130836579288),
    ];
    let freqs = ngspice_reference
        .iter()
        .map(|(freq, _, _)| *freq)
        .collect::<Vec<_>>();
    let results = engine()
        .run_ac(&netlist, &freqs)
        .expect("PD CAPMOD=3 AC runs natively");

    for (result, (freq, re_ref, im_ref)) in results.iter().zip(ngspice_reference) {
        let idx = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .expect("node out in AC result");
        let v = result.voltages[idx];
        let reference_mag = re_ref.hypot(im_ref).max(1.0e-12);
        let rel = (v.re - re_ref).hypot(v.im - im_ref) / reference_mag;
        assert!(
            rel < 5.0e-5,
            "PD CAPMOD=3 AC mismatch at {freq:.3e} Hz: rspice=({:.9e},{:.9e}) ngspice=({re_ref:.9e},{im_ref:.9e}) rel={rel:.3e}",
            v.re,
            v.im
        );
    }
}

#[test]
fn xyce_level10_capmod3_pd_transient_matches_ngspice46_level57_oracle() {
    let deck = "\
        * Xyce LEVEL=10 SOIMOD=0 CAPMOD=3 routes to native PD transient charges\n\
        m1 out in 0 0 nmos w=4u l=1u\n\
        rload out vdd 25k\n\
        vdd vdd 0 5\n\
        vin in 0 pulse(0 1.2 0 50p 50p 1.0n 2.0n)\n\
        .model nmos nmos level=10 soimod=0 capmod=3\n\
        .tran 0.05n 4n\n\
        .end\n";

    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = engine()
        .run_tran(&netlist, 4.0e-9, 0.05e-9)
        .expect("PD CAPMOD=3 transient runs natively");
    let out = transient_node_series(&result.node_names, &result.voltages, "out");

    // ngspice-46 LEVEL=57 CAPMOD=3 oracle from the same deck, `tran 0.05n 4n`
    // followed by `linearize v(out)`.
    let reference = [
        (0.20e-9, 3.893267591199596),
        (1.00e-9, 3.905511537365071),
        (1.20e-9, 4.937371670965534),
        (2.50e-9, 3.915015700243554),
        (3.20e-9, 4.938431276907882),
        (4.00e-9, 4.999999874999889),
    ];
    for (time, expected) in reference {
        let got = interpolate(&result.time, out, time);
        assert_rel(&format!("V(out) at {time:.3e}s"), got, expected, 2.0e-2);
    }
}

#[test]
fn xyce_level10_capmod3_pd_other_charge_analyses_run_natively() {
    let deck = "\
        * CAPMOD=3 PD charge model should be available to all small-signal charge analyses\n\
        m1 out in 0 0 nmos w=4u l=1u\n\
        rload out vdd 25k\n\
        vdd vdd 0 5\n\
        vin in 0 1.2 ac 1\n\
        .model nmos nmos level=10 soimod=0 capmod=3\n\
        .end\n";
    let netlist = Netlist::parse(deck).expect("deck parses");

    engine()
        .run_noise(&netlist, 1, &[1.0e6], 300.15)
        .expect("noise should use native PD CAPMOD=3 charge model");

    engine()
        .run_pz(&netlist, 1, 2)
        .expect("pole-zero should use native PD CAPMOD=3 charge model");

    let stb_deck = "\
        * STB also uses the PD small-signal charge path\n\
        m1 out in 0 0 nmos w=4u l=1u\n\
        vdd out 0 5\n\
        vin in 0 1.2\n\
        vprobe out sense 0\n\
        rload sense 0 1k\n\
        .model nmos nmos level=10 soimod=0 capmod=3\n\
        .end\n";
    let stb_netlist = Netlist::parse(stb_deck).expect("STB deck parses");
    engine()
        .run_stb(
            &stb_netlist,
            StbConfig::new()
                .with_sweep(1.0e3, 1.0e6, 3)
                .with_probe("vprobe"),
        )
        .expect("STB should use native PD CAPMOD=3 charge model");
}

#[test]
fn zero_hz_transfer_function_does_not_require_dynamic_charge_support() {
    let deck = "\
        * .TF is a 0 Hz small-signal solve, so FD CAPMOD=2 charges are not needed\n\
        m1 out in 0 0 nmos w=4u l=1u\n\
        rload out vdd 25k\n\
        vdd vdd 0 5\n\
        vin in 0 1.2\n\
        .model nmos nmos level=55 capmod=2\n\
        .end\n";

    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = engine()
        .run_transfer_function(&netlist, "out", None, false, "vin")
        .expect(".TF should not require a dynamic charge model at 0 Hz");

    assert!(
        result.gain.is_finite(),
        ".TF gain should be a finite DC small-signal value"
    );
}

#[test]
fn debug_minus_one_suppresses_unsupported_b3soi_dynamic_charges_in_ac() {
    for (level, capmod) in [(55, 2), (56, 2), (57, 0)] {
        let deck = format!(
            "\
            * DEBUG=-1 runs the SOI device quasi-statically without dynamic charge stamps\n\
            m1 out in 0 0 nmos w=4u l=1u debug=-1\n\
            rload out vdd 25k\n\
            vdd vdd 0 5\n\
            vin in 0 1.2 ac 1\n\
            .model nmos nmos level={level} capmod={capmod}\n\
            .end\n"
        );
        let netlist = Netlist::parse(&deck).expect("deck parses");
        engine().run_ac(&netlist, &[1.0e6]).unwrap_or_else(|err| {
            panic!("LEVEL={level} CAPMOD={capmod} DEBUG=-1 AC should suppress charges: {err}")
        });
    }
}

#[test]
fn native_soi_dd_capmod2_charge_analyses_run_natively() {
    let deck = "\
        * DD CAPMOD=2 has native charge equations\n\
        m1 out in 0 0 nmos w=4u l=1u\n\
        vdd out 0 5\n\
        vin in 0 1.2 ac 1\n\
        .model nmos nmos level=56 capmod=2\n\
        .end\n";
    let netlist = Netlist::parse(deck).expect("deck parses");
    engine()
        .run_dc_op(&netlist)
        .unwrap_or_else(|err| panic!("B3SOIDD CAPMOD=2 DC should run: {err}"));

    engine()
        .run_ac(&netlist, &[1.0e6])
        .expect("DD CAPMOD=2 AC should run");
    engine()
        .run_tran(&netlist, 1.0e-9, 0.1e-9)
        .expect("DD CAPMOD=2 transient should run");
    engine()
        .run_noise(&netlist, 1, &[1.0e6], 300.15)
        .expect("DD CAPMOD=2 noise should run");
    engine()
        .run_pz(&netlist, 1, 2)
        .expect("DD CAPMOD=2 pole-zero should run");

    let stb_deck = "\
        * STB shares the DD small-signal charge gate\n\
        m1 out in 0 0 nmos w=4u l=1u\n\
        vdd out 0 5\n\
        vin in 0 1.2\n\
        vprobe out sense 0\n\
        rload sense 0 1k\n\
        .model nmos nmos level=56 capmod=2\n\
        .end\n";
    let stb_netlist = Netlist::parse(stb_deck).expect("STB deck parses");
    engine()
        .run_stb(
            &stb_netlist,
            StbConfig::new()
                .with_sweep(1.0e3, 1.0e6, 3)
                .with_probe("vprobe"),
        )
        .expect("DD CAPMOD=2 STB should run");
}

#[test]
fn native_soi_fd_capmod2_charge_analyses_run_natively() {
    let deck = "\
        * FD CAPMOD=2 has native charge equations\n\
        m1 out in 0 0 nmos w=4u l=1u\n\
        rload out vdd 25k\n\
        vdd vdd 0 5\n\
        vin in 0 1.2 ac 1\n\
        .model nmos nmos level=55 capmod=2\n\
        .end\n";
    let netlist = Netlist::parse(deck).expect("deck parses");
    engine()
        .run_dc_op(&netlist)
        .expect("FD CAPMOD=2 DC should run");
    engine()
        .run_ac(&netlist, &[1.0e6])
        .expect("FD CAPMOD=2 AC should run");
    engine()
        .run_tran(&netlist, 1.0e-9, 0.1e-9)
        .expect("FD CAPMOD=2 transient should run");
    engine()
        .run_noise(&netlist, 1, &[1.0e6], 300.15)
        .expect("FD CAPMOD=2 noise should run");
    engine()
        .run_pz(&netlist, 1, 2)
        .expect("FD CAPMOD=2 pole-zero should run");

    let stb_deck = "\
        * STB shares the FD small-signal charge path\n\
        m1 out in 0 0 nmos w=4u l=1u\n\
        vdd out 0 5\n\
        vin in 0 1.2\n\
        vprobe out sense 0\n\
        rload sense 0 1k\n\
        .model nmos nmos level=55 capmod=2\n\
        .end\n";
    let stb_netlist = Netlist::parse(stb_deck).expect("STB deck parses");
    engine()
        .run_stb(
            &stb_netlist,
            StbConfig::new()
                .with_sweep(1.0e3, 1.0e6, 3)
                .with_probe("vprobe"),
        )
        .expect("FD CAPMOD=2 STB should run");
}

#[test]
fn native_soi_fd_capmod2_ac_matches_ngspice46_level55_oracle() {
    let deck = "\
        * Native B3SOIFD CAPMOD=2 should use its own charge equations\n\
        m1 out in 0 0 nmos w=4u l=1u\n\
        rload out vdd 25k\n\
        vdd vdd 0 5\n\
        vin in 0 1.2 ac 1\n\
        .model nmos nmos level=55 capmod=2\n\
        .end\n";

    let netlist = Netlist::parse(deck).expect("deck parses");
    let ngspice_reference: [(f64, f64, f64); 4] = [
        (1.0e6, -9.66674e-2, 2.709996e-4),
        (1.0e7, -9.66625e-2, 2.709987e-3),
        (1.0e8, -9.61760e-2, 2.709105e-2),
        (1.0e9, -4.90758e-2, 2.623668e-1),
    ];
    let freqs = ngspice_reference
        .iter()
        .map(|(freq, _, _)| *freq)
        .collect::<Vec<_>>();
    let results = engine()
        .run_ac(&netlist, &freqs)
        .expect("FD CAPMOD=2 AC runs natively");

    for (result, (freq, re_ref, im_ref)) in results.iter().zip(ngspice_reference) {
        let idx = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .expect("node out in AC result");
        let v = result.voltages[idx];
        let reference_mag = re_ref.hypot(im_ref).max(1.0e-12);
        let rel = (v.re - re_ref).hypot(v.im - im_ref) / reference_mag;
        assert!(
            rel < 5.0e-5,
            "FD CAPMOD=2 AC mismatch at {freq:.3e} Hz: rspice=({:.9e},{:.9e}) ngspice=({re_ref:.9e},{im_ref:.9e}) rel={rel:.3e}",
            v.re,
            v.im
        );
    }
}

#[test]
fn native_soi_dd_capmod2_ac_matches_ngspice46_level56_oracle() {
    let model = b3soidd_n1_model_card().replace("CAPMOD = 3", "CAPMOD = 2");
    let deck = format!(
        "\
        * Native B3SOIDD CAPMOD=2 should use its own charge equations\n\
        m1 out in 0 e n1 w=4u l=1u\n\
        rload out vdd 25k\n\
        vdd vdd 0 5\n\
        vin in 0 1.2 ac 1\n\
        ve e 0 1.25\n\
        .option gmin=1e-20 itl1=200\n\
        {model}\n\
        .end\n"
    );

    let netlist = Netlist::parse(&deck).expect("deck parses");
    let ngspice_reference: [(f64, f64, f64); 3] = [
        (1.0e6, -5.98864750945694e-1, 3.012189044297493e-4),
        (1.0e9, -5.70599050583291e-1, 2.985427360741024e-1),
        (1.0e12, 2.582255591770817, 3.359900547376408e-2),
    ];
    let freqs = ngspice_reference
        .iter()
        .map(|(freq, _, _)| *freq)
        .collect::<Vec<_>>();
    let results = engine()
        .run_ac(&netlist, &freqs)
        .expect("DD CAPMOD=2 AC runs natively");

    for (result, (freq, re_ref, im_ref)) in results.iter().zip(ngspice_reference) {
        let idx = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .expect("node out in AC result");
        let v = result.voltages[idx];
        let reference_mag = re_ref.hypot(im_ref).max(1.0e-12);
        let rel = (v.re - re_ref).hypot(v.im - im_ref) / reference_mag;
        assert!(
            rel < 5.0e-5,
            "DD CAPMOD=2 AC mismatch at {freq:.3e} Hz: rspice=({:.9e},{:.9e}) ngspice=({re_ref:.9e},{im_ref:.9e}) rel={rel:.3e}",
            v.re,
            v.im
        );
    }
}

#[test]
fn native_level55_fd_self_heating_op_allocates_temperature_node() {
    let deck = "\
        * Native B3SOIFD self-heating should use an internal temperature-rise node\n\
        m1 d g s e nmos w=10u l=0.25u\n\
        vd d 0 1.5\n\
        vg g 0 1.5\n\
        vs s 0 0\n\
        ve e 0 0\n\
        .model nmos nmos level=55 shmod=1 rth0=1 cth0=1e-6 capmod=3\n\
        .op\n\
        .end\n";

    let netlist = Netlist::parse(deck).expect("deck parses");
    let op = engine()
        .run_dc_op(&netlist)
        .expect("B3SOIFD self-heating OP runs natively");

    let temp = op
        .try_voltage_named("m1.__temp.internal")
        .expect("self-heating temperature node");
    assert!(
        temp.is_finite() && temp >= 0.0,
        "temperature rise should be finite and non-negative, got {temp:.9e}"
    );
    assert!(
        op.branch_current_named("vd")
            .expect("VD branch current")
            .abs()
            > 0.0,
        "biased FD device should conduct"
    );
}

#[test]
fn native_level55_fd_self_heating_uses_instance_rth_cth_overrides() {
    let deck = "\
        * ngspice B3SOIFD accepts RTH0/CTH0 on the instance as well as the model\n\
        m1 d g s e nmos w=10u l=0.25u rth0=1 cth0=1e-6\n\
        vd d 0 1.5\n\
        vg g 0 1.5\n\
        vs s 0 0\n\
        ve e 0 0\n\
        .model nmos nmos level=55 shmod=1 capmod=3\n\
        .op\n\
        .end\n";

    let netlist = Netlist::parse(deck).expect("deck parses");
    let op = engine()
        .run_dc_op(&netlist)
        .expect("B3SOIFD instance-level self-heating OP runs natively");

    let temp = op
        .try_voltage_named("m1.__temp.internal")
        .expect("instance RTH0 should allocate the self-heating temperature node");
    assert!(
        temp.is_finite() && temp >= 0.0,
        "temperature rise should be finite and non-negative, got {temp:.9e}"
    );
}

#[test]
fn native_level57_pd_self_heating_op_allocates_temperature_node() {
    let deck = "\
        * Native B3SOIPD self-heating should use an internal temperature-rise node\n\
        m1 d g s e nmos w=10u l=0.35u\n\
        vd d 0 1.5\n\
        vg g 0 1.5\n\
        vs s 0 0\n\
        ve e 0 0\n\
        .model nmos nmos level=57 shmod=1 rth0=.01 cth0=1.46e-5 capmod=2\n\
        .op\n\
        .end\n";

    let netlist = Netlist::parse(deck).expect("deck parses");
    let op = engine()
        .run_dc_op(&netlist)
        .expect("B3SOIPD self-heating OP runs natively");

    let temp = op
        .try_voltage_named("m1.__temp.internal")
        .expect("self-heating temperature node");
    assert!(
        temp.is_finite() && temp >= 0.0,
        "temperature rise should be finite and non-negative, got {temp:.9e}"
    );
    assert![
        op.branch_current_named("vd")
            .expect("VD branch current")
            .abs()
            > 0.0,
        "biased PD device should conduct"
    ];
}

#[test]
fn native_level57_pd_self_heating_inverse_mode_temp_rise_is_non_negative() {
    let deck = "\
        * In inverse mode, PD self-heating power uses ngspice's mode-selected Vds\n\
        m1 d g s e nmos w=10u l=0.35u\n\
        vd d 0 0\n\
        vg g 0 0\n\
        vs s 0 1.5\n\
        ve e 0 0\n\
        .model nmos nmos level=57 shmod=1 rth0=.01 cth0=1.46e-5 capmod=2\n\
        .op\n\
        .end\n";

    let netlist = Netlist::parse(deck).expect("deck parses");
    let op = engine()
        .run_dc_op(&netlist)
        .expect("inverse-mode B3SOIPD self-heating OP runs natively");

    let temp = op
        .try_voltage_named("m1.__temp.internal")
        .expect("self-heating temperature node");
    assert!(
        temp.is_finite() && temp >= 0.0,
        "inverse-mode dissipated power should not cool the device, got {temp:.9e}"
    );
}

#[test]
fn xyce_level10_soimod1_capmod2_dd_ac_bounds_xyce710_delta() {
    let deck = "\
        * Xyce LEVEL=10 SOIMOD=1 CAPMOD=2 routes to native DD charge model\n\
        m1 3 2 0 0 nmos w=4u l=1u\n\
        rsource 1 2 100k\n\
        rload 3 vdd 25k\n\
        vdd1 vdd 0 5\n\
        vin 1 0 1.44 ac .1\n\
        .model nmos nmos level=10 soimod=1 capmod=2 shmod=0 rsh=0\n\
        .end\n";

    let netlist = Netlist::parse(deck).expect("deck parses");
    let xyce_reference: [(f64, f64, f64); 4] = [
        (1.0e6, -3.80268211e-1, 3.55367453e-3),
        (1.0e7, -3.77068848e-1, 3.52434178e-2),
        (1.0e8, -2.03231156e-1, 1.93053377e-1),
        (1.0e9, 2.77946796e-3, 4.10060511e-2),
    ];
    let freqs = xyce_reference
        .iter()
        .map(|(freq, _, _)| *freq)
        .collect::<Vec<_>>();
    let results = engine()
        .run_ac(&netlist, &freqs)
        .expect("LEVEL=10 SOIMOD=1 CAPMOD=2 DD AC runs natively");

    for (result, (freq, re_ref, im_ref)) in results.iter().zip(xyce_reference) {
        let idx = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("3"))
            .expect("node 3 in AC result");
        let v = result.voltages[idx];
        let reference_mag = re_ref.hypot(im_ref).max(1.0e-12);
        let rel = (v.re - re_ref).hypot(v.im - im_ref) / reference_mag;
        assert!(
            rel < 4.0e-1,
            "LEVEL=10 SOIMOD=1 CAPMOD=2 Xyce compatibility delta at {freq:.3e} Hz exceeded bound: rspice=({:.9e},{:.9e}) xyce=({re_ref:.9e},{im_ref:.9e}) rel={rel:.3e}",
            v.re,
            v.im
        );
    }
}

#[test]
fn xyce_level10_soimod1_self_heating_ac_bounds_xyce710_delta() {
    let deck = format!(
        "\
        * Xyce ACtests/bsim3soi/gain-stagesoi.cir with nmos_3_2.mod\n\
        m1 3 2 0 0 nmos w=4u l=1u\n\
        rsource 1 2 100k\n\
        rload 3 vdd 25k\n\
        vdd1 vdd 0 5\n\
        vin 1 0 1.44 ac .1\n\
        {}\n\
        .end\n",
        xyce_bsim3soi_self_heating_model_card()
    );

    let netlist = Netlist::parse(&deck).expect("deck parses");
    let xyce_reference: [(f64, f64, f64); 4] = [
        (1.0e2, -2.38326508e-2, 1.12247799e-6),
        (1.0e6, -2.37515427e-2, 5.71821269e-4),
        (1.0e8, -3.23776652e-3, 8.94195047e-3),
        (1.0e9, 6.08258996e-4, 9.79155551e-4),
    ];
    let freqs = xyce_reference
        .iter()
        .map(|(freq, _, _)| *freq)
        .collect::<Vec<_>>();

    let results = engine()
        .run_ac(&netlist, &freqs)
        .expect("LEVEL=10 SOIMOD=1 SHMOD=1 AC runs natively");

    for (result, (freq, re_ref, im_ref)) in results.iter().zip(xyce_reference) {
        let idx = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("3"))
            .expect("node 3 in AC result");
        let v = result.voltages[idx];
        let reference_mag = re_ref.hypot(im_ref).max(1.0e-12);
        let rel = (v.re - re_ref).hypot(v.im - im_ref) / reference_mag;
        // This is a Xyce regression-gold coverage check, not the exact physics
        // pin: RSpice currently routes LEVEL=10/SOIMOD=1 to the ngspice-derived
        // DD flavor. Exact Xyce high-frequency AC parity needs a separate
        // native Xyce BSIMSOI3 flavor, like the default-gain-stage test below.
        assert!(
            rel < 6.0e-2,
            "LEVEL=10 SOIMOD=1 SHMOD=1 Xyce AC compatibility delta at {freq:.3e} Hz exceeded bound: rspice=({:.9e},{:.9e}) xyce=({re_ref:.9e},{im_ref:.9e}) rel={rel:.3e}",
            v.re,
            v.im
        );
    }
}

#[test]
fn xyce_level10_soimod1_self_heating_transient_cth_sets_thermal_lag() {
    let run = |cth0: Value| {
        let model = xyce_bsim3soi_self_heating_model_card_with_cth(cth0);
        let deck = format!(
            "\
            * LEVEL=10 SOIMOD=1 SHMOD=1 transient thermal pole check\n\
            m1 out in 0 0 nmos w=4u l=1u\n\
            rload out vdd 25k\n\
            vdd vdd 0 5\n\
            vin in 0 pulse(0 1.44 0 20p 20p 2n 4n)\n\
            {model}\n\
            .tran 0.02n 2n\n\
            .end\n"
        );
        let netlist = Netlist::parse(&deck).expect("self-heating transient deck parses");
        let result = engine()
            .run_tran(&netlist, 2.0e-9, 0.02e-9)
            .expect("self-heating transient runs");
        let temp =
            transient_node_series(&result.node_names, &result.voltages, "m1.__temp.internal");
        (result.time, temp.to_vec())
    };

    let (fast_time, fast_temp) = run(1.0e-8);
    let (slow_time, slow_temp) = run(1.0e-5);

    let sample_time = 2.0e-9;
    let fast = interpolate(&fast_time, &fast_temp, sample_time);
    let slow = interpolate(&slow_time, &slow_temp, sample_time);
    assert!(
        slow.is_finite() && slow > 0.0,
        "slow CTH0 temp rise should be finite and positive, got {slow:.9e}"
    );
    assert!(
        fast > 5.0 * slow,
        "smaller CTH0 should rise faster at {sample_time:.3e}s: fast={fast:.9e}, slow={slow:.9e}"
    );
}

#[test]
fn native_soi_fd_dd_capmod3_charge_analyses_stay_allowed() {
    for level in [55, 56] {
        let deck = format!(
            "\
            * FD/DD native dynamic charge support keeps CAPMOD=3 available\n\
            m1 out in in 0 nmos w=4u l=1u\n\
            rload out vdd 25k\n\
            vdd vdd 0 5\n\
            vin in 0 1.2 ac 1\n\
            .model nmos nmos level={level} capmod=3\n\
            .end\n"
        );
        let netlist = Netlist::parse(&deck).expect("deck parses");
        engine()
            .run_ac(&netlist, &[1.0e6])
            .unwrap_or_else(|err| panic!("LEVEL={level} CAPMOD=3 AC should run: {err}"));
    }
}

#[test]
fn native_soi_fd_rsh_series_resistance_matches_ngspice46_op() {
    let deck = "\
        * FD sheet resistance lowers to drain/source prime resistors\n\
        m1 d g s 0 nmos w=4u l=1u nrd=1.5 nrs=0.5\n\
        vd d 0 5\n\
        vg g 0 1.2\n\
        vs s 0 0\n\
        .model nmos nmos level=55 rsh=10 capmod=2\n\
        .op\n\
        .end\n";

    let netlist = Netlist::parse(deck).expect("deck parses");
    let op = engine()
        .run_dc_op(&netlist)
        .expect("B3SOIFD RSH operating point runs");

    assert_rel(
        "FD I(VD)",
        op.branch_current_named("vd").expect("VD branch current"),
        -1.2135878862951088e-7,
        5.0e-5,
    );
    assert_rel(
        "FD I(VS)",
        op.branch_current_named("vs").expect("VS branch current"),
        1.21354988565947605e-7,
        2.0e-5,
    );
    assert_rel(
        "FD V(M1.__dint)",
        op.try_voltage_named("M1.__dint")
            .expect("FD internal drain prime"),
        4.999998179615,
        2.0e-8,
    );
    assert_rel(
        "FD V(M1.__sint)",
        op.try_voltage_named("M1.__sint")
            .expect("FD internal source prime"),
        6.06775e-7,
        2.0e-5,
    );
}

#[test]
fn native_soi_dd_rsh_series_resistance_matches_ngspice46_op() {
    let deck = "\
        * DD sheet resistance lowers to drain/source prime resistors\n\
        m1 d g s 0 nmos w=4u l=1u nrd=1.5 nrs=0.5\n\
        vd d 0 5\n\
        vg g 0 1.2\n\
        vs s 0 0\n\
        .model nmos nmos level=56 rsh=10 capmod=2\n\
        .op\n\
        .end\n";

    let netlist = Netlist::parse(deck).expect("deck parses");
    let op = engine()
        .run_dc_op(&netlist)
        .expect("B3SOIDD RSH operating point runs");

    assert_rel(
        "DD I(VD)",
        op.branch_current_named("vd").expect("VD branch current"),
        1.74762808391459856e-7,
        5.0e-5,
    );
    assert_rel(
        "DD I(VS)",
        op.branch_current_named("vs").expect("VS branch current"),
        -1.7476660845898979e-7,
        2.0e-5,
    );
    assert_rel(
        "DD V(M1.__dint)",
        op.try_voltage_named("M1.__dint")
            .expect("DD internal drain prime"),
        5.000002621442,
        2.0e-8,
    );
    assert_rel(
        "DD V(M1.__sint)",
        op.try_voltage_named("M1.__sint")
            .expect("DD internal source prime"),
        -8.73835e-7,
        2.0e-5,
    );
}

fn default_pd_geometry() -> B3SoiPdGeometry {
    B3SoiPdGeometry {
        l: 1.0e-6,
        w: 4.0e-6,
        drain_area: 0.0,
        source_area: 0.0,
        drain_squares: 1.0,
        source_squares: 1.0,
        drain_perimeter: 0.0,
        source_perimeter: 0.0,
        body_squares: 1.0,
        rth0: 0.0,
        cth0: 0.0,
        nseg: 1.0,
    }
}

fn pd_model_consts(model: &B3SoiPdModel) -> ModelConsts {
    ModelConsts {
        cap_mod: model.cap_mod,
        cox: model.cox,
        cbox: model.cbox,
        csi: model.csi,
        csieff: model.csieff,
        qsi: model.qsi,
        qsieff: model.qsieff,
        adice: model.adice,
        tox: model.tox,
        dtoxcv: model.dtoxcv,
        tsi: model.tsi,
        xj: model.xj,
        charge_q: CHARGE_Q,
        mob_mod: model.mob_mod,
        cboxt: model.cboxt,
        xpart: model.xpart,
        tt: model.tt,
        mjswg: model.body_jct_gate_side_grading_coeff,
        phibswg: model.gate_sidewall_jct_potential.max(0.1),
        cjswg: model.unit_length_gate_sidewall_jct_cap,
        mtype: model.mtype,
    }
}

fn assert_rel(what: &str, got: Value, reference: Value, rel_tol: Value) {
    let denom = reference.abs().max(1.0e-30);
    let rel = (got - reference).abs() / denom;
    assert!(
        rel <= rel_tol,
        "{what}: rspice={got:.9e} oracle={reference:.9e} rel={rel:.3e}"
    );
}

fn transient_node_series<'a>(
    names: &[String],
    voltages: &'a [Vec<Value>],
    node_name: &str,
) -> &'a [Value] {
    let node = names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node_name))
        .unwrap_or_else(|| panic!("missing node {node_name} in {:?}", names));
    &voltages[node]
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
fn xyce_level10_default_gain_stage_matches_ngspice46_pd_ac_and_bounds_xyce710_delta() {
    let netlist = Netlist::parse(xyce_level10_default_gain_stage()).expect("deck parses");

    // ngspice-46 LEVEL=57 AC reference for the equivalent PD model. Xyce's
    // LEVEL=10 gold agrees on the DC point but diverges in the high-frequency
    // AC capacitance path, so the native PD port is pinned to ngspice here and
    // the Xyce values below are kept as bounded compatibility coverage.
    let ngspice_reference: [(f64, f64, f64); 8] = [
        (1.0e2, -3.83394e-1, -7.85629e-7),
        (1.0e3, -3.83394e-1, 4.404913e-6),
        (1.0e4, -3.83394e-1, 4.527525e-5),
        (1.0e5, -3.83394e-1, 4.528745e-4),
        (1.0e6, -3.83342e-1, 4.528163e-3),
        (1.0e7, -3.78249e-1, 4.469528e-2),
        (1.0e8, -1.59138e-1, 1.947314e-1),
        (1.0e9, 7.084906e-3, 3.307193e-2),
    ];
    let freqs = ngspice_reference
        .iter()
        .map(|(freq, _, _)| *freq)
        .collect::<Vec<_>>();
    let results = engine().run_ac(&netlist, &freqs).expect("AC runs");

    for (result, (freq, re_ref, im_ref)) in results.iter().zip(ngspice_reference) {
        let idx = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("3"))
            .expect("node 3 in AC result");
        let v = result.voltages[idx];
        let reference_mag = re_ref.hypot(im_ref).max(1.0e-12);
        let re_rel = (v.re - re_ref).abs() / reference_mag;
        let im_rel = (v.im - im_ref).abs() / reference_mag;
        assert!(
            re_rel < 3.0e-5,
            "LEVEL=10 AC real mismatch at {freq:.3e} Hz: rspice={:.9e} ngspice={re_ref:.9e} rel={re_rel:.3e}",
            v.re
        );
        assert!(
            im_rel < 3.0e-5,
            "LEVEL=10 AC imag mismatch at {freq:.3e} Hz: rspice={:.9e} ngspice={im_ref:.9e} rel={im_rel:.3e}",
            v.im
        );
    }

    // Xyce 7.10 regression gold for ACtests/bsim3soi/gain-stagesoi_default.cir.
    // This is intentionally a broad compatibility bound, not the physics pin:
    // exact Xyce high-frequency AC matching will need a separate native Xyce
    // BSIMSOI3 flavor.
    let xyce_reference: [(f64, f64, f64); 4] = [
        (1.0e2, -3.833977e-1, -8.795912e-7),
        (1.0e6, -3.833646e-1, 3.591540e-3),
        (1.0e8, -2.044012e-1, 1.946377e-1),
        (1.0e9, 2.762399e-3, 4.123262e-2),
    ];
    for (freq, re_ref, im_ref) in xyce_reference {
        let result = results
            .iter()
            .find(|result| (result.frequency - freq).abs() <= freq * 1.0e-12)
            .expect("matching frequency");
        let idx = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("3"))
            .expect("node 3 in AC result");
        let v = result.voltages[idx];
        let reference_mag = re_ref.hypot(im_ref).max(1.0e-12);
        let rel = (v.re - re_ref).hypot(v.im - im_ref) / reference_mag;
        assert!(
            rel < 2.4e-1,
            "LEVEL=10 AC compatibility delta at {freq:.3e} Hz exceeded bound: rspice=({:.9e},{:.9e}) xyce=({re_ref:.9e},{im_ref:.9e}) rel={rel:.3e}",
            v.re,
            v.im
        );
    }
}

#[test]
fn default_pd_eval_at_xyce_ngspice_op_bias_matches_show_m1() {
    let params = HashMap::new();
    let model = B3SoiPdModel::from_params(&params, false, 300.15);
    let sized = B3SoiPdSized::new(&model, &default_pd_geometry(), 300.15).expect("sized model");
    let consts = pd_model_consts(&model);
    let op = eval::eval_dc(
        &sized,
        &consts,
        B3SoiPdBias {
            vbs: 3.563028966782028e-2,
            vgs: 1.44,
            vds: 4.281330041254987,
            ves: 0.0,
            vps: 0.0,
            ..Default::default()
        },
        1.0,
    );

    assert_rel("ids", op.ids, 2.87468e-5, 2.0e-5);
    assert_rel("gm", op.gm, 1.38003e-4, 2.0e-5);
    assert_rel("gds", op.gds, 1.00837e-6, 2.0e-5);
    assert_rel("gmb", op.gmbs, 6.49865e-5, 2.0e-5);
    assert_rel("vth", op.von, 1.0875, 2.0e-5);
    assert_rel("vdsat", op.vdsat, 2.35915e-1, 2.0e-5);

    let charge = eval::eval(
        &sized,
        &consts,
        B3SoiPdBias {
            vbs: 3.563028966782028e-2,
            vgs: 1.44,
            vds: 4.281330041254987,
            ves: 0.0,
            vps: 0.0,
            ..Default::default()
        },
        1.0,
        true,
    )
    .charge
    .expect("charge state");
    // ngspice-46 LEVEL=57 `debug=-1` exposes B3SOIPDcbgb directly at this
    // operating-point bias.
    assert_rel("cbg", charge.gcbgb, -3.266521635578415e-15, 2.0e-5);
}

#[test]
fn xyce_level10_default_gain_stage_matches_xyce_ngspice_dc_op() {
    let netlist = Netlist::parse(xyce_level10_default_gain_stage()).expect("deck parses");
    let (op, report) = engine()
        .run_dc_op_with_report(&netlist)
        .expect("DC OP runs");

    let voltage = |name: &str| {
        op.try_voltage_named(name)
            .unwrap_or_else(|| panic!("missing node {name}"))
    };
    let branch = |name: &str| {
        op.branch_current_named(name)
            .unwrap_or_else(|| panic!("missing branch {name}"))
    };
    // Xyce 7.10 LEVEL=10 SOIMOD=0 and ngspice-46 LEVEL=57 agree on this
    // operating point. The internal body node name is RSpice's builder-owned
    // floating-body node for M1.
    assert_rel("V(3)", voltage("3"), 4.281330e0, 2.0e-6);
    assert_rel(
        "V(M1.__body.internal)",
        voltage("M1.__body.internal"),
        3.563029e-2,
        2.0e-5,
    );
    assert_rel("I(VDD1)", branch("vdd1"), -2.87468e-5, 2.0e-5);

    let entry = report
        .entries
        .iter()
        .find(|entry| entry.name.eq_ignore_ascii_case("m1"))
        .expect("m1 OP entry");
    assert_eq!(entry.device_kind, "B3SOIPD");
    assert_eq!(entry.region, Some("saturation"));
    let param = |name: &str| {
        entry
            .params
            .iter()
            .find(|(key, _)| *key == name)
            .map(|(_, value)| *value)
            .unwrap_or_else(|| panic!("missing OP param {name}"))
    };

    // ngspice-46 `show m1` for the equivalent LEVEL=57 deck.
    assert_rel("ids", param("id"), 2.87468e-5, 2.0e-5);
    assert_rel("gm", param("gm"), 1.38003e-4, 2.0e-5);
    assert_rel("gds", param("gds"), 1.00837e-6, 2.0e-5);
    assert_rel("gmb", param("gmb"), 6.49865e-5, 2.0e-5);
    assert_rel("vth", param("vth"), 1.0875, 2.0e-5);
    assert_rel("vdsat", param("vdsat"), 2.35915e-1, 2.0e-5);
}
