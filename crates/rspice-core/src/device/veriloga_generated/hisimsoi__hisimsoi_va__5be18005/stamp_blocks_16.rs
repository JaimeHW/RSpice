#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_127(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_guard1153: f64,
        var_ldrifte: f64,
        var_lgle: f64,
        var_rrdrbb: f64,
        var_rrdrbb_dn10: f64,
        var_t3: f64,
        var_t3_dn0: f64,
        var_t3_dn10: f64,
        var_t3_dn11: f64,
        var_t3_dn12: f64,
        var_t3_dn17: f64,
        var_t3_dn2: f64,
        var_t3_dn6: f64,
        var_t3_dn7: f64,
        var_ttemp: f64,
        var_ttemp_dn10: f64,
        var_uc_tnom: f64,
        var_wg: f64,
        var_edri__blk1195_slot: &mut f64,
        var_edri__blk1195_dn0_slot: &mut f64,
        var_edri__blk1195_dn2_slot: &mut f64,
        var_edri__blk1195_dn6_slot: &mut f64,
        var_edri__blk1195_dn7_slot: &mut f64,
        var_edri__blk1195_rv_slot: &mut f64,
        var_guard1177_slot: &mut f64,
        var_guard1177_rv_slot: &mut f64,
        var_guard1178_slot: &mut f64,
        var_guard1178_rv_slot: &mut f64,
        var_guard1181_slot: &mut f64,
        var_guard1181_rv_slot: &mut f64,
        var_guard1201_slot: &mut f64,
        var_guard1201_rv_slot: &mut f64,
        var_ldrifte__blk1191_slot: &mut f64,
        var_ldrifte__blk1191_rv_slot: &mut f64,
        var_mks_rdrmue__blk1185_slot: &mut f64,
        var_mks_rdrmue__blk1185_rv_slot: &mut f64,
        var_mks_rdrvmax__blk1186_slot: &mut f64,
        var_mks_rdrvmax__blk1186_rv_slot: &mut f64,
        var_mu0__blk1193_slot: &mut f64,
        var_mu0__blk1193_dn0_slot: &mut f64,
        var_mu0__blk1193_dn10_slot: &mut f64,
        var_mu0__blk1193_dn11_slot: &mut f64,
        var_mu0__blk1193_dn12_slot: &mut f64,
        var_mu0__blk1193_dn17_slot: &mut f64,
        var_mu0__blk1193_dn2_slot: &mut f64,
        var_mu0__blk1193_dn6_slot: &mut f64,
        var_mu0__blk1193_dn7_slot: &mut f64,
        var_mu0__blk1193_rv_slot: &mut f64,
        var_rdmod_slot: &mut f64,
        var_rdmod_rv_slot: &mut f64,
        var_rdrmuele__blk1182_slot: &mut f64,
        var_rdrmuele__blk1182_rv_slot: &mut f64,
        var_rdrvmaxle__blk1184_slot: &mut f64,
        var_rdrvmaxle__blk1184_rv_slot: &mut f64,
        var_rdrvmaxwe__blk1183_slot: &mut f64,
        var_rdrvmaxwe__blk1183_rv_slot: &mut f64,
        var_rrdrbb__blk1187_slot: &mut f64,
        var_rrdrbb__blk1187_dn10_slot: &mut f64,
        var_rrdrbb__blk1187_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn12_slot: &mut f64,
        var_t0_dn17_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn17_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn12_slot: &mut f64,
        var_t2_dn17_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn0_slot: &mut f64,
        var_t4_dn10_slot: &mut f64,
        var_t4_dn11_slot: &mut f64,
        var_t4_dn12_slot: &mut f64,
        var_t4_dn17_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t4_rv_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_dn0_slot: &mut f64,
        var_t5_dn10_slot: &mut f64,
        var_t5_dn11_slot: &mut f64,
        var_t5_dn12_slot: &mut f64,
        var_t5_dn17_slot: &mut f64,
        var_t5_dn2_slot: &mut f64,
        var_t5_dn6_slot: &mut f64,
        var_t5_dn7_slot: &mut f64,
        var_t5_rv_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_dn0_slot: &mut f64,
        var_t6_dn10_slot: &mut f64,
        var_t6_dn11_slot: &mut f64,
        var_t6_dn12_slot: &mut f64,
        var_t6_dn17_slot: &mut f64,
        var_t6_dn2_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn7_slot: &mut f64,
        var_t6_rv_slot: &mut f64,
        var_tratio__blk1190_slot: &mut f64,
        var_tratio__blk1190_dn10_slot: &mut f64,
        var_tratio__blk1190_rv_slot: &mut f64,
        var_vmaxe__blk1194_slot: &mut f64,
        var_vmaxe__blk1194_dn0_slot: &mut f64,
        var_vmaxe__blk1194_dn10_slot: &mut f64,
        var_vmaxe__blk1194_dn11_slot: &mut f64,
        var_vmaxe__blk1194_dn12_slot: &mut f64,
        var_vmaxe__blk1194_dn17_slot: &mut f64,
        var_vmaxe__blk1194_dn2_slot: &mut f64,
        var_vmaxe__blk1194_dn6_slot: &mut f64,
        var_vmaxe__blk1194_dn7_slot: &mut f64,
        var_vmaxe__blk1194_rv_slot: &mut f64,
        var_vrdr__blk1189_slot: &mut f64,
        var_vrdr__blk1189_dn0_slot: &mut f64,
        var_vrdr__blk1189_dn2_slot: &mut f64,
        var_vrdr__blk1189_dn6_slot: &mut f64,
        var_vrdr__blk1189_dn7_slot: &mut f64,
        var_vrdr__blk1189_rv_slot: &mut f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let mut var_edri__blk1195: f64 = *var_edri__blk1195_slot;
        let mut var_edri__blk1195_dn0: f64 = *var_edri__blk1195_dn0_slot;
        let mut var_edri__blk1195_dn2: f64 = *var_edri__blk1195_dn2_slot;
        let mut var_edri__blk1195_dn6: f64 = *var_edri__blk1195_dn6_slot;
        let mut var_edri__blk1195_dn7: f64 = *var_edri__blk1195_dn7_slot;
        let mut var_edri__blk1195_rv: f64 = *var_edri__blk1195_rv_slot;
        let mut var_guard1177: f64 = *var_guard1177_slot;
        let mut var_guard1177_rv: f64 = *var_guard1177_rv_slot;
        let mut var_guard1178: f64 = *var_guard1178_slot;
        let mut var_guard1178_rv: f64 = *var_guard1178_rv_slot;
        let mut var_guard1181: f64 = *var_guard1181_slot;
        let mut var_guard1181_rv: f64 = *var_guard1181_rv_slot;
        let mut var_guard1201: f64 = *var_guard1201_slot;
        let mut var_guard1201_rv: f64 = *var_guard1201_rv_slot;
        let mut var_ldrifte__blk1191: f64 = *var_ldrifte__blk1191_slot;
        let mut var_ldrifte__blk1191_rv: f64 = *var_ldrifte__blk1191_rv_slot;
        let mut var_mks_rdrmue__blk1185: f64 = *var_mks_rdrmue__blk1185_slot;
        let mut var_mks_rdrmue__blk1185_rv: f64 = *var_mks_rdrmue__blk1185_rv_slot;
        let mut var_mks_rdrvmax__blk1186: f64 = *var_mks_rdrvmax__blk1186_slot;
        let mut var_mks_rdrvmax__blk1186_rv: f64 = *var_mks_rdrvmax__blk1186_rv_slot;
        let mut var_mu0__blk1193: f64 = *var_mu0__blk1193_slot;
        let mut var_mu0__blk1193_dn0: f64 = *var_mu0__blk1193_dn0_slot;
        let mut var_mu0__blk1193_dn10: f64 = *var_mu0__blk1193_dn10_slot;
        let mut var_mu0__blk1193_dn11: f64 = *var_mu0__blk1193_dn11_slot;
        let mut var_mu0__blk1193_dn12: f64 = *var_mu0__blk1193_dn12_slot;
        let mut var_mu0__blk1193_dn17: f64 = *var_mu0__blk1193_dn17_slot;
        let mut var_mu0__blk1193_dn2: f64 = *var_mu0__blk1193_dn2_slot;
        let mut var_mu0__blk1193_dn6: f64 = *var_mu0__blk1193_dn6_slot;
        let mut var_mu0__blk1193_dn7: f64 = *var_mu0__blk1193_dn7_slot;
        let mut var_mu0__blk1193_rv: f64 = *var_mu0__blk1193_rv_slot;
        let mut var_rdmod: f64 = *var_rdmod_slot;
        let mut var_rdmod_rv: f64 = *var_rdmod_rv_slot;
        let mut var_rdrmuele__blk1182: f64 = *var_rdrmuele__blk1182_slot;
        let mut var_rdrmuele__blk1182_rv: f64 = *var_rdrmuele__blk1182_rv_slot;
        let mut var_rdrvmaxle__blk1184: f64 = *var_rdrvmaxle__blk1184_slot;
        let mut var_rdrvmaxle__blk1184_rv: f64 = *var_rdrvmaxle__blk1184_rv_slot;
        let mut var_rdrvmaxwe__blk1183: f64 = *var_rdrvmaxwe__blk1183_slot;
        let mut var_rdrvmaxwe__blk1183_rv: f64 = *var_rdrvmaxwe__blk1183_rv_slot;
        let mut var_rrdrbb__blk1187: f64 = *var_rrdrbb__blk1187_slot;
        let mut var_rrdrbb__blk1187_dn10: f64 = *var_rrdrbb__blk1187_dn10_slot;
        let mut var_rrdrbb__blk1187_rv: f64 = *var_rrdrbb__blk1187_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn12: f64 = *var_t0_dn12_slot;
        let mut var_t0_dn17: f64 = *var_t0_dn17_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn17: f64 = *var_t1_dn17_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn12: f64 = *var_t2_dn12_slot;
        let mut var_t2_dn17: f64 = *var_t2_dn17_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn0: f64 = *var_t4_dn0_slot;
        let mut var_t4_dn10: f64 = *var_t4_dn10_slot;
        let mut var_t4_dn11: f64 = *var_t4_dn11_slot;
        let mut var_t4_dn12: f64 = *var_t4_dn12_slot;
        let mut var_t4_dn17: f64 = *var_t4_dn17_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t4_rv: f64 = *var_t4_rv_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_dn0: f64 = *var_t5_dn0_slot;
        let mut var_t5_dn10: f64 = *var_t5_dn10_slot;
        let mut var_t5_dn11: f64 = *var_t5_dn11_slot;
        let mut var_t5_dn12: f64 = *var_t5_dn12_slot;
        let mut var_t5_dn17: f64 = *var_t5_dn17_slot;
        let mut var_t5_dn2: f64 = *var_t5_dn2_slot;
        let mut var_t5_dn6: f64 = *var_t5_dn6_slot;
        let mut var_t5_dn7: f64 = *var_t5_dn7_slot;
        let mut var_t5_rv: f64 = *var_t5_rv_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_dn0: f64 = *var_t6_dn0_slot;
        let mut var_t6_dn10: f64 = *var_t6_dn10_slot;
        let mut var_t6_dn11: f64 = *var_t6_dn11_slot;
        let mut var_t6_dn12: f64 = *var_t6_dn12_slot;
        let mut var_t6_dn17: f64 = *var_t6_dn17_slot;
        let mut var_t6_dn2: f64 = *var_t6_dn2_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn7: f64 = *var_t6_dn7_slot;
        let mut var_t6_rv: f64 = *var_t6_rv_slot;
        let mut var_tratio__blk1190: f64 = *var_tratio__blk1190_slot;
        let mut var_tratio__blk1190_dn10: f64 = *var_tratio__blk1190_dn10_slot;
        let mut var_tratio__blk1190_rv: f64 = *var_tratio__blk1190_rv_slot;
        let mut var_vmaxe__blk1194: f64 = *var_vmaxe__blk1194_slot;
        let mut var_vmaxe__blk1194_dn0: f64 = *var_vmaxe__blk1194_dn0_slot;
        let mut var_vmaxe__blk1194_dn10: f64 = *var_vmaxe__blk1194_dn10_slot;
        let mut var_vmaxe__blk1194_dn11: f64 = *var_vmaxe__blk1194_dn11_slot;
        let mut var_vmaxe__blk1194_dn12: f64 = *var_vmaxe__blk1194_dn12_slot;
        let mut var_vmaxe__blk1194_dn17: f64 = *var_vmaxe__blk1194_dn17_slot;
        let mut var_vmaxe__blk1194_dn2: f64 = *var_vmaxe__blk1194_dn2_slot;
        let mut var_vmaxe__blk1194_dn6: f64 = *var_vmaxe__blk1194_dn6_slot;
        let mut var_vmaxe__blk1194_dn7: f64 = *var_vmaxe__blk1194_dn7_slot;
        let mut var_vmaxe__blk1194_rv: f64 = *var_vmaxe__blk1194_rv_slot;
        let mut var_vrdr__blk1189: f64 = *var_vrdr__blk1189_slot;
        let mut var_vrdr__blk1189_dn0: f64 = *var_vrdr__blk1189_dn0_slot;
        let mut var_vrdr__blk1189_dn2: f64 = *var_vrdr__blk1189_dn2_slot;
        let mut var_vrdr__blk1189_dn6: f64 = *var_vrdr__blk1189_dn6_slot;
        let mut var_vrdr__blk1189_dn7: f64 = *var_vrdr__blk1189_dn7_slot;
        let mut var_vrdr__blk1189_rv: f64 = *var_vrdr__blk1189_rv_slot;

        let (assign35510_e50364, assign35510_e50364_d_n0, assign35510_e50364_d_n2, assign35510_e50364_d_n6, assign35510_e50364_d_n7, assign35510_e50364_d_n10, assign35510_e50364_d_n11, assign35510_e50364_d_n12, assign35510_e50364_d_n17,) = {
    if (var_guard1153 != 0.0) {
        let assign35510_e50362: f64 = (var_t1 * var_t3);
        (assign35510_e50362, ((var_t1_dn0 * var_t3) + (var_t1 * var_t3_dn0)), ((var_t1_dn2 * var_t3) + (var_t1 * var_t3_dn2)), ((var_t1_dn6 * var_t3) + (var_t1 * var_t3_dn6)), ((var_t1_dn7 * var_t3) + (var_t1 * var_t3_dn7)), ((var_t1_dn10 * var_t3) + (var_t1 * var_t3_dn10)), ((var_t1_dn11 * var_t3) + (var_t1 * var_t3_dn11)), ((var_t1_dn12 * var_t3) + (var_t1 * var_t3_dn12)), ((var_t1_dn17 * var_t3) + (var_t1 * var_t3_dn17)),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn6, var_t2_dn7, var_t2_dn10, var_t2_dn11, var_t2_dn12, var_t2_dn17,)
    }
};
        var_t2 = assign35510_e50364;
        var_t2_dn0 = assign35510_e50364_d_n0;
        var_t2_dn2 = assign35510_e50364_d_n2;
        var_t2_dn6 = assign35510_e50364_d_n6;
        var_t2_dn7 = assign35510_e50364_d_n7;
        var_t2_dn10 = assign35510_e50364_d_n10;
        var_t2_dn11 = assign35510_e50364_d_n11;
        var_t2_dn12 = assign35510_e50364_d_n12;
        var_t2_dn17 = assign35510_e50364_d_n17;
        var_t2_rv = 0.0;

        let (assign35520_e50370, assign35520_e50370_d_n0, assign35520_e50370_d_n2, assign35520_e50370_d_n6, assign35520_e50370_d_n7, assign35520_e50370_d_n10, assign35520_e50370_d_n11, assign35520_e50370_d_n12, assign35520_e50370_d_n17,) = {
    if (var_guard1153 != 0.0) {
        let assign35520_e50368: f64 = (1.0 + var_t2);
        (assign35520_e50368, var_t2_dn0, var_t2_dn2, var_t2_dn6, var_t2_dn7, var_t2_dn10, var_t2_dn11, var_t2_dn12, var_t2_dn17,)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn6, var_t4_dn7, var_t4_dn10, var_t4_dn11, var_t4_dn12, var_t4_dn17,)
    }
};
        var_t4 = assign35520_e50370;
        var_t4_dn0 = assign35520_e50370_d_n0;
        var_t4_dn2 = assign35520_e50370_d_n2;
        var_t4_dn6 = assign35520_e50370_d_n6;
        var_t4_dn7 = assign35520_e50370_d_n7;
        var_t4_dn10 = assign35520_e50370_d_n10;
        var_t4_dn11 = assign35520_e50370_d_n11;
        var_t4_dn12 = assign35520_e50370_d_n12;
        var_t4_dn17 = assign35520_e50370_d_n17;
        var_t4_rv = 0.0;

        let assign35530_e50374: f64 = (10.0 * 2.220446049250313e-16);
        let assign35530_e50375: f64 = (1.0 - assign35530_e50374);
        let assign35530_e50382: f64 = (10.0 * 2.220446049250313e-16);
        let assign35530_e50383: f64 = (1.0 + assign35530_e50382);
        let assign35530_e50385: f64 = if ((assign35530_e50375 <= var_rrdrbb) && (var_rrdrbb <= assign35530_e50383)) { 1.0 } else { 0.0 };
        var_guard1177 = assign35530_e50385;
        var_guard1177_rv = 0.0;

        let (assign35540_e50393, assign35540_e50393_d_n0, assign35540_e50393_d_n2, assign35540_e50393_d_n6, assign35540_e50393_d_n7, assign35540_e50393_d_n10, assign35540_e50393_d_n11, assign35540_e50393_d_n12, assign35540_e50393_d_n17,) = {
    if ((var_guard1153 != 0.0) && (var_guard1177 != 0.0)) {
        let assign35540_e50391: f64 = (1.0 / var_t4);
        (assign35540_e50391, (-(var_t4_dn0 / (var_t4 * var_t4))), (-(var_t4_dn2 / (var_t4 * var_t4))), (-(var_t4_dn6 / (var_t4 * var_t4))), (-(var_t4_dn7 / (var_t4 * var_t4))), (-(var_t4_dn10 / (var_t4 * var_t4))), (-(var_t4_dn11 / (var_t4 * var_t4))), (-(var_t4_dn12 / (var_t4 * var_t4))), (-(var_t4_dn17 / (var_t4 * var_t4))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn6, var_t5_dn7, var_t5_dn10, var_t5_dn11, var_t5_dn12, var_t5_dn17,)
    }
};
        var_t5 = assign35540_e50393;
        var_t5_dn0 = assign35540_e50393_d_n0;
        var_t5_dn2 = assign35540_e50393_d_n2;
        var_t5_dn6 = assign35540_e50393_d_n6;
        var_t5_dn7 = assign35540_e50393_d_n7;
        var_t5_dn10 = assign35540_e50393_d_n10;
        var_t5_dn11 = assign35540_e50393_d_n11;
        var_t5_dn12 = assign35540_e50393_d_n12;
        var_t5_dn17 = assign35540_e50393_d_n17;
        var_t5_rv = 0.0;

        let assign35550_e50397: f64 = (10.0 * 2.220446049250313e-16);
        let assign35550_e50398: f64 = (2.0 - assign35550_e50397);
        let assign35550_e50405: f64 = (10.0 * 2.220446049250313e-16);
        let assign35550_e50406: f64 = (2.0 + assign35550_e50405);
        let assign35550_e50408: f64 = if ((assign35550_e50398 <= var_rrdrbb) && (var_rrdrbb <= assign35550_e50406)) { 1.0 } else { 0.0 };
        var_guard1178 = assign35550_e50408;
        var_guard1178_rv = 0.0;

        let (assign35560_e50420, assign35560_e50420_d_n0, assign35560_e50420_d_n2, assign35560_e50420_d_n6, assign35560_e50420_d_n7, assign35560_e50420_d_n10, assign35560_e50420_d_n11, assign35560_e50420_d_n12, assign35560_e50420_d_n17,) = {
    if (((var_guard1153 != 0.0) && (var_guard1177 == 0.0)) && (var_guard1178 != 0.0)) {
        let assign35560_e50417: f64 = (var_t4).sqrt();
        let assign35560_e50418: f64 = (1.0 / assign35560_e50417);
        (assign35560_e50418, (-((var_t4_dn0 / (2.0 * assign35560_e50417)) / (assign35560_e50417 * assign35560_e50417))), (-((var_t4_dn2 / (2.0 * assign35560_e50417)) / (assign35560_e50417 * assign35560_e50417))), (-((var_t4_dn6 / (2.0 * assign35560_e50417)) / (assign35560_e50417 * assign35560_e50417))), (-((var_t4_dn7 / (2.0 * assign35560_e50417)) / (assign35560_e50417 * assign35560_e50417))), (-((var_t4_dn10 / (2.0 * assign35560_e50417)) / (assign35560_e50417 * assign35560_e50417))), (-((var_t4_dn11 / (2.0 * assign35560_e50417)) / (assign35560_e50417 * assign35560_e50417))), (-((var_t4_dn12 / (2.0 * assign35560_e50417)) / (assign35560_e50417 * assign35560_e50417))), (-((var_t4_dn17 / (2.0 * assign35560_e50417)) / (assign35560_e50417 * assign35560_e50417))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn6, var_t5_dn7, var_t5_dn10, var_t5_dn11, var_t5_dn12, var_t5_dn17,)
    }
};
        var_t5 = assign35560_e50420;
        var_t5_dn0 = assign35560_e50420_d_n0;
        var_t5_dn2 = assign35560_e50420_d_n2;
        var_t5_dn6 = assign35560_e50420_d_n6;
        var_t5_dn7 = assign35560_e50420_d_n7;
        var_t5_dn10 = assign35560_e50420_d_n10;
        var_t5_dn11 = assign35560_e50420_d_n11;
        var_t5_dn12 = assign35560_e50420_d_n12;
        var_t5_dn17 = assign35560_e50420_d_n17;
        var_t5_rv = 0.0;

        let (assign35570_e50437, assign35570_e50437_d_n0, assign35570_e50437_d_n2, assign35570_e50437_d_n6, assign35570_e50437_d_n7, assign35570_e50437_d_n10, assign35570_e50437_d_n11, assign35570_e50437_d_n12, assign35570_e50437_d_n17,) = {
    if (((var_guard1153 != 0.0) && (var_guard1177 == 0.0)) && (var_guard1178 == 0.0)) {
        let assign35570_e50430: f64 = (-1.0);
        let assign35570_e50432: f64 = (assign35570_e50430 / var_rrdrbb);
        let assign35570_e50434: f64 = (assign35570_e50432 - 1.0);
        let assign35570_e50435: f64 = (var_t4).powf(assign35570_e50434);
        (assign35570_e50435, if 0.0 == 0.0 && ((assign35570_e50434) as f64).is_finite() && ((assign35570_e50434) as f64).fract() == 0.0 { if assign35570_e50434 == 0.0 { 0.0 } else { (assign35570_e50434 * ((var_t4).powf(assign35570_e50434 - 1.0) * var_t4_dn0)) } } else { (assign35570_e50435 * (assign35570_e50434 * (var_t4_dn0 / var_t4))) }, if 0.0 == 0.0 && ((assign35570_e50434) as f64).is_finite() && ((assign35570_e50434) as f64).fract() == 0.0 { if assign35570_e50434 == 0.0 { 0.0 } else { (assign35570_e50434 * ((var_t4).powf(assign35570_e50434 - 1.0) * var_t4_dn2)) } } else { (assign35570_e50435 * (assign35570_e50434 * (var_t4_dn2 / var_t4))) }, if 0.0 == 0.0 && ((assign35570_e50434) as f64).is_finite() && ((assign35570_e50434) as f64).fract() == 0.0 { if assign35570_e50434 == 0.0 { 0.0 } else { (assign35570_e50434 * ((var_t4).powf(assign35570_e50434 - 1.0) * var_t4_dn6)) } } else { (assign35570_e50435 * (assign35570_e50434 * (var_t4_dn6 / var_t4))) }, if 0.0 == 0.0 && ((assign35570_e50434) as f64).is_finite() && ((assign35570_e50434) as f64).fract() == 0.0 { if assign35570_e50434 == 0.0 { 0.0 } else { (assign35570_e50434 * ((var_t4).powf(assign35570_e50434 - 1.0) * var_t4_dn7)) } } else { (assign35570_e50435 * (assign35570_e50434 * (var_t4_dn7 / var_t4))) }, if (-((assign35570_e50430 * var_rrdrbb_dn10) / (var_rrdrbb * var_rrdrbb))) == 0.0 && ((assign35570_e50434) as f64).is_finite() && ((assign35570_e50434) as f64).fract() == 0.0 { if assign35570_e50434 == 0.0 { 0.0 } else { (assign35570_e50434 * ((var_t4).powf(assign35570_e50434 - 1.0) * var_t4_dn10)) } } else { (assign35570_e50435 * (((-((assign35570_e50430 * var_rrdrbb_dn10) / (var_rrdrbb * var_rrdrbb))) * (var_t4).ln()) + (assign35570_e50434 * (var_t4_dn10 / var_t4)))) }, if 0.0 == 0.0 && ((assign35570_e50434) as f64).is_finite() && ((assign35570_e50434) as f64).fract() == 0.0 { if assign35570_e50434 == 0.0 { 0.0 } else { (assign35570_e50434 * ((var_t4).powf(assign35570_e50434 - 1.0) * var_t4_dn11)) } } else { (assign35570_e50435 * (assign35570_e50434 * (var_t4_dn11 / var_t4))) }, if 0.0 == 0.0 && ((assign35570_e50434) as f64).is_finite() && ((assign35570_e50434) as f64).fract() == 0.0 { if assign35570_e50434 == 0.0 { 0.0 } else { (assign35570_e50434 * ((var_t4).powf(assign35570_e50434 - 1.0) * var_t4_dn12)) } } else { (assign35570_e50435 * (assign35570_e50434 * (var_t4_dn12 / var_t4))) }, if 0.0 == 0.0 && ((assign35570_e50434) as f64).is_finite() && ((assign35570_e50434) as f64).fract() == 0.0 { if assign35570_e50434 == 0.0 { 0.0 } else { (assign35570_e50434 * ((var_t4).powf(assign35570_e50434 - 1.0) * var_t4_dn17)) } } else { (assign35570_e50435 * (assign35570_e50434 * (var_t4_dn17 / var_t4))) },)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn6, var_t6_dn7, var_t6_dn10, var_t6_dn11, var_t6_dn12, var_t6_dn17,)
    }
};
        var_t6 = assign35570_e50437;
        var_t6_dn0 = assign35570_e50437_d_n0;
        var_t6_dn2 = assign35570_e50437_d_n2;
        var_t6_dn6 = assign35570_e50437_d_n6;
        var_t6_dn7 = assign35570_e50437_d_n7;
        var_t6_dn10 = assign35570_e50437_d_n10;
        var_t6_dn11 = assign35570_e50437_d_n11;
        var_t6_dn12 = assign35570_e50437_d_n12;
        var_t6_dn17 = assign35570_e50437_d_n17;
        var_t6_rv = 0.0;

        let (assign35580_e50449, assign35580_e50449_d_n0, assign35580_e50449_d_n2, assign35580_e50449_d_n6, assign35580_e50449_d_n7, assign35580_e50449_d_n10, assign35580_e50449_d_n11, assign35580_e50449_d_n12, assign35580_e50449_d_n17,) = {
    if (((var_guard1153 != 0.0) && (var_guard1177 == 0.0)) && (var_guard1178 == 0.0)) {
        let assign35580_e50447: f64 = (var_t4 * var_t6);
        (assign35580_e50447, ((var_t4_dn0 * var_t6) + (var_t4 * var_t6_dn0)), ((var_t4_dn2 * var_t6) + (var_t4 * var_t6_dn2)), ((var_t4_dn6 * var_t6) + (var_t4 * var_t6_dn6)), ((var_t4_dn7 * var_t6) + (var_t4 * var_t6_dn7)), ((var_t4_dn10 * var_t6) + (var_t4 * var_t6_dn10)), ((var_t4_dn11 * var_t6) + (var_t4 * var_t6_dn11)), ((var_t4_dn12 * var_t6) + (var_t4 * var_t6_dn12)), ((var_t4_dn17 * var_t6) + (var_t4 * var_t6_dn17)),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn6, var_t5_dn7, var_t5_dn10, var_t5_dn11, var_t5_dn12, var_t5_dn17,)
    }
};
        var_t5 = assign35580_e50449;
        var_t5_dn0 = assign35580_e50449_d_n0;
        var_t5_dn2 = assign35580_e50449_d_n2;
        var_t5_dn6 = assign35580_e50449_d_n6;
        var_t5_dn7 = assign35580_e50449_d_n7;
        var_t5_dn10 = assign35580_e50449_d_n10;
        var_t5_dn11 = assign35580_e50449_d_n11;
        var_t5_dn12 = assign35580_e50449_d_n12;
        var_t5_dn17 = assign35580_e50449_d_n17;
        var_t5_rv = 0.0;

        let (assign35600_e50461, assign35600_e50461_d_n0, assign35600_e50461_d_n2, assign35600_e50461_d_n6, assign35600_e50461_d_n7, assign35600_e50461_d_n10, assign35600_e50461_d_n11, assign35600_e50461_d_n12, assign35600_e50461_d_n17,) = {
    if (var_guard1153 != 0.0) {
        let assign35600_e50459: f64 = (1.6021918e-19 / var_ldrifte);
        (assign35600_e50459, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign35600_e50461;
        var_t1_dn0 = assign35600_e50461_d_n0;
        var_t1_dn2 = assign35600_e50461_d_n2;
        var_t1_dn6 = assign35600_e50461_d_n6;
        var_t1_dn7 = assign35600_e50461_d_n7;
        var_t1_dn10 = assign35600_e50461_d_n10;
        var_t1_dn11 = assign35600_e50461_d_n11;
        var_t1_dn12 = assign35600_e50461_d_n12;
        var_t1_dn17 = assign35600_e50461_d_n17;
        var_t1_rv = 0.0;

        let assign35720_e50535: f64 = if p.p260 == 1.0 { 1.0 } else { 0.0 };
        var_guard1181 = assign35720_e50535;
        var_guard1181_rv = 0.0;

        let (assign35730_e50539,) = {
    if (var_guard1181 != 0.0) {
        (2.0,)
    } else {
        (var_rdmod,)
    }
};
        var_rdmod = assign35730_e50539;
        var_rdmod_rv = 0.0;

        let assign35740_e50542: f64 = if var_rdmod == 1.0 { 1.0 } else { 0.0 };
        var_guard1201 = assign35740_e50542;
        var_guard1201_rv = 0.0;

        let (assign35760_e50556,) = {
    if ((var_guard1181 != 0.0) && (var_guard1201 != 0.0)) {
        (p.p266,)
    } else {
        (var_mks_rdrmue__blk1185,)
    }
};
        var_mks_rdrmue__blk1185 = assign35760_e50556;
        var_mks_rdrmue__blk1185_rv = 0.0;

        let (assign35770_e50562,) = {
    if ((var_guard1181 != 0.0) && (var_guard1201 != 0.0)) {
        (p.p268,)
    } else {
        (var_mks_rdrvmax__blk1186,)
    }
};
        var_mks_rdrvmax__blk1186 = assign35770_e50562;
        var_mks_rdrvmax__blk1186_rv = 0.0;

        let (assign35780_e50568, assign35780_e50568_d_n10,) = {
    if ((var_guard1181 != 0.0) && (var_guard1201 != 0.0)) {
        (p.p273, 0.0,)
    } else {
        (var_rrdrbb__blk1187, var_rrdrbb__blk1187_dn10,)
    }
};
        var_rrdrbb__blk1187 = assign35780_e50568;
        var_rrdrbb__blk1187_dn10 = assign35780_e50568_d_n10;
        var_rrdrbb__blk1187_rv = 0.0;

        let (assign35800_e50587,) = {
    if ((var_guard1181 != 0.0) && (var_guard1201 != 0.0)) {
        (p.p258,)
    } else {
        (var_ldrifte__blk1191,)
    }
};
        var_ldrifte__blk1191 = assign35800_e50587;
        var_ldrifte__blk1191_rv = 0.0;

        let (assign35810_e50595, assign35810_e50595_d_n0, assign35810_e50595_d_n2, assign35810_e50595_d_n6, assign35810_e50595_d_n7,) = {
    if ((var_guard1181 != 0.0) && (var_guard1201 != 0.0)) {
        let assign35810_e50593: f64 = (p.p50 * (nv7 - nv2));
        (assign35810_e50593, 0.0, (-p.p50), 0.0, p.p50,)
    } else {
        (var_vrdr__blk1189, var_vrdr__blk1189_dn0, var_vrdr__blk1189_dn2, var_vrdr__blk1189_dn6, var_vrdr__blk1189_dn7,)
    }
};
        var_vrdr__blk1189 = assign35810_e50595;
        var_vrdr__blk1189_dn0 = assign35810_e50595_d_n0;
        var_vrdr__blk1189_dn2 = assign35810_e50595_d_n2;
        var_vrdr__blk1189_dn6 = assign35810_e50595_d_n6;
        var_vrdr__blk1189_dn7 = assign35810_e50595_d_n7;
        var_vrdr__blk1189_rv = 0.0;

        let (assign35830_e50611,) = {
    if ((var_guard1181 != 0.0) && (var_guard1201 == 0.0)) {
        (p.p265,)
    } else {
        (var_mks_rdrmue__blk1185,)
    }
};
        var_mks_rdrmue__blk1185 = assign35830_e50611;
        var_mks_rdrmue__blk1185_rv = 0.0;

        let (assign35840_e50618,) = {
    if ((var_guard1181 != 0.0) && (var_guard1201 == 0.0)) {
        (p.p267,)
    } else {
        (var_mks_rdrvmax__blk1186,)
    }
};
        var_mks_rdrvmax__blk1186 = assign35840_e50618;
        var_mks_rdrvmax__blk1186_rv = 0.0;

        let (assign35850_e50625, assign35850_e50625_d_n10,) = {
    if ((var_guard1181 != 0.0) && (var_guard1201 == 0.0)) {
        (p.p272, 0.0,)
    } else {
        (var_rrdrbb__blk1187, var_rrdrbb__blk1187_dn10,)
    }
};
        var_rrdrbb__blk1187 = assign35850_e50625;
        var_rrdrbb__blk1187_dn10 = assign35850_e50625_d_n10;
        var_rrdrbb__blk1187_rv = 0.0;

        let (assign35870_e50646,) = {
    if ((var_guard1181 != 0.0) && (var_guard1201 == 0.0)) {
        (p.p257,)
    } else {
        (var_ldrifte__blk1191,)
    }
};
        var_ldrifte__blk1191 = assign35870_e50646;
        var_ldrifte__blk1191_rv = 0.0;

        let (assign35880_e50655, assign35880_e50655_d_n0, assign35880_e50655_d_n2, assign35880_e50655_d_n6, assign35880_e50655_d_n7,) = {
    if ((var_guard1181 != 0.0) && (var_guard1201 == 0.0)) {
        let assign35880_e50653: f64 = (p.p50 * (nv0 - nv6));
        (assign35880_e50653, p.p50, 0.0, (-p.p50), 0.0,)
    } else {
        (var_vrdr__blk1189, var_vrdr__blk1189_dn0, var_vrdr__blk1189_dn2, var_vrdr__blk1189_dn6, var_vrdr__blk1189_dn7,)
    }
};
        var_vrdr__blk1189 = assign35880_e50655;
        var_vrdr__blk1189_dn0 = assign35880_e50655_d_n0;
        var_vrdr__blk1189_dn2 = assign35880_e50655_d_n2;
        var_vrdr__blk1189_dn6 = assign35880_e50655_d_n6;
        var_vrdr__blk1189_dn7 = assign35880_e50655_d_n7;
        var_vrdr__blk1189_rv = 0.0;

        let (assign35910_e50678,) = {
    if (var_guard1181 != 0.0) {
        let assign35910_e50676: f64 = (var_mks_rdrmue__blk1185 / 10000.0);
        (assign35910_e50676,)
    } else {
        (var_mks_rdrmue__blk1185,)
    }
};
        var_mks_rdrmue__blk1185 = assign35910_e50678;
        var_mks_rdrmue__blk1185_rv = 0.0;

        let (assign35920_e50684,) = {
    if (var_guard1181 != 0.0) {
        let assign35920_e50682: f64 = (var_mks_rdrvmax__blk1186 / 100.0);
        (assign35920_e50682,)
    } else {
        (var_mks_rdrvmax__blk1186,)
    }
};
        var_mks_rdrvmax__blk1186 = assign35920_e50684;
        var_mks_rdrvmax__blk1186_rv = 0.0;

        let (assign35930_e50690, assign35930_e50690_d_n10,) = {
    if (var_guard1181 != 0.0) {
        let assign35930_e50688: f64 = (var_ttemp / var_uc_tnom);
        (assign35930_e50688, (var_ttemp_dn10 / var_uc_tnom),)
    } else {
        (var_tratio__blk1190, var_tratio__blk1190_dn10,)
    }
};
        var_tratio__blk1190 = assign35930_e50690;
        var_tratio__blk1190_dn10 = assign35930_e50690_d_n10;
        var_tratio__blk1190_rv = 0.0;

        let (assign35940_e50696, assign35940_e50696_d_n0, assign35940_e50696_d_n2, assign35940_e50696_d_n6, assign35940_e50696_d_n7, assign35940_e50696_d_n10, assign35940_e50696_d_n11, assign35940_e50696_d_n12, assign35940_e50696_d_n17,) = {
    if (var_guard1181 != 0.0) {
        let assign35940_e50694: f64 = (var_tratio__blk1190).powf(p.p269);
        (assign35940_e50694, 0.0, 0.0, 0.0, 0.0, if 0.0 == 0.0 && ((p.p269) as f64).is_finite() && ((p.p269) as f64).fract() == 0.0 { if p.p269 == 0.0 { 0.0 } else { (p.p269 * ((var_tratio__blk1190).powf(p.p269 - 1.0) * var_tratio__blk1190_dn10)) } } else { (assign35940_e50694 * (p.p269 * (var_tratio__blk1190_dn10 / var_tratio__blk1190))) }, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign35940_e50696;
        var_t1_dn0 = assign35940_e50696_d_n0;
        var_t1_dn2 = assign35940_e50696_d_n2;
        var_t1_dn6 = assign35940_e50696_d_n6;
        var_t1_dn7 = assign35940_e50696_d_n7;
        var_t1_dn10 = assign35940_e50696_d_n10;
        var_t1_dn11 = assign35940_e50696_d_n11;
        var_t1_dn12 = assign35940_e50696_d_n12;
        var_t1_dn17 = assign35940_e50696_d_n17;
        var_t1_rv = 0.0;

        let (assign35950_e50702, assign35950_e50702_d_n0, assign35950_e50702_d_n2, assign35950_e50702_d_n6, assign35950_e50702_d_n7, assign35950_e50702_d_n10, assign35950_e50702_d_n11, assign35950_e50702_d_n12, assign35950_e50702_d_n17,) = {
    if (var_guard1181 != 0.0) {
        let assign35950_e50700: f64 = (var_mks_rdrmue__blk1185 / var_t1);
        (assign35950_e50700, (-((var_mks_rdrmue__blk1185 * var_t1_dn0) / (var_t1 * var_t1))), (-((var_mks_rdrmue__blk1185 * var_t1_dn2) / (var_t1 * var_t1))), (-((var_mks_rdrmue__blk1185 * var_t1_dn6) / (var_t1 * var_t1))), (-((var_mks_rdrmue__blk1185 * var_t1_dn7) / (var_t1 * var_t1))), (-((var_mks_rdrmue__blk1185 * var_t1_dn10) / (var_t1 * var_t1))), (-((var_mks_rdrmue__blk1185 * var_t1_dn11) / (var_t1 * var_t1))), (-((var_mks_rdrmue__blk1185 * var_t1_dn12) / (var_t1 * var_t1))), (-((var_mks_rdrmue__blk1185 * var_t1_dn17) / (var_t1 * var_t1))),)
    } else {
        (var_mu0__blk1193, var_mu0__blk1193_dn0, var_mu0__blk1193_dn2, var_mu0__blk1193_dn6, var_mu0__blk1193_dn7, var_mu0__blk1193_dn10, var_mu0__blk1193_dn11, var_mu0__blk1193_dn12, var_mu0__blk1193_dn17,)
    }
};
        var_mu0__blk1193 = assign35950_e50702;
        var_mu0__blk1193_dn0 = assign35950_e50702_d_n0;
        var_mu0__blk1193_dn2 = assign35950_e50702_d_n2;
        var_mu0__blk1193_dn6 = assign35950_e50702_d_n6;
        var_mu0__blk1193_dn7 = assign35950_e50702_d_n7;
        var_mu0__blk1193_dn10 = assign35950_e50702_d_n10;
        var_mu0__blk1193_dn11 = assign35950_e50702_d_n11;
        var_mu0__blk1193_dn12 = assign35950_e50702_d_n12;
        var_mu0__blk1193_dn17 = assign35950_e50702_d_n17;
        var_mu0__blk1193_rv = 0.0;

        let (assign35960_e50722, assign35960_e50722_d_n0, assign35960_e50722_d_n2, assign35960_e50722_d_n6, assign35960_e50722_d_n7, assign35960_e50722_d_n10, assign35960_e50722_d_n11, assign35960_e50722_d_n12, assign35960_e50722_d_n17,) = {
    if (var_guard1181 != 0.0) {
        let assign35960_e50707: f64 = (0.4 * var_tratio__blk1190);
        let assign35960_e50708: f64 = (1.8 + assign35960_e50707);
        let assign35960_e50711: f64 = (0.1 * var_tratio__blk1190);
        let assign35960_e50713: f64 = (assign35960_e50711 * var_tratio__blk1190);
        let assign35960_e50714: f64 = (assign35960_e50708 + assign35960_e50713);
        let assign35960_e50718: f64 = (1.0 - var_tratio__blk1190);
        let assign35960_e50719: f64 = (p.p270 * assign35960_e50718);
        let assign35960_e50720: f64 = (assign35960_e50714 - assign35960_e50719);
        (assign35960_e50720, 0.0, 0.0, 0.0, 0.0, (((0.4 * var_tratio__blk1190_dn10) + (((0.1 * var_tratio__blk1190_dn10) * var_tratio__blk1190) + (assign35960_e50711 * var_tratio__blk1190_dn10))) - (p.p270 * (-var_tratio__blk1190_dn10))), 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn6, var_t0_dn7, var_t0_dn10, var_t0_dn11, var_t0_dn12, var_t0_dn17,)
    }
};
        var_t0 = assign35960_e50722;
        var_t0_dn0 = assign35960_e50722_d_n0;
        var_t0_dn2 = assign35960_e50722_d_n2;
        var_t0_dn6 = assign35960_e50722_d_n6;
        var_t0_dn7 = assign35960_e50722_d_n7;
        var_t0_dn10 = assign35960_e50722_d_n10;
        var_t0_dn11 = assign35960_e50722_d_n11;
        var_t0_dn12 = assign35960_e50722_d_n12;
        var_t0_dn17 = assign35960_e50722_d_n17;
        var_t0_rv = 0.0;

        let (assign35970_e50728, assign35970_e50728_d_n0, assign35970_e50728_d_n2, assign35970_e50728_d_n6, assign35970_e50728_d_n7, assign35970_e50728_d_n10, assign35970_e50728_d_n11, assign35970_e50728_d_n12, assign35970_e50728_d_n17,) = {
    if (var_guard1181 != 0.0) {
        let assign35970_e50726: f64 = (var_mks_rdrvmax__blk1186 / var_t0);
        (assign35970_e50726, (-((var_mks_rdrvmax__blk1186 * var_t0_dn0) / (var_t0 * var_t0))), (-((var_mks_rdrvmax__blk1186 * var_t0_dn2) / (var_t0 * var_t0))), (-((var_mks_rdrvmax__blk1186 * var_t0_dn6) / (var_t0 * var_t0))), (-((var_mks_rdrvmax__blk1186 * var_t0_dn7) / (var_t0 * var_t0))), (-((var_mks_rdrvmax__blk1186 * var_t0_dn10) / (var_t0 * var_t0))), (-((var_mks_rdrvmax__blk1186 * var_t0_dn11) / (var_t0 * var_t0))), (-((var_mks_rdrvmax__blk1186 * var_t0_dn12) / (var_t0 * var_t0))), (-((var_mks_rdrvmax__blk1186 * var_t0_dn17) / (var_t0 * var_t0))),)
    } else {
        (var_vmaxe__blk1194, var_vmaxe__blk1194_dn0, var_vmaxe__blk1194_dn2, var_vmaxe__blk1194_dn6, var_vmaxe__blk1194_dn7, var_vmaxe__blk1194_dn10, var_vmaxe__blk1194_dn11, var_vmaxe__blk1194_dn12, var_vmaxe__blk1194_dn17,)
    }
};
        var_vmaxe__blk1194 = assign35970_e50728;
        var_vmaxe__blk1194_dn0 = assign35970_e50728_d_n0;
        var_vmaxe__blk1194_dn2 = assign35970_e50728_d_n2;
        var_vmaxe__blk1194_dn6 = assign35970_e50728_d_n6;
        var_vmaxe__blk1194_dn7 = assign35970_e50728_d_n7;
        var_vmaxe__blk1194_dn10 = assign35970_e50728_d_n10;
        var_vmaxe__blk1194_dn11 = assign35970_e50728_d_n11;
        var_vmaxe__blk1194_dn12 = assign35970_e50728_d_n12;
        var_vmaxe__blk1194_dn17 = assign35970_e50728_d_n17;
        var_vmaxe__blk1194_rv = 0.0;

        let (assign35980_e50738, assign35980_e50738_d_n10,) = {
    if (var_guard1181 != 0.0) {
        let assign35980_e50734: f64 = (var_ttemp - var_uc_tnom);
        let assign35980_e50735: f64 = (p.p274 * assign35980_e50734);
        let assign35980_e50736: f64 = (var_rrdrbb__blk1187 + assign35980_e50735);
        (assign35980_e50736, (var_rrdrbb__blk1187_dn10 + (p.p274 * var_ttemp_dn10)),)
    } else {
        (var_rrdrbb__blk1187, var_rrdrbb__blk1187_dn10,)
    }
};
        var_rrdrbb__blk1187 = assign35980_e50738;
        var_rrdrbb__blk1187_dn10 = assign35980_e50738_d_n10;
        var_rrdrbb__blk1187_rv = 0.0;

        let (assign35990_e50748,) = {
    if (var_guard1181 != 0.0) {
        let assign35990_e50744: f64 = (var_lgle).powf(p.p280);
        let assign35990_e50745: f64 = (p.p279 / assign35990_e50744);
        let assign35990_e50746: f64 = (1.0 + assign35990_e50745);
        (assign35990_e50746,)
    } else {
        (var_rdrmuele__blk1182,)
    }
};
        var_rdrmuele__blk1182 = assign35990_e50748;
        var_rdrmuele__blk1182_rv = 0.0;

        let (assign36000_e50758,) = {
    if (var_guard1181 != 0.0) {
        let assign36000_e50754: f64 = (var_lgle).powf(p.p278);
        let assign36000_e50755: f64 = (p.p277 / assign36000_e50754);
        let assign36000_e50756: f64 = (1.0 + assign36000_e50755);
        (assign36000_e50756,)
    } else {
        (var_rdrvmaxle__blk1184,)
    }
};
        var_rdrvmaxle__blk1184 = assign36000_e50758;
        var_rdrvmaxle__blk1184_rv = 0.0;

        let (assign36010_e50768,) = {
    if (var_guard1181 != 0.0) {
        let assign36010_e50764: f64 = (var_wg).powf(p.p276);
        let assign36010_e50765: f64 = (p.p275 / assign36010_e50764);
        let assign36010_e50766: f64 = (1.0 + assign36010_e50765);
        (assign36010_e50766,)
    } else {
        (var_rdrvmaxwe__blk1183,)
    }
};
        var_rdrvmaxwe__blk1183 = assign36010_e50768;
        var_rdrvmaxwe__blk1183_rv = 0.0;

        let (assign36020_e50774, assign36020_e50774_d_n0, assign36020_e50774_d_n2, assign36020_e50774_d_n6, assign36020_e50774_d_n7, assign36020_e50774_d_n10, assign36020_e50774_d_n11, assign36020_e50774_d_n12, assign36020_e50774_d_n17,) = {
    if (var_guard1181 != 0.0) {
        let assign36020_e50772: f64 = (var_mu0__blk1193 * var_rdrmuele__blk1182);
        (assign36020_e50772, (var_mu0__blk1193_dn0 * var_rdrmuele__blk1182), (var_mu0__blk1193_dn2 * var_rdrmuele__blk1182), (var_mu0__blk1193_dn6 * var_rdrmuele__blk1182), (var_mu0__blk1193_dn7 * var_rdrmuele__blk1182), (var_mu0__blk1193_dn10 * var_rdrmuele__blk1182), (var_mu0__blk1193_dn11 * var_rdrmuele__blk1182), (var_mu0__blk1193_dn12 * var_rdrmuele__blk1182), (var_mu0__blk1193_dn17 * var_rdrmuele__blk1182),)
    } else {
        (var_mu0__blk1193, var_mu0__blk1193_dn0, var_mu0__blk1193_dn2, var_mu0__blk1193_dn6, var_mu0__blk1193_dn7, var_mu0__blk1193_dn10, var_mu0__blk1193_dn11, var_mu0__blk1193_dn12, var_mu0__blk1193_dn17,)
    }
};
        var_mu0__blk1193 = assign36020_e50774;
        var_mu0__blk1193_dn0 = assign36020_e50774_d_n0;
        var_mu0__blk1193_dn2 = assign36020_e50774_d_n2;
        var_mu0__blk1193_dn6 = assign36020_e50774_d_n6;
        var_mu0__blk1193_dn7 = assign36020_e50774_d_n7;
        var_mu0__blk1193_dn10 = assign36020_e50774_d_n10;
        var_mu0__blk1193_dn11 = assign36020_e50774_d_n11;
        var_mu0__blk1193_dn12 = assign36020_e50774_d_n12;
        var_mu0__blk1193_dn17 = assign36020_e50774_d_n17;
        var_mu0__blk1193_rv = 0.0;

        let (assign36030_e50784, assign36030_e50784_d_n0, assign36030_e50784_d_n2, assign36030_e50784_d_n6, assign36030_e50784_d_n7, assign36030_e50784_d_n10, assign36030_e50784_d_n11, assign36030_e50784_d_n12, assign36030_e50784_d_n17,) = {
    if (var_guard1181 != 0.0) {
        let assign36030_e50778: f64 = (var_vmaxe__blk1194 * var_rdrvmaxwe__blk1183);
        let assign36030_e50780: f64 = (assign36030_e50778 * var_rdrvmaxle__blk1184);
        let assign36030_e50782: f64 = (assign36030_e50780 + 1e-50);
        (assign36030_e50782, ((var_vmaxe__blk1194_dn0 * var_rdrvmaxwe__blk1183) * var_rdrvmaxle__blk1184), ((var_vmaxe__blk1194_dn2 * var_rdrvmaxwe__blk1183) * var_rdrvmaxle__blk1184), ((var_vmaxe__blk1194_dn6 * var_rdrvmaxwe__blk1183) * var_rdrvmaxle__blk1184), ((var_vmaxe__blk1194_dn7 * var_rdrvmaxwe__blk1183) * var_rdrvmaxle__blk1184), ((var_vmaxe__blk1194_dn10 * var_rdrvmaxwe__blk1183) * var_rdrvmaxle__blk1184), ((var_vmaxe__blk1194_dn11 * var_rdrvmaxwe__blk1183) * var_rdrvmaxle__blk1184), ((var_vmaxe__blk1194_dn12 * var_rdrvmaxwe__blk1183) * var_rdrvmaxle__blk1184), ((var_vmaxe__blk1194_dn17 * var_rdrvmaxwe__blk1183) * var_rdrvmaxle__blk1184),)
    } else {
        (var_vmaxe__blk1194, var_vmaxe__blk1194_dn0, var_vmaxe__blk1194_dn2, var_vmaxe__blk1194_dn6, var_vmaxe__blk1194_dn7, var_vmaxe__blk1194_dn10, var_vmaxe__blk1194_dn11, var_vmaxe__blk1194_dn12, var_vmaxe__blk1194_dn17,)
    }
};
        var_vmaxe__blk1194 = assign36030_e50784;
        var_vmaxe__blk1194_dn0 = assign36030_e50784_d_n0;
        var_vmaxe__blk1194_dn2 = assign36030_e50784_d_n2;
        var_vmaxe__blk1194_dn6 = assign36030_e50784_d_n6;
        var_vmaxe__blk1194_dn7 = assign36030_e50784_d_n7;
        var_vmaxe__blk1194_dn10 = assign36030_e50784_d_n10;
        var_vmaxe__blk1194_dn11 = assign36030_e50784_d_n11;
        var_vmaxe__blk1194_dn12 = assign36030_e50784_d_n12;
        var_vmaxe__blk1194_dn17 = assign36030_e50784_d_n17;
        var_vmaxe__blk1194_rv = 0.0;

        let (assign36040_e50790, assign36040_e50790_d_n0, assign36040_e50790_d_n2, assign36040_e50790_d_n6, assign36040_e50790_d_n7,) = {
    if (var_guard1181 != 0.0) {
        let assign36040_e50788: f64 = (var_vrdr__blk1189 / var_ldrifte__blk1191);
        (assign36040_e50788, (var_vrdr__blk1189_dn0 / var_ldrifte__blk1191), (var_vrdr__blk1189_dn2 / var_ldrifte__blk1191), (var_vrdr__blk1189_dn6 / var_ldrifte__blk1191), (var_vrdr__blk1189_dn7 / var_ldrifte__blk1191),)
    } else {
        (var_edri__blk1195, var_edri__blk1195_dn0, var_edri__blk1195_dn2, var_edri__blk1195_dn6, var_edri__blk1195_dn7,)
    }
};
        var_edri__blk1195 = assign36040_e50790;
        var_edri__blk1195_dn0 = assign36040_e50790_d_n0;
        var_edri__blk1195_dn2 = assign36040_e50790_d_n2;
        var_edri__blk1195_dn6 = assign36040_e50790_d_n6;
        var_edri__blk1195_dn7 = assign36040_e50790_d_n7;
        var_edri__blk1195_rv = 0.0;

        *var_edri__blk1195_slot = var_edri__blk1195;
        *var_edri__blk1195_dn0_slot = var_edri__blk1195_dn0;
        *var_edri__blk1195_dn2_slot = var_edri__blk1195_dn2;
        *var_edri__blk1195_dn6_slot = var_edri__blk1195_dn6;
        *var_edri__blk1195_dn7_slot = var_edri__blk1195_dn7;
        *var_edri__blk1195_rv_slot = var_edri__blk1195_rv;
        *var_guard1177_slot = var_guard1177;
        *var_guard1177_rv_slot = var_guard1177_rv;
        *var_guard1178_slot = var_guard1178;
        *var_guard1178_rv_slot = var_guard1178_rv;
        *var_guard1181_slot = var_guard1181;
        *var_guard1181_rv_slot = var_guard1181_rv;
        *var_guard1201_slot = var_guard1201;
        *var_guard1201_rv_slot = var_guard1201_rv;
        *var_ldrifte__blk1191_slot = var_ldrifte__blk1191;
        *var_ldrifte__blk1191_rv_slot = var_ldrifte__blk1191_rv;
        *var_mks_rdrmue__blk1185_slot = var_mks_rdrmue__blk1185;
        *var_mks_rdrmue__blk1185_rv_slot = var_mks_rdrmue__blk1185_rv;
        *var_mks_rdrvmax__blk1186_slot = var_mks_rdrvmax__blk1186;
        *var_mks_rdrvmax__blk1186_rv_slot = var_mks_rdrvmax__blk1186_rv;
        *var_mu0__blk1193_slot = var_mu0__blk1193;
        *var_mu0__blk1193_dn0_slot = var_mu0__blk1193_dn0;
        *var_mu0__blk1193_dn10_slot = var_mu0__blk1193_dn10;
        *var_mu0__blk1193_dn11_slot = var_mu0__blk1193_dn11;
        *var_mu0__blk1193_dn12_slot = var_mu0__blk1193_dn12;
        *var_mu0__blk1193_dn17_slot = var_mu0__blk1193_dn17;
        *var_mu0__blk1193_dn2_slot = var_mu0__blk1193_dn2;
        *var_mu0__blk1193_dn6_slot = var_mu0__blk1193_dn6;
        *var_mu0__blk1193_dn7_slot = var_mu0__blk1193_dn7;
        *var_mu0__blk1193_rv_slot = var_mu0__blk1193_rv;
        *var_rdmod_slot = var_rdmod;
        *var_rdmod_rv_slot = var_rdmod_rv;
        *var_rdrmuele__blk1182_slot = var_rdrmuele__blk1182;
        *var_rdrmuele__blk1182_rv_slot = var_rdrmuele__blk1182_rv;
        *var_rdrvmaxle__blk1184_slot = var_rdrvmaxle__blk1184;
        *var_rdrvmaxle__blk1184_rv_slot = var_rdrvmaxle__blk1184_rv;
        *var_rdrvmaxwe__blk1183_slot = var_rdrvmaxwe__blk1183;
        *var_rdrvmaxwe__blk1183_rv_slot = var_rdrvmaxwe__blk1183_rv;
        *var_rrdrbb__blk1187_slot = var_rrdrbb__blk1187;
        *var_rrdrbb__blk1187_dn10_slot = var_rrdrbb__blk1187_dn10;
        *var_rrdrbb__blk1187_rv_slot = var_rrdrbb__blk1187_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn12_slot = var_t0_dn12;
        *var_t0_dn17_slot = var_t0_dn17;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_rv_slot = var_t0_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn17_slot = var_t1_dn17;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_rv_slot = var_t1_rv;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn12_slot = var_t2_dn12;
        *var_t2_dn17_slot = var_t2_dn17;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_rv_slot = var_t2_rv;
        *var_t4_slot = var_t4;
        *var_t4_dn0_slot = var_t4_dn0;
        *var_t4_dn10_slot = var_t4_dn10;
        *var_t4_dn11_slot = var_t4_dn11;
        *var_t4_dn12_slot = var_t4_dn12;
        *var_t4_dn17_slot = var_t4_dn17;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t4_rv_slot = var_t4_rv;
        *var_t5_slot = var_t5;
        *var_t5_dn0_slot = var_t5_dn0;
        *var_t5_dn10_slot = var_t5_dn10;
        *var_t5_dn11_slot = var_t5_dn11;
        *var_t5_dn12_slot = var_t5_dn12;
        *var_t5_dn17_slot = var_t5_dn17;
        *var_t5_dn2_slot = var_t5_dn2;
        *var_t5_dn6_slot = var_t5_dn6;
        *var_t5_dn7_slot = var_t5_dn7;
        *var_t5_rv_slot = var_t5_rv;
        *var_t6_slot = var_t6;
        *var_t6_dn0_slot = var_t6_dn0;
        *var_t6_dn10_slot = var_t6_dn10;
        *var_t6_dn11_slot = var_t6_dn11;
        *var_t6_dn12_slot = var_t6_dn12;
        *var_t6_dn17_slot = var_t6_dn17;
        *var_t6_dn2_slot = var_t6_dn2;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn7_slot = var_t6_dn7;
        *var_t6_rv_slot = var_t6_rv;
        *var_tratio__blk1190_slot = var_tratio__blk1190;
        *var_tratio__blk1190_dn10_slot = var_tratio__blk1190_dn10;
        *var_tratio__blk1190_rv_slot = var_tratio__blk1190_rv;
        *var_vmaxe__blk1194_slot = var_vmaxe__blk1194;
        *var_vmaxe__blk1194_dn0_slot = var_vmaxe__blk1194_dn0;
        *var_vmaxe__blk1194_dn10_slot = var_vmaxe__blk1194_dn10;
        *var_vmaxe__blk1194_dn11_slot = var_vmaxe__blk1194_dn11;
        *var_vmaxe__blk1194_dn12_slot = var_vmaxe__blk1194_dn12;
        *var_vmaxe__blk1194_dn17_slot = var_vmaxe__blk1194_dn17;
        *var_vmaxe__blk1194_dn2_slot = var_vmaxe__blk1194_dn2;
        *var_vmaxe__blk1194_dn6_slot = var_vmaxe__blk1194_dn6;
        *var_vmaxe__blk1194_dn7_slot = var_vmaxe__blk1194_dn7;
        *var_vmaxe__blk1194_rv_slot = var_vmaxe__blk1194_rv;
        *var_vrdr__blk1189_slot = var_vrdr__blk1189;
        *var_vrdr__blk1189_dn0_slot = var_vrdr__blk1189_dn0;
        *var_vrdr__blk1189_dn2_slot = var_vrdr__blk1189_dn2;
        *var_vrdr__blk1189_dn6_slot = var_vrdr__blk1189_dn6;
        *var_vrdr__blk1189_dn7_slot = var_vrdr__blk1189_dn7;
        *var_vrdr__blk1189_rv_slot = var_vrdr__blk1189_rv;
    }

    pub(super) fn stamp_reactive_block_128(
        p: &Parameters,
        var_edri__blk1195: f64,
        var_edri__blk1195_dn0: f64,
        var_edri__blk1195_dn2: f64,
        var_edri__blk1195_dn6: f64,
        var_edri__blk1195_dn7: f64,
        var_flg_nqs: f64,
        var_guard1181: f64,
        var_ldrifte__blk1191: f64,
        var_mode: f64,
        var_mu0__blk1193: f64,
        var_mu0__blk1193_dn0: f64,
        var_mu0__blk1193_dn10: f64,
        var_mu0__blk1193_dn11: f64,
        var_mu0__blk1193_dn12: f64,
        var_mu0__blk1193_dn17: f64,
        var_mu0__blk1193_dn2: f64,
        var_mu0__blk1193_dn6: f64,
        var_mu0__blk1193_dn7: f64,
        var_q_bt_ge: f64,
        var_q_bt_ge_dn0: f64,
        var_q_bt_ge_dn10: f64,
        var_q_bt_ge_dn11: f64,
        var_q_bt_ge_dn12: f64,
        var_q_bt_ge_dn17: f64,
        var_q_bt_ge_dn2: f64,
        var_q_bt_ge_dn6: f64,
        var_q_bt_ge_dn7: f64,
        var_q_bt_se: f64,
        var_q_bt_se_dn0: f64,
        var_q_bt_se_dn10: f64,
        var_q_bt_se_dn11: f64,
        var_q_bt_se_dn12: f64,
        var_q_bt_se_dn17: f64,
        var_q_bt_se_dn2: f64,
        var_q_bt_se_dn6: f64,
        var_q_bt_se_dn7: f64,
        var_qi_nqs: f64,
        var_qi_nqs_dn18: f64,
        var_rrdrbb__blk1187: f64,
        var_rrdrbb__blk1187_dn10: f64,
        var_vmaxe__blk1194: f64,
        var_vmaxe__blk1194_dn0: f64,
        var_vmaxe__blk1194_dn10: f64,
        var_vmaxe__blk1194_dn11: f64,
        var_vmaxe__blk1194_dn12: f64,
        var_vmaxe__blk1194_dn17: f64,
        var_vmaxe__blk1194_dn2: f64,
        var_vmaxe__blk1194_dn6: f64,
        var_vmaxe__blk1194_dn7: f64,
        var_vrdr__blk1189: f64,
        var_xd: f64,
        var_xd_dn0: f64,
        var_xd_dn10: f64,
        var_xd_dn11: f64,
        var_xd_dn12: f64,
        var_xd_dn17: f64,
        var_xd_dn2: f64,
        var_xd_dn6: f64,
        var_xd_dn7: f64,
        var_guard1202_slot: &mut f64,
        var_guard1202_rv_slot: &mut f64,
        var_guard1203_slot: &mut f64,
        var_guard1203_rv_slot: &mut f64,
        var_guard1204_slot: &mut f64,
        var_guard1204_rv_slot: &mut f64,
        var_guard1205_slot: &mut f64,
        var_guard1205_rv_slot: &mut f64,
        var_guard1206_slot: &mut f64,
        var_guard1206_rv_slot: &mut f64,
        var_guard1209_slot: &mut f64,
        var_guard1209_rv_slot: &mut f64,
        var_qb_nqs_slot: &mut f64,
        var_qb_nqs_dn13_slot: &mut f64,
        var_qb_nqs_rv_slot: &mut f64,
        var_qd_nqs_slot: &mut f64,
        var_qd_nqs_dn0_slot: &mut f64,
        var_qd_nqs_dn10_slot: &mut f64,
        var_qd_nqs_dn11_slot: &mut f64,
        var_qd_nqs_dn12_slot: &mut f64,
        var_qd_nqs_dn15_slot: &mut f64,
        var_qd_nqs_dn17_slot: &mut f64,
        var_qd_nqs_dn18_slot: &mut f64,
        var_qd_nqs_dn2_slot: &mut f64,
        var_qd_nqs_dn6_slot: &mut f64,
        var_qd_nqs_dn7_slot: &mut f64,
        var_qd_nqs_rv_slot: &mut f64,
        var_qdrat_slot: &mut f64,
        var_qdrat_dn0_slot: &mut f64,
        var_qdrat_dn10_slot: &mut f64,
        var_qdrat_dn11_slot: &mut f64,
        var_qdrat_dn12_slot: &mut f64,
        var_qdrat_dn17_slot: &mut f64,
        var_qdrat_dn2_slot: &mut f64,
        var_qdrat_dn6_slot: &mut f64,
        var_qdrat_dn7_slot: &mut f64,
        var_qdrat_rv_slot: &mut f64,
        var_qg_nqs_slot: &mut f64,
        var_qg_nqs_dn0_slot: &mut f64,
        var_qg_nqs_dn10_slot: &mut f64,
        var_qg_nqs_dn11_slot: &mut f64,
        var_qg_nqs_dn12_slot: &mut f64,
        var_qg_nqs_dn13_slot: &mut f64,
        var_qg_nqs_dn15_slot: &mut f64,
        var_qg_nqs_dn16_slot: &mut f64,
        var_qg_nqs_dn17_slot: &mut f64,
        var_qg_nqs_dn18_slot: &mut f64,
        var_qg_nqs_dn2_slot: &mut f64,
        var_qg_nqs_dn6_slot: &mut f64,
        var_qg_nqs_dn7_slot: &mut f64,
        var_qg_nqs_rv_slot: &mut f64,
        var_qs_nqs_slot: &mut f64,
        var_qs_nqs_dn0_slot: &mut f64,
        var_qs_nqs_dn10_slot: &mut f64,
        var_qs_nqs_dn11_slot: &mut f64,
        var_qs_nqs_dn12_slot: &mut f64,
        var_qs_nqs_dn16_slot: &mut f64,
        var_qs_nqs_dn17_slot: &mut f64,
        var_qs_nqs_dn18_slot: &mut f64,
        var_qs_nqs_dn2_slot: &mut f64,
        var_qs_nqs_dn6_slot: &mut f64,
        var_qs_nqs_dn7_slot: &mut f64,
        var_qs_nqs_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn17_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn12_slot: &mut f64,
        var_t2_dn17_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn12_slot: &mut f64,
        var_t3_dn17_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn0_slot: &mut f64,
        var_t4_dn10_slot: &mut f64,
        var_t4_dn11_slot: &mut f64,
        var_t4_dn12_slot: &mut f64,
        var_t4_dn17_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t4_rv_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_dn0_slot: &mut f64,
        var_t5_dn10_slot: &mut f64,
        var_t5_dn11_slot: &mut f64,
        var_t5_dn12_slot: &mut f64,
        var_t5_dn17_slot: &mut f64,
        var_t5_dn2_slot: &mut f64,
        var_t5_dn6_slot: &mut f64,
        var_t5_dn7_slot: &mut f64,
        var_t5_rv_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_dn0_slot: &mut f64,
        var_t6_dn10_slot: &mut f64,
        var_t6_dn11_slot: &mut f64,
        var_t6_dn12_slot: &mut f64,
        var_t6_dn17_slot: &mut f64,
        var_t6_dn2_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn7_slot: &mut f64,
        var_t6_rv_slot: &mut f64,
        var_vdri__blk1196_slot: &mut f64,
        var_vdri__blk1196_dn0_slot: &mut f64,
        var_vdri__blk1196_dn10_slot: &mut f64,
        var_vdri__blk1196_dn11_slot: &mut f64,
        var_vdri__blk1196_dn12_slot: &mut f64,
        var_vdri__blk1196_dn17_slot: &mut f64,
        var_vdri__blk1196_dn2_slot: &mut f64,
        var_vdri__blk1196_dn6_slot: &mut f64,
        var_vdri__blk1196_dn7_slot: &mut f64,
        var_vdri__blk1196_rv_slot: &mut f64,
    ) {
        let mut var_guard1202: f64 = *var_guard1202_slot;
        let mut var_guard1202_rv: f64 = *var_guard1202_rv_slot;
        let mut var_guard1203: f64 = *var_guard1203_slot;
        let mut var_guard1203_rv: f64 = *var_guard1203_rv_slot;
        let mut var_guard1204: f64 = *var_guard1204_slot;
        let mut var_guard1204_rv: f64 = *var_guard1204_rv_slot;
        let mut var_guard1205: f64 = *var_guard1205_slot;
        let mut var_guard1205_rv: f64 = *var_guard1205_rv_slot;
        let mut var_guard1206: f64 = *var_guard1206_slot;
        let mut var_guard1206_rv: f64 = *var_guard1206_rv_slot;
        let mut var_guard1209: f64 = *var_guard1209_slot;
        let mut var_guard1209_rv: f64 = *var_guard1209_rv_slot;
        let mut var_qb_nqs: f64 = *var_qb_nqs_slot;
        let mut var_qb_nqs_dn13: f64 = *var_qb_nqs_dn13_slot;
        let mut var_qb_nqs_rv: f64 = *var_qb_nqs_rv_slot;
        let mut var_qd_nqs: f64 = *var_qd_nqs_slot;
        let mut var_qd_nqs_dn0: f64 = *var_qd_nqs_dn0_slot;
        let mut var_qd_nqs_dn10: f64 = *var_qd_nqs_dn10_slot;
        let mut var_qd_nqs_dn11: f64 = *var_qd_nqs_dn11_slot;
        let mut var_qd_nqs_dn12: f64 = *var_qd_nqs_dn12_slot;
        let mut var_qd_nqs_dn15: f64 = *var_qd_nqs_dn15_slot;
        let mut var_qd_nqs_dn17: f64 = *var_qd_nqs_dn17_slot;
        let mut var_qd_nqs_dn18: f64 = *var_qd_nqs_dn18_slot;
        let mut var_qd_nqs_dn2: f64 = *var_qd_nqs_dn2_slot;
        let mut var_qd_nqs_dn6: f64 = *var_qd_nqs_dn6_slot;
        let mut var_qd_nqs_dn7: f64 = *var_qd_nqs_dn7_slot;
        let mut var_qd_nqs_rv: f64 = *var_qd_nqs_rv_slot;
        let mut var_qdrat: f64 = *var_qdrat_slot;
        let mut var_qdrat_dn0: f64 = *var_qdrat_dn0_slot;
        let mut var_qdrat_dn10: f64 = *var_qdrat_dn10_slot;
        let mut var_qdrat_dn11: f64 = *var_qdrat_dn11_slot;
        let mut var_qdrat_dn12: f64 = *var_qdrat_dn12_slot;
        let mut var_qdrat_dn17: f64 = *var_qdrat_dn17_slot;
        let mut var_qdrat_dn2: f64 = *var_qdrat_dn2_slot;
        let mut var_qdrat_dn6: f64 = *var_qdrat_dn6_slot;
        let mut var_qdrat_dn7: f64 = *var_qdrat_dn7_slot;
        let mut var_qdrat_rv: f64 = *var_qdrat_rv_slot;
        let mut var_qg_nqs: f64 = *var_qg_nqs_slot;
        let mut var_qg_nqs_dn0: f64 = *var_qg_nqs_dn0_slot;
        let mut var_qg_nqs_dn10: f64 = *var_qg_nqs_dn10_slot;
        let mut var_qg_nqs_dn11: f64 = *var_qg_nqs_dn11_slot;
        let mut var_qg_nqs_dn12: f64 = *var_qg_nqs_dn12_slot;
        let mut var_qg_nqs_dn13: f64 = *var_qg_nqs_dn13_slot;
        let mut var_qg_nqs_dn15: f64 = *var_qg_nqs_dn15_slot;
        let mut var_qg_nqs_dn16: f64 = *var_qg_nqs_dn16_slot;
        let mut var_qg_nqs_dn17: f64 = *var_qg_nqs_dn17_slot;
        let mut var_qg_nqs_dn18: f64 = *var_qg_nqs_dn18_slot;
        let mut var_qg_nqs_dn2: f64 = *var_qg_nqs_dn2_slot;
        let mut var_qg_nqs_dn6: f64 = *var_qg_nqs_dn6_slot;
        let mut var_qg_nqs_dn7: f64 = *var_qg_nqs_dn7_slot;
        let mut var_qg_nqs_rv: f64 = *var_qg_nqs_rv_slot;
        let mut var_qs_nqs: f64 = *var_qs_nqs_slot;
        let mut var_qs_nqs_dn0: f64 = *var_qs_nqs_dn0_slot;
        let mut var_qs_nqs_dn10: f64 = *var_qs_nqs_dn10_slot;
        let mut var_qs_nqs_dn11: f64 = *var_qs_nqs_dn11_slot;
        let mut var_qs_nqs_dn12: f64 = *var_qs_nqs_dn12_slot;
        let mut var_qs_nqs_dn16: f64 = *var_qs_nqs_dn16_slot;
        let mut var_qs_nqs_dn17: f64 = *var_qs_nqs_dn17_slot;
        let mut var_qs_nqs_dn18: f64 = *var_qs_nqs_dn18_slot;
        let mut var_qs_nqs_dn2: f64 = *var_qs_nqs_dn2_slot;
        let mut var_qs_nqs_dn6: f64 = *var_qs_nqs_dn6_slot;
        let mut var_qs_nqs_dn7: f64 = *var_qs_nqs_dn7_slot;
        let mut var_qs_nqs_rv: f64 = *var_qs_nqs_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn17: f64 = *var_t1_dn17_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn12: f64 = *var_t2_dn12_slot;
        let mut var_t2_dn17: f64 = *var_t2_dn17_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn12: f64 = *var_t3_dn12_slot;
        let mut var_t3_dn17: f64 = *var_t3_dn17_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn0: f64 = *var_t4_dn0_slot;
        let mut var_t4_dn10: f64 = *var_t4_dn10_slot;
        let mut var_t4_dn11: f64 = *var_t4_dn11_slot;
        let mut var_t4_dn12: f64 = *var_t4_dn12_slot;
        let mut var_t4_dn17: f64 = *var_t4_dn17_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t4_rv: f64 = *var_t4_rv_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_dn0: f64 = *var_t5_dn0_slot;
        let mut var_t5_dn10: f64 = *var_t5_dn10_slot;
        let mut var_t5_dn11: f64 = *var_t5_dn11_slot;
        let mut var_t5_dn12: f64 = *var_t5_dn12_slot;
        let mut var_t5_dn17: f64 = *var_t5_dn17_slot;
        let mut var_t5_dn2: f64 = *var_t5_dn2_slot;
        let mut var_t5_dn6: f64 = *var_t5_dn6_slot;
        let mut var_t5_dn7: f64 = *var_t5_dn7_slot;
        let mut var_t5_rv: f64 = *var_t5_rv_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_dn0: f64 = *var_t6_dn0_slot;
        let mut var_t6_dn10: f64 = *var_t6_dn10_slot;
        let mut var_t6_dn11: f64 = *var_t6_dn11_slot;
        let mut var_t6_dn12: f64 = *var_t6_dn12_slot;
        let mut var_t6_dn17: f64 = *var_t6_dn17_slot;
        let mut var_t6_dn2: f64 = *var_t6_dn2_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn7: f64 = *var_t6_dn7_slot;
        let mut var_t6_rv: f64 = *var_t6_rv_slot;
        let mut var_vdri__blk1196: f64 = *var_vdri__blk1196_slot;
        let mut var_vdri__blk1196_dn0: f64 = *var_vdri__blk1196_dn0_slot;
        let mut var_vdri__blk1196_dn10: f64 = *var_vdri__blk1196_dn10_slot;
        let mut var_vdri__blk1196_dn11: f64 = *var_vdri__blk1196_dn11_slot;
        let mut var_vdri__blk1196_dn12: f64 = *var_vdri__blk1196_dn12_slot;
        let mut var_vdri__blk1196_dn17: f64 = *var_vdri__blk1196_dn17_slot;
        let mut var_vdri__blk1196_dn2: f64 = *var_vdri__blk1196_dn2_slot;
        let mut var_vdri__blk1196_dn6: f64 = *var_vdri__blk1196_dn6_slot;
        let mut var_vdri__blk1196_dn7: f64 = *var_vdri__blk1196_dn7_slot;
        let mut var_vdri__blk1196_rv: f64 = *var_vdri__blk1196_rv_slot;

        let (assign36050_e50796, assign36050_e50796_d_n0, assign36050_e50796_d_n2, assign36050_e50796_d_n6, assign36050_e50796_d_n7, assign36050_e50796_d_n10, assign36050_e50796_d_n11, assign36050_e50796_d_n12, assign36050_e50796_d_n17,) = {
    if (var_guard1181 != 0.0) {
        let assign36050_e50794: f64 = (var_mu0__blk1193 * var_edri__blk1195);
        (assign36050_e50794, ((var_mu0__blk1193_dn0 * var_edri__blk1195) + (var_mu0__blk1193 * var_edri__blk1195_dn0)), ((var_mu0__blk1193_dn2 * var_edri__blk1195) + (var_mu0__blk1193 * var_edri__blk1195_dn2)), ((var_mu0__blk1193_dn6 * var_edri__blk1195) + (var_mu0__blk1193 * var_edri__blk1195_dn6)), ((var_mu0__blk1193_dn7 * var_edri__blk1195) + (var_mu0__blk1193 * var_edri__blk1195_dn7)), (var_mu0__blk1193_dn10 * var_edri__blk1195), (var_mu0__blk1193_dn11 * var_edri__blk1195), (var_mu0__blk1193_dn12 * var_edri__blk1195), (var_mu0__blk1193_dn17 * var_edri__blk1195),)
    } else {
        (var_vdri__blk1196, var_vdri__blk1196_dn0, var_vdri__blk1196_dn2, var_vdri__blk1196_dn6, var_vdri__blk1196_dn7, var_vdri__blk1196_dn10, var_vdri__blk1196_dn11, var_vdri__blk1196_dn12, var_vdri__blk1196_dn17,)
    }
};
        var_vdri__blk1196 = assign36050_e50796;
        var_vdri__blk1196_dn0 = assign36050_e50796_d_n0;
        var_vdri__blk1196_dn2 = assign36050_e50796_d_n2;
        var_vdri__blk1196_dn6 = assign36050_e50796_d_n6;
        var_vdri__blk1196_dn7 = assign36050_e50796_d_n7;
        var_vdri__blk1196_dn10 = assign36050_e50796_d_n10;
        var_vdri__blk1196_dn11 = assign36050_e50796_d_n11;
        var_vdri__blk1196_dn12 = assign36050_e50796_d_n12;
        var_vdri__blk1196_dn17 = assign36050_e50796_d_n17;
        var_vdri__blk1196_rv = 0.0;

        let assign36060_e50799: f64 = if var_vrdr__blk1189 >= 0.0 { 1.0 } else { 0.0 };
        var_guard1202 = assign36060_e50799;
        var_guard1202_rv = 0.0;

        let (assign36070_e50807, assign36070_e50807_d_n0, assign36070_e50807_d_n2, assign36070_e50807_d_n6, assign36070_e50807_d_n7, assign36070_e50807_d_n10, assign36070_e50807_d_n11, assign36070_e50807_d_n12, assign36070_e50807_d_n17,) = {
    if ((var_guard1181 != 0.0) && (var_guard1202 != 0.0)) {
        let assign36070_e50805: f64 = (var_vdri__blk1196 / var_vmaxe__blk1194);
        (assign36070_e50805, (((var_vdri__blk1196_dn0 * var_vmaxe__blk1194) - (var_vdri__blk1196 * var_vmaxe__blk1194_dn0)) / (var_vmaxe__blk1194 * var_vmaxe__blk1194)), (((var_vdri__blk1196_dn2 * var_vmaxe__blk1194) - (var_vdri__blk1196 * var_vmaxe__blk1194_dn2)) / (var_vmaxe__blk1194 * var_vmaxe__blk1194)), (((var_vdri__blk1196_dn6 * var_vmaxe__blk1194) - (var_vdri__blk1196 * var_vmaxe__blk1194_dn6)) / (var_vmaxe__blk1194 * var_vmaxe__blk1194)), (((var_vdri__blk1196_dn7 * var_vmaxe__blk1194) - (var_vdri__blk1196 * var_vmaxe__blk1194_dn7)) / (var_vmaxe__blk1194 * var_vmaxe__blk1194)), (((var_vdri__blk1196_dn10 * var_vmaxe__blk1194) - (var_vdri__blk1196 * var_vmaxe__blk1194_dn10)) / (var_vmaxe__blk1194 * var_vmaxe__blk1194)), (((var_vdri__blk1196_dn11 * var_vmaxe__blk1194) - (var_vdri__blk1196 * var_vmaxe__blk1194_dn11)) / (var_vmaxe__blk1194 * var_vmaxe__blk1194)), (((var_vdri__blk1196_dn12 * var_vmaxe__blk1194) - (var_vdri__blk1196 * var_vmaxe__blk1194_dn12)) / (var_vmaxe__blk1194 * var_vmaxe__blk1194)), (((var_vdri__blk1196_dn17 * var_vmaxe__blk1194) - (var_vdri__blk1196 * var_vmaxe__blk1194_dn17)) / (var_vmaxe__blk1194 * var_vmaxe__blk1194)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign36070_e50807;
        var_t1_dn0 = assign36070_e50807_d_n0;
        var_t1_dn2 = assign36070_e50807_d_n2;
        var_t1_dn6 = assign36070_e50807_d_n6;
        var_t1_dn7 = assign36070_e50807_d_n7;
        var_t1_dn10 = assign36070_e50807_d_n10;
        var_t1_dn11 = assign36070_e50807_d_n11;
        var_t1_dn12 = assign36070_e50807_d_n12;
        var_t1_dn17 = assign36070_e50807_d_n17;
        var_t1_rv = 0.0;

        let (assign36080_e50817, assign36080_e50817_d_n0, assign36080_e50817_d_n2, assign36080_e50817_d_n6, assign36080_e50817_d_n7, assign36080_e50817_d_n10, assign36080_e50817_d_n11, assign36080_e50817_d_n12, assign36080_e50817_d_n17,) = {
    if ((var_guard1181 != 0.0) && (var_guard1202 == 0.0)) {
        let assign36080_e50813: f64 = (-var_vdri__blk1196);
        let assign36080_e50815: f64 = (assign36080_e50813 / var_vmaxe__blk1194);
        (assign36080_e50815, ((((-var_vdri__blk1196_dn0) * var_vmaxe__blk1194) - (assign36080_e50813 * var_vmaxe__blk1194_dn0)) / (var_vmaxe__blk1194 * var_vmaxe__blk1194)), ((((-var_vdri__blk1196_dn2) * var_vmaxe__blk1194) - (assign36080_e50813 * var_vmaxe__blk1194_dn2)) / (var_vmaxe__blk1194 * var_vmaxe__blk1194)), ((((-var_vdri__blk1196_dn6) * var_vmaxe__blk1194) - (assign36080_e50813 * var_vmaxe__blk1194_dn6)) / (var_vmaxe__blk1194 * var_vmaxe__blk1194)), ((((-var_vdri__blk1196_dn7) * var_vmaxe__blk1194) - (assign36080_e50813 * var_vmaxe__blk1194_dn7)) / (var_vmaxe__blk1194 * var_vmaxe__blk1194)), ((((-var_vdri__blk1196_dn10) * var_vmaxe__blk1194) - (assign36080_e50813 * var_vmaxe__blk1194_dn10)) / (var_vmaxe__blk1194 * var_vmaxe__blk1194)), ((((-var_vdri__blk1196_dn11) * var_vmaxe__blk1194) - (assign36080_e50813 * var_vmaxe__blk1194_dn11)) / (var_vmaxe__blk1194 * var_vmaxe__blk1194)), ((((-var_vdri__blk1196_dn12) * var_vmaxe__blk1194) - (assign36080_e50813 * var_vmaxe__blk1194_dn12)) / (var_vmaxe__blk1194 * var_vmaxe__blk1194)), ((((-var_vdri__blk1196_dn17) * var_vmaxe__blk1194) - (assign36080_e50813 * var_vmaxe__blk1194_dn17)) / (var_vmaxe__blk1194 * var_vmaxe__blk1194)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign36080_e50817;
        var_t1_dn0 = assign36080_e50817_d_n0;
        var_t1_dn2 = assign36080_e50817_d_n2;
        var_t1_dn6 = assign36080_e50817_d_n6;
        var_t1_dn7 = assign36080_e50817_d_n7;
        var_t1_dn10 = assign36080_e50817_d_n10;
        var_t1_dn11 = assign36080_e50817_d_n11;
        var_t1_dn12 = assign36080_e50817_d_n12;
        var_t1_dn17 = assign36080_e50817_d_n17;
        var_t1_rv = 0.0;

        let assign36090_e50821: f64 = (10.0 * 2.220446049250313e-16);
        let assign36090_e50822: f64 = (1.0 - assign36090_e50821);
        let assign36090_e50829: f64 = (10.0 * 2.220446049250313e-16);
        let assign36090_e50830: f64 = (1.0 + assign36090_e50829);
        let assign36090_e50832: f64 = if ((assign36090_e50822 <= var_rrdrbb__blk1187) && (var_rrdrbb__blk1187 <= assign36090_e50830)) { 1.0 } else { 0.0 };
        var_guard1203 = assign36090_e50832;
        var_guard1203_rv = 0.0;

        let (assign36100_e50838, assign36100_e50838_d_n0, assign36100_e50838_d_n2, assign36100_e50838_d_n6, assign36100_e50838_d_n7, assign36100_e50838_d_n10, assign36100_e50838_d_n11, assign36100_e50838_d_n12, assign36100_e50838_d_n17,) = {
    if ((var_guard1181 != 0.0) && (var_guard1203 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn6, var_t3_dn7, var_t3_dn10, var_t3_dn11, var_t3_dn12, var_t3_dn17,)
    }
};
        var_t3 = assign36100_e50838;
        var_t3_dn0 = assign36100_e50838_d_n0;
        var_t3_dn2 = assign36100_e50838_d_n2;
        var_t3_dn6 = assign36100_e50838_d_n6;
        var_t3_dn7 = assign36100_e50838_d_n7;
        var_t3_dn10 = assign36100_e50838_d_n10;
        var_t3_dn11 = assign36100_e50838_d_n11;
        var_t3_dn12 = assign36100_e50838_d_n12;
        var_t3_dn17 = assign36100_e50838_d_n17;
        var_t3_rv = 0.0;

        let assign36110_e50842: f64 = (10.0 * 2.220446049250313e-16);
        let assign36110_e50843: f64 = (2.0 - assign36110_e50842);
        let assign36110_e50850: f64 = (10.0 * 2.220446049250313e-16);
        let assign36110_e50851: f64 = (2.0 + assign36110_e50850);
        let assign36110_e50853: f64 = if ((assign36110_e50843 <= var_rrdrbb__blk1187) && (var_rrdrbb__blk1187 <= assign36110_e50851)) { 1.0 } else { 0.0 };
        var_guard1204 = assign36110_e50853;
        var_guard1204_rv = 0.0;

        let (assign36120_e50862, assign36120_e50862_d_n0, assign36120_e50862_d_n2, assign36120_e50862_d_n6, assign36120_e50862_d_n7, assign36120_e50862_d_n10, assign36120_e50862_d_n11, assign36120_e50862_d_n12, assign36120_e50862_d_n17,) = {
    if (((var_guard1181 != 0.0) && (var_guard1203 == 0.0)) && (var_guard1204 != 0.0)) {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn6, var_t3_dn7, var_t3_dn10, var_t3_dn11, var_t3_dn12, var_t3_dn17,)
    }
};
        var_t3 = assign36120_e50862;
        var_t3_dn0 = assign36120_e50862_d_n0;
        var_t3_dn2 = assign36120_e50862_d_n2;
        var_t3_dn6 = assign36120_e50862_d_n6;
        var_t3_dn7 = assign36120_e50862_d_n7;
        var_t3_dn10 = assign36120_e50862_d_n10;
        var_t3_dn11 = assign36120_e50862_d_n11;
        var_t3_dn12 = assign36120_e50862_d_n12;
        var_t3_dn17 = assign36120_e50862_d_n17;
        var_t3_rv = 0.0;

        let (assign36130_e50876, assign36130_e50876_d_n0, assign36130_e50876_d_n2, assign36130_e50876_d_n6, assign36130_e50876_d_n7, assign36130_e50876_d_n10, assign36130_e50876_d_n11, assign36130_e50876_d_n12, assign36130_e50876_d_n17,) = {
    if (((var_guard1181 != 0.0) && (var_guard1203 == 0.0)) && (var_guard1204 == 0.0)) {
        let assign36130_e50873: f64 = (var_rrdrbb__blk1187 - 1.0);
        let assign36130_e50874: f64 = (var_t1).powf(assign36130_e50873);
        (assign36130_e50874, if 0.0 == 0.0 && ((assign36130_e50873) as f64).is_finite() && ((assign36130_e50873) as f64).fract() == 0.0 { if assign36130_e50873 == 0.0 { 0.0 } else { (assign36130_e50873 * ((var_t1).powf(assign36130_e50873 - 1.0) * var_t1_dn0)) } } else { (assign36130_e50874 * (assign36130_e50873 * (var_t1_dn0 / var_t1))) }, if 0.0 == 0.0 && ((assign36130_e50873) as f64).is_finite() && ((assign36130_e50873) as f64).fract() == 0.0 { if assign36130_e50873 == 0.0 { 0.0 } else { (assign36130_e50873 * ((var_t1).powf(assign36130_e50873 - 1.0) * var_t1_dn2)) } } else { (assign36130_e50874 * (assign36130_e50873 * (var_t1_dn2 / var_t1))) }, if 0.0 == 0.0 && ((assign36130_e50873) as f64).is_finite() && ((assign36130_e50873) as f64).fract() == 0.0 { if assign36130_e50873 == 0.0 { 0.0 } else { (assign36130_e50873 * ((var_t1).powf(assign36130_e50873 - 1.0) * var_t1_dn6)) } } else { (assign36130_e50874 * (assign36130_e50873 * (var_t1_dn6 / var_t1))) }, if 0.0 == 0.0 && ((assign36130_e50873) as f64).is_finite() && ((assign36130_e50873) as f64).fract() == 0.0 { if assign36130_e50873 == 0.0 { 0.0 } else { (assign36130_e50873 * ((var_t1).powf(assign36130_e50873 - 1.0) * var_t1_dn7)) } } else { (assign36130_e50874 * (assign36130_e50873 * (var_t1_dn7 / var_t1))) }, if var_rrdrbb__blk1187_dn10 == 0.0 && ((assign36130_e50873) as f64).is_finite() && ((assign36130_e50873) as f64).fract() == 0.0 { if assign36130_e50873 == 0.0 { 0.0 } else { (assign36130_e50873 * ((var_t1).powf(assign36130_e50873 - 1.0) * var_t1_dn10)) } } else { (assign36130_e50874 * ((var_rrdrbb__blk1187_dn10 * (var_t1).ln()) + (assign36130_e50873 * (var_t1_dn10 / var_t1)))) }, if 0.0 == 0.0 && ((assign36130_e50873) as f64).is_finite() && ((assign36130_e50873) as f64).fract() == 0.0 { if assign36130_e50873 == 0.0 { 0.0 } else { (assign36130_e50873 * ((var_t1).powf(assign36130_e50873 - 1.0) * var_t1_dn11)) } } else { (assign36130_e50874 * (assign36130_e50873 * (var_t1_dn11 / var_t1))) }, if 0.0 == 0.0 && ((assign36130_e50873) as f64).is_finite() && ((assign36130_e50873) as f64).fract() == 0.0 { if assign36130_e50873 == 0.0 { 0.0 } else { (assign36130_e50873 * ((var_t1).powf(assign36130_e50873 - 1.0) * var_t1_dn12)) } } else { (assign36130_e50874 * (assign36130_e50873 * (var_t1_dn12 / var_t1))) }, if 0.0 == 0.0 && ((assign36130_e50873) as f64).is_finite() && ((assign36130_e50873) as f64).fract() == 0.0 { if assign36130_e50873 == 0.0 { 0.0 } else { (assign36130_e50873 * ((var_t1).powf(assign36130_e50873 - 1.0) * var_t1_dn17)) } } else { (assign36130_e50874 * (assign36130_e50873 * (var_t1_dn17 / var_t1))) },)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn6, var_t3_dn7, var_t3_dn10, var_t3_dn11, var_t3_dn12, var_t3_dn17,)
    }
};
        var_t3 = assign36130_e50876;
        var_t3_dn0 = assign36130_e50876_d_n0;
        var_t3_dn2 = assign36130_e50876_d_n2;
        var_t3_dn6 = assign36130_e50876_d_n6;
        var_t3_dn7 = assign36130_e50876_d_n7;
        var_t3_dn10 = assign36130_e50876_d_n10;
        var_t3_dn11 = assign36130_e50876_d_n11;
        var_t3_dn12 = assign36130_e50876_d_n12;
        var_t3_dn17 = assign36130_e50876_d_n17;
        var_t3_rv = 0.0;

        let (assign36140_e50882, assign36140_e50882_d_n0, assign36140_e50882_d_n2, assign36140_e50882_d_n6, assign36140_e50882_d_n7, assign36140_e50882_d_n10, assign36140_e50882_d_n11, assign36140_e50882_d_n12, assign36140_e50882_d_n17,) = {
    if (var_guard1181 != 0.0) {
        let assign36140_e50880: f64 = (var_t1 * var_t3);
        (assign36140_e50880, ((var_t1_dn0 * var_t3) + (var_t1 * var_t3_dn0)), ((var_t1_dn2 * var_t3) + (var_t1 * var_t3_dn2)), ((var_t1_dn6 * var_t3) + (var_t1 * var_t3_dn6)), ((var_t1_dn7 * var_t3) + (var_t1 * var_t3_dn7)), ((var_t1_dn10 * var_t3) + (var_t1 * var_t3_dn10)), ((var_t1_dn11 * var_t3) + (var_t1 * var_t3_dn11)), ((var_t1_dn12 * var_t3) + (var_t1 * var_t3_dn12)), ((var_t1_dn17 * var_t3) + (var_t1 * var_t3_dn17)),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn6, var_t2_dn7, var_t2_dn10, var_t2_dn11, var_t2_dn12, var_t2_dn17,)
    }
};
        var_t2 = assign36140_e50882;
        var_t2_dn0 = assign36140_e50882_d_n0;
        var_t2_dn2 = assign36140_e50882_d_n2;
        var_t2_dn6 = assign36140_e50882_d_n6;
        var_t2_dn7 = assign36140_e50882_d_n7;
        var_t2_dn10 = assign36140_e50882_d_n10;
        var_t2_dn11 = assign36140_e50882_d_n11;
        var_t2_dn12 = assign36140_e50882_d_n12;
        var_t2_dn17 = assign36140_e50882_d_n17;
        var_t2_rv = 0.0;

        let (assign36150_e50888, assign36150_e50888_d_n0, assign36150_e50888_d_n2, assign36150_e50888_d_n6, assign36150_e50888_d_n7, assign36150_e50888_d_n10, assign36150_e50888_d_n11, assign36150_e50888_d_n12, assign36150_e50888_d_n17,) = {
    if (var_guard1181 != 0.0) {
        let assign36150_e50886: f64 = (1.0 + var_t2);
        (assign36150_e50886, var_t2_dn0, var_t2_dn2, var_t2_dn6, var_t2_dn7, var_t2_dn10, var_t2_dn11, var_t2_dn12, var_t2_dn17,)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn6, var_t4_dn7, var_t4_dn10, var_t4_dn11, var_t4_dn12, var_t4_dn17,)
    }
};
        var_t4 = assign36150_e50888;
        var_t4_dn0 = assign36150_e50888_d_n0;
        var_t4_dn2 = assign36150_e50888_d_n2;
        var_t4_dn6 = assign36150_e50888_d_n6;
        var_t4_dn7 = assign36150_e50888_d_n7;
        var_t4_dn10 = assign36150_e50888_d_n10;
        var_t4_dn11 = assign36150_e50888_d_n11;
        var_t4_dn12 = assign36150_e50888_d_n12;
        var_t4_dn17 = assign36150_e50888_d_n17;
        var_t4_rv = 0.0;

        let assign36160_e50892: f64 = (10.0 * 2.220446049250313e-16);
        let assign36160_e50893: f64 = (1.0 - assign36160_e50892);
        let assign36160_e50900: f64 = (10.0 * 2.220446049250313e-16);
        let assign36160_e50901: f64 = (1.0 + assign36160_e50900);
        let assign36160_e50903: f64 = if ((assign36160_e50893 <= var_rrdrbb__blk1187) && (var_rrdrbb__blk1187 <= assign36160_e50901)) { 1.0 } else { 0.0 };
        var_guard1205 = assign36160_e50903;
        var_guard1205_rv = 0.0;

        let (assign36170_e50911, assign36170_e50911_d_n0, assign36170_e50911_d_n2, assign36170_e50911_d_n6, assign36170_e50911_d_n7, assign36170_e50911_d_n10, assign36170_e50911_d_n11, assign36170_e50911_d_n12, assign36170_e50911_d_n17,) = {
    if ((var_guard1181 != 0.0) && (var_guard1205 != 0.0)) {
        let assign36170_e50909: f64 = (1.0 / var_t4);
        (assign36170_e50909, (-(var_t4_dn0 / (var_t4 * var_t4))), (-(var_t4_dn2 / (var_t4 * var_t4))), (-(var_t4_dn6 / (var_t4 * var_t4))), (-(var_t4_dn7 / (var_t4 * var_t4))), (-(var_t4_dn10 / (var_t4 * var_t4))), (-(var_t4_dn11 / (var_t4 * var_t4))), (-(var_t4_dn12 / (var_t4 * var_t4))), (-(var_t4_dn17 / (var_t4 * var_t4))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn6, var_t5_dn7, var_t5_dn10, var_t5_dn11, var_t5_dn12, var_t5_dn17,)
    }
};
        var_t5 = assign36170_e50911;
        var_t5_dn0 = assign36170_e50911_d_n0;
        var_t5_dn2 = assign36170_e50911_d_n2;
        var_t5_dn6 = assign36170_e50911_d_n6;
        var_t5_dn7 = assign36170_e50911_d_n7;
        var_t5_dn10 = assign36170_e50911_d_n10;
        var_t5_dn11 = assign36170_e50911_d_n11;
        var_t5_dn12 = assign36170_e50911_d_n12;
        var_t5_dn17 = assign36170_e50911_d_n17;
        var_t5_rv = 0.0;

        let assign36180_e50915: f64 = (10.0 * 2.220446049250313e-16);
        let assign36180_e50916: f64 = (2.0 - assign36180_e50915);
        let assign36180_e50923: f64 = (10.0 * 2.220446049250313e-16);
        let assign36180_e50924: f64 = (2.0 + assign36180_e50923);
        let assign36180_e50926: f64 = if ((assign36180_e50916 <= var_rrdrbb__blk1187) && (var_rrdrbb__blk1187 <= assign36180_e50924)) { 1.0 } else { 0.0 };
        var_guard1206 = assign36180_e50926;
        var_guard1206_rv = 0.0;

        let (assign36190_e50938, assign36190_e50938_d_n0, assign36190_e50938_d_n2, assign36190_e50938_d_n6, assign36190_e50938_d_n7, assign36190_e50938_d_n10, assign36190_e50938_d_n11, assign36190_e50938_d_n12, assign36190_e50938_d_n17,) = {
    if (((var_guard1181 != 0.0) && (var_guard1205 == 0.0)) && (var_guard1206 != 0.0)) {
        let assign36190_e50935: f64 = (var_t4).sqrt();
        let assign36190_e50936: f64 = (1.0 / assign36190_e50935);
        (assign36190_e50936, (-((var_t4_dn0 / (2.0 * assign36190_e50935)) / (assign36190_e50935 * assign36190_e50935))), (-((var_t4_dn2 / (2.0 * assign36190_e50935)) / (assign36190_e50935 * assign36190_e50935))), (-((var_t4_dn6 / (2.0 * assign36190_e50935)) / (assign36190_e50935 * assign36190_e50935))), (-((var_t4_dn7 / (2.0 * assign36190_e50935)) / (assign36190_e50935 * assign36190_e50935))), (-((var_t4_dn10 / (2.0 * assign36190_e50935)) / (assign36190_e50935 * assign36190_e50935))), (-((var_t4_dn11 / (2.0 * assign36190_e50935)) / (assign36190_e50935 * assign36190_e50935))), (-((var_t4_dn12 / (2.0 * assign36190_e50935)) / (assign36190_e50935 * assign36190_e50935))), (-((var_t4_dn17 / (2.0 * assign36190_e50935)) / (assign36190_e50935 * assign36190_e50935))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn6, var_t5_dn7, var_t5_dn10, var_t5_dn11, var_t5_dn12, var_t5_dn17,)
    }
};
        var_t5 = assign36190_e50938;
        var_t5_dn0 = assign36190_e50938_d_n0;
        var_t5_dn2 = assign36190_e50938_d_n2;
        var_t5_dn6 = assign36190_e50938_d_n6;
        var_t5_dn7 = assign36190_e50938_d_n7;
        var_t5_dn10 = assign36190_e50938_d_n10;
        var_t5_dn11 = assign36190_e50938_d_n11;
        var_t5_dn12 = assign36190_e50938_d_n12;
        var_t5_dn17 = assign36190_e50938_d_n17;
        var_t5_rv = 0.0;

        let (assign36200_e50955, assign36200_e50955_d_n0, assign36200_e50955_d_n2, assign36200_e50955_d_n6, assign36200_e50955_d_n7, assign36200_e50955_d_n10, assign36200_e50955_d_n11, assign36200_e50955_d_n12, assign36200_e50955_d_n17,) = {
    if (((var_guard1181 != 0.0) && (var_guard1205 == 0.0)) && (var_guard1206 == 0.0)) {
        let assign36200_e50948: f64 = (-1.0);
        let assign36200_e50950: f64 = (assign36200_e50948 / var_rrdrbb__blk1187);
        let assign36200_e50952: f64 = (assign36200_e50950 - 1.0);
        let assign36200_e50953: f64 = (var_t4).powf(assign36200_e50952);
        (assign36200_e50953, if 0.0 == 0.0 && ((assign36200_e50952) as f64).is_finite() && ((assign36200_e50952) as f64).fract() == 0.0 { if assign36200_e50952 == 0.0 { 0.0 } else { (assign36200_e50952 * ((var_t4).powf(assign36200_e50952 - 1.0) * var_t4_dn0)) } } else { (assign36200_e50953 * (assign36200_e50952 * (var_t4_dn0 / var_t4))) }, if 0.0 == 0.0 && ((assign36200_e50952) as f64).is_finite() && ((assign36200_e50952) as f64).fract() == 0.0 { if assign36200_e50952 == 0.0 { 0.0 } else { (assign36200_e50952 * ((var_t4).powf(assign36200_e50952 - 1.0) * var_t4_dn2)) } } else { (assign36200_e50953 * (assign36200_e50952 * (var_t4_dn2 / var_t4))) }, if 0.0 == 0.0 && ((assign36200_e50952) as f64).is_finite() && ((assign36200_e50952) as f64).fract() == 0.0 { if assign36200_e50952 == 0.0 { 0.0 } else { (assign36200_e50952 * ((var_t4).powf(assign36200_e50952 - 1.0) * var_t4_dn6)) } } else { (assign36200_e50953 * (assign36200_e50952 * (var_t4_dn6 / var_t4))) }, if 0.0 == 0.0 && ((assign36200_e50952) as f64).is_finite() && ((assign36200_e50952) as f64).fract() == 0.0 { if assign36200_e50952 == 0.0 { 0.0 } else { (assign36200_e50952 * ((var_t4).powf(assign36200_e50952 - 1.0) * var_t4_dn7)) } } else { (assign36200_e50953 * (assign36200_e50952 * (var_t4_dn7 / var_t4))) }, if (-((assign36200_e50948 * var_rrdrbb__blk1187_dn10) / (var_rrdrbb__blk1187 * var_rrdrbb__blk1187))) == 0.0 && ((assign36200_e50952) as f64).is_finite() && ((assign36200_e50952) as f64).fract() == 0.0 { if assign36200_e50952 == 0.0 { 0.0 } else { (assign36200_e50952 * ((var_t4).powf(assign36200_e50952 - 1.0) * var_t4_dn10)) } } else { (assign36200_e50953 * (((-((assign36200_e50948 * var_rrdrbb__blk1187_dn10) / (var_rrdrbb__blk1187 * var_rrdrbb__blk1187))) * (var_t4).ln()) + (assign36200_e50952 * (var_t4_dn10 / var_t4)))) }, if 0.0 == 0.0 && ((assign36200_e50952) as f64).is_finite() && ((assign36200_e50952) as f64).fract() == 0.0 { if assign36200_e50952 == 0.0 { 0.0 } else { (assign36200_e50952 * ((var_t4).powf(assign36200_e50952 - 1.0) * var_t4_dn11)) } } else { (assign36200_e50953 * (assign36200_e50952 * (var_t4_dn11 / var_t4))) }, if 0.0 == 0.0 && ((assign36200_e50952) as f64).is_finite() && ((assign36200_e50952) as f64).fract() == 0.0 { if assign36200_e50952 == 0.0 { 0.0 } else { (assign36200_e50952 * ((var_t4).powf(assign36200_e50952 - 1.0) * var_t4_dn12)) } } else { (assign36200_e50953 * (assign36200_e50952 * (var_t4_dn12 / var_t4))) }, if 0.0 == 0.0 && ((assign36200_e50952) as f64).is_finite() && ((assign36200_e50952) as f64).fract() == 0.0 { if assign36200_e50952 == 0.0 { 0.0 } else { (assign36200_e50952 * ((var_t4).powf(assign36200_e50952 - 1.0) * var_t4_dn17)) } } else { (assign36200_e50953 * (assign36200_e50952 * (var_t4_dn17 / var_t4))) },)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn6, var_t6_dn7, var_t6_dn10, var_t6_dn11, var_t6_dn12, var_t6_dn17,)
    }
};
        var_t6 = assign36200_e50955;
        var_t6_dn0 = assign36200_e50955_d_n0;
        var_t6_dn2 = assign36200_e50955_d_n2;
        var_t6_dn6 = assign36200_e50955_d_n6;
        var_t6_dn7 = assign36200_e50955_d_n7;
        var_t6_dn10 = assign36200_e50955_d_n10;
        var_t6_dn11 = assign36200_e50955_d_n11;
        var_t6_dn12 = assign36200_e50955_d_n12;
        var_t6_dn17 = assign36200_e50955_d_n17;
        var_t6_rv = 0.0;

        let (assign36210_e50967, assign36210_e50967_d_n0, assign36210_e50967_d_n2, assign36210_e50967_d_n6, assign36210_e50967_d_n7, assign36210_e50967_d_n10, assign36210_e50967_d_n11, assign36210_e50967_d_n12, assign36210_e50967_d_n17,) = {
    if (((var_guard1181 != 0.0) && (var_guard1205 == 0.0)) && (var_guard1206 == 0.0)) {
        let assign36210_e50965: f64 = (var_t4 * var_t6);
        (assign36210_e50965, ((var_t4_dn0 * var_t6) + (var_t4 * var_t6_dn0)), ((var_t4_dn2 * var_t6) + (var_t4 * var_t6_dn2)), ((var_t4_dn6 * var_t6) + (var_t4 * var_t6_dn6)), ((var_t4_dn7 * var_t6) + (var_t4 * var_t6_dn7)), ((var_t4_dn10 * var_t6) + (var_t4 * var_t6_dn10)), ((var_t4_dn11 * var_t6) + (var_t4 * var_t6_dn11)), ((var_t4_dn12 * var_t6) + (var_t4 * var_t6_dn12)), ((var_t4_dn17 * var_t6) + (var_t4 * var_t6_dn17)),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn6, var_t5_dn7, var_t5_dn10, var_t5_dn11, var_t5_dn12, var_t5_dn17,)
    }
};
        var_t5 = assign36210_e50967;
        var_t5_dn0 = assign36210_e50967_d_n0;
        var_t5_dn2 = assign36210_e50967_d_n2;
        var_t5_dn6 = assign36210_e50967_d_n6;
        var_t5_dn7 = assign36210_e50967_d_n7;
        var_t5_dn10 = assign36210_e50967_d_n10;
        var_t5_dn11 = assign36210_e50967_d_n11;
        var_t5_dn12 = assign36210_e50967_d_n12;
        var_t5_dn17 = assign36210_e50967_d_n17;
        var_t5_rv = 0.0;

        let (assign36230_e50979, assign36230_e50979_d_n0, assign36230_e50979_d_n2, assign36230_e50979_d_n6, assign36230_e50979_d_n7, assign36230_e50979_d_n10, assign36230_e50979_d_n11, assign36230_e50979_d_n12, assign36230_e50979_d_n17,) = {
    if (var_guard1181 != 0.0) {
        let assign36230_e50977: f64 = (1.6021918e-19 / var_ldrifte__blk1191);
        (assign36230_e50977, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign36230_e50979;
        var_t1_dn0 = assign36230_e50979_d_n0;
        var_t1_dn2 = assign36230_e50979_d_n2;
        var_t1_dn6 = assign36230_e50979_d_n6;
        var_t1_dn7 = assign36230_e50979_d_n7;
        var_t1_dn10 = assign36230_e50979_d_n10;
        var_t1_dn11 = assign36230_e50979_d_n11;
        var_t1_dn12 = assign36230_e50979_d_n12;
        var_t1_dn17 = assign36230_e50979_d_n17;
        var_t1_rv = 0.0;

        let assign36350_e51053: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard1209 = assign36350_e51053;
        var_guard1209_rv = 0.0;

        let (assign36400_e51096, assign36400_e51096_d_n0, assign36400_e51096_d_n2, assign36400_e51096_d_n6, assign36400_e51096_d_n7, assign36400_e51096_d_n10, assign36400_e51096_d_n11, assign36400_e51096_d_n12, assign36400_e51096_d_n17,) = {
    if ((var_guard1209 != 0.0) && (var_flg_nqs != 0.0)) {
        let (assign36400_e51094, assign36400_e51094_d_n0, assign36400_e51094_d_n2, assign36400_e51094_d_n6, assign36400_e51094_d_n7, assign36400_e51094_d_n10, assign36400_e51094_d_n11, assign36400_e51094_d_n12, assign36400_e51094_d_n17,) = {
            if (var_mode == 1.0) {
                (var_xd, var_xd_dn0, var_xd_dn2, var_xd_dn6, var_xd_dn7, var_xd_dn10, var_xd_dn11, var_xd_dn12, var_xd_dn17,)
            } else {
                let assign36400_e51093: f64 = (1.0 - var_xd);
                (assign36400_e51093, (-var_xd_dn0), (-var_xd_dn2), (-var_xd_dn6), (-var_xd_dn7), (-var_xd_dn10), (-var_xd_dn11), (-var_xd_dn12), (-var_xd_dn17),)
            }
        };
        (assign36400_e51094, assign36400_e51094_d_n0, assign36400_e51094_d_n2, assign36400_e51094_d_n6, assign36400_e51094_d_n7, assign36400_e51094_d_n10, assign36400_e51094_d_n11, assign36400_e51094_d_n12, assign36400_e51094_d_n17,)
    } else {
        (var_qdrat, var_qdrat_dn0, var_qdrat_dn2, var_qdrat_dn6, var_qdrat_dn7, var_qdrat_dn10, var_qdrat_dn11, var_qdrat_dn12, var_qdrat_dn17,)
    }
};
        var_qdrat = assign36400_e51096;
        var_qdrat_dn0 = assign36400_e51096_d_n0;
        var_qdrat_dn2 = assign36400_e51096_d_n2;
        var_qdrat_dn6 = assign36400_e51096_d_n6;
        var_qdrat_dn7 = assign36400_e51096_d_n7;
        var_qdrat_dn10 = assign36400_e51096_d_n10;
        var_qdrat_dn11 = assign36400_e51096_d_n11;
        var_qdrat_dn12 = assign36400_e51096_d_n12;
        var_qdrat_dn17 = assign36400_e51096_d_n17;
        var_qdrat_rv = 0.0;

        let (assign36430_e51126, assign36430_e51126_d_n0, assign36430_e51126_d_n2, assign36430_e51126_d_n6, assign36430_e51126_d_n7, assign36430_e51126_d_n10, assign36430_e51126_d_n11, assign36430_e51126_d_n12, assign36430_e51126_d_n15, assign36430_e51126_d_n17, assign36430_e51126_d_n18,) = {
    if ((var_guard1209 != 0.0) && (var_flg_nqs != 0.0)) {
        let assign36430_e51122: f64 = (var_qi_nqs * var_qdrat);
        let assign36430_e51124: f64 = (assign36430_e51122 + var_q_bt_se);
        (assign36430_e51124, ((var_qi_nqs * var_qdrat_dn0) + var_q_bt_se_dn0), ((var_qi_nqs * var_qdrat_dn2) + var_q_bt_se_dn2), ((var_qi_nqs * var_qdrat_dn6) + var_q_bt_se_dn6), ((var_qi_nqs * var_qdrat_dn7) + var_q_bt_se_dn7), ((var_qi_nqs * var_qdrat_dn10) + var_q_bt_se_dn10), ((var_qi_nqs * var_qdrat_dn11) + var_q_bt_se_dn11), ((var_qi_nqs * var_qdrat_dn12) + var_q_bt_se_dn12), 0.0, ((var_qi_nqs * var_qdrat_dn17) + var_q_bt_se_dn17), (var_qi_nqs_dn18 * var_qdrat),)
    } else {
        (var_qd_nqs, var_qd_nqs_dn0, var_qd_nqs_dn2, var_qd_nqs_dn6, var_qd_nqs_dn7, var_qd_nqs_dn10, var_qd_nqs_dn11, var_qd_nqs_dn12, var_qd_nqs_dn15, var_qd_nqs_dn17, var_qd_nqs_dn18,)
    }
};
        var_qd_nqs = assign36430_e51126;
        var_qd_nqs_dn0 = assign36430_e51126_d_n0;
        var_qd_nqs_dn2 = assign36430_e51126_d_n2;
        var_qd_nqs_dn6 = assign36430_e51126_d_n6;
        var_qd_nqs_dn7 = assign36430_e51126_d_n7;
        var_qd_nqs_dn10 = assign36430_e51126_d_n10;
        var_qd_nqs_dn11 = assign36430_e51126_d_n11;
        var_qd_nqs_dn12 = assign36430_e51126_d_n12;
        var_qd_nqs_dn15 = assign36430_e51126_d_n15;
        var_qd_nqs_dn17 = assign36430_e51126_d_n17;
        var_qd_nqs_dn18 = assign36430_e51126_d_n18;
        var_qd_nqs_rv = 0.0;

        let (assign36440_e51138, assign36440_e51138_d_n0, assign36440_e51138_d_n2, assign36440_e51138_d_n6, assign36440_e51138_d_n7, assign36440_e51138_d_n10, assign36440_e51138_d_n11, assign36440_e51138_d_n12, assign36440_e51138_d_n16, assign36440_e51138_d_n17, assign36440_e51138_d_n18,) = {
    if ((var_guard1209 != 0.0) && (var_flg_nqs != 0.0)) {
        let assign36440_e51133: f64 = (1.0 - var_qdrat);
        let assign36440_e51134: f64 = (var_qi_nqs * assign36440_e51133);
        let assign36440_e51136: f64 = (assign36440_e51134 + var_q_bt_se);
        (assign36440_e51136, ((var_qi_nqs * (-var_qdrat_dn0)) + var_q_bt_se_dn0), ((var_qi_nqs * (-var_qdrat_dn2)) + var_q_bt_se_dn2), ((var_qi_nqs * (-var_qdrat_dn6)) + var_q_bt_se_dn6), ((var_qi_nqs * (-var_qdrat_dn7)) + var_q_bt_se_dn7), ((var_qi_nqs * (-var_qdrat_dn10)) + var_q_bt_se_dn10), ((var_qi_nqs * (-var_qdrat_dn11)) + var_q_bt_se_dn11), ((var_qi_nqs * (-var_qdrat_dn12)) + var_q_bt_se_dn12), 0.0, ((var_qi_nqs * (-var_qdrat_dn17)) + var_q_bt_se_dn17), (var_qi_nqs_dn18 * assign36440_e51133),)
    } else {
        (var_qs_nqs, var_qs_nqs_dn0, var_qs_nqs_dn2, var_qs_nqs_dn6, var_qs_nqs_dn7, var_qs_nqs_dn10, var_qs_nqs_dn11, var_qs_nqs_dn12, var_qs_nqs_dn16, var_qs_nqs_dn17, var_qs_nqs_dn18,)
    }
};
        var_qs_nqs = assign36440_e51138;
        var_qs_nqs_dn0 = assign36440_e51138_d_n0;
        var_qs_nqs_dn2 = assign36440_e51138_d_n2;
        var_qs_nqs_dn6 = assign36440_e51138_d_n6;
        var_qs_nqs_dn7 = assign36440_e51138_d_n7;
        var_qs_nqs_dn10 = assign36440_e51138_d_n10;
        var_qs_nqs_dn11 = assign36440_e51138_d_n11;
        var_qs_nqs_dn12 = assign36440_e51138_d_n12;
        var_qs_nqs_dn16 = assign36440_e51138_d_n16;
        var_qs_nqs_dn17 = assign36440_e51138_d_n17;
        var_qs_nqs_dn18 = assign36440_e51138_d_n18;
        var_qs_nqs_rv = 0.0;

        let (assign36450_e51149, assign36450_e51149_d_n0, assign36450_e51149_d_n2, assign36450_e51149_d_n6, assign36450_e51149_d_n7, assign36450_e51149_d_n10, assign36450_e51149_d_n11, assign36450_e51149_d_n12, assign36450_e51149_d_n13, assign36450_e51149_d_n15, assign36450_e51149_d_n16, assign36450_e51149_d_n17, assign36450_e51149_d_n18,) = {
    if ((var_guard1209 != 0.0) && (var_flg_nqs != 0.0)) {
        let assign36450_e51143: f64 = (-var_qi_nqs);
        let assign36450_e51145: f64 = (assign36450_e51143 - var_qb_nqs);
        let assign36450_e51147: f64 = (assign36450_e51145 + var_q_bt_ge);
        (assign36450_e51147, var_q_bt_ge_dn0, var_q_bt_ge_dn2, var_q_bt_ge_dn6, var_q_bt_ge_dn7, var_q_bt_ge_dn10, var_q_bt_ge_dn11, var_q_bt_ge_dn12, (-var_qb_nqs_dn13), 0.0, 0.0, var_q_bt_ge_dn17, (-var_qi_nqs_dn18),)
    } else {
        (var_qg_nqs, var_qg_nqs_dn0, var_qg_nqs_dn2, var_qg_nqs_dn6, var_qg_nqs_dn7, var_qg_nqs_dn10, var_qg_nqs_dn11, var_qg_nqs_dn12, var_qg_nqs_dn13, var_qg_nqs_dn15, var_qg_nqs_dn16, var_qg_nqs_dn17, var_qg_nqs_dn18,)
    }
};
        var_qg_nqs = assign36450_e51149;
        var_qg_nqs_dn0 = assign36450_e51149_d_n0;
        var_qg_nqs_dn2 = assign36450_e51149_d_n2;
        var_qg_nqs_dn6 = assign36450_e51149_d_n6;
        var_qg_nqs_dn7 = assign36450_e51149_d_n7;
        var_qg_nqs_dn10 = assign36450_e51149_d_n10;
        var_qg_nqs_dn11 = assign36450_e51149_d_n11;
        var_qg_nqs_dn12 = assign36450_e51149_d_n12;
        var_qg_nqs_dn13 = assign36450_e51149_d_n13;
        var_qg_nqs_dn15 = assign36450_e51149_d_n15;
        var_qg_nqs_dn16 = assign36450_e51149_d_n16;
        var_qg_nqs_dn17 = assign36450_e51149_d_n17;
        var_qg_nqs_dn18 = assign36450_e51149_d_n18;
        var_qg_nqs_rv = 0.0;

        let (assign36480_e51170, assign36480_e51170_d_n0, assign36480_e51170_d_n2, assign36480_e51170_d_n6, assign36480_e51170_d_n7, assign36480_e51170_d_n10, assign36480_e51170_d_n11, assign36480_e51170_d_n12, assign36480_e51170_d_n15, assign36480_e51170_d_n17, assign36480_e51170_d_n18,) = {
    if ((var_guard1209 != 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qd_nqs, var_qd_nqs_dn0, var_qd_nqs_dn2, var_qd_nqs_dn6, var_qd_nqs_dn7, var_qd_nqs_dn10, var_qd_nqs_dn11, var_qd_nqs_dn12, var_qd_nqs_dn15, var_qd_nqs_dn17, var_qd_nqs_dn18,)
    }
};
        var_qd_nqs = assign36480_e51170;
        var_qd_nqs_dn0 = assign36480_e51170_d_n0;
        var_qd_nqs_dn2 = assign36480_e51170_d_n2;
        var_qd_nqs_dn6 = assign36480_e51170_d_n6;
        var_qd_nqs_dn7 = assign36480_e51170_d_n7;
        var_qd_nqs_dn10 = assign36480_e51170_d_n10;
        var_qd_nqs_dn11 = assign36480_e51170_d_n11;
        var_qd_nqs_dn12 = assign36480_e51170_d_n12;
        var_qd_nqs_dn15 = assign36480_e51170_d_n15;
        var_qd_nqs_dn17 = assign36480_e51170_d_n17;
        var_qd_nqs_dn18 = assign36480_e51170_d_n18;
        var_qd_nqs_rv = 0.0;

        let (assign36490_e51177, assign36490_e51177_d_n0, assign36490_e51177_d_n2, assign36490_e51177_d_n6, assign36490_e51177_d_n7, assign36490_e51177_d_n10, assign36490_e51177_d_n11, assign36490_e51177_d_n12, assign36490_e51177_d_n16, assign36490_e51177_d_n17, assign36490_e51177_d_n18,) = {
    if ((var_guard1209 != 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qs_nqs, var_qs_nqs_dn0, var_qs_nqs_dn2, var_qs_nqs_dn6, var_qs_nqs_dn7, var_qs_nqs_dn10, var_qs_nqs_dn11, var_qs_nqs_dn12, var_qs_nqs_dn16, var_qs_nqs_dn17, var_qs_nqs_dn18,)
    }
};
        var_qs_nqs = assign36490_e51177;
        var_qs_nqs_dn0 = assign36490_e51177_d_n0;
        var_qs_nqs_dn2 = assign36490_e51177_d_n2;
        var_qs_nqs_dn6 = assign36490_e51177_d_n6;
        var_qs_nqs_dn7 = assign36490_e51177_d_n7;
        var_qs_nqs_dn10 = assign36490_e51177_d_n10;
        var_qs_nqs_dn11 = assign36490_e51177_d_n11;
        var_qs_nqs_dn12 = assign36490_e51177_d_n12;
        var_qs_nqs_dn16 = assign36490_e51177_d_n16;
        var_qs_nqs_dn17 = assign36490_e51177_d_n17;
        var_qs_nqs_dn18 = assign36490_e51177_d_n18;
        var_qs_nqs_rv = 0.0;

        let (assign36500_e51184, assign36500_e51184_d_n0, assign36500_e51184_d_n2, assign36500_e51184_d_n6, assign36500_e51184_d_n7, assign36500_e51184_d_n10, assign36500_e51184_d_n11, assign36500_e51184_d_n12, assign36500_e51184_d_n13, assign36500_e51184_d_n15, assign36500_e51184_d_n16, assign36500_e51184_d_n17, assign36500_e51184_d_n18,) = {
    if ((var_guard1209 != 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qg_nqs, var_qg_nqs_dn0, var_qg_nqs_dn2, var_qg_nqs_dn6, var_qg_nqs_dn7, var_qg_nqs_dn10, var_qg_nqs_dn11, var_qg_nqs_dn12, var_qg_nqs_dn13, var_qg_nqs_dn15, var_qg_nqs_dn16, var_qg_nqs_dn17, var_qg_nqs_dn18,)
    }
};
        var_qg_nqs = assign36500_e51184;
        var_qg_nqs_dn0 = assign36500_e51184_d_n0;
        var_qg_nqs_dn2 = assign36500_e51184_d_n2;
        var_qg_nqs_dn6 = assign36500_e51184_d_n6;
        var_qg_nqs_dn7 = assign36500_e51184_d_n7;
        var_qg_nqs_dn10 = assign36500_e51184_d_n10;
        var_qg_nqs_dn11 = assign36500_e51184_d_n11;
        var_qg_nqs_dn12 = assign36500_e51184_d_n12;
        var_qg_nqs_dn13 = assign36500_e51184_d_n13;
        var_qg_nqs_dn15 = assign36500_e51184_d_n15;
        var_qg_nqs_dn16 = assign36500_e51184_d_n16;
        var_qg_nqs_dn17 = assign36500_e51184_d_n17;
        var_qg_nqs_dn18 = assign36500_e51184_d_n18;
        var_qg_nqs_rv = 0.0;

        let (assign36510_e51191, assign36510_e51191_d_n13,) = {
    if ((var_guard1209 != 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_qb_nqs, var_qb_nqs_dn13,)
    }
};
        var_qb_nqs = assign36510_e51191;
        var_qb_nqs_dn13 = assign36510_e51191_d_n13;
        var_qb_nqs_rv = 0.0;

        let (assign36600_e51275, assign36600_e51275_d_n0, assign36600_e51275_d_n2, assign36600_e51275_d_n6, assign36600_e51275_d_n7, assign36600_e51275_d_n10, assign36600_e51275_d_n11, assign36600_e51275_d_n12, assign36600_e51275_d_n13, assign36600_e51275_d_n15, assign36600_e51275_d_n16, assign36600_e51275_d_n17, assign36600_e51275_d_n18,) = {
    if ((var_guard1209 == 0.0) && (var_flg_nqs != 0.0)) {
        let assign36600_e51269: f64 = (-var_qd_nqs);
        let assign36600_e51271: f64 = (assign36600_e51269 - var_qs_nqs);
        let assign36600_e51273: f64 = (assign36600_e51271 - var_qb_nqs);
        (assign36600_e51273, ((-var_qd_nqs_dn0) - var_qs_nqs_dn0), ((-var_qd_nqs_dn2) - var_qs_nqs_dn2), ((-var_qd_nqs_dn6) - var_qs_nqs_dn6), ((-var_qd_nqs_dn7) - var_qs_nqs_dn7), ((-var_qd_nqs_dn10) - var_qs_nqs_dn10), ((-var_qd_nqs_dn11) - var_qs_nqs_dn11), ((-var_qd_nqs_dn12) - var_qs_nqs_dn12), (-var_qb_nqs_dn13), (-var_qd_nqs_dn15), (-var_qs_nqs_dn16), ((-var_qd_nqs_dn17) - var_qs_nqs_dn17), ((-var_qd_nqs_dn18) - var_qs_nqs_dn18),)
    } else {
        (var_qg_nqs, var_qg_nqs_dn0, var_qg_nqs_dn2, var_qg_nqs_dn6, var_qg_nqs_dn7, var_qg_nqs_dn10, var_qg_nqs_dn11, var_qg_nqs_dn12, var_qg_nqs_dn13, var_qg_nqs_dn15, var_qg_nqs_dn16, var_qg_nqs_dn17, var_qg_nqs_dn18,)
    }
};
        var_qg_nqs = assign36600_e51275;
        var_qg_nqs_dn0 = assign36600_e51275_d_n0;
        var_qg_nqs_dn2 = assign36600_e51275_d_n2;
        var_qg_nqs_dn6 = assign36600_e51275_d_n6;
        var_qg_nqs_dn7 = assign36600_e51275_d_n7;
        var_qg_nqs_dn10 = assign36600_e51275_d_n10;
        var_qg_nqs_dn11 = assign36600_e51275_d_n11;
        var_qg_nqs_dn12 = assign36600_e51275_d_n12;
        var_qg_nqs_dn13 = assign36600_e51275_d_n13;
        var_qg_nqs_dn15 = assign36600_e51275_d_n15;
        var_qg_nqs_dn16 = assign36600_e51275_d_n16;
        var_qg_nqs_dn17 = assign36600_e51275_d_n17;
        var_qg_nqs_dn18 = assign36600_e51275_d_n18;
        var_qg_nqs_rv = 0.0;

        let (assign36640_e51307, assign36640_e51307_d_n0, assign36640_e51307_d_n2, assign36640_e51307_d_n6, assign36640_e51307_d_n7, assign36640_e51307_d_n10, assign36640_e51307_d_n11, assign36640_e51307_d_n12, assign36640_e51307_d_n15, assign36640_e51307_d_n17, assign36640_e51307_d_n18,) = {
    if ((var_guard1209 == 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qd_nqs, var_qd_nqs_dn0, var_qd_nqs_dn2, var_qd_nqs_dn6, var_qd_nqs_dn7, var_qd_nqs_dn10, var_qd_nqs_dn11, var_qd_nqs_dn12, var_qd_nqs_dn15, var_qd_nqs_dn17, var_qd_nqs_dn18,)
    }
};
        var_qd_nqs = assign36640_e51307;
        var_qd_nqs_dn0 = assign36640_e51307_d_n0;
        var_qd_nqs_dn2 = assign36640_e51307_d_n2;
        var_qd_nqs_dn6 = assign36640_e51307_d_n6;
        var_qd_nqs_dn7 = assign36640_e51307_d_n7;
        var_qd_nqs_dn10 = assign36640_e51307_d_n10;
        var_qd_nqs_dn11 = assign36640_e51307_d_n11;
        var_qd_nqs_dn12 = assign36640_e51307_d_n12;
        var_qd_nqs_dn15 = assign36640_e51307_d_n15;
        var_qd_nqs_dn17 = assign36640_e51307_d_n17;
        var_qd_nqs_dn18 = assign36640_e51307_d_n18;
        var_qd_nqs_rv = 0.0;

        *var_guard1202_slot = var_guard1202;
        *var_guard1202_rv_slot = var_guard1202_rv;
        *var_guard1203_slot = var_guard1203;
        *var_guard1203_rv_slot = var_guard1203_rv;
        *var_guard1204_slot = var_guard1204;
        *var_guard1204_rv_slot = var_guard1204_rv;
        *var_guard1205_slot = var_guard1205;
        *var_guard1205_rv_slot = var_guard1205_rv;
        *var_guard1206_slot = var_guard1206;
        *var_guard1206_rv_slot = var_guard1206_rv;
        *var_guard1209_slot = var_guard1209;
        *var_guard1209_rv_slot = var_guard1209_rv;
        *var_qb_nqs_slot = var_qb_nqs;
        *var_qb_nqs_dn13_slot = var_qb_nqs_dn13;
        *var_qb_nqs_rv_slot = var_qb_nqs_rv;
        *var_qd_nqs_slot = var_qd_nqs;
        *var_qd_nqs_dn0_slot = var_qd_nqs_dn0;
        *var_qd_nqs_dn10_slot = var_qd_nqs_dn10;
        *var_qd_nqs_dn11_slot = var_qd_nqs_dn11;
        *var_qd_nqs_dn12_slot = var_qd_nqs_dn12;
        *var_qd_nqs_dn15_slot = var_qd_nqs_dn15;
        *var_qd_nqs_dn17_slot = var_qd_nqs_dn17;
        *var_qd_nqs_dn18_slot = var_qd_nqs_dn18;
        *var_qd_nqs_dn2_slot = var_qd_nqs_dn2;
        *var_qd_nqs_dn6_slot = var_qd_nqs_dn6;
        *var_qd_nqs_dn7_slot = var_qd_nqs_dn7;
        *var_qd_nqs_rv_slot = var_qd_nqs_rv;
        *var_qdrat_slot = var_qdrat;
        *var_qdrat_dn0_slot = var_qdrat_dn0;
        *var_qdrat_dn10_slot = var_qdrat_dn10;
        *var_qdrat_dn11_slot = var_qdrat_dn11;
        *var_qdrat_dn12_slot = var_qdrat_dn12;
        *var_qdrat_dn17_slot = var_qdrat_dn17;
        *var_qdrat_dn2_slot = var_qdrat_dn2;
        *var_qdrat_dn6_slot = var_qdrat_dn6;
        *var_qdrat_dn7_slot = var_qdrat_dn7;
        *var_qdrat_rv_slot = var_qdrat_rv;
        *var_qg_nqs_slot = var_qg_nqs;
        *var_qg_nqs_dn0_slot = var_qg_nqs_dn0;
        *var_qg_nqs_dn10_slot = var_qg_nqs_dn10;
        *var_qg_nqs_dn11_slot = var_qg_nqs_dn11;
        *var_qg_nqs_dn12_slot = var_qg_nqs_dn12;
        *var_qg_nqs_dn13_slot = var_qg_nqs_dn13;
        *var_qg_nqs_dn15_slot = var_qg_nqs_dn15;
        *var_qg_nqs_dn16_slot = var_qg_nqs_dn16;
        *var_qg_nqs_dn17_slot = var_qg_nqs_dn17;
        *var_qg_nqs_dn18_slot = var_qg_nqs_dn18;
        *var_qg_nqs_dn2_slot = var_qg_nqs_dn2;
        *var_qg_nqs_dn6_slot = var_qg_nqs_dn6;
        *var_qg_nqs_dn7_slot = var_qg_nqs_dn7;
        *var_qg_nqs_rv_slot = var_qg_nqs_rv;
        *var_qs_nqs_slot = var_qs_nqs;
        *var_qs_nqs_dn0_slot = var_qs_nqs_dn0;
        *var_qs_nqs_dn10_slot = var_qs_nqs_dn10;
        *var_qs_nqs_dn11_slot = var_qs_nqs_dn11;
        *var_qs_nqs_dn12_slot = var_qs_nqs_dn12;
        *var_qs_nqs_dn16_slot = var_qs_nqs_dn16;
        *var_qs_nqs_dn17_slot = var_qs_nqs_dn17;
        *var_qs_nqs_dn18_slot = var_qs_nqs_dn18;
        *var_qs_nqs_dn2_slot = var_qs_nqs_dn2;
        *var_qs_nqs_dn6_slot = var_qs_nqs_dn6;
        *var_qs_nqs_dn7_slot = var_qs_nqs_dn7;
        *var_qs_nqs_rv_slot = var_qs_nqs_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn17_slot = var_t1_dn17;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_rv_slot = var_t1_rv;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn12_slot = var_t2_dn12;
        *var_t2_dn17_slot = var_t2_dn17;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_rv_slot = var_t2_rv;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn12_slot = var_t3_dn12;
        *var_t3_dn17_slot = var_t3_dn17;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_rv_slot = var_t3_rv;
        *var_t4_slot = var_t4;
        *var_t4_dn0_slot = var_t4_dn0;
        *var_t4_dn10_slot = var_t4_dn10;
        *var_t4_dn11_slot = var_t4_dn11;
        *var_t4_dn12_slot = var_t4_dn12;
        *var_t4_dn17_slot = var_t4_dn17;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t4_rv_slot = var_t4_rv;
        *var_t5_slot = var_t5;
        *var_t5_dn0_slot = var_t5_dn0;
        *var_t5_dn10_slot = var_t5_dn10;
        *var_t5_dn11_slot = var_t5_dn11;
        *var_t5_dn12_slot = var_t5_dn12;
        *var_t5_dn17_slot = var_t5_dn17;
        *var_t5_dn2_slot = var_t5_dn2;
        *var_t5_dn6_slot = var_t5_dn6;
        *var_t5_dn7_slot = var_t5_dn7;
        *var_t5_rv_slot = var_t5_rv;
        *var_t6_slot = var_t6;
        *var_t6_dn0_slot = var_t6_dn0;
        *var_t6_dn10_slot = var_t6_dn10;
        *var_t6_dn11_slot = var_t6_dn11;
        *var_t6_dn12_slot = var_t6_dn12;
        *var_t6_dn17_slot = var_t6_dn17;
        *var_t6_dn2_slot = var_t6_dn2;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn7_slot = var_t6_dn7;
        *var_t6_rv_slot = var_t6_rv;
        *var_vdri__blk1196_slot = var_vdri__blk1196;
        *var_vdri__blk1196_dn0_slot = var_vdri__blk1196_dn0;
        *var_vdri__blk1196_dn10_slot = var_vdri__blk1196_dn10;
        *var_vdri__blk1196_dn11_slot = var_vdri__blk1196_dn11;
        *var_vdri__blk1196_dn12_slot = var_vdri__blk1196_dn12;
        *var_vdri__blk1196_dn17_slot = var_vdri__blk1196_dn17;
        *var_vdri__blk1196_dn2_slot = var_vdri__blk1196_dn2;
        *var_vdri__blk1196_dn6_slot = var_vdri__blk1196_dn6;
        *var_vdri__blk1196_dn7_slot = var_vdri__blk1196_dn7;
        *var_vdri__blk1196_rv_slot = var_vdri__blk1196_rv;
    }

    pub(super) fn stamp_reactive_block_129(
        p: &Parameters,
        var_cth: f64,
        var_flg_nqs: f64,
        var_guard1209: f64,
        var_ibdb: f64,
        var_ibdb_dn0: f64,
        var_ibdb_dn10: f64,
        var_ibdb_dn11: f64,
        var_ibdb_dn12: f64,
        var_ibdb_dn17: f64,
        var_ibdb_dn2: f64,
        var_ibdb_dn6: f64,
        var_ibdb_dn7: f64,
        var_ibsb: f64,
        var_ibsb_dn0: f64,
        var_ibsb_dn10: f64,
        var_ibsb_dn11: f64,
        var_ibsb_dn12: f64,
        var_ibsb_dn17: f64,
        var_ibsb_dn2: f64,
        var_ibsb_dn6: f64,
        var_ibsb_dn7: f64,
        var_isube: f64,
        var_isube_dn0: f64,
        var_isube_dn10: f64,
        var_isube_dn11: f64,
        var_isube_dn12: f64,
        var_isube_dn17: f64,
        var_isube_dn2: f64,
        var_isube_dn6: f64,
        var_isube_dn7: f64,
        var_mks_rth0: f64,
        var_mode: f64,
        var_qbd_s0: f64,
        var_qbd_s0_dn0: f64,
        var_qbd_s0_dn10: f64,
        var_qbd_s0_dn11: f64,
        var_qbd_s0_dn12: f64,
        var_qbd_s0_dn17: f64,
        var_qbd_s0_dn2: f64,
        var_qbd_s0_dn6: f64,
        var_qbd_s0_dn7: f64,
        var_qbs_s0: f64,
        var_qbs_s0_dn0: f64,
        var_qbs_s0_dn10: f64,
        var_qbs_s0_dn11: f64,
        var_qbs_s0_dn12: f64,
        var_qbs_s0_dn17: f64,
        var_qbs_s0_dn2: f64,
        var_qbs_s0_dn6: f64,
        var_qbs_s0_dn7: f64,
        var_qd_nqs: f64,
        var_qd_nqs_dn0: f64,
        var_qd_nqs_dn10: f64,
        var_qd_nqs_dn11: f64,
        var_qd_nqs_dn12: f64,
        var_qd_nqs_dn15: f64,
        var_qd_nqs_dn17: f64,
        var_qd_nqs_dn18: f64,
        var_qd_nqs_dn2: f64,
        var_qd_nqs_dn6: f64,
        var_qd_nqs_dn7: f64,
        var_qde: f64,
        var_qde_dn0: f64,
        var_qde_dn10: f64,
        var_qde_dn11: f64,
        var_qde_dn12: f64,
        var_qde_dn13: f64,
        var_qde_dn15: f64,
        var_qde_dn16: f64,
        var_qde_dn17: f64,
        var_qde_dn18: f64,
        var_qde_dn2: f64,
        var_qde_dn6: f64,
        var_qde_dn7: f64,
        var_qge: f64,
        var_qge_dn0: f64,
        var_qge_dn10: f64,
        var_qge_dn11: f64,
        var_qge_dn12: f64,
        var_qge_dn13: f64,
        var_qge_dn15: f64,
        var_qge_dn16: f64,
        var_qge_dn17: f64,
        var_qge_dn18: f64,
        var_qge_dn2: f64,
        var_qge_dn6: f64,
        var_qge_dn7: f64,
        var_qse: f64,
        var_qse_dn0: f64,
        var_qse_dn10: f64,
        var_qse_dn11: f64,
        var_qse_dn12: f64,
        var_qse_dn13: f64,
        var_qse_dn15: f64,
        var_qse_dn16: f64,
        var_qse_dn17: f64,
        var_qse_dn18: f64,
        var_qse_dn2: f64,
        var_qse_dn6: f64,
        var_qse_dn7: f64,
        var_cgdbd_slot: &mut f64,
        var_cgdbd_dn0_slot: &mut f64,
        var_cgdbd_dn10_slot: &mut f64,
        var_cgdbd_dn11_slot: &mut f64,
        var_cgdbd_dn12_slot: &mut f64,
        var_cgdbd_dn13_slot: &mut f64,
        var_cgdbd_dn15_slot: &mut f64,
        var_cgdbd_dn16_slot: &mut f64,
        var_cgdbd_dn17_slot: &mut f64,
        var_cgdbd_dn18_slot: &mut f64,
        var_cgdbd_dn2_slot: &mut f64,
        var_cgdbd_dn6_slot: &mut f64,
        var_cgdbd_dn7_slot: &mut f64,
        var_cgdbd_rv_slot: &mut f64,
        var_cgsbd_slot: &mut f64,
        var_cgsbd_dn0_slot: &mut f64,
        var_cgsbd_dn10_slot: &mut f64,
        var_cgsbd_dn11_slot: &mut f64,
        var_cgsbd_dn12_slot: &mut f64,
        var_cgsbd_dn13_slot: &mut f64,
        var_cgsbd_dn15_slot: &mut f64,
        var_cgsbd_dn16_slot: &mut f64,
        var_cgsbd_dn17_slot: &mut f64,
        var_cgsbd_dn18_slot: &mut f64,
        var_cgsbd_dn2_slot: &mut f64,
        var_cgsbd_dn6_slot: &mut f64,
        var_cgsbd_dn7_slot: &mut f64,
        var_cgsbd_rv_slot: &mut f64,
        var_cthe_slot: &mut f64,
        var_cthe_rv_slot: &mut f64,
        var_guard1214_slot: &mut f64,
        var_guard1214_rv_slot: &mut f64,
        var_guard1215_slot: &mut f64,
        var_guard1215_rv_slot: &mut f64,
        var_guard1216_slot: &mut f64,
        var_guard1216_rv_slot: &mut f64,
        var_guard1218_slot: &mut f64,
        var_guard1218_rv_slot: &mut f64,
        var_ibd_slot: &mut f64,
        var_ibd_dn0_slot: &mut f64,
        var_ibd_dn10_slot: &mut f64,
        var_ibd_dn11_slot: &mut f64,
        var_ibd_dn12_slot: &mut f64,
        var_ibd_dn17_slot: &mut f64,
        var_ibd_dn2_slot: &mut f64,
        var_ibd_dn6_slot: &mut f64,
        var_ibd_dn7_slot: &mut f64,
        var_ibd_rv_slot: &mut f64,
        var_ibs_slot: &mut f64,
        var_ibs_dn0_slot: &mut f64,
        var_ibs_dn10_slot: &mut f64,
        var_ibs_dn11_slot: &mut f64,
        var_ibs_dn12_slot: &mut f64,
        var_ibs_dn17_slot: &mut f64,
        var_ibs_dn2_slot: &mut f64,
        var_ibs_dn6_slot: &mut f64,
        var_ibs_dn7_slot: &mut f64,
        var_ibs_rv_slot: &mut f64,
        var_ids_slot: &mut f64,
        var_ids_dn0_slot: &mut f64,
        var_ids_dn10_slot: &mut f64,
        var_ids_dn11_slot: &mut f64,
        var_ids_dn12_slot: &mut f64,
        var_ids_dn17_slot: &mut f64,
        var_ids_dn2_slot: &mut f64,
        var_ids_dn6_slot: &mut f64,
        var_ids_dn7_slot: &mut f64,
        var_ids_rv_slot: &mut f64,
        var_idse_slot: &mut f64,
        var_idse_dn0_slot: &mut f64,
        var_idse_dn10_slot: &mut f64,
        var_idse_dn11_slot: &mut f64,
        var_idse_dn12_slot: &mut f64,
        var_idse_dn17_slot: &mut f64,
        var_idse_dn2_slot: &mut f64,
        var_idse_dn6_slot: &mut f64,
        var_idse_dn7_slot: &mut f64,
        var_idse_rv_slot: &mut f64,
        var_isub_slot: &mut f64,
        var_isub_dn0_slot: &mut f64,
        var_isub_dn10_slot: &mut f64,
        var_isub_dn11_slot: &mut f64,
        var_isub_dn12_slot: &mut f64,
        var_isub_dn17_slot: &mut f64,
        var_isub_dn2_slot: &mut f64,
        var_isub_dn6_slot: &mut f64,
        var_isub_dn7_slot: &mut f64,
        var_isub_rv_slot: &mut f64,
        var_qb_slot: &mut f64,
        var_qb_dn0_slot: &mut f64,
        var_qb_dn10_slot: &mut f64,
        var_qb_dn11_slot: &mut f64,
        var_qb_dn12_slot: &mut f64,
        var_qb_dn13_slot: &mut f64,
        var_qb_dn15_slot: &mut f64,
        var_qb_dn16_slot: &mut f64,
        var_qb_dn17_slot: &mut f64,
        var_qb_dn18_slot: &mut f64,
        var_qb_dn2_slot: &mut f64,
        var_qb_dn6_slot: &mut f64,
        var_qb_dn7_slot: &mut f64,
        var_qb_nqs_slot: &mut f64,
        var_qb_nqs_dn13_slot: &mut f64,
        var_qb_nqs_rv_slot: &mut f64,
        var_qb_rv_slot: &mut f64,
        var_qbd_slot: &mut f64,
        var_qbd_dn0_slot: &mut f64,
        var_qbd_dn10_slot: &mut f64,
        var_qbd_dn11_slot: &mut f64,
        var_qbd_dn12_slot: &mut f64,
        var_qbd_dn17_slot: &mut f64,
        var_qbd_dn2_slot: &mut f64,
        var_qbd_dn6_slot: &mut f64,
        var_qbd_dn7_slot: &mut f64,
        var_qbd_rv_slot: &mut f64,
        var_qbe_slot: &mut f64,
        var_qbe_dn0_slot: &mut f64,
        var_qbe_dn10_slot: &mut f64,
        var_qbe_dn11_slot: &mut f64,
        var_qbe_dn12_slot: &mut f64,
        var_qbe_dn13_slot: &mut f64,
        var_qbe_dn15_slot: &mut f64,
        var_qbe_dn16_slot: &mut f64,
        var_qbe_dn17_slot: &mut f64,
        var_qbe_dn18_slot: &mut f64,
        var_qbe_dn2_slot: &mut f64,
        var_qbe_dn6_slot: &mut f64,
        var_qbe_dn7_slot: &mut f64,
        var_qbe_rv_slot: &mut f64,
        var_qbs_slot: &mut f64,
        var_qbs_dn0_slot: &mut f64,
        var_qbs_dn10_slot: &mut f64,
        var_qbs_dn11_slot: &mut f64,
        var_qbs_dn12_slot: &mut f64,
        var_qbs_dn17_slot: &mut f64,
        var_qbs_dn2_slot: &mut f64,
        var_qbs_dn6_slot: &mut f64,
        var_qbs_dn7_slot: &mut f64,
        var_qbs_rv_slot: &mut f64,
        var_qd_slot: &mut f64,
        var_qd_dn0_slot: &mut f64,
        var_qd_dn10_slot: &mut f64,
        var_qd_dn11_slot: &mut f64,
        var_qd_dn12_slot: &mut f64,
        var_qd_dn13_slot: &mut f64,
        var_qd_dn15_slot: &mut f64,
        var_qd_dn16_slot: &mut f64,
        var_qd_dn17_slot: &mut f64,
        var_qd_dn18_slot: &mut f64,
        var_qd_dn2_slot: &mut f64,
        var_qd_dn6_slot: &mut f64,
        var_qd_dn7_slot: &mut f64,
        var_qd_rv_slot: &mut f64,
        var_qg_slot: &mut f64,
        var_qg_dn0_slot: &mut f64,
        var_qg_dn10_slot: &mut f64,
        var_qg_dn11_slot: &mut f64,
        var_qg_dn12_slot: &mut f64,
        var_qg_dn13_slot: &mut f64,
        var_qg_dn15_slot: &mut f64,
        var_qg_dn16_slot: &mut f64,
        var_qg_dn17_slot: &mut f64,
        var_qg_dn18_slot: &mut f64,
        var_qg_dn2_slot: &mut f64,
        var_qg_dn6_slot: &mut f64,
        var_qg_dn7_slot: &mut f64,
        var_qg_nqs_slot: &mut f64,
        var_qg_nqs_dn0_slot: &mut f64,
        var_qg_nqs_dn10_slot: &mut f64,
        var_qg_nqs_dn11_slot: &mut f64,
        var_qg_nqs_dn12_slot: &mut f64,
        var_qg_nqs_dn13_slot: &mut f64,
        var_qg_nqs_dn15_slot: &mut f64,
        var_qg_nqs_dn16_slot: &mut f64,
        var_qg_nqs_dn17_slot: &mut f64,
        var_qg_nqs_dn18_slot: &mut f64,
        var_qg_nqs_dn2_slot: &mut f64,
        var_qg_nqs_dn6_slot: &mut f64,
        var_qg_nqs_dn7_slot: &mut f64,
        var_qg_nqs_rv_slot: &mut f64,
        var_qg_rv_slot: &mut f64,
        var_qs_nqs_slot: &mut f64,
        var_qs_nqs_dn0_slot: &mut f64,
        var_qs_nqs_dn10_slot: &mut f64,
        var_qs_nqs_dn11_slot: &mut f64,
        var_qs_nqs_dn12_slot: &mut f64,
        var_qs_nqs_dn16_slot: &mut f64,
        var_qs_nqs_dn17_slot: &mut f64,
        var_qs_nqs_dn18_slot: &mut f64,
        var_qs_nqs_dn2_slot: &mut f64,
        var_qs_nqs_dn6_slot: &mut f64,
        var_qs_nqs_dn7_slot: &mut f64,
        var_qs_nqs_rv_slot: &mut f64,
    ) {
        let mut var_cgdbd: f64 = *var_cgdbd_slot;
        let mut var_cgdbd_dn0: f64 = *var_cgdbd_dn0_slot;
        let mut var_cgdbd_dn10: f64 = *var_cgdbd_dn10_slot;
        let mut var_cgdbd_dn11: f64 = *var_cgdbd_dn11_slot;
        let mut var_cgdbd_dn12: f64 = *var_cgdbd_dn12_slot;
        let mut var_cgdbd_dn13: f64 = *var_cgdbd_dn13_slot;
        let mut var_cgdbd_dn15: f64 = *var_cgdbd_dn15_slot;
        let mut var_cgdbd_dn16: f64 = *var_cgdbd_dn16_slot;
        let mut var_cgdbd_dn17: f64 = *var_cgdbd_dn17_slot;
        let mut var_cgdbd_dn18: f64 = *var_cgdbd_dn18_slot;
        let mut var_cgdbd_dn2: f64 = *var_cgdbd_dn2_slot;
        let mut var_cgdbd_dn6: f64 = *var_cgdbd_dn6_slot;
        let mut var_cgdbd_dn7: f64 = *var_cgdbd_dn7_slot;
        let mut var_cgdbd_rv: f64 = *var_cgdbd_rv_slot;
        let mut var_cgsbd: f64 = *var_cgsbd_slot;
        let mut var_cgsbd_dn0: f64 = *var_cgsbd_dn0_slot;
        let mut var_cgsbd_dn10: f64 = *var_cgsbd_dn10_slot;
        let mut var_cgsbd_dn11: f64 = *var_cgsbd_dn11_slot;
        let mut var_cgsbd_dn12: f64 = *var_cgsbd_dn12_slot;
        let mut var_cgsbd_dn13: f64 = *var_cgsbd_dn13_slot;
        let mut var_cgsbd_dn15: f64 = *var_cgsbd_dn15_slot;
        let mut var_cgsbd_dn16: f64 = *var_cgsbd_dn16_slot;
        let mut var_cgsbd_dn17: f64 = *var_cgsbd_dn17_slot;
        let mut var_cgsbd_dn18: f64 = *var_cgsbd_dn18_slot;
        let mut var_cgsbd_dn2: f64 = *var_cgsbd_dn2_slot;
        let mut var_cgsbd_dn6: f64 = *var_cgsbd_dn6_slot;
        let mut var_cgsbd_dn7: f64 = *var_cgsbd_dn7_slot;
        let mut var_cgsbd_rv: f64 = *var_cgsbd_rv_slot;
        let mut var_cthe: f64 = *var_cthe_slot;
        let mut var_cthe_rv: f64 = *var_cthe_rv_slot;
        let mut var_guard1214: f64 = *var_guard1214_slot;
        let mut var_guard1214_rv: f64 = *var_guard1214_rv_slot;
        let mut var_guard1215: f64 = *var_guard1215_slot;
        let mut var_guard1215_rv: f64 = *var_guard1215_rv_slot;
        let mut var_guard1216: f64 = *var_guard1216_slot;
        let mut var_guard1216_rv: f64 = *var_guard1216_rv_slot;
        let mut var_guard1218: f64 = *var_guard1218_slot;
        let mut var_guard1218_rv: f64 = *var_guard1218_rv_slot;
        let mut var_ibd: f64 = *var_ibd_slot;
        let mut var_ibd_dn0: f64 = *var_ibd_dn0_slot;
        let mut var_ibd_dn10: f64 = *var_ibd_dn10_slot;
        let mut var_ibd_dn11: f64 = *var_ibd_dn11_slot;
        let mut var_ibd_dn12: f64 = *var_ibd_dn12_slot;
        let mut var_ibd_dn17: f64 = *var_ibd_dn17_slot;
        let mut var_ibd_dn2: f64 = *var_ibd_dn2_slot;
        let mut var_ibd_dn6: f64 = *var_ibd_dn6_slot;
        let mut var_ibd_dn7: f64 = *var_ibd_dn7_slot;
        let mut var_ibd_rv: f64 = *var_ibd_rv_slot;
        let mut var_ibs: f64 = *var_ibs_slot;
        let mut var_ibs_dn0: f64 = *var_ibs_dn0_slot;
        let mut var_ibs_dn10: f64 = *var_ibs_dn10_slot;
        let mut var_ibs_dn11: f64 = *var_ibs_dn11_slot;
        let mut var_ibs_dn12: f64 = *var_ibs_dn12_slot;
        let mut var_ibs_dn17: f64 = *var_ibs_dn17_slot;
        let mut var_ibs_dn2: f64 = *var_ibs_dn2_slot;
        let mut var_ibs_dn6: f64 = *var_ibs_dn6_slot;
        let mut var_ibs_dn7: f64 = *var_ibs_dn7_slot;
        let mut var_ibs_rv: f64 = *var_ibs_rv_slot;
        let mut var_ids: f64 = *var_ids_slot;
        let mut var_ids_dn0: f64 = *var_ids_dn0_slot;
        let mut var_ids_dn10: f64 = *var_ids_dn10_slot;
        let mut var_ids_dn11: f64 = *var_ids_dn11_slot;
        let mut var_ids_dn12: f64 = *var_ids_dn12_slot;
        let mut var_ids_dn17: f64 = *var_ids_dn17_slot;
        let mut var_ids_dn2: f64 = *var_ids_dn2_slot;
        let mut var_ids_dn6: f64 = *var_ids_dn6_slot;
        let mut var_ids_dn7: f64 = *var_ids_dn7_slot;
        let mut var_ids_rv: f64 = *var_ids_rv_slot;
        let mut var_idse: f64 = *var_idse_slot;
        let mut var_idse_dn0: f64 = *var_idse_dn0_slot;
        let mut var_idse_dn10: f64 = *var_idse_dn10_slot;
        let mut var_idse_dn11: f64 = *var_idse_dn11_slot;
        let mut var_idse_dn12: f64 = *var_idse_dn12_slot;
        let mut var_idse_dn17: f64 = *var_idse_dn17_slot;
        let mut var_idse_dn2: f64 = *var_idse_dn2_slot;
        let mut var_idse_dn6: f64 = *var_idse_dn6_slot;
        let mut var_idse_dn7: f64 = *var_idse_dn7_slot;
        let mut var_idse_rv: f64 = *var_idse_rv_slot;
        let mut var_isub: f64 = *var_isub_slot;
        let mut var_isub_dn0: f64 = *var_isub_dn0_slot;
        let mut var_isub_dn10: f64 = *var_isub_dn10_slot;
        let mut var_isub_dn11: f64 = *var_isub_dn11_slot;
        let mut var_isub_dn12: f64 = *var_isub_dn12_slot;
        let mut var_isub_dn17: f64 = *var_isub_dn17_slot;
        let mut var_isub_dn2: f64 = *var_isub_dn2_slot;
        let mut var_isub_dn6: f64 = *var_isub_dn6_slot;
        let mut var_isub_dn7: f64 = *var_isub_dn7_slot;
        let mut var_isub_rv: f64 = *var_isub_rv_slot;
        let mut var_qb: f64 = *var_qb_slot;
        let mut var_qb_dn0: f64 = *var_qb_dn0_slot;
        let mut var_qb_dn10: f64 = *var_qb_dn10_slot;
        let mut var_qb_dn11: f64 = *var_qb_dn11_slot;
        let mut var_qb_dn12: f64 = *var_qb_dn12_slot;
        let mut var_qb_dn13: f64 = *var_qb_dn13_slot;
        let mut var_qb_dn15: f64 = *var_qb_dn15_slot;
        let mut var_qb_dn16: f64 = *var_qb_dn16_slot;
        let mut var_qb_dn17: f64 = *var_qb_dn17_slot;
        let mut var_qb_dn18: f64 = *var_qb_dn18_slot;
        let mut var_qb_dn2: f64 = *var_qb_dn2_slot;
        let mut var_qb_dn6: f64 = *var_qb_dn6_slot;
        let mut var_qb_dn7: f64 = *var_qb_dn7_slot;
        let mut var_qb_nqs: f64 = *var_qb_nqs_slot;
        let mut var_qb_nqs_dn13: f64 = *var_qb_nqs_dn13_slot;
        let mut var_qb_nqs_rv: f64 = *var_qb_nqs_rv_slot;
        let mut var_qb_rv: f64 = *var_qb_rv_slot;
        let mut var_qbd: f64 = *var_qbd_slot;
        let mut var_qbd_dn0: f64 = *var_qbd_dn0_slot;
        let mut var_qbd_dn10: f64 = *var_qbd_dn10_slot;
        let mut var_qbd_dn11: f64 = *var_qbd_dn11_slot;
        let mut var_qbd_dn12: f64 = *var_qbd_dn12_slot;
        let mut var_qbd_dn17: f64 = *var_qbd_dn17_slot;
        let mut var_qbd_dn2: f64 = *var_qbd_dn2_slot;
        let mut var_qbd_dn6: f64 = *var_qbd_dn6_slot;
        let mut var_qbd_dn7: f64 = *var_qbd_dn7_slot;
        let mut var_qbd_rv: f64 = *var_qbd_rv_slot;
        let mut var_qbe: f64 = *var_qbe_slot;
        let mut var_qbe_dn0: f64 = *var_qbe_dn0_slot;
        let mut var_qbe_dn10: f64 = *var_qbe_dn10_slot;
        let mut var_qbe_dn11: f64 = *var_qbe_dn11_slot;
        let mut var_qbe_dn12: f64 = *var_qbe_dn12_slot;
        let mut var_qbe_dn13: f64 = *var_qbe_dn13_slot;
        let mut var_qbe_dn15: f64 = *var_qbe_dn15_slot;
        let mut var_qbe_dn16: f64 = *var_qbe_dn16_slot;
        let mut var_qbe_dn17: f64 = *var_qbe_dn17_slot;
        let mut var_qbe_dn18: f64 = *var_qbe_dn18_slot;
        let mut var_qbe_dn2: f64 = *var_qbe_dn2_slot;
        let mut var_qbe_dn6: f64 = *var_qbe_dn6_slot;
        let mut var_qbe_dn7: f64 = *var_qbe_dn7_slot;
        let mut var_qbe_rv: f64 = *var_qbe_rv_slot;
        let mut var_qbs: f64 = *var_qbs_slot;
        let mut var_qbs_dn0: f64 = *var_qbs_dn0_slot;
        let mut var_qbs_dn10: f64 = *var_qbs_dn10_slot;
        let mut var_qbs_dn11: f64 = *var_qbs_dn11_slot;
        let mut var_qbs_dn12: f64 = *var_qbs_dn12_slot;
        let mut var_qbs_dn17: f64 = *var_qbs_dn17_slot;
        let mut var_qbs_dn2: f64 = *var_qbs_dn2_slot;
        let mut var_qbs_dn6: f64 = *var_qbs_dn6_slot;
        let mut var_qbs_dn7: f64 = *var_qbs_dn7_slot;
        let mut var_qbs_rv: f64 = *var_qbs_rv_slot;
        let mut var_qd: f64 = *var_qd_slot;
        let mut var_qd_dn0: f64 = *var_qd_dn0_slot;
        let mut var_qd_dn10: f64 = *var_qd_dn10_slot;
        let mut var_qd_dn11: f64 = *var_qd_dn11_slot;
        let mut var_qd_dn12: f64 = *var_qd_dn12_slot;
        let mut var_qd_dn13: f64 = *var_qd_dn13_slot;
        let mut var_qd_dn15: f64 = *var_qd_dn15_slot;
        let mut var_qd_dn16: f64 = *var_qd_dn16_slot;
        let mut var_qd_dn17: f64 = *var_qd_dn17_slot;
        let mut var_qd_dn18: f64 = *var_qd_dn18_slot;
        let mut var_qd_dn2: f64 = *var_qd_dn2_slot;
        let mut var_qd_dn6: f64 = *var_qd_dn6_slot;
        let mut var_qd_dn7: f64 = *var_qd_dn7_slot;
        let mut var_qd_rv: f64 = *var_qd_rv_slot;
        let mut var_qg: f64 = *var_qg_slot;
        let mut var_qg_dn0: f64 = *var_qg_dn0_slot;
        let mut var_qg_dn10: f64 = *var_qg_dn10_slot;
        let mut var_qg_dn11: f64 = *var_qg_dn11_slot;
        let mut var_qg_dn12: f64 = *var_qg_dn12_slot;
        let mut var_qg_dn13: f64 = *var_qg_dn13_slot;
        let mut var_qg_dn15: f64 = *var_qg_dn15_slot;
        let mut var_qg_dn16: f64 = *var_qg_dn16_slot;
        let mut var_qg_dn17: f64 = *var_qg_dn17_slot;
        let mut var_qg_dn18: f64 = *var_qg_dn18_slot;
        let mut var_qg_dn2: f64 = *var_qg_dn2_slot;
        let mut var_qg_dn6: f64 = *var_qg_dn6_slot;
        let mut var_qg_dn7: f64 = *var_qg_dn7_slot;
        let mut var_qg_nqs: f64 = *var_qg_nqs_slot;
        let mut var_qg_nqs_dn0: f64 = *var_qg_nqs_dn0_slot;
        let mut var_qg_nqs_dn10: f64 = *var_qg_nqs_dn10_slot;
        let mut var_qg_nqs_dn11: f64 = *var_qg_nqs_dn11_slot;
        let mut var_qg_nqs_dn12: f64 = *var_qg_nqs_dn12_slot;
        let mut var_qg_nqs_dn13: f64 = *var_qg_nqs_dn13_slot;
        let mut var_qg_nqs_dn15: f64 = *var_qg_nqs_dn15_slot;
        let mut var_qg_nqs_dn16: f64 = *var_qg_nqs_dn16_slot;
        let mut var_qg_nqs_dn17: f64 = *var_qg_nqs_dn17_slot;
        let mut var_qg_nqs_dn18: f64 = *var_qg_nqs_dn18_slot;
        let mut var_qg_nqs_dn2: f64 = *var_qg_nqs_dn2_slot;
        let mut var_qg_nqs_dn6: f64 = *var_qg_nqs_dn6_slot;
        let mut var_qg_nqs_dn7: f64 = *var_qg_nqs_dn7_slot;
        let mut var_qg_nqs_rv: f64 = *var_qg_nqs_rv_slot;
        let mut var_qg_rv: f64 = *var_qg_rv_slot;
        let mut var_qs_nqs: f64 = *var_qs_nqs_slot;
        let mut var_qs_nqs_dn0: f64 = *var_qs_nqs_dn0_slot;
        let mut var_qs_nqs_dn10: f64 = *var_qs_nqs_dn10_slot;
        let mut var_qs_nqs_dn11: f64 = *var_qs_nqs_dn11_slot;
        let mut var_qs_nqs_dn12: f64 = *var_qs_nqs_dn12_slot;
        let mut var_qs_nqs_dn16: f64 = *var_qs_nqs_dn16_slot;
        let mut var_qs_nqs_dn17: f64 = *var_qs_nqs_dn17_slot;
        let mut var_qs_nqs_dn18: f64 = *var_qs_nqs_dn18_slot;
        let mut var_qs_nqs_dn2: f64 = *var_qs_nqs_dn2_slot;
        let mut var_qs_nqs_dn6: f64 = *var_qs_nqs_dn6_slot;
        let mut var_qs_nqs_dn7: f64 = *var_qs_nqs_dn7_slot;
        let mut var_qs_nqs_rv: f64 = *var_qs_nqs_rv_slot;

        let (assign36650_e51315, assign36650_e51315_d_n0, assign36650_e51315_d_n2, assign36650_e51315_d_n6, assign36650_e51315_d_n7, assign36650_e51315_d_n10, assign36650_e51315_d_n11, assign36650_e51315_d_n12, assign36650_e51315_d_n16, assign36650_e51315_d_n17, assign36650_e51315_d_n18,) = {
    if ((var_guard1209 == 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qs_nqs, var_qs_nqs_dn0, var_qs_nqs_dn2, var_qs_nqs_dn6, var_qs_nqs_dn7, var_qs_nqs_dn10, var_qs_nqs_dn11, var_qs_nqs_dn12, var_qs_nqs_dn16, var_qs_nqs_dn17, var_qs_nqs_dn18,)
    }
};
        var_qs_nqs = assign36650_e51315;
        var_qs_nqs_dn0 = assign36650_e51315_d_n0;
        var_qs_nqs_dn2 = assign36650_e51315_d_n2;
        var_qs_nqs_dn6 = assign36650_e51315_d_n6;
        var_qs_nqs_dn7 = assign36650_e51315_d_n7;
        var_qs_nqs_dn10 = assign36650_e51315_d_n10;
        var_qs_nqs_dn11 = assign36650_e51315_d_n11;
        var_qs_nqs_dn12 = assign36650_e51315_d_n12;
        var_qs_nqs_dn16 = assign36650_e51315_d_n16;
        var_qs_nqs_dn17 = assign36650_e51315_d_n17;
        var_qs_nqs_dn18 = assign36650_e51315_d_n18;
        var_qs_nqs_rv = 0.0;

        let (assign36660_e51323, assign36660_e51323_d_n0, assign36660_e51323_d_n2, assign36660_e51323_d_n6, assign36660_e51323_d_n7, assign36660_e51323_d_n10, assign36660_e51323_d_n11, assign36660_e51323_d_n12, assign36660_e51323_d_n13, assign36660_e51323_d_n15, assign36660_e51323_d_n16, assign36660_e51323_d_n17, assign36660_e51323_d_n18,) = {
    if ((var_guard1209 == 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qg_nqs, var_qg_nqs_dn0, var_qg_nqs_dn2, var_qg_nqs_dn6, var_qg_nqs_dn7, var_qg_nqs_dn10, var_qg_nqs_dn11, var_qg_nqs_dn12, var_qg_nqs_dn13, var_qg_nqs_dn15, var_qg_nqs_dn16, var_qg_nqs_dn17, var_qg_nqs_dn18,)
    }
};
        var_qg_nqs = assign36660_e51323;
        var_qg_nqs_dn0 = assign36660_e51323_d_n0;
        var_qg_nqs_dn2 = assign36660_e51323_d_n2;
        var_qg_nqs_dn6 = assign36660_e51323_d_n6;
        var_qg_nqs_dn7 = assign36660_e51323_d_n7;
        var_qg_nqs_dn10 = assign36660_e51323_d_n10;
        var_qg_nqs_dn11 = assign36660_e51323_d_n11;
        var_qg_nqs_dn12 = assign36660_e51323_d_n12;
        var_qg_nqs_dn13 = assign36660_e51323_d_n13;
        var_qg_nqs_dn15 = assign36660_e51323_d_n15;
        var_qg_nqs_dn16 = assign36660_e51323_d_n16;
        var_qg_nqs_dn17 = assign36660_e51323_d_n17;
        var_qg_nqs_dn18 = assign36660_e51323_d_n18;
        var_qg_nqs_rv = 0.0;

        let (assign36670_e51331, assign36670_e51331_d_n13,) = {
    if ((var_guard1209 == 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_qb_nqs, var_qb_nqs_dn13,)
    }
};
        var_qb_nqs = assign36670_e51331;
        var_qb_nqs_dn13 = assign36670_e51331_d_n13;
        var_qb_nqs_rv = 0.0;

        let assign36700_e51336: f64 = if var_mode == 1.0 { 1.0 } else { 0.0 };
        var_guard1214 = assign36700_e51336;
        var_guard1214_rv = 0.0;

        let (assign36710_e51340, assign36710_e51340_d_n0, assign36710_e51340_d_n2, assign36710_e51340_d_n6, assign36710_e51340_d_n7, assign36710_e51340_d_n10, assign36710_e51340_d_n11, assign36710_e51340_d_n12, assign36710_e51340_d_n17,) = {
    if (var_guard1214 != 0.0) {
        (var_idse, var_idse_dn0, var_idse_dn2, var_idse_dn6, var_idse_dn7, var_idse_dn10, var_idse_dn11, var_idse_dn12, var_idse_dn17,)
    } else {
        (var_ids, var_ids_dn0, var_ids_dn2, var_ids_dn6, var_ids_dn7, var_ids_dn10, var_ids_dn11, var_ids_dn12, var_ids_dn17,)
    }
};
        var_ids = assign36710_e51340;
        var_ids_dn0 = assign36710_e51340_d_n0;
        var_ids_dn2 = assign36710_e51340_d_n2;
        var_ids_dn6 = assign36710_e51340_d_n6;
        var_ids_dn7 = assign36710_e51340_d_n7;
        var_ids_dn10 = assign36710_e51340_d_n10;
        var_ids_dn11 = assign36710_e51340_d_n11;
        var_ids_dn12 = assign36710_e51340_d_n12;
        var_ids_dn17 = assign36710_e51340_d_n17;
        var_ids_rv = 0.0;

        let (assign36720_e51344, assign36720_e51344_d_n0, assign36720_e51344_d_n2, assign36720_e51344_d_n6, assign36720_e51344_d_n7, assign36720_e51344_d_n10, assign36720_e51344_d_n11, assign36720_e51344_d_n12, assign36720_e51344_d_n17,) = {
    if (var_guard1214 != 0.0) {
        (var_isube, var_isube_dn0, var_isube_dn2, var_isube_dn6, var_isube_dn7, var_isube_dn10, var_isube_dn11, var_isube_dn12, var_isube_dn17,)
    } else {
        (var_isub, var_isub_dn0, var_isub_dn2, var_isub_dn6, var_isub_dn7, var_isub_dn10, var_isub_dn11, var_isub_dn12, var_isub_dn17,)
    }
};
        var_isub = assign36720_e51344;
        var_isub_dn0 = assign36720_e51344_d_n0;
        var_isub_dn2 = assign36720_e51344_d_n2;
        var_isub_dn6 = assign36720_e51344_d_n6;
        var_isub_dn7 = assign36720_e51344_d_n7;
        var_isub_dn10 = assign36720_e51344_d_n10;
        var_isub_dn11 = assign36720_e51344_d_n11;
        var_isub_dn12 = assign36720_e51344_d_n12;
        var_isub_dn17 = assign36720_e51344_d_n17;
        var_isub_rv = 0.0;

        let (assign36740_e51354, assign36740_e51354_d_n0, assign36740_e51354_d_n2, assign36740_e51354_d_n6, assign36740_e51354_d_n7, assign36740_e51354_d_n10, assign36740_e51354_d_n11, assign36740_e51354_d_n12, assign36740_e51354_d_n13, assign36740_e51354_d_n15, assign36740_e51354_d_n16, assign36740_e51354_d_n17, assign36740_e51354_d_n18,) = {
    if (var_guard1214 != 0.0) {
        let assign36740_e51352: f64 = (var_qge + var_qg_nqs);
        (assign36740_e51352, (var_qge_dn0 + var_qg_nqs_dn0), (var_qge_dn2 + var_qg_nqs_dn2), (var_qge_dn6 + var_qg_nqs_dn6), (var_qge_dn7 + var_qg_nqs_dn7), (var_qge_dn10 + var_qg_nqs_dn10), (var_qge_dn11 + var_qg_nqs_dn11), (var_qge_dn12 + var_qg_nqs_dn12), (var_qge_dn13 + var_qg_nqs_dn13), (var_qge_dn15 + var_qg_nqs_dn15), (var_qge_dn16 + var_qg_nqs_dn16), (var_qge_dn17 + var_qg_nqs_dn17), (var_qge_dn18 + var_qg_nqs_dn18),)
    } else {
        (var_qg, var_qg_dn0, var_qg_dn2, var_qg_dn6, var_qg_dn7, var_qg_dn10, var_qg_dn11, var_qg_dn12, var_qg_dn13, var_qg_dn15, var_qg_dn16, var_qg_dn17, var_qg_dn18,)
    }
};
        var_qg = assign36740_e51354;
        var_qg_dn0 = assign36740_e51354_d_n0;
        var_qg_dn2 = assign36740_e51354_d_n2;
        var_qg_dn6 = assign36740_e51354_d_n6;
        var_qg_dn7 = assign36740_e51354_d_n7;
        var_qg_dn10 = assign36740_e51354_d_n10;
        var_qg_dn11 = assign36740_e51354_d_n11;
        var_qg_dn12 = assign36740_e51354_d_n12;
        var_qg_dn13 = assign36740_e51354_d_n13;
        var_qg_dn15 = assign36740_e51354_d_n15;
        var_qg_dn16 = assign36740_e51354_d_n16;
        var_qg_dn17 = assign36740_e51354_d_n17;
        var_qg_dn18 = assign36740_e51354_d_n18;
        var_qg_rv = 0.0;

        let (assign36750_e51360, assign36750_e51360_d_n0, assign36750_e51360_d_n2, assign36750_e51360_d_n6, assign36750_e51360_d_n7, assign36750_e51360_d_n10, assign36750_e51360_d_n11, assign36750_e51360_d_n12, assign36750_e51360_d_n13, assign36750_e51360_d_n15, assign36750_e51360_d_n16, assign36750_e51360_d_n17, assign36750_e51360_d_n18,) = {
    if (var_guard1214 != 0.0) {
        let assign36750_e51358: f64 = (var_qde + var_qd_nqs);
        (assign36750_e51358, (var_qde_dn0 + var_qd_nqs_dn0), (var_qde_dn2 + var_qd_nqs_dn2), (var_qde_dn6 + var_qd_nqs_dn6), (var_qde_dn7 + var_qd_nqs_dn7), (var_qde_dn10 + var_qd_nqs_dn10), (var_qde_dn11 + var_qd_nqs_dn11), (var_qde_dn12 + var_qd_nqs_dn12), var_qde_dn13, (var_qde_dn15 + var_qd_nqs_dn15), var_qde_dn16, (var_qde_dn17 + var_qd_nqs_dn17), (var_qde_dn18 + var_qd_nqs_dn18),)
    } else {
        (var_qd, var_qd_dn0, var_qd_dn2, var_qd_dn6, var_qd_dn7, var_qd_dn10, var_qd_dn11, var_qd_dn12, var_qd_dn13, var_qd_dn15, var_qd_dn16, var_qd_dn17, var_qd_dn18,)
    }
};
        var_qd = assign36750_e51360;
        var_qd_dn0 = assign36750_e51360_d_n0;
        var_qd_dn2 = assign36750_e51360_d_n2;
        var_qd_dn6 = assign36750_e51360_d_n6;
        var_qd_dn7 = assign36750_e51360_d_n7;
        var_qd_dn10 = assign36750_e51360_d_n10;
        var_qd_dn11 = assign36750_e51360_d_n11;
        var_qd_dn12 = assign36750_e51360_d_n12;
        var_qd_dn13 = assign36750_e51360_d_n13;
        var_qd_dn15 = assign36750_e51360_d_n15;
        var_qd_dn16 = assign36750_e51360_d_n16;
        var_qd_dn17 = assign36750_e51360_d_n17;
        var_qd_dn18 = assign36750_e51360_d_n18;
        var_qd_rv = 0.0;

        let (assign36770_e51375, assign36770_e51375_d_n0, assign36770_e51375_d_n2, assign36770_e51375_d_n6, assign36770_e51375_d_n7, assign36770_e51375_d_n10, assign36770_e51375_d_n11, assign36770_e51375_d_n12, assign36770_e51375_d_n13, assign36770_e51375_d_n15, assign36770_e51375_d_n16, assign36770_e51375_d_n17, assign36770_e51375_d_n18,) = {
    if (var_guard1214 != 0.0) {
        let assign36770_e51370: f64 = (var_qge + var_qde);
        let assign36770_e51372: f64 = (assign36770_e51370 + var_qse);
        let assign36770_e51373: f64 = (-assign36770_e51372);
        (assign36770_e51373, (-((var_qge_dn0 + var_qde_dn0) + var_qse_dn0)), (-((var_qge_dn2 + var_qde_dn2) + var_qse_dn2)), (-((var_qge_dn6 + var_qde_dn6) + var_qse_dn6)), (-((var_qge_dn7 + var_qde_dn7) + var_qse_dn7)), (-((var_qge_dn10 + var_qde_dn10) + var_qse_dn10)), (-((var_qge_dn11 + var_qde_dn11) + var_qse_dn11)), (-((var_qge_dn12 + var_qde_dn12) + var_qse_dn12)), (-((var_qge_dn13 + var_qde_dn13) + var_qse_dn13)), (-((var_qge_dn15 + var_qde_dn15) + var_qse_dn15)), (-((var_qge_dn16 + var_qde_dn16) + var_qse_dn16)), (-((var_qge_dn17 + var_qde_dn17) + var_qse_dn17)), (-((var_qge_dn18 + var_qde_dn18) + var_qse_dn18)),)
    } else {
        (var_qbe, var_qbe_dn0, var_qbe_dn2, var_qbe_dn6, var_qbe_dn7, var_qbe_dn10, var_qbe_dn11, var_qbe_dn12, var_qbe_dn13, var_qbe_dn15, var_qbe_dn16, var_qbe_dn17, var_qbe_dn18,)
    }
};
        var_qbe = assign36770_e51375;
        var_qbe_dn0 = assign36770_e51375_d_n0;
        var_qbe_dn2 = assign36770_e51375_d_n2;
        var_qbe_dn6 = assign36770_e51375_d_n6;
        var_qbe_dn7 = assign36770_e51375_d_n7;
        var_qbe_dn10 = assign36770_e51375_d_n10;
        var_qbe_dn11 = assign36770_e51375_d_n11;
        var_qbe_dn12 = assign36770_e51375_d_n12;
        var_qbe_dn13 = assign36770_e51375_d_n13;
        var_qbe_dn15 = assign36770_e51375_d_n15;
        var_qbe_dn16 = assign36770_e51375_d_n16;
        var_qbe_dn17 = assign36770_e51375_d_n17;
        var_qbe_dn18 = assign36770_e51375_d_n18;
        var_qbe_rv = 0.0;

        let (assign36780_e51381, assign36780_e51381_d_n0, assign36780_e51381_d_n2, assign36780_e51381_d_n6, assign36780_e51381_d_n7, assign36780_e51381_d_n10, assign36780_e51381_d_n11, assign36780_e51381_d_n12, assign36780_e51381_d_n13, assign36780_e51381_d_n15, assign36780_e51381_d_n16, assign36780_e51381_d_n17, assign36780_e51381_d_n18,) = {
    if (var_guard1214 != 0.0) {
        let assign36780_e51379: f64 = (var_qbe + var_qb_nqs);
        (assign36780_e51379, var_qbe_dn0, var_qbe_dn2, var_qbe_dn6, var_qbe_dn7, var_qbe_dn10, var_qbe_dn11, var_qbe_dn12, (var_qbe_dn13 + var_qb_nqs_dn13), var_qbe_dn15, var_qbe_dn16, var_qbe_dn17, var_qbe_dn18,)
    } else {
        (var_qb, var_qb_dn0, var_qb_dn2, var_qb_dn6, var_qb_dn7, var_qb_dn10, var_qb_dn11, var_qb_dn12, var_qb_dn13, var_qb_dn15, var_qb_dn16, var_qb_dn17, var_qb_dn18,)
    }
};
        var_qb = assign36780_e51381;
        var_qb_dn0 = assign36780_e51381_d_n0;
        var_qb_dn2 = assign36780_e51381_d_n2;
        var_qb_dn6 = assign36780_e51381_d_n6;
        var_qb_dn7 = assign36780_e51381_d_n7;
        var_qb_dn10 = assign36780_e51381_d_n10;
        var_qb_dn11 = assign36780_e51381_d_n11;
        var_qb_dn12 = assign36780_e51381_d_n12;
        var_qb_dn13 = assign36780_e51381_d_n13;
        var_qb_dn15 = assign36780_e51381_d_n15;
        var_qb_dn16 = assign36780_e51381_d_n16;
        var_qb_dn17 = assign36780_e51381_d_n17;
        var_qb_dn18 = assign36780_e51381_d_n18;
        var_qb_rv = 0.0;

        let (assign36790_e51387, assign36790_e51387_d_n0, assign36790_e51387_d_n2, assign36790_e51387_d_n6, assign36790_e51387_d_n7, assign36790_e51387_d_n10, assign36790_e51387_d_n11, assign36790_e51387_d_n12, assign36790_e51387_d_n17,) = {
    if (var_guard1214 == 0.0) {
        let assign36790_e51385: f64 = (-var_idse);
        (assign36790_e51385, (-var_idse_dn0), (-var_idse_dn2), (-var_idse_dn6), (-var_idse_dn7), (-var_idse_dn10), (-var_idse_dn11), (-var_idse_dn12), (-var_idse_dn17),)
    } else {
        (var_ids, var_ids_dn0, var_ids_dn2, var_ids_dn6, var_ids_dn7, var_ids_dn10, var_ids_dn11, var_ids_dn12, var_ids_dn17,)
    }
};
        var_ids = assign36790_e51387;
        var_ids_dn0 = assign36790_e51387_d_n0;
        var_ids_dn2 = assign36790_e51387_d_n2;
        var_ids_dn6 = assign36790_e51387_d_n6;
        var_ids_dn7 = assign36790_e51387_d_n7;
        var_ids_dn10 = assign36790_e51387_d_n10;
        var_ids_dn11 = assign36790_e51387_d_n11;
        var_ids_dn12 = assign36790_e51387_d_n12;
        var_ids_dn17 = assign36790_e51387_d_n17;
        var_ids_rv = 0.0;

        let (assign36810_e51397, assign36810_e51397_d_n0, assign36810_e51397_d_n2, assign36810_e51397_d_n6, assign36810_e51397_d_n7, assign36810_e51397_d_n10, assign36810_e51397_d_n11, assign36810_e51397_d_n12, assign36810_e51397_d_n17,) = {
    if (var_guard1214 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isub, var_isub_dn0, var_isub_dn2, var_isub_dn6, var_isub_dn7, var_isub_dn10, var_isub_dn11, var_isub_dn12, var_isub_dn17,)
    }
};
        var_isub = assign36810_e51397;
        var_isub_dn0 = assign36810_e51397_d_n0;
        var_isub_dn2 = assign36810_e51397_d_n2;
        var_isub_dn6 = assign36810_e51397_d_n6;
        var_isub_dn7 = assign36810_e51397_d_n7;
        var_isub_dn10 = assign36810_e51397_d_n10;
        var_isub_dn11 = assign36810_e51397_d_n11;
        var_isub_dn12 = assign36810_e51397_d_n12;
        var_isub_dn17 = assign36810_e51397_d_n17;
        var_isub_rv = 0.0;

        let (assign36820_e51404, assign36820_e51404_d_n0, assign36820_e51404_d_n2, assign36820_e51404_d_n6, assign36820_e51404_d_n7, assign36820_e51404_d_n10, assign36820_e51404_d_n11, assign36820_e51404_d_n12, assign36820_e51404_d_n13, assign36820_e51404_d_n15, assign36820_e51404_d_n16, assign36820_e51404_d_n17, assign36820_e51404_d_n18,) = {
    if (var_guard1214 == 0.0) {
        let assign36820_e51402: f64 = (var_qge + var_qg_nqs);
        (assign36820_e51402, (var_qge_dn0 + var_qg_nqs_dn0), (var_qge_dn2 + var_qg_nqs_dn2), (var_qge_dn6 + var_qg_nqs_dn6), (var_qge_dn7 + var_qg_nqs_dn7), (var_qge_dn10 + var_qg_nqs_dn10), (var_qge_dn11 + var_qg_nqs_dn11), (var_qge_dn12 + var_qg_nqs_dn12), (var_qge_dn13 + var_qg_nqs_dn13), (var_qge_dn15 + var_qg_nqs_dn15), (var_qge_dn16 + var_qg_nqs_dn16), (var_qge_dn17 + var_qg_nqs_dn17), (var_qge_dn18 + var_qg_nqs_dn18),)
    } else {
        (var_qg, var_qg_dn0, var_qg_dn2, var_qg_dn6, var_qg_dn7, var_qg_dn10, var_qg_dn11, var_qg_dn12, var_qg_dn13, var_qg_dn15, var_qg_dn16, var_qg_dn17, var_qg_dn18,)
    }
};
        var_qg = assign36820_e51404;
        var_qg_dn0 = assign36820_e51404_d_n0;
        var_qg_dn2 = assign36820_e51404_d_n2;
        var_qg_dn6 = assign36820_e51404_d_n6;
        var_qg_dn7 = assign36820_e51404_d_n7;
        var_qg_dn10 = assign36820_e51404_d_n10;
        var_qg_dn11 = assign36820_e51404_d_n11;
        var_qg_dn12 = assign36820_e51404_d_n12;
        var_qg_dn13 = assign36820_e51404_d_n13;
        var_qg_dn15 = assign36820_e51404_d_n15;
        var_qg_dn16 = assign36820_e51404_d_n16;
        var_qg_dn17 = assign36820_e51404_d_n17;
        var_qg_dn18 = assign36820_e51404_d_n18;
        var_qg_rv = 0.0;

        let (assign36830_e51411, assign36830_e51411_d_n0, assign36830_e51411_d_n2, assign36830_e51411_d_n6, assign36830_e51411_d_n7, assign36830_e51411_d_n10, assign36830_e51411_d_n11, assign36830_e51411_d_n12, assign36830_e51411_d_n13, assign36830_e51411_d_n15, assign36830_e51411_d_n16, assign36830_e51411_d_n17, assign36830_e51411_d_n18,) = {
    if (var_guard1214 == 0.0) {
        let assign36830_e51409: f64 = (var_qse + var_qs_nqs);
        (assign36830_e51409, (var_qse_dn0 + var_qs_nqs_dn0), (var_qse_dn2 + var_qs_nqs_dn2), (var_qse_dn6 + var_qs_nqs_dn6), (var_qse_dn7 + var_qs_nqs_dn7), (var_qse_dn10 + var_qs_nqs_dn10), (var_qse_dn11 + var_qs_nqs_dn11), (var_qse_dn12 + var_qs_nqs_dn12), var_qse_dn13, var_qse_dn15, (var_qse_dn16 + var_qs_nqs_dn16), (var_qse_dn17 + var_qs_nqs_dn17), (var_qse_dn18 + var_qs_nqs_dn18),)
    } else {
        (var_qd, var_qd_dn0, var_qd_dn2, var_qd_dn6, var_qd_dn7, var_qd_dn10, var_qd_dn11, var_qd_dn12, var_qd_dn13, var_qd_dn15, var_qd_dn16, var_qd_dn17, var_qd_dn18,)
    }
};
        var_qd = assign36830_e51411;
        var_qd_dn0 = assign36830_e51411_d_n0;
        var_qd_dn2 = assign36830_e51411_d_n2;
        var_qd_dn6 = assign36830_e51411_d_n6;
        var_qd_dn7 = assign36830_e51411_d_n7;
        var_qd_dn10 = assign36830_e51411_d_n10;
        var_qd_dn11 = assign36830_e51411_d_n11;
        var_qd_dn12 = assign36830_e51411_d_n12;
        var_qd_dn13 = assign36830_e51411_d_n13;
        var_qd_dn15 = assign36830_e51411_d_n15;
        var_qd_dn16 = assign36830_e51411_d_n16;
        var_qd_dn17 = assign36830_e51411_d_n17;
        var_qd_dn18 = assign36830_e51411_d_n18;
        var_qd_rv = 0.0;

        let (assign36850_e51428, assign36850_e51428_d_n0, assign36850_e51428_d_n2, assign36850_e51428_d_n6, assign36850_e51428_d_n7, assign36850_e51428_d_n10, assign36850_e51428_d_n11, assign36850_e51428_d_n12, assign36850_e51428_d_n13, assign36850_e51428_d_n15, assign36850_e51428_d_n16, assign36850_e51428_d_n17, assign36850_e51428_d_n18,) = {
    if (var_guard1214 == 0.0) {
        let assign36850_e51423: f64 = (var_qge + var_qde);
        let assign36850_e51425: f64 = (assign36850_e51423 + var_qse);
        let assign36850_e51426: f64 = (-assign36850_e51425);
        (assign36850_e51426, (-((var_qge_dn0 + var_qde_dn0) + var_qse_dn0)), (-((var_qge_dn2 + var_qde_dn2) + var_qse_dn2)), (-((var_qge_dn6 + var_qde_dn6) + var_qse_dn6)), (-((var_qge_dn7 + var_qde_dn7) + var_qse_dn7)), (-((var_qge_dn10 + var_qde_dn10) + var_qse_dn10)), (-((var_qge_dn11 + var_qde_dn11) + var_qse_dn11)), (-((var_qge_dn12 + var_qde_dn12) + var_qse_dn12)), (-((var_qge_dn13 + var_qde_dn13) + var_qse_dn13)), (-((var_qge_dn15 + var_qde_dn15) + var_qse_dn15)), (-((var_qge_dn16 + var_qde_dn16) + var_qse_dn16)), (-((var_qge_dn17 + var_qde_dn17) + var_qse_dn17)), (-((var_qge_dn18 + var_qde_dn18) + var_qse_dn18)),)
    } else {
        (var_qbe, var_qbe_dn0, var_qbe_dn2, var_qbe_dn6, var_qbe_dn7, var_qbe_dn10, var_qbe_dn11, var_qbe_dn12, var_qbe_dn13, var_qbe_dn15, var_qbe_dn16, var_qbe_dn17, var_qbe_dn18,)
    }
};
        var_qbe = assign36850_e51428;
        var_qbe_dn0 = assign36850_e51428_d_n0;
        var_qbe_dn2 = assign36850_e51428_d_n2;
        var_qbe_dn6 = assign36850_e51428_d_n6;
        var_qbe_dn7 = assign36850_e51428_d_n7;
        var_qbe_dn10 = assign36850_e51428_d_n10;
        var_qbe_dn11 = assign36850_e51428_d_n11;
        var_qbe_dn12 = assign36850_e51428_d_n12;
        var_qbe_dn13 = assign36850_e51428_d_n13;
        var_qbe_dn15 = assign36850_e51428_d_n15;
        var_qbe_dn16 = assign36850_e51428_d_n16;
        var_qbe_dn17 = assign36850_e51428_d_n17;
        var_qbe_dn18 = assign36850_e51428_d_n18;
        var_qbe_rv = 0.0;

        let (assign36860_e51435, assign36860_e51435_d_n0, assign36860_e51435_d_n2, assign36860_e51435_d_n6, assign36860_e51435_d_n7, assign36860_e51435_d_n10, assign36860_e51435_d_n11, assign36860_e51435_d_n12, assign36860_e51435_d_n13, assign36860_e51435_d_n15, assign36860_e51435_d_n16, assign36860_e51435_d_n17, assign36860_e51435_d_n18,) = {
    if (var_guard1214 == 0.0) {
        let assign36860_e51433: f64 = (var_qbe + var_qb_nqs);
        (assign36860_e51433, var_qbe_dn0, var_qbe_dn2, var_qbe_dn6, var_qbe_dn7, var_qbe_dn10, var_qbe_dn11, var_qbe_dn12, (var_qbe_dn13 + var_qb_nqs_dn13), var_qbe_dn15, var_qbe_dn16, var_qbe_dn17, var_qbe_dn18,)
    } else {
        (var_qb, var_qb_dn0, var_qb_dn2, var_qb_dn6, var_qb_dn7, var_qb_dn10, var_qb_dn11, var_qb_dn12, var_qb_dn13, var_qb_dn15, var_qb_dn16, var_qb_dn17, var_qb_dn18,)
    }
};
        var_qb = assign36860_e51435;
        var_qb_dn0 = assign36860_e51435_d_n0;
        var_qb_dn2 = assign36860_e51435_d_n2;
        var_qb_dn6 = assign36860_e51435_d_n6;
        var_qb_dn7 = assign36860_e51435_d_n7;
        var_qb_dn10 = assign36860_e51435_d_n10;
        var_qb_dn11 = assign36860_e51435_d_n11;
        var_qb_dn12 = assign36860_e51435_d_n12;
        var_qb_dn13 = assign36860_e51435_d_n13;
        var_qb_dn15 = assign36860_e51435_d_n15;
        var_qb_dn16 = assign36860_e51435_d_n16;
        var_qb_dn17 = assign36860_e51435_d_n17;
        var_qb_dn18 = assign36860_e51435_d_n18;
        var_qb_rv = 0.0;

        let assign36920_e51443: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard1215 = assign36920_e51443;
        var_guard1215_rv = 0.0;

        let (assign36930_e51447, assign36930_e51447_d_n0, assign36930_e51447_d_n2, assign36930_e51447_d_n6, assign36930_e51447_d_n7, assign36930_e51447_d_n10, assign36930_e51447_d_n11, assign36930_e51447_d_n12, assign36930_e51447_d_n17,) = {
    if (var_guard1215 != 0.0) {
        (var_ibdb, var_ibdb_dn0, var_ibdb_dn2, var_ibdb_dn6, var_ibdb_dn7, var_ibdb_dn10, var_ibdb_dn11, var_ibdb_dn12, var_ibdb_dn17,)
    } else {
        (var_ibd, var_ibd_dn0, var_ibd_dn2, var_ibd_dn6, var_ibd_dn7, var_ibd_dn10, var_ibd_dn11, var_ibd_dn12, var_ibd_dn17,)
    }
};
        var_ibd = assign36930_e51447;
        var_ibd_dn0 = assign36930_e51447_d_n0;
        var_ibd_dn2 = assign36930_e51447_d_n2;
        var_ibd_dn6 = assign36930_e51447_d_n6;
        var_ibd_dn7 = assign36930_e51447_d_n7;
        var_ibd_dn10 = assign36930_e51447_d_n10;
        var_ibd_dn11 = assign36930_e51447_d_n11;
        var_ibd_dn12 = assign36930_e51447_d_n12;
        var_ibd_dn17 = assign36930_e51447_d_n17;
        var_ibd_rv = 0.0;

        let (assign36940_e51451, assign36940_e51451_d_n0, assign36940_e51451_d_n2, assign36940_e51451_d_n6, assign36940_e51451_d_n7, assign36940_e51451_d_n10, assign36940_e51451_d_n11, assign36940_e51451_d_n12, assign36940_e51451_d_n17,) = {
    if (var_guard1215 != 0.0) {
        (var_qbd_s0, var_qbd_s0_dn0, var_qbd_s0_dn2, var_qbd_s0_dn6, var_qbd_s0_dn7, var_qbd_s0_dn10, var_qbd_s0_dn11, var_qbd_s0_dn12, var_qbd_s0_dn17,)
    } else {
        (var_qbd, var_qbd_dn0, var_qbd_dn2, var_qbd_dn6, var_qbd_dn7, var_qbd_dn10, var_qbd_dn11, var_qbd_dn12, var_qbd_dn17,)
    }
};
        var_qbd = assign36940_e51451;
        var_qbd_dn0 = assign36940_e51451_d_n0;
        var_qbd_dn2 = assign36940_e51451_d_n2;
        var_qbd_dn6 = assign36940_e51451_d_n6;
        var_qbd_dn7 = assign36940_e51451_d_n7;
        var_qbd_dn10 = assign36940_e51451_d_n10;
        var_qbd_dn11 = assign36940_e51451_d_n11;
        var_qbd_dn12 = assign36940_e51451_d_n12;
        var_qbd_dn17 = assign36940_e51451_d_n17;
        var_qbd_rv = 0.0;

        let (assign36950_e51455, assign36950_e51455_d_n0, assign36950_e51455_d_n2, assign36950_e51455_d_n6, assign36950_e51455_d_n7, assign36950_e51455_d_n10, assign36950_e51455_d_n11, assign36950_e51455_d_n12, assign36950_e51455_d_n17,) = {
    if (var_guard1215 != 0.0) {
        (var_ibsb, var_ibsb_dn0, var_ibsb_dn2, var_ibsb_dn6, var_ibsb_dn7, var_ibsb_dn10, var_ibsb_dn11, var_ibsb_dn12, var_ibsb_dn17,)
    } else {
        (var_ibs, var_ibs_dn0, var_ibs_dn2, var_ibs_dn6, var_ibs_dn7, var_ibs_dn10, var_ibs_dn11, var_ibs_dn12, var_ibs_dn17,)
    }
};
        var_ibs = assign36950_e51455;
        var_ibs_dn0 = assign36950_e51455_d_n0;
        var_ibs_dn2 = assign36950_e51455_d_n2;
        var_ibs_dn6 = assign36950_e51455_d_n6;
        var_ibs_dn7 = assign36950_e51455_d_n7;
        var_ibs_dn10 = assign36950_e51455_d_n10;
        var_ibs_dn11 = assign36950_e51455_d_n11;
        var_ibs_dn12 = assign36950_e51455_d_n12;
        var_ibs_dn17 = assign36950_e51455_d_n17;
        var_ibs_rv = 0.0;

        let (assign36960_e51459, assign36960_e51459_d_n0, assign36960_e51459_d_n2, assign36960_e51459_d_n6, assign36960_e51459_d_n7, assign36960_e51459_d_n10, assign36960_e51459_d_n11, assign36960_e51459_d_n12, assign36960_e51459_d_n17,) = {
    if (var_guard1215 != 0.0) {
        (var_qbs_s0, var_qbs_s0_dn0, var_qbs_s0_dn2, var_qbs_s0_dn6, var_qbs_s0_dn7, var_qbs_s0_dn10, var_qbs_s0_dn11, var_qbs_s0_dn12, var_qbs_s0_dn17,)
    } else {
        (var_qbs, var_qbs_dn0, var_qbs_dn2, var_qbs_dn6, var_qbs_dn7, var_qbs_dn10, var_qbs_dn11, var_qbs_dn12, var_qbs_dn17,)
    }
};
        var_qbs = assign36960_e51459;
        var_qbs_dn0 = assign36960_e51459_d_n0;
        var_qbs_dn2 = assign36960_e51459_d_n2;
        var_qbs_dn6 = assign36960_e51459_d_n6;
        var_qbs_dn7 = assign36960_e51459_d_n7;
        var_qbs_dn10 = assign36960_e51459_d_n10;
        var_qbs_dn11 = assign36960_e51459_d_n11;
        var_qbs_dn12 = assign36960_e51459_d_n12;
        var_qbs_dn17 = assign36960_e51459_d_n17;
        var_qbs_rv = 0.0;

        let assign36970_e51466: f64 = if ((p.p38 == 1.0) && (var_mks_rth0 > 0.0)) { 1.0 } else { 0.0 };
        var_guard1216 = assign36970_e51466;
        var_guard1216_rv = 0.0;

        let (assign36990_e51476,) = {
    if (var_guard1216 != 0.0) {
        (var_cth,)
    } else {
        (var_cthe,)
    }
};
        var_cthe = assign36990_e51476;
        var_cthe_rv = 0.0;

        let (assign37020_e51492,) = {
    if (var_guard1216 == 0.0) {
        (0.0,)
    } else {
        (var_cthe,)
    }
};
        var_cthe = assign37020_e51492;
        var_cthe_rv = 0.0;

        var_idse = var_ids;
        var_idse_dn0 = var_ids_dn0;
        var_idse_dn2 = var_ids_dn2;
        var_idse_dn6 = var_ids_dn6;
        var_idse_dn7 = var_ids_dn7;
        var_idse_dn10 = var_ids_dn10;
        var_idse_dn11 = var_ids_dn11;
        var_idse_dn12 = var_ids_dn12;
        var_idse_dn17 = var_ids_dn17;
        var_idse_rv = 0.0;

        let assign37190_e51546: f64 = var_qg_dn6;
        var_cgdbd = assign37190_e51546;
        var_cgdbd_dn0 = 0.0;
        var_cgdbd_dn2 = 0.0;
        var_cgdbd_dn6 = 0.0;
        var_cgdbd_dn7 = 0.0;
        var_cgdbd_dn10 = 0.0;
        var_cgdbd_dn11 = 0.0;
        var_cgdbd_dn12 = 0.0;
        var_cgdbd_dn13 = 0.0;
        var_cgdbd_dn15 = 0.0;
        var_cgdbd_dn16 = 0.0;
        var_cgdbd_dn17 = 0.0;
        var_cgdbd_dn18 = 0.0;
        var_cgdbd_rv = 0.0;

        let assign37200_e51549: f64 = (p.p50 * var_cgdbd);
        var_cgdbd = assign37200_e51549;
        var_cgdbd_dn0 = (p.p50 * var_cgdbd_dn0);
        var_cgdbd_dn2 = (p.p50 * var_cgdbd_dn2);
        var_cgdbd_dn6 = (p.p50 * var_cgdbd_dn6);
        var_cgdbd_dn7 = (p.p50 * var_cgdbd_dn7);
        var_cgdbd_dn10 = (p.p50 * var_cgdbd_dn10);
        var_cgdbd_dn11 = (p.p50 * var_cgdbd_dn11);
        var_cgdbd_dn12 = (p.p50 * var_cgdbd_dn12);
        var_cgdbd_dn13 = (p.p50 * var_cgdbd_dn13);
        var_cgdbd_dn15 = (p.p50 * var_cgdbd_dn15);
        var_cgdbd_dn16 = (p.p50 * var_cgdbd_dn16);
        var_cgdbd_dn17 = (p.p50 * var_cgdbd_dn17);
        var_cgdbd_dn18 = (p.p50 * var_cgdbd_dn18);
        var_cgdbd_rv = 0.0;

        let assign37210_e51552: f64 = var_qg_dn7;
        var_cgsbd = assign37210_e51552;
        var_cgsbd_dn0 = 0.0;
        var_cgsbd_dn2 = 0.0;
        var_cgsbd_dn6 = 0.0;
        var_cgsbd_dn7 = 0.0;
        var_cgsbd_dn10 = 0.0;
        var_cgsbd_dn11 = 0.0;
        var_cgsbd_dn12 = 0.0;
        var_cgsbd_dn13 = 0.0;
        var_cgsbd_dn15 = 0.0;
        var_cgsbd_dn16 = 0.0;
        var_cgsbd_dn17 = 0.0;
        var_cgsbd_dn18 = 0.0;
        var_cgsbd_rv = 0.0;

        let assign37220_e51555: f64 = (p.p50 * var_cgsbd);
        var_cgsbd = assign37220_e51555;
        var_cgsbd_dn0 = (p.p50 * var_cgsbd_dn0);
        var_cgsbd_dn2 = (p.p50 * var_cgsbd_dn2);
        var_cgsbd_dn6 = (p.p50 * var_cgsbd_dn6);
        var_cgsbd_dn7 = (p.p50 * var_cgsbd_dn7);
        var_cgsbd_dn10 = (p.p50 * var_cgsbd_dn10);
        var_cgsbd_dn11 = (p.p50 * var_cgsbd_dn11);
        var_cgsbd_dn12 = (p.p50 * var_cgsbd_dn12);
        var_cgsbd_dn13 = (p.p50 * var_cgsbd_dn13);
        var_cgsbd_dn15 = (p.p50 * var_cgsbd_dn15);
        var_cgsbd_dn16 = (p.p50 * var_cgsbd_dn16);
        var_cgsbd_dn17 = (p.p50 * var_cgsbd_dn17);
        var_cgsbd_dn18 = (p.p50 * var_cgsbd_dn18);
        var_cgsbd_rv = 0.0;

        let assign37490_e51636: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard1218 = assign37490_e51636;
        var_guard1218_rv = 0.0;

        *var_cgdbd_slot = var_cgdbd;
        *var_cgdbd_dn0_slot = var_cgdbd_dn0;
        *var_cgdbd_dn10_slot = var_cgdbd_dn10;
        *var_cgdbd_dn11_slot = var_cgdbd_dn11;
        *var_cgdbd_dn12_slot = var_cgdbd_dn12;
        *var_cgdbd_dn13_slot = var_cgdbd_dn13;
        *var_cgdbd_dn15_slot = var_cgdbd_dn15;
        *var_cgdbd_dn16_slot = var_cgdbd_dn16;
        *var_cgdbd_dn17_slot = var_cgdbd_dn17;
        *var_cgdbd_dn18_slot = var_cgdbd_dn18;
        *var_cgdbd_dn2_slot = var_cgdbd_dn2;
        *var_cgdbd_dn6_slot = var_cgdbd_dn6;
        *var_cgdbd_dn7_slot = var_cgdbd_dn7;
        *var_cgdbd_rv_slot = var_cgdbd_rv;
        *var_cgsbd_slot = var_cgsbd;
        *var_cgsbd_dn0_slot = var_cgsbd_dn0;
        *var_cgsbd_dn10_slot = var_cgsbd_dn10;
        *var_cgsbd_dn11_slot = var_cgsbd_dn11;
        *var_cgsbd_dn12_slot = var_cgsbd_dn12;
        *var_cgsbd_dn13_slot = var_cgsbd_dn13;
        *var_cgsbd_dn15_slot = var_cgsbd_dn15;
        *var_cgsbd_dn16_slot = var_cgsbd_dn16;
        *var_cgsbd_dn17_slot = var_cgsbd_dn17;
        *var_cgsbd_dn18_slot = var_cgsbd_dn18;
        *var_cgsbd_dn2_slot = var_cgsbd_dn2;
        *var_cgsbd_dn6_slot = var_cgsbd_dn6;
        *var_cgsbd_dn7_slot = var_cgsbd_dn7;
        *var_cgsbd_rv_slot = var_cgsbd_rv;
        *var_cthe_slot = var_cthe;
        *var_cthe_rv_slot = var_cthe_rv;
        *var_guard1214_slot = var_guard1214;
        *var_guard1214_rv_slot = var_guard1214_rv;
        *var_guard1215_slot = var_guard1215;
        *var_guard1215_rv_slot = var_guard1215_rv;
        *var_guard1216_slot = var_guard1216;
        *var_guard1216_rv_slot = var_guard1216_rv;
        *var_guard1218_slot = var_guard1218;
        *var_guard1218_rv_slot = var_guard1218_rv;
        *var_ibd_slot = var_ibd;
        *var_ibd_dn0_slot = var_ibd_dn0;
        *var_ibd_dn10_slot = var_ibd_dn10;
        *var_ibd_dn11_slot = var_ibd_dn11;
        *var_ibd_dn12_slot = var_ibd_dn12;
        *var_ibd_dn17_slot = var_ibd_dn17;
        *var_ibd_dn2_slot = var_ibd_dn2;
        *var_ibd_dn6_slot = var_ibd_dn6;
        *var_ibd_dn7_slot = var_ibd_dn7;
        *var_ibd_rv_slot = var_ibd_rv;
        *var_ibs_slot = var_ibs;
        *var_ibs_dn0_slot = var_ibs_dn0;
        *var_ibs_dn10_slot = var_ibs_dn10;
        *var_ibs_dn11_slot = var_ibs_dn11;
        *var_ibs_dn12_slot = var_ibs_dn12;
        *var_ibs_dn17_slot = var_ibs_dn17;
        *var_ibs_dn2_slot = var_ibs_dn2;
        *var_ibs_dn6_slot = var_ibs_dn6;
        *var_ibs_dn7_slot = var_ibs_dn7;
        *var_ibs_rv_slot = var_ibs_rv;
        *var_ids_slot = var_ids;
        *var_ids_dn0_slot = var_ids_dn0;
        *var_ids_dn10_slot = var_ids_dn10;
        *var_ids_dn11_slot = var_ids_dn11;
        *var_ids_dn12_slot = var_ids_dn12;
        *var_ids_dn17_slot = var_ids_dn17;
        *var_ids_dn2_slot = var_ids_dn2;
        *var_ids_dn6_slot = var_ids_dn6;
        *var_ids_dn7_slot = var_ids_dn7;
        *var_ids_rv_slot = var_ids_rv;
        *var_idse_slot = var_idse;
        *var_idse_dn0_slot = var_idse_dn0;
        *var_idse_dn10_slot = var_idse_dn10;
        *var_idse_dn11_slot = var_idse_dn11;
        *var_idse_dn12_slot = var_idse_dn12;
        *var_idse_dn17_slot = var_idse_dn17;
        *var_idse_dn2_slot = var_idse_dn2;
        *var_idse_dn6_slot = var_idse_dn6;
        *var_idse_dn7_slot = var_idse_dn7;
        *var_idse_rv_slot = var_idse_rv;
        *var_isub_slot = var_isub;
        *var_isub_dn0_slot = var_isub_dn0;
        *var_isub_dn10_slot = var_isub_dn10;
        *var_isub_dn11_slot = var_isub_dn11;
        *var_isub_dn12_slot = var_isub_dn12;
        *var_isub_dn17_slot = var_isub_dn17;
        *var_isub_dn2_slot = var_isub_dn2;
        *var_isub_dn6_slot = var_isub_dn6;
        *var_isub_dn7_slot = var_isub_dn7;
        *var_isub_rv_slot = var_isub_rv;
        *var_qb_slot = var_qb;
        *var_qb_dn0_slot = var_qb_dn0;
        *var_qb_dn10_slot = var_qb_dn10;
        *var_qb_dn11_slot = var_qb_dn11;
        *var_qb_dn12_slot = var_qb_dn12;
        *var_qb_dn13_slot = var_qb_dn13;
        *var_qb_dn15_slot = var_qb_dn15;
        *var_qb_dn16_slot = var_qb_dn16;
        *var_qb_dn17_slot = var_qb_dn17;
        *var_qb_dn18_slot = var_qb_dn18;
        *var_qb_dn2_slot = var_qb_dn2;
        *var_qb_dn6_slot = var_qb_dn6;
        *var_qb_dn7_slot = var_qb_dn7;
        *var_qb_nqs_slot = var_qb_nqs;
        *var_qb_nqs_dn13_slot = var_qb_nqs_dn13;
        *var_qb_nqs_rv_slot = var_qb_nqs_rv;
        *var_qb_rv_slot = var_qb_rv;
        *var_qbd_slot = var_qbd;
        *var_qbd_dn0_slot = var_qbd_dn0;
        *var_qbd_dn10_slot = var_qbd_dn10;
        *var_qbd_dn11_slot = var_qbd_dn11;
        *var_qbd_dn12_slot = var_qbd_dn12;
        *var_qbd_dn17_slot = var_qbd_dn17;
        *var_qbd_dn2_slot = var_qbd_dn2;
        *var_qbd_dn6_slot = var_qbd_dn6;
        *var_qbd_dn7_slot = var_qbd_dn7;
        *var_qbd_rv_slot = var_qbd_rv;
        *var_qbe_slot = var_qbe;
        *var_qbe_dn0_slot = var_qbe_dn0;
        *var_qbe_dn10_slot = var_qbe_dn10;
        *var_qbe_dn11_slot = var_qbe_dn11;
        *var_qbe_dn12_slot = var_qbe_dn12;
        *var_qbe_dn13_slot = var_qbe_dn13;
        *var_qbe_dn15_slot = var_qbe_dn15;
        *var_qbe_dn16_slot = var_qbe_dn16;
        *var_qbe_dn17_slot = var_qbe_dn17;
        *var_qbe_dn18_slot = var_qbe_dn18;
        *var_qbe_dn2_slot = var_qbe_dn2;
        *var_qbe_dn6_slot = var_qbe_dn6;
        *var_qbe_dn7_slot = var_qbe_dn7;
        *var_qbe_rv_slot = var_qbe_rv;
        *var_qbs_slot = var_qbs;
        *var_qbs_dn0_slot = var_qbs_dn0;
        *var_qbs_dn10_slot = var_qbs_dn10;
        *var_qbs_dn11_slot = var_qbs_dn11;
        *var_qbs_dn12_slot = var_qbs_dn12;
        *var_qbs_dn17_slot = var_qbs_dn17;
        *var_qbs_dn2_slot = var_qbs_dn2;
        *var_qbs_dn6_slot = var_qbs_dn6;
        *var_qbs_dn7_slot = var_qbs_dn7;
        *var_qbs_rv_slot = var_qbs_rv;
        *var_qd_slot = var_qd;
        *var_qd_dn0_slot = var_qd_dn0;
        *var_qd_dn10_slot = var_qd_dn10;
        *var_qd_dn11_slot = var_qd_dn11;
        *var_qd_dn12_slot = var_qd_dn12;
        *var_qd_dn13_slot = var_qd_dn13;
        *var_qd_dn15_slot = var_qd_dn15;
        *var_qd_dn16_slot = var_qd_dn16;
        *var_qd_dn17_slot = var_qd_dn17;
        *var_qd_dn18_slot = var_qd_dn18;
        *var_qd_dn2_slot = var_qd_dn2;
        *var_qd_dn6_slot = var_qd_dn6;
        *var_qd_dn7_slot = var_qd_dn7;
        *var_qd_rv_slot = var_qd_rv;
        *var_qg_slot = var_qg;
        *var_qg_dn0_slot = var_qg_dn0;
        *var_qg_dn10_slot = var_qg_dn10;
        *var_qg_dn11_slot = var_qg_dn11;
        *var_qg_dn12_slot = var_qg_dn12;
        *var_qg_dn13_slot = var_qg_dn13;
        *var_qg_dn15_slot = var_qg_dn15;
        *var_qg_dn16_slot = var_qg_dn16;
        *var_qg_dn17_slot = var_qg_dn17;
        *var_qg_dn18_slot = var_qg_dn18;
        *var_qg_dn2_slot = var_qg_dn2;
        *var_qg_dn6_slot = var_qg_dn6;
        *var_qg_dn7_slot = var_qg_dn7;
        *var_qg_nqs_slot = var_qg_nqs;
        *var_qg_nqs_dn0_slot = var_qg_nqs_dn0;
        *var_qg_nqs_dn10_slot = var_qg_nqs_dn10;
        *var_qg_nqs_dn11_slot = var_qg_nqs_dn11;
        *var_qg_nqs_dn12_slot = var_qg_nqs_dn12;
        *var_qg_nqs_dn13_slot = var_qg_nqs_dn13;
        *var_qg_nqs_dn15_slot = var_qg_nqs_dn15;
        *var_qg_nqs_dn16_slot = var_qg_nqs_dn16;
        *var_qg_nqs_dn17_slot = var_qg_nqs_dn17;
        *var_qg_nqs_dn18_slot = var_qg_nqs_dn18;
        *var_qg_nqs_dn2_slot = var_qg_nqs_dn2;
        *var_qg_nqs_dn6_slot = var_qg_nqs_dn6;
        *var_qg_nqs_dn7_slot = var_qg_nqs_dn7;
        *var_qg_nqs_rv_slot = var_qg_nqs_rv;
        *var_qg_rv_slot = var_qg_rv;
        *var_qs_nqs_slot = var_qs_nqs;
        *var_qs_nqs_dn0_slot = var_qs_nqs_dn0;
        *var_qs_nqs_dn10_slot = var_qs_nqs_dn10;
        *var_qs_nqs_dn11_slot = var_qs_nqs_dn11;
        *var_qs_nqs_dn12_slot = var_qs_nqs_dn12;
        *var_qs_nqs_dn16_slot = var_qs_nqs_dn16;
        *var_qs_nqs_dn17_slot = var_qs_nqs_dn17;
        *var_qs_nqs_dn18_slot = var_qs_nqs_dn18;
        *var_qs_nqs_dn2_slot = var_qs_nqs_dn2;
        *var_qs_nqs_dn6_slot = var_qs_nqs_dn6;
        *var_qs_nqs_dn7_slot = var_qs_nqs_dn7;
        *var_qs_nqs_rv_slot = var_qs_nqs_rv;
    }

    pub(super) fn stamp_reactive_block_130(
        p: &Parameters,
        var_guard1218: f64,
        var_ibd: f64,
        var_ibd_dn0: f64,
        var_ibd_dn10: f64,
        var_ibd_dn11: f64,
        var_ibd_dn12: f64,
        var_ibd_dn17: f64,
        var_ibd_dn2: f64,
        var_ibd_dn6: f64,
        var_ibd_dn7: f64,
        var_ibs: f64,
        var_ibs_dn0: f64,
        var_ibs_dn10: f64,
        var_ibs_dn11: f64,
        var_ibs_dn12: f64,
        var_ibs_dn17: f64,
        var_ibs_dn2: f64,
        var_ibs_dn6: f64,
        var_ibs_dn7: f64,
        var_mode: f64,
        var_noiigate: f64,
        var_noiigate_dn0: f64,
        var_noiigate_dn10: f64,
        var_noiigate_dn11: f64,
        var_noiigate_dn12: f64,
        var_noiigate_dn13: f64,
        var_noiigate_dn15: f64,
        var_noiigate_dn16: f64,
        var_noiigate_dn17: f64,
        var_noiigate_dn18: f64,
        var_noiigate_dn2: f64,
        var_noiigate_dn6: f64,
        var_noiigate_dn7: f64,
        var_noithrml: f64,
        var_noithrml_dn0: f64,
        var_noithrml_dn10: f64,
        var_noithrml_dn11: f64,
        var_noithrml_dn12: f64,
        var_noithrml_dn17: f64,
        var_noithrml_dn2: f64,
        var_noithrml_dn6: f64,
        var_noithrml_dn7: f64,
        var_qdrat_noi: f64,
        var_qdrat_noi_dn0: f64,
        var_qdrat_noi_dn10: f64,
        var_qdrat_noi_dn11: f64,
        var_qdrat_noi_dn12: f64,
        var_qdrat_noi_dn17: f64,
        var_qdrat_noi_dn2: f64,
        var_qdrat_noi_dn6: f64,
        var_qdrat_noi_dn7: f64,
        var_ttemp: f64,
        var_ttemp_dn10: f64,
        var_guard1226_slot: &mut f64,
        var_guard1226_rv_slot: &mut f64,
        var_guard1227_slot: &mut f64,
        var_guard1227_rv_slot: &mut f64,
        var_guard1228_slot: &mut f64,
        var_guard1228_rv_slot: &mut f64,
        var_ibdb_slot: &mut f64,
        var_ibdb_dn0_slot: &mut f64,
        var_ibdb_dn10_slot: &mut f64,
        var_ibdb_dn11_slot: &mut f64,
        var_ibdb_dn12_slot: &mut f64,
        var_ibdb_dn17_slot: &mut f64,
        var_ibdb_dn2_slot: &mut f64,
        var_ibdb_dn6_slot: &mut f64,
        var_ibdb_dn7_slot: &mut f64,
        var_ibdb_rv_slot: &mut f64,
        var_ibsb_slot: &mut f64,
        var_ibsb_dn0_slot: &mut f64,
        var_ibsb_dn10_slot: &mut f64,
        var_ibsb_dn11_slot: &mut f64,
        var_ibsb_dn12_slot: &mut f64,
        var_ibsb_dn17_slot: &mut f64,
        var_ibsb_dn2_slot: &mut f64,
        var_ibsb_dn6_slot: &mut f64,
        var_ibsb_dn7_slot: &mut f64,
        var_ibsb_rv_slot: &mut f64,
        var_qdrat_slot: &mut f64,
        var_qdrat_dn0_slot: &mut f64,
        var_qdrat_dn10_slot: &mut f64,
        var_qdrat_dn11_slot: &mut f64,
        var_qdrat_dn12_slot: &mut f64,
        var_qdrat_dn17_slot: &mut f64,
        var_qdrat_dn2_slot: &mut f64,
        var_qdrat_dn6_slot: &mut f64,
        var_qdrat_dn7_slot: &mut f64,
        var_qdrat_rv_slot: &mut f64,
        var_sid_slot: &mut f64,
        var_sid_dn0_slot: &mut f64,
        var_sid_dn10_slot: &mut f64,
        var_sid_dn11_slot: &mut f64,
        var_sid_dn12_slot: &mut f64,
        var_sid_dn17_slot: &mut f64,
        var_sid_dn2_slot: &mut f64,
        var_sid_dn6_slot: &mut f64,
        var_sid_dn7_slot: &mut f64,
        var_sid_rv_slot: &mut f64,
        var_sigrat_slot: &mut f64,
        var_sigrat_d_slot: &mut f64,
        var_sigrat_d_dn0_slot: &mut f64,
        var_sigrat_d_dn10_slot: &mut f64,
        var_sigrat_d_dn11_slot: &mut f64,
        var_sigrat_d_dn12_slot: &mut f64,
        var_sigrat_d_dn13_slot: &mut f64,
        var_sigrat_d_dn15_slot: &mut f64,
        var_sigrat_d_dn16_slot: &mut f64,
        var_sigrat_d_dn17_slot: &mut f64,
        var_sigrat_d_dn18_slot: &mut f64,
        var_sigrat_d_dn2_slot: &mut f64,
        var_sigrat_d_dn6_slot: &mut f64,
        var_sigrat_d_dn7_slot: &mut f64,
        var_sigrat_d_rv_slot: &mut f64,
        var_sigrat_dn0_slot: &mut f64,
        var_sigrat_dn10_slot: &mut f64,
        var_sigrat_dn11_slot: &mut f64,
        var_sigrat_dn12_slot: &mut f64,
        var_sigrat_dn13_slot: &mut f64,
        var_sigrat_dn15_slot: &mut f64,
        var_sigrat_dn16_slot: &mut f64,
        var_sigrat_dn17_slot: &mut f64,
        var_sigrat_dn18_slot: &mut f64,
        var_sigrat_dn2_slot: &mut f64,
        var_sigrat_dn6_slot: &mut f64,
        var_sigrat_dn7_slot: &mut f64,
        var_sigrat_rv_slot: &mut f64,
        var_sigrat_s_slot: &mut f64,
        var_sigrat_s_dn0_slot: &mut f64,
        var_sigrat_s_dn10_slot: &mut f64,
        var_sigrat_s_dn11_slot: &mut f64,
        var_sigrat_s_dn12_slot: &mut f64,
        var_sigrat_s_dn13_slot: &mut f64,
        var_sigrat_s_dn15_slot: &mut f64,
        var_sigrat_s_dn16_slot: &mut f64,
        var_sigrat_s_dn17_slot: &mut f64,
        var_sigrat_s_dn18_slot: &mut f64,
        var_sigrat_s_dn2_slot: &mut f64,
        var_sigrat_s_dn6_slot: &mut f64,
        var_sigrat_s_dn7_slot: &mut f64,
        var_sigrat_s_rv_slot: &mut f64,
        var_whi_noise_slot: &mut f64,
        var_whi_noise_dn10_slot: &mut f64,
        var_whi_noise_rv_slot: &mut f64,
    ) {
        let mut var_guard1226: f64 = *var_guard1226_slot;
        let mut var_guard1226_rv: f64 = *var_guard1226_rv_slot;
        let mut var_guard1227: f64 = *var_guard1227_slot;
        let mut var_guard1227_rv: f64 = *var_guard1227_rv_slot;
        let mut var_guard1228: f64 = *var_guard1228_slot;
        let mut var_guard1228_rv: f64 = *var_guard1228_rv_slot;
        let mut var_ibdb: f64 = *var_ibdb_slot;
        let mut var_ibdb_dn0: f64 = *var_ibdb_dn0_slot;
        let mut var_ibdb_dn10: f64 = *var_ibdb_dn10_slot;
        let mut var_ibdb_dn11: f64 = *var_ibdb_dn11_slot;
        let mut var_ibdb_dn12: f64 = *var_ibdb_dn12_slot;
        let mut var_ibdb_dn17: f64 = *var_ibdb_dn17_slot;
        let mut var_ibdb_dn2: f64 = *var_ibdb_dn2_slot;
        let mut var_ibdb_dn6: f64 = *var_ibdb_dn6_slot;
        let mut var_ibdb_dn7: f64 = *var_ibdb_dn7_slot;
        let mut var_ibdb_rv: f64 = *var_ibdb_rv_slot;
        let mut var_ibsb: f64 = *var_ibsb_slot;
        let mut var_ibsb_dn0: f64 = *var_ibsb_dn0_slot;
        let mut var_ibsb_dn10: f64 = *var_ibsb_dn10_slot;
        let mut var_ibsb_dn11: f64 = *var_ibsb_dn11_slot;
        let mut var_ibsb_dn12: f64 = *var_ibsb_dn12_slot;
        let mut var_ibsb_dn17: f64 = *var_ibsb_dn17_slot;
        let mut var_ibsb_dn2: f64 = *var_ibsb_dn2_slot;
        let mut var_ibsb_dn6: f64 = *var_ibsb_dn6_slot;
        let mut var_ibsb_dn7: f64 = *var_ibsb_dn7_slot;
        let mut var_ibsb_rv: f64 = *var_ibsb_rv_slot;
        let mut var_qdrat: f64 = *var_qdrat_slot;
        let mut var_qdrat_dn0: f64 = *var_qdrat_dn0_slot;
        let mut var_qdrat_dn10: f64 = *var_qdrat_dn10_slot;
        let mut var_qdrat_dn11: f64 = *var_qdrat_dn11_slot;
        let mut var_qdrat_dn12: f64 = *var_qdrat_dn12_slot;
        let mut var_qdrat_dn17: f64 = *var_qdrat_dn17_slot;
        let mut var_qdrat_dn2: f64 = *var_qdrat_dn2_slot;
        let mut var_qdrat_dn6: f64 = *var_qdrat_dn6_slot;
        let mut var_qdrat_dn7: f64 = *var_qdrat_dn7_slot;
        let mut var_qdrat_rv: f64 = *var_qdrat_rv_slot;
        let mut var_sid: f64 = *var_sid_slot;
        let mut var_sid_dn0: f64 = *var_sid_dn0_slot;
        let mut var_sid_dn10: f64 = *var_sid_dn10_slot;
        let mut var_sid_dn11: f64 = *var_sid_dn11_slot;
        let mut var_sid_dn12: f64 = *var_sid_dn12_slot;
        let mut var_sid_dn17: f64 = *var_sid_dn17_slot;
        let mut var_sid_dn2: f64 = *var_sid_dn2_slot;
        let mut var_sid_dn6: f64 = *var_sid_dn6_slot;
        let mut var_sid_dn7: f64 = *var_sid_dn7_slot;
        let mut var_sid_rv: f64 = *var_sid_rv_slot;
        let mut var_sigrat: f64 = *var_sigrat_slot;
        let mut var_sigrat_d: f64 = *var_sigrat_d_slot;
        let mut var_sigrat_d_dn0: f64 = *var_sigrat_d_dn0_slot;
        let mut var_sigrat_d_dn10: f64 = *var_sigrat_d_dn10_slot;
        let mut var_sigrat_d_dn11: f64 = *var_sigrat_d_dn11_slot;
        let mut var_sigrat_d_dn12: f64 = *var_sigrat_d_dn12_slot;
        let mut var_sigrat_d_dn13: f64 = *var_sigrat_d_dn13_slot;
        let mut var_sigrat_d_dn15: f64 = *var_sigrat_d_dn15_slot;
        let mut var_sigrat_d_dn16: f64 = *var_sigrat_d_dn16_slot;
        let mut var_sigrat_d_dn17: f64 = *var_sigrat_d_dn17_slot;
        let mut var_sigrat_d_dn18: f64 = *var_sigrat_d_dn18_slot;
        let mut var_sigrat_d_dn2: f64 = *var_sigrat_d_dn2_slot;
        let mut var_sigrat_d_dn6: f64 = *var_sigrat_d_dn6_slot;
        let mut var_sigrat_d_dn7: f64 = *var_sigrat_d_dn7_slot;
        let mut var_sigrat_d_rv: f64 = *var_sigrat_d_rv_slot;
        let mut var_sigrat_dn0: f64 = *var_sigrat_dn0_slot;
        let mut var_sigrat_dn10: f64 = *var_sigrat_dn10_slot;
        let mut var_sigrat_dn11: f64 = *var_sigrat_dn11_slot;
        let mut var_sigrat_dn12: f64 = *var_sigrat_dn12_slot;
        let mut var_sigrat_dn13: f64 = *var_sigrat_dn13_slot;
        let mut var_sigrat_dn15: f64 = *var_sigrat_dn15_slot;
        let mut var_sigrat_dn16: f64 = *var_sigrat_dn16_slot;
        let mut var_sigrat_dn17: f64 = *var_sigrat_dn17_slot;
        let mut var_sigrat_dn18: f64 = *var_sigrat_dn18_slot;
        let mut var_sigrat_dn2: f64 = *var_sigrat_dn2_slot;
        let mut var_sigrat_dn6: f64 = *var_sigrat_dn6_slot;
        let mut var_sigrat_dn7: f64 = *var_sigrat_dn7_slot;
        let mut var_sigrat_rv: f64 = *var_sigrat_rv_slot;
        let mut var_sigrat_s: f64 = *var_sigrat_s_slot;
        let mut var_sigrat_s_dn0: f64 = *var_sigrat_s_dn0_slot;
        let mut var_sigrat_s_dn10: f64 = *var_sigrat_s_dn10_slot;
        let mut var_sigrat_s_dn11: f64 = *var_sigrat_s_dn11_slot;
        let mut var_sigrat_s_dn12: f64 = *var_sigrat_s_dn12_slot;
        let mut var_sigrat_s_dn13: f64 = *var_sigrat_s_dn13_slot;
        let mut var_sigrat_s_dn15: f64 = *var_sigrat_s_dn15_slot;
        let mut var_sigrat_s_dn16: f64 = *var_sigrat_s_dn16_slot;
        let mut var_sigrat_s_dn17: f64 = *var_sigrat_s_dn17_slot;
        let mut var_sigrat_s_dn18: f64 = *var_sigrat_s_dn18_slot;
        let mut var_sigrat_s_dn2: f64 = *var_sigrat_s_dn2_slot;
        let mut var_sigrat_s_dn6: f64 = *var_sigrat_s_dn6_slot;
        let mut var_sigrat_s_dn7: f64 = *var_sigrat_s_dn7_slot;
        let mut var_sigrat_s_rv: f64 = *var_sigrat_s_rv_slot;
        let mut var_whi_noise: f64 = *var_whi_noise_slot;
        let mut var_whi_noise_dn10: f64 = *var_whi_noise_dn10_slot;
        let mut var_whi_noise_rv: f64 = *var_whi_noise_rv_slot;

        let (assign37500_e51642, assign37500_e51642_d_n0, assign37500_e51642_d_n2, assign37500_e51642_d_n6, assign37500_e51642_d_n7, assign37500_e51642_d_n10, assign37500_e51642_d_n11, assign37500_e51642_d_n12, assign37500_e51642_d_n17,) = {
    if (var_guard1218 != 0.0) {
        let assign37500_e51640: f64 = (p.p50 * var_ibd);
        (assign37500_e51640, (p.p50 * var_ibd_dn0), (p.p50 * var_ibd_dn2), (p.p50 * var_ibd_dn6), (p.p50 * var_ibd_dn7), (p.p50 * var_ibd_dn10), (p.p50 * var_ibd_dn11), (p.p50 * var_ibd_dn12), (p.p50 * var_ibd_dn17),)
    } else {
        (var_ibdb, var_ibdb_dn0, var_ibdb_dn2, var_ibdb_dn6, var_ibdb_dn7, var_ibdb_dn10, var_ibdb_dn11, var_ibdb_dn12, var_ibdb_dn17,)
    }
};
        var_ibdb = assign37500_e51642;
        var_ibdb_dn0 = assign37500_e51642_d_n0;
        var_ibdb_dn2 = assign37500_e51642_d_n2;
        var_ibdb_dn6 = assign37500_e51642_d_n6;
        var_ibdb_dn7 = assign37500_e51642_d_n7;
        var_ibdb_dn10 = assign37500_e51642_d_n10;
        var_ibdb_dn11 = assign37500_e51642_d_n11;
        var_ibdb_dn12 = assign37500_e51642_d_n12;
        var_ibdb_dn17 = assign37500_e51642_d_n17;
        var_ibdb_rv = 0.0;

        let (assign37510_e51648, assign37510_e51648_d_n0, assign37510_e51648_d_n2, assign37510_e51648_d_n6, assign37510_e51648_d_n7, assign37510_e51648_d_n10, assign37510_e51648_d_n11, assign37510_e51648_d_n12, assign37510_e51648_d_n17,) = {
    if (var_guard1218 != 0.0) {
        let assign37510_e51646: f64 = (p.p50 * var_ibs);
        (assign37510_e51646, (p.p50 * var_ibs_dn0), (p.p50 * var_ibs_dn2), (p.p50 * var_ibs_dn6), (p.p50 * var_ibs_dn7), (p.p50 * var_ibs_dn10), (p.p50 * var_ibs_dn11), (p.p50 * var_ibs_dn12), (p.p50 * var_ibs_dn17),)
    } else {
        (var_ibsb, var_ibsb_dn0, var_ibsb_dn2, var_ibsb_dn6, var_ibsb_dn7, var_ibsb_dn10, var_ibsb_dn11, var_ibsb_dn12, var_ibsb_dn17,)
    }
};
        var_ibsb = assign37510_e51648;
        var_ibsb_dn0 = assign37510_e51648_d_n0;
        var_ibsb_dn2 = assign37510_e51648_d_n2;
        var_ibsb_dn6 = assign37510_e51648_d_n6;
        var_ibsb_dn7 = assign37510_e51648_d_n7;
        var_ibsb_dn10 = assign37510_e51648_d_n10;
        var_ibsb_dn11 = assign37510_e51648_d_n11;
        var_ibsb_dn12 = assign37510_e51648_d_n12;
        var_ibsb_dn17 = assign37510_e51648_d_n17;
        var_ibsb_rv = 0.0;

        let assign37630_e51700: f64 = (4.0 * 1.3806226e-23);
        let assign37630_e51702: f64 = (assign37630_e51700 * var_ttemp);
        let assign37630_e51704: f64 = assign37630_e51702;
        var_whi_noise = assign37630_e51704;
        var_whi_noise_dn10 = (assign37630_e51700 * var_ttemp_dn10);
        var_whi_noise_rv = 0.0;

        var_qdrat = var_qdrat_noi;
        var_qdrat_dn0 = var_qdrat_noi_dn0;
        var_qdrat_dn2 = var_qdrat_noi_dn2;
        var_qdrat_dn6 = var_qdrat_noi_dn6;
        var_qdrat_dn7 = var_qdrat_noi_dn7;
        var_qdrat_dn10 = var_qdrat_noi_dn10;
        var_qdrat_dn11 = var_qdrat_noi_dn11;
        var_qdrat_dn12 = var_qdrat_noi_dn12;
        var_qdrat_dn17 = var_qdrat_noi_dn17;
        var_qdrat_rv = 0.0;

        let assign37660_e51711: f64 = (var_whi_noise * var_noithrml);
        var_sid = assign37660_e51711;
        var_sid_dn0 = (var_whi_noise * var_noithrml_dn0);
        var_sid_dn2 = (var_whi_noise * var_noithrml_dn2);
        var_sid_dn6 = (var_whi_noise * var_noithrml_dn6);
        var_sid_dn7 = (var_whi_noise * var_noithrml_dn7);
        var_sid_dn10 = ((var_whi_noise_dn10 * var_noithrml) + (var_whi_noise * var_noithrml_dn10));
        var_sid_dn11 = (var_whi_noise * var_noithrml_dn11);
        var_sid_dn12 = (var_whi_noise * var_noithrml_dn12);
        var_sid_dn17 = (var_whi_noise * var_noithrml_dn17);
        var_sid_rv = 0.0;

        let (assign37680_e51725, assign37680_e51725_d_n0, assign37680_e51725_d_n2, assign37680_e51725_d_n6, assign37680_e51725_d_n7, assign37680_e51725_d_n10, assign37680_e51725_d_n11, assign37680_e51725_d_n12, assign37680_e51725_d_n13, assign37680_e51725_d_n15, assign37680_e51725_d_n16, assign37680_e51725_d_n17, assign37680_e51725_d_n18,) = {
    if ((var_sid > 0.0) && (var_noiigate > 0.0)) {
        let assign37680_e51722: f64 = (var_noiigate / var_sid);
        let assign37680_e51723: f64 = (assign37680_e51722).sqrt();
        (assign37680_e51723, ((((var_noiigate_dn0 * var_sid) - (var_noiigate * var_sid_dn0)) / (var_sid * var_sid)) / (2.0 * assign37680_e51723)), ((((var_noiigate_dn2 * var_sid) - (var_noiigate * var_sid_dn2)) / (var_sid * var_sid)) / (2.0 * assign37680_e51723)), ((((var_noiigate_dn6 * var_sid) - (var_noiigate * var_sid_dn6)) / (var_sid * var_sid)) / (2.0 * assign37680_e51723)), ((((var_noiigate_dn7 * var_sid) - (var_noiigate * var_sid_dn7)) / (var_sid * var_sid)) / (2.0 * assign37680_e51723)), ((((var_noiigate_dn10 * var_sid) - (var_noiigate * var_sid_dn10)) / (var_sid * var_sid)) / (2.0 * assign37680_e51723)), ((((var_noiigate_dn11 * var_sid) - (var_noiigate * var_sid_dn11)) / (var_sid * var_sid)) / (2.0 * assign37680_e51723)), ((((var_noiigate_dn12 * var_sid) - (var_noiigate * var_sid_dn12)) / (var_sid * var_sid)) / (2.0 * assign37680_e51723)), ((var_noiigate_dn13 / var_sid) / (2.0 * assign37680_e51723)), ((var_noiigate_dn15 / var_sid) / (2.0 * assign37680_e51723)), ((var_noiigate_dn16 / var_sid) / (2.0 * assign37680_e51723)), ((((var_noiigate_dn17 * var_sid) - (var_noiigate * var_sid_dn17)) / (var_sid * var_sid)) / (2.0 * assign37680_e51723)), ((var_noiigate_dn18 / var_sid) / (2.0 * assign37680_e51723)),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        var_sigrat = assign37680_e51725;
        var_sigrat_dn0 = assign37680_e51725_d_n0;
        var_sigrat_dn2 = assign37680_e51725_d_n2;
        var_sigrat_dn6 = assign37680_e51725_d_n6;
        var_sigrat_dn7 = assign37680_e51725_d_n7;
        var_sigrat_dn10 = assign37680_e51725_d_n10;
        var_sigrat_dn11 = assign37680_e51725_d_n11;
        var_sigrat_dn12 = assign37680_e51725_d_n12;
        var_sigrat_dn13 = assign37680_e51725_d_n13;
        var_sigrat_dn15 = assign37680_e51725_d_n15;
        var_sigrat_dn16 = assign37680_e51725_d_n16;
        var_sigrat_dn17 = assign37680_e51725_d_n17;
        var_sigrat_dn18 = assign37680_e51725_d_n18;
        var_sigrat_rv = 0.0;

        let (assign37690_e51737, assign37690_e51737_d_n0, assign37690_e51737_d_n2, assign37690_e51737_d_n6, assign37690_e51737_d_n7, assign37690_e51737_d_n10, assign37690_e51737_d_n11, assign37690_e51737_d_n12, assign37690_e51737_d_n13, assign37690_e51737_d_n15, assign37690_e51737_d_n16, assign37690_e51737_d_n17, assign37690_e51737_d_n18,) = {
    if (var_mode > 0.0) {
        let assign37690_e51732: f64 = (1.0 - var_qdrat);
        let assign37690_e51733: f64 = (var_sigrat * assign37690_e51732);
        (assign37690_e51733, ((var_sigrat_dn0 * assign37690_e51732) + (var_sigrat * (-var_qdrat_dn0))), ((var_sigrat_dn2 * assign37690_e51732) + (var_sigrat * (-var_qdrat_dn2))), ((var_sigrat_dn6 * assign37690_e51732) + (var_sigrat * (-var_qdrat_dn6))), ((var_sigrat_dn7 * assign37690_e51732) + (var_sigrat * (-var_qdrat_dn7))), ((var_sigrat_dn10 * assign37690_e51732) + (var_sigrat * (-var_qdrat_dn10))), ((var_sigrat_dn11 * assign37690_e51732) + (var_sigrat * (-var_qdrat_dn11))), ((var_sigrat_dn12 * assign37690_e51732) + (var_sigrat * (-var_qdrat_dn12))), (var_sigrat_dn13 * assign37690_e51732), (var_sigrat_dn15 * assign37690_e51732), (var_sigrat_dn16 * assign37690_e51732), ((var_sigrat_dn17 * assign37690_e51732) + (var_sigrat * (-var_qdrat_dn17))), (var_sigrat_dn18 * assign37690_e51732),)
    } else {
        let assign37690_e51736: f64 = (var_sigrat * var_qdrat);
        (assign37690_e51736, ((var_sigrat_dn0 * var_qdrat) + (var_sigrat * var_qdrat_dn0)), ((var_sigrat_dn2 * var_qdrat) + (var_sigrat * var_qdrat_dn2)), ((var_sigrat_dn6 * var_qdrat) + (var_sigrat * var_qdrat_dn6)), ((var_sigrat_dn7 * var_qdrat) + (var_sigrat * var_qdrat_dn7)), ((var_sigrat_dn10 * var_qdrat) + (var_sigrat * var_qdrat_dn10)), ((var_sigrat_dn11 * var_qdrat) + (var_sigrat * var_qdrat_dn11)), ((var_sigrat_dn12 * var_qdrat) + (var_sigrat * var_qdrat_dn12)), (var_sigrat_dn13 * var_qdrat), (var_sigrat_dn15 * var_qdrat), (var_sigrat_dn16 * var_qdrat), ((var_sigrat_dn17 * var_qdrat) + (var_sigrat * var_qdrat_dn17)), (var_sigrat_dn18 * var_qdrat),)
    }
};
        var_sigrat_s = assign37690_e51737;
        var_sigrat_s_dn0 = assign37690_e51737_d_n0;
        var_sigrat_s_dn2 = assign37690_e51737_d_n2;
        var_sigrat_s_dn6 = assign37690_e51737_d_n6;
        var_sigrat_s_dn7 = assign37690_e51737_d_n7;
        var_sigrat_s_dn10 = assign37690_e51737_d_n10;
        var_sigrat_s_dn11 = assign37690_e51737_d_n11;
        var_sigrat_s_dn12 = assign37690_e51737_d_n12;
        var_sigrat_s_dn13 = assign37690_e51737_d_n13;
        var_sigrat_s_dn15 = assign37690_e51737_d_n15;
        var_sigrat_s_dn16 = assign37690_e51737_d_n16;
        var_sigrat_s_dn17 = assign37690_e51737_d_n17;
        var_sigrat_s_dn18 = assign37690_e51737_d_n18;
        var_sigrat_s_rv = 0.0;

        let (assign37700_e51749, assign37700_e51749_d_n0, assign37700_e51749_d_n2, assign37700_e51749_d_n6, assign37700_e51749_d_n7, assign37700_e51749_d_n10, assign37700_e51749_d_n11, assign37700_e51749_d_n12, assign37700_e51749_d_n13, assign37700_e51749_d_n15, assign37700_e51749_d_n16, assign37700_e51749_d_n17, assign37700_e51749_d_n18,) = {
    if (var_mode > 0.0) {
        let assign37700_e51743: f64 = (var_sigrat * var_qdrat);
        (assign37700_e51743, ((var_sigrat_dn0 * var_qdrat) + (var_sigrat * var_qdrat_dn0)), ((var_sigrat_dn2 * var_qdrat) + (var_sigrat * var_qdrat_dn2)), ((var_sigrat_dn6 * var_qdrat) + (var_sigrat * var_qdrat_dn6)), ((var_sigrat_dn7 * var_qdrat) + (var_sigrat * var_qdrat_dn7)), ((var_sigrat_dn10 * var_qdrat) + (var_sigrat * var_qdrat_dn10)), ((var_sigrat_dn11 * var_qdrat) + (var_sigrat * var_qdrat_dn11)), ((var_sigrat_dn12 * var_qdrat) + (var_sigrat * var_qdrat_dn12)), (var_sigrat_dn13 * var_qdrat), (var_sigrat_dn15 * var_qdrat), (var_sigrat_dn16 * var_qdrat), ((var_sigrat_dn17 * var_qdrat) + (var_sigrat * var_qdrat_dn17)), (var_sigrat_dn18 * var_qdrat),)
    } else {
        let assign37700_e51747: f64 = (1.0 - var_qdrat);
        let assign37700_e51748: f64 = (var_sigrat * assign37700_e51747);
        (assign37700_e51748, ((var_sigrat_dn0 * assign37700_e51747) + (var_sigrat * (-var_qdrat_dn0))), ((var_sigrat_dn2 * assign37700_e51747) + (var_sigrat * (-var_qdrat_dn2))), ((var_sigrat_dn6 * assign37700_e51747) + (var_sigrat * (-var_qdrat_dn6))), ((var_sigrat_dn7 * assign37700_e51747) + (var_sigrat * (-var_qdrat_dn7))), ((var_sigrat_dn10 * assign37700_e51747) + (var_sigrat * (-var_qdrat_dn10))), ((var_sigrat_dn11 * assign37700_e51747) + (var_sigrat * (-var_qdrat_dn11))), ((var_sigrat_dn12 * assign37700_e51747) + (var_sigrat * (-var_qdrat_dn12))), (var_sigrat_dn13 * assign37700_e51747), (var_sigrat_dn15 * assign37700_e51747), (var_sigrat_dn16 * assign37700_e51747), ((var_sigrat_dn17 * assign37700_e51747) + (var_sigrat * (-var_qdrat_dn17))), (var_sigrat_dn18 * assign37700_e51747),)
    }
};
        var_sigrat_d = assign37700_e51749;
        var_sigrat_d_dn0 = assign37700_e51749_d_n0;
        var_sigrat_d_dn2 = assign37700_e51749_d_n2;
        var_sigrat_d_dn6 = assign37700_e51749_d_n6;
        var_sigrat_d_dn7 = assign37700_e51749_d_n7;
        var_sigrat_d_dn10 = assign37700_e51749_d_n10;
        var_sigrat_d_dn11 = assign37700_e51749_d_n11;
        var_sigrat_d_dn12 = assign37700_e51749_d_n12;
        var_sigrat_d_dn13 = assign37700_e51749_d_n13;
        var_sigrat_d_dn15 = assign37700_e51749_d_n15;
        var_sigrat_d_dn16 = assign37700_e51749_d_n16;
        var_sigrat_d_dn17 = assign37700_e51749_d_n17;
        var_sigrat_d_dn18 = assign37700_e51749_d_n18;
        var_sigrat_d_rv = 0.0;

        let assign37720_e51759: f64 = if ((p.p38 > 0.0) && (p.p242 > 0.0)) { 1.0 } else { 0.0 };
        var_guard1226 = assign37720_e51759;
        var_guard1226_rv = 0.0;

        let assign37740_e51766: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard1227 = assign37740_e51766;
        var_guard1227_rv = 0.0;

        let assign37750_e51775: f64 = if ((p.p37 != 0.0) || ((p.p25 == 1.0) && (p.p26 == 2.0))) { 1.0 } else { 0.0 };
        var_guard1228 = assign37750_e51775;
        var_guard1228_rv = 0.0;

        *var_guard1226_slot = var_guard1226;
        *var_guard1226_rv_slot = var_guard1226_rv;
        *var_guard1227_slot = var_guard1227;
        *var_guard1227_rv_slot = var_guard1227_rv;
        *var_guard1228_slot = var_guard1228;
        *var_guard1228_rv_slot = var_guard1228_rv;
        *var_ibdb_slot = var_ibdb;
        *var_ibdb_dn0_slot = var_ibdb_dn0;
        *var_ibdb_dn10_slot = var_ibdb_dn10;
        *var_ibdb_dn11_slot = var_ibdb_dn11;
        *var_ibdb_dn12_slot = var_ibdb_dn12;
        *var_ibdb_dn17_slot = var_ibdb_dn17;
        *var_ibdb_dn2_slot = var_ibdb_dn2;
        *var_ibdb_dn6_slot = var_ibdb_dn6;
        *var_ibdb_dn7_slot = var_ibdb_dn7;
        *var_ibdb_rv_slot = var_ibdb_rv;
        *var_ibsb_slot = var_ibsb;
        *var_ibsb_dn0_slot = var_ibsb_dn0;
        *var_ibsb_dn10_slot = var_ibsb_dn10;
        *var_ibsb_dn11_slot = var_ibsb_dn11;
        *var_ibsb_dn12_slot = var_ibsb_dn12;
        *var_ibsb_dn17_slot = var_ibsb_dn17;
        *var_ibsb_dn2_slot = var_ibsb_dn2;
        *var_ibsb_dn6_slot = var_ibsb_dn6;
        *var_ibsb_dn7_slot = var_ibsb_dn7;
        *var_ibsb_rv_slot = var_ibsb_rv;
        *var_qdrat_slot = var_qdrat;
        *var_qdrat_dn0_slot = var_qdrat_dn0;
        *var_qdrat_dn10_slot = var_qdrat_dn10;
        *var_qdrat_dn11_slot = var_qdrat_dn11;
        *var_qdrat_dn12_slot = var_qdrat_dn12;
        *var_qdrat_dn17_slot = var_qdrat_dn17;
        *var_qdrat_dn2_slot = var_qdrat_dn2;
        *var_qdrat_dn6_slot = var_qdrat_dn6;
        *var_qdrat_dn7_slot = var_qdrat_dn7;
        *var_qdrat_rv_slot = var_qdrat_rv;
        *var_sid_slot = var_sid;
        *var_sid_dn0_slot = var_sid_dn0;
        *var_sid_dn10_slot = var_sid_dn10;
        *var_sid_dn11_slot = var_sid_dn11;
        *var_sid_dn12_slot = var_sid_dn12;
        *var_sid_dn17_slot = var_sid_dn17;
        *var_sid_dn2_slot = var_sid_dn2;
        *var_sid_dn6_slot = var_sid_dn6;
        *var_sid_dn7_slot = var_sid_dn7;
        *var_sid_rv_slot = var_sid_rv;
        *var_sigrat_slot = var_sigrat;
        *var_sigrat_d_slot = var_sigrat_d;
        *var_sigrat_d_dn0_slot = var_sigrat_d_dn0;
        *var_sigrat_d_dn10_slot = var_sigrat_d_dn10;
        *var_sigrat_d_dn11_slot = var_sigrat_d_dn11;
        *var_sigrat_d_dn12_slot = var_sigrat_d_dn12;
        *var_sigrat_d_dn13_slot = var_sigrat_d_dn13;
        *var_sigrat_d_dn15_slot = var_sigrat_d_dn15;
        *var_sigrat_d_dn16_slot = var_sigrat_d_dn16;
        *var_sigrat_d_dn17_slot = var_sigrat_d_dn17;
        *var_sigrat_d_dn18_slot = var_sigrat_d_dn18;
        *var_sigrat_d_dn2_slot = var_sigrat_d_dn2;
        *var_sigrat_d_dn6_slot = var_sigrat_d_dn6;
        *var_sigrat_d_dn7_slot = var_sigrat_d_dn7;
        *var_sigrat_d_rv_slot = var_sigrat_d_rv;
        *var_sigrat_dn0_slot = var_sigrat_dn0;
        *var_sigrat_dn10_slot = var_sigrat_dn10;
        *var_sigrat_dn11_slot = var_sigrat_dn11;
        *var_sigrat_dn12_slot = var_sigrat_dn12;
        *var_sigrat_dn13_slot = var_sigrat_dn13;
        *var_sigrat_dn15_slot = var_sigrat_dn15;
        *var_sigrat_dn16_slot = var_sigrat_dn16;
        *var_sigrat_dn17_slot = var_sigrat_dn17;
        *var_sigrat_dn18_slot = var_sigrat_dn18;
        *var_sigrat_dn2_slot = var_sigrat_dn2;
        *var_sigrat_dn6_slot = var_sigrat_dn6;
        *var_sigrat_dn7_slot = var_sigrat_dn7;
        *var_sigrat_rv_slot = var_sigrat_rv;
        *var_sigrat_s_slot = var_sigrat_s;
        *var_sigrat_s_dn0_slot = var_sigrat_s_dn0;
        *var_sigrat_s_dn10_slot = var_sigrat_s_dn10;
        *var_sigrat_s_dn11_slot = var_sigrat_s_dn11;
        *var_sigrat_s_dn12_slot = var_sigrat_s_dn12;
        *var_sigrat_s_dn13_slot = var_sigrat_s_dn13;
        *var_sigrat_s_dn15_slot = var_sigrat_s_dn15;
        *var_sigrat_s_dn16_slot = var_sigrat_s_dn16;
        *var_sigrat_s_dn17_slot = var_sigrat_s_dn17;
        *var_sigrat_s_dn18_slot = var_sigrat_s_dn18;
        *var_sigrat_s_dn2_slot = var_sigrat_s_dn2;
        *var_sigrat_s_dn6_slot = var_sigrat_s_dn6;
        *var_sigrat_s_dn7_slot = var_sigrat_s_dn7;
        *var_sigrat_s_rv_slot = var_sigrat_s_rv;
        *var_whi_noise_slot = var_whi_noise;
        *var_whi_noise_dn10_slot = var_whi_noise_dn10;
        *var_whi_noise_rv_slot = var_whi_noise_rv;
    }

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        var_ci: f64,
        var_ci_dn0: f64,
        var_ci_dn10: f64,
        var_ci_dn11: f64,
        var_ci_dn12: f64,
        var_ci_dn17: f64,
        var_ci_dn2: f64,
        var_ci_dn6: f64,
        var_ci_dn7: f64,
        var_cthe: f64,
        var_grg: f64,
        var_gth: f64,
        var_guard1224: f64,
        var_guard1226: f64,
        var_guard1227: f64,
        var_ibs: f64,
        var_ibs_dn0: f64,
        var_ibs_dn10: f64,
        var_ibs_dn11: f64,
        var_ibs_dn12: f64,
        var_ibs_dn17: f64,
        var_ibs_dn2: f64,
        var_ibs_dn6: f64,
        var_ibs_dn7: f64,
        var_ids: f64,
        var_ids_dn0: f64,
        var_ids_dn10: f64,
        var_ids_dn11: f64,
        var_ids_dn12: f64,
        var_ids_dn17: f64,
        var_ids_dn2: f64,
        var_ids_dn6: f64,
        var_ids_dn7: f64,
        var_igb: f64,
        var_igb_dn0: f64,
        var_igb_dn10: f64,
        var_igb_dn11: f64,
        var_igb_dn12: f64,
        var_igb_dn17: f64,
        var_igb_dn2: f64,
        var_igb_dn6: f64,
        var_igb_dn7: f64,
        var_igd: f64,
        var_igd_dn0: f64,
        var_igd_dn10: f64,
        var_igd_dn11: f64,
        var_igd_dn12: f64,
        var_igd_dn17: f64,
        var_igd_dn2: f64,
        var_igd_dn6: f64,
        var_igd_dn7: f64,
        var_igidl: f64,
        var_igidl_dn0: f64,
        var_igidl_dn10: f64,
        var_igidl_dn11: f64,
        var_igidl_dn12: f64,
        var_igidl_dn17: f64,
        var_igidl_dn2: f64,
        var_igidl_dn6: f64,
        var_igidl_dn7: f64,
        var_igisl: f64,
        var_igisl_dn0: f64,
        var_igisl_dn10: f64,
        var_igisl_dn11: f64,
        var_igisl_dn12: f64,
        var_igisl_dn17: f64,
        var_igisl_dn2: f64,
        var_igisl_dn6: f64,
        var_igisl_dn7: f64,
        var_igs: f64,
        var_igs_dn0: f64,
        var_igs_dn10: f64,
        var_igs_dn11: f64,
        var_igs_dn12: f64,
        var_igs_dn17: f64,
        var_igs_dn2: f64,
        var_igs_dn6: f64,
        var_igs_dn7: f64,
        var_isub: f64,
        var_isub_dn0: f64,
        var_isub_dn10: f64,
        var_isub_dn11: f64,
        var_isub_dn12: f64,
        var_isub_dn17: f64,
        var_isub_dn2: f64,
        var_isub_dn6: f64,
        var_isub_dn7: f64,
        var_isubs: f64,
        var_isubs_dn0: f64,
        var_isubs_dn10: f64,
        var_isubs_dn11: f64,
        var_isubs_dn12: f64,
        var_isubs_dn17: f64,
        var_isubs_dn2: f64,
        var_isubs_dn6: f64,
        var_isubs_dn7: f64,
        var_itemp: f64,
        var_itemp_dn0: f64,
        var_itemp_dn10: f64,
        var_itemp_dn11: f64,
        var_itemp_dn12: f64,
        var_itemp_dn17: f64,
        var_itemp_dn2: f64,
        var_itemp_dn6: f64,
        var_itemp_dn7: f64,
        var_qb: f64,
        var_qb_dn0: f64,
        var_qb_dn10: f64,
        var_qb_dn11: f64,
        var_qb_dn12: f64,
        var_qb_dn13: f64,
        var_qb_dn15: f64,
        var_qb_dn16: f64,
        var_qb_dn17: f64,
        var_qb_dn18: f64,
        var_qb_dn2: f64,
        var_qb_dn6: f64,
        var_qb_dn7: f64,
        var_qbs: f64,
        var_qbs_dn0: f64,
        var_qbs_dn10: f64,
        var_qbs_dn11: f64,
        var_qbs_dn12: f64,
        var_qbs_dn17: f64,
        var_qbs_dn2: f64,
        var_qbs_dn6: f64,
        var_qbs_dn7: f64,
        var_qd: f64,
        var_qd_dn0: f64,
        var_qd_dn10: f64,
        var_qd_dn11: f64,
        var_qd_dn12: f64,
        var_qd_dn13: f64,
        var_qd_dn15: f64,
        var_qd_dn16: f64,
        var_qd_dn17: f64,
        var_qd_dn18: f64,
        var_qd_dn2: f64,
        var_qd_dn6: f64,
        var_qd_dn7: f64,
        var_qg: f64,
        var_qg_dn0: f64,
        var_qg_dn10: f64,
        var_qg_dn11: f64,
        var_qg_dn12: f64,
        var_qg_dn13: f64,
        var_qg_dn15: f64,
        var_qg_dn16: f64,
        var_qg_dn17: f64,
        var_qg_dn18: f64,
        var_qg_dn2: f64,
        var_qg_dn6: f64,
        var_qg_dn7: f64,
        var_rdd: f64,
        var_rdd_dn0: f64,
        var_rdd_dn10: f64,
        var_rdd_dn11: f64,
        var_rdd_dn12: f64,
        var_rdd_dn17: f64,
        var_rdd_dn2: f64,
        var_rdd_dn6: f64,
        var_rdd_dn7: f64,
        var_rsd: f64,
        var_rsd_dn0: f64,
        var_rsd_dn10: f64,
        var_rsd_dn11: f64,
        var_rsd_dn12: f64,
        var_rsd_dn17: f64,
        var_rsd_dn2: f64,
        var_rsd_dn6: f64,
        var_rsd_dn7: f64,
        var_sigrat_d: f64,
        var_sigrat_d_dn0: f64,
        var_sigrat_d_dn10: f64,
        var_sigrat_d_dn11: f64,
        var_sigrat_d_dn12: f64,
        var_sigrat_d_dn13: f64,
        var_sigrat_d_dn15: f64,
        var_sigrat_d_dn16: f64,
        var_sigrat_d_dn17: f64,
        var_sigrat_d_dn18: f64,
        var_sigrat_d_dn2: f64,
        var_sigrat_d_dn6: f64,
        var_sigrat_d_dn7: f64,
        var_sigrat_s: f64,
        var_sigrat_s_dn0: f64,
        var_sigrat_s_dn10: f64,
        var_sigrat_s_dn11: f64,
        var_sigrat_s_dn12: f64,
        var_sigrat_s_dn13: f64,
        var_sigrat_s_dn15: f64,
        var_sigrat_s_dn16: f64,
        var_sigrat_s_dn17: f64,
        var_sigrat_s_dn18: f64,
        var_sigrat_s_dn2: f64,
        var_sigrat_s_dn6: f64,
        var_sigrat_s_dn7: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let eq3_e324: f64 = (p.p50 * var_ids);
        let eq3_e324_d_n0: f64 = (p.p50 * var_ids_dn0);
        let eq3_e324_d_n2: f64 = (p.p50 * var_ids_dn2);
        let eq3_e324_d_n6: f64 = (p.p50 * var_ids_dn6);
        let eq3_e324_d_n7: f64 = (p.p50 * var_ids_dn7);
        let eq3_e324_d_n10: f64 = (p.p50 * var_ids_dn10);
        let eq3_e324_d_n11: f64 = (p.p50 * var_ids_dn11);
        let eq3_e324_d_n12: f64 = (p.p50 * var_ids_dn12);
        let eq3_e324_d_n17: f64 = (p.p50 * var_ids_dn17);
        let eq3_value: f64 = eq3_e324;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq3_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq3_e324_d_n0), multiplicity * (eq3_e324_d_n2), multiplicity * (eq3_e324_d_n6), multiplicity * (eq3_e324_d_n7), multiplicity * (eq3_e324_d_n10), multiplicity * (eq3_e324_d_n11), multiplicity * (eq3_e324_d_n12), multiplicity * (eq3_e324_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq4_e330, eq4_e330_d_n0, eq4_e330_d_n2, eq4_e330_d_n6, eq4_e330_d_n7, eq4_e330_d_n10, eq4_e330_d_n11, eq4_e330_d_n12, eq4_e330_d_n17,) = {
    if (var_guard1224 != 0.0) {
        let eq4_e328: f64 = (p.p50 * var_igs);
        let eq4_e328_d_n0: f64 = (p.p50 * var_igs_dn0);
        let eq4_e328_d_n2: f64 = (p.p50 * var_igs_dn2);
        let eq4_e328_d_n6: f64 = (p.p50 * var_igs_dn6);
        let eq4_e328_d_n7: f64 = (p.p50 * var_igs_dn7);
        let eq4_e328_d_n10: f64 = (p.p50 * var_igs_dn10);
        let eq4_e328_d_n11: f64 = (p.p50 * var_igs_dn11);
        let eq4_e328_d_n12: f64 = (p.p50 * var_igs_dn12);
        let eq4_e328_d_n17: f64 = (p.p50 * var_igs_dn17);
        (eq4_e328, eq4_e328_d_n0, eq4_e328_d_n2, eq4_e328_d_n6, eq4_e328_d_n7, eq4_e328_d_n10, eq4_e328_d_n11, eq4_e328_d_n12, eq4_e328_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e330;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(11),
            Some(7),
            multiplicity * (eq4_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq4_e330_d_n0), multiplicity * (eq4_e330_d_n2), multiplicity * (eq4_e330_d_n6), multiplicity * (eq4_e330_d_n7), multiplicity * (eq4_e330_d_n10), multiplicity * (eq4_e330_d_n11), multiplicity * (eq4_e330_d_n12), multiplicity * (eq4_e330_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq5_e336, eq5_e336_d_n0, eq5_e336_d_n2, eq5_e336_d_n6, eq5_e336_d_n7, eq5_e336_d_n10, eq5_e336_d_n11, eq5_e336_d_n12, eq5_e336_d_n17,) = {
    if (var_guard1224 != 0.0) {
        let eq5_e334: f64 = (p.p50 * var_igd);
        let eq5_e334_d_n0: f64 = (p.p50 * var_igd_dn0);
        let eq5_e334_d_n2: f64 = (p.p50 * var_igd_dn2);
        let eq5_e334_d_n6: f64 = (p.p50 * var_igd_dn6);
        let eq5_e334_d_n7: f64 = (p.p50 * var_igd_dn7);
        let eq5_e334_d_n10: f64 = (p.p50 * var_igd_dn10);
        let eq5_e334_d_n11: f64 = (p.p50 * var_igd_dn11);
        let eq5_e334_d_n12: f64 = (p.p50 * var_igd_dn12);
        let eq5_e334_d_n17: f64 = (p.p50 * var_igd_dn17);
        (eq5_e334, eq5_e334_d_n0, eq5_e334_d_n2, eq5_e334_d_n6, eq5_e334_d_n7, eq5_e334_d_n10, eq5_e334_d_n11, eq5_e334_d_n12, eq5_e334_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e336;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(11),
            Some(6),
            multiplicity * (eq5_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq5_e336_d_n0), multiplicity * (eq5_e336_d_n2), multiplicity * (eq5_e336_d_n6), multiplicity * (eq5_e336_d_n7), multiplicity * (eq5_e336_d_n10), multiplicity * (eq5_e336_d_n11), multiplicity * (eq5_e336_d_n12), multiplicity * (eq5_e336_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq6_e342, eq6_e342_d_n0, eq6_e342_d_n2, eq6_e342_d_n6, eq6_e342_d_n7, eq6_e342_d_n10, eq6_e342_d_n11, eq6_e342_d_n12, eq6_e342_d_n17,) = {
    if (var_guard1224 != 0.0) {
        let eq6_e340: f64 = (p.p50 * var_igb);
        let eq6_e340_d_n0: f64 = (p.p50 * var_igb_dn0);
        let eq6_e340_d_n2: f64 = (p.p50 * var_igb_dn2);
        let eq6_e340_d_n6: f64 = (p.p50 * var_igb_dn6);
        let eq6_e340_d_n7: f64 = (p.p50 * var_igb_dn7);
        let eq6_e340_d_n10: f64 = (p.p50 * var_igb_dn10);
        let eq6_e340_d_n11: f64 = (p.p50 * var_igb_dn11);
        let eq6_e340_d_n12: f64 = (p.p50 * var_igb_dn12);
        let eq6_e340_d_n17: f64 = (p.p50 * var_igb_dn17);
        (eq6_e340, eq6_e340_d_n0, eq6_e340_d_n2, eq6_e340_d_n6, eq6_e340_d_n7, eq6_e340_d_n10, eq6_e340_d_n11, eq6_e340_d_n12, eq6_e340_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e342;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(11),
            Some(12),
            multiplicity * (eq6_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq6_e342_d_n0), multiplicity * (eq6_e342_d_n2), multiplicity * (eq6_e342_d_n6), multiplicity * (eq6_e342_d_n7), multiplicity * (eq6_e342_d_n10), multiplicity * (eq6_e342_d_n11), multiplicity * (eq6_e342_d_n12), multiplicity * (eq6_e342_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq7_e348, eq7_e348_d_n0, eq7_e348_d_n2, eq7_e348_d_n6, eq7_e348_d_n7, eq7_e348_d_n10, eq7_e348_d_n11, eq7_e348_d_n12, eq7_e348_d_n17,) = {
    if (p.p259 != 0.0) {
        let eq7_e346: f64 = ((nv7 - nv2) / var_rsd);
        let eq7_e346_d_n0: f64 = (-(((nv7 - nv2) * var_rsd_dn0) / (var_rsd * var_rsd)));
        let eq7_e346_d_n2: f64 = (((-var_rsd) - ((nv7 - nv2) * var_rsd_dn2)) / (var_rsd * var_rsd));
        let eq7_e346_d_n6: f64 = (-(((nv7 - nv2) * var_rsd_dn6) / (var_rsd * var_rsd)));
        let eq7_e346_d_n7: f64 = ((var_rsd - ((nv7 - nv2) * var_rsd_dn7)) / (var_rsd * var_rsd));
        let eq7_e346_d_n10: f64 = (-(((nv7 - nv2) * var_rsd_dn10) / (var_rsd * var_rsd)));
        let eq7_e346_d_n11: f64 = (-(((nv7 - nv2) * var_rsd_dn11) / (var_rsd * var_rsd)));
        let eq7_e346_d_n12: f64 = (-(((nv7 - nv2) * var_rsd_dn12) / (var_rsd * var_rsd)));
        let eq7_e346_d_n17: f64 = (-(((nv7 - nv2) * var_rsd_dn17) / (var_rsd * var_rsd)));
        (eq7_e346, eq7_e346_d_n0, eq7_e346_d_n2, eq7_e346_d_n6, eq7_e346_d_n7, eq7_e346_d_n10, eq7_e346_d_n11, eq7_e346_d_n12, eq7_e346_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e348;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(7),
            Some(2),
            multiplicity * (eq7_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq7_e348_d_n0), multiplicity * (eq7_e348_d_n2), multiplicity * (eq7_e348_d_n6), multiplicity * (eq7_e348_d_n7), multiplicity * (eq7_e348_d_n10), multiplicity * (eq7_e348_d_n11), multiplicity * (eq7_e348_d_n12), multiplicity * (eq7_e348_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq9_e359, eq9_e359_d_n0, eq9_e359_d_n2, eq9_e359_d_n6, eq9_e359_d_n7, eq9_e359_d_n10, eq9_e359_d_n11, eq9_e359_d_n12, eq9_e359_d_n17,) = {
    if (p.p260 != 0.0) {
        let eq9_e357: f64 = ((nv0 - nv6) / var_rdd);
        let eq9_e357_d_n0: f64 = ((var_rdd - ((nv0 - nv6) * var_rdd_dn0)) / (var_rdd * var_rdd));
        let eq9_e357_d_n2: f64 = (-(((nv0 - nv6) * var_rdd_dn2) / (var_rdd * var_rdd)));
        let eq9_e357_d_n6: f64 = (((-var_rdd) - ((nv0 - nv6) * var_rdd_dn6)) / (var_rdd * var_rdd));
        let eq9_e357_d_n7: f64 = (-(((nv0 - nv6) * var_rdd_dn7) / (var_rdd * var_rdd)));
        let eq9_e357_d_n10: f64 = (-(((nv0 - nv6) * var_rdd_dn10) / (var_rdd * var_rdd)));
        let eq9_e357_d_n11: f64 = (-(((nv0 - nv6) * var_rdd_dn11) / (var_rdd * var_rdd)));
        let eq9_e357_d_n12: f64 = (-(((nv0 - nv6) * var_rdd_dn12) / (var_rdd * var_rdd)));
        let eq9_e357_d_n17: f64 = (-(((nv0 - nv6) * var_rdd_dn17) / (var_rdd * var_rdd)));
        (eq9_e357, eq9_e357_d_n0, eq9_e357_d_n2, eq9_e357_d_n6, eq9_e357_d_n7, eq9_e357_d_n10, eq9_e357_d_n11, eq9_e357_d_n12, eq9_e357_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e359;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(0),
            Some(6),
            multiplicity * (eq9_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq9_e359_d_n0), multiplicity * (eq9_e359_d_n2), multiplicity * (eq9_e359_d_n6), multiplicity * (eq9_e359_d_n7), multiplicity * (eq9_e359_d_n10), multiplicity * (eq9_e359_d_n11), multiplicity * (eq9_e359_d_n12), multiplicity * (eq9_e359_d_n17)],
            [],
            [],
            1.0,
        );
        let eq11_e367: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, var_qg);
        let eq11_e368: f64 = (p.p50 * eq11_e367);
        let eq11_e368_d_n0: f64 = (p.p50 * (var_qg_dn0 * ddt_scale));
        let eq11_e368_d_n2: f64 = (p.p50 * (var_qg_dn2 * ddt_scale));
        let eq11_e368_d_n6: f64 = (p.p50 * (var_qg_dn6 * ddt_scale));
        let eq11_e368_d_n7: f64 = (p.p50 * (var_qg_dn7 * ddt_scale));
        let eq11_e368_d_n10: f64 = (p.p50 * (var_qg_dn10 * ddt_scale));
        let eq11_e368_d_n11: f64 = (p.p50 * (var_qg_dn11 * ddt_scale));
        let eq11_e368_d_n12: f64 = (p.p50 * (var_qg_dn12 * ddt_scale));
        let eq11_e368_d_n13: f64 = (p.p50 * (var_qg_dn13 * ddt_scale));
        let eq11_e368_d_n15: f64 = (p.p50 * (var_qg_dn15 * ddt_scale));
        let eq11_e368_d_n16: f64 = (p.p50 * (var_qg_dn16 * ddt_scale));
        let eq11_e368_d_n17: f64 = (p.p50 * (var_qg_dn17 * ddt_scale));
        let eq11_e368_d_n18: f64 = (p.p50 * (var_qg_dn18 * ddt_scale));
        let eq11_value: f64 = eq11_e368;
        let eq11_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq11_node_derivatives: [f64; 12] = [eq11_e368_d_n0, eq11_e368_d_n2, eq11_e368_d_n6, eq11_e368_d_n7, eq11_e368_d_n10, eq11_e368_d_n11, eq11_e368_d_n12, eq11_e368_d_n13, eq11_e368_d_n15, eq11_e368_d_n16, eq11_e368_d_n17, eq11_e368_d_n18];
        let eq11_branch_derivative_indices: [usize; 0] = [];
        let eq11_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq11_value),
            &eq11_node_derivative_indices,
            &eq11_node_derivatives,
            &eq11_branch_derivative_indices,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let eq12_e371: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, var_qd);
        let eq12_e372: f64 = (p.p50 * eq12_e371);
        let eq12_e372_d_n0: f64 = (p.p50 * (var_qd_dn0 * ddt_scale));
        let eq12_e372_d_n2: f64 = (p.p50 * (var_qd_dn2 * ddt_scale));
        let eq12_e372_d_n6: f64 = (p.p50 * (var_qd_dn6 * ddt_scale));
        let eq12_e372_d_n7: f64 = (p.p50 * (var_qd_dn7 * ddt_scale));
        let eq12_e372_d_n10: f64 = (p.p50 * (var_qd_dn10 * ddt_scale));
        let eq12_e372_d_n11: f64 = (p.p50 * (var_qd_dn11 * ddt_scale));
        let eq12_e372_d_n12: f64 = (p.p50 * (var_qd_dn12 * ddt_scale));
        let eq12_e372_d_n13: f64 = (p.p50 * (var_qd_dn13 * ddt_scale));
        let eq12_e372_d_n15: f64 = (p.p50 * (var_qd_dn15 * ddt_scale));
        let eq12_e372_d_n16: f64 = (p.p50 * (var_qd_dn16 * ddt_scale));
        let eq12_e372_d_n17: f64 = (p.p50 * (var_qd_dn17 * ddt_scale));
        let eq12_e372_d_n18: f64 = (p.p50 * (var_qd_dn18 * ddt_scale));
        let eq12_value: f64 = eq12_e372;
        let eq12_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq12_node_derivatives: [f64; 12] = [eq12_e372_d_n0, eq12_e372_d_n2, eq12_e372_d_n6, eq12_e372_d_n7, eq12_e372_d_n10, eq12_e372_d_n11, eq12_e372_d_n12, eq12_e372_d_n13, eq12_e372_d_n15, eq12_e372_d_n16, eq12_e372_d_n17, eq12_e372_d_n18];
        let eq12_branch_derivative_indices: [usize; 0] = [];
        let eq12_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq12_value),
            &eq12_node_derivative_indices,
            &eq12_node_derivatives,
            &eq12_branch_derivative_indices,
            &eq12_branch_derivatives,
            multiplicity,
        );
        let eq13_e375: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, var_qb);
        let eq13_e376: f64 = (p.p50 * eq13_e375);
        let eq13_e376_d_n0: f64 = (p.p50 * (var_qb_dn0 * ddt_scale));
        let eq13_e376_d_n2: f64 = (p.p50 * (var_qb_dn2 * ddt_scale));
        let eq13_e376_d_n6: f64 = (p.p50 * (var_qb_dn6 * ddt_scale));
        let eq13_e376_d_n7: f64 = (p.p50 * (var_qb_dn7 * ddt_scale));
        let eq13_e376_d_n10: f64 = (p.p50 * (var_qb_dn10 * ddt_scale));
        let eq13_e376_d_n11: f64 = (p.p50 * (var_qb_dn11 * ddt_scale));
        let eq13_e376_d_n12: f64 = (p.p50 * (var_qb_dn12 * ddt_scale));
        let eq13_e376_d_n13: f64 = (p.p50 * (var_qb_dn13 * ddt_scale));
        let eq13_e376_d_n15: f64 = (p.p50 * (var_qb_dn15 * ddt_scale));
        let eq13_e376_d_n16: f64 = (p.p50 * (var_qb_dn16 * ddt_scale));
        let eq13_e376_d_n17: f64 = (p.p50 * (var_qb_dn17 * ddt_scale));
        let eq13_e376_d_n18: f64 = (p.p50 * (var_qb_dn18 * ddt_scale));
        let eq13_value: f64 = eq13_e376;
        let eq13_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq13_node_derivatives: [f64; 12] = [eq13_e376_d_n0, eq13_e376_d_n2, eq13_e376_d_n6, eq13_e376_d_n7, eq13_e376_d_n10, eq13_e376_d_n11, eq13_e376_d_n12, eq13_e376_d_n13, eq13_e376_d_n15, eq13_e376_d_n16, eq13_e376_d_n17, eq13_e376_d_n18];
        let eq13_branch_derivative_indices: [usize; 0] = [];
        let eq13_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(12),
            Some(7),
            multiplicity * (eq13_value),
            &eq13_node_derivative_indices,
            &eq13_node_derivatives,
            &eq13_branch_derivative_indices,
            &eq13_branch_derivatives,
            multiplicity,
        );
        let eq18_e402: f64 = (var_ci * (nv14 - 0.0));
        let eq18_e402_d_n0: f64 = (var_ci_dn0 * (nv14 - 0.0));
        let eq18_e402_d_n2: f64 = (var_ci_dn2 * (nv14 - 0.0));
        let eq18_e402_d_n6: f64 = (var_ci_dn6 * (nv14 - 0.0));
        let eq18_e402_d_n7: f64 = (var_ci_dn7 * (nv14 - 0.0));
        let eq18_e402_d_n10: f64 = (var_ci_dn10 * (nv14 - 0.0));
        let eq18_e402_d_n11: f64 = (var_ci_dn11 * (nv14 - 0.0));
        let eq18_e402_d_n12: f64 = (var_ci_dn12 * (nv14 - 0.0));
        let eq18_e402_d_n17: f64 = (var_ci_dn17 * (nv14 - 0.0));
        let eq18_value: f64 = eq18_e402;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq18_value),
            [0, 2, 6, 7, 10, 11, 12, 14, 17],
            [multiplicity * (eq18_e402_d_n0), multiplicity * (eq18_e402_d_n2), multiplicity * (eq18_e402_d_n6), multiplicity * (eq18_e402_d_n7), multiplicity * (eq18_e402_d_n10), multiplicity * (eq18_e402_d_n11), multiplicity * (eq18_e402_d_n12), multiplicity * (var_ci), multiplicity * (eq18_e402_d_n17)],
            [],
            [],
            1.0,
        );
        let eq19_e405: f64 = ((nv14 - 0.0) * var_sigrat_s);
        let eq19_e405_d_n0: f64 = ((nv14 - 0.0) * var_sigrat_s_dn0);
        let eq19_e405_d_n2: f64 = ((nv14 - 0.0) * var_sigrat_s_dn2);
        let eq19_e405_d_n6: f64 = ((nv14 - 0.0) * var_sigrat_s_dn6);
        let eq19_e405_d_n7: f64 = ((nv14 - 0.0) * var_sigrat_s_dn7);
        let eq19_e405_d_n10: f64 = ((nv14 - 0.0) * var_sigrat_s_dn10);
        let eq19_e405_d_n11: f64 = ((nv14 - 0.0) * var_sigrat_s_dn11);
        let eq19_e405_d_n12: f64 = ((nv14 - 0.0) * var_sigrat_s_dn12);
        let eq19_e405_d_n13: f64 = ((nv14 - 0.0) * var_sigrat_s_dn13);
        let eq19_e405_d_n15: f64 = ((nv14 - 0.0) * var_sigrat_s_dn15);
        let eq19_e405_d_n16: f64 = ((nv14 - 0.0) * var_sigrat_s_dn16);
        let eq19_e405_d_n17: f64 = ((nv14 - 0.0) * var_sigrat_s_dn17);
        let eq19_e405_d_n18: f64 = ((nv14 - 0.0) * var_sigrat_s_dn18);
        let eq19_e406: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq19_e405);
        let eq19_value: f64 = eq19_e406;
        let eq19_node_derivative_indices: [usize; 13] = [0, 2, 6, 7, 10, 11, 12, 13, 14, 15, 16, 17, 18];
        let eq19_node_derivatives: [f64; 13] = [(eq19_e405_d_n0 * ddt_scale), (eq19_e405_d_n2 * ddt_scale), (eq19_e405_d_n6 * ddt_scale), (eq19_e405_d_n7 * ddt_scale), (eq19_e405_d_n10 * ddt_scale), (eq19_e405_d_n11 * ddt_scale), (eq19_e405_d_n12 * ddt_scale), (eq19_e405_d_n13 * ddt_scale), (var_sigrat_s * ddt_scale), (eq19_e405_d_n15 * ddt_scale), (eq19_e405_d_n16 * ddt_scale), (eq19_e405_d_n17 * ddt_scale), (eq19_e405_d_n18 * ddt_scale)];
        let eq19_branch_derivative_indices: [usize; 0] = [];
        let eq19_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq19_value),
            &eq19_node_derivative_indices,
            &eq19_node_derivatives,
            &eq19_branch_derivative_indices,
            &eq19_branch_derivatives,
            multiplicity,
        );
        let eq20_e409: f64 = ((nv14 - 0.0) * var_sigrat_d);
        let eq20_e409_d_n0: f64 = ((nv14 - 0.0) * var_sigrat_d_dn0);
        let eq20_e409_d_n2: f64 = ((nv14 - 0.0) * var_sigrat_d_dn2);
        let eq20_e409_d_n6: f64 = ((nv14 - 0.0) * var_sigrat_d_dn6);
        let eq20_e409_d_n7: f64 = ((nv14 - 0.0) * var_sigrat_d_dn7);
        let eq20_e409_d_n10: f64 = ((nv14 - 0.0) * var_sigrat_d_dn10);
        let eq20_e409_d_n11: f64 = ((nv14 - 0.0) * var_sigrat_d_dn11);
        let eq20_e409_d_n12: f64 = ((nv14 - 0.0) * var_sigrat_d_dn12);
        let eq20_e409_d_n13: f64 = ((nv14 - 0.0) * var_sigrat_d_dn13);
        let eq20_e409_d_n15: f64 = ((nv14 - 0.0) * var_sigrat_d_dn15);
        let eq20_e409_d_n16: f64 = ((nv14 - 0.0) * var_sigrat_d_dn16);
        let eq20_e409_d_n17: f64 = ((nv14 - 0.0) * var_sigrat_d_dn17);
        let eq20_e409_d_n18: f64 = ((nv14 - 0.0) * var_sigrat_d_dn18);
        let eq20_e410: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq20_e409);
        let eq20_value: f64 = eq20_e410;
        let eq20_node_derivative_indices: [usize; 13] = [0, 2, 6, 7, 10, 11, 12, 13, 14, 15, 16, 17, 18];
        let eq20_node_derivatives: [f64; 13] = [(eq20_e409_d_n0 * ddt_scale), (eq20_e409_d_n2 * ddt_scale), (eq20_e409_d_n6 * ddt_scale), (eq20_e409_d_n7 * ddt_scale), (eq20_e409_d_n10 * ddt_scale), (eq20_e409_d_n11 * ddt_scale), (eq20_e409_d_n12 * ddt_scale), (eq20_e409_d_n13 * ddt_scale), (var_sigrat_d * ddt_scale), (eq20_e409_d_n15 * ddt_scale), (eq20_e409_d_n16 * ddt_scale), (eq20_e409_d_n17 * ddt_scale), (eq20_e409_d_n18 * ddt_scale)];
        let eq20_branch_derivative_indices: [usize; 0] = [];
        let eq20_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(6),
            multiplicity * (eq20_value),
            &eq20_node_derivative_indices,
            &eq20_node_derivatives,
            &eq20_branch_derivative_indices,
            &eq20_branch_derivatives,
            multiplicity,
        );
        let (eq26_e462, eq26_e462_d_n1, eq26_e462_d_n11,) = {
    if (p.p35 != 0.0) {
        let eq26_e460: f64 = (var_grg * (nv1 - nv11));
        (eq26_e460, var_grg, (-var_grg),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq26_value: f64 = eq26_e462;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(11),
            multiplicity * (eq26_value),
            1,
            multiplicity * (eq26_e462_d_n1),
            11,
            multiplicity * (eq26_e462_d_n11),
        );
        let (eq28_e473, eq28_e473_d_n10,) = {
    if (var_guard1226 != 0.0) {
        let eq28_e471: f64 = ((nv10 - 0.0) * var_gth);
        (eq28_e471, var_gth,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq28_value: f64 = eq28_e473;
        stamper.stamp_current_node1_local(
            Some(10),
            None,
            multiplicity * (eq28_value),
            10,
            multiplicity * (eq28_e473_d_n10),
        );
        let (eq29_e478, eq29_e478_d_n0, eq29_e478_d_n2, eq29_e478_d_n6, eq29_e478_d_n7, eq29_e478_d_n10, eq29_e478_d_n11, eq29_e478_d_n12, eq29_e478_d_n17,) = {
    if (var_guard1226 != 0.0) {
        let eq29_e476: f64 = (-var_itemp);
        (eq29_e476, (-var_itemp_dn0), (-var_itemp_dn2), (-var_itemp_dn6), (-var_itemp_dn7), (-var_itemp_dn10), (-var_itemp_dn11), (-var_itemp_dn12), (-var_itemp_dn17),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq29_value: f64 = eq29_e478;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(10),
            None,
            multiplicity * (eq29_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq29_e478_d_n0), multiplicity * (eq29_e478_d_n2), multiplicity * (eq29_e478_d_n6), multiplicity * (eq29_e478_d_n7), multiplicity * (eq29_e478_d_n10), multiplicity * (eq29_e478_d_n11), multiplicity * (eq29_e478_d_n12), multiplicity * (eq29_e478_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq31_e491, eq31_e491_d_n10,) = {
    if (var_guard1226 != 0.0) {
        let eq31_e488: f64 = (var_cthe * (nv10 - 0.0));
        let eq31_e489: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq31_e488);
        (eq31_e489, (var_cthe * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq31_value: f64 = eq31_e491;
        stamper.stamp_current_node1_local(
            Some(10),
            None,
            multiplicity * (eq31_value),
            10,
            multiplicity * (eq31_e491_d_n10),
        );
        let (eq33_e506, eq33_e506_d_n0, eq33_e506_d_n2, eq33_e506_d_n6, eq33_e506_d_n7, eq33_e506_d_n10, eq33_e506_d_n11, eq33_e506_d_n12, eq33_e506_d_n17,) = {
    if (var_guard1227 != 0.0) {
        let eq33_e503: f64 = (var_igidl + var_isub);
        let eq33_e503_d_n0: f64 = (var_igidl_dn0 + var_isub_dn0);
        let eq33_e503_d_n2: f64 = (var_igidl_dn2 + var_isub_dn2);
        let eq33_e503_d_n6: f64 = (var_igidl_dn6 + var_isub_dn6);
        let eq33_e503_d_n7: f64 = (var_igidl_dn7 + var_isub_dn7);
        let eq33_e503_d_n10: f64 = (var_igidl_dn10 + var_isub_dn10);
        let eq33_e503_d_n11: f64 = (var_igidl_dn11 + var_isub_dn11);
        let eq33_e503_d_n12: f64 = (var_igidl_dn12 + var_isub_dn12);
        let eq33_e503_d_n17: f64 = (var_igidl_dn17 + var_isub_dn17);
        let eq33_e504: f64 = (p.p50 * eq33_e503);
        let eq33_e504_d_n0: f64 = (p.p50 * eq33_e503_d_n0);
        let eq33_e504_d_n2: f64 = (p.p50 * eq33_e503_d_n2);
        let eq33_e504_d_n6: f64 = (p.p50 * eq33_e503_d_n6);
        let eq33_e504_d_n7: f64 = (p.p50 * eq33_e503_d_n7);
        let eq33_e504_d_n10: f64 = (p.p50 * eq33_e503_d_n10);
        let eq33_e504_d_n11: f64 = (p.p50 * eq33_e503_d_n11);
        let eq33_e504_d_n12: f64 = (p.p50 * eq33_e503_d_n12);
        let eq33_e504_d_n17: f64 = (p.p50 * eq33_e503_d_n17);
        (eq33_e504, eq33_e504_d_n0, eq33_e504_d_n2, eq33_e504_d_n6, eq33_e504_d_n7, eq33_e504_d_n10, eq33_e504_d_n11, eq33_e504_d_n12, eq33_e504_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e506;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(6),
            Some(12),
            multiplicity * (eq33_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq33_e506_d_n0), multiplicity * (eq33_e506_d_n2), multiplicity * (eq33_e506_d_n6), multiplicity * (eq33_e506_d_n7), multiplicity * (eq33_e506_d_n10), multiplicity * (eq33_e506_d_n11), multiplicity * (eq33_e506_d_n12), multiplicity * (eq33_e506_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq34_e514, eq34_e514_d_n0, eq34_e514_d_n2, eq34_e514_d_n6, eq34_e514_d_n7, eq34_e514_d_n10, eq34_e514_d_n11, eq34_e514_d_n12, eq34_e514_d_n17,) = {
    if (var_guard1227 != 0.0) {
        let eq34_e511: f64 = (var_igisl + var_isubs);
        let eq34_e511_d_n0: f64 = (var_igisl_dn0 + var_isubs_dn0);
        let eq34_e511_d_n2: f64 = (var_igisl_dn2 + var_isubs_dn2);
        let eq34_e511_d_n6: f64 = (var_igisl_dn6 + var_isubs_dn6);
        let eq34_e511_d_n7: f64 = (var_igisl_dn7 + var_isubs_dn7);
        let eq34_e511_d_n10: f64 = (var_igisl_dn10 + var_isubs_dn10);
        let eq34_e511_d_n11: f64 = (var_igisl_dn11 + var_isubs_dn11);
        let eq34_e511_d_n12: f64 = (var_igisl_dn12 + var_isubs_dn12);
        let eq34_e511_d_n17: f64 = (var_igisl_dn17 + var_isubs_dn17);
        let eq34_e512: f64 = (p.p50 * eq34_e511);
        let eq34_e512_d_n0: f64 = (p.p50 * eq34_e511_d_n0);
        let eq34_e512_d_n2: f64 = (p.p50 * eq34_e511_d_n2);
        let eq34_e512_d_n6: f64 = (p.p50 * eq34_e511_d_n6);
        let eq34_e512_d_n7: f64 = (p.p50 * eq34_e511_d_n7);
        let eq34_e512_d_n10: f64 = (p.p50 * eq34_e511_d_n10);
        let eq34_e512_d_n11: f64 = (p.p50 * eq34_e511_d_n11);
        let eq34_e512_d_n12: f64 = (p.p50 * eq34_e511_d_n12);
        let eq34_e512_d_n17: f64 = (p.p50 * eq34_e511_d_n17);
        (eq34_e512, eq34_e512_d_n0, eq34_e512_d_n2, eq34_e512_d_n6, eq34_e512_d_n7, eq34_e512_d_n10, eq34_e512_d_n11, eq34_e512_d_n12, eq34_e512_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq34_value: f64 = eq34_e514;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(7),
            Some(12),
            multiplicity * (eq34_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq34_e514_d_n0), multiplicity * (eq34_e514_d_n2), multiplicity * (eq34_e514_d_n6), multiplicity * (eq34_e514_d_n7), multiplicity * (eq34_e514_d_n10), multiplicity * (eq34_e514_d_n11), multiplicity * (eq34_e514_d_n12), multiplicity * (eq34_e514_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq35_e523, eq35_e523_d_n0, eq35_e523_d_n2, eq35_e523_d_n6, eq35_e523_d_n7, eq35_e523_d_n10, eq35_e523_d_n11, eq35_e523_d_n12, eq35_e523_d_n17,) = {
    if (var_guard1227 != 0.0) {
        let eq35_e519: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, var_qbs);
        let eq35_e520: f64 = (var_ibs + eq35_e519);
        let eq35_e520_d_n0: f64 = (var_ibs_dn0 + (var_qbs_dn0 * ddt_scale));
        let eq35_e520_d_n2: f64 = (var_ibs_dn2 + (var_qbs_dn2 * ddt_scale));
        let eq35_e520_d_n6: f64 = (var_ibs_dn6 + (var_qbs_dn6 * ddt_scale));
        let eq35_e520_d_n7: f64 = (var_ibs_dn7 + (var_qbs_dn7 * ddt_scale));
        let eq35_e520_d_n10: f64 = (var_ibs_dn10 + (var_qbs_dn10 * ddt_scale));
        let eq35_e520_d_n11: f64 = (var_ibs_dn11 + (var_qbs_dn11 * ddt_scale));
        let eq35_e520_d_n12: f64 = (var_ibs_dn12 + (var_qbs_dn12 * ddt_scale));
        let eq35_e520_d_n17: f64 = (var_ibs_dn17 + (var_qbs_dn17 * ddt_scale));
        let eq35_e521: f64 = (p.p50 * eq35_e520);
        let eq35_e521_d_n0: f64 = (p.p50 * eq35_e520_d_n0);
        let eq35_e521_d_n2: f64 = (p.p50 * eq35_e520_d_n2);
        let eq35_e521_d_n6: f64 = (p.p50 * eq35_e520_d_n6);
        let eq35_e521_d_n7: f64 = (p.p50 * eq35_e520_d_n7);
        let eq35_e521_d_n10: f64 = (p.p50 * eq35_e520_d_n10);
        let eq35_e521_d_n11: f64 = (p.p50 * eq35_e520_d_n11);
        let eq35_e521_d_n12: f64 = (p.p50 * eq35_e520_d_n12);
        let eq35_e521_d_n17: f64 = (p.p50 * eq35_e520_d_n17);
        (eq35_e521, eq35_e521_d_n0, eq35_e521_d_n2, eq35_e521_d_n6, eq35_e521_d_n7, eq35_e521_d_n10, eq35_e521_d_n11, eq35_e521_d_n12, eq35_e521_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_value: f64 = eq35_e523;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(9),
            Some(7),
            multiplicity * (eq35_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq35_e523_d_n0), multiplicity * (eq35_e523_d_n2), multiplicity * (eq35_e523_d_n6), multiplicity * (eq35_e523_d_n7), multiplicity * (eq35_e523_d_n10), multiplicity * (eq35_e523_d_n11), multiplicity * (eq35_e523_d_n12), multiplicity * (eq35_e523_d_n17)],
            [],
            [],
            1.0,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        var_guard1227: f64,
        var_guard1228: f64,
        var_ibd: f64,
        var_ibd_dn0: f64,
        var_ibd_dn10: f64,
        var_ibd_dn11: f64,
        var_ibd_dn12: f64,
        var_ibd_dn17: f64,
        var_ibd_dn2: f64,
        var_ibd_dn6: f64,
        var_ibd_dn7: f64,
        var_igidl: f64,
        var_igidl_dn0: f64,
        var_igidl_dn10: f64,
        var_igidl_dn11: f64,
        var_igidl_dn12: f64,
        var_igidl_dn17: f64,
        var_igidl_dn2: f64,
        var_igidl_dn6: f64,
        var_igidl_dn7: f64,
        var_igisl: f64,
        var_igisl_dn0: f64,
        var_igisl_dn10: f64,
        var_igisl_dn11: f64,
        var_igisl_dn12: f64,
        var_igisl_dn17: f64,
        var_igisl_dn2: f64,
        var_igisl_dn6: f64,
        var_igisl_dn7: f64,
        var_iqb_nqs: f64,
        var_iqb_nqs_dn0: f64,
        var_iqb_nqs_dn10: f64,
        var_iqb_nqs_dn11: f64,
        var_iqb_nqs_dn12: f64,
        var_iqb_nqs_dn13: f64,
        var_iqb_nqs_dn15: f64,
        var_iqb_nqs_dn16: f64,
        var_iqb_nqs_dn17: f64,
        var_iqb_nqs_dn18: f64,
        var_iqb_nqs_dn2: f64,
        var_iqb_nqs_dn6: f64,
        var_iqb_nqs_dn7: f64,
        var_iqd_nqs: f64,
        var_iqd_nqs_dn0: f64,
        var_iqd_nqs_dn10: f64,
        var_iqd_nqs_dn11: f64,
        var_iqd_nqs_dn12: f64,
        var_iqd_nqs_dn13: f64,
        var_iqd_nqs_dn15: f64,
        var_iqd_nqs_dn16: f64,
        var_iqd_nqs_dn17: f64,
        var_iqd_nqs_dn18: f64,
        var_iqd_nqs_dn2: f64,
        var_iqd_nqs_dn6: f64,
        var_iqd_nqs_dn7: f64,
        var_iqh_nqs: f64,
        var_iqh_nqs_dn0: f64,
        var_iqh_nqs_dn10: f64,
        var_iqh_nqs_dn11: f64,
        var_iqh_nqs_dn12: f64,
        var_iqh_nqs_dn17: f64,
        var_iqh_nqs_dn2: f64,
        var_iqh_nqs_dn6: f64,
        var_iqh_nqs_dn7: f64,
        var_iqi_nqs: f64,
        var_iqi_nqs_dn0: f64,
        var_iqi_nqs_dn10: f64,
        var_iqi_nqs_dn11: f64,
        var_iqi_nqs_dn12: f64,
        var_iqi_nqs_dn17: f64,
        var_iqi_nqs_dn18: f64,
        var_iqi_nqs_dn2: f64,
        var_iqi_nqs_dn6: f64,
        var_iqi_nqs_dn7: f64,
        var_iqs_nqs: f64,
        var_iqs_nqs_dn0: f64,
        var_iqs_nqs_dn10: f64,
        var_iqs_nqs_dn11: f64,
        var_iqs_nqs_dn12: f64,
        var_iqs_nqs_dn13: f64,
        var_iqs_nqs_dn15: f64,
        var_iqs_nqs_dn16: f64,
        var_iqs_nqs_dn17: f64,
        var_iqs_nqs_dn18: f64,
        var_iqs_nqs_dn2: f64,
        var_iqs_nqs_dn6: f64,
        var_iqs_nqs_dn7: f64,
        var_isub: f64,
        var_isub_dn0: f64,
        var_isub_dn10: f64,
        var_isub_dn11: f64,
        var_isub_dn12: f64,
        var_isub_dn17: f64,
        var_isub_dn2: f64,
        var_isub_dn6: f64,
        var_isub_dn7: f64,
        var_isubs: f64,
        var_isubs_dn0: f64,
        var_isubs_dn10: f64,
        var_isubs_dn11: f64,
        var_isubs_dn12: f64,
        var_isubs_dn17: f64,
        var_isubs_dn2: f64,
        var_isubs_dn6: f64,
        var_isubs_dn7: f64,
        var_qbd: f64,
        var_qbd_dn0: f64,
        var_qbd_dn10: f64,
        var_qbd_dn11: f64,
        var_qbd_dn12: f64,
        var_qbd_dn17: f64,
        var_qbd_dn2: f64,
        var_qbd_dn6: f64,
        var_qbd_dn7: f64,
        var_rbulk: f64,
        var_rbulk_dn0: f64,
        var_rbulk_dn10: f64,
        var_rbulk_dn11: f64,
        var_rbulk_dn12: f64,
        var_rbulk_dn17: f64,
        var_rbulk_dn2: f64,
        var_rbulk_dn6: f64,
        var_rbulk_dn7: f64,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let (eq36_e532, eq36_e532_d_n0, eq36_e532_d_n2, eq36_e532_d_n6, eq36_e532_d_n7, eq36_e532_d_n10, eq36_e532_d_n11, eq36_e532_d_n12, eq36_e532_d_n17,) = {
    if (var_guard1227 != 0.0) {
        let eq36_e528: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, var_qbd);
        let eq36_e529: f64 = (var_ibd + eq36_e528);
        let eq36_e529_d_n0: f64 = (var_ibd_dn0 + (var_qbd_dn0 * ddt_scale));
        let eq36_e529_d_n2: f64 = (var_ibd_dn2 + (var_qbd_dn2 * ddt_scale));
        let eq36_e529_d_n6: f64 = (var_ibd_dn6 + (var_qbd_dn6 * ddt_scale));
        let eq36_e529_d_n7: f64 = (var_ibd_dn7 + (var_qbd_dn7 * ddt_scale));
        let eq36_e529_d_n10: f64 = (var_ibd_dn10 + (var_qbd_dn10 * ddt_scale));
        let eq36_e529_d_n11: f64 = (var_ibd_dn11 + (var_qbd_dn11 * ddt_scale));
        let eq36_e529_d_n12: f64 = (var_ibd_dn12 + (var_qbd_dn12 * ddt_scale));
        let eq36_e529_d_n17: f64 = (var_ibd_dn17 + (var_qbd_dn17 * ddt_scale));
        let eq36_e530: f64 = (p.p50 * eq36_e529);
        let eq36_e530_d_n0: f64 = (p.p50 * eq36_e529_d_n0);
        let eq36_e530_d_n2: f64 = (p.p50 * eq36_e529_d_n2);
        let eq36_e530_d_n6: f64 = (p.p50 * eq36_e529_d_n6);
        let eq36_e530_d_n7: f64 = (p.p50 * eq36_e529_d_n7);
        let eq36_e530_d_n10: f64 = (p.p50 * eq36_e529_d_n10);
        let eq36_e530_d_n11: f64 = (p.p50 * eq36_e529_d_n11);
        let eq36_e530_d_n12: f64 = (p.p50 * eq36_e529_d_n12);
        let eq36_e530_d_n17: f64 = (p.p50 * eq36_e529_d_n17);
        (eq36_e530, eq36_e530_d_n0, eq36_e530_d_n2, eq36_e530_d_n6, eq36_e530_d_n7, eq36_e530_d_n10, eq36_e530_d_n11, eq36_e530_d_n12, eq36_e530_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_value: f64 = eq36_e532;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(8),
            Some(6),
            multiplicity * (eq36_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq36_e532_d_n0), multiplicity * (eq36_e532_d_n2), multiplicity * (eq36_e532_d_n6), multiplicity * (eq36_e532_d_n7), multiplicity * (eq36_e532_d_n10), multiplicity * (eq36_e532_d_n11), multiplicity * (eq36_e532_d_n12), multiplicity * (eq36_e532_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq37_e540, eq37_e540_d_n0, eq37_e540_d_n2, eq37_e540_d_n4, eq37_e540_d_n6, eq37_e540_d_n7, eq37_e540_d_n10, eq37_e540_d_n11, eq37_e540_d_n12, eq37_e540_d_n17,) = {
    if ((var_guard1227 != 0.0) && (p.p261 != 0.0)) {
        let eq37_e538: f64 = ((nv4 - nv12) / var_rbulk);
        let eq37_e538_d_n0: f64 = (-(((nv4 - nv12) * var_rbulk_dn0) / (var_rbulk * var_rbulk)));
        let eq37_e538_d_n2: f64 = (-(((nv4 - nv12) * var_rbulk_dn2) / (var_rbulk * var_rbulk)));
        let eq37_e538_d_n4: f64 = (1.0 / var_rbulk);
        let eq37_e538_d_n6: f64 = (-(((nv4 - nv12) * var_rbulk_dn6) / (var_rbulk * var_rbulk)));
        let eq37_e538_d_n7: f64 = (-(((nv4 - nv12) * var_rbulk_dn7) / (var_rbulk * var_rbulk)));
        let eq37_e538_d_n10: f64 = (-(((nv4 - nv12) * var_rbulk_dn10) / (var_rbulk * var_rbulk)));
        let eq37_e538_d_n11: f64 = (-(((nv4 - nv12) * var_rbulk_dn11) / (var_rbulk * var_rbulk)));
        let eq37_e538_d_n12: f64 = (((-var_rbulk) - ((nv4 - nv12) * var_rbulk_dn12)) / (var_rbulk * var_rbulk));
        let eq37_e538_d_n17: f64 = (-(((nv4 - nv12) * var_rbulk_dn17) / (var_rbulk * var_rbulk)));
        (eq37_e538, eq37_e538_d_n0, eq37_e538_d_n2, eq37_e538_d_n4, eq37_e538_d_n6, eq37_e538_d_n7, eq37_e538_d_n10, eq37_e538_d_n11, eq37_e538_d_n12, eq37_e538_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq37_value: f64 = eq37_e540;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(4),
            Some(12),
            multiplicity * (eq37_value),
            [0, 2, 4, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq37_e540_d_n0), multiplicity * (eq37_e540_d_n2), multiplicity * (eq37_e540_d_n4), multiplicity * (eq37_e540_d_n6), multiplicity * (eq37_e540_d_n7), multiplicity * (eq37_e540_d_n10), multiplicity * (eq37_e540_d_n11), multiplicity * (eq37_e540_d_n12), multiplicity * (eq37_e540_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq43_e583, eq43_e583_d_n0, eq43_e583_d_n2, eq43_e583_d_n6, eq43_e583_d_n7, eq43_e583_d_n10, eq43_e583_d_n11, eq43_e583_d_n12, eq43_e583_d_n17, eq43_e583_d_n18,) = {
    if ((var_guard1227 != 0.0) && (p.p34 != 0.0)) {
        (var_iqi_nqs, var_iqi_nqs_dn0, var_iqi_nqs_dn2, var_iqi_nqs_dn6, var_iqi_nqs_dn7, var_iqi_nqs_dn10, var_iqi_nqs_dn11, var_iqi_nqs_dn12, var_iqi_nqs_dn17, var_iqi_nqs_dn18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq43_value: f64 = eq43_e583;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(18),
            None,
            multiplicity * (eq43_value),
            [0, 2, 6, 7, 10, 11, 12, 17, 18],
            [multiplicity * (eq43_e583_d_n0), multiplicity * (eq43_e583_d_n2), multiplicity * (eq43_e583_d_n6), multiplicity * (eq43_e583_d_n7), multiplicity * (eq43_e583_d_n10), multiplicity * (eq43_e583_d_n11), multiplicity * (eq43_e583_d_n12), multiplicity * (eq43_e583_d_n17), multiplicity * (eq43_e583_d_n18)],
            [],
            [],
            1.0,
        );
        let (eq44_e589, eq44_e589_d_n0, eq44_e589_d_n2, eq44_e589_d_n6, eq44_e589_d_n7, eq44_e589_d_n10, eq44_e589_d_n11, eq44_e589_d_n12, eq44_e589_d_n13, eq44_e589_d_n15, eq44_e589_d_n16, eq44_e589_d_n17, eq44_e589_d_n18,) = {
    if ((var_guard1227 != 0.0) && (p.p34 != 0.0)) {
        (var_iqb_nqs, var_iqb_nqs_dn0, var_iqb_nqs_dn2, var_iqb_nqs_dn6, var_iqb_nqs_dn7, var_iqb_nqs_dn10, var_iqb_nqs_dn11, var_iqb_nqs_dn12, var_iqb_nqs_dn13, var_iqb_nqs_dn15, var_iqb_nqs_dn16, var_iqb_nqs_dn17, var_iqb_nqs_dn18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq44_value: f64 = eq44_e589;
        let eq44_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq44_node_derivatives: [f64; 12] = [eq44_e589_d_n0, eq44_e589_d_n2, eq44_e589_d_n6, eq44_e589_d_n7, eq44_e589_d_n10, eq44_e589_d_n11, eq44_e589_d_n12, eq44_e589_d_n13, eq44_e589_d_n15, eq44_e589_d_n16, eq44_e589_d_n17, eq44_e589_d_n18];
        let eq44_branch_derivative_indices: [usize; 0] = [];
        let eq44_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(13),
            None,
            multiplicity * (eq44_value),
            &eq44_node_derivative_indices,
            &eq44_node_derivatives,
            &eq44_branch_derivative_indices,
            &eq44_branch_derivatives,
            multiplicity,
        );
        let (eq47_e616, eq47_e616_d_n18,) = {
    if ((var_guard1227 != 0.0) && (p.p34 != 0.0)) {
        let eq47_e611: f64 = (1e-9 / 0.0001);
        let eq47_e613: f64 = (eq47_e611 * (nv18 - 0.0));
        let eq47_e614: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq47_e613);
        (eq47_e614, (eq47_e611 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq47_value: f64 = eq47_e616;
        stamper.stamp_current_node1_local(
            Some(18),
            None,
            multiplicity * (eq47_value),
            18,
            multiplicity * (eq47_e616_d_n18),
        );
        let (eq48_e627, eq48_e627_d_n13,) = {
    if ((var_guard1227 != 0.0) && (p.p34 != 0.0)) {
        let eq48_e622: f64 = (1e-9 / 0.0001);
        let eq48_e624: f64 = (eq48_e622 * (nv13 - 0.0));
        let eq48_e625: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq48_e624);
        (eq48_e625, (eq48_e622 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq48_value: f64 = eq48_e627;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq48_value),
            13,
            multiplicity * (eq48_e627_d_n13),
        );
        let (eq51_e647, eq51_e647_d_n0, eq51_e647_d_n2, eq51_e647_d_n6, eq51_e647_d_n7, eq51_e647_d_n10, eq51_e647_d_n11, eq51_e647_d_n12, eq51_e647_d_n17,) = {
    if ((var_guard1227 != 0.0) && (var_guard1228 != 0.0)) {
        (var_iqh_nqs, var_iqh_nqs_dn0, var_iqh_nqs_dn2, var_iqh_nqs_dn6, var_iqh_nqs_dn7, var_iqh_nqs_dn10, var_iqh_nqs_dn11, var_iqh_nqs_dn12, var_iqh_nqs_dn17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e647;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(17),
            None,
            multiplicity * (eq51_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq51_e647_d_n0), multiplicity * (eq51_e647_d_n2), multiplicity * (eq51_e647_d_n6), multiplicity * (eq51_e647_d_n7), multiplicity * (eq51_e647_d_n10), multiplicity * (eq51_e647_d_n11), multiplicity * (eq51_e647_d_n12), multiplicity * (eq51_e647_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq53_e666, eq53_e666_d_n17,) = {
    if ((var_guard1227 != 0.0) && (var_guard1228 != 0.0)) {
        let eq53_e661: f64 = (1e-9 / 0.0001);
        let eq53_e663: f64 = (eq53_e661 * (nv17 - 0.0));
        let eq53_e664: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, eq53_e663);
        (eq53_e664, (eq53_e661 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq53_value: f64 = eq53_e666;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (eq53_value),
            17,
            multiplicity * (eq53_e666_d_n17),
        );
        let (eq55_e682, eq55_e682_d_n0, eq55_e682_d_n2, eq55_e682_d_n6, eq55_e682_d_n7, eq55_e682_d_n10, eq55_e682_d_n11, eq55_e682_d_n12, eq55_e682_d_n17,) = {
    if (var_guard1227 == 0.0) {
        let eq55_e679: f64 = (var_igidl + var_isub);
        let eq55_e679_d_n0: f64 = (var_igidl_dn0 + var_isub_dn0);
        let eq55_e679_d_n2: f64 = (var_igidl_dn2 + var_isub_dn2);
        let eq55_e679_d_n6: f64 = (var_igidl_dn6 + var_isub_dn6);
        let eq55_e679_d_n7: f64 = (var_igidl_dn7 + var_isub_dn7);
        let eq55_e679_d_n10: f64 = (var_igidl_dn10 + var_isub_dn10);
        let eq55_e679_d_n11: f64 = (var_igidl_dn11 + var_isub_dn11);
        let eq55_e679_d_n12: f64 = (var_igidl_dn12 + var_isub_dn12);
        let eq55_e679_d_n17: f64 = (var_igidl_dn17 + var_isub_dn17);
        let eq55_e680: f64 = (p.p50 * eq55_e679);
        let eq55_e680_d_n0: f64 = (p.p50 * eq55_e679_d_n0);
        let eq55_e680_d_n2: f64 = (p.p50 * eq55_e679_d_n2);
        let eq55_e680_d_n6: f64 = (p.p50 * eq55_e679_d_n6);
        let eq55_e680_d_n7: f64 = (p.p50 * eq55_e679_d_n7);
        let eq55_e680_d_n10: f64 = (p.p50 * eq55_e679_d_n10);
        let eq55_e680_d_n11: f64 = (p.p50 * eq55_e679_d_n11);
        let eq55_e680_d_n12: f64 = (p.p50 * eq55_e679_d_n12);
        let eq55_e680_d_n17: f64 = (p.p50 * eq55_e679_d_n17);
        (eq55_e680, eq55_e680_d_n0, eq55_e680_d_n2, eq55_e680_d_n6, eq55_e680_d_n7, eq55_e680_d_n10, eq55_e680_d_n11, eq55_e680_d_n12, eq55_e680_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e682;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq55_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq55_e682_d_n0), multiplicity * (eq55_e682_d_n2), multiplicity * (eq55_e682_d_n6), multiplicity * (eq55_e682_d_n7), multiplicity * (eq55_e682_d_n10), multiplicity * (eq55_e682_d_n11), multiplicity * (eq55_e682_d_n12), multiplicity * (eq55_e682_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq56_e691, eq56_e691_d_n0, eq56_e691_d_n2, eq56_e691_d_n6, eq56_e691_d_n7, eq56_e691_d_n10, eq56_e691_d_n11, eq56_e691_d_n12, eq56_e691_d_n17,) = {
    if (var_guard1227 == 0.0) {
        let eq56_e688: f64 = (var_igisl + var_isubs);
        let eq56_e688_d_n0: f64 = (var_igisl_dn0 + var_isubs_dn0);
        let eq56_e688_d_n2: f64 = (var_igisl_dn2 + var_isubs_dn2);
        let eq56_e688_d_n6: f64 = (var_igisl_dn6 + var_isubs_dn6);
        let eq56_e688_d_n7: f64 = (var_igisl_dn7 + var_isubs_dn7);
        let eq56_e688_d_n10: f64 = (var_igisl_dn10 + var_isubs_dn10);
        let eq56_e688_d_n11: f64 = (var_igisl_dn11 + var_isubs_dn11);
        let eq56_e688_d_n12: f64 = (var_igisl_dn12 + var_isubs_dn12);
        let eq56_e688_d_n17: f64 = (var_igisl_dn17 + var_isubs_dn17);
        let eq56_e689: f64 = (p.p50 * eq56_e688);
        let eq56_e689_d_n0: f64 = (p.p50 * eq56_e688_d_n0);
        let eq56_e689_d_n2: f64 = (p.p50 * eq56_e688_d_n2);
        let eq56_e689_d_n6: f64 = (p.p50 * eq56_e688_d_n6);
        let eq56_e689_d_n7: f64 = (p.p50 * eq56_e688_d_n7);
        let eq56_e689_d_n10: f64 = (p.p50 * eq56_e688_d_n10);
        let eq56_e689_d_n11: f64 = (p.p50 * eq56_e688_d_n11);
        let eq56_e689_d_n12: f64 = (p.p50 * eq56_e688_d_n12);
        let eq56_e689_d_n17: f64 = (p.p50 * eq56_e688_d_n17);
        (eq56_e689, eq56_e689_d_n0, eq56_e689_d_n2, eq56_e689_d_n6, eq56_e689_d_n7, eq56_e689_d_n10, eq56_e689_d_n11, eq56_e689_d_n12, eq56_e689_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq56_value: f64 = eq56_e691;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(7),
            Some(6),
            multiplicity * (eq56_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq56_e691_d_n0), multiplicity * (eq56_e691_d_n2), multiplicity * (eq56_e691_d_n6), multiplicity * (eq56_e691_d_n7), multiplicity * (eq56_e691_d_n10), multiplicity * (eq56_e691_d_n11), multiplicity * (eq56_e691_d_n12), multiplicity * (eq56_e691_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq58_e703, eq58_e703_d_n0, eq58_e703_d_n2, eq58_e703_d_n6, eq58_e703_d_n7, eq58_e703_d_n10, eq58_e703_d_n11, eq58_e703_d_n12, eq58_e703_d_n17,) = {
    if ((var_guard1227 == 0.0) && (p.p37 != 0.0)) {
        (var_iqh_nqs, var_iqh_nqs_dn0, var_iqh_nqs_dn2, var_iqh_nqs_dn6, var_iqh_nqs_dn7, var_iqh_nqs_dn10, var_iqh_nqs_dn11, var_iqh_nqs_dn12, var_iqh_nqs_dn17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq58_value: f64 = eq58_e703;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(17),
            None,
            multiplicity * (eq58_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq58_e703_d_n0), multiplicity * (eq58_e703_d_n2), multiplicity * (eq58_e703_d_n6), multiplicity * (eq58_e703_d_n7), multiplicity * (eq58_e703_d_n10), multiplicity * (eq58_e703_d_n11), multiplicity * (eq58_e703_d_n12), multiplicity * (eq58_e703_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq60_e724, eq60_e724_d_n17,) = {
    if ((var_guard1227 == 0.0) && (p.p37 != 0.0)) {
        let eq60_e719: f64 = (1e-9 / 0.0001);
        let eq60_e721: f64 = (eq60_e719 * (nv17 - 0.0));
        let eq60_e722: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, eq60_e721);
        (eq60_e722, (eq60_e719 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq60_value: f64 = eq60_e724;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (eq60_value),
            17,
            multiplicity * (eq60_e724_d_n17),
        );
        let (eq62_e739, eq62_e739_d_n0, eq62_e739_d_n2, eq62_e739_d_n6, eq62_e739_d_n7, eq62_e739_d_n10, eq62_e739_d_n11, eq62_e739_d_n12, eq62_e739_d_n13, eq62_e739_d_n15, eq62_e739_d_n16, eq62_e739_d_n17, eq62_e739_d_n18,) = {
    if ((var_guard1227 == 0.0) && (p.p34 != 0.0)) {
        (var_iqd_nqs, var_iqd_nqs_dn0, var_iqd_nqs_dn2, var_iqd_nqs_dn6, var_iqd_nqs_dn7, var_iqd_nqs_dn10, var_iqd_nqs_dn11, var_iqd_nqs_dn12, var_iqd_nqs_dn13, var_iqd_nqs_dn15, var_iqd_nqs_dn16, var_iqd_nqs_dn17, var_iqd_nqs_dn18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_value: f64 = eq62_e739;
        let eq62_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq62_node_derivatives: [f64; 12] = [eq62_e739_d_n0, eq62_e739_d_n2, eq62_e739_d_n6, eq62_e739_d_n7, eq62_e739_d_n10, eq62_e739_d_n11, eq62_e739_d_n12, eq62_e739_d_n13, eq62_e739_d_n15, eq62_e739_d_n16, eq62_e739_d_n17, eq62_e739_d_n18];
        let eq62_branch_derivative_indices: [usize; 0] = [];
        let eq62_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(15),
            None,
            multiplicity * (eq62_value),
            &eq62_node_derivative_indices,
            &eq62_node_derivatives,
            &eq62_branch_derivative_indices,
            &eq62_branch_derivatives,
            multiplicity,
        );
        let (eq63_e746, eq63_e746_d_n0, eq63_e746_d_n2, eq63_e746_d_n6, eq63_e746_d_n7, eq63_e746_d_n10, eq63_e746_d_n11, eq63_e746_d_n12, eq63_e746_d_n13, eq63_e746_d_n15, eq63_e746_d_n16, eq63_e746_d_n17, eq63_e746_d_n18,) = {
    if ((var_guard1227 == 0.0) && (p.p34 != 0.0)) {
        (var_iqs_nqs, var_iqs_nqs_dn0, var_iqs_nqs_dn2, var_iqs_nqs_dn6, var_iqs_nqs_dn7, var_iqs_nqs_dn10, var_iqs_nqs_dn11, var_iqs_nqs_dn12, var_iqs_nqs_dn13, var_iqs_nqs_dn15, var_iqs_nqs_dn16, var_iqs_nqs_dn17, var_iqs_nqs_dn18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq63_value: f64 = eq63_e746;
        let eq63_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq63_node_derivatives: [f64; 12] = [eq63_e746_d_n0, eq63_e746_d_n2, eq63_e746_d_n6, eq63_e746_d_n7, eq63_e746_d_n10, eq63_e746_d_n11, eq63_e746_d_n12, eq63_e746_d_n13, eq63_e746_d_n15, eq63_e746_d_n16, eq63_e746_d_n17, eq63_e746_d_n18];
        let eq63_branch_derivative_indices: [usize; 0] = [];
        let eq63_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(16),
            None,
            multiplicity * (eq63_value),
            &eq63_node_derivative_indices,
            &eq63_node_derivatives,
            &eq63_branch_derivative_indices,
            &eq63_branch_derivatives,
            multiplicity,
        );
        let (eq64_e753, eq64_e753_d_n0, eq64_e753_d_n2, eq64_e753_d_n6, eq64_e753_d_n7, eq64_e753_d_n10, eq64_e753_d_n11, eq64_e753_d_n12, eq64_e753_d_n13, eq64_e753_d_n15, eq64_e753_d_n16, eq64_e753_d_n17, eq64_e753_d_n18,) = {
    if ((var_guard1227 == 0.0) && (p.p34 != 0.0)) {
        (var_iqb_nqs, var_iqb_nqs_dn0, var_iqb_nqs_dn2, var_iqb_nqs_dn6, var_iqb_nqs_dn7, var_iqb_nqs_dn10, var_iqb_nqs_dn11, var_iqb_nqs_dn12, var_iqb_nqs_dn13, var_iqb_nqs_dn15, var_iqb_nqs_dn16, var_iqb_nqs_dn17, var_iqb_nqs_dn18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq64_value: f64 = eq64_e753;
        let eq64_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq64_node_derivatives: [f64; 12] = [eq64_e753_d_n0, eq64_e753_d_n2, eq64_e753_d_n6, eq64_e753_d_n7, eq64_e753_d_n10, eq64_e753_d_n11, eq64_e753_d_n12, eq64_e753_d_n13, eq64_e753_d_n15, eq64_e753_d_n16, eq64_e753_d_n17, eq64_e753_d_n18];
        let eq64_branch_derivative_indices: [usize; 0] = [];
        let eq64_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(13),
            None,
            multiplicity * (eq64_value),
            &eq64_node_derivative_indices,
            &eq64_node_derivatives,
            &eq64_branch_derivative_indices,
            &eq64_branch_derivatives,
            multiplicity,
        );
        let (eq68_e792, eq68_e792_d_n15,) = {
    if ((var_guard1227 == 0.0) && (p.p34 != 0.0)) {
        let eq68_e787: f64 = (1e-9 / 0.0001);
        let eq68_e789: f64 = (eq68_e787 * (nv15 - 0.0));
        let eq68_e790: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, eq68_e789);
        (eq68_e790, (eq68_e787 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq68_value: f64 = eq68_e792;
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (eq68_value),
            15,
            multiplicity * (eq68_e792_d_n15),
        );
        let (eq69_e804, eq69_e804_d_n16,) = {
    if ((var_guard1227 == 0.0) && (p.p34 != 0.0)) {
        let eq69_e799: f64 = (1e-9 / 0.0001);
        let eq69_e801: f64 = (eq69_e799 * (nv16 - 0.0));
        let eq69_e802: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, eq69_e801);
        (eq69_e802, (eq69_e799 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq69_value: f64 = eq69_e804;
        stamper.stamp_current_node1_local(
            Some(16),
            None,
            multiplicity * (eq69_value),
            16,
            multiplicity * (eq69_e804_d_n16),
        );
        let (eq70_e816, eq70_e816_d_n13,) = {
    if ((var_guard1227 == 0.0) && (p.p34 != 0.0)) {
        let eq70_e811: f64 = (1e-9 / 0.0001);
        let eq70_e813: f64 = (eq70_e811 * (nv13 - 0.0));
        let eq70_e814: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, eq70_e813);
        (eq70_e814, (eq70_e811 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq70_value: f64 = eq70_e816;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq70_value),
            13,
            multiplicity * (eq70_e816_d_n13),
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        var_cthe: f64,
        var_guard1226: f64,
        var_guard1227: f64,
        var_guard1228: f64,
        var_ibd: f64,
        var_ibd_dn0: f64,
        var_ibd_dn10: f64,
        var_ibd_dn11: f64,
        var_ibd_dn12: f64,
        var_ibd_dn17: f64,
        var_ibd_dn2: f64,
        var_ibd_dn6: f64,
        var_ibd_dn7: f64,
        var_ibs: f64,
        var_ibs_dn0: f64,
        var_ibs_dn10: f64,
        var_ibs_dn11: f64,
        var_ibs_dn12: f64,
        var_ibs_dn17: f64,
        var_ibs_dn2: f64,
        var_ibs_dn6: f64,
        var_ibs_dn7: f64,
        var_qb: f64,
        var_qb_dn0: f64,
        var_qb_dn10: f64,
        var_qb_dn11: f64,
        var_qb_dn12: f64,
        var_qb_dn13: f64,
        var_qb_dn15: f64,
        var_qb_dn16: f64,
        var_qb_dn17: f64,
        var_qb_dn18: f64,
        var_qb_dn2: f64,
        var_qb_dn6: f64,
        var_qb_dn7: f64,
        var_qbd: f64,
        var_qbd_dn0: f64,
        var_qbd_dn10: f64,
        var_qbd_dn11: f64,
        var_qbd_dn12: f64,
        var_qbd_dn17: f64,
        var_qbd_dn2: f64,
        var_qbd_dn6: f64,
        var_qbd_dn7: f64,
        var_qbs: f64,
        var_qbs_dn0: f64,
        var_qbs_dn10: f64,
        var_qbs_dn11: f64,
        var_qbs_dn12: f64,
        var_qbs_dn17: f64,
        var_qbs_dn2: f64,
        var_qbs_dn6: f64,
        var_qbs_dn7: f64,
        var_qd: f64,
        var_qd_dn0: f64,
        var_qd_dn10: f64,
        var_qd_dn11: f64,
        var_qd_dn12: f64,
        var_qd_dn13: f64,
        var_qd_dn15: f64,
        var_qd_dn16: f64,
        var_qd_dn17: f64,
        var_qd_dn18: f64,
        var_qd_dn2: f64,
        var_qd_dn6: f64,
        var_qd_dn7: f64,
        var_qg: f64,
        var_qg_dn0: f64,
        var_qg_dn10: f64,
        var_qg_dn11: f64,
        var_qg_dn12: f64,
        var_qg_dn13: f64,
        var_qg_dn15: f64,
        var_qg_dn16: f64,
        var_qg_dn17: f64,
        var_qg_dn18: f64,
        var_qg_dn2: f64,
        var_qg_dn6: f64,
        var_qg_dn7: f64,
        var_sigrat_d: f64,
        var_sigrat_d_dn0: f64,
        var_sigrat_d_dn10: f64,
        var_sigrat_d_dn11: f64,
        var_sigrat_d_dn12: f64,
        var_sigrat_d_dn13: f64,
        var_sigrat_d_dn15: f64,
        var_sigrat_d_dn16: f64,
        var_sigrat_d_dn17: f64,
        var_sigrat_d_dn18: f64,
        var_sigrat_d_dn2: f64,
        var_sigrat_d_dn6: f64,
        var_sigrat_d_dn7: f64,
        var_sigrat_s: f64,
        var_sigrat_s_dn0: f64,
        var_sigrat_s_dn10: f64,
        var_sigrat_s_dn11: f64,
        var_sigrat_s_dn12: f64,
        var_sigrat_s_dn13: f64,
        var_sigrat_s_dn15: f64,
        var_sigrat_s_dn16: f64,
        var_sigrat_s_dn17: f64,
        var_sigrat_s_dn18: f64,
        var_sigrat_s_dn2: f64,
        var_sigrat_s_dn6: f64,
        var_sigrat_s_dn7: f64,
    ) {
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let eq11_e367_q: f64 = var_qg;
        let eq11_e368: f64 = (p.p50 * var_qg);
        let eq11_e368_d_n0: f64 = (p.p50 * var_qg_dn0);
        let eq11_e368_d_n2: f64 = (p.p50 * var_qg_dn2);
        let eq11_e368_d_n6: f64 = (p.p50 * var_qg_dn6);
        let eq11_e368_d_n7: f64 = (p.p50 * var_qg_dn7);
        let eq11_e368_d_n10: f64 = (p.p50 * var_qg_dn10);
        let eq11_e368_d_n11: f64 = (p.p50 * var_qg_dn11);
        let eq11_e368_d_n12: f64 = (p.p50 * var_qg_dn12);
        let eq11_e368_d_n13: f64 = (p.p50 * var_qg_dn13);
        let eq11_e368_d_n15: f64 = (p.p50 * var_qg_dn15);
        let eq11_e368_d_n16: f64 = (p.p50 * var_qg_dn16);
        let eq11_e368_d_n17: f64 = (p.p50 * var_qg_dn17);
        let eq11_e368_d_n18: f64 = (p.p50 * var_qg_dn18);
        let eq11_e368_q: f64 = (p.p50 * eq11_e367_q);
        let eq11_reactive_node_derivatives: [f64; 19] = [eq11_e368_d_n0, 0.0, eq11_e368_d_n2, 0.0, 0.0, 0.0, eq11_e368_d_n6, eq11_e368_d_n7, 0.0, 0.0, eq11_e368_d_n10, eq11_e368_d_n11, eq11_e368_d_n12, eq11_e368_d_n13, 0.0, eq11_e368_d_n15, eq11_e368_d_n16, eq11_e368_d_n17, eq11_e368_d_n18];
        let eq11_reactive_branch_derivatives: [f64; 16] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            nodes,
            &eq11_reactive_node_derivatives,
            branches,
            &eq11_reactive_branch_derivatives,
            multiplicity,
        );
        let eq12_e371_q: f64 = var_qd;
        let eq12_e372: f64 = (p.p50 * var_qd);
        let eq12_e372_d_n0: f64 = (p.p50 * var_qd_dn0);
        let eq12_e372_d_n2: f64 = (p.p50 * var_qd_dn2);
        let eq12_e372_d_n6: f64 = (p.p50 * var_qd_dn6);
        let eq12_e372_d_n7: f64 = (p.p50 * var_qd_dn7);
        let eq12_e372_d_n10: f64 = (p.p50 * var_qd_dn10);
        let eq12_e372_d_n11: f64 = (p.p50 * var_qd_dn11);
        let eq12_e372_d_n12: f64 = (p.p50 * var_qd_dn12);
        let eq12_e372_d_n13: f64 = (p.p50 * var_qd_dn13);
        let eq12_e372_d_n15: f64 = (p.p50 * var_qd_dn15);
        let eq12_e372_d_n16: f64 = (p.p50 * var_qd_dn16);
        let eq12_e372_d_n17: f64 = (p.p50 * var_qd_dn17);
        let eq12_e372_d_n18: f64 = (p.p50 * var_qd_dn18);
        let eq12_e372_q: f64 = (p.p50 * eq12_e371_q);
        let eq12_reactive_node_derivatives: [f64; 19] = [eq12_e372_d_n0, 0.0, eq12_e372_d_n2, 0.0, 0.0, 0.0, eq12_e372_d_n6, eq12_e372_d_n7, 0.0, 0.0, eq12_e372_d_n10, eq12_e372_d_n11, eq12_e372_d_n12, eq12_e372_d_n13, 0.0, eq12_e372_d_n15, eq12_e372_d_n16, eq12_e372_d_n17, eq12_e372_d_n18];
        let eq12_reactive_branch_derivatives: [f64; 16] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            nodes,
            &eq12_reactive_node_derivatives,
            branches,
            &eq12_reactive_branch_derivatives,
            multiplicity,
        );
        let eq13_e375_q: f64 = var_qb;
        let eq13_e376: f64 = (p.p50 * var_qb);
        let eq13_e376_d_n0: f64 = (p.p50 * var_qb_dn0);
        let eq13_e376_d_n2: f64 = (p.p50 * var_qb_dn2);
        let eq13_e376_d_n6: f64 = (p.p50 * var_qb_dn6);
        let eq13_e376_d_n7: f64 = (p.p50 * var_qb_dn7);
        let eq13_e376_d_n10: f64 = (p.p50 * var_qb_dn10);
        let eq13_e376_d_n11: f64 = (p.p50 * var_qb_dn11);
        let eq13_e376_d_n12: f64 = (p.p50 * var_qb_dn12);
        let eq13_e376_d_n13: f64 = (p.p50 * var_qb_dn13);
        let eq13_e376_d_n15: f64 = (p.p50 * var_qb_dn15);
        let eq13_e376_d_n16: f64 = (p.p50 * var_qb_dn16);
        let eq13_e376_d_n17: f64 = (p.p50 * var_qb_dn17);
        let eq13_e376_d_n18: f64 = (p.p50 * var_qb_dn18);
        let eq13_e376_q: f64 = (p.p50 * eq13_e375_q);
        let eq13_reactive_node_derivatives: [f64; 19] = [eq13_e376_d_n0, 0.0, eq13_e376_d_n2, 0.0, 0.0, 0.0, eq13_e376_d_n6, eq13_e376_d_n7, 0.0, 0.0, eq13_e376_d_n10, eq13_e376_d_n11, eq13_e376_d_n12, eq13_e376_d_n13, 0.0, eq13_e376_d_n15, eq13_e376_d_n16, eq13_e376_d_n17, eq13_e376_d_n18];
        let eq13_reactive_branch_derivatives: [f64; 16] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            Some(nodes[7]),
            nodes,
            &eq13_reactive_node_derivatives,
            branches,
            &eq13_reactive_branch_derivatives,
            multiplicity,
        );
        let eq19_e405: f64 = ((nv14 - 0.0) * var_sigrat_s);
        let eq19_e405_d_n0: f64 = ((nv14 - 0.0) * var_sigrat_s_dn0);
        let eq19_e405_d_n2: f64 = ((nv14 - 0.0) * var_sigrat_s_dn2);
        let eq19_e405_d_n6: f64 = ((nv14 - 0.0) * var_sigrat_s_dn6);
        let eq19_e405_d_n7: f64 = ((nv14 - 0.0) * var_sigrat_s_dn7);
        let eq19_e405_d_n10: f64 = ((nv14 - 0.0) * var_sigrat_s_dn10);
        let eq19_e405_d_n11: f64 = ((nv14 - 0.0) * var_sigrat_s_dn11);
        let eq19_e405_d_n12: f64 = ((nv14 - 0.0) * var_sigrat_s_dn12);
        let eq19_e405_d_n13: f64 = ((nv14 - 0.0) * var_sigrat_s_dn13);
        let eq19_e405_d_n15: f64 = ((nv14 - 0.0) * var_sigrat_s_dn15);
        let eq19_e405_d_n16: f64 = ((nv14 - 0.0) * var_sigrat_s_dn16);
        let eq19_e405_d_n17: f64 = ((nv14 - 0.0) * var_sigrat_s_dn17);
        let eq19_e405_d_n18: f64 = ((nv14 - 0.0) * var_sigrat_s_dn18);
        let eq19_e406_q: f64 = eq19_e405;
        let eq19_reactive_node_derivatives: [f64; 19] = [eq19_e405_d_n0, 0.0, eq19_e405_d_n2, 0.0, 0.0, 0.0, eq19_e405_d_n6, eq19_e405_d_n7, 0.0, 0.0, eq19_e405_d_n10, eq19_e405_d_n11, eq19_e405_d_n12, eq19_e405_d_n13, var_sigrat_s, eq19_e405_d_n15, eq19_e405_d_n16, eq19_e405_d_n17, eq19_e405_d_n18];
        let eq19_reactive_branch_derivatives: [f64; 16] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            nodes,
            &eq19_reactive_node_derivatives,
            branches,
            &eq19_reactive_branch_derivatives,
            multiplicity,
        );
        let eq20_e409: f64 = ((nv14 - 0.0) * var_sigrat_d);
        let eq20_e409_d_n0: f64 = ((nv14 - 0.0) * var_sigrat_d_dn0);
        let eq20_e409_d_n2: f64 = ((nv14 - 0.0) * var_sigrat_d_dn2);
        let eq20_e409_d_n6: f64 = ((nv14 - 0.0) * var_sigrat_d_dn6);
        let eq20_e409_d_n7: f64 = ((nv14 - 0.0) * var_sigrat_d_dn7);
        let eq20_e409_d_n10: f64 = ((nv14 - 0.0) * var_sigrat_d_dn10);
        let eq20_e409_d_n11: f64 = ((nv14 - 0.0) * var_sigrat_d_dn11);
        let eq20_e409_d_n12: f64 = ((nv14 - 0.0) * var_sigrat_d_dn12);
        let eq20_e409_d_n13: f64 = ((nv14 - 0.0) * var_sigrat_d_dn13);
        let eq20_e409_d_n15: f64 = ((nv14 - 0.0) * var_sigrat_d_dn15);
        let eq20_e409_d_n16: f64 = ((nv14 - 0.0) * var_sigrat_d_dn16);
        let eq20_e409_d_n17: f64 = ((nv14 - 0.0) * var_sigrat_d_dn17);
        let eq20_e409_d_n18: f64 = ((nv14 - 0.0) * var_sigrat_d_dn18);
        let eq20_e410_q: f64 = eq20_e409;
        let eq20_reactive_node_derivatives: [f64; 19] = [eq20_e409_d_n0, 0.0, eq20_e409_d_n2, 0.0, 0.0, 0.0, eq20_e409_d_n6, eq20_e409_d_n7, 0.0, 0.0, eq20_e409_d_n10, eq20_e409_d_n11, eq20_e409_d_n12, eq20_e409_d_n13, var_sigrat_d, eq20_e409_d_n15, eq20_e409_d_n16, eq20_e409_d_n17, eq20_e409_d_n18];
        let eq20_reactive_branch_derivatives: [f64; 16] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[6]),
            nodes,
            &eq20_reactive_node_derivatives,
            branches,
            &eq20_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq31_e491, eq31_e491_d_n10, eq31_e491_q,) = {
    if (var_guard1226 != 0.0) {
        let eq31_e488: f64 = (var_cthe * (nv10 - 0.0));
        let eq31_e489_q: f64 = eq31_e488;
        (eq31_e488, var_cthe, eq31_e489_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[10]),
            None,
            nodes[10],
            multiplicity * (eq31_e491_d_n10),
        );
        let (eq35_e523, eq35_e523_d_n0, eq35_e523_d_n2, eq35_e523_d_n6, eq35_e523_d_n7, eq35_e523_d_n10, eq35_e523_d_n11, eq35_e523_d_n12, eq35_e523_d_n17, eq35_e523_q, eq35_e523_q_d_n0, eq35_e523_q_d_n2, eq35_e523_q_d_n6, eq35_e523_q_d_n7, eq35_e523_q_d_n10, eq35_e523_q_d_n11, eq35_e523_q_d_n12, eq35_e523_q_d_n17,) = {
    if (var_guard1227 != 0.0) {
        let eq35_e519_q: f64 = var_qbs;
        let eq35_e520: f64 = (var_ibs + var_qbs);
        let eq35_e520_d_n0: f64 = (var_ibs_dn0 + var_qbs_dn0);
        let eq35_e520_d_n2: f64 = (var_ibs_dn2 + var_qbs_dn2);
        let eq35_e520_d_n6: f64 = (var_ibs_dn6 + var_qbs_dn6);
        let eq35_e520_d_n7: f64 = (var_ibs_dn7 + var_qbs_dn7);
        let eq35_e520_d_n10: f64 = (var_ibs_dn10 + var_qbs_dn10);
        let eq35_e520_d_n11: f64 = (var_ibs_dn11 + var_qbs_dn11);
        let eq35_e520_d_n12: f64 = (var_ibs_dn12 + var_qbs_dn12);
        let eq35_e520_d_n17: f64 = (var_ibs_dn17 + var_qbs_dn17);
        let eq35_e520_q: f64 = eq35_e519_q;
        let eq35_e521: f64 = (p.p50 * eq35_e520);
        let eq35_e521_d_n0: f64 = (p.p50 * eq35_e520_d_n0);
        let eq35_e521_d_n2: f64 = (p.p50 * eq35_e520_d_n2);
        let eq35_e521_d_n6: f64 = (p.p50 * eq35_e520_d_n6);
        let eq35_e521_d_n7: f64 = (p.p50 * eq35_e520_d_n7);
        let eq35_e521_d_n10: f64 = (p.p50 * eq35_e520_d_n10);
        let eq35_e521_d_n11: f64 = (p.p50 * eq35_e520_d_n11);
        let eq35_e521_d_n12: f64 = (p.p50 * eq35_e520_d_n12);
        let eq35_e521_d_n17: f64 = (p.p50 * eq35_e520_d_n17);
        let eq35_e521_q: f64 = (p.p50 * eq35_e520_q);
        let eq35_e521_q_d_n0: f64 = (p.p50 * var_qbs_dn0);
        let eq35_e521_q_d_n2: f64 = (p.p50 * var_qbs_dn2);
        let eq35_e521_q_d_n6: f64 = (p.p50 * var_qbs_dn6);
        let eq35_e521_q_d_n7: f64 = (p.p50 * var_qbs_dn7);
        let eq35_e521_q_d_n10: f64 = (p.p50 * var_qbs_dn10);
        let eq35_e521_q_d_n11: f64 = (p.p50 * var_qbs_dn11);
        let eq35_e521_q_d_n12: f64 = (p.p50 * var_qbs_dn12);
        let eq35_e521_q_d_n17: f64 = (p.p50 * var_qbs_dn17);
        (eq35_e521, eq35_e521_d_n0, eq35_e521_d_n2, eq35_e521_d_n6, eq35_e521_d_n7, eq35_e521_d_n10, eq35_e521_d_n11, eq35_e521_d_n12, eq35_e521_d_n17, eq35_e521_q, eq35_e521_q_d_n0, eq35_e521_q_d_n2, eq35_e521_q_d_n6, eq35_e521_q_d_n7, eq35_e521_q_d_n10, eq35_e521_q_d_n11, eq35_e521_q_d_n12, eq35_e521_q_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_reactive_node_derivatives: [f64; 19] = [eq35_e523_q_d_n0, 0.0, eq35_e523_q_d_n2, 0.0, 0.0, 0.0, eq35_e523_q_d_n6, eq35_e523_q_d_n7, 0.0, 0.0, eq35_e523_q_d_n10, eq35_e523_q_d_n11, eq35_e523_q_d_n12, 0.0, 0.0, 0.0, 0.0, eq35_e523_q_d_n17, 0.0];
        let eq35_reactive_branch_derivatives: [f64; 16] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq35_reactive_node_derivatives,
            branches,
            &eq35_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq36_e532, eq36_e532_d_n0, eq36_e532_d_n2, eq36_e532_d_n6, eq36_e532_d_n7, eq36_e532_d_n10, eq36_e532_d_n11, eq36_e532_d_n12, eq36_e532_d_n17, eq36_e532_q, eq36_e532_q_d_n0, eq36_e532_q_d_n2, eq36_e532_q_d_n6, eq36_e532_q_d_n7, eq36_e532_q_d_n10, eq36_e532_q_d_n11, eq36_e532_q_d_n12, eq36_e532_q_d_n17,) = {
    if (var_guard1227 != 0.0) {
        let eq36_e528_q: f64 = var_qbd;
        let eq36_e529: f64 = (var_ibd + var_qbd);
        let eq36_e529_d_n0: f64 = (var_ibd_dn0 + var_qbd_dn0);
        let eq36_e529_d_n2: f64 = (var_ibd_dn2 + var_qbd_dn2);
        let eq36_e529_d_n6: f64 = (var_ibd_dn6 + var_qbd_dn6);
        let eq36_e529_d_n7: f64 = (var_ibd_dn7 + var_qbd_dn7);
        let eq36_e529_d_n10: f64 = (var_ibd_dn10 + var_qbd_dn10);
        let eq36_e529_d_n11: f64 = (var_ibd_dn11 + var_qbd_dn11);
        let eq36_e529_d_n12: f64 = (var_ibd_dn12 + var_qbd_dn12);
        let eq36_e529_d_n17: f64 = (var_ibd_dn17 + var_qbd_dn17);
        let eq36_e529_q: f64 = eq36_e528_q;
        let eq36_e530: f64 = (p.p50 * eq36_e529);
        let eq36_e530_d_n0: f64 = (p.p50 * eq36_e529_d_n0);
        let eq36_e530_d_n2: f64 = (p.p50 * eq36_e529_d_n2);
        let eq36_e530_d_n6: f64 = (p.p50 * eq36_e529_d_n6);
        let eq36_e530_d_n7: f64 = (p.p50 * eq36_e529_d_n7);
        let eq36_e530_d_n10: f64 = (p.p50 * eq36_e529_d_n10);
        let eq36_e530_d_n11: f64 = (p.p50 * eq36_e529_d_n11);
        let eq36_e530_d_n12: f64 = (p.p50 * eq36_e529_d_n12);
        let eq36_e530_d_n17: f64 = (p.p50 * eq36_e529_d_n17);
        let eq36_e530_q: f64 = (p.p50 * eq36_e529_q);
        let eq36_e530_q_d_n0: f64 = (p.p50 * var_qbd_dn0);
        let eq36_e530_q_d_n2: f64 = (p.p50 * var_qbd_dn2);
        let eq36_e530_q_d_n6: f64 = (p.p50 * var_qbd_dn6);
        let eq36_e530_q_d_n7: f64 = (p.p50 * var_qbd_dn7);
        let eq36_e530_q_d_n10: f64 = (p.p50 * var_qbd_dn10);
        let eq36_e530_q_d_n11: f64 = (p.p50 * var_qbd_dn11);
        let eq36_e530_q_d_n12: f64 = (p.p50 * var_qbd_dn12);
        let eq36_e530_q_d_n17: f64 = (p.p50 * var_qbd_dn17);
        (eq36_e530, eq36_e530_d_n0, eq36_e530_d_n2, eq36_e530_d_n6, eq36_e530_d_n7, eq36_e530_d_n10, eq36_e530_d_n11, eq36_e530_d_n12, eq36_e530_d_n17, eq36_e530_q, eq36_e530_q_d_n0, eq36_e530_q_d_n2, eq36_e530_q_d_n6, eq36_e530_q_d_n7, eq36_e530_q_d_n10, eq36_e530_q_d_n11, eq36_e530_q_d_n12, eq36_e530_q_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_reactive_node_derivatives: [f64; 19] = [eq36_e532_q_d_n0, 0.0, eq36_e532_q_d_n2, 0.0, 0.0, 0.0, eq36_e532_q_d_n6, eq36_e532_q_d_n7, 0.0, 0.0, eq36_e532_q_d_n10, eq36_e532_q_d_n11, eq36_e532_q_d_n12, 0.0, 0.0, 0.0, 0.0, eq36_e532_q_d_n17, 0.0];
        let eq36_reactive_branch_derivatives: [f64; 16] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq36_reactive_node_derivatives,
            branches,
            &eq36_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq47_e616, eq47_e616_d_n18, eq47_e616_q,) = {
    if ((var_guard1227 != 0.0) && (p.p34 != 0.0)) {
        let eq47_e611: f64 = (1e-9 / 0.0001);
        let eq47_e613: f64 = (eq47_e611 * (nv18 - 0.0));
        let eq47_e614_q: f64 = eq47_e613;
        (eq47_e613, eq47_e611, eq47_e614_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[18]),
            None,
            nodes[18],
            multiplicity * (eq47_e616_d_n18),
        );
        let (eq48_e627, eq48_e627_d_n13, eq48_e627_q,) = {
    if ((var_guard1227 != 0.0) && (p.p34 != 0.0)) {
        let eq48_e622: f64 = (1e-9 / 0.0001);
        let eq48_e624: f64 = (eq48_e622 * (nv13 - 0.0));
        let eq48_e625_q: f64 = eq48_e624;
        (eq48_e624, eq48_e622, eq48_e625_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[13]),
            None,
            nodes[13],
            multiplicity * (eq48_e627_d_n13),
        );
        let (eq53_e666, eq53_e666_d_n17, eq53_e666_q,) = {
    if ((var_guard1227 != 0.0) && (var_guard1228 != 0.0)) {
        let eq53_e661: f64 = (1e-9 / 0.0001);
        let eq53_e663: f64 = (eq53_e661 * (nv17 - 0.0));
        let eq53_e664_q: f64 = eq53_e663;
        (eq53_e663, eq53_e661, eq53_e664_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[17]),
            None,
            nodes[17],
            multiplicity * (eq53_e666_d_n17),
        );
        let (eq60_e724, eq60_e724_d_n17, eq60_e724_q,) = {
    if ((var_guard1227 == 0.0) && (p.p37 != 0.0)) {
        let eq60_e719: f64 = (1e-9 / 0.0001);
        let eq60_e721: f64 = (eq60_e719 * (nv17 - 0.0));
        let eq60_e722_q: f64 = eq60_e721;
        (eq60_e721, eq60_e719, eq60_e722_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[17]),
            None,
            nodes[17],
            multiplicity * (eq60_e724_d_n17),
        );
        let (eq68_e792, eq68_e792_d_n15, eq68_e792_q,) = {
    if ((var_guard1227 == 0.0) && (p.p34 != 0.0)) {
        let eq68_e787: f64 = (1e-9 / 0.0001);
        let eq68_e789: f64 = (eq68_e787 * (nv15 - 0.0));
        let eq68_e790_q: f64 = eq68_e789;
        (eq68_e789, eq68_e787, eq68_e790_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[15]),
            None,
            nodes[15],
            multiplicity * (eq68_e792_d_n15),
        );
        let (eq69_e804, eq69_e804_d_n16, eq69_e804_q,) = {
    if ((var_guard1227 == 0.0) && (p.p34 != 0.0)) {
        let eq69_e799: f64 = (1e-9 / 0.0001);
        let eq69_e801: f64 = (eq69_e799 * (nv16 - 0.0));
        let eq69_e802_q: f64 = eq69_e801;
        (eq69_e801, eq69_e799, eq69_e802_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[16]),
            None,
            nodes[16],
            multiplicity * (eq69_e804_d_n16),
        );
        let (eq70_e816, eq70_e816_d_n13, eq70_e816_q,) = {
    if ((var_guard1227 == 0.0) && (p.p34 != 0.0)) {
        let eq70_e811: f64 = (1e-9 / 0.0001);
        let eq70_e813: f64 = (eq70_e811 * (nv13 - 0.0));
        let eq70_e814_q: f64 = eq70_e813;
        (eq70_e813, eq70_e811, eq70_e814_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[13]),
            None,
            nodes[13],
            multiplicity * (eq70_e816_d_n13),
        );
    }
}
