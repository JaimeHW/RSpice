//! SP vectors: the scattering, admittance and impedance matrices ngspice's
//! `sp` analysis leaves in its plot (spicelib/analysis/cktspdum.c), one
//! `S_i_j`, `Y_i_j` and `Z_i_j` per port pair, over the frequency scale.
use super::*;
use crate::analysis::s_param::{invert_complex_matrix, y_from_s};
use crate::engine::SParameterRun;

/// The port matrix an SP vector reads.
#[derive(Clone, Copy)]
enum PortMatrix {
    Scattering,
    Admittance,
    Impedance,
}

impl PortMatrix {
    /// In the order span.c names them.
    const ALL: [Self; 3] = [Self::Scattering, Self::Admittance, Self::Impedance];

    const fn prefix(self) -> char {
        match self {
            Self::Scattering => 'S',
            Self::Admittance => 'Y',
            Self::Impedance => 'Z',
        }
    }

    fn from_prefix(prefix: char) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|matrix| matrix.prefix().eq_ignore_ascii_case(&prefix))
    }

    const fn unit(self) -> SignalUnit {
        match self {
            Self::Scattering => SignalUnit::Dimensionless,
            Self::Admittance => SignalUnit::Siemens,
            Self::Impedance => SignalUnit::Ohm,
        }
    }

    /// The vector type `display` names.
    const fn type_name(self) -> &'static str {
        match self {
            Self::Scattering => "s-param",
            Self::Admittance => "admittance",
            Self::Impedance => "impedance",
        }
    }
}

/// `S_row_col`: the wave leaving port `row` for a wave entering port `col`;
/// `Y_row_col` and `Z_row_col` are the same pair of the admittance and
/// impedance matrices.
#[derive(Clone, Copy)]
pub(super) struct SParameterColumn {
    matrix: PortMatrix,
    row: usize,
    col: usize,
}

impl SParameterColumn {
    pub(super) fn sample(self, run: &SParameterRun, point: usize) -> Option<ComplexValue> {
        let scattering = run.scattering.data.get(point)?;
        if let PortMatrix::Scattering = self.matrix {
            return Some(scattering.get(self.row, self.col));
        }
        let ports = run.scattering.num_ports;
        let s = (1..=ports)
            .map(|row| (1..=ports).map(|col| scattering.get(row, col)).collect())
            .collect::<Vec<Vec<_>>>();
        let z0 = reference_impedances(run);
        let matrix = match self.matrix {
            PortMatrix::Admittance => y_from_s(&s, &z0).ok(),
            _ => z_from_s(&s, &z0),
        };
        // ngspice inverts through the determinant (maths/dense/dense.c
        // cinverse); a matrix with no inverse leaves no finite entry.
        let unbounded = ComplexValue::new(f64::NAN, f64::NAN);
        Some(matrix.map_or(unbounded, |matrix| matrix[self.row - 1][self.col - 1]))
    }
}

/// Each port's reference impedance, in port order: the impedances the
/// scattering matrices are normalized to.
fn reference_impedances(run: &SParameterRun) -> Vec<f64> {
    let ports = run.scattering.num_ports;
    if run.ports.len() == ports {
        run.ports.iter().map(|port| port.z0).collect()
    } else {
        vec![run.scattering.z0; ports]
    }
}

/// `Z = sqrt(z0) (I - S)^-1 (I + S) sqrt(z0)`, as cktspdum.c's
/// CKTspCalcSMatrix forms it with `Gn = 1/sqrt(z0)`.
fn z_from_s(s: &[Vec<ComplexValue>], z0: &[f64]) -> Option<Vec<Vec<ComplexValue>>> {
    let size = s.len();
    let identity = |row: usize, col: usize| if row == col { 1.0 } else { 0.0 };
    let difference = (0..size)
        .map(|row| {
            (0..size)
                .map(|col| ComplexValue::new(identity(row, col), 0.0) - s[row][col])
                .collect()
        })
        .collect::<Vec<Vec<_>>>();
    let inverse = invert_complex_matrix(&difference)?;
    let scale = z0.iter().map(|z0| z0.sqrt()).collect::<Vec<_>>();
    Some(
        (0..size)
            .map(|row| {
                (0..size)
                    .map(|col| {
                        let product: ComplexValue = (0..size)
                            .map(|k| inverse[row][k] * (s[k][col] + identity(k, col)))
                            .sum();
                        product * scale[row] * scale[col]
                    })
                    .collect()
            })
            .collect(),
    )
}

/// An `S_i_j`, `Y_i_j` or `Z_i_j` vector of an SP dataset, named without
/// regard to case.
pub(super) fn select<'a>(dataset: &'a ControlNamedDataset, name: &str) -> Option<Selected<'a>> {
    let ControlAnalysisResult::SParameters(run) = &dataset.result else {
        return None;
    };
    let (matrix, row, col) = port_pair(name, run.scattering.num_ports)?;
    Some(Selected {
        dataset,
        id: ControlVectorId {
            dataset: dataset.name.clone(),
            signal: format!("{}_{row}_{col}", matrix.prefix()),
        },
        column: Column::SParameter(SParameterColumn { matrix, row, col }),
        unit: matrix.unit(),
        noise_probe: None,
    })
}

fn port_pair(name: &str, ports: usize) -> Option<(PortMatrix, usize, usize)> {
    let matrix = PortMatrix::from_prefix(name.chars().next()?)?;
    let rest = name.get(1..)?.strip_prefix('_')?;
    let (row, col) = rest.split_once('_')?;
    let (row, col): (usize, usize) = (row.parse().ok()?, col.parse().ok()?);
    let ports = 1..=ports;
    (ports.contains(&row) && ports.contains(&col)).then_some((matrix, row, col))
}

/// Every `S_i_j`, `Y_i_j` and `Z_i_j` name of a run with its vector type,
/// matrix by matrix and row by row.
pub(super) fn names(run: &SParameterRun) -> Vec<(String, &'static str)> {
    let ports = run.scattering.num_ports;
    PortMatrix::ALL
        .into_iter()
        .flat_map(|matrix| {
            (1..=ports).flat_map(move |row| {
                (1..=ports).map(move |col| {
                    (
                        format!("{}_{row}_{col}", matrix.prefix()),
                        matrix.type_name(),
                    )
                })
            })
        })
        .collect()
}
