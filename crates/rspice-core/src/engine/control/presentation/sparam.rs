//! SP vectors: the `S_i_j` scattering parameters ngspice's `sp` analysis
//! leaves in its plot (spicelib/analysis/span.c), one per port pair, over
//! the frequency scale.
use super::*;
use crate::engine::SParameterRun;

/// `S_row_col`: the wave leaving port `row` for a wave entering port `col`.
#[derive(Clone, Copy)]
pub(super) struct SParameterColumn {
    row: usize,
    col: usize,
}

impl SParameterColumn {
    pub(super) fn sample(self, run: &SParameterRun, point: usize) -> Option<ComplexValue> {
        Some(run.scattering.data.get(point)?.get(self.row, self.col))
    }
}

/// An `S_i_j` vector of an SP dataset, named without regard to case.
pub(super) fn select<'a>(dataset: &'a ControlNamedDataset, name: &str) -> Option<Selected<'a>> {
    let ControlAnalysisResult::SParameters(run) = &dataset.result else {
        return None;
    };
    let (row, col) = port_pair(name, run.scattering.num_ports)?;
    Some(Selected {
        dataset,
        id: ControlVectorId {
            dataset: dataset.name.clone(),
            signal: format!("S_{row}_{col}"),
        },
        column: Column::SParameter(SParameterColumn { row, col }),
        unit: SignalUnit::Dimensionless,
        noise_probe: None,
    })
}

fn port_pair(name: &str, ports: usize) -> Option<(usize, usize)> {
    let rest = name.strip_prefix(['s', 'S'])?.strip_prefix('_')?;
    let (row, col) = rest.split_once('_')?;
    let (row, col): (usize, usize) = (row.parse().ok()?, col.parse().ok()?);
    let ports = 1..=ports;
    (ports.contains(&row) && ports.contains(&col)).then_some((row, col))
}

/// Every `S_i_j` name of a run, row by row.
pub(super) fn names(run: &SParameterRun) -> Vec<String> {
    let ports = run.scattering.num_ports;
    (1..=ports)
        .flat_map(|row| (1..=ports).map(move |col| format!("S_{row}_{col}")))
        .collect()
}
