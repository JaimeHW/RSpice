use rspice_veriloga::canonical_ir::{
    CanonicalIrArtifact, CfgTerminator, CfgValueKind, DigitalWait,
};
use rspice_veriloga::{
    CompilerOptions, VerilogACompiler, VirtualCompileLimits, VirtualSourceBundle, VirtualSourceFile,
};

fn delay_ticks(artifact: &CanonicalIrArtifact) -> Vec<i64> {
    artifact
        .digital
        .processes
        .iter()
        .flat_map(|process| {
            process.function.blocks.iter().filter_map(|block| {
                let CfgTerminator::Wait {
                    wait: DigitalWait::Delay(value),
                    ..
                } = &block.terminator
                else {
                    return None;
                };
                Some(match &process.function.value(*value).kind {
                    CfgValueKind::IntegerConstant(value) => i64::from(*value),
                    CfgValueKind::FourStateConstant(value) => {
                        i64::try_from(value.to_u64().unwrap()).unwrap()
                    }
                    other => panic!("delay was not resolved: {other:?}"),
                })
            })
        })
        .collect()
}

#[test]
fn timescale_include_scope_rounding_hierarchy_and_transport_share_one_contract() {
    let child = "`timescale 10ns/100ps\nmodule child(q); output q; reg q; initial begin q=0; #0.125 q=1; end endmodule\n";
    let root = "`include \"child.vh\"\n`ifdef NEVER_ACTIVE\n`timescale invalid\n`endif\n`timescale 1ns/1ps\nmodule top(q,c); output q; reg q; output c; wire c; child u(c); parameter real D=0.4505; initial begin q=0; #D q=1; end endmodule\n`resetall\nmodule legacy(q); output q; reg q; initial #1 q=1; endmodule\n";
    let bundle = VirtualSourceBundle::new(
        "top.va",
        [
            VirtualSourceFile::new("top.va", root),
            VirtualSourceFile::new("child.vh", child),
        ],
    )
    .unwrap();
    let compiler = VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..Default::default()
    });
    let prepared = compiler
        .prepare_virtual_runtime_source(&bundle, VirtualCompileLimits::default())
        .unwrap();
    let top = prepared.compile_runtime("top").unwrap();
    let artifact = &top.runtime.canonical_ir;
    assert_eq!(artifact.digital.timing.root.unit_exponent(), -9);
    assert_eq!(artifact.digital.timing.precision_exponent, -12);
    assert_eq!(delay_ticks(artifact), [451, 1300]);
    assert_eq!(artifact.digital.processes[1].time_scale.unit_exponent(), -8);
    let legacy = prepared.compile_runtime("legacy").unwrap();
    assert_eq!(
        legacy
            .runtime
            .canonical_ir
            .digital
            .timing
            .precision_exponent,
        -9
    );
    assert_eq!(delay_ticks(&legacy.runtime.canonical_ir), [1]);

    let decoded: CanonicalIrArtifact =
        serde_json::from_slice(&serde_json::to_vec(artifact).unwrap()).unwrap();
    decoded.digital.validate().unwrap();
    assert_eq!(decoded.digital, artifact.digital);
    let mut changed = decoded.digital.clone();
    changed.timing.precision_exponent = -13;
    assert!(
        changed.validate().is_err(),
        "timing participates in content identity"
    );
    let mut missing = serde_json::to_value(&decoded.digital).unwrap();
    missing.as_object_mut().unwrap().remove("timing");
    assert!(
        serde_json::from_value::<rspice_veriloga::canonical_ir::CanonicalDigitalPlan>(missing)
            .is_err()
    );
}

#[test]
fn timescale_delays_round_before_rescaling_and_never_clamp() {
    let compiler = VerilogACompiler::default();
    for (scale, delay, expected) in [
        ("1ns/100ps", "0.049", 0),
        ("1ns/100ps", "0.05", 1),
        ("1ns/100ps", "0.15", 2),
        ("1ns/1ps", "1.25", 1250),
        ("100s/10s", "1.25", 13),
        ("1ns/1ns", "2147483648", 2147483648),
        ("1ns/1ns", "9007199254740992+1", 9007199254740993),
    ] {
        let source = format!(
            "`timescale {scale}\nmodule timed(q); output q; reg q; initial #({delay}) q=1; endmodule"
        );
        let artifact = compiler.compile_canonical_ir(&source).unwrap();
        assert_eq!(delay_ticks(&artifact), [expected], "{scale}, {delay}");
    }
    for (scale, delay) in [
        ("2ns/1ps", "1"),
        ("1ns/1us", "1"),
        ("1ns/1ps junk", "1"),
        ("1ns/1ps", "-1"),
        ("1ns/1ps", "1e30"),
    ] {
        let source = format!(
            "`timescale {scale}\nmodule timed(q); output q; reg q; initial #({delay}) q=1; endmodule"
        );
        assert!(
            compiler.compile_canonical_ir(&source).is_err(),
            "{scale}, {delay}"
        );
    }
}
