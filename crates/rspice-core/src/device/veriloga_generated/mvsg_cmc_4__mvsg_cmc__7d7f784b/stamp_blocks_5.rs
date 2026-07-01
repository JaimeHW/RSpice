#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_80(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_fn382_calc_iq__alpha_phit: f64,
        var_fn382_calc_iq__alpha_phit_dn4: f64,
        var_fn382_calc_iq__etad0: f64,
        var_fn382_calc_iq__etad0_dn4: f64,
        var_fn382_calc_iq__etad0_dn5: f64,
        var_fn382_calc_iq__etad0_dn8: f64,
        var_fn382_calc_iq__etad0_dn9: f64,
        var_fn382_calc_iq__idsout: f64,
        var_fn382_calc_iq__idsout_dn22: f64,
        var_fn382_calc_iq__idsout_dn23: f64,
        var_fn382_calc_iq__idsout_dn25: f64,
        var_fn382_calc_iq__idsout_dn26: f64,
        var_fn382_calc_iq__idsout_dn4: f64,
        var_fn382_calc_iq__idsout_dn5: f64,
        var_fn382_calc_iq__idsout_dn8: f64,
        var_fn382_calc_iq__idsout_dn9: f64,
        var_fn382_calc_iq__lin: f64,
        var_fn382_calc_iq__ngf: f64,
        var_fn382_calc_iq__qcbflag: f64,
        var_fn382_calc_iq__qgsflag: f64,
        var_fn382_calc_iq__qinvs0: f64,
        var_fn382_calc_iq__qinvs0_dn4: f64,
        var_fn382_calc_iq__qinvs0_dn5: f64,
        var_fn382_calc_iq__qinvs0_dn8: f64,
        var_fn382_calc_iq__qinvs0_dn9: f64,
        var_fn382_calc_iq__qref0: f64,
        var_fn382_calc_iq__qref0_dn4: f64,
        var_fn382_calc_iq__trapfracdl: f64,
        var_fn382_calc_iq__trapfracdl_dn22: f64,
        var_fn382_calc_iq__trapfracdl_dn23: f64,
        var_fn382_calc_iq__trapfracdl_dn25: f64,
        var_fn382_calc_iq__trapfracdl_dn26: f64,
        var_fn382_calc_iq__two_n_phit0: f64,
        var_fn382_calc_iq__two_n_phit0_dn4: f64,
        var_fn382_calc_iq__type: f64,
        var_fn382_calc_iq__vbin: f64,
        var_fn382_calc_iq__vcin: f64,
        var_fn382_calc_iq__vgsin: f64,
        var_fn382_calc_iq__vgsin_dn8: f64,
        var_fn382_calc_iq__vgsin_dn9: f64,
        var_fn382_calc_iq__vtof: f64,
        var_fn382_calc_iq__vtof_dn4: f64,
        var_fn382_calc_iq__w: f64,
        var_guard406: f64,
        var_guard407: f64,
        var_phit: f64,
        var_phit_dn4: f64,
        var_fn382_calc_iq__etab_slot: &mut f64,
        var_fn382_calc_iq__etab_dn4_slot: &mut f64,
        var_fn382_calc_iq__etac_slot: &mut f64,
        var_fn382_calc_iq__etac_dn4_slot: &mut f64,
        var_fn382_calc_iq__etags_slot: &mut f64,
        var_fn382_calc_iq__etags_dn4_slot: &mut f64,
        var_fn382_calc_iq__etags_dn8_slot: &mut f64,
        var_fn382_calc_iq__etags_dn9_slot: &mut f64,
        var_fn382_calc_iq__exparg_slot: &mut f64,
        var_fn382_calc_iq__exparg_dn4_slot: &mut f64,
        var_fn382_calc_iq__exparg_dn5_slot: &mut f64,
        var_fn382_calc_iq__exparg_dn8_slot: &mut f64,
        var_fn382_calc_iq__exparg_dn9_slot: &mut f64,
        var_fn382_calc_iq__qd_slot: &mut f64,
        var_fn382_calc_iq__qd1_slot: &mut f64,
        var_fn382_calc_iq__qd1_dn4_slot: &mut f64,
        var_fn382_calc_iq__qd1_dn5_slot: &mut f64,
        var_fn382_calc_iq__qd1_dn8_slot: &mut f64,
        var_fn382_calc_iq__qd1_dn9_slot: &mut f64,
        var_fn382_calc_iq__qd2_slot: &mut f64,
        var_fn382_calc_iq__qd2_dn4_slot: &mut f64,
        var_fn382_calc_iq__qd2_dn5_slot: &mut f64,
        var_fn382_calc_iq__qd2_dn8_slot: &mut f64,
        var_fn382_calc_iq__qd2_dn9_slot: &mut f64,
        var_fn382_calc_iq__qd3_slot: &mut f64,
        var_fn382_calc_iq__qd3_dn4_slot: &mut f64,
        var_fn382_calc_iq__qd3_dn5_slot: &mut f64,
        var_fn382_calc_iq__qd3_dn8_slot: &mut f64,
        var_fn382_calc_iq__qd3_dn9_slot: &mut f64,
        var_fn382_calc_iq__qd_dn4_slot: &mut f64,
        var_fn382_calc_iq__qd_dn5_slot: &mut f64,
        var_fn382_calc_iq__qd_dn8_slot: &mut f64,
        var_fn382_calc_iq__qd_dn9_slot: &mut f64,
        var_fn382_calc_iq__qgdout_slot: &mut f64,
        var_fn382_calc_iq__qgdout_dn22_slot: &mut f64,
        var_fn382_calc_iq__qgdout_dn23_slot: &mut f64,
        var_fn382_calc_iq__qgdout_dn25_slot: &mut f64,
        var_fn382_calc_iq__qgdout_dn26_slot: &mut f64,
        var_fn382_calc_iq__qgdout_dn4_slot: &mut f64,
        var_fn382_calc_iq__qgdout_dn5_slot: &mut f64,
        var_fn382_calc_iq__qgdout_dn8_slot: &mut f64,
        var_fn382_calc_iq__qgdout_dn9_slot: &mut f64,
        var_fn382_calc_iq__qgsout_slot: &mut f64,
        var_fn382_calc_iq__qgsout_dn22_slot: &mut f64,
        var_fn382_calc_iq__qgsout_dn23_slot: &mut f64,
        var_fn382_calc_iq__qgsout_dn25_slot: &mut f64,
        var_fn382_calc_iq__qgsout_dn26_slot: &mut f64,
        var_fn382_calc_iq__qgsout_dn4_slot: &mut f64,
        var_fn382_calc_iq__qgsout_dn5_slot: &mut f64,
        var_fn382_calc_iq__qgsout_dn8_slot: &mut f64,
        var_fn382_calc_iq__qgsout_dn9_slot: &mut f64,
        var_fn382_calc_iq__qinvd0_slot: &mut f64,
        var_fn382_calc_iq__qinvd0_dn4_slot: &mut f64,
        var_fn382_calc_iq__qinvd0_dn5_slot: &mut f64,
        var_fn382_calc_iq__qinvd0_dn8_slot: &mut f64,
        var_fn382_calc_iq__qinvd0_dn9_slot: &mut f64,
        var_fn382_calc_iq__qinvdd_slot: &mut f64,
        var_fn382_calc_iq__qinvdd_dn4_slot: &mut f64,
        var_fn382_calc_iq__qinvdd_dn5_slot: &mut f64,
        var_fn382_calc_iq__qinvdd_dn8_slot: &mut f64,
        var_fn382_calc_iq__qinvdd_dn9_slot: &mut f64,
        var_fn382_calc_iq__qs_slot: &mut f64,
        var_fn382_calc_iq__qs2_slot: &mut f64,
        var_fn382_calc_iq__qs2_dn4_slot: &mut f64,
        var_fn382_calc_iq__qs2_dn5_slot: &mut f64,
        var_fn382_calc_iq__qs2_dn8_slot: &mut f64,
        var_fn382_calc_iq__qs2_dn9_slot: &mut f64,
        var_fn382_calc_iq__qs3_slot: &mut f64,
        var_fn382_calc_iq__qs3_dn4_slot: &mut f64,
        var_fn382_calc_iq__qs3_dn5_slot: &mut f64,
        var_fn382_calc_iq__qs3_dn8_slot: &mut f64,
        var_fn382_calc_iq__qs3_dn9_slot: &mut f64,
        var_fn382_calc_iq__qs_dn4_slot: &mut f64,
        var_fn382_calc_iq__qs_dn5_slot: &mut f64,
        var_fn382_calc_iq__qs_dn8_slot: &mut f64,
        var_fn382_calc_iq__qs_dn9_slot: &mut f64,
        var_fn382_calc_iq__qsqd_slot: &mut f64,
        var_fn382_calc_iq__qsqd_dn4_slot: &mut f64,
        var_fn382_calc_iq__qsqd_dn5_slot: &mut f64,
        var_fn382_calc_iq__qsqd_dn8_slot: &mut f64,
        var_fn382_calc_iq__qsqd_dn9_slot: &mut f64,
        var_fn382_calc_iq__return_slot: &mut f64,
        var_fn382_calc_iq__return_dn22_slot: &mut f64,
        var_fn382_calc_iq__return_dn23_slot: &mut f64,
        var_fn382_calc_iq__return_dn25_slot: &mut f64,
        var_fn382_calc_iq__return_dn26_slot: &mut f64,
        var_fn382_calc_iq__return_dn4_slot: &mut f64,
        var_fn382_calc_iq__return_dn5_slot: &mut f64,
        var_fn382_calc_iq__return_dn8_slot: &mut f64,
        var_fn382_calc_iq__return_dn9_slot: &mut f64,
        var_fn418_calc_ig__isdiodeout_slot: &mut f64,
        var_fn418_calc_ig__isdiodeout_dn4_slot: &mut f64,
        var_fn418_calc_ig__isrecout_slot: &mut f64,
        var_fn418_calc_ig__isrecout_dn4_slot: &mut f64,
        var_fn418_calc_ig__phitin_slot: &mut f64,
        var_fn418_calc_ig__phitin_dn4_slot: &mut f64,
        var_fn418_calc_ig__return_slot: &mut f64,
        var_fn418_calc_ig__return_dn13_slot: &mut f64,
        var_fn418_calc_ig__return_dn4_slot: &mut f64,
        var_fn418_calc_ig__return_dn8_slot: &mut f64,
        var_fn418_calc_ig__vgin_slot: &mut f64,
        var_fn418_calc_ig__vgin_dn13_slot: &mut f64,
        var_fn418_calc_ig__vgin_dn8_slot: &mut f64,
        var_fn418_calc_ig__vgsatin_slot: &mut f64,
        var_guard408_slot: &mut f64,
        var_guard409_slot: &mut f64,
        var_guard410_slot: &mut f64,
        var_guard411_slot: &mut f64,
        var_guard412_slot: &mut f64,
        var_guard413_slot: &mut f64,
        var_guard414_slot: &mut f64,
        var_guard415_slot: &mut f64,
        var_guard416_slot: &mut f64,
        var_guard417_slot: &mut f64,
        var_ids_slot: &mut f64,
        var_ids_dn22_slot: &mut f64,
        var_ids_dn23_slot: &mut f64,
        var_ids_dn25_slot: &mut f64,
        var_ids_dn26_slot: &mut f64,
        var_ids_dn4_slot: &mut f64,
        var_ids_dn5_slot: &mut f64,
        var_ids_dn8_slot: &mut f64,
        var_ids_dn9_slot: &mut f64,
        var_igdi_slot: &mut f64,
        var_igdi2_slot: &mut f64,
        var_igdi2_dn17_slot: &mut f64,
        var_igdi2_dn4_slot: &mut f64,
        var_igdi2_dn8_slot: &mut f64,
        var_igdi2db_slot: &mut f64,
        var_igdi2db_dn4_slot: &mut f64,
        var_igdi2db_dn5_slot: &mut f64,
        var_igdi2db_dn8_slot: &mut f64,
        var_igdi_dn17_slot: &mut f64,
        var_igdi_dn4_slot: &mut f64,
        var_igdi_dn8_slot: &mut f64,
        var_igdidb_slot: &mut f64,
        var_igdidb_dn4_slot: &mut f64,
        var_igdidb_dn5_slot: &mut f64,
        var_igdidb_dn8_slot: &mut f64,
        var_igsi_slot: &mut f64,
        var_igsi2_slot: &mut f64,
        var_igsi2_dn13_slot: &mut f64,
        var_igsi2_dn4_slot: &mut f64,
        var_igsi2_dn8_slot: &mut f64,
        var_igsi2db_slot: &mut f64,
        var_igsi2db_dn4_slot: &mut f64,
        var_igsi2db_dn8_slot: &mut f64,
        var_igsi2db_dn9_slot: &mut f64,
        var_igsi_dn13_slot: &mut f64,
        var_igsi_dn4_slot: &mut f64,
        var_igsi_dn8_slot: &mut f64,
        var_igsidb_slot: &mut f64,
        var_igsidb_dn4_slot: &mut f64,
        var_igsidb_dn8_slot: &mut f64,
        var_igsidb_dn9_slot: &mut f64,
        var_qgd_slot: &mut f64,
        var_qgd_dn22_slot: &mut f64,
        var_qgd_dn23_slot: &mut f64,
        var_qgd_dn25_slot: &mut f64,
        var_qgd_dn26_slot: &mut f64,
        var_qgd_dn4_slot: &mut f64,
        var_qgd_dn5_slot: &mut f64,
        var_qgd_dn8_slot: &mut f64,
        var_qgd_dn9_slot: &mut f64,
        var_qgs_slot: &mut f64,
        var_qgs_dn22_slot: &mut f64,
        var_qgs_dn23_slot: &mut f64,
        var_qgs_dn25_slot: &mut f64,
        var_qgs_dn26_slot: &mut f64,
        var_qgs_dn4_slot: &mut f64,
        var_qgs_dn5_slot: &mut f64,
        var_qgs_dn8_slot: &mut f64,
        var_qgs_dn9_slot: &mut f64,
    ) {
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let mut var_fn382_calc_iq__etab: f64 = *var_fn382_calc_iq__etab_slot;
        let mut var_fn382_calc_iq__etab_dn4: f64 = *var_fn382_calc_iq__etab_dn4_slot;
        let mut var_fn382_calc_iq__etac: f64 = *var_fn382_calc_iq__etac_slot;
        let mut var_fn382_calc_iq__etac_dn4: f64 = *var_fn382_calc_iq__etac_dn4_slot;
        let mut var_fn382_calc_iq__etags: f64 = *var_fn382_calc_iq__etags_slot;
        let mut var_fn382_calc_iq__etags_dn4: f64 = *var_fn382_calc_iq__etags_dn4_slot;
        let mut var_fn382_calc_iq__etags_dn8: f64 = *var_fn382_calc_iq__etags_dn8_slot;
        let mut var_fn382_calc_iq__etags_dn9: f64 = *var_fn382_calc_iq__etags_dn9_slot;
        let mut var_fn382_calc_iq__exparg: f64 = *var_fn382_calc_iq__exparg_slot;
        let mut var_fn382_calc_iq__exparg_dn4: f64 = *var_fn382_calc_iq__exparg_dn4_slot;
        let mut var_fn382_calc_iq__exparg_dn5: f64 = *var_fn382_calc_iq__exparg_dn5_slot;
        let mut var_fn382_calc_iq__exparg_dn8: f64 = *var_fn382_calc_iq__exparg_dn8_slot;
        let mut var_fn382_calc_iq__exparg_dn9: f64 = *var_fn382_calc_iq__exparg_dn9_slot;
        let mut var_fn382_calc_iq__qd: f64 = *var_fn382_calc_iq__qd_slot;
        let mut var_fn382_calc_iq__qd1: f64 = *var_fn382_calc_iq__qd1_slot;
        let mut var_fn382_calc_iq__qd1_dn4: f64 = *var_fn382_calc_iq__qd1_dn4_slot;
        let mut var_fn382_calc_iq__qd1_dn5: f64 = *var_fn382_calc_iq__qd1_dn5_slot;
        let mut var_fn382_calc_iq__qd1_dn8: f64 = *var_fn382_calc_iq__qd1_dn8_slot;
        let mut var_fn382_calc_iq__qd1_dn9: f64 = *var_fn382_calc_iq__qd1_dn9_slot;
        let mut var_fn382_calc_iq__qd2: f64 = *var_fn382_calc_iq__qd2_slot;
        let mut var_fn382_calc_iq__qd2_dn4: f64 = *var_fn382_calc_iq__qd2_dn4_slot;
        let mut var_fn382_calc_iq__qd2_dn5: f64 = *var_fn382_calc_iq__qd2_dn5_slot;
        let mut var_fn382_calc_iq__qd2_dn8: f64 = *var_fn382_calc_iq__qd2_dn8_slot;
        let mut var_fn382_calc_iq__qd2_dn9: f64 = *var_fn382_calc_iq__qd2_dn9_slot;
        let mut var_fn382_calc_iq__qd3: f64 = *var_fn382_calc_iq__qd3_slot;
        let mut var_fn382_calc_iq__qd3_dn4: f64 = *var_fn382_calc_iq__qd3_dn4_slot;
        let mut var_fn382_calc_iq__qd3_dn5: f64 = *var_fn382_calc_iq__qd3_dn5_slot;
        let mut var_fn382_calc_iq__qd3_dn8: f64 = *var_fn382_calc_iq__qd3_dn8_slot;
        let mut var_fn382_calc_iq__qd3_dn9: f64 = *var_fn382_calc_iq__qd3_dn9_slot;
        let mut var_fn382_calc_iq__qd_dn4: f64 = *var_fn382_calc_iq__qd_dn4_slot;
        let mut var_fn382_calc_iq__qd_dn5: f64 = *var_fn382_calc_iq__qd_dn5_slot;
        let mut var_fn382_calc_iq__qd_dn8: f64 = *var_fn382_calc_iq__qd_dn8_slot;
        let mut var_fn382_calc_iq__qd_dn9: f64 = *var_fn382_calc_iq__qd_dn9_slot;
        let mut var_fn382_calc_iq__qgdout: f64 = *var_fn382_calc_iq__qgdout_slot;
        let mut var_fn382_calc_iq__qgdout_dn22: f64 = *var_fn382_calc_iq__qgdout_dn22_slot;
        let mut var_fn382_calc_iq__qgdout_dn23: f64 = *var_fn382_calc_iq__qgdout_dn23_slot;
        let mut var_fn382_calc_iq__qgdout_dn25: f64 = *var_fn382_calc_iq__qgdout_dn25_slot;
        let mut var_fn382_calc_iq__qgdout_dn26: f64 = *var_fn382_calc_iq__qgdout_dn26_slot;
        let mut var_fn382_calc_iq__qgdout_dn4: f64 = *var_fn382_calc_iq__qgdout_dn4_slot;
        let mut var_fn382_calc_iq__qgdout_dn5: f64 = *var_fn382_calc_iq__qgdout_dn5_slot;
        let mut var_fn382_calc_iq__qgdout_dn8: f64 = *var_fn382_calc_iq__qgdout_dn8_slot;
        let mut var_fn382_calc_iq__qgdout_dn9: f64 = *var_fn382_calc_iq__qgdout_dn9_slot;
        let mut var_fn382_calc_iq__qgsout: f64 = *var_fn382_calc_iq__qgsout_slot;
        let mut var_fn382_calc_iq__qgsout_dn22: f64 = *var_fn382_calc_iq__qgsout_dn22_slot;
        let mut var_fn382_calc_iq__qgsout_dn23: f64 = *var_fn382_calc_iq__qgsout_dn23_slot;
        let mut var_fn382_calc_iq__qgsout_dn25: f64 = *var_fn382_calc_iq__qgsout_dn25_slot;
        let mut var_fn382_calc_iq__qgsout_dn26: f64 = *var_fn382_calc_iq__qgsout_dn26_slot;
        let mut var_fn382_calc_iq__qgsout_dn4: f64 = *var_fn382_calc_iq__qgsout_dn4_slot;
        let mut var_fn382_calc_iq__qgsout_dn5: f64 = *var_fn382_calc_iq__qgsout_dn5_slot;
        let mut var_fn382_calc_iq__qgsout_dn8: f64 = *var_fn382_calc_iq__qgsout_dn8_slot;
        let mut var_fn382_calc_iq__qgsout_dn9: f64 = *var_fn382_calc_iq__qgsout_dn9_slot;
        let mut var_fn382_calc_iq__qinvd0: f64 = *var_fn382_calc_iq__qinvd0_slot;
        let mut var_fn382_calc_iq__qinvd0_dn4: f64 = *var_fn382_calc_iq__qinvd0_dn4_slot;
        let mut var_fn382_calc_iq__qinvd0_dn5: f64 = *var_fn382_calc_iq__qinvd0_dn5_slot;
        let mut var_fn382_calc_iq__qinvd0_dn8: f64 = *var_fn382_calc_iq__qinvd0_dn8_slot;
        let mut var_fn382_calc_iq__qinvd0_dn9: f64 = *var_fn382_calc_iq__qinvd0_dn9_slot;
        let mut var_fn382_calc_iq__qinvdd: f64 = *var_fn382_calc_iq__qinvdd_slot;
        let mut var_fn382_calc_iq__qinvdd_dn4: f64 = *var_fn382_calc_iq__qinvdd_dn4_slot;
        let mut var_fn382_calc_iq__qinvdd_dn5: f64 = *var_fn382_calc_iq__qinvdd_dn5_slot;
        let mut var_fn382_calc_iq__qinvdd_dn8: f64 = *var_fn382_calc_iq__qinvdd_dn8_slot;
        let mut var_fn382_calc_iq__qinvdd_dn9: f64 = *var_fn382_calc_iq__qinvdd_dn9_slot;
        let mut var_fn382_calc_iq__qs: f64 = *var_fn382_calc_iq__qs_slot;
        let mut var_fn382_calc_iq__qs2: f64 = *var_fn382_calc_iq__qs2_slot;
        let mut var_fn382_calc_iq__qs2_dn4: f64 = *var_fn382_calc_iq__qs2_dn4_slot;
        let mut var_fn382_calc_iq__qs2_dn5: f64 = *var_fn382_calc_iq__qs2_dn5_slot;
        let mut var_fn382_calc_iq__qs2_dn8: f64 = *var_fn382_calc_iq__qs2_dn8_slot;
        let mut var_fn382_calc_iq__qs2_dn9: f64 = *var_fn382_calc_iq__qs2_dn9_slot;
        let mut var_fn382_calc_iq__qs3: f64 = *var_fn382_calc_iq__qs3_slot;
        let mut var_fn382_calc_iq__qs3_dn4: f64 = *var_fn382_calc_iq__qs3_dn4_slot;
        let mut var_fn382_calc_iq__qs3_dn5: f64 = *var_fn382_calc_iq__qs3_dn5_slot;
        let mut var_fn382_calc_iq__qs3_dn8: f64 = *var_fn382_calc_iq__qs3_dn8_slot;
        let mut var_fn382_calc_iq__qs3_dn9: f64 = *var_fn382_calc_iq__qs3_dn9_slot;
        let mut var_fn382_calc_iq__qs_dn4: f64 = *var_fn382_calc_iq__qs_dn4_slot;
        let mut var_fn382_calc_iq__qs_dn5: f64 = *var_fn382_calc_iq__qs_dn5_slot;
        let mut var_fn382_calc_iq__qs_dn8: f64 = *var_fn382_calc_iq__qs_dn8_slot;
        let mut var_fn382_calc_iq__qs_dn9: f64 = *var_fn382_calc_iq__qs_dn9_slot;
        let mut var_fn382_calc_iq__qsqd: f64 = *var_fn382_calc_iq__qsqd_slot;
        let mut var_fn382_calc_iq__qsqd_dn4: f64 = *var_fn382_calc_iq__qsqd_dn4_slot;
        let mut var_fn382_calc_iq__qsqd_dn5: f64 = *var_fn382_calc_iq__qsqd_dn5_slot;
        let mut var_fn382_calc_iq__qsqd_dn8: f64 = *var_fn382_calc_iq__qsqd_dn8_slot;
        let mut var_fn382_calc_iq__qsqd_dn9: f64 = *var_fn382_calc_iq__qsqd_dn9_slot;
        let mut var_fn382_calc_iq__return: f64 = *var_fn382_calc_iq__return_slot;
        let mut var_fn382_calc_iq__return_dn22: f64 = *var_fn382_calc_iq__return_dn22_slot;
        let mut var_fn382_calc_iq__return_dn23: f64 = *var_fn382_calc_iq__return_dn23_slot;
        let mut var_fn382_calc_iq__return_dn25: f64 = *var_fn382_calc_iq__return_dn25_slot;
        let mut var_fn382_calc_iq__return_dn26: f64 = *var_fn382_calc_iq__return_dn26_slot;
        let mut var_fn382_calc_iq__return_dn4: f64 = *var_fn382_calc_iq__return_dn4_slot;
        let mut var_fn382_calc_iq__return_dn5: f64 = *var_fn382_calc_iq__return_dn5_slot;
        let mut var_fn382_calc_iq__return_dn8: f64 = *var_fn382_calc_iq__return_dn8_slot;
        let mut var_fn382_calc_iq__return_dn9: f64 = *var_fn382_calc_iq__return_dn9_slot;
        let mut var_fn418_calc_ig__isdiodeout: f64 = *var_fn418_calc_ig__isdiodeout_slot;
        let mut var_fn418_calc_ig__isdiodeout_dn4: f64 = *var_fn418_calc_ig__isdiodeout_dn4_slot;
        let mut var_fn418_calc_ig__isrecout: f64 = *var_fn418_calc_ig__isrecout_slot;
        let mut var_fn418_calc_ig__isrecout_dn4: f64 = *var_fn418_calc_ig__isrecout_dn4_slot;
        let mut var_fn418_calc_ig__phitin: f64 = *var_fn418_calc_ig__phitin_slot;
        let mut var_fn418_calc_ig__phitin_dn4: f64 = *var_fn418_calc_ig__phitin_dn4_slot;
        let mut var_fn418_calc_ig__return: f64 = *var_fn418_calc_ig__return_slot;
        let mut var_fn418_calc_ig__return_dn13: f64 = *var_fn418_calc_ig__return_dn13_slot;
        let mut var_fn418_calc_ig__return_dn4: f64 = *var_fn418_calc_ig__return_dn4_slot;
        let mut var_fn418_calc_ig__return_dn8: f64 = *var_fn418_calc_ig__return_dn8_slot;
        let mut var_fn418_calc_ig__vgin: f64 = *var_fn418_calc_ig__vgin_slot;
        let mut var_fn418_calc_ig__vgin_dn13: f64 = *var_fn418_calc_ig__vgin_dn13_slot;
        let mut var_fn418_calc_ig__vgin_dn8: f64 = *var_fn418_calc_ig__vgin_dn8_slot;
        let mut var_fn418_calc_ig__vgsatin: f64 = *var_fn418_calc_ig__vgsatin_slot;
        let mut var_guard408: f64 = *var_guard408_slot;
        let mut var_guard409: f64 = *var_guard409_slot;
        let mut var_guard410: f64 = *var_guard410_slot;
        let mut var_guard411: f64 = *var_guard411_slot;
        let mut var_guard412: f64 = *var_guard412_slot;
        let mut var_guard413: f64 = *var_guard413_slot;
        let mut var_guard414: f64 = *var_guard414_slot;
        let mut var_guard415: f64 = *var_guard415_slot;
        let mut var_guard416: f64 = *var_guard416_slot;
        let mut var_guard417: f64 = *var_guard417_slot;
        let mut var_ids: f64 = *var_ids_slot;
        let mut var_ids_dn22: f64 = *var_ids_dn22_slot;
        let mut var_ids_dn23: f64 = *var_ids_dn23_slot;
        let mut var_ids_dn25: f64 = *var_ids_dn25_slot;
        let mut var_ids_dn26: f64 = *var_ids_dn26_slot;
        let mut var_ids_dn4: f64 = *var_ids_dn4_slot;
        let mut var_ids_dn5: f64 = *var_ids_dn5_slot;
        let mut var_ids_dn8: f64 = *var_ids_dn8_slot;
        let mut var_ids_dn9: f64 = *var_ids_dn9_slot;
        let mut var_igdi: f64 = *var_igdi_slot;
        let mut var_igdi2: f64 = *var_igdi2_slot;
        let mut var_igdi2_dn17: f64 = *var_igdi2_dn17_slot;
        let mut var_igdi2_dn4: f64 = *var_igdi2_dn4_slot;
        let mut var_igdi2_dn8: f64 = *var_igdi2_dn8_slot;
        let mut var_igdi2db: f64 = *var_igdi2db_slot;
        let mut var_igdi2db_dn4: f64 = *var_igdi2db_dn4_slot;
        let mut var_igdi2db_dn5: f64 = *var_igdi2db_dn5_slot;
        let mut var_igdi2db_dn8: f64 = *var_igdi2db_dn8_slot;
        let mut var_igdi_dn17: f64 = *var_igdi_dn17_slot;
        let mut var_igdi_dn4: f64 = *var_igdi_dn4_slot;
        let mut var_igdi_dn8: f64 = *var_igdi_dn8_slot;
        let mut var_igdidb: f64 = *var_igdidb_slot;
        let mut var_igdidb_dn4: f64 = *var_igdidb_dn4_slot;
        let mut var_igdidb_dn5: f64 = *var_igdidb_dn5_slot;
        let mut var_igdidb_dn8: f64 = *var_igdidb_dn8_slot;
        let mut var_igsi: f64 = *var_igsi_slot;
        let mut var_igsi2: f64 = *var_igsi2_slot;
        let mut var_igsi2_dn13: f64 = *var_igsi2_dn13_slot;
        let mut var_igsi2_dn4: f64 = *var_igsi2_dn4_slot;
        let mut var_igsi2_dn8: f64 = *var_igsi2_dn8_slot;
        let mut var_igsi2db: f64 = *var_igsi2db_slot;
        let mut var_igsi2db_dn4: f64 = *var_igsi2db_dn4_slot;
        let mut var_igsi2db_dn8: f64 = *var_igsi2db_dn8_slot;
        let mut var_igsi2db_dn9: f64 = *var_igsi2db_dn9_slot;
        let mut var_igsi_dn13: f64 = *var_igsi_dn13_slot;
        let mut var_igsi_dn4: f64 = *var_igsi_dn4_slot;
        let mut var_igsi_dn8: f64 = *var_igsi_dn8_slot;
        let mut var_igsidb: f64 = *var_igsidb_slot;
        let mut var_igsidb_dn4: f64 = *var_igsidb_dn4_slot;
        let mut var_igsidb_dn8: f64 = *var_igsidb_dn8_slot;
        let mut var_igsidb_dn9: f64 = *var_igsidb_dn9_slot;
        let mut var_qgd: f64 = *var_qgd_slot;
        let mut var_qgd_dn22: f64 = *var_qgd_dn22_slot;
        let mut var_qgd_dn23: f64 = *var_qgd_dn23_slot;
        let mut var_qgd_dn25: f64 = *var_qgd_dn25_slot;
        let mut var_qgd_dn26: f64 = *var_qgd_dn26_slot;
        let mut var_qgd_dn4: f64 = *var_qgd_dn4_slot;
        let mut var_qgd_dn5: f64 = *var_qgd_dn5_slot;
        let mut var_qgd_dn8: f64 = *var_qgd_dn8_slot;
        let mut var_qgd_dn9: f64 = *var_qgd_dn9_slot;
        let mut var_qgs: f64 = *var_qgs_slot;
        let mut var_qgs_dn22: f64 = *var_qgs_dn22_slot;
        let mut var_qgs_dn23: f64 = *var_qgs_dn23_slot;
        let mut var_qgs_dn25: f64 = *var_qgs_dn25_slot;
        let mut var_qgs_dn26: f64 = *var_qgs_dn26_slot;
        let mut var_qgs_dn4: f64 = *var_qgs_dn4_slot;
        let mut var_qgs_dn5: f64 = *var_qgs_dn5_slot;
        let mut var_qgs_dn8: f64 = *var_qgs_dn8_slot;
        let mut var_qgs_dn9: f64 = *var_qgs_dn9_slot;

        let (assign32300_e29516, assign32300_e29516_d_n4, assign32300_e29516_d_n5, assign32300_e29516_d_n8, assign32300_e29516_d_n9,) = {
    if ((var_guard406 == 0.0) && (var_guard407 != 0.0)) {
        let assign32300_e29513: f64 = (var_fn382_calc_iq__etad0).exp();
        let assign32300_e29514: f64 = (var_fn382_calc_iq__qref0 * assign32300_e29513);
        (assign32300_e29514, ((var_fn382_calc_iq__qref0_dn4 * assign32300_e29513) + (var_fn382_calc_iq__qref0 * (assign32300_e29513 * var_fn382_calc_iq__etad0_dn4))), (var_fn382_calc_iq__qref0 * (assign32300_e29513 * var_fn382_calc_iq__etad0_dn5)), (var_fn382_calc_iq__qref0 * (assign32300_e29513 * var_fn382_calc_iq__etad0_dn8)), (var_fn382_calc_iq__qref0 * (assign32300_e29513 * var_fn382_calc_iq__etad0_dn9)),)
    } else {
        (var_fn382_calc_iq__qinvd0, var_fn382_calc_iq__qinvd0_dn4, var_fn382_calc_iq__qinvd0_dn5, var_fn382_calc_iq__qinvd0_dn8, var_fn382_calc_iq__qinvd0_dn9,)
    }
};
        var_fn382_calc_iq__qinvd0 = assign32300_e29516;
        var_fn382_calc_iq__qinvd0_dn4 = assign32300_e29516_d_n4;
        var_fn382_calc_iq__qinvd0_dn5 = assign32300_e29516_d_n5;
        var_fn382_calc_iq__qinvd0_dn8 = assign32300_e29516_d_n8;
        var_fn382_calc_iq__qinvd0_dn9 = assign32300_e29516_d_n9;

        let (assign32310_e29530, assign32310_e29530_d_n4, assign32310_e29530_d_n5, assign32310_e29530_d_n8, assign32310_e29530_d_n9,) = {
    if ((var_guard406 == 0.0) && (var_guard407 == 0.0)) {
        let assign32310_e29525: f64 = (var_fn382_calc_iq__etad0).exp();
        let assign32310_e29526: f64 = (1.0 + assign32310_e29525);
        let assign32310_e29527: f64 = (assign32310_e29526).ln();
        let assign32310_e29528: f64 = (var_fn382_calc_iq__qref0 * assign32310_e29527);
        (assign32310_e29528, ((var_fn382_calc_iq__qref0_dn4 * assign32310_e29527) + (var_fn382_calc_iq__qref0 * ((assign32310_e29525 * var_fn382_calc_iq__etad0_dn4) / assign32310_e29526))), (var_fn382_calc_iq__qref0 * ((assign32310_e29525 * var_fn382_calc_iq__etad0_dn5) / assign32310_e29526)), (var_fn382_calc_iq__qref0 * ((assign32310_e29525 * var_fn382_calc_iq__etad0_dn8) / assign32310_e29526)), (var_fn382_calc_iq__qref0 * ((assign32310_e29525 * var_fn382_calc_iq__etad0_dn9) / assign32310_e29526)),)
    } else {
        (var_fn382_calc_iq__qinvd0, var_fn382_calc_iq__qinvd0_dn4, var_fn382_calc_iq__qinvd0_dn5, var_fn382_calc_iq__qinvd0_dn8, var_fn382_calc_iq__qinvd0_dn9,)
    }
};
        var_fn382_calc_iq__qinvd0 = assign32310_e29530;
        var_fn382_calc_iq__qinvd0_dn4 = assign32310_e29530_d_n4;
        var_fn382_calc_iq__qinvd0_dn5 = assign32310_e29530_d_n5;
        var_fn382_calc_iq__qinvd0_dn8 = assign32310_e29530_d_n8;
        var_fn382_calc_iq__qinvd0_dn9 = assign32310_e29530_d_n9;

        let assign32320_e29533: f64 = (var_fn382_calc_iq__qinvs0 * var_fn382_calc_iq__qinvs0);
        let assign32320_e29535: f64 = (assign32320_e29533 + 1e-38);
        var_fn382_calc_iq__qs2 = assign32320_e29535;
        var_fn382_calc_iq__qs2_dn4 = ((var_fn382_calc_iq__qinvs0_dn4 * var_fn382_calc_iq__qinvs0) + (var_fn382_calc_iq__qinvs0 * var_fn382_calc_iq__qinvs0_dn4));
        var_fn382_calc_iq__qs2_dn5 = ((var_fn382_calc_iq__qinvs0_dn5 * var_fn382_calc_iq__qinvs0) + (var_fn382_calc_iq__qinvs0 * var_fn382_calc_iq__qinvs0_dn5));
        var_fn382_calc_iq__qs2_dn8 = ((var_fn382_calc_iq__qinvs0_dn8 * var_fn382_calc_iq__qinvs0) + (var_fn382_calc_iq__qinvs0 * var_fn382_calc_iq__qinvs0_dn8));
        var_fn382_calc_iq__qs2_dn9 = ((var_fn382_calc_iq__qinvs0_dn9 * var_fn382_calc_iq__qinvs0) + (var_fn382_calc_iq__qinvs0 * var_fn382_calc_iq__qinvs0_dn9));

        let assign32330_e29538: f64 = (var_fn382_calc_iq__qs2 * var_fn382_calc_iq__qinvs0);
        let assign32330_e29540: f64 = (assign32330_e29538 + 1e-57);
        var_fn382_calc_iq__qs3 = assign32330_e29540;
        var_fn382_calc_iq__qs3_dn4 = ((var_fn382_calc_iq__qs2_dn4 * var_fn382_calc_iq__qinvs0) + (var_fn382_calc_iq__qs2 * var_fn382_calc_iq__qinvs0_dn4));
        var_fn382_calc_iq__qs3_dn5 = ((var_fn382_calc_iq__qs2_dn5 * var_fn382_calc_iq__qinvs0) + (var_fn382_calc_iq__qs2 * var_fn382_calc_iq__qinvs0_dn5));
        var_fn382_calc_iq__qs3_dn8 = ((var_fn382_calc_iq__qs2_dn8 * var_fn382_calc_iq__qinvs0) + (var_fn382_calc_iq__qs2 * var_fn382_calc_iq__qinvs0_dn8));
        var_fn382_calc_iq__qs3_dn9 = ((var_fn382_calc_iq__qs2_dn9 * var_fn382_calc_iq__qinvs0) + (var_fn382_calc_iq__qs2 * var_fn382_calc_iq__qinvs0_dn9));

        let assign32340_e29543: f64 = (var_fn382_calc_iq__qinvd0 * var_fn382_calc_iq__qinvd0);
        let assign32340_e29545: f64 = (assign32340_e29543 + 1e-38);
        var_fn382_calc_iq__qd2 = assign32340_e29545;
        var_fn382_calc_iq__qd2_dn4 = ((var_fn382_calc_iq__qinvd0_dn4 * var_fn382_calc_iq__qinvd0) + (var_fn382_calc_iq__qinvd0 * var_fn382_calc_iq__qinvd0_dn4));
        var_fn382_calc_iq__qd2_dn5 = ((var_fn382_calc_iq__qinvd0_dn5 * var_fn382_calc_iq__qinvd0) + (var_fn382_calc_iq__qinvd0 * var_fn382_calc_iq__qinvd0_dn5));
        var_fn382_calc_iq__qd2_dn8 = ((var_fn382_calc_iq__qinvd0_dn8 * var_fn382_calc_iq__qinvd0) + (var_fn382_calc_iq__qinvd0 * var_fn382_calc_iq__qinvd0_dn8));
        var_fn382_calc_iq__qd2_dn9 = ((var_fn382_calc_iq__qinvd0_dn9 * var_fn382_calc_iq__qinvd0) + (var_fn382_calc_iq__qinvd0 * var_fn382_calc_iq__qinvd0_dn9));

        let assign32350_e29548: f64 = (var_fn382_calc_iq__qd2 * var_fn382_calc_iq__qinvd0);
        let assign32350_e29550: f64 = (assign32350_e29548 + 1e-57);
        var_fn382_calc_iq__qd3 = assign32350_e29550;
        var_fn382_calc_iq__qd3_dn4 = ((var_fn382_calc_iq__qd2_dn4 * var_fn382_calc_iq__qinvd0) + (var_fn382_calc_iq__qd2 * var_fn382_calc_iq__qinvd0_dn4));
        var_fn382_calc_iq__qd3_dn5 = ((var_fn382_calc_iq__qd2_dn5 * var_fn382_calc_iq__qinvd0) + (var_fn382_calc_iq__qd2 * var_fn382_calc_iq__qinvd0_dn5));
        var_fn382_calc_iq__qd3_dn8 = ((var_fn382_calc_iq__qd2_dn8 * var_fn382_calc_iq__qinvd0) + (var_fn382_calc_iq__qd2 * var_fn382_calc_iq__qinvd0_dn8));
        var_fn382_calc_iq__qd3_dn9 = ((var_fn382_calc_iq__qd2_dn9 * var_fn382_calc_iq__qinvd0) + (var_fn382_calc_iq__qd2 * var_fn382_calc_iq__qinvd0_dn9));

        let assign32360_e29553: f64 = (var_fn382_calc_iq__qinvs0 * var_fn382_calc_iq__qinvd0);
        let assign32360_e29555: f64 = (assign32360_e29553 + 1e-38);
        var_fn382_calc_iq__qsqd = assign32360_e29555;
        var_fn382_calc_iq__qsqd_dn4 = ((var_fn382_calc_iq__qinvs0_dn4 * var_fn382_calc_iq__qinvd0) + (var_fn382_calc_iq__qinvs0 * var_fn382_calc_iq__qinvd0_dn4));
        var_fn382_calc_iq__qsqd_dn5 = ((var_fn382_calc_iq__qinvs0_dn5 * var_fn382_calc_iq__qinvd0) + (var_fn382_calc_iq__qinvs0 * var_fn382_calc_iq__qinvd0_dn5));
        var_fn382_calc_iq__qsqd_dn8 = ((var_fn382_calc_iq__qinvs0_dn8 * var_fn382_calc_iq__qinvd0) + (var_fn382_calc_iq__qinvs0 * var_fn382_calc_iq__qinvd0_dn8));
        var_fn382_calc_iq__qsqd_dn9 = ((var_fn382_calc_iq__qinvs0_dn9 * var_fn382_calc_iq__qinvd0) + (var_fn382_calc_iq__qinvs0 * var_fn382_calc_iq__qinvd0_dn9));

        let assign32370_e29558: f64 = (2.0 / 3.0);
        let assign32370_e29561: f64 = (var_fn382_calc_iq__qs2 + var_fn382_calc_iq__qd2);
        let assign32370_e29563: f64 = (assign32370_e29561 + var_fn382_calc_iq__qsqd);
        let assign32370_e29564: f64 = (assign32370_e29558 * assign32370_e29563);
        let assign32370_e29567: f64 = (var_fn382_calc_iq__qinvs0 + var_fn382_calc_iq__qinvd0);
        let assign32370_e29569: f64 = (assign32370_e29567 + 2e-19);
        let assign32370_e29570: f64 = (assign32370_e29564 / assign32370_e29569);
        var_fn382_calc_iq__qinvdd = assign32370_e29570;
        var_fn382_calc_iq__qinvdd_dn4 = ((((assign32370_e29558 * ((var_fn382_calc_iq__qs2_dn4 + var_fn382_calc_iq__qd2_dn4) + var_fn382_calc_iq__qsqd_dn4)) * assign32370_e29569) - (assign32370_e29564 * (var_fn382_calc_iq__qinvs0_dn4 + var_fn382_calc_iq__qinvd0_dn4))) / (assign32370_e29569 * assign32370_e29569));
        var_fn382_calc_iq__qinvdd_dn5 = ((((assign32370_e29558 * ((var_fn382_calc_iq__qs2_dn5 + var_fn382_calc_iq__qd2_dn5) + var_fn382_calc_iq__qsqd_dn5)) * assign32370_e29569) - (assign32370_e29564 * (var_fn382_calc_iq__qinvs0_dn5 + var_fn382_calc_iq__qinvd0_dn5))) / (assign32370_e29569 * assign32370_e29569));
        var_fn382_calc_iq__qinvdd_dn8 = ((((assign32370_e29558 * ((var_fn382_calc_iq__qs2_dn8 + var_fn382_calc_iq__qd2_dn8) + var_fn382_calc_iq__qsqd_dn8)) * assign32370_e29569) - (assign32370_e29564 * (var_fn382_calc_iq__qinvs0_dn8 + var_fn382_calc_iq__qinvd0_dn8))) / (assign32370_e29569 * assign32370_e29569));
        var_fn382_calc_iq__qinvdd_dn9 = ((((assign32370_e29558 * ((var_fn382_calc_iq__qs2_dn9 + var_fn382_calc_iq__qd2_dn9) + var_fn382_calc_iq__qsqd_dn9)) * assign32370_e29569) - (assign32370_e29564 * (var_fn382_calc_iq__qinvs0_dn9 + var_fn382_calc_iq__qinvd0_dn9))) / (assign32370_e29569 * assign32370_e29569));

        let assign32380_e29574: f64 = (2.0 * var_fn382_calc_iq__qs3);
        let assign32380_e29577: f64 = (3.0 * var_fn382_calc_iq__qd3);
        let assign32380_e29578: f64 = (assign32380_e29574 + assign32380_e29577);
        let assign32380_e29581: f64 = (4.0 * var_fn382_calc_iq__qs2);
        let assign32380_e29583: f64 = (assign32380_e29581 * var_fn382_calc_iq__qinvd0);
        let assign32380_e29584: f64 = (assign32380_e29578 + assign32380_e29583);
        let assign32380_e29587: f64 = (6.0 * var_fn382_calc_iq__qd2);
        let assign32380_e29589: f64 = (assign32380_e29587 * var_fn382_calc_iq__qinvs0);
        let assign32380_e29590: f64 = (assign32380_e29584 + assign32380_e29589);
        let assign32380_e29591: f64 = (2.0 * assign32380_e29590);
        let assign32380_e29595: f64 = (var_fn382_calc_iq__qs2 + var_fn382_calc_iq__qd2);
        let assign32380_e29598: f64 = (2.0 * var_fn382_calc_iq__qsqd);
        let assign32380_e29599: f64 = (assign32380_e29595 + assign32380_e29598);
        let assign32380_e29600: f64 = (15.0 * assign32380_e29599);
        let assign32380_e29601: f64 = (assign32380_e29591 / assign32380_e29600);
        var_fn382_calc_iq__qd1 = assign32380_e29601;
        var_fn382_calc_iq__qd1_dn4 = ((((2.0 * ((((2.0 * var_fn382_calc_iq__qs3_dn4) + (3.0 * var_fn382_calc_iq__qd3_dn4)) + (((4.0 * var_fn382_calc_iq__qs2_dn4) * var_fn382_calc_iq__qinvd0) + (assign32380_e29581 * var_fn382_calc_iq__qinvd0_dn4))) + (((6.0 * var_fn382_calc_iq__qd2_dn4) * var_fn382_calc_iq__qinvs0) + (assign32380_e29587 * var_fn382_calc_iq__qinvs0_dn4)))) * assign32380_e29600) - (assign32380_e29591 * (15.0 * ((var_fn382_calc_iq__qs2_dn4 + var_fn382_calc_iq__qd2_dn4) + (2.0 * var_fn382_calc_iq__qsqd_dn4))))) / (assign32380_e29600 * assign32380_e29600));
        var_fn382_calc_iq__qd1_dn5 = ((((2.0 * ((((2.0 * var_fn382_calc_iq__qs3_dn5) + (3.0 * var_fn382_calc_iq__qd3_dn5)) + (((4.0 * var_fn382_calc_iq__qs2_dn5) * var_fn382_calc_iq__qinvd0) + (assign32380_e29581 * var_fn382_calc_iq__qinvd0_dn5))) + (((6.0 * var_fn382_calc_iq__qd2_dn5) * var_fn382_calc_iq__qinvs0) + (assign32380_e29587 * var_fn382_calc_iq__qinvs0_dn5)))) * assign32380_e29600) - (assign32380_e29591 * (15.0 * ((var_fn382_calc_iq__qs2_dn5 + var_fn382_calc_iq__qd2_dn5) + (2.0 * var_fn382_calc_iq__qsqd_dn5))))) / (assign32380_e29600 * assign32380_e29600));
        var_fn382_calc_iq__qd1_dn8 = ((((2.0 * ((((2.0 * var_fn382_calc_iq__qs3_dn8) + (3.0 * var_fn382_calc_iq__qd3_dn8)) + (((4.0 * var_fn382_calc_iq__qs2_dn8) * var_fn382_calc_iq__qinvd0) + (assign32380_e29581 * var_fn382_calc_iq__qinvd0_dn8))) + (((6.0 * var_fn382_calc_iq__qd2_dn8) * var_fn382_calc_iq__qinvs0) + (assign32380_e29587 * var_fn382_calc_iq__qinvs0_dn8)))) * assign32380_e29600) - (assign32380_e29591 * (15.0 * ((var_fn382_calc_iq__qs2_dn8 + var_fn382_calc_iq__qd2_dn8) + (2.0 * var_fn382_calc_iq__qsqd_dn8))))) / (assign32380_e29600 * assign32380_e29600));
        var_fn382_calc_iq__qd1_dn9 = ((((2.0 * ((((2.0 * var_fn382_calc_iq__qs3_dn9) + (3.0 * var_fn382_calc_iq__qd3_dn9)) + (((4.0 * var_fn382_calc_iq__qs2_dn9) * var_fn382_calc_iq__qinvd0) + (assign32380_e29581 * var_fn382_calc_iq__qinvd0_dn9))) + (((6.0 * var_fn382_calc_iq__qd2_dn9) * var_fn382_calc_iq__qinvs0) + (assign32380_e29587 * var_fn382_calc_iq__qinvs0_dn9)))) * assign32380_e29600) - (assign32380_e29591 * (15.0 * ((var_fn382_calc_iq__qs2_dn9 + var_fn382_calc_iq__qd2_dn9) + (2.0 * var_fn382_calc_iq__qsqd_dn9))))) / (assign32380_e29600 * assign32380_e29600));

        let assign32390_e29604: f64 = (var_fn382_calc_iq__qinvdd - var_fn382_calc_iq__qd1);
        var_fn382_calc_iq__qs = assign32390_e29604;
        var_fn382_calc_iq__qs_dn4 = (var_fn382_calc_iq__qinvdd_dn4 - var_fn382_calc_iq__qd1_dn4);
        var_fn382_calc_iq__qs_dn5 = (var_fn382_calc_iq__qinvdd_dn5 - var_fn382_calc_iq__qd1_dn5);
        var_fn382_calc_iq__qs_dn8 = (var_fn382_calc_iq__qinvdd_dn8 - var_fn382_calc_iq__qd1_dn8);
        var_fn382_calc_iq__qs_dn9 = (var_fn382_calc_iq__qinvdd_dn9 - var_fn382_calc_iq__qd1_dn9);

        var_fn382_calc_iq__qd = var_fn382_calc_iq__qd1;
        var_fn382_calc_iq__qd_dn4 = var_fn382_calc_iq__qd1_dn4;
        var_fn382_calc_iq__qd_dn5 = var_fn382_calc_iq__qd1_dn5;
        var_fn382_calc_iq__qd_dn8 = var_fn382_calc_iq__qd1_dn8;
        var_fn382_calc_iq__qd_dn9 = var_fn382_calc_iq__qd1_dn9;

        let assign32410_e29608: f64 = (var_fn382_calc_iq__w * var_fn382_calc_iq__ngf);
        let assign32410_e29610: f64 = (assign32410_e29608 * var_fn382_calc_iq__lin);
        let assign32410_e29612: f64 = (assign32410_e29610 * var_fn382_calc_iq__type);
        let assign32410_e29614: f64 = (assign32410_e29612 * var_fn382_calc_iq__qs);
        let assign32410_e29616: f64 = (assign32410_e29614 * var_fn382_calc_iq__trapfracdl);
        var_fn382_calc_iq__qgsout = assign32410_e29616;
        var_fn382_calc_iq__qgsout_dn4 = ((assign32410_e29612 * var_fn382_calc_iq__qs_dn4) * var_fn382_calc_iq__trapfracdl);
        var_fn382_calc_iq__qgsout_dn5 = ((assign32410_e29612 * var_fn382_calc_iq__qs_dn5) * var_fn382_calc_iq__trapfracdl);
        var_fn382_calc_iq__qgsout_dn8 = ((assign32410_e29612 * var_fn382_calc_iq__qs_dn8) * var_fn382_calc_iq__trapfracdl);
        var_fn382_calc_iq__qgsout_dn9 = ((assign32410_e29612 * var_fn382_calc_iq__qs_dn9) * var_fn382_calc_iq__trapfracdl);
        var_fn382_calc_iq__qgsout_dn22 = (assign32410_e29614 * var_fn382_calc_iq__trapfracdl_dn22);
        var_fn382_calc_iq__qgsout_dn23 = (assign32410_e29614 * var_fn382_calc_iq__trapfracdl_dn23);
        var_fn382_calc_iq__qgsout_dn25 = (assign32410_e29614 * var_fn382_calc_iq__trapfracdl_dn25);
        var_fn382_calc_iq__qgsout_dn26 = (assign32410_e29614 * var_fn382_calc_iq__trapfracdl_dn26);

        let assign32420_e29619: f64 = (var_fn382_calc_iq__w * var_fn382_calc_iq__ngf);
        let assign32420_e29621: f64 = (assign32420_e29619 * var_fn382_calc_iq__lin);
        let assign32420_e29623: f64 = (assign32420_e29621 * var_fn382_calc_iq__type);
        let assign32420_e29625: f64 = (assign32420_e29623 * var_fn382_calc_iq__qd);
        let assign32420_e29627: f64 = (assign32420_e29625 * var_fn382_calc_iq__trapfracdl);
        var_fn382_calc_iq__qgdout = assign32420_e29627;
        var_fn382_calc_iq__qgdout_dn4 = ((assign32420_e29623 * var_fn382_calc_iq__qd_dn4) * var_fn382_calc_iq__trapfracdl);
        var_fn382_calc_iq__qgdout_dn5 = ((assign32420_e29623 * var_fn382_calc_iq__qd_dn5) * var_fn382_calc_iq__trapfracdl);
        var_fn382_calc_iq__qgdout_dn8 = ((assign32420_e29623 * var_fn382_calc_iq__qd_dn8) * var_fn382_calc_iq__trapfracdl);
        var_fn382_calc_iq__qgdout_dn9 = ((assign32420_e29623 * var_fn382_calc_iq__qd_dn9) * var_fn382_calc_iq__trapfracdl);
        var_fn382_calc_iq__qgdout_dn22 = (assign32420_e29625 * var_fn382_calc_iq__trapfracdl_dn22);
        var_fn382_calc_iq__qgdout_dn23 = (assign32420_e29625 * var_fn382_calc_iq__trapfracdl_dn23);
        var_fn382_calc_iq__qgdout_dn25 = (assign32420_e29625 * var_fn382_calc_iq__trapfracdl_dn25);
        var_fn382_calc_iq__qgdout_dn26 = (assign32420_e29625 * var_fn382_calc_iq__trapfracdl_dn26);

        let assign32430_e29630: f64 = if var_fn382_calc_iq__qcbflag == 1.0 { 1.0 } else { 0.0 };
        var_guard408 = assign32430_e29630;

        let (assign32440_e29644, assign32440_e29644_d_n4,) = {
    if (var_guard408 != 0.0) {
        let assign32440_e29636: f64 = (p.p51 * 0.5);
        let assign32440_e29638: f64 = (assign32440_e29636 * var_fn382_calc_iq__alpha_phit);
        let assign32440_e29639: f64 = (var_fn382_calc_iq__vtof - assign32440_e29638);
        let assign32440_e29640: f64 = (var_fn382_calc_iq__vcin - assign32440_e29639);
        let assign32440_e29642: f64 = (assign32440_e29640 / var_fn382_calc_iq__two_n_phit0);
        (assign32440_e29642, ((((-(var_fn382_calc_iq__vtof_dn4 - (assign32440_e29636 * var_fn382_calc_iq__alpha_phit_dn4))) * var_fn382_calc_iq__two_n_phit0) - (assign32440_e29640 * var_fn382_calc_iq__two_n_phit0_dn4)) / (var_fn382_calc_iq__two_n_phit0 * var_fn382_calc_iq__two_n_phit0)),)
    } else {
        (var_fn382_calc_iq__etac, var_fn382_calc_iq__etac_dn4,)
    }
};
        var_fn382_calc_iq__etac = assign32440_e29644;
        var_fn382_calc_iq__etac_dn4 = assign32440_e29644_d_n4;

        let assign32450_e29647: f64 = if var_fn382_calc_iq__etac > 50.0 { 1.0 } else { 0.0 };
        var_guard409 = assign32450_e29647;

        let (assign32460_e29653, assign32460_e29653_d_n4, assign32460_e29653_d_n5, assign32460_e29653_d_n8, assign32460_e29653_d_n9,) = {
    if ((var_guard408 != 0.0) && (var_guard409 != 0.0)) {
        (var_fn382_calc_iq__etac, var_fn382_calc_iq__etac_dn4, 0.0, 0.0, 0.0,)
    } else {
        (var_fn382_calc_iq__exparg, var_fn382_calc_iq__exparg_dn4, var_fn382_calc_iq__exparg_dn5, var_fn382_calc_iq__exparg_dn8, var_fn382_calc_iq__exparg_dn9,)
    }
};
        var_fn382_calc_iq__exparg = assign32460_e29653;
        var_fn382_calc_iq__exparg_dn4 = assign32460_e29653_d_n4;
        var_fn382_calc_iq__exparg_dn5 = assign32460_e29653_d_n5;
        var_fn382_calc_iq__exparg_dn8 = assign32460_e29653_d_n8;
        var_fn382_calc_iq__exparg_dn9 = assign32460_e29653_d_n9;

        let assign32470_e29656: f64 = (-50.0);
        let assign32470_e29657: f64 = if var_fn382_calc_iq__etac < assign32470_e29656 { 1.0 } else { 0.0 };
        var_guard410 = assign32470_e29657;

        let (assign32480_e29667, assign32480_e29667_d_n4, assign32480_e29667_d_n5, assign32480_e29667_d_n8, assign32480_e29667_d_n9,) = {
    if (((var_guard408 != 0.0) && (var_guard409 == 0.0)) && (var_guard410 != 0.0)) {
        let assign32480_e29665: f64 = (var_fn382_calc_iq__etac).exp();
        (assign32480_e29665, (assign32480_e29665 * var_fn382_calc_iq__etac_dn4), 0.0, 0.0, 0.0,)
    } else {
        (var_fn382_calc_iq__exparg, var_fn382_calc_iq__exparg_dn4, var_fn382_calc_iq__exparg_dn5, var_fn382_calc_iq__exparg_dn8, var_fn382_calc_iq__exparg_dn9,)
    }
};
        var_fn382_calc_iq__exparg = assign32480_e29667;
        var_fn382_calc_iq__exparg_dn4 = assign32480_e29667_d_n4;
        var_fn382_calc_iq__exparg_dn5 = assign32480_e29667_d_n5;
        var_fn382_calc_iq__exparg_dn8 = assign32480_e29667_d_n8;
        var_fn382_calc_iq__exparg_dn9 = assign32480_e29667_d_n9;

        let (assign32490_e29681, assign32490_e29681_d_n4, assign32490_e29681_d_n5, assign32490_e29681_d_n8, assign32490_e29681_d_n9,) = {
    if (((var_guard408 != 0.0) && (var_guard409 == 0.0)) && (var_guard410 == 0.0)) {
        let assign32490_e29677: f64 = (var_fn382_calc_iq__etac).exp();
        let assign32490_e29678: f64 = (1.0 + assign32490_e29677);
        let assign32490_e29679: f64 = (assign32490_e29678).ln();
        (assign32490_e29679, ((assign32490_e29677 * var_fn382_calc_iq__etac_dn4) / assign32490_e29678), 0.0, 0.0, 0.0,)
    } else {
        (var_fn382_calc_iq__exparg, var_fn382_calc_iq__exparg_dn4, var_fn382_calc_iq__exparg_dn5, var_fn382_calc_iq__exparg_dn8, var_fn382_calc_iq__exparg_dn9,)
    }
};
        var_fn382_calc_iq__exparg = assign32490_e29681;
        var_fn382_calc_iq__exparg_dn4 = assign32490_e29681_d_n4;
        var_fn382_calc_iq__exparg_dn5 = assign32490_e29681_d_n5;
        var_fn382_calc_iq__exparg_dn8 = assign32490_e29681_d_n8;
        var_fn382_calc_iq__exparg_dn9 = assign32490_e29681_d_n9;

        let (assign32510_e29711, assign32510_e29711_d_n4,) = {
    if (var_guard408 != 0.0) {
        let assign32510_e29703: f64 = (p.p51 * 0.5);
        let assign32510_e29705: f64 = (assign32510_e29703 * var_fn382_calc_iq__alpha_phit);
        let assign32510_e29706: f64 = (var_fn382_calc_iq__vtof - assign32510_e29705);
        let assign32510_e29707: f64 = (var_fn382_calc_iq__vbin - assign32510_e29706);
        let assign32510_e29709: f64 = (assign32510_e29707 / var_fn382_calc_iq__two_n_phit0);
        (assign32510_e29709, ((((-(var_fn382_calc_iq__vtof_dn4 - (assign32510_e29703 * var_fn382_calc_iq__alpha_phit_dn4))) * var_fn382_calc_iq__two_n_phit0) - (assign32510_e29707 * var_fn382_calc_iq__two_n_phit0_dn4)) / (var_fn382_calc_iq__two_n_phit0 * var_fn382_calc_iq__two_n_phit0)),)
    } else {
        (var_fn382_calc_iq__etab, var_fn382_calc_iq__etab_dn4,)
    }
};
        var_fn382_calc_iq__etab = assign32510_e29711;
        var_fn382_calc_iq__etab_dn4 = assign32510_e29711_d_n4;

        let assign32520_e29714: f64 = if var_fn382_calc_iq__etab > 50.0 { 1.0 } else { 0.0 };
        var_guard411 = assign32520_e29714;

        let (assign32530_e29720, assign32530_e29720_d_n4, assign32530_e29720_d_n5, assign32530_e29720_d_n8, assign32530_e29720_d_n9,) = {
    if ((var_guard408 != 0.0) && (var_guard411 != 0.0)) {
        (var_fn382_calc_iq__etab, var_fn382_calc_iq__etab_dn4, 0.0, 0.0, 0.0,)
    } else {
        (var_fn382_calc_iq__exparg, var_fn382_calc_iq__exparg_dn4, var_fn382_calc_iq__exparg_dn5, var_fn382_calc_iq__exparg_dn8, var_fn382_calc_iq__exparg_dn9,)
    }
};
        var_fn382_calc_iq__exparg = assign32530_e29720;
        var_fn382_calc_iq__exparg_dn4 = assign32530_e29720_d_n4;
        var_fn382_calc_iq__exparg_dn5 = assign32530_e29720_d_n5;
        var_fn382_calc_iq__exparg_dn8 = assign32530_e29720_d_n8;
        var_fn382_calc_iq__exparg_dn9 = assign32530_e29720_d_n9;

        let assign32540_e29723: f64 = (-50.0);
        let assign32540_e29724: f64 = if var_fn382_calc_iq__etab < assign32540_e29723 { 1.0 } else { 0.0 };
        var_guard412 = assign32540_e29724;

        let (assign32550_e29734, assign32550_e29734_d_n4, assign32550_e29734_d_n5, assign32550_e29734_d_n8, assign32550_e29734_d_n9,) = {
    if (((var_guard408 != 0.0) && (var_guard411 == 0.0)) && (var_guard412 != 0.0)) {
        let assign32550_e29732: f64 = (var_fn382_calc_iq__etab).exp();
        (assign32550_e29732, (assign32550_e29732 * var_fn382_calc_iq__etab_dn4), 0.0, 0.0, 0.0,)
    } else {
        (var_fn382_calc_iq__exparg, var_fn382_calc_iq__exparg_dn4, var_fn382_calc_iq__exparg_dn5, var_fn382_calc_iq__exparg_dn8, var_fn382_calc_iq__exparg_dn9,)
    }
};
        var_fn382_calc_iq__exparg = assign32550_e29734;
        var_fn382_calc_iq__exparg_dn4 = assign32550_e29734_d_n4;
        var_fn382_calc_iq__exparg_dn5 = assign32550_e29734_d_n5;
        var_fn382_calc_iq__exparg_dn8 = assign32550_e29734_d_n8;
        var_fn382_calc_iq__exparg_dn9 = assign32550_e29734_d_n9;

        let (assign32560_e29748, assign32560_e29748_d_n4, assign32560_e29748_d_n5, assign32560_e29748_d_n8, assign32560_e29748_d_n9,) = {
    if (((var_guard408 != 0.0) && (var_guard411 == 0.0)) && (var_guard412 == 0.0)) {
        let assign32560_e29744: f64 = (var_fn382_calc_iq__etab).exp();
        let assign32560_e29745: f64 = (1.0 + assign32560_e29744);
        let assign32560_e29746: f64 = (assign32560_e29745).ln();
        (assign32560_e29746, ((assign32560_e29744 * var_fn382_calc_iq__etab_dn4) / assign32560_e29745), 0.0, 0.0, 0.0,)
    } else {
        (var_fn382_calc_iq__exparg, var_fn382_calc_iq__exparg_dn4, var_fn382_calc_iq__exparg_dn5, var_fn382_calc_iq__exparg_dn8, var_fn382_calc_iq__exparg_dn9,)
    }
};
        var_fn382_calc_iq__exparg = assign32560_e29748;
        var_fn382_calc_iq__exparg_dn4 = assign32560_e29748_d_n4;
        var_fn382_calc_iq__exparg_dn5 = assign32560_e29748_d_n5;
        var_fn382_calc_iq__exparg_dn8 = assign32560_e29748_d_n8;
        var_fn382_calc_iq__exparg_dn9 = assign32560_e29748_d_n9;

        let assign32600_e29777: f64 = if var_fn382_calc_iq__qgsflag == 1.0 { 1.0 } else { 0.0 };
        var_guard413 = assign32600_e29777;

        let (assign32610_e29791, assign32610_e29791_d_n4, assign32610_e29791_d_n8, assign32610_e29791_d_n9,) = {
    if (var_guard413 != 0.0) {
        let assign32610_e29783: f64 = (p.p51 * 0.5);
        let assign32610_e29785: f64 = (assign32610_e29783 * var_fn382_calc_iq__alpha_phit);
        let assign32610_e29786: f64 = (var_fn382_calc_iq__vtof - assign32610_e29785);
        let assign32610_e29787: f64 = (var_fn382_calc_iq__vgsin - assign32610_e29786);
        let assign32610_e29789: f64 = (assign32610_e29787 / var_fn382_calc_iq__two_n_phit0);
        (assign32610_e29789, ((((-(var_fn382_calc_iq__vtof_dn4 - (assign32610_e29783 * var_fn382_calc_iq__alpha_phit_dn4))) * var_fn382_calc_iq__two_n_phit0) - (assign32610_e29787 * var_fn382_calc_iq__two_n_phit0_dn4)) / (var_fn382_calc_iq__two_n_phit0 * var_fn382_calc_iq__two_n_phit0)), (var_fn382_calc_iq__vgsin_dn8 / var_fn382_calc_iq__two_n_phit0), (var_fn382_calc_iq__vgsin_dn9 / var_fn382_calc_iq__two_n_phit0),)
    } else {
        (var_fn382_calc_iq__etags, var_fn382_calc_iq__etags_dn4, var_fn382_calc_iq__etags_dn8, var_fn382_calc_iq__etags_dn9,)
    }
};
        var_fn382_calc_iq__etags = assign32610_e29791;
        var_fn382_calc_iq__etags_dn4 = assign32610_e29791_d_n4;
        var_fn382_calc_iq__etags_dn8 = assign32610_e29791_d_n8;
        var_fn382_calc_iq__etags_dn9 = assign32610_e29791_d_n9;

        let assign32620_e29794: f64 = if var_fn382_calc_iq__etags > 50.0 { 1.0 } else { 0.0 };
        var_guard414 = assign32620_e29794;

        let (assign32630_e29800, assign32630_e29800_d_n4, assign32630_e29800_d_n5, assign32630_e29800_d_n8, assign32630_e29800_d_n9,) = {
    if ((var_guard413 != 0.0) && (var_guard414 != 0.0)) {
        (var_fn382_calc_iq__etags, var_fn382_calc_iq__etags_dn4, 0.0, var_fn382_calc_iq__etags_dn8, var_fn382_calc_iq__etags_dn9,)
    } else {
        (var_fn382_calc_iq__exparg, var_fn382_calc_iq__exparg_dn4, var_fn382_calc_iq__exparg_dn5, var_fn382_calc_iq__exparg_dn8, var_fn382_calc_iq__exparg_dn9,)
    }
};
        var_fn382_calc_iq__exparg = assign32630_e29800;
        var_fn382_calc_iq__exparg_dn4 = assign32630_e29800_d_n4;
        var_fn382_calc_iq__exparg_dn5 = assign32630_e29800_d_n5;
        var_fn382_calc_iq__exparg_dn8 = assign32630_e29800_d_n8;
        var_fn382_calc_iq__exparg_dn9 = assign32630_e29800_d_n9;

        let assign32640_e29803: f64 = (-50.0);
        let assign32640_e29804: f64 = if var_fn382_calc_iq__etags < assign32640_e29803 { 1.0 } else { 0.0 };
        var_guard415 = assign32640_e29804;

        let (assign32650_e29814, assign32650_e29814_d_n4, assign32650_e29814_d_n5, assign32650_e29814_d_n8, assign32650_e29814_d_n9,) = {
    if (((var_guard413 != 0.0) && (var_guard414 == 0.0)) && (var_guard415 != 0.0)) {
        let assign32650_e29812: f64 = (var_fn382_calc_iq__etags).exp();
        (assign32650_e29812, (assign32650_e29812 * var_fn382_calc_iq__etags_dn4), 0.0, (assign32650_e29812 * var_fn382_calc_iq__etags_dn8), (assign32650_e29812 * var_fn382_calc_iq__etags_dn9),)
    } else {
        (var_fn382_calc_iq__exparg, var_fn382_calc_iq__exparg_dn4, var_fn382_calc_iq__exparg_dn5, var_fn382_calc_iq__exparg_dn8, var_fn382_calc_iq__exparg_dn9,)
    }
};
        var_fn382_calc_iq__exparg = assign32650_e29814;
        var_fn382_calc_iq__exparg_dn4 = assign32650_e29814_d_n4;
        var_fn382_calc_iq__exparg_dn5 = assign32650_e29814_d_n5;
        var_fn382_calc_iq__exparg_dn8 = assign32650_e29814_d_n8;
        var_fn382_calc_iq__exparg_dn9 = assign32650_e29814_d_n9;

        let (assign32660_e29828, assign32660_e29828_d_n4, assign32660_e29828_d_n5, assign32660_e29828_d_n8, assign32660_e29828_d_n9,) = {
    if (((var_guard413 != 0.0) && (var_guard414 == 0.0)) && (var_guard415 == 0.0)) {
        let assign32660_e29824: f64 = (var_fn382_calc_iq__etags).exp();
        let assign32660_e29825: f64 = (1.0 + assign32660_e29824);
        let assign32660_e29826: f64 = (assign32660_e29825).ln();
        (assign32660_e29826, ((assign32660_e29824 * var_fn382_calc_iq__etags_dn4) / assign32660_e29825), 0.0, ((assign32660_e29824 * var_fn382_calc_iq__etags_dn8) / assign32660_e29825), ((assign32660_e29824 * var_fn382_calc_iq__etags_dn9) / assign32660_e29825),)
    } else {
        (var_fn382_calc_iq__exparg, var_fn382_calc_iq__exparg_dn4, var_fn382_calc_iq__exparg_dn5, var_fn382_calc_iq__exparg_dn8, var_fn382_calc_iq__exparg_dn9,)
    }
};
        var_fn382_calc_iq__exparg = assign32660_e29828;
        var_fn382_calc_iq__exparg_dn4 = assign32660_e29828_d_n4;
        var_fn382_calc_iq__exparg_dn5 = assign32660_e29828_d_n5;
        var_fn382_calc_iq__exparg_dn8 = assign32660_e29828_d_n8;
        var_fn382_calc_iq__exparg_dn9 = assign32660_e29828_d_n9;

        var_fn382_calc_iq__return = var_fn382_calc_iq__idsout;
        var_fn382_calc_iq__return_dn4 = var_fn382_calc_iq__idsout_dn4;
        var_fn382_calc_iq__return_dn5 = var_fn382_calc_iq__idsout_dn5;
        var_fn382_calc_iq__return_dn8 = var_fn382_calc_iq__idsout_dn8;
        var_fn382_calc_iq__return_dn9 = var_fn382_calc_iq__idsout_dn9;
        var_fn382_calc_iq__return_dn22 = var_fn382_calc_iq__idsout_dn22;
        var_fn382_calc_iq__return_dn23 = var_fn382_calc_iq__idsout_dn23;
        var_fn382_calc_iq__return_dn25 = var_fn382_calc_iq__idsout_dn25;
        var_fn382_calc_iq__return_dn26 = var_fn382_calc_iq__idsout_dn26;

        var_ids = var_fn382_calc_iq__idsout;
        var_ids_dn4 = var_fn382_calc_iq__idsout_dn4;
        var_ids_dn5 = var_fn382_calc_iq__idsout_dn5;
        var_ids_dn8 = var_fn382_calc_iq__idsout_dn8;
        var_ids_dn9 = var_fn382_calc_iq__idsout_dn9;
        var_ids_dn22 = var_fn382_calc_iq__idsout_dn22;
        var_ids_dn23 = var_fn382_calc_iq__idsout_dn23;
        var_ids_dn25 = var_fn382_calc_iq__idsout_dn25;
        var_ids_dn26 = var_fn382_calc_iq__idsout_dn26;

        var_qgs = var_fn382_calc_iq__qgsout;
        var_qgs_dn4 = var_fn382_calc_iq__qgsout_dn4;
        var_qgs_dn5 = var_fn382_calc_iq__qgsout_dn5;
        var_qgs_dn8 = var_fn382_calc_iq__qgsout_dn8;
        var_qgs_dn9 = var_fn382_calc_iq__qgsout_dn9;
        var_qgs_dn22 = var_fn382_calc_iq__qgsout_dn22;
        var_qgs_dn23 = var_fn382_calc_iq__qgsout_dn23;
        var_qgs_dn25 = var_fn382_calc_iq__qgsout_dn25;
        var_qgs_dn26 = var_fn382_calc_iq__qgsout_dn26;

        var_qgd = var_fn382_calc_iq__qgdout;
        var_qgd_dn4 = var_fn382_calc_iq__qgdout_dn4;
        var_qgd_dn5 = var_fn382_calc_iq__qgdout_dn5;
        var_qgd_dn8 = var_fn382_calc_iq__qgdout_dn8;
        var_qgd_dn9 = var_fn382_calc_iq__qgdout_dn9;
        var_qgd_dn22 = var_fn382_calc_iq__qgdout_dn22;
        var_qgd_dn23 = var_fn382_calc_iq__qgdout_dn23;
        var_qgd_dn25 = var_fn382_calc_iq__qgdout_dn25;
        var_qgd_dn26 = var_fn382_calc_iq__qgdout_dn26;

        var_ids = var_fn382_calc_iq__return;
        var_ids_dn4 = var_fn382_calc_iq__return_dn4;
        var_ids_dn5 = var_fn382_calc_iq__return_dn5;
        var_ids_dn8 = var_fn382_calc_iq__return_dn8;
        var_ids_dn9 = var_fn382_calc_iq__return_dn9;
        var_ids_dn22 = var_fn382_calc_iq__return_dn22;
        var_ids_dn23 = var_fn382_calc_iq__return_dn23;
        var_ids_dn25 = var_fn382_calc_iq__return_dn25;
        var_ids_dn26 = var_fn382_calc_iq__return_dn26;

        let assign32800_e29863: f64 = if p.p322 == 0.0 { 1.0 } else { 0.0 };
        var_guard416 = assign32800_e29863;

        var_igsi = 0.0;
        var_igsi_dn4 = 0.0;
        var_igsi_dn8 = 0.0;
        var_igsi_dn13 = 0.0;

        var_igdi = 0.0;
        var_igdi_dn4 = 0.0;
        var_igdi_dn8 = 0.0;
        var_igdi_dn17 = 0.0;

        var_igsi2 = 0.0;
        var_igsi2_dn4 = 0.0;
        var_igsi2_dn8 = 0.0;
        var_igsi2_dn13 = 0.0;

        var_igdi2 = 0.0;
        var_igdi2_dn4 = 0.0;
        var_igdi2_dn8 = 0.0;
        var_igdi2_dn17 = 0.0;

        var_igsidb = 0.0;
        var_igsidb_dn4 = 0.0;
        var_igsidb_dn8 = 0.0;
        var_igsidb_dn9 = 0.0;

        var_igdidb = 0.0;
        var_igdidb_dn4 = 0.0;
        var_igdidb_dn5 = 0.0;
        var_igdidb_dn8 = 0.0;

        var_igsi2db = 0.0;
        var_igsi2db_dn4 = 0.0;
        var_igsi2db_dn8 = 0.0;
        var_igsi2db_dn9 = 0.0;

        var_igdi2db = 0.0;
        var_igdi2db_dn4 = 0.0;
        var_igdi2db_dn5 = 0.0;
        var_igdi2db_dn8 = 0.0;

        let assign33050_e29890: f64 = if p.p254 == 1.0 { 1.0 } else { 0.0 };
        var_guard417 = assign33050_e29890;

        let (assign33060_e29894, assign33060_e29894_d_n4, assign33060_e29894_d_n8, assign33060_e29894_d_n13,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn418_calc_ig__return, var_fn418_calc_ig__return_dn4, var_fn418_calc_ig__return_dn8, var_fn418_calc_ig__return_dn13,)
    }
};
        var_fn418_calc_ig__return = assign33060_e29894;
        var_fn418_calc_ig__return_dn4 = assign33060_e29894_d_n4;
        var_fn418_calc_ig__return_dn8 = assign33060_e29894_d_n8;
        var_fn418_calc_ig__return_dn13 = assign33060_e29894_d_n13;

        let (assign33070_e29898, assign33070_e29898_d_n4,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn418_calc_ig__isdiodeout, var_fn418_calc_ig__isdiodeout_dn4,)
    }
};
        var_fn418_calc_ig__isdiodeout = assign33070_e29898;
        var_fn418_calc_ig__isdiodeout_dn4 = assign33070_e29898_d_n4;

        let (assign33080_e29902, assign33080_e29902_d_n4,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn418_calc_ig__isrecout, var_fn418_calc_ig__isrecout_dn4,)
    }
};
        var_fn418_calc_ig__isrecout = assign33080_e29902;
        var_fn418_calc_ig__isrecout_dn4 = assign33080_e29902_d_n4;

        let (assign33090_e29908, assign33090_e29908_d_n8, assign33090_e29908_d_n13,) = {
    if (var_guard417 != 0.0) {
        let assign33090_e29906: f64 = (p.p6 * (nv8 - nv13));
        (assign33090_e29906, p.p6, (-p.p6),)
    } else {
        (var_fn418_calc_ig__vgin, var_fn418_calc_ig__vgin_dn8, var_fn418_calc_ig__vgin_dn13,)
    }
};
        var_fn418_calc_ig__vgin = assign33090_e29908;
        var_fn418_calc_ig__vgin_dn8 = assign33090_e29908_d_n8;
        var_fn418_calc_ig__vgin_dn13 = assign33090_e29908_d_n13;

        let (assign33100_e29912, assign33100_e29912_d_n4,) = {
    if (var_guard417 != 0.0) {
        (var_phit, var_phit_dn4,)
    } else {
        (var_fn418_calc_ig__phitin, var_fn418_calc_ig__phitin_dn4,)
    }
};
        var_fn418_calc_ig__phitin = assign33100_e29912;
        var_fn418_calc_ig__phitin_dn4 = assign33100_e29912_d_n4;

        let (assign33110_e29916,) = {
    if (var_guard417 != 0.0) {
        (p.p260,)
    } else {
        (var_fn418_calc_ig__vgsatin,)
    }
};
        var_fn418_calc_ig__vgsatin = assign33110_e29916;

        *var_fn382_calc_iq__etab_slot = var_fn382_calc_iq__etab;
        *var_fn382_calc_iq__etab_dn4_slot = var_fn382_calc_iq__etab_dn4;
        *var_fn382_calc_iq__etac_slot = var_fn382_calc_iq__etac;
        *var_fn382_calc_iq__etac_dn4_slot = var_fn382_calc_iq__etac_dn4;
        *var_fn382_calc_iq__etags_slot = var_fn382_calc_iq__etags;
        *var_fn382_calc_iq__etags_dn4_slot = var_fn382_calc_iq__etags_dn4;
        *var_fn382_calc_iq__etags_dn8_slot = var_fn382_calc_iq__etags_dn8;
        *var_fn382_calc_iq__etags_dn9_slot = var_fn382_calc_iq__etags_dn9;
        *var_fn382_calc_iq__exparg_slot = var_fn382_calc_iq__exparg;
        *var_fn382_calc_iq__exparg_dn4_slot = var_fn382_calc_iq__exparg_dn4;
        *var_fn382_calc_iq__exparg_dn5_slot = var_fn382_calc_iq__exparg_dn5;
        *var_fn382_calc_iq__exparg_dn8_slot = var_fn382_calc_iq__exparg_dn8;
        *var_fn382_calc_iq__exparg_dn9_slot = var_fn382_calc_iq__exparg_dn9;
        *var_fn382_calc_iq__qd_slot = var_fn382_calc_iq__qd;
        *var_fn382_calc_iq__qd1_slot = var_fn382_calc_iq__qd1;
        *var_fn382_calc_iq__qd1_dn4_slot = var_fn382_calc_iq__qd1_dn4;
        *var_fn382_calc_iq__qd1_dn5_slot = var_fn382_calc_iq__qd1_dn5;
        *var_fn382_calc_iq__qd1_dn8_slot = var_fn382_calc_iq__qd1_dn8;
        *var_fn382_calc_iq__qd1_dn9_slot = var_fn382_calc_iq__qd1_dn9;
        *var_fn382_calc_iq__qd2_slot = var_fn382_calc_iq__qd2;
        *var_fn382_calc_iq__qd2_dn4_slot = var_fn382_calc_iq__qd2_dn4;
        *var_fn382_calc_iq__qd2_dn5_slot = var_fn382_calc_iq__qd2_dn5;
        *var_fn382_calc_iq__qd2_dn8_slot = var_fn382_calc_iq__qd2_dn8;
        *var_fn382_calc_iq__qd2_dn9_slot = var_fn382_calc_iq__qd2_dn9;
        *var_fn382_calc_iq__qd3_slot = var_fn382_calc_iq__qd3;
        *var_fn382_calc_iq__qd3_dn4_slot = var_fn382_calc_iq__qd3_dn4;
        *var_fn382_calc_iq__qd3_dn5_slot = var_fn382_calc_iq__qd3_dn5;
        *var_fn382_calc_iq__qd3_dn8_slot = var_fn382_calc_iq__qd3_dn8;
        *var_fn382_calc_iq__qd3_dn9_slot = var_fn382_calc_iq__qd3_dn9;
        *var_fn382_calc_iq__qd_dn4_slot = var_fn382_calc_iq__qd_dn4;
        *var_fn382_calc_iq__qd_dn5_slot = var_fn382_calc_iq__qd_dn5;
        *var_fn382_calc_iq__qd_dn8_slot = var_fn382_calc_iq__qd_dn8;
        *var_fn382_calc_iq__qd_dn9_slot = var_fn382_calc_iq__qd_dn9;
        *var_fn382_calc_iq__qgdout_slot = var_fn382_calc_iq__qgdout;
        *var_fn382_calc_iq__qgdout_dn22_slot = var_fn382_calc_iq__qgdout_dn22;
        *var_fn382_calc_iq__qgdout_dn23_slot = var_fn382_calc_iq__qgdout_dn23;
        *var_fn382_calc_iq__qgdout_dn25_slot = var_fn382_calc_iq__qgdout_dn25;
        *var_fn382_calc_iq__qgdout_dn26_slot = var_fn382_calc_iq__qgdout_dn26;
        *var_fn382_calc_iq__qgdout_dn4_slot = var_fn382_calc_iq__qgdout_dn4;
        *var_fn382_calc_iq__qgdout_dn5_slot = var_fn382_calc_iq__qgdout_dn5;
        *var_fn382_calc_iq__qgdout_dn8_slot = var_fn382_calc_iq__qgdout_dn8;
        *var_fn382_calc_iq__qgdout_dn9_slot = var_fn382_calc_iq__qgdout_dn9;
        *var_fn382_calc_iq__qgsout_slot = var_fn382_calc_iq__qgsout;
        *var_fn382_calc_iq__qgsout_dn22_slot = var_fn382_calc_iq__qgsout_dn22;
        *var_fn382_calc_iq__qgsout_dn23_slot = var_fn382_calc_iq__qgsout_dn23;
        *var_fn382_calc_iq__qgsout_dn25_slot = var_fn382_calc_iq__qgsout_dn25;
        *var_fn382_calc_iq__qgsout_dn26_slot = var_fn382_calc_iq__qgsout_dn26;
        *var_fn382_calc_iq__qgsout_dn4_slot = var_fn382_calc_iq__qgsout_dn4;
        *var_fn382_calc_iq__qgsout_dn5_slot = var_fn382_calc_iq__qgsout_dn5;
        *var_fn382_calc_iq__qgsout_dn8_slot = var_fn382_calc_iq__qgsout_dn8;
        *var_fn382_calc_iq__qgsout_dn9_slot = var_fn382_calc_iq__qgsout_dn9;
        *var_fn382_calc_iq__qinvd0_slot = var_fn382_calc_iq__qinvd0;
        *var_fn382_calc_iq__qinvd0_dn4_slot = var_fn382_calc_iq__qinvd0_dn4;
        *var_fn382_calc_iq__qinvd0_dn5_slot = var_fn382_calc_iq__qinvd0_dn5;
        *var_fn382_calc_iq__qinvd0_dn8_slot = var_fn382_calc_iq__qinvd0_dn8;
        *var_fn382_calc_iq__qinvd0_dn9_slot = var_fn382_calc_iq__qinvd0_dn9;
        *var_fn382_calc_iq__qinvdd_slot = var_fn382_calc_iq__qinvdd;
        *var_fn382_calc_iq__qinvdd_dn4_slot = var_fn382_calc_iq__qinvdd_dn4;
        *var_fn382_calc_iq__qinvdd_dn5_slot = var_fn382_calc_iq__qinvdd_dn5;
        *var_fn382_calc_iq__qinvdd_dn8_slot = var_fn382_calc_iq__qinvdd_dn8;
        *var_fn382_calc_iq__qinvdd_dn9_slot = var_fn382_calc_iq__qinvdd_dn9;
        *var_fn382_calc_iq__qs_slot = var_fn382_calc_iq__qs;
        *var_fn382_calc_iq__qs2_slot = var_fn382_calc_iq__qs2;
        *var_fn382_calc_iq__qs2_dn4_slot = var_fn382_calc_iq__qs2_dn4;
        *var_fn382_calc_iq__qs2_dn5_slot = var_fn382_calc_iq__qs2_dn5;
        *var_fn382_calc_iq__qs2_dn8_slot = var_fn382_calc_iq__qs2_dn8;
        *var_fn382_calc_iq__qs2_dn9_slot = var_fn382_calc_iq__qs2_dn9;
        *var_fn382_calc_iq__qs3_slot = var_fn382_calc_iq__qs3;
        *var_fn382_calc_iq__qs3_dn4_slot = var_fn382_calc_iq__qs3_dn4;
        *var_fn382_calc_iq__qs3_dn5_slot = var_fn382_calc_iq__qs3_dn5;
        *var_fn382_calc_iq__qs3_dn8_slot = var_fn382_calc_iq__qs3_dn8;
        *var_fn382_calc_iq__qs3_dn9_slot = var_fn382_calc_iq__qs3_dn9;
        *var_fn382_calc_iq__qs_dn4_slot = var_fn382_calc_iq__qs_dn4;
        *var_fn382_calc_iq__qs_dn5_slot = var_fn382_calc_iq__qs_dn5;
        *var_fn382_calc_iq__qs_dn8_slot = var_fn382_calc_iq__qs_dn8;
        *var_fn382_calc_iq__qs_dn9_slot = var_fn382_calc_iq__qs_dn9;
        *var_fn382_calc_iq__qsqd_slot = var_fn382_calc_iq__qsqd;
        *var_fn382_calc_iq__qsqd_dn4_slot = var_fn382_calc_iq__qsqd_dn4;
        *var_fn382_calc_iq__qsqd_dn5_slot = var_fn382_calc_iq__qsqd_dn5;
        *var_fn382_calc_iq__qsqd_dn8_slot = var_fn382_calc_iq__qsqd_dn8;
        *var_fn382_calc_iq__qsqd_dn9_slot = var_fn382_calc_iq__qsqd_dn9;
        *var_fn382_calc_iq__return_slot = var_fn382_calc_iq__return;
        *var_fn382_calc_iq__return_dn22_slot = var_fn382_calc_iq__return_dn22;
        *var_fn382_calc_iq__return_dn23_slot = var_fn382_calc_iq__return_dn23;
        *var_fn382_calc_iq__return_dn25_slot = var_fn382_calc_iq__return_dn25;
        *var_fn382_calc_iq__return_dn26_slot = var_fn382_calc_iq__return_dn26;
        *var_fn382_calc_iq__return_dn4_slot = var_fn382_calc_iq__return_dn4;
        *var_fn382_calc_iq__return_dn5_slot = var_fn382_calc_iq__return_dn5;
        *var_fn382_calc_iq__return_dn8_slot = var_fn382_calc_iq__return_dn8;
        *var_fn382_calc_iq__return_dn9_slot = var_fn382_calc_iq__return_dn9;
        *var_fn418_calc_ig__isdiodeout_slot = var_fn418_calc_ig__isdiodeout;
        *var_fn418_calc_ig__isdiodeout_dn4_slot = var_fn418_calc_ig__isdiodeout_dn4;
        *var_fn418_calc_ig__isrecout_slot = var_fn418_calc_ig__isrecout;
        *var_fn418_calc_ig__isrecout_dn4_slot = var_fn418_calc_ig__isrecout_dn4;
        *var_fn418_calc_ig__phitin_slot = var_fn418_calc_ig__phitin;
        *var_fn418_calc_ig__phitin_dn4_slot = var_fn418_calc_ig__phitin_dn4;
        *var_fn418_calc_ig__return_slot = var_fn418_calc_ig__return;
        *var_fn418_calc_ig__return_dn13_slot = var_fn418_calc_ig__return_dn13;
        *var_fn418_calc_ig__return_dn4_slot = var_fn418_calc_ig__return_dn4;
        *var_fn418_calc_ig__return_dn8_slot = var_fn418_calc_ig__return_dn8;
        *var_fn418_calc_ig__vgin_slot = var_fn418_calc_ig__vgin;
        *var_fn418_calc_ig__vgin_dn13_slot = var_fn418_calc_ig__vgin_dn13;
        *var_fn418_calc_ig__vgin_dn8_slot = var_fn418_calc_ig__vgin_dn8;
        *var_fn418_calc_ig__vgsatin_slot = var_fn418_calc_ig__vgsatin;
        *var_guard408_slot = var_guard408;
        *var_guard409_slot = var_guard409;
        *var_guard410_slot = var_guard410;
        *var_guard411_slot = var_guard411;
        *var_guard412_slot = var_guard412;
        *var_guard413_slot = var_guard413;
        *var_guard414_slot = var_guard414;
        *var_guard415_slot = var_guard415;
        *var_guard416_slot = var_guard416;
        *var_guard417_slot = var_guard417;
        *var_ids_slot = var_ids;
        *var_ids_dn22_slot = var_ids_dn22;
        *var_ids_dn23_slot = var_ids_dn23;
        *var_ids_dn25_slot = var_ids_dn25;
        *var_ids_dn26_slot = var_ids_dn26;
        *var_ids_dn4_slot = var_ids_dn4;
        *var_ids_dn5_slot = var_ids_dn5;
        *var_ids_dn8_slot = var_ids_dn8;
        *var_ids_dn9_slot = var_ids_dn9;
        *var_igdi_slot = var_igdi;
        *var_igdi2_slot = var_igdi2;
        *var_igdi2_dn17_slot = var_igdi2_dn17;
        *var_igdi2_dn4_slot = var_igdi2_dn4;
        *var_igdi2_dn8_slot = var_igdi2_dn8;
        *var_igdi2db_slot = var_igdi2db;
        *var_igdi2db_dn4_slot = var_igdi2db_dn4;
        *var_igdi2db_dn5_slot = var_igdi2db_dn5;
        *var_igdi2db_dn8_slot = var_igdi2db_dn8;
        *var_igdi_dn17_slot = var_igdi_dn17;
        *var_igdi_dn4_slot = var_igdi_dn4;
        *var_igdi_dn8_slot = var_igdi_dn8;
        *var_igdidb_slot = var_igdidb;
        *var_igdidb_dn4_slot = var_igdidb_dn4;
        *var_igdidb_dn5_slot = var_igdidb_dn5;
        *var_igdidb_dn8_slot = var_igdidb_dn8;
        *var_igsi_slot = var_igsi;
        *var_igsi2_slot = var_igsi2;
        *var_igsi2_dn13_slot = var_igsi2_dn13;
        *var_igsi2_dn4_slot = var_igsi2_dn4;
        *var_igsi2_dn8_slot = var_igsi2_dn8;
        *var_igsi2db_slot = var_igsi2db;
        *var_igsi2db_dn4_slot = var_igsi2db_dn4;
        *var_igsi2db_dn8_slot = var_igsi2db_dn8;
        *var_igsi2db_dn9_slot = var_igsi2db_dn9;
        *var_igsi_dn13_slot = var_igsi_dn13;
        *var_igsi_dn4_slot = var_igsi_dn4;
        *var_igsi_dn8_slot = var_igsi_dn8;
        *var_igsidb_slot = var_igsidb;
        *var_igsidb_dn4_slot = var_igsidb_dn4;
        *var_igsidb_dn8_slot = var_igsidb_dn8;
        *var_igsidb_dn9_slot = var_igsidb_dn9;
        *var_qgd_slot = var_qgd;
        *var_qgd_dn22_slot = var_qgd_dn22;
        *var_qgd_dn23_slot = var_qgd_dn23;
        *var_qgd_dn25_slot = var_qgd_dn25;
        *var_qgd_dn26_slot = var_qgd_dn26;
        *var_qgd_dn4_slot = var_qgd_dn4;
        *var_qgd_dn5_slot = var_qgd_dn5;
        *var_qgd_dn8_slot = var_qgd_dn8;
        *var_qgd_dn9_slot = var_qgd_dn9;
        *var_qgs_slot = var_qgs;
        *var_qgs_dn22_slot = var_qgs_dn22;
        *var_qgs_dn23_slot = var_qgs_dn23;
        *var_qgs_dn25_slot = var_qgs_dn25;
        *var_qgs_dn26_slot = var_qgs_dn26;
        *var_qgs_dn4_slot = var_qgs_dn4;
        *var_qgs_dn5_slot = var_qgs_dn5;
        *var_qgs_dn8_slot = var_qgs_dn8;
        *var_qgs_dn9_slot = var_qgs_dn9;
    }

    pub(super) fn stamp_transient_block_81(
        p: &Parameters,
        var_guard417: f64,
        var_tfacdiode: f64,
        var_tfacdiode_dn4: f64,
        var_fn418_calc_ig__alpha2_phit_slot: &mut f64,
        var_fn418_calc_ig__alpha2_phit_dn4_slot: &mut f64,
        var_fn418_calc_ig__alphagin_slot: &mut f64,
        var_fn418_calc_ig__betarecin_slot: &mut f64,
        var_fn418_calc_ig__expbd1_slot: &mut f64,
        var_fn418_calc_ig__expbd1_dn13_slot: &mut f64,
        var_fn418_calc_ig__expbd1_dn4_slot: &mut f64,
        var_fn418_calc_ig__expbd1_dn8_slot: &mut f64,
        var_fn418_calc_ig__expbd1_vgsat_slot: &mut f64,
        var_fn418_calc_ig__expbd1_vgsat_dn4_slot: &mut f64,
        var_fn418_calc_ig__expbd2_slot: &mut f64,
        var_fn418_calc_ig__expbd2_dn4_slot: &mut f64,
        var_fn418_calc_ig__expbdarg1_slot: &mut f64,
        var_fn418_calc_ig__expbdarg1_dn13_slot: &mut f64,
        var_fn418_calc_ig__expbdarg1_dn4_slot: &mut f64,
        var_fn418_calc_ig__expbdarg1_dn8_slot: &mut f64,
        var_fn418_calc_ig__expbdarg1_vgsat_slot: &mut f64,
        var_fn418_calc_ig__expbdarg1_vgsat_dn4_slot: &mut f64,
        var_fn418_calc_ig__expbdarg2_slot: &mut f64,
        var_fn418_calc_ig__expbdarg2_dn4_slot: &mut f64,
        var_fn418_calc_ig__expffvarg_slot: &mut f64,
        var_fn418_calc_ig__expffvarg_dn13_slot: &mut f64,
        var_fn418_calc_ig__expffvarg_dn4_slot: &mut f64,
        var_fn418_calc_ig__expffvarg_dn8_slot: &mut f64,
        var_fn418_calc_ig__expifor_slot: &mut f64,
        var_fn418_calc_ig__expifor_dn13_slot: &mut f64,
        var_fn418_calc_ig__expifor_dn4_slot: &mut f64,
        var_fn418_calc_ig__expifor_dn8_slot: &mut f64,
        var_fn418_calc_ig__expifor_hinj_slot: &mut f64,
        var_fn418_calc_ig__expifor_hinj_dn13_slot: &mut f64,
        var_fn418_calc_ig__expifor_hinj_dn4_slot: &mut f64,
        var_fn418_calc_ig__expifor_hinj_dn8_slot: &mut f64,
        var_fn418_calc_ig__expifor_hinj_vgsat_slot: &mut f64,
        var_fn418_calc_ig__expifor_hinj_vgsat_dn4_slot: &mut f64,
        var_fn418_calc_ig__expifor_nohinj_vgsat_slot: &mut f64,
        var_fn418_calc_ig__expifor_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn418_calc_ig__expiforarg_slot: &mut f64,
        var_fn418_calc_ig__expiforarg_dn13_slot: &mut f64,
        var_fn418_calc_ig__expiforarg_dn4_slot: &mut f64,
        var_fn418_calc_ig__expiforarg_dn8_slot: &mut f64,
        var_fn418_calc_ig__expiforarg_hinj_slot: &mut f64,
        var_fn418_calc_ig__expiforarg_hinj_dn13_slot: &mut f64,
        var_fn418_calc_ig__expiforarg_hinj_dn4_slot: &mut f64,
        var_fn418_calc_ig__expiforarg_hinj_dn8_slot: &mut f64,
        var_fn418_calc_ig__expiforarg_hinj_vgsat_slot: &mut f64,
        var_fn418_calc_ig__expiforarg_hinj_vgsat_dn4_slot: &mut f64,
        var_fn418_calc_ig__expiforarg_nohinj_vgsat_slot: &mut f64,
        var_fn418_calc_ig__expiforarg_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn418_calc_ig__expirev_slot: &mut f64,
        var_fn418_calc_ig__expirev_dn13_slot: &mut f64,
        var_fn418_calc_ig__expirev_dn4_slot: &mut f64,
        var_fn418_calc_ig__expirev_dn8_slot: &mut f64,
        var_fn418_calc_ig__expirevarg_slot: &mut f64,
        var_fn418_calc_ig__expirevarg_dn13_slot: &mut f64,
        var_fn418_calc_ig__expirevarg_dn4_slot: &mut f64,
        var_fn418_calc_ig__expirevarg_dn8_slot: &mut f64,
        var_fn418_calc_ig__expphib_slot: &mut f64,
        var_fn418_calc_ig__expphib_dn4_slot: &mut f64,
        var_fn418_calc_ig__ffvgin_slot: &mut f64,
        var_fn418_calc_ig__ffvgin_dn13_slot: &mut f64,
        var_fn418_calc_ig__ffvgin_dn4_slot: &mut f64,
        var_fn418_calc_ig__ffvgin_dn8_slot: &mut f64,
        var_fn418_calc_ig__fracin_slot: &mut f64,
        var_fn418_calc_ig__frecgin_slot: &mut f64,
        var_fn418_calc_ig__frecgin_dn13_slot: &mut f64,
        var_fn418_calc_ig__frecgin_dn8_slot: &mut f64,
        var_fn418_calc_ig__iginbd_slot: &mut f64,
        var_fn418_calc_ig__iginbd_dn13_slot: &mut f64,
        var_fn418_calc_ig__iginbd_dn4_slot: &mut f64,
        var_fn418_calc_ig__iginbd_dn8_slot: &mut f64,
        var_fn418_calc_ig__iginbd_vgsat_slot: &mut f64,
        var_fn418_calc_ig__iginbd_vgsat_dn4_slot: &mut f64,
        var_fn418_calc_ig__igindiode_slot: &mut f64,
        var_fn418_calc_ig__igindiode_dn13_slot: &mut f64,
        var_fn418_calc_ig__igindiode_dn4_slot: &mut f64,
        var_fn418_calc_ig__igindiode_dn8_slot: &mut f64,
        var_fn418_calc_ig__igindiode_hinj_pre_slot: &mut f64,
        var_fn418_calc_ig__igindiode_hinj_pre_dn4_slot: &mut f64,
        var_fn418_calc_ig__igindiode_hinj_vgsat_slot: &mut f64,
        var_fn418_calc_ig__igindiode_hinj_vgsat_dn4_slot: &mut f64,
        var_fn418_calc_ig__igindiode_nohinj_slot: &mut f64,
        var_fn418_calc_ig__igindiode_nohinj_dn13_slot: &mut f64,
        var_fn418_calc_ig__igindiode_nohinj_dn4_slot: &mut f64,
        var_fn418_calc_ig__igindiode_nohinj_dn8_slot: &mut f64,
        var_fn418_calc_ig__igindiode_nohinj_vgsat_slot: &mut f64,
        var_fn418_calc_ig__igindiode_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn418_calc_ig__iginrec_slot: &mut f64,
        var_fn418_calc_ig__iginrec_dn13_slot: &mut f64,
        var_fn418_calc_ig__iginrec_dn4_slot: &mut f64,
        var_fn418_calc_ig__iginrec_dn8_slot: &mut f64,
        var_fn418_calc_ig__igout_slot: &mut f64,
        var_fn418_calc_ig__igout_dn13_slot: &mut f64,
        var_fn418_calc_ig__igout_dn4_slot: &mut f64,
        var_fn418_calc_ig__igout_dn8_slot: &mut f64,
        var_fn418_calc_ig__ijin_slot: &mut f64,
        var_fn418_calc_ig__irecin_slot: &mut f64,
        var_fn418_calc_ig__kbdgatein_slot: &mut f64,
        var_fn418_calc_ig__ngf_slot: &mut f64,
        var_fn418_calc_ig__pbdgin_slot: &mut f64,
        var_fn418_calc_ig__pg_param1_slot: &mut f64,
        var_fn418_calc_ig__pg_paramin_slot: &mut f64,
        var_fn418_calc_ig__pg_paramin_hinj_slot: &mut f64,
        var_fn418_calc_ig__pgsrecin_slot: &mut f64,
        var_fn418_calc_ig__t0_slot: &mut f64,
        var_fn418_calc_ig__t0_dn4_slot: &mut f64,
        var_fn418_calc_ig__tfacdiodein_slot: &mut f64,
        var_fn418_calc_ig__tfacdiodein_dn4_slot: &mut f64,
        var_fn418_calc_ig__type_slot: &mut f64,
        var_fn418_calc_ig__vbdgin_slot: &mut f64,
        var_fn418_calc_ig__vgsatqin_slot: &mut f64,
        var_fn418_calc_ig__vjg_slot: &mut f64,
        var_fn418_calc_ig__w_slot: &mut f64,
    ) {
        let mut var_fn418_calc_ig__alpha2_phit: f64 = *var_fn418_calc_ig__alpha2_phit_slot;
        let mut var_fn418_calc_ig__alpha2_phit_dn4: f64 = *var_fn418_calc_ig__alpha2_phit_dn4_slot;
        let mut var_fn418_calc_ig__alphagin: f64 = *var_fn418_calc_ig__alphagin_slot;
        let mut var_fn418_calc_ig__betarecin: f64 = *var_fn418_calc_ig__betarecin_slot;
        let mut var_fn418_calc_ig__expbd1: f64 = *var_fn418_calc_ig__expbd1_slot;
        let mut var_fn418_calc_ig__expbd1_dn13: f64 = *var_fn418_calc_ig__expbd1_dn13_slot;
        let mut var_fn418_calc_ig__expbd1_dn4: f64 = *var_fn418_calc_ig__expbd1_dn4_slot;
        let mut var_fn418_calc_ig__expbd1_dn8: f64 = *var_fn418_calc_ig__expbd1_dn8_slot;
        let mut var_fn418_calc_ig__expbd1_vgsat: f64 = *var_fn418_calc_ig__expbd1_vgsat_slot;
        let mut var_fn418_calc_ig__expbd1_vgsat_dn4: f64 = *var_fn418_calc_ig__expbd1_vgsat_dn4_slot;
        let mut var_fn418_calc_ig__expbd2: f64 = *var_fn418_calc_ig__expbd2_slot;
        let mut var_fn418_calc_ig__expbd2_dn4: f64 = *var_fn418_calc_ig__expbd2_dn4_slot;
        let mut var_fn418_calc_ig__expbdarg1: f64 = *var_fn418_calc_ig__expbdarg1_slot;
        let mut var_fn418_calc_ig__expbdarg1_dn13: f64 = *var_fn418_calc_ig__expbdarg1_dn13_slot;
        let mut var_fn418_calc_ig__expbdarg1_dn4: f64 = *var_fn418_calc_ig__expbdarg1_dn4_slot;
        let mut var_fn418_calc_ig__expbdarg1_dn8: f64 = *var_fn418_calc_ig__expbdarg1_dn8_slot;
        let mut var_fn418_calc_ig__expbdarg1_vgsat: f64 = *var_fn418_calc_ig__expbdarg1_vgsat_slot;
        let mut var_fn418_calc_ig__expbdarg1_vgsat_dn4: f64 = *var_fn418_calc_ig__expbdarg1_vgsat_dn4_slot;
        let mut var_fn418_calc_ig__expbdarg2: f64 = *var_fn418_calc_ig__expbdarg2_slot;
        let mut var_fn418_calc_ig__expbdarg2_dn4: f64 = *var_fn418_calc_ig__expbdarg2_dn4_slot;
        let mut var_fn418_calc_ig__expffvarg: f64 = *var_fn418_calc_ig__expffvarg_slot;
        let mut var_fn418_calc_ig__expffvarg_dn13: f64 = *var_fn418_calc_ig__expffvarg_dn13_slot;
        let mut var_fn418_calc_ig__expffvarg_dn4: f64 = *var_fn418_calc_ig__expffvarg_dn4_slot;
        let mut var_fn418_calc_ig__expffvarg_dn8: f64 = *var_fn418_calc_ig__expffvarg_dn8_slot;
        let mut var_fn418_calc_ig__expifor: f64 = *var_fn418_calc_ig__expifor_slot;
        let mut var_fn418_calc_ig__expifor_dn13: f64 = *var_fn418_calc_ig__expifor_dn13_slot;
        let mut var_fn418_calc_ig__expifor_dn4: f64 = *var_fn418_calc_ig__expifor_dn4_slot;
        let mut var_fn418_calc_ig__expifor_dn8: f64 = *var_fn418_calc_ig__expifor_dn8_slot;
        let mut var_fn418_calc_ig__expifor_hinj: f64 = *var_fn418_calc_ig__expifor_hinj_slot;
        let mut var_fn418_calc_ig__expifor_hinj_dn13: f64 = *var_fn418_calc_ig__expifor_hinj_dn13_slot;
        let mut var_fn418_calc_ig__expifor_hinj_dn4: f64 = *var_fn418_calc_ig__expifor_hinj_dn4_slot;
        let mut var_fn418_calc_ig__expifor_hinj_dn8: f64 = *var_fn418_calc_ig__expifor_hinj_dn8_slot;
        let mut var_fn418_calc_ig__expifor_hinj_vgsat: f64 = *var_fn418_calc_ig__expifor_hinj_vgsat_slot;
        let mut var_fn418_calc_ig__expifor_hinj_vgsat_dn4: f64 = *var_fn418_calc_ig__expifor_hinj_vgsat_dn4_slot;
        let mut var_fn418_calc_ig__expifor_nohinj_vgsat: f64 = *var_fn418_calc_ig__expifor_nohinj_vgsat_slot;
        let mut var_fn418_calc_ig__expifor_nohinj_vgsat_dn4: f64 = *var_fn418_calc_ig__expifor_nohinj_vgsat_dn4_slot;
        let mut var_fn418_calc_ig__expiforarg: f64 = *var_fn418_calc_ig__expiforarg_slot;
        let mut var_fn418_calc_ig__expiforarg_dn13: f64 = *var_fn418_calc_ig__expiforarg_dn13_slot;
        let mut var_fn418_calc_ig__expiforarg_dn4: f64 = *var_fn418_calc_ig__expiforarg_dn4_slot;
        let mut var_fn418_calc_ig__expiforarg_dn8: f64 = *var_fn418_calc_ig__expiforarg_dn8_slot;
        let mut var_fn418_calc_ig__expiforarg_hinj: f64 = *var_fn418_calc_ig__expiforarg_hinj_slot;
        let mut var_fn418_calc_ig__expiforarg_hinj_dn13: f64 = *var_fn418_calc_ig__expiforarg_hinj_dn13_slot;
        let mut var_fn418_calc_ig__expiforarg_hinj_dn4: f64 = *var_fn418_calc_ig__expiforarg_hinj_dn4_slot;
        let mut var_fn418_calc_ig__expiforarg_hinj_dn8: f64 = *var_fn418_calc_ig__expiforarg_hinj_dn8_slot;
        let mut var_fn418_calc_ig__expiforarg_hinj_vgsat: f64 = *var_fn418_calc_ig__expiforarg_hinj_vgsat_slot;
        let mut var_fn418_calc_ig__expiforarg_hinj_vgsat_dn4: f64 = *var_fn418_calc_ig__expiforarg_hinj_vgsat_dn4_slot;
        let mut var_fn418_calc_ig__expiforarg_nohinj_vgsat: f64 = *var_fn418_calc_ig__expiforarg_nohinj_vgsat_slot;
        let mut var_fn418_calc_ig__expiforarg_nohinj_vgsat_dn4: f64 = *var_fn418_calc_ig__expiforarg_nohinj_vgsat_dn4_slot;
        let mut var_fn418_calc_ig__expirev: f64 = *var_fn418_calc_ig__expirev_slot;
        let mut var_fn418_calc_ig__expirev_dn13: f64 = *var_fn418_calc_ig__expirev_dn13_slot;
        let mut var_fn418_calc_ig__expirev_dn4: f64 = *var_fn418_calc_ig__expirev_dn4_slot;
        let mut var_fn418_calc_ig__expirev_dn8: f64 = *var_fn418_calc_ig__expirev_dn8_slot;
        let mut var_fn418_calc_ig__expirevarg: f64 = *var_fn418_calc_ig__expirevarg_slot;
        let mut var_fn418_calc_ig__expirevarg_dn13: f64 = *var_fn418_calc_ig__expirevarg_dn13_slot;
        let mut var_fn418_calc_ig__expirevarg_dn4: f64 = *var_fn418_calc_ig__expirevarg_dn4_slot;
        let mut var_fn418_calc_ig__expirevarg_dn8: f64 = *var_fn418_calc_ig__expirevarg_dn8_slot;
        let mut var_fn418_calc_ig__expphib: f64 = *var_fn418_calc_ig__expphib_slot;
        let mut var_fn418_calc_ig__expphib_dn4: f64 = *var_fn418_calc_ig__expphib_dn4_slot;
        let mut var_fn418_calc_ig__ffvgin: f64 = *var_fn418_calc_ig__ffvgin_slot;
        let mut var_fn418_calc_ig__ffvgin_dn13: f64 = *var_fn418_calc_ig__ffvgin_dn13_slot;
        let mut var_fn418_calc_ig__ffvgin_dn4: f64 = *var_fn418_calc_ig__ffvgin_dn4_slot;
        let mut var_fn418_calc_ig__ffvgin_dn8: f64 = *var_fn418_calc_ig__ffvgin_dn8_slot;
        let mut var_fn418_calc_ig__fracin: f64 = *var_fn418_calc_ig__fracin_slot;
        let mut var_fn418_calc_ig__frecgin: f64 = *var_fn418_calc_ig__frecgin_slot;
        let mut var_fn418_calc_ig__frecgin_dn13: f64 = *var_fn418_calc_ig__frecgin_dn13_slot;
        let mut var_fn418_calc_ig__frecgin_dn8: f64 = *var_fn418_calc_ig__frecgin_dn8_slot;
        let mut var_fn418_calc_ig__iginbd: f64 = *var_fn418_calc_ig__iginbd_slot;
        let mut var_fn418_calc_ig__iginbd_dn13: f64 = *var_fn418_calc_ig__iginbd_dn13_slot;
        let mut var_fn418_calc_ig__iginbd_dn4: f64 = *var_fn418_calc_ig__iginbd_dn4_slot;
        let mut var_fn418_calc_ig__iginbd_dn8: f64 = *var_fn418_calc_ig__iginbd_dn8_slot;
        let mut var_fn418_calc_ig__iginbd_vgsat: f64 = *var_fn418_calc_ig__iginbd_vgsat_slot;
        let mut var_fn418_calc_ig__iginbd_vgsat_dn4: f64 = *var_fn418_calc_ig__iginbd_vgsat_dn4_slot;
        let mut var_fn418_calc_ig__igindiode: f64 = *var_fn418_calc_ig__igindiode_slot;
        let mut var_fn418_calc_ig__igindiode_dn13: f64 = *var_fn418_calc_ig__igindiode_dn13_slot;
        let mut var_fn418_calc_ig__igindiode_dn4: f64 = *var_fn418_calc_ig__igindiode_dn4_slot;
        let mut var_fn418_calc_ig__igindiode_dn8: f64 = *var_fn418_calc_ig__igindiode_dn8_slot;
        let mut var_fn418_calc_ig__igindiode_hinj_pre: f64 = *var_fn418_calc_ig__igindiode_hinj_pre_slot;
        let mut var_fn418_calc_ig__igindiode_hinj_pre_dn4: f64 = *var_fn418_calc_ig__igindiode_hinj_pre_dn4_slot;
        let mut var_fn418_calc_ig__igindiode_hinj_vgsat: f64 = *var_fn418_calc_ig__igindiode_hinj_vgsat_slot;
        let mut var_fn418_calc_ig__igindiode_hinj_vgsat_dn4: f64 = *var_fn418_calc_ig__igindiode_hinj_vgsat_dn4_slot;
        let mut var_fn418_calc_ig__igindiode_nohinj: f64 = *var_fn418_calc_ig__igindiode_nohinj_slot;
        let mut var_fn418_calc_ig__igindiode_nohinj_dn13: f64 = *var_fn418_calc_ig__igindiode_nohinj_dn13_slot;
        let mut var_fn418_calc_ig__igindiode_nohinj_dn4: f64 = *var_fn418_calc_ig__igindiode_nohinj_dn4_slot;
        let mut var_fn418_calc_ig__igindiode_nohinj_dn8: f64 = *var_fn418_calc_ig__igindiode_nohinj_dn8_slot;
        let mut var_fn418_calc_ig__igindiode_nohinj_vgsat: f64 = *var_fn418_calc_ig__igindiode_nohinj_vgsat_slot;
        let mut var_fn418_calc_ig__igindiode_nohinj_vgsat_dn4: f64 = *var_fn418_calc_ig__igindiode_nohinj_vgsat_dn4_slot;
        let mut var_fn418_calc_ig__iginrec: f64 = *var_fn418_calc_ig__iginrec_slot;
        let mut var_fn418_calc_ig__iginrec_dn13: f64 = *var_fn418_calc_ig__iginrec_dn13_slot;
        let mut var_fn418_calc_ig__iginrec_dn4: f64 = *var_fn418_calc_ig__iginrec_dn4_slot;
        let mut var_fn418_calc_ig__iginrec_dn8: f64 = *var_fn418_calc_ig__iginrec_dn8_slot;
        let mut var_fn418_calc_ig__igout: f64 = *var_fn418_calc_ig__igout_slot;
        let mut var_fn418_calc_ig__igout_dn13: f64 = *var_fn418_calc_ig__igout_dn13_slot;
        let mut var_fn418_calc_ig__igout_dn4: f64 = *var_fn418_calc_ig__igout_dn4_slot;
        let mut var_fn418_calc_ig__igout_dn8: f64 = *var_fn418_calc_ig__igout_dn8_slot;
        let mut var_fn418_calc_ig__ijin: f64 = *var_fn418_calc_ig__ijin_slot;
        let mut var_fn418_calc_ig__irecin: f64 = *var_fn418_calc_ig__irecin_slot;
        let mut var_fn418_calc_ig__kbdgatein: f64 = *var_fn418_calc_ig__kbdgatein_slot;
        let mut var_fn418_calc_ig__ngf: f64 = *var_fn418_calc_ig__ngf_slot;
        let mut var_fn418_calc_ig__pbdgin: f64 = *var_fn418_calc_ig__pbdgin_slot;
        let mut var_fn418_calc_ig__pg_param1: f64 = *var_fn418_calc_ig__pg_param1_slot;
        let mut var_fn418_calc_ig__pg_paramin: f64 = *var_fn418_calc_ig__pg_paramin_slot;
        let mut var_fn418_calc_ig__pg_paramin_hinj: f64 = *var_fn418_calc_ig__pg_paramin_hinj_slot;
        let mut var_fn418_calc_ig__pgsrecin: f64 = *var_fn418_calc_ig__pgsrecin_slot;
        let mut var_fn418_calc_ig__t0: f64 = *var_fn418_calc_ig__t0_slot;
        let mut var_fn418_calc_ig__t0_dn4: f64 = *var_fn418_calc_ig__t0_dn4_slot;
        let mut var_fn418_calc_ig__tfacdiodein: f64 = *var_fn418_calc_ig__tfacdiodein_slot;
        let mut var_fn418_calc_ig__tfacdiodein_dn4: f64 = *var_fn418_calc_ig__tfacdiodein_dn4_slot;
        let mut var_fn418_calc_ig__type: f64 = *var_fn418_calc_ig__type_slot;
        let mut var_fn418_calc_ig__vbdgin: f64 = *var_fn418_calc_ig__vbdgin_slot;
        let mut var_fn418_calc_ig__vgsatqin: f64 = *var_fn418_calc_ig__vgsatqin_slot;
        let mut var_fn418_calc_ig__vjg: f64 = *var_fn418_calc_ig__vjg_slot;
        let mut var_fn418_calc_ig__w: f64 = *var_fn418_calc_ig__w_slot;

        let (assign33120_e29920,) = {
    if (var_guard417 != 0.0) {
        (p.p262,)
    } else {
        (var_fn418_calc_ig__alphagin,)
    }
};
        var_fn418_calc_ig__alphagin = assign33120_e29920;

        let (assign33130_e29924,) = {
    if (var_guard417 != 0.0) {
        (p.p261,)
    } else {
        (var_fn418_calc_ig__fracin,)
    }
};
        var_fn418_calc_ig__fracin = assign33130_e29924;

        let (assign33140_e29928,) = {
    if (var_guard417 != 0.0) {
        (p.p258,)
    } else {
        (var_fn418_calc_ig__pg_paramin,)
    }
};
        var_fn418_calc_ig__pg_paramin = assign33140_e29928;

        let (assign33150_e29932,) = {
    if (var_guard417 != 0.0) {
        (p.p278,)
    } else {
        (var_fn418_calc_ig__pbdgin,)
    }
};
        var_fn418_calc_ig__pbdgin = assign33150_e29932;

        let (assign33160_e29936,) = {
    if (var_guard417 != 0.0) {
        (p.p277,)
    } else {
        (var_fn418_calc_ig__vbdgin,)
    }
};
        var_fn418_calc_ig__vbdgin = assign33160_e29936;

        let (assign33170_e29940, assign33170_e29940_d_n4,) = {
    if (var_guard417 != 0.0) {
        (var_tfacdiode, var_tfacdiode_dn4,)
    } else {
        (var_fn418_calc_ig__tfacdiodein, var_fn418_calc_ig__tfacdiodein_dn4,)
    }
};
        var_fn418_calc_ig__tfacdiodein = assign33170_e29940;
        var_fn418_calc_ig__tfacdiodein_dn4 = assign33170_e29940_d_n4;

        let (assign33180_e29944,) = {
    if (var_guard417 != 0.0) {
        (p.p0,)
    } else {
        (var_fn418_calc_ig__w,)
    }
};
        var_fn418_calc_ig__w = assign33180_e29944;

        let (assign33190_e29948,) = {
    if (var_guard417 != 0.0) {
        (p.p2,)
    } else {
        (var_fn418_calc_ig__ngf,)
    }
};
        var_fn418_calc_ig__ngf = assign33190_e29948;

        let (assign33200_e29956,) = {
    if (var_guard417 != 0.0) {
        let assign33200_e29952: f64 = (1.0 - p.p255);
        let assign33200_e29954: f64 = (assign33200_e29952 * p.p259);
        (assign33200_e29954,)
    } else {
        (var_fn418_calc_ig__ijin,)
    }
};
        var_fn418_calc_ig__ijin = assign33200_e29956;

        let (assign33210_e29960,) = {
    if (var_guard417 != 0.0) {
        (p.p276,)
    } else {
        (var_fn418_calc_ig__kbdgatein,)
    }
};
        var_fn418_calc_ig__kbdgatein = assign33210_e29960;

        let (assign33220_e29964,) = {
    if (var_guard417 != 0.0) {
        (p.p270,)
    } else {
        (var_fn418_calc_ig__vgsatqin,)
    }
};
        var_fn418_calc_ig__vgsatqin = assign33220_e29964;

        let (assign33230_e29968,) = {
    if (var_guard417 != 0.0) {
        (p.p271,)
    } else {
        (var_fn418_calc_ig__betarecin,)
    }
};
        var_fn418_calc_ig__betarecin = assign33230_e29968;

        let (assign33240_e29976,) = {
    if (var_guard417 != 0.0) {
        let assign33240_e29972: f64 = (1.0 - p.p255);
        let assign33240_e29974: f64 = (assign33240_e29972 * p.p269);
        (assign33240_e29974,)
    } else {
        (var_fn418_calc_ig__irecin,)
    }
};
        var_fn418_calc_ig__irecin = assign33240_e29976;

        let (assign33250_e29980,) = {
    if (var_guard417 != 0.0) {
        (p.p268,)
    } else {
        (var_fn418_calc_ig__pgsrecin,)
    }
};
        var_fn418_calc_ig__pgsrecin = assign33250_e29980;

        let (assign33260_e29984,) = {
    if (var_guard417 != 0.0) {
        (p.p257,)
    } else {
        (var_fn418_calc_ig__pg_param1,)
    }
};
        var_fn418_calc_ig__pg_param1 = assign33260_e29984;

        let (assign33270_e29988,) = {
    if (var_guard417 != 0.0) {
        (p.p256,)
    } else {
        (var_fn418_calc_ig__vjg,)
    }
};
        var_fn418_calc_ig__vjg = assign33270_e29988;

        let (assign33280_e29992,) = {
    if (var_guard417 != 0.0) {
        (p.p6,)
    } else {
        (var_fn418_calc_ig__type,)
    }
};
        var_fn418_calc_ig__type = assign33280_e29992;

        let (assign33290_e29996, assign33290_e29996_d_n4, assign33290_e29996_d_n8, assign33290_e29996_d_n13,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn418_calc_ig__igout, var_fn418_calc_ig__igout_dn4, var_fn418_calc_ig__igout_dn8, var_fn418_calc_ig__igout_dn13,)
    }
};
        var_fn418_calc_ig__igout = assign33290_e29996;
        var_fn418_calc_ig__igout_dn4 = assign33290_e29996_d_n4;
        var_fn418_calc_ig__igout_dn8 = assign33290_e29996_d_n8;
        var_fn418_calc_ig__igout_dn13 = assign33290_e29996_d_n13;

        let (assign33300_e30000, assign33300_e30000_d_n4,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn418_calc_ig__alpha2_phit, var_fn418_calc_ig__alpha2_phit_dn4,)
    }
};
        var_fn418_calc_ig__alpha2_phit = assign33300_e30000;
        var_fn418_calc_ig__alpha2_phit_dn4 = assign33300_e30000_d_n4;

        let (assign33310_e30004, assign33310_e30004_d_n4,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn418_calc_ig__t0, var_fn418_calc_ig__t0_dn4,)
    }
};
        var_fn418_calc_ig__t0 = assign33310_e30004;
        var_fn418_calc_ig__t0_dn4 = assign33310_e30004_d_n4;

        let (assign33320_e30008, assign33320_e30008_d_n4, assign33320_e30008_d_n8, assign33320_e30008_d_n13,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn418_calc_ig__ffvgin, var_fn418_calc_ig__ffvgin_dn4, var_fn418_calc_ig__ffvgin_dn8, var_fn418_calc_ig__ffvgin_dn13,)
    }
};
        var_fn418_calc_ig__ffvgin = assign33320_e30008;
        var_fn418_calc_ig__ffvgin_dn4 = assign33320_e30008_d_n4;
        var_fn418_calc_ig__ffvgin_dn8 = assign33320_e30008_d_n8;
        var_fn418_calc_ig__ffvgin_dn13 = assign33320_e30008_d_n13;

        let (assign33330_e30012, assign33330_e30012_d_n4, assign33330_e30012_d_n8, assign33330_e30012_d_n13,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn418_calc_ig__iginbd, var_fn418_calc_ig__iginbd_dn4, var_fn418_calc_ig__iginbd_dn8, var_fn418_calc_ig__iginbd_dn13,)
    }
};
        var_fn418_calc_ig__iginbd = assign33330_e30012;
        var_fn418_calc_ig__iginbd_dn4 = assign33330_e30012_d_n4;
        var_fn418_calc_ig__iginbd_dn8 = assign33330_e30012_d_n8;
        var_fn418_calc_ig__iginbd_dn13 = assign33330_e30012_d_n13;

        let (assign33340_e30016, assign33340_e30016_d_n4, assign33340_e30016_d_n8, assign33340_e30016_d_n13,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn418_calc_ig__igindiode, var_fn418_calc_ig__igindiode_dn4, var_fn418_calc_ig__igindiode_dn8, var_fn418_calc_ig__igindiode_dn13,)
    }
};
        var_fn418_calc_ig__igindiode = assign33340_e30016;
        var_fn418_calc_ig__igindiode_dn4 = assign33340_e30016_d_n4;
        var_fn418_calc_ig__igindiode_dn8 = assign33340_e30016_d_n8;
        var_fn418_calc_ig__igindiode_dn13 = assign33340_e30016_d_n13;

        let (assign33350_e30020, assign33350_e30020_d_n8, assign33350_e30020_d_n13,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_fn418_calc_ig__frecgin, var_fn418_calc_ig__frecgin_dn8, var_fn418_calc_ig__frecgin_dn13,)
    }
};
        var_fn418_calc_ig__frecgin = assign33350_e30020;
        var_fn418_calc_ig__frecgin_dn8 = assign33350_e30020_d_n8;
        var_fn418_calc_ig__frecgin_dn13 = assign33350_e30020_d_n13;

        let (assign33360_e30024, assign33360_e30024_d_n4, assign33360_e30024_d_n8, assign33360_e30024_d_n13,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn418_calc_ig__iginrec, var_fn418_calc_ig__iginrec_dn4, var_fn418_calc_ig__iginrec_dn8, var_fn418_calc_ig__iginrec_dn13,)
    }
};
        var_fn418_calc_ig__iginrec = assign33360_e30024;
        var_fn418_calc_ig__iginrec_dn4 = assign33360_e30024_d_n4;
        var_fn418_calc_ig__iginrec_dn8 = assign33360_e30024_d_n8;
        var_fn418_calc_ig__iginrec_dn13 = assign33360_e30024_d_n13;

        let (assign33370_e30028, assign33370_e30028_d_n4, assign33370_e30028_d_n8, assign33370_e30028_d_n13,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn418_calc_ig__expbdarg1, var_fn418_calc_ig__expbdarg1_dn4, var_fn418_calc_ig__expbdarg1_dn8, var_fn418_calc_ig__expbdarg1_dn13,)
    }
};
        var_fn418_calc_ig__expbdarg1 = assign33370_e30028;
        var_fn418_calc_ig__expbdarg1_dn4 = assign33370_e30028_d_n4;
        var_fn418_calc_ig__expbdarg1_dn8 = assign33370_e30028_d_n8;
        var_fn418_calc_ig__expbdarg1_dn13 = assign33370_e30028_d_n13;

        let (assign33380_e30032, assign33380_e30032_d_n4,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn418_calc_ig__expbdarg2, var_fn418_calc_ig__expbdarg2_dn4,)
    }
};
        var_fn418_calc_ig__expbdarg2 = assign33380_e30032;
        var_fn418_calc_ig__expbdarg2_dn4 = assign33380_e30032_d_n4;

        let (assign33390_e30036, assign33390_e30036_d_n4, assign33390_e30036_d_n8, assign33390_e30036_d_n13,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn418_calc_ig__expbd1, var_fn418_calc_ig__expbd1_dn4, var_fn418_calc_ig__expbd1_dn8, var_fn418_calc_ig__expbd1_dn13,)
    }
};
        var_fn418_calc_ig__expbd1 = assign33390_e30036;
        var_fn418_calc_ig__expbd1_dn4 = assign33390_e30036_d_n4;
        var_fn418_calc_ig__expbd1_dn8 = assign33390_e30036_d_n8;
        var_fn418_calc_ig__expbd1_dn13 = assign33390_e30036_d_n13;

        let (assign33400_e30040, assign33400_e30040_d_n4,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn418_calc_ig__expbd2, var_fn418_calc_ig__expbd2_dn4,)
    }
};
        var_fn418_calc_ig__expbd2 = assign33400_e30040;
        var_fn418_calc_ig__expbd2_dn4 = assign33400_e30040_d_n4;

        let (assign33410_e30044, assign33410_e30044_d_n4,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn418_calc_ig__expphib, var_fn418_calc_ig__expphib_dn4,)
    }
};
        var_fn418_calc_ig__expphib = assign33410_e30044;
        var_fn418_calc_ig__expphib_dn4 = assign33410_e30044_d_n4;

        let (assign33420_e30048, assign33420_e30048_d_n4, assign33420_e30048_d_n8, assign33420_e30048_d_n13,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn418_calc_ig__expffvarg, var_fn418_calc_ig__expffvarg_dn4, var_fn418_calc_ig__expffvarg_dn8, var_fn418_calc_ig__expffvarg_dn13,)
    }
};
        var_fn418_calc_ig__expffvarg = assign33420_e30048;
        var_fn418_calc_ig__expffvarg_dn4 = assign33420_e30048_d_n4;
        var_fn418_calc_ig__expffvarg_dn8 = assign33420_e30048_d_n8;
        var_fn418_calc_ig__expffvarg_dn13 = assign33420_e30048_d_n13;

        let (assign33430_e30052, assign33430_e30052_d_n4, assign33430_e30052_d_n8, assign33430_e30052_d_n13,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn418_calc_ig__expiforarg, var_fn418_calc_ig__expiforarg_dn4, var_fn418_calc_ig__expiforarg_dn8, var_fn418_calc_ig__expiforarg_dn13,)
    }
};
        var_fn418_calc_ig__expiforarg = assign33430_e30052;
        var_fn418_calc_ig__expiforarg_dn4 = assign33430_e30052_d_n4;
        var_fn418_calc_ig__expiforarg_dn8 = assign33430_e30052_d_n8;
        var_fn418_calc_ig__expiforarg_dn13 = assign33430_e30052_d_n13;

        let (assign33440_e30056, assign33440_e30056_d_n4, assign33440_e30056_d_n8, assign33440_e30056_d_n13,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn418_calc_ig__expifor, var_fn418_calc_ig__expifor_dn4, var_fn418_calc_ig__expifor_dn8, var_fn418_calc_ig__expifor_dn13,)
    }
};
        var_fn418_calc_ig__expifor = assign33440_e30056;
        var_fn418_calc_ig__expifor_dn4 = assign33440_e30056_d_n4;
        var_fn418_calc_ig__expifor_dn8 = assign33440_e30056_d_n8;
        var_fn418_calc_ig__expifor_dn13 = assign33440_e30056_d_n13;

        let (assign33450_e30060, assign33450_e30060_d_n4, assign33450_e30060_d_n8, assign33450_e30060_d_n13,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn418_calc_ig__expirevarg, var_fn418_calc_ig__expirevarg_dn4, var_fn418_calc_ig__expirevarg_dn8, var_fn418_calc_ig__expirevarg_dn13,)
    }
};
        var_fn418_calc_ig__expirevarg = assign33450_e30060;
        var_fn418_calc_ig__expirevarg_dn4 = assign33450_e30060_d_n4;
        var_fn418_calc_ig__expirevarg_dn8 = assign33450_e30060_d_n8;
        var_fn418_calc_ig__expirevarg_dn13 = assign33450_e30060_d_n13;

        let (assign33460_e30064, assign33460_e30064_d_n4, assign33460_e30064_d_n8, assign33460_e30064_d_n13,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn418_calc_ig__expirev, var_fn418_calc_ig__expirev_dn4, var_fn418_calc_ig__expirev_dn8, var_fn418_calc_ig__expirev_dn13,)
    }
};
        var_fn418_calc_ig__expirev = assign33460_e30064;
        var_fn418_calc_ig__expirev_dn4 = assign33460_e30064_d_n4;
        var_fn418_calc_ig__expirev_dn8 = assign33460_e30064_d_n8;
        var_fn418_calc_ig__expirev_dn13 = assign33460_e30064_d_n13;

        let (assign33470_e30068,) = {
    if (var_guard417 != 0.0) {
        (0.0,)
    } else {
        (var_fn418_calc_ig__pg_paramin_hinj,)
    }
};
        var_fn418_calc_ig__pg_paramin_hinj = assign33470_e30068;

        let (assign33480_e30072, assign33480_e30072_d_n4,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn418_calc_ig__expbdarg1_vgsat, var_fn418_calc_ig__expbdarg1_vgsat_dn4,)
    }
};
        var_fn418_calc_ig__expbdarg1_vgsat = assign33480_e30072;
        var_fn418_calc_ig__expbdarg1_vgsat_dn4 = assign33480_e30072_d_n4;

        let (assign33490_e30076, assign33490_e30076_d_n4,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn418_calc_ig__expbd1_vgsat, var_fn418_calc_ig__expbd1_vgsat_dn4,)
    }
};
        var_fn418_calc_ig__expbd1_vgsat = assign33490_e30076;
        var_fn418_calc_ig__expbd1_vgsat_dn4 = assign33490_e30076_d_n4;

        let (assign33500_e30080, assign33500_e30080_d_n4,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn418_calc_ig__iginbd_vgsat, var_fn418_calc_ig__iginbd_vgsat_dn4,)
    }
};
        var_fn418_calc_ig__iginbd_vgsat = assign33500_e30080;
        var_fn418_calc_ig__iginbd_vgsat_dn4 = assign33500_e30080_d_n4;

        let (assign33510_e30084, assign33510_e30084_d_n4,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn418_calc_ig__expiforarg_nohinj_vgsat, var_fn418_calc_ig__expiforarg_nohinj_vgsat_dn4,)
    }
};
        var_fn418_calc_ig__expiforarg_nohinj_vgsat = assign33510_e30084;
        var_fn418_calc_ig__expiforarg_nohinj_vgsat_dn4 = assign33510_e30084_d_n4;

        let (assign33520_e30088, assign33520_e30088_d_n4,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn418_calc_ig__expifor_nohinj_vgsat, var_fn418_calc_ig__expifor_nohinj_vgsat_dn4,)
    }
};
        var_fn418_calc_ig__expifor_nohinj_vgsat = assign33520_e30088;
        var_fn418_calc_ig__expifor_nohinj_vgsat_dn4 = assign33520_e30088_d_n4;

        let (assign33530_e30092, assign33530_e30092_d_n4,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn418_calc_ig__igindiode_nohinj_vgsat, var_fn418_calc_ig__igindiode_nohinj_vgsat_dn4,)
    }
};
        var_fn418_calc_ig__igindiode_nohinj_vgsat = assign33530_e30092;
        var_fn418_calc_ig__igindiode_nohinj_vgsat_dn4 = assign33530_e30092_d_n4;

        let (assign33540_e30096, assign33540_e30096_d_n4, assign33540_e30096_d_n8, assign33540_e30096_d_n13,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn418_calc_ig__igindiode_nohinj, var_fn418_calc_ig__igindiode_nohinj_dn4, var_fn418_calc_ig__igindiode_nohinj_dn8, var_fn418_calc_ig__igindiode_nohinj_dn13,)
    }
};
        var_fn418_calc_ig__igindiode_nohinj = assign33540_e30096;
        var_fn418_calc_ig__igindiode_nohinj_dn4 = assign33540_e30096_d_n4;
        var_fn418_calc_ig__igindiode_nohinj_dn8 = assign33540_e30096_d_n8;
        var_fn418_calc_ig__igindiode_nohinj_dn13 = assign33540_e30096_d_n13;

        let (assign33550_e30100, assign33550_e30100_d_n4,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn418_calc_ig__expiforarg_hinj_vgsat, var_fn418_calc_ig__expiforarg_hinj_vgsat_dn4,)
    }
};
        var_fn418_calc_ig__expiforarg_hinj_vgsat = assign33550_e30100;
        var_fn418_calc_ig__expiforarg_hinj_vgsat_dn4 = assign33550_e30100_d_n4;

        let (assign33560_e30104, assign33560_e30104_d_n4,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn418_calc_ig__expifor_hinj_vgsat, var_fn418_calc_ig__expifor_hinj_vgsat_dn4,)
    }
};
        var_fn418_calc_ig__expifor_hinj_vgsat = assign33560_e30104;
        var_fn418_calc_ig__expifor_hinj_vgsat_dn4 = assign33560_e30104_d_n4;

        let (assign33570_e30108, assign33570_e30108_d_n4,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn418_calc_ig__igindiode_hinj_vgsat, var_fn418_calc_ig__igindiode_hinj_vgsat_dn4,)
    }
};
        var_fn418_calc_ig__igindiode_hinj_vgsat = assign33570_e30108;
        var_fn418_calc_ig__igindiode_hinj_vgsat_dn4 = assign33570_e30108_d_n4;

        let (assign33580_e30112, assign33580_e30112_d_n4, assign33580_e30112_d_n8, assign33580_e30112_d_n13,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn418_calc_ig__expiforarg_hinj, var_fn418_calc_ig__expiforarg_hinj_dn4, var_fn418_calc_ig__expiforarg_hinj_dn8, var_fn418_calc_ig__expiforarg_hinj_dn13,)
    }
};
        var_fn418_calc_ig__expiforarg_hinj = assign33580_e30112;
        var_fn418_calc_ig__expiforarg_hinj_dn4 = assign33580_e30112_d_n4;
        var_fn418_calc_ig__expiforarg_hinj_dn8 = assign33580_e30112_d_n8;
        var_fn418_calc_ig__expiforarg_hinj_dn13 = assign33580_e30112_d_n13;

        let (assign33590_e30116, assign33590_e30116_d_n4, assign33590_e30116_d_n8, assign33590_e30116_d_n13,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn418_calc_ig__expifor_hinj, var_fn418_calc_ig__expifor_hinj_dn4, var_fn418_calc_ig__expifor_hinj_dn8, var_fn418_calc_ig__expifor_hinj_dn13,)
    }
};
        var_fn418_calc_ig__expifor_hinj = assign33590_e30116;
        var_fn418_calc_ig__expifor_hinj_dn4 = assign33590_e30116_d_n4;
        var_fn418_calc_ig__expifor_hinj_dn8 = assign33590_e30116_d_n8;
        var_fn418_calc_ig__expifor_hinj_dn13 = assign33590_e30116_d_n13;

        let (assign33600_e30120, assign33600_e30120_d_n4,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn418_calc_ig__igindiode_hinj_pre, var_fn418_calc_ig__igindiode_hinj_pre_dn4,)
    }
};
        var_fn418_calc_ig__igindiode_hinj_pre = assign33600_e30120;
        var_fn418_calc_ig__igindiode_hinj_pre_dn4 = assign33600_e30120_d_n4;

        *var_fn418_calc_ig__alpha2_phit_slot = var_fn418_calc_ig__alpha2_phit;
        *var_fn418_calc_ig__alpha2_phit_dn4_slot = var_fn418_calc_ig__alpha2_phit_dn4;
        *var_fn418_calc_ig__alphagin_slot = var_fn418_calc_ig__alphagin;
        *var_fn418_calc_ig__betarecin_slot = var_fn418_calc_ig__betarecin;
        *var_fn418_calc_ig__expbd1_slot = var_fn418_calc_ig__expbd1;
        *var_fn418_calc_ig__expbd1_dn13_slot = var_fn418_calc_ig__expbd1_dn13;
        *var_fn418_calc_ig__expbd1_dn4_slot = var_fn418_calc_ig__expbd1_dn4;
        *var_fn418_calc_ig__expbd1_dn8_slot = var_fn418_calc_ig__expbd1_dn8;
        *var_fn418_calc_ig__expbd1_vgsat_slot = var_fn418_calc_ig__expbd1_vgsat;
        *var_fn418_calc_ig__expbd1_vgsat_dn4_slot = var_fn418_calc_ig__expbd1_vgsat_dn4;
        *var_fn418_calc_ig__expbd2_slot = var_fn418_calc_ig__expbd2;
        *var_fn418_calc_ig__expbd2_dn4_slot = var_fn418_calc_ig__expbd2_dn4;
        *var_fn418_calc_ig__expbdarg1_slot = var_fn418_calc_ig__expbdarg1;
        *var_fn418_calc_ig__expbdarg1_dn13_slot = var_fn418_calc_ig__expbdarg1_dn13;
        *var_fn418_calc_ig__expbdarg1_dn4_slot = var_fn418_calc_ig__expbdarg1_dn4;
        *var_fn418_calc_ig__expbdarg1_dn8_slot = var_fn418_calc_ig__expbdarg1_dn8;
        *var_fn418_calc_ig__expbdarg1_vgsat_slot = var_fn418_calc_ig__expbdarg1_vgsat;
        *var_fn418_calc_ig__expbdarg1_vgsat_dn4_slot = var_fn418_calc_ig__expbdarg1_vgsat_dn4;
        *var_fn418_calc_ig__expbdarg2_slot = var_fn418_calc_ig__expbdarg2;
        *var_fn418_calc_ig__expbdarg2_dn4_slot = var_fn418_calc_ig__expbdarg2_dn4;
        *var_fn418_calc_ig__expffvarg_slot = var_fn418_calc_ig__expffvarg;
        *var_fn418_calc_ig__expffvarg_dn13_slot = var_fn418_calc_ig__expffvarg_dn13;
        *var_fn418_calc_ig__expffvarg_dn4_slot = var_fn418_calc_ig__expffvarg_dn4;
        *var_fn418_calc_ig__expffvarg_dn8_slot = var_fn418_calc_ig__expffvarg_dn8;
        *var_fn418_calc_ig__expifor_slot = var_fn418_calc_ig__expifor;
        *var_fn418_calc_ig__expifor_dn13_slot = var_fn418_calc_ig__expifor_dn13;
        *var_fn418_calc_ig__expifor_dn4_slot = var_fn418_calc_ig__expifor_dn4;
        *var_fn418_calc_ig__expifor_dn8_slot = var_fn418_calc_ig__expifor_dn8;
        *var_fn418_calc_ig__expifor_hinj_slot = var_fn418_calc_ig__expifor_hinj;
        *var_fn418_calc_ig__expifor_hinj_dn13_slot = var_fn418_calc_ig__expifor_hinj_dn13;
        *var_fn418_calc_ig__expifor_hinj_dn4_slot = var_fn418_calc_ig__expifor_hinj_dn4;
        *var_fn418_calc_ig__expifor_hinj_dn8_slot = var_fn418_calc_ig__expifor_hinj_dn8;
        *var_fn418_calc_ig__expifor_hinj_vgsat_slot = var_fn418_calc_ig__expifor_hinj_vgsat;
        *var_fn418_calc_ig__expifor_hinj_vgsat_dn4_slot = var_fn418_calc_ig__expifor_hinj_vgsat_dn4;
        *var_fn418_calc_ig__expifor_nohinj_vgsat_slot = var_fn418_calc_ig__expifor_nohinj_vgsat;
        *var_fn418_calc_ig__expifor_nohinj_vgsat_dn4_slot = var_fn418_calc_ig__expifor_nohinj_vgsat_dn4;
        *var_fn418_calc_ig__expiforarg_slot = var_fn418_calc_ig__expiforarg;
        *var_fn418_calc_ig__expiforarg_dn13_slot = var_fn418_calc_ig__expiforarg_dn13;
        *var_fn418_calc_ig__expiforarg_dn4_slot = var_fn418_calc_ig__expiforarg_dn4;
        *var_fn418_calc_ig__expiforarg_dn8_slot = var_fn418_calc_ig__expiforarg_dn8;
        *var_fn418_calc_ig__expiforarg_hinj_slot = var_fn418_calc_ig__expiforarg_hinj;
        *var_fn418_calc_ig__expiforarg_hinj_dn13_slot = var_fn418_calc_ig__expiforarg_hinj_dn13;
        *var_fn418_calc_ig__expiforarg_hinj_dn4_slot = var_fn418_calc_ig__expiforarg_hinj_dn4;
        *var_fn418_calc_ig__expiforarg_hinj_dn8_slot = var_fn418_calc_ig__expiforarg_hinj_dn8;
        *var_fn418_calc_ig__expiforarg_hinj_vgsat_slot = var_fn418_calc_ig__expiforarg_hinj_vgsat;
        *var_fn418_calc_ig__expiforarg_hinj_vgsat_dn4_slot = var_fn418_calc_ig__expiforarg_hinj_vgsat_dn4;
        *var_fn418_calc_ig__expiforarg_nohinj_vgsat_slot = var_fn418_calc_ig__expiforarg_nohinj_vgsat;
        *var_fn418_calc_ig__expiforarg_nohinj_vgsat_dn4_slot = var_fn418_calc_ig__expiforarg_nohinj_vgsat_dn4;
        *var_fn418_calc_ig__expirev_slot = var_fn418_calc_ig__expirev;
        *var_fn418_calc_ig__expirev_dn13_slot = var_fn418_calc_ig__expirev_dn13;
        *var_fn418_calc_ig__expirev_dn4_slot = var_fn418_calc_ig__expirev_dn4;
        *var_fn418_calc_ig__expirev_dn8_slot = var_fn418_calc_ig__expirev_dn8;
        *var_fn418_calc_ig__expirevarg_slot = var_fn418_calc_ig__expirevarg;
        *var_fn418_calc_ig__expirevarg_dn13_slot = var_fn418_calc_ig__expirevarg_dn13;
        *var_fn418_calc_ig__expirevarg_dn4_slot = var_fn418_calc_ig__expirevarg_dn4;
        *var_fn418_calc_ig__expirevarg_dn8_slot = var_fn418_calc_ig__expirevarg_dn8;
        *var_fn418_calc_ig__expphib_slot = var_fn418_calc_ig__expphib;
        *var_fn418_calc_ig__expphib_dn4_slot = var_fn418_calc_ig__expphib_dn4;
        *var_fn418_calc_ig__ffvgin_slot = var_fn418_calc_ig__ffvgin;
        *var_fn418_calc_ig__ffvgin_dn13_slot = var_fn418_calc_ig__ffvgin_dn13;
        *var_fn418_calc_ig__ffvgin_dn4_slot = var_fn418_calc_ig__ffvgin_dn4;
        *var_fn418_calc_ig__ffvgin_dn8_slot = var_fn418_calc_ig__ffvgin_dn8;
        *var_fn418_calc_ig__fracin_slot = var_fn418_calc_ig__fracin;
        *var_fn418_calc_ig__frecgin_slot = var_fn418_calc_ig__frecgin;
        *var_fn418_calc_ig__frecgin_dn13_slot = var_fn418_calc_ig__frecgin_dn13;
        *var_fn418_calc_ig__frecgin_dn8_slot = var_fn418_calc_ig__frecgin_dn8;
        *var_fn418_calc_ig__iginbd_slot = var_fn418_calc_ig__iginbd;
        *var_fn418_calc_ig__iginbd_dn13_slot = var_fn418_calc_ig__iginbd_dn13;
        *var_fn418_calc_ig__iginbd_dn4_slot = var_fn418_calc_ig__iginbd_dn4;
        *var_fn418_calc_ig__iginbd_dn8_slot = var_fn418_calc_ig__iginbd_dn8;
        *var_fn418_calc_ig__iginbd_vgsat_slot = var_fn418_calc_ig__iginbd_vgsat;
        *var_fn418_calc_ig__iginbd_vgsat_dn4_slot = var_fn418_calc_ig__iginbd_vgsat_dn4;
        *var_fn418_calc_ig__igindiode_slot = var_fn418_calc_ig__igindiode;
        *var_fn418_calc_ig__igindiode_dn13_slot = var_fn418_calc_ig__igindiode_dn13;
        *var_fn418_calc_ig__igindiode_dn4_slot = var_fn418_calc_ig__igindiode_dn4;
        *var_fn418_calc_ig__igindiode_dn8_slot = var_fn418_calc_ig__igindiode_dn8;
        *var_fn418_calc_ig__igindiode_hinj_pre_slot = var_fn418_calc_ig__igindiode_hinj_pre;
        *var_fn418_calc_ig__igindiode_hinj_pre_dn4_slot = var_fn418_calc_ig__igindiode_hinj_pre_dn4;
        *var_fn418_calc_ig__igindiode_hinj_vgsat_slot = var_fn418_calc_ig__igindiode_hinj_vgsat;
        *var_fn418_calc_ig__igindiode_hinj_vgsat_dn4_slot = var_fn418_calc_ig__igindiode_hinj_vgsat_dn4;
        *var_fn418_calc_ig__igindiode_nohinj_slot = var_fn418_calc_ig__igindiode_nohinj;
        *var_fn418_calc_ig__igindiode_nohinj_dn13_slot = var_fn418_calc_ig__igindiode_nohinj_dn13;
        *var_fn418_calc_ig__igindiode_nohinj_dn4_slot = var_fn418_calc_ig__igindiode_nohinj_dn4;
        *var_fn418_calc_ig__igindiode_nohinj_dn8_slot = var_fn418_calc_ig__igindiode_nohinj_dn8;
        *var_fn418_calc_ig__igindiode_nohinj_vgsat_slot = var_fn418_calc_ig__igindiode_nohinj_vgsat;
        *var_fn418_calc_ig__igindiode_nohinj_vgsat_dn4_slot = var_fn418_calc_ig__igindiode_nohinj_vgsat_dn4;
        *var_fn418_calc_ig__iginrec_slot = var_fn418_calc_ig__iginrec;
        *var_fn418_calc_ig__iginrec_dn13_slot = var_fn418_calc_ig__iginrec_dn13;
        *var_fn418_calc_ig__iginrec_dn4_slot = var_fn418_calc_ig__iginrec_dn4;
        *var_fn418_calc_ig__iginrec_dn8_slot = var_fn418_calc_ig__iginrec_dn8;
        *var_fn418_calc_ig__igout_slot = var_fn418_calc_ig__igout;
        *var_fn418_calc_ig__igout_dn13_slot = var_fn418_calc_ig__igout_dn13;
        *var_fn418_calc_ig__igout_dn4_slot = var_fn418_calc_ig__igout_dn4;
        *var_fn418_calc_ig__igout_dn8_slot = var_fn418_calc_ig__igout_dn8;
        *var_fn418_calc_ig__ijin_slot = var_fn418_calc_ig__ijin;
        *var_fn418_calc_ig__irecin_slot = var_fn418_calc_ig__irecin;
        *var_fn418_calc_ig__kbdgatein_slot = var_fn418_calc_ig__kbdgatein;
        *var_fn418_calc_ig__ngf_slot = var_fn418_calc_ig__ngf;
        *var_fn418_calc_ig__pbdgin_slot = var_fn418_calc_ig__pbdgin;
        *var_fn418_calc_ig__pg_param1_slot = var_fn418_calc_ig__pg_param1;
        *var_fn418_calc_ig__pg_paramin_slot = var_fn418_calc_ig__pg_paramin;
        *var_fn418_calc_ig__pg_paramin_hinj_slot = var_fn418_calc_ig__pg_paramin_hinj;
        *var_fn418_calc_ig__pgsrecin_slot = var_fn418_calc_ig__pgsrecin;
        *var_fn418_calc_ig__t0_slot = var_fn418_calc_ig__t0;
        *var_fn418_calc_ig__t0_dn4_slot = var_fn418_calc_ig__t0_dn4;
        *var_fn418_calc_ig__tfacdiodein_slot = var_fn418_calc_ig__tfacdiodein;
        *var_fn418_calc_ig__tfacdiodein_dn4_slot = var_fn418_calc_ig__tfacdiodein_dn4;
        *var_fn418_calc_ig__type_slot = var_fn418_calc_ig__type;
        *var_fn418_calc_ig__vbdgin_slot = var_fn418_calc_ig__vbdgin;
        *var_fn418_calc_ig__vgsatqin_slot = var_fn418_calc_ig__vgsatqin;
        *var_fn418_calc_ig__vjg_slot = var_fn418_calc_ig__vjg;
        *var_fn418_calc_ig__w_slot = var_fn418_calc_ig__w;
    }

    pub(super) fn stamp_transient_block_82(
        var_fn418_calc_ig__fracin: f64,
        var_fn418_calc_ig__ijin: f64,
        var_fn418_calc_ig__kbdgatein: f64,
        var_fn418_calc_ig__ngf: f64,
        var_fn418_calc_ig__pbdgin: f64,
        var_fn418_calc_ig__pg_param1: f64,
        var_fn418_calc_ig__pg_paramin: f64,
        var_fn418_calc_ig__phitin: f64,
        var_fn418_calc_ig__phitin_dn4: f64,
        var_fn418_calc_ig__tfacdiodein: f64,
        var_fn418_calc_ig__tfacdiodein_dn4: f64,
        var_fn418_calc_ig__type: f64,
        var_fn418_calc_ig__vbdgin: f64,
        var_fn418_calc_ig__vgin: f64,
        var_fn418_calc_ig__vgin_dn13: f64,
        var_fn418_calc_ig__vgin_dn8: f64,
        var_fn418_calc_ig__vgsatin: f64,
        var_fn418_calc_ig__vjg: f64,
        var_fn418_calc_ig__w: f64,
        var_guard417: f64,
        var_fn418_calc_ig__expbd1_slot: &mut f64,
        var_fn418_calc_ig__expbd1_dn13_slot: &mut f64,
        var_fn418_calc_ig__expbd1_dn4_slot: &mut f64,
        var_fn418_calc_ig__expbd1_dn8_slot: &mut f64,
        var_fn418_calc_ig__expbd1_vgsat_slot: &mut f64,
        var_fn418_calc_ig__expbd1_vgsat_dn4_slot: &mut f64,
        var_fn418_calc_ig__expbd2_slot: &mut f64,
        var_fn418_calc_ig__expbd2_dn4_slot: &mut f64,
        var_fn418_calc_ig__expbdarg1_slot: &mut f64,
        var_fn418_calc_ig__expbdarg1_dn13_slot: &mut f64,
        var_fn418_calc_ig__expbdarg1_dn4_slot: &mut f64,
        var_fn418_calc_ig__expbdarg1_dn8_slot: &mut f64,
        var_fn418_calc_ig__expbdarg1_vgsat_slot: &mut f64,
        var_fn418_calc_ig__expbdarg1_vgsat_dn4_slot: &mut f64,
        var_fn418_calc_ig__expbdarg2_slot: &mut f64,
        var_fn418_calc_ig__expbdarg2_dn4_slot: &mut f64,
        var_fn418_calc_ig__expifor_slot: &mut f64,
        var_fn418_calc_ig__expifor_dn13_slot: &mut f64,
        var_fn418_calc_ig__expifor_dn4_slot: &mut f64,
        var_fn418_calc_ig__expifor_dn8_slot: &mut f64,
        var_fn418_calc_ig__expifor_hinj_vgsat_slot: &mut f64,
        var_fn418_calc_ig__expifor_hinj_vgsat_dn4_slot: &mut f64,
        var_fn418_calc_ig__expifor_nohinj_vgsat_slot: &mut f64,
        var_fn418_calc_ig__expifor_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn418_calc_ig__expiforarg_slot: &mut f64,
        var_fn418_calc_ig__expiforarg_dn13_slot: &mut f64,
        var_fn418_calc_ig__expiforarg_dn4_slot: &mut f64,
        var_fn418_calc_ig__expiforarg_dn8_slot: &mut f64,
        var_fn418_calc_ig__expiforarg_hinj_vgsat_slot: &mut f64,
        var_fn418_calc_ig__expiforarg_hinj_vgsat_dn4_slot: &mut f64,
        var_fn418_calc_ig__expiforarg_nohinj_vgsat_slot: &mut f64,
        var_fn418_calc_ig__expiforarg_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn418_calc_ig__expphib_slot: &mut f64,
        var_fn418_calc_ig__expphib_dn4_slot: &mut f64,
        var_fn418_calc_ig__iginbd_slot: &mut f64,
        var_fn418_calc_ig__iginbd_dn13_slot: &mut f64,
        var_fn418_calc_ig__iginbd_dn4_slot: &mut f64,
        var_fn418_calc_ig__iginbd_dn8_slot: &mut f64,
        var_fn418_calc_ig__iginbd_vgsat_slot: &mut f64,
        var_fn418_calc_ig__iginbd_vgsat_dn4_slot: &mut f64,
        var_fn418_calc_ig__igindiode_slot: &mut f64,
        var_fn418_calc_ig__igindiode_dn13_slot: &mut f64,
        var_fn418_calc_ig__igindiode_dn4_slot: &mut f64,
        var_fn418_calc_ig__igindiode_dn8_slot: &mut f64,
        var_fn418_calc_ig__igindiode_hinj_slot: &mut f64,
        var_fn418_calc_ig__igindiode_hinj_dn13_slot: &mut f64,
        var_fn418_calc_ig__igindiode_hinj_dn4_slot: &mut f64,
        var_fn418_calc_ig__igindiode_hinj_dn8_slot: &mut f64,
        var_fn418_calc_ig__igindiode_hinj_vgsat_slot: &mut f64,
        var_fn418_calc_ig__igindiode_hinj_vgsat_dn4_slot: &mut f64,
        var_fn418_calc_ig__igindiode_nohinj_slot: &mut f64,
        var_fn418_calc_ig__igindiode_nohinj_dn13_slot: &mut f64,
        var_fn418_calc_ig__igindiode_nohinj_dn4_slot: &mut f64,
        var_fn418_calc_ig__igindiode_nohinj_dn8_slot: &mut f64,
        var_fn418_calc_ig__igindiode_nohinj_vgsat_slot: &mut f64,
        var_fn418_calc_ig__igindiode_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn418_calc_ig__isdiodeout_slot: &mut f64,
        var_fn418_calc_ig__isdiodeout_dn4_slot: &mut f64,
        var_fn418_calc_ig__pg_paramin_hinj_slot: &mut f64,
        var_fn418_calc_ig__t0_slot: &mut f64,
        var_fn418_calc_ig__t0_dn4_slot: &mut f64,
        var_guard419_slot: &mut f64,
        var_guard420_slot: &mut f64,
    ) {
        let mut var_fn418_calc_ig__expbd1: f64 = *var_fn418_calc_ig__expbd1_slot;
        let mut var_fn418_calc_ig__expbd1_dn13: f64 = *var_fn418_calc_ig__expbd1_dn13_slot;
        let mut var_fn418_calc_ig__expbd1_dn4: f64 = *var_fn418_calc_ig__expbd1_dn4_slot;
        let mut var_fn418_calc_ig__expbd1_dn8: f64 = *var_fn418_calc_ig__expbd1_dn8_slot;
        let mut var_fn418_calc_ig__expbd1_vgsat: f64 = *var_fn418_calc_ig__expbd1_vgsat_slot;
        let mut var_fn418_calc_ig__expbd1_vgsat_dn4: f64 = *var_fn418_calc_ig__expbd1_vgsat_dn4_slot;
        let mut var_fn418_calc_ig__expbd2: f64 = *var_fn418_calc_ig__expbd2_slot;
        let mut var_fn418_calc_ig__expbd2_dn4: f64 = *var_fn418_calc_ig__expbd2_dn4_slot;
        let mut var_fn418_calc_ig__expbdarg1: f64 = *var_fn418_calc_ig__expbdarg1_slot;
        let mut var_fn418_calc_ig__expbdarg1_dn13: f64 = *var_fn418_calc_ig__expbdarg1_dn13_slot;
        let mut var_fn418_calc_ig__expbdarg1_dn4: f64 = *var_fn418_calc_ig__expbdarg1_dn4_slot;
        let mut var_fn418_calc_ig__expbdarg1_dn8: f64 = *var_fn418_calc_ig__expbdarg1_dn8_slot;
        let mut var_fn418_calc_ig__expbdarg1_vgsat: f64 = *var_fn418_calc_ig__expbdarg1_vgsat_slot;
        let mut var_fn418_calc_ig__expbdarg1_vgsat_dn4: f64 = *var_fn418_calc_ig__expbdarg1_vgsat_dn4_slot;
        let mut var_fn418_calc_ig__expbdarg2: f64 = *var_fn418_calc_ig__expbdarg2_slot;
        let mut var_fn418_calc_ig__expbdarg2_dn4: f64 = *var_fn418_calc_ig__expbdarg2_dn4_slot;
        let mut var_fn418_calc_ig__expifor: f64 = *var_fn418_calc_ig__expifor_slot;
        let mut var_fn418_calc_ig__expifor_dn13: f64 = *var_fn418_calc_ig__expifor_dn13_slot;
        let mut var_fn418_calc_ig__expifor_dn4: f64 = *var_fn418_calc_ig__expifor_dn4_slot;
        let mut var_fn418_calc_ig__expifor_dn8: f64 = *var_fn418_calc_ig__expifor_dn8_slot;
        let mut var_fn418_calc_ig__expifor_hinj_vgsat: f64 = *var_fn418_calc_ig__expifor_hinj_vgsat_slot;
        let mut var_fn418_calc_ig__expifor_hinj_vgsat_dn4: f64 = *var_fn418_calc_ig__expifor_hinj_vgsat_dn4_slot;
        let mut var_fn418_calc_ig__expifor_nohinj_vgsat: f64 = *var_fn418_calc_ig__expifor_nohinj_vgsat_slot;
        let mut var_fn418_calc_ig__expifor_nohinj_vgsat_dn4: f64 = *var_fn418_calc_ig__expifor_nohinj_vgsat_dn4_slot;
        let mut var_fn418_calc_ig__expiforarg: f64 = *var_fn418_calc_ig__expiforarg_slot;
        let mut var_fn418_calc_ig__expiforarg_dn13: f64 = *var_fn418_calc_ig__expiforarg_dn13_slot;
        let mut var_fn418_calc_ig__expiforarg_dn4: f64 = *var_fn418_calc_ig__expiforarg_dn4_slot;
        let mut var_fn418_calc_ig__expiforarg_dn8: f64 = *var_fn418_calc_ig__expiforarg_dn8_slot;
        let mut var_fn418_calc_ig__expiforarg_hinj_vgsat: f64 = *var_fn418_calc_ig__expiforarg_hinj_vgsat_slot;
        let mut var_fn418_calc_ig__expiforarg_hinj_vgsat_dn4: f64 = *var_fn418_calc_ig__expiforarg_hinj_vgsat_dn4_slot;
        let mut var_fn418_calc_ig__expiforarg_nohinj_vgsat: f64 = *var_fn418_calc_ig__expiforarg_nohinj_vgsat_slot;
        let mut var_fn418_calc_ig__expiforarg_nohinj_vgsat_dn4: f64 = *var_fn418_calc_ig__expiforarg_nohinj_vgsat_dn4_slot;
        let mut var_fn418_calc_ig__expphib: f64 = *var_fn418_calc_ig__expphib_slot;
        let mut var_fn418_calc_ig__expphib_dn4: f64 = *var_fn418_calc_ig__expphib_dn4_slot;
        let mut var_fn418_calc_ig__iginbd: f64 = *var_fn418_calc_ig__iginbd_slot;
        let mut var_fn418_calc_ig__iginbd_dn13: f64 = *var_fn418_calc_ig__iginbd_dn13_slot;
        let mut var_fn418_calc_ig__iginbd_dn4: f64 = *var_fn418_calc_ig__iginbd_dn4_slot;
        let mut var_fn418_calc_ig__iginbd_dn8: f64 = *var_fn418_calc_ig__iginbd_dn8_slot;
        let mut var_fn418_calc_ig__iginbd_vgsat: f64 = *var_fn418_calc_ig__iginbd_vgsat_slot;
        let mut var_fn418_calc_ig__iginbd_vgsat_dn4: f64 = *var_fn418_calc_ig__iginbd_vgsat_dn4_slot;
        let mut var_fn418_calc_ig__igindiode: f64 = *var_fn418_calc_ig__igindiode_slot;
        let mut var_fn418_calc_ig__igindiode_dn13: f64 = *var_fn418_calc_ig__igindiode_dn13_slot;
        let mut var_fn418_calc_ig__igindiode_dn4: f64 = *var_fn418_calc_ig__igindiode_dn4_slot;
        let mut var_fn418_calc_ig__igindiode_dn8: f64 = *var_fn418_calc_ig__igindiode_dn8_slot;
        let mut var_fn418_calc_ig__igindiode_hinj: f64 = *var_fn418_calc_ig__igindiode_hinj_slot;
        let mut var_fn418_calc_ig__igindiode_hinj_dn13: f64 = *var_fn418_calc_ig__igindiode_hinj_dn13_slot;
        let mut var_fn418_calc_ig__igindiode_hinj_dn4: f64 = *var_fn418_calc_ig__igindiode_hinj_dn4_slot;
        let mut var_fn418_calc_ig__igindiode_hinj_dn8: f64 = *var_fn418_calc_ig__igindiode_hinj_dn8_slot;
        let mut var_fn418_calc_ig__igindiode_hinj_vgsat: f64 = *var_fn418_calc_ig__igindiode_hinj_vgsat_slot;
        let mut var_fn418_calc_ig__igindiode_hinj_vgsat_dn4: f64 = *var_fn418_calc_ig__igindiode_hinj_vgsat_dn4_slot;
        let mut var_fn418_calc_ig__igindiode_nohinj: f64 = *var_fn418_calc_ig__igindiode_nohinj_slot;
        let mut var_fn418_calc_ig__igindiode_nohinj_dn13: f64 = *var_fn418_calc_ig__igindiode_nohinj_dn13_slot;
        let mut var_fn418_calc_ig__igindiode_nohinj_dn4: f64 = *var_fn418_calc_ig__igindiode_nohinj_dn4_slot;
        let mut var_fn418_calc_ig__igindiode_nohinj_dn8: f64 = *var_fn418_calc_ig__igindiode_nohinj_dn8_slot;
        let mut var_fn418_calc_ig__igindiode_nohinj_vgsat: f64 = *var_fn418_calc_ig__igindiode_nohinj_vgsat_slot;
        let mut var_fn418_calc_ig__igindiode_nohinj_vgsat_dn4: f64 = *var_fn418_calc_ig__igindiode_nohinj_vgsat_dn4_slot;
        let mut var_fn418_calc_ig__isdiodeout: f64 = *var_fn418_calc_ig__isdiodeout_slot;
        let mut var_fn418_calc_ig__isdiodeout_dn4: f64 = *var_fn418_calc_ig__isdiodeout_dn4_slot;
        let mut var_fn418_calc_ig__pg_paramin_hinj: f64 = *var_fn418_calc_ig__pg_paramin_hinj_slot;
        let mut var_fn418_calc_ig__t0: f64 = *var_fn418_calc_ig__t0_slot;
        let mut var_fn418_calc_ig__t0_dn4: f64 = *var_fn418_calc_ig__t0_dn4_slot;
        let mut var_guard419: f64 = *var_guard419_slot;
        let mut var_guard420: f64 = *var_guard420_slot;

        let (assign33610_e30124, assign33610_e30124_d_n4, assign33610_e30124_d_n8, assign33610_e30124_d_n13,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn418_calc_ig__igindiode_hinj, var_fn418_calc_ig__igindiode_hinj_dn4, var_fn418_calc_ig__igindiode_hinj_dn8, var_fn418_calc_ig__igindiode_hinj_dn13,)
    }
};
        var_fn418_calc_ig__igindiode_hinj = assign33610_e30124;
        var_fn418_calc_ig__igindiode_hinj_dn4 = assign33610_e30124_d_n4;
        var_fn418_calc_ig__igindiode_hinj_dn8 = assign33610_e30124_d_n8;
        var_fn418_calc_ig__igindiode_hinj_dn13 = assign33610_e30124_d_n13;

        let (assign33620_e30133, assign33620_e30133_d_n4,) = {
    if (var_guard417 != 0.0) {
        let assign33620_e30128: f64 = (var_fn418_calc_ig__pg_param1 / var_fn418_calc_ig__phitin);
        let assign33620_e30130: f64 = (-var_fn418_calc_ig__vjg);
        let assign33620_e30131: f64 = (assign33620_e30128 * assign33620_e30130);
        (assign33620_e30131, ((-((var_fn418_calc_ig__pg_param1 * var_fn418_calc_ig__phitin_dn4) / (var_fn418_calc_ig__phitin * var_fn418_calc_ig__phitin))) * assign33620_e30130),)
    } else {
        (var_fn418_calc_ig__expphib, var_fn418_calc_ig__expphib_dn4,)
    }
};
        var_fn418_calc_ig__expphib = assign33620_e30133;
        var_fn418_calc_ig__expphib_dn4 = assign33620_e30133_d_n4;

        let (assign33630_e30175, assign33630_e30175_d_n4,) = {
    if (var_guard417 != 0.0) {
        let assign33630_e30141: f64 = (-50.0);
        let (assign33630_e30173, assign33630_e30173_d_n4,) = {
            if ((!(var_fn418_calc_ig__expphib > 50.0)) && (!(var_fn418_calc_ig__expphib < assign33630_e30141))) {
                let assign33630_e30146: f64 = (var_fn418_calc_ig__expphib).exp();
                (assign33630_e30146, (assign33630_e30146 * var_fn418_calc_ig__expphib_dn4),)
            } else {
                let assign33630_e30153: f64 = (-50.0);
                let (assign33630_e30172, assign33630_e30172_d_n4,) = {
                    if ((!(var_fn418_calc_ig__expphib > 50.0)) && (var_fn418_calc_ig__expphib < assign33630_e30153)) {
                        let assign33630_e30157: f64 = (-50.0);
                        let assign33630_e30158: f64 = (assign33630_e30157).exp();
                        (assign33630_e30158, 0.0,)
                    } else {
                        let (assign33630_e30171, assign33630_e30171_d_n4,) = {
                            if (var_fn418_calc_ig__expphib > 50.0) {
                                let assign33630_e30163: f64 = (50.0_f64).exp();
                                let assign33630_e30167: f64 = (var_fn418_calc_ig__expphib - 50.0);
                                let assign33630_e30168: f64 = (1.0 + assign33630_e30167);
                                let assign33630_e30169: f64 = (assign33630_e30163 * assign33630_e30168);
                                (assign33630_e30169, (assign33630_e30163 * var_fn418_calc_ig__expphib_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign33630_e30171, assign33630_e30171_d_n4,)
                    }
                };
                (assign33630_e30172, assign33630_e30172_d_n4,)
            }
        };
        (assign33630_e30173, assign33630_e30173_d_n4,)
    } else {
        (var_fn418_calc_ig__t0, var_fn418_calc_ig__t0_dn4,)
    }
};
        var_fn418_calc_ig__t0 = assign33630_e30175;
        var_fn418_calc_ig__t0_dn4 = assign33630_e30175_d_n4;

        let (assign33640_e30186, assign33640_e30186_d_n4, assign33640_e30186_d_n8, assign33640_e30186_d_n13,) = {
    if (var_guard417 != 0.0) {
        let assign33640_e30179: f64 = (-var_fn418_calc_ig__vgin);
        let assign33640_e30181: f64 = (assign33640_e30179 - var_fn418_calc_ig__vbdgin);
        let assign33640_e30182: f64 = (var_fn418_calc_ig__pbdgin * assign33640_e30181);
        let assign33640_e30184: f64 = (assign33640_e30182 + var_fn418_calc_ig__expphib);
        (assign33640_e30184, var_fn418_calc_ig__expphib_dn4, (var_fn418_calc_ig__pbdgin * (-var_fn418_calc_ig__vgin_dn8)), (var_fn418_calc_ig__pbdgin * (-var_fn418_calc_ig__vgin_dn13)),)
    } else {
        (var_fn418_calc_ig__expbdarg1, var_fn418_calc_ig__expbdarg1_dn4, var_fn418_calc_ig__expbdarg1_dn8, var_fn418_calc_ig__expbdarg1_dn13,)
    }
};
        var_fn418_calc_ig__expbdarg1 = assign33640_e30186;
        var_fn418_calc_ig__expbdarg1_dn4 = assign33640_e30186_d_n4;
        var_fn418_calc_ig__expbdarg1_dn8 = assign33640_e30186_d_n8;
        var_fn418_calc_ig__expbdarg1_dn13 = assign33640_e30186_d_n13;

        let (assign33650_e30195, assign33650_e30195_d_n4,) = {
    if (var_guard417 != 0.0) {
        let assign33650_e30189: f64 = (-var_fn418_calc_ig__pbdgin);
        let assign33650_e30191: f64 = (assign33650_e30189 * var_fn418_calc_ig__vbdgin);
        let assign33650_e30193: f64 = (assign33650_e30191 + var_fn418_calc_ig__expphib);
        (assign33650_e30193, var_fn418_calc_ig__expphib_dn4,)
    } else {
        (var_fn418_calc_ig__expbdarg2, var_fn418_calc_ig__expbdarg2_dn4,)
    }
};
        var_fn418_calc_ig__expbdarg2 = assign33650_e30195;
        var_fn418_calc_ig__expbdarg2_dn4 = assign33650_e30195_d_n4;

        let (assign33660_e30237, assign33660_e30237_d_n4, assign33660_e30237_d_n8, assign33660_e30237_d_n13,) = {
    if (var_guard417 != 0.0) {
        let assign33660_e30203: f64 = (-50.0);
        let (assign33660_e30235, assign33660_e30235_d_n4, assign33660_e30235_d_n8, assign33660_e30235_d_n13,) = {
            if ((!(var_fn418_calc_ig__expbdarg1 > 50.0)) && (!(var_fn418_calc_ig__expbdarg1 < assign33660_e30203))) {
                let assign33660_e30208: f64 = (var_fn418_calc_ig__expbdarg1).exp();
                (assign33660_e30208, (assign33660_e30208 * var_fn418_calc_ig__expbdarg1_dn4), (assign33660_e30208 * var_fn418_calc_ig__expbdarg1_dn8), (assign33660_e30208 * var_fn418_calc_ig__expbdarg1_dn13),)
            } else {
                let assign33660_e30215: f64 = (-50.0);
                let (assign33660_e30234, assign33660_e30234_d_n4, assign33660_e30234_d_n8, assign33660_e30234_d_n13,) = {
                    if ((!(var_fn418_calc_ig__expbdarg1 > 50.0)) && (var_fn418_calc_ig__expbdarg1 < assign33660_e30215)) {
                        let assign33660_e30219: f64 = (-50.0);
                        let assign33660_e30220: f64 = (assign33660_e30219).exp();
                        (assign33660_e30220, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign33660_e30233, assign33660_e30233_d_n4, assign33660_e30233_d_n8, assign33660_e30233_d_n13,) = {
                            if (var_fn418_calc_ig__expbdarg1 > 50.0) {
                                let assign33660_e30225: f64 = (50.0_f64).exp();
                                let assign33660_e30229: f64 = (var_fn418_calc_ig__expbdarg1 - 50.0);
                                let assign33660_e30230: f64 = (1.0 + assign33660_e30229);
                                let assign33660_e30231: f64 = (assign33660_e30225 * assign33660_e30230);
                                (assign33660_e30231, (assign33660_e30225 * var_fn418_calc_ig__expbdarg1_dn4), (assign33660_e30225 * var_fn418_calc_ig__expbdarg1_dn8), (assign33660_e30225 * var_fn418_calc_ig__expbdarg1_dn13),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign33660_e30233, assign33660_e30233_d_n4, assign33660_e30233_d_n8, assign33660_e30233_d_n13,)
                    }
                };
                (assign33660_e30234, assign33660_e30234_d_n4, assign33660_e30234_d_n8, assign33660_e30234_d_n13,)
            }
        };
        (assign33660_e30235, assign33660_e30235_d_n4, assign33660_e30235_d_n8, assign33660_e30235_d_n13,)
    } else {
        (var_fn418_calc_ig__expbd1, var_fn418_calc_ig__expbd1_dn4, var_fn418_calc_ig__expbd1_dn8, var_fn418_calc_ig__expbd1_dn13,)
    }
};
        var_fn418_calc_ig__expbd1 = assign33660_e30237;
        var_fn418_calc_ig__expbd1_dn4 = assign33660_e30237_d_n4;
        var_fn418_calc_ig__expbd1_dn8 = assign33660_e30237_d_n8;
        var_fn418_calc_ig__expbd1_dn13 = assign33660_e30237_d_n13;

        let (assign33670_e30279, assign33670_e30279_d_n4,) = {
    if (var_guard417 != 0.0) {
        let assign33670_e30245: f64 = (-50.0);
        let (assign33670_e30277, assign33670_e30277_d_n4,) = {
            if ((!(var_fn418_calc_ig__expbdarg2 > 50.0)) && (!(var_fn418_calc_ig__expbdarg2 < assign33670_e30245))) {
                let assign33670_e30250: f64 = (var_fn418_calc_ig__expbdarg2).exp();
                (assign33670_e30250, (assign33670_e30250 * var_fn418_calc_ig__expbdarg2_dn4),)
            } else {
                let assign33670_e30257: f64 = (-50.0);
                let (assign33670_e30276, assign33670_e30276_d_n4,) = {
                    if ((!(var_fn418_calc_ig__expbdarg2 > 50.0)) && (var_fn418_calc_ig__expbdarg2 < assign33670_e30257)) {
                        let assign33670_e30261: f64 = (-50.0);
                        let assign33670_e30262: f64 = (assign33670_e30261).exp();
                        (assign33670_e30262, 0.0,)
                    } else {
                        let (assign33670_e30275, assign33670_e30275_d_n4,) = {
                            if (var_fn418_calc_ig__expbdarg2 > 50.0) {
                                let assign33670_e30267: f64 = (50.0_f64).exp();
                                let assign33670_e30271: f64 = (var_fn418_calc_ig__expbdarg2 - 50.0);
                                let assign33670_e30272: f64 = (1.0 + assign33670_e30271);
                                let assign33670_e30273: f64 = (assign33670_e30267 * assign33670_e30272);
                                (assign33670_e30273, (assign33670_e30267 * var_fn418_calc_ig__expbdarg2_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign33670_e30275, assign33670_e30275_d_n4,)
                    }
                };
                (assign33670_e30276, assign33670_e30276_d_n4,)
            }
        };
        (assign33670_e30277, assign33670_e30277_d_n4,)
    } else {
        (var_fn418_calc_ig__expbd2, var_fn418_calc_ig__expbd2_dn4,)
    }
};
        var_fn418_calc_ig__expbd2 = assign33670_e30279;
        var_fn418_calc_ig__expbd2_dn4 = assign33670_e30279_d_n4;

        let (assign33680_e30285, assign33680_e30285_d_n4, assign33680_e30285_d_n8, assign33680_e30285_d_n13,) = {
    if (var_guard417 != 0.0) {
        let assign33680_e30283: f64 = (var_fn418_calc_ig__expbd1 - var_fn418_calc_ig__expbd2);
        (assign33680_e30283, (var_fn418_calc_ig__expbd1_dn4 - var_fn418_calc_ig__expbd2_dn4), var_fn418_calc_ig__expbd1_dn8, var_fn418_calc_ig__expbd1_dn13,)
    } else {
        (var_fn418_calc_ig__iginbd, var_fn418_calc_ig__iginbd_dn4, var_fn418_calc_ig__iginbd_dn8, var_fn418_calc_ig__iginbd_dn13,)
    }
};
        var_fn418_calc_ig__iginbd = assign33680_e30285;
        var_fn418_calc_ig__iginbd_dn4 = assign33680_e30285_d_n4;
        var_fn418_calc_ig__iginbd_dn8 = assign33680_e30285_d_n8;
        var_fn418_calc_ig__iginbd_dn13 = assign33680_e30285_d_n13;

        let (assign33690_e30297, assign33690_e30297_d_n4,) = {
    if (var_guard417 != 0.0) {
        let assign33690_e30289: f64 = (var_fn418_calc_ig__type * var_fn418_calc_ig__w);
        let assign33690_e30291: f64 = (assign33690_e30289 * var_fn418_calc_ig__ngf);
        let assign33690_e30293: f64 = (assign33690_e30291 * var_fn418_calc_ig__ijin);
        let assign33690_e30295: f64 = (assign33690_e30293 * var_fn418_calc_ig__tfacdiodein);
        (assign33690_e30295, (assign33690_e30293 * var_fn418_calc_ig__tfacdiodein_dn4),)
    } else {
        (var_fn418_calc_ig__isdiodeout, var_fn418_calc_ig__isdiodeout_dn4,)
    }
};
        var_fn418_calc_ig__isdiodeout = assign33690_e30297;
        var_fn418_calc_ig__isdiodeout_dn4 = assign33690_e30297_d_n4;

        let (assign33700_e30307, assign33700_e30307_d_n4, assign33700_e30307_d_n8, assign33700_e30307_d_n13,) = {
    if (var_guard417 != 0.0) {
        let assign33700_e30301: f64 = (var_fn418_calc_ig__pg_paramin / var_fn418_calc_ig__phitin);
        let assign33700_e30303: f64 = (assign33700_e30301 * var_fn418_calc_ig__vgin);
        let assign33700_e30305: f64 = (assign33700_e30303 + var_fn418_calc_ig__expphib);
        (assign33700_e30305, (((-((var_fn418_calc_ig__pg_paramin * var_fn418_calc_ig__phitin_dn4) / (var_fn418_calc_ig__phitin * var_fn418_calc_ig__phitin))) * var_fn418_calc_ig__vgin) + var_fn418_calc_ig__expphib_dn4), (assign33700_e30301 * var_fn418_calc_ig__vgin_dn8), (assign33700_e30301 * var_fn418_calc_ig__vgin_dn13),)
    } else {
        (var_fn418_calc_ig__expiforarg, var_fn418_calc_ig__expiforarg_dn4, var_fn418_calc_ig__expiforarg_dn8, var_fn418_calc_ig__expiforarg_dn13,)
    }
};
        var_fn418_calc_ig__expiforarg = assign33700_e30307;
        var_fn418_calc_ig__expiforarg_dn4 = assign33700_e30307_d_n4;
        var_fn418_calc_ig__expiforarg_dn8 = assign33700_e30307_d_n8;
        var_fn418_calc_ig__expiforarg_dn13 = assign33700_e30307_d_n13;

        let (assign33710_e30349, assign33710_e30349_d_n4, assign33710_e30349_d_n8, assign33710_e30349_d_n13,) = {
    if (var_guard417 != 0.0) {
        let assign33710_e30315: f64 = (-50.0);
        let (assign33710_e30347, assign33710_e30347_d_n4, assign33710_e30347_d_n8, assign33710_e30347_d_n13,) = {
            if ((!(var_fn418_calc_ig__expiforarg > 50.0)) && (!(var_fn418_calc_ig__expiforarg < assign33710_e30315))) {
                let assign33710_e30320: f64 = (var_fn418_calc_ig__expiforarg).exp();
                (assign33710_e30320, (assign33710_e30320 * var_fn418_calc_ig__expiforarg_dn4), (assign33710_e30320 * var_fn418_calc_ig__expiforarg_dn8), (assign33710_e30320 * var_fn418_calc_ig__expiforarg_dn13),)
            } else {
                let assign33710_e30327: f64 = (-50.0);
                let (assign33710_e30346, assign33710_e30346_d_n4, assign33710_e30346_d_n8, assign33710_e30346_d_n13,) = {
                    if ((!(var_fn418_calc_ig__expiforarg > 50.0)) && (var_fn418_calc_ig__expiforarg < assign33710_e30327)) {
                        let assign33710_e30331: f64 = (-50.0);
                        let assign33710_e30332: f64 = (assign33710_e30331).exp();
                        (assign33710_e30332, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign33710_e30345, assign33710_e30345_d_n4, assign33710_e30345_d_n8, assign33710_e30345_d_n13,) = {
                            if (var_fn418_calc_ig__expiforarg > 50.0) {
                                let assign33710_e30337: f64 = (50.0_f64).exp();
                                let assign33710_e30341: f64 = (var_fn418_calc_ig__expiforarg - 50.0);
                                let assign33710_e30342: f64 = (1.0 + assign33710_e30341);
                                let assign33710_e30343: f64 = (assign33710_e30337 * assign33710_e30342);
                                (assign33710_e30343, (assign33710_e30337 * var_fn418_calc_ig__expiforarg_dn4), (assign33710_e30337 * var_fn418_calc_ig__expiforarg_dn8), (assign33710_e30337 * var_fn418_calc_ig__expiforarg_dn13),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign33710_e30345, assign33710_e30345_d_n4, assign33710_e30345_d_n8, assign33710_e30345_d_n13,)
                    }
                };
                (assign33710_e30346, assign33710_e30346_d_n4, assign33710_e30346_d_n8, assign33710_e30346_d_n13,)
            }
        };
        (assign33710_e30347, assign33710_e30347_d_n4, assign33710_e30347_d_n8, assign33710_e30347_d_n13,)
    } else {
        (var_fn418_calc_ig__expifor, var_fn418_calc_ig__expifor_dn4, var_fn418_calc_ig__expifor_dn8, var_fn418_calc_ig__expifor_dn13,)
    }
};
        var_fn418_calc_ig__expifor = assign33710_e30349;
        var_fn418_calc_ig__expifor_dn4 = assign33710_e30349_d_n4;
        var_fn418_calc_ig__expifor_dn8 = assign33710_e30349_d_n8;
        var_fn418_calc_ig__expifor_dn13 = assign33710_e30349_d_n13;

        let assign33720_e30352: f64 = if var_fn418_calc_ig__fracin == 1.0 { 1.0 } else { 0.0 };
        var_guard419 = assign33720_e30352;

        let (assign33730_e30366, assign33730_e30366_d_n4, assign33730_e30366_d_n8, assign33730_e30366_d_n13,) = {
    if ((var_guard417 != 0.0) && (var_guard419 != 0.0)) {
        let assign33730_e30360: f64 = (var_fn418_calc_ig__kbdgatein * var_fn418_calc_ig__iginbd);
        let assign33730_e30361: f64 = (var_fn418_calc_ig__expifor - assign33730_e30360);
        let assign33730_e30363: f64 = (assign33730_e30361 - var_fn418_calc_ig__t0);
        let assign33730_e30364: f64 = (var_fn418_calc_ig__isdiodeout * assign33730_e30363);
        (assign33730_e30364, ((var_fn418_calc_ig__isdiodeout_dn4 * assign33730_e30363) + (var_fn418_calc_ig__isdiodeout * ((var_fn418_calc_ig__expifor_dn4 - (var_fn418_calc_ig__kbdgatein * var_fn418_calc_ig__iginbd_dn4)) - var_fn418_calc_ig__t0_dn4))), (var_fn418_calc_ig__isdiodeout * (var_fn418_calc_ig__expifor_dn8 - (var_fn418_calc_ig__kbdgatein * var_fn418_calc_ig__iginbd_dn8))), (var_fn418_calc_ig__isdiodeout * (var_fn418_calc_ig__expifor_dn13 - (var_fn418_calc_ig__kbdgatein * var_fn418_calc_ig__iginbd_dn13))),)
    } else {
        (var_fn418_calc_ig__igindiode, var_fn418_calc_ig__igindiode_dn4, var_fn418_calc_ig__igindiode_dn8, var_fn418_calc_ig__igindiode_dn13,)
    }
};
        var_fn418_calc_ig__igindiode = assign33730_e30366;
        var_fn418_calc_ig__igindiode_dn4 = assign33730_e30366_d_n4;
        var_fn418_calc_ig__igindiode_dn8 = assign33730_e30366_d_n8;
        var_fn418_calc_ig__igindiode_dn13 = assign33730_e30366_d_n13;

        let (assign33740_e30380, assign33740_e30380_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard419 == 0.0)) {
        let assign33740_e30373: f64 = (-var_fn418_calc_ig__vgsatin);
        let assign33740_e30375: f64 = (assign33740_e30373 - var_fn418_calc_ig__vbdgin);
        let assign33740_e30376: f64 = (var_fn418_calc_ig__pbdgin * assign33740_e30375);
        let assign33740_e30378: f64 = (assign33740_e30376 + var_fn418_calc_ig__expphib);
        (assign33740_e30378, var_fn418_calc_ig__expphib_dn4,)
    } else {
        (var_fn418_calc_ig__expbdarg1_vgsat, var_fn418_calc_ig__expbdarg1_vgsat_dn4,)
    }
};
        var_fn418_calc_ig__expbdarg1_vgsat = assign33740_e30380;
        var_fn418_calc_ig__expbdarg1_vgsat_dn4 = assign33740_e30380_d_n4;

        let (assign33750_e30425, assign33750_e30425_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard419 == 0.0)) {
        let assign33750_e30391: f64 = (-50.0);
        let (assign33750_e30423, assign33750_e30423_d_n4,) = {
            if ((!(var_fn418_calc_ig__expbdarg1_vgsat > 50.0)) && (!(var_fn418_calc_ig__expbdarg1_vgsat < assign33750_e30391))) {
                let assign33750_e30396: f64 = (var_fn418_calc_ig__expbdarg1_vgsat).exp();
                (assign33750_e30396, (assign33750_e30396 * var_fn418_calc_ig__expbdarg1_vgsat_dn4),)
            } else {
                let assign33750_e30403: f64 = (-50.0);
                let (assign33750_e30422, assign33750_e30422_d_n4,) = {
                    if ((!(var_fn418_calc_ig__expbdarg1_vgsat > 50.0)) && (var_fn418_calc_ig__expbdarg1_vgsat < assign33750_e30403)) {
                        let assign33750_e30407: f64 = (-50.0);
                        let assign33750_e30408: f64 = (assign33750_e30407).exp();
                        (assign33750_e30408, 0.0,)
                    } else {
                        let (assign33750_e30421, assign33750_e30421_d_n4,) = {
                            if (var_fn418_calc_ig__expbdarg1_vgsat > 50.0) {
                                let assign33750_e30413: f64 = (50.0_f64).exp();
                                let assign33750_e30417: f64 = (var_fn418_calc_ig__expbdarg1_vgsat - 50.0);
                                let assign33750_e30418: f64 = (1.0 + assign33750_e30417);
                                let assign33750_e30419: f64 = (assign33750_e30413 * assign33750_e30418);
                                (assign33750_e30419, (assign33750_e30413 * var_fn418_calc_ig__expbdarg1_vgsat_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign33750_e30421, assign33750_e30421_d_n4,)
                    }
                };
                (assign33750_e30422, assign33750_e30422_d_n4,)
            }
        };
        (assign33750_e30423, assign33750_e30423_d_n4,)
    } else {
        (var_fn418_calc_ig__expbd1_vgsat, var_fn418_calc_ig__expbd1_vgsat_dn4,)
    }
};
        var_fn418_calc_ig__expbd1_vgsat = assign33750_e30425;
        var_fn418_calc_ig__expbd1_vgsat_dn4 = assign33750_e30425_d_n4;

        let (assign33760_e30434, assign33760_e30434_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard419 == 0.0)) {
        let assign33760_e30432: f64 = (var_fn418_calc_ig__expbd1_vgsat - var_fn418_calc_ig__expbd2);
        (assign33760_e30432, (var_fn418_calc_ig__expbd1_vgsat_dn4 - var_fn418_calc_ig__expbd2_dn4),)
    } else {
        (var_fn418_calc_ig__iginbd_vgsat, var_fn418_calc_ig__iginbd_vgsat_dn4,)
    }
};
        var_fn418_calc_ig__iginbd_vgsat = assign33760_e30434;
        var_fn418_calc_ig__iginbd_vgsat_dn4 = assign33760_e30434_d_n4;

        let (assign33770_e30447, assign33770_e30447_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard419 == 0.0)) {
        let assign33770_e30441: f64 = (var_fn418_calc_ig__pg_paramin / var_fn418_calc_ig__phitin);
        let assign33770_e30443: f64 = (assign33770_e30441 * var_fn418_calc_ig__vgsatin);
        let assign33770_e30445: f64 = (assign33770_e30443 + var_fn418_calc_ig__expphib);
        (assign33770_e30445, (((-((var_fn418_calc_ig__pg_paramin * var_fn418_calc_ig__phitin_dn4) / (var_fn418_calc_ig__phitin * var_fn418_calc_ig__phitin))) * var_fn418_calc_ig__vgsatin) + var_fn418_calc_ig__expphib_dn4),)
    } else {
        (var_fn418_calc_ig__expiforarg_nohinj_vgsat, var_fn418_calc_ig__expiforarg_nohinj_vgsat_dn4,)
    }
};
        var_fn418_calc_ig__expiforarg_nohinj_vgsat = assign33770_e30447;
        var_fn418_calc_ig__expiforarg_nohinj_vgsat_dn4 = assign33770_e30447_d_n4;

        let (assign33780_e30492, assign33780_e30492_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard419 == 0.0)) {
        let assign33780_e30458: f64 = (-50.0);
        let (assign33780_e30490, assign33780_e30490_d_n4,) = {
            if ((!(var_fn418_calc_ig__expiforarg_nohinj_vgsat > 50.0)) && (!(var_fn418_calc_ig__expiforarg_nohinj_vgsat < assign33780_e30458))) {
                let assign33780_e30463: f64 = (var_fn418_calc_ig__expiforarg_nohinj_vgsat).exp();
                (assign33780_e30463, (assign33780_e30463 * var_fn418_calc_ig__expiforarg_nohinj_vgsat_dn4),)
            } else {
                let assign33780_e30470: f64 = (-50.0);
                let (assign33780_e30489, assign33780_e30489_d_n4,) = {
                    if ((!(var_fn418_calc_ig__expiforarg_nohinj_vgsat > 50.0)) && (var_fn418_calc_ig__expiforarg_nohinj_vgsat < assign33780_e30470)) {
                        let assign33780_e30474: f64 = (-50.0);
                        let assign33780_e30475: f64 = (assign33780_e30474).exp();
                        (assign33780_e30475, 0.0,)
                    } else {
                        let (assign33780_e30488, assign33780_e30488_d_n4,) = {
                            if (var_fn418_calc_ig__expiforarg_nohinj_vgsat > 50.0) {
                                let assign33780_e30480: f64 = (50.0_f64).exp();
                                let assign33780_e30484: f64 = (var_fn418_calc_ig__expiforarg_nohinj_vgsat - 50.0);
                                let assign33780_e30485: f64 = (1.0 + assign33780_e30484);
                                let assign33780_e30486: f64 = (assign33780_e30480 * assign33780_e30485);
                                (assign33780_e30486, (assign33780_e30480 * var_fn418_calc_ig__expiforarg_nohinj_vgsat_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign33780_e30488, assign33780_e30488_d_n4,)
                    }
                };
                (assign33780_e30489, assign33780_e30489_d_n4,)
            }
        };
        (assign33780_e30490, assign33780_e30490_d_n4,)
    } else {
        (var_fn418_calc_ig__expifor_nohinj_vgsat, var_fn418_calc_ig__expifor_nohinj_vgsat_dn4,)
    }
};
        var_fn418_calc_ig__expifor_nohinj_vgsat = assign33780_e30492;
        var_fn418_calc_ig__expifor_nohinj_vgsat_dn4 = assign33780_e30492_d_n4;

        let (assign33790_e30505, assign33790_e30505_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard419 == 0.0)) {
        let assign33790_e30500: f64 = (var_fn418_calc_ig__kbdgatein * var_fn418_calc_ig__iginbd_vgsat);
        let assign33790_e30501: f64 = (var_fn418_calc_ig__expifor_nohinj_vgsat - assign33790_e30500);
        let assign33790_e30503: f64 = (assign33790_e30501 - var_fn418_calc_ig__t0);
        (assign33790_e30503, ((var_fn418_calc_ig__expifor_nohinj_vgsat_dn4 - (var_fn418_calc_ig__kbdgatein * var_fn418_calc_ig__iginbd_vgsat_dn4)) - var_fn418_calc_ig__t0_dn4),)
    } else {
        (var_fn418_calc_ig__igindiode_nohinj_vgsat, var_fn418_calc_ig__igindiode_nohinj_vgsat_dn4,)
    }
};
        var_fn418_calc_ig__igindiode_nohinj_vgsat = assign33790_e30505;
        var_fn418_calc_ig__igindiode_nohinj_vgsat_dn4 = assign33790_e30505_d_n4;

        let (assign33800_e30520, assign33800_e30520_d_n4, assign33800_e30520_d_n8, assign33800_e30520_d_n13,) = {
    if ((var_guard417 != 0.0) && (var_guard419 == 0.0)) {
        let assign33800_e30514: f64 = (var_fn418_calc_ig__kbdgatein * var_fn418_calc_ig__iginbd);
        let assign33800_e30515: f64 = (var_fn418_calc_ig__expifor - assign33800_e30514);
        let assign33800_e30517: f64 = (assign33800_e30515 - var_fn418_calc_ig__t0);
        let assign33800_e30518: f64 = (var_fn418_calc_ig__isdiodeout * assign33800_e30517);
        (assign33800_e30518, ((var_fn418_calc_ig__isdiodeout_dn4 * assign33800_e30517) + (var_fn418_calc_ig__isdiodeout * ((var_fn418_calc_ig__expifor_dn4 - (var_fn418_calc_ig__kbdgatein * var_fn418_calc_ig__iginbd_dn4)) - var_fn418_calc_ig__t0_dn4))), (var_fn418_calc_ig__isdiodeout * (var_fn418_calc_ig__expifor_dn8 - (var_fn418_calc_ig__kbdgatein * var_fn418_calc_ig__iginbd_dn8))), (var_fn418_calc_ig__isdiodeout * (var_fn418_calc_ig__expifor_dn13 - (var_fn418_calc_ig__kbdgatein * var_fn418_calc_ig__iginbd_dn13))),)
    } else {
        (var_fn418_calc_ig__igindiode_nohinj, var_fn418_calc_ig__igindiode_nohinj_dn4, var_fn418_calc_ig__igindiode_nohinj_dn8, var_fn418_calc_ig__igindiode_nohinj_dn13,)
    }
};
        var_fn418_calc_ig__igindiode_nohinj = assign33800_e30520;
        var_fn418_calc_ig__igindiode_nohinj_dn4 = assign33800_e30520_d_n4;
        var_fn418_calc_ig__igindiode_nohinj_dn8 = assign33800_e30520_d_n8;
        var_fn418_calc_ig__igindiode_nohinj_dn13 = assign33800_e30520_d_n13;

        let assign33810_e30523: f64 = if var_fn418_calc_ig__fracin > 0.0 { 1.0 } else { 0.0 };
        var_guard420 = assign33810_e30523;

        let (assign33820_e30534,) = {
    if (((var_guard417 != 0.0) && (var_guard419 == 0.0)) && (var_guard420 != 0.0)) {
        let assign33820_e30532: f64 = (var_fn418_calc_ig__fracin * var_fn418_calc_ig__pg_paramin);
        (assign33820_e30532,)
    } else {
        (var_fn418_calc_ig__pg_paramin_hinj,)
    }
};
        var_fn418_calc_ig__pg_paramin_hinj = assign33820_e30534;

        let (assign33830_e30549, assign33830_e30549_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard419 == 0.0)) && (var_guard420 != 0.0)) {
        let assign33830_e30543: f64 = (var_fn418_calc_ig__pg_paramin_hinj / var_fn418_calc_ig__phitin);
        let assign33830_e30545: f64 = (assign33830_e30543 * var_fn418_calc_ig__vgsatin);
        let assign33830_e30547: f64 = (assign33830_e30545 + var_fn418_calc_ig__expphib);
        (assign33830_e30547, (((-((var_fn418_calc_ig__pg_paramin_hinj * var_fn418_calc_ig__phitin_dn4) / (var_fn418_calc_ig__phitin * var_fn418_calc_ig__phitin))) * var_fn418_calc_ig__vgsatin) + var_fn418_calc_ig__expphib_dn4),)
    } else {
        (var_fn418_calc_ig__expiforarg_hinj_vgsat, var_fn418_calc_ig__expiforarg_hinj_vgsat_dn4,)
    }
};
        var_fn418_calc_ig__expiforarg_hinj_vgsat = assign33830_e30549;
        var_fn418_calc_ig__expiforarg_hinj_vgsat_dn4 = assign33830_e30549_d_n4;

        let (assign33840_e30596, assign33840_e30596_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard419 == 0.0)) && (var_guard420 != 0.0)) {
        let assign33840_e30562: f64 = (-50.0);
        let (assign33840_e30594, assign33840_e30594_d_n4,) = {
            if ((!(var_fn418_calc_ig__expiforarg_hinj_vgsat > 50.0)) && (!(var_fn418_calc_ig__expiforarg_hinj_vgsat < assign33840_e30562))) {
                let assign33840_e30567: f64 = (var_fn418_calc_ig__expiforarg_hinj_vgsat).exp();
                (assign33840_e30567, (assign33840_e30567 * var_fn418_calc_ig__expiforarg_hinj_vgsat_dn4),)
            } else {
                let assign33840_e30574: f64 = (-50.0);
                let (assign33840_e30593, assign33840_e30593_d_n4,) = {
                    if ((!(var_fn418_calc_ig__expiforarg_hinj_vgsat > 50.0)) && (var_fn418_calc_ig__expiforarg_hinj_vgsat < assign33840_e30574)) {
                        let assign33840_e30578: f64 = (-50.0);
                        let assign33840_e30579: f64 = (assign33840_e30578).exp();
                        (assign33840_e30579, 0.0,)
                    } else {
                        let (assign33840_e30592, assign33840_e30592_d_n4,) = {
                            if (var_fn418_calc_ig__expiforarg_hinj_vgsat > 50.0) {
                                let assign33840_e30584: f64 = (50.0_f64).exp();
                                let assign33840_e30588: f64 = (var_fn418_calc_ig__expiforarg_hinj_vgsat - 50.0);
                                let assign33840_e30589: f64 = (1.0 + assign33840_e30588);
                                let assign33840_e30590: f64 = (assign33840_e30584 * assign33840_e30589);
                                (assign33840_e30590, (assign33840_e30584 * var_fn418_calc_ig__expiforarg_hinj_vgsat_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign33840_e30592, assign33840_e30592_d_n4,)
                    }
                };
                (assign33840_e30593, assign33840_e30593_d_n4,)
            }
        };
        (assign33840_e30594, assign33840_e30594_d_n4,)
    } else {
        (var_fn418_calc_ig__expifor_hinj_vgsat, var_fn418_calc_ig__expifor_hinj_vgsat_dn4,)
    }
};
        var_fn418_calc_ig__expifor_hinj_vgsat = assign33840_e30596;
        var_fn418_calc_ig__expifor_hinj_vgsat_dn4 = assign33840_e30596_d_n4;

        let (assign33850_e30611, assign33850_e30611_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard419 == 0.0)) && (var_guard420 != 0.0)) {
        let assign33850_e30606: f64 = (var_fn418_calc_ig__kbdgatein * var_fn418_calc_ig__iginbd_vgsat);
        let assign33850_e30607: f64 = (var_fn418_calc_ig__expifor_hinj_vgsat - assign33850_e30606);
        let assign33850_e30609: f64 = (assign33850_e30607 - var_fn418_calc_ig__t0);
        (assign33850_e30609, ((var_fn418_calc_ig__expifor_hinj_vgsat_dn4 - (var_fn418_calc_ig__kbdgatein * var_fn418_calc_ig__iginbd_vgsat_dn4)) - var_fn418_calc_ig__t0_dn4),)
    } else {
        (var_fn418_calc_ig__igindiode_hinj_vgsat, var_fn418_calc_ig__igindiode_hinj_vgsat_dn4,)
    }
};
        var_fn418_calc_ig__igindiode_hinj_vgsat = assign33850_e30611;
        var_fn418_calc_ig__igindiode_hinj_vgsat_dn4 = assign33850_e30611_d_n4;

        *var_fn418_calc_ig__expbd1_slot = var_fn418_calc_ig__expbd1;
        *var_fn418_calc_ig__expbd1_dn13_slot = var_fn418_calc_ig__expbd1_dn13;
        *var_fn418_calc_ig__expbd1_dn4_slot = var_fn418_calc_ig__expbd1_dn4;
        *var_fn418_calc_ig__expbd1_dn8_slot = var_fn418_calc_ig__expbd1_dn8;
        *var_fn418_calc_ig__expbd1_vgsat_slot = var_fn418_calc_ig__expbd1_vgsat;
        *var_fn418_calc_ig__expbd1_vgsat_dn4_slot = var_fn418_calc_ig__expbd1_vgsat_dn4;
        *var_fn418_calc_ig__expbd2_slot = var_fn418_calc_ig__expbd2;
        *var_fn418_calc_ig__expbd2_dn4_slot = var_fn418_calc_ig__expbd2_dn4;
        *var_fn418_calc_ig__expbdarg1_slot = var_fn418_calc_ig__expbdarg1;
        *var_fn418_calc_ig__expbdarg1_dn13_slot = var_fn418_calc_ig__expbdarg1_dn13;
        *var_fn418_calc_ig__expbdarg1_dn4_slot = var_fn418_calc_ig__expbdarg1_dn4;
        *var_fn418_calc_ig__expbdarg1_dn8_slot = var_fn418_calc_ig__expbdarg1_dn8;
        *var_fn418_calc_ig__expbdarg1_vgsat_slot = var_fn418_calc_ig__expbdarg1_vgsat;
        *var_fn418_calc_ig__expbdarg1_vgsat_dn4_slot = var_fn418_calc_ig__expbdarg1_vgsat_dn4;
        *var_fn418_calc_ig__expbdarg2_slot = var_fn418_calc_ig__expbdarg2;
        *var_fn418_calc_ig__expbdarg2_dn4_slot = var_fn418_calc_ig__expbdarg2_dn4;
        *var_fn418_calc_ig__expifor_slot = var_fn418_calc_ig__expifor;
        *var_fn418_calc_ig__expifor_dn13_slot = var_fn418_calc_ig__expifor_dn13;
        *var_fn418_calc_ig__expifor_dn4_slot = var_fn418_calc_ig__expifor_dn4;
        *var_fn418_calc_ig__expifor_dn8_slot = var_fn418_calc_ig__expifor_dn8;
        *var_fn418_calc_ig__expifor_hinj_vgsat_slot = var_fn418_calc_ig__expifor_hinj_vgsat;
        *var_fn418_calc_ig__expifor_hinj_vgsat_dn4_slot = var_fn418_calc_ig__expifor_hinj_vgsat_dn4;
        *var_fn418_calc_ig__expifor_nohinj_vgsat_slot = var_fn418_calc_ig__expifor_nohinj_vgsat;
        *var_fn418_calc_ig__expifor_nohinj_vgsat_dn4_slot = var_fn418_calc_ig__expifor_nohinj_vgsat_dn4;
        *var_fn418_calc_ig__expiforarg_slot = var_fn418_calc_ig__expiforarg;
        *var_fn418_calc_ig__expiforarg_dn13_slot = var_fn418_calc_ig__expiforarg_dn13;
        *var_fn418_calc_ig__expiforarg_dn4_slot = var_fn418_calc_ig__expiforarg_dn4;
        *var_fn418_calc_ig__expiforarg_dn8_slot = var_fn418_calc_ig__expiforarg_dn8;
        *var_fn418_calc_ig__expiforarg_hinj_vgsat_slot = var_fn418_calc_ig__expiforarg_hinj_vgsat;
        *var_fn418_calc_ig__expiforarg_hinj_vgsat_dn4_slot = var_fn418_calc_ig__expiforarg_hinj_vgsat_dn4;
        *var_fn418_calc_ig__expiforarg_nohinj_vgsat_slot = var_fn418_calc_ig__expiforarg_nohinj_vgsat;
        *var_fn418_calc_ig__expiforarg_nohinj_vgsat_dn4_slot = var_fn418_calc_ig__expiforarg_nohinj_vgsat_dn4;
        *var_fn418_calc_ig__expphib_slot = var_fn418_calc_ig__expphib;
        *var_fn418_calc_ig__expphib_dn4_slot = var_fn418_calc_ig__expphib_dn4;
        *var_fn418_calc_ig__iginbd_slot = var_fn418_calc_ig__iginbd;
        *var_fn418_calc_ig__iginbd_dn13_slot = var_fn418_calc_ig__iginbd_dn13;
        *var_fn418_calc_ig__iginbd_dn4_slot = var_fn418_calc_ig__iginbd_dn4;
        *var_fn418_calc_ig__iginbd_dn8_slot = var_fn418_calc_ig__iginbd_dn8;
        *var_fn418_calc_ig__iginbd_vgsat_slot = var_fn418_calc_ig__iginbd_vgsat;
        *var_fn418_calc_ig__iginbd_vgsat_dn4_slot = var_fn418_calc_ig__iginbd_vgsat_dn4;
        *var_fn418_calc_ig__igindiode_slot = var_fn418_calc_ig__igindiode;
        *var_fn418_calc_ig__igindiode_dn13_slot = var_fn418_calc_ig__igindiode_dn13;
        *var_fn418_calc_ig__igindiode_dn4_slot = var_fn418_calc_ig__igindiode_dn4;
        *var_fn418_calc_ig__igindiode_dn8_slot = var_fn418_calc_ig__igindiode_dn8;
        *var_fn418_calc_ig__igindiode_hinj_slot = var_fn418_calc_ig__igindiode_hinj;
        *var_fn418_calc_ig__igindiode_hinj_dn13_slot = var_fn418_calc_ig__igindiode_hinj_dn13;
        *var_fn418_calc_ig__igindiode_hinj_dn4_slot = var_fn418_calc_ig__igindiode_hinj_dn4;
        *var_fn418_calc_ig__igindiode_hinj_dn8_slot = var_fn418_calc_ig__igindiode_hinj_dn8;
        *var_fn418_calc_ig__igindiode_hinj_vgsat_slot = var_fn418_calc_ig__igindiode_hinj_vgsat;
        *var_fn418_calc_ig__igindiode_hinj_vgsat_dn4_slot = var_fn418_calc_ig__igindiode_hinj_vgsat_dn4;
        *var_fn418_calc_ig__igindiode_nohinj_slot = var_fn418_calc_ig__igindiode_nohinj;
        *var_fn418_calc_ig__igindiode_nohinj_dn13_slot = var_fn418_calc_ig__igindiode_nohinj_dn13;
        *var_fn418_calc_ig__igindiode_nohinj_dn4_slot = var_fn418_calc_ig__igindiode_nohinj_dn4;
        *var_fn418_calc_ig__igindiode_nohinj_dn8_slot = var_fn418_calc_ig__igindiode_nohinj_dn8;
        *var_fn418_calc_ig__igindiode_nohinj_vgsat_slot = var_fn418_calc_ig__igindiode_nohinj_vgsat;
        *var_fn418_calc_ig__igindiode_nohinj_vgsat_dn4_slot = var_fn418_calc_ig__igindiode_nohinj_vgsat_dn4;
        *var_fn418_calc_ig__isdiodeout_slot = var_fn418_calc_ig__isdiodeout;
        *var_fn418_calc_ig__isdiodeout_dn4_slot = var_fn418_calc_ig__isdiodeout_dn4;
        *var_fn418_calc_ig__pg_paramin_hinj_slot = var_fn418_calc_ig__pg_paramin_hinj;
        *var_fn418_calc_ig__t0_slot = var_fn418_calc_ig__t0;
        *var_fn418_calc_ig__t0_dn4_slot = var_fn418_calc_ig__t0_dn4;
        *var_guard419_slot = var_guard419;
        *var_guard420_slot = var_guard420;
    }

    pub(super) fn stamp_transient_block_83(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_fn418_calc_ig__alphagin: f64,
        var_fn418_calc_ig__betarecin: f64,
        var_fn418_calc_ig__expphib: f64,
        var_fn418_calc_ig__expphib_dn4: f64,
        var_fn418_calc_ig__iginbd: f64,
        var_fn418_calc_ig__iginbd_dn13: f64,
        var_fn418_calc_ig__iginbd_dn4: f64,
        var_fn418_calc_ig__iginbd_dn8: f64,
        var_fn418_calc_ig__igindiode_hinj_vgsat: f64,
        var_fn418_calc_ig__igindiode_hinj_vgsat_dn4: f64,
        var_fn418_calc_ig__igindiode_nohinj: f64,
        var_fn418_calc_ig__igindiode_nohinj_dn13: f64,
        var_fn418_calc_ig__igindiode_nohinj_dn4: f64,
        var_fn418_calc_ig__igindiode_nohinj_dn8: f64,
        var_fn418_calc_ig__igindiode_nohinj_vgsat: f64,
        var_fn418_calc_ig__igindiode_nohinj_vgsat_dn4: f64,
        var_fn418_calc_ig__irecin: f64,
        var_fn418_calc_ig__isdiodeout: f64,
        var_fn418_calc_ig__isdiodeout_dn4: f64,
        var_fn418_calc_ig__kbdgatein: f64,
        var_fn418_calc_ig__ngf: f64,
        var_fn418_calc_ig__pg_paramin_hinj: f64,
        var_fn418_calc_ig__pgsrecin: f64,
        var_fn418_calc_ig__phitin: f64,
        var_fn418_calc_ig__phitin_dn4: f64,
        var_fn418_calc_ig__t0: f64,
        var_fn418_calc_ig__t0_dn4: f64,
        var_fn418_calc_ig__tfacdiodein: f64,
        var_fn418_calc_ig__tfacdiodein_dn4: f64,
        var_fn418_calc_ig__type: f64,
        var_fn418_calc_ig__vgin: f64,
        var_fn418_calc_ig__vgin_dn13: f64,
        var_fn418_calc_ig__vgin_dn8: f64,
        var_fn418_calc_ig__vgsatin: f64,
        var_fn418_calc_ig__vgsatqin: f64,
        var_fn418_calc_ig__w: f64,
        var_guard417: f64,
        var_guard419: f64,
        var_guard420: f64,
        var_phit: f64,
        var_phit_dn4: f64,
        var_tfacdiode: f64,
        var_tfacdiode_dn4: f64,
        var_fn418_calc_ig__alpha2_phit_slot: &mut f64,
        var_fn418_calc_ig__alpha2_phit_dn4_slot: &mut f64,
        var_fn418_calc_ig__expffvarg_slot: &mut f64,
        var_fn418_calc_ig__expffvarg_dn13_slot: &mut f64,
        var_fn418_calc_ig__expffvarg_dn4_slot: &mut f64,
        var_fn418_calc_ig__expffvarg_dn8_slot: &mut f64,
        var_fn418_calc_ig__expifor_hinj_slot: &mut f64,
        var_fn418_calc_ig__expifor_hinj_dn13_slot: &mut f64,
        var_fn418_calc_ig__expifor_hinj_dn4_slot: &mut f64,
        var_fn418_calc_ig__expifor_hinj_dn8_slot: &mut f64,
        var_fn418_calc_ig__expiforarg_hinj_slot: &mut f64,
        var_fn418_calc_ig__expiforarg_hinj_dn13_slot: &mut f64,
        var_fn418_calc_ig__expiforarg_hinj_dn4_slot: &mut f64,
        var_fn418_calc_ig__expiforarg_hinj_dn8_slot: &mut f64,
        var_fn418_calc_ig__expirev_slot: &mut f64,
        var_fn418_calc_ig__expirev_dn13_slot: &mut f64,
        var_fn418_calc_ig__expirev_dn4_slot: &mut f64,
        var_fn418_calc_ig__expirev_dn8_slot: &mut f64,
        var_fn418_calc_ig__expirevarg_slot: &mut f64,
        var_fn418_calc_ig__expirevarg_dn13_slot: &mut f64,
        var_fn418_calc_ig__expirevarg_dn4_slot: &mut f64,
        var_fn418_calc_ig__expirevarg_dn8_slot: &mut f64,
        var_fn418_calc_ig__ffvgin_slot: &mut f64,
        var_fn418_calc_ig__ffvgin_dn13_slot: &mut f64,
        var_fn418_calc_ig__ffvgin_dn4_slot: &mut f64,
        var_fn418_calc_ig__ffvgin_dn8_slot: &mut f64,
        var_fn418_calc_ig__frecgin_slot: &mut f64,
        var_fn418_calc_ig__frecgin_dn13_slot: &mut f64,
        var_fn418_calc_ig__frecgin_dn8_slot: &mut f64,
        var_fn418_calc_ig__igindiode_slot: &mut f64,
        var_fn418_calc_ig__igindiode_dn13_slot: &mut f64,
        var_fn418_calc_ig__igindiode_dn4_slot: &mut f64,
        var_fn418_calc_ig__igindiode_dn8_slot: &mut f64,
        var_fn418_calc_ig__igindiode_hinj_slot: &mut f64,
        var_fn418_calc_ig__igindiode_hinj_dn13_slot: &mut f64,
        var_fn418_calc_ig__igindiode_hinj_dn4_slot: &mut f64,
        var_fn418_calc_ig__igindiode_hinj_dn8_slot: &mut f64,
        var_fn418_calc_ig__igindiode_hinj_pre_slot: &mut f64,
        var_fn418_calc_ig__igindiode_hinj_pre_dn4_slot: &mut f64,
        var_fn418_calc_ig__iginrec_slot: &mut f64,
        var_fn418_calc_ig__iginrec_dn13_slot: &mut f64,
        var_fn418_calc_ig__iginrec_dn4_slot: &mut f64,
        var_fn418_calc_ig__iginrec_dn8_slot: &mut f64,
        var_fn418_calc_ig__igout_slot: &mut f64,
        var_fn418_calc_ig__igout_dn13_slot: &mut f64,
        var_fn418_calc_ig__igout_dn4_slot: &mut f64,
        var_fn418_calc_ig__igout_dn8_slot: &mut f64,
        var_fn418_calc_ig__isrecout_slot: &mut f64,
        var_fn418_calc_ig__isrecout_dn4_slot: &mut f64,
        var_fn418_calc_ig__return_slot: &mut f64,
        var_fn418_calc_ig__return_dn13_slot: &mut f64,
        var_fn418_calc_ig__return_dn4_slot: &mut f64,
        var_fn418_calc_ig__return_dn8_slot: &mut f64,
        var_fn423_calc_ig__alphagin_slot: &mut f64,
        var_fn423_calc_ig__fracin_slot: &mut f64,
        var_fn423_calc_ig__ijin_slot: &mut f64,
        var_fn423_calc_ig__isdiodeout_slot: &mut f64,
        var_fn423_calc_ig__isdiodeout_dn4_slot: &mut f64,
        var_fn423_calc_ig__isrecout_slot: &mut f64,
        var_fn423_calc_ig__isrecout_dn4_slot: &mut f64,
        var_fn423_calc_ig__kbdgatein_slot: &mut f64,
        var_fn423_calc_ig__ngf_slot: &mut f64,
        var_fn423_calc_ig__pbdgin_slot: &mut f64,
        var_fn423_calc_ig__pg_paramin_slot: &mut f64,
        var_fn423_calc_ig__phitin_slot: &mut f64,
        var_fn423_calc_ig__phitin_dn4_slot: &mut f64,
        var_fn423_calc_ig__return_slot: &mut f64,
        var_fn423_calc_ig__return_dn17_slot: &mut f64,
        var_fn423_calc_ig__return_dn4_slot: &mut f64,
        var_fn423_calc_ig__return_dn8_slot: &mut f64,
        var_fn423_calc_ig__tfacdiodein_slot: &mut f64,
        var_fn423_calc_ig__tfacdiodein_dn4_slot: &mut f64,
        var_fn423_calc_ig__vbdgin_slot: &mut f64,
        var_fn423_calc_ig__vgin_slot: &mut f64,
        var_fn423_calc_ig__vgin_dn17_slot: &mut f64,
        var_fn423_calc_ig__vgin_dn8_slot: &mut f64,
        var_fn423_calc_ig__vgsatin_slot: &mut f64,
        var_fn423_calc_ig__w_slot: &mut f64,
        var_guard421_slot: &mut f64,
        var_guard422_slot: &mut f64,
        var_igsi_slot: &mut f64,
        var_igsi_dn13_slot: &mut f64,
        var_igsi_dn4_slot: &mut f64,
        var_igsi_dn8_slot: &mut f64,
    ) {
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let mut var_fn418_calc_ig__alpha2_phit: f64 = *var_fn418_calc_ig__alpha2_phit_slot;
        let mut var_fn418_calc_ig__alpha2_phit_dn4: f64 = *var_fn418_calc_ig__alpha2_phit_dn4_slot;
        let mut var_fn418_calc_ig__expffvarg: f64 = *var_fn418_calc_ig__expffvarg_slot;
        let mut var_fn418_calc_ig__expffvarg_dn13: f64 = *var_fn418_calc_ig__expffvarg_dn13_slot;
        let mut var_fn418_calc_ig__expffvarg_dn4: f64 = *var_fn418_calc_ig__expffvarg_dn4_slot;
        let mut var_fn418_calc_ig__expffvarg_dn8: f64 = *var_fn418_calc_ig__expffvarg_dn8_slot;
        let mut var_fn418_calc_ig__expifor_hinj: f64 = *var_fn418_calc_ig__expifor_hinj_slot;
        let mut var_fn418_calc_ig__expifor_hinj_dn13: f64 = *var_fn418_calc_ig__expifor_hinj_dn13_slot;
        let mut var_fn418_calc_ig__expifor_hinj_dn4: f64 = *var_fn418_calc_ig__expifor_hinj_dn4_slot;
        let mut var_fn418_calc_ig__expifor_hinj_dn8: f64 = *var_fn418_calc_ig__expifor_hinj_dn8_slot;
        let mut var_fn418_calc_ig__expiforarg_hinj: f64 = *var_fn418_calc_ig__expiforarg_hinj_slot;
        let mut var_fn418_calc_ig__expiforarg_hinj_dn13: f64 = *var_fn418_calc_ig__expiforarg_hinj_dn13_slot;
        let mut var_fn418_calc_ig__expiforarg_hinj_dn4: f64 = *var_fn418_calc_ig__expiforarg_hinj_dn4_slot;
        let mut var_fn418_calc_ig__expiforarg_hinj_dn8: f64 = *var_fn418_calc_ig__expiforarg_hinj_dn8_slot;
        let mut var_fn418_calc_ig__expirev: f64 = *var_fn418_calc_ig__expirev_slot;
        let mut var_fn418_calc_ig__expirev_dn13: f64 = *var_fn418_calc_ig__expirev_dn13_slot;
        let mut var_fn418_calc_ig__expirev_dn4: f64 = *var_fn418_calc_ig__expirev_dn4_slot;
        let mut var_fn418_calc_ig__expirev_dn8: f64 = *var_fn418_calc_ig__expirev_dn8_slot;
        let mut var_fn418_calc_ig__expirevarg: f64 = *var_fn418_calc_ig__expirevarg_slot;
        let mut var_fn418_calc_ig__expirevarg_dn13: f64 = *var_fn418_calc_ig__expirevarg_dn13_slot;
        let mut var_fn418_calc_ig__expirevarg_dn4: f64 = *var_fn418_calc_ig__expirevarg_dn4_slot;
        let mut var_fn418_calc_ig__expirevarg_dn8: f64 = *var_fn418_calc_ig__expirevarg_dn8_slot;
        let mut var_fn418_calc_ig__ffvgin: f64 = *var_fn418_calc_ig__ffvgin_slot;
        let mut var_fn418_calc_ig__ffvgin_dn13: f64 = *var_fn418_calc_ig__ffvgin_dn13_slot;
        let mut var_fn418_calc_ig__ffvgin_dn4: f64 = *var_fn418_calc_ig__ffvgin_dn4_slot;
        let mut var_fn418_calc_ig__ffvgin_dn8: f64 = *var_fn418_calc_ig__ffvgin_dn8_slot;
        let mut var_fn418_calc_ig__frecgin: f64 = *var_fn418_calc_ig__frecgin_slot;
        let mut var_fn418_calc_ig__frecgin_dn13: f64 = *var_fn418_calc_ig__frecgin_dn13_slot;
        let mut var_fn418_calc_ig__frecgin_dn8: f64 = *var_fn418_calc_ig__frecgin_dn8_slot;
        let mut var_fn418_calc_ig__igindiode: f64 = *var_fn418_calc_ig__igindiode_slot;
        let mut var_fn418_calc_ig__igindiode_dn13: f64 = *var_fn418_calc_ig__igindiode_dn13_slot;
        let mut var_fn418_calc_ig__igindiode_dn4: f64 = *var_fn418_calc_ig__igindiode_dn4_slot;
        let mut var_fn418_calc_ig__igindiode_dn8: f64 = *var_fn418_calc_ig__igindiode_dn8_slot;
        let mut var_fn418_calc_ig__igindiode_hinj: f64 = *var_fn418_calc_ig__igindiode_hinj_slot;
        let mut var_fn418_calc_ig__igindiode_hinj_dn13: f64 = *var_fn418_calc_ig__igindiode_hinj_dn13_slot;
        let mut var_fn418_calc_ig__igindiode_hinj_dn4: f64 = *var_fn418_calc_ig__igindiode_hinj_dn4_slot;
        let mut var_fn418_calc_ig__igindiode_hinj_dn8: f64 = *var_fn418_calc_ig__igindiode_hinj_dn8_slot;
        let mut var_fn418_calc_ig__igindiode_hinj_pre: f64 = *var_fn418_calc_ig__igindiode_hinj_pre_slot;
        let mut var_fn418_calc_ig__igindiode_hinj_pre_dn4: f64 = *var_fn418_calc_ig__igindiode_hinj_pre_dn4_slot;
        let mut var_fn418_calc_ig__iginrec: f64 = *var_fn418_calc_ig__iginrec_slot;
        let mut var_fn418_calc_ig__iginrec_dn13: f64 = *var_fn418_calc_ig__iginrec_dn13_slot;
        let mut var_fn418_calc_ig__iginrec_dn4: f64 = *var_fn418_calc_ig__iginrec_dn4_slot;
        let mut var_fn418_calc_ig__iginrec_dn8: f64 = *var_fn418_calc_ig__iginrec_dn8_slot;
        let mut var_fn418_calc_ig__igout: f64 = *var_fn418_calc_ig__igout_slot;
        let mut var_fn418_calc_ig__igout_dn13: f64 = *var_fn418_calc_ig__igout_dn13_slot;
        let mut var_fn418_calc_ig__igout_dn4: f64 = *var_fn418_calc_ig__igout_dn4_slot;
        let mut var_fn418_calc_ig__igout_dn8: f64 = *var_fn418_calc_ig__igout_dn8_slot;
        let mut var_fn418_calc_ig__isrecout: f64 = *var_fn418_calc_ig__isrecout_slot;
        let mut var_fn418_calc_ig__isrecout_dn4: f64 = *var_fn418_calc_ig__isrecout_dn4_slot;
        let mut var_fn418_calc_ig__return: f64 = *var_fn418_calc_ig__return_slot;
        let mut var_fn418_calc_ig__return_dn13: f64 = *var_fn418_calc_ig__return_dn13_slot;
        let mut var_fn418_calc_ig__return_dn4: f64 = *var_fn418_calc_ig__return_dn4_slot;
        let mut var_fn418_calc_ig__return_dn8: f64 = *var_fn418_calc_ig__return_dn8_slot;
        let mut var_fn423_calc_ig__alphagin: f64 = *var_fn423_calc_ig__alphagin_slot;
        let mut var_fn423_calc_ig__fracin: f64 = *var_fn423_calc_ig__fracin_slot;
        let mut var_fn423_calc_ig__ijin: f64 = *var_fn423_calc_ig__ijin_slot;
        let mut var_fn423_calc_ig__isdiodeout: f64 = *var_fn423_calc_ig__isdiodeout_slot;
        let mut var_fn423_calc_ig__isdiodeout_dn4: f64 = *var_fn423_calc_ig__isdiodeout_dn4_slot;
        let mut var_fn423_calc_ig__isrecout: f64 = *var_fn423_calc_ig__isrecout_slot;
        let mut var_fn423_calc_ig__isrecout_dn4: f64 = *var_fn423_calc_ig__isrecout_dn4_slot;
        let mut var_fn423_calc_ig__kbdgatein: f64 = *var_fn423_calc_ig__kbdgatein_slot;
        let mut var_fn423_calc_ig__ngf: f64 = *var_fn423_calc_ig__ngf_slot;
        let mut var_fn423_calc_ig__pbdgin: f64 = *var_fn423_calc_ig__pbdgin_slot;
        let mut var_fn423_calc_ig__pg_paramin: f64 = *var_fn423_calc_ig__pg_paramin_slot;
        let mut var_fn423_calc_ig__phitin: f64 = *var_fn423_calc_ig__phitin_slot;
        let mut var_fn423_calc_ig__phitin_dn4: f64 = *var_fn423_calc_ig__phitin_dn4_slot;
        let mut var_fn423_calc_ig__return: f64 = *var_fn423_calc_ig__return_slot;
        let mut var_fn423_calc_ig__return_dn17: f64 = *var_fn423_calc_ig__return_dn17_slot;
        let mut var_fn423_calc_ig__return_dn4: f64 = *var_fn423_calc_ig__return_dn4_slot;
        let mut var_fn423_calc_ig__return_dn8: f64 = *var_fn423_calc_ig__return_dn8_slot;
        let mut var_fn423_calc_ig__tfacdiodein: f64 = *var_fn423_calc_ig__tfacdiodein_slot;
        let mut var_fn423_calc_ig__tfacdiodein_dn4: f64 = *var_fn423_calc_ig__tfacdiodein_dn4_slot;
        let mut var_fn423_calc_ig__vbdgin: f64 = *var_fn423_calc_ig__vbdgin_slot;
        let mut var_fn423_calc_ig__vgin: f64 = *var_fn423_calc_ig__vgin_slot;
        let mut var_fn423_calc_ig__vgin_dn17: f64 = *var_fn423_calc_ig__vgin_dn17_slot;
        let mut var_fn423_calc_ig__vgin_dn8: f64 = *var_fn423_calc_ig__vgin_dn8_slot;
        let mut var_fn423_calc_ig__vgsatin: f64 = *var_fn423_calc_ig__vgsatin_slot;
        let mut var_fn423_calc_ig__w: f64 = *var_fn423_calc_ig__w_slot;
        let mut var_guard421: f64 = *var_guard421_slot;
        let mut var_guard422: f64 = *var_guard422_slot;
        let mut var_igsi: f64 = *var_igsi_slot;
        let mut var_igsi_dn13: f64 = *var_igsi_dn13_slot;
        let mut var_igsi_dn4: f64 = *var_igsi_dn4_slot;
        let mut var_igsi_dn8: f64 = *var_igsi_dn8_slot;

        let (assign33860_e30626, assign33860_e30626_d_n4, assign33860_e30626_d_n8, assign33860_e30626_d_n13,) = {
    if (((var_guard417 != 0.0) && (var_guard419 == 0.0)) && (var_guard420 != 0.0)) {
        let assign33860_e30620: f64 = (var_fn418_calc_ig__pg_paramin_hinj / var_fn418_calc_ig__phitin);
        let assign33860_e30622: f64 = (assign33860_e30620 * var_fn418_calc_ig__vgin);
        let assign33860_e30624: f64 = (assign33860_e30622 + var_fn418_calc_ig__expphib);
        (assign33860_e30624, (((-((var_fn418_calc_ig__pg_paramin_hinj * var_fn418_calc_ig__phitin_dn4) / (var_fn418_calc_ig__phitin * var_fn418_calc_ig__phitin))) * var_fn418_calc_ig__vgin) + var_fn418_calc_ig__expphib_dn4), (assign33860_e30620 * var_fn418_calc_ig__vgin_dn8), (assign33860_e30620 * var_fn418_calc_ig__vgin_dn13),)
    } else {
        (var_fn418_calc_ig__expiforarg_hinj, var_fn418_calc_ig__expiforarg_hinj_dn4, var_fn418_calc_ig__expiforarg_hinj_dn8, var_fn418_calc_ig__expiforarg_hinj_dn13,)
    }
};
        var_fn418_calc_ig__expiforarg_hinj = assign33860_e30626;
        var_fn418_calc_ig__expiforarg_hinj_dn4 = assign33860_e30626_d_n4;
        var_fn418_calc_ig__expiforarg_hinj_dn8 = assign33860_e30626_d_n8;
        var_fn418_calc_ig__expiforarg_hinj_dn13 = assign33860_e30626_d_n13;

        let (assign33870_e30673, assign33870_e30673_d_n4, assign33870_e30673_d_n8, assign33870_e30673_d_n13,) = {
    if (((var_guard417 != 0.0) && (var_guard419 == 0.0)) && (var_guard420 != 0.0)) {
        let assign33870_e30639: f64 = (-50.0);
        let (assign33870_e30671, assign33870_e30671_d_n4, assign33870_e30671_d_n8, assign33870_e30671_d_n13,) = {
            if ((!(var_fn418_calc_ig__expiforarg_hinj > 50.0)) && (!(var_fn418_calc_ig__expiforarg_hinj < assign33870_e30639))) {
                let assign33870_e30644: f64 = (var_fn418_calc_ig__expiforarg_hinj).exp();
                (assign33870_e30644, (assign33870_e30644 * var_fn418_calc_ig__expiforarg_hinj_dn4), (assign33870_e30644 * var_fn418_calc_ig__expiforarg_hinj_dn8), (assign33870_e30644 * var_fn418_calc_ig__expiforarg_hinj_dn13),)
            } else {
                let assign33870_e30651: f64 = (-50.0);
                let (assign33870_e30670, assign33870_e30670_d_n4, assign33870_e30670_d_n8, assign33870_e30670_d_n13,) = {
                    if ((!(var_fn418_calc_ig__expiforarg_hinj > 50.0)) && (var_fn418_calc_ig__expiforarg_hinj < assign33870_e30651)) {
                        let assign33870_e30655: f64 = (-50.0);
                        let assign33870_e30656: f64 = (assign33870_e30655).exp();
                        (assign33870_e30656, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign33870_e30669, assign33870_e30669_d_n4, assign33870_e30669_d_n8, assign33870_e30669_d_n13,) = {
                            if (var_fn418_calc_ig__expiforarg_hinj > 50.0) {
                                let assign33870_e30661: f64 = (50.0_f64).exp();
                                let assign33870_e30665: f64 = (var_fn418_calc_ig__expiforarg_hinj - 50.0);
                                let assign33870_e30666: f64 = (1.0 + assign33870_e30665);
                                let assign33870_e30667: f64 = (assign33870_e30661 * assign33870_e30666);
                                (assign33870_e30667, (assign33870_e30661 * var_fn418_calc_ig__expiforarg_hinj_dn4), (assign33870_e30661 * var_fn418_calc_ig__expiforarg_hinj_dn8), (assign33870_e30661 * var_fn418_calc_ig__expiforarg_hinj_dn13),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign33870_e30669, assign33870_e30669_d_n4, assign33870_e30669_d_n8, assign33870_e30669_d_n13,)
                    }
                };
                (assign33870_e30670, assign33870_e30670_d_n4, assign33870_e30670_d_n8, assign33870_e30670_d_n13,)
            }
        };
        (assign33870_e30671, assign33870_e30671_d_n4, assign33870_e30671_d_n8, assign33870_e30671_d_n13,)
    } else {
        (var_fn418_calc_ig__expifor_hinj, var_fn418_calc_ig__expifor_hinj_dn4, var_fn418_calc_ig__expifor_hinj_dn8, var_fn418_calc_ig__expifor_hinj_dn13,)
    }
};
        var_fn418_calc_ig__expifor_hinj = assign33870_e30673;
        var_fn418_calc_ig__expifor_hinj_dn4 = assign33870_e30673_d_n4;
        var_fn418_calc_ig__expifor_hinj_dn8 = assign33870_e30673_d_n8;
        var_fn418_calc_ig__expifor_hinj_dn13 = assign33870_e30673_d_n13;

        let (assign33880_e30686, assign33880_e30686_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard419 == 0.0)) && (var_guard420 != 0.0)) {
        let assign33880_e30682: f64 = (var_fn418_calc_ig__isdiodeout * var_fn418_calc_ig__igindiode_nohinj_vgsat);
        let assign33880_e30684: f64 = (assign33880_e30682 / var_fn418_calc_ig__igindiode_hinj_vgsat);
        (assign33880_e30684, (((((var_fn418_calc_ig__isdiodeout_dn4 * var_fn418_calc_ig__igindiode_nohinj_vgsat) + (var_fn418_calc_ig__isdiodeout * var_fn418_calc_ig__igindiode_nohinj_vgsat_dn4)) * var_fn418_calc_ig__igindiode_hinj_vgsat) - (assign33880_e30682 * var_fn418_calc_ig__igindiode_hinj_vgsat_dn4)) / (var_fn418_calc_ig__igindiode_hinj_vgsat * var_fn418_calc_ig__igindiode_hinj_vgsat)),)
    } else {
        (var_fn418_calc_ig__igindiode_hinj_pre, var_fn418_calc_ig__igindiode_hinj_pre_dn4,)
    }
};
        var_fn418_calc_ig__igindiode_hinj_pre = assign33880_e30686;
        var_fn418_calc_ig__igindiode_hinj_pre_dn4 = assign33880_e30686_d_n4;

        let (assign33890_e30703, assign33890_e30703_d_n4, assign33890_e30703_d_n8, assign33890_e30703_d_n13,) = {
    if (((var_guard417 != 0.0) && (var_guard419 == 0.0)) && (var_guard420 != 0.0)) {
        let assign33890_e30697: f64 = (var_fn418_calc_ig__kbdgatein * var_fn418_calc_ig__iginbd);
        let assign33890_e30698: f64 = (var_fn418_calc_ig__expifor_hinj - assign33890_e30697);
        let assign33890_e30700: f64 = (assign33890_e30698 - var_fn418_calc_ig__t0);
        let assign33890_e30701: f64 = (var_fn418_calc_ig__igindiode_hinj_pre * assign33890_e30700);
        (assign33890_e30701, ((var_fn418_calc_ig__igindiode_hinj_pre_dn4 * assign33890_e30700) + (var_fn418_calc_ig__igindiode_hinj_pre * ((var_fn418_calc_ig__expifor_hinj_dn4 - (var_fn418_calc_ig__kbdgatein * var_fn418_calc_ig__iginbd_dn4)) - var_fn418_calc_ig__t0_dn4))), (var_fn418_calc_ig__igindiode_hinj_pre * (var_fn418_calc_ig__expifor_hinj_dn8 - (var_fn418_calc_ig__kbdgatein * var_fn418_calc_ig__iginbd_dn8))), (var_fn418_calc_ig__igindiode_hinj_pre * (var_fn418_calc_ig__expifor_hinj_dn13 - (var_fn418_calc_ig__kbdgatein * var_fn418_calc_ig__iginbd_dn13))),)
    } else {
        (var_fn418_calc_ig__igindiode_hinj, var_fn418_calc_ig__igindiode_hinj_dn4, var_fn418_calc_ig__igindiode_hinj_dn8, var_fn418_calc_ig__igindiode_hinj_dn13,)
    }
};
        var_fn418_calc_ig__igindiode_hinj = assign33890_e30703;
        var_fn418_calc_ig__igindiode_hinj_dn4 = assign33890_e30703_d_n4;
        var_fn418_calc_ig__igindiode_hinj_dn8 = assign33890_e30703_d_n8;
        var_fn418_calc_ig__igindiode_hinj_dn13 = assign33890_e30703_d_n13;

        let (assign33900_e30715, assign33900_e30715_d_n4, assign33900_e30715_d_n8, assign33900_e30715_d_n13,) = {
    if (((var_guard417 != 0.0) && (var_guard419 == 0.0)) && (var_guard420 == 0.0)) {
        let assign33900_e30713: f64 = (var_fn418_calc_ig__isdiodeout * var_fn418_calc_ig__igindiode_nohinj_vgsat);
        (assign33900_e30713, ((var_fn418_calc_ig__isdiodeout_dn4 * var_fn418_calc_ig__igindiode_nohinj_vgsat) + (var_fn418_calc_ig__isdiodeout * var_fn418_calc_ig__igindiode_nohinj_vgsat_dn4)), 0.0, 0.0,)
    } else {
        (var_fn418_calc_ig__igindiode_hinj, var_fn418_calc_ig__igindiode_hinj_dn4, var_fn418_calc_ig__igindiode_hinj_dn8, var_fn418_calc_ig__igindiode_hinj_dn13,)
    }
};
        var_fn418_calc_ig__igindiode_hinj = assign33900_e30715;
        var_fn418_calc_ig__igindiode_hinj_dn4 = assign33900_e30715_d_n4;
        var_fn418_calc_ig__igindiode_hinj_dn8 = assign33900_e30715_d_n8;
        var_fn418_calc_ig__igindiode_hinj_dn13 = assign33900_e30715_d_n13;

        let (assign33910_e30726, assign33910_e30726_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard419 == 0.0)) {
        let assign33910_e30722: f64 = (var_fn418_calc_ig__alphagin * var_fn418_calc_ig__alphagin);
        let assign33910_e30724: f64 = (assign33910_e30722 * var_fn418_calc_ig__phitin);
        (assign33910_e30724, (assign33910_e30722 * var_fn418_calc_ig__phitin_dn4),)
    } else {
        (var_fn418_calc_ig__alpha2_phit, var_fn418_calc_ig__alpha2_phit_dn4,)
    }
};
        var_fn418_calc_ig__alpha2_phit = assign33910_e30726;
        var_fn418_calc_ig__alpha2_phit_dn4 = assign33910_e30726_d_n4;

        let (assign33920_e30741, assign33920_e30741_d_n4, assign33920_e30741_d_n8, assign33920_e30741_d_n13,) = {
    if ((var_guard417 != 0.0) && (var_guard419 == 0.0)) {
        let assign33920_e30735: f64 = (var_fn418_calc_ig__alpha2_phit / 2.0);
        let assign33920_e30736: f64 = (var_fn418_calc_ig__vgsatin - assign33920_e30735);
        let assign33920_e30737: f64 = (var_fn418_calc_ig__vgin - assign33920_e30736);
        let assign33920_e30739: f64 = (assign33920_e30737 / var_fn418_calc_ig__alpha2_phit);
        (assign33920_e30739, ((((-(-(var_fn418_calc_ig__alpha2_phit_dn4 / 2.0))) * var_fn418_calc_ig__alpha2_phit) - (assign33920_e30737 * var_fn418_calc_ig__alpha2_phit_dn4)) / (var_fn418_calc_ig__alpha2_phit * var_fn418_calc_ig__alpha2_phit)), (var_fn418_calc_ig__vgin_dn8 / var_fn418_calc_ig__alpha2_phit), (var_fn418_calc_ig__vgin_dn13 / var_fn418_calc_ig__alpha2_phit),)
    } else {
        (var_fn418_calc_ig__expffvarg, var_fn418_calc_ig__expffvarg_dn4, var_fn418_calc_ig__expffvarg_dn8, var_fn418_calc_ig__expffvarg_dn13,)
    }
};
        var_fn418_calc_ig__expffvarg = assign33920_e30741;
        var_fn418_calc_ig__expffvarg_dn4 = assign33920_e30741_d_n4;
        var_fn418_calc_ig__expffvarg_dn8 = assign33920_e30741_d_n8;
        var_fn418_calc_ig__expffvarg_dn13 = assign33920_e30741_d_n13;

        let assign33930_e30744: f64 = if var_fn418_calc_ig__expffvarg > 50.0 { 1.0 } else { 0.0 };
        var_guard421 = assign33930_e30744;

        let (assign33940_e30753, assign33940_e30753_d_n4, assign33940_e30753_d_n8, assign33940_e30753_d_n13,) = {
    if (((var_guard417 != 0.0) && (var_guard419 == 0.0)) && (var_guard421 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn418_calc_ig__ffvgin, var_fn418_calc_ig__ffvgin_dn4, var_fn418_calc_ig__ffvgin_dn8, var_fn418_calc_ig__ffvgin_dn13,)
    }
};
        var_fn418_calc_ig__ffvgin = assign33940_e30753;
        var_fn418_calc_ig__ffvgin_dn4 = assign33940_e30753_d_n4;
        var_fn418_calc_ig__ffvgin_dn8 = assign33940_e30753_d_n8;
        var_fn418_calc_ig__ffvgin_dn13 = assign33940_e30753_d_n13;

        let assign33950_e30756: f64 = (-50.0);
        let assign33950_e30757: f64 = if var_fn418_calc_ig__expffvarg < assign33950_e30756 { 1.0 } else { 0.0 };
        var_guard422 = assign33950_e30757;

        let (assign33960_e30769, assign33960_e30769_d_n4, assign33960_e30769_d_n8, assign33960_e30769_d_n13,) = {
    if ((((var_guard417 != 0.0) && (var_guard419 == 0.0)) && (var_guard421 == 0.0)) && (var_guard422 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn418_calc_ig__ffvgin, var_fn418_calc_ig__ffvgin_dn4, var_fn418_calc_ig__ffvgin_dn8, var_fn418_calc_ig__ffvgin_dn13,)
    }
};
        var_fn418_calc_ig__ffvgin = assign33960_e30769;
        var_fn418_calc_ig__ffvgin_dn4 = assign33960_e30769_d_n4;
        var_fn418_calc_ig__ffvgin_dn8 = assign33960_e30769_d_n8;
        var_fn418_calc_ig__ffvgin_dn13 = assign33960_e30769_d_n13;

        let (assign33970_e30787, assign33970_e30787_d_n4, assign33970_e30787_d_n8, assign33970_e30787_d_n13,) = {
    if ((((var_guard417 != 0.0) && (var_guard419 == 0.0)) && (var_guard421 == 0.0)) && (var_guard422 == 0.0)) {
        let assign33970_e30783: f64 = (var_fn418_calc_ig__expffvarg).exp();
        let assign33970_e30784: f64 = (1.0 + assign33970_e30783);
        let assign33970_e30785: f64 = (1.0 / assign33970_e30784);
        (assign33970_e30785, (-((assign33970_e30783 * var_fn418_calc_ig__expffvarg_dn4) / (assign33970_e30784 * assign33970_e30784))), (-((assign33970_e30783 * var_fn418_calc_ig__expffvarg_dn8) / (assign33970_e30784 * assign33970_e30784))), (-((assign33970_e30783 * var_fn418_calc_ig__expffvarg_dn13) / (assign33970_e30784 * assign33970_e30784))),)
    } else {
        (var_fn418_calc_ig__ffvgin, var_fn418_calc_ig__ffvgin_dn4, var_fn418_calc_ig__ffvgin_dn8, var_fn418_calc_ig__ffvgin_dn13,)
    }
};
        var_fn418_calc_ig__ffvgin = assign33970_e30787;
        var_fn418_calc_ig__ffvgin_dn4 = assign33970_e30787_d_n4;
        var_fn418_calc_ig__ffvgin_dn8 = assign33970_e30787_d_n8;
        var_fn418_calc_ig__ffvgin_dn13 = assign33970_e30787_d_n13;

        let (assign33980_e30802, assign33980_e30802_d_n4, assign33980_e30802_d_n8, assign33980_e30802_d_n13,) = {
    if ((var_guard417 != 0.0) && (var_guard419 == 0.0)) {
        let assign33980_e30794: f64 = (var_fn418_calc_ig__ffvgin * var_fn418_calc_ig__igindiode_nohinj);
        let assign33980_e30797: f64 = (1.0 - var_fn418_calc_ig__ffvgin);
        let assign33980_e30799: f64 = (assign33980_e30797 * var_fn418_calc_ig__igindiode_hinj);
        let assign33980_e30800: f64 = (assign33980_e30794 + assign33980_e30799);
        (assign33980_e30800, (((var_fn418_calc_ig__ffvgin_dn4 * var_fn418_calc_ig__igindiode_nohinj) + (var_fn418_calc_ig__ffvgin * var_fn418_calc_ig__igindiode_nohinj_dn4)) + (((-var_fn418_calc_ig__ffvgin_dn4) * var_fn418_calc_ig__igindiode_hinj) + (assign33980_e30797 * var_fn418_calc_ig__igindiode_hinj_dn4))), (((var_fn418_calc_ig__ffvgin_dn8 * var_fn418_calc_ig__igindiode_nohinj) + (var_fn418_calc_ig__ffvgin * var_fn418_calc_ig__igindiode_nohinj_dn8)) + (((-var_fn418_calc_ig__ffvgin_dn8) * var_fn418_calc_ig__igindiode_hinj) + (assign33980_e30797 * var_fn418_calc_ig__igindiode_hinj_dn8))), (((var_fn418_calc_ig__ffvgin_dn13 * var_fn418_calc_ig__igindiode_nohinj) + (var_fn418_calc_ig__ffvgin * var_fn418_calc_ig__igindiode_nohinj_dn13)) + (((-var_fn418_calc_ig__ffvgin_dn13) * var_fn418_calc_ig__igindiode_hinj) + (assign33980_e30797 * var_fn418_calc_ig__igindiode_hinj_dn13))),)
    } else {
        (var_fn418_calc_ig__igindiode, var_fn418_calc_ig__igindiode_dn4, var_fn418_calc_ig__igindiode_dn8, var_fn418_calc_ig__igindiode_dn13,)
    }
};
        var_fn418_calc_ig__igindiode = assign33980_e30802;
        var_fn418_calc_ig__igindiode_dn4 = assign33980_e30802_d_n4;
        var_fn418_calc_ig__igindiode_dn8 = assign33980_e30802_d_n8;
        var_fn418_calc_ig__igindiode_dn13 = assign33980_e30802_d_n13;

        let (assign33990_e30848, assign33990_e30848_d_n8, assign33990_e30848_d_n13,) = {
    if (var_guard417 != 0.0) {
        let assign33990_e30805: f64 = (-var_fn418_calc_ig__vgin);
        let (assign33990_e30838, assign33990_e30838_d_n8, assign33990_e30838_d_n13,) = {
            if (p.p52 != 0.0) {
                let assign33990_e30813: f64 = (var_fn418_calc_ig__vgin / var_fn418_calc_ig__vgsatqin);
                let assign33990_e30816: f64 = (0.001 / p.p53);
                let assign33990_e30819: f64 = (var_fn418_calc_ig__vgin / var_fn418_calc_ig__vgsatqin);
                let assign33990_e30820: f64 = (assign33990_e30816 * assign33990_e30819);
                let assign33990_e30821: f64 = (assign33990_e30820).tanh();
                let assign33990_e30822: f64 = (assign33990_e30813 * assign33990_e30821);
                (assign33990_e30822, (((var_fn418_calc_ig__vgin_dn8 / var_fn418_calc_ig__vgsatqin) * assign33990_e30821) + (assign33990_e30813 * ((assign33990_e30816 * (var_fn418_calc_ig__vgin_dn8 / var_fn418_calc_ig__vgsatqin)) / ((assign33990_e30820).cosh() * (assign33990_e30820).cosh())))), (((var_fn418_calc_ig__vgin_dn13 / var_fn418_calc_ig__vgsatqin) * assign33990_e30821) + (assign33990_e30813 * ((assign33990_e30816 * (var_fn418_calc_ig__vgin_dn13 / var_fn418_calc_ig__vgsatqin)) / ((assign33990_e30820).cosh() * (assign33990_e30820).cosh())))),)
            } else {
                let (assign33990_e30837, assign33990_e30837_d_n8, assign33990_e30837_d_n13,) = {
                    if (p.p52 == 0.0) {
                        let __rspice_inv_cse_0: f64 = 1.0 / var_fn418_calc_ig__vgsatqin;
                        let assign33990_e30828: f64 = (var_fn418_calc_ig__vgin * __rspice_inv_cse_0);
                        let assign33990_e30831: f64 = (var_fn418_calc_ig__vgin * __rspice_inv_cse_0);
                        let assign33990_e30832: f64 = (assign33990_e30828 * assign33990_e30831);
                        let assign33990_e30834: f64 = (assign33990_e30832 + p.p53);
                        let assign33990_e30835: f64 = (assign33990_e30834).sqrt();
                        (assign33990_e30835, ((((var_fn418_calc_ig__vgin_dn8 / var_fn418_calc_ig__vgsatqin) * assign33990_e30831) + (assign33990_e30828 * (var_fn418_calc_ig__vgin_dn8 / var_fn418_calc_ig__vgsatqin))) / (2.0 * assign33990_e30835)), ((((var_fn418_calc_ig__vgin_dn13 / var_fn418_calc_ig__vgsatqin) * assign33990_e30831) + (assign33990_e30828 * (var_fn418_calc_ig__vgin_dn13 / var_fn418_calc_ig__vgsatqin))) / (2.0 * assign33990_e30835)),)
                    } else {
                        (0.0, 0.0, 0.0,)
                    }
                };
                (assign33990_e30837, assign33990_e30837_d_n8, assign33990_e30837_d_n13,)
            }
        };
        let assign33990_e30840: f64 = (assign33990_e30838).powf(var_fn418_calc_ig__betarecin);
        let assign33990_e30841: f64 = (1.0 + assign33990_e30840);
        let assign33990_e30844: f64 = (1.0 / var_fn418_calc_ig__betarecin);
        let assign33990_e30845: f64 = (assign33990_e30841).powf(assign33990_e30844);
        let assign33990_e30846: f64 = (assign33990_e30805 / assign33990_e30845);
        (assign33990_e30846, ((((-var_fn418_calc_ig__vgin_dn8) * assign33990_e30845) - (assign33990_e30805 * if 0.0 == 0.0 && ((assign33990_e30844) as f64).is_finite() && ((assign33990_e30844) as f64).fract() == 0.0 { if assign33990_e30844 == 0.0 { 0.0 } else { (assign33990_e30844 * ((assign33990_e30841).powf(assign33990_e30844 - 1.0) * if 0.0 == 0.0 && ((var_fn418_calc_ig__betarecin) as f64).is_finite() && ((var_fn418_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn418_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn418_calc_ig__betarecin * ((assign33990_e30838).powf(var_fn418_calc_ig__betarecin - 1.0) * assign33990_e30838_d_n8)) } } else { (assign33990_e30840 * (var_fn418_calc_ig__betarecin * (assign33990_e30838_d_n8 / assign33990_e30838))) })) } } else { (assign33990_e30845 * (assign33990_e30844 * (if 0.0 == 0.0 && ((var_fn418_calc_ig__betarecin) as f64).is_finite() && ((var_fn418_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn418_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn418_calc_ig__betarecin * ((assign33990_e30838).powf(var_fn418_calc_ig__betarecin - 1.0) * assign33990_e30838_d_n8)) } } else { (assign33990_e30840 * (var_fn418_calc_ig__betarecin * (assign33990_e30838_d_n8 / assign33990_e30838))) } / assign33990_e30841))) })) / (assign33990_e30845 * assign33990_e30845)), ((((-var_fn418_calc_ig__vgin_dn13) * assign33990_e30845) - (assign33990_e30805 * if 0.0 == 0.0 && ((assign33990_e30844) as f64).is_finite() && ((assign33990_e30844) as f64).fract() == 0.0 { if assign33990_e30844 == 0.0 { 0.0 } else { (assign33990_e30844 * ((assign33990_e30841).powf(assign33990_e30844 - 1.0) * if 0.0 == 0.0 && ((var_fn418_calc_ig__betarecin) as f64).is_finite() && ((var_fn418_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn418_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn418_calc_ig__betarecin * ((assign33990_e30838).powf(var_fn418_calc_ig__betarecin - 1.0) * assign33990_e30838_d_n13)) } } else { (assign33990_e30840 * (var_fn418_calc_ig__betarecin * (assign33990_e30838_d_n13 / assign33990_e30838))) })) } } else { (assign33990_e30845 * (assign33990_e30844 * (if 0.0 == 0.0 && ((var_fn418_calc_ig__betarecin) as f64).is_finite() && ((var_fn418_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn418_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn418_calc_ig__betarecin * ((assign33990_e30838).powf(var_fn418_calc_ig__betarecin - 1.0) * assign33990_e30838_d_n13)) } } else { (assign33990_e30840 * (var_fn418_calc_ig__betarecin * (assign33990_e30838_d_n13 / assign33990_e30838))) } / assign33990_e30841))) })) / (assign33990_e30845 * assign33990_e30845)),)
    } else {
        (var_fn418_calc_ig__frecgin, var_fn418_calc_ig__frecgin_dn8, var_fn418_calc_ig__frecgin_dn13,)
    }
};
        var_fn418_calc_ig__frecgin = assign33990_e30848;
        var_fn418_calc_ig__frecgin_dn8 = assign33990_e30848_d_n8;
        var_fn418_calc_ig__frecgin_dn13 = assign33990_e30848_d_n13;

        let (assign34000_e30863, assign34000_e30863_d_n4,) = {
    if (var_guard417 != 0.0) {
        let assign34000_e30851: f64 = (-var_fn418_calc_ig__type);
        let assign34000_e30853: f64 = (assign34000_e30851 * var_fn418_calc_ig__w);
        let assign34000_e30855: f64 = (assign34000_e30853 * var_fn418_calc_ig__ngf);
        let assign34000_e30857: f64 = (assign34000_e30855 * var_fn418_calc_ig__irecin);
        let assign34000_e30859: f64 = (assign34000_e30857 * var_fn418_calc_ig__tfacdiodein);
        let assign34000_e30861: f64 = assign34000_e30859;
        (assign34000_e30861, (assign34000_e30857 * var_fn418_calc_ig__tfacdiodein_dn4),)
    } else {
        (var_fn418_calc_ig__isrecout, var_fn418_calc_ig__isrecout_dn4,)
    }
};
        var_fn418_calc_ig__isrecout = assign34000_e30863;
        var_fn418_calc_ig__isrecout_dn4 = assign34000_e30863_d_n4;

        let (assign34010_e30871, assign34010_e30871_d_n4, assign34010_e30871_d_n8, assign34010_e30871_d_n13,) = {
    if (var_guard417 != 0.0) {
        let assign34010_e30867: f64 = (var_fn418_calc_ig__pgsrecin / var_fn418_calc_ig__phitin);
        let assign34010_e30869: f64 = (assign34010_e30867 * var_fn418_calc_ig__frecgin);
        (assign34010_e30869, ((-((var_fn418_calc_ig__pgsrecin * var_fn418_calc_ig__phitin_dn4) / (var_fn418_calc_ig__phitin * var_fn418_calc_ig__phitin))) * var_fn418_calc_ig__frecgin), (assign34010_e30867 * var_fn418_calc_ig__frecgin_dn8), (assign34010_e30867 * var_fn418_calc_ig__frecgin_dn13),)
    } else {
        (var_fn418_calc_ig__expirevarg, var_fn418_calc_ig__expirevarg_dn4, var_fn418_calc_ig__expirevarg_dn8, var_fn418_calc_ig__expirevarg_dn13,)
    }
};
        var_fn418_calc_ig__expirevarg = assign34010_e30871;
        var_fn418_calc_ig__expirevarg_dn4 = assign34010_e30871_d_n4;
        var_fn418_calc_ig__expirevarg_dn8 = assign34010_e30871_d_n8;
        var_fn418_calc_ig__expirevarg_dn13 = assign34010_e30871_d_n13;

        let (assign34020_e30913, assign34020_e30913_d_n4, assign34020_e30913_d_n8, assign34020_e30913_d_n13,) = {
    if (var_guard417 != 0.0) {
        let assign34020_e30879: f64 = (-50.0);
        let (assign34020_e30911, assign34020_e30911_d_n4, assign34020_e30911_d_n8, assign34020_e30911_d_n13,) = {
            if ((!(var_fn418_calc_ig__expirevarg > 50.0)) && (!(var_fn418_calc_ig__expirevarg < assign34020_e30879))) {
                let assign34020_e30884: f64 = (var_fn418_calc_ig__expirevarg).exp();
                (assign34020_e30884, (assign34020_e30884 * var_fn418_calc_ig__expirevarg_dn4), (assign34020_e30884 * var_fn418_calc_ig__expirevarg_dn8), (assign34020_e30884 * var_fn418_calc_ig__expirevarg_dn13),)
            } else {
                let assign34020_e30891: f64 = (-50.0);
                let (assign34020_e30910, assign34020_e30910_d_n4, assign34020_e30910_d_n8, assign34020_e30910_d_n13,) = {
                    if ((!(var_fn418_calc_ig__expirevarg > 50.0)) && (var_fn418_calc_ig__expirevarg < assign34020_e30891)) {
                        let assign34020_e30895: f64 = (-50.0);
                        let assign34020_e30896: f64 = (assign34020_e30895).exp();
                        (assign34020_e30896, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign34020_e30909, assign34020_e30909_d_n4, assign34020_e30909_d_n8, assign34020_e30909_d_n13,) = {
                            if (var_fn418_calc_ig__expirevarg > 50.0) {
                                let assign34020_e30901: f64 = (50.0_f64).exp();
                                let assign34020_e30905: f64 = (var_fn418_calc_ig__expirevarg - 50.0);
                                let assign34020_e30906: f64 = (1.0 + assign34020_e30905);
                                let assign34020_e30907: f64 = (assign34020_e30901 * assign34020_e30906);
                                (assign34020_e30907, (assign34020_e30901 * var_fn418_calc_ig__expirevarg_dn4), (assign34020_e30901 * var_fn418_calc_ig__expirevarg_dn8), (assign34020_e30901 * var_fn418_calc_ig__expirevarg_dn13),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign34020_e30909, assign34020_e30909_d_n4, assign34020_e30909_d_n8, assign34020_e30909_d_n13,)
                    }
                };
                (assign34020_e30910, assign34020_e30910_d_n4, assign34020_e30910_d_n8, assign34020_e30910_d_n13,)
            }
        };
        (assign34020_e30911, assign34020_e30911_d_n4, assign34020_e30911_d_n8, assign34020_e30911_d_n13,)
    } else {
        (var_fn418_calc_ig__expirev, var_fn418_calc_ig__expirev_dn4, var_fn418_calc_ig__expirev_dn8, var_fn418_calc_ig__expirev_dn13,)
    }
};
        var_fn418_calc_ig__expirev = assign34020_e30913;
        var_fn418_calc_ig__expirev_dn4 = assign34020_e30913_d_n4;
        var_fn418_calc_ig__expirev_dn8 = assign34020_e30913_d_n8;
        var_fn418_calc_ig__expirev_dn13 = assign34020_e30913_d_n13;

        let (assign34030_e30921, assign34030_e30921_d_n4, assign34030_e30921_d_n8, assign34030_e30921_d_n13,) = {
    if (var_guard417 != 0.0) {
        let assign34030_e30918: f64 = (var_fn418_calc_ig__expirev - 1.0);
        let assign34030_e30919: f64 = (var_fn418_calc_ig__isrecout * assign34030_e30918);
        (assign34030_e30919, ((var_fn418_calc_ig__isrecout_dn4 * assign34030_e30918) + (var_fn418_calc_ig__isrecout * var_fn418_calc_ig__expirev_dn4)), (var_fn418_calc_ig__isrecout * var_fn418_calc_ig__expirev_dn8), (var_fn418_calc_ig__isrecout * var_fn418_calc_ig__expirev_dn13),)
    } else {
        (var_fn418_calc_ig__iginrec, var_fn418_calc_ig__iginrec_dn4, var_fn418_calc_ig__iginrec_dn8, var_fn418_calc_ig__iginrec_dn13,)
    }
};
        var_fn418_calc_ig__iginrec = assign34030_e30921;
        var_fn418_calc_ig__iginrec_dn4 = assign34030_e30921_d_n4;
        var_fn418_calc_ig__iginrec_dn8 = assign34030_e30921_d_n8;
        var_fn418_calc_ig__iginrec_dn13 = assign34030_e30921_d_n13;

        let (assign34040_e30927, assign34040_e30927_d_n4, assign34040_e30927_d_n8, assign34040_e30927_d_n13,) = {
    if (var_guard417 != 0.0) {
        let assign34040_e30925: f64 = (var_fn418_calc_ig__igindiode + var_fn418_calc_ig__iginrec);
        (assign34040_e30925, (var_fn418_calc_ig__igindiode_dn4 + var_fn418_calc_ig__iginrec_dn4), (var_fn418_calc_ig__igindiode_dn8 + var_fn418_calc_ig__iginrec_dn8), (var_fn418_calc_ig__igindiode_dn13 + var_fn418_calc_ig__iginrec_dn13),)
    } else {
        (var_fn418_calc_ig__igout, var_fn418_calc_ig__igout_dn4, var_fn418_calc_ig__igout_dn8, var_fn418_calc_ig__igout_dn13,)
    }
};
        var_fn418_calc_ig__igout = assign34040_e30927;
        var_fn418_calc_ig__igout_dn4 = assign34040_e30927_d_n4;
        var_fn418_calc_ig__igout_dn8 = assign34040_e30927_d_n8;
        var_fn418_calc_ig__igout_dn13 = assign34040_e30927_d_n13;

        let (assign34050_e30931, assign34050_e30931_d_n4, assign34050_e30931_d_n8, assign34050_e30931_d_n13,) = {
    if (var_guard417 != 0.0) {
        (var_fn418_calc_ig__igout, var_fn418_calc_ig__igout_dn4, var_fn418_calc_ig__igout_dn8, var_fn418_calc_ig__igout_dn13,)
    } else {
        (var_fn418_calc_ig__return, var_fn418_calc_ig__return_dn4, var_fn418_calc_ig__return_dn8, var_fn418_calc_ig__return_dn13,)
    }
};
        var_fn418_calc_ig__return = assign34050_e30931;
        var_fn418_calc_ig__return_dn4 = assign34050_e30931_d_n4;
        var_fn418_calc_ig__return_dn8 = assign34050_e30931_d_n8;
        var_fn418_calc_ig__return_dn13 = assign34050_e30931_d_n13;

        let (assign34080_e30943, assign34080_e30943_d_n4, assign34080_e30943_d_n8, assign34080_e30943_d_n13,) = {
    if (var_guard417 != 0.0) {
        (var_fn418_calc_ig__return, var_fn418_calc_ig__return_dn4, var_fn418_calc_ig__return_dn8, var_fn418_calc_ig__return_dn13,)
    } else {
        (var_igsi, var_igsi_dn4, var_igsi_dn8, var_igsi_dn13,)
    }
};
        var_igsi = assign34080_e30943;
        var_igsi_dn4 = assign34080_e30943_d_n4;
        var_igsi_dn8 = assign34080_e30943_d_n8;
        var_igsi_dn13 = assign34080_e30943_d_n13;

        let (assign34090_e30947, assign34090_e30947_d_n4, assign34090_e30947_d_n8, assign34090_e30947_d_n17,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn423_calc_ig__return, var_fn423_calc_ig__return_dn4, var_fn423_calc_ig__return_dn8, var_fn423_calc_ig__return_dn17,)
    }
};
        var_fn423_calc_ig__return = assign34090_e30947;
        var_fn423_calc_ig__return_dn4 = assign34090_e30947_d_n4;
        var_fn423_calc_ig__return_dn8 = assign34090_e30947_d_n8;
        var_fn423_calc_ig__return_dn17 = assign34090_e30947_d_n17;

        let (assign34100_e30951, assign34100_e30951_d_n4,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn423_calc_ig__isdiodeout, var_fn423_calc_ig__isdiodeout_dn4,)
    }
};
        var_fn423_calc_ig__isdiodeout = assign34100_e30951;
        var_fn423_calc_ig__isdiodeout_dn4 = assign34100_e30951_d_n4;

        let (assign34110_e30955, assign34110_e30955_d_n4,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn423_calc_ig__isrecout, var_fn423_calc_ig__isrecout_dn4,)
    }
};
        var_fn423_calc_ig__isrecout = assign34110_e30955;
        var_fn423_calc_ig__isrecout_dn4 = assign34110_e30955_d_n4;

        let (assign34120_e30961, assign34120_e30961_d_n8, assign34120_e30961_d_n17,) = {
    if (var_guard417 != 0.0) {
        let assign34120_e30959: f64 = (p.p6 * (nv8 - nv17));
        (assign34120_e30959, p.p6, (-p.p6),)
    } else {
        (var_fn423_calc_ig__vgin, var_fn423_calc_ig__vgin_dn8, var_fn423_calc_ig__vgin_dn17,)
    }
};
        var_fn423_calc_ig__vgin = assign34120_e30961;
        var_fn423_calc_ig__vgin_dn8 = assign34120_e30961_d_n8;
        var_fn423_calc_ig__vgin_dn17 = assign34120_e30961_d_n17;

        let (assign34130_e30965, assign34130_e30965_d_n4,) = {
    if (var_guard417 != 0.0) {
        (var_phit, var_phit_dn4,)
    } else {
        (var_fn423_calc_ig__phitin, var_fn423_calc_ig__phitin_dn4,)
    }
};
        var_fn423_calc_ig__phitin = assign34130_e30965;
        var_fn423_calc_ig__phitin_dn4 = assign34130_e30965_d_n4;

        let (assign34140_e30969,) = {
    if (var_guard417 != 0.0) {
        (p.p265,)
    } else {
        (var_fn423_calc_ig__vgsatin,)
    }
};
        var_fn423_calc_ig__vgsatin = assign34140_e30969;

        let (assign34150_e30973,) = {
    if (var_guard417 != 0.0) {
        (p.p267,)
    } else {
        (var_fn423_calc_ig__alphagin,)
    }
};
        var_fn423_calc_ig__alphagin = assign34150_e30973;

        let (assign34160_e30977,) = {
    if (var_guard417 != 0.0) {
        (p.p266,)
    } else {
        (var_fn423_calc_ig__fracin,)
    }
};
        var_fn423_calc_ig__fracin = assign34160_e30977;

        let (assign34170_e30981,) = {
    if (var_guard417 != 0.0) {
        (p.p263,)
    } else {
        (var_fn423_calc_ig__pg_paramin,)
    }
};
        var_fn423_calc_ig__pg_paramin = assign34170_e30981;

        let (assign34180_e30985,) = {
    if (var_guard417 != 0.0) {
        (p.p281,)
    } else {
        (var_fn423_calc_ig__pbdgin,)
    }
};
        var_fn423_calc_ig__pbdgin = assign34180_e30985;

        let (assign34190_e30989,) = {
    if (var_guard417 != 0.0) {
        (p.p280,)
    } else {
        (var_fn423_calc_ig__vbdgin,)
    }
};
        var_fn423_calc_ig__vbdgin = assign34190_e30989;

        let (assign34200_e30993, assign34200_e30993_d_n4,) = {
    if (var_guard417 != 0.0) {
        (var_tfacdiode, var_tfacdiode_dn4,)
    } else {
        (var_fn423_calc_ig__tfacdiodein, var_fn423_calc_ig__tfacdiodein_dn4,)
    }
};
        var_fn423_calc_ig__tfacdiodein = assign34200_e30993;
        var_fn423_calc_ig__tfacdiodein_dn4 = assign34200_e30993_d_n4;

        let (assign34210_e30997,) = {
    if (var_guard417 != 0.0) {
        (p.p0,)
    } else {
        (var_fn423_calc_ig__w,)
    }
};
        var_fn423_calc_ig__w = assign34210_e30997;

        let (assign34220_e31001,) = {
    if (var_guard417 != 0.0) {
        (p.p2,)
    } else {
        (var_fn423_calc_ig__ngf,)
    }
};
        var_fn423_calc_ig__ngf = assign34220_e31001;

        let (assign34230_e31009,) = {
    if (var_guard417 != 0.0) {
        let assign34230_e31005: f64 = (1.0 - p.p255);
        let assign34230_e31007: f64 = (assign34230_e31005 * p.p264);
        (assign34230_e31007,)
    } else {
        (var_fn423_calc_ig__ijin,)
    }
};
        var_fn423_calc_ig__ijin = assign34230_e31009;

        let (assign34240_e31013,) = {
    if (var_guard417 != 0.0) {
        (p.p279,)
    } else {
        (var_fn423_calc_ig__kbdgatein,)
    }
};
        var_fn423_calc_ig__kbdgatein = assign34240_e31013;

        *var_fn418_calc_ig__alpha2_phit_slot = var_fn418_calc_ig__alpha2_phit;
        *var_fn418_calc_ig__alpha2_phit_dn4_slot = var_fn418_calc_ig__alpha2_phit_dn4;
        *var_fn418_calc_ig__expffvarg_slot = var_fn418_calc_ig__expffvarg;
        *var_fn418_calc_ig__expffvarg_dn13_slot = var_fn418_calc_ig__expffvarg_dn13;
        *var_fn418_calc_ig__expffvarg_dn4_slot = var_fn418_calc_ig__expffvarg_dn4;
        *var_fn418_calc_ig__expffvarg_dn8_slot = var_fn418_calc_ig__expffvarg_dn8;
        *var_fn418_calc_ig__expifor_hinj_slot = var_fn418_calc_ig__expifor_hinj;
        *var_fn418_calc_ig__expifor_hinj_dn13_slot = var_fn418_calc_ig__expifor_hinj_dn13;
        *var_fn418_calc_ig__expifor_hinj_dn4_slot = var_fn418_calc_ig__expifor_hinj_dn4;
        *var_fn418_calc_ig__expifor_hinj_dn8_slot = var_fn418_calc_ig__expifor_hinj_dn8;
        *var_fn418_calc_ig__expiforarg_hinj_slot = var_fn418_calc_ig__expiforarg_hinj;
        *var_fn418_calc_ig__expiforarg_hinj_dn13_slot = var_fn418_calc_ig__expiforarg_hinj_dn13;
        *var_fn418_calc_ig__expiforarg_hinj_dn4_slot = var_fn418_calc_ig__expiforarg_hinj_dn4;
        *var_fn418_calc_ig__expiforarg_hinj_dn8_slot = var_fn418_calc_ig__expiforarg_hinj_dn8;
        *var_fn418_calc_ig__expirev_slot = var_fn418_calc_ig__expirev;
        *var_fn418_calc_ig__expirev_dn13_slot = var_fn418_calc_ig__expirev_dn13;
        *var_fn418_calc_ig__expirev_dn4_slot = var_fn418_calc_ig__expirev_dn4;
        *var_fn418_calc_ig__expirev_dn8_slot = var_fn418_calc_ig__expirev_dn8;
        *var_fn418_calc_ig__expirevarg_slot = var_fn418_calc_ig__expirevarg;
        *var_fn418_calc_ig__expirevarg_dn13_slot = var_fn418_calc_ig__expirevarg_dn13;
        *var_fn418_calc_ig__expirevarg_dn4_slot = var_fn418_calc_ig__expirevarg_dn4;
        *var_fn418_calc_ig__expirevarg_dn8_slot = var_fn418_calc_ig__expirevarg_dn8;
        *var_fn418_calc_ig__ffvgin_slot = var_fn418_calc_ig__ffvgin;
        *var_fn418_calc_ig__ffvgin_dn13_slot = var_fn418_calc_ig__ffvgin_dn13;
        *var_fn418_calc_ig__ffvgin_dn4_slot = var_fn418_calc_ig__ffvgin_dn4;
        *var_fn418_calc_ig__ffvgin_dn8_slot = var_fn418_calc_ig__ffvgin_dn8;
        *var_fn418_calc_ig__frecgin_slot = var_fn418_calc_ig__frecgin;
        *var_fn418_calc_ig__frecgin_dn13_slot = var_fn418_calc_ig__frecgin_dn13;
        *var_fn418_calc_ig__frecgin_dn8_slot = var_fn418_calc_ig__frecgin_dn8;
        *var_fn418_calc_ig__igindiode_slot = var_fn418_calc_ig__igindiode;
        *var_fn418_calc_ig__igindiode_dn13_slot = var_fn418_calc_ig__igindiode_dn13;
        *var_fn418_calc_ig__igindiode_dn4_slot = var_fn418_calc_ig__igindiode_dn4;
        *var_fn418_calc_ig__igindiode_dn8_slot = var_fn418_calc_ig__igindiode_dn8;
        *var_fn418_calc_ig__igindiode_hinj_slot = var_fn418_calc_ig__igindiode_hinj;
        *var_fn418_calc_ig__igindiode_hinj_dn13_slot = var_fn418_calc_ig__igindiode_hinj_dn13;
        *var_fn418_calc_ig__igindiode_hinj_dn4_slot = var_fn418_calc_ig__igindiode_hinj_dn4;
        *var_fn418_calc_ig__igindiode_hinj_dn8_slot = var_fn418_calc_ig__igindiode_hinj_dn8;
        *var_fn418_calc_ig__igindiode_hinj_pre_slot = var_fn418_calc_ig__igindiode_hinj_pre;
        *var_fn418_calc_ig__igindiode_hinj_pre_dn4_slot = var_fn418_calc_ig__igindiode_hinj_pre_dn4;
        *var_fn418_calc_ig__iginrec_slot = var_fn418_calc_ig__iginrec;
        *var_fn418_calc_ig__iginrec_dn13_slot = var_fn418_calc_ig__iginrec_dn13;
        *var_fn418_calc_ig__iginrec_dn4_slot = var_fn418_calc_ig__iginrec_dn4;
        *var_fn418_calc_ig__iginrec_dn8_slot = var_fn418_calc_ig__iginrec_dn8;
        *var_fn418_calc_ig__igout_slot = var_fn418_calc_ig__igout;
        *var_fn418_calc_ig__igout_dn13_slot = var_fn418_calc_ig__igout_dn13;
        *var_fn418_calc_ig__igout_dn4_slot = var_fn418_calc_ig__igout_dn4;
        *var_fn418_calc_ig__igout_dn8_slot = var_fn418_calc_ig__igout_dn8;
        *var_fn418_calc_ig__isrecout_slot = var_fn418_calc_ig__isrecout;
        *var_fn418_calc_ig__isrecout_dn4_slot = var_fn418_calc_ig__isrecout_dn4;
        *var_fn418_calc_ig__return_slot = var_fn418_calc_ig__return;
        *var_fn418_calc_ig__return_dn13_slot = var_fn418_calc_ig__return_dn13;
        *var_fn418_calc_ig__return_dn4_slot = var_fn418_calc_ig__return_dn4;
        *var_fn418_calc_ig__return_dn8_slot = var_fn418_calc_ig__return_dn8;
        *var_fn423_calc_ig__alphagin_slot = var_fn423_calc_ig__alphagin;
        *var_fn423_calc_ig__fracin_slot = var_fn423_calc_ig__fracin;
        *var_fn423_calc_ig__ijin_slot = var_fn423_calc_ig__ijin;
        *var_fn423_calc_ig__isdiodeout_slot = var_fn423_calc_ig__isdiodeout;
        *var_fn423_calc_ig__isdiodeout_dn4_slot = var_fn423_calc_ig__isdiodeout_dn4;
        *var_fn423_calc_ig__isrecout_slot = var_fn423_calc_ig__isrecout;
        *var_fn423_calc_ig__isrecout_dn4_slot = var_fn423_calc_ig__isrecout_dn4;
        *var_fn423_calc_ig__kbdgatein_slot = var_fn423_calc_ig__kbdgatein;
        *var_fn423_calc_ig__ngf_slot = var_fn423_calc_ig__ngf;
        *var_fn423_calc_ig__pbdgin_slot = var_fn423_calc_ig__pbdgin;
        *var_fn423_calc_ig__pg_paramin_slot = var_fn423_calc_ig__pg_paramin;
        *var_fn423_calc_ig__phitin_slot = var_fn423_calc_ig__phitin;
        *var_fn423_calc_ig__phitin_dn4_slot = var_fn423_calc_ig__phitin_dn4;
        *var_fn423_calc_ig__return_slot = var_fn423_calc_ig__return;
        *var_fn423_calc_ig__return_dn17_slot = var_fn423_calc_ig__return_dn17;
        *var_fn423_calc_ig__return_dn4_slot = var_fn423_calc_ig__return_dn4;
        *var_fn423_calc_ig__return_dn8_slot = var_fn423_calc_ig__return_dn8;
        *var_fn423_calc_ig__tfacdiodein_slot = var_fn423_calc_ig__tfacdiodein;
        *var_fn423_calc_ig__tfacdiodein_dn4_slot = var_fn423_calc_ig__tfacdiodein_dn4;
        *var_fn423_calc_ig__vbdgin_slot = var_fn423_calc_ig__vbdgin;
        *var_fn423_calc_ig__vgin_slot = var_fn423_calc_ig__vgin;
        *var_fn423_calc_ig__vgin_dn17_slot = var_fn423_calc_ig__vgin_dn17;
        *var_fn423_calc_ig__vgin_dn8_slot = var_fn423_calc_ig__vgin_dn8;
        *var_fn423_calc_ig__vgsatin_slot = var_fn423_calc_ig__vgsatin;
        *var_fn423_calc_ig__w_slot = var_fn423_calc_ig__w;
        *var_guard421_slot = var_guard421;
        *var_guard422_slot = var_guard422;
        *var_igsi_slot = var_igsi;
        *var_igsi_dn13_slot = var_igsi_dn13;
        *var_igsi_dn4_slot = var_igsi_dn4;
        *var_igsi_dn8_slot = var_igsi_dn8;
    }

    pub(super) fn stamp_transient_block_84(
        p: &Parameters,
        var_fn423_calc_ig__pbdgin: f64,
        var_fn423_calc_ig__phitin: f64,
        var_fn423_calc_ig__phitin_dn4: f64,
        var_fn423_calc_ig__vbdgin: f64,
        var_fn423_calc_ig__vgin: f64,
        var_fn423_calc_ig__vgin_dn17: f64,
        var_fn423_calc_ig__vgin_dn8: f64,
        var_guard417: f64,
        var_fn423_calc_ig__alpha2_phit_slot: &mut f64,
        var_fn423_calc_ig__alpha2_phit_dn4_slot: &mut f64,
        var_fn423_calc_ig__betarecin_slot: &mut f64,
        var_fn423_calc_ig__expbd1_slot: &mut f64,
        var_fn423_calc_ig__expbd1_dn17_slot: &mut f64,
        var_fn423_calc_ig__expbd1_dn4_slot: &mut f64,
        var_fn423_calc_ig__expbd1_dn8_slot: &mut f64,
        var_fn423_calc_ig__expbd1_vgsat_slot: &mut f64,
        var_fn423_calc_ig__expbd1_vgsat_dn4_slot: &mut f64,
        var_fn423_calc_ig__expbd2_slot: &mut f64,
        var_fn423_calc_ig__expbd2_dn4_slot: &mut f64,
        var_fn423_calc_ig__expbdarg1_slot: &mut f64,
        var_fn423_calc_ig__expbdarg1_dn17_slot: &mut f64,
        var_fn423_calc_ig__expbdarg1_dn4_slot: &mut f64,
        var_fn423_calc_ig__expbdarg1_dn8_slot: &mut f64,
        var_fn423_calc_ig__expbdarg1_vgsat_slot: &mut f64,
        var_fn423_calc_ig__expbdarg1_vgsat_dn4_slot: &mut f64,
        var_fn423_calc_ig__expbdarg2_slot: &mut f64,
        var_fn423_calc_ig__expbdarg2_dn4_slot: &mut f64,
        var_fn423_calc_ig__expffvarg_slot: &mut f64,
        var_fn423_calc_ig__expffvarg_dn17_slot: &mut f64,
        var_fn423_calc_ig__expffvarg_dn4_slot: &mut f64,
        var_fn423_calc_ig__expffvarg_dn8_slot: &mut f64,
        var_fn423_calc_ig__expifor_slot: &mut f64,
        var_fn423_calc_ig__expifor_dn17_slot: &mut f64,
        var_fn423_calc_ig__expifor_dn4_slot: &mut f64,
        var_fn423_calc_ig__expifor_dn8_slot: &mut f64,
        var_fn423_calc_ig__expifor_hinj_slot: &mut f64,
        var_fn423_calc_ig__expifor_hinj_dn17_slot: &mut f64,
        var_fn423_calc_ig__expifor_hinj_dn4_slot: &mut f64,
        var_fn423_calc_ig__expifor_hinj_dn8_slot: &mut f64,
        var_fn423_calc_ig__expifor_hinj_vgsat_slot: &mut f64,
        var_fn423_calc_ig__expifor_hinj_vgsat_dn4_slot: &mut f64,
        var_fn423_calc_ig__expifor_nohinj_vgsat_slot: &mut f64,
        var_fn423_calc_ig__expifor_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn423_calc_ig__expiforarg_slot: &mut f64,
        var_fn423_calc_ig__expiforarg_dn17_slot: &mut f64,
        var_fn423_calc_ig__expiforarg_dn4_slot: &mut f64,
        var_fn423_calc_ig__expiforarg_dn8_slot: &mut f64,
        var_fn423_calc_ig__expiforarg_hinj_slot: &mut f64,
        var_fn423_calc_ig__expiforarg_hinj_dn17_slot: &mut f64,
        var_fn423_calc_ig__expiforarg_hinj_dn4_slot: &mut f64,
        var_fn423_calc_ig__expiforarg_hinj_dn8_slot: &mut f64,
        var_fn423_calc_ig__expiforarg_hinj_vgsat_slot: &mut f64,
        var_fn423_calc_ig__expiforarg_hinj_vgsat_dn4_slot: &mut f64,
        var_fn423_calc_ig__expiforarg_nohinj_vgsat_slot: &mut f64,
        var_fn423_calc_ig__expiforarg_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn423_calc_ig__expirev_slot: &mut f64,
        var_fn423_calc_ig__expirev_dn17_slot: &mut f64,
        var_fn423_calc_ig__expirev_dn4_slot: &mut f64,
        var_fn423_calc_ig__expirev_dn8_slot: &mut f64,
        var_fn423_calc_ig__expirevarg_slot: &mut f64,
        var_fn423_calc_ig__expirevarg_dn17_slot: &mut f64,
        var_fn423_calc_ig__expirevarg_dn4_slot: &mut f64,
        var_fn423_calc_ig__expirevarg_dn8_slot: &mut f64,
        var_fn423_calc_ig__expphib_slot: &mut f64,
        var_fn423_calc_ig__expphib_dn4_slot: &mut f64,
        var_fn423_calc_ig__ffvgin_slot: &mut f64,
        var_fn423_calc_ig__ffvgin_dn17_slot: &mut f64,
        var_fn423_calc_ig__ffvgin_dn4_slot: &mut f64,
        var_fn423_calc_ig__ffvgin_dn8_slot: &mut f64,
        var_fn423_calc_ig__frecgin_slot: &mut f64,
        var_fn423_calc_ig__frecgin_dn17_slot: &mut f64,
        var_fn423_calc_ig__frecgin_dn8_slot: &mut f64,
        var_fn423_calc_ig__iginbd_slot: &mut f64,
        var_fn423_calc_ig__iginbd_dn17_slot: &mut f64,
        var_fn423_calc_ig__iginbd_dn4_slot: &mut f64,
        var_fn423_calc_ig__iginbd_dn8_slot: &mut f64,
        var_fn423_calc_ig__iginbd_vgsat_slot: &mut f64,
        var_fn423_calc_ig__iginbd_vgsat_dn4_slot: &mut f64,
        var_fn423_calc_ig__igindiode_slot: &mut f64,
        var_fn423_calc_ig__igindiode_dn17_slot: &mut f64,
        var_fn423_calc_ig__igindiode_dn4_slot: &mut f64,
        var_fn423_calc_ig__igindiode_dn8_slot: &mut f64,
        var_fn423_calc_ig__igindiode_hinj_slot: &mut f64,
        var_fn423_calc_ig__igindiode_hinj_dn17_slot: &mut f64,
        var_fn423_calc_ig__igindiode_hinj_dn4_slot: &mut f64,
        var_fn423_calc_ig__igindiode_hinj_dn8_slot: &mut f64,
        var_fn423_calc_ig__igindiode_hinj_pre_slot: &mut f64,
        var_fn423_calc_ig__igindiode_hinj_pre_dn4_slot: &mut f64,
        var_fn423_calc_ig__igindiode_hinj_vgsat_slot: &mut f64,
        var_fn423_calc_ig__igindiode_hinj_vgsat_dn4_slot: &mut f64,
        var_fn423_calc_ig__igindiode_nohinj_slot: &mut f64,
        var_fn423_calc_ig__igindiode_nohinj_dn17_slot: &mut f64,
        var_fn423_calc_ig__igindiode_nohinj_dn4_slot: &mut f64,
        var_fn423_calc_ig__igindiode_nohinj_dn8_slot: &mut f64,
        var_fn423_calc_ig__igindiode_nohinj_vgsat_slot: &mut f64,
        var_fn423_calc_ig__igindiode_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn423_calc_ig__iginrec_slot: &mut f64,
        var_fn423_calc_ig__iginrec_dn17_slot: &mut f64,
        var_fn423_calc_ig__iginrec_dn4_slot: &mut f64,
        var_fn423_calc_ig__iginrec_dn8_slot: &mut f64,
        var_fn423_calc_ig__igout_slot: &mut f64,
        var_fn423_calc_ig__igout_dn17_slot: &mut f64,
        var_fn423_calc_ig__igout_dn4_slot: &mut f64,
        var_fn423_calc_ig__igout_dn8_slot: &mut f64,
        var_fn423_calc_ig__irecin_slot: &mut f64,
        var_fn423_calc_ig__pg_param1_slot: &mut f64,
        var_fn423_calc_ig__pg_paramin_hinj_slot: &mut f64,
        var_fn423_calc_ig__pgsrecin_slot: &mut f64,
        var_fn423_calc_ig__t0_slot: &mut f64,
        var_fn423_calc_ig__t0_dn4_slot: &mut f64,
        var_fn423_calc_ig__type_slot: &mut f64,
        var_fn423_calc_ig__vgsatqin_slot: &mut f64,
        var_fn423_calc_ig__vjg_slot: &mut f64,
    ) {
        let mut var_fn423_calc_ig__alpha2_phit: f64 = *var_fn423_calc_ig__alpha2_phit_slot;
        let mut var_fn423_calc_ig__alpha2_phit_dn4: f64 = *var_fn423_calc_ig__alpha2_phit_dn4_slot;
        let mut var_fn423_calc_ig__betarecin: f64 = *var_fn423_calc_ig__betarecin_slot;
        let mut var_fn423_calc_ig__expbd1: f64 = *var_fn423_calc_ig__expbd1_slot;
        let mut var_fn423_calc_ig__expbd1_dn17: f64 = *var_fn423_calc_ig__expbd1_dn17_slot;
        let mut var_fn423_calc_ig__expbd1_dn4: f64 = *var_fn423_calc_ig__expbd1_dn4_slot;
        let mut var_fn423_calc_ig__expbd1_dn8: f64 = *var_fn423_calc_ig__expbd1_dn8_slot;
        let mut var_fn423_calc_ig__expbd1_vgsat: f64 = *var_fn423_calc_ig__expbd1_vgsat_slot;
        let mut var_fn423_calc_ig__expbd1_vgsat_dn4: f64 = *var_fn423_calc_ig__expbd1_vgsat_dn4_slot;
        let mut var_fn423_calc_ig__expbd2: f64 = *var_fn423_calc_ig__expbd2_slot;
        let mut var_fn423_calc_ig__expbd2_dn4: f64 = *var_fn423_calc_ig__expbd2_dn4_slot;
        let mut var_fn423_calc_ig__expbdarg1: f64 = *var_fn423_calc_ig__expbdarg1_slot;
        let mut var_fn423_calc_ig__expbdarg1_dn17: f64 = *var_fn423_calc_ig__expbdarg1_dn17_slot;
        let mut var_fn423_calc_ig__expbdarg1_dn4: f64 = *var_fn423_calc_ig__expbdarg1_dn4_slot;
        let mut var_fn423_calc_ig__expbdarg1_dn8: f64 = *var_fn423_calc_ig__expbdarg1_dn8_slot;
        let mut var_fn423_calc_ig__expbdarg1_vgsat: f64 = *var_fn423_calc_ig__expbdarg1_vgsat_slot;
        let mut var_fn423_calc_ig__expbdarg1_vgsat_dn4: f64 = *var_fn423_calc_ig__expbdarg1_vgsat_dn4_slot;
        let mut var_fn423_calc_ig__expbdarg2: f64 = *var_fn423_calc_ig__expbdarg2_slot;
        let mut var_fn423_calc_ig__expbdarg2_dn4: f64 = *var_fn423_calc_ig__expbdarg2_dn4_slot;
        let mut var_fn423_calc_ig__expffvarg: f64 = *var_fn423_calc_ig__expffvarg_slot;
        let mut var_fn423_calc_ig__expffvarg_dn17: f64 = *var_fn423_calc_ig__expffvarg_dn17_slot;
        let mut var_fn423_calc_ig__expffvarg_dn4: f64 = *var_fn423_calc_ig__expffvarg_dn4_slot;
        let mut var_fn423_calc_ig__expffvarg_dn8: f64 = *var_fn423_calc_ig__expffvarg_dn8_slot;
        let mut var_fn423_calc_ig__expifor: f64 = *var_fn423_calc_ig__expifor_slot;
        let mut var_fn423_calc_ig__expifor_dn17: f64 = *var_fn423_calc_ig__expifor_dn17_slot;
        let mut var_fn423_calc_ig__expifor_dn4: f64 = *var_fn423_calc_ig__expifor_dn4_slot;
        let mut var_fn423_calc_ig__expifor_dn8: f64 = *var_fn423_calc_ig__expifor_dn8_slot;
        let mut var_fn423_calc_ig__expifor_hinj: f64 = *var_fn423_calc_ig__expifor_hinj_slot;
        let mut var_fn423_calc_ig__expifor_hinj_dn17: f64 = *var_fn423_calc_ig__expifor_hinj_dn17_slot;
        let mut var_fn423_calc_ig__expifor_hinj_dn4: f64 = *var_fn423_calc_ig__expifor_hinj_dn4_slot;
        let mut var_fn423_calc_ig__expifor_hinj_dn8: f64 = *var_fn423_calc_ig__expifor_hinj_dn8_slot;
        let mut var_fn423_calc_ig__expifor_hinj_vgsat: f64 = *var_fn423_calc_ig__expifor_hinj_vgsat_slot;
        let mut var_fn423_calc_ig__expifor_hinj_vgsat_dn4: f64 = *var_fn423_calc_ig__expifor_hinj_vgsat_dn4_slot;
        let mut var_fn423_calc_ig__expifor_nohinj_vgsat: f64 = *var_fn423_calc_ig__expifor_nohinj_vgsat_slot;
        let mut var_fn423_calc_ig__expifor_nohinj_vgsat_dn4: f64 = *var_fn423_calc_ig__expifor_nohinj_vgsat_dn4_slot;
        let mut var_fn423_calc_ig__expiforarg: f64 = *var_fn423_calc_ig__expiforarg_slot;
        let mut var_fn423_calc_ig__expiforarg_dn17: f64 = *var_fn423_calc_ig__expiforarg_dn17_slot;
        let mut var_fn423_calc_ig__expiforarg_dn4: f64 = *var_fn423_calc_ig__expiforarg_dn4_slot;
        let mut var_fn423_calc_ig__expiforarg_dn8: f64 = *var_fn423_calc_ig__expiforarg_dn8_slot;
        let mut var_fn423_calc_ig__expiforarg_hinj: f64 = *var_fn423_calc_ig__expiforarg_hinj_slot;
        let mut var_fn423_calc_ig__expiforarg_hinj_dn17: f64 = *var_fn423_calc_ig__expiforarg_hinj_dn17_slot;
        let mut var_fn423_calc_ig__expiforarg_hinj_dn4: f64 = *var_fn423_calc_ig__expiforarg_hinj_dn4_slot;
        let mut var_fn423_calc_ig__expiforarg_hinj_dn8: f64 = *var_fn423_calc_ig__expiforarg_hinj_dn8_slot;
        let mut var_fn423_calc_ig__expiforarg_hinj_vgsat: f64 = *var_fn423_calc_ig__expiforarg_hinj_vgsat_slot;
        let mut var_fn423_calc_ig__expiforarg_hinj_vgsat_dn4: f64 = *var_fn423_calc_ig__expiforarg_hinj_vgsat_dn4_slot;
        let mut var_fn423_calc_ig__expiforarg_nohinj_vgsat: f64 = *var_fn423_calc_ig__expiforarg_nohinj_vgsat_slot;
        let mut var_fn423_calc_ig__expiforarg_nohinj_vgsat_dn4: f64 = *var_fn423_calc_ig__expiforarg_nohinj_vgsat_dn4_slot;
        let mut var_fn423_calc_ig__expirev: f64 = *var_fn423_calc_ig__expirev_slot;
        let mut var_fn423_calc_ig__expirev_dn17: f64 = *var_fn423_calc_ig__expirev_dn17_slot;
        let mut var_fn423_calc_ig__expirev_dn4: f64 = *var_fn423_calc_ig__expirev_dn4_slot;
        let mut var_fn423_calc_ig__expirev_dn8: f64 = *var_fn423_calc_ig__expirev_dn8_slot;
        let mut var_fn423_calc_ig__expirevarg: f64 = *var_fn423_calc_ig__expirevarg_slot;
        let mut var_fn423_calc_ig__expirevarg_dn17: f64 = *var_fn423_calc_ig__expirevarg_dn17_slot;
        let mut var_fn423_calc_ig__expirevarg_dn4: f64 = *var_fn423_calc_ig__expirevarg_dn4_slot;
        let mut var_fn423_calc_ig__expirevarg_dn8: f64 = *var_fn423_calc_ig__expirevarg_dn8_slot;
        let mut var_fn423_calc_ig__expphib: f64 = *var_fn423_calc_ig__expphib_slot;
        let mut var_fn423_calc_ig__expphib_dn4: f64 = *var_fn423_calc_ig__expphib_dn4_slot;
        let mut var_fn423_calc_ig__ffvgin: f64 = *var_fn423_calc_ig__ffvgin_slot;
        let mut var_fn423_calc_ig__ffvgin_dn17: f64 = *var_fn423_calc_ig__ffvgin_dn17_slot;
        let mut var_fn423_calc_ig__ffvgin_dn4: f64 = *var_fn423_calc_ig__ffvgin_dn4_slot;
        let mut var_fn423_calc_ig__ffvgin_dn8: f64 = *var_fn423_calc_ig__ffvgin_dn8_slot;
        let mut var_fn423_calc_ig__frecgin: f64 = *var_fn423_calc_ig__frecgin_slot;
        let mut var_fn423_calc_ig__frecgin_dn17: f64 = *var_fn423_calc_ig__frecgin_dn17_slot;
        let mut var_fn423_calc_ig__frecgin_dn8: f64 = *var_fn423_calc_ig__frecgin_dn8_slot;
        let mut var_fn423_calc_ig__iginbd: f64 = *var_fn423_calc_ig__iginbd_slot;
        let mut var_fn423_calc_ig__iginbd_dn17: f64 = *var_fn423_calc_ig__iginbd_dn17_slot;
        let mut var_fn423_calc_ig__iginbd_dn4: f64 = *var_fn423_calc_ig__iginbd_dn4_slot;
        let mut var_fn423_calc_ig__iginbd_dn8: f64 = *var_fn423_calc_ig__iginbd_dn8_slot;
        let mut var_fn423_calc_ig__iginbd_vgsat: f64 = *var_fn423_calc_ig__iginbd_vgsat_slot;
        let mut var_fn423_calc_ig__iginbd_vgsat_dn4: f64 = *var_fn423_calc_ig__iginbd_vgsat_dn4_slot;
        let mut var_fn423_calc_ig__igindiode: f64 = *var_fn423_calc_ig__igindiode_slot;
        let mut var_fn423_calc_ig__igindiode_dn17: f64 = *var_fn423_calc_ig__igindiode_dn17_slot;
        let mut var_fn423_calc_ig__igindiode_dn4: f64 = *var_fn423_calc_ig__igindiode_dn4_slot;
        let mut var_fn423_calc_ig__igindiode_dn8: f64 = *var_fn423_calc_ig__igindiode_dn8_slot;
        let mut var_fn423_calc_ig__igindiode_hinj: f64 = *var_fn423_calc_ig__igindiode_hinj_slot;
        let mut var_fn423_calc_ig__igindiode_hinj_dn17: f64 = *var_fn423_calc_ig__igindiode_hinj_dn17_slot;
        let mut var_fn423_calc_ig__igindiode_hinj_dn4: f64 = *var_fn423_calc_ig__igindiode_hinj_dn4_slot;
        let mut var_fn423_calc_ig__igindiode_hinj_dn8: f64 = *var_fn423_calc_ig__igindiode_hinj_dn8_slot;
        let mut var_fn423_calc_ig__igindiode_hinj_pre: f64 = *var_fn423_calc_ig__igindiode_hinj_pre_slot;
        let mut var_fn423_calc_ig__igindiode_hinj_pre_dn4: f64 = *var_fn423_calc_ig__igindiode_hinj_pre_dn4_slot;
        let mut var_fn423_calc_ig__igindiode_hinj_vgsat: f64 = *var_fn423_calc_ig__igindiode_hinj_vgsat_slot;
        let mut var_fn423_calc_ig__igindiode_hinj_vgsat_dn4: f64 = *var_fn423_calc_ig__igindiode_hinj_vgsat_dn4_slot;
        let mut var_fn423_calc_ig__igindiode_nohinj: f64 = *var_fn423_calc_ig__igindiode_nohinj_slot;
        let mut var_fn423_calc_ig__igindiode_nohinj_dn17: f64 = *var_fn423_calc_ig__igindiode_nohinj_dn17_slot;
        let mut var_fn423_calc_ig__igindiode_nohinj_dn4: f64 = *var_fn423_calc_ig__igindiode_nohinj_dn4_slot;
        let mut var_fn423_calc_ig__igindiode_nohinj_dn8: f64 = *var_fn423_calc_ig__igindiode_nohinj_dn8_slot;
        let mut var_fn423_calc_ig__igindiode_nohinj_vgsat: f64 = *var_fn423_calc_ig__igindiode_nohinj_vgsat_slot;
        let mut var_fn423_calc_ig__igindiode_nohinj_vgsat_dn4: f64 = *var_fn423_calc_ig__igindiode_nohinj_vgsat_dn4_slot;
        let mut var_fn423_calc_ig__iginrec: f64 = *var_fn423_calc_ig__iginrec_slot;
        let mut var_fn423_calc_ig__iginrec_dn17: f64 = *var_fn423_calc_ig__iginrec_dn17_slot;
        let mut var_fn423_calc_ig__iginrec_dn4: f64 = *var_fn423_calc_ig__iginrec_dn4_slot;
        let mut var_fn423_calc_ig__iginrec_dn8: f64 = *var_fn423_calc_ig__iginrec_dn8_slot;
        let mut var_fn423_calc_ig__igout: f64 = *var_fn423_calc_ig__igout_slot;
        let mut var_fn423_calc_ig__igout_dn17: f64 = *var_fn423_calc_ig__igout_dn17_slot;
        let mut var_fn423_calc_ig__igout_dn4: f64 = *var_fn423_calc_ig__igout_dn4_slot;
        let mut var_fn423_calc_ig__igout_dn8: f64 = *var_fn423_calc_ig__igout_dn8_slot;
        let mut var_fn423_calc_ig__irecin: f64 = *var_fn423_calc_ig__irecin_slot;
        let mut var_fn423_calc_ig__pg_param1: f64 = *var_fn423_calc_ig__pg_param1_slot;
        let mut var_fn423_calc_ig__pg_paramin_hinj: f64 = *var_fn423_calc_ig__pg_paramin_hinj_slot;
        let mut var_fn423_calc_ig__pgsrecin: f64 = *var_fn423_calc_ig__pgsrecin_slot;
        let mut var_fn423_calc_ig__t0: f64 = *var_fn423_calc_ig__t0_slot;
        let mut var_fn423_calc_ig__t0_dn4: f64 = *var_fn423_calc_ig__t0_dn4_slot;
        let mut var_fn423_calc_ig__type: f64 = *var_fn423_calc_ig__type_slot;
        let mut var_fn423_calc_ig__vgsatqin: f64 = *var_fn423_calc_ig__vgsatqin_slot;
        let mut var_fn423_calc_ig__vjg: f64 = *var_fn423_calc_ig__vjg_slot;

        let (assign34250_e31017,) = {
    if (var_guard417 != 0.0) {
        (p.p274,)
    } else {
        (var_fn423_calc_ig__vgsatqin,)
    }
};
        var_fn423_calc_ig__vgsatqin = assign34250_e31017;

        let (assign34260_e31021,) = {
    if (var_guard417 != 0.0) {
        (p.p275,)
    } else {
        (var_fn423_calc_ig__betarecin,)
    }
};
        var_fn423_calc_ig__betarecin = assign34260_e31021;

        let (assign34270_e31029,) = {
    if (var_guard417 != 0.0) {
        let assign34270_e31025: f64 = (1.0 - p.p255);
        let assign34270_e31027: f64 = (assign34270_e31025 * p.p273);
        (assign34270_e31027,)
    } else {
        (var_fn423_calc_ig__irecin,)
    }
};
        var_fn423_calc_ig__irecin = assign34270_e31029;

        let (assign34280_e31033,) = {
    if (var_guard417 != 0.0) {
        (p.p272,)
    } else {
        (var_fn423_calc_ig__pgsrecin,)
    }
};
        var_fn423_calc_ig__pgsrecin = assign34280_e31033;

        let (assign34290_e31037,) = {
    if (var_guard417 != 0.0) {
        (p.p257,)
    } else {
        (var_fn423_calc_ig__pg_param1,)
    }
};
        var_fn423_calc_ig__pg_param1 = assign34290_e31037;

        let (assign34300_e31041,) = {
    if (var_guard417 != 0.0) {
        (p.p256,)
    } else {
        (var_fn423_calc_ig__vjg,)
    }
};
        var_fn423_calc_ig__vjg = assign34300_e31041;

        let (assign34310_e31045,) = {
    if (var_guard417 != 0.0) {
        (p.p6,)
    } else {
        (var_fn423_calc_ig__type,)
    }
};
        var_fn423_calc_ig__type = assign34310_e31045;

        let (assign34320_e31049, assign34320_e31049_d_n4, assign34320_e31049_d_n8, assign34320_e31049_d_n17,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn423_calc_ig__igout, var_fn423_calc_ig__igout_dn4, var_fn423_calc_ig__igout_dn8, var_fn423_calc_ig__igout_dn17,)
    }
};
        var_fn423_calc_ig__igout = assign34320_e31049;
        var_fn423_calc_ig__igout_dn4 = assign34320_e31049_d_n4;
        var_fn423_calc_ig__igout_dn8 = assign34320_e31049_d_n8;
        var_fn423_calc_ig__igout_dn17 = assign34320_e31049_d_n17;

        let (assign34330_e31053, assign34330_e31053_d_n4,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn423_calc_ig__alpha2_phit, var_fn423_calc_ig__alpha2_phit_dn4,)
    }
};
        var_fn423_calc_ig__alpha2_phit = assign34330_e31053;
        var_fn423_calc_ig__alpha2_phit_dn4 = assign34330_e31053_d_n4;

        let (assign34340_e31057, assign34340_e31057_d_n4,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn423_calc_ig__t0, var_fn423_calc_ig__t0_dn4,)
    }
};
        var_fn423_calc_ig__t0 = assign34340_e31057;
        var_fn423_calc_ig__t0_dn4 = assign34340_e31057_d_n4;

        let (assign34350_e31061, assign34350_e31061_d_n4, assign34350_e31061_d_n8, assign34350_e31061_d_n17,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn423_calc_ig__ffvgin, var_fn423_calc_ig__ffvgin_dn4, var_fn423_calc_ig__ffvgin_dn8, var_fn423_calc_ig__ffvgin_dn17,)
    }
};
        var_fn423_calc_ig__ffvgin = assign34350_e31061;
        var_fn423_calc_ig__ffvgin_dn4 = assign34350_e31061_d_n4;
        var_fn423_calc_ig__ffvgin_dn8 = assign34350_e31061_d_n8;
        var_fn423_calc_ig__ffvgin_dn17 = assign34350_e31061_d_n17;

        let (assign34360_e31065, assign34360_e31065_d_n4, assign34360_e31065_d_n8, assign34360_e31065_d_n17,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn423_calc_ig__iginbd, var_fn423_calc_ig__iginbd_dn4, var_fn423_calc_ig__iginbd_dn8, var_fn423_calc_ig__iginbd_dn17,)
    }
};
        var_fn423_calc_ig__iginbd = assign34360_e31065;
        var_fn423_calc_ig__iginbd_dn4 = assign34360_e31065_d_n4;
        var_fn423_calc_ig__iginbd_dn8 = assign34360_e31065_d_n8;
        var_fn423_calc_ig__iginbd_dn17 = assign34360_e31065_d_n17;

        let (assign34370_e31069, assign34370_e31069_d_n4, assign34370_e31069_d_n8, assign34370_e31069_d_n17,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn423_calc_ig__igindiode, var_fn423_calc_ig__igindiode_dn4, var_fn423_calc_ig__igindiode_dn8, var_fn423_calc_ig__igindiode_dn17,)
    }
};
        var_fn423_calc_ig__igindiode = assign34370_e31069;
        var_fn423_calc_ig__igindiode_dn4 = assign34370_e31069_d_n4;
        var_fn423_calc_ig__igindiode_dn8 = assign34370_e31069_d_n8;
        var_fn423_calc_ig__igindiode_dn17 = assign34370_e31069_d_n17;

        let (assign34380_e31073, assign34380_e31073_d_n8, assign34380_e31073_d_n17,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_fn423_calc_ig__frecgin, var_fn423_calc_ig__frecgin_dn8, var_fn423_calc_ig__frecgin_dn17,)
    }
};
        var_fn423_calc_ig__frecgin = assign34380_e31073;
        var_fn423_calc_ig__frecgin_dn8 = assign34380_e31073_d_n8;
        var_fn423_calc_ig__frecgin_dn17 = assign34380_e31073_d_n17;

        let (assign34390_e31077, assign34390_e31077_d_n4, assign34390_e31077_d_n8, assign34390_e31077_d_n17,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn423_calc_ig__iginrec, var_fn423_calc_ig__iginrec_dn4, var_fn423_calc_ig__iginrec_dn8, var_fn423_calc_ig__iginrec_dn17,)
    }
};
        var_fn423_calc_ig__iginrec = assign34390_e31077;
        var_fn423_calc_ig__iginrec_dn4 = assign34390_e31077_d_n4;
        var_fn423_calc_ig__iginrec_dn8 = assign34390_e31077_d_n8;
        var_fn423_calc_ig__iginrec_dn17 = assign34390_e31077_d_n17;

        let (assign34400_e31081, assign34400_e31081_d_n4, assign34400_e31081_d_n8, assign34400_e31081_d_n17,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn423_calc_ig__expbdarg1, var_fn423_calc_ig__expbdarg1_dn4, var_fn423_calc_ig__expbdarg1_dn8, var_fn423_calc_ig__expbdarg1_dn17,)
    }
};
        var_fn423_calc_ig__expbdarg1 = assign34400_e31081;
        var_fn423_calc_ig__expbdarg1_dn4 = assign34400_e31081_d_n4;
        var_fn423_calc_ig__expbdarg1_dn8 = assign34400_e31081_d_n8;
        var_fn423_calc_ig__expbdarg1_dn17 = assign34400_e31081_d_n17;

        let (assign34410_e31085, assign34410_e31085_d_n4,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn423_calc_ig__expbdarg2, var_fn423_calc_ig__expbdarg2_dn4,)
    }
};
        var_fn423_calc_ig__expbdarg2 = assign34410_e31085;
        var_fn423_calc_ig__expbdarg2_dn4 = assign34410_e31085_d_n4;

        let (assign34420_e31089, assign34420_e31089_d_n4, assign34420_e31089_d_n8, assign34420_e31089_d_n17,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn423_calc_ig__expbd1, var_fn423_calc_ig__expbd1_dn4, var_fn423_calc_ig__expbd1_dn8, var_fn423_calc_ig__expbd1_dn17,)
    }
};
        var_fn423_calc_ig__expbd1 = assign34420_e31089;
        var_fn423_calc_ig__expbd1_dn4 = assign34420_e31089_d_n4;
        var_fn423_calc_ig__expbd1_dn8 = assign34420_e31089_d_n8;
        var_fn423_calc_ig__expbd1_dn17 = assign34420_e31089_d_n17;

        let (assign34430_e31093, assign34430_e31093_d_n4,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn423_calc_ig__expbd2, var_fn423_calc_ig__expbd2_dn4,)
    }
};
        var_fn423_calc_ig__expbd2 = assign34430_e31093;
        var_fn423_calc_ig__expbd2_dn4 = assign34430_e31093_d_n4;

        let (assign34440_e31097, assign34440_e31097_d_n4,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn423_calc_ig__expphib, var_fn423_calc_ig__expphib_dn4,)
    }
};
        var_fn423_calc_ig__expphib = assign34440_e31097;
        var_fn423_calc_ig__expphib_dn4 = assign34440_e31097_d_n4;

        let (assign34450_e31101, assign34450_e31101_d_n4, assign34450_e31101_d_n8, assign34450_e31101_d_n17,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn423_calc_ig__expffvarg, var_fn423_calc_ig__expffvarg_dn4, var_fn423_calc_ig__expffvarg_dn8, var_fn423_calc_ig__expffvarg_dn17,)
    }
};
        var_fn423_calc_ig__expffvarg = assign34450_e31101;
        var_fn423_calc_ig__expffvarg_dn4 = assign34450_e31101_d_n4;
        var_fn423_calc_ig__expffvarg_dn8 = assign34450_e31101_d_n8;
        var_fn423_calc_ig__expffvarg_dn17 = assign34450_e31101_d_n17;

        let (assign34460_e31105, assign34460_e31105_d_n4, assign34460_e31105_d_n8, assign34460_e31105_d_n17,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn423_calc_ig__expiforarg, var_fn423_calc_ig__expiforarg_dn4, var_fn423_calc_ig__expiforarg_dn8, var_fn423_calc_ig__expiforarg_dn17,)
    }
};
        var_fn423_calc_ig__expiforarg = assign34460_e31105;
        var_fn423_calc_ig__expiforarg_dn4 = assign34460_e31105_d_n4;
        var_fn423_calc_ig__expiforarg_dn8 = assign34460_e31105_d_n8;
        var_fn423_calc_ig__expiforarg_dn17 = assign34460_e31105_d_n17;

        let (assign34470_e31109, assign34470_e31109_d_n4, assign34470_e31109_d_n8, assign34470_e31109_d_n17,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn423_calc_ig__expifor, var_fn423_calc_ig__expifor_dn4, var_fn423_calc_ig__expifor_dn8, var_fn423_calc_ig__expifor_dn17,)
    }
};
        var_fn423_calc_ig__expifor = assign34470_e31109;
        var_fn423_calc_ig__expifor_dn4 = assign34470_e31109_d_n4;
        var_fn423_calc_ig__expifor_dn8 = assign34470_e31109_d_n8;
        var_fn423_calc_ig__expifor_dn17 = assign34470_e31109_d_n17;

        let (assign34480_e31113, assign34480_e31113_d_n4, assign34480_e31113_d_n8, assign34480_e31113_d_n17,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn423_calc_ig__expirevarg, var_fn423_calc_ig__expirevarg_dn4, var_fn423_calc_ig__expirevarg_dn8, var_fn423_calc_ig__expirevarg_dn17,)
    }
};
        var_fn423_calc_ig__expirevarg = assign34480_e31113;
        var_fn423_calc_ig__expirevarg_dn4 = assign34480_e31113_d_n4;
        var_fn423_calc_ig__expirevarg_dn8 = assign34480_e31113_d_n8;
        var_fn423_calc_ig__expirevarg_dn17 = assign34480_e31113_d_n17;

        let (assign34490_e31117, assign34490_e31117_d_n4, assign34490_e31117_d_n8, assign34490_e31117_d_n17,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn423_calc_ig__expirev, var_fn423_calc_ig__expirev_dn4, var_fn423_calc_ig__expirev_dn8, var_fn423_calc_ig__expirev_dn17,)
    }
};
        var_fn423_calc_ig__expirev = assign34490_e31117;
        var_fn423_calc_ig__expirev_dn4 = assign34490_e31117_d_n4;
        var_fn423_calc_ig__expirev_dn8 = assign34490_e31117_d_n8;
        var_fn423_calc_ig__expirev_dn17 = assign34490_e31117_d_n17;

        let (assign34500_e31121,) = {
    if (var_guard417 != 0.0) {
        (0.0,)
    } else {
        (var_fn423_calc_ig__pg_paramin_hinj,)
    }
};
        var_fn423_calc_ig__pg_paramin_hinj = assign34500_e31121;

        let (assign34510_e31125, assign34510_e31125_d_n4,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn423_calc_ig__expbdarg1_vgsat, var_fn423_calc_ig__expbdarg1_vgsat_dn4,)
    }
};
        var_fn423_calc_ig__expbdarg1_vgsat = assign34510_e31125;
        var_fn423_calc_ig__expbdarg1_vgsat_dn4 = assign34510_e31125_d_n4;

        let (assign34520_e31129, assign34520_e31129_d_n4,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn423_calc_ig__expbd1_vgsat, var_fn423_calc_ig__expbd1_vgsat_dn4,)
    }
};
        var_fn423_calc_ig__expbd1_vgsat = assign34520_e31129;
        var_fn423_calc_ig__expbd1_vgsat_dn4 = assign34520_e31129_d_n4;

        let (assign34530_e31133, assign34530_e31133_d_n4,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn423_calc_ig__iginbd_vgsat, var_fn423_calc_ig__iginbd_vgsat_dn4,)
    }
};
        var_fn423_calc_ig__iginbd_vgsat = assign34530_e31133;
        var_fn423_calc_ig__iginbd_vgsat_dn4 = assign34530_e31133_d_n4;

        let (assign34540_e31137, assign34540_e31137_d_n4,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn423_calc_ig__expiforarg_nohinj_vgsat, var_fn423_calc_ig__expiforarg_nohinj_vgsat_dn4,)
    }
};
        var_fn423_calc_ig__expiforarg_nohinj_vgsat = assign34540_e31137;
        var_fn423_calc_ig__expiforarg_nohinj_vgsat_dn4 = assign34540_e31137_d_n4;

        let (assign34550_e31141, assign34550_e31141_d_n4,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn423_calc_ig__expifor_nohinj_vgsat, var_fn423_calc_ig__expifor_nohinj_vgsat_dn4,)
    }
};
        var_fn423_calc_ig__expifor_nohinj_vgsat = assign34550_e31141;
        var_fn423_calc_ig__expifor_nohinj_vgsat_dn4 = assign34550_e31141_d_n4;

        let (assign34560_e31145, assign34560_e31145_d_n4,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn423_calc_ig__igindiode_nohinj_vgsat, var_fn423_calc_ig__igindiode_nohinj_vgsat_dn4,)
    }
};
        var_fn423_calc_ig__igindiode_nohinj_vgsat = assign34560_e31145;
        var_fn423_calc_ig__igindiode_nohinj_vgsat_dn4 = assign34560_e31145_d_n4;

        let (assign34570_e31149, assign34570_e31149_d_n4, assign34570_e31149_d_n8, assign34570_e31149_d_n17,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn423_calc_ig__igindiode_nohinj, var_fn423_calc_ig__igindiode_nohinj_dn4, var_fn423_calc_ig__igindiode_nohinj_dn8, var_fn423_calc_ig__igindiode_nohinj_dn17,)
    }
};
        var_fn423_calc_ig__igindiode_nohinj = assign34570_e31149;
        var_fn423_calc_ig__igindiode_nohinj_dn4 = assign34570_e31149_d_n4;
        var_fn423_calc_ig__igindiode_nohinj_dn8 = assign34570_e31149_d_n8;
        var_fn423_calc_ig__igindiode_nohinj_dn17 = assign34570_e31149_d_n17;

        let (assign34580_e31153, assign34580_e31153_d_n4,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn423_calc_ig__expiforarg_hinj_vgsat, var_fn423_calc_ig__expiforarg_hinj_vgsat_dn4,)
    }
};
        var_fn423_calc_ig__expiforarg_hinj_vgsat = assign34580_e31153;
        var_fn423_calc_ig__expiforarg_hinj_vgsat_dn4 = assign34580_e31153_d_n4;

        let (assign34590_e31157, assign34590_e31157_d_n4,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn423_calc_ig__expifor_hinj_vgsat, var_fn423_calc_ig__expifor_hinj_vgsat_dn4,)
    }
};
        var_fn423_calc_ig__expifor_hinj_vgsat = assign34590_e31157;
        var_fn423_calc_ig__expifor_hinj_vgsat_dn4 = assign34590_e31157_d_n4;

        let (assign34600_e31161, assign34600_e31161_d_n4,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn423_calc_ig__igindiode_hinj_vgsat, var_fn423_calc_ig__igindiode_hinj_vgsat_dn4,)
    }
};
        var_fn423_calc_ig__igindiode_hinj_vgsat = assign34600_e31161;
        var_fn423_calc_ig__igindiode_hinj_vgsat_dn4 = assign34600_e31161_d_n4;

        let (assign34610_e31165, assign34610_e31165_d_n4, assign34610_e31165_d_n8, assign34610_e31165_d_n17,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn423_calc_ig__expiforarg_hinj, var_fn423_calc_ig__expiforarg_hinj_dn4, var_fn423_calc_ig__expiforarg_hinj_dn8, var_fn423_calc_ig__expiforarg_hinj_dn17,)
    }
};
        var_fn423_calc_ig__expiforarg_hinj = assign34610_e31165;
        var_fn423_calc_ig__expiforarg_hinj_dn4 = assign34610_e31165_d_n4;
        var_fn423_calc_ig__expiforarg_hinj_dn8 = assign34610_e31165_d_n8;
        var_fn423_calc_ig__expiforarg_hinj_dn17 = assign34610_e31165_d_n17;

        let (assign34620_e31169, assign34620_e31169_d_n4, assign34620_e31169_d_n8, assign34620_e31169_d_n17,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn423_calc_ig__expifor_hinj, var_fn423_calc_ig__expifor_hinj_dn4, var_fn423_calc_ig__expifor_hinj_dn8, var_fn423_calc_ig__expifor_hinj_dn17,)
    }
};
        var_fn423_calc_ig__expifor_hinj = assign34620_e31169;
        var_fn423_calc_ig__expifor_hinj_dn4 = assign34620_e31169_d_n4;
        var_fn423_calc_ig__expifor_hinj_dn8 = assign34620_e31169_d_n8;
        var_fn423_calc_ig__expifor_hinj_dn17 = assign34620_e31169_d_n17;

        let (assign34630_e31173, assign34630_e31173_d_n4,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0,)
    } else {
        (var_fn423_calc_ig__igindiode_hinj_pre, var_fn423_calc_ig__igindiode_hinj_pre_dn4,)
    }
};
        var_fn423_calc_ig__igindiode_hinj_pre = assign34630_e31173;
        var_fn423_calc_ig__igindiode_hinj_pre_dn4 = assign34630_e31173_d_n4;

        let (assign34640_e31177, assign34640_e31177_d_n4, assign34640_e31177_d_n8, assign34640_e31177_d_n17,) = {
    if (var_guard417 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn423_calc_ig__igindiode_hinj, var_fn423_calc_ig__igindiode_hinj_dn4, var_fn423_calc_ig__igindiode_hinj_dn8, var_fn423_calc_ig__igindiode_hinj_dn17,)
    }
};
        var_fn423_calc_ig__igindiode_hinj = assign34640_e31177;
        var_fn423_calc_ig__igindiode_hinj_dn4 = assign34640_e31177_d_n4;
        var_fn423_calc_ig__igindiode_hinj_dn8 = assign34640_e31177_d_n8;
        var_fn423_calc_ig__igindiode_hinj_dn17 = assign34640_e31177_d_n17;

        let (assign34650_e31186, assign34650_e31186_d_n4,) = {
    if (var_guard417 != 0.0) {
        let assign34650_e31181: f64 = (var_fn423_calc_ig__pg_param1 / var_fn423_calc_ig__phitin);
        let assign34650_e31183: f64 = (-var_fn423_calc_ig__vjg);
        let assign34650_e31184: f64 = (assign34650_e31181 * assign34650_e31183);
        (assign34650_e31184, ((-((var_fn423_calc_ig__pg_param1 * var_fn423_calc_ig__phitin_dn4) / (var_fn423_calc_ig__phitin * var_fn423_calc_ig__phitin))) * assign34650_e31183),)
    } else {
        (var_fn423_calc_ig__expphib, var_fn423_calc_ig__expphib_dn4,)
    }
};
        var_fn423_calc_ig__expphib = assign34650_e31186;
        var_fn423_calc_ig__expphib_dn4 = assign34650_e31186_d_n4;

        let (assign34660_e31228, assign34660_e31228_d_n4,) = {
    if (var_guard417 != 0.0) {
        let assign34660_e31194: f64 = (-50.0);
        let (assign34660_e31226, assign34660_e31226_d_n4,) = {
            if ((!(var_fn423_calc_ig__expphib > 50.0)) && (!(var_fn423_calc_ig__expphib < assign34660_e31194))) {
                let assign34660_e31199: f64 = (var_fn423_calc_ig__expphib).exp();
                (assign34660_e31199, (assign34660_e31199 * var_fn423_calc_ig__expphib_dn4),)
            } else {
                let assign34660_e31206: f64 = (-50.0);
                let (assign34660_e31225, assign34660_e31225_d_n4,) = {
                    if ((!(var_fn423_calc_ig__expphib > 50.0)) && (var_fn423_calc_ig__expphib < assign34660_e31206)) {
                        let assign34660_e31210: f64 = (-50.0);
                        let assign34660_e31211: f64 = (assign34660_e31210).exp();
                        (assign34660_e31211, 0.0,)
                    } else {
                        let (assign34660_e31224, assign34660_e31224_d_n4,) = {
                            if (var_fn423_calc_ig__expphib > 50.0) {
                                let assign34660_e31216: f64 = (50.0_f64).exp();
                                let assign34660_e31220: f64 = (var_fn423_calc_ig__expphib - 50.0);
                                let assign34660_e31221: f64 = (1.0 + assign34660_e31220);
                                let assign34660_e31222: f64 = (assign34660_e31216 * assign34660_e31221);
                                (assign34660_e31222, (assign34660_e31216 * var_fn423_calc_ig__expphib_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign34660_e31224, assign34660_e31224_d_n4,)
                    }
                };
                (assign34660_e31225, assign34660_e31225_d_n4,)
            }
        };
        (assign34660_e31226, assign34660_e31226_d_n4,)
    } else {
        (var_fn423_calc_ig__t0, var_fn423_calc_ig__t0_dn4,)
    }
};
        var_fn423_calc_ig__t0 = assign34660_e31228;
        var_fn423_calc_ig__t0_dn4 = assign34660_e31228_d_n4;

        let (assign34670_e31239, assign34670_e31239_d_n4, assign34670_e31239_d_n8, assign34670_e31239_d_n17,) = {
    if (var_guard417 != 0.0) {
        let assign34670_e31232: f64 = (-var_fn423_calc_ig__vgin);
        let assign34670_e31234: f64 = (assign34670_e31232 - var_fn423_calc_ig__vbdgin);
        let assign34670_e31235: f64 = (var_fn423_calc_ig__pbdgin * assign34670_e31234);
        let assign34670_e31237: f64 = (assign34670_e31235 + var_fn423_calc_ig__expphib);
        (assign34670_e31237, var_fn423_calc_ig__expphib_dn4, (var_fn423_calc_ig__pbdgin * (-var_fn423_calc_ig__vgin_dn8)), (var_fn423_calc_ig__pbdgin * (-var_fn423_calc_ig__vgin_dn17)),)
    } else {
        (var_fn423_calc_ig__expbdarg1, var_fn423_calc_ig__expbdarg1_dn4, var_fn423_calc_ig__expbdarg1_dn8, var_fn423_calc_ig__expbdarg1_dn17,)
    }
};
        var_fn423_calc_ig__expbdarg1 = assign34670_e31239;
        var_fn423_calc_ig__expbdarg1_dn4 = assign34670_e31239_d_n4;
        var_fn423_calc_ig__expbdarg1_dn8 = assign34670_e31239_d_n8;
        var_fn423_calc_ig__expbdarg1_dn17 = assign34670_e31239_d_n17;

        let (assign34680_e31248, assign34680_e31248_d_n4,) = {
    if (var_guard417 != 0.0) {
        let assign34680_e31242: f64 = (-var_fn423_calc_ig__pbdgin);
        let assign34680_e31244: f64 = (assign34680_e31242 * var_fn423_calc_ig__vbdgin);
        let assign34680_e31246: f64 = (assign34680_e31244 + var_fn423_calc_ig__expphib);
        (assign34680_e31246, var_fn423_calc_ig__expphib_dn4,)
    } else {
        (var_fn423_calc_ig__expbdarg2, var_fn423_calc_ig__expbdarg2_dn4,)
    }
};
        var_fn423_calc_ig__expbdarg2 = assign34680_e31248;
        var_fn423_calc_ig__expbdarg2_dn4 = assign34680_e31248_d_n4;

        *var_fn423_calc_ig__alpha2_phit_slot = var_fn423_calc_ig__alpha2_phit;
        *var_fn423_calc_ig__alpha2_phit_dn4_slot = var_fn423_calc_ig__alpha2_phit_dn4;
        *var_fn423_calc_ig__betarecin_slot = var_fn423_calc_ig__betarecin;
        *var_fn423_calc_ig__expbd1_slot = var_fn423_calc_ig__expbd1;
        *var_fn423_calc_ig__expbd1_dn17_slot = var_fn423_calc_ig__expbd1_dn17;
        *var_fn423_calc_ig__expbd1_dn4_slot = var_fn423_calc_ig__expbd1_dn4;
        *var_fn423_calc_ig__expbd1_dn8_slot = var_fn423_calc_ig__expbd1_dn8;
        *var_fn423_calc_ig__expbd1_vgsat_slot = var_fn423_calc_ig__expbd1_vgsat;
        *var_fn423_calc_ig__expbd1_vgsat_dn4_slot = var_fn423_calc_ig__expbd1_vgsat_dn4;
        *var_fn423_calc_ig__expbd2_slot = var_fn423_calc_ig__expbd2;
        *var_fn423_calc_ig__expbd2_dn4_slot = var_fn423_calc_ig__expbd2_dn4;
        *var_fn423_calc_ig__expbdarg1_slot = var_fn423_calc_ig__expbdarg1;
        *var_fn423_calc_ig__expbdarg1_dn17_slot = var_fn423_calc_ig__expbdarg1_dn17;
        *var_fn423_calc_ig__expbdarg1_dn4_slot = var_fn423_calc_ig__expbdarg1_dn4;
        *var_fn423_calc_ig__expbdarg1_dn8_slot = var_fn423_calc_ig__expbdarg1_dn8;
        *var_fn423_calc_ig__expbdarg1_vgsat_slot = var_fn423_calc_ig__expbdarg1_vgsat;
        *var_fn423_calc_ig__expbdarg1_vgsat_dn4_slot = var_fn423_calc_ig__expbdarg1_vgsat_dn4;
        *var_fn423_calc_ig__expbdarg2_slot = var_fn423_calc_ig__expbdarg2;
        *var_fn423_calc_ig__expbdarg2_dn4_slot = var_fn423_calc_ig__expbdarg2_dn4;
        *var_fn423_calc_ig__expffvarg_slot = var_fn423_calc_ig__expffvarg;
        *var_fn423_calc_ig__expffvarg_dn17_slot = var_fn423_calc_ig__expffvarg_dn17;
        *var_fn423_calc_ig__expffvarg_dn4_slot = var_fn423_calc_ig__expffvarg_dn4;
        *var_fn423_calc_ig__expffvarg_dn8_slot = var_fn423_calc_ig__expffvarg_dn8;
        *var_fn423_calc_ig__expifor_slot = var_fn423_calc_ig__expifor;
        *var_fn423_calc_ig__expifor_dn17_slot = var_fn423_calc_ig__expifor_dn17;
        *var_fn423_calc_ig__expifor_dn4_slot = var_fn423_calc_ig__expifor_dn4;
        *var_fn423_calc_ig__expifor_dn8_slot = var_fn423_calc_ig__expifor_dn8;
        *var_fn423_calc_ig__expifor_hinj_slot = var_fn423_calc_ig__expifor_hinj;
        *var_fn423_calc_ig__expifor_hinj_dn17_slot = var_fn423_calc_ig__expifor_hinj_dn17;
        *var_fn423_calc_ig__expifor_hinj_dn4_slot = var_fn423_calc_ig__expifor_hinj_dn4;
        *var_fn423_calc_ig__expifor_hinj_dn8_slot = var_fn423_calc_ig__expifor_hinj_dn8;
        *var_fn423_calc_ig__expifor_hinj_vgsat_slot = var_fn423_calc_ig__expifor_hinj_vgsat;
        *var_fn423_calc_ig__expifor_hinj_vgsat_dn4_slot = var_fn423_calc_ig__expifor_hinj_vgsat_dn4;
        *var_fn423_calc_ig__expifor_nohinj_vgsat_slot = var_fn423_calc_ig__expifor_nohinj_vgsat;
        *var_fn423_calc_ig__expifor_nohinj_vgsat_dn4_slot = var_fn423_calc_ig__expifor_nohinj_vgsat_dn4;
        *var_fn423_calc_ig__expiforarg_slot = var_fn423_calc_ig__expiforarg;
        *var_fn423_calc_ig__expiforarg_dn17_slot = var_fn423_calc_ig__expiforarg_dn17;
        *var_fn423_calc_ig__expiforarg_dn4_slot = var_fn423_calc_ig__expiforarg_dn4;
        *var_fn423_calc_ig__expiforarg_dn8_slot = var_fn423_calc_ig__expiforarg_dn8;
        *var_fn423_calc_ig__expiforarg_hinj_slot = var_fn423_calc_ig__expiforarg_hinj;
        *var_fn423_calc_ig__expiforarg_hinj_dn17_slot = var_fn423_calc_ig__expiforarg_hinj_dn17;
        *var_fn423_calc_ig__expiforarg_hinj_dn4_slot = var_fn423_calc_ig__expiforarg_hinj_dn4;
        *var_fn423_calc_ig__expiforarg_hinj_dn8_slot = var_fn423_calc_ig__expiforarg_hinj_dn8;
        *var_fn423_calc_ig__expiforarg_hinj_vgsat_slot = var_fn423_calc_ig__expiforarg_hinj_vgsat;
        *var_fn423_calc_ig__expiforarg_hinj_vgsat_dn4_slot = var_fn423_calc_ig__expiforarg_hinj_vgsat_dn4;
        *var_fn423_calc_ig__expiforarg_nohinj_vgsat_slot = var_fn423_calc_ig__expiforarg_nohinj_vgsat;
        *var_fn423_calc_ig__expiforarg_nohinj_vgsat_dn4_slot = var_fn423_calc_ig__expiforarg_nohinj_vgsat_dn4;
        *var_fn423_calc_ig__expirev_slot = var_fn423_calc_ig__expirev;
        *var_fn423_calc_ig__expirev_dn17_slot = var_fn423_calc_ig__expirev_dn17;
        *var_fn423_calc_ig__expirev_dn4_slot = var_fn423_calc_ig__expirev_dn4;
        *var_fn423_calc_ig__expirev_dn8_slot = var_fn423_calc_ig__expirev_dn8;
        *var_fn423_calc_ig__expirevarg_slot = var_fn423_calc_ig__expirevarg;
        *var_fn423_calc_ig__expirevarg_dn17_slot = var_fn423_calc_ig__expirevarg_dn17;
        *var_fn423_calc_ig__expirevarg_dn4_slot = var_fn423_calc_ig__expirevarg_dn4;
        *var_fn423_calc_ig__expirevarg_dn8_slot = var_fn423_calc_ig__expirevarg_dn8;
        *var_fn423_calc_ig__expphib_slot = var_fn423_calc_ig__expphib;
        *var_fn423_calc_ig__expphib_dn4_slot = var_fn423_calc_ig__expphib_dn4;
        *var_fn423_calc_ig__ffvgin_slot = var_fn423_calc_ig__ffvgin;
        *var_fn423_calc_ig__ffvgin_dn17_slot = var_fn423_calc_ig__ffvgin_dn17;
        *var_fn423_calc_ig__ffvgin_dn4_slot = var_fn423_calc_ig__ffvgin_dn4;
        *var_fn423_calc_ig__ffvgin_dn8_slot = var_fn423_calc_ig__ffvgin_dn8;
        *var_fn423_calc_ig__frecgin_slot = var_fn423_calc_ig__frecgin;
        *var_fn423_calc_ig__frecgin_dn17_slot = var_fn423_calc_ig__frecgin_dn17;
        *var_fn423_calc_ig__frecgin_dn8_slot = var_fn423_calc_ig__frecgin_dn8;
        *var_fn423_calc_ig__iginbd_slot = var_fn423_calc_ig__iginbd;
        *var_fn423_calc_ig__iginbd_dn17_slot = var_fn423_calc_ig__iginbd_dn17;
        *var_fn423_calc_ig__iginbd_dn4_slot = var_fn423_calc_ig__iginbd_dn4;
        *var_fn423_calc_ig__iginbd_dn8_slot = var_fn423_calc_ig__iginbd_dn8;
        *var_fn423_calc_ig__iginbd_vgsat_slot = var_fn423_calc_ig__iginbd_vgsat;
        *var_fn423_calc_ig__iginbd_vgsat_dn4_slot = var_fn423_calc_ig__iginbd_vgsat_dn4;
        *var_fn423_calc_ig__igindiode_slot = var_fn423_calc_ig__igindiode;
        *var_fn423_calc_ig__igindiode_dn17_slot = var_fn423_calc_ig__igindiode_dn17;
        *var_fn423_calc_ig__igindiode_dn4_slot = var_fn423_calc_ig__igindiode_dn4;
        *var_fn423_calc_ig__igindiode_dn8_slot = var_fn423_calc_ig__igindiode_dn8;
        *var_fn423_calc_ig__igindiode_hinj_slot = var_fn423_calc_ig__igindiode_hinj;
        *var_fn423_calc_ig__igindiode_hinj_dn17_slot = var_fn423_calc_ig__igindiode_hinj_dn17;
        *var_fn423_calc_ig__igindiode_hinj_dn4_slot = var_fn423_calc_ig__igindiode_hinj_dn4;
        *var_fn423_calc_ig__igindiode_hinj_dn8_slot = var_fn423_calc_ig__igindiode_hinj_dn8;
        *var_fn423_calc_ig__igindiode_hinj_pre_slot = var_fn423_calc_ig__igindiode_hinj_pre;
        *var_fn423_calc_ig__igindiode_hinj_pre_dn4_slot = var_fn423_calc_ig__igindiode_hinj_pre_dn4;
        *var_fn423_calc_ig__igindiode_hinj_vgsat_slot = var_fn423_calc_ig__igindiode_hinj_vgsat;
        *var_fn423_calc_ig__igindiode_hinj_vgsat_dn4_slot = var_fn423_calc_ig__igindiode_hinj_vgsat_dn4;
        *var_fn423_calc_ig__igindiode_nohinj_slot = var_fn423_calc_ig__igindiode_nohinj;
        *var_fn423_calc_ig__igindiode_nohinj_dn17_slot = var_fn423_calc_ig__igindiode_nohinj_dn17;
        *var_fn423_calc_ig__igindiode_nohinj_dn4_slot = var_fn423_calc_ig__igindiode_nohinj_dn4;
        *var_fn423_calc_ig__igindiode_nohinj_dn8_slot = var_fn423_calc_ig__igindiode_nohinj_dn8;
        *var_fn423_calc_ig__igindiode_nohinj_vgsat_slot = var_fn423_calc_ig__igindiode_nohinj_vgsat;
        *var_fn423_calc_ig__igindiode_nohinj_vgsat_dn4_slot = var_fn423_calc_ig__igindiode_nohinj_vgsat_dn4;
        *var_fn423_calc_ig__iginrec_slot = var_fn423_calc_ig__iginrec;
        *var_fn423_calc_ig__iginrec_dn17_slot = var_fn423_calc_ig__iginrec_dn17;
        *var_fn423_calc_ig__iginrec_dn4_slot = var_fn423_calc_ig__iginrec_dn4;
        *var_fn423_calc_ig__iginrec_dn8_slot = var_fn423_calc_ig__iginrec_dn8;
        *var_fn423_calc_ig__igout_slot = var_fn423_calc_ig__igout;
        *var_fn423_calc_ig__igout_dn17_slot = var_fn423_calc_ig__igout_dn17;
        *var_fn423_calc_ig__igout_dn4_slot = var_fn423_calc_ig__igout_dn4;
        *var_fn423_calc_ig__igout_dn8_slot = var_fn423_calc_ig__igout_dn8;
        *var_fn423_calc_ig__irecin_slot = var_fn423_calc_ig__irecin;
        *var_fn423_calc_ig__pg_param1_slot = var_fn423_calc_ig__pg_param1;
        *var_fn423_calc_ig__pg_paramin_hinj_slot = var_fn423_calc_ig__pg_paramin_hinj;
        *var_fn423_calc_ig__pgsrecin_slot = var_fn423_calc_ig__pgsrecin;
        *var_fn423_calc_ig__t0_slot = var_fn423_calc_ig__t0;
        *var_fn423_calc_ig__t0_dn4_slot = var_fn423_calc_ig__t0_dn4;
        *var_fn423_calc_ig__type_slot = var_fn423_calc_ig__type;
        *var_fn423_calc_ig__vgsatqin_slot = var_fn423_calc_ig__vgsatqin;
        *var_fn423_calc_ig__vjg_slot = var_fn423_calc_ig__vjg;
    }

    pub(super) fn stamp_transient_block_85(
        var_fn423_calc_ig__expbdarg1: f64,
        var_fn423_calc_ig__expbdarg1_dn17: f64,
        var_fn423_calc_ig__expbdarg1_dn4: f64,
        var_fn423_calc_ig__expbdarg1_dn8: f64,
        var_fn423_calc_ig__expbdarg2: f64,
        var_fn423_calc_ig__expbdarg2_dn4: f64,
        var_fn423_calc_ig__expphib: f64,
        var_fn423_calc_ig__expphib_dn4: f64,
        var_fn423_calc_ig__fracin: f64,
        var_fn423_calc_ig__ijin: f64,
        var_fn423_calc_ig__kbdgatein: f64,
        var_fn423_calc_ig__ngf: f64,
        var_fn423_calc_ig__pbdgin: f64,
        var_fn423_calc_ig__pg_paramin: f64,
        var_fn423_calc_ig__phitin: f64,
        var_fn423_calc_ig__phitin_dn4: f64,
        var_fn423_calc_ig__t0: f64,
        var_fn423_calc_ig__t0_dn4: f64,
        var_fn423_calc_ig__tfacdiodein: f64,
        var_fn423_calc_ig__tfacdiodein_dn4: f64,
        var_fn423_calc_ig__type: f64,
        var_fn423_calc_ig__vbdgin: f64,
        var_fn423_calc_ig__vgin: f64,
        var_fn423_calc_ig__vgin_dn17: f64,
        var_fn423_calc_ig__vgin_dn8: f64,
        var_fn423_calc_ig__vgsatin: f64,
        var_fn423_calc_ig__w: f64,
        var_guard417: f64,
        var_fn423_calc_ig__expbd1_slot: &mut f64,
        var_fn423_calc_ig__expbd1_dn17_slot: &mut f64,
        var_fn423_calc_ig__expbd1_dn4_slot: &mut f64,
        var_fn423_calc_ig__expbd1_dn8_slot: &mut f64,
        var_fn423_calc_ig__expbd1_vgsat_slot: &mut f64,
        var_fn423_calc_ig__expbd1_vgsat_dn4_slot: &mut f64,
        var_fn423_calc_ig__expbd2_slot: &mut f64,
        var_fn423_calc_ig__expbd2_dn4_slot: &mut f64,
        var_fn423_calc_ig__expbdarg1_vgsat_slot: &mut f64,
        var_fn423_calc_ig__expbdarg1_vgsat_dn4_slot: &mut f64,
        var_fn423_calc_ig__expifor_slot: &mut f64,
        var_fn423_calc_ig__expifor_dn17_slot: &mut f64,
        var_fn423_calc_ig__expifor_dn4_slot: &mut f64,
        var_fn423_calc_ig__expifor_dn8_slot: &mut f64,
        var_fn423_calc_ig__expifor_hinj_slot: &mut f64,
        var_fn423_calc_ig__expifor_hinj_dn17_slot: &mut f64,
        var_fn423_calc_ig__expifor_hinj_dn4_slot: &mut f64,
        var_fn423_calc_ig__expifor_hinj_dn8_slot: &mut f64,
        var_fn423_calc_ig__expifor_hinj_vgsat_slot: &mut f64,
        var_fn423_calc_ig__expifor_hinj_vgsat_dn4_slot: &mut f64,
        var_fn423_calc_ig__expifor_nohinj_vgsat_slot: &mut f64,
        var_fn423_calc_ig__expifor_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn423_calc_ig__expiforarg_slot: &mut f64,
        var_fn423_calc_ig__expiforarg_dn17_slot: &mut f64,
        var_fn423_calc_ig__expiforarg_dn4_slot: &mut f64,
        var_fn423_calc_ig__expiforarg_dn8_slot: &mut f64,
        var_fn423_calc_ig__expiforarg_hinj_slot: &mut f64,
        var_fn423_calc_ig__expiforarg_hinj_dn17_slot: &mut f64,
        var_fn423_calc_ig__expiforarg_hinj_dn4_slot: &mut f64,
        var_fn423_calc_ig__expiforarg_hinj_dn8_slot: &mut f64,
        var_fn423_calc_ig__expiforarg_hinj_vgsat_slot: &mut f64,
        var_fn423_calc_ig__expiforarg_hinj_vgsat_dn4_slot: &mut f64,
        var_fn423_calc_ig__expiforarg_nohinj_vgsat_slot: &mut f64,
        var_fn423_calc_ig__expiforarg_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn423_calc_ig__iginbd_slot: &mut f64,
        var_fn423_calc_ig__iginbd_dn17_slot: &mut f64,
        var_fn423_calc_ig__iginbd_dn4_slot: &mut f64,
        var_fn423_calc_ig__iginbd_dn8_slot: &mut f64,
        var_fn423_calc_ig__iginbd_vgsat_slot: &mut f64,
        var_fn423_calc_ig__iginbd_vgsat_dn4_slot: &mut f64,
        var_fn423_calc_ig__igindiode_slot: &mut f64,
        var_fn423_calc_ig__igindiode_dn17_slot: &mut f64,
        var_fn423_calc_ig__igindiode_dn4_slot: &mut f64,
        var_fn423_calc_ig__igindiode_dn8_slot: &mut f64,
        var_fn423_calc_ig__igindiode_hinj_slot: &mut f64,
        var_fn423_calc_ig__igindiode_hinj_dn17_slot: &mut f64,
        var_fn423_calc_ig__igindiode_hinj_dn4_slot: &mut f64,
        var_fn423_calc_ig__igindiode_hinj_dn8_slot: &mut f64,
        var_fn423_calc_ig__igindiode_hinj_pre_slot: &mut f64,
        var_fn423_calc_ig__igindiode_hinj_pre_dn4_slot: &mut f64,
        var_fn423_calc_ig__igindiode_hinj_vgsat_slot: &mut f64,
        var_fn423_calc_ig__igindiode_hinj_vgsat_dn4_slot: &mut f64,
        var_fn423_calc_ig__igindiode_nohinj_slot: &mut f64,
        var_fn423_calc_ig__igindiode_nohinj_dn17_slot: &mut f64,
        var_fn423_calc_ig__igindiode_nohinj_dn4_slot: &mut f64,
        var_fn423_calc_ig__igindiode_nohinj_dn8_slot: &mut f64,
        var_fn423_calc_ig__igindiode_nohinj_vgsat_slot: &mut f64,
        var_fn423_calc_ig__igindiode_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn423_calc_ig__isdiodeout_slot: &mut f64,
        var_fn423_calc_ig__isdiodeout_dn4_slot: &mut f64,
        var_fn423_calc_ig__pg_paramin_hinj_slot: &mut f64,
        var_guard424_slot: &mut f64,
        var_guard425_slot: &mut f64,
    ) {
        let mut var_fn423_calc_ig__expbd1: f64 = *var_fn423_calc_ig__expbd1_slot;
        let mut var_fn423_calc_ig__expbd1_dn17: f64 = *var_fn423_calc_ig__expbd1_dn17_slot;
        let mut var_fn423_calc_ig__expbd1_dn4: f64 = *var_fn423_calc_ig__expbd1_dn4_slot;
        let mut var_fn423_calc_ig__expbd1_dn8: f64 = *var_fn423_calc_ig__expbd1_dn8_slot;
        let mut var_fn423_calc_ig__expbd1_vgsat: f64 = *var_fn423_calc_ig__expbd1_vgsat_slot;
        let mut var_fn423_calc_ig__expbd1_vgsat_dn4: f64 = *var_fn423_calc_ig__expbd1_vgsat_dn4_slot;
        let mut var_fn423_calc_ig__expbd2: f64 = *var_fn423_calc_ig__expbd2_slot;
        let mut var_fn423_calc_ig__expbd2_dn4: f64 = *var_fn423_calc_ig__expbd2_dn4_slot;
        let mut var_fn423_calc_ig__expbdarg1_vgsat: f64 = *var_fn423_calc_ig__expbdarg1_vgsat_slot;
        let mut var_fn423_calc_ig__expbdarg1_vgsat_dn4: f64 = *var_fn423_calc_ig__expbdarg1_vgsat_dn4_slot;
        let mut var_fn423_calc_ig__expifor: f64 = *var_fn423_calc_ig__expifor_slot;
        let mut var_fn423_calc_ig__expifor_dn17: f64 = *var_fn423_calc_ig__expifor_dn17_slot;
        let mut var_fn423_calc_ig__expifor_dn4: f64 = *var_fn423_calc_ig__expifor_dn4_slot;
        let mut var_fn423_calc_ig__expifor_dn8: f64 = *var_fn423_calc_ig__expifor_dn8_slot;
        let mut var_fn423_calc_ig__expifor_hinj: f64 = *var_fn423_calc_ig__expifor_hinj_slot;
        let mut var_fn423_calc_ig__expifor_hinj_dn17: f64 = *var_fn423_calc_ig__expifor_hinj_dn17_slot;
        let mut var_fn423_calc_ig__expifor_hinj_dn4: f64 = *var_fn423_calc_ig__expifor_hinj_dn4_slot;
        let mut var_fn423_calc_ig__expifor_hinj_dn8: f64 = *var_fn423_calc_ig__expifor_hinj_dn8_slot;
        let mut var_fn423_calc_ig__expifor_hinj_vgsat: f64 = *var_fn423_calc_ig__expifor_hinj_vgsat_slot;
        let mut var_fn423_calc_ig__expifor_hinj_vgsat_dn4: f64 = *var_fn423_calc_ig__expifor_hinj_vgsat_dn4_slot;
        let mut var_fn423_calc_ig__expifor_nohinj_vgsat: f64 = *var_fn423_calc_ig__expifor_nohinj_vgsat_slot;
        let mut var_fn423_calc_ig__expifor_nohinj_vgsat_dn4: f64 = *var_fn423_calc_ig__expifor_nohinj_vgsat_dn4_slot;
        let mut var_fn423_calc_ig__expiforarg: f64 = *var_fn423_calc_ig__expiforarg_slot;
        let mut var_fn423_calc_ig__expiforarg_dn17: f64 = *var_fn423_calc_ig__expiforarg_dn17_slot;
        let mut var_fn423_calc_ig__expiforarg_dn4: f64 = *var_fn423_calc_ig__expiforarg_dn4_slot;
        let mut var_fn423_calc_ig__expiforarg_dn8: f64 = *var_fn423_calc_ig__expiforarg_dn8_slot;
        let mut var_fn423_calc_ig__expiforarg_hinj: f64 = *var_fn423_calc_ig__expiforarg_hinj_slot;
        let mut var_fn423_calc_ig__expiforarg_hinj_dn17: f64 = *var_fn423_calc_ig__expiforarg_hinj_dn17_slot;
        let mut var_fn423_calc_ig__expiforarg_hinj_dn4: f64 = *var_fn423_calc_ig__expiforarg_hinj_dn4_slot;
        let mut var_fn423_calc_ig__expiforarg_hinj_dn8: f64 = *var_fn423_calc_ig__expiforarg_hinj_dn8_slot;
        let mut var_fn423_calc_ig__expiforarg_hinj_vgsat: f64 = *var_fn423_calc_ig__expiforarg_hinj_vgsat_slot;
        let mut var_fn423_calc_ig__expiforarg_hinj_vgsat_dn4: f64 = *var_fn423_calc_ig__expiforarg_hinj_vgsat_dn4_slot;
        let mut var_fn423_calc_ig__expiforarg_nohinj_vgsat: f64 = *var_fn423_calc_ig__expiforarg_nohinj_vgsat_slot;
        let mut var_fn423_calc_ig__expiforarg_nohinj_vgsat_dn4: f64 = *var_fn423_calc_ig__expiforarg_nohinj_vgsat_dn4_slot;
        let mut var_fn423_calc_ig__iginbd: f64 = *var_fn423_calc_ig__iginbd_slot;
        let mut var_fn423_calc_ig__iginbd_dn17: f64 = *var_fn423_calc_ig__iginbd_dn17_slot;
        let mut var_fn423_calc_ig__iginbd_dn4: f64 = *var_fn423_calc_ig__iginbd_dn4_slot;
        let mut var_fn423_calc_ig__iginbd_dn8: f64 = *var_fn423_calc_ig__iginbd_dn8_slot;
        let mut var_fn423_calc_ig__iginbd_vgsat: f64 = *var_fn423_calc_ig__iginbd_vgsat_slot;
        let mut var_fn423_calc_ig__iginbd_vgsat_dn4: f64 = *var_fn423_calc_ig__iginbd_vgsat_dn4_slot;
        let mut var_fn423_calc_ig__igindiode: f64 = *var_fn423_calc_ig__igindiode_slot;
        let mut var_fn423_calc_ig__igindiode_dn17: f64 = *var_fn423_calc_ig__igindiode_dn17_slot;
        let mut var_fn423_calc_ig__igindiode_dn4: f64 = *var_fn423_calc_ig__igindiode_dn4_slot;
        let mut var_fn423_calc_ig__igindiode_dn8: f64 = *var_fn423_calc_ig__igindiode_dn8_slot;
        let mut var_fn423_calc_ig__igindiode_hinj: f64 = *var_fn423_calc_ig__igindiode_hinj_slot;
        let mut var_fn423_calc_ig__igindiode_hinj_dn17: f64 = *var_fn423_calc_ig__igindiode_hinj_dn17_slot;
        let mut var_fn423_calc_ig__igindiode_hinj_dn4: f64 = *var_fn423_calc_ig__igindiode_hinj_dn4_slot;
        let mut var_fn423_calc_ig__igindiode_hinj_dn8: f64 = *var_fn423_calc_ig__igindiode_hinj_dn8_slot;
        let mut var_fn423_calc_ig__igindiode_hinj_pre: f64 = *var_fn423_calc_ig__igindiode_hinj_pre_slot;
        let mut var_fn423_calc_ig__igindiode_hinj_pre_dn4: f64 = *var_fn423_calc_ig__igindiode_hinj_pre_dn4_slot;
        let mut var_fn423_calc_ig__igindiode_hinj_vgsat: f64 = *var_fn423_calc_ig__igindiode_hinj_vgsat_slot;
        let mut var_fn423_calc_ig__igindiode_hinj_vgsat_dn4: f64 = *var_fn423_calc_ig__igindiode_hinj_vgsat_dn4_slot;
        let mut var_fn423_calc_ig__igindiode_nohinj: f64 = *var_fn423_calc_ig__igindiode_nohinj_slot;
        let mut var_fn423_calc_ig__igindiode_nohinj_dn17: f64 = *var_fn423_calc_ig__igindiode_nohinj_dn17_slot;
        let mut var_fn423_calc_ig__igindiode_nohinj_dn4: f64 = *var_fn423_calc_ig__igindiode_nohinj_dn4_slot;
        let mut var_fn423_calc_ig__igindiode_nohinj_dn8: f64 = *var_fn423_calc_ig__igindiode_nohinj_dn8_slot;
        let mut var_fn423_calc_ig__igindiode_nohinj_vgsat: f64 = *var_fn423_calc_ig__igindiode_nohinj_vgsat_slot;
        let mut var_fn423_calc_ig__igindiode_nohinj_vgsat_dn4: f64 = *var_fn423_calc_ig__igindiode_nohinj_vgsat_dn4_slot;
        let mut var_fn423_calc_ig__isdiodeout: f64 = *var_fn423_calc_ig__isdiodeout_slot;
        let mut var_fn423_calc_ig__isdiodeout_dn4: f64 = *var_fn423_calc_ig__isdiodeout_dn4_slot;
        let mut var_fn423_calc_ig__pg_paramin_hinj: f64 = *var_fn423_calc_ig__pg_paramin_hinj_slot;
        let mut var_guard424: f64 = *var_guard424_slot;
        let mut var_guard425: f64 = *var_guard425_slot;

        let (assign34690_e31290, assign34690_e31290_d_n4, assign34690_e31290_d_n8, assign34690_e31290_d_n17,) = {
    if (var_guard417 != 0.0) {
        let assign34690_e31256: f64 = (-50.0);
        let (assign34690_e31288, assign34690_e31288_d_n4, assign34690_e31288_d_n8, assign34690_e31288_d_n17,) = {
            if ((!(var_fn423_calc_ig__expbdarg1 > 50.0)) && (!(var_fn423_calc_ig__expbdarg1 < assign34690_e31256))) {
                let assign34690_e31261: f64 = (var_fn423_calc_ig__expbdarg1).exp();
                (assign34690_e31261, (assign34690_e31261 * var_fn423_calc_ig__expbdarg1_dn4), (assign34690_e31261 * var_fn423_calc_ig__expbdarg1_dn8), (assign34690_e31261 * var_fn423_calc_ig__expbdarg1_dn17),)
            } else {
                let assign34690_e31268: f64 = (-50.0);
                let (assign34690_e31287, assign34690_e31287_d_n4, assign34690_e31287_d_n8, assign34690_e31287_d_n17,) = {
                    if ((!(var_fn423_calc_ig__expbdarg1 > 50.0)) && (var_fn423_calc_ig__expbdarg1 < assign34690_e31268)) {
                        let assign34690_e31272: f64 = (-50.0);
                        let assign34690_e31273: f64 = (assign34690_e31272).exp();
                        (assign34690_e31273, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign34690_e31286, assign34690_e31286_d_n4, assign34690_e31286_d_n8, assign34690_e31286_d_n17,) = {
                            if (var_fn423_calc_ig__expbdarg1 > 50.0) {
                                let assign34690_e31278: f64 = (50.0_f64).exp();
                                let assign34690_e31282: f64 = (var_fn423_calc_ig__expbdarg1 - 50.0);
                                let assign34690_e31283: f64 = (1.0 + assign34690_e31282);
                                let assign34690_e31284: f64 = (assign34690_e31278 * assign34690_e31283);
                                (assign34690_e31284, (assign34690_e31278 * var_fn423_calc_ig__expbdarg1_dn4), (assign34690_e31278 * var_fn423_calc_ig__expbdarg1_dn8), (assign34690_e31278 * var_fn423_calc_ig__expbdarg1_dn17),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign34690_e31286, assign34690_e31286_d_n4, assign34690_e31286_d_n8, assign34690_e31286_d_n17,)
                    }
                };
                (assign34690_e31287, assign34690_e31287_d_n4, assign34690_e31287_d_n8, assign34690_e31287_d_n17,)
            }
        };
        (assign34690_e31288, assign34690_e31288_d_n4, assign34690_e31288_d_n8, assign34690_e31288_d_n17,)
    } else {
        (var_fn423_calc_ig__expbd1, var_fn423_calc_ig__expbd1_dn4, var_fn423_calc_ig__expbd1_dn8, var_fn423_calc_ig__expbd1_dn17,)
    }
};
        var_fn423_calc_ig__expbd1 = assign34690_e31290;
        var_fn423_calc_ig__expbd1_dn4 = assign34690_e31290_d_n4;
        var_fn423_calc_ig__expbd1_dn8 = assign34690_e31290_d_n8;
        var_fn423_calc_ig__expbd1_dn17 = assign34690_e31290_d_n17;

        let (assign34700_e31332, assign34700_e31332_d_n4,) = {
    if (var_guard417 != 0.0) {
        let assign34700_e31298: f64 = (-50.0);
        let (assign34700_e31330, assign34700_e31330_d_n4,) = {
            if ((!(var_fn423_calc_ig__expbdarg2 > 50.0)) && (!(var_fn423_calc_ig__expbdarg2 < assign34700_e31298))) {
                let assign34700_e31303: f64 = (var_fn423_calc_ig__expbdarg2).exp();
                (assign34700_e31303, (assign34700_e31303 * var_fn423_calc_ig__expbdarg2_dn4),)
            } else {
                let assign34700_e31310: f64 = (-50.0);
                let (assign34700_e31329, assign34700_e31329_d_n4,) = {
                    if ((!(var_fn423_calc_ig__expbdarg2 > 50.0)) && (var_fn423_calc_ig__expbdarg2 < assign34700_e31310)) {
                        let assign34700_e31314: f64 = (-50.0);
                        let assign34700_e31315: f64 = (assign34700_e31314).exp();
                        (assign34700_e31315, 0.0,)
                    } else {
                        let (assign34700_e31328, assign34700_e31328_d_n4,) = {
                            if (var_fn423_calc_ig__expbdarg2 > 50.0) {
                                let assign34700_e31320: f64 = (50.0_f64).exp();
                                let assign34700_e31324: f64 = (var_fn423_calc_ig__expbdarg2 - 50.0);
                                let assign34700_e31325: f64 = (1.0 + assign34700_e31324);
                                let assign34700_e31326: f64 = (assign34700_e31320 * assign34700_e31325);
                                (assign34700_e31326, (assign34700_e31320 * var_fn423_calc_ig__expbdarg2_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign34700_e31328, assign34700_e31328_d_n4,)
                    }
                };
                (assign34700_e31329, assign34700_e31329_d_n4,)
            }
        };
        (assign34700_e31330, assign34700_e31330_d_n4,)
    } else {
        (var_fn423_calc_ig__expbd2, var_fn423_calc_ig__expbd2_dn4,)
    }
};
        var_fn423_calc_ig__expbd2 = assign34700_e31332;
        var_fn423_calc_ig__expbd2_dn4 = assign34700_e31332_d_n4;

        let (assign34710_e31338, assign34710_e31338_d_n4, assign34710_e31338_d_n8, assign34710_e31338_d_n17,) = {
    if (var_guard417 != 0.0) {
        let assign34710_e31336: f64 = (var_fn423_calc_ig__expbd1 - var_fn423_calc_ig__expbd2);
        (assign34710_e31336, (var_fn423_calc_ig__expbd1_dn4 - var_fn423_calc_ig__expbd2_dn4), var_fn423_calc_ig__expbd1_dn8, var_fn423_calc_ig__expbd1_dn17,)
    } else {
        (var_fn423_calc_ig__iginbd, var_fn423_calc_ig__iginbd_dn4, var_fn423_calc_ig__iginbd_dn8, var_fn423_calc_ig__iginbd_dn17,)
    }
};
        var_fn423_calc_ig__iginbd = assign34710_e31338;
        var_fn423_calc_ig__iginbd_dn4 = assign34710_e31338_d_n4;
        var_fn423_calc_ig__iginbd_dn8 = assign34710_e31338_d_n8;
        var_fn423_calc_ig__iginbd_dn17 = assign34710_e31338_d_n17;

        let (assign34720_e31350, assign34720_e31350_d_n4,) = {
    if (var_guard417 != 0.0) {
        let assign34720_e31342: f64 = (var_fn423_calc_ig__type * var_fn423_calc_ig__w);
        let assign34720_e31344: f64 = (assign34720_e31342 * var_fn423_calc_ig__ngf);
        let assign34720_e31346: f64 = (assign34720_e31344 * var_fn423_calc_ig__ijin);
        let assign34720_e31348: f64 = (assign34720_e31346 * var_fn423_calc_ig__tfacdiodein);
        (assign34720_e31348, (assign34720_e31346 * var_fn423_calc_ig__tfacdiodein_dn4),)
    } else {
        (var_fn423_calc_ig__isdiodeout, var_fn423_calc_ig__isdiodeout_dn4,)
    }
};
        var_fn423_calc_ig__isdiodeout = assign34720_e31350;
        var_fn423_calc_ig__isdiodeout_dn4 = assign34720_e31350_d_n4;

        let (assign34730_e31360, assign34730_e31360_d_n4, assign34730_e31360_d_n8, assign34730_e31360_d_n17,) = {
    if (var_guard417 != 0.0) {
        let assign34730_e31354: f64 = (var_fn423_calc_ig__pg_paramin / var_fn423_calc_ig__phitin);
        let assign34730_e31356: f64 = (assign34730_e31354 * var_fn423_calc_ig__vgin);
        let assign34730_e31358: f64 = (assign34730_e31356 + var_fn423_calc_ig__expphib);
        (assign34730_e31358, (((-((var_fn423_calc_ig__pg_paramin * var_fn423_calc_ig__phitin_dn4) / (var_fn423_calc_ig__phitin * var_fn423_calc_ig__phitin))) * var_fn423_calc_ig__vgin) + var_fn423_calc_ig__expphib_dn4), (assign34730_e31354 * var_fn423_calc_ig__vgin_dn8), (assign34730_e31354 * var_fn423_calc_ig__vgin_dn17),)
    } else {
        (var_fn423_calc_ig__expiforarg, var_fn423_calc_ig__expiforarg_dn4, var_fn423_calc_ig__expiforarg_dn8, var_fn423_calc_ig__expiforarg_dn17,)
    }
};
        var_fn423_calc_ig__expiforarg = assign34730_e31360;
        var_fn423_calc_ig__expiforarg_dn4 = assign34730_e31360_d_n4;
        var_fn423_calc_ig__expiforarg_dn8 = assign34730_e31360_d_n8;
        var_fn423_calc_ig__expiforarg_dn17 = assign34730_e31360_d_n17;

        let (assign34740_e31402, assign34740_e31402_d_n4, assign34740_e31402_d_n8, assign34740_e31402_d_n17,) = {
    if (var_guard417 != 0.0) {
        let assign34740_e31368: f64 = (-50.0);
        let (assign34740_e31400, assign34740_e31400_d_n4, assign34740_e31400_d_n8, assign34740_e31400_d_n17,) = {
            if ((!(var_fn423_calc_ig__expiforarg > 50.0)) && (!(var_fn423_calc_ig__expiforarg < assign34740_e31368))) {
                let assign34740_e31373: f64 = (var_fn423_calc_ig__expiforarg).exp();
                (assign34740_e31373, (assign34740_e31373 * var_fn423_calc_ig__expiforarg_dn4), (assign34740_e31373 * var_fn423_calc_ig__expiforarg_dn8), (assign34740_e31373 * var_fn423_calc_ig__expiforarg_dn17),)
            } else {
                let assign34740_e31380: f64 = (-50.0);
                let (assign34740_e31399, assign34740_e31399_d_n4, assign34740_e31399_d_n8, assign34740_e31399_d_n17,) = {
                    if ((!(var_fn423_calc_ig__expiforarg > 50.0)) && (var_fn423_calc_ig__expiforarg < assign34740_e31380)) {
                        let assign34740_e31384: f64 = (-50.0);
                        let assign34740_e31385: f64 = (assign34740_e31384).exp();
                        (assign34740_e31385, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign34740_e31398, assign34740_e31398_d_n4, assign34740_e31398_d_n8, assign34740_e31398_d_n17,) = {
                            if (var_fn423_calc_ig__expiforarg > 50.0) {
                                let assign34740_e31390: f64 = (50.0_f64).exp();
                                let assign34740_e31394: f64 = (var_fn423_calc_ig__expiforarg - 50.0);
                                let assign34740_e31395: f64 = (1.0 + assign34740_e31394);
                                let assign34740_e31396: f64 = (assign34740_e31390 * assign34740_e31395);
                                (assign34740_e31396, (assign34740_e31390 * var_fn423_calc_ig__expiforarg_dn4), (assign34740_e31390 * var_fn423_calc_ig__expiforarg_dn8), (assign34740_e31390 * var_fn423_calc_ig__expiforarg_dn17),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign34740_e31398, assign34740_e31398_d_n4, assign34740_e31398_d_n8, assign34740_e31398_d_n17,)
                    }
                };
                (assign34740_e31399, assign34740_e31399_d_n4, assign34740_e31399_d_n8, assign34740_e31399_d_n17,)
            }
        };
        (assign34740_e31400, assign34740_e31400_d_n4, assign34740_e31400_d_n8, assign34740_e31400_d_n17,)
    } else {
        (var_fn423_calc_ig__expifor, var_fn423_calc_ig__expifor_dn4, var_fn423_calc_ig__expifor_dn8, var_fn423_calc_ig__expifor_dn17,)
    }
};
        var_fn423_calc_ig__expifor = assign34740_e31402;
        var_fn423_calc_ig__expifor_dn4 = assign34740_e31402_d_n4;
        var_fn423_calc_ig__expifor_dn8 = assign34740_e31402_d_n8;
        var_fn423_calc_ig__expifor_dn17 = assign34740_e31402_d_n17;

        let assign34750_e31405: f64 = if var_fn423_calc_ig__fracin == 1.0 { 1.0 } else { 0.0 };
        var_guard424 = assign34750_e31405;

        let (assign34760_e31419, assign34760_e31419_d_n4, assign34760_e31419_d_n8, assign34760_e31419_d_n17,) = {
    if ((var_guard417 != 0.0) && (var_guard424 != 0.0)) {
        let assign34760_e31413: f64 = (var_fn423_calc_ig__kbdgatein * var_fn423_calc_ig__iginbd);
        let assign34760_e31414: f64 = (var_fn423_calc_ig__expifor - assign34760_e31413);
        let assign34760_e31416: f64 = (assign34760_e31414 - var_fn423_calc_ig__t0);
        let assign34760_e31417: f64 = (var_fn423_calc_ig__isdiodeout * assign34760_e31416);
        (assign34760_e31417, ((var_fn423_calc_ig__isdiodeout_dn4 * assign34760_e31416) + (var_fn423_calc_ig__isdiodeout * ((var_fn423_calc_ig__expifor_dn4 - (var_fn423_calc_ig__kbdgatein * var_fn423_calc_ig__iginbd_dn4)) - var_fn423_calc_ig__t0_dn4))), (var_fn423_calc_ig__isdiodeout * (var_fn423_calc_ig__expifor_dn8 - (var_fn423_calc_ig__kbdgatein * var_fn423_calc_ig__iginbd_dn8))), (var_fn423_calc_ig__isdiodeout * (var_fn423_calc_ig__expifor_dn17 - (var_fn423_calc_ig__kbdgatein * var_fn423_calc_ig__iginbd_dn17))),)
    } else {
        (var_fn423_calc_ig__igindiode, var_fn423_calc_ig__igindiode_dn4, var_fn423_calc_ig__igindiode_dn8, var_fn423_calc_ig__igindiode_dn17,)
    }
};
        var_fn423_calc_ig__igindiode = assign34760_e31419;
        var_fn423_calc_ig__igindiode_dn4 = assign34760_e31419_d_n4;
        var_fn423_calc_ig__igindiode_dn8 = assign34760_e31419_d_n8;
        var_fn423_calc_ig__igindiode_dn17 = assign34760_e31419_d_n17;

        let (assign34770_e31433, assign34770_e31433_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard424 == 0.0)) {
        let assign34770_e31426: f64 = (-var_fn423_calc_ig__vgsatin);
        let assign34770_e31428: f64 = (assign34770_e31426 - var_fn423_calc_ig__vbdgin);
        let assign34770_e31429: f64 = (var_fn423_calc_ig__pbdgin * assign34770_e31428);
        let assign34770_e31431: f64 = (assign34770_e31429 + var_fn423_calc_ig__expphib);
        (assign34770_e31431, var_fn423_calc_ig__expphib_dn4,)
    } else {
        (var_fn423_calc_ig__expbdarg1_vgsat, var_fn423_calc_ig__expbdarg1_vgsat_dn4,)
    }
};
        var_fn423_calc_ig__expbdarg1_vgsat = assign34770_e31433;
        var_fn423_calc_ig__expbdarg1_vgsat_dn4 = assign34770_e31433_d_n4;

        let (assign34780_e31478, assign34780_e31478_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard424 == 0.0)) {
        let assign34780_e31444: f64 = (-50.0);
        let (assign34780_e31476, assign34780_e31476_d_n4,) = {
            if ((!(var_fn423_calc_ig__expbdarg1_vgsat > 50.0)) && (!(var_fn423_calc_ig__expbdarg1_vgsat < assign34780_e31444))) {
                let assign34780_e31449: f64 = (var_fn423_calc_ig__expbdarg1_vgsat).exp();
                (assign34780_e31449, (assign34780_e31449 * var_fn423_calc_ig__expbdarg1_vgsat_dn4),)
            } else {
                let assign34780_e31456: f64 = (-50.0);
                let (assign34780_e31475, assign34780_e31475_d_n4,) = {
                    if ((!(var_fn423_calc_ig__expbdarg1_vgsat > 50.0)) && (var_fn423_calc_ig__expbdarg1_vgsat < assign34780_e31456)) {
                        let assign34780_e31460: f64 = (-50.0);
                        let assign34780_e31461: f64 = (assign34780_e31460).exp();
                        (assign34780_e31461, 0.0,)
                    } else {
                        let (assign34780_e31474, assign34780_e31474_d_n4,) = {
                            if (var_fn423_calc_ig__expbdarg1_vgsat > 50.0) {
                                let assign34780_e31466: f64 = (50.0_f64).exp();
                                let assign34780_e31470: f64 = (var_fn423_calc_ig__expbdarg1_vgsat - 50.0);
                                let assign34780_e31471: f64 = (1.0 + assign34780_e31470);
                                let assign34780_e31472: f64 = (assign34780_e31466 * assign34780_e31471);
                                (assign34780_e31472, (assign34780_e31466 * var_fn423_calc_ig__expbdarg1_vgsat_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign34780_e31474, assign34780_e31474_d_n4,)
                    }
                };
                (assign34780_e31475, assign34780_e31475_d_n4,)
            }
        };
        (assign34780_e31476, assign34780_e31476_d_n4,)
    } else {
        (var_fn423_calc_ig__expbd1_vgsat, var_fn423_calc_ig__expbd1_vgsat_dn4,)
    }
};
        var_fn423_calc_ig__expbd1_vgsat = assign34780_e31478;
        var_fn423_calc_ig__expbd1_vgsat_dn4 = assign34780_e31478_d_n4;

        let (assign34790_e31487, assign34790_e31487_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard424 == 0.0)) {
        let assign34790_e31485: f64 = (var_fn423_calc_ig__expbd1_vgsat - var_fn423_calc_ig__expbd2);
        (assign34790_e31485, (var_fn423_calc_ig__expbd1_vgsat_dn4 - var_fn423_calc_ig__expbd2_dn4),)
    } else {
        (var_fn423_calc_ig__iginbd_vgsat, var_fn423_calc_ig__iginbd_vgsat_dn4,)
    }
};
        var_fn423_calc_ig__iginbd_vgsat = assign34790_e31487;
        var_fn423_calc_ig__iginbd_vgsat_dn4 = assign34790_e31487_d_n4;

        let (assign34800_e31500, assign34800_e31500_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard424 == 0.0)) {
        let assign34800_e31494: f64 = (var_fn423_calc_ig__pg_paramin / var_fn423_calc_ig__phitin);
        let assign34800_e31496: f64 = (assign34800_e31494 * var_fn423_calc_ig__vgsatin);
        let assign34800_e31498: f64 = (assign34800_e31496 + var_fn423_calc_ig__expphib);
        (assign34800_e31498, (((-((var_fn423_calc_ig__pg_paramin * var_fn423_calc_ig__phitin_dn4) / (var_fn423_calc_ig__phitin * var_fn423_calc_ig__phitin))) * var_fn423_calc_ig__vgsatin) + var_fn423_calc_ig__expphib_dn4),)
    } else {
        (var_fn423_calc_ig__expiforarg_nohinj_vgsat, var_fn423_calc_ig__expiforarg_nohinj_vgsat_dn4,)
    }
};
        var_fn423_calc_ig__expiforarg_nohinj_vgsat = assign34800_e31500;
        var_fn423_calc_ig__expiforarg_nohinj_vgsat_dn4 = assign34800_e31500_d_n4;

        let (assign34810_e31545, assign34810_e31545_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard424 == 0.0)) {
        let assign34810_e31511: f64 = (-50.0);
        let (assign34810_e31543, assign34810_e31543_d_n4,) = {
            if ((!(var_fn423_calc_ig__expiforarg_nohinj_vgsat > 50.0)) && (!(var_fn423_calc_ig__expiforarg_nohinj_vgsat < assign34810_e31511))) {
                let assign34810_e31516: f64 = (var_fn423_calc_ig__expiforarg_nohinj_vgsat).exp();
                (assign34810_e31516, (assign34810_e31516 * var_fn423_calc_ig__expiforarg_nohinj_vgsat_dn4),)
            } else {
                let assign34810_e31523: f64 = (-50.0);
                let (assign34810_e31542, assign34810_e31542_d_n4,) = {
                    if ((!(var_fn423_calc_ig__expiforarg_nohinj_vgsat > 50.0)) && (var_fn423_calc_ig__expiforarg_nohinj_vgsat < assign34810_e31523)) {
                        let assign34810_e31527: f64 = (-50.0);
                        let assign34810_e31528: f64 = (assign34810_e31527).exp();
                        (assign34810_e31528, 0.0,)
                    } else {
                        let (assign34810_e31541, assign34810_e31541_d_n4,) = {
                            if (var_fn423_calc_ig__expiforarg_nohinj_vgsat > 50.0) {
                                let assign34810_e31533: f64 = (50.0_f64).exp();
                                let assign34810_e31537: f64 = (var_fn423_calc_ig__expiforarg_nohinj_vgsat - 50.0);
                                let assign34810_e31538: f64 = (1.0 + assign34810_e31537);
                                let assign34810_e31539: f64 = (assign34810_e31533 * assign34810_e31538);
                                (assign34810_e31539, (assign34810_e31533 * var_fn423_calc_ig__expiforarg_nohinj_vgsat_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign34810_e31541, assign34810_e31541_d_n4,)
                    }
                };
                (assign34810_e31542, assign34810_e31542_d_n4,)
            }
        };
        (assign34810_e31543, assign34810_e31543_d_n4,)
    } else {
        (var_fn423_calc_ig__expifor_nohinj_vgsat, var_fn423_calc_ig__expifor_nohinj_vgsat_dn4,)
    }
};
        var_fn423_calc_ig__expifor_nohinj_vgsat = assign34810_e31545;
        var_fn423_calc_ig__expifor_nohinj_vgsat_dn4 = assign34810_e31545_d_n4;

        let (assign34820_e31558, assign34820_e31558_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard424 == 0.0)) {
        let assign34820_e31553: f64 = (var_fn423_calc_ig__kbdgatein * var_fn423_calc_ig__iginbd_vgsat);
        let assign34820_e31554: f64 = (var_fn423_calc_ig__expifor_nohinj_vgsat - assign34820_e31553);
        let assign34820_e31556: f64 = (assign34820_e31554 - var_fn423_calc_ig__t0);
        (assign34820_e31556, ((var_fn423_calc_ig__expifor_nohinj_vgsat_dn4 - (var_fn423_calc_ig__kbdgatein * var_fn423_calc_ig__iginbd_vgsat_dn4)) - var_fn423_calc_ig__t0_dn4),)
    } else {
        (var_fn423_calc_ig__igindiode_nohinj_vgsat, var_fn423_calc_ig__igindiode_nohinj_vgsat_dn4,)
    }
};
        var_fn423_calc_ig__igindiode_nohinj_vgsat = assign34820_e31558;
        var_fn423_calc_ig__igindiode_nohinj_vgsat_dn4 = assign34820_e31558_d_n4;

        let (assign34830_e31573, assign34830_e31573_d_n4, assign34830_e31573_d_n8, assign34830_e31573_d_n17,) = {
    if ((var_guard417 != 0.0) && (var_guard424 == 0.0)) {
        let assign34830_e31567: f64 = (var_fn423_calc_ig__kbdgatein * var_fn423_calc_ig__iginbd);
        let assign34830_e31568: f64 = (var_fn423_calc_ig__expifor - assign34830_e31567);
        let assign34830_e31570: f64 = (assign34830_e31568 - var_fn423_calc_ig__t0);
        let assign34830_e31571: f64 = (var_fn423_calc_ig__isdiodeout * assign34830_e31570);
        (assign34830_e31571, ((var_fn423_calc_ig__isdiodeout_dn4 * assign34830_e31570) + (var_fn423_calc_ig__isdiodeout * ((var_fn423_calc_ig__expifor_dn4 - (var_fn423_calc_ig__kbdgatein * var_fn423_calc_ig__iginbd_dn4)) - var_fn423_calc_ig__t0_dn4))), (var_fn423_calc_ig__isdiodeout * (var_fn423_calc_ig__expifor_dn8 - (var_fn423_calc_ig__kbdgatein * var_fn423_calc_ig__iginbd_dn8))), (var_fn423_calc_ig__isdiodeout * (var_fn423_calc_ig__expifor_dn17 - (var_fn423_calc_ig__kbdgatein * var_fn423_calc_ig__iginbd_dn17))),)
    } else {
        (var_fn423_calc_ig__igindiode_nohinj, var_fn423_calc_ig__igindiode_nohinj_dn4, var_fn423_calc_ig__igindiode_nohinj_dn8, var_fn423_calc_ig__igindiode_nohinj_dn17,)
    }
};
        var_fn423_calc_ig__igindiode_nohinj = assign34830_e31573;
        var_fn423_calc_ig__igindiode_nohinj_dn4 = assign34830_e31573_d_n4;
        var_fn423_calc_ig__igindiode_nohinj_dn8 = assign34830_e31573_d_n8;
        var_fn423_calc_ig__igindiode_nohinj_dn17 = assign34830_e31573_d_n17;

        let assign34840_e31576: f64 = if var_fn423_calc_ig__fracin > 0.0 { 1.0 } else { 0.0 };
        var_guard425 = assign34840_e31576;

        let (assign34850_e31587,) = {
    if (((var_guard417 != 0.0) && (var_guard424 == 0.0)) && (var_guard425 != 0.0)) {
        let assign34850_e31585: f64 = (var_fn423_calc_ig__fracin * var_fn423_calc_ig__pg_paramin);
        (assign34850_e31585,)
    } else {
        (var_fn423_calc_ig__pg_paramin_hinj,)
    }
};
        var_fn423_calc_ig__pg_paramin_hinj = assign34850_e31587;

        let (assign34860_e31602, assign34860_e31602_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard424 == 0.0)) && (var_guard425 != 0.0)) {
        let assign34860_e31596: f64 = (var_fn423_calc_ig__pg_paramin_hinj / var_fn423_calc_ig__phitin);
        let assign34860_e31598: f64 = (assign34860_e31596 * var_fn423_calc_ig__vgsatin);
        let assign34860_e31600: f64 = (assign34860_e31598 + var_fn423_calc_ig__expphib);
        (assign34860_e31600, (((-((var_fn423_calc_ig__pg_paramin_hinj * var_fn423_calc_ig__phitin_dn4) / (var_fn423_calc_ig__phitin * var_fn423_calc_ig__phitin))) * var_fn423_calc_ig__vgsatin) + var_fn423_calc_ig__expphib_dn4),)
    } else {
        (var_fn423_calc_ig__expiforarg_hinj_vgsat, var_fn423_calc_ig__expiforarg_hinj_vgsat_dn4,)
    }
};
        var_fn423_calc_ig__expiforarg_hinj_vgsat = assign34860_e31602;
        var_fn423_calc_ig__expiforarg_hinj_vgsat_dn4 = assign34860_e31602_d_n4;

        let (assign34870_e31649, assign34870_e31649_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard424 == 0.0)) && (var_guard425 != 0.0)) {
        let assign34870_e31615: f64 = (-50.0);
        let (assign34870_e31647, assign34870_e31647_d_n4,) = {
            if ((!(var_fn423_calc_ig__expiforarg_hinj_vgsat > 50.0)) && (!(var_fn423_calc_ig__expiforarg_hinj_vgsat < assign34870_e31615))) {
                let assign34870_e31620: f64 = (var_fn423_calc_ig__expiforarg_hinj_vgsat).exp();
                (assign34870_e31620, (assign34870_e31620 * var_fn423_calc_ig__expiforarg_hinj_vgsat_dn4),)
            } else {
                let assign34870_e31627: f64 = (-50.0);
                let (assign34870_e31646, assign34870_e31646_d_n4,) = {
                    if ((!(var_fn423_calc_ig__expiforarg_hinj_vgsat > 50.0)) && (var_fn423_calc_ig__expiforarg_hinj_vgsat < assign34870_e31627)) {
                        let assign34870_e31631: f64 = (-50.0);
                        let assign34870_e31632: f64 = (assign34870_e31631).exp();
                        (assign34870_e31632, 0.0,)
                    } else {
                        let (assign34870_e31645, assign34870_e31645_d_n4,) = {
                            if (var_fn423_calc_ig__expiforarg_hinj_vgsat > 50.0) {
                                let assign34870_e31637: f64 = (50.0_f64).exp();
                                let assign34870_e31641: f64 = (var_fn423_calc_ig__expiforarg_hinj_vgsat - 50.0);
                                let assign34870_e31642: f64 = (1.0 + assign34870_e31641);
                                let assign34870_e31643: f64 = (assign34870_e31637 * assign34870_e31642);
                                (assign34870_e31643, (assign34870_e31637 * var_fn423_calc_ig__expiforarg_hinj_vgsat_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign34870_e31645, assign34870_e31645_d_n4,)
                    }
                };
                (assign34870_e31646, assign34870_e31646_d_n4,)
            }
        };
        (assign34870_e31647, assign34870_e31647_d_n4,)
    } else {
        (var_fn423_calc_ig__expifor_hinj_vgsat, var_fn423_calc_ig__expifor_hinj_vgsat_dn4,)
    }
};
        var_fn423_calc_ig__expifor_hinj_vgsat = assign34870_e31649;
        var_fn423_calc_ig__expifor_hinj_vgsat_dn4 = assign34870_e31649_d_n4;

        let (assign34880_e31664, assign34880_e31664_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard424 == 0.0)) && (var_guard425 != 0.0)) {
        let assign34880_e31659: f64 = (var_fn423_calc_ig__kbdgatein * var_fn423_calc_ig__iginbd_vgsat);
        let assign34880_e31660: f64 = (var_fn423_calc_ig__expifor_hinj_vgsat - assign34880_e31659);
        let assign34880_e31662: f64 = (assign34880_e31660 - var_fn423_calc_ig__t0);
        (assign34880_e31662, ((var_fn423_calc_ig__expifor_hinj_vgsat_dn4 - (var_fn423_calc_ig__kbdgatein * var_fn423_calc_ig__iginbd_vgsat_dn4)) - var_fn423_calc_ig__t0_dn4),)
    } else {
        (var_fn423_calc_ig__igindiode_hinj_vgsat, var_fn423_calc_ig__igindiode_hinj_vgsat_dn4,)
    }
};
        var_fn423_calc_ig__igindiode_hinj_vgsat = assign34880_e31664;
        var_fn423_calc_ig__igindiode_hinj_vgsat_dn4 = assign34880_e31664_d_n4;

        let (assign34890_e31679, assign34890_e31679_d_n4, assign34890_e31679_d_n8, assign34890_e31679_d_n17,) = {
    if (((var_guard417 != 0.0) && (var_guard424 == 0.0)) && (var_guard425 != 0.0)) {
        let assign34890_e31673: f64 = (var_fn423_calc_ig__pg_paramin_hinj / var_fn423_calc_ig__phitin);
        let assign34890_e31675: f64 = (assign34890_e31673 * var_fn423_calc_ig__vgin);
        let assign34890_e31677: f64 = (assign34890_e31675 + var_fn423_calc_ig__expphib);
        (assign34890_e31677, (((-((var_fn423_calc_ig__pg_paramin_hinj * var_fn423_calc_ig__phitin_dn4) / (var_fn423_calc_ig__phitin * var_fn423_calc_ig__phitin))) * var_fn423_calc_ig__vgin) + var_fn423_calc_ig__expphib_dn4), (assign34890_e31673 * var_fn423_calc_ig__vgin_dn8), (assign34890_e31673 * var_fn423_calc_ig__vgin_dn17),)
    } else {
        (var_fn423_calc_ig__expiforarg_hinj, var_fn423_calc_ig__expiforarg_hinj_dn4, var_fn423_calc_ig__expiforarg_hinj_dn8, var_fn423_calc_ig__expiforarg_hinj_dn17,)
    }
};
        var_fn423_calc_ig__expiforarg_hinj = assign34890_e31679;
        var_fn423_calc_ig__expiforarg_hinj_dn4 = assign34890_e31679_d_n4;
        var_fn423_calc_ig__expiforarg_hinj_dn8 = assign34890_e31679_d_n8;
        var_fn423_calc_ig__expiforarg_hinj_dn17 = assign34890_e31679_d_n17;

        let (assign34900_e31726, assign34900_e31726_d_n4, assign34900_e31726_d_n8, assign34900_e31726_d_n17,) = {
    if (((var_guard417 != 0.0) && (var_guard424 == 0.0)) && (var_guard425 != 0.0)) {
        let assign34900_e31692: f64 = (-50.0);
        let (assign34900_e31724, assign34900_e31724_d_n4, assign34900_e31724_d_n8, assign34900_e31724_d_n17,) = {
            if ((!(var_fn423_calc_ig__expiforarg_hinj > 50.0)) && (!(var_fn423_calc_ig__expiforarg_hinj < assign34900_e31692))) {
                let assign34900_e31697: f64 = (var_fn423_calc_ig__expiforarg_hinj).exp();
                (assign34900_e31697, (assign34900_e31697 * var_fn423_calc_ig__expiforarg_hinj_dn4), (assign34900_e31697 * var_fn423_calc_ig__expiforarg_hinj_dn8), (assign34900_e31697 * var_fn423_calc_ig__expiforarg_hinj_dn17),)
            } else {
                let assign34900_e31704: f64 = (-50.0);
                let (assign34900_e31723, assign34900_e31723_d_n4, assign34900_e31723_d_n8, assign34900_e31723_d_n17,) = {
                    if ((!(var_fn423_calc_ig__expiforarg_hinj > 50.0)) && (var_fn423_calc_ig__expiforarg_hinj < assign34900_e31704)) {
                        let assign34900_e31708: f64 = (-50.0);
                        let assign34900_e31709: f64 = (assign34900_e31708).exp();
                        (assign34900_e31709, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign34900_e31722, assign34900_e31722_d_n4, assign34900_e31722_d_n8, assign34900_e31722_d_n17,) = {
                            if (var_fn423_calc_ig__expiforarg_hinj > 50.0) {
                                let assign34900_e31714: f64 = (50.0_f64).exp();
                                let assign34900_e31718: f64 = (var_fn423_calc_ig__expiforarg_hinj - 50.0);
                                let assign34900_e31719: f64 = (1.0 + assign34900_e31718);
                                let assign34900_e31720: f64 = (assign34900_e31714 * assign34900_e31719);
                                (assign34900_e31720, (assign34900_e31714 * var_fn423_calc_ig__expiforarg_hinj_dn4), (assign34900_e31714 * var_fn423_calc_ig__expiforarg_hinj_dn8), (assign34900_e31714 * var_fn423_calc_ig__expiforarg_hinj_dn17),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign34900_e31722, assign34900_e31722_d_n4, assign34900_e31722_d_n8, assign34900_e31722_d_n17,)
                    }
                };
                (assign34900_e31723, assign34900_e31723_d_n4, assign34900_e31723_d_n8, assign34900_e31723_d_n17,)
            }
        };
        (assign34900_e31724, assign34900_e31724_d_n4, assign34900_e31724_d_n8, assign34900_e31724_d_n17,)
    } else {
        (var_fn423_calc_ig__expifor_hinj, var_fn423_calc_ig__expifor_hinj_dn4, var_fn423_calc_ig__expifor_hinj_dn8, var_fn423_calc_ig__expifor_hinj_dn17,)
    }
};
        var_fn423_calc_ig__expifor_hinj = assign34900_e31726;
        var_fn423_calc_ig__expifor_hinj_dn4 = assign34900_e31726_d_n4;
        var_fn423_calc_ig__expifor_hinj_dn8 = assign34900_e31726_d_n8;
        var_fn423_calc_ig__expifor_hinj_dn17 = assign34900_e31726_d_n17;

        let (assign34910_e31739, assign34910_e31739_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard424 == 0.0)) && (var_guard425 != 0.0)) {
        let assign34910_e31735: f64 = (var_fn423_calc_ig__isdiodeout * var_fn423_calc_ig__igindiode_nohinj_vgsat);
        let assign34910_e31737: f64 = (assign34910_e31735 / var_fn423_calc_ig__igindiode_hinj_vgsat);
        (assign34910_e31737, (((((var_fn423_calc_ig__isdiodeout_dn4 * var_fn423_calc_ig__igindiode_nohinj_vgsat) + (var_fn423_calc_ig__isdiodeout * var_fn423_calc_ig__igindiode_nohinj_vgsat_dn4)) * var_fn423_calc_ig__igindiode_hinj_vgsat) - (assign34910_e31735 * var_fn423_calc_ig__igindiode_hinj_vgsat_dn4)) / (var_fn423_calc_ig__igindiode_hinj_vgsat * var_fn423_calc_ig__igindiode_hinj_vgsat)),)
    } else {
        (var_fn423_calc_ig__igindiode_hinj_pre, var_fn423_calc_ig__igindiode_hinj_pre_dn4,)
    }
};
        var_fn423_calc_ig__igindiode_hinj_pre = assign34910_e31739;
        var_fn423_calc_ig__igindiode_hinj_pre_dn4 = assign34910_e31739_d_n4;

        let (assign34920_e31756, assign34920_e31756_d_n4, assign34920_e31756_d_n8, assign34920_e31756_d_n17,) = {
    if (((var_guard417 != 0.0) && (var_guard424 == 0.0)) && (var_guard425 != 0.0)) {
        let assign34920_e31750: f64 = (var_fn423_calc_ig__kbdgatein * var_fn423_calc_ig__iginbd);
        let assign34920_e31751: f64 = (var_fn423_calc_ig__expifor_hinj - assign34920_e31750);
        let assign34920_e31753: f64 = (assign34920_e31751 - var_fn423_calc_ig__t0);
        let assign34920_e31754: f64 = (var_fn423_calc_ig__igindiode_hinj_pre * assign34920_e31753);
        (assign34920_e31754, ((var_fn423_calc_ig__igindiode_hinj_pre_dn4 * assign34920_e31753) + (var_fn423_calc_ig__igindiode_hinj_pre * ((var_fn423_calc_ig__expifor_hinj_dn4 - (var_fn423_calc_ig__kbdgatein * var_fn423_calc_ig__iginbd_dn4)) - var_fn423_calc_ig__t0_dn4))), (var_fn423_calc_ig__igindiode_hinj_pre * (var_fn423_calc_ig__expifor_hinj_dn8 - (var_fn423_calc_ig__kbdgatein * var_fn423_calc_ig__iginbd_dn8))), (var_fn423_calc_ig__igindiode_hinj_pre * (var_fn423_calc_ig__expifor_hinj_dn17 - (var_fn423_calc_ig__kbdgatein * var_fn423_calc_ig__iginbd_dn17))),)
    } else {
        (var_fn423_calc_ig__igindiode_hinj, var_fn423_calc_ig__igindiode_hinj_dn4, var_fn423_calc_ig__igindiode_hinj_dn8, var_fn423_calc_ig__igindiode_hinj_dn17,)
    }
};
        var_fn423_calc_ig__igindiode_hinj = assign34920_e31756;
        var_fn423_calc_ig__igindiode_hinj_dn4 = assign34920_e31756_d_n4;
        var_fn423_calc_ig__igindiode_hinj_dn8 = assign34920_e31756_d_n8;
        var_fn423_calc_ig__igindiode_hinj_dn17 = assign34920_e31756_d_n17;

        let (assign34930_e31768, assign34930_e31768_d_n4, assign34930_e31768_d_n8, assign34930_e31768_d_n17,) = {
    if (((var_guard417 != 0.0) && (var_guard424 == 0.0)) && (var_guard425 == 0.0)) {
        let assign34930_e31766: f64 = (var_fn423_calc_ig__isdiodeout * var_fn423_calc_ig__igindiode_nohinj_vgsat);
        (assign34930_e31766, ((var_fn423_calc_ig__isdiodeout_dn4 * var_fn423_calc_ig__igindiode_nohinj_vgsat) + (var_fn423_calc_ig__isdiodeout * var_fn423_calc_ig__igindiode_nohinj_vgsat_dn4)), 0.0, 0.0,)
    } else {
        (var_fn423_calc_ig__igindiode_hinj, var_fn423_calc_ig__igindiode_hinj_dn4, var_fn423_calc_ig__igindiode_hinj_dn8, var_fn423_calc_ig__igindiode_hinj_dn17,)
    }
};
        var_fn423_calc_ig__igindiode_hinj = assign34930_e31768;
        var_fn423_calc_ig__igindiode_hinj_dn4 = assign34930_e31768_d_n4;
        var_fn423_calc_ig__igindiode_hinj_dn8 = assign34930_e31768_d_n8;
        var_fn423_calc_ig__igindiode_hinj_dn17 = assign34930_e31768_d_n17;

        *var_fn423_calc_ig__expbd1_slot = var_fn423_calc_ig__expbd1;
        *var_fn423_calc_ig__expbd1_dn17_slot = var_fn423_calc_ig__expbd1_dn17;
        *var_fn423_calc_ig__expbd1_dn4_slot = var_fn423_calc_ig__expbd1_dn4;
        *var_fn423_calc_ig__expbd1_dn8_slot = var_fn423_calc_ig__expbd1_dn8;
        *var_fn423_calc_ig__expbd1_vgsat_slot = var_fn423_calc_ig__expbd1_vgsat;
        *var_fn423_calc_ig__expbd1_vgsat_dn4_slot = var_fn423_calc_ig__expbd1_vgsat_dn4;
        *var_fn423_calc_ig__expbd2_slot = var_fn423_calc_ig__expbd2;
        *var_fn423_calc_ig__expbd2_dn4_slot = var_fn423_calc_ig__expbd2_dn4;
        *var_fn423_calc_ig__expbdarg1_vgsat_slot = var_fn423_calc_ig__expbdarg1_vgsat;
        *var_fn423_calc_ig__expbdarg1_vgsat_dn4_slot = var_fn423_calc_ig__expbdarg1_vgsat_dn4;
        *var_fn423_calc_ig__expifor_slot = var_fn423_calc_ig__expifor;
        *var_fn423_calc_ig__expifor_dn17_slot = var_fn423_calc_ig__expifor_dn17;
        *var_fn423_calc_ig__expifor_dn4_slot = var_fn423_calc_ig__expifor_dn4;
        *var_fn423_calc_ig__expifor_dn8_slot = var_fn423_calc_ig__expifor_dn8;
        *var_fn423_calc_ig__expifor_hinj_slot = var_fn423_calc_ig__expifor_hinj;
        *var_fn423_calc_ig__expifor_hinj_dn17_slot = var_fn423_calc_ig__expifor_hinj_dn17;
        *var_fn423_calc_ig__expifor_hinj_dn4_slot = var_fn423_calc_ig__expifor_hinj_dn4;
        *var_fn423_calc_ig__expifor_hinj_dn8_slot = var_fn423_calc_ig__expifor_hinj_dn8;
        *var_fn423_calc_ig__expifor_hinj_vgsat_slot = var_fn423_calc_ig__expifor_hinj_vgsat;
        *var_fn423_calc_ig__expifor_hinj_vgsat_dn4_slot = var_fn423_calc_ig__expifor_hinj_vgsat_dn4;
        *var_fn423_calc_ig__expifor_nohinj_vgsat_slot = var_fn423_calc_ig__expifor_nohinj_vgsat;
        *var_fn423_calc_ig__expifor_nohinj_vgsat_dn4_slot = var_fn423_calc_ig__expifor_nohinj_vgsat_dn4;
        *var_fn423_calc_ig__expiforarg_slot = var_fn423_calc_ig__expiforarg;
        *var_fn423_calc_ig__expiforarg_dn17_slot = var_fn423_calc_ig__expiforarg_dn17;
        *var_fn423_calc_ig__expiforarg_dn4_slot = var_fn423_calc_ig__expiforarg_dn4;
        *var_fn423_calc_ig__expiforarg_dn8_slot = var_fn423_calc_ig__expiforarg_dn8;
        *var_fn423_calc_ig__expiforarg_hinj_slot = var_fn423_calc_ig__expiforarg_hinj;
        *var_fn423_calc_ig__expiforarg_hinj_dn17_slot = var_fn423_calc_ig__expiforarg_hinj_dn17;
        *var_fn423_calc_ig__expiforarg_hinj_dn4_slot = var_fn423_calc_ig__expiforarg_hinj_dn4;
        *var_fn423_calc_ig__expiforarg_hinj_dn8_slot = var_fn423_calc_ig__expiforarg_hinj_dn8;
        *var_fn423_calc_ig__expiforarg_hinj_vgsat_slot = var_fn423_calc_ig__expiforarg_hinj_vgsat;
        *var_fn423_calc_ig__expiforarg_hinj_vgsat_dn4_slot = var_fn423_calc_ig__expiforarg_hinj_vgsat_dn4;
        *var_fn423_calc_ig__expiforarg_nohinj_vgsat_slot = var_fn423_calc_ig__expiforarg_nohinj_vgsat;
        *var_fn423_calc_ig__expiforarg_nohinj_vgsat_dn4_slot = var_fn423_calc_ig__expiforarg_nohinj_vgsat_dn4;
        *var_fn423_calc_ig__iginbd_slot = var_fn423_calc_ig__iginbd;
        *var_fn423_calc_ig__iginbd_dn17_slot = var_fn423_calc_ig__iginbd_dn17;
        *var_fn423_calc_ig__iginbd_dn4_slot = var_fn423_calc_ig__iginbd_dn4;
        *var_fn423_calc_ig__iginbd_dn8_slot = var_fn423_calc_ig__iginbd_dn8;
        *var_fn423_calc_ig__iginbd_vgsat_slot = var_fn423_calc_ig__iginbd_vgsat;
        *var_fn423_calc_ig__iginbd_vgsat_dn4_slot = var_fn423_calc_ig__iginbd_vgsat_dn4;
        *var_fn423_calc_ig__igindiode_slot = var_fn423_calc_ig__igindiode;
        *var_fn423_calc_ig__igindiode_dn17_slot = var_fn423_calc_ig__igindiode_dn17;
        *var_fn423_calc_ig__igindiode_dn4_slot = var_fn423_calc_ig__igindiode_dn4;
        *var_fn423_calc_ig__igindiode_dn8_slot = var_fn423_calc_ig__igindiode_dn8;
        *var_fn423_calc_ig__igindiode_hinj_slot = var_fn423_calc_ig__igindiode_hinj;
        *var_fn423_calc_ig__igindiode_hinj_dn17_slot = var_fn423_calc_ig__igindiode_hinj_dn17;
        *var_fn423_calc_ig__igindiode_hinj_dn4_slot = var_fn423_calc_ig__igindiode_hinj_dn4;
        *var_fn423_calc_ig__igindiode_hinj_dn8_slot = var_fn423_calc_ig__igindiode_hinj_dn8;
        *var_fn423_calc_ig__igindiode_hinj_pre_slot = var_fn423_calc_ig__igindiode_hinj_pre;
        *var_fn423_calc_ig__igindiode_hinj_pre_dn4_slot = var_fn423_calc_ig__igindiode_hinj_pre_dn4;
        *var_fn423_calc_ig__igindiode_hinj_vgsat_slot = var_fn423_calc_ig__igindiode_hinj_vgsat;
        *var_fn423_calc_ig__igindiode_hinj_vgsat_dn4_slot = var_fn423_calc_ig__igindiode_hinj_vgsat_dn4;
        *var_fn423_calc_ig__igindiode_nohinj_slot = var_fn423_calc_ig__igindiode_nohinj;
        *var_fn423_calc_ig__igindiode_nohinj_dn17_slot = var_fn423_calc_ig__igindiode_nohinj_dn17;
        *var_fn423_calc_ig__igindiode_nohinj_dn4_slot = var_fn423_calc_ig__igindiode_nohinj_dn4;
        *var_fn423_calc_ig__igindiode_nohinj_dn8_slot = var_fn423_calc_ig__igindiode_nohinj_dn8;
        *var_fn423_calc_ig__igindiode_nohinj_vgsat_slot = var_fn423_calc_ig__igindiode_nohinj_vgsat;
        *var_fn423_calc_ig__igindiode_nohinj_vgsat_dn4_slot = var_fn423_calc_ig__igindiode_nohinj_vgsat_dn4;
        *var_fn423_calc_ig__isdiodeout_slot = var_fn423_calc_ig__isdiodeout;
        *var_fn423_calc_ig__isdiodeout_dn4_slot = var_fn423_calc_ig__isdiodeout_dn4;
        *var_fn423_calc_ig__pg_paramin_hinj_slot = var_fn423_calc_ig__pg_paramin_hinj;
        *var_guard424_slot = var_guard424;
        *var_guard425_slot = var_guard425;
    }

    pub(super) fn stamp_transient_block_86(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_fn423_calc_ig__alphagin: f64,
        var_fn423_calc_ig__betarecin: f64,
        var_fn423_calc_ig__igindiode_hinj: f64,
        var_fn423_calc_ig__igindiode_hinj_dn17: f64,
        var_fn423_calc_ig__igindiode_hinj_dn4: f64,
        var_fn423_calc_ig__igindiode_hinj_dn8: f64,
        var_fn423_calc_ig__igindiode_nohinj: f64,
        var_fn423_calc_ig__igindiode_nohinj_dn17: f64,
        var_fn423_calc_ig__igindiode_nohinj_dn4: f64,
        var_fn423_calc_ig__igindiode_nohinj_dn8: f64,
        var_fn423_calc_ig__irecin: f64,
        var_fn423_calc_ig__ngf: f64,
        var_fn423_calc_ig__pgsrecin: f64,
        var_fn423_calc_ig__phitin: f64,
        var_fn423_calc_ig__phitin_dn4: f64,
        var_fn423_calc_ig__tfacdiodein: f64,
        var_fn423_calc_ig__tfacdiodein_dn4: f64,
        var_fn423_calc_ig__type: f64,
        var_fn423_calc_ig__vgin: f64,
        var_fn423_calc_ig__vgin_dn17: f64,
        var_fn423_calc_ig__vgin_dn8: f64,
        var_fn423_calc_ig__vgsatin: f64,
        var_fn423_calc_ig__vgsatqin: f64,
        var_fn423_calc_ig__w: f64,
        var_guard417: f64,
        var_guard424: f64,
        var_phit: f64,
        var_phit_dn4: f64,
        var_tfacdiode: f64,
        var_tfacdiode_dn4: f64,
        var_fn423_calc_ig__alpha2_phit_slot: &mut f64,
        var_fn423_calc_ig__alpha2_phit_dn4_slot: &mut f64,
        var_fn423_calc_ig__expffvarg_slot: &mut f64,
        var_fn423_calc_ig__expffvarg_dn17_slot: &mut f64,
        var_fn423_calc_ig__expffvarg_dn4_slot: &mut f64,
        var_fn423_calc_ig__expffvarg_dn8_slot: &mut f64,
        var_fn423_calc_ig__expirev_slot: &mut f64,
        var_fn423_calc_ig__expirev_dn17_slot: &mut f64,
        var_fn423_calc_ig__expirev_dn4_slot: &mut f64,
        var_fn423_calc_ig__expirev_dn8_slot: &mut f64,
        var_fn423_calc_ig__expirevarg_slot: &mut f64,
        var_fn423_calc_ig__expirevarg_dn17_slot: &mut f64,
        var_fn423_calc_ig__expirevarg_dn4_slot: &mut f64,
        var_fn423_calc_ig__expirevarg_dn8_slot: &mut f64,
        var_fn423_calc_ig__ffvgin_slot: &mut f64,
        var_fn423_calc_ig__ffvgin_dn17_slot: &mut f64,
        var_fn423_calc_ig__ffvgin_dn4_slot: &mut f64,
        var_fn423_calc_ig__ffvgin_dn8_slot: &mut f64,
        var_fn423_calc_ig__frecgin_slot: &mut f64,
        var_fn423_calc_ig__frecgin_dn17_slot: &mut f64,
        var_fn423_calc_ig__frecgin_dn8_slot: &mut f64,
        var_fn423_calc_ig__igindiode_slot: &mut f64,
        var_fn423_calc_ig__igindiode_dn17_slot: &mut f64,
        var_fn423_calc_ig__igindiode_dn4_slot: &mut f64,
        var_fn423_calc_ig__igindiode_dn8_slot: &mut f64,
        var_fn423_calc_ig__iginrec_slot: &mut f64,
        var_fn423_calc_ig__iginrec_dn17_slot: &mut f64,
        var_fn423_calc_ig__iginrec_dn4_slot: &mut f64,
        var_fn423_calc_ig__iginrec_dn8_slot: &mut f64,
        var_fn423_calc_ig__igout_slot: &mut f64,
        var_fn423_calc_ig__igout_dn17_slot: &mut f64,
        var_fn423_calc_ig__igout_dn4_slot: &mut f64,
        var_fn423_calc_ig__igout_dn8_slot: &mut f64,
        var_fn423_calc_ig__isrecout_slot: &mut f64,
        var_fn423_calc_ig__isrecout_dn4_slot: &mut f64,
        var_fn423_calc_ig__return_slot: &mut f64,
        var_fn423_calc_ig__return_dn17_slot: &mut f64,
        var_fn423_calc_ig__return_dn4_slot: &mut f64,
        var_fn423_calc_ig__return_dn8_slot: &mut f64,
        var_fn429_calc_ig__alpha2_phit_slot: &mut f64,
        var_fn429_calc_ig__alpha2_phit_dn4_slot: &mut f64,
        var_fn429_calc_ig__alphagin_slot: &mut f64,
        var_fn429_calc_ig__betarecin_slot: &mut f64,
        var_fn429_calc_ig__fracin_slot: &mut f64,
        var_fn429_calc_ig__igout_slot: &mut f64,
        var_fn429_calc_ig__igout_dn13_slot: &mut f64,
        var_fn429_calc_ig__igout_dn4_slot: &mut f64,
        var_fn429_calc_ig__igout_dn8_slot: &mut f64,
        var_fn429_calc_ig__ijin_slot: &mut f64,
        var_fn429_calc_ig__irecin_slot: &mut f64,
        var_fn429_calc_ig__isdiodeout_slot: &mut f64,
        var_fn429_calc_ig__isdiodeout_dn4_slot: &mut f64,
        var_fn429_calc_ig__isrecout_slot: &mut f64,
        var_fn429_calc_ig__isrecout_dn4_slot: &mut f64,
        var_fn429_calc_ig__kbdgatein_slot: &mut f64,
        var_fn429_calc_ig__ngf_slot: &mut f64,
        var_fn429_calc_ig__pbdgin_slot: &mut f64,
        var_fn429_calc_ig__pg_param1_slot: &mut f64,
        var_fn429_calc_ig__pg_paramin_slot: &mut f64,
        var_fn429_calc_ig__pgsrecin_slot: &mut f64,
        var_fn429_calc_ig__phitin_slot: &mut f64,
        var_fn429_calc_ig__phitin_dn4_slot: &mut f64,
        var_fn429_calc_ig__return_slot: &mut f64,
        var_fn429_calc_ig__return_dn13_slot: &mut f64,
        var_fn429_calc_ig__return_dn4_slot: &mut f64,
        var_fn429_calc_ig__return_dn8_slot: &mut f64,
        var_fn429_calc_ig__t0_slot: &mut f64,
        var_fn429_calc_ig__t0_dn4_slot: &mut f64,
        var_fn429_calc_ig__tfacdiodein_slot: &mut f64,
        var_fn429_calc_ig__tfacdiodein_dn4_slot: &mut f64,
        var_fn429_calc_ig__type_slot: &mut f64,
        var_fn429_calc_ig__vbdgin_slot: &mut f64,
        var_fn429_calc_ig__vgin_slot: &mut f64,
        var_fn429_calc_ig__vgin_dn13_slot: &mut f64,
        var_fn429_calc_ig__vgin_dn8_slot: &mut f64,
        var_fn429_calc_ig__vgsatin_slot: &mut f64,
        var_fn429_calc_ig__vgsatqin_slot: &mut f64,
        var_fn429_calc_ig__vjg_slot: &mut f64,
        var_fn429_calc_ig__w_slot: &mut f64,
        var_guard426_slot: &mut f64,
        var_guard427_slot: &mut f64,
        var_guard428_slot: &mut f64,
        var_igdi_slot: &mut f64,
        var_igdi_dn17_slot: &mut f64,
        var_igdi_dn4_slot: &mut f64,
        var_igdi_dn8_slot: &mut f64,
    ) {
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let mut var_fn423_calc_ig__alpha2_phit: f64 = *var_fn423_calc_ig__alpha2_phit_slot;
        let mut var_fn423_calc_ig__alpha2_phit_dn4: f64 = *var_fn423_calc_ig__alpha2_phit_dn4_slot;
        let mut var_fn423_calc_ig__expffvarg: f64 = *var_fn423_calc_ig__expffvarg_slot;
        let mut var_fn423_calc_ig__expffvarg_dn17: f64 = *var_fn423_calc_ig__expffvarg_dn17_slot;
        let mut var_fn423_calc_ig__expffvarg_dn4: f64 = *var_fn423_calc_ig__expffvarg_dn4_slot;
        let mut var_fn423_calc_ig__expffvarg_dn8: f64 = *var_fn423_calc_ig__expffvarg_dn8_slot;
        let mut var_fn423_calc_ig__expirev: f64 = *var_fn423_calc_ig__expirev_slot;
        let mut var_fn423_calc_ig__expirev_dn17: f64 = *var_fn423_calc_ig__expirev_dn17_slot;
        let mut var_fn423_calc_ig__expirev_dn4: f64 = *var_fn423_calc_ig__expirev_dn4_slot;
        let mut var_fn423_calc_ig__expirev_dn8: f64 = *var_fn423_calc_ig__expirev_dn8_slot;
        let mut var_fn423_calc_ig__expirevarg: f64 = *var_fn423_calc_ig__expirevarg_slot;
        let mut var_fn423_calc_ig__expirevarg_dn17: f64 = *var_fn423_calc_ig__expirevarg_dn17_slot;
        let mut var_fn423_calc_ig__expirevarg_dn4: f64 = *var_fn423_calc_ig__expirevarg_dn4_slot;
        let mut var_fn423_calc_ig__expirevarg_dn8: f64 = *var_fn423_calc_ig__expirevarg_dn8_slot;
        let mut var_fn423_calc_ig__ffvgin: f64 = *var_fn423_calc_ig__ffvgin_slot;
        let mut var_fn423_calc_ig__ffvgin_dn17: f64 = *var_fn423_calc_ig__ffvgin_dn17_slot;
        let mut var_fn423_calc_ig__ffvgin_dn4: f64 = *var_fn423_calc_ig__ffvgin_dn4_slot;
        let mut var_fn423_calc_ig__ffvgin_dn8: f64 = *var_fn423_calc_ig__ffvgin_dn8_slot;
        let mut var_fn423_calc_ig__frecgin: f64 = *var_fn423_calc_ig__frecgin_slot;
        let mut var_fn423_calc_ig__frecgin_dn17: f64 = *var_fn423_calc_ig__frecgin_dn17_slot;
        let mut var_fn423_calc_ig__frecgin_dn8: f64 = *var_fn423_calc_ig__frecgin_dn8_slot;
        let mut var_fn423_calc_ig__igindiode: f64 = *var_fn423_calc_ig__igindiode_slot;
        let mut var_fn423_calc_ig__igindiode_dn17: f64 = *var_fn423_calc_ig__igindiode_dn17_slot;
        let mut var_fn423_calc_ig__igindiode_dn4: f64 = *var_fn423_calc_ig__igindiode_dn4_slot;
        let mut var_fn423_calc_ig__igindiode_dn8: f64 = *var_fn423_calc_ig__igindiode_dn8_slot;
        let mut var_fn423_calc_ig__iginrec: f64 = *var_fn423_calc_ig__iginrec_slot;
        let mut var_fn423_calc_ig__iginrec_dn17: f64 = *var_fn423_calc_ig__iginrec_dn17_slot;
        let mut var_fn423_calc_ig__iginrec_dn4: f64 = *var_fn423_calc_ig__iginrec_dn4_slot;
        let mut var_fn423_calc_ig__iginrec_dn8: f64 = *var_fn423_calc_ig__iginrec_dn8_slot;
        let mut var_fn423_calc_ig__igout: f64 = *var_fn423_calc_ig__igout_slot;
        let mut var_fn423_calc_ig__igout_dn17: f64 = *var_fn423_calc_ig__igout_dn17_slot;
        let mut var_fn423_calc_ig__igout_dn4: f64 = *var_fn423_calc_ig__igout_dn4_slot;
        let mut var_fn423_calc_ig__igout_dn8: f64 = *var_fn423_calc_ig__igout_dn8_slot;
        let mut var_fn423_calc_ig__isrecout: f64 = *var_fn423_calc_ig__isrecout_slot;
        let mut var_fn423_calc_ig__isrecout_dn4: f64 = *var_fn423_calc_ig__isrecout_dn4_slot;
        let mut var_fn423_calc_ig__return: f64 = *var_fn423_calc_ig__return_slot;
        let mut var_fn423_calc_ig__return_dn17: f64 = *var_fn423_calc_ig__return_dn17_slot;
        let mut var_fn423_calc_ig__return_dn4: f64 = *var_fn423_calc_ig__return_dn4_slot;
        let mut var_fn423_calc_ig__return_dn8: f64 = *var_fn423_calc_ig__return_dn8_slot;
        let mut var_fn429_calc_ig__alpha2_phit: f64 = *var_fn429_calc_ig__alpha2_phit_slot;
        let mut var_fn429_calc_ig__alpha2_phit_dn4: f64 = *var_fn429_calc_ig__alpha2_phit_dn4_slot;
        let mut var_fn429_calc_ig__alphagin: f64 = *var_fn429_calc_ig__alphagin_slot;
        let mut var_fn429_calc_ig__betarecin: f64 = *var_fn429_calc_ig__betarecin_slot;
        let mut var_fn429_calc_ig__fracin: f64 = *var_fn429_calc_ig__fracin_slot;
        let mut var_fn429_calc_ig__igout: f64 = *var_fn429_calc_ig__igout_slot;
        let mut var_fn429_calc_ig__igout_dn13: f64 = *var_fn429_calc_ig__igout_dn13_slot;
        let mut var_fn429_calc_ig__igout_dn4: f64 = *var_fn429_calc_ig__igout_dn4_slot;
        let mut var_fn429_calc_ig__igout_dn8: f64 = *var_fn429_calc_ig__igout_dn8_slot;
        let mut var_fn429_calc_ig__ijin: f64 = *var_fn429_calc_ig__ijin_slot;
        let mut var_fn429_calc_ig__irecin: f64 = *var_fn429_calc_ig__irecin_slot;
        let mut var_fn429_calc_ig__isdiodeout: f64 = *var_fn429_calc_ig__isdiodeout_slot;
        let mut var_fn429_calc_ig__isdiodeout_dn4: f64 = *var_fn429_calc_ig__isdiodeout_dn4_slot;
        let mut var_fn429_calc_ig__isrecout: f64 = *var_fn429_calc_ig__isrecout_slot;
        let mut var_fn429_calc_ig__isrecout_dn4: f64 = *var_fn429_calc_ig__isrecout_dn4_slot;
        let mut var_fn429_calc_ig__kbdgatein: f64 = *var_fn429_calc_ig__kbdgatein_slot;
        let mut var_fn429_calc_ig__ngf: f64 = *var_fn429_calc_ig__ngf_slot;
        let mut var_fn429_calc_ig__pbdgin: f64 = *var_fn429_calc_ig__pbdgin_slot;
        let mut var_fn429_calc_ig__pg_param1: f64 = *var_fn429_calc_ig__pg_param1_slot;
        let mut var_fn429_calc_ig__pg_paramin: f64 = *var_fn429_calc_ig__pg_paramin_slot;
        let mut var_fn429_calc_ig__pgsrecin: f64 = *var_fn429_calc_ig__pgsrecin_slot;
        let mut var_fn429_calc_ig__phitin: f64 = *var_fn429_calc_ig__phitin_slot;
        let mut var_fn429_calc_ig__phitin_dn4: f64 = *var_fn429_calc_ig__phitin_dn4_slot;
        let mut var_fn429_calc_ig__return: f64 = *var_fn429_calc_ig__return_slot;
        let mut var_fn429_calc_ig__return_dn13: f64 = *var_fn429_calc_ig__return_dn13_slot;
        let mut var_fn429_calc_ig__return_dn4: f64 = *var_fn429_calc_ig__return_dn4_slot;
        let mut var_fn429_calc_ig__return_dn8: f64 = *var_fn429_calc_ig__return_dn8_slot;
        let mut var_fn429_calc_ig__t0: f64 = *var_fn429_calc_ig__t0_slot;
        let mut var_fn429_calc_ig__t0_dn4: f64 = *var_fn429_calc_ig__t0_dn4_slot;
        let mut var_fn429_calc_ig__tfacdiodein: f64 = *var_fn429_calc_ig__tfacdiodein_slot;
        let mut var_fn429_calc_ig__tfacdiodein_dn4: f64 = *var_fn429_calc_ig__tfacdiodein_dn4_slot;
        let mut var_fn429_calc_ig__type: f64 = *var_fn429_calc_ig__type_slot;
        let mut var_fn429_calc_ig__vbdgin: f64 = *var_fn429_calc_ig__vbdgin_slot;
        let mut var_fn429_calc_ig__vgin: f64 = *var_fn429_calc_ig__vgin_slot;
        let mut var_fn429_calc_ig__vgin_dn13: f64 = *var_fn429_calc_ig__vgin_dn13_slot;
        let mut var_fn429_calc_ig__vgin_dn8: f64 = *var_fn429_calc_ig__vgin_dn8_slot;
        let mut var_fn429_calc_ig__vgsatin: f64 = *var_fn429_calc_ig__vgsatin_slot;
        let mut var_fn429_calc_ig__vgsatqin: f64 = *var_fn429_calc_ig__vgsatqin_slot;
        let mut var_fn429_calc_ig__vjg: f64 = *var_fn429_calc_ig__vjg_slot;
        let mut var_fn429_calc_ig__w: f64 = *var_fn429_calc_ig__w_slot;
        let mut var_guard426: f64 = *var_guard426_slot;
        let mut var_guard427: f64 = *var_guard427_slot;
        let mut var_guard428: f64 = *var_guard428_slot;
        let mut var_igdi: f64 = *var_igdi_slot;
        let mut var_igdi_dn17: f64 = *var_igdi_dn17_slot;
        let mut var_igdi_dn4: f64 = *var_igdi_dn4_slot;
        let mut var_igdi_dn8: f64 = *var_igdi_dn8_slot;

        let (assign34940_e31779, assign34940_e31779_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard424 == 0.0)) {
        let assign34940_e31775: f64 = (var_fn423_calc_ig__alphagin * var_fn423_calc_ig__alphagin);
        let assign34940_e31777: f64 = (assign34940_e31775 * var_fn423_calc_ig__phitin);
        (assign34940_e31777, (assign34940_e31775 * var_fn423_calc_ig__phitin_dn4),)
    } else {
        (var_fn423_calc_ig__alpha2_phit, var_fn423_calc_ig__alpha2_phit_dn4,)
    }
};
        var_fn423_calc_ig__alpha2_phit = assign34940_e31779;
        var_fn423_calc_ig__alpha2_phit_dn4 = assign34940_e31779_d_n4;

        let (assign34950_e31794, assign34950_e31794_d_n4, assign34950_e31794_d_n8, assign34950_e31794_d_n17,) = {
    if ((var_guard417 != 0.0) && (var_guard424 == 0.0)) {
        let assign34950_e31788: f64 = (var_fn423_calc_ig__alpha2_phit / 2.0);
        let assign34950_e31789: f64 = (var_fn423_calc_ig__vgsatin - assign34950_e31788);
        let assign34950_e31790: f64 = (var_fn423_calc_ig__vgin - assign34950_e31789);
        let assign34950_e31792: f64 = (assign34950_e31790 / var_fn423_calc_ig__alpha2_phit);
        (assign34950_e31792, ((((-(-(var_fn423_calc_ig__alpha2_phit_dn4 / 2.0))) * var_fn423_calc_ig__alpha2_phit) - (assign34950_e31790 * var_fn423_calc_ig__alpha2_phit_dn4)) / (var_fn423_calc_ig__alpha2_phit * var_fn423_calc_ig__alpha2_phit)), (var_fn423_calc_ig__vgin_dn8 / var_fn423_calc_ig__alpha2_phit), (var_fn423_calc_ig__vgin_dn17 / var_fn423_calc_ig__alpha2_phit),)
    } else {
        (var_fn423_calc_ig__expffvarg, var_fn423_calc_ig__expffvarg_dn4, var_fn423_calc_ig__expffvarg_dn8, var_fn423_calc_ig__expffvarg_dn17,)
    }
};
        var_fn423_calc_ig__expffvarg = assign34950_e31794;
        var_fn423_calc_ig__expffvarg_dn4 = assign34950_e31794_d_n4;
        var_fn423_calc_ig__expffvarg_dn8 = assign34950_e31794_d_n8;
        var_fn423_calc_ig__expffvarg_dn17 = assign34950_e31794_d_n17;

        let assign34960_e31797: f64 = if var_fn423_calc_ig__expffvarg > 50.0 { 1.0 } else { 0.0 };
        var_guard426 = assign34960_e31797;

        let (assign34970_e31806, assign34970_e31806_d_n4, assign34970_e31806_d_n8, assign34970_e31806_d_n17,) = {
    if (((var_guard417 != 0.0) && (var_guard424 == 0.0)) && (var_guard426 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn423_calc_ig__ffvgin, var_fn423_calc_ig__ffvgin_dn4, var_fn423_calc_ig__ffvgin_dn8, var_fn423_calc_ig__ffvgin_dn17,)
    }
};
        var_fn423_calc_ig__ffvgin = assign34970_e31806;
        var_fn423_calc_ig__ffvgin_dn4 = assign34970_e31806_d_n4;
        var_fn423_calc_ig__ffvgin_dn8 = assign34970_e31806_d_n8;
        var_fn423_calc_ig__ffvgin_dn17 = assign34970_e31806_d_n17;

        let assign34980_e31809: f64 = (-50.0);
        let assign34980_e31810: f64 = if var_fn423_calc_ig__expffvarg < assign34980_e31809 { 1.0 } else { 0.0 };
        var_guard427 = assign34980_e31810;

        let (assign34990_e31822, assign34990_e31822_d_n4, assign34990_e31822_d_n8, assign34990_e31822_d_n17,) = {
    if ((((var_guard417 != 0.0) && (var_guard424 == 0.0)) && (var_guard426 == 0.0)) && (var_guard427 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn423_calc_ig__ffvgin, var_fn423_calc_ig__ffvgin_dn4, var_fn423_calc_ig__ffvgin_dn8, var_fn423_calc_ig__ffvgin_dn17,)
    }
};
        var_fn423_calc_ig__ffvgin = assign34990_e31822;
        var_fn423_calc_ig__ffvgin_dn4 = assign34990_e31822_d_n4;
        var_fn423_calc_ig__ffvgin_dn8 = assign34990_e31822_d_n8;
        var_fn423_calc_ig__ffvgin_dn17 = assign34990_e31822_d_n17;

        let (assign35000_e31840, assign35000_e31840_d_n4, assign35000_e31840_d_n8, assign35000_e31840_d_n17,) = {
    if ((((var_guard417 != 0.0) && (var_guard424 == 0.0)) && (var_guard426 == 0.0)) && (var_guard427 == 0.0)) {
        let assign35000_e31836: f64 = (var_fn423_calc_ig__expffvarg).exp();
        let assign35000_e31837: f64 = (1.0 + assign35000_e31836);
        let assign35000_e31838: f64 = (1.0 / assign35000_e31837);
        (assign35000_e31838, (-((assign35000_e31836 * var_fn423_calc_ig__expffvarg_dn4) / (assign35000_e31837 * assign35000_e31837))), (-((assign35000_e31836 * var_fn423_calc_ig__expffvarg_dn8) / (assign35000_e31837 * assign35000_e31837))), (-((assign35000_e31836 * var_fn423_calc_ig__expffvarg_dn17) / (assign35000_e31837 * assign35000_e31837))),)
    } else {
        (var_fn423_calc_ig__ffvgin, var_fn423_calc_ig__ffvgin_dn4, var_fn423_calc_ig__ffvgin_dn8, var_fn423_calc_ig__ffvgin_dn17,)
    }
};
        var_fn423_calc_ig__ffvgin = assign35000_e31840;
        var_fn423_calc_ig__ffvgin_dn4 = assign35000_e31840_d_n4;
        var_fn423_calc_ig__ffvgin_dn8 = assign35000_e31840_d_n8;
        var_fn423_calc_ig__ffvgin_dn17 = assign35000_e31840_d_n17;

        let (assign35010_e31855, assign35010_e31855_d_n4, assign35010_e31855_d_n8, assign35010_e31855_d_n17,) = {
    if ((var_guard417 != 0.0) && (var_guard424 == 0.0)) {
        let assign35010_e31847: f64 = (var_fn423_calc_ig__ffvgin * var_fn423_calc_ig__igindiode_nohinj);
        let assign35010_e31850: f64 = (1.0 - var_fn423_calc_ig__ffvgin);
        let assign35010_e31852: f64 = (assign35010_e31850 * var_fn423_calc_ig__igindiode_hinj);
        let assign35010_e31853: f64 = (assign35010_e31847 + assign35010_e31852);
        (assign35010_e31853, (((var_fn423_calc_ig__ffvgin_dn4 * var_fn423_calc_ig__igindiode_nohinj) + (var_fn423_calc_ig__ffvgin * var_fn423_calc_ig__igindiode_nohinj_dn4)) + (((-var_fn423_calc_ig__ffvgin_dn4) * var_fn423_calc_ig__igindiode_hinj) + (assign35010_e31850 * var_fn423_calc_ig__igindiode_hinj_dn4))), (((var_fn423_calc_ig__ffvgin_dn8 * var_fn423_calc_ig__igindiode_nohinj) + (var_fn423_calc_ig__ffvgin * var_fn423_calc_ig__igindiode_nohinj_dn8)) + (((-var_fn423_calc_ig__ffvgin_dn8) * var_fn423_calc_ig__igindiode_hinj) + (assign35010_e31850 * var_fn423_calc_ig__igindiode_hinj_dn8))), (((var_fn423_calc_ig__ffvgin_dn17 * var_fn423_calc_ig__igindiode_nohinj) + (var_fn423_calc_ig__ffvgin * var_fn423_calc_ig__igindiode_nohinj_dn17)) + (((-var_fn423_calc_ig__ffvgin_dn17) * var_fn423_calc_ig__igindiode_hinj) + (assign35010_e31850 * var_fn423_calc_ig__igindiode_hinj_dn17))),)
    } else {
        (var_fn423_calc_ig__igindiode, var_fn423_calc_ig__igindiode_dn4, var_fn423_calc_ig__igindiode_dn8, var_fn423_calc_ig__igindiode_dn17,)
    }
};
        var_fn423_calc_ig__igindiode = assign35010_e31855;
        var_fn423_calc_ig__igindiode_dn4 = assign35010_e31855_d_n4;
        var_fn423_calc_ig__igindiode_dn8 = assign35010_e31855_d_n8;
        var_fn423_calc_ig__igindiode_dn17 = assign35010_e31855_d_n17;

        let (assign35020_e31901, assign35020_e31901_d_n8, assign35020_e31901_d_n17,) = {
    if (var_guard417 != 0.0) {
        let assign35020_e31858: f64 = (-var_fn423_calc_ig__vgin);
        let (assign35020_e31891, assign35020_e31891_d_n8, assign35020_e31891_d_n17,) = {
            if (p.p52 != 0.0) {
                let assign35020_e31866: f64 = (var_fn423_calc_ig__vgin / var_fn423_calc_ig__vgsatqin);
                let assign35020_e31869: f64 = (0.001 / p.p53);
                let assign35020_e31872: f64 = (var_fn423_calc_ig__vgin / var_fn423_calc_ig__vgsatqin);
                let assign35020_e31873: f64 = (assign35020_e31869 * assign35020_e31872);
                let assign35020_e31874: f64 = (assign35020_e31873).tanh();
                let assign35020_e31875: f64 = (assign35020_e31866 * assign35020_e31874);
                (assign35020_e31875, (((var_fn423_calc_ig__vgin_dn8 / var_fn423_calc_ig__vgsatqin) * assign35020_e31874) + (assign35020_e31866 * ((assign35020_e31869 * (var_fn423_calc_ig__vgin_dn8 / var_fn423_calc_ig__vgsatqin)) / ((assign35020_e31873).cosh() * (assign35020_e31873).cosh())))), (((var_fn423_calc_ig__vgin_dn17 / var_fn423_calc_ig__vgsatqin) * assign35020_e31874) + (assign35020_e31866 * ((assign35020_e31869 * (var_fn423_calc_ig__vgin_dn17 / var_fn423_calc_ig__vgsatqin)) / ((assign35020_e31873).cosh() * (assign35020_e31873).cosh())))),)
            } else {
                let (assign35020_e31890, assign35020_e31890_d_n8, assign35020_e31890_d_n17,) = {
                    if (p.p52 == 0.0) {
                        let __rspice_inv_cse_0: f64 = 1.0 / var_fn423_calc_ig__vgsatqin;
                        let assign35020_e31881: f64 = (var_fn423_calc_ig__vgin * __rspice_inv_cse_0);
                        let assign35020_e31884: f64 = (var_fn423_calc_ig__vgin * __rspice_inv_cse_0);
                        let assign35020_e31885: f64 = (assign35020_e31881 * assign35020_e31884);
                        let assign35020_e31887: f64 = (assign35020_e31885 + p.p53);
                        let assign35020_e31888: f64 = (assign35020_e31887).sqrt();
                        (assign35020_e31888, ((((var_fn423_calc_ig__vgin_dn8 / var_fn423_calc_ig__vgsatqin) * assign35020_e31884) + (assign35020_e31881 * (var_fn423_calc_ig__vgin_dn8 / var_fn423_calc_ig__vgsatqin))) / (2.0 * assign35020_e31888)), ((((var_fn423_calc_ig__vgin_dn17 / var_fn423_calc_ig__vgsatqin) * assign35020_e31884) + (assign35020_e31881 * (var_fn423_calc_ig__vgin_dn17 / var_fn423_calc_ig__vgsatqin))) / (2.0 * assign35020_e31888)),)
                    } else {
                        (0.0, 0.0, 0.0,)
                    }
                };
                (assign35020_e31890, assign35020_e31890_d_n8, assign35020_e31890_d_n17,)
            }
        };
        let assign35020_e31893: f64 = (assign35020_e31891).powf(var_fn423_calc_ig__betarecin);
        let assign35020_e31894: f64 = (1.0 + assign35020_e31893);
        let assign35020_e31897: f64 = (1.0 / var_fn423_calc_ig__betarecin);
        let assign35020_e31898: f64 = (assign35020_e31894).powf(assign35020_e31897);
        let assign35020_e31899: f64 = (assign35020_e31858 / assign35020_e31898);
        (assign35020_e31899, ((((-var_fn423_calc_ig__vgin_dn8) * assign35020_e31898) - (assign35020_e31858 * if 0.0 == 0.0 && ((assign35020_e31897) as f64).is_finite() && ((assign35020_e31897) as f64).fract() == 0.0 { if assign35020_e31897 == 0.0 { 0.0 } else { (assign35020_e31897 * ((assign35020_e31894).powf(assign35020_e31897 - 1.0) * if 0.0 == 0.0 && ((var_fn423_calc_ig__betarecin) as f64).is_finite() && ((var_fn423_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn423_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn423_calc_ig__betarecin * ((assign35020_e31891).powf(var_fn423_calc_ig__betarecin - 1.0) * assign35020_e31891_d_n8)) } } else { (assign35020_e31893 * (var_fn423_calc_ig__betarecin * (assign35020_e31891_d_n8 / assign35020_e31891))) })) } } else { (assign35020_e31898 * (assign35020_e31897 * (if 0.0 == 0.0 && ((var_fn423_calc_ig__betarecin) as f64).is_finite() && ((var_fn423_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn423_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn423_calc_ig__betarecin * ((assign35020_e31891).powf(var_fn423_calc_ig__betarecin - 1.0) * assign35020_e31891_d_n8)) } } else { (assign35020_e31893 * (var_fn423_calc_ig__betarecin * (assign35020_e31891_d_n8 / assign35020_e31891))) } / assign35020_e31894))) })) / (assign35020_e31898 * assign35020_e31898)), ((((-var_fn423_calc_ig__vgin_dn17) * assign35020_e31898) - (assign35020_e31858 * if 0.0 == 0.0 && ((assign35020_e31897) as f64).is_finite() && ((assign35020_e31897) as f64).fract() == 0.0 { if assign35020_e31897 == 0.0 { 0.0 } else { (assign35020_e31897 * ((assign35020_e31894).powf(assign35020_e31897 - 1.0) * if 0.0 == 0.0 && ((var_fn423_calc_ig__betarecin) as f64).is_finite() && ((var_fn423_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn423_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn423_calc_ig__betarecin * ((assign35020_e31891).powf(var_fn423_calc_ig__betarecin - 1.0) * assign35020_e31891_d_n17)) } } else { (assign35020_e31893 * (var_fn423_calc_ig__betarecin * (assign35020_e31891_d_n17 / assign35020_e31891))) })) } } else { (assign35020_e31898 * (assign35020_e31897 * (if 0.0 == 0.0 && ((var_fn423_calc_ig__betarecin) as f64).is_finite() && ((var_fn423_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn423_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn423_calc_ig__betarecin * ((assign35020_e31891).powf(var_fn423_calc_ig__betarecin - 1.0) * assign35020_e31891_d_n17)) } } else { (assign35020_e31893 * (var_fn423_calc_ig__betarecin * (assign35020_e31891_d_n17 / assign35020_e31891))) } / assign35020_e31894))) })) / (assign35020_e31898 * assign35020_e31898)),)
    } else {
        (var_fn423_calc_ig__frecgin, var_fn423_calc_ig__frecgin_dn8, var_fn423_calc_ig__frecgin_dn17,)
    }
};
        var_fn423_calc_ig__frecgin = assign35020_e31901;
        var_fn423_calc_ig__frecgin_dn8 = assign35020_e31901_d_n8;
        var_fn423_calc_ig__frecgin_dn17 = assign35020_e31901_d_n17;

        let (assign35030_e31916, assign35030_e31916_d_n4,) = {
    if (var_guard417 != 0.0) {
        let assign35030_e31904: f64 = (-var_fn423_calc_ig__type);
        let assign35030_e31906: f64 = (assign35030_e31904 * var_fn423_calc_ig__w);
        let assign35030_e31908: f64 = (assign35030_e31906 * var_fn423_calc_ig__ngf);
        let assign35030_e31910: f64 = (assign35030_e31908 * var_fn423_calc_ig__irecin);
        let assign35030_e31912: f64 = (assign35030_e31910 * var_fn423_calc_ig__tfacdiodein);
        let assign35030_e31914: f64 = assign35030_e31912;
        (assign35030_e31914, (assign35030_e31910 * var_fn423_calc_ig__tfacdiodein_dn4),)
    } else {
        (var_fn423_calc_ig__isrecout, var_fn423_calc_ig__isrecout_dn4,)
    }
};
        var_fn423_calc_ig__isrecout = assign35030_e31916;
        var_fn423_calc_ig__isrecout_dn4 = assign35030_e31916_d_n4;

        let (assign35040_e31924, assign35040_e31924_d_n4, assign35040_e31924_d_n8, assign35040_e31924_d_n17,) = {
    if (var_guard417 != 0.0) {
        let assign35040_e31920: f64 = (var_fn423_calc_ig__pgsrecin / var_fn423_calc_ig__phitin);
        let assign35040_e31922: f64 = (assign35040_e31920 * var_fn423_calc_ig__frecgin);
        (assign35040_e31922, ((-((var_fn423_calc_ig__pgsrecin * var_fn423_calc_ig__phitin_dn4) / (var_fn423_calc_ig__phitin * var_fn423_calc_ig__phitin))) * var_fn423_calc_ig__frecgin), (assign35040_e31920 * var_fn423_calc_ig__frecgin_dn8), (assign35040_e31920 * var_fn423_calc_ig__frecgin_dn17),)
    } else {
        (var_fn423_calc_ig__expirevarg, var_fn423_calc_ig__expirevarg_dn4, var_fn423_calc_ig__expirevarg_dn8, var_fn423_calc_ig__expirevarg_dn17,)
    }
};
        var_fn423_calc_ig__expirevarg = assign35040_e31924;
        var_fn423_calc_ig__expirevarg_dn4 = assign35040_e31924_d_n4;
        var_fn423_calc_ig__expirevarg_dn8 = assign35040_e31924_d_n8;
        var_fn423_calc_ig__expirevarg_dn17 = assign35040_e31924_d_n17;

        let (assign35050_e31966, assign35050_e31966_d_n4, assign35050_e31966_d_n8, assign35050_e31966_d_n17,) = {
    if (var_guard417 != 0.0) {
        let assign35050_e31932: f64 = (-50.0);
        let (assign35050_e31964, assign35050_e31964_d_n4, assign35050_e31964_d_n8, assign35050_e31964_d_n17,) = {
            if ((!(var_fn423_calc_ig__expirevarg > 50.0)) && (!(var_fn423_calc_ig__expirevarg < assign35050_e31932))) {
                let assign35050_e31937: f64 = (var_fn423_calc_ig__expirevarg).exp();
                (assign35050_e31937, (assign35050_e31937 * var_fn423_calc_ig__expirevarg_dn4), (assign35050_e31937 * var_fn423_calc_ig__expirevarg_dn8), (assign35050_e31937 * var_fn423_calc_ig__expirevarg_dn17),)
            } else {
                let assign35050_e31944: f64 = (-50.0);
                let (assign35050_e31963, assign35050_e31963_d_n4, assign35050_e31963_d_n8, assign35050_e31963_d_n17,) = {
                    if ((!(var_fn423_calc_ig__expirevarg > 50.0)) && (var_fn423_calc_ig__expirevarg < assign35050_e31944)) {
                        let assign35050_e31948: f64 = (-50.0);
                        let assign35050_e31949: f64 = (assign35050_e31948).exp();
                        (assign35050_e31949, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign35050_e31962, assign35050_e31962_d_n4, assign35050_e31962_d_n8, assign35050_e31962_d_n17,) = {
                            if (var_fn423_calc_ig__expirevarg > 50.0) {
                                let assign35050_e31954: f64 = (50.0_f64).exp();
                                let assign35050_e31958: f64 = (var_fn423_calc_ig__expirevarg - 50.0);
                                let assign35050_e31959: f64 = (1.0 + assign35050_e31958);
                                let assign35050_e31960: f64 = (assign35050_e31954 * assign35050_e31959);
                                (assign35050_e31960, (assign35050_e31954 * var_fn423_calc_ig__expirevarg_dn4), (assign35050_e31954 * var_fn423_calc_ig__expirevarg_dn8), (assign35050_e31954 * var_fn423_calc_ig__expirevarg_dn17),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign35050_e31962, assign35050_e31962_d_n4, assign35050_e31962_d_n8, assign35050_e31962_d_n17,)
                    }
                };
                (assign35050_e31963, assign35050_e31963_d_n4, assign35050_e31963_d_n8, assign35050_e31963_d_n17,)
            }
        };
        (assign35050_e31964, assign35050_e31964_d_n4, assign35050_e31964_d_n8, assign35050_e31964_d_n17,)
    } else {
        (var_fn423_calc_ig__expirev, var_fn423_calc_ig__expirev_dn4, var_fn423_calc_ig__expirev_dn8, var_fn423_calc_ig__expirev_dn17,)
    }
};
        var_fn423_calc_ig__expirev = assign35050_e31966;
        var_fn423_calc_ig__expirev_dn4 = assign35050_e31966_d_n4;
        var_fn423_calc_ig__expirev_dn8 = assign35050_e31966_d_n8;
        var_fn423_calc_ig__expirev_dn17 = assign35050_e31966_d_n17;

        let (assign35060_e31974, assign35060_e31974_d_n4, assign35060_e31974_d_n8, assign35060_e31974_d_n17,) = {
    if (var_guard417 != 0.0) {
        let assign35060_e31971: f64 = (var_fn423_calc_ig__expirev - 1.0);
        let assign35060_e31972: f64 = (var_fn423_calc_ig__isrecout * assign35060_e31971);
        (assign35060_e31972, ((var_fn423_calc_ig__isrecout_dn4 * assign35060_e31971) + (var_fn423_calc_ig__isrecout * var_fn423_calc_ig__expirev_dn4)), (var_fn423_calc_ig__isrecout * var_fn423_calc_ig__expirev_dn8), (var_fn423_calc_ig__isrecout * var_fn423_calc_ig__expirev_dn17),)
    } else {
        (var_fn423_calc_ig__iginrec, var_fn423_calc_ig__iginrec_dn4, var_fn423_calc_ig__iginrec_dn8, var_fn423_calc_ig__iginrec_dn17,)
    }
};
        var_fn423_calc_ig__iginrec = assign35060_e31974;
        var_fn423_calc_ig__iginrec_dn4 = assign35060_e31974_d_n4;
        var_fn423_calc_ig__iginrec_dn8 = assign35060_e31974_d_n8;
        var_fn423_calc_ig__iginrec_dn17 = assign35060_e31974_d_n17;

        let (assign35070_e31980, assign35070_e31980_d_n4, assign35070_e31980_d_n8, assign35070_e31980_d_n17,) = {
    if (var_guard417 != 0.0) {
        let assign35070_e31978: f64 = (var_fn423_calc_ig__igindiode + var_fn423_calc_ig__iginrec);
        (assign35070_e31978, (var_fn423_calc_ig__igindiode_dn4 + var_fn423_calc_ig__iginrec_dn4), (var_fn423_calc_ig__igindiode_dn8 + var_fn423_calc_ig__iginrec_dn8), (var_fn423_calc_ig__igindiode_dn17 + var_fn423_calc_ig__iginrec_dn17),)
    } else {
        (var_fn423_calc_ig__igout, var_fn423_calc_ig__igout_dn4, var_fn423_calc_ig__igout_dn8, var_fn423_calc_ig__igout_dn17,)
    }
};
        var_fn423_calc_ig__igout = assign35070_e31980;
        var_fn423_calc_ig__igout_dn4 = assign35070_e31980_d_n4;
        var_fn423_calc_ig__igout_dn8 = assign35070_e31980_d_n8;
        var_fn423_calc_ig__igout_dn17 = assign35070_e31980_d_n17;

        let (assign35080_e31984, assign35080_e31984_d_n4, assign35080_e31984_d_n8, assign35080_e31984_d_n17,) = {
    if (var_guard417 != 0.0) {
        (var_fn423_calc_ig__igout, var_fn423_calc_ig__igout_dn4, var_fn423_calc_ig__igout_dn8, var_fn423_calc_ig__igout_dn17,)
    } else {
        (var_fn423_calc_ig__return, var_fn423_calc_ig__return_dn4, var_fn423_calc_ig__return_dn8, var_fn423_calc_ig__return_dn17,)
    }
};
        var_fn423_calc_ig__return = assign35080_e31984;
        var_fn423_calc_ig__return_dn4 = assign35080_e31984_d_n4;
        var_fn423_calc_ig__return_dn8 = assign35080_e31984_d_n8;
        var_fn423_calc_ig__return_dn17 = assign35080_e31984_d_n17;

        let (assign35110_e31996, assign35110_e31996_d_n4, assign35110_e31996_d_n8, assign35110_e31996_d_n17,) = {
    if (var_guard417 != 0.0) {
        (var_fn423_calc_ig__return, var_fn423_calc_ig__return_dn4, var_fn423_calc_ig__return_dn8, var_fn423_calc_ig__return_dn17,)
    } else {
        (var_igdi, var_igdi_dn4, var_igdi_dn8, var_igdi_dn17,)
    }
};
        var_igdi = assign35110_e31996;
        var_igdi_dn4 = assign35110_e31996_d_n4;
        var_igdi_dn8 = assign35110_e31996_d_n8;
        var_igdi_dn17 = assign35110_e31996_d_n17;

        let assign35120_e31999: f64 = if p.p282 == 1.0 { 1.0 } else { 0.0 };
        var_guard428 = assign35120_e31999;

        let (assign35130_e32005, assign35130_e32005_d_n4, assign35130_e32005_d_n8, assign35130_e32005_d_n13,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn429_calc_ig__return, var_fn429_calc_ig__return_dn4, var_fn429_calc_ig__return_dn8, var_fn429_calc_ig__return_dn13,)
    }
};
        var_fn429_calc_ig__return = assign35130_e32005;
        var_fn429_calc_ig__return_dn4 = assign35130_e32005_d_n4;
        var_fn429_calc_ig__return_dn8 = assign35130_e32005_d_n8;
        var_fn429_calc_ig__return_dn13 = assign35130_e32005_d_n13;

        let (assign35140_e32011, assign35140_e32011_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn429_calc_ig__isdiodeout, var_fn429_calc_ig__isdiodeout_dn4,)
    }
};
        var_fn429_calc_ig__isdiodeout = assign35140_e32011;
        var_fn429_calc_ig__isdiodeout_dn4 = assign35140_e32011_d_n4;

        let (assign35150_e32017, assign35150_e32017_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn429_calc_ig__isrecout, var_fn429_calc_ig__isrecout_dn4,)
    }
};
        var_fn429_calc_ig__isrecout = assign35150_e32017;
        var_fn429_calc_ig__isrecout_dn4 = assign35150_e32017_d_n4;

        let (assign35160_e32025, assign35160_e32025_d_n8, assign35160_e32025_d_n13,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        let assign35160_e32023: f64 = (p.p6 * (nv8 - nv13));
        (assign35160_e32023, p.p6, (-p.p6),)
    } else {
        (var_fn429_calc_ig__vgin, var_fn429_calc_ig__vgin_dn8, var_fn429_calc_ig__vgin_dn13,)
    }
};
        var_fn429_calc_ig__vgin = assign35160_e32025;
        var_fn429_calc_ig__vgin_dn8 = assign35160_e32025_d_n8;
        var_fn429_calc_ig__vgin_dn13 = assign35160_e32025_d_n13;

        let (assign35170_e32031, assign35170_e32031_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (var_phit, var_phit_dn4,)
    } else {
        (var_fn429_calc_ig__phitin, var_fn429_calc_ig__phitin_dn4,)
    }
};
        var_fn429_calc_ig__phitin = assign35170_e32031;
        var_fn429_calc_ig__phitin_dn4 = assign35170_e32031_d_n4;

        let (assign35180_e32037,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (p.p260,)
    } else {
        (var_fn429_calc_ig__vgsatin,)
    }
};
        var_fn429_calc_ig__vgsatin = assign35180_e32037;

        let (assign35190_e32043,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (p.p262,)
    } else {
        (var_fn429_calc_ig__alphagin,)
    }
};
        var_fn429_calc_ig__alphagin = assign35190_e32043;

        let (assign35200_e32049,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (1.0,)
    } else {
        (var_fn429_calc_ig__fracin,)
    }
};
        var_fn429_calc_ig__fracin = assign35200_e32049;

        let (assign35210_e32055,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (p.p258,)
    } else {
        (var_fn429_calc_ig__pg_paramin,)
    }
};
        var_fn429_calc_ig__pg_paramin = assign35210_e32055;

        let (assign35220_e32061,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (p.p278,)
    } else {
        (var_fn429_calc_ig__pbdgin,)
    }
};
        var_fn429_calc_ig__pbdgin = assign35220_e32061;

        let (assign35230_e32067,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (p.p277,)
    } else {
        (var_fn429_calc_ig__vbdgin,)
    }
};
        var_fn429_calc_ig__vbdgin = assign35230_e32067;

        let (assign35240_e32073, assign35240_e32073_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (var_tfacdiode, var_tfacdiode_dn4,)
    } else {
        (var_fn429_calc_ig__tfacdiodein, var_fn429_calc_ig__tfacdiodein_dn4,)
    }
};
        var_fn429_calc_ig__tfacdiodein = assign35240_e32073;
        var_fn429_calc_ig__tfacdiodein_dn4 = assign35240_e32073_d_n4;

        let (assign35250_e32079,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (p.p0,)
    } else {
        (var_fn429_calc_ig__w,)
    }
};
        var_fn429_calc_ig__w = assign35250_e32079;

        let (assign35260_e32085,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (p.p2,)
    } else {
        (var_fn429_calc_ig__ngf,)
    }
};
        var_fn429_calc_ig__ngf = assign35260_e32085;

        let (assign35270_e32091,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0,)
    } else {
        (var_fn429_calc_ig__ijin,)
    }
};
        var_fn429_calc_ig__ijin = assign35270_e32091;

        let (assign35280_e32097,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0,)
    } else {
        (var_fn429_calc_ig__kbdgatein,)
    }
};
        var_fn429_calc_ig__kbdgatein = assign35280_e32097;

        let (assign35290_e32103,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (p.p285,)
    } else {
        (var_fn429_calc_ig__vgsatqin,)
    }
};
        var_fn429_calc_ig__vgsatqin = assign35290_e32103;

        let (assign35300_e32109,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (p.p286,)
    } else {
        (var_fn429_calc_ig__betarecin,)
    }
};
        var_fn429_calc_ig__betarecin = assign35300_e32109;

        let (assign35310_e32119,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        let assign35310_e32115: f64 = (1.0 - p.p255);
        let assign35310_e32117: f64 = (assign35310_e32115 * p.p284);
        (assign35310_e32117,)
    } else {
        (var_fn429_calc_ig__irecin,)
    }
};
        var_fn429_calc_ig__irecin = assign35310_e32119;

        let (assign35320_e32125,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (p.p283,)
    } else {
        (var_fn429_calc_ig__pgsrecin,)
    }
};
        var_fn429_calc_ig__pgsrecin = assign35320_e32125;

        let (assign35330_e32131,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (p.p257,)
    } else {
        (var_fn429_calc_ig__pg_param1,)
    }
};
        var_fn429_calc_ig__pg_param1 = assign35330_e32131;

        let (assign35340_e32137,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (p.p256,)
    } else {
        (var_fn429_calc_ig__vjg,)
    }
};
        var_fn429_calc_ig__vjg = assign35340_e32137;

        let (assign35350_e32143,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (p.p6,)
    } else {
        (var_fn429_calc_ig__type,)
    }
};
        var_fn429_calc_ig__type = assign35350_e32143;

        let (assign35360_e32149, assign35360_e32149_d_n4, assign35360_e32149_d_n8, assign35360_e32149_d_n13,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn429_calc_ig__igout, var_fn429_calc_ig__igout_dn4, var_fn429_calc_ig__igout_dn8, var_fn429_calc_ig__igout_dn13,)
    }
};
        var_fn429_calc_ig__igout = assign35360_e32149;
        var_fn429_calc_ig__igout_dn4 = assign35360_e32149_d_n4;
        var_fn429_calc_ig__igout_dn8 = assign35360_e32149_d_n8;
        var_fn429_calc_ig__igout_dn13 = assign35360_e32149_d_n13;

        let (assign35370_e32155, assign35370_e32155_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn429_calc_ig__alpha2_phit, var_fn429_calc_ig__alpha2_phit_dn4,)
    }
};
        var_fn429_calc_ig__alpha2_phit = assign35370_e32155;
        var_fn429_calc_ig__alpha2_phit_dn4 = assign35370_e32155_d_n4;

        let (assign35380_e32161, assign35380_e32161_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn429_calc_ig__t0, var_fn429_calc_ig__t0_dn4,)
    }
};
        var_fn429_calc_ig__t0 = assign35380_e32161;
        var_fn429_calc_ig__t0_dn4 = assign35380_e32161_d_n4;

        *var_fn423_calc_ig__alpha2_phit_slot = var_fn423_calc_ig__alpha2_phit;
        *var_fn423_calc_ig__alpha2_phit_dn4_slot = var_fn423_calc_ig__alpha2_phit_dn4;
        *var_fn423_calc_ig__expffvarg_slot = var_fn423_calc_ig__expffvarg;
        *var_fn423_calc_ig__expffvarg_dn17_slot = var_fn423_calc_ig__expffvarg_dn17;
        *var_fn423_calc_ig__expffvarg_dn4_slot = var_fn423_calc_ig__expffvarg_dn4;
        *var_fn423_calc_ig__expffvarg_dn8_slot = var_fn423_calc_ig__expffvarg_dn8;
        *var_fn423_calc_ig__expirev_slot = var_fn423_calc_ig__expirev;
        *var_fn423_calc_ig__expirev_dn17_slot = var_fn423_calc_ig__expirev_dn17;
        *var_fn423_calc_ig__expirev_dn4_slot = var_fn423_calc_ig__expirev_dn4;
        *var_fn423_calc_ig__expirev_dn8_slot = var_fn423_calc_ig__expirev_dn8;
        *var_fn423_calc_ig__expirevarg_slot = var_fn423_calc_ig__expirevarg;
        *var_fn423_calc_ig__expirevarg_dn17_slot = var_fn423_calc_ig__expirevarg_dn17;
        *var_fn423_calc_ig__expirevarg_dn4_slot = var_fn423_calc_ig__expirevarg_dn4;
        *var_fn423_calc_ig__expirevarg_dn8_slot = var_fn423_calc_ig__expirevarg_dn8;
        *var_fn423_calc_ig__ffvgin_slot = var_fn423_calc_ig__ffvgin;
        *var_fn423_calc_ig__ffvgin_dn17_slot = var_fn423_calc_ig__ffvgin_dn17;
        *var_fn423_calc_ig__ffvgin_dn4_slot = var_fn423_calc_ig__ffvgin_dn4;
        *var_fn423_calc_ig__ffvgin_dn8_slot = var_fn423_calc_ig__ffvgin_dn8;
        *var_fn423_calc_ig__frecgin_slot = var_fn423_calc_ig__frecgin;
        *var_fn423_calc_ig__frecgin_dn17_slot = var_fn423_calc_ig__frecgin_dn17;
        *var_fn423_calc_ig__frecgin_dn8_slot = var_fn423_calc_ig__frecgin_dn8;
        *var_fn423_calc_ig__igindiode_slot = var_fn423_calc_ig__igindiode;
        *var_fn423_calc_ig__igindiode_dn17_slot = var_fn423_calc_ig__igindiode_dn17;
        *var_fn423_calc_ig__igindiode_dn4_slot = var_fn423_calc_ig__igindiode_dn4;
        *var_fn423_calc_ig__igindiode_dn8_slot = var_fn423_calc_ig__igindiode_dn8;
        *var_fn423_calc_ig__iginrec_slot = var_fn423_calc_ig__iginrec;
        *var_fn423_calc_ig__iginrec_dn17_slot = var_fn423_calc_ig__iginrec_dn17;
        *var_fn423_calc_ig__iginrec_dn4_slot = var_fn423_calc_ig__iginrec_dn4;
        *var_fn423_calc_ig__iginrec_dn8_slot = var_fn423_calc_ig__iginrec_dn8;
        *var_fn423_calc_ig__igout_slot = var_fn423_calc_ig__igout;
        *var_fn423_calc_ig__igout_dn17_slot = var_fn423_calc_ig__igout_dn17;
        *var_fn423_calc_ig__igout_dn4_slot = var_fn423_calc_ig__igout_dn4;
        *var_fn423_calc_ig__igout_dn8_slot = var_fn423_calc_ig__igout_dn8;
        *var_fn423_calc_ig__isrecout_slot = var_fn423_calc_ig__isrecout;
        *var_fn423_calc_ig__isrecout_dn4_slot = var_fn423_calc_ig__isrecout_dn4;
        *var_fn423_calc_ig__return_slot = var_fn423_calc_ig__return;
        *var_fn423_calc_ig__return_dn17_slot = var_fn423_calc_ig__return_dn17;
        *var_fn423_calc_ig__return_dn4_slot = var_fn423_calc_ig__return_dn4;
        *var_fn423_calc_ig__return_dn8_slot = var_fn423_calc_ig__return_dn8;
        *var_fn429_calc_ig__alpha2_phit_slot = var_fn429_calc_ig__alpha2_phit;
        *var_fn429_calc_ig__alpha2_phit_dn4_slot = var_fn429_calc_ig__alpha2_phit_dn4;
        *var_fn429_calc_ig__alphagin_slot = var_fn429_calc_ig__alphagin;
        *var_fn429_calc_ig__betarecin_slot = var_fn429_calc_ig__betarecin;
        *var_fn429_calc_ig__fracin_slot = var_fn429_calc_ig__fracin;
        *var_fn429_calc_ig__igout_slot = var_fn429_calc_ig__igout;
        *var_fn429_calc_ig__igout_dn13_slot = var_fn429_calc_ig__igout_dn13;
        *var_fn429_calc_ig__igout_dn4_slot = var_fn429_calc_ig__igout_dn4;
        *var_fn429_calc_ig__igout_dn8_slot = var_fn429_calc_ig__igout_dn8;
        *var_fn429_calc_ig__ijin_slot = var_fn429_calc_ig__ijin;
        *var_fn429_calc_ig__irecin_slot = var_fn429_calc_ig__irecin;
        *var_fn429_calc_ig__isdiodeout_slot = var_fn429_calc_ig__isdiodeout;
        *var_fn429_calc_ig__isdiodeout_dn4_slot = var_fn429_calc_ig__isdiodeout_dn4;
        *var_fn429_calc_ig__isrecout_slot = var_fn429_calc_ig__isrecout;
        *var_fn429_calc_ig__isrecout_dn4_slot = var_fn429_calc_ig__isrecout_dn4;
        *var_fn429_calc_ig__kbdgatein_slot = var_fn429_calc_ig__kbdgatein;
        *var_fn429_calc_ig__ngf_slot = var_fn429_calc_ig__ngf;
        *var_fn429_calc_ig__pbdgin_slot = var_fn429_calc_ig__pbdgin;
        *var_fn429_calc_ig__pg_param1_slot = var_fn429_calc_ig__pg_param1;
        *var_fn429_calc_ig__pg_paramin_slot = var_fn429_calc_ig__pg_paramin;
        *var_fn429_calc_ig__pgsrecin_slot = var_fn429_calc_ig__pgsrecin;
        *var_fn429_calc_ig__phitin_slot = var_fn429_calc_ig__phitin;
        *var_fn429_calc_ig__phitin_dn4_slot = var_fn429_calc_ig__phitin_dn4;
        *var_fn429_calc_ig__return_slot = var_fn429_calc_ig__return;
        *var_fn429_calc_ig__return_dn13_slot = var_fn429_calc_ig__return_dn13;
        *var_fn429_calc_ig__return_dn4_slot = var_fn429_calc_ig__return_dn4;
        *var_fn429_calc_ig__return_dn8_slot = var_fn429_calc_ig__return_dn8;
        *var_fn429_calc_ig__t0_slot = var_fn429_calc_ig__t0;
        *var_fn429_calc_ig__t0_dn4_slot = var_fn429_calc_ig__t0_dn4;
        *var_fn429_calc_ig__tfacdiodein_slot = var_fn429_calc_ig__tfacdiodein;
        *var_fn429_calc_ig__tfacdiodein_dn4_slot = var_fn429_calc_ig__tfacdiodein_dn4;
        *var_fn429_calc_ig__type_slot = var_fn429_calc_ig__type;
        *var_fn429_calc_ig__vbdgin_slot = var_fn429_calc_ig__vbdgin;
        *var_fn429_calc_ig__vgin_slot = var_fn429_calc_ig__vgin;
        *var_fn429_calc_ig__vgin_dn13_slot = var_fn429_calc_ig__vgin_dn13;
        *var_fn429_calc_ig__vgin_dn8_slot = var_fn429_calc_ig__vgin_dn8;
        *var_fn429_calc_ig__vgsatin_slot = var_fn429_calc_ig__vgsatin;
        *var_fn429_calc_ig__vgsatqin_slot = var_fn429_calc_ig__vgsatqin;
        *var_fn429_calc_ig__vjg_slot = var_fn429_calc_ig__vjg;
        *var_fn429_calc_ig__w_slot = var_fn429_calc_ig__w;
        *var_guard426_slot = var_guard426;
        *var_guard427_slot = var_guard427;
        *var_guard428_slot = var_guard428;
        *var_igdi_slot = var_igdi;
        *var_igdi_dn17_slot = var_igdi_dn17;
        *var_igdi_dn4_slot = var_igdi_dn4;
        *var_igdi_dn8_slot = var_igdi_dn8;
    }

    pub(super) fn stamp_transient_block_87(
        var_fn429_calc_ig__pbdgin: f64,
        var_fn429_calc_ig__pg_param1: f64,
        var_fn429_calc_ig__phitin: f64,
        var_fn429_calc_ig__phitin_dn4: f64,
        var_fn429_calc_ig__vbdgin: f64,
        var_fn429_calc_ig__vgin: f64,
        var_fn429_calc_ig__vgin_dn13: f64,
        var_fn429_calc_ig__vgin_dn8: f64,
        var_fn429_calc_ig__vjg: f64,
        var_guard417: f64,
        var_guard428: f64,
        var_fn429_calc_ig__expbd1_slot: &mut f64,
        var_fn429_calc_ig__expbd1_dn13_slot: &mut f64,
        var_fn429_calc_ig__expbd1_dn4_slot: &mut f64,
        var_fn429_calc_ig__expbd1_dn8_slot: &mut f64,
        var_fn429_calc_ig__expbd1_vgsat_slot: &mut f64,
        var_fn429_calc_ig__expbd1_vgsat_dn4_slot: &mut f64,
        var_fn429_calc_ig__expbd2_slot: &mut f64,
        var_fn429_calc_ig__expbd2_dn4_slot: &mut f64,
        var_fn429_calc_ig__expbdarg1_slot: &mut f64,
        var_fn429_calc_ig__expbdarg1_dn13_slot: &mut f64,
        var_fn429_calc_ig__expbdarg1_dn4_slot: &mut f64,
        var_fn429_calc_ig__expbdarg1_dn8_slot: &mut f64,
        var_fn429_calc_ig__expbdarg1_vgsat_slot: &mut f64,
        var_fn429_calc_ig__expbdarg1_vgsat_dn4_slot: &mut f64,
        var_fn429_calc_ig__expbdarg2_slot: &mut f64,
        var_fn429_calc_ig__expbdarg2_dn4_slot: &mut f64,
        var_fn429_calc_ig__expffvarg_slot: &mut f64,
        var_fn429_calc_ig__expffvarg_dn13_slot: &mut f64,
        var_fn429_calc_ig__expffvarg_dn4_slot: &mut f64,
        var_fn429_calc_ig__expffvarg_dn8_slot: &mut f64,
        var_fn429_calc_ig__expifor_slot: &mut f64,
        var_fn429_calc_ig__expifor_dn13_slot: &mut f64,
        var_fn429_calc_ig__expifor_dn4_slot: &mut f64,
        var_fn429_calc_ig__expifor_dn8_slot: &mut f64,
        var_fn429_calc_ig__expifor_hinj_slot: &mut f64,
        var_fn429_calc_ig__expifor_hinj_dn13_slot: &mut f64,
        var_fn429_calc_ig__expifor_hinj_dn4_slot: &mut f64,
        var_fn429_calc_ig__expifor_hinj_dn8_slot: &mut f64,
        var_fn429_calc_ig__expifor_hinj_vgsat_slot: &mut f64,
        var_fn429_calc_ig__expifor_hinj_vgsat_dn4_slot: &mut f64,
        var_fn429_calc_ig__expifor_nohinj_vgsat_slot: &mut f64,
        var_fn429_calc_ig__expifor_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn429_calc_ig__expiforarg_slot: &mut f64,
        var_fn429_calc_ig__expiforarg_dn13_slot: &mut f64,
        var_fn429_calc_ig__expiforarg_dn4_slot: &mut f64,
        var_fn429_calc_ig__expiforarg_dn8_slot: &mut f64,
        var_fn429_calc_ig__expiforarg_hinj_slot: &mut f64,
        var_fn429_calc_ig__expiforarg_hinj_dn13_slot: &mut f64,
        var_fn429_calc_ig__expiforarg_hinj_dn4_slot: &mut f64,
        var_fn429_calc_ig__expiforarg_hinj_dn8_slot: &mut f64,
        var_fn429_calc_ig__expiforarg_hinj_vgsat_slot: &mut f64,
        var_fn429_calc_ig__expiforarg_hinj_vgsat_dn4_slot: &mut f64,
        var_fn429_calc_ig__expiforarg_nohinj_vgsat_slot: &mut f64,
        var_fn429_calc_ig__expiforarg_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn429_calc_ig__expirev_slot: &mut f64,
        var_fn429_calc_ig__expirev_dn13_slot: &mut f64,
        var_fn429_calc_ig__expirev_dn4_slot: &mut f64,
        var_fn429_calc_ig__expirev_dn8_slot: &mut f64,
        var_fn429_calc_ig__expirevarg_slot: &mut f64,
        var_fn429_calc_ig__expirevarg_dn13_slot: &mut f64,
        var_fn429_calc_ig__expirevarg_dn4_slot: &mut f64,
        var_fn429_calc_ig__expirevarg_dn8_slot: &mut f64,
        var_fn429_calc_ig__expphib_slot: &mut f64,
        var_fn429_calc_ig__expphib_dn4_slot: &mut f64,
        var_fn429_calc_ig__ffvgin_slot: &mut f64,
        var_fn429_calc_ig__ffvgin_dn13_slot: &mut f64,
        var_fn429_calc_ig__ffvgin_dn4_slot: &mut f64,
        var_fn429_calc_ig__ffvgin_dn8_slot: &mut f64,
        var_fn429_calc_ig__frecgin_slot: &mut f64,
        var_fn429_calc_ig__frecgin_dn13_slot: &mut f64,
        var_fn429_calc_ig__frecgin_dn8_slot: &mut f64,
        var_fn429_calc_ig__iginbd_slot: &mut f64,
        var_fn429_calc_ig__iginbd_dn13_slot: &mut f64,
        var_fn429_calc_ig__iginbd_dn4_slot: &mut f64,
        var_fn429_calc_ig__iginbd_dn8_slot: &mut f64,
        var_fn429_calc_ig__iginbd_vgsat_slot: &mut f64,
        var_fn429_calc_ig__iginbd_vgsat_dn4_slot: &mut f64,
        var_fn429_calc_ig__igindiode_slot: &mut f64,
        var_fn429_calc_ig__igindiode_dn13_slot: &mut f64,
        var_fn429_calc_ig__igindiode_dn4_slot: &mut f64,
        var_fn429_calc_ig__igindiode_dn8_slot: &mut f64,
        var_fn429_calc_ig__igindiode_hinj_slot: &mut f64,
        var_fn429_calc_ig__igindiode_hinj_dn13_slot: &mut f64,
        var_fn429_calc_ig__igindiode_hinj_dn4_slot: &mut f64,
        var_fn429_calc_ig__igindiode_hinj_dn8_slot: &mut f64,
        var_fn429_calc_ig__igindiode_hinj_pre_slot: &mut f64,
        var_fn429_calc_ig__igindiode_hinj_pre_dn4_slot: &mut f64,
        var_fn429_calc_ig__igindiode_hinj_vgsat_slot: &mut f64,
        var_fn429_calc_ig__igindiode_hinj_vgsat_dn4_slot: &mut f64,
        var_fn429_calc_ig__igindiode_nohinj_slot: &mut f64,
        var_fn429_calc_ig__igindiode_nohinj_dn13_slot: &mut f64,
        var_fn429_calc_ig__igindiode_nohinj_dn4_slot: &mut f64,
        var_fn429_calc_ig__igindiode_nohinj_dn8_slot: &mut f64,
        var_fn429_calc_ig__igindiode_nohinj_vgsat_slot: &mut f64,
        var_fn429_calc_ig__igindiode_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn429_calc_ig__iginrec_slot: &mut f64,
        var_fn429_calc_ig__iginrec_dn13_slot: &mut f64,
        var_fn429_calc_ig__iginrec_dn4_slot: &mut f64,
        var_fn429_calc_ig__iginrec_dn8_slot: &mut f64,
        var_fn429_calc_ig__pg_paramin_hinj_slot: &mut f64,
        var_fn429_calc_ig__t0_slot: &mut f64,
        var_fn429_calc_ig__t0_dn4_slot: &mut f64,
    ) {
        let mut var_fn429_calc_ig__expbd1: f64 = *var_fn429_calc_ig__expbd1_slot;
        let mut var_fn429_calc_ig__expbd1_dn13: f64 = *var_fn429_calc_ig__expbd1_dn13_slot;
        let mut var_fn429_calc_ig__expbd1_dn4: f64 = *var_fn429_calc_ig__expbd1_dn4_slot;
        let mut var_fn429_calc_ig__expbd1_dn8: f64 = *var_fn429_calc_ig__expbd1_dn8_slot;
        let mut var_fn429_calc_ig__expbd1_vgsat: f64 = *var_fn429_calc_ig__expbd1_vgsat_slot;
        let mut var_fn429_calc_ig__expbd1_vgsat_dn4: f64 = *var_fn429_calc_ig__expbd1_vgsat_dn4_slot;
        let mut var_fn429_calc_ig__expbd2: f64 = *var_fn429_calc_ig__expbd2_slot;
        let mut var_fn429_calc_ig__expbd2_dn4: f64 = *var_fn429_calc_ig__expbd2_dn4_slot;
        let mut var_fn429_calc_ig__expbdarg1: f64 = *var_fn429_calc_ig__expbdarg1_slot;
        let mut var_fn429_calc_ig__expbdarg1_dn13: f64 = *var_fn429_calc_ig__expbdarg1_dn13_slot;
        let mut var_fn429_calc_ig__expbdarg1_dn4: f64 = *var_fn429_calc_ig__expbdarg1_dn4_slot;
        let mut var_fn429_calc_ig__expbdarg1_dn8: f64 = *var_fn429_calc_ig__expbdarg1_dn8_slot;
        let mut var_fn429_calc_ig__expbdarg1_vgsat: f64 = *var_fn429_calc_ig__expbdarg1_vgsat_slot;
        let mut var_fn429_calc_ig__expbdarg1_vgsat_dn4: f64 = *var_fn429_calc_ig__expbdarg1_vgsat_dn4_slot;
        let mut var_fn429_calc_ig__expbdarg2: f64 = *var_fn429_calc_ig__expbdarg2_slot;
        let mut var_fn429_calc_ig__expbdarg2_dn4: f64 = *var_fn429_calc_ig__expbdarg2_dn4_slot;
        let mut var_fn429_calc_ig__expffvarg: f64 = *var_fn429_calc_ig__expffvarg_slot;
        let mut var_fn429_calc_ig__expffvarg_dn13: f64 = *var_fn429_calc_ig__expffvarg_dn13_slot;
        let mut var_fn429_calc_ig__expffvarg_dn4: f64 = *var_fn429_calc_ig__expffvarg_dn4_slot;
        let mut var_fn429_calc_ig__expffvarg_dn8: f64 = *var_fn429_calc_ig__expffvarg_dn8_slot;
        let mut var_fn429_calc_ig__expifor: f64 = *var_fn429_calc_ig__expifor_slot;
        let mut var_fn429_calc_ig__expifor_dn13: f64 = *var_fn429_calc_ig__expifor_dn13_slot;
        let mut var_fn429_calc_ig__expifor_dn4: f64 = *var_fn429_calc_ig__expifor_dn4_slot;
        let mut var_fn429_calc_ig__expifor_dn8: f64 = *var_fn429_calc_ig__expifor_dn8_slot;
        let mut var_fn429_calc_ig__expifor_hinj: f64 = *var_fn429_calc_ig__expifor_hinj_slot;
        let mut var_fn429_calc_ig__expifor_hinj_dn13: f64 = *var_fn429_calc_ig__expifor_hinj_dn13_slot;
        let mut var_fn429_calc_ig__expifor_hinj_dn4: f64 = *var_fn429_calc_ig__expifor_hinj_dn4_slot;
        let mut var_fn429_calc_ig__expifor_hinj_dn8: f64 = *var_fn429_calc_ig__expifor_hinj_dn8_slot;
        let mut var_fn429_calc_ig__expifor_hinj_vgsat: f64 = *var_fn429_calc_ig__expifor_hinj_vgsat_slot;
        let mut var_fn429_calc_ig__expifor_hinj_vgsat_dn4: f64 = *var_fn429_calc_ig__expifor_hinj_vgsat_dn4_slot;
        let mut var_fn429_calc_ig__expifor_nohinj_vgsat: f64 = *var_fn429_calc_ig__expifor_nohinj_vgsat_slot;
        let mut var_fn429_calc_ig__expifor_nohinj_vgsat_dn4: f64 = *var_fn429_calc_ig__expifor_nohinj_vgsat_dn4_slot;
        let mut var_fn429_calc_ig__expiforarg: f64 = *var_fn429_calc_ig__expiforarg_slot;
        let mut var_fn429_calc_ig__expiforarg_dn13: f64 = *var_fn429_calc_ig__expiforarg_dn13_slot;
        let mut var_fn429_calc_ig__expiforarg_dn4: f64 = *var_fn429_calc_ig__expiforarg_dn4_slot;
        let mut var_fn429_calc_ig__expiforarg_dn8: f64 = *var_fn429_calc_ig__expiforarg_dn8_slot;
        let mut var_fn429_calc_ig__expiforarg_hinj: f64 = *var_fn429_calc_ig__expiforarg_hinj_slot;
        let mut var_fn429_calc_ig__expiforarg_hinj_dn13: f64 = *var_fn429_calc_ig__expiforarg_hinj_dn13_slot;
        let mut var_fn429_calc_ig__expiforarg_hinj_dn4: f64 = *var_fn429_calc_ig__expiforarg_hinj_dn4_slot;
        let mut var_fn429_calc_ig__expiforarg_hinj_dn8: f64 = *var_fn429_calc_ig__expiforarg_hinj_dn8_slot;
        let mut var_fn429_calc_ig__expiforarg_hinj_vgsat: f64 = *var_fn429_calc_ig__expiforarg_hinj_vgsat_slot;
        let mut var_fn429_calc_ig__expiforarg_hinj_vgsat_dn4: f64 = *var_fn429_calc_ig__expiforarg_hinj_vgsat_dn4_slot;
        let mut var_fn429_calc_ig__expiforarg_nohinj_vgsat: f64 = *var_fn429_calc_ig__expiforarg_nohinj_vgsat_slot;
        let mut var_fn429_calc_ig__expiforarg_nohinj_vgsat_dn4: f64 = *var_fn429_calc_ig__expiforarg_nohinj_vgsat_dn4_slot;
        let mut var_fn429_calc_ig__expirev: f64 = *var_fn429_calc_ig__expirev_slot;
        let mut var_fn429_calc_ig__expirev_dn13: f64 = *var_fn429_calc_ig__expirev_dn13_slot;
        let mut var_fn429_calc_ig__expirev_dn4: f64 = *var_fn429_calc_ig__expirev_dn4_slot;
        let mut var_fn429_calc_ig__expirev_dn8: f64 = *var_fn429_calc_ig__expirev_dn8_slot;
        let mut var_fn429_calc_ig__expirevarg: f64 = *var_fn429_calc_ig__expirevarg_slot;
        let mut var_fn429_calc_ig__expirevarg_dn13: f64 = *var_fn429_calc_ig__expirevarg_dn13_slot;
        let mut var_fn429_calc_ig__expirevarg_dn4: f64 = *var_fn429_calc_ig__expirevarg_dn4_slot;
        let mut var_fn429_calc_ig__expirevarg_dn8: f64 = *var_fn429_calc_ig__expirevarg_dn8_slot;
        let mut var_fn429_calc_ig__expphib: f64 = *var_fn429_calc_ig__expphib_slot;
        let mut var_fn429_calc_ig__expphib_dn4: f64 = *var_fn429_calc_ig__expphib_dn4_slot;
        let mut var_fn429_calc_ig__ffvgin: f64 = *var_fn429_calc_ig__ffvgin_slot;
        let mut var_fn429_calc_ig__ffvgin_dn13: f64 = *var_fn429_calc_ig__ffvgin_dn13_slot;
        let mut var_fn429_calc_ig__ffvgin_dn4: f64 = *var_fn429_calc_ig__ffvgin_dn4_slot;
        let mut var_fn429_calc_ig__ffvgin_dn8: f64 = *var_fn429_calc_ig__ffvgin_dn8_slot;
        let mut var_fn429_calc_ig__frecgin: f64 = *var_fn429_calc_ig__frecgin_slot;
        let mut var_fn429_calc_ig__frecgin_dn13: f64 = *var_fn429_calc_ig__frecgin_dn13_slot;
        let mut var_fn429_calc_ig__frecgin_dn8: f64 = *var_fn429_calc_ig__frecgin_dn8_slot;
        let mut var_fn429_calc_ig__iginbd: f64 = *var_fn429_calc_ig__iginbd_slot;
        let mut var_fn429_calc_ig__iginbd_dn13: f64 = *var_fn429_calc_ig__iginbd_dn13_slot;
        let mut var_fn429_calc_ig__iginbd_dn4: f64 = *var_fn429_calc_ig__iginbd_dn4_slot;
        let mut var_fn429_calc_ig__iginbd_dn8: f64 = *var_fn429_calc_ig__iginbd_dn8_slot;
        let mut var_fn429_calc_ig__iginbd_vgsat: f64 = *var_fn429_calc_ig__iginbd_vgsat_slot;
        let mut var_fn429_calc_ig__iginbd_vgsat_dn4: f64 = *var_fn429_calc_ig__iginbd_vgsat_dn4_slot;
        let mut var_fn429_calc_ig__igindiode: f64 = *var_fn429_calc_ig__igindiode_slot;
        let mut var_fn429_calc_ig__igindiode_dn13: f64 = *var_fn429_calc_ig__igindiode_dn13_slot;
        let mut var_fn429_calc_ig__igindiode_dn4: f64 = *var_fn429_calc_ig__igindiode_dn4_slot;
        let mut var_fn429_calc_ig__igindiode_dn8: f64 = *var_fn429_calc_ig__igindiode_dn8_slot;
        let mut var_fn429_calc_ig__igindiode_hinj: f64 = *var_fn429_calc_ig__igindiode_hinj_slot;
        let mut var_fn429_calc_ig__igindiode_hinj_dn13: f64 = *var_fn429_calc_ig__igindiode_hinj_dn13_slot;
        let mut var_fn429_calc_ig__igindiode_hinj_dn4: f64 = *var_fn429_calc_ig__igindiode_hinj_dn4_slot;
        let mut var_fn429_calc_ig__igindiode_hinj_dn8: f64 = *var_fn429_calc_ig__igindiode_hinj_dn8_slot;
        let mut var_fn429_calc_ig__igindiode_hinj_pre: f64 = *var_fn429_calc_ig__igindiode_hinj_pre_slot;
        let mut var_fn429_calc_ig__igindiode_hinj_pre_dn4: f64 = *var_fn429_calc_ig__igindiode_hinj_pre_dn4_slot;
        let mut var_fn429_calc_ig__igindiode_hinj_vgsat: f64 = *var_fn429_calc_ig__igindiode_hinj_vgsat_slot;
        let mut var_fn429_calc_ig__igindiode_hinj_vgsat_dn4: f64 = *var_fn429_calc_ig__igindiode_hinj_vgsat_dn4_slot;
        let mut var_fn429_calc_ig__igindiode_nohinj: f64 = *var_fn429_calc_ig__igindiode_nohinj_slot;
        let mut var_fn429_calc_ig__igindiode_nohinj_dn13: f64 = *var_fn429_calc_ig__igindiode_nohinj_dn13_slot;
        let mut var_fn429_calc_ig__igindiode_nohinj_dn4: f64 = *var_fn429_calc_ig__igindiode_nohinj_dn4_slot;
        let mut var_fn429_calc_ig__igindiode_nohinj_dn8: f64 = *var_fn429_calc_ig__igindiode_nohinj_dn8_slot;
        let mut var_fn429_calc_ig__igindiode_nohinj_vgsat: f64 = *var_fn429_calc_ig__igindiode_nohinj_vgsat_slot;
        let mut var_fn429_calc_ig__igindiode_nohinj_vgsat_dn4: f64 = *var_fn429_calc_ig__igindiode_nohinj_vgsat_dn4_slot;
        let mut var_fn429_calc_ig__iginrec: f64 = *var_fn429_calc_ig__iginrec_slot;
        let mut var_fn429_calc_ig__iginrec_dn13: f64 = *var_fn429_calc_ig__iginrec_dn13_slot;
        let mut var_fn429_calc_ig__iginrec_dn4: f64 = *var_fn429_calc_ig__iginrec_dn4_slot;
        let mut var_fn429_calc_ig__iginrec_dn8: f64 = *var_fn429_calc_ig__iginrec_dn8_slot;
        let mut var_fn429_calc_ig__pg_paramin_hinj: f64 = *var_fn429_calc_ig__pg_paramin_hinj_slot;
        let mut var_fn429_calc_ig__t0: f64 = *var_fn429_calc_ig__t0_slot;
        let mut var_fn429_calc_ig__t0_dn4: f64 = *var_fn429_calc_ig__t0_dn4_slot;

        let (assign35390_e32167, assign35390_e32167_d_n4, assign35390_e32167_d_n8, assign35390_e32167_d_n13,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn429_calc_ig__ffvgin, var_fn429_calc_ig__ffvgin_dn4, var_fn429_calc_ig__ffvgin_dn8, var_fn429_calc_ig__ffvgin_dn13,)
    }
};
        var_fn429_calc_ig__ffvgin = assign35390_e32167;
        var_fn429_calc_ig__ffvgin_dn4 = assign35390_e32167_d_n4;
        var_fn429_calc_ig__ffvgin_dn8 = assign35390_e32167_d_n8;
        var_fn429_calc_ig__ffvgin_dn13 = assign35390_e32167_d_n13;

        let (assign35400_e32173, assign35400_e32173_d_n4, assign35400_e32173_d_n8, assign35400_e32173_d_n13,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn429_calc_ig__iginbd, var_fn429_calc_ig__iginbd_dn4, var_fn429_calc_ig__iginbd_dn8, var_fn429_calc_ig__iginbd_dn13,)
    }
};
        var_fn429_calc_ig__iginbd = assign35400_e32173;
        var_fn429_calc_ig__iginbd_dn4 = assign35400_e32173_d_n4;
        var_fn429_calc_ig__iginbd_dn8 = assign35400_e32173_d_n8;
        var_fn429_calc_ig__iginbd_dn13 = assign35400_e32173_d_n13;

        let (assign35410_e32179, assign35410_e32179_d_n4, assign35410_e32179_d_n8, assign35410_e32179_d_n13,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn429_calc_ig__igindiode, var_fn429_calc_ig__igindiode_dn4, var_fn429_calc_ig__igindiode_dn8, var_fn429_calc_ig__igindiode_dn13,)
    }
};
        var_fn429_calc_ig__igindiode = assign35410_e32179;
        var_fn429_calc_ig__igindiode_dn4 = assign35410_e32179_d_n4;
        var_fn429_calc_ig__igindiode_dn8 = assign35410_e32179_d_n8;
        var_fn429_calc_ig__igindiode_dn13 = assign35410_e32179_d_n13;

        let (assign35420_e32185, assign35420_e32185_d_n8, assign35420_e32185_d_n13,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_fn429_calc_ig__frecgin, var_fn429_calc_ig__frecgin_dn8, var_fn429_calc_ig__frecgin_dn13,)
    }
};
        var_fn429_calc_ig__frecgin = assign35420_e32185;
        var_fn429_calc_ig__frecgin_dn8 = assign35420_e32185_d_n8;
        var_fn429_calc_ig__frecgin_dn13 = assign35420_e32185_d_n13;

        let (assign35430_e32191, assign35430_e32191_d_n4, assign35430_e32191_d_n8, assign35430_e32191_d_n13,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn429_calc_ig__iginrec, var_fn429_calc_ig__iginrec_dn4, var_fn429_calc_ig__iginrec_dn8, var_fn429_calc_ig__iginrec_dn13,)
    }
};
        var_fn429_calc_ig__iginrec = assign35430_e32191;
        var_fn429_calc_ig__iginrec_dn4 = assign35430_e32191_d_n4;
        var_fn429_calc_ig__iginrec_dn8 = assign35430_e32191_d_n8;
        var_fn429_calc_ig__iginrec_dn13 = assign35430_e32191_d_n13;

        let (assign35440_e32197, assign35440_e32197_d_n4, assign35440_e32197_d_n8, assign35440_e32197_d_n13,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn429_calc_ig__expbdarg1, var_fn429_calc_ig__expbdarg1_dn4, var_fn429_calc_ig__expbdarg1_dn8, var_fn429_calc_ig__expbdarg1_dn13,)
    }
};
        var_fn429_calc_ig__expbdarg1 = assign35440_e32197;
        var_fn429_calc_ig__expbdarg1_dn4 = assign35440_e32197_d_n4;
        var_fn429_calc_ig__expbdarg1_dn8 = assign35440_e32197_d_n8;
        var_fn429_calc_ig__expbdarg1_dn13 = assign35440_e32197_d_n13;

        let (assign35450_e32203, assign35450_e32203_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn429_calc_ig__expbdarg2, var_fn429_calc_ig__expbdarg2_dn4,)
    }
};
        var_fn429_calc_ig__expbdarg2 = assign35450_e32203;
        var_fn429_calc_ig__expbdarg2_dn4 = assign35450_e32203_d_n4;

        let (assign35460_e32209, assign35460_e32209_d_n4, assign35460_e32209_d_n8, assign35460_e32209_d_n13,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn429_calc_ig__expbd1, var_fn429_calc_ig__expbd1_dn4, var_fn429_calc_ig__expbd1_dn8, var_fn429_calc_ig__expbd1_dn13,)
    }
};
        var_fn429_calc_ig__expbd1 = assign35460_e32209;
        var_fn429_calc_ig__expbd1_dn4 = assign35460_e32209_d_n4;
        var_fn429_calc_ig__expbd1_dn8 = assign35460_e32209_d_n8;
        var_fn429_calc_ig__expbd1_dn13 = assign35460_e32209_d_n13;

        let (assign35470_e32215, assign35470_e32215_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn429_calc_ig__expbd2, var_fn429_calc_ig__expbd2_dn4,)
    }
};
        var_fn429_calc_ig__expbd2 = assign35470_e32215;
        var_fn429_calc_ig__expbd2_dn4 = assign35470_e32215_d_n4;

        let (assign35480_e32221, assign35480_e32221_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn429_calc_ig__expphib, var_fn429_calc_ig__expphib_dn4,)
    }
};
        var_fn429_calc_ig__expphib = assign35480_e32221;
        var_fn429_calc_ig__expphib_dn4 = assign35480_e32221_d_n4;

        let (assign35490_e32227, assign35490_e32227_d_n4, assign35490_e32227_d_n8, assign35490_e32227_d_n13,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn429_calc_ig__expffvarg, var_fn429_calc_ig__expffvarg_dn4, var_fn429_calc_ig__expffvarg_dn8, var_fn429_calc_ig__expffvarg_dn13,)
    }
};
        var_fn429_calc_ig__expffvarg = assign35490_e32227;
        var_fn429_calc_ig__expffvarg_dn4 = assign35490_e32227_d_n4;
        var_fn429_calc_ig__expffvarg_dn8 = assign35490_e32227_d_n8;
        var_fn429_calc_ig__expffvarg_dn13 = assign35490_e32227_d_n13;

        let (assign35500_e32233, assign35500_e32233_d_n4, assign35500_e32233_d_n8, assign35500_e32233_d_n13,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn429_calc_ig__expiforarg, var_fn429_calc_ig__expiforarg_dn4, var_fn429_calc_ig__expiforarg_dn8, var_fn429_calc_ig__expiforarg_dn13,)
    }
};
        var_fn429_calc_ig__expiforarg = assign35500_e32233;
        var_fn429_calc_ig__expiforarg_dn4 = assign35500_e32233_d_n4;
        var_fn429_calc_ig__expiforarg_dn8 = assign35500_e32233_d_n8;
        var_fn429_calc_ig__expiforarg_dn13 = assign35500_e32233_d_n13;

        let (assign35510_e32239, assign35510_e32239_d_n4, assign35510_e32239_d_n8, assign35510_e32239_d_n13,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn429_calc_ig__expifor, var_fn429_calc_ig__expifor_dn4, var_fn429_calc_ig__expifor_dn8, var_fn429_calc_ig__expifor_dn13,)
    }
};
        var_fn429_calc_ig__expifor = assign35510_e32239;
        var_fn429_calc_ig__expifor_dn4 = assign35510_e32239_d_n4;
        var_fn429_calc_ig__expifor_dn8 = assign35510_e32239_d_n8;
        var_fn429_calc_ig__expifor_dn13 = assign35510_e32239_d_n13;

        let (assign35520_e32245, assign35520_e32245_d_n4, assign35520_e32245_d_n8, assign35520_e32245_d_n13,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn429_calc_ig__expirevarg, var_fn429_calc_ig__expirevarg_dn4, var_fn429_calc_ig__expirevarg_dn8, var_fn429_calc_ig__expirevarg_dn13,)
    }
};
        var_fn429_calc_ig__expirevarg = assign35520_e32245;
        var_fn429_calc_ig__expirevarg_dn4 = assign35520_e32245_d_n4;
        var_fn429_calc_ig__expirevarg_dn8 = assign35520_e32245_d_n8;
        var_fn429_calc_ig__expirevarg_dn13 = assign35520_e32245_d_n13;

        let (assign35530_e32251, assign35530_e32251_d_n4, assign35530_e32251_d_n8, assign35530_e32251_d_n13,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn429_calc_ig__expirev, var_fn429_calc_ig__expirev_dn4, var_fn429_calc_ig__expirev_dn8, var_fn429_calc_ig__expirev_dn13,)
    }
};
        var_fn429_calc_ig__expirev = assign35530_e32251;
        var_fn429_calc_ig__expirev_dn4 = assign35530_e32251_d_n4;
        var_fn429_calc_ig__expirev_dn8 = assign35530_e32251_d_n8;
        var_fn429_calc_ig__expirev_dn13 = assign35530_e32251_d_n13;

        let (assign35540_e32257,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0,)
    } else {
        (var_fn429_calc_ig__pg_paramin_hinj,)
    }
};
        var_fn429_calc_ig__pg_paramin_hinj = assign35540_e32257;

        let (assign35550_e32263, assign35550_e32263_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn429_calc_ig__expbdarg1_vgsat, var_fn429_calc_ig__expbdarg1_vgsat_dn4,)
    }
};
        var_fn429_calc_ig__expbdarg1_vgsat = assign35550_e32263;
        var_fn429_calc_ig__expbdarg1_vgsat_dn4 = assign35550_e32263_d_n4;

        let (assign35560_e32269, assign35560_e32269_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn429_calc_ig__expbd1_vgsat, var_fn429_calc_ig__expbd1_vgsat_dn4,)
    }
};
        var_fn429_calc_ig__expbd1_vgsat = assign35560_e32269;
        var_fn429_calc_ig__expbd1_vgsat_dn4 = assign35560_e32269_d_n4;

        let (assign35570_e32275, assign35570_e32275_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn429_calc_ig__iginbd_vgsat, var_fn429_calc_ig__iginbd_vgsat_dn4,)
    }
};
        var_fn429_calc_ig__iginbd_vgsat = assign35570_e32275;
        var_fn429_calc_ig__iginbd_vgsat_dn4 = assign35570_e32275_d_n4;

        let (assign35580_e32281, assign35580_e32281_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn429_calc_ig__expiforarg_nohinj_vgsat, var_fn429_calc_ig__expiforarg_nohinj_vgsat_dn4,)
    }
};
        var_fn429_calc_ig__expiforarg_nohinj_vgsat = assign35580_e32281;
        var_fn429_calc_ig__expiforarg_nohinj_vgsat_dn4 = assign35580_e32281_d_n4;

        let (assign35590_e32287, assign35590_e32287_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn429_calc_ig__expifor_nohinj_vgsat, var_fn429_calc_ig__expifor_nohinj_vgsat_dn4,)
    }
};
        var_fn429_calc_ig__expifor_nohinj_vgsat = assign35590_e32287;
        var_fn429_calc_ig__expifor_nohinj_vgsat_dn4 = assign35590_e32287_d_n4;

        let (assign35600_e32293, assign35600_e32293_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn429_calc_ig__igindiode_nohinj_vgsat, var_fn429_calc_ig__igindiode_nohinj_vgsat_dn4,)
    }
};
        var_fn429_calc_ig__igindiode_nohinj_vgsat = assign35600_e32293;
        var_fn429_calc_ig__igindiode_nohinj_vgsat_dn4 = assign35600_e32293_d_n4;

        let (assign35610_e32299, assign35610_e32299_d_n4, assign35610_e32299_d_n8, assign35610_e32299_d_n13,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn429_calc_ig__igindiode_nohinj, var_fn429_calc_ig__igindiode_nohinj_dn4, var_fn429_calc_ig__igindiode_nohinj_dn8, var_fn429_calc_ig__igindiode_nohinj_dn13,)
    }
};
        var_fn429_calc_ig__igindiode_nohinj = assign35610_e32299;
        var_fn429_calc_ig__igindiode_nohinj_dn4 = assign35610_e32299_d_n4;
        var_fn429_calc_ig__igindiode_nohinj_dn8 = assign35610_e32299_d_n8;
        var_fn429_calc_ig__igindiode_nohinj_dn13 = assign35610_e32299_d_n13;

        let (assign35620_e32305, assign35620_e32305_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn429_calc_ig__expiforarg_hinj_vgsat, var_fn429_calc_ig__expiforarg_hinj_vgsat_dn4,)
    }
};
        var_fn429_calc_ig__expiforarg_hinj_vgsat = assign35620_e32305;
        var_fn429_calc_ig__expiforarg_hinj_vgsat_dn4 = assign35620_e32305_d_n4;

        let (assign35630_e32311, assign35630_e32311_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn429_calc_ig__expifor_hinj_vgsat, var_fn429_calc_ig__expifor_hinj_vgsat_dn4,)
    }
};
        var_fn429_calc_ig__expifor_hinj_vgsat = assign35630_e32311;
        var_fn429_calc_ig__expifor_hinj_vgsat_dn4 = assign35630_e32311_d_n4;

        let (assign35640_e32317, assign35640_e32317_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn429_calc_ig__igindiode_hinj_vgsat, var_fn429_calc_ig__igindiode_hinj_vgsat_dn4,)
    }
};
        var_fn429_calc_ig__igindiode_hinj_vgsat = assign35640_e32317;
        var_fn429_calc_ig__igindiode_hinj_vgsat_dn4 = assign35640_e32317_d_n4;

        let (assign35650_e32323, assign35650_e32323_d_n4, assign35650_e32323_d_n8, assign35650_e32323_d_n13,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn429_calc_ig__expiforarg_hinj, var_fn429_calc_ig__expiforarg_hinj_dn4, var_fn429_calc_ig__expiforarg_hinj_dn8, var_fn429_calc_ig__expiforarg_hinj_dn13,)
    }
};
        var_fn429_calc_ig__expiforarg_hinj = assign35650_e32323;
        var_fn429_calc_ig__expiforarg_hinj_dn4 = assign35650_e32323_d_n4;
        var_fn429_calc_ig__expiforarg_hinj_dn8 = assign35650_e32323_d_n8;
        var_fn429_calc_ig__expiforarg_hinj_dn13 = assign35650_e32323_d_n13;

        let (assign35660_e32329, assign35660_e32329_d_n4, assign35660_e32329_d_n8, assign35660_e32329_d_n13,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn429_calc_ig__expifor_hinj, var_fn429_calc_ig__expifor_hinj_dn4, var_fn429_calc_ig__expifor_hinj_dn8, var_fn429_calc_ig__expifor_hinj_dn13,)
    }
};
        var_fn429_calc_ig__expifor_hinj = assign35660_e32329;
        var_fn429_calc_ig__expifor_hinj_dn4 = assign35660_e32329_d_n4;
        var_fn429_calc_ig__expifor_hinj_dn8 = assign35660_e32329_d_n8;
        var_fn429_calc_ig__expifor_hinj_dn13 = assign35660_e32329_d_n13;

        let (assign35670_e32335, assign35670_e32335_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn429_calc_ig__igindiode_hinj_pre, var_fn429_calc_ig__igindiode_hinj_pre_dn4,)
    }
};
        var_fn429_calc_ig__igindiode_hinj_pre = assign35670_e32335;
        var_fn429_calc_ig__igindiode_hinj_pre_dn4 = assign35670_e32335_d_n4;

        let (assign35680_e32341, assign35680_e32341_d_n4, assign35680_e32341_d_n8, assign35680_e32341_d_n13,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn429_calc_ig__igindiode_hinj, var_fn429_calc_ig__igindiode_hinj_dn4, var_fn429_calc_ig__igindiode_hinj_dn8, var_fn429_calc_ig__igindiode_hinj_dn13,)
    }
};
        var_fn429_calc_ig__igindiode_hinj = assign35680_e32341;
        var_fn429_calc_ig__igindiode_hinj_dn4 = assign35680_e32341_d_n4;
        var_fn429_calc_ig__igindiode_hinj_dn8 = assign35680_e32341_d_n8;
        var_fn429_calc_ig__igindiode_hinj_dn13 = assign35680_e32341_d_n13;

        let (assign35690_e32352, assign35690_e32352_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        let assign35690_e32347: f64 = (var_fn429_calc_ig__pg_param1 / var_fn429_calc_ig__phitin);
        let assign35690_e32349: f64 = (-var_fn429_calc_ig__vjg);
        let assign35690_e32350: f64 = (assign35690_e32347 * assign35690_e32349);
        (assign35690_e32350, ((-((var_fn429_calc_ig__pg_param1 * var_fn429_calc_ig__phitin_dn4) / (var_fn429_calc_ig__phitin * var_fn429_calc_ig__phitin))) * assign35690_e32349),)
    } else {
        (var_fn429_calc_ig__expphib, var_fn429_calc_ig__expphib_dn4,)
    }
};
        var_fn429_calc_ig__expphib = assign35690_e32352;
        var_fn429_calc_ig__expphib_dn4 = assign35690_e32352_d_n4;

        let (assign35700_e32396, assign35700_e32396_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        let assign35700_e32362: f64 = (-50.0);
        let (assign35700_e32394, assign35700_e32394_d_n4,) = {
            if ((!(var_fn429_calc_ig__expphib > 50.0)) && (!(var_fn429_calc_ig__expphib < assign35700_e32362))) {
                let assign35700_e32367: f64 = (var_fn429_calc_ig__expphib).exp();
                (assign35700_e32367, (assign35700_e32367 * var_fn429_calc_ig__expphib_dn4),)
            } else {
                let assign35700_e32374: f64 = (-50.0);
                let (assign35700_e32393, assign35700_e32393_d_n4,) = {
                    if ((!(var_fn429_calc_ig__expphib > 50.0)) && (var_fn429_calc_ig__expphib < assign35700_e32374)) {
                        let assign35700_e32378: f64 = (-50.0);
                        let assign35700_e32379: f64 = (assign35700_e32378).exp();
                        (assign35700_e32379, 0.0,)
                    } else {
                        let (assign35700_e32392, assign35700_e32392_d_n4,) = {
                            if (var_fn429_calc_ig__expphib > 50.0) {
                                let assign35700_e32384: f64 = (50.0_f64).exp();
                                let assign35700_e32388: f64 = (var_fn429_calc_ig__expphib - 50.0);
                                let assign35700_e32389: f64 = (1.0 + assign35700_e32388);
                                let assign35700_e32390: f64 = (assign35700_e32384 * assign35700_e32389);
                                (assign35700_e32390, (assign35700_e32384 * var_fn429_calc_ig__expphib_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign35700_e32392, assign35700_e32392_d_n4,)
                    }
                };
                (assign35700_e32393, assign35700_e32393_d_n4,)
            }
        };
        (assign35700_e32394, assign35700_e32394_d_n4,)
    } else {
        (var_fn429_calc_ig__t0, var_fn429_calc_ig__t0_dn4,)
    }
};
        var_fn429_calc_ig__t0 = assign35700_e32396;
        var_fn429_calc_ig__t0_dn4 = assign35700_e32396_d_n4;

        let (assign35710_e32409, assign35710_e32409_d_n4, assign35710_e32409_d_n8, assign35710_e32409_d_n13,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        let assign35710_e32402: f64 = (-var_fn429_calc_ig__vgin);
        let assign35710_e32404: f64 = (assign35710_e32402 - var_fn429_calc_ig__vbdgin);
        let assign35710_e32405: f64 = (var_fn429_calc_ig__pbdgin * assign35710_e32404);
        let assign35710_e32407: f64 = (assign35710_e32405 + var_fn429_calc_ig__expphib);
        (assign35710_e32407, var_fn429_calc_ig__expphib_dn4, (var_fn429_calc_ig__pbdgin * (-var_fn429_calc_ig__vgin_dn8)), (var_fn429_calc_ig__pbdgin * (-var_fn429_calc_ig__vgin_dn13)),)
    } else {
        (var_fn429_calc_ig__expbdarg1, var_fn429_calc_ig__expbdarg1_dn4, var_fn429_calc_ig__expbdarg1_dn8, var_fn429_calc_ig__expbdarg1_dn13,)
    }
};
        var_fn429_calc_ig__expbdarg1 = assign35710_e32409;
        var_fn429_calc_ig__expbdarg1_dn4 = assign35710_e32409_d_n4;
        var_fn429_calc_ig__expbdarg1_dn8 = assign35710_e32409_d_n8;
        var_fn429_calc_ig__expbdarg1_dn13 = assign35710_e32409_d_n13;

        let (assign35720_e32420, assign35720_e32420_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        let assign35720_e32414: f64 = (-var_fn429_calc_ig__pbdgin);
        let assign35720_e32416: f64 = (assign35720_e32414 * var_fn429_calc_ig__vbdgin);
        let assign35720_e32418: f64 = (assign35720_e32416 + var_fn429_calc_ig__expphib);
        (assign35720_e32418, var_fn429_calc_ig__expphib_dn4,)
    } else {
        (var_fn429_calc_ig__expbdarg2, var_fn429_calc_ig__expbdarg2_dn4,)
    }
};
        var_fn429_calc_ig__expbdarg2 = assign35720_e32420;
        var_fn429_calc_ig__expbdarg2_dn4 = assign35720_e32420_d_n4;

        let (assign35730_e32464, assign35730_e32464_d_n4, assign35730_e32464_d_n8, assign35730_e32464_d_n13,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        let assign35730_e32430: f64 = (-50.0);
        let (assign35730_e32462, assign35730_e32462_d_n4, assign35730_e32462_d_n8, assign35730_e32462_d_n13,) = {
            if ((!(var_fn429_calc_ig__expbdarg1 > 50.0)) && (!(var_fn429_calc_ig__expbdarg1 < assign35730_e32430))) {
                let assign35730_e32435: f64 = (var_fn429_calc_ig__expbdarg1).exp();
                (assign35730_e32435, (assign35730_e32435 * var_fn429_calc_ig__expbdarg1_dn4), (assign35730_e32435 * var_fn429_calc_ig__expbdarg1_dn8), (assign35730_e32435 * var_fn429_calc_ig__expbdarg1_dn13),)
            } else {
                let assign35730_e32442: f64 = (-50.0);
                let (assign35730_e32461, assign35730_e32461_d_n4, assign35730_e32461_d_n8, assign35730_e32461_d_n13,) = {
                    if ((!(var_fn429_calc_ig__expbdarg1 > 50.0)) && (var_fn429_calc_ig__expbdarg1 < assign35730_e32442)) {
                        let assign35730_e32446: f64 = (-50.0);
                        let assign35730_e32447: f64 = (assign35730_e32446).exp();
                        (assign35730_e32447, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign35730_e32460, assign35730_e32460_d_n4, assign35730_e32460_d_n8, assign35730_e32460_d_n13,) = {
                            if (var_fn429_calc_ig__expbdarg1 > 50.0) {
                                let assign35730_e32452: f64 = (50.0_f64).exp();
                                let assign35730_e32456: f64 = (var_fn429_calc_ig__expbdarg1 - 50.0);
                                let assign35730_e32457: f64 = (1.0 + assign35730_e32456);
                                let assign35730_e32458: f64 = (assign35730_e32452 * assign35730_e32457);
                                (assign35730_e32458, (assign35730_e32452 * var_fn429_calc_ig__expbdarg1_dn4), (assign35730_e32452 * var_fn429_calc_ig__expbdarg1_dn8), (assign35730_e32452 * var_fn429_calc_ig__expbdarg1_dn13),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign35730_e32460, assign35730_e32460_d_n4, assign35730_e32460_d_n8, assign35730_e32460_d_n13,)
                    }
                };
                (assign35730_e32461, assign35730_e32461_d_n4, assign35730_e32461_d_n8, assign35730_e32461_d_n13,)
            }
        };
        (assign35730_e32462, assign35730_e32462_d_n4, assign35730_e32462_d_n8, assign35730_e32462_d_n13,)
    } else {
        (var_fn429_calc_ig__expbd1, var_fn429_calc_ig__expbd1_dn4, var_fn429_calc_ig__expbd1_dn8, var_fn429_calc_ig__expbd1_dn13,)
    }
};
        var_fn429_calc_ig__expbd1 = assign35730_e32464;
        var_fn429_calc_ig__expbd1_dn4 = assign35730_e32464_d_n4;
        var_fn429_calc_ig__expbd1_dn8 = assign35730_e32464_d_n8;
        var_fn429_calc_ig__expbd1_dn13 = assign35730_e32464_d_n13;

        let (assign35740_e32508, assign35740_e32508_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        let assign35740_e32474: f64 = (-50.0);
        let (assign35740_e32506, assign35740_e32506_d_n4,) = {
            if ((!(var_fn429_calc_ig__expbdarg2 > 50.0)) && (!(var_fn429_calc_ig__expbdarg2 < assign35740_e32474))) {
                let assign35740_e32479: f64 = (var_fn429_calc_ig__expbdarg2).exp();
                (assign35740_e32479, (assign35740_e32479 * var_fn429_calc_ig__expbdarg2_dn4),)
            } else {
                let assign35740_e32486: f64 = (-50.0);
                let (assign35740_e32505, assign35740_e32505_d_n4,) = {
                    if ((!(var_fn429_calc_ig__expbdarg2 > 50.0)) && (var_fn429_calc_ig__expbdarg2 < assign35740_e32486)) {
                        let assign35740_e32490: f64 = (-50.0);
                        let assign35740_e32491: f64 = (assign35740_e32490).exp();
                        (assign35740_e32491, 0.0,)
                    } else {
                        let (assign35740_e32504, assign35740_e32504_d_n4,) = {
                            if (var_fn429_calc_ig__expbdarg2 > 50.0) {
                                let assign35740_e32496: f64 = (50.0_f64).exp();
                                let assign35740_e32500: f64 = (var_fn429_calc_ig__expbdarg2 - 50.0);
                                let assign35740_e32501: f64 = (1.0 + assign35740_e32500);
                                let assign35740_e32502: f64 = (assign35740_e32496 * assign35740_e32501);
                                (assign35740_e32502, (assign35740_e32496 * var_fn429_calc_ig__expbdarg2_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign35740_e32504, assign35740_e32504_d_n4,)
                    }
                };
                (assign35740_e32505, assign35740_e32505_d_n4,)
            }
        };
        (assign35740_e32506, assign35740_e32506_d_n4,)
    } else {
        (var_fn429_calc_ig__expbd2, var_fn429_calc_ig__expbd2_dn4,)
    }
};
        var_fn429_calc_ig__expbd2 = assign35740_e32508;
        var_fn429_calc_ig__expbd2_dn4 = assign35740_e32508_d_n4;

        let (assign35750_e32516, assign35750_e32516_d_n4, assign35750_e32516_d_n8, assign35750_e32516_d_n13,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        let assign35750_e32514: f64 = (var_fn429_calc_ig__expbd1 - var_fn429_calc_ig__expbd2);
        (assign35750_e32514, (var_fn429_calc_ig__expbd1_dn4 - var_fn429_calc_ig__expbd2_dn4), var_fn429_calc_ig__expbd1_dn8, var_fn429_calc_ig__expbd1_dn13,)
    } else {
        (var_fn429_calc_ig__iginbd, var_fn429_calc_ig__iginbd_dn4, var_fn429_calc_ig__iginbd_dn8, var_fn429_calc_ig__iginbd_dn13,)
    }
};
        var_fn429_calc_ig__iginbd = assign35750_e32516;
        var_fn429_calc_ig__iginbd_dn4 = assign35750_e32516_d_n4;
        var_fn429_calc_ig__iginbd_dn8 = assign35750_e32516_d_n8;
        var_fn429_calc_ig__iginbd_dn13 = assign35750_e32516_d_n13;

        *var_fn429_calc_ig__expbd1_slot = var_fn429_calc_ig__expbd1;
        *var_fn429_calc_ig__expbd1_dn13_slot = var_fn429_calc_ig__expbd1_dn13;
        *var_fn429_calc_ig__expbd1_dn4_slot = var_fn429_calc_ig__expbd1_dn4;
        *var_fn429_calc_ig__expbd1_dn8_slot = var_fn429_calc_ig__expbd1_dn8;
        *var_fn429_calc_ig__expbd1_vgsat_slot = var_fn429_calc_ig__expbd1_vgsat;
        *var_fn429_calc_ig__expbd1_vgsat_dn4_slot = var_fn429_calc_ig__expbd1_vgsat_dn4;
        *var_fn429_calc_ig__expbd2_slot = var_fn429_calc_ig__expbd2;
        *var_fn429_calc_ig__expbd2_dn4_slot = var_fn429_calc_ig__expbd2_dn4;
        *var_fn429_calc_ig__expbdarg1_slot = var_fn429_calc_ig__expbdarg1;
        *var_fn429_calc_ig__expbdarg1_dn13_slot = var_fn429_calc_ig__expbdarg1_dn13;
        *var_fn429_calc_ig__expbdarg1_dn4_slot = var_fn429_calc_ig__expbdarg1_dn4;
        *var_fn429_calc_ig__expbdarg1_dn8_slot = var_fn429_calc_ig__expbdarg1_dn8;
        *var_fn429_calc_ig__expbdarg1_vgsat_slot = var_fn429_calc_ig__expbdarg1_vgsat;
        *var_fn429_calc_ig__expbdarg1_vgsat_dn4_slot = var_fn429_calc_ig__expbdarg1_vgsat_dn4;
        *var_fn429_calc_ig__expbdarg2_slot = var_fn429_calc_ig__expbdarg2;
        *var_fn429_calc_ig__expbdarg2_dn4_slot = var_fn429_calc_ig__expbdarg2_dn4;
        *var_fn429_calc_ig__expffvarg_slot = var_fn429_calc_ig__expffvarg;
        *var_fn429_calc_ig__expffvarg_dn13_slot = var_fn429_calc_ig__expffvarg_dn13;
        *var_fn429_calc_ig__expffvarg_dn4_slot = var_fn429_calc_ig__expffvarg_dn4;
        *var_fn429_calc_ig__expffvarg_dn8_slot = var_fn429_calc_ig__expffvarg_dn8;
        *var_fn429_calc_ig__expifor_slot = var_fn429_calc_ig__expifor;
        *var_fn429_calc_ig__expifor_dn13_slot = var_fn429_calc_ig__expifor_dn13;
        *var_fn429_calc_ig__expifor_dn4_slot = var_fn429_calc_ig__expifor_dn4;
        *var_fn429_calc_ig__expifor_dn8_slot = var_fn429_calc_ig__expifor_dn8;
        *var_fn429_calc_ig__expifor_hinj_slot = var_fn429_calc_ig__expifor_hinj;
        *var_fn429_calc_ig__expifor_hinj_dn13_slot = var_fn429_calc_ig__expifor_hinj_dn13;
        *var_fn429_calc_ig__expifor_hinj_dn4_slot = var_fn429_calc_ig__expifor_hinj_dn4;
        *var_fn429_calc_ig__expifor_hinj_dn8_slot = var_fn429_calc_ig__expifor_hinj_dn8;
        *var_fn429_calc_ig__expifor_hinj_vgsat_slot = var_fn429_calc_ig__expifor_hinj_vgsat;
        *var_fn429_calc_ig__expifor_hinj_vgsat_dn4_slot = var_fn429_calc_ig__expifor_hinj_vgsat_dn4;
        *var_fn429_calc_ig__expifor_nohinj_vgsat_slot = var_fn429_calc_ig__expifor_nohinj_vgsat;
        *var_fn429_calc_ig__expifor_nohinj_vgsat_dn4_slot = var_fn429_calc_ig__expifor_nohinj_vgsat_dn4;
        *var_fn429_calc_ig__expiforarg_slot = var_fn429_calc_ig__expiforarg;
        *var_fn429_calc_ig__expiforarg_dn13_slot = var_fn429_calc_ig__expiforarg_dn13;
        *var_fn429_calc_ig__expiforarg_dn4_slot = var_fn429_calc_ig__expiforarg_dn4;
        *var_fn429_calc_ig__expiforarg_dn8_slot = var_fn429_calc_ig__expiforarg_dn8;
        *var_fn429_calc_ig__expiforarg_hinj_slot = var_fn429_calc_ig__expiforarg_hinj;
        *var_fn429_calc_ig__expiforarg_hinj_dn13_slot = var_fn429_calc_ig__expiforarg_hinj_dn13;
        *var_fn429_calc_ig__expiforarg_hinj_dn4_slot = var_fn429_calc_ig__expiforarg_hinj_dn4;
        *var_fn429_calc_ig__expiforarg_hinj_dn8_slot = var_fn429_calc_ig__expiforarg_hinj_dn8;
        *var_fn429_calc_ig__expiforarg_hinj_vgsat_slot = var_fn429_calc_ig__expiforarg_hinj_vgsat;
        *var_fn429_calc_ig__expiforarg_hinj_vgsat_dn4_slot = var_fn429_calc_ig__expiforarg_hinj_vgsat_dn4;
        *var_fn429_calc_ig__expiforarg_nohinj_vgsat_slot = var_fn429_calc_ig__expiforarg_nohinj_vgsat;
        *var_fn429_calc_ig__expiforarg_nohinj_vgsat_dn4_slot = var_fn429_calc_ig__expiforarg_nohinj_vgsat_dn4;
        *var_fn429_calc_ig__expirev_slot = var_fn429_calc_ig__expirev;
        *var_fn429_calc_ig__expirev_dn13_slot = var_fn429_calc_ig__expirev_dn13;
        *var_fn429_calc_ig__expirev_dn4_slot = var_fn429_calc_ig__expirev_dn4;
        *var_fn429_calc_ig__expirev_dn8_slot = var_fn429_calc_ig__expirev_dn8;
        *var_fn429_calc_ig__expirevarg_slot = var_fn429_calc_ig__expirevarg;
        *var_fn429_calc_ig__expirevarg_dn13_slot = var_fn429_calc_ig__expirevarg_dn13;
        *var_fn429_calc_ig__expirevarg_dn4_slot = var_fn429_calc_ig__expirevarg_dn4;
        *var_fn429_calc_ig__expirevarg_dn8_slot = var_fn429_calc_ig__expirevarg_dn8;
        *var_fn429_calc_ig__expphib_slot = var_fn429_calc_ig__expphib;
        *var_fn429_calc_ig__expphib_dn4_slot = var_fn429_calc_ig__expphib_dn4;
        *var_fn429_calc_ig__ffvgin_slot = var_fn429_calc_ig__ffvgin;
        *var_fn429_calc_ig__ffvgin_dn13_slot = var_fn429_calc_ig__ffvgin_dn13;
        *var_fn429_calc_ig__ffvgin_dn4_slot = var_fn429_calc_ig__ffvgin_dn4;
        *var_fn429_calc_ig__ffvgin_dn8_slot = var_fn429_calc_ig__ffvgin_dn8;
        *var_fn429_calc_ig__frecgin_slot = var_fn429_calc_ig__frecgin;
        *var_fn429_calc_ig__frecgin_dn13_slot = var_fn429_calc_ig__frecgin_dn13;
        *var_fn429_calc_ig__frecgin_dn8_slot = var_fn429_calc_ig__frecgin_dn8;
        *var_fn429_calc_ig__iginbd_slot = var_fn429_calc_ig__iginbd;
        *var_fn429_calc_ig__iginbd_dn13_slot = var_fn429_calc_ig__iginbd_dn13;
        *var_fn429_calc_ig__iginbd_dn4_slot = var_fn429_calc_ig__iginbd_dn4;
        *var_fn429_calc_ig__iginbd_dn8_slot = var_fn429_calc_ig__iginbd_dn8;
        *var_fn429_calc_ig__iginbd_vgsat_slot = var_fn429_calc_ig__iginbd_vgsat;
        *var_fn429_calc_ig__iginbd_vgsat_dn4_slot = var_fn429_calc_ig__iginbd_vgsat_dn4;
        *var_fn429_calc_ig__igindiode_slot = var_fn429_calc_ig__igindiode;
        *var_fn429_calc_ig__igindiode_dn13_slot = var_fn429_calc_ig__igindiode_dn13;
        *var_fn429_calc_ig__igindiode_dn4_slot = var_fn429_calc_ig__igindiode_dn4;
        *var_fn429_calc_ig__igindiode_dn8_slot = var_fn429_calc_ig__igindiode_dn8;
        *var_fn429_calc_ig__igindiode_hinj_slot = var_fn429_calc_ig__igindiode_hinj;
        *var_fn429_calc_ig__igindiode_hinj_dn13_slot = var_fn429_calc_ig__igindiode_hinj_dn13;
        *var_fn429_calc_ig__igindiode_hinj_dn4_slot = var_fn429_calc_ig__igindiode_hinj_dn4;
        *var_fn429_calc_ig__igindiode_hinj_dn8_slot = var_fn429_calc_ig__igindiode_hinj_dn8;
        *var_fn429_calc_ig__igindiode_hinj_pre_slot = var_fn429_calc_ig__igindiode_hinj_pre;
        *var_fn429_calc_ig__igindiode_hinj_pre_dn4_slot = var_fn429_calc_ig__igindiode_hinj_pre_dn4;
        *var_fn429_calc_ig__igindiode_hinj_vgsat_slot = var_fn429_calc_ig__igindiode_hinj_vgsat;
        *var_fn429_calc_ig__igindiode_hinj_vgsat_dn4_slot = var_fn429_calc_ig__igindiode_hinj_vgsat_dn4;
        *var_fn429_calc_ig__igindiode_nohinj_slot = var_fn429_calc_ig__igindiode_nohinj;
        *var_fn429_calc_ig__igindiode_nohinj_dn13_slot = var_fn429_calc_ig__igindiode_nohinj_dn13;
        *var_fn429_calc_ig__igindiode_nohinj_dn4_slot = var_fn429_calc_ig__igindiode_nohinj_dn4;
        *var_fn429_calc_ig__igindiode_nohinj_dn8_slot = var_fn429_calc_ig__igindiode_nohinj_dn8;
        *var_fn429_calc_ig__igindiode_nohinj_vgsat_slot = var_fn429_calc_ig__igindiode_nohinj_vgsat;
        *var_fn429_calc_ig__igindiode_nohinj_vgsat_dn4_slot = var_fn429_calc_ig__igindiode_nohinj_vgsat_dn4;
        *var_fn429_calc_ig__iginrec_slot = var_fn429_calc_ig__iginrec;
        *var_fn429_calc_ig__iginrec_dn13_slot = var_fn429_calc_ig__iginrec_dn13;
        *var_fn429_calc_ig__iginrec_dn4_slot = var_fn429_calc_ig__iginrec_dn4;
        *var_fn429_calc_ig__iginrec_dn8_slot = var_fn429_calc_ig__iginrec_dn8;
        *var_fn429_calc_ig__pg_paramin_hinj_slot = var_fn429_calc_ig__pg_paramin_hinj;
        *var_fn429_calc_ig__t0_slot = var_fn429_calc_ig__t0;
        *var_fn429_calc_ig__t0_dn4_slot = var_fn429_calc_ig__t0_dn4;
    }

    pub(super) fn stamp_transient_block_88(
        var_fn429_calc_ig__alphagin: f64,
        var_fn429_calc_ig__expbd2: f64,
        var_fn429_calc_ig__expbd2_dn4: f64,
        var_fn429_calc_ig__expphib: f64,
        var_fn429_calc_ig__expphib_dn4: f64,
        var_fn429_calc_ig__fracin: f64,
        var_fn429_calc_ig__iginbd: f64,
        var_fn429_calc_ig__iginbd_dn13: f64,
        var_fn429_calc_ig__iginbd_dn4: f64,
        var_fn429_calc_ig__iginbd_dn8: f64,
        var_fn429_calc_ig__ijin: f64,
        var_fn429_calc_ig__kbdgatein: f64,
        var_fn429_calc_ig__ngf: f64,
        var_fn429_calc_ig__pbdgin: f64,
        var_fn429_calc_ig__pg_paramin: f64,
        var_fn429_calc_ig__phitin: f64,
        var_fn429_calc_ig__phitin_dn4: f64,
        var_fn429_calc_ig__t0: f64,
        var_fn429_calc_ig__t0_dn4: f64,
        var_fn429_calc_ig__tfacdiodein: f64,
        var_fn429_calc_ig__tfacdiodein_dn4: f64,
        var_fn429_calc_ig__type: f64,
        var_fn429_calc_ig__vbdgin: f64,
        var_fn429_calc_ig__vgin: f64,
        var_fn429_calc_ig__vgin_dn13: f64,
        var_fn429_calc_ig__vgin_dn8: f64,
        var_fn429_calc_ig__vgsatin: f64,
        var_fn429_calc_ig__w: f64,
        var_guard417: f64,
        var_guard428: f64,
        var_fn429_calc_ig__alpha2_phit_slot: &mut f64,
        var_fn429_calc_ig__alpha2_phit_dn4_slot: &mut f64,
        var_fn429_calc_ig__expbd1_vgsat_slot: &mut f64,
        var_fn429_calc_ig__expbd1_vgsat_dn4_slot: &mut f64,
        var_fn429_calc_ig__expbdarg1_vgsat_slot: &mut f64,
        var_fn429_calc_ig__expbdarg1_vgsat_dn4_slot: &mut f64,
        var_fn429_calc_ig__expffvarg_slot: &mut f64,
        var_fn429_calc_ig__expffvarg_dn13_slot: &mut f64,
        var_fn429_calc_ig__expffvarg_dn4_slot: &mut f64,
        var_fn429_calc_ig__expffvarg_dn8_slot: &mut f64,
        var_fn429_calc_ig__expifor_slot: &mut f64,
        var_fn429_calc_ig__expifor_dn13_slot: &mut f64,
        var_fn429_calc_ig__expifor_dn4_slot: &mut f64,
        var_fn429_calc_ig__expifor_dn8_slot: &mut f64,
        var_fn429_calc_ig__expifor_hinj_slot: &mut f64,
        var_fn429_calc_ig__expifor_hinj_dn13_slot: &mut f64,
        var_fn429_calc_ig__expifor_hinj_dn4_slot: &mut f64,
        var_fn429_calc_ig__expifor_hinj_dn8_slot: &mut f64,
        var_fn429_calc_ig__expifor_hinj_vgsat_slot: &mut f64,
        var_fn429_calc_ig__expifor_hinj_vgsat_dn4_slot: &mut f64,
        var_fn429_calc_ig__expifor_nohinj_vgsat_slot: &mut f64,
        var_fn429_calc_ig__expifor_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn429_calc_ig__expiforarg_slot: &mut f64,
        var_fn429_calc_ig__expiforarg_dn13_slot: &mut f64,
        var_fn429_calc_ig__expiforarg_dn4_slot: &mut f64,
        var_fn429_calc_ig__expiforarg_dn8_slot: &mut f64,
        var_fn429_calc_ig__expiforarg_hinj_slot: &mut f64,
        var_fn429_calc_ig__expiforarg_hinj_dn13_slot: &mut f64,
        var_fn429_calc_ig__expiforarg_hinj_dn4_slot: &mut f64,
        var_fn429_calc_ig__expiforarg_hinj_dn8_slot: &mut f64,
        var_fn429_calc_ig__expiforarg_hinj_vgsat_slot: &mut f64,
        var_fn429_calc_ig__expiforarg_hinj_vgsat_dn4_slot: &mut f64,
        var_fn429_calc_ig__expiforarg_nohinj_vgsat_slot: &mut f64,
        var_fn429_calc_ig__expiforarg_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn429_calc_ig__ffvgin_slot: &mut f64,
        var_fn429_calc_ig__ffvgin_dn13_slot: &mut f64,
        var_fn429_calc_ig__ffvgin_dn4_slot: &mut f64,
        var_fn429_calc_ig__ffvgin_dn8_slot: &mut f64,
        var_fn429_calc_ig__iginbd_vgsat_slot: &mut f64,
        var_fn429_calc_ig__iginbd_vgsat_dn4_slot: &mut f64,
        var_fn429_calc_ig__igindiode_slot: &mut f64,
        var_fn429_calc_ig__igindiode_dn13_slot: &mut f64,
        var_fn429_calc_ig__igindiode_dn4_slot: &mut f64,
        var_fn429_calc_ig__igindiode_dn8_slot: &mut f64,
        var_fn429_calc_ig__igindiode_hinj_slot: &mut f64,
        var_fn429_calc_ig__igindiode_hinj_dn13_slot: &mut f64,
        var_fn429_calc_ig__igindiode_hinj_dn4_slot: &mut f64,
        var_fn429_calc_ig__igindiode_hinj_dn8_slot: &mut f64,
        var_fn429_calc_ig__igindiode_hinj_pre_slot: &mut f64,
        var_fn429_calc_ig__igindiode_hinj_pre_dn4_slot: &mut f64,
        var_fn429_calc_ig__igindiode_hinj_vgsat_slot: &mut f64,
        var_fn429_calc_ig__igindiode_hinj_vgsat_dn4_slot: &mut f64,
        var_fn429_calc_ig__igindiode_nohinj_slot: &mut f64,
        var_fn429_calc_ig__igindiode_nohinj_dn13_slot: &mut f64,
        var_fn429_calc_ig__igindiode_nohinj_dn4_slot: &mut f64,
        var_fn429_calc_ig__igindiode_nohinj_dn8_slot: &mut f64,
        var_fn429_calc_ig__igindiode_nohinj_vgsat_slot: &mut f64,
        var_fn429_calc_ig__igindiode_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn429_calc_ig__isdiodeout_slot: &mut f64,
        var_fn429_calc_ig__isdiodeout_dn4_slot: &mut f64,
        var_fn429_calc_ig__pg_paramin_hinj_slot: &mut f64,
        var_guard430_slot: &mut f64,
        var_guard431_slot: &mut f64,
        var_guard432_slot: &mut f64,
        var_guard433_slot: &mut f64,
    ) {
        let mut var_fn429_calc_ig__alpha2_phit: f64 = *var_fn429_calc_ig__alpha2_phit_slot;
        let mut var_fn429_calc_ig__alpha2_phit_dn4: f64 = *var_fn429_calc_ig__alpha2_phit_dn4_slot;
        let mut var_fn429_calc_ig__expbd1_vgsat: f64 = *var_fn429_calc_ig__expbd1_vgsat_slot;
        let mut var_fn429_calc_ig__expbd1_vgsat_dn4: f64 = *var_fn429_calc_ig__expbd1_vgsat_dn4_slot;
        let mut var_fn429_calc_ig__expbdarg1_vgsat: f64 = *var_fn429_calc_ig__expbdarg1_vgsat_slot;
        let mut var_fn429_calc_ig__expbdarg1_vgsat_dn4: f64 = *var_fn429_calc_ig__expbdarg1_vgsat_dn4_slot;
        let mut var_fn429_calc_ig__expffvarg: f64 = *var_fn429_calc_ig__expffvarg_slot;
        let mut var_fn429_calc_ig__expffvarg_dn13: f64 = *var_fn429_calc_ig__expffvarg_dn13_slot;
        let mut var_fn429_calc_ig__expffvarg_dn4: f64 = *var_fn429_calc_ig__expffvarg_dn4_slot;
        let mut var_fn429_calc_ig__expffvarg_dn8: f64 = *var_fn429_calc_ig__expffvarg_dn8_slot;
        let mut var_fn429_calc_ig__expifor: f64 = *var_fn429_calc_ig__expifor_slot;
        let mut var_fn429_calc_ig__expifor_dn13: f64 = *var_fn429_calc_ig__expifor_dn13_slot;
        let mut var_fn429_calc_ig__expifor_dn4: f64 = *var_fn429_calc_ig__expifor_dn4_slot;
        let mut var_fn429_calc_ig__expifor_dn8: f64 = *var_fn429_calc_ig__expifor_dn8_slot;
        let mut var_fn429_calc_ig__expifor_hinj: f64 = *var_fn429_calc_ig__expifor_hinj_slot;
        let mut var_fn429_calc_ig__expifor_hinj_dn13: f64 = *var_fn429_calc_ig__expifor_hinj_dn13_slot;
        let mut var_fn429_calc_ig__expifor_hinj_dn4: f64 = *var_fn429_calc_ig__expifor_hinj_dn4_slot;
        let mut var_fn429_calc_ig__expifor_hinj_dn8: f64 = *var_fn429_calc_ig__expifor_hinj_dn8_slot;
        let mut var_fn429_calc_ig__expifor_hinj_vgsat: f64 = *var_fn429_calc_ig__expifor_hinj_vgsat_slot;
        let mut var_fn429_calc_ig__expifor_hinj_vgsat_dn4: f64 = *var_fn429_calc_ig__expifor_hinj_vgsat_dn4_slot;
        let mut var_fn429_calc_ig__expifor_nohinj_vgsat: f64 = *var_fn429_calc_ig__expifor_nohinj_vgsat_slot;
        let mut var_fn429_calc_ig__expifor_nohinj_vgsat_dn4: f64 = *var_fn429_calc_ig__expifor_nohinj_vgsat_dn4_slot;
        let mut var_fn429_calc_ig__expiforarg: f64 = *var_fn429_calc_ig__expiforarg_slot;
        let mut var_fn429_calc_ig__expiforarg_dn13: f64 = *var_fn429_calc_ig__expiforarg_dn13_slot;
        let mut var_fn429_calc_ig__expiforarg_dn4: f64 = *var_fn429_calc_ig__expiforarg_dn4_slot;
        let mut var_fn429_calc_ig__expiforarg_dn8: f64 = *var_fn429_calc_ig__expiforarg_dn8_slot;
        let mut var_fn429_calc_ig__expiforarg_hinj: f64 = *var_fn429_calc_ig__expiforarg_hinj_slot;
        let mut var_fn429_calc_ig__expiforarg_hinj_dn13: f64 = *var_fn429_calc_ig__expiforarg_hinj_dn13_slot;
        let mut var_fn429_calc_ig__expiforarg_hinj_dn4: f64 = *var_fn429_calc_ig__expiforarg_hinj_dn4_slot;
        let mut var_fn429_calc_ig__expiforarg_hinj_dn8: f64 = *var_fn429_calc_ig__expiforarg_hinj_dn8_slot;
        let mut var_fn429_calc_ig__expiforarg_hinj_vgsat: f64 = *var_fn429_calc_ig__expiforarg_hinj_vgsat_slot;
        let mut var_fn429_calc_ig__expiforarg_hinj_vgsat_dn4: f64 = *var_fn429_calc_ig__expiforarg_hinj_vgsat_dn4_slot;
        let mut var_fn429_calc_ig__expiforarg_nohinj_vgsat: f64 = *var_fn429_calc_ig__expiforarg_nohinj_vgsat_slot;
        let mut var_fn429_calc_ig__expiforarg_nohinj_vgsat_dn4: f64 = *var_fn429_calc_ig__expiforarg_nohinj_vgsat_dn4_slot;
        let mut var_fn429_calc_ig__ffvgin: f64 = *var_fn429_calc_ig__ffvgin_slot;
        let mut var_fn429_calc_ig__ffvgin_dn13: f64 = *var_fn429_calc_ig__ffvgin_dn13_slot;
        let mut var_fn429_calc_ig__ffvgin_dn4: f64 = *var_fn429_calc_ig__ffvgin_dn4_slot;
        let mut var_fn429_calc_ig__ffvgin_dn8: f64 = *var_fn429_calc_ig__ffvgin_dn8_slot;
        let mut var_fn429_calc_ig__iginbd_vgsat: f64 = *var_fn429_calc_ig__iginbd_vgsat_slot;
        let mut var_fn429_calc_ig__iginbd_vgsat_dn4: f64 = *var_fn429_calc_ig__iginbd_vgsat_dn4_slot;
        let mut var_fn429_calc_ig__igindiode: f64 = *var_fn429_calc_ig__igindiode_slot;
        let mut var_fn429_calc_ig__igindiode_dn13: f64 = *var_fn429_calc_ig__igindiode_dn13_slot;
        let mut var_fn429_calc_ig__igindiode_dn4: f64 = *var_fn429_calc_ig__igindiode_dn4_slot;
        let mut var_fn429_calc_ig__igindiode_dn8: f64 = *var_fn429_calc_ig__igindiode_dn8_slot;
        let mut var_fn429_calc_ig__igindiode_hinj: f64 = *var_fn429_calc_ig__igindiode_hinj_slot;
        let mut var_fn429_calc_ig__igindiode_hinj_dn13: f64 = *var_fn429_calc_ig__igindiode_hinj_dn13_slot;
        let mut var_fn429_calc_ig__igindiode_hinj_dn4: f64 = *var_fn429_calc_ig__igindiode_hinj_dn4_slot;
        let mut var_fn429_calc_ig__igindiode_hinj_dn8: f64 = *var_fn429_calc_ig__igindiode_hinj_dn8_slot;
        let mut var_fn429_calc_ig__igindiode_hinj_pre: f64 = *var_fn429_calc_ig__igindiode_hinj_pre_slot;
        let mut var_fn429_calc_ig__igindiode_hinj_pre_dn4: f64 = *var_fn429_calc_ig__igindiode_hinj_pre_dn4_slot;
        let mut var_fn429_calc_ig__igindiode_hinj_vgsat: f64 = *var_fn429_calc_ig__igindiode_hinj_vgsat_slot;
        let mut var_fn429_calc_ig__igindiode_hinj_vgsat_dn4: f64 = *var_fn429_calc_ig__igindiode_hinj_vgsat_dn4_slot;
        let mut var_fn429_calc_ig__igindiode_nohinj: f64 = *var_fn429_calc_ig__igindiode_nohinj_slot;
        let mut var_fn429_calc_ig__igindiode_nohinj_dn13: f64 = *var_fn429_calc_ig__igindiode_nohinj_dn13_slot;
        let mut var_fn429_calc_ig__igindiode_nohinj_dn4: f64 = *var_fn429_calc_ig__igindiode_nohinj_dn4_slot;
        let mut var_fn429_calc_ig__igindiode_nohinj_dn8: f64 = *var_fn429_calc_ig__igindiode_nohinj_dn8_slot;
        let mut var_fn429_calc_ig__igindiode_nohinj_vgsat: f64 = *var_fn429_calc_ig__igindiode_nohinj_vgsat_slot;
        let mut var_fn429_calc_ig__igindiode_nohinj_vgsat_dn4: f64 = *var_fn429_calc_ig__igindiode_nohinj_vgsat_dn4_slot;
        let mut var_fn429_calc_ig__isdiodeout: f64 = *var_fn429_calc_ig__isdiodeout_slot;
        let mut var_fn429_calc_ig__isdiodeout_dn4: f64 = *var_fn429_calc_ig__isdiodeout_dn4_slot;
        let mut var_fn429_calc_ig__pg_paramin_hinj: f64 = *var_fn429_calc_ig__pg_paramin_hinj_slot;
        let mut var_guard430: f64 = *var_guard430_slot;
        let mut var_guard431: f64 = *var_guard431_slot;
        let mut var_guard432: f64 = *var_guard432_slot;
        let mut var_guard433: f64 = *var_guard433_slot;

        let (assign35760_e32530, assign35760_e32530_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        let assign35760_e32522: f64 = (var_fn429_calc_ig__type * var_fn429_calc_ig__w);
        let assign35760_e32524: f64 = (assign35760_e32522 * var_fn429_calc_ig__ngf);
        let assign35760_e32526: f64 = (assign35760_e32524 * var_fn429_calc_ig__ijin);
        let assign35760_e32528: f64 = (assign35760_e32526 * var_fn429_calc_ig__tfacdiodein);
        (assign35760_e32528, (assign35760_e32526 * var_fn429_calc_ig__tfacdiodein_dn4),)
    } else {
        (var_fn429_calc_ig__isdiodeout, var_fn429_calc_ig__isdiodeout_dn4,)
    }
};
        var_fn429_calc_ig__isdiodeout = assign35760_e32530;
        var_fn429_calc_ig__isdiodeout_dn4 = assign35760_e32530_d_n4;

        let (assign35770_e32542, assign35770_e32542_d_n4, assign35770_e32542_d_n8, assign35770_e32542_d_n13,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        let assign35770_e32536: f64 = (var_fn429_calc_ig__pg_paramin / var_fn429_calc_ig__phitin);
        let assign35770_e32538: f64 = (assign35770_e32536 * var_fn429_calc_ig__vgin);
        let assign35770_e32540: f64 = (assign35770_e32538 + var_fn429_calc_ig__expphib);
        (assign35770_e32540, (((-((var_fn429_calc_ig__pg_paramin * var_fn429_calc_ig__phitin_dn4) / (var_fn429_calc_ig__phitin * var_fn429_calc_ig__phitin))) * var_fn429_calc_ig__vgin) + var_fn429_calc_ig__expphib_dn4), (assign35770_e32536 * var_fn429_calc_ig__vgin_dn8), (assign35770_e32536 * var_fn429_calc_ig__vgin_dn13),)
    } else {
        (var_fn429_calc_ig__expiforarg, var_fn429_calc_ig__expiforarg_dn4, var_fn429_calc_ig__expiforarg_dn8, var_fn429_calc_ig__expiforarg_dn13,)
    }
};
        var_fn429_calc_ig__expiforarg = assign35770_e32542;
        var_fn429_calc_ig__expiforarg_dn4 = assign35770_e32542_d_n4;
        var_fn429_calc_ig__expiforarg_dn8 = assign35770_e32542_d_n8;
        var_fn429_calc_ig__expiforarg_dn13 = assign35770_e32542_d_n13;

        let (assign35780_e32586, assign35780_e32586_d_n4, assign35780_e32586_d_n8, assign35780_e32586_d_n13,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        let assign35780_e32552: f64 = (-50.0);
        let (assign35780_e32584, assign35780_e32584_d_n4, assign35780_e32584_d_n8, assign35780_e32584_d_n13,) = {
            if ((!(var_fn429_calc_ig__expiforarg > 50.0)) && (!(var_fn429_calc_ig__expiforarg < assign35780_e32552))) {
                let assign35780_e32557: f64 = (var_fn429_calc_ig__expiforarg).exp();
                (assign35780_e32557, (assign35780_e32557 * var_fn429_calc_ig__expiforarg_dn4), (assign35780_e32557 * var_fn429_calc_ig__expiforarg_dn8), (assign35780_e32557 * var_fn429_calc_ig__expiforarg_dn13),)
            } else {
                let assign35780_e32564: f64 = (-50.0);
                let (assign35780_e32583, assign35780_e32583_d_n4, assign35780_e32583_d_n8, assign35780_e32583_d_n13,) = {
                    if ((!(var_fn429_calc_ig__expiforarg > 50.0)) && (var_fn429_calc_ig__expiforarg < assign35780_e32564)) {
                        let assign35780_e32568: f64 = (-50.0);
                        let assign35780_e32569: f64 = (assign35780_e32568).exp();
                        (assign35780_e32569, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign35780_e32582, assign35780_e32582_d_n4, assign35780_e32582_d_n8, assign35780_e32582_d_n13,) = {
                            if (var_fn429_calc_ig__expiforarg > 50.0) {
                                let assign35780_e32574: f64 = (50.0_f64).exp();
                                let assign35780_e32578: f64 = (var_fn429_calc_ig__expiforarg - 50.0);
                                let assign35780_e32579: f64 = (1.0 + assign35780_e32578);
                                let assign35780_e32580: f64 = (assign35780_e32574 * assign35780_e32579);
                                (assign35780_e32580, (assign35780_e32574 * var_fn429_calc_ig__expiforarg_dn4), (assign35780_e32574 * var_fn429_calc_ig__expiforarg_dn8), (assign35780_e32574 * var_fn429_calc_ig__expiforarg_dn13),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign35780_e32582, assign35780_e32582_d_n4, assign35780_e32582_d_n8, assign35780_e32582_d_n13,)
                    }
                };
                (assign35780_e32583, assign35780_e32583_d_n4, assign35780_e32583_d_n8, assign35780_e32583_d_n13,)
            }
        };
        (assign35780_e32584, assign35780_e32584_d_n4, assign35780_e32584_d_n8, assign35780_e32584_d_n13,)
    } else {
        (var_fn429_calc_ig__expifor, var_fn429_calc_ig__expifor_dn4, var_fn429_calc_ig__expifor_dn8, var_fn429_calc_ig__expifor_dn13,)
    }
};
        var_fn429_calc_ig__expifor = assign35780_e32586;
        var_fn429_calc_ig__expifor_dn4 = assign35780_e32586_d_n4;
        var_fn429_calc_ig__expifor_dn8 = assign35780_e32586_d_n8;
        var_fn429_calc_ig__expifor_dn13 = assign35780_e32586_d_n13;

        let assign35790_e32589: f64 = if var_fn429_calc_ig__fracin == 1.0 { 1.0 } else { 0.0 };
        var_guard430 = assign35790_e32589;

        let (assign35800_e32605, assign35800_e32605_d_n4, assign35800_e32605_d_n8, assign35800_e32605_d_n13,) = {
    if (((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard430 != 0.0)) {
        let assign35800_e32599: f64 = (var_fn429_calc_ig__kbdgatein * var_fn429_calc_ig__iginbd);
        let assign35800_e32600: f64 = (var_fn429_calc_ig__expifor - assign35800_e32599);
        let assign35800_e32602: f64 = (assign35800_e32600 - var_fn429_calc_ig__t0);
        let assign35800_e32603: f64 = (var_fn429_calc_ig__isdiodeout * assign35800_e32602);
        (assign35800_e32603, ((var_fn429_calc_ig__isdiodeout_dn4 * assign35800_e32602) + (var_fn429_calc_ig__isdiodeout * ((var_fn429_calc_ig__expifor_dn4 - (var_fn429_calc_ig__kbdgatein * var_fn429_calc_ig__iginbd_dn4)) - var_fn429_calc_ig__t0_dn4))), (var_fn429_calc_ig__isdiodeout * (var_fn429_calc_ig__expifor_dn8 - (var_fn429_calc_ig__kbdgatein * var_fn429_calc_ig__iginbd_dn8))), (var_fn429_calc_ig__isdiodeout * (var_fn429_calc_ig__expifor_dn13 - (var_fn429_calc_ig__kbdgatein * var_fn429_calc_ig__iginbd_dn13))),)
    } else {
        (var_fn429_calc_ig__igindiode, var_fn429_calc_ig__igindiode_dn4, var_fn429_calc_ig__igindiode_dn8, var_fn429_calc_ig__igindiode_dn13,)
    }
};
        var_fn429_calc_ig__igindiode = assign35800_e32605;
        var_fn429_calc_ig__igindiode_dn4 = assign35800_e32605_d_n4;
        var_fn429_calc_ig__igindiode_dn8 = assign35800_e32605_d_n8;
        var_fn429_calc_ig__igindiode_dn13 = assign35800_e32605_d_n13;

        let (assign35810_e32621, assign35810_e32621_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard430 == 0.0)) {
        let assign35810_e32614: f64 = (-var_fn429_calc_ig__vgsatin);
        let assign35810_e32616: f64 = (assign35810_e32614 - var_fn429_calc_ig__vbdgin);
        let assign35810_e32617: f64 = (var_fn429_calc_ig__pbdgin * assign35810_e32616);
        let assign35810_e32619: f64 = (assign35810_e32617 + var_fn429_calc_ig__expphib);
        (assign35810_e32619, var_fn429_calc_ig__expphib_dn4,)
    } else {
        (var_fn429_calc_ig__expbdarg1_vgsat, var_fn429_calc_ig__expbdarg1_vgsat_dn4,)
    }
};
        var_fn429_calc_ig__expbdarg1_vgsat = assign35810_e32621;
        var_fn429_calc_ig__expbdarg1_vgsat_dn4 = assign35810_e32621_d_n4;

        let (assign35820_e32668, assign35820_e32668_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard430 == 0.0)) {
        let assign35820_e32634: f64 = (-50.0);
        let (assign35820_e32666, assign35820_e32666_d_n4,) = {
            if ((!(var_fn429_calc_ig__expbdarg1_vgsat > 50.0)) && (!(var_fn429_calc_ig__expbdarg1_vgsat < assign35820_e32634))) {
                let assign35820_e32639: f64 = (var_fn429_calc_ig__expbdarg1_vgsat).exp();
                (assign35820_e32639, (assign35820_e32639 * var_fn429_calc_ig__expbdarg1_vgsat_dn4),)
            } else {
                let assign35820_e32646: f64 = (-50.0);
                let (assign35820_e32665, assign35820_e32665_d_n4,) = {
                    if ((!(var_fn429_calc_ig__expbdarg1_vgsat > 50.0)) && (var_fn429_calc_ig__expbdarg1_vgsat < assign35820_e32646)) {
                        let assign35820_e32650: f64 = (-50.0);
                        let assign35820_e32651: f64 = (assign35820_e32650).exp();
                        (assign35820_e32651, 0.0,)
                    } else {
                        let (assign35820_e32664, assign35820_e32664_d_n4,) = {
                            if (var_fn429_calc_ig__expbdarg1_vgsat > 50.0) {
                                let assign35820_e32656: f64 = (50.0_f64).exp();
                                let assign35820_e32660: f64 = (var_fn429_calc_ig__expbdarg1_vgsat - 50.0);
                                let assign35820_e32661: f64 = (1.0 + assign35820_e32660);
                                let assign35820_e32662: f64 = (assign35820_e32656 * assign35820_e32661);
                                (assign35820_e32662, (assign35820_e32656 * var_fn429_calc_ig__expbdarg1_vgsat_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign35820_e32664, assign35820_e32664_d_n4,)
                    }
                };
                (assign35820_e32665, assign35820_e32665_d_n4,)
            }
        };
        (assign35820_e32666, assign35820_e32666_d_n4,)
    } else {
        (var_fn429_calc_ig__expbd1_vgsat, var_fn429_calc_ig__expbd1_vgsat_dn4,)
    }
};
        var_fn429_calc_ig__expbd1_vgsat = assign35820_e32668;
        var_fn429_calc_ig__expbd1_vgsat_dn4 = assign35820_e32668_d_n4;

        let (assign35830_e32679, assign35830_e32679_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard430 == 0.0)) {
        let assign35830_e32677: f64 = (var_fn429_calc_ig__expbd1_vgsat - var_fn429_calc_ig__expbd2);
        (assign35830_e32677, (var_fn429_calc_ig__expbd1_vgsat_dn4 - var_fn429_calc_ig__expbd2_dn4),)
    } else {
        (var_fn429_calc_ig__iginbd_vgsat, var_fn429_calc_ig__iginbd_vgsat_dn4,)
    }
};
        var_fn429_calc_ig__iginbd_vgsat = assign35830_e32679;
        var_fn429_calc_ig__iginbd_vgsat_dn4 = assign35830_e32679_d_n4;

        let (assign35840_e32694, assign35840_e32694_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard430 == 0.0)) {
        let assign35840_e32688: f64 = (var_fn429_calc_ig__pg_paramin / var_fn429_calc_ig__phitin);
        let assign35840_e32690: f64 = (assign35840_e32688 * var_fn429_calc_ig__vgsatin);
        let assign35840_e32692: f64 = (assign35840_e32690 + var_fn429_calc_ig__expphib);
        (assign35840_e32692, (((-((var_fn429_calc_ig__pg_paramin * var_fn429_calc_ig__phitin_dn4) / (var_fn429_calc_ig__phitin * var_fn429_calc_ig__phitin))) * var_fn429_calc_ig__vgsatin) + var_fn429_calc_ig__expphib_dn4),)
    } else {
        (var_fn429_calc_ig__expiforarg_nohinj_vgsat, var_fn429_calc_ig__expiforarg_nohinj_vgsat_dn4,)
    }
};
        var_fn429_calc_ig__expiforarg_nohinj_vgsat = assign35840_e32694;
        var_fn429_calc_ig__expiforarg_nohinj_vgsat_dn4 = assign35840_e32694_d_n4;

        let (assign35850_e32741, assign35850_e32741_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard430 == 0.0)) {
        let assign35850_e32707: f64 = (-50.0);
        let (assign35850_e32739, assign35850_e32739_d_n4,) = {
            if ((!(var_fn429_calc_ig__expiforarg_nohinj_vgsat > 50.0)) && (!(var_fn429_calc_ig__expiforarg_nohinj_vgsat < assign35850_e32707))) {
                let assign35850_e32712: f64 = (var_fn429_calc_ig__expiforarg_nohinj_vgsat).exp();
                (assign35850_e32712, (assign35850_e32712 * var_fn429_calc_ig__expiforarg_nohinj_vgsat_dn4),)
            } else {
                let assign35850_e32719: f64 = (-50.0);
                let (assign35850_e32738, assign35850_e32738_d_n4,) = {
                    if ((!(var_fn429_calc_ig__expiforarg_nohinj_vgsat > 50.0)) && (var_fn429_calc_ig__expiforarg_nohinj_vgsat < assign35850_e32719)) {
                        let assign35850_e32723: f64 = (-50.0);
                        let assign35850_e32724: f64 = (assign35850_e32723).exp();
                        (assign35850_e32724, 0.0,)
                    } else {
                        let (assign35850_e32737, assign35850_e32737_d_n4,) = {
                            if (var_fn429_calc_ig__expiforarg_nohinj_vgsat > 50.0) {
                                let assign35850_e32729: f64 = (50.0_f64).exp();
                                let assign35850_e32733: f64 = (var_fn429_calc_ig__expiforarg_nohinj_vgsat - 50.0);
                                let assign35850_e32734: f64 = (1.0 + assign35850_e32733);
                                let assign35850_e32735: f64 = (assign35850_e32729 * assign35850_e32734);
                                (assign35850_e32735, (assign35850_e32729 * var_fn429_calc_ig__expiforarg_nohinj_vgsat_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign35850_e32737, assign35850_e32737_d_n4,)
                    }
                };
                (assign35850_e32738, assign35850_e32738_d_n4,)
            }
        };
        (assign35850_e32739, assign35850_e32739_d_n4,)
    } else {
        (var_fn429_calc_ig__expifor_nohinj_vgsat, var_fn429_calc_ig__expifor_nohinj_vgsat_dn4,)
    }
};
        var_fn429_calc_ig__expifor_nohinj_vgsat = assign35850_e32741;
        var_fn429_calc_ig__expifor_nohinj_vgsat_dn4 = assign35850_e32741_d_n4;

        let (assign35860_e32756, assign35860_e32756_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard430 == 0.0)) {
        let assign35860_e32751: f64 = (var_fn429_calc_ig__kbdgatein * var_fn429_calc_ig__iginbd_vgsat);
        let assign35860_e32752: f64 = (var_fn429_calc_ig__expifor_nohinj_vgsat - assign35860_e32751);
        let assign35860_e32754: f64 = (assign35860_e32752 - var_fn429_calc_ig__t0);
        (assign35860_e32754, ((var_fn429_calc_ig__expifor_nohinj_vgsat_dn4 - (var_fn429_calc_ig__kbdgatein * var_fn429_calc_ig__iginbd_vgsat_dn4)) - var_fn429_calc_ig__t0_dn4),)
    } else {
        (var_fn429_calc_ig__igindiode_nohinj_vgsat, var_fn429_calc_ig__igindiode_nohinj_vgsat_dn4,)
    }
};
        var_fn429_calc_ig__igindiode_nohinj_vgsat = assign35860_e32756;
        var_fn429_calc_ig__igindiode_nohinj_vgsat_dn4 = assign35860_e32756_d_n4;

        let (assign35870_e32773, assign35870_e32773_d_n4, assign35870_e32773_d_n8, assign35870_e32773_d_n13,) = {
    if (((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard430 == 0.0)) {
        let assign35870_e32767: f64 = (var_fn429_calc_ig__kbdgatein * var_fn429_calc_ig__iginbd);
        let assign35870_e32768: f64 = (var_fn429_calc_ig__expifor - assign35870_e32767);
        let assign35870_e32770: f64 = (assign35870_e32768 - var_fn429_calc_ig__t0);
        let assign35870_e32771: f64 = (var_fn429_calc_ig__isdiodeout * assign35870_e32770);
        (assign35870_e32771, ((var_fn429_calc_ig__isdiodeout_dn4 * assign35870_e32770) + (var_fn429_calc_ig__isdiodeout * ((var_fn429_calc_ig__expifor_dn4 - (var_fn429_calc_ig__kbdgatein * var_fn429_calc_ig__iginbd_dn4)) - var_fn429_calc_ig__t0_dn4))), (var_fn429_calc_ig__isdiodeout * (var_fn429_calc_ig__expifor_dn8 - (var_fn429_calc_ig__kbdgatein * var_fn429_calc_ig__iginbd_dn8))), (var_fn429_calc_ig__isdiodeout * (var_fn429_calc_ig__expifor_dn13 - (var_fn429_calc_ig__kbdgatein * var_fn429_calc_ig__iginbd_dn13))),)
    } else {
        (var_fn429_calc_ig__igindiode_nohinj, var_fn429_calc_ig__igindiode_nohinj_dn4, var_fn429_calc_ig__igindiode_nohinj_dn8, var_fn429_calc_ig__igindiode_nohinj_dn13,)
    }
};
        var_fn429_calc_ig__igindiode_nohinj = assign35870_e32773;
        var_fn429_calc_ig__igindiode_nohinj_dn4 = assign35870_e32773_d_n4;
        var_fn429_calc_ig__igindiode_nohinj_dn8 = assign35870_e32773_d_n8;
        var_fn429_calc_ig__igindiode_nohinj_dn13 = assign35870_e32773_d_n13;

        let assign35880_e32776: f64 = if var_fn429_calc_ig__fracin > 0.0 { 1.0 } else { 0.0 };
        var_guard431 = assign35880_e32776;

        let (assign35890_e32789,) = {
    if ((((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard430 == 0.0)) && (var_guard431 != 0.0)) {
        let assign35890_e32787: f64 = (var_fn429_calc_ig__fracin * var_fn429_calc_ig__pg_paramin);
        (assign35890_e32787,)
    } else {
        (var_fn429_calc_ig__pg_paramin_hinj,)
    }
};
        var_fn429_calc_ig__pg_paramin_hinj = assign35890_e32789;

        let (assign35900_e32806, assign35900_e32806_d_n4,) = {
    if ((((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard430 == 0.0)) && (var_guard431 != 0.0)) {
        let assign35900_e32800: f64 = (var_fn429_calc_ig__pg_paramin_hinj / var_fn429_calc_ig__phitin);
        let assign35900_e32802: f64 = (assign35900_e32800 * var_fn429_calc_ig__vgsatin);
        let assign35900_e32804: f64 = (assign35900_e32802 + var_fn429_calc_ig__expphib);
        (assign35900_e32804, (((-((var_fn429_calc_ig__pg_paramin_hinj * var_fn429_calc_ig__phitin_dn4) / (var_fn429_calc_ig__phitin * var_fn429_calc_ig__phitin))) * var_fn429_calc_ig__vgsatin) + var_fn429_calc_ig__expphib_dn4),)
    } else {
        (var_fn429_calc_ig__expiforarg_hinj_vgsat, var_fn429_calc_ig__expiforarg_hinj_vgsat_dn4,)
    }
};
        var_fn429_calc_ig__expiforarg_hinj_vgsat = assign35900_e32806;
        var_fn429_calc_ig__expiforarg_hinj_vgsat_dn4 = assign35900_e32806_d_n4;

        let (assign35910_e32855, assign35910_e32855_d_n4,) = {
    if ((((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard430 == 0.0)) && (var_guard431 != 0.0)) {
        let assign35910_e32821: f64 = (-50.0);
        let (assign35910_e32853, assign35910_e32853_d_n4,) = {
            if ((!(var_fn429_calc_ig__expiforarg_hinj_vgsat > 50.0)) && (!(var_fn429_calc_ig__expiforarg_hinj_vgsat < assign35910_e32821))) {
                let assign35910_e32826: f64 = (var_fn429_calc_ig__expiforarg_hinj_vgsat).exp();
                (assign35910_e32826, (assign35910_e32826 * var_fn429_calc_ig__expiforarg_hinj_vgsat_dn4),)
            } else {
                let assign35910_e32833: f64 = (-50.0);
                let (assign35910_e32852, assign35910_e32852_d_n4,) = {
                    if ((!(var_fn429_calc_ig__expiforarg_hinj_vgsat > 50.0)) && (var_fn429_calc_ig__expiforarg_hinj_vgsat < assign35910_e32833)) {
                        let assign35910_e32837: f64 = (-50.0);
                        let assign35910_e32838: f64 = (assign35910_e32837).exp();
                        (assign35910_e32838, 0.0,)
                    } else {
                        let (assign35910_e32851, assign35910_e32851_d_n4,) = {
                            if (var_fn429_calc_ig__expiforarg_hinj_vgsat > 50.0) {
                                let assign35910_e32843: f64 = (50.0_f64).exp();
                                let assign35910_e32847: f64 = (var_fn429_calc_ig__expiforarg_hinj_vgsat - 50.0);
                                let assign35910_e32848: f64 = (1.0 + assign35910_e32847);
                                let assign35910_e32849: f64 = (assign35910_e32843 * assign35910_e32848);
                                (assign35910_e32849, (assign35910_e32843 * var_fn429_calc_ig__expiforarg_hinj_vgsat_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign35910_e32851, assign35910_e32851_d_n4,)
                    }
                };
                (assign35910_e32852, assign35910_e32852_d_n4,)
            }
        };
        (assign35910_e32853, assign35910_e32853_d_n4,)
    } else {
        (var_fn429_calc_ig__expifor_hinj_vgsat, var_fn429_calc_ig__expifor_hinj_vgsat_dn4,)
    }
};
        var_fn429_calc_ig__expifor_hinj_vgsat = assign35910_e32855;
        var_fn429_calc_ig__expifor_hinj_vgsat_dn4 = assign35910_e32855_d_n4;

        let (assign35920_e32872, assign35920_e32872_d_n4,) = {
    if ((((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard430 == 0.0)) && (var_guard431 != 0.0)) {
        let assign35920_e32867: f64 = (var_fn429_calc_ig__kbdgatein * var_fn429_calc_ig__iginbd_vgsat);
        let assign35920_e32868: f64 = (var_fn429_calc_ig__expifor_hinj_vgsat - assign35920_e32867);
        let assign35920_e32870: f64 = (assign35920_e32868 - var_fn429_calc_ig__t0);
        (assign35920_e32870, ((var_fn429_calc_ig__expifor_hinj_vgsat_dn4 - (var_fn429_calc_ig__kbdgatein * var_fn429_calc_ig__iginbd_vgsat_dn4)) - var_fn429_calc_ig__t0_dn4),)
    } else {
        (var_fn429_calc_ig__igindiode_hinj_vgsat, var_fn429_calc_ig__igindiode_hinj_vgsat_dn4,)
    }
};
        var_fn429_calc_ig__igindiode_hinj_vgsat = assign35920_e32872;
        var_fn429_calc_ig__igindiode_hinj_vgsat_dn4 = assign35920_e32872_d_n4;

        let (assign35930_e32889, assign35930_e32889_d_n4, assign35930_e32889_d_n8, assign35930_e32889_d_n13,) = {
    if ((((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard430 == 0.0)) && (var_guard431 != 0.0)) {
        let assign35930_e32883: f64 = (var_fn429_calc_ig__pg_paramin_hinj / var_fn429_calc_ig__phitin);
        let assign35930_e32885: f64 = (assign35930_e32883 * var_fn429_calc_ig__vgin);
        let assign35930_e32887: f64 = (assign35930_e32885 + var_fn429_calc_ig__expphib);
        (assign35930_e32887, (((-((var_fn429_calc_ig__pg_paramin_hinj * var_fn429_calc_ig__phitin_dn4) / (var_fn429_calc_ig__phitin * var_fn429_calc_ig__phitin))) * var_fn429_calc_ig__vgin) + var_fn429_calc_ig__expphib_dn4), (assign35930_e32883 * var_fn429_calc_ig__vgin_dn8), (assign35930_e32883 * var_fn429_calc_ig__vgin_dn13),)
    } else {
        (var_fn429_calc_ig__expiforarg_hinj, var_fn429_calc_ig__expiforarg_hinj_dn4, var_fn429_calc_ig__expiforarg_hinj_dn8, var_fn429_calc_ig__expiforarg_hinj_dn13,)
    }
};
        var_fn429_calc_ig__expiforarg_hinj = assign35930_e32889;
        var_fn429_calc_ig__expiforarg_hinj_dn4 = assign35930_e32889_d_n4;
        var_fn429_calc_ig__expiforarg_hinj_dn8 = assign35930_e32889_d_n8;
        var_fn429_calc_ig__expiforarg_hinj_dn13 = assign35930_e32889_d_n13;

        let (assign35940_e32938, assign35940_e32938_d_n4, assign35940_e32938_d_n8, assign35940_e32938_d_n13,) = {
    if ((((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard430 == 0.0)) && (var_guard431 != 0.0)) {
        let assign35940_e32904: f64 = (-50.0);
        let (assign35940_e32936, assign35940_e32936_d_n4, assign35940_e32936_d_n8, assign35940_e32936_d_n13,) = {
            if ((!(var_fn429_calc_ig__expiforarg_hinj > 50.0)) && (!(var_fn429_calc_ig__expiforarg_hinj < assign35940_e32904))) {
                let assign35940_e32909: f64 = (var_fn429_calc_ig__expiforarg_hinj).exp();
                (assign35940_e32909, (assign35940_e32909 * var_fn429_calc_ig__expiforarg_hinj_dn4), (assign35940_e32909 * var_fn429_calc_ig__expiforarg_hinj_dn8), (assign35940_e32909 * var_fn429_calc_ig__expiforarg_hinj_dn13),)
            } else {
                let assign35940_e32916: f64 = (-50.0);
                let (assign35940_e32935, assign35940_e32935_d_n4, assign35940_e32935_d_n8, assign35940_e32935_d_n13,) = {
                    if ((!(var_fn429_calc_ig__expiforarg_hinj > 50.0)) && (var_fn429_calc_ig__expiforarg_hinj < assign35940_e32916)) {
                        let assign35940_e32920: f64 = (-50.0);
                        let assign35940_e32921: f64 = (assign35940_e32920).exp();
                        (assign35940_e32921, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign35940_e32934, assign35940_e32934_d_n4, assign35940_e32934_d_n8, assign35940_e32934_d_n13,) = {
                            if (var_fn429_calc_ig__expiforarg_hinj > 50.0) {
                                let assign35940_e32926: f64 = (50.0_f64).exp();
                                let assign35940_e32930: f64 = (var_fn429_calc_ig__expiforarg_hinj - 50.0);
                                let assign35940_e32931: f64 = (1.0 + assign35940_e32930);
                                let assign35940_e32932: f64 = (assign35940_e32926 * assign35940_e32931);
                                (assign35940_e32932, (assign35940_e32926 * var_fn429_calc_ig__expiforarg_hinj_dn4), (assign35940_e32926 * var_fn429_calc_ig__expiforarg_hinj_dn8), (assign35940_e32926 * var_fn429_calc_ig__expiforarg_hinj_dn13),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign35940_e32934, assign35940_e32934_d_n4, assign35940_e32934_d_n8, assign35940_e32934_d_n13,)
                    }
                };
                (assign35940_e32935, assign35940_e32935_d_n4, assign35940_e32935_d_n8, assign35940_e32935_d_n13,)
            }
        };
        (assign35940_e32936, assign35940_e32936_d_n4, assign35940_e32936_d_n8, assign35940_e32936_d_n13,)
    } else {
        (var_fn429_calc_ig__expifor_hinj, var_fn429_calc_ig__expifor_hinj_dn4, var_fn429_calc_ig__expifor_hinj_dn8, var_fn429_calc_ig__expifor_hinj_dn13,)
    }
};
        var_fn429_calc_ig__expifor_hinj = assign35940_e32938;
        var_fn429_calc_ig__expifor_hinj_dn4 = assign35940_e32938_d_n4;
        var_fn429_calc_ig__expifor_hinj_dn8 = assign35940_e32938_d_n8;
        var_fn429_calc_ig__expifor_hinj_dn13 = assign35940_e32938_d_n13;

        let (assign35950_e32953, assign35950_e32953_d_n4,) = {
    if ((((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard430 == 0.0)) && (var_guard431 != 0.0)) {
        let assign35950_e32949: f64 = (var_fn429_calc_ig__isdiodeout * var_fn429_calc_ig__igindiode_nohinj_vgsat);
        let assign35950_e32951: f64 = (assign35950_e32949 / var_fn429_calc_ig__igindiode_hinj_vgsat);
        (assign35950_e32951, (((((var_fn429_calc_ig__isdiodeout_dn4 * var_fn429_calc_ig__igindiode_nohinj_vgsat) + (var_fn429_calc_ig__isdiodeout * var_fn429_calc_ig__igindiode_nohinj_vgsat_dn4)) * var_fn429_calc_ig__igindiode_hinj_vgsat) - (assign35950_e32949 * var_fn429_calc_ig__igindiode_hinj_vgsat_dn4)) / (var_fn429_calc_ig__igindiode_hinj_vgsat * var_fn429_calc_ig__igindiode_hinj_vgsat)),)
    } else {
        (var_fn429_calc_ig__igindiode_hinj_pre, var_fn429_calc_ig__igindiode_hinj_pre_dn4,)
    }
};
        var_fn429_calc_ig__igindiode_hinj_pre = assign35950_e32953;
        var_fn429_calc_ig__igindiode_hinj_pre_dn4 = assign35950_e32953_d_n4;

        let (assign35960_e32972, assign35960_e32972_d_n4, assign35960_e32972_d_n8, assign35960_e32972_d_n13,) = {
    if ((((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard430 == 0.0)) && (var_guard431 != 0.0)) {
        let assign35960_e32966: f64 = (var_fn429_calc_ig__kbdgatein * var_fn429_calc_ig__iginbd);
        let assign35960_e32967: f64 = (var_fn429_calc_ig__expifor_hinj - assign35960_e32966);
        let assign35960_e32969: f64 = (assign35960_e32967 - var_fn429_calc_ig__t0);
        let assign35960_e32970: f64 = (var_fn429_calc_ig__igindiode_hinj_pre * assign35960_e32969);
        (assign35960_e32970, ((var_fn429_calc_ig__igindiode_hinj_pre_dn4 * assign35960_e32969) + (var_fn429_calc_ig__igindiode_hinj_pre * ((var_fn429_calc_ig__expifor_hinj_dn4 - (var_fn429_calc_ig__kbdgatein * var_fn429_calc_ig__iginbd_dn4)) - var_fn429_calc_ig__t0_dn4))), (var_fn429_calc_ig__igindiode_hinj_pre * (var_fn429_calc_ig__expifor_hinj_dn8 - (var_fn429_calc_ig__kbdgatein * var_fn429_calc_ig__iginbd_dn8))), (var_fn429_calc_ig__igindiode_hinj_pre * (var_fn429_calc_ig__expifor_hinj_dn13 - (var_fn429_calc_ig__kbdgatein * var_fn429_calc_ig__iginbd_dn13))),)
    } else {
        (var_fn429_calc_ig__igindiode_hinj, var_fn429_calc_ig__igindiode_hinj_dn4, var_fn429_calc_ig__igindiode_hinj_dn8, var_fn429_calc_ig__igindiode_hinj_dn13,)
    }
};
        var_fn429_calc_ig__igindiode_hinj = assign35960_e32972;
        var_fn429_calc_ig__igindiode_hinj_dn4 = assign35960_e32972_d_n4;
        var_fn429_calc_ig__igindiode_hinj_dn8 = assign35960_e32972_d_n8;
        var_fn429_calc_ig__igindiode_hinj_dn13 = assign35960_e32972_d_n13;

        let (assign35970_e32986, assign35970_e32986_d_n4, assign35970_e32986_d_n8, assign35970_e32986_d_n13,) = {
    if ((((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard430 == 0.0)) && (var_guard431 == 0.0)) {
        let assign35970_e32984: f64 = (var_fn429_calc_ig__isdiodeout * var_fn429_calc_ig__igindiode_nohinj_vgsat);
        (assign35970_e32984, ((var_fn429_calc_ig__isdiodeout_dn4 * var_fn429_calc_ig__igindiode_nohinj_vgsat) + (var_fn429_calc_ig__isdiodeout * var_fn429_calc_ig__igindiode_nohinj_vgsat_dn4)), 0.0, 0.0,)
    } else {
        (var_fn429_calc_ig__igindiode_hinj, var_fn429_calc_ig__igindiode_hinj_dn4, var_fn429_calc_ig__igindiode_hinj_dn8, var_fn429_calc_ig__igindiode_hinj_dn13,)
    }
};
        var_fn429_calc_ig__igindiode_hinj = assign35970_e32986;
        var_fn429_calc_ig__igindiode_hinj_dn4 = assign35970_e32986_d_n4;
        var_fn429_calc_ig__igindiode_hinj_dn8 = assign35970_e32986_d_n8;
        var_fn429_calc_ig__igindiode_hinj_dn13 = assign35970_e32986_d_n13;

        let (assign35980_e32999, assign35980_e32999_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard430 == 0.0)) {
        let assign35980_e32995: f64 = (var_fn429_calc_ig__alphagin * var_fn429_calc_ig__alphagin);
        let assign35980_e32997: f64 = (assign35980_e32995 * var_fn429_calc_ig__phitin);
        (assign35980_e32997, (assign35980_e32995 * var_fn429_calc_ig__phitin_dn4),)
    } else {
        (var_fn429_calc_ig__alpha2_phit, var_fn429_calc_ig__alpha2_phit_dn4,)
    }
};
        var_fn429_calc_ig__alpha2_phit = assign35980_e32999;
        var_fn429_calc_ig__alpha2_phit_dn4 = assign35980_e32999_d_n4;

        let (assign35990_e33016, assign35990_e33016_d_n4, assign35990_e33016_d_n8, assign35990_e33016_d_n13,) = {
    if (((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard430 == 0.0)) {
        let assign35990_e33010: f64 = (var_fn429_calc_ig__alpha2_phit / 2.0);
        let assign35990_e33011: f64 = (var_fn429_calc_ig__vgsatin - assign35990_e33010);
        let assign35990_e33012: f64 = (var_fn429_calc_ig__vgin - assign35990_e33011);
        let assign35990_e33014: f64 = (assign35990_e33012 / var_fn429_calc_ig__alpha2_phit);
        (assign35990_e33014, ((((-(-(var_fn429_calc_ig__alpha2_phit_dn4 / 2.0))) * var_fn429_calc_ig__alpha2_phit) - (assign35990_e33012 * var_fn429_calc_ig__alpha2_phit_dn4)) / (var_fn429_calc_ig__alpha2_phit * var_fn429_calc_ig__alpha2_phit)), (var_fn429_calc_ig__vgin_dn8 / var_fn429_calc_ig__alpha2_phit), (var_fn429_calc_ig__vgin_dn13 / var_fn429_calc_ig__alpha2_phit),)
    } else {
        (var_fn429_calc_ig__expffvarg, var_fn429_calc_ig__expffvarg_dn4, var_fn429_calc_ig__expffvarg_dn8, var_fn429_calc_ig__expffvarg_dn13,)
    }
};
        var_fn429_calc_ig__expffvarg = assign35990_e33016;
        var_fn429_calc_ig__expffvarg_dn4 = assign35990_e33016_d_n4;
        var_fn429_calc_ig__expffvarg_dn8 = assign35990_e33016_d_n8;
        var_fn429_calc_ig__expffvarg_dn13 = assign35990_e33016_d_n13;

        let assign36000_e33019: f64 = if var_fn429_calc_ig__expffvarg > 50.0 { 1.0 } else { 0.0 };
        var_guard432 = assign36000_e33019;

        let (assign36010_e33030, assign36010_e33030_d_n4, assign36010_e33030_d_n8, assign36010_e33030_d_n13,) = {
    if ((((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard430 == 0.0)) && (var_guard432 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn429_calc_ig__ffvgin, var_fn429_calc_ig__ffvgin_dn4, var_fn429_calc_ig__ffvgin_dn8, var_fn429_calc_ig__ffvgin_dn13,)
    }
};
        var_fn429_calc_ig__ffvgin = assign36010_e33030;
        var_fn429_calc_ig__ffvgin_dn4 = assign36010_e33030_d_n4;
        var_fn429_calc_ig__ffvgin_dn8 = assign36010_e33030_d_n8;
        var_fn429_calc_ig__ffvgin_dn13 = assign36010_e33030_d_n13;

        let assign36020_e33033: f64 = (-50.0);
        let assign36020_e33034: f64 = if var_fn429_calc_ig__expffvarg < assign36020_e33033 { 1.0 } else { 0.0 };
        var_guard433 = assign36020_e33034;

        let (assign36030_e33048, assign36030_e33048_d_n4, assign36030_e33048_d_n8, assign36030_e33048_d_n13,) = {
    if (((((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard430 == 0.0)) && (var_guard432 == 0.0)) && (var_guard433 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn429_calc_ig__ffvgin, var_fn429_calc_ig__ffvgin_dn4, var_fn429_calc_ig__ffvgin_dn8, var_fn429_calc_ig__ffvgin_dn13,)
    }
};
        var_fn429_calc_ig__ffvgin = assign36030_e33048;
        var_fn429_calc_ig__ffvgin_dn4 = assign36030_e33048_d_n4;
        var_fn429_calc_ig__ffvgin_dn8 = assign36030_e33048_d_n8;
        var_fn429_calc_ig__ffvgin_dn13 = assign36030_e33048_d_n13;

        let (assign36040_e33068, assign36040_e33068_d_n4, assign36040_e33068_d_n8, assign36040_e33068_d_n13,) = {
    if (((((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard430 == 0.0)) && (var_guard432 == 0.0)) && (var_guard433 == 0.0)) {
        let assign36040_e33064: f64 = (var_fn429_calc_ig__expffvarg).exp();
        let assign36040_e33065: f64 = (1.0 + assign36040_e33064);
        let assign36040_e33066: f64 = (1.0 / assign36040_e33065);
        (assign36040_e33066, (-((assign36040_e33064 * var_fn429_calc_ig__expffvarg_dn4) / (assign36040_e33065 * assign36040_e33065))), (-((assign36040_e33064 * var_fn429_calc_ig__expffvarg_dn8) / (assign36040_e33065 * assign36040_e33065))), (-((assign36040_e33064 * var_fn429_calc_ig__expffvarg_dn13) / (assign36040_e33065 * assign36040_e33065))),)
    } else {
        (var_fn429_calc_ig__ffvgin, var_fn429_calc_ig__ffvgin_dn4, var_fn429_calc_ig__ffvgin_dn8, var_fn429_calc_ig__ffvgin_dn13,)
    }
};
        var_fn429_calc_ig__ffvgin = assign36040_e33068;
        var_fn429_calc_ig__ffvgin_dn4 = assign36040_e33068_d_n4;
        var_fn429_calc_ig__ffvgin_dn8 = assign36040_e33068_d_n8;
        var_fn429_calc_ig__ffvgin_dn13 = assign36040_e33068_d_n13;

        let (assign36050_e33085, assign36050_e33085_d_n4, assign36050_e33085_d_n8, assign36050_e33085_d_n13,) = {
    if (((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard430 == 0.0)) {
        let assign36050_e33077: f64 = (var_fn429_calc_ig__ffvgin * var_fn429_calc_ig__igindiode_nohinj);
        let assign36050_e33080: f64 = (1.0 - var_fn429_calc_ig__ffvgin);
        let assign36050_e33082: f64 = (assign36050_e33080 * var_fn429_calc_ig__igindiode_hinj);
        let assign36050_e33083: f64 = (assign36050_e33077 + assign36050_e33082);
        (assign36050_e33083, (((var_fn429_calc_ig__ffvgin_dn4 * var_fn429_calc_ig__igindiode_nohinj) + (var_fn429_calc_ig__ffvgin * var_fn429_calc_ig__igindiode_nohinj_dn4)) + (((-var_fn429_calc_ig__ffvgin_dn4) * var_fn429_calc_ig__igindiode_hinj) + (assign36050_e33080 * var_fn429_calc_ig__igindiode_hinj_dn4))), (((var_fn429_calc_ig__ffvgin_dn8 * var_fn429_calc_ig__igindiode_nohinj) + (var_fn429_calc_ig__ffvgin * var_fn429_calc_ig__igindiode_nohinj_dn8)) + (((-var_fn429_calc_ig__ffvgin_dn8) * var_fn429_calc_ig__igindiode_hinj) + (assign36050_e33080 * var_fn429_calc_ig__igindiode_hinj_dn8))), (((var_fn429_calc_ig__ffvgin_dn13 * var_fn429_calc_ig__igindiode_nohinj) + (var_fn429_calc_ig__ffvgin * var_fn429_calc_ig__igindiode_nohinj_dn13)) + (((-var_fn429_calc_ig__ffvgin_dn13) * var_fn429_calc_ig__igindiode_hinj) + (assign36050_e33080 * var_fn429_calc_ig__igindiode_hinj_dn13))),)
    } else {
        (var_fn429_calc_ig__igindiode, var_fn429_calc_ig__igindiode_dn4, var_fn429_calc_ig__igindiode_dn8, var_fn429_calc_ig__igindiode_dn13,)
    }
};
        var_fn429_calc_ig__igindiode = assign36050_e33085;
        var_fn429_calc_ig__igindiode_dn4 = assign36050_e33085_d_n4;
        var_fn429_calc_ig__igindiode_dn8 = assign36050_e33085_d_n8;
        var_fn429_calc_ig__igindiode_dn13 = assign36050_e33085_d_n13;

        *var_fn429_calc_ig__alpha2_phit_slot = var_fn429_calc_ig__alpha2_phit;
        *var_fn429_calc_ig__alpha2_phit_dn4_slot = var_fn429_calc_ig__alpha2_phit_dn4;
        *var_fn429_calc_ig__expbd1_vgsat_slot = var_fn429_calc_ig__expbd1_vgsat;
        *var_fn429_calc_ig__expbd1_vgsat_dn4_slot = var_fn429_calc_ig__expbd1_vgsat_dn4;
        *var_fn429_calc_ig__expbdarg1_vgsat_slot = var_fn429_calc_ig__expbdarg1_vgsat;
        *var_fn429_calc_ig__expbdarg1_vgsat_dn4_slot = var_fn429_calc_ig__expbdarg1_vgsat_dn4;
        *var_fn429_calc_ig__expffvarg_slot = var_fn429_calc_ig__expffvarg;
        *var_fn429_calc_ig__expffvarg_dn13_slot = var_fn429_calc_ig__expffvarg_dn13;
        *var_fn429_calc_ig__expffvarg_dn4_slot = var_fn429_calc_ig__expffvarg_dn4;
        *var_fn429_calc_ig__expffvarg_dn8_slot = var_fn429_calc_ig__expffvarg_dn8;
        *var_fn429_calc_ig__expifor_slot = var_fn429_calc_ig__expifor;
        *var_fn429_calc_ig__expifor_dn13_slot = var_fn429_calc_ig__expifor_dn13;
        *var_fn429_calc_ig__expifor_dn4_slot = var_fn429_calc_ig__expifor_dn4;
        *var_fn429_calc_ig__expifor_dn8_slot = var_fn429_calc_ig__expifor_dn8;
        *var_fn429_calc_ig__expifor_hinj_slot = var_fn429_calc_ig__expifor_hinj;
        *var_fn429_calc_ig__expifor_hinj_dn13_slot = var_fn429_calc_ig__expifor_hinj_dn13;
        *var_fn429_calc_ig__expifor_hinj_dn4_slot = var_fn429_calc_ig__expifor_hinj_dn4;
        *var_fn429_calc_ig__expifor_hinj_dn8_slot = var_fn429_calc_ig__expifor_hinj_dn8;
        *var_fn429_calc_ig__expifor_hinj_vgsat_slot = var_fn429_calc_ig__expifor_hinj_vgsat;
        *var_fn429_calc_ig__expifor_hinj_vgsat_dn4_slot = var_fn429_calc_ig__expifor_hinj_vgsat_dn4;
        *var_fn429_calc_ig__expifor_nohinj_vgsat_slot = var_fn429_calc_ig__expifor_nohinj_vgsat;
        *var_fn429_calc_ig__expifor_nohinj_vgsat_dn4_slot = var_fn429_calc_ig__expifor_nohinj_vgsat_dn4;
        *var_fn429_calc_ig__expiforarg_slot = var_fn429_calc_ig__expiforarg;
        *var_fn429_calc_ig__expiforarg_dn13_slot = var_fn429_calc_ig__expiforarg_dn13;
        *var_fn429_calc_ig__expiforarg_dn4_slot = var_fn429_calc_ig__expiforarg_dn4;
        *var_fn429_calc_ig__expiforarg_dn8_slot = var_fn429_calc_ig__expiforarg_dn8;
        *var_fn429_calc_ig__expiforarg_hinj_slot = var_fn429_calc_ig__expiforarg_hinj;
        *var_fn429_calc_ig__expiforarg_hinj_dn13_slot = var_fn429_calc_ig__expiforarg_hinj_dn13;
        *var_fn429_calc_ig__expiforarg_hinj_dn4_slot = var_fn429_calc_ig__expiforarg_hinj_dn4;
        *var_fn429_calc_ig__expiforarg_hinj_dn8_slot = var_fn429_calc_ig__expiforarg_hinj_dn8;
        *var_fn429_calc_ig__expiforarg_hinj_vgsat_slot = var_fn429_calc_ig__expiforarg_hinj_vgsat;
        *var_fn429_calc_ig__expiforarg_hinj_vgsat_dn4_slot = var_fn429_calc_ig__expiforarg_hinj_vgsat_dn4;
        *var_fn429_calc_ig__expiforarg_nohinj_vgsat_slot = var_fn429_calc_ig__expiforarg_nohinj_vgsat;
        *var_fn429_calc_ig__expiforarg_nohinj_vgsat_dn4_slot = var_fn429_calc_ig__expiforarg_nohinj_vgsat_dn4;
        *var_fn429_calc_ig__ffvgin_slot = var_fn429_calc_ig__ffvgin;
        *var_fn429_calc_ig__ffvgin_dn13_slot = var_fn429_calc_ig__ffvgin_dn13;
        *var_fn429_calc_ig__ffvgin_dn4_slot = var_fn429_calc_ig__ffvgin_dn4;
        *var_fn429_calc_ig__ffvgin_dn8_slot = var_fn429_calc_ig__ffvgin_dn8;
        *var_fn429_calc_ig__iginbd_vgsat_slot = var_fn429_calc_ig__iginbd_vgsat;
        *var_fn429_calc_ig__iginbd_vgsat_dn4_slot = var_fn429_calc_ig__iginbd_vgsat_dn4;
        *var_fn429_calc_ig__igindiode_slot = var_fn429_calc_ig__igindiode;
        *var_fn429_calc_ig__igindiode_dn13_slot = var_fn429_calc_ig__igindiode_dn13;
        *var_fn429_calc_ig__igindiode_dn4_slot = var_fn429_calc_ig__igindiode_dn4;
        *var_fn429_calc_ig__igindiode_dn8_slot = var_fn429_calc_ig__igindiode_dn8;
        *var_fn429_calc_ig__igindiode_hinj_slot = var_fn429_calc_ig__igindiode_hinj;
        *var_fn429_calc_ig__igindiode_hinj_dn13_slot = var_fn429_calc_ig__igindiode_hinj_dn13;
        *var_fn429_calc_ig__igindiode_hinj_dn4_slot = var_fn429_calc_ig__igindiode_hinj_dn4;
        *var_fn429_calc_ig__igindiode_hinj_dn8_slot = var_fn429_calc_ig__igindiode_hinj_dn8;
        *var_fn429_calc_ig__igindiode_hinj_pre_slot = var_fn429_calc_ig__igindiode_hinj_pre;
        *var_fn429_calc_ig__igindiode_hinj_pre_dn4_slot = var_fn429_calc_ig__igindiode_hinj_pre_dn4;
        *var_fn429_calc_ig__igindiode_hinj_vgsat_slot = var_fn429_calc_ig__igindiode_hinj_vgsat;
        *var_fn429_calc_ig__igindiode_hinj_vgsat_dn4_slot = var_fn429_calc_ig__igindiode_hinj_vgsat_dn4;
        *var_fn429_calc_ig__igindiode_nohinj_slot = var_fn429_calc_ig__igindiode_nohinj;
        *var_fn429_calc_ig__igindiode_nohinj_dn13_slot = var_fn429_calc_ig__igindiode_nohinj_dn13;
        *var_fn429_calc_ig__igindiode_nohinj_dn4_slot = var_fn429_calc_ig__igindiode_nohinj_dn4;
        *var_fn429_calc_ig__igindiode_nohinj_dn8_slot = var_fn429_calc_ig__igindiode_nohinj_dn8;
        *var_fn429_calc_ig__igindiode_nohinj_vgsat_slot = var_fn429_calc_ig__igindiode_nohinj_vgsat;
        *var_fn429_calc_ig__igindiode_nohinj_vgsat_dn4_slot = var_fn429_calc_ig__igindiode_nohinj_vgsat_dn4;
        *var_fn429_calc_ig__isdiodeout_slot = var_fn429_calc_ig__isdiodeout;
        *var_fn429_calc_ig__isdiodeout_dn4_slot = var_fn429_calc_ig__isdiodeout_dn4;
        *var_fn429_calc_ig__pg_paramin_hinj_slot = var_fn429_calc_ig__pg_paramin_hinj;
        *var_guard430_slot = var_guard430;
        *var_guard431_slot = var_guard431;
        *var_guard432_slot = var_guard432;
        *var_guard433_slot = var_guard433;
    }

    pub(super) fn stamp_transient_block_89(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_fn429_calc_ig__betarecin: f64,
        var_fn429_calc_ig__igindiode: f64,
        var_fn429_calc_ig__igindiode_dn13: f64,
        var_fn429_calc_ig__igindiode_dn4: f64,
        var_fn429_calc_ig__igindiode_dn8: f64,
        var_fn429_calc_ig__irecin: f64,
        var_fn429_calc_ig__ngf: f64,
        var_fn429_calc_ig__pgsrecin: f64,
        var_fn429_calc_ig__phitin: f64,
        var_fn429_calc_ig__phitin_dn4: f64,
        var_fn429_calc_ig__tfacdiodein: f64,
        var_fn429_calc_ig__tfacdiodein_dn4: f64,
        var_fn429_calc_ig__type: f64,
        var_fn429_calc_ig__vgin: f64,
        var_fn429_calc_ig__vgin_dn13: f64,
        var_fn429_calc_ig__vgin_dn8: f64,
        var_fn429_calc_ig__vgsatqin: f64,
        var_fn429_calc_ig__w: f64,
        var_guard417: f64,
        var_guard428: f64,
        var_phit: f64,
        var_phit_dn4: f64,
        var_tfacdiode: f64,
        var_tfacdiode_dn4: f64,
        var_fn429_calc_ig__expirev_slot: &mut f64,
        var_fn429_calc_ig__expirev_dn13_slot: &mut f64,
        var_fn429_calc_ig__expirev_dn4_slot: &mut f64,
        var_fn429_calc_ig__expirev_dn8_slot: &mut f64,
        var_fn429_calc_ig__expirevarg_slot: &mut f64,
        var_fn429_calc_ig__expirevarg_dn13_slot: &mut f64,
        var_fn429_calc_ig__expirevarg_dn4_slot: &mut f64,
        var_fn429_calc_ig__expirevarg_dn8_slot: &mut f64,
        var_fn429_calc_ig__frecgin_slot: &mut f64,
        var_fn429_calc_ig__frecgin_dn13_slot: &mut f64,
        var_fn429_calc_ig__frecgin_dn8_slot: &mut f64,
        var_fn429_calc_ig__iginrec_slot: &mut f64,
        var_fn429_calc_ig__iginrec_dn13_slot: &mut f64,
        var_fn429_calc_ig__iginrec_dn4_slot: &mut f64,
        var_fn429_calc_ig__iginrec_dn8_slot: &mut f64,
        var_fn429_calc_ig__igout_slot: &mut f64,
        var_fn429_calc_ig__igout_dn13_slot: &mut f64,
        var_fn429_calc_ig__igout_dn4_slot: &mut f64,
        var_fn429_calc_ig__igout_dn8_slot: &mut f64,
        var_fn429_calc_ig__isrecout_slot: &mut f64,
        var_fn429_calc_ig__isrecout_dn4_slot: &mut f64,
        var_fn429_calc_ig__return_slot: &mut f64,
        var_fn429_calc_ig__return_dn13_slot: &mut f64,
        var_fn429_calc_ig__return_dn4_slot: &mut f64,
        var_fn429_calc_ig__return_dn8_slot: &mut f64,
        var_fn434_calc_ig__alpha2_phit_slot: &mut f64,
        var_fn434_calc_ig__alpha2_phit_dn4_slot: &mut f64,
        var_fn434_calc_ig__alphagin_slot: &mut f64,
        var_fn434_calc_ig__betarecin_slot: &mut f64,
        var_fn434_calc_ig__expbd1_slot: &mut f64,
        var_fn434_calc_ig__expbd1_dn17_slot: &mut f64,
        var_fn434_calc_ig__expbd1_dn4_slot: &mut f64,
        var_fn434_calc_ig__expbd1_dn8_slot: &mut f64,
        var_fn434_calc_ig__expbdarg1_slot: &mut f64,
        var_fn434_calc_ig__expbdarg1_dn17_slot: &mut f64,
        var_fn434_calc_ig__expbdarg1_dn4_slot: &mut f64,
        var_fn434_calc_ig__expbdarg1_dn8_slot: &mut f64,
        var_fn434_calc_ig__expbdarg2_slot: &mut f64,
        var_fn434_calc_ig__expbdarg2_dn4_slot: &mut f64,
        var_fn434_calc_ig__ffvgin_slot: &mut f64,
        var_fn434_calc_ig__ffvgin_dn17_slot: &mut f64,
        var_fn434_calc_ig__ffvgin_dn4_slot: &mut f64,
        var_fn434_calc_ig__ffvgin_dn8_slot: &mut f64,
        var_fn434_calc_ig__fracin_slot: &mut f64,
        var_fn434_calc_ig__frecgin_slot: &mut f64,
        var_fn434_calc_ig__frecgin_dn17_slot: &mut f64,
        var_fn434_calc_ig__frecgin_dn8_slot: &mut f64,
        var_fn434_calc_ig__iginbd_slot: &mut f64,
        var_fn434_calc_ig__iginbd_dn17_slot: &mut f64,
        var_fn434_calc_ig__iginbd_dn4_slot: &mut f64,
        var_fn434_calc_ig__iginbd_dn8_slot: &mut f64,
        var_fn434_calc_ig__igindiode_slot: &mut f64,
        var_fn434_calc_ig__igindiode_dn17_slot: &mut f64,
        var_fn434_calc_ig__igindiode_dn4_slot: &mut f64,
        var_fn434_calc_ig__igindiode_dn8_slot: &mut f64,
        var_fn434_calc_ig__iginrec_slot: &mut f64,
        var_fn434_calc_ig__iginrec_dn17_slot: &mut f64,
        var_fn434_calc_ig__iginrec_dn4_slot: &mut f64,
        var_fn434_calc_ig__iginrec_dn8_slot: &mut f64,
        var_fn434_calc_ig__igout_slot: &mut f64,
        var_fn434_calc_ig__igout_dn17_slot: &mut f64,
        var_fn434_calc_ig__igout_dn4_slot: &mut f64,
        var_fn434_calc_ig__igout_dn8_slot: &mut f64,
        var_fn434_calc_ig__ijin_slot: &mut f64,
        var_fn434_calc_ig__irecin_slot: &mut f64,
        var_fn434_calc_ig__isdiodeout_slot: &mut f64,
        var_fn434_calc_ig__isdiodeout_dn4_slot: &mut f64,
        var_fn434_calc_ig__isrecout_slot: &mut f64,
        var_fn434_calc_ig__isrecout_dn4_slot: &mut f64,
        var_fn434_calc_ig__kbdgatein_slot: &mut f64,
        var_fn434_calc_ig__ngf_slot: &mut f64,
        var_fn434_calc_ig__pbdgin_slot: &mut f64,
        var_fn434_calc_ig__pg_param1_slot: &mut f64,
        var_fn434_calc_ig__pg_paramin_slot: &mut f64,
        var_fn434_calc_ig__pgsrecin_slot: &mut f64,
        var_fn434_calc_ig__phitin_slot: &mut f64,
        var_fn434_calc_ig__phitin_dn4_slot: &mut f64,
        var_fn434_calc_ig__return_slot: &mut f64,
        var_fn434_calc_ig__return_dn17_slot: &mut f64,
        var_fn434_calc_ig__return_dn4_slot: &mut f64,
        var_fn434_calc_ig__return_dn8_slot: &mut f64,
        var_fn434_calc_ig__t0_slot: &mut f64,
        var_fn434_calc_ig__t0_dn4_slot: &mut f64,
        var_fn434_calc_ig__tfacdiodein_slot: &mut f64,
        var_fn434_calc_ig__tfacdiodein_dn4_slot: &mut f64,
        var_fn434_calc_ig__type_slot: &mut f64,
        var_fn434_calc_ig__vbdgin_slot: &mut f64,
        var_fn434_calc_ig__vgin_slot: &mut f64,
        var_fn434_calc_ig__vgin_dn17_slot: &mut f64,
        var_fn434_calc_ig__vgin_dn8_slot: &mut f64,
        var_fn434_calc_ig__vgsatin_slot: &mut f64,
        var_fn434_calc_ig__vgsatqin_slot: &mut f64,
        var_fn434_calc_ig__vjg_slot: &mut f64,
        var_fn434_calc_ig__w_slot: &mut f64,
        var_igsi2_slot: &mut f64,
        var_igsi2_dn13_slot: &mut f64,
        var_igsi2_dn4_slot: &mut f64,
        var_igsi2_dn8_slot: &mut f64,
    ) {
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let mut var_fn429_calc_ig__expirev: f64 = *var_fn429_calc_ig__expirev_slot;
        let mut var_fn429_calc_ig__expirev_dn13: f64 = *var_fn429_calc_ig__expirev_dn13_slot;
        let mut var_fn429_calc_ig__expirev_dn4: f64 = *var_fn429_calc_ig__expirev_dn4_slot;
        let mut var_fn429_calc_ig__expirev_dn8: f64 = *var_fn429_calc_ig__expirev_dn8_slot;
        let mut var_fn429_calc_ig__expirevarg: f64 = *var_fn429_calc_ig__expirevarg_slot;
        let mut var_fn429_calc_ig__expirevarg_dn13: f64 = *var_fn429_calc_ig__expirevarg_dn13_slot;
        let mut var_fn429_calc_ig__expirevarg_dn4: f64 = *var_fn429_calc_ig__expirevarg_dn4_slot;
        let mut var_fn429_calc_ig__expirevarg_dn8: f64 = *var_fn429_calc_ig__expirevarg_dn8_slot;
        let mut var_fn429_calc_ig__frecgin: f64 = *var_fn429_calc_ig__frecgin_slot;
        let mut var_fn429_calc_ig__frecgin_dn13: f64 = *var_fn429_calc_ig__frecgin_dn13_slot;
        let mut var_fn429_calc_ig__frecgin_dn8: f64 = *var_fn429_calc_ig__frecgin_dn8_slot;
        let mut var_fn429_calc_ig__iginrec: f64 = *var_fn429_calc_ig__iginrec_slot;
        let mut var_fn429_calc_ig__iginrec_dn13: f64 = *var_fn429_calc_ig__iginrec_dn13_slot;
        let mut var_fn429_calc_ig__iginrec_dn4: f64 = *var_fn429_calc_ig__iginrec_dn4_slot;
        let mut var_fn429_calc_ig__iginrec_dn8: f64 = *var_fn429_calc_ig__iginrec_dn8_slot;
        let mut var_fn429_calc_ig__igout: f64 = *var_fn429_calc_ig__igout_slot;
        let mut var_fn429_calc_ig__igout_dn13: f64 = *var_fn429_calc_ig__igout_dn13_slot;
        let mut var_fn429_calc_ig__igout_dn4: f64 = *var_fn429_calc_ig__igout_dn4_slot;
        let mut var_fn429_calc_ig__igout_dn8: f64 = *var_fn429_calc_ig__igout_dn8_slot;
        let mut var_fn429_calc_ig__isrecout: f64 = *var_fn429_calc_ig__isrecout_slot;
        let mut var_fn429_calc_ig__isrecout_dn4: f64 = *var_fn429_calc_ig__isrecout_dn4_slot;
        let mut var_fn429_calc_ig__return: f64 = *var_fn429_calc_ig__return_slot;
        let mut var_fn429_calc_ig__return_dn13: f64 = *var_fn429_calc_ig__return_dn13_slot;
        let mut var_fn429_calc_ig__return_dn4: f64 = *var_fn429_calc_ig__return_dn4_slot;
        let mut var_fn429_calc_ig__return_dn8: f64 = *var_fn429_calc_ig__return_dn8_slot;
        let mut var_fn434_calc_ig__alpha2_phit: f64 = *var_fn434_calc_ig__alpha2_phit_slot;
        let mut var_fn434_calc_ig__alpha2_phit_dn4: f64 = *var_fn434_calc_ig__alpha2_phit_dn4_slot;
        let mut var_fn434_calc_ig__alphagin: f64 = *var_fn434_calc_ig__alphagin_slot;
        let mut var_fn434_calc_ig__betarecin: f64 = *var_fn434_calc_ig__betarecin_slot;
        let mut var_fn434_calc_ig__expbd1: f64 = *var_fn434_calc_ig__expbd1_slot;
        let mut var_fn434_calc_ig__expbd1_dn17: f64 = *var_fn434_calc_ig__expbd1_dn17_slot;
        let mut var_fn434_calc_ig__expbd1_dn4: f64 = *var_fn434_calc_ig__expbd1_dn4_slot;
        let mut var_fn434_calc_ig__expbd1_dn8: f64 = *var_fn434_calc_ig__expbd1_dn8_slot;
        let mut var_fn434_calc_ig__expbdarg1: f64 = *var_fn434_calc_ig__expbdarg1_slot;
        let mut var_fn434_calc_ig__expbdarg1_dn17: f64 = *var_fn434_calc_ig__expbdarg1_dn17_slot;
        let mut var_fn434_calc_ig__expbdarg1_dn4: f64 = *var_fn434_calc_ig__expbdarg1_dn4_slot;
        let mut var_fn434_calc_ig__expbdarg1_dn8: f64 = *var_fn434_calc_ig__expbdarg1_dn8_slot;
        let mut var_fn434_calc_ig__expbdarg2: f64 = *var_fn434_calc_ig__expbdarg2_slot;
        let mut var_fn434_calc_ig__expbdarg2_dn4: f64 = *var_fn434_calc_ig__expbdarg2_dn4_slot;
        let mut var_fn434_calc_ig__ffvgin: f64 = *var_fn434_calc_ig__ffvgin_slot;
        let mut var_fn434_calc_ig__ffvgin_dn17: f64 = *var_fn434_calc_ig__ffvgin_dn17_slot;
        let mut var_fn434_calc_ig__ffvgin_dn4: f64 = *var_fn434_calc_ig__ffvgin_dn4_slot;
        let mut var_fn434_calc_ig__ffvgin_dn8: f64 = *var_fn434_calc_ig__ffvgin_dn8_slot;
        let mut var_fn434_calc_ig__fracin: f64 = *var_fn434_calc_ig__fracin_slot;
        let mut var_fn434_calc_ig__frecgin: f64 = *var_fn434_calc_ig__frecgin_slot;
        let mut var_fn434_calc_ig__frecgin_dn17: f64 = *var_fn434_calc_ig__frecgin_dn17_slot;
        let mut var_fn434_calc_ig__frecgin_dn8: f64 = *var_fn434_calc_ig__frecgin_dn8_slot;
        let mut var_fn434_calc_ig__iginbd: f64 = *var_fn434_calc_ig__iginbd_slot;
        let mut var_fn434_calc_ig__iginbd_dn17: f64 = *var_fn434_calc_ig__iginbd_dn17_slot;
        let mut var_fn434_calc_ig__iginbd_dn4: f64 = *var_fn434_calc_ig__iginbd_dn4_slot;
        let mut var_fn434_calc_ig__iginbd_dn8: f64 = *var_fn434_calc_ig__iginbd_dn8_slot;
        let mut var_fn434_calc_ig__igindiode: f64 = *var_fn434_calc_ig__igindiode_slot;
        let mut var_fn434_calc_ig__igindiode_dn17: f64 = *var_fn434_calc_ig__igindiode_dn17_slot;
        let mut var_fn434_calc_ig__igindiode_dn4: f64 = *var_fn434_calc_ig__igindiode_dn4_slot;
        let mut var_fn434_calc_ig__igindiode_dn8: f64 = *var_fn434_calc_ig__igindiode_dn8_slot;
        let mut var_fn434_calc_ig__iginrec: f64 = *var_fn434_calc_ig__iginrec_slot;
        let mut var_fn434_calc_ig__iginrec_dn17: f64 = *var_fn434_calc_ig__iginrec_dn17_slot;
        let mut var_fn434_calc_ig__iginrec_dn4: f64 = *var_fn434_calc_ig__iginrec_dn4_slot;
        let mut var_fn434_calc_ig__iginrec_dn8: f64 = *var_fn434_calc_ig__iginrec_dn8_slot;
        let mut var_fn434_calc_ig__igout: f64 = *var_fn434_calc_ig__igout_slot;
        let mut var_fn434_calc_ig__igout_dn17: f64 = *var_fn434_calc_ig__igout_dn17_slot;
        let mut var_fn434_calc_ig__igout_dn4: f64 = *var_fn434_calc_ig__igout_dn4_slot;
        let mut var_fn434_calc_ig__igout_dn8: f64 = *var_fn434_calc_ig__igout_dn8_slot;
        let mut var_fn434_calc_ig__ijin: f64 = *var_fn434_calc_ig__ijin_slot;
        let mut var_fn434_calc_ig__irecin: f64 = *var_fn434_calc_ig__irecin_slot;
        let mut var_fn434_calc_ig__isdiodeout: f64 = *var_fn434_calc_ig__isdiodeout_slot;
        let mut var_fn434_calc_ig__isdiodeout_dn4: f64 = *var_fn434_calc_ig__isdiodeout_dn4_slot;
        let mut var_fn434_calc_ig__isrecout: f64 = *var_fn434_calc_ig__isrecout_slot;
        let mut var_fn434_calc_ig__isrecout_dn4: f64 = *var_fn434_calc_ig__isrecout_dn4_slot;
        let mut var_fn434_calc_ig__kbdgatein: f64 = *var_fn434_calc_ig__kbdgatein_slot;
        let mut var_fn434_calc_ig__ngf: f64 = *var_fn434_calc_ig__ngf_slot;
        let mut var_fn434_calc_ig__pbdgin: f64 = *var_fn434_calc_ig__pbdgin_slot;
        let mut var_fn434_calc_ig__pg_param1: f64 = *var_fn434_calc_ig__pg_param1_slot;
        let mut var_fn434_calc_ig__pg_paramin: f64 = *var_fn434_calc_ig__pg_paramin_slot;
        let mut var_fn434_calc_ig__pgsrecin: f64 = *var_fn434_calc_ig__pgsrecin_slot;
        let mut var_fn434_calc_ig__phitin: f64 = *var_fn434_calc_ig__phitin_slot;
        let mut var_fn434_calc_ig__phitin_dn4: f64 = *var_fn434_calc_ig__phitin_dn4_slot;
        let mut var_fn434_calc_ig__return: f64 = *var_fn434_calc_ig__return_slot;
        let mut var_fn434_calc_ig__return_dn17: f64 = *var_fn434_calc_ig__return_dn17_slot;
        let mut var_fn434_calc_ig__return_dn4: f64 = *var_fn434_calc_ig__return_dn4_slot;
        let mut var_fn434_calc_ig__return_dn8: f64 = *var_fn434_calc_ig__return_dn8_slot;
        let mut var_fn434_calc_ig__t0: f64 = *var_fn434_calc_ig__t0_slot;
        let mut var_fn434_calc_ig__t0_dn4: f64 = *var_fn434_calc_ig__t0_dn4_slot;
        let mut var_fn434_calc_ig__tfacdiodein: f64 = *var_fn434_calc_ig__tfacdiodein_slot;
        let mut var_fn434_calc_ig__tfacdiodein_dn4: f64 = *var_fn434_calc_ig__tfacdiodein_dn4_slot;
        let mut var_fn434_calc_ig__type: f64 = *var_fn434_calc_ig__type_slot;
        let mut var_fn434_calc_ig__vbdgin: f64 = *var_fn434_calc_ig__vbdgin_slot;
        let mut var_fn434_calc_ig__vgin: f64 = *var_fn434_calc_ig__vgin_slot;
        let mut var_fn434_calc_ig__vgin_dn17: f64 = *var_fn434_calc_ig__vgin_dn17_slot;
        let mut var_fn434_calc_ig__vgin_dn8: f64 = *var_fn434_calc_ig__vgin_dn8_slot;
        let mut var_fn434_calc_ig__vgsatin: f64 = *var_fn434_calc_ig__vgsatin_slot;
        let mut var_fn434_calc_ig__vgsatqin: f64 = *var_fn434_calc_ig__vgsatqin_slot;
        let mut var_fn434_calc_ig__vjg: f64 = *var_fn434_calc_ig__vjg_slot;
        let mut var_fn434_calc_ig__w: f64 = *var_fn434_calc_ig__w_slot;
        let mut var_igsi2: f64 = *var_igsi2_slot;
        let mut var_igsi2_dn13: f64 = *var_igsi2_dn13_slot;
        let mut var_igsi2_dn4: f64 = *var_igsi2_dn4_slot;
        let mut var_igsi2_dn8: f64 = *var_igsi2_dn8_slot;

        let (assign36060_e33133, assign36060_e33133_d_n8, assign36060_e33133_d_n13,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        let assign36060_e33090: f64 = (-var_fn429_calc_ig__vgin);
        let (assign36060_e33123, assign36060_e33123_d_n8, assign36060_e33123_d_n13,) = {
            if (p.p52 != 0.0) {
                let assign36060_e33098: f64 = (var_fn429_calc_ig__vgin / var_fn429_calc_ig__vgsatqin);
                let assign36060_e33101: f64 = (0.001 / p.p53);
                let assign36060_e33104: f64 = (var_fn429_calc_ig__vgin / var_fn429_calc_ig__vgsatqin);
                let assign36060_e33105: f64 = (assign36060_e33101 * assign36060_e33104);
                let assign36060_e33106: f64 = (assign36060_e33105).tanh();
                let assign36060_e33107: f64 = (assign36060_e33098 * assign36060_e33106);
                (assign36060_e33107, (((var_fn429_calc_ig__vgin_dn8 / var_fn429_calc_ig__vgsatqin) * assign36060_e33106) + (assign36060_e33098 * ((assign36060_e33101 * (var_fn429_calc_ig__vgin_dn8 / var_fn429_calc_ig__vgsatqin)) / ((assign36060_e33105).cosh() * (assign36060_e33105).cosh())))), (((var_fn429_calc_ig__vgin_dn13 / var_fn429_calc_ig__vgsatqin) * assign36060_e33106) + (assign36060_e33098 * ((assign36060_e33101 * (var_fn429_calc_ig__vgin_dn13 / var_fn429_calc_ig__vgsatqin)) / ((assign36060_e33105).cosh() * (assign36060_e33105).cosh())))),)
            } else {
                let (assign36060_e33122, assign36060_e33122_d_n8, assign36060_e33122_d_n13,) = {
                    if (p.p52 == 0.0) {
                        let __rspice_inv_cse_0: f64 = 1.0 / var_fn429_calc_ig__vgsatqin;
                        let assign36060_e33113: f64 = (var_fn429_calc_ig__vgin * __rspice_inv_cse_0);
                        let assign36060_e33116: f64 = (var_fn429_calc_ig__vgin * __rspice_inv_cse_0);
                        let assign36060_e33117: f64 = (assign36060_e33113 * assign36060_e33116);
                        let assign36060_e33119: f64 = (assign36060_e33117 + p.p53);
                        let assign36060_e33120: f64 = (assign36060_e33119).sqrt();
                        (assign36060_e33120, ((((var_fn429_calc_ig__vgin_dn8 / var_fn429_calc_ig__vgsatqin) * assign36060_e33116) + (assign36060_e33113 * (var_fn429_calc_ig__vgin_dn8 / var_fn429_calc_ig__vgsatqin))) / (2.0 * assign36060_e33120)), ((((var_fn429_calc_ig__vgin_dn13 / var_fn429_calc_ig__vgsatqin) * assign36060_e33116) + (assign36060_e33113 * (var_fn429_calc_ig__vgin_dn13 / var_fn429_calc_ig__vgsatqin))) / (2.0 * assign36060_e33120)),)
                    } else {
                        (0.0, 0.0, 0.0,)
                    }
                };
                (assign36060_e33122, assign36060_e33122_d_n8, assign36060_e33122_d_n13,)
            }
        };
        let assign36060_e33125: f64 = (assign36060_e33123).powf(var_fn429_calc_ig__betarecin);
        let assign36060_e33126: f64 = (1.0 + assign36060_e33125);
        let assign36060_e33129: f64 = (1.0 / var_fn429_calc_ig__betarecin);
        let assign36060_e33130: f64 = (assign36060_e33126).powf(assign36060_e33129);
        let assign36060_e33131: f64 = (assign36060_e33090 / assign36060_e33130);
        (assign36060_e33131, ((((-var_fn429_calc_ig__vgin_dn8) * assign36060_e33130) - (assign36060_e33090 * if 0.0 == 0.0 && ((assign36060_e33129) as f64).is_finite() && ((assign36060_e33129) as f64).fract() == 0.0 { if assign36060_e33129 == 0.0 { 0.0 } else { (assign36060_e33129 * ((assign36060_e33126).powf(assign36060_e33129 - 1.0) * if 0.0 == 0.0 && ((var_fn429_calc_ig__betarecin) as f64).is_finite() && ((var_fn429_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn429_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn429_calc_ig__betarecin * ((assign36060_e33123).powf(var_fn429_calc_ig__betarecin - 1.0) * assign36060_e33123_d_n8)) } } else { (assign36060_e33125 * (var_fn429_calc_ig__betarecin * (assign36060_e33123_d_n8 / assign36060_e33123))) })) } } else { (assign36060_e33130 * (assign36060_e33129 * (if 0.0 == 0.0 && ((var_fn429_calc_ig__betarecin) as f64).is_finite() && ((var_fn429_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn429_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn429_calc_ig__betarecin * ((assign36060_e33123).powf(var_fn429_calc_ig__betarecin - 1.0) * assign36060_e33123_d_n8)) } } else { (assign36060_e33125 * (var_fn429_calc_ig__betarecin * (assign36060_e33123_d_n8 / assign36060_e33123))) } / assign36060_e33126))) })) / (assign36060_e33130 * assign36060_e33130)), ((((-var_fn429_calc_ig__vgin_dn13) * assign36060_e33130) - (assign36060_e33090 * if 0.0 == 0.0 && ((assign36060_e33129) as f64).is_finite() && ((assign36060_e33129) as f64).fract() == 0.0 { if assign36060_e33129 == 0.0 { 0.0 } else { (assign36060_e33129 * ((assign36060_e33126).powf(assign36060_e33129 - 1.0) * if 0.0 == 0.0 && ((var_fn429_calc_ig__betarecin) as f64).is_finite() && ((var_fn429_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn429_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn429_calc_ig__betarecin * ((assign36060_e33123).powf(var_fn429_calc_ig__betarecin - 1.0) * assign36060_e33123_d_n13)) } } else { (assign36060_e33125 * (var_fn429_calc_ig__betarecin * (assign36060_e33123_d_n13 / assign36060_e33123))) })) } } else { (assign36060_e33130 * (assign36060_e33129 * (if 0.0 == 0.0 && ((var_fn429_calc_ig__betarecin) as f64).is_finite() && ((var_fn429_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn429_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn429_calc_ig__betarecin * ((assign36060_e33123).powf(var_fn429_calc_ig__betarecin - 1.0) * assign36060_e33123_d_n13)) } } else { (assign36060_e33125 * (var_fn429_calc_ig__betarecin * (assign36060_e33123_d_n13 / assign36060_e33123))) } / assign36060_e33126))) })) / (assign36060_e33130 * assign36060_e33130)),)
    } else {
        (var_fn429_calc_ig__frecgin, var_fn429_calc_ig__frecgin_dn8, var_fn429_calc_ig__frecgin_dn13,)
    }
};
        var_fn429_calc_ig__frecgin = assign36060_e33133;
        var_fn429_calc_ig__frecgin_dn8 = assign36060_e33133_d_n8;
        var_fn429_calc_ig__frecgin_dn13 = assign36060_e33133_d_n13;

        let (assign36070_e33150, assign36070_e33150_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        let assign36070_e33138: f64 = (-var_fn429_calc_ig__type);
        let assign36070_e33140: f64 = (assign36070_e33138 * var_fn429_calc_ig__w);
        let assign36070_e33142: f64 = (assign36070_e33140 * var_fn429_calc_ig__ngf);
        let assign36070_e33144: f64 = (assign36070_e33142 * var_fn429_calc_ig__irecin);
        let assign36070_e33146: f64 = (assign36070_e33144 * var_fn429_calc_ig__tfacdiodein);
        let assign36070_e33148: f64 = assign36070_e33146;
        (assign36070_e33148, (assign36070_e33144 * var_fn429_calc_ig__tfacdiodein_dn4),)
    } else {
        (var_fn429_calc_ig__isrecout, var_fn429_calc_ig__isrecout_dn4,)
    }
};
        var_fn429_calc_ig__isrecout = assign36070_e33150;
        var_fn429_calc_ig__isrecout_dn4 = assign36070_e33150_d_n4;

        let (assign36080_e33160, assign36080_e33160_d_n4, assign36080_e33160_d_n8, assign36080_e33160_d_n13,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        let assign36080_e33156: f64 = (var_fn429_calc_ig__pgsrecin / var_fn429_calc_ig__phitin);
        let assign36080_e33158: f64 = (assign36080_e33156 * var_fn429_calc_ig__frecgin);
        (assign36080_e33158, ((-((var_fn429_calc_ig__pgsrecin * var_fn429_calc_ig__phitin_dn4) / (var_fn429_calc_ig__phitin * var_fn429_calc_ig__phitin))) * var_fn429_calc_ig__frecgin), (assign36080_e33156 * var_fn429_calc_ig__frecgin_dn8), (assign36080_e33156 * var_fn429_calc_ig__frecgin_dn13),)
    } else {
        (var_fn429_calc_ig__expirevarg, var_fn429_calc_ig__expirevarg_dn4, var_fn429_calc_ig__expirevarg_dn8, var_fn429_calc_ig__expirevarg_dn13,)
    }
};
        var_fn429_calc_ig__expirevarg = assign36080_e33160;
        var_fn429_calc_ig__expirevarg_dn4 = assign36080_e33160_d_n4;
        var_fn429_calc_ig__expirevarg_dn8 = assign36080_e33160_d_n8;
        var_fn429_calc_ig__expirevarg_dn13 = assign36080_e33160_d_n13;

        let (assign36090_e33204, assign36090_e33204_d_n4, assign36090_e33204_d_n8, assign36090_e33204_d_n13,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        let assign36090_e33170: f64 = (-50.0);
        let (assign36090_e33202, assign36090_e33202_d_n4, assign36090_e33202_d_n8, assign36090_e33202_d_n13,) = {
            if ((!(var_fn429_calc_ig__expirevarg > 50.0)) && (!(var_fn429_calc_ig__expirevarg < assign36090_e33170))) {
                let assign36090_e33175: f64 = (var_fn429_calc_ig__expirevarg).exp();
                (assign36090_e33175, (assign36090_e33175 * var_fn429_calc_ig__expirevarg_dn4), (assign36090_e33175 * var_fn429_calc_ig__expirevarg_dn8), (assign36090_e33175 * var_fn429_calc_ig__expirevarg_dn13),)
            } else {
                let assign36090_e33182: f64 = (-50.0);
                let (assign36090_e33201, assign36090_e33201_d_n4, assign36090_e33201_d_n8, assign36090_e33201_d_n13,) = {
                    if ((!(var_fn429_calc_ig__expirevarg > 50.0)) && (var_fn429_calc_ig__expirevarg < assign36090_e33182)) {
                        let assign36090_e33186: f64 = (-50.0);
                        let assign36090_e33187: f64 = (assign36090_e33186).exp();
                        (assign36090_e33187, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign36090_e33200, assign36090_e33200_d_n4, assign36090_e33200_d_n8, assign36090_e33200_d_n13,) = {
                            if (var_fn429_calc_ig__expirevarg > 50.0) {
                                let assign36090_e33192: f64 = (50.0_f64).exp();
                                let assign36090_e33196: f64 = (var_fn429_calc_ig__expirevarg - 50.0);
                                let assign36090_e33197: f64 = (1.0 + assign36090_e33196);
                                let assign36090_e33198: f64 = (assign36090_e33192 * assign36090_e33197);
                                (assign36090_e33198, (assign36090_e33192 * var_fn429_calc_ig__expirevarg_dn4), (assign36090_e33192 * var_fn429_calc_ig__expirevarg_dn8), (assign36090_e33192 * var_fn429_calc_ig__expirevarg_dn13),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign36090_e33200, assign36090_e33200_d_n4, assign36090_e33200_d_n8, assign36090_e33200_d_n13,)
                    }
                };
                (assign36090_e33201, assign36090_e33201_d_n4, assign36090_e33201_d_n8, assign36090_e33201_d_n13,)
            }
        };
        (assign36090_e33202, assign36090_e33202_d_n4, assign36090_e33202_d_n8, assign36090_e33202_d_n13,)
    } else {
        (var_fn429_calc_ig__expirev, var_fn429_calc_ig__expirev_dn4, var_fn429_calc_ig__expirev_dn8, var_fn429_calc_ig__expirev_dn13,)
    }
};
        var_fn429_calc_ig__expirev = assign36090_e33204;
        var_fn429_calc_ig__expirev_dn4 = assign36090_e33204_d_n4;
        var_fn429_calc_ig__expirev_dn8 = assign36090_e33204_d_n8;
        var_fn429_calc_ig__expirev_dn13 = assign36090_e33204_d_n13;

        let (assign36100_e33214, assign36100_e33214_d_n4, assign36100_e33214_d_n8, assign36100_e33214_d_n13,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        let assign36100_e33211: f64 = (var_fn429_calc_ig__expirev - 1.0);
        let assign36100_e33212: f64 = (var_fn429_calc_ig__isrecout * assign36100_e33211);
        (assign36100_e33212, ((var_fn429_calc_ig__isrecout_dn4 * assign36100_e33211) + (var_fn429_calc_ig__isrecout * var_fn429_calc_ig__expirev_dn4)), (var_fn429_calc_ig__isrecout * var_fn429_calc_ig__expirev_dn8), (var_fn429_calc_ig__isrecout * var_fn429_calc_ig__expirev_dn13),)
    } else {
        (var_fn429_calc_ig__iginrec, var_fn429_calc_ig__iginrec_dn4, var_fn429_calc_ig__iginrec_dn8, var_fn429_calc_ig__iginrec_dn13,)
    }
};
        var_fn429_calc_ig__iginrec = assign36100_e33214;
        var_fn429_calc_ig__iginrec_dn4 = assign36100_e33214_d_n4;
        var_fn429_calc_ig__iginrec_dn8 = assign36100_e33214_d_n8;
        var_fn429_calc_ig__iginrec_dn13 = assign36100_e33214_d_n13;

        let (assign36110_e33222, assign36110_e33222_d_n4, assign36110_e33222_d_n8, assign36110_e33222_d_n13,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        let assign36110_e33220: f64 = (var_fn429_calc_ig__igindiode + var_fn429_calc_ig__iginrec);
        (assign36110_e33220, (var_fn429_calc_ig__igindiode_dn4 + var_fn429_calc_ig__iginrec_dn4), (var_fn429_calc_ig__igindiode_dn8 + var_fn429_calc_ig__iginrec_dn8), (var_fn429_calc_ig__igindiode_dn13 + var_fn429_calc_ig__iginrec_dn13),)
    } else {
        (var_fn429_calc_ig__igout, var_fn429_calc_ig__igout_dn4, var_fn429_calc_ig__igout_dn8, var_fn429_calc_ig__igout_dn13,)
    }
};
        var_fn429_calc_ig__igout = assign36110_e33222;
        var_fn429_calc_ig__igout_dn4 = assign36110_e33222_d_n4;
        var_fn429_calc_ig__igout_dn8 = assign36110_e33222_d_n8;
        var_fn429_calc_ig__igout_dn13 = assign36110_e33222_d_n13;

        let (assign36120_e33228, assign36120_e33228_d_n4, assign36120_e33228_d_n8, assign36120_e33228_d_n13,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (var_fn429_calc_ig__igout, var_fn429_calc_ig__igout_dn4, var_fn429_calc_ig__igout_dn8, var_fn429_calc_ig__igout_dn13,)
    } else {
        (var_fn429_calc_ig__return, var_fn429_calc_ig__return_dn4, var_fn429_calc_ig__return_dn8, var_fn429_calc_ig__return_dn13,)
    }
};
        var_fn429_calc_ig__return = assign36120_e33228;
        var_fn429_calc_ig__return_dn4 = assign36120_e33228_d_n4;
        var_fn429_calc_ig__return_dn8 = assign36120_e33228_d_n8;
        var_fn429_calc_ig__return_dn13 = assign36120_e33228_d_n13;

        let (assign36150_e33246, assign36150_e33246_d_n4, assign36150_e33246_d_n8, assign36150_e33246_d_n13,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (var_fn429_calc_ig__return, var_fn429_calc_ig__return_dn4, var_fn429_calc_ig__return_dn8, var_fn429_calc_ig__return_dn13,)
    } else {
        (var_igsi2, var_igsi2_dn4, var_igsi2_dn8, var_igsi2_dn13,)
    }
};
        var_igsi2 = assign36150_e33246;
        var_igsi2_dn4 = assign36150_e33246_d_n4;
        var_igsi2_dn8 = assign36150_e33246_d_n8;
        var_igsi2_dn13 = assign36150_e33246_d_n13;

        let (assign36160_e33252, assign36160_e33252_d_n4, assign36160_e33252_d_n8, assign36160_e33252_d_n17,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn434_calc_ig__return, var_fn434_calc_ig__return_dn4, var_fn434_calc_ig__return_dn8, var_fn434_calc_ig__return_dn17,)
    }
};
        var_fn434_calc_ig__return = assign36160_e33252;
        var_fn434_calc_ig__return_dn4 = assign36160_e33252_d_n4;
        var_fn434_calc_ig__return_dn8 = assign36160_e33252_d_n8;
        var_fn434_calc_ig__return_dn17 = assign36160_e33252_d_n17;

        let (assign36170_e33258, assign36170_e33258_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn434_calc_ig__isdiodeout, var_fn434_calc_ig__isdiodeout_dn4,)
    }
};
        var_fn434_calc_ig__isdiodeout = assign36170_e33258;
        var_fn434_calc_ig__isdiodeout_dn4 = assign36170_e33258_d_n4;

        let (assign36180_e33264, assign36180_e33264_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn434_calc_ig__isrecout, var_fn434_calc_ig__isrecout_dn4,)
    }
};
        var_fn434_calc_ig__isrecout = assign36180_e33264;
        var_fn434_calc_ig__isrecout_dn4 = assign36180_e33264_d_n4;

        let (assign36190_e33272, assign36190_e33272_d_n8, assign36190_e33272_d_n17,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        let assign36190_e33270: f64 = (p.p6 * (nv8 - nv17));
        (assign36190_e33270, p.p6, (-p.p6),)
    } else {
        (var_fn434_calc_ig__vgin, var_fn434_calc_ig__vgin_dn8, var_fn434_calc_ig__vgin_dn17,)
    }
};
        var_fn434_calc_ig__vgin = assign36190_e33272;
        var_fn434_calc_ig__vgin_dn8 = assign36190_e33272_d_n8;
        var_fn434_calc_ig__vgin_dn17 = assign36190_e33272_d_n17;

        let (assign36200_e33278, assign36200_e33278_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (var_phit, var_phit_dn4,)
    } else {
        (var_fn434_calc_ig__phitin, var_fn434_calc_ig__phitin_dn4,)
    }
};
        var_fn434_calc_ig__phitin = assign36200_e33278;
        var_fn434_calc_ig__phitin_dn4 = assign36200_e33278_d_n4;

        let (assign36210_e33284,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (p.p265,)
    } else {
        (var_fn434_calc_ig__vgsatin,)
    }
};
        var_fn434_calc_ig__vgsatin = assign36210_e33284;

        let (assign36220_e33290,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (p.p267,)
    } else {
        (var_fn434_calc_ig__alphagin,)
    }
};
        var_fn434_calc_ig__alphagin = assign36220_e33290;

        let (assign36230_e33296,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (1.0,)
    } else {
        (var_fn434_calc_ig__fracin,)
    }
};
        var_fn434_calc_ig__fracin = assign36230_e33296;

        let (assign36240_e33302,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (p.p263,)
    } else {
        (var_fn434_calc_ig__pg_paramin,)
    }
};
        var_fn434_calc_ig__pg_paramin = assign36240_e33302;

        let (assign36250_e33308,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (p.p281,)
    } else {
        (var_fn434_calc_ig__pbdgin,)
    }
};
        var_fn434_calc_ig__pbdgin = assign36250_e33308;

        let (assign36260_e33314,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (p.p280,)
    } else {
        (var_fn434_calc_ig__vbdgin,)
    }
};
        var_fn434_calc_ig__vbdgin = assign36260_e33314;

        let (assign36270_e33320, assign36270_e33320_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (var_tfacdiode, var_tfacdiode_dn4,)
    } else {
        (var_fn434_calc_ig__tfacdiodein, var_fn434_calc_ig__tfacdiodein_dn4,)
    }
};
        var_fn434_calc_ig__tfacdiodein = assign36270_e33320;
        var_fn434_calc_ig__tfacdiodein_dn4 = assign36270_e33320_d_n4;

        let (assign36280_e33326,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (p.p0,)
    } else {
        (var_fn434_calc_ig__w,)
    }
};
        var_fn434_calc_ig__w = assign36280_e33326;

        let (assign36290_e33332,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (p.p2,)
    } else {
        (var_fn434_calc_ig__ngf,)
    }
};
        var_fn434_calc_ig__ngf = assign36290_e33332;

        let (assign36300_e33338,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0,)
    } else {
        (var_fn434_calc_ig__ijin,)
    }
};
        var_fn434_calc_ig__ijin = assign36300_e33338;

        let (assign36310_e33344,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0,)
    } else {
        (var_fn434_calc_ig__kbdgatein,)
    }
};
        var_fn434_calc_ig__kbdgatein = assign36310_e33344;

        let (assign36320_e33350,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (p.p289,)
    } else {
        (var_fn434_calc_ig__vgsatqin,)
    }
};
        var_fn434_calc_ig__vgsatqin = assign36320_e33350;

        let (assign36330_e33356,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (p.p290,)
    } else {
        (var_fn434_calc_ig__betarecin,)
    }
};
        var_fn434_calc_ig__betarecin = assign36330_e33356;

        let (assign36340_e33366,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        let assign36340_e33362: f64 = (1.0 - p.p255);
        let assign36340_e33364: f64 = (assign36340_e33362 * p.p288);
        (assign36340_e33364,)
    } else {
        (var_fn434_calc_ig__irecin,)
    }
};
        var_fn434_calc_ig__irecin = assign36340_e33366;

        let (assign36350_e33372,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (p.p287,)
    } else {
        (var_fn434_calc_ig__pgsrecin,)
    }
};
        var_fn434_calc_ig__pgsrecin = assign36350_e33372;

        let (assign36360_e33378,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (p.p257,)
    } else {
        (var_fn434_calc_ig__pg_param1,)
    }
};
        var_fn434_calc_ig__pg_param1 = assign36360_e33378;

        let (assign36370_e33384,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (p.p256,)
    } else {
        (var_fn434_calc_ig__vjg,)
    }
};
        var_fn434_calc_ig__vjg = assign36370_e33384;

        let (assign36380_e33390,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (p.p6,)
    } else {
        (var_fn434_calc_ig__type,)
    }
};
        var_fn434_calc_ig__type = assign36380_e33390;

        let (assign36390_e33396, assign36390_e33396_d_n4, assign36390_e33396_d_n8, assign36390_e33396_d_n17,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn434_calc_ig__igout, var_fn434_calc_ig__igout_dn4, var_fn434_calc_ig__igout_dn8, var_fn434_calc_ig__igout_dn17,)
    }
};
        var_fn434_calc_ig__igout = assign36390_e33396;
        var_fn434_calc_ig__igout_dn4 = assign36390_e33396_d_n4;
        var_fn434_calc_ig__igout_dn8 = assign36390_e33396_d_n8;
        var_fn434_calc_ig__igout_dn17 = assign36390_e33396_d_n17;

        let (assign36400_e33402, assign36400_e33402_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn434_calc_ig__alpha2_phit, var_fn434_calc_ig__alpha2_phit_dn4,)
    }
};
        var_fn434_calc_ig__alpha2_phit = assign36400_e33402;
        var_fn434_calc_ig__alpha2_phit_dn4 = assign36400_e33402_d_n4;

        let (assign36410_e33408, assign36410_e33408_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn434_calc_ig__t0, var_fn434_calc_ig__t0_dn4,)
    }
};
        var_fn434_calc_ig__t0 = assign36410_e33408;
        var_fn434_calc_ig__t0_dn4 = assign36410_e33408_d_n4;

        let (assign36420_e33414, assign36420_e33414_d_n4, assign36420_e33414_d_n8, assign36420_e33414_d_n17,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn434_calc_ig__ffvgin, var_fn434_calc_ig__ffvgin_dn4, var_fn434_calc_ig__ffvgin_dn8, var_fn434_calc_ig__ffvgin_dn17,)
    }
};
        var_fn434_calc_ig__ffvgin = assign36420_e33414;
        var_fn434_calc_ig__ffvgin_dn4 = assign36420_e33414_d_n4;
        var_fn434_calc_ig__ffvgin_dn8 = assign36420_e33414_d_n8;
        var_fn434_calc_ig__ffvgin_dn17 = assign36420_e33414_d_n17;

        let (assign36430_e33420, assign36430_e33420_d_n4, assign36430_e33420_d_n8, assign36430_e33420_d_n17,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn434_calc_ig__iginbd, var_fn434_calc_ig__iginbd_dn4, var_fn434_calc_ig__iginbd_dn8, var_fn434_calc_ig__iginbd_dn17,)
    }
};
        var_fn434_calc_ig__iginbd = assign36430_e33420;
        var_fn434_calc_ig__iginbd_dn4 = assign36430_e33420_d_n4;
        var_fn434_calc_ig__iginbd_dn8 = assign36430_e33420_d_n8;
        var_fn434_calc_ig__iginbd_dn17 = assign36430_e33420_d_n17;

        let (assign36440_e33426, assign36440_e33426_d_n4, assign36440_e33426_d_n8, assign36440_e33426_d_n17,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn434_calc_ig__igindiode, var_fn434_calc_ig__igindiode_dn4, var_fn434_calc_ig__igindiode_dn8, var_fn434_calc_ig__igindiode_dn17,)
    }
};
        var_fn434_calc_ig__igindiode = assign36440_e33426;
        var_fn434_calc_ig__igindiode_dn4 = assign36440_e33426_d_n4;
        var_fn434_calc_ig__igindiode_dn8 = assign36440_e33426_d_n8;
        var_fn434_calc_ig__igindiode_dn17 = assign36440_e33426_d_n17;

        let (assign36450_e33432, assign36450_e33432_d_n8, assign36450_e33432_d_n17,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_fn434_calc_ig__frecgin, var_fn434_calc_ig__frecgin_dn8, var_fn434_calc_ig__frecgin_dn17,)
    }
};
        var_fn434_calc_ig__frecgin = assign36450_e33432;
        var_fn434_calc_ig__frecgin_dn8 = assign36450_e33432_d_n8;
        var_fn434_calc_ig__frecgin_dn17 = assign36450_e33432_d_n17;

        let (assign36460_e33438, assign36460_e33438_d_n4, assign36460_e33438_d_n8, assign36460_e33438_d_n17,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn434_calc_ig__iginrec, var_fn434_calc_ig__iginrec_dn4, var_fn434_calc_ig__iginrec_dn8, var_fn434_calc_ig__iginrec_dn17,)
    }
};
        var_fn434_calc_ig__iginrec = assign36460_e33438;
        var_fn434_calc_ig__iginrec_dn4 = assign36460_e33438_d_n4;
        var_fn434_calc_ig__iginrec_dn8 = assign36460_e33438_d_n8;
        var_fn434_calc_ig__iginrec_dn17 = assign36460_e33438_d_n17;

        let (assign36470_e33444, assign36470_e33444_d_n4, assign36470_e33444_d_n8, assign36470_e33444_d_n17,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn434_calc_ig__expbdarg1, var_fn434_calc_ig__expbdarg1_dn4, var_fn434_calc_ig__expbdarg1_dn8, var_fn434_calc_ig__expbdarg1_dn17,)
    }
};
        var_fn434_calc_ig__expbdarg1 = assign36470_e33444;
        var_fn434_calc_ig__expbdarg1_dn4 = assign36470_e33444_d_n4;
        var_fn434_calc_ig__expbdarg1_dn8 = assign36470_e33444_d_n8;
        var_fn434_calc_ig__expbdarg1_dn17 = assign36470_e33444_d_n17;

        let (assign36480_e33450, assign36480_e33450_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn434_calc_ig__expbdarg2, var_fn434_calc_ig__expbdarg2_dn4,)
    }
};
        var_fn434_calc_ig__expbdarg2 = assign36480_e33450;
        var_fn434_calc_ig__expbdarg2_dn4 = assign36480_e33450_d_n4;

        let (assign36490_e33456, assign36490_e33456_d_n4, assign36490_e33456_d_n8, assign36490_e33456_d_n17,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn434_calc_ig__expbd1, var_fn434_calc_ig__expbd1_dn4, var_fn434_calc_ig__expbd1_dn8, var_fn434_calc_ig__expbd1_dn17,)
    }
};
        var_fn434_calc_ig__expbd1 = assign36490_e33456;
        var_fn434_calc_ig__expbd1_dn4 = assign36490_e33456_d_n4;
        var_fn434_calc_ig__expbd1_dn8 = assign36490_e33456_d_n8;
        var_fn434_calc_ig__expbd1_dn17 = assign36490_e33456_d_n17;

        *var_fn429_calc_ig__expirev_slot = var_fn429_calc_ig__expirev;
        *var_fn429_calc_ig__expirev_dn13_slot = var_fn429_calc_ig__expirev_dn13;
        *var_fn429_calc_ig__expirev_dn4_slot = var_fn429_calc_ig__expirev_dn4;
        *var_fn429_calc_ig__expirev_dn8_slot = var_fn429_calc_ig__expirev_dn8;
        *var_fn429_calc_ig__expirevarg_slot = var_fn429_calc_ig__expirevarg;
        *var_fn429_calc_ig__expirevarg_dn13_slot = var_fn429_calc_ig__expirevarg_dn13;
        *var_fn429_calc_ig__expirevarg_dn4_slot = var_fn429_calc_ig__expirevarg_dn4;
        *var_fn429_calc_ig__expirevarg_dn8_slot = var_fn429_calc_ig__expirevarg_dn8;
        *var_fn429_calc_ig__frecgin_slot = var_fn429_calc_ig__frecgin;
        *var_fn429_calc_ig__frecgin_dn13_slot = var_fn429_calc_ig__frecgin_dn13;
        *var_fn429_calc_ig__frecgin_dn8_slot = var_fn429_calc_ig__frecgin_dn8;
        *var_fn429_calc_ig__iginrec_slot = var_fn429_calc_ig__iginrec;
        *var_fn429_calc_ig__iginrec_dn13_slot = var_fn429_calc_ig__iginrec_dn13;
        *var_fn429_calc_ig__iginrec_dn4_slot = var_fn429_calc_ig__iginrec_dn4;
        *var_fn429_calc_ig__iginrec_dn8_slot = var_fn429_calc_ig__iginrec_dn8;
        *var_fn429_calc_ig__igout_slot = var_fn429_calc_ig__igout;
        *var_fn429_calc_ig__igout_dn13_slot = var_fn429_calc_ig__igout_dn13;
        *var_fn429_calc_ig__igout_dn4_slot = var_fn429_calc_ig__igout_dn4;
        *var_fn429_calc_ig__igout_dn8_slot = var_fn429_calc_ig__igout_dn8;
        *var_fn429_calc_ig__isrecout_slot = var_fn429_calc_ig__isrecout;
        *var_fn429_calc_ig__isrecout_dn4_slot = var_fn429_calc_ig__isrecout_dn4;
        *var_fn429_calc_ig__return_slot = var_fn429_calc_ig__return;
        *var_fn429_calc_ig__return_dn13_slot = var_fn429_calc_ig__return_dn13;
        *var_fn429_calc_ig__return_dn4_slot = var_fn429_calc_ig__return_dn4;
        *var_fn429_calc_ig__return_dn8_slot = var_fn429_calc_ig__return_dn8;
        *var_fn434_calc_ig__alpha2_phit_slot = var_fn434_calc_ig__alpha2_phit;
        *var_fn434_calc_ig__alpha2_phit_dn4_slot = var_fn434_calc_ig__alpha2_phit_dn4;
        *var_fn434_calc_ig__alphagin_slot = var_fn434_calc_ig__alphagin;
        *var_fn434_calc_ig__betarecin_slot = var_fn434_calc_ig__betarecin;
        *var_fn434_calc_ig__expbd1_slot = var_fn434_calc_ig__expbd1;
        *var_fn434_calc_ig__expbd1_dn17_slot = var_fn434_calc_ig__expbd1_dn17;
        *var_fn434_calc_ig__expbd1_dn4_slot = var_fn434_calc_ig__expbd1_dn4;
        *var_fn434_calc_ig__expbd1_dn8_slot = var_fn434_calc_ig__expbd1_dn8;
        *var_fn434_calc_ig__expbdarg1_slot = var_fn434_calc_ig__expbdarg1;
        *var_fn434_calc_ig__expbdarg1_dn17_slot = var_fn434_calc_ig__expbdarg1_dn17;
        *var_fn434_calc_ig__expbdarg1_dn4_slot = var_fn434_calc_ig__expbdarg1_dn4;
        *var_fn434_calc_ig__expbdarg1_dn8_slot = var_fn434_calc_ig__expbdarg1_dn8;
        *var_fn434_calc_ig__expbdarg2_slot = var_fn434_calc_ig__expbdarg2;
        *var_fn434_calc_ig__expbdarg2_dn4_slot = var_fn434_calc_ig__expbdarg2_dn4;
        *var_fn434_calc_ig__ffvgin_slot = var_fn434_calc_ig__ffvgin;
        *var_fn434_calc_ig__ffvgin_dn17_slot = var_fn434_calc_ig__ffvgin_dn17;
        *var_fn434_calc_ig__ffvgin_dn4_slot = var_fn434_calc_ig__ffvgin_dn4;
        *var_fn434_calc_ig__ffvgin_dn8_slot = var_fn434_calc_ig__ffvgin_dn8;
        *var_fn434_calc_ig__fracin_slot = var_fn434_calc_ig__fracin;
        *var_fn434_calc_ig__frecgin_slot = var_fn434_calc_ig__frecgin;
        *var_fn434_calc_ig__frecgin_dn17_slot = var_fn434_calc_ig__frecgin_dn17;
        *var_fn434_calc_ig__frecgin_dn8_slot = var_fn434_calc_ig__frecgin_dn8;
        *var_fn434_calc_ig__iginbd_slot = var_fn434_calc_ig__iginbd;
        *var_fn434_calc_ig__iginbd_dn17_slot = var_fn434_calc_ig__iginbd_dn17;
        *var_fn434_calc_ig__iginbd_dn4_slot = var_fn434_calc_ig__iginbd_dn4;
        *var_fn434_calc_ig__iginbd_dn8_slot = var_fn434_calc_ig__iginbd_dn8;
        *var_fn434_calc_ig__igindiode_slot = var_fn434_calc_ig__igindiode;
        *var_fn434_calc_ig__igindiode_dn17_slot = var_fn434_calc_ig__igindiode_dn17;
        *var_fn434_calc_ig__igindiode_dn4_slot = var_fn434_calc_ig__igindiode_dn4;
        *var_fn434_calc_ig__igindiode_dn8_slot = var_fn434_calc_ig__igindiode_dn8;
        *var_fn434_calc_ig__iginrec_slot = var_fn434_calc_ig__iginrec;
        *var_fn434_calc_ig__iginrec_dn17_slot = var_fn434_calc_ig__iginrec_dn17;
        *var_fn434_calc_ig__iginrec_dn4_slot = var_fn434_calc_ig__iginrec_dn4;
        *var_fn434_calc_ig__iginrec_dn8_slot = var_fn434_calc_ig__iginrec_dn8;
        *var_fn434_calc_ig__igout_slot = var_fn434_calc_ig__igout;
        *var_fn434_calc_ig__igout_dn17_slot = var_fn434_calc_ig__igout_dn17;
        *var_fn434_calc_ig__igout_dn4_slot = var_fn434_calc_ig__igout_dn4;
        *var_fn434_calc_ig__igout_dn8_slot = var_fn434_calc_ig__igout_dn8;
        *var_fn434_calc_ig__ijin_slot = var_fn434_calc_ig__ijin;
        *var_fn434_calc_ig__irecin_slot = var_fn434_calc_ig__irecin;
        *var_fn434_calc_ig__isdiodeout_slot = var_fn434_calc_ig__isdiodeout;
        *var_fn434_calc_ig__isdiodeout_dn4_slot = var_fn434_calc_ig__isdiodeout_dn4;
        *var_fn434_calc_ig__isrecout_slot = var_fn434_calc_ig__isrecout;
        *var_fn434_calc_ig__isrecout_dn4_slot = var_fn434_calc_ig__isrecout_dn4;
        *var_fn434_calc_ig__kbdgatein_slot = var_fn434_calc_ig__kbdgatein;
        *var_fn434_calc_ig__ngf_slot = var_fn434_calc_ig__ngf;
        *var_fn434_calc_ig__pbdgin_slot = var_fn434_calc_ig__pbdgin;
        *var_fn434_calc_ig__pg_param1_slot = var_fn434_calc_ig__pg_param1;
        *var_fn434_calc_ig__pg_paramin_slot = var_fn434_calc_ig__pg_paramin;
        *var_fn434_calc_ig__pgsrecin_slot = var_fn434_calc_ig__pgsrecin;
        *var_fn434_calc_ig__phitin_slot = var_fn434_calc_ig__phitin;
        *var_fn434_calc_ig__phitin_dn4_slot = var_fn434_calc_ig__phitin_dn4;
        *var_fn434_calc_ig__return_slot = var_fn434_calc_ig__return;
        *var_fn434_calc_ig__return_dn17_slot = var_fn434_calc_ig__return_dn17;
        *var_fn434_calc_ig__return_dn4_slot = var_fn434_calc_ig__return_dn4;
        *var_fn434_calc_ig__return_dn8_slot = var_fn434_calc_ig__return_dn8;
        *var_fn434_calc_ig__t0_slot = var_fn434_calc_ig__t0;
        *var_fn434_calc_ig__t0_dn4_slot = var_fn434_calc_ig__t0_dn4;
        *var_fn434_calc_ig__tfacdiodein_slot = var_fn434_calc_ig__tfacdiodein;
        *var_fn434_calc_ig__tfacdiodein_dn4_slot = var_fn434_calc_ig__tfacdiodein_dn4;
        *var_fn434_calc_ig__type_slot = var_fn434_calc_ig__type;
        *var_fn434_calc_ig__vbdgin_slot = var_fn434_calc_ig__vbdgin;
        *var_fn434_calc_ig__vgin_slot = var_fn434_calc_ig__vgin;
        *var_fn434_calc_ig__vgin_dn17_slot = var_fn434_calc_ig__vgin_dn17;
        *var_fn434_calc_ig__vgin_dn8_slot = var_fn434_calc_ig__vgin_dn8;
        *var_fn434_calc_ig__vgsatin_slot = var_fn434_calc_ig__vgsatin;
        *var_fn434_calc_ig__vgsatqin_slot = var_fn434_calc_ig__vgsatqin;
        *var_fn434_calc_ig__vjg_slot = var_fn434_calc_ig__vjg;
        *var_fn434_calc_ig__w_slot = var_fn434_calc_ig__w;
        *var_igsi2_slot = var_igsi2;
        *var_igsi2_dn13_slot = var_igsi2_dn13;
        *var_igsi2_dn4_slot = var_igsi2_dn4;
        *var_igsi2_dn8_slot = var_igsi2_dn8;
    }

    pub(super) fn stamp_transient_block_90(
        var_fn434_calc_ig__fracin: f64,
        var_fn434_calc_ig__ijin: f64,
        var_fn434_calc_ig__kbdgatein: f64,
        var_fn434_calc_ig__ngf: f64,
        var_fn434_calc_ig__pbdgin: f64,
        var_fn434_calc_ig__pg_param1: f64,
        var_fn434_calc_ig__pg_paramin: f64,
        var_fn434_calc_ig__phitin: f64,
        var_fn434_calc_ig__phitin_dn4: f64,
        var_fn434_calc_ig__tfacdiodein: f64,
        var_fn434_calc_ig__tfacdiodein_dn4: f64,
        var_fn434_calc_ig__type: f64,
        var_fn434_calc_ig__vbdgin: f64,
        var_fn434_calc_ig__vgin: f64,
        var_fn434_calc_ig__vgin_dn17: f64,
        var_fn434_calc_ig__vgin_dn8: f64,
        var_fn434_calc_ig__vjg: f64,
        var_fn434_calc_ig__w: f64,
        var_guard417: f64,
        var_guard428: f64,
        var_fn434_calc_ig__expbd1_slot: &mut f64,
        var_fn434_calc_ig__expbd1_dn17_slot: &mut f64,
        var_fn434_calc_ig__expbd1_dn4_slot: &mut f64,
        var_fn434_calc_ig__expbd1_dn8_slot: &mut f64,
        var_fn434_calc_ig__expbd1_vgsat_slot: &mut f64,
        var_fn434_calc_ig__expbd1_vgsat_dn4_slot: &mut f64,
        var_fn434_calc_ig__expbd2_slot: &mut f64,
        var_fn434_calc_ig__expbd2_dn4_slot: &mut f64,
        var_fn434_calc_ig__expbdarg1_slot: &mut f64,
        var_fn434_calc_ig__expbdarg1_dn17_slot: &mut f64,
        var_fn434_calc_ig__expbdarg1_dn4_slot: &mut f64,
        var_fn434_calc_ig__expbdarg1_dn8_slot: &mut f64,
        var_fn434_calc_ig__expbdarg1_vgsat_slot: &mut f64,
        var_fn434_calc_ig__expbdarg1_vgsat_dn4_slot: &mut f64,
        var_fn434_calc_ig__expbdarg2_slot: &mut f64,
        var_fn434_calc_ig__expbdarg2_dn4_slot: &mut f64,
        var_fn434_calc_ig__expffvarg_slot: &mut f64,
        var_fn434_calc_ig__expffvarg_dn17_slot: &mut f64,
        var_fn434_calc_ig__expffvarg_dn4_slot: &mut f64,
        var_fn434_calc_ig__expffvarg_dn8_slot: &mut f64,
        var_fn434_calc_ig__expifor_slot: &mut f64,
        var_fn434_calc_ig__expifor_dn17_slot: &mut f64,
        var_fn434_calc_ig__expifor_dn4_slot: &mut f64,
        var_fn434_calc_ig__expifor_dn8_slot: &mut f64,
        var_fn434_calc_ig__expifor_hinj_slot: &mut f64,
        var_fn434_calc_ig__expifor_hinj_dn17_slot: &mut f64,
        var_fn434_calc_ig__expifor_hinj_dn4_slot: &mut f64,
        var_fn434_calc_ig__expifor_hinj_dn8_slot: &mut f64,
        var_fn434_calc_ig__expifor_hinj_vgsat_slot: &mut f64,
        var_fn434_calc_ig__expifor_hinj_vgsat_dn4_slot: &mut f64,
        var_fn434_calc_ig__expifor_nohinj_vgsat_slot: &mut f64,
        var_fn434_calc_ig__expifor_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn434_calc_ig__expiforarg_slot: &mut f64,
        var_fn434_calc_ig__expiforarg_dn17_slot: &mut f64,
        var_fn434_calc_ig__expiforarg_dn4_slot: &mut f64,
        var_fn434_calc_ig__expiforarg_dn8_slot: &mut f64,
        var_fn434_calc_ig__expiforarg_hinj_slot: &mut f64,
        var_fn434_calc_ig__expiforarg_hinj_dn17_slot: &mut f64,
        var_fn434_calc_ig__expiforarg_hinj_dn4_slot: &mut f64,
        var_fn434_calc_ig__expiforarg_hinj_dn8_slot: &mut f64,
        var_fn434_calc_ig__expiforarg_hinj_vgsat_slot: &mut f64,
        var_fn434_calc_ig__expiforarg_hinj_vgsat_dn4_slot: &mut f64,
        var_fn434_calc_ig__expiforarg_nohinj_vgsat_slot: &mut f64,
        var_fn434_calc_ig__expiforarg_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn434_calc_ig__expirev_slot: &mut f64,
        var_fn434_calc_ig__expirev_dn17_slot: &mut f64,
        var_fn434_calc_ig__expirev_dn4_slot: &mut f64,
        var_fn434_calc_ig__expirev_dn8_slot: &mut f64,
        var_fn434_calc_ig__expirevarg_slot: &mut f64,
        var_fn434_calc_ig__expirevarg_dn17_slot: &mut f64,
        var_fn434_calc_ig__expirevarg_dn4_slot: &mut f64,
        var_fn434_calc_ig__expirevarg_dn8_slot: &mut f64,
        var_fn434_calc_ig__expphib_slot: &mut f64,
        var_fn434_calc_ig__expphib_dn4_slot: &mut f64,
        var_fn434_calc_ig__iginbd_slot: &mut f64,
        var_fn434_calc_ig__iginbd_dn17_slot: &mut f64,
        var_fn434_calc_ig__iginbd_dn4_slot: &mut f64,
        var_fn434_calc_ig__iginbd_dn8_slot: &mut f64,
        var_fn434_calc_ig__iginbd_vgsat_slot: &mut f64,
        var_fn434_calc_ig__iginbd_vgsat_dn4_slot: &mut f64,
        var_fn434_calc_ig__igindiode_slot: &mut f64,
        var_fn434_calc_ig__igindiode_dn17_slot: &mut f64,
        var_fn434_calc_ig__igindiode_dn4_slot: &mut f64,
        var_fn434_calc_ig__igindiode_dn8_slot: &mut f64,
        var_fn434_calc_ig__igindiode_hinj_slot: &mut f64,
        var_fn434_calc_ig__igindiode_hinj_dn17_slot: &mut f64,
        var_fn434_calc_ig__igindiode_hinj_dn4_slot: &mut f64,
        var_fn434_calc_ig__igindiode_hinj_dn8_slot: &mut f64,
        var_fn434_calc_ig__igindiode_hinj_pre_slot: &mut f64,
        var_fn434_calc_ig__igindiode_hinj_pre_dn4_slot: &mut f64,
        var_fn434_calc_ig__igindiode_hinj_vgsat_slot: &mut f64,
        var_fn434_calc_ig__igindiode_hinj_vgsat_dn4_slot: &mut f64,
        var_fn434_calc_ig__igindiode_nohinj_slot: &mut f64,
        var_fn434_calc_ig__igindiode_nohinj_dn17_slot: &mut f64,
        var_fn434_calc_ig__igindiode_nohinj_dn4_slot: &mut f64,
        var_fn434_calc_ig__igindiode_nohinj_dn8_slot: &mut f64,
        var_fn434_calc_ig__igindiode_nohinj_vgsat_slot: &mut f64,
        var_fn434_calc_ig__igindiode_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn434_calc_ig__isdiodeout_slot: &mut f64,
        var_fn434_calc_ig__isdiodeout_dn4_slot: &mut f64,
        var_fn434_calc_ig__pg_paramin_hinj_slot: &mut f64,
        var_fn434_calc_ig__t0_slot: &mut f64,
        var_fn434_calc_ig__t0_dn4_slot: &mut f64,
        var_guard435_slot: &mut f64,
    ) {
        let mut var_fn434_calc_ig__expbd1: f64 = *var_fn434_calc_ig__expbd1_slot;
        let mut var_fn434_calc_ig__expbd1_dn17: f64 = *var_fn434_calc_ig__expbd1_dn17_slot;
        let mut var_fn434_calc_ig__expbd1_dn4: f64 = *var_fn434_calc_ig__expbd1_dn4_slot;
        let mut var_fn434_calc_ig__expbd1_dn8: f64 = *var_fn434_calc_ig__expbd1_dn8_slot;
        let mut var_fn434_calc_ig__expbd1_vgsat: f64 = *var_fn434_calc_ig__expbd1_vgsat_slot;
        let mut var_fn434_calc_ig__expbd1_vgsat_dn4: f64 = *var_fn434_calc_ig__expbd1_vgsat_dn4_slot;
        let mut var_fn434_calc_ig__expbd2: f64 = *var_fn434_calc_ig__expbd2_slot;
        let mut var_fn434_calc_ig__expbd2_dn4: f64 = *var_fn434_calc_ig__expbd2_dn4_slot;
        let mut var_fn434_calc_ig__expbdarg1: f64 = *var_fn434_calc_ig__expbdarg1_slot;
        let mut var_fn434_calc_ig__expbdarg1_dn17: f64 = *var_fn434_calc_ig__expbdarg1_dn17_slot;
        let mut var_fn434_calc_ig__expbdarg1_dn4: f64 = *var_fn434_calc_ig__expbdarg1_dn4_slot;
        let mut var_fn434_calc_ig__expbdarg1_dn8: f64 = *var_fn434_calc_ig__expbdarg1_dn8_slot;
        let mut var_fn434_calc_ig__expbdarg1_vgsat: f64 = *var_fn434_calc_ig__expbdarg1_vgsat_slot;
        let mut var_fn434_calc_ig__expbdarg1_vgsat_dn4: f64 = *var_fn434_calc_ig__expbdarg1_vgsat_dn4_slot;
        let mut var_fn434_calc_ig__expbdarg2: f64 = *var_fn434_calc_ig__expbdarg2_slot;
        let mut var_fn434_calc_ig__expbdarg2_dn4: f64 = *var_fn434_calc_ig__expbdarg2_dn4_slot;
        let mut var_fn434_calc_ig__expffvarg: f64 = *var_fn434_calc_ig__expffvarg_slot;
        let mut var_fn434_calc_ig__expffvarg_dn17: f64 = *var_fn434_calc_ig__expffvarg_dn17_slot;
        let mut var_fn434_calc_ig__expffvarg_dn4: f64 = *var_fn434_calc_ig__expffvarg_dn4_slot;
        let mut var_fn434_calc_ig__expffvarg_dn8: f64 = *var_fn434_calc_ig__expffvarg_dn8_slot;
        let mut var_fn434_calc_ig__expifor: f64 = *var_fn434_calc_ig__expifor_slot;
        let mut var_fn434_calc_ig__expifor_dn17: f64 = *var_fn434_calc_ig__expifor_dn17_slot;
        let mut var_fn434_calc_ig__expifor_dn4: f64 = *var_fn434_calc_ig__expifor_dn4_slot;
        let mut var_fn434_calc_ig__expifor_dn8: f64 = *var_fn434_calc_ig__expifor_dn8_slot;
        let mut var_fn434_calc_ig__expifor_hinj: f64 = *var_fn434_calc_ig__expifor_hinj_slot;
        let mut var_fn434_calc_ig__expifor_hinj_dn17: f64 = *var_fn434_calc_ig__expifor_hinj_dn17_slot;
        let mut var_fn434_calc_ig__expifor_hinj_dn4: f64 = *var_fn434_calc_ig__expifor_hinj_dn4_slot;
        let mut var_fn434_calc_ig__expifor_hinj_dn8: f64 = *var_fn434_calc_ig__expifor_hinj_dn8_slot;
        let mut var_fn434_calc_ig__expifor_hinj_vgsat: f64 = *var_fn434_calc_ig__expifor_hinj_vgsat_slot;
        let mut var_fn434_calc_ig__expifor_hinj_vgsat_dn4: f64 = *var_fn434_calc_ig__expifor_hinj_vgsat_dn4_slot;
        let mut var_fn434_calc_ig__expifor_nohinj_vgsat: f64 = *var_fn434_calc_ig__expifor_nohinj_vgsat_slot;
        let mut var_fn434_calc_ig__expifor_nohinj_vgsat_dn4: f64 = *var_fn434_calc_ig__expifor_nohinj_vgsat_dn4_slot;
        let mut var_fn434_calc_ig__expiforarg: f64 = *var_fn434_calc_ig__expiforarg_slot;
        let mut var_fn434_calc_ig__expiforarg_dn17: f64 = *var_fn434_calc_ig__expiforarg_dn17_slot;
        let mut var_fn434_calc_ig__expiforarg_dn4: f64 = *var_fn434_calc_ig__expiforarg_dn4_slot;
        let mut var_fn434_calc_ig__expiforarg_dn8: f64 = *var_fn434_calc_ig__expiforarg_dn8_slot;
        let mut var_fn434_calc_ig__expiforarg_hinj: f64 = *var_fn434_calc_ig__expiforarg_hinj_slot;
        let mut var_fn434_calc_ig__expiforarg_hinj_dn17: f64 = *var_fn434_calc_ig__expiforarg_hinj_dn17_slot;
        let mut var_fn434_calc_ig__expiforarg_hinj_dn4: f64 = *var_fn434_calc_ig__expiforarg_hinj_dn4_slot;
        let mut var_fn434_calc_ig__expiforarg_hinj_dn8: f64 = *var_fn434_calc_ig__expiforarg_hinj_dn8_slot;
        let mut var_fn434_calc_ig__expiforarg_hinj_vgsat: f64 = *var_fn434_calc_ig__expiforarg_hinj_vgsat_slot;
        let mut var_fn434_calc_ig__expiforarg_hinj_vgsat_dn4: f64 = *var_fn434_calc_ig__expiforarg_hinj_vgsat_dn4_slot;
        let mut var_fn434_calc_ig__expiforarg_nohinj_vgsat: f64 = *var_fn434_calc_ig__expiforarg_nohinj_vgsat_slot;
        let mut var_fn434_calc_ig__expiforarg_nohinj_vgsat_dn4: f64 = *var_fn434_calc_ig__expiforarg_nohinj_vgsat_dn4_slot;
        let mut var_fn434_calc_ig__expirev: f64 = *var_fn434_calc_ig__expirev_slot;
        let mut var_fn434_calc_ig__expirev_dn17: f64 = *var_fn434_calc_ig__expirev_dn17_slot;
        let mut var_fn434_calc_ig__expirev_dn4: f64 = *var_fn434_calc_ig__expirev_dn4_slot;
        let mut var_fn434_calc_ig__expirev_dn8: f64 = *var_fn434_calc_ig__expirev_dn8_slot;
        let mut var_fn434_calc_ig__expirevarg: f64 = *var_fn434_calc_ig__expirevarg_slot;
        let mut var_fn434_calc_ig__expirevarg_dn17: f64 = *var_fn434_calc_ig__expirevarg_dn17_slot;
        let mut var_fn434_calc_ig__expirevarg_dn4: f64 = *var_fn434_calc_ig__expirevarg_dn4_slot;
        let mut var_fn434_calc_ig__expirevarg_dn8: f64 = *var_fn434_calc_ig__expirevarg_dn8_slot;
        let mut var_fn434_calc_ig__expphib: f64 = *var_fn434_calc_ig__expphib_slot;
        let mut var_fn434_calc_ig__expphib_dn4: f64 = *var_fn434_calc_ig__expphib_dn4_slot;
        let mut var_fn434_calc_ig__iginbd: f64 = *var_fn434_calc_ig__iginbd_slot;
        let mut var_fn434_calc_ig__iginbd_dn17: f64 = *var_fn434_calc_ig__iginbd_dn17_slot;
        let mut var_fn434_calc_ig__iginbd_dn4: f64 = *var_fn434_calc_ig__iginbd_dn4_slot;
        let mut var_fn434_calc_ig__iginbd_dn8: f64 = *var_fn434_calc_ig__iginbd_dn8_slot;
        let mut var_fn434_calc_ig__iginbd_vgsat: f64 = *var_fn434_calc_ig__iginbd_vgsat_slot;
        let mut var_fn434_calc_ig__iginbd_vgsat_dn4: f64 = *var_fn434_calc_ig__iginbd_vgsat_dn4_slot;
        let mut var_fn434_calc_ig__igindiode: f64 = *var_fn434_calc_ig__igindiode_slot;
        let mut var_fn434_calc_ig__igindiode_dn17: f64 = *var_fn434_calc_ig__igindiode_dn17_slot;
        let mut var_fn434_calc_ig__igindiode_dn4: f64 = *var_fn434_calc_ig__igindiode_dn4_slot;
        let mut var_fn434_calc_ig__igindiode_dn8: f64 = *var_fn434_calc_ig__igindiode_dn8_slot;
        let mut var_fn434_calc_ig__igindiode_hinj: f64 = *var_fn434_calc_ig__igindiode_hinj_slot;
        let mut var_fn434_calc_ig__igindiode_hinj_dn17: f64 = *var_fn434_calc_ig__igindiode_hinj_dn17_slot;
        let mut var_fn434_calc_ig__igindiode_hinj_dn4: f64 = *var_fn434_calc_ig__igindiode_hinj_dn4_slot;
        let mut var_fn434_calc_ig__igindiode_hinj_dn8: f64 = *var_fn434_calc_ig__igindiode_hinj_dn8_slot;
        let mut var_fn434_calc_ig__igindiode_hinj_pre: f64 = *var_fn434_calc_ig__igindiode_hinj_pre_slot;
        let mut var_fn434_calc_ig__igindiode_hinj_pre_dn4: f64 = *var_fn434_calc_ig__igindiode_hinj_pre_dn4_slot;
        let mut var_fn434_calc_ig__igindiode_hinj_vgsat: f64 = *var_fn434_calc_ig__igindiode_hinj_vgsat_slot;
        let mut var_fn434_calc_ig__igindiode_hinj_vgsat_dn4: f64 = *var_fn434_calc_ig__igindiode_hinj_vgsat_dn4_slot;
        let mut var_fn434_calc_ig__igindiode_nohinj: f64 = *var_fn434_calc_ig__igindiode_nohinj_slot;
        let mut var_fn434_calc_ig__igindiode_nohinj_dn17: f64 = *var_fn434_calc_ig__igindiode_nohinj_dn17_slot;
        let mut var_fn434_calc_ig__igindiode_nohinj_dn4: f64 = *var_fn434_calc_ig__igindiode_nohinj_dn4_slot;
        let mut var_fn434_calc_ig__igindiode_nohinj_dn8: f64 = *var_fn434_calc_ig__igindiode_nohinj_dn8_slot;
        let mut var_fn434_calc_ig__igindiode_nohinj_vgsat: f64 = *var_fn434_calc_ig__igindiode_nohinj_vgsat_slot;
        let mut var_fn434_calc_ig__igindiode_nohinj_vgsat_dn4: f64 = *var_fn434_calc_ig__igindiode_nohinj_vgsat_dn4_slot;
        let mut var_fn434_calc_ig__isdiodeout: f64 = *var_fn434_calc_ig__isdiodeout_slot;
        let mut var_fn434_calc_ig__isdiodeout_dn4: f64 = *var_fn434_calc_ig__isdiodeout_dn4_slot;
        let mut var_fn434_calc_ig__pg_paramin_hinj: f64 = *var_fn434_calc_ig__pg_paramin_hinj_slot;
        let mut var_fn434_calc_ig__t0: f64 = *var_fn434_calc_ig__t0_slot;
        let mut var_fn434_calc_ig__t0_dn4: f64 = *var_fn434_calc_ig__t0_dn4_slot;
        let mut var_guard435: f64 = *var_guard435_slot;

        let (assign36500_e33462, assign36500_e33462_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn434_calc_ig__expbd2, var_fn434_calc_ig__expbd2_dn4,)
    }
};
        var_fn434_calc_ig__expbd2 = assign36500_e33462;
        var_fn434_calc_ig__expbd2_dn4 = assign36500_e33462_d_n4;

        let (assign36510_e33468, assign36510_e33468_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn434_calc_ig__expphib, var_fn434_calc_ig__expphib_dn4,)
    }
};
        var_fn434_calc_ig__expphib = assign36510_e33468;
        var_fn434_calc_ig__expphib_dn4 = assign36510_e33468_d_n4;

        let (assign36520_e33474, assign36520_e33474_d_n4, assign36520_e33474_d_n8, assign36520_e33474_d_n17,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn434_calc_ig__expffvarg, var_fn434_calc_ig__expffvarg_dn4, var_fn434_calc_ig__expffvarg_dn8, var_fn434_calc_ig__expffvarg_dn17,)
    }
};
        var_fn434_calc_ig__expffvarg = assign36520_e33474;
        var_fn434_calc_ig__expffvarg_dn4 = assign36520_e33474_d_n4;
        var_fn434_calc_ig__expffvarg_dn8 = assign36520_e33474_d_n8;
        var_fn434_calc_ig__expffvarg_dn17 = assign36520_e33474_d_n17;

        let (assign36530_e33480, assign36530_e33480_d_n4, assign36530_e33480_d_n8, assign36530_e33480_d_n17,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn434_calc_ig__expiforarg, var_fn434_calc_ig__expiforarg_dn4, var_fn434_calc_ig__expiforarg_dn8, var_fn434_calc_ig__expiforarg_dn17,)
    }
};
        var_fn434_calc_ig__expiforarg = assign36530_e33480;
        var_fn434_calc_ig__expiforarg_dn4 = assign36530_e33480_d_n4;
        var_fn434_calc_ig__expiforarg_dn8 = assign36530_e33480_d_n8;
        var_fn434_calc_ig__expiforarg_dn17 = assign36530_e33480_d_n17;

        let (assign36540_e33486, assign36540_e33486_d_n4, assign36540_e33486_d_n8, assign36540_e33486_d_n17,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn434_calc_ig__expifor, var_fn434_calc_ig__expifor_dn4, var_fn434_calc_ig__expifor_dn8, var_fn434_calc_ig__expifor_dn17,)
    }
};
        var_fn434_calc_ig__expifor = assign36540_e33486;
        var_fn434_calc_ig__expifor_dn4 = assign36540_e33486_d_n4;
        var_fn434_calc_ig__expifor_dn8 = assign36540_e33486_d_n8;
        var_fn434_calc_ig__expifor_dn17 = assign36540_e33486_d_n17;

        let (assign36550_e33492, assign36550_e33492_d_n4, assign36550_e33492_d_n8, assign36550_e33492_d_n17,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn434_calc_ig__expirevarg, var_fn434_calc_ig__expirevarg_dn4, var_fn434_calc_ig__expirevarg_dn8, var_fn434_calc_ig__expirevarg_dn17,)
    }
};
        var_fn434_calc_ig__expirevarg = assign36550_e33492;
        var_fn434_calc_ig__expirevarg_dn4 = assign36550_e33492_d_n4;
        var_fn434_calc_ig__expirevarg_dn8 = assign36550_e33492_d_n8;
        var_fn434_calc_ig__expirevarg_dn17 = assign36550_e33492_d_n17;

        let (assign36560_e33498, assign36560_e33498_d_n4, assign36560_e33498_d_n8, assign36560_e33498_d_n17,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn434_calc_ig__expirev, var_fn434_calc_ig__expirev_dn4, var_fn434_calc_ig__expirev_dn8, var_fn434_calc_ig__expirev_dn17,)
    }
};
        var_fn434_calc_ig__expirev = assign36560_e33498;
        var_fn434_calc_ig__expirev_dn4 = assign36560_e33498_d_n4;
        var_fn434_calc_ig__expirev_dn8 = assign36560_e33498_d_n8;
        var_fn434_calc_ig__expirev_dn17 = assign36560_e33498_d_n17;

        let (assign36570_e33504,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0,)
    } else {
        (var_fn434_calc_ig__pg_paramin_hinj,)
    }
};
        var_fn434_calc_ig__pg_paramin_hinj = assign36570_e33504;

        let (assign36580_e33510, assign36580_e33510_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn434_calc_ig__expbdarg1_vgsat, var_fn434_calc_ig__expbdarg1_vgsat_dn4,)
    }
};
        var_fn434_calc_ig__expbdarg1_vgsat = assign36580_e33510;
        var_fn434_calc_ig__expbdarg1_vgsat_dn4 = assign36580_e33510_d_n4;

        let (assign36590_e33516, assign36590_e33516_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn434_calc_ig__expbd1_vgsat, var_fn434_calc_ig__expbd1_vgsat_dn4,)
    }
};
        var_fn434_calc_ig__expbd1_vgsat = assign36590_e33516;
        var_fn434_calc_ig__expbd1_vgsat_dn4 = assign36590_e33516_d_n4;

        let (assign36600_e33522, assign36600_e33522_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn434_calc_ig__iginbd_vgsat, var_fn434_calc_ig__iginbd_vgsat_dn4,)
    }
};
        var_fn434_calc_ig__iginbd_vgsat = assign36600_e33522;
        var_fn434_calc_ig__iginbd_vgsat_dn4 = assign36600_e33522_d_n4;

        let (assign36610_e33528, assign36610_e33528_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn434_calc_ig__expiforarg_nohinj_vgsat, var_fn434_calc_ig__expiforarg_nohinj_vgsat_dn4,)
    }
};
        var_fn434_calc_ig__expiforarg_nohinj_vgsat = assign36610_e33528;
        var_fn434_calc_ig__expiforarg_nohinj_vgsat_dn4 = assign36610_e33528_d_n4;

        let (assign36620_e33534, assign36620_e33534_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn434_calc_ig__expifor_nohinj_vgsat, var_fn434_calc_ig__expifor_nohinj_vgsat_dn4,)
    }
};
        var_fn434_calc_ig__expifor_nohinj_vgsat = assign36620_e33534;
        var_fn434_calc_ig__expifor_nohinj_vgsat_dn4 = assign36620_e33534_d_n4;

        let (assign36630_e33540, assign36630_e33540_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn434_calc_ig__igindiode_nohinj_vgsat, var_fn434_calc_ig__igindiode_nohinj_vgsat_dn4,)
    }
};
        var_fn434_calc_ig__igindiode_nohinj_vgsat = assign36630_e33540;
        var_fn434_calc_ig__igindiode_nohinj_vgsat_dn4 = assign36630_e33540_d_n4;

        let (assign36640_e33546, assign36640_e33546_d_n4, assign36640_e33546_d_n8, assign36640_e33546_d_n17,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn434_calc_ig__igindiode_nohinj, var_fn434_calc_ig__igindiode_nohinj_dn4, var_fn434_calc_ig__igindiode_nohinj_dn8, var_fn434_calc_ig__igindiode_nohinj_dn17,)
    }
};
        var_fn434_calc_ig__igindiode_nohinj = assign36640_e33546;
        var_fn434_calc_ig__igindiode_nohinj_dn4 = assign36640_e33546_d_n4;
        var_fn434_calc_ig__igindiode_nohinj_dn8 = assign36640_e33546_d_n8;
        var_fn434_calc_ig__igindiode_nohinj_dn17 = assign36640_e33546_d_n17;

        let (assign36650_e33552, assign36650_e33552_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn434_calc_ig__expiforarg_hinj_vgsat, var_fn434_calc_ig__expiforarg_hinj_vgsat_dn4,)
    }
};
        var_fn434_calc_ig__expiforarg_hinj_vgsat = assign36650_e33552;
        var_fn434_calc_ig__expiforarg_hinj_vgsat_dn4 = assign36650_e33552_d_n4;

        let (assign36660_e33558, assign36660_e33558_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn434_calc_ig__expifor_hinj_vgsat, var_fn434_calc_ig__expifor_hinj_vgsat_dn4,)
    }
};
        var_fn434_calc_ig__expifor_hinj_vgsat = assign36660_e33558;
        var_fn434_calc_ig__expifor_hinj_vgsat_dn4 = assign36660_e33558_d_n4;

        let (assign36670_e33564, assign36670_e33564_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn434_calc_ig__igindiode_hinj_vgsat, var_fn434_calc_ig__igindiode_hinj_vgsat_dn4,)
    }
};
        var_fn434_calc_ig__igindiode_hinj_vgsat = assign36670_e33564;
        var_fn434_calc_ig__igindiode_hinj_vgsat_dn4 = assign36670_e33564_d_n4;

        let (assign36680_e33570, assign36680_e33570_d_n4, assign36680_e33570_d_n8, assign36680_e33570_d_n17,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn434_calc_ig__expiforarg_hinj, var_fn434_calc_ig__expiforarg_hinj_dn4, var_fn434_calc_ig__expiforarg_hinj_dn8, var_fn434_calc_ig__expiforarg_hinj_dn17,)
    }
};
        var_fn434_calc_ig__expiforarg_hinj = assign36680_e33570;
        var_fn434_calc_ig__expiforarg_hinj_dn4 = assign36680_e33570_d_n4;
        var_fn434_calc_ig__expiforarg_hinj_dn8 = assign36680_e33570_d_n8;
        var_fn434_calc_ig__expiforarg_hinj_dn17 = assign36680_e33570_d_n17;

        let (assign36690_e33576, assign36690_e33576_d_n4, assign36690_e33576_d_n8, assign36690_e33576_d_n17,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn434_calc_ig__expifor_hinj, var_fn434_calc_ig__expifor_hinj_dn4, var_fn434_calc_ig__expifor_hinj_dn8, var_fn434_calc_ig__expifor_hinj_dn17,)
    }
};
        var_fn434_calc_ig__expifor_hinj = assign36690_e33576;
        var_fn434_calc_ig__expifor_hinj_dn4 = assign36690_e33576_d_n4;
        var_fn434_calc_ig__expifor_hinj_dn8 = assign36690_e33576_d_n8;
        var_fn434_calc_ig__expifor_hinj_dn17 = assign36690_e33576_d_n17;

        let (assign36700_e33582, assign36700_e33582_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn434_calc_ig__igindiode_hinj_pre, var_fn434_calc_ig__igindiode_hinj_pre_dn4,)
    }
};
        var_fn434_calc_ig__igindiode_hinj_pre = assign36700_e33582;
        var_fn434_calc_ig__igindiode_hinj_pre_dn4 = assign36700_e33582_d_n4;

        let (assign36710_e33588, assign36710_e33588_d_n4, assign36710_e33588_d_n8, assign36710_e33588_d_n17,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn434_calc_ig__igindiode_hinj, var_fn434_calc_ig__igindiode_hinj_dn4, var_fn434_calc_ig__igindiode_hinj_dn8, var_fn434_calc_ig__igindiode_hinj_dn17,)
    }
};
        var_fn434_calc_ig__igindiode_hinj = assign36710_e33588;
        var_fn434_calc_ig__igindiode_hinj_dn4 = assign36710_e33588_d_n4;
        var_fn434_calc_ig__igindiode_hinj_dn8 = assign36710_e33588_d_n8;
        var_fn434_calc_ig__igindiode_hinj_dn17 = assign36710_e33588_d_n17;

        let (assign36720_e33599, assign36720_e33599_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        let assign36720_e33594: f64 = (var_fn434_calc_ig__pg_param1 / var_fn434_calc_ig__phitin);
        let assign36720_e33596: f64 = (-var_fn434_calc_ig__vjg);
        let assign36720_e33597: f64 = (assign36720_e33594 * assign36720_e33596);
        (assign36720_e33597, ((-((var_fn434_calc_ig__pg_param1 * var_fn434_calc_ig__phitin_dn4) / (var_fn434_calc_ig__phitin * var_fn434_calc_ig__phitin))) * assign36720_e33596),)
    } else {
        (var_fn434_calc_ig__expphib, var_fn434_calc_ig__expphib_dn4,)
    }
};
        var_fn434_calc_ig__expphib = assign36720_e33599;
        var_fn434_calc_ig__expphib_dn4 = assign36720_e33599_d_n4;

        let (assign36730_e33643, assign36730_e33643_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        let assign36730_e33609: f64 = (-50.0);
        let (assign36730_e33641, assign36730_e33641_d_n4,) = {
            if ((!(var_fn434_calc_ig__expphib > 50.0)) && (!(var_fn434_calc_ig__expphib < assign36730_e33609))) {
                let assign36730_e33614: f64 = (var_fn434_calc_ig__expphib).exp();
                (assign36730_e33614, (assign36730_e33614 * var_fn434_calc_ig__expphib_dn4),)
            } else {
                let assign36730_e33621: f64 = (-50.0);
                let (assign36730_e33640, assign36730_e33640_d_n4,) = {
                    if ((!(var_fn434_calc_ig__expphib > 50.0)) && (var_fn434_calc_ig__expphib < assign36730_e33621)) {
                        let assign36730_e33625: f64 = (-50.0);
                        let assign36730_e33626: f64 = (assign36730_e33625).exp();
                        (assign36730_e33626, 0.0,)
                    } else {
                        let (assign36730_e33639, assign36730_e33639_d_n4,) = {
                            if (var_fn434_calc_ig__expphib > 50.0) {
                                let assign36730_e33631: f64 = (50.0_f64).exp();
                                let assign36730_e33635: f64 = (var_fn434_calc_ig__expphib - 50.0);
                                let assign36730_e33636: f64 = (1.0 + assign36730_e33635);
                                let assign36730_e33637: f64 = (assign36730_e33631 * assign36730_e33636);
                                (assign36730_e33637, (assign36730_e33631 * var_fn434_calc_ig__expphib_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign36730_e33639, assign36730_e33639_d_n4,)
                    }
                };
                (assign36730_e33640, assign36730_e33640_d_n4,)
            }
        };
        (assign36730_e33641, assign36730_e33641_d_n4,)
    } else {
        (var_fn434_calc_ig__t0, var_fn434_calc_ig__t0_dn4,)
    }
};
        var_fn434_calc_ig__t0 = assign36730_e33643;
        var_fn434_calc_ig__t0_dn4 = assign36730_e33643_d_n4;

        let (assign36740_e33656, assign36740_e33656_d_n4, assign36740_e33656_d_n8, assign36740_e33656_d_n17,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        let assign36740_e33649: f64 = (-var_fn434_calc_ig__vgin);
        let assign36740_e33651: f64 = (assign36740_e33649 - var_fn434_calc_ig__vbdgin);
        let assign36740_e33652: f64 = (var_fn434_calc_ig__pbdgin * assign36740_e33651);
        let assign36740_e33654: f64 = (assign36740_e33652 + var_fn434_calc_ig__expphib);
        (assign36740_e33654, var_fn434_calc_ig__expphib_dn4, (var_fn434_calc_ig__pbdgin * (-var_fn434_calc_ig__vgin_dn8)), (var_fn434_calc_ig__pbdgin * (-var_fn434_calc_ig__vgin_dn17)),)
    } else {
        (var_fn434_calc_ig__expbdarg1, var_fn434_calc_ig__expbdarg1_dn4, var_fn434_calc_ig__expbdarg1_dn8, var_fn434_calc_ig__expbdarg1_dn17,)
    }
};
        var_fn434_calc_ig__expbdarg1 = assign36740_e33656;
        var_fn434_calc_ig__expbdarg1_dn4 = assign36740_e33656_d_n4;
        var_fn434_calc_ig__expbdarg1_dn8 = assign36740_e33656_d_n8;
        var_fn434_calc_ig__expbdarg1_dn17 = assign36740_e33656_d_n17;

        let (assign36750_e33667, assign36750_e33667_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        let assign36750_e33661: f64 = (-var_fn434_calc_ig__pbdgin);
        let assign36750_e33663: f64 = (assign36750_e33661 * var_fn434_calc_ig__vbdgin);
        let assign36750_e33665: f64 = (assign36750_e33663 + var_fn434_calc_ig__expphib);
        (assign36750_e33665, var_fn434_calc_ig__expphib_dn4,)
    } else {
        (var_fn434_calc_ig__expbdarg2, var_fn434_calc_ig__expbdarg2_dn4,)
    }
};
        var_fn434_calc_ig__expbdarg2 = assign36750_e33667;
        var_fn434_calc_ig__expbdarg2_dn4 = assign36750_e33667_d_n4;

        let (assign36760_e33711, assign36760_e33711_d_n4, assign36760_e33711_d_n8, assign36760_e33711_d_n17,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        let assign36760_e33677: f64 = (-50.0);
        let (assign36760_e33709, assign36760_e33709_d_n4, assign36760_e33709_d_n8, assign36760_e33709_d_n17,) = {
            if ((!(var_fn434_calc_ig__expbdarg1 > 50.0)) && (!(var_fn434_calc_ig__expbdarg1 < assign36760_e33677))) {
                let assign36760_e33682: f64 = (var_fn434_calc_ig__expbdarg1).exp();
                (assign36760_e33682, (assign36760_e33682 * var_fn434_calc_ig__expbdarg1_dn4), (assign36760_e33682 * var_fn434_calc_ig__expbdarg1_dn8), (assign36760_e33682 * var_fn434_calc_ig__expbdarg1_dn17),)
            } else {
                let assign36760_e33689: f64 = (-50.0);
                let (assign36760_e33708, assign36760_e33708_d_n4, assign36760_e33708_d_n8, assign36760_e33708_d_n17,) = {
                    if ((!(var_fn434_calc_ig__expbdarg1 > 50.0)) && (var_fn434_calc_ig__expbdarg1 < assign36760_e33689)) {
                        let assign36760_e33693: f64 = (-50.0);
                        let assign36760_e33694: f64 = (assign36760_e33693).exp();
                        (assign36760_e33694, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign36760_e33707, assign36760_e33707_d_n4, assign36760_e33707_d_n8, assign36760_e33707_d_n17,) = {
                            if (var_fn434_calc_ig__expbdarg1 > 50.0) {
                                let assign36760_e33699: f64 = (50.0_f64).exp();
                                let assign36760_e33703: f64 = (var_fn434_calc_ig__expbdarg1 - 50.0);
                                let assign36760_e33704: f64 = (1.0 + assign36760_e33703);
                                let assign36760_e33705: f64 = (assign36760_e33699 * assign36760_e33704);
                                (assign36760_e33705, (assign36760_e33699 * var_fn434_calc_ig__expbdarg1_dn4), (assign36760_e33699 * var_fn434_calc_ig__expbdarg1_dn8), (assign36760_e33699 * var_fn434_calc_ig__expbdarg1_dn17),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign36760_e33707, assign36760_e33707_d_n4, assign36760_e33707_d_n8, assign36760_e33707_d_n17,)
                    }
                };
                (assign36760_e33708, assign36760_e33708_d_n4, assign36760_e33708_d_n8, assign36760_e33708_d_n17,)
            }
        };
        (assign36760_e33709, assign36760_e33709_d_n4, assign36760_e33709_d_n8, assign36760_e33709_d_n17,)
    } else {
        (var_fn434_calc_ig__expbd1, var_fn434_calc_ig__expbd1_dn4, var_fn434_calc_ig__expbd1_dn8, var_fn434_calc_ig__expbd1_dn17,)
    }
};
        var_fn434_calc_ig__expbd1 = assign36760_e33711;
        var_fn434_calc_ig__expbd1_dn4 = assign36760_e33711_d_n4;
        var_fn434_calc_ig__expbd1_dn8 = assign36760_e33711_d_n8;
        var_fn434_calc_ig__expbd1_dn17 = assign36760_e33711_d_n17;

        let (assign36770_e33755, assign36770_e33755_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        let assign36770_e33721: f64 = (-50.0);
        let (assign36770_e33753, assign36770_e33753_d_n4,) = {
            if ((!(var_fn434_calc_ig__expbdarg2 > 50.0)) && (!(var_fn434_calc_ig__expbdarg2 < assign36770_e33721))) {
                let assign36770_e33726: f64 = (var_fn434_calc_ig__expbdarg2).exp();
                (assign36770_e33726, (assign36770_e33726 * var_fn434_calc_ig__expbdarg2_dn4),)
            } else {
                let assign36770_e33733: f64 = (-50.0);
                let (assign36770_e33752, assign36770_e33752_d_n4,) = {
                    if ((!(var_fn434_calc_ig__expbdarg2 > 50.0)) && (var_fn434_calc_ig__expbdarg2 < assign36770_e33733)) {
                        let assign36770_e33737: f64 = (-50.0);
                        let assign36770_e33738: f64 = (assign36770_e33737).exp();
                        (assign36770_e33738, 0.0,)
                    } else {
                        let (assign36770_e33751, assign36770_e33751_d_n4,) = {
                            if (var_fn434_calc_ig__expbdarg2 > 50.0) {
                                let assign36770_e33743: f64 = (50.0_f64).exp();
                                let assign36770_e33747: f64 = (var_fn434_calc_ig__expbdarg2 - 50.0);
                                let assign36770_e33748: f64 = (1.0 + assign36770_e33747);
                                let assign36770_e33749: f64 = (assign36770_e33743 * assign36770_e33748);
                                (assign36770_e33749, (assign36770_e33743 * var_fn434_calc_ig__expbdarg2_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign36770_e33751, assign36770_e33751_d_n4,)
                    }
                };
                (assign36770_e33752, assign36770_e33752_d_n4,)
            }
        };
        (assign36770_e33753, assign36770_e33753_d_n4,)
    } else {
        (var_fn434_calc_ig__expbd2, var_fn434_calc_ig__expbd2_dn4,)
    }
};
        var_fn434_calc_ig__expbd2 = assign36770_e33755;
        var_fn434_calc_ig__expbd2_dn4 = assign36770_e33755_d_n4;

        let (assign36780_e33763, assign36780_e33763_d_n4, assign36780_e33763_d_n8, assign36780_e33763_d_n17,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        let assign36780_e33761: f64 = (var_fn434_calc_ig__expbd1 - var_fn434_calc_ig__expbd2);
        (assign36780_e33761, (var_fn434_calc_ig__expbd1_dn4 - var_fn434_calc_ig__expbd2_dn4), var_fn434_calc_ig__expbd1_dn8, var_fn434_calc_ig__expbd1_dn17,)
    } else {
        (var_fn434_calc_ig__iginbd, var_fn434_calc_ig__iginbd_dn4, var_fn434_calc_ig__iginbd_dn8, var_fn434_calc_ig__iginbd_dn17,)
    }
};
        var_fn434_calc_ig__iginbd = assign36780_e33763;
        var_fn434_calc_ig__iginbd_dn4 = assign36780_e33763_d_n4;
        var_fn434_calc_ig__iginbd_dn8 = assign36780_e33763_d_n8;
        var_fn434_calc_ig__iginbd_dn17 = assign36780_e33763_d_n17;

        let (assign36790_e33777, assign36790_e33777_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        let assign36790_e33769: f64 = (var_fn434_calc_ig__type * var_fn434_calc_ig__w);
        let assign36790_e33771: f64 = (assign36790_e33769 * var_fn434_calc_ig__ngf);
        let assign36790_e33773: f64 = (assign36790_e33771 * var_fn434_calc_ig__ijin);
        let assign36790_e33775: f64 = (assign36790_e33773 * var_fn434_calc_ig__tfacdiodein);
        (assign36790_e33775, (assign36790_e33773 * var_fn434_calc_ig__tfacdiodein_dn4),)
    } else {
        (var_fn434_calc_ig__isdiodeout, var_fn434_calc_ig__isdiodeout_dn4,)
    }
};
        var_fn434_calc_ig__isdiodeout = assign36790_e33777;
        var_fn434_calc_ig__isdiodeout_dn4 = assign36790_e33777_d_n4;

        let (assign36800_e33789, assign36800_e33789_d_n4, assign36800_e33789_d_n8, assign36800_e33789_d_n17,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        let assign36800_e33783: f64 = (var_fn434_calc_ig__pg_paramin / var_fn434_calc_ig__phitin);
        let assign36800_e33785: f64 = (assign36800_e33783 * var_fn434_calc_ig__vgin);
        let assign36800_e33787: f64 = (assign36800_e33785 + var_fn434_calc_ig__expphib);
        (assign36800_e33787, (((-((var_fn434_calc_ig__pg_paramin * var_fn434_calc_ig__phitin_dn4) / (var_fn434_calc_ig__phitin * var_fn434_calc_ig__phitin))) * var_fn434_calc_ig__vgin) + var_fn434_calc_ig__expphib_dn4), (assign36800_e33783 * var_fn434_calc_ig__vgin_dn8), (assign36800_e33783 * var_fn434_calc_ig__vgin_dn17),)
    } else {
        (var_fn434_calc_ig__expiforarg, var_fn434_calc_ig__expiforarg_dn4, var_fn434_calc_ig__expiforarg_dn8, var_fn434_calc_ig__expiforarg_dn17,)
    }
};
        var_fn434_calc_ig__expiforarg = assign36800_e33789;
        var_fn434_calc_ig__expiforarg_dn4 = assign36800_e33789_d_n4;
        var_fn434_calc_ig__expiforarg_dn8 = assign36800_e33789_d_n8;
        var_fn434_calc_ig__expiforarg_dn17 = assign36800_e33789_d_n17;

        let (assign36810_e33833, assign36810_e33833_d_n4, assign36810_e33833_d_n8, assign36810_e33833_d_n17,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        let assign36810_e33799: f64 = (-50.0);
        let (assign36810_e33831, assign36810_e33831_d_n4, assign36810_e33831_d_n8, assign36810_e33831_d_n17,) = {
            if ((!(var_fn434_calc_ig__expiforarg > 50.0)) && (!(var_fn434_calc_ig__expiforarg < assign36810_e33799))) {
                let assign36810_e33804: f64 = (var_fn434_calc_ig__expiforarg).exp();
                (assign36810_e33804, (assign36810_e33804 * var_fn434_calc_ig__expiforarg_dn4), (assign36810_e33804 * var_fn434_calc_ig__expiforarg_dn8), (assign36810_e33804 * var_fn434_calc_ig__expiforarg_dn17),)
            } else {
                let assign36810_e33811: f64 = (-50.0);
                let (assign36810_e33830, assign36810_e33830_d_n4, assign36810_e33830_d_n8, assign36810_e33830_d_n17,) = {
                    if ((!(var_fn434_calc_ig__expiforarg > 50.0)) && (var_fn434_calc_ig__expiforarg < assign36810_e33811)) {
                        let assign36810_e33815: f64 = (-50.0);
                        let assign36810_e33816: f64 = (assign36810_e33815).exp();
                        (assign36810_e33816, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign36810_e33829, assign36810_e33829_d_n4, assign36810_e33829_d_n8, assign36810_e33829_d_n17,) = {
                            if (var_fn434_calc_ig__expiforarg > 50.0) {
                                let assign36810_e33821: f64 = (50.0_f64).exp();
                                let assign36810_e33825: f64 = (var_fn434_calc_ig__expiforarg - 50.0);
                                let assign36810_e33826: f64 = (1.0 + assign36810_e33825);
                                let assign36810_e33827: f64 = (assign36810_e33821 * assign36810_e33826);
                                (assign36810_e33827, (assign36810_e33821 * var_fn434_calc_ig__expiforarg_dn4), (assign36810_e33821 * var_fn434_calc_ig__expiforarg_dn8), (assign36810_e33821 * var_fn434_calc_ig__expiforarg_dn17),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign36810_e33829, assign36810_e33829_d_n4, assign36810_e33829_d_n8, assign36810_e33829_d_n17,)
                    }
                };
                (assign36810_e33830, assign36810_e33830_d_n4, assign36810_e33830_d_n8, assign36810_e33830_d_n17,)
            }
        };
        (assign36810_e33831, assign36810_e33831_d_n4, assign36810_e33831_d_n8, assign36810_e33831_d_n17,)
    } else {
        (var_fn434_calc_ig__expifor, var_fn434_calc_ig__expifor_dn4, var_fn434_calc_ig__expifor_dn8, var_fn434_calc_ig__expifor_dn17,)
    }
};
        var_fn434_calc_ig__expifor = assign36810_e33833;
        var_fn434_calc_ig__expifor_dn4 = assign36810_e33833_d_n4;
        var_fn434_calc_ig__expifor_dn8 = assign36810_e33833_d_n8;
        var_fn434_calc_ig__expifor_dn17 = assign36810_e33833_d_n17;

        let assign36820_e33836: f64 = if var_fn434_calc_ig__fracin == 1.0 { 1.0 } else { 0.0 };
        var_guard435 = assign36820_e33836;

        let (assign36830_e33852, assign36830_e33852_d_n4, assign36830_e33852_d_n8, assign36830_e33852_d_n17,) = {
    if (((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard435 != 0.0)) {
        let assign36830_e33846: f64 = (var_fn434_calc_ig__kbdgatein * var_fn434_calc_ig__iginbd);
        let assign36830_e33847: f64 = (var_fn434_calc_ig__expifor - assign36830_e33846);
        let assign36830_e33849: f64 = (assign36830_e33847 - var_fn434_calc_ig__t0);
        let assign36830_e33850: f64 = (var_fn434_calc_ig__isdiodeout * assign36830_e33849);
        (assign36830_e33850, ((var_fn434_calc_ig__isdiodeout_dn4 * assign36830_e33849) + (var_fn434_calc_ig__isdiodeout * ((var_fn434_calc_ig__expifor_dn4 - (var_fn434_calc_ig__kbdgatein * var_fn434_calc_ig__iginbd_dn4)) - var_fn434_calc_ig__t0_dn4))), (var_fn434_calc_ig__isdiodeout * (var_fn434_calc_ig__expifor_dn8 - (var_fn434_calc_ig__kbdgatein * var_fn434_calc_ig__iginbd_dn8))), (var_fn434_calc_ig__isdiodeout * (var_fn434_calc_ig__expifor_dn17 - (var_fn434_calc_ig__kbdgatein * var_fn434_calc_ig__iginbd_dn17))),)
    } else {
        (var_fn434_calc_ig__igindiode, var_fn434_calc_ig__igindiode_dn4, var_fn434_calc_ig__igindiode_dn8, var_fn434_calc_ig__igindiode_dn17,)
    }
};
        var_fn434_calc_ig__igindiode = assign36830_e33852;
        var_fn434_calc_ig__igindiode_dn4 = assign36830_e33852_d_n4;
        var_fn434_calc_ig__igindiode_dn8 = assign36830_e33852_d_n8;
        var_fn434_calc_ig__igindiode_dn17 = assign36830_e33852_d_n17;

        *var_fn434_calc_ig__expbd1_slot = var_fn434_calc_ig__expbd1;
        *var_fn434_calc_ig__expbd1_dn17_slot = var_fn434_calc_ig__expbd1_dn17;
        *var_fn434_calc_ig__expbd1_dn4_slot = var_fn434_calc_ig__expbd1_dn4;
        *var_fn434_calc_ig__expbd1_dn8_slot = var_fn434_calc_ig__expbd1_dn8;
        *var_fn434_calc_ig__expbd1_vgsat_slot = var_fn434_calc_ig__expbd1_vgsat;
        *var_fn434_calc_ig__expbd1_vgsat_dn4_slot = var_fn434_calc_ig__expbd1_vgsat_dn4;
        *var_fn434_calc_ig__expbd2_slot = var_fn434_calc_ig__expbd2;
        *var_fn434_calc_ig__expbd2_dn4_slot = var_fn434_calc_ig__expbd2_dn4;
        *var_fn434_calc_ig__expbdarg1_slot = var_fn434_calc_ig__expbdarg1;
        *var_fn434_calc_ig__expbdarg1_dn17_slot = var_fn434_calc_ig__expbdarg1_dn17;
        *var_fn434_calc_ig__expbdarg1_dn4_slot = var_fn434_calc_ig__expbdarg1_dn4;
        *var_fn434_calc_ig__expbdarg1_dn8_slot = var_fn434_calc_ig__expbdarg1_dn8;
        *var_fn434_calc_ig__expbdarg1_vgsat_slot = var_fn434_calc_ig__expbdarg1_vgsat;
        *var_fn434_calc_ig__expbdarg1_vgsat_dn4_slot = var_fn434_calc_ig__expbdarg1_vgsat_dn4;
        *var_fn434_calc_ig__expbdarg2_slot = var_fn434_calc_ig__expbdarg2;
        *var_fn434_calc_ig__expbdarg2_dn4_slot = var_fn434_calc_ig__expbdarg2_dn4;
        *var_fn434_calc_ig__expffvarg_slot = var_fn434_calc_ig__expffvarg;
        *var_fn434_calc_ig__expffvarg_dn17_slot = var_fn434_calc_ig__expffvarg_dn17;
        *var_fn434_calc_ig__expffvarg_dn4_slot = var_fn434_calc_ig__expffvarg_dn4;
        *var_fn434_calc_ig__expffvarg_dn8_slot = var_fn434_calc_ig__expffvarg_dn8;
        *var_fn434_calc_ig__expifor_slot = var_fn434_calc_ig__expifor;
        *var_fn434_calc_ig__expifor_dn17_slot = var_fn434_calc_ig__expifor_dn17;
        *var_fn434_calc_ig__expifor_dn4_slot = var_fn434_calc_ig__expifor_dn4;
        *var_fn434_calc_ig__expifor_dn8_slot = var_fn434_calc_ig__expifor_dn8;
        *var_fn434_calc_ig__expifor_hinj_slot = var_fn434_calc_ig__expifor_hinj;
        *var_fn434_calc_ig__expifor_hinj_dn17_slot = var_fn434_calc_ig__expifor_hinj_dn17;
        *var_fn434_calc_ig__expifor_hinj_dn4_slot = var_fn434_calc_ig__expifor_hinj_dn4;
        *var_fn434_calc_ig__expifor_hinj_dn8_slot = var_fn434_calc_ig__expifor_hinj_dn8;
        *var_fn434_calc_ig__expifor_hinj_vgsat_slot = var_fn434_calc_ig__expifor_hinj_vgsat;
        *var_fn434_calc_ig__expifor_hinj_vgsat_dn4_slot = var_fn434_calc_ig__expifor_hinj_vgsat_dn4;
        *var_fn434_calc_ig__expifor_nohinj_vgsat_slot = var_fn434_calc_ig__expifor_nohinj_vgsat;
        *var_fn434_calc_ig__expifor_nohinj_vgsat_dn4_slot = var_fn434_calc_ig__expifor_nohinj_vgsat_dn4;
        *var_fn434_calc_ig__expiforarg_slot = var_fn434_calc_ig__expiforarg;
        *var_fn434_calc_ig__expiforarg_dn17_slot = var_fn434_calc_ig__expiforarg_dn17;
        *var_fn434_calc_ig__expiforarg_dn4_slot = var_fn434_calc_ig__expiforarg_dn4;
        *var_fn434_calc_ig__expiforarg_dn8_slot = var_fn434_calc_ig__expiforarg_dn8;
        *var_fn434_calc_ig__expiforarg_hinj_slot = var_fn434_calc_ig__expiforarg_hinj;
        *var_fn434_calc_ig__expiforarg_hinj_dn17_slot = var_fn434_calc_ig__expiforarg_hinj_dn17;
        *var_fn434_calc_ig__expiforarg_hinj_dn4_slot = var_fn434_calc_ig__expiforarg_hinj_dn4;
        *var_fn434_calc_ig__expiforarg_hinj_dn8_slot = var_fn434_calc_ig__expiforarg_hinj_dn8;
        *var_fn434_calc_ig__expiforarg_hinj_vgsat_slot = var_fn434_calc_ig__expiforarg_hinj_vgsat;
        *var_fn434_calc_ig__expiforarg_hinj_vgsat_dn4_slot = var_fn434_calc_ig__expiforarg_hinj_vgsat_dn4;
        *var_fn434_calc_ig__expiforarg_nohinj_vgsat_slot = var_fn434_calc_ig__expiforarg_nohinj_vgsat;
        *var_fn434_calc_ig__expiforarg_nohinj_vgsat_dn4_slot = var_fn434_calc_ig__expiforarg_nohinj_vgsat_dn4;
        *var_fn434_calc_ig__expirev_slot = var_fn434_calc_ig__expirev;
        *var_fn434_calc_ig__expirev_dn17_slot = var_fn434_calc_ig__expirev_dn17;
        *var_fn434_calc_ig__expirev_dn4_slot = var_fn434_calc_ig__expirev_dn4;
        *var_fn434_calc_ig__expirev_dn8_slot = var_fn434_calc_ig__expirev_dn8;
        *var_fn434_calc_ig__expirevarg_slot = var_fn434_calc_ig__expirevarg;
        *var_fn434_calc_ig__expirevarg_dn17_slot = var_fn434_calc_ig__expirevarg_dn17;
        *var_fn434_calc_ig__expirevarg_dn4_slot = var_fn434_calc_ig__expirevarg_dn4;
        *var_fn434_calc_ig__expirevarg_dn8_slot = var_fn434_calc_ig__expirevarg_dn8;
        *var_fn434_calc_ig__expphib_slot = var_fn434_calc_ig__expphib;
        *var_fn434_calc_ig__expphib_dn4_slot = var_fn434_calc_ig__expphib_dn4;
        *var_fn434_calc_ig__iginbd_slot = var_fn434_calc_ig__iginbd;
        *var_fn434_calc_ig__iginbd_dn17_slot = var_fn434_calc_ig__iginbd_dn17;
        *var_fn434_calc_ig__iginbd_dn4_slot = var_fn434_calc_ig__iginbd_dn4;
        *var_fn434_calc_ig__iginbd_dn8_slot = var_fn434_calc_ig__iginbd_dn8;
        *var_fn434_calc_ig__iginbd_vgsat_slot = var_fn434_calc_ig__iginbd_vgsat;
        *var_fn434_calc_ig__iginbd_vgsat_dn4_slot = var_fn434_calc_ig__iginbd_vgsat_dn4;
        *var_fn434_calc_ig__igindiode_slot = var_fn434_calc_ig__igindiode;
        *var_fn434_calc_ig__igindiode_dn17_slot = var_fn434_calc_ig__igindiode_dn17;
        *var_fn434_calc_ig__igindiode_dn4_slot = var_fn434_calc_ig__igindiode_dn4;
        *var_fn434_calc_ig__igindiode_dn8_slot = var_fn434_calc_ig__igindiode_dn8;
        *var_fn434_calc_ig__igindiode_hinj_slot = var_fn434_calc_ig__igindiode_hinj;
        *var_fn434_calc_ig__igindiode_hinj_dn17_slot = var_fn434_calc_ig__igindiode_hinj_dn17;
        *var_fn434_calc_ig__igindiode_hinj_dn4_slot = var_fn434_calc_ig__igindiode_hinj_dn4;
        *var_fn434_calc_ig__igindiode_hinj_dn8_slot = var_fn434_calc_ig__igindiode_hinj_dn8;
        *var_fn434_calc_ig__igindiode_hinj_pre_slot = var_fn434_calc_ig__igindiode_hinj_pre;
        *var_fn434_calc_ig__igindiode_hinj_pre_dn4_slot = var_fn434_calc_ig__igindiode_hinj_pre_dn4;
        *var_fn434_calc_ig__igindiode_hinj_vgsat_slot = var_fn434_calc_ig__igindiode_hinj_vgsat;
        *var_fn434_calc_ig__igindiode_hinj_vgsat_dn4_slot = var_fn434_calc_ig__igindiode_hinj_vgsat_dn4;
        *var_fn434_calc_ig__igindiode_nohinj_slot = var_fn434_calc_ig__igindiode_nohinj;
        *var_fn434_calc_ig__igindiode_nohinj_dn17_slot = var_fn434_calc_ig__igindiode_nohinj_dn17;
        *var_fn434_calc_ig__igindiode_nohinj_dn4_slot = var_fn434_calc_ig__igindiode_nohinj_dn4;
        *var_fn434_calc_ig__igindiode_nohinj_dn8_slot = var_fn434_calc_ig__igindiode_nohinj_dn8;
        *var_fn434_calc_ig__igindiode_nohinj_vgsat_slot = var_fn434_calc_ig__igindiode_nohinj_vgsat;
        *var_fn434_calc_ig__igindiode_nohinj_vgsat_dn4_slot = var_fn434_calc_ig__igindiode_nohinj_vgsat_dn4;
        *var_fn434_calc_ig__isdiodeout_slot = var_fn434_calc_ig__isdiodeout;
        *var_fn434_calc_ig__isdiodeout_dn4_slot = var_fn434_calc_ig__isdiodeout_dn4;
        *var_fn434_calc_ig__pg_paramin_hinj_slot = var_fn434_calc_ig__pg_paramin_hinj;
        *var_fn434_calc_ig__t0_slot = var_fn434_calc_ig__t0;
        *var_fn434_calc_ig__t0_dn4_slot = var_fn434_calc_ig__t0_dn4;
        *var_guard435_slot = var_guard435;
    }

    pub(super) fn stamp_transient_block_91(
        p: &Parameters,
        var_fn434_calc_ig__alphagin: f64,
        var_fn434_calc_ig__betarecin: f64,
        var_fn434_calc_ig__expbd2: f64,
        var_fn434_calc_ig__expbd2_dn4: f64,
        var_fn434_calc_ig__expifor: f64,
        var_fn434_calc_ig__expifor_dn17: f64,
        var_fn434_calc_ig__expifor_dn4: f64,
        var_fn434_calc_ig__expifor_dn8: f64,
        var_fn434_calc_ig__expphib: f64,
        var_fn434_calc_ig__expphib_dn4: f64,
        var_fn434_calc_ig__fracin: f64,
        var_fn434_calc_ig__iginbd: f64,
        var_fn434_calc_ig__iginbd_dn17: f64,
        var_fn434_calc_ig__iginbd_dn4: f64,
        var_fn434_calc_ig__iginbd_dn8: f64,
        var_fn434_calc_ig__irecin: f64,
        var_fn434_calc_ig__isdiodeout: f64,
        var_fn434_calc_ig__isdiodeout_dn4: f64,
        var_fn434_calc_ig__kbdgatein: f64,
        var_fn434_calc_ig__ngf: f64,
        var_fn434_calc_ig__pbdgin: f64,
        var_fn434_calc_ig__pg_paramin: f64,
        var_fn434_calc_ig__pgsrecin: f64,
        var_fn434_calc_ig__phitin: f64,
        var_fn434_calc_ig__phitin_dn4: f64,
        var_fn434_calc_ig__t0: f64,
        var_fn434_calc_ig__t0_dn4: f64,
        var_fn434_calc_ig__tfacdiodein: f64,
        var_fn434_calc_ig__tfacdiodein_dn4: f64,
        var_fn434_calc_ig__type: f64,
        var_fn434_calc_ig__vbdgin: f64,
        var_fn434_calc_ig__vgin: f64,
        var_fn434_calc_ig__vgin_dn17: f64,
        var_fn434_calc_ig__vgin_dn8: f64,
        var_fn434_calc_ig__vgsatin: f64,
        var_fn434_calc_ig__vgsatqin: f64,
        var_fn434_calc_ig__w: f64,
        var_guard417: f64,
        var_guard428: f64,
        var_guard435: f64,
        var_fn434_calc_ig__alpha2_phit_slot: &mut f64,
        var_fn434_calc_ig__alpha2_phit_dn4_slot: &mut f64,
        var_fn434_calc_ig__expbd1_vgsat_slot: &mut f64,
        var_fn434_calc_ig__expbd1_vgsat_dn4_slot: &mut f64,
        var_fn434_calc_ig__expbdarg1_vgsat_slot: &mut f64,
        var_fn434_calc_ig__expbdarg1_vgsat_dn4_slot: &mut f64,
        var_fn434_calc_ig__expffvarg_slot: &mut f64,
        var_fn434_calc_ig__expffvarg_dn17_slot: &mut f64,
        var_fn434_calc_ig__expffvarg_dn4_slot: &mut f64,
        var_fn434_calc_ig__expffvarg_dn8_slot: &mut f64,
        var_fn434_calc_ig__expifor_hinj_slot: &mut f64,
        var_fn434_calc_ig__expifor_hinj_dn17_slot: &mut f64,
        var_fn434_calc_ig__expifor_hinj_dn4_slot: &mut f64,
        var_fn434_calc_ig__expifor_hinj_dn8_slot: &mut f64,
        var_fn434_calc_ig__expifor_hinj_vgsat_slot: &mut f64,
        var_fn434_calc_ig__expifor_hinj_vgsat_dn4_slot: &mut f64,
        var_fn434_calc_ig__expifor_nohinj_vgsat_slot: &mut f64,
        var_fn434_calc_ig__expifor_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn434_calc_ig__expiforarg_hinj_slot: &mut f64,
        var_fn434_calc_ig__expiforarg_hinj_dn17_slot: &mut f64,
        var_fn434_calc_ig__expiforarg_hinj_dn4_slot: &mut f64,
        var_fn434_calc_ig__expiforarg_hinj_dn8_slot: &mut f64,
        var_fn434_calc_ig__expiforarg_hinj_vgsat_slot: &mut f64,
        var_fn434_calc_ig__expiforarg_hinj_vgsat_dn4_slot: &mut f64,
        var_fn434_calc_ig__expiforarg_nohinj_vgsat_slot: &mut f64,
        var_fn434_calc_ig__expiforarg_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn434_calc_ig__expirevarg_slot: &mut f64,
        var_fn434_calc_ig__expirevarg_dn17_slot: &mut f64,
        var_fn434_calc_ig__expirevarg_dn4_slot: &mut f64,
        var_fn434_calc_ig__expirevarg_dn8_slot: &mut f64,
        var_fn434_calc_ig__ffvgin_slot: &mut f64,
        var_fn434_calc_ig__ffvgin_dn17_slot: &mut f64,
        var_fn434_calc_ig__ffvgin_dn4_slot: &mut f64,
        var_fn434_calc_ig__ffvgin_dn8_slot: &mut f64,
        var_fn434_calc_ig__frecgin_slot: &mut f64,
        var_fn434_calc_ig__frecgin_dn17_slot: &mut f64,
        var_fn434_calc_ig__frecgin_dn8_slot: &mut f64,
        var_fn434_calc_ig__iginbd_vgsat_slot: &mut f64,
        var_fn434_calc_ig__iginbd_vgsat_dn4_slot: &mut f64,
        var_fn434_calc_ig__igindiode_slot: &mut f64,
        var_fn434_calc_ig__igindiode_dn17_slot: &mut f64,
        var_fn434_calc_ig__igindiode_dn4_slot: &mut f64,
        var_fn434_calc_ig__igindiode_dn8_slot: &mut f64,
        var_fn434_calc_ig__igindiode_hinj_slot: &mut f64,
        var_fn434_calc_ig__igindiode_hinj_dn17_slot: &mut f64,
        var_fn434_calc_ig__igindiode_hinj_dn4_slot: &mut f64,
        var_fn434_calc_ig__igindiode_hinj_dn8_slot: &mut f64,
        var_fn434_calc_ig__igindiode_hinj_pre_slot: &mut f64,
        var_fn434_calc_ig__igindiode_hinj_pre_dn4_slot: &mut f64,
        var_fn434_calc_ig__igindiode_hinj_vgsat_slot: &mut f64,
        var_fn434_calc_ig__igindiode_hinj_vgsat_dn4_slot: &mut f64,
        var_fn434_calc_ig__igindiode_nohinj_slot: &mut f64,
        var_fn434_calc_ig__igindiode_nohinj_dn17_slot: &mut f64,
        var_fn434_calc_ig__igindiode_nohinj_dn4_slot: &mut f64,
        var_fn434_calc_ig__igindiode_nohinj_dn8_slot: &mut f64,
        var_fn434_calc_ig__igindiode_nohinj_vgsat_slot: &mut f64,
        var_fn434_calc_ig__igindiode_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn434_calc_ig__isrecout_slot: &mut f64,
        var_fn434_calc_ig__isrecout_dn4_slot: &mut f64,
        var_fn434_calc_ig__pg_paramin_hinj_slot: &mut f64,
        var_guard436_slot: &mut f64,
        var_guard437_slot: &mut f64,
        var_guard438_slot: &mut f64,
    ) {
        let mut var_fn434_calc_ig__alpha2_phit: f64 = *var_fn434_calc_ig__alpha2_phit_slot;
        let mut var_fn434_calc_ig__alpha2_phit_dn4: f64 = *var_fn434_calc_ig__alpha2_phit_dn4_slot;
        let mut var_fn434_calc_ig__expbd1_vgsat: f64 = *var_fn434_calc_ig__expbd1_vgsat_slot;
        let mut var_fn434_calc_ig__expbd1_vgsat_dn4: f64 = *var_fn434_calc_ig__expbd1_vgsat_dn4_slot;
        let mut var_fn434_calc_ig__expbdarg1_vgsat: f64 = *var_fn434_calc_ig__expbdarg1_vgsat_slot;
        let mut var_fn434_calc_ig__expbdarg1_vgsat_dn4: f64 = *var_fn434_calc_ig__expbdarg1_vgsat_dn4_slot;
        let mut var_fn434_calc_ig__expffvarg: f64 = *var_fn434_calc_ig__expffvarg_slot;
        let mut var_fn434_calc_ig__expffvarg_dn17: f64 = *var_fn434_calc_ig__expffvarg_dn17_slot;
        let mut var_fn434_calc_ig__expffvarg_dn4: f64 = *var_fn434_calc_ig__expffvarg_dn4_slot;
        let mut var_fn434_calc_ig__expffvarg_dn8: f64 = *var_fn434_calc_ig__expffvarg_dn8_slot;
        let mut var_fn434_calc_ig__expifor_hinj: f64 = *var_fn434_calc_ig__expifor_hinj_slot;
        let mut var_fn434_calc_ig__expifor_hinj_dn17: f64 = *var_fn434_calc_ig__expifor_hinj_dn17_slot;
        let mut var_fn434_calc_ig__expifor_hinj_dn4: f64 = *var_fn434_calc_ig__expifor_hinj_dn4_slot;
        let mut var_fn434_calc_ig__expifor_hinj_dn8: f64 = *var_fn434_calc_ig__expifor_hinj_dn8_slot;
        let mut var_fn434_calc_ig__expifor_hinj_vgsat: f64 = *var_fn434_calc_ig__expifor_hinj_vgsat_slot;
        let mut var_fn434_calc_ig__expifor_hinj_vgsat_dn4: f64 = *var_fn434_calc_ig__expifor_hinj_vgsat_dn4_slot;
        let mut var_fn434_calc_ig__expifor_nohinj_vgsat: f64 = *var_fn434_calc_ig__expifor_nohinj_vgsat_slot;
        let mut var_fn434_calc_ig__expifor_nohinj_vgsat_dn4: f64 = *var_fn434_calc_ig__expifor_nohinj_vgsat_dn4_slot;
        let mut var_fn434_calc_ig__expiforarg_hinj: f64 = *var_fn434_calc_ig__expiforarg_hinj_slot;
        let mut var_fn434_calc_ig__expiforarg_hinj_dn17: f64 = *var_fn434_calc_ig__expiforarg_hinj_dn17_slot;
        let mut var_fn434_calc_ig__expiforarg_hinj_dn4: f64 = *var_fn434_calc_ig__expiforarg_hinj_dn4_slot;
        let mut var_fn434_calc_ig__expiforarg_hinj_dn8: f64 = *var_fn434_calc_ig__expiforarg_hinj_dn8_slot;
        let mut var_fn434_calc_ig__expiforarg_hinj_vgsat: f64 = *var_fn434_calc_ig__expiforarg_hinj_vgsat_slot;
        let mut var_fn434_calc_ig__expiforarg_hinj_vgsat_dn4: f64 = *var_fn434_calc_ig__expiforarg_hinj_vgsat_dn4_slot;
        let mut var_fn434_calc_ig__expiforarg_nohinj_vgsat: f64 = *var_fn434_calc_ig__expiforarg_nohinj_vgsat_slot;
        let mut var_fn434_calc_ig__expiforarg_nohinj_vgsat_dn4: f64 = *var_fn434_calc_ig__expiforarg_nohinj_vgsat_dn4_slot;
        let mut var_fn434_calc_ig__expirevarg: f64 = *var_fn434_calc_ig__expirevarg_slot;
        let mut var_fn434_calc_ig__expirevarg_dn17: f64 = *var_fn434_calc_ig__expirevarg_dn17_slot;
        let mut var_fn434_calc_ig__expirevarg_dn4: f64 = *var_fn434_calc_ig__expirevarg_dn4_slot;
        let mut var_fn434_calc_ig__expirevarg_dn8: f64 = *var_fn434_calc_ig__expirevarg_dn8_slot;
        let mut var_fn434_calc_ig__ffvgin: f64 = *var_fn434_calc_ig__ffvgin_slot;
        let mut var_fn434_calc_ig__ffvgin_dn17: f64 = *var_fn434_calc_ig__ffvgin_dn17_slot;
        let mut var_fn434_calc_ig__ffvgin_dn4: f64 = *var_fn434_calc_ig__ffvgin_dn4_slot;
        let mut var_fn434_calc_ig__ffvgin_dn8: f64 = *var_fn434_calc_ig__ffvgin_dn8_slot;
        let mut var_fn434_calc_ig__frecgin: f64 = *var_fn434_calc_ig__frecgin_slot;
        let mut var_fn434_calc_ig__frecgin_dn17: f64 = *var_fn434_calc_ig__frecgin_dn17_slot;
        let mut var_fn434_calc_ig__frecgin_dn8: f64 = *var_fn434_calc_ig__frecgin_dn8_slot;
        let mut var_fn434_calc_ig__iginbd_vgsat: f64 = *var_fn434_calc_ig__iginbd_vgsat_slot;
        let mut var_fn434_calc_ig__iginbd_vgsat_dn4: f64 = *var_fn434_calc_ig__iginbd_vgsat_dn4_slot;
        let mut var_fn434_calc_ig__igindiode: f64 = *var_fn434_calc_ig__igindiode_slot;
        let mut var_fn434_calc_ig__igindiode_dn17: f64 = *var_fn434_calc_ig__igindiode_dn17_slot;
        let mut var_fn434_calc_ig__igindiode_dn4: f64 = *var_fn434_calc_ig__igindiode_dn4_slot;
        let mut var_fn434_calc_ig__igindiode_dn8: f64 = *var_fn434_calc_ig__igindiode_dn8_slot;
        let mut var_fn434_calc_ig__igindiode_hinj: f64 = *var_fn434_calc_ig__igindiode_hinj_slot;
        let mut var_fn434_calc_ig__igindiode_hinj_dn17: f64 = *var_fn434_calc_ig__igindiode_hinj_dn17_slot;
        let mut var_fn434_calc_ig__igindiode_hinj_dn4: f64 = *var_fn434_calc_ig__igindiode_hinj_dn4_slot;
        let mut var_fn434_calc_ig__igindiode_hinj_dn8: f64 = *var_fn434_calc_ig__igindiode_hinj_dn8_slot;
        let mut var_fn434_calc_ig__igindiode_hinj_pre: f64 = *var_fn434_calc_ig__igindiode_hinj_pre_slot;
        let mut var_fn434_calc_ig__igindiode_hinj_pre_dn4: f64 = *var_fn434_calc_ig__igindiode_hinj_pre_dn4_slot;
        let mut var_fn434_calc_ig__igindiode_hinj_vgsat: f64 = *var_fn434_calc_ig__igindiode_hinj_vgsat_slot;
        let mut var_fn434_calc_ig__igindiode_hinj_vgsat_dn4: f64 = *var_fn434_calc_ig__igindiode_hinj_vgsat_dn4_slot;
        let mut var_fn434_calc_ig__igindiode_nohinj: f64 = *var_fn434_calc_ig__igindiode_nohinj_slot;
        let mut var_fn434_calc_ig__igindiode_nohinj_dn17: f64 = *var_fn434_calc_ig__igindiode_nohinj_dn17_slot;
        let mut var_fn434_calc_ig__igindiode_nohinj_dn4: f64 = *var_fn434_calc_ig__igindiode_nohinj_dn4_slot;
        let mut var_fn434_calc_ig__igindiode_nohinj_dn8: f64 = *var_fn434_calc_ig__igindiode_nohinj_dn8_slot;
        let mut var_fn434_calc_ig__igindiode_nohinj_vgsat: f64 = *var_fn434_calc_ig__igindiode_nohinj_vgsat_slot;
        let mut var_fn434_calc_ig__igindiode_nohinj_vgsat_dn4: f64 = *var_fn434_calc_ig__igindiode_nohinj_vgsat_dn4_slot;
        let mut var_fn434_calc_ig__isrecout: f64 = *var_fn434_calc_ig__isrecout_slot;
        let mut var_fn434_calc_ig__isrecout_dn4: f64 = *var_fn434_calc_ig__isrecout_dn4_slot;
        let mut var_fn434_calc_ig__pg_paramin_hinj: f64 = *var_fn434_calc_ig__pg_paramin_hinj_slot;
        let mut var_guard436: f64 = *var_guard436_slot;
        let mut var_guard437: f64 = *var_guard437_slot;
        let mut var_guard438: f64 = *var_guard438_slot;

        let (assign36840_e33868, assign36840_e33868_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard435 == 0.0)) {
        let assign36840_e33861: f64 = (-var_fn434_calc_ig__vgsatin);
        let assign36840_e33863: f64 = (assign36840_e33861 - var_fn434_calc_ig__vbdgin);
        let assign36840_e33864: f64 = (var_fn434_calc_ig__pbdgin * assign36840_e33863);
        let assign36840_e33866: f64 = (assign36840_e33864 + var_fn434_calc_ig__expphib);
        (assign36840_e33866, var_fn434_calc_ig__expphib_dn4,)
    } else {
        (var_fn434_calc_ig__expbdarg1_vgsat, var_fn434_calc_ig__expbdarg1_vgsat_dn4,)
    }
};
        var_fn434_calc_ig__expbdarg1_vgsat = assign36840_e33868;
        var_fn434_calc_ig__expbdarg1_vgsat_dn4 = assign36840_e33868_d_n4;

        let (assign36850_e33915, assign36850_e33915_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard435 == 0.0)) {
        let assign36850_e33881: f64 = (-50.0);
        let (assign36850_e33913, assign36850_e33913_d_n4,) = {
            if ((!(var_fn434_calc_ig__expbdarg1_vgsat > 50.0)) && (!(var_fn434_calc_ig__expbdarg1_vgsat < assign36850_e33881))) {
                let assign36850_e33886: f64 = (var_fn434_calc_ig__expbdarg1_vgsat).exp();
                (assign36850_e33886, (assign36850_e33886 * var_fn434_calc_ig__expbdarg1_vgsat_dn4),)
            } else {
                let assign36850_e33893: f64 = (-50.0);
                let (assign36850_e33912, assign36850_e33912_d_n4,) = {
                    if ((!(var_fn434_calc_ig__expbdarg1_vgsat > 50.0)) && (var_fn434_calc_ig__expbdarg1_vgsat < assign36850_e33893)) {
                        let assign36850_e33897: f64 = (-50.0);
                        let assign36850_e33898: f64 = (assign36850_e33897).exp();
                        (assign36850_e33898, 0.0,)
                    } else {
                        let (assign36850_e33911, assign36850_e33911_d_n4,) = {
                            if (var_fn434_calc_ig__expbdarg1_vgsat > 50.0) {
                                let assign36850_e33903: f64 = (50.0_f64).exp();
                                let assign36850_e33907: f64 = (var_fn434_calc_ig__expbdarg1_vgsat - 50.0);
                                let assign36850_e33908: f64 = (1.0 + assign36850_e33907);
                                let assign36850_e33909: f64 = (assign36850_e33903 * assign36850_e33908);
                                (assign36850_e33909, (assign36850_e33903 * var_fn434_calc_ig__expbdarg1_vgsat_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign36850_e33911, assign36850_e33911_d_n4,)
                    }
                };
                (assign36850_e33912, assign36850_e33912_d_n4,)
            }
        };
        (assign36850_e33913, assign36850_e33913_d_n4,)
    } else {
        (var_fn434_calc_ig__expbd1_vgsat, var_fn434_calc_ig__expbd1_vgsat_dn4,)
    }
};
        var_fn434_calc_ig__expbd1_vgsat = assign36850_e33915;
        var_fn434_calc_ig__expbd1_vgsat_dn4 = assign36850_e33915_d_n4;

        let (assign36860_e33926, assign36860_e33926_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard435 == 0.0)) {
        let assign36860_e33924: f64 = (var_fn434_calc_ig__expbd1_vgsat - var_fn434_calc_ig__expbd2);
        (assign36860_e33924, (var_fn434_calc_ig__expbd1_vgsat_dn4 - var_fn434_calc_ig__expbd2_dn4),)
    } else {
        (var_fn434_calc_ig__iginbd_vgsat, var_fn434_calc_ig__iginbd_vgsat_dn4,)
    }
};
        var_fn434_calc_ig__iginbd_vgsat = assign36860_e33926;
        var_fn434_calc_ig__iginbd_vgsat_dn4 = assign36860_e33926_d_n4;

        let (assign36870_e33941, assign36870_e33941_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard435 == 0.0)) {
        let assign36870_e33935: f64 = (var_fn434_calc_ig__pg_paramin / var_fn434_calc_ig__phitin);
        let assign36870_e33937: f64 = (assign36870_e33935 * var_fn434_calc_ig__vgsatin);
        let assign36870_e33939: f64 = (assign36870_e33937 + var_fn434_calc_ig__expphib);
        (assign36870_e33939, (((-((var_fn434_calc_ig__pg_paramin * var_fn434_calc_ig__phitin_dn4) / (var_fn434_calc_ig__phitin * var_fn434_calc_ig__phitin))) * var_fn434_calc_ig__vgsatin) + var_fn434_calc_ig__expphib_dn4),)
    } else {
        (var_fn434_calc_ig__expiforarg_nohinj_vgsat, var_fn434_calc_ig__expiforarg_nohinj_vgsat_dn4,)
    }
};
        var_fn434_calc_ig__expiforarg_nohinj_vgsat = assign36870_e33941;
        var_fn434_calc_ig__expiforarg_nohinj_vgsat_dn4 = assign36870_e33941_d_n4;

        let (assign36880_e33988, assign36880_e33988_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard435 == 0.0)) {
        let assign36880_e33954: f64 = (-50.0);
        let (assign36880_e33986, assign36880_e33986_d_n4,) = {
            if ((!(var_fn434_calc_ig__expiforarg_nohinj_vgsat > 50.0)) && (!(var_fn434_calc_ig__expiforarg_nohinj_vgsat < assign36880_e33954))) {
                let assign36880_e33959: f64 = (var_fn434_calc_ig__expiforarg_nohinj_vgsat).exp();
                (assign36880_e33959, (assign36880_e33959 * var_fn434_calc_ig__expiforarg_nohinj_vgsat_dn4),)
            } else {
                let assign36880_e33966: f64 = (-50.0);
                let (assign36880_e33985, assign36880_e33985_d_n4,) = {
                    if ((!(var_fn434_calc_ig__expiforarg_nohinj_vgsat > 50.0)) && (var_fn434_calc_ig__expiforarg_nohinj_vgsat < assign36880_e33966)) {
                        let assign36880_e33970: f64 = (-50.0);
                        let assign36880_e33971: f64 = (assign36880_e33970).exp();
                        (assign36880_e33971, 0.0,)
                    } else {
                        let (assign36880_e33984, assign36880_e33984_d_n4,) = {
                            if (var_fn434_calc_ig__expiforarg_nohinj_vgsat > 50.0) {
                                let assign36880_e33976: f64 = (50.0_f64).exp();
                                let assign36880_e33980: f64 = (var_fn434_calc_ig__expiforarg_nohinj_vgsat - 50.0);
                                let assign36880_e33981: f64 = (1.0 + assign36880_e33980);
                                let assign36880_e33982: f64 = (assign36880_e33976 * assign36880_e33981);
                                (assign36880_e33982, (assign36880_e33976 * var_fn434_calc_ig__expiforarg_nohinj_vgsat_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign36880_e33984, assign36880_e33984_d_n4,)
                    }
                };
                (assign36880_e33985, assign36880_e33985_d_n4,)
            }
        };
        (assign36880_e33986, assign36880_e33986_d_n4,)
    } else {
        (var_fn434_calc_ig__expifor_nohinj_vgsat, var_fn434_calc_ig__expifor_nohinj_vgsat_dn4,)
    }
};
        var_fn434_calc_ig__expifor_nohinj_vgsat = assign36880_e33988;
        var_fn434_calc_ig__expifor_nohinj_vgsat_dn4 = assign36880_e33988_d_n4;

        let (assign36890_e34003, assign36890_e34003_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard435 == 0.0)) {
        let assign36890_e33998: f64 = (var_fn434_calc_ig__kbdgatein * var_fn434_calc_ig__iginbd_vgsat);
        let assign36890_e33999: f64 = (var_fn434_calc_ig__expifor_nohinj_vgsat - assign36890_e33998);
        let assign36890_e34001: f64 = (assign36890_e33999 - var_fn434_calc_ig__t0);
        (assign36890_e34001, ((var_fn434_calc_ig__expifor_nohinj_vgsat_dn4 - (var_fn434_calc_ig__kbdgatein * var_fn434_calc_ig__iginbd_vgsat_dn4)) - var_fn434_calc_ig__t0_dn4),)
    } else {
        (var_fn434_calc_ig__igindiode_nohinj_vgsat, var_fn434_calc_ig__igindiode_nohinj_vgsat_dn4,)
    }
};
        var_fn434_calc_ig__igindiode_nohinj_vgsat = assign36890_e34003;
        var_fn434_calc_ig__igindiode_nohinj_vgsat_dn4 = assign36890_e34003_d_n4;

        let (assign36900_e34020, assign36900_e34020_d_n4, assign36900_e34020_d_n8, assign36900_e34020_d_n17,) = {
    if (((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard435 == 0.0)) {
        let assign36900_e34014: f64 = (var_fn434_calc_ig__kbdgatein * var_fn434_calc_ig__iginbd);
        let assign36900_e34015: f64 = (var_fn434_calc_ig__expifor - assign36900_e34014);
        let assign36900_e34017: f64 = (assign36900_e34015 - var_fn434_calc_ig__t0);
        let assign36900_e34018: f64 = (var_fn434_calc_ig__isdiodeout * assign36900_e34017);
        (assign36900_e34018, ((var_fn434_calc_ig__isdiodeout_dn4 * assign36900_e34017) + (var_fn434_calc_ig__isdiodeout * ((var_fn434_calc_ig__expifor_dn4 - (var_fn434_calc_ig__kbdgatein * var_fn434_calc_ig__iginbd_dn4)) - var_fn434_calc_ig__t0_dn4))), (var_fn434_calc_ig__isdiodeout * (var_fn434_calc_ig__expifor_dn8 - (var_fn434_calc_ig__kbdgatein * var_fn434_calc_ig__iginbd_dn8))), (var_fn434_calc_ig__isdiodeout * (var_fn434_calc_ig__expifor_dn17 - (var_fn434_calc_ig__kbdgatein * var_fn434_calc_ig__iginbd_dn17))),)
    } else {
        (var_fn434_calc_ig__igindiode_nohinj, var_fn434_calc_ig__igindiode_nohinj_dn4, var_fn434_calc_ig__igindiode_nohinj_dn8, var_fn434_calc_ig__igindiode_nohinj_dn17,)
    }
};
        var_fn434_calc_ig__igindiode_nohinj = assign36900_e34020;
        var_fn434_calc_ig__igindiode_nohinj_dn4 = assign36900_e34020_d_n4;
        var_fn434_calc_ig__igindiode_nohinj_dn8 = assign36900_e34020_d_n8;
        var_fn434_calc_ig__igindiode_nohinj_dn17 = assign36900_e34020_d_n17;

        let assign36910_e34023: f64 = if var_fn434_calc_ig__fracin > 0.0 { 1.0 } else { 0.0 };
        var_guard436 = assign36910_e34023;

        let (assign36920_e34036,) = {
    if ((((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard435 == 0.0)) && (var_guard436 != 0.0)) {
        let assign36920_e34034: f64 = (var_fn434_calc_ig__fracin * var_fn434_calc_ig__pg_paramin);
        (assign36920_e34034,)
    } else {
        (var_fn434_calc_ig__pg_paramin_hinj,)
    }
};
        var_fn434_calc_ig__pg_paramin_hinj = assign36920_e34036;

        let (assign36930_e34053, assign36930_e34053_d_n4,) = {
    if ((((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard435 == 0.0)) && (var_guard436 != 0.0)) {
        let assign36930_e34047: f64 = (var_fn434_calc_ig__pg_paramin_hinj / var_fn434_calc_ig__phitin);
        let assign36930_e34049: f64 = (assign36930_e34047 * var_fn434_calc_ig__vgsatin);
        let assign36930_e34051: f64 = (assign36930_e34049 + var_fn434_calc_ig__expphib);
        (assign36930_e34051, (((-((var_fn434_calc_ig__pg_paramin_hinj * var_fn434_calc_ig__phitin_dn4) / (var_fn434_calc_ig__phitin * var_fn434_calc_ig__phitin))) * var_fn434_calc_ig__vgsatin) + var_fn434_calc_ig__expphib_dn4),)
    } else {
        (var_fn434_calc_ig__expiforarg_hinj_vgsat, var_fn434_calc_ig__expiforarg_hinj_vgsat_dn4,)
    }
};
        var_fn434_calc_ig__expiforarg_hinj_vgsat = assign36930_e34053;
        var_fn434_calc_ig__expiforarg_hinj_vgsat_dn4 = assign36930_e34053_d_n4;

        let (assign36940_e34102, assign36940_e34102_d_n4,) = {
    if ((((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard435 == 0.0)) && (var_guard436 != 0.0)) {
        let assign36940_e34068: f64 = (-50.0);
        let (assign36940_e34100, assign36940_e34100_d_n4,) = {
            if ((!(var_fn434_calc_ig__expiforarg_hinj_vgsat > 50.0)) && (!(var_fn434_calc_ig__expiforarg_hinj_vgsat < assign36940_e34068))) {
                let assign36940_e34073: f64 = (var_fn434_calc_ig__expiforarg_hinj_vgsat).exp();
                (assign36940_e34073, (assign36940_e34073 * var_fn434_calc_ig__expiforarg_hinj_vgsat_dn4),)
            } else {
                let assign36940_e34080: f64 = (-50.0);
                let (assign36940_e34099, assign36940_e34099_d_n4,) = {
                    if ((!(var_fn434_calc_ig__expiforarg_hinj_vgsat > 50.0)) && (var_fn434_calc_ig__expiforarg_hinj_vgsat < assign36940_e34080)) {
                        let assign36940_e34084: f64 = (-50.0);
                        let assign36940_e34085: f64 = (assign36940_e34084).exp();
                        (assign36940_e34085, 0.0,)
                    } else {
                        let (assign36940_e34098, assign36940_e34098_d_n4,) = {
                            if (var_fn434_calc_ig__expiforarg_hinj_vgsat > 50.0) {
                                let assign36940_e34090: f64 = (50.0_f64).exp();
                                let assign36940_e34094: f64 = (var_fn434_calc_ig__expiforarg_hinj_vgsat - 50.0);
                                let assign36940_e34095: f64 = (1.0 + assign36940_e34094);
                                let assign36940_e34096: f64 = (assign36940_e34090 * assign36940_e34095);
                                (assign36940_e34096, (assign36940_e34090 * var_fn434_calc_ig__expiforarg_hinj_vgsat_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign36940_e34098, assign36940_e34098_d_n4,)
                    }
                };
                (assign36940_e34099, assign36940_e34099_d_n4,)
            }
        };
        (assign36940_e34100, assign36940_e34100_d_n4,)
    } else {
        (var_fn434_calc_ig__expifor_hinj_vgsat, var_fn434_calc_ig__expifor_hinj_vgsat_dn4,)
    }
};
        var_fn434_calc_ig__expifor_hinj_vgsat = assign36940_e34102;
        var_fn434_calc_ig__expifor_hinj_vgsat_dn4 = assign36940_e34102_d_n4;

        let (assign36950_e34119, assign36950_e34119_d_n4,) = {
    if ((((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard435 == 0.0)) && (var_guard436 != 0.0)) {
        let assign36950_e34114: f64 = (var_fn434_calc_ig__kbdgatein * var_fn434_calc_ig__iginbd_vgsat);
        let assign36950_e34115: f64 = (var_fn434_calc_ig__expifor_hinj_vgsat - assign36950_e34114);
        let assign36950_e34117: f64 = (assign36950_e34115 - var_fn434_calc_ig__t0);
        (assign36950_e34117, ((var_fn434_calc_ig__expifor_hinj_vgsat_dn4 - (var_fn434_calc_ig__kbdgatein * var_fn434_calc_ig__iginbd_vgsat_dn4)) - var_fn434_calc_ig__t0_dn4),)
    } else {
        (var_fn434_calc_ig__igindiode_hinj_vgsat, var_fn434_calc_ig__igindiode_hinj_vgsat_dn4,)
    }
};
        var_fn434_calc_ig__igindiode_hinj_vgsat = assign36950_e34119;
        var_fn434_calc_ig__igindiode_hinj_vgsat_dn4 = assign36950_e34119_d_n4;

        let (assign36960_e34136, assign36960_e34136_d_n4, assign36960_e34136_d_n8, assign36960_e34136_d_n17,) = {
    if ((((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard435 == 0.0)) && (var_guard436 != 0.0)) {
        let assign36960_e34130: f64 = (var_fn434_calc_ig__pg_paramin_hinj / var_fn434_calc_ig__phitin);
        let assign36960_e34132: f64 = (assign36960_e34130 * var_fn434_calc_ig__vgin);
        let assign36960_e34134: f64 = (assign36960_e34132 + var_fn434_calc_ig__expphib);
        (assign36960_e34134, (((-((var_fn434_calc_ig__pg_paramin_hinj * var_fn434_calc_ig__phitin_dn4) / (var_fn434_calc_ig__phitin * var_fn434_calc_ig__phitin))) * var_fn434_calc_ig__vgin) + var_fn434_calc_ig__expphib_dn4), (assign36960_e34130 * var_fn434_calc_ig__vgin_dn8), (assign36960_e34130 * var_fn434_calc_ig__vgin_dn17),)
    } else {
        (var_fn434_calc_ig__expiforarg_hinj, var_fn434_calc_ig__expiforarg_hinj_dn4, var_fn434_calc_ig__expiforarg_hinj_dn8, var_fn434_calc_ig__expiforarg_hinj_dn17,)
    }
};
        var_fn434_calc_ig__expiforarg_hinj = assign36960_e34136;
        var_fn434_calc_ig__expiforarg_hinj_dn4 = assign36960_e34136_d_n4;
        var_fn434_calc_ig__expiforarg_hinj_dn8 = assign36960_e34136_d_n8;
        var_fn434_calc_ig__expiforarg_hinj_dn17 = assign36960_e34136_d_n17;

        let (assign36970_e34185, assign36970_e34185_d_n4, assign36970_e34185_d_n8, assign36970_e34185_d_n17,) = {
    if ((((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard435 == 0.0)) && (var_guard436 != 0.0)) {
        let assign36970_e34151: f64 = (-50.0);
        let (assign36970_e34183, assign36970_e34183_d_n4, assign36970_e34183_d_n8, assign36970_e34183_d_n17,) = {
            if ((!(var_fn434_calc_ig__expiforarg_hinj > 50.0)) && (!(var_fn434_calc_ig__expiforarg_hinj < assign36970_e34151))) {
                let assign36970_e34156: f64 = (var_fn434_calc_ig__expiforarg_hinj).exp();
                (assign36970_e34156, (assign36970_e34156 * var_fn434_calc_ig__expiforarg_hinj_dn4), (assign36970_e34156 * var_fn434_calc_ig__expiforarg_hinj_dn8), (assign36970_e34156 * var_fn434_calc_ig__expiforarg_hinj_dn17),)
            } else {
                let assign36970_e34163: f64 = (-50.0);
                let (assign36970_e34182, assign36970_e34182_d_n4, assign36970_e34182_d_n8, assign36970_e34182_d_n17,) = {
                    if ((!(var_fn434_calc_ig__expiforarg_hinj > 50.0)) && (var_fn434_calc_ig__expiforarg_hinj < assign36970_e34163)) {
                        let assign36970_e34167: f64 = (-50.0);
                        let assign36970_e34168: f64 = (assign36970_e34167).exp();
                        (assign36970_e34168, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign36970_e34181, assign36970_e34181_d_n4, assign36970_e34181_d_n8, assign36970_e34181_d_n17,) = {
                            if (var_fn434_calc_ig__expiforarg_hinj > 50.0) {
                                let assign36970_e34173: f64 = (50.0_f64).exp();
                                let assign36970_e34177: f64 = (var_fn434_calc_ig__expiforarg_hinj - 50.0);
                                let assign36970_e34178: f64 = (1.0 + assign36970_e34177);
                                let assign36970_e34179: f64 = (assign36970_e34173 * assign36970_e34178);
                                (assign36970_e34179, (assign36970_e34173 * var_fn434_calc_ig__expiforarg_hinj_dn4), (assign36970_e34173 * var_fn434_calc_ig__expiforarg_hinj_dn8), (assign36970_e34173 * var_fn434_calc_ig__expiforarg_hinj_dn17),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign36970_e34181, assign36970_e34181_d_n4, assign36970_e34181_d_n8, assign36970_e34181_d_n17,)
                    }
                };
                (assign36970_e34182, assign36970_e34182_d_n4, assign36970_e34182_d_n8, assign36970_e34182_d_n17,)
            }
        };
        (assign36970_e34183, assign36970_e34183_d_n4, assign36970_e34183_d_n8, assign36970_e34183_d_n17,)
    } else {
        (var_fn434_calc_ig__expifor_hinj, var_fn434_calc_ig__expifor_hinj_dn4, var_fn434_calc_ig__expifor_hinj_dn8, var_fn434_calc_ig__expifor_hinj_dn17,)
    }
};
        var_fn434_calc_ig__expifor_hinj = assign36970_e34185;
        var_fn434_calc_ig__expifor_hinj_dn4 = assign36970_e34185_d_n4;
        var_fn434_calc_ig__expifor_hinj_dn8 = assign36970_e34185_d_n8;
        var_fn434_calc_ig__expifor_hinj_dn17 = assign36970_e34185_d_n17;

        let (assign36980_e34200, assign36980_e34200_d_n4,) = {
    if ((((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard435 == 0.0)) && (var_guard436 != 0.0)) {
        let assign36980_e34196: f64 = (var_fn434_calc_ig__isdiodeout * var_fn434_calc_ig__igindiode_nohinj_vgsat);
        let assign36980_e34198: f64 = (assign36980_e34196 / var_fn434_calc_ig__igindiode_hinj_vgsat);
        (assign36980_e34198, (((((var_fn434_calc_ig__isdiodeout_dn4 * var_fn434_calc_ig__igindiode_nohinj_vgsat) + (var_fn434_calc_ig__isdiodeout * var_fn434_calc_ig__igindiode_nohinj_vgsat_dn4)) * var_fn434_calc_ig__igindiode_hinj_vgsat) - (assign36980_e34196 * var_fn434_calc_ig__igindiode_hinj_vgsat_dn4)) / (var_fn434_calc_ig__igindiode_hinj_vgsat * var_fn434_calc_ig__igindiode_hinj_vgsat)),)
    } else {
        (var_fn434_calc_ig__igindiode_hinj_pre, var_fn434_calc_ig__igindiode_hinj_pre_dn4,)
    }
};
        var_fn434_calc_ig__igindiode_hinj_pre = assign36980_e34200;
        var_fn434_calc_ig__igindiode_hinj_pre_dn4 = assign36980_e34200_d_n4;

        let (assign36990_e34219, assign36990_e34219_d_n4, assign36990_e34219_d_n8, assign36990_e34219_d_n17,) = {
    if ((((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard435 == 0.0)) && (var_guard436 != 0.0)) {
        let assign36990_e34213: f64 = (var_fn434_calc_ig__kbdgatein * var_fn434_calc_ig__iginbd);
        let assign36990_e34214: f64 = (var_fn434_calc_ig__expifor_hinj - assign36990_e34213);
        let assign36990_e34216: f64 = (assign36990_e34214 - var_fn434_calc_ig__t0);
        let assign36990_e34217: f64 = (var_fn434_calc_ig__igindiode_hinj_pre * assign36990_e34216);
        (assign36990_e34217, ((var_fn434_calc_ig__igindiode_hinj_pre_dn4 * assign36990_e34216) + (var_fn434_calc_ig__igindiode_hinj_pre * ((var_fn434_calc_ig__expifor_hinj_dn4 - (var_fn434_calc_ig__kbdgatein * var_fn434_calc_ig__iginbd_dn4)) - var_fn434_calc_ig__t0_dn4))), (var_fn434_calc_ig__igindiode_hinj_pre * (var_fn434_calc_ig__expifor_hinj_dn8 - (var_fn434_calc_ig__kbdgatein * var_fn434_calc_ig__iginbd_dn8))), (var_fn434_calc_ig__igindiode_hinj_pre * (var_fn434_calc_ig__expifor_hinj_dn17 - (var_fn434_calc_ig__kbdgatein * var_fn434_calc_ig__iginbd_dn17))),)
    } else {
        (var_fn434_calc_ig__igindiode_hinj, var_fn434_calc_ig__igindiode_hinj_dn4, var_fn434_calc_ig__igindiode_hinj_dn8, var_fn434_calc_ig__igindiode_hinj_dn17,)
    }
};
        var_fn434_calc_ig__igindiode_hinj = assign36990_e34219;
        var_fn434_calc_ig__igindiode_hinj_dn4 = assign36990_e34219_d_n4;
        var_fn434_calc_ig__igindiode_hinj_dn8 = assign36990_e34219_d_n8;
        var_fn434_calc_ig__igindiode_hinj_dn17 = assign36990_e34219_d_n17;

        let (assign37000_e34233, assign37000_e34233_d_n4, assign37000_e34233_d_n8, assign37000_e34233_d_n17,) = {
    if ((((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard435 == 0.0)) && (var_guard436 == 0.0)) {
        let assign37000_e34231: f64 = (var_fn434_calc_ig__isdiodeout * var_fn434_calc_ig__igindiode_nohinj_vgsat);
        (assign37000_e34231, ((var_fn434_calc_ig__isdiodeout_dn4 * var_fn434_calc_ig__igindiode_nohinj_vgsat) + (var_fn434_calc_ig__isdiodeout * var_fn434_calc_ig__igindiode_nohinj_vgsat_dn4)), 0.0, 0.0,)
    } else {
        (var_fn434_calc_ig__igindiode_hinj, var_fn434_calc_ig__igindiode_hinj_dn4, var_fn434_calc_ig__igindiode_hinj_dn8, var_fn434_calc_ig__igindiode_hinj_dn17,)
    }
};
        var_fn434_calc_ig__igindiode_hinj = assign37000_e34233;
        var_fn434_calc_ig__igindiode_hinj_dn4 = assign37000_e34233_d_n4;
        var_fn434_calc_ig__igindiode_hinj_dn8 = assign37000_e34233_d_n8;
        var_fn434_calc_ig__igindiode_hinj_dn17 = assign37000_e34233_d_n17;

        let (assign37010_e34246, assign37010_e34246_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard435 == 0.0)) {
        let assign37010_e34242: f64 = (var_fn434_calc_ig__alphagin * var_fn434_calc_ig__alphagin);
        let assign37010_e34244: f64 = (assign37010_e34242 * var_fn434_calc_ig__phitin);
        (assign37010_e34244, (assign37010_e34242 * var_fn434_calc_ig__phitin_dn4),)
    } else {
        (var_fn434_calc_ig__alpha2_phit, var_fn434_calc_ig__alpha2_phit_dn4,)
    }
};
        var_fn434_calc_ig__alpha2_phit = assign37010_e34246;
        var_fn434_calc_ig__alpha2_phit_dn4 = assign37010_e34246_d_n4;

        let (assign37020_e34263, assign37020_e34263_d_n4, assign37020_e34263_d_n8, assign37020_e34263_d_n17,) = {
    if (((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard435 == 0.0)) {
        let assign37020_e34257: f64 = (var_fn434_calc_ig__alpha2_phit / 2.0);
        let assign37020_e34258: f64 = (var_fn434_calc_ig__vgsatin - assign37020_e34257);
        let assign37020_e34259: f64 = (var_fn434_calc_ig__vgin - assign37020_e34258);
        let assign37020_e34261: f64 = (assign37020_e34259 / var_fn434_calc_ig__alpha2_phit);
        (assign37020_e34261, ((((-(-(var_fn434_calc_ig__alpha2_phit_dn4 / 2.0))) * var_fn434_calc_ig__alpha2_phit) - (assign37020_e34259 * var_fn434_calc_ig__alpha2_phit_dn4)) / (var_fn434_calc_ig__alpha2_phit * var_fn434_calc_ig__alpha2_phit)), (var_fn434_calc_ig__vgin_dn8 / var_fn434_calc_ig__alpha2_phit), (var_fn434_calc_ig__vgin_dn17 / var_fn434_calc_ig__alpha2_phit),)
    } else {
        (var_fn434_calc_ig__expffvarg, var_fn434_calc_ig__expffvarg_dn4, var_fn434_calc_ig__expffvarg_dn8, var_fn434_calc_ig__expffvarg_dn17,)
    }
};
        var_fn434_calc_ig__expffvarg = assign37020_e34263;
        var_fn434_calc_ig__expffvarg_dn4 = assign37020_e34263_d_n4;
        var_fn434_calc_ig__expffvarg_dn8 = assign37020_e34263_d_n8;
        var_fn434_calc_ig__expffvarg_dn17 = assign37020_e34263_d_n17;

        let assign37030_e34266: f64 = if var_fn434_calc_ig__expffvarg > 50.0 { 1.0 } else { 0.0 };
        var_guard437 = assign37030_e34266;

        let (assign37040_e34277, assign37040_e34277_d_n4, assign37040_e34277_d_n8, assign37040_e34277_d_n17,) = {
    if ((((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard435 == 0.0)) && (var_guard437 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn434_calc_ig__ffvgin, var_fn434_calc_ig__ffvgin_dn4, var_fn434_calc_ig__ffvgin_dn8, var_fn434_calc_ig__ffvgin_dn17,)
    }
};
        var_fn434_calc_ig__ffvgin = assign37040_e34277;
        var_fn434_calc_ig__ffvgin_dn4 = assign37040_e34277_d_n4;
        var_fn434_calc_ig__ffvgin_dn8 = assign37040_e34277_d_n8;
        var_fn434_calc_ig__ffvgin_dn17 = assign37040_e34277_d_n17;

        let assign37050_e34280: f64 = (-50.0);
        let assign37050_e34281: f64 = if var_fn434_calc_ig__expffvarg < assign37050_e34280 { 1.0 } else { 0.0 };
        var_guard438 = assign37050_e34281;

        let (assign37060_e34295, assign37060_e34295_d_n4, assign37060_e34295_d_n8, assign37060_e34295_d_n17,) = {
    if (((((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard435 == 0.0)) && (var_guard437 == 0.0)) && (var_guard438 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn434_calc_ig__ffvgin, var_fn434_calc_ig__ffvgin_dn4, var_fn434_calc_ig__ffvgin_dn8, var_fn434_calc_ig__ffvgin_dn17,)
    }
};
        var_fn434_calc_ig__ffvgin = assign37060_e34295;
        var_fn434_calc_ig__ffvgin_dn4 = assign37060_e34295_d_n4;
        var_fn434_calc_ig__ffvgin_dn8 = assign37060_e34295_d_n8;
        var_fn434_calc_ig__ffvgin_dn17 = assign37060_e34295_d_n17;

        let (assign37070_e34315, assign37070_e34315_d_n4, assign37070_e34315_d_n8, assign37070_e34315_d_n17,) = {
    if (((((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard435 == 0.0)) && (var_guard437 == 0.0)) && (var_guard438 == 0.0)) {
        let assign37070_e34311: f64 = (var_fn434_calc_ig__expffvarg).exp();
        let assign37070_e34312: f64 = (1.0 + assign37070_e34311);
        let assign37070_e34313: f64 = (1.0 / assign37070_e34312);
        (assign37070_e34313, (-((assign37070_e34311 * var_fn434_calc_ig__expffvarg_dn4) / (assign37070_e34312 * assign37070_e34312))), (-((assign37070_e34311 * var_fn434_calc_ig__expffvarg_dn8) / (assign37070_e34312 * assign37070_e34312))), (-((assign37070_e34311 * var_fn434_calc_ig__expffvarg_dn17) / (assign37070_e34312 * assign37070_e34312))),)
    } else {
        (var_fn434_calc_ig__ffvgin, var_fn434_calc_ig__ffvgin_dn4, var_fn434_calc_ig__ffvgin_dn8, var_fn434_calc_ig__ffvgin_dn17,)
    }
};
        var_fn434_calc_ig__ffvgin = assign37070_e34315;
        var_fn434_calc_ig__ffvgin_dn4 = assign37070_e34315_d_n4;
        var_fn434_calc_ig__ffvgin_dn8 = assign37070_e34315_d_n8;
        var_fn434_calc_ig__ffvgin_dn17 = assign37070_e34315_d_n17;

        let (assign37080_e34332, assign37080_e34332_d_n4, assign37080_e34332_d_n8, assign37080_e34332_d_n17,) = {
    if (((var_guard417 != 0.0) && (var_guard428 != 0.0)) && (var_guard435 == 0.0)) {
        let assign37080_e34324: f64 = (var_fn434_calc_ig__ffvgin * var_fn434_calc_ig__igindiode_nohinj);
        let assign37080_e34327: f64 = (1.0 - var_fn434_calc_ig__ffvgin);
        let assign37080_e34329: f64 = (assign37080_e34327 * var_fn434_calc_ig__igindiode_hinj);
        let assign37080_e34330: f64 = (assign37080_e34324 + assign37080_e34329);
        (assign37080_e34330, (((var_fn434_calc_ig__ffvgin_dn4 * var_fn434_calc_ig__igindiode_nohinj) + (var_fn434_calc_ig__ffvgin * var_fn434_calc_ig__igindiode_nohinj_dn4)) + (((-var_fn434_calc_ig__ffvgin_dn4) * var_fn434_calc_ig__igindiode_hinj) + (assign37080_e34327 * var_fn434_calc_ig__igindiode_hinj_dn4))), (((var_fn434_calc_ig__ffvgin_dn8 * var_fn434_calc_ig__igindiode_nohinj) + (var_fn434_calc_ig__ffvgin * var_fn434_calc_ig__igindiode_nohinj_dn8)) + (((-var_fn434_calc_ig__ffvgin_dn8) * var_fn434_calc_ig__igindiode_hinj) + (assign37080_e34327 * var_fn434_calc_ig__igindiode_hinj_dn8))), (((var_fn434_calc_ig__ffvgin_dn17 * var_fn434_calc_ig__igindiode_nohinj) + (var_fn434_calc_ig__ffvgin * var_fn434_calc_ig__igindiode_nohinj_dn17)) + (((-var_fn434_calc_ig__ffvgin_dn17) * var_fn434_calc_ig__igindiode_hinj) + (assign37080_e34327 * var_fn434_calc_ig__igindiode_hinj_dn17))),)
    } else {
        (var_fn434_calc_ig__igindiode, var_fn434_calc_ig__igindiode_dn4, var_fn434_calc_ig__igindiode_dn8, var_fn434_calc_ig__igindiode_dn17,)
    }
};
        var_fn434_calc_ig__igindiode = assign37080_e34332;
        var_fn434_calc_ig__igindiode_dn4 = assign37080_e34332_d_n4;
        var_fn434_calc_ig__igindiode_dn8 = assign37080_e34332_d_n8;
        var_fn434_calc_ig__igindiode_dn17 = assign37080_e34332_d_n17;

        let (assign37090_e34380, assign37090_e34380_d_n8, assign37090_e34380_d_n17,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        let assign37090_e34337: f64 = (-var_fn434_calc_ig__vgin);
        let (assign37090_e34370, assign37090_e34370_d_n8, assign37090_e34370_d_n17,) = {
            if (p.p52 != 0.0) {
                let assign37090_e34345: f64 = (var_fn434_calc_ig__vgin / var_fn434_calc_ig__vgsatqin);
                let assign37090_e34348: f64 = (0.001 / p.p53);
                let assign37090_e34351: f64 = (var_fn434_calc_ig__vgin / var_fn434_calc_ig__vgsatqin);
                let assign37090_e34352: f64 = (assign37090_e34348 * assign37090_e34351);
                let assign37090_e34353: f64 = (assign37090_e34352).tanh();
                let assign37090_e34354: f64 = (assign37090_e34345 * assign37090_e34353);
                (assign37090_e34354, (((var_fn434_calc_ig__vgin_dn8 / var_fn434_calc_ig__vgsatqin) * assign37090_e34353) + (assign37090_e34345 * ((assign37090_e34348 * (var_fn434_calc_ig__vgin_dn8 / var_fn434_calc_ig__vgsatqin)) / ((assign37090_e34352).cosh() * (assign37090_e34352).cosh())))), (((var_fn434_calc_ig__vgin_dn17 / var_fn434_calc_ig__vgsatqin) * assign37090_e34353) + (assign37090_e34345 * ((assign37090_e34348 * (var_fn434_calc_ig__vgin_dn17 / var_fn434_calc_ig__vgsatqin)) / ((assign37090_e34352).cosh() * (assign37090_e34352).cosh())))),)
            } else {
                let (assign37090_e34369, assign37090_e34369_d_n8, assign37090_e34369_d_n17,) = {
                    if (p.p52 == 0.0) {
                        let __rspice_inv_cse_0: f64 = 1.0 / var_fn434_calc_ig__vgsatqin;
                        let assign37090_e34360: f64 = (var_fn434_calc_ig__vgin * __rspice_inv_cse_0);
                        let assign37090_e34363: f64 = (var_fn434_calc_ig__vgin * __rspice_inv_cse_0);
                        let assign37090_e34364: f64 = (assign37090_e34360 * assign37090_e34363);
                        let assign37090_e34366: f64 = (assign37090_e34364 + p.p53);
                        let assign37090_e34367: f64 = (assign37090_e34366).sqrt();
                        (assign37090_e34367, ((((var_fn434_calc_ig__vgin_dn8 / var_fn434_calc_ig__vgsatqin) * assign37090_e34363) + (assign37090_e34360 * (var_fn434_calc_ig__vgin_dn8 / var_fn434_calc_ig__vgsatqin))) / (2.0 * assign37090_e34367)), ((((var_fn434_calc_ig__vgin_dn17 / var_fn434_calc_ig__vgsatqin) * assign37090_e34363) + (assign37090_e34360 * (var_fn434_calc_ig__vgin_dn17 / var_fn434_calc_ig__vgsatqin))) / (2.0 * assign37090_e34367)),)
                    } else {
                        (0.0, 0.0, 0.0,)
                    }
                };
                (assign37090_e34369, assign37090_e34369_d_n8, assign37090_e34369_d_n17,)
            }
        };
        let assign37090_e34372: f64 = (assign37090_e34370).powf(var_fn434_calc_ig__betarecin);
        let assign37090_e34373: f64 = (1.0 + assign37090_e34372);
        let assign37090_e34376: f64 = (1.0 / var_fn434_calc_ig__betarecin);
        let assign37090_e34377: f64 = (assign37090_e34373).powf(assign37090_e34376);
        let assign37090_e34378: f64 = (assign37090_e34337 / assign37090_e34377);
        (assign37090_e34378, ((((-var_fn434_calc_ig__vgin_dn8) * assign37090_e34377) - (assign37090_e34337 * if 0.0 == 0.0 && ((assign37090_e34376) as f64).is_finite() && ((assign37090_e34376) as f64).fract() == 0.0 { if assign37090_e34376 == 0.0 { 0.0 } else { (assign37090_e34376 * ((assign37090_e34373).powf(assign37090_e34376 - 1.0) * if 0.0 == 0.0 && ((var_fn434_calc_ig__betarecin) as f64).is_finite() && ((var_fn434_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn434_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn434_calc_ig__betarecin * ((assign37090_e34370).powf(var_fn434_calc_ig__betarecin - 1.0) * assign37090_e34370_d_n8)) } } else { (assign37090_e34372 * (var_fn434_calc_ig__betarecin * (assign37090_e34370_d_n8 / assign37090_e34370))) })) } } else { (assign37090_e34377 * (assign37090_e34376 * (if 0.0 == 0.0 && ((var_fn434_calc_ig__betarecin) as f64).is_finite() && ((var_fn434_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn434_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn434_calc_ig__betarecin * ((assign37090_e34370).powf(var_fn434_calc_ig__betarecin - 1.0) * assign37090_e34370_d_n8)) } } else { (assign37090_e34372 * (var_fn434_calc_ig__betarecin * (assign37090_e34370_d_n8 / assign37090_e34370))) } / assign37090_e34373))) })) / (assign37090_e34377 * assign37090_e34377)), ((((-var_fn434_calc_ig__vgin_dn17) * assign37090_e34377) - (assign37090_e34337 * if 0.0 == 0.0 && ((assign37090_e34376) as f64).is_finite() && ((assign37090_e34376) as f64).fract() == 0.0 { if assign37090_e34376 == 0.0 { 0.0 } else { (assign37090_e34376 * ((assign37090_e34373).powf(assign37090_e34376 - 1.0) * if 0.0 == 0.0 && ((var_fn434_calc_ig__betarecin) as f64).is_finite() && ((var_fn434_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn434_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn434_calc_ig__betarecin * ((assign37090_e34370).powf(var_fn434_calc_ig__betarecin - 1.0) * assign37090_e34370_d_n17)) } } else { (assign37090_e34372 * (var_fn434_calc_ig__betarecin * (assign37090_e34370_d_n17 / assign37090_e34370))) })) } } else { (assign37090_e34377 * (assign37090_e34376 * (if 0.0 == 0.0 && ((var_fn434_calc_ig__betarecin) as f64).is_finite() && ((var_fn434_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn434_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn434_calc_ig__betarecin * ((assign37090_e34370).powf(var_fn434_calc_ig__betarecin - 1.0) * assign37090_e34370_d_n17)) } } else { (assign37090_e34372 * (var_fn434_calc_ig__betarecin * (assign37090_e34370_d_n17 / assign37090_e34370))) } / assign37090_e34373))) })) / (assign37090_e34377 * assign37090_e34377)),)
    } else {
        (var_fn434_calc_ig__frecgin, var_fn434_calc_ig__frecgin_dn8, var_fn434_calc_ig__frecgin_dn17,)
    }
};
        var_fn434_calc_ig__frecgin = assign37090_e34380;
        var_fn434_calc_ig__frecgin_dn8 = assign37090_e34380_d_n8;
        var_fn434_calc_ig__frecgin_dn17 = assign37090_e34380_d_n17;

        let (assign37100_e34397, assign37100_e34397_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        let assign37100_e34385: f64 = (-var_fn434_calc_ig__type);
        let assign37100_e34387: f64 = (assign37100_e34385 * var_fn434_calc_ig__w);
        let assign37100_e34389: f64 = (assign37100_e34387 * var_fn434_calc_ig__ngf);
        let assign37100_e34391: f64 = (assign37100_e34389 * var_fn434_calc_ig__irecin);
        let assign37100_e34393: f64 = (assign37100_e34391 * var_fn434_calc_ig__tfacdiodein);
        let assign37100_e34395: f64 = assign37100_e34393;
        (assign37100_e34395, (assign37100_e34391 * var_fn434_calc_ig__tfacdiodein_dn4),)
    } else {
        (var_fn434_calc_ig__isrecout, var_fn434_calc_ig__isrecout_dn4,)
    }
};
        var_fn434_calc_ig__isrecout = assign37100_e34397;
        var_fn434_calc_ig__isrecout_dn4 = assign37100_e34397_d_n4;

        let (assign37110_e34407, assign37110_e34407_d_n4, assign37110_e34407_d_n8, assign37110_e34407_d_n17,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        let assign37110_e34403: f64 = (var_fn434_calc_ig__pgsrecin / var_fn434_calc_ig__phitin);
        let assign37110_e34405: f64 = (assign37110_e34403 * var_fn434_calc_ig__frecgin);
        (assign37110_e34405, ((-((var_fn434_calc_ig__pgsrecin * var_fn434_calc_ig__phitin_dn4) / (var_fn434_calc_ig__phitin * var_fn434_calc_ig__phitin))) * var_fn434_calc_ig__frecgin), (assign37110_e34403 * var_fn434_calc_ig__frecgin_dn8), (assign37110_e34403 * var_fn434_calc_ig__frecgin_dn17),)
    } else {
        (var_fn434_calc_ig__expirevarg, var_fn434_calc_ig__expirevarg_dn4, var_fn434_calc_ig__expirevarg_dn8, var_fn434_calc_ig__expirevarg_dn17,)
    }
};
        var_fn434_calc_ig__expirevarg = assign37110_e34407;
        var_fn434_calc_ig__expirevarg_dn4 = assign37110_e34407_d_n4;
        var_fn434_calc_ig__expirevarg_dn8 = assign37110_e34407_d_n8;
        var_fn434_calc_ig__expirevarg_dn17 = assign37110_e34407_d_n17;

        *var_fn434_calc_ig__alpha2_phit_slot = var_fn434_calc_ig__alpha2_phit;
        *var_fn434_calc_ig__alpha2_phit_dn4_slot = var_fn434_calc_ig__alpha2_phit_dn4;
        *var_fn434_calc_ig__expbd1_vgsat_slot = var_fn434_calc_ig__expbd1_vgsat;
        *var_fn434_calc_ig__expbd1_vgsat_dn4_slot = var_fn434_calc_ig__expbd1_vgsat_dn4;
        *var_fn434_calc_ig__expbdarg1_vgsat_slot = var_fn434_calc_ig__expbdarg1_vgsat;
        *var_fn434_calc_ig__expbdarg1_vgsat_dn4_slot = var_fn434_calc_ig__expbdarg1_vgsat_dn4;
        *var_fn434_calc_ig__expffvarg_slot = var_fn434_calc_ig__expffvarg;
        *var_fn434_calc_ig__expffvarg_dn17_slot = var_fn434_calc_ig__expffvarg_dn17;
        *var_fn434_calc_ig__expffvarg_dn4_slot = var_fn434_calc_ig__expffvarg_dn4;
        *var_fn434_calc_ig__expffvarg_dn8_slot = var_fn434_calc_ig__expffvarg_dn8;
        *var_fn434_calc_ig__expifor_hinj_slot = var_fn434_calc_ig__expifor_hinj;
        *var_fn434_calc_ig__expifor_hinj_dn17_slot = var_fn434_calc_ig__expifor_hinj_dn17;
        *var_fn434_calc_ig__expifor_hinj_dn4_slot = var_fn434_calc_ig__expifor_hinj_dn4;
        *var_fn434_calc_ig__expifor_hinj_dn8_slot = var_fn434_calc_ig__expifor_hinj_dn8;
        *var_fn434_calc_ig__expifor_hinj_vgsat_slot = var_fn434_calc_ig__expifor_hinj_vgsat;
        *var_fn434_calc_ig__expifor_hinj_vgsat_dn4_slot = var_fn434_calc_ig__expifor_hinj_vgsat_dn4;
        *var_fn434_calc_ig__expifor_nohinj_vgsat_slot = var_fn434_calc_ig__expifor_nohinj_vgsat;
        *var_fn434_calc_ig__expifor_nohinj_vgsat_dn4_slot = var_fn434_calc_ig__expifor_nohinj_vgsat_dn4;
        *var_fn434_calc_ig__expiforarg_hinj_slot = var_fn434_calc_ig__expiforarg_hinj;
        *var_fn434_calc_ig__expiforarg_hinj_dn17_slot = var_fn434_calc_ig__expiforarg_hinj_dn17;
        *var_fn434_calc_ig__expiforarg_hinj_dn4_slot = var_fn434_calc_ig__expiforarg_hinj_dn4;
        *var_fn434_calc_ig__expiforarg_hinj_dn8_slot = var_fn434_calc_ig__expiforarg_hinj_dn8;
        *var_fn434_calc_ig__expiforarg_hinj_vgsat_slot = var_fn434_calc_ig__expiforarg_hinj_vgsat;
        *var_fn434_calc_ig__expiforarg_hinj_vgsat_dn4_slot = var_fn434_calc_ig__expiforarg_hinj_vgsat_dn4;
        *var_fn434_calc_ig__expiforarg_nohinj_vgsat_slot = var_fn434_calc_ig__expiforarg_nohinj_vgsat;
        *var_fn434_calc_ig__expiforarg_nohinj_vgsat_dn4_slot = var_fn434_calc_ig__expiforarg_nohinj_vgsat_dn4;
        *var_fn434_calc_ig__expirevarg_slot = var_fn434_calc_ig__expirevarg;
        *var_fn434_calc_ig__expirevarg_dn17_slot = var_fn434_calc_ig__expirevarg_dn17;
        *var_fn434_calc_ig__expirevarg_dn4_slot = var_fn434_calc_ig__expirevarg_dn4;
        *var_fn434_calc_ig__expirevarg_dn8_slot = var_fn434_calc_ig__expirevarg_dn8;
        *var_fn434_calc_ig__ffvgin_slot = var_fn434_calc_ig__ffvgin;
        *var_fn434_calc_ig__ffvgin_dn17_slot = var_fn434_calc_ig__ffvgin_dn17;
        *var_fn434_calc_ig__ffvgin_dn4_slot = var_fn434_calc_ig__ffvgin_dn4;
        *var_fn434_calc_ig__ffvgin_dn8_slot = var_fn434_calc_ig__ffvgin_dn8;
        *var_fn434_calc_ig__frecgin_slot = var_fn434_calc_ig__frecgin;
        *var_fn434_calc_ig__frecgin_dn17_slot = var_fn434_calc_ig__frecgin_dn17;
        *var_fn434_calc_ig__frecgin_dn8_slot = var_fn434_calc_ig__frecgin_dn8;
        *var_fn434_calc_ig__iginbd_vgsat_slot = var_fn434_calc_ig__iginbd_vgsat;
        *var_fn434_calc_ig__iginbd_vgsat_dn4_slot = var_fn434_calc_ig__iginbd_vgsat_dn4;
        *var_fn434_calc_ig__igindiode_slot = var_fn434_calc_ig__igindiode;
        *var_fn434_calc_ig__igindiode_dn17_slot = var_fn434_calc_ig__igindiode_dn17;
        *var_fn434_calc_ig__igindiode_dn4_slot = var_fn434_calc_ig__igindiode_dn4;
        *var_fn434_calc_ig__igindiode_dn8_slot = var_fn434_calc_ig__igindiode_dn8;
        *var_fn434_calc_ig__igindiode_hinj_slot = var_fn434_calc_ig__igindiode_hinj;
        *var_fn434_calc_ig__igindiode_hinj_dn17_slot = var_fn434_calc_ig__igindiode_hinj_dn17;
        *var_fn434_calc_ig__igindiode_hinj_dn4_slot = var_fn434_calc_ig__igindiode_hinj_dn4;
        *var_fn434_calc_ig__igindiode_hinj_dn8_slot = var_fn434_calc_ig__igindiode_hinj_dn8;
        *var_fn434_calc_ig__igindiode_hinj_pre_slot = var_fn434_calc_ig__igindiode_hinj_pre;
        *var_fn434_calc_ig__igindiode_hinj_pre_dn4_slot = var_fn434_calc_ig__igindiode_hinj_pre_dn4;
        *var_fn434_calc_ig__igindiode_hinj_vgsat_slot = var_fn434_calc_ig__igindiode_hinj_vgsat;
        *var_fn434_calc_ig__igindiode_hinj_vgsat_dn4_slot = var_fn434_calc_ig__igindiode_hinj_vgsat_dn4;
        *var_fn434_calc_ig__igindiode_nohinj_slot = var_fn434_calc_ig__igindiode_nohinj;
        *var_fn434_calc_ig__igindiode_nohinj_dn17_slot = var_fn434_calc_ig__igindiode_nohinj_dn17;
        *var_fn434_calc_ig__igindiode_nohinj_dn4_slot = var_fn434_calc_ig__igindiode_nohinj_dn4;
        *var_fn434_calc_ig__igindiode_nohinj_dn8_slot = var_fn434_calc_ig__igindiode_nohinj_dn8;
        *var_fn434_calc_ig__igindiode_nohinj_vgsat_slot = var_fn434_calc_ig__igindiode_nohinj_vgsat;
        *var_fn434_calc_ig__igindiode_nohinj_vgsat_dn4_slot = var_fn434_calc_ig__igindiode_nohinj_vgsat_dn4;
        *var_fn434_calc_ig__isrecout_slot = var_fn434_calc_ig__isrecout;
        *var_fn434_calc_ig__isrecout_dn4_slot = var_fn434_calc_ig__isrecout_dn4;
        *var_fn434_calc_ig__pg_paramin_hinj_slot = var_fn434_calc_ig__pg_paramin_hinj;
        *var_guard436_slot = var_guard436;
        *var_guard437_slot = var_guard437;
        *var_guard438_slot = var_guard438;
    }

    pub(super) fn stamp_transient_block_92(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_fn434_calc_ig__expirevarg: f64,
        var_fn434_calc_ig__expirevarg_dn17: f64,
        var_fn434_calc_ig__expirevarg_dn4: f64,
        var_fn434_calc_ig__expirevarg_dn8: f64,
        var_fn434_calc_ig__igindiode: f64,
        var_fn434_calc_ig__igindiode_dn17: f64,
        var_fn434_calc_ig__igindiode_dn4: f64,
        var_fn434_calc_ig__igindiode_dn8: f64,
        var_fn434_calc_ig__isrecout: f64,
        var_fn434_calc_ig__isrecout_dn4: f64,
        var_guard417: f64,
        var_guard428: f64,
        var_phit: f64,
        var_phit_dn4: f64,
        var_tfacdiode: f64,
        var_tfacdiode_dn4: f64,
        var_fn434_calc_ig__expirev_slot: &mut f64,
        var_fn434_calc_ig__expirev_dn17_slot: &mut f64,
        var_fn434_calc_ig__expirev_dn4_slot: &mut f64,
        var_fn434_calc_ig__expirev_dn8_slot: &mut f64,
        var_fn434_calc_ig__iginrec_slot: &mut f64,
        var_fn434_calc_ig__iginrec_dn17_slot: &mut f64,
        var_fn434_calc_ig__iginrec_dn4_slot: &mut f64,
        var_fn434_calc_ig__iginrec_dn8_slot: &mut f64,
        var_fn434_calc_ig__igout_slot: &mut f64,
        var_fn434_calc_ig__igout_dn17_slot: &mut f64,
        var_fn434_calc_ig__igout_dn4_slot: &mut f64,
        var_fn434_calc_ig__igout_dn8_slot: &mut f64,
        var_fn434_calc_ig__return_slot: &mut f64,
        var_fn434_calc_ig__return_dn17_slot: &mut f64,
        var_fn434_calc_ig__return_dn4_slot: &mut f64,
        var_fn434_calc_ig__return_dn8_slot: &mut f64,
        var_fn440_calc_ig__alpha2_phit_slot: &mut f64,
        var_fn440_calc_ig__alpha2_phit_dn4_slot: &mut f64,
        var_fn440_calc_ig__alphagin_slot: &mut f64,
        var_fn440_calc_ig__betarecin_slot: &mut f64,
        var_fn440_calc_ig__expbd1_slot: &mut f64,
        var_fn440_calc_ig__expbd1_dn4_slot: &mut f64,
        var_fn440_calc_ig__expbd1_dn8_slot: &mut f64,
        var_fn440_calc_ig__expbd1_dn9_slot: &mut f64,
        var_fn440_calc_ig__expbd2_slot: &mut f64,
        var_fn440_calc_ig__expbd2_dn4_slot: &mut f64,
        var_fn440_calc_ig__expbdarg1_slot: &mut f64,
        var_fn440_calc_ig__expbdarg1_dn4_slot: &mut f64,
        var_fn440_calc_ig__expbdarg1_dn8_slot: &mut f64,
        var_fn440_calc_ig__expbdarg1_dn9_slot: &mut f64,
        var_fn440_calc_ig__expbdarg2_slot: &mut f64,
        var_fn440_calc_ig__expbdarg2_dn4_slot: &mut f64,
        var_fn440_calc_ig__expffvarg_slot: &mut f64,
        var_fn440_calc_ig__expffvarg_dn4_slot: &mut f64,
        var_fn440_calc_ig__expffvarg_dn8_slot: &mut f64,
        var_fn440_calc_ig__expffvarg_dn9_slot: &mut f64,
        var_fn440_calc_ig__expifor_slot: &mut f64,
        var_fn440_calc_ig__expifor_dn4_slot: &mut f64,
        var_fn440_calc_ig__expifor_dn8_slot: &mut f64,
        var_fn440_calc_ig__expifor_dn9_slot: &mut f64,
        var_fn440_calc_ig__expiforarg_slot: &mut f64,
        var_fn440_calc_ig__expiforarg_dn4_slot: &mut f64,
        var_fn440_calc_ig__expiforarg_dn8_slot: &mut f64,
        var_fn440_calc_ig__expiforarg_dn9_slot: &mut f64,
        var_fn440_calc_ig__expirevarg_slot: &mut f64,
        var_fn440_calc_ig__expirevarg_dn4_slot: &mut f64,
        var_fn440_calc_ig__expirevarg_dn8_slot: &mut f64,
        var_fn440_calc_ig__expirevarg_dn9_slot: &mut f64,
        var_fn440_calc_ig__expphib_slot: &mut f64,
        var_fn440_calc_ig__expphib_dn4_slot: &mut f64,
        var_fn440_calc_ig__ffvgin_slot: &mut f64,
        var_fn440_calc_ig__ffvgin_dn4_slot: &mut f64,
        var_fn440_calc_ig__ffvgin_dn8_slot: &mut f64,
        var_fn440_calc_ig__ffvgin_dn9_slot: &mut f64,
        var_fn440_calc_ig__fracin_slot: &mut f64,
        var_fn440_calc_ig__frecgin_slot: &mut f64,
        var_fn440_calc_ig__frecgin_dn8_slot: &mut f64,
        var_fn440_calc_ig__frecgin_dn9_slot: &mut f64,
        var_fn440_calc_ig__iginbd_slot: &mut f64,
        var_fn440_calc_ig__iginbd_dn4_slot: &mut f64,
        var_fn440_calc_ig__iginbd_dn8_slot: &mut f64,
        var_fn440_calc_ig__iginbd_dn9_slot: &mut f64,
        var_fn440_calc_ig__igindiode_slot: &mut f64,
        var_fn440_calc_ig__igindiode_dn4_slot: &mut f64,
        var_fn440_calc_ig__igindiode_dn8_slot: &mut f64,
        var_fn440_calc_ig__igindiode_dn9_slot: &mut f64,
        var_fn440_calc_ig__iginrec_slot: &mut f64,
        var_fn440_calc_ig__iginrec_dn4_slot: &mut f64,
        var_fn440_calc_ig__iginrec_dn8_slot: &mut f64,
        var_fn440_calc_ig__iginrec_dn9_slot: &mut f64,
        var_fn440_calc_ig__igout_slot: &mut f64,
        var_fn440_calc_ig__igout_dn4_slot: &mut f64,
        var_fn440_calc_ig__igout_dn8_slot: &mut f64,
        var_fn440_calc_ig__igout_dn9_slot: &mut f64,
        var_fn440_calc_ig__ijin_slot: &mut f64,
        var_fn440_calc_ig__irecin_slot: &mut f64,
        var_fn440_calc_ig__isdiodeout_slot: &mut f64,
        var_fn440_calc_ig__isdiodeout_dn4_slot: &mut f64,
        var_fn440_calc_ig__isrecout_slot: &mut f64,
        var_fn440_calc_ig__isrecout_dn4_slot: &mut f64,
        var_fn440_calc_ig__kbdgatein_slot: &mut f64,
        var_fn440_calc_ig__ngf_slot: &mut f64,
        var_fn440_calc_ig__pbdgin_slot: &mut f64,
        var_fn440_calc_ig__pg_param1_slot: &mut f64,
        var_fn440_calc_ig__pg_paramin_slot: &mut f64,
        var_fn440_calc_ig__pgsrecin_slot: &mut f64,
        var_fn440_calc_ig__phitin_slot: &mut f64,
        var_fn440_calc_ig__phitin_dn4_slot: &mut f64,
        var_fn440_calc_ig__return_slot: &mut f64,
        var_fn440_calc_ig__return_dn4_slot: &mut f64,
        var_fn440_calc_ig__return_dn8_slot: &mut f64,
        var_fn440_calc_ig__return_dn9_slot: &mut f64,
        var_fn440_calc_ig__t0_slot: &mut f64,
        var_fn440_calc_ig__t0_dn4_slot: &mut f64,
        var_fn440_calc_ig__tfacdiodein_slot: &mut f64,
        var_fn440_calc_ig__tfacdiodein_dn4_slot: &mut f64,
        var_fn440_calc_ig__type_slot: &mut f64,
        var_fn440_calc_ig__vbdgin_slot: &mut f64,
        var_fn440_calc_ig__vgin_slot: &mut f64,
        var_fn440_calc_ig__vgin_dn8_slot: &mut f64,
        var_fn440_calc_ig__vgin_dn9_slot: &mut f64,
        var_fn440_calc_ig__vgsatin_slot: &mut f64,
        var_fn440_calc_ig__vgsatqin_slot: &mut f64,
        var_fn440_calc_ig__vjg_slot: &mut f64,
        var_fn440_calc_ig__w_slot: &mut f64,
        var_guard439_slot: &mut f64,
        var_igdi2_slot: &mut f64,
        var_igdi2_dn17_slot: &mut f64,
        var_igdi2_dn4_slot: &mut f64,
        var_igdi2_dn8_slot: &mut f64,
    ) {
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let mut var_fn434_calc_ig__expirev: f64 = *var_fn434_calc_ig__expirev_slot;
        let mut var_fn434_calc_ig__expirev_dn17: f64 = *var_fn434_calc_ig__expirev_dn17_slot;
        let mut var_fn434_calc_ig__expirev_dn4: f64 = *var_fn434_calc_ig__expirev_dn4_slot;
        let mut var_fn434_calc_ig__expirev_dn8: f64 = *var_fn434_calc_ig__expirev_dn8_slot;
        let mut var_fn434_calc_ig__iginrec: f64 = *var_fn434_calc_ig__iginrec_slot;
        let mut var_fn434_calc_ig__iginrec_dn17: f64 = *var_fn434_calc_ig__iginrec_dn17_slot;
        let mut var_fn434_calc_ig__iginrec_dn4: f64 = *var_fn434_calc_ig__iginrec_dn4_slot;
        let mut var_fn434_calc_ig__iginrec_dn8: f64 = *var_fn434_calc_ig__iginrec_dn8_slot;
        let mut var_fn434_calc_ig__igout: f64 = *var_fn434_calc_ig__igout_slot;
        let mut var_fn434_calc_ig__igout_dn17: f64 = *var_fn434_calc_ig__igout_dn17_slot;
        let mut var_fn434_calc_ig__igout_dn4: f64 = *var_fn434_calc_ig__igout_dn4_slot;
        let mut var_fn434_calc_ig__igout_dn8: f64 = *var_fn434_calc_ig__igout_dn8_slot;
        let mut var_fn434_calc_ig__return: f64 = *var_fn434_calc_ig__return_slot;
        let mut var_fn434_calc_ig__return_dn17: f64 = *var_fn434_calc_ig__return_dn17_slot;
        let mut var_fn434_calc_ig__return_dn4: f64 = *var_fn434_calc_ig__return_dn4_slot;
        let mut var_fn434_calc_ig__return_dn8: f64 = *var_fn434_calc_ig__return_dn8_slot;
        let mut var_fn440_calc_ig__alpha2_phit: f64 = *var_fn440_calc_ig__alpha2_phit_slot;
        let mut var_fn440_calc_ig__alpha2_phit_dn4: f64 = *var_fn440_calc_ig__alpha2_phit_dn4_slot;
        let mut var_fn440_calc_ig__alphagin: f64 = *var_fn440_calc_ig__alphagin_slot;
        let mut var_fn440_calc_ig__betarecin: f64 = *var_fn440_calc_ig__betarecin_slot;
        let mut var_fn440_calc_ig__expbd1: f64 = *var_fn440_calc_ig__expbd1_slot;
        let mut var_fn440_calc_ig__expbd1_dn4: f64 = *var_fn440_calc_ig__expbd1_dn4_slot;
        let mut var_fn440_calc_ig__expbd1_dn8: f64 = *var_fn440_calc_ig__expbd1_dn8_slot;
        let mut var_fn440_calc_ig__expbd1_dn9: f64 = *var_fn440_calc_ig__expbd1_dn9_slot;
        let mut var_fn440_calc_ig__expbd2: f64 = *var_fn440_calc_ig__expbd2_slot;
        let mut var_fn440_calc_ig__expbd2_dn4: f64 = *var_fn440_calc_ig__expbd2_dn4_slot;
        let mut var_fn440_calc_ig__expbdarg1: f64 = *var_fn440_calc_ig__expbdarg1_slot;
        let mut var_fn440_calc_ig__expbdarg1_dn4: f64 = *var_fn440_calc_ig__expbdarg1_dn4_slot;
        let mut var_fn440_calc_ig__expbdarg1_dn8: f64 = *var_fn440_calc_ig__expbdarg1_dn8_slot;
        let mut var_fn440_calc_ig__expbdarg1_dn9: f64 = *var_fn440_calc_ig__expbdarg1_dn9_slot;
        let mut var_fn440_calc_ig__expbdarg2: f64 = *var_fn440_calc_ig__expbdarg2_slot;
        let mut var_fn440_calc_ig__expbdarg2_dn4: f64 = *var_fn440_calc_ig__expbdarg2_dn4_slot;
        let mut var_fn440_calc_ig__expffvarg: f64 = *var_fn440_calc_ig__expffvarg_slot;
        let mut var_fn440_calc_ig__expffvarg_dn4: f64 = *var_fn440_calc_ig__expffvarg_dn4_slot;
        let mut var_fn440_calc_ig__expffvarg_dn8: f64 = *var_fn440_calc_ig__expffvarg_dn8_slot;
        let mut var_fn440_calc_ig__expffvarg_dn9: f64 = *var_fn440_calc_ig__expffvarg_dn9_slot;
        let mut var_fn440_calc_ig__expifor: f64 = *var_fn440_calc_ig__expifor_slot;
        let mut var_fn440_calc_ig__expifor_dn4: f64 = *var_fn440_calc_ig__expifor_dn4_slot;
        let mut var_fn440_calc_ig__expifor_dn8: f64 = *var_fn440_calc_ig__expifor_dn8_slot;
        let mut var_fn440_calc_ig__expifor_dn9: f64 = *var_fn440_calc_ig__expifor_dn9_slot;
        let mut var_fn440_calc_ig__expiforarg: f64 = *var_fn440_calc_ig__expiforarg_slot;
        let mut var_fn440_calc_ig__expiforarg_dn4: f64 = *var_fn440_calc_ig__expiforarg_dn4_slot;
        let mut var_fn440_calc_ig__expiforarg_dn8: f64 = *var_fn440_calc_ig__expiforarg_dn8_slot;
        let mut var_fn440_calc_ig__expiforarg_dn9: f64 = *var_fn440_calc_ig__expiforarg_dn9_slot;
        let mut var_fn440_calc_ig__expirevarg: f64 = *var_fn440_calc_ig__expirevarg_slot;
        let mut var_fn440_calc_ig__expirevarg_dn4: f64 = *var_fn440_calc_ig__expirevarg_dn4_slot;
        let mut var_fn440_calc_ig__expirevarg_dn8: f64 = *var_fn440_calc_ig__expirevarg_dn8_slot;
        let mut var_fn440_calc_ig__expirevarg_dn9: f64 = *var_fn440_calc_ig__expirevarg_dn9_slot;
        let mut var_fn440_calc_ig__expphib: f64 = *var_fn440_calc_ig__expphib_slot;
        let mut var_fn440_calc_ig__expphib_dn4: f64 = *var_fn440_calc_ig__expphib_dn4_slot;
        let mut var_fn440_calc_ig__ffvgin: f64 = *var_fn440_calc_ig__ffvgin_slot;
        let mut var_fn440_calc_ig__ffvgin_dn4: f64 = *var_fn440_calc_ig__ffvgin_dn4_slot;
        let mut var_fn440_calc_ig__ffvgin_dn8: f64 = *var_fn440_calc_ig__ffvgin_dn8_slot;
        let mut var_fn440_calc_ig__ffvgin_dn9: f64 = *var_fn440_calc_ig__ffvgin_dn9_slot;
        let mut var_fn440_calc_ig__fracin: f64 = *var_fn440_calc_ig__fracin_slot;
        let mut var_fn440_calc_ig__frecgin: f64 = *var_fn440_calc_ig__frecgin_slot;
        let mut var_fn440_calc_ig__frecgin_dn8: f64 = *var_fn440_calc_ig__frecgin_dn8_slot;
        let mut var_fn440_calc_ig__frecgin_dn9: f64 = *var_fn440_calc_ig__frecgin_dn9_slot;
        let mut var_fn440_calc_ig__iginbd: f64 = *var_fn440_calc_ig__iginbd_slot;
        let mut var_fn440_calc_ig__iginbd_dn4: f64 = *var_fn440_calc_ig__iginbd_dn4_slot;
        let mut var_fn440_calc_ig__iginbd_dn8: f64 = *var_fn440_calc_ig__iginbd_dn8_slot;
        let mut var_fn440_calc_ig__iginbd_dn9: f64 = *var_fn440_calc_ig__iginbd_dn9_slot;
        let mut var_fn440_calc_ig__igindiode: f64 = *var_fn440_calc_ig__igindiode_slot;
        let mut var_fn440_calc_ig__igindiode_dn4: f64 = *var_fn440_calc_ig__igindiode_dn4_slot;
        let mut var_fn440_calc_ig__igindiode_dn8: f64 = *var_fn440_calc_ig__igindiode_dn8_slot;
        let mut var_fn440_calc_ig__igindiode_dn9: f64 = *var_fn440_calc_ig__igindiode_dn9_slot;
        let mut var_fn440_calc_ig__iginrec: f64 = *var_fn440_calc_ig__iginrec_slot;
        let mut var_fn440_calc_ig__iginrec_dn4: f64 = *var_fn440_calc_ig__iginrec_dn4_slot;
        let mut var_fn440_calc_ig__iginrec_dn8: f64 = *var_fn440_calc_ig__iginrec_dn8_slot;
        let mut var_fn440_calc_ig__iginrec_dn9: f64 = *var_fn440_calc_ig__iginrec_dn9_slot;
        let mut var_fn440_calc_ig__igout: f64 = *var_fn440_calc_ig__igout_slot;
        let mut var_fn440_calc_ig__igout_dn4: f64 = *var_fn440_calc_ig__igout_dn4_slot;
        let mut var_fn440_calc_ig__igout_dn8: f64 = *var_fn440_calc_ig__igout_dn8_slot;
        let mut var_fn440_calc_ig__igout_dn9: f64 = *var_fn440_calc_ig__igout_dn9_slot;
        let mut var_fn440_calc_ig__ijin: f64 = *var_fn440_calc_ig__ijin_slot;
        let mut var_fn440_calc_ig__irecin: f64 = *var_fn440_calc_ig__irecin_slot;
        let mut var_fn440_calc_ig__isdiodeout: f64 = *var_fn440_calc_ig__isdiodeout_slot;
        let mut var_fn440_calc_ig__isdiodeout_dn4: f64 = *var_fn440_calc_ig__isdiodeout_dn4_slot;
        let mut var_fn440_calc_ig__isrecout: f64 = *var_fn440_calc_ig__isrecout_slot;
        let mut var_fn440_calc_ig__isrecout_dn4: f64 = *var_fn440_calc_ig__isrecout_dn4_slot;
        let mut var_fn440_calc_ig__kbdgatein: f64 = *var_fn440_calc_ig__kbdgatein_slot;
        let mut var_fn440_calc_ig__ngf: f64 = *var_fn440_calc_ig__ngf_slot;
        let mut var_fn440_calc_ig__pbdgin: f64 = *var_fn440_calc_ig__pbdgin_slot;
        let mut var_fn440_calc_ig__pg_param1: f64 = *var_fn440_calc_ig__pg_param1_slot;
        let mut var_fn440_calc_ig__pg_paramin: f64 = *var_fn440_calc_ig__pg_paramin_slot;
        let mut var_fn440_calc_ig__pgsrecin: f64 = *var_fn440_calc_ig__pgsrecin_slot;
        let mut var_fn440_calc_ig__phitin: f64 = *var_fn440_calc_ig__phitin_slot;
        let mut var_fn440_calc_ig__phitin_dn4: f64 = *var_fn440_calc_ig__phitin_dn4_slot;
        let mut var_fn440_calc_ig__return: f64 = *var_fn440_calc_ig__return_slot;
        let mut var_fn440_calc_ig__return_dn4: f64 = *var_fn440_calc_ig__return_dn4_slot;
        let mut var_fn440_calc_ig__return_dn8: f64 = *var_fn440_calc_ig__return_dn8_slot;
        let mut var_fn440_calc_ig__return_dn9: f64 = *var_fn440_calc_ig__return_dn9_slot;
        let mut var_fn440_calc_ig__t0: f64 = *var_fn440_calc_ig__t0_slot;
        let mut var_fn440_calc_ig__t0_dn4: f64 = *var_fn440_calc_ig__t0_dn4_slot;
        let mut var_fn440_calc_ig__tfacdiodein: f64 = *var_fn440_calc_ig__tfacdiodein_slot;
        let mut var_fn440_calc_ig__tfacdiodein_dn4: f64 = *var_fn440_calc_ig__tfacdiodein_dn4_slot;
        let mut var_fn440_calc_ig__type: f64 = *var_fn440_calc_ig__type_slot;
        let mut var_fn440_calc_ig__vbdgin: f64 = *var_fn440_calc_ig__vbdgin_slot;
        let mut var_fn440_calc_ig__vgin: f64 = *var_fn440_calc_ig__vgin_slot;
        let mut var_fn440_calc_ig__vgin_dn8: f64 = *var_fn440_calc_ig__vgin_dn8_slot;
        let mut var_fn440_calc_ig__vgin_dn9: f64 = *var_fn440_calc_ig__vgin_dn9_slot;
        let mut var_fn440_calc_ig__vgsatin: f64 = *var_fn440_calc_ig__vgsatin_slot;
        let mut var_fn440_calc_ig__vgsatqin: f64 = *var_fn440_calc_ig__vgsatqin_slot;
        let mut var_fn440_calc_ig__vjg: f64 = *var_fn440_calc_ig__vjg_slot;
        let mut var_fn440_calc_ig__w: f64 = *var_fn440_calc_ig__w_slot;
        let mut var_guard439: f64 = *var_guard439_slot;
        let mut var_igdi2: f64 = *var_igdi2_slot;
        let mut var_igdi2_dn17: f64 = *var_igdi2_dn17_slot;
        let mut var_igdi2_dn4: f64 = *var_igdi2_dn4_slot;
        let mut var_igdi2_dn8: f64 = *var_igdi2_dn8_slot;

        let (assign37120_e34451, assign37120_e34451_d_n4, assign37120_e34451_d_n8, assign37120_e34451_d_n17,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        let assign37120_e34417: f64 = (-50.0);
        let (assign37120_e34449, assign37120_e34449_d_n4, assign37120_e34449_d_n8, assign37120_e34449_d_n17,) = {
            if ((!(var_fn434_calc_ig__expirevarg > 50.0)) && (!(var_fn434_calc_ig__expirevarg < assign37120_e34417))) {
                let assign37120_e34422: f64 = (var_fn434_calc_ig__expirevarg).exp();
                (assign37120_e34422, (assign37120_e34422 * var_fn434_calc_ig__expirevarg_dn4), (assign37120_e34422 * var_fn434_calc_ig__expirevarg_dn8), (assign37120_e34422 * var_fn434_calc_ig__expirevarg_dn17),)
            } else {
                let assign37120_e34429: f64 = (-50.0);
                let (assign37120_e34448, assign37120_e34448_d_n4, assign37120_e34448_d_n8, assign37120_e34448_d_n17,) = {
                    if ((!(var_fn434_calc_ig__expirevarg > 50.0)) && (var_fn434_calc_ig__expirevarg < assign37120_e34429)) {
                        let assign37120_e34433: f64 = (-50.0);
                        let assign37120_e34434: f64 = (assign37120_e34433).exp();
                        (assign37120_e34434, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign37120_e34447, assign37120_e34447_d_n4, assign37120_e34447_d_n8, assign37120_e34447_d_n17,) = {
                            if (var_fn434_calc_ig__expirevarg > 50.0) {
                                let assign37120_e34439: f64 = (50.0_f64).exp();
                                let assign37120_e34443: f64 = (var_fn434_calc_ig__expirevarg - 50.0);
                                let assign37120_e34444: f64 = (1.0 + assign37120_e34443);
                                let assign37120_e34445: f64 = (assign37120_e34439 * assign37120_e34444);
                                (assign37120_e34445, (assign37120_e34439 * var_fn434_calc_ig__expirevarg_dn4), (assign37120_e34439 * var_fn434_calc_ig__expirevarg_dn8), (assign37120_e34439 * var_fn434_calc_ig__expirevarg_dn17),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign37120_e34447, assign37120_e34447_d_n4, assign37120_e34447_d_n8, assign37120_e34447_d_n17,)
                    }
                };
                (assign37120_e34448, assign37120_e34448_d_n4, assign37120_e34448_d_n8, assign37120_e34448_d_n17,)
            }
        };
        (assign37120_e34449, assign37120_e34449_d_n4, assign37120_e34449_d_n8, assign37120_e34449_d_n17,)
    } else {
        (var_fn434_calc_ig__expirev, var_fn434_calc_ig__expirev_dn4, var_fn434_calc_ig__expirev_dn8, var_fn434_calc_ig__expirev_dn17,)
    }
};
        var_fn434_calc_ig__expirev = assign37120_e34451;
        var_fn434_calc_ig__expirev_dn4 = assign37120_e34451_d_n4;
        var_fn434_calc_ig__expirev_dn8 = assign37120_e34451_d_n8;
        var_fn434_calc_ig__expirev_dn17 = assign37120_e34451_d_n17;

        let (assign37130_e34461, assign37130_e34461_d_n4, assign37130_e34461_d_n8, assign37130_e34461_d_n17,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        let assign37130_e34458: f64 = (var_fn434_calc_ig__expirev - 1.0);
        let assign37130_e34459: f64 = (var_fn434_calc_ig__isrecout * assign37130_e34458);
        (assign37130_e34459, ((var_fn434_calc_ig__isrecout_dn4 * assign37130_e34458) + (var_fn434_calc_ig__isrecout * var_fn434_calc_ig__expirev_dn4)), (var_fn434_calc_ig__isrecout * var_fn434_calc_ig__expirev_dn8), (var_fn434_calc_ig__isrecout * var_fn434_calc_ig__expirev_dn17),)
    } else {
        (var_fn434_calc_ig__iginrec, var_fn434_calc_ig__iginrec_dn4, var_fn434_calc_ig__iginrec_dn8, var_fn434_calc_ig__iginrec_dn17,)
    }
};
        var_fn434_calc_ig__iginrec = assign37130_e34461;
        var_fn434_calc_ig__iginrec_dn4 = assign37130_e34461_d_n4;
        var_fn434_calc_ig__iginrec_dn8 = assign37130_e34461_d_n8;
        var_fn434_calc_ig__iginrec_dn17 = assign37130_e34461_d_n17;

        let (assign37140_e34469, assign37140_e34469_d_n4, assign37140_e34469_d_n8, assign37140_e34469_d_n17,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        let assign37140_e34467: f64 = (var_fn434_calc_ig__igindiode + var_fn434_calc_ig__iginrec);
        (assign37140_e34467, (var_fn434_calc_ig__igindiode_dn4 + var_fn434_calc_ig__iginrec_dn4), (var_fn434_calc_ig__igindiode_dn8 + var_fn434_calc_ig__iginrec_dn8), (var_fn434_calc_ig__igindiode_dn17 + var_fn434_calc_ig__iginrec_dn17),)
    } else {
        (var_fn434_calc_ig__igout, var_fn434_calc_ig__igout_dn4, var_fn434_calc_ig__igout_dn8, var_fn434_calc_ig__igout_dn17,)
    }
};
        var_fn434_calc_ig__igout = assign37140_e34469;
        var_fn434_calc_ig__igout_dn4 = assign37140_e34469_d_n4;
        var_fn434_calc_ig__igout_dn8 = assign37140_e34469_d_n8;
        var_fn434_calc_ig__igout_dn17 = assign37140_e34469_d_n17;

        let (assign37150_e34475, assign37150_e34475_d_n4, assign37150_e34475_d_n8, assign37150_e34475_d_n17,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (var_fn434_calc_ig__igout, var_fn434_calc_ig__igout_dn4, var_fn434_calc_ig__igout_dn8, var_fn434_calc_ig__igout_dn17,)
    } else {
        (var_fn434_calc_ig__return, var_fn434_calc_ig__return_dn4, var_fn434_calc_ig__return_dn8, var_fn434_calc_ig__return_dn17,)
    }
};
        var_fn434_calc_ig__return = assign37150_e34475;
        var_fn434_calc_ig__return_dn4 = assign37150_e34475_d_n4;
        var_fn434_calc_ig__return_dn8 = assign37150_e34475_d_n8;
        var_fn434_calc_ig__return_dn17 = assign37150_e34475_d_n17;

        let (assign37180_e34493, assign37180_e34493_d_n4, assign37180_e34493_d_n8, assign37180_e34493_d_n17,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        (var_fn434_calc_ig__return, var_fn434_calc_ig__return_dn4, var_fn434_calc_ig__return_dn8, var_fn434_calc_ig__return_dn17,)
    } else {
        (var_igdi2, var_igdi2_dn4, var_igdi2_dn8, var_igdi2_dn17,)
    }
};
        var_igdi2 = assign37180_e34493;
        var_igdi2_dn4 = assign37180_e34493_d_n4;
        var_igdi2_dn8 = assign37180_e34493_d_n8;
        var_igdi2_dn17 = assign37180_e34493_d_n17;

        let assign37190_e34496: f64 = if p.p255 != 0.0 { 1.0 } else { 0.0 };
        var_guard439 = assign37190_e34496;

        let (assign37200_e34502, assign37200_e34502_d_n4, assign37200_e34502_d_n8, assign37200_e34502_d_n9,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn440_calc_ig__return, var_fn440_calc_ig__return_dn4, var_fn440_calc_ig__return_dn8, var_fn440_calc_ig__return_dn9,)
    }
};
        var_fn440_calc_ig__return = assign37200_e34502;
        var_fn440_calc_ig__return_dn4 = assign37200_e34502_d_n4;
        var_fn440_calc_ig__return_dn8 = assign37200_e34502_d_n8;
        var_fn440_calc_ig__return_dn9 = assign37200_e34502_d_n9;

        let (assign37210_e34508, assign37210_e34508_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn440_calc_ig__isdiodeout, var_fn440_calc_ig__isdiodeout_dn4,)
    }
};
        var_fn440_calc_ig__isdiodeout = assign37210_e34508;
        var_fn440_calc_ig__isdiodeout_dn4 = assign37210_e34508_d_n4;

        let (assign37220_e34514, assign37220_e34514_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn440_calc_ig__isrecout, var_fn440_calc_ig__isrecout_dn4,)
    }
};
        var_fn440_calc_ig__isrecout = assign37220_e34514;
        var_fn440_calc_ig__isrecout_dn4 = assign37220_e34514_d_n4;

        let (assign37230_e34522, assign37230_e34522_d_n8, assign37230_e34522_d_n9,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        let assign37230_e34520: f64 = (p.p6 * (nv8 - nv9));
        (assign37230_e34520, p.p6, (-p.p6),)
    } else {
        (var_fn440_calc_ig__vgin, var_fn440_calc_ig__vgin_dn8, var_fn440_calc_ig__vgin_dn9,)
    }
};
        var_fn440_calc_ig__vgin = assign37230_e34522;
        var_fn440_calc_ig__vgin_dn8 = assign37230_e34522_d_n8;
        var_fn440_calc_ig__vgin_dn9 = assign37230_e34522_d_n9;

        let (assign37240_e34528, assign37240_e34528_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (var_phit, var_phit_dn4,)
    } else {
        (var_fn440_calc_ig__phitin, var_fn440_calc_ig__phitin_dn4,)
    }
};
        var_fn440_calc_ig__phitin = assign37240_e34528;
        var_fn440_calc_ig__phitin_dn4 = assign37240_e34528_d_n4;

        let (assign37250_e34534,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (p.p260,)
    } else {
        (var_fn440_calc_ig__vgsatin,)
    }
};
        var_fn440_calc_ig__vgsatin = assign37250_e34534;

        let (assign37260_e34540,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (p.p262,)
    } else {
        (var_fn440_calc_ig__alphagin,)
    }
};
        var_fn440_calc_ig__alphagin = assign37260_e34540;

        let (assign37270_e34546,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (p.p261,)
    } else {
        (var_fn440_calc_ig__fracin,)
    }
};
        var_fn440_calc_ig__fracin = assign37270_e34546;

        let (assign37280_e34552,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (p.p258,)
    } else {
        (var_fn440_calc_ig__pg_paramin,)
    }
};
        var_fn440_calc_ig__pg_paramin = assign37280_e34552;

        let (assign37290_e34558,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (p.p278,)
    } else {
        (var_fn440_calc_ig__pbdgin,)
    }
};
        var_fn440_calc_ig__pbdgin = assign37290_e34558;

        let (assign37300_e34564,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (p.p277,)
    } else {
        (var_fn440_calc_ig__vbdgin,)
    }
};
        var_fn440_calc_ig__vbdgin = assign37300_e34564;

        let (assign37310_e34570, assign37310_e34570_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (var_tfacdiode, var_tfacdiode_dn4,)
    } else {
        (var_fn440_calc_ig__tfacdiodein, var_fn440_calc_ig__tfacdiodein_dn4,)
    }
};
        var_fn440_calc_ig__tfacdiodein = assign37310_e34570;
        var_fn440_calc_ig__tfacdiodein_dn4 = assign37310_e34570_d_n4;

        let (assign37320_e34576,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (p.p0,)
    } else {
        (var_fn440_calc_ig__w,)
    }
};
        var_fn440_calc_ig__w = assign37320_e34576;

        let (assign37330_e34582,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (p.p2,)
    } else {
        (var_fn440_calc_ig__ngf,)
    }
};
        var_fn440_calc_ig__ngf = assign37330_e34582;

        let (assign37340_e34590,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        let assign37340_e34588: f64 = (p.p255 * p.p259);
        (assign37340_e34588,)
    } else {
        (var_fn440_calc_ig__ijin,)
    }
};
        var_fn440_calc_ig__ijin = assign37340_e34590;

        let (assign37350_e34596,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (p.p276,)
    } else {
        (var_fn440_calc_ig__kbdgatein,)
    }
};
        var_fn440_calc_ig__kbdgatein = assign37350_e34596;

        let (assign37360_e34602,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (p.p270,)
    } else {
        (var_fn440_calc_ig__vgsatqin,)
    }
};
        var_fn440_calc_ig__vgsatqin = assign37360_e34602;

        let (assign37370_e34608,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (p.p271,)
    } else {
        (var_fn440_calc_ig__betarecin,)
    }
};
        var_fn440_calc_ig__betarecin = assign37370_e34608;

        let (assign37380_e34616,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        let assign37380_e34614: f64 = (p.p255 * p.p269);
        (assign37380_e34614,)
    } else {
        (var_fn440_calc_ig__irecin,)
    }
};
        var_fn440_calc_ig__irecin = assign37380_e34616;

        let (assign37390_e34622,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (p.p268,)
    } else {
        (var_fn440_calc_ig__pgsrecin,)
    }
};
        var_fn440_calc_ig__pgsrecin = assign37390_e34622;

        let (assign37400_e34628,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (p.p257,)
    } else {
        (var_fn440_calc_ig__pg_param1,)
    }
};
        var_fn440_calc_ig__pg_param1 = assign37400_e34628;

        let (assign37410_e34634,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (p.p256,)
    } else {
        (var_fn440_calc_ig__vjg,)
    }
};
        var_fn440_calc_ig__vjg = assign37410_e34634;

        let (assign37420_e34640,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (p.p6,)
    } else {
        (var_fn440_calc_ig__type,)
    }
};
        var_fn440_calc_ig__type = assign37420_e34640;

        let (assign37430_e34646, assign37430_e34646_d_n4, assign37430_e34646_d_n8, assign37430_e34646_d_n9,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn440_calc_ig__igout, var_fn440_calc_ig__igout_dn4, var_fn440_calc_ig__igout_dn8, var_fn440_calc_ig__igout_dn9,)
    }
};
        var_fn440_calc_ig__igout = assign37430_e34646;
        var_fn440_calc_ig__igout_dn4 = assign37430_e34646_d_n4;
        var_fn440_calc_ig__igout_dn8 = assign37430_e34646_d_n8;
        var_fn440_calc_ig__igout_dn9 = assign37430_e34646_d_n9;

        let (assign37440_e34652, assign37440_e34652_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn440_calc_ig__alpha2_phit, var_fn440_calc_ig__alpha2_phit_dn4,)
    }
};
        var_fn440_calc_ig__alpha2_phit = assign37440_e34652;
        var_fn440_calc_ig__alpha2_phit_dn4 = assign37440_e34652_d_n4;

        let (assign37450_e34658, assign37450_e34658_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn440_calc_ig__t0, var_fn440_calc_ig__t0_dn4,)
    }
};
        var_fn440_calc_ig__t0 = assign37450_e34658;
        var_fn440_calc_ig__t0_dn4 = assign37450_e34658_d_n4;

        let (assign37460_e34664, assign37460_e34664_d_n4, assign37460_e34664_d_n8, assign37460_e34664_d_n9,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn440_calc_ig__ffvgin, var_fn440_calc_ig__ffvgin_dn4, var_fn440_calc_ig__ffvgin_dn8, var_fn440_calc_ig__ffvgin_dn9,)
    }
};
        var_fn440_calc_ig__ffvgin = assign37460_e34664;
        var_fn440_calc_ig__ffvgin_dn4 = assign37460_e34664_d_n4;
        var_fn440_calc_ig__ffvgin_dn8 = assign37460_e34664_d_n8;
        var_fn440_calc_ig__ffvgin_dn9 = assign37460_e34664_d_n9;

        let (assign37470_e34670, assign37470_e34670_d_n4, assign37470_e34670_d_n8, assign37470_e34670_d_n9,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn440_calc_ig__iginbd, var_fn440_calc_ig__iginbd_dn4, var_fn440_calc_ig__iginbd_dn8, var_fn440_calc_ig__iginbd_dn9,)
    }
};
        var_fn440_calc_ig__iginbd = assign37470_e34670;
        var_fn440_calc_ig__iginbd_dn4 = assign37470_e34670_d_n4;
        var_fn440_calc_ig__iginbd_dn8 = assign37470_e34670_d_n8;
        var_fn440_calc_ig__iginbd_dn9 = assign37470_e34670_d_n9;

        let (assign37480_e34676, assign37480_e34676_d_n4, assign37480_e34676_d_n8, assign37480_e34676_d_n9,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn440_calc_ig__igindiode, var_fn440_calc_ig__igindiode_dn4, var_fn440_calc_ig__igindiode_dn8, var_fn440_calc_ig__igindiode_dn9,)
    }
};
        var_fn440_calc_ig__igindiode = assign37480_e34676;
        var_fn440_calc_ig__igindiode_dn4 = assign37480_e34676_d_n4;
        var_fn440_calc_ig__igindiode_dn8 = assign37480_e34676_d_n8;
        var_fn440_calc_ig__igindiode_dn9 = assign37480_e34676_d_n9;

        let (assign37490_e34682, assign37490_e34682_d_n8, assign37490_e34682_d_n9,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_fn440_calc_ig__frecgin, var_fn440_calc_ig__frecgin_dn8, var_fn440_calc_ig__frecgin_dn9,)
    }
};
        var_fn440_calc_ig__frecgin = assign37490_e34682;
        var_fn440_calc_ig__frecgin_dn8 = assign37490_e34682_d_n8;
        var_fn440_calc_ig__frecgin_dn9 = assign37490_e34682_d_n9;

        let (assign37500_e34688, assign37500_e34688_d_n4, assign37500_e34688_d_n8, assign37500_e34688_d_n9,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn440_calc_ig__iginrec, var_fn440_calc_ig__iginrec_dn4, var_fn440_calc_ig__iginrec_dn8, var_fn440_calc_ig__iginrec_dn9,)
    }
};
        var_fn440_calc_ig__iginrec = assign37500_e34688;
        var_fn440_calc_ig__iginrec_dn4 = assign37500_e34688_d_n4;
        var_fn440_calc_ig__iginrec_dn8 = assign37500_e34688_d_n8;
        var_fn440_calc_ig__iginrec_dn9 = assign37500_e34688_d_n9;

        let (assign37510_e34694, assign37510_e34694_d_n4, assign37510_e34694_d_n8, assign37510_e34694_d_n9,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn440_calc_ig__expbdarg1, var_fn440_calc_ig__expbdarg1_dn4, var_fn440_calc_ig__expbdarg1_dn8, var_fn440_calc_ig__expbdarg1_dn9,)
    }
};
        var_fn440_calc_ig__expbdarg1 = assign37510_e34694;
        var_fn440_calc_ig__expbdarg1_dn4 = assign37510_e34694_d_n4;
        var_fn440_calc_ig__expbdarg1_dn8 = assign37510_e34694_d_n8;
        var_fn440_calc_ig__expbdarg1_dn9 = assign37510_e34694_d_n9;

        let (assign37520_e34700, assign37520_e34700_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn440_calc_ig__expbdarg2, var_fn440_calc_ig__expbdarg2_dn4,)
    }
};
        var_fn440_calc_ig__expbdarg2 = assign37520_e34700;
        var_fn440_calc_ig__expbdarg2_dn4 = assign37520_e34700_d_n4;

        let (assign37530_e34706, assign37530_e34706_d_n4, assign37530_e34706_d_n8, assign37530_e34706_d_n9,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn440_calc_ig__expbd1, var_fn440_calc_ig__expbd1_dn4, var_fn440_calc_ig__expbd1_dn8, var_fn440_calc_ig__expbd1_dn9,)
    }
};
        var_fn440_calc_ig__expbd1 = assign37530_e34706;
        var_fn440_calc_ig__expbd1_dn4 = assign37530_e34706_d_n4;
        var_fn440_calc_ig__expbd1_dn8 = assign37530_e34706_d_n8;
        var_fn440_calc_ig__expbd1_dn9 = assign37530_e34706_d_n9;

        let (assign37540_e34712, assign37540_e34712_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn440_calc_ig__expbd2, var_fn440_calc_ig__expbd2_dn4,)
    }
};
        var_fn440_calc_ig__expbd2 = assign37540_e34712;
        var_fn440_calc_ig__expbd2_dn4 = assign37540_e34712_d_n4;

        let (assign37550_e34718, assign37550_e34718_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn440_calc_ig__expphib, var_fn440_calc_ig__expphib_dn4,)
    }
};
        var_fn440_calc_ig__expphib = assign37550_e34718;
        var_fn440_calc_ig__expphib_dn4 = assign37550_e34718_d_n4;

        let (assign37560_e34724, assign37560_e34724_d_n4, assign37560_e34724_d_n8, assign37560_e34724_d_n9,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn440_calc_ig__expffvarg, var_fn440_calc_ig__expffvarg_dn4, var_fn440_calc_ig__expffvarg_dn8, var_fn440_calc_ig__expffvarg_dn9,)
    }
};
        var_fn440_calc_ig__expffvarg = assign37560_e34724;
        var_fn440_calc_ig__expffvarg_dn4 = assign37560_e34724_d_n4;
        var_fn440_calc_ig__expffvarg_dn8 = assign37560_e34724_d_n8;
        var_fn440_calc_ig__expffvarg_dn9 = assign37560_e34724_d_n9;

        let (assign37570_e34730, assign37570_e34730_d_n4, assign37570_e34730_d_n8, assign37570_e34730_d_n9,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn440_calc_ig__expiforarg, var_fn440_calc_ig__expiforarg_dn4, var_fn440_calc_ig__expiforarg_dn8, var_fn440_calc_ig__expiforarg_dn9,)
    }
};
        var_fn440_calc_ig__expiforarg = assign37570_e34730;
        var_fn440_calc_ig__expiforarg_dn4 = assign37570_e34730_d_n4;
        var_fn440_calc_ig__expiforarg_dn8 = assign37570_e34730_d_n8;
        var_fn440_calc_ig__expiforarg_dn9 = assign37570_e34730_d_n9;

        let (assign37580_e34736, assign37580_e34736_d_n4, assign37580_e34736_d_n8, assign37580_e34736_d_n9,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn440_calc_ig__expifor, var_fn440_calc_ig__expifor_dn4, var_fn440_calc_ig__expifor_dn8, var_fn440_calc_ig__expifor_dn9,)
    }
};
        var_fn440_calc_ig__expifor = assign37580_e34736;
        var_fn440_calc_ig__expifor_dn4 = assign37580_e34736_d_n4;
        var_fn440_calc_ig__expifor_dn8 = assign37580_e34736_d_n8;
        var_fn440_calc_ig__expifor_dn9 = assign37580_e34736_d_n9;

        let (assign37590_e34742, assign37590_e34742_d_n4, assign37590_e34742_d_n8, assign37590_e34742_d_n9,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn440_calc_ig__expirevarg, var_fn440_calc_ig__expirevarg_dn4, var_fn440_calc_ig__expirevarg_dn8, var_fn440_calc_ig__expirevarg_dn9,)
    }
};
        var_fn440_calc_ig__expirevarg = assign37590_e34742;
        var_fn440_calc_ig__expirevarg_dn4 = assign37590_e34742_d_n4;
        var_fn440_calc_ig__expirevarg_dn8 = assign37590_e34742_d_n8;
        var_fn440_calc_ig__expirevarg_dn9 = assign37590_e34742_d_n9;

        *var_fn434_calc_ig__expirev_slot = var_fn434_calc_ig__expirev;
        *var_fn434_calc_ig__expirev_dn17_slot = var_fn434_calc_ig__expirev_dn17;
        *var_fn434_calc_ig__expirev_dn4_slot = var_fn434_calc_ig__expirev_dn4;
        *var_fn434_calc_ig__expirev_dn8_slot = var_fn434_calc_ig__expirev_dn8;
        *var_fn434_calc_ig__iginrec_slot = var_fn434_calc_ig__iginrec;
        *var_fn434_calc_ig__iginrec_dn17_slot = var_fn434_calc_ig__iginrec_dn17;
        *var_fn434_calc_ig__iginrec_dn4_slot = var_fn434_calc_ig__iginrec_dn4;
        *var_fn434_calc_ig__iginrec_dn8_slot = var_fn434_calc_ig__iginrec_dn8;
        *var_fn434_calc_ig__igout_slot = var_fn434_calc_ig__igout;
        *var_fn434_calc_ig__igout_dn17_slot = var_fn434_calc_ig__igout_dn17;
        *var_fn434_calc_ig__igout_dn4_slot = var_fn434_calc_ig__igout_dn4;
        *var_fn434_calc_ig__igout_dn8_slot = var_fn434_calc_ig__igout_dn8;
        *var_fn434_calc_ig__return_slot = var_fn434_calc_ig__return;
        *var_fn434_calc_ig__return_dn17_slot = var_fn434_calc_ig__return_dn17;
        *var_fn434_calc_ig__return_dn4_slot = var_fn434_calc_ig__return_dn4;
        *var_fn434_calc_ig__return_dn8_slot = var_fn434_calc_ig__return_dn8;
        *var_fn440_calc_ig__alpha2_phit_slot = var_fn440_calc_ig__alpha2_phit;
        *var_fn440_calc_ig__alpha2_phit_dn4_slot = var_fn440_calc_ig__alpha2_phit_dn4;
        *var_fn440_calc_ig__alphagin_slot = var_fn440_calc_ig__alphagin;
        *var_fn440_calc_ig__betarecin_slot = var_fn440_calc_ig__betarecin;
        *var_fn440_calc_ig__expbd1_slot = var_fn440_calc_ig__expbd1;
        *var_fn440_calc_ig__expbd1_dn4_slot = var_fn440_calc_ig__expbd1_dn4;
        *var_fn440_calc_ig__expbd1_dn8_slot = var_fn440_calc_ig__expbd1_dn8;
        *var_fn440_calc_ig__expbd1_dn9_slot = var_fn440_calc_ig__expbd1_dn9;
        *var_fn440_calc_ig__expbd2_slot = var_fn440_calc_ig__expbd2;
        *var_fn440_calc_ig__expbd2_dn4_slot = var_fn440_calc_ig__expbd2_dn4;
        *var_fn440_calc_ig__expbdarg1_slot = var_fn440_calc_ig__expbdarg1;
        *var_fn440_calc_ig__expbdarg1_dn4_slot = var_fn440_calc_ig__expbdarg1_dn4;
        *var_fn440_calc_ig__expbdarg1_dn8_slot = var_fn440_calc_ig__expbdarg1_dn8;
        *var_fn440_calc_ig__expbdarg1_dn9_slot = var_fn440_calc_ig__expbdarg1_dn9;
        *var_fn440_calc_ig__expbdarg2_slot = var_fn440_calc_ig__expbdarg2;
        *var_fn440_calc_ig__expbdarg2_dn4_slot = var_fn440_calc_ig__expbdarg2_dn4;
        *var_fn440_calc_ig__expffvarg_slot = var_fn440_calc_ig__expffvarg;
        *var_fn440_calc_ig__expffvarg_dn4_slot = var_fn440_calc_ig__expffvarg_dn4;
        *var_fn440_calc_ig__expffvarg_dn8_slot = var_fn440_calc_ig__expffvarg_dn8;
        *var_fn440_calc_ig__expffvarg_dn9_slot = var_fn440_calc_ig__expffvarg_dn9;
        *var_fn440_calc_ig__expifor_slot = var_fn440_calc_ig__expifor;
        *var_fn440_calc_ig__expifor_dn4_slot = var_fn440_calc_ig__expifor_dn4;
        *var_fn440_calc_ig__expifor_dn8_slot = var_fn440_calc_ig__expifor_dn8;
        *var_fn440_calc_ig__expifor_dn9_slot = var_fn440_calc_ig__expifor_dn9;
        *var_fn440_calc_ig__expiforarg_slot = var_fn440_calc_ig__expiforarg;
        *var_fn440_calc_ig__expiforarg_dn4_slot = var_fn440_calc_ig__expiforarg_dn4;
        *var_fn440_calc_ig__expiforarg_dn8_slot = var_fn440_calc_ig__expiforarg_dn8;
        *var_fn440_calc_ig__expiforarg_dn9_slot = var_fn440_calc_ig__expiforarg_dn9;
        *var_fn440_calc_ig__expirevarg_slot = var_fn440_calc_ig__expirevarg;
        *var_fn440_calc_ig__expirevarg_dn4_slot = var_fn440_calc_ig__expirevarg_dn4;
        *var_fn440_calc_ig__expirevarg_dn8_slot = var_fn440_calc_ig__expirevarg_dn8;
        *var_fn440_calc_ig__expirevarg_dn9_slot = var_fn440_calc_ig__expirevarg_dn9;
        *var_fn440_calc_ig__expphib_slot = var_fn440_calc_ig__expphib;
        *var_fn440_calc_ig__expphib_dn4_slot = var_fn440_calc_ig__expphib_dn4;
        *var_fn440_calc_ig__ffvgin_slot = var_fn440_calc_ig__ffvgin;
        *var_fn440_calc_ig__ffvgin_dn4_slot = var_fn440_calc_ig__ffvgin_dn4;
        *var_fn440_calc_ig__ffvgin_dn8_slot = var_fn440_calc_ig__ffvgin_dn8;
        *var_fn440_calc_ig__ffvgin_dn9_slot = var_fn440_calc_ig__ffvgin_dn9;
        *var_fn440_calc_ig__fracin_slot = var_fn440_calc_ig__fracin;
        *var_fn440_calc_ig__frecgin_slot = var_fn440_calc_ig__frecgin;
        *var_fn440_calc_ig__frecgin_dn8_slot = var_fn440_calc_ig__frecgin_dn8;
        *var_fn440_calc_ig__frecgin_dn9_slot = var_fn440_calc_ig__frecgin_dn9;
        *var_fn440_calc_ig__iginbd_slot = var_fn440_calc_ig__iginbd;
        *var_fn440_calc_ig__iginbd_dn4_slot = var_fn440_calc_ig__iginbd_dn4;
        *var_fn440_calc_ig__iginbd_dn8_slot = var_fn440_calc_ig__iginbd_dn8;
        *var_fn440_calc_ig__iginbd_dn9_slot = var_fn440_calc_ig__iginbd_dn9;
        *var_fn440_calc_ig__igindiode_slot = var_fn440_calc_ig__igindiode;
        *var_fn440_calc_ig__igindiode_dn4_slot = var_fn440_calc_ig__igindiode_dn4;
        *var_fn440_calc_ig__igindiode_dn8_slot = var_fn440_calc_ig__igindiode_dn8;
        *var_fn440_calc_ig__igindiode_dn9_slot = var_fn440_calc_ig__igindiode_dn9;
        *var_fn440_calc_ig__iginrec_slot = var_fn440_calc_ig__iginrec;
        *var_fn440_calc_ig__iginrec_dn4_slot = var_fn440_calc_ig__iginrec_dn4;
        *var_fn440_calc_ig__iginrec_dn8_slot = var_fn440_calc_ig__iginrec_dn8;
        *var_fn440_calc_ig__iginrec_dn9_slot = var_fn440_calc_ig__iginrec_dn9;
        *var_fn440_calc_ig__igout_slot = var_fn440_calc_ig__igout;
        *var_fn440_calc_ig__igout_dn4_slot = var_fn440_calc_ig__igout_dn4;
        *var_fn440_calc_ig__igout_dn8_slot = var_fn440_calc_ig__igout_dn8;
        *var_fn440_calc_ig__igout_dn9_slot = var_fn440_calc_ig__igout_dn9;
        *var_fn440_calc_ig__ijin_slot = var_fn440_calc_ig__ijin;
        *var_fn440_calc_ig__irecin_slot = var_fn440_calc_ig__irecin;
        *var_fn440_calc_ig__isdiodeout_slot = var_fn440_calc_ig__isdiodeout;
        *var_fn440_calc_ig__isdiodeout_dn4_slot = var_fn440_calc_ig__isdiodeout_dn4;
        *var_fn440_calc_ig__isrecout_slot = var_fn440_calc_ig__isrecout;
        *var_fn440_calc_ig__isrecout_dn4_slot = var_fn440_calc_ig__isrecout_dn4;
        *var_fn440_calc_ig__kbdgatein_slot = var_fn440_calc_ig__kbdgatein;
        *var_fn440_calc_ig__ngf_slot = var_fn440_calc_ig__ngf;
        *var_fn440_calc_ig__pbdgin_slot = var_fn440_calc_ig__pbdgin;
        *var_fn440_calc_ig__pg_param1_slot = var_fn440_calc_ig__pg_param1;
        *var_fn440_calc_ig__pg_paramin_slot = var_fn440_calc_ig__pg_paramin;
        *var_fn440_calc_ig__pgsrecin_slot = var_fn440_calc_ig__pgsrecin;
        *var_fn440_calc_ig__phitin_slot = var_fn440_calc_ig__phitin;
        *var_fn440_calc_ig__phitin_dn4_slot = var_fn440_calc_ig__phitin_dn4;
        *var_fn440_calc_ig__return_slot = var_fn440_calc_ig__return;
        *var_fn440_calc_ig__return_dn4_slot = var_fn440_calc_ig__return_dn4;
        *var_fn440_calc_ig__return_dn8_slot = var_fn440_calc_ig__return_dn8;
        *var_fn440_calc_ig__return_dn9_slot = var_fn440_calc_ig__return_dn9;
        *var_fn440_calc_ig__t0_slot = var_fn440_calc_ig__t0;
        *var_fn440_calc_ig__t0_dn4_slot = var_fn440_calc_ig__t0_dn4;
        *var_fn440_calc_ig__tfacdiodein_slot = var_fn440_calc_ig__tfacdiodein;
        *var_fn440_calc_ig__tfacdiodein_dn4_slot = var_fn440_calc_ig__tfacdiodein_dn4;
        *var_fn440_calc_ig__type_slot = var_fn440_calc_ig__type;
        *var_fn440_calc_ig__vbdgin_slot = var_fn440_calc_ig__vbdgin;
        *var_fn440_calc_ig__vgin_slot = var_fn440_calc_ig__vgin;
        *var_fn440_calc_ig__vgin_dn8_slot = var_fn440_calc_ig__vgin_dn8;
        *var_fn440_calc_ig__vgin_dn9_slot = var_fn440_calc_ig__vgin_dn9;
        *var_fn440_calc_ig__vgsatin_slot = var_fn440_calc_ig__vgsatin;
        *var_fn440_calc_ig__vgsatqin_slot = var_fn440_calc_ig__vgsatqin;
        *var_fn440_calc_ig__vjg_slot = var_fn440_calc_ig__vjg;
        *var_fn440_calc_ig__w_slot = var_fn440_calc_ig__w;
        *var_guard439_slot = var_guard439;
        *var_igdi2_slot = var_igdi2;
        *var_igdi2_dn17_slot = var_igdi2_dn17;
        *var_igdi2_dn4_slot = var_igdi2_dn4;
        *var_igdi2_dn8_slot = var_igdi2_dn8;
    }

    pub(super) fn stamp_transient_block_93(
        var_fn440_calc_ig__fracin: f64,
        var_fn440_calc_ig__ijin: f64,
        var_fn440_calc_ig__kbdgatein: f64,
        var_fn440_calc_ig__ngf: f64,
        var_fn440_calc_ig__pbdgin: f64,
        var_fn440_calc_ig__pg_param1: f64,
        var_fn440_calc_ig__pg_paramin: f64,
        var_fn440_calc_ig__phitin: f64,
        var_fn440_calc_ig__phitin_dn4: f64,
        var_fn440_calc_ig__tfacdiodein: f64,
        var_fn440_calc_ig__tfacdiodein_dn4: f64,
        var_fn440_calc_ig__type: f64,
        var_fn440_calc_ig__vbdgin: f64,
        var_fn440_calc_ig__vgin: f64,
        var_fn440_calc_ig__vgin_dn8: f64,
        var_fn440_calc_ig__vgin_dn9: f64,
        var_fn440_calc_ig__vgsatin: f64,
        var_fn440_calc_ig__vjg: f64,
        var_fn440_calc_ig__w: f64,
        var_guard417: f64,
        var_guard439: f64,
        var_fn440_calc_ig__expbd1_slot: &mut f64,
        var_fn440_calc_ig__expbd1_dn4_slot: &mut f64,
        var_fn440_calc_ig__expbd1_dn8_slot: &mut f64,
        var_fn440_calc_ig__expbd1_dn9_slot: &mut f64,
        var_fn440_calc_ig__expbd1_vgsat_slot: &mut f64,
        var_fn440_calc_ig__expbd1_vgsat_dn4_slot: &mut f64,
        var_fn440_calc_ig__expbd2_slot: &mut f64,
        var_fn440_calc_ig__expbd2_dn4_slot: &mut f64,
        var_fn440_calc_ig__expbdarg1_slot: &mut f64,
        var_fn440_calc_ig__expbdarg1_dn4_slot: &mut f64,
        var_fn440_calc_ig__expbdarg1_dn8_slot: &mut f64,
        var_fn440_calc_ig__expbdarg1_dn9_slot: &mut f64,
        var_fn440_calc_ig__expbdarg1_vgsat_slot: &mut f64,
        var_fn440_calc_ig__expbdarg1_vgsat_dn4_slot: &mut f64,
        var_fn440_calc_ig__expbdarg2_slot: &mut f64,
        var_fn440_calc_ig__expbdarg2_dn4_slot: &mut f64,
        var_fn440_calc_ig__expifor_slot: &mut f64,
        var_fn440_calc_ig__expifor_dn4_slot: &mut f64,
        var_fn440_calc_ig__expifor_dn8_slot: &mut f64,
        var_fn440_calc_ig__expifor_dn9_slot: &mut f64,
        var_fn440_calc_ig__expifor_hinj_slot: &mut f64,
        var_fn440_calc_ig__expifor_hinj_dn4_slot: &mut f64,
        var_fn440_calc_ig__expifor_hinj_dn8_slot: &mut f64,
        var_fn440_calc_ig__expifor_hinj_dn9_slot: &mut f64,
        var_fn440_calc_ig__expifor_hinj_vgsat_slot: &mut f64,
        var_fn440_calc_ig__expifor_hinj_vgsat_dn4_slot: &mut f64,
        var_fn440_calc_ig__expifor_nohinj_vgsat_slot: &mut f64,
        var_fn440_calc_ig__expifor_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn440_calc_ig__expiforarg_slot: &mut f64,
        var_fn440_calc_ig__expiforarg_dn4_slot: &mut f64,
        var_fn440_calc_ig__expiforarg_dn8_slot: &mut f64,
        var_fn440_calc_ig__expiforarg_dn9_slot: &mut f64,
        var_fn440_calc_ig__expiforarg_hinj_slot: &mut f64,
        var_fn440_calc_ig__expiforarg_hinj_dn4_slot: &mut f64,
        var_fn440_calc_ig__expiforarg_hinj_dn8_slot: &mut f64,
        var_fn440_calc_ig__expiforarg_hinj_dn9_slot: &mut f64,
        var_fn440_calc_ig__expiforarg_hinj_vgsat_slot: &mut f64,
        var_fn440_calc_ig__expiforarg_hinj_vgsat_dn4_slot: &mut f64,
        var_fn440_calc_ig__expiforarg_nohinj_vgsat_slot: &mut f64,
        var_fn440_calc_ig__expiforarg_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn440_calc_ig__expirev_slot: &mut f64,
        var_fn440_calc_ig__expirev_dn4_slot: &mut f64,
        var_fn440_calc_ig__expirev_dn8_slot: &mut f64,
        var_fn440_calc_ig__expirev_dn9_slot: &mut f64,
        var_fn440_calc_ig__expphib_slot: &mut f64,
        var_fn440_calc_ig__expphib_dn4_slot: &mut f64,
        var_fn440_calc_ig__iginbd_slot: &mut f64,
        var_fn440_calc_ig__iginbd_dn4_slot: &mut f64,
        var_fn440_calc_ig__iginbd_dn8_slot: &mut f64,
        var_fn440_calc_ig__iginbd_dn9_slot: &mut f64,
        var_fn440_calc_ig__iginbd_vgsat_slot: &mut f64,
        var_fn440_calc_ig__iginbd_vgsat_dn4_slot: &mut f64,
        var_fn440_calc_ig__igindiode_slot: &mut f64,
        var_fn440_calc_ig__igindiode_dn4_slot: &mut f64,
        var_fn440_calc_ig__igindiode_dn8_slot: &mut f64,
        var_fn440_calc_ig__igindiode_dn9_slot: &mut f64,
        var_fn440_calc_ig__igindiode_hinj_slot: &mut f64,
        var_fn440_calc_ig__igindiode_hinj_dn4_slot: &mut f64,
        var_fn440_calc_ig__igindiode_hinj_dn8_slot: &mut f64,
        var_fn440_calc_ig__igindiode_hinj_dn9_slot: &mut f64,
        var_fn440_calc_ig__igindiode_hinj_pre_slot: &mut f64,
        var_fn440_calc_ig__igindiode_hinj_pre_dn4_slot: &mut f64,
        var_fn440_calc_ig__igindiode_hinj_vgsat_slot: &mut f64,
        var_fn440_calc_ig__igindiode_hinj_vgsat_dn4_slot: &mut f64,
        var_fn440_calc_ig__igindiode_nohinj_slot: &mut f64,
        var_fn440_calc_ig__igindiode_nohinj_dn4_slot: &mut f64,
        var_fn440_calc_ig__igindiode_nohinj_dn8_slot: &mut f64,
        var_fn440_calc_ig__igindiode_nohinj_dn9_slot: &mut f64,
        var_fn440_calc_ig__igindiode_nohinj_vgsat_slot: &mut f64,
        var_fn440_calc_ig__igindiode_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn440_calc_ig__isdiodeout_slot: &mut f64,
        var_fn440_calc_ig__isdiodeout_dn4_slot: &mut f64,
        var_fn440_calc_ig__pg_paramin_hinj_slot: &mut f64,
        var_fn440_calc_ig__t0_slot: &mut f64,
        var_fn440_calc_ig__t0_dn4_slot: &mut f64,
        var_guard441_slot: &mut f64,
    ) {
        let mut var_fn440_calc_ig__expbd1: f64 = *var_fn440_calc_ig__expbd1_slot;
        let mut var_fn440_calc_ig__expbd1_dn4: f64 = *var_fn440_calc_ig__expbd1_dn4_slot;
        let mut var_fn440_calc_ig__expbd1_dn8: f64 = *var_fn440_calc_ig__expbd1_dn8_slot;
        let mut var_fn440_calc_ig__expbd1_dn9: f64 = *var_fn440_calc_ig__expbd1_dn9_slot;
        let mut var_fn440_calc_ig__expbd1_vgsat: f64 = *var_fn440_calc_ig__expbd1_vgsat_slot;
        let mut var_fn440_calc_ig__expbd1_vgsat_dn4: f64 = *var_fn440_calc_ig__expbd1_vgsat_dn4_slot;
        let mut var_fn440_calc_ig__expbd2: f64 = *var_fn440_calc_ig__expbd2_slot;
        let mut var_fn440_calc_ig__expbd2_dn4: f64 = *var_fn440_calc_ig__expbd2_dn4_slot;
        let mut var_fn440_calc_ig__expbdarg1: f64 = *var_fn440_calc_ig__expbdarg1_slot;
        let mut var_fn440_calc_ig__expbdarg1_dn4: f64 = *var_fn440_calc_ig__expbdarg1_dn4_slot;
        let mut var_fn440_calc_ig__expbdarg1_dn8: f64 = *var_fn440_calc_ig__expbdarg1_dn8_slot;
        let mut var_fn440_calc_ig__expbdarg1_dn9: f64 = *var_fn440_calc_ig__expbdarg1_dn9_slot;
        let mut var_fn440_calc_ig__expbdarg1_vgsat: f64 = *var_fn440_calc_ig__expbdarg1_vgsat_slot;
        let mut var_fn440_calc_ig__expbdarg1_vgsat_dn4: f64 = *var_fn440_calc_ig__expbdarg1_vgsat_dn4_slot;
        let mut var_fn440_calc_ig__expbdarg2: f64 = *var_fn440_calc_ig__expbdarg2_slot;
        let mut var_fn440_calc_ig__expbdarg2_dn4: f64 = *var_fn440_calc_ig__expbdarg2_dn4_slot;
        let mut var_fn440_calc_ig__expifor: f64 = *var_fn440_calc_ig__expifor_slot;
        let mut var_fn440_calc_ig__expifor_dn4: f64 = *var_fn440_calc_ig__expifor_dn4_slot;
        let mut var_fn440_calc_ig__expifor_dn8: f64 = *var_fn440_calc_ig__expifor_dn8_slot;
        let mut var_fn440_calc_ig__expifor_dn9: f64 = *var_fn440_calc_ig__expifor_dn9_slot;
        let mut var_fn440_calc_ig__expifor_hinj: f64 = *var_fn440_calc_ig__expifor_hinj_slot;
        let mut var_fn440_calc_ig__expifor_hinj_dn4: f64 = *var_fn440_calc_ig__expifor_hinj_dn4_slot;
        let mut var_fn440_calc_ig__expifor_hinj_dn8: f64 = *var_fn440_calc_ig__expifor_hinj_dn8_slot;
        let mut var_fn440_calc_ig__expifor_hinj_dn9: f64 = *var_fn440_calc_ig__expifor_hinj_dn9_slot;
        let mut var_fn440_calc_ig__expifor_hinj_vgsat: f64 = *var_fn440_calc_ig__expifor_hinj_vgsat_slot;
        let mut var_fn440_calc_ig__expifor_hinj_vgsat_dn4: f64 = *var_fn440_calc_ig__expifor_hinj_vgsat_dn4_slot;
        let mut var_fn440_calc_ig__expifor_nohinj_vgsat: f64 = *var_fn440_calc_ig__expifor_nohinj_vgsat_slot;
        let mut var_fn440_calc_ig__expifor_nohinj_vgsat_dn4: f64 = *var_fn440_calc_ig__expifor_nohinj_vgsat_dn4_slot;
        let mut var_fn440_calc_ig__expiforarg: f64 = *var_fn440_calc_ig__expiforarg_slot;
        let mut var_fn440_calc_ig__expiforarg_dn4: f64 = *var_fn440_calc_ig__expiforarg_dn4_slot;
        let mut var_fn440_calc_ig__expiforarg_dn8: f64 = *var_fn440_calc_ig__expiforarg_dn8_slot;
        let mut var_fn440_calc_ig__expiforarg_dn9: f64 = *var_fn440_calc_ig__expiforarg_dn9_slot;
        let mut var_fn440_calc_ig__expiforarg_hinj: f64 = *var_fn440_calc_ig__expiforarg_hinj_slot;
        let mut var_fn440_calc_ig__expiforarg_hinj_dn4: f64 = *var_fn440_calc_ig__expiforarg_hinj_dn4_slot;
        let mut var_fn440_calc_ig__expiforarg_hinj_dn8: f64 = *var_fn440_calc_ig__expiforarg_hinj_dn8_slot;
        let mut var_fn440_calc_ig__expiforarg_hinj_dn9: f64 = *var_fn440_calc_ig__expiforarg_hinj_dn9_slot;
        let mut var_fn440_calc_ig__expiforarg_hinj_vgsat: f64 = *var_fn440_calc_ig__expiforarg_hinj_vgsat_slot;
        let mut var_fn440_calc_ig__expiforarg_hinj_vgsat_dn4: f64 = *var_fn440_calc_ig__expiforarg_hinj_vgsat_dn4_slot;
        let mut var_fn440_calc_ig__expiforarg_nohinj_vgsat: f64 = *var_fn440_calc_ig__expiforarg_nohinj_vgsat_slot;
        let mut var_fn440_calc_ig__expiforarg_nohinj_vgsat_dn4: f64 = *var_fn440_calc_ig__expiforarg_nohinj_vgsat_dn4_slot;
        let mut var_fn440_calc_ig__expirev: f64 = *var_fn440_calc_ig__expirev_slot;
        let mut var_fn440_calc_ig__expirev_dn4: f64 = *var_fn440_calc_ig__expirev_dn4_slot;
        let mut var_fn440_calc_ig__expirev_dn8: f64 = *var_fn440_calc_ig__expirev_dn8_slot;
        let mut var_fn440_calc_ig__expirev_dn9: f64 = *var_fn440_calc_ig__expirev_dn9_slot;
        let mut var_fn440_calc_ig__expphib: f64 = *var_fn440_calc_ig__expphib_slot;
        let mut var_fn440_calc_ig__expphib_dn4: f64 = *var_fn440_calc_ig__expphib_dn4_slot;
        let mut var_fn440_calc_ig__iginbd: f64 = *var_fn440_calc_ig__iginbd_slot;
        let mut var_fn440_calc_ig__iginbd_dn4: f64 = *var_fn440_calc_ig__iginbd_dn4_slot;
        let mut var_fn440_calc_ig__iginbd_dn8: f64 = *var_fn440_calc_ig__iginbd_dn8_slot;
        let mut var_fn440_calc_ig__iginbd_dn9: f64 = *var_fn440_calc_ig__iginbd_dn9_slot;
        let mut var_fn440_calc_ig__iginbd_vgsat: f64 = *var_fn440_calc_ig__iginbd_vgsat_slot;
        let mut var_fn440_calc_ig__iginbd_vgsat_dn4: f64 = *var_fn440_calc_ig__iginbd_vgsat_dn4_slot;
        let mut var_fn440_calc_ig__igindiode: f64 = *var_fn440_calc_ig__igindiode_slot;
        let mut var_fn440_calc_ig__igindiode_dn4: f64 = *var_fn440_calc_ig__igindiode_dn4_slot;
        let mut var_fn440_calc_ig__igindiode_dn8: f64 = *var_fn440_calc_ig__igindiode_dn8_slot;
        let mut var_fn440_calc_ig__igindiode_dn9: f64 = *var_fn440_calc_ig__igindiode_dn9_slot;
        let mut var_fn440_calc_ig__igindiode_hinj: f64 = *var_fn440_calc_ig__igindiode_hinj_slot;
        let mut var_fn440_calc_ig__igindiode_hinj_dn4: f64 = *var_fn440_calc_ig__igindiode_hinj_dn4_slot;
        let mut var_fn440_calc_ig__igindiode_hinj_dn8: f64 = *var_fn440_calc_ig__igindiode_hinj_dn8_slot;
        let mut var_fn440_calc_ig__igindiode_hinj_dn9: f64 = *var_fn440_calc_ig__igindiode_hinj_dn9_slot;
        let mut var_fn440_calc_ig__igindiode_hinj_pre: f64 = *var_fn440_calc_ig__igindiode_hinj_pre_slot;
        let mut var_fn440_calc_ig__igindiode_hinj_pre_dn4: f64 = *var_fn440_calc_ig__igindiode_hinj_pre_dn4_slot;
        let mut var_fn440_calc_ig__igindiode_hinj_vgsat: f64 = *var_fn440_calc_ig__igindiode_hinj_vgsat_slot;
        let mut var_fn440_calc_ig__igindiode_hinj_vgsat_dn4: f64 = *var_fn440_calc_ig__igindiode_hinj_vgsat_dn4_slot;
        let mut var_fn440_calc_ig__igindiode_nohinj: f64 = *var_fn440_calc_ig__igindiode_nohinj_slot;
        let mut var_fn440_calc_ig__igindiode_nohinj_dn4: f64 = *var_fn440_calc_ig__igindiode_nohinj_dn4_slot;
        let mut var_fn440_calc_ig__igindiode_nohinj_dn8: f64 = *var_fn440_calc_ig__igindiode_nohinj_dn8_slot;
        let mut var_fn440_calc_ig__igindiode_nohinj_dn9: f64 = *var_fn440_calc_ig__igindiode_nohinj_dn9_slot;
        let mut var_fn440_calc_ig__igindiode_nohinj_vgsat: f64 = *var_fn440_calc_ig__igindiode_nohinj_vgsat_slot;
        let mut var_fn440_calc_ig__igindiode_nohinj_vgsat_dn4: f64 = *var_fn440_calc_ig__igindiode_nohinj_vgsat_dn4_slot;
        let mut var_fn440_calc_ig__isdiodeout: f64 = *var_fn440_calc_ig__isdiodeout_slot;
        let mut var_fn440_calc_ig__isdiodeout_dn4: f64 = *var_fn440_calc_ig__isdiodeout_dn4_slot;
        let mut var_fn440_calc_ig__pg_paramin_hinj: f64 = *var_fn440_calc_ig__pg_paramin_hinj_slot;
        let mut var_fn440_calc_ig__t0: f64 = *var_fn440_calc_ig__t0_slot;
        let mut var_fn440_calc_ig__t0_dn4: f64 = *var_fn440_calc_ig__t0_dn4_slot;
        let mut var_guard441: f64 = *var_guard441_slot;

        let (assign37600_e34748, assign37600_e34748_d_n4, assign37600_e34748_d_n8, assign37600_e34748_d_n9,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn440_calc_ig__expirev, var_fn440_calc_ig__expirev_dn4, var_fn440_calc_ig__expirev_dn8, var_fn440_calc_ig__expirev_dn9,)
    }
};
        var_fn440_calc_ig__expirev = assign37600_e34748;
        var_fn440_calc_ig__expirev_dn4 = assign37600_e34748_d_n4;
        var_fn440_calc_ig__expirev_dn8 = assign37600_e34748_d_n8;
        var_fn440_calc_ig__expirev_dn9 = assign37600_e34748_d_n9;

        let (assign37610_e34754,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0,)
    } else {
        (var_fn440_calc_ig__pg_paramin_hinj,)
    }
};
        var_fn440_calc_ig__pg_paramin_hinj = assign37610_e34754;

        let (assign37620_e34760, assign37620_e34760_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn440_calc_ig__expbdarg1_vgsat, var_fn440_calc_ig__expbdarg1_vgsat_dn4,)
    }
};
        var_fn440_calc_ig__expbdarg1_vgsat = assign37620_e34760;
        var_fn440_calc_ig__expbdarg1_vgsat_dn4 = assign37620_e34760_d_n4;

        let (assign37630_e34766, assign37630_e34766_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn440_calc_ig__expbd1_vgsat, var_fn440_calc_ig__expbd1_vgsat_dn4,)
    }
};
        var_fn440_calc_ig__expbd1_vgsat = assign37630_e34766;
        var_fn440_calc_ig__expbd1_vgsat_dn4 = assign37630_e34766_d_n4;

        let (assign37640_e34772, assign37640_e34772_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn440_calc_ig__iginbd_vgsat, var_fn440_calc_ig__iginbd_vgsat_dn4,)
    }
};
        var_fn440_calc_ig__iginbd_vgsat = assign37640_e34772;
        var_fn440_calc_ig__iginbd_vgsat_dn4 = assign37640_e34772_d_n4;

        let (assign37650_e34778, assign37650_e34778_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn440_calc_ig__expiforarg_nohinj_vgsat, var_fn440_calc_ig__expiforarg_nohinj_vgsat_dn4,)
    }
};
        var_fn440_calc_ig__expiforarg_nohinj_vgsat = assign37650_e34778;
        var_fn440_calc_ig__expiforarg_nohinj_vgsat_dn4 = assign37650_e34778_d_n4;

        let (assign37660_e34784, assign37660_e34784_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn440_calc_ig__expifor_nohinj_vgsat, var_fn440_calc_ig__expifor_nohinj_vgsat_dn4,)
    }
};
        var_fn440_calc_ig__expifor_nohinj_vgsat = assign37660_e34784;
        var_fn440_calc_ig__expifor_nohinj_vgsat_dn4 = assign37660_e34784_d_n4;

        let (assign37670_e34790, assign37670_e34790_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn440_calc_ig__igindiode_nohinj_vgsat, var_fn440_calc_ig__igindiode_nohinj_vgsat_dn4,)
    }
};
        var_fn440_calc_ig__igindiode_nohinj_vgsat = assign37670_e34790;
        var_fn440_calc_ig__igindiode_nohinj_vgsat_dn4 = assign37670_e34790_d_n4;

        let (assign37680_e34796, assign37680_e34796_d_n4, assign37680_e34796_d_n8, assign37680_e34796_d_n9,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn440_calc_ig__igindiode_nohinj, var_fn440_calc_ig__igindiode_nohinj_dn4, var_fn440_calc_ig__igindiode_nohinj_dn8, var_fn440_calc_ig__igindiode_nohinj_dn9,)
    }
};
        var_fn440_calc_ig__igindiode_nohinj = assign37680_e34796;
        var_fn440_calc_ig__igindiode_nohinj_dn4 = assign37680_e34796_d_n4;
        var_fn440_calc_ig__igindiode_nohinj_dn8 = assign37680_e34796_d_n8;
        var_fn440_calc_ig__igindiode_nohinj_dn9 = assign37680_e34796_d_n9;

        let (assign37690_e34802, assign37690_e34802_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn440_calc_ig__expiforarg_hinj_vgsat, var_fn440_calc_ig__expiforarg_hinj_vgsat_dn4,)
    }
};
        var_fn440_calc_ig__expiforarg_hinj_vgsat = assign37690_e34802;
        var_fn440_calc_ig__expiforarg_hinj_vgsat_dn4 = assign37690_e34802_d_n4;

        let (assign37700_e34808, assign37700_e34808_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn440_calc_ig__expifor_hinj_vgsat, var_fn440_calc_ig__expifor_hinj_vgsat_dn4,)
    }
};
        var_fn440_calc_ig__expifor_hinj_vgsat = assign37700_e34808;
        var_fn440_calc_ig__expifor_hinj_vgsat_dn4 = assign37700_e34808_d_n4;

        let (assign37710_e34814, assign37710_e34814_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn440_calc_ig__igindiode_hinj_vgsat, var_fn440_calc_ig__igindiode_hinj_vgsat_dn4,)
    }
};
        var_fn440_calc_ig__igindiode_hinj_vgsat = assign37710_e34814;
        var_fn440_calc_ig__igindiode_hinj_vgsat_dn4 = assign37710_e34814_d_n4;

        let (assign37720_e34820, assign37720_e34820_d_n4, assign37720_e34820_d_n8, assign37720_e34820_d_n9,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn440_calc_ig__expiforarg_hinj, var_fn440_calc_ig__expiforarg_hinj_dn4, var_fn440_calc_ig__expiforarg_hinj_dn8, var_fn440_calc_ig__expiforarg_hinj_dn9,)
    }
};
        var_fn440_calc_ig__expiforarg_hinj = assign37720_e34820;
        var_fn440_calc_ig__expiforarg_hinj_dn4 = assign37720_e34820_d_n4;
        var_fn440_calc_ig__expiforarg_hinj_dn8 = assign37720_e34820_d_n8;
        var_fn440_calc_ig__expiforarg_hinj_dn9 = assign37720_e34820_d_n9;

        let (assign37730_e34826, assign37730_e34826_d_n4, assign37730_e34826_d_n8, assign37730_e34826_d_n9,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn440_calc_ig__expifor_hinj, var_fn440_calc_ig__expifor_hinj_dn4, var_fn440_calc_ig__expifor_hinj_dn8, var_fn440_calc_ig__expifor_hinj_dn9,)
    }
};
        var_fn440_calc_ig__expifor_hinj = assign37730_e34826;
        var_fn440_calc_ig__expifor_hinj_dn4 = assign37730_e34826_d_n4;
        var_fn440_calc_ig__expifor_hinj_dn8 = assign37730_e34826_d_n8;
        var_fn440_calc_ig__expifor_hinj_dn9 = assign37730_e34826_d_n9;

        let (assign37740_e34832, assign37740_e34832_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn440_calc_ig__igindiode_hinj_pre, var_fn440_calc_ig__igindiode_hinj_pre_dn4,)
    }
};
        var_fn440_calc_ig__igindiode_hinj_pre = assign37740_e34832;
        var_fn440_calc_ig__igindiode_hinj_pre_dn4 = assign37740_e34832_d_n4;

        let (assign37750_e34838, assign37750_e34838_d_n4, assign37750_e34838_d_n8, assign37750_e34838_d_n9,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn440_calc_ig__igindiode_hinj, var_fn440_calc_ig__igindiode_hinj_dn4, var_fn440_calc_ig__igindiode_hinj_dn8, var_fn440_calc_ig__igindiode_hinj_dn9,)
    }
};
        var_fn440_calc_ig__igindiode_hinj = assign37750_e34838;
        var_fn440_calc_ig__igindiode_hinj_dn4 = assign37750_e34838_d_n4;
        var_fn440_calc_ig__igindiode_hinj_dn8 = assign37750_e34838_d_n8;
        var_fn440_calc_ig__igindiode_hinj_dn9 = assign37750_e34838_d_n9;

        let (assign37760_e34849, assign37760_e34849_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        let assign37760_e34844: f64 = (var_fn440_calc_ig__pg_param1 / var_fn440_calc_ig__phitin);
        let assign37760_e34846: f64 = (-var_fn440_calc_ig__vjg);
        let assign37760_e34847: f64 = (assign37760_e34844 * assign37760_e34846);
        (assign37760_e34847, ((-((var_fn440_calc_ig__pg_param1 * var_fn440_calc_ig__phitin_dn4) / (var_fn440_calc_ig__phitin * var_fn440_calc_ig__phitin))) * assign37760_e34846),)
    } else {
        (var_fn440_calc_ig__expphib, var_fn440_calc_ig__expphib_dn4,)
    }
};
        var_fn440_calc_ig__expphib = assign37760_e34849;
        var_fn440_calc_ig__expphib_dn4 = assign37760_e34849_d_n4;

        let (assign37770_e34893, assign37770_e34893_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        let assign37770_e34859: f64 = (-50.0);
        let (assign37770_e34891, assign37770_e34891_d_n4,) = {
            if ((!(var_fn440_calc_ig__expphib > 50.0)) && (!(var_fn440_calc_ig__expphib < assign37770_e34859))) {
                let assign37770_e34864: f64 = (var_fn440_calc_ig__expphib).exp();
                (assign37770_e34864, (assign37770_e34864 * var_fn440_calc_ig__expphib_dn4),)
            } else {
                let assign37770_e34871: f64 = (-50.0);
                let (assign37770_e34890, assign37770_e34890_d_n4,) = {
                    if ((!(var_fn440_calc_ig__expphib > 50.0)) && (var_fn440_calc_ig__expphib < assign37770_e34871)) {
                        let assign37770_e34875: f64 = (-50.0);
                        let assign37770_e34876: f64 = (assign37770_e34875).exp();
                        (assign37770_e34876, 0.0,)
                    } else {
                        let (assign37770_e34889, assign37770_e34889_d_n4,) = {
                            if (var_fn440_calc_ig__expphib > 50.0) {
                                let assign37770_e34881: f64 = (50.0_f64).exp();
                                let assign37770_e34885: f64 = (var_fn440_calc_ig__expphib - 50.0);
                                let assign37770_e34886: f64 = (1.0 + assign37770_e34885);
                                let assign37770_e34887: f64 = (assign37770_e34881 * assign37770_e34886);
                                (assign37770_e34887, (assign37770_e34881 * var_fn440_calc_ig__expphib_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign37770_e34889, assign37770_e34889_d_n4,)
                    }
                };
                (assign37770_e34890, assign37770_e34890_d_n4,)
            }
        };
        (assign37770_e34891, assign37770_e34891_d_n4,)
    } else {
        (var_fn440_calc_ig__t0, var_fn440_calc_ig__t0_dn4,)
    }
};
        var_fn440_calc_ig__t0 = assign37770_e34893;
        var_fn440_calc_ig__t0_dn4 = assign37770_e34893_d_n4;

        let (assign37780_e34906, assign37780_e34906_d_n4, assign37780_e34906_d_n8, assign37780_e34906_d_n9,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        let assign37780_e34899: f64 = (-var_fn440_calc_ig__vgin);
        let assign37780_e34901: f64 = (assign37780_e34899 - var_fn440_calc_ig__vbdgin);
        let assign37780_e34902: f64 = (var_fn440_calc_ig__pbdgin * assign37780_e34901);
        let assign37780_e34904: f64 = (assign37780_e34902 + var_fn440_calc_ig__expphib);
        (assign37780_e34904, var_fn440_calc_ig__expphib_dn4, (var_fn440_calc_ig__pbdgin * (-var_fn440_calc_ig__vgin_dn8)), (var_fn440_calc_ig__pbdgin * (-var_fn440_calc_ig__vgin_dn9)),)
    } else {
        (var_fn440_calc_ig__expbdarg1, var_fn440_calc_ig__expbdarg1_dn4, var_fn440_calc_ig__expbdarg1_dn8, var_fn440_calc_ig__expbdarg1_dn9,)
    }
};
        var_fn440_calc_ig__expbdarg1 = assign37780_e34906;
        var_fn440_calc_ig__expbdarg1_dn4 = assign37780_e34906_d_n4;
        var_fn440_calc_ig__expbdarg1_dn8 = assign37780_e34906_d_n8;
        var_fn440_calc_ig__expbdarg1_dn9 = assign37780_e34906_d_n9;

        let (assign37790_e34917, assign37790_e34917_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        let assign37790_e34911: f64 = (-var_fn440_calc_ig__pbdgin);
        let assign37790_e34913: f64 = (assign37790_e34911 * var_fn440_calc_ig__vbdgin);
        let assign37790_e34915: f64 = (assign37790_e34913 + var_fn440_calc_ig__expphib);
        (assign37790_e34915, var_fn440_calc_ig__expphib_dn4,)
    } else {
        (var_fn440_calc_ig__expbdarg2, var_fn440_calc_ig__expbdarg2_dn4,)
    }
};
        var_fn440_calc_ig__expbdarg2 = assign37790_e34917;
        var_fn440_calc_ig__expbdarg2_dn4 = assign37790_e34917_d_n4;

        let (assign37800_e34961, assign37800_e34961_d_n4, assign37800_e34961_d_n8, assign37800_e34961_d_n9,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        let assign37800_e34927: f64 = (-50.0);
        let (assign37800_e34959, assign37800_e34959_d_n4, assign37800_e34959_d_n8, assign37800_e34959_d_n9,) = {
            if ((!(var_fn440_calc_ig__expbdarg1 > 50.0)) && (!(var_fn440_calc_ig__expbdarg1 < assign37800_e34927))) {
                let assign37800_e34932: f64 = (var_fn440_calc_ig__expbdarg1).exp();
                (assign37800_e34932, (assign37800_e34932 * var_fn440_calc_ig__expbdarg1_dn4), (assign37800_e34932 * var_fn440_calc_ig__expbdarg1_dn8), (assign37800_e34932 * var_fn440_calc_ig__expbdarg1_dn9),)
            } else {
                let assign37800_e34939: f64 = (-50.0);
                let (assign37800_e34958, assign37800_e34958_d_n4, assign37800_e34958_d_n8, assign37800_e34958_d_n9,) = {
                    if ((!(var_fn440_calc_ig__expbdarg1 > 50.0)) && (var_fn440_calc_ig__expbdarg1 < assign37800_e34939)) {
                        let assign37800_e34943: f64 = (-50.0);
                        let assign37800_e34944: f64 = (assign37800_e34943).exp();
                        (assign37800_e34944, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign37800_e34957, assign37800_e34957_d_n4, assign37800_e34957_d_n8, assign37800_e34957_d_n9,) = {
                            if (var_fn440_calc_ig__expbdarg1 > 50.0) {
                                let assign37800_e34949: f64 = (50.0_f64).exp();
                                let assign37800_e34953: f64 = (var_fn440_calc_ig__expbdarg1 - 50.0);
                                let assign37800_e34954: f64 = (1.0 + assign37800_e34953);
                                let assign37800_e34955: f64 = (assign37800_e34949 * assign37800_e34954);
                                (assign37800_e34955, (assign37800_e34949 * var_fn440_calc_ig__expbdarg1_dn4), (assign37800_e34949 * var_fn440_calc_ig__expbdarg1_dn8), (assign37800_e34949 * var_fn440_calc_ig__expbdarg1_dn9),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign37800_e34957, assign37800_e34957_d_n4, assign37800_e34957_d_n8, assign37800_e34957_d_n9,)
                    }
                };
                (assign37800_e34958, assign37800_e34958_d_n4, assign37800_e34958_d_n8, assign37800_e34958_d_n9,)
            }
        };
        (assign37800_e34959, assign37800_e34959_d_n4, assign37800_e34959_d_n8, assign37800_e34959_d_n9,)
    } else {
        (var_fn440_calc_ig__expbd1, var_fn440_calc_ig__expbd1_dn4, var_fn440_calc_ig__expbd1_dn8, var_fn440_calc_ig__expbd1_dn9,)
    }
};
        var_fn440_calc_ig__expbd1 = assign37800_e34961;
        var_fn440_calc_ig__expbd1_dn4 = assign37800_e34961_d_n4;
        var_fn440_calc_ig__expbd1_dn8 = assign37800_e34961_d_n8;
        var_fn440_calc_ig__expbd1_dn9 = assign37800_e34961_d_n9;

        let (assign37810_e35005, assign37810_e35005_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        let assign37810_e34971: f64 = (-50.0);
        let (assign37810_e35003, assign37810_e35003_d_n4,) = {
            if ((!(var_fn440_calc_ig__expbdarg2 > 50.0)) && (!(var_fn440_calc_ig__expbdarg2 < assign37810_e34971))) {
                let assign37810_e34976: f64 = (var_fn440_calc_ig__expbdarg2).exp();
                (assign37810_e34976, (assign37810_e34976 * var_fn440_calc_ig__expbdarg2_dn4),)
            } else {
                let assign37810_e34983: f64 = (-50.0);
                let (assign37810_e35002, assign37810_e35002_d_n4,) = {
                    if ((!(var_fn440_calc_ig__expbdarg2 > 50.0)) && (var_fn440_calc_ig__expbdarg2 < assign37810_e34983)) {
                        let assign37810_e34987: f64 = (-50.0);
                        let assign37810_e34988: f64 = (assign37810_e34987).exp();
                        (assign37810_e34988, 0.0,)
                    } else {
                        let (assign37810_e35001, assign37810_e35001_d_n4,) = {
                            if (var_fn440_calc_ig__expbdarg2 > 50.0) {
                                let assign37810_e34993: f64 = (50.0_f64).exp();
                                let assign37810_e34997: f64 = (var_fn440_calc_ig__expbdarg2 - 50.0);
                                let assign37810_e34998: f64 = (1.0 + assign37810_e34997);
                                let assign37810_e34999: f64 = (assign37810_e34993 * assign37810_e34998);
                                (assign37810_e34999, (assign37810_e34993 * var_fn440_calc_ig__expbdarg2_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign37810_e35001, assign37810_e35001_d_n4,)
                    }
                };
                (assign37810_e35002, assign37810_e35002_d_n4,)
            }
        };
        (assign37810_e35003, assign37810_e35003_d_n4,)
    } else {
        (var_fn440_calc_ig__expbd2, var_fn440_calc_ig__expbd2_dn4,)
    }
};
        var_fn440_calc_ig__expbd2 = assign37810_e35005;
        var_fn440_calc_ig__expbd2_dn4 = assign37810_e35005_d_n4;

        let (assign37820_e35013, assign37820_e35013_d_n4, assign37820_e35013_d_n8, assign37820_e35013_d_n9,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        let assign37820_e35011: f64 = (var_fn440_calc_ig__expbd1 - var_fn440_calc_ig__expbd2);
        (assign37820_e35011, (var_fn440_calc_ig__expbd1_dn4 - var_fn440_calc_ig__expbd2_dn4), var_fn440_calc_ig__expbd1_dn8, var_fn440_calc_ig__expbd1_dn9,)
    } else {
        (var_fn440_calc_ig__iginbd, var_fn440_calc_ig__iginbd_dn4, var_fn440_calc_ig__iginbd_dn8, var_fn440_calc_ig__iginbd_dn9,)
    }
};
        var_fn440_calc_ig__iginbd = assign37820_e35013;
        var_fn440_calc_ig__iginbd_dn4 = assign37820_e35013_d_n4;
        var_fn440_calc_ig__iginbd_dn8 = assign37820_e35013_d_n8;
        var_fn440_calc_ig__iginbd_dn9 = assign37820_e35013_d_n9;

        let (assign37830_e35027, assign37830_e35027_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        let assign37830_e35019: f64 = (var_fn440_calc_ig__type * var_fn440_calc_ig__w);
        let assign37830_e35021: f64 = (assign37830_e35019 * var_fn440_calc_ig__ngf);
        let assign37830_e35023: f64 = (assign37830_e35021 * var_fn440_calc_ig__ijin);
        let assign37830_e35025: f64 = (assign37830_e35023 * var_fn440_calc_ig__tfacdiodein);
        (assign37830_e35025, (assign37830_e35023 * var_fn440_calc_ig__tfacdiodein_dn4),)
    } else {
        (var_fn440_calc_ig__isdiodeout, var_fn440_calc_ig__isdiodeout_dn4,)
    }
};
        var_fn440_calc_ig__isdiodeout = assign37830_e35027;
        var_fn440_calc_ig__isdiodeout_dn4 = assign37830_e35027_d_n4;

        let (assign37840_e35039, assign37840_e35039_d_n4, assign37840_e35039_d_n8, assign37840_e35039_d_n9,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        let assign37840_e35033: f64 = (var_fn440_calc_ig__pg_paramin / var_fn440_calc_ig__phitin);
        let assign37840_e35035: f64 = (assign37840_e35033 * var_fn440_calc_ig__vgin);
        let assign37840_e35037: f64 = (assign37840_e35035 + var_fn440_calc_ig__expphib);
        (assign37840_e35037, (((-((var_fn440_calc_ig__pg_paramin * var_fn440_calc_ig__phitin_dn4) / (var_fn440_calc_ig__phitin * var_fn440_calc_ig__phitin))) * var_fn440_calc_ig__vgin) + var_fn440_calc_ig__expphib_dn4), (assign37840_e35033 * var_fn440_calc_ig__vgin_dn8), (assign37840_e35033 * var_fn440_calc_ig__vgin_dn9),)
    } else {
        (var_fn440_calc_ig__expiforarg, var_fn440_calc_ig__expiforarg_dn4, var_fn440_calc_ig__expiforarg_dn8, var_fn440_calc_ig__expiforarg_dn9,)
    }
};
        var_fn440_calc_ig__expiforarg = assign37840_e35039;
        var_fn440_calc_ig__expiforarg_dn4 = assign37840_e35039_d_n4;
        var_fn440_calc_ig__expiforarg_dn8 = assign37840_e35039_d_n8;
        var_fn440_calc_ig__expiforarg_dn9 = assign37840_e35039_d_n9;

        let (assign37850_e35083, assign37850_e35083_d_n4, assign37850_e35083_d_n8, assign37850_e35083_d_n9,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        let assign37850_e35049: f64 = (-50.0);
        let (assign37850_e35081, assign37850_e35081_d_n4, assign37850_e35081_d_n8, assign37850_e35081_d_n9,) = {
            if ((!(var_fn440_calc_ig__expiforarg > 50.0)) && (!(var_fn440_calc_ig__expiforarg < assign37850_e35049))) {
                let assign37850_e35054: f64 = (var_fn440_calc_ig__expiforarg).exp();
                (assign37850_e35054, (assign37850_e35054 * var_fn440_calc_ig__expiforarg_dn4), (assign37850_e35054 * var_fn440_calc_ig__expiforarg_dn8), (assign37850_e35054 * var_fn440_calc_ig__expiforarg_dn9),)
            } else {
                let assign37850_e35061: f64 = (-50.0);
                let (assign37850_e35080, assign37850_e35080_d_n4, assign37850_e35080_d_n8, assign37850_e35080_d_n9,) = {
                    if ((!(var_fn440_calc_ig__expiforarg > 50.0)) && (var_fn440_calc_ig__expiforarg < assign37850_e35061)) {
                        let assign37850_e35065: f64 = (-50.0);
                        let assign37850_e35066: f64 = (assign37850_e35065).exp();
                        (assign37850_e35066, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign37850_e35079, assign37850_e35079_d_n4, assign37850_e35079_d_n8, assign37850_e35079_d_n9,) = {
                            if (var_fn440_calc_ig__expiforarg > 50.0) {
                                let assign37850_e35071: f64 = (50.0_f64).exp();
                                let assign37850_e35075: f64 = (var_fn440_calc_ig__expiforarg - 50.0);
                                let assign37850_e35076: f64 = (1.0 + assign37850_e35075);
                                let assign37850_e35077: f64 = (assign37850_e35071 * assign37850_e35076);
                                (assign37850_e35077, (assign37850_e35071 * var_fn440_calc_ig__expiforarg_dn4), (assign37850_e35071 * var_fn440_calc_ig__expiforarg_dn8), (assign37850_e35071 * var_fn440_calc_ig__expiforarg_dn9),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign37850_e35079, assign37850_e35079_d_n4, assign37850_e35079_d_n8, assign37850_e35079_d_n9,)
                    }
                };
                (assign37850_e35080, assign37850_e35080_d_n4, assign37850_e35080_d_n8, assign37850_e35080_d_n9,)
            }
        };
        (assign37850_e35081, assign37850_e35081_d_n4, assign37850_e35081_d_n8, assign37850_e35081_d_n9,)
    } else {
        (var_fn440_calc_ig__expifor, var_fn440_calc_ig__expifor_dn4, var_fn440_calc_ig__expifor_dn8, var_fn440_calc_ig__expifor_dn9,)
    }
};
        var_fn440_calc_ig__expifor = assign37850_e35083;
        var_fn440_calc_ig__expifor_dn4 = assign37850_e35083_d_n4;
        var_fn440_calc_ig__expifor_dn8 = assign37850_e35083_d_n8;
        var_fn440_calc_ig__expifor_dn9 = assign37850_e35083_d_n9;

        let assign37860_e35086: f64 = if var_fn440_calc_ig__fracin == 1.0 { 1.0 } else { 0.0 };
        var_guard441 = assign37860_e35086;

        let (assign37870_e35102, assign37870_e35102_d_n4, assign37870_e35102_d_n8, assign37870_e35102_d_n9,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard441 != 0.0)) {
        let assign37870_e35096: f64 = (var_fn440_calc_ig__kbdgatein * var_fn440_calc_ig__iginbd);
        let assign37870_e35097: f64 = (var_fn440_calc_ig__expifor - assign37870_e35096);
        let assign37870_e35099: f64 = (assign37870_e35097 - var_fn440_calc_ig__t0);
        let assign37870_e35100: f64 = (var_fn440_calc_ig__isdiodeout * assign37870_e35099);
        (assign37870_e35100, ((var_fn440_calc_ig__isdiodeout_dn4 * assign37870_e35099) + (var_fn440_calc_ig__isdiodeout * ((var_fn440_calc_ig__expifor_dn4 - (var_fn440_calc_ig__kbdgatein * var_fn440_calc_ig__iginbd_dn4)) - var_fn440_calc_ig__t0_dn4))), (var_fn440_calc_ig__isdiodeout * (var_fn440_calc_ig__expifor_dn8 - (var_fn440_calc_ig__kbdgatein * var_fn440_calc_ig__iginbd_dn8))), (var_fn440_calc_ig__isdiodeout * (var_fn440_calc_ig__expifor_dn9 - (var_fn440_calc_ig__kbdgatein * var_fn440_calc_ig__iginbd_dn9))),)
    } else {
        (var_fn440_calc_ig__igindiode, var_fn440_calc_ig__igindiode_dn4, var_fn440_calc_ig__igindiode_dn8, var_fn440_calc_ig__igindiode_dn9,)
    }
};
        var_fn440_calc_ig__igindiode = assign37870_e35102;
        var_fn440_calc_ig__igindiode_dn4 = assign37870_e35102_d_n4;
        var_fn440_calc_ig__igindiode_dn8 = assign37870_e35102_d_n8;
        var_fn440_calc_ig__igindiode_dn9 = assign37870_e35102_d_n9;

        let (assign37880_e35118, assign37880_e35118_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard441 == 0.0)) {
        let assign37880_e35111: f64 = (-var_fn440_calc_ig__vgsatin);
        let assign37880_e35113: f64 = (assign37880_e35111 - var_fn440_calc_ig__vbdgin);
        let assign37880_e35114: f64 = (var_fn440_calc_ig__pbdgin * assign37880_e35113);
        let assign37880_e35116: f64 = (assign37880_e35114 + var_fn440_calc_ig__expphib);
        (assign37880_e35116, var_fn440_calc_ig__expphib_dn4,)
    } else {
        (var_fn440_calc_ig__expbdarg1_vgsat, var_fn440_calc_ig__expbdarg1_vgsat_dn4,)
    }
};
        var_fn440_calc_ig__expbdarg1_vgsat = assign37880_e35118;
        var_fn440_calc_ig__expbdarg1_vgsat_dn4 = assign37880_e35118_d_n4;

        let (assign37890_e35165, assign37890_e35165_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard441 == 0.0)) {
        let assign37890_e35131: f64 = (-50.0);
        let (assign37890_e35163, assign37890_e35163_d_n4,) = {
            if ((!(var_fn440_calc_ig__expbdarg1_vgsat > 50.0)) && (!(var_fn440_calc_ig__expbdarg1_vgsat < assign37890_e35131))) {
                let assign37890_e35136: f64 = (var_fn440_calc_ig__expbdarg1_vgsat).exp();
                (assign37890_e35136, (assign37890_e35136 * var_fn440_calc_ig__expbdarg1_vgsat_dn4),)
            } else {
                let assign37890_e35143: f64 = (-50.0);
                let (assign37890_e35162, assign37890_e35162_d_n4,) = {
                    if ((!(var_fn440_calc_ig__expbdarg1_vgsat > 50.0)) && (var_fn440_calc_ig__expbdarg1_vgsat < assign37890_e35143)) {
                        let assign37890_e35147: f64 = (-50.0);
                        let assign37890_e35148: f64 = (assign37890_e35147).exp();
                        (assign37890_e35148, 0.0,)
                    } else {
                        let (assign37890_e35161, assign37890_e35161_d_n4,) = {
                            if (var_fn440_calc_ig__expbdarg1_vgsat > 50.0) {
                                let assign37890_e35153: f64 = (50.0_f64).exp();
                                let assign37890_e35157: f64 = (var_fn440_calc_ig__expbdarg1_vgsat - 50.0);
                                let assign37890_e35158: f64 = (1.0 + assign37890_e35157);
                                let assign37890_e35159: f64 = (assign37890_e35153 * assign37890_e35158);
                                (assign37890_e35159, (assign37890_e35153 * var_fn440_calc_ig__expbdarg1_vgsat_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign37890_e35161, assign37890_e35161_d_n4,)
                    }
                };
                (assign37890_e35162, assign37890_e35162_d_n4,)
            }
        };
        (assign37890_e35163, assign37890_e35163_d_n4,)
    } else {
        (var_fn440_calc_ig__expbd1_vgsat, var_fn440_calc_ig__expbd1_vgsat_dn4,)
    }
};
        var_fn440_calc_ig__expbd1_vgsat = assign37890_e35165;
        var_fn440_calc_ig__expbd1_vgsat_dn4 = assign37890_e35165_d_n4;

        let (assign37900_e35176, assign37900_e35176_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard441 == 0.0)) {
        let assign37900_e35174: f64 = (var_fn440_calc_ig__expbd1_vgsat - var_fn440_calc_ig__expbd2);
        (assign37900_e35174, (var_fn440_calc_ig__expbd1_vgsat_dn4 - var_fn440_calc_ig__expbd2_dn4),)
    } else {
        (var_fn440_calc_ig__iginbd_vgsat, var_fn440_calc_ig__iginbd_vgsat_dn4,)
    }
};
        var_fn440_calc_ig__iginbd_vgsat = assign37900_e35176;
        var_fn440_calc_ig__iginbd_vgsat_dn4 = assign37900_e35176_d_n4;

        *var_fn440_calc_ig__expbd1_slot = var_fn440_calc_ig__expbd1;
        *var_fn440_calc_ig__expbd1_dn4_slot = var_fn440_calc_ig__expbd1_dn4;
        *var_fn440_calc_ig__expbd1_dn8_slot = var_fn440_calc_ig__expbd1_dn8;
        *var_fn440_calc_ig__expbd1_dn9_slot = var_fn440_calc_ig__expbd1_dn9;
        *var_fn440_calc_ig__expbd1_vgsat_slot = var_fn440_calc_ig__expbd1_vgsat;
        *var_fn440_calc_ig__expbd1_vgsat_dn4_slot = var_fn440_calc_ig__expbd1_vgsat_dn4;
        *var_fn440_calc_ig__expbd2_slot = var_fn440_calc_ig__expbd2;
        *var_fn440_calc_ig__expbd2_dn4_slot = var_fn440_calc_ig__expbd2_dn4;
        *var_fn440_calc_ig__expbdarg1_slot = var_fn440_calc_ig__expbdarg1;
        *var_fn440_calc_ig__expbdarg1_dn4_slot = var_fn440_calc_ig__expbdarg1_dn4;
        *var_fn440_calc_ig__expbdarg1_dn8_slot = var_fn440_calc_ig__expbdarg1_dn8;
        *var_fn440_calc_ig__expbdarg1_dn9_slot = var_fn440_calc_ig__expbdarg1_dn9;
        *var_fn440_calc_ig__expbdarg1_vgsat_slot = var_fn440_calc_ig__expbdarg1_vgsat;
        *var_fn440_calc_ig__expbdarg1_vgsat_dn4_slot = var_fn440_calc_ig__expbdarg1_vgsat_dn4;
        *var_fn440_calc_ig__expbdarg2_slot = var_fn440_calc_ig__expbdarg2;
        *var_fn440_calc_ig__expbdarg2_dn4_slot = var_fn440_calc_ig__expbdarg2_dn4;
        *var_fn440_calc_ig__expifor_slot = var_fn440_calc_ig__expifor;
        *var_fn440_calc_ig__expifor_dn4_slot = var_fn440_calc_ig__expifor_dn4;
        *var_fn440_calc_ig__expifor_dn8_slot = var_fn440_calc_ig__expifor_dn8;
        *var_fn440_calc_ig__expifor_dn9_slot = var_fn440_calc_ig__expifor_dn9;
        *var_fn440_calc_ig__expifor_hinj_slot = var_fn440_calc_ig__expifor_hinj;
        *var_fn440_calc_ig__expifor_hinj_dn4_slot = var_fn440_calc_ig__expifor_hinj_dn4;
        *var_fn440_calc_ig__expifor_hinj_dn8_slot = var_fn440_calc_ig__expifor_hinj_dn8;
        *var_fn440_calc_ig__expifor_hinj_dn9_slot = var_fn440_calc_ig__expifor_hinj_dn9;
        *var_fn440_calc_ig__expifor_hinj_vgsat_slot = var_fn440_calc_ig__expifor_hinj_vgsat;
        *var_fn440_calc_ig__expifor_hinj_vgsat_dn4_slot = var_fn440_calc_ig__expifor_hinj_vgsat_dn4;
        *var_fn440_calc_ig__expifor_nohinj_vgsat_slot = var_fn440_calc_ig__expifor_nohinj_vgsat;
        *var_fn440_calc_ig__expifor_nohinj_vgsat_dn4_slot = var_fn440_calc_ig__expifor_nohinj_vgsat_dn4;
        *var_fn440_calc_ig__expiforarg_slot = var_fn440_calc_ig__expiforarg;
        *var_fn440_calc_ig__expiforarg_dn4_slot = var_fn440_calc_ig__expiforarg_dn4;
        *var_fn440_calc_ig__expiforarg_dn8_slot = var_fn440_calc_ig__expiforarg_dn8;
        *var_fn440_calc_ig__expiforarg_dn9_slot = var_fn440_calc_ig__expiforarg_dn9;
        *var_fn440_calc_ig__expiforarg_hinj_slot = var_fn440_calc_ig__expiforarg_hinj;
        *var_fn440_calc_ig__expiforarg_hinj_dn4_slot = var_fn440_calc_ig__expiforarg_hinj_dn4;
        *var_fn440_calc_ig__expiforarg_hinj_dn8_slot = var_fn440_calc_ig__expiforarg_hinj_dn8;
        *var_fn440_calc_ig__expiforarg_hinj_dn9_slot = var_fn440_calc_ig__expiforarg_hinj_dn9;
        *var_fn440_calc_ig__expiforarg_hinj_vgsat_slot = var_fn440_calc_ig__expiforarg_hinj_vgsat;
        *var_fn440_calc_ig__expiforarg_hinj_vgsat_dn4_slot = var_fn440_calc_ig__expiforarg_hinj_vgsat_dn4;
        *var_fn440_calc_ig__expiforarg_nohinj_vgsat_slot = var_fn440_calc_ig__expiforarg_nohinj_vgsat;
        *var_fn440_calc_ig__expiforarg_nohinj_vgsat_dn4_slot = var_fn440_calc_ig__expiforarg_nohinj_vgsat_dn4;
        *var_fn440_calc_ig__expirev_slot = var_fn440_calc_ig__expirev;
        *var_fn440_calc_ig__expirev_dn4_slot = var_fn440_calc_ig__expirev_dn4;
        *var_fn440_calc_ig__expirev_dn8_slot = var_fn440_calc_ig__expirev_dn8;
        *var_fn440_calc_ig__expirev_dn9_slot = var_fn440_calc_ig__expirev_dn9;
        *var_fn440_calc_ig__expphib_slot = var_fn440_calc_ig__expphib;
        *var_fn440_calc_ig__expphib_dn4_slot = var_fn440_calc_ig__expphib_dn4;
        *var_fn440_calc_ig__iginbd_slot = var_fn440_calc_ig__iginbd;
        *var_fn440_calc_ig__iginbd_dn4_slot = var_fn440_calc_ig__iginbd_dn4;
        *var_fn440_calc_ig__iginbd_dn8_slot = var_fn440_calc_ig__iginbd_dn8;
        *var_fn440_calc_ig__iginbd_dn9_slot = var_fn440_calc_ig__iginbd_dn9;
        *var_fn440_calc_ig__iginbd_vgsat_slot = var_fn440_calc_ig__iginbd_vgsat;
        *var_fn440_calc_ig__iginbd_vgsat_dn4_slot = var_fn440_calc_ig__iginbd_vgsat_dn4;
        *var_fn440_calc_ig__igindiode_slot = var_fn440_calc_ig__igindiode;
        *var_fn440_calc_ig__igindiode_dn4_slot = var_fn440_calc_ig__igindiode_dn4;
        *var_fn440_calc_ig__igindiode_dn8_slot = var_fn440_calc_ig__igindiode_dn8;
        *var_fn440_calc_ig__igindiode_dn9_slot = var_fn440_calc_ig__igindiode_dn9;
        *var_fn440_calc_ig__igindiode_hinj_slot = var_fn440_calc_ig__igindiode_hinj;
        *var_fn440_calc_ig__igindiode_hinj_dn4_slot = var_fn440_calc_ig__igindiode_hinj_dn4;
        *var_fn440_calc_ig__igindiode_hinj_dn8_slot = var_fn440_calc_ig__igindiode_hinj_dn8;
        *var_fn440_calc_ig__igindiode_hinj_dn9_slot = var_fn440_calc_ig__igindiode_hinj_dn9;
        *var_fn440_calc_ig__igindiode_hinj_pre_slot = var_fn440_calc_ig__igindiode_hinj_pre;
        *var_fn440_calc_ig__igindiode_hinj_pre_dn4_slot = var_fn440_calc_ig__igindiode_hinj_pre_dn4;
        *var_fn440_calc_ig__igindiode_hinj_vgsat_slot = var_fn440_calc_ig__igindiode_hinj_vgsat;
        *var_fn440_calc_ig__igindiode_hinj_vgsat_dn4_slot = var_fn440_calc_ig__igindiode_hinj_vgsat_dn4;
        *var_fn440_calc_ig__igindiode_nohinj_slot = var_fn440_calc_ig__igindiode_nohinj;
        *var_fn440_calc_ig__igindiode_nohinj_dn4_slot = var_fn440_calc_ig__igindiode_nohinj_dn4;
        *var_fn440_calc_ig__igindiode_nohinj_dn8_slot = var_fn440_calc_ig__igindiode_nohinj_dn8;
        *var_fn440_calc_ig__igindiode_nohinj_dn9_slot = var_fn440_calc_ig__igindiode_nohinj_dn9;
        *var_fn440_calc_ig__igindiode_nohinj_vgsat_slot = var_fn440_calc_ig__igindiode_nohinj_vgsat;
        *var_fn440_calc_ig__igindiode_nohinj_vgsat_dn4_slot = var_fn440_calc_ig__igindiode_nohinj_vgsat_dn4;
        *var_fn440_calc_ig__isdiodeout_slot = var_fn440_calc_ig__isdiodeout;
        *var_fn440_calc_ig__isdiodeout_dn4_slot = var_fn440_calc_ig__isdiodeout_dn4;
        *var_fn440_calc_ig__pg_paramin_hinj_slot = var_fn440_calc_ig__pg_paramin_hinj;
        *var_fn440_calc_ig__t0_slot = var_fn440_calc_ig__t0;
        *var_fn440_calc_ig__t0_dn4_slot = var_fn440_calc_ig__t0_dn4;
        *var_guard441_slot = var_guard441;
    }

    pub(super) fn stamp_transient_block_94(
        p: &Parameters,
        var_fn440_calc_ig__alphagin: f64,
        var_fn440_calc_ig__betarecin: f64,
        var_fn440_calc_ig__expifor: f64,
        var_fn440_calc_ig__expifor_dn4: f64,
        var_fn440_calc_ig__expifor_dn8: f64,
        var_fn440_calc_ig__expifor_dn9: f64,
        var_fn440_calc_ig__expphib: f64,
        var_fn440_calc_ig__expphib_dn4: f64,
        var_fn440_calc_ig__fracin: f64,
        var_fn440_calc_ig__iginbd: f64,
        var_fn440_calc_ig__iginbd_dn4: f64,
        var_fn440_calc_ig__iginbd_dn8: f64,
        var_fn440_calc_ig__iginbd_dn9: f64,
        var_fn440_calc_ig__iginbd_vgsat: f64,
        var_fn440_calc_ig__iginbd_vgsat_dn4: f64,
        var_fn440_calc_ig__irecin: f64,
        var_fn440_calc_ig__isdiodeout: f64,
        var_fn440_calc_ig__isdiodeout_dn4: f64,
        var_fn440_calc_ig__kbdgatein: f64,
        var_fn440_calc_ig__ngf: f64,
        var_fn440_calc_ig__pg_paramin: f64,
        var_fn440_calc_ig__pgsrecin: f64,
        var_fn440_calc_ig__phitin: f64,
        var_fn440_calc_ig__phitin_dn4: f64,
        var_fn440_calc_ig__t0: f64,
        var_fn440_calc_ig__t0_dn4: f64,
        var_fn440_calc_ig__tfacdiodein: f64,
        var_fn440_calc_ig__tfacdiodein_dn4: f64,
        var_fn440_calc_ig__type: f64,
        var_fn440_calc_ig__vgin: f64,
        var_fn440_calc_ig__vgin_dn8: f64,
        var_fn440_calc_ig__vgin_dn9: f64,
        var_fn440_calc_ig__vgsatin: f64,
        var_fn440_calc_ig__vgsatqin: f64,
        var_fn440_calc_ig__w: f64,
        var_guard417: f64,
        var_guard439: f64,
        var_guard441: f64,
        var_fn440_calc_ig__alpha2_phit_slot: &mut f64,
        var_fn440_calc_ig__alpha2_phit_dn4_slot: &mut f64,
        var_fn440_calc_ig__expffvarg_slot: &mut f64,
        var_fn440_calc_ig__expffvarg_dn4_slot: &mut f64,
        var_fn440_calc_ig__expffvarg_dn8_slot: &mut f64,
        var_fn440_calc_ig__expffvarg_dn9_slot: &mut f64,
        var_fn440_calc_ig__expifor_hinj_slot: &mut f64,
        var_fn440_calc_ig__expifor_hinj_dn4_slot: &mut f64,
        var_fn440_calc_ig__expifor_hinj_dn8_slot: &mut f64,
        var_fn440_calc_ig__expifor_hinj_dn9_slot: &mut f64,
        var_fn440_calc_ig__expifor_hinj_vgsat_slot: &mut f64,
        var_fn440_calc_ig__expifor_hinj_vgsat_dn4_slot: &mut f64,
        var_fn440_calc_ig__expifor_nohinj_vgsat_slot: &mut f64,
        var_fn440_calc_ig__expifor_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn440_calc_ig__expiforarg_hinj_slot: &mut f64,
        var_fn440_calc_ig__expiforarg_hinj_dn4_slot: &mut f64,
        var_fn440_calc_ig__expiforarg_hinj_dn8_slot: &mut f64,
        var_fn440_calc_ig__expiforarg_hinj_dn9_slot: &mut f64,
        var_fn440_calc_ig__expiforarg_hinj_vgsat_slot: &mut f64,
        var_fn440_calc_ig__expiforarg_hinj_vgsat_dn4_slot: &mut f64,
        var_fn440_calc_ig__expiforarg_nohinj_vgsat_slot: &mut f64,
        var_fn440_calc_ig__expiforarg_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn440_calc_ig__expirev_slot: &mut f64,
        var_fn440_calc_ig__expirev_dn4_slot: &mut f64,
        var_fn440_calc_ig__expirev_dn8_slot: &mut f64,
        var_fn440_calc_ig__expirev_dn9_slot: &mut f64,
        var_fn440_calc_ig__expirevarg_slot: &mut f64,
        var_fn440_calc_ig__expirevarg_dn4_slot: &mut f64,
        var_fn440_calc_ig__expirevarg_dn8_slot: &mut f64,
        var_fn440_calc_ig__expirevarg_dn9_slot: &mut f64,
        var_fn440_calc_ig__ffvgin_slot: &mut f64,
        var_fn440_calc_ig__ffvgin_dn4_slot: &mut f64,
        var_fn440_calc_ig__ffvgin_dn8_slot: &mut f64,
        var_fn440_calc_ig__ffvgin_dn9_slot: &mut f64,
        var_fn440_calc_ig__frecgin_slot: &mut f64,
        var_fn440_calc_ig__frecgin_dn8_slot: &mut f64,
        var_fn440_calc_ig__frecgin_dn9_slot: &mut f64,
        var_fn440_calc_ig__igindiode_slot: &mut f64,
        var_fn440_calc_ig__igindiode_dn4_slot: &mut f64,
        var_fn440_calc_ig__igindiode_dn8_slot: &mut f64,
        var_fn440_calc_ig__igindiode_dn9_slot: &mut f64,
        var_fn440_calc_ig__igindiode_hinj_slot: &mut f64,
        var_fn440_calc_ig__igindiode_hinj_dn4_slot: &mut f64,
        var_fn440_calc_ig__igindiode_hinj_dn8_slot: &mut f64,
        var_fn440_calc_ig__igindiode_hinj_dn9_slot: &mut f64,
        var_fn440_calc_ig__igindiode_hinj_pre_slot: &mut f64,
        var_fn440_calc_ig__igindiode_hinj_pre_dn4_slot: &mut f64,
        var_fn440_calc_ig__igindiode_hinj_vgsat_slot: &mut f64,
        var_fn440_calc_ig__igindiode_hinj_vgsat_dn4_slot: &mut f64,
        var_fn440_calc_ig__igindiode_nohinj_slot: &mut f64,
        var_fn440_calc_ig__igindiode_nohinj_dn4_slot: &mut f64,
        var_fn440_calc_ig__igindiode_nohinj_dn8_slot: &mut f64,
        var_fn440_calc_ig__igindiode_nohinj_dn9_slot: &mut f64,
        var_fn440_calc_ig__igindiode_nohinj_vgsat_slot: &mut f64,
        var_fn440_calc_ig__igindiode_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn440_calc_ig__iginrec_slot: &mut f64,
        var_fn440_calc_ig__iginrec_dn4_slot: &mut f64,
        var_fn440_calc_ig__iginrec_dn8_slot: &mut f64,
        var_fn440_calc_ig__iginrec_dn9_slot: &mut f64,
        var_fn440_calc_ig__igout_slot: &mut f64,
        var_fn440_calc_ig__igout_dn4_slot: &mut f64,
        var_fn440_calc_ig__igout_dn8_slot: &mut f64,
        var_fn440_calc_ig__igout_dn9_slot: &mut f64,
        var_fn440_calc_ig__isrecout_slot: &mut f64,
        var_fn440_calc_ig__isrecout_dn4_slot: &mut f64,
        var_fn440_calc_ig__pg_paramin_hinj_slot: &mut f64,
        var_fn440_calc_ig__return_slot: &mut f64,
        var_fn440_calc_ig__return_dn4_slot: &mut f64,
        var_fn440_calc_ig__return_dn8_slot: &mut f64,
        var_fn440_calc_ig__return_dn9_slot: &mut f64,
        var_guard442_slot: &mut f64,
        var_guard443_slot: &mut f64,
        var_guard444_slot: &mut f64,
    ) {
        let mut var_fn440_calc_ig__alpha2_phit: f64 = *var_fn440_calc_ig__alpha2_phit_slot;
        let mut var_fn440_calc_ig__alpha2_phit_dn4: f64 = *var_fn440_calc_ig__alpha2_phit_dn4_slot;
        let mut var_fn440_calc_ig__expffvarg: f64 = *var_fn440_calc_ig__expffvarg_slot;
        let mut var_fn440_calc_ig__expffvarg_dn4: f64 = *var_fn440_calc_ig__expffvarg_dn4_slot;
        let mut var_fn440_calc_ig__expffvarg_dn8: f64 = *var_fn440_calc_ig__expffvarg_dn8_slot;
        let mut var_fn440_calc_ig__expffvarg_dn9: f64 = *var_fn440_calc_ig__expffvarg_dn9_slot;
        let mut var_fn440_calc_ig__expifor_hinj: f64 = *var_fn440_calc_ig__expifor_hinj_slot;
        let mut var_fn440_calc_ig__expifor_hinj_dn4: f64 = *var_fn440_calc_ig__expifor_hinj_dn4_slot;
        let mut var_fn440_calc_ig__expifor_hinj_dn8: f64 = *var_fn440_calc_ig__expifor_hinj_dn8_slot;
        let mut var_fn440_calc_ig__expifor_hinj_dn9: f64 = *var_fn440_calc_ig__expifor_hinj_dn9_slot;
        let mut var_fn440_calc_ig__expifor_hinj_vgsat: f64 = *var_fn440_calc_ig__expifor_hinj_vgsat_slot;
        let mut var_fn440_calc_ig__expifor_hinj_vgsat_dn4: f64 = *var_fn440_calc_ig__expifor_hinj_vgsat_dn4_slot;
        let mut var_fn440_calc_ig__expifor_nohinj_vgsat: f64 = *var_fn440_calc_ig__expifor_nohinj_vgsat_slot;
        let mut var_fn440_calc_ig__expifor_nohinj_vgsat_dn4: f64 = *var_fn440_calc_ig__expifor_nohinj_vgsat_dn4_slot;
        let mut var_fn440_calc_ig__expiforarg_hinj: f64 = *var_fn440_calc_ig__expiforarg_hinj_slot;
        let mut var_fn440_calc_ig__expiforarg_hinj_dn4: f64 = *var_fn440_calc_ig__expiforarg_hinj_dn4_slot;
        let mut var_fn440_calc_ig__expiforarg_hinj_dn8: f64 = *var_fn440_calc_ig__expiforarg_hinj_dn8_slot;
        let mut var_fn440_calc_ig__expiforarg_hinj_dn9: f64 = *var_fn440_calc_ig__expiforarg_hinj_dn9_slot;
        let mut var_fn440_calc_ig__expiforarg_hinj_vgsat: f64 = *var_fn440_calc_ig__expiforarg_hinj_vgsat_slot;
        let mut var_fn440_calc_ig__expiforarg_hinj_vgsat_dn4: f64 = *var_fn440_calc_ig__expiforarg_hinj_vgsat_dn4_slot;
        let mut var_fn440_calc_ig__expiforarg_nohinj_vgsat: f64 = *var_fn440_calc_ig__expiforarg_nohinj_vgsat_slot;
        let mut var_fn440_calc_ig__expiforarg_nohinj_vgsat_dn4: f64 = *var_fn440_calc_ig__expiforarg_nohinj_vgsat_dn4_slot;
        let mut var_fn440_calc_ig__expirev: f64 = *var_fn440_calc_ig__expirev_slot;
        let mut var_fn440_calc_ig__expirev_dn4: f64 = *var_fn440_calc_ig__expirev_dn4_slot;
        let mut var_fn440_calc_ig__expirev_dn8: f64 = *var_fn440_calc_ig__expirev_dn8_slot;
        let mut var_fn440_calc_ig__expirev_dn9: f64 = *var_fn440_calc_ig__expirev_dn9_slot;
        let mut var_fn440_calc_ig__expirevarg: f64 = *var_fn440_calc_ig__expirevarg_slot;
        let mut var_fn440_calc_ig__expirevarg_dn4: f64 = *var_fn440_calc_ig__expirevarg_dn4_slot;
        let mut var_fn440_calc_ig__expirevarg_dn8: f64 = *var_fn440_calc_ig__expirevarg_dn8_slot;
        let mut var_fn440_calc_ig__expirevarg_dn9: f64 = *var_fn440_calc_ig__expirevarg_dn9_slot;
        let mut var_fn440_calc_ig__ffvgin: f64 = *var_fn440_calc_ig__ffvgin_slot;
        let mut var_fn440_calc_ig__ffvgin_dn4: f64 = *var_fn440_calc_ig__ffvgin_dn4_slot;
        let mut var_fn440_calc_ig__ffvgin_dn8: f64 = *var_fn440_calc_ig__ffvgin_dn8_slot;
        let mut var_fn440_calc_ig__ffvgin_dn9: f64 = *var_fn440_calc_ig__ffvgin_dn9_slot;
        let mut var_fn440_calc_ig__frecgin: f64 = *var_fn440_calc_ig__frecgin_slot;
        let mut var_fn440_calc_ig__frecgin_dn8: f64 = *var_fn440_calc_ig__frecgin_dn8_slot;
        let mut var_fn440_calc_ig__frecgin_dn9: f64 = *var_fn440_calc_ig__frecgin_dn9_slot;
        let mut var_fn440_calc_ig__igindiode: f64 = *var_fn440_calc_ig__igindiode_slot;
        let mut var_fn440_calc_ig__igindiode_dn4: f64 = *var_fn440_calc_ig__igindiode_dn4_slot;
        let mut var_fn440_calc_ig__igindiode_dn8: f64 = *var_fn440_calc_ig__igindiode_dn8_slot;
        let mut var_fn440_calc_ig__igindiode_dn9: f64 = *var_fn440_calc_ig__igindiode_dn9_slot;
        let mut var_fn440_calc_ig__igindiode_hinj: f64 = *var_fn440_calc_ig__igindiode_hinj_slot;
        let mut var_fn440_calc_ig__igindiode_hinj_dn4: f64 = *var_fn440_calc_ig__igindiode_hinj_dn4_slot;
        let mut var_fn440_calc_ig__igindiode_hinj_dn8: f64 = *var_fn440_calc_ig__igindiode_hinj_dn8_slot;
        let mut var_fn440_calc_ig__igindiode_hinj_dn9: f64 = *var_fn440_calc_ig__igindiode_hinj_dn9_slot;
        let mut var_fn440_calc_ig__igindiode_hinj_pre: f64 = *var_fn440_calc_ig__igindiode_hinj_pre_slot;
        let mut var_fn440_calc_ig__igindiode_hinj_pre_dn4: f64 = *var_fn440_calc_ig__igindiode_hinj_pre_dn4_slot;
        let mut var_fn440_calc_ig__igindiode_hinj_vgsat: f64 = *var_fn440_calc_ig__igindiode_hinj_vgsat_slot;
        let mut var_fn440_calc_ig__igindiode_hinj_vgsat_dn4: f64 = *var_fn440_calc_ig__igindiode_hinj_vgsat_dn4_slot;
        let mut var_fn440_calc_ig__igindiode_nohinj: f64 = *var_fn440_calc_ig__igindiode_nohinj_slot;
        let mut var_fn440_calc_ig__igindiode_nohinj_dn4: f64 = *var_fn440_calc_ig__igindiode_nohinj_dn4_slot;
        let mut var_fn440_calc_ig__igindiode_nohinj_dn8: f64 = *var_fn440_calc_ig__igindiode_nohinj_dn8_slot;
        let mut var_fn440_calc_ig__igindiode_nohinj_dn9: f64 = *var_fn440_calc_ig__igindiode_nohinj_dn9_slot;
        let mut var_fn440_calc_ig__igindiode_nohinj_vgsat: f64 = *var_fn440_calc_ig__igindiode_nohinj_vgsat_slot;
        let mut var_fn440_calc_ig__igindiode_nohinj_vgsat_dn4: f64 = *var_fn440_calc_ig__igindiode_nohinj_vgsat_dn4_slot;
        let mut var_fn440_calc_ig__iginrec: f64 = *var_fn440_calc_ig__iginrec_slot;
        let mut var_fn440_calc_ig__iginrec_dn4: f64 = *var_fn440_calc_ig__iginrec_dn4_slot;
        let mut var_fn440_calc_ig__iginrec_dn8: f64 = *var_fn440_calc_ig__iginrec_dn8_slot;
        let mut var_fn440_calc_ig__iginrec_dn9: f64 = *var_fn440_calc_ig__iginrec_dn9_slot;
        let mut var_fn440_calc_ig__igout: f64 = *var_fn440_calc_ig__igout_slot;
        let mut var_fn440_calc_ig__igout_dn4: f64 = *var_fn440_calc_ig__igout_dn4_slot;
        let mut var_fn440_calc_ig__igout_dn8: f64 = *var_fn440_calc_ig__igout_dn8_slot;
        let mut var_fn440_calc_ig__igout_dn9: f64 = *var_fn440_calc_ig__igout_dn9_slot;
        let mut var_fn440_calc_ig__isrecout: f64 = *var_fn440_calc_ig__isrecout_slot;
        let mut var_fn440_calc_ig__isrecout_dn4: f64 = *var_fn440_calc_ig__isrecout_dn4_slot;
        let mut var_fn440_calc_ig__pg_paramin_hinj: f64 = *var_fn440_calc_ig__pg_paramin_hinj_slot;
        let mut var_fn440_calc_ig__return: f64 = *var_fn440_calc_ig__return_slot;
        let mut var_fn440_calc_ig__return_dn4: f64 = *var_fn440_calc_ig__return_dn4_slot;
        let mut var_fn440_calc_ig__return_dn8: f64 = *var_fn440_calc_ig__return_dn8_slot;
        let mut var_fn440_calc_ig__return_dn9: f64 = *var_fn440_calc_ig__return_dn9_slot;
        let mut var_guard442: f64 = *var_guard442_slot;
        let mut var_guard443: f64 = *var_guard443_slot;
        let mut var_guard444: f64 = *var_guard444_slot;

        let (assign37910_e35191, assign37910_e35191_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard441 == 0.0)) {
        let assign37910_e35185: f64 = (var_fn440_calc_ig__pg_paramin / var_fn440_calc_ig__phitin);
        let assign37910_e35187: f64 = (assign37910_e35185 * var_fn440_calc_ig__vgsatin);
        let assign37910_e35189: f64 = (assign37910_e35187 + var_fn440_calc_ig__expphib);
        (assign37910_e35189, (((-((var_fn440_calc_ig__pg_paramin * var_fn440_calc_ig__phitin_dn4) / (var_fn440_calc_ig__phitin * var_fn440_calc_ig__phitin))) * var_fn440_calc_ig__vgsatin) + var_fn440_calc_ig__expphib_dn4),)
    } else {
        (var_fn440_calc_ig__expiforarg_nohinj_vgsat, var_fn440_calc_ig__expiforarg_nohinj_vgsat_dn4,)
    }
};
        var_fn440_calc_ig__expiforarg_nohinj_vgsat = assign37910_e35191;
        var_fn440_calc_ig__expiforarg_nohinj_vgsat_dn4 = assign37910_e35191_d_n4;

        let (assign37920_e35238, assign37920_e35238_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard441 == 0.0)) {
        let assign37920_e35204: f64 = (-50.0);
        let (assign37920_e35236, assign37920_e35236_d_n4,) = {
            if ((!(var_fn440_calc_ig__expiforarg_nohinj_vgsat > 50.0)) && (!(var_fn440_calc_ig__expiforarg_nohinj_vgsat < assign37920_e35204))) {
                let assign37920_e35209: f64 = (var_fn440_calc_ig__expiforarg_nohinj_vgsat).exp();
                (assign37920_e35209, (assign37920_e35209 * var_fn440_calc_ig__expiforarg_nohinj_vgsat_dn4),)
            } else {
                let assign37920_e35216: f64 = (-50.0);
                let (assign37920_e35235, assign37920_e35235_d_n4,) = {
                    if ((!(var_fn440_calc_ig__expiforarg_nohinj_vgsat > 50.0)) && (var_fn440_calc_ig__expiforarg_nohinj_vgsat < assign37920_e35216)) {
                        let assign37920_e35220: f64 = (-50.0);
                        let assign37920_e35221: f64 = (assign37920_e35220).exp();
                        (assign37920_e35221, 0.0,)
                    } else {
                        let (assign37920_e35234, assign37920_e35234_d_n4,) = {
                            if (var_fn440_calc_ig__expiforarg_nohinj_vgsat > 50.0) {
                                let assign37920_e35226: f64 = (50.0_f64).exp();
                                let assign37920_e35230: f64 = (var_fn440_calc_ig__expiforarg_nohinj_vgsat - 50.0);
                                let assign37920_e35231: f64 = (1.0 + assign37920_e35230);
                                let assign37920_e35232: f64 = (assign37920_e35226 * assign37920_e35231);
                                (assign37920_e35232, (assign37920_e35226 * var_fn440_calc_ig__expiforarg_nohinj_vgsat_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign37920_e35234, assign37920_e35234_d_n4,)
                    }
                };
                (assign37920_e35235, assign37920_e35235_d_n4,)
            }
        };
        (assign37920_e35236, assign37920_e35236_d_n4,)
    } else {
        (var_fn440_calc_ig__expifor_nohinj_vgsat, var_fn440_calc_ig__expifor_nohinj_vgsat_dn4,)
    }
};
        var_fn440_calc_ig__expifor_nohinj_vgsat = assign37920_e35238;
        var_fn440_calc_ig__expifor_nohinj_vgsat_dn4 = assign37920_e35238_d_n4;

        let (assign37930_e35253, assign37930_e35253_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard441 == 0.0)) {
        let assign37930_e35248: f64 = (var_fn440_calc_ig__kbdgatein * var_fn440_calc_ig__iginbd_vgsat);
        let assign37930_e35249: f64 = (var_fn440_calc_ig__expifor_nohinj_vgsat - assign37930_e35248);
        let assign37930_e35251: f64 = (assign37930_e35249 - var_fn440_calc_ig__t0);
        (assign37930_e35251, ((var_fn440_calc_ig__expifor_nohinj_vgsat_dn4 - (var_fn440_calc_ig__kbdgatein * var_fn440_calc_ig__iginbd_vgsat_dn4)) - var_fn440_calc_ig__t0_dn4),)
    } else {
        (var_fn440_calc_ig__igindiode_nohinj_vgsat, var_fn440_calc_ig__igindiode_nohinj_vgsat_dn4,)
    }
};
        var_fn440_calc_ig__igindiode_nohinj_vgsat = assign37930_e35253;
        var_fn440_calc_ig__igindiode_nohinj_vgsat_dn4 = assign37930_e35253_d_n4;

        let (assign37940_e35270, assign37940_e35270_d_n4, assign37940_e35270_d_n8, assign37940_e35270_d_n9,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard441 == 0.0)) {
        let assign37940_e35264: f64 = (var_fn440_calc_ig__kbdgatein * var_fn440_calc_ig__iginbd);
        let assign37940_e35265: f64 = (var_fn440_calc_ig__expifor - assign37940_e35264);
        let assign37940_e35267: f64 = (assign37940_e35265 - var_fn440_calc_ig__t0);
        let assign37940_e35268: f64 = (var_fn440_calc_ig__isdiodeout * assign37940_e35267);
        (assign37940_e35268, ((var_fn440_calc_ig__isdiodeout_dn4 * assign37940_e35267) + (var_fn440_calc_ig__isdiodeout * ((var_fn440_calc_ig__expifor_dn4 - (var_fn440_calc_ig__kbdgatein * var_fn440_calc_ig__iginbd_dn4)) - var_fn440_calc_ig__t0_dn4))), (var_fn440_calc_ig__isdiodeout * (var_fn440_calc_ig__expifor_dn8 - (var_fn440_calc_ig__kbdgatein * var_fn440_calc_ig__iginbd_dn8))), (var_fn440_calc_ig__isdiodeout * (var_fn440_calc_ig__expifor_dn9 - (var_fn440_calc_ig__kbdgatein * var_fn440_calc_ig__iginbd_dn9))),)
    } else {
        (var_fn440_calc_ig__igindiode_nohinj, var_fn440_calc_ig__igindiode_nohinj_dn4, var_fn440_calc_ig__igindiode_nohinj_dn8, var_fn440_calc_ig__igindiode_nohinj_dn9,)
    }
};
        var_fn440_calc_ig__igindiode_nohinj = assign37940_e35270;
        var_fn440_calc_ig__igindiode_nohinj_dn4 = assign37940_e35270_d_n4;
        var_fn440_calc_ig__igindiode_nohinj_dn8 = assign37940_e35270_d_n8;
        var_fn440_calc_ig__igindiode_nohinj_dn9 = assign37940_e35270_d_n9;

        let assign37950_e35273: f64 = if var_fn440_calc_ig__fracin > 0.0 { 1.0 } else { 0.0 };
        var_guard442 = assign37950_e35273;

        let (assign37960_e35286,) = {
    if ((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard441 == 0.0)) && (var_guard442 != 0.0)) {
        let assign37960_e35284: f64 = (var_fn440_calc_ig__fracin * var_fn440_calc_ig__pg_paramin);
        (assign37960_e35284,)
    } else {
        (var_fn440_calc_ig__pg_paramin_hinj,)
    }
};
        var_fn440_calc_ig__pg_paramin_hinj = assign37960_e35286;

        let (assign37970_e35303, assign37970_e35303_d_n4,) = {
    if ((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard441 == 0.0)) && (var_guard442 != 0.0)) {
        let assign37970_e35297: f64 = (var_fn440_calc_ig__pg_paramin_hinj / var_fn440_calc_ig__phitin);
        let assign37970_e35299: f64 = (assign37970_e35297 * var_fn440_calc_ig__vgsatin);
        let assign37970_e35301: f64 = (assign37970_e35299 + var_fn440_calc_ig__expphib);
        (assign37970_e35301, (((-((var_fn440_calc_ig__pg_paramin_hinj * var_fn440_calc_ig__phitin_dn4) / (var_fn440_calc_ig__phitin * var_fn440_calc_ig__phitin))) * var_fn440_calc_ig__vgsatin) + var_fn440_calc_ig__expphib_dn4),)
    } else {
        (var_fn440_calc_ig__expiforarg_hinj_vgsat, var_fn440_calc_ig__expiforarg_hinj_vgsat_dn4,)
    }
};
        var_fn440_calc_ig__expiforarg_hinj_vgsat = assign37970_e35303;
        var_fn440_calc_ig__expiforarg_hinj_vgsat_dn4 = assign37970_e35303_d_n4;

        let (assign37980_e35352, assign37980_e35352_d_n4,) = {
    if ((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard441 == 0.0)) && (var_guard442 != 0.0)) {
        let assign37980_e35318: f64 = (-50.0);
        let (assign37980_e35350, assign37980_e35350_d_n4,) = {
            if ((!(var_fn440_calc_ig__expiforarg_hinj_vgsat > 50.0)) && (!(var_fn440_calc_ig__expiforarg_hinj_vgsat < assign37980_e35318))) {
                let assign37980_e35323: f64 = (var_fn440_calc_ig__expiforarg_hinj_vgsat).exp();
                (assign37980_e35323, (assign37980_e35323 * var_fn440_calc_ig__expiforarg_hinj_vgsat_dn4),)
            } else {
                let assign37980_e35330: f64 = (-50.0);
                let (assign37980_e35349, assign37980_e35349_d_n4,) = {
                    if ((!(var_fn440_calc_ig__expiforarg_hinj_vgsat > 50.0)) && (var_fn440_calc_ig__expiforarg_hinj_vgsat < assign37980_e35330)) {
                        let assign37980_e35334: f64 = (-50.0);
                        let assign37980_e35335: f64 = (assign37980_e35334).exp();
                        (assign37980_e35335, 0.0,)
                    } else {
                        let (assign37980_e35348, assign37980_e35348_d_n4,) = {
                            if (var_fn440_calc_ig__expiforarg_hinj_vgsat > 50.0) {
                                let assign37980_e35340: f64 = (50.0_f64).exp();
                                let assign37980_e35344: f64 = (var_fn440_calc_ig__expiforarg_hinj_vgsat - 50.0);
                                let assign37980_e35345: f64 = (1.0 + assign37980_e35344);
                                let assign37980_e35346: f64 = (assign37980_e35340 * assign37980_e35345);
                                (assign37980_e35346, (assign37980_e35340 * var_fn440_calc_ig__expiforarg_hinj_vgsat_dn4),)
                            } else {
                                (0.0, 0.0,)
                            }
                        };
                        (assign37980_e35348, assign37980_e35348_d_n4,)
                    }
                };
                (assign37980_e35349, assign37980_e35349_d_n4,)
            }
        };
        (assign37980_e35350, assign37980_e35350_d_n4,)
    } else {
        (var_fn440_calc_ig__expifor_hinj_vgsat, var_fn440_calc_ig__expifor_hinj_vgsat_dn4,)
    }
};
        var_fn440_calc_ig__expifor_hinj_vgsat = assign37980_e35352;
        var_fn440_calc_ig__expifor_hinj_vgsat_dn4 = assign37980_e35352_d_n4;

        let (assign37990_e35369, assign37990_e35369_d_n4,) = {
    if ((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard441 == 0.0)) && (var_guard442 != 0.0)) {
        let assign37990_e35364: f64 = (var_fn440_calc_ig__kbdgatein * var_fn440_calc_ig__iginbd_vgsat);
        let assign37990_e35365: f64 = (var_fn440_calc_ig__expifor_hinj_vgsat - assign37990_e35364);
        let assign37990_e35367: f64 = (assign37990_e35365 - var_fn440_calc_ig__t0);
        (assign37990_e35367, ((var_fn440_calc_ig__expifor_hinj_vgsat_dn4 - (var_fn440_calc_ig__kbdgatein * var_fn440_calc_ig__iginbd_vgsat_dn4)) - var_fn440_calc_ig__t0_dn4),)
    } else {
        (var_fn440_calc_ig__igindiode_hinj_vgsat, var_fn440_calc_ig__igindiode_hinj_vgsat_dn4,)
    }
};
        var_fn440_calc_ig__igindiode_hinj_vgsat = assign37990_e35369;
        var_fn440_calc_ig__igindiode_hinj_vgsat_dn4 = assign37990_e35369_d_n4;

        let (assign38000_e35386, assign38000_e35386_d_n4, assign38000_e35386_d_n8, assign38000_e35386_d_n9,) = {
    if ((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard441 == 0.0)) && (var_guard442 != 0.0)) {
        let assign38000_e35380: f64 = (var_fn440_calc_ig__pg_paramin_hinj / var_fn440_calc_ig__phitin);
        let assign38000_e35382: f64 = (assign38000_e35380 * var_fn440_calc_ig__vgin);
        let assign38000_e35384: f64 = (assign38000_e35382 + var_fn440_calc_ig__expphib);
        (assign38000_e35384, (((-((var_fn440_calc_ig__pg_paramin_hinj * var_fn440_calc_ig__phitin_dn4) / (var_fn440_calc_ig__phitin * var_fn440_calc_ig__phitin))) * var_fn440_calc_ig__vgin) + var_fn440_calc_ig__expphib_dn4), (assign38000_e35380 * var_fn440_calc_ig__vgin_dn8), (assign38000_e35380 * var_fn440_calc_ig__vgin_dn9),)
    } else {
        (var_fn440_calc_ig__expiforarg_hinj, var_fn440_calc_ig__expiforarg_hinj_dn4, var_fn440_calc_ig__expiforarg_hinj_dn8, var_fn440_calc_ig__expiforarg_hinj_dn9,)
    }
};
        var_fn440_calc_ig__expiforarg_hinj = assign38000_e35386;
        var_fn440_calc_ig__expiforarg_hinj_dn4 = assign38000_e35386_d_n4;
        var_fn440_calc_ig__expiforarg_hinj_dn8 = assign38000_e35386_d_n8;
        var_fn440_calc_ig__expiforarg_hinj_dn9 = assign38000_e35386_d_n9;

        let (assign38010_e35435, assign38010_e35435_d_n4, assign38010_e35435_d_n8, assign38010_e35435_d_n9,) = {
    if ((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard441 == 0.0)) && (var_guard442 != 0.0)) {
        let assign38010_e35401: f64 = (-50.0);
        let (assign38010_e35433, assign38010_e35433_d_n4, assign38010_e35433_d_n8, assign38010_e35433_d_n9,) = {
            if ((!(var_fn440_calc_ig__expiforarg_hinj > 50.0)) && (!(var_fn440_calc_ig__expiforarg_hinj < assign38010_e35401))) {
                let assign38010_e35406: f64 = (var_fn440_calc_ig__expiforarg_hinj).exp();
                (assign38010_e35406, (assign38010_e35406 * var_fn440_calc_ig__expiforarg_hinj_dn4), (assign38010_e35406 * var_fn440_calc_ig__expiforarg_hinj_dn8), (assign38010_e35406 * var_fn440_calc_ig__expiforarg_hinj_dn9),)
            } else {
                let assign38010_e35413: f64 = (-50.0);
                let (assign38010_e35432, assign38010_e35432_d_n4, assign38010_e35432_d_n8, assign38010_e35432_d_n9,) = {
                    if ((!(var_fn440_calc_ig__expiforarg_hinj > 50.0)) && (var_fn440_calc_ig__expiforarg_hinj < assign38010_e35413)) {
                        let assign38010_e35417: f64 = (-50.0);
                        let assign38010_e35418: f64 = (assign38010_e35417).exp();
                        (assign38010_e35418, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign38010_e35431, assign38010_e35431_d_n4, assign38010_e35431_d_n8, assign38010_e35431_d_n9,) = {
                            if (var_fn440_calc_ig__expiforarg_hinj > 50.0) {
                                let assign38010_e35423: f64 = (50.0_f64).exp();
                                let assign38010_e35427: f64 = (var_fn440_calc_ig__expiforarg_hinj - 50.0);
                                let assign38010_e35428: f64 = (1.0 + assign38010_e35427);
                                let assign38010_e35429: f64 = (assign38010_e35423 * assign38010_e35428);
                                (assign38010_e35429, (assign38010_e35423 * var_fn440_calc_ig__expiforarg_hinj_dn4), (assign38010_e35423 * var_fn440_calc_ig__expiforarg_hinj_dn8), (assign38010_e35423 * var_fn440_calc_ig__expiforarg_hinj_dn9),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign38010_e35431, assign38010_e35431_d_n4, assign38010_e35431_d_n8, assign38010_e35431_d_n9,)
                    }
                };
                (assign38010_e35432, assign38010_e35432_d_n4, assign38010_e35432_d_n8, assign38010_e35432_d_n9,)
            }
        };
        (assign38010_e35433, assign38010_e35433_d_n4, assign38010_e35433_d_n8, assign38010_e35433_d_n9,)
    } else {
        (var_fn440_calc_ig__expifor_hinj, var_fn440_calc_ig__expifor_hinj_dn4, var_fn440_calc_ig__expifor_hinj_dn8, var_fn440_calc_ig__expifor_hinj_dn9,)
    }
};
        var_fn440_calc_ig__expifor_hinj = assign38010_e35435;
        var_fn440_calc_ig__expifor_hinj_dn4 = assign38010_e35435_d_n4;
        var_fn440_calc_ig__expifor_hinj_dn8 = assign38010_e35435_d_n8;
        var_fn440_calc_ig__expifor_hinj_dn9 = assign38010_e35435_d_n9;

        let (assign38020_e35450, assign38020_e35450_d_n4,) = {
    if ((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard441 == 0.0)) && (var_guard442 != 0.0)) {
        let assign38020_e35446: f64 = (var_fn440_calc_ig__isdiodeout * var_fn440_calc_ig__igindiode_nohinj_vgsat);
        let assign38020_e35448: f64 = (assign38020_e35446 / var_fn440_calc_ig__igindiode_hinj_vgsat);
        (assign38020_e35448, (((((var_fn440_calc_ig__isdiodeout_dn4 * var_fn440_calc_ig__igindiode_nohinj_vgsat) + (var_fn440_calc_ig__isdiodeout * var_fn440_calc_ig__igindiode_nohinj_vgsat_dn4)) * var_fn440_calc_ig__igindiode_hinj_vgsat) - (assign38020_e35446 * var_fn440_calc_ig__igindiode_hinj_vgsat_dn4)) / (var_fn440_calc_ig__igindiode_hinj_vgsat * var_fn440_calc_ig__igindiode_hinj_vgsat)),)
    } else {
        (var_fn440_calc_ig__igindiode_hinj_pre, var_fn440_calc_ig__igindiode_hinj_pre_dn4,)
    }
};
        var_fn440_calc_ig__igindiode_hinj_pre = assign38020_e35450;
        var_fn440_calc_ig__igindiode_hinj_pre_dn4 = assign38020_e35450_d_n4;

        let (assign38030_e35469, assign38030_e35469_d_n4, assign38030_e35469_d_n8, assign38030_e35469_d_n9,) = {
    if ((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard441 == 0.0)) && (var_guard442 != 0.0)) {
        let assign38030_e35463: f64 = (var_fn440_calc_ig__kbdgatein * var_fn440_calc_ig__iginbd);
        let assign38030_e35464: f64 = (var_fn440_calc_ig__expifor_hinj - assign38030_e35463);
        let assign38030_e35466: f64 = (assign38030_e35464 - var_fn440_calc_ig__t0);
        let assign38030_e35467: f64 = (var_fn440_calc_ig__igindiode_hinj_pre * assign38030_e35466);
        (assign38030_e35467, ((var_fn440_calc_ig__igindiode_hinj_pre_dn4 * assign38030_e35466) + (var_fn440_calc_ig__igindiode_hinj_pre * ((var_fn440_calc_ig__expifor_hinj_dn4 - (var_fn440_calc_ig__kbdgatein * var_fn440_calc_ig__iginbd_dn4)) - var_fn440_calc_ig__t0_dn4))), (var_fn440_calc_ig__igindiode_hinj_pre * (var_fn440_calc_ig__expifor_hinj_dn8 - (var_fn440_calc_ig__kbdgatein * var_fn440_calc_ig__iginbd_dn8))), (var_fn440_calc_ig__igindiode_hinj_pre * (var_fn440_calc_ig__expifor_hinj_dn9 - (var_fn440_calc_ig__kbdgatein * var_fn440_calc_ig__iginbd_dn9))),)
    } else {
        (var_fn440_calc_ig__igindiode_hinj, var_fn440_calc_ig__igindiode_hinj_dn4, var_fn440_calc_ig__igindiode_hinj_dn8, var_fn440_calc_ig__igindiode_hinj_dn9,)
    }
};
        var_fn440_calc_ig__igindiode_hinj = assign38030_e35469;
        var_fn440_calc_ig__igindiode_hinj_dn4 = assign38030_e35469_d_n4;
        var_fn440_calc_ig__igindiode_hinj_dn8 = assign38030_e35469_d_n8;
        var_fn440_calc_ig__igindiode_hinj_dn9 = assign38030_e35469_d_n9;

        let (assign38040_e35483, assign38040_e35483_d_n4, assign38040_e35483_d_n8, assign38040_e35483_d_n9,) = {
    if ((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard441 == 0.0)) && (var_guard442 == 0.0)) {
        let assign38040_e35481: f64 = (var_fn440_calc_ig__isdiodeout * var_fn440_calc_ig__igindiode_nohinj_vgsat);
        (assign38040_e35481, ((var_fn440_calc_ig__isdiodeout_dn4 * var_fn440_calc_ig__igindiode_nohinj_vgsat) + (var_fn440_calc_ig__isdiodeout * var_fn440_calc_ig__igindiode_nohinj_vgsat_dn4)), 0.0, 0.0,)
    } else {
        (var_fn440_calc_ig__igindiode_hinj, var_fn440_calc_ig__igindiode_hinj_dn4, var_fn440_calc_ig__igindiode_hinj_dn8, var_fn440_calc_ig__igindiode_hinj_dn9,)
    }
};
        var_fn440_calc_ig__igindiode_hinj = assign38040_e35483;
        var_fn440_calc_ig__igindiode_hinj_dn4 = assign38040_e35483_d_n4;
        var_fn440_calc_ig__igindiode_hinj_dn8 = assign38040_e35483_d_n8;
        var_fn440_calc_ig__igindiode_hinj_dn9 = assign38040_e35483_d_n9;

        let (assign38050_e35496, assign38050_e35496_d_n4,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard441 == 0.0)) {
        let assign38050_e35492: f64 = (var_fn440_calc_ig__alphagin * var_fn440_calc_ig__alphagin);
        let assign38050_e35494: f64 = (assign38050_e35492 * var_fn440_calc_ig__phitin);
        (assign38050_e35494, (assign38050_e35492 * var_fn440_calc_ig__phitin_dn4),)
    } else {
        (var_fn440_calc_ig__alpha2_phit, var_fn440_calc_ig__alpha2_phit_dn4,)
    }
};
        var_fn440_calc_ig__alpha2_phit = assign38050_e35496;
        var_fn440_calc_ig__alpha2_phit_dn4 = assign38050_e35496_d_n4;

        let (assign38060_e35513, assign38060_e35513_d_n4, assign38060_e35513_d_n8, assign38060_e35513_d_n9,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard441 == 0.0)) {
        let assign38060_e35507: f64 = (var_fn440_calc_ig__alpha2_phit / 2.0);
        let assign38060_e35508: f64 = (var_fn440_calc_ig__vgsatin - assign38060_e35507);
        let assign38060_e35509: f64 = (var_fn440_calc_ig__vgin - assign38060_e35508);
        let assign38060_e35511: f64 = (assign38060_e35509 / var_fn440_calc_ig__alpha2_phit);
        (assign38060_e35511, ((((-(-(var_fn440_calc_ig__alpha2_phit_dn4 / 2.0))) * var_fn440_calc_ig__alpha2_phit) - (assign38060_e35509 * var_fn440_calc_ig__alpha2_phit_dn4)) / (var_fn440_calc_ig__alpha2_phit * var_fn440_calc_ig__alpha2_phit)), (var_fn440_calc_ig__vgin_dn8 / var_fn440_calc_ig__alpha2_phit), (var_fn440_calc_ig__vgin_dn9 / var_fn440_calc_ig__alpha2_phit),)
    } else {
        (var_fn440_calc_ig__expffvarg, var_fn440_calc_ig__expffvarg_dn4, var_fn440_calc_ig__expffvarg_dn8, var_fn440_calc_ig__expffvarg_dn9,)
    }
};
        var_fn440_calc_ig__expffvarg = assign38060_e35513;
        var_fn440_calc_ig__expffvarg_dn4 = assign38060_e35513_d_n4;
        var_fn440_calc_ig__expffvarg_dn8 = assign38060_e35513_d_n8;
        var_fn440_calc_ig__expffvarg_dn9 = assign38060_e35513_d_n9;

        let assign38070_e35516: f64 = if var_fn440_calc_ig__expffvarg > 50.0 { 1.0 } else { 0.0 };
        var_guard443 = assign38070_e35516;

        let (assign38080_e35527, assign38080_e35527_d_n4, assign38080_e35527_d_n8, assign38080_e35527_d_n9,) = {
    if ((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard441 == 0.0)) && (var_guard443 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn440_calc_ig__ffvgin, var_fn440_calc_ig__ffvgin_dn4, var_fn440_calc_ig__ffvgin_dn8, var_fn440_calc_ig__ffvgin_dn9,)
    }
};
        var_fn440_calc_ig__ffvgin = assign38080_e35527;
        var_fn440_calc_ig__ffvgin_dn4 = assign38080_e35527_d_n4;
        var_fn440_calc_ig__ffvgin_dn8 = assign38080_e35527_d_n8;
        var_fn440_calc_ig__ffvgin_dn9 = assign38080_e35527_d_n9;

        let assign38090_e35530: f64 = (-50.0);
        let assign38090_e35531: f64 = if var_fn440_calc_ig__expffvarg < assign38090_e35530 { 1.0 } else { 0.0 };
        var_guard444 = assign38090_e35531;

        let (assign38100_e35545, assign38100_e35545_d_n4, assign38100_e35545_d_n8, assign38100_e35545_d_n9,) = {
    if (((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard441 == 0.0)) && (var_guard443 == 0.0)) && (var_guard444 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn440_calc_ig__ffvgin, var_fn440_calc_ig__ffvgin_dn4, var_fn440_calc_ig__ffvgin_dn8, var_fn440_calc_ig__ffvgin_dn9,)
    }
};
        var_fn440_calc_ig__ffvgin = assign38100_e35545;
        var_fn440_calc_ig__ffvgin_dn4 = assign38100_e35545_d_n4;
        var_fn440_calc_ig__ffvgin_dn8 = assign38100_e35545_d_n8;
        var_fn440_calc_ig__ffvgin_dn9 = assign38100_e35545_d_n9;

        let (assign38110_e35565, assign38110_e35565_d_n4, assign38110_e35565_d_n8, assign38110_e35565_d_n9,) = {
    if (((((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard441 == 0.0)) && (var_guard443 == 0.0)) && (var_guard444 == 0.0)) {
        let assign38110_e35561: f64 = (var_fn440_calc_ig__expffvarg).exp();
        let assign38110_e35562: f64 = (1.0 + assign38110_e35561);
        let assign38110_e35563: f64 = (1.0 / assign38110_e35562);
        (assign38110_e35563, (-((assign38110_e35561 * var_fn440_calc_ig__expffvarg_dn4) / (assign38110_e35562 * assign38110_e35562))), (-((assign38110_e35561 * var_fn440_calc_ig__expffvarg_dn8) / (assign38110_e35562 * assign38110_e35562))), (-((assign38110_e35561 * var_fn440_calc_ig__expffvarg_dn9) / (assign38110_e35562 * assign38110_e35562))),)
    } else {
        (var_fn440_calc_ig__ffvgin, var_fn440_calc_ig__ffvgin_dn4, var_fn440_calc_ig__ffvgin_dn8, var_fn440_calc_ig__ffvgin_dn9,)
    }
};
        var_fn440_calc_ig__ffvgin = assign38110_e35565;
        var_fn440_calc_ig__ffvgin_dn4 = assign38110_e35565_d_n4;
        var_fn440_calc_ig__ffvgin_dn8 = assign38110_e35565_d_n8;
        var_fn440_calc_ig__ffvgin_dn9 = assign38110_e35565_d_n9;

        let (assign38120_e35582, assign38120_e35582_d_n4, assign38120_e35582_d_n8, assign38120_e35582_d_n9,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard441 == 0.0)) {
        let assign38120_e35574: f64 = (var_fn440_calc_ig__ffvgin * var_fn440_calc_ig__igindiode_nohinj);
        let assign38120_e35577: f64 = (1.0 - var_fn440_calc_ig__ffvgin);
        let assign38120_e35579: f64 = (assign38120_e35577 * var_fn440_calc_ig__igindiode_hinj);
        let assign38120_e35580: f64 = (assign38120_e35574 + assign38120_e35579);
        (assign38120_e35580, (((var_fn440_calc_ig__ffvgin_dn4 * var_fn440_calc_ig__igindiode_nohinj) + (var_fn440_calc_ig__ffvgin * var_fn440_calc_ig__igindiode_nohinj_dn4)) + (((-var_fn440_calc_ig__ffvgin_dn4) * var_fn440_calc_ig__igindiode_hinj) + (assign38120_e35577 * var_fn440_calc_ig__igindiode_hinj_dn4))), (((var_fn440_calc_ig__ffvgin_dn8 * var_fn440_calc_ig__igindiode_nohinj) + (var_fn440_calc_ig__ffvgin * var_fn440_calc_ig__igindiode_nohinj_dn8)) + (((-var_fn440_calc_ig__ffvgin_dn8) * var_fn440_calc_ig__igindiode_hinj) + (assign38120_e35577 * var_fn440_calc_ig__igindiode_hinj_dn8))), (((var_fn440_calc_ig__ffvgin_dn9 * var_fn440_calc_ig__igindiode_nohinj) + (var_fn440_calc_ig__ffvgin * var_fn440_calc_ig__igindiode_nohinj_dn9)) + (((-var_fn440_calc_ig__ffvgin_dn9) * var_fn440_calc_ig__igindiode_hinj) + (assign38120_e35577 * var_fn440_calc_ig__igindiode_hinj_dn9))),)
    } else {
        (var_fn440_calc_ig__igindiode, var_fn440_calc_ig__igindiode_dn4, var_fn440_calc_ig__igindiode_dn8, var_fn440_calc_ig__igindiode_dn9,)
    }
};
        var_fn440_calc_ig__igindiode = assign38120_e35582;
        var_fn440_calc_ig__igindiode_dn4 = assign38120_e35582_d_n4;
        var_fn440_calc_ig__igindiode_dn8 = assign38120_e35582_d_n8;
        var_fn440_calc_ig__igindiode_dn9 = assign38120_e35582_d_n9;

        let (assign38130_e35630, assign38130_e35630_d_n8, assign38130_e35630_d_n9,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        let assign38130_e35587: f64 = (-var_fn440_calc_ig__vgin);
        let (assign38130_e35620, assign38130_e35620_d_n8, assign38130_e35620_d_n9,) = {
            if (p.p52 != 0.0) {
                let assign38130_e35595: f64 = (var_fn440_calc_ig__vgin / var_fn440_calc_ig__vgsatqin);
                let assign38130_e35598: f64 = (0.001 / p.p53);
                let assign38130_e35601: f64 = (var_fn440_calc_ig__vgin / var_fn440_calc_ig__vgsatqin);
                let assign38130_e35602: f64 = (assign38130_e35598 * assign38130_e35601);
                let assign38130_e35603: f64 = (assign38130_e35602).tanh();
                let assign38130_e35604: f64 = (assign38130_e35595 * assign38130_e35603);
                (assign38130_e35604, (((var_fn440_calc_ig__vgin_dn8 / var_fn440_calc_ig__vgsatqin) * assign38130_e35603) + (assign38130_e35595 * ((assign38130_e35598 * (var_fn440_calc_ig__vgin_dn8 / var_fn440_calc_ig__vgsatqin)) / ((assign38130_e35602).cosh() * (assign38130_e35602).cosh())))), (((var_fn440_calc_ig__vgin_dn9 / var_fn440_calc_ig__vgsatqin) * assign38130_e35603) + (assign38130_e35595 * ((assign38130_e35598 * (var_fn440_calc_ig__vgin_dn9 / var_fn440_calc_ig__vgsatqin)) / ((assign38130_e35602).cosh() * (assign38130_e35602).cosh())))),)
            } else {
                let (assign38130_e35619, assign38130_e35619_d_n8, assign38130_e35619_d_n9,) = {
                    if (p.p52 == 0.0) {
                        let __rspice_inv_cse_0: f64 = 1.0 / var_fn440_calc_ig__vgsatqin;
                        let assign38130_e35610: f64 = (var_fn440_calc_ig__vgin * __rspice_inv_cse_0);
                        let assign38130_e35613: f64 = (var_fn440_calc_ig__vgin * __rspice_inv_cse_0);
                        let assign38130_e35614: f64 = (assign38130_e35610 * assign38130_e35613);
                        let assign38130_e35616: f64 = (assign38130_e35614 + p.p53);
                        let assign38130_e35617: f64 = (assign38130_e35616).sqrt();
                        (assign38130_e35617, ((((var_fn440_calc_ig__vgin_dn8 / var_fn440_calc_ig__vgsatqin) * assign38130_e35613) + (assign38130_e35610 * (var_fn440_calc_ig__vgin_dn8 / var_fn440_calc_ig__vgsatqin))) / (2.0 * assign38130_e35617)), ((((var_fn440_calc_ig__vgin_dn9 / var_fn440_calc_ig__vgsatqin) * assign38130_e35613) + (assign38130_e35610 * (var_fn440_calc_ig__vgin_dn9 / var_fn440_calc_ig__vgsatqin))) / (2.0 * assign38130_e35617)),)
                    } else {
                        (0.0, 0.0, 0.0,)
                    }
                };
                (assign38130_e35619, assign38130_e35619_d_n8, assign38130_e35619_d_n9,)
            }
        };
        let assign38130_e35622: f64 = (assign38130_e35620).powf(var_fn440_calc_ig__betarecin);
        let assign38130_e35623: f64 = (1.0 + assign38130_e35622);
        let assign38130_e35626: f64 = (1.0 / var_fn440_calc_ig__betarecin);
        let assign38130_e35627: f64 = (assign38130_e35623).powf(assign38130_e35626);
        let assign38130_e35628: f64 = (assign38130_e35587 / assign38130_e35627);
        (assign38130_e35628, ((((-var_fn440_calc_ig__vgin_dn8) * assign38130_e35627) - (assign38130_e35587 * if 0.0 == 0.0 && ((assign38130_e35626) as f64).is_finite() && ((assign38130_e35626) as f64).fract() == 0.0 { if assign38130_e35626 == 0.0 { 0.0 } else { (assign38130_e35626 * ((assign38130_e35623).powf(assign38130_e35626 - 1.0) * if 0.0 == 0.0 && ((var_fn440_calc_ig__betarecin) as f64).is_finite() && ((var_fn440_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn440_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn440_calc_ig__betarecin * ((assign38130_e35620).powf(var_fn440_calc_ig__betarecin - 1.0) * assign38130_e35620_d_n8)) } } else { (assign38130_e35622 * (var_fn440_calc_ig__betarecin * (assign38130_e35620_d_n8 / assign38130_e35620))) })) } } else { (assign38130_e35627 * (assign38130_e35626 * (if 0.0 == 0.0 && ((var_fn440_calc_ig__betarecin) as f64).is_finite() && ((var_fn440_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn440_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn440_calc_ig__betarecin * ((assign38130_e35620).powf(var_fn440_calc_ig__betarecin - 1.0) * assign38130_e35620_d_n8)) } } else { (assign38130_e35622 * (var_fn440_calc_ig__betarecin * (assign38130_e35620_d_n8 / assign38130_e35620))) } / assign38130_e35623))) })) / (assign38130_e35627 * assign38130_e35627)), ((((-var_fn440_calc_ig__vgin_dn9) * assign38130_e35627) - (assign38130_e35587 * if 0.0 == 0.0 && ((assign38130_e35626) as f64).is_finite() && ((assign38130_e35626) as f64).fract() == 0.0 { if assign38130_e35626 == 0.0 { 0.0 } else { (assign38130_e35626 * ((assign38130_e35623).powf(assign38130_e35626 - 1.0) * if 0.0 == 0.0 && ((var_fn440_calc_ig__betarecin) as f64).is_finite() && ((var_fn440_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn440_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn440_calc_ig__betarecin * ((assign38130_e35620).powf(var_fn440_calc_ig__betarecin - 1.0) * assign38130_e35620_d_n9)) } } else { (assign38130_e35622 * (var_fn440_calc_ig__betarecin * (assign38130_e35620_d_n9 / assign38130_e35620))) })) } } else { (assign38130_e35627 * (assign38130_e35626 * (if 0.0 == 0.0 && ((var_fn440_calc_ig__betarecin) as f64).is_finite() && ((var_fn440_calc_ig__betarecin) as f64).fract() == 0.0 { if var_fn440_calc_ig__betarecin == 0.0 { 0.0 } else { (var_fn440_calc_ig__betarecin * ((assign38130_e35620).powf(var_fn440_calc_ig__betarecin - 1.0) * assign38130_e35620_d_n9)) } } else { (assign38130_e35622 * (var_fn440_calc_ig__betarecin * (assign38130_e35620_d_n9 / assign38130_e35620))) } / assign38130_e35623))) })) / (assign38130_e35627 * assign38130_e35627)),)
    } else {
        (var_fn440_calc_ig__frecgin, var_fn440_calc_ig__frecgin_dn8, var_fn440_calc_ig__frecgin_dn9,)
    }
};
        var_fn440_calc_ig__frecgin = assign38130_e35630;
        var_fn440_calc_ig__frecgin_dn8 = assign38130_e35630_d_n8;
        var_fn440_calc_ig__frecgin_dn9 = assign38130_e35630_d_n9;

        let (assign38140_e35647, assign38140_e35647_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        let assign38140_e35635: f64 = (-var_fn440_calc_ig__type);
        let assign38140_e35637: f64 = (assign38140_e35635 * var_fn440_calc_ig__w);
        let assign38140_e35639: f64 = (assign38140_e35637 * var_fn440_calc_ig__ngf);
        let assign38140_e35641: f64 = (assign38140_e35639 * var_fn440_calc_ig__irecin);
        let assign38140_e35643: f64 = (assign38140_e35641 * var_fn440_calc_ig__tfacdiodein);
        let assign38140_e35645: f64 = assign38140_e35643;
        (assign38140_e35645, (assign38140_e35641 * var_fn440_calc_ig__tfacdiodein_dn4),)
    } else {
        (var_fn440_calc_ig__isrecout, var_fn440_calc_ig__isrecout_dn4,)
    }
};
        var_fn440_calc_ig__isrecout = assign38140_e35647;
        var_fn440_calc_ig__isrecout_dn4 = assign38140_e35647_d_n4;

        let (assign38150_e35657, assign38150_e35657_d_n4, assign38150_e35657_d_n8, assign38150_e35657_d_n9,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        let assign38150_e35653: f64 = (var_fn440_calc_ig__pgsrecin / var_fn440_calc_ig__phitin);
        let assign38150_e35655: f64 = (assign38150_e35653 * var_fn440_calc_ig__frecgin);
        (assign38150_e35655, ((-((var_fn440_calc_ig__pgsrecin * var_fn440_calc_ig__phitin_dn4) / (var_fn440_calc_ig__phitin * var_fn440_calc_ig__phitin))) * var_fn440_calc_ig__frecgin), (assign38150_e35653 * var_fn440_calc_ig__frecgin_dn8), (assign38150_e35653 * var_fn440_calc_ig__frecgin_dn9),)
    } else {
        (var_fn440_calc_ig__expirevarg, var_fn440_calc_ig__expirevarg_dn4, var_fn440_calc_ig__expirevarg_dn8, var_fn440_calc_ig__expirevarg_dn9,)
    }
};
        var_fn440_calc_ig__expirevarg = assign38150_e35657;
        var_fn440_calc_ig__expirevarg_dn4 = assign38150_e35657_d_n4;
        var_fn440_calc_ig__expirevarg_dn8 = assign38150_e35657_d_n8;
        var_fn440_calc_ig__expirevarg_dn9 = assign38150_e35657_d_n9;

        let (assign38160_e35701, assign38160_e35701_d_n4, assign38160_e35701_d_n8, assign38160_e35701_d_n9,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        let assign38160_e35667: f64 = (-50.0);
        let (assign38160_e35699, assign38160_e35699_d_n4, assign38160_e35699_d_n8, assign38160_e35699_d_n9,) = {
            if ((!(var_fn440_calc_ig__expirevarg > 50.0)) && (!(var_fn440_calc_ig__expirevarg < assign38160_e35667))) {
                let assign38160_e35672: f64 = (var_fn440_calc_ig__expirevarg).exp();
                (assign38160_e35672, (assign38160_e35672 * var_fn440_calc_ig__expirevarg_dn4), (assign38160_e35672 * var_fn440_calc_ig__expirevarg_dn8), (assign38160_e35672 * var_fn440_calc_ig__expirevarg_dn9),)
            } else {
                let assign38160_e35679: f64 = (-50.0);
                let (assign38160_e35698, assign38160_e35698_d_n4, assign38160_e35698_d_n8, assign38160_e35698_d_n9,) = {
                    if ((!(var_fn440_calc_ig__expirevarg > 50.0)) && (var_fn440_calc_ig__expirevarg < assign38160_e35679)) {
                        let assign38160_e35683: f64 = (-50.0);
                        let assign38160_e35684: f64 = (assign38160_e35683).exp();
                        (assign38160_e35684, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign38160_e35697, assign38160_e35697_d_n4, assign38160_e35697_d_n8, assign38160_e35697_d_n9,) = {
                            if (var_fn440_calc_ig__expirevarg > 50.0) {
                                let assign38160_e35689: f64 = (50.0_f64).exp();
                                let assign38160_e35693: f64 = (var_fn440_calc_ig__expirevarg - 50.0);
                                let assign38160_e35694: f64 = (1.0 + assign38160_e35693);
                                let assign38160_e35695: f64 = (assign38160_e35689 * assign38160_e35694);
                                (assign38160_e35695, (assign38160_e35689 * var_fn440_calc_ig__expirevarg_dn4), (assign38160_e35689 * var_fn440_calc_ig__expirevarg_dn8), (assign38160_e35689 * var_fn440_calc_ig__expirevarg_dn9),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign38160_e35697, assign38160_e35697_d_n4, assign38160_e35697_d_n8, assign38160_e35697_d_n9,)
                    }
                };
                (assign38160_e35698, assign38160_e35698_d_n4, assign38160_e35698_d_n8, assign38160_e35698_d_n9,)
            }
        };
        (assign38160_e35699, assign38160_e35699_d_n4, assign38160_e35699_d_n8, assign38160_e35699_d_n9,)
    } else {
        (var_fn440_calc_ig__expirev, var_fn440_calc_ig__expirev_dn4, var_fn440_calc_ig__expirev_dn8, var_fn440_calc_ig__expirev_dn9,)
    }
};
        var_fn440_calc_ig__expirev = assign38160_e35701;
        var_fn440_calc_ig__expirev_dn4 = assign38160_e35701_d_n4;
        var_fn440_calc_ig__expirev_dn8 = assign38160_e35701_d_n8;
        var_fn440_calc_ig__expirev_dn9 = assign38160_e35701_d_n9;

        let (assign38170_e35711, assign38170_e35711_d_n4, assign38170_e35711_d_n8, assign38170_e35711_d_n9,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        let assign38170_e35708: f64 = (var_fn440_calc_ig__expirev - 1.0);
        let assign38170_e35709: f64 = (var_fn440_calc_ig__isrecout * assign38170_e35708);
        (assign38170_e35709, ((var_fn440_calc_ig__isrecout_dn4 * assign38170_e35708) + (var_fn440_calc_ig__isrecout * var_fn440_calc_ig__expirev_dn4)), (var_fn440_calc_ig__isrecout * var_fn440_calc_ig__expirev_dn8), (var_fn440_calc_ig__isrecout * var_fn440_calc_ig__expirev_dn9),)
    } else {
        (var_fn440_calc_ig__iginrec, var_fn440_calc_ig__iginrec_dn4, var_fn440_calc_ig__iginrec_dn8, var_fn440_calc_ig__iginrec_dn9,)
    }
};
        var_fn440_calc_ig__iginrec = assign38170_e35711;
        var_fn440_calc_ig__iginrec_dn4 = assign38170_e35711_d_n4;
        var_fn440_calc_ig__iginrec_dn8 = assign38170_e35711_d_n8;
        var_fn440_calc_ig__iginrec_dn9 = assign38170_e35711_d_n9;

        let (assign38180_e35719, assign38180_e35719_d_n4, assign38180_e35719_d_n8, assign38180_e35719_d_n9,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        let assign38180_e35717: f64 = (var_fn440_calc_ig__igindiode + var_fn440_calc_ig__iginrec);
        (assign38180_e35717, (var_fn440_calc_ig__igindiode_dn4 + var_fn440_calc_ig__iginrec_dn4), (var_fn440_calc_ig__igindiode_dn8 + var_fn440_calc_ig__iginrec_dn8), (var_fn440_calc_ig__igindiode_dn9 + var_fn440_calc_ig__iginrec_dn9),)
    } else {
        (var_fn440_calc_ig__igout, var_fn440_calc_ig__igout_dn4, var_fn440_calc_ig__igout_dn8, var_fn440_calc_ig__igout_dn9,)
    }
};
        var_fn440_calc_ig__igout = assign38180_e35719;
        var_fn440_calc_ig__igout_dn4 = assign38180_e35719_d_n4;
        var_fn440_calc_ig__igout_dn8 = assign38180_e35719_d_n8;
        var_fn440_calc_ig__igout_dn9 = assign38180_e35719_d_n9;

        let (assign38190_e35725, assign38190_e35725_d_n4, assign38190_e35725_d_n8, assign38190_e35725_d_n9,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (var_fn440_calc_ig__igout, var_fn440_calc_ig__igout_dn4, var_fn440_calc_ig__igout_dn8, var_fn440_calc_ig__igout_dn9,)
    } else {
        (var_fn440_calc_ig__return, var_fn440_calc_ig__return_dn4, var_fn440_calc_ig__return_dn8, var_fn440_calc_ig__return_dn9,)
    }
};
        var_fn440_calc_ig__return = assign38190_e35725;
        var_fn440_calc_ig__return_dn4 = assign38190_e35725_d_n4;
        var_fn440_calc_ig__return_dn8 = assign38190_e35725_d_n8;
        var_fn440_calc_ig__return_dn9 = assign38190_e35725_d_n9;

        *var_fn440_calc_ig__alpha2_phit_slot = var_fn440_calc_ig__alpha2_phit;
        *var_fn440_calc_ig__alpha2_phit_dn4_slot = var_fn440_calc_ig__alpha2_phit_dn4;
        *var_fn440_calc_ig__expffvarg_slot = var_fn440_calc_ig__expffvarg;
        *var_fn440_calc_ig__expffvarg_dn4_slot = var_fn440_calc_ig__expffvarg_dn4;
        *var_fn440_calc_ig__expffvarg_dn8_slot = var_fn440_calc_ig__expffvarg_dn8;
        *var_fn440_calc_ig__expffvarg_dn9_slot = var_fn440_calc_ig__expffvarg_dn9;
        *var_fn440_calc_ig__expifor_hinj_slot = var_fn440_calc_ig__expifor_hinj;
        *var_fn440_calc_ig__expifor_hinj_dn4_slot = var_fn440_calc_ig__expifor_hinj_dn4;
        *var_fn440_calc_ig__expifor_hinj_dn8_slot = var_fn440_calc_ig__expifor_hinj_dn8;
        *var_fn440_calc_ig__expifor_hinj_dn9_slot = var_fn440_calc_ig__expifor_hinj_dn9;
        *var_fn440_calc_ig__expifor_hinj_vgsat_slot = var_fn440_calc_ig__expifor_hinj_vgsat;
        *var_fn440_calc_ig__expifor_hinj_vgsat_dn4_slot = var_fn440_calc_ig__expifor_hinj_vgsat_dn4;
        *var_fn440_calc_ig__expifor_nohinj_vgsat_slot = var_fn440_calc_ig__expifor_nohinj_vgsat;
        *var_fn440_calc_ig__expifor_nohinj_vgsat_dn4_slot = var_fn440_calc_ig__expifor_nohinj_vgsat_dn4;
        *var_fn440_calc_ig__expiforarg_hinj_slot = var_fn440_calc_ig__expiforarg_hinj;
        *var_fn440_calc_ig__expiforarg_hinj_dn4_slot = var_fn440_calc_ig__expiforarg_hinj_dn4;
        *var_fn440_calc_ig__expiforarg_hinj_dn8_slot = var_fn440_calc_ig__expiforarg_hinj_dn8;
        *var_fn440_calc_ig__expiforarg_hinj_dn9_slot = var_fn440_calc_ig__expiforarg_hinj_dn9;
        *var_fn440_calc_ig__expiforarg_hinj_vgsat_slot = var_fn440_calc_ig__expiforarg_hinj_vgsat;
        *var_fn440_calc_ig__expiforarg_hinj_vgsat_dn4_slot = var_fn440_calc_ig__expiforarg_hinj_vgsat_dn4;
        *var_fn440_calc_ig__expiforarg_nohinj_vgsat_slot = var_fn440_calc_ig__expiforarg_nohinj_vgsat;
        *var_fn440_calc_ig__expiforarg_nohinj_vgsat_dn4_slot = var_fn440_calc_ig__expiforarg_nohinj_vgsat_dn4;
        *var_fn440_calc_ig__expirev_slot = var_fn440_calc_ig__expirev;
        *var_fn440_calc_ig__expirev_dn4_slot = var_fn440_calc_ig__expirev_dn4;
        *var_fn440_calc_ig__expirev_dn8_slot = var_fn440_calc_ig__expirev_dn8;
        *var_fn440_calc_ig__expirev_dn9_slot = var_fn440_calc_ig__expirev_dn9;
        *var_fn440_calc_ig__expirevarg_slot = var_fn440_calc_ig__expirevarg;
        *var_fn440_calc_ig__expirevarg_dn4_slot = var_fn440_calc_ig__expirevarg_dn4;
        *var_fn440_calc_ig__expirevarg_dn8_slot = var_fn440_calc_ig__expirevarg_dn8;
        *var_fn440_calc_ig__expirevarg_dn9_slot = var_fn440_calc_ig__expirevarg_dn9;
        *var_fn440_calc_ig__ffvgin_slot = var_fn440_calc_ig__ffvgin;
        *var_fn440_calc_ig__ffvgin_dn4_slot = var_fn440_calc_ig__ffvgin_dn4;
        *var_fn440_calc_ig__ffvgin_dn8_slot = var_fn440_calc_ig__ffvgin_dn8;
        *var_fn440_calc_ig__ffvgin_dn9_slot = var_fn440_calc_ig__ffvgin_dn9;
        *var_fn440_calc_ig__frecgin_slot = var_fn440_calc_ig__frecgin;
        *var_fn440_calc_ig__frecgin_dn8_slot = var_fn440_calc_ig__frecgin_dn8;
        *var_fn440_calc_ig__frecgin_dn9_slot = var_fn440_calc_ig__frecgin_dn9;
        *var_fn440_calc_ig__igindiode_slot = var_fn440_calc_ig__igindiode;
        *var_fn440_calc_ig__igindiode_dn4_slot = var_fn440_calc_ig__igindiode_dn4;
        *var_fn440_calc_ig__igindiode_dn8_slot = var_fn440_calc_ig__igindiode_dn8;
        *var_fn440_calc_ig__igindiode_dn9_slot = var_fn440_calc_ig__igindiode_dn9;
        *var_fn440_calc_ig__igindiode_hinj_slot = var_fn440_calc_ig__igindiode_hinj;
        *var_fn440_calc_ig__igindiode_hinj_dn4_slot = var_fn440_calc_ig__igindiode_hinj_dn4;
        *var_fn440_calc_ig__igindiode_hinj_dn8_slot = var_fn440_calc_ig__igindiode_hinj_dn8;
        *var_fn440_calc_ig__igindiode_hinj_dn9_slot = var_fn440_calc_ig__igindiode_hinj_dn9;
        *var_fn440_calc_ig__igindiode_hinj_pre_slot = var_fn440_calc_ig__igindiode_hinj_pre;
        *var_fn440_calc_ig__igindiode_hinj_pre_dn4_slot = var_fn440_calc_ig__igindiode_hinj_pre_dn4;
        *var_fn440_calc_ig__igindiode_hinj_vgsat_slot = var_fn440_calc_ig__igindiode_hinj_vgsat;
        *var_fn440_calc_ig__igindiode_hinj_vgsat_dn4_slot = var_fn440_calc_ig__igindiode_hinj_vgsat_dn4;
        *var_fn440_calc_ig__igindiode_nohinj_slot = var_fn440_calc_ig__igindiode_nohinj;
        *var_fn440_calc_ig__igindiode_nohinj_dn4_slot = var_fn440_calc_ig__igindiode_nohinj_dn4;
        *var_fn440_calc_ig__igindiode_nohinj_dn8_slot = var_fn440_calc_ig__igindiode_nohinj_dn8;
        *var_fn440_calc_ig__igindiode_nohinj_dn9_slot = var_fn440_calc_ig__igindiode_nohinj_dn9;
        *var_fn440_calc_ig__igindiode_nohinj_vgsat_slot = var_fn440_calc_ig__igindiode_nohinj_vgsat;
        *var_fn440_calc_ig__igindiode_nohinj_vgsat_dn4_slot = var_fn440_calc_ig__igindiode_nohinj_vgsat_dn4;
        *var_fn440_calc_ig__iginrec_slot = var_fn440_calc_ig__iginrec;
        *var_fn440_calc_ig__iginrec_dn4_slot = var_fn440_calc_ig__iginrec_dn4;
        *var_fn440_calc_ig__iginrec_dn8_slot = var_fn440_calc_ig__iginrec_dn8;
        *var_fn440_calc_ig__iginrec_dn9_slot = var_fn440_calc_ig__iginrec_dn9;
        *var_fn440_calc_ig__igout_slot = var_fn440_calc_ig__igout;
        *var_fn440_calc_ig__igout_dn4_slot = var_fn440_calc_ig__igout_dn4;
        *var_fn440_calc_ig__igout_dn8_slot = var_fn440_calc_ig__igout_dn8;
        *var_fn440_calc_ig__igout_dn9_slot = var_fn440_calc_ig__igout_dn9;
        *var_fn440_calc_ig__isrecout_slot = var_fn440_calc_ig__isrecout;
        *var_fn440_calc_ig__isrecout_dn4_slot = var_fn440_calc_ig__isrecout_dn4;
        *var_fn440_calc_ig__pg_paramin_hinj_slot = var_fn440_calc_ig__pg_paramin_hinj;
        *var_fn440_calc_ig__return_slot = var_fn440_calc_ig__return;
        *var_fn440_calc_ig__return_dn4_slot = var_fn440_calc_ig__return_dn4;
        *var_fn440_calc_ig__return_dn8_slot = var_fn440_calc_ig__return_dn8;
        *var_fn440_calc_ig__return_dn9_slot = var_fn440_calc_ig__return_dn9;
        *var_guard442_slot = var_guard442;
        *var_guard443_slot = var_guard443;
        *var_guard444_slot = var_guard444;
    }

    pub(super) fn stamp_transient_block_95(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_fn440_calc_ig__return: f64,
        var_fn440_calc_ig__return_dn4: f64,
        var_fn440_calc_ig__return_dn8: f64,
        var_fn440_calc_ig__return_dn9: f64,
        var_guard417: f64,
        var_guard439: f64,
        var_phit: f64,
        var_phit_dn4: f64,
        var_tfacdiode: f64,
        var_tfacdiode_dn4: f64,
        var_fn445_calc_ig__alpha2_phit_slot: &mut f64,
        var_fn445_calc_ig__alpha2_phit_dn4_slot: &mut f64,
        var_fn445_calc_ig__alphagin_slot: &mut f64,
        var_fn445_calc_ig__betarecin_slot: &mut f64,
        var_fn445_calc_ig__expbd1_slot: &mut f64,
        var_fn445_calc_ig__expbd1_dn4_slot: &mut f64,
        var_fn445_calc_ig__expbd1_dn5_slot: &mut f64,
        var_fn445_calc_ig__expbd1_dn8_slot: &mut f64,
        var_fn445_calc_ig__expbd1_vgsat_slot: &mut f64,
        var_fn445_calc_ig__expbd1_vgsat_dn4_slot: &mut f64,
        var_fn445_calc_ig__expbd2_slot: &mut f64,
        var_fn445_calc_ig__expbd2_dn4_slot: &mut f64,
        var_fn445_calc_ig__expbdarg1_slot: &mut f64,
        var_fn445_calc_ig__expbdarg1_dn4_slot: &mut f64,
        var_fn445_calc_ig__expbdarg1_dn5_slot: &mut f64,
        var_fn445_calc_ig__expbdarg1_dn8_slot: &mut f64,
        var_fn445_calc_ig__expbdarg1_vgsat_slot: &mut f64,
        var_fn445_calc_ig__expbdarg1_vgsat_dn4_slot: &mut f64,
        var_fn445_calc_ig__expbdarg2_slot: &mut f64,
        var_fn445_calc_ig__expbdarg2_dn4_slot: &mut f64,
        var_fn445_calc_ig__expffvarg_slot: &mut f64,
        var_fn445_calc_ig__expffvarg_dn4_slot: &mut f64,
        var_fn445_calc_ig__expffvarg_dn5_slot: &mut f64,
        var_fn445_calc_ig__expffvarg_dn8_slot: &mut f64,
        var_fn445_calc_ig__expifor_slot: &mut f64,
        var_fn445_calc_ig__expifor_dn4_slot: &mut f64,
        var_fn445_calc_ig__expifor_dn5_slot: &mut f64,
        var_fn445_calc_ig__expifor_dn8_slot: &mut f64,
        var_fn445_calc_ig__expifor_nohinj_vgsat_slot: &mut f64,
        var_fn445_calc_ig__expifor_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn445_calc_ig__expiforarg_slot: &mut f64,
        var_fn445_calc_ig__expiforarg_dn4_slot: &mut f64,
        var_fn445_calc_ig__expiforarg_dn5_slot: &mut f64,
        var_fn445_calc_ig__expiforarg_dn8_slot: &mut f64,
        var_fn445_calc_ig__expiforarg_nohinj_vgsat_slot: &mut f64,
        var_fn445_calc_ig__expiforarg_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn445_calc_ig__expirev_slot: &mut f64,
        var_fn445_calc_ig__expirev_dn4_slot: &mut f64,
        var_fn445_calc_ig__expirev_dn5_slot: &mut f64,
        var_fn445_calc_ig__expirev_dn8_slot: &mut f64,
        var_fn445_calc_ig__expirevarg_slot: &mut f64,
        var_fn445_calc_ig__expirevarg_dn4_slot: &mut f64,
        var_fn445_calc_ig__expirevarg_dn5_slot: &mut f64,
        var_fn445_calc_ig__expirevarg_dn8_slot: &mut f64,
        var_fn445_calc_ig__expphib_slot: &mut f64,
        var_fn445_calc_ig__expphib_dn4_slot: &mut f64,
        var_fn445_calc_ig__ffvgin_slot: &mut f64,
        var_fn445_calc_ig__ffvgin_dn4_slot: &mut f64,
        var_fn445_calc_ig__ffvgin_dn5_slot: &mut f64,
        var_fn445_calc_ig__ffvgin_dn8_slot: &mut f64,
        var_fn445_calc_ig__fracin_slot: &mut f64,
        var_fn445_calc_ig__frecgin_slot: &mut f64,
        var_fn445_calc_ig__frecgin_dn5_slot: &mut f64,
        var_fn445_calc_ig__frecgin_dn8_slot: &mut f64,
        var_fn445_calc_ig__iginbd_slot: &mut f64,
        var_fn445_calc_ig__iginbd_dn4_slot: &mut f64,
        var_fn445_calc_ig__iginbd_dn5_slot: &mut f64,
        var_fn445_calc_ig__iginbd_dn8_slot: &mut f64,
        var_fn445_calc_ig__iginbd_vgsat_slot: &mut f64,
        var_fn445_calc_ig__iginbd_vgsat_dn4_slot: &mut f64,
        var_fn445_calc_ig__igindiode_slot: &mut f64,
        var_fn445_calc_ig__igindiode_dn4_slot: &mut f64,
        var_fn445_calc_ig__igindiode_dn5_slot: &mut f64,
        var_fn445_calc_ig__igindiode_dn8_slot: &mut f64,
        var_fn445_calc_ig__igindiode_nohinj_vgsat_slot: &mut f64,
        var_fn445_calc_ig__igindiode_nohinj_vgsat_dn4_slot: &mut f64,
        var_fn445_calc_ig__iginrec_slot: &mut f64,
        var_fn445_calc_ig__iginrec_dn4_slot: &mut f64,
        var_fn445_calc_ig__iginrec_dn5_slot: &mut f64,
        var_fn445_calc_ig__iginrec_dn8_slot: &mut f64,
        var_fn445_calc_ig__igout_slot: &mut f64,
        var_fn445_calc_ig__igout_dn4_slot: &mut f64,
        var_fn445_calc_ig__igout_dn5_slot: &mut f64,
        var_fn445_calc_ig__igout_dn8_slot: &mut f64,
        var_fn445_calc_ig__ijin_slot: &mut f64,
        var_fn445_calc_ig__irecin_slot: &mut f64,
        var_fn445_calc_ig__isdiodeout_slot: &mut f64,
        var_fn445_calc_ig__isdiodeout_dn4_slot: &mut f64,
        var_fn445_calc_ig__isrecout_slot: &mut f64,
        var_fn445_calc_ig__isrecout_dn4_slot: &mut f64,
        var_fn445_calc_ig__kbdgatein_slot: &mut f64,
        var_fn445_calc_ig__ngf_slot: &mut f64,
        var_fn445_calc_ig__pbdgin_slot: &mut f64,
        var_fn445_calc_ig__pg_param1_slot: &mut f64,
        var_fn445_calc_ig__pg_paramin_slot: &mut f64,
        var_fn445_calc_ig__pg_paramin_hinj_slot: &mut f64,
        var_fn445_calc_ig__pgsrecin_slot: &mut f64,
        var_fn445_calc_ig__phitin_slot: &mut f64,
        var_fn445_calc_ig__phitin_dn4_slot: &mut f64,
        var_fn445_calc_ig__return_slot: &mut f64,
        var_fn445_calc_ig__return_dn4_slot: &mut f64,
        var_fn445_calc_ig__return_dn5_slot: &mut f64,
        var_fn445_calc_ig__return_dn8_slot: &mut f64,
        var_fn445_calc_ig__t0_slot: &mut f64,
        var_fn445_calc_ig__t0_dn4_slot: &mut f64,
        var_fn445_calc_ig__tfacdiodein_slot: &mut f64,
        var_fn445_calc_ig__tfacdiodein_dn4_slot: &mut f64,
        var_fn445_calc_ig__type_slot: &mut f64,
        var_fn445_calc_ig__vbdgin_slot: &mut f64,
        var_fn445_calc_ig__vgin_slot: &mut f64,
        var_fn445_calc_ig__vgin_dn5_slot: &mut f64,
        var_fn445_calc_ig__vgin_dn8_slot: &mut f64,
        var_fn445_calc_ig__vgsatin_slot: &mut f64,
        var_fn445_calc_ig__vgsatqin_slot: &mut f64,
        var_fn445_calc_ig__vjg_slot: &mut f64,
        var_fn445_calc_ig__w_slot: &mut f64,
        var_igsidb_slot: &mut f64,
        var_igsidb_dn4_slot: &mut f64,
        var_igsidb_dn8_slot: &mut f64,
        var_igsidb_dn9_slot: &mut f64,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let mut var_fn445_calc_ig__alpha2_phit: f64 = *var_fn445_calc_ig__alpha2_phit_slot;
        let mut var_fn445_calc_ig__alpha2_phit_dn4: f64 = *var_fn445_calc_ig__alpha2_phit_dn4_slot;
        let mut var_fn445_calc_ig__alphagin: f64 = *var_fn445_calc_ig__alphagin_slot;
        let mut var_fn445_calc_ig__betarecin: f64 = *var_fn445_calc_ig__betarecin_slot;
        let mut var_fn445_calc_ig__expbd1: f64 = *var_fn445_calc_ig__expbd1_slot;
        let mut var_fn445_calc_ig__expbd1_dn4: f64 = *var_fn445_calc_ig__expbd1_dn4_slot;
        let mut var_fn445_calc_ig__expbd1_dn5: f64 = *var_fn445_calc_ig__expbd1_dn5_slot;
        let mut var_fn445_calc_ig__expbd1_dn8: f64 = *var_fn445_calc_ig__expbd1_dn8_slot;
        let mut var_fn445_calc_ig__expbd1_vgsat: f64 = *var_fn445_calc_ig__expbd1_vgsat_slot;
        let mut var_fn445_calc_ig__expbd1_vgsat_dn4: f64 = *var_fn445_calc_ig__expbd1_vgsat_dn4_slot;
        let mut var_fn445_calc_ig__expbd2: f64 = *var_fn445_calc_ig__expbd2_slot;
        let mut var_fn445_calc_ig__expbd2_dn4: f64 = *var_fn445_calc_ig__expbd2_dn4_slot;
        let mut var_fn445_calc_ig__expbdarg1: f64 = *var_fn445_calc_ig__expbdarg1_slot;
        let mut var_fn445_calc_ig__expbdarg1_dn4: f64 = *var_fn445_calc_ig__expbdarg1_dn4_slot;
        let mut var_fn445_calc_ig__expbdarg1_dn5: f64 = *var_fn445_calc_ig__expbdarg1_dn5_slot;
        let mut var_fn445_calc_ig__expbdarg1_dn8: f64 = *var_fn445_calc_ig__expbdarg1_dn8_slot;
        let mut var_fn445_calc_ig__expbdarg1_vgsat: f64 = *var_fn445_calc_ig__expbdarg1_vgsat_slot;
        let mut var_fn445_calc_ig__expbdarg1_vgsat_dn4: f64 = *var_fn445_calc_ig__expbdarg1_vgsat_dn4_slot;
        let mut var_fn445_calc_ig__expbdarg2: f64 = *var_fn445_calc_ig__expbdarg2_slot;
        let mut var_fn445_calc_ig__expbdarg2_dn4: f64 = *var_fn445_calc_ig__expbdarg2_dn4_slot;
        let mut var_fn445_calc_ig__expffvarg: f64 = *var_fn445_calc_ig__expffvarg_slot;
        let mut var_fn445_calc_ig__expffvarg_dn4: f64 = *var_fn445_calc_ig__expffvarg_dn4_slot;
        let mut var_fn445_calc_ig__expffvarg_dn5: f64 = *var_fn445_calc_ig__expffvarg_dn5_slot;
        let mut var_fn445_calc_ig__expffvarg_dn8: f64 = *var_fn445_calc_ig__expffvarg_dn8_slot;
        let mut var_fn445_calc_ig__expifor: f64 = *var_fn445_calc_ig__expifor_slot;
        let mut var_fn445_calc_ig__expifor_dn4: f64 = *var_fn445_calc_ig__expifor_dn4_slot;
        let mut var_fn445_calc_ig__expifor_dn5: f64 = *var_fn445_calc_ig__expifor_dn5_slot;
        let mut var_fn445_calc_ig__expifor_dn8: f64 = *var_fn445_calc_ig__expifor_dn8_slot;
        let mut var_fn445_calc_ig__expifor_nohinj_vgsat: f64 = *var_fn445_calc_ig__expifor_nohinj_vgsat_slot;
        let mut var_fn445_calc_ig__expifor_nohinj_vgsat_dn4: f64 = *var_fn445_calc_ig__expifor_nohinj_vgsat_dn4_slot;
        let mut var_fn445_calc_ig__expiforarg: f64 = *var_fn445_calc_ig__expiforarg_slot;
        let mut var_fn445_calc_ig__expiforarg_dn4: f64 = *var_fn445_calc_ig__expiforarg_dn4_slot;
        let mut var_fn445_calc_ig__expiforarg_dn5: f64 = *var_fn445_calc_ig__expiforarg_dn5_slot;
        let mut var_fn445_calc_ig__expiforarg_dn8: f64 = *var_fn445_calc_ig__expiforarg_dn8_slot;
        let mut var_fn445_calc_ig__expiforarg_nohinj_vgsat: f64 = *var_fn445_calc_ig__expiforarg_nohinj_vgsat_slot;
        let mut var_fn445_calc_ig__expiforarg_nohinj_vgsat_dn4: f64 = *var_fn445_calc_ig__expiforarg_nohinj_vgsat_dn4_slot;
        let mut var_fn445_calc_ig__expirev: f64 = *var_fn445_calc_ig__expirev_slot;
        let mut var_fn445_calc_ig__expirev_dn4: f64 = *var_fn445_calc_ig__expirev_dn4_slot;
        let mut var_fn445_calc_ig__expirev_dn5: f64 = *var_fn445_calc_ig__expirev_dn5_slot;
        let mut var_fn445_calc_ig__expirev_dn8: f64 = *var_fn445_calc_ig__expirev_dn8_slot;
        let mut var_fn445_calc_ig__expirevarg: f64 = *var_fn445_calc_ig__expirevarg_slot;
        let mut var_fn445_calc_ig__expirevarg_dn4: f64 = *var_fn445_calc_ig__expirevarg_dn4_slot;
        let mut var_fn445_calc_ig__expirevarg_dn5: f64 = *var_fn445_calc_ig__expirevarg_dn5_slot;
        let mut var_fn445_calc_ig__expirevarg_dn8: f64 = *var_fn445_calc_ig__expirevarg_dn8_slot;
        let mut var_fn445_calc_ig__expphib: f64 = *var_fn445_calc_ig__expphib_slot;
        let mut var_fn445_calc_ig__expphib_dn4: f64 = *var_fn445_calc_ig__expphib_dn4_slot;
        let mut var_fn445_calc_ig__ffvgin: f64 = *var_fn445_calc_ig__ffvgin_slot;
        let mut var_fn445_calc_ig__ffvgin_dn4: f64 = *var_fn445_calc_ig__ffvgin_dn4_slot;
        let mut var_fn445_calc_ig__ffvgin_dn5: f64 = *var_fn445_calc_ig__ffvgin_dn5_slot;
        let mut var_fn445_calc_ig__ffvgin_dn8: f64 = *var_fn445_calc_ig__ffvgin_dn8_slot;
        let mut var_fn445_calc_ig__fracin: f64 = *var_fn445_calc_ig__fracin_slot;
        let mut var_fn445_calc_ig__frecgin: f64 = *var_fn445_calc_ig__frecgin_slot;
        let mut var_fn445_calc_ig__frecgin_dn5: f64 = *var_fn445_calc_ig__frecgin_dn5_slot;
        let mut var_fn445_calc_ig__frecgin_dn8: f64 = *var_fn445_calc_ig__frecgin_dn8_slot;
        let mut var_fn445_calc_ig__iginbd: f64 = *var_fn445_calc_ig__iginbd_slot;
        let mut var_fn445_calc_ig__iginbd_dn4: f64 = *var_fn445_calc_ig__iginbd_dn4_slot;
        let mut var_fn445_calc_ig__iginbd_dn5: f64 = *var_fn445_calc_ig__iginbd_dn5_slot;
        let mut var_fn445_calc_ig__iginbd_dn8: f64 = *var_fn445_calc_ig__iginbd_dn8_slot;
        let mut var_fn445_calc_ig__iginbd_vgsat: f64 = *var_fn445_calc_ig__iginbd_vgsat_slot;
        let mut var_fn445_calc_ig__iginbd_vgsat_dn4: f64 = *var_fn445_calc_ig__iginbd_vgsat_dn4_slot;
        let mut var_fn445_calc_ig__igindiode: f64 = *var_fn445_calc_ig__igindiode_slot;
        let mut var_fn445_calc_ig__igindiode_dn4: f64 = *var_fn445_calc_ig__igindiode_dn4_slot;
        let mut var_fn445_calc_ig__igindiode_dn5: f64 = *var_fn445_calc_ig__igindiode_dn5_slot;
        let mut var_fn445_calc_ig__igindiode_dn8: f64 = *var_fn445_calc_ig__igindiode_dn8_slot;
        let mut var_fn445_calc_ig__igindiode_nohinj_vgsat: f64 = *var_fn445_calc_ig__igindiode_nohinj_vgsat_slot;
        let mut var_fn445_calc_ig__igindiode_nohinj_vgsat_dn4: f64 = *var_fn445_calc_ig__igindiode_nohinj_vgsat_dn4_slot;
        let mut var_fn445_calc_ig__iginrec: f64 = *var_fn445_calc_ig__iginrec_slot;
        let mut var_fn445_calc_ig__iginrec_dn4: f64 = *var_fn445_calc_ig__iginrec_dn4_slot;
        let mut var_fn445_calc_ig__iginrec_dn5: f64 = *var_fn445_calc_ig__iginrec_dn5_slot;
        let mut var_fn445_calc_ig__iginrec_dn8: f64 = *var_fn445_calc_ig__iginrec_dn8_slot;
        let mut var_fn445_calc_ig__igout: f64 = *var_fn445_calc_ig__igout_slot;
        let mut var_fn445_calc_ig__igout_dn4: f64 = *var_fn445_calc_ig__igout_dn4_slot;
        let mut var_fn445_calc_ig__igout_dn5: f64 = *var_fn445_calc_ig__igout_dn5_slot;
        let mut var_fn445_calc_ig__igout_dn8: f64 = *var_fn445_calc_ig__igout_dn8_slot;
        let mut var_fn445_calc_ig__ijin: f64 = *var_fn445_calc_ig__ijin_slot;
        let mut var_fn445_calc_ig__irecin: f64 = *var_fn445_calc_ig__irecin_slot;
        let mut var_fn445_calc_ig__isdiodeout: f64 = *var_fn445_calc_ig__isdiodeout_slot;
        let mut var_fn445_calc_ig__isdiodeout_dn4: f64 = *var_fn445_calc_ig__isdiodeout_dn4_slot;
        let mut var_fn445_calc_ig__isrecout: f64 = *var_fn445_calc_ig__isrecout_slot;
        let mut var_fn445_calc_ig__isrecout_dn4: f64 = *var_fn445_calc_ig__isrecout_dn4_slot;
        let mut var_fn445_calc_ig__kbdgatein: f64 = *var_fn445_calc_ig__kbdgatein_slot;
        let mut var_fn445_calc_ig__ngf: f64 = *var_fn445_calc_ig__ngf_slot;
        let mut var_fn445_calc_ig__pbdgin: f64 = *var_fn445_calc_ig__pbdgin_slot;
        let mut var_fn445_calc_ig__pg_param1: f64 = *var_fn445_calc_ig__pg_param1_slot;
        let mut var_fn445_calc_ig__pg_paramin: f64 = *var_fn445_calc_ig__pg_paramin_slot;
        let mut var_fn445_calc_ig__pg_paramin_hinj: f64 = *var_fn445_calc_ig__pg_paramin_hinj_slot;
        let mut var_fn445_calc_ig__pgsrecin: f64 = *var_fn445_calc_ig__pgsrecin_slot;
        let mut var_fn445_calc_ig__phitin: f64 = *var_fn445_calc_ig__phitin_slot;
        let mut var_fn445_calc_ig__phitin_dn4: f64 = *var_fn445_calc_ig__phitin_dn4_slot;
        let mut var_fn445_calc_ig__return: f64 = *var_fn445_calc_ig__return_slot;
        let mut var_fn445_calc_ig__return_dn4: f64 = *var_fn445_calc_ig__return_dn4_slot;
        let mut var_fn445_calc_ig__return_dn5: f64 = *var_fn445_calc_ig__return_dn5_slot;
        let mut var_fn445_calc_ig__return_dn8: f64 = *var_fn445_calc_ig__return_dn8_slot;
        let mut var_fn445_calc_ig__t0: f64 = *var_fn445_calc_ig__t0_slot;
        let mut var_fn445_calc_ig__t0_dn4: f64 = *var_fn445_calc_ig__t0_dn4_slot;
        let mut var_fn445_calc_ig__tfacdiodein: f64 = *var_fn445_calc_ig__tfacdiodein_slot;
        let mut var_fn445_calc_ig__tfacdiodein_dn4: f64 = *var_fn445_calc_ig__tfacdiodein_dn4_slot;
        let mut var_fn445_calc_ig__type: f64 = *var_fn445_calc_ig__type_slot;
        let mut var_fn445_calc_ig__vbdgin: f64 = *var_fn445_calc_ig__vbdgin_slot;
        let mut var_fn445_calc_ig__vgin: f64 = *var_fn445_calc_ig__vgin_slot;
        let mut var_fn445_calc_ig__vgin_dn5: f64 = *var_fn445_calc_ig__vgin_dn5_slot;
        let mut var_fn445_calc_ig__vgin_dn8: f64 = *var_fn445_calc_ig__vgin_dn8_slot;
        let mut var_fn445_calc_ig__vgsatin: f64 = *var_fn445_calc_ig__vgsatin_slot;
        let mut var_fn445_calc_ig__vgsatqin: f64 = *var_fn445_calc_ig__vgsatqin_slot;
        let mut var_fn445_calc_ig__vjg: f64 = *var_fn445_calc_ig__vjg_slot;
        let mut var_fn445_calc_ig__w: f64 = *var_fn445_calc_ig__w_slot;
        let mut var_igsidb: f64 = *var_igsidb_slot;
        let mut var_igsidb_dn4: f64 = *var_igsidb_dn4_slot;
        let mut var_igsidb_dn8: f64 = *var_igsidb_dn8_slot;
        let mut var_igsidb_dn9: f64 = *var_igsidb_dn9_slot;

        let (assign38220_e35743, assign38220_e35743_d_n4, assign38220_e35743_d_n8, assign38220_e35743_d_n9,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (var_fn440_calc_ig__return, var_fn440_calc_ig__return_dn4, var_fn440_calc_ig__return_dn8, var_fn440_calc_ig__return_dn9,)
    } else {
        (var_igsidb, var_igsidb_dn4, var_igsidb_dn8, var_igsidb_dn9,)
    }
};
        var_igsidb = assign38220_e35743;
        var_igsidb_dn4 = assign38220_e35743_d_n4;
        var_igsidb_dn8 = assign38220_e35743_d_n8;
        var_igsidb_dn9 = assign38220_e35743_d_n9;

        let (assign38230_e35749, assign38230_e35749_d_n4, assign38230_e35749_d_n5, assign38230_e35749_d_n8,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn445_calc_ig__return, var_fn445_calc_ig__return_dn4, var_fn445_calc_ig__return_dn5, var_fn445_calc_ig__return_dn8,)
    }
};
        var_fn445_calc_ig__return = assign38230_e35749;
        var_fn445_calc_ig__return_dn4 = assign38230_e35749_d_n4;
        var_fn445_calc_ig__return_dn5 = assign38230_e35749_d_n5;
        var_fn445_calc_ig__return_dn8 = assign38230_e35749_d_n8;

        let (assign38240_e35755, assign38240_e35755_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn445_calc_ig__isdiodeout, var_fn445_calc_ig__isdiodeout_dn4,)
    }
};
        var_fn445_calc_ig__isdiodeout = assign38240_e35755;
        var_fn445_calc_ig__isdiodeout_dn4 = assign38240_e35755_d_n4;

        let (assign38250_e35761, assign38250_e35761_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn445_calc_ig__isrecout, var_fn445_calc_ig__isrecout_dn4,)
    }
};
        var_fn445_calc_ig__isrecout = assign38250_e35761;
        var_fn445_calc_ig__isrecout_dn4 = assign38250_e35761_d_n4;

        let (assign38260_e35769, assign38260_e35769_d_n5, assign38260_e35769_d_n8,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        let assign38260_e35767: f64 = (p.p6 * (nv8 - nv5));
        (assign38260_e35767, (-p.p6), p.p6,)
    } else {
        (var_fn445_calc_ig__vgin, var_fn445_calc_ig__vgin_dn5, var_fn445_calc_ig__vgin_dn8,)
    }
};
        var_fn445_calc_ig__vgin = assign38260_e35769;
        var_fn445_calc_ig__vgin_dn5 = assign38260_e35769_d_n5;
        var_fn445_calc_ig__vgin_dn8 = assign38260_e35769_d_n8;

        let (assign38270_e35775, assign38270_e35775_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (var_phit, var_phit_dn4,)
    } else {
        (var_fn445_calc_ig__phitin, var_fn445_calc_ig__phitin_dn4,)
    }
};
        var_fn445_calc_ig__phitin = assign38270_e35775;
        var_fn445_calc_ig__phitin_dn4 = assign38270_e35775_d_n4;

        let (assign38280_e35781,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (p.p265,)
    } else {
        (var_fn445_calc_ig__vgsatin,)
    }
};
        var_fn445_calc_ig__vgsatin = assign38280_e35781;

        let (assign38290_e35787,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (p.p267,)
    } else {
        (var_fn445_calc_ig__alphagin,)
    }
};
        var_fn445_calc_ig__alphagin = assign38290_e35787;

        let (assign38300_e35793,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (p.p266,)
    } else {
        (var_fn445_calc_ig__fracin,)
    }
};
        var_fn445_calc_ig__fracin = assign38300_e35793;

        let (assign38310_e35799,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (p.p263,)
    } else {
        (var_fn445_calc_ig__pg_paramin,)
    }
};
        var_fn445_calc_ig__pg_paramin = assign38310_e35799;

        let (assign38320_e35805,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (p.p281,)
    } else {
        (var_fn445_calc_ig__pbdgin,)
    }
};
        var_fn445_calc_ig__pbdgin = assign38320_e35805;

        let (assign38330_e35811,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (p.p280,)
    } else {
        (var_fn445_calc_ig__vbdgin,)
    }
};
        var_fn445_calc_ig__vbdgin = assign38330_e35811;

        let (assign38340_e35817, assign38340_e35817_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (var_tfacdiode, var_tfacdiode_dn4,)
    } else {
        (var_fn445_calc_ig__tfacdiodein, var_fn445_calc_ig__tfacdiodein_dn4,)
    }
};
        var_fn445_calc_ig__tfacdiodein = assign38340_e35817;
        var_fn445_calc_ig__tfacdiodein_dn4 = assign38340_e35817_d_n4;

        let (assign38350_e35823,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (p.p0,)
    } else {
        (var_fn445_calc_ig__w,)
    }
};
        var_fn445_calc_ig__w = assign38350_e35823;

        let (assign38360_e35829,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (p.p2,)
    } else {
        (var_fn445_calc_ig__ngf,)
    }
};
        var_fn445_calc_ig__ngf = assign38360_e35829;

        let (assign38370_e35837,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        let assign38370_e35835: f64 = (p.p255 * p.p264);
        (assign38370_e35835,)
    } else {
        (var_fn445_calc_ig__ijin,)
    }
};
        var_fn445_calc_ig__ijin = assign38370_e35837;

        let (assign38380_e35843,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (p.p279,)
    } else {
        (var_fn445_calc_ig__kbdgatein,)
    }
};
        var_fn445_calc_ig__kbdgatein = assign38380_e35843;

        let (assign38390_e35849,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (p.p274,)
    } else {
        (var_fn445_calc_ig__vgsatqin,)
    }
};
        var_fn445_calc_ig__vgsatqin = assign38390_e35849;

        let (assign38400_e35855,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (p.p275,)
    } else {
        (var_fn445_calc_ig__betarecin,)
    }
};
        var_fn445_calc_ig__betarecin = assign38400_e35855;

        let (assign38410_e35863,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        let assign38410_e35861: f64 = (p.p255 * p.p273);
        (assign38410_e35861,)
    } else {
        (var_fn445_calc_ig__irecin,)
    }
};
        var_fn445_calc_ig__irecin = assign38410_e35863;

        let (assign38420_e35869,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (p.p272,)
    } else {
        (var_fn445_calc_ig__pgsrecin,)
    }
};
        var_fn445_calc_ig__pgsrecin = assign38420_e35869;

        let (assign38430_e35875,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (p.p257,)
    } else {
        (var_fn445_calc_ig__pg_param1,)
    }
};
        var_fn445_calc_ig__pg_param1 = assign38430_e35875;

        let (assign38440_e35881,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (p.p256,)
    } else {
        (var_fn445_calc_ig__vjg,)
    }
};
        var_fn445_calc_ig__vjg = assign38440_e35881;

        let (assign38450_e35887,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (p.p6,)
    } else {
        (var_fn445_calc_ig__type,)
    }
};
        var_fn445_calc_ig__type = assign38450_e35887;

        let (assign38460_e35893, assign38460_e35893_d_n4, assign38460_e35893_d_n5, assign38460_e35893_d_n8,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn445_calc_ig__igout, var_fn445_calc_ig__igout_dn4, var_fn445_calc_ig__igout_dn5, var_fn445_calc_ig__igout_dn8,)
    }
};
        var_fn445_calc_ig__igout = assign38460_e35893;
        var_fn445_calc_ig__igout_dn4 = assign38460_e35893_d_n4;
        var_fn445_calc_ig__igout_dn5 = assign38460_e35893_d_n5;
        var_fn445_calc_ig__igout_dn8 = assign38460_e35893_d_n8;

        let (assign38470_e35899, assign38470_e35899_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn445_calc_ig__alpha2_phit, var_fn445_calc_ig__alpha2_phit_dn4,)
    }
};
        var_fn445_calc_ig__alpha2_phit = assign38470_e35899;
        var_fn445_calc_ig__alpha2_phit_dn4 = assign38470_e35899_d_n4;

        let (assign38480_e35905, assign38480_e35905_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn445_calc_ig__t0, var_fn445_calc_ig__t0_dn4,)
    }
};
        var_fn445_calc_ig__t0 = assign38480_e35905;
        var_fn445_calc_ig__t0_dn4 = assign38480_e35905_d_n4;

        let (assign38490_e35911, assign38490_e35911_d_n4, assign38490_e35911_d_n5, assign38490_e35911_d_n8,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn445_calc_ig__ffvgin, var_fn445_calc_ig__ffvgin_dn4, var_fn445_calc_ig__ffvgin_dn5, var_fn445_calc_ig__ffvgin_dn8,)
    }
};
        var_fn445_calc_ig__ffvgin = assign38490_e35911;
        var_fn445_calc_ig__ffvgin_dn4 = assign38490_e35911_d_n4;
        var_fn445_calc_ig__ffvgin_dn5 = assign38490_e35911_d_n5;
        var_fn445_calc_ig__ffvgin_dn8 = assign38490_e35911_d_n8;

        let (assign38500_e35917, assign38500_e35917_d_n4, assign38500_e35917_d_n5, assign38500_e35917_d_n8,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn445_calc_ig__iginbd, var_fn445_calc_ig__iginbd_dn4, var_fn445_calc_ig__iginbd_dn5, var_fn445_calc_ig__iginbd_dn8,)
    }
};
        var_fn445_calc_ig__iginbd = assign38500_e35917;
        var_fn445_calc_ig__iginbd_dn4 = assign38500_e35917_d_n4;
        var_fn445_calc_ig__iginbd_dn5 = assign38500_e35917_d_n5;
        var_fn445_calc_ig__iginbd_dn8 = assign38500_e35917_d_n8;

        let (assign38510_e35923, assign38510_e35923_d_n4, assign38510_e35923_d_n5, assign38510_e35923_d_n8,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn445_calc_ig__igindiode, var_fn445_calc_ig__igindiode_dn4, var_fn445_calc_ig__igindiode_dn5, var_fn445_calc_ig__igindiode_dn8,)
    }
};
        var_fn445_calc_ig__igindiode = assign38510_e35923;
        var_fn445_calc_ig__igindiode_dn4 = assign38510_e35923_d_n4;
        var_fn445_calc_ig__igindiode_dn5 = assign38510_e35923_d_n5;
        var_fn445_calc_ig__igindiode_dn8 = assign38510_e35923_d_n8;

        let (assign38520_e35929, assign38520_e35929_d_n5, assign38520_e35929_d_n8,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_fn445_calc_ig__frecgin, var_fn445_calc_ig__frecgin_dn5, var_fn445_calc_ig__frecgin_dn8,)
    }
};
        var_fn445_calc_ig__frecgin = assign38520_e35929;
        var_fn445_calc_ig__frecgin_dn5 = assign38520_e35929_d_n5;
        var_fn445_calc_ig__frecgin_dn8 = assign38520_e35929_d_n8;

        let (assign38530_e35935, assign38530_e35935_d_n4, assign38530_e35935_d_n5, assign38530_e35935_d_n8,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn445_calc_ig__iginrec, var_fn445_calc_ig__iginrec_dn4, var_fn445_calc_ig__iginrec_dn5, var_fn445_calc_ig__iginrec_dn8,)
    }
};
        var_fn445_calc_ig__iginrec = assign38530_e35935;
        var_fn445_calc_ig__iginrec_dn4 = assign38530_e35935_d_n4;
        var_fn445_calc_ig__iginrec_dn5 = assign38530_e35935_d_n5;
        var_fn445_calc_ig__iginrec_dn8 = assign38530_e35935_d_n8;

        let (assign38540_e35941, assign38540_e35941_d_n4, assign38540_e35941_d_n5, assign38540_e35941_d_n8,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn445_calc_ig__expbdarg1, var_fn445_calc_ig__expbdarg1_dn4, var_fn445_calc_ig__expbdarg1_dn5, var_fn445_calc_ig__expbdarg1_dn8,)
    }
};
        var_fn445_calc_ig__expbdarg1 = assign38540_e35941;
        var_fn445_calc_ig__expbdarg1_dn4 = assign38540_e35941_d_n4;
        var_fn445_calc_ig__expbdarg1_dn5 = assign38540_e35941_d_n5;
        var_fn445_calc_ig__expbdarg1_dn8 = assign38540_e35941_d_n8;

        let (assign38550_e35947, assign38550_e35947_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn445_calc_ig__expbdarg2, var_fn445_calc_ig__expbdarg2_dn4,)
    }
};
        var_fn445_calc_ig__expbdarg2 = assign38550_e35947;
        var_fn445_calc_ig__expbdarg2_dn4 = assign38550_e35947_d_n4;

        let (assign38560_e35953, assign38560_e35953_d_n4, assign38560_e35953_d_n5, assign38560_e35953_d_n8,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn445_calc_ig__expbd1, var_fn445_calc_ig__expbd1_dn4, var_fn445_calc_ig__expbd1_dn5, var_fn445_calc_ig__expbd1_dn8,)
    }
};
        var_fn445_calc_ig__expbd1 = assign38560_e35953;
        var_fn445_calc_ig__expbd1_dn4 = assign38560_e35953_d_n4;
        var_fn445_calc_ig__expbd1_dn5 = assign38560_e35953_d_n5;
        var_fn445_calc_ig__expbd1_dn8 = assign38560_e35953_d_n8;

        let (assign38570_e35959, assign38570_e35959_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn445_calc_ig__expbd2, var_fn445_calc_ig__expbd2_dn4,)
    }
};
        var_fn445_calc_ig__expbd2 = assign38570_e35959;
        var_fn445_calc_ig__expbd2_dn4 = assign38570_e35959_d_n4;

        let (assign38580_e35965, assign38580_e35965_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn445_calc_ig__expphib, var_fn445_calc_ig__expphib_dn4,)
    }
};
        var_fn445_calc_ig__expphib = assign38580_e35965;
        var_fn445_calc_ig__expphib_dn4 = assign38580_e35965_d_n4;

        let (assign38590_e35971, assign38590_e35971_d_n4, assign38590_e35971_d_n5, assign38590_e35971_d_n8,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn445_calc_ig__expffvarg, var_fn445_calc_ig__expffvarg_dn4, var_fn445_calc_ig__expffvarg_dn5, var_fn445_calc_ig__expffvarg_dn8,)
    }
};
        var_fn445_calc_ig__expffvarg = assign38590_e35971;
        var_fn445_calc_ig__expffvarg_dn4 = assign38590_e35971_d_n4;
        var_fn445_calc_ig__expffvarg_dn5 = assign38590_e35971_d_n5;
        var_fn445_calc_ig__expffvarg_dn8 = assign38590_e35971_d_n8;

        let (assign38600_e35977, assign38600_e35977_d_n4, assign38600_e35977_d_n5, assign38600_e35977_d_n8,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn445_calc_ig__expiforarg, var_fn445_calc_ig__expiforarg_dn4, var_fn445_calc_ig__expiforarg_dn5, var_fn445_calc_ig__expiforarg_dn8,)
    }
};
        var_fn445_calc_ig__expiforarg = assign38600_e35977;
        var_fn445_calc_ig__expiforarg_dn4 = assign38600_e35977_d_n4;
        var_fn445_calc_ig__expiforarg_dn5 = assign38600_e35977_d_n5;
        var_fn445_calc_ig__expiforarg_dn8 = assign38600_e35977_d_n8;

        let (assign38610_e35983, assign38610_e35983_d_n4, assign38610_e35983_d_n5, assign38610_e35983_d_n8,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn445_calc_ig__expifor, var_fn445_calc_ig__expifor_dn4, var_fn445_calc_ig__expifor_dn5, var_fn445_calc_ig__expifor_dn8,)
    }
};
        var_fn445_calc_ig__expifor = assign38610_e35983;
        var_fn445_calc_ig__expifor_dn4 = assign38610_e35983_d_n4;
        var_fn445_calc_ig__expifor_dn5 = assign38610_e35983_d_n5;
        var_fn445_calc_ig__expifor_dn8 = assign38610_e35983_d_n8;

        let (assign38620_e35989, assign38620_e35989_d_n4, assign38620_e35989_d_n5, assign38620_e35989_d_n8,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn445_calc_ig__expirevarg, var_fn445_calc_ig__expirevarg_dn4, var_fn445_calc_ig__expirevarg_dn5, var_fn445_calc_ig__expirevarg_dn8,)
    }
};
        var_fn445_calc_ig__expirevarg = assign38620_e35989;
        var_fn445_calc_ig__expirevarg_dn4 = assign38620_e35989_d_n4;
        var_fn445_calc_ig__expirevarg_dn5 = assign38620_e35989_d_n5;
        var_fn445_calc_ig__expirevarg_dn8 = assign38620_e35989_d_n8;

        let (assign38630_e35995, assign38630_e35995_d_n4, assign38630_e35995_d_n5, assign38630_e35995_d_n8,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fn445_calc_ig__expirev, var_fn445_calc_ig__expirev_dn4, var_fn445_calc_ig__expirev_dn5, var_fn445_calc_ig__expirev_dn8,)
    }
};
        var_fn445_calc_ig__expirev = assign38630_e35995;
        var_fn445_calc_ig__expirev_dn4 = assign38630_e35995_d_n4;
        var_fn445_calc_ig__expirev_dn5 = assign38630_e35995_d_n5;
        var_fn445_calc_ig__expirev_dn8 = assign38630_e35995_d_n8;

        let (assign38640_e36001,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0,)
    } else {
        (var_fn445_calc_ig__pg_paramin_hinj,)
    }
};
        var_fn445_calc_ig__pg_paramin_hinj = assign38640_e36001;

        let (assign38650_e36007, assign38650_e36007_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn445_calc_ig__expbdarg1_vgsat, var_fn445_calc_ig__expbdarg1_vgsat_dn4,)
    }
};
        var_fn445_calc_ig__expbdarg1_vgsat = assign38650_e36007;
        var_fn445_calc_ig__expbdarg1_vgsat_dn4 = assign38650_e36007_d_n4;

        let (assign38660_e36013, assign38660_e36013_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn445_calc_ig__expbd1_vgsat, var_fn445_calc_ig__expbd1_vgsat_dn4,)
    }
};
        var_fn445_calc_ig__expbd1_vgsat = assign38660_e36013;
        var_fn445_calc_ig__expbd1_vgsat_dn4 = assign38660_e36013_d_n4;

        let (assign38670_e36019, assign38670_e36019_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn445_calc_ig__iginbd_vgsat, var_fn445_calc_ig__iginbd_vgsat_dn4,)
    }
};
        var_fn445_calc_ig__iginbd_vgsat = assign38670_e36019;
        var_fn445_calc_ig__iginbd_vgsat_dn4 = assign38670_e36019_d_n4;

        let (assign38680_e36025, assign38680_e36025_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn445_calc_ig__expiforarg_nohinj_vgsat, var_fn445_calc_ig__expiforarg_nohinj_vgsat_dn4,)
    }
};
        var_fn445_calc_ig__expiforarg_nohinj_vgsat = assign38680_e36025;
        var_fn445_calc_ig__expiforarg_nohinj_vgsat_dn4 = assign38680_e36025_d_n4;

        let (assign38690_e36031, assign38690_e36031_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn445_calc_ig__expifor_nohinj_vgsat, var_fn445_calc_ig__expifor_nohinj_vgsat_dn4,)
    }
};
        var_fn445_calc_ig__expifor_nohinj_vgsat = assign38690_e36031;
        var_fn445_calc_ig__expifor_nohinj_vgsat_dn4 = assign38690_e36031_d_n4;

        let (assign38700_e36037, assign38700_e36037_d_n4,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_fn445_calc_ig__igindiode_nohinj_vgsat, var_fn445_calc_ig__igindiode_nohinj_vgsat_dn4,)
    }
};
        var_fn445_calc_ig__igindiode_nohinj_vgsat = assign38700_e36037;
        var_fn445_calc_ig__igindiode_nohinj_vgsat_dn4 = assign38700_e36037_d_n4;

        *var_fn445_calc_ig__alpha2_phit_slot = var_fn445_calc_ig__alpha2_phit;
        *var_fn445_calc_ig__alpha2_phit_dn4_slot = var_fn445_calc_ig__alpha2_phit_dn4;
        *var_fn445_calc_ig__alphagin_slot = var_fn445_calc_ig__alphagin;
        *var_fn445_calc_ig__betarecin_slot = var_fn445_calc_ig__betarecin;
        *var_fn445_calc_ig__expbd1_slot = var_fn445_calc_ig__expbd1;
        *var_fn445_calc_ig__expbd1_dn4_slot = var_fn445_calc_ig__expbd1_dn4;
        *var_fn445_calc_ig__expbd1_dn5_slot = var_fn445_calc_ig__expbd1_dn5;
        *var_fn445_calc_ig__expbd1_dn8_slot = var_fn445_calc_ig__expbd1_dn8;
        *var_fn445_calc_ig__expbd1_vgsat_slot = var_fn445_calc_ig__expbd1_vgsat;
        *var_fn445_calc_ig__expbd1_vgsat_dn4_slot = var_fn445_calc_ig__expbd1_vgsat_dn4;
        *var_fn445_calc_ig__expbd2_slot = var_fn445_calc_ig__expbd2;
        *var_fn445_calc_ig__expbd2_dn4_slot = var_fn445_calc_ig__expbd2_dn4;
        *var_fn445_calc_ig__expbdarg1_slot = var_fn445_calc_ig__expbdarg1;
        *var_fn445_calc_ig__expbdarg1_dn4_slot = var_fn445_calc_ig__expbdarg1_dn4;
        *var_fn445_calc_ig__expbdarg1_dn5_slot = var_fn445_calc_ig__expbdarg1_dn5;
        *var_fn445_calc_ig__expbdarg1_dn8_slot = var_fn445_calc_ig__expbdarg1_dn8;
        *var_fn445_calc_ig__expbdarg1_vgsat_slot = var_fn445_calc_ig__expbdarg1_vgsat;
        *var_fn445_calc_ig__expbdarg1_vgsat_dn4_slot = var_fn445_calc_ig__expbdarg1_vgsat_dn4;
        *var_fn445_calc_ig__expbdarg2_slot = var_fn445_calc_ig__expbdarg2;
        *var_fn445_calc_ig__expbdarg2_dn4_slot = var_fn445_calc_ig__expbdarg2_dn4;
        *var_fn445_calc_ig__expffvarg_slot = var_fn445_calc_ig__expffvarg;
        *var_fn445_calc_ig__expffvarg_dn4_slot = var_fn445_calc_ig__expffvarg_dn4;
        *var_fn445_calc_ig__expffvarg_dn5_slot = var_fn445_calc_ig__expffvarg_dn5;
        *var_fn445_calc_ig__expffvarg_dn8_slot = var_fn445_calc_ig__expffvarg_dn8;
        *var_fn445_calc_ig__expifor_slot = var_fn445_calc_ig__expifor;
        *var_fn445_calc_ig__expifor_dn4_slot = var_fn445_calc_ig__expifor_dn4;
        *var_fn445_calc_ig__expifor_dn5_slot = var_fn445_calc_ig__expifor_dn5;
        *var_fn445_calc_ig__expifor_dn8_slot = var_fn445_calc_ig__expifor_dn8;
        *var_fn445_calc_ig__expifor_nohinj_vgsat_slot = var_fn445_calc_ig__expifor_nohinj_vgsat;
        *var_fn445_calc_ig__expifor_nohinj_vgsat_dn4_slot = var_fn445_calc_ig__expifor_nohinj_vgsat_dn4;
        *var_fn445_calc_ig__expiforarg_slot = var_fn445_calc_ig__expiforarg;
        *var_fn445_calc_ig__expiforarg_dn4_slot = var_fn445_calc_ig__expiforarg_dn4;
        *var_fn445_calc_ig__expiforarg_dn5_slot = var_fn445_calc_ig__expiforarg_dn5;
        *var_fn445_calc_ig__expiforarg_dn8_slot = var_fn445_calc_ig__expiforarg_dn8;
        *var_fn445_calc_ig__expiforarg_nohinj_vgsat_slot = var_fn445_calc_ig__expiforarg_nohinj_vgsat;
        *var_fn445_calc_ig__expiforarg_nohinj_vgsat_dn4_slot = var_fn445_calc_ig__expiforarg_nohinj_vgsat_dn4;
        *var_fn445_calc_ig__expirev_slot = var_fn445_calc_ig__expirev;
        *var_fn445_calc_ig__expirev_dn4_slot = var_fn445_calc_ig__expirev_dn4;
        *var_fn445_calc_ig__expirev_dn5_slot = var_fn445_calc_ig__expirev_dn5;
        *var_fn445_calc_ig__expirev_dn8_slot = var_fn445_calc_ig__expirev_dn8;
        *var_fn445_calc_ig__expirevarg_slot = var_fn445_calc_ig__expirevarg;
        *var_fn445_calc_ig__expirevarg_dn4_slot = var_fn445_calc_ig__expirevarg_dn4;
        *var_fn445_calc_ig__expirevarg_dn5_slot = var_fn445_calc_ig__expirevarg_dn5;
        *var_fn445_calc_ig__expirevarg_dn8_slot = var_fn445_calc_ig__expirevarg_dn8;
        *var_fn445_calc_ig__expphib_slot = var_fn445_calc_ig__expphib;
        *var_fn445_calc_ig__expphib_dn4_slot = var_fn445_calc_ig__expphib_dn4;
        *var_fn445_calc_ig__ffvgin_slot = var_fn445_calc_ig__ffvgin;
        *var_fn445_calc_ig__ffvgin_dn4_slot = var_fn445_calc_ig__ffvgin_dn4;
        *var_fn445_calc_ig__ffvgin_dn5_slot = var_fn445_calc_ig__ffvgin_dn5;
        *var_fn445_calc_ig__ffvgin_dn8_slot = var_fn445_calc_ig__ffvgin_dn8;
        *var_fn445_calc_ig__fracin_slot = var_fn445_calc_ig__fracin;
        *var_fn445_calc_ig__frecgin_slot = var_fn445_calc_ig__frecgin;
        *var_fn445_calc_ig__frecgin_dn5_slot = var_fn445_calc_ig__frecgin_dn5;
        *var_fn445_calc_ig__frecgin_dn8_slot = var_fn445_calc_ig__frecgin_dn8;
        *var_fn445_calc_ig__iginbd_slot = var_fn445_calc_ig__iginbd;
        *var_fn445_calc_ig__iginbd_dn4_slot = var_fn445_calc_ig__iginbd_dn4;
        *var_fn445_calc_ig__iginbd_dn5_slot = var_fn445_calc_ig__iginbd_dn5;
        *var_fn445_calc_ig__iginbd_dn8_slot = var_fn445_calc_ig__iginbd_dn8;
        *var_fn445_calc_ig__iginbd_vgsat_slot = var_fn445_calc_ig__iginbd_vgsat;
        *var_fn445_calc_ig__iginbd_vgsat_dn4_slot = var_fn445_calc_ig__iginbd_vgsat_dn4;
        *var_fn445_calc_ig__igindiode_slot = var_fn445_calc_ig__igindiode;
        *var_fn445_calc_ig__igindiode_dn4_slot = var_fn445_calc_ig__igindiode_dn4;
        *var_fn445_calc_ig__igindiode_dn5_slot = var_fn445_calc_ig__igindiode_dn5;
        *var_fn445_calc_ig__igindiode_dn8_slot = var_fn445_calc_ig__igindiode_dn8;
        *var_fn445_calc_ig__igindiode_nohinj_vgsat_slot = var_fn445_calc_ig__igindiode_nohinj_vgsat;
        *var_fn445_calc_ig__igindiode_nohinj_vgsat_dn4_slot = var_fn445_calc_ig__igindiode_nohinj_vgsat_dn4;
        *var_fn445_calc_ig__iginrec_slot = var_fn445_calc_ig__iginrec;
        *var_fn445_calc_ig__iginrec_dn4_slot = var_fn445_calc_ig__iginrec_dn4;
        *var_fn445_calc_ig__iginrec_dn5_slot = var_fn445_calc_ig__iginrec_dn5;
        *var_fn445_calc_ig__iginrec_dn8_slot = var_fn445_calc_ig__iginrec_dn8;
        *var_fn445_calc_ig__igout_slot = var_fn445_calc_ig__igout;
        *var_fn445_calc_ig__igout_dn4_slot = var_fn445_calc_ig__igout_dn4;
        *var_fn445_calc_ig__igout_dn5_slot = var_fn445_calc_ig__igout_dn5;
        *var_fn445_calc_ig__igout_dn8_slot = var_fn445_calc_ig__igout_dn8;
        *var_fn445_calc_ig__ijin_slot = var_fn445_calc_ig__ijin;
        *var_fn445_calc_ig__irecin_slot = var_fn445_calc_ig__irecin;
        *var_fn445_calc_ig__isdiodeout_slot = var_fn445_calc_ig__isdiodeout;
        *var_fn445_calc_ig__isdiodeout_dn4_slot = var_fn445_calc_ig__isdiodeout_dn4;
        *var_fn445_calc_ig__isrecout_slot = var_fn445_calc_ig__isrecout;
        *var_fn445_calc_ig__isrecout_dn4_slot = var_fn445_calc_ig__isrecout_dn4;
        *var_fn445_calc_ig__kbdgatein_slot = var_fn445_calc_ig__kbdgatein;
        *var_fn445_calc_ig__ngf_slot = var_fn445_calc_ig__ngf;
        *var_fn445_calc_ig__pbdgin_slot = var_fn445_calc_ig__pbdgin;
        *var_fn445_calc_ig__pg_param1_slot = var_fn445_calc_ig__pg_param1;
        *var_fn445_calc_ig__pg_paramin_slot = var_fn445_calc_ig__pg_paramin;
        *var_fn445_calc_ig__pg_paramin_hinj_slot = var_fn445_calc_ig__pg_paramin_hinj;
        *var_fn445_calc_ig__pgsrecin_slot = var_fn445_calc_ig__pgsrecin;
        *var_fn445_calc_ig__phitin_slot = var_fn445_calc_ig__phitin;
        *var_fn445_calc_ig__phitin_dn4_slot = var_fn445_calc_ig__phitin_dn4;
        *var_fn445_calc_ig__return_slot = var_fn445_calc_ig__return;
        *var_fn445_calc_ig__return_dn4_slot = var_fn445_calc_ig__return_dn4;
        *var_fn445_calc_ig__return_dn5_slot = var_fn445_calc_ig__return_dn5;
        *var_fn445_calc_ig__return_dn8_slot = var_fn445_calc_ig__return_dn8;
        *var_fn445_calc_ig__t0_slot = var_fn445_calc_ig__t0;
        *var_fn445_calc_ig__t0_dn4_slot = var_fn445_calc_ig__t0_dn4;
        *var_fn445_calc_ig__tfacdiodein_slot = var_fn445_calc_ig__tfacdiodein;
        *var_fn445_calc_ig__tfacdiodein_dn4_slot = var_fn445_calc_ig__tfacdiodein_dn4;
        *var_fn445_calc_ig__type_slot = var_fn445_calc_ig__type;
        *var_fn445_calc_ig__vbdgin_slot = var_fn445_calc_ig__vbdgin;
        *var_fn445_calc_ig__vgin_slot = var_fn445_calc_ig__vgin;
        *var_fn445_calc_ig__vgin_dn5_slot = var_fn445_calc_ig__vgin_dn5;
        *var_fn445_calc_ig__vgin_dn8_slot = var_fn445_calc_ig__vgin_dn8;
        *var_fn445_calc_ig__vgsatin_slot = var_fn445_calc_ig__vgsatin;
        *var_fn445_calc_ig__vgsatqin_slot = var_fn445_calc_ig__vgsatqin;
        *var_fn445_calc_ig__vjg_slot = var_fn445_calc_ig__vjg;
        *var_fn445_calc_ig__w_slot = var_fn445_calc_ig__w;
        *var_igsidb_slot = var_igsidb;
        *var_igsidb_dn4_slot = var_igsidb_dn4;
        *var_igsidb_dn8_slot = var_igsidb_dn8;
        *var_igsidb_dn9_slot = var_igsidb_dn9;
    }
}
