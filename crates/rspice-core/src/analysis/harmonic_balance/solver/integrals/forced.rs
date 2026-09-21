//! Voltages fixed by exact independent-source constraints. This is only a
//! large-signal operating-point identity, never a small-signal substitution.
use super::*;
use std::collections::VecDeque;

pub(in crate::analysis::harmonic_balance::solver) struct ForcedNode {
    pub node: usize,
    pub parent: Option<usize>,
    pub branch: usize,
    pub sign: Value,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::{CountingAbort, NoAbort};
    use crate::device::behavioral::{
        BehavioralBranchResolution, BehavioralSources, BehavioralVoltageSource,
    };

    #[test]
    fn driven_integral_source_tree_keeps_polarity_and_excludes_floating_islands() {
        let mut solver = HbSolver::new(HbConfig::new(1e3).with_harmonics(1), 5);
        for (i, (pos, neg)) in [(0, 1), (2, 1), (4, 5)].into_iter().enumerate() {
            let name = format!("V{i}");
            let input = solver
                .try_add_named_voltage_source_branch_harmonics(pos, neg, 0.0, &[], &name)
                .unwrap();
            solver
                .try_add_periodic_voltage_source_branch(pos, neg, input, i + 1, &name)
                .unwrap();
        }
        let tree = solver.forced_voltage_tree(true, &NoAbort).unwrap();
        let actual: Vec<_> = tree
            .iter()
            .map(|n| (n.node, n.parent, n.branch, n.sign))
            .collect();
        assert_eq!(actual, [(0, None, 0, -1.0), (1, Some(0), 1, 1.0)]);
        assert!(matches!(
            solver.forced_voltage_tree(true, &CountingAbort::new(0)),
            Err(HbError::Aborted)
        ));
    }

    #[test]
    fn driven_integral_keeps_response_derivatives_when_reusing_the_hb_solver() {
        let rate = 1e3;
        let omega = std::f64::consts::TAU * rate;
        let mut solver = HbSolver::new(HbConfig::new(rate).with_harmonics(2), 2);
        let input = solver
            .try_add_named_voltage_source_branch_harmonics(
                1,
                0,
                0.0,
                &[(1, 1.0, -std::f64::consts::FRAC_PI_2)],
                "V1",
            )
            .unwrap();
        solver
            .try_add_periodic_voltage_source_branch(1, 0, input, 1, "V1")
            .unwrap();
        solver
            .try_add_periodic_constitutive_port_branch(2, 0, 2, "B1")
            .unwrap();
        solver
            .try_add_exact_mna_static_entry(3, 1, 1.0, "B1")
            .unwrap();
        solver.add_resistor(1, 2, 1e3);
        let mut source =
            BehavioralVoltageSource::new("B1".into(), 2, 0, 2, &format!("{omega}*sdt(v(input))"))
                .unwrap();
        source
            .bind_references(|_| Some(1), |_| BehavioralBranchResolution::MissingDevice)
            .unwrap();
        let sources = BehavioralSources {
            voltage_sources: vec![source],
            current_sources: vec![],
        };
        assert!(
            solver
                .prepare_prescribed_integrals(&sources, 1, false, &NoAbort)
                .is_err()
        );
        assert!(matches!(
            solver.prepare_prescribed_integrals(
                &sources,
                usize::MAX,
                false,
                &CountingAbort::new(3)
            ),
            Err(HbError::Aborted)
        ));
        solver
            .set_periodic_behavioral_sources(&sources, false, usize::MAX, false, &NoAbort)
            .unwrap();
        let mut state = HbSolverState::new(2, 2);
        solver
            .solve_newton_with_abort(&mut state, &NoAbort)
            .unwrap();
        assert!((state.x[1][0].re - 1.0).abs() < 1e-8);
        let original = state.x.clone();
        let response = solver
            .solve_periodic_ac_with_branch_voltages(
                &state,
                PeriodicSidebandWindow {
                    offset_hz: rate * 0.13,
                    sideband_min: 0,
                    sideband_max: 0,
                },
                &[PeriodicAcExcitation {
                    sideband: 0,
                    injections: vec![],
                }],
                &[&[(0, Complex64::ONE)]],
            )
            .unwrap();
        assert!((response[0][1][0] - 1.0 / Complex64::new(0.0, 0.13)).norm() < 1e-8);
        assert!(
            solver
                .solve_periodic_ac_with_branch_voltages(
                    &state,
                    PeriodicSidebandWindow {
                        offset_hz: 0.0,
                        sideband_min: 0,
                        sideband_max: 0
                    },
                    &[PeriodicAcExcitation {
                        sideband: 0,
                        injections: vec![]
                    }],
                    &[&[(0, Complex64::ONE)]]
                )
                .is_err(),
            "a DC perturbation of a free integrator has no steady response"
        );
        solver
            .solve_newton_with_abort(&mut state, &NoAbort)
            .unwrap();
        assert_eq!(
            state.x, original,
            "a consumer cannot change the producer's integral constraints"
        );
    }
}

impl HbSolver {
    /// A spanning tree from ground fixes absolute node potentials. Floating
    /// source islands remain unknown. Original branch equations stay in MNA,
    /// so an inconsistent or redundant ideal-source loop is still diagnosed.
    /// Callers charge 8*(nodes+branches+1) before allocating this topology.
    pub(in crate::analysis::harmonic_balance::solver) fn forced_voltage_tree(
        &self,
        authored_hb: bool,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<ForcedNode>, HbError> {
        let mut adjacent = vec![Vec::new(); self.num_nodes + 1];
        for (index, branch) in self.exact_mna_branches().iter().enumerate() {
            if abort.is_aborted() {
                return Err(HbError::Aborted);
            }
            let ExactMnaBranch::VoltageSource {
                node_pos,
                node_neg,
                source,
                ..
            } = branch
            else {
                continue;
            };
            if authored_hb && source.is_none() {
                continue;
            }
            if *node_pos > self.num_nodes || *node_neg > self.num_nodes || node_pos == node_neg {
                return Err(HbError::InvalidCircuit(
                    "invalid ideal-source terminals in integral preparation".into(),
                ));
            }
            adjacent[*node_neg].push((*node_pos, index, 1.0));
            adjacent[*node_pos].push((*node_neg, index, -1.0));
        }
        let mut seen = vec![false; self.num_nodes + 1];
        seen[0] = true;
        let mut queue = VecDeque::from([0]);
        let mut tree = Vec::new();
        while let Some(parent) = queue.pop_front() {
            if abort.is_aborted() {
                return Err(HbError::Aborted);
            }
            for &(node, branch, sign) in &adjacent[parent] {
                if std::mem::replace(&mut seen[node], true) {
                    continue;
                }
                tree.push(ForcedNode {
                    node: node - 1,
                    parent: parent.checked_sub(1),
                    branch,
                    sign,
                });
                queue.push_back(node);
            }
        }
        Ok(tree)
    }
}
