//! Native MEXTRAM 504.12.1 compact model shell.

mod eval;
mod params;
mod temp;

use crate::Value;
use crate::circuit::NodeId;
use crate::solver::CscIndex;
use std::cell::Cell;

pub use params::{Mextram504Model, Mextram504Polarity};

#[derive(Debug, Clone, Copy)]
pub struct Mextram504Nodes {
    pub c: NodeId,
    pub b: NodeId,
    pub e: NodeId,
    pub s: NodeId,
    pub e1: NodeId,
    pub b1: NodeId,
    pub b2: NodeId,
    pub c1: NodeId,
    pub c2: NodeId,
    pub c3: NodeId,
    pub c4: NodeId,
    pub noi: NodeId,
}

impl Mextram504Nodes {
    pub fn xyce_static_node_order(self) -> [NodeId; 12] {
        [
            self.c, self.b, self.e, self.s, self.e1, self.b1, self.b2, self.c1, self.c2, self.c3,
            self.c4, self.noi,
        ]
    }
}

#[derive(Debug, Clone, Default)]
pub struct Mextram504Indices {
    pub slots: Vec<(NodeId, NodeId, Option<CscIndex>)>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Mextram504Op {
    pub source_c: Value,
    pub source_b: Value,
    pub source_e: Value,
    pub source_s: Value,
}

#[derive(Debug, Clone)]
pub struct Mextram504 {
    pub name: String,
    pub nodes: Mextram504Nodes,
    pub model: Mextram504Model,
    pub indices: Mextram504Indices,
    last_op: Cell<Mextram504Op>,
}

impl Mextram504 {
    pub fn new(name: String, nodes: Mextram504Nodes, model: Mextram504Model) -> Self {
        Self {
            name,
            nodes,
            model,
            indices: Mextram504Indices::default(),
            last_op: Cell::new(Mextram504Op::default()),
        }
    }

    pub fn op_at_solution(&self, voltages: &[Value]) -> Mextram504Op {
        let op = eval::evaluate_dc(&self.model, self.nodes, voltages);
        self.last_op.set(op);
        op
    }

    pub fn last_op(&self) -> Mextram504Op {
        self.last_op.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    #[should_panic(expected = "native MEXTRAM 504 evaluator is not complete")]
    fn op_at_solution_fails_closed_while_evaluator_is_incomplete() {
        let nodes = Mextram504Nodes {
            c: 1,
            b: 2,
            e: 0,
            s: 0,
            e1: 3,
            b1: 4,
            b2: 5,
            c1: 6,
            c2: 7,
            c3: 8,
            c4: 9,
            noi: 10,
        };
        let model =
            Mextram504Model::from_params(&HashMap::new(), &HashMap::new(), Mextram504Polarity::Npn);
        let device = Mextram504::new("q1".to_string(), nodes, model);

        let _ = device.op_at_solution(&[0.0; 12]);
    }

    #[test]
    fn xyce_static_node_order_includes_noise_node() {
        let nodes = Mextram504Nodes {
            c: 1,
            b: 2,
            e: 3,
            s: 4,
            e1: 5,
            b1: 6,
            b2: 7,
            c1: 8,
            c2: 9,
            c3: 10,
            c4: 11,
            noi: 12,
        };

        assert_eq!(
            nodes.xyce_static_node_order(),
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
        );
    }
}
