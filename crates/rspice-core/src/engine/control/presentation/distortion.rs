//! Explicit spectral selection on the swept F1 grid. Samples remain peak phasors.
use super::*;
use crate::analysis::{DistortionPointResult, DistortionProduct};

#[derive(Clone, Copy)]
pub(super) enum Band {
    F1,
    F2,
    Product(DistortionProduct),
}

impl Band {
    pub(super) fn parse(expression: &Expr, line: usize) -> Result<Self, ControlError> {
        let label = match expression {
            Expr::StringLiteral(label) | Expr::Param(label) => label,
            _ => {
                return Err(command_error(
                    line,
                    "DISTO requires a spectral label such as \"2f1\"",
                ));
            }
        };
        Ok(match label.to_ascii_lowercase().as_str() {
            "f1" => Self::F1,
            "f2" => Self::F2,
            "2f1" => Self::Product(DistortionProduct::SecondHarmonic),
            "3f1" => Self::Product(DistortionProduct::ThirdHarmonic),
            "f1+f2" => Self::Product(DistortionProduct::Sum),
            "f1-f2" => Self::Product(DistortionProduct::Difference),
            "2f1-f2" => Self::Product(DistortionProduct::ThirdOrderDifference),
            _ => {
                return Err(command_error(
                    line,
                    format!("unknown distortion spectrum '{label}'"),
                ));
            }
        })
    }

    fn label(self) -> &'static str {
        match self {
            Self::F1 => "f1",
            Self::F2 => "f2",
            Self::Product(product) => product.label(),
        }
    }

    fn response(self, point: &DistortionPointResult) -> Option<&AcResult> {
        match self {
            Self::F1 => Some(&point.fundamental_f1),
            Self::F2 => point.fundamental_f2.as_ref(),
            Self::Product(product) => point.product(product).map(|product| &product.response),
        }
    }
}

#[derive(Clone, Copy)]
enum Signal {
    Node(usize),
    Branch(usize),
    Ground,
    Frequency,
}

#[derive(Clone, Copy)]
pub(super) struct DistortionColumn {
    band: Band,
    signal: Signal,
}

impl DistortionColumn {
    pub(super) fn sample(
        self,
        result: &crate::analysis::DistortionAnalysisResult,
        row: usize,
    ) -> Option<ComplexValue> {
        let reference = &result.points.first()?.fundamental_f1;
        let response = self.band.response(result.points.get(row)?)?;
        if response.node_names != reference.node_names
            || response.branch_names != reference.branch_names
        {
            return None;
        }
        match self.signal {
            Signal::Node(index) => response.voltages.get(index).copied(),
            Signal::Branch(index) => response.currents.get(index).copied(),
            Signal::Ground => Some(0.0.into()),
            Signal::Frequency => Some(response.frequency.into()),
        }
    }
}

pub(super) fn select_band(
    mut selected: Selected<'_>,
    band: Band,
    line: usize,
) -> Result<Selected<'_>, ControlError> {
    let ControlAnalysisResult::Distortion(result) = &selected.dataset.result else {
        return Err(command_error(
            line,
            "DISTO spectral selection requires a distortion dataset",
        ));
    };
    if result
        .points
        .first()
        .and_then(|point| band.response(point))
        .is_none()
    {
        return Err(unavailable(line, selected.dataset, band.label()));
    }
    let (signal, unit) = match selected.column {
        Column::Node(index) => (Signal::Node(index), SignalUnit::Volt),
        Column::Branch(index) => (Signal::Branch(index), SignalUnit::Ampere),
        Column::Ground => (Signal::Ground, SignalUnit::Volt),
        Column::Scale => (Signal::Frequency, SignalUnit::Hertz),
        _ => {
            return Err(command_error(
                line,
                "DISTO requires a voltage, current or frequency vector",
            ));
        }
    };
    // Unqualified vectors already name F1. Its explicit spelling must retain
    // the same metadata identity, including SETTYPE changes through either alias.
    if !matches!(band, Band::F1) {
        selected.id.signal = format!("disto(\"{}\",{})", band.label(), selected.id.signal);
    }
    selected.column = Column::Distortion(DistortionColumn { band, signal });
    selected.unit = unit;
    Ok(selected)
}
