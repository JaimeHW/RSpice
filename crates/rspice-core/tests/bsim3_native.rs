//! Engine-level validation of the native BSIM3v3.3 (MOS LEVEL=8/49) wiring.
//!
//! The model module (`device/mosfet/bsim3v3`) is pinned against ngspice-46
//! standalone; these tests prove the *engine* reproduces those values
//! through its own Newton solve, DC sweep, and transient integration:
//!
//! - a single-NMOS `.op` must hit the module's pinned oracle table;
//! - a CMOS inverter VTC `.dc` sweep is compared point-by-point against an
//!   ngspice-46 run of the same deck;
//! - a 3-stage ring oscillator `.tran` must oscillate with the period
//!   ngspice produces for the same deck (5% tolerance);
//! - a LEVEL=49 card builds and runs natively, without the
//!   `allow_simplified_mos` escape hatch.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

/// The module's own oracle model cards (n018/p018, LEVEL=49, CAPMOD=3).
fn models018() -> String {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/device/mosfet/bsim3v3/testdata/models018.lib"
    );
    std::fs::read_to_string(path).expect("read models018.lib")
}

fn engine() -> Engine {
    Engine::new(SimulationConfig::default())
}

#[test]
fn single_nmos_op_matches_module_oracle() {
    // m1 = 10u/0.18u at vds=1.2, vgs=1.2, vbs=0, T=27C: the exact bias of
    // the module's ngspice_pinned_nmos_idvg_saturation row. The engine must
    // reproduce what the module produced standalone — that proves the
    // builder/stamp/solve wiring, not just the device math.
    let deck = format!(
        "* bsim3 native op\n\
         vd d 0 dc 1.2\n\
         vg g 0 dc 1.2\n\
         m1 d g 0 0 n018 w=10u l=0.18u ad=4.2p as=4.2p pd=20.84u ps=20.84u nrd=0 nrs=0\n\
         {}\n\
         .op\n\
         .end\n",
        models018()
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
        entry.device_kind, "BSIM3",
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
    // ngspice-46 references (module tests.rs, 9 significant digits). The
    // module matches them at <= 4.9e-9 relative; through the engine solve
    // the only extra error is the Newton stopping tolerance on the
    // (source-driven, exactly-biased) terminals — keep 1e-6.
    let assert_rel = |what: &str, ours: f64, reference: f64| {
        let rel = (ours - reference).abs() / reference.abs();
        assert!(
            rel < 1e-6,
            "{what}: engine={ours:.9e} oracle={reference:.9e} rel={rel:.2e}"
        );
    };
    assert_rel("id", get("id"), 2.389567960e-03);
    assert_rel("gm", get("gm"), 4.543583450e-03);
    assert_rel("gds", get("gds"), 2.234402370e-04);
    assert_rel("gmb", get("gmb"), 1.134341020e-03);
    assert_rel("vth", get("vth"), 4.967558170e-01);
    assert_rel("vdsat", get("vdsat"), 3.733014340e-01);
}

/// The inverter used by the VTC and ring tests: 2u/0.18u PMOS over
/// 1u/0.18u NMOS on a 1.8 V rail, junction geometry spelled out.
fn inverter_pair(name: &str, input: &str, output: &str) -> String {
    format!(
        "mp{name} {output} {input} vdd vdd p018 w=2u l=0.18u ad=0.84p as=0.84p pd=4.84u ps=4.84u nrd=0 nrs=0\n\
         mn{name} {output} {input} 0 0 n018 w=1u l=0.18u ad=0.42p as=0.42p pd=2.84u ps=2.84u nrd=0 nrs=0\n"
    )
}

#[test]
fn cmos_inverter_vtc_matches_ngspice() {
    let deck = format!(
        "* bsim3 inverter vtc\n\
         vdd vdd 0 dc 1.8\n\
         vin in 0 dc 0\n\
         {}\
         {}\n\
         .end\n",
        inverter_pair("1", "in", "out"),
        models018()
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let results = engine()
        .run_dc_sweep(&netlist, "vin", 0.0, 1.8, 0.1)
        .expect("vtc sweep converges");

    // ngspice-46 reference: `ngspice -b` on this same deck (models018.lib,
    // `dc vin 0 1.8 0.1` with `.option reltol=1e-6`, wrdata v(out)), run
    // 2026-06-12 from the local ngspice-46 source build. The tightened
    // reltol matters: at its default 1e-3, ngspice's sweep-continuation
    // Newton leaves a step-size-dependent residue of up to ~1.4 mV on the
    // low-gain shoulders (its own standalone `.op` at vin=0.7 agrees with
    // the tight sweep, not the loose one).
    let reference: &[(f64, f64)] = &[
        (0.3, 1.79997200e+00),
        (0.5, 1.79598045e+00),
        (0.7, 1.71660112e+00),
        (0.8, 1.59036404e+00),
        (0.9, 6.77698565e-01), // steepest point of the transition
        (1.0, 1.30887788e-01),
        (1.2, 2.55494996e-02),
        (1.5, 1.47447798e-04),
        (1.8, 7.02511507e-08),
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
        // point is input-error amplified, so it gets a wider (still
        // sub-mV) gate.
        let tol = if (vin - 0.9).abs() < 1e-9 {
            5e-4
        } else {
            1e-5 * vout_ref.abs() + 1e-6
        };
        println!(
            "VTC vin={vin:.1}: engine={vout:.9e} ngspice={vout_ref:.9e} delta={:.2e}",
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
    // 3-stage ring, CAPMOD=3 intrinsic + overlap + junction charges as the
    // only load. `.ic v(n1)=0` kicks it off the metastable rail.
    let deck = format!(
        "* bsim3 ring oscillator\n\
         vdd vdd 0 dc 1.8\n\
         {}\
         {}\
         {}\
         .ic v(n1)=0\n\
         {}\n\
         .tran 0.5p 4n\n\
         .end\n",
        inverter_pair("1", "n1", "n2"),
        inverter_pair("2", "n2", "n3"),
        inverter_pair("3", "n3", "n1"),
        models018()
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let result = engine()
        .run_tran(&netlist, 4e-9, 2e-12)
        .expect("ring transient runs");
    let wave = result
        .try_voltage_waveform_named("n1")
        .expect("n1 waveform");
    let crossings = rising_crossings(&result.time, wave, 0.9);
    assert!(
        crossings.len() >= 8,
        "ring must oscillate: only {} rising crossings of 0.9 V in 4 ns",
        crossings.len()
    );
    let periods: Vec<f64> = crossings.windows(2).map(|w| w[1] - w[0]).collect();
    let tail = &periods[periods.len().saturating_sub(5)..];
    let period = tail.iter().sum::<f64>() / tail.len() as f64;

    // ngspice-46 reference: same deck (`.tran 0.5p 4n`, `.ic v(n1)=0`),
    // run 2026-06-12; the period settles to 2.2217e-10 s (4.50 GHz) with
    // <0.01% cycle-to-cycle spread after the first cycle.
    let reference = 2.2217e-10;
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
    // junction conductances) on the real axis, the mode-assembled BSIM3
    // capacitance matrix (intrinsic CAPMOD=3 + overlaps + capbd/capbs) on
    // the imaginary axis. The inverter is biased mid-transition with a
    // 10 fF load so the pole sits inside the sweep.
    let deck = format!(
        "* bsim3 inverter ac\n\
         vdd vdd 0 dc 1.8\n\
         vin in 0 dc 0.9 ac 1\n\
         {}\
         cl out 0 10f\n\
         {}\n\
         .end\n",
        inverter_pair("1", "in", "out"),
        models018()
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    // ngspice-46 reference: `ac dec 2 1e6 1e11` on this deck with
    // `.option reltol=1e-6`, run 2026-06-12 (vdb(out), ph(out)).
    let reference: &[(f64, f64, f64)] = &[
        (1.000000e6, 2.462154e+01, 3.139444e+00),
        (1.000000e7, 2.461961e+01, 3.120110e+00),
        (1.000000e8, 2.443028e+01, 2.929833e+00),
        (1.000000e9, 1.721812e+01, 1.984708e+00),
        (1.000000e10, -1.63000e+00, 1.359302e+00),
        (1.000000e11, -1.28853e+01, 3.661702e-01),
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
fn level49_runs_without_simplified_mos_optin() {
    // The full LEVEL=49 card must build and solve natively: no
    // `.options allow_simplified_mos`, no rejection, and the OP report
    // names the BSIM3 port for every instance.
    let deck = format!(
        "* level 49 native\n\
         vdd vdd 0 dc 1.8\n\
         vin in 0 dc 0.9\n\
         {}\
         {}\n\
         .op\n\
         .end\n",
        inverter_pair("1", "in", "out"),
        models018()
    );
    assert!(
        !deck.contains("allow_simplified_mos"),
        "deck must not opt into the approximation"
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let (_, report) = engine()
        .run_dc_op_with_report(&netlist)
        .expect("LEVEL=49 deck runs natively");
    let bsim3_count = report
        .entries
        .iter()
        .filter(|e| e.device_kind == "BSIM3")
        .count();
    assert_eq!(bsim3_count, 2, "both transistors use the native port");
}
