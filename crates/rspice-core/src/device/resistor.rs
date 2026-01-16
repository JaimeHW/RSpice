//! Resistor device model

use crate::{circuit::NodeId, Value};
use super::traits::{LinearDevice, MatrixStamper};

/// Linear resistor
#[derive(Debug, Clone)]
pub struct Resistor {
    pub name: String,
    pub node_pos: NodeId,
    pub node_neg: NodeId,
    pub resistance: Value,
}

impl Resistor {
    pub fn new(name: String, node_pos: NodeId, node_neg: NodeId, resistance: Value) -> Self {
        assert!(resistance > 0.0, "Resistance must be positive");
        Self {
            name,
            node_pos,
            node_neg,
            resistance,
        }
    }

    /// Get conductance (1/R)
    pub fn conductance(&self) -> Value {
        1.0 / self.resistance
    }
}

impl LinearDevice for Resistor {
    fn stamp_linear(
        &self,
        matrix: &mut impl MatrixStamper,
        _rhs: &mut [Value],
    ) {
        let g = self.conductance();
        
        // Standard conductance stamp pattern
        // G(n+, n+) += g
        // G(n+, n-) -= g
        // G(n-, n+) -= g
        // G(n-, n-) += g
        
        matrix.stamp(self.node_pos, self.node_pos, g);
        matrix.stamp(self.node_pos, self.node_neg, -g);
        matrix.stamp(self.node_neg, self.node_pos, -g);
        matrix.stamp(self.node_neg, self.node_neg, g);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resistor_conductance() {
        let r = Resistor::new("R1".to_string(), 1, 0, 1000.0);
        assert!((r.conductance() - 0.001).abs() < 1e-10);
    }
}
