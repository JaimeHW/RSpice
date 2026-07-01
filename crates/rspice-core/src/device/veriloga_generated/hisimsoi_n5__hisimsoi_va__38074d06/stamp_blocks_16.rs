#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_127(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_guard1151: f64,
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
        var_edri__blk1193_slot: &mut f64,
        var_edri__blk1193_dn0_slot: &mut f64,
        var_edri__blk1193_dn2_slot: &mut f64,
        var_edri__blk1193_dn6_slot: &mut f64,
        var_edri__blk1193_dn7_slot: &mut f64,
        var_edri__blk1193_rv_slot: &mut f64,
        var_guard1175_slot: &mut f64,
        var_guard1175_rv_slot: &mut f64,
        var_guard1176_slot: &mut f64,
        var_guard1176_rv_slot: &mut f64,
        var_guard1179_slot: &mut f64,
        var_guard1179_rv_slot: &mut f64,
        var_guard1199_slot: &mut f64,
        var_guard1199_rv_slot: &mut f64,
        var_ldrifte__blk1189_slot: &mut f64,
        var_ldrifte__blk1189_rv_slot: &mut f64,
        var_mks_rdrmue__blk1183_slot: &mut f64,
        var_mks_rdrmue__blk1183_rv_slot: &mut f64,
        var_mks_rdrvmax__blk1184_slot: &mut f64,
        var_mks_rdrvmax__blk1184_rv_slot: &mut f64,
        var_mu0__blk1191_slot: &mut f64,
        var_mu0__blk1191_dn0_slot: &mut f64,
        var_mu0__blk1191_dn10_slot: &mut f64,
        var_mu0__blk1191_dn11_slot: &mut f64,
        var_mu0__blk1191_dn12_slot: &mut f64,
        var_mu0__blk1191_dn17_slot: &mut f64,
        var_mu0__blk1191_dn2_slot: &mut f64,
        var_mu0__blk1191_dn6_slot: &mut f64,
        var_mu0__blk1191_dn7_slot: &mut f64,
        var_mu0__blk1191_rv_slot: &mut f64,
        var_rdmod_slot: &mut f64,
        var_rdmod_rv_slot: &mut f64,
        var_rdrmuele__blk1180_slot: &mut f64,
        var_rdrmuele__blk1180_rv_slot: &mut f64,
        var_rdrvmaxle__blk1182_slot: &mut f64,
        var_rdrvmaxle__blk1182_rv_slot: &mut f64,
        var_rdrvmaxwe__blk1181_slot: &mut f64,
        var_rdrvmaxwe__blk1181_rv_slot: &mut f64,
        var_rrdrbb__blk1185_slot: &mut f64,
        var_rrdrbb__blk1185_dn10_slot: &mut f64,
        var_rrdrbb__blk1185_rv_slot: &mut f64,
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
        var_tratio__blk1188_slot: &mut f64,
        var_tratio__blk1188_dn10_slot: &mut f64,
        var_tratio__blk1188_rv_slot: &mut f64,
        var_vmaxe__blk1192_slot: &mut f64,
        var_vmaxe__blk1192_dn0_slot: &mut f64,
        var_vmaxe__blk1192_dn10_slot: &mut f64,
        var_vmaxe__blk1192_dn11_slot: &mut f64,
        var_vmaxe__blk1192_dn12_slot: &mut f64,
        var_vmaxe__blk1192_dn17_slot: &mut f64,
        var_vmaxe__blk1192_dn2_slot: &mut f64,
        var_vmaxe__blk1192_dn6_slot: &mut f64,
        var_vmaxe__blk1192_dn7_slot: &mut f64,
        var_vmaxe__blk1192_rv_slot: &mut f64,
        var_vrdr__blk1187_slot: &mut f64,
        var_vrdr__blk1187_dn0_slot: &mut f64,
        var_vrdr__blk1187_dn2_slot: &mut f64,
        var_vrdr__blk1187_dn6_slot: &mut f64,
        var_vrdr__blk1187_dn7_slot: &mut f64,
        var_vrdr__blk1187_rv_slot: &mut f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let mut var_edri__blk1193: f64 = *var_edri__blk1193_slot;
        let mut var_edri__blk1193_dn0: f64 = *var_edri__blk1193_dn0_slot;
        let mut var_edri__blk1193_dn2: f64 = *var_edri__blk1193_dn2_slot;
        let mut var_edri__blk1193_dn6: f64 = *var_edri__blk1193_dn6_slot;
        let mut var_edri__blk1193_dn7: f64 = *var_edri__blk1193_dn7_slot;
        let mut var_edri__blk1193_rv: f64 = *var_edri__blk1193_rv_slot;
        let mut var_guard1175: f64 = *var_guard1175_slot;
        let mut var_guard1175_rv: f64 = *var_guard1175_rv_slot;
        let mut var_guard1176: f64 = *var_guard1176_slot;
        let mut var_guard1176_rv: f64 = *var_guard1176_rv_slot;
        let mut var_guard1179: f64 = *var_guard1179_slot;
        let mut var_guard1179_rv: f64 = *var_guard1179_rv_slot;
        let mut var_guard1199: f64 = *var_guard1199_slot;
        let mut var_guard1199_rv: f64 = *var_guard1199_rv_slot;
        let mut var_ldrifte__blk1189: f64 = *var_ldrifte__blk1189_slot;
        let mut var_ldrifte__blk1189_rv: f64 = *var_ldrifte__blk1189_rv_slot;
        let mut var_mks_rdrmue__blk1183: f64 = *var_mks_rdrmue__blk1183_slot;
        let mut var_mks_rdrmue__blk1183_rv: f64 = *var_mks_rdrmue__blk1183_rv_slot;
        let mut var_mks_rdrvmax__blk1184: f64 = *var_mks_rdrvmax__blk1184_slot;
        let mut var_mks_rdrvmax__blk1184_rv: f64 = *var_mks_rdrvmax__blk1184_rv_slot;
        let mut var_mu0__blk1191: f64 = *var_mu0__blk1191_slot;
        let mut var_mu0__blk1191_dn0: f64 = *var_mu0__blk1191_dn0_slot;
        let mut var_mu0__blk1191_dn10: f64 = *var_mu0__blk1191_dn10_slot;
        let mut var_mu0__blk1191_dn11: f64 = *var_mu0__blk1191_dn11_slot;
        let mut var_mu0__blk1191_dn12: f64 = *var_mu0__blk1191_dn12_slot;
        let mut var_mu0__blk1191_dn17: f64 = *var_mu0__blk1191_dn17_slot;
        let mut var_mu0__blk1191_dn2: f64 = *var_mu0__blk1191_dn2_slot;
        let mut var_mu0__blk1191_dn6: f64 = *var_mu0__blk1191_dn6_slot;
        let mut var_mu0__blk1191_dn7: f64 = *var_mu0__blk1191_dn7_slot;
        let mut var_mu0__blk1191_rv: f64 = *var_mu0__blk1191_rv_slot;
        let mut var_rdmod: f64 = *var_rdmod_slot;
        let mut var_rdmod_rv: f64 = *var_rdmod_rv_slot;
        let mut var_rdrmuele__blk1180: f64 = *var_rdrmuele__blk1180_slot;
        let mut var_rdrmuele__blk1180_rv: f64 = *var_rdrmuele__blk1180_rv_slot;
        let mut var_rdrvmaxle__blk1182: f64 = *var_rdrvmaxle__blk1182_slot;
        let mut var_rdrvmaxle__blk1182_rv: f64 = *var_rdrvmaxle__blk1182_rv_slot;
        let mut var_rdrvmaxwe__blk1181: f64 = *var_rdrvmaxwe__blk1181_slot;
        let mut var_rdrvmaxwe__blk1181_rv: f64 = *var_rdrvmaxwe__blk1181_rv_slot;
        let mut var_rrdrbb__blk1185: f64 = *var_rrdrbb__blk1185_slot;
        let mut var_rrdrbb__blk1185_dn10: f64 = *var_rrdrbb__blk1185_dn10_slot;
        let mut var_rrdrbb__blk1185_rv: f64 = *var_rrdrbb__blk1185_rv_slot;
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
        let mut var_tratio__blk1188: f64 = *var_tratio__blk1188_slot;
        let mut var_tratio__blk1188_dn10: f64 = *var_tratio__blk1188_dn10_slot;
        let mut var_tratio__blk1188_rv: f64 = *var_tratio__blk1188_rv_slot;
        let mut var_vmaxe__blk1192: f64 = *var_vmaxe__blk1192_slot;
        let mut var_vmaxe__blk1192_dn0: f64 = *var_vmaxe__blk1192_dn0_slot;
        let mut var_vmaxe__blk1192_dn10: f64 = *var_vmaxe__blk1192_dn10_slot;
        let mut var_vmaxe__blk1192_dn11: f64 = *var_vmaxe__blk1192_dn11_slot;
        let mut var_vmaxe__blk1192_dn12: f64 = *var_vmaxe__blk1192_dn12_slot;
        let mut var_vmaxe__blk1192_dn17: f64 = *var_vmaxe__blk1192_dn17_slot;
        let mut var_vmaxe__blk1192_dn2: f64 = *var_vmaxe__blk1192_dn2_slot;
        let mut var_vmaxe__blk1192_dn6: f64 = *var_vmaxe__blk1192_dn6_slot;
        let mut var_vmaxe__blk1192_dn7: f64 = *var_vmaxe__blk1192_dn7_slot;
        let mut var_vmaxe__blk1192_rv: f64 = *var_vmaxe__blk1192_rv_slot;
        let mut var_vrdr__blk1187: f64 = *var_vrdr__blk1187_slot;
        let mut var_vrdr__blk1187_dn0: f64 = *var_vrdr__blk1187_dn0_slot;
        let mut var_vrdr__blk1187_dn2: f64 = *var_vrdr__blk1187_dn2_slot;
        let mut var_vrdr__blk1187_dn6: f64 = *var_vrdr__blk1187_dn6_slot;
        let mut var_vrdr__blk1187_dn7: f64 = *var_vrdr__blk1187_dn7_slot;
        let mut var_vrdr__blk1187_rv: f64 = *var_vrdr__blk1187_rv_slot;

        let (assign35490_e50349, assign35490_e50349_d_n0, assign35490_e50349_d_n2, assign35490_e50349_d_n6, assign35490_e50349_d_n7, assign35490_e50349_d_n10, assign35490_e50349_d_n11, assign35490_e50349_d_n12, assign35490_e50349_d_n17,) = {
    if (var_guard1151 != 0.0) {
        let assign35490_e50347: f64 = (var_t1 * var_t3);
        (assign35490_e50347, ((var_t1_dn0 * var_t3) + (var_t1 * var_t3_dn0)), ((var_t1_dn2 * var_t3) + (var_t1 * var_t3_dn2)), ((var_t1_dn6 * var_t3) + (var_t1 * var_t3_dn6)), ((var_t1_dn7 * var_t3) + (var_t1 * var_t3_dn7)), ((var_t1_dn10 * var_t3) + (var_t1 * var_t3_dn10)), ((var_t1_dn11 * var_t3) + (var_t1 * var_t3_dn11)), ((var_t1_dn12 * var_t3) + (var_t1 * var_t3_dn12)), ((var_t1_dn17 * var_t3) + (var_t1 * var_t3_dn17)),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn6, var_t2_dn7, var_t2_dn10, var_t2_dn11, var_t2_dn12, var_t2_dn17,)
    }
};
        var_t2 = assign35490_e50349;
        var_t2_dn0 = assign35490_e50349_d_n0;
        var_t2_dn2 = assign35490_e50349_d_n2;
        var_t2_dn6 = assign35490_e50349_d_n6;
        var_t2_dn7 = assign35490_e50349_d_n7;
        var_t2_dn10 = assign35490_e50349_d_n10;
        var_t2_dn11 = assign35490_e50349_d_n11;
        var_t2_dn12 = assign35490_e50349_d_n12;
        var_t2_dn17 = assign35490_e50349_d_n17;
        var_t2_rv = 0.0;

        let (assign35500_e50355, assign35500_e50355_d_n0, assign35500_e50355_d_n2, assign35500_e50355_d_n6, assign35500_e50355_d_n7, assign35500_e50355_d_n10, assign35500_e50355_d_n11, assign35500_e50355_d_n12, assign35500_e50355_d_n17,) = {
    if (var_guard1151 != 0.0) {
        let assign35500_e50353: f64 = (1.0 + var_t2);
        (assign35500_e50353, var_t2_dn0, var_t2_dn2, var_t2_dn6, var_t2_dn7, var_t2_dn10, var_t2_dn11, var_t2_dn12, var_t2_dn17,)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn6, var_t4_dn7, var_t4_dn10, var_t4_dn11, var_t4_dn12, var_t4_dn17,)
    }
};
        var_t4 = assign35500_e50355;
        var_t4_dn0 = assign35500_e50355_d_n0;
        var_t4_dn2 = assign35500_e50355_d_n2;
        var_t4_dn6 = assign35500_e50355_d_n6;
        var_t4_dn7 = assign35500_e50355_d_n7;
        var_t4_dn10 = assign35500_e50355_d_n10;
        var_t4_dn11 = assign35500_e50355_d_n11;
        var_t4_dn12 = assign35500_e50355_d_n12;
        var_t4_dn17 = assign35500_e50355_d_n17;
        var_t4_rv = 0.0;

        let assign35510_e50359: f64 = (10.0 * 2.220446049250313e-16);
        let assign35510_e50360: f64 = (1.0 - assign35510_e50359);
        let assign35510_e50367: f64 = (10.0 * 2.220446049250313e-16);
        let assign35510_e50368: f64 = (1.0 + assign35510_e50367);
        let assign35510_e50370: f64 = if ((assign35510_e50360 <= var_rrdrbb) && (var_rrdrbb <= assign35510_e50368)) { 1.0 } else { 0.0 };
        var_guard1175 = assign35510_e50370;
        var_guard1175_rv = 0.0;

        let (assign35520_e50378, assign35520_e50378_d_n0, assign35520_e50378_d_n2, assign35520_e50378_d_n6, assign35520_e50378_d_n7, assign35520_e50378_d_n10, assign35520_e50378_d_n11, assign35520_e50378_d_n12, assign35520_e50378_d_n17,) = {
    if ((var_guard1151 != 0.0) && (var_guard1175 != 0.0)) {
        let assign35520_e50376: f64 = (1.0 / var_t4);
        (assign35520_e50376, (-(var_t4_dn0 / (var_t4 * var_t4))), (-(var_t4_dn2 / (var_t4 * var_t4))), (-(var_t4_dn6 / (var_t4 * var_t4))), (-(var_t4_dn7 / (var_t4 * var_t4))), (-(var_t4_dn10 / (var_t4 * var_t4))), (-(var_t4_dn11 / (var_t4 * var_t4))), (-(var_t4_dn12 / (var_t4 * var_t4))), (-(var_t4_dn17 / (var_t4 * var_t4))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn6, var_t5_dn7, var_t5_dn10, var_t5_dn11, var_t5_dn12, var_t5_dn17,)
    }
};
        var_t5 = assign35520_e50378;
        var_t5_dn0 = assign35520_e50378_d_n0;
        var_t5_dn2 = assign35520_e50378_d_n2;
        var_t5_dn6 = assign35520_e50378_d_n6;
        var_t5_dn7 = assign35520_e50378_d_n7;
        var_t5_dn10 = assign35520_e50378_d_n10;
        var_t5_dn11 = assign35520_e50378_d_n11;
        var_t5_dn12 = assign35520_e50378_d_n12;
        var_t5_dn17 = assign35520_e50378_d_n17;
        var_t5_rv = 0.0;

        let assign35530_e50382: f64 = (10.0 * 2.220446049250313e-16);
        let assign35530_e50383: f64 = (2.0 - assign35530_e50382);
        let assign35530_e50390: f64 = (10.0 * 2.220446049250313e-16);
        let assign35530_e50391: f64 = (2.0 + assign35530_e50390);
        let assign35530_e50393: f64 = if ((assign35530_e50383 <= var_rrdrbb) && (var_rrdrbb <= assign35530_e50391)) { 1.0 } else { 0.0 };
        var_guard1176 = assign35530_e50393;
        var_guard1176_rv = 0.0;

        let (assign35540_e50405, assign35540_e50405_d_n0, assign35540_e50405_d_n2, assign35540_e50405_d_n6, assign35540_e50405_d_n7, assign35540_e50405_d_n10, assign35540_e50405_d_n11, assign35540_e50405_d_n12, assign35540_e50405_d_n17,) = {
    if (((var_guard1151 != 0.0) && (var_guard1175 == 0.0)) && (var_guard1176 != 0.0)) {
        let assign35540_e50402: f64 = (var_t4).sqrt();
        let assign35540_e50403: f64 = (1.0 / assign35540_e50402);
        (assign35540_e50403, (-((var_t4_dn0 / (2.0 * assign35540_e50402)) / (assign35540_e50402 * assign35540_e50402))), (-((var_t4_dn2 / (2.0 * assign35540_e50402)) / (assign35540_e50402 * assign35540_e50402))), (-((var_t4_dn6 / (2.0 * assign35540_e50402)) / (assign35540_e50402 * assign35540_e50402))), (-((var_t4_dn7 / (2.0 * assign35540_e50402)) / (assign35540_e50402 * assign35540_e50402))), (-((var_t4_dn10 / (2.0 * assign35540_e50402)) / (assign35540_e50402 * assign35540_e50402))), (-((var_t4_dn11 / (2.0 * assign35540_e50402)) / (assign35540_e50402 * assign35540_e50402))), (-((var_t4_dn12 / (2.0 * assign35540_e50402)) / (assign35540_e50402 * assign35540_e50402))), (-((var_t4_dn17 / (2.0 * assign35540_e50402)) / (assign35540_e50402 * assign35540_e50402))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn6, var_t5_dn7, var_t5_dn10, var_t5_dn11, var_t5_dn12, var_t5_dn17,)
    }
};
        var_t5 = assign35540_e50405;
        var_t5_dn0 = assign35540_e50405_d_n0;
        var_t5_dn2 = assign35540_e50405_d_n2;
        var_t5_dn6 = assign35540_e50405_d_n6;
        var_t5_dn7 = assign35540_e50405_d_n7;
        var_t5_dn10 = assign35540_e50405_d_n10;
        var_t5_dn11 = assign35540_e50405_d_n11;
        var_t5_dn12 = assign35540_e50405_d_n12;
        var_t5_dn17 = assign35540_e50405_d_n17;
        var_t5_rv = 0.0;

        let (assign35550_e50422, assign35550_e50422_d_n0, assign35550_e50422_d_n2, assign35550_e50422_d_n6, assign35550_e50422_d_n7, assign35550_e50422_d_n10, assign35550_e50422_d_n11, assign35550_e50422_d_n12, assign35550_e50422_d_n17,) = {
    if (((var_guard1151 != 0.0) && (var_guard1175 == 0.0)) && (var_guard1176 == 0.0)) {
        let assign35550_e50415: f64 = (-1.0);
        let assign35550_e50417: f64 = (assign35550_e50415 / var_rrdrbb);
        let assign35550_e50419: f64 = (assign35550_e50417 - 1.0);
        let assign35550_e50420: f64 = (var_t4).powf(assign35550_e50419);
        (assign35550_e50420, if 0.0 == 0.0 && ((assign35550_e50419) as f64).is_finite() && ((assign35550_e50419) as f64).fract() == 0.0 { if assign35550_e50419 == 0.0 { 0.0 } else { (assign35550_e50419 * ((var_t4).powf(assign35550_e50419 - 1.0) * var_t4_dn0)) } } else { (assign35550_e50420 * (assign35550_e50419 * (var_t4_dn0 / var_t4))) }, if 0.0 == 0.0 && ((assign35550_e50419) as f64).is_finite() && ((assign35550_e50419) as f64).fract() == 0.0 { if assign35550_e50419 == 0.0 { 0.0 } else { (assign35550_e50419 * ((var_t4).powf(assign35550_e50419 - 1.0) * var_t4_dn2)) } } else { (assign35550_e50420 * (assign35550_e50419 * (var_t4_dn2 / var_t4))) }, if 0.0 == 0.0 && ((assign35550_e50419) as f64).is_finite() && ((assign35550_e50419) as f64).fract() == 0.0 { if assign35550_e50419 == 0.0 { 0.0 } else { (assign35550_e50419 * ((var_t4).powf(assign35550_e50419 - 1.0) * var_t4_dn6)) } } else { (assign35550_e50420 * (assign35550_e50419 * (var_t4_dn6 / var_t4))) }, if 0.0 == 0.0 && ((assign35550_e50419) as f64).is_finite() && ((assign35550_e50419) as f64).fract() == 0.0 { if assign35550_e50419 == 0.0 { 0.0 } else { (assign35550_e50419 * ((var_t4).powf(assign35550_e50419 - 1.0) * var_t4_dn7)) } } else { (assign35550_e50420 * (assign35550_e50419 * (var_t4_dn7 / var_t4))) }, if (-((assign35550_e50415 * var_rrdrbb_dn10) / (var_rrdrbb * var_rrdrbb))) == 0.0 && ((assign35550_e50419) as f64).is_finite() && ((assign35550_e50419) as f64).fract() == 0.0 { if assign35550_e50419 == 0.0 { 0.0 } else { (assign35550_e50419 * ((var_t4).powf(assign35550_e50419 - 1.0) * var_t4_dn10)) } } else { (assign35550_e50420 * (((-((assign35550_e50415 * var_rrdrbb_dn10) / (var_rrdrbb * var_rrdrbb))) * (var_t4).ln()) + (assign35550_e50419 * (var_t4_dn10 / var_t4)))) }, if 0.0 == 0.0 && ((assign35550_e50419) as f64).is_finite() && ((assign35550_e50419) as f64).fract() == 0.0 { if assign35550_e50419 == 0.0 { 0.0 } else { (assign35550_e50419 * ((var_t4).powf(assign35550_e50419 - 1.0) * var_t4_dn11)) } } else { (assign35550_e50420 * (assign35550_e50419 * (var_t4_dn11 / var_t4))) }, if 0.0 == 0.0 && ((assign35550_e50419) as f64).is_finite() && ((assign35550_e50419) as f64).fract() == 0.0 { if assign35550_e50419 == 0.0 { 0.0 } else { (assign35550_e50419 * ((var_t4).powf(assign35550_e50419 - 1.0) * var_t4_dn12)) } } else { (assign35550_e50420 * (assign35550_e50419 * (var_t4_dn12 / var_t4))) }, if 0.0 == 0.0 && ((assign35550_e50419) as f64).is_finite() && ((assign35550_e50419) as f64).fract() == 0.0 { if assign35550_e50419 == 0.0 { 0.0 } else { (assign35550_e50419 * ((var_t4).powf(assign35550_e50419 - 1.0) * var_t4_dn17)) } } else { (assign35550_e50420 * (assign35550_e50419 * (var_t4_dn17 / var_t4))) },)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn6, var_t6_dn7, var_t6_dn10, var_t6_dn11, var_t6_dn12, var_t6_dn17,)
    }
};
        var_t6 = assign35550_e50422;
        var_t6_dn0 = assign35550_e50422_d_n0;
        var_t6_dn2 = assign35550_e50422_d_n2;
        var_t6_dn6 = assign35550_e50422_d_n6;
        var_t6_dn7 = assign35550_e50422_d_n7;
        var_t6_dn10 = assign35550_e50422_d_n10;
        var_t6_dn11 = assign35550_e50422_d_n11;
        var_t6_dn12 = assign35550_e50422_d_n12;
        var_t6_dn17 = assign35550_e50422_d_n17;
        var_t6_rv = 0.0;

        let (assign35560_e50434, assign35560_e50434_d_n0, assign35560_e50434_d_n2, assign35560_e50434_d_n6, assign35560_e50434_d_n7, assign35560_e50434_d_n10, assign35560_e50434_d_n11, assign35560_e50434_d_n12, assign35560_e50434_d_n17,) = {
    if (((var_guard1151 != 0.0) && (var_guard1175 == 0.0)) && (var_guard1176 == 0.0)) {
        let assign35560_e50432: f64 = (var_t4 * var_t6);
        (assign35560_e50432, ((var_t4_dn0 * var_t6) + (var_t4 * var_t6_dn0)), ((var_t4_dn2 * var_t6) + (var_t4 * var_t6_dn2)), ((var_t4_dn6 * var_t6) + (var_t4 * var_t6_dn6)), ((var_t4_dn7 * var_t6) + (var_t4 * var_t6_dn7)), ((var_t4_dn10 * var_t6) + (var_t4 * var_t6_dn10)), ((var_t4_dn11 * var_t6) + (var_t4 * var_t6_dn11)), ((var_t4_dn12 * var_t6) + (var_t4 * var_t6_dn12)), ((var_t4_dn17 * var_t6) + (var_t4 * var_t6_dn17)),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn6, var_t5_dn7, var_t5_dn10, var_t5_dn11, var_t5_dn12, var_t5_dn17,)
    }
};
        var_t5 = assign35560_e50434;
        var_t5_dn0 = assign35560_e50434_d_n0;
        var_t5_dn2 = assign35560_e50434_d_n2;
        var_t5_dn6 = assign35560_e50434_d_n6;
        var_t5_dn7 = assign35560_e50434_d_n7;
        var_t5_dn10 = assign35560_e50434_d_n10;
        var_t5_dn11 = assign35560_e50434_d_n11;
        var_t5_dn12 = assign35560_e50434_d_n12;
        var_t5_dn17 = assign35560_e50434_d_n17;
        var_t5_rv = 0.0;

        let (assign35580_e50446, assign35580_e50446_d_n0, assign35580_e50446_d_n2, assign35580_e50446_d_n6, assign35580_e50446_d_n7, assign35580_e50446_d_n10, assign35580_e50446_d_n11, assign35580_e50446_d_n12, assign35580_e50446_d_n17,) = {
    if (var_guard1151 != 0.0) {
        let assign35580_e50444: f64 = (1.6021918e-19 / var_ldrifte);
        (assign35580_e50444, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign35580_e50446;
        var_t1_dn0 = assign35580_e50446_d_n0;
        var_t1_dn2 = assign35580_e50446_d_n2;
        var_t1_dn6 = assign35580_e50446_d_n6;
        var_t1_dn7 = assign35580_e50446_d_n7;
        var_t1_dn10 = assign35580_e50446_d_n10;
        var_t1_dn11 = assign35580_e50446_d_n11;
        var_t1_dn12 = assign35580_e50446_d_n12;
        var_t1_dn17 = assign35580_e50446_d_n17;
        var_t1_rv = 0.0;

        let assign35700_e50520: f64 = if p.p260 == 1.0 { 1.0 } else { 0.0 };
        var_guard1179 = assign35700_e50520;
        var_guard1179_rv = 0.0;

        let (assign35710_e50524,) = {
    if (var_guard1179 != 0.0) {
        (2.0,)
    } else {
        (var_rdmod,)
    }
};
        var_rdmod = assign35710_e50524;
        var_rdmod_rv = 0.0;

        let assign35720_e50527: f64 = if var_rdmod == 1.0 { 1.0 } else { 0.0 };
        var_guard1199 = assign35720_e50527;
        var_guard1199_rv = 0.0;

        let (assign35740_e50541,) = {
    if ((var_guard1179 != 0.0) && (var_guard1199 != 0.0)) {
        (p.p266,)
    } else {
        (var_mks_rdrmue__blk1183,)
    }
};
        var_mks_rdrmue__blk1183 = assign35740_e50541;
        var_mks_rdrmue__blk1183_rv = 0.0;

        let (assign35750_e50547,) = {
    if ((var_guard1179 != 0.0) && (var_guard1199 != 0.0)) {
        (p.p268,)
    } else {
        (var_mks_rdrvmax__blk1184,)
    }
};
        var_mks_rdrvmax__blk1184 = assign35750_e50547;
        var_mks_rdrvmax__blk1184_rv = 0.0;

        let (assign35760_e50553, assign35760_e50553_d_n10,) = {
    if ((var_guard1179 != 0.0) && (var_guard1199 != 0.0)) {
        (p.p273, 0.0,)
    } else {
        (var_rrdrbb__blk1185, var_rrdrbb__blk1185_dn10,)
    }
};
        var_rrdrbb__blk1185 = assign35760_e50553;
        var_rrdrbb__blk1185_dn10 = assign35760_e50553_d_n10;
        var_rrdrbb__blk1185_rv = 0.0;

        let (assign35780_e50572,) = {
    if ((var_guard1179 != 0.0) && (var_guard1199 != 0.0)) {
        (p.p258,)
    } else {
        (var_ldrifte__blk1189,)
    }
};
        var_ldrifte__blk1189 = assign35780_e50572;
        var_ldrifte__blk1189_rv = 0.0;

        let (assign35790_e50580, assign35790_e50580_d_n0, assign35790_e50580_d_n2, assign35790_e50580_d_n6, assign35790_e50580_d_n7,) = {
    if ((var_guard1179 != 0.0) && (var_guard1199 != 0.0)) {
        let assign35790_e50578: f64 = (p.p50 * (nv7 - nv2));
        (assign35790_e50578, 0.0, (-p.p50), 0.0, p.p50,)
    } else {
        (var_vrdr__blk1187, var_vrdr__blk1187_dn0, var_vrdr__blk1187_dn2, var_vrdr__blk1187_dn6, var_vrdr__blk1187_dn7,)
    }
};
        var_vrdr__blk1187 = assign35790_e50580;
        var_vrdr__blk1187_dn0 = assign35790_e50580_d_n0;
        var_vrdr__blk1187_dn2 = assign35790_e50580_d_n2;
        var_vrdr__blk1187_dn6 = assign35790_e50580_d_n6;
        var_vrdr__blk1187_dn7 = assign35790_e50580_d_n7;
        var_vrdr__blk1187_rv = 0.0;

        let (assign35810_e50596,) = {
    if ((var_guard1179 != 0.0) && (var_guard1199 == 0.0)) {
        (p.p265,)
    } else {
        (var_mks_rdrmue__blk1183,)
    }
};
        var_mks_rdrmue__blk1183 = assign35810_e50596;
        var_mks_rdrmue__blk1183_rv = 0.0;

        let (assign35820_e50603,) = {
    if ((var_guard1179 != 0.0) && (var_guard1199 == 0.0)) {
        (p.p267,)
    } else {
        (var_mks_rdrvmax__blk1184,)
    }
};
        var_mks_rdrvmax__blk1184 = assign35820_e50603;
        var_mks_rdrvmax__blk1184_rv = 0.0;

        let (assign35830_e50610, assign35830_e50610_d_n10,) = {
    if ((var_guard1179 != 0.0) && (var_guard1199 == 0.0)) {
        (p.p272, 0.0,)
    } else {
        (var_rrdrbb__blk1185, var_rrdrbb__blk1185_dn10,)
    }
};
        var_rrdrbb__blk1185 = assign35830_e50610;
        var_rrdrbb__blk1185_dn10 = assign35830_e50610_d_n10;
        var_rrdrbb__blk1185_rv = 0.0;

        let (assign35850_e50631,) = {
    if ((var_guard1179 != 0.0) && (var_guard1199 == 0.0)) {
        (p.p257,)
    } else {
        (var_ldrifte__blk1189,)
    }
};
        var_ldrifte__blk1189 = assign35850_e50631;
        var_ldrifte__blk1189_rv = 0.0;

        let (assign35860_e50640, assign35860_e50640_d_n0, assign35860_e50640_d_n2, assign35860_e50640_d_n6, assign35860_e50640_d_n7,) = {
    if ((var_guard1179 != 0.0) && (var_guard1199 == 0.0)) {
        let assign35860_e50638: f64 = (p.p50 * (nv0 - nv6));
        (assign35860_e50638, p.p50, 0.0, (-p.p50), 0.0,)
    } else {
        (var_vrdr__blk1187, var_vrdr__blk1187_dn0, var_vrdr__blk1187_dn2, var_vrdr__blk1187_dn6, var_vrdr__blk1187_dn7,)
    }
};
        var_vrdr__blk1187 = assign35860_e50640;
        var_vrdr__blk1187_dn0 = assign35860_e50640_d_n0;
        var_vrdr__blk1187_dn2 = assign35860_e50640_d_n2;
        var_vrdr__blk1187_dn6 = assign35860_e50640_d_n6;
        var_vrdr__blk1187_dn7 = assign35860_e50640_d_n7;
        var_vrdr__blk1187_rv = 0.0;

        let (assign35890_e50663,) = {
    if (var_guard1179 != 0.0) {
        let assign35890_e50661: f64 = (var_mks_rdrmue__blk1183 / 10000.0);
        (assign35890_e50661,)
    } else {
        (var_mks_rdrmue__blk1183,)
    }
};
        var_mks_rdrmue__blk1183 = assign35890_e50663;
        var_mks_rdrmue__blk1183_rv = 0.0;

        let (assign35900_e50669,) = {
    if (var_guard1179 != 0.0) {
        let assign35900_e50667: f64 = (var_mks_rdrvmax__blk1184 / 100.0);
        (assign35900_e50667,)
    } else {
        (var_mks_rdrvmax__blk1184,)
    }
};
        var_mks_rdrvmax__blk1184 = assign35900_e50669;
        var_mks_rdrvmax__blk1184_rv = 0.0;

        let (assign35910_e50675, assign35910_e50675_d_n10,) = {
    if (var_guard1179 != 0.0) {
        let assign35910_e50673: f64 = (var_ttemp / var_uc_tnom);
        (assign35910_e50673, (var_ttemp_dn10 / var_uc_tnom),)
    } else {
        (var_tratio__blk1188, var_tratio__blk1188_dn10,)
    }
};
        var_tratio__blk1188 = assign35910_e50675;
        var_tratio__blk1188_dn10 = assign35910_e50675_d_n10;
        var_tratio__blk1188_rv = 0.0;

        let (assign35920_e50681, assign35920_e50681_d_n0, assign35920_e50681_d_n2, assign35920_e50681_d_n6, assign35920_e50681_d_n7, assign35920_e50681_d_n10, assign35920_e50681_d_n11, assign35920_e50681_d_n12, assign35920_e50681_d_n17,) = {
    if (var_guard1179 != 0.0) {
        let assign35920_e50679: f64 = (var_tratio__blk1188).powf(p.p269);
        (assign35920_e50679, 0.0, 0.0, 0.0, 0.0, if 0.0 == 0.0 && ((p.p269) as f64).is_finite() && ((p.p269) as f64).fract() == 0.0 { if p.p269 == 0.0 { 0.0 } else { (p.p269 * ((var_tratio__blk1188).powf(p.p269 - 1.0) * var_tratio__blk1188_dn10)) } } else { (assign35920_e50679 * (p.p269 * (var_tratio__blk1188_dn10 / var_tratio__blk1188))) }, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign35920_e50681;
        var_t1_dn0 = assign35920_e50681_d_n0;
        var_t1_dn2 = assign35920_e50681_d_n2;
        var_t1_dn6 = assign35920_e50681_d_n6;
        var_t1_dn7 = assign35920_e50681_d_n7;
        var_t1_dn10 = assign35920_e50681_d_n10;
        var_t1_dn11 = assign35920_e50681_d_n11;
        var_t1_dn12 = assign35920_e50681_d_n12;
        var_t1_dn17 = assign35920_e50681_d_n17;
        var_t1_rv = 0.0;

        let (assign35930_e50687, assign35930_e50687_d_n0, assign35930_e50687_d_n2, assign35930_e50687_d_n6, assign35930_e50687_d_n7, assign35930_e50687_d_n10, assign35930_e50687_d_n11, assign35930_e50687_d_n12, assign35930_e50687_d_n17,) = {
    if (var_guard1179 != 0.0) {
        let assign35930_e50685: f64 = (var_mks_rdrmue__blk1183 / var_t1);
        (assign35930_e50685, (-((var_mks_rdrmue__blk1183 * var_t1_dn0) / (var_t1 * var_t1))), (-((var_mks_rdrmue__blk1183 * var_t1_dn2) / (var_t1 * var_t1))), (-((var_mks_rdrmue__blk1183 * var_t1_dn6) / (var_t1 * var_t1))), (-((var_mks_rdrmue__blk1183 * var_t1_dn7) / (var_t1 * var_t1))), (-((var_mks_rdrmue__blk1183 * var_t1_dn10) / (var_t1 * var_t1))), (-((var_mks_rdrmue__blk1183 * var_t1_dn11) / (var_t1 * var_t1))), (-((var_mks_rdrmue__blk1183 * var_t1_dn12) / (var_t1 * var_t1))), (-((var_mks_rdrmue__blk1183 * var_t1_dn17) / (var_t1 * var_t1))),)
    } else {
        (var_mu0__blk1191, var_mu0__blk1191_dn0, var_mu0__blk1191_dn2, var_mu0__blk1191_dn6, var_mu0__blk1191_dn7, var_mu0__blk1191_dn10, var_mu0__blk1191_dn11, var_mu0__blk1191_dn12, var_mu0__blk1191_dn17,)
    }
};
        var_mu0__blk1191 = assign35930_e50687;
        var_mu0__blk1191_dn0 = assign35930_e50687_d_n0;
        var_mu0__blk1191_dn2 = assign35930_e50687_d_n2;
        var_mu0__blk1191_dn6 = assign35930_e50687_d_n6;
        var_mu0__blk1191_dn7 = assign35930_e50687_d_n7;
        var_mu0__blk1191_dn10 = assign35930_e50687_d_n10;
        var_mu0__blk1191_dn11 = assign35930_e50687_d_n11;
        var_mu0__blk1191_dn12 = assign35930_e50687_d_n12;
        var_mu0__blk1191_dn17 = assign35930_e50687_d_n17;
        var_mu0__blk1191_rv = 0.0;

        let (assign35940_e50707, assign35940_e50707_d_n0, assign35940_e50707_d_n2, assign35940_e50707_d_n6, assign35940_e50707_d_n7, assign35940_e50707_d_n10, assign35940_e50707_d_n11, assign35940_e50707_d_n12, assign35940_e50707_d_n17,) = {
    if (var_guard1179 != 0.0) {
        let assign35940_e50692: f64 = (0.4 * var_tratio__blk1188);
        let assign35940_e50693: f64 = (1.8 + assign35940_e50692);
        let assign35940_e50696: f64 = (0.1 * var_tratio__blk1188);
        let assign35940_e50698: f64 = (assign35940_e50696 * var_tratio__blk1188);
        let assign35940_e50699: f64 = (assign35940_e50693 + assign35940_e50698);
        let assign35940_e50703: f64 = (1.0 - var_tratio__blk1188);
        let assign35940_e50704: f64 = (p.p270 * assign35940_e50703);
        let assign35940_e50705: f64 = (assign35940_e50699 - assign35940_e50704);
        (assign35940_e50705, 0.0, 0.0, 0.0, 0.0, (((0.4 * var_tratio__blk1188_dn10) + (((0.1 * var_tratio__blk1188_dn10) * var_tratio__blk1188) + (assign35940_e50696 * var_tratio__blk1188_dn10))) - (p.p270 * (-var_tratio__blk1188_dn10))), 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn6, var_t0_dn7, var_t0_dn10, var_t0_dn11, var_t0_dn12, var_t0_dn17,)
    }
};
        var_t0 = assign35940_e50707;
        var_t0_dn0 = assign35940_e50707_d_n0;
        var_t0_dn2 = assign35940_e50707_d_n2;
        var_t0_dn6 = assign35940_e50707_d_n6;
        var_t0_dn7 = assign35940_e50707_d_n7;
        var_t0_dn10 = assign35940_e50707_d_n10;
        var_t0_dn11 = assign35940_e50707_d_n11;
        var_t0_dn12 = assign35940_e50707_d_n12;
        var_t0_dn17 = assign35940_e50707_d_n17;
        var_t0_rv = 0.0;

        let (assign35950_e50713, assign35950_e50713_d_n0, assign35950_e50713_d_n2, assign35950_e50713_d_n6, assign35950_e50713_d_n7, assign35950_e50713_d_n10, assign35950_e50713_d_n11, assign35950_e50713_d_n12, assign35950_e50713_d_n17,) = {
    if (var_guard1179 != 0.0) {
        let assign35950_e50711: f64 = (var_mks_rdrvmax__blk1184 / var_t0);
        (assign35950_e50711, (-((var_mks_rdrvmax__blk1184 * var_t0_dn0) / (var_t0 * var_t0))), (-((var_mks_rdrvmax__blk1184 * var_t0_dn2) / (var_t0 * var_t0))), (-((var_mks_rdrvmax__blk1184 * var_t0_dn6) / (var_t0 * var_t0))), (-((var_mks_rdrvmax__blk1184 * var_t0_dn7) / (var_t0 * var_t0))), (-((var_mks_rdrvmax__blk1184 * var_t0_dn10) / (var_t0 * var_t0))), (-((var_mks_rdrvmax__blk1184 * var_t0_dn11) / (var_t0 * var_t0))), (-((var_mks_rdrvmax__blk1184 * var_t0_dn12) / (var_t0 * var_t0))), (-((var_mks_rdrvmax__blk1184 * var_t0_dn17) / (var_t0 * var_t0))),)
    } else {
        (var_vmaxe__blk1192, var_vmaxe__blk1192_dn0, var_vmaxe__blk1192_dn2, var_vmaxe__blk1192_dn6, var_vmaxe__blk1192_dn7, var_vmaxe__blk1192_dn10, var_vmaxe__blk1192_dn11, var_vmaxe__blk1192_dn12, var_vmaxe__blk1192_dn17,)
    }
};
        var_vmaxe__blk1192 = assign35950_e50713;
        var_vmaxe__blk1192_dn0 = assign35950_e50713_d_n0;
        var_vmaxe__blk1192_dn2 = assign35950_e50713_d_n2;
        var_vmaxe__blk1192_dn6 = assign35950_e50713_d_n6;
        var_vmaxe__blk1192_dn7 = assign35950_e50713_d_n7;
        var_vmaxe__blk1192_dn10 = assign35950_e50713_d_n10;
        var_vmaxe__blk1192_dn11 = assign35950_e50713_d_n11;
        var_vmaxe__blk1192_dn12 = assign35950_e50713_d_n12;
        var_vmaxe__blk1192_dn17 = assign35950_e50713_d_n17;
        var_vmaxe__blk1192_rv = 0.0;

        let (assign35960_e50723, assign35960_e50723_d_n10,) = {
    if (var_guard1179 != 0.0) {
        let assign35960_e50719: f64 = (var_ttemp - var_uc_tnom);
        let assign35960_e50720: f64 = (p.p274 * assign35960_e50719);
        let assign35960_e50721: f64 = (var_rrdrbb__blk1185 + assign35960_e50720);
        (assign35960_e50721, (var_rrdrbb__blk1185_dn10 + (p.p274 * var_ttemp_dn10)),)
    } else {
        (var_rrdrbb__blk1185, var_rrdrbb__blk1185_dn10,)
    }
};
        var_rrdrbb__blk1185 = assign35960_e50723;
        var_rrdrbb__blk1185_dn10 = assign35960_e50723_d_n10;
        var_rrdrbb__blk1185_rv = 0.0;

        let (assign35970_e50733,) = {
    if (var_guard1179 != 0.0) {
        let assign35970_e50729: f64 = (var_lgle).powf(p.p280);
        let assign35970_e50730: f64 = (p.p279 / assign35970_e50729);
        let assign35970_e50731: f64 = (1.0 + assign35970_e50730);
        (assign35970_e50731,)
    } else {
        (var_rdrmuele__blk1180,)
    }
};
        var_rdrmuele__blk1180 = assign35970_e50733;
        var_rdrmuele__blk1180_rv = 0.0;

        let (assign35980_e50743,) = {
    if (var_guard1179 != 0.0) {
        let assign35980_e50739: f64 = (var_lgle).powf(p.p278);
        let assign35980_e50740: f64 = (p.p277 / assign35980_e50739);
        let assign35980_e50741: f64 = (1.0 + assign35980_e50740);
        (assign35980_e50741,)
    } else {
        (var_rdrvmaxle__blk1182,)
    }
};
        var_rdrvmaxle__blk1182 = assign35980_e50743;
        var_rdrvmaxle__blk1182_rv = 0.0;

        let (assign35990_e50753,) = {
    if (var_guard1179 != 0.0) {
        let assign35990_e50749: f64 = (var_wg).powf(p.p276);
        let assign35990_e50750: f64 = (p.p275 / assign35990_e50749);
        let assign35990_e50751: f64 = (1.0 + assign35990_e50750);
        (assign35990_e50751,)
    } else {
        (var_rdrvmaxwe__blk1181,)
    }
};
        var_rdrvmaxwe__blk1181 = assign35990_e50753;
        var_rdrvmaxwe__blk1181_rv = 0.0;

        let (assign36000_e50759, assign36000_e50759_d_n0, assign36000_e50759_d_n2, assign36000_e50759_d_n6, assign36000_e50759_d_n7, assign36000_e50759_d_n10, assign36000_e50759_d_n11, assign36000_e50759_d_n12, assign36000_e50759_d_n17,) = {
    if (var_guard1179 != 0.0) {
        let assign36000_e50757: f64 = (var_mu0__blk1191 * var_rdrmuele__blk1180);
        (assign36000_e50757, (var_mu0__blk1191_dn0 * var_rdrmuele__blk1180), (var_mu0__blk1191_dn2 * var_rdrmuele__blk1180), (var_mu0__blk1191_dn6 * var_rdrmuele__blk1180), (var_mu0__blk1191_dn7 * var_rdrmuele__blk1180), (var_mu0__blk1191_dn10 * var_rdrmuele__blk1180), (var_mu0__blk1191_dn11 * var_rdrmuele__blk1180), (var_mu0__blk1191_dn12 * var_rdrmuele__blk1180), (var_mu0__blk1191_dn17 * var_rdrmuele__blk1180),)
    } else {
        (var_mu0__blk1191, var_mu0__blk1191_dn0, var_mu0__blk1191_dn2, var_mu0__blk1191_dn6, var_mu0__blk1191_dn7, var_mu0__blk1191_dn10, var_mu0__blk1191_dn11, var_mu0__blk1191_dn12, var_mu0__blk1191_dn17,)
    }
};
        var_mu0__blk1191 = assign36000_e50759;
        var_mu0__blk1191_dn0 = assign36000_e50759_d_n0;
        var_mu0__blk1191_dn2 = assign36000_e50759_d_n2;
        var_mu0__blk1191_dn6 = assign36000_e50759_d_n6;
        var_mu0__blk1191_dn7 = assign36000_e50759_d_n7;
        var_mu0__blk1191_dn10 = assign36000_e50759_d_n10;
        var_mu0__blk1191_dn11 = assign36000_e50759_d_n11;
        var_mu0__blk1191_dn12 = assign36000_e50759_d_n12;
        var_mu0__blk1191_dn17 = assign36000_e50759_d_n17;
        var_mu0__blk1191_rv = 0.0;

        let (assign36010_e50769, assign36010_e50769_d_n0, assign36010_e50769_d_n2, assign36010_e50769_d_n6, assign36010_e50769_d_n7, assign36010_e50769_d_n10, assign36010_e50769_d_n11, assign36010_e50769_d_n12, assign36010_e50769_d_n17,) = {
    if (var_guard1179 != 0.0) {
        let assign36010_e50763: f64 = (var_vmaxe__blk1192 * var_rdrvmaxwe__blk1181);
        let assign36010_e50765: f64 = (assign36010_e50763 * var_rdrvmaxle__blk1182);
        let assign36010_e50767: f64 = (assign36010_e50765 + 1e-50);
        (assign36010_e50767, ((var_vmaxe__blk1192_dn0 * var_rdrvmaxwe__blk1181) * var_rdrvmaxle__blk1182), ((var_vmaxe__blk1192_dn2 * var_rdrvmaxwe__blk1181) * var_rdrvmaxle__blk1182), ((var_vmaxe__blk1192_dn6 * var_rdrvmaxwe__blk1181) * var_rdrvmaxle__blk1182), ((var_vmaxe__blk1192_dn7 * var_rdrvmaxwe__blk1181) * var_rdrvmaxle__blk1182), ((var_vmaxe__blk1192_dn10 * var_rdrvmaxwe__blk1181) * var_rdrvmaxle__blk1182), ((var_vmaxe__blk1192_dn11 * var_rdrvmaxwe__blk1181) * var_rdrvmaxle__blk1182), ((var_vmaxe__blk1192_dn12 * var_rdrvmaxwe__blk1181) * var_rdrvmaxle__blk1182), ((var_vmaxe__blk1192_dn17 * var_rdrvmaxwe__blk1181) * var_rdrvmaxle__blk1182),)
    } else {
        (var_vmaxe__blk1192, var_vmaxe__blk1192_dn0, var_vmaxe__blk1192_dn2, var_vmaxe__blk1192_dn6, var_vmaxe__blk1192_dn7, var_vmaxe__blk1192_dn10, var_vmaxe__blk1192_dn11, var_vmaxe__blk1192_dn12, var_vmaxe__blk1192_dn17,)
    }
};
        var_vmaxe__blk1192 = assign36010_e50769;
        var_vmaxe__blk1192_dn0 = assign36010_e50769_d_n0;
        var_vmaxe__blk1192_dn2 = assign36010_e50769_d_n2;
        var_vmaxe__blk1192_dn6 = assign36010_e50769_d_n6;
        var_vmaxe__blk1192_dn7 = assign36010_e50769_d_n7;
        var_vmaxe__blk1192_dn10 = assign36010_e50769_d_n10;
        var_vmaxe__blk1192_dn11 = assign36010_e50769_d_n11;
        var_vmaxe__blk1192_dn12 = assign36010_e50769_d_n12;
        var_vmaxe__blk1192_dn17 = assign36010_e50769_d_n17;
        var_vmaxe__blk1192_rv = 0.0;

        let (assign36020_e50775, assign36020_e50775_d_n0, assign36020_e50775_d_n2, assign36020_e50775_d_n6, assign36020_e50775_d_n7,) = {
    if (var_guard1179 != 0.0) {
        let assign36020_e50773: f64 = (var_vrdr__blk1187 / var_ldrifte__blk1189);
        (assign36020_e50773, (var_vrdr__blk1187_dn0 / var_ldrifte__blk1189), (var_vrdr__blk1187_dn2 / var_ldrifte__blk1189), (var_vrdr__blk1187_dn6 / var_ldrifte__blk1189), (var_vrdr__blk1187_dn7 / var_ldrifte__blk1189),)
    } else {
        (var_edri__blk1193, var_edri__blk1193_dn0, var_edri__blk1193_dn2, var_edri__blk1193_dn6, var_edri__blk1193_dn7,)
    }
};
        var_edri__blk1193 = assign36020_e50775;
        var_edri__blk1193_dn0 = assign36020_e50775_d_n0;
        var_edri__blk1193_dn2 = assign36020_e50775_d_n2;
        var_edri__blk1193_dn6 = assign36020_e50775_d_n6;
        var_edri__blk1193_dn7 = assign36020_e50775_d_n7;
        var_edri__blk1193_rv = 0.0;

        *var_edri__blk1193_slot = var_edri__blk1193;
        *var_edri__blk1193_dn0_slot = var_edri__blk1193_dn0;
        *var_edri__blk1193_dn2_slot = var_edri__blk1193_dn2;
        *var_edri__blk1193_dn6_slot = var_edri__blk1193_dn6;
        *var_edri__blk1193_dn7_slot = var_edri__blk1193_dn7;
        *var_edri__blk1193_rv_slot = var_edri__blk1193_rv;
        *var_guard1175_slot = var_guard1175;
        *var_guard1175_rv_slot = var_guard1175_rv;
        *var_guard1176_slot = var_guard1176;
        *var_guard1176_rv_slot = var_guard1176_rv;
        *var_guard1179_slot = var_guard1179;
        *var_guard1179_rv_slot = var_guard1179_rv;
        *var_guard1199_slot = var_guard1199;
        *var_guard1199_rv_slot = var_guard1199_rv;
        *var_ldrifte__blk1189_slot = var_ldrifte__blk1189;
        *var_ldrifte__blk1189_rv_slot = var_ldrifte__blk1189_rv;
        *var_mks_rdrmue__blk1183_slot = var_mks_rdrmue__blk1183;
        *var_mks_rdrmue__blk1183_rv_slot = var_mks_rdrmue__blk1183_rv;
        *var_mks_rdrvmax__blk1184_slot = var_mks_rdrvmax__blk1184;
        *var_mks_rdrvmax__blk1184_rv_slot = var_mks_rdrvmax__blk1184_rv;
        *var_mu0__blk1191_slot = var_mu0__blk1191;
        *var_mu0__blk1191_dn0_slot = var_mu0__blk1191_dn0;
        *var_mu0__blk1191_dn10_slot = var_mu0__blk1191_dn10;
        *var_mu0__blk1191_dn11_slot = var_mu0__blk1191_dn11;
        *var_mu0__blk1191_dn12_slot = var_mu0__blk1191_dn12;
        *var_mu0__blk1191_dn17_slot = var_mu0__blk1191_dn17;
        *var_mu0__blk1191_dn2_slot = var_mu0__blk1191_dn2;
        *var_mu0__blk1191_dn6_slot = var_mu0__blk1191_dn6;
        *var_mu0__blk1191_dn7_slot = var_mu0__blk1191_dn7;
        *var_mu0__blk1191_rv_slot = var_mu0__blk1191_rv;
        *var_rdmod_slot = var_rdmod;
        *var_rdmod_rv_slot = var_rdmod_rv;
        *var_rdrmuele__blk1180_slot = var_rdrmuele__blk1180;
        *var_rdrmuele__blk1180_rv_slot = var_rdrmuele__blk1180_rv;
        *var_rdrvmaxle__blk1182_slot = var_rdrvmaxle__blk1182;
        *var_rdrvmaxle__blk1182_rv_slot = var_rdrvmaxle__blk1182_rv;
        *var_rdrvmaxwe__blk1181_slot = var_rdrvmaxwe__blk1181;
        *var_rdrvmaxwe__blk1181_rv_slot = var_rdrvmaxwe__blk1181_rv;
        *var_rrdrbb__blk1185_slot = var_rrdrbb__blk1185;
        *var_rrdrbb__blk1185_dn10_slot = var_rrdrbb__blk1185_dn10;
        *var_rrdrbb__blk1185_rv_slot = var_rrdrbb__blk1185_rv;
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
        *var_tratio__blk1188_slot = var_tratio__blk1188;
        *var_tratio__blk1188_dn10_slot = var_tratio__blk1188_dn10;
        *var_tratio__blk1188_rv_slot = var_tratio__blk1188_rv;
        *var_vmaxe__blk1192_slot = var_vmaxe__blk1192;
        *var_vmaxe__blk1192_dn0_slot = var_vmaxe__blk1192_dn0;
        *var_vmaxe__blk1192_dn10_slot = var_vmaxe__blk1192_dn10;
        *var_vmaxe__blk1192_dn11_slot = var_vmaxe__blk1192_dn11;
        *var_vmaxe__blk1192_dn12_slot = var_vmaxe__blk1192_dn12;
        *var_vmaxe__blk1192_dn17_slot = var_vmaxe__blk1192_dn17;
        *var_vmaxe__blk1192_dn2_slot = var_vmaxe__blk1192_dn2;
        *var_vmaxe__blk1192_dn6_slot = var_vmaxe__blk1192_dn6;
        *var_vmaxe__blk1192_dn7_slot = var_vmaxe__blk1192_dn7;
        *var_vmaxe__blk1192_rv_slot = var_vmaxe__blk1192_rv;
        *var_vrdr__blk1187_slot = var_vrdr__blk1187;
        *var_vrdr__blk1187_dn0_slot = var_vrdr__blk1187_dn0;
        *var_vrdr__blk1187_dn2_slot = var_vrdr__blk1187_dn2;
        *var_vrdr__blk1187_dn6_slot = var_vrdr__blk1187_dn6;
        *var_vrdr__blk1187_dn7_slot = var_vrdr__blk1187_dn7;
        *var_vrdr__blk1187_rv_slot = var_vrdr__blk1187_rv;
    }

    pub(super) fn stamp_reactive_block_128(
        p: &Parameters,
        var_edri__blk1193: f64,
        var_edri__blk1193_dn0: f64,
        var_edri__blk1193_dn2: f64,
        var_edri__blk1193_dn6: f64,
        var_edri__blk1193_dn7: f64,
        var_flg_nqs: f64,
        var_guard1179: f64,
        var_ldrifte__blk1189: f64,
        var_mode: f64,
        var_mu0__blk1191: f64,
        var_mu0__blk1191_dn0: f64,
        var_mu0__blk1191_dn10: f64,
        var_mu0__blk1191_dn11: f64,
        var_mu0__blk1191_dn12: f64,
        var_mu0__blk1191_dn17: f64,
        var_mu0__blk1191_dn2: f64,
        var_mu0__blk1191_dn6: f64,
        var_mu0__blk1191_dn7: f64,
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
        var_rrdrbb__blk1185: f64,
        var_rrdrbb__blk1185_dn10: f64,
        var_vmaxe__blk1192: f64,
        var_vmaxe__blk1192_dn0: f64,
        var_vmaxe__blk1192_dn10: f64,
        var_vmaxe__blk1192_dn11: f64,
        var_vmaxe__blk1192_dn12: f64,
        var_vmaxe__blk1192_dn17: f64,
        var_vmaxe__blk1192_dn2: f64,
        var_vmaxe__blk1192_dn6: f64,
        var_vmaxe__blk1192_dn7: f64,
        var_vrdr__blk1187: f64,
        var_xd: f64,
        var_xd_dn0: f64,
        var_xd_dn10: f64,
        var_xd_dn11: f64,
        var_xd_dn12: f64,
        var_xd_dn17: f64,
        var_xd_dn2: f64,
        var_xd_dn6: f64,
        var_xd_dn7: f64,
        var_guard1200_slot: &mut f64,
        var_guard1200_rv_slot: &mut f64,
        var_guard1201_slot: &mut f64,
        var_guard1201_rv_slot: &mut f64,
        var_guard1202_slot: &mut f64,
        var_guard1202_rv_slot: &mut f64,
        var_guard1203_slot: &mut f64,
        var_guard1203_rv_slot: &mut f64,
        var_guard1204_slot: &mut f64,
        var_guard1204_rv_slot: &mut f64,
        var_guard1207_slot: &mut f64,
        var_guard1207_rv_slot: &mut f64,
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
        var_vdri__blk1194_slot: &mut f64,
        var_vdri__blk1194_dn0_slot: &mut f64,
        var_vdri__blk1194_dn10_slot: &mut f64,
        var_vdri__blk1194_dn11_slot: &mut f64,
        var_vdri__blk1194_dn12_slot: &mut f64,
        var_vdri__blk1194_dn17_slot: &mut f64,
        var_vdri__blk1194_dn2_slot: &mut f64,
        var_vdri__blk1194_dn6_slot: &mut f64,
        var_vdri__blk1194_dn7_slot: &mut f64,
        var_vdri__blk1194_rv_slot: &mut f64,
    ) {
        let mut var_guard1200: f64 = *var_guard1200_slot;
        let mut var_guard1200_rv: f64 = *var_guard1200_rv_slot;
        let mut var_guard1201: f64 = *var_guard1201_slot;
        let mut var_guard1201_rv: f64 = *var_guard1201_rv_slot;
        let mut var_guard1202: f64 = *var_guard1202_slot;
        let mut var_guard1202_rv: f64 = *var_guard1202_rv_slot;
        let mut var_guard1203: f64 = *var_guard1203_slot;
        let mut var_guard1203_rv: f64 = *var_guard1203_rv_slot;
        let mut var_guard1204: f64 = *var_guard1204_slot;
        let mut var_guard1204_rv: f64 = *var_guard1204_rv_slot;
        let mut var_guard1207: f64 = *var_guard1207_slot;
        let mut var_guard1207_rv: f64 = *var_guard1207_rv_slot;
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
        let mut var_vdri__blk1194: f64 = *var_vdri__blk1194_slot;
        let mut var_vdri__blk1194_dn0: f64 = *var_vdri__blk1194_dn0_slot;
        let mut var_vdri__blk1194_dn10: f64 = *var_vdri__blk1194_dn10_slot;
        let mut var_vdri__blk1194_dn11: f64 = *var_vdri__blk1194_dn11_slot;
        let mut var_vdri__blk1194_dn12: f64 = *var_vdri__blk1194_dn12_slot;
        let mut var_vdri__blk1194_dn17: f64 = *var_vdri__blk1194_dn17_slot;
        let mut var_vdri__blk1194_dn2: f64 = *var_vdri__blk1194_dn2_slot;
        let mut var_vdri__blk1194_dn6: f64 = *var_vdri__blk1194_dn6_slot;
        let mut var_vdri__blk1194_dn7: f64 = *var_vdri__blk1194_dn7_slot;
        let mut var_vdri__blk1194_rv: f64 = *var_vdri__blk1194_rv_slot;

        let (assign36030_e50781, assign36030_e50781_d_n0, assign36030_e50781_d_n2, assign36030_e50781_d_n6, assign36030_e50781_d_n7, assign36030_e50781_d_n10, assign36030_e50781_d_n11, assign36030_e50781_d_n12, assign36030_e50781_d_n17,) = {
    if (var_guard1179 != 0.0) {
        let assign36030_e50779: f64 = (var_mu0__blk1191 * var_edri__blk1193);
        (assign36030_e50779, ((var_mu0__blk1191_dn0 * var_edri__blk1193) + (var_mu0__blk1191 * var_edri__blk1193_dn0)), ((var_mu0__blk1191_dn2 * var_edri__blk1193) + (var_mu0__blk1191 * var_edri__blk1193_dn2)), ((var_mu0__blk1191_dn6 * var_edri__blk1193) + (var_mu0__blk1191 * var_edri__blk1193_dn6)), ((var_mu0__blk1191_dn7 * var_edri__blk1193) + (var_mu0__blk1191 * var_edri__blk1193_dn7)), (var_mu0__blk1191_dn10 * var_edri__blk1193), (var_mu0__blk1191_dn11 * var_edri__blk1193), (var_mu0__blk1191_dn12 * var_edri__blk1193), (var_mu0__blk1191_dn17 * var_edri__blk1193),)
    } else {
        (var_vdri__blk1194, var_vdri__blk1194_dn0, var_vdri__blk1194_dn2, var_vdri__blk1194_dn6, var_vdri__blk1194_dn7, var_vdri__blk1194_dn10, var_vdri__blk1194_dn11, var_vdri__blk1194_dn12, var_vdri__blk1194_dn17,)
    }
};
        var_vdri__blk1194 = assign36030_e50781;
        var_vdri__blk1194_dn0 = assign36030_e50781_d_n0;
        var_vdri__blk1194_dn2 = assign36030_e50781_d_n2;
        var_vdri__blk1194_dn6 = assign36030_e50781_d_n6;
        var_vdri__blk1194_dn7 = assign36030_e50781_d_n7;
        var_vdri__blk1194_dn10 = assign36030_e50781_d_n10;
        var_vdri__blk1194_dn11 = assign36030_e50781_d_n11;
        var_vdri__blk1194_dn12 = assign36030_e50781_d_n12;
        var_vdri__blk1194_dn17 = assign36030_e50781_d_n17;
        var_vdri__blk1194_rv = 0.0;

        let assign36040_e50784: f64 = if var_vrdr__blk1187 >= 0.0 { 1.0 } else { 0.0 };
        var_guard1200 = assign36040_e50784;
        var_guard1200_rv = 0.0;

        let (assign36050_e50792, assign36050_e50792_d_n0, assign36050_e50792_d_n2, assign36050_e50792_d_n6, assign36050_e50792_d_n7, assign36050_e50792_d_n10, assign36050_e50792_d_n11, assign36050_e50792_d_n12, assign36050_e50792_d_n17,) = {
    if ((var_guard1179 != 0.0) && (var_guard1200 != 0.0)) {
        let assign36050_e50790: f64 = (var_vdri__blk1194 / var_vmaxe__blk1192);
        (assign36050_e50790, (((var_vdri__blk1194_dn0 * var_vmaxe__blk1192) - (var_vdri__blk1194 * var_vmaxe__blk1192_dn0)) / (var_vmaxe__blk1192 * var_vmaxe__blk1192)), (((var_vdri__blk1194_dn2 * var_vmaxe__blk1192) - (var_vdri__blk1194 * var_vmaxe__blk1192_dn2)) / (var_vmaxe__blk1192 * var_vmaxe__blk1192)), (((var_vdri__blk1194_dn6 * var_vmaxe__blk1192) - (var_vdri__blk1194 * var_vmaxe__blk1192_dn6)) / (var_vmaxe__blk1192 * var_vmaxe__blk1192)), (((var_vdri__blk1194_dn7 * var_vmaxe__blk1192) - (var_vdri__blk1194 * var_vmaxe__blk1192_dn7)) / (var_vmaxe__blk1192 * var_vmaxe__blk1192)), (((var_vdri__blk1194_dn10 * var_vmaxe__blk1192) - (var_vdri__blk1194 * var_vmaxe__blk1192_dn10)) / (var_vmaxe__blk1192 * var_vmaxe__blk1192)), (((var_vdri__blk1194_dn11 * var_vmaxe__blk1192) - (var_vdri__blk1194 * var_vmaxe__blk1192_dn11)) / (var_vmaxe__blk1192 * var_vmaxe__blk1192)), (((var_vdri__blk1194_dn12 * var_vmaxe__blk1192) - (var_vdri__blk1194 * var_vmaxe__blk1192_dn12)) / (var_vmaxe__blk1192 * var_vmaxe__blk1192)), (((var_vdri__blk1194_dn17 * var_vmaxe__blk1192) - (var_vdri__blk1194 * var_vmaxe__blk1192_dn17)) / (var_vmaxe__blk1192 * var_vmaxe__blk1192)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign36050_e50792;
        var_t1_dn0 = assign36050_e50792_d_n0;
        var_t1_dn2 = assign36050_e50792_d_n2;
        var_t1_dn6 = assign36050_e50792_d_n6;
        var_t1_dn7 = assign36050_e50792_d_n7;
        var_t1_dn10 = assign36050_e50792_d_n10;
        var_t1_dn11 = assign36050_e50792_d_n11;
        var_t1_dn12 = assign36050_e50792_d_n12;
        var_t1_dn17 = assign36050_e50792_d_n17;
        var_t1_rv = 0.0;

        let (assign36060_e50802, assign36060_e50802_d_n0, assign36060_e50802_d_n2, assign36060_e50802_d_n6, assign36060_e50802_d_n7, assign36060_e50802_d_n10, assign36060_e50802_d_n11, assign36060_e50802_d_n12, assign36060_e50802_d_n17,) = {
    if ((var_guard1179 != 0.0) && (var_guard1200 == 0.0)) {
        let assign36060_e50798: f64 = (-var_vdri__blk1194);
        let assign36060_e50800: f64 = (assign36060_e50798 / var_vmaxe__blk1192);
        (assign36060_e50800, ((((-var_vdri__blk1194_dn0) * var_vmaxe__blk1192) - (assign36060_e50798 * var_vmaxe__blk1192_dn0)) / (var_vmaxe__blk1192 * var_vmaxe__blk1192)), ((((-var_vdri__blk1194_dn2) * var_vmaxe__blk1192) - (assign36060_e50798 * var_vmaxe__blk1192_dn2)) / (var_vmaxe__blk1192 * var_vmaxe__blk1192)), ((((-var_vdri__blk1194_dn6) * var_vmaxe__blk1192) - (assign36060_e50798 * var_vmaxe__blk1192_dn6)) / (var_vmaxe__blk1192 * var_vmaxe__blk1192)), ((((-var_vdri__blk1194_dn7) * var_vmaxe__blk1192) - (assign36060_e50798 * var_vmaxe__blk1192_dn7)) / (var_vmaxe__blk1192 * var_vmaxe__blk1192)), ((((-var_vdri__blk1194_dn10) * var_vmaxe__blk1192) - (assign36060_e50798 * var_vmaxe__blk1192_dn10)) / (var_vmaxe__blk1192 * var_vmaxe__blk1192)), ((((-var_vdri__blk1194_dn11) * var_vmaxe__blk1192) - (assign36060_e50798 * var_vmaxe__blk1192_dn11)) / (var_vmaxe__blk1192 * var_vmaxe__blk1192)), ((((-var_vdri__blk1194_dn12) * var_vmaxe__blk1192) - (assign36060_e50798 * var_vmaxe__blk1192_dn12)) / (var_vmaxe__blk1192 * var_vmaxe__blk1192)), ((((-var_vdri__blk1194_dn17) * var_vmaxe__blk1192) - (assign36060_e50798 * var_vmaxe__blk1192_dn17)) / (var_vmaxe__blk1192 * var_vmaxe__blk1192)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign36060_e50802;
        var_t1_dn0 = assign36060_e50802_d_n0;
        var_t1_dn2 = assign36060_e50802_d_n2;
        var_t1_dn6 = assign36060_e50802_d_n6;
        var_t1_dn7 = assign36060_e50802_d_n7;
        var_t1_dn10 = assign36060_e50802_d_n10;
        var_t1_dn11 = assign36060_e50802_d_n11;
        var_t1_dn12 = assign36060_e50802_d_n12;
        var_t1_dn17 = assign36060_e50802_d_n17;
        var_t1_rv = 0.0;

        let assign36070_e50806: f64 = (10.0 * 2.220446049250313e-16);
        let assign36070_e50807: f64 = (1.0 - assign36070_e50806);
        let assign36070_e50814: f64 = (10.0 * 2.220446049250313e-16);
        let assign36070_e50815: f64 = (1.0 + assign36070_e50814);
        let assign36070_e50817: f64 = if ((assign36070_e50807 <= var_rrdrbb__blk1185) && (var_rrdrbb__blk1185 <= assign36070_e50815)) { 1.0 } else { 0.0 };
        var_guard1201 = assign36070_e50817;
        var_guard1201_rv = 0.0;

        let (assign36080_e50823, assign36080_e50823_d_n0, assign36080_e50823_d_n2, assign36080_e50823_d_n6, assign36080_e50823_d_n7, assign36080_e50823_d_n10, assign36080_e50823_d_n11, assign36080_e50823_d_n12, assign36080_e50823_d_n17,) = {
    if ((var_guard1179 != 0.0) && (var_guard1201 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn6, var_t3_dn7, var_t3_dn10, var_t3_dn11, var_t3_dn12, var_t3_dn17,)
    }
};
        var_t3 = assign36080_e50823;
        var_t3_dn0 = assign36080_e50823_d_n0;
        var_t3_dn2 = assign36080_e50823_d_n2;
        var_t3_dn6 = assign36080_e50823_d_n6;
        var_t3_dn7 = assign36080_e50823_d_n7;
        var_t3_dn10 = assign36080_e50823_d_n10;
        var_t3_dn11 = assign36080_e50823_d_n11;
        var_t3_dn12 = assign36080_e50823_d_n12;
        var_t3_dn17 = assign36080_e50823_d_n17;
        var_t3_rv = 0.0;

        let assign36090_e50827: f64 = (10.0 * 2.220446049250313e-16);
        let assign36090_e50828: f64 = (2.0 - assign36090_e50827);
        let assign36090_e50835: f64 = (10.0 * 2.220446049250313e-16);
        let assign36090_e50836: f64 = (2.0 + assign36090_e50835);
        let assign36090_e50838: f64 = if ((assign36090_e50828 <= var_rrdrbb__blk1185) && (var_rrdrbb__blk1185 <= assign36090_e50836)) { 1.0 } else { 0.0 };
        var_guard1202 = assign36090_e50838;
        var_guard1202_rv = 0.0;

        let (assign36100_e50847, assign36100_e50847_d_n0, assign36100_e50847_d_n2, assign36100_e50847_d_n6, assign36100_e50847_d_n7, assign36100_e50847_d_n10, assign36100_e50847_d_n11, assign36100_e50847_d_n12, assign36100_e50847_d_n17,) = {
    if (((var_guard1179 != 0.0) && (var_guard1201 == 0.0)) && (var_guard1202 != 0.0)) {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn6, var_t3_dn7, var_t3_dn10, var_t3_dn11, var_t3_dn12, var_t3_dn17,)
    }
};
        var_t3 = assign36100_e50847;
        var_t3_dn0 = assign36100_e50847_d_n0;
        var_t3_dn2 = assign36100_e50847_d_n2;
        var_t3_dn6 = assign36100_e50847_d_n6;
        var_t3_dn7 = assign36100_e50847_d_n7;
        var_t3_dn10 = assign36100_e50847_d_n10;
        var_t3_dn11 = assign36100_e50847_d_n11;
        var_t3_dn12 = assign36100_e50847_d_n12;
        var_t3_dn17 = assign36100_e50847_d_n17;
        var_t3_rv = 0.0;

        let (assign36110_e50861, assign36110_e50861_d_n0, assign36110_e50861_d_n2, assign36110_e50861_d_n6, assign36110_e50861_d_n7, assign36110_e50861_d_n10, assign36110_e50861_d_n11, assign36110_e50861_d_n12, assign36110_e50861_d_n17,) = {
    if (((var_guard1179 != 0.0) && (var_guard1201 == 0.0)) && (var_guard1202 == 0.0)) {
        let assign36110_e50858: f64 = (var_rrdrbb__blk1185 - 1.0);
        let assign36110_e50859: f64 = (var_t1).powf(assign36110_e50858);
        (assign36110_e50859, if 0.0 == 0.0 && ((assign36110_e50858) as f64).is_finite() && ((assign36110_e50858) as f64).fract() == 0.0 { if assign36110_e50858 == 0.0 { 0.0 } else { (assign36110_e50858 * ((var_t1).powf(assign36110_e50858 - 1.0) * var_t1_dn0)) } } else { (assign36110_e50859 * (assign36110_e50858 * (var_t1_dn0 / var_t1))) }, if 0.0 == 0.0 && ((assign36110_e50858) as f64).is_finite() && ((assign36110_e50858) as f64).fract() == 0.0 { if assign36110_e50858 == 0.0 { 0.0 } else { (assign36110_e50858 * ((var_t1).powf(assign36110_e50858 - 1.0) * var_t1_dn2)) } } else { (assign36110_e50859 * (assign36110_e50858 * (var_t1_dn2 / var_t1))) }, if 0.0 == 0.0 && ((assign36110_e50858) as f64).is_finite() && ((assign36110_e50858) as f64).fract() == 0.0 { if assign36110_e50858 == 0.0 { 0.0 } else { (assign36110_e50858 * ((var_t1).powf(assign36110_e50858 - 1.0) * var_t1_dn6)) } } else { (assign36110_e50859 * (assign36110_e50858 * (var_t1_dn6 / var_t1))) }, if 0.0 == 0.0 && ((assign36110_e50858) as f64).is_finite() && ((assign36110_e50858) as f64).fract() == 0.0 { if assign36110_e50858 == 0.0 { 0.0 } else { (assign36110_e50858 * ((var_t1).powf(assign36110_e50858 - 1.0) * var_t1_dn7)) } } else { (assign36110_e50859 * (assign36110_e50858 * (var_t1_dn7 / var_t1))) }, if var_rrdrbb__blk1185_dn10 == 0.0 && ((assign36110_e50858) as f64).is_finite() && ((assign36110_e50858) as f64).fract() == 0.0 { if assign36110_e50858 == 0.0 { 0.0 } else { (assign36110_e50858 * ((var_t1).powf(assign36110_e50858 - 1.0) * var_t1_dn10)) } } else { (assign36110_e50859 * ((var_rrdrbb__blk1185_dn10 * (var_t1).ln()) + (assign36110_e50858 * (var_t1_dn10 / var_t1)))) }, if 0.0 == 0.0 && ((assign36110_e50858) as f64).is_finite() && ((assign36110_e50858) as f64).fract() == 0.0 { if assign36110_e50858 == 0.0 { 0.0 } else { (assign36110_e50858 * ((var_t1).powf(assign36110_e50858 - 1.0) * var_t1_dn11)) } } else { (assign36110_e50859 * (assign36110_e50858 * (var_t1_dn11 / var_t1))) }, if 0.0 == 0.0 && ((assign36110_e50858) as f64).is_finite() && ((assign36110_e50858) as f64).fract() == 0.0 { if assign36110_e50858 == 0.0 { 0.0 } else { (assign36110_e50858 * ((var_t1).powf(assign36110_e50858 - 1.0) * var_t1_dn12)) } } else { (assign36110_e50859 * (assign36110_e50858 * (var_t1_dn12 / var_t1))) }, if 0.0 == 0.0 && ((assign36110_e50858) as f64).is_finite() && ((assign36110_e50858) as f64).fract() == 0.0 { if assign36110_e50858 == 0.0 { 0.0 } else { (assign36110_e50858 * ((var_t1).powf(assign36110_e50858 - 1.0) * var_t1_dn17)) } } else { (assign36110_e50859 * (assign36110_e50858 * (var_t1_dn17 / var_t1))) },)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn6, var_t3_dn7, var_t3_dn10, var_t3_dn11, var_t3_dn12, var_t3_dn17,)
    }
};
        var_t3 = assign36110_e50861;
        var_t3_dn0 = assign36110_e50861_d_n0;
        var_t3_dn2 = assign36110_e50861_d_n2;
        var_t3_dn6 = assign36110_e50861_d_n6;
        var_t3_dn7 = assign36110_e50861_d_n7;
        var_t3_dn10 = assign36110_e50861_d_n10;
        var_t3_dn11 = assign36110_e50861_d_n11;
        var_t3_dn12 = assign36110_e50861_d_n12;
        var_t3_dn17 = assign36110_e50861_d_n17;
        var_t3_rv = 0.0;

        let (assign36120_e50867, assign36120_e50867_d_n0, assign36120_e50867_d_n2, assign36120_e50867_d_n6, assign36120_e50867_d_n7, assign36120_e50867_d_n10, assign36120_e50867_d_n11, assign36120_e50867_d_n12, assign36120_e50867_d_n17,) = {
    if (var_guard1179 != 0.0) {
        let assign36120_e50865: f64 = (var_t1 * var_t3);
        (assign36120_e50865, ((var_t1_dn0 * var_t3) + (var_t1 * var_t3_dn0)), ((var_t1_dn2 * var_t3) + (var_t1 * var_t3_dn2)), ((var_t1_dn6 * var_t3) + (var_t1 * var_t3_dn6)), ((var_t1_dn7 * var_t3) + (var_t1 * var_t3_dn7)), ((var_t1_dn10 * var_t3) + (var_t1 * var_t3_dn10)), ((var_t1_dn11 * var_t3) + (var_t1 * var_t3_dn11)), ((var_t1_dn12 * var_t3) + (var_t1 * var_t3_dn12)), ((var_t1_dn17 * var_t3) + (var_t1 * var_t3_dn17)),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn6, var_t2_dn7, var_t2_dn10, var_t2_dn11, var_t2_dn12, var_t2_dn17,)
    }
};
        var_t2 = assign36120_e50867;
        var_t2_dn0 = assign36120_e50867_d_n0;
        var_t2_dn2 = assign36120_e50867_d_n2;
        var_t2_dn6 = assign36120_e50867_d_n6;
        var_t2_dn7 = assign36120_e50867_d_n7;
        var_t2_dn10 = assign36120_e50867_d_n10;
        var_t2_dn11 = assign36120_e50867_d_n11;
        var_t2_dn12 = assign36120_e50867_d_n12;
        var_t2_dn17 = assign36120_e50867_d_n17;
        var_t2_rv = 0.0;

        let (assign36130_e50873, assign36130_e50873_d_n0, assign36130_e50873_d_n2, assign36130_e50873_d_n6, assign36130_e50873_d_n7, assign36130_e50873_d_n10, assign36130_e50873_d_n11, assign36130_e50873_d_n12, assign36130_e50873_d_n17,) = {
    if (var_guard1179 != 0.0) {
        let assign36130_e50871: f64 = (1.0 + var_t2);
        (assign36130_e50871, var_t2_dn0, var_t2_dn2, var_t2_dn6, var_t2_dn7, var_t2_dn10, var_t2_dn11, var_t2_dn12, var_t2_dn17,)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn6, var_t4_dn7, var_t4_dn10, var_t4_dn11, var_t4_dn12, var_t4_dn17,)
    }
};
        var_t4 = assign36130_e50873;
        var_t4_dn0 = assign36130_e50873_d_n0;
        var_t4_dn2 = assign36130_e50873_d_n2;
        var_t4_dn6 = assign36130_e50873_d_n6;
        var_t4_dn7 = assign36130_e50873_d_n7;
        var_t4_dn10 = assign36130_e50873_d_n10;
        var_t4_dn11 = assign36130_e50873_d_n11;
        var_t4_dn12 = assign36130_e50873_d_n12;
        var_t4_dn17 = assign36130_e50873_d_n17;
        var_t4_rv = 0.0;

        let assign36140_e50877: f64 = (10.0 * 2.220446049250313e-16);
        let assign36140_e50878: f64 = (1.0 - assign36140_e50877);
        let assign36140_e50885: f64 = (10.0 * 2.220446049250313e-16);
        let assign36140_e50886: f64 = (1.0 + assign36140_e50885);
        let assign36140_e50888: f64 = if ((assign36140_e50878 <= var_rrdrbb__blk1185) && (var_rrdrbb__blk1185 <= assign36140_e50886)) { 1.0 } else { 0.0 };
        var_guard1203 = assign36140_e50888;
        var_guard1203_rv = 0.0;

        let (assign36150_e50896, assign36150_e50896_d_n0, assign36150_e50896_d_n2, assign36150_e50896_d_n6, assign36150_e50896_d_n7, assign36150_e50896_d_n10, assign36150_e50896_d_n11, assign36150_e50896_d_n12, assign36150_e50896_d_n17,) = {
    if ((var_guard1179 != 0.0) && (var_guard1203 != 0.0)) {
        let assign36150_e50894: f64 = (1.0 / var_t4);
        (assign36150_e50894, (-(var_t4_dn0 / (var_t4 * var_t4))), (-(var_t4_dn2 / (var_t4 * var_t4))), (-(var_t4_dn6 / (var_t4 * var_t4))), (-(var_t4_dn7 / (var_t4 * var_t4))), (-(var_t4_dn10 / (var_t4 * var_t4))), (-(var_t4_dn11 / (var_t4 * var_t4))), (-(var_t4_dn12 / (var_t4 * var_t4))), (-(var_t4_dn17 / (var_t4 * var_t4))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn6, var_t5_dn7, var_t5_dn10, var_t5_dn11, var_t5_dn12, var_t5_dn17,)
    }
};
        var_t5 = assign36150_e50896;
        var_t5_dn0 = assign36150_e50896_d_n0;
        var_t5_dn2 = assign36150_e50896_d_n2;
        var_t5_dn6 = assign36150_e50896_d_n6;
        var_t5_dn7 = assign36150_e50896_d_n7;
        var_t5_dn10 = assign36150_e50896_d_n10;
        var_t5_dn11 = assign36150_e50896_d_n11;
        var_t5_dn12 = assign36150_e50896_d_n12;
        var_t5_dn17 = assign36150_e50896_d_n17;
        var_t5_rv = 0.0;

        let assign36160_e50900: f64 = (10.0 * 2.220446049250313e-16);
        let assign36160_e50901: f64 = (2.0 - assign36160_e50900);
        let assign36160_e50908: f64 = (10.0 * 2.220446049250313e-16);
        let assign36160_e50909: f64 = (2.0 + assign36160_e50908);
        let assign36160_e50911: f64 = if ((assign36160_e50901 <= var_rrdrbb__blk1185) && (var_rrdrbb__blk1185 <= assign36160_e50909)) { 1.0 } else { 0.0 };
        var_guard1204 = assign36160_e50911;
        var_guard1204_rv = 0.0;

        let (assign36170_e50923, assign36170_e50923_d_n0, assign36170_e50923_d_n2, assign36170_e50923_d_n6, assign36170_e50923_d_n7, assign36170_e50923_d_n10, assign36170_e50923_d_n11, assign36170_e50923_d_n12, assign36170_e50923_d_n17,) = {
    if (((var_guard1179 != 0.0) && (var_guard1203 == 0.0)) && (var_guard1204 != 0.0)) {
        let assign36170_e50920: f64 = (var_t4).sqrt();
        let assign36170_e50921: f64 = (1.0 / assign36170_e50920);
        (assign36170_e50921, (-((var_t4_dn0 / (2.0 * assign36170_e50920)) / (assign36170_e50920 * assign36170_e50920))), (-((var_t4_dn2 / (2.0 * assign36170_e50920)) / (assign36170_e50920 * assign36170_e50920))), (-((var_t4_dn6 / (2.0 * assign36170_e50920)) / (assign36170_e50920 * assign36170_e50920))), (-((var_t4_dn7 / (2.0 * assign36170_e50920)) / (assign36170_e50920 * assign36170_e50920))), (-((var_t4_dn10 / (2.0 * assign36170_e50920)) / (assign36170_e50920 * assign36170_e50920))), (-((var_t4_dn11 / (2.0 * assign36170_e50920)) / (assign36170_e50920 * assign36170_e50920))), (-((var_t4_dn12 / (2.0 * assign36170_e50920)) / (assign36170_e50920 * assign36170_e50920))), (-((var_t4_dn17 / (2.0 * assign36170_e50920)) / (assign36170_e50920 * assign36170_e50920))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn6, var_t5_dn7, var_t5_dn10, var_t5_dn11, var_t5_dn12, var_t5_dn17,)
    }
};
        var_t5 = assign36170_e50923;
        var_t5_dn0 = assign36170_e50923_d_n0;
        var_t5_dn2 = assign36170_e50923_d_n2;
        var_t5_dn6 = assign36170_e50923_d_n6;
        var_t5_dn7 = assign36170_e50923_d_n7;
        var_t5_dn10 = assign36170_e50923_d_n10;
        var_t5_dn11 = assign36170_e50923_d_n11;
        var_t5_dn12 = assign36170_e50923_d_n12;
        var_t5_dn17 = assign36170_e50923_d_n17;
        var_t5_rv = 0.0;

        let (assign36180_e50940, assign36180_e50940_d_n0, assign36180_e50940_d_n2, assign36180_e50940_d_n6, assign36180_e50940_d_n7, assign36180_e50940_d_n10, assign36180_e50940_d_n11, assign36180_e50940_d_n12, assign36180_e50940_d_n17,) = {
    if (((var_guard1179 != 0.0) && (var_guard1203 == 0.0)) && (var_guard1204 == 0.0)) {
        let assign36180_e50933: f64 = (-1.0);
        let assign36180_e50935: f64 = (assign36180_e50933 / var_rrdrbb__blk1185);
        let assign36180_e50937: f64 = (assign36180_e50935 - 1.0);
        let assign36180_e50938: f64 = (var_t4).powf(assign36180_e50937);
        (assign36180_e50938, if 0.0 == 0.0 && ((assign36180_e50937) as f64).is_finite() && ((assign36180_e50937) as f64).fract() == 0.0 { if assign36180_e50937 == 0.0 { 0.0 } else { (assign36180_e50937 * ((var_t4).powf(assign36180_e50937 - 1.0) * var_t4_dn0)) } } else { (assign36180_e50938 * (assign36180_e50937 * (var_t4_dn0 / var_t4))) }, if 0.0 == 0.0 && ((assign36180_e50937) as f64).is_finite() && ((assign36180_e50937) as f64).fract() == 0.0 { if assign36180_e50937 == 0.0 { 0.0 } else { (assign36180_e50937 * ((var_t4).powf(assign36180_e50937 - 1.0) * var_t4_dn2)) } } else { (assign36180_e50938 * (assign36180_e50937 * (var_t4_dn2 / var_t4))) }, if 0.0 == 0.0 && ((assign36180_e50937) as f64).is_finite() && ((assign36180_e50937) as f64).fract() == 0.0 { if assign36180_e50937 == 0.0 { 0.0 } else { (assign36180_e50937 * ((var_t4).powf(assign36180_e50937 - 1.0) * var_t4_dn6)) } } else { (assign36180_e50938 * (assign36180_e50937 * (var_t4_dn6 / var_t4))) }, if 0.0 == 0.0 && ((assign36180_e50937) as f64).is_finite() && ((assign36180_e50937) as f64).fract() == 0.0 { if assign36180_e50937 == 0.0 { 0.0 } else { (assign36180_e50937 * ((var_t4).powf(assign36180_e50937 - 1.0) * var_t4_dn7)) } } else { (assign36180_e50938 * (assign36180_e50937 * (var_t4_dn7 / var_t4))) }, if (-((assign36180_e50933 * var_rrdrbb__blk1185_dn10) / (var_rrdrbb__blk1185 * var_rrdrbb__blk1185))) == 0.0 && ((assign36180_e50937) as f64).is_finite() && ((assign36180_e50937) as f64).fract() == 0.0 { if assign36180_e50937 == 0.0 { 0.0 } else { (assign36180_e50937 * ((var_t4).powf(assign36180_e50937 - 1.0) * var_t4_dn10)) } } else { (assign36180_e50938 * (((-((assign36180_e50933 * var_rrdrbb__blk1185_dn10) / (var_rrdrbb__blk1185 * var_rrdrbb__blk1185))) * (var_t4).ln()) + (assign36180_e50937 * (var_t4_dn10 / var_t4)))) }, if 0.0 == 0.0 && ((assign36180_e50937) as f64).is_finite() && ((assign36180_e50937) as f64).fract() == 0.0 { if assign36180_e50937 == 0.0 { 0.0 } else { (assign36180_e50937 * ((var_t4).powf(assign36180_e50937 - 1.0) * var_t4_dn11)) } } else { (assign36180_e50938 * (assign36180_e50937 * (var_t4_dn11 / var_t4))) }, if 0.0 == 0.0 && ((assign36180_e50937) as f64).is_finite() && ((assign36180_e50937) as f64).fract() == 0.0 { if assign36180_e50937 == 0.0 { 0.0 } else { (assign36180_e50937 * ((var_t4).powf(assign36180_e50937 - 1.0) * var_t4_dn12)) } } else { (assign36180_e50938 * (assign36180_e50937 * (var_t4_dn12 / var_t4))) }, if 0.0 == 0.0 && ((assign36180_e50937) as f64).is_finite() && ((assign36180_e50937) as f64).fract() == 0.0 { if assign36180_e50937 == 0.0 { 0.0 } else { (assign36180_e50937 * ((var_t4).powf(assign36180_e50937 - 1.0) * var_t4_dn17)) } } else { (assign36180_e50938 * (assign36180_e50937 * (var_t4_dn17 / var_t4))) },)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn6, var_t6_dn7, var_t6_dn10, var_t6_dn11, var_t6_dn12, var_t6_dn17,)
    }
};
        var_t6 = assign36180_e50940;
        var_t6_dn0 = assign36180_e50940_d_n0;
        var_t6_dn2 = assign36180_e50940_d_n2;
        var_t6_dn6 = assign36180_e50940_d_n6;
        var_t6_dn7 = assign36180_e50940_d_n7;
        var_t6_dn10 = assign36180_e50940_d_n10;
        var_t6_dn11 = assign36180_e50940_d_n11;
        var_t6_dn12 = assign36180_e50940_d_n12;
        var_t6_dn17 = assign36180_e50940_d_n17;
        var_t6_rv = 0.0;

        let (assign36190_e50952, assign36190_e50952_d_n0, assign36190_e50952_d_n2, assign36190_e50952_d_n6, assign36190_e50952_d_n7, assign36190_e50952_d_n10, assign36190_e50952_d_n11, assign36190_e50952_d_n12, assign36190_e50952_d_n17,) = {
    if (((var_guard1179 != 0.0) && (var_guard1203 == 0.0)) && (var_guard1204 == 0.0)) {
        let assign36190_e50950: f64 = (var_t4 * var_t6);
        (assign36190_e50950, ((var_t4_dn0 * var_t6) + (var_t4 * var_t6_dn0)), ((var_t4_dn2 * var_t6) + (var_t4 * var_t6_dn2)), ((var_t4_dn6 * var_t6) + (var_t4 * var_t6_dn6)), ((var_t4_dn7 * var_t6) + (var_t4 * var_t6_dn7)), ((var_t4_dn10 * var_t6) + (var_t4 * var_t6_dn10)), ((var_t4_dn11 * var_t6) + (var_t4 * var_t6_dn11)), ((var_t4_dn12 * var_t6) + (var_t4 * var_t6_dn12)), ((var_t4_dn17 * var_t6) + (var_t4 * var_t6_dn17)),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn6, var_t5_dn7, var_t5_dn10, var_t5_dn11, var_t5_dn12, var_t5_dn17,)
    }
};
        var_t5 = assign36190_e50952;
        var_t5_dn0 = assign36190_e50952_d_n0;
        var_t5_dn2 = assign36190_e50952_d_n2;
        var_t5_dn6 = assign36190_e50952_d_n6;
        var_t5_dn7 = assign36190_e50952_d_n7;
        var_t5_dn10 = assign36190_e50952_d_n10;
        var_t5_dn11 = assign36190_e50952_d_n11;
        var_t5_dn12 = assign36190_e50952_d_n12;
        var_t5_dn17 = assign36190_e50952_d_n17;
        var_t5_rv = 0.0;

        let (assign36210_e50964, assign36210_e50964_d_n0, assign36210_e50964_d_n2, assign36210_e50964_d_n6, assign36210_e50964_d_n7, assign36210_e50964_d_n10, assign36210_e50964_d_n11, assign36210_e50964_d_n12, assign36210_e50964_d_n17,) = {
    if (var_guard1179 != 0.0) {
        let assign36210_e50962: f64 = (1.6021918e-19 / var_ldrifte__blk1189);
        (assign36210_e50962, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn6, var_t1_dn7, var_t1_dn10, var_t1_dn11, var_t1_dn12, var_t1_dn17,)
    }
};
        var_t1 = assign36210_e50964;
        var_t1_dn0 = assign36210_e50964_d_n0;
        var_t1_dn2 = assign36210_e50964_d_n2;
        var_t1_dn6 = assign36210_e50964_d_n6;
        var_t1_dn7 = assign36210_e50964_d_n7;
        var_t1_dn10 = assign36210_e50964_d_n10;
        var_t1_dn11 = assign36210_e50964_d_n11;
        var_t1_dn12 = assign36210_e50964_d_n12;
        var_t1_dn17 = assign36210_e50964_d_n17;
        var_t1_rv = 0.0;

        let assign36330_e51038: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard1207 = assign36330_e51038;
        var_guard1207_rv = 0.0;

        let (assign36380_e51081, assign36380_e51081_d_n0, assign36380_e51081_d_n2, assign36380_e51081_d_n6, assign36380_e51081_d_n7, assign36380_e51081_d_n10, assign36380_e51081_d_n11, assign36380_e51081_d_n12, assign36380_e51081_d_n17,) = {
    if ((var_guard1207 != 0.0) && (var_flg_nqs != 0.0)) {
        let (assign36380_e51079, assign36380_e51079_d_n0, assign36380_e51079_d_n2, assign36380_e51079_d_n6, assign36380_e51079_d_n7, assign36380_e51079_d_n10, assign36380_e51079_d_n11, assign36380_e51079_d_n12, assign36380_e51079_d_n17,) = {
            if (var_mode == 1.0) {
                (var_xd, var_xd_dn0, var_xd_dn2, var_xd_dn6, var_xd_dn7, var_xd_dn10, var_xd_dn11, var_xd_dn12, var_xd_dn17,)
            } else {
                let assign36380_e51078: f64 = (1.0 - var_xd);
                (assign36380_e51078, (-var_xd_dn0), (-var_xd_dn2), (-var_xd_dn6), (-var_xd_dn7), (-var_xd_dn10), (-var_xd_dn11), (-var_xd_dn12), (-var_xd_dn17),)
            }
        };
        (assign36380_e51079, assign36380_e51079_d_n0, assign36380_e51079_d_n2, assign36380_e51079_d_n6, assign36380_e51079_d_n7, assign36380_e51079_d_n10, assign36380_e51079_d_n11, assign36380_e51079_d_n12, assign36380_e51079_d_n17,)
    } else {
        (var_qdrat, var_qdrat_dn0, var_qdrat_dn2, var_qdrat_dn6, var_qdrat_dn7, var_qdrat_dn10, var_qdrat_dn11, var_qdrat_dn12, var_qdrat_dn17,)
    }
};
        var_qdrat = assign36380_e51081;
        var_qdrat_dn0 = assign36380_e51081_d_n0;
        var_qdrat_dn2 = assign36380_e51081_d_n2;
        var_qdrat_dn6 = assign36380_e51081_d_n6;
        var_qdrat_dn7 = assign36380_e51081_d_n7;
        var_qdrat_dn10 = assign36380_e51081_d_n10;
        var_qdrat_dn11 = assign36380_e51081_d_n11;
        var_qdrat_dn12 = assign36380_e51081_d_n12;
        var_qdrat_dn17 = assign36380_e51081_d_n17;
        var_qdrat_rv = 0.0;

        let (assign36410_e51111, assign36410_e51111_d_n0, assign36410_e51111_d_n2, assign36410_e51111_d_n6, assign36410_e51111_d_n7, assign36410_e51111_d_n10, assign36410_e51111_d_n11, assign36410_e51111_d_n12, assign36410_e51111_d_n15, assign36410_e51111_d_n17, assign36410_e51111_d_n18,) = {
    if ((var_guard1207 != 0.0) && (var_flg_nqs != 0.0)) {
        let assign36410_e51107: f64 = (var_qi_nqs * var_qdrat);
        let assign36410_e51109: f64 = (assign36410_e51107 + var_q_bt_se);
        (assign36410_e51109, ((var_qi_nqs * var_qdrat_dn0) + var_q_bt_se_dn0), ((var_qi_nqs * var_qdrat_dn2) + var_q_bt_se_dn2), ((var_qi_nqs * var_qdrat_dn6) + var_q_bt_se_dn6), ((var_qi_nqs * var_qdrat_dn7) + var_q_bt_se_dn7), ((var_qi_nqs * var_qdrat_dn10) + var_q_bt_se_dn10), ((var_qi_nqs * var_qdrat_dn11) + var_q_bt_se_dn11), ((var_qi_nqs * var_qdrat_dn12) + var_q_bt_se_dn12), 0.0, ((var_qi_nqs * var_qdrat_dn17) + var_q_bt_se_dn17), (var_qi_nqs_dn18 * var_qdrat),)
    } else {
        (var_qd_nqs, var_qd_nqs_dn0, var_qd_nqs_dn2, var_qd_nqs_dn6, var_qd_nqs_dn7, var_qd_nqs_dn10, var_qd_nqs_dn11, var_qd_nqs_dn12, var_qd_nqs_dn15, var_qd_nqs_dn17, var_qd_nqs_dn18,)
    }
};
        var_qd_nqs = assign36410_e51111;
        var_qd_nqs_dn0 = assign36410_e51111_d_n0;
        var_qd_nqs_dn2 = assign36410_e51111_d_n2;
        var_qd_nqs_dn6 = assign36410_e51111_d_n6;
        var_qd_nqs_dn7 = assign36410_e51111_d_n7;
        var_qd_nqs_dn10 = assign36410_e51111_d_n10;
        var_qd_nqs_dn11 = assign36410_e51111_d_n11;
        var_qd_nqs_dn12 = assign36410_e51111_d_n12;
        var_qd_nqs_dn15 = assign36410_e51111_d_n15;
        var_qd_nqs_dn17 = assign36410_e51111_d_n17;
        var_qd_nqs_dn18 = assign36410_e51111_d_n18;
        var_qd_nqs_rv = 0.0;

        let (assign36420_e51123, assign36420_e51123_d_n0, assign36420_e51123_d_n2, assign36420_e51123_d_n6, assign36420_e51123_d_n7, assign36420_e51123_d_n10, assign36420_e51123_d_n11, assign36420_e51123_d_n12, assign36420_e51123_d_n16, assign36420_e51123_d_n17, assign36420_e51123_d_n18,) = {
    if ((var_guard1207 != 0.0) && (var_flg_nqs != 0.0)) {
        let assign36420_e51118: f64 = (1.0 - var_qdrat);
        let assign36420_e51119: f64 = (var_qi_nqs * assign36420_e51118);
        let assign36420_e51121: f64 = (assign36420_e51119 + var_q_bt_se);
        (assign36420_e51121, ((var_qi_nqs * (-var_qdrat_dn0)) + var_q_bt_se_dn0), ((var_qi_nqs * (-var_qdrat_dn2)) + var_q_bt_se_dn2), ((var_qi_nqs * (-var_qdrat_dn6)) + var_q_bt_se_dn6), ((var_qi_nqs * (-var_qdrat_dn7)) + var_q_bt_se_dn7), ((var_qi_nqs * (-var_qdrat_dn10)) + var_q_bt_se_dn10), ((var_qi_nqs * (-var_qdrat_dn11)) + var_q_bt_se_dn11), ((var_qi_nqs * (-var_qdrat_dn12)) + var_q_bt_se_dn12), 0.0, ((var_qi_nqs * (-var_qdrat_dn17)) + var_q_bt_se_dn17), (var_qi_nqs_dn18 * assign36420_e51118),)
    } else {
        (var_qs_nqs, var_qs_nqs_dn0, var_qs_nqs_dn2, var_qs_nqs_dn6, var_qs_nqs_dn7, var_qs_nqs_dn10, var_qs_nqs_dn11, var_qs_nqs_dn12, var_qs_nqs_dn16, var_qs_nqs_dn17, var_qs_nqs_dn18,)
    }
};
        var_qs_nqs = assign36420_e51123;
        var_qs_nqs_dn0 = assign36420_e51123_d_n0;
        var_qs_nqs_dn2 = assign36420_e51123_d_n2;
        var_qs_nqs_dn6 = assign36420_e51123_d_n6;
        var_qs_nqs_dn7 = assign36420_e51123_d_n7;
        var_qs_nqs_dn10 = assign36420_e51123_d_n10;
        var_qs_nqs_dn11 = assign36420_e51123_d_n11;
        var_qs_nqs_dn12 = assign36420_e51123_d_n12;
        var_qs_nqs_dn16 = assign36420_e51123_d_n16;
        var_qs_nqs_dn17 = assign36420_e51123_d_n17;
        var_qs_nqs_dn18 = assign36420_e51123_d_n18;
        var_qs_nqs_rv = 0.0;

        let (assign36430_e51134, assign36430_e51134_d_n0, assign36430_e51134_d_n2, assign36430_e51134_d_n6, assign36430_e51134_d_n7, assign36430_e51134_d_n10, assign36430_e51134_d_n11, assign36430_e51134_d_n12, assign36430_e51134_d_n13, assign36430_e51134_d_n15, assign36430_e51134_d_n16, assign36430_e51134_d_n17, assign36430_e51134_d_n18,) = {
    if ((var_guard1207 != 0.0) && (var_flg_nqs != 0.0)) {
        let assign36430_e51128: f64 = (-var_qi_nqs);
        let assign36430_e51130: f64 = (assign36430_e51128 - var_qb_nqs);
        let assign36430_e51132: f64 = (assign36430_e51130 + var_q_bt_ge);
        (assign36430_e51132, var_q_bt_ge_dn0, var_q_bt_ge_dn2, var_q_bt_ge_dn6, var_q_bt_ge_dn7, var_q_bt_ge_dn10, var_q_bt_ge_dn11, var_q_bt_ge_dn12, (-var_qb_nqs_dn13), 0.0, 0.0, var_q_bt_ge_dn17, (-var_qi_nqs_dn18),)
    } else {
        (var_qg_nqs, var_qg_nqs_dn0, var_qg_nqs_dn2, var_qg_nqs_dn6, var_qg_nqs_dn7, var_qg_nqs_dn10, var_qg_nqs_dn11, var_qg_nqs_dn12, var_qg_nqs_dn13, var_qg_nqs_dn15, var_qg_nqs_dn16, var_qg_nqs_dn17, var_qg_nqs_dn18,)
    }
};
        var_qg_nqs = assign36430_e51134;
        var_qg_nqs_dn0 = assign36430_e51134_d_n0;
        var_qg_nqs_dn2 = assign36430_e51134_d_n2;
        var_qg_nqs_dn6 = assign36430_e51134_d_n6;
        var_qg_nqs_dn7 = assign36430_e51134_d_n7;
        var_qg_nqs_dn10 = assign36430_e51134_d_n10;
        var_qg_nqs_dn11 = assign36430_e51134_d_n11;
        var_qg_nqs_dn12 = assign36430_e51134_d_n12;
        var_qg_nqs_dn13 = assign36430_e51134_d_n13;
        var_qg_nqs_dn15 = assign36430_e51134_d_n15;
        var_qg_nqs_dn16 = assign36430_e51134_d_n16;
        var_qg_nqs_dn17 = assign36430_e51134_d_n17;
        var_qg_nqs_dn18 = assign36430_e51134_d_n18;
        var_qg_nqs_rv = 0.0;

        let (assign36460_e51155, assign36460_e51155_d_n0, assign36460_e51155_d_n2, assign36460_e51155_d_n6, assign36460_e51155_d_n7, assign36460_e51155_d_n10, assign36460_e51155_d_n11, assign36460_e51155_d_n12, assign36460_e51155_d_n15, assign36460_e51155_d_n17, assign36460_e51155_d_n18,) = {
    if ((var_guard1207 != 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qd_nqs, var_qd_nqs_dn0, var_qd_nqs_dn2, var_qd_nqs_dn6, var_qd_nqs_dn7, var_qd_nqs_dn10, var_qd_nqs_dn11, var_qd_nqs_dn12, var_qd_nqs_dn15, var_qd_nqs_dn17, var_qd_nqs_dn18,)
    }
};
        var_qd_nqs = assign36460_e51155;
        var_qd_nqs_dn0 = assign36460_e51155_d_n0;
        var_qd_nqs_dn2 = assign36460_e51155_d_n2;
        var_qd_nqs_dn6 = assign36460_e51155_d_n6;
        var_qd_nqs_dn7 = assign36460_e51155_d_n7;
        var_qd_nqs_dn10 = assign36460_e51155_d_n10;
        var_qd_nqs_dn11 = assign36460_e51155_d_n11;
        var_qd_nqs_dn12 = assign36460_e51155_d_n12;
        var_qd_nqs_dn15 = assign36460_e51155_d_n15;
        var_qd_nqs_dn17 = assign36460_e51155_d_n17;
        var_qd_nqs_dn18 = assign36460_e51155_d_n18;
        var_qd_nqs_rv = 0.0;

        let (assign36470_e51162, assign36470_e51162_d_n0, assign36470_e51162_d_n2, assign36470_e51162_d_n6, assign36470_e51162_d_n7, assign36470_e51162_d_n10, assign36470_e51162_d_n11, assign36470_e51162_d_n12, assign36470_e51162_d_n16, assign36470_e51162_d_n17, assign36470_e51162_d_n18,) = {
    if ((var_guard1207 != 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qs_nqs, var_qs_nqs_dn0, var_qs_nqs_dn2, var_qs_nqs_dn6, var_qs_nqs_dn7, var_qs_nqs_dn10, var_qs_nqs_dn11, var_qs_nqs_dn12, var_qs_nqs_dn16, var_qs_nqs_dn17, var_qs_nqs_dn18,)
    }
};
        var_qs_nqs = assign36470_e51162;
        var_qs_nqs_dn0 = assign36470_e51162_d_n0;
        var_qs_nqs_dn2 = assign36470_e51162_d_n2;
        var_qs_nqs_dn6 = assign36470_e51162_d_n6;
        var_qs_nqs_dn7 = assign36470_e51162_d_n7;
        var_qs_nqs_dn10 = assign36470_e51162_d_n10;
        var_qs_nqs_dn11 = assign36470_e51162_d_n11;
        var_qs_nqs_dn12 = assign36470_e51162_d_n12;
        var_qs_nqs_dn16 = assign36470_e51162_d_n16;
        var_qs_nqs_dn17 = assign36470_e51162_d_n17;
        var_qs_nqs_dn18 = assign36470_e51162_d_n18;
        var_qs_nqs_rv = 0.0;

        let (assign36480_e51169, assign36480_e51169_d_n0, assign36480_e51169_d_n2, assign36480_e51169_d_n6, assign36480_e51169_d_n7, assign36480_e51169_d_n10, assign36480_e51169_d_n11, assign36480_e51169_d_n12, assign36480_e51169_d_n13, assign36480_e51169_d_n15, assign36480_e51169_d_n16, assign36480_e51169_d_n17, assign36480_e51169_d_n18,) = {
    if ((var_guard1207 != 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qg_nqs, var_qg_nqs_dn0, var_qg_nqs_dn2, var_qg_nqs_dn6, var_qg_nqs_dn7, var_qg_nqs_dn10, var_qg_nqs_dn11, var_qg_nqs_dn12, var_qg_nqs_dn13, var_qg_nqs_dn15, var_qg_nqs_dn16, var_qg_nqs_dn17, var_qg_nqs_dn18,)
    }
};
        var_qg_nqs = assign36480_e51169;
        var_qg_nqs_dn0 = assign36480_e51169_d_n0;
        var_qg_nqs_dn2 = assign36480_e51169_d_n2;
        var_qg_nqs_dn6 = assign36480_e51169_d_n6;
        var_qg_nqs_dn7 = assign36480_e51169_d_n7;
        var_qg_nqs_dn10 = assign36480_e51169_d_n10;
        var_qg_nqs_dn11 = assign36480_e51169_d_n11;
        var_qg_nqs_dn12 = assign36480_e51169_d_n12;
        var_qg_nqs_dn13 = assign36480_e51169_d_n13;
        var_qg_nqs_dn15 = assign36480_e51169_d_n15;
        var_qg_nqs_dn16 = assign36480_e51169_d_n16;
        var_qg_nqs_dn17 = assign36480_e51169_d_n17;
        var_qg_nqs_dn18 = assign36480_e51169_d_n18;
        var_qg_nqs_rv = 0.0;

        let (assign36490_e51176, assign36490_e51176_d_n13,) = {
    if ((var_guard1207 != 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_qb_nqs, var_qb_nqs_dn13,)
    }
};
        var_qb_nqs = assign36490_e51176;
        var_qb_nqs_dn13 = assign36490_e51176_d_n13;
        var_qb_nqs_rv = 0.0;

        let (assign36580_e51260, assign36580_e51260_d_n0, assign36580_e51260_d_n2, assign36580_e51260_d_n6, assign36580_e51260_d_n7, assign36580_e51260_d_n10, assign36580_e51260_d_n11, assign36580_e51260_d_n12, assign36580_e51260_d_n13, assign36580_e51260_d_n15, assign36580_e51260_d_n16, assign36580_e51260_d_n17, assign36580_e51260_d_n18,) = {
    if ((var_guard1207 == 0.0) && (var_flg_nqs != 0.0)) {
        let assign36580_e51254: f64 = (-var_qd_nqs);
        let assign36580_e51256: f64 = (assign36580_e51254 - var_qs_nqs);
        let assign36580_e51258: f64 = (assign36580_e51256 - var_qb_nqs);
        (assign36580_e51258, ((-var_qd_nqs_dn0) - var_qs_nqs_dn0), ((-var_qd_nqs_dn2) - var_qs_nqs_dn2), ((-var_qd_nqs_dn6) - var_qs_nqs_dn6), ((-var_qd_nqs_dn7) - var_qs_nqs_dn7), ((-var_qd_nqs_dn10) - var_qs_nqs_dn10), ((-var_qd_nqs_dn11) - var_qs_nqs_dn11), ((-var_qd_nqs_dn12) - var_qs_nqs_dn12), (-var_qb_nqs_dn13), (-var_qd_nqs_dn15), (-var_qs_nqs_dn16), ((-var_qd_nqs_dn17) - var_qs_nqs_dn17), ((-var_qd_nqs_dn18) - var_qs_nqs_dn18),)
    } else {
        (var_qg_nqs, var_qg_nqs_dn0, var_qg_nqs_dn2, var_qg_nqs_dn6, var_qg_nqs_dn7, var_qg_nqs_dn10, var_qg_nqs_dn11, var_qg_nqs_dn12, var_qg_nqs_dn13, var_qg_nqs_dn15, var_qg_nqs_dn16, var_qg_nqs_dn17, var_qg_nqs_dn18,)
    }
};
        var_qg_nqs = assign36580_e51260;
        var_qg_nqs_dn0 = assign36580_e51260_d_n0;
        var_qg_nqs_dn2 = assign36580_e51260_d_n2;
        var_qg_nqs_dn6 = assign36580_e51260_d_n6;
        var_qg_nqs_dn7 = assign36580_e51260_d_n7;
        var_qg_nqs_dn10 = assign36580_e51260_d_n10;
        var_qg_nqs_dn11 = assign36580_e51260_d_n11;
        var_qg_nqs_dn12 = assign36580_e51260_d_n12;
        var_qg_nqs_dn13 = assign36580_e51260_d_n13;
        var_qg_nqs_dn15 = assign36580_e51260_d_n15;
        var_qg_nqs_dn16 = assign36580_e51260_d_n16;
        var_qg_nqs_dn17 = assign36580_e51260_d_n17;
        var_qg_nqs_dn18 = assign36580_e51260_d_n18;
        var_qg_nqs_rv = 0.0;

        let (assign36620_e51292, assign36620_e51292_d_n0, assign36620_e51292_d_n2, assign36620_e51292_d_n6, assign36620_e51292_d_n7, assign36620_e51292_d_n10, assign36620_e51292_d_n11, assign36620_e51292_d_n12, assign36620_e51292_d_n15, assign36620_e51292_d_n17, assign36620_e51292_d_n18,) = {
    if ((var_guard1207 == 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qd_nqs, var_qd_nqs_dn0, var_qd_nqs_dn2, var_qd_nqs_dn6, var_qd_nqs_dn7, var_qd_nqs_dn10, var_qd_nqs_dn11, var_qd_nqs_dn12, var_qd_nqs_dn15, var_qd_nqs_dn17, var_qd_nqs_dn18,)
    }
};
        var_qd_nqs = assign36620_e51292;
        var_qd_nqs_dn0 = assign36620_e51292_d_n0;
        var_qd_nqs_dn2 = assign36620_e51292_d_n2;
        var_qd_nqs_dn6 = assign36620_e51292_d_n6;
        var_qd_nqs_dn7 = assign36620_e51292_d_n7;
        var_qd_nqs_dn10 = assign36620_e51292_d_n10;
        var_qd_nqs_dn11 = assign36620_e51292_d_n11;
        var_qd_nqs_dn12 = assign36620_e51292_d_n12;
        var_qd_nqs_dn15 = assign36620_e51292_d_n15;
        var_qd_nqs_dn17 = assign36620_e51292_d_n17;
        var_qd_nqs_dn18 = assign36620_e51292_d_n18;
        var_qd_nqs_rv = 0.0;

        *var_guard1200_slot = var_guard1200;
        *var_guard1200_rv_slot = var_guard1200_rv;
        *var_guard1201_slot = var_guard1201;
        *var_guard1201_rv_slot = var_guard1201_rv;
        *var_guard1202_slot = var_guard1202;
        *var_guard1202_rv_slot = var_guard1202_rv;
        *var_guard1203_slot = var_guard1203;
        *var_guard1203_rv_slot = var_guard1203_rv;
        *var_guard1204_slot = var_guard1204;
        *var_guard1204_rv_slot = var_guard1204_rv;
        *var_guard1207_slot = var_guard1207;
        *var_guard1207_rv_slot = var_guard1207_rv;
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
        *var_vdri__blk1194_slot = var_vdri__blk1194;
        *var_vdri__blk1194_dn0_slot = var_vdri__blk1194_dn0;
        *var_vdri__blk1194_dn10_slot = var_vdri__blk1194_dn10;
        *var_vdri__blk1194_dn11_slot = var_vdri__blk1194_dn11;
        *var_vdri__blk1194_dn12_slot = var_vdri__blk1194_dn12;
        *var_vdri__blk1194_dn17_slot = var_vdri__blk1194_dn17;
        *var_vdri__blk1194_dn2_slot = var_vdri__blk1194_dn2;
        *var_vdri__blk1194_dn6_slot = var_vdri__blk1194_dn6;
        *var_vdri__blk1194_dn7_slot = var_vdri__blk1194_dn7;
        *var_vdri__blk1194_rv_slot = var_vdri__blk1194_rv;
    }

    pub(super) fn stamp_reactive_block_129(
        p: &Parameters,
        var_cth: f64,
        var_flg_nqs: f64,
        var_guard1207: f64,
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
        var_guard1212_slot: &mut f64,
        var_guard1212_rv_slot: &mut f64,
        var_guard1213_slot: &mut f64,
        var_guard1213_rv_slot: &mut f64,
        var_guard1214_slot: &mut f64,
        var_guard1214_rv_slot: &mut f64,
        var_guard1216_slot: &mut f64,
        var_guard1216_rv_slot: &mut f64,
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
        let mut var_guard1212: f64 = *var_guard1212_slot;
        let mut var_guard1212_rv: f64 = *var_guard1212_rv_slot;
        let mut var_guard1213: f64 = *var_guard1213_slot;
        let mut var_guard1213_rv: f64 = *var_guard1213_rv_slot;
        let mut var_guard1214: f64 = *var_guard1214_slot;
        let mut var_guard1214_rv: f64 = *var_guard1214_rv_slot;
        let mut var_guard1216: f64 = *var_guard1216_slot;
        let mut var_guard1216_rv: f64 = *var_guard1216_rv_slot;
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

        let (assign36630_e51300, assign36630_e51300_d_n0, assign36630_e51300_d_n2, assign36630_e51300_d_n6, assign36630_e51300_d_n7, assign36630_e51300_d_n10, assign36630_e51300_d_n11, assign36630_e51300_d_n12, assign36630_e51300_d_n16, assign36630_e51300_d_n17, assign36630_e51300_d_n18,) = {
    if ((var_guard1207 == 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qs_nqs, var_qs_nqs_dn0, var_qs_nqs_dn2, var_qs_nqs_dn6, var_qs_nqs_dn7, var_qs_nqs_dn10, var_qs_nqs_dn11, var_qs_nqs_dn12, var_qs_nqs_dn16, var_qs_nqs_dn17, var_qs_nqs_dn18,)
    }
};
        var_qs_nqs = assign36630_e51300;
        var_qs_nqs_dn0 = assign36630_e51300_d_n0;
        var_qs_nqs_dn2 = assign36630_e51300_d_n2;
        var_qs_nqs_dn6 = assign36630_e51300_d_n6;
        var_qs_nqs_dn7 = assign36630_e51300_d_n7;
        var_qs_nqs_dn10 = assign36630_e51300_d_n10;
        var_qs_nqs_dn11 = assign36630_e51300_d_n11;
        var_qs_nqs_dn12 = assign36630_e51300_d_n12;
        var_qs_nqs_dn16 = assign36630_e51300_d_n16;
        var_qs_nqs_dn17 = assign36630_e51300_d_n17;
        var_qs_nqs_dn18 = assign36630_e51300_d_n18;
        var_qs_nqs_rv = 0.0;

        let (assign36640_e51308, assign36640_e51308_d_n0, assign36640_e51308_d_n2, assign36640_e51308_d_n6, assign36640_e51308_d_n7, assign36640_e51308_d_n10, assign36640_e51308_d_n11, assign36640_e51308_d_n12, assign36640_e51308_d_n13, assign36640_e51308_d_n15, assign36640_e51308_d_n16, assign36640_e51308_d_n17, assign36640_e51308_d_n18,) = {
    if ((var_guard1207 == 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qg_nqs, var_qg_nqs_dn0, var_qg_nqs_dn2, var_qg_nqs_dn6, var_qg_nqs_dn7, var_qg_nqs_dn10, var_qg_nqs_dn11, var_qg_nqs_dn12, var_qg_nqs_dn13, var_qg_nqs_dn15, var_qg_nqs_dn16, var_qg_nqs_dn17, var_qg_nqs_dn18,)
    }
};
        var_qg_nqs = assign36640_e51308;
        var_qg_nqs_dn0 = assign36640_e51308_d_n0;
        var_qg_nqs_dn2 = assign36640_e51308_d_n2;
        var_qg_nqs_dn6 = assign36640_e51308_d_n6;
        var_qg_nqs_dn7 = assign36640_e51308_d_n7;
        var_qg_nqs_dn10 = assign36640_e51308_d_n10;
        var_qg_nqs_dn11 = assign36640_e51308_d_n11;
        var_qg_nqs_dn12 = assign36640_e51308_d_n12;
        var_qg_nqs_dn13 = assign36640_e51308_d_n13;
        var_qg_nqs_dn15 = assign36640_e51308_d_n15;
        var_qg_nqs_dn16 = assign36640_e51308_d_n16;
        var_qg_nqs_dn17 = assign36640_e51308_d_n17;
        var_qg_nqs_dn18 = assign36640_e51308_d_n18;
        var_qg_nqs_rv = 0.0;

        let (assign36650_e51316, assign36650_e51316_d_n13,) = {
    if ((var_guard1207 == 0.0) && (var_flg_nqs == 0.0)) {
        (0.0, 0.0,)
    } else {
        (var_qb_nqs, var_qb_nqs_dn13,)
    }
};
        var_qb_nqs = assign36650_e51316;
        var_qb_nqs_dn13 = assign36650_e51316_d_n13;
        var_qb_nqs_rv = 0.0;

        let assign36680_e51321: f64 = if var_mode == 1.0 { 1.0 } else { 0.0 };
        var_guard1212 = assign36680_e51321;
        var_guard1212_rv = 0.0;

        let (assign36690_e51325, assign36690_e51325_d_n0, assign36690_e51325_d_n2, assign36690_e51325_d_n6, assign36690_e51325_d_n7, assign36690_e51325_d_n10, assign36690_e51325_d_n11, assign36690_e51325_d_n12, assign36690_e51325_d_n17,) = {
    if (var_guard1212 != 0.0) {
        (var_idse, var_idse_dn0, var_idse_dn2, var_idse_dn6, var_idse_dn7, var_idse_dn10, var_idse_dn11, var_idse_dn12, var_idse_dn17,)
    } else {
        (var_ids, var_ids_dn0, var_ids_dn2, var_ids_dn6, var_ids_dn7, var_ids_dn10, var_ids_dn11, var_ids_dn12, var_ids_dn17,)
    }
};
        var_ids = assign36690_e51325;
        var_ids_dn0 = assign36690_e51325_d_n0;
        var_ids_dn2 = assign36690_e51325_d_n2;
        var_ids_dn6 = assign36690_e51325_d_n6;
        var_ids_dn7 = assign36690_e51325_d_n7;
        var_ids_dn10 = assign36690_e51325_d_n10;
        var_ids_dn11 = assign36690_e51325_d_n11;
        var_ids_dn12 = assign36690_e51325_d_n12;
        var_ids_dn17 = assign36690_e51325_d_n17;
        var_ids_rv = 0.0;

        let (assign36700_e51329, assign36700_e51329_d_n0, assign36700_e51329_d_n2, assign36700_e51329_d_n6, assign36700_e51329_d_n7, assign36700_e51329_d_n10, assign36700_e51329_d_n11, assign36700_e51329_d_n12, assign36700_e51329_d_n17,) = {
    if (var_guard1212 != 0.0) {
        (var_isube, var_isube_dn0, var_isube_dn2, var_isube_dn6, var_isube_dn7, var_isube_dn10, var_isube_dn11, var_isube_dn12, var_isube_dn17,)
    } else {
        (var_isub, var_isub_dn0, var_isub_dn2, var_isub_dn6, var_isub_dn7, var_isub_dn10, var_isub_dn11, var_isub_dn12, var_isub_dn17,)
    }
};
        var_isub = assign36700_e51329;
        var_isub_dn0 = assign36700_e51329_d_n0;
        var_isub_dn2 = assign36700_e51329_d_n2;
        var_isub_dn6 = assign36700_e51329_d_n6;
        var_isub_dn7 = assign36700_e51329_d_n7;
        var_isub_dn10 = assign36700_e51329_d_n10;
        var_isub_dn11 = assign36700_e51329_d_n11;
        var_isub_dn12 = assign36700_e51329_d_n12;
        var_isub_dn17 = assign36700_e51329_d_n17;
        var_isub_rv = 0.0;

        let (assign36720_e51339, assign36720_e51339_d_n0, assign36720_e51339_d_n2, assign36720_e51339_d_n6, assign36720_e51339_d_n7, assign36720_e51339_d_n10, assign36720_e51339_d_n11, assign36720_e51339_d_n12, assign36720_e51339_d_n13, assign36720_e51339_d_n15, assign36720_e51339_d_n16, assign36720_e51339_d_n17, assign36720_e51339_d_n18,) = {
    if (var_guard1212 != 0.0) {
        let assign36720_e51337: f64 = (var_qge + var_qg_nqs);
        (assign36720_e51337, (var_qge_dn0 + var_qg_nqs_dn0), (var_qge_dn2 + var_qg_nqs_dn2), (var_qge_dn6 + var_qg_nqs_dn6), (var_qge_dn7 + var_qg_nqs_dn7), (var_qge_dn10 + var_qg_nqs_dn10), (var_qge_dn11 + var_qg_nqs_dn11), (var_qge_dn12 + var_qg_nqs_dn12), (var_qge_dn13 + var_qg_nqs_dn13), (var_qge_dn15 + var_qg_nqs_dn15), (var_qge_dn16 + var_qg_nqs_dn16), (var_qge_dn17 + var_qg_nqs_dn17), (var_qge_dn18 + var_qg_nqs_dn18),)
    } else {
        (var_qg, var_qg_dn0, var_qg_dn2, var_qg_dn6, var_qg_dn7, var_qg_dn10, var_qg_dn11, var_qg_dn12, var_qg_dn13, var_qg_dn15, var_qg_dn16, var_qg_dn17, var_qg_dn18,)
    }
};
        var_qg = assign36720_e51339;
        var_qg_dn0 = assign36720_e51339_d_n0;
        var_qg_dn2 = assign36720_e51339_d_n2;
        var_qg_dn6 = assign36720_e51339_d_n6;
        var_qg_dn7 = assign36720_e51339_d_n7;
        var_qg_dn10 = assign36720_e51339_d_n10;
        var_qg_dn11 = assign36720_e51339_d_n11;
        var_qg_dn12 = assign36720_e51339_d_n12;
        var_qg_dn13 = assign36720_e51339_d_n13;
        var_qg_dn15 = assign36720_e51339_d_n15;
        var_qg_dn16 = assign36720_e51339_d_n16;
        var_qg_dn17 = assign36720_e51339_d_n17;
        var_qg_dn18 = assign36720_e51339_d_n18;
        var_qg_rv = 0.0;

        let (assign36730_e51345, assign36730_e51345_d_n0, assign36730_e51345_d_n2, assign36730_e51345_d_n6, assign36730_e51345_d_n7, assign36730_e51345_d_n10, assign36730_e51345_d_n11, assign36730_e51345_d_n12, assign36730_e51345_d_n13, assign36730_e51345_d_n15, assign36730_e51345_d_n16, assign36730_e51345_d_n17, assign36730_e51345_d_n18,) = {
    if (var_guard1212 != 0.0) {
        let assign36730_e51343: f64 = (var_qde + var_qd_nqs);
        (assign36730_e51343, (var_qde_dn0 + var_qd_nqs_dn0), (var_qde_dn2 + var_qd_nqs_dn2), (var_qde_dn6 + var_qd_nqs_dn6), (var_qde_dn7 + var_qd_nqs_dn7), (var_qde_dn10 + var_qd_nqs_dn10), (var_qde_dn11 + var_qd_nqs_dn11), (var_qde_dn12 + var_qd_nqs_dn12), var_qde_dn13, (var_qde_dn15 + var_qd_nqs_dn15), var_qde_dn16, (var_qde_dn17 + var_qd_nqs_dn17), (var_qde_dn18 + var_qd_nqs_dn18),)
    } else {
        (var_qd, var_qd_dn0, var_qd_dn2, var_qd_dn6, var_qd_dn7, var_qd_dn10, var_qd_dn11, var_qd_dn12, var_qd_dn13, var_qd_dn15, var_qd_dn16, var_qd_dn17, var_qd_dn18,)
    }
};
        var_qd = assign36730_e51345;
        var_qd_dn0 = assign36730_e51345_d_n0;
        var_qd_dn2 = assign36730_e51345_d_n2;
        var_qd_dn6 = assign36730_e51345_d_n6;
        var_qd_dn7 = assign36730_e51345_d_n7;
        var_qd_dn10 = assign36730_e51345_d_n10;
        var_qd_dn11 = assign36730_e51345_d_n11;
        var_qd_dn12 = assign36730_e51345_d_n12;
        var_qd_dn13 = assign36730_e51345_d_n13;
        var_qd_dn15 = assign36730_e51345_d_n15;
        var_qd_dn16 = assign36730_e51345_d_n16;
        var_qd_dn17 = assign36730_e51345_d_n17;
        var_qd_dn18 = assign36730_e51345_d_n18;
        var_qd_rv = 0.0;

        let (assign36750_e51360, assign36750_e51360_d_n0, assign36750_e51360_d_n2, assign36750_e51360_d_n6, assign36750_e51360_d_n7, assign36750_e51360_d_n10, assign36750_e51360_d_n11, assign36750_e51360_d_n12, assign36750_e51360_d_n13, assign36750_e51360_d_n15, assign36750_e51360_d_n16, assign36750_e51360_d_n17, assign36750_e51360_d_n18,) = {
    if (var_guard1212 != 0.0) {
        let assign36750_e51355: f64 = (var_qge + var_qde);
        let assign36750_e51357: f64 = (assign36750_e51355 + var_qse);
        let assign36750_e51358: f64 = (-assign36750_e51357);
        (assign36750_e51358, (-((var_qge_dn0 + var_qde_dn0) + var_qse_dn0)), (-((var_qge_dn2 + var_qde_dn2) + var_qse_dn2)), (-((var_qge_dn6 + var_qde_dn6) + var_qse_dn6)), (-((var_qge_dn7 + var_qde_dn7) + var_qse_dn7)), (-((var_qge_dn10 + var_qde_dn10) + var_qse_dn10)), (-((var_qge_dn11 + var_qde_dn11) + var_qse_dn11)), (-((var_qge_dn12 + var_qde_dn12) + var_qse_dn12)), (-((var_qge_dn13 + var_qde_dn13) + var_qse_dn13)), (-((var_qge_dn15 + var_qde_dn15) + var_qse_dn15)), (-((var_qge_dn16 + var_qde_dn16) + var_qse_dn16)), (-((var_qge_dn17 + var_qde_dn17) + var_qse_dn17)), (-((var_qge_dn18 + var_qde_dn18) + var_qse_dn18)),)
    } else {
        (var_qbe, var_qbe_dn0, var_qbe_dn2, var_qbe_dn6, var_qbe_dn7, var_qbe_dn10, var_qbe_dn11, var_qbe_dn12, var_qbe_dn13, var_qbe_dn15, var_qbe_dn16, var_qbe_dn17, var_qbe_dn18,)
    }
};
        var_qbe = assign36750_e51360;
        var_qbe_dn0 = assign36750_e51360_d_n0;
        var_qbe_dn2 = assign36750_e51360_d_n2;
        var_qbe_dn6 = assign36750_e51360_d_n6;
        var_qbe_dn7 = assign36750_e51360_d_n7;
        var_qbe_dn10 = assign36750_e51360_d_n10;
        var_qbe_dn11 = assign36750_e51360_d_n11;
        var_qbe_dn12 = assign36750_e51360_d_n12;
        var_qbe_dn13 = assign36750_e51360_d_n13;
        var_qbe_dn15 = assign36750_e51360_d_n15;
        var_qbe_dn16 = assign36750_e51360_d_n16;
        var_qbe_dn17 = assign36750_e51360_d_n17;
        var_qbe_dn18 = assign36750_e51360_d_n18;
        var_qbe_rv = 0.0;

        let (assign36760_e51366, assign36760_e51366_d_n0, assign36760_e51366_d_n2, assign36760_e51366_d_n6, assign36760_e51366_d_n7, assign36760_e51366_d_n10, assign36760_e51366_d_n11, assign36760_e51366_d_n12, assign36760_e51366_d_n13, assign36760_e51366_d_n15, assign36760_e51366_d_n16, assign36760_e51366_d_n17, assign36760_e51366_d_n18,) = {
    if (var_guard1212 != 0.0) {
        let assign36760_e51364: f64 = (var_qbe + var_qb_nqs);
        (assign36760_e51364, var_qbe_dn0, var_qbe_dn2, var_qbe_dn6, var_qbe_dn7, var_qbe_dn10, var_qbe_dn11, var_qbe_dn12, (var_qbe_dn13 + var_qb_nqs_dn13), var_qbe_dn15, var_qbe_dn16, var_qbe_dn17, var_qbe_dn18,)
    } else {
        (var_qb, var_qb_dn0, var_qb_dn2, var_qb_dn6, var_qb_dn7, var_qb_dn10, var_qb_dn11, var_qb_dn12, var_qb_dn13, var_qb_dn15, var_qb_dn16, var_qb_dn17, var_qb_dn18,)
    }
};
        var_qb = assign36760_e51366;
        var_qb_dn0 = assign36760_e51366_d_n0;
        var_qb_dn2 = assign36760_e51366_d_n2;
        var_qb_dn6 = assign36760_e51366_d_n6;
        var_qb_dn7 = assign36760_e51366_d_n7;
        var_qb_dn10 = assign36760_e51366_d_n10;
        var_qb_dn11 = assign36760_e51366_d_n11;
        var_qb_dn12 = assign36760_e51366_d_n12;
        var_qb_dn13 = assign36760_e51366_d_n13;
        var_qb_dn15 = assign36760_e51366_d_n15;
        var_qb_dn16 = assign36760_e51366_d_n16;
        var_qb_dn17 = assign36760_e51366_d_n17;
        var_qb_dn18 = assign36760_e51366_d_n18;
        var_qb_rv = 0.0;

        let (assign36770_e51372, assign36770_e51372_d_n0, assign36770_e51372_d_n2, assign36770_e51372_d_n6, assign36770_e51372_d_n7, assign36770_e51372_d_n10, assign36770_e51372_d_n11, assign36770_e51372_d_n12, assign36770_e51372_d_n17,) = {
    if (var_guard1212 == 0.0) {
        let assign36770_e51370: f64 = (-var_idse);
        (assign36770_e51370, (-var_idse_dn0), (-var_idse_dn2), (-var_idse_dn6), (-var_idse_dn7), (-var_idse_dn10), (-var_idse_dn11), (-var_idse_dn12), (-var_idse_dn17),)
    } else {
        (var_ids, var_ids_dn0, var_ids_dn2, var_ids_dn6, var_ids_dn7, var_ids_dn10, var_ids_dn11, var_ids_dn12, var_ids_dn17,)
    }
};
        var_ids = assign36770_e51372;
        var_ids_dn0 = assign36770_e51372_d_n0;
        var_ids_dn2 = assign36770_e51372_d_n2;
        var_ids_dn6 = assign36770_e51372_d_n6;
        var_ids_dn7 = assign36770_e51372_d_n7;
        var_ids_dn10 = assign36770_e51372_d_n10;
        var_ids_dn11 = assign36770_e51372_d_n11;
        var_ids_dn12 = assign36770_e51372_d_n12;
        var_ids_dn17 = assign36770_e51372_d_n17;
        var_ids_rv = 0.0;

        let (assign36790_e51382, assign36790_e51382_d_n0, assign36790_e51382_d_n2, assign36790_e51382_d_n6, assign36790_e51382_d_n7, assign36790_e51382_d_n10, assign36790_e51382_d_n11, assign36790_e51382_d_n12, assign36790_e51382_d_n17,) = {
    if (var_guard1212 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isub, var_isub_dn0, var_isub_dn2, var_isub_dn6, var_isub_dn7, var_isub_dn10, var_isub_dn11, var_isub_dn12, var_isub_dn17,)
    }
};
        var_isub = assign36790_e51382;
        var_isub_dn0 = assign36790_e51382_d_n0;
        var_isub_dn2 = assign36790_e51382_d_n2;
        var_isub_dn6 = assign36790_e51382_d_n6;
        var_isub_dn7 = assign36790_e51382_d_n7;
        var_isub_dn10 = assign36790_e51382_d_n10;
        var_isub_dn11 = assign36790_e51382_d_n11;
        var_isub_dn12 = assign36790_e51382_d_n12;
        var_isub_dn17 = assign36790_e51382_d_n17;
        var_isub_rv = 0.0;

        let (assign36800_e51389, assign36800_e51389_d_n0, assign36800_e51389_d_n2, assign36800_e51389_d_n6, assign36800_e51389_d_n7, assign36800_e51389_d_n10, assign36800_e51389_d_n11, assign36800_e51389_d_n12, assign36800_e51389_d_n13, assign36800_e51389_d_n15, assign36800_e51389_d_n16, assign36800_e51389_d_n17, assign36800_e51389_d_n18,) = {
    if (var_guard1212 == 0.0) {
        let assign36800_e51387: f64 = (var_qge + var_qg_nqs);
        (assign36800_e51387, (var_qge_dn0 + var_qg_nqs_dn0), (var_qge_dn2 + var_qg_nqs_dn2), (var_qge_dn6 + var_qg_nqs_dn6), (var_qge_dn7 + var_qg_nqs_dn7), (var_qge_dn10 + var_qg_nqs_dn10), (var_qge_dn11 + var_qg_nqs_dn11), (var_qge_dn12 + var_qg_nqs_dn12), (var_qge_dn13 + var_qg_nqs_dn13), (var_qge_dn15 + var_qg_nqs_dn15), (var_qge_dn16 + var_qg_nqs_dn16), (var_qge_dn17 + var_qg_nqs_dn17), (var_qge_dn18 + var_qg_nqs_dn18),)
    } else {
        (var_qg, var_qg_dn0, var_qg_dn2, var_qg_dn6, var_qg_dn7, var_qg_dn10, var_qg_dn11, var_qg_dn12, var_qg_dn13, var_qg_dn15, var_qg_dn16, var_qg_dn17, var_qg_dn18,)
    }
};
        var_qg = assign36800_e51389;
        var_qg_dn0 = assign36800_e51389_d_n0;
        var_qg_dn2 = assign36800_e51389_d_n2;
        var_qg_dn6 = assign36800_e51389_d_n6;
        var_qg_dn7 = assign36800_e51389_d_n7;
        var_qg_dn10 = assign36800_e51389_d_n10;
        var_qg_dn11 = assign36800_e51389_d_n11;
        var_qg_dn12 = assign36800_e51389_d_n12;
        var_qg_dn13 = assign36800_e51389_d_n13;
        var_qg_dn15 = assign36800_e51389_d_n15;
        var_qg_dn16 = assign36800_e51389_d_n16;
        var_qg_dn17 = assign36800_e51389_d_n17;
        var_qg_dn18 = assign36800_e51389_d_n18;
        var_qg_rv = 0.0;

        let (assign36810_e51396, assign36810_e51396_d_n0, assign36810_e51396_d_n2, assign36810_e51396_d_n6, assign36810_e51396_d_n7, assign36810_e51396_d_n10, assign36810_e51396_d_n11, assign36810_e51396_d_n12, assign36810_e51396_d_n13, assign36810_e51396_d_n15, assign36810_e51396_d_n16, assign36810_e51396_d_n17, assign36810_e51396_d_n18,) = {
    if (var_guard1212 == 0.0) {
        let assign36810_e51394: f64 = (var_qse + var_qs_nqs);
        (assign36810_e51394, (var_qse_dn0 + var_qs_nqs_dn0), (var_qse_dn2 + var_qs_nqs_dn2), (var_qse_dn6 + var_qs_nqs_dn6), (var_qse_dn7 + var_qs_nqs_dn7), (var_qse_dn10 + var_qs_nqs_dn10), (var_qse_dn11 + var_qs_nqs_dn11), (var_qse_dn12 + var_qs_nqs_dn12), var_qse_dn13, var_qse_dn15, (var_qse_dn16 + var_qs_nqs_dn16), (var_qse_dn17 + var_qs_nqs_dn17), (var_qse_dn18 + var_qs_nqs_dn18),)
    } else {
        (var_qd, var_qd_dn0, var_qd_dn2, var_qd_dn6, var_qd_dn7, var_qd_dn10, var_qd_dn11, var_qd_dn12, var_qd_dn13, var_qd_dn15, var_qd_dn16, var_qd_dn17, var_qd_dn18,)
    }
};
        var_qd = assign36810_e51396;
        var_qd_dn0 = assign36810_e51396_d_n0;
        var_qd_dn2 = assign36810_e51396_d_n2;
        var_qd_dn6 = assign36810_e51396_d_n6;
        var_qd_dn7 = assign36810_e51396_d_n7;
        var_qd_dn10 = assign36810_e51396_d_n10;
        var_qd_dn11 = assign36810_e51396_d_n11;
        var_qd_dn12 = assign36810_e51396_d_n12;
        var_qd_dn13 = assign36810_e51396_d_n13;
        var_qd_dn15 = assign36810_e51396_d_n15;
        var_qd_dn16 = assign36810_e51396_d_n16;
        var_qd_dn17 = assign36810_e51396_d_n17;
        var_qd_dn18 = assign36810_e51396_d_n18;
        var_qd_rv = 0.0;

        let (assign36830_e51413, assign36830_e51413_d_n0, assign36830_e51413_d_n2, assign36830_e51413_d_n6, assign36830_e51413_d_n7, assign36830_e51413_d_n10, assign36830_e51413_d_n11, assign36830_e51413_d_n12, assign36830_e51413_d_n13, assign36830_e51413_d_n15, assign36830_e51413_d_n16, assign36830_e51413_d_n17, assign36830_e51413_d_n18,) = {
    if (var_guard1212 == 0.0) {
        let assign36830_e51408: f64 = (var_qge + var_qde);
        let assign36830_e51410: f64 = (assign36830_e51408 + var_qse);
        let assign36830_e51411: f64 = (-assign36830_e51410);
        (assign36830_e51411, (-((var_qge_dn0 + var_qde_dn0) + var_qse_dn0)), (-((var_qge_dn2 + var_qde_dn2) + var_qse_dn2)), (-((var_qge_dn6 + var_qde_dn6) + var_qse_dn6)), (-((var_qge_dn7 + var_qde_dn7) + var_qse_dn7)), (-((var_qge_dn10 + var_qde_dn10) + var_qse_dn10)), (-((var_qge_dn11 + var_qde_dn11) + var_qse_dn11)), (-((var_qge_dn12 + var_qde_dn12) + var_qse_dn12)), (-((var_qge_dn13 + var_qde_dn13) + var_qse_dn13)), (-((var_qge_dn15 + var_qde_dn15) + var_qse_dn15)), (-((var_qge_dn16 + var_qde_dn16) + var_qse_dn16)), (-((var_qge_dn17 + var_qde_dn17) + var_qse_dn17)), (-((var_qge_dn18 + var_qde_dn18) + var_qse_dn18)),)
    } else {
        (var_qbe, var_qbe_dn0, var_qbe_dn2, var_qbe_dn6, var_qbe_dn7, var_qbe_dn10, var_qbe_dn11, var_qbe_dn12, var_qbe_dn13, var_qbe_dn15, var_qbe_dn16, var_qbe_dn17, var_qbe_dn18,)
    }
};
        var_qbe = assign36830_e51413;
        var_qbe_dn0 = assign36830_e51413_d_n0;
        var_qbe_dn2 = assign36830_e51413_d_n2;
        var_qbe_dn6 = assign36830_e51413_d_n6;
        var_qbe_dn7 = assign36830_e51413_d_n7;
        var_qbe_dn10 = assign36830_e51413_d_n10;
        var_qbe_dn11 = assign36830_e51413_d_n11;
        var_qbe_dn12 = assign36830_e51413_d_n12;
        var_qbe_dn13 = assign36830_e51413_d_n13;
        var_qbe_dn15 = assign36830_e51413_d_n15;
        var_qbe_dn16 = assign36830_e51413_d_n16;
        var_qbe_dn17 = assign36830_e51413_d_n17;
        var_qbe_dn18 = assign36830_e51413_d_n18;
        var_qbe_rv = 0.0;

        let (assign36840_e51420, assign36840_e51420_d_n0, assign36840_e51420_d_n2, assign36840_e51420_d_n6, assign36840_e51420_d_n7, assign36840_e51420_d_n10, assign36840_e51420_d_n11, assign36840_e51420_d_n12, assign36840_e51420_d_n13, assign36840_e51420_d_n15, assign36840_e51420_d_n16, assign36840_e51420_d_n17, assign36840_e51420_d_n18,) = {
    if (var_guard1212 == 0.0) {
        let assign36840_e51418: f64 = (var_qbe + var_qb_nqs);
        (assign36840_e51418, var_qbe_dn0, var_qbe_dn2, var_qbe_dn6, var_qbe_dn7, var_qbe_dn10, var_qbe_dn11, var_qbe_dn12, (var_qbe_dn13 + var_qb_nqs_dn13), var_qbe_dn15, var_qbe_dn16, var_qbe_dn17, var_qbe_dn18,)
    } else {
        (var_qb, var_qb_dn0, var_qb_dn2, var_qb_dn6, var_qb_dn7, var_qb_dn10, var_qb_dn11, var_qb_dn12, var_qb_dn13, var_qb_dn15, var_qb_dn16, var_qb_dn17, var_qb_dn18,)
    }
};
        var_qb = assign36840_e51420;
        var_qb_dn0 = assign36840_e51420_d_n0;
        var_qb_dn2 = assign36840_e51420_d_n2;
        var_qb_dn6 = assign36840_e51420_d_n6;
        var_qb_dn7 = assign36840_e51420_d_n7;
        var_qb_dn10 = assign36840_e51420_d_n10;
        var_qb_dn11 = assign36840_e51420_d_n11;
        var_qb_dn12 = assign36840_e51420_d_n12;
        var_qb_dn13 = assign36840_e51420_d_n13;
        var_qb_dn15 = assign36840_e51420_d_n15;
        var_qb_dn16 = assign36840_e51420_d_n16;
        var_qb_dn17 = assign36840_e51420_d_n17;
        var_qb_dn18 = assign36840_e51420_d_n18;
        var_qb_rv = 0.0;

        let assign36900_e51428: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard1213 = assign36900_e51428;
        var_guard1213_rv = 0.0;

        let (assign36910_e51432, assign36910_e51432_d_n0, assign36910_e51432_d_n2, assign36910_e51432_d_n6, assign36910_e51432_d_n7, assign36910_e51432_d_n10, assign36910_e51432_d_n11, assign36910_e51432_d_n12, assign36910_e51432_d_n17,) = {
    if (var_guard1213 != 0.0) {
        (var_ibdb, var_ibdb_dn0, var_ibdb_dn2, var_ibdb_dn6, var_ibdb_dn7, var_ibdb_dn10, var_ibdb_dn11, var_ibdb_dn12, var_ibdb_dn17,)
    } else {
        (var_ibd, var_ibd_dn0, var_ibd_dn2, var_ibd_dn6, var_ibd_dn7, var_ibd_dn10, var_ibd_dn11, var_ibd_dn12, var_ibd_dn17,)
    }
};
        var_ibd = assign36910_e51432;
        var_ibd_dn0 = assign36910_e51432_d_n0;
        var_ibd_dn2 = assign36910_e51432_d_n2;
        var_ibd_dn6 = assign36910_e51432_d_n6;
        var_ibd_dn7 = assign36910_e51432_d_n7;
        var_ibd_dn10 = assign36910_e51432_d_n10;
        var_ibd_dn11 = assign36910_e51432_d_n11;
        var_ibd_dn12 = assign36910_e51432_d_n12;
        var_ibd_dn17 = assign36910_e51432_d_n17;
        var_ibd_rv = 0.0;

        let (assign36920_e51436, assign36920_e51436_d_n0, assign36920_e51436_d_n2, assign36920_e51436_d_n6, assign36920_e51436_d_n7, assign36920_e51436_d_n10, assign36920_e51436_d_n11, assign36920_e51436_d_n12, assign36920_e51436_d_n17,) = {
    if (var_guard1213 != 0.0) {
        (var_qbd_s0, var_qbd_s0_dn0, var_qbd_s0_dn2, var_qbd_s0_dn6, var_qbd_s0_dn7, var_qbd_s0_dn10, var_qbd_s0_dn11, var_qbd_s0_dn12, var_qbd_s0_dn17,)
    } else {
        (var_qbd, var_qbd_dn0, var_qbd_dn2, var_qbd_dn6, var_qbd_dn7, var_qbd_dn10, var_qbd_dn11, var_qbd_dn12, var_qbd_dn17,)
    }
};
        var_qbd = assign36920_e51436;
        var_qbd_dn0 = assign36920_e51436_d_n0;
        var_qbd_dn2 = assign36920_e51436_d_n2;
        var_qbd_dn6 = assign36920_e51436_d_n6;
        var_qbd_dn7 = assign36920_e51436_d_n7;
        var_qbd_dn10 = assign36920_e51436_d_n10;
        var_qbd_dn11 = assign36920_e51436_d_n11;
        var_qbd_dn12 = assign36920_e51436_d_n12;
        var_qbd_dn17 = assign36920_e51436_d_n17;
        var_qbd_rv = 0.0;

        let (assign36930_e51440, assign36930_e51440_d_n0, assign36930_e51440_d_n2, assign36930_e51440_d_n6, assign36930_e51440_d_n7, assign36930_e51440_d_n10, assign36930_e51440_d_n11, assign36930_e51440_d_n12, assign36930_e51440_d_n17,) = {
    if (var_guard1213 != 0.0) {
        (var_ibsb, var_ibsb_dn0, var_ibsb_dn2, var_ibsb_dn6, var_ibsb_dn7, var_ibsb_dn10, var_ibsb_dn11, var_ibsb_dn12, var_ibsb_dn17,)
    } else {
        (var_ibs, var_ibs_dn0, var_ibs_dn2, var_ibs_dn6, var_ibs_dn7, var_ibs_dn10, var_ibs_dn11, var_ibs_dn12, var_ibs_dn17,)
    }
};
        var_ibs = assign36930_e51440;
        var_ibs_dn0 = assign36930_e51440_d_n0;
        var_ibs_dn2 = assign36930_e51440_d_n2;
        var_ibs_dn6 = assign36930_e51440_d_n6;
        var_ibs_dn7 = assign36930_e51440_d_n7;
        var_ibs_dn10 = assign36930_e51440_d_n10;
        var_ibs_dn11 = assign36930_e51440_d_n11;
        var_ibs_dn12 = assign36930_e51440_d_n12;
        var_ibs_dn17 = assign36930_e51440_d_n17;
        var_ibs_rv = 0.0;

        let (assign36940_e51444, assign36940_e51444_d_n0, assign36940_e51444_d_n2, assign36940_e51444_d_n6, assign36940_e51444_d_n7, assign36940_e51444_d_n10, assign36940_e51444_d_n11, assign36940_e51444_d_n12, assign36940_e51444_d_n17,) = {
    if (var_guard1213 != 0.0) {
        (var_qbs_s0, var_qbs_s0_dn0, var_qbs_s0_dn2, var_qbs_s0_dn6, var_qbs_s0_dn7, var_qbs_s0_dn10, var_qbs_s0_dn11, var_qbs_s0_dn12, var_qbs_s0_dn17,)
    } else {
        (var_qbs, var_qbs_dn0, var_qbs_dn2, var_qbs_dn6, var_qbs_dn7, var_qbs_dn10, var_qbs_dn11, var_qbs_dn12, var_qbs_dn17,)
    }
};
        var_qbs = assign36940_e51444;
        var_qbs_dn0 = assign36940_e51444_d_n0;
        var_qbs_dn2 = assign36940_e51444_d_n2;
        var_qbs_dn6 = assign36940_e51444_d_n6;
        var_qbs_dn7 = assign36940_e51444_d_n7;
        var_qbs_dn10 = assign36940_e51444_d_n10;
        var_qbs_dn11 = assign36940_e51444_d_n11;
        var_qbs_dn12 = assign36940_e51444_d_n12;
        var_qbs_dn17 = assign36940_e51444_d_n17;
        var_qbs_rv = 0.0;

        let assign36950_e51451: f64 = if ((p.p38 == 1.0) && (var_mks_rth0 > 0.0)) { 1.0 } else { 0.0 };
        var_guard1214 = assign36950_e51451;
        var_guard1214_rv = 0.0;

        let (assign36970_e51461,) = {
    if (var_guard1214 != 0.0) {
        (var_cth,)
    } else {
        (var_cthe,)
    }
};
        var_cthe = assign36970_e51461;
        var_cthe_rv = 0.0;

        let (assign37000_e51477,) = {
    if (var_guard1214 == 0.0) {
        (0.0,)
    } else {
        (var_cthe,)
    }
};
        var_cthe = assign37000_e51477;
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

        let assign37170_e51531: f64 = var_qg_dn6;
        var_cgdbd = assign37170_e51531;
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

        let assign37180_e51534: f64 = (p.p50 * var_cgdbd);
        var_cgdbd = assign37180_e51534;
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

        let assign37190_e51537: f64 = var_qg_dn7;
        var_cgsbd = assign37190_e51537;
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

        let assign37200_e51540: f64 = (p.p50 * var_cgsbd);
        var_cgsbd = assign37200_e51540;
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

        let assign37470_e51621: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard1216 = assign37470_e51621;
        var_guard1216_rv = 0.0;

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
        *var_guard1212_slot = var_guard1212;
        *var_guard1212_rv_slot = var_guard1212_rv;
        *var_guard1213_slot = var_guard1213;
        *var_guard1213_rv_slot = var_guard1213_rv;
        *var_guard1214_slot = var_guard1214;
        *var_guard1214_rv_slot = var_guard1214_rv;
        *var_guard1216_slot = var_guard1216;
        *var_guard1216_rv_slot = var_guard1216_rv;
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
        var_guard1216: f64,
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
        var_guard1224_slot: &mut f64,
        var_guard1224_rv_slot: &mut f64,
        var_guard1225_slot: &mut f64,
        var_guard1225_rv_slot: &mut f64,
        var_guard1226_slot: &mut f64,
        var_guard1226_rv_slot: &mut f64,
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
        let mut var_guard1224: f64 = *var_guard1224_slot;
        let mut var_guard1224_rv: f64 = *var_guard1224_rv_slot;
        let mut var_guard1225: f64 = *var_guard1225_slot;
        let mut var_guard1225_rv: f64 = *var_guard1225_rv_slot;
        let mut var_guard1226: f64 = *var_guard1226_slot;
        let mut var_guard1226_rv: f64 = *var_guard1226_rv_slot;
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

        let (assign37480_e51627, assign37480_e51627_d_n0, assign37480_e51627_d_n2, assign37480_e51627_d_n6, assign37480_e51627_d_n7, assign37480_e51627_d_n10, assign37480_e51627_d_n11, assign37480_e51627_d_n12, assign37480_e51627_d_n17,) = {
    if (var_guard1216 != 0.0) {
        let assign37480_e51625: f64 = (p.p50 * var_ibd);
        (assign37480_e51625, (p.p50 * var_ibd_dn0), (p.p50 * var_ibd_dn2), (p.p50 * var_ibd_dn6), (p.p50 * var_ibd_dn7), (p.p50 * var_ibd_dn10), (p.p50 * var_ibd_dn11), (p.p50 * var_ibd_dn12), (p.p50 * var_ibd_dn17),)
    } else {
        (var_ibdb, var_ibdb_dn0, var_ibdb_dn2, var_ibdb_dn6, var_ibdb_dn7, var_ibdb_dn10, var_ibdb_dn11, var_ibdb_dn12, var_ibdb_dn17,)
    }
};
        var_ibdb = assign37480_e51627;
        var_ibdb_dn0 = assign37480_e51627_d_n0;
        var_ibdb_dn2 = assign37480_e51627_d_n2;
        var_ibdb_dn6 = assign37480_e51627_d_n6;
        var_ibdb_dn7 = assign37480_e51627_d_n7;
        var_ibdb_dn10 = assign37480_e51627_d_n10;
        var_ibdb_dn11 = assign37480_e51627_d_n11;
        var_ibdb_dn12 = assign37480_e51627_d_n12;
        var_ibdb_dn17 = assign37480_e51627_d_n17;
        var_ibdb_rv = 0.0;

        let (assign37490_e51633, assign37490_e51633_d_n0, assign37490_e51633_d_n2, assign37490_e51633_d_n6, assign37490_e51633_d_n7, assign37490_e51633_d_n10, assign37490_e51633_d_n11, assign37490_e51633_d_n12, assign37490_e51633_d_n17,) = {
    if (var_guard1216 != 0.0) {
        let assign37490_e51631: f64 = (p.p50 * var_ibs);
        (assign37490_e51631, (p.p50 * var_ibs_dn0), (p.p50 * var_ibs_dn2), (p.p50 * var_ibs_dn6), (p.p50 * var_ibs_dn7), (p.p50 * var_ibs_dn10), (p.p50 * var_ibs_dn11), (p.p50 * var_ibs_dn12), (p.p50 * var_ibs_dn17),)
    } else {
        (var_ibsb, var_ibsb_dn0, var_ibsb_dn2, var_ibsb_dn6, var_ibsb_dn7, var_ibsb_dn10, var_ibsb_dn11, var_ibsb_dn12, var_ibsb_dn17,)
    }
};
        var_ibsb = assign37490_e51633;
        var_ibsb_dn0 = assign37490_e51633_d_n0;
        var_ibsb_dn2 = assign37490_e51633_d_n2;
        var_ibsb_dn6 = assign37490_e51633_d_n6;
        var_ibsb_dn7 = assign37490_e51633_d_n7;
        var_ibsb_dn10 = assign37490_e51633_d_n10;
        var_ibsb_dn11 = assign37490_e51633_d_n11;
        var_ibsb_dn12 = assign37490_e51633_d_n12;
        var_ibsb_dn17 = assign37490_e51633_d_n17;
        var_ibsb_rv = 0.0;

        let assign37610_e51685: f64 = (4.0 * 1.3806226e-23);
        let assign37610_e51687: f64 = (assign37610_e51685 * var_ttemp);
        let assign37610_e51689: f64 = assign37610_e51687;
        var_whi_noise = assign37610_e51689;
        var_whi_noise_dn10 = (assign37610_e51685 * var_ttemp_dn10);
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

        let assign37640_e51696: f64 = (var_whi_noise * var_noithrml);
        var_sid = assign37640_e51696;
        var_sid_dn0 = (var_whi_noise * var_noithrml_dn0);
        var_sid_dn2 = (var_whi_noise * var_noithrml_dn2);
        var_sid_dn6 = (var_whi_noise * var_noithrml_dn6);
        var_sid_dn7 = (var_whi_noise * var_noithrml_dn7);
        var_sid_dn10 = ((var_whi_noise_dn10 * var_noithrml) + (var_whi_noise * var_noithrml_dn10));
        var_sid_dn11 = (var_whi_noise * var_noithrml_dn11);
        var_sid_dn12 = (var_whi_noise * var_noithrml_dn12);
        var_sid_dn17 = (var_whi_noise * var_noithrml_dn17);
        var_sid_rv = 0.0;

        let (assign37660_e51710, assign37660_e51710_d_n0, assign37660_e51710_d_n2, assign37660_e51710_d_n6, assign37660_e51710_d_n7, assign37660_e51710_d_n10, assign37660_e51710_d_n11, assign37660_e51710_d_n12, assign37660_e51710_d_n13, assign37660_e51710_d_n15, assign37660_e51710_d_n16, assign37660_e51710_d_n17, assign37660_e51710_d_n18,) = {
    if ((var_sid > 0.0) && (var_noiigate > 0.0)) {
        let assign37660_e51707: f64 = (var_noiigate / var_sid);
        let assign37660_e51708: f64 = (assign37660_e51707).sqrt();
        (assign37660_e51708, ((((var_noiigate_dn0 * var_sid) - (var_noiigate * var_sid_dn0)) / (var_sid * var_sid)) / (2.0 * assign37660_e51708)), ((((var_noiigate_dn2 * var_sid) - (var_noiigate * var_sid_dn2)) / (var_sid * var_sid)) / (2.0 * assign37660_e51708)), ((((var_noiigate_dn6 * var_sid) - (var_noiigate * var_sid_dn6)) / (var_sid * var_sid)) / (2.0 * assign37660_e51708)), ((((var_noiigate_dn7 * var_sid) - (var_noiigate * var_sid_dn7)) / (var_sid * var_sid)) / (2.0 * assign37660_e51708)), ((((var_noiigate_dn10 * var_sid) - (var_noiigate * var_sid_dn10)) / (var_sid * var_sid)) / (2.0 * assign37660_e51708)), ((((var_noiigate_dn11 * var_sid) - (var_noiigate * var_sid_dn11)) / (var_sid * var_sid)) / (2.0 * assign37660_e51708)), ((((var_noiigate_dn12 * var_sid) - (var_noiigate * var_sid_dn12)) / (var_sid * var_sid)) / (2.0 * assign37660_e51708)), ((var_noiigate_dn13 / var_sid) / (2.0 * assign37660_e51708)), ((var_noiigate_dn15 / var_sid) / (2.0 * assign37660_e51708)), ((var_noiigate_dn16 / var_sid) / (2.0 * assign37660_e51708)), ((((var_noiigate_dn17 * var_sid) - (var_noiigate * var_sid_dn17)) / (var_sid * var_sid)) / (2.0 * assign37660_e51708)), ((var_noiigate_dn18 / var_sid) / (2.0 * assign37660_e51708)),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        var_sigrat = assign37660_e51710;
        var_sigrat_dn0 = assign37660_e51710_d_n0;
        var_sigrat_dn2 = assign37660_e51710_d_n2;
        var_sigrat_dn6 = assign37660_e51710_d_n6;
        var_sigrat_dn7 = assign37660_e51710_d_n7;
        var_sigrat_dn10 = assign37660_e51710_d_n10;
        var_sigrat_dn11 = assign37660_e51710_d_n11;
        var_sigrat_dn12 = assign37660_e51710_d_n12;
        var_sigrat_dn13 = assign37660_e51710_d_n13;
        var_sigrat_dn15 = assign37660_e51710_d_n15;
        var_sigrat_dn16 = assign37660_e51710_d_n16;
        var_sigrat_dn17 = assign37660_e51710_d_n17;
        var_sigrat_dn18 = assign37660_e51710_d_n18;
        var_sigrat_rv = 0.0;

        let (assign37670_e51722, assign37670_e51722_d_n0, assign37670_e51722_d_n2, assign37670_e51722_d_n6, assign37670_e51722_d_n7, assign37670_e51722_d_n10, assign37670_e51722_d_n11, assign37670_e51722_d_n12, assign37670_e51722_d_n13, assign37670_e51722_d_n15, assign37670_e51722_d_n16, assign37670_e51722_d_n17, assign37670_e51722_d_n18,) = {
    if (var_mode > 0.0) {
        let assign37670_e51717: f64 = (1.0 - var_qdrat);
        let assign37670_e51718: f64 = (var_sigrat * assign37670_e51717);
        (assign37670_e51718, ((var_sigrat_dn0 * assign37670_e51717) + (var_sigrat * (-var_qdrat_dn0))), ((var_sigrat_dn2 * assign37670_e51717) + (var_sigrat * (-var_qdrat_dn2))), ((var_sigrat_dn6 * assign37670_e51717) + (var_sigrat * (-var_qdrat_dn6))), ((var_sigrat_dn7 * assign37670_e51717) + (var_sigrat * (-var_qdrat_dn7))), ((var_sigrat_dn10 * assign37670_e51717) + (var_sigrat * (-var_qdrat_dn10))), ((var_sigrat_dn11 * assign37670_e51717) + (var_sigrat * (-var_qdrat_dn11))), ((var_sigrat_dn12 * assign37670_e51717) + (var_sigrat * (-var_qdrat_dn12))), (var_sigrat_dn13 * assign37670_e51717), (var_sigrat_dn15 * assign37670_e51717), (var_sigrat_dn16 * assign37670_e51717), ((var_sigrat_dn17 * assign37670_e51717) + (var_sigrat * (-var_qdrat_dn17))), (var_sigrat_dn18 * assign37670_e51717),)
    } else {
        let assign37670_e51721: f64 = (var_sigrat * var_qdrat);
        (assign37670_e51721, ((var_sigrat_dn0 * var_qdrat) + (var_sigrat * var_qdrat_dn0)), ((var_sigrat_dn2 * var_qdrat) + (var_sigrat * var_qdrat_dn2)), ((var_sigrat_dn6 * var_qdrat) + (var_sigrat * var_qdrat_dn6)), ((var_sigrat_dn7 * var_qdrat) + (var_sigrat * var_qdrat_dn7)), ((var_sigrat_dn10 * var_qdrat) + (var_sigrat * var_qdrat_dn10)), ((var_sigrat_dn11 * var_qdrat) + (var_sigrat * var_qdrat_dn11)), ((var_sigrat_dn12 * var_qdrat) + (var_sigrat * var_qdrat_dn12)), (var_sigrat_dn13 * var_qdrat), (var_sigrat_dn15 * var_qdrat), (var_sigrat_dn16 * var_qdrat), ((var_sigrat_dn17 * var_qdrat) + (var_sigrat * var_qdrat_dn17)), (var_sigrat_dn18 * var_qdrat),)
    }
};
        var_sigrat_s = assign37670_e51722;
        var_sigrat_s_dn0 = assign37670_e51722_d_n0;
        var_sigrat_s_dn2 = assign37670_e51722_d_n2;
        var_sigrat_s_dn6 = assign37670_e51722_d_n6;
        var_sigrat_s_dn7 = assign37670_e51722_d_n7;
        var_sigrat_s_dn10 = assign37670_e51722_d_n10;
        var_sigrat_s_dn11 = assign37670_e51722_d_n11;
        var_sigrat_s_dn12 = assign37670_e51722_d_n12;
        var_sigrat_s_dn13 = assign37670_e51722_d_n13;
        var_sigrat_s_dn15 = assign37670_e51722_d_n15;
        var_sigrat_s_dn16 = assign37670_e51722_d_n16;
        var_sigrat_s_dn17 = assign37670_e51722_d_n17;
        var_sigrat_s_dn18 = assign37670_e51722_d_n18;
        var_sigrat_s_rv = 0.0;

        let (assign37680_e51734, assign37680_e51734_d_n0, assign37680_e51734_d_n2, assign37680_e51734_d_n6, assign37680_e51734_d_n7, assign37680_e51734_d_n10, assign37680_e51734_d_n11, assign37680_e51734_d_n12, assign37680_e51734_d_n13, assign37680_e51734_d_n15, assign37680_e51734_d_n16, assign37680_e51734_d_n17, assign37680_e51734_d_n18,) = {
    if (var_mode > 0.0) {
        let assign37680_e51728: f64 = (var_sigrat * var_qdrat);
        (assign37680_e51728, ((var_sigrat_dn0 * var_qdrat) + (var_sigrat * var_qdrat_dn0)), ((var_sigrat_dn2 * var_qdrat) + (var_sigrat * var_qdrat_dn2)), ((var_sigrat_dn6 * var_qdrat) + (var_sigrat * var_qdrat_dn6)), ((var_sigrat_dn7 * var_qdrat) + (var_sigrat * var_qdrat_dn7)), ((var_sigrat_dn10 * var_qdrat) + (var_sigrat * var_qdrat_dn10)), ((var_sigrat_dn11 * var_qdrat) + (var_sigrat * var_qdrat_dn11)), ((var_sigrat_dn12 * var_qdrat) + (var_sigrat * var_qdrat_dn12)), (var_sigrat_dn13 * var_qdrat), (var_sigrat_dn15 * var_qdrat), (var_sigrat_dn16 * var_qdrat), ((var_sigrat_dn17 * var_qdrat) + (var_sigrat * var_qdrat_dn17)), (var_sigrat_dn18 * var_qdrat),)
    } else {
        let assign37680_e51732: f64 = (1.0 - var_qdrat);
        let assign37680_e51733: f64 = (var_sigrat * assign37680_e51732);
        (assign37680_e51733, ((var_sigrat_dn0 * assign37680_e51732) + (var_sigrat * (-var_qdrat_dn0))), ((var_sigrat_dn2 * assign37680_e51732) + (var_sigrat * (-var_qdrat_dn2))), ((var_sigrat_dn6 * assign37680_e51732) + (var_sigrat * (-var_qdrat_dn6))), ((var_sigrat_dn7 * assign37680_e51732) + (var_sigrat * (-var_qdrat_dn7))), ((var_sigrat_dn10 * assign37680_e51732) + (var_sigrat * (-var_qdrat_dn10))), ((var_sigrat_dn11 * assign37680_e51732) + (var_sigrat * (-var_qdrat_dn11))), ((var_sigrat_dn12 * assign37680_e51732) + (var_sigrat * (-var_qdrat_dn12))), (var_sigrat_dn13 * assign37680_e51732), (var_sigrat_dn15 * assign37680_e51732), (var_sigrat_dn16 * assign37680_e51732), ((var_sigrat_dn17 * assign37680_e51732) + (var_sigrat * (-var_qdrat_dn17))), (var_sigrat_dn18 * assign37680_e51732),)
    }
};
        var_sigrat_d = assign37680_e51734;
        var_sigrat_d_dn0 = assign37680_e51734_d_n0;
        var_sigrat_d_dn2 = assign37680_e51734_d_n2;
        var_sigrat_d_dn6 = assign37680_e51734_d_n6;
        var_sigrat_d_dn7 = assign37680_e51734_d_n7;
        var_sigrat_d_dn10 = assign37680_e51734_d_n10;
        var_sigrat_d_dn11 = assign37680_e51734_d_n11;
        var_sigrat_d_dn12 = assign37680_e51734_d_n12;
        var_sigrat_d_dn13 = assign37680_e51734_d_n13;
        var_sigrat_d_dn15 = assign37680_e51734_d_n15;
        var_sigrat_d_dn16 = assign37680_e51734_d_n16;
        var_sigrat_d_dn17 = assign37680_e51734_d_n17;
        var_sigrat_d_dn18 = assign37680_e51734_d_n18;
        var_sigrat_d_rv = 0.0;

        let assign37700_e51744: f64 = if ((p.p38 > 0.0) && (p.p242 > 0.0)) { 1.0 } else { 0.0 };
        var_guard1224 = assign37700_e51744;
        var_guard1224_rv = 0.0;

        let assign37720_e51751: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard1225 = assign37720_e51751;
        var_guard1225_rv = 0.0;

        let assign37730_e51760: f64 = if ((p.p37 != 0.0) || ((p.p25 == 1.0) && (p.p26 == 2.0))) { 1.0 } else { 0.0 };
        var_guard1226 = assign37730_e51760;
        var_guard1226_rv = 0.0;

        *var_guard1224_slot = var_guard1224;
        *var_guard1224_rv_slot = var_guard1224_rv;
        *var_guard1225_slot = var_guard1225;
        *var_guard1225_rv_slot = var_guard1225_rv;
        *var_guard1226_slot = var_guard1226;
        *var_guard1226_rv_slot = var_guard1226_rv;
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
        var_guard1222: f64,
        var_guard1224: f64,
        var_guard1225: f64,
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
        let eq2_e316: f64 = (p.p50 * var_ids);
        let eq2_e316_d_n0: f64 = (p.p50 * var_ids_dn0);
        let eq2_e316_d_n2: f64 = (p.p50 * var_ids_dn2);
        let eq2_e316_d_n6: f64 = (p.p50 * var_ids_dn6);
        let eq2_e316_d_n7: f64 = (p.p50 * var_ids_dn7);
        let eq2_e316_d_n10: f64 = (p.p50 * var_ids_dn10);
        let eq2_e316_d_n11: f64 = (p.p50 * var_ids_dn11);
        let eq2_e316_d_n12: f64 = (p.p50 * var_ids_dn12);
        let eq2_e316_d_n17: f64 = (p.p50 * var_ids_dn17);
        let eq2_value: f64 = eq2_e316;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq2_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq2_e316_d_n0), multiplicity * (eq2_e316_d_n2), multiplicity * (eq2_e316_d_n6), multiplicity * (eq2_e316_d_n7), multiplicity * (eq2_e316_d_n10), multiplicity * (eq2_e316_d_n11), multiplicity * (eq2_e316_d_n12), multiplicity * (eq2_e316_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq3_e322, eq3_e322_d_n0, eq3_e322_d_n2, eq3_e322_d_n6, eq3_e322_d_n7, eq3_e322_d_n10, eq3_e322_d_n11, eq3_e322_d_n12, eq3_e322_d_n17,) = {
    if (var_guard1222 != 0.0) {
        let eq3_e320: f64 = (p.p50 * var_igs);
        let eq3_e320_d_n0: f64 = (p.p50 * var_igs_dn0);
        let eq3_e320_d_n2: f64 = (p.p50 * var_igs_dn2);
        let eq3_e320_d_n6: f64 = (p.p50 * var_igs_dn6);
        let eq3_e320_d_n7: f64 = (p.p50 * var_igs_dn7);
        let eq3_e320_d_n10: f64 = (p.p50 * var_igs_dn10);
        let eq3_e320_d_n11: f64 = (p.p50 * var_igs_dn11);
        let eq3_e320_d_n12: f64 = (p.p50 * var_igs_dn12);
        let eq3_e320_d_n17: f64 = (p.p50 * var_igs_dn17);
        (eq3_e320, eq3_e320_d_n0, eq3_e320_d_n2, eq3_e320_d_n6, eq3_e320_d_n7, eq3_e320_d_n10, eq3_e320_d_n11, eq3_e320_d_n12, eq3_e320_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e322;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(11),
            Some(7),
            multiplicity * (eq3_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq3_e322_d_n0), multiplicity * (eq3_e322_d_n2), multiplicity * (eq3_e322_d_n6), multiplicity * (eq3_e322_d_n7), multiplicity * (eq3_e322_d_n10), multiplicity * (eq3_e322_d_n11), multiplicity * (eq3_e322_d_n12), multiplicity * (eq3_e322_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq4_e328, eq4_e328_d_n0, eq4_e328_d_n2, eq4_e328_d_n6, eq4_e328_d_n7, eq4_e328_d_n10, eq4_e328_d_n11, eq4_e328_d_n12, eq4_e328_d_n17,) = {
    if (var_guard1222 != 0.0) {
        let eq4_e326: f64 = (p.p50 * var_igd);
        let eq4_e326_d_n0: f64 = (p.p50 * var_igd_dn0);
        let eq4_e326_d_n2: f64 = (p.p50 * var_igd_dn2);
        let eq4_e326_d_n6: f64 = (p.p50 * var_igd_dn6);
        let eq4_e326_d_n7: f64 = (p.p50 * var_igd_dn7);
        let eq4_e326_d_n10: f64 = (p.p50 * var_igd_dn10);
        let eq4_e326_d_n11: f64 = (p.p50 * var_igd_dn11);
        let eq4_e326_d_n12: f64 = (p.p50 * var_igd_dn12);
        let eq4_e326_d_n17: f64 = (p.p50 * var_igd_dn17);
        (eq4_e326, eq4_e326_d_n0, eq4_e326_d_n2, eq4_e326_d_n6, eq4_e326_d_n7, eq4_e326_d_n10, eq4_e326_d_n11, eq4_e326_d_n12, eq4_e326_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e328;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(11),
            Some(6),
            multiplicity * (eq4_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq4_e328_d_n0), multiplicity * (eq4_e328_d_n2), multiplicity * (eq4_e328_d_n6), multiplicity * (eq4_e328_d_n7), multiplicity * (eq4_e328_d_n10), multiplicity * (eq4_e328_d_n11), multiplicity * (eq4_e328_d_n12), multiplicity * (eq4_e328_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq5_e334, eq5_e334_d_n0, eq5_e334_d_n2, eq5_e334_d_n6, eq5_e334_d_n7, eq5_e334_d_n10, eq5_e334_d_n11, eq5_e334_d_n12, eq5_e334_d_n17,) = {
    if (var_guard1222 != 0.0) {
        let eq5_e332: f64 = (p.p50 * var_igb);
        let eq5_e332_d_n0: f64 = (p.p50 * var_igb_dn0);
        let eq5_e332_d_n2: f64 = (p.p50 * var_igb_dn2);
        let eq5_e332_d_n6: f64 = (p.p50 * var_igb_dn6);
        let eq5_e332_d_n7: f64 = (p.p50 * var_igb_dn7);
        let eq5_e332_d_n10: f64 = (p.p50 * var_igb_dn10);
        let eq5_e332_d_n11: f64 = (p.p50 * var_igb_dn11);
        let eq5_e332_d_n12: f64 = (p.p50 * var_igb_dn12);
        let eq5_e332_d_n17: f64 = (p.p50 * var_igb_dn17);
        (eq5_e332, eq5_e332_d_n0, eq5_e332_d_n2, eq5_e332_d_n6, eq5_e332_d_n7, eq5_e332_d_n10, eq5_e332_d_n11, eq5_e332_d_n12, eq5_e332_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e334;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(11),
            Some(12),
            multiplicity * (eq5_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq5_e334_d_n0), multiplicity * (eq5_e334_d_n2), multiplicity * (eq5_e334_d_n6), multiplicity * (eq5_e334_d_n7), multiplicity * (eq5_e334_d_n10), multiplicity * (eq5_e334_d_n11), multiplicity * (eq5_e334_d_n12), multiplicity * (eq5_e334_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq6_e340, eq6_e340_d_n0, eq6_e340_d_n2, eq6_e340_d_n6, eq6_e340_d_n7, eq6_e340_d_n10, eq6_e340_d_n11, eq6_e340_d_n12, eq6_e340_d_n17,) = {
    if (p.p259 != 0.0) {
        let eq6_e338: f64 = ((nv7 - nv2) / var_rsd);
        let eq6_e338_d_n0: f64 = (-(((nv7 - nv2) * var_rsd_dn0) / (var_rsd * var_rsd)));
        let eq6_e338_d_n2: f64 = (((-var_rsd) - ((nv7 - nv2) * var_rsd_dn2)) / (var_rsd * var_rsd));
        let eq6_e338_d_n6: f64 = (-(((nv7 - nv2) * var_rsd_dn6) / (var_rsd * var_rsd)));
        let eq6_e338_d_n7: f64 = ((var_rsd - ((nv7 - nv2) * var_rsd_dn7)) / (var_rsd * var_rsd));
        let eq6_e338_d_n10: f64 = (-(((nv7 - nv2) * var_rsd_dn10) / (var_rsd * var_rsd)));
        let eq6_e338_d_n11: f64 = (-(((nv7 - nv2) * var_rsd_dn11) / (var_rsd * var_rsd)));
        let eq6_e338_d_n12: f64 = (-(((nv7 - nv2) * var_rsd_dn12) / (var_rsd * var_rsd)));
        let eq6_e338_d_n17: f64 = (-(((nv7 - nv2) * var_rsd_dn17) / (var_rsd * var_rsd)));
        (eq6_e338, eq6_e338_d_n0, eq6_e338_d_n2, eq6_e338_d_n6, eq6_e338_d_n7, eq6_e338_d_n10, eq6_e338_d_n11, eq6_e338_d_n12, eq6_e338_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e340;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(7),
            Some(2),
            multiplicity * (eq6_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq6_e340_d_n0), multiplicity * (eq6_e340_d_n2), multiplicity * (eq6_e340_d_n6), multiplicity * (eq6_e340_d_n7), multiplicity * (eq6_e340_d_n10), multiplicity * (eq6_e340_d_n11), multiplicity * (eq6_e340_d_n12), multiplicity * (eq6_e340_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq8_e351, eq8_e351_d_n0, eq8_e351_d_n2, eq8_e351_d_n6, eq8_e351_d_n7, eq8_e351_d_n10, eq8_e351_d_n11, eq8_e351_d_n12, eq8_e351_d_n17,) = {
    if (p.p260 != 0.0) {
        let eq8_e349: f64 = ((nv0 - nv6) / var_rdd);
        let eq8_e349_d_n0: f64 = ((var_rdd - ((nv0 - nv6) * var_rdd_dn0)) / (var_rdd * var_rdd));
        let eq8_e349_d_n2: f64 = (-(((nv0 - nv6) * var_rdd_dn2) / (var_rdd * var_rdd)));
        let eq8_e349_d_n6: f64 = (((-var_rdd) - ((nv0 - nv6) * var_rdd_dn6)) / (var_rdd * var_rdd));
        let eq8_e349_d_n7: f64 = (-(((nv0 - nv6) * var_rdd_dn7) / (var_rdd * var_rdd)));
        let eq8_e349_d_n10: f64 = (-(((nv0 - nv6) * var_rdd_dn10) / (var_rdd * var_rdd)));
        let eq8_e349_d_n11: f64 = (-(((nv0 - nv6) * var_rdd_dn11) / (var_rdd * var_rdd)));
        let eq8_e349_d_n12: f64 = (-(((nv0 - nv6) * var_rdd_dn12) / (var_rdd * var_rdd)));
        let eq8_e349_d_n17: f64 = (-(((nv0 - nv6) * var_rdd_dn17) / (var_rdd * var_rdd)));
        (eq8_e349, eq8_e349_d_n0, eq8_e349_d_n2, eq8_e349_d_n6, eq8_e349_d_n7, eq8_e349_d_n10, eq8_e349_d_n11, eq8_e349_d_n12, eq8_e349_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e351;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(0),
            Some(6),
            multiplicity * (eq8_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq8_e351_d_n0), multiplicity * (eq8_e351_d_n2), multiplicity * (eq8_e351_d_n6), multiplicity * (eq8_e351_d_n7), multiplicity * (eq8_e351_d_n10), multiplicity * (eq8_e351_d_n11), multiplicity * (eq8_e351_d_n12), multiplicity * (eq8_e351_d_n17)],
            [],
            [],
            1.0,
        );
        let eq10_e359: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, var_qg);
        let eq10_e360: f64 = (p.p50 * eq10_e359);
        let eq10_e360_d_n0: f64 = (p.p50 * (var_qg_dn0 * ddt_scale));
        let eq10_e360_d_n2: f64 = (p.p50 * (var_qg_dn2 * ddt_scale));
        let eq10_e360_d_n6: f64 = (p.p50 * (var_qg_dn6 * ddt_scale));
        let eq10_e360_d_n7: f64 = (p.p50 * (var_qg_dn7 * ddt_scale));
        let eq10_e360_d_n10: f64 = (p.p50 * (var_qg_dn10 * ddt_scale));
        let eq10_e360_d_n11: f64 = (p.p50 * (var_qg_dn11 * ddt_scale));
        let eq10_e360_d_n12: f64 = (p.p50 * (var_qg_dn12 * ddt_scale));
        let eq10_e360_d_n13: f64 = (p.p50 * (var_qg_dn13 * ddt_scale));
        let eq10_e360_d_n15: f64 = (p.p50 * (var_qg_dn15 * ddt_scale));
        let eq10_e360_d_n16: f64 = (p.p50 * (var_qg_dn16 * ddt_scale));
        let eq10_e360_d_n17: f64 = (p.p50 * (var_qg_dn17 * ddt_scale));
        let eq10_e360_d_n18: f64 = (p.p50 * (var_qg_dn18 * ddt_scale));
        let eq10_value: f64 = eq10_e360;
        let eq10_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq10_node_derivatives: [f64; 12] = [eq10_e360_d_n0, eq10_e360_d_n2, eq10_e360_d_n6, eq10_e360_d_n7, eq10_e360_d_n10, eq10_e360_d_n11, eq10_e360_d_n12, eq10_e360_d_n13, eq10_e360_d_n15, eq10_e360_d_n16, eq10_e360_d_n17, eq10_e360_d_n18];
        let eq10_branch_derivative_indices: [usize; 0] = [];
        let eq10_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq10_value),
            &eq10_node_derivative_indices,
            &eq10_node_derivatives,
            &eq10_branch_derivative_indices,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let eq11_e363: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, var_qd);
        let eq11_e364: f64 = (p.p50 * eq11_e363);
        let eq11_e364_d_n0: f64 = (p.p50 * (var_qd_dn0 * ddt_scale));
        let eq11_e364_d_n2: f64 = (p.p50 * (var_qd_dn2 * ddt_scale));
        let eq11_e364_d_n6: f64 = (p.p50 * (var_qd_dn6 * ddt_scale));
        let eq11_e364_d_n7: f64 = (p.p50 * (var_qd_dn7 * ddt_scale));
        let eq11_e364_d_n10: f64 = (p.p50 * (var_qd_dn10 * ddt_scale));
        let eq11_e364_d_n11: f64 = (p.p50 * (var_qd_dn11 * ddt_scale));
        let eq11_e364_d_n12: f64 = (p.p50 * (var_qd_dn12 * ddt_scale));
        let eq11_e364_d_n13: f64 = (p.p50 * (var_qd_dn13 * ddt_scale));
        let eq11_e364_d_n15: f64 = (p.p50 * (var_qd_dn15 * ddt_scale));
        let eq11_e364_d_n16: f64 = (p.p50 * (var_qd_dn16 * ddt_scale));
        let eq11_e364_d_n17: f64 = (p.p50 * (var_qd_dn17 * ddt_scale));
        let eq11_e364_d_n18: f64 = (p.p50 * (var_qd_dn18 * ddt_scale));
        let eq11_value: f64 = eq11_e364;
        let eq11_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq11_node_derivatives: [f64; 12] = [eq11_e364_d_n0, eq11_e364_d_n2, eq11_e364_d_n6, eq11_e364_d_n7, eq11_e364_d_n10, eq11_e364_d_n11, eq11_e364_d_n12, eq11_e364_d_n13, eq11_e364_d_n15, eq11_e364_d_n16, eq11_e364_d_n17, eq11_e364_d_n18];
        let eq11_branch_derivative_indices: [usize; 0] = [];
        let eq11_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq11_value),
            &eq11_node_derivative_indices,
            &eq11_node_derivatives,
            &eq11_branch_derivative_indices,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let eq12_e367: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, var_qb);
        let eq12_e368: f64 = (p.p50 * eq12_e367);
        let eq12_e368_d_n0: f64 = (p.p50 * (var_qb_dn0 * ddt_scale));
        let eq12_e368_d_n2: f64 = (p.p50 * (var_qb_dn2 * ddt_scale));
        let eq12_e368_d_n6: f64 = (p.p50 * (var_qb_dn6 * ddt_scale));
        let eq12_e368_d_n7: f64 = (p.p50 * (var_qb_dn7 * ddt_scale));
        let eq12_e368_d_n10: f64 = (p.p50 * (var_qb_dn10 * ddt_scale));
        let eq12_e368_d_n11: f64 = (p.p50 * (var_qb_dn11 * ddt_scale));
        let eq12_e368_d_n12: f64 = (p.p50 * (var_qb_dn12 * ddt_scale));
        let eq12_e368_d_n13: f64 = (p.p50 * (var_qb_dn13 * ddt_scale));
        let eq12_e368_d_n15: f64 = (p.p50 * (var_qb_dn15 * ddt_scale));
        let eq12_e368_d_n16: f64 = (p.p50 * (var_qb_dn16 * ddt_scale));
        let eq12_e368_d_n17: f64 = (p.p50 * (var_qb_dn17 * ddt_scale));
        let eq12_e368_d_n18: f64 = (p.p50 * (var_qb_dn18 * ddt_scale));
        let eq12_value: f64 = eq12_e368;
        let eq12_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq12_node_derivatives: [f64; 12] = [eq12_e368_d_n0, eq12_e368_d_n2, eq12_e368_d_n6, eq12_e368_d_n7, eq12_e368_d_n10, eq12_e368_d_n11, eq12_e368_d_n12, eq12_e368_d_n13, eq12_e368_d_n15, eq12_e368_d_n16, eq12_e368_d_n17, eq12_e368_d_n18];
        let eq12_branch_derivative_indices: [usize; 0] = [];
        let eq12_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(12),
            Some(7),
            multiplicity * (eq12_value),
            &eq12_node_derivative_indices,
            &eq12_node_derivatives,
            &eq12_branch_derivative_indices,
            &eq12_branch_derivatives,
            multiplicity,
        );
        let eq17_e394: f64 = (var_ci * (nv14 - 0.0));
        let eq17_e394_d_n0: f64 = (var_ci_dn0 * (nv14 - 0.0));
        let eq17_e394_d_n2: f64 = (var_ci_dn2 * (nv14 - 0.0));
        let eq17_e394_d_n6: f64 = (var_ci_dn6 * (nv14 - 0.0));
        let eq17_e394_d_n7: f64 = (var_ci_dn7 * (nv14 - 0.0));
        let eq17_e394_d_n10: f64 = (var_ci_dn10 * (nv14 - 0.0));
        let eq17_e394_d_n11: f64 = (var_ci_dn11 * (nv14 - 0.0));
        let eq17_e394_d_n12: f64 = (var_ci_dn12 * (nv14 - 0.0));
        let eq17_e394_d_n17: f64 = (var_ci_dn17 * (nv14 - 0.0));
        let eq17_value: f64 = eq17_e394;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq17_value),
            [0, 2, 6, 7, 10, 11, 12, 14, 17],
            [multiplicity * (eq17_e394_d_n0), multiplicity * (eq17_e394_d_n2), multiplicity * (eq17_e394_d_n6), multiplicity * (eq17_e394_d_n7), multiplicity * (eq17_e394_d_n10), multiplicity * (eq17_e394_d_n11), multiplicity * (eq17_e394_d_n12), multiplicity * (var_ci), multiplicity * (eq17_e394_d_n17)],
            [],
            [],
            1.0,
        );
        let eq18_e397: f64 = ((nv14 - 0.0) * var_sigrat_s);
        let eq18_e397_d_n0: f64 = ((nv14 - 0.0) * var_sigrat_s_dn0);
        let eq18_e397_d_n2: f64 = ((nv14 - 0.0) * var_sigrat_s_dn2);
        let eq18_e397_d_n6: f64 = ((nv14 - 0.0) * var_sigrat_s_dn6);
        let eq18_e397_d_n7: f64 = ((nv14 - 0.0) * var_sigrat_s_dn7);
        let eq18_e397_d_n10: f64 = ((nv14 - 0.0) * var_sigrat_s_dn10);
        let eq18_e397_d_n11: f64 = ((nv14 - 0.0) * var_sigrat_s_dn11);
        let eq18_e397_d_n12: f64 = ((nv14 - 0.0) * var_sigrat_s_dn12);
        let eq18_e397_d_n13: f64 = ((nv14 - 0.0) * var_sigrat_s_dn13);
        let eq18_e397_d_n15: f64 = ((nv14 - 0.0) * var_sigrat_s_dn15);
        let eq18_e397_d_n16: f64 = ((nv14 - 0.0) * var_sigrat_s_dn16);
        let eq18_e397_d_n17: f64 = ((nv14 - 0.0) * var_sigrat_s_dn17);
        let eq18_e397_d_n18: f64 = ((nv14 - 0.0) * var_sigrat_s_dn18);
        let eq18_e398: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq18_e397);
        let eq18_value: f64 = eq18_e398;
        let eq18_node_derivative_indices: [usize; 13] = [0, 2, 6, 7, 10, 11, 12, 13, 14, 15, 16, 17, 18];
        let eq18_node_derivatives: [f64; 13] = [(eq18_e397_d_n0 * ddt_scale), (eq18_e397_d_n2 * ddt_scale), (eq18_e397_d_n6 * ddt_scale), (eq18_e397_d_n7 * ddt_scale), (eq18_e397_d_n10 * ddt_scale), (eq18_e397_d_n11 * ddt_scale), (eq18_e397_d_n12 * ddt_scale), (eq18_e397_d_n13 * ddt_scale), (var_sigrat_s * ddt_scale), (eq18_e397_d_n15 * ddt_scale), (eq18_e397_d_n16 * ddt_scale), (eq18_e397_d_n17 * ddt_scale), (eq18_e397_d_n18 * ddt_scale)];
        let eq18_branch_derivative_indices: [usize; 0] = [];
        let eq18_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq18_value),
            &eq18_node_derivative_indices,
            &eq18_node_derivatives,
            &eq18_branch_derivative_indices,
            &eq18_branch_derivatives,
            multiplicity,
        );
        let eq19_e401: f64 = ((nv14 - 0.0) * var_sigrat_d);
        let eq19_e401_d_n0: f64 = ((nv14 - 0.0) * var_sigrat_d_dn0);
        let eq19_e401_d_n2: f64 = ((nv14 - 0.0) * var_sigrat_d_dn2);
        let eq19_e401_d_n6: f64 = ((nv14 - 0.0) * var_sigrat_d_dn6);
        let eq19_e401_d_n7: f64 = ((nv14 - 0.0) * var_sigrat_d_dn7);
        let eq19_e401_d_n10: f64 = ((nv14 - 0.0) * var_sigrat_d_dn10);
        let eq19_e401_d_n11: f64 = ((nv14 - 0.0) * var_sigrat_d_dn11);
        let eq19_e401_d_n12: f64 = ((nv14 - 0.0) * var_sigrat_d_dn12);
        let eq19_e401_d_n13: f64 = ((nv14 - 0.0) * var_sigrat_d_dn13);
        let eq19_e401_d_n15: f64 = ((nv14 - 0.0) * var_sigrat_d_dn15);
        let eq19_e401_d_n16: f64 = ((nv14 - 0.0) * var_sigrat_d_dn16);
        let eq19_e401_d_n17: f64 = ((nv14 - 0.0) * var_sigrat_d_dn17);
        let eq19_e401_d_n18: f64 = ((nv14 - 0.0) * var_sigrat_d_dn18);
        let eq19_e402: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq19_e401);
        let eq19_value: f64 = eq19_e402;
        let eq19_node_derivative_indices: [usize; 13] = [0, 2, 6, 7, 10, 11, 12, 13, 14, 15, 16, 17, 18];
        let eq19_node_derivatives: [f64; 13] = [(eq19_e401_d_n0 * ddt_scale), (eq19_e401_d_n2 * ddt_scale), (eq19_e401_d_n6 * ddt_scale), (eq19_e401_d_n7 * ddt_scale), (eq19_e401_d_n10 * ddt_scale), (eq19_e401_d_n11 * ddt_scale), (eq19_e401_d_n12 * ddt_scale), (eq19_e401_d_n13 * ddt_scale), (var_sigrat_d * ddt_scale), (eq19_e401_d_n15 * ddt_scale), (eq19_e401_d_n16 * ddt_scale), (eq19_e401_d_n17 * ddt_scale), (eq19_e401_d_n18 * ddt_scale)];
        let eq19_branch_derivative_indices: [usize; 0] = [];
        let eq19_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(6),
            multiplicity * (eq19_value),
            &eq19_node_derivative_indices,
            &eq19_node_derivatives,
            &eq19_branch_derivative_indices,
            &eq19_branch_derivatives,
            multiplicity,
        );
        let (eq25_e454, eq25_e454_d_n1, eq25_e454_d_n11,) = {
    if (p.p35 != 0.0) {
        let eq25_e452: f64 = (var_grg * (nv1 - nv11));
        (eq25_e452, var_grg, (-var_grg),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq25_value: f64 = eq25_e454;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(11),
            multiplicity * (eq25_value),
            1,
            multiplicity * (eq25_e454_d_n1),
            11,
            multiplicity * (eq25_e454_d_n11),
        );
        let (eq27_e465, eq27_e465_d_n10,) = {
    if (var_guard1224 != 0.0) {
        let eq27_e463: f64 = ((nv10 - 0.0) * var_gth);
        (eq27_e463, var_gth,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e465;
        stamper.stamp_current_node1_local(
            Some(10),
            None,
            multiplicity * (eq27_value),
            10,
            multiplicity * (eq27_e465_d_n10),
        );
        let (eq28_e470, eq28_e470_d_n0, eq28_e470_d_n2, eq28_e470_d_n6, eq28_e470_d_n7, eq28_e470_d_n10, eq28_e470_d_n11, eq28_e470_d_n12, eq28_e470_d_n17,) = {
    if (var_guard1224 != 0.0) {
        let eq28_e468: f64 = (-var_itemp);
        (eq28_e468, (-var_itemp_dn0), (-var_itemp_dn2), (-var_itemp_dn6), (-var_itemp_dn7), (-var_itemp_dn10), (-var_itemp_dn11), (-var_itemp_dn12), (-var_itemp_dn17),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_value: f64 = eq28_e470;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(10),
            None,
            multiplicity * (eq28_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq28_e470_d_n0), multiplicity * (eq28_e470_d_n2), multiplicity * (eq28_e470_d_n6), multiplicity * (eq28_e470_d_n7), multiplicity * (eq28_e470_d_n10), multiplicity * (eq28_e470_d_n11), multiplicity * (eq28_e470_d_n12), multiplicity * (eq28_e470_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq30_e483, eq30_e483_d_n10,) = {
    if (var_guard1224 != 0.0) {
        let eq30_e480: f64 = (var_cthe * (nv10 - 0.0));
        let eq30_e481: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq30_e480);
        (eq30_e481, (var_cthe * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e483;
        stamper.stamp_current_node1_local(
            Some(10),
            None,
            multiplicity * (eq30_value),
            10,
            multiplicity * (eq30_e483_d_n10),
        );
        let (eq32_e498, eq32_e498_d_n0, eq32_e498_d_n2, eq32_e498_d_n6, eq32_e498_d_n7, eq32_e498_d_n10, eq32_e498_d_n11, eq32_e498_d_n12, eq32_e498_d_n17,) = {
    if (var_guard1225 != 0.0) {
        let eq32_e495: f64 = (var_igidl + var_isub);
        let eq32_e495_d_n0: f64 = (var_igidl_dn0 + var_isub_dn0);
        let eq32_e495_d_n2: f64 = (var_igidl_dn2 + var_isub_dn2);
        let eq32_e495_d_n6: f64 = (var_igidl_dn6 + var_isub_dn6);
        let eq32_e495_d_n7: f64 = (var_igidl_dn7 + var_isub_dn7);
        let eq32_e495_d_n10: f64 = (var_igidl_dn10 + var_isub_dn10);
        let eq32_e495_d_n11: f64 = (var_igidl_dn11 + var_isub_dn11);
        let eq32_e495_d_n12: f64 = (var_igidl_dn12 + var_isub_dn12);
        let eq32_e495_d_n17: f64 = (var_igidl_dn17 + var_isub_dn17);
        let eq32_e496: f64 = (p.p50 * eq32_e495);
        let eq32_e496_d_n0: f64 = (p.p50 * eq32_e495_d_n0);
        let eq32_e496_d_n2: f64 = (p.p50 * eq32_e495_d_n2);
        let eq32_e496_d_n6: f64 = (p.p50 * eq32_e495_d_n6);
        let eq32_e496_d_n7: f64 = (p.p50 * eq32_e495_d_n7);
        let eq32_e496_d_n10: f64 = (p.p50 * eq32_e495_d_n10);
        let eq32_e496_d_n11: f64 = (p.p50 * eq32_e495_d_n11);
        let eq32_e496_d_n12: f64 = (p.p50 * eq32_e495_d_n12);
        let eq32_e496_d_n17: f64 = (p.p50 * eq32_e495_d_n17);
        (eq32_e496, eq32_e496_d_n0, eq32_e496_d_n2, eq32_e496_d_n6, eq32_e496_d_n7, eq32_e496_d_n10, eq32_e496_d_n11, eq32_e496_d_n12, eq32_e496_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq32_value: f64 = eq32_e498;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(6),
            Some(12),
            multiplicity * (eq32_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq32_e498_d_n0), multiplicity * (eq32_e498_d_n2), multiplicity * (eq32_e498_d_n6), multiplicity * (eq32_e498_d_n7), multiplicity * (eq32_e498_d_n10), multiplicity * (eq32_e498_d_n11), multiplicity * (eq32_e498_d_n12), multiplicity * (eq32_e498_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq33_e506, eq33_e506_d_n0, eq33_e506_d_n2, eq33_e506_d_n6, eq33_e506_d_n7, eq33_e506_d_n10, eq33_e506_d_n11, eq33_e506_d_n12, eq33_e506_d_n17,) = {
    if (var_guard1225 != 0.0) {
        let eq33_e503: f64 = (var_igisl + var_isubs);
        let eq33_e503_d_n0: f64 = (var_igisl_dn0 + var_isubs_dn0);
        let eq33_e503_d_n2: f64 = (var_igisl_dn2 + var_isubs_dn2);
        let eq33_e503_d_n6: f64 = (var_igisl_dn6 + var_isubs_dn6);
        let eq33_e503_d_n7: f64 = (var_igisl_dn7 + var_isubs_dn7);
        let eq33_e503_d_n10: f64 = (var_igisl_dn10 + var_isubs_dn10);
        let eq33_e503_d_n11: f64 = (var_igisl_dn11 + var_isubs_dn11);
        let eq33_e503_d_n12: f64 = (var_igisl_dn12 + var_isubs_dn12);
        let eq33_e503_d_n17: f64 = (var_igisl_dn17 + var_isubs_dn17);
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
            Some(7),
            Some(12),
            multiplicity * (eq33_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq33_e506_d_n0), multiplicity * (eq33_e506_d_n2), multiplicity * (eq33_e506_d_n6), multiplicity * (eq33_e506_d_n7), multiplicity * (eq33_e506_d_n10), multiplicity * (eq33_e506_d_n11), multiplicity * (eq33_e506_d_n12), multiplicity * (eq33_e506_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq34_e515, eq34_e515_d_n0, eq34_e515_d_n2, eq34_e515_d_n6, eq34_e515_d_n7, eq34_e515_d_n10, eq34_e515_d_n11, eq34_e515_d_n12, eq34_e515_d_n17,) = {
    if (var_guard1225 != 0.0) {
        let eq34_e511: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, var_qbs);
        let eq34_e512: f64 = (var_ibs + eq34_e511);
        let eq34_e512_d_n0: f64 = (var_ibs_dn0 + (var_qbs_dn0 * ddt_scale));
        let eq34_e512_d_n2: f64 = (var_ibs_dn2 + (var_qbs_dn2 * ddt_scale));
        let eq34_e512_d_n6: f64 = (var_ibs_dn6 + (var_qbs_dn6 * ddt_scale));
        let eq34_e512_d_n7: f64 = (var_ibs_dn7 + (var_qbs_dn7 * ddt_scale));
        let eq34_e512_d_n10: f64 = (var_ibs_dn10 + (var_qbs_dn10 * ddt_scale));
        let eq34_e512_d_n11: f64 = (var_ibs_dn11 + (var_qbs_dn11 * ddt_scale));
        let eq34_e512_d_n12: f64 = (var_ibs_dn12 + (var_qbs_dn12 * ddt_scale));
        let eq34_e512_d_n17: f64 = (var_ibs_dn17 + (var_qbs_dn17 * ddt_scale));
        let eq34_e513: f64 = (p.p50 * eq34_e512);
        let eq34_e513_d_n0: f64 = (p.p50 * eq34_e512_d_n0);
        let eq34_e513_d_n2: f64 = (p.p50 * eq34_e512_d_n2);
        let eq34_e513_d_n6: f64 = (p.p50 * eq34_e512_d_n6);
        let eq34_e513_d_n7: f64 = (p.p50 * eq34_e512_d_n7);
        let eq34_e513_d_n10: f64 = (p.p50 * eq34_e512_d_n10);
        let eq34_e513_d_n11: f64 = (p.p50 * eq34_e512_d_n11);
        let eq34_e513_d_n12: f64 = (p.p50 * eq34_e512_d_n12);
        let eq34_e513_d_n17: f64 = (p.p50 * eq34_e512_d_n17);
        (eq34_e513, eq34_e513_d_n0, eq34_e513_d_n2, eq34_e513_d_n6, eq34_e513_d_n7, eq34_e513_d_n10, eq34_e513_d_n11, eq34_e513_d_n12, eq34_e513_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq34_value: f64 = eq34_e515;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(9),
            Some(7),
            multiplicity * (eq34_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq34_e515_d_n0), multiplicity * (eq34_e515_d_n2), multiplicity * (eq34_e515_d_n6), multiplicity * (eq34_e515_d_n7), multiplicity * (eq34_e515_d_n10), multiplicity * (eq34_e515_d_n11), multiplicity * (eq34_e515_d_n12), multiplicity * (eq34_e515_d_n17)],
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
        var_guard1225: f64,
        var_guard1226: f64,
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
        let (eq35_e524, eq35_e524_d_n0, eq35_e524_d_n2, eq35_e524_d_n6, eq35_e524_d_n7, eq35_e524_d_n10, eq35_e524_d_n11, eq35_e524_d_n12, eq35_e524_d_n17,) = {
    if (var_guard1225 != 0.0) {
        let eq35_e520: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, var_qbd);
        let eq35_e521: f64 = (var_ibd + eq35_e520);
        let eq35_e521_d_n0: f64 = (var_ibd_dn0 + (var_qbd_dn0 * ddt_scale));
        let eq35_e521_d_n2: f64 = (var_ibd_dn2 + (var_qbd_dn2 * ddt_scale));
        let eq35_e521_d_n6: f64 = (var_ibd_dn6 + (var_qbd_dn6 * ddt_scale));
        let eq35_e521_d_n7: f64 = (var_ibd_dn7 + (var_qbd_dn7 * ddt_scale));
        let eq35_e521_d_n10: f64 = (var_ibd_dn10 + (var_qbd_dn10 * ddt_scale));
        let eq35_e521_d_n11: f64 = (var_ibd_dn11 + (var_qbd_dn11 * ddt_scale));
        let eq35_e521_d_n12: f64 = (var_ibd_dn12 + (var_qbd_dn12 * ddt_scale));
        let eq35_e521_d_n17: f64 = (var_ibd_dn17 + (var_qbd_dn17 * ddt_scale));
        let eq35_e522: f64 = (p.p50 * eq35_e521);
        let eq35_e522_d_n0: f64 = (p.p50 * eq35_e521_d_n0);
        let eq35_e522_d_n2: f64 = (p.p50 * eq35_e521_d_n2);
        let eq35_e522_d_n6: f64 = (p.p50 * eq35_e521_d_n6);
        let eq35_e522_d_n7: f64 = (p.p50 * eq35_e521_d_n7);
        let eq35_e522_d_n10: f64 = (p.p50 * eq35_e521_d_n10);
        let eq35_e522_d_n11: f64 = (p.p50 * eq35_e521_d_n11);
        let eq35_e522_d_n12: f64 = (p.p50 * eq35_e521_d_n12);
        let eq35_e522_d_n17: f64 = (p.p50 * eq35_e521_d_n17);
        (eq35_e522, eq35_e522_d_n0, eq35_e522_d_n2, eq35_e522_d_n6, eq35_e522_d_n7, eq35_e522_d_n10, eq35_e522_d_n11, eq35_e522_d_n12, eq35_e522_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_value: f64 = eq35_e524;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(8),
            Some(6),
            multiplicity * (eq35_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq35_e524_d_n0), multiplicity * (eq35_e524_d_n2), multiplicity * (eq35_e524_d_n6), multiplicity * (eq35_e524_d_n7), multiplicity * (eq35_e524_d_n10), multiplicity * (eq35_e524_d_n11), multiplicity * (eq35_e524_d_n12), multiplicity * (eq35_e524_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq36_e532, eq36_e532_d_n0, eq36_e532_d_n2, eq36_e532_d_n4, eq36_e532_d_n6, eq36_e532_d_n7, eq36_e532_d_n10, eq36_e532_d_n11, eq36_e532_d_n12, eq36_e532_d_n17,) = {
    if ((var_guard1225 != 0.0) && (p.p261 != 0.0)) {
        let eq36_e530: f64 = ((nv4 - nv12) / var_rbulk);
        let eq36_e530_d_n0: f64 = (-(((nv4 - nv12) * var_rbulk_dn0) / (var_rbulk * var_rbulk)));
        let eq36_e530_d_n2: f64 = (-(((nv4 - nv12) * var_rbulk_dn2) / (var_rbulk * var_rbulk)));
        let eq36_e530_d_n4: f64 = (1.0 / var_rbulk);
        let eq36_e530_d_n6: f64 = (-(((nv4 - nv12) * var_rbulk_dn6) / (var_rbulk * var_rbulk)));
        let eq36_e530_d_n7: f64 = (-(((nv4 - nv12) * var_rbulk_dn7) / (var_rbulk * var_rbulk)));
        let eq36_e530_d_n10: f64 = (-(((nv4 - nv12) * var_rbulk_dn10) / (var_rbulk * var_rbulk)));
        let eq36_e530_d_n11: f64 = (-(((nv4 - nv12) * var_rbulk_dn11) / (var_rbulk * var_rbulk)));
        let eq36_e530_d_n12: f64 = (((-var_rbulk) - ((nv4 - nv12) * var_rbulk_dn12)) / (var_rbulk * var_rbulk));
        let eq36_e530_d_n17: f64 = (-(((nv4 - nv12) * var_rbulk_dn17) / (var_rbulk * var_rbulk)));
        (eq36_e530, eq36_e530_d_n0, eq36_e530_d_n2, eq36_e530_d_n4, eq36_e530_d_n6, eq36_e530_d_n7, eq36_e530_d_n10, eq36_e530_d_n11, eq36_e530_d_n12, eq36_e530_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_value: f64 = eq36_e532;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(4),
            Some(12),
            multiplicity * (eq36_value),
            [0, 2, 4, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq36_e532_d_n0), multiplicity * (eq36_e532_d_n2), multiplicity * (eq36_e532_d_n4), multiplicity * (eq36_e532_d_n6), multiplicity * (eq36_e532_d_n7), multiplicity * (eq36_e532_d_n10), multiplicity * (eq36_e532_d_n11), multiplicity * (eq36_e532_d_n12), multiplicity * (eq36_e532_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq42_e575, eq42_e575_d_n0, eq42_e575_d_n2, eq42_e575_d_n6, eq42_e575_d_n7, eq42_e575_d_n10, eq42_e575_d_n11, eq42_e575_d_n12, eq42_e575_d_n17, eq42_e575_d_n18,) = {
    if ((var_guard1225 != 0.0) && (p.p34 != 0.0)) {
        (var_iqi_nqs, var_iqi_nqs_dn0, var_iqi_nqs_dn2, var_iqi_nqs_dn6, var_iqi_nqs_dn7, var_iqi_nqs_dn10, var_iqi_nqs_dn11, var_iqi_nqs_dn12, var_iqi_nqs_dn17, var_iqi_nqs_dn18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq42_value: f64 = eq42_e575;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(18),
            None,
            multiplicity * (eq42_value),
            [0, 2, 6, 7, 10, 11, 12, 17, 18],
            [multiplicity * (eq42_e575_d_n0), multiplicity * (eq42_e575_d_n2), multiplicity * (eq42_e575_d_n6), multiplicity * (eq42_e575_d_n7), multiplicity * (eq42_e575_d_n10), multiplicity * (eq42_e575_d_n11), multiplicity * (eq42_e575_d_n12), multiplicity * (eq42_e575_d_n17), multiplicity * (eq42_e575_d_n18)],
            [],
            [],
            1.0,
        );
        let (eq43_e581, eq43_e581_d_n0, eq43_e581_d_n2, eq43_e581_d_n6, eq43_e581_d_n7, eq43_e581_d_n10, eq43_e581_d_n11, eq43_e581_d_n12, eq43_e581_d_n13, eq43_e581_d_n15, eq43_e581_d_n16, eq43_e581_d_n17, eq43_e581_d_n18,) = {
    if ((var_guard1225 != 0.0) && (p.p34 != 0.0)) {
        (var_iqb_nqs, var_iqb_nqs_dn0, var_iqb_nqs_dn2, var_iqb_nqs_dn6, var_iqb_nqs_dn7, var_iqb_nqs_dn10, var_iqb_nqs_dn11, var_iqb_nqs_dn12, var_iqb_nqs_dn13, var_iqb_nqs_dn15, var_iqb_nqs_dn16, var_iqb_nqs_dn17, var_iqb_nqs_dn18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq43_value: f64 = eq43_e581;
        let eq43_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq43_node_derivatives: [f64; 12] = [eq43_e581_d_n0, eq43_e581_d_n2, eq43_e581_d_n6, eq43_e581_d_n7, eq43_e581_d_n10, eq43_e581_d_n11, eq43_e581_d_n12, eq43_e581_d_n13, eq43_e581_d_n15, eq43_e581_d_n16, eq43_e581_d_n17, eq43_e581_d_n18];
        let eq43_branch_derivative_indices: [usize; 0] = [];
        let eq43_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(13),
            None,
            multiplicity * (eq43_value),
            &eq43_node_derivative_indices,
            &eq43_node_derivatives,
            &eq43_branch_derivative_indices,
            &eq43_branch_derivatives,
            multiplicity,
        );
        let (eq46_e608, eq46_e608_d_n18,) = {
    if ((var_guard1225 != 0.0) && (p.p34 != 0.0)) {
        let eq46_e603: f64 = (1e-9 / 0.0001);
        let eq46_e605: f64 = (eq46_e603 * (nv18 - 0.0));
        let eq46_e606: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq46_e605);
        (eq46_e606, (eq46_e603 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq46_value: f64 = eq46_e608;
        stamper.stamp_current_node1_local(
            Some(18),
            None,
            multiplicity * (eq46_value),
            18,
            multiplicity * (eq46_e608_d_n18),
        );
        let (eq47_e619, eq47_e619_d_n13,) = {
    if ((var_guard1225 != 0.0) && (p.p34 != 0.0)) {
        let eq47_e614: f64 = (1e-9 / 0.0001);
        let eq47_e616: f64 = (eq47_e614 * (nv13 - 0.0));
        let eq47_e617: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq47_e616);
        (eq47_e617, (eq47_e614 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq47_value: f64 = eq47_e619;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq47_value),
            13,
            multiplicity * (eq47_e619_d_n13),
        );
        let (eq50_e639, eq50_e639_d_n0, eq50_e639_d_n2, eq50_e639_d_n6, eq50_e639_d_n7, eq50_e639_d_n10, eq50_e639_d_n11, eq50_e639_d_n12, eq50_e639_d_n17,) = {
    if ((var_guard1225 != 0.0) && (var_guard1226 != 0.0)) {
        (var_iqh_nqs, var_iqh_nqs_dn0, var_iqh_nqs_dn2, var_iqh_nqs_dn6, var_iqh_nqs_dn7, var_iqh_nqs_dn10, var_iqh_nqs_dn11, var_iqh_nqs_dn12, var_iqh_nqs_dn17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_value: f64 = eq50_e639;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(17),
            None,
            multiplicity * (eq50_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq50_e639_d_n0), multiplicity * (eq50_e639_d_n2), multiplicity * (eq50_e639_d_n6), multiplicity * (eq50_e639_d_n7), multiplicity * (eq50_e639_d_n10), multiplicity * (eq50_e639_d_n11), multiplicity * (eq50_e639_d_n12), multiplicity * (eq50_e639_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq52_e658, eq52_e658_d_n17,) = {
    if ((var_guard1225 != 0.0) && (var_guard1226 != 0.0)) {
        let eq52_e653: f64 = (1e-9 / 0.0001);
        let eq52_e655: f64 = (eq52_e653 * (nv17 - 0.0));
        let eq52_e656: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, eq52_e655);
        (eq52_e656, (eq52_e653 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq52_value: f64 = eq52_e658;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (eq52_value),
            17,
            multiplicity * (eq52_e658_d_n17),
        );
        let (eq54_e674, eq54_e674_d_n0, eq54_e674_d_n2, eq54_e674_d_n6, eq54_e674_d_n7, eq54_e674_d_n10, eq54_e674_d_n11, eq54_e674_d_n12, eq54_e674_d_n17,) = {
    if (var_guard1225 == 0.0) {
        let eq54_e671: f64 = (var_igidl + var_isub);
        let eq54_e671_d_n0: f64 = (var_igidl_dn0 + var_isub_dn0);
        let eq54_e671_d_n2: f64 = (var_igidl_dn2 + var_isub_dn2);
        let eq54_e671_d_n6: f64 = (var_igidl_dn6 + var_isub_dn6);
        let eq54_e671_d_n7: f64 = (var_igidl_dn7 + var_isub_dn7);
        let eq54_e671_d_n10: f64 = (var_igidl_dn10 + var_isub_dn10);
        let eq54_e671_d_n11: f64 = (var_igidl_dn11 + var_isub_dn11);
        let eq54_e671_d_n12: f64 = (var_igidl_dn12 + var_isub_dn12);
        let eq54_e671_d_n17: f64 = (var_igidl_dn17 + var_isub_dn17);
        let eq54_e672: f64 = (p.p50 * eq54_e671);
        let eq54_e672_d_n0: f64 = (p.p50 * eq54_e671_d_n0);
        let eq54_e672_d_n2: f64 = (p.p50 * eq54_e671_d_n2);
        let eq54_e672_d_n6: f64 = (p.p50 * eq54_e671_d_n6);
        let eq54_e672_d_n7: f64 = (p.p50 * eq54_e671_d_n7);
        let eq54_e672_d_n10: f64 = (p.p50 * eq54_e671_d_n10);
        let eq54_e672_d_n11: f64 = (p.p50 * eq54_e671_d_n11);
        let eq54_e672_d_n12: f64 = (p.p50 * eq54_e671_d_n12);
        let eq54_e672_d_n17: f64 = (p.p50 * eq54_e671_d_n17);
        (eq54_e672, eq54_e672_d_n0, eq54_e672_d_n2, eq54_e672_d_n6, eq54_e672_d_n7, eq54_e672_d_n10, eq54_e672_d_n11, eq54_e672_d_n12, eq54_e672_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq54_value: f64 = eq54_e674;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq54_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq54_e674_d_n0), multiplicity * (eq54_e674_d_n2), multiplicity * (eq54_e674_d_n6), multiplicity * (eq54_e674_d_n7), multiplicity * (eq54_e674_d_n10), multiplicity * (eq54_e674_d_n11), multiplicity * (eq54_e674_d_n12), multiplicity * (eq54_e674_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq55_e683, eq55_e683_d_n0, eq55_e683_d_n2, eq55_e683_d_n6, eq55_e683_d_n7, eq55_e683_d_n10, eq55_e683_d_n11, eq55_e683_d_n12, eq55_e683_d_n17,) = {
    if (var_guard1225 == 0.0) {
        let eq55_e680: f64 = (var_igisl + var_isubs);
        let eq55_e680_d_n0: f64 = (var_igisl_dn0 + var_isubs_dn0);
        let eq55_e680_d_n2: f64 = (var_igisl_dn2 + var_isubs_dn2);
        let eq55_e680_d_n6: f64 = (var_igisl_dn6 + var_isubs_dn6);
        let eq55_e680_d_n7: f64 = (var_igisl_dn7 + var_isubs_dn7);
        let eq55_e680_d_n10: f64 = (var_igisl_dn10 + var_isubs_dn10);
        let eq55_e680_d_n11: f64 = (var_igisl_dn11 + var_isubs_dn11);
        let eq55_e680_d_n12: f64 = (var_igisl_dn12 + var_isubs_dn12);
        let eq55_e680_d_n17: f64 = (var_igisl_dn17 + var_isubs_dn17);
        let eq55_e681: f64 = (p.p50 * eq55_e680);
        let eq55_e681_d_n0: f64 = (p.p50 * eq55_e680_d_n0);
        let eq55_e681_d_n2: f64 = (p.p50 * eq55_e680_d_n2);
        let eq55_e681_d_n6: f64 = (p.p50 * eq55_e680_d_n6);
        let eq55_e681_d_n7: f64 = (p.p50 * eq55_e680_d_n7);
        let eq55_e681_d_n10: f64 = (p.p50 * eq55_e680_d_n10);
        let eq55_e681_d_n11: f64 = (p.p50 * eq55_e680_d_n11);
        let eq55_e681_d_n12: f64 = (p.p50 * eq55_e680_d_n12);
        let eq55_e681_d_n17: f64 = (p.p50 * eq55_e680_d_n17);
        (eq55_e681, eq55_e681_d_n0, eq55_e681_d_n2, eq55_e681_d_n6, eq55_e681_d_n7, eq55_e681_d_n10, eq55_e681_d_n11, eq55_e681_d_n12, eq55_e681_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e683;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(7),
            Some(6),
            multiplicity * (eq55_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq55_e683_d_n0), multiplicity * (eq55_e683_d_n2), multiplicity * (eq55_e683_d_n6), multiplicity * (eq55_e683_d_n7), multiplicity * (eq55_e683_d_n10), multiplicity * (eq55_e683_d_n11), multiplicity * (eq55_e683_d_n12), multiplicity * (eq55_e683_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq57_e695, eq57_e695_d_n0, eq57_e695_d_n2, eq57_e695_d_n6, eq57_e695_d_n7, eq57_e695_d_n10, eq57_e695_d_n11, eq57_e695_d_n12, eq57_e695_d_n17,) = {
    if ((var_guard1225 == 0.0) && (p.p37 != 0.0)) {
        (var_iqh_nqs, var_iqh_nqs_dn0, var_iqh_nqs_dn2, var_iqh_nqs_dn6, var_iqh_nqs_dn7, var_iqh_nqs_dn10, var_iqh_nqs_dn11, var_iqh_nqs_dn12, var_iqh_nqs_dn17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e695;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(17),
            None,
            multiplicity * (eq57_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq57_e695_d_n0), multiplicity * (eq57_e695_d_n2), multiplicity * (eq57_e695_d_n6), multiplicity * (eq57_e695_d_n7), multiplicity * (eq57_e695_d_n10), multiplicity * (eq57_e695_d_n11), multiplicity * (eq57_e695_d_n12), multiplicity * (eq57_e695_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq59_e716, eq59_e716_d_n17,) = {
    if ((var_guard1225 == 0.0) && (p.p37 != 0.0)) {
        let eq59_e711: f64 = (1e-9 / 0.0001);
        let eq59_e713: f64 = (eq59_e711 * (nv17 - 0.0));
        let eq59_e714: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, eq59_e713);
        (eq59_e714, (eq59_e711 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq59_value: f64 = eq59_e716;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (eq59_value),
            17,
            multiplicity * (eq59_e716_d_n17),
        );
        let (eq61_e731, eq61_e731_d_n0, eq61_e731_d_n2, eq61_e731_d_n6, eq61_e731_d_n7, eq61_e731_d_n10, eq61_e731_d_n11, eq61_e731_d_n12, eq61_e731_d_n13, eq61_e731_d_n15, eq61_e731_d_n16, eq61_e731_d_n17, eq61_e731_d_n18,) = {
    if ((var_guard1225 == 0.0) && (p.p34 != 0.0)) {
        (var_iqd_nqs, var_iqd_nqs_dn0, var_iqd_nqs_dn2, var_iqd_nqs_dn6, var_iqd_nqs_dn7, var_iqd_nqs_dn10, var_iqd_nqs_dn11, var_iqd_nqs_dn12, var_iqd_nqs_dn13, var_iqd_nqs_dn15, var_iqd_nqs_dn16, var_iqd_nqs_dn17, var_iqd_nqs_dn18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq61_value: f64 = eq61_e731;
        let eq61_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq61_node_derivatives: [f64; 12] = [eq61_e731_d_n0, eq61_e731_d_n2, eq61_e731_d_n6, eq61_e731_d_n7, eq61_e731_d_n10, eq61_e731_d_n11, eq61_e731_d_n12, eq61_e731_d_n13, eq61_e731_d_n15, eq61_e731_d_n16, eq61_e731_d_n17, eq61_e731_d_n18];
        let eq61_branch_derivative_indices: [usize; 0] = [];
        let eq61_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(15),
            None,
            multiplicity * (eq61_value),
            &eq61_node_derivative_indices,
            &eq61_node_derivatives,
            &eq61_branch_derivative_indices,
            &eq61_branch_derivatives,
            multiplicity,
        );
        let (eq62_e738, eq62_e738_d_n0, eq62_e738_d_n2, eq62_e738_d_n6, eq62_e738_d_n7, eq62_e738_d_n10, eq62_e738_d_n11, eq62_e738_d_n12, eq62_e738_d_n13, eq62_e738_d_n15, eq62_e738_d_n16, eq62_e738_d_n17, eq62_e738_d_n18,) = {
    if ((var_guard1225 == 0.0) && (p.p34 != 0.0)) {
        (var_iqs_nqs, var_iqs_nqs_dn0, var_iqs_nqs_dn2, var_iqs_nqs_dn6, var_iqs_nqs_dn7, var_iqs_nqs_dn10, var_iqs_nqs_dn11, var_iqs_nqs_dn12, var_iqs_nqs_dn13, var_iqs_nqs_dn15, var_iqs_nqs_dn16, var_iqs_nqs_dn17, var_iqs_nqs_dn18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_value: f64 = eq62_e738;
        let eq62_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq62_node_derivatives: [f64; 12] = [eq62_e738_d_n0, eq62_e738_d_n2, eq62_e738_d_n6, eq62_e738_d_n7, eq62_e738_d_n10, eq62_e738_d_n11, eq62_e738_d_n12, eq62_e738_d_n13, eq62_e738_d_n15, eq62_e738_d_n16, eq62_e738_d_n17, eq62_e738_d_n18];
        let eq62_branch_derivative_indices: [usize; 0] = [];
        let eq62_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(16),
            None,
            multiplicity * (eq62_value),
            &eq62_node_derivative_indices,
            &eq62_node_derivatives,
            &eq62_branch_derivative_indices,
            &eq62_branch_derivatives,
            multiplicity,
        );
        let (eq63_e745, eq63_e745_d_n0, eq63_e745_d_n2, eq63_e745_d_n6, eq63_e745_d_n7, eq63_e745_d_n10, eq63_e745_d_n11, eq63_e745_d_n12, eq63_e745_d_n13, eq63_e745_d_n15, eq63_e745_d_n16, eq63_e745_d_n17, eq63_e745_d_n18,) = {
    if ((var_guard1225 == 0.0) && (p.p34 != 0.0)) {
        (var_iqb_nqs, var_iqb_nqs_dn0, var_iqb_nqs_dn2, var_iqb_nqs_dn6, var_iqb_nqs_dn7, var_iqb_nqs_dn10, var_iqb_nqs_dn11, var_iqb_nqs_dn12, var_iqb_nqs_dn13, var_iqb_nqs_dn15, var_iqb_nqs_dn16, var_iqb_nqs_dn17, var_iqb_nqs_dn18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq63_value: f64 = eq63_e745;
        let eq63_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq63_node_derivatives: [f64; 12] = [eq63_e745_d_n0, eq63_e745_d_n2, eq63_e745_d_n6, eq63_e745_d_n7, eq63_e745_d_n10, eq63_e745_d_n11, eq63_e745_d_n12, eq63_e745_d_n13, eq63_e745_d_n15, eq63_e745_d_n16, eq63_e745_d_n17, eq63_e745_d_n18];
        let eq63_branch_derivative_indices: [usize; 0] = [];
        let eq63_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(13),
            None,
            multiplicity * (eq63_value),
            &eq63_node_derivative_indices,
            &eq63_node_derivatives,
            &eq63_branch_derivative_indices,
            &eq63_branch_derivatives,
            multiplicity,
        );
        let (eq67_e784, eq67_e784_d_n15,) = {
    if ((var_guard1225 == 0.0) && (p.p34 != 0.0)) {
        let eq67_e779: f64 = (1e-9 / 0.0001);
        let eq67_e781: f64 = (eq67_e779 * (nv15 - 0.0));
        let eq67_e782: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, eq67_e781);
        (eq67_e782, (eq67_e779 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq67_value: f64 = eq67_e784;
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (eq67_value),
            15,
            multiplicity * (eq67_e784_d_n15),
        );
        let (eq68_e796, eq68_e796_d_n16,) = {
    if ((var_guard1225 == 0.0) && (p.p34 != 0.0)) {
        let eq68_e791: f64 = (1e-9 / 0.0001);
        let eq68_e793: f64 = (eq68_e791 * (nv16 - 0.0));
        let eq68_e794: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, eq68_e793);
        (eq68_e794, (eq68_e791 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq68_value: f64 = eq68_e796;
        stamper.stamp_current_node1_local(
            Some(16),
            None,
            multiplicity * (eq68_value),
            16,
            multiplicity * (eq68_e796_d_n16),
        );
        let (eq69_e808, eq69_e808_d_n13,) = {
    if ((var_guard1225 == 0.0) && (p.p34 != 0.0)) {
        let eq69_e803: f64 = (1e-9 / 0.0001);
        let eq69_e805: f64 = (eq69_e803 * (nv13 - 0.0));
        let eq69_e806: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, eq69_e805);
        (eq69_e806, (eq69_e803 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq69_value: f64 = eq69_e808;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq69_value),
            13,
            multiplicity * (eq69_e808_d_n13),
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
        var_guard1224: f64,
        var_guard1225: f64,
        var_guard1226: f64,
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
        let eq10_e359_q: f64 = var_qg;
        let eq10_e360: f64 = (p.p50 * var_qg);
        let eq10_e360_d_n0: f64 = (p.p50 * var_qg_dn0);
        let eq10_e360_d_n2: f64 = (p.p50 * var_qg_dn2);
        let eq10_e360_d_n6: f64 = (p.p50 * var_qg_dn6);
        let eq10_e360_d_n7: f64 = (p.p50 * var_qg_dn7);
        let eq10_e360_d_n10: f64 = (p.p50 * var_qg_dn10);
        let eq10_e360_d_n11: f64 = (p.p50 * var_qg_dn11);
        let eq10_e360_d_n12: f64 = (p.p50 * var_qg_dn12);
        let eq10_e360_d_n13: f64 = (p.p50 * var_qg_dn13);
        let eq10_e360_d_n15: f64 = (p.p50 * var_qg_dn15);
        let eq10_e360_d_n16: f64 = (p.p50 * var_qg_dn16);
        let eq10_e360_d_n17: f64 = (p.p50 * var_qg_dn17);
        let eq10_e360_d_n18: f64 = (p.p50 * var_qg_dn18);
        let eq10_e360_q: f64 = (p.p50 * eq10_e359_q);
        let eq10_reactive_node_derivatives: [f64; 19] = [eq10_e360_d_n0, 0.0, eq10_e360_d_n2, 0.0, 0.0, 0.0, eq10_e360_d_n6, eq10_e360_d_n7, 0.0, 0.0, eq10_e360_d_n10, eq10_e360_d_n11, eq10_e360_d_n12, eq10_e360_d_n13, 0.0, eq10_e360_d_n15, eq10_e360_d_n16, eq10_e360_d_n17, eq10_e360_d_n18];
        let eq10_reactive_branch_derivatives: [f64; 15] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            nodes,
            &eq10_reactive_node_derivatives,
            branches,
            &eq10_reactive_branch_derivatives,
            multiplicity,
        );
        let eq11_e363_q: f64 = var_qd;
        let eq11_e364: f64 = (p.p50 * var_qd);
        let eq11_e364_d_n0: f64 = (p.p50 * var_qd_dn0);
        let eq11_e364_d_n2: f64 = (p.p50 * var_qd_dn2);
        let eq11_e364_d_n6: f64 = (p.p50 * var_qd_dn6);
        let eq11_e364_d_n7: f64 = (p.p50 * var_qd_dn7);
        let eq11_e364_d_n10: f64 = (p.p50 * var_qd_dn10);
        let eq11_e364_d_n11: f64 = (p.p50 * var_qd_dn11);
        let eq11_e364_d_n12: f64 = (p.p50 * var_qd_dn12);
        let eq11_e364_d_n13: f64 = (p.p50 * var_qd_dn13);
        let eq11_e364_d_n15: f64 = (p.p50 * var_qd_dn15);
        let eq11_e364_d_n16: f64 = (p.p50 * var_qd_dn16);
        let eq11_e364_d_n17: f64 = (p.p50 * var_qd_dn17);
        let eq11_e364_d_n18: f64 = (p.p50 * var_qd_dn18);
        let eq11_e364_q: f64 = (p.p50 * eq11_e363_q);
        let eq11_reactive_node_derivatives: [f64; 19] = [eq11_e364_d_n0, 0.0, eq11_e364_d_n2, 0.0, 0.0, 0.0, eq11_e364_d_n6, eq11_e364_d_n7, 0.0, 0.0, eq11_e364_d_n10, eq11_e364_d_n11, eq11_e364_d_n12, eq11_e364_d_n13, 0.0, eq11_e364_d_n15, eq11_e364_d_n16, eq11_e364_d_n17, eq11_e364_d_n18];
        let eq11_reactive_branch_derivatives: [f64; 15] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            nodes,
            &eq11_reactive_node_derivatives,
            branches,
            &eq11_reactive_branch_derivatives,
            multiplicity,
        );
        let eq12_e367_q: f64 = var_qb;
        let eq12_e368: f64 = (p.p50 * var_qb);
        let eq12_e368_d_n0: f64 = (p.p50 * var_qb_dn0);
        let eq12_e368_d_n2: f64 = (p.p50 * var_qb_dn2);
        let eq12_e368_d_n6: f64 = (p.p50 * var_qb_dn6);
        let eq12_e368_d_n7: f64 = (p.p50 * var_qb_dn7);
        let eq12_e368_d_n10: f64 = (p.p50 * var_qb_dn10);
        let eq12_e368_d_n11: f64 = (p.p50 * var_qb_dn11);
        let eq12_e368_d_n12: f64 = (p.p50 * var_qb_dn12);
        let eq12_e368_d_n13: f64 = (p.p50 * var_qb_dn13);
        let eq12_e368_d_n15: f64 = (p.p50 * var_qb_dn15);
        let eq12_e368_d_n16: f64 = (p.p50 * var_qb_dn16);
        let eq12_e368_d_n17: f64 = (p.p50 * var_qb_dn17);
        let eq12_e368_d_n18: f64 = (p.p50 * var_qb_dn18);
        let eq12_e368_q: f64 = (p.p50 * eq12_e367_q);
        let eq12_reactive_node_derivatives: [f64; 19] = [eq12_e368_d_n0, 0.0, eq12_e368_d_n2, 0.0, 0.0, 0.0, eq12_e368_d_n6, eq12_e368_d_n7, 0.0, 0.0, eq12_e368_d_n10, eq12_e368_d_n11, eq12_e368_d_n12, eq12_e368_d_n13, 0.0, eq12_e368_d_n15, eq12_e368_d_n16, eq12_e368_d_n17, eq12_e368_d_n18];
        let eq12_reactive_branch_derivatives: [f64; 15] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            Some(nodes[7]),
            nodes,
            &eq12_reactive_node_derivatives,
            branches,
            &eq12_reactive_branch_derivatives,
            multiplicity,
        );
        let eq18_e397: f64 = ((nv14 - 0.0) * var_sigrat_s);
        let eq18_e397_d_n0: f64 = ((nv14 - 0.0) * var_sigrat_s_dn0);
        let eq18_e397_d_n2: f64 = ((nv14 - 0.0) * var_sigrat_s_dn2);
        let eq18_e397_d_n6: f64 = ((nv14 - 0.0) * var_sigrat_s_dn6);
        let eq18_e397_d_n7: f64 = ((nv14 - 0.0) * var_sigrat_s_dn7);
        let eq18_e397_d_n10: f64 = ((nv14 - 0.0) * var_sigrat_s_dn10);
        let eq18_e397_d_n11: f64 = ((nv14 - 0.0) * var_sigrat_s_dn11);
        let eq18_e397_d_n12: f64 = ((nv14 - 0.0) * var_sigrat_s_dn12);
        let eq18_e397_d_n13: f64 = ((nv14 - 0.0) * var_sigrat_s_dn13);
        let eq18_e397_d_n15: f64 = ((nv14 - 0.0) * var_sigrat_s_dn15);
        let eq18_e397_d_n16: f64 = ((nv14 - 0.0) * var_sigrat_s_dn16);
        let eq18_e397_d_n17: f64 = ((nv14 - 0.0) * var_sigrat_s_dn17);
        let eq18_e397_d_n18: f64 = ((nv14 - 0.0) * var_sigrat_s_dn18);
        let eq18_e398_q: f64 = eq18_e397;
        let eq18_reactive_node_derivatives: [f64; 19] = [eq18_e397_d_n0, 0.0, eq18_e397_d_n2, 0.0, 0.0, 0.0, eq18_e397_d_n6, eq18_e397_d_n7, 0.0, 0.0, eq18_e397_d_n10, eq18_e397_d_n11, eq18_e397_d_n12, eq18_e397_d_n13, var_sigrat_s, eq18_e397_d_n15, eq18_e397_d_n16, eq18_e397_d_n17, eq18_e397_d_n18];
        let eq18_reactive_branch_derivatives: [f64; 15] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            nodes,
            &eq18_reactive_node_derivatives,
            branches,
            &eq18_reactive_branch_derivatives,
            multiplicity,
        );
        let eq19_e401: f64 = ((nv14 - 0.0) * var_sigrat_d);
        let eq19_e401_d_n0: f64 = ((nv14 - 0.0) * var_sigrat_d_dn0);
        let eq19_e401_d_n2: f64 = ((nv14 - 0.0) * var_sigrat_d_dn2);
        let eq19_e401_d_n6: f64 = ((nv14 - 0.0) * var_sigrat_d_dn6);
        let eq19_e401_d_n7: f64 = ((nv14 - 0.0) * var_sigrat_d_dn7);
        let eq19_e401_d_n10: f64 = ((nv14 - 0.0) * var_sigrat_d_dn10);
        let eq19_e401_d_n11: f64 = ((nv14 - 0.0) * var_sigrat_d_dn11);
        let eq19_e401_d_n12: f64 = ((nv14 - 0.0) * var_sigrat_d_dn12);
        let eq19_e401_d_n13: f64 = ((nv14 - 0.0) * var_sigrat_d_dn13);
        let eq19_e401_d_n15: f64 = ((nv14 - 0.0) * var_sigrat_d_dn15);
        let eq19_e401_d_n16: f64 = ((nv14 - 0.0) * var_sigrat_d_dn16);
        let eq19_e401_d_n17: f64 = ((nv14 - 0.0) * var_sigrat_d_dn17);
        let eq19_e401_d_n18: f64 = ((nv14 - 0.0) * var_sigrat_d_dn18);
        let eq19_e402_q: f64 = eq19_e401;
        let eq19_reactive_node_derivatives: [f64; 19] = [eq19_e401_d_n0, 0.0, eq19_e401_d_n2, 0.0, 0.0, 0.0, eq19_e401_d_n6, eq19_e401_d_n7, 0.0, 0.0, eq19_e401_d_n10, eq19_e401_d_n11, eq19_e401_d_n12, eq19_e401_d_n13, var_sigrat_d, eq19_e401_d_n15, eq19_e401_d_n16, eq19_e401_d_n17, eq19_e401_d_n18];
        let eq19_reactive_branch_derivatives: [f64; 15] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[6]),
            nodes,
            &eq19_reactive_node_derivatives,
            branches,
            &eq19_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq30_e483, eq30_e483_d_n10, eq30_e483_q,) = {
    if (var_guard1224 != 0.0) {
        let eq30_e480: f64 = (var_cthe * (nv10 - 0.0));
        let eq30_e481_q: f64 = eq30_e480;
        (eq30_e480, var_cthe, eq30_e481_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[10]),
            None,
            nodes[10],
            multiplicity * (eq30_e483_d_n10),
        );
        let (eq34_e515, eq34_e515_d_n0, eq34_e515_d_n2, eq34_e515_d_n6, eq34_e515_d_n7, eq34_e515_d_n10, eq34_e515_d_n11, eq34_e515_d_n12, eq34_e515_d_n17, eq34_e515_q, eq34_e515_q_d_n0, eq34_e515_q_d_n2, eq34_e515_q_d_n6, eq34_e515_q_d_n7, eq34_e515_q_d_n10, eq34_e515_q_d_n11, eq34_e515_q_d_n12, eq34_e515_q_d_n17,) = {
    if (var_guard1225 != 0.0) {
        let eq34_e511_q: f64 = var_qbs;
        let eq34_e512: f64 = (var_ibs + var_qbs);
        let eq34_e512_d_n0: f64 = (var_ibs_dn0 + var_qbs_dn0);
        let eq34_e512_d_n2: f64 = (var_ibs_dn2 + var_qbs_dn2);
        let eq34_e512_d_n6: f64 = (var_ibs_dn6 + var_qbs_dn6);
        let eq34_e512_d_n7: f64 = (var_ibs_dn7 + var_qbs_dn7);
        let eq34_e512_d_n10: f64 = (var_ibs_dn10 + var_qbs_dn10);
        let eq34_e512_d_n11: f64 = (var_ibs_dn11 + var_qbs_dn11);
        let eq34_e512_d_n12: f64 = (var_ibs_dn12 + var_qbs_dn12);
        let eq34_e512_d_n17: f64 = (var_ibs_dn17 + var_qbs_dn17);
        let eq34_e512_q: f64 = eq34_e511_q;
        let eq34_e513: f64 = (p.p50 * eq34_e512);
        let eq34_e513_d_n0: f64 = (p.p50 * eq34_e512_d_n0);
        let eq34_e513_d_n2: f64 = (p.p50 * eq34_e512_d_n2);
        let eq34_e513_d_n6: f64 = (p.p50 * eq34_e512_d_n6);
        let eq34_e513_d_n7: f64 = (p.p50 * eq34_e512_d_n7);
        let eq34_e513_d_n10: f64 = (p.p50 * eq34_e512_d_n10);
        let eq34_e513_d_n11: f64 = (p.p50 * eq34_e512_d_n11);
        let eq34_e513_d_n12: f64 = (p.p50 * eq34_e512_d_n12);
        let eq34_e513_d_n17: f64 = (p.p50 * eq34_e512_d_n17);
        let eq34_e513_q: f64 = (p.p50 * eq34_e512_q);
        let eq34_e513_q_d_n0: f64 = (p.p50 * var_qbs_dn0);
        let eq34_e513_q_d_n2: f64 = (p.p50 * var_qbs_dn2);
        let eq34_e513_q_d_n6: f64 = (p.p50 * var_qbs_dn6);
        let eq34_e513_q_d_n7: f64 = (p.p50 * var_qbs_dn7);
        let eq34_e513_q_d_n10: f64 = (p.p50 * var_qbs_dn10);
        let eq34_e513_q_d_n11: f64 = (p.p50 * var_qbs_dn11);
        let eq34_e513_q_d_n12: f64 = (p.p50 * var_qbs_dn12);
        let eq34_e513_q_d_n17: f64 = (p.p50 * var_qbs_dn17);
        (eq34_e513, eq34_e513_d_n0, eq34_e513_d_n2, eq34_e513_d_n6, eq34_e513_d_n7, eq34_e513_d_n10, eq34_e513_d_n11, eq34_e513_d_n12, eq34_e513_d_n17, eq34_e513_q, eq34_e513_q_d_n0, eq34_e513_q_d_n2, eq34_e513_q_d_n6, eq34_e513_q_d_n7, eq34_e513_q_d_n10, eq34_e513_q_d_n11, eq34_e513_q_d_n12, eq34_e513_q_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq34_reactive_node_derivatives: [f64; 19] = [eq34_e515_q_d_n0, 0.0, eq34_e515_q_d_n2, 0.0, 0.0, 0.0, eq34_e515_q_d_n6, eq34_e515_q_d_n7, 0.0, 0.0, eq34_e515_q_d_n10, eq34_e515_q_d_n11, eq34_e515_q_d_n12, 0.0, 0.0, 0.0, 0.0, eq34_e515_q_d_n17, 0.0];
        let eq34_reactive_branch_derivatives: [f64; 15] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq34_reactive_node_derivatives,
            branches,
            &eq34_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq35_e524, eq35_e524_d_n0, eq35_e524_d_n2, eq35_e524_d_n6, eq35_e524_d_n7, eq35_e524_d_n10, eq35_e524_d_n11, eq35_e524_d_n12, eq35_e524_d_n17, eq35_e524_q, eq35_e524_q_d_n0, eq35_e524_q_d_n2, eq35_e524_q_d_n6, eq35_e524_q_d_n7, eq35_e524_q_d_n10, eq35_e524_q_d_n11, eq35_e524_q_d_n12, eq35_e524_q_d_n17,) = {
    if (var_guard1225 != 0.0) {
        let eq35_e520_q: f64 = var_qbd;
        let eq35_e521: f64 = (var_ibd + var_qbd);
        let eq35_e521_d_n0: f64 = (var_ibd_dn0 + var_qbd_dn0);
        let eq35_e521_d_n2: f64 = (var_ibd_dn2 + var_qbd_dn2);
        let eq35_e521_d_n6: f64 = (var_ibd_dn6 + var_qbd_dn6);
        let eq35_e521_d_n7: f64 = (var_ibd_dn7 + var_qbd_dn7);
        let eq35_e521_d_n10: f64 = (var_ibd_dn10 + var_qbd_dn10);
        let eq35_e521_d_n11: f64 = (var_ibd_dn11 + var_qbd_dn11);
        let eq35_e521_d_n12: f64 = (var_ibd_dn12 + var_qbd_dn12);
        let eq35_e521_d_n17: f64 = (var_ibd_dn17 + var_qbd_dn17);
        let eq35_e521_q: f64 = eq35_e520_q;
        let eq35_e522: f64 = (p.p50 * eq35_e521);
        let eq35_e522_d_n0: f64 = (p.p50 * eq35_e521_d_n0);
        let eq35_e522_d_n2: f64 = (p.p50 * eq35_e521_d_n2);
        let eq35_e522_d_n6: f64 = (p.p50 * eq35_e521_d_n6);
        let eq35_e522_d_n7: f64 = (p.p50 * eq35_e521_d_n7);
        let eq35_e522_d_n10: f64 = (p.p50 * eq35_e521_d_n10);
        let eq35_e522_d_n11: f64 = (p.p50 * eq35_e521_d_n11);
        let eq35_e522_d_n12: f64 = (p.p50 * eq35_e521_d_n12);
        let eq35_e522_d_n17: f64 = (p.p50 * eq35_e521_d_n17);
        let eq35_e522_q: f64 = (p.p50 * eq35_e521_q);
        let eq35_e522_q_d_n0: f64 = (p.p50 * var_qbd_dn0);
        let eq35_e522_q_d_n2: f64 = (p.p50 * var_qbd_dn2);
        let eq35_e522_q_d_n6: f64 = (p.p50 * var_qbd_dn6);
        let eq35_e522_q_d_n7: f64 = (p.p50 * var_qbd_dn7);
        let eq35_e522_q_d_n10: f64 = (p.p50 * var_qbd_dn10);
        let eq35_e522_q_d_n11: f64 = (p.p50 * var_qbd_dn11);
        let eq35_e522_q_d_n12: f64 = (p.p50 * var_qbd_dn12);
        let eq35_e522_q_d_n17: f64 = (p.p50 * var_qbd_dn17);
        (eq35_e522, eq35_e522_d_n0, eq35_e522_d_n2, eq35_e522_d_n6, eq35_e522_d_n7, eq35_e522_d_n10, eq35_e522_d_n11, eq35_e522_d_n12, eq35_e522_d_n17, eq35_e522_q, eq35_e522_q_d_n0, eq35_e522_q_d_n2, eq35_e522_q_d_n6, eq35_e522_q_d_n7, eq35_e522_q_d_n10, eq35_e522_q_d_n11, eq35_e522_q_d_n12, eq35_e522_q_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_reactive_node_derivatives: [f64; 19] = [eq35_e524_q_d_n0, 0.0, eq35_e524_q_d_n2, 0.0, 0.0, 0.0, eq35_e524_q_d_n6, eq35_e524_q_d_n7, 0.0, 0.0, eq35_e524_q_d_n10, eq35_e524_q_d_n11, eq35_e524_q_d_n12, 0.0, 0.0, 0.0, 0.0, eq35_e524_q_d_n17, 0.0];
        let eq35_reactive_branch_derivatives: [f64; 15] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq35_reactive_node_derivatives,
            branches,
            &eq35_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq46_e608, eq46_e608_d_n18, eq46_e608_q,) = {
    if ((var_guard1225 != 0.0) && (p.p34 != 0.0)) {
        let eq46_e603: f64 = (1e-9 / 0.0001);
        let eq46_e605: f64 = (eq46_e603 * (nv18 - 0.0));
        let eq46_e606_q: f64 = eq46_e605;
        (eq46_e605, eq46_e603, eq46_e606_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[18]),
            None,
            nodes[18],
            multiplicity * (eq46_e608_d_n18),
        );
        let (eq47_e619, eq47_e619_d_n13, eq47_e619_q,) = {
    if ((var_guard1225 != 0.0) && (p.p34 != 0.0)) {
        let eq47_e614: f64 = (1e-9 / 0.0001);
        let eq47_e616: f64 = (eq47_e614 * (nv13 - 0.0));
        let eq47_e617_q: f64 = eq47_e616;
        (eq47_e616, eq47_e614, eq47_e617_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[13]),
            None,
            nodes[13],
            multiplicity * (eq47_e619_d_n13),
        );
        let (eq52_e658, eq52_e658_d_n17, eq52_e658_q,) = {
    if ((var_guard1225 != 0.0) && (var_guard1226 != 0.0)) {
        let eq52_e653: f64 = (1e-9 / 0.0001);
        let eq52_e655: f64 = (eq52_e653 * (nv17 - 0.0));
        let eq52_e656_q: f64 = eq52_e655;
        (eq52_e655, eq52_e653, eq52_e656_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[17]),
            None,
            nodes[17],
            multiplicity * (eq52_e658_d_n17),
        );
        let (eq59_e716, eq59_e716_d_n17, eq59_e716_q,) = {
    if ((var_guard1225 == 0.0) && (p.p37 != 0.0)) {
        let eq59_e711: f64 = (1e-9 / 0.0001);
        let eq59_e713: f64 = (eq59_e711 * (nv17 - 0.0));
        let eq59_e714_q: f64 = eq59_e713;
        (eq59_e713, eq59_e711, eq59_e714_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[17]),
            None,
            nodes[17],
            multiplicity * (eq59_e716_d_n17),
        );
        let (eq67_e784, eq67_e784_d_n15, eq67_e784_q,) = {
    if ((var_guard1225 == 0.0) && (p.p34 != 0.0)) {
        let eq67_e779: f64 = (1e-9 / 0.0001);
        let eq67_e781: f64 = (eq67_e779 * (nv15 - 0.0));
        let eq67_e782_q: f64 = eq67_e781;
        (eq67_e781, eq67_e779, eq67_e782_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[15]),
            None,
            nodes[15],
            multiplicity * (eq67_e784_d_n15),
        );
        let (eq68_e796, eq68_e796_d_n16, eq68_e796_q,) = {
    if ((var_guard1225 == 0.0) && (p.p34 != 0.0)) {
        let eq68_e791: f64 = (1e-9 / 0.0001);
        let eq68_e793: f64 = (eq68_e791 * (nv16 - 0.0));
        let eq68_e794_q: f64 = eq68_e793;
        (eq68_e793, eq68_e791, eq68_e794_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[16]),
            None,
            nodes[16],
            multiplicity * (eq68_e796_d_n16),
        );
        let (eq69_e808, eq69_e808_d_n13, eq69_e808_q,) = {
    if ((var_guard1225 == 0.0) && (p.p34 != 0.0)) {
        let eq69_e803: f64 = (1e-9 / 0.0001);
        let eq69_e805: f64 = (eq69_e803 * (nv13 - 0.0));
        let eq69_e806_q: f64 = eq69_e805;
        (eq69_e805, eq69_e803, eq69_e806_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[13]),
            None,
            nodes[13],
            multiplicity * (eq69_e808_d_n13),
        );
    }
}
