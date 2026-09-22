//! Independently determined linear coordinates for zero-origin primitives.
//!
//! A structural matching excludes every coordinate reachable from a free
//! column. Each remaining component is closed under the original linear
//! equations. Singular components stay in the full nonlinear solve; no DC
//! gauge, leakage, or small-coefficient threshold is introduced here.

use super::*;
use crate::analysis::quasi_periodic::QuasiPeriodicError as Error;
use crate::device::behavioral::BehavioralSources;
use crate::solver::{ComplexMatrix, SolverError, StaticMatrix};
use crate::{ResourceKind, ResourceLimitError};
use std::collections::{BTreeSet, VecDeque};

fn check_abort(abort: &dyn AbortSignal) -> Result<(), Error> {
    if abort.is_aborted() {
        Err(Error::Aborted)
    } else {
        Ok(())
    }
}

// Return closed components as (equation row, coordinate column) pairs.
pub(super) fn closed_components(
    rows: &[Vec<usize>],
    abort: &dyn AbortSignal,
) -> Result<Vec<Vec<(usize, usize)>>, Error> {
    let n = rows.len();
    let mut row_match = vec![None; n];
    let mut col_match = vec![None; n];
    let mut parent = vec![None; n];
    let mut queue = VecDeque::new();
    for root in 0..n {
        check_abort(abort)?;
        parent.fill(None);
        queue.clear();
        queue.push_back(root);
        let mut free = None;
        'search: while let Some(row) = queue.pop_front() {
            check_abort(abort)?;
            for &col in &rows[row] {
                if parent[col].is_some() {
                    continue;
                }
                parent[col] = Some(row);
                if let Some(next) = col_match[col] {
                    queue.push_back(next);
                } else {
                    free = Some(col);
                    break 'search;
                }
            }
        }
        while let Some(col) = free {
            let row = parent[col].expect("augmenting path");
            free = row_match[row];
            row_match[row] = Some(col);
            col_match[col] = Some(row);
        }
    }
    let mut incidence = vec![Vec::new(); n];
    for (row, cols) in rows.iter().enumerate() {
        for &col in cols {
            incidence[col].push(row);
        }
    }
    let mut unknown: Vec<_> = col_match.iter().map(Option::is_none).collect();
    queue.clear();
    queue.extend((0..n).filter(|&col| unknown[col]));
    while let Some(col) = queue.pop_front() {
        check_abort(abort)?;
        for &row in &incidence[col] {
            if let Some(next) = row_match[row]
                && !unknown[next]
            {
                unknown[next] = true;
                queue.push_back(next);
            }
        }
    }
    let mut visited = unknown.clone();
    let mut components = Vec::new();
    for root in 0..n {
        if visited[root] {
            continue;
        }
        let mut component = Vec::new();
        visited[root] = true;
        queue.push_back(root);
        while let Some(col) = queue.pop_front() {
            check_abort(abort)?;
            let row = col_match[col].expect("closed matched column");
            component.push((row, col));
            // Both directions are needed: a known voltage can feed another
            // equation without that equation feeding the voltage constraint.
            for next in rows[row]
                .iter()
                .copied()
                .chain(incidence[col].iter().filter_map(|&r| row_match[r]))
            {
                if !unknown[next] && !visited[next] {
                    visited[next] = true;
                    queue.push_back(next);
                }
            }
        }
        components.push(component);
    }
    Ok(components)
}

impl HbSolver {
    pub(in crate::analysis::harmonic_balance::solver) fn linear_driven_spectra(
        &self,
        sources: &BehavioralSources,
        frequencies: &[Value],
        rhs: impl Fn(usize, usize) -> Complex64,
        needed: impl Fn(usize) -> bool,
        max_values: usize,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Option<Vec<Complex64>>>, Error> {
        let n = self.num_nodes + self.exact_mna_branches().len();
        let budget = |values| {
            ResourceLimitError::ensure(ResourceKind::ResultValues, values, max_values)
                .map_err(Error::from)
        };
        // Matching, inverse incidence, component maps, and output descriptors.
        let mut storage = n.saturating_mul(40);
        budget(storage)?;
        check_abort(abort)?;
        let mut linear = vec![true; n];
        let mut exclude_node = |node: usize| {
            if node > 0 && node <= self.num_nodes {
                linear[node - 1] = false;
            }
        };
        for source in &sources.current_sources {
            exclude_node(source.node_pos);
            exclude_node(source.node_neg);
        }
        for device in &self.nonlinear_devices {
            for &node in &device.terminals {
                if node < self.num_nodes {
                    exclude_node(node + 1);
                }
            }
        }
        for device in &self.native_bjts {
            for node in device.mna_coupling_nodes() {
                exclude_node(node);
            }
        }
        for capacitor in &self.periodic_capacitors {
            exclude_node(capacitor.pos);
            exclude_node(capacitor.neg);
        }
        // Legacy nodal inductors do not use the exact branch registry.
        // Conservatively exclude their rows instead of substituting a DC short.
        for &(row, _, _) in &self.l_matrix {
            exclude_node(row + 1);
        }
        for source in &sources.voltage_sources {
            linear[self.num_nodes + source.branch_ordinal - 1] = false;
        }
        for row in &mut linear[self.num_nodes + self.physical_branch_count()..] {
            *row = false;
        }
        #[cfg(feature = "veriloga")]
        for device in &self.veriloga_nonlinear_devices {
            for &(row, _) in device.rhs_rows.iter().flatten() {
                linear[row] = false;
            }
        }
        let mut pattern = BTreeSet::new();
        for &frequency in frequencies {
            check_abort(abort)?;
            self.visit_periodic_linear_entries(frequency, false, |row, col, value| {
                check_abort(abort)?;
                if row >= n || col >= n || !value.re.is_finite() || !value.im.is_finite() {
                    return Err(Error::InvalidCircuit(
                        "invalid driven integral linear stamp".into(),
                    ));
                }
                if linear[row] && value != Complex64::ZERO && !pattern.contains(&(row, col)) {
                    storage = storage.saturating_add(12);
                    budget(storage)?;
                    pattern.insert((row, col));
                }
                Ok(())
            })?;
        }
        let mut rows = vec![Vec::new(); n];
        for &(row, col) in &pattern {
            rows[row].push(col);
        }
        let components: Vec<_> = closed_components(&rows, abort)?
            .into_iter()
            .filter(|component| component.iter().any(|&(_, col)| needed(col)))
            .collect();
        let known = components
            .iter()
            .flatten()
            .filter(|&&(_, col)| needed(col))
            .count();
        storage = storage.saturating_add(known.saturating_mul(frequencies.len()).saturating_mul(2));
        budget(storage)?;
        let mut spectra = vec![None; n];
        let mut row_index = vec![usize::MAX; n];
        let mut col_index = vec![usize::MAX; n];
        for component in components {
            check_abort(abort)?;
            let size = component.len();
            // Bound factor fill and the optional extended-precision fallback.
            budget(storage.saturating_add(size.saturating_mul(size).saturating_mul(64)))?;
            row_index.fill(usize::MAX);
            col_index.fill(usize::MAX);
            for (i, &(row, col)) in component.iter().enumerate() {
                row_index[row] = i;
                col_index[col] = i;
            }
            let structure: Vec<_> = pattern
                .iter()
                .filter_map(|&(row, col)| {
                    (row_index[row] != usize::MAX).then(|| {
                        debug_assert_ne!(col_index[col], usize::MAX, "closed linear equations");
                        (row_index[row], col_index[col], 0.0)
                    })
                })
                .collect();
            let structure = StaticMatrix::from_triplets(size, size, &structure)?;
            let mut matrix = ComplexMatrix::from_real_structure(&structure);
            let mut values: Vec<_> = component
                .iter()
                .map(|&(_, col)| needed(col).then(|| vec![Complex64::ZERO; frequencies.len()]))
                .collect();
            let mut determined = true;
            for (k, &frequency) in frequencies.iter().enumerate() {
                check_abort(abort)?;
                matrix.clear_values();
                self.visit_periodic_linear_entries(frequency, false, |row, col, value| {
                    if row_index[row] != usize::MAX && value != Complex64::ZERO {
                        matrix.try_add(row_index[row], col_index[col], value)?;
                    }
                    Ok(())
                })?;
                let rhs: Vec<_> = component.iter().map(|&(row, _)| rhs(row, k)).collect();
                let result = matrix.solve(&rhs).or_else(|error| match error {
                    SolverError::InaccurateSolution(_) if size <= 64 => {
                        matrix.solve_dense_extended(&rhs)
                    }
                    other => Err(other),
                });
                match result {
                    Ok(solution) => {
                        for (row, value) in values.iter_mut().zip(solution) {
                            if let Some(row) = row {
                                row[k] = value;
                            }
                        }
                    }
                    Err(SolverError::SingularMatrix | SolverError::InaccurateSolution(_)) => {
                        determined = false;
                        break;
                    }
                    Err(error) => return Err(error.into()),
                }
            }
            if determined {
                for ((_, col), row) in component.into_iter().zip(values) {
                    spectra[col] = row;
                }
            }
        }
        check_abort(abort)?;
        Ok(spectra)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::{CountingAbort, NoAbort};
    use crate::device::behavioral::{BehavioralBranchResolution, BehavioralVoltageSource};

    #[test]
    fn driven_integral_preparation_uses_the_complete_device_registry() {
        let mut solver = HbSolver::new(HbConfig::new(1e3).with_harmonics(1), 3);
        solver.add_resistor(0, 3, 1e3);
        solver.add_resistor(0, 1, 1e3);
        solver.add_resistor(2, 3, 1e3);
        solver.add_capacitance(1, 1, 1e-9);
        solver
            .try_add_periodic_constitutive_port_branch(3, 0, 1, "B1")
            .unwrap();
        solver
            .try_add_exact_mna_static_entry(3, 2, 1.0, "B1")
            .unwrap();
        let mut source =
            BehavioralVoltageSource::new("B1".into(), 3, 0, 1, "sdt(v(filtered))").unwrap();
        source
            .bind_references(|_| Some(2), |_| BehavioralBranchResolution::MissingDevice)
            .unwrap();
        solver
            .set_periodic_behavioral_sources(
                &BehavioralSources {
                    voltage_sources: vec![source],
                    current_sources: vec![],
                },
                false,
                8192,
                false,
                &NoAbort,
            )
            .unwrap();
        assert!(
            solver.prescribed_integrals[0].is_none(),
            "registration is not the complete circuit"
        );
        solver.refresh_driven_integrals(&NoAbort).unwrap();
        assert!(
            solver.prescribed_integrals[0]
                .as_ref()
                .unwrap()
                .is_circuit_driven()
        );
        solver.add_diode(1, 3, 1e-14, 1.0);
        solver.refresh_driven_integrals(&NoAbort).unwrap();
        assert!(
            solver.prescribed_integrals[0].is_none(),
            "a previously known input now has nonlinear loading"
        );
        assert_eq!(solver.behavioral_sources.voltage_sources.len(), 1);
    }

    #[test]
    fn driven_linear_coordinates_keep_singular_and_nonlinear_components_unresolved() {
        let mut solver = HbSolver::new(HbConfig::new(1e3).with_harmonics(1), 43);
        solver.add_resistor(0, 43, 1e3);
        solver.add_resistor(0, 1, 1e3);
        solver.add_capacitance(1, 1, 1.0 / (1e6 * std::f64::consts::TAU));
        // This capacitor has no determined DC potential. It cannot prevent
        // an independent RC network from supplying an integral trajectory.
        solver.add_capacitance(2, 2, 1e-9);
        for node in 3..43 {
            solver.add_resistor(node, if node == 42 { 43 } else { node + 1 }, 1e3);
        }
        let sources = BehavioralSources::default();
        let rhs = |row, k| {
            if row == 0 && k == 1 {
                Complex64::new(0.0005, 0.0)
            } else {
                Complex64::ZERO
            }
        };
        let spectra = solver
            .linear_driven_spectra(
                &sources,
                &[0.0, 1e3],
                rhs,
                |row| row == 1 || row == 2,
                8192,
                &NoAbort,
            )
            .unwrap();
        assert!((spectra[1].as_ref().unwrap()[1] - 0.5 / Complex64::new(1.0, 2.0)).norm() < 1e-12);
        assert!(spectra[2].is_none());
        assert!(matches!(
            solver.linear_driven_spectra(
                &sources,
                &[0.0, 1e3],
                rhs,
                |row| row == 1 || row == 2,
                8,
                &NoAbort
            ),
            Err(Error::ResourceLimit(_))
        ));
        assert!(matches!(
            solver.linear_driven_spectra(
                &sources,
                &[0.0, 1e3],
                rhs,
                |row| row == 1 || row == 2,
                8192,
                &CountingAbort::new(20)
            ),
            Err(Error::Aborted)
        ));
        solver.add_diode(1, 43, 1e-14, 1.0);
        let spectra = solver
            .linear_driven_spectra(
                &sources,
                &[0.0, 1e3],
                rhs,
                |row| row == 1 || row == 2,
                8192,
                &NoAbort,
            )
            .unwrap();
        assert!(
            spectra.iter().all(Option::is_none),
            "nonlinear loading must propagate upstream"
        );
    }
}
