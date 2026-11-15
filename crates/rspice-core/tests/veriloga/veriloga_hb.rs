use num_complex::Complex64;
use rspice_core::analysis::PssConfig;
use rspice_core::analysis::harmonic_balance::HbConfig;
use rspice_core::engine::{Engine, SimulationConfig, TransientStartupMode};
use rspice_core::identity::AnalysisKind;
use rspice_core::netlist::Netlist;
use rspice_core::{SimulationError, UnsupportedCapabilityError};

fn write_model(name: &str, source: &str) -> String {
    let mut path = std::env::temp_dir();
    path.push(format!("rspice_hb_{}_{}.va", name, std::process::id()));
    std::fs::write(&path, source).expect("write temporary Verilog-A model");
    path.to_string_lossy().replace('\\', "/")
}

const F0: f64 = 1.0e6;

fn run_hb(deck: &str, harmonics: usize) -> rspice_core::engine::HbAnalysisResult {
    let netlist = Netlist::parse(deck).expect("parse");
    Engine::new(SimulationConfig::default())
        .run_hb(&netlist, HbConfig::new(F0).with_harmonics(harmonics))
        .unwrap_or_else(|error| panic!("HB must run: {error}\n{deck}"))
}

fn node(result: &rspice_core::engine::HbAnalysisResult, name: &str) -> Vec<Complex64> {
    result
        .result
        .spectral_voltages
        .iter()
        .find(|voltage| voltage.node_name.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("node {name} present"))
        .coefficients
        .clone()
}

const RESISTOR: (&str, &str) = (
    "va_hb_resistor",
    "parameter real r = 1.0e3;\n    analog I(p, n) <+ V(p, n) / r;",
);
const CAPACITOR: (&str, &str) = (
    "va_hb_capacitor",
    "parameter real c = 1.0e-12;\n    analog I(p, n) <+ ddt(c * V(p, n));",
);
const DIODE: (&str, &str) = (
    "va_hb_diode",
    "parameter real is = 1.0e-14;\n    analog I(p, n) <+ is * (exp(V(p, n) / $vt) - 1.0);",
);

/// One two-terminal module in a file of the calling test's own, and the
/// `.va` card that loads it.
fn two_terminal(test: &str, (module, body): (&str, &str)) -> (String, String) {
    let path = write_model(
        &format!("{test}_{module}"),
        &format!(
            "`include \"disciplines.vams\"\n\
             module {module}(p, n);\n\
             inout p, n;\n\
             electrical p, n;\n\
             {body}\n\
             endmodule\n"
        ),
    );
    let card = format!(".va \"{path}\" {module}");
    (path, card)
}

/// The same driven diode clipper with each native device swapped, in turn,
/// for its Verilog-A twin gives the native harmonics at every node.
///
/// The native harmonic-balance path is the reference, to its own Newton
/// tolerance: every coefficient agrees within 1e-6 of the largest one, which
/// is well inside the solver's convergence tolerances and far outside its
/// roundoff.
#[test]
fn veriloga_twins_of_native_devices_give_the_native_harmonics() {
    let models = [RESISTOR, CAPACITOR, DIODE].map(|model| two_terminal("twins", model));
    let cards = models
        .iter()
        .map(|(_, card)| card.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    let deck = |resistor: &str, capacitor: &str, diode: &str| {
        format!(
            "* driven diode clipper\n\
             V1 in 0 DC 0.3 SIN(0.3 0.4 1meg)\n\
             R1 in a 1k\n\
             {resistor}\n\
             {capacitor}\n\
             {diode}\n\
             .model dmod D IS=1e-14 N=1\n\
             {cards}\n\
             .end\n"
        )
    };
    let native = ("R2 a 0 10k", "C1 a 0 100p", "D1 a 0 dmod");
    let reference = run_hb(&deck(native.0, native.1, native.2), 12);
    assert!(reference.converged);
    let scale = node(&reference, "a")
        .iter()
        .chain(&node(&reference, "in"))
        .map(|value| value.norm())
        .fold(0.0, f64::max);
    for (twin, deck) in [
        (
            "resistor",
            deck("XR2 a 0 va_hb_resistor r=10k", native.1, native.2),
        ),
        (
            "capacitor",
            deck(native.0, "XC1 a 0 va_hb_capacitor c=100p", native.2),
        ),
        (
            "diode",
            deck(native.0, native.1, "XD1 a 0 va_hb_diode is=1e-14"),
        ),
    ] {
        let result = run_hb(&deck, 12);
        assert!(result.converged, "{twin}");
        for name in ["in", "a"] {
            for (k, (twin_value, native_value)) in node(&result, name)
                .iter()
                .zip(node(&reference, name))
                .enumerate()
            {
                assert!(
                    (twin_value - native_value).norm() <= 1.0e-6 * scale,
                    "{twin} twin, V({name}) harmonic {k}: {twin_value} against native {native_value}"
                );
            }
        }
    }
    for (path, _) in models {
        let _ = std::fs::remove_file(path);
    }
}

/// A Verilog-A capacitor behind a resistor is a first-order low-pass whose
/// fundamental is H(jω) = 1/(1 + jωRC) of the drive, to 1e-9.
#[test]
fn a_veriloga_rc_low_pass_matches_its_transfer_function() {
    let (model, card) = two_terminal("low_pass", CAPACITOR);
    let deck = format!(
        "* Verilog-A RC low-pass\n\
         V1 in 0 DC 0 SIN(0 1 1meg)\n\
         R1 in out 1k\n\
         XC1 out 0 va_hb_capacitor c=100p\n\
         {card}\n\
         .end\n"
    );
    let result = run_hb(&deck, 3);
    let _ = std::fs::remove_file(model);
    assert!(result.converged);
    let omega = std::f64::consts::TAU * F0;
    let drive = Complex64::new(0.0, -1.0);
    let expected = drive / Complex64::new(1.0, omega * 1.0e3 * 100.0e-12);
    let out = node(&result, "out");
    assert!(
        (out[1] - expected).norm() <= 1.0e-9 * expected.norm(),
        "V(out) fundamental {} against H(jω)·V(in) = {expected}",
        out[1]
    );
    for (k, harmonic) in out.iter().enumerate().skip(2) {
        assert!(
            harmonic.norm() <= 1.0e-12,
            "a linear filter has no harmonic {k}: {harmonic}"
        );
    }
}

/// A charge Q = C0·v + C1·v² driven by v = A·sin(ωt) draws
/// i = C0·A·ω·cos(ωt) + C1·A²·ω·sin(2ωt): the fundamental is C0·A·ω and the
/// second harmonic -j·C1·A²·ω, to 1e-9. The source carries the current the
/// other way.
#[test]
fn a_veriloga_varactor_draws_its_closed_form_harmonics() {
    let model = write_model(
        "varactor",
        r#"
`include "disciplines.vams"

module va_hb_varactor(p, n);
    inout p, n;
    electrical p, n;
    parameter real c0 = 1.0e-12;
    parameter real c1 = 0.5e-12;
    analog I(p, n) <+ ddt(c0 * V(p, n) + c1 * V(p, n) * V(p, n));
endmodule
"#,
    );
    let deck = format!(
        "* Verilog-A varactor on an ideal source\n\
         V1 in 0 DC 0 SIN(0 0.5 1meg)\n\
         XC1 in 0 va_hb_varactor\n\
         .va \"{model}\" va_hb_varactor\n\
         .end\n"
    );
    let result = run_hb(&deck, 4);
    let _ = std::fs::remove_file(model);
    assert!(result.converged);
    let (c0, c1, amplitude) = (1.0e-12, 0.5e-12, 0.5);
    let omega = std::f64::consts::TAU * F0;
    let source = &result
        .result
        .mna_branch_currents
        .iter()
        .find(|branch| branch.device_name.eq_ignore_ascii_case("V1"))
        .expect("V1 branch current")
        .coefficients;
    let expected = [
        Complex64::new(0.0, 0.0),
        Complex64::new(c0 * amplitude * omega, 0.0),
        Complex64::new(0.0, -c1 * amplitude * amplitude * omega),
        Complex64::new(0.0, 0.0),
    ];
    let scale = expected[1].norm();
    for (k, expected) in expected.iter().enumerate() {
        assert!(
            (source[k] + expected).norm() <= 1.0e-9 * scale,
            "I(V1) harmonic {k}: {} against -{expected}",
            source[k]
        );
    }
}

/// Every operator harmonic balance cannot carry is refused before solving,
/// by its own name, with the instance and module.
/// Each operator harmonic balance cannot carry is refused by name, with the
/// line the module wrote it on when the compiler located it: the analog body
/// is the sixth line of each module file.
#[test]
fn harmonic_balance_refuses_each_operator_family_by_name() {
    for (operator, body, located) in [
        (
            "transition",
            "I(out) <+ 1.0e-3 * transition(V(in) > 0.5 ? 1.0 : 0.0, 0, 1n);",
            true,
        ),
        ("slew", "I(out) <+ 1.0e-3 * slew(V(in), 1.0e9);", true),
        (
            "cross",
            "I(out) <+ V(out) / 1.0e3; @(cross(V(in) - 0.5, 1)) $discontinuity(0);",
            true,
        ),
        (
            "above",
            "I(out) <+ V(out) / 1.0e3; @(above(V(in) - 0.5)) $discontinuity(0);",
            true,
        ),
        (
            "timer",
            "I(out) <+ V(out) / 1.0e3; @(timer(0, 1u)) $discontinuity(0);",
            true,
        ),
        (
            "last_crossing",
            "I(out) <+ 1.0e3 * last_crossing(V(in) - 0.5, 1);",
            false,
        ),
        ("idtmod", "I(out) <+ 1.0e-3 * idtmod(V(in), 0, 1.0);", true),
        (
            "zi_nd",
            "I(out) <+ 1.0e-3 * zi_nd(V(in), '{1.0}, '{1.0, -0.5}, 1u);",
            true,
        ),
        // Nothing fixes the DC term of a filter with a pole at the origin.
        (
            "laplace_nd",
            "I(out) <+ 1.0e-3 * laplace_nd(V(in), '{1.0}, '{0.0, 1.0e-6});",
            true,
        ),
    ] {
        let module = "va_hb_refused";
        let name = format!(
            "refused_{}",
            operator.trim_matches(|c: char| !c.is_alphanumeric())
        );
        let model = write_model(
            &name,
            &format!(
                "`include \"disciplines.vams\"\n\
                 module {module}(in, out);\n\
                 inout in, out;\n\
                 electrical in, out;\n\
                 analog begin\n\
                 {body}\n\
                 end\n\
                 endmodule\n"
            ),
        );
        let deck = format!(
            "* harmonic balance refuses {operator}\n\
             V1 in 0 DC 0 SIN(0 1 1meg)\n\
             R1 out 0 1k\n\
             XVA in out {module}\n\
             .va \"{model}\" {module}\n\
             .end\n"
        );
        let netlist = Netlist::parse(&deck).expect("parse");
        let error = Engine::new(SimulationConfig::default())
            .run_hb(&netlist, HbConfig::new(F0).with_harmonics(2))
            .map(drop)
            .expect_err("HB must refuse the operator");
        let _ = std::fs::remove_file(&model);
        let SimulationError::UnsupportedCapability(refusal) = &error else {
            panic!("{operator}: expected a capability refusal, got {error}");
        };
        let UnsupportedCapabilityError {
            capability,
            analysis_kind,
            instance,
            module: refused_module,
            detail,
            ..
        } = refusal.as_ref();
        assert_eq!(
            *capability, "analysis.hb.veriloga_operator",
            "{operator}: {error}"
        );
        assert_eq!(
            *analysis_kind,
            Some(AnalysisKind::HarmonicBalance),
            "{error}"
        );
        assert_eq!(instance.as_deref(), Some("XVA"), "{error}");
        assert_eq!(refused_module.as_deref(), Some(module), "{error}");
        assert!(
            detail.contains("harmonic-balance analysis")
                && detail.contains(&format!("`{operator}`")),
            "{operator}: the refusal must name the analysis and the operator: {detail}"
        );
        if located {
            let file = model.rsplit('/').next().expect("file name");
            assert!(
                detail.contains(&format!("`{operator}` at {file}:6")),
                "{operator}: the refusal must name the source line: {detail}"
            );
        }
    }
}

/// The spectrum of `waveform`'s last period ending at `end`, sampled at
/// `samples` uniform points by linear interpolation: `c_0` the mean and
/// `c_k = (2/N) sum v(t_n) e^{-jk w t_n}`, the convention harmonic balance
/// reports.
fn period_spectrum(
    time: &[f64],
    waveform: &[f64],
    end: f64,
    samples: usize,
    harmonics: usize,
) -> Vec<Complex64> {
    let period = F0.recip();
    let start = end - period;
    let at = |t: f64| {
        let index = time
            .partition_point(|&sample| sample < t)
            .clamp(1, time.len() - 1);
        let (t0, t1) = (time[index - 1], time[index]);
        let (v0, v1) = (waveform[index - 1], waveform[index]);
        v0 + (v1 - v0) * (t - t0) / (t1 - t0)
    };
    (0..=harmonics)
        .map(|k| {
            let sum: Complex64 = (0..samples)
                .map(|n| {
                    let phase = n as f64 / samples as f64;
                    at(start + phase * period)
                        * Complex64::from_polar(1.0, -std::f64::consts::TAU * k as f64 * phase)
                })
                .sum();
            let scale = if k == 0 { 1.0 } else { 2.0 };
            sum * scale / samples as f64
        })
        .collect()
}

/// Every shape of `ddt` the residual cannot carry as the derivative of a
/// contributed charge becomes an unknown of its own, and its harmonics are
/// those of RSpice's own settled transient: one scaled by a quantity that
/// varies with bias, a nested one, one that steers a branch, one a
/// branch-current read sees, and one inside a nonlinear function, whose
/// charge nothing recovers.
///
/// Each module drives `out`, loaded by 1 kΩ and 50 pF, from `in`, driven by
/// one tone. Settle test and allowance as for the generated models' fixtures,
/// fixed before the first comparison: the transient starts from rest (UIC)
/// and runs 24 periods at T/512; the DC term and first three harmonics of
/// its last period differ from the period before by at most 1e-5 of the
/// fundamental, and harmonic balance's are within 1e-3 of the fundamental of
/// the transient's. 32 harmonics keep the truncation of the steered case's
/// `d|d|` (harmonics falling as 1/k^3) far inside the allowance.
#[test]
fn every_ddt_shape_reaches_its_settled_transient() {
    const PERIODS: f64 = 24.0;
    const SAMPLES: usize = 512;
    const HARMONICS: usize = 32;
    for (shape, declarations, body) in [
        (
            "bias_scaled",
            "",
            "I(out) <+ -(1.0 + 0.5 * V(in) * V(in)) * 1.0e-10 * ddt(V(in));",
        ),
        ("nested", "", "I(out) <+ -2.5e-17 * ddt(ddt(V(in)));"),
        (
            "steering",
            "real d;",
            "d = 1.0e-7 * ddt(V(in));\n\
             if (d > 0) I(out) <+ -1.0e-3 * d * d; else I(out) <+ 1.0e-3 * d * d;",
        ),
        (
            "flow_probe",
            "",
            "I(in, out) <+ 2.0e-11 * ddt(V(in, out));\n\
             I(out) <+ 0.5 * I(in, out);",
        ),
        (
            "unrecovered",
            "",
            "I(out) <+ -1.0e-3 * tanh(1.0e-7 * ddt(V(in)));",
        ),
    ] {
        let module = format!("va_hb_ddt_{shape}");
        let model = write_model(
            &format!("ddt_{shape}"),
            &format!(
                "`include \"disciplines.vams\"\n\
                 module {module}(in, out);\n\
                 inout in, out;\n\
                 electrical in, out;\n\
                 {declarations}\n\
                 analog begin\n\
                 {body}\n\
                 end\n\
                 endmodule\n"
            ),
        );
        let deck = format!(
            "* harmonic balance carries a {shape} ddt\n\
             V1 in 0 DC 0 SIN(0.5 1 1meg)\n\
             R1 out 0 1k\n\
             C1 out 0 50p\n\
             XVA in out {module}\n\
             .va \"{model}\" {module}\n\
             .end\n"
        );
        let hb = node(&run_hb(&deck, HARMONICS), "out");
        let netlist = Netlist::parse(&deck).expect("parse");
        let tstop = PERIODS / F0;
        let tran = Engine::new(SimulationConfig::default())
            .run_tran_with_startup_mode_and_abort(
                &netlist,
                tstop,
                1.0 / (F0 * SAMPLES as f64),
                TransientStartupMode::Uic,
                &rspice_core::NoAbort,
            )
            .unwrap_or_else(|error| panic!("{shape}: transient must run: {error}"));
        let _ = std::fs::remove_file(&model);
        let waveform = tran
            .try_voltage_waveform_named("out")
            .unwrap_or_else(|| panic!("{shape}: V(out) present"));
        let last = period_spectrum(&tran.time, waveform, tstop, SAMPLES, 3);
        let before = period_spectrum(&tran.time, waveform, tstop - 1.0 / F0, SAMPLES, 3);
        let fundamental = last[1].norm();
        for k in 0..=3 {
            assert!(
                (last[k] - before[k]).norm() <= 1e-5 * fundamental,
                "{shape}: harmonic {k} has not settled: {} then {}",
                before[k],
                last[k]
            );
            assert!(
                (hb[k] - last[k]).norm() <= 1e-3 * fundamental,
                "{shape}: harmonic {k}: harmonic balance {} against the settled transient {} \
                 (fundamental {fundamental})",
                hb[k],
                last[k]
            );
        }
        eprintln!(
            "{shape}: worst {:.3e} of the fundamental {fundamental:.6}",
            (0..=3)
                .map(|k| (hb[k] - last[k]).norm() / fundamental)
                .fold(0.0, f64::max)
        );
    }
}

/// A module with one operator site between `in` and `out`, loaded by a
/// 1 kΩ resistor: the module drives `-1 mA` per unit of `expression`, so
/// `V(out)` is the expression.
fn operator_deck(test: &str, drive: &str, expression: &str) -> (String, String) {
    let module = format!("va_hb_{test}");
    let model = write_model(
        test,
        &format!(
            "`include \"disciplines.vams\"\n\
             module {module}(in, out);\n\
             inout in, out;\n\
             electrical in, out;\n\
             analog I(out) <+ -1.0e-3 * {expression};\n\
             endmodule\n"
        ),
    );
    let deck = format!(
        "* harmonic balance carries {test}\n\
         V1 in 0 DC 0 {drive}\n\
         R1 out 0 1k\n\
         XVA in out {module}\n\
         .va \"{model}\" {module}\n\
         .end\n"
    );
    (model, deck)
}

/// Each harmonic of `V(out)` against its closed form, to 1e-9 of the
/// largest expected coefficient (a missing entry is zero).
fn assert_spectrum(case: &str, actual: &[Complex64], expected: &[Complex64]) {
    let scale = expected
        .iter()
        .map(|value| value.norm())
        .fold(0.0, f64::max);
    for (k, value) in actual.iter().enumerate() {
        let want = expected.get(k).copied().unwrap_or_default();
        assert!(
            (value - want).norm() <= 1.0e-9 * scale,
            "{case}: harmonic {k} is {value}, closed form {want}"
        );
    }
}

/// `idt(V(in), ic)` of `sin(ωt)` is `-cos(ωt)/ω` at the fundamental, with
/// its initial condition as the DC term, to 1e-9.
#[test]
fn an_idt_with_an_initial_condition_integrates_one_tone() {
    let (model, deck) = operator_deck("integrator", "SIN(0 1 1meg)", "1.0e6 * idt(V(in), 2.5e-7)");
    let result = run_hb(&deck, 4);
    let _ = std::fs::remove_file(model);
    assert!(result.converged);
    let omega = std::f64::consts::TAU * F0;
    assert_spectrum(
        "idt",
        &node(&result, "out"),
        &[
            Complex64::new(0.25, 0.0),
            Complex64::new(-1.0e6 / omega, 0.0),
        ],
    );
}

/// An `idt` whose initial condition depends on the unknowns,
/// `idt(w0 (V(in) - V(out)), V(in))` driving `V(out)`: a periodic orbit has
/// no start time for the condition to act at, so harmonic balance holds the
/// integrand's mean at zero and the loop sets the DC term. `V(out)` is the
/// one-pole low-pass `w0 / (jω + w0)` of `0.5 + sin(ωt)`, to 1e-9.
#[test]
fn an_idt_whose_initial_condition_moves_with_the_orbit_holds_its_input_mean_at_zero() {
    let (model, deck) = operator_deck(
        "bias_dependent_integrator",
        "SIN(0.5 1 1meg)",
        "idt(1.0e6 * (V(in) - V(out)), V(in))",
    );
    let result = run_hb(&deck, 4);
    let _ = std::fs::remove_file(model);
    assert!(result.converged);
    let omega = std::f64::consts::TAU * F0;
    let pole = Complex64::new(1.0e6, 0.0);
    assert_spectrum(
        "bias-dependent idt",
        &node(&result, "out"),
        &[
            Complex64::new(0.5, 0.0),
            Complex64::new(0.0, -1.0) * pole / (Complex64::new(0.0, omega) + pole),
        ],
    );
}

/// An `idt` whose bias-dependent initial condition picks one of the two
/// roots of its integrand, `idt(-k (V(out) - V(in)) (V(out) + V(in)/5), V(in))`
/// driving `V(out)`: at `V(out) = V(in)`, where the condition puts it, it is
/// stable, and at `-V(in)/5` it runs away. Harmonic balance's DC seed is the
/// circuit's operating point, where LRM 2.4 section 4.5.4 holds the output
/// at its initial condition, so the periodic solve starts on the stable root
/// and its harmonics are the settled transient's, within 1e-3 of the
/// fundamental. A seed that held the integrand's mean at zero from a zero
/// start took the other root: a DC term of -0.1 V against the transient's
/// 0.5 V.
#[test]
fn a_bias_dependent_idt_starts_the_orbit_from_its_initial_condition() {
    const PERIODS: f64 = 24.0;
    const SAMPLES: usize = 512;
    let (model, deck) = operator_deck(
        "two_root_integrator",
        "SIN(0.5 0.05 1meg)",
        "idt(-2.0e7 * (V(out) - V(in)) * (V(out) + 0.2 * V(in)), V(in))",
    );
    let hb = node(&run_hb(&deck, 16), "out");
    let netlist = Netlist::parse(&deck).expect("parse");
    let tstop = PERIODS / F0;
    let tran = Engine::new(SimulationConfig::default())
        .run_tran_with_startup_mode_and_abort(
            &netlist,
            tstop,
            1.0 / (F0 * SAMPLES as f64),
            TransientStartupMode::Uic,
            &rspice_core::NoAbort,
        )
        .unwrap_or_else(|error| panic!("the transient must run: {error}"));
    let _ = std::fs::remove_file(&model);
    let waveform = tran.try_voltage_waveform_named("out").expect("V(out)");
    let last = period_spectrum(&tran.time, waveform, tstop, SAMPLES, 3);
    let before = period_spectrum(&tran.time, waveform, tstop - 1.0 / F0, SAMPLES, 3);
    let fundamental = last[1].norm();
    for k in 0..=3 {
        assert!(
            (last[k] - before[k]).norm() <= 1e-5 * fundamental,
            "harmonic {k} has not settled: {} then {}",
            before[k],
            last[k]
        );
        assert!(
            (hb[k] - last[k]).norm() <= 1e-3 * fundamental,
            "harmonic {k}: harmonic balance {} against the settled transient {} \
             (fundamental {fundamental})",
            hb[k],
            last[k]
        );
    }
}

/// `$abstime` reads each sample's instant. A tone at the analysis's own
/// frequency, scaled by the drive, gives its closed-form harmonics to 1e-9:
/// `(1 + sin(ωt)/2) sin(ωt) = 1/4 + sin(ωt) - cos(2ωt)/4`. A ramp, whose
/// equations one period later differ, is refused before solving by module
/// and by what it reads.
#[test]
fn abstime_reads_each_samples_instant_and_a_ramp_is_refused() {
    let (model, deck) = operator_deck(
        "abstime_tone",
        "SIN(0 1 1meg)",
        "(1.0 + 0.5 * V(in)) * sin(6.283185307179586e6 * $abstime)",
    );
    let result = run_hb(&deck, 4);
    let _ = std::fs::remove_file(model);
    assert!(result.converged);
    assert_spectrum(
        "$abstime tone",
        &node(&result, "out"),
        &[
            Complex64::new(0.25, 0.0),
            Complex64::new(0.0, -1.0),
            Complex64::new(-0.25, 0.0),
        ],
    );

    let (model, deck) = operator_deck("abstime_ramp", "SIN(0 1 1meg)", "1.0e6 * $abstime");
    let netlist = Netlist::parse(&deck).expect("parse");
    let error = Engine::new(SimulationConfig::default())
        .run_hb(&netlist, HbConfig::new(F0).with_harmonics(2))
        .map(drop)
        .expect_err("HB must refuse a ramp in time");
    let _ = std::fs::remove_file(model);
    let message = error.to_string();
    assert!(
        message.contains("module 'va_hb_abstime_ramp'")
            && message.contains("reads `$abstime`")
            && message.contains("not periodic over the analysis period"),
        "the refusal must name the module and what it reads: {message}"
    );
}

/// The transfer of a rational `laplace_nd`, as its coefficients give it.
fn rational(numerator: &[f64], denominator: &[f64], omega: f64) -> Complex64 {
    let s = Complex64::new(0.0, omega);
    let polynomial = |coefficients: &[f64]| {
        coefficients
            .iter()
            .rev()
            .fold(Complex64::default(), |sum, &c| sum * s + c)
    };
    polynomial(numerator) / polynomial(denominator)
}

fn literal(coefficients: &[f64]) -> String {
    let terms = coefficients
        .iter()
        .map(|c| format!("{c:e}"))
        .collect::<Vec<_>>()
        .join(", ");
    format!("'{{{terms}}}")
}

/// A first-order `laplace_nd` low-pass of one tone, and a second-order
/// band-pass of `V(in)²` (DC and the second harmonic), give H(jkω) times
/// each harmonic of their input, to 1e-9.
#[test]
fn rational_laplace_filters_match_their_transfer_at_each_harmonic() {
    let omega = std::f64::consts::TAU * F0;
    let (numerator, denominator) = ([1.0], [1.0, 1.0e-7]);
    let (model, deck) = operator_deck(
        "laplace_low_pass",
        "SIN(0 1 1meg)",
        &format!(
            "laplace_nd(V(in), {}, {})",
            literal(&numerator),
            literal(&denominator)
        ),
    );
    let result = run_hb(&deck, 4);
    let _ = std::fs::remove_file(model);
    assert!(result.converged);
    assert_spectrum(
        "low-pass",
        &node(&result, "out"),
        &[
            Complex64::default(),
            rational(&numerator, &denominator, omega) * Complex64::new(0.0, -1.0),
        ],
    );

    // ω0 = 2π·1.5 MHz, Q = 2, normalized: H(s) = (s/(ω0·Q)) / (1 + s/(ω0·Q) + s²/ω0²).
    let w0 = std::f64::consts::TAU * 1.5e6;
    let (numerator, denominator) = (
        [0.0, 1.0 / (w0 * 2.0)],
        [1.0, 1.0 / (w0 * 2.0), 1.0 / (w0 * w0)],
    );
    let (model, deck) = operator_deck(
        "laplace_band_pass",
        "SIN(0 1 1meg)",
        &format!(
            "laplace_nd(V(in) * V(in), {}, {})",
            literal(&numerator),
            literal(&denominator)
        ),
    );
    let result = run_hb(&deck, 4);
    let _ = std::fs::remove_file(model);
    assert!(result.converged);
    // sin²(ωt) = 1/2 - cos(2ωt)/2.
    assert_spectrum(
        "band-pass",
        &node(&result, "out"),
        &[
            rational(&numerator, &denominator, 0.0) * 0.5,
            Complex64::default(),
            rational(&numerator, &denominator, 2.0 * omega) * -0.5,
        ],
    );
}

/// `absdelay` by τ multiplies harmonic k of its input by e^{-jkωτ}: the input
/// `V(in) + V(in)²` carries DC, the fundamental and the second harmonic.
#[test]
fn an_absdelay_delays_each_harmonic_by_its_phase() {
    let tau = 2.0e-7;
    let (model, deck) = operator_deck(
        "delay",
        "SIN(0 1 1meg)",
        &format!("absdelay(V(in) + V(in) * V(in), {tau:e})"),
    );
    let result = run_hb(&deck, 4);
    let _ = std::fs::remove_file(model);
    assert!(result.converged);
    let omega = std::f64::consts::TAU * F0;
    let delayed = |k: f64, x: Complex64| x * Complex64::from_polar(1.0, -k * omega * tau);
    assert_spectrum(
        "absdelay",
        &node(&result, "out"),
        &[
            delayed(0.0, Complex64::new(0.5, 0.0)),
            delayed(1.0, Complex64::new(0.0, -1.0)),
            delayed(2.0, Complex64::new(-0.5, 0.0)),
        ],
    );
}

/// A `laplace_nd` low-pass behind a diode clipper gives the harmonics of its
/// native twin, a buffered RC with the same time constant, at every node: the
/// clipped waveform puts every harmonic through the filter. The native path
/// is the reference, to 1e-6 of the largest coefficient as for the device
/// twins.
#[test]
fn a_veriloga_laplace_low_pass_gives_its_native_rc_twins_harmonics() {
    let module = "va_hb_laplace_twin";
    let model = write_model(
        "laplace_twin",
        &format!(
            "`include \"disciplines.vams\"\n\
             module {module}(in, out);\n\
             inout in, out;\n\
             electrical in, out;\n\
             analog I(out) <+ -1.0e-3 * laplace_nd(V(in), '{{1.0}}, '{{1.0, 1.0e-7}});\n\
             endmodule\n"
        ),
    );
    let deck = |filter: &str| {
        format!(
            "* clipped tone through a low-pass\n\
             V1 in 0 DC 0.3 SIN(0.3 0.8 1meg)\n\
             R1 in a 1k\n\
             D1 a 0 dmod\n\
             {filter}\n\
             .model dmod D IS=1e-14 N=1\n\
             .va \"{model}\" {module}\n\
             .end\n"
        )
    };
    let reference = run_hb(&deck("E1 b 0 a 0 1\nR2 b out 1k\nC2 out 0 100p"), 12);
    let twin = run_hb(&deck(&format!("XVA a out {module}\nR2 out 0 1k")), 12);
    let _ = std::fs::remove_file(&model);
    assert!(reference.converged && twin.converged);
    let scale = ["in", "a", "out"]
        .iter()
        .flat_map(|name| node(&reference, name))
        .map(|value| value.norm())
        .fold(0.0, f64::max);
    for name in ["a", "out"] {
        for (k, (twin_value, native_value)) in node(&twin, name)
            .iter()
            .zip(node(&reference, name))
            .enumerate()
        {
            assert!(
                (twin_value - native_value).norm() <= 1.0e-6 * scale,
                "V({name}) harmonic {k}: {twin_value} against native {native_value}"
            );
        }
    }
}

/// A variable read before it is assigned keeps its value from the last
/// accepted time point, which no harmonic-balance spectrum carries. Harmonic
/// balance refuses the instance before solving, as SpectreRF does, and names
/// every such variable so the author knows which to assign. PSS carries them
/// as coordinates of event shooting's period map: it solves the hold, and
/// refuses by name the counter of accepted points, which changes at every
/// instant of the period.
#[test]
fn periodic_analyses_refuse_retained_variables_by_name_before_solving() {
    for (module, body, named) in [
        (
            "va_periodic_retained",
            "real points, changes, was_above, above, held[0:1];
            analog begin
                points = points + 1;
                above = V(sense) > 0.5;
                if (above != was_above)
                    changes = changes + 1;
                was_above = above;
                if (above)
                    held[1] = V(sense);
                I(out) <+ (points + changes + held[1]) * V(out);
            end",
            "`points`, `changes`, `was_above` and `held` are read before they are assigned, \
             so they keep their values from the last accepted time point",
        ),
        (
            "va_periodic_hold",
            "real held;
            analog begin
                if (V(sense) > 0.5)
                    held = V(sense);
                I(out) <+ held * V(out);
            end",
            "`held` is read before it is assigned, so it keeps its value from the last \
             accepted time point",
        ),
    ] {
        let model = write_model(
            module,
            &format!(
                "`include \"disciplines.vams\"\n\
                 module {module}(sense, out);\n\
                 inout sense, out;\n\
                 electrical sense, out;\n\
                 {body}\n\
                 endmodule\n"
            ),
        );
        let deck = format!(
            "* periodic analyses refuse retained Verilog-A variables\n\
             V1 sense 0 DC 0 SIN(0 1 1meg)\n\
             R1 out 0 1k\n\
             XVA sense out {module}\n\
             .va \"{model}\" {module}\n\
             .end\n"
        );
        let netlist = Netlist::parse(&deck).expect("parse");
        let engine = Engine::new(SimulationConfig::default());
        let pss = engine.run_pss(&netlist, PssConfig::new(1.0e6).with_harmonics(2));
        if module == "va_periodic_retained" {
            let error = pss
                .map(drop)
                .expect_err("PSS must refuse a counter of accepted points");
            let SimulationError::UnsupportedCapability(refusal) = &error else {
                panic!("{module}: expected a capability refusal, got {error}");
            };
            assert_eq!(
                refusal.capability, "analysis.pss.no_quiet_boundary",
                "{error}"
            );
            assert!(refusal.detail.contains("XVA.points"), "{error}");
        } else {
            pss.unwrap_or_else(|error| {
                panic!("{module}: event shooting carries the hold: {error}")
            });
        }
        let error = engine
            .run_hb(&netlist, HbConfig::new(1.0e6).with_harmonics(2))
            .map(drop)
            .expect_err("HB must refuse a retained variable");
        let SimulationError::UnsupportedCapability(refusal) = &error else {
            panic!("{module}: expected a capability refusal, got {error}");
        };
        let UnsupportedCapabilityError {
            capability,
            analysis_kind,
            instance,
            module: refused_module,
            detail,
            ..
        } = refusal.as_ref();
        assert_eq!(*capability, "analysis.hb.veriloga_operator", "{error}");
        assert_eq!(
            *analysis_kind,
            Some(AnalysisKind::HarmonicBalance),
            "{error}"
        );
        assert_eq!(instance.as_deref(), Some("XVA"), "{error}");
        assert_eq!(refused_module.as_deref(), Some(module), "{error}");
        assert!(
            detail.contains("harmonic-balance analysis") && detail.contains(named),
            "{module}: the refusal must name the analysis and the retained variables: {detail}"
        );
        let _ = std::fs::remove_file(model);
    }
}

/// Periodic AC from the small-signal source `vrf` into sidebands -2..=2 of
/// `V(a)` at three offsets, and the periodic transfer function from it into
/// the upper first sideband, on the deck's harmonic-balance carrier: once
/// solved by periodic AC itself and once retained from `.hb`.
fn hb_carrier_transfers(deck: &str) -> Vec<Complex64> {
    use rspice_core::NoAbort;
    use rspice_core::analysis::pac::{PacConfig, PacSweepType};
    use rspice_core::netlist::{FreqVariation, PeriodicSourceSelector, PeriodicSweep, PxfCard};
    let (start, stop) = (0.13 * F0, 0.37 * F0);
    let netlist = Netlist::parse(deck).expect("parse");
    let engine = Engine::new(SimulationConfig::default());
    let config = PacConfig::new()
        .with_fundamental(F0)
        .with_sweep(start, stop, 3)
        .with_sweep_type(PacSweepType::Linear)
        .with_sidebands(-2, 2)
        .with_input_source("vrf")
        .with_output_node("a");
    let card = PxfCard {
        sweep: PeriodicSweep {
            variation: FreqVariation::Lin,
            points: 3,
            start_freq: start,
            stop_freq: stop,
        },
        input_source: "vrf".into(),
        input_sideband: 0,
        output_node: "a".into(),
        output_ref: None,
        output_sideband: 1,
        max_sideband: 2,
        reltol: 1e-9,
        abstol: 1e-15,
        source: PeriodicSourceSelector::Preceding,
    };
    let hb = engine
        .run_hb(&netlist, HbConfig::new(F0).with_harmonics(8))
        .unwrap_or_else(|error| panic!("HB must run: {error}\n{deck}"));
    let fresh = engine
        .run_pac(&netlist, config.clone())
        .unwrap_or_else(|error| panic!("PAC must run: {error}\n{deck}"));
    let retained = engine
        .run_pac_from_hb_with_abort(&netlist, config, &hb.operating_point, &NoAbort)
        .unwrap_or_else(|error| panic!("PAC on the HB carrier must run: {error}\n{deck}"));
    let pxf = engine
        .run_pxf_card_from_hb_with_abort(&netlist, &card, &hb.operating_point, &NoAbort)
        .unwrap_or_else(|error| panic!("PXF on the HB carrier must run: {error}\n{deck}"));
    let mut transfers = Vec::new();
    for pac in [&fresh, &retained] {
        for offset in 0..3 {
            for sideband in -2..=2 {
                transfers.push(
                    pac.result
                        .conversion_matrix
                        .get(offset, sideband, 0)
                        .expect("sideband present"),
                );
            }
        }
    }
    transfers.extend(pxf.points.iter().map(|point| point.transfer));
    assert_eq!(transfers.len(), 2 * 3 * 5 + 3);
    transfers
}

/// The pumped diode mixer periodic AC and PXF linearize, with its diode
/// native or written in Verilog-A.
fn pumped_mixer(diode: &str, cards: &str) -> String {
    format!(
        "* pumped diode mixer\n\
         VLO lo 0 DC 0.6 SIN(0.6 0.4 1meg)\n\
         R1 lo a 1k\n\
         VRF rf 0 DC 0\n\
         R2 rf a 10k\n\
         C1 a 0 100p\n\
         {diode}\n\
         .model dmod D IS=1e-14 N=1\n\
         {cards}\n\
         .end\n"
    )
}

/// Periodic AC and periodic transfer functions on a harmonic-balance carrier
/// linearize harmonic balance's own samples of a Verilog-A module, so a
/// pumped Verilog-A diode converts a small signal into every sideband as its
/// native twin does. Reference: the native diode on the same carrier.
/// Allowance, as for the large-signal twins: 1e-6 of the largest native
/// transfer, at each of three offsets, on a carrier periodic AC solves itself
/// and on one retained from `.hb`.
#[test]
fn a_veriloga_diode_mixes_its_native_twins_sidebands_on_a_harmonic_balance_carrier() {
    let (model, card) = two_terminal("pac_twin", DIODE);
    let native = hb_carrier_transfers(&pumped_mixer("D1 a 0 dmod", ""));
    let twin = hb_carrier_transfers(&pumped_mixer("XD1 a 0 va_hb_diode is=1e-14", &card));
    let scale = native.iter().map(|value| value.norm()).fold(0.0, f64::max);
    for (index, (twin_value, native_value)) in twin.iter().zip(&native).enumerate() {
        assert!(
            (twin_value - native_value).norm() <= 1.0e-6 * scale,
            "transfer {index}: Verilog-A {twin_value} against native {native_value}"
        );
    }
    // Sideband -1 and +1 of the first offset: the carrier must mix.
    assert!(
        native[1].norm() > 1e-3 * scale && native[3].norm() > 1e-3 * scale,
        "the pumped diode must convert into the first sidebands: {native:?}"
    );
    let _ = std::fs::remove_file(model);
}

/// Periodic AC from `vrf` into sidebands -2..=2 of `V(a)` at three offsets,
/// and the periodic transfer function into the upper first sideband, on the
/// deck's shooting (PSS) carrier.
fn shooting_carrier_transfers(deck: &str) -> Vec<Complex64> {
    use rspice_core::NoAbort;
    use rspice_core::analysis::pac::{PacConfig, PacSweepType};
    use rspice_core::netlist::{FreqVariation, PeriodicSourceSelector, PeriodicSweep, PxfCard};
    let (start, stop) = (0.13 * F0, 0.37 * F0);
    let netlist = Netlist::parse(deck).expect("parse");
    let engine = Engine::new(SimulationConfig::default());
    let point = engine
        .run_pss_operating_point_with_abort(
            &netlist,
            PssConfig::new(F0)
                .with_harmonics(8)
                .with_points_per_period(256)
                .with_tolerance(1e-9),
            &NoAbort,
        )
        .unwrap_or_else(|error| panic!("shooting must run: {error}\n{deck}"));
    let config = PacConfig::new()
        .with_fundamental(F0)
        .with_sweep(start, stop, 3)
        .with_sweep_type(PacSweepType::Linear)
        .with_sidebands(-2, 2)
        .with_input_source("vrf")
        .with_output_node("a");
    let card = PxfCard {
        sweep: PeriodicSweep {
            variation: FreqVariation::Lin,
            points: 3,
            start_freq: start,
            stop_freq: stop,
        },
        input_source: "vrf".into(),
        input_sideband: 0,
        output_node: "a".into(),
        output_ref: None,
        output_sideband: 1,
        max_sideband: 2,
        reltol: 1e-9,
        abstol: 1e-15,
        source: PeriodicSourceSelector::Preceding,
    };
    let pac = engine
        .run_pac_from_pss_with_abort(&netlist, config, &point, &NoAbort)
        .unwrap_or_else(|error| panic!("PAC on the shooting carrier must run: {error}\n{deck}"));
    let pxf = engine
        .run_pxf_card_from_pss_with_abort(&netlist, &card, &point, &NoAbort)
        .unwrap_or_else(|error| panic!("PXF on the shooting carrier must run: {error}\n{deck}"));
    let mut transfers = Vec::new();
    for offset in 0..3 {
        for sideband in -2..=2 {
            transfers.push(
                pac.result
                    .conversion_matrix
                    .get(offset, sideband, 0)
                    .expect("sideband present"),
            );
        }
    }
    transfers.extend(pxf.points.iter().map(|point| point.transfer));
    assert_eq!(transfers.len(), 3 * 5 + 3);
    transfers
}

/// A shooting carrier now carries a runtime module whose state is the
/// charges it contributes, and periodic AC and periodic transfer functions
/// linearize its projected orbit through harmonic balance's own samples, so
/// the pumped Verilog-A diode gives every sideband transfer its native twin
/// gives on that carrier. Allowance: 1e-6 of the largest native transfer, as
/// on the harmonic-balance carrier.
#[test]
fn a_veriloga_diode_mixes_its_native_twins_sidebands_on_a_shooting_carrier() {
    let (model, card) = two_terminal("pss_carrier", DIODE);
    let native = shooting_carrier_transfers(&pumped_mixer("D1 a 0 dmod", ""));
    let twin = shooting_carrier_transfers(&pumped_mixer("XD1 a 0 va_hb_diode is=1e-14", &card));
    let _ = std::fs::remove_file(model);
    let scale = native.iter().map(|value| value.norm()).fold(0.0, f64::max);
    for (index, (twin_value, native_value)) in twin.iter().zip(&native).enumerate() {
        assert!(
            (twin_value - native_value).norm() <= 1.0e-6 * scale,
            "transfer {index}: Verilog-A {twin_value} against native {native_value}"
        );
    }
    assert!(
        native[1].norm() > 1e-3 * scale && native[3].norm() > 1e-3 * scale,
        "the pumped diode must convert into the first sidebands: {native:?}"
    );
}

/// Periodic AC and periodic transfer functions read a runtime module's
/// stored flux on the shooting orbit that carries it as a coordinate: the
/// Verilog-A inductor [`flux_inductor`] in the pumped stage gives every
/// sideband transfer its native twin `L1 a 0 100u` gives on that carrier.
/// Allowance: 1e-6 of the largest native transfer, as for the diode.
#[test]
fn a_veriloga_inductor_gives_its_native_twins_sidebands_on_a_shooting_carrier() {
    let model = flux_inductor("pss_carrier_inductor");
    let native = shooting_carrier_transfers(&pumped_mixer("L1 a 0 100u", ""));
    let twin = shooting_carrier_transfers(&pumped_mixer(
        "XL1 a 0 va_pss_inductor",
        &format!(".va \"{model}\" va_pss_inductor"),
    ));
    let _ = std::fs::remove_file(model);
    let scale = native.iter().map(|value| value.norm()).fold(0.0, f64::max);
    for (index, (twin_value, native_value)) in twin.iter().zip(&native).enumerate() {
        assert!(
            (twin_value - native_value).norm() <= 1.0e-6 * scale,
            "transfer {index}: Verilog-A {twin_value} against native {native_value}"
        );
    }
}

/// Shooting takes a runtime module whose state is the charges it contributes,
/// integrating them over the period from the node voltages the period starts
/// at, and reaches RSpice's own settled transient: a nonlinear varactor with
/// a parallel conductance, and a series RC whose charge sits on a node the
/// module declares internally.
///
/// Each module loads `a`, driven from one tone through 1 kΩ. Settle test and
/// allowance as for the generated models' fixtures, fixed before the first
/// comparison: the transient starts from rest (UIC) and runs 24 periods at
/// T/512; the DC term and first three harmonics of its last period differ
/// from the period before by at most 1e-5 of the fundamental, and shooting's
/// are within 1e-3 of the fundamental of the transient's.
#[test]
fn a_charge_form_module_reaches_its_settled_transient_by_shooting() {
    const PERIODS: f64 = 24.0;
    const SAMPLES: usize = 512;
    for (shape, declarations, body) in [
        (
            "varactor",
            "",
            "I(p, n) <+ V(p, n) / 2.0e3 + ddt(20.0e-12 * V(p, n) + 10.0e-12 * V(p, n) * V(p, n));",
        ),
        (
            "internal_rc",
            "electrical x;",
            "I(p, x) <+ V(p, x) / 500.0;\n\
             I(x, n) <+ ddt(100.0e-12 * V(x, n));",
        ),
    ] {
        let module = format!("va_pss_{shape}");
        let model = write_model(
            &format!("pss_{shape}"),
            &format!(
                "`include \"disciplines.vams\"\n\
                 module {module}(p, n);\n\
                 inout p, n;\n\
                 electrical p, n;\n\
                 {declarations}\n\
                 analog begin\n\
                 {body}\n\
                 end\n\
                 endmodule\n"
            ),
        );
        let deck = format!(
            "* shooting integrates a {shape} module's charges\n\
             V1 in 0 DC 0 SIN(0.5 1 1meg)\n\
             R1 in a 1k\n\
             XVA a 0 {module}\n\
             .va \"{model}\" {module}\n\
             .end\n"
        );
        let netlist = Netlist::parse(&deck).expect("parse");
        let engine = Engine::new(SimulationConfig::default());
        let pss = engine
            .run_pss(
                &netlist,
                PssConfig::new(F0)
                    .with_points_per_period(SAMPLES)
                    .with_tolerance(1e-9),
            )
            .unwrap_or_else(|error| panic!("{shape}: shooting must run: {error}"))
            .result;
        let output = pss
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("a"))
            .unwrap_or_else(|| panic!("{shape}: V(a) present"));
        let end = *pss.time.last().expect("a periodic waveform");
        let shooting = period_spectrum(&pss.time, &pss.waveforms[output].values, end, SAMPLES, 3);
        let tstop = PERIODS / F0;
        let tran = engine
            .run_tran_with_startup_mode_and_abort(
                &netlist,
                tstop,
                1.0 / (F0 * SAMPLES as f64),
                TransientStartupMode::Uic,
                &rspice_core::NoAbort,
            )
            .unwrap_or_else(|error| panic!("{shape}: transient must run: {error}"));
        let _ = std::fs::remove_file(&model);
        let waveform = tran
            .try_voltage_waveform_named("a")
            .unwrap_or_else(|| panic!("{shape}: V(a) present"));
        let last = period_spectrum(&tran.time, waveform, tstop, SAMPLES, 3);
        let before = period_spectrum(&tran.time, waveform, tstop - 1.0 / F0, SAMPLES, 3);
        let fundamental = last[1].norm();
        for k in 0..=3 {
            assert!(
                (last[k] - before[k]).norm() <= 1e-5 * fundamental,
                "{shape}: harmonic {k} has not settled: {} then {}",
                before[k],
                last[k]
            );
            assert!(
                (shooting[k] - last[k]).norm() <= 1e-3 * fundamental,
                "{shape}: harmonic {k}: shooting {} against the settled transient {} \
                 (fundamental {fundamental})",
                shooting[k],
                last[k]
            );
        }
        eprintln!(
            "{shape}: shooting worst {:.3e} of the fundamental {fundamental:.6}",
            (0..=3)
                .map(|k| (shooting[k] - last[k]).norm() / fundamental)
                .fold(0.0, f64::max)
        );
    }
}

/// The inductance of [`flux_inductor`].
const FLUX_INDUCTANCE: f64 = 100.0e-6;

/// Write a runtime inductor that stores flux, `V(p, n) <+ ddt(L·I(p, n))`
/// with L = [`FLUX_INDUCTANCE`], as module `va_pss_inductor`.
fn flux_inductor(name: &str) -> String {
    write_model(
        name,
        &format!(
            "`include \"disciplines.vams\"\n\
             module va_pss_inductor(p, n);\n\
             inout p, n;\n\
             electrical p, n;\n\
             analog V(p, n) <+ ddt({FLUX_INDUCTANCE:e} * I(p, n));\n\
             endmodule\n"
        ),
    )
}

/// Shooting carries a runtime module's stored flux: an inductor written as
/// `V(p, n) <+ ddt(L·I(p, n))`, driven through a resistor, gives at the node
/// between them the fundamental of the closed form `jωL/(R + jωL)` and no
/// other harmonic. Allowance: 1e-3 of the fundamental, as for the
/// charge-form modules.
#[test]
fn a_flux_form_module_reaches_its_closed_form_by_shooting() {
    const SAMPLES: usize = 512;
    let resistance = 1.0e3;
    let model = flux_inductor("pss_inductor");
    let deck = format!(
        "* shooting integrates a module's flux\n\
         V1 in 0 DC 0 SIN(0 1 1meg)\n\
         R1 in a {resistance:e}\n\
         XVA a 0 va_pss_inductor\n\
         .va \"{model}\" va_pss_inductor\n\
         .end\n"
    );
    let netlist = Netlist::parse(&deck).expect("parse");
    let pss = Engine::new(SimulationConfig::default()).run_pss(
        &netlist,
        PssConfig::new(F0)
            .with_points_per_period(SAMPLES)
            .with_tolerance(1e-9),
    );
    let _ = std::fs::remove_file(&model);
    let pss = pss
        .unwrap_or_else(|error| panic!("shooting must run: {error}"))
        .result;
    let output = pss
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("a"))
        .expect("V(a) present");
    let end = *pss.time.last().expect("a periodic waveform");
    let shooting = period_spectrum(&pss.time, &pss.waveforms[output].values, end, SAMPLES, 3);
    let reactance = Complex64::new(0.0, std::f64::consts::TAU * F0 * FLUX_INDUCTANCE);
    let expected = Complex64::new(0.0, -1.0) * reactance / (resistance + reactance);
    let error = (0..=3)
        .map(|k| {
            let reference = if k == 1 {
                expected
            } else {
                Complex64::new(0.0, 0.0)
            };
            (shooting[k] - reference).norm() / expected.norm()
        })
        .fold(0.0, f64::max);
    eprintln!("flux module: shooting worst {error:.3e} of the closed form's fundamental");
    assert!(
        error <= 1e-3,
        "shooting {shooting:?} against the closed form's fundamental {expected}"
    );
}

/// Shooting refuses a runtime module whose operator keeps a history neither
/// native nor event shooting carries, before solving, by the operator's own
/// name and the line the module wrote it on (the analog body is the sixth
/// line of each module file), with the instance, the module and shooting's
/// own token. An `idt` output is a coordinate of event shooting's period map.
#[test]
fn shooting_refuses_each_operator_with_state_by_name() {
    const STATE: &str = "whose operator state is not a node voltage the shooting state fixes at \
                         the period start";
    for (operator, body) in [
        (
            "laplace_nd",
            "I(out) <+ 1.0e-3 * laplace_nd(V(in), '{1.0}, '{1.0, 1.0e-7});",
        ),
        ("absdelay", "I(out) <+ 1.0e-3 * absdelay(V(in), 1.0e-7);"),
    ] {
        let module = "va_pss_refused";
        let model = write_model(
            &format!("pss_refused_{operator}"),
            &format!(
                "`include \"disciplines.vams\"\n\
                 module {module}(in, out);\n\
                 inout in, out;\n\
                 electrical in, out;\n\
                 analog begin\n\
                 {body}\n\
                 end\n\
                 endmodule\n"
            ),
        );
        let deck = format!(
            "* shooting refuses {operator}\n\
             V1 in 0 DC 0 SIN(0 1 1meg)\n\
             R1 out 0 1k\n\
             XVA in out {module}\n\
             .va \"{model}\" {module}\n\
             .end\n"
        );
        let netlist = Netlist::parse(&deck).expect("parse");
        let error = Engine::new(SimulationConfig::default())
            .run_pss(&netlist, PssConfig::new(F0))
            .map(drop)
            .expect_err("shooting must refuse the operator");
        let _ = std::fs::remove_file(&model);
        let SimulationError::UnsupportedCapability(refusal) = &error else {
            panic!("{operator}: expected a capability refusal, got {error}");
        };
        let UnsupportedCapabilityError {
            capability,
            analysis_kind,
            instance,
            module: refused_module,
            detail,
            ..
        } = refusal.as_ref();
        assert_eq!(
            *capability, "analysis.pss.veriloga_operator",
            "{operator}: {error}"
        );
        assert_eq!(*analysis_kind, Some(AnalysisKind::Pss), "{error}");
        assert_eq!(instance.as_deref(), Some("XVA"), "{error}");
        assert_eq!(refused_module.as_deref(), Some(module), "{error}");
        let file = model.rsplit('/').next().expect("file name");
        assert!(
            detail.contains("PSS analysis")
                && detail.contains(&format!("`{operator}` at {file}:6"))
                && detail.ends_with(STATE),
            "{operator}: the refusal must name the analysis, the operator, its line and why: \
             {detail}"
        );
    }
}

/// A resistor with thermal noise and, as a native model with `AF=2`, flicker
/// noise in its current squared, with the circuit's CODATA 2018 Boltzmann
/// constant.
const NOISY_RESISTOR: (&str, &str) = (
    "va_noisy_resistor",
    "parameter real r = 10.0e3;\n    parameter real kf = 1.0e-10;\n    analog begin\n        \
     I(p, n) <+ V(p, n) / r;\n        \
     I(p, n) <+ white_noise(4.0 * 1.380649e-23 * $temperature / r, \"thermal\");\n        \
     I(p, n) <+ flicker_noise(kf * pow(V(p, n) / r, 2.0), 1.0, \"flicker\");\n    end",
);

/// The ideal diode with shot noise `2q|I|`, with the circuit's CODATA 2018
/// electron charge.
const NOISY_DIODE: (&str, &str) = (
    "va_noisy_diode",
    "parameter real is = 1.0e-14;\n    real id;\n    analog begin\n        \
     id = is * (exp(V(p, n) / $vt) - 1.0);\n        I(p, n) <+ id;\n        \
     I(p, n) <+ white_noise(2.0 * 1.602176634e-19 * abs(id), \"shot\");\n    end",
);

/// A diode pumped by `VLO` through `R1`, with `resistor` and `diode` at node
/// `a`, which never falls below 0.15 V.
fn noisy_mixer(resistor: &str, diode: &str, cards: &str) -> String {
    format!(
        "* noisy pumped mixer\n\
         VLO lo 0 DC 0.6 SIN(0.6 0.4 1meg)\n\
         R1 lo a 1k\n\
         {resistor}\n\
         C1 a 0 100p\n\
         {diode}\n\
         .model dmod D IS=1e-14 N=1\n\
         .model rm R(KF=1e-10 AF=2 EF=1)\n\
         {cards}\n\
         .end\n"
    )
}

/// One output sideband's periodic noise at each offset: the total, and the
/// part the sources of one instance contribute.
struct SidebandNoise {
    total: Vec<f64>,
    device: Vec<f64>,
}

/// Periodic noise of `V(a)` at output sidebands -2..=2 and two offsets, on
/// the deck's harmonic-balance carrier or its shooting (PSS) carrier, with
/// the contributions of the sources whose labels name `device`.
fn pnoise_by_sideband(deck: &str, device: &str, shooting: bool) -> Vec<SidebandNoise> {
    use rspice_core::NoAbort;
    use rspice_core::engine::{PeriodicNoiseRequest, PeriodicNoiseSidebands};
    let netlist = Netlist::parse(deck).expect("parse");
    let engine = Engine::new(SimulationConfig::default());
    let hb = (!shooting).then(|| {
        engine
            .run_hb(&netlist, HbConfig::new(F0).with_harmonics(8))
            .unwrap_or_else(|error| panic!("HB must run: {error}\n{deck}"))
    });
    let pss = shooting.then(|| {
        engine
            .run_pss_operating_point_with_abort(
                &netlist,
                PssConfig::new(F0)
                    .with_harmonics(8)
                    .with_points_per_period(256)
                    .with_tolerance(1e-9),
                &NoAbort,
            )
            .unwrap_or_else(|error| panic!("shooting must run: {error}\n{deck}"))
    });
    let offsets = [0.13 * F0, 0.37 * F0];
    let names = |label: &str| {
        let label = label.to_ascii_lowercase();
        label
            .strip_prefix(device)
            .is_some_and(|rest| rest.is_empty() || rest.starts_with([' ', ':']))
    };
    (-2..=2)
        .map(|output| {
            let request = PeriodicNoiseRequest {
                sampling: None,
                offsets: &offsets,
                output_node: "a",
                output_ref: None,
                input_source: None,
                max_sideband: 4,
                sidebands: PeriodicNoiseSidebands { input: 0, output },
            };
            let result = match (&hb, &pss) {
                (Some(hb), _) => engine.run_pnoise_from_hb_request_with_abort(
                    &netlist,
                    &request,
                    &hb.operating_point,
                    &NoAbort,
                ),
                (_, Some(point)) => engine
                    .run_pnoise_from_pss_request_with_abort(&netlist, &request, point, &NoAbort),
                (None, None) => unreachable!("one carrier is solved"),
            }
            .unwrap_or_else(|error| panic!("pnoise must run: {error}\n{deck}"));
            let mut device = vec![0.0; offsets.len()];
            for (_, values) in result.contributors.iter().filter(|(label, _)| names(label)) {
                for (sum, value) in device.iter_mut().zip(values) {
                    *sum += value;
                }
            }
            SidebandNoise {
                total: result.output_noise,
                device,
            }
        })
        .collect()
}

/// Periodic noise folds a pumped Verilog-A module's `white_noise` and
/// `flicker_noise` processes as it folds its native twin's noise: a
/// resistor's thermal noise, and its flicker noise modulated by the pumped
/// current through it; and a diode's shot noise, which switches with its
/// current. Reference: the native resistor (`AF=2` flicker) and diode on the
/// same carrier, harmonic balance's and shooting's, at output sidebands
/// -2..=2 and two offsets. Allowance: 1e-6 of the largest native total at
/// that sideband, as for the periodic AC twins; the modules state the
/// circuit's constants.
#[test]
fn veriloga_noise_folds_as_its_native_twins_on_a_pumped_carrier() {
    let (resistor_model, resistor_card) = two_terminal("pnoise_twin", NOISY_RESISTOR);
    let (diode_model, diode_card) = two_terminal("pnoise_twin", NOISY_DIODE);
    let cases = [
        (
            noisy_mixer("R2 a 0 rm 10k", "D1 a 0 dmod", ""),
            "r2",
            noisy_mixer("XR2 a 0 va_noisy_resistor", "D1 a 0 dmod", &resistor_card),
            "xr2",
        ),
        (
            noisy_mixer("R2 a 0 10k", "D1 a 0 dmod", ""),
            "d1",
            noisy_mixer("R2 a 0 10k", "XD1 a 0 va_noisy_diode", &diode_card),
            "xd1",
        ),
    ];
    for ((native_deck, native_device, twin_deck, twin_device), shooting) in
        cases.iter().flat_map(|case| [(case, false), (case, true)])
    {
        let native = pnoise_by_sideband(native_deck, native_device, shooting);
        let twin = pnoise_by_sideband(twin_deck, twin_device, shooting);
        for (sideband, (native, twin)) in (-2..=2).zip(native.iter().zip(&twin)) {
            let scale = native.total.iter().copied().fold(0.0, f64::max);
            for (index, (native_value, twin_value)) in native
                .total
                .iter()
                .chain(&native.device)
                .zip(twin.total.iter().chain(&twin.device))
                .enumerate()
            {
                assert!(
                    (twin_value - native_value).abs() <= 1.0e-6 * scale,
                    "{twin_device} (shooting: {shooting}) sideband {sideband} value {index}: \
                     Verilog-A {twin_value:e} against native {native_value:e}"
                );
            }
            assert!(
                native.device.iter().all(|&value| value > 1.0e-3 * scale),
                "{native_device} must contribute at sideband {sideband}: {:?} of {:?}",
                native.device,
                native.total
            );
        }
    }
    let _ = std::fs::remove_file(resistor_model);
    let _ = std::fs::remove_file(diode_model);
}

/// One noise train `n` shared by two branches, LRM 2.4 section 4.6.4.6
/// Example 1, perfectly correlated: a current `n` through a 1 kOhm branch
/// from `a` to `b` and `-0.5 n` through a 2 kOhm branch from `c` to `b`,
/// each injected at a node row of its own.
const SHARED_CURRENT_NOISE: &str = "`include \"disciplines.vams\"\n\
     module va_shared_current_noise(a, b, c);\n\
     inout a, b, c;\n\
     electrical a, b, c;\n\
     real n;\n\
     analog begin\n\
         n = white_noise(1.0e-16, \"shared\");\n\
         I(a, b) <+ V(a, b) / 1.0e3 + n;\n\
         I(c, b) <+ V(c, b) / 2.0e3 - 0.5 * n;\n\
     end\n\
     endmodule\n";

/// The same train as series noise voltages of the two branches, each
/// branch equation a row of its own.
const SHARED_VOLTAGE_NOISE: &str = "`include \"disciplines.vams\"\n\
     module va_shared_voltage_noise(a, b, c);\n\
     inout a, b, c;\n\
     electrical a, b, c;\n\
     real n;\n\
     analog begin\n\
         n = white_noise(1.0e-16, \"shared\");\n\
         V(a, b) <+ 1.0e3 * I(a, b) + n;\n\
         V(c, b) <+ 2.0e3 * I(c, b) - 0.5 * n;\n\
     end\n\
     endmodule\n";

/// Periodic noise folds one noise train that reaches several rows as one
/// correlated injection, at node rows and at branch rows: driven through
/// `R1` from a carrier, with `C1` at `a` and `R3` from `a` to `c`, the
/// module's contribution to V(a) at output sidebands -2..=2 is
/// `1e-16 |H(f)|^2` at `f = |offset + k f0|`, the linear stage's closed
/// form. Folded as two independent sources it would be 2.5 times that in
/// the current form and 1.48 times in the voltage form, with the
/// injections' relative sign reversed 4 and 1.96 times. The current form is
/// checked on harmonic balance's carrier and on
/// shooting's; the voltage form on harmonic balance's, since shooting's
/// period map stores no flux for a voltage contribution and its event
/// carrier forms no Verilog-A operator. Allowance: 1e-9 of the closed form.
#[test]
fn periodic_noise_folds_one_train_shared_by_two_branches() {
    // V(a)/n from the nodal equations at `a` and `c`, `lo` held: the
    // branches' conductances and the train's weight into each node's row.
    let transfer = |frequency: f64, weight_a: f64, weight_c: f64| {
        let (g1, g3, ga, gc) = (1.0e-3, 1.0e-3, 1.0e-3, 0.5e-3);
        let y = Complex64::new(0.0, std::f64::consts::TAU * frequency * 100.0e-12);
        let numerator = weight_a + weight_c * g3 / (g3 + gc);
        numerator / (Complex64::new(g1 + ga + g3 - g3 * g3 / (g3 + gc), 0.0) + y)
    };
    let cases = [
        (
            "va_shared_current_noise",
            SHARED_CURRENT_NOISE,
            (-1.0, 0.5),
            &[false, true][..],
        ),
        (
            "va_shared_voltage_noise",
            SHARED_VOLTAGE_NOISE,
            (1.0e-3, -0.25e-3),
            &[false][..],
        ),
    ];
    for (module, source, (weight_a, weight_c), carriers) in cases {
        let path = write_model(module, source);
        let deck = format!(
            "* noise train shared by two branches\n\
             VLO lo 0 DC 0 SIN(0 1 1meg)\n\
             R1 lo a 1k\n\
             C1 a 0 100p\n\
             R3 a c 1k\n\
             XN a 0 c {module}\n\
             .va \"{path}\" {module}\n\
             .end\n"
        );
        for &shooting in carriers {
            let sidebands = pnoise_by_sideband(&deck, "xn", shooting);
            for (sideband, noise) in (-2..=2).zip(&sidebands) {
                for (&offset, &value) in [0.13 * F0, 0.37 * F0].iter().zip(&noise.device) {
                    let frequency = (offset + f64::from(sideband) * F0).abs();
                    let expected = 1.0e-16 * transfer(frequency, weight_a, weight_c).norm_sqr();
                    assert!(
                        (value - expected).abs() <= 1.0e-9 * expected,
                        "{module}, shooting: {shooting}, sideband {sideband}, offset {offset} \
                         Hz: the shared train contributes {value:e} against the closed form's \
                         {expected:e}"
                    );
                }
            }
        }
        let _ = std::fs::remove_file(path);
    }
}

/// Periodic noise refuses, by instance and module, the Verilog-A noise it
/// cannot fold: a noise table, which has no cyclostationary form, refused as
/// the folding evaluates it; and noise in a module with an operator harmonic
/// balance carries as an unknown of its own, whose noise evaluation does not
/// see that unknown, refused before any evaluation.
#[test]
fn periodic_noise_refuses_verilog_a_noise_it_cannot_fold_by_name() {
    use rspice_core::NoAbort;
    use rspice_core::engine::{PeriodicNoiseRequest, PeriodicNoiseSidebands};
    for (module, body, reason) in [
        (
            "va_table_noise",
            "analog I(p, n) <+ V(p, n) / 10.0e3 \
             + noise_table('{1.0, 1.0e-18, 1.0e7, 3.0e-18}, \"tbl\");",
            "a noise table has no cyclostationary form",
        ),
        (
            "va_delayed_noise",
            "analog I(p, n) <+ V(p, n) / 10.0e3 + 1.0e-5 * absdelay(V(p, n), 1.0e-8) \
             + white_noise(1.0e-24, \"white\");",
            "noise in a module with an operator harmonic balance carries as an unknown",
        ),
    ] {
        let (model, card) = two_terminal("pnoise_refusal", (module, body));
        let deck = noisy_mixer(&format!("XN a 0 {module}"), "D1 a 0 dmod", &card);
        let netlist = Netlist::parse(&deck).expect("parse");
        let engine = Engine::new(SimulationConfig::default());
        let hb = engine
            .run_hb(&netlist, HbConfig::new(F0).with_harmonics(8))
            .unwrap_or_else(|error| panic!("HB must run: {error}\n{deck}"));
        let error = engine
            .run_pnoise_from_hb_request_with_abort(
                &netlist,
                &PeriodicNoiseRequest {
                    sampling: None,
                    offsets: &[0.13 * F0],
                    output_node: "a",
                    output_ref: None,
                    input_source: None,
                    max_sideband: 2,
                    sidebands: PeriodicNoiseSidebands::default(),
                },
                &hb.operating_point,
                &NoAbort,
            )
            .map(drop)
            .expect_err("pnoise must refuse the noise");
        let _ = std::fs::remove_file(model);
        let SimulationError::UnsupportedCapability(refusal) = &error else {
            panic!("{module}: expected a capability refusal, got {error}");
        };
        assert_eq!(
            refusal.capability, "analysis.pnoise.colored_noise",
            "{error}"
        );
        assert!(
            refusal.detail.contains("'XN'")
                && refusal.detail.contains(&format!("'{module}'"))
                && refusal.detail.contains(reason),
            "{module}: the refusal must name the instance, the module and why: {}",
            refusal.detail
        );
    }
}

/// A module that reads a zero series resistance as an open branch, as
/// `vbic_4T_et_cf.va` does: at `r = 0` its internal nodes `x` and `y` are
/// joined by a junction to each other and to nothing else, and at `rth = 0`
/// its terminal `t` carries no equation.
const OPEN_ISLAND: &str = "`include \"disciplines.vams\"\n\
     module va_hb_open_island(p, n, t);\n\
     inout p, n, t;\n\
     electrical p, n, t, x, y;\n\
     parameter real r = 0.0;\n\
     parameter real rth = 0.0;\n\
     parameter real iloop = 0.0;\n\
     analog begin\n\
         if (r > 0.0) I(p, x) <+ V(p, x) / r; else I(p, x) <+ 0.0;\n\
         I(x, y) <+ 1.0e-14 * (limexp(V(x, y) / $vt) - 1.0);\n\
         I(y, x) <+ iloop;\n\
         if (r > 0.0) I(y, n) <+ V(y, n) / r; else I(y, n) <+ 0.0;\n\
         if (rth > 0.0) I(t) <+ V(t) / rth; else I(t) <+ 0.0;\n\
         I(p, n) <+ V(p, n) / 1.0e3;\n\
     end\n\
     endmodule\n";

fn open_island_deck(test: &str, parameters: &str) -> String {
    let path = write_model(test, OPEN_ISLAND);
    format!(
        "* open series branches\n\
         V1 in 0 DC 0.5 SIN(0.5 0.2 1meg)\n\
         R1 in p 1k\n\
         XVA p 0 t va_hb_open_island {parameters}\n\
         .va \"{path}\" va_hb_open_island\n\
         .end\n"
    )
}

/// Spectre installs GMIN on a node with no DC path at setup ("Gmin installed
/// to provide path"), so every analysis holds a compact model's floating
/// sub-network and its unconnected terminal net the same way: the operating
/// point, a transient from it, a transient from UIC and harmonic balance all
/// give `p` the 1k-1k divider of the drive, and `t` reads 0 V. Before, the
/// operating point and the transient failed on the singular system while
/// harmonic balance answered.
#[test]
fn every_analysis_holds_a_floating_compact_model_sub_network_through_gmin() {
    let deck = open_island_deck("island", "");
    let netlist = Netlist::parse(&deck).expect("parse");
    let engine = Engine::new(SimulationConfig::default());
    let op = engine
        .run_dc_op(&netlist)
        .unwrap_or_else(|error| panic!("the operating point holds the sub-network: {error}"));
    for (name, expected) in [("p", 0.25), ("t", 0.0)] {
        let value = op.try_voltage_named(name).expect("node voltage");
        assert!((value - expected).abs() <= 1e-9, "V({name}) = {value}");
    }
    let period = F0.recip();
    for startup in [
        TransientStartupMode::OperatingPoint,
        TransientStartupMode::Uic,
    ] {
        let tran = engine
            .run_tran_with_startup_mode_and_abort(
                &netlist,
                2.0 * period,
                period / 256.0,
                startup,
                &rspice_core::NoAbort,
            )
            .unwrap_or_else(|error| panic!("the {startup:?} transient runs: {error}"));
        let time = &tran.time;
        for (name, expected) in [
            (
                "p",
                Box::new(|t: f64| 0.25 + 0.1 * (std::f64::consts::TAU * F0 * t).sin())
                    as Box<dyn Fn(f64) -> f64>,
            ),
            ("t", Box::new(|_| 0.0)),
        ] {
            let waveform = tran.try_voltage_waveform_named(name).expect("waveform");
            // UIC's first point is its authored state, not the drive's.
            for (&t, &value) in time.iter().zip(waveform).skip(1) {
                assert!(
                    (value - expected(t)).abs() <= 1e-6,
                    "{startup:?} V({name}) at {t:e}: {value}"
                );
            }
        }
    }
    let hb = run_hb(&deck, 4);
    let p = node(&hb, "p");
    assert!(
        (p[0].re - 0.25).abs() <= 1e-6 && (p[1].norm() - 0.1).abs() <= 1e-6,
        "{p:?}"
    );
    assert!(node(&hb, "t")[0].norm() <= 1e-9);
}

/// A held sub-network whose own loop current sets a voltage across it has
/// no canonical level: only GMIN would place it. The operating point
/// refuses it by name, and harmonic balance gives the same refusal rather
/// than an answer from a seed of zero.
#[test]
fn a_held_sub_network_only_gmin_would_place_is_refused_by_every_analysis() {
    let deck = open_island_deck("island_loop", "iloop=1u");
    let netlist = Netlist::parse(&deck).expect("parse");
    let engine = Engine::new(SimulationConfig::default());
    let op = engine
        .run_dc_op(&netlist)
        .expect_err("only GMIN would place the sub-network")
        .to_string();
    assert!(
        op.contains(" has no DC path: it belongs to a sub-network"),
        "{op}"
    );
    let hb = engine
        .run_hb(&netlist, HbConfig::new(F0).with_harmonics(4))
        .expect_err("harmonic balance refuses as the operating point does")
        .to_string();
    assert_eq!(hb, op);
}

/// The PSTB spectrum of `deck`'s orbit read at inductor `probe`, the
/// multipliers sorted by magnitude, largest first, and the orbit.
fn pstb_spectrum(
    deck: &str,
    config: PssConfig,
    probe: &str,
) -> (Vec<Complex64>, rspice_core::engine::PssOperatingPoint) {
    let netlist = Netlist::parse(deck).expect("parse");
    let engine = Engine::new(SimulationConfig::default());
    let point = engine
        .run_pss_operating_point_with_abort(&netlist, config, &rspice_core::NoAbort)
        .unwrap_or_else(|error| panic!("PSS must solve: {error}\n{deck}"));
    let card = rspice_core::netlist::PstbCard {
        probe_instance: probe.to_owned(),
        max_harmonics: 4,
        stability_threshold: 1.0 + 1e-6,
        detect_subharmonics: true,
    };
    let stability = engine
        .run_pstb_card_from_pss_with_abort(&netlist, &card, &point, &rspice_core::NoAbort)
        .unwrap_or_else(|error| panic!("PSTB must run: {error}\n{deck}"));
    let mut multipliers: Vec<Complex64> = stability
        .result
        .multipliers
        .iter()
        .map(|mode| mode.value)
        .collect();
    multipliers.sort_by(|a, b| b.norm().total_cmp(&a.norm()));
    (multipliers, point)
}

/// The probe loop every PSTB contract deck carries beside its subject: V2
/// drives L2 through R2, an RL whose period maps a current deviation by
/// exp(−T·R2/L2) = exp(−1).
const PROBE_LOOP: &str = "V2 p 0 SIN(0 1 1meg)\nR2 p q 100\nL2 q 0 100u\n";

fn assert_multipliers(name: &str, actual: &[Complex64], expected: &[f64], tolerance: f64) {
    eprintln!("{name}: multipliers {actual:?} against {expected:?}");
    assert_eq!(actual.len(), expected.len(), "{name}: {actual:?}");
    for (got, want) in actual.iter().zip(expected) {
        assert!(
            (got.re / want - 1.0).abs() <= tolerance && got.im.abs() <= tolerance * want,
            "{name}: {got} against {want}"
        );
    }
}

/// PSTB takes a runtime Verilog-A module's integration state where its PSS
/// route carries it: native shooting integrates a `ddt` charge from the node
/// voltages the period starts at. A driven RC whose capacitor is the module
/// `I(p, n) <+ ddt(C·V(p, n))`, C = 500 pF behind R1 = 1 kΩ, maps a voltage
/// deviation over the period by exp(−T/RC) = exp(−2); the probe loop adds
/// exp(−1). The native twin's capacitor gives the same spectrum.
#[test]
fn pstb_reads_a_veriloga_capacitors_charge_from_its_period_map() {
    let model = write_model(
        "pstb_cap",
        "`include \"disciplines.vams\"\n\
         module va_pstb_cap(p, n);\n\
         inout p, n;\n\
         electrical p, n;\n\
         analog I(p, n) <+ ddt(500.0e-12 * V(p, n));\n\
         endmodule\n",
    );
    let deck = |capacitor: &str| {
        format!(
            "* pstb of a driven RC\nV1 in 0 SIN(0 1 1meg)\nR1 in a 1k\n{capacitor}\n{PROBE_LOOP}.end\n"
        )
    };
    let config = || {
        PssConfig::new(F0)
            .with_points_per_period(512)
            .with_tolerance(1e-9)
    };
    let expected = [(-1.0_f64).exp(), (-2.0_f64).exp()];
    let (twin, twin_point) = pstb_spectrum(&deck("C1 a 0 500p"), config(), "L2");
    let (module, point) = pstb_spectrum(
        &deck(&format!(".va \"{model}\" va_pstb_cap\nXC1 a 0 va_pstb_cap")),
        config(),
        "L2",
    );
    let _ = std::fs::remove_file(model);
    assert!(!point.shooting_state_basis()[0].starts_with("event:"));
    assert!(!twin_point.shooting_state_basis()[0].starts_with("event:"));
    assert_multipliers("Verilog-A ddt RC", &module, &expected, 1.0e-4);
    assert_multipliers("native RC twin", &twin, &expected, 1.0e-4);
    for (a, b) in module.iter().zip(&twin) {
        assert!((a - b).norm() <= 1.0e-9, "Verilog-A {a} against native {b}");
    }
}

/// PSTB reads a runtime module's stored flux from the period map that
/// carries its branch current as a coordinate: a driven RL whose inductor is
/// [`flux_inductor`], L = 100 µH behind R1 = 200 Ω, maps a current deviation
/// over the period by exp(−T·R/L) = exp(−2); the probe loop adds exp(−1).
/// The native twin's inductor gives the same spectrum.
#[test]
fn pstb_reads_a_veriloga_inductors_flux_from_its_period_map() {
    let model = flux_inductor("pstb_inductor");
    let deck = |inductor: &str| {
        format!(
            "* pstb of a driven RL\nV1 in 0 SIN(0 1 1meg)\nR1 in a 200\n{inductor}\n{PROBE_LOOP}.end\n"
        )
    };
    let config = || {
        PssConfig::new(F0)
            .with_points_per_period(512)
            .with_tolerance(1e-9)
    };
    let expected = [(-1.0_f64).exp(), (-2.0_f64).exp()];
    let (twin, _) = pstb_spectrum(&deck("L1 a 0 100u"), config(), "L2");
    let (module, point) = pstb_spectrum(
        &deck(&format!(
            ".va \"{model}\" va_pss_inductor\nXL1 a 0 va_pss_inductor"
        )),
        config(),
        "L2",
    );
    let _ = std::fs::remove_file(model);
    assert!(!point.shooting_state_basis()[0].starts_with("event:"));
    assert_multipliers("Verilog-A flux RL", &module, &expected, 1.0e-4);
    assert_multipliers("native RL twin", &twin, &expected, 1.0e-4);
    for (a, b) in module.iter().zip(&twin) {
        assert!((a - b).norm() <= 1.0e-9, "Verilog-A {a} against native {b}");
    }
}

/// PSTB takes an `idt` integrator's state where event shooting carries it:
/// `V(out) <+ idt(k·(V(in) − V(out)), 0)` closes a leaky loop
/// y' = k·(u − y), whose period maps a deviation by exp(−k·T) = exp(−2) at
/// k = 2/T. Native shooting does not carry `idt`, so the orbit is event
/// shooting's, its coordinates the whole solution and the integrator's
/// state; the spectrum is read over those the period remembers. The native
/// twin is the RC with RC = 1/k, on native shooting.
#[test]
fn pstb_reads_an_idt_integrators_state_on_an_event_orbit() {
    let model = write_model(
        "pstb_idt",
        "`include \"disciplines.vams\"\n\
         module va_pstb_leaky(inp, out);\n\
         inout inp, out;\n\
         electrical inp, out;\n\
         analog V(out) <+ idt(2.0e6 * (V(inp) - V(out)), 0.0);\n\
         endmodule\n",
    );
    let deck = |integrator: &str| {
        format!(
            "* pstb of a leaky integrator\nV1 in 0 SIN(0 1 1meg)\n{integrator}\n{PROBE_LOOP}.end\n"
        )
    };
    let expected = [(-1.0_f64).exp(), (-2.0_f64).exp()];
    let (module, point) = pstb_spectrum(
        &deck(&format!(
            ".va \"{model}\" va_pstb_leaky\nXI1 in out va_pstb_leaky\nRL out 0 1meg"
        )),
        PssConfig::new(F0).with_tstab(20.0e-6),
        "L2",
    );
    let _ = std::fs::remove_file(model);
    assert!(point.shooting_state_basis()[0].starts_with("event:"));
    let (twin, _) = pstb_spectrum(
        &deck("R1 in out 1k\nC1 out 0 500p"),
        PssConfig::new(F0)
            .with_points_per_period(512)
            .with_tolerance(1e-9),
        "L2",
    );
    assert_multipliers("Verilog-A idt loop", &module, &expected, 1.0e-3);
    assert_multipliers("native RC twin", &twin, &expected, 1.0e-4);
}

/// PSTB of an autonomous orbit a runtime module sustains: the cubic
/// conductance `I(p, n) <+ −g1·V + g3·V³` (g1 = 2 mS, g3 = 8/3 mS/V²) across
/// a 10 µH ∥ 1 nF tank is a van der Pol oscillator. One multiplier is the
/// phase's, 1 within the solver's tolerance; the other follows from
/// Liouville's formula, λ1·λ2 = exp(∮ tr A dt), with tr A = −f'(v)/C on the
/// tank (the inductor's row has no diagonal), so
/// λ2 = exp(−(1/C)·∮(−g1 + 3·g3·v²) dt), integrated over the solved orbit.
/// The native twin's behavioral current source gives the same spectrum. A
/// 0.5 V kick at 1 µs, after the orbit's window, starts the oscillation in
/// the stabilization.
#[test]
fn pstb_reads_a_veriloga_oscillators_orbit_with_its_phase_multiplier() {
    let model = write_model(
        "pstb_vdp",
        "`include \"disciplines.vams\"\n\
         module va_pstb_cubic(p, n);\n\
         inout p, n;\n\
         electrical p, n;\n\
         analog I(p, n) <+ -2.0e-3 * V(p, n) + 2.6666666666666667e-3 * V(p, n) * V(p, n) * V(p, n);\n\
         endmodule\n",
    );
    let deck = |conductance: &str| {
        format!(
            "* van der Pol\nL2 p 0 10u\nC1 p 0 1n\nI0 0 p PULSE(0 10m 1u 1n 1n 50n 1)\n{conductance}\n.end\n"
        )
    };
    let config = || {
        PssConfig::autonomous()
            .with_period_guess(std::f64::consts::TAU * (10.0e-6_f64 * 1.0e-9).sqrt())
            .with_oscillator_node("p")
            .with_tstab_periods(40)
            .with_points_per_period(512)
            .with_tolerance(1e-9)
    };
    let (module, point) = pstb_spectrum(
        &deck(&format!(
            ".va \"{model}\" va_pstb_cubic\nXG1 p 0 va_pstb_cubic"
        )),
        config(),
        "L2",
    );
    let _ = std::fs::remove_file(model);
    let (twin, _) = pstb_spectrum(
        &deck("B1 p 0 I=-2m*V(p)+2.6666666666666667m*V(p)*V(p)*V(p)"),
        config(),
        "L2",
    );
    let orbit = &point.analysis().result;
    let p = orbit
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("p"))
        .expect("p is retained");
    let v = &orbit.waveforms[p].values;
    let divergence: f64 = orbit
        .time
        .windows(2)
        .zip(v.windows(2))
        .map(|(t, v)| {
            let slope = |v: f64| -2.0e-3 + 3.0 * 2.6666666666666667e-3 * v * v;
            0.5 * (t[1] - t[0]) * (slope(v[0]) + slope(v[1]))
        })
        .sum();
    let expected = [1.0, (-divergence / 1.0e-9).exp()];
    assert_multipliers("Verilog-A van der Pol", &module, &expected, 2.0e-3);
    assert_multipliers("native van der Pol twin", &twin, &expected, 2.0e-3);
    for (a, b) in module.iter().zip(&twin) {
        assert!((a - b).norm() <= 1.0e-9, "Verilog-A {a} against native {b}");
    }
}

/// Periodic AC, periodic transfer functions and periodic noise read an
/// `idt` carried as unknowns of its own at every sideband: a
/// non-quasi-static charge relaxing to `cV` in `τ`, `q = idt((cV − q)/τ, cV)`,
/// is `cV/(1 + jωτ)`, so behind `R` the stage passes
/// `V(a)/V(rf) = 1/(1 + jωRc/(1 + jωτ))` at sideband 0, nothing at the
/// others, and `R`'s thermal noise reaches `V(a)` as `4kTR|H|²`. Beside it a
/// pumped branch the stimulus never reaches, and a pair of the module's
/// nodes whose equations read only each other while its current and the
/// charge's rate read them (PSP's noise nodes), solve to exactly zero.
/// Forty-one sidebands, on a carrier of the forty harmonics they span, put
/// the sideband systems on the Krylov route, where GMRES left rounding
/// remnants on that pair and periodic AC failed its componentwise
/// certificate at 1.0. Allowance: 1e-9 of the transfer, 1e-6 of the noise.
#[test]
fn a_carried_non_quasi_static_charge_gives_its_closed_form_in_periodic_small_signal() {
    use rspice_core::NoAbort;
    use rspice_core::analysis::pac::{PacConfig, PacSweepType};
    use rspice_core::constants::K_BOLTZMANN;
    use rspice_core::engine::{PeriodicNoiseRequest, PeriodicNoiseSidebands};
    use rspice_core::netlist::{FreqVariation, PeriodicSourceSelector, PeriodicSweep, PxfCard};
    const MAX_SIDEBAND: i32 = 20;
    let (resistance, capacitance, tau) = (1.0e3, 1.0e-9, 2.0e-7);
    let (_, card) = two_terminal(
        "pac_idt",
        (
            "va_nqs_charge",
            "electrical q, noi, noi2;\n\
             analog begin\n\
             V(q) <+ idt((1.0e-9 * V(p, n) - V(q)) / 2.0e-7 + 1.0e6 * V(noi2), 1.0e-9 * V(p, n));\n\
             I(p, n) <+ ddt(V(q));\n\
             I(noi2) <+ V(noi2);\n\
             I(noi) <+ 1.0e-3 * V(noi) - 1.0e3 * V(noi2);\n\
             I(p, n) <+ 1.0e-3 * V(noi2) + ddt(1.0e-12 * V(noi));\n\
             end",
        ),
    );
    let deck = format!(
        "* carried non-quasi-static charge\nVRF rf 0 DC 0\nR1 rf a 1k\nXQ a 0 va_nqs_charge\n\
         VP p 0 SIN(0 1 1meg)\nRP p 0 1k\n{card}\n.end\n"
    );
    let netlist = Netlist::parse(&deck).expect("parse");
    let engine = Engine::new(SimulationConfig::default());
    let hb = engine
        .run_hb(
            &netlist,
            HbConfig::new(F0).with_harmonics(2 * MAX_SIDEBAND as usize),
        )
        .unwrap_or_else(|error| panic!("HB must run: {error}\n{deck}"));
    let offsets = [0.13 * F0, 0.37 * F0];
    let transfer = |offset: f64| {
        let omega = std::f64::consts::TAU * offset;
        let charge = Complex64::new(capacitance, 0.0) / Complex64::new(1.0, omega * tau);
        Complex64::new(1.0, 0.0) / (1.0 + Complex64::new(0.0, omega * resistance) * charge)
    };
    let pac = engine
        .run_pac_from_hb_with_abort(
            &netlist,
            PacConfig::new()
                .with_fundamental(F0)
                .with_sweep(offsets[0], offsets[1], 2)
                .with_sweep_type(PacSweepType::Linear)
                .with_sidebands(-MAX_SIDEBAND, MAX_SIDEBAND)
                .with_input_source("vrf")
                .with_output_node("a"),
            &hb.operating_point,
            &NoAbort,
        )
        .unwrap_or_else(|error| panic!("PAC must run: {error}\n{deck}"));
    let pxf = engine
        .run_pxf_card_from_hb_with_abort(
            &netlist,
            &PxfCard {
                sweep: PeriodicSweep {
                    variation: FreqVariation::Lin,
                    points: 2,
                    start_freq: offsets[0],
                    stop_freq: offsets[1],
                },
                input_source: "vrf".into(),
                input_sideband: 0,
                output_node: "a".into(),
                output_ref: None,
                output_sideband: 0,
                max_sideband: MAX_SIDEBAND,
                reltol: 1e-9,
                abstol: 1e-15,
                source: PeriodicSourceSelector::Preceding,
            },
            &hb.operating_point,
            &NoAbort,
        )
        .unwrap_or_else(|error| panic!("PXF must run: {error}\n{deck}"));
    let noise = engine
        .run_pnoise_from_hb_request_with_abort(
            &netlist,
            &PeriodicNoiseRequest {
                sampling: None,
                offsets: &offsets,
                output_node: "a",
                output_ref: None,
                input_source: None,
                max_sideband: MAX_SIDEBAND,
                sidebands: PeriodicNoiseSidebands {
                    input: 0,
                    output: 0,
                },
            },
            &hb.operating_point,
            &NoAbort,
        )
        .unwrap_or_else(|error| panic!("pnoise must run: {error}\n{deck}"));
    for (index, &offset) in offsets.iter().enumerate() {
        let expected = transfer(offset);
        for sideband in -MAX_SIDEBAND..=MAX_SIDEBAND {
            let actual = pac
                .result
                .conversion_matrix
                .get(index, sideband, 0)
                .expect("sideband present");
            let reference = if sideband == 0 {
                expected
            } else {
                Complex64::new(0.0, 0.0)
            };
            assert!(
                (actual - reference).norm() <= 1e-9 * expected.norm(),
                "PAC at {offset} Hz, sideband {sideband}: {actual} against {reference}"
            );
        }
        let point = pxf.points[index].transfer;
        assert!(
            (point - expected).norm() <= 1e-9 * expected.norm(),
            "PXF at {offset} Hz: {point} against {expected}"
        );
        let thermal = 4.0 * K_BOLTZMANN * 300.15 * resistance * expected.norm_sqr();
        assert!(
            (noise.output_noise[index] - thermal).abs() <= 1e-6 * thermal,
            "pnoise at {offset} Hz: {} against {thermal}",
            noise.output_noise[index]
        );
    }
}
