//! Runtime ABI for build-time generated Verilog-A devices.
//!
//! Generated device modules call this small surface directly from their
//! hand-emitted Rust stamps. Keep it narrow, deterministic, and free of
//! interpreter concepts.

use crate::Value;
use crate::solver::StaticMatrix;

pub mod builtins {
    include!(concat!(env!("OUT_DIR"), "/veriloga_builtins/registry.rs"));
}

#[derive(Debug, Clone, Copy)]
pub struct GeneratedEvalContext<'a> {
    voltages: &'a [Value],
}

impl<'a> GeneratedEvalContext<'a> {
    #[inline]
    pub fn new(voltages: &'a [Value]) -> Self {
        Self { voltages }
    }

    #[inline]
    pub fn node_voltage(&self, node: usize) -> Value {
        if node == 0 {
            0.0
        } else {
            self.voltages.get(node).copied().unwrap_or(0.0)
        }
    }
}

pub struct GeneratedStamper<'a> {
    matrix: &'a mut StaticMatrix,
    rhs: &'a mut [Value],
    voltages: &'a [Value],
}

impl<'a> GeneratedStamper<'a> {
    #[inline]
    pub fn new(matrix: &'a mut StaticMatrix, rhs: &'a mut [Value], voltages: &'a [Value]) -> Self {
        Self {
            matrix,
            rhs,
            voltages,
        }
    }

    #[inline]
    pub fn stamp_current(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        derivatives: &[(usize, Value)],
    ) {
        let mut equivalent = value;
        for &(node, derivative) in derivatives {
            equivalent -= derivative * self.node_voltage(node);
        }

        if let Some(row) = pos {
            self.stamp_current_row(row, 1.0, equivalent, derivatives);
        }
        if let Some(row) = neg {
            self.stamp_current_row(row, -1.0, equivalent, derivatives);
        }
    }

    #[inline]
    fn stamp_current_row(
        &mut self,
        row_node: usize,
        row_sign: Value,
        equivalent: Value,
        derivatives: &[(usize, Value)],
    ) {
        if row_node == 0 {
            return;
        }
        let row = row_node - 1;
        for &(col_node, derivative) in derivatives {
            if col_node > 0 {
                self.matrix.add(row, col_node - 1, row_sign * derivative);
            }
        }
        if let Some(slot) = self.rhs.get_mut(row) {
            *slot += -row_sign * equivalent;
        }
    }

    #[inline]
    fn node_voltage(&self, node: usize) -> Value {
        if node == 0 {
            0.0
        } else {
            self.voltages.get(node).copied().unwrap_or(0.0)
        }
    }
}
