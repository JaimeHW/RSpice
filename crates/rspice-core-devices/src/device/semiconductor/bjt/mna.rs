//! Shared MNA topology for Gummel-Poon and VBIC internal states.
//!
//! ngspice solves the VBIC network by making every internal state a matrix
//! unknown (vbicsetup.c); RSpice mirrors that topology here. The circuit
//! builder allocates the internal nodes through [`Bjt::assign_mna_internal_nodes`],
//! aliasing collapsed states onto their parent nodes exactly like ngspice
//! collapses zero-resistance branches, and the device then participates in
//! the global Newton iteration through [`Bjt::update_mna`] and
//! [`Bjt::stamp_mna`] instead of nesting a private solver inside each
//! evaluation.
//!
//! Sign conventions: the intrinsic residual rows built by
//! `internal_kcl_linearization_from_eval` are oriented as "current in minus
//! current out" for the CX..SI rows and "sink minus power" for the thermal
//! row. MNA rows must accumulate currents *leaving* each node so that shared
//! rows (aliased nodes, engine gmin shunts) compose correctly, so the CX..SI
//! rows are negated when stamped while the thermal and excess-phase rows
//! already match the MNA orientation.

use super::*;

/// Operating-point noise description of a promoted VBIC instance,
/// following the selected native model's thermal conductances and shot/flicker
/// currents, each with its injection node pair on the internal topology.
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct VbicNoiseOperatingModel {
    /// Explicit compact-model constants take precedence over dialect defaults.
    pub physical_constants: Option<crate::constants::NoisePhysicalConstants>,
    /// VBIC 1.3 evaluates thermal noise at the local operating temperature.
    pub absolute_temperature: Option<Value>,
    /// `(mechanism, node+, node−, conductance)`; absent sources have zero strength.
    pub thermal: [(&'static str, NodeId, NodeId, Value); 7],
    /// `(mechanism, node+, node-, noise current)` including model multiplicity.
    pub shot: [(&'static str, NodeId, NodeId, Value); 5],
    /// Lower bound on per-copy flicker current, before applying AFN.
    pub flicker_current_floor: Value,
    /// `(mechanism, node+, node-, total current, coefficient scale)` for KFN flicker.
    pub flicker: [(&'static str, NodeId, NodeId, Value, Value); 3],
}

/// HB owns the RBI port's linear KCL incidence. The native model retains its
/// constitutive row and heat coupling; RHS terms are emitted separately below.
struct PeriodicJacobian<'a, S> {
    stamper: &'a mut S,
    rbi: NodeId,
    base: [NodeId; 2],
}

impl<S: MatrixStamper> MatrixStamper for PeriodicJacobian<'_, S> {
    fn stamp(&mut self, row: NodeId, col: NodeId, value: Value) {
        if self.rbi == 0 || col != self.rbi || !self.base.contains(&row) {
            self.stamper.stamp(row, col, value);
        }
    }
    fn stamp_rhs(&mut self, _row: NodeId, _value: Value) {}
}

/// Stamps a promoted Gummel-Poon device whose collector, base or emitter
/// share a node the way its branch form does
/// ([`Bjt::project_legacy_tied_terminal_system`]): the shared node is the
/// reference, and the device never adds to its row or column directly.
/// Every other entry is stamped as it arrives, and the reference row and
/// column are recovered from those entries: the device's electrical rows
/// carry currents that sum to zero (KCL), and its currents depend on its
/// electrical voltages only through their differences (shift invariance).
///
/// Adding the reference entries directly would sum a shorted junction's
/// equal and opposite derivatives onto the one shared diagonal, where an
/// IS = 1e20 junction's +/-4e21 S leaves nothing of the circuit's own
/// conductance. The recovered entries carry only the device's unshorted
/// branches, as the branch form's reduced system does.
#[doc(hidden)]
pub struct TiedTerminalProjection<'a, S: MatrixStamper> {
    stamper: &'a mut S,
    reference: NodeId,
    /// Every electrical node of the device, its terminals and every internal
    /// state but the thermal one: its KCL rows and the voltages its currents
    /// are invariant to a common shift of. Thermal, excess-phase and
    /// RBI-current unknowns are not among them.
    electrical: [NodeId; EXTERNAL_DIM + INTERNAL_DIM - 1],
}

impl<S: MatrixStamper> MatrixStamper for TiedTerminalProjection<'_, S> {
    fn stamp(&mut self, row: NodeId, col: NodeId, value: Value) {
        if value == 0.0 || row == self.reference || col == self.reference {
            return;
        }
        let kcl_row = self.electrical.contains(&row);
        let voltage_col = self.electrical.contains(&col);
        self.stamper.stamp(row, col, value);
        if voltage_col {
            self.stamper.stamp(row, self.reference, -value);
        }
        if kcl_row {
            self.stamper.stamp(self.reference, col, -value);
            if voltage_col {
                self.stamper.stamp(self.reference, self.reference, value);
            }
        }
    }

    fn stamp_rhs(&mut self, row: NodeId, value: Value) {
        if value == 0.0 || row == self.reference {
            return;
        }
        self.stamper.stamp_rhs(row, value);
        if self.electrical.contains(&row) {
            self.stamper.stamp_rhs(self.reference, -value);
        }
    }
}

/// Where one promoted static load writes: the Jacobian, and a right-hand side
/// for each point its sources are taken relative to.
trait MnaLoadTargets {
    /// How many right-hand sides the load writes, one or two.
    const LANES: usize;

    /// The voltages `lane`'s sources are taken relative to; `None` is zero.
    fn anchor(&self, lane: usize) -> Option<&[Value]>;

    fn stamp(&mut self, row: NodeId, col: NodeId, value: Value);

    fn stamp_rhs(&mut self, lane: usize, row: NodeId, value: Value);

    /// Add `values[lane]` to each lane's right-hand side at `row`.
    fn stamp_rhs_lanes(&mut self, row: NodeId, values: [Value; 2]) {
        for (lane, value) in values.into_iter().enumerate().take(Self::LANES) {
            self.stamp_rhs(lane, row, value);
        }
    }

    /// `source(lane)` for each lane.
    fn lanes(&self, mut source: impl FnMut(usize) -> Value) -> [Value; 2] {
        let mut values = [0.0; 2];
        for (lane, value) in values.iter_mut().enumerate().take(Self::LANES) {
            *value = source(lane);
        }
        values
    }

    /// `branch`'s source about each lane's voltages: what
    /// `self.lanes(|lane| branch.source(internal[lane], &external[lane]))`
    /// returns, with both lanes summed in one pass.
    fn sources(
        &self,
        branch: &BranchLinearization,
        internal: [&[Value; INTERNAL_DIM]; 2],
        external: &[[Value; EXTERNAL_DIM]; 2],
    ) -> [Value; 2] {
        if Self::LANES == 2 {
            branch.source_pair(internal, [&external[0], &external[1]])
        } else {
            [branch.source(internal[0], &external[0]), 0.0]
        }
    }
}

/// One stamper takes the Jacobian and the right-hand side about `anchor`.
struct SingleLoad<'s, S> {
    stamper: &'s mut S,
    anchor: Option<&'s [Value]>,
}

impl<S: MatrixStamper> MnaLoadTargets for SingleLoad<'_, S> {
    const LANES: usize = 1;

    fn anchor(&self, _lane: usize) -> Option<&[Value]> {
        self.anchor
    }

    fn stamp(&mut self, row: NodeId, col: NodeId, value: Value) {
        self.stamper.stamp(row, col, value);
    }

    fn stamp_rhs(&mut self, _lane: usize, row: NodeId, value: Value) {
        self.stamper.stamp_rhs(row, value);
    }
}

/// A Newton load and its direct correction from one evaluation: `newton`
/// takes the Jacobian and the right-hand side about zero, `correction` the
/// right-hand side about `anchor` and nothing else.
struct NewtonAndCorrection<'s, N, C> {
    newton: &'s mut N,
    correction: &'s mut C,
    anchor: &'s [Value],
}

impl<N: MatrixStamper, C: MatrixStamper> MnaLoadTargets for NewtonAndCorrection<'_, N, C> {
    const LANES: usize = 2;

    fn anchor(&self, lane: usize) -> Option<&[Value]> {
        (lane == 1).then_some(self.anchor)
    }

    fn stamp(&mut self, row: NodeId, col: NodeId, value: Value) {
        self.newton.stamp(row, col, value);
    }

    fn stamp_rhs(&mut self, lane: usize, row: NodeId, value: Value) {
        if lane == 0 {
            self.newton.stamp_rhs(row, value);
        } else {
            self.correction.stamp_rhs(row, value);
        }
    }
}

impl Bjt {
    #[doc(hidden)]
    pub fn uses_three_terminal_vbic(&self) -> bool {
        self.vbic_three_terminal
    }

    #[doc(hidden)]
    pub fn noise_enabled(&self) -> bool {
        !self.uses_vbic_dynamic_charges() || self.vbic_noise_enabled
    }

    /// Matrix-node incidence of each structurally present electrical BJT charge.
    /// Constant offsets in charge do not create a shooting coordinate, and
    /// collapsed/disabled states retain no independent voltage difference.
    pub(crate) fn electrical_charge_storage_nodes(
        &self,
    ) -> [Option<(NodeId, NodeId)>; BJT_DYNAMIC_CHARGE_COUNT - 3] {
        if self.uses_legacy_gummel_poon() {
            let nodes = self.external_terminal_nodes();
            let node = |terminal: (Option<usize>, Option<usize>)| {
                terminal.1.map(|index| nodes[index]).or_else(|| {
                    terminal
                        .0
                        .filter(|_| self.mna_promoted())
                        .map(|index| self.mna_internal_node(index))
                })
            };
            let pair = |a, b| node(a).zip(node(b)).filter(|(a, b)| a != b);
            let base = self.legacy_charge_base_terminal();
            let collector = self.legacy_charge_collector_terminal();
            let gmin_charge = self.xyce_compatibility && self.nonlinear_branch_gmin() != 0.0;
            let forward_storage = self.is != 0.0
                || self.legacy_current_scale(LegacyCurrent::Forward).is_some()
                || gmin_charge;
            let reverse_storage = self.legacy_reverse_saturation_current() != 0.0
                || self.legacy_current_scale(LegacyCurrent::Reverse).is_some()
                || gmin_charge;
            let epi_storage = self.legacy_quasi_saturation().is_some()
                && self.qco != 0.0
                && self.gamm != 0.0
                && Self::series_active(self.rci);
            let mut storage = [None; BJT_DYNAMIC_CHARGE_COUNT - 3];
            if self.cje != 0.0 || self.cbeo != 0.0 || (self.tf != 0.0 && forward_storage) {
                storage[0] = pair(base, self.legacy_charge_emitter_terminal());
            }
            if epi_storage {
                let epi = if Self::series_active(self.rcx) {
                    (Some(IDX_VCX), None)
                } else {
                    (None, Some(EXT_C))
                };
                storage[1] = pair(base, epi);
            }
            if self.cjc * self.xcjc != 0.0
                || self.cbco != 0.0
                || (self.tr != 0.0 && reverse_storage)
                || epi_storage
            {
                storage[2] = pair(base, collector);
            }
            if self.cjc != 0.0 && self.xcjc != 1.0 {
                storage[3] = self
                    .legacy_external_bc_charge_nodes()
                    .map(|[a, b]| (a, b))
                    .or_else(|| pair((None, Some(EXT_B)), collector))
                    .filter(|(a, b)| a != b);
            }
            if self.cjcp != 0.0 {
                storage[7] = pair(
                    self.legacy_charge_substrate_connection_terminal(),
                    self.legacy_charge_substrate_terminal(),
                );
            }
            return storage;
        }
        if !self.mna_promoted() {
            return [None; BJT_DYNAMIC_CHARGE_COUNT - 3];
        }
        let active = [
            (self.cje_nominal > 0.0 && self.wbe != 0.0)
                || (self.tf != 0.0 && self.is_nominal > 0.0),
            self.cje_nominal > 0.0 && self.wbe != 1.0,
            self.cjc_nominal > 0.0
                || (self.tr != 0.0 && self.is_nominal > 0.0 && self.isrr_nominal > 0.0)
                || (self.qco_nominal > 0.0 && self.gamm_nominal > 0.0),
            self.qco_nominal > 0.0 && self.gamm_nominal > 0.0,
            self.cjep_nominal > 0.0 || (self.tr != 0.0 && self.isp_nominal > 0.0),
            self.cbeo_nominal > 0.0,
            self.cbco_nominal > 0.0,
            !self.vbic_three_terminal && (self.cjcp_nominal > 0.0 || self.ccso_nominal > 0.0),
        ];
        let nodes = [
            (self.node_bi, self.node_ei),
            (self.node_bx, self.node_ei),
            (self.node_bi, self.node_ci),
            (self.node_bi, self.node_cx),
            (self.node_bx, self.node_bp),
            (self.node_base, self.node_emitter),
            (self.node_base, self.node_collector),
            (self.node_si, self.node_bp),
        ];
        std::array::from_fn(|index| {
            (active[index] && nodes[index].0 != nodes[index].1).then_some(nodes[index])
        })
    }

    /// Incidence of all physical storage coordinates. The last three are
    /// temperature rise (K) and two excess-phase transport currents (A),
    /// even though MNA represents them with node unknowns.
    #[doc(hidden)]
    pub fn charge_storage_nodes(&self) -> [Option<(NodeId, NodeId)>; BJT_DYNAMIC_CHARGE_COUNT] {
        let electrical = self.electrical_charge_storage_nodes();
        let mut storage = std::array::from_fn(|index| electrical.get(index).copied().flatten());
        if self.uses_vbic_dynamic_charges() && self.mna_promoted() {
            if self.node_rth != 0 && self.thermal_capacitance() > 0.0 {
                storage[IDX_QCTH] = Some((self.node_rth, 0));
            }
            if self.td > 0.0 {
                storage[IDX_QXF1] = (self.node_xf1 != 0).then_some((self.node_xf1, 0));
                storage[IDX_QXF2] = (self.node_xf2 != 0).then_some((self.node_xf2, 0));
            }
        }
        storage
    }

    fn mna_checkpoint_runtime_tag(&self) -> &'static str {
        if self.uses_legacy_gummel_poon() {
            GP_MNA_ACCEPTED_NONLINEAR_RUNTIME_TAG
        } else {
            VBIC_ACCEPTED_NONLINEAR_RUNTIME_TAG
        }
    }

    pub(super) fn capture_mna_checkpoint(&self) -> Result<AcceptedBjtNonlinearCheckpoint, String> {
        let mut values = Vec::with_capacity(VBIC_ACCEPTED_NONLINEAR_STATE_VALUE_COUNT);
        self.checkpoint_push_evaluation_state(&mut values);
        values.push(self.junction_gmin);
        values.extend([
            u8::from(self.mna_eval.is_some()) as Value,
            u8::from(self.mna_limited_from.get().is_some()) as Value,
            u8::from(self.vbic_startup_load_pending) as Value,
            u8::from(self.mna_charge_cache_valid.get()) as Value,
        ]);
        values.extend(
            self.mna_limited_from
                .get()
                .unwrap_or([0.0; EXTERNAL_DIM + BJT_INTERNAL_STATE_DIM]),
        );
        Self::checkpoint_push_linearization(
            &mut values,
            self.mna_eval
                .map_or_else(BjtLinearization::default, |eval| eval.linearized),
        );
        let branches = self
            .mna_eval
            .map_or([BranchLinearization::default(); 15], |eval| {
                [
                    eval.ibe, eval.ibex, eval.ibc, eval.iciei, eval.ircx, eval.irci, eval.irbx,
                    eval.irbi, eval.ire, eval.ibep, eval.irbp, eval.ibcp, eval.iccp, eval.irs,
                    eval.igcx,
                ]
            });
        for branch in branches {
            values.push(branch.current);
            values.extend(branch.d_internal);
            values.extend(branch.d_external);
        }
        for branch in self
            .mna_delay_branches
            .into_iter()
            .chain([self.mna_delay_thermal])
        {
            Self::checkpoint_push_charge_branch(
                &mut values,
                BjtChargeBranch {
                    charge: branch.current,
                    d_internal: branch.d_internal,
                    d_external: branch.d_external,
                    pos_internal: branch.pos_internal,
                    neg_internal: branch.neg_internal,
                    pos_external: branch.pos_external,
                    neg_external: branch.neg_external,
                },
            );
        }
        for branch in self.mna_charge_cache.get() {
            Self::checkpoint_push_charge_branch(&mut values, branch);
        }
        values.push(self.mna_rbi_current);
        debug_assert_eq!(values.len(), VBIC_ACCEPTED_NONLINEAR_STATE_VALUE_COUNT);
        let checkpoint = AcceptedBjtNonlinearCheckpoint {
            instance_name: self.name.clone(),
            runtime_tag: self.mna_checkpoint_runtime_tag().to_string(),
            legacy_junction_limited: self.uses_legacy_gummel_poon() && self.legacy_junction_limited,
            reduced_linearization_valid: false,
            previous_reduced_linearization_valid: self.uses_legacy_gummel_poon()
                && self.previous_reduced_linearization_valid,
            charge_snapshot_valid: false,
            state_values: values,
        };
        self.validate_mna_checkpoint(&checkpoint)?;
        Ok(checkpoint)
    }

    pub(super) fn validate_mna_checkpoint(
        &self,
        checkpoint: &AcceptedBjtNonlinearCheckpoint,
    ) -> Result<(), String> {
        if checkpoint.instance_name != self.name
            || checkpoint.runtime_tag != self.mna_checkpoint_runtime_tag()
        {
            return Err(format!(
                "BJT '{}' promoted BJT checkpoint instance/runtime mismatch",
                self.name
            ));
        }
        if checkpoint.state_values.len() != VBIC_ACCEPTED_NONLINEAR_STATE_VALUE_COUNT {
            return Err(format!(
                "BJT '{}' promoted BJT checkpoint shape mismatch: captured {}, expected {}",
                self.name,
                checkpoint.state_values.len(),
                VBIC_ACCEPTED_NONLINEAR_STATE_VALUE_COUNT
            ));
        }
        if (!self.uses_legacy_gummel_poon()
            && (checkpoint.legacy_junction_limited
                || checkpoint.previous_reduced_linearization_valid))
            || checkpoint.reduced_linearization_valid
            || checkpoint.charge_snapshot_valid
        {
            return Err(format!(
                "BJT '{}' promoted BJT checkpoint contains reduced-runtime cache flags",
                self.name
            ));
        }
        let values = &checkpoint.state_values;
        if values.iter().any(|value| !value.is_finite())
            || values[BJT_ACCEPTED_SCALAR_VALUE_COUNT] < 0.0
        {
            return Err(format!(
                "BJT '{}' promoted BJT checkpoint contains non-finite state or invalid junction GMIN",
                self.name
            ));
        }
        let flags_start = BJT_ACCEPTED_SCALAR_VALUE_COUNT + 1;
        let flags = &values[flags_start..flags_start + 4];
        if flags
            .iter()
            .any(|value| ![0.0_f64.to_bits(), 1.0_f64.to_bits()].contains(&value.to_bits()))
            || (flags[1] != 0.0 && flags[0] == 0.0)
        {
            return Err(format!(
                "BJT '{}' promoted BJT checkpoint contains invalid cache status",
                self.name
            ));
        }
        let branch_start = flags_start
            + 4
            + EXTERNAL_DIM
            + BJT_INTERNAL_STATE_DIM
            + 12
            + 15 * (1 + INTERNAL_DIM + EXTERNAL_DIM);
        // Derive endpoint incidence from the same branch builders as the
        // runtime, without solving or replacing the captured numerical cache.
        let template = BjtDynamicReduction::default();
        let expected_delay = self.vbic_delay_static_branches(&template);
        let expected_thermal = self.vbic_delay_static_thermal_branch(&template);
        let expected_charge = if self.uses_legacy_gummel_poon() {
            self.legacy_dynamic_charge_branches(
                template.external_voltages,
                template.internal_voltages,
            )
        } else {
            self.dynamic_charge_branches_from_inputs(&template, BjtDynamicChargeInputs::default())
        };
        for (branch, data) in values[branch_start..values.len() - 1]
            .chunks_exact(BJT_CHARGE_BRANCH_CHECKPOINT_VALUE_COUNT)
            .enumerate()
        {
            let endpoints = &data[1 + BJT_INTERNAL_STATE_DIM + EXTERNAL_DIM..];
            for (lane, (&value, dimension)) in endpoints
                .iter()
                .zip([
                    BJT_INTERNAL_STATE_DIM,
                    BJT_INTERNAL_STATE_DIM,
                    EXTERNAL_DIM,
                    EXTERNAL_DIM,
                ])
                .enumerate()
            {
                if value.to_bits() != (-1.0_f64).to_bits()
                    && !(value >= 0.0 && value.fract() == 0.0 && value < dimension as Value)
                {
                    return Err(format!(
                        "BJT '{}' promoted BJT branch {branch} endpoint {lane} is outside its runtime topology",
                        self.name
                    ));
                }
            }
            if (endpoints[0] >= 0.0 && endpoints[2] >= 0.0)
                || (endpoints[1] >= 0.0 && endpoints[3] >= 0.0)
            {
                return Err(format!(
                    "BJT '{}' promoted BJT branch {branch} selects both internal and external endpoints",
                    self.name
                ));
            }
            let expected = if branch <= VBIC_DELAY_BRANCH_COUNT {
                let expected = if flags[0] == 0.0 {
                    BjtCurrentBranch::default()
                } else if branch < VBIC_DELAY_BRANCH_COUNT {
                    expected_delay[branch]
                } else {
                    expected_thermal
                };
                Some([
                    expected.pos_internal,
                    expected.neg_internal,
                    expected.pos_external,
                    expected.neg_external,
                ])
            } else if flags[3] != 0.0 {
                let expected = expected_charge[branch - VBIC_DELAY_BRANCH_COUNT - 1];
                Some([
                    expected.pos_internal,
                    expected.neg_internal,
                    expected.pos_external,
                    expected.neg_external,
                ])
            } else {
                None // Invalid charge caches are overwritten before use.
            };
            if expected
                .is_some_and(|expected| endpoints != expected.map(Self::checkpoint_endpoint_value))
            {
                return Err(format!(
                    "BJT '{}' promoted BJT branch {branch} endpoint topology mismatch",
                    self.name
                ));
            }
        }
        Ok(())
    }

    /// Restore the complete cached MNA evaluation without recomputing it.
    /// Derivative caches can precede the current junction GMIN or limiter
    /// policy; reconstructing them at a new bias would change the first load.
    pub(super) fn restore_mna_checkpoint(&mut self, checkpoint: &AcceptedBjtNonlinearCheckpoint) {
        let values = &checkpoint.state_values;
        let mut cursor = 0;
        self.checkpoint_restore_evaluation_state(values, &mut cursor);
        self.junction_gmin = values[cursor];
        self.clear_thermal_variant_cache();
        self.mna_eval_at_state = false;
        cursor += 1;
        let flags = Self::checkpoint_take_array::<4>(values, &mut cursor);
        let limited_from = Self::checkpoint_take_array(values, &mut cursor);
        self.mna_limited_from
            .set((flags[1] != 0.0).then_some(limited_from));
        // The image predates the dissipation anchor, so a restored limited
        // load re-derives its drops from the candidate it was limited from.
        // Only the MODEINITJCT load anchors elsewhere, and an accepted point
        // is never that first evaluation.
        self.mna_dissipation_drops =
            (flags[1] != 0.0 && !self.uses_legacy_gummel_poon()).then(|| {
                let [vc, vb, ve, vs] = [
                    limited_from[0],
                    limited_from[1],
                    limited_from[2],
                    limited_from[3],
                ];
                let mut solution_bias = [0.0; INTERNAL_DIM];
                solution_bias
                    .copy_from_slice(&limited_from[EXTERNAL_DIM..EXTERNAL_DIM + INTERNAL_DIM]);
                self.impose_intrinsic_node_constraints(&mut solution_bias, vc, vb, ve, vs);
                Self::vbic_parasitic_drops([vc, vb, ve, vs], solution_bias)
            });
        self.vbic_startup_load_pending = flags[2] != 0.0;
        let linearized = Self::checkpoint_take_linearization(values, &mut cursor);
        let [
            ibe,
            ibex,
            ibc,
            iciei,
            ircx,
            irci,
            irbx,
            irbi,
            ire,
            ibep,
            irbp,
            ibcp,
            iccp,
            irs,
            igcx,
        ] = std::array::from_fn(|_| {
            let current = values[cursor];
            cursor += 1;
            BranchLinearization {
                current,
                d_internal: Self::checkpoint_take_array(values, &mut cursor),
                d_external: Self::checkpoint_take_array(values, &mut cursor),
            }
        });
        self.mna_terminal_rows = MnaTerminalRows(None);
        self.mna_eval = (flags[0] != 0.0).then_some(EvaluatedBjtState {
            linearized,
            ibe,
            ibex,
            ibc,
            iciei,
            ircx,
            irci,
            irbx,
            irbi,
            ire,
            ibep,
            irbp,
            ibcp,
            iccp,
            irs,
            igcx,
        });
        let [delay0, delay1, delay2, avalanche, thermal] = std::array::from_fn(|_| {
            let branch = Self::checkpoint_take_charge_branch(values, &mut cursor);
            BjtCurrentBranch {
                current: branch.charge,
                d_internal: branch.d_internal,
                d_external: branch.d_external,
                pos_internal: branch.pos_internal,
                neg_internal: branch.neg_internal,
                pos_external: branch.pos_external,
                neg_external: branch.neg_external,
            }
        });
        self.mna_delay_branches = [delay0, delay1, delay2, avalanche];
        self.mna_delay_thermal = thermal;
        self.mna_charge_cache.set(std::array::from_fn(|_| {
            Self::checkpoint_take_charge_branch(values, &mut cursor)
        }));
        self.mna_charge_cache_valid.set(flags[3] != 0.0);
        self.mna_rbi_current = values[cursor];
        cursor += 1;
        self.reduced_linearization_cache_valid.set(false);
        self.previous_reduced_linearization_valid = checkpoint.previous_reduced_linearization_valid;
        self.legacy_junction_limited = checkpoint.legacy_junction_limited;
        self.charge_snapshot_cache_valid.set(false);
        debug_assert_eq!(cursor, values.len());
    }

    /// True once the builder has promoted this instance's internal states to MNA
    /// unknowns.
    #[inline]
    #[doc(hidden)]
    pub fn mna_promoted(&self) -> bool {
        self.mna_promoted
    }

    #[doc(hidden)]
    pub fn needs_mna_rbi_branch(&self) -> bool {
        self.mna_promoted() && Self::series_active(self.rbi)
    }

    #[doc(hidden)]
    pub fn assign_mna_rbi_branch(&mut self, branch: NodeId) {
        self.mna_rbi_branch = Some(branch);
    }

    #[doc(hidden)]
    pub fn mna_rbi_branch_matrix_node(&self, num_nodes: usize) -> Option<NodeId> {
        self.mna_rbi_branch.map(|branch| num_nodes + branch)
    }

    #[doc(hidden)]
    pub fn resolve_mna_rbi_branch(&mut self, num_nodes: usize) {
        self.mna_rbi_matrix_node = self.mna_rbi_branch_matrix_node(num_nodes).unwrap_or(0);
    }

    /// RBI/qb and its partials for the explicit-current constitutive row.
    /// Taking ratios before multiplying keeps very large qb derivatives
    /// from underflowing through an intermediate 1/qb^2.
    fn mna_rbi_resistance(&self, linearized: BjtLinearization, vrth: Value) -> BranchLinearization {
        if self.uses_legacy_gummel_poon() {
            return BranchLinearization {
                current: self.legacy_gp_base_resistance(
                    linearized,
                    self.guarded_series_resistance(self.rbi),
                ),
                ..Default::default()
            };
        }
        let qb = linearized.qb.max(1e-12);
        let bare = |model: &Bjt| model.guarded_series_resistance(model.rbi);
        let mut resistance = BranchLinearization {
            current: self.with_temperature_variant(vrth, bare) / qb,
            ..BranchLinearization::default()
        };
        if linearized.qb > 1e-12 {
            let be = linearized.dqb_dvbe / qb;
            let bc = linearized.dqb_dvbc / qb;
            resistance.d_internal[IDX_VBI] = -resistance.current * (be + bc);
            resistance.d_internal[IDX_VCI] = resistance.current * bc;
            resistance.d_internal[IDX_VEI] = resistance.current * be;
        }
        if self.thermal_model_enabled() {
            let h = self.thermal_derivative_step(vrth);
            let plus = self.with_temperature_derivative_variant(vrth + h, vrth, bare);
            let minus = self.with_temperature_derivative_variant(vrth - h, vrth, bare);
            let dq_ratio = if linearized.qb > 1e-12 {
                linearized.dqb_dvrth / qb
            } else {
                0.0
            };
            resistance.d_internal[IDX_VRTH] =
                (-resistance.current).mul_add(dq_ratio, ((plus - minus) / (2.0 * h)) / qb);
        }
        resistance
    }

    /// Noise sources at the accepted bias. VBIC 1.3 follows vbic_1p3.va;
    /// older VBIC retains ngspice's emitter injection for RBP and its
    /// omission of extrinsic B-E flicker. RS and Iccp count wherever the
    /// substrate network exists, as LTspice and Xyce count them and ngspice
    /// integrates them.
    #[doc(hidden)]
    pub fn vbic_noise_operating_model(&self) -> Option<VbicNoiseOperatingModel> {
        if !self.uses_vbic_dynamic_charges() || !self.mna_promoted() {
            return None;
        }
        let eval = self.mna_eval?;
        let (g_rci, transport_current) = if self.vbic_13 {
            self.with_temperature_variant(self.vrth, |model| {
                let p = self.polarity();
                let transport = model
                    .transport_charge_state(p * (self.vbi - self.vei), p * (self.vbi - self.vci));
                let conductance = if Self::series_active(model.rci) {
                    // Irci already includes M. The VA noise equation multiplies
                    // it by M again, but Gci's small-voltage floor only once.
                    (self.m * eval.irci.current.abs() + 1e-10 / model.rci)
                        / ((self.vcx - self.vci).abs() + 1e-10)
                } else {
                    0.0
                };
                (conductance, self.m * transport.itzf)
            })
        } else {
            (
                self.irci_branch_with_self_conductance(self.vcx, self.vci, self.vbi)
                    .1,
                eval.iciei.current,
            )
        };
        let shot_scale = if self.vbic_13 { self.m } else { 1.0 };
        let substrate_noise = !self.vbic_three_terminal;
        Some(VbicNoiseOperatingModel {
            physical_constants: self
                .vbic_13
                .then_some(crate::constants::NoisePhysicalConstants::VBIC_1_3),
            absolute_temperature: self.vbic_13.then(|| {
                self.mapped_temperature(self.requested_temperature() + self.vrth)
                    .0
            }),
            thermal: [
                (
                    "RCX",
                    self.node_cx,
                    self.node_collector,
                    eval.ircx.d_external[EXT_C],
                ),
                ("RCI", self.node_cx, self.node_ci, g_rci),
                (
                    "RBX",
                    self.node_bx,
                    self.node_base,
                    eval.irbx.d_external[EXT_B],
                ),
                (
                    "RBI",
                    self.node_bx,
                    self.node_bi,
                    if self.mna_rbi_branch.is_some() {
                        self.mna_rbi_resistance(eval.linearized, self.vrth)
                            .current
                            .recip()
                    } else {
                        eval.irbi.d_internal[IDX_VBX]
                    },
                ),
                (
                    "RE",
                    self.node_ei,
                    self.node_emitter,
                    eval.ire.d_external[EXT_E],
                ),
                (
                    "RBP",
                    if self.vbic_13 {
                        self.node_bp
                    } else {
                        self.node_ei
                    },
                    if self.vbic_13 {
                        self.node_cx
                    } else {
                        self.node_emitter
                    },
                    -eval.irbp.d_internal[IDX_VCX],
                ),
                (
                    "RS",
                    self.node_si,
                    self.node_substrate,
                    if substrate_noise {
                        eval.irs.d_external[EXT_S]
                    } else {
                        0.0
                    },
                ),
            ],
            shot: [
                ("IC", self.node_ci, self.node_ei, transport_current),
                (
                    "IBE",
                    self.node_bi,
                    self.node_ei,
                    shot_scale * eval.ibe.current,
                ),
                (
                    "IBEX",
                    self.node_bx,
                    self.node_ei,
                    shot_scale * eval.ibex.current,
                ),
                (
                    "IBEP",
                    self.node_bx,
                    self.node_bp,
                    shot_scale * eval.ibep.current,
                ),
                (
                    "ICCP",
                    self.node_bx,
                    self.node_si,
                    if substrate_noise {
                        shot_scale * eval.iccp.current
                    } else {
                        0.0
                    },
                ),
            ],
            // vbicnoise.c uses N_MINLOG; VBIC 1.3 has no current floor.
            flicker_current_floor: if self.vbic_13 { 0.0 } else { 1e-38 },
            flicker: [
                ("FN", self.node_bi, self.node_ei, eval.ibe.current, 1.0),
                (
                    "FN_BEX",
                    self.node_bx,
                    self.node_ei,
                    eval.ibex.current,
                    if self.vbic_13 { 1.0 } else { 0.0 },
                ),
                (
                    "FN_BEP",
                    self.node_bx,
                    self.node_bp,
                    eval.ibep.current,
                    shot_scale,
                ),
            ],
        })
    }

    #[inline]
    pub(in crate::device::semiconductor::bjt) fn has_substrate_resistance(&self) -> bool {
        !self.vbic_three_terminal && Self::series_active(self.rs)
    }

    /// True when the parasitic (bp) state carries its own KCL row; mirrors the
    /// `solve_vbp` condition of the intrinsic residual so the promoted
    /// topology and the reduced solve collapse the same states.
    #[inline]
    pub(in crate::device::semiconductor::bjt) fn vbic_solves_vbp(&self) -> bool {
        Self::series_active(self.rbp)
    }

    /// Allocate the VBIC internal nodes using the effective series resistances.
    /// `alloc` receives a short state suffix and must
    /// return a fresh circuit node. Collapsed states alias their parent node
    /// so each retains exactly one matrix column; disabled states (thermal
    /// without self-heating, excess phase without TD) stay at ground and all
    /// of their stamps drop.
    pub fn assign_vbic_internal_nodes(&mut self, alloc: impl FnMut(&str) -> NodeId) {
        debug_assert!(self.uses_vbic_dynamic_charges());
        self.assign_mna_internal_nodes(alloc);
    }

    #[doc(hidden)]
    pub fn assign_mna_internal_nodes(&mut self, mut alloc: impl FnMut(&str) -> NodeId) {
        self.node_cx = if Self::series_active(self.rcx) {
            alloc("cx")
        } else {
            self.node_collector
        };
        self.node_ci = if Self::series_active(self.rci) {
            alloc("ci")
        } else {
            self.node_cx
        };
        self.node_bx = if Self::series_active(self.rbx) {
            alloc("bx")
        } else {
            self.node_base
        };
        self.node_bi = if Self::series_active(self.rbi) {
            alloc("bi")
        } else {
            self.node_bx
        };
        self.node_ei = if Self::series_active(self.re) {
            alloc("ei")
        } else {
            self.node_emitter
        };
        self.node_bp = if self.vbic_solves_vbp() {
            alloc("bp")
        } else {
            self.node_cx
        };
        self.node_si = if self.vbic_three_terminal {
            0
        } else if self.has_substrate_resistance() {
            alloc("si")
        } else {
            self.node_substrate
        };
        self.node_rth = if self.thermal_model_enabled() {
            if self.vbic_external_thermal_node {
                self.node_rth
            } else {
                // The promoted self-heating unknown is a temperature rise, so
                // it carries the identity the VBIC 1.3 reference module gives
                // it: `dt`, the first internal node the generated `vbic13` and
                // `vbic13_4t` models declare. A card that reroutes to one of
                // those keeps the same result column and the same checkpointed
                // state, which is what lets an image written by one build
                // resume under the other. `rth` is the thermal-resistance
                // parameter, not the state.
                alloc("dt")
            }
        } else {
            0
        };
        // The excess-phase network belongs to the VBIC charge model.
        if self.uses_vbic_dynamic_charges() && self.td > 0.0 {
            self.node_xf1 = alloc("xf1");
            self.node_xf2 = alloc("xf2");
        } else {
            self.node_xf1 = 0;
            self.node_xf2 = 0;
        }
        self.mna_promoted = true;
    }

    /// Physical GP junction voltages for accepted charge history.
    #[doc(hidden)]
    pub fn mna_junction_voltages(&self, solution: &[Value]) -> [Value; 3] {
        let value = |(internal, external): (Option<usize>, Option<usize>)| {
            let node = internal
                .map(|index| self.mna_internal_node(index))
                .or_else(|| external.map(|index| self.external_terminal_nodes()[index]))
                .unwrap_or(0);
            Self::node_voltage(solution, node)
        };
        [
            Self::node_voltage(solution, self.node_bi) - Self::node_voltage(solution, self.node_ei),
            Self::node_voltage(solution, self.node_bi) - Self::node_voltage(solution, self.node_ci),
            value(self.legacy_charge_substrate_connection_terminal())
                - value(self.legacy_charge_substrate_terminal()),
        ]
    }

    /// Matrix node for a dynamic internal state index (collapsed states alias
    /// their parent node; disabled states map to ground).
    #[inline]
    #[doc(hidden)]
    pub fn mna_internal_node(&self, idx: usize) -> NodeId {
        match idx {
            IDX_VCX => self.node_cx,
            IDX_VCI => self.node_ci,
            IDX_VBX => self.node_bx,
            IDX_VBI => self.node_bi,
            IDX_VEI => self.node_ei,
            IDX_VBP => self.node_bp,
            IDX_VSI => self.node_si,
            IDX_VRTH => self.node_rth,
            IDX_VXF1 => self.node_xf1,
            IDX_VXF2 => self.node_xf2,
            _ => 0,
        }
    }

    /// Attribute an aliased internal endpoint to its physical lead by device
    /// topology. Comparing global node numbers would merge tied C/B/E/S leads.
    #[doc(hidden)]
    pub fn mna_external_lead(
        &self,
        internal: Option<usize>,
        external: Option<usize>,
    ) -> Option<usize> {
        external.or_else(|| match internal? {
            IDX_VCX if !Self::series_active(self.rcx) => Some(EXT_C),
            IDX_VCI if !Self::series_active(self.rcx) && !Self::series_active(self.rci) => {
                Some(EXT_C)
            }
            IDX_VBX if !Self::series_active(self.rbx) => Some(EXT_B),
            IDX_VBI if !Self::series_active(self.rbx) && !Self::series_active(self.rbi) => {
                Some(EXT_B)
            }
            IDX_VEI if !Self::series_active(self.re) => Some(EXT_E),
            IDX_VBP if !self.vbic_solves_vbp() && !Self::series_active(self.rcx) => Some(EXT_C),
            IDX_VSI if !self.vbic_three_terminal && !self.has_substrate_resistance() => Some(EXT_S),
            _ => None,
        })
    }

    /// Whether internal state `idx` owns its own KCL row. Collapsed states are
    /// aliased onto a parent node whose row already carries their branch
    /// currents (via `external_terminal_branches` and the active rows), so
    /// they must not be stamped separately.
    #[inline]
    fn mna_internal_row_active(&self, idx: usize) -> bool {
        match idx {
            IDX_VCX => Self::series_active(self.rcx),
            IDX_VCI => Self::series_active(self.rci),
            IDX_VBX => Self::series_active(self.rbx),
            IDX_VBI => Self::series_active(self.rbi),
            IDX_VEI => Self::series_active(self.re),
            IDX_VBP => self.vbic_solves_vbp(),
            IDX_VSI => self.has_substrate_resistance(),
            IDX_VRTH => self.thermal_model_enabled(),
            _ => false,
        }
    }

    /// Whether any electrical internal state owns a node of its own rather
    /// than collapsing onto a terminal.
    pub(in crate::device::semiconductor::bjt) fn mna_has_private_internal_nodes(&self) -> bool {
        [
            IDX_VCX, IDX_VCI, IDX_VBX, IDX_VBI, IDX_VEI, IDX_VBP, IDX_VSI,
        ]
        .into_iter()
        .any(|idx| self.mna_internal_row_active(idx))
    }

    /// Residual-row orientation relative to the MNA "currents leaving the
    /// node" convention: the CX..SI residual rows are current-in minus
    /// current-out (flip), while the thermal and excess-phase rows already
    /// accumulate leaving terms.
    #[inline]
    fn vbic_residual_row_sign(idx: usize) -> Value {
        if idx < IDX_VRTH { -1.0 } else { 1.0 }
    }

    /// All matrix nodes this device couples once promoted, for sparsity
    /// reservations. Zero entries are ground/disabled and must be skipped.
    #[doc(hidden)]
    pub fn mna_coupling_nodes(&self) -> [NodeId; EXTERNAL_DIM + DYNAMIC_INTERNAL_DIM] {
        [
            self.node_collector,
            self.node_base,
            self.node_emitter,
            self.node_substrate,
            self.node_cx,
            self.node_ci,
            self.node_bx,
            self.node_bi,
            self.node_ei,
            self.node_bp,
            self.node_si,
            self.node_rth,
            self.node_xf1,
            self.node_xf2,
        ]
    }

    /// Internal state (including excess phase) at the current linearization
    /// point, in dynamic-state index order.
    #[inline]
    pub(crate) fn mna_internal_state(&self) -> [Value; BJT_INTERNAL_STATE_DIM] {
        [
            self.vcx, self.vci, self.vbx, self.vbi, self.vei, self.vbp, self.vsi, self.vrth,
            self.vxf1, self.vxf2,
        ]
    }

    /// External terminal voltages at the current linearization point.
    #[inline]
    pub(crate) fn mna_external_state(&self) -> [Value; EXTERNAL_DIM] {
        [self.vc_ext, self.vb_ext, self.ve_ext, self.vs_ext]
    }

    /// Keep ideal connections exact after prediction or junction limiting.
    /// Shared by the promoted and private solves, since their projected
    /// candidates can otherwise split nodes that denote the same wire.
    pub(super) fn impose_intrinsic_node_constraints(
        &self,
        state: &mut [Value; INTERNAL_DIM],
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
    ) {
        if !Self::series_active(self.rcx) {
            state[IDX_VCX] = vc;
        }
        if !Self::series_active(self.rci) {
            state[IDX_VCI] = state[IDX_VCX];
        }
        if !Self::series_active(self.rbx) {
            state[IDX_VBX] = vb;
        }
        if !Self::series_active(self.rbi) {
            state[IDX_VBI] = state[IDX_VBX];
        }
        if !Self::series_active(self.re) {
            state[IDX_VEI] = ve;
        }
        if !self.vbic_solves_vbp() {
            state[IDX_VBP] = state[IDX_VCX];
        }
        if !self.has_substrate_resistance() {
            state[IDX_VSI] = if self.vbic_three_terminal { 0.0 } else { vs };
        }
        if !self.thermal_model_enabled() {
            state[IDX_VRTH] = 0.0;
        }
    }

    /// This instance's whole read of a solution vector: its four terminal
    /// voltages followed by its ten internal states, before limiting.
    fn mna_solution_bias(
        &self,
        voltages: &[Value],
    ) -> [Value; EXTERNAL_DIM + BJT_INTERNAL_STATE_DIM] {
        let [vc, vb, ve, vs] = self.external_terminal_voltages(voltages);
        [
            vc,
            vb,
            ve,
            vs,
            Self::node_voltage(voltages, self.node_cx),
            Self::node_voltage(voltages, self.node_ci),
            Self::node_voltage(voltages, self.node_bx),
            Self::node_voltage(voltages, self.node_bi),
            Self::node_voltage(voltages, self.node_ei),
            Self::node_voltage(voltages, self.node_bp),
            Self::node_voltage(voltages, self.node_si),
            Self::node_voltage(voltages, self.node_rth),
            Self::node_voltage(voltages, self.node_xf1),
            Self::node_voltage(voltages, self.node_xf2),
        ]
    }

    fn update_mna_from_solution(&mut self, voltages: &[Value], apply_limiting: bool) {
        // Device update and matrix load are separate solver phases, so one
        // Newton iterate reaches this device twice: once when its candidate is
        // tested for device convergence and again when that same candidate is
        // stamped. pnjlim limits against the previous iterate rather than
        // being a function of the candidate alone, so evaluating one vector
        // twice retargets the limiter at its own output and lets each junction
        // travel roughly twice as far per iterate as ngspice's single
        // vbicload.c pass allows. Reuse the cached evaluation instead. The
        // legacy Gummel-Poon path holds the same invariant through its
        // reduced-linearization cache; the promoted path returns before that
        // guard and needs its own.
        let previous_iteration_available = self.mna_eval.is_some();
        let candidate = self.mna_solution_bias(voltages);
        let rbi_current = Self::node_voltage(voltages, self.mna_rbi_matrix_node);
        if apply_limiting
            && self.mna_eval.is_some()
            && self.mna_limited_from.get() == Some(candidate)
            && self.mna_rbi_current == rbi_current
        {
            // A solved voltage constraint can change only its reaction
            // current on the next Newton iteration. Once limiting is inactive,
            // compare the repeated evaluation against itself so the old voltage
            // delta cannot keep this exact candidate permanently unconverged.
            if self.mna_internal_state() == candidate[EXTERNAL_DIM..] {
                self.remember_mna_iteration();
            }
            return;
        }
        if !apply_limiting && self.mna_eval_is_unlimited_at(&candidate, rbi_current) {
            // A static probe of the bias the last load evaluated without
            // limiting it: the evaluation below would reproduce the cached
            // one bit for bit, so only the probe's bookkeeping runs.
            self.mna_limited_from.set(None);
            self.remember_mna_iteration();
            self.legacy_junction_limited = false;
            self.reduced_linearization_cache_valid.set(false);
            self.charge_snapshot_cache_valid.set(false);
            return;
        }
        self.mna_limited_from
            .set(apply_limiting.then_some(candidate));
        self.remember_mna_iteration();
        self.mna_rbi_current = rbi_current;

        let [mut vc, mut vb, mut ve, mut vs] =
            [candidate[0], candidate[1], candidate[2], candidate[3]];
        let mut raw = [0.0; INTERNAL_DIM];
        raw.copy_from_slice(&candidate[EXTERNAL_DIM..EXTERNAL_DIM + INTERNAL_DIM]);
        let previous = [
            self.vcx, self.vci, self.vbx, self.vbi, self.vei, self.vbp, self.vsi, self.vrth,
        ];
        let startup_load = apply_limiting && self.vbic_startup_load_pending;
        if apply_limiting {
            self.vbic_startup_load_pending = false;
        }
        self.legacy_junction_limited = false;
        self.mna_dissipation_drops = None;
        let mut state = if self.uses_legacy_gummel_poon() {
            let mut state = raw;
            if apply_limiting
                && (self.uses_legacy_junction_limiting()
                    || (self.initial_off && !previous_iteration_available))
            {
                let limited = self.limit_legacy_terminal_state_against_iterate(
                    self.intrinsic_state_from_internal_vector(raw),
                    previous_iteration_available,
                );
                state = [
                    limited.vcx,
                    limited.vci,
                    limited.vbx,
                    limited.vbi,
                    limited.vei,
                    limited.vbp,
                    limited.vsi,
                    limited.vrth,
                ];
                let before = self.legacy_nonlinear_branch_voltages(raw);
                let after = self.legacy_nonlinear_branch_voltages(state);
                self.legacy_junction_limited = (before.vbe - after.vbe).abs() > 1e-12
                    || (before.vbc - after.vbc).abs() > 1e-12
                    || (before.vsub - after.vsub).abs() > 1e-12;
                // Collapsed intrinsic and terminal coordinates share the limited
                // anchor; the correction stamp retains the raw matrix candidate.
                if !Self::series_active(self.rcx) && !Self::series_active(self.rci) {
                    vc = state[IDX_VCI];
                }
                if !Self::series_active(self.rbx) && !Self::series_active(self.rbi) {
                    vb = state[IDX_VBI];
                }
                if !Self::series_active(self.re) {
                    ve = state[IDX_VEI];
                }
                if !self.has_substrate_resistance() {
                    vs = state[IDX_VSI];
                }
            }
            if apply_limiting {
                self.legacy_initial_junction_pending = false;
            }
            state
        } else if let Some(startup) = startup_load
            .then(|| self.vbic_startup_load_internal_state(raw))
            .flatten()
        {
            // vbicload.c's MODEINITJCT load zeroes every resistor drop
            // (lines 255-256) along with the junctions it seeds.
            self.mna_dissipation_drops = Some([0.0; VBIC_PARASITIC_DROP_DIM]);
            startup
        } else if apply_limiting && self.voltage_limiting_enabled {
            let mut solution_bias = raw;
            self.impose_intrinsic_node_constraints(&mut solution_bias, vc, vb, ve, vs);
            self.mna_dissipation_drops =
                Some(Self::vbic_parasitic_drops([vc, vb, ve, vs], solution_bias));
            self.limit_vbic_internal_state_to_previous(raw, previous)
        } else {
            // A static probe, or a Newton load under `DEVICE VOLTLIM=0`,
            // evaluates the iterate as it stands: Xyce's `$limit` hands back
            // its argument when voltageLimiterFlag is clear.
            raw
        };
        self.impose_intrinsic_node_constraints(&mut state, vc, vb, ve, vs);
        self.eval_anchor = [vc, vb, ve, vs];

        let voltages = BjtNodeVoltages {
            vc,
            vb,
            ve,
            vs,
            vcx: state[IDX_VCX],
            vci: state[IDX_VCI],
            vbx: state[IDX_VBX],
            vbi: state[IDX_VBI],
            vei: state[IDX_VEI],
            vbp: state[IDX_VBP],
            vsi: state[IDX_VSI],
        };
        let transport = Cell::new(None);
        let mut eval = self.evaluate_state_with_rbi_current_and_transport(
            voltages,
            state[IDX_VRTH],
            self.mna_rbi_branch.map(|_| rbi_current),
            Some(&transport),
        );
        if let Some(drops) = self.mna_dissipation_drops {
            self.anchor_parasitics_at_drops(
                &mut eval,
                voltages,
                state[IDX_VRTH],
                drops,
                self.mna_rbi_branch.is_some(),
            );
        }
        let terminal_currents = self.external_terminal_branches(eval);

        self.vc_ext = vc;
        self.vb_ext = vb;
        self.ve_ext = ve;
        self.vs_ext = vs;
        self.vcx = state[IDX_VCX];
        self.vci = state[IDX_VCI];
        self.vbx = state[IDX_VBX];
        self.vbi = state[IDX_VBI];
        self.vei = state[IDX_VEI];
        self.vbp = state[IDX_VBP];
        self.vsi = state[IDX_VSI];
        self.vrth = state[IDX_VRTH];
        self.vxf1 = candidate[EXTERNAL_DIM + IDX_VXF1];
        self.vxf2 = candidate[EXTERNAL_DIM + IDX_VXF2];
        self.vbe = self.vbi - self.vei;
        self.vbc = self.vbi - self.vci;
        self.ic = terminal_currents[EXT_C].current;
        self.ib = terminal_currents[EXT_B].current;
        self.ie = terminal_currents[EXT_E].current;
        self.isub = terminal_currents[EXT_S].current;
        self.intrinsic_linearization = eval.linearized;
        self.mna_eval = Some(eval);
        self.mna_terminal_rows = MnaTerminalRows(Some(terminal_currents));
        // A parasitic-drop anchor expands the evaluation about drops that
        // are not the state's own.
        self.mna_eval_at_state = self.mna_dissipation_drops.is_none();
        self.refresh_mna_dynamic_state(transport.get());
        self.reduced_linearization_cache_valid.set(false);
        self.charge_snapshot_cache_valid.set(false);
    }

    /// Whether the cached evaluation, and the charge branches refreshed with
    /// it, are the model's at `candidate` read without limiting: the four
    /// terminal voltages as they stand and the internal ones under the ideal
    /// connections.
    fn mna_eval_is_unlimited_at(
        &self,
        candidate: &[Value; EXTERNAL_DIM + BJT_INTERNAL_STATE_DIM],
        rbi_current: Value,
    ) -> bool {
        if !self.mna_eval_at_state || self.mna_eval.is_none() || !self.mna_charge_cache_valid.get()
        {
            return false;
        }
        let [vc, vb, ve, vs] = [candidate[0], candidate[1], candidate[2], candidate[3]];
        let mut state = [0.0; INTERNAL_DIM];
        state.copy_from_slice(&candidate[EXTERNAL_DIM..EXTERNAL_DIM + INTERNAL_DIM]);
        self.impose_intrinsic_node_constraints(&mut state, vc, vb, ve, vs);
        let held = [
            self.vcx, self.vci, self.vbx, self.vbi, self.vei, self.vbp, self.vsi, self.vrth,
        ];
        same_bits(&self.eval_anchor, &[vc, vb, ve, vs])
            && same_bits(&held, &state)
            && same_bits(
                &[self.vxf1, self.vxf2],
                &candidate[EXTERNAL_DIM + IDX_VXF1..=EXTERNAL_DIM + IDX_VXF2],
            )
            && self.mna_rbi_current.to_bits() == rbi_current.to_bits()
    }

    /// Per-iteration update for the promoted device: read the internal node
    /// voltages from the global solution, apply ngspice junction limiting
    /// against the previous iterate (vbicload.c:656-670), and evaluate the
    /// branch system once at the limited bias.
    pub(super) fn update_mna(&mut self, voltages: &[Value]) {
        self.update_mna_from_solution(voltages, true);
    }

    fn remember_mna_iteration(&mut self) {
        if self.uses_legacy_gummel_poon() {
            self.previous_reduced_linearization_valid = self.mna_eval.is_some();
        }
        self.vbe_prev = self.vbe;
        self.vbc_prev = self.vbc;
        self.vcx_prev = self.vcx;
        self.vbi_prev = self.vbi;
        self.vci_prev = self.vci;
        self.vbx_prev = self.vbx;
        self.vei_prev = self.vei;
        self.vbp_prev = self.vbp;
        self.vsi_prev = self.vsi;
        self.vrth_prev = self.vrth;
        self.ic_prev = self.ic;
        self.ib_prev = self.ib;
        self.ie_prev = self.ie;
        self.isub_prev = self.isub;
        self.intrinsic_linearization_prev = self.intrinsic_linearization;
    }

    /// Re-linearize directly at a static residual/validation candidate.
    ///
    /// Newton iterations must keep the ngspice limited companion, but residual
    /// probes must evaluate the physical candidate voltage itself so stalled
    /// limiter states are rejected and true operating-point roots are accepted.
    #[doc(hidden)]
    pub fn update_mna_static_probe(&mut self, voltages: &[Value]) {
        self.update_mna_from_solution(voltages, false);
    }

    /// A retained periodic solution owns the accepted bias, including OFF
    /// instances. Install the unlimited model there before starting Newton.
    #[doc(hidden)]
    pub fn seed_accepted_periodic_bias(&mut self, voltages: &[Value]) {
        if self.mna_promoted() {
            self.update_mna_from_solution(voltages, false);
            self.vbic_startup_load_pending = false;
        } else {
            let limiting = self.voltage_limiting_enabled;
            let off = self.initial_off;
            self.voltage_limiting_enabled = false;
            self.initial_off = false;
            self.reduced_linearization_cache_valid.set(false);
            self.update(voltages);
            self.voltage_limiting_enabled = limiting;
            self.initial_off = off;
            self.previous_reduced_linearization = self.reduced_linearization_cache.get();
        }
        self.remember_mna_iteration();
        self.previous_reduced_linearization_valid = true;
    }

    /// Recompute the dynamic charge branches and excess-phase rows at the
    /// limited bias just written by `update_mna`, from the Gummel-Poon
    /// transport its evaluation computed when that is at the same bias.
    fn refresh_mna_dynamic_state(&mut self, transport: Option<BiasedTransport>) {
        let internal = self.mna_internal_state();
        let external = self.mna_external_state();
        if self.uses_legacy_gummel_poon() {
            self.mna_charge_cache.set(
                self.legacy_dynamic_charge_branches_from(external, internal, false, transport),
            );
            self.mna_charge_cache_valid.set(true);
            self.mna_delay_branches = [BjtCurrentBranch::default(); VBIC_DELAY_BRANCH_COUNT];
            self.mna_delay_thermal = BjtCurrentBranch::default();
            return;
        }
        let (branches, inputs, d_itzf_d_vrth) =
            self.vbic_dynamic_charge_state_at_bias(external, internal, None);
        self.mna_charge_cache.set(branches);
        self.mna_charge_cache_valid.set(true);

        if self.td > 0.0 {
            let reduction = BjtDynamicReduction {
                internal_voltages: internal,
                external_voltages: external,
                vbic_transport: inputs.transport,
                vbic_d_itzf_d_vrth: d_itzf_d_vrth,
                ..Default::default()
            };
            self.mna_delay_branches = self.vbic_delay_static_branches(&reduction);
            self.mna_delay_thermal = self.vbic_delay_static_thermal_branch(&reduction);
        } else {
            self.mna_delay_branches = [BjtCurrentBranch::default(); VBIC_DELAY_BRANCH_COUNT];
            self.mna_delay_thermal = BjtCurrentBranch::default();
        }
    }

    /// Dynamic charge branches plus their linearization-point voltages, for
    /// the engine's transient companion and AC passes. Valid after `update`.
    #[doc(hidden)]
    pub fn mna_charge_state(
        &self,
    ) -> (
        [BjtChargeBranch; BJT_DYNAMIC_CHARGE_COUNT],
        [Value; BJT_INTERNAL_STATE_DIM],
        [Value; EXTERNAL_DIM],
    ) {
        let internal = self.mna_internal_state();
        let external = self.mna_external_state();
        if !self.mna_charge_cache_valid.get() {
            let branches = self.dynamic_charge_branches_at_bias(external, internal);
            self.mna_charge_cache.set(branches);
            self.mna_charge_cache_valid.set(true);
        }
        (self.mna_charge_cache.get(), internal, external)
    }

    #[doc(hidden)]
    pub fn mna_internal_state_at_solution(
        &self,
        voltages: &[Value],
    ) -> [Value; BJT_INTERNAL_STATE_DIM] {
        std::array::from_fn(|index| Self::node_voltage(voltages, self.mna_internal_node(index)))
    }

    /// Charge branches evaluated directly at a solution vector (history
    /// initialization, accepted-step commit, and LTE candidate paths). The
    /// bias is read raw: aliased states share their parent solution entry,
    /// and accepted/candidate points are evaluated where they stand rather
    /// than at a limited iterate.
    #[doc(hidden)]
    pub fn mna_charge_state_at_solution(
        &self,
        voltages: &[Value],
    ) -> (
        [BjtChargeBranch; BJT_DYNAMIC_CHARGE_COUNT],
        [Value; BJT_INTERNAL_STATE_DIM],
        [Value; EXTERNAL_DIM],
    ) {
        self.mna_charge_state_at_solution_with_forward_limit(voltages, false)
    }

    #[doc(hidden)]
    pub fn mna_charge_state_at_solution_with_forward_limit(
        &self,
        voltages: &[Value],
        forward_limit: bool,
    ) -> (
        [BjtChargeBranch; BJT_DYNAMIC_CHARGE_COUNT],
        [Value; BJT_INTERNAL_STATE_DIM],
        [Value; EXTERNAL_DIM],
    ) {
        let external = self.external_terminal_voltages(voltages);
        let internal = self.mna_internal_state_at_solution(voltages);
        let branches = if forward_limit && self.uses_legacy_gummel_poon() {
            self.legacy_dynamic_charge_branches_with_forward_limit(external, internal, true)
        } else if self.mna_charge_cache_is_at(external, internal) {
            // The last evaluation was of this very bias.
            self.mna_charge_cache.get()
        } else {
            self.dynamic_charge_branches_at_bias(external, internal)
        };
        (branches, internal, external)
    }

    /// Whether the charge branches refreshed with the cached evaluation are
    /// those of exactly this bias.
    fn mna_charge_cache_is_at(
        &self,
        external: [Value; EXTERNAL_DIM],
        internal: [Value; BJT_INTERNAL_STATE_DIM],
    ) -> bool {
        self.mna_eval_at_state
            && self.mna_charge_cache_valid.get()
            && same_bits(&self.mna_external_state(), &external)
            && same_bits(&self.mna_internal_state(), &internal)
    }

    /// The cached evaluation, when it is the model's at exactly the bias
    /// `bias` (a solution read by `mna_solution_bias`) and RBI branch current
    /// `rbi_current`.
    fn mna_eval_at(
        &self,
        bias: &[Value; EXTERNAL_DIM + BJT_INTERNAL_STATE_DIM],
        rbi_current: Value,
    ) -> Option<EvaluatedBjtState> {
        let held = [
            self.vcx, self.vci, self.vbx, self.vbi, self.vei, self.vbp, self.vsi, self.vrth,
        ];
        let at = self.mna_eval_at_state
            && same_bits(&self.eval_anchor, &bias[..EXTERNAL_DIM])
            && same_bits(&held, &bias[EXTERNAL_DIM..EXTERNAL_DIM + INTERNAL_DIM])
            && self.mna_rbi_current.to_bits() == rbi_current.to_bits();
        self.mna_eval.filter(|_| at)
    }

    #[doc(hidden)]
    pub fn mna_terminal_currents_at_solution(&self, solution: &[Value]) -> [Value; EXTERNAL_DIM] {
        let v = self.mna_solution_bias(solution);
        let delayed = self.uses_vbic_dynamic_charges() && self.td > 0.0;
        let rbi_current = Self::node_voltage(solution, self.mna_rbi_matrix_node);
        if !delayed && let Some(eval) = self.mna_eval_at(&v, rbi_current) {
            // The last evaluation was of this very bias.
            return self
                .external_terminal_branches(eval)
                .map(|branch| branch.current);
        }
        let eval = self.evaluate_state_with_rbi_current(
            BjtNodeVoltages {
                vc: v[0],
                vb: v[1],
                ve: v[2],
                vs: v[3],
                vcx: v[EXTERNAL_DIM + IDX_VCX],
                vci: v[EXTERNAL_DIM + IDX_VCI],
                vbx: v[EXTERNAL_DIM + IDX_VBX],
                vbi: v[EXTERNAL_DIM + IDX_VBI],
                vei: v[EXTERNAL_DIM + IDX_VEI],
                vbp: v[EXTERNAL_DIM + IDX_VBP],
                vsi: v[EXTERNAL_DIM + IDX_VSI],
            },
            v[EXTERNAL_DIM + IDX_VRTH],
            self.mna_rbi_branch
                .map(|_| Self::node_voltage(solution, self.mna_rbi_matrix_node)),
        );
        let mut terminal = self
            .external_terminal_branches(eval)
            .map(|branch| branch.current);
        if self.uses_vbic_dynamic_charges() && self.td > 0.0 {
            // The accepted solution owns the delay coordinates. Cached Newton
            // branches may still refer to a limited iterate.
            let external = self.external_terminal_voltages(solution);
            let internal = self.mna_internal_state_at_solution(solution);
            let (_, inputs, d_itzf_d_vrth) =
                self.vbic_dynamic_charge_state_at_bias(external, internal, None);
            let reduction = BjtDynamicReduction {
                internal_voltages: internal,
                external_voltages: external,
                vbic_transport: inputs.transport,
                vbic_d_itzf_d_vrth: d_itzf_d_vrth,
                ..Default::default()
            };
            for branch in self.vbic_delay_static_branches(&reduction) {
                if let Some(index) =
                    self.mna_external_lead(branch.pos_internal, branch.pos_external)
                {
                    terminal[index] += branch
                        .pos_internal
                        .map_or(1.0, Self::vbic_residual_row_sign)
                        * branch.current;
                }
                if let Some(index) =
                    self.mna_external_lead(branch.neg_internal, branch.neg_external)
                {
                    terminal[index] -= branch
                        .neg_internal
                        .map_or(1.0, Self::vbic_residual_row_sign)
                        * branch.current;
                }
            }
        }
        terminal
    }

    /// Authored lead F/Q at the unlimited periodic bias most recently sampled.
    /// Keep terminal identities before global node stamping merges tied leads.
    #[doc(hidden)]
    pub fn periodic_lead_fq(
        &self,
        solution: &[Value],
    ) -> Result<([Value; EXTERNAL_DIM], [Value; EXTERNAL_DIM]), String> {
        self.mna_eval
            .ok_or_else(|| format!("BJT '{}' has no periodic bias", self.name))?;
        let current = self.mna_terminal_currents_at_solution(solution);
        let mut charge = [0.0; EXTERNAL_DIM];
        for (index, branch) in self.mna_charge_state().0.iter().enumerate() {
            let value = self.charge_branch_polarity(index) * branch.charge;
            if let Some(terminal) = self.mna_external_lead(branch.pos_internal, branch.pos_external)
            {
                charge[terminal] += value;
            }
            if let Some(terminal) = self.mna_external_lead(branch.neg_internal, branch.neg_external)
            {
                charge[terminal] -= value;
            }
        }
        let current = self.authored_transient_lead_currents(solution, current, 0.0)?;
        let external_charge = self
            .legacy_external_bc_charge(solution)
            .map_or(0.0, |charge| charge.charge);
        let charge = self.authored_transient_lead_impulses(charge, external_charge)?;
        if current
            .iter()
            .chain(&charge)
            .any(|value| !value.is_finite())
        {
            return Err(format!(
                "BJT '{}' has non-finite periodic lead F/Q",
                self.name
            ));
        }
        Ok((current, charge))
    }

    /// Prepare the native physical MNA topology for a periodic evaluator.
    /// Netlist construction already allocates every non-collapsed unknown.
    #[doc(hidden)]
    pub fn prepare_periodic_mna(&mut self, num_nodes: usize) -> Result<(), String> {
        if !self.mna_promoted() {
            if self.has_intrinsic_state_unknowns() {
                return Err(format!(
                    "BJT '{}' has unbound periodic internal states",
                    self.name
                ));
            }
            self.assign_mna_internal_nodes(|_| {
                unreachable!("collapsed native BJT needs no new node")
            });
        }
        self.resolve_mna_rbi_branch(num_nodes);
        Ok(())
    }

    /// Sample physical F/Q and their Jacobians at an unlimited state. Stamped
    /// RHS values are -F and -Q (not absolute Newton companion sources); both
    /// matrices contain positive derivatives of the physical equations. The
    /// canonical RBI port separately owns its two linear electrical KCL terms.
    #[doc(hidden)]
    pub fn stamp_periodic_fq(
        &mut self,
        solution: &[Value],
        static_part: &mut impl MatrixStamper,
        charge_part: &mut impl MatrixStamper,
    ) {
        self.stamp_periodic_fq_with_forward_limit(solution, static_part, charge_part, false);
    }

    #[doc(hidden)]
    pub fn stamp_periodic_fq_with_forward_limit(
        &mut self,
        solution: &[Value],
        static_part: &mut impl MatrixStamper,
        charge_part: &mut impl MatrixStamper,
        forward_limit: bool,
    ) {
        self.update_mna_static_probe(solution);
        self.stamp_mna_at(
            &mut PeriodicJacobian {
                stamper: static_part,
                rbi: self.mna_rbi_matrix_node,
                base: [self.node_bx, self.node_bi],
            },
            Some(solution),
            true,
        );
        self.stamp_periodic_static_terms(static_part);
        let external_nodes = self.external_terminal_nodes();
        let branches = if forward_limit && self.uses_legacy_gummel_poon() {
            self.legacy_dynamic_charge_branches_with_forward_limit(
                self.mna_external_state(),
                self.mna_internal_state(),
                true,
            )
        } else {
            self.mna_charge_state().0
        };
        for (index, branch) in branches.iter().enumerate() {
            let polarity = self.charge_branch_polarity(index);
            let node = |internal: Option<usize>, external: Option<usize>| {
                internal
                    .map(|index| self.mna_internal_node(index))
                    .or_else(|| external.map(|index| external_nodes[index]))
                    .unwrap_or(0)
            };
            let pos = node(branch.pos_internal, branch.pos_external);
            let neg = node(branch.neg_internal, branch.neg_external);
            if pos == neg {
                continue;
            }
            for (row, sign) in [(pos, polarity), (neg, -polarity)] {
                if row == 0 {
                    continue;
                }
                charge_part.stamp_rhs(row, -sign * branch.charge);
                for (column, &derivative) in branch.d_internal.iter().enumerate() {
                    if derivative != 0.0 {
                        charge_part.stamp(row, self.mna_internal_node(column), sign * derivative);
                    }
                }
                for (&column, &derivative) in external_nodes.iter().zip(&branch.d_external) {
                    if derivative != 0.0 {
                        charge_part.stamp(row, column, sign * derivative);
                    }
                }
            }
        }
        // A constant GP base lead can be externalized by the builder. Its
        // original-base collector charge then lives outside the intrinsic
        // branch array and must still enter the full periodic circuit.
        if let Some(charge) = self.legacy_external_bc_charge(solution) {
            let [pos, neg] = charge.nodes;
            for (row, sign) in [(pos, 1.0), (neg, -1.0)] {
                charge_part.stamp_rhs(row, -sign * charge.charge);
                charge_part.stamp(row, pos, sign * charge.capacitance);
                charge_part.stamp(row, neg, -sign * charge.capacitance);
            }
        }
    }

    /// Emit each physical term separately, in a stable topology order, so HB
    /// can Fourier-transform before summing its convergence magnitudes. The
    /// canonical RBI port owns its two electrical current incidences.
    fn stamp_periodic_static_terms(&self, stamper: &mut impl MatrixStamper) {
        let Some(eval) = self.mna_eval else {
            return;
        };
        let state = IntrinsicTerminalState {
            vcx: self.vcx,
            vci: self.vci,
            vbx: self.vbx,
            vbi: self.vbi,
            vei: self.vei,
            vbp: self.vbp,
            vsi: self.vsi,
            vrth: self.vrth,
        };
        let external_rbi_port = self.mna_rbi_matrix_node != 0;
        self.visit_internal_kcl_terms(
            state,
            &eval,
            self.mna_external_state(),
            external_rbi_port,
            self.mna_dissipation_drops,
            |row, terms| {
                let sign = Self::vbic_residual_row_sign(row);
                for term in terms {
                    stamper.stamp_rhs(self.mna_internal_node(row), -sign * term.current);
                }
            },
        );
        let external_nodes = self.external_terminal_nodes();
        self.visit_external_kcl_terms(eval, external_rbi_port, |row, terms| {
            for term in terms {
                stamper.stamp_rhs(external_nodes[row], -term.current);
            }
        });
        if external_rbi_port {
            let resistance = self.mna_rbi_stamp_resistance(eval.linearized, true).current;
            stamper.stamp_rhs(self.mna_rbi_matrix_node, -(self.vbx - self.vbi));
            stamper.stamp_rhs(self.mna_rbi_matrix_node, resistance * self.mna_rbi_current);
        }
        // Delay equations are affine in the transport-current coordinates.
        // Their linear terms must not disappear into a near-zero DC balance.
        for branch in self
            .mna_delay_branches
            .iter()
            .chain([&self.mna_delay_thermal])
        {
            if !branch.is_active() {
                continue;
            }
            let linear = [
                branch.d_internal[IDX_VXF1] * self.vxf1,
                branch.d_internal[IDX_VXF2] * self.vxf2,
            ];
            let remainder = branch.current - linear.iter().sum::<Value>();
            self.visit_vbic_residual_rows(branch, |row, sign| {
                for term in linear.into_iter().chain([remainder]) {
                    stamper.stamp_rhs(row, -sign * term);
                }
            });
        }
    }

    /// Stamp the full promoted static system: the four terminal KCL rows, the
    /// active internal KCL rows, and the excess-phase algebraic rows, all
    /// linearized at the limited bias from the last `update`.
    pub(in crate::device::semiconductor::bjt) fn stamp_mna(
        &self,
        stamper: &mut impl MatrixStamper,
    ) {
        self.stamp_mna_at(stamper, None, false);
    }

    /// Stamp J and -F directly for a Newton correction at `anchor`. If the
    /// cached evaluation was junction-limited, retain J*(limited-anchor).
    /// This avoids subtracting large absolute-voltage companions to recover
    /// small physical currents at steep thermal slopes.
    #[doc(hidden)]
    pub fn stamp_mna_correction(&self, stamper: &mut impl MatrixStamper, anchor: &[Value]) {
        self.stamp_mna_at(stamper, Some(anchor), false);
    }

    /// [`Self::stamp_mna`] into `newton` and the right-hand side of
    /// [`Self::stamp_mna_correction`] at `anchor` into `correction`, from one
    /// pass over the cached evaluation. The Jacobian goes to `newton` alone;
    /// each right-hand side receives exactly the additions its own stamp makes.
    #[doc(hidden)]
    pub fn stamp_mna_with_correction(
        &self,
        newton: &mut impl MatrixStamper,
        correction: &mut impl MatrixStamper,
        anchor: &[Value],
    ) {
        match self.promoted_tied_terminal_reference() {
            Some(reference) => self.stamp_mna_unprojected(
                &mut NewtonAndCorrection {
                    newton: &mut self.tied_terminal_projection(newton, reference),
                    correction: &mut self.tied_terminal_projection(correction, reference),
                    anchor,
                },
                false,
            ),
            None => self.stamp_mna_unprojected(
                &mut NewtonAndCorrection {
                    newton,
                    correction,
                    anchor,
                },
                false,
            ),
        }
    }

    /// The node a promoted Gummel-Poon device's collector, base or emitter
    /// share with another of them, when it is not ground. It is the
    /// reference [`Self::project_legacy_tied_terminal_system`] reduces the
    /// branch form onto; a ground reference needs no projection, because
    /// ground's row and column are never stamped.
    #[doc(hidden)]
    pub fn promoted_tied_terminal_reference(&self) -> Option<NodeId> {
        if !self.mna_promoted || !self.uses_legacy_gummel_poon() {
            return None;
        }
        let [collector, base, emitter, _] = self.external_terminal_nodes();
        let reference = if collector == base || collector == emitter {
            collector
        } else if base == emitter {
            base
        } else {
            return None;
        };
        (reference != 0).then_some(reference)
    }

    /// `stamper` seen through the tied-terminal projection onto `reference`.
    #[doc(hidden)]
    pub fn tied_terminal_projection<'a, S: MatrixStamper>(
        &self,
        stamper: &'a mut S,
        reference: NodeId,
    ) -> TiedTerminalProjection<'a, S> {
        TiedTerminalProjection {
            stamper,
            reference,
            electrical: [
                self.node_collector,
                self.node_base,
                self.node_emitter,
                self.node_substrate,
                self.node_cx,
                self.node_ci,
                self.node_bx,
                self.node_bi,
                self.node_ei,
                self.node_bp,
                self.node_si,
            ],
        }
    }

    fn stamp_mna_at(
        &self,
        stamper: &mut impl MatrixStamper,
        anchor: Option<&[Value]>,
        exact: bool,
    ) {
        // Only Newton loads project: the periodic evaluator's exact F/Q
        // samples go through their own adapter.
        match self.promoted_tied_terminal_reference().filter(|_| !exact) {
            Some(reference) => self.stamp_mna_unprojected(
                &mut SingleLoad {
                    stamper: &mut self.tied_terminal_projection(stamper, reference),
                    anchor,
                },
                exact,
            ),
            None => self.stamp_mna_unprojected(&mut SingleLoad { stamper, anchor }, exact),
        }
    }

    fn stamp_mna_unprojected<T: MnaLoadTargets>(&self, targets: &mut T, exact: bool) {
        let Some(eval) = self.mna_eval.as_ref() else {
            return;
        };
        // The load consumes this iterate's cached evaluation. A voltage
        // constraint can repeat the raw candidate on the next iteration even
        // while limiting remains active; keeping the cache then stalls it.
        self.mna_limited_from.set(None);
        let state = IntrinsicTerminalState {
            vcx: self.vcx,
            vci: self.vci,
            vbx: self.vbx,
            vbi: self.vbi,
            vei: self.vei,
            vbp: self.vbp,
            vsi: self.vsi,
            vrth: self.vrth,
        };
        let [vc, vb, ve, vs] = self.mna_external_state();
        let internal_nodes: [NodeId; INTERNAL_DIM] = [
            self.node_cx,
            self.node_ci,
            self.node_bx,
            self.node_bi,
            self.node_ei,
            self.node_bp,
            self.node_si,
            self.node_rth,
        ];
        let external_nodes = self.external_terminal_nodes();
        // Each right-hand side's state, relative to its own anchor.
        let mut internal = [self.mna_internal_state(); 2];
        let mut external = [[vc, vb, ve, vs]; 2];
        for lane in 0..T::LANES {
            if let Some(anchor) = targets.anchor(lane) {
                for (index, voltage) in internal[lane].iter_mut().enumerate() {
                    *voltage -= Self::node_voltage(anchor, self.mna_internal_node(index));
                }
                for (voltage, node) in external[lane].iter_mut().zip(external_nodes) {
                    *voltage -= Self::node_voltage(anchor, node);
                }
            }
        }
        let static_internal: [&[Value; INTERNAL_DIM]; 2] =
            [0, 1].map(|lane| internal[lane][..INTERNAL_DIM].try_into().unwrap());

        // Active internal KCL rows (residual orientation, flipped onto the
        // MNA leaving-current convention), in row order, each stamped as it
        // is summed.
        self.visit_internal_kcl_terms(
            state,
            eval,
            [vc, vb, ve, vs],
            false,
            self.mna_dissipation_drops,
            |row, terms| {
                let branch = Self::sum_branch_terms(terms);
                let sign = Self::vbic_residual_row_sign(row);
                let row_node = internal_nodes[row];
                for (&node, &derivative) in internal_nodes.iter().zip(&branch.d_internal) {
                    if derivative != 0.0 {
                        targets.stamp(row_node, node, sign * derivative);
                    }
                }
                for (&node, &derivative) in external_nodes.iter().zip(&branch.d_external) {
                    if derivative != 0.0 {
                        targets.stamp(row_node, node, sign * derivative);
                    }
                }
                let source = targets.sources(&branch, static_internal, &external);
                targets.stamp_rhs_lanes(row_node, source.map(|source| sign * source));
            },
        );

        // External terminal rows: current into the device from each terminal,
        // with collapse-aware branch selection. The update that cached this
        // evaluation summed them already.
        let terminal_currents = self
            .mna_terminal_rows
            .0
            .unwrap_or_else(|| self.external_terminal_branches(*eval));
        debug_assert!(
            terminal_currents
                .iter()
                .zip(self.external_terminal_branches(*eval))
                .all(
                    |(kept, summed)| kept.current.to_bits() == summed.current.to_bits()
                        && same_bits(&kept.d_internal, &summed.d_internal)
                        && same_bits(&kept.d_external, &summed.d_external)
                ),
            "the kept terminal rows are not the cached evaluation's"
        );
        for row in 0..EXTERNAL_DIM {
            let row_node = external_nodes[row];
            let branch = terminal_currents[row];
            let source = targets.sources(&branch, static_internal, &external);
            for (&node, &derivative) in internal_nodes.iter().zip(&branch.d_internal) {
                if derivative != 0.0 {
                    targets.stamp(row_node, node, derivative);
                }
            }
            for (&node, &derivative) in external_nodes.iter().zip(&branch.d_external) {
                if derivative != 0.0 {
                    targets.stamp(row_node, node, derivative);
                }
            }
            targets.stamp_rhs_lanes(row_node, source);
        }

        if self.mna_rbi_matrix_node != 0 {
            self.stamp_mna_rbi_current(targets, eval.linearized, &internal, &external, exact);
        }

        // Excess-phase network: algebraic xf rows plus the xf2-controlled
        // transport replacement (and its thermal power correction).
        if self.td > 0.0 {
            for branch in &self.mna_delay_branches {
                self.stamp_vbic_residual_branch(targets, branch, &internal, &external);
            }
            if self.thermal_model_enabled() {
                self.stamp_vbic_residual_branch(
                    targets,
                    &self.mna_delay_thermal,
                    &internal,
                    &external,
                );
            }
        }
    }

    fn mna_rbi_stamp_resistance(
        &self,
        linearized: BjtLinearization,
        exact: bool,
    ) -> BranchLinearization {
        if exact && self.uses_legacy_gummel_poon() {
            self.legacy_gp_base_resistance_law::<true>(
                linearized,
                self.guarded_series_resistance(self.rbi),
            )
        } else {
            self.mna_rbi_resistance(linearized, self.vrth)
        }
    }

    /// Add a physical GP transport correction at the cached limited bias.
    /// The ordinary static load freezes RBI; its independent-current equation
    /// needs -I*dR/dV to complete the coherent transient tangent. This does
    /// not alter the resistance current or use VBIC residual row orientations.
    #[doc(hidden)]
    pub fn stamp_legacy_mna_phase_correction(
        &self,
        stamper: &mut impl MatrixStamper,
        correction: &BjtCurrentBranch,
        static_weight: Value,
        anchor: Option<&[Value]>,
    ) -> Result<(), String> {
        if !self.mna_promoted() || !self.uses_legacy_gummel_poon() {
            return Err("GP phase correction requires a promoted GP device".into());
        }
        let mut internal = self.mna_internal_state();
        let mut external = self.mna_external_state();
        if let Some(anchor) = anchor {
            for (index, value) in internal.iter_mut().enumerate() {
                *value -= Self::node_voltage(anchor, self.mna_internal_node(index));
            }
            for (value, node) in external.iter_mut().zip(self.external_terminal_nodes()) {
                *value -= Self::node_voltage(anchor, node);
            }
        }
        let source = correction.linearization_dot(&internal, &external) - correction.current;
        let mut resistance_gradient = BranchLinearization::default();
        if Self::series_active(self.rbi) {
            if self.mna_rbi_matrix_node == 0 {
                return Err("GP phase correction requires its bound RBI current unknown".into());
            }
            let eval = self
                .mna_eval
                .ok_or("GP phase correction has no cached model evaluation")?;
            let resistance = self.mna_rbi_stamp_resistance(eval.linearized, true);
            if !resistance.current.is_finite() || resistance.current <= 0.0 {
                return Err("GP phase base resistance is not finite and positive".into());
            }
            for (gradient, derivative) in resistance_gradient
                .d_internal
                .iter_mut()
                .zip(resistance.d_internal)
            {
                *gradient = -self.mna_rbi_current * (static_weight * derivative);
            }
        }
        let resistance_source =
            resistance_gradient.source(internal[..INTERNAL_DIM].try_into().unwrap(), &external);
        if !source.is_finite()
            || !resistance_source.is_finite()
            || correction
                .d_internal
                .iter()
                .chain(&correction.d_external)
                .chain(&resistance_gradient.d_internal)
                .any(|value| !value.is_finite())
        {
            return Err("GP phase companion is not representable".into());
        }
        let external_nodes = self.external_terminal_nodes();
        let node = |internal: Option<usize>, external: Option<usize>| {
            internal
                .map(|index| self.mna_internal_node(index))
                .or_else(|| external.map(|index| external_nodes[index]))
                .unwrap_or(0)
        };
        // A ground row reaches the stamper too, which discards it; the
        // tied-terminal projection needs every row to recover KCL.
        for (row, sign) in [
            (node(correction.pos_internal, correction.pos_external), 1.0),
            (node(correction.neg_internal, correction.neg_external), -1.0),
        ] {
            for (column, derivative) in correction.d_internal.iter().enumerate() {
                if *derivative != 0.0 {
                    stamper.stamp(row, self.mna_internal_node(column), sign * derivative);
                }
            }
            for (column, derivative) in external_nodes.iter().zip(&correction.d_external) {
                if *derivative != 0.0 {
                    stamper.stamp(row, *column, sign * derivative);
                }
            }
            stamper.stamp_rhs(row, sign * source);
        }
        if self.mna_rbi_matrix_node != 0 {
            for (column, derivative) in resistance_gradient.d_internal.iter().enumerate() {
                if *derivative != 0.0 {
                    stamper.stamp(
                        self.mna_rbi_matrix_node,
                        self.mna_internal_node(column),
                        *derivative,
                    );
                }
            }
            stamper.stamp_rhs(self.mna_rbi_matrix_node, resistance_source);
        }
        Ok(())
    }

    fn stamp_mna_rbi_current<T: MnaLoadTargets>(
        &self,
        targets: &mut T,
        linearized: BjtLinearization,
        internal: &[[Value; BJT_INTERNAL_STATE_DIM]; 2],
        external: &[[Value; EXTERNAL_DIM]; 2],
        exact: bool,
    ) {
        let branch = self.mna_rbi_matrix_node;
        let current = self.mna_rbi_current;
        let coordinate = targets.lanes(|lane| {
            current
                - targets
                    .anchor(lane)
                    .map_or(0.0, |point| Self::node_voltage(point, branch))
        });
        // The ordinary KCL/heat stamp already includes the evaluated current.
        // Supply its independent column and the matching limiter offset.
        for (node, sign) in [(self.node_bx, 1.0), (self.node_bi, -1.0)] {
            targets.stamp(node, branch, sign);
            targets.stamp_rhs_lanes(node, coordinate.map(|coordinate| sign * coordinate));
        }
        if self.thermal_model_enabled() && self.vbic_heat_generation {
            let derivative = -(self.vbx - self.vbi);
            targets.stamp(self.node_rth, branch, derivative);
            targets.stamp_rhs_lanes(
                self.node_rth,
                coordinate.map(|coordinate| derivative * coordinate),
            );
        }

        // Vbx - Vbi - (RBI/qb)*I = 0. The temperature and charge-control
        // partials use the solved current, not an unresolvable voltage drop.
        let resistance = self.mna_rbi_stamp_resistance(linearized, exact);
        let mut equation = Self::scale_branch(resistance, -current);
        equation.current += self.vbx - self.vbi;
        equation.d_internal[IDX_VBX] += 1.0;
        equation.d_internal[IDX_VBI] -= 1.0;
        for (index, derivative) in equation.d_internal.iter().copied().enumerate() {
            if derivative != 0.0 {
                targets.stamp(branch, self.mna_internal_node(index), derivative);
            }
        }
        targets.stamp(branch, branch, -resistance.current);
        // Cancel the linear voltage and R*I terms algebraically. Reconstructing
        // J*x-F and then subtracting R*I leaves a fictitious voltage source
        // even for constant R, which prevents tight residual convergence.
        let resistance_gradient = BranchLinearization {
            current: 0.0,
            ..resistance
        };
        let source = targets.lanes(|lane| {
            -current
                * resistance_gradient.source(
                    internal[lane][..INTERNAL_DIM].try_into().unwrap(),
                    &external[lane],
                )
                + targets.anchor(lane).map_or(0.0, |point| {
                    resistance.current * Self::node_voltage(point, branch)
                        - (Self::node_voltage(point, self.node_bx)
                            - Self::node_voltage(point, self.node_bi))
                })
        });
        targets.stamp_rhs_lanes(branch, source);
    }

    /// Stamp one residual-convention current branch onto the promoted rows,
    /// applying the per-row orientation flip.
    fn stamp_vbic_residual_branch<T: MnaLoadTargets>(
        &self,
        targets: &mut T,
        branch: &BjtCurrentBranch,
        internal: &[[Value; BJT_INTERNAL_STATE_DIM]; 2],
        external: &[[Value; EXTERNAL_DIM]; 2],
    ) {
        if !branch.is_active() {
            return;
        }
        let external_nodes = self.external_terminal_nodes();
        let source = targets.lanes(|lane| {
            branch.linearization_dot(&internal[lane], &external[lane]) - branch.current
        });

        // A ground row reaches the stamper too, which discards it; the
        // tied-terminal projection needs every row to recover KCL.
        let stamp_side = |row_node: NodeId, sign: Value| {
            for col in 0..BJT_INTERNAL_STATE_DIM {
                if branch.d_internal[col] != 0.0 {
                    targets.stamp(
                        row_node,
                        self.mna_internal_node(col),
                        sign * branch.d_internal[col],
                    );
                }
            }
            for (col, &external_node) in external_nodes.iter().enumerate().take(EXTERNAL_DIM) {
                if branch.d_external[col] != 0.0 {
                    targets.stamp(row_node, external_node, sign * branch.d_external[col]);
                }
            }
            targets.stamp_rhs_lanes(row_node, source.map(|source| sign * source));
        };

        self.visit_vbic_residual_rows(branch, stamp_side);
    }

    fn visit_vbic_residual_rows(
        &self,
        branch: &BjtCurrentBranch,
        mut visit: impl FnMut(NodeId, Value),
    ) {
        let external_nodes = self.external_terminal_nodes();
        if let Some(idx) = branch.pos_internal {
            visit(
                self.mna_internal_node(idx),
                Self::vbic_residual_row_sign(idx),
            );
        }
        if let Some(idx) = branch.neg_internal {
            visit(
                self.mna_internal_node(idx),
                -Self::vbic_residual_row_sign(idx),
            );
        }
        if let Some(idx) = branch.pos_external {
            visit(external_nodes[idx], 1.0);
        }
        if let Some(idx) = branch.neg_external {
            visit(external_nodes[idx], -1.0);
        }
    }
}

/// Whether two value lists hold the same bits, entry by entry.
fn same_bits(left: &[Value], right: &[Value]) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .zip(right)
            .all(|(left, right)| left.to_bits() == right.to_bits())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::traits::NonlinearDevice;

    /// Dense stamper capturing the promoted system for finite-difference
    /// Jacobian validation.
    struct DenseStamper {
        n: usize,
        a: Vec<Vec<Value>>,
        b: Vec<Value>,
    }

    impl DenseStamper {
        fn new(n: usize) -> Self {
            Self {
                n,
                a: vec![vec![0.0; n]; n],
                b: vec![0.0; n],
            }
        }

        /// Linearized residual A*v - b == f(v) at the stamp's own bias.
        fn residual(&self, v: &[Value]) -> Vec<Value> {
            (0..self.n)
                .map(|row| {
                    (0..self.n)
                        .map(|col| self.a[row][col] * v[col])
                        .sum::<Value>()
                        - self.b[row]
                })
                .collect()
        }
    }

    impl MatrixStamper for DenseStamper {
        fn stamp(&mut self, row: NodeId, col: NodeId, value: Value) {
            if row > 0 && col > 0 {
                self.a[row - 1][col - 1] += value;
            }
        }

        fn stamp_rhs(&mut self, index: NodeId, value: Value) {
            if index > 0 {
                self.b[index - 1] += value;
            }
        }
    }

    #[test]
    fn periodic_native_fq_matches_all_physical_mna_derivatives() {
        for (level, irb) in [
            (1.0, 0.0),
            (1.0, 1e-5),
            (4.0, 0.0),
            (11.0, 0.0),
            (12.0, 0.0),
        ] {
            for p in [1.0, -1.0] {
                let mut params = std::collections::HashMap::from_iter(
                    [
                        ("LEVEL", level),
                        ("IS", 1e-14),
                        ("BF", 80.0),
                        ("VAF", 30.0),
                        ("IKF", 1e-3),
                        ("RB", 2000.0),
                        ("RBM", 100.0),
                        ("IRB", irb),
                        ("RCX", 10.0),
                        ("RCI", 20.0),
                        ("RBX", 10.0),
                        ("RBI", 40.0),
                        ("RE", 1.0),
                        ("RBP", 10.0),
                        ("RS", 1.0),
                        ("IBEI", 1e-16),
                        ("IBCI", 1e-16),
                        ("ISP", 1e-16),
                        ("CJE", 10e-12),
                        ("CJC", 5e-12),
                        ("CJEP", 3e-12),
                        ("CJCP", 2e-12),
                        ("TF", 10e-9),
                        ("TR", 2e-9),
                        ("XCJC", 0.4),
                        ("WBE", 0.8),
                        ("QCO", 1e-14),
                        ("GAMM", 1e-9),
                    ]
                    .map(|(key, value)| (key.to_string(), value)),
                );
                if level != 1.0 {
                    for (name, value) in [
                        ("SELFT", 1.0),
                        ("RTH", 1000.0),
                        ("CTH", 1e-9),
                        ("TD", 100e-9),
                    ] {
                        params.insert(name.into(), value);
                    }
                }
                let mut bjt = if p > 0.0 {
                    Bjt::new_npn("QP".into(), 1, 2, 3)
                } else {
                    Bjt::new_pnp("QP".into(), 1, 2, 3)
                }
                .with_params(&params)
                .with_instance_params(&[("AREA".into(), 2.0), ("M".into(), 3.0)]);
                bjt.set_substrate_node(4);
                let mut next = 5;
                bjt.assign_mna_internal_nodes(|_| {
                    let node = next;
                    next += 1;
                    node
                });
                bjt.assign_mna_rbi_branch(1);
                bjt.prepare_periodic_mna(next - 1).unwrap();
                let n = next;
                let mut bias = vec![0.0; n];
                for (node, value) in [
                    (1, 1.2),
                    (2, 0.68),
                    (3, 0.0),
                    (4, 0.0),
                    (bjt.node_cx, 1.199),
                    (bjt.node_ci, 1.198),
                    (bjt.node_bx, 0.6799),
                    (bjt.node_bi, 0.677),
                    (bjt.node_ei, 0.0001),
                    (bjt.node_bp, 1.197),
                    (bjt.node_si, 0.00001),
                    (bjt.mna_rbi_matrix_node, 2e-5),
                ] {
                    if node != 0 {
                        bias[node - 1] = p * value;
                    }
                }
                if level != 1.0 {
                    assert!(bjt.node_rth != 0 && bjt.node_xf1 != 0 && bjt.node_xf2 != 0);
                    bias[bjt.node_rth - 1] = 20.0;
                    bias[bjt.node_xf1 - 1] = 2e-4;
                    bias[bjt.node_xf2 - 1] = 3e-4;
                }
                let sample = |bjt: &mut Bjt, point: &[Value]| {
                    let mut f = DenseStamper::new(n);
                    let mut q = DenseStamper::new(n);
                    bjt.stamp_periodic_fq(point, &mut f, &mut q);
                    let branch = bjt.mna_rbi_matrix_node;
                    for (row, sign) in [(bjt.node_bx, 1.0), (bjt.node_bi, -1.0)] {
                        f.stamp(row, branch, sign);
                        f.stamp_rhs(row, -sign * point[branch - 1]);
                    }
                    [f, q]
                };
                let base = sample(&mut bjt, &bias);
                for col in 0..n {
                    let step = if col + 1 == bjt.node_rth {
                        1e-3
                    } else if col == n - 1 || [bjt.node_xf1, bjt.node_xf2].contains(&(col + 1)) {
                        1e-8
                    } else {
                        1e-6
                    };
                    let mut plus = bias.clone();
                    plus[col] += step;
                    let mut minus = bias.clone();
                    minus[col] -= step;
                    let plus = sample(&mut bjt, &plus);
                    let minus = sample(&mut bjt, &minus);
                    for (kind, ((base, plus), minus)) in
                        base.iter().zip(&plus).zip(&minus).enumerate()
                    {
                        for row in 0..n {
                            let fd = -(plus.b[row] - minus.b[row]) / (2.0 * step);
                            let analytic = base.a[row][col];
                            let roundoff =
                                64.0 * Value::EPSILON * (plus.b[row].abs() + minus.b[row].abs())
                                    / step;
                            let floor = if kind == 0 { 1e-10 } else { 1e-20 };
                            assert!(
                                (fd - analytic).abs()
                                    <= 2e-5 * fd.abs().max(analytic.abs()) + roundoff + floor,
                                "LEVEL={level} IRB={irb} polarity={p} F/Q={kind} ({row},{col}): FD={fd:e}, analytic={analytic:e}"
                            );
                        }
                    }
                }
            }
        }
    }

    fn diffamp_pnp(level: Value) -> Bjt {
        let mut params = std::collections::HashMap::new();
        for (key, value) in [
            ("LEVEL", level),
            ("IS", 1e-16),
            ("IBEI", 1e-18),
            ("IBEN", 5e-15),
            ("IBCI", 2e-17),
            ("IBCN", 5e-15),
            ("ISP", 1e-15),
            ("RCX", 10.0),
            ("RCI", 60.0),
            ("RBX", 10.0),
            ("RBI", 40.0),
            ("RE", 2.0),
            ("RS", 20.0),
            ("RBP", 40.0),
            ("VEF", 10.0),
            ("VER", 4.0),
            ("IKF", 2e-3),
            ("ITF", 8e-2),
            ("XTF", 20.0),
            ("IKR", 2e-4),
            ("IKP", 2e-4),
            ("CJE", 1e-13),
            ("CJC", 2e-14),
            ("CJEP", 1e-13),
            ("CJCP", 4e-13),
            ("VO", 2.0),
            ("GAMM", 2e-11),
            ("HRCF", 2.0),
            ("QCO", 1e-12),
            ("AVC1", 2.0),
            ("AVC2", 15.0),
            ("TF", 10e-12),
            ("TR", 100e-12),
            ("TD", 2e-11),
            ("RTH", 300.0),
        ] {
            params.insert(key.to_string(), value);
        }
        if level == 11.0 {
            params.insert("TCVEF".into(), 0.05);
            params.insert("TCVER".into(), -0.01);
            params.insert("AVC1".into(), 0.2);
            params.insert("AVC2".into(), 0.3);
            params.insert("TAVC".into(), 0.01);
            params.insert("AVCX1".into(), 0.2);
            params.insert("AVCX2".into(), 0.3);
            params.insert("TAVCX".into(), 0.01);
        }
        // Diffamp cascode: collector node 1, base node 2, emitter node 3,
        // substrate tied to the collector node like the deck instances.
        let mut bjt = Bjt::new_pnp("QT".to_string(), 1, 2, 3);
        bjt = bjt.with_params(&params);
        bjt.set_substrate_node(1);
        let mut next = 4;
        bjt.assign_vbic_internal_nodes(|_| {
            let node = next;
            next += 1;
            node
        });
        bjt
    }

    #[test]
    fn vbic_direct_correction_retains_currents_at_a_steep_thermal_cutoff() {
        let coefficient = (Value::EPSILON - 1.0) / 20.0;
        for level in [11.0, 12.0] {
            for (model, p) in [
                (Bjt::new_npn("q".into(), 1, 2, 0), 1.0),
                (Bjt::new_pnp("q".into(), 1, 2, 0), -1.0),
            ] {
                let params = [
                    ("LEVEL", level),
                    ("VEF", 1e15),
                    ("VER", 3.0),
                    ("TCVEF", coefficient),
                    ("TCVER", -0.02),
                    ("IS", 1e-16),
                    ("IBEI", 1e-18),
                    ("IBCI", 1e-18),
                    ("RCX", 1.0),
                    ("RCI", 1.0),
                    ("RBX", 1.0),
                    ("RBI", 1.0),
                    ("RE", 1.0),
                    ("RTH", 1000.0),
                    ("TD", 1e-9),
                    ("GMIN", 0.0),
                ]
                .map(|(name, value)| (name.to_owned(), value))
                .into_iter()
                .collect();
                let mut bjt = model
                    .with_params(&params)
                    .with_instance_params(&[("M".into(), 3.0)]);
                bjt.set_vbic_external_thermal_node(3);
                let mut next = 4;
                bjt.assign_vbic_internal_nodes(|_| {
                    let node = next;
                    next += 1;
                    node
                });
                let mut bias = vec![0.0; next - 1];
                bias[0] = p * 0.6;
                bias[1] = p * 0.7;
                bias[2] = 20.0;
                for (index, value) in [0.6, 0.5998, 0.7, 0.69999, 0.0001, 0.6, 0.0]
                    .into_iter()
                    .enumerate()
                {
                    let node = bjt.mna_internal_node(index);
                    if node != 0 {
                        bias[node - 1] = p * value;
                    }
                }
                bias[bjt.node_xf1 - 1] = 1e-4;
                bias[bjt.node_xf2 - 1] = 2e-4;
                bjt.update_mna_static_probe(&bias);
                let internal = bjt.mna_internal_state();
                let (physical, _) = bjt.intrinsic_state_residual_jacobian(
                    bias[0],
                    bias[1],
                    0.0,
                    0.0,
                    internal[..INTERNAL_DIM].try_into().unwrap(),
                );
                let mut expected = vec![0.0; bias.len()];
                for (index, value) in physical.into_iter().enumerate() {
                    if bjt.mna_internal_row_active(index) {
                        expected[bjt.mna_internal_node(index) - 1] +=
                            Bjt::vbic_residual_row_sign(index) * value;
                    }
                }
                for (node, branch) in bjt
                    .external_terminal_nodes()
                    .into_iter()
                    .zip(bjt.external_terminal_branches(bjt.mna_eval.unwrap()))
                {
                    if node != 0 {
                        expected[node - 1] += branch.current;
                    }
                }
                for branch in bjt
                    .mna_delay_branches
                    .iter()
                    .chain([&bjt.mna_delay_thermal])
                {
                    for (index, sign) in [(branch.pos_internal, 1.0), (branch.neg_internal, -1.0)] {
                        if let Some(index) = index {
                            let node = bjt.mna_internal_node(index);
                            if node != 0 {
                                expected[node - 1] +=
                                    sign * Bjt::vbic_residual_row_sign(index) * branch.current;
                            }
                        }
                    }
                }
                let mut direct = DenseStamper::new(bias.len());
                bjt.stamp_mna_correction(&mut direct, &bias);
                for (row, (&rhs, &physical)) in direct.b.iter().zip(&expected).enumerate() {
                    assert!(
                        (rhs + physical).abs() < 1e-12 * physical.abs() + 1e-15,
                        "LEVEL={level} p={p} row={row}: {:e} != {physical:e}",
                        -rhs
                    );
                }
                let mut companion = DenseStamper::new(bias.len());
                bjt.stamp_mna(&mut companion);
                assert_eq!(direct.a, companion.a);
                assert!(
                    companion
                        .residual(&bias)
                        .iter()
                        .zip(expected)
                        .any(|(actual, expected)| (actual - expected).abs() > 1e-5)
                );
            }
        }
    }

    #[test]
    fn vbic13_noise_uses_the_prescribed_temperature_even_with_heat_generation_off() {
        let params = [("LEVEL", 11.0), ("RCX", 10.0), ("RTH", 100.0)]
            .map(|(name, value)| (name.to_owned(), value))
            .into_iter()
            .collect();
        let mut bjt = Bjt::new_npn("q".into(), 1, 2, 0)
            .with_params(&params)
            .with_instance_params(&[("SW_ET".to_string(), 0.0), ("TRISE".to_string(), 5.0)]);
        bjt.set_vbic_external_thermal_node(3);
        let mut next = 4;
        bjt.assign_vbic_internal_nodes(|_| {
            let node = next;
            next += 1;
            node
        });
        let mut v = vec![0.0; next - 1];
        v[2] = 20.0;
        bjt.update_mna_static_probe(&v);
        let noise = bjt.vbic_noise_operating_model().unwrap();
        assert_eq!(noise.absolute_temperature, Some(325.15));
        assert!(
            noise
                .thermal
                .iter()
                .any(|(_, _, _, conductance)| *conductance > 0.0)
        );
    }

    #[test]
    fn three_terminal_vbic_retains_parasitic_base_transport_and_diffusion_charge() {
        let params = std::collections::HashMap::from([
            ("LEVEL".to_string(), 11.0),
            ("RCI".to_string(), 0.0),
            ("RBI".to_string(), 0.0),
            ("RBP".to_string(), 100.0),
            ("ISP".to_string(), 1e-15),
            ("IKP".to_string(), 1e-4),
            ("WSP".to_string(), 0.4),
            ("TR".to_string(), 2e-9),
            ("CJEP".to_string(), 0.0),
            ("CJCP".to_string(), 1e-6),
            ("RS".to_string(), 100.0),
        ]);
        let mut bjt = Bjt::new_npn("q1".to_string(), 1, 2, 0).with_params(&params);
        let mut next = 3;
        bjt.assign_vbic_internal_nodes(|_| {
            let node = next;
            next += 1;
            node
        });
        let mut bias = vec![0.0; next - 1];
        bias[bjt.node_collector - 1] = 0.1;
        bias[bjt.node_base - 1] = 0.7;
        // VBIC 1.3 retains these internal nodes through its resistance floor.
        for node in [bjt.node_cx, bjt.node_ci] {
            bias[node - 1] = 0.1;
        }
        for node in [bjt.node_bx, bjt.node_bi] {
            bias[node - 1] = 0.7;
        }
        bias[bjt.node_bp - 1] = 0.05;
        // Independent threeTerminal equations from Xyce vbic_1p3.va:
        // Ifp remains in both qbp (Rbp) and TR*Ifp (Qbep) without Iccp.
        let vt = 1.380662e-23 * 300.15 / 1.602189e-19;
        let ifp = 1e-15 * (0.4 * (0.65_f64 / vt).exp() + 0.6 * (0.6_f64 / vt).exp() - 1.0);
        let qbp = 0.5 * (1.0 + (1.0 + 4.0 * ifp / 1e-4).sqrt());
        assert!(qbp > 1.05);
        let irbp = bjt.irbp_branch(0.7, 0.7, 0.1, 0.1, 0.05, 0.0);
        assert!((irbp.current - (-0.05 / 100.0) * qbp).abs() < 1e-15);
        let (charges, _, _) = bjt.mna_charge_state_at_solution(&bias);
        assert!((charges[4].charge - 2e-9 * ifp).abs() < 1e-22);
        assert!(!charges[7].is_active());
        assert_eq!(bjt.electrical_charge_storage_nodes()[7], None);
        assert_eq!(bjt.node_si, 0);
    }

    #[test]
    fn external_vbic_thermal_terminal_enables_rth_without_selft() {
        let params = std::collections::HashMap::from([
            ("LEVEL".to_string(), 4.0),
            ("RTH".to_string(), 100.0),
        ]);
        let mut bjt = Bjt::new_npn("q1".to_string(), 1, 2, 3).with_params(&params);

        assert!(
            !bjt.thermal_model_enabled(),
            "internal VBIC self-heating still requires SELFT when no external dt terminal is present"
        );

        bjt.set_vbic_external_thermal_node(4);
        let mut next = 5;
        bjt.assign_vbic_internal_nodes(|_| {
            let node = next;
            next += 1;
            node
        });

        assert!(bjt.thermal_model_enabled());
        assert_eq!(bjt.node_rth, 4);
    }

    /// The promoted stamp is a Newton linearization: for biases close enough
    /// that junction limiting stays inactive, f(v') - f(v) must match
    /// A(v) * (v' - v) to first order across every promoted column. A wrong or
    /// missing Jacobian entry shows up as a first-order mismatch.
    #[test]
    fn promoted_stamp_matches_finite_difference_jacobian() {
        for level in [4.0, 11.0] {
            assert_promoted_stamp_matches_finite_difference_jacobian(level);
        }
    }

    fn assert_promoted_stamp_matches_finite_difference_jacobian(level: Value) {
        let mut bjt = diffamp_pnp(level);
        bjt.set_vbic_external_thermal_node(14);
        bjt.assign_mna_rbi_branch(1);
        bjt.resolve_mna_rbi_branch(14);
        let n = 15;

        // Bias near the diffamp PNP operating point with the b-c junction at
        // the saturation knife edge (vbci slightly forward, PNP polarity).
        let mut v = vec![0.0; n];
        let assign = |v: &mut Vec<Value>, node: NodeId, value: Value| {
            if node > 0 {
                v[node - 1] = value;
            }
        };
        assign(&mut v, bjt.node_collector, 2.6234);
        assign(&mut v, bjt.node_base, 2.6180);
        assign(&mut v, bjt.node_emitter, 3.3000);
        assign(&mut v, bjt.node_cx, 2.6238);
        assign(&mut v, bjt.node_ci, 2.6252);
        assign(&mut v, bjt.node_bx, 2.6181);
        assign(&mut v, bjt.node_bi, 2.6178);
        assign(&mut v, bjt.node_si, 2.6235);
        assign(&mut v, bjt.node_bp, 2.6239);
        assign(&mut v, bjt.node_ei, 3.2999);
        assign(&mut v, bjt.node_xf1, 2.05e-5);
        assign(&mut v, bjt.node_xf2, 2.05e-5);
        assign(&mut v, bjt.node_rth, 20.0);
        assign(&mut v, bjt.mna_rbi_matrix_node, -1e-5);

        // Settle the limiter anchor at the bias so pnjlim stays inactive for
        // the FD probes. Handing the same candidate to `update` twice cannot do
        // it: the promoted update limits once per Newton iterate and reuses its
        // evaluation for a repeat. Step the device onto the bias, re-linearize
        // at the bias itself, then limit once from there — with the previous
        // iterate equal to the candidate, pnjlim is inactive and the anchor
        // lands exactly on `v`.
        bjt.update(&v);
        bjt.update_mna_static_probe(&v);
        bjt.update(&v);
        let mut base = DenseStamper::new(n);
        bjt.stamp_mna(&mut base);
        let f0 = base.residual(&v);

        let h = 1e-7;
        let mut worst: (Value, usize, usize) = (0.0, 0, 0);
        for col in 0..n {
            let mut vp = v.clone();
            vp[col] += h;
            bjt.update(&vp);
            let mut pert = DenseStamper::new(n);
            bjt.stamp_mna(&mut pert);
            let f1 = pert.residual(&vp);
            // Restore the limiter anchor for the next probe.
            bjt.update(&v);

            for row in 0..n {
                let fd = (f1[row] - f0[row]) / h;
                let analytic = base.a[row][col];
                let scale = analytic.abs().max(fd.abs()).max(1e-6);
                let err = (fd - analytic).abs() / scale;
                if err > worst.0 {
                    worst = (err, row, col);
                }
                assert!(
                    err < 5e-3,
                    "Jacobian mismatch at row {row} col {col}: fd={fd:.6e} analytic={analytic:.6e} rel_err={err:.3e}"
                );
            }
        }
        println!(
            "worst relative error {:.3e} at row {} col {}",
            worst.0, worst.1, worst.2
        );
    }

    /// The charge branches are the transient Jacobian: every dq/dv column
    /// must match a finite difference of the branch charge at the same bias,
    /// including at the saturation knife edge where the epi charge (qbcx)
    /// turns on exponentially.
    #[test]
    fn promoted_charge_branches_match_finite_difference() {
        for level in [4.0, 11.0] {
            assert_promoted_charge_branches_match_finite_difference(level);
        }
    }

    fn assert_promoted_charge_branches_match_finite_difference(level: Value) {
        let bjt = diffamp_pnp(level);
        let n = 13;

        let mut v = vec![0.0; n];
        let assign = |v: &mut Vec<Value>, node: NodeId, value: Value| {
            if node > 0 {
                v[node - 1] = value;
            }
        };
        assign(&mut v, bjt.node_collector, 2.6234);
        assign(&mut v, bjt.node_base, 2.6180);
        assign(&mut v, bjt.node_emitter, 3.3000);
        assign(&mut v, bjt.node_cx, 2.6238);
        assign(&mut v, bjt.node_ci, 2.6252);
        assign(&mut v, bjt.node_bx, 2.6181);
        assign(&mut v, bjt.node_bi, 2.6178);
        assign(&mut v, bjt.node_si, 2.6235);
        assign(&mut v, bjt.node_bp, 2.6239);
        assign(&mut v, bjt.node_ei, 3.2999);
        assign(&mut v, bjt.node_xf1, 2.05e-5);
        assign(&mut v, bjt.node_xf2, 2.05e-5);

        let (base_branches, base_internal, base_external) = bjt.mna_charge_state_at_solution(&v);

        let h = 1e-7;
        for col in 0..n {
            let mut vp = v.clone();
            vp[col] += h;
            let (pert_branches, pert_internal, pert_external) =
                bjt.mna_charge_state_at_solution(&vp);

            for branch_idx in 0..BJT_DYNAMIC_CHARGE_COUNT {
                if !base_branches[branch_idx].is_active() {
                    continue;
                }
                let fd = (pert_branches[branch_idx].charge - base_branches[branch_idx].charge) / h;
                let mut analytic = 0.0;
                for idx in 0..BJT_INTERNAL_STATE_DIM {
                    let d_int = (pert_internal[idx] - base_internal[idx]) / h;
                    analytic += base_branches[branch_idx].d_internal[idx] * d_int;
                }
                for idx in 0..EXTERNAL_DIM {
                    let d_ext = (pert_external[idx] - base_external[idx]) / h;
                    analytic += base_branches[branch_idx].d_external[idx] * d_ext;
                }
                let scale = analytic.abs().max(fd.abs()).max(1e-16);
                let err = (fd - analytic).abs() / scale;
                assert!(
                    err < 5e-3,
                    "charge Jacobian mismatch branch {branch_idx} col {col}: fd={fd:.6e} analytic={analytic:.6e} rel_err={err:.3e}"
                );
            }
        }
    }

    /// A solution vector holding every promoted node of `bjt` at its own
    /// terminal's potential, so no parasitic resistance carries a drop.
    fn promoted_bias(
        bjt: &Bjt,
        n: usize,
        collector: Value,
        base: Value,
        emitter: Value,
    ) -> Vec<Value> {
        let mut v = vec![0.0; n];
        for (node, value) in [
            (bjt.node_collector, collector),
            (bjt.node_cx, collector),
            (bjt.node_ci, collector),
            (bjt.node_bp, collector),
            (bjt.node_si, collector),
            (bjt.node_base, base),
            (bjt.node_bx, base),
            (bjt.node_bi, base),
            (bjt.node_emitter, emitter),
            (bjt.node_ei, emitter),
        ] {
            if node > 0 {
                v[node - 1] = value;
            }
        }
        v
    }

    /// Device update and matrix load are separate solver phases, so one Newton
    /// iterate hands the same candidate to `update` twice. pnjlim limits
    /// against the previous iterate rather than being a function of the
    /// candidate alone, so a second pass would retarget the limiter at its own
    /// output and let the junction travel twice as far as one vbicload.c pass
    /// allows.
    #[test]
    fn repeating_a_promoted_candidate_does_not_advance_the_limiter_twice() {
        let mut bjt = diffamp_pnp(4.0);
        let n = 13;
        // PNP: a forward B-E junction is a base below the emitter, so this step
        // opens vbei by well over tVcrit and pnjlim has to replace it.
        let settled = promoted_bias(&bjt, n, 3.3, 3.3, 3.3);
        let candidate = promoted_bias(&bjt, n, 2.0, 2.0, 3.3);

        bjt.update(&settled);
        bjt.update(&settled);
        let previous = bjt.mna_internal_state();

        bjt.update(&candidate);
        let once = bjt.mna_internal_state();
        assert!(
            (once[IDX_VBI] - previous[IDX_VBI]).abs() > 1e-6,
            "the candidate must engage the limiter for this to test anything"
        );

        bjt.update(&candidate);
        assert_eq!(
            bjt.mna_internal_state(),
            once,
            "re-evaluating an identical candidate moved the limited bias"
        );

        // What the second pass would have produced: limiting the same raw
        // candidate again, now against its own output. A junction that travels
        // further on the second pass is the whole defect.
        let mut raw = [0.0; INTERNAL_DIM];
        raw.copy_from_slice(
            &bjt.mna_solution_bias(&candidate)[EXTERNAL_DIM..EXTERNAL_DIM + INTERNAL_DIM],
        );
        let mut once_internal = [0.0; INTERNAL_DIM];
        once_internal.copy_from_slice(&once[..INTERNAL_DIM]);
        let twice = bjt.limit_vbic_internal_state_to_previous(raw, once_internal);
        assert!(
            (twice[IDX_VBI] - twice[IDX_VEI] - (once[IDX_VBI] - once[IDX_VEI])).abs() > 1e-6,
            "a second limiting pass is supposed to move vbei further; the guard is what stops it"
        );

        // After the matrix load, identical voltages belong to a new Newton
        // iteration and must be allowed to advance the limited state.
        bjt.stamp_mna(&mut DenseStamper::new(n));
        bjt.update(&candidate);
        assert_eq!(&bjt.mna_internal_state()[..INTERNAL_DIM], &twice);
    }

    /// The junction limiter is defined on branch voltages; a promoted instance
    /// carries node voltages, so the projection back onto the nodes decides
    /// which parasitic drops the correction disturbs. It has to spend the
    /// correction where the network holds the node loosest — a substrate node
    /// behind a low `RS` takes amps for every volt spent there, and a
    /// self-heating instance reads that as watts on its thermal row.
    #[test]
    fn junction_limiting_moves_the_loosely_held_internal_nodes() {
        let mut params = std::collections::HashMap::new();
        for (key, value) in [
            ("LEVEL", 4.0),
            ("IS", 1e-16),
            ("IBEI", 1e-18),
            ("IBCI", 2e-17),
            ("ISP", 1e-15),
            ("RCX", 10.0),
            ("RCI", 1000.0),
            ("RBX", 500.0),
            ("RBI", 20.0),
            ("RE", 30.0),
            // The substrate node is nearly pinned to its terminal.
            ("RS", 1.0e-3),
            ("RBP", 1.0),
        ] {
            params.insert(key.to_string(), value);
        }
        let mut bjt = Bjt::new_npn("QW".to_string(), 1, 2, 3).with_params(&params);
        bjt.set_substrate_node(4);
        let mut next = 5;
        bjt.assign_vbic_internal_nodes(|_| {
            let node = next;
            next += 1;
            node
        });

        let raw = [0.0; INTERNAL_DIM];
        let limited = VbicNonlinearBranchVoltages {
            vbcp: -1.0,
            ..VbicNonlinearBranchVoltages::default()
        };
        let projected = bjt.project_vbic_limited_branches_onto_internal_state(raw, limited);

        // vbcp = vsi - vbp: the 1 mΩ substrate path must yield to the 11 Ω
        // parasitic base path, not share the correction with it.
        assert!(
            (projected[IDX_VSI] - projected[IDX_VBP] - (-1.0)).abs() < 1e-9,
            "the limited branch target must still be met exactly"
        );
        assert!(
            projected[IDX_VSI].abs() < 1e-3,
            "substrate node moved {:.4} V behind a 1 mΩ RS",
            projected[IDX_VSI]
        );
    }

    /// A promoted, self-heating LEVEL=4 instance on nodes 1-3, substrate 4,
    /// thermal 5, with every parasitic resistance present; returns it and the
    /// solution length.
    fn self_heating_vbic() -> (Bjt, usize) {
        let mut params = std::collections::HashMap::new();
        for (key, value) in [
            ("LEVEL", 4.0),
            ("IS", 1e-16),
            ("IBEI", 1e-18),
            ("RCX", 10.0),
            ("RCI", 1.0),
            ("RBX", 100.0),
            ("RBI", 20.0),
            ("RE", 15.0),
            ("RS", 1.0),
            ("RBP", 1.0),
            ("XRCX", 2.0),
            ("XRBX", 2.0),
            ("XRE", 2.0),
            ("XRS", 2.0),
            ("RTH", 150.0),
        ] {
            params.insert(key.to_string(), value);
        }
        let mut bjt = Bjt::new_npn("QT".to_string(), 1, 2, 3).with_params(&params);
        bjt.set_substrate_node(4);
        bjt.set_vbic_external_thermal_node(5);
        let mut next = 6;
        bjt.assign_vbic_internal_nodes(|_| {
            let node = next;
            next += 1;
            node
        });
        let n = next - 1;
        assert!(bjt.thermal_model_enabled());
        (bjt, n)
    }

    /// A self-heating instance whose parasitic resistors carry no current in
    /// the solution must not see them dissipate or couple temperature into
    /// their rows, however far the junction limiter moved its internal nodes:
    /// vbicload.c reads the resistor drops from the solution and limits only
    /// the junctions. Here every internal node sits on its terminal, so each
    /// drop is zero, and the limited junctions are realized by moving the
    /// nodes by volts.
    #[test]
    fn a_limited_self_heating_load_reads_the_solution_resistor_drops() {
        let (mut bjt, n) = self_heating_vbic();

        let mut bias = vec![0.0; n];
        for (node, value) in [
            (bjt.node_collector, 5.0),
            (bjt.node_cx, 5.0),
            (bjt.node_ci, 5.0),
            (bjt.node_bp, 5.0),
            (bjt.node_base, 2.5),
            (bjt.node_bx, 2.5),
            (bjt.node_bi, 2.5),
        ] {
            bias[node - 1] = value;
        }
        bjt.update(&vec![0.0; n]);
        bjt.stamp_mna(&mut DenseStamper::new(n));
        bjt.update(&bias);

        let state = bjt.mna_internal_state();
        assert!(
            (state[IDX_VCX] - 5.0).abs() > 0.1 || state[IDX_VSI].abs() > 0.1,
            "the limiter must move the internal nodes for this to test anything"
        );
        let eval = bjt.mna_eval.expect("a limited load caches its evaluation");
        for (name, branch) in [
            ("RCX", eval.ircx),
            ("RBX", eval.irbx),
            ("RE", eval.ire),
            ("RS", eval.irs),
        ] {
            assert!(
                branch.d_internal[IDX_VRTH].abs() < 1e-12,
                "{name} couples {:e} A/K into its rows at zero solution drop",
                branch.d_internal[IDX_VRTH]
            );
        }

        // Only the resistors' dissipation reads the external terminals.
        let mut stamper = DenseStamper::new(n);
        bjt.stamp_mna(&mut stamper);
        let thermal_row = &stamper.a[bjt.node_rth - 1];
        for terminal in [
            bjt.node_collector,
            bjt.node_base,
            bjt.node_emitter,
            bjt.node_substrate,
        ] {
            assert!(
                thermal_row[terminal - 1].abs() < 1e-12,
                "terminal {terminal} enters the heat row with {:e} W/V",
                thermal_row[terminal - 1]
            );
        }
    }

    /// vbicload.c's MODEINITJCT load (lines 250-258) opens a non-OFF instance
    /// at `Vbei = Vbex = tVcrit`, `Vbci = Vbcp = -tVcrit`, `Vbcx = Vbep = 0`
    /// and `Vrth = 0` whatever the solution vector holds, self-heating or not.
    #[test]
    fn a_self_heating_vbic_opens_on_the_modeinitjct_load() {
        let (mut bjt, n) = self_heating_vbic();
        let mut bias = vec![0.0; n];
        for (node, value) in [
            (bjt.node_collector, 5.0),
            (bjt.node_cx, 5.0),
            (bjt.node_ci, 5.0),
            (bjt.node_bp, 5.0),
            (bjt.node_base, 2.5),
            (bjt.node_bx, 2.5),
            (bjt.node_bi, 2.5),
            (bjt.node_rth, 40.0),
        ] {
            bias[node - 1] = value;
        }
        bjt.update(&bias);

        let (_vt, vcrit) = bjt.vbic_limiting_parameters(0.0);
        let mut state = [0.0; INTERNAL_DIM];
        state.copy_from_slice(&bjt.mna_internal_state()[..INTERNAL_DIM]);
        let opened = bjt.vbic_nonlinear_branch_voltages(state);
        for (name, actual, expected) in [
            ("Vbei", opened.vbei, vcrit),
            ("Vbex", opened.vbex, vcrit),
            ("Vbci", opened.vbci, -vcrit),
            ("Vbcx", opened.vbcx, 0.0),
            ("Vbep", opened.vbep, 0.0),
            ("Vbcp", opened.vbcp, -vcrit),
            ("Vrth", opened.vrth, 0.0),
        ] {
            assert!(
                (actual - expected).abs() < 1e-9,
                "{name} opened at {actual} instead of {expected}"
            );
        }
    }

    /// vbicload.c evaluates an OFF instance at zero on every junction on its
    /// first load, ignoring the solution vector. That is what lets the keyword
    /// steer a bistable operating point: a seed alone is a starting vector
    /// Newton leaves on its first step.
    #[test]
    fn an_off_promoted_instance_loads_at_zero_junction_bias() {
        let params = std::collections::HashMap::from([
            ("LEVEL".to_string(), 4.0),
            ("IS".to_string(), 1e-16),
            ("RCX".to_string(), 10.0),
            ("RCI".to_string(), 100.0),
            ("RBX".to_string(), 100.0),
            ("RBI".to_string(), 20.0),
            ("RE".to_string(), 5.0),
        ]);
        let mut off = Bjt::new_npn("QOFF".to_string(), 1, 2, 3)
            .with_params(&params)
            .with_instance_params(&[("OFF".to_string(), 1.0)]);
        let mut next = 4;
        off.assign_vbic_internal_nodes(|_| {
            let node = next;
            next += 1;
            node
        });
        assert!(off.is_initially_off());

        // A forward-biased base with the internal nodes already at their
        // terminals: the ordinary load would conduct here.
        let n = 12;
        let mut v = vec![0.0; n];
        v[off.node_collector - 1] = 2.0;
        v[off.node_base - 1] = 0.85;
        for node in [off.node_cx, off.node_ci, off.node_bp] {
            v[node - 1] = 2.0;
        }
        for node in [off.node_bx, off.node_bi] {
            v[node - 1] = 0.85;
        }

        off.update(&v);
        let state = off.mna_internal_state();
        assert!(
            (state[IDX_VBI] - state[IDX_VEI]).abs() < 1e-9,
            "OFF instance loaded with vbei = {:.4} V",
            state[IDX_VBI] - state[IDX_VEI]
        );
        assert!(
            (state[IDX_VBI] - state[IDX_VCI]).abs() < 1e-9,
            "OFF instance loaded with vbci = {:.4} V",
            state[IDX_VBI] - state[IDX_VCI]
        );

        // Only the first load: the next iterate limits against it as usual, so
        // the keyword steers the solve rather than pinning the instance off.
        let mut next_iterate = v.clone();
        next_iterate[off.node_base - 1] = 0.84;
        off.update(&next_iterate);
        let second = off.mna_internal_state();
        assert!(
            (second[IDX_VBI] - second[IDX_VEI]).abs() > 1e-6,
            "the OFF load must apply once, not pin the instance off for the solve"
        );
    }

    /// Under `DEVICE VOLTLIM=0` a VBIC loads each iterate as it stands: its
    /// first load takes no MODEINITJCT seed and later ones are not limited,
    /// as Xyce's voltageLimiterFlag gates both (N_DEV_ADMSvbic13.C:3575,
    /// 3588). An OFF instance still opens at zero junction bias, once.
    #[test]
    fn a_vbic_without_voltage_limiting_loads_each_iterate_as_it_stands() {
        for off in [false, true] {
            let (mut bjt, n) = self_heating_vbic();
            bjt.set_voltage_limiting_enabled(false);
            bjt.initial_off = off;
            let collector = [bjt.node_collector, bjt.node_cx, bjt.node_ci, bjt.node_bp];
            let base = [bjt.node_base, bjt.node_bx, bjt.node_bi];
            let rth = bjt.node_rth;
            let iterate = |vb: Value| {
                let mut bias = vec![0.0; n];
                for node in collector {
                    bias[node - 1] = 5.0;
                }
                for node in base {
                    bias[node - 1] = vb;
                }
                bias[rth - 1] = 40.0;
                bias
            };

            let first = iterate(2.5);
            bjt.update(&first);
            let state = bjt.mna_internal_state();
            if off {
                assert!(
                    (state[IDX_VBI] - state[IDX_VEI]).abs() < 1e-9
                        && (state[IDX_VBI] - state[IDX_VCI]).abs() < 1e-9,
                    "the OFF instance opened at {state:?}"
                );
            } else {
                assert_eq!(
                    &state[..INTERNAL_DIM],
                    &bjt.mna_internal_state_at_solution(&first)[..INTERNAL_DIM],
                    "the first load was seeded"
                );
            }

            // A volt further forward than the last load: the limited load
            // cuts this step, the unlimited one takes it whole.
            let second = iterate(3.5);
            bjt.update(&second);
            assert_eq!(
                &bjt.mna_internal_state()[..INTERNAL_DIM],
                &bjt.mna_internal_state_at_solution(&second)[..INTERNAL_DIM],
                "OFF = {off}: the second load was limited"
            );
        }
    }

    /// Promoted Gummel-Poon cards: the fourbitadder gate transistor (base
    /// resistance only, so collector, emitter and substrate collapse onto
    /// their terminals) and one with every series resistance.
    const GUMMEL_POON_CARDS: [&[(&str, Value)]; 2] = [
        &[("BF", 75.0), ("RB", 100.0), ("CJE", 1e-12), ("CJC", 3e-12)],
        &[
            ("BF", 75.0),
            ("RB", 100.0),
            ("RC", 2.0),
            ("RE", 1.0),
            ("CJE", 1e-12),
            ("CJC", 3e-12),
            ("TF", 1e-10),
            ("VAF", 50.0),
        ],
    ];

    /// A forward-active collector, base and emitter bias.
    const FORWARD_ACTIVE: [Value; 3] = [1.4, 0.76, 0.02];

    /// A promoted instance numbered as the builder numbers a transient's,
    /// loaded at a collector, base and emitter bias until its loads stop
    /// limiting.
    fn settled_gummel_poon(
        card: &[(&str, Value)],
        [collector, base, emitter]: [Value; 3],
    ) -> (Bjt, Vec<Value>) {
        let params = card
            .iter()
            .map(|&(name, value)| (name.to_owned(), value))
            .collect();
        let mut bjt = Bjt::new_npn("qsettled".into(), 1, 2, 3).with_params(&params);
        let mut nodes = 4;
        bjt.assign_mna_internal_nodes(|_| {
            nodes += 1;
            nodes
        });
        if bjt.needs_mna_rbi_branch() {
            bjt.assign_mna_rbi_branch(1);
            bjt.resolve_mna_rbi_branch(nodes);
            nodes += 1;
        }
        assert!(bjt.mna_promoted());
        let mut bias = promoted_bias(&bjt, nodes, collector, base, emitter);
        if bjt.needs_mna_rbi_branch() {
            bias[nodes - 1] = 2e-5;
        }
        for _ in 0..40 {
            bjt.update(&bias);
            bjt.stamp_mna(&mut DenseStamper::new(nodes));
        }
        bjt.update(&bias);
        (bjt, bias)
    }

    /// A load that junction limiting leaves alone evaluates at the solver's
    /// own bias, as ngspice's does. At these biases, projecting the unchanged
    /// junction voltages back onto the nodes used to leave the emitter or
    /// substrate node a few ulps off (0.02 V came back as
    /// 0.020000000000000014).
    #[test]
    fn an_unlimited_load_evaluates_at_the_solution_itself() {
        for card in GUMMEL_POON_CARDS {
            for bias in [FORWARD_ACTIVE, [5.0, 0.7, 0.0], [3.1, 0.81, 0.05]] {
                let (bjt, solution) = settled_gummel_poon(card, bias);
                assert!(!bjt.legacy_junction_limited, "{bias:?}");
                let rbi = Bjt::node_voltage(&solution, bjt.mna_rbi_matrix_node);
                assert!(
                    bjt.mna_eval_is_unlimited_at(&bjt.mna_solution_bias(&solution), rbi),
                    "{bias:?}"
                );
            }
        }
    }

    /// A static probe of the bias the last load evaluated without limiting
    /// keeps that evaluation, and leaves the instance exactly as a probe that
    /// evaluates again does. Another bias, or a GMIN change in between,
    /// evaluates again.
    #[test]
    fn a_static_probe_of_an_unlimited_load_keeps_its_evaluation() {
        for card in GUMMEL_POON_CARDS {
            let (mut bjt, bias) = settled_gummel_poon(card, FORWARD_ACTIVE);
            let rbi = Bjt::node_voltage(&bias, bjt.mna_rbi_matrix_node);
            let candidate = bjt.mna_solution_bias(&bias);
            assert!(
                bjt.mna_eval_is_unlimited_at(&candidate, rbi),
                "the load must settle unlimited for this to test anything"
            );
            let mut evaluated = bjt.clone();
            evaluated.mna_eval_at_state = false;
            bjt.update_static_linearization(&bias);
            evaluated.update_static_linearization(&bias);
            assert_eq!(format!("{bjt:?}"), format!("{evaluated:?}"));

            let mut moved = bias.clone();
            moved[bjt.node_base - 1] += 1e-3;
            assert!(!bjt.mna_eval_is_unlimited_at(&bjt.mna_solution_bias(&moved), rbi));
            bjt.set_junction_gmin(1e-9);
            assert!(!bjt.mna_eval_is_unlimited_at(&candidate, rbi));
        }
    }

    /// An accepted step reads the charges and terminal currents at its
    /// solution. When the last evaluation was of exactly that bias the reads
    /// return it, bit for bit what evaluating the solution again gives.
    #[test]
    fn accepted_point_reads_of_the_evaluated_bias_return_that_evaluation() {
        for card in GUMMEL_POON_CARDS {
            let (bjt, bias) = settled_gummel_poon(card, FORWARD_ACTIVE);
            assert_accepted_reads_reuse_the_evaluation(&bjt, &bias);
        }
        // A VBIC load expands about the solution's resistor drops, so only a
        // static probe leaves an evaluation of the bias itself.
        let mut bjt = diffamp_pnp(4.0);
        let bias = promoted_bias(&bjt, 13, 2.0, 2.6, 3.3);
        bjt.update(&bias);
        assert!(!bjt.mna_charge_cache_is_at(
            bjt.external_terminal_voltages(&bias),
            bjt.mna_internal_state_at_solution(&bias)
        ));
        bjt.update_static_linearization(&bias);
        assert_accepted_reads_reuse_the_evaluation(&bjt, &bias);
    }

    fn assert_accepted_reads_reuse_the_evaluation(bjt: &Bjt, bias: &[Value]) {
        let external = bjt.external_terminal_voltages(bias);
        let internal = bjt.mna_internal_state_at_solution(bias);
        assert!(bjt.mna_charge_cache_is_at(external, internal));
        let rbi = Bjt::node_voltage(bias, bjt.mna_rbi_matrix_node);
        assert!(bjt.mna_eval_at(&bjt.mna_solution_bias(bias), rbi).is_some());
        let mut evaluated = bjt.clone();
        evaluated.mna_eval_at_state = false;
        assert_eq!(
            format!("{:?}", bjt.mna_charge_state_at_solution(bias)),
            format!("{:?}", evaluated.mna_charge_state_at_solution(bias))
        );
        assert_eq!(
            bjt.mna_terminal_currents_at_solution(bias)
                .map(Value::to_bits),
            evaluated
                .mna_terminal_currents_at_solution(bias)
                .map(Value::to_bits)
        );
    }

    /// Every addition a stamp makes, in order.
    #[derive(Default, Debug, PartialEq)]
    struct Additions {
        matrix: Vec<(NodeId, NodeId, u64)>,
        rhs: Vec<(NodeId, u64)>,
    }

    impl MatrixStamper for Additions {
        fn stamp(&mut self, row: NodeId, col: NodeId, value: Value) {
            self.matrix.push((row, col, value.to_bits()));
        }

        fn stamp_rhs(&mut self, row: NodeId, value: Value) {
            self.rhs.push((row, value.to_bits()));
        }
    }

    /// One pass over a promoted load makes the Newton stamp's additions and
    /// the correction stamp's right-hand-side additions, each in its own
    /// order: limited and unlimited Gummel-Poon loads, tied terminals, and
    /// VBIC loads with excess phase and self-heating.
    #[test]
    fn one_pass_makes_the_newton_and_correction_stamps_additions() {
        let mut loads = Vec::new();
        for card in GUMMEL_POON_CARDS {
            let (bjt, bias) = settled_gummel_poon(card, FORWARD_ACTIVE);
            let mut limited = bjt.clone();
            let mut raw = bias.clone();
            raw[bjt.node_bi - 1] += 0.5;
            limited.update(&raw);
            assert!(limited.legacy_junction_limited);
            loads.push((bjt, bias));
            loads.push((limited, raw));
        }
        let params = GUMMEL_POON_CARDS[1]
            .iter()
            .map(|&(name, value)| (name.to_owned(), value))
            .collect();
        let mut tied = Bjt::new_npn("qtied".into(), 1, 1, 2).with_params(&params);
        let mut nodes = 2;
        tied.assign_mna_internal_nodes(|_| {
            nodes += 1;
            nodes
        });
        if tied.needs_mna_rbi_branch() {
            tied.assign_mna_rbi_branch(1);
            tied.resolve_mna_rbi_branch(nodes);
            nodes += 1;
        }
        assert!(tied.promoted_tied_terminal_reference().is_some());
        let bias = promoted_bias(&tied, nodes, 0.8, 0.8, 0.0);
        tied.update(&bias);
        loads.push((tied, bias));
        for level in [4.0, 11.0, 12.0] {
            let mut bjt = diffamp_pnp(level);
            let bias = promoted_bias(&bjt, 13, 2.0, 2.6, 3.3);
            bjt.update(&bias);
            loads.push((bjt, bias));
        }
        let (mut bjt, n) = self_heating_vbic();
        let bias = promoted_bias(&bjt, n, 2.0, 0.8, 0.0);
        bjt.update(&bias);
        loads.push((bjt, bias));

        for (bjt, anchor) in &loads {
            let mut newton = Additions::default();
            bjt.stamp_mna(&mut newton);
            let mut correction = Additions::default();
            bjt.stamp_mna_correction(&mut correction, anchor);
            let mut one_newton = Additions::default();
            let mut one_correction = Additions::default();
            bjt.stamp_mna_with_correction(&mut one_newton, &mut one_correction, anchor);
            assert_ne!(newton.rhs, correction.rhs, "{}", bjt.name);
            assert_eq!(one_newton, newton, "{}", bjt.name);
            assert_eq!(one_correction.rhs, correction.rhs, "{}", bjt.name);
            assert!(one_correction.matrix.is_empty(), "{}", bjt.name);
        }
    }

    /// A Gummel-Poon card with every stored charge: an external base-collector
    /// depletion charge (XCJC < 1), a substrate junction and reverse transit
    /// time, but no forward transit time.
    const SPLIT_JUNCTION_GUMMEL_POON: &[(&str, Value)] = &[
        ("BF", 75.0),
        ("RB", 100.0),
        ("CJE", 1e-12),
        ("CJC", 3e-12),
        ("XCJC", 0.5),
        ("CJS", 2e-12),
        ("TR", 1e-9),
        ("VAF", 50.0),
    ];

    /// The charge branches computed from only the charges the device has a
    /// branch for (no external base-collector or substrate depletion charge
    /// without its branch, no forward transport without TF) are bit for bit
    /// the branches computed from every charge.
    #[test]
    fn gummel_poon_charge_branches_compute_only_the_charges_they_read() {
        let mut loads = gummel_poon_loads_in_every_region();
        for region in GUMMEL_POON_REGIONS {
            loads.push(settled_gummel_poon(SPLIT_JUNCTION_GUMMEL_POON, region));
        }
        for (bjt, bias) in loads {
            let external = bjt.mna_external_state();
            let internal = bjt.mna_internal_state();
            let all = bjt.legacy_dynamic_charge_branches_of(
                external,
                internal,
                false,
                None,
                LegacyChargeTerms::ALL,
            );
            assert_eq!(
                format!(
                    "{:?}",
                    bjt.legacy_dynamic_charge_branches(external, internal)
                ),
                format!("{all:?}"),
                "{bias:?}"
            );
            assert_eq!(
                format!("{:?}", bjt.mna_charge_cache.get()),
                format!("{all:?}"),
                "{bias:?}"
            );
        }
    }

    /// The promoted stamp as it was before it stamped each internal row as it
    /// summed it and took both lanes' sources in one pass: the reference the
    /// stamp is held to, addition for addition.
    fn reference_stamp<T: MnaLoadTargets>(bjt: &Bjt, targets: &mut T) {
        let Some(eval) = bjt.mna_eval else {
            return;
        };
        bjt.mna_limited_from.set(None);
        let state = IntrinsicTerminalState {
            vcx: bjt.vcx,
            vci: bjt.vci,
            vbx: bjt.vbx,
            vbi: bjt.vbi,
            vei: bjt.vei,
            vbp: bjt.vbp,
            vsi: bjt.vsi,
            vrth: bjt.vrth,
        };
        let [vc, vb, ve, vs] = bjt.mna_external_state();
        let internal_nodes: [NodeId; INTERNAL_DIM] = [
            bjt.node_cx,
            bjt.node_ci,
            bjt.node_bx,
            bjt.node_bi,
            bjt.node_ei,
            bjt.node_bp,
            bjt.node_si,
            bjt.node_rth,
        ];
        let external_nodes = bjt.external_terminal_nodes();
        let mut internal = [bjt.mna_internal_state(); 2];
        let mut external = [[vc, vb, ve, vs]; 2];
        for lane in 0..T::LANES {
            if let Some(anchor) = targets.anchor(lane) {
                for (index, voltage) in internal[lane].iter_mut().enumerate() {
                    *voltage -= Bjt::node_voltage(anchor, bjt.mna_internal_node(index));
                }
                for (voltage, node) in external[lane].iter_mut().zip(external_nodes) {
                    *voltage -= Bjt::node_voltage(anchor, node);
                }
            }
        }
        let static_internal = |lane: usize| -> &[Value; INTERNAL_DIM] {
            internal[lane][..INTERNAL_DIM].try_into().unwrap()
        };
        let (g_ii, g_ie, z_i) = bjt.internal_kcl_linearization_from_eval_with_source(
            state,
            eval,
            [vc, vb, ve, vs],
            bjt.mna_dissipation_drops,
            |row| targets.lanes(|lane| row.source(static_internal(lane), &external[lane])),
        );
        for row in 0..INTERNAL_DIM {
            if !bjt.mna_internal_row_active(row) {
                continue;
            }
            let sign = Bjt::vbic_residual_row_sign(row);
            let row_node = internal_nodes[row];
            for col in 0..INTERNAL_DIM {
                if g_ii[row][col] != 0.0 {
                    targets.stamp(row_node, internal_nodes[col], sign * g_ii[row][col]);
                }
            }
            for col in 0..EXTERNAL_DIM {
                if g_ie[row][col] != 0.0 {
                    targets.stamp(row_node, external_nodes[col], sign * g_ie[row][col]);
                }
            }
            targets.stamp_rhs_lanes(row_node, z_i[row].map(|source| sign * source));
        }
        let terminal_currents = bjt.external_terminal_branches(eval);
        for row in 0..EXTERNAL_DIM {
            let row_node = external_nodes[row];
            let branch = terminal_currents[row];
            let source =
                targets.lanes(|lane| branch.source(static_internal(lane), &external[lane]));
            for (&node, &derivative) in internal_nodes.iter().zip(&branch.d_internal) {
                if derivative != 0.0 {
                    targets.stamp(row_node, node, derivative);
                }
            }
            for (&node, &derivative) in external_nodes.iter().zip(&branch.d_external) {
                if derivative != 0.0 {
                    targets.stamp(row_node, node, derivative);
                }
            }
            targets.stamp_rhs_lanes(row_node, source);
        }
        if bjt.mna_rbi_matrix_node != 0 {
            bjt.stamp_mna_rbi_current(targets, eval.linearized, &internal, &external, false);
        }
        if bjt.td > 0.0 {
            for branch in &bjt.mna_delay_branches {
                bjt.stamp_vbic_residual_branch(targets, branch, &internal, &external);
            }
            if bjt.thermal_model_enabled() {
                bjt.stamp_vbic_residual_branch(
                    targets,
                    &bjt.mna_delay_thermal,
                    &internal,
                    &external,
                );
            }
        }
    }

    /// [`reference_stamp`] of the Newton load and its correction at `anchor`,
    /// through the tied-terminal projection as the device applies it.
    fn reference_stamp_with_correction(
        bjt: &Bjt,
        newton: &mut Additions,
        correction: &mut Additions,
        anchor: &[Value],
    ) {
        match bjt.promoted_tied_terminal_reference() {
            Some(reference) => reference_stamp(
                bjt,
                &mut NewtonAndCorrection {
                    newton: &mut bjt.tied_terminal_projection(newton, reference),
                    correction: &mut bjt.tied_terminal_projection(correction, reference),
                    anchor,
                },
            ),
            None => reference_stamp(
                bjt,
                &mut NewtonAndCorrection {
                    newton,
                    correction,
                    anchor,
                },
            ),
        }
    }

    /// Promoted loads with each feature the stamp handles: both Gummel-Poon
    /// cards in every region, settled and junction-limited; Gummel-Poon cards
    /// with quasi-saturation, excess phase and an IRB branch in every region;
    /// a pending OFF load; tied terminals; and VBIC loads with a substrate
    /// resistor, excess phase and self-heating.
    fn promoted_loads_with_every_feature() -> Vec<(Bjt, Vec<Value>)> {
        let mut loads = gummel_poon_loads_in_every_region();
        let quasi_saturation: &[(&str, Value)] = &[
            ("BF", 75.0),
            ("RB", 100.0),
            ("RC", 10.0),
            ("RCO", 50.0),
            ("QCO", 1e-12),
            ("VO", 2.0),
            ("GAMMA", 1e-11),
            ("CJC", 3e-12),
        ];
        let excess_phase: &[(&str, Value)] = &[
            ("BF", 75.0),
            ("RB", 100.0),
            ("RE", 1.0),
            ("TF", 1e-10),
            ("PTF", 30.0),
            ("CJE", 1e-12),
        ];
        let irb: &[(&str, Value)] = &[
            ("BF", 75.0),
            ("RB", 100.0),
            ("RBM", 10.0),
            ("IRB", 1e-4),
            ("CJE", 1e-12),
        ];
        for card in [quasi_saturation, excess_phase, irb] {
            for region in GUMMEL_POON_REGIONS {
                loads.push(settled_gummel_poon(card, region));
            }
        }
        assert!(loads.last().unwrap().0.mna_rbi_matrix_node != 0);

        let params = GUMMEL_POON_CARDS[1]
            .iter()
            .map(|&(name, value)| (name.to_owned(), value))
            .collect();
        let mut off = Bjt::new_npn("qoff".into(), 1, 2, 3)
            .with_params(&params)
            .with_instance_params(&[("OFF".to_string(), 1.0)]);
        let mut nodes = 4;
        off.assign_mna_internal_nodes(|_| {
            nodes += 1;
            nodes
        });
        let bias = promoted_bias(&off, nodes, 1.4, 0.76, 0.02);
        off.update(&bias);
        loads.push((off, bias));

        let mut tied = Bjt::new_npn("qtied".into(), 1, 1, 2).with_params(&params);
        let mut nodes = 2;
        tied.assign_mna_internal_nodes(|_| {
            nodes += 1;
            nodes
        });
        assert!(tied.promoted_tied_terminal_reference().is_some());
        let bias = promoted_bias(&tied, nodes, 0.8, 0.8, 0.0);
        tied.update(&bias);
        loads.push((tied, bias));

        for level in [4.0, 11.0, 12.0] {
            let mut bjt = diffamp_pnp(level);
            // LEVEL=11 is the three-terminal VBIC, with no substrate resistor.
            assert!(bjt.td > 0.0 && (level == 11.0 || bjt.has_substrate_resistance()));
            let bias = promoted_bias(&bjt, 13, 2.0, 2.6, 3.3);
            bjt.update(&bias);
            loads.push((bjt, bias));
        }
        let (mut bjt, n) = self_heating_vbic();
        let bias = promoted_bias(&bjt, n, 2.0, 0.8, 0.0);
        bjt.update(&bias);
        loads.push((bjt, bias));
        loads
    }

    /// The stamp makes the reference stamp's additions, value for value and
    /// in order, for the Newton load, the correction, and both at once.
    #[test]
    fn the_promoted_stamp_makes_the_reference_stamps_additions() {
        for (bjt, anchor) in promoted_loads_with_every_feature() {
            let mut newton = Additions::default();
            bjt.stamp_mna(&mut newton);
            let mut expected = Additions::default();
            reference_stamp(
                &bjt,
                &mut SingleLoad {
                    stamper: &mut expected,
                    anchor: None,
                },
            );
            if bjt.promoted_tied_terminal_reference().is_none() {
                assert_eq!(newton, expected, "{} {anchor:?}", bjt.name);
            }
            let [mut one_newton, mut one_correction] = [(); 2].map(|_| Additions::default());
            bjt.stamp_mna_with_correction(&mut one_newton, &mut one_correction, &anchor);
            let [mut ref_newton, mut ref_correction] = [(); 2].map(|_| Additions::default());
            reference_stamp_with_correction(&bjt, &mut ref_newton, &mut ref_correction, &anchor);
            assert_eq!(one_newton, ref_newton, "{} {anchor:?}", bjt.name);
            assert_eq!(one_correction, ref_correction, "{} {anchor:?}", bjt.name);
            assert_eq!(one_newton, newton, "{} {anchor:?}", bjt.name);
        }
    }

    /// Collector, base and emitter biases the Gummel-Poon fast paths are held
    /// to: forward active, saturated, cut off and reverse active.
    const GUMMEL_POON_REGIONS: [[Value; 3]; 4] = [
        FORWARD_ACTIVE,
        [0.1, 0.8, 0.0],
        [2.0, 0.0, 0.5],
        [0.0, 0.7, 2.0],
    ];

    /// Settled and junction-limited promoted Gummel-Poon loads of both
    /// cards in every region, with the bias each was loaded at.
    fn gummel_poon_loads_in_every_region() -> Vec<(Bjt, Vec<Value>)> {
        let mut loads = Vec::new();
        for card in GUMMEL_POON_CARDS {
            for region in GUMMEL_POON_REGIONS {
                let (bjt, bias) = settled_gummel_poon(card, region);
                let mut limited = bjt.clone();
                let mut raw = bias.clone();
                raw[bjt.node_bi - 1] += 0.5;
                limited.update(&raw);
                loads.push((bjt, bias));
                loads.push((limited, raw));
            }
        }
        loads
    }

    /// The update keeps the terminal rows it summed, and the stamp makes
    /// exactly the additions it makes when it sums them itself. The charges
    /// the update computed from its evaluation's transport are bit for bit
    /// the ones computed from their own.
    #[test]
    fn a_gummel_poon_update_shares_its_transport_and_terminal_rows() {
        for (bjt, anchor) in gummel_poon_loads_in_every_region() {
            assert!(bjt.mna_terminal_rows.0.is_some());
            let mut summed = bjt.clone();
            summed.mna_terminal_rows = MnaTerminalRows(None);
            let mut kept = Additions::default();
            bjt.stamp_mna(&mut kept);
            let mut own = Additions::default();
            summed.stamp_mna(&mut own);
            assert_eq!(kept, own, "{anchor:?}");
            let [mut kept_newton, mut kept_correction] = [(); 2].map(|_| Additions::default());
            bjt.stamp_mna_with_correction(&mut kept_newton, &mut kept_correction, &anchor);
            let [mut own_newton, mut own_correction] = [(); 2].map(|_| Additions::default());
            summed.stamp_mna_with_correction(&mut own_newton, &mut own_correction, &anchor);
            assert_eq!(kept_newton, own_newton, "{anchor:?}");
            assert_eq!(kept_correction, own_correction, "{anchor:?}");
            assert_eq!(
                format!("{:?}", bjt.mna_charge_cache.get()),
                format!(
                    "{:?}",
                    bjt.legacy_dynamic_charge_branches(
                        bjt.mna_external_state(),
                        bjt.mna_internal_state()
                    )
                ),
                "{anchor:?}"
            );
        }
    }
}
