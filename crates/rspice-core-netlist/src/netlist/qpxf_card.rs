//! Native QPXF authoring: output-frequency sweep, sources, sidebands and observation.
use super::{FreqVariation, QpacSweep};
use crate::Value;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum QpxfCardSources {
    #[default]
    AllIndependent,
    Named(Vec<String>),
}
#[derive(Debug, Clone, PartialEq, Default)]
pub enum QpxfCardLattices {
    #[default]
    AllRetained,
    Explicit(Vec<Vec<i32>>),
    MaxOrders(Vec<usize>),
}
#[derive(Debug, Clone, PartialEq)]
pub enum QpxfCardOutput {
    Voltage { positive: String, negative: String },
    BranchCurrent { branch: String },
}
impl Default for QpxfCardOutput {
    fn default() -> Self {
        Self::Voltage {
            positive: String::new(),
            negative: "0".into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
pub struct QpxfCard {
    pub sweep: QpacSweep,
    pub frequency_axis: Option<String>,
    pub input_sources: QpxfCardSources,
    pub input_lattices: QpxfCardLattices,
    pub output: QpxfCardOutput,
    pub output_lattice: Vec<i32>,
    pub linear_solver: Option<String>,
    pub krylov_restart: Option<usize>,
    pub krylov_cycles: Option<usize>,
    pub linear_tolerance: Option<Value>,
    pub group_delay: Option<bool>,
    pub group_delay_magnitude_floor: Option<Value>,
}

impl QpxfCard {
    /// Native syntax preserves authored sweep spelling and numerical precision.
    /// Validate programmatically authored cards at the engine boundary.
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
        let sources = match &self.input_sources {
            QpxfCardSources::AllIndependent => "ALL".into(),
            QpxfCardSources::Named(names) => format!(
                "({})",
                names.iter().map(|s| name(s)).collect::<Vec<_>>().join(",")
            ),
        };
        let output = match &self.output {
            QpxfCardOutput::Voltage { positive, negative } => {
                format!("V({},{})", name(positive), name(negative))
            }
            QpxfCardOutput::BranchCurrent { branch } => format!("I({})", name(branch)),
        };
        let selection = match &self.input_lattices {
            QpxfCardLattices::AllRetained => "INLATTICES=ALL".into(),
            QpxfCardLattices::Explicit(tuples) => format!(
                "INLATTICES=({})",
                tuples
                    .iter()
                    .map(|tuple| format!("({})", list(tuple)))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            QpxfCardLattices::MaxOrders(orders) => format!("MAXORDERS=({})", list(orders)),
        };
        let mut fields = vec![format!(
            ".QPXF {sweep} SOURCES={sources} OUT={output} {selection} OUTLATTICE=({})",
            list(&self.output_lattice)
        )];
        if let Some(axis) = &self.frequency_axis {
            fields.push(format!("AXIS={axis}"));
        }
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
            ("LINEARTOL", self.linear_tolerance),
            ("GDFLOOR", self.group_delay_magnitude_floor),
        ] {
            if let Some(value) = value {
                fields.push(format!("{key}={value}"));
            }
        }
        if let Some(value) = self.group_delay {
            fields.push(format!("GROUPDELAY={}", if value { "YES" } else { "NO" }));
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
