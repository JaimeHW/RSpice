//! Matrix offsets of each device's stamp sequence, replayed across loads.
//!
//! A BSIM4 instance stamps the same positions in nearly the same order on
//! every load, and through [`StaticMatrix::add`] each position is a binary
//! search of its column. ngspice binds every element pointer once at setup;
//! here the first load records the offset of each position an instance
//! reaches, and later loads add straight into the recorded offset while the
//! instance repeats its sequence. An entry skipped because its value is
//! exactly zero is stepped over; a position the record lacks is looked up
//! and inserted. A recorded offset is used only for the identical row and
//! column under the identical sparsity pattern, so every value lands in the
//! entry `add` would choose, in the same order: the matrix is bit-identical.

use rspice_matrix::{CscIndex, CscPatternToken};

use crate::device::MatrixStamper;
use crate::solver::StaticMatrix;
use crate::{NodeId, Value};

/// How far past the expected position a load may resume its record.
const RESYNC_WINDOW: usize = 4;

type Entry = (usize, usize, Option<CscIndex>);

/// Per-device stamp records for one matrix pattern. A clone starts empty:
/// the records are a cache, never state.
#[derive(Debug, Default)]
pub(crate) struct StampReplay {
    pattern: Option<CscPatternToken>,
    devices: Vec<Vec<Entry>>,
}

impl Clone for StampReplay {
    fn clone(&self) -> Self {
        Self::default()
    }
}

impl StampReplay {
    /// Bind the records to `matrix`'s pattern for `devices` instances,
    /// forgetting records made for any other pattern.
    pub(crate) fn prepare(&mut self, matrix: &StaticMatrix, devices: usize) {
        let pattern = matrix.pattern_token();
        if self.pattern != Some(pattern) {
            self.devices.clear();
            self.pattern = Some(pattern);
        }
        self.devices.resize_with(devices, Vec::new);
    }

    /// A stamper for instance `device` (after [`Self::prepare`]).
    pub(crate) fn stamper<'a>(
        &'a mut self,
        device: usize,
        matrix: &'a mut StaticMatrix,
        rhs: &'a mut [Value],
    ) -> ReplayStamper<'a> {
        ReplayStamper {
            matrix,
            rhs,
            record: &mut self.devices[device],
            cursor: 0,
        }
    }
}

/// [`MatrixStamper`] over a [`StaticMatrix`] with the semantics of the
/// plain 1-based stamper (`add` at `row - 1, col - 1`; node 0 is ground),
/// replaying one device's recorded offsets.
pub(crate) struct ReplayStamper<'a> {
    matrix: &'a mut StaticMatrix,
    rhs: &'a mut [Value],
    record: &'a mut Vec<Entry>,
    cursor: usize,
}

impl ReplayStamper<'_> {
    fn index(&mut self, row: usize, col: usize) -> Option<CscIndex> {
        let end = self.record.len().min(self.cursor + RESYNC_WINDOW);
        for position in self.cursor..end {
            let (r, c, index) = self.record[position];
            if r == row && c == col {
                self.cursor = position + 1;
                return index;
            }
        }
        let index = self.matrix.get_index(row, col);
        self.record.insert(self.cursor, (row, col, index));
        self.cursor += 1;
        index
    }
}

impl MatrixStamper for ReplayStamper<'_> {
    #[inline]
    fn stamp(&mut self, row: NodeId, col: NodeId, value: Value) {
        if row > 0 && col > 0 {
            match self.index(row - 1, col - 1) {
                Some(index) => self.matrix.stamp_direct(index, value),
                // `add` records the missing position as the stamp error.
                None => self.matrix.add(row - 1, col - 1, value),
            }
        }
    }

    #[inline]
    fn stamp_rhs(&mut self, index: NodeId, value: Value) {
        if index > 0 && index <= self.rhs.len() {
            self.rhs[index - 1] += value;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn matrix(extra: Option<(usize, usize)>) -> StaticMatrix {
        let mut triplets = vec![
            (0, 0, 0.0),
            (0, 1, 0.0),
            (1, 0, 0.0),
            (1, 1, 0.0),
            (2, 2, 0.0),
        ];
        triplets.extend(extra.map(|(row, col)| (row, col, 0.0)));
        StaticMatrix::from_triplets(3, 3, &triplets).expect("pattern")
    }

    fn bits(matrix: &StaticMatrix) -> Vec<(usize, usize, u64)> {
        matrix
            .stored_entries()
            .map(|(row, col, value)| (row, col, value.to_bits()))
            .collect()
    }

    /// Loads that skip a recorded position, add one the record lacks, and
    /// move to another pattern leave exactly the matrix `add` builds.
    #[test]
    fn replayed_loads_build_the_matrix_plain_adds_build() {
        let loads: [&[(NodeId, NodeId, Value)]; 4] = [
            &[
                (1, 1, 1.0),
                (1, 2, 2.0),
                (2, 1, 3.0),
                (2, 2, 4.0),
                (1, 1, 0.5),
            ],
            &[(1, 1, 0.5), (2, 1, 3.5), (2, 2, 4.5), (1, 1, 0.25)],
            &[
                (1, 1, 0.25),
                (3, 3, 7.0),
                (1, 2, 2.25),
                (0, 2, 9.0),
                (2, 1, 1.0),
            ],
            &[(3, 1, 0.125), (1, 1, 1.5), (2, 2, -4.0)],
        ];
        // The first three loads share one pattern, as Newton loads do; the
        // last moves to another.
        let (shared, other) = (matrix(None), matrix(Some((2, 0))));
        let mut replay = StampReplay::default();
        for (index, load) in loads.iter().enumerate() {
            let base = if index == 3 { &other } else { &shared };
            let (mut replayed, mut plain) = (base.clone_structure(), base.clone_structure());
            let (mut replayed_rhs, mut plain_rhs) = ([0.0; 3], [0.0; 3]);
            replay.prepare(&replayed, 1);
            let mut stamper = replay.stamper(0, &mut replayed, &mut replayed_rhs);
            for &(row, col, value) in *load {
                stamper.stamp(row, col, value);
                stamper.stamp_rhs(row, value);
            }
            for &(row, col, value) in *load {
                if row > 0 && col > 0 {
                    plain.add(row - 1, col - 1, value);
                }
                if row > 0 {
                    plain_rhs[row - 1] += value;
                }
            }
            assert_eq!(bits(&replayed), bits(&plain), "load {index}");
            assert_eq!(replayed_rhs, plain_rhs, "load {index}");
        }
    }
}
