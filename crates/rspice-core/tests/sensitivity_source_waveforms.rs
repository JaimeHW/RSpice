//! `.SENS` with respect to an independent source is the derivative with
//! respect to the value the operating point gives that source: its DC value,
//! or for a waveform without one the `t = 0` value `.op` takes. Every waveform
//! kind has one, and it enters the DC equations only as the source's
//! excitation, so the derivative is exact.
//!
//! The LDO cases are corpus design C5 (`tests/mixed-corpus/c05-ldo-trim`,
//! fixtures copied under `testdata/ldo_trim`): a BSIM4 regulator whose output
//! moves by ~2e-9 V for a 1 nA load step, under the ~1e-11 V each re-solved
//! operating point carries, so a finite difference of re-solves could not
//! resolve the load current's row. The reference values are ngspice-46's
//! `.sens v(vout)` on the same circuit (trim held at code 0, no ESD junction).

use rspice_core::NoAbort;
use rspice_core::analysis::{Sensitivity, SensitivityResult};
use rspice_core::engine::{Engine, SensitivityCardResult, SimulationConfig};
use rspice_core::netlist::{AnalysisCommand, Netlist};
use std::path::{Path, PathBuf};

fn sens(netlist: &Netlist) -> SensitivityResult {
    let card = netlist
        .analyses
        .iter()
        .find(|analysis| matches!(analysis, AnalysisCommand::Sensitivity { .. }))
        .expect("deck has a .sens card");
    match Engine::new(SimulationConfig::default())
        .run_sensitivity_from_card_with_abort(netlist, card, &NoAbort)
        .unwrap_or_else(|error| panic!("sensitivity resolves: {error}"))
    {
        SensitivityCardResult::Dc(result) => result,
        SensitivityCardResult::Ac(_) => panic!("a .sens card without AC is a DC study"),
    }
}

fn row<'a>(result: &'a SensitivityResult, name: &str, parameter: &str) -> &'a Sensitivity {
    result
        .sensitivities
        .iter()
        .find(|row| {
            row.element.eq_ignore_ascii_case(name) && row.parameter.eq_ignore_ascii_case(parameter)
        })
        .unwrap_or_else(|| panic!("{name}:{parameter} in {:?}", result.sensitivities))
}

fn assert_relative(actual: f64, expected: f64, tolerance: f64, what: &str) {
    assert!(
        (actual - expected).abs() <= tolerance * expected.abs(),
        "{what}: {actual:e} vs {expected:e} (relative {:e})",
        (actual - expected).abs() / expected.abs()
    );
}

// ---------------------------------------------------------------------------
// Every waveform kind, on a divider with a closed form
// ---------------------------------------------------------------------------

/// `vin` drives `out` through 1k with 3k to ground and `itest` injects into
/// `out`: `V(out) = (Vin / 1k + Itest) * 750`, so `dV/dVin = 0.75` and
/// `dV/dItest = 750` whatever either source's waveform is.
fn divider(vin: &str, itest: &str) -> Netlist {
    Netlist::parse(&format!(
        "* source-value sensitivity\nvin in 0 {vin}\nrin in out 1k\nrload out 0 3k\nitest 0 out {itest}\n.sens v(out)\n.end\n"
    ))
    .expect("deck parses")
}

/// The solved equations carry the 1e-12 S nodal conditioning on every node,
/// which moves this divider's closed form by under 1e-12 relative. A
/// finite-difference row on a zero-valued current source missed it by ~2e-9.
const CLOSED_FORM: f64 = 1e-11;

/// Check one waveform kind.
///
/// On the divider, each source's row is the closed form, its nominal value is
/// the value `.op` gives the source, and the nominal output is the operating
/// point's. As the C5 LDO's load current, the current waveform's row resolves
/// and is the row of a plain DC source at that `.op` value.
fn check_kind(kind: &str, vin: &str, vin_op: f64, itest: &str, itest_op: f64) {
    let (nominal, waveform) = ldo_load_row(itest);
    assert_eq!(nominal, itest_op, "{kind}: the LDO load's .op value");
    let (_, dc) = ldo_load_row(&format!("dc {itest_op:e}"));
    assert_relative(waveform, dc, 1e-12, &format!("{kind}: dV(vout)/dIload"));

    let netlist = divider(vin, itest);
    let result = sens(&netlist);

    let voltage = row(&result, "vin", "dc");
    assert_eq!(voltage.nominal_value, vin_op, "{kind}: vin's .op value");
    assert_relative(
        voltage.absolute,
        0.75,
        CLOSED_FORM,
        &format!("{kind}: dV(out)/dVin"),
    );
    let current = row(&result, "itest", "dc");
    assert_eq!(current.nominal_value, itest_op, "{kind}: itest's .op value");
    assert_relative(
        current.absolute,
        750.0,
        CLOSED_FORM,
        &format!("{kind}: dV(out)/dItest"),
    );

    let op = Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .expect("operating point");
    let out = op.try_voltage_named("out").expect("v(out)");
    assert_eq!(
        result.output_value, out,
        "{kind}: the study's nominal is .op's"
    );
    let expected = (vin_op / 1.0e3 + itest_op) * 750.0;
    assert!(
        (out - expected).abs() <= CLOSED_FORM * expected.abs().max(1e-3),
        "{kind}: v(out) {out:e} vs {expected:e}"
    );
}

#[test]
fn dc_source_rows_are_exact() {
    check_kind("dc", "dc 1.2", 1.2, "dc 0.4m", 0.4e-3);
}

#[test]
fn pulse_source_rows_differentiate_the_initial_value() {
    check_kind(
        "pulse",
        "pulse(0.2 1 1u 1n 1n 1u 2u)",
        0.2,
        "pulse(0.1m 1m 1u 1n 1n 1u 2u)",
        0.1e-3,
    );
}

#[test]
fn sin_source_rows_differentiate_the_offset_plus_phase_value() {
    let phase = 30.0_f64.to_radians().sin();
    check_kind(
        "sin",
        "sin(0.5 1 1k 0 0 30)",
        0.5 + phase,
        "sin(0.2m 0.4m 1k 0 0 30)",
        0.2e-3 + 0.4e-3 * phase,
    );
}

#[test]
fn exp_source_rows_differentiate_the_initial_value() {
    check_kind(
        "exp",
        "exp(0.3 1 1u 1u 2u 1u)",
        0.3,
        "exp(0.3m 1m 1u 1u 2u 1u)",
        0.3e-3,
    );
}

#[test]
fn pwl_source_rows_differentiate_the_first_point() {
    // ILOAD's shape on the C5 deck: zero until a load step.
    check_kind(
        "pwl",
        "pwl(0 0.4 1u 1)",
        0.4,
        "pwl(0 0 200u 0 201u 10m 300u 10m 301u 0)",
        0.0,
    );
}

#[test]
fn file_backed_pwl_source_rows_differentiate_the_file_value_at_zero() {
    let dir = std::env::temp_dir().join(format!("rspice_sens_pwl_file_{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("scratch directory");
    let voltage = dir.join("v.pwl");
    let current = dir.join("i.pwl");
    std::fs::write(&voltage, "0 0.6\n1e-6 1\n").expect("voltage waveform");
    std::fs::write(&current, "0 5e-4\n1e-6 1e-3\n").expect("current waveform");
    let quoted = |path: &PathBuf| {
        format!(
            "pwl file=\"{}\"",
            path.display().to_string().replace('\\', "/")
        )
    };
    check_kind("pwl file", &quoted(&voltage), 0.6, &quoted(&current), 5e-4);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn sffm_source_rows_differentiate_the_zero_initial_value() {
    // ngspice's SFFM is exactly zero at t <= TD, offset included.
    check_kind(
        "sffm",
        "sffm(0.2 1 1meg 5 100k)",
        0.0,
        "sffm(0.1m 1m 1meg 5 100k)",
        0.0,
    );
}

#[test]
fn am_source_rows_differentiate_the_zero_initial_value() {
    check_kind(
        "am",
        "am(1 0.5 1k 100k 0)",
        0.0,
        "am(1m 0.5m 1k 100k 0)",
        0.0,
    );
}

#[test]
fn trnoise_source_rows_differentiate_the_zero_mean() {
    check_kind(
        "trnoise",
        "trnoise(1m 1n 0 0)",
        0.0,
        "trnoise(1u 1n 0 0)",
        0.0,
    );
}

#[test]
fn trrandom_source_rows_differentiate_the_offset() {
    check_kind(
        "trrandom",
        "trrandom(2 1u 0 0.1 0.6)",
        0.6,
        "trrandom(2 1u 0 0.1m 0.6m)",
        0.6e-3,
    );
}

// ---------------------------------------------------------------------------
// The C5 LDO against ngspice-46
// ---------------------------------------------------------------------------

fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/testdata/ldo_trim")
}

/// The C5 load current: zero until a load step.
const C5_LOAD: &str = "pwl(0 0 200u 0 201u 10m 300u 10m 301u 0)";

/// The trim bits held at code 0 behind 20 ohm, as the ngspice oracle deck
/// holds them.
const HELD_TRIM: &str = "\
vtr3 tr3_h 0 0\nrtr3 tr3_h tr3 20\nvtr2 tr2_h 0 0\nrtr2 tr2_h tr2 20
vtr1 tr1_h 0 0\nrtr1 tr1_h tr1 20\nvtr0 tr0_h 0 0\nrtr0 tr0_h tr0 20
";

/// The C5 harness (`net/ldo.cir`) with the given load and `.sens` card.
fn ldo(load: &str, sens: &str, trim: &str) -> Netlist {
    let deck = format!(
        "* C5 LDO sensitivity\n.include models-bsim4.cir\n.include ldo_body.inc\n\
         vin vin 0 1.8 ac 1\nrload vout 0 100\niload vout 0 {load}\ncout vout co 1u\nresr co 0 0.1\n\
         {sens}\n{trim}.end\n"
    );
    Netlist::parse_with_path(&deck, &fixtures().join("deck.cir")).expect("LDO deck parses")
}

/// The load current's row and nominal value with the load driven by `load`.
fn ldo_load_row(load: &str) -> (f64, f64) {
    let result = sens(&ldo(load, ".sens v(vout) iload", HELD_TRIM));
    let row = row(&result, "iload", "dc");
    (row.nominal_value, row.absolute)
}

/// Only the rows ngspice is compared on.
const COMPARED: &str = ".sens v(vout) iload vin vref ibias rload rtop rb0";

/// ngspice-46 `.sens v(vout)` on the C5 LDO, trim held at code 0 behind 20
/// ohm, without the ESD junction.
fn assert_matches_ngspice(result: &SensitivityResult) {
    // An independent source's row is exact in both simulators.
    for (name, expected) in [
        ("iload", -1.80351693112),
        ("vin", 4.248118960493e-3),
        ("vref", 2.291766555858),
        ("ibias", 1.025257331278e3),
    ] {
        assert_relative(row(result, name, "dc").absolute, expected, 1e-7, name);
    }
    // ngspice perturbs a resistor by a relative 1e-6.
    for (name, expected) in [
        ("rload", 2.076135507367e-4),
        ("rtop", 6.420261420185e-6),
        ("rb0", -8.58233152447e-6),
    ] {
        assert_relative(row(result, name, "r").absolute, expected, 1e-5, name);
    }
}

#[test]
fn ldo_load_current_row_matches_ngspice() {
    assert_matches_ngspice(&sens(&ldo(C5_LOAD, COMPARED, HELD_TRIM)));
}

/// Design M1: every device-instance and model-parameter row of the
/// unfiltered `.sens v(vout)` is an operating-point tangent, so none is
/// refused, and each matches ngspice-46's re-solved operating points.
///
/// The reference is ngspice re-solved: `alter`/`altermod` the parameter by
/// ±1e-3 of its value and difference two `op`s. ngspice's own `.sens` is not
/// the reference for these rows: on this circuit it disagrees with its own
/// re-solved operating points on 378 of 554 rows (`m1_w` is -637.6 V/m
/// against a re-solved -21128.6 V/m), while RSpice's tangents agree with
/// them to a median 4e-5 (worst 1e-2, the re-solve's own truncation) on the
/// 84 rows the re-solve resolves cleanly.
#[test]
fn ldo_unfiltered_rows_are_operating_point_tangents() {
    let result = sens(&ldo(C5_LOAD, ".sens v(vout)", HELD_TRIM));
    assert!(
        result.sensitivities.len() > 500,
        "every BSIM4 model and instance row resolves: {} rows",
        result.sensitivities.len()
    );
    assert_matches_ngspice(&result);
    for (name, parameter, expected) in [
        ("m1", "w", -2.1128569e4),
        ("m1", "l", 1.3433635e5),
        ("mp", "w", 2.0484692e3),
        ("nbsim4", "a0", -4.5134057e-4),
        ("nbsim4", "vth0", -2.9395412e-2),
        ("pbsim4", "vth0", 1.7567007e-2),
        ("pbsim4", "pclm", 9.2768694e-4),
        ("rref", "r", -8.2186814e-9),
    ] {
        assert_relative(
            row(&result, name, parameter).absolute,
            expected,
            1e-3,
            &format!("{name}:{parameter}"),
        );
    }
    // A1 = 0 is a kink of the model: a positive A1 leaves the residual bit
    // for bit where it was and a negative one moves it. The row is the
    // forward one, as SPICE's forward perturbation reports it, and named.
    let one_sided = |name: &str| {
        result
            .one_sided
            .iter()
            .any(|row| row.eq_ignore_ascii_case(name))
    };
    assert!(one_sided("NBSIM4:A1"), "{:?}", result.one_sided);
    assert_eq!(row(&result, "nbsim4", "a1").absolute, 0.0);
    // VFB is smooth but strongly curved: its one-sided differences at the
    // step differ by a fifth, and halve with the step.
    for name in ["NBSIM4:VTH0", "NBSIM4:VFB"] {
        assert!(!one_sided(name), "{name} is two-sided");
    }
    assert_relative(
        row(&result, "nbsim4", "vfb").absolute,
        2.6759911e-4,
        2e-3,
        "nbsim4:vfb",
    );
}

/// The corpus deck itself: the trim register is a Verilog module whose
/// outputs reach the ladder switches through built-in converters. Its
/// initial block holds code 0, the operating point's discrete state, and the
/// switch controls draw no current, so the rows are the held-trim rows.
#[cfg(feature = "veriloga")]
#[test]
fn ldo_rows_hold_the_trim_registers_discrete_state() {
    let result = sens(&ldo(
        C5_LOAD,
        COMPARED,
        ".hdl \"spi_trim.v\"\n.param vcc=1.8\n\
         vcsn csn 0 pwl(0 1.8 400u 1.8 400.01u 0 405u 0 405.01u 1.8)\n\
         vsdi sdi 0 pwl(0 0 400u 0 400.01u 1.8)\n\
         vsclk sclk 0 pwl(0 0 401u 0 401.01u 1.8 401.5u 1.8 401.51u 0)\n\
         xtrim sclk sdi csn tr3 tr2 tr1 tr0 spi_trim\n",
    ));
    assert_matches_ngspice(&result);
}

/// O5 for the unfiltered study: the trim register's rows are its held twin's
/// rows. Each residual tangent rebuilds the circuit, and the rebuilt host
/// runs its initial blocks and settles at the converged solution, so it
/// holds the same discrete state the operating point held.
#[cfg(feature = "veriloga")]
#[test]
fn ldo_unfiltered_rows_hold_the_trim_registers_discrete_state() {
    let held = sens(&ldo(C5_LOAD, ".sens v(vout)", HELD_TRIM));
    let mixed = sens(&ldo(
        C5_LOAD,
        ".sens v(vout)",
        ".hdl \"spi_trim.v\"\n.param vcc=1.8\n\
         vcsn csn 0 pwl(0 1.8 400u 1.8 400.01u 0 405u 0 405.01u 1.8)\n\
         vsdi sdi 0 pwl(0 0 400u 0 400.01u 1.8)\n\
         vsclk sclk 0 pwl(0 0 401u 0 401.01u 1.8 401.5u 1.8 401.51u 0)\n\
         xtrim sclk sdi csn tr3 tr2 tr1 tr0 spi_trim\n",
    ));
    let mut compared = 0;
    for twin in &held.sensitivities {
        let Some(row) = mixed
            .sensitivities
            .iter()
            .find(|row| row.vector_name.eq_ignore_ascii_case(&twin.vector_name))
        else {
            continue;
        };
        compared += 1;
        assert!(
            (row.absolute - twin.absolute).abs() <= 1e-12 * twin.absolute.abs().max(1e-300),
            "{}: {:e} held {:e}",
            twin.vector_name,
            row.absolute,
            twin.absolute
        );
    }
    assert!(compared > 500, "{compared} rows compared");
    assert_eq!(mixed.one_sided, held.one_sided);
}
