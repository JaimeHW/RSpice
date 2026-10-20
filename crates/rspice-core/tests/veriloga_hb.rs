#![cfg(feature = "veriloga")]

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

#[test]
fn dynamic_and_noisy_veriloga_fails_closed_until_hb_metadata_is_exact() {
    let model = write_model(
        "dynamic_noise",
        r#"
`include "disciplines.vams"

module va_hb_dynamic_noise(p, n);
    inout p, n;
    electrical p, n;
    parameter real g = 1.0e-3;
    parameter real c = 1.0e-12;
    analog I(p, n) <+ g * V(p, n) + ddt(c * V(p, n))
        + white_noise(1.0e-18, "thermal");
endmodule
"#,
    );

    let deck = format!(
        "* veriloga HB exact-capability diagnostic\n\
         V1 in 0 DC 1 SIN(1 0 1meg)\n\
         R1 in 0 1k\n\
         XVA in 0 va_hb_dynamic_noise\n\
         .va \"{model}\" va_hb_dynamic_noise\n\
         .end\n"
    );

    let netlist = Netlist::parse(&deck).expect("parse");
    let result = std::panic::catch_unwind(|| {
        Engine::new(SimulationConfig::default())
            .run_hb(&netlist, HbConfig::new(1.0e6).with_harmonics(3))
    });

    let _ = std::fs::remove_file(model);

    let result = result.expect("unsupported Verilog-A HB capability must not panic");
    let err = result.expect_err("HB must not omit Verilog-A charge or noise contributions");
    let text = err.to_string();
    assert!(
        text.contains("runtime Verilog-A")
            && text.contains("exact HB charge/noise capability metadata"),
        "diagnostic should identify the unavailable exact Verilog-A HB capability, got: {text}"
    );
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
        for (error, kind, label) in [
            (
                engine
                    .run_pss(&netlist, PssConfig::new(1.0e6).with_harmonics(2))
                    .map(drop)
                    .expect_err("PSS must refuse a retained variable"),
                AnalysisKind::Pss,
                "PSS analysis",
            ),
            (
                engine
                    .run_hb(&netlist, HbConfig::new(1.0e6).with_harmonics(2))
                    .map(drop)
                    .expect_err("HB must refuse a retained variable"),
                AnalysisKind::HarmonicBalance,
                "harmonic-balance analysis",
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
            assert_eq!(*capability, "analysis.periodic.veriloga", "{error}");
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
