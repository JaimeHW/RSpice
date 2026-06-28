# Native JIT X64 Model Coverage Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Expand the first native x64 Verilog-A JIT slice from simple external-node scalar models to representative device-level coverage for internal-node voltages, scalar simulator context reads, parameter/port flags, and branch-source model images while preserving the full-JIT-or-error contract.

**Architecture:** This is Plan 3 of the native JIT replacement. It keeps the temporary bytecode-to-native expression adapter isolated, broadens only the straight-line x64 subset, and adds contract tests at the `VerilogADevice` boundary so native executable entry offsets, runtime context pointers, and MNA stamping behavior are exercised together. Unsupported bytecode remains a native JIT hard failure with no interpreter fallback.

**Tech Stack:** Rust 2024, `rspice-veriloga`, x86-64 Windows and System V C ABI, SSE2 scalar `f64`, Verilog-A compiled model artifacts, Superpowers TDD and subagent review gates.

---

## Scope Check

This plan does not implement optimizer passes, canonical OptIR scheduling, external helper calls, current-probe helper calls, reactive/stateful operators, AArch64, or performance benchmarking. It expands the currently committed native x64 executable-image subset in small correctness-first steps:

- Lower unified internal-node `PushVoltage` operands into native internal-voltage loads.
- Load scalar simulator context fields used by straight-line expressions: `$temperature`, `$abstime`/`$realtime`, and `$mfactor`.
- Load boolean context flags used as real-valued expressions: `$param_given(name)` and `$port_connected(port)`.
- Lock multi-stamp and potential-branch behavior at the device contract layer.
- Keep loops, indexed arrays, noise, reactive Jacobians, static conditions, `PushCurrent`, and other helper/stateful bytecode as fail-closed native JIT errors.

The branch currently has unrelated unstaged formatter churn in:

- `crates/rspice-core/src/device/veriloga_generated/support.rs`
- `crates/rspice-core/tests/generated_veriloga_runtime.rs`

Do not stage, edit, or revert those files in this plan.

## File Structure

- `crates/rspice-veriloga/src/native/expr.rs`: owns the temporary native expression IR and bytecode adapter. Add supported native ops and validation for internal-node voltage indices.
- `crates/rspice-veriloga/src/native/x64/codegen.rs`: emits x64 machine code for new expression ops and context loads.
- `crates/rspice-veriloga/src/native/x64/encoder.rs`: adds the minimal integer-to-double encoding helpers needed for boolean flag loads.
- `crates/rspice-veriloga/src/native/x64/mod.rs`: passes model terminal/internal-node counts into native expression lowering.
- `crates/rspice-veriloga/tests/native_contract.rs`: adds device-level x64 native tests for internal nodes, simulator context, flags, multi-stamp images, branch-source images, and hard-fail unsupported coverage.

## Task 1: Unified Internal-Node Voltage Loads

**Files:**
- Modify: `crates/rspice-veriloga/src/native/expr.rs`
- Modify: `crates/rspice-veriloga/src/native/x64/codegen.rs`
- Modify: `crates/rspice-veriloga/src/native/x64/mod.rs`
- Modify: `crates/rspice-veriloga/tests/native_contract.rs`

- [ ] **Step 1: Write the failing device contract test**

Add this fixture to `crates/rspice-veriloga/tests/native_contract.rs` near the other model fixture helpers:

```rust
fn internal_node_divider_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_divider(p, n);
    inout p, n;
    electrical p, n;
    electrical mid;
    parameter real r = 1.0 from (0:inf);
    analog begin
        I(p, mid) <+ (V(p) - V(mid)) / r;
        I(mid, n) <+ V(mid, n) / r;
    end
endmodule
"#,
    )
}
```

Add these tests after `native_device_executes_scalar_assignments_in_source_order`:

```rust
#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_evaluates_internal_node_voltage_contributions() {
    let model = internal_node_divider_model();
    assert_eq!(model.internal_nodes, 1);
    assert_eq!(model.stamp_programs.len(), 2);

    let mut device =
        VerilogADevice::try_new("DIV1", model, &[1, 0]).expect("divider uses native JIT");
    assert!(device.is_using_native());
    device.set_internal_node_indices(&[2]);
    device.update_all_voltages(&[2.0, 0.5]);

    let currents = device
        .try_evaluate()
        .expect("native internal-node evaluation succeeds");

    assert!((currents[0] - 1.5).abs() < 1e-12, "currents: {currents:?}");
    assert!((currents[1] - 0.5).abs() < 1e-12, "currents: {currents:?}");
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_stamps_internal_node_jacobians() {
    let model = internal_node_divider_model();
    let mut device =
        VerilogADevice::try_new("DIV2", model, &[1, 0]).expect("divider uses native JIT");
    device.set_internal_node_indices(&[2]);

    let (matrix, rhs) = stamp_device(&mut device, &[2.0, 0.5]);

    assert!((matrix.get(&(0, 0)).copied().unwrap_or_default() - 1.0).abs() < 1e-12);
    assert!((matrix.get(&(0, 1)).copied().unwrap_or_default() + 1.0).abs() < 1e-12);
    assert!((matrix.get(&(1, 0)).copied().unwrap_or_default() + 1.0).abs() < 1e-12);
    assert!((matrix.get(&(1, 1)).copied().unwrap_or_default() - 2.0).abs() < 1e-12);
    assert!(
        rhs.values().map(|value| value.abs()).sum::<f64>() < 1e-12,
        "rhs: {rhs:?}"
    );
}
```

- [ ] **Step 2: Run the red test**

Run:

```powershell
& "$env:USERPROFILE\.cargo\bin\cargo.exe" test -p rspice-veriloga --features native --test native_contract native_device_evaluates_internal_node_voltage_contributions -- --nocapture
```

Expected: FAIL before implementation. The failure must be a native JIT unsupported `PushVoltage unified node` diagnostic, not a test compile error.

- [ ] **Step 3: Add internal nodes to the native expression IR**

In `crates/rspice-veriloga/src/native/expr.rs`, change `VoltageNode` and `NativeProgram::from_bytecode` like this:

```rust
pub(crate) enum VoltageNode {
    Terminal(usize),
    Internal(usize),
    Ground,
}

pub(crate) fn from_bytecode(
    model: impl Into<SmolStr>,
    entry_kind: EntryKind,
    program: &BytecodeProgram,
    terminal_count: usize,
    internal_node_count: usize,
) -> JitResult<Self> {
    // existing body, but PushVoltage calls lower_voltage_node with both counts
}
```

Replace `lower_voltage_node` with:

```rust
fn lower_voltage_node(
    model: SmolStr,
    node: usize,
    terminal_count: usize,
    internal_node_count: usize,
) -> JitResult<VoltageNode> {
    if node == usize::MAX {
        return Ok(VoltageNode::Ground);
    }

    if node < terminal_count {
        return Ok(VoltageNode::Terminal(node));
    }

    let internal_index = node - terminal_count;
    if internal_index < internal_node_count {
        return Ok(VoltageNode::Internal(internal_index));
    }

    Err(JitError::unsupported_program_op(
        model,
        format!("PushVoltage unified node {node}"),
    ))
}
```

Update existing `NativeProgram::from_bytecode` call sites to pass `model.internal_nodes` in production and `0` or the needed count in tests.

- [ ] **Step 4: Emit internal-voltage x64 loads**

In `crates/rspice-veriloga/src/native/x64/codegen.rs`, update `emit_voltage_load` so every combination of `Terminal`, `Internal`, and `Ground` is handled. Add this helper:

```rust
fn emit_internal_voltage_load(&mut self, dst: Xmm, index: usize) -> JitResult<()> {
    self.emit_context_pointer_load(INTERNAL_VOLTAGES_OFFSET);
    self.encoder
        .movsd_xmm_m64_base_disp32(dst, Gpr::Rax, byte_disp(index)?);
    Ok(())
}
```

For mixed non-ground voltages, load the positive node into `dst`, load the negative node into `scratch`, then subtract `scratch` from `dst`.

- [ ] **Step 5: Update native expression unit tests**

Replace the old rejection test for unified internal voltage in `expr.rs` with:

```rust
#[test]
fn lowers_unified_internal_voltage_index_when_internal_count_is_known() {
    let program = BytecodeProgram {
        instructions: vec![Instruction::PushVoltage(1, 2)],
    };

    let lowered = NativeProgram::from_bytecode("int", EntryKind::StampValue, &program, 2, 1)
        .expect("lower terminal-to-internal voltage");

    assert_eq!(
        lowered.ops(),
        &[NativeOp::LoadVoltage {
            pos: VoltageNode::Terminal(1),
            neg: VoltageNode::Internal(0),
        }]
    );
}

#[test]
fn lowering_rejects_unified_voltage_index_outside_known_nodes() {
    let program = BytecodeProgram {
        instructions: vec![Instruction::PushVoltage(3, usize::MAX)],
    };

    let error = NativeProgram::from_bytecode("bad", EntryKind::StampValue, &program, 2, 1)
        .expect_err("node outside terminals plus internals must fail closed");
    let msg = error.to_string();
    assert!(msg.contains("PushVoltage unified node 3"), "got: {msg}");
    assert!(msg.contains("no interpreter fallback"), "got: {msg}");
}
```

- [ ] **Step 6: Run the green tests**

Run:

```powershell
& "$env:USERPROFILE\.cargo\bin\cargo.exe" test -p rspice-veriloga --features native --test native_contract native_device_evaluates_internal_node_voltage_contributions native_device_stamps_internal_node_jacobians -- --nocapture
& "$env:USERPROFILE\.cargo\bin\cargo.exe" test -p rspice-veriloga --features native native::expr::tests native::x64::codegen::tests -- --nocapture
```

Expected: all selected tests pass.

- [ ] **Step 7: Commit**

```powershell
git add crates/rspice-veriloga/src/native/expr.rs crates/rspice-veriloga/src/native/x64/codegen.rs crates/rspice-veriloga/src/native/x64/mod.rs crates/rspice-veriloga/tests/native_contract.rs
git commit -m "feat(native): support x64 internal voltage loads"
```

## Task 2: Scalar Simulator Context Loads

**Files:**
- Modify: `crates/rspice-veriloga/src/native/expr.rs`
- Modify: `crates/rspice-veriloga/src/native/x64/codegen.rs`
- Modify: `crates/rspice-veriloga/tests/native_contract.rs`

- [ ] **Step 1: Write the failing scalar-context test**

Add this fixture to `native_contract.rs`:

```rust
fn scalar_context_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_context_scalar(p, n);
    inout p, n;
    electrical p, n;
    real gain;
    analog begin
        gain = (($temperature - 300.0) * 1.0e-3) + $abstime + $mfactor;
        I(p, n) <+ gain * V(p, n);
    end
endmodule
"#,
    )
}
```

Add this test:

```rust
#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_scalar_simulator_context_reads() {
    let model = scalar_context_model();
    let mut device =
        VerilogADevice::try_new("CTX1", model, &[1, 0]).expect("context scalar model uses native JIT");
    device.set_temperature(310.0);
    device.set_time(2.0);
    device.set_multiplicity(3.0);
    device.update_voltages(&[4.0]);

    let currents = device
        .try_evaluate()
        .expect("native context scalar evaluation succeeds");

    assert!((currents[0] - 20.04).abs() < 1e-12, "currents: {currents:?}");
    assert!((device.variable("gain").unwrap() - 5.01).abs() < 1e-12);
}
```

- [ ] **Step 2: Run the red test**

Run:

```powershell
& "$env:USERPROFILE\.cargo\bin\cargo.exe" test -p rspice-veriloga --features native --test native_contract native_device_executes_scalar_simulator_context_reads -- --nocapture
```

Expected: FAIL before implementation with unsupported `PushTemperature`, `PushTime`, or `PushMfactor`.

- [ ] **Step 3: Add scalar context native ops**

In `expr.rs`, add:

```rust
LoadTemperature,
LoadTime,
LoadMfactor,
```

to `NativeOp`, and map:

```rust
Instruction::PushTemperature => {
    ops.push(NativeOp::LoadTemperature);
    push_stack(&mut depth, &mut max_stack_depth);
}
Instruction::PushTime => {
    ops.push(NativeOp::LoadTime);
    push_stack(&mut depth, &mut max_stack_depth);
}
Instruction::PushMfactor => {
    ops.push(NativeOp::LoadMfactor);
    push_stack(&mut depth, &mut max_stack_depth);
}
```

- [ ] **Step 4: Emit direct context-field loads**

In `codegen.rs`, add:

```rust
const TEMPERATURE_OFFSET: i32 = 80;
const TIME_OFFSET: i32 = 88;
const MFACTOR_OFFSET: i32 = 176;
```

Add:

```rust
fn emit_context_f64_load(&mut self, ctx_field_offset: i32) -> JitResult<()> {
    let dst = self.push_register()?;
    self.encoder
        .movsd_xmm_m64_base_disp32(dst, host_ctx_arg_reg(), ctx_field_offset);
    Ok(())
}
```

Handle the new ops with `emit_context_f64_load`.

- [ ] **Step 5: Run the green tests**

Run:

```powershell
& "$env:USERPROFILE\.cargo\bin\cargo.exe" test -p rspice-veriloga --features native --test native_contract native_device_executes_scalar_simulator_context_reads -- --nocapture
& "$env:USERPROFILE\.cargo\bin\cargo.exe" test -p rspice-veriloga --features native native::x64::codegen::tests -- --nocapture
```

Expected: all selected tests pass.

- [ ] **Step 6: Commit**

```powershell
git add crates/rspice-veriloga/src/native/expr.rs crates/rspice-veriloga/src/native/x64/codegen.rs crates/rspice-veriloga/tests/native_contract.rs
git commit -m "feat(native): load scalar simulator context in x64"
```

## Task 3: Parameter-Given And Port-Connected Flag Loads

**Files:**
- Modify: `crates/rspice-veriloga/src/native/expr.rs`
- Modify: `crates/rspice-veriloga/src/native/x64/encoder.rs`
- Modify: `crates/rspice-veriloga/src/native/x64/codegen.rs`
- Modify: `crates/rspice-veriloga/tests/native_contract.rs`

- [ ] **Step 1: Write the failing flag-load test**

Add this fixture to `native_contract.rs`:

```rust
fn flag_context_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_context_flags(p, n, opt);
    inout p, n, opt;
    electrical p, n, opt;
    parameter real rknob = 2.0 from (0:inf);
    real gain;
    analog begin
        gain = ($param_given(rknob) + $port_connected(opt)) * 0.5;
        I(p, n) <+ gain * V(p, n);
    end
endmodule
"#,
    )
}
```

Add this test:

```rust
#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_executes_param_given_and_port_connected_reads() {
    let model = flag_context_model();

    let mut omitted =
        VerilogADevice::try_new("FLG1", model.clone(), &[1, 0]).expect("flag model uses native JIT");
    omitted.update_voltages(&[2.0]);
    assert_eq!(omitted.try_evaluate().unwrap()[0], 0.0);
    assert_eq!(omitted.variable("gain"), Some(0.0));

    let mut connected =
        VerilogADevice::try_new("FLG2", model, &[1, 0, 0]).expect("flag model uses native JIT");
    assert!(connected.set_parameter("rknob", 2.0));
    connected.update_voltages(&[2.0]);

    let currents = connected.try_evaluate().unwrap();
    assert!((currents[0] - 2.0).abs() < 1e-12, "currents: {currents:?}");
    assert_eq!(connected.variable("gain"), Some(1.0));
}
```

- [ ] **Step 2: Run the red test**

Run:

```powershell
& "$env:USERPROFILE\.cargo\bin\cargo.exe" test -p rspice-veriloga --features native --test native_contract native_device_executes_param_given_and_port_connected_reads -- --nocapture
```

Expected: FAIL before implementation with unsupported `PushParamGiven` or `PushPortConnected`.

- [ ] **Step 3: Add flag native ops**

In `expr.rs`, add:

```rust
LoadParamGiven(usize),
LoadPortConnected(usize),
```

Map `Instruction::PushParamGiven(index)` and `Instruction::PushPortConnected(index)` to those ops with normal stack pushes.

- [ ] **Step 4: Add x64 flag-load encoding helpers**

In `encoder.rs`, add:

```rust
pub(crate) fn movzx_r32_m8_base_disp32(&mut self, dst: Gpr, base: Gpr, disp: i32) {
    self.emit_rex(false, dst.code(), 0, base.code());
    self.emit_all(&[0x0F, 0xB6]);
    self.emit_base_disp32_modrm(dst.code(), base.code(), disp);
}

pub(crate) fn cvtsi2sd_xmm_r32(&mut self, dst: Xmm, src: Gpr) {
    self.emit_u8(0xF2);
    self.emit_rex(false, dst.code(), 0, src.code());
    self.emit_all(&[0x0F, 0x2A]);
    self.emit_modrm(0b11, dst.code(), src.code());
}
```

Add an encoder unit test that verifies `movzx_r32_m8_base_disp32(Gpr::R10, Gpr::Rax, 8)` and `cvtsi2sd_xmm_r32(Xmm::Xmm0, Gpr::R10)` encode to:

```rust
[
    0x44, 0x0F, 0xB6, 0x90, 8, 0, 0, 0,
    0xF2, 0x41, 0x0F, 0x2A, 0xC2,
]
```

- [ ] **Step 5: Emit flag loads as real-valued 0.0 or 1.0**

In `codegen.rs`, add:

```rust
const PORT_CONNECTED_OFFSET: i32 = 64;
const PARAM_GIVEN_OFFSET: i32 = 152;
```

Add:

```rust
fn emit_context_u8_flag_load(&mut self, ctx_pointer_field_offset: i32, index: usize) -> JitResult<()> {
    let dst = self.push_register()?;
    self.emit_context_pointer_load(ctx_pointer_field_offset);
    self.encoder
        .movzx_r32_m8_base_disp32(Gpr::R10, Gpr::Rax, byte_disp_u8(index)?);
    self.encoder.cvtsi2sd_xmm_r32(dst, Gpr::R10);
    Ok(())
}

fn byte_disp_u8(index: usize) -> JitResult<i32> {
    i32::try_from(index).map_err(|_| JitError::Encoding {
        model: MODEL.into(),
        detail: format!("u8 flag index {index} exceeds x64 disp32 range").into(),
    })
}
```

Handle `LoadParamGiven(index)` with `PARAM_GIVEN_OFFSET` and `LoadPortConnected(index)` with `PORT_CONNECTED_OFFSET`.

- [ ] **Step 6: Run the green tests**

Run:

```powershell
& "$env:USERPROFILE\.cargo\bin\cargo.exe" test -p rspice-veriloga --features native --test native_contract native_device_executes_param_given_and_port_connected_reads -- --nocapture
& "$env:USERPROFILE\.cargo\bin\cargo.exe" test -p rspice-veriloga --features native native::x64::encoder::tests native::x64::codegen::tests -- --nocapture
```

Expected: all selected tests pass.

- [ ] **Step 7: Commit**

```powershell
git add crates/rspice-veriloga/src/native/expr.rs crates/rspice-veriloga/src/native/x64/encoder.rs crates/rspice-veriloga/src/native/x64/codegen.rs crates/rspice-veriloga/tests/native_contract.rs
git commit -m "feat(native): load native context flags in x64"
```

## Task 4: Multi-Entry Image And Branch-Source Contracts

**Files:**
- Modify: `crates/rspice-veriloga/tests/native_contract.rs`

- [ ] **Step 1: Add multi-stamp and branch-source fixtures**

Add:

```rust
fn multi_stamp_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_multi_stamp(p, n);
    inout p, n;
    electrical p, n;
    parameter real g1 = 0.25;
    parameter real g2 = 0.75;
    analog begin
        I(p, n) <+ g1 * V(p, n);
        I(p, n) <+ g2 * V(p, n);
    end
endmodule
"#,
    )
}

fn potential_branch_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_zres(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 2000.0 from (0:inf);
    analog V(p, n) <+ I(p, n) * r;
endmodule
"#,
    )
}
```

- [ ] **Step 2: Add multi-entry image tests**

Add:

```rust
#[cfg(target_arch = "x86_64")]
#[test]
fn native_model_image_publishes_multiple_stamp_and_jacobian_entries() {
    let model = multi_stamp_model();
    assert_eq!(model.stamp_programs.len(), 2);

    let native = compile_native(&model).expect("multi-stamp model compiles native");

    assert_eq!(native.native_stamp_count(), 2);
    assert_eq!(native.plan_stats().stamp_value_entry_points, 2);
    assert_eq!(
        native.plan_stats().jacobian_entry_points,
        model
            .stamp_programs
            .iter()
            .map(|stamp| stamp.jacobian_programs.len())
            .sum::<usize>()
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_stamps_multiple_flow_contributions_from_one_image() {
    let model = multi_stamp_model();
    let mut device =
        VerilogADevice::try_new("MS1", model, &[1, 0]).expect("multi-stamp model uses native JIT");

    let currents = {
        device.update_voltages(&[4.0]);
        device.try_evaluate().expect("native multi-stamp evaluate")
    };
    assert_eq!(currents.len(), 2);
    assert!((currents[0] - 1.0).abs() < 1e-12, "currents: {currents:?}");
    assert!((currents[1] - 3.0).abs() < 1e-12, "currents: {currents:?}");

    let (matrix, rhs) = stamp_device(&mut device, &[4.0]);
    assert!((matrix.get(&(0, 0)).copied().unwrap_or_default() - 1.0).abs() < 1e-12);
    assert!(
        rhs.values().map(|value| value.abs()).sum::<f64>() < 1e-12,
        "rhs: {rhs:?}"
    );
}
```

- [ ] **Step 3: Add branch-source native tests**

Add:

```rust
#[cfg(target_arch = "x86_64")]
#[test]
fn native_device_stamps_potential_branch_unknowns() {
    let model = potential_branch_model();
    assert_eq!(model.branch_sources.len(), 1);

    let mut device =
        VerilogADevice::try_new("Z1", model, &[1, 2]).expect("potential branch uses native JIT");
    device.set_branch_current_indices(&[3]);

    let (matrix, rhs) = stamp_device(&mut device, &[1.0, 0.5, 1.0e-3]);

    assert!((matrix.get(&(0, 2)).copied().unwrap_or_default() - 1.0).abs() < 1e-12);
    assert!((matrix.get(&(1, 2)).copied().unwrap_or_default() + 1.0).abs() < 1e-12);
    assert!((matrix.get(&(2, 0)).copied().unwrap_or_default() - 1.0).abs() < 1e-12);
    assert!((matrix.get(&(2, 1)).copied().unwrap_or_default() + 1.0).abs() < 1e-12);
    assert!((matrix.get(&(2, 2)).copied().unwrap_or_default() + 2000.0).abs() < 1e-9);
    assert!(rhs.get(&2).copied().unwrap_or_default().abs() < 1e-12, "rhs: {rhs:?}");
}
```

- [ ] **Step 4: Run the contract tests**

Run:

```powershell
& "$env:USERPROFILE\.cargo\bin\cargo.exe" test -p rspice-veriloga --features native --test native_contract native_model_image_publishes_multiple_stamp_and_jacobian_entries native_device_stamps_multiple_flow_contributions_from_one_image native_device_stamps_potential_branch_unknowns -- --nocapture
```

Expected: all selected tests pass. If one fails, fix only the native image layout or x64 lowering issue exposed by that test; do not add interpreter fallback.

- [ ] **Step 5: Add hard-fail coverage for terminal-pair current probes**

Add:

```rust
fn current_probe_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module native_current_probe(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        I(p, n) <+ V(p, n);
        I(p, n) <+ I(p, n) * 0.1;
    end
endmodule
"#,
    )
}

#[test]
fn native_compile_rejects_terminal_pair_current_probes_without_fallback() {
    let model = current_probe_model();

    let err = compile_native(&model).expect_err("PushCurrent remains outside this native slice");
    let msg = err.to_string();

    assert_native_hard_fail_message(&msg);
    assert!(msg.contains("PushCurrent"), "error must name PushCurrent, got: {msg}");
}
```

- [ ] **Step 6: Run the hard-fail test**

Run:

```powershell
& "$env:USERPROFILE\.cargo\bin\cargo.exe" test -p rspice-veriloga --features native --test native_contract native_compile_rejects_terminal_pair_current_probes_without_fallback -- --nocapture
```

Expected: pass with a native JIT no-fallback error that names `PushCurrent`.

- [ ] **Step 7: Commit**

```powershell
git add crates/rspice-veriloga/tests/native_contract.rs
git commit -m "test(native): cover multi-entry x64 model images"
```

## Final Verification

- [ ] **Step 1: Run focused native tests**

```powershell
& "$env:USERPROFILE\.cargo\bin\cargo.exe" test -p rspice-veriloga --features native --test native_contract -- --nocapture
& "$env:USERPROFILE\.cargo\bin\cargo.exe" test -p rspice-veriloga --features native native:: -- --nocapture
& "$env:USERPROFILE\.cargo\bin\cargo.exe" check -p rspice-veriloga --features native
```

Expected: all commands pass.

- [ ] **Step 2: Re-run no-fallback and no-Cranelift guardrails**

```powershell
rg -n "force_interpreter|PlanStep::Interpret|falling back|fallback to the interpreter|using interpreter|Option<.*NativeModel|Vec<Option<StampFn>>|stamp_value_fn|jacobian_fn|and_then\(\|n\|" crates/rspice-veriloga/src crates/rspice-veriloga/tests -g '!target/**'
rg -n "cranelift|Cranelift|cranelift_jit" Cargo.toml Cargo.lock crates README.md NOTICE docs/legal -g '!target/**'
```

Expected: no matches in active code, manifests, lockfile, legal notices, or product docs.

- [ ] **Step 3: Check staged scope before final review**

```powershell
git status --short
git diff --stat HEAD -- crates/rspice-veriloga/src/native crates/rspice-veriloga/tests/native_contract.rs
```

Expected: native-JIT changes are confined to `rspice-veriloga`. The two unrelated `rspice-core` formatter files may still be unstaged.

## Self-Review

- Spec coverage: Tasks 1-3 add the new supported x64 expression coverage; Task 4 locks multi-entry image behavior and the no-fallback boundary for unsupported current probes.
- Placeholder scan: this plan intentionally contains no open-ended implementation placeholders and no unspecified "write tests" steps.
- Type consistency: the plan uses existing `VerilogADevice`, `CompiledModel`, `NativeProgram`, `NativeOp`, `VoltageNode`, `EvalContext`, `Gpr`, `Xmm`, and `compile_native` names from the current codebase.
