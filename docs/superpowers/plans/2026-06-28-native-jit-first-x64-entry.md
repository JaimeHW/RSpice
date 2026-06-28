# Native JIT First X64 Entry Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Compile the first complete native x64 Verilog-A subset into owned executable code, so simple straight-line conductance models run through real native entry points and every unsupported construct still fails with no interpreter fallback.

**Architecture:** This is Plan 2 of the native JIT replacement. The slice introduces executable-image ownership in `NativeModel`, pins the native ABI, expands the x64 encoder for leaf SSE2 functions, adds a small backend-owned expression IR, and lowers supported compiled expression programs into x64 functions. The expression IR is the temporary adapter boundary while canonical OptIR is still only `EvaluateEquation`; it must stay isolated so the canonical OptIR value graph can feed the same backend directly later.

**Tech Stack:** Rust 2024, `rspice-veriloga`, x86-64 Windows and System V C ABI, SSE2 scalar `f64`, W^X executable memory, Verilog-A compiled model artifacts as a temporary semantic adapter, Superpowers TDD and subagent review gates.

---

## Scope Check

The approved design covers canonical OptIR expansion, JIT IR, register allocation, x64 and AArch64 backends, performance benchmarking, and production optimizer work. This plan implements only the first executable x64 slice:

- `NativeModel` owns executable memory for all generated entry points.
- No public safe API returns raw native function pointers.
- `EvalContext` layout and native ABI assumptions are tested.
- x64 encoder supports the small set of scalar loads, stores, arithmetic, constant loads, and returns needed by straight-line leaf functions.
- The backend compiles simple scalar programs containing `PushConst`, `PushParam`, `PushVoltage`, `PushInternalVoltage`, `PushVariable`, `PushBranchCurrent`, `Add`, `Sub`, `Mul`, `Div`, and `Neg`.
- Assignment codegen supports no assignments and scalar `AssignmentStep::Assign` with supported expressions.
- Stamp values and Jacobian programs are compiled for every active stamp in the supported subset.
- Unsupported bytecode instructions, indexed assignments, loops, static conditions, noise, reactive Jacobians, helper calls, stateful operators, current probes, and target gaps return `JitError`; they never run through the interpreter in native mode.

This plan does not claim LLVM-class final performance. It creates the first correct native code path and the ownership, ABI, and lowering boundaries needed for later optimizer and register allocator work.

## File Structure

- `crates/rspice-veriloga/src/native/mod.rs`: route x64 targets to the new backend, keep AArch64 hard-fail, stop publicly re-exporting raw executable memory or function pointer types.
- `crates/rspice-veriloga/src/native/model.rs`: replace static function-pointer construction with owned executable-image entry offsets and call methods.
- `crates/rspice-veriloga/src/native/runtime.rs`: make executable memory an internal native-module implementation detail and add checked pointer helpers.
- `crates/rspice-veriloga/src/native/abi.rs`: add layout tests for `EvalContext` and document ABI register assumptions.
- `crates/rspice-veriloga/src/native/error.rs`: add helper constructors for unsupported compiled-program and entry-layout errors.
- `crates/rspice-veriloga/src/native/expr.rs`: create the backend expression IR and temporary bytecode-program adapter for the supported straight-line subset.
- `crates/rspice-veriloga/src/native/x64/encoder.rs`: add register enums, ModRM/SIB helpers, generic SSE2 arithmetic, memory loads/stores, RIP-relative constant loads, and patching.
- `crates/rspice-veriloga/src/native/x64/codegen.rs`: lower native expression programs into leaf x64 functions and concatenate model entry points into one text image.
- `crates/rspice-veriloga/src/native/x64/mod.rs`: expose the x64 model compiler internally.
- `crates/rspice-veriloga/src/device.rs`: replace native function-pointer getters with `NativeModel` call methods.
- `crates/rspice-veriloga/tests/native_contract.rs`: add success coverage for a simple resistor and keep unsupported no-fallback coverage.

## Task 1: Own Native Executable Images In `NativeModel`

**Files:**
- Modify: `crates/rspice-veriloga/src/native/model.rs`
- Modify: `crates/rspice-veriloga/src/native/runtime.rs`
- Modify: `crates/rspice-veriloga/src/native/mod.rs`
- Modify: `crates/rspice-veriloga/src/device.rs`

- [ ] **Step 1: Write failing model ownership tests**

Add tests to `crates/rspice-veriloga/src/native/model.rs` under `#[cfg(all(test, feature = "native", target_arch = "x86_64"))]`:

```rust
#[test]
fn native_model_calls_entry_points_from_owned_image() {
    let bytes = [
        0xC3,                                           // assignment: ret
        0x66, 0x0F, 0x57, 0xC0, 0xC3,                   // stamp: xorpd xmm0,xmm0; ret
        0x66, 0x0F, 0x57, 0xC0, 0xC3,                   // jacobian: xorpd xmm0,xmm0; ret
    ];
    let image = ExecutableMemory::allocate(&bytes).expect("allocate native test image");
    let model = NativeModel::from_executable_image(
        0,
        image,
        NativeEntryOffsets {
            assignment: CodeOffset::new(0),
            stamp_values: vec![CodeOffset::new(1)],
            jacobians: vec![vec![CodeOffset::new(6)]],
            reactive_jacobians: vec![],
        },
    )
    .expect("publish owned native model");

    let ctx = empty_eval_context();
    model.run_assignments(&ctx, std::ptr::null_mut());
    assert_eq!(model.run_stamp_value(0, &ctx, std::ptr::null()), 0.0);
    assert_eq!(model.run_jacobian(0, 0, &ctx, std::ptr::null()), 0.0);
    assert_eq!(model.native_stamp_count(), 1);
    assert_eq!(model.plan_stats().jacobian_entry_points, 1);
}

#[test]
fn native_model_rejects_entry_offsets_outside_owned_image() {
    let image = ExecutableMemory::allocate(&[0xC3]).expect("allocate native test image");
    let error = NativeModel::from_executable_image(
        0,
        image,
        NativeEntryOffsets {
            assignment: CodeOffset::new(1),
            stamp_values: vec![],
            jacobians: vec![],
            reactive_jacobians: vec![],
        },
    )
    .expect_err("entry at image length must be rejected");

    assert!(error.to_string().contains("entry offset"));
    assert!(error.to_string().contains("no interpreter fallback"));
}
```

Also add the test helper:

```rust
fn empty_eval_context() -> EvalContext {
    EvalContext {
        voltages: std::ptr::null(),
        internal_voltages: std::ptr::null(),
        params: std::ptr::null(),
        branch_currents: std::ptr::null(),
        branch_currents_len: 0,
        currents: std::ptr::null(),
        currents_len: 0,
        num_terminals: 0,
        port_connected: std::ptr::null(),
        port_connected_len: 0,
        temperature: 0.0,
        time: 0.0,
        timestep: 0.0,
        state_prev: std::ptr::null(),
        state_values: std::ptr::null_mut(),
        lookup_tables: std::ptr::null(),
        lookup_tables_len: 0,
        laplace_filters: std::ptr::null_mut(),
        laplace_filters_len: 0,
        param_given: std::ptr::null(),
        branch_unknowns: std::ptr::null(),
        analysis_type: 0,
        multiplicity: 1.0,
    }
}
```

- [ ] **Step 2: Run ownership tests and verify they fail**

Run:

```powershell
cargo test -p rspice-veriloga --features native native::model::tests -- --nocapture
```

Expected: FAIL to compile because `ExecutableMemory`, `NativeEntryOffsets`, `CodeOffset`, and `NativeModel::from_executable_image` are not implemented in `model.rs`.

- [ ] **Step 3: Implement owned executable image entries**

Replace public function pointer getters with internal typed call methods. The final shape must include these exact public and crate-private elements:

```rust
type AssignmentEntry = unsafe extern "C" fn(*const EvalContext, *mut f64);
type ValueEntry = unsafe extern "C" fn(*const EvalContext, *const f64) -> f64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CodeOffset(usize);

impl CodeOffset {
    pub(crate) fn new(offset: usize) -> Self {
        Self(offset)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NativeEntryOffsets {
    pub assignment: CodeOffset,
    pub stamp_values: Vec<CodeOffset>,
    pub jacobians: Vec<Vec<CodeOffset>>,
    pub reactive_jacobians: Vec<Vec<CodeOffset>>,
}

pub struct NativeModel {
    pub num_variables: usize,
    image: ExecutableMemory,
    entries: NativeEntryOffsets,
    stats: PlanStats,
}
```

`NativeModel::from_executable_image` must consume `ExecutableMemory`, validate every `CodeOffset` is strictly less than `image.len()`, compute `PlanStats`, and return `JitResult<Self>`.

The call methods must be safe APIs with the unsafe transmute contained inside `NativeModel`:

```rust
pub(crate) fn run_assignments(&self, ctx: &EvalContext, vars: *mut f64) {
    let entry: AssignmentEntry = unsafe { std::mem::transmute(self.entry_ptr(self.entries.assignment)) };
    unsafe { entry(ctx as *const EvalContext, vars) };
}

pub(crate) fn run_stamp_value(&self, index: usize, ctx: &EvalContext, vars: *const f64) -> f64 {
    self.run_value_entry(self.entries.stamp_values[index], ctx, vars)
}

pub(crate) fn run_jacobian(&self, stamp: usize, entry: usize, ctx: &EvalContext, vars: *const f64) -> f64 {
    self.run_value_entry(self.entries.jacobians[stamp][entry], ctx, vars)
}

pub(crate) fn run_reactive_jacobian(&self, stamp: usize, entry: usize, ctx: &EvalContext, vars: *const f64) -> f64 {
    self.run_value_entry(self.entries.reactive_jacobians[stamp][entry], ctx, vars)
}
```

Keep a short safety comment above each transmute explaining that offsets were validated and the `ExecutableMemory` is owned by `self`.

- [ ] **Step 4: Make executable memory internal to native**

In `crates/rspice-veriloga/src/native/runtime.rs`, change `ExecutableMemory` to `pub(crate) struct ExecutableMemory` and expose only crate-private code pointer access:

```rust
pub(crate) fn ptr_at(&self, offset: usize) -> JitResult<*const u8> {
    if offset >= self.len {
        return Err(JitError::ExecutableMemory {
            detail: format!("entry offset {offset} outside executable image length {}", self.len).into(),
        });
    }
    Ok(unsafe { self.ptr.add(offset).cast_const() })
}
```

Keep `len()` and `allocate()` crate-visible. Do not re-export `ExecutableMemory` from `native/mod.rs`.

- [ ] **Step 5: Update device native calls**

In `crates/rspice-veriloga/src/device.rs`, replace `native.stamp_value_fn(...)`, `native.jacobian_fn(...)`, and `native.reactive_jacobian_fn(...)` call paths with `native.run_stamp_value(...)`, `native.run_jacobian(...)`, and `native.run_reactive_jacobian(...)`. `run_value_program` should become:

```rust
#[cfg(feature = "native")]
fn run_value_program(
    vm: &mut Vm<'_>,
    _program: &crate::codegen::BytecodeProgram,
    native: &NativeModel,
    entry: NativeValueEntry,
) -> Result<f64, VmError> {
    let ctx = Self::eval_context_from(vm.context);
    let vars_ptr = vm.context.variables.as_ptr();
    Ok(match entry {
        NativeValueEntry::StampValue(index) => native.run_stamp_value(index, &ctx, vars_ptr),
        NativeValueEntry::Jacobian { stamp, entry } => native.run_jacobian(stamp, entry, &ctx, vars_ptr),
        NativeValueEntry::ReactiveJacobian { stamp, entry } => native.run_reactive_jacobian(stamp, entry, &ctx, vars_ptr),
    })
}
```

Add:

```rust
#[cfg(feature = "native")]
enum NativeValueEntry {
    StampValue(usize),
    Jacobian { stamp: usize, entry: usize },
    ReactiveJacobian { stamp: usize, entry: usize },
}
```

No safe device code may store a native function pointer.

- [ ] **Step 6: Run ownership and native module tests**

Run:

```powershell
cargo test -p rspice-veriloga --features native native::model::tests -- --nocapture
cargo test -p rspice-veriloga --features native native::runtime::tests -- --nocapture
```

Expected: PASS.

- [ ] **Step 7: Commit**

Run:

```powershell
git add crates/rspice-veriloga/src/native/model.rs crates/rspice-veriloga/src/native/runtime.rs crates/rspice-veriloga/src/native/mod.rs crates/rspice-veriloga/src/device.rs
git commit -m "feat: own native jit executable images"
```

## Task 2: Pin Native ABI Layout And Expand X64 Encoder

**Files:**
- Modify: `crates/rspice-veriloga/src/native/abi.rs`
- Modify: `crates/rspice-veriloga/src/native/x64/encoder.rs`

- [ ] **Step 1: Write failing ABI layout tests**

Add tests to `native/abi.rs`:

```rust
#[cfg(all(test, feature = "native", target_arch = "x86_64"))]
mod tests {
    use super::EvalContext;
    use std::mem::{align_of, offset_of, size_of};

    #[test]
    fn eval_context_layout_matches_x64_jit_offsets() {
        assert_eq!(offset_of!(EvalContext, voltages), 0);
        assert_eq!(offset_of!(EvalContext, internal_voltages), 8);
        assert_eq!(offset_of!(EvalContext, params), 16);
        assert_eq!(offset_of!(EvalContext, branch_currents), 24);
        assert_eq!(offset_of!(EvalContext, branch_currents_len), 32);
        assert_eq!(offset_of!(EvalContext, currents), 40);
        assert_eq!(offset_of!(EvalContext, currents_len), 48);
        assert_eq!(offset_of!(EvalContext, num_terminals), 56);
        assert_eq!(offset_of!(EvalContext, port_connected), 64);
        assert_eq!(offset_of!(EvalContext, port_connected_len), 72);
        assert_eq!(offset_of!(EvalContext, temperature), 80);
        assert_eq!(offset_of!(EvalContext, time), 88);
        assert_eq!(offset_of!(EvalContext, timestep), 96);
        assert_eq!(offset_of!(EvalContext, state_prev), 104);
        assert_eq!(offset_of!(EvalContext, state_values), 112);
        assert_eq!(offset_of!(EvalContext, lookup_tables), 120);
        assert_eq!(offset_of!(EvalContext, lookup_tables_len), 128);
        assert_eq!(offset_of!(EvalContext, laplace_filters), 136);
        assert_eq!(offset_of!(EvalContext, laplace_filters_len), 144);
        assert_eq!(offset_of!(EvalContext, param_given), 152);
        assert_eq!(offset_of!(EvalContext, branch_unknowns), 160);
        assert_eq!(offset_of!(EvalContext, analysis_type), 168);
        assert_eq!(offset_of!(EvalContext, multiplicity), 176);
        assert_eq!(size_of::<EvalContext>(), 184);
        assert_eq!(align_of::<EvalContext>(), 8);
    }
}
```

- [ ] **Step 2: Write failing encoder tests**

In `x64/encoder.rs`, add register enums and tests first:

```rust
#[test]
fn encodes_windows_param_load_leaf() {
    let mut encoder = X64Encoder::new();
    encoder.mov_r64_m64_base_disp32(Gpr::Rax, Gpr::Rcx, 16);
    encoder.movsd_xmm_m64_base_disp32(Xmm::Xmm0, Gpr::Rax, 24);
    encoder.ret();

    assert_eq!(
        encoder.into_bytes(),
        [0x48, 0x8B, 0x81, 16, 0, 0, 0, 0xF2, 0x0F, 0x10, 0x80, 24, 0, 0, 0, 0xC3]
    );
}

#[test]
fn encodes_system_v_param_load_leaf() {
    let mut encoder = X64Encoder::new();
    encoder.mov_r64_m64_base_disp32(Gpr::Rax, Gpr::Rdi, 16);
    encoder.movsd_xmm_m64_base_disp32(Xmm::Xmm0, Gpr::Rax, 24);
    encoder.ret();

    assert_eq!(
        encoder.into_bytes(),
        [0x48, 0x8B, 0x87, 16, 0, 0, 0, 0xF2, 0x0F, 0x10, 0x80, 24, 0, 0, 0, 0xC3]
    );
}

#[test]
fn encodes_scalar_register_and_memory_ops() {
    let mut encoder = X64Encoder::new();
    encoder.xorpd_xmm_xmm(Xmm::Xmm0, Xmm::Xmm0);
    encoder.movsd_xmm_xmm(Xmm::Xmm1, Xmm::Xmm0);
    encoder.addsd_xmm_xmm(Xmm::Xmm1, Xmm::Xmm0);
    encoder.subsd_xmm_xmm(Xmm::Xmm1, Xmm::Xmm0);
    encoder.mulsd_xmm_xmm(Xmm::Xmm1, Xmm::Xmm0);
    encoder.divsd_xmm_xmm(Xmm::Xmm1, Xmm::Xmm0);
    encoder.movsd_m64_base_disp32_xmm(Gpr::Rdx, 16, Xmm::Xmm1);
    encoder.ret();

    assert_eq!(
        encoder.into_bytes(),
        [
            0x66, 0x0F, 0x57, 0xC0,
            0xF2, 0x0F, 0x10, 0xC8,
            0xF2, 0x0F, 0x58, 0xC8,
            0xF2, 0x0F, 0x5C, 0xC8,
            0xF2, 0x0F, 0x59, 0xC8,
            0xF2, 0x0F, 0x5E, 0xC8,
            0xF2, 0x0F, 0x11, 0x8A, 16, 0, 0, 0,
            0xC3,
        ]
    );
}
```

- [ ] **Step 3: Run ABI and encoder tests and verify they fail**

Run:

```powershell
cargo test -p rspice-veriloga --features native native::abi::tests::eval_context_layout_matches_x64_jit_offsets
cargo test -p rspice-veriloga --features native native::x64::encoder::tests -- --nocapture
```

Expected: ABI test may pass once added; encoder tests fail to compile because register-generic methods do not exist.

- [ ] **Step 4: Implement generic x64 encoder primitives**

Add:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Gpr { Rax, Rcx, Rdx, Rdi, Rsi, R10, R11 }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Xmm { Xmm0, Xmm1, Xmm2, Xmm3, Xmm4, Xmm5 }
```

Implement ModRM-based emitters for:

```rust
mov_r64_m64_base_disp32(dst: Gpr, base: Gpr, disp: i32)
movsd_xmm_m64_base_disp32(dst: Xmm, base: Gpr, disp: i32)
movsd_m64_base_disp32_xmm(base: Gpr, disp: i32, src: Xmm)
movsd_xmm_m64_rip_disp32(dst: Xmm, disp: i32) -> usize
movsd_xmm_xmm(dst: Xmm, src: Xmm)
xorpd_xmm_xmm(dst: Xmm, src: Xmm)
addsd_xmm_xmm(dst: Xmm, src: Xmm)
subsd_xmm_xmm(dst: Xmm, src: Xmm)
mulsd_xmm_xmm(dst: Xmm, src: Xmm)
divsd_xmm_xmm(dst: Xmm, src: Xmm)
patch_i32(offset: usize, value: i32)
position() -> usize
```

Preserve the old fixed `addsd_xmm0_xmm1` methods as wrappers if existing tests use them.

- [ ] **Step 5: Add executable ABI smoke tests**

In `x64/encoder.rs`, add one runtime test that builds and executes a leaf function:

```rust
#[test]
fn encoded_leaf_loads_and_combines_context_values() {
    let params = [2.0_f64];
    let voltages = [5.0_f64];
    let internals = [1.0_f64];
    let vars = [0.0_f64, 8.0_f64];
    let branch_unknowns = [4.0_f64];
    let ctx = EvalContext { /* use same empty helper shape, with arrays above wired in */ };

    let mut encoder = X64Encoder::new();
    let ctx_reg = host_ctx_arg_reg();
    let vars_reg = host_vars_arg_reg();
    encoder.mov_r64_m64_base_disp32(Gpr::Rax, ctx_reg, 0);
    encoder.movsd_xmm_m64_base_disp32(Xmm::Xmm0, Gpr::Rax, 0);
    encoder.mov_r64_m64_base_disp32(Gpr::Rax, ctx_reg, 8);
    encoder.movsd_xmm_m64_base_disp32(Xmm::Xmm1, Gpr::Rax, 0);
    encoder.subsd_xmm_xmm(Xmm::Xmm0, Xmm::Xmm1);
    encoder.mov_r64_m64_base_disp32(Gpr::Rax, ctx_reg, 16);
    encoder.movsd_xmm_m64_base_disp32(Xmm::Xmm1, Gpr::Rax, 0);
    encoder.mulsd_xmm_xmm(Xmm::Xmm0, Xmm::Xmm1);
    encoder.movsd_xmm_m64_base_disp32(Xmm::Xmm1, vars_reg, 8);
    encoder.mov_r64_m64_base_disp32(Gpr::Rax, ctx_reg, 160);
    encoder.movsd_xmm_m64_base_disp32(Xmm::Xmm2, Gpr::Rax, 0);
    encoder.divsd_xmm_xmm(Xmm::Xmm1, Xmm::Xmm2);
    encoder.addsd_xmm_xmm(Xmm::Xmm0, Xmm::Xmm1);
    encoder.ret();

    let memory = ExecutableMemory::allocate(&encoder.into_bytes()).expect("allocate leaf");
    let entry = memory.ptr_at(0).expect("entry point inside image");
    let f: extern "C" fn(*const EvalContext, *const f64) -> f64 = unsafe { std::mem::transmute(entry) };
    assert_eq!(f(&ctx, vars.as_ptr()), 10.0);
}
```

- [ ] **Step 6: Run tests**

Run:

```powershell
cargo test -p rspice-veriloga --features native native::abi::tests native::x64::encoder::tests -- --nocapture
```

Expected: PASS.

- [ ] **Step 7: Commit**

Run:

```powershell
git add crates/rspice-veriloga/src/native/abi.rs crates/rspice-veriloga/src/native/x64/encoder.rs
git commit -m "feat: expand x64 leaf encoder and pin abi"
```

## Task 3: Add Backend Expression IR And Supported Program Lowering

**Files:**
- Create: `crates/rspice-veriloga/src/native/expr.rs`
- Modify: `crates/rspice-veriloga/src/native/error.rs`
- Modify: `crates/rspice-veriloga/src/native/mod.rs`

- [ ] **Step 1: Write failing expression lowering tests**

Create `native/expr.rs` with tests first:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::{BytecodeProgram, Instruction};

    #[test]
    fn lowers_supported_stack_program_to_native_expr_ops() {
        let program = BytecodeProgram {
            instructions: vec![
                Instruction::PushVoltage(0, 1),
                Instruction::PushParam(0),
                Instruction::Div,
                Instruction::PushConst(2.0),
                Instruction::Mul,
            ],
        };

        let lowered = NativeProgram::from_bytecode("res", EntryKind::StampValue, &program)
            .expect("lower supported program");

        assert_eq!(
            lowered.ops(),
            &[
                NativeOp::LoadVoltage { pos: 0, neg: 1 },
                NativeOp::LoadParam(0),
                NativeOp::Div,
                NativeOp::Const(2.0),
                NativeOp::Mul,
            ]
        );
        assert_eq!(lowered.max_stack_depth(), 2);
    }

    #[test]
    fn lowering_rejects_current_probe_without_fallback() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::PushCurrent(0, 1)],
        };

        let error = NativeProgram::from_bytecode("probe", EntryKind::StampValue, &program)
            .expect_err("current probe is outside this slice");
        let msg = error.to_string();
        assert!(msg.contains("PushCurrent"));
        assert!(msg.contains("native JIT"));
        assert!(msg.contains("no interpreter fallback"));
    }

    #[test]
    fn lowering_rejects_unbalanced_stack() {
        let program = BytecodeProgram {
            instructions: vec![Instruction::Add],
        };

        let error = NativeProgram::from_bytecode("bad", EntryKind::StampValue, &program)
            .expect_err("binary op without operands must fail");
        assert!(error.to_string().contains("stack"));
    }
}
```

- [ ] **Step 2: Run expression tests and verify they fail**

Run:

```powershell
cargo test -p rspice-veriloga --features native native::expr::tests -- --nocapture
```

Expected: FAIL to compile because `native::expr` is not wired.

- [ ] **Step 3: Implement expression IR and bytecode adapter**

Add:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum EntryKind {
    Assignment,
    StampValue,
    Jacobian,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum NativeOp {
    Const(f64),
    LoadParam(usize),
    LoadVoltage { pos: usize, neg: usize },
    LoadInternalVoltage(usize),
    LoadVariable(usize),
    LoadBranchUnknown(usize),
    Add,
    Sub,
    Mul,
    Div,
    Neg,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct NativeProgram {
    ops: Vec<NativeOp>,
    max_stack_depth: usize,
}
```

`NativeProgram::from_bytecode` must support only:

```rust
Instruction::PushConst
Instruction::PushParam
Instruction::PushVoltage
Instruction::PushInternalVoltage
Instruction::PushVariable
Instruction::PushBranchCurrent
Instruction::Add
Instruction::Sub
Instruction::Mul
Instruction::Div
Instruction::Neg
```

Map `PushBranchCurrent(index)` to `LoadBranchUnknown(index)`. Reject `PushCurrent(pos, neg)` for now because it requires the sequential contribution-current/current-probe semantics.

Validate stack depth exactly:
- load ops push one value.
- `Neg` requires one value and keeps depth unchanged.
- binary ops require depth >= 2 and reduce depth by one.
- final depth must be exactly one for value entries and assignment expression programs.

Add `mod expr;` to `native/mod.rs`.

- [ ] **Step 4: Add error helpers**

In `native/error.rs`, add:

```rust
pub fn unsupported_program_op(
    model: impl Into<SmolStr>,
    op: impl Into<SmolStr>,
) -> Self {
    Self::UnsupportedCanonicalOp {
        model: model.into(),
        op: op.into(),
    }
}
```

Use operation names such as `PushCurrent`, `PushVariableDyn`, `Limexp`, and `DdtState` in diagnostics. The message still states `native JIT` and `no interpreter fallback`.

- [ ] **Step 5: Run expression tests**

Run:

```powershell
cargo test -p rspice-veriloga --features native native::expr::tests -- --nocapture
```

Expected: PASS.

- [ ] **Step 6: Commit**

Run:

```powershell
git add crates/rspice-veriloga/src/native/expr.rs crates/rspice-veriloga/src/native/error.rs crates/rspice-veriloga/src/native/mod.rs
git commit -m "feat: add native expression lowering subset"
```

## Task 4: Generate Leaf X64 Functions From Native Expressions

**Files:**
- Create: `crates/rspice-veriloga/src/native/x64/codegen.rs`
- Modify: `crates/rspice-veriloga/src/native/x64/mod.rs`
- Modify: `crates/rspice-veriloga/src/native/x64/encoder.rs`

- [ ] **Step 1: Write failing x64 expression-codegen tests**

Create `native/x64/codegen.rs` with tests:

```rust
#[cfg(all(test, feature = "native", target_arch = "x86_64"))]
mod tests {
    use super::*;

    #[test]
    fn generated_leaf_returns_arithmetic_over_context_arrays() {
        let mut program = NativeProgram::empty_for_test();
        program.push_for_test(NativeOp::LoadParam(0));
        program.push_for_test(NativeOp::LoadVoltage { pos: 0, neg: 1 });
        program.push_for_test(NativeOp::Mul);
        program.push_for_test(NativeOp::LoadVariable(1));
        program.push_for_test(NativeOp::Const(4.0));
        program.push_for_test(NativeOp::Div);
        program.push_for_test(NativeOp::Add);

        let function = compile_value_function_for_test(&program).expect("compile leaf value fn");
        let params = [2.0_f64];
        let voltages = [5.0_f64, 1.0_f64];
        let vars = [0.0_f64, 8.0_f64];
        let mut ctx = empty_eval_context();
        ctx.params = params.as_ptr();
        ctx.voltages = voltages.as_ptr();

        let result = function.call(&ctx, vars.as_ptr());
        assert_eq!(result, 10.0);
    }

    #[test]
    fn generated_assignment_writes_variable_slot() {
        let mut program = NativeProgram::empty_for_test();
        program.push_for_test(NativeOp::LoadParam(0));
        program.push_for_test(NativeOp::LoadVoltage { pos: 1, neg: 0 });
        program.push_for_test(NativeOp::Add);

        let function = compile_assignment_function_for_test(2, &program)
            .expect("compile assignment fn");
        let params = [3.0_f64];
        let voltages = [1.0_f64, 6.0_f64];
        let mut vars = [0.0_f64, 0.0_f64, 0.0_f64];
        let mut ctx = empty_eval_context();
        ctx.params = params.as_ptr();
        ctx.voltages = voltages.as_ptr();

        function.call(&ctx, vars.as_mut_ptr());
        assert_eq!(vars[2], 8.0);
    }

    fn empty_eval_context() -> EvalContext {
        EvalContext {
            voltages: std::ptr::null(),
            internal_voltages: std::ptr::null(),
            params: std::ptr::null(),
            branch_currents: std::ptr::null(),
            branch_currents_len: 0,
            currents: std::ptr::null(),
            currents_len: 0,
            num_terminals: 0,
            port_connected: std::ptr::null(),
            port_connected_len: 0,
            temperature: 0.0,
            time: 0.0,
            timestep: 0.0,
            state_prev: std::ptr::null(),
            state_values: std::ptr::null_mut(),
            lookup_tables: std::ptr::null(),
            lookup_tables_len: 0,
            laplace_filters: std::ptr::null_mut(),
            laplace_filters_len: 0,
            param_given: std::ptr::null(),
            branch_unknowns: std::ptr::null(),
            analysis_type: 0,
            multiplicity: 1.0,
        }
    }
}
```

- [ ] **Step 2: Run codegen tests and verify they fail**

Run:

```powershell
cargo test -p rspice-veriloga --features native native::x64::codegen::tests -- --nocapture
```

Expected: FAIL to compile because codegen is not implemented.

- [ ] **Step 3: Implement x64 leaf compiler**

Implement a simple stack compiler:

- ABI argument registers:
  - Windows: `ctx = Gpr::Rcx`, `vars = Gpr::Rdx`.
  - Unix System V: `ctx = Gpr::Rdi`, `vars = Gpr::Rsi`.
- Use only `Xmm0` through `Xmm5`.
- Reject programs whose max stack depth is greater than six with `JitError::RegisterAllocation`.
- Load pointers from `EvalContext` into `Rax` using pinned offsets:
  - voltages: 0
  - internal_voltages: 8
  - params: 16
  - branch_unknowns: 160
- Load `V(pos, neg)` by loading each terminal voltage into registers; use `xorpd` for ground only when a future node mapping marks ground. For this temporary adapter, `PushVoltage(pos, neg)` reads both terminal indices from `ctx.voltages`.
- Load constants from a per-function literal pool with RIP-relative `movsd`.
- Emit binary ops in stack-machine order: left operand remains below right operand, result replaces left.
- Emit `ret` with no prologue and no stack use.
- For assignment functions, compile each scalar assignment expression and then `movsd [vars + var_index * 8], xmm_result`.

The implementation must not call helper functions, allocate stack space, or use non-volatile registers in this slice.

- [ ] **Step 4: Run x64 codegen tests**

Run:

```powershell
cargo test -p rspice-veriloga --features native native::x64::codegen::tests -- --nocapture
```

Expected: PASS.

- [ ] **Step 5: Commit**

Run:

```powershell
git add crates/rspice-veriloga/src/native/x64/codegen.rs crates/rspice-veriloga/src/native/x64/mod.rs crates/rspice-veriloga/src/native/x64/encoder.rs
git commit -m "feat: generate x64 leaf expression functions"
```

## Task 5: Compile Supported Models To Complete Native Images

**Files:**
- Modify: `crates/rspice-veriloga/src/native/mod.rs`
- Modify: `crates/rspice-veriloga/src/native/x64/codegen.rs`
- Modify: `crates/rspice-veriloga/src/native/x64/mod.rs`
- Modify: `crates/rspice-veriloga/tests/native_contract.rs`

- [ ] **Step 1: Write failing native success and no-fallback tests**

Add to `native_contract.rs`:

```rust
fn simple_resistor_model() -> rspice_veriloga::CompiledModel {
    compile(
        r#"
`include "disciplines.vams"
module rnative(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 2.0 from (0:inf);
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#,
    )
}

#[test]
fn native_compile_accepts_simple_resistor_subset() {
    let model = simple_resistor_model();
    let native = compile_native(&model).expect("simple resistor is in the first native subset");

    assert_eq!(native.native_stamp_count(), 1);
    assert_eq!(native.plan_stats().jacobian_entry_points, 1);
}

#[test]
fn native_device_stamps_simple_resistor_without_interpreter_fallback() {
    let model = simple_resistor_model();
    let mut device = VerilogADevice::try_new("RN1", model, &[1, 0])
        .expect("simple resistor must construct with complete native code");

    assert!(device.is_using_native());

    let mut matrix = std::collections::HashMap::<(usize, usize), f64>::new();
    let mut rhs = std::collections::HashMap::<usize, f64>::new();
    device.stamp(
        &[4.0],
        |row, col, value| *matrix.entry((row, col)).or_insert(0.0) += value,
        |node, value| *rhs.entry(node).or_insert(0.0) += value,
    );

    assert_eq!(matrix.len(), 1);
    assert!((matrix[&(0, 0)] - 0.5).abs() < 1.0e-12);
    assert!(rhs.values().map(|v| v.abs()).sum::<f64>() < 1.0e-12);
}

#[test]
fn native_compile_rejects_runtime_loop_without_fallback() {
    let model = hybrid_fallback_model();
    let error = compile_native(&model).expect_err("runtime loops are outside this slice");
    let msg = error.to_string();
    assert_native_hard_fail_message(&msg);
    assert!(
        msg.contains("Loop") || msg.contains("PushVariableDyn") || msg.contains("unsupported"),
        "error should name the unsupported subset feature, got: {msg}"
    );
}
```

- [ ] **Step 2: Run tests and verify they fail**

Run:

```powershell
cargo test -p rspice-veriloga --features native --test native_contract native_compile_accepts_simple_resistor_subset -- --nocapture
cargo test -p rspice-veriloga --features native --test native_contract native_device_stamps_simple_resistor_without_interpreter_fallback -- --nocapture
cargo test -p rspice-veriloga --features native --test native_contract native_compile_rejects_runtime_loop_without_fallback -- --nocapture
```

Expected: simple resistor tests fail because `compile_native` still returns unsupported `EvaluateEquation`; unsupported runtime loop may already fail but must continue to include the no-fallback contract.

- [ ] **Step 3: Implement x64 model compilation**

In `x64/mod.rs`, expose:

```rust
pub(crate) fn compile_model(model: &CompiledModel) -> JitResult<NativeModel>
```

The implementation must:

- Reject any parameter with `default_program.is_some()` using `UnsupportedNativeCoverage` feature `DependentParameterDefaults`.
- Reject any `stamp.static_condition.is_some()` using `UnsupportedNativeCoverage` feature `StaticConditionPrograms`.
- Reject non-empty `noise_sources` and non-empty `reactive_jacobians`.
- Reject `AssignmentStep::Loop` as `Loop`.
- Reject `AssignmentStep::AssignIndexed` as `AssignIndexed`.
- Compile every scalar `AssignmentStep::Assign` through `NativeProgram::from_bytecode`.
- Emit a no-op assignment function containing only `ret` when there are no assignments.
- Compile every stamp value program and every jacobian program.
- Concatenate all generated functions into one `Vec<u8>` and record `CodeOffset`s.
- Allocate one `ExecutableMemory` from that text image.
- Return `NativeModel::from_executable_image(model.num_variables, image, offsets)`.

In `native/mod.rs`, change the x64 branch to call `x64::compile_model(model)`.

- [ ] **Step 4: Run native contract tests**

Run:

```powershell
cargo test -p rspice-veriloga --features native --test native_contract -- --nocapture
```

Expected: PASS. The simple resistor constructs and stamps with native x64 functions. The hybrid model, noise model, and reactive model still fail closed with no interpreter fallback.

- [ ] **Step 5: Commit**

Run:

```powershell
git add crates/rspice-veriloga/src/native/mod.rs crates/rspice-veriloga/src/native/x64/codegen.rs crates/rspice-veriloga/src/native/x64/mod.rs crates/rspice-veriloga/tests/native_contract.rs
git commit -m "feat: compile simple veriloga subset to native x64"
```

## Task 6: Verification And Guardrails

**Files:**
- Inspect: all files changed by Tasks 1-5

- [ ] **Step 1: Verify focused native tests**

Run:

```powershell
cargo test -p rspice-veriloga --features native --test native_contract -- --nocapture
cargo test -p rspice-veriloga --features native native:: -- --nocapture
```

Expected: PASS.

- [ ] **Step 2: Verify no native fallback symbols reappeared**

Run:

```powershell
rg -n "force_interpreter|PlanStep::Interpret|falling back|fallback to the interpreter|using interpreter|Option<.*NativeModel|Vec<Option<StampFn>>|stamp_value_fn|jacobian_fn|and_then\\(\\|n\\|" crates/rspice-veriloga/src crates/rspice-veriloga/tests -g '!target/**'
```

Expected: no matches that represent native-mode fallback or public function-pointer entry getters.

- [ ] **Step 3: Verify Cranelift remains absent**

Run:

```powershell
rg -n "cranelift|Cranelift|cranelift_jit" Cargo.toml Cargo.lock crates README.md NOTICE docs/legal -g '!target/**'
```

Expected: no matches in active source, manifests, lockfile, legal notices, or product documentation.

- [ ] **Step 4: Verify crate build**

Run:

```powershell
cargo check -p rspice-veriloga --features native
```

Expected: PASS.

- [ ] **Step 5: Re-run known wider checks and classify failures**

Run:

```powershell
cargo test -p rspice-veriloga
cargo check -p rspice-core --features veriloga-native
cargo check -p rspice-cli
cargo check -p rspice-ui --no-default-features --features desktop
```

Expected: native-owned changes must not introduce new failures. If the already-observed AOT Rust backend golden failure or existing `rspice-core` signature drift remains, record it as pre-existing only after confirming it is unchanged from the previous baseline notes.

- [ ] **Step 6: Commit verification fixes if needed**

If any native-owned verification issue required edits, commit them:

```powershell
git add <fixed files>
git commit -m "fix: verify first native x64 entry slice"
```

No commit is needed if verification passes without edits.

## Completion Criteria

This plan is complete when:

- `NativeModel` owns `ExecutableMemory` and validates entry offsets before publication.
- Device code calls native entry methods on `NativeModel` and never stores raw function pointers.
- `EvalContext` layout is pinned with x64 offset tests.
- x64 encoder tests cover register-generic scalar loads, stores, arithmetic, constants, and returns.
- The x64 backend compiles a simple two-terminal resistor into native assignment, stamp, and jacobian entry points.
- `VerilogADevice::try_new` succeeds for the simple resistor under `--features native`.
- Unsupported models still fail at construction or native compilation with messages containing `native JIT` and `no interpreter fallback`.
- Focused native tests and `cargo check -p rspice-veriloga --features native` pass.
- Active Cranelift and native-interpreter-fallback searches remain clean.
