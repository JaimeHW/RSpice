//! Native multi-output QPNOISE authoring; numerical defaults resolve in the engine.
use super::{FreqVariation, QpacSweep, QpxfCardOutput};
use crate::Value;
#[derive(Debug, Clone, PartialEq, Default)]
pub enum QpnoiseCardLattices {
    #[default]
    AllRetained,
    Explicit(Vec<Vec<i32>>),
    MaxOrders(Vec<usize>),
    Range {
        minimum: Vec<i32>,
        maximum: Vec<i32>,
    },
}
#[derive(Debug, Clone, PartialEq, Default)]
pub enum QpnoiseCardSources {
    #[default]
    All,
    Only(Vec<String>),
    Except(Vec<String>),
}
#[derive(Debug, Clone, PartialEq)]
pub struct QpnoiseCardOutput {
    pub observation: QpxfCardOutput,
    pub lattice: Vec<i32>,
}
#[derive(Debug, Clone, PartialEq, Default)]
pub struct QpnoiseCard {
    pub sweep: QpacSweep,
    pub frequency_axis: Option<String>,
    pub outputs: Vec<QpnoiseCardOutput>,
    pub input_source: Option<String>,
    pub input_lattice: Option<Vec<i32>>,
    pub noise_lattices: QpnoiseCardLattices,
    pub noise_sources: QpnoiseCardSources,
    pub integrated_noise: Option<bool>,
    pub integration_band: Option<[Value; 2]>,
    pub integration_method: Option<String>,
    pub contributor_ranking: Option<bool>,
    pub noise_figure: Option<bool>,
    pub source_resistor: Option<String>,
    pub reference_temperature: Option<Value>,
    pub reference_lattices: Option<Vec<Vec<i32>>>,
    pub linear_solver: Option<String>,
    pub krylov_restart: Option<usize>,
    pub krylov_cycles: Option<usize>,
    pub linear_tolerance: Option<Value>,
}
impl QpnoiseCard {
    pub fn to_spice(&self) -> String {
        let sweep = match &self.sweep {
            QpacSweep::Explicit(values) => format!("LIST=({})", list(values)),
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
        };
        let outputs = self
            .outputs
            .iter()
            .map(|o| {
                format!(
                    "({},({}))",
                    match &o.observation {
                        QpxfCardOutput::Voltage { positive, negative } =>
                            format!("V({},{})", name(positive), name(negative)),
                        QpxfCardOutput::BranchCurrent { branch } => format!("I({})", name(branch)),
                    },
                    list(&o.lattice)
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        let mut fields = vec![format!(".QPNOISE {sweep} OUTS=({outputs})")];
        if let Some(source) = &self.input_source {
            fields.push(format!("SOURCE={}", name(source)));
        }
        if let Some(tuple) = &self.input_lattice {
            fields.push(format!("INLATTICE=({})", list(tuple)));
        }
        fields.push(match &self.noise_lattices {
            QpnoiseCardLattices::AllRetained => "NOISELATTICES=ALL".into(),
            QpnoiseCardLattices::Explicit(tuples) => {
                format!("NOISELATTICES=({})", tuple_list(tuples))
            }
            QpnoiseCardLattices::MaxOrders(orders) => format!("MAXORDERS=({})", list(orders)),
            QpnoiseCardLattices::Range { minimum, maximum } => format!(
                "MINLATTICE=({}) MAXLATTICE=({})",
                list(minimum),
                list(maximum)
            ),
        });
        fields.push(match &self.noise_sources {
            QpnoiseCardSources::All => "NOISESOURCES=ALL".into(),
            QpnoiseCardSources::Only(names) => format!(
                "NOISESOURCES=({})",
                names.iter().map(|s| name(s)).collect::<Vec<_>>().join(",")
            ),
            QpnoiseCardSources::Except(names) => format!(
                "EXCLUDESOURCES=({})",
                names.iter().map(|s| name(s)).collect::<Vec<_>>().join(",")
            ),
        });
        for (key, value) in [
            ("AXIS", &self.frequency_axis),
            ("INTEGRATION", &self.integration_method),
            ("SOLVER", &self.linear_solver),
        ] {
            if let Some(value) = value {
                fields.push(format!("{key}={value}"));
            }
        }
        for (key, value) in [
            ("INTEGRATED", self.integrated_noise),
            ("RANK", self.contributor_ranking),
            ("NOISEFIGURE", self.noise_figure),
        ] {
            if let Some(value) = value {
                fields.push(format!("{key}={}", if value { "YES" } else { "NO" }));
            }
        }
        if let Some(band) = self.integration_band {
            fields.push(format!("BAND=({})", list(&band)));
        }
        if let Some(resistor) = &self.source_resistor {
            fields.push(format!("SOURCERESISTOR={}", name(resistor)));
        }
        if let Some(tuples) = &self.reference_lattices {
            fields.push(format!("REFLATTICES=({})", tuple_list(tuples)));
        }
        for (key, value) in [
            ("TREF", self.reference_temperature),
            ("LINEARTOL", self.linear_tolerance),
        ] {
            if let Some(value) = value {
                fields.push(format!("{key}={value}"));
            }
        }
        for (key, value) in [
            ("KRYLOVRESTART", self.krylov_restart),
            ("KRYLOVCYCLES", self.krylov_cycles),
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
fn tuple_list(values: &[Vec<i32>]) -> String {
    values
        .iter()
        .map(|t| format!("({})", list(t)))
        .collect::<Vec<_>>()
        .join(",")
}
fn name(value: &str) -> String {
    format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
}
