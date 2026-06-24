# Canonical Verilog-A IR Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the first canonical multi-level Verilog-A IR foundation for RSpice without changing production simulation behavior.

**Architecture:** Add a new `rspice-veriloga::canonical_ir` module beside the current `ir` and `codegen` pipeline. The first slice creates typed IDs, metadata, diagnostics, HIR, MIR, OptIR schedules, deterministic dumps, verifier tests, and a public compiler API that emits the canonical IR artifact while leaving bytecode and Cranelift as legacy runtime paths.

**Tech Stack:** Rust 1.94, `rspice-veriloga`, existing parser and semantic analyzer, `serde`, `smol_str`, focused `cargo test -p rspice-veriloga --test canonical_ir`.

---

## File Structure

- Create `crates/rspice-veriloga/src/canonical_ir/mod.rs`: module root and public re-exports.
- Create `crates/rspice-veriloga/src/canonical_ir/ids.rs`: dense typed ID wrappers for compiler entities.
- Create `crates/rspice-veriloga/src/canonical_ir/diagnostic.rs`: phase-aware diagnostics and verifier result helpers.
- Create `crates/rspice-veriloga/src/canonical_ir/metadata.rs`: deterministic metadata and stable digest helpers.
- Create `crates/rspice-veriloga/src/canonical_ir/hir.rs`: high-level semantic IR and lowering from `AnalyzedModule`.
- Create `crates/rspice-veriloga/src/canonical_ir/mir.rs`: normalized device-equation IR and HIR-to-MIR lowering.
- Create `crates/rspice-veriloga/src/canonical_ir/opt.rs`: backend-independent schedule IR and MIR-to-OptIR lowering.
- Create `crates/rspice-veriloga/src/canonical_ir/artifact.rs`: canonical artifact wrapper and deterministic text dumps.
- Modify `crates/rspice-veriloga/src/lib.rs`: expose `canonical_ir` and add canonical IR compiler entry points.
- Create `crates/rspice-veriloga/tests/canonical_ir.rs`: focused integration tests for the new IR slice.

## Shared Test Helper

Use this helper in `crates/rspice-veriloga/tests/canonical_ir.rs` when tasks need an analyzed module:

```rust
use rspice_veriloga::{
    Lexer, Parser, SemanticAnalyzer, SourceMap,
    canonical_ir::CanonicalMetadata,
};

fn analyze_fixture(
    source: &str,
    module_name: &str,
) -> rspice_veriloga::CompileResult<rspice_veriloga::semantic::AnalyzedModule> {
    let mut source_map = SourceMap::new();
    let source_id = source_map.add_source_mut("<fixture>", source);
    let tokens = Lexer::new(source, source_id).collect_tokens()?;
    let source_file = Parser::new(&tokens).parse()?;
    let analyzed = SemanticAnalyzer::new().analyze(&source_file)?;
    analyzed
        .modules
        .get(module_name)
        .cloned()
        .ok_or_else(|| rspice_veriloga::CompileError::ModuleSelection(module_name.to_string()))
}

fn tiny_resistor_source() -> &'static str {
    r#"
module tiny_res(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 1000.0 from (0:inf);
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#
}
```

---

### Task 1: Add Canonical IR Module And Typed IDs

**Files:**
- Create: `crates/rspice-veriloga/src/canonical_ir/mod.rs`
- Create: `crates/rspice-veriloga/src/canonical_ir/ids.rs`
- Modify: `crates/rspice-veriloga/src/lib.rs`
- Test: `crates/rspice-veriloga/tests/canonical_ir.rs`

- [ ] **Step 1: Write the failing ID test**

Create `crates/rspice-veriloga/tests/canonical_ir.rs` with:

```rust
use rspice_veriloga::canonical_ir::{ModuleId, ParamId, PortId, SourceId};

#[test]
fn typed_ids_are_dense_copyable_and_displayable() {
    let module = ModuleId::new(7);
    let source = SourceId::new(3);
    let port = PortId::new(2);
    let param = ParamId::new(5);

    assert_eq!(module.index(), 7);
    assert_eq!(source.index(), 3);
    assert_eq!(port.index(), 2);
    assert_eq!(param.index(), 5);
    assert_eq!(module.to_string(), "ModuleId(7)");
    assert_eq!(port.next(), PortId::new(3));
}
```

- [ ] **Step 2: Run the red test**

Run:

```powershell
cargo test -p rspice-veriloga --test canonical_ir typed_ids_are_dense_copyable_and_displayable
```

Expected: FAIL with `could not find canonical_ir in rspice_veriloga`.

- [ ] **Step 3: Add the ID implementation**

Create `crates/rspice-veriloga/src/canonical_ir/ids.rs`:

```rust
use serde::{Deserialize, Serialize};
use std::fmt;

macro_rules! id_type {
    ($name:ident) => {
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
        )]
        pub struct $name(u32);

        impl $name {
            pub const fn new(index: u32) -> Self {
                Self(index)
            }

            pub const fn index(self) -> u32 {
                self.0
            }

            pub const fn next(self) -> Self {
                Self(self.0 + 1)
            }
        }

        impl From<usize> for $name {
            fn from(value: usize) -> Self {
                Self(value as u32)
            }
        }

        impl From<$name> for usize {
            fn from(value: $name) -> Self {
                value.0 as usize
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}({})", stringify!($name), self.0)
            }
        }
    };
}

id_type!(ModuleId);
id_type!(SourceId);
id_type!(SymbolId);
id_type!(PortId);
id_type!(DisciplineId);
id_type!(ParamId);
id_type!(VariableId);
id_type!(ArrayId);
id_type!(NodeId);
id_type!(BranchId);
id_type!(BranchUnknownId);
id_type!(StateId);
id_type!(EquationId);
id_type!(ContributionId);
id_type!(NoiseSourceId);
id_type!(RegionId);
id_type!(ExprId);
id_type!(ValueId);
id_type!(ScheduleId);
```

Create `crates/rspice-veriloga/src/canonical_ir/mod.rs`:

```rust
//! Canonical multi-level Verilog-A IR.
//!
//! This module is the new semantic compiler target for Verilog-A models.
//! The existing bytecode and Cranelift paths remain legacy runtime paths
//! while this IR is introduced and verified.

pub mod ids;

pub use ids::{
    ArrayId, BranchId, BranchUnknownId, ContributionId, DisciplineId, EquationId, ExprId,
    ModuleId, NodeId, NoiseSourceId, ParamId, PortId, RegionId, ScheduleId, SourceId, StateId,
    SymbolId, ValueId, VariableId,
};
```

Modify `crates/rspice-veriloga/src/lib.rs` by adding this near the existing public modules:

```rust
pub mod canonical_ir;
```

- [ ] **Step 4: Run the green test**

Run:

```powershell
cargo test -p rspice-veriloga --test canonical_ir typed_ids_are_dense_copyable_and_displayable
```

Expected: PASS.

- [ ] **Step 5: Commit**

Run:

```powershell
git add crates/rspice-veriloga/src/canonical_ir/mod.rs crates/rspice-veriloga/src/canonical_ir/ids.rs crates/rspice-veriloga/src/lib.rs crates/rspice-veriloga/tests/canonical_ir.rs
git commit -m "feat(veriloga): add canonical ir id types"
```

---

### Task 2: Add Metadata, Stable Digests, And Diagnostics

**Files:**
- Create: `crates/rspice-veriloga/src/canonical_ir/metadata.rs`
- Create: `crates/rspice-veriloga/src/canonical_ir/diagnostic.rs`
- Modify: `crates/rspice-veriloga/src/canonical_ir/mod.rs`
- Test: `crates/rspice-veriloga/tests/canonical_ir.rs`

- [ ] **Step 1: Add the failing metadata and diagnostics tests**

Append to `crates/rspice-veriloga/tests/canonical_ir.rs`:

```rust
use rspice_veriloga::canonical_ir::{
    CanonicalMetadata, CompilerPhase, DiagnosticSeverity, IrDiagnostic, SourceSpanRef,
    StableDigest,
};

#[test]
fn metadata_digest_is_stable_and_hex_encoded() {
    let digest = StableDigest::from_text("module tiny; endmodule");
    assert_eq!(digest.as_hex().len(), 16);
    assert_eq!(digest, StableDigest::from_text("module tiny; endmodule"));
    assert_ne!(digest, StableDigest::from_text("module other; endmodule"));

    let metadata = CanonicalMetadata::for_source("fixture", "module tiny; endmodule");
    assert_eq!(metadata.schema_version, 1);
    assert_eq!(metadata.source_package.as_str(), "fixture");
    assert_eq!(metadata.source_digest.as_str(), digest.as_hex());
}

#[test]
fn diagnostics_are_phase_aware_and_source_spanned() {
    let diagnostic = IrDiagnostic::error(
        CompilerPhase::MirValidation,
        "missing equation row",
        SourceSpanRef {
            source: 0,
            start: 12,
            end: 20,
        },
    );

    assert_eq!(diagnostic.severity, DiagnosticSeverity::Error);
    assert_eq!(diagnostic.phase, CompilerPhase::MirValidation);
    assert_eq!(diagnostic.message, "missing equation row");
    assert!(diagnostic.to_string().contains("MirValidation"));
}
```

- [ ] **Step 2: Run the red tests**

Run:

```powershell
cargo test -p rspice-veriloga --test canonical_ir metadata_digest_is_stable_and_hex_encoded
cargo test -p rspice-veriloga --test canonical_ir diagnostics_are_phase_aware_and_source_spanned
```

Expected: FAIL with unresolved imports for `CanonicalMetadata`, `CompilerPhase`, `IrDiagnostic`, `SourceSpanRef`, and `StableDigest`.

- [ ] **Step 3: Add metadata**

Create `crates/rspice-veriloga/src/canonical_ir/metadata.rs`:

```rust
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StableDigest(u64);

impl StableDigest {
    pub fn from_text(text: &str) -> Self {
        let mut hash = 0xcbf29ce484222325u64;
        for byte in text.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        Self(hash)
    }

    pub fn as_hex(self) -> String {
        format!("{:016x}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalMetadata {
    pub schema_version: u32,
    pub source_package: SmolStr,
    pub source_digest: SmolStr,
    pub compiler_version: SmolStr,
    pub feature_flags: Vec<SmolStr>,
}

impl CanonicalMetadata {
    pub fn for_source(source_package: impl Into<SmolStr>, source_text: &str) -> Self {
        Self {
            schema_version: 1,
            source_package: source_package.into(),
            source_digest: StableDigest::from_text(source_text).as_hex().into(),
            compiler_version: env!("CARGO_PKG_VERSION").into(),
            feature_flags: Vec::new(),
        }
    }
}
```

- [ ] **Step 4: Add diagnostics**

Create `crates/rspice-veriloga/src/canonical_ir/diagnostic.rs`:

```rust
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceSpanRef {
    pub source: u32,
    pub start: u32,
    pub end: u32,
}

impl From<crate::source::Span> for SourceSpanRef {
    fn from(value: crate::source::Span) -> Self {
        Self {
            source: value.source.raw(),
            start: value.start,
            end: value.end,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompilerPhase {
    HirLowering,
    HirValidation,
    MirLowering,
    MirValidation,
    OptLowering,
    OptValidation,
    Scheduling,
    Artifact,
    BackendLowering,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Note,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IrDiagnostic {
    pub severity: DiagnosticSeverity,
    pub phase: CompilerPhase,
    pub message: String,
    pub span: SourceSpanRef,
}

impl IrDiagnostic {
    pub fn error(
        phase: CompilerPhase,
        message: impl Into<String>,
        span: SourceSpanRef,
    ) -> Self {
        Self {
            severity: DiagnosticSeverity::Error,
            phase,
            message: message.into(),
            span,
        }
    }
}

impl fmt::Display for IrDiagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} {:?} at {}:{}-{}: {}",
            self.severity, self.phase, self.span.source, self.span.start, self.span.end, self.message
        )
    }
}

pub type IrValidationResult = Result<(), Vec<IrDiagnostic>>;
```

Modify `crates/rspice-veriloga/src/canonical_ir/mod.rs`:

```rust
pub mod diagnostic;
pub mod ids;
pub mod metadata;

pub use diagnostic::{
    CompilerPhase, DiagnosticSeverity, IrDiagnostic, IrValidationResult, SourceSpanRef,
};
pub use ids::{
    ArrayId, BranchId, BranchUnknownId, ContributionId, DisciplineId, EquationId, ExprId,
    ModuleId, NodeId, NoiseSourceId, ParamId, PortId, RegionId, ScheduleId, SourceId, StateId,
    SymbolId, ValueId, VariableId,
};
pub use metadata::{CanonicalMetadata, StableDigest};
```

- [ ] **Step 5: Run the green tests**

Run:

```powershell
cargo test -p rspice-veriloga --test canonical_ir metadata_digest_is_stable_and_hex_encoded
cargo test -p rspice-veriloga --test canonical_ir diagnostics_are_phase_aware_and_source_spanned
```

Expected: PASS.

- [ ] **Step 6: Commit**

Run:

```powershell
git add crates/rspice-veriloga/src/canonical_ir/metadata.rs crates/rspice-veriloga/src/canonical_ir/diagnostic.rs crates/rspice-veriloga/src/canonical_ir/mod.rs crates/rspice-veriloga/tests/canonical_ir.rs
git commit -m "feat(veriloga): add canonical ir metadata diagnostics"
```

---

### Task 3: Add HIR Model, Lowering, And Verifier

**Files:**
- Create: `crates/rspice-veriloga/src/canonical_ir/hir.rs`
- Modify: `crates/rspice-veriloga/src/canonical_ir/mod.rs`
- Test: `crates/rspice-veriloga/tests/canonical_ir.rs`

- [ ] **Step 1: Add the failing HIR lowering test**

Append to `crates/rspice-veriloga/tests/canonical_ir.rs`:

```rust
use rspice_veriloga::canonical_ir::{HirContributionKind, HirModel};

#[test]
fn hir_lowering_preserves_analyzed_module_surface() {
    let analyzed = analyze_fixture(tiny_resistor_source(), "tiny_res").expect("analyze fixture");
    let metadata = CanonicalMetadata::for_source("fixture", tiny_resistor_source());
    let hir = HirModel::from_analyzed_module(&metadata, &analyzed);

    assert_eq!(hir.module_name.as_str(), "tiny_res");
    assert_eq!(hir.ports.len(), 2);
    assert_eq!(hir.ports[0].name.as_str(), "p");
    assert_eq!(hir.ports[1].name.as_str(), "n");
    assert_eq!(hir.parameters.len(), 1);
    assert_eq!(hir.parameters[0].name.as_str(), "r");
    assert_eq!(hir.parameters[0].default, Some(1000.0));
    assert_eq!(hir.contributions.len(), 1);
    assert_eq!(hir.contributions[0].kind, HirContributionKind::Current);
    assert!(hir.validate().is_ok());
}
```

- [ ] **Step 2: Run the red test**

Run:

```powershell
cargo test -p rspice-veriloga --test canonical_ir hir_lowering_preserves_analyzed_module_surface
```

Expected: FAIL with unresolved imports for `HirModel` and `HirContributionKind`.

- [ ] **Step 3: Add the HIR implementation**

Create `crates/rspice-veriloga/src/canonical_ir/hir.rs`:

```rust
use crate::ast::Expression;
use crate::semantic::{AnalyzedContribution, AnalyzedModule, AnalyzedStatement};
use crate::types::{ParameterRange, ValueType};
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

use super::{
    ArrayId, CompilerPhase, ContributionId, IrDiagnostic, IrValidationResult, ParamId, PortId,
    SourceSpanRef, VariableId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CanonicalValueType {
    Real,
    Integer,
    String,
    Boolean,
    NatureAccess,
    Void,
    Unknown,
    Error,
}

impl From<ValueType> for CanonicalValueType {
    fn from(value: ValueType) -> Self {
        match value {
            ValueType::Real => Self::Real,
            ValueType::Integer => Self::Integer,
            ValueType::String => Self::String,
            ValueType::Boolean => Self::Boolean,
            ValueType::NatureAccess => Self::NatureAccess,
            ValueType::Void => Self::Void,
            ValueType::Unknown => Self::Unknown,
            ValueType::Error => Self::Error,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HirParamRange {
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub min_exclusive: bool,
    pub max_exclusive: bool,
    pub exclude: Vec<f64>,
}

impl HirParamRange {
    fn from_range(value: &ParameterRange) -> Self {
        Self {
            min: value.min,
            max: value.max,
            min_exclusive: value.min_exclusive,
            max_exclusive: value.max_exclusive,
            exclude: value.exclude.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HirExprRef {
    pub kind: SmolStr,
    pub span: SourceSpanRef,
}

impl HirExprRef {
    fn from_expr(expr: &Expression) -> Self {
        Self {
            kind: expr_kind(expr),
            span: SourceSpanRef::from(expr.span()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HirPort {
    pub id: PortId,
    pub name: SmolStr,
    pub direction: SmolStr,
    pub discipline: SmolStr,
    pub nature_potential: Option<SmolStr>,
    pub nature_flow: Option<SmolStr>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HirParameter {
    pub id: ParamId,
    pub name: SmolStr,
    pub value_type: CanonicalValueType,
    pub default: Option<f64>,
    pub default_expr: Option<HirExprRef>,
    pub range: Option<HirParamRange>,
    pub aliases: Vec<SmolStr>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HirVariable {
    pub id: VariableId,
    pub name: SmolStr,
    pub value_type: CanonicalValueType,
    pub is_state: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HirArray {
    pub id: ArrayId,
    pub name: SmolStr,
    pub base: u32,
    pub lower: i64,
    pub len: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HirBranch {
    pub name: SmolStr,
    pub pos_node: SmolStr,
    pub neg_node: SmolStr,
    pub discipline: SmolStr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HirContributionKind {
    Current,
    Potential,
    Indirect,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HirContribution {
    pub id: ContributionId,
    pub branch: SmolStr,
    pub kind: HirContributionKind,
    pub expr_type: CanonicalValueType,
    pub expr: HirExprRef,
    pub span: SourceSpanRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HirStatement {
    pub kind: SmolStr,
    pub span: SourceSpanRef,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HirModel {
    pub source_package: SmolStr,
    pub source_digest: SmolStr,
    pub module_name: SmolStr,
    pub ports: Vec<HirPort>,
    pub parameters: Vec<HirParameter>,
    pub variables: Vec<HirVariable>,
    pub arrays: Vec<HirArray>,
    pub branches: Vec<HirBranch>,
    pub contributions: Vec<HirContribution>,
    pub statements: Vec<HirStatement>,
    pub internal_nodes: Vec<SmolStr>,
    pub ground_nodes: Vec<SmolStr>,
}

impl HirModel {
    pub fn from_analyzed_module(
        metadata: &super::CanonicalMetadata,
        module: &AnalyzedModule,
    ) -> Self {
        let mut parameters: Vec<HirParameter> = module
            .parameters
            .iter()
            .enumerate()
            .map(|(idx, param)| HirParameter {
                id: ParamId::from(idx),
                name: param.name.clone(),
                value_type: CanonicalValueType::from(param.value_type),
                default: param.default,
                default_expr: param.default_expr.as_ref().map(HirExprRef::from_expr),
                range: param.range.as_ref().map(HirParamRange::from_range),
                aliases: Vec::new(),
            })
            .collect();

        for alias in &module.param_aliases {
            if let Some(param) = parameters.get_mut(alias.target) {
                param.aliases.push(alias.alias.clone());
            }
        }

        Self {
            source_package: metadata.source_package.clone(),
            source_digest: metadata.source_digest.clone(),
            module_name: module.name.clone(),
            ports: module
                .ports
                .iter()
                .enumerate()
                .map(|(idx, port)| HirPort {
                    id: PortId::from(idx),
                    name: port.name.clone(),
                    direction: format!("{:?}", port.direction).into(),
                    discipline: port.discipline.clone(),
                    nature_potential: port.nature_potential.clone(),
                    nature_flow: port.nature_flow.clone(),
                })
                .collect(),
            parameters,
            variables: module
                .variables
                .iter()
                .enumerate()
                .map(|(idx, variable)| HirVariable {
                    id: VariableId::from(idx),
                    name: variable.name.clone(),
                    value_type: CanonicalValueType::from(variable.value_type),
                    is_state: variable.is_state,
                })
                .collect(),
            arrays: module
                .arrays
                .iter()
                .enumerate()
                .map(|(idx, (name, array))| HirArray {
                    id: ArrayId::from(idx),
                    name: name.clone(),
                    base: array.base as u32,
                    lower: array.lower,
                    len: array.len,
                })
                .collect(),
            branches: module
                .branches
                .iter()
                .map(|branch| HirBranch {
                    name: branch.name.clone(),
                    pos_node: branch.pos_node.clone(),
                    neg_node: branch.neg_node.clone(),
                    discipline: branch.discipline.clone(),
                })
                .collect(),
            contributions: module
                .contributions
                .iter()
                .enumerate()
                .map(|(idx, contribution)| lower_contribution(idx, contribution))
                .collect(),
            statements: module.statements.iter().map(lower_statement).collect(),
            internal_nodes: module
                .internal_nodes
                .iter()
                .map(|node| node.name.clone())
                .collect(),
            ground_nodes: module.ground_nodes.clone(),
        }
    }

    pub fn validate(&self) -> IrValidationResult {
        let mut diagnostics = Vec::new();
        if self.module_name.is_empty() {
            diagnostics.push(IrDiagnostic::error(
                CompilerPhase::HirValidation,
                "HIR module name is empty",
                SourceSpanRef {
                    source: 0,
                    start: 0,
                    end: 0,
                },
            ));
        }
        if self.ports.is_empty() {
            diagnostics.push(IrDiagnostic::error(
                CompilerPhase::HirValidation,
                "HIR model has no ports",
                SourceSpanRef {
                    source: 0,
                    start: 0,
                    end: 0,
                },
            ));
        }
        if diagnostics.is_empty() {
            Ok(())
        } else {
            Err(diagnostics)
        }
    }
}

fn lower_contribution(idx: usize, contribution: &AnalyzedContribution) -> HirContribution {
    let kind = if contribution.indirect {
        HirContributionKind::Indirect
    } else if contribution.is_current {
        HirContributionKind::Current
    } else {
        HirContributionKind::Potential
    };

    HirContribution {
        id: ContributionId::from(idx),
        branch: contribution.branch.clone(),
        kind,
        expr_type: CanonicalValueType::from(contribution.expr_type),
        expr: HirExprRef::from_expr(&contribution.expression),
        span: SourceSpanRef::from(contribution.span),
    }
}

fn lower_statement(statement: &AnalyzedStatement) -> HirStatement {
    match statement {
        AnalyzedStatement::Assignment(assignment) => HirStatement {
            kind: "assignment".into(),
            span: SourceSpanRef::from(assignment.span),
        },
        AnalyzedStatement::Loop(loop_stmt) => HirStatement {
            kind: "loop".into(),
            span: SourceSpanRef::from(loop_stmt.span),
        },
    }
}

fn expr_kind(expr: &Expression) -> SmolStr {
    match expr {
        Expression::Number(_) => "number".into(),
        Expression::StringLit(_) => "string".into(),
        Expression::Identifier(_) => "identifier".into(),
        Expression::SystemFunction(_) => "system_function".into(),
        Expression::Binary(_) => "binary".into(),
        Expression::Unary(_) => "unary".into(),
        Expression::Conditional(_) => "conditional".into(),
        Expression::Call(_) => "call".into(),
        Expression::BranchAccess(_) => "branch_access".into(),
        Expression::ArrayAccess(_) => "array_access".into(),
        Expression::ArrayLiteral(_) => "array_literal".into(),
        Expression::AnalogOperator(_) => "analog_operator".into(),
        Expression::NoiseSource(_) => "noise_source".into(),
    }
}
```

Modify `crates/rspice-veriloga/src/canonical_ir/mod.rs`:

```rust
pub mod diagnostic;
pub mod hir;
pub mod ids;
pub mod metadata;

pub use diagnostic::{
    CompilerPhase, DiagnosticSeverity, IrDiagnostic, IrValidationResult, SourceSpanRef,
};
pub use hir::{
    CanonicalValueType, HirArray, HirBranch, HirContribution, HirContributionKind, HirExprRef,
    HirModel, HirParamRange, HirParameter, HirPort, HirStatement, HirVariable,
};
pub use ids::{
    ArrayId, BranchId, BranchUnknownId, ContributionId, DisciplineId, EquationId, ExprId,
    ModuleId, NodeId, NoiseSourceId, ParamId, PortId, RegionId, ScheduleId, SourceId, StateId,
    SymbolId, ValueId, VariableId,
};
pub use metadata::{CanonicalMetadata, StableDigest};
```

- [ ] **Step 4: Run the green test**

Run:

```powershell
cargo test -p rspice-veriloga --test canonical_ir hir_lowering_preserves_analyzed_module_surface
```

Expected: PASS.

- [ ] **Step 5: Commit**

Run:

```powershell
git add crates/rspice-veriloga/src/canonical_ir/hir.rs crates/rspice-veriloga/src/canonical_ir/mod.rs crates/rspice-veriloga/tests/canonical_ir.rs
git commit -m "feat(veriloga): lower analyzed modules to canonical hir"
```

---

### Task 4: Add MIR Model, HIR-To-MIR Lowering, And Verifier

**Files:**
- Create: `crates/rspice-veriloga/src/canonical_ir/mir.rs`
- Modify: `crates/rspice-veriloga/src/canonical_ir/mod.rs`
- Test: `crates/rspice-veriloga/tests/canonical_ir.rs`

- [ ] **Step 1: Add the failing MIR test**

Append to `crates/rspice-veriloga/tests/canonical_ir.rs`:

```rust
use rspice_veriloga::canonical_ir::{MirEquationKind, MirModel};

#[test]
fn mir_lowering_makes_contributions_explicit_equations() {
    let analyzed = analyze_fixture(tiny_resistor_source(), "tiny_res").expect("analyze fixture");
    let metadata = CanonicalMetadata::for_source("fixture", tiny_resistor_source());
    let hir = HirModel::from_analyzed_module(&metadata, &analyzed);
    let mir = MirModel::from_hir(&hir).expect("lower MIR");

    assert_eq!(mir.module_name.as_str(), "tiny_res");
    assert_eq!(mir.nodes.len(), 2);
    assert_eq!(mir.equations.len(), 1);
    assert_eq!(mir.equations[0].contribution.index(), 0);
    assert_eq!(mir.equations[0].kind, MirEquationKind::Current);
    assert!(mir.validate().is_ok());
}
```

- [ ] **Step 2: Run the red test**

Run:

```powershell
cargo test -p rspice-veriloga --test canonical_ir mir_lowering_makes_contributions_explicit_equations
```

Expected: FAIL with unresolved imports for `MirModel` and `MirEquationKind`.

- [ ] **Step 3: Add MIR implementation**

Create `crates/rspice-veriloga/src/canonical_ir/mir.rs`:

```rust
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

use super::{
    CompilerPhase, ContributionId, EquationId, HirContributionKind, HirModel, IrDiagnostic,
    IrValidationResult, NodeId, ParamId, SourceSpanRef, StateId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MirAnalysisDomain {
    Dc,
    Ac,
    Transient,
    Noise,
    OperatingPoint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MirEquationKind {
    Current,
    Potential,
    Indirect,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirNode {
    pub id: NodeId,
    pub name: SmolStr,
    pub is_external: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MirParameterSlot {
    pub id: ParamId,
    pub name: SmolStr,
    pub default: Option<f64>,
    pub aliases: Vec<SmolStr>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirStateSlot {
    pub id: StateId,
    pub name: SmolStr,
    pub owner: ContributionId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirEquation {
    pub id: EquationId,
    pub contribution: ContributionId,
    pub branch: SmolStr,
    pub kind: MirEquationKind,
    pub active_domains: Vec<MirAnalysisDomain>,
    pub span: SourceSpanRef,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MirModel {
    pub module_name: SmolStr,
    pub nodes: Vec<MirNode>,
    pub parameters: Vec<MirParameterSlot>,
    pub state_slots: Vec<MirStateSlot>,
    pub equations: Vec<MirEquation>,
}

impl MirModel {
    pub fn from_hir(hir: &HirModel) -> Result<Self, Vec<IrDiagnostic>> {
        hir.validate()?;

        let mut nodes: Vec<MirNode> = hir
            .ports
            .iter()
            .enumerate()
            .map(|(idx, port)| MirNode {
                id: NodeId::from(idx),
                name: port.name.clone(),
                is_external: true,
            })
            .collect();

        for internal in &hir.internal_nodes {
            nodes.push(MirNode {
                id: NodeId::from(nodes.len()),
                name: internal.clone(),
                is_external: false,
            });
        }

        let equations = hir
            .contributions
            .iter()
            .enumerate()
            .map(|(idx, contribution)| MirEquation {
                id: EquationId::from(idx),
                contribution: contribution.id,
                branch: contribution.branch.clone(),
                kind: match contribution.kind {
                    HirContributionKind::Current => MirEquationKind::Current,
                    HirContributionKind::Potential => MirEquationKind::Potential,
                    HirContributionKind::Indirect => MirEquationKind::Indirect,
                },
                active_domains: vec![
                    MirAnalysisDomain::Dc,
                    MirAnalysisDomain::Ac,
                    MirAnalysisDomain::Transient,
                    MirAnalysisDomain::OperatingPoint,
                ],
                span: contribution.span,
            })
            .collect();

        let model = Self {
            module_name: hir.module_name.clone(),
            nodes,
            parameters: hir
                .parameters
                .iter()
                .map(|param| MirParameterSlot {
                    id: param.id,
                    name: param.name.clone(),
                    default: param.default,
                    aliases: param.aliases.clone(),
                })
                .collect(),
            state_slots: Vec::new(),
            equations,
        };
        model.validate()?;
        Ok(model)
    }

    pub fn validate(&self) -> IrValidationResult {
        let mut diagnostics = Vec::new();
        if self.nodes.is_empty() {
            diagnostics.push(IrDiagnostic::error(
                CompilerPhase::MirValidation,
                "MIR model has no nodes",
                SourceSpanRef {
                    source: 0,
                    start: 0,
                    end: 0,
                },
            ));
        }
        for equation in &self.equations {
            if equation.active_domains.is_empty() {
                diagnostics.push(IrDiagnostic::error(
                    CompilerPhase::MirValidation,
                    format!("MIR equation {} has no active domains", equation.id.index()),
                    equation.span,
                ));
            }
        }
        if diagnostics.is_empty() {
            Ok(())
        } else {
            Err(diagnostics)
        }
    }
}
```

Modify `crates/rspice-veriloga/src/canonical_ir/mod.rs`:

```rust
pub mod diagnostic;
pub mod hir;
pub mod ids;
pub mod metadata;
pub mod mir;

pub use diagnostic::{
    CompilerPhase, DiagnosticSeverity, IrDiagnostic, IrValidationResult, SourceSpanRef,
};
pub use hir::{
    CanonicalValueType, HirArray, HirBranch, HirContribution, HirContributionKind, HirExprRef,
    HirModel, HirParamRange, HirParameter, HirPort, HirStatement, HirVariable,
};
pub use ids::{
    ArrayId, BranchId, BranchUnknownId, ContributionId, DisciplineId, EquationId, ExprId,
    ModuleId, NodeId, NoiseSourceId, ParamId, PortId, RegionId, ScheduleId, SourceId, StateId,
    SymbolId, ValueId, VariableId,
};
pub use metadata::{CanonicalMetadata, StableDigest};
pub use mir::{
    MirAnalysisDomain, MirEquation, MirEquationKind, MirModel, MirNode, MirParameterSlot,
    MirStateSlot,
};
```

- [ ] **Step 4: Run the green test**

Run:

```powershell
cargo test -p rspice-veriloga --test canonical_ir mir_lowering_makes_contributions_explicit_equations
```

Expected: PASS.

- [ ] **Step 5: Commit**

Run:

```powershell
git add crates/rspice-veriloga/src/canonical_ir/mir.rs crates/rspice-veriloga/src/canonical_ir/mod.rs crates/rspice-veriloga/tests/canonical_ir.rs
git commit -m "feat(veriloga): normalize canonical hir to mir"
```

---

### Task 5: Add OptIR Scheduling Skeleton And Verifier

**Files:**
- Create: `crates/rspice-veriloga/src/canonical_ir/opt.rs`
- Modify: `crates/rspice-veriloga/src/canonical_ir/mod.rs`
- Test: `crates/rspice-veriloga/tests/canonical_ir.rs`

- [ ] **Step 1: Add the failing OptIR schedule test**

Append to `crates/rspice-veriloga/tests/canonical_ir.rs`:

```rust
use rspice_veriloga::canonical_ir::{InvalidationClass, OptModel};

#[test]
fn opt_lowering_builds_newton_schedule_from_mir_equations() {
    let analyzed = analyze_fixture(tiny_resistor_source(), "tiny_res").expect("analyze fixture");
    let metadata = CanonicalMetadata::for_source("fixture", tiny_resistor_source());
    let hir = HirModel::from_analyzed_module(&metadata, &analyzed);
    let mir = MirModel::from_hir(&hir).expect("lower MIR");
    let opt = OptModel::from_mir(&mir).expect("lower OptIR");

    assert_eq!(opt.module_name.as_str(), "tiny_res");
    assert!(opt
        .schedules
        .iter()
        .any(|schedule| schedule.invalidation == InvalidationClass::NewtonIteration));
    let newton = opt
        .schedules
        .iter()
        .find(|schedule| schedule.invalidation == InvalidationClass::NewtonIteration)
        .expect("newton schedule");
    assert_eq!(newton.ops.len(), 1);
    assert!(opt.validate().is_ok());
}
```

- [ ] **Step 2: Run the red test**

Run:

```powershell
cargo test -p rspice-veriloga --test canonical_ir opt_lowering_builds_newton_schedule_from_mir_equations
```

Expected: FAIL with unresolved imports for `OptModel` and `InvalidationClass`.

- [ ] **Step 3: Add OptIR implementation**

Create `crates/rspice-veriloga/src/canonical_ir/opt.rs`:

```rust
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

use super::{
    CompilerPhase, EquationId, IrDiagnostic, IrValidationResult, MirModel, ScheduleId,
    SourceSpanRef, ValueId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvalidationClass {
    InstanceStatic,
    TemperatureStatic,
    TimestepStatic,
    OperatingPointStatic,
    NewtonIteration,
    AcFrequency,
    NoiseFrequency,
    OperatingPointReport,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OptValueType {
    Real,
    Boolean,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptValue {
    pub id: ValueId,
    pub value_type: OptValueType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OptOp {
    EvaluateEquation { equation: EquationId },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptSchedule {
    pub id: ScheduleId,
    pub invalidation: InvalidationClass,
    pub ops: Vec<OptOp>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptModel {
    pub module_name: SmolStr,
    pub values: Vec<OptValue>,
    pub schedules: Vec<OptSchedule>,
}

impl OptModel {
    pub fn from_mir(mir: &MirModel) -> Result<Self, Vec<IrDiagnostic>> {
        mir.validate()?;

        let mut schedules = Vec::new();
        if !mir.parameters.is_empty() {
            schedules.push(OptSchedule {
                id: ScheduleId::from(schedules.len()),
                invalidation: InvalidationClass::InstanceStatic,
                ops: Vec::new(),
            });
        }

        schedules.push(OptSchedule {
            id: ScheduleId::from(schedules.len()),
            invalidation: InvalidationClass::NewtonIteration,
            ops: mir
                .equations
                .iter()
                .map(|equation| OptOp::EvaluateEquation {
                    equation: equation.id,
                })
                .collect(),
        });

        let model = Self {
            module_name: mir.module_name.clone(),
            values: Vec::new(),
            schedules,
        };
        model.validate()?;
        Ok(model)
    }

    pub fn validate(&self) -> IrValidationResult {
        let has_newton_schedule = self
            .schedules
            .iter()
            .any(|schedule| schedule.invalidation == InvalidationClass::NewtonIteration);
        if has_newton_schedule {
            Ok(())
        } else {
            Err(vec![IrDiagnostic::error(
                CompilerPhase::OptValidation,
                "OptIR model has no Newton-iteration schedule",
                SourceSpanRef {
                    source: 0,
                    start: 0,
                    end: 0,
                },
            )])
        }
    }
}
```

Modify `crates/rspice-veriloga/src/canonical_ir/mod.rs`:

```rust
pub mod diagnostic;
pub mod hir;
pub mod ids;
pub mod metadata;
pub mod mir;
pub mod opt;

pub use diagnostic::{
    CompilerPhase, DiagnosticSeverity, IrDiagnostic, IrValidationResult, SourceSpanRef,
};
pub use hir::{
    CanonicalValueType, HirArray, HirBranch, HirContribution, HirContributionKind, HirExprRef,
    HirModel, HirParamRange, HirParameter, HirPort, HirStatement, HirVariable,
};
pub use ids::{
    ArrayId, BranchId, BranchUnknownId, ContributionId, DisciplineId, EquationId, ExprId,
    ModuleId, NodeId, NoiseSourceId, ParamId, PortId, RegionId, ScheduleId, SourceId, StateId,
    SymbolId, ValueId, VariableId,
};
pub use metadata::{CanonicalMetadata, StableDigest};
pub use mir::{
    MirAnalysisDomain, MirEquation, MirEquationKind, MirModel, MirNode, MirParameterSlot,
    MirStateSlot,
};
pub use opt::{InvalidationClass, OptModel, OptOp, OptSchedule, OptValue, OptValueType};
```

- [ ] **Step 4: Run the green test**

Run:

```powershell
cargo test -p rspice-veriloga --test canonical_ir opt_lowering_builds_newton_schedule_from_mir_equations
```

Expected: PASS.

- [ ] **Step 5: Commit**

Run:

```powershell
git add crates/rspice-veriloga/src/canonical_ir/opt.rs crates/rspice-veriloga/src/canonical_ir/mod.rs crates/rspice-veriloga/tests/canonical_ir.rs
git commit -m "feat(veriloga): add canonical opt ir schedules"
```

---

### Task 6: Add Canonical Artifact And Deterministic Dumps

**Files:**
- Create: `crates/rspice-veriloga/src/canonical_ir/artifact.rs`
- Modify: `crates/rspice-veriloga/src/canonical_ir/mod.rs`
- Test: `crates/rspice-veriloga/tests/canonical_ir.rs`

- [ ] **Step 1: Add the failing artifact dump test**

Append to `crates/rspice-veriloga/tests/canonical_ir.rs`:

```rust
use rspice_veriloga::canonical_ir::CanonicalIrArtifact;

#[test]
fn artifact_dump_is_deterministic_and_contains_phase_summaries() {
    let analyzed = analyze_fixture(tiny_resistor_source(), "tiny_res").expect("analyze fixture");
    let metadata = CanonicalMetadata::for_source("fixture", tiny_resistor_source());
    let hir = HirModel::from_analyzed_module(&metadata, &analyzed);
    let mir = MirModel::from_hir(&hir).expect("lower MIR");
    let opt = OptModel::from_mir(&mir).expect("lower OptIR");
    let artifact =
        CanonicalIrArtifact::from_parts(metadata, hir, mir, opt).expect("artifact");

    let dump_a = artifact.dump_text();
    let dump_b = artifact.dump_text();
    assert_eq!(dump_a, dump_b);
    assert!(dump_a.contains("schema_version=1"));
    assert!(dump_a.contains("hir module=tiny_res ports=2 parameters=1 contributions=1"));
    assert!(dump_a.contains("mir nodes=2 equations=1"));
    assert!(dump_a.contains("opt schedules="));
}
```

- [ ] **Step 2: Run the red test**

Run:

```powershell
cargo test -p rspice-veriloga --test canonical_ir artifact_dump_is_deterministic_and_contains_phase_summaries
```

Expected: FAIL with unresolved import for `CanonicalIrArtifact`.

- [ ] **Step 3: Add artifact implementation**

Create `crates/rspice-veriloga/src/canonical_ir/artifact.rs`:

```rust
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

use super::{
    CanonicalMetadata, HirModel, IrDiagnostic, MirModel, OptModel, StableDigest,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanonicalIrArtifact {
    pub metadata: CanonicalMetadata,
    pub hir_digest: SmolStr,
    pub mir_digest: SmolStr,
    pub opt_digest: SmolStr,
    pub hir: HirModel,
    pub mir: MirModel,
    pub opt: OptModel,
}

impl CanonicalIrArtifact {
    pub fn from_parts(
        metadata: CanonicalMetadata,
        hir: HirModel,
        mir: MirModel,
        opt: OptModel,
    ) -> Result<Self, Vec<IrDiagnostic>> {
        hir.validate()?;
        mir.validate()?;
        opt.validate()?;

        let hir_summary = format!(
            "hir:{}:{}:{}:{}",
            hir.module_name,
            hir.ports.len(),
            hir.parameters.len(),
            hir.contributions.len()
        );
        let mir_summary = format!(
            "mir:{}:{}:{}",
            mir.module_name,
            mir.nodes.len(),
            mir.equations.len()
        );
        let opt_summary = format!("opt:{}:{}", opt.module_name, opt.schedules.len());

        Ok(Self {
            metadata,
            hir_digest: StableDigest::from_text(&hir_summary).as_hex().into(),
            mir_digest: StableDigest::from_text(&mir_summary).as_hex().into(),
            opt_digest: StableDigest::from_text(&opt_summary).as_hex().into(),
            hir,
            mir,
            opt,
        })
    }

    pub fn dump_text(&self) -> String {
        let mut out = String::new();
        out.push_str("canonical-veriloga-ir\n");
        out.push_str(&format!(
            "schema_version={}\nsource_package={}\nsource_digest={}\ncompiler_version={}\n",
            self.metadata.schema_version,
            self.metadata.source_package,
            self.metadata.source_digest,
            self.metadata.compiler_version
        ));
        out.push_str(&format!("hir_digest={}\n", self.hir_digest));
        out.push_str(&format!("mir_digest={}\n", self.mir_digest));
        out.push_str(&format!("opt_digest={}\n", self.opt_digest));
        out.push_str(&format!(
            "hir module={} ports={} parameters={} contributions={}\n",
            self.hir.module_name,
            self.hir.ports.len(),
            self.hir.parameters.len(),
            self.hir.contributions.len()
        ));
        out.push_str(&format!(
            "mir nodes={} equations={}\n",
            self.mir.nodes.len(),
            self.mir.equations.len()
        ));
        out.push_str(&format!("opt schedules={}\n", self.opt.schedules.len()));
        out
    }
}
```

Modify `crates/rspice-veriloga/src/canonical_ir/mod.rs`:

```rust
pub mod artifact;
pub mod diagnostic;
pub mod hir;
pub mod ids;
pub mod metadata;
pub mod mir;
pub mod opt;

pub use artifact::CanonicalIrArtifact;
pub use diagnostic::{
    CompilerPhase, DiagnosticSeverity, IrDiagnostic, IrValidationResult, SourceSpanRef,
};
pub use hir::{
    CanonicalValueType, HirArray, HirBranch, HirContribution, HirContributionKind, HirExprRef,
    HirModel, HirParamRange, HirParameter, HirPort, HirStatement, HirVariable,
};
pub use ids::{
    ArrayId, BranchId, BranchUnknownId, ContributionId, DisciplineId, EquationId, ExprId,
    ModuleId, NodeId, NoiseSourceId, ParamId, PortId, RegionId, ScheduleId, SourceId, StateId,
    SymbolId, ValueId, VariableId,
};
pub use metadata::{CanonicalMetadata, StableDigest};
pub use mir::{
    MirAnalysisDomain, MirEquation, MirEquationKind, MirModel, MirNode, MirParameterSlot,
    MirStateSlot,
};
pub use opt::{InvalidationClass, OptModel, OptOp, OptSchedule, OptValue, OptValueType};
```

- [ ] **Step 4: Run the green test**

Run:

```powershell
cargo test -p rspice-veriloga --test canonical_ir artifact_dump_is_deterministic_and_contains_phase_summaries
```

Expected: PASS.

- [ ] **Step 5: Commit**

Run:

```powershell
git add crates/rspice-veriloga/src/canonical_ir/artifact.rs crates/rspice-veriloga/src/canonical_ir/mod.rs crates/rspice-veriloga/tests/canonical_ir.rs
git commit -m "feat(veriloga): add canonical ir artifacts"
```

---

### Task 7: Add Public Canonical IR Compiler API

**Files:**
- Modify: `crates/rspice-veriloga/src/lib.rs`
- Test: `crates/rspice-veriloga/tests/canonical_ir.rs`

- [ ] **Step 1: Add the failing public API test**

Append to `crates/rspice-veriloga/tests/canonical_ir.rs`:

```rust
#[test]
fn compiler_can_emit_canonical_ir_without_bytecode_runtime() {
    let compiler = rspice_veriloga::VerilogACompiler::default();
    let artifact = compiler
        .compile_canonical_ir(tiny_resistor_source())
        .expect("canonical IR");

    assert_eq!(artifact.hir.module_name.as_str(), "tiny_res");
    assert_eq!(artifact.mir.equations.len(), 1);
    assert!(artifact.dump_text().contains("canonical-veriloga-ir"));
}
```

- [ ] **Step 2: Run the red test**

Run:

```powershell
cargo test -p rspice-veriloga --test canonical_ir compiler_can_emit_canonical_ir_without_bytecode_runtime
```

Expected: FAIL with `no method named compile_canonical_ir found for struct VerilogACompiler`.

- [ ] **Step 3: Add canonical IR entry points**

Modify `crates/rspice-veriloga/src/lib.rs` by adding this public type near `CompiledFile`:

```rust
/// Result of compiling a Verilog-A source file into the canonical IR.
#[derive(Debug, Clone)]
pub struct CanonicalIrFile {
    pub artifact: canonical_ir::CanonicalIrArtifact,
    pub dependencies: Vec<std::path::PathBuf>,
}
```

Add these methods inside `impl VerilogACompiler`:

```rust
    /// Compile Verilog-A source code to the canonical multi-level IR.
    pub fn compile_canonical_ir(
        &self,
        source: &str,
    ) -> CompileResult<canonical_ir::CanonicalIrArtifact> {
        self.compile_canonical_ir_module(source, None)
    }

    /// Compile one module from Verilog-A source code to the canonical multi-level IR.
    pub fn compile_canonical_ir_module(
        &self,
        source: &str,
        module_name: Option<&str>,
    ) -> CompileResult<canonical_ir::CanonicalIrArtifact> {
        let mut pp = self.configured_preprocessor();
        let preprocessed = pp
            .preprocess_source(source)
            .map_err(|e| CompileError::io_error(format!("Preprocessor error: {}", e)))?;
        self.compile_canonical_ir_preprocessed(&preprocessed, module_name)
    }

    fn compile_canonical_ir_preprocessed(
        &self,
        source: &str,
        module_name: Option<&str>,
    ) -> CompileResult<canonical_ir::CanonicalIrArtifact> {
        let source_map = SourceMap::new();
        let source_id = source_map.add_source("<input>", source);
        let tokens = Lexer::new(source, source_id).collect_tokens()?;
        let source_file = Parser::new(&tokens).parse()?;
        let analyzed = SemanticAnalyzer::new().analyze(&source_file)?;
        let module = self.select_analyzed_module(&analyzed, module_name)?;
        let metadata = canonical_ir::CanonicalMetadata::for_source("<input>", source);
        let hir = canonical_ir::HirModel::from_analyzed_module(&metadata, module);
        let mir = canonical_ir::MirModel::from_hir(&hir).map_err(Self::canonical_ir_error)?;
        let opt = canonical_ir::OptModel::from_mir(&mir).map_err(Self::canonical_ir_error)?;
        canonical_ir::CanonicalIrArtifact::from_parts(metadata, hir, mir, opt)
            .map_err(Self::canonical_ir_error)
    }

    fn select_analyzed_module<'a>(
        &self,
        analyzed: &'a semantic::AnalyzedFile,
        module_name: Option<&str>,
    ) -> CompileResult<&'a semantic::AnalyzedModule> {
        if let Some(module_name) = module_name {
            analyzed
                .modules
                .get(module_name)
                .ok_or_else(|| CompileError::ModuleSelection(module_name.to_string()))
        } else if analyzed.modules.len() == 1 {
            Ok(analyzed.modules.values().next().expect("one module"))
        } else {
            let names = analyzed
                .modules
                .keys()
                .map(|name| name.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            Err(CompileError::ModuleSelection(format!(
                "multiple modules found; choose one of: {}",
                names
            )))
        }
    }

    fn canonical_ir_error(diagnostics: Vec<canonical_ir::IrDiagnostic>) -> CompileError {
        let message = diagnostics
            .into_iter()
            .map(|diagnostic| diagnostic.to_string())
            .collect::<Vec<_>>()
            .join("; ");
        CompileError::CodeGen(error::CodeGenError::new(
            error::CodeGenErrorKind::Internal(message),
        ))
    }
```

Add file-based entry points after `compile_file_module_with_metadata`:

```rust
    /// Compile a source file from disk into the canonical IR with dependency metadata.
    pub fn compile_file_canonical_ir_with_metadata(
        &self,
        path: &std::path::Path,
        module_name: Option<&str>,
    ) -> CompileResult<CanonicalIrFile> {
        let mut pp = self.configured_preprocessor();
        let preprocessed = pp
            .preprocess_file(path)
            .map_err(|e| CompileError::io_error(format!("Preprocessor error: {}", e)))?;
        let dependencies = pp.take_dependencies();
        let mut artifact = self.compile_canonical_ir_preprocessed(&preprocessed, module_name)?;
        artifact.metadata.source_package = path.display().to_string().into();
        Ok(CanonicalIrFile {
            artifact,
            dependencies,
        })
    }
```

- [ ] **Step 4: Run the green public API test**

Run:

```powershell
cargo test -p rspice-veriloga --test canonical_ir compiler_can_emit_canonical_ir_without_bytecode_runtime
```

Expected: PASS.

- [ ] **Step 5: Run the full canonical IR test file**

Run:

```powershell
cargo test -p rspice-veriloga --test canonical_ir
```

Expected: PASS.

- [ ] **Step 6: Commit**

Run:

```powershell
git add crates/rspice-veriloga/src/lib.rs crates/rspice-veriloga/tests/canonical_ir.rs
git commit -m "feat(veriloga): expose canonical ir compiler api"
```

---

### Task 8: Final Verification

**Files:**
- All files touched by Tasks 1-7.

- [ ] **Step 1: Format the touched Rust files**

Run:

```powershell
cargo fmt --package rspice-veriloga
```

Expected: command exits 0.

- [ ] **Step 2: Run focused canonical IR tests**

Run:

```powershell
cargo test -p rspice-veriloga --test canonical_ir
```

Expected: PASS.

- [ ] **Step 3: Run crate tests**

Run:

```powershell
cargo test -p rspice-veriloga
```

Expected: PASS. If unrelated shared-worktree changes make this fail, capture the first failing test name and the exact error before changing code.

- [ ] **Step 4: Check the staged diff before the final commit**

Run:

```powershell
git status --short
git diff -- crates/rspice-veriloga/src/canonical_ir crates/rspice-veriloga/src/lib.rs crates/rspice-veriloga/tests/canonical_ir.rs
```

Expected: the diff is limited to the canonical IR module, the public API additions in `lib.rs`, and the canonical IR integration test.

- [ ] **Step 5: Commit formatting or final cleanups**

Run only if Step 1 changed files after the previous task commit:

```powershell
git add crates/rspice-veriloga/src/canonical_ir crates/rspice-veriloga/src/lib.rs crates/rspice-veriloga/tests/canonical_ir.rs
git commit -m "chore(veriloga): format canonical ir foundation"
```

Expected: commit succeeds, or Git reports there is nothing to commit.

---

## Self-Review

- Spec coverage: Tasks 1-7 implement the approved IR foundation slice: typed IDs, metadata, diagnostics, HIR, MIR, OptIR, deterministic dumps, validators, and a compiler API. Generated Rust and the custom JIT remain downstream consumers as required by the spec.
- Scope check: this plan does not replace bytecode runtime behavior, does not remove Cranelift, and does not claim CMC support.
- Type consistency: all task code uses the same exported names: `CanonicalMetadata`, `HirModel`, `MirModel`, `OptModel`, `CanonicalIrArtifact`, `IrDiagnostic`, and `InvalidationClass`.
- Verification: focused tests are introduced before each implementation step, and final verification runs both the canonical IR test file and the full `rspice-veriloga` test suite.
