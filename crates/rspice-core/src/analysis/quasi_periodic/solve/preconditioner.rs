//! Certified circuit-sized mean-derivative solves for each retained tone tuple.
use super::evaluation::Evaluation;
use super::*;
use crate::analysis::quasi_periodic::{check_abort, finite};
use crate::solver::{ComplexMatrix, SolverError, StaticMatrix};
use std::collections::BTreeMap;

pub(super) struct FrequencyBlocks {
    blocks: Vec<Option<ComplexMatrix>>,
    unknowns: usize,
    entries: usize,
    /// Constant derivatives make these blocks the entire Jacobian.
    pub exact: bool,
}

impl FrequencyBlocks {
    pub fn build(
        work: &Workspace<'_>,
        evaluation: &Evaluation,
        abort: &dyn AbortSignal,
    ) -> Result<Self, Error> {
        check_abort(abort)?;
        let first = evaluation
            .jacobian
            .first()
            .ok_or_else(|| Error::InvalidConfig("missing QPSS derivative samples".into()))?;
        let exact = evaluation.jacobian.iter().all(|sample| {
            sample.conductance == first.conductance && sample.capacitance == first.capacitance
        });
        let mut means: BTreeMap<(usize, usize), (Value, Value)> = BTreeMap::new();
        let samples = if exact {
            &evaluation.jacobian[..1]
        } else {
            &evaluation.jacobian[..]
        };
        let divisor = samples.len() as Value;
        for sample in samples {
            check_abort(abort)?;
            for (is_charge, terms) in [(false, &sample.conductance), (true, &sample.capacitance)] {
                for &(row, col, value) in terms {
                    let pair = means.entry((row, col)).or_default();
                    if is_charge {
                        pair.1 += value / divisor;
                    } else {
                        pair.0 += value / divisor;
                    }
                }
            }
        }
        let n = work.unknowns;
        let entries = work.grid.len();
        let dc = entries / 2;
        let mut blocks = Vec::with_capacity(dc + 1);
        for k in dc..entries {
            check_abort(abort)?;
            let mut pattern: Vec<_> = (0..n).map(|i| (i, i, 0.0)).collect();
            pattern.extend(means.keys().map(|&(r, c)| (r, c, 0.0)));
            pattern.extend(work.linear[k].iter().map(|&(r, c, _)| (r, c, 0.0)));
            let structure = StaticMatrix::from_triplets(n, n, &pattern)?;
            let mut matrix = ComplexMatrix::from_real_structure(&structure);
            let jw = Complex64::new(0.0, std::f64::consts::TAU * work.grid.frequencies_hz()[k]);
            for (&(row, col), &(g, c)) in &means {
                let value = Complex64::new(g, 0.0) + jw * c;
                if !finite(value) {
                    return Err(Error::Numerical("QPSS mean derivative overflowed".into()));
                }
                matrix.add(row, col, value);
            }
            for &(row, col, value) in &work.linear[k] {
                matrix.add(row, col, value);
            }
            // Factor even an unexcited block. No pivot regularization may
            // turn a floating linear circuit into a qualified zero solution.
            match matrix.solve(&vec![Complex64::ZERO; n]) {
                Ok(_) => blocks.push(Some(matrix)),
                Err(SolverError::SingularMatrix | SolverError::InaccurateSolution(_)) if !exact => {
                    // A singular mean block need not imply a singular coupled
                    // operator. Identity is a preconditioner only; the complete
                    // inverse certificate below still decides qualification.
                    blocks.push(None);
                }
                Err(error) => return Err(error.into()),
            }
        }
        Ok(Self {
            blocks,
            unknowns: n,
            entries,
            exact,
        })
    }

    pub fn apply(
        &mut self,
        direction: &[Value],
        divisors: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, Error> {
        check_abort(abort)?;
        if direction.len() != self.unknowns * self.entries {
            return Err(Error::InvalidConfig(
                "QPSS block vector has the wrong size".into(),
            ));
        }
        let mut spectra = coordinates::decode(direction, self.entries);
        let dc = self.entries / 2;
        let mut rhs = vec![Complex64::ZERO; self.unknowns];
        for (offset, matrix) in self.blocks.iter_mut().enumerate() {
            check_abort(abort)?;
            let k = dc + offset;
            for row in 0..self.unknowns {
                rhs[row] = spectra[row][k] * divisors[row];
            }
            let solved = match matrix {
                Some(matrix) => matrix.solve(&rhs)?,
                None => rhs.clone(),
            };
            for (row, value) in solved.into_iter().enumerate() {
                if !finite(value) || (k == dc && value.im != 0.0) {
                    return Err(Error::Numerical(
                        "QPSS block solve returned invalid real coordinates".into(),
                    ));
                }
                spectra[row][k] = value;
                spectra[row][self.entries - 1 - k] = value.conj();
            }
        }
        Ok(coordinates::encode(&spectra))
    }
}
