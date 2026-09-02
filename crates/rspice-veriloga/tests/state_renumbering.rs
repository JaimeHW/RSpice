//! One canonical operator site owns one runtime state record.
//!
//! The bytecode generator allocated a scalar-state slot at every *emission* of
//! an integration operator, and it compiles one source operator more than once:
//! `I(p,n) <+ V(p,n)*ddt(V(p,n))` puts a `ddt` in the contribution's value
//! program and, by the product rule, a second one in its resistive derivative.
//! The compiler now rewrites those slots into the canonical per-site numbering
//! before either runtime sees the model.
//!
//! These tests run unchanged in both runtime configurations. Without
//! `feature = "native"` the device interprets the renumbered bytecode; with it,
//! the device is a JIT image compiled from the same model, and neither
//! constructs at all if the numbering is inconsistent. Both are asserted
//! against the *same* exact expectations — the un-renumbered twin's trajectory,
//! bit for bit — so a route that disagreed with the other would have to
//! disagree with its own twin first.

use std::sync::Arc;

use rspice_veriloga::canonical_ir::{
    CanonicalIrArtifact, CanonicalStateFamily, CanonicalStateLayout,
};
use rspice_veriloga::codegen::{AssignmentStep, BytecodeProgram, CompiledModel, Instruction};
use rspice_veriloga::device::VerilogADevice;
use rspice_veriloga::{CompilerOptions, VerilogACompiler};

/// A `ddt` the generator compiles twice: once in the contribution's value and
/// once in the derivative the product rule leaves it in.
const TWICE_EMITTED_DDT: &str = r#"
`include "disciplines.vams"
module renumbered_ddt(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ V(p, n) * ddt(V(p, n));
endmodule
"#;

/// Compile through the entry the engine's `.VERILOGA` cache reads, which is
/// where the renumbering attaches.
fn compile_runtime_pair(source: &str, stem: &str) -> (CompiledModel, CanonicalIrArtifact) {
    let directory = std::env::temp_dir().join(format!("rspice-state-renumbering-{stem}"));
    std::fs::create_dir_all(&directory).expect("scratch directory");
    let path = directory.join(format!("{stem}.va"));
    std::fs::write(&path, source).expect("write scratch source");
    let compiled = VerilogACompiler::new(CompilerOptions::default())
        .compile_file_runtime_with_metadata(&path, None)
        .expect("runtime compilation succeeds");
    let _ = std::fs::remove_file(&path);
    (compiled.model, compiled.canonical_ir)
}

/// Every integration-family slot the model addresses, in the order the walk
/// meets them.
///
/// Deliberately written against the public model rather than reusing the
/// compiler's own traversal: a test that asked the renumbering which slots
/// exist would agree with it by construction.
fn integration_slots(model: &CompiledModel) -> Vec<usize> {
    fn from_program(program: &BytecodeProgram, out: &mut Vec<usize>) {
        for instruction in &program.instructions {
            match instruction {
                Instruction::DdtState(slot)
                | Instruction::IdtState(slot)
                | Instruction::IdtModState(slot)
                | Instruction::LimitState(slot)
                | Instruction::CanonicalLimitState(slot) => out.push(*slot),
                _ => {}
            }
        }
    }

    fn from_steps(steps: &[AssignmentStep], out: &mut Vec<usize>) {
        for step in steps {
            match step {
                AssignmentStep::Assign(assignment) => from_program(&assignment.program, out),
                AssignmentStep::AssignIndexed { index, value, .. } => {
                    from_program(value, out);
                    from_program(index, out);
                }
                AssignmentStep::Loop { condition, body } => {
                    from_program(condition, out);
                    from_steps(body, out);
                }
            }
        }
    }

    let mut slots = Vec::new();
    for parameter in &model.parameters {
        for program in parameter
            .default_program
            .iter()
            .chain(parameter.min_program.iter())
            .chain(parameter.max_program.iter())
            .chain(parameter.exclude_programs.iter())
        {
            from_program(program, &mut slots);
        }
    }
    from_steps(&model.assignment_steps, &mut slots);
    from_steps(&model.noise_assignment_steps, &mut slots);
    for stamp in &model.stamp_programs {
        from_program(&stamp.value_program, &mut slots);
        if let Some(condition) = &stamp.static_condition {
            from_program(condition, &mut slots);
        }
        for entry in stamp
            .jacobian_programs
            .iter()
            .chain(stamp.reactive_jacobians.iter())
        {
            from_program(&entry.program, &mut slots);
        }
    }
    for source in &model.noise_sources {
        from_program(&source.psd_program, &mut slots);
        if let Some(program) = &source.exponent_program {
            from_program(program, &mut slots);
        }
        for injection in &source.injections {
            from_program(&injection.gain_program, &mut slots);
        }
    }
    slots
}

fn distinct(slots: &[usize]) -> Vec<usize> {
    let mut unique = slots.to_vec();
    unique.sort_unstable();
    unique.dedup();
    unique
}

#[test]
fn one_canonical_site_owns_one_integration_slot() {
    let (model, artifact) = compile_runtime_pair(TWICE_EMITTED_DDT, "one-site");
    let emitted = integration_slots(&model);
    let unique = distinct(&emitted);
    let sites =
        CanonicalStateLayout::from_hir(&artifact.hir).family_len(CanonicalStateFamily::Integration);

    assert_eq!(
        sites, 1,
        "the fixture is one `ddt` site; the canonical layout numbers {sites}"
    );
    assert!(
        emitted.len() > unique.len(),
        "the fixture must emit the same site more than once or it proves nothing: emitted \
         {emitted:?}"
    );
    assert_eq!(
        unique,
        vec![0],
        "every emission of the one `ddt` site must address slot 0; emitted {emitted:?}"
    );
}

/// The renumbering rewrites indices, not physics.
///
/// The same source compiled the two ways — through the runtime entry, which
/// renumbers, and through the bytecode-only entry, which does not — must
/// integrate the identical history. The comparison is on raw bits: an
/// integration that had picked up a different record would not be off by a
/// tolerance, it would be off by a whole step's worth of history.
#[test]
fn renumbering_leaves_the_integrated_history_unchanged() {
    let (renumbered, artifact) = compile_runtime_pair(TWICE_EMITTED_DDT, "history");
    let emitted = VerilogACompiler::new(CompilerOptions::default())
        .compile(TWICE_EMITTED_DDT)
        .expect("bytecode-only compilation succeeds");

    assert_ne!(
        integration_slots(&renumbered),
        integration_slots(&emitted),
        "the two entries must disagree about the numbering or this test compares a model with \
         itself"
    );

    let renumbered_trajectory = ramp(&renumbered, &artifact);
    let emitted_trajectory = ramp(&emitted, &artifact);

    assert_eq!(
        renumbered_trajectory
            .iter()
            .map(|value| value.to_bits())
            .collect::<Vec<_>>(),
        emitted_trajectory
            .iter()
            .map(|value| value.to_bits())
            .collect::<Vec<_>>(),
        "renumbered {renumbered_trajectory:?} vs emitted {emitted_trajectory:?}"
    );
    assert!(
        renumbered_trajectory.iter().all(|value| value.is_finite()),
        "the trajectory must be a real integration: {renumbered_trajectory:?}"
    );
    assert!(
        renumbered_trajectory
            .iter()
            .any(|value| *value != renumbered_trajectory[0]),
        "a constant trajectory would pass this test without integrating anything: \
         {renumbered_trajectory:?}"
    );
}

/// Drive one device through a uniform voltage ramp, one accepted step per
/// point.
fn ramp(model: &CompiledModel, artifact: &CanonicalIrArtifact) -> Vec<f64> {
    let mut device =
        VerilogADevice::try_new_with_canonical_ir("A1", Arc::new(model.clone()), artifact, &[1, 0])
            .expect("device constructs");
    device.set_analysis_type(2);
    device.set_timestep(0.5);

    let mut trajectory = Vec::new();
    for step in 0..5_u32 {
        let time = f64::from(step) * 0.5;
        device.set_time(time);
        device.update_voltages(&[f64::from(step)]);
        let stamped = device.try_evaluate().expect("evaluation succeeds");
        trajectory.push(stamped[0]);
        device.advance_state();
    }
    trajectory
}
