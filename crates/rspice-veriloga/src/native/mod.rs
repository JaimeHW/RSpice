//! Native JIT backend for Verilog-A models.
//!
//! Native mode is full JIT or error. The bytecode interpreter is not a
//! fallback path when this module is asked to compile a model.

pub(crate) mod aarch64;
mod abi;
pub mod bench;
mod model;
mod runtime;
mod target;
pub mod x64;

pub(crate) use crate::jit::{assignment, expr, model_plan, ssa, value_cache};

pub use crate::jit::{JitError, JitResult};
pub use abi::{
    EvalContext, rspice_absdelay_state_native, rspice_laplace_derivative_native,
    rspice_laplace_step_native, rspice_limexp, rspice_limited_exp, rspice_slew_state_native,
    rspice_timer_state_native, rspice_transition_state_native, rspice_zi_derivative_native,
    rspice_zi_step_native,
};
pub(crate) use abi::{NativeRuntimeError, NativeRuntimeErrorKind};
pub(crate) use model::NativeRequiredStorage;
pub(crate) use model::NativeStampKernelIo;
pub use model::{NativeModel, PlanStats};
pub use target::{Architecture, TargetSpec};

/// Release-qualification ceiling for any single shipped Verilog-A model's
/// complete native executable image. This is intentionally much lower than
/// the architectural image limit: exceeding it is a code-size regression even
/// when the backend could technically publish the image.
pub const SHIPPED_MODEL_NATIVE_CODE_SIZE_BUDGET_BYTES: usize = 60 * 1024 * 1024;

use crate::canonical_ir::CanonicalIrArtifact;
use crate::codegen::CompiledModel;

#[cfg(feature = "native-bytecode-contract-tests")]
pub fn compile_native(model: &CompiledModel) -> JitResult<NativeModel> {
    validate_native_coverage(model)?;

    let target = TargetSpec::host().ok_or_else(|| JitError::UnsupportedTarget {
        target: "unknown".into(),
        reason: "host architecture is not supported".into(),
    })?;
    if !cfg!(any(target_os = "macos", target_os = "linux", windows)) {
        return Err(JitError::UnsupportedTarget {
            target: target.display_name().into(),
            reason: "the native JIT is qualified only for macOS, Linux, and Windows desktop hosts"
                .into(),
        });
    }
    match target.arch {
        Architecture::X64 => x64::compile_model(model),
        Architecture::AArch64 => Err(JitError::UnsupportedTarget {
            target: target.display_name().into(),
            reason: "the bytecode-only native contract compiler is x64-specific; production AArch64 compilation requires authenticated canonical IR".into(),
        }),
    }
}

pub fn compile_native_with_canonical_ir(
    model: &CompiledModel,
    artifact: &CanonicalIrArtifact,
) -> JitResult<NativeModel> {
    validate_native_coverage(model)?;

    let target = TargetSpec::host().ok_or_else(|| JitError::UnsupportedTarget {
        target: "unknown".into(),
        reason: "host architecture is not supported".into(),
    })?;
    if !cfg!(any(target_os = "macos", target_os = "linux", windows)) {
        return Err(JitError::UnsupportedTarget {
            target: target.display_name().into(),
            reason: "the native JIT is qualified only for macOS, Linux, and Windows desktop hosts"
                .into(),
        });
    }
    match target.arch {
        Architecture::X64 => x64::compile_model_with_canonical_ir(model, artifact),
        Architecture::AArch64 => {
            #[cfg(target_arch = "aarch64")]
            {
                let plan =
                    crate::jit::plan_builder::build_model_plan_with_canonical_ir(model, artifact)?;
                aarch64::compile_model_plan(model, &plan)
            }
            #[cfg(not(target_arch = "aarch64"))]
            {
                Err(JitError::UnsupportedTarget {
                    target: target.display_name().into(),
                    reason: "AArch64 target dispatch cannot execute on this compilation host"
                        .into(),
                })
            }
        }
    }
}

fn validate_native_coverage(model: &CompiledModel) -> JitResult<()> {
    crate::jit::coverage::validate_jit_coverage(model)
}

#[cfg(test)]
mod tests {
    #[cfg(target_arch = "aarch64")]
    use super::{EvalContext, NativeStampKernelIo, compile_native_with_canonical_ir};
    #[cfg(target_arch = "aarch64")]
    use crate::{CompilerOptions, VerilogACompiler};

    #[cfg(target_arch = "aarch64")]
    #[test]
    fn public_canonical_dispatch_compiles_and_executes_real_verilog_a() {
        let source = r#"
`include "disciplines.vams"
module arm64_resistor(p, n);
  inout p, n;
  electrical p, n;
  parameter real resistance = 2.0;
  real voltage;
  analog begin
    voltage = V(p, n);
    I(p, n) <+ voltage / resistance;
  end
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler
            .compile(source)
            .expect("compile Verilog-A bytecode");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile canonical Verilog-A IR");
        let native = compile_native_with_canonical_ir(&model, &artifact)
            .expect("compile public canonical ARM64 native model");

        let params = [2.0_f64];
        let voltages = [4.0_f64, 0.0];
        let mut currents = vec![0.0_f64; model.stamp_programs.len()];
        let mut branch_currents =
            vec![0.0_f64; (model.num_terminals + 1) * (model.num_terminals + 1)];
        let mut context = EvalContext::empty_for_test();
        context.params = params.as_ptr();
        context.voltages = voltages.as_ptr();
        context.num_terminals = model.num_terminals;
        context.currents = currents.as_mut_ptr();
        context.currents_len = currents.len();
        context.branch_currents = branch_currents.as_mut_ptr();
        context.branch_currents_len = branch_currents.len();
        let mut variables = vec![0.0_f64; model.num_variables];

        assert_eq!(
            native.run_parameter_default(0, &context, variables.as_ptr()),
            None
        );
        native.run_assignments(&context, variables.as_mut_ptr());
        let value = native
            .run_stamp_value(0, &context, variables.as_ptr())
            .expect("compiled stamp value");
        assert!((value - 2.0).abs() < 1.0e-14);

        let active = vec![1_u8; model.stamp_programs.len()];
        let io = NativeStampKernelIo {
            program_active: active.as_ptr(),
            jacobians: std::ptr::null_mut(),
        };
        assert!(native.run_evaluation_kernel(&context, variables.as_mut_ptr(), &io));
        assert!((currents[0] - 2.0).abs() < 1.0e-14);
        assert!(native.evaluation_kernel_is_eligible());
        assert!(context.take_runtime_error().is_none());
    }

    #[cfg(target_arch = "aarch64")]
    #[test]
    fn public_canonical_dispatch_compiles_stateful_and_reactive_model() {
        let source = r#"
`include "disciplines.vams"
module arm64_stateful(p, n);
  inout p, n;
  electrical p, n;
  parameter real capacitance = 1.0e-12;
  real limited_voltage;
  analog begin
    limited_voltage = $limit(V(p, n), 0.25);
    I(p, n) <+ limited_voltage + ddt(capacitance * V(p, n));
  end
endmodule
"#;
        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let model = compiler
            .compile(source)
            .expect("compile stateful Verilog-A bytecode");
        let artifact = compiler
            .compile_canonical_ir(source)
            .expect("compile stateful canonical Verilog-A IR");
        let native = compile_native_with_canonical_ir(&model, &artifact)
            .expect("compile stateful public ARM64 native model");
        let required = native.required_storage();
        assert!(required.state_values > 0);
        assert!(
            native.plan_stats().reactive_jacobian_entry_points > 0,
            "ddt contribution must retain reactive Jacobian entries"
        );

        let params = [1.0e-12_f64];
        let voltages = [1.0_f64, 0.0];
        let mut states = vec![0.0_f64; required.state_values];
        let mut initialized = vec![0_u8; required.state_initialized];
        let mut candidate_valid = vec![0_u8; required.state_candidate_valid];
        let mut context = EvalContext::empty_for_test();
        context.params = params.as_ptr();
        context.voltages = voltages.as_ptr();
        context.state_values = states.as_mut_ptr();
        context.state_values_len = states.len();
        context.state_initialized = initialized.as_mut_ptr();
        context.state_initialized_len = initialized.len();
        context.state_candidate_valid = candidate_valid.as_mut_ptr();
        context.state_candidate_valid_len = candidate_valid.len();
        let mut variables = vec![0.0_f64; model.num_variables];
        native.run_assignments(&context, variables.as_mut_ptr());
        assert!(variables.iter().all(|value| value.is_finite()));
        assert!(context.take_runtime_error().is_none());

        let reactive = native
            .run_reactive_jacobian(0, 0, &context, variables.as_ptr())
            .expect("compiled reactive Jacobian");
        assert!(reactive.is_finite());
        assert!(context.take_runtime_error().is_none());
    }
}
