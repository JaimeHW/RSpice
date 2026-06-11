//! Shared transient system assembly and residual reconstruction.
//!
//! The Newton loop, the restamp residual check, and the gmin-continuation
//! rescue all need the same equations: one assembly routine stamps the full
//! transient system (linear, companion, nonlinear, behavioral, XSPICE) at a
//! given iterate so the three consumers can never drift apart.

use super::*;

/// Per-step invariants of the transient system assembly. Holds borrows of
/// the integration coefficients and the per-device-family histories, so a
/// context is constructed locally at each use site (the histories are
/// mutated when a step is accepted) and dropped before the commit walks.
pub(super) struct TransientSystemContext<'a> {
    pub(super) coeff: &'a CompanionCoefficients,
    pub(super) method: IntegrationMethod,
    pub(super) trap_order: u8,
    pub(super) bjt_history: &'a BjtTransientHistory,
    pub(super) jfet_history: &'a JfetTransientHistory,
    pub(super) mosfet_history: &'a MosfetTransientHistory,
    pub(super) b3soi_history: &'a B3SoiTransientHistory,
    pub(super) suppress_gate_charge: bool,
    pub(super) tline_dc_refs: &'a [(Value, Value)],
    pub(super) coupled_tline_refs: &'a [CoupledTlineReferenceState],
}

impl Engine {
    /// Assemble the complete transient system `A(x)·x = b(x)` at `solution`
    /// for the timepoint `time` reached with step `dt`.
    ///
    /// `extra_diag_gmin` adds a continuation shunt on top of the baseline
    /// GMIN; both land only on node-voltage equations — branch-current
    /// equations (voltage source/inductor branches) must not receive the
    /// shunt or transient references are biased. `refresh_nonlinear` lets
    /// the Newton loop skip the device re-evaluation when its state already
    /// matches `solution`.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn stamp_transient_system(
        &self,
        circuit: &mut crate::circuit::Circuit,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        solution: &[Value],
        time: Value,
        dt: Value,
        ctx: &TransientSystemContext<'_>,
        vbic_snapshot_cache: &mut [Option<BjtChargeSnapshot>],
        vbic_reuse: VbicCachedSnapshotReuse,
        refresh_nonlinear: bool,
        extra_diag_gmin: Value,
    ) {
        let num_nodes = circuit.num_nodes();
        matrix.clear_values();
        rhs.fill(0.0);

        let diag_gmin = self.config.convergence_config.gmin_target.max(0.0) + extra_diag_gmin;
        if diag_gmin > 0.0 {
            for i in 0..num_nodes {
                matrix.add(i, i, diag_gmin);
            }
        }

        // Stamp linear devices (R, V, I) for transient; tline transient
        // behavior is stamped separately via companions.
        circuit.stamp_transient_linear_direct(matrix, rhs);
        circuit
            .voltage_sources
            .update_transient_rhs(rhs, time, |br_ordinal| num_nodes + br_ordinal);
        circuit.current_sources.update_transient_rhs(rhs, time);

        circuit.refresh_jiles_atherton_inductances(solution);
        if refresh_nonlinear && circuit.has_nonlinear_devices() {
            circuit.update_nonlinear(solution);
        }

        circuit
            .capacitors
            .stamp_transient_companion(matrix, rhs, dt, ctx.coeff);
        circuit
            .inductors
            .stamp_transient_companion(matrix, rhs, dt, ctx.coeff, num_nodes);
        circuit.stamp_coupled_inductor_pairs_transient(matrix, rhs, dt, ctx.coeff);
        circuit.stamp_multi_winding_transformers_transient(matrix, rhs, dt, ctx.coeff);

        Self::stamp_bjt_transient_companions(
            circuit,
            matrix,
            rhs,
            solution,
            ctx.method,
            ctx.trap_order,
            dt,
            ctx.bjt_history,
            vbic_snapshot_cache,
            vbic_reuse,
            self.voltage_abstol(),
            self.voltage_reltol(),
        );
        Self::stamp_jfet_transient_companions(
            circuit,
            matrix,
            rhs,
            solution,
            ctx.method,
            ctx.trap_order,
            dt,
            ctx.jfet_history,
            ctx.suppress_gate_charge,
        );
        Self::stamp_mosfet_transient_companions(
            circuit,
            matrix,
            rhs,
            solution,
            ctx.method,
            ctx.trap_order,
            dt,
            ctx.mosfet_history,
            ctx.suppress_gate_charge,
        );
        Self::stamp_b3soi_transient_companions(
            circuit,
            matrix,
            rhs,
            solution,
            ctx.method,
            ctx.trap_order,
            dt,
            ctx.b3soi_history,
        );
        Self::stamp_tline_companions(circuit, matrix, rhs, time, ctx.tline_dc_refs);
        Self::stamp_coupled_tline_companions(circuit, matrix, rhs, time, dt, ctx.coupled_tline_refs);

        if circuit.has_nonlinear_devices() {
            #[cfg(feature = "veriloga")]
            if circuit.has_veriloga_devices() {
                circuit.prepare_veriloga_timepoint(time, dt);
            }
            circuit.stamp_nonlinear(matrix, rhs, solution);
            circuit.stamp_behavioral(matrix, rhs, solution, time);
        }

        if circuit.has_xspice_devices() {
            circuit.evaluate_xspice_with_timestep(time, dt, solution);
            circuit.stamp_xspice(matrix, rhs);
        }
    }

    /// Restamp the full system at `solution` and test the true nonlinear
    /// residual against the engine's convergence scaling.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn transient_nonlinear_residual_converged(
        &self,
        circuit: &mut crate::circuit::Circuit,
        matrix: &mut crate::solver::StaticMatrix,
        rhs: &mut [Value],
        solution: &[Value],
        time: Value,
        dt: Value,
        ctx: &TransientSystemContext<'_>,
        vbic_snapshot_cache: &mut [Option<BjtChargeSnapshot>],
    ) -> bool {
        if solution.iter().any(|value| !value.is_finite()) {
            return false;
        }

        self.stamp_transient_system(
            circuit,
            matrix,
            rhs,
            solution,
            time,
            dt,
            ctx,
            vbic_snapshot_cache,
            VbicCachedSnapshotReuse::SeedOnly,
            true,
            0.0,
        );
        self.residual_convergence_met(matrix, solution, rhs)
    }

    #[inline]
    pub(super) fn should_prefer_dense_transient_solver(
        is_strictly_linear_transient: bool,
        size: usize,
        has_transformer_or_coupled_inductor: bool,
        has_xspice_devices: bool,
    ) -> bool {
        if is_strictly_linear_transient {
            return size <= 160 && has_transformer_or_coupled_inductor;
        }

        // Small nonlinear systems benefit from dense LU due to reduced sparse
        // symbolic/indirection overhead at this scale.
        !has_xspice_devices && size <= 64
    }
}

#[cfg(test)]
mod tests {
    //! Whole-system Newton consistency: the stamped transient matrix must be
    //! the Jacobian of the true assembled residual `F(x) = A(x)·x − b(x)`.
    //!
    //! The device-level FD tests pin each model's static Jacobian and charge
    //! derivatives in isolation; what they cannot catch is an inconsistency
    //! *between* the pieces as assembled — a companion source folded at a
    //! different bias than its conductance, a sign flipped on one row of a
    //! charge branch, or a derivative stamped onto the wrong matrix column.
    //! Any such defect makes Newton iterate against a matrix that is not the
    //! derivative of the residual it is reducing, which shows up as the
    //! non-contracting mV-scale limit cycles seen on saturation knife edges.
    //!
    //! Consistency holds exactly (up to FD truncation) whenever the junction
    //! limiters are pass-through, i.e. when the device update state is
    //! settled at the probe base and the probes are small.

    use super::*;
    use crate::Netlist;

    /// The vbic_excess_phase_oracle testbench: the full diffamp N1 card
    /// (Kull epi resistance, avalanche, parasitic transistor, TD=2e-11)
    /// in a stable common-emitter stage. Exercises the promoted VBIC
    /// internal nodes, the xf delay states, and their charge companions.
    const VBIC_XF_DECK: &str = "\
VBIC whole-system Jacobian consistency testbench

V1 VCC 0 3.3
VIN B 0 DC 0.8 SIN(0.8 0.05 1G 0 0)
RC VCC C 1k
RE E 0 100
Q1 C B E 0 N1

.MODEL N1 NPN LEVEL=4
+ IS=1e-16 IBEI=1e-18 IBEN=5e-15 IBCI=2e-17 IBCN=5e-15 ISP=1e-15 RCX=10
+ RCI=60 RBX=10 RBI=40 RE=2 RS=20 RBP=40 VEF=10 VER=4 IKF=2e-3 ITF=8e-2
+ XTF=20 IKR=2e-4 IKP=2e-4 CJE=1e-13 CJC=2e-14 CJEP=1e-13 CJCP=4e-13 VO=2
+ GAMM=2e-11 HRCF=2 QCO=1e-12 AVC1=2 AVC2=15 TF=10e-12 TR=100e-12 TD=2e-11 RTH=300

.END
";

    /// Legacy Gummel-Poon BJT through the snapshot companion path.
    const GP_BJT_DECK: &str = "\
Legacy GP whole-system Jacobian consistency testbench

V1 VCC 0 5
VIN B 0 DC 0.7
RC VCC C 2k
RE E 0 100
Q1 C B E 0 QN

.MODEL QN NPN IS=1e-15 BF=100 BR=2 CJE=2e-12 CJC=1e-12 TF=3e-10 TR=5e-9

.END
";

    fn matrix_row_label(circuit: &crate::circuit::Circuit, index: usize) -> String {
        let num_nodes = circuit.num_nodes();
        if index < num_nodes {
            let names = circuit.node_names_sorted();
            names
                .get(index)
                .cloned()
                .unwrap_or_else(|| format!("node#{}", index + 1))
        } else {
            format!("branch#{}", index - num_nodes + 1)
        }
    }

    /// Assemble the transient system at every coordinate-perturbed point and
    /// compare central FD columns of the true residual against the stamped
    /// matrix. Returns human-readable descriptions of every inconsistent
    /// entry; empty means the assembly is Newton-consistent at `base`.
    fn assembled_jacobian_fd_failures(
        deck: &str,
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        kick: Option<(&str, Value)>,
    ) -> Vec<String> {
        let netlist = Netlist::parse(deck).expect("deck parses");
        let engine = Engine::default().resolved_for_netlist(&netlist);
        let mut circuit = engine.build_circuit(&netlist).expect("circuit builds");
        let mut matrix = engine.build_matrix(&circuit).expect("matrix builds");
        circuit.link_indices(&matrix);
        circuit.voltage_sources.set_transient_context(dt, 1e-9);
        circuit.current_sources.set_transient_context(dt, 1e-9);

        let mut base = engine
            .solve_dc_operating_point(&netlist, &mut circuit, &mut matrix)
            .expect("operating point converges");
        if let Some((node, delta)) = kick {
            let id = circuit
                .get_node_by_name(node)
                .expect("kick node exists in the deck");
            base[id - 1] += delta;
        }
        // Settle the device update state (junction-limiter references) at
        // the probe base so the limiters are pass-through for small probes.
        if circuit.has_nonlinear_devices() {
            circuit.update_nonlinear(&base);
            circuit.update_nonlinear(&base);
        }

        let tline_dc_refs = Engine::initialize_tline_history(&mut circuit, &base, 0.0);
        let coupled_tline_refs =
            Engine::initialize_coupled_tline_history(&mut circuit, &base, 0.0);
        let mut bjt_history = Engine::initialize_bjt_history(&circuit, &base);
        bjt_history.accepted_dt_prev = dt;
        bjt_history.accepted_dt_prev_prev = dt;
        let mut jfet_history = Engine::initialize_jfet_history(&circuit, &base);
        jfet_history.accepted_dt_prev = dt;
        jfet_history.accepted_dt_prev_prev = dt;
        let mut mosfet_history = Engine::initialize_mosfet_history(&circuit, &base);
        mosfet_history.accepted_dt_prev = dt;
        mosfet_history.accepted_dt_prev_prev = dt;
        let mut b3soi_history = Engine::initialize_b3soi_history(&circuit, &base);
        b3soi_history.accepted_dt_prev = dt;
        b3soi_history.accepted_dt_prev_prev = dt;
        let mut vbic_snapshot_cache = vec![None; circuit.bjts.devices.len()];
        circuit.set_semiconductor_junction_gmin(
            engine.effective_device_junction_gmin(engine.config.convergence_config.gmin_target),
        );

        let coeff = CompanionCoefficients::for_method(Engine::effective_companion_method(
            method, trap_order,
        ));
        let ctx = TransientSystemContext {
            coeff: &coeff,
            method,
            trap_order,
            bjt_history: &bjt_history,
            jfet_history: &jfet_history,
            mosfet_history: &mosfet_history,
            b3soi_history: &b3soi_history,
            suppress_gate_charge: false,
            tline_dc_refs: &tline_dc_refs,
            coupled_tline_refs: &coupled_tline_refs,
        };

        let size = circuit.matrix_size();
        let time = dt;
        let mut rhs = vec![0.0; size];

        // Stamp at the base point and capture the asserted Jacobian columns
        // A(base)·e_j before any probe overwrites the matrix values.
        engine.stamp_transient_system(
            &mut circuit,
            &mut matrix,
            &mut rhs,
            &base,
            time,
            dt,
            &ctx,
            &mut vbic_snapshot_cache,
            VbicCachedSnapshotReuse::SeedOnly,
            true,
            0.0,
        );
        let zeros = vec![0.0; size];
        let mut stamped_columns = Vec::with_capacity(size);
        for j in 0..size {
            let mut unit = vec![0.0; size];
            unit[j] = 1.0;
            stamped_columns.push(
                matrix
                    .residual_vector(&unit, &zeros)
                    .expect("matrix column extraction"),
            );
        }

        let mut failures = Vec::new();
        for j in 0..size {
            let h = 1e-7 * base[j].abs().max(1.0);
            let mut probe = base.clone();
            probe[j] += h;
            engine.stamp_transient_system(
                &mut circuit,
                &mut matrix,
                &mut rhs,
                &probe,
                time,
                dt,
                &ctx,
                &mut vbic_snapshot_cache,
                VbicCachedSnapshotReuse::SeedOnly,
                true,
                0.0,
            );
            let f_plus = matrix
                .residual_vector(&probe, &rhs)
                .expect("residual at +h probe");

            probe[j] = base[j] - h;
            engine.stamp_transient_system(
                &mut circuit,
                &mut matrix,
                &mut rhs,
                &probe,
                time,
                dt,
                &ctx,
                &mut vbic_snapshot_cache,
                VbicCachedSnapshotReuse::SeedOnly,
                true,
                0.0,
            );
            let f_minus = matrix
                .residual_vector(&probe, &rhs)
                .expect("residual at -h probe");

            let column_scale = stamped_columns[j]
                .iter()
                .fold(0.0_f64, |acc, value| acc.max(value.abs()));
            for i in 0..size {
                let fd = (f_plus[i] - f_minus[i]) / (2.0 * h);
                let stamped = stamped_columns[j][i];
                let error = (fd - stamped).abs();
                let gate = 1e-6 * column_scale.max(1e-12) + 1e-4 * fd.abs().max(stamped.abs());
                if error > gate {
                    failures.push(format!(
                        "d F[{}] / d x[{}]: stamped {:.9e} vs FD {:.9e} (err {:.3e})",
                        matrix_row_label(&circuit, i),
                        matrix_row_label(&circuit, j),
                        stamped,
                        fd,
                        error,
                    ));
                }
            }
        }
        failures
    }

    fn assert_consistent(
        deck: &str,
        method: IntegrationMethod,
        trap_order: u8,
        dt: Value,
        kick: Option<(&str, Value)>,
    ) {
        let failures = assembled_jacobian_fd_failures(deck, method, trap_order, dt, kick);
        assert!(
            failures.is_empty(),
            "assembled transient matrix is not the Jacobian of the assembled residual \
             (method {:?}, trap order {}, dt {:.1e}, kick {:?}):\n{}",
            method,
            trap_order,
            dt,
            kick,
            failures.join("\n"),
        );
    }

    #[test]
    fn promoted_vbic_assembly_is_newton_consistent_at_small_dt() {
        assert_consistent(VBIC_XF_DECK, IntegrationMethod::Trapezoidal, 2, 2e-12, None);
    }

    #[test]
    fn promoted_vbic_assembly_is_newton_consistent_at_diffamp_scale_dt() {
        // dt/td = 5, the regime where the diffamp deck amplifies per
        // accepted step; trapezoidal and Gear2 both fail there, so both
        // discretizations must prove consistent.
        assert_consistent(VBIC_XF_DECK, IntegrationMethod::Trapezoidal, 2, 1e-10, None);
        assert_consistent(VBIC_XF_DECK, IntegrationMethod::Gear2, 2, 1e-10, None);
    }

    #[test]
    fn promoted_vbic_assembly_is_newton_consistent_at_backward_euler_startup() {
        // Trap order 1 resolves to the backward-Euler companion used on the
        // first accepted points and after every breakpoint restart.
        assert_consistent(VBIC_XF_DECK, IntegrationMethod::Trapezoidal, 1, 2e-12, None);
    }

    #[test]
    fn promoted_vbic_assembly_is_newton_consistent_off_equilibrium() {
        // A base-drive kick moves the BE junction off the solved manifold —
        // the consistency contract must hold at unconverged iterates too,
        // since that is where Newton actually navigates.
        assert_consistent(
            VBIC_XF_DECK,
            IntegrationMethod::Trapezoidal,
            2,
            1e-10,
            Some(("B", 0.03)),
        );
    }

    #[test]
    fn legacy_gp_snapshot_assembly_is_newton_consistent() {
        assert_consistent(GP_BJT_DECK, IntegrationMethod::Trapezoidal, 2, 1e-10, None);
        assert_consistent(GP_BJT_DECK, IntegrationMethod::Gear2, 2, 1e-10, None);
    }
}
