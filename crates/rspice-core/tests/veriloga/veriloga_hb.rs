use num_complex::Complex64;
use rspice_core::analysis::PssConfig;
use rspice_core::analysis::harmonic_balance::HbConfig;
use rspice_core::engine::{Engine, SimulationConfig};
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
        (
            "$abstime",
            "I(out) <+ 1.0e-3 * sin(6.283185307e6 * $abstime);",
            false,
        ),
        ("ddt", "I(out) <+ 1.0e-21 * ddt(ddt(V(in)));", false),
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
/// accepted time point, which no periodic solution carries. Shooting and
/// harmonic balance both refuse the instance before solving, as SpectreRF
/// does, and name every such variable so the author knows which to assign.
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
        for (error, kind, label, token) in [
            (
                engine
                    .run_pss(&netlist, PssConfig::new(1.0e6).with_harmonics(2))
                    .map(drop)
                    .expect_err("PSS must refuse a retained variable"),
                AnalysisKind::Pss,
                "PSS analysis",
                "analysis.periodic.veriloga",
            ),
            (
                engine
                    .run_hb(&netlist, HbConfig::new(1.0e6).with_harmonics(2))
                    .map(drop)
                    .expect_err("HB must refuse a retained variable"),
                AnalysisKind::HarmonicBalance,
                "harmonic-balance analysis",
                "analysis.hb.veriloga_operator",
            ),
        ] {
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
            assert_eq!(*capability, token, "{error}");
            assert_eq!(*analysis_kind, Some(kind), "{error}");
            assert_eq!(instance.as_deref(), Some("XVA"), "{error}");
            assert_eq!(refused_module.as_deref(), Some(module), "{error}");
            assert!(
                detail.contains(label) && detail.contains(named),
                "{module}: the refusal must name {label} and the retained variables: {detail}"
            );
        }
        let _ = std::fs::remove_file(model);
    }
}
