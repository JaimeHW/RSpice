//! Browser JIT capabilities and the startup probes that qualify them.

static WASM_JIT_ARCHITECTURE_PROBE_FRAME: std::sync::Mutex<[f64; 2]> =
    std::sync::Mutex::new([0.0, 0.0]);

const WASM_JIT_ARCHITECTURE_PROBE_INPUT: f64 = 21.0;

pub fn rspice_ui_wasm_jit_probe_module() -> Result<Vec<u8>, String> {
    rspice_veriloga::wasm_jit::emit_architecture_probe()
        .map(|artifact| artifact.into_bytes())
        .map_err(|error| error.to_string())
}

pub const fn rspice_ui_wasm_jit_abi_version() -> u32 {
    rspice_veriloga::wasm_jit::WASM_JIT_ABI_VERSION
}

pub const fn rspice_ui_wasm_jit_emitter_version() -> u32 {
    rspice_veriloga::wasm_jit::WASM_JIT_EMITTER_VERSION
}

#[allow(clippy::too_many_arguments)]
pub fn rspice_ui_wasm_jit_eval_op_v1(
    frame_offset: u32,
    opcode: i32,
    aux0: i32,
    aux1: i32,
    aux2: i64,
    operand0: f64,
    operand1: f64,
    operand2: f64,
    operand3: f64,
    operand4: f64,
) -> f64 {
    rspice_veriloga::wasm_jit::eval_op_v1(
        frame_offset,
        opcode,
        aux0,
        aux1,
        aux2,
        operand0,
        operand1,
        operand2,
        operand3,
        operand4,
    )
}

pub fn rspice_ui_wasm_jit_eval_op_slice_v1(
    frame_offset: u32,
    opcode: i32,
    aux0: i32,
    aux1: i32,
    aux2: i64,
    operand_count: i32,
) -> f64 {
    rspice_veriloga::wasm_jit::eval_op_slice_v1(
        frame_offset,
        opcode,
        aux0,
        aux1,
        aux2,
        operand_count,
    )
}

/// Frame-free unary transcendental capability for generated modules.
///
/// The browser binds this to the secondary module's `math1_v1` import as a raw
/// WebAssembly export, so the call is wasm-to-wasm with no JavaScript frame
/// between a model's `exp()` and its implementation.
pub fn rspice_ui_wasm_jit_math1_v1(opcode: i32, value: f64) -> f64 {
    rspice_veriloga::wasm_jit::math1_v1(opcode, value)
}

/// Frame-free binary transcendental capability. See
/// [`rspice_ui_wasm_jit_math1_v1`].
pub fn rspice_ui_wasm_jit_math2_v1(opcode: i32, left: f64, right: f64) -> f64 {
    rspice_veriloga::wasm_jit::math2_v1(opcode, left, right)
}

pub fn prepare_rspice_ui_wasm_jit_probe() -> Result<u32, String> {
    let mut frame = WASM_JIT_ARCHITECTURE_PROBE_FRAME
        .lock()
        .map_err(|_| "WASM JIT architecture-probe frame lock is poisoned".to_owned())?;
    *frame = [WASM_JIT_ARCHITECTURE_PROBE_INPUT, 0.0];
    let offset = frame.as_ptr() as usize;
    u32::try_from(offset)
        .map_err(|_| "WASM JIT architecture-probe frame is outside wasm32 memory".to_owned())
}

pub fn finish_rspice_ui_wasm_jit_probe(frame_offset: u32, status: i32) -> Result<f64, String> {
    if status != 0 {
        return Err(format!(
            "WASM JIT architecture probe returned status {status}"
        ));
    }
    let frame = WASM_JIT_ARCHITECTURE_PROBE_FRAME
        .lock()
        .map_err(|_| "WASM JIT architecture-probe frame lock is poisoned".to_owned())?;
    let expected_offset = u32::try_from(frame.as_ptr() as usize)
        .map_err(|_| "WASM JIT architecture-probe frame is outside wasm32 memory".to_owned())?;
    if frame_offset != expected_offset {
        return Err("WASM JIT architecture probe used an unowned memory frame".to_owned());
    }
    let expected = WASM_JIT_ARCHITECTURE_PROBE_INPUT * 2.0;
    if frame[1].to_bits() != expected.to_bits() {
        return Err(format!(
            "WASM JIT architecture probe produced {}, expected {expected}",
            frame[1]
        ));
    }
    Ok(frame[1])
}

const WASM_JIT_SOLVER_PROBE_SOURCE: &str = r#"
`include "disciplines.vams"
module rspice_wasm_solver_probe(p, n);
  inout p, n;
  electrical p, n;
  parameter real gain = 2.0;
  real bias;
  real declaration_seed = 1.0;
  real initialized_gain;
  integer task_index;
  analog initial initialized_gain = initialized_gain + declaration_seed * gain;
  analog begin
    bias = analysis("tran") ? ($param_given(gain) ? 100.0 : 1.0) : -1000.0;
    task_index = 0;
    while (task_index < 2) begin
      $finish(task_index);
      task_index = task_index + 1;
    end
    if (V(p, n) < 0.0) $finish(99);
    I(p, n) <+ bias + initialized_gain * V(p, n) + ddt(V(p, n));
  end
endmodule
"#;

/// A model reaching the parts of the browser backend the solver probe leaves
/// untouched: several contributions published by one fused dispatch, a square
/// root and an exponential through the frame-free math capability, an inlined
/// extremum, and a contribution carrying several Jacobian entries.
///
/// Every expected result is exact in binary floating point, so the browser is
/// compared bit for bit against the machine backends rather than within a
/// tolerance.
const WASM_JIT_KERNEL_PROBE_SOURCE: &str = r#"
`include "disciplines.vams"
module rspice_wasm_kernel_probe(a, b, c);
  inout a, b, c;
  electrical a, b, c;
  parameter real scale = 2.0;
  real shaped;
  analog begin
    shaped = sqrt(V(a, b)) + exp(0.0);
    I(a, b) <+ shaped * scale;
    I(c, b) <+ max(V(c, b), 3.0) * scale;
    I(a, c) <+ V(a, b) * V(c, b);
  end
endmodule
"#;

/// Solver-node voltages, ground excluded: `V(a, b)` is 4 so its square root is
/// exact, and `V(c, b)` is 5 so the extremum selects its variable arm and
/// carries a derivative.
const WASM_JIT_KERNEL_PROBE_VOLTAGES: [f64; 2] = [4.0, 5.0];

const WASM_JIT_KERNEL_PROBE_CURRENTS: [f64; 3] = [6.0, 10.0, 20.0];

const WASM_JIT_KERNEL_PROBE_JACOBIAN: [f64; 14] = [
    0.5, -0.5, -0.5, 0.5, -2.0, 2.0, 2.0, -2.0, 5.0, -5.0, -9.0, 9.0, 4.0, -4.0,
];

/// Enough stamps that the browser's millisecond clock resolves the per-stamp
/// cost, without making the gate a long-running job.
const WASM_JIT_KERNEL_PROBE_STAMPS: u32 = 20_000;

fn compile_wasm_jit_kernel_probe() -> Result<rspice_veriloga::RuntimeCompileReport, String> {
    rspice_veriloga::VerilogACompiler::new(rspice_veriloga::CompilerOptions::default())
        .compile_runtime(
            WASM_JIT_KERNEL_PROBE_SOURCE,
            Some("rspice_wasm_kernel_probe"),
        )
        .map_err(|error| format!("WASM JIT kernel probe compilation failed: {error}"))
}

/// Emit the fused-driver probe's model module for the worker to install.
pub fn rspice_ui_wasm_jit_kernel_probe_artifact()
-> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    let report =
        compile_wasm_jit_kernel_probe().map_err(|error| wasm_bindgen::JsValue::from_str(&error))?;
    let artifact =
        rspice_veriloga::wasm_jit::compile_model_value_module(&report.model, &report.canonical_ir)
            .map_err(|error| wasm_bindgen::JsValue::from_str(&error.to_string()))?;
    let artifact =
        rspice_simulation::project_veriloga::worker::WasmJitWorkerArtifact::from_compiled(
            &artifact,
        );
    serde_wasm_bindgen::to_value(&artifact)
        .map_err(|error| wasm_bindgen::JsValue::from_str(&error.to_string()))
}

/// What the browser engine produced, and how long a fused stamp costs there.
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WasmJitKernelProbeReport {
    pub contributions: usize,
    pub jacobian_entries: usize,
    pub stamps: u32,
    pub elapsed_ms: f64,
    pub nanoseconds_per_stamp: f64,
}

fn expect_exact_probe_values(what: &str, actual: &[f64], expected: &[f64]) -> Result<(), String> {
    if actual.len() != expected.len()
        || actual
            .iter()
            .zip(expected)
            .any(|(actual, expected)| actual.to_bits() != expected.to_bits())
    {
        return Err(format!(
            "WASM JIT kernel probe {what} mismatch: {actual:?}, expected {expected:?}"
        ));
    }
    Ok(())
}

/// Evaluate and stamp a multi-contribution model in the browser engine,
/// checking every result bit for bit and timing the fused stamp driver.
///
/// The three checks reach three different paths: the contributions come from
/// the fused evaluation driver, the derivatives from the per-entry Jacobian
/// exports, and the timing from the fused stamp driver. The timing is the only
/// evidence that catches a capability bound to a JavaScript wrapper rather
/// than a raw export, because such a build still computes the right answer.
pub fn rspice_ui_wasm_jit_run_kernel_probe() -> Result<wasm_bindgen::JsValue, String> {
    let report = compile_wasm_jit_kernel_probe()?;
    let mut device = rspice_veriloga::device::VerilogADevice::try_new_with_canonical_ir(
        "WASMJITKERNEL1",
        std::sync::Arc::new(report.model),
        &report.canonical_ir,
        &[1, 0, 2],
    )
    .map_err(|error| error.to_string())?;
    if !device.fused_stamp_driver_is_active() {
        return Err(
            "WASM JIT kernel probe model did not qualify for the fused stamp driver".to_owned(),
        );
    }
    device
        .try_update_all_voltages(&WASM_JIT_KERNEL_PROBE_VOLTAGES)
        .map_err(|error| error.to_string())?;

    let currents = device.try_evaluate().map_err(|error| error.to_string())?;
    expect_exact_probe_values("contribution", &currents, &WASM_JIT_KERNEL_PROBE_CURRENTS)?;
    let jacobian = device
        .try_compute_jacobian()
        .map_err(|error| error.to_string())?;
    let entries = jacobian
        .iter()
        .map(|entry| entry.value)
        .collect::<Vec<f64>>();
    expect_exact_probe_values("Jacobian", &entries, &WASM_JIT_KERNEL_PROBE_JACOBIAN)?;

    // Nudging one voltage per pass keeps the device from being handed the same
    // operating point twice, by less than the extremum's margin so the arm the
    // checked values came from stays selected.
    let started = web_time::Instant::now();
    for step in 0..WASM_JIT_KERNEL_PROBE_STAMPS {
        let voltages = [
            WASM_JIT_KERNEL_PROBE_VOLTAGES[0],
            WASM_JIT_KERNEL_PROBE_VOLTAGES[1] + f64::from(step) * f64::EPSILON,
        ];
        device
            .try_stamp(&voltages, |_, _, _| {}, |_, _| {})
            .map_err(|error| error.to_string())?;
    }
    let elapsed_ms = started.elapsed().as_secs_f64() * 1000.0;

    let report = WasmJitKernelProbeReport {
        contributions: currents.len(),
        jacobian_entries: entries.len(),
        stamps: WASM_JIT_KERNEL_PROBE_STAMPS,
        elapsed_ms,
        nanoseconds_per_stamp: elapsed_ms * 1.0e6 / f64::from(WASM_JIT_KERNEL_PROBE_STAMPS),
    };
    serde_wasm_bindgen::to_value(&report).map_err(|error| error.to_string())
}

fn compile_wasm_jit_solver_probe() -> Result<rspice_veriloga::RuntimeCompileReport, String> {
    rspice_veriloga::VerilogACompiler::new(rspice_veriloga::CompilerOptions::default())
        .compile_runtime(
            WASM_JIT_SOLVER_PROBE_SOURCE,
            Some("rspice_wasm_solver_probe"),
        )
        .map_err(|error| format!("WASM JIT solver probe compilation failed: {error}"))
}

/// Emit a real canonical model used to qualify the installed-module solver
/// bridge in the browser engine, not merely WebAssembly compilation support.
pub fn rspice_ui_wasm_jit_solver_probe_artifact()
-> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    let report =
        compile_wasm_jit_solver_probe().map_err(|error| wasm_bindgen::JsValue::from_str(&error))?;
    let artifact =
        rspice_veriloga::wasm_jit::compile_model_value_module(&report.model, &report.canonical_ir)
            .map_err(|error| wasm_bindgen::JsValue::from_str(&error.to_string()))?;
    let artifact =
        rspice_simulation::project_veriloga::worker::WasmJitWorkerArtifact::from_compiled(
            &artifact,
        );
    serde_wasm_bindgen::to_value(&artifact)
        .map_err(|error| wasm_bindgen::JsValue::from_str(&error.to_string()))
}

/// Exercise parameter, analysis, assignment, stateful transient, value,
/// Jacobian, matrix, and RHS dispatch through an installed secondary module.
pub fn rspice_ui_wasm_jit_run_solver_probe() -> Result<f64, String> {
    use rspice_core::device::veriloga_builtins::{AnalogTaskArgument, AnalogTaskKind};

    fn accept_tasks(
        device: &mut rspice_veriloga::device::VerilogADevice,
        time: f64,
    ) -> Result<(), String> {
        if device.drain_accepted_analog_tasks().next().is_some() {
            return Err("WASM JIT solver probe published an unaccepted task".into());
        }
        device
            .try_advance_state()
            .map_err(|error| error.to_string())?;
        let calls = device.drain_accepted_analog_tasks().collect::<Vec<_>>();
        if calls.len() != 2
            || calls.iter().enumerate().any(|(index, call)| {
                call.kind != AnalogTaskKind::Finish
                    || call.time != time
                    || call.site != calls[0].site
                    || call.arguments.as_ref() != [AnalogTaskArgument::Integer(index as i64)]
            })
        {
            return Err(format!(
                "WASM JIT solver probe task delivery mismatch: {calls:?}"
            ));
        }
        Ok(())
    }

    let report = compile_wasm_jit_solver_probe()?;
    let mut device = rspice_veriloga::device::VerilogADevice::try_new_with_canonical_ir(
        "WASMJITPROBE1",
        std::sync::Arc::new(report.model),
        &report.canonical_ir,
        &[1, 0],
    )
    .map_err(|error| error.to_string())?;
    device
        .try_set_analysis_type(2)
        .map_err(|error| error.to_string())?;
    device
        .try_set_timestep(0.5)
        .map_err(|error| error.to_string())?;
    device
        .try_update_all_voltages(&[3.0])
        .map_err(|error| error.to_string())?;

    let initial_currents = device.try_evaluate().map_err(|error| error.to_string())?;
    if initial_currents.len() != 1 || initial_currents[0].to_bits() != 7.0_f64.to_bits() {
        return Err(format!(
            "WASM JIT solver probe initial current mismatch: {initial_currents:?}, expected [7.0]"
        ));
    }
    accept_tasks(&mut device, 0.0)?;
    device
        .try_set_time(0.5)
        .map_err(|error| error.to_string())?;
    device
        .try_update_all_voltages(&[5.0])
        .map_err(|error| error.to_string())?;

    let currents = device.try_evaluate().map_err(|error| error.to_string())?;
    if currents.len() != 1 || currents[0].to_bits() != 15.0_f64.to_bits() {
        return Err(format!(
            "WASM JIT solver probe committed-state current mismatch: {currents:?}, expected [15.0]"
        ));
    }
    let jacobian = device
        .try_compute_jacobian()
        .map_err(|error| error.to_string())?;
    let expected_jacobian = [4.0_f64, -4.0, -4.0, 4.0];
    if jacobian.len() != expected_jacobian.len()
        || jacobian
            .iter()
            .zip(expected_jacobian)
            .any(|(entry, expected)| entry.value.to_bits() != expected.to_bits())
    {
        return Err(format!(
            "WASM JIT solver probe Jacobian mismatch: {jacobian:?}, expected values {expected_jacobian:?}"
        ));
    }

    let mut matrix = Vec::new();
    let mut rhs = Vec::new();
    device
        .try_stamp(
            &[5.0],
            |row, col, value| matrix.push((row, col, value)),
            |row, value| rhs.push((row, value)),
        )
        .map_err(|error| error.to_string())?;
    if !matrix
        .iter()
        .any(|&(row, col, value)| row == 0 && col == 0 && value.to_bits() == 4.0_f64.to_bits())
        || !rhs
            .iter()
            .any(|&(row, value)| row == 0 && value.to_bits() == 5.0_f64.to_bits())
    {
        return Err(format!(
            "WASM JIT solver probe stamp mismatch: matrix={matrix:?}, rhs={rhs:?}"
        ));
    }
    accept_tasks(&mut device, 0.5)?;
    Ok(currents[0])
}

pub fn install_rspice_ui_wasm_jit_dispatcher(dispatcher: js_sys::Function) {
    rspice_veriloga::wasm_jit::install_browser_dispatcher(
        move |cache_key, export_name, frame_offset| {
            let value = dispatcher
                .call3(
                    &wasm_bindgen::JsValue::UNDEFINED,
                    &wasm_bindgen::JsValue::from_str(cache_key),
                    &wasm_bindgen::JsValue::from_str(export_name),
                    &wasm_bindgen::JsValue::from_f64(f64::from(frame_offset)),
                )
                .map_err(|error| format!("browser WASM JIT dispatch threw: {error:?}"))?;
            let status = value
                .as_f64()
                .filter(|value| {
                    value.fract() == 0.0
                        && *value >= f64::from(i32::MIN)
                        && *value <= f64::from(i32::MAX)
                })
                .ok_or_else(|| "browser WASM JIT dispatch returned a non-i32 status".to_owned())?;
            Ok(status as i32)
        },
    );
}
