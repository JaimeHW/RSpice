# Verilog-A Rust Transpiler Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

> **Status update (2026-06-25):** This plan is historical. The generated
> built-in path now exists behind `veriloga-builtins`, but the implemented build
> integration materializes generated source under
> `crates/rspice-core/src/device/veriloga_generated/` instead of including an
> `OUT_DIR` registry. Use the crate READMEs and current source as the active
> contract.

**Goal:** Build the first production-grade Rust transpiler path that turns canonical Verilog-A IR into per-device generated Rust source and wires generated built-ins into the simulator build.

**Architecture:** `rspice-veriloga` owns the transpiler, deterministic source emission, diagnostics, and build-discovery helpers. `rspice-core` owns a small generated-device runtime ABI, an `OUT_DIR` registry inclusion point, and circuit-builder lookup. The first executable slice supports fast, analytic, allocation-free DC stamping for algebraic current contributions, then leaves hard compile-time diagnostics for unsupported constructs until later operator-specific tasks add them.

**Tech Stack:** Rust 2024, Cargo build scripts, canonical IR in `rspice-veriloga`, `OUT_DIR` generated modules, existing `rspice-core` nonlinear stamping callbacks, no Cranelift, no bytecode for generated built-ins.

---

## File Structure

- Create `crates/rspice-veriloga/src/rust_backend/mod.rs`: public Rust transpiler API.
- Create `crates/rspice-veriloga/src/rust_backend/error.rs`: phase-aware backend diagnostics.
- Create `crates/rspice-veriloga/src/rust_backend/names.rs`: deterministic folder/module/name mangling.
- Create `crates/rspice-veriloga/src/rust_backend/files.rs`: generated file model and filesystem writer.
- Create `crates/rspice-veriloga/src/rust_backend/discover.rs`: deterministic directory scan and module-bearing source discovery for build scripts.
- Create `crates/rspice-veriloga/src/rust_backend/expr.rs`: canonical expression to optimized Rust expression lowering.
- Create `crates/rspice-veriloga/src/rust_backend/device.rs`: per-device source generation.
- Modify `crates/rspice-veriloga/src/lib.rs`: export `rust_backend`.
- Create `crates/rspice-veriloga/tests/rust_backend.rs`: backend unit tests.
- Modify `crates/rspice-core/Cargo.toml`: add `build = "build.rs"`, `veriloga-builtins` feature, and build-dependency on `rspice-veriloga`.
- Create `crates/rspice-core/build.rs`: generate empty or real built-in registry into `OUT_DIR`.
- Create `crates/rspice-core/src/device/veriloga_generated/mod.rs`: generated-device runtime ABI and registry include.
- Modify `crates/rspice-core/src/device/mod.rs`: expose `veriloga_generated` behind `veriloga-builtins`.
- Modify `crates/rspice-core/src/circuit/mod.rs`, `construction.rs`, `external_models.rs`, and `nonlinear.rs`: store and stamp generated Verilog-A devices.
- Modify `crates/rspice-core/src/engine/builder.rs`: resolve generated built-ins before legacy external `.veriloga`.
- Create `crates/rspice-core/tests/generated_veriloga_builtins.rs`: end-to-end fixture test.
- Create `crates/rspice-core/tests/fixtures/veriloga_builtins/simple_res.va`: tiny build-script fixture.

---

### Task 1: Add Transpiler Public API And Diagnostics

**Files:**
- Create: `crates/rspice-veriloga/src/rust_backend/mod.rs`
- Create: `crates/rspice-veriloga/src/rust_backend/error.rs`
- Modify: `crates/rspice-veriloga/src/lib.rs`
- Test: `crates/rspice-veriloga/tests/rust_backend.rs`

- [ ] **Step 1: Write the failing API test**

Add this to `crates/rspice-veriloga/tests/rust_backend.rs`:

```rust
use rspice_veriloga::rust_backend::{GeneratedRustDevice, RustBackendError, RustTranspiler};

#[test]
fn rust_backend_public_api_exists() {
    let _ = RustTranspiler::default();
    let diagnostic = RustBackendError::unsupported("fixture.va", "tiny_res", "arrays");
    assert!(diagnostic.to_string().contains("fixture.va"));
    assert!(diagnostic.to_string().contains("tiny_res"));
    assert!(diagnostic.to_string().contains("arrays"));
}

#[test]
fn generated_device_records_multiple_files() {
    let device = GeneratedRustDevice {
        module_name: "tiny_res".to_string(),
        public_model_name: "tiny_res".to_string(),
        folder_name: "tiny_res__tiny_res__00000000".to_string(),
        files: Vec::new(),
        source_digest: "0000000000000000".to_string(),
    };

    assert_eq!(device.module_name, "tiny_res");
    assert!(device.files.is_empty());
}
```

- [ ] **Step 2: Run the test to verify it fails**

Run:

```powershell
cargo test -p rspice-veriloga --test rust_backend rust_backend_public_api_exists -- --nocapture
```

Expected: fail because `rspice_veriloga::rust_backend` does not exist.

- [ ] **Step 3: Add the API module**

Create `crates/rspice-veriloga/src/rust_backend/error.rs`:

```rust
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RustBackendError {
    pub source: String,
    pub module: String,
    pub message: String,
}

impl RustBackendError {
    pub fn unsupported(
        source: impl Into<String>,
        module: impl Into<String>,
        feature: impl Into<String>,
    ) -> Self {
        Self {
            source: source.into(),
            module: module.into(),
            message: format!("unsupported Verilog-A construct for Rust backend: {}", feature.into()),
        }
    }

    pub fn internal(
        source: impl Into<String>,
        module: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            source: source.into(),
            module: module.into(),
            message: message.into(),
        }
    }
}

impl fmt::Display for RustBackendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Rust backend error in {} module {}: {}",
            self.source, self.module, self.message
        )
    }
}

impl std::error::Error for RustBackendError {}
```

Create `crates/rspice-veriloga/src/rust_backend/mod.rs`:

```rust
mod error;

pub use error::RustBackendError;

use crate::canonical_ir::CanonicalIrArtifact;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedRustFile {
    pub relative_path: String,
    pub contents: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedRustDevice {
    pub module_name: String,
    pub public_model_name: String,
    pub folder_name: String,
    pub files: Vec<GeneratedRustFile>,
    pub source_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RustTranspileOptions {
    pub runtime_path: String,
}

impl Default for RustTranspileOptions {
    fn default() -> Self {
        Self {
            runtime_path: "crate::device::veriloga_generated".to_string(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct RustTranspiler {
    options: RustTranspileOptions,
}

impl RustTranspiler {
    pub fn new(options: RustTranspileOptions) -> Self {
        Self { options }
    }

    pub fn transpile(
        &self,
        artifact: &CanonicalIrArtifact,
    ) -> Result<GeneratedRustDevice, RustBackendError> {
        let _ = (&self.options, artifact);
        Err(RustBackendError::unsupported(
            artifact.metadata.source_package.as_str(),
            artifact.mir.module_name.as_str(),
            "device lowering is not implemented yet",
        ))
    }
}
```

Modify `crates/rspice-veriloga/src/lib.rs`:

```rust
pub mod rust_backend;
```

Place it near the other public modules, after `pub mod preprocessor;`.

- [ ] **Step 4: Run the test to verify it passes**

Run:

```powershell
cargo test -p rspice-veriloga --test rust_backend rust_backend_public_api_exists generated_device_records_multiple_files -- --nocapture
```

Expected: both tests pass.

- [ ] **Step 5: Commit**

```powershell
git add crates/rspice-veriloga/src/lib.rs crates/rspice-veriloga/src/rust_backend crates/rspice-veriloga/tests/rust_backend.rs
git commit -m "feat(veriloga): add rust backend api"
```

---

### Task 2: Deterministic Name Mangling And Per-Device File Model

**Files:**
- Create: `crates/rspice-veriloga/src/rust_backend/names.rs`
- Create: `crates/rspice-veriloga/src/rust_backend/files.rs`
- Modify: `crates/rspice-veriloga/src/rust_backend/mod.rs`
- Test: `crates/rspice-veriloga/tests/rust_backend.rs`

- [ ] **Step 1: Write failing tests for commercial-safe output names**

Append:

```rust
use rspice_veriloga::rust_backend::{
    GeneratedRustFile, RustDeviceNames, write_generated_device,
};

#[test]
fn rust_backend_mangles_names_deterministically() {
    let names = RustDeviceNames::new("psp103.va", "PSP103_Module", "abcdef0123456789");

    assert_eq!(names.public_model_name, "PSP103_Module");
    assert_eq!(names.rust_module, "psp103_module__abcdef01");
    assert_eq!(names.folder, "psp103__psp103_module__abcdef01");
}

#[test]
fn generated_device_writer_splits_files_under_device_folder() {
    let temp = std::env::temp_dir().join(format!(
        "rspice-rust-backend-files-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos()
    ));

    let device = GeneratedRustDevice {
        module_name: "tiny_res".to_string(),
        public_model_name: "tiny_res".to_string(),
        folder_name: "tiny_res__tiny_res__abc12345".to_string(),
        source_digest: "abc1234567890000".to_string(),
        files: vec![
            GeneratedRustFile {
                relative_path: "mod.rs".to_string(),
                contents: "pub mod eval;\n".to_string(),
            },
            GeneratedRustFile {
                relative_path: "eval.rs".to_string(),
                contents: "pub fn marker() -> f64 { 1.0 }\n".to_string(),
            },
        ],
    };

    write_generated_device(&temp, &device).expect("write generated device");
    assert!(temp.join("tiny_res__tiny_res__abc12345").join("mod.rs").exists());
    assert!(temp.join("tiny_res__tiny_res__abc12345").join("eval.rs").exists());

    let _ = std::fs::remove_dir_all(temp);
}
```

- [ ] **Step 2: Run the tests to verify they fail**

Run:

```powershell
cargo test -p rspice-veriloga --test rust_backend rust_backend_mangles_names_deterministically generated_device_writer_splits_files_under_device_folder -- --nocapture
```

Expected: fail because `RustDeviceNames` and `write_generated_device` do not exist.

- [ ] **Step 3: Implement deterministic names and file writing**

Create `crates/rspice-veriloga/src/rust_backend/names.rs`:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RustDeviceNames {
    pub public_model_name: String,
    pub rust_module: String,
    pub folder: String,
}

impl RustDeviceNames {
    pub fn new(source_file_name: &str, module_name: &str, digest: &str) -> Self {
        let source_stem = source_file_name
            .rsplit(['/', '\\'])
            .next()
            .unwrap_or(source_file_name)
            .split('.')
            .next()
            .unwrap_or(source_file_name);
        let digest = digest.chars().take(8).collect::<String>();
        let source = sanitize_identifier(source_stem);
        let module = sanitize_identifier(module_name);

        Self {
            public_model_name: module_name.to_string(),
            rust_module: format!("{module}__{digest}"),
            folder: format!("{source}__{module}__{digest}"),
        }
    }
}

pub fn sanitize_identifier(input: &str) -> String {
    let mut out = String::with_capacity(input.len().max(1));
    for (index, ch) in input.chars().enumerate() {
        let valid = ch == '_' || ch.is_ascii_alphanumeric();
        if valid {
            if index == 0 && ch.is_ascii_digit() {
                out.push('_');
            }
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push('_');
        }
    }
    let out = out.trim_matches('_').to_string();
    if out.is_empty() { "_model".to_string() } else { out }
}
```

Create `crates/rspice-veriloga/src/rust_backend/files.rs`:

```rust
use std::path::{Path, PathBuf};

use super::{GeneratedRustDevice, RustBackendError};

pub fn write_generated_device(
    root: impl AsRef<Path>,
    device: &GeneratedRustDevice,
) -> Result<Vec<PathBuf>, RustBackendError> {
    let root = root.as_ref();
    let device_dir = root.join(&device.folder_name);
    std::fs::create_dir_all(&device_dir).map_err(|error| {
        RustBackendError::internal(
            "<generated>",
            &device.module_name,
            format!("failed to create generated device directory '{}': {error}", device_dir.display()),
        )
    })?;

    let mut written = Vec::with_capacity(device.files.len());
    for file in &device.files {
        if file.relative_path.contains("..") || file.relative_path.contains('\\') {
            return Err(RustBackendError::internal(
                "<generated>",
                &device.module_name,
                format!("unsafe generated relative path '{}'", file.relative_path),
            ));
        }

        let path = device_dir.join(&file.relative_path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|error| {
                RustBackendError::internal(
                    "<generated>",
                    &device.module_name,
                    format!("failed to create generated file parent '{}': {error}", parent.display()),
                )
            })?;
        }
        std::fs::write(&path, &file.contents).map_err(|error| {
            RustBackendError::internal(
                "<generated>",
                &device.module_name,
                format!("failed to write generated file '{}': {error}", path.display()),
            )
        })?;
        written.push(path);
    }

    Ok(written)
}
```

Modify `crates/rspice-veriloga/src/rust_backend/mod.rs`:

```rust
mod files;
mod names;

pub use files::write_generated_device;
pub use names::{RustDeviceNames, sanitize_identifier};
```

- [ ] **Step 4: Run the focused tests**

Run:

```powershell
cargo test -p rspice-veriloga --test rust_backend -- --nocapture
```

Expected: all Rust backend tests pass.

- [ ] **Step 5: Commit**

```powershell
git add crates/rspice-veriloga/src/rust_backend crates/rspice-veriloga/tests/rust_backend.rs
git commit -m "feat(veriloga): add deterministic rust backend files"
```

---

### Task 3: Directory Discovery For Build Scripts

**Files:**
- Create: `crates/rspice-veriloga/src/rust_backend/discover.rs`
- Modify: `crates/rspice-veriloga/src/rust_backend/mod.rs`
- Test: `crates/rspice-veriloga/tests/rust_backend.rs`

- [ ] **Step 1: Write failing discovery tests**

Append:

```rust
use rspice_veriloga::rust_backend::discover_veriloga_sources;

fn temp_dir(prefix: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "{prefix}-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos()
    ));
    std::fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

#[test]
fn discovery_skips_include_only_files_and_sorts_modules() {
    let dir = temp_dir("rspice-va-discovery");
    std::fs::write(dir.join("defs.include"), "`define GAIN 1.0\n").expect("write include");
    std::fs::write(dir.join("disciplines.vams"), "nature Voltage; endnature\n").expect("write vams");
    std::fs::write(
        dir.join("b.va"),
        "module beta(p,n); inout p,n; electrical p,n; analog I(p,n)<+V(p,n); endmodule\n",
    )
    .expect("write beta");
    std::fs::write(
        dir.join("a.va"),
        "module alpha(p,n); inout p,n; electrical p,n; analog I(p,n)<+V(p,n); endmodule\n",
    )
    .expect("write alpha");

    let found = discover_veriloga_sources(&dir).expect("discover sources");
    let names: Vec<_> = found
        .iter()
        .flat_map(|source| source.modules.iter().cloned())
        .collect();

    assert_eq!(names, vec!["alpha".to_string(), "beta".to_string()]);
    assert_eq!(found.len(), 2);

    let _ = std::fs::remove_dir_all(dir);
}
```

- [ ] **Step 2: Run the test to verify it fails**

Run:

```powershell
cargo test -p rspice-veriloga --test rust_backend discovery_skips_include_only_files_and_sorts_modules -- --nocapture
```

Expected: fail because discovery API does not exist.

- [ ] **Step 3: Implement scanner and module detector**

Create `crates/rspice-veriloga/src/rust_backend/discover.rs`:

```rust
use std::path::{Path, PathBuf};

use super::RustBackendError;
use crate::{Lexer, Parser, SourceMap};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerilogASourceCandidate {
    pub path: PathBuf,
    pub modules: Vec<String>,
}

pub fn discover_veriloga_sources(
    root: impl AsRef<Path>,
) -> Result<Vec<VerilogASourceCandidate>, RustBackendError> {
    let root = root.as_ref();
    let mut files = Vec::new();
    collect_va_files(root, &mut files)?;
    files.sort();

    let mut candidates = Vec::new();
    for path in files {
        let source = std::fs::read_to_string(&path).map_err(|error| {
            RustBackendError::internal(
                path.display().to_string(),
                "<scan>",
                format!("failed to read candidate: {error}"),
            )
        })?;
        let modules = module_names_in_source(&path, &source)?;
        if !modules.is_empty() {
            candidates.push(VerilogASourceCandidate { path, modules });
        }
    }

    Ok(candidates)
}

fn collect_va_files(root: &Path, files: &mut Vec<PathBuf>) -> Result<(), RustBackendError> {
    for entry in std::fs::read_dir(root).map_err(|error| {
        RustBackendError::internal(
            root.display().to_string(),
            "<scan>",
            format!("failed to read directory: {error}"),
        )
    })? {
        let entry = entry.map_err(|error| {
            RustBackendError::internal(
                root.display().to_string(),
                "<scan>",
                format!("failed to read directory entry: {error}"),
            )
        })?;
        let path = entry.path();
        let file_type = entry.file_type().map_err(|error| {
            RustBackendError::internal(
                path.display().to_string(),
                "<scan>",
                format!("failed to read file type: {error}"),
            )
        })?;

        if file_type.is_dir() {
            collect_va_files(&path, files)?;
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("va") {
            files.push(path.canonicalize().unwrap_or(path));
        }
    }
    Ok(())
}

fn module_names_in_source(path: &Path, source: &str) -> Result<Vec<String>, RustBackendError> {
    let mut source_map = SourceMap::new();
    let source_id = source_map.add_source_mut(path.display().to_string(), source);
    let tokens = Lexer::new(source, source_id).collect_tokens().map_err(|error| {
        RustBackendError::internal(path.display().to_string(), "<scan>", error.to_string())
    })?;
    let parsed = Parser::new(&tokens).parse().map_err(|error| {
        RustBackendError::internal(path.display().to_string(), "<scan>", error.to_string())
    })?;

    let mut modules = parsed
        .items
        .iter()
        .filter_map(|item| match item {
            crate::ast::Item::Module(module) => Some(module.name.to_string()),
            _ => None,
        })
        .collect::<Vec<_>>();
    modules.sort();
    Ok(modules)
}
```

Modify `crates/rspice-veriloga/src/rust_backend/mod.rs`:

```rust
mod discover;
pub use discover::{VerilogASourceCandidate, discover_veriloga_sources};
```

- [ ] **Step 4: Run discovery tests**

Run:

```powershell
cargo test -p rspice-veriloga --test rust_backend discovery_skips_include_only_files_and_sorts_modules -- --nocapture
```

Expected: pass.

- [ ] **Step 5: Commit**

```powershell
git add crates/rspice-veriloga/src/rust_backend crates/rspice-veriloga/tests/rust_backend.rs
git commit -m "feat(veriloga): discover rust backend source roots"
```

---

### Task 4: Generate Optimized Rust For Simple Algebraic Current Devices

**Files:**
- Create: `crates/rspice-veriloga/src/rust_backend/expr.rs`
- Create: `crates/rspice-veriloga/src/rust_backend/device.rs`
- Modify: `crates/rspice-veriloga/src/rust_backend/mod.rs`
- Test: `crates/rspice-veriloga/tests/rust_backend.rs`

- [ ] **Step 1: Write failing transpiler test**

Append:

```rust
use rspice_veriloga::{VerilogACompiler};

const TINY_RES: &str = r#"
module tiny_res(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 1000.0 from (0:inf);
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#;

#[test]
fn transpiler_emits_split_rust_files_for_tiny_resistor() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(TINY_RES)
        .expect("canonical IR");
    let generated = RustTranspiler::default()
        .transpile(&artifact)
        .expect("transpile tiny resistor");

    let file_names: Vec<_> = generated
        .files
        .iter()
        .map(|file| file.relative_path.as_str())
        .collect();
    assert!(file_names.contains(&"mod.rs"));
    assert!(file_names.contains(&"metadata.rs"));
    assert!(file_names.contains(&"params.rs"));
    assert!(file_names.contains(&"state.rs"));
    assert!(file_names.contains(&"eval.rs"));
    assert!(file_names.contains(&"stamp.rs"));

    let stamp = generated
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp file");
    assert!(stamp.contents.contains("let v_0 = ctx.voltage(0, 1);"));
    assert!(stamp.contents.contains("let i_0 = v_0 / self.params.r;"));
    assert!(stamp.contents.contains("let g_0_0 = 1.0 / self.params.r;"));
}
```

- [ ] **Step 2: Run the test to verify it fails**

Run:

```powershell
cargo test -p rspice-veriloga --test rust_backend transpiler_emits_split_rust_files_for_tiny_resistor -- --nocapture
```

Expected: fail because `transpile` still returns unsupported.

- [ ] **Step 3: Implement expression lowering with hard unsupported diagnostics**

Create `crates/rspice-veriloga/src/rust_backend/expr.rs`:

```rust
use crate::canonical_ir::{HirExprKind, HirExpression, MirModel};

use super::RustBackendError;

#[derive(Debug, Clone, PartialEq)]
pub struct LoweredExpr {
    pub code: String,
    pub derivative_by_node: Vec<String>,
}

pub fn lower_expr(mir: &MirModel, expr_id: usize) -> Result<LoweredExpr, RustBackendError> {
    let node_count = mir.nodes.len();
    let expr = mir.expressions.get(expr_id).ok_or_else(|| {
        RustBackendError::internal(
            mir.module_name.as_str(),
            mir.module_name.as_str(),
            format!("expression {expr_id} out of range"),
        )
    })?;
    lower_expr_inner(mir, expr, node_count)
}

fn zero_derivatives(node_count: usize) -> Vec<String> {
    vec!["0.0".to_string(); node_count]
}

fn lower_expr_inner(
    mir: &MirModel,
    expr: &HirExpression,
    node_count: usize,
) -> Result<LoweredExpr, RustBackendError> {
    match &expr.kind {
        HirExprKind::Number { value, .. } => Ok(LoweredExpr {
            code: f64_literal(*value),
            derivative_by_node: zero_derivatives(node_count),
        }),
        HirExprKind::Identifier { name } => {
            let param = mir.parameters.iter().find(|param| param.name == *name).ok_or_else(|| {
                RustBackendError::unsupported(
                    mir.module_name.as_str(),
                    mir.module_name.as_str(),
                    format!("identifier '{name}' is not a parameter"),
                )
            })?;
            Ok(LoweredExpr {
                code: format!("self.params.{}", crate::rust_backend::sanitize_identifier(param.name.as_str())),
                derivative_by_node: zero_derivatives(node_count),
            })
        }
        HirExprKind::BranchAccess { access, pos, neg } if access.as_str() == "V" => {
            let pos_id = node_index(mir, pos)?;
            let neg_id = neg.as_ref().map(|name| node_index(mir, name)).transpose()?;
            let neg_code = neg_id.map_or("usize::MAX".to_string(), |id| id.to_string());
            let mut derivative = zero_derivatives(node_count);
            derivative[pos_id] = "1.0".to_string();
            if let Some(neg_id) = neg_id {
                derivative[neg_id] = "-1.0".to_string();
            }
            Ok(LoweredExpr {
                code: format!("ctx.voltage({pos_id}, {neg_code})"),
                derivative_by_node: derivative,
            })
        }
        HirExprKind::Binary { op, left, right } => {
            let left = lower_expr(mir, usize::from(*left))?;
            let right = lower_expr(mir, usize::from(*right))?;
            lower_binary(mir, op.as_str(), left, right)
        }
        HirExprKind::Unary { op, operand } if op.as_str() == "Neg" => {
            let operand = lower_expr(mir, usize::from(*operand))?;
            Ok(LoweredExpr {
                code: format!("-({})", operand.code),
                derivative_by_node: operand
                    .derivative_by_node
                    .into_iter()
                    .map(|d| format!("-({d})"))
                    .collect(),
            })
        }
        HirExprKind::AnalogOperator { op: crate::canonical_ir::HirAnalogOperator::Limexp { expr } } => {
            let inner = lower_expr(mir, usize::from(*expr))?;
            Ok(LoweredExpr {
                code: format!("limexp({})", inner.code),
                derivative_by_node: inner
                    .derivative_by_node
                    .into_iter()
                    .map(|d| format!("limexp_derivative({}) * ({d})", inner.code))
                    .collect(),
            })
        }
        other => Err(RustBackendError::unsupported(
            mir.metadata_source_for_error(),
            mir.module_name.as_str(),
            format!("expression kind {other:?}"),
        )),
    }
}

fn lower_binary(
    mir: &MirModel,
    op: &str,
    left: LoweredExpr,
    right: LoweredExpr,
) -> Result<LoweredExpr, RustBackendError> {
    let (code, derivative_by_node) = match op {
        "Add" => (
            format!("({}) + ({})", left.code, right.code),
            left.derivative_by_node
                .iter()
                .zip(right.derivative_by_node.iter())
                .map(|(l, r)| format!("({l}) + ({r})"))
                .collect(),
        ),
        "Sub" => (
            format!("({}) - ({})", left.code, right.code),
            left.derivative_by_node
                .iter()
                .zip(right.derivative_by_node.iter())
                .map(|(l, r)| format!("({l}) - ({r})"))
                .collect(),
        ),
        "Mul" => (
            format!("({}) * ({})", left.code, right.code),
            left.derivative_by_node
                .iter()
                .zip(right.derivative_by_node.iter())
                .map(|(l, r)| format!("({l}) * ({}) + ({}) * ({r})", right.code, left.code))
                .collect(),
        ),
        "Div" => (
            format!("({}) / ({})", left.code, right.code),
            left.derivative_by_node
                .iter()
                .zip(right.derivative_by_node.iter())
                .map(|(l, r)| {
                    format!("(({l}) * ({}) - ({}) * ({r})) / (({}) * ({}))", right.code, left.code, right.code, right.code)
                })
                .collect(),
        ),
        _ => {
            return Err(RustBackendError::unsupported(
                mir.module_name.as_str(),
                mir.module_name.as_str(),
                format!("binary operator {op}"),
            ));
        }
    };

    Ok(LoweredExpr { code, derivative_by_node })
}

fn node_index(mir: &MirModel, name: &str) -> Result<usize, RustBackendError> {
    if name == "0" || mir.ground_nodes.iter().any(|ground| ground.as_str() == name) {
        return Ok(usize::MAX);
    }
    mir.nodes
        .iter()
        .position(|node| node.name.as_str() == name)
        .ok_or_else(|| {
            RustBackendError::internal(
                mir.module_name.as_str(),
                mir.module_name.as_str(),
                format!("unknown node '{name}'"),
            )
        })
}

fn f64_literal(value: f64) -> String {
    if value == f64::INFINITY {
        "f64::INFINITY".to_string()
    } else if value == f64::NEG_INFINITY {
        "f64::NEG_INFINITY".to_string()
    } else {
        format!("{value:?}")
    }
}

trait MirErrorSource {
    fn metadata_source_for_error(&self) -> &str;
}

impl MirErrorSource for MirModel {
    fn metadata_source_for_error(&self) -> &str {
        self.module_name.as_str()
    }
}
```

Adjust this implementation while coding if exact `HirExprKind::Binary` operator labels differ; preserve the test's requirement that a resistor emits value and analytic conductance code.

- [ ] **Step 4: Implement device file generation**

Create `crates/rspice-veriloga/src/rust_backend/device.rs` with generator functions:

```rust
use super::{GeneratedRustDevice, GeneratedRustFile, RustBackendError, RustDeviceNames};
use crate::canonical_ir::{CanonicalIrArtifact, MirEquationKind};

pub fn generate_device(
    artifact: &CanonicalIrArtifact,
    runtime_path: &str,
) -> Result<GeneratedRustDevice, RustBackendError> {
    artifact.validate().map_err(|diagnostics| {
        RustBackendError::internal(
            artifact.metadata.source_package.as_str(),
            artifact.mir.module_name.as_str(),
            diagnostics
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("; "),
        )
    })?;

    if artifact.mir.state_slots.len() > 0 {
        return Err(RustBackendError::unsupported(
            artifact.metadata.source_package.as_str(),
            artifact.mir.module_name.as_str(),
            "stateful operators in first Rust backend slice",
        ));
    }

    if artifact.mir.equations.iter().any(|eq| eq.kind != MirEquationKind::Current) {
        return Err(RustBackendError::unsupported(
            artifact.metadata.source_package.as_str(),
            artifact.mir.module_name.as_str(),
            "potential or indirect contributions in first Rust backend slice",
        ));
    }

    let source_name = artifact
        .metadata
        .source_package
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or(artifact.metadata.source_package.as_str());
    let names = RustDeviceNames::new(
        source_name,
        artifact.mir.module_name.as_str(),
        artifact.mir_digest.as_str(),
    );

    let params = emit_params(artifact);
    let stamp = emit_stamp(artifact, runtime_path)?;

    Ok(GeneratedRustDevice {
        module_name: artifact.mir.module_name.to_string(),
        public_model_name: names.public_model_name,
        folder_name: names.folder,
        source_digest: artifact.metadata.source_digest.to_string(),
        files: vec![
            GeneratedRustFile { relative_path: "mod.rs".to_string(), contents: emit_mod() },
            GeneratedRustFile { relative_path: "metadata.rs".to_string(), contents: emit_metadata(artifact) },
            GeneratedRustFile { relative_path: "params.rs".to_string(), contents: params },
            GeneratedRustFile { relative_path: "state.rs".to_string(), contents: emit_state() },
            GeneratedRustFile { relative_path: "eval.rs".to_string(), contents: emit_eval_helpers() },
            GeneratedRustFile { relative_path: "stamp.rs".to_string(), contents: stamp },
        ],
    })
}

fn emit_mod() -> String {
    [
        "pub mod metadata;\n",
        "pub mod params;\n",
        "pub mod state;\n",
        "pub mod eval;\n",
        "pub mod stamp;\n",
        "pub use params::Params;\n",
        "pub use state::Instance;\n",
    ].concat()
}

fn emit_metadata(artifact: &CanonicalIrArtifact) -> String {
    format!(
        "pub const MODEL_NAME: &str = {:?};\npub const SOURCE_DIGEST: &str = {:?};\npub const TERMINALS: &[&str] = &{:?};\n",
        artifact.mir.module_name.as_str(),
        artifact.metadata.source_digest.as_str(),
        artifact.mir.nodes.iter().filter(|n| n.is_external).map(|n| n.name.to_string()).collect::<Vec<_>>()
    )
}

fn emit_params(artifact: &CanonicalIrArtifact) -> String {
    let fields = artifact.mir.parameters.iter().map(|param| {
        format!("    pub {}: f64,\n", crate::rust_backend::sanitize_identifier(param.name.as_str()))
    }).collect::<String>();
    let defaults = artifact.mir.parameters.iter().map(|param| {
        format!(
            "            {}: {:?},\n",
            crate::rust_backend::sanitize_identifier(param.name.as_str()),
            param.default.unwrap_or(0.0)
        )
    }).collect::<String>();
    format!(
        "#[derive(Debug, Clone, Copy)]\npub struct Params {{\n{fields}}}\n\nimpl Default for Params {{\n    fn default() -> Self {{\n        Self {{\n{defaults}        }}\n    }}\n}}\n"
    )
}

fn emit_state() -> String {
    "#[derive(Debug, Clone)]\npub struct Instance {\n    pub params: super::params::Params,\n    pub nodes: Vec<usize>,\n}\n\nimpl Instance {\n    pub fn new(nodes: &[usize]) -> Self {\n        Self { params: super::params::Params::default(), nodes: nodes.to_vec() }\n    }\n}\n".to_string()
}

fn emit_eval_helpers() -> String {
    "#[inline]\npub fn limexp(x: f64) -> f64 { if x > 80.0 { (80.0_f64).exp() * (x - 79.0) } else { x.exp() } }\n#[inline]\npub fn limexp_derivative(x: f64) -> f64 { if x > 80.0 { (80.0_f64).exp() } else { x.exp() } }\n".to_string()
}
```

Then implement `emit_stamp` using `super::expr::lower_expr` and the same companion model formula used by the legacy Verilog-A device:

```rust
fn emit_stamp(
    artifact: &CanonicalIrArtifact,
    runtime_path: &str,
) -> Result<String, RustBackendError> {
    let mut body = String::new();
    body.push_str("use super::eval::{limexp, limexp_derivative};\n");
    body.push_str(&format!("use {runtime_path}::{{GeneratedEvalContext, GeneratedStamper}};\n\n"));
    body.push_str("impl super::state::Instance {\n");
    body.push_str("    #[inline]\n");
    body.push_str("    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {\n");

    for (equation_index, equation) in artifact.mir.equations.iter().enumerate() {
        let lowered = super::expr::lower_expr(&artifact.mir, usize::from(equation.expression.id))?;
        body.push_str(&format!("        let v_{equation_index} = {};\n", branch_voltage_code(equation)));
        body.push_str(&format!("        let i_{equation_index} = {};\n", lowered.code));
        body.push_str(&format!("        let mut eq_{equation_index} = i_{equation_index};\n"));
        for (node_index, derivative) in lowered.derivative_by_node.iter().enumerate() {
            body.push_str(&format!("        let g_{equation_index}_{node_index} = {derivative};\n"));
            body.push_str(&format!("        eq_{equation_index} -= g_{equation_index}_{node_index} * ctx.node_voltage({node_index});\n"));
        }
        let pos = equation.branch.pos_node.map(|id| id.index()).unwrap_or(u32::MAX);
        let neg = equation.branch.neg_node.map(|id| id.index()).unwrap_or(u32::MAX);
        for (node_index, _node) in artifact.mir.nodes.iter().enumerate() {
            body.push_str(&format!("        stamp_current_jacobian(stamper, {pos}, {neg}, {node_index}, g_{equation_index}_{node_index});\n"));
        }
        body.push_str(&format!("        stamp_current_rhs(stamper, {pos}, {neg}, eq_{equation_index});\n"));
    }

    body.push_str("    }\n}\n\n");
    body.push_str("#[inline]\nfn stamp_current_jacobian(stamper: &mut GeneratedStamper<'_>, pos: u32, neg: u32, col: usize, g: f64) {\n");
    body.push_str("    if pos != u32::MAX { stamper.matrix(pos as usize, col, g); }\n");
    body.push_str("    if neg != u32::MAX { stamper.matrix(neg as usize, col, -g); }\n");
    body.push_str("}\n\n");
    body.push_str("#[inline]\nfn stamp_current_rhs(stamper: &mut GeneratedStamper<'_>, pos: u32, neg: u32, value: f64) {\n");
    body.push_str("    if pos != u32::MAX { stamper.rhs(pos as usize, -value); }\n");
    body.push_str("    if neg != u32::MAX { stamper.rhs(neg as usize, value); }\n");
    body.push_str("}\n");
    Ok(body)
}

fn branch_voltage_code(equation: &crate::canonical_ir::MirEquation) -> String {
    let pos = equation.branch.pos_node.map(|id| id.index()).unwrap_or(u32::MAX);
    let neg = equation.branch.neg_node.map(|id| id.index()).unwrap_or(u32::MAX);
    let pos = if pos == u32::MAX { "usize::MAX".to_string() } else { pos.to_string() };
    let neg = if neg == u32::MAX { "usize::MAX".to_string() } else { neg.to_string() };
    format!("ctx.voltage({pos}, {neg})")
}
```

Wire `RustTranspiler::transpile` to call `device::generate_device`.

- [ ] **Step 5: Run tests and fix operator labels only if needed**

Run:

```powershell
cargo test -p rspice-veriloga --test rust_backend transpiler_emits_split_rust_files_for_tiny_resistor -- --nocapture
```

Expected: pass after aligning exact binary operator labels with canonical HIR.

- [ ] **Step 6: Commit**

```powershell
git add crates/rspice-veriloga/src/rust_backend crates/rspice-veriloga/tests/rust_backend.rs
git commit -m "feat(veriloga): generate rust for algebraic current devices"
```

---

### Task 5: Add Core Generated-Device Runtime ABI

**Files:**
- Create: `crates/rspice-core/src/device/veriloga_generated/mod.rs`
- Modify: `crates/rspice-core/src/device/mod.rs`
- Test: `crates/rspice-core/src/device/veriloga_generated/mod.rs`

- [ ] **Step 1: Write runtime ABI unit tests**

Create `crates/rspice-core/src/device/veriloga_generated/mod.rs` with the test module first:

```rust
use crate::Value;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_context_maps_ground_to_zero() {
        let solution = [1.5, -0.25];
        let nodes = [1, 0];
        let ctx = GeneratedEvalContext::new(&solution, &nodes, &[], &[]);

        assert_eq!(ctx.node_voltage(0), 1.5);
        assert_eq!(ctx.node_voltage(usize::MAX), 0.0);
        assert_eq!(ctx.voltage(0, usize::MAX), 1.5);
    }

    #[test]
    fn stamper_maps_generated_node_indexes_to_circuit_rows() {
        let mut matrix = Vec::new();
        let mut rhs = Vec::new();
        let nodes = [2, 0];
        let mut stamper = GeneratedStamper::new(
            &nodes,
            &[],
            &[],
            |row, col, value| matrix.push((row, col, value)),
            |row, value| rhs.push((row, value)),
        );

        stamper.matrix(0, 0, 2.0);
        stamper.matrix(0, 1, -2.0);
        stamper.rhs(0, -0.25);
        stamper.rhs(1, 0.25);

        assert_eq!(matrix, vec![(1, 1, 2.0)]);
        assert_eq!(rhs, vec![(1, -0.25)]);
    }
}
```

- [ ] **Step 2: Run the tests to verify they fail**

Run:

```powershell
cargo test -p rspice-core --features veriloga-builtins --lib veriloga_generated -- --nocapture
```

Expected: fail because the feature/module does not exist.

- [ ] **Step 3: Add feature and module export**

Modify `crates/rspice-core/Cargo.toml`:

```toml
[features]
veriloga-builtins = ["veriloga"]
```

Modify `crates/rspice-core/src/device/mod.rs`:

```rust
#[cfg(feature = "veriloga-builtins")]
pub mod veriloga_generated;
```

- [ ] **Step 4: Implement ABI types**

Fill `crates/rspice-core/src/device/veriloga_generated/mod.rs` above the tests:

```rust
use crate::Value;

pub struct GeneratedEvalContext<'a> {
    solution: &'a [Value],
    nodes: &'a [usize],
    internal_nodes: &'a [usize],
    branch_nodes: &'a [usize],
}

impl<'a> GeneratedEvalContext<'a> {
    pub fn new(
        solution: &'a [Value],
        nodes: &'a [usize],
        internal_nodes: &'a [usize],
        branch_nodes: &'a [usize],
    ) -> Self {
        Self {
            solution,
            nodes,
            internal_nodes,
            branch_nodes,
        }
    }

    #[inline]
    pub fn node_voltage(&self, generated_node: usize) -> Value {
        if generated_node == usize::MAX {
            return 0.0;
        }
        let circuit_node = if generated_node < self.nodes.len() {
            self.nodes[generated_node]
        } else {
            self.internal_nodes
                .get(generated_node - self.nodes.len())
                .copied()
                .unwrap_or(0)
        };
        if circuit_node == 0 {
            0.0
        } else {
            self.solution.get(circuit_node - 1).copied().unwrap_or(0.0)
        }
    }

    #[inline]
    pub fn voltage(&self, pos: usize, neg: usize) -> Value {
        self.node_voltage(pos) - self.node_voltage(neg)
    }

    #[inline]
    pub fn branch_current(&self, branch: usize) -> Value {
        let circuit_node = self.branch_nodes.get(branch).copied().unwrap_or(0);
        if circuit_node == 0 {
            0.0
        } else {
            self.solution.get(circuit_node - 1).copied().unwrap_or(0.0)
        }
    }
}

pub struct GeneratedStamper<'a> {
    nodes: &'a [usize],
    internal_nodes: &'a [usize],
    branch_nodes: &'a [usize],
    matrix_add: Box<dyn FnMut(usize, usize, Value) + 'a>,
    rhs_add: Box<dyn FnMut(usize, Value) + 'a>,
}

impl<'a> GeneratedStamper<'a> {
    pub fn new<M, R>(
        nodes: &'a [usize],
        internal_nodes: &'a [usize],
        branch_nodes: &'a [usize],
        matrix_add: M,
        rhs_add: R,
    ) -> Self
    where
        M: FnMut(usize, usize, Value) + 'a,
        R: FnMut(usize, Value) + 'a,
    {
        Self {
            nodes,
            internal_nodes,
            branch_nodes,
            matrix_add: Box::new(matrix_add),
            rhs_add: Box::new(rhs_add),
        }
    }

    #[inline]
    pub fn matrix(&mut self, row: usize, col: usize, value: Value) {
        let Some(row) = self.circuit_index(row) else { return };
        let Some(col) = self.circuit_index(col) else { return };
        (self.matrix_add)(row, col, value);
    }

    #[inline]
    pub fn rhs(&mut self, row: usize, value: Value) {
        let Some(row) = self.circuit_index(row) else { return };
        (self.rhs_add)(row, value);
    }

    #[inline]
    fn circuit_index(&self, generated_index: usize) -> Option<usize> {
        if generated_index == usize::MAX {
            return None;
        }
        let circuit_node = if generated_index < self.nodes.len() {
            self.nodes[generated_index]
        } else if generated_index < self.nodes.len() + self.internal_nodes.len() {
            self.internal_nodes[generated_index - self.nodes.len()]
        } else {
            let branch = generated_index - self.nodes.len() - self.internal_nodes.len();
            self.branch_nodes.get(branch).copied().unwrap_or(0)
        };
        (circuit_node > 0).then_some(circuit_node - 1)
    }
}
```

- [ ] **Step 5: Run the tests**

Run:

```powershell
cargo test -p rspice-core --features veriloga-builtins --lib veriloga_generated -- --nocapture
```

Expected: runtime ABI tests pass.

- [ ] **Step 6: Commit**

```powershell
git add crates/rspice-core/Cargo.toml crates/rspice-core/src/device/mod.rs crates/rspice-core/src/device/veriloga_generated
git commit -m "feat(core): add generated veriloga runtime abi"
```

---

### Task 6: Add Cargo Build Script Registry Generation

**Files:**
- Modify: `crates/rspice-core/Cargo.toml`
- Create: `crates/rspice-core/build.rs`
- Modify: `crates/rspice-core/src/device/veriloga_generated/mod.rs`
- Create: `crates/rspice-core/tests/fixtures/veriloga_builtins/simple_res.va`

- [ ] **Step 1: Add a fixture Verilog-A built-in**

Create `crates/rspice-core/tests/fixtures/veriloga_builtins/simple_res.va`:

```verilog
module simple_res(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 1000.0 from (0:inf);
    analog I(p, n) <+ V(p, n) / r;
endmodule
```

- [ ] **Step 2: Add build-dependency and build script declaration**

Modify `crates/rspice-core/Cargo.toml`:

```toml
[package]
build = "build.rs"

[build-dependencies]
rspice-veriloga = { path = "../rspice-veriloga", version = "0.1.0" }
```

If a `[build-dependencies]` table already exists when implementing, merge this line into it.

- [ ] **Step 3: Create build script with empty registry fallback**

Create `crates/rspice-core/build.rs`:

```rust
use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR must be set"));
    let generated_root = out_dir.join("veriloga_builtins");
    std::fs::create_dir_all(&generated_root).expect("create generated Verilog-A root");

    if env::var_os("CARGO_FEATURE_VERILOGA_BUILTINS").is_none() {
        write_empty_registry(&generated_root);
        return;
    }

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("manifest dir"));
    let default_root = manifest_dir.join("..").join("..").join("models").join("veriloga");
    let source_root = env::var_os("RSPICE_VERILOGA_BUILTINS_DIR")
        .map(PathBuf::from)
        .unwrap_or(default_root);

    println!("cargo:rerun-if-env-changed=RSPICE_VERILOGA_BUILTINS_DIR");
    println!("cargo:rerun-if-changed={}", source_root.display());

    if let Err(error) = generate_registry(&source_root, &generated_root) {
        panic!("failed to generate Verilog-A built-ins: {error}");
    }
}

fn write_empty_registry(root: &Path) {
    let registry = root.join("registry.rs");
    std::fs::write(
        registry,
        "pub fn builtin_names() -> &'static [&'static str] { &[] }\n",
    )
    .expect("write empty Verilog-A registry");
}

fn generate_registry(source_root: &Path, generated_root: &Path) -> Result<(), String> {
    use rspice_veriloga::rust_backend::{
        RustTranspiler, discover_veriloga_sources, write_generated_device,
    };
    use rspice_veriloga::VerilogACompiler;

    let sources = discover_veriloga_sources(source_root).map_err(|e| e.to_string())?;
    let devices_root = generated_root.join("devices");
    std::fs::create_dir_all(&devices_root).map_err(|e| e.to_string())?;

    let compiler = VerilogACompiler::default();
    let transpiler = RustTranspiler::default();
    let mut registry = String::new();
    let mut names = Vec::new();

    for source in sources {
        println!("cargo:rerun-if-changed={}", source.path.display());
        for module in source.modules {
            let compiled = compiler
                .compile_file_canonical_ir_with_metadata(&source.path, Some(&module))
                .map_err(|e| format!("{} module {}: {e}", source.path.display(), module))?;
            for dep in compiled.dependencies {
                println!("cargo:rerun-if-changed={}", dep.display());
            }
            let device = transpiler
                .transpile(&compiled.artifact)
                .map_err(|e| e.to_string())?;
            write_generated_device(&devices_root, &device).map_err(|e| e.to_string())?;
            registry.push_str(&format!(
                "#[path = {:?}]\npub mod {};\n",
                devices_root.join(&device.folder_name).join("mod.rs").display().to_string(),
                rspice_veriloga::rust_backend::sanitize_identifier(&device.folder_name)
            ));
            names.push(device.public_model_name);
        }
    }

    names.sort();
    names.dedup();
    registry.push_str("pub fn builtin_names() -> &'static [&'static str] {\n    &[\n");
    for name in &names {
        registry.push_str(&format!("        {:?},\n", name));
    }
    registry.push_str("    ]\n}\n");

    std::fs::write(generated_root.join("registry.rs"), registry).map_err(|e| e.to_string())?;
    Ok(())
}
```

- [ ] **Step 4: Include generated registry from runtime module**

Append to `crates/rspice-core/src/device/veriloga_generated/mod.rs`:

```rust
#[cfg(feature = "veriloga-builtins")]
pub mod builtins {
    include!(concat!(env!("OUT_DIR"), "/veriloga_builtins/registry.rs"));
}
```

- [ ] **Step 5: Run build-script check with fixture directory**

Run:

```powershell
$env:RSPICE_VERILOGA_BUILTINS_DIR='C:\Users\James\Desktop\RSpice\.worktrees\veriloga-rust-transpiler-design\crates\rspice-core\tests\fixtures\veriloga_builtins'
cargo check -p rspice-core --features veriloga-builtins
Remove-Item Env:\RSPICE_VERILOGA_BUILTINS_DIR
```

Expected: `rspice-core` checks successfully and the build log mentions generated Verilog-A built-ins.

- [ ] **Step 6: Commit**

```powershell
git add crates/rspice-core/Cargo.toml crates/rspice-core/build.rs crates/rspice-core/src/device/veriloga_generated/mod.rs crates/rspice-core/tests/fixtures/veriloga_builtins/simple_res.va
git commit -m "feat(core): generate veriloga builtins at build time"
```

---

### Task 7: Wire Generated Built-Ins Into Circuit Construction

**Files:**
- Modify: `crates/rspice-core/src/device/veriloga_generated/mod.rs`
- Modify: `crates/rspice-core/src/circuit/mod.rs`
- Modify: `crates/rspice-core/src/circuit/construction.rs`
- Modify: `crates/rspice-core/src/circuit/external_models.rs`
- Modify: `crates/rspice-core/src/circuit/nonlinear.rs`
- Modify: `crates/rspice-core/src/engine/builder.rs`
- Test: `crates/rspice-core/tests/generated_veriloga_builtins.rs`

- [ ] **Step 1: Write failing end-to-end test**

Create `crates/rspice-core/tests/generated_veriloga_builtins.rs`:

```rust
#![cfg(feature = "veriloga-builtins")]

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

#[test]
fn generated_builtin_resistor_runs_without_veriloga_directive() {
    let deck = r#"
v1 in 0 dc 1
X1 in out simple_res r=1000
X2 out 0 simple_res r=1000
.end
"#;
    let netlist = Netlist::parse(deck).expect("parse deck");
    let engine = Engine::new(SimulationConfig::default());
    let result = engine.run_dc_op(&netlist).expect("dc op");

    let out = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .expect("out node");
    assert!((result.node_voltages[out] - 0.5).abs() < 1e-10);
}
```

- [ ] **Step 2: Run it to verify failure**

Run with the fixture env var:

```powershell
$env:RSPICE_VERILOGA_BUILTINS_DIR='C:\Users\James\Desktop\RSpice\.worktrees\veriloga-rust-transpiler-design\crates\rspice-core\tests\fixtures\veriloga_builtins'
cargo test -p rspice-core --features veriloga-builtins --test generated_veriloga_builtins -- --nocapture
Remove-Item Env:\RSPICE_VERILOGA_BUILTINS_DIR
```

Expected: fail because the builder does not instantiate generated built-ins.

- [ ] **Step 3: Add generated instance enum and collection**

Extend `crates/rspice-core/src/device/veriloga_generated/mod.rs`:

```rust
#[derive(Debug, Clone)]
pub struct BuiltinVerilogAInstance {
    pub model_name: &'static str,
    pub instance_name: String,
    pub nodes: Vec<usize>,
    pub internal_nodes: Vec<usize>,
    pub branch_nodes: Vec<usize>,
    pub kind: BuiltinVerilogAKind,
}

#[derive(Debug, Clone)]
pub enum BuiltinVerilogAKind {
    #[cfg(feature = "veriloga-builtins")]
    Generated(String),
}

#[derive(Debug, Clone, Default)]
pub struct BuiltinVerilogADevices {
    devices: Vec<BuiltinVerilogAInstance>,
}

impl BuiltinVerilogADevices {
    pub fn new() -> Self { Self { devices: Vec::new() } }
    pub fn add(&mut self, device: BuiltinVerilogAInstance) { self.devices.push(device); }
    pub fn is_empty(&self) -> bool { self.devices.is_empty() }
    pub fn len(&self) -> usize { self.devices.len() }
    pub fn iter(&self) -> impl Iterator<Item = &BuiltinVerilogAInstance> { self.devices.iter() }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut BuiltinVerilogAInstance> { self.devices.iter_mut() }

    pub fn stamp_all<M, R>(&mut self, voltages: &[Value], mut matrix_add: M, mut rhs_add: R)
    where
        M: FnMut(usize, usize, Value),
        R: FnMut(usize, Value),
    {
        for device in &mut self.devices {
            device.stamp(voltages, &mut matrix_add, &mut rhs_add);
        }
    }
}

impl BuiltinVerilogAInstance {
    pub fn stamp<M, R>(&mut self, voltages: &[Value], matrix_add: M, rhs_add: R)
    where
        M: FnMut(usize, usize, Value),
        R: FnMut(usize, Value),
    {
        let ctx = GeneratedEvalContext::new(voltages, &self.nodes, &self.internal_nodes, &self.branch_nodes);
        let mut stamper = GeneratedStamper::new(&self.nodes, &self.internal_nodes, &self.branch_nodes, matrix_add, rhs_add);
        match &mut self.kind {
            BuiltinVerilogAKind::Generated(_) => {
                // Task 8 replaces this temporary inactive branch with typed generated enum dispatch.
            }
        }
    }
}
```

Task 8 replaces the temporary inactive dispatch branch with typed generated enum dispatch. This step creates the storage and call sites first.

- [ ] **Step 4: Add generated-device storage to `CircuitData`**

Modify `crates/rspice-core/src/circuit/mod.rs`:

```rust
#[cfg(feature = "veriloga-builtins")]
pub(crate) generated_veriloga_devices: crate::device::veriloga_generated::BuiltinVerilogADevices,
```

Modify `crates/rspice-core/src/circuit/construction.rs` inside `CircuitData::new()`:

```rust
#[cfg(feature = "veriloga-builtins")]
generated_veriloga_devices: crate::device::veriloga_generated::BuiltinVerilogADevices::new(),
```

Modify `crates/rspice-core/src/circuit/external_models.rs`:

```rust
#[cfg(feature = "veriloga-builtins")]
pub fn has_generated_veriloga_devices(&self) -> bool {
    !self.generated_veriloga_devices.is_empty()
}

#[cfg(feature = "veriloga-builtins")]
pub fn add_generated_veriloga_device(
    &mut self,
    device: crate::device::veriloga_generated::BuiltinVerilogAInstance,
) {
    self.generated_veriloga_devices.add(device);
}
```

Modify `crates/rspice-core/src/circuit/nonlinear.rs`:

```rust
#[cfg(feature = "veriloga-builtins")]
generated_veriloga_devices: crate::device::veriloga_generated::BuiltinVerilogADevices,
```

Include generated devices in `has_nonlinear_devices`, `has_physical_nonlinear_devices`, snapshots, restore, and `stamp_nonlinear`:

```rust
#[cfg(feature = "veriloga-builtins")]
{
    self.has_generated_veriloga_devices()
}
```

and:

```rust
#[cfg(feature = "veriloga-builtins")]
{
    self.generated_veriloga_devices.stamp_all(
        voltages,
        |row, col, value| matrix.add(row, col, value),
        |index, value| {
            if let Some(slot) = rhs.get_mut(index) {
                *slot += value;
            }
        },
    );
}
```

- [ ] **Step 5: Add builder lookup hook**

Modify `crates/rspice-core/src/engine/builder.rs` in the `ElementKind::Subcircuit` branch before the legacy `veriloga_models` lookup:

```rust
#[cfg(feature = "veriloga-builtins")]
{
    if let Some(mut device) = crate::device::veriloga_generated::instantiate_builtin(
        subckt_name,
        &element.name,
        &element.nodes,
        params,
        &netlist.params,
        &mut circuit,
    )? {
        circuit.add_generated_veriloga_device(device);
        continue;
    }
}
```

Add `instantiate_builtin` in `veriloga_generated/mod.rs` returning `Ok(None)` for now so the test still fails until Task 8 adds generated dispatch.

- [ ] **Step 6: Run focused check**

Run:

```powershell
cargo check -p rspice-core --features veriloga-builtins
```

Expected: compile succeeds.

- [ ] **Step 7: Commit**

```powershell
git add crates/rspice-core/src/device/veriloga_generated/mod.rs crates/rspice-core/src/circuit crates/rspice-core/src/engine/builder.rs crates/rspice-core/tests/generated_veriloga_builtins.rs
git commit -m "feat(core): wire generated veriloga device storage"
```

---

### Task 8: Generate Typed Registry Dispatch And Instance Construction

**Files:**
- Modify: `crates/rspice-veriloga/src/rust_backend/device.rs`
- Modify: `crates/rspice-core/build.rs`
- Modify: `crates/rspice-core/src/device/veriloga_generated/mod.rs`
- Test: `crates/rspice-core/tests/generated_veriloga_builtins.rs`

- [ ] **Step 1: Extend generated modules with construction hooks**

Update generated `state.rs` emission in `crates/rspice-veriloga/src/rust_backend/device.rs` so each generated `Instance` has parameter setters:

```rust
impl Instance {
    pub fn set_parameter(&mut self, name: &str, value: f64) -> bool {
        match name.to_ascii_lowercase().as_str() {
            "r" => { self.params.r = value; true }
            _ => false,
        }
    }
}
```

Generate the match arms from `artifact.mir.parameters`, including aliases. Use sanitized field names and original/alias lowercase names.

- [ ] **Step 2: Emit typed registry enum from `build.rs`**

Change `generate_registry` in `crates/rspice-core/build.rs` so `registry.rs` contains:

```rust
#[derive(Debug, Clone)]
pub enum GeneratedBuiltinKind {
    SimpleRes(simple_res__simple_res__digest::Instance),
}

impl GeneratedBuiltinKind {
    pub fn stamp(
        &mut self,
        ctx: &super::GeneratedEvalContext<'_>,
        stamper: &mut super::GeneratedStamper<'_>,
    ) {
        match self {
            Self::SimpleRes(device) => device.stamp(ctx, stamper),
        }
    }
}

pub fn instantiate(
    model_name: &str,
    nodes: &[usize],
    params: &[(String, f64)],
) -> Option<GeneratedBuiltinKind> {
    match model_name.to_ascii_uppercase().as_str() {
        "SIMPLE_RES" => {
            let mut instance = simple_res__simple_res__digest::Instance::new(nodes);
            for (name, value) in params {
                let _ = instance.set_parameter(name, *value);
            }
            Some(GeneratedBuiltinKind::SimpleRes(instance))
        }
        _ => None,
    }
}
```

Use generated Rust identifiers from `RustDeviceNames`; do not hardcode `SimpleRes`.

- [ ] **Step 3: Replace temporary generated kind dispatch**

Modify `crates/rspice-core/src/device/veriloga_generated/mod.rs`:

```rust
#[derive(Debug, Clone)]
pub enum BuiltinVerilogAKind {
    Generated(builtins::GeneratedBuiltinKind),
}
```

Update `BuiltinVerilogAInstance::stamp`:

```rust
match &mut self.kind {
    BuiltinVerilogAKind::Generated(device) => device.stamp(&ctx, &mut stamper),
}
```

Implement `instantiate_builtin`:

```rust
pub fn instantiate_builtin(
    model_name: &str,
    instance_name: &str,
    node_names: &[String],
    params: &[(String, crate::netlist::ParametricValue)],
    param_ctx: &crate::netlist::ParamContext,
    circuit: &mut crate::CircuitData,
) -> Result<Option<BuiltinVerilogAInstance>, crate::engine::SimulationError> {
    let Some(descriptor_name) = builtins::builtin_names()
        .iter()
        .find(|name| name.eq_ignore_ascii_case(model_name))
        .copied()
    else {
        return Ok(None);
    };

    let mut nodes = Vec::with_capacity(node_names.len());
    for node_name in node_names {
        nodes.push(if node_name.eq_ignore_ascii_case("0") {
            0
        } else {
            circuit.get_or_create_node(node_name)
        });
    }

    let mut resolved = Vec::with_capacity(params.len());
    for (name, value) in params {
        let value = match value {
            crate::netlist::ParametricValue::Resolved(value) => *value,
            crate::netlist::ParametricValue::Expression(expr) => {
                crate::netlist::expr::eval_expression(expr, param_ctx).map_err(|error| {
                    crate::engine::SimulationError::Circuit(format!(
                        "Failed to resolve generated Verilog-A parameter '{}': {}",
                        name, error
                    ))
                })?
            }
        };
        resolved.push((name.clone(), value));
    }

    let Some(kind) = builtins::instantiate(descriptor_name, &nodes, &resolved) else {
        return Ok(None);
    };

    Ok(Some(BuiltinVerilogAInstance {
        model_name: descriptor_name,
        instance_name: instance_name.to_string(),
        nodes,
        internal_nodes: Vec::new(),
        branch_nodes: Vec::new(),
        kind: BuiltinVerilogAKind::Generated(kind),
    }))
}
```

- [ ] **Step 4: Run the end-to-end generated built-in test**

Run:

```powershell
$env:RSPICE_VERILOGA_BUILTINS_DIR='C:\Users\James\Desktop\RSpice\.worktrees\veriloga-rust-transpiler-design\crates\rspice-core\tests\fixtures\veriloga_builtins'
cargo test -p rspice-core --features veriloga-builtins --test generated_veriloga_builtins -- --nocapture
Remove-Item Env:\RSPICE_VERILOGA_BUILTINS_DIR
```

Expected: pass. The deck does not contain `.va` or `.veriloga`.

- [ ] **Step 5: Commit**

```powershell
git add crates/rspice-veriloga/src/rust_backend/device.rs crates/rspice-core/build.rs crates/rspice-core/src/device/veriloga_generated/mod.rs crates/rspice-core/tests/generated_veriloga_builtins.rs
git commit -m "feat(core): instantiate generated veriloga builtins"
```

---

### Task 9: Guard Unsupported Constructs With Precise Diagnostics

**Files:**
- Modify: `crates/rspice-veriloga/src/rust_backend/device.rs`
- Modify: `crates/rspice-veriloga/src/rust_backend/expr.rs`
- Test: `crates/rspice-veriloga/tests/rust_backend.rs`

- [ ] **Step 1: Add failing tests for fail-closed behavior**

Append:

```rust
#[test]
fn rust_backend_rejects_stateful_operator_until_supported() {
    let src = r#"
module cap(p, n);
    inout p, n;
    electrical p, n;
    parameter real c = 1e-12;
    analog I(p, n) <+ ddt(c * V(p, n));
endmodule
"#;
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(src)
        .expect("canonical IR");
    let err = RustTranspiler::default()
        .transpile(&artifact)
        .expect_err("ddt unsupported in first slice");
    assert!(err.to_string().contains("stateful"));
}

#[test]
fn rust_backend_rejects_non_current_contribution_until_supported() {
    let src = r#"
module vsrc(p, n);
    inout p, n;
    electrical p, n;
    analog V(p, n) <+ 1.0;
endmodule
"#;
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(src)
        .expect("canonical IR");
    let err = RustTranspiler::default()
        .transpile(&artifact)
        .expect_err("potential unsupported in first slice");
    assert!(err.to_string().contains("potential"));
}
```

- [ ] **Step 2: Run the tests**

Run:

```powershell
cargo test -p rspice-veriloga --test rust_backend rust_backend_rejects_stateful_operator_until_supported rust_backend_rejects_non_current_contribution_until_supported -- --nocapture
```

Expected: fail if diagnostics are vague or unsupported cases slip through.

- [ ] **Step 3: Tighten validation**

Ensure `generate_device` recursively scans every expression used by equations and statements. Add helper:

```rust
fn reject_unsupported_effects(artifact: &CanonicalIrArtifact) -> Result<(), RustBackendError> {
    for expr in &artifact.mir.expressions {
        match &expr.kind {
            crate::canonical_ir::HirExprKind::AnalogOperator { op } => match op {
                crate::canonical_ir::HirAnalogOperator::Limexp { .. } => {}
                _ => {
                    return Err(RustBackendError::unsupported(
                        artifact.metadata.source_package.as_str(),
                        artifact.mir.module_name.as_str(),
                        format!("stateful or effectful analog operator {op:?}"),
                    ));
                }
            },
            crate::canonical_ir::HirExprKind::NoiseSource { .. }
            | crate::canonical_ir::HirExprKind::Laplace { .. }
            | crate::canonical_ir::HirExprKind::Zi { .. } => {
                return Err(RustBackendError::unsupported(
                    artifact.metadata.source_package.as_str(),
                    artifact.mir.module_name.as_str(),
                    format!("expression kind {:?}", expr.kind),
                ));
            }
            _ => {}
        }
    }
    Ok(())
}
```

Call this at the start of `generate_device`.

- [ ] **Step 4: Run all backend tests**

Run:

```powershell
cargo test -p rspice-veriloga --test rust_backend -- --nocapture
```

Expected: all backend tests pass.

- [ ] **Step 5: Commit**

```powershell
git add crates/rspice-veriloga/src/rust_backend crates/rspice-veriloga/tests/rust_backend.rs
git commit -m "feat(veriloga): fail closed for unsupported rust backend constructs"
```

---

### Task 10: Verification And Product Build Gate

**Files:**
- Modify only if verification exposes small integration issues.

- [ ] **Step 1: Run formatting**

```powershell
cargo fmt --package rspice-veriloga --package rspice-core -- --check
```

Expected: pass. If it fails, run `cargo fmt --package rspice-veriloga --package rspice-core`, inspect the diff, then rerun the check.

- [ ] **Step 2: Run Verilog-A backend tests**

```powershell
cargo test -p rspice-veriloga --test rust_backend -- --nocapture
```

Expected: all Rust backend tests pass.

- [ ] **Step 3: Run existing canonical IR tests**

```powershell
cargo test -p rspice-veriloga --test canonical_ir -- --nocapture
```

Expected: all canonical IR tests pass.

- [ ] **Step 4: Run core generated built-in smoke test**

```powershell
$env:RSPICE_VERILOGA_BUILTINS_DIR='C:\Users\James\Desktop\RSpice\.worktrees\veriloga-rust-transpiler-design\crates\rspice-core\tests\fixtures\veriloga_builtins'
cargo test -p rspice-core --features veriloga-builtins --test generated_veriloga_builtins -- --nocapture
Remove-Item Env:\RSPICE_VERILOGA_BUILTINS_DIR
```

Expected: pass. This proves the simulator can instantiate a generated built-in without `.veriloga`.

- [ ] **Step 5: Run legacy external Verilog-A regression**

```powershell
cargo test -p rspice-core --features veriloga --test veriloga_mfactor -- --nocapture
```

Expected: pass. This proves the old external `.va` path still works during migration.

- [ ] **Step 6: Run package checks**

```powershell
cargo check -p rspice-core --features veriloga-builtins
cargo check -p rspice-cli
```

Expected: both pass. `rspice-cli` must keep building with its existing `veriloga-native` feature.

- [ ] **Step 7: Inspect generated code size and hot-path shape**

After the fixture build, inspect the generated `OUT_DIR/veriloga_builtins/devices/.../stamp.rs` path from the cargo build output or from `target/debug/build/rspice-core-*/out`. Confirm:

- one device folder exists for `simple_res`;
- `registry.rs` is small and imports the device module;
- `stamp.rs` computes value and analytic Jacobian directly;
- no generated hot-path code allocates `Vec`, `String`, `HashMap`, or calls parser/compiler APIs.

- [ ] **Step 8: Commit any verification fixes**

If verification required code changes:

```powershell
git add -u
git commit -m "fix(veriloga): polish generated rust backend integration"
```

If no changes were required, do not create an empty commit.

---

## Follow-Up Implementation Slices

These are intentionally outside the first implementation branch. They should become separate plans after the backend spine is merged:

- Add potential contributions and branch-current unknown generation.
- Add internal-node allocation and generated remapping.
- Add assignment statements, local variables, arrays, and bounded loops.
- Add `ddt`, `idt`, `idtmod`, charge/flux companion generation, and AC reactive stamps.
- Add noise source generation.
- Add event, delay, transition, slew, Laplace, and `zi_*` runtime state.
- Add richer OptIR scheduling so generated residual/Jacobian code fuses common subexpressions across equations.
- Enable `veriloga-builtins` in product crates once generated backend coverage matches the shipped built-in model set.

## Self-Review Notes

- Spec coverage: directory scanning, per-device generated folders, build-script inclusion, runtime lookup without `.veriloga`, hard unsupported diagnostics, and no golden dumps are covered.
- Scope boundary: this first branch builds a real generated Rust path for algebraic current devices and wires it into core. Full CMC parity is split into follow-up operator/model-family slices.
- Performance boundary: the first generated stamp path emits direct Rust with analytic derivatives and no hot-path interpreter. The runtime ABI uses closures initially for integration; a later optimization should replace boxed stamper closures with generic monomorphized stamper methods if profiling shows measurable overhead.
