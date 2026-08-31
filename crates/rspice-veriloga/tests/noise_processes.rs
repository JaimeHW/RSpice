use rspice_veriloga::{CompilerOptions, VerilogACompiler};

fn compile(source: &str) -> rspice_veriloga::CompiledModel {
    VerilogACompiler::new(CompilerOptions::default())
        .compile(source)
        .expect("noise process model compiles")
}

#[test]
fn assigned_noise_reuse_is_one_process_with_two_coherent_injections() {
    let model = compile(
        r#"
module assigned_noise(p, n);
    inout p, n;
    electrical p, n;
    real process;
    analog begin
        process = white_noise(2.0, "shared");
        I(p, n) <+ process;
        I(p, n) <+ -process;
    end
endmodule
"#,
    );
    assert_eq!(model.noise_sources.len(), 1);
    assert_eq!(model.noise_sources[0].process_id, 0);
    assert_eq!(model.noise_sources[0].injections.len(), 2);
}

#[test]
fn equal_labels_do_not_merge_distinct_syntactic_processes() {
    let model = compile(
        r#"
module equal_labels(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        I(p, n) <+ white_noise(1.0, "same");
        I(p, n) <+ -white_noise(1.0, "same");
    end
endmodule
"#,
    );
    assert_eq!(model.noise_sources.len(), 2);
    assert_eq!(model.noise_sources[0].process_id, 0);
    assert_eq!(model.noise_sources[1].process_id, 1);
    assert_eq!(model.noise_sources[0].injections.len(), 1);
    assert_eq!(model.noise_sources[1].injections.len(), 1);
}

#[test]
fn laplace_filtered_noise_is_accepted_and_keeps_a_complex_gain_program() {
    let model = compile(
        r#"
module filtered_noise(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ laplace_nd(
        white_noise(3.0, "filtered"),
        '{1.0},
        '{1.0, 1.0e-3}
    );
endmodule
"#,
    );
    assert_eq!(model.noise_sources.len(), 1);
    assert_eq!(model.noise_sources[0].injections.len(), 1);
    assert!(
        !model.noise_sources[0].injections[0]
            .gain_program
            .instructions
            .is_empty()
    );
}

#[test]
fn noise_only_assignment_replay_resolves_ddx_before_codegen() {
    let model = compile(
        r#"
module noise_ddx(p, n);
    inout p, n;
    electrical p, n;
    real gain, process, shaped;
    analog begin
        gain = ddx(V(p, n) * V(p, n), V(p, n));
        process = white_noise(1.0, "shared");
        shaped = gain * process;
        I(p, n) <+ shaped;
    end
endmodule
"#,
    );
    assert_eq!(model.noise_sources.len(), 1);
    assert_eq!(model.noise_sources[0].injections.len(), 1);
    assert!(!model.noise_assignment_steps.is_empty());
}

#[test]
fn static_unroll_assigns_distinct_dense_process_ids() {
    let model = compile(
        r#"
module unrolled_noise(p, n);
    inout p, n; electrical p, n;
    integer i;
    analog begin
        for (i = 0; i < 2; i = i + 1)
            I(p, n) <+ white_noise(1.0, "unrolled");
    end
endmodule
"#,
    );
    assert_eq!(model.noise_sources.len(), 2);
    assert_eq!(model.noise_sources[0].process_id, 0);
    assert_eq!(model.noise_sources[1].process_id, 1);
}

#[test]
fn runtime_bounded_noise_loop_fails_closed() {
    let source = r#"
module runtime_loop_noise(p, n);
    inout p, n; electrical p, n;
    parameter real count = 1.0;
    integer i; real process;
    analog begin
        i = 0;
        while (i < count) begin
            process = white_noise(1.0, "runtime");
            i = i + 1;
        end
        I(p, n) <+ process;
    end
endmodule
"#;
    let error = VerilogACompiler::new(CompilerOptions::default())
        .compile(source)
        .expect_err("runtime process cardinality must be rejected");
    assert!(
        error.to_string().contains("runtime-bounded loops"),
        "{error}"
    );
}

#[test]
fn malformed_dollar_noise_arity_is_a_diagnostic_not_a_panic() {
    let source = r#"
module malformed_dollar_noise(p, n);
    inout p, n; electrical p, n;
    analog I(p, n) <+ $white_noise();
endmodule
"#;
    let error = VerilogACompiler::new(CompilerOptions::default())
        .compile(source)
        .expect_err("missing PSD operand must be rejected");
    assert!(
        error.to_string().contains("requires 1 to 2 arguments"),
        "{error}"
    );
}

#[test]
fn process_128_has_an_exact_sparse_shadow_without_aliasing_lower_processes() {
    let mut source = String::from("module many_noise(p,n); inout p,n; electrical p,n; real live");
    for index in 0..128 {
        source.push_str(&format!(", d{index}"));
    }
    source.push_str("; analog begin ");
    for index in 0..128 {
        source.push_str(&format!("d{index}=white_noise(1.0,\"dead{index}\");"));
    }
    source.push_str("live=white_noise(1.0,\"live128\"); I(p,n)<+live; end endmodule");
    let model = compile(&source);
    assert_eq!(model.noise_sources.len(), 129);
    assert_eq!(model.noise_sources[128].process_id, 128);
    assert_eq!(model.noise_sources[128].injections.len(), 1);
    assert!(
        model.noise_sources[..128]
            .iter()
            .all(|source| source.injections.is_empty())
    );
    let noise_shadows = model
        .variable_names
        .iter()
        .filter(|name| name.contains("@dN"))
        .collect::<Vec<_>>();
    assert_eq!(
        noise_shadows.len(),
        1,
        "unexpected shadows: {noise_shadows:?}"
    );
    assert!(noise_shadows[0].ends_with("@dN128"));
}

#[cfg(feature = "native")]
#[test]
fn native_backend_constructs_assignment_reuse_process_plan() {
    let source = r#"
module native_shared_noise(p, n);
    inout p, n;
    electrical p, n;
    real process;
    analog begin
        process = white_noise(1.0, "shared");
        I(p, n) <+ process;
        I(p, n) <+ laplace_nd(process, '{1.0}, '{1.0, 1.0e-3});
    end
endmodule
"#;
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let model = compiler.compile(source).expect("native model compiles");
    let canonical = compiler
        .compile_canonical_ir(source)
        .expect("canonical model compiles");
    let mut device = rspice_veriloga::device::VerilogADevice::try_new_with_canonical_ir(
        "A1",
        model,
        &canonical,
        &[1, 0],
    )
    .expect("native grouped-noise device constructs");
    device
        .try_set_analysis_type(3)
        .expect("native noise analysis configures");
    let processes = device
        .try_noise_processes_at_frequency(&[0.0], 1.0e3)
        .expect("native scalar PSD plus shared complex gains evaluate");
    assert_eq!(processes.len(), 1);
    assert_eq!(processes[0].injections.len(), 2);
}

mod runtime {
    use rspice_veriloga::device::VerilogADevice;
    use std::sync::Arc;

    fn device(source: &str) -> VerilogADevice {
        device_with_nodes(source, &[1, 0])
    }

    fn device_with_nodes(source: &str, nodes: &[usize]) -> VerilogADevice {
        let compiler =
            rspice_veriloga::VerilogACompiler::new(rspice_veriloga::CompilerOptions::default());
        let model = compiler.compile(source).expect("noise model compiles");
        let canonical = compiler
            .compile_canonical_ir(source)
            .expect("canonical noise model compiles");
        let mut device =
            VerilogADevice::try_new_with_canonical_ir("A1", Arc::new(model), &canonical, nodes)
                .expect("noise process device constructs");
        device
            .try_set_analysis_type(3)
            .expect("noise analysis configures");
        device
    }

    #[test]
    fn raw_metadata_uses_each_call_sites_reaching_definition() {
        let mut device = device(
            r#"
module reaching_noise(p, n);
    inout p, n; electrical p, n;
    real scale;
    analog begin
        scale = 2.0;
        I(p, n) <+ white_noise(scale, "first");
        scale = 5.0;
        I(p, n) <+ white_noise(scale, "second");
    end
endmodule
"#,
        );
        let processes = device
            .try_noise_processes_at_frequency(&[0.0], 1.0e3)
            .expect("site-local PSD metadata evaluates");
        assert_eq!(processes.len(), 2);
        assert!((processes[0].psd - 2.0).abs() < 1.0e-12);
        assert!((processes[1].psd - 5.0).abs() < 1.0e-12);
    }

    #[test]
    fn assigned_source_snapshots_operands_before_later_overwrite() {
        let mut device = device(
            r#"
module assigned_snapshot(p, n);
    inout p, n; electrical p, n;
    real scale, process;
    analog begin
        scale = 2.0;
        process = white_noise(scale, "snapshot");
        scale = 5.0;
        I(p, n) <+ process;
    end
endmodule
"#,
        );
        let processes = device
            .try_noise_processes_at_frequency(&[0.0], 1.0e3)
            .expect("assigned source snapshot evaluates");
        assert_eq!(processes.len(), 1);
        assert!((processes[0].psd - 2.0).abs() < 1.0e-12);
    }

    #[test]
    fn untaken_invalid_process_is_lazy_and_dead_invalid_process_is_ignored() {
        let mut device = device(
            r#"
module lazy_dead_noise(p, n);
    inout p, n; electrical p, n;
    real hidden, dead;
    analog begin
        if (0.0) hidden = white_noise(-1.0, "untaken");
        dead = white_noise(-2.0, "dead");
        I(p, n) <+ hidden + white_noise(3.0, "live");
    end
endmodule
"#,
        );
        let processes = device
            .try_noise_processes_at_frequency(&[0.0], 1.0e3)
            .expect("untaken/dead invalid PSDs are not evaluated");
        assert_eq!(processes.len(), 1);
        assert_eq!(processes[0].name, "live");
        assert!((processes[0].psd - 3.0).abs() < 1.0e-12);
    }

    #[test]
    fn live_zero_gain_still_validates_raw_psd() {
        let mut device = device(
            r#"
module live_zero_gain(p, n);
    inout p, n; electrical p, n;
    real gain, process;
    analog begin
        gain = 0.0;
        process = white_noise(-1.0, "invalid");
        I(p, n) <+ gain * process;
    end
endmodule
"#,
        );
        let error = device
            .try_noise_processes_at_frequency(&[0.0], 1.0e3)
            .expect_err("live source with zero numeric gain must validate PSD");
        assert!(error.to_string().contains("negative"), "{error}");
    }

    #[test]
    fn mixed_current_and_potential_injections_keep_rhs_phase() {
        let mut device = device(
            r#"
module mixed_rhs_noise(p, n);
    inout p, n; electrical p, n;
    real process;
    analog begin
        process = white_noise(1.0, "shared");
        I(p, n) <+ process;
        V(p, n) <+ process;
    end
endmodule
"#,
        );
        device.set_branch_current_indices(&[2]);
        let processes = device
            .try_noise_processes_at_frequency(&[0.0, 0.0, 0.0], 1.0e3)
            .expect("mixed RHS process evaluates");
        assert_eq!(processes[0].injections.len(), 2);
        let mut gains = processes[0]
            .injections
            .iter()
            .map(|injection| injection.gain.re)
            .collect::<Vec<_>>();
        gains.sort_by(f64::total_cmp);
        assert_eq!(gains, vec![-1.0, 1.0]);
    }

    #[test]
    fn canonical_metadata_matches_smallsig_simparam_and_optional_port_runtime() {
        let mut device = device_with_nodes(
            r#"
module runtime_metadata(p, n, optional);
    inout p, n, optional; electrical p, n, optional;
    analog I(p, n) <+ white_noise(
        (analysis("smallsig") ? 1.0 : -100.0)
        + $simparam("gmin")
        + $simparam("ignored", 2.0)
        + ($port_connected(optional) ? -100.0 : 0.0),
        "runtime"
    );
endmodule
"#,
            &[1, 0],
        );
        let processes = device
            .try_noise_processes_at_frequency(&[0.0], 1.0e3)
            .expect("runtime metadata leaves evaluate");
        assert!((processes[0].psd - (3.0 + 1.0e-12)).abs() < 1.0e-15);
    }

    #[test]
    fn canonical_metadata_observes_noise_analysis_step_lifecycle() {
        let source = r#"
module lifecycle_noise(p, n);
    inout p, n; electrical p, n;
    real power;
    analog begin
        power = 10.0;
        @(initial_step("noise")) power = power + 1.0;
        @(final_step("noise")) power = power + 2.0;
        I(p, n) <+ white_noise(power, "lifecycle");
    end
endmodule
"#;

        for (initial, final_step, expected, phase) in [
            (false, false, 10.0, "middle point"),
            (false, true, 12.0, "final point of a multi-point analysis"),
            (true, true, 13.0, "single-point analysis"),
        ] {
            let mut device = device(source);
            device
                .try_set_analysis_step(initial, final_step)
                .expect("analysis-step lifecycle configures");

            let selected_backend = device
                .try_noise_sources(&[0.0])
                .expect("selected backend noise metadata evaluates");
            assert_eq!(selected_backend.len(), 1, "{phase}");
            assert!(
                (selected_backend[0].psd - expected).abs() < 1.0e-12,
                "{phase}"
            );

            let grouped = device
                .try_noise_processes_at_frequency(&[0.0], 1.0e3)
                .expect("canonical grouped metadata evaluates");
            assert_eq!(grouped.len(), 1, "{phase}");
            assert!((grouped[0].psd - expected).abs() < 1.0e-12, "{phase}");
        }
    }

    #[test]
    fn mismatched_canonical_artifact_is_rejected() {
        let compiler =
            rspice_veriloga::VerilogACompiler::new(rspice_veriloga::CompilerOptions::default());
        let first = "module mismatch(p,n); inout p,n; electrical p,n; analog I(p,n)<+white_noise(1.0); endmodule";
        let second = "module mismatch(p,n); inout p,n; electrical p,n; analog I(p,n)<+white_noise(2.0); endmodule";
        let model = compiler.compile(first).expect("first model compiles");
        let canonical = compiler
            .compile_canonical_ir(second)
            .expect("second artifact compiles");
        let error =
            VerilogADevice::try_new_with_canonical_ir("A1", Arc::new(model), &canonical, &[1, 0])
                .expect_err("source-mismatched artifact must fail");
        assert!(
            error.to_string().contains("artifact/model mismatch"),
            "{error}"
        );
    }

    #[test]
    fn canonical_validator_rejects_corrupt_grounded_current_stamp_endpoints() {
        use rspice_veriloga::codegen::StampIndex;

        fn compile_pair(
            source: &str,
        ) -> (
            rspice_veriloga::CompiledModel,
            rspice_veriloga::canonical_ir::CanonicalIrArtifact,
        ) {
            let compiler =
                rspice_veriloga::VerilogACompiler::new(rspice_veriloga::CompilerOptions::default());
            (
                compiler
                    .compile(source)
                    .expect("grounded current model compiles"),
                compiler
                    .compile_canonical_ir(source)
                    .expect("grounded current canonical IR compiles"),
            )
        }

        for (module_name, contribution) in [
            ("current_to_ground", "I(p, 0) <+ V(p, 0)"),
            ("current_from_ground", "I(0, p) <+ V(p, 0)"),
        ] {
            let source = format!(
                "module {module_name}(p, spare); inout p, spare; electrical p, spare; analog {contribution}; endmodule"
            );
            let (model, canonical) = compile_pair(&source);
            VerilogADevice::try_new_with_canonical_ir(
                "Avalid",
                Arc::new(model.clone()),
                &canonical,
                &[1, 2],
            )
            .expect("valid grounded current stamp matches canonical endpoints");

            let source_row = model.stamp_programs[0]
                .stamp_locations
                .iter()
                .position(|location| !matches!(location.row, StampIndex::Ground))
                .expect("grounded current stamp has one circuit row");

            for corrupt_row in [
                StampIndex::Terminal(1),
                StampIndex::Terminal(usize::MAX),
                StampIndex::Branch(0),
            ] {
                let mut corrupt = model.clone();
                corrupt.stamp_programs[0].stamp_locations[source_row].row = corrupt_row;
                let error = VerilogADevice::try_new_with_canonical_ir(
                    "Acorrupt",
                    Arc::new(corrupt),
                    &canonical,
                    &[1, 2],
                )
                .expect_err("corrupt grounded current endpoint must be rejected");
                let message = error.to_string();
                assert!(
                    message.contains("artifact/model mismatch")
                        || message.contains("invalid canonical IR"),
                    "{error}"
                );
            }

            let mut reversed = model;
            reversed.stamp_programs[0].stamp_locations[source_row].sign *= -1.0;
            let error = VerilogADevice::try_new_with_canonical_ir(
                "Areversed",
                Arc::new(reversed),
                &canonical,
                &[1, 2],
            )
            .expect_err("reversed grounded current endpoint must be rejected");
            let message = error.to_string();
            assert!(
                message.contains("artifact/model mismatch")
                    || message.contains("invalid canonical IR"),
                "{error}"
            );
        }
    }

    #[test]
    fn canonical_validator_rejects_corrupt_grouped_noise_injection_routing() {
        use rspice_veriloga::codegen::StampIndex;

        let source = r#"
module corrupt_noise_routing(p, n);
    inout p, n; electrical p, n;
    analog I(p, n) <+ white_noise(1.0, "routing");
endmodule
"#;
        let compiler =
            rspice_veriloga::VerilogACompiler::new(rspice_veriloga::CompilerOptions::default());
        let model = compiler.compile(source).expect("noise model compiles");
        let canonical = compiler
            .compile_canonical_ir(source)
            .expect("canonical noise model compiles");
        VerilogADevice::try_new_with_canonical_ir(
            "Avalid",
            Arc::new(model.clone()),
            &canonical,
            &[1, 0],
        )
        .expect("uncorrupted grouped-noise routing constructs");

        let assert_rejected = |corrupt: rspice_veriloga::CompiledModel, defect: &str| {
            let error = VerilogADevice::try_new_with_canonical_ir(
                "Acorrupt",
                Arc::new(corrupt),
                &canonical,
                &[1, 0],
            )
            .expect_err(defect);
            assert!(
                error.to_string().contains("artifact/model mismatch"),
                "{defect}: {error}"
            );
        };

        let mut endpoint_redirect = model.clone();
        endpoint_redirect.noise_sources[0].injections[0].pos = StampIndex::Terminal(1);
        assert_rejected(endpoint_redirect, "redirected endpoint must be rejected");

        let mut out_of_bounds_program = model.clone();
        out_of_bounds_program.noise_sources[0].injections[0].program_idx =
            out_of_bounds_program.stamp_programs.len();
        assert_rejected(
            out_of_bounds_program,
            "out-of-bounds program identity must be rejected",
        );

        let mut wrong_kind = model.clone();
        wrong_kind.noise_sources[0].injections[0].is_current = false;
        assert_rejected(wrong_kind, "mismatched contribution kind must be rejected");

        let mut wrong_ordinal = model.clone();
        wrong_ordinal.noise_sources[0].injections[0].branch_ordinal = Some(0);
        assert_rejected(wrong_ordinal, "mismatched branch ordinal must be rejected");

        let mut wrong_sign = model;
        wrong_sign.noise_sources[0].injections[0].rhs_sign = 1.0;
        assert_rejected(wrong_sign, "wrong RHS sign must be rejected");
    }

    #[test]
    fn contribution_current_suffix_required_by_stamp_fails_closed() {
        let source = r#"
module current_suffix_cycle(p, n);
    inout p, n; electrical p, n;
    real sensed, required;
    analog begin
        sensed = I(p, n);
        required = sensed;
        I(p, n) <+ required + white_noise(1.0, "live");
    end
endmodule
"#;
        let compiler =
            rspice_veriloga::VerilogACompiler::new(rspice_veriloga::CompilerOptions::default());
        let model = compiler.compile(source).expect("cycle model compiles");
        let canonical = compiler
            .compile_canonical_ir(source)
            .expect("cycle canonical IR compiles");
        let error =
            VerilogADevice::try_new_with_canonical_ir("A1", Arc::new(model), &canonical, &[1, 0])
                .expect_err("current-dependent suffix required by the stamp must fail closed");
        assert!(
            error
                .to_string()
                .contains("required before contribution-current evaluation"),
            "{error}"
        );
    }

    #[test]
    fn mixed_deterministic_noise_operator_metadata_dependency_fails_closed() {
        let source = r#"
module mixed_operator_metadata(p, n);
    inout p, n; electrical p, n;
    real mixed;
    analog begin
        mixed = absdelay(V(p, n) + white_noise(1.0, "inner"), 1.0e-9);
        I(p, n) <+ white_noise(mixed, "outer");
    end
endmodule
"#;
        let compiler =
            rspice_veriloga::VerilogACompiler::new(rspice_veriloga::CompilerOptions::default());
        let model = compiler
            .compile(source)
            .expect("mixed operator model compiles");
        let canonical = compiler
            .compile_canonical_ir(source)
            .expect("mixed operator canonical IR compiles");
        let error =
            VerilogADevice::try_new_with_canonical_ir("A1", Arc::new(model), &canonical, &[1, 0])
                .expect_err("zero-primal substitution must not corrupt later PSD metadata");
        assert!(
            error
                .to_string()
                .contains("canonical grouped-noise CFG lowering failed"),
            "{error}"
        );
    }

    #[test]
    fn runtime_indexed_noise_assignment_fails_closed() {
        let source = r#"
module indexed_noise_metadata(p, n);
    inout p, n; electrical p, n;
    real values[0:1]; integer index;
    analog begin
        index = V(p, n) > 0.0;
        values[index] = white_noise(1.0, "indexed");
        I(p, n) <+ values[index];
    end
endmodule
"#;
        let compiler =
            rspice_veriloga::VerilogACompiler::new(rspice_veriloga::CompilerOptions::default());
        let model = compiler.compile(source).expect("indexed model compiles");
        let canonical = compiler
            .compile_canonical_ir(source)
            .expect("indexed canonical IR compiles");
        let error =
            VerilogADevice::try_new_with_canonical_ir("A1", Arc::new(model), &canonical, &[1, 0])
                .expect_err("runtime-indexed grouped-noise metadata must fail closed");
        assert!(
            error.to_string().contains("run-time array index"),
            "{error}"
        );
    }

    #[test]
    fn reused_opposite_injections_cancel_before_power_is_taken() {
        let mut device = device(
            r#"
module cancel_noise(p, n);
    inout p, n;
    electrical p, n;
    real process;
    analog begin
        process = white_noise(2.0, "shared");
        I(p, n) <+ process;
        I(p, n) <+ -process;
    end
endmodule
"#,
        );
        let processes = device
            .try_noise_processes_at_frequency(&[0.0], 1.0e3)
            .expect("coherent process evaluates");
        assert_eq!(processes.len(), 1);
        let gain = processes[0]
            .injections
            .iter()
            .map(|injection| injection.gain)
            .sum::<num_complex::Complex64>();
        assert!(gain.norm() < 1.0e-12, "residual gain {gain:?}");
    }

    #[test]
    fn direct_and_delayed_reuse_cancel_at_half_period() {
        let delay = 1.0e-3;
        let mut device = device(
            r#"
module delayed_cancel(p, n);
    inout p, n;
    electrical p, n;
    real process;
    analog begin
        process = white_noise(1.0, "shared");
        I(p, n) <+ process + absdelay(process, 1.0e-3, 1.0e-3);
    end
endmodule
"#,
        );
        let processes = device
            .try_noise_processes_at_frequency(&[0.0], 0.5 / delay)
            .expect("delayed process evaluates");
        let gain = processes[0].injections[0].gain;
        assert!(gain.norm() < 1.0e-10, "residual delayed gain {gain:?}");
    }

    #[test]
    fn multiplicity_scales_current_and_potential_injection_amplitudes() {
        let mut device = device(
            r#"
module mixed_mfactor(p, n);
    inout p, n;
    electrical p, n;
    real process;
    analog begin
        process = white_noise(1.0, "shared");
        I(p, n) <+ process;
        V(p, n) <+ process;
    end
endmodule
"#,
        );
        device.set_multiplicity(4.0);
        let processes = device
            .try_noise_processes_at_frequency(&[0.0, 0.0], 1.0e3)
            .expect("mixed multiplicity process evaluates");
        assert_eq!(processes.len(), 1);
        assert_eq!(processes[0].injections.len(), 2);
        let mut magnitudes = processes[0]
            .injections
            .iter()
            .map(|injection| injection.gain.norm())
            .collect::<Vec<_>>();
        magnitudes.sort_by(f64::total_cmp);
        assert!((magnitudes[0] - 0.5).abs() < 1.0e-12);
        assert!((magnitudes[1] - 2.0).abs() < 1.0e-12);
    }

    #[test]
    fn legacy_two_source_artifact_gets_distinct_dense_process_ids() {
        let source = r#"
module legacy_noise(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        I(p, n) <+ white_noise(2.0, "first");
        I(p, n) <+ white_noise(3.0, "second");
    end
endmodule
"#;
        let compiler =
            rspice_veriloga::VerilogACompiler::new(rspice_veriloga::CompilerOptions::default());
        let mut model = compiler.compile(source).expect("legacy model compiles");
        let canonical = compiler
            .compile_canonical_ir(source)
            .expect("legacy canonical model compiles");
        for source in &mut model.noise_sources {
            source.process_id = 0;
            source.injections.clear();
        }
        model.noise_process_schema = 0;
        let mut device =
            VerilogADevice::try_new_with_canonical_ir("A1", Arc::new(model), &canonical, &[1, 0])
                .expect("legacy noise device constructs");
        device
            .try_set_analysis_type(3)
            .expect("noise analysis configures");
        let processes = device
            .try_noise_processes_at_frequency(&[0.0], 1.0e3)
            .expect("legacy sources migrate independently");
        assert_eq!(processes.len(), 2);
        assert_eq!(processes[0].process_id, 0);
        assert_eq!(processes[1].process_id, 1);
        assert_eq!(processes[0].name, "first");
        assert_eq!(processes[1].name, "second");
        assert!((processes[0].psd - 2.0).abs() < 1.0e-12);
        assert!((processes[1].psd - 3.0).abs() < 1.0e-12);
    }

    #[test]
    fn indirect_current_constraint_uses_branch_row_and_voltage_multiplicity() {
        let source = r#"
module indirect_current_noise(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n): white_noise(1.0, "constraint") == 0.0;
endmodule
"#;
        let compiler =
            rspice_veriloga::VerilogACompiler::new(rspice_veriloga::CompilerOptions::default());
        let model = compiler.compile(source).expect("noise model compiles");
        assert!(model.stamp_programs[0].indirect);
        assert!(model.noise_sources[0].is_current);
        assert_eq!(model.noise_sources[0].branch_ordinal, Some(0));
        let canonical = compiler
            .compile_canonical_ir(source)
            .expect("canonical noise model compiles");
        let mut device =
            VerilogADevice::try_new_with_canonical_ir("A1", Arc::new(model), &canonical, &[1, 0])
                .expect("indirect-current noise device constructs");
        device.set_branch_current_indices(&[2]);
        device.set_multiplicity(4.0);
        device
            .try_set_analysis_type(3)
            .expect("noise analysis configures");
        let processes = device
            .try_noise_processes_at_frequency(&[0.0, 0.0, 0.0], 1.0e3)
            .expect("indirect-current noise evaluates");
        assert_eq!(processes.len(), 1);
        assert_eq!(processes[0].injections.len(), 1);
        assert_eq!(processes[0].injections[0].node_pos, 2);
        assert_eq!(processes[0].injections[0].node_neg, 0);
        assert!((processes[0].injections[0].gain.norm() - 0.5).abs() < 1.0e-12);
    }

    #[test]
    #[ignore = "bounded release-only grouped-noise microbenchmark"]
    fn grouped_noise_per_frequency_microbenchmark() {
        use std::hint::black_box;
        use std::time::Instant;

        fn setting(name: &str, default: usize, maximum: usize) -> usize {
            std::env::var(name)
                .ok()
                .and_then(|value| value.parse().ok())
                .unwrap_or(default)
                .clamp(1, maximum)
        }
        fn median(mut values: Vec<f64>) -> f64 {
            values.sort_by(f64::total_cmp);
            values[values.len() / 2]
        }
        fn measure(samples: usize, iterations: usize, mut operation: impl FnMut()) -> f64 {
            let mut elapsed = Vec::with_capacity(samples);
            for _ in 0..samples {
                let start = Instant::now();
                for _ in 0..iterations {
                    operation();
                }
                elapsed.push(start.elapsed().as_nanos() as f64 / iterations as f64);
            }
            median(elapsed)
        }

        let source = r#"
module grouped_noise_bench(p, n);
    inout p, n; electrical p, n;
    real process;
    analog begin
        process = white_noise(2.0, "shared");
        I(p, n) <+ process;
        I(p, n) <+ laplace_nd(process, '{1.0}, '{1.0, 1.0e-6});
    end
endmodule
"#;
        let compiler =
            rspice_veriloga::VerilogACompiler::new(rspice_veriloga::CompilerOptions::default());
        let grouped_model = compiler.compile(source).expect("benchmark model compiles");
        let canonical = compiler
            .compile_canonical_ir(source)
            .expect("benchmark canonical IR compiles");
        let mut grouped = VerilogADevice::try_new_with_canonical_ir(
            "Abench",
            Arc::new(grouped_model.clone()),
            &canonical,
            &[1, 0],
        )
        .expect("grouped benchmark device constructs");
        grouped
            .try_set_analysis_type(3)
            .expect("grouped benchmark configures");

        let mut legacy_model = grouped_model;
        legacy_model.noise_process_schema = 0;
        for source in &mut legacy_model.noise_sources {
            source.process_id = 0;
            source.injections.clear();
        }
        let mut legacy = VerilogADevice::try_new_with_canonical_ir(
            "Alegacy",
            Arc::new(legacy_model),
            &canonical,
            &[1, 0],
        )
        .expect("legacy benchmark device constructs");
        legacy
            .try_set_analysis_type(3)
            .expect("legacy benchmark configures");

        let iterations = setting("RSPICE_NOISE_BENCH_ITERATIONS", 200, 10_000);
        let samples = setting("RSPICE_NOISE_BENCH_SAMPLES", 5, 20);
        let clone_ns = measure(samples, iterations, || {
            black_box(grouped.clone());
        });
        let legacy_ns = measure(samples, iterations, || {
            let mut probe = legacy.clone();
            black_box(
                probe
                    .try_noise_processes_at_frequency(&[0.0], 1.0e6)
                    .expect("legacy benchmark evaluates"),
            );
        });
        let grouped_ns = measure(samples, iterations, || {
            let mut probe = grouped.clone();
            black_box(
                probe
                    .try_noise_processes_at_frequency(&[0.0], 1.0e6)
                    .expect("grouped benchmark evaluates"),
            );
        });
        assert!(clone_ns.is_finite() && legacy_ns.is_finite() && grouped_ns.is_finite());
        eprintln!(
            "grouped-noise benchmark: samples={samples} iterations={iterations} clone-only={clone_ns:.1} ns legacy-reference-total={legacy_ns:.1} ns grouped-total={grouped_ns:.1} ns grouped-minus-clone={:.1} ns",
            grouped_ns - clone_ns
        );
    }
}
