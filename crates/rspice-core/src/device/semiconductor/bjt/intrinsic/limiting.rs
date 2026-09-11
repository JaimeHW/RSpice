//! Intrinsic-state limiting, prediction, and continuation step-limit helpers.

use super::*;

impl Bjt {
    #[inline]
    pub(in crate::device::semiconductor::bjt) fn limit_logarithmic_step(
        vnew: Value,
        vold: Value,
        limit: Value,
    ) -> Value {
        let limit = limit.max(1e-18);
        if !vnew.is_finite() {
            return vold;
        }
        if !vold.is_finite() {
            return vnew;
        }

        if vnew > vold + limit {
            vold + limit + ((vnew - vold) / limit).log10()
        } else if vnew < vold - limit {
            vold - limit - ((vold - vnew) / limit).log10()
        } else {
            vnew
        }
    }

    #[inline]
    pub(in crate::device::semiconductor::bjt) fn junction_critical_voltage(
        vt: Value,
        isat: Value,
    ) -> Value {
        let vt = vt.max(1e-18);
        let isat = isat.abs();
        // pnjlim's forward logarithm requires a positive trial voltage. A
        // negative critical voltage can send a return to zero through log(0)
        // and prevent the limiter from ever releasing that bias. Start at
        // equilibrium once IS reaches VT/sqrt(2). Comparing first also avoids
        // overflowing sqrt(2)*IS for a large, finite saturation current.
        if isat.is_finite() && isat >= vt / core::f64::consts::SQRT_2 {
            return 0.0;
        }
        if isat == 0.0 {
            return Value::INFINITY;
        }
        let ratio = vt / (core::f64::consts::SQRT_2 * isat);
        if ratio.is_finite() && isat.is_normal() {
            vt * ratio.ln()
        } else {
            // The logarithm remains finite when the intermediate ratio does
            // not. Separate the logs without rounding sqrt(2)*subnormal IS.
            vt * (vt.ln() - core::f64::consts::SQRT_2.ln() - isat.ln())
        }
    }

    #[inline]
    pub(in crate::device::semiconductor::bjt) fn vbic_limiting_parameters(
        &self,
        previous_vrth: Value,
    ) -> (Value, Value) {
        self.with_temperature_variant(previous_vrth, |model| {
            let vt = model.vt.max(1e-18);
            let nominal_is = model.is_nominal * model.instance_scale();
            let vcrit = Self::junction_critical_voltage(vt, nominal_is);
            (vt, vcrit)
        })
    }

    #[inline]
    pub(in crate::device::semiconductor::bjt) fn vbic_nonlinear_branch_voltages(
        &self,
        internal: [Value; INTERNAL_DIM],
    ) -> VbicNonlinearBranchVoltages {
        let p = self.polarity();
        VbicNonlinearBranchVoltages {
            vbei: p * (internal[IDX_VBI] - internal[IDX_VEI]),
            vbex: p * (internal[IDX_VBX] - internal[IDX_VEI]),
            vbci: p * (internal[IDX_VBI] - internal[IDX_VCI]),
            vbcx: p * (internal[IDX_VBI] - internal[IDX_VCX]),
            vbep: p * (internal[IDX_VBX] - internal[IDX_VBP]),
            vbcp: if self.vbic_three_terminal {
                0.0
            } else {
                p * (internal[IDX_VSI] - internal[IDX_VBP])
            },
            vrth: internal[IDX_VRTH],
        }
    }

    fn legacy_critical_voltage(&self, kind: LegacyCurrent, isat: Value) -> Value {
        if let Some(scale) = self.legacy_current_scale(kind) {
            let log_is = scale.mantissa.ln()
                + Value::from(scale.binary_exponent) * core::f64::consts::LN_2
                + scale.thermal_exponent;
            (self.vt * (self.vt.ln() - core::f64::consts::SQRT_2.ln() - log_is)).max(0.0)
        } else {
            Self::junction_critical_voltage(self.vt, isat)
        }
    }

    #[inline]
    pub(in crate::device::semiconductor::bjt) fn legacy_limiting_parameters(
        &self,
        previous_vrth: Value,
    ) -> (Value, Value, Value) {
        self.with_temperature_variant(previous_vrth, |model| {
            let vt = model.vt.max(1e-18);
            let vcrit = model
                .legacy_critical_voltage(LegacyCurrent::Forward, model.is)
                .min(model.legacy_critical_voltage(
                    LegacyCurrent::Reverse,
                    model.legacy_reverse_saturation_current(),
                ));
            let sub_vcrit = model
                .legacy_junction_params
                .as_ref()
                .filter(|junctions| {
                    junctions.substrate_current > 0.0
                        || model
                            .legacy_current_scale(LegacyCurrent::Substrate)
                            .is_some()
                })
                .map_or(50.0, |junctions| {
                    model.legacy_critical_voltage(
                        LegacyCurrent::Substrate,
                        junctions.substrate_current,
                    )
                });
            (vt, vcrit, sub_vcrit)
        })
    }

    #[inline]
    pub(in crate::device::semiconductor::bjt) fn legacy_nonlinear_branch_voltages(
        &self,
        internal: [Value; INTERNAL_DIM],
    ) -> LegacyNonlinearBranchVoltages {
        let p = self.polarity();
        LegacyNonlinearBranchVoltages {
            vbe: p * (internal[IDX_VBI] - internal[IDX_VEI]),
            vbc: p * (internal[IDX_VBI] - internal[IDX_VCI]),
            vsub: match self.substrate_topology {
                BjtSubstrateTopology::Vertical => p * (internal[IDX_VSI] - internal[IDX_VCI]),
                BjtSubstrateTopology::Lateral => p * (internal[IDX_VBI] - internal[IDX_VSI]),
            },
        }
    }

    /// How tightly the network holds each junction node to an external
    /// terminal, as the conductance of the parasitic path between them.
    ///
    /// The junction limiter is defined on branch voltages, but a promoted
    /// instance carries node voltages, so a limited branch set has to be
    /// projected back onto the nodes — and the projection has to decide which
    /// nodes absorb the correction. ngspice never faces that choice: its
    /// parasitic resistor drops are separate state variables read straight out
    /// of the solution vector, so limiting a junction cannot disturb them. The
    /// closest a node-space projection gets is to move each node in inverse
    /// proportion to how hard its parasitic path resists being moved, which
    /// keeps the correction off the drops that would change most.
    ///
    /// It is not a cosmetic preference. A substrate node behind a 1 Ω `RS`
    /// takes amps for every volt the projection spends there, and on a
    /// self-heating instance the power sum turns that into watts on the thermal
    /// row — enough to walk the temperature away from an otherwise converged
    /// operating point. A collapsed state is stiffest of all: it shares a matrix
    /// column with the node it aliases, so `impose_intrinsic_node_constraints`
    /// discards whatever the projection put there.
    fn vbic_junction_node_stiffness(&self) -> [Value; VBIC_JUNCTION_NODE_DIM] {
        let series = |resistance: Value| {
            if Self::series_active(resistance) {
                resistance
            } else {
                0.0
            }
        };
        let r_cx = series(self.rcx);
        let r_ci = r_cx + series(self.rci);
        let r_bx = series(self.rbx);
        let r_bi = r_bx + series(self.rbi);
        let r_ei = series(self.re);
        let r_bp = if self.vbic_solves_vbp() {
            r_cx + series(self.rbp)
        } else {
            r_cx
        };
        let r_si = if self.vbic_three_terminal {
            0.0
        } else {
            series(self.rs)
        };
        [r_cx, r_ci, r_bx, r_bi, r_ei, r_bp, r_si].map(|r| 1.0 / r.max(1.0e-3))
    }

    pub(in crate::device::semiconductor::bjt) fn project_vbic_limited_branches_onto_internal_state(
        &self,
        raw: [Value; INTERNAL_DIM],
        limited: VbicNonlinearBranchVoltages,
    ) -> [Value; INTERNAL_DIM] {
        let p = self.polarity();
        let stiffness = self.vbic_junction_node_stiffness();
        let raw_nodes = [
            raw[IDX_VCX],
            raw[IDX_VCI],
            raw[IDX_VBX],
            raw[IDX_VBI],
            raw[IDX_VEI],
            raw[IDX_VBP],
            raw[IDX_VSI],
        ];
        let constraints = [
            [0.0, 0.0, 0.0, p, -p, 0.0, 0.0],
            [0.0, 0.0, p, 0.0, -p, 0.0, 0.0],
            [0.0, -p, 0.0, p, 0.0, 0.0, 0.0],
            [-p, 0.0, 0.0, p, 0.0, 0.0, 0.0],
            [0.0, 0.0, p, 0.0, 0.0, -p, 0.0],
            [0.0, 0.0, 0.0, 0.0, 0.0, -p, p],
        ];
        let targets = [
            limited.vbei,
            limited.vbex,
            limited.vbci,
            limited.vbcx,
            limited.vbep,
            limited.vbcp,
        ];

        // LEVEL=11 has five limited electrical junctions and no Vbcp.
        let branch_count = if self.vbic_three_terminal {
            5
        } else {
            VBIC_LIMITED_BRANCH_DIM
        };
        let mut residual = [0.0; VBIC_LIMITED_BRANCH_DIM];
        for row in 0..branch_count {
            residual[row] = -targets[row];
            for col in 0..raw_nodes.len() {
                residual[row] += constraints[row][col] * raw_nodes[col];
            }
        }

        let mut gram = [[0.0; VBIC_LIMITED_BRANCH_DIM]; VBIC_LIMITED_BRANCH_DIM];
        for row in 0..branch_count {
            for col in 0..branch_count {
                gram[row][col] = (0..raw_nodes.len())
                    .map(|idx| constraints[row][idx] * constraints[col][idx] / stiffness[idx])
                    .sum();
            }
        }

        let Some(lagrange) = crate::numerics::solve_small_dense(&gram, &residual, branch_count)
        else {
            let mut fallback = raw;
            fallback[IDX_VRTH] = limited.vrth;
            return fallback;
        };

        let mut projected = raw;
        for node_idx in 0..raw_nodes.len() {
            let correction = (0..branch_count)
                .map(|row| constraints[row][node_idx] * lagrange[row])
                .sum::<Value>()
                / stiffness[node_idx];
            projected[node_idx] = raw_nodes[node_idx] - correction;
        }
        projected[IDX_VRTH] = limited.vrth;
        projected
    }

    pub(in crate::device::semiconductor::bjt) fn project_legacy_limited_branches_onto_internal_state(
        &self,
        raw: [Value; INTERNAL_DIM],
        mut limited: LegacyNonlinearBranchVoltages,
    ) -> [Value; INTERNAL_DIM] {
        let terminals = [
            self.legacy_charge_collector_terminal(),
            self.legacy_charge_base_terminal(),
            self.legacy_charge_emitter_terminal(),
            self.legacy_charge_substrate_terminal(),
        ];
        let nodes = self.external_terminal_nodes();
        let tied = |a: usize, b: usize| {
            matches!((terminals[a].1, terminals[b].1),
                (Some(i), Some(j)) if nodes[i] == nodes[j])
        };
        // Limiting cannot introduce a voltage across a physical wire. Series
        // nodes remain independent, even if the authored terminals are tied.
        if tied(EXT_B, EXT_E) {
            limited.vbe = 0.0;
        }
        if tied(EXT_B, EXT_C) {
            limited.vbc = 0.0;
        }
        if tied(EXT_C, EXT_E) {
            limited.vbc = limited.vbe;
        }
        match self.substrate_topology {
            BjtSubstrateTopology::Vertical => {
                if tied(EXT_S, EXT_C) {
                    limited.vsub = 0.0;
                } else if tied(EXT_S, EXT_B) {
                    limited.vsub = limited.vbc;
                } else if tied(EXT_S, EXT_E) {
                    limited.vsub = limited.vbc - limited.vbe;
                }
            }
            BjtSubstrateTopology::Lateral => {
                if tied(EXT_S, EXT_B) {
                    limited.vsub = 0.0;
                } else if tied(EXT_S, EXT_C) {
                    limited.vsub = limited.vbc;
                } else if tied(EXT_S, EXT_E) {
                    limited.vsub = limited.vbe;
                }
            }
        }
        let p = self.polarity();
        let raw_nodes = [
            raw[IDX_VCX],
            raw[IDX_VCI],
            raw[IDX_VBX],
            raw[IDX_VBI],
            raw[IDX_VEI],
            raw[IDX_VBP],
            raw[IDX_VSI],
        ];
        let constraints = [
            [0.0, 0.0, 0.0, p, -p, 0.0, 0.0],
            [0.0, -p, 0.0, p, 0.0, 0.0, 0.0],
            match self.substrate_topology {
                BjtSubstrateTopology::Vertical => [0.0, -p, 0.0, 0.0, 0.0, 0.0, p],
                BjtSubstrateTopology::Lateral => [0.0, 0.0, 0.0, p, 0.0, 0.0, -p],
            },
        ];
        let targets = [limited.vbe, limited.vbc, limited.vsub];

        let mut residual = [0.0; LEGACY_LIMITED_BRANCH_DIM];
        for row in 0..LEGACY_LIMITED_BRANCH_DIM {
            residual[row] = -targets[row];
            for col in 0..raw_nodes.len() {
                residual[row] += constraints[row][col] * raw_nodes[col];
            }
        }

        let mut gram = [[0.0; LEGACY_LIMITED_BRANCH_DIM]; LEGACY_LIMITED_BRANCH_DIM];
        for row in 0..LEGACY_LIMITED_BRANCH_DIM {
            for col in 0..LEGACY_LIMITED_BRANCH_DIM {
                gram[row][col] = (0..raw_nodes.len())
                    .map(|idx| constraints[row][idx] * constraints[col][idx])
                    .sum();
            }
        }

        let Some(lagrange) =
            crate::numerics::solve_small_dense(&gram, &residual, LEGACY_LIMITED_BRANCH_DIM)
        else {
            return raw;
        };

        let mut projected = raw;
        for node_idx in 0..raw_nodes.len() {
            let correction = (0..LEGACY_LIMITED_BRANCH_DIM)
                .map(|row| constraints[row][node_idx] * lagrange[row])
                .sum::<Value>();
            projected[node_idx] = raw_nodes[node_idx] - correction;
        }
        // The projection solve may leave equal nodes one ulp apart. Copy
        // the representative so large junction slopes still see exactly zero.
        let internal = [IDX_VCI, IDX_VBI, IDX_VEI, IDX_VSI];
        for terminal in 1..EXTERNAL_DIM {
            if let Some(other) = (0..terminal).find(|&other| tied(terminal, other)) {
                projected[internal[terminal]] = projected[internal[other]];
            }
        }
        projected
    }

    /// vbicload.c's MODEINITJCT load (lines 250-269) on the promoted internal
    /// node vector: a non-OFF instance opens forward-biased at `tVcrit`, a
    /// marked one opens with every junction at zero, and neither reads the
    /// solution vector's junctions at all.
    ///
    /// That is not the same thing as handing those values to pnjlim as a
    /// history, which is what the operating-point seed already does. The seed
    /// is a starting vector Newton leaves on its first step; the load decides
    /// what the first Jacobian contains. On a latch that is the difference
    /// between an OFF instance steering which root the operating point settles
    /// on and the keyword doing nothing: the seed's zero junctions are pulled
    /// back toward the terminals by the parasitic resistances before the rest
    /// of the circuit has committed, whereas a first Jacobian carrying thirteen
    /// conducting transistors and one dead one is already asymmetric.
    ///
    /// ngspice states the load as thirteen branch quantities over seven
    /// junction nodes, so it is not a node vector and cannot be realized whole:
    /// `Vrci ≡ Vbci - Vbcx` identically, so `Vbci = -tVcrit` with `Vbcx = 0`
    /// already contradicts the `Vrci = 0` it also asks for, and an instance
    /// whose epi resistance is a milliohm would open on a kiloamp. Pinning
    /// `Vbci = Vbcx = 0` gives up the one target the model barely reads — the
    /// intrinsic B-C junction carries no forward current at either 0 or
    /// `-tVcrit`, and the transport term this load exists to create comes from
    /// `Vbei` — and buys back every resistor drop vbicload.c declares: `Vrci`,
    /// `Vrbi` and `Vrbp` all fall out at zero, so neither genuinely nonlinear
    /// parasitic (the VO/GAMM/HRCF epi resistance, the base-width-modulated
    /// RBI) carries current.
    ///
    /// The four external drops cannot follow: six constraints over seven nodes
    /// leave one degree of freedom, a common shift no internal branch sees, and
    /// the terminals are wherever the circuit put them. Their companions are
    /// exact regardless — the branches are linear — so this is free except on a
    /// self-heating instance, whose thermal row reads the power sum. That sum is
    /// quadratic in the parasitic drops, so a drop this load invents is watts of
    /// fictitious dissipation: on `RS = 1 Ω` with the substrate a volt away, it
    /// is hundreds of kelvin, and the temperature walks away from an otherwise
    /// converged operating point. A self-heating instance therefore keeps the
    /// ordinary limited load, which is the one case where a node-space port
    /// cannot reproduce vbicload.c.
    pub(in crate::device::semiconductor::bjt) fn vbic_startup_load_internal_state(
        &self,
        raw: [Value; INTERNAL_DIM],
    ) -> Option<[Value; INTERNAL_DIM]> {
        if self.charge_model != BjtChargeModel::Vbic {
            return None;
        }
        let startup = if self.initial_off {
            VbicNonlinearBranchVoltages::default()
        } else if self.thermal_model_enabled() {
            return None;
        } else {
            let (_vt, vcrit) = self.vbic_limiting_parameters(0.0);
            VbicNonlinearBranchVoltages {
                vbei: vcrit,
                vbex: vcrit,
                vbci: 0.0,
                vbcx: 0.0,
                vbep: 0.0,
                vbcp: -vcrit,
                vrth: 0.0,
            }
        };
        let projected = self.project_vbic_limited_branches_onto_internal_state(raw, startup);
        projected
            .iter()
            .all(|value| value.is_finite())
            .then_some(projected)
    }

    pub(in crate::device::semiconductor::bjt) fn limit_vbic_internal_state_to_previous(
        &self,
        raw: [Value; INTERNAL_DIM],
        previous: [Value; INTERNAL_DIM],
    ) -> [Value; INTERNAL_DIM] {
        if self.charge_model != BjtChargeModel::Vbic {
            return raw;
        }

        let raw_branches = self.vbic_nonlinear_branch_voltages(raw);
        let previous_branches = self.vbic_nonlinear_branch_voltages(previous);
        let (vt, vcrit) = self.vbic_limiting_parameters(previous[IDX_VRTH]);
        let limited_branches = VbicNonlinearBranchVoltages {
            vbei: Self::limit_junction_voltage(
                raw_branches.vbei,
                previous_branches.vbei,
                vt,
                vcrit,
            ),
            vbex: Self::limit_junction_voltage(
                raw_branches.vbex,
                previous_branches.vbex,
                vt,
                vcrit,
            ),
            vbci: Self::limit_junction_voltage(
                raw_branches.vbci,
                previous_branches.vbci,
                vt,
                vcrit,
            ),
            vbcx: Self::limit_junction_voltage(
                raw_branches.vbcx,
                previous_branches.vbcx,
                vt,
                vcrit,
            ),
            vbep: Self::limit_junction_voltage(
                raw_branches.vbep,
                previous_branches.vbep,
                vt,
                vcrit,
            ),
            vbcp: Self::limit_junction_voltage(
                raw_branches.vbcp,
                previous_branches.vbcp,
                vt,
                vcrit,
            ),
            vrth: if self.thermal_model_enabled() {
                Self::limit_logarithmic_step(raw_branches.vrth, previous_branches.vrth, 100.0)
                    .max(self.minimum_thermal_rise())
            } else {
                0.0
            },
        };

        let projected =
            self.project_vbic_limited_branches_onto_internal_state(raw, limited_branches);
        if projected.iter().all(|value| value.is_finite()) {
            projected
        } else {
            raw
        }
    }

    /// ngspice bjtload.c discipline for the promoted legacy topology: every
    /// Newton iterate limits the junction voltages via pnjlim against the
    /// previous iterate's limited values and the model evaluates and stamps
    /// at the limited bias. With the constant series resistances
    /// externalized the junction nodes are external matrix unknowns taking
    /// full Newton steps, so without this per-iterate replacement an update
    /// could overshoot the exponential junctions arbitrarily. When no
    /// previous iterate exists the load is the MODEINITJCT device state
    /// (`vbe = tVcrit`, or both junctions off for an OFF instance) so even
    /// the first evaluation of a fresh Newton solve cannot land on an
    /// unbounded exponential.
    pub(in crate::device::semiconductor::bjt) fn limit_legacy_terminal_state_against_iterate(
        &self,
        state: IntrinsicTerminalState,
        previous_iterate_available: bool,
    ) -> IntrinsicTerminalState {
        let raw = [
            state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp, state.vsi, state.vrth,
        ];
        // Operating-point initialization is an explicit device state, not
        // merely the history supplied to pnjlim. Both references assign the
        // first-load junction voltages outright and only then invoke pnjlim
        // against them. ngspice's bjtload.c has two MODEINITJCT arms: an
        // unmarked instance takes `vbe = tVcrit; vbc = vbcx = 0` at
        // bjtload.c:253-257, and an OFF instance (or any instance under
        // MODEINITFIX) takes `vbe = vbc = vbcx = 0` at bjtload.c:259-265.
        // Xyce's N_DEV_BJT.C reaches the same two states, setting `vBE =
        // tVCrit` for an unmarked instance and zeroing all three drops for an
        // OFF one. Neither reference makes the unmarked arm conditional on a
        // compatibility mode, so it runs here under every dialect: pnjlim
        // against a tVcrit reference limits a forward iterate but does not
        // place the junction there, and the intrinsic-solve seed is a starting
        // point the inner Newton converges away from, so without the explicit
        // assignment the first linearization is taken at whatever bias the
        // external solution happens to carry.
        //
        // The two references differ on the collector-base drop of an unmarked
        // instance: ngspice zeroes `vbc`, `vbcx`, `vbx`, `vsub` and `vrci`,
        // flagging the last three in its own source as a stopgap, while Xyce
        // leaves `vBC` and `vCS` at the incoming bias. They coincide where
        // MODEINITJCT actually means what it says, at a solution vector that
        // is still zero, so the assignment stays confined to `vbe`: the
        // condition here is RSpice's per-device "no previous iterate", which
        // is also reached mid-continuation where the incoming collector bias
        // is real information ngspice would be reading out of MODEINITFLOAT.
        // `vbcx` and `vrci` have no counterpart to zero at all — this port
        // carries no quasi-saturation epi branch, and the promoted topology's
        // extrinsic base node is a matrix unknown rather than a junction
        // state, so the legacy branch set is exactly `vbe`, `vbc`, `vsub`.
        if !previous_iterate_available {
            let raw_branches = self.legacy_nonlinear_branch_voltages(raw);
            let (_vt, vcrit, _sub_vcrit) = self.legacy_limiting_parameters(state.vrth);
            let initialized_branches = LegacyNonlinearBranchVoltages {
                vbe: if self.initial_off { 0.0 } else { vcrit },
                vbc: if self.initial_off {
                    0.0
                } else {
                    raw_branches.vbc
                },
                vsub: raw_branches.vsub,
            };
            let initialized =
                self.project_legacy_limited_branches_onto_internal_state(raw, initialized_branches);
            if initialized.iter().all(|value| value.is_finite()) {
                return self.intrinsic_state_from_internal_vector(initialized);
            }
            return state;
        }
        let previous = [
            self.vcx, self.vci, self.vbx, self.vbi, self.vei, self.vbp, self.vsi, self.vrth,
        ];
        if !previous.iter().all(|value| value.is_finite()) {
            return state;
        }
        let limited = self.limit_legacy_internal_state_to_previous(raw, previous);
        IntrinsicTerminalState {
            vcx: limited[IDX_VCX],
            vci: limited[IDX_VCI],
            vbx: limited[IDX_VBX],
            vbi: limited[IDX_VBI],
            vei: limited[IDX_VEI],
            vbp: limited[IDX_VBP],
            vsi: limited[IDX_VSI],
            vrth: limited[IDX_VRTH],
        }
    }

    pub(in crate::device::semiconductor::bjt) fn limit_legacy_internal_state_to_previous(
        &self,
        raw: [Value; INTERNAL_DIM],
        previous: [Value; INTERNAL_DIM],
    ) -> [Value; INTERNAL_DIM] {
        if self.charge_model != BjtChargeModel::LegacyGummelPoon {
            return raw;
        }

        let raw_branches = self.legacy_nonlinear_branch_voltages(raw);
        let previous_branches = self.legacy_nonlinear_branch_voltages(previous);
        let (vt, vcrit, sub_vcrit) = self.legacy_limiting_parameters(previous[IDX_VRTH]);
        let limit_junction = |vnew, vold, vcrit| {
            if self.xyce_compatibility {
                Self::limit_xyce_junction_voltage(vnew, vold, vt, vcrit)
            } else {
                Self::limit_junction_voltage(vnew, vold, vt, vcrit)
            }
        };
        let limited_branches = LegacyNonlinearBranchVoltages {
            vbe: limit_junction(raw_branches.vbe, previous_branches.vbe, vcrit),
            vbc: limit_junction(raw_branches.vbc, previous_branches.vbc, vcrit),
            vsub: if self.xyce_compatibility {
                raw_branches.vsub
            } else {
                limit_junction(raw_branches.vsub, previous_branches.vsub, sub_vcrit)
            },
        };

        let projected =
            self.project_legacy_limited_branches_onto_internal_state(raw, limited_branches);
        if projected.iter().all(|value| value.is_finite()) {
            projected
        } else {
            raw
        }
    }

    #[inline]
    pub(in crate::device::semiconductor::bjt) fn limit_intrinsic_state_against_previous(
        &self,
        raw: [Value; INTERNAL_DIM],
        previous: [Value; INTERNAL_DIM],
    ) -> [Value; INTERNAL_DIM] {
        let mut limited = if self.charge_model == BjtChargeModel::Vbic {
            self.limit_vbic_internal_state_to_previous(raw, previous)
        } else {
            self.limit_legacy_internal_state_to_previous(raw, previous)
        };

        if self.charge_model != BjtChargeModel::Vbic && self.thermal_model_enabled() {
            limited[IDX_VRTH] =
                Self::limit_logarithmic_step(raw[IDX_VRTH], previous[IDX_VRTH], 100.0)
                    .max(1.0 - self.requested_temperature());
        }

        limited
    }

    pub(in crate::device::semiconductor::bjt) fn predict_intrinsic_state_from_previous_external_bias_unlimited(
        &self,
        previous_external: [Value; EXTERNAL_DIM],
        previous_internal: [Value; INTERNAL_DIM],
        proposed_external: [Value; EXTERNAL_DIM],
    ) -> Option<[Value; INTERNAL_DIM]> {
        let previous_state = self.intrinsic_state_from_internal_vector(previous_internal);
        let sensitivities = self.internal_voltage_sensitivities(
            previous_state,
            previous_external[EXT_C],
            previous_external[EXT_B],
            previous_external[EXT_E],
            previous_external[EXT_S],
        );
        let delta_external = [
            proposed_external[EXT_C] - previous_external[EXT_C],
            proposed_external[EXT_B] - previous_external[EXT_B],
            proposed_external[EXT_E] - previous_external[EXT_E],
            proposed_external[EXT_S] - previous_external[EXT_S],
        ];

        let mut predicted = previous_internal;
        for internal_idx in 0..INTERNAL_DIM {
            predicted[internal_idx] += sensitivities[internal_idx]
                .iter()
                .zip(delta_external.iter())
                .map(|(sensitivity, delta)| sensitivity * delta)
                .sum::<Value>();
        }

        predicted
            .iter()
            .all(|value| value.is_finite())
            .then_some(predicted)
    }

    pub(in crate::device::semiconductor::bjt) fn predict_intrinsic_state_from_previous_external_bias(
        &self,
        previous_external: [Value; EXTERNAL_DIM],
        previous_internal: [Value; INTERNAL_DIM],
        proposed_external: [Value; EXTERNAL_DIM],
    ) -> Option<[Value; INTERNAL_DIM]> {
        let predicted = self.predict_intrinsic_state_from_previous_external_bias_unlimited(
            previous_external,
            previous_internal,
            proposed_external,
        )?;
        Some(self.limit_intrinsic_state_against_previous(predicted, previous_internal))
    }

    #[inline]
    pub(in crate::device::semiconductor::bjt) fn vbic_max_local_branch_delta(
        &self,
        lhs: [Value; INTERNAL_DIM],
        rhs: [Value; INTERNAL_DIM],
    ) -> Value {
        if self.charge_model != BjtChargeModel::Vbic {
            return lhs
                .iter()
                .zip(rhs.iter())
                .map(|(lhs, rhs)| (lhs - rhs).abs())
                .fold(0.0, Value::max);
        }

        let lhs_branches = self.vbic_nonlinear_branch_voltages(lhs);
        let rhs_branches = self.vbic_nonlinear_branch_voltages(rhs);
        [
            (lhs_branches.vbei - rhs_branches.vbei).abs(),
            (lhs_branches.vbex - rhs_branches.vbex).abs(),
            (lhs_branches.vbci - rhs_branches.vbci).abs(),
            (lhs_branches.vbcx - rhs_branches.vbcx).abs(),
            (lhs_branches.vbep - rhs_branches.vbep).abs(),
            (lhs_branches.vbcp - rhs_branches.vbcp).abs(),
            (lhs_branches.vrth - rhs_branches.vrth).abs(),
        ]
        .into_iter()
        .fold(0.0, Value::max)
    }

    /// Per-iteration junction-limiting scale for any BJT charge model.
    ///
    /// Always `None`: both charge models run ngspice's pnjlim discipline on
    /// their own junction state inside `update` (the VBIC MNA promotion and
    /// the legacy per-iterate junction replacement), so their external nodes
    /// take full Newton steps exactly like ngspice's flat MNA.
    pub(crate) fn junction_external_step_limit_scale_against_previous(
        &self,
        _previous_external: [Value; EXTERNAL_DIM],
        _proposed_external: [Value; EXTERNAL_DIM],
    ) -> Option<Value> {
        match self.charge_model {
            BjtChargeModel::Vbic | BjtChargeModel::LegacyGummelPoon => None,
        }
    }
}
