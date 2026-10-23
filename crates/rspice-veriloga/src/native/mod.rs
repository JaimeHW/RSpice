//! Native JIT backend for Verilog-A models.
//!
//! Native mode is full JIT or error. The bytecode interpreter is not a
//! fallback path when this module is asked to compile a model.

pub(crate) mod aarch64;
mod abi;
pub mod bench;
#[cfg(all(
    test,
    feature = "native",
    feature = "rust-codegen",
    target_arch = "x86_64"
))]
mod branch_agreement;
pub(crate) mod cache;
#[cfg(all(test, feature = "native", target_arch = "x86_64"))]
mod census_filter;
#[cfg(all(
    test,
    feature = "native",
    feature = "rust-codegen",
    target_arch = "x86_64"
))]
pub(crate) mod census_models;
#[cfg(all(
    test,
    feature = "native",
    feature = "rust-codegen",
    target_arch = "x86_64"
))]
mod cfg_census;
#[cfg(all(
    test,
    feature = "native",
    feature = "rust-codegen",
    target_arch = "x86_64"
))]
mod cfg_mir_census;
#[cfg(all(
    test,
    feature = "native",
    feature = "rust-codegen",
    target_arch = "x86_64"
))]
mod cfg_noise_pins;
#[cfg(all(
    test,
    feature = "native",
    feature = "rust-codegen",
    target_arch = "x86_64"
))]
mod cfg_prelude_census;
#[cfg(all(
    test,
    feature = "native",
    feature = "rust-codegen",
    target_arch = "x86_64"
))]
mod cfg_prelude_round_trip;
#[cfg(all(
    test,
    feature = "native",
    feature = "rust-codegen",
    target_arch = "x86_64"
))]
mod code_identity;
#[cfg(all(
    test,
    feature = "native",
    feature = "rust-codegen",
    target_arch = "x86_64"
))]
mod double_double;
#[cfg(all(
    test,
    feature = "native",
    any(target_arch = "x86_64", target_arch = "aarch64")
))]
mod evaluation_state;
#[cfg(all(
    test,
    feature = "native",
    feature = "rust-codegen",
    target_arch = "x86_64"
))]
mod json_float_census;
#[cfg(all(
    test,
    feature = "native",
    any(target_arch = "x86_64", target_arch = "aarch64")
))]
mod math_sharing;
#[cfg(all(
    test,
    feature = "native",
    feature = "rust-codegen",
    target_arch = "x86_64"
))]
mod mir_postfix;
mod model;
mod runtime;
#[cfg(all(
    test,
    feature = "native",
    feature = "rust-codegen",
    target_arch = "x86_64"
))]
mod state_pairing_census;
mod target;
/// A machine backend for another instruction set is dead code on this host:
/// its dispatch is compiled out, and so is every test that reaches into it.
/// [`aarch64`] states the mirror image of this inside itself; the x64 module
/// is stated here so the two backends carry the same rule without this crate's
/// x64 tree being edited for a lint that is about where it is *not* compiled.
///
/// The lint is untouched on x86-64, which is where it earns its keep.
#[cfg_attr(not(target_arch = "x86_64"), allow(dead_code))]
pub mod x64;

pub(crate) use crate::jit::{assignment, expr, model_plan, plan_program, ssa, value_cache};

pub use crate::jit::{JitError, JitResult};
pub use abi::{
    EvalContext, rspice_absdelay_state_native, rspice_laplace_derivative_native,
    rspice_laplace_step_native, rspice_limexp, rspice_limited_exp, rspice_slew_derivative_native,
    rspice_slew_state_native, rspice_timer_state_native, rspice_transition_derivative_native,
    rspice_transition_state_native, rspice_zi_derivative_native, rspice_zi_step_native,
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
///
/// # Where 16 MiB comes from
///
/// The largest image in the shipped estate is `hisimhv_n5_va` at 12,888,476
/// bytes — 12.29 MiB — with `hisimhv_va` and `hisimhv_n4_va` a few thousand
/// bytes behind it and nothing else above 5.6 MB
/// ([`crate::native::code_identity`] prints all forty-three). 16 MiB is the
/// smallest power-of-two number of mebibytes that clears it: it leaves 23.2 per
/// cent of the ceiling unused, and the next size down is below the image
/// outright.
///
/// It was 60 MiB, set when a CFG plan still ran the whole postfix assignment
/// pass before its own prelude and those three images were 57 MiB each. Nothing
/// about the ceiling made them that size and nothing about lowering it makes
/// them smaller; it is a tripwire, and one set four and a half times above the
/// estate would not trip on anything short of a catastrophe.
pub const SHIPPED_MODEL_NATIVE_CODE_SIZE_BUDGET_BYTES: usize = 16 * 1024 * 1024;

/// How many instructions an entry may hold and still be inlined into a fused
/// kernel rather than called there.
///
/// **This is a time guard, not a size break-even.** On bytes a call wins almost
/// immediately: the call sequence — the argument moves, the `call`/`bl`, the
/// stack adjustment and the runtime-status test — is about fifty bytes, against
/// twenty to thirty-four bytes for every inlined instruction, so an entry of
/// three instructions is already cheaper to call. What the limit buys is the
/// other axis. A call costs a fixed handful of nanoseconds that an entry doing
/// almost no arithmetic cannot amortize, and under a CFG plan almost every
/// entry is exactly that: a two-instruction read of a prelude slot, in a kernel
/// that runs on every Newton iteration.
///
/// The value sits in a gap three orders of magnitude wide, which is why its
/// exact number does not matter. A CFG plan's entry is either a prelude-slot
/// load — one or two instructions — or a cone the `LiveCurrentTaint` partition
/// deferred; `l_utsoi`, the only shipped module that has any, defers eleven of
/// eleven thousand eight hundred to eighteen thousand eight hundred
/// instructions. Nothing in the corpus lands between two and eleven thousand.
///
/// Above the limit the arithmetic inverts hard: before this rule the x64 fused
/// stamp kernel held a second full copy of every one of those eleven cones,
/// 4.46 MB of a 11.87 MB image, for entries the image already carries once.
pub(crate) const FUSED_KERNEL_INLINE_LIMIT: usize = 32;

use crate::canonical_ir::CanonicalIrArtifact;
use crate::codegen::CompiledModel;

/// The host this crate is qualified to publish executable code on.
///
/// One answer for every entry point that publishes an image, because the
/// qualification is a property of the host and not of what is being compiled.
fn qualified_host_target() -> JitResult<TargetSpec> {
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
    Ok(target)
}

#[cfg(feature = "native-bytecode-contract-tests")]
pub fn compile_native(model: &CompiledModel) -> JitResult<NativeModel> {
    validate_native_coverage(model)?;

    let target = qualified_host_target()?;
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

    let target = qualified_host_target()?;
    match target.arch {
        Architecture::X64 => x64::compile_model_with_canonical_ir(model, artifact),
        Architecture::AArch64 => {
            #[cfg(target_arch = "aarch64")]
            {
                let plan = crate::jit::cfg_plan_builder::build_default_model_plan(model, artifact)?;
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

/// Compile the observation pass — the program that publishes every externally
/// observable variable — into an image of its own.
///
/// Its cost is one compile per model, paid the first time a readback is asked
/// for and never on an evaluation, which is why this is a separate entry point
/// rather than a field of the model image: a deck that never reads a named
/// variable back never emits a byte of it.
pub fn compile_observation_image_with_canonical_ir(
    model: &CompiledModel,
    artifact: &CanonicalIrArtifact,
) -> JitResult<NativeModel> {
    let target = qualified_host_target()?;
    let plan = crate::jit::plan_builder::build_observation_plan(model, artifact)?;
    match target.arch {
        Architecture::X64 => x64::compile_observation_image(model, &plan),
        Architecture::AArch64 => {
            #[cfg(target_arch = "aarch64")]
            {
                aarch64::compile_observation_image(model, &plan)
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
    use super::{EvalContext, NativeStampKernelIo, compile_native_with_canonical_ir};
    use crate::{CompilerOptions, VerilogACompiler};

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
            .expect("compile public canonical host-native model");
        // Mirror the device's evaluation storage: CFG entries consume the
        // prelude's published slots even when there are no model variables.
        let mut prelude_slots = vec![f64::NAN; native.required_storage().prelude_slots];

        let params = [2.0_f64];
        let voltages = [4.0_f64, 0.0];
        let mut currents = vec![0.0_f64; model.stamp_programs.len()];
        let mut branch_currents =
            vec![0.0_f64; (model.num_terminals + 1) * (model.num_terminals + 1)];
        let mut context = EvalContext::empty_for_test();
        context.prelude_slots = prelude_slots.as_mut_ptr();
        context.prelude_slots_len = prelude_slots.len();
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
        native.run_prelude(&context, variables.as_ptr());
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

    /// A module contributing a right-nested `?:` chain of `branches` arms:
    /// arm `i` selects `i` below `i + 0.5`, and the last else-branch is -1.
    fn conditional_chain_source(branches: usize) -> String {
        let arms = (0..branches)
            .map(|i| format!("x < {i}.5 ? {i}.0 : "))
            .collect::<String>();
        format!(
            "module chain(p, n); inout p, n; electrical p, n; real x;\n\
             analog begin x = V(p, n);\nI(p, n) <+ {arms}-1.0;\nend\nendmodule\n"
        )
    }

    /// Select arms of a right-nested `?:` chain of `branches` arms, each arm
    /// at the voltage that selects it, through the native device, built and
    /// run from a caller with a 1 MiB stack.
    fn select_conditional_chain_arms(branches: usize, selections: Vec<usize>) {
        std::thread::Builder::new()
            .stack_size(1024 * 1024)
            .spawn(move || {
                let source = conditional_chain_source(branches);
                let report = VerilogACompiler::new(CompilerOptions::default())
                    .compile_runtime(&source, None)
                    .expect("a chain of any length compiles");
                let mut device = crate::device::VerilogADevice::try_new_with_canonical_ir(
                    "X",
                    report.model,
                    &report.canonical_ir,
                    &[1, 0],
                )
                .expect("native device");
                for selected in selections {
                    device.update_voltages(&[selected as f64]);
                    let expected = if selected == branches {
                        -1.0
                    } else {
                        selected as f64
                    };
                    assert_eq!(
                        device.try_evaluate().expect("chain evaluates"),
                        vec![expected],
                        "{branches} arms, V = {selected}"
                    );
                }
            })
            .unwrap()
            .join()
            .unwrap();
    }

    /// A right-nested `?:` chain has no length limit, and the native route
    /// lowers it arm by arm: every arm of ivtest's `br_gh330` shape, a
    /// 512-arm chain, and its last else-branch.
    #[test]
    fn every_arm_of_a_512_arm_conditional_chain_runs_natively() {
        select_conditional_chain_arms(512, (0..=512).collect());
    }

    /// Every 97th arm, the last arm and the last else-branch of a 10,000-arm
    /// chain: selecting all of them would walk fifty million conditions.
    /// Release only, as the release `native::` run in CI has it: compiling
    /// the chain and building its 10,000-diamond device takes about sixteen
    /// minutes unoptimized.
    #[test]
    #[cfg(all(not(debug_assertions), not(target_arch = "aarch64")))]
    fn a_10000_arm_conditional_chain_runs_natively() {
        select_conditional_chain_arms(
            10_000,
            (0..10_000).step_by(97).chain([9_999, 10_000]).collect(),
        );
    }

    /// On AArch64 the 10,000-arm chain needs spill slots past the 4,095
    /// eight-byte slots a scaled 12-bit `ldr`/`str` offset reaches from the
    /// stack pointer, and the JIT refuses the device by name rather than
    /// address a frame it cannot reach. Release only, as above.
    #[test]
    #[cfg(all(not(debug_assertions), target_arch = "aarch64"))]
    fn a_10000_arm_conditional_chain_is_refused_by_name_on_aarch64() {
        std::thread::Builder::new()
            .stack_size(1024 * 1024)
            .spawn(|| {
                let report = VerilogACompiler::new(CompilerOptions::default())
                    .compile_runtime(&conditional_chain_source(10_000), None)
                    .expect("a chain of any length compiles");
                let error = crate::device::VerilogADevice::try_new_with_canonical_ir(
                    "X",
                    report.model,
                    &report.canonical_ir,
                    &[1, 0],
                )
                .err()
                .expect("the frame is past the AArch64 spill-slot reach");
                let message = format!("{error:?}");
                assert!(
                    message.contains("exceeds the scaled imm12 frame range"),
                    "{message}"
                );
            })
            .unwrap()
            .join()
            .unwrap();
    }

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
            .expect("compile stateful public host-native model");
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
        let mut prelude_slots = vec![f64::NAN; required.prelude_slots];
        let mut context = EvalContext::empty_for_test();
        context.prelude_slots = prelude_slots.as_mut_ptr();
        context.prelude_slots_len = prelude_slots.len();
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
        native.run_prelude(&context, variables.as_ptr());
        assert!(variables.iter().all(|value| value.is_finite()));
        assert!(context.take_runtime_error().is_none());

        let reactive = native
            .run_reactive_jacobian(0, 0, &context, variables.as_ptr())
            .expect("compiled reactive Jacobian");
        assert!(reactive.is_finite());
        assert!(context.take_runtime_error().is_none());
    }
}
