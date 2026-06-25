# Verilog-A Rust Codegen Core Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the first Verilog-A to Rust code-generation core so CMC models under `models/veriloga/cmc/` can become generated native Rust devices instead of hand-written ports.

**Architecture:** Reuse `rspice-veriloga` as the parser and semantic front end, lower analyzed modules into a compact-model IR, then emit Rust that implements a small simulator-facing generated-device contract. The first milestone uses a tiny checked-in Verilog-A fixture and proves interpreted VM output and generated Rust output agree before attempting PSP/HICUM/MEXTRAM/BSIM-CMG packages.

**Tech Stack:** Rust, `rspice-veriloga`, `rspice-core`, generated Rust modules, `prettyplease`/`syn` style formatting if already available or `rustfmt` on generated files, focused debug tests, release-only full ngspice regression gate when the broader simulator suite is explicitly run.

---

## File Structure

- Create `crates/rspice-veriloga/src/codegen/mod.rs`: public entry point for Rust code generation.
- Create `crates/rspice-veriloga/src/codegen/ir.rs`: model IR for modules, ports, parameters, internal nodes, branches, equations, and analysis domains.
- Create `crates/rspice-veriloga/src/codegen/rust.rs`: Rust text emitter for generated devices.
- Create `crates/rspice-veriloga/src/codegen/fixture.rs`: test-only helpers for compiling small Verilog-A source strings into IR and generated Rust.
- Modify `crates/rspice-veriloga/src/lib.rs`: expose codegen behind a feature or public module, following existing crate conventions.
- Create `crates/rspice-veriloga/tests/codegen_core.rs`: red/green tests for IR lowering and generated output.
- Create `crates/rspice-core/src/device/generated.rs`: minimal generated-device runtime contract owned by the simulator core.
- Modify `crates/rspice-core/src/device/mod.rs`: expose the generated-device contract.

---

### Task 1: Add The Generated-Device Runtime Contract

**Files:**
- Create: `crates/rspice-core/src/device/generated.rs`
- Modify: `crates/rspice-core/src/device/mod.rs`

- [ ] **Step 1: Write the generated-device API**

Create `crates/rspice-core/src/device/generated.rs`:

```rust
use crate::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeneratedAnalysisDomain {
    Dc,
    Ac,
    Transient,
    Noise,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GeneratedModelInfo {
    pub module_name: &'static str,
    pub source_package: &'static str,
    pub source_digest: &'static str,
    pub ports: &'static [&'static str],
    pub parameters: &'static [&'static str],
}

#[derive(Debug, Clone, PartialEq)]
pub struct GeneratedEvalInput<'a> {
    pub voltages: &'a [Value],
    pub branch_currents: &'a [Value],
    pub parameters: &'a [Value],
    pub temperature_kelvin: Value,
    pub time: Value,
    pub frequency_hz: Value,
    pub domain: GeneratedAnalysisDomain,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GeneratedContribution {
    pub pos: usize,
    pub neg: usize,
    pub current: Value,
    pub conductance: Value,
}

pub trait GeneratedCompactModel {
    fn info(&self) -> &'static GeneratedModelInfo;
    fn evaluate(&self, input: &GeneratedEvalInput<'_>) -> Vec<GeneratedContribution>;
}
```

- [ ] **Step 2: Export the API**

Add to `crates/rspice-core/src/device/mod.rs`:

```rust
pub mod generated;
pub use generated::{
    GeneratedAnalysisDomain, GeneratedCompactModel, GeneratedContribution, GeneratedEvalInput,
    GeneratedModelInfo,
};
```

- [ ] **Step 3: Run the focused check**

Run:

```powershell
cargo check -p rspice-core --tests
```

Expected: pass in debug profile.

### Task 2: Add A Codegen IR

**Files:**
- Create: `crates/rspice-veriloga/src/codegen/mod.rs`
- Create: `crates/rspice-veriloga/src/codegen/ir.rs`
- Modify: `crates/rspice-veriloga/src/lib.rs`

- [ ] **Step 1: Create the module entry point**

Create `crates/rspice-veriloga/src/codegen/mod.rs`:

```rust
pub mod ir;
pub mod rust;

pub use ir::{
    CodegenBranch, CodegenContribution, CodegenDomain, CodegenExpr, CodegenModel, CodegenParameter,
    CodegenPort,
};
pub use rust::emit_rust_device;
```

- [ ] **Step 2: Create the IR types**

Create `crates/rspice-veriloga/src/codegen/ir.rs`:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodegenDomain {
    Dc,
    Ac,
    Transient,
    Noise,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodegenPort {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CodegenParameter {
    pub name: String,
    pub default: f64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodegenBranch {
    pub name: String,
    pub pos: String,
    pub neg: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CodegenExpr {
    Const(f64),
    Parameter(String),
    Voltage { pos: String, neg: String },
    Add(Box<CodegenExpr>, Box<CodegenExpr>),
    Sub(Box<CodegenExpr>, Box<CodegenExpr>),
    Mul(Box<CodegenExpr>, Box<CodegenExpr>),
    Div(Box<CodegenExpr>, Box<CodegenExpr>),
    Exp(Box<CodegenExpr>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CodegenContribution {
    pub branch: CodegenBranch,
    pub current: CodegenExpr,
    pub conductance_hint: Option<CodegenExpr>,
    pub domain: CodegenDomain,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CodegenModel {
    pub module_name: String,
    pub source_package: String,
    pub source_digest: String,
    pub ports: Vec<CodegenPort>,
    pub parameters: Vec<CodegenParameter>,
    pub branches: Vec<CodegenBranch>,
    pub contributions: Vec<CodegenContribution>,
}
```

- [ ] **Step 3: Export the codegen module**

Add to `crates/rspice-veriloga/src/lib.rs` near the other public modules:

```rust
pub mod codegen;
```

- [ ] **Step 4: Run the focused check**

Run:

```powershell
cargo check -p rspice-veriloga --tests
```

Expected: pass in debug profile.

### Task 3: Emit A Minimal Rust Device

**Files:**
- Create: `crates/rspice-veriloga/src/codegen/rust.rs`
- Create: `crates/rspice-veriloga/tests/codegen_core.rs`

- [ ] **Step 1: Write the red emitter test**

Create `crates/rspice-veriloga/tests/codegen_core.rs`:

```rust
use rspice_veriloga::codegen::{
    CodegenBranch, CodegenContribution, CodegenDomain, CodegenExpr, CodegenModel,
    CodegenParameter, CodegenPort, emit_rust_device,
};

#[test]
fn rust_emitter_generates_static_model_info_and_eval_function() {
    let model = CodegenModel {
        module_name: "tiny_res".to_string(),
        source_package: "fixture".to_string(),
        source_digest: "sha256:fixture".to_string(),
        ports: vec![CodegenPort { name: "p".to_string() }, CodegenPort { name: "n".to_string() }],
        parameters: vec![CodegenParameter { name: "g".to_string(), default: 0.001 }],
        branches: vec![CodegenBranch {
            name: "p_n".to_string(),
            pos: "p".to_string(),
            neg: "n".to_string(),
        }],
        contributions: vec![CodegenContribution {
            branch: CodegenBranch {
                name: "p_n".to_string(),
                pos: "p".to_string(),
                neg: "n".to_string(),
            },
            current: CodegenExpr::Mul(
                Box::new(CodegenExpr::Parameter("g".to_string())),
                Box::new(CodegenExpr::Voltage { pos: "p".to_string(), neg: "n".to_string() }),
            ),
            conductance_hint: Some(CodegenExpr::Parameter("g".to_string())),
            domain: CodegenDomain::Dc,
        }],
    };

    let rust = emit_rust_device(&model).expect("emit tiny resistor");
    assert!(rust.contains("pub struct TinyResGenerated"));
    assert!(rust.contains("GeneratedCompactModel for TinyResGenerated"));
    assert!(rust.contains("module_name: \"tiny_res\""));
    assert!(rust.contains("source_digest: \"sha256:fixture\""));
}
```

- [ ] **Step 2: Run the red test**

Run:

```powershell
cargo test -p rspice-veriloga --test codegen_core rust_emitter_generates_static_model_info_and_eval_function -- --nocapture
```

Expected: fail because `emit_rust_device` and `codegen::rust` do not exist yet.

- [ ] **Step 3: Implement the minimal emitter**

Create `crates/rspice-veriloga/src/codegen/rust.rs`:

```rust
use super::ir::{CodegenExpr, CodegenModel};

pub fn emit_rust_device(model: &CodegenModel) -> Result<String, String> {
    let struct_name = rust_struct_name(&model.module_name);
    let ports = string_slice(&model.ports.iter().map(|p| p.name.as_str()).collect::<Vec<_>>());
    let params = string_slice(&model.parameters.iter().map(|p| p.name.as_str()).collect::<Vec<_>>());
    let current_expr = model
        .contributions
        .first()
        .map(|c| emit_expr(&c.current, model))
        .transpose()?
        .unwrap_or_else(|| "0.0".to_string());
    let conductance_expr = model
        .contributions
        .first()
        .and_then(|c| c.conductance_hint.as_ref())
        .map(|expr| emit_expr(expr, model))
        .transpose()?
        .unwrap_or_else(|| "0.0".to_string());

    Ok(format!(
        r#"use rspice_core::device::{{
    GeneratedCompactModel, GeneratedContribution, GeneratedEvalInput, GeneratedModelInfo,
}};

pub struct {struct_name};

static INFO: GeneratedModelInfo = GeneratedModelInfo {{
    module_name: "{module}",
    source_package: "{package}",
    source_digest: "{digest}",
    ports: &{ports},
    parameters: &{params},
}};

impl GeneratedCompactModel for {struct_name} {{
    fn info(&self) -> &'static GeneratedModelInfo {{
        &INFO
    }}

    fn evaluate(&self, input: &GeneratedEvalInput<'_>) -> Vec<GeneratedContribution> {{
        vec![GeneratedContribution {{
            pos: 0,
            neg: 1,
            current: {current_expr},
            conductance: {conductance_expr},
        }}]
    }}
}}
"#,
        module = model.module_name,
        package = model.source_package,
        digest = model.source_digest,
    ))
}

fn rust_struct_name(module_name: &str) -> String {
    let mut out = String::new();
    let mut uppercase_next = true;
    for ch in module_name.chars() {
        if ch.is_ascii_alphanumeric() {
            if uppercase_next {
                out.push(ch.to_ascii_uppercase());
                uppercase_next = false;
            } else {
                out.push(ch);
            }
        } else {
            uppercase_next = true;
        }
    }
    out.push_str("Generated");
    out
}

fn string_slice(items: &[&str]) -> String {
    let joined = items
        .iter()
        .map(|item| format!("\"{item}\""))
        .collect::<Vec<_>>()
        .join(", ");
    format!("[{joined}]")
}

fn emit_expr(expr: &CodegenExpr, model: &CodegenModel) -> Result<String, String> {
    match expr {
        CodegenExpr::Const(value) => Ok(format!("{value:?}")),
        CodegenExpr::Parameter(name) => {
            let Some(index) = model.parameters.iter().position(|p| p.name == *name) else {
                return Err(format!("unknown parameter '{name}'"));
            };
            Ok(format!("input.parameters.get({index}).copied().unwrap_or({:?})", model.parameters[index].default))
        }
        CodegenExpr::Voltage { pos, neg } => {
            let Some(pos_index) = model.ports.iter().position(|p| p.name == *pos) else {
                return Err(format!("unknown voltage positive port '{pos}'"));
            };
            let Some(neg_index) = model.ports.iter().position(|p| p.name == *neg) else {
                return Err(format!("unknown voltage negative port '{neg}'"));
            };
            Ok(format!(
                "(input.voltages.get({pos_index}).copied().unwrap_or(0.0) - input.voltages.get({neg_index}).copied().unwrap_or(0.0))"
            ))
        }
        CodegenExpr::Add(a, b) => Ok(format!("({} + {})", emit_expr(a, model)?, emit_expr(b, model)?)),
        CodegenExpr::Sub(a, b) => Ok(format!("({} - {})", emit_expr(a, model)?, emit_expr(b, model)?)),
        CodegenExpr::Mul(a, b) => Ok(format!("({} * {})", emit_expr(a, model)?, emit_expr(b, model)?)),
        CodegenExpr::Div(a, b) => Ok(format!("({} / {})", emit_expr(a, model)?, emit_expr(b, model)?)),
        CodegenExpr::Exp(a) => Ok(format!("({}).exp()", emit_expr(a, model)?)),
    }
}
```

- [ ] **Step 4: Run the emitter test**

Run:

```powershell
cargo test -p rspice-veriloga --test codegen_core rust_emitter_generates_static_model_info_and_eval_function -- --nocapture
```

Expected: pass.

### Task 4: Lower A Tiny Verilog-A Fixture Into IR

**Files:**
- Create: `crates/rspice-veriloga/src/codegen/fixture.rs`
- Modify: `crates/rspice-veriloga/src/codegen/mod.rs`
- Modify: `crates/rspice-veriloga/tests/codegen_core.rs`

- [ ] **Step 1: Add a red fixture-lowering test**

Append to `crates/rspice-veriloga/tests/codegen_core.rs`:

```rust
#[test]
fn fixture_resistor_lowers_to_codegen_ir() {
    let source = r#"
`include "disciplines.vams"
module tiny_res(p, n);
  inout p, n;
  electrical p, n;
  parameter real g = 1e-3;
  analog begin
    I(p, n) <+ g * V(p, n);
  end
endmodule
"#;

    let model = rspice_veriloga::codegen::fixture::lower_tiny_resistor_fixture(source)
        .expect("lower tiny resistor");
    assert_eq!(model.module_name, "tiny_res");
    assert_eq!(model.ports.iter().map(|p| p.name.as_str()).collect::<Vec<_>>(), ["p", "n"]);
    assert_eq!(model.parameters[0].name, "g");
    assert_eq!(model.contributions.len(), 1);
}
```

- [ ] **Step 2: Run the red test**

Run:

```powershell
cargo test -p rspice-veriloga --test codegen_core fixture_resistor_lowers_to_codegen_ir -- --nocapture
```

Expected: fail because the fixture lowering helper does not exist.

- [ ] **Step 3: Implement the fixture helper**

Create `crates/rspice-veriloga/src/codegen/fixture.rs`:

```rust
use super::ir::{
    CodegenBranch, CodegenContribution, CodegenDomain, CodegenExpr, CodegenModel,
    CodegenParameter, CodegenPort,
};

pub fn lower_tiny_resistor_fixture(source: &str) -> Result<CodegenModel, String> {
    if !source.contains("module tiny_res") || !source.contains("I(p, n) <+ g * V(p, n)") {
        return Err("fixture helper only accepts the tiny_res smoke model".to_string());
    }

    Ok(CodegenModel {
        module_name: "tiny_res".to_string(),
        source_package: "fixture".to_string(),
        source_digest: format!("fixture-bytes:{}", source.len()),
        ports: vec![CodegenPort { name: "p".to_string() }, CodegenPort { name: "n".to_string() }],
        parameters: vec![CodegenParameter { name: "g".to_string(), default: 1.0e-3 }],
        branches: vec![CodegenBranch {
            name: "p_n".to_string(),
            pos: "p".to_string(),
            neg: "n".to_string(),
        }],
        contributions: vec![CodegenContribution {
            branch: CodegenBranch {
                name: "p_n".to_string(),
                pos: "p".to_string(),
                neg: "n".to_string(),
            },
            current: CodegenExpr::Mul(
                Box::new(CodegenExpr::Parameter("g".to_string())),
                Box::new(CodegenExpr::Voltage { pos: "p".to_string(), neg: "n".to_string() }),
            ),
            conductance_hint: Some(CodegenExpr::Parameter("g".to_string())),
            domain: CodegenDomain::Dc,
        }],
    })
}
```

Modify `crates/rspice-veriloga/src/codegen/mod.rs`:

```rust
#[cfg(test)]
pub mod fixture;
```

- [ ] **Step 4: Run the fixture test**

Run:

```powershell
cargo test -p rspice-veriloga --test codegen_core fixture_resistor_lowers_to_codegen_ir -- --nocapture
```

Expected: pass. This helper is intentionally only the first smoke fixture; later tasks replace it with real semantic lowering from the existing Verilog-A AST.

### Task 5: Verification

**Files:**
- Verify: `crates/rspice-core/src/device/generated.rs`
- Verify: `crates/rspice-veriloga/src/codegen/`
- Verify: `crates/rspice-veriloga/tests/codegen_core.rs`

- [ ] **Step 1: Run focused debug checks**

Run:

```powershell
cargo fmt --check --package rspice-core --package rspice-veriloga
cargo check -p rspice-core --tests
cargo check -p rspice-veriloga --tests
cargo test -p rspice-veriloga --test codegen_core -- --nocapture
```

Expected: all pass in debug profile.

- [ ] **Step 2: Do not run the full ngspice suite**

This codegen-core slice does not need the full ngspice regression suite. When that suite is explicitly required, run it only in `--release`.

---

## Self-Review

- Spec coverage: this plan creates the first generated-native path without starting any new hand-native CMC model implementation.
- Placeholder scan: no `TBD`, `TODO`, or vague edge-case placeholders remain.
- Type consistency: `GeneratedCompactModel`, `CodegenModel`, and `emit_rust_device` are named consistently across tasks.
