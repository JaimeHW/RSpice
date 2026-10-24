//! Authored independent-phase small-signal card; numerical defaults are resolved by the engine.
use super::{FreqVariation, PeriodicSweep};
use crate::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum QpacSweep {
    Generated(PeriodicSweep),
    Explicit(Vec<Value>),
}

impl Default for QpacSweep {
    fn default() -> Self {
        Self::Explicit(Vec::new())
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct QpacCard {
    pub sweep: QpacSweep,
    pub input_source: String,
    pub output_node: String,
    pub output_ref: String,
    pub input_lattice: Vec<i32>,
    pub output_lattice: Vec<i32>,
    pub magnitude: Option<Value>,
    pub phase_degrees: Option<Value>,
    pub linear_solver: Option<String>,
    pub krylov_restart: Option<usize>,
    pub krylov_cycles: Option<usize>,
    pub linear_tolerance: Option<Value>,
    pub current_absolute_tolerance: Option<Value>,
    pub voltage_absolute_tolerance: Option<Value>,
}

impl QpacCard {
    /// Render authored fields without rounding numeric values or inserting
    /// numerical defaults. Validate public AST values at the engine boundary.
    pub fn to_spice(&self) -> String {
        let sweep = match &self.sweep {
            QpacSweep::Generated(s) => format!(
                "{} {} {} {}",
                match s.variation {
                    FreqVariation::Lin => "LIN",
                    FreqVariation::Dec => "DEC",
                    FreqVariation::Oct => "OCT",
                },
                s.points,
                s.start_freq,
                s.stop_freq
            ),
            QpacSweep::Explicit(values) => format!("LIST=({})", list(values)),
        };
        let mut fields = vec![format!(
            ".QPAC {sweep} SOURCE={} OUT=V({},{}) INLATTICE=({}) OUTLATTICE=({})",
            name(&self.input_source),
            name(&self.output_node),
            name(&self.output_ref),
            list(&self.input_lattice),
            list(&self.output_lattice)
        )];
        if let Some(method) = &self.linear_solver {
            fields.push(format!("SOLVER={method}"));
        }
        for (key, value) in [
            ("KRYLOVRESTART", self.krylov_restart),
            ("KRYLOVCYCLES", self.krylov_cycles),
        ] {
            if let Some(value) = value {
                fields.push(format!("{key}={value}"));
            }
        }
        for (key, value) in [
            ("MAG", self.magnitude),
            ("PHASE", self.phase_degrees),
            ("LINEARTOL", self.linear_tolerance),
            ("IABSTOL", self.current_absolute_tolerance),
            ("VABSTOL", self.voltage_absolute_tolerance),
        ] {
            if let Some(value) = value {
                fields.push(format!("{key}={value}"));
            }
        }
        fields.join(" ")
    }
}

fn list<T: ToString>(values: &[T]) -> String {
    values
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

fn name(value: &str) -> String {
    format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
}
