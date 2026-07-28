//! The Cranelift JIT, judged against the CFG reference interpreter.
//!
//! A backend cannot check itself, so every case here builds one CFG, runs it
//! through [`evaluate_cfg`] and through the JIT, and compares. The interpreter
//! is the oracle Phase 2 built for exactly this, and the two share no lowering
//! code — the JIT emits machine code and the interpreter walks the graph.
//!
//! Agreement is demanded **exactly**, not to a tolerance. The transcendental
//! shims dispatch to the same `CfgScalar for f64` implementation the
//! interpreter calls, and every other operation is IEEE arithmetic on the same
//! operands in the same order, so any difference at all is a lowering bug
//! rather than rounding. A tolerance here would hide precisely what the test
//! exists to find.

#![cfg(feature = "cranelift")]

use std::collections::{HashMap, HashSet};

use rspice_veriloga::canonical_ir::cfg::{
    CfgBinaryOp, CfgBlock, CfgFunction, CfgInstruction, CfgTerminator, CfgUnaryOp, CfgValue,
    CfgValueKind, CfgValueType,
};
use rspice_veriloga::canonical_ir::cfg_eval::{CfgEvalInputs, evaluate};
use rspice_veriloga::canonical_ir::ids::{BlockId, NodeId, ParamId, ValueId};
use rspice_veriloga::cranelift_backend::{CfgJitInputs, CfgJitScalars, CraneliftError, compile};

/// Builds a single-block or multi-block CFG without the ceremony of `SsaBuilder`,
/// which is aimed at lowering a source body rather than at hand-written cases.
struct Fixture {
    values: Vec<CfgValue>,
    blocks: Vec<CfgBlock>,
}

impl Fixture {
    fn new() -> Self {
        Self {
            values: Vec::new(),
            blocks: Vec::new(),
        }
    }

    fn value(&mut self, kind: CfgValueKind) -> ValueId {
        let id = ValueId::from(self.values.len());
        self.values.push(CfgValue {
            id,
            kind,
            value_type: CfgValueType::Real,
        });
        id
    }

    fn block(&mut self, params: Vec<ValueId>) -> BlockId {
        let id = BlockId::from(self.blocks.len());
        self.blocks.push(CfgBlock {
            id,
            params,
            instructions: Vec::new(),
            terminator: CfgTerminator::Unset,
        });
        id
    }

    /// Define `kind` inside `block`.
    fn emit(&mut self, block: BlockId, kind: CfgValueKind) -> ValueId {
        let id = self.value(kind);
        self.blocks[usize::from(block)]
            .instructions
            .push(CfgInstruction { result: id });
        id
    }

    fn terminate(&mut self, block: BlockId, terminator: CfgTerminator) {
        self.blocks[usize::from(block)].terminator = terminator;
    }

    fn finish(self, entry: BlockId) -> CfgFunction {
        CfgFunction {
            entry,
            blocks: self.blocks,
            values: self.values,
            shapes: Vec::new(),
        }
    }
}

fn eval_inputs(parameters: Vec<f64>, nodes: Vec<f64>) -> CfgEvalInputs<f64> {
    CfgEvalInputs {
        parameter_given: vec![false; parameters.len()],
        parameters,
        node_potentials: nodes,
        branch_flows: Vec::new(),
        branch_unknown_flows: Vec::new(),
        temperature: 300.15,
        thermal_voltage: 0.025_852_0,
        multiplicity: 1.0,
        time: 0.0,
        analyses: HashSet::new(),
        simparams: HashMap::new(),
        ddt: 0.0,
        ddt_scale: 0.0,
        idt: 0.0,
        idt_scale: 0.0,
        staged: Vec::new(),
    }
}

fn jit_inputs(parameters: Vec<f64>, nodes: Vec<f64>) -> CfgJitInputs {
    CfgJitInputs {
        parameters,
        node_potentials: nodes,
        branch_flows: Vec::new(),
        branch_unknown_flows: Vec::new(),
        scalars: CfgJitScalars::default(),
    }
}

/// Run both engines on the same inputs and require bit-identical exports.
fn agree(function: &CfgFunction, exports: &[ValueId], parameters: Vec<f64>, nodes: Vec<f64>) {
    let snapshot = evaluate(function, &eval_inputs(parameters.clone(), nodes.clone()))
        .expect("the interpreter must evaluate the fixture");
    let expected: Vec<f64> = exports
        .iter()
        .map(|value| {
            snapshot
                .value(*value)
                .unwrap_or_else(|| panic!("the interpreter left {value} undefined"))
        })
        .collect();

    let compiled = compile(function, exports).expect("the fixture must lower");
    let actual = compiled.call(&jit_inputs(parameters.clone(), nodes.clone()));

    assert_eq!(
        actual.len(),
        expected.len(),
        "the JIT must write one entry per export"
    );
    for (index, (got, want)) in actual.iter().zip(expected.iter()).enumerate() {
        assert_eq!(
            got.to_bits(),
            want.to_bits(),
            "export {index} disagrees: JIT {got:e}, interpreter {want:e} \
             (parameters {parameters:?}, nodes {nodes:?})"
        );
    }
}

#[test]
fn arithmetic_agrees_with_the_interpreter() {
    let mut fixture = Fixture::new();
    let entry = fixture.block(Vec::new());

    let p0 = fixture.emit(entry, CfgValueKind::Parameter(ParamId::from(0usize)));
    let v0 = fixture.emit(entry, CfgValueKind::NodePotential(NodeId::from(0usize)));
    let v1 = fixture.emit(entry, CfgValueKind::NodePotential(NodeId::from(1usize)));

    let diff = fixture.emit(
        entry,
        CfgValueKind::Binary {
            op: CfgBinaryOp::Sub,
            left: v0,
            right: v1,
        },
    );
    let scaled = fixture.emit(
        entry,
        CfgValueKind::Binary {
            op: CfgBinaryOp::Mul,
            left: diff,
            right: p0,
        },
    );
    let quotient = fixture.emit(
        entry,
        CfgValueKind::Binary {
            op: CfgBinaryOp::Div,
            left: scaled,
            right: p0,
        },
    );
    let negated = fixture.emit(
        entry,
        CfgValueKind::Unary {
            op: CfgUnaryOp::Neg,
            input: quotient,
        },
    );
    let magnitude = fixture.emit(
        entry,
        CfgValueKind::Unary {
            op: CfgUnaryOp::Abs,
            input: negated,
        },
    );
    fixture.terminate(entry, CfgTerminator::Return);

    let function = fixture.finish(entry);
    let exports = [diff, scaled, quotient, negated, magnitude];
    for (parameter, left, right) in [
        (2.0, 0.7, 0.2),
        (1e-9, -0.55, 0.85),
        (1e6, 0.0, 0.0),
        (-3.25, 1.5, -1.5),
    ] {
        agree(&function, &exports, vec![parameter], vec![left, right]);
    }
}

#[test]
fn transcendentals_agree_bit_for_bit() {
    // Bit-exactness is the whole point of routing the shims through
    // `CfgScalar for f64`: if the JIT called libm directly and the interpreter
    // called Rust's `f64::exp`, they could differ in the last place and no
    // tolerance would tell that apart from a real lowering bug.
    let ops = [
        CfgUnaryOp::Exp,
        CfgUnaryOp::LimExp,
        CfgUnaryOp::Ln,
        CfgUnaryOp::Sqrt,
        CfgUnaryOp::Sin,
        CfgUnaryOp::Cos,
        CfgUnaryOp::Tan,
        CfgUnaryOp::Sinh,
        CfgUnaryOp::Cosh,
        CfgUnaryOp::Tanh,
        CfgUnaryOp::Atan,
        CfgUnaryOp::Asinh,
        CfgUnaryOp::Floor,
        CfgUnaryOp::Ceil,
    ];

    let mut fixture = Fixture::new();
    let entry = fixture.block(Vec::new());
    let input = fixture.emit(entry, CfgValueKind::NodePotential(NodeId::from(0usize)));
    let mut exports = Vec::new();
    for op in ops {
        exports.push(fixture.emit(entry, CfgValueKind::Unary { op, input }));
    }
    fixture.terminate(entry, CfgTerminator::Return);

    let function = fixture.finish(entry);
    // Positive only: `ln` and `sqrt` of a negative are NaN in both engines,
    // and NaN compares unequal by bits in a way that says nothing.
    for probe in [0.5, 1.0, 2.5, 40.0, 120.0] {
        agree(&function, &exports, Vec::new(), vec![probe]);
    }
}

#[test]
fn a_guard_and_its_merge_agree() {
    // The case the Rust emitter has to work around with captured stage exports,
    // and which Cranelift takes directly: a value defined in one arm and read
    // after the join, carried as a block parameter.
    let mut fixture = Fixture::new();
    let entry = fixture.block(Vec::new());
    let then_block = fixture.block(Vec::new());
    let else_block = fixture.block(Vec::new());

    let merged = fixture.value(CfgValueKind::BlockParameter);
    let join = fixture.block(vec![merged]);

    let bias = fixture.emit(entry, CfgValueKind::NodePotential(NodeId::from(0usize)));
    let threshold = fixture.emit(entry, CfgValueKind::RealConstant(0.5));
    let over = fixture.emit(
        entry,
        CfgValueKind::Binary {
            op: CfgBinaryOp::Gt,
            left: bias,
            right: threshold,
        },
    );
    fixture.terminate(
        entry,
        CfgTerminator::Branch {
            condition: over,
            then_target: then_block,
            then_args: Vec::new(),
            else_target: else_block,
            else_args: Vec::new(),
        },
    );

    let strong = fixture.emit(
        then_block,
        CfgValueKind::Unary {
            op: CfgUnaryOp::Exp,
            input: bias,
        },
    );
    fixture.terminate(
        then_block,
        CfgTerminator::Jump {
            target: join,
            args: vec![strong],
        },
    );

    let weak = fixture.emit(
        else_block,
        CfgValueKind::Binary {
            op: CfgBinaryOp::Mul,
            left: bias,
            right: bias,
        },
    );
    fixture.terminate(
        else_block,
        CfgTerminator::Jump {
            target: join,
            args: vec![weak],
        },
    );

    let doubled = fixture.emit(
        join,
        CfgValueKind::Binary {
            op: CfgBinaryOp::Add,
            left: merged,
            right: merged,
        },
    );
    fixture.terminate(join, CfgTerminator::Return);

    let function = fixture.finish(entry);
    let exports = [merged, doubled];
    // Both sides of the guard, and the boundary itself.
    for bias in [0.0, 0.5, 0.5000001, 1.0, -2.0] {
        agree(&function, &exports, Vec::new(), vec![bias]);
    }
}

#[test]
fn a_back_edge_agrees() {
    // A loop is a back edge in this IR and nothing else, so the lowering has to
    // handle a block whose predecessor comes after it. Cranelift needs its
    // blocks sealed only once every predecessor is known, which is why the
    // lowering seals at the end rather than as it goes; without that this case
    // would either trap or silently read an undefined block parameter.
    let mut fixture = Fixture::new();
    let entry = fixture.block(Vec::new());

    let counter = fixture.value(CfgValueKind::BlockParameter);
    let total = fixture.value(CfgValueKind::BlockParameter);
    let header = fixture.block(vec![counter, total]);
    let body = fixture.block(Vec::new());
    let exit = fixture.block(Vec::new());

    let zero = fixture.emit(entry, CfgValueKind::RealConstant(0.0));
    let seed = fixture.emit(entry, CfgValueKind::NodePotential(NodeId::from(0usize)));
    fixture.terminate(
        entry,
        CfgTerminator::Jump {
            target: header,
            args: vec![zero, seed],
        },
    );

    let limit = fixture.emit(header, CfgValueKind::RealConstant(5.0));
    let more = fixture.emit(
        header,
        CfgValueKind::Binary {
            op: CfgBinaryOp::Lt,
            left: counter,
            right: limit,
        },
    );
    fixture.terminate(
        header,
        CfgTerminator::Branch {
            condition: more,
            then_target: body,
            then_args: Vec::new(),
            else_target: exit,
            else_args: Vec::new(),
        },
    );

    let one = fixture.emit(body, CfgValueKind::RealConstant(1.0));
    let next_counter = fixture.emit(
        body,
        CfgValueKind::Binary {
            op: CfgBinaryOp::Add,
            left: counter,
            right: one,
        },
    );
    let next_total = fixture.emit(
        body,
        CfgValueKind::Binary {
            op: CfgBinaryOp::Mul,
            left: total,
            right: total,
        },
    );
    fixture.terminate(
        body,
        CfgTerminator::Jump {
            target: header,
            args: vec![next_counter, next_total],
        },
    );

    fixture.terminate(exit, CfgTerminator::Return);

    let function = fixture.finish(entry);
    for seed in [1.0000001, 0.5, 1.0] {
        agree(&function, &[counter, total], Vec::new(), vec![seed]);
    }
}

#[test]
fn an_unlowerable_kind_is_named_rather_than_approximated() {
    let mut fixture = Fixture::new();
    let entry = fixture.block(Vec::new());
    let input = fixture.emit(entry, CfgValueKind::NodePotential(NodeId::from(0usize)));
    let stored = fixture.emit(
        entry,
        CfgValueKind::Ddt {
            operator: rspice_veriloga::canonical_ir::ids::ExprId::from(0usize),
            input,
        },
    );
    fixture.terminate(entry, CfgTerminator::Return);

    let function = fixture.finish(entry);
    match compile(&function, &[stored]) {
        Err(CraneliftError::Unsupported(what)) => {
            assert!(
                what.contains("ddt"),
                "the diagnostic must name the kind, got {what:?}"
            );
        }
        Err(other) => panic!("expected an Unsupported diagnostic, got {other}"),
        Ok(_) => panic!(
            "a ddt has per-instance state and no runtime contract here yet; \
             lowering it silently would be a converged answer to the wrong equation"
        ),
    }
}
