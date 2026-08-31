//! Native JIT benchmark helpers used by `rspice-bench`.
//!
//! This module stays inside `rspice-veriloga` so benchmarks can exercise
//! native entrypoints without exposing the low-level executable-code ABI as a
//! stable public interface.

use super::{EvalContext, NativeModel, PlanStats, compile_native_with_canonical_ir};
use crate::codegen::{
    AssignmentStep, BytecodeProgram, ColumnAxis, CompiledModel, Instruction, StampIndex,
};
use crate::device::VerilogADevice;
use crate::vm::{Vm, VmContext, VmError};
use crate::{CompilerOptions, VerilogACompiler};
use serde::Serialize;
use web_time::Instant;

const DEFAULT_ITERATIONS: usize = 200_000;
const DEFAULT_SAMPLES: usize = 7;
const MAX_RUNTIME_LOOP_ITERATIONS: usize = 100_000;
const DEVICE_NODE_MAPPING: &[usize] = &[1, 0, 2];
const DEVICE_CIRCUIT_VOLTAGES: &[f64] = &[0.8, 0.2];
const DEVICE_MATRIX_DIMENSION: usize = 2;
const DENSE_MODEL_SOURCE: &str = r#"
`include "disciplines.vams"
module native_jit_dense_perf_guard(p, n, ctrl);
  inout p, n, ctrl;
  electrical p, n, ctrl;
  parameter real base = 1.5;
  parameter real gain = base * 2.0;
  parameter real enable_a = 1.0;
  parameter real enable_b = 1.0;
  real vp;
  real vc;
  real g0;
  real g1;
  real g2;
  real g3;
  real g4;
  real g5;
  real accum;
  analog begin
    vp = V(p, n);
    vc = V(ctrl, n);
    g0 = gain + vc * 0.1;
    g1 = g0 * g0 + 0.25;
    g2 = sqrt(g1) + exp(0.01 * vp);
    g3 = sin(vc) - cos(vp);
    g4 = tanh(g3 * 0.125) + ln(g1 + 1.0);
    g5 = atan(g4) + hypot(g2, g3);
    accum = g0 + g1 + g2 + g3 + g4 + g5;
    if (enable_a > 0.5) begin
      I(p, n) <+ g0 * vp;
      I(p, n) <+ g1 * vp;
      I(ctrl, n) <+ 0.5 * vc;
    end
    if (enable_b > 0.5) begin
      I(p, n) <+ accum * 0.125;
    end
    I(p, n) <+ g2 * vp + g3;
    I(p, n) <+ g5 * vp - g4;
    I(p, n) <+ white_noise(1.0e-18 * (1.0 + abs(vp)), "thermal");
    I(p, n) <+ flicker_noise(2.0e-18 * (1.0 + abs(vc)), 1.0, "flicker");
  end
endmodule
"#;

#[derive(Debug, Clone, Copy)]
pub struct NativeBenchConfig {
    pub iterations: usize,
    pub samples: usize,
    pub min_dense_speedup: f64,
    pub min_speedup: f64,
    pub min_full_stamp_speedup: f64,
    pub max_native_setup_ms: Option<f64>,
    pub max_native_p95_ns_per_sweep: Option<f64>,
    pub max_relative_stddev: Option<f64>,
    pub max_native_code_bytes: Option<usize>,
}

impl Default for NativeBenchConfig {
    fn default() -> Self {
        Self {
            iterations: DEFAULT_ITERATIONS,
            samples: DEFAULT_SAMPLES,
            min_dense_speedup: 2.0,
            min_speedup: 3.0,
            min_full_stamp_speedup: 2.0,
            max_native_setup_ms: None,
            max_native_p95_ns_per_sweep: None,
            max_relative_stddev: None,
            max_native_code_bytes: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct NativeBenchReport {
    pub generated_with: &'static str,
    pub methodology: &'static str,
    pub target: String,
    pub iterations: usize,
    pub samples: usize,
    pub min_dense_speedup: f64,
    pub min_speedup: f64,
    pub min_full_stamp_speedup: f64,
    pub max_native_setup_ms: Option<f64>,
    pub max_native_p95_ns_per_sweep: Option<f64>,
    pub max_relative_stddev: Option<f64>,
    pub max_native_code_bytes: Option<usize>,
    pub cases: Vec<NativeBenchCaseReport>,
    pub passed: bool,
}

#[derive(Debug, Serialize)]
pub struct NativeBenchCaseReport {
    pub name: &'static str,
    pub plan_stats: PlanStats,
    pub model_shape: NativeBenchModelShape,
    pub native_setup_ns: f64,
    pub native_code_bytes: usize,
    pub native_ns_per_sweep: TimingStats,
    pub bytecode_ns_per_sweep: TimingStats,
    pub speedup_median: f64,
    pub required_speedup: f64,
    pub checksum_native: f64,
    pub checksum_bytecode: f64,
    pub passed: bool,
    pub failure: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct NativeBenchModelShape {
    pub parameters: usize,
    pub variables: usize,
    pub assignment_steps: usize,
    pub stamps: usize,
    pub jacobians: usize,
    pub reactive_jacobians: usize,
    pub noise_entries: usize,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct TimingStats {
    pub min: f64,
    pub median: f64,
    pub p95: f64,
    pub mean: f64,
    pub standard_deviation: f64,
    pub relative_standard_deviation: f64,
}

pub fn run_native_x64_benchmarks(config: NativeBenchConfig) -> Result<NativeBenchReport, String> {
    if config.iterations == 0 {
        return Err("native JIT benchmark iterations must be greater than zero".into());
    }
    if config.samples == 0 {
        return Err("native JIT benchmark samples must be greater than zero".into());
    }
    if !config.min_speedup.is_finite() || config.min_speedup < 0.0 {
        return Err("native JIT minimum speedup must be finite and non-negative".into());
    }
    if !config.min_dense_speedup.is_finite() || config.min_dense_speedup < 0.0 {
        return Err("native JIT dense minimum speedup must be finite and non-negative".into());
    }
    if !config.min_full_stamp_speedup.is_finite() || config.min_full_stamp_speedup < 0.0 {
        return Err("native JIT full-stamp minimum speedup must be finite and non-negative".into());
    }
    if config
        .max_native_setup_ms
        .is_some_and(|budget| !budget.is_finite() || budget <= 0.0)
    {
        return Err("native JIT setup budget must be finite and greater than zero".into());
    }
    if config
        .max_native_p95_ns_per_sweep
        .is_some_and(|budget| !budget.is_finite() || budget <= 0.0)
    {
        return Err("native JIT p95 budget must be finite and greater than zero".into());
    }
    if config
        .max_relative_stddev
        .is_some_and(|budget| !budget.is_finite() || budget < 0.0)
    {
        return Err(
            "native JIT relative standard-deviation budget must be finite and non-negative".into(),
        );
    }
    if config.max_native_code_bytes == Some(0) {
        return Err("native JIT code-size budget must be greater than zero".into());
    }

    let target = super::TargetSpec::host()
        .map(|target| target.display_name())
        .unwrap_or_else(|| "unsupported".to_string());
    let cases = vec![
        run_dense_entrypoint_case(config)?,
        run_device_evaluate_case(config)?,
        run_device_stamp_case(config)?,
    ];
    let passed = cases.iter().all(|case| case.passed);
    Ok(NativeBenchReport {
        generated_with: "rspice-veriloga-native-bench",
        methodology: "in-process native JIT entrypoint, device-evaluation, and full-stamp sweeps \
            compared against bytecode VM references; one warmup precedes timed samples; native \
            and reference order alternates by sample; report uses median, p95, and relative \
            standard deviation in ns per sweep",
        target,
        iterations: config.iterations,
        samples: config.samples,
        min_dense_speedup: config.min_dense_speedup,
        min_speedup: config.min_speedup,
        min_full_stamp_speedup: config.min_full_stamp_speedup,
        max_native_setup_ms: config.max_native_setup_ms,
        max_native_p95_ns_per_sweep: config.max_native_p95_ns_per_sweep,
        max_relative_stddev: config.max_relative_stddev,
        max_native_code_bytes: config.max_native_code_bytes,
        cases,
        passed,
    })
}

fn run_dense_entrypoint_case(config: NativeBenchConfig) -> Result<NativeBenchCaseReport, String> {
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let model = compiler
        .compile(DENSE_MODEL_SOURCE)
        .map_err(|error| format!("compile dense native benchmark model: {error}"))?;
    let artifact = compiler
        .compile_canonical_ir(DENSE_MODEL_SOURCE)
        .map_err(|error| format!("compile dense native benchmark canonical IR: {error}"))?;
    let native_setup_start = Instant::now();
    let native = compile_native_with_canonical_ir(&model, &artifact)
        .map_err(|error| format!("compile dense model to native JIT: {error}"))?;
    let native_setup_ns = native_setup_start.elapsed().as_nanos() as f64;
    let shape = model_shape(&model);
    validate_shape(&shape, &native.plan_stats())?;

    let mut native_context = benchmark_context(&model);
    let mut bytecode_context = benchmark_context(&model);
    resolve_native_defaults(&model, &native, &mut native_context)?;
    resolve_bytecode_defaults(&model, &mut bytecode_context)?;

    let checksum_native = run_native_sweep(&model, &native, &mut native_context)?;
    let checksum_bytecode = run_bytecode_sweep(&model, &mut bytecode_context)?;
    assert_close(
        "dense native benchmark checksum",
        checksum_bytecode,
        checksum_native,
    )?;

    let mut native_warmup = native_context.clone();
    let mut bytecode_warmup = bytecode_context.clone();
    std::hint::black_box(run_native_sample(
        &model,
        &native,
        &mut native_warmup,
        (config.iterations / 10).max(1),
    )?);
    std::hint::black_box(run_bytecode_sample(
        &model,
        &mut bytecode_warmup,
        (config.iterations / 10).max(1),
    )?);

    let mut native_samples = Vec::with_capacity(config.samples);
    let mut bytecode_samples = Vec::with_capacity(config.samples);
    let mut native_checksum = 0.0;
    let mut bytecode_checksum = 0.0;
    for sample_index in 0..config.samples {
        if sample_index.is_multiple_of(2) {
            let mut context = native_context.clone();
            let start = Instant::now();
            native_checksum += run_native_sample(&model, &native, &mut context, config.iterations)?;
            native_samples.push(start.elapsed().as_nanos() as f64 / config.iterations as f64);

            let mut context = bytecode_context.clone();
            let start = Instant::now();
            bytecode_checksum += run_bytecode_sample(&model, &mut context, config.iterations)?;
            bytecode_samples.push(start.elapsed().as_nanos() as f64 / config.iterations as f64);
        } else {
            let mut context = bytecode_context.clone();
            let start = Instant::now();
            bytecode_checksum += run_bytecode_sample(&model, &mut context, config.iterations)?;
            bytecode_samples.push(start.elapsed().as_nanos() as f64 / config.iterations as f64);

            let mut context = native_context.clone();
            let start = Instant::now();
            native_checksum += run_native_sample(&model, &native, &mut context, config.iterations)?;
            native_samples.push(start.elapsed().as_nanos() as f64 / config.iterations as f64);
        }
    }

    let native_stats = timing_stats(native_samples);
    let bytecode_stats = timing_stats(bytecode_samples);
    let speedup_median = bytecode_stats.median / native_stats.median.max(f64::MIN_POSITIVE);
    let native_code_bytes = native.code_size_bytes();
    let failures = benchmark_failures(
        config,
        config.min_dense_speedup,
        native_setup_ns,
        native_code_bytes,
        native_stats,
        speedup_median,
        native_checksum,
        bytecode_checksum,
    );

    Ok(NativeBenchCaseReport {
        name: "dense_entrypoint_sweep",
        plan_stats: native.plan_stats(),
        model_shape: shape,
        native_setup_ns,
        native_code_bytes,
        native_ns_per_sweep: native_stats,
        bytecode_ns_per_sweep: bytecode_stats,
        speedup_median,
        required_speedup: config.min_dense_speedup,
        checksum_native: std::hint::black_box(native_checksum),
        checksum_bytecode: std::hint::black_box(bytecode_checksum),
        passed: failures.is_empty(),
        failure: (!failures.is_empty()).then(|| failures.join("; ")),
    })
}

fn run_device_evaluate_case(config: NativeBenchConfig) -> Result<NativeBenchCaseReport, String> {
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let model = compiler
        .compile(DENSE_MODEL_SOURCE)
        .map_err(|error| format!("compile device native benchmark model: {error}"))?;
    let artifact = compiler
        .compile_canonical_ir(DENSE_MODEL_SOURCE)
        .map_err(|error| format!("compile device native benchmark canonical IR: {error}"))?;
    let shape = model_shape(&model);
    let native_setup_start = Instant::now();
    let mut native_device = VerilogADevice::try_new_with_canonical_ir(
        "native-jit-device-bench",
        model.clone(),
        &artifact,
        DEVICE_NODE_MAPPING,
    )
    .map_err(|error| format!("construct native device benchmark model: {error}"))?;
    let native_setup_ns = native_setup_start.elapsed().as_nanos() as f64;
    let plan_stats = native_device.native_plan_stats();
    validate_shape(&shape, &plan_stats)?;

    let mut bytecode_context = benchmark_context(&model);
    resolve_bytecode_defaults(&model, &mut bytecode_context)?;

    let checksum_native = run_native_device_evaluate_sweep(&mut native_device)?;
    let checksum_bytecode = run_bytecode_evaluate_sweep(&model, &mut bytecode_context)?;
    assert_close(
        "device native benchmark checksum",
        checksum_bytecode,
        checksum_native,
    )?;

    std::hint::black_box(run_native_device_evaluate_sample(
        &mut native_device,
        (config.iterations / 10).max(1),
    )?);
    let mut bytecode_warmup = bytecode_context.clone();
    std::hint::black_box(run_bytecode_evaluate_sample(
        &model,
        &mut bytecode_warmup,
        (config.iterations / 10).max(1),
    )?);

    let mut native_samples = Vec::with_capacity(config.samples);
    let mut bytecode_samples = Vec::with_capacity(config.samples);
    let mut native_checksum = 0.0;
    let mut bytecode_checksum = 0.0;
    for sample_index in 0..config.samples {
        if sample_index.is_multiple_of(2) {
            let start = Instant::now();
            native_checksum +=
                run_native_device_evaluate_sample(&mut native_device, config.iterations)?;
            native_samples.push(start.elapsed().as_nanos() as f64 / config.iterations as f64);

            let mut context = bytecode_context.clone();
            let start = Instant::now();
            bytecode_checksum +=
                run_bytecode_evaluate_sample(&model, &mut context, config.iterations)?;
            bytecode_samples.push(start.elapsed().as_nanos() as f64 / config.iterations as f64);
        } else {
            let mut context = bytecode_context.clone();
            let start = Instant::now();
            bytecode_checksum +=
                run_bytecode_evaluate_sample(&model, &mut context, config.iterations)?;
            bytecode_samples.push(start.elapsed().as_nanos() as f64 / config.iterations as f64);

            let start = Instant::now();
            native_checksum +=
                run_native_device_evaluate_sample(&mut native_device, config.iterations)?;
            native_samples.push(start.elapsed().as_nanos() as f64 / config.iterations as f64);
        }
    }

    let native_stats = timing_stats(native_samples);
    let bytecode_stats = timing_stats(bytecode_samples);
    let speedup_median = bytecode_stats.median / native_stats.median.max(f64::MIN_POSITIVE);
    let native_code_bytes = native_device.native_code_size_bytes();
    let failures = benchmark_failures(
        config,
        config.min_speedup,
        native_setup_ns,
        native_code_bytes,
        native_stats,
        speedup_median,
        native_checksum,
        bytecode_checksum,
    );

    Ok(NativeBenchCaseReport {
        name: "device_evaluate_sweep",
        plan_stats,
        model_shape: shape,
        native_setup_ns,
        native_code_bytes,
        native_ns_per_sweep: native_stats,
        bytecode_ns_per_sweep: bytecode_stats,
        speedup_median,
        required_speedup: config.min_speedup,
        checksum_native: std::hint::black_box(native_checksum),
        checksum_bytecode: std::hint::black_box(bytecode_checksum),
        passed: failures.is_empty(),
        failure: (!failures.is_empty()).then(|| failures.join("; ")),
    })
}

fn run_device_stamp_case(config: NativeBenchConfig) -> Result<NativeBenchCaseReport, String> {
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let model = compiler
        .compile(DENSE_MODEL_SOURCE)
        .map_err(|error| format!("compile stamp native benchmark model: {error}"))?;
    let artifact = compiler
        .compile_canonical_ir(DENSE_MODEL_SOURCE)
        .map_err(|error| format!("compile stamp native benchmark canonical IR: {error}"))?;
    let shape = model_shape(&model);
    let native_setup_start = Instant::now();
    let mut native_device = VerilogADevice::try_new_with_canonical_ir(
        "native-jit-stamp-bench",
        model.clone(),
        &artifact,
        DEVICE_NODE_MAPPING,
    )
    .map_err(|error| format!("construct native stamp benchmark model: {error}"))?;
    let native_setup_ns = native_setup_start.elapsed().as_nanos() as f64;
    let plan_stats = native_device.native_plan_stats();
    validate_shape(&shape, &plan_stats)?;

    let mut bytecode_context = benchmark_context(&model);
    resolve_bytecode_defaults(&model, &mut bytecode_context)?;

    let native_snapshot = run_native_device_stamp_sweep(&mut native_device)?;
    let bytecode_snapshot = run_bytecode_stamp_sweep(&model, &mut bytecode_context)?;
    assert_stamp_close(&bytecode_snapshot, &native_snapshot)?;

    std::hint::black_box(run_native_device_stamp_sample(
        &mut native_device,
        (config.iterations / 10).max(1),
    )?);
    let mut bytecode_warmup = bytecode_context.clone();
    std::hint::black_box(run_bytecode_stamp_sample(
        &model,
        &mut bytecode_warmup,
        (config.iterations / 10).max(1),
    )?);

    let mut native_samples = Vec::with_capacity(config.samples);
    let mut bytecode_samples = Vec::with_capacity(config.samples);
    let mut native_checksum = 0.0;
    let mut bytecode_checksum = 0.0;
    for sample_index in 0..config.samples {
        if sample_index.is_multiple_of(2) {
            let start = Instant::now();
            native_checksum +=
                run_native_device_stamp_sample(&mut native_device, config.iterations)?;
            native_samples.push(start.elapsed().as_nanos() as f64 / config.iterations as f64);

            let mut context = bytecode_context.clone();
            let start = Instant::now();
            bytecode_checksum +=
                run_bytecode_stamp_sample(&model, &mut context, config.iterations)?;
            bytecode_samples.push(start.elapsed().as_nanos() as f64 / config.iterations as f64);
        } else {
            let mut context = bytecode_context.clone();
            let start = Instant::now();
            bytecode_checksum +=
                run_bytecode_stamp_sample(&model, &mut context, config.iterations)?;
            bytecode_samples.push(start.elapsed().as_nanos() as f64 / config.iterations as f64);

            let start = Instant::now();
            native_checksum +=
                run_native_device_stamp_sample(&mut native_device, config.iterations)?;
            native_samples.push(start.elapsed().as_nanos() as f64 / config.iterations as f64);
        }
    }

    let native_stats = timing_stats(native_samples);
    let bytecode_stats = timing_stats(bytecode_samples);
    let speedup_median = bytecode_stats.median / native_stats.median.max(f64::MIN_POSITIVE);
    let native_code_bytes = native_device.native_code_size_bytes();
    let failures = benchmark_failures(
        config,
        config.min_full_stamp_speedup,
        native_setup_ns,
        native_code_bytes,
        native_stats,
        speedup_median,
        native_checksum,
        bytecode_checksum,
    );

    Ok(NativeBenchCaseReport {
        name: "device_full_stamp_sweep",
        plan_stats,
        model_shape: shape,
        native_setup_ns,
        native_code_bytes,
        native_ns_per_sweep: native_stats,
        bytecode_ns_per_sweep: bytecode_stats,
        speedup_median,
        required_speedup: config.min_full_stamp_speedup,
        checksum_native: std::hint::black_box(native_checksum),
        checksum_bytecode: std::hint::black_box(bytecode_checksum),
        passed: failures.is_empty(),
        failure: (!failures.is_empty()).then(|| failures.join("; ")),
    })
}

fn validate_shape(shape: &NativeBenchModelShape, stats: &PlanStats) -> Result<(), String> {
    if shape.assignment_steps < 8 {
        return Err("dense benchmark model did not produce enough assignment steps".into());
    }
    if shape.stamps < 6 {
        return Err("dense benchmark model did not produce enough stamp programs".into());
    }
    if shape.jacobians < shape.stamps {
        return Err(
            "dense benchmark model did not produce Jacobian coverage for every stamp".into(),
        );
    }
    if shape.noise_entries < 3 {
        return Err("dense benchmark model did not produce expected noise entries".into());
    }
    if stats.stamp_value_entry_points != shape.stamps
        || stats.jacobian_entry_points != shape.jacobians
        || stats.noise_source_entry_points != shape.noise_entries
        || stats.stamp_kernel_entry_points != 1
    {
        return Err("native plan stats do not match dense benchmark model shape".into());
    }
    Ok(())
}

fn model_shape(model: &CompiledModel) -> NativeBenchModelShape {
    NativeBenchModelShape {
        parameters: model.parameters.len(),
        variables: model.num_variables,
        assignment_steps: count_assignment_steps(&model.assignment_steps),
        stamps: model.stamp_programs.len(),
        jacobians: model
            .stamp_programs
            .iter()
            .map(|stamp| stamp.jacobian_programs.len())
            .sum(),
        reactive_jacobians: model
            .stamp_programs
            .iter()
            .map(|stamp| stamp.reactive_jacobians.len())
            .sum(),
        noise_entries: model.noise_sources.len()
            + model
                .noise_sources
                .iter()
                .filter(|source| source.exponent_program.is_some())
                .count(),
    }
}

fn count_assignment_steps(steps: &[AssignmentStep]) -> usize {
    steps
        .iter()
        .map(|step| match step {
            AssignmentStep::Assign(_) | AssignmentStep::AssignIndexed { .. } => 1,
            AssignmentStep::Loop { body, .. } => 1 + count_assignment_steps(body),
        })
        .sum()
}

fn benchmark_context(model: &CompiledModel) -> VmContext {
    let mut context = VmContext::with_internal_nodes(model.num_terminals, model.internal_nodes);
    context.voltages = vec![0.8, 0.0, 0.2];
    context.parameters = model
        .parameters
        .iter()
        .map(|parameter| parameter.default)
        .collect();
    context.param_given = vec![0; model.parameters.len()];
    context.variables = vec![0.0; model.num_variables.max(1)];
    context.currents = vec![0.0; model.stamp_programs.len()];
    context.branch_current_values = vec![0.0; model.branch_sources.len()];
    context.lookup_tables = model.lookup_tables.clone();
    context.laplace_filters = model.laplace_filters.clone();
    context.zi_filters = model.zi_filters.clone();
    context.time = 1.0e-9;
    context.set_timestep(1.0e-12);
    context.set_integration_coefficients(crate::vm::IntegrationCoefficients::inactive());
    preallocate_context(&mut context, model);
    context
}

fn resolve_native_defaults(
    model: &CompiledModel,
    native: &NativeModel,
    context: &mut VmContext,
) -> Result<(), String> {
    for index in 0..model.parameters.len() {
        let mut ctx = eval_context_from_vm_context(context);
        let value = native.run_parameter_default(index, &ctx, context.variables.as_ptr());
        take_native_error(&mut ctx, "parameter defaults")?;
        if let Some(value) = value {
            context.parameters[index] = value;
        }
    }
    Ok(())
}

fn resolve_bytecode_defaults(model: &CompiledModel, context: &mut VmContext) -> Result<(), String> {
    for (index, parameter) in model.parameters.iter().enumerate() {
        if let Some(program) = &parameter.default_program {
            let mut vm = Vm::new(context);
            let value = vm
                .execute(program)
                .map_err(|error| format!("bytecode parameter default: {error}"))?;
            context.parameters[index] = value;
        }
    }
    Ok(())
}

fn run_native_sample(
    model: &CompiledModel,
    native: &NativeModel,
    context: &mut VmContext,
    iterations: usize,
) -> Result<f64, String> {
    let mut checksum = 0.0;
    for _ in 0..iterations {
        checksum += std::hint::black_box(run_native_sweep(model, native, context)?);
    }
    Ok(std::hint::black_box(checksum))
}

fn run_bytecode_sample(
    model: &CompiledModel,
    context: &mut VmContext,
    iterations: usize,
) -> Result<f64, String> {
    let mut checksum = 0.0;
    for _ in 0..iterations {
        checksum += std::hint::black_box(run_bytecode_sweep(model, context)?);
    }
    Ok(std::hint::black_box(checksum))
}

fn run_native_device_evaluate_sample(
    device: &mut VerilogADevice,
    iterations: usize,
) -> Result<f64, String> {
    let mut checksum = 0.0;
    for _ in 0..iterations {
        checksum += std::hint::black_box(run_native_device_evaluate_sweep(device)?);
    }
    Ok(std::hint::black_box(checksum))
}

fn run_bytecode_evaluate_sample(
    model: &CompiledModel,
    context: &mut VmContext,
    iterations: usize,
) -> Result<f64, String> {
    let mut checksum = 0.0;
    for _ in 0..iterations {
        checksum += std::hint::black_box(run_bytecode_evaluate_sweep(model, context)?);
    }
    Ok(std::hint::black_box(checksum))
}

#[derive(Debug, Clone, Copy)]
struct StampSnapshot {
    matrix: [f64; DEVICE_MATRIX_DIMENSION * DEVICE_MATRIX_DIMENSION],
    rhs: [f64; DEVICE_MATRIX_DIMENSION],
}

impl StampSnapshot {
    fn checksum(self) -> f64 {
        let matrix = self
            .matrix
            .iter()
            .enumerate()
            .map(|(index, value)| (index + 1) as f64 * value)
            .sum::<f64>();
        let rhs = self
            .rhs
            .iter()
            .enumerate()
            .map(|(index, value)| (index + self.matrix.len() + 1) as f64 * value)
            .sum::<f64>();
        matrix + rhs
    }
}

fn run_native_device_stamp_sample(
    device: &mut VerilogADevice,
    iterations: usize,
) -> Result<f64, String> {
    let mut checksum = 0.0;
    for _ in 0..iterations {
        checksum += std::hint::black_box(run_native_device_stamp_sweep(device)?.checksum());
    }
    Ok(std::hint::black_box(checksum))
}

fn run_bytecode_stamp_sample(
    model: &CompiledModel,
    context: &mut VmContext,
    iterations: usize,
) -> Result<f64, String> {
    let mut checksum = 0.0;
    for _ in 0..iterations {
        checksum += std::hint::black_box(run_bytecode_stamp_sweep(model, context)?.checksum());
    }
    Ok(std::hint::black_box(checksum))
}

fn run_native_device_stamp_sweep(device: &mut VerilogADevice) -> Result<StampSnapshot, String> {
    let mut matrix = [0.0; DEVICE_MATRIX_DIMENSION * DEVICE_MATRIX_DIMENSION];
    let mut rhs = [0.0; DEVICE_MATRIX_DIMENSION];
    let mut matrix_error = None;
    let mut rhs_error = None;
    device
        .try_stamp(
            DEVICE_CIRCUIT_VOLTAGES,
            |row, col, value| {
                if row < DEVICE_MATRIX_DIMENSION && col < DEVICE_MATRIX_DIMENSION {
                    matrix[row * DEVICE_MATRIX_DIMENSION + col] += value;
                } else {
                    matrix_error = Some(format!(
                        "native stamp emitted matrix index ({row}, {col}) outside benchmark dimension {DEVICE_MATRIX_DIMENSION}"
                    ));
                }
            },
            |row, value| {
                if let Some(slot) = rhs.get_mut(row) {
                    *slot += value;
                } else {
                    rhs_error = Some(format!(
                        "native stamp emitted RHS index {row} outside benchmark dimension {DEVICE_MATRIX_DIMENSION}"
                    ));
                }
            },
        )
        .map_err(|error| format!("native device stamp: {error}"))?;
    if let Some(error) = matrix_error.or(rhs_error) {
        return Err(error);
    }
    Ok(StampSnapshot { matrix, rhs })
}

fn run_bytecode_stamp_sweep(
    model: &CompiledModel,
    context: &mut VmContext,
) -> Result<StampSnapshot, String> {
    if model.internal_nodes != 0 || !model.branch_sources.is_empty() {
        return Err(
            "dense bytecode stamp reference only supports terminal-only current contributions"
                .into(),
        );
    }

    context.clear_currents();
    context.currents.reserve(model.stamp_programs.len());
    {
        let mut vm = Vm::new(context);
        execute_assignment_steps(&mut vm, &model.assignment_steps)
            .map_err(|error| format!("bytecode stamp assignments: {error}"))?;
    }

    let mut matrix = [0.0; DEVICE_MATRIX_DIMENSION * DEVICE_MATRIX_DIMENSION];
    let mut rhs = [0.0; DEVICE_MATRIX_DIMENSION];
    for (stamp_index, stamp) in model.stamp_programs.iter().enumerate() {
        if stamp.branch_ordinal.is_some() || stamp.indirect {
            return Err(format!(
                "dense bytecode stamp reference encountered non-current contribution {stamp_index}"
            ));
        }

        let active = if let Some(condition) = &stamp.static_condition {
            let mut vm = Vm::new(context);
            vm.execute(condition)
                .map_err(|error| format!("bytecode stamp condition {stamp_index}: {error}"))?
                != 0.0
        } else {
            true
        };
        if !active {
            continue;
        }

        let value = {
            let mut vm = Vm::new(context);
            vm.execute(&stamp.value_program)
                .map_err(|error| format!("bytecode stamp value {stamp_index}: {error}"))?
        };
        if !value.is_finite() {
            return Err(format!(
                "bytecode stamp value {stamp_index} became non-finite: {value}"
            ));
        }
        context.currents.push(value);
        let mut equivalent_source = value;

        for (entry_index, jacobian) in stamp.jacobian_programs.iter().enumerate() {
            let derivative = {
                let mut vm = Vm::new(context);
                vm.execute(&jacobian.program).map_err(|error| {
                    format!("bytecode stamp Jacobian {stamp_index}.{entry_index}: {error}")
                })?
            };
            if !derivative.is_finite() {
                return Err(format!(
                    "bytecode stamp Jacobian {stamp_index}.{entry_index} became non-finite: {derivative}"
                ));
            }
            if jacobian.sign > 0.0 {
                equivalent_source -=
                    derivative * benchmark_axis_value(context, &jacobian.col_axis)?;
            }
            if let (Some(row), Some(col)) = (
                benchmark_stamp_index(&jacobian.row)?,
                benchmark_stamp_index(&jacobian.col)?,
            ) {
                matrix[row * DEVICE_MATRIX_DIMENSION + col] += jacobian.sign * derivative;
            }
        }

        if !equivalent_source.is_finite() {
            return Err(format!(
                "bytecode equivalent source {stamp_index} became non-finite: {equivalent_source}"
            ));
        }
        for location in &stamp.stamp_locations {
            if let Some(row) = benchmark_stamp_index(&location.row)? {
                rhs[row] += location.sign * equivalent_source;
            }
        }
    }

    Ok(StampSnapshot { matrix, rhs })
}

fn benchmark_stamp_index(index: &StampIndex) -> Result<Option<usize>, String> {
    let mapped = match index {
        StampIndex::Terminal(terminal) => {
            let node = DEVICE_NODE_MAPPING.get(*terminal).copied().ok_or_else(|| {
                format!("benchmark stamp terminal {terminal} has no circuit-node mapping")
            })?;
            (node > 0).then_some(node.saturating_sub(1))
        }
        StampIndex::Ground => None,
        StampIndex::Internal(index) => {
            return Err(format!(
                "benchmark bytecode stamp reference cannot map internal node {index}"
            ));
        }
        StampIndex::Branch(index) => {
            return Err(format!(
                "benchmark bytecode stamp reference cannot map branch unknown {index}"
            ));
        }
    };
    if mapped.is_some_and(|node| node >= DEVICE_MATRIX_DIMENSION) {
        return Err(format!(
            "benchmark stamp maps to node outside dimension {DEVICE_MATRIX_DIMENSION}"
        ));
    }
    Ok(mapped)
}

fn benchmark_axis_value(context: &VmContext, axis: &ColumnAxis) -> Result<f64, String> {
    match axis {
        ColumnAxis::Node(node) if *node == usize::MAX => Ok(0.0),
        ColumnAxis::Node(node) if *node < context.terminal_count() => context
            .voltages
            .get(*node)
            .copied()
            .ok_or_else(|| format!("benchmark axis terminal {node} is unavailable")),
        ColumnAxis::Node(node) => context
            .internal_voltages
            .get(*node - context.terminal_count())
            .copied()
            .ok_or_else(|| format!("benchmark axis internal node {node} is unavailable")),
        ColumnAxis::Branch(index) => context
            .branch_current_values
            .get(*index)
            .copied()
            .ok_or_else(|| format!("benchmark axis branch unknown {index} is unavailable")),
    }
}

fn assert_stamp_close(expected: &StampSnapshot, actual: &StampSnapshot) -> Result<(), String> {
    for (index, (expected, actual)) in expected.matrix.iter().zip(actual.matrix.iter()).enumerate()
    {
        assert_close(&format!("stamp matrix entry {index}"), *expected, *actual)?;
    }
    for (index, (expected, actual)) in expected.rhs.iter().zip(actual.rhs.iter()).enumerate() {
        assert_close(&format!("stamp RHS entry {index}"), *expected, *actual)?;
    }
    Ok(())
}

fn run_native_device_evaluate_sweep(device: &mut VerilogADevice) -> Result<f64, String> {
    device
        .try_update_voltages(DEVICE_CIRCUIT_VOLTAGES)
        .map_err(|error| format!("native device voltage update: {error}"))?;
    let currents = device
        .try_evaluate()
        .map_err(|error| format!("native device evaluate: {error}"))?;
    Ok(currents.into_iter().sum())
}

fn run_native_sweep(
    model: &CompiledModel,
    native: &NativeModel,
    context: &mut VmContext,
) -> Result<f64, String> {
    context.clear_currents();
    context.currents.resize(model.stamp_programs.len(), 0.0);

    let mut ctx = eval_context_from_vm_context(context);
    native.run_assignments(&ctx, context.variables.as_mut_ptr());
    take_native_error(&mut ctx, "assignments")?;

    let mut checksum = 0.0;
    for (stamp_index, stamp) in model.stamp_programs.iter().enumerate() {
        ctx = eval_context_from_vm_context(context);
        let active = native.run_static_condition(stamp_index, &ctx, context.variables.as_ptr());
        take_native_error(&mut ctx, "static condition")?;
        if active.is_some_and(|active| active == 0.0) {
            continue;
        }

        ctx = eval_context_from_vm_context(context);
        let value = native
            .run_stamp_value(stamp_index, &ctx, context.variables.as_ptr())
            .ok_or_else(|| format!("native sweep missing stamp-value entry {stamp_index}"))?;
        take_native_error(&mut ctx, "stamp value")?;
        checksum += value;
        context.currents[stamp_index] = value;

        for entry_index in 0..stamp.jacobian_programs.len() {
            ctx = eval_context_from_vm_context(context);
            let value = native
                .run_jacobian(stamp_index, entry_index, &ctx, context.variables.as_ptr())
                .ok_or_else(|| {
                    format!("native sweep missing Jacobian entry {stamp_index}.{entry_index}")
                })?;
            take_native_error(&mut ctx, "Jacobian")?;
            checksum += value;
        }
        for entry_index in 0..stamp.reactive_jacobians.len() {
            ctx = eval_context_from_vm_context(context);
            let value = native
                .run_reactive_jacobian(stamp_index, entry_index, &ctx, context.variables.as_ptr())
                .ok_or_else(|| {
                    format!(
                        "native sweep missing reactive-Jacobian entry {stamp_index}.{entry_index}"
                    )
                })?;
            take_native_error(&mut ctx, "reactive Jacobian")?;
            checksum += value;
        }
    }

    for source_index in 0..model.noise_sources.len() {
        ctx = eval_context_from_vm_context(context);
        let psd = native
            .run_noise_psd(source_index, &ctx, context.variables.as_ptr())
            .ok_or_else(|| format!("native sweep missing noise PSD entry {source_index}"))?;
        take_native_error(&mut ctx, "noise PSD")?;
        checksum += psd;
        let exponent = native.run_noise_exponent(source_index, &ctx, context.variables.as_ptr());
        take_native_error(&mut ctx, "noise exponent")?;
        if let Some(exponent) = exponent {
            checksum += exponent;
        }
    }

    Ok(checksum)
}

fn run_bytecode_evaluate_sweep(
    model: &CompiledModel,
    context: &mut VmContext,
) -> Result<f64, String> {
    context.clear_currents();
    context.currents.resize(model.stamp_programs.len(), 0.0);

    {
        let mut vm = Vm::new(context);
        execute_assignment_steps(&mut vm, &model.assignment_steps)
            .map_err(|error| format!("bytecode evaluate assignments: {error}"))?;
    }

    let mut checksum = 0.0;
    for (stamp_index, stamp) in model.stamp_programs.iter().enumerate() {
        let active = if let Some(condition) = &stamp.static_condition {
            let mut vm = Vm::new(context);
            vm.execute(condition).map_err(|error| {
                format!("bytecode evaluate static condition {stamp_index}: {error}")
            })? != 0.0
        } else {
            true
        };
        if !active {
            continue;
        }

        let value = {
            let mut vm = Vm::new(context);
            vm.execute(&stamp.value_program)
                .map_err(|error| format!("bytecode evaluate stamp {stamp_index}: {error}"))?
        };
        checksum += value;
        context.currents[stamp_index] = value;
    }

    Ok(checksum)
}

fn run_bytecode_sweep(model: &CompiledModel, context: &mut VmContext) -> Result<f64, String> {
    context.clear_currents();
    context.currents.resize(model.stamp_programs.len(), 0.0);
    let mut checksum = 0.0;

    {
        let mut vm = Vm::new(context);
        execute_assignment_steps(&mut vm, &model.assignment_steps)
            .map_err(|error| format!("bytecode assignments: {error}"))?;
    }

    for (stamp_index, stamp) in model.stamp_programs.iter().enumerate() {
        let active = if let Some(condition) = &stamp.static_condition {
            let mut vm = Vm::new(context);
            vm.execute(condition)
                .map_err(|error| format!("bytecode static condition {stamp_index}: {error}"))?
                != 0.0
        } else {
            true
        };
        if !active {
            continue;
        }

        let value = {
            let mut vm = Vm::new(context);
            vm.execute(&stamp.value_program)
                .map_err(|error| format!("bytecode stamp {stamp_index}: {error}"))?
        };
        checksum += value;
        context.currents[stamp_index] = value;

        for (entry_index, jacobian) in stamp.jacobian_programs.iter().enumerate() {
            let mut vm = Vm::new(context);
            checksum += vm.execute(&jacobian.program).map_err(|error| {
                format!("bytecode jacobian {stamp_index}.{entry_index}: {error}")
            })?;
        }
        for (entry_index, jacobian) in stamp.reactive_jacobians.iter().enumerate() {
            let mut vm = Vm::new(context);
            checksum += vm.execute(&jacobian.program).map_err(|error| {
                format!("bytecode reactive jacobian {stamp_index}.{entry_index}: {error}")
            })?;
        }
    }

    for (source_index, source) in model.noise_sources.iter().enumerate() {
        let mut vm = Vm::new(context);
        checksum += vm
            .execute(&source.psd_program)
            .map_err(|error| format!("bytecode noise PSD {source_index}: {error}"))?;
        if let Some(program) = &source.exponent_program {
            let mut vm = Vm::new(context);
            checksum += vm
                .execute(program)
                .map_err(|error| format!("bytecode noise exponent {source_index}: {error}"))?;
        }
    }

    Ok(checksum)
}

fn execute_assignment_steps(vm: &mut Vm<'_>, steps: &[AssignmentStep]) -> Result<(), VmError> {
    for step in steps {
        match step {
            AssignmentStep::Assign(assignment) => {
                let value = vm.execute(&assignment.program)?;
                vm.context.variables[assignment.var_index] = value;
            }
            AssignmentStep::AssignIndexed {
                base,
                len,
                lower,
                index,
                value,
            } => {
                let raw_index = vm.execute(index)?;
                let slot = Vm::array_slot(raw_index, *base, *len, *lower)?;
                let value = vm.execute(value)?;
                vm.context.variables[slot] = value;
            }
            AssignmentStep::Loop { condition, body } => {
                let mut iterations = 0usize;
                while vm.execute(condition)? != 0.0 {
                    execute_assignment_steps(vm, body)?;
                    iterations += 1;
                    if iterations >= MAX_RUNTIME_LOOP_ITERATIONS {
                        return Err(VmError::InvalidInstruction(
                            "runtime loop iteration limit exceeded",
                        ));
                    }
                }
            }
        }
    }
    Ok(())
}

fn preallocate_context(context: &mut VmContext, model: &CompiledModel) {
    let mut max_state = None;
    let mut max_delay_buffer = None;
    let mut max_transition_filter = None;
    let mut max_slew_filter = None;
    let mut max_cross_detector = None;

    let mut scan_program = |program: &BytecodeProgram| {
        for instruction in &program.instructions {
            match instruction {
                Instruction::DdtState(idx)
                | Instruction::IdtState(idx)
                | Instruction::IdtModState(idx)
                | Instruction::LimitState(idx)
                | Instruction::CanonicalLimitState(idx) => update_max_slot(&mut max_state, *idx),
                Instruction::AbsDelayState(idx)
                | Instruction::AbsDelayStateMax(idx)
                | Instruction::AbsDelayStateDerivative(idx)
                | Instruction::AbsDelayStateDerivativeMax(idx) => {
                    update_max_slot(&mut max_delay_buffer, *idx)
                }
                Instruction::TransitionState(idx) | Instruction::TransitionStateDerivative(idx) => {
                    update_max_slot(&mut max_transition_filter, *idx);
                }
                Instruction::SlewState(idx) | Instruction::SlewStateDerivative(idx) => {
                    update_max_slot(&mut max_slew_filter, *idx)
                }
                Instruction::CrossState(idx)
                | Instruction::AboveState(idx)
                | Instruction::LastCrossingState(idx) => {
                    update_max_slot(&mut max_cross_detector, *idx)
                }
                _ => {}
            }
        }
    };

    for parameter in &model.parameters {
        if let Some(program) = &parameter.default_program {
            scan_program(program);
        }
    }
    scan_assignment_steps(&model.assignment_steps, &mut scan_program);
    for stamp in &model.stamp_programs {
        if let Some(condition) = &stamp.static_condition {
            scan_program(condition);
        }
        scan_program(&stamp.value_program);
        for jacobian in &stamp.jacobian_programs {
            scan_program(&jacobian.program);
        }
        for jacobian in &stamp.reactive_jacobians {
            scan_program(&jacobian.program);
        }
    }
    for source in &model.noise_sources {
        scan_program(&source.psd_program);
        if let Some(program) = &source.exponent_program {
            scan_program(program);
        }
    }

    if let Some(max_idx) = max_state {
        context.allocate_states(max_idx + 1);
    }
    if let Some(max_idx) = max_delay_buffer {
        context.allocate_delay_buffers(max_idx + 1);
    }
    if let Some(max_idx) = max_transition_filter {
        context.allocate_transition_filters(max_idx + 1);
    }
    if let Some(max_idx) = max_slew_filter {
        context.allocate_slew_filters(max_idx + 1);
    }
    if let Some(max_idx) = max_cross_detector {
        context.allocate_cross_detectors(max_idx + 1);
    }
}

fn scan_assignment_steps(
    steps: &[AssignmentStep],
    scan_program: &mut impl FnMut(&BytecodeProgram),
) {
    for step in steps {
        match step {
            AssignmentStep::Assign(assignment) => scan_program(&assignment.program),
            AssignmentStep::AssignIndexed { index, value, .. } => {
                scan_program(index);
                scan_program(value);
            }
            AssignmentStep::Loop { condition, body } => {
                scan_program(condition);
                scan_assignment_steps(body, scan_program);
            }
        }
    }
}

fn update_max_slot(slot: &mut Option<usize>, index: usize) {
    *slot = Some(slot.map_or(index, |current| current.max(index)));
}

fn eval_context_from_vm_context(context: &mut VmContext) -> EvalContext {
    let integration = context.integration_coefficients();
    EvalContext {
        voltages: context.voltages.as_ptr(),
        internal_voltages: context.internal_voltages.as_ptr(),
        params: context.parameters.as_ptr(),
        branch_currents: context.terminal_pair_currents_ptr(),
        branch_currents_len: context.terminal_pair_currents_len(),
        currents: context.currents.as_ptr(),
        currents_len: context.currents.len(),
        num_terminals: context.terminal_count(),
        port_connected: context.port_connected.as_ptr(),
        port_connected_len: context.port_connected.len(),
        temperature: context.temperature,
        time: context.time,
        timestep: context.timestep(),
        state_prev: if context.state_values_prev.is_empty() {
            std::ptr::null()
        } else {
            context.state_values_prev.as_ptr()
        },
        state_values: if context.state_values.is_empty() {
            std::ptr::null_mut()
        } else {
            context.state_values.as_mut_ptr()
        },
        state_initialized: if context.state_initialized.is_empty() {
            std::ptr::null_mut()
        } else {
            context.state_initialized.as_mut_ptr() as *mut u8
        },
        state_initialized_len: context.state_initialized.len(),
        lookup_tables: if context.lookup_tables.is_empty() {
            std::ptr::null()
        } else {
            context.lookup_tables.as_ptr()
        },
        lookup_tables_len: context.lookup_tables.len(),
        laplace_filters: if context.laplace_filters.is_empty() {
            std::ptr::null_mut()
        } else {
            context.laplace_filters.as_mut_ptr()
        },
        laplace_filters_len: context.laplace_filters.len(),
        param_given: context.param_given.as_ptr(),
        param_given_len: context.param_given.len(),
        branch_unknowns: if context.branch_current_values.is_empty() {
            std::ptr::null()
        } else {
            context.branch_current_values.as_ptr()
        },
        analysis_type: context.analysis_type,
        multiplicity: context.multiplicity,
        zi_filters: if context.zi_filters.is_empty() {
            std::ptr::null_mut()
        } else {
            context.zi_filters.as_mut_ptr()
        },
        zi_filters_len: context.zi_filters.len(),
        transition_filters: if context.transition_filters.is_empty() {
            std::ptr::null_mut()
        } else {
            context.transition_filters.as_mut_ptr()
        },
        transition_filters_len: context.transition_filters.len(),
        slew_filters: if context.slew_filters.is_empty() {
            std::ptr::null_mut()
        } else {
            context.slew_filters.as_mut_ptr()
        },
        slew_filters_len: context.slew_filters.len(),
        delay_buffers: if context.delay_buffers.is_empty() {
            std::ptr::null_mut()
        } else {
            context.delay_buffers.as_mut_ptr()
        },
        delay_buffers_len: context.delay_buffers.len(),
        cross_detectors: if context.cross_detectors.is_empty() {
            std::ptr::null_mut()
        } else {
            context.cross_detectors.as_mut_ptr()
        },
        cross_detectors_len: context.cross_detectors.len(),
        state_prev_len: context.state_values_prev.len(),
        state_values_len: context.state_values.len(),
        timer_event_bound: &mut context.timer_event_bound,
        analysis_initial_step: u8::from(context.analysis_initial_step),
        analysis_final_step: u8::from(context.analysis_final_step),
        state_older: context.state_values_older.as_ptr(),
        state_older_len: context.state_values_older.len(),
        state_derivatives: context.state_derivatives.as_mut_ptr(),
        state_derivatives_len: context.state_derivatives.len(),
        state_derivatives_prev: context.state_derivatives_prev.as_ptr(),
        state_derivatives_prev_len: context.state_derivatives_prev.len(),
        integration_derivative_scale: integration.derivative_scale,
        integration_previous_value_scale: integration.previous_value_scale,
        integration_older_value_scale: integration.older_value_scale,
        integration_previous_derivative_scale: integration.previous_derivative_scale,
        integration_active: u8::from(integration.active),
        limiter_active: &mut context.limiter_active,
        limiting_enabled: u8::from(context.evaluation_mode.limiting_enabled()),
        runtime_status: Default::default(),
        state_candidate_valid: if context.state_candidate_valid.is_empty() {
            std::ptr::null_mut()
        } else {
            context.state_candidate_valid.as_mut_ptr()
        },
        state_candidate_valid_len: context.state_candidate_valid.len(),
        state_older_candidate: if context.state_older_candidate.is_empty() {
            std::ptr::null_mut()
        } else {
            context.state_older_candidate.as_mut_ptr()
        },
        state_older_candidate_len: context.state_older_candidate.len(),
    }
}

fn benchmark_failures(
    config: NativeBenchConfig,
    required_speedup: f64,
    native_setup_ns: f64,
    native_code_bytes: usize,
    native_stats: TimingStats,
    speedup_median: f64,
    native_checksum: f64,
    bytecode_checksum: f64,
) -> Vec<String> {
    let mut failures = Vec::new();
    if speedup_median < required_speedup {
        failures.push(format!(
            "median speedup {:.3}x is below required {:.3}x",
            speedup_median, required_speedup
        ));
    }
    if let Some(max_native_setup_ms) = config.max_native_setup_ms
        && native_setup_ns > max_native_setup_ms * 1_000_000.0
    {
        failures.push(format!(
            "native setup {:.3} ms exceeds {:.3} ms",
            native_setup_ns / 1_000_000.0,
            max_native_setup_ms
        ));
    }
    if let Some(max_native_code_bytes) = config.max_native_code_bytes
        && native_code_bytes > max_native_code_bytes
    {
        failures.push(format!(
            "native code size {native_code_bytes} bytes exceeds {max_native_code_bytes} bytes"
        ));
    }
    if let Some(max_native_p95) = config.max_native_p95_ns_per_sweep
        && native_stats.p95 > max_native_p95
    {
        failures.push(format!(
            "native p95 {:.3} ns/sweep exceeds {:.3} ns/sweep",
            native_stats.p95, max_native_p95
        ));
    }
    if let Some(max_relative_stddev) = config.max_relative_stddev
        && native_stats.relative_standard_deviation > max_relative_stddev
    {
        failures.push(format!(
            "native relative standard deviation {:.4} exceeds required {:.4}",
            native_stats.relative_standard_deviation, max_relative_stddev
        ));
    }
    if !native_checksum.is_finite() || !bytecode_checksum.is_finite() {
        failures.push("benchmark checksum became non-finite".into());
    }
    failures
}

fn timing_stats(mut samples: Vec<f64>) -> TimingStats {
    samples.sort_by(|left, right| left.total_cmp(right));
    let count = samples.len();
    let median = if count.is_multiple_of(2) {
        (samples[count / 2 - 1] + samples[count / 2]) * 0.5
    } else {
        samples[count / 2]
    };
    let p95_index = ((count as f64 * 0.95).ceil() as usize).saturating_sub(1);
    let mean = samples.iter().sum::<f64>() / count as f64;
    let variance = samples
        .iter()
        .map(|sample| {
            let delta = sample - mean;
            delta * delta
        })
        .sum::<f64>()
        / count as f64;
    let standard_deviation = variance.sqrt();
    TimingStats {
        min: samples[0],
        median,
        p95: samples[p95_index.min(count - 1)],
        mean,
        standard_deviation,
        relative_standard_deviation: standard_deviation / mean.max(f64::MIN_POSITIVE),
    }
}

fn assert_close(name: &str, expected: f64, actual: f64) -> Result<(), String> {
    if expected == actual {
        return Ok(());
    }
    let scale = expected.abs().max(actual.abs()).max(1.0);
    let tolerance = 1.0e-8 * scale;
    let delta = (expected - actual).abs();
    if delta <= tolerance {
        Ok(())
    } else {
        Err(format!(
            "{name}: expected {expected:.17e}, actual {actual:.17e}, delta {delta:.17e}, tolerance {tolerance:.17e}"
        ))
    }
}

fn take_native_error(ctx: &mut EvalContext, phase: &str) -> Result<(), String> {
    if let Some(error) = ctx.take_runtime_error() {
        Err(format!("native runtime error during {phase}: {error}"))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{DENSE_MODEL_SOURCE, NativeBenchConfig, benchmark_failures, timing_stats};
    #[cfg(any(
        target_arch = "x86_64",
        all(target_arch = "aarch64", target_os = "macos")
    ))]
    use super::{run_dense_entrypoint_case, run_device_evaluate_case, run_device_stamp_case};
    use crate::{CompilerOptions, VerilogACompiler};

    fn smoke_config() -> NativeBenchConfig {
        NativeBenchConfig {
            iterations: 1,
            samples: 1,
            min_dense_speedup: 0.0,
            min_speedup: 0.0,
            min_full_stamp_speedup: 0.0,
            max_native_setup_ms: None,
            max_native_p95_ns_per_sweep: None,
            max_relative_stddev: None,
            max_native_code_bytes: None,
        }
    }

    #[test]
    #[cfg(any(
        target_arch = "x86_64",
        all(target_arch = "aarch64", target_os = "macos")
    ))]
    fn dense_entrypoint_benchmark_smoke_runs_native_sweep() {
        let report =
            run_dense_entrypoint_case(smoke_config()).expect("dense native benchmark case runs");

        assert!(report.checksum_native.is_finite());
        assert!(report.checksum_bytecode.is_finite());
    }

    #[test]
    #[cfg(any(
        target_arch = "x86_64",
        all(target_arch = "aarch64", target_os = "macos")
    ))]
    fn device_evaluate_benchmark_smoke_runs_native_sweep() {
        let report =
            run_device_evaluate_case(smoke_config()).expect("device native benchmark case runs");

        assert!(report.checksum_native.is_finite());
        assert!(report.checksum_bytecode.is_finite());
    }

    #[test]
    #[cfg(any(
        target_arch = "x86_64",
        all(target_arch = "aarch64", target_os = "macos")
    ))]
    fn device_stamp_benchmark_smoke_matches_matrix_and_rhs_reference() {
        let report =
            run_device_stamp_case(smoke_config()).expect("device stamp native benchmark case runs");

        assert!(report.checksum_native.is_finite());
        assert!(report.checksum_bytecode.is_finite());
        assert!(report.native_code_bytes > 0);
    }

    #[test]
    fn dense_model_recompilation_preserves_runtime_variable_layout() {
        let expected = VerilogACompiler::new(CompilerOptions::default())
            .compile(DENSE_MODEL_SOURCE)
            .expect("compile dense layout reference")
            .variable_names;
        assert!(
            expected.iter().any(|name| name.contains('@')),
            "fixture must exercise generated derivative-shadow slots"
        );

        for iteration in 0..32 {
            let actual = VerilogACompiler::new(CompilerOptions::default())
                .compile(DENSE_MODEL_SOURCE)
                .unwrap_or_else(|error| {
                    panic!("compile dense layout iteration {iteration}: {error}")
                })
                .variable_names;
            assert_eq!(
                actual, expected,
                "identical source changed runtime variable-slot layout at iteration {iteration}"
            );
        }
    }

    #[test]
    fn timing_stats_report_population_standard_deviation() {
        let stats = timing_stats(vec![1.0, 2.0, 3.0]);

        assert_eq!(stats.mean, 2.0);
        assert!((stats.standard_deviation - (2.0_f64 / 3.0).sqrt()).abs() <= f64::EPSILON);
        assert!(
            (stats.relative_standard_deviation - stats.standard_deviation / 2.0).abs()
                <= f64::EPSILON
        );
    }

    #[test]
    fn benchmark_gate_rejects_native_setup_budget_regression() {
        let mut config = smoke_config();
        config.max_native_setup_ms = Some(1.0);
        let failures = benchmark_failures(
            config,
            0.0,
            2_000_000.0,
            1,
            timing_stats(vec![1.0]),
            1.0,
            0.0,
            0.0,
        );

        assert_eq!(
            failures,
            ["native setup 2.000 ms exceeds 1.000 ms".to_string()]
        );
    }

    #[test]
    fn benchmark_gate_rejects_native_code_size_regression() {
        let mut config = smoke_config();
        config.max_native_code_bytes = Some(8_000);
        let failures = benchmark_failures(
            config,
            0.0,
            1.0,
            8_001,
            timing_stats(vec![1.0]),
            1.0,
            0.0,
            0.0,
        );

        assert_eq!(
            failures,
            ["native code size 8001 bytes exceeds 8000 bytes".to_string()]
        );
    }
}
