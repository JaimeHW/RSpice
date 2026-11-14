//! A BJT's rollback point: the state a load, stamp or probe writes, without
//! the model, temperature scaling, topology and matrix linkage that stay put.

use super::{
    BJT_DYNAMIC_CHARGE_COUNT, BJT_INTERNAL_STATE_DIM, Bjt, BjtChargeBranch, BjtChargeSnapshot,
    BjtCurrentBranch, BjtLinearization, BjtReducedLinearization, BjtThermalVariantKey,
    EXTERNAL_DIM, EvaluatedBjtState, VBIC_DELAY_BRANCH_COUNT, VBIC_PARASITIC_DROP_DIM,
};
use crate::Value;

/// Mutable Newton, limiter and cache state for a BJT rollback point.
///
/// A transient step takes one per device at every attempt. Cloning the device
/// instead copies its model card and matrix linkage too, twice over, and
/// reallocates its name and junction parameters; refreshing this image in
/// place copies only what an attempt can change. Temperature variants are
/// derived from the junction GMIN in force, so they travel with it.
///
/// A promoted device loads, stamps, probes and integrates through its MNA
/// evaluation and never reads the reduced linearization and charge snapshot
/// caches of the private solve, so its image leaves those three out (their
/// validity flags still travel).
#[derive(Debug, Clone, Default)]
#[doc(hidden)]
pub struct BjtNonlinearState {
    image: BjtEvaluationImage,
    thermal_variants: Vec<(BjtThermalVariantKey, Box<Bjt>)>,
}

#[derive(Debug, Clone, Copy, Default)]
struct BjtEvaluationImage {
    junction_gmin: Value,
    vbe: Value,
    vbc: Value,
    vcx: Value,
    vbi: Value,
    vci: Value,
    vbx: Value,
    vei: Value,
    vbp: Value,
    vsi: Value,
    vrth: Value,
    vxf1: Value,
    vxf2: Value,
    vc_ext: Value,
    vb_ext: Value,
    ve_ext: Value,
    vs_ext: Value,
    eval_anchor: [Value; EXTERNAL_DIM],
    legacy_junction_limited: bool,
    legacy_update_open: bool,
    legacy_initial_junction_pending: bool,
    ic: Value,
    ib: Value,
    ie: Value,
    isub: Value,
    intrinsic_linearization: BjtLinearization,
    vbe_prev: Value,
    vbc_prev: Value,
    vcx_prev: Value,
    vbi_prev: Value,
    vci_prev: Value,
    vbx_prev: Value,
    vei_prev: Value,
    vbp_prev: Value,
    vsi_prev: Value,
    vrth_prev: Value,
    ic_prev: Value,
    ib_prev: Value,
    ie_prev: Value,
    isub_prev: Value,
    intrinsic_linearization_prev: BjtLinearization,
    reduced_linearization_cache: BjtReducedLinearization,
    reduced_linearization_cache_valid: bool,
    previous_reduced_linearization: BjtReducedLinearization,
    previous_reduced_linearization_valid: bool,
    charge_snapshot_cache: BjtChargeSnapshot,
    charge_snapshot_cache_valid: bool,
    mna_eval: Option<EvaluatedBjtState>,
    mna_rbi_current: Value,
    mna_limited_from: Option<[Value; EXTERNAL_DIM + BJT_INTERNAL_STATE_DIM]>,
    mna_eval_at_state: bool,
    vbic_startup_load_pending: bool,
    mna_dissipation_drops: Option<[Value; VBIC_PARASITIC_DROP_DIM]>,
    mna_delay_branches: [BjtCurrentBranch; VBIC_DELAY_BRANCH_COUNT],
    mna_delay_thermal: BjtCurrentBranch,
    mna_charge_cache: [BjtChargeBranch; BJT_DYNAMIC_CHARGE_COUNT],
    mna_charge_cache_valid: bool,
}

impl Bjt {
    /// Write this device's rollback point into `state`, reusing its storage.
    #[doc(hidden)]
    pub fn nonlinear_state_snapshot_into(&self, state: &mut BjtNonlinearState) {
        let image = &mut state.image;
        image.junction_gmin = self.junction_gmin;
        image.vbe = self.vbe;
        image.vbc = self.vbc;
        image.vcx = self.vcx;
        image.vbi = self.vbi;
        image.vci = self.vci;
        image.vbx = self.vbx;
        image.vei = self.vei;
        image.vbp = self.vbp;
        image.vsi = self.vsi;
        image.vrth = self.vrth;
        image.vxf1 = self.vxf1;
        image.vxf2 = self.vxf2;
        image.vc_ext = self.vc_ext;
        image.vb_ext = self.vb_ext;
        image.ve_ext = self.ve_ext;
        image.vs_ext = self.vs_ext;
        image.eval_anchor = self.eval_anchor;
        image.legacy_junction_limited = self.legacy_junction_limited;
        image.legacy_update_open = self.legacy_update_open.get();
        image.legacy_initial_junction_pending = self.legacy_initial_junction_pending;
        image.ic = self.ic;
        image.ib = self.ib;
        image.ie = self.ie;
        image.isub = self.isub;
        image.intrinsic_linearization = self.intrinsic_linearization;
        image.vbe_prev = self.vbe_prev;
        image.vbc_prev = self.vbc_prev;
        image.vcx_prev = self.vcx_prev;
        image.vbi_prev = self.vbi_prev;
        image.vci_prev = self.vci_prev;
        image.vbx_prev = self.vbx_prev;
        image.vei_prev = self.vei_prev;
        image.vbp_prev = self.vbp_prev;
        image.vsi_prev = self.vsi_prev;
        image.vrth_prev = self.vrth_prev;
        image.ic_prev = self.ic_prev;
        image.ib_prev = self.ib_prev;
        image.ie_prev = self.ie_prev;
        image.isub_prev = self.isub_prev;
        image.intrinsic_linearization_prev = self.intrinsic_linearization_prev;
        image.reduced_linearization_cache_valid = self.reduced_linearization_cache_valid.get();
        image.previous_reduced_linearization_valid = self.previous_reduced_linearization_valid;
        image.charge_snapshot_cache_valid = self.charge_snapshot_cache_valid.get();
        if !self.mna_promoted {
            image.reduced_linearization_cache = self.reduced_linearization_cache.get();
            image.previous_reduced_linearization = self.previous_reduced_linearization;
            image.charge_snapshot_cache = self.charge_snapshot_cache.get();
        }
        image.mna_eval = self.mna_eval;
        image.mna_rbi_current = self.mna_rbi_current;
        image.mna_limited_from = self.mna_limited_from.get();
        image.mna_eval_at_state = self.mna_eval_at_state;
        image.vbic_startup_load_pending = self.vbic_startup_load_pending;
        image.mna_dissipation_drops = self.mna_dissipation_drops;
        image.mna_delay_branches = self.mna_delay_branches;
        image.mna_delay_thermal = self.mna_delay_thermal;
        image.mna_charge_cache = self.mna_charge_cache;
        image.mna_charge_cache_valid = self.mna_charge_cache_valid.get();
        state
            .thermal_variants
            .clone_from(&self.thermal_variant_cache.borrow());
    }

    /// Return to a rollback point this device wrote.
    #[doc(hidden)]
    pub fn restore_nonlinear_state(&mut self, state: BjtNonlinearState) {
        let BjtNonlinearState {
            image,
            thermal_variants,
        } = state;
        self.junction_gmin = image.junction_gmin;
        self.vbe = image.vbe;
        self.vbc = image.vbc;
        self.vcx = image.vcx;
        self.vbi = image.vbi;
        self.vci = image.vci;
        self.vbx = image.vbx;
        self.vei = image.vei;
        self.vbp = image.vbp;
        self.vsi = image.vsi;
        self.vrth = image.vrth;
        self.vxf1 = image.vxf1;
        self.vxf2 = image.vxf2;
        self.vc_ext = image.vc_ext;
        self.vb_ext = image.vb_ext;
        self.ve_ext = image.ve_ext;
        self.vs_ext = image.vs_ext;
        self.eval_anchor = image.eval_anchor;
        self.legacy_junction_limited = image.legacy_junction_limited;
        self.legacy_update_open.set(image.legacy_update_open);
        self.legacy_initial_junction_pending = image.legacy_initial_junction_pending;
        self.ic = image.ic;
        self.ib = image.ib;
        self.ie = image.ie;
        self.isub = image.isub;
        self.intrinsic_linearization = image.intrinsic_linearization;
        self.vbe_prev = image.vbe_prev;
        self.vbc_prev = image.vbc_prev;
        self.vcx_prev = image.vcx_prev;
        self.vbi_prev = image.vbi_prev;
        self.vci_prev = image.vci_prev;
        self.vbx_prev = image.vbx_prev;
        self.vei_prev = image.vei_prev;
        self.vbp_prev = image.vbp_prev;
        self.vsi_prev = image.vsi_prev;
        self.vrth_prev = image.vrth_prev;
        self.ic_prev = image.ic_prev;
        self.ib_prev = image.ib_prev;
        self.ie_prev = image.ie_prev;
        self.isub_prev = image.isub_prev;
        self.intrinsic_linearization_prev = image.intrinsic_linearization_prev;
        self.reduced_linearization_cache_valid
            .set(image.reduced_linearization_cache_valid);
        self.previous_reduced_linearization_valid = image.previous_reduced_linearization_valid;
        self.charge_snapshot_cache_valid
            .set(image.charge_snapshot_cache_valid);
        if !self.mna_promoted {
            self.reduced_linearization_cache
                .set(image.reduced_linearization_cache);
            self.previous_reduced_linearization = image.previous_reduced_linearization;
            self.charge_snapshot_cache.set(image.charge_snapshot_cache);
        }
        self.mna_eval = image.mna_eval;
        // The image leaves out the terminal rows the update summed from this
        // evaluation; the next stamp sums them from the restored one.
        self.mna_terminal_rows = super::MnaTerminalRows(None);
        self.mna_rbi_current = image.mna_rbi_current;
        self.mna_limited_from.set(image.mna_limited_from);
        self.mna_eval_at_state = image.mna_eval_at_state;
        self.vbic_startup_load_pending = image.vbic_startup_load_pending;
        self.mna_dissipation_drops = image.mna_dissipation_drops;
        self.mna_delay_branches = image.mna_delay_branches;
        self.mna_delay_thermal = image.mna_delay_thermal;
        self.mna_charge_cache = image.mna_charge_cache;
        self.mna_charge_cache_valid
            .set(image.mna_charge_cache_valid);
        *self.thermal_variant_cache.get_mut() = thermal_variants;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NodeId;
    use crate::device::traits::{MatrixStamper, NonlinearConvergenceCriteria, NonlinearDevice};

    struct Discard;

    impl MatrixStamper for Discard {
        fn stamp(&mut self, _row: NodeId, _col: NodeId, _value: Value) {}

        fn stamp_rhs(&mut self, _index: NodeId, _value: Value) {}
    }

    /// Every stamp, bit for bit.
    #[derive(Default)]
    struct Record(Vec<u64>);

    impl MatrixStamper for Record {
        fn stamp(&mut self, row: NodeId, col: NodeId, value: Value) {
            self.0.extend([row as u64, col as u64, value.to_bits()]);
        }

        fn stamp_rhs(&mut self, index: NodeId, value: Value) {
            self.0.extend([index as u64, value.to_bits()]);
        }
    }

    fn with(params: &[(&str, Value)]) -> Bjt {
        let params = params
            .iter()
            .map(|&(name, value)| (name.to_owned(), value))
            .collect();
        Bjt::new_npn("qrollback".into(), 1, 2, 3).with_params(&params)
    }

    /// Number a promoted device's internal unknowns after its four
    /// terminals, as the builder does for a transient, followed by its RBI
    /// branch current. Returns the unknown count.
    fn promote(bjt: &mut Bjt) -> usize {
        let mut last_node = 4;
        bjt.assign_mna_internal_nodes(|_| {
            last_node += 1;
            last_node
        });
        if bjt.needs_mna_rbi_branch() {
            bjt.assign_mna_rbi_branch(1);
            bjt.resolve_mna_rbi_branch(last_node);
            last_node += 1;
        }
        last_node
    }

    /// What a transient attempt does to a device, in the engine's order.
    #[derive(Clone, Copy)]
    enum Step {
        Load(usize),
        Stamp(usize),
        Probe(usize),
        Charge(usize),
        Gmin(Value),
        Reopen,
    }

    /// Newton loads and stamps, residual probes and charge passes at three
    /// biases, with a GMIN change and an operating-point reopen between
    /// them. It ends on an open load, so the limiter and cache keys are live.
    const ATTEMPT: [Step; 17] = [
        Step::Load(0),
        Step::Stamp(0),
        Step::Load(0),
        Step::Probe(0),
        Step::Charge(0),
        Step::Load(1),
        Step::Stamp(1),
        Step::Gmin(1e-9),
        Step::Load(1),
        Step::Charge(1),
        Step::Reopen,
        Step::Load(2),
        Step::Stamp(2),
        Step::Load(2),
        Step::Probe(2),
        Step::Charge(2),
        Step::Load(1),
    ];

    fn take(bjt: &mut Bjt, step: Step, biases: &[Vec<Value>; 3]) {
        match step {
            Step::Load(index) => bjt.update(&biases[index]),
            Step::Stamp(index) => bjt.stamp_nonlinear(&biases[index], &mut Discard, &mut []),
            Step::Probe(index) => bjt.update_static_linearization(&biases[index]),
            Step::Charge(index) => {
                if bjt.mna_promoted() {
                    let _ = bjt.mna_charge_state();
                } else {
                    let [vc, vb, ve] = [0, 1, 2].map(|node| biases[index][node]);
                    let _ = bjt.charge_snapshot(vc, vb, ve, 0.0);
                }
            }
            Step::Gmin(gmin) => bjt.set_junction_gmin(gmin),
            Step::Reopen => bjt.reopen_operating_point(),
        }
    }

    /// Terminals at `terminals`, internal states near the base and any RBI
    /// branch carrying a base-sized current.
    fn bias(bjt: &Bjt, nodes: usize, terminals: [Value; 3]) -> Vec<Value> {
        let mut solution = vec![terminals[1] - 0.05; nodes];
        solution[..3].copy_from_slice(&terminals);
        if bjt.needs_mna_rbi_branch() {
            solution[nodes - 1] = 2e-5;
        }
        solution
    }

    fn biases(bjt: &Bjt, nodes: usize) -> [Vec<Value>; 3] {
        [
            bias(bjt, nodes, [1.4, 0.76, 0.02]),
            bias(bjt, nodes, [0.9, 0.81, 0.05]),
            bias(bjt, nodes, [2.0, 0.70, 0.0]),
        ]
    }

    /// A restore brings back an earlier evaluation without the terminal rows
    /// the update summed from it, so the next stamp sums the restored
    /// evaluation's rows instead of reusing those of the load after it.
    #[test]
    fn a_stamp_after_a_restore_sums_the_restored_terminal_rows() {
        let mut bjt = with(&[
            ("BF", 75.0),
            ("RB", 100.0),
            ("RC", 2.0),
            ("RE", 1.0),
            ("CJE", 1e-12),
            ("CJC", 3e-12),
            ("TF", 1e-10),
        ]);
        let nodes = promote(&mut bjt);
        let [first, second, _] = biases(&bjt, nodes);
        bjt.update(&first);
        let mut state = BjtNonlinearState::default();
        bjt.nonlinear_state_snapshot_into(&mut state);
        let mut expected = Record::default();
        bjt.stamp_mna(&mut expected);
        bjt.update(&second);
        bjt.restore_nonlinear_state(state);
        assert!(bjt.mna_terminal_rows.0.is_none());
        let mut restored = Record::default();
        bjt.stamp_mna(&mut restored);
        assert_eq!(restored.0, expected.0);
    }

    /// Everything an attempt writes comes back. The state is taken after
    /// every prefix of an attempt, into one reused image; the rest of that
    /// attempt, a whole further one and a load at a third bias follow; the
    /// restored device must print exactly as it did when its state was
    /// taken, every cache and flag included.
    fn assert_rollback_restores(bjt: Bjt, nodes: usize) {
        let biases = biases(&bjt, nodes);
        let mut state = BjtNonlinearState::default();
        for split in 0..=ATTEMPT.len() {
            let mut device = bjt.clone();
            for &step in &ATTEMPT[..split] {
                take(&mut device, step, &biases);
            }
            let taken = format!("{device:?}");
            device.nonlinear_state_snapshot_into(&mut state);
            let next = ATTEMPT.iter().chain(&[Step::Load(2)]);
            for &step in ATTEMPT[split..].iter().chain(next) {
                take(&mut device, step, &biases);
            }
            assert_ne!(format!("{device:?}"), taken, "the attempt changed nothing");
            device.restore_nonlinear_state(state.clone());
            assert_eq!(format!("{device:?}"), taken, "restored after {split} steps");
        }
    }

    fn poison_reduced(reduced: &mut BjtReducedLinearization) {
        let nan = Value::NAN;
        reduced.internal_voltages.fill(nan);
        reduced.external_voltages.fill(nan);
        reduced.g_ii.iter_mut().for_each(|row| row.fill(nan));
        reduced.g_ie.iter_mut().for_each(|row| row.fill(nan));
        reduced.g_ei.iter_mut().for_each(|row| row.fill(nan));
        reduced.g_ee.iter_mut().for_each(|row| row.fill(nan));
        reduced.g_reduced.iter_mut().for_each(|row| row.fill(nan));
        reduced.z_i_static.fill(nan);
        reduced.z_e_static.fill(nan);
        reduced.cached_dynamic_inputs = Some(Default::default());
    }

    /// Every value in the private solve's reduction and charge caches made NaN.
    fn poison_legacy_caches(bjt: &mut Bjt) {
        let nan = Value::NAN;
        let mut reduced = bjt.reduced_linearization_cache.get();
        poison_reduced(&mut reduced);
        bjt.reduced_linearization_cache.set(reduced);
        poison_reduced(&mut bjt.previous_reduced_linearization);
        let mut snapshot = bjt.charge_snapshot_cache.get();
        let reduction = &mut snapshot.reduction;
        reduction.internal_voltages.fill(nan);
        reduction.external_voltages.fill(nan);
        reduction.g_ii.iter_mut().for_each(|row| row.fill(nan));
        reduction.g_ie.iter_mut().for_each(|row| row.fill(nan));
        reduction.g_ei.iter_mut().for_each(|row| row.fill(nan));
        reduction.g_ee.iter_mut().for_each(|row| row.fill(nan));
        reduction.g_reduced.iter_mut().for_each(|row| row.fill(nan));
        reduction.z_i_static.fill(nan);
        reduction.z_e_static.fill(nan);
        reduction.vbic_d_itzf_d_vrth = nan;
        for branch in &mut snapshot.branches {
            branch.charge = nan;
            branch.d_internal.fill(nan);
            branch.d_external.fill(nan);
        }
        bjt.charge_snapshot_cache.set(snapshot);
    }

    /// What the engine reads back from a promoted device through an attempt,
    /// bit for bit: its Newton and correction stamps, charge companions,
    /// convergence verdict, and the charges and currents an accepted step
    /// takes at the solution.
    fn promoted_reads(bjt: &mut Bjt, biases: &[Vec<Value>; 3]) -> Vec<u64> {
        let mut record = Record::default();
        for &step in ATTEMPT.iter().chain(&[Step::Load(2)]) {
            take(bjt, step, biases);
            let bias = match step {
                Step::Load(index)
                | Step::Stamp(index)
                | Step::Probe(index)
                | Step::Charge(index) => &biases[index],
                Step::Gmin(_) | Step::Reopen => &biases[0],
            };
            bjt.stamp_nonlinear(bias, &mut record, &mut []);
            bjt.stamp_mna_correction(&mut record, bias);
            record.0.push(u64::from(
                bjt.is_converged(NonlinearConvergenceCriteria::default()),
            ));
            let (branches, internal, external) = bjt.mna_charge_state();
            let (accepted, ..) = bjt.mna_charge_state_at_solution(bias);
            for branch in branches.iter().chain(&accepted) {
                let values = std::iter::once(branch.charge)
                    .chain(branch.d_internal)
                    .chain(branch.d_external);
                record.0.extend(values.map(Value::to_bits));
            }
            let values = internal
                .into_iter()
                .chain(external)
                .chain(bjt.mna_terminal_currents_at_solution(bias))
                .chain(bjt.mna_junction_voltages(bias));
            record.0.extend(values.map(Value::to_bits));
            if let Some(charge) = bjt.legacy_external_bc_charge(bias) {
                let values = [charge.voltage, charge.charge, charge.capacitance];
                record.0.extend(values.map(Value::to_bits));
            }
        }
        record.0
    }

    /// Copy the private solve's caches from `from`, temperature variants
    /// (which clone them from their parent) included.
    fn adopt_legacy_caches(bjt: &mut Bjt, from: &Bjt) {
        bjt.reduced_linearization_cache
            .set(from.reduced_linearization_cache.get());
        bjt.previous_reduced_linearization = from.previous_reduced_linearization;
        bjt.charge_snapshot_cache
            .set(from.charge_snapshot_cache.get());
        let variants = from.thermal_variant_cache.borrow();
        let own = bjt.thermal_variant_cache.get_mut();
        assert_eq!(own.len(), variants.len(), "temperature variants differ");
        for ((_, variant), (_, source)) in own.iter_mut().zip(variants.iter()) {
            adopt_legacy_caches(variant, source);
        }
    }

    /// A promoted device's image leaves out the private solve's caches. Take
    /// the state after every prefix of an attempt and poison those caches at
    /// once; through the rest of that attempt, a whole further one and the
    /// restore, nothing may read them: the restored device must read back
    /// exactly what the same device restored whole reads back, and match it
    /// in every other field.
    fn assert_promoted_rollback_skips_unread_caches(bjt: Bjt, nodes: usize) {
        assert!(bjt.mna_promoted());
        let biases = biases(&bjt, nodes);
        let mut state = BjtNonlinearState::default();
        for split in 0..=ATTEMPT.len() {
            let mut device = bjt.clone();
            for &step in &ATTEMPT[..split] {
                take(&mut device, step, &biases);
            }
            let mut whole = device.clone();
            device.nonlinear_state_snapshot_into(&mut state);
            poison_legacy_caches(&mut device);
            for &step in ATTEMPT[split..].iter().chain(&ATTEMPT) {
                take(&mut device, step, &biases);
            }
            device.restore_nonlinear_state(state.clone());
            assert_eq!(
                promoted_reads(&mut device, &biases),
                promoted_reads(&mut whole, &biases),
                "reads after a restore at {split} steps"
            );
            adopt_legacy_caches(&mut device, &whole);
            assert_eq!(
                format!("{device:?}"),
                format!("{whole:?}"),
                "state after a restore at {split} steps"
            );
        }
    }

    #[test]
    fn a_legacy_gummel_poon_rollback_restores_every_field_an_attempt_writes() {
        let card = [
            ("BF", 75.0),
            ("RB", 100.0),
            ("RC", 2.0),
            ("RE", 1.0),
            ("CJE", 1e-12),
            ("CJC", 3e-12),
            ("TF", 1e-10),
            ("VAF", 50.0),
        ];
        assert_rollback_restores(with(&card), 3);
        let mut bjt = with(&card);
        let nodes = promote(&mut bjt);
        assert_rollback_restores(bjt.clone(), nodes);
        assert_promoted_rollback_skips_unread_caches(bjt, nodes);
    }

    #[test]
    fn a_vbic_rollback_restores_every_field_an_attempt_writes() {
        let card = [
            ("LEVEL", 4.0),
            ("RCX", 2.0),
            ("RCI", 3.0),
            ("RBX", 4.0),
            ("RBI", 5.0),
            ("RE", 1.0),
            ("RS", 6.0),
            ("RBP", 7.0),
            ("CJE", 1e-12),
            ("CJC", 2e-12),
            ("TF", 1e-10),
            ("TD", 3e-11),
            ("SELFT", 1.0),
            ("RTH", 300.0),
            ("CTH", 1e-12),
        ];
        let mut bjt = with(&card[..11]);
        bjt.node_substrate = 4;
        assert_rollback_restores(bjt, 4);
        let mut bjt = with(&card);
        bjt.node_substrate = 4;
        let nodes = promote(&mut bjt);
        assert_rollback_restores(bjt.clone(), nodes);
        assert_promoted_rollback_skips_unread_caches(bjt, nodes);
    }
}
