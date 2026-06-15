//! Engine-level validation of the native BSIM4 v4.8 (MOS LEVEL=14/54) wiring.
//!
//! The model module (`device/mosfet/bsim4v8`) is pinned against ngspice-46
//! standalone; these tests prove the *engine* reproduces those values
//! through its own Newton solve, DC sweep, and transient integration:
//!
//! - a single-NMOS `.op` must hit the module's pinned oracle table;
//! - a CMOS inverter VTC `.dc` sweep is compared point-by-point against an
//!   ngspice-46 run of the same deck;
//! - a 3-stage ring oscillator `.tran` must oscillate with the period
//!   ngspice produces for the same deck (5% tolerance);
//! - a LEVEL=54 card builds and runs natively, without the
//!   `allow_simplified_mos` escape hatch;
//! - an unported mode knob (rdsMod=1) fails with the module's typed
//!   construction error instead of running silently.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

/// The module's own oracle model cards (n45/p90, LEVEL=54, CAPMOD=2).
fn models45() -> String {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/device/mosfet/bsim4v8/testdata/models45.lib"
    );
    std::fs::read_to_string(path).expect("read models45.lib")
}

fn engine() -> Engine {
    Engine::new(SimulationConfig::default())
}

#[test]
fn single_nmos_op_matches_module_oracle() {
    // m1 = 1u/45n at vds=1.1, vgs=1.1, vbs=0, T=27C: the exact bias of the
    // module's ngspice_pinned_nmos_idvg_saturation row (and the geometry of
    // testdata/nmos_oracle.sp m1). The engine must reproduce what the
    // module produced standalone — that proves the builder/stamp/solve
    // wiring, not just the device math.
    let deck = format!(
        "* bsim4 native op\n\
         vd d 0 dc 1.1\n\
         vg g 0 dc 1.1\n\
         m1 d g 0 0 n45 w=1u l=45n ad=0.1p as=0.1p pd=2.2u ps=2.2u nrd=0 nrs=0\n\
         {}\n\
         .op\n\
         .end\n",
        models45()
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let (_, report) = engine()
        .run_dc_op_with_report(&netlist)
        .expect("op converges");
    let entry = report
        .entries
        .iter()
        .find(|e| e.name.eq_ignore_ascii_case("m1"))
        .expect("m1 op entry");
    assert_eq!(
        entry.device_kind, "BSIM4",
        "native port, not an approximation"
    );
    assert_eq!(entry.region, Some("saturation"));
    let get = |key: &str| {
        entry
            .params
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| *v)
            .unwrap_or_else(|| panic!("missing op param {key}"))
    };
    // ngspice-46 references (module tests.rs, 9 significant digits; the
    // vth row is the vgs=0 entry of the same vds=1.1/vbs=0 sweep — Vth does
    // not depend on vgs). The module matches them at <= 4.9e-9 relative;
    // through the engine solve the only extra error is the Newton stopping
    // tolerance on the (source-driven, exactly-biased) terminals — keep 1e-6.
    let assert_rel = |what: &str, ours: f64, reference: f64| {
        let rel = (ours - reference).abs() / reference.abs();
        assert!(
            rel < 1e-6,
            "{what}: engine={ours:.9e} oracle={reference:.9e} rel={rel:.2e}"
        );
    };
    assert_rel("id", get("id"), 1.40891935e-03);
    assert_rel("gm", get("gm"), 1.87452469e-03);
    assert_rel("gds", get("gds"), 3.04461834e-04);
    assert_rel("gmb", get("gmb"), -1.78776023e-03);
    assert_rel("vth", get("vth"), 3.16523792e-01);
    assert_rel("vdsat", get("vdsat"), 3.92689365e-01);
}

/// The inverter used by the VTC, AC, and ring tests: 2u/90n p90 PMOS over
/// 1u/45n n45 NMOS on a 1.1 V rail, junction geometry spelled out.
fn inverter_pair(name: &str, input: &str, output: &str) -> String {
    format!(
        "mp{name} {output} {input} vdd vdd p90 w=2u l=90n ad=0.2p as=0.2p pd=4.2u ps=4.2u nrd=0 nrs=0\n\
         mn{name} {output} {input} 0 0 n45 w=1u l=45n ad=0.1p as=0.1p pd=2.2u ps=2.2u nrd=0 nrs=0\n"
    )
}

#[test]
fn cmos_inverter_vtc_matches_ngspice() {
    let deck = format!(
        "* bsim4 inverter vtc\n\
         vdd vdd 0 dc 1.1\n\
         vin in 0 dc 0\n\
         {}\
         {}\n\
         .end\n",
        inverter_pair("1", "in", "out"),
        models45()
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let results = engine()
        .run_dc_sweep(&netlist, "vin", 0.0, 1.1, 0.05)
        .expect("vtc sweep converges");

    // ngspice-46 reference: `ngspice -b` on this same deck (models45.lib,
    // `dc vin 0 1.1 0.05` with `.option reltol=1e-6`, wrdata v(out)), run
    // 2026-06-12 from the local ngspice-46 source build. GIDL and the
    // junction/TAT leakage set the sub-mV rail offsets, so they are part of
    // the comparison.
    let reference: &[(f64, f64)] = &[
        (0.2, 1.09852267e+00),
        (0.3, 1.08160866e+00),
        (0.4, 9.57362956e-01),
        (0.45, 5.96754014e-01), // steepest point of the transition
        (0.5, 1.14564480e-01),
        (0.6, 2.24662187e-02),
        (0.8, 1.13518376e-03),
        (1.0, 1.27938201e-05),
        (1.1, 1.04583797e-06),
    ];
    for &(vin, vout_ref) in reference {
        let (_, result) = results
            .iter()
            .find(|(v, _)| (v - vin).abs() < 1e-9)
            .unwrap_or_else(|| panic!("sweep point vin={vin} present"));
        let node = result.node_index_named("out").expect("node out in result");
        let vout = result.voltage(node);
        // The device math matches ngspice to ~5e-9; the budget here is the
        // two solvers' Newton stopping criteria. The high-gain transition
        // points are input-error amplified, so they get a wider (still
        // sub-mV) gate.
        let tol = if (vin - 0.45).abs() < 1e-9 || (vin - 0.4).abs() < 1e-9 {
            5e-4
        } else {
            1e-5 * vout_ref.abs() + 1e-6
        };
        println!(
            "VTC vin={vin:.2}: engine={vout:.9e} ngspice={vout_ref:.9e} delta={:.2e}",
            (vout - vout_ref).abs()
        );
        assert!(
            (vout - vout_ref).abs() < tol,
            "VTC at vin={vin}: engine={vout:.9e} ngspice={vout_ref:.9e} (tol {tol:.1e})"
        );
    }
}

/// Rising-edge crossing times of `threshold`, linearly interpolated.
fn rising_crossings(time: &[f64], wave: &[f64], threshold: f64) -> Vec<f64> {
    let mut crossings = Vec::new();
    for i in 1..time.len() {
        if wave[i - 1] < threshold && wave[i] >= threshold {
            let f = (threshold - wave[i - 1]) / (wave[i] - wave[i - 1]);
            crossings.push(time[i - 1] + f * (time[i] - time[i - 1]));
        }
    }
    crossings
}

#[test]
fn ring_oscillator_period_matches_ngspice() {
    // 3-stage ring, CAPMOD=2 intrinsic + overlap + junction charges as the
    // only load. `.ic v(n1)=0` kicks it off the metastable rail.
    let deck = format!(
        "* bsim4 ring oscillator\n\
         vdd vdd 0 dc 1.1\n\
         {}\
         {}\
         {}\
         .ic v(n1)=0\n\
         {}\n\
         .tran 0.5p 2n\n\
         .end\n",
        inverter_pair("1", "n1", "n2"),
        inverter_pair("2", "n2", "n3"),
        inverter_pair("3", "n3", "n1"),
        models45()
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    // 0.5 ps step cap, the deck's own tstep: at this 67 ps period the
    // engine's period error is step-resolution dominated and first-order
    // in the cap (2p -> 5.0%, 1p -> 2.4%, 0.5p -> 1.2%, 0.25p -> 0.6%
    // against the ngspice reference below).
    let result = engine()
        .run_tran(&netlist, 2e-9, 0.5e-12)
        .expect("ring transient runs");
    let wave = result
        .try_voltage_waveform_named("n1")
        .expect("n1 waveform");
    let crossings = rising_crossings(&result.time, wave, 0.55);
    assert!(
        crossings.len() >= 10,
        "ring must oscillate: only {} rising crossings of 0.55 V in 2 ns",
        crossings.len()
    );
    let periods: Vec<f64> = crossings.windows(2).map(|w| w[1] - w[0]).collect();
    let tail = &periods[periods.len().saturating_sub(5)..];
    let period = tail.iter().sum::<f64>() / tail.len() as f64;

    // ngspice-46 reference: same deck (`.tran 0.5p 2n`, `.ic v(n1)=0`),
    // run 2026-06-12; the period settles to 6.7417e-11 s (14.8 GHz) with
    // <0.001% cycle-to-cycle spread after the first cycle (and is
    // tolerance-converged: reltol=1e-5/trtol=1/tstep=0.1p moves it only
    // to 6.7398e-11).
    let reference = 6.7417e-11;
    let rel = (period - reference).abs() / reference;
    println!(
        "ring: {} crossings, period engine={period:.6e} ngspice={reference:.4e} rel={rel:.4}",
        crossings.len()
    );
    assert!(
        rel < 0.05,
        "ring period: engine={period:.4e} ngspice={reference:.4e} rel={rel:.3}"
    );
}

#[test]
fn inverter_ac_response_matches_ngspice() {
    // Small-signal check of the AC path: DC linearization (gm/gds/gmbs +
    // junction/GIDL conductances) on the real axis, the mode-assembled
    // BSIM4 capacitance matrix (intrinsic CAPMOD=2 + overlaps +
    // capbd/capbs) on the imaginary axis. The inverter is biased
    // mid-transition with a 10 fF load so the pole sits inside the sweep.
    let deck = format!(
        "* bsim4 inverter ac\n\
         vdd vdd 0 dc 1.1\n\
         vin in 0 dc 0.45 ac 1\n\
         {}\
         cl out 0 10f\n\
         {}\n\
         .end\n",
        inverter_pair("1", "in", "out"),
        models45()
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    // ngspice-46 reference: `ac dec 2 1e6 1e11` on this deck with
    // `.option reltol=1e-6`, run 2026-06-12 (vdb(out), ph(out)).
    let reference: &[(f64, f64, f64)] = &[
        (1.000000e6, 2.18108888e+01, 3.14106759e+00),
        (1.000000e7, 2.18107735e+01, 3.13634209e+00),
        (1.000000e8, 2.17992505e+01, 3.08913285e+00),
        (1.000000e9, 2.07784764e+01, 2.65660742e+00),
        (1.000000e10, 7.38496114e+00, 1.69188063e+00),
        (1.000000e11, -1.07592260e+01, 9.81387293e-01),
    ];
    let freqs: Vec<f64> = reference.iter().map(|&(f, _, _)| f).collect();
    let results = engine().run_ac(&netlist, &freqs).expect("ac runs");
    for ((freq, db_ref, ph_ref), result) in reference.iter().zip(&results) {
        let idx = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case("out"))
            .expect("out in ac result");
        let v = result.voltages[idx];
        let db = 20.0 * v.norm().log10();
        let ph = v.arg();
        println!("AC f={freq:.3e}: engine ({db:.5} dB, {ph:.5} rad) ngspice ({db_ref}, {ph_ref})");
        assert!(
            (db - db_ref).abs() < 1e-3,
            "AC magnitude at {freq:.3e} Hz: engine={db:.5} dB ngspice={db_ref} dB"
        );
        assert!(
            (ph - ph_ref).abs() < 1e-3,
            "AC phase at {freq:.3e} Hz: engine={ph:.5} ngspice={ph_ref}"
        );
    }
}

#[test]
fn level54_runs_without_simplified_mos_optin() {
    // The full LEVEL=54 card must build and solve natively: no
    // `.options allow_simplified_mos`, no rejection, and the OP report
    // names the BSIM4 port for every instance.
    let deck = format!(
        "* level 54 native\n\
         vdd vdd 0 dc 1.1\n\
         vin in 0 dc 0.45\n\
         {}\
         {}\n\
         .op\n\
         .end\n",
        inverter_pair("1", "in", "out"),
        models45()
    );
    assert!(
        !deck.contains("allow_simplified_mos"),
        "deck must not opt into the approximation"
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let (_, report) = engine()
        .run_dc_op_with_report(&netlist)
        .expect("LEVEL=54 deck runs natively");
    let bsim4_count = report
        .entries
        .iter()
        .filter(|e| e.device_kind == "BSIM4")
        .count();
    assert_eq!(bsim4_count, 2, "both transistors use the native port");
}

#[test]
fn rdsmod_unported_knob_fails_with_typed_error() {
    // RDSMOD=1 (external S/D resistance nodes) is not ported: the card
    // must be rejected at build time with the module's typed error — not
    // run silently with the knob ignored, and not panic.
    let deck = "* rdsmod=1 rejection\n\
         vd d 0 dc 1.1\n\
         vg g 0 dc 1.1\n\
         m1 d g 0 0 nmod w=1u l=45n\n\
         .model nmod NMOS (LEVEL=54 VTH0=0.5 TOXE=1.4n NDEP=3e18 RDSMOD=1)\n\
         .op\n\
         .end\n";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let err = engine()
        .run_dc_op_with_report(&netlist)
        .expect_err("RDSMOD=1 must be rejected");
    let message = err.to_string();
    assert!(
        message.contains("RDSMOD=1") && message.contains("BSIM4"),
        "typed rejection names the unported knob: {message}"
    );
}
