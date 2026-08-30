//! AArch64 machine backend.
//!
//! The checked encoder, independent verifier, shared canonical model plan,
//! executable image builder, and platform unwind publisher form the complete
//! production path used by native AArch64 target dispatch.

// Non-AArch64 hosts still compile this module in contract tests so the checked
// encoder and metadata formats remain portable-testable without executing A64.
#![cfg_attr(not(target_arch = "aarch64"), allow(dead_code))]

pub(crate) mod calling_convention;
pub(crate) mod codegen;
pub(crate) mod encoder;
pub(crate) mod image;
pub(crate) mod unwind;
pub(crate) mod verifier;

#[cfg(target_arch = "aarch64")]
use self::image::A64ImageBuilder;
#[cfg(target_arch = "aarch64")]
use super::JitResult;
#[cfg(target_arch = "aarch64")]
use super::model::{NativeEntryOffsets, NativeModel, NativeRequiredStorage};
#[cfg(target_arch = "aarch64")]
use super::model_plan::NativeModelPlan;
#[cfg(target_arch = "aarch64")]
use super::runtime::ExecutableMemory;
#[cfg(target_arch = "aarch64")]
use crate::codegen::CompiledModel;

#[cfg(target_arch = "aarch64")]
pub(super) fn compile_model_plan(
    model: &CompiledModel,
    plan: &NativeModelPlan,
) -> JitResult<NativeModel> {
    plan.validate_shape(model)?;
    let mut image = A64ImageBuilder::new();
    let assignment = image.append_assignment_pass(&plan.assignments, "assignment")?;
    let post_assignment = if plan.post_assignments.is_empty() {
        None
    } else {
        Some(image.append_assignment_pass(&plan.post_assignments, "post-assignment")?)
    };

    let parameter_defaults = plan
        .parameter_defaults
        .iter()
        .enumerate()
        .map(|(index, program)| {
            program
                .as_ref()
                .map(|program| image.append_value(program, &format!("parameter default {index}")))
                .transpose()
        })
        .collect::<JitResult<Vec<_>>>()?;
    let static_conditions = plan
        .static_conditions
        .iter()
        .enumerate()
        .map(|(index, program)| {
            program
                .as_ref()
                .map(|program| image.append_value(program, &format!("static condition {index}")))
                .transpose()
        })
        .collect::<JitResult<Vec<_>>>()?;
    let stamp_values = plan
        .stamp_values
        .iter()
        .enumerate()
        .map(|(index, program)| image.append_value(program, &format!("stamp value {index}")))
        .collect::<JitResult<Vec<_>>>()?;
    let jacobians = plan
        .jacobians
        .iter()
        .enumerate()
        .map(|(stamp, programs)| {
            programs
                .iter()
                .enumerate()
                .map(|(entry, program)| {
                    image.append_value(program, &format!("Jacobian {stamp}.{entry}"))
                })
                .collect::<JitResult<Vec<_>>>()
        })
        .collect::<JitResult<Vec<_>>>()?;
    let reactive_jacobians = plan
        .reactive_jacobians
        .iter()
        .enumerate()
        .map(|(stamp, programs)| {
            programs
                .iter()
                .enumerate()
                .map(|(entry, program)| {
                    image.append_value(program, &format!("reactive Jacobian {stamp}.{entry}"))
                })
                .collect::<JitResult<Vec<_>>>()
        })
        .collect::<JitResult<Vec<_>>>()?;
    let noise_psd = plan
        .noise_psd
        .iter()
        .enumerate()
        .map(|(index, program)| image.append_value(program, &format!("noise PSD {index}")))
        .collect::<JitResult<Vec<_>>>()?;
    let noise_exponents = plan
        .noise_exponents
        .iter()
        .enumerate()
        .map(|(index, program)| {
            program
                .as_ref()
                .map(|program| image.append_value(program, &format!("noise exponent {index}")))
                .transpose()
        })
        .collect::<JitResult<Vec<_>>>()?;
    let evaluation_kernel = image.append_fused_evaluation_kernel(
        assignment,
        &plan.stamp_values,
        &stamp_values,
        &plan.published_current_pairs,
    )?;
    let stamp_kernel = image.append_fused_stamp_kernel(
        assignment,
        &plan.stamp_values,
        &plan.jacobians,
        &stamp_values,
        &jacobians,
        &plan.published_current_pairs,
    )?;
    let entries = NativeEntryOffsets {
        assignment,
        post_assignment,
        evaluation_kernel: Some(evaluation_kernel),
        stamp_kernel: Some(stamp_kernel),
        parameter_defaults,
        static_conditions,
        stamp_values,
        jacobians,
        reactive_jacobians,
        noise_psd,
        noise_exponents,
    };
    let (bytes, entry_starts, unwind_functions) = image.finish()?;
    #[cfg(all(target_arch = "aarch64", windows))]
    let executable = {
        let mut bytes = bytes;
        let runtime_functions = unwind::append_windows_unwind_data(&mut bytes, &unwind_functions)?;
        ExecutableMemory::allocate_with_aarch64_unwind(&bytes, &runtime_functions)?
    };
    #[cfg(all(target_arch = "aarch64", unix))]
    let executable = ExecutableMemory::allocate_with_aarch64_unwind(&bytes, &unwind_functions)?;
    #[cfg(not(all(target_arch = "aarch64", any(unix, windows))))]
    let executable = ExecutableMemory::allocate(&bytes)?;
    NativeModel::from_executable_image_with_dependencies(
        model.num_terminals,
        model.internal_nodes,
        model.num_variables,
        model.parameters.len(),
        model.branch_sources.len(),
        executable,
        entries,
        entry_starts,
        plan.current_dependencies.clone(),
        NativeRequiredStorage::for_model(model),
    )
}

#[cfg(all(test, target_arch = "aarch64"))]
mod tests {
    use super::compile_model_plan;
    use crate::codegen::CompiledModel;
    use crate::native::EvalContext;
    use crate::native::assignment::NativeAssignment;
    use crate::native::expr::{NativeOp, NativeProgram};
    use crate::native::model::{NativeCurrentDependencies, NativeStampKernelIo};
    use crate::native::model_plan::NativeModelPlan;
    use smol_str::SmolStr;

    fn empty_model(num_variables: usize) -> CompiledModel {
        CompiledModel {
            name: SmolStr::new("aarch64_model_plan_test"),
            source_digest: SmolStr::default(),
            num_terminals: 0,
            terminal_names: Vec::new(),
            parameters: Vec::new(),
            num_variables,
            variable_names: Vec::new(),
            event_state_variables: Vec::new(),
            assignment_steps: Vec::new(),
            stamp_programs: Vec::new(),
            lookup_tables: Vec::new(),
            internal_nodes: 0,
            branch_sources: Vec::new(),
            laplace_filters: Vec::new(),
            zi_filters: Vec::new(),
            zi_filter_definitions: Vec::new(),
            noise_sources: Vec::new(),
        }
    }

    fn constant(value: f64) -> NativeProgram {
        NativeProgram::from_ops_for_test(vec![NativeOp::Const(value)], 1, Vec::new(), Vec::new())
    }

    #[cfg(target_arch = "aarch64")]
    #[test]
    fn shared_model_plan_publishes_through_aarch64_backend() {
        let model = empty_model(1);
        let plan = NativeModelPlan {
            assignments: vec![NativeAssignment::Direct {
                var_index: 0,
                program: constant(3.0),
            }],
            post_assignments: Vec::new(),
            parameter_defaults: Vec::new(),
            static_conditions: Vec::new(),
            stamp_values: Vec::new(),
            jacobians: Vec::new(),
            reactive_jacobians: Vec::new(),
            noise_psd: Vec::new(),
            noise_exponents: Vec::new(),
            published_current_pairs: Vec::new(),
            current_dependencies: NativeCurrentDependencies::default(),
        };
        let native = compile_model_plan(&model, &plan).expect("compile shared AArch64 model plan");
        let context = EvalContext::empty_for_test();
        let mut variables = [0.0_f64];
        native.run_assignments(&context, variables.as_mut_ptr());
        assert_eq!(variables, [3.0]);

        variables[0] = 0.0;
        let io = NativeStampKernelIo {
            program_active: std::ptr::null(),
            jacobians: std::ptr::null_mut(),
        };
        assert!(native.run_evaluation_kernel(&context, variables.as_mut_ptr(), &io));
        assert_eq!(variables, [3.0]);
        assert_eq!(native.plan_stats().total_entry_points(), 3);
        assert!(context.take_runtime_error().is_none());
    }
}
