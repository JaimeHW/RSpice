//! Opt-in executable compilation of both halves of one mixed module.

use rspice_veriloga::{CompilerOptions, VerilogACompiler};

const MIXED: &str = r#"
module mixed(p, n, clk, q);
  inout p, n; electrical p, n;
  input clk; output q; wire clk; reg q;
  initial q = 1'b0;
  always @(posedge clk) q <= ~q;
  analog I(p, n) <+ V(p, n) / 1000.0;
endmodule
"#;

#[test]
fn connection_metadata_retains_physical_definitions_without_connect_rules() {
    let compiler = VerilogACompiler::default();
    let source = "nature ProbeQuantity; units=\"V\"; access=Probe; abstol=1e-7; endnature
        discipline sensing; potential ProbeQuantity; enddiscipline
        module sensor(p); inout p; sensing p; analog Probe(p)<+1; endmodule";
    let specification = compiler
        .connect_specification_from_preprocessed(source)
        .unwrap();
    assert!(specification.declares_module);
    assert_eq!(
        specification.disciplines.natures["ProbeQuantity"].abstol,
        1e-7
    );
    assert_eq!(
        specification.disciplines.disciplines["sensing"]
            .potential
            .as_deref(),
        Some("ProbeQuantity")
    );
    let invalid = source.replace("abstol=1e-7", "abstol=unknown");
    assert!(
        compiler
            .connect_specification_from_preprocessed(&invalid)
            .is_err()
    );
}

#[test]
fn connection_closure_is_retained_validated_and_reused_for_specialization() {
    use rspice_veriloga::canonical_ir::{CanonicalConnectionContext, CanonicalIrArtifact};

    let compiler = VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..Default::default()
    });
    let source = format!(
        "nature ProbeQuantity; units=\"V\"; access=Probe; abstol=1e-7; endnature\ndiscipline sensing; potential ProbeQuantity; enddiscipline\nmodule sized(q); parameter integer WIDTH=2; output [WIDTH-1:0] q; reg [WIDTH-1:0] q; initial q=1; endmodule\n{}",
        rspice_veriloga::connect::library::builtin_connect_library_source()
    );
    let runtime = compiler.compile_runtime(&source, None).unwrap();
    assert!(
        runtime.canonical_ir.parameter_source.is_none(),
        "the closure is stored once"
    );
    let encoded = serde_json::to_value(&runtime.canonical_ir).unwrap();
    let artifact: CanonicalIrArtifact = serde_json::from_value(encoded.clone()).unwrap();
    artifact.validate().unwrap();
    assert!(
        artifact
            .connections
            .source()
            .unwrap()
            .contains("connectrules")
    );
    let specification = compiler
        .connect_specification_from_preprocessed(artifact.connections.source().unwrap())
        .unwrap();
    assert_eq!(
        specification.disciplines.natures["ProbeQuantity"].abstol,
        1e-7
    );
    assert_eq!(
        specification.disciplines.disciplines["sensing"]
            .potential
            .as_deref(),
        Some("ProbeQuantity")
    );
    let specialized = compiler
        .specialize_mixed_runtime(
            &artifact,
            &[("WIDTH", 4.0)],
            &rspice_veriloga::NoPipelineControl,
        )
        .unwrap();
    assert_eq!(specialized.canonical_ir.digital.signals[0].width, 4);
    assert_eq!(specialized.canonical_ir.connections, artifact.connections);

    let mut missing = encoded;
    missing.as_object_mut().unwrap().remove("connections");
    assert!(serde_json::from_value::<CanonicalIrArtifact>(missing).is_err());
    let mut lost = artifact.clone();
    lost.connections = CanonicalConnectionContext::None;
    assert!(
        lost.validate()
            .unwrap_err()
            .iter()
            .any(|error| { error.to_string().contains("stored connection identity") })
    );
    let mut altered = artifact;
    altered.connections = CanonicalConnectionContext::Source(
        altered
            .connections
            .source()
            .unwrap()
            .replace("WIDTH=2", "WIDTH=3")
            .into(),
    );
    assert!(
        altered
            .validate()
            .unwrap_err()
            .iter()
            .any(|error| { error.to_string().contains("connection elaboration source") })
    );
}

#[test]
fn mixed_parameter_specialization_survives_serialization_and_changes_port_shape() {
    let compiler = VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..Default::default()
    });
    let source = "module sized(q); parameter integer WIDTH=2; output [WIDTH-1:0] q; reg [WIDTH-1:0] q; initial q=1; endmodule";
    let runtime = compiler.compile_runtime(source, None).unwrap();
    let bytes = serde_json::to_vec(&runtime.canonical_ir).unwrap();
    let artifact = serde_json::from_slice(&bytes).unwrap();
    let specialized = compiler
        .specialize_mixed_runtime(
            &artifact,
            &[("WIDTH", 4.0)],
            &rspice_veriloga::NoPipelineControl,
        )
        .unwrap();
    specialized.validate_integrity().unwrap();
    assert_eq!(specialized.canonical_ir.digital.signals[0].width, 4);
    assert_eq!(runtime.canonical_ir.digital.signals[0].width, 2);
    assert_ne!(
        runtime.canonical_ir.digital_identity,
        specialized.canonical_ir.digital_identity
    );
    for parameters in [
        vec![("missing", 1.0)],
        vec![("WIDTH", f64::NAN)],
        vec![("WIDTH", 2.0), ("WIDTH", 3.0)],
    ] {
        assert!(
            compiler
                .specialize_mixed_runtime(
                    &artifact,
                    &parameters,
                    &rspice_veriloga::NoPipelineControl
                )
                .is_err()
        );
    }
    let mut corrupted = runtime.canonical_ir;
    corrupted.parameter_source = Some(source.replace("WIDTH=2", "WIDTH=3").into());
    assert!(corrupted.validate().is_err());
}

#[test]
fn shared_discrete_inputs_reject_dual_writers_and_unrepresentable_widths() {
    let compiler = VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..Default::default()
    });
    for (source, expected) in [
        (
            "module dual(p); inout p; electrical p; reg state; initial state=0; analog begin state=1; I(p)<+state; end endmodule",
            "cannot be written by the analog body",
        ),
        // VAMS-2023 Table 7-1 separates an `integer` variable from a packed
        // bit grouping: the grouping is zero-extended into the analog domain
        // and carries at most 31 bits, whatever its signedness in a discrete
        // expression. A 64-bit `reg` names no analog value, so the refusal is
        // the width and it says which spelling does carry a signed 32-bit one.
        (
            "module wide(p); inout p; electrical p; reg [63:0] state; initial state=0; analog I(p)<+state; endmodule",
            "exceeds the 31-bit grouping limit",
        ),
    ] {
        let error = compiler
            .compile_runtime(source, None)
            .unwrap_err()
            .to_string();
        assert!(error.contains(expected), "{error}");
    }
}

#[test]
fn mixed_runtime_compilation_is_explicit_and_retains_both_domains() {
    let refused = VerilogACompiler::new(CompilerOptions::default())
        .compile_runtime(MIXED, None)
        .expect_err("ordinary analog runtime compilation must remain fail-closed");
    assert_eq!(
        refused.diagnostic_code(),
        "VA-CODEGEN-UNSUPPORTED-AMS-DIGITAL"
    );

    let runtime = VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..CompilerOptions::default()
    })
    .compile_runtime(MIXED, None)
    .expect("an opted-in mixed host owns digital execution");
    assert_eq!(runtime.model.stamp_programs.len(), 1);
    assert_eq!(runtime.canonical_ir.mir.equations.len(), 1);
    assert_eq!(runtime.canonical_ir.digital.processes.len(), 2);
}

#[test]
fn malformed_or_unsupported_mixed_content_still_fails_closed() {
    let compiler = VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..CompilerOptions::default()
    });
    assert!(
        compiler
            .compile_runtime("module broken(p); always @( endmodule", None)
            .is_err()
    );

    let shared_crossing = r#"
module unsupported(p, n, q);
  inout p, n; electrical p, n;
  output q; reg q; real shared;
  always @(q) q = shared;
  analog begin shared = V(p, n); I(p, n) <+ shared; end
endmodule
"#;
    let runtime = compiler.compile_runtime(shared_crossing, None).unwrap();
    assert_eq!(
        runtime
            .canonical_ir
            .hir
            .digital_observations
            .iter()
            .map(|name| name.as_str())
            .collect::<Vec<_>>(),
        vec!["shared"]
    );
}

#[cfg(feature = "native")]
#[test]
fn analog_variable_reads_are_published_by_the_normal_native_evaluation() {
    use rspice_veriloga::device::VerilogADevice;
    let compiler = VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..Default::default()
    });
    let runtime = compiler
        .compile_runtime(
            r#"
module observed(p,q); inout p; electrical p; output q; reg q;
real measured; integer count;
analog begin measured=2*V(p); count=-3; I(p)<+V(p)/1000; end
initial begin #1; q=(measured>1.0)&&(count<0); end
endmodule
"#,
            None,
        )
        .unwrap();
    assert_eq!(
        runtime
            .canonical_ir
            .hir
            .digital_observations
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>(),
        vec!["count", "measured"]
    );
    let mut device = VerilogADevice::try_new_with_canonical_ir(
        "x",
        runtime.model,
        &runtime.canonical_ir,
        &[1, 2],
    )
    .unwrap();
    for (input, expected) in [(1.5, 3.0), (2.25, 4.5)] {
        device
            .try_stamp(&[input, 0.0], |_, _, _| {}, |_, _| {})
            .unwrap();
        assert_eq!(device.variable("measured"), Some(expected));
        assert_eq!(device.variable("count"), Some(-3.0));
    }
}

#[cfg(feature = "native")]
#[test]
fn discrete_value_validity_is_checked_only_at_executed_selected_reads() {
    use rspice_veriloga::device::VerilogADevice;
    let source = r#"
module selected(p); inout p; electrical p;
 reg [7:0] data[-1:0]; integer index; reg enabled;
 initial begin index=-1; enabled=0; data[-1]=5; end
 analog begin
   if (enabled) I(p)<+(data[index]>0 ? 2 : 1);
   else I(p)<+3;
 end
endmodule
"#;
    let compiler = VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..Default::default()
    });
    let runtime = compiler.compile_runtime(source, None).unwrap();
    let hir = &runtime.canonical_ir.hir;
    assert_eq!(hir.discrete_inputs.len(), 4);
    let encoded = serde_json::to_vec(&runtime.canonical_ir).unwrap();
    let restored: rspice_veriloga::canonical_ir::CanonicalIrArtifact =
        serde_json::from_slice(&encoded).unwrap();
    restored.validate().unwrap();
    let mut malformed = hir.clone();
    malformed.discrete_inputs[0][1] = malformed.discrete_inputs[0][0];
    assert!(malformed.validate().is_err());
    let mut device =
        VerilogADevice::try_new_with_canonical_ir("x", runtime.model, &runtime.canonical_ir, &[1])
            .unwrap();
    let sample = |device: &mut VerilogADevice, name: &str, value: Option<f64>| {
        let [numeric, valid] = *hir
            .discrete_inputs
            .iter()
            .find(|pair| hir.variables[usize::from(pair[0])].name == name)
            .unwrap();
        device
            .sample_discrete_state(usize::from(numeric), value.unwrap_or(0.0))
            .unwrap();
        device
            .sample_discrete_state(usize::from(valid), f64::from(u8::from(value.is_some())))
            .unwrap();
    };
    let stamp = |device: &mut VerilogADevice| {
        let mut rhs = 0.0;
        device
            .try_stamp(&[0.0], |_, _, _| {}, |_, value| rhs += value)
            .map(|_| rhs)
    };
    sample(&mut device, "enabled", Some(0.0));
    sample(&mut device, "index", Some(-1.0));
    sample(&mut device, "data[-1]", None);
    sample(&mut device, "data[0]", None);
    assert_eq!(stamp(&mut device).unwrap(), -3.0);
    sample(&mut device, "data[-1]", Some(5.0));
    sample(&mut device, "enabled", Some(1.0));
    assert_eq!(stamp(&mut device).unwrap(), -2.0);
    sample(&mut device, "index", Some(0.0));
    let error = stamp(&mut device).unwrap_err();
    assert!(
        error.to_string().contains("analog read of discrete input"),
        "{error}"
    );
    sample(&mut device, "data[0]", Some(0.0));
    assert_eq!(stamp(&mut device).unwrap(), -1.0);
    sample(&mut device, "data[0]", None);
    assert!(stamp(&mut device).is_err());
    sample(&mut device, "enabled", Some(0.0));
    assert_eq!(stamp(&mut device).unwrap(), -3.0);
}

#[test]
fn discrete_array_selector_keeps_one_operator_site_and_portable_value() {
    use rspice_veriloga::canonical_ir::state::{CanonicalStateFamily, CanonicalStateLayout};
    use rspice_veriloga::vm::{Vm, VmContext};
    let source = r#"
module delayed_selector(p); inout p; electrical p;
 reg [7:0] data[0:1]; integer index;
 initial begin index=0; data[0]=7; end
 analog I(p)<+data[absdelay(index, 1)];
endmodule
"#;
    let compiler = VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..Default::default()
    });
    let runtime = compiler.compile_runtime(source, None).unwrap();
    let hir = &runtime.canonical_ir.hir;
    let layout = CanonicalStateLayout::from_hir(hir);
    assert_eq!(
        layout.family_len(CanonicalStateFamily::DelayBuffer),
        1,
        "one authored selector must own exactly one delay history"
    );
    let mut context = VmContext::new(runtime.model.num_terminals);
    context.variables.resize(runtime.model.num_variables, 0.0);
    context.allocate_delay_buffers(1);
    let sample = |context: &mut VmContext, name: &str, value: Option<f64>| {
        let pair = hir
            .discrete_inputs
            .iter()
            .find(|pair| hir.variables[usize::from(pair[0])].name == name)
            .unwrap();
        context.variables[usize::from(pair[0])] = value.unwrap_or(0.0);
        context.variables[usize::from(pair[1])] = f64::from(u8::from(value.is_some()));
    };
    sample(&mut context, "index", Some(0.0));
    sample(&mut context, "data[0]", Some(7.0));
    sample(&mut context, "data[1]", None);
    let program = &runtime.model.stamp_programs[0].value_program;
    assert_eq!(Vm::new(&mut context).execute(program).unwrap(), 7.0);
    sample(&mut context, "index", Some(1.0));
    assert!(
        Vm::new(&mut context)
            .execute(program)
            .unwrap_err()
            .to_string()
            .contains("analog read of discrete input")
    );
    sample(&mut context, "data[1]", Some(3.0));
    assert_eq!(Vm::new(&mut context).execute(program).unwrap(), 3.0);

    #[cfg(feature = "native")]
    {
        let mut device = rspice_veriloga::device::VerilogADevice::try_new_with_canonical_ir(
            "x",
            runtime.model.clone(),
            &runtime.canonical_ir,
            &[1],
        )
        .unwrap();
        for pair in &hir.discrete_inputs {
            for slot in pair {
                let slot = usize::from(*slot);
                device
                    .sample_discrete_state(slot, context.variables[slot])
                    .unwrap();
            }
        }
        let mut rhs = 0.0;
        device
            .try_stamp(&[0.0], |_, _, _| {}, |_, value| rhs += value)
            .unwrap();
        assert_eq!(rhs, -3.0);
    }

    let mut malformed = hir.clone();
    let access = malformed
        .expressions
        .iter_mut()
        .find_map(|expr| {
            if let rspice_veriloga::canonical_ir::hir::HirExprKind::ArrayAccess {
                discrete_validity: Some(name),
                ..
            } = &mut expr.kind
            {
                Some(name)
            } else {
                None
            }
        })
        .unwrap();
    *access = "data".into();
    assert!(
        malformed.validate().is_err(),
        "value cells cannot masquerade as validity cells"
    );
}

#[test]
fn packed_analog_selection_validates_grouping_and_keeps_one_word_selector() {
    use rspice_veriloga::canonical_ir::state::{CanonicalStateFamily, CanonicalStateLayout};
    let compiler = VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..Default::default()
    });
    let source = r#"
module selected(p); inout p; electrical p;
 reg [95:0] data[-1:0]; integer index;
 initial begin index=-1; data[-1]=96'bx; data[-1][3:0]=4'd7; end
 analog begin
   begin : first_scope
     I(p)<+data[absdelay(index,1)][3:0];
   end
   begin : second_scope
     I(p)<+(data[index][3:0] & 3);
   end
 end
endmodule
"#;
    let runtime = compiler.compile_runtime(source, None).unwrap();
    let hir = &runtime.canonical_ir.hir;
    assert_eq!(
        CanonicalStateLayout::from_hir(hir).family_len(CanonicalStateFamily::DelayBuffer),
        1
    );
    assert_eq!(hir.discrete_selections.len(), 2);
    for (declaration, read, diagnostic) in [
        ("reg [95:0] value;", "value[31:0]", "31-bit grouping limit"),
        ("reg [95:0] value;", "value", "31-bit grouping limit"),
        (
            "reg [0:95] value;",
            "value[3:0]",
            "reverses its declared range",
        ),
        ("real value;", "value[0]", "four-state"),
    ] {
        let source = format!(
            "module invalid(p); inout p; electrical p; {declaration} integer index; initial begin index=0; value=0; end analog I(p)<+{read}; endmodule"
        );
        let error = compiler
            .compile_runtime(&source, None)
            .err()
            .expect("invalid packed read");
        assert!(error.to_string().contains(diagnostic), "{read}: {error}");
    }
}

#[test]
fn dynamic_packed_analog_selectors_preserve_chunks_validity_and_history() {
    use rspice_veriloga::canonical_ir::state::{CanonicalStateFamily, CanonicalStateLayout};
    use rspice_veriloga::vm::{Vm, VmContext};
    let compiler = VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..Default::default()
    });
    let source = r#"
module dynamic_packed(p); inout p; electrical p;
 reg [95:0] data[-1:0]; reg [0:95] ascending;
 integer word; real selector;
 initial begin word=-1; selector=14; data[-1]=96'bx; ascending=96'bz; end
 analog I(p)<+data[absdelay(word,1)][absdelay(selector,1)]+ascending[selector];
endmodule
"#;
    let runtime = compiler.compile_runtime(source, None).unwrap();
    let hir = &runtime.canonical_ir.hir;
    assert_eq!(
        CanonicalStateLayout::from_hir(hir).family_len(CanonicalStateFamily::DelayBuffer),
        2
    );
    assert_eq!(
        hir.discrete_selections.len(),
        21,
        "seven chunks per 96-bit word"
    );
    let mut context = VmContext::new(runtime.model.num_terminals);
    context.variables.resize(runtime.model.num_variables, 0.0);
    context.allocate_delay_buffers(2);
    for [value, valid] in &hir.discrete_inputs {
        context.variables[usize::from(*valid)] = 1.0;
        let Some(selection) = hir.discrete_selections.iter().find(|s| s.value == *value) else {
            continue;
        };
        assert!(selection.encoded);
        let known: &[(i64, bool)] = match selection.signal.as_str() {
            "data[-1]" => &[(14, true), (15, false), (95, true)],
            "data[0]" => &[(15, true), (95, false)],
            "ascending" => &[(81, true), (80, true), (0, false)],
            name => panic!("unexpected source {name}"),
        };
        let mut encoded = 0u32;
        for &(position, value) in known {
            let bit = position - selection.lsb;
            if (0..i64::from(selection.width)).contains(&bit) {
                encoded |= 1 << (15 + bit);
                encoded |= u32::from(value) << bit;
            }
        }
        context.variables[usize::from(*value)] = f64::from(encoded);
    }
    let slot = |name: &str| {
        usize::from(
            hir.discrete_inputs
                .iter()
                .find(|pair| hir.variables[usize::from(pair[0])].name == name)
                .unwrap()[0],
        )
    };
    #[cfg(feature = "native")]
    let mut device = rspice_veriloga::device::VerilogADevice::try_new_with_canonical_ir(
        "x",
        runtime.model.clone(),
        &runtime.canonical_ir,
        &[1],
    )
    .unwrap();
    let program = &runtime.model.stamp_programs[0].value_program;
    for (word, bit, expected) in [
        (-1.0, 14.0, Some(2.0)),
        (-1.0, 15.0, Some(1.0)),
        (-1.0, 95.0, Some(1.0)),
        (0.0, 15.0, Some(2.0)),
        (0.0, 95.0, Some(0.0)),
        (-1.0, 14.5, Some(1.0)),
        (-1.0, 13.0, None),
        (0.0, 14.0, None),
        (-1.0, 96.0, None),
        (1.0, 15.0, None),
    ] {
        context.variables[slot("word")] = word;
        context.variables[slot("selector")] = bit;
        let result = Vm::new(&mut context).execute(program);
        match expected {
            Some(value) => assert_eq!(result.unwrap(), value, "{word}[{bit}]"),
            None => assert!(result.is_err(), "{word}[{bit}]"),
        }
        #[cfg(feature = "native")]
        {
            for pair in &hir.discrete_inputs {
                for slot in pair {
                    let slot = usize::from(*slot);
                    device
                        .sample_discrete_state(slot, context.variables[slot])
                        .unwrap();
                }
            }
            let mut rhs = 0.0;
            let result = device.try_stamp(&[0.0], |_, _, _| {}, |_, v| rhs += v);
            match expected {
                Some(value) => {
                    result.unwrap();
                    assert_eq!(rhs, -value, "{word}[{bit}]");
                }
                None => assert!(result.is_err(), "{word}[{bit}]"),
            }
        }
    }
    let mut malformed = hir.clone();
    malformed.discrete_selections[0].encoded = false;
    assert!(malformed.validate().is_err());
    let mut malformed = hir.clone();
    if let rspice_veriloga::canonical_ir::hir::HirExprKind::ArrayAccess {
        packed: Some(packed),
        ..
    } = &mut malformed
        .expressions
        .iter_mut()
        .find(|e| {
            matches!(
                &e.kind,
                rspice_veriloga::canonical_ir::hir::HirExprKind::ArrayAccess {
                    packed: Some(_),
                    ..
                }
            )
        })
        .unwrap()
        .kind
    {
        packed.layout.word_len = 0;
    }
    assert!(malformed.validate().is_err());
}
