#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_217(
        locals: &mut StampLocals,
    ) {
        let (assign58850_e91737, assign58850_e91737_d_n0, assign58850_e91737_d_n2, assign58850_e91737_d_n4, assign58850_e91737_d_n5, assign58850_e91737_d_n6, assign58850_e91737_d_n7, assign58850_e91737_d_n8, assign58850_e91737_d_n9, assign58850_e91737_d_n10, assign58850_e91737_d_n11, assign58850_e91737_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign58850_e91734: f64 = (locals.var_cox * locals.var_cox);
        let assign58850_e91735: f64 = (locals.var_qnsub_esi / assign58850_e91734);
        (assign58850_e91735, (((locals.var_qnsub_esi_dn0 * assign58850_e91734) - (locals.var_qnsub_esi * ((locals.var_cox_dn0 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn0)))) / (assign58850_e91734 * assign58850_e91734)), (((locals.var_qnsub_esi_dn2 * assign58850_e91734) - (locals.var_qnsub_esi * ((locals.var_cox_dn2 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn2)))) / (assign58850_e91734 * assign58850_e91734)), (((locals.var_qnsub_esi_dn4 * assign58850_e91734) - (locals.var_qnsub_esi * ((locals.var_cox_dn4 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn4)))) / (assign58850_e91734 * assign58850_e91734)), (((locals.var_qnsub_esi_dn5 * assign58850_e91734) - (locals.var_qnsub_esi * ((locals.var_cox_dn5 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn5)))) / (assign58850_e91734 * assign58850_e91734)), (((locals.var_qnsub_esi_dn6 * assign58850_e91734) - (locals.var_qnsub_esi * ((locals.var_cox_dn6 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn6)))) / (assign58850_e91734 * assign58850_e91734)), (((locals.var_qnsub_esi_dn7 * assign58850_e91734) - (locals.var_qnsub_esi * ((locals.var_cox_dn7 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn7)))) / (assign58850_e91734 * assign58850_e91734)), (((locals.var_qnsub_esi_dn8 * assign58850_e91734) - (locals.var_qnsub_esi * ((locals.var_cox_dn8 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn8)))) / (assign58850_e91734 * assign58850_e91734)), (((locals.var_qnsub_esi_dn9 * assign58850_e91734) - (locals.var_qnsub_esi * ((locals.var_cox_dn9 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn9)))) / (assign58850_e91734 * assign58850_e91734)), (((locals.var_qnsub_esi_dn10 * assign58850_e91734) - (locals.var_qnsub_esi * ((locals.var_cox_dn10 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn10)))) / (assign58850_e91734 * assign58850_e91734)), (((locals.var_qnsub_esi_dn11 * assign58850_e91734) - (locals.var_qnsub_esi * ((locals.var_cox_dn11 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn11)))) / (assign58850_e91734 * assign58850_e91734)), (((locals.var_qnsub_esi_dn14 * assign58850_e91734) - (locals.var_qnsub_esi * ((locals.var_cox_dn14 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn14)))) / (assign58850_e91734 * assign58850_e91734)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign58850_e91737;
        locals.var_t2_dn0 = assign58850_e91737_d_n0;
        locals.var_t2_dn2 = assign58850_e91737_d_n2;
        locals.var_t2_dn4 = assign58850_e91737_d_n4;
        locals.var_t2_dn5 = assign58850_e91737_d_n5;
        locals.var_t2_dn6 = assign58850_e91737_d_n6;
        locals.var_t2_dn7 = assign58850_e91737_d_n7;
        locals.var_t2_dn8 = assign58850_e91737_d_n8;
        locals.var_t2_dn9 = assign58850_e91737_d_n9;
        locals.var_t2_dn10 = assign58850_e91737_d_n10;
        locals.var_t2_dn11 = assign58850_e91737_d_n11;
        locals.var_t2_dn14 = assign58850_e91737_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign58860_e91748, assign58860_e91748_d_n0, assign58860_e91748_d_n2, assign58860_e91748_d_n4, assign58860_e91748_d_n5, assign58860_e91748_d_n6, assign58860_e91748_d_n7, assign58860_e91748_d_n8, assign58860_e91748_d_n9, assign58860_e91748_d_n10, assign58860_e91748_d_n11, assign58860_e91748_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign58860_e91744: f64 = (locals.var_vgp - locals.var_beta_inv);
        let assign58860_e91746: f64 = (assign58860_e91744 - locals.var_vbsz__blk442);
        (assign58860_e91746, ((locals.var_vgp_dn0 - locals.var_beta_inv_dn0) - locals.var_vbsz__blk442_dn0), ((locals.var_vgp_dn2 - locals.var_beta_inv_dn2) - locals.var_vbsz__blk442_dn2), ((locals.var_vgp_dn4 - locals.var_beta_inv_dn4) - locals.var_vbsz__blk442_dn4), ((locals.var_vgp_dn5 - locals.var_beta_inv_dn5) - locals.var_vbsz__blk442_dn5), ((locals.var_vgp_dn6 - locals.var_beta_inv_dn6) - locals.var_vbsz__blk442_dn6), ((locals.var_vgp_dn7 - locals.var_beta_inv_dn7) - locals.var_vbsz__blk442_dn7), ((locals.var_vgp_dn8 - locals.var_beta_inv_dn8) - locals.var_vbsz__blk442_dn8), ((locals.var_vgp_dn9 - locals.var_beta_inv_dn9) - locals.var_vbsz__blk442_dn9), ((locals.var_vgp_dn10 - locals.var_beta_inv_dn10) - locals.var_vbsz__blk442_dn10), ((locals.var_vgp_dn11 - locals.var_beta_inv_dn11) - locals.var_vbsz__blk442_dn11), ((locals.var_vgp_dn14 - locals.var_beta_inv_dn14) - locals.var_vbsz__blk442_dn14),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign58860_e91748;
        locals.var_t0_dn0 = assign58860_e91748_d_n0;
        locals.var_t0_dn2 = assign58860_e91748_d_n2;
        locals.var_t0_dn4 = assign58860_e91748_d_n4;
        locals.var_t0_dn5 = assign58860_e91748_d_n5;
        locals.var_t0_dn6 = assign58860_e91748_d_n6;
        locals.var_t0_dn7 = assign58860_e91748_d_n7;
        locals.var_t0_dn8 = assign58860_e91748_d_n8;
        locals.var_t0_dn9 = assign58860_e91748_d_n9;
        locals.var_t0_dn10 = assign58860_e91748_d_n10;
        locals.var_t0_dn11 = assign58860_e91748_d_n11;
        locals.var_t0_dn14 = assign58860_e91748_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign58870_e91761, assign58870_e91761_d_n0, assign58870_e91761_d_n2, assign58870_e91761_d_n4, assign58870_e91761_d_n5, assign58870_e91761_d_n6, assign58870_e91761_d_n7, assign58870_e91761_d_n8, assign58870_e91761_d_n9, assign58870_e91761_d_n10, assign58870_e91761_d_n11, assign58870_e91761_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign58870_e91756: f64 = (2.0 / locals.var_t2);
        let assign58870_e91758: f64 = (assign58870_e91756 * locals.var_t0);
        let assign58870_e91759: f64 = (1.0 + assign58870_e91758);
        (assign58870_e91759, (((-((2.0 * locals.var_t2_dn0) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign58870_e91756 * locals.var_t0_dn0)), (((-((2.0 * locals.var_t2_dn2) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign58870_e91756 * locals.var_t0_dn2)), (((-((2.0 * locals.var_t2_dn4) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign58870_e91756 * locals.var_t0_dn4)), (((-((2.0 * locals.var_t2_dn5) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign58870_e91756 * locals.var_t0_dn5)), (((-((2.0 * locals.var_t2_dn6) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign58870_e91756 * locals.var_t0_dn6)), (((-((2.0 * locals.var_t2_dn7) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign58870_e91756 * locals.var_t0_dn7)), (((-((2.0 * locals.var_t2_dn8) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign58870_e91756 * locals.var_t0_dn8)), (((-((2.0 * locals.var_t2_dn9) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign58870_e91756 * locals.var_t0_dn9)), (((-((2.0 * locals.var_t2_dn10) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign58870_e91756 * locals.var_t0_dn10)), (((-((2.0 * locals.var_t2_dn11) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign58870_e91756 * locals.var_t0_dn11)), (((-((2.0 * locals.var_t2_dn14) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign58870_e91756 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign58870_e91761;
        locals.var_t1_dn0 = assign58870_e91761_d_n0;
        locals.var_t1_dn2 = assign58870_e91761_d_n2;
        locals.var_t1_dn4 = assign58870_e91761_d_n4;
        locals.var_t1_dn5 = assign58870_e91761_d_n5;
        locals.var_t1_dn6 = assign58870_e91761_d_n6;
        locals.var_t1_dn7 = assign58870_e91761_d_n7;
        locals.var_t1_dn8 = assign58870_e91761_d_n8;
        locals.var_t1_dn9 = assign58870_e91761_d_n9;
        locals.var_t1_dn10 = assign58870_e91761_d_n10;
        locals.var_t1_dn11 = assign58870_e91761_d_n11;
        locals.var_t1_dn14 = assign58870_e91761_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign58880_e91777, assign58880_e91777_d_n0, assign58880_e91777_d_n2, assign58880_e91777_d_n4, assign58880_e91777_d_n5, assign58880_e91777_d_n6, assign58880_e91777_d_n7, assign58880_e91777_d_n8, assign58880_e91777_d_n9, assign58880_e91777_d_n10, assign58880_e91777_d_n11, assign58880_e91777_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign58880_e91768: f64 = (locals.var_t1 * locals.var_t1);
        let assign58880_e91771: f64 = (4.0 * 0.001);
        let assign58880_e91773: f64 = (assign58880_e91771 * 0.001);
        let assign58880_e91774: f64 = (assign58880_e91768 + assign58880_e91773);
        let assign58880_e91775: f64 = (assign58880_e91774).sqrt();
        (assign58880_e91775, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign58880_e91775)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign58880_e91775)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign58880_e91775)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign58880_e91775)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign58880_e91775)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign58880_e91775)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign58880_e91775)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign58880_e91775)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign58880_e91775)), (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign58880_e91775)), (((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) / (2.0 * assign58880_e91775)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign58880_e91777;
        locals.var_tmf2_dn0 = assign58880_e91777_d_n0;
        locals.var_tmf2_dn2 = assign58880_e91777_d_n2;
        locals.var_tmf2_dn4 = assign58880_e91777_d_n4;
        locals.var_tmf2_dn5 = assign58880_e91777_d_n5;
        locals.var_tmf2_dn6 = assign58880_e91777_d_n6;
        locals.var_tmf2_dn7 = assign58880_e91777_d_n7;
        locals.var_tmf2_dn8 = assign58880_e91777_d_n8;
        locals.var_tmf2_dn9 = assign58880_e91777_d_n9;
        locals.var_tmf2_dn10 = assign58880_e91777_d_n10;
        locals.var_tmf2_dn11 = assign58880_e91777_d_n11;
        locals.var_tmf2_dn14 = assign58880_e91777_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign58890_e91790, assign58890_e91790_d_n0, assign58890_e91790_d_n2, assign58890_e91790_d_n4, assign58890_e91790_d_n5, assign58890_e91790_d_n6, assign58890_e91790_d_n7, assign58890_e91790_d_n8, assign58890_e91790_d_n9, assign58890_e91790_d_n10, assign58890_e91790_d_n11, assign58890_e91790_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign58890_e91786: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign58890_e91787: f64 = (1.0 + assign58890_e91786);
        let assign58890_e91788: f64 = (0.5 * assign58890_e91787);
        (assign58890_e91788, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn7 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn9 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn11 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn14 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign58890_e91790;
        locals.var_tx_dn0 = assign58890_e91790_d_n0;
        locals.var_tx_dn2 = assign58890_e91790_d_n2;
        locals.var_tx_dn4 = assign58890_e91790_d_n4;
        locals.var_tx_dn5 = assign58890_e91790_d_n5;
        locals.var_tx_dn6 = assign58890_e91790_d_n6;
        locals.var_tx_dn7 = assign58890_e91790_d_n7;
        locals.var_tx_dn8 = assign58890_e91790_d_n8;
        locals.var_tx_dn9 = assign58890_e91790_d_n9;
        locals.var_tx_dn10 = assign58890_e91790_d_n10;
        locals.var_tx_dn11 = assign58890_e91790_d_n11;
        locals.var_tx_dn14 = assign58890_e91790_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign58900_e91801, assign58900_e91801_d_n0, assign58900_e91801_d_n2, assign58900_e91801_d_n4, assign58900_e91801_d_n5, assign58900_e91801_d_n6, assign58900_e91801_d_n7, assign58900_e91801_d_n8, assign58900_e91801_d_n9, assign58900_e91801_d_n10, assign58900_e91801_d_n11, assign58900_e91801_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign58900_e91798: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign58900_e91799: f64 = (0.5 * assign58900_e91798);
        (assign58900_e91799, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign58900_e91801;
        locals.var_t9_dn0 = assign58900_e91801_d_n0;
        locals.var_t9_dn2 = assign58900_e91801_d_n2;
        locals.var_t9_dn4 = assign58900_e91801_d_n4;
        locals.var_t9_dn5 = assign58900_e91801_d_n5;
        locals.var_t9_dn6 = assign58900_e91801_d_n6;
        locals.var_t9_dn7 = assign58900_e91801_d_n7;
        locals.var_t9_dn8 = assign58900_e91801_d_n8;
        locals.var_t9_dn9 = assign58900_e91801_d_n9;
        locals.var_t9_dn10 = assign58900_e91801_d_n10;
        locals.var_t9_dn11 = assign58900_e91801_d_n11;
        locals.var_t9_dn14 = assign58900_e91801_d_n14;
        locals.var_t9_rv = 0.0;

        let assign58910_e91804: f64 = if locals.var_t9 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1449 = assign58910_e91804;
        locals.var_guard1449_rv = 0.0;

        let (assign58920_e91813, assign58920_e91813_d_n0, assign58920_e91813_d_n2, assign58920_e91813_d_n4, assign58920_e91813_d_n5, assign58920_e91813_d_n6, assign58920_e91813_d_n7, assign58920_e91813_d_n8, assign58920_e91813_d_n9, assign58920_e91813_d_n10, assign58920_e91813_d_n11, assign58920_e91813_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1449 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign58920_e91813;
        locals.var_t9_dn0 = assign58920_e91813_d_n0;
        locals.var_t9_dn2 = assign58920_e91813_d_n2;
        locals.var_t9_dn4 = assign58920_e91813_d_n4;
        locals.var_t9_dn5 = assign58920_e91813_d_n5;
        locals.var_t9_dn6 = assign58920_e91813_d_n6;
        locals.var_t9_dn7 = assign58920_e91813_d_n7;
        locals.var_t9_dn8 = assign58920_e91813_d_n8;
        locals.var_t9_dn9 = assign58920_e91813_d_n9;
        locals.var_t9_dn10 = assign58920_e91813_d_n10;
        locals.var_t9_dn11 = assign58920_e91813_d_n11;
        locals.var_t9_dn14 = assign58920_e91813_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign58930_e91822, assign58930_e91822_d_n0, assign58930_e91822_d_n2, assign58930_e91822_d_n4, assign58930_e91822_d_n5, assign58930_e91822_d_n6, assign58930_e91822_d_n7, assign58930_e91822_d_n8, assign58930_e91822_d_n9, assign58930_e91822_d_n10, assign58930_e91822_d_n11, assign58930_e91822_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1449 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign58930_e91822;
        locals.var_tx_dn0 = assign58930_e91822_d_n0;
        locals.var_tx_dn2 = assign58930_e91822_d_n2;
        locals.var_tx_dn4 = assign58930_e91822_d_n4;
        locals.var_tx_dn5 = assign58930_e91822_d_n5;
        locals.var_tx_dn6 = assign58930_e91822_d_n6;
        locals.var_tx_dn7 = assign58930_e91822_d_n7;
        locals.var_tx_dn8 = assign58930_e91822_d_n8;
        locals.var_tx_dn9 = assign58930_e91822_d_n9;
        locals.var_tx_dn10 = assign58930_e91822_d_n10;
        locals.var_tx_dn11 = assign58930_e91822_d_n11;
        locals.var_tx_dn14 = assign58930_e91822_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign58940_e91831, assign58940_e91831_d_n0, assign58940_e91831_d_n2, assign58940_e91831_d_n4, assign58940_e91831_d_n5, assign58940_e91831_d_n6, assign58940_e91831_d_n7, assign58940_e91831_d_n8, assign58940_e91831_d_n9, assign58940_e91831_d_n10, assign58940_e91831_d_n11, assign58940_e91831_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign58940_e91829: f64 = (locals.var_t9 + 1e-25);
        (assign58940_e91829, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign58940_e91831;
        locals.var_t9_dn0 = assign58940_e91831_d_n0;
        locals.var_t9_dn2 = assign58940_e91831_d_n2;
        locals.var_t9_dn4 = assign58940_e91831_d_n4;
        locals.var_t9_dn5 = assign58940_e91831_d_n5;
        locals.var_t9_dn6 = assign58940_e91831_d_n6;
        locals.var_t9_dn7 = assign58940_e91831_d_n7;
        locals.var_t9_dn8 = assign58940_e91831_d_n8;
        locals.var_t9_dn9 = assign58940_e91831_d_n9;
        locals.var_t9_dn10 = assign58940_e91831_d_n10;
        locals.var_t9_dn11 = assign58940_e91831_d_n11;
        locals.var_t9_dn14 = assign58940_e91831_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign58950_e91839, assign58950_e91839_d_n0, assign58950_e91839_d_n2, assign58950_e91839_d_n4, assign58950_e91839_d_n5, assign58950_e91839_d_n6, assign58950_e91839_d_n7, assign58950_e91839_d_n8, assign58950_e91839_d_n9, assign58950_e91839_d_n10, assign58950_e91839_d_n11, assign58950_e91839_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign58950_e91837: f64 = (locals.var_t9).sqrt();
        (assign58950_e91837, (locals.var_t9_dn0 / (2.0 * assign58950_e91837)), (locals.var_t9_dn2 / (2.0 * assign58950_e91837)), (locals.var_t9_dn4 / (2.0 * assign58950_e91837)), (locals.var_t9_dn5 / (2.0 * assign58950_e91837)), (locals.var_t9_dn6 / (2.0 * assign58950_e91837)), (locals.var_t9_dn7 / (2.0 * assign58950_e91837)), (locals.var_t9_dn8 / (2.0 * assign58950_e91837)), (locals.var_t9_dn9 / (2.0 * assign58950_e91837)), (locals.var_t9_dn10 / (2.0 * assign58950_e91837)), (locals.var_t9_dn11 / (2.0 * assign58950_e91837)), (locals.var_t9_dn14 / (2.0 * assign58950_e91837)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign58950_e91839;
        locals.var_t3_dn0 = assign58950_e91839_d_n0;
        locals.var_t3_dn2 = assign58950_e91839_d_n2;
        locals.var_t3_dn4 = assign58950_e91839_d_n4;
        locals.var_t3_dn5 = assign58950_e91839_d_n5;
        locals.var_t3_dn6 = assign58950_e91839_d_n6;
        locals.var_t3_dn7 = assign58950_e91839_d_n7;
        locals.var_t3_dn8 = assign58950_e91839_d_n8;
        locals.var_t3_dn9 = assign58950_e91839_d_n9;
        locals.var_t3_dn10 = assign58950_e91839_d_n10;
        locals.var_t3_dn11 = assign58950_e91839_d_n11;
        locals.var_t3_dn14 = assign58950_e91839_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign58960_e91852, assign58960_e91852_d_n0, assign58960_e91852_d_n2, assign58960_e91852_d_n4, assign58960_e91852_d_n5, assign58960_e91852_d_n6, assign58960_e91852_d_n7, assign58960_e91852_d_n8, assign58960_e91852_d_n9, assign58960_e91852_d_n10, assign58960_e91852_d_n11, assign58960_e91852_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign58960_e91848: f64 = (1.0 - locals.var_t3);
        let assign58960_e91849: f64 = (locals.var_t2 * assign58960_e91848);
        let assign58960_e91850: f64 = (locals.var_vgp + assign58960_e91849);
        (assign58960_e91850, (locals.var_vgp_dn0 + ((locals.var_t2_dn0 * assign58960_e91848) + (locals.var_t2 * (-locals.var_t3_dn0)))), (locals.var_vgp_dn2 + ((locals.var_t2_dn2 * assign58960_e91848) + (locals.var_t2 * (-locals.var_t3_dn2)))), (locals.var_vgp_dn4 + ((locals.var_t2_dn4 * assign58960_e91848) + (locals.var_t2 * (-locals.var_t3_dn4)))), (locals.var_vgp_dn5 + ((locals.var_t2_dn5 * assign58960_e91848) + (locals.var_t2 * (-locals.var_t3_dn5)))), (locals.var_vgp_dn6 + ((locals.var_t2_dn6 * assign58960_e91848) + (locals.var_t2 * (-locals.var_t3_dn6)))), (locals.var_vgp_dn7 + ((locals.var_t2_dn7 * assign58960_e91848) + (locals.var_t2 * (-locals.var_t3_dn7)))), (locals.var_vgp_dn8 + ((locals.var_t2_dn8 * assign58960_e91848) + (locals.var_t2 * (-locals.var_t3_dn8)))), (locals.var_vgp_dn9 + ((locals.var_t2_dn9 * assign58960_e91848) + (locals.var_t2 * (-locals.var_t3_dn9)))), (locals.var_vgp_dn10 + ((locals.var_t2_dn10 * assign58960_e91848) + (locals.var_t2 * (-locals.var_t3_dn10)))), (locals.var_vgp_dn11 + ((locals.var_t2_dn11 * assign58960_e91848) + (locals.var_t2 * (-locals.var_t3_dn11)))), (locals.var_vgp_dn14 + ((locals.var_t2_dn14 * assign58960_e91848) + (locals.var_t2 * (-locals.var_t3_dn14)))),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign58960_e91852;
        locals.var_t10_dn0 = assign58960_e91852_d_n0;
        locals.var_t10_dn2 = assign58960_e91852_d_n2;
        locals.var_t10_dn4 = assign58960_e91852_d_n4;
        locals.var_t10_dn5 = assign58960_e91852_d_n5;
        locals.var_t10_dn6 = assign58960_e91852_d_n6;
        locals.var_t10_dn7 = assign58960_e91852_d_n7;
        locals.var_t10_dn8 = assign58960_e91852_d_n8;
        locals.var_t10_dn9 = assign58960_e91852_d_n9;
        locals.var_t10_dn10 = assign58960_e91852_d_n10;
        locals.var_t10_dn11 = assign58960_e91852_d_n11;
        locals.var_t10_dn14 = assign58960_e91852_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign58970_e91868, assign58970_e91868_d_n0, assign58970_e91868_d_n2, assign58970_e91868_d_n4, assign58970_e91868_d_n5, assign58970_e91868_d_n6, assign58970_e91868_d_n7, assign58970_e91868_d_n8, assign58970_e91868_d_n9, assign58970_e91868_d_n10, assign58970_e91868_d_n11, assign58970_e91868_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign58970_e91859: f64 = (locals.var_t10 * locals.var_t10);
        let assign58970_e91862: f64 = (4.0 * 0.01);
        let assign58970_e91864: f64 = (assign58970_e91862 * 0.01);
        let assign58970_e91865: f64 = (assign58970_e91859 + assign58970_e91864);
        let assign58970_e91866: f64 = (assign58970_e91865).sqrt();
        (assign58970_e91866, (((locals.var_t10_dn0 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn0)) / (2.0 * assign58970_e91866)), (((locals.var_t10_dn2 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn2)) / (2.0 * assign58970_e91866)), (((locals.var_t10_dn4 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn4)) / (2.0 * assign58970_e91866)), (((locals.var_t10_dn5 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn5)) / (2.0 * assign58970_e91866)), (((locals.var_t10_dn6 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn6)) / (2.0 * assign58970_e91866)), (((locals.var_t10_dn7 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn7)) / (2.0 * assign58970_e91866)), (((locals.var_t10_dn8 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn8)) / (2.0 * assign58970_e91866)), (((locals.var_t10_dn9 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn9)) / (2.0 * assign58970_e91866)), (((locals.var_t10_dn10 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn10)) / (2.0 * assign58970_e91866)), (((locals.var_t10_dn11 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn11)) / (2.0 * assign58970_e91866)), (((locals.var_t10_dn14 * locals.var_t10) + (locals.var_t10 * locals.var_t10_dn14)) / (2.0 * assign58970_e91866)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign58970_e91868;
        locals.var_tmf2_dn0 = assign58970_e91868_d_n0;
        locals.var_tmf2_dn2 = assign58970_e91868_d_n2;
        locals.var_tmf2_dn4 = assign58970_e91868_d_n4;
        locals.var_tmf2_dn5 = assign58970_e91868_d_n5;
        locals.var_tmf2_dn6 = assign58970_e91868_d_n6;
        locals.var_tmf2_dn7 = assign58970_e91868_d_n7;
        locals.var_tmf2_dn8 = assign58970_e91868_d_n8;
        locals.var_tmf2_dn9 = assign58970_e91868_d_n9;
        locals.var_tmf2_dn10 = assign58970_e91868_d_n10;
        locals.var_tmf2_dn11 = assign58970_e91868_d_n11;
        locals.var_tmf2_dn14 = assign58970_e91868_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign58980_e91881, assign58980_e91881_d_n0, assign58980_e91881_d_n2, assign58980_e91881_d_n4, assign58980_e91881_d_n5, assign58980_e91881_d_n6, assign58980_e91881_d_n7, assign58980_e91881_d_n8, assign58980_e91881_d_n9, assign58980_e91881_d_n10, assign58980_e91881_d_n11, assign58980_e91881_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign58980_e91877: f64 = (locals.var_t10 / locals.var_tmf2);
        let assign58980_e91878: f64 = (1.0 + assign58980_e91877);
        let assign58980_e91879: f64 = (0.5 * assign58980_e91878);
        (assign58980_e91879, (0.5 * (((locals.var_t10_dn0 * locals.var_tmf2) - (locals.var_t10 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t10_dn2 * locals.var_tmf2) - (locals.var_t10 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t10_dn4 * locals.var_tmf2) - (locals.var_t10 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t10_dn5 * locals.var_tmf2) - (locals.var_t10 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t10_dn6 * locals.var_tmf2) - (locals.var_t10 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t10_dn7 * locals.var_tmf2) - (locals.var_t10 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t10_dn8 * locals.var_tmf2) - (locals.var_t10 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t10_dn9 * locals.var_tmf2) - (locals.var_t10 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t10_dn10 * locals.var_tmf2) - (locals.var_t10 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t10_dn11 * locals.var_tmf2) - (locals.var_t10 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t10_dn14 * locals.var_tmf2) - (locals.var_t10 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign58980_e91881;
        locals.var_t0_dn0 = assign58980_e91881_d_n0;
        locals.var_t0_dn2 = assign58980_e91881_d_n2;
        locals.var_t0_dn4 = assign58980_e91881_d_n4;
        locals.var_t0_dn5 = assign58980_e91881_d_n5;
        locals.var_t0_dn6 = assign58980_e91881_d_n6;
        locals.var_t0_dn7 = assign58980_e91881_d_n7;
        locals.var_t0_dn8 = assign58980_e91881_d_n8;
        locals.var_t0_dn9 = assign58980_e91881_d_n9;
        locals.var_t0_dn10 = assign58980_e91881_d_n10;
        locals.var_t0_dn11 = assign58980_e91881_d_n11;
        locals.var_t0_dn14 = assign58980_e91881_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign58990_e91892, assign58990_e91892_d_n0, assign58990_e91892_d_n2, assign58990_e91892_d_n4, assign58990_e91892_d_n5, assign58990_e91892_d_n6, assign58990_e91892_d_n7, assign58990_e91892_d_n8, assign58990_e91892_d_n9, assign58990_e91892_d_n10, assign58990_e91892_d_n11, assign58990_e91892_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign58990_e91889: f64 = (locals.var_t10 + locals.var_tmf2);
        let assign58990_e91890: f64 = (0.5 * assign58990_e91889);
        (assign58990_e91890, (0.5 * (locals.var_t10_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t10_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t10_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t10_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t10_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t10_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t10_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t10_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t10_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t10_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t10_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign58990_e91892;
        locals.var_t10_dn0 = assign58990_e91892_d_n0;
        locals.var_t10_dn2 = assign58990_e91892_d_n2;
        locals.var_t10_dn4 = assign58990_e91892_d_n4;
        locals.var_t10_dn5 = assign58990_e91892_d_n5;
        locals.var_t10_dn6 = assign58990_e91892_d_n6;
        locals.var_t10_dn7 = assign58990_e91892_d_n7;
        locals.var_t10_dn8 = assign58990_e91892_d_n8;
        locals.var_t10_dn9 = assign58990_e91892_d_n9;
        locals.var_t10_dn10 = assign58990_e91892_d_n10;
        locals.var_t10_dn11 = assign58990_e91892_d_n11;
        locals.var_t10_dn14 = assign58990_e91892_d_n14;
        locals.var_t10_rv = 0.0;

        let assign59000_e91895: f64 = if locals.var_t10 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1450 = assign59000_e91895;
        locals.var_guard1450_rv = 0.0;

        let (assign59010_e91904, assign59010_e91904_d_n0, assign59010_e91904_d_n2, assign59010_e91904_d_n4, assign59010_e91904_d_n5, assign59010_e91904_d_n6, assign59010_e91904_d_n7, assign59010_e91904_d_n8, assign59010_e91904_d_n9, assign59010_e91904_d_n10, assign59010_e91904_d_n11, assign59010_e91904_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1450 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign59010_e91904;
        locals.var_t10_dn0 = assign59010_e91904_d_n0;
        locals.var_t10_dn2 = assign59010_e91904_d_n2;
        locals.var_t10_dn4 = assign59010_e91904_d_n4;
        locals.var_t10_dn5 = assign59010_e91904_d_n5;
        locals.var_t10_dn6 = assign59010_e91904_d_n6;
        locals.var_t10_dn7 = assign59010_e91904_d_n7;
        locals.var_t10_dn8 = assign59010_e91904_d_n8;
        locals.var_t10_dn9 = assign59010_e91904_d_n9;
        locals.var_t10_dn10 = assign59010_e91904_d_n10;
        locals.var_t10_dn11 = assign59010_e91904_d_n11;
        locals.var_t10_dn14 = assign59010_e91904_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign59020_e91913, assign59020_e91913_d_n0, assign59020_e91913_d_n2, assign59020_e91913_d_n4, assign59020_e91913_d_n5, assign59020_e91913_d_n6, assign59020_e91913_d_n7, assign59020_e91913_d_n8, assign59020_e91913_d_n9, assign59020_e91913_d_n10, assign59020_e91913_d_n11, assign59020_e91913_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1450 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign59020_e91913;
        locals.var_t0_dn0 = assign59020_e91913_d_n0;
        locals.var_t0_dn2 = assign59020_e91913_d_n2;
        locals.var_t0_dn4 = assign59020_e91913_d_n4;
        locals.var_t0_dn5 = assign59020_e91913_d_n5;
        locals.var_t0_dn6 = assign59020_e91913_d_n6;
        locals.var_t0_dn7 = assign59020_e91913_d_n7;
        locals.var_t0_dn8 = assign59020_e91913_d_n8;
        locals.var_t0_dn9 = assign59020_e91913_d_n9;
        locals.var_t0_dn10 = assign59020_e91913_d_n10;
        locals.var_t0_dn11 = assign59020_e91913_d_n11;
        locals.var_t0_dn14 = assign59020_e91913_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign59030_e91924, assign59030_e91924_d_n0, assign59030_e91924_d_n2, assign59030_e91924_d_n4, assign59030_e91924_d_n5, assign59030_e91924_d_n6, assign59030_e91924_d_n7, assign59030_e91924_d_n8, assign59030_e91924_d_n9, assign59030_e91924_d_n10, assign59030_e91924_d_n11, assign59030_e91924_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign59030_e91921: f64 = (10.0 * 2.220446049250313e-16);
        let assign59030_e91922: f64 = (locals.var_t10 + assign59030_e91921);
        (assign59030_e91922, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign59030_e91924;
        locals.var_t10_dn0 = assign59030_e91924_d_n0;
        locals.var_t10_dn2 = assign59030_e91924_d_n2;
        locals.var_t10_dn4 = assign59030_e91924_d_n4;
        locals.var_t10_dn5 = assign59030_e91924_d_n5;
        locals.var_t10_dn6 = assign59030_e91924_d_n6;
        locals.var_t10_dn7 = assign59030_e91924_d_n7;
        locals.var_t10_dn8 = assign59030_e91924_d_n8;
        locals.var_t10_dn9 = assign59030_e91924_d_n9;
        locals.var_t10_dn10 = assign59030_e91924_d_n10;
        locals.var_t10_dn11 = assign59030_e91924_d_n11;
        locals.var_t10_dn14 = assign59030_e91924_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign59040_e91933, assign59040_e91933_d_n0, assign59040_e91933_d_n2, assign59040_e91933_d_n4, assign59040_e91933_d_n5, assign59040_e91933_d_n6, assign59040_e91933_d_n7, assign59040_e91933_d_n8, assign59040_e91933_d_n9, assign59040_e91933_d_n10, assign59040_e91933_d_n11, assign59040_e91933_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign59040_e91931: f64 = (locals.var_vds / locals.var_t10);
        (assign59040_e91931, (((locals.var_vds_dn0 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn0)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn2 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn2)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn4 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn4)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn5 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn5)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn6 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn6)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn7 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn7)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn8 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn8)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn9 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn9)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn10 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn10)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn11 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn11)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_dn14 * locals.var_t10) - (locals.var_vds * locals.var_t10_dn14)) / (locals.var_t10 * locals.var_t10)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign59040_e91933;
        locals.var_t1_dn0 = assign59040_e91933_d_n0;
        locals.var_t1_dn2 = assign59040_e91933_d_n2;
        locals.var_t1_dn4 = assign59040_e91933_d_n4;
        locals.var_t1_dn5 = assign59040_e91933_d_n5;
        locals.var_t1_dn6 = assign59040_e91933_d_n6;
        locals.var_t1_dn7 = assign59040_e91933_d_n7;
        locals.var_t1_dn8 = assign59040_e91933_d_n8;
        locals.var_t1_dn9 = assign59040_e91933_d_n9;
        locals.var_t1_dn10 = assign59040_e91933_d_n10;
        locals.var_t1_dn11 = assign59040_e91933_d_n11;
        locals.var_t1_dn14 = assign59040_e91933_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign59050_e91949, assign59050_e91949_d_n0, assign59050_e91949_d_n2, assign59050_e91949_d_n4, assign59050_e91949_d_n5, assign59050_e91949_d_n6, assign59050_e91949_d_n7, assign59050_e91949_d_n8, assign59050_e91949_d_n9, assign59050_e91949_d_n10, assign59050_e91949_d_n11, assign59050_e91949_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let (assign59050_e91947, assign59050_e91947_d_n0, assign59050_e91947_d_n2, assign59050_e91947_d_n4, assign59050_e91947_d_n5, assign59050_e91947_d_n6, assign59050_e91947_d_n7, assign59050_e91947_d_n8, assign59050_e91947_d_n9, assign59050_e91947_d_n10, assign59050_e91947_d_n11, assign59050_e91947_d_n14,) = {
            if (locals.var_t1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign59050_e91945: f64 = (locals.var_ddlte - 1.0);
                let assign59050_e91946: f64 = (locals.var_t1).powf(assign59050_e91945);
                (assign59050_e91946, if locals.var_ddlte_dn0 == 0.0 && ((assign59050_e91945) as f64).is_finite() && ((assign59050_e91945) as f64).fract() == 0.0 { if assign59050_e91945 == 0.0 { 0.0 } else { (assign59050_e91945 * ((locals.var_t1).powf(assign59050_e91945 - 1.0) * locals.var_t1_dn0)) } } else { (assign59050_e91946 * ((locals.var_ddlte_dn0 * (locals.var_t1).ln()) + (assign59050_e91945 * (locals.var_t1_dn0 / locals.var_t1)))) }, if locals.var_ddlte_dn2 == 0.0 && ((assign59050_e91945) as f64).is_finite() && ((assign59050_e91945) as f64).fract() == 0.0 { if assign59050_e91945 == 0.0 { 0.0 } else { (assign59050_e91945 * ((locals.var_t1).powf(assign59050_e91945 - 1.0) * locals.var_t1_dn2)) } } else { (assign59050_e91946 * ((locals.var_ddlte_dn2 * (locals.var_t1).ln()) + (assign59050_e91945 * (locals.var_t1_dn2 / locals.var_t1)))) }, if locals.var_ddlte_dn4 == 0.0 && ((assign59050_e91945) as f64).is_finite() && ((assign59050_e91945) as f64).fract() == 0.0 { if assign59050_e91945 == 0.0 { 0.0 } else { (assign59050_e91945 * ((locals.var_t1).powf(assign59050_e91945 - 1.0) * locals.var_t1_dn4)) } } else { (assign59050_e91946 * ((locals.var_ddlte_dn4 * (locals.var_t1).ln()) + (assign59050_e91945 * (locals.var_t1_dn4 / locals.var_t1)))) }, if locals.var_ddlte_dn5 == 0.0 && ((assign59050_e91945) as f64).is_finite() && ((assign59050_e91945) as f64).fract() == 0.0 { if assign59050_e91945 == 0.0 { 0.0 } else { (assign59050_e91945 * ((locals.var_t1).powf(assign59050_e91945 - 1.0) * locals.var_t1_dn5)) } } else { (assign59050_e91946 * ((locals.var_ddlte_dn5 * (locals.var_t1).ln()) + (assign59050_e91945 * (locals.var_t1_dn5 / locals.var_t1)))) }, if locals.var_ddlte_dn6 == 0.0 && ((assign59050_e91945) as f64).is_finite() && ((assign59050_e91945) as f64).fract() == 0.0 { if assign59050_e91945 == 0.0 { 0.0 } else { (assign59050_e91945 * ((locals.var_t1).powf(assign59050_e91945 - 1.0) * locals.var_t1_dn6)) } } else { (assign59050_e91946 * ((locals.var_ddlte_dn6 * (locals.var_t1).ln()) + (assign59050_e91945 * (locals.var_t1_dn6 / locals.var_t1)))) }, if locals.var_ddlte_dn7 == 0.0 && ((assign59050_e91945) as f64).is_finite() && ((assign59050_e91945) as f64).fract() == 0.0 { if assign59050_e91945 == 0.0 { 0.0 } else { (assign59050_e91945 * ((locals.var_t1).powf(assign59050_e91945 - 1.0) * locals.var_t1_dn7)) } } else { (assign59050_e91946 * ((locals.var_ddlte_dn7 * (locals.var_t1).ln()) + (assign59050_e91945 * (locals.var_t1_dn7 / locals.var_t1)))) }, if locals.var_ddlte_dn8 == 0.0 && ((assign59050_e91945) as f64).is_finite() && ((assign59050_e91945) as f64).fract() == 0.0 { if assign59050_e91945 == 0.0 { 0.0 } else { (assign59050_e91945 * ((locals.var_t1).powf(assign59050_e91945 - 1.0) * locals.var_t1_dn8)) } } else { (assign59050_e91946 * ((locals.var_ddlte_dn8 * (locals.var_t1).ln()) + (assign59050_e91945 * (locals.var_t1_dn8 / locals.var_t1)))) }, if locals.var_ddlte_dn9 == 0.0 && ((assign59050_e91945) as f64).is_finite() && ((assign59050_e91945) as f64).fract() == 0.0 { if assign59050_e91945 == 0.0 { 0.0 } else { (assign59050_e91945 * ((locals.var_t1).powf(assign59050_e91945 - 1.0) * locals.var_t1_dn9)) } } else { (assign59050_e91946 * ((locals.var_ddlte_dn9 * (locals.var_t1).ln()) + (assign59050_e91945 * (locals.var_t1_dn9 / locals.var_t1)))) }, if locals.var_ddlte_dn10 == 0.0 && ((assign59050_e91945) as f64).is_finite() && ((assign59050_e91945) as f64).fract() == 0.0 { if assign59050_e91945 == 0.0 { 0.0 } else { (assign59050_e91945 * ((locals.var_t1).powf(assign59050_e91945 - 1.0) * locals.var_t1_dn10)) } } else { (assign59050_e91946 * ((locals.var_ddlte_dn10 * (locals.var_t1).ln()) + (assign59050_e91945 * (locals.var_t1_dn10 / locals.var_t1)))) }, if locals.var_ddlte_dn11 == 0.0 && ((assign59050_e91945) as f64).is_finite() && ((assign59050_e91945) as f64).fract() == 0.0 { if assign59050_e91945 == 0.0 { 0.0 } else { (assign59050_e91945 * ((locals.var_t1).powf(assign59050_e91945 - 1.0) * locals.var_t1_dn11)) } } else { (assign59050_e91946 * ((locals.var_ddlte_dn11 * (locals.var_t1).ln()) + (assign59050_e91945 * (locals.var_t1_dn11 / locals.var_t1)))) }, if locals.var_ddlte_dn14 == 0.0 && ((assign59050_e91945) as f64).is_finite() && ((assign59050_e91945) as f64).fract() == 0.0 { if assign59050_e91945 == 0.0 { 0.0 } else { (assign59050_e91945 * ((locals.var_t1).powf(assign59050_e91945 - 1.0) * locals.var_t1_dn14)) } } else { (assign59050_e91946 * ((locals.var_ddlte_dn14 * (locals.var_t1).ln()) + (assign59050_e91945 * (locals.var_t1_dn14 / locals.var_t1)))) },)
            }
        };
        (assign59050_e91947, assign59050_e91947_d_n0, assign59050_e91947_d_n2, assign59050_e91947_d_n4, assign59050_e91947_d_n5, assign59050_e91947_d_n6, assign59050_e91947_d_n7, assign59050_e91947_d_n8, assign59050_e91947_d_n9, assign59050_e91947_d_n10, assign59050_e91947_d_n11, assign59050_e91947_d_n14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign59050_e91949;
        locals.var_t2_dn0 = assign59050_e91949_d_n0;
        locals.var_t2_dn2 = assign59050_e91949_d_n2;
        locals.var_t2_dn4 = assign59050_e91949_d_n4;
        locals.var_t2_dn5 = assign59050_e91949_d_n5;
        locals.var_t2_dn6 = assign59050_e91949_d_n6;
        locals.var_t2_dn7 = assign59050_e91949_d_n7;
        locals.var_t2_dn8 = assign59050_e91949_d_n8;
        locals.var_t2_dn9 = assign59050_e91949_d_n9;
        locals.var_t2_dn10 = assign59050_e91949_d_n10;
        locals.var_t2_dn11 = assign59050_e91949_d_n11;
        locals.var_t2_dn14 = assign59050_e91949_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign59060_e91960, assign59060_e91960_d_n0, assign59060_e91960_d_n2, assign59060_e91960_d_n4, assign59060_e91960_d_n5, assign59060_e91960_d_n6, assign59060_e91960_d_n7, assign59060_e91960_d_n8, assign59060_e91960_d_n9, assign59060_e91960_d_n10, assign59060_e91960_d_n11, assign59060_e91960_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign59060_e91957: f64 = (locals.var_t2 * locals.var_t1);
        let assign59060_e91958: f64 = (1.0 + assign59060_e91957);
        (assign59060_e91958, ((locals.var_t2_dn0 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn0)), ((locals.var_t2_dn2 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn2)), ((locals.var_t2_dn4 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn4)), ((locals.var_t2_dn5 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn5)), ((locals.var_t2_dn6 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn6)), ((locals.var_t2_dn7 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn7)), ((locals.var_t2_dn8 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn8)), ((locals.var_t2_dn9 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn9)), ((locals.var_t2_dn10 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn10)), ((locals.var_t2_dn11 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn11)), ((locals.var_t2_dn14 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign59060_e91960;
        locals.var_t3_dn0 = assign59060_e91960_d_n0;
        locals.var_t3_dn2 = assign59060_e91960_d_n2;
        locals.var_t3_dn4 = assign59060_e91960_d_n4;
        locals.var_t3_dn5 = assign59060_e91960_d_n5;
        locals.var_t3_dn6 = assign59060_e91960_d_n6;
        locals.var_t3_dn7 = assign59060_e91960_d_n7;
        locals.var_t3_dn8 = assign59060_e91960_d_n8;
        locals.var_t3_dn9 = assign59060_e91960_d_n9;
        locals.var_t3_dn10 = assign59060_e91960_d_n10;
        locals.var_t3_dn11 = assign59060_e91960_d_n11;
        locals.var_t3_dn14 = assign59060_e91960_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign59070_e91978, assign59070_e91978_d_n0, assign59070_e91978_d_n2, assign59070_e91978_d_n4, assign59070_e91978_d_n5, assign59070_e91978_d_n6, assign59070_e91978_d_n7, assign59070_e91978_d_n8, assign59070_e91978_d_n9, assign59070_e91978_d_n10, assign59070_e91978_d_n11, assign59070_e91978_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let (assign59070_e91976, assign59070_e91976_d_n0, assign59070_e91976_d_n2, assign59070_e91976_d_n4, assign59070_e91976_d_n5, assign59070_e91976_d_n6, assign59070_e91976_d_n7, assign59070_e91976_d_n8, assign59070_e91976_d_n9, assign59070_e91976_d_n10, assign59070_e91976_d_n11, assign59070_e91976_d_n14,) = {
            if (locals.var_t3 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign59070_e91972: f64 = (1.0 / locals.var_ddlte);
                let assign59070_e91974: f64 = (assign59070_e91972 - 1.0);
                let assign59070_e91975: f64 = (locals.var_t3).powf(assign59070_e91974);
                (assign59070_e91975, if (-(locals.var_ddlte_dn0 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign59070_e91974) as f64).is_finite() && ((assign59070_e91974) as f64).fract() == 0.0 { if assign59070_e91974 == 0.0 { 0.0 } else { (assign59070_e91974 * ((locals.var_t3).powf(assign59070_e91974 - 1.0) * locals.var_t3_dn0)) } } else { (assign59070_e91975 * (((-(locals.var_ddlte_dn0 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign59070_e91974 * (locals.var_t3_dn0 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn2 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign59070_e91974) as f64).is_finite() && ((assign59070_e91974) as f64).fract() == 0.0 { if assign59070_e91974 == 0.0 { 0.0 } else { (assign59070_e91974 * ((locals.var_t3).powf(assign59070_e91974 - 1.0) * locals.var_t3_dn2)) } } else { (assign59070_e91975 * (((-(locals.var_ddlte_dn2 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign59070_e91974 * (locals.var_t3_dn2 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn4 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign59070_e91974) as f64).is_finite() && ((assign59070_e91974) as f64).fract() == 0.0 { if assign59070_e91974 == 0.0 { 0.0 } else { (assign59070_e91974 * ((locals.var_t3).powf(assign59070_e91974 - 1.0) * locals.var_t3_dn4)) } } else { (assign59070_e91975 * (((-(locals.var_ddlte_dn4 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign59070_e91974 * (locals.var_t3_dn4 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn5 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign59070_e91974) as f64).is_finite() && ((assign59070_e91974) as f64).fract() == 0.0 { if assign59070_e91974 == 0.0 { 0.0 } else { (assign59070_e91974 * ((locals.var_t3).powf(assign59070_e91974 - 1.0) * locals.var_t3_dn5)) } } else { (assign59070_e91975 * (((-(locals.var_ddlte_dn5 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign59070_e91974 * (locals.var_t3_dn5 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn6 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign59070_e91974) as f64).is_finite() && ((assign59070_e91974) as f64).fract() == 0.0 { if assign59070_e91974 == 0.0 { 0.0 } else { (assign59070_e91974 * ((locals.var_t3).powf(assign59070_e91974 - 1.0) * locals.var_t3_dn6)) } } else { (assign59070_e91975 * (((-(locals.var_ddlte_dn6 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign59070_e91974 * (locals.var_t3_dn6 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn7 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign59070_e91974) as f64).is_finite() && ((assign59070_e91974) as f64).fract() == 0.0 { if assign59070_e91974 == 0.0 { 0.0 } else { (assign59070_e91974 * ((locals.var_t3).powf(assign59070_e91974 - 1.0) * locals.var_t3_dn7)) } } else { (assign59070_e91975 * (((-(locals.var_ddlte_dn7 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign59070_e91974 * (locals.var_t3_dn7 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn8 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign59070_e91974) as f64).is_finite() && ((assign59070_e91974) as f64).fract() == 0.0 { if assign59070_e91974 == 0.0 { 0.0 } else { (assign59070_e91974 * ((locals.var_t3).powf(assign59070_e91974 - 1.0) * locals.var_t3_dn8)) } } else { (assign59070_e91975 * (((-(locals.var_ddlte_dn8 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign59070_e91974 * (locals.var_t3_dn8 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn9 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign59070_e91974) as f64).is_finite() && ((assign59070_e91974) as f64).fract() == 0.0 { if assign59070_e91974 == 0.0 { 0.0 } else { (assign59070_e91974 * ((locals.var_t3).powf(assign59070_e91974 - 1.0) * locals.var_t3_dn9)) } } else { (assign59070_e91975 * (((-(locals.var_ddlte_dn9 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign59070_e91974 * (locals.var_t3_dn9 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn10 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign59070_e91974) as f64).is_finite() && ((assign59070_e91974) as f64).fract() == 0.0 { if assign59070_e91974 == 0.0 { 0.0 } else { (assign59070_e91974 * ((locals.var_t3).powf(assign59070_e91974 - 1.0) * locals.var_t3_dn10)) } } else { (assign59070_e91975 * (((-(locals.var_ddlte_dn10 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign59070_e91974 * (locals.var_t3_dn10 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn11 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign59070_e91974) as f64).is_finite() && ((assign59070_e91974) as f64).fract() == 0.0 { if assign59070_e91974 == 0.0 { 0.0 } else { (assign59070_e91974 * ((locals.var_t3).powf(assign59070_e91974 - 1.0) * locals.var_t3_dn11)) } } else { (assign59070_e91975 * (((-(locals.var_ddlte_dn11 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign59070_e91974 * (locals.var_t3_dn11 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn14 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign59070_e91974) as f64).is_finite() && ((assign59070_e91974) as f64).fract() == 0.0 { if assign59070_e91974 == 0.0 { 0.0 } else { (assign59070_e91974 * ((locals.var_t3).powf(assign59070_e91974 - 1.0) * locals.var_t3_dn14)) } } else { (assign59070_e91975 * (((-(locals.var_ddlte_dn14 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign59070_e91974 * (locals.var_t3_dn14 / locals.var_t3)))) },)
            }
        };
        (assign59070_e91976, assign59070_e91976_d_n0, assign59070_e91976_d_n2, assign59070_e91976_d_n4, assign59070_e91976_d_n5, assign59070_e91976_d_n6, assign59070_e91976_d_n7, assign59070_e91976_d_n8, assign59070_e91976_d_n9, assign59070_e91976_d_n10, assign59070_e91976_d_n11, assign59070_e91976_d_n14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign59070_e91978;
        locals.var_t4_dn0 = assign59070_e91978_d_n0;
        locals.var_t4_dn2 = assign59070_e91978_d_n2;
        locals.var_t4_dn4 = assign59070_e91978_d_n4;
        locals.var_t4_dn5 = assign59070_e91978_d_n5;
        locals.var_t4_dn6 = assign59070_e91978_d_n6;
        locals.var_t4_dn7 = assign59070_e91978_d_n7;
        locals.var_t4_dn8 = assign59070_e91978_d_n8;
        locals.var_t4_dn9 = assign59070_e91978_d_n9;
        locals.var_t4_dn10 = assign59070_e91978_d_n10;
        locals.var_t4_dn11 = assign59070_e91978_d_n11;
        locals.var_t4_dn14 = assign59070_e91978_d_n14;
        locals.var_t4_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_218(
        locals: &mut StampLocals,
    ) {
        let (assign59080_e91987, assign59080_e91987_d_n0, assign59080_e91987_d_n2, assign59080_e91987_d_n4, assign59080_e91987_d_n5, assign59080_e91987_d_n6, assign59080_e91987_d_n7, assign59080_e91987_d_n8, assign59080_e91987_d_n9, assign59080_e91987_d_n10, assign59080_e91987_d_n11, assign59080_e91987_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign59080_e91985: f64 = (locals.var_t4 * locals.var_t3);
        (assign59080_e91985, ((locals.var_t4_dn0 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn0)), ((locals.var_t4_dn2 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn2)), ((locals.var_t4_dn4 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn4)), ((locals.var_t4_dn5 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn5)), ((locals.var_t4_dn6 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn6)), ((locals.var_t4_dn7 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn7)), ((locals.var_t4_dn8 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn8)), ((locals.var_t4_dn9 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn9)), ((locals.var_t4_dn10 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn10)), ((locals.var_t4_dn11 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn11)), ((locals.var_t4_dn14 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign59080_e91987;
        locals.var_t6_dn0 = assign59080_e91987_d_n0;
        locals.var_t6_dn2 = assign59080_e91987_d_n2;
        locals.var_t6_dn4 = assign59080_e91987_d_n4;
        locals.var_t6_dn5 = assign59080_e91987_d_n5;
        locals.var_t6_dn6 = assign59080_e91987_d_n6;
        locals.var_t6_dn7 = assign59080_e91987_d_n7;
        locals.var_t6_dn8 = assign59080_e91987_d_n8;
        locals.var_t6_dn9 = assign59080_e91987_d_n9;
        locals.var_t6_dn10 = assign59080_e91987_d_n10;
        locals.var_t6_dn11 = assign59080_e91987_d_n11;
        locals.var_t6_dn14 = assign59080_e91987_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign59090_e91996, assign59090_e91996_d_n0, assign59090_e91996_d_n2, assign59090_e91996_d_n4, assign59090_e91996_d_n5, assign59090_e91996_d_n6, assign59090_e91996_d_n7, assign59090_e91996_d_n8, assign59090_e91996_d_n9, assign59090_e91996_d_n10, assign59090_e91996_d_n11, assign59090_e91996_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign59090_e91994: f64 = (locals.var_vds / locals.var_t6);
        (assign59090_e91994, (((locals.var_vds_dn0 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn0)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn2 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn2)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn4 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn4)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn5 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn5)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn6 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn6)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn7 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn7)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn8 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn8)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn9 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn9)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn10 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn10)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn11 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn11)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_dn14 * locals.var_t6) - (locals.var_vds * locals.var_t6_dn14)) / (locals.var_t6 * locals.var_t6)),)
    } else {
        (locals.var_vdseff, locals.var_vdseff_dn0, locals.var_vdseff_dn2, locals.var_vdseff_dn4, locals.var_vdseff_dn5, locals.var_vdseff_dn6, locals.var_vdseff_dn7, locals.var_vdseff_dn8, locals.var_vdseff_dn9, locals.var_vdseff_dn10, locals.var_vdseff_dn11, locals.var_vdseff_dn14,)
    }
};
        locals.var_vdseff = assign59090_e91996;
        locals.var_vdseff_dn0 = assign59090_e91996_d_n0;
        locals.var_vdseff_dn2 = assign59090_e91996_d_n2;
        locals.var_vdseff_dn4 = assign59090_e91996_d_n4;
        locals.var_vdseff_dn5 = assign59090_e91996_d_n5;
        locals.var_vdseff_dn6 = assign59090_e91996_d_n6;
        locals.var_vdseff_dn7 = assign59090_e91996_d_n7;
        locals.var_vdseff_dn8 = assign59090_e91996_d_n8;
        locals.var_vdseff_dn9 = assign59090_e91996_d_n9;
        locals.var_vdseff_dn10 = assign59090_e91996_d_n10;
        locals.var_vdseff_dn11 = assign59090_e91996_d_n11;
        locals.var_vdseff_dn14 = assign59090_e91996_d_n14;
        locals.var_vdseff_rv = 0.0;

        let (assign59100_e92003, assign59100_e92003_d_n0, assign59100_e92003_d_n2, assign59100_e92003_d_n4, assign59100_e92003_d_n5, assign59100_e92003_d_n6, assign59100_e92003_d_n7, assign59100_e92003_d_n8, assign59100_e92003_d_n9, assign59100_e92003_d_n10, assign59100_e92003_d_n11, assign59100_e92003_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        (locals.var_vdseff, locals.var_vdseff_dn0, locals.var_vdseff_dn2, locals.var_vdseff_dn4, locals.var_vdseff_dn5, locals.var_vdseff_dn6, locals.var_vdseff_dn7, locals.var_vdseff_dn8, locals.var_vdseff_dn9, locals.var_vdseff_dn10, locals.var_vdseff_dn11, locals.var_vdseff_dn14,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn14,)
    }
};
        locals.var_vds = assign59100_e92003;
        locals.var_vds_dn0 = assign59100_e92003_d_n0;
        locals.var_vds_dn2 = assign59100_e92003_d_n2;
        locals.var_vds_dn4 = assign59100_e92003_d_n4;
        locals.var_vds_dn5 = assign59100_e92003_d_n5;
        locals.var_vds_dn6 = assign59100_e92003_d_n6;
        locals.var_vds_dn7 = assign59100_e92003_d_n7;
        locals.var_vds_dn8 = assign59100_e92003_d_n8;
        locals.var_vds_dn9 = assign59100_e92003_d_n9;
        locals.var_vds_dn10 = assign59100_e92003_d_n10;
        locals.var_vds_dn11 = assign59100_e92003_d_n11;
        locals.var_vds_dn14 = assign59100_e92003_d_n14;
        locals.var_vds_rv = 0.0;

        let (assign59120_e92022, assign59120_e92022_d_n0, assign59120_e92022_d_n2, assign59120_e92022_d_n4, assign59120_e92022_d_n5, assign59120_e92022_d_n6, assign59120_e92022_d_n7, assign59120_e92022_d_n8, assign59120_e92022_d_n9, assign59120_e92022_d_n10, assign59120_e92022_d_n11, assign59120_e92022_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign59120_e92018: f64 = (locals.var_vbscl__blk439 - locals.var_vds);
        let assign59120_e92019: f64 = (locals.var_beta * assign59120_e92018);
        let assign59120_e92020: f64 = (assign59120_e92019).exp();
        (assign59120_e92020, (assign59120_e92020 * ((locals.var_beta_dn0 * assign59120_e92018) + (locals.var_beta * (locals.var_vbscl__blk439_dn0 - locals.var_vds_dn0)))), (assign59120_e92020 * ((locals.var_beta_dn2 * assign59120_e92018) + (locals.var_beta * (locals.var_vbscl__blk439_dn2 - locals.var_vds_dn2)))), (assign59120_e92020 * ((locals.var_beta_dn4 * assign59120_e92018) + (locals.var_beta * (locals.var_vbscl__blk439_dn4 - locals.var_vds_dn4)))), (assign59120_e92020 * ((locals.var_beta_dn5 * assign59120_e92018) + (locals.var_beta * (locals.var_vbscl__blk439_dn5 - locals.var_vds_dn5)))), (assign59120_e92020 * ((locals.var_beta_dn6 * assign59120_e92018) + (locals.var_beta * (locals.var_vbscl__blk439_dn6 - locals.var_vds_dn6)))), (assign59120_e92020 * ((locals.var_beta_dn7 * assign59120_e92018) + (locals.var_beta * (locals.var_vbscl__blk439_dn7 - locals.var_vds_dn7)))), (assign59120_e92020 * ((locals.var_beta_dn8 * assign59120_e92018) + (locals.var_beta * (locals.var_vbscl__blk439_dn8 - locals.var_vds_dn8)))), (assign59120_e92020 * ((locals.var_beta_dn9 * assign59120_e92018) + (locals.var_beta * (locals.var_vbscl__blk439_dn9 - locals.var_vds_dn9)))), (assign59120_e92020 * ((locals.var_beta_dn10 * assign59120_e92018) + (locals.var_beta * (locals.var_vbscl__blk439_dn10 - locals.var_vds_dn10)))), (assign59120_e92020 * ((locals.var_beta_dn11 * assign59120_e92018) + (locals.var_beta * (locals.var_vbscl__blk439_dn11 - locals.var_vds_dn11)))), (assign59120_e92020 * ((locals.var_beta_dn14 * assign59120_e92018) + (locals.var_beta * (locals.var_vbscl__blk439_dn14 - locals.var_vds_dn14)))),)
    } else {
        (locals.var_exp_bvbsvds, locals.var_exp_bvbsvds_dn0, locals.var_exp_bvbsvds_dn2, locals.var_exp_bvbsvds_dn4, locals.var_exp_bvbsvds_dn5, locals.var_exp_bvbsvds_dn6, locals.var_exp_bvbsvds_dn7, locals.var_exp_bvbsvds_dn8, locals.var_exp_bvbsvds_dn9, locals.var_exp_bvbsvds_dn10, locals.var_exp_bvbsvds_dn11, locals.var_exp_bvbsvds_dn14,)
    }
};
        locals.var_exp_bvbsvds = assign59120_e92022;
        locals.var_exp_bvbsvds_dn0 = assign59120_e92022_d_n0;
        locals.var_exp_bvbsvds_dn2 = assign59120_e92022_d_n2;
        locals.var_exp_bvbsvds_dn4 = assign59120_e92022_d_n4;
        locals.var_exp_bvbsvds_dn5 = assign59120_e92022_d_n5;
        locals.var_exp_bvbsvds_dn6 = assign59120_e92022_d_n6;
        locals.var_exp_bvbsvds_dn7 = assign59120_e92022_d_n7;
        locals.var_exp_bvbsvds_dn8 = assign59120_e92022_d_n8;
        locals.var_exp_bvbsvds_dn9 = assign59120_e92022_d_n9;
        locals.var_exp_bvbsvds_dn10 = assign59120_e92022_d_n10;
        locals.var_exp_bvbsvds_dn11 = assign59120_e92022_d_n11;
        locals.var_exp_bvbsvds_dn14 = assign59120_e92022_d_n14;
        locals.var_exp_bvbsvds_rv = 0.0;

        let assign59130_e92025: f64 = if locals.var_vds < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1451 = assign59130_e92025;
        locals.var_guard1451_rv = 0.0;

        let (assign59140_e92034, assign59140_e92034_d_n0, assign59140_e92034_d_n2, assign59140_e92034_d_n4, assign59140_e92034_d_n5, assign59140_e92034_d_n6, assign59140_e92034_d_n7, assign59140_e92034_d_n8, assign59140_e92034_d_n9, assign59140_e92034_d_n10, assign59140_e92034_d_n11, assign59140_e92034_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1451 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pds, locals.var_pds_dn0, locals.var_pds_dn2, locals.var_pds_dn4, locals.var_pds_dn5, locals.var_pds_dn6, locals.var_pds_dn7, locals.var_pds_dn8, locals.var_pds_dn9, locals.var_pds_dn10, locals.var_pds_dn11, locals.var_pds_dn14,)
    }
};
        locals.var_pds = assign59140_e92034;
        locals.var_pds_dn0 = assign59140_e92034_d_n0;
        locals.var_pds_dn2 = assign59140_e92034_d_n2;
        locals.var_pds_dn4 = assign59140_e92034_d_n4;
        locals.var_pds_dn5 = assign59140_e92034_d_n5;
        locals.var_pds_dn6 = assign59140_e92034_d_n6;
        locals.var_pds_dn7 = assign59140_e92034_d_n7;
        locals.var_pds_dn8 = assign59140_e92034_d_n8;
        locals.var_pds_dn9 = assign59140_e92034_d_n9;
        locals.var_pds_dn10 = assign59140_e92034_d_n10;
        locals.var_pds_dn11 = assign59140_e92034_d_n11;
        locals.var_pds_dn14 = assign59140_e92034_d_n14;
        locals.var_pds_rv = 0.0;

        let (assign59150_e92043, assign59150_e92043_d_n0, assign59150_e92043_d_n2, assign59150_e92043_d_n4, assign59150_e92043_d_n5, assign59150_e92043_d_n6, assign59150_e92043_d_n7, assign59150_e92043_d_n8, assign59150_e92043_d_n9, assign59150_e92043_d_n10, assign59150_e92043_d_n11, assign59150_e92043_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1451 != 0.0)) {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn4, locals.var_ps0_dn5, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn8, locals.var_ps0_dn9, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn14,)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn4, locals.var_psl_dn5, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn8, locals.var_psl_dn9, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn14,)
    }
};
        locals.var_psl = assign59150_e92043;
        locals.var_psl_dn0 = assign59150_e92043_d_n0;
        locals.var_psl_dn2 = assign59150_e92043_d_n2;
        locals.var_psl_dn4 = assign59150_e92043_d_n4;
        locals.var_psl_dn5 = assign59150_e92043_d_n5;
        locals.var_psl_dn6 = assign59150_e92043_d_n6;
        locals.var_psl_dn7 = assign59150_e92043_d_n7;
        locals.var_psl_dn8 = assign59150_e92043_d_n8;
        locals.var_psl_dn9 = assign59150_e92043_d_n9;
        locals.var_psl_dn10 = assign59150_e92043_d_n10;
        locals.var_psl_dn11 = assign59150_e92043_d_n11;
        locals.var_psl_dn14 = assign59150_e92043_d_n14;
        locals.var_psl_rv = 0.0;

        let (assign59160_e92052,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1451 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_start_of_loopl,)
    }
};
        locals.var_start_of_loopl = assign59160_e92052;
        locals.var_start_of_loopl_rv = 0.0;

        let assign59170_e92055: f64 = if locals.var_start_of_loopl == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1452 = assign59170_e92055;
        locals.var_guard1452_rv = 0.0;

        let assign59180_e92058: f64 = if locals.var_flg_pprv == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1453 = assign59180_e92058;
        locals.var_guard1453_rv = 0.0;

        let (assign59190_e92078, assign59190_e92078_d_n0, assign59190_e92078_d_n2, assign59190_e92078_d_n4, assign59190_e92078_d_n5, assign59190_e92078_d_n6, assign59190_e92078_d_n7, assign59190_e92078_d_n8, assign59190_e92078_d_n9, assign59190_e92078_d_n10, assign59190_e92078_d_n11, assign59190_e92078_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1452 != 0.0)) && (locals.var_guard1453 != 0.0)) {
        let assign59190_e92069: f64 = (locals.var_psl_lim - locals.var_ps0);
        let (assign59190_e92076, assign59190_e92076_d_n0, assign59190_e92076_d_n2, assign59190_e92076_d_n4, assign59190_e92076_d_n5, assign59190_e92076_d_n6, assign59190_e92076_d_n7, assign59190_e92076_d_n8, assign59190_e92076_d_n9, assign59190_e92076_d_n10, assign59190_e92076_d_n11, assign59190_e92076_d_n14,) = {
            if (assign59190_e92069 >= 0.0) {
                let assign59190_e92074: f64 = (locals.var_psl_lim - locals.var_ps0);
                (assign59190_e92074, (locals.var_psl_lim_dn0 - locals.var_ps0_dn0), (locals.var_psl_lim_dn2 - locals.var_ps0_dn2), (locals.var_psl_lim_dn4 - locals.var_ps0_dn4), (locals.var_psl_lim_dn5 - locals.var_ps0_dn5), (locals.var_psl_lim_dn6 - locals.var_ps0_dn6), (locals.var_psl_lim_dn7 - locals.var_ps0_dn7), (locals.var_psl_lim_dn8 - locals.var_ps0_dn8), (locals.var_psl_lim_dn9 - locals.var_ps0_dn9), (locals.var_psl_lim_dn10 - locals.var_ps0_dn10), (locals.var_psl_lim_dn11 - locals.var_ps0_dn11), (locals.var_psl_lim_dn14 - locals.var_ps0_dn14),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign59190_e92076, assign59190_e92076_d_n0, assign59190_e92076_d_n2, assign59190_e92076_d_n4, assign59190_e92076_d_n5, assign59190_e92076_d_n6, assign59190_e92076_d_n7, assign59190_e92076_d_n8, assign59190_e92076_d_n9, assign59190_e92076_d_n10, assign59190_e92076_d_n11, assign59190_e92076_d_n14,)
    } else {
        (locals.var_pds_max, locals.var_pds_max_dn0, locals.var_pds_max_dn2, locals.var_pds_max_dn4, locals.var_pds_max_dn5, locals.var_pds_max_dn6, locals.var_pds_max_dn7, locals.var_pds_max_dn8, locals.var_pds_max_dn9, locals.var_pds_max_dn10, locals.var_pds_max_dn11, locals.var_pds_max_dn14,)
    }
};
        locals.var_pds_max = assign59190_e92078;
        locals.var_pds_max_dn0 = assign59190_e92078_d_n0;
        locals.var_pds_max_dn2 = assign59190_e92078_d_n2;
        locals.var_pds_max_dn4 = assign59190_e92078_d_n4;
        locals.var_pds_max_dn5 = assign59190_e92078_d_n5;
        locals.var_pds_max_dn6 = assign59190_e92078_d_n6;
        locals.var_pds_max_dn7 = assign59190_e92078_d_n7;
        locals.var_pds_max_dn8 = assign59190_e92078_d_n8;
        locals.var_pds_max_dn9 = assign59190_e92078_d_n9;
        locals.var_pds_max_dn10 = assign59190_e92078_d_n10;
        locals.var_pds_max_dn11 = assign59190_e92078_d_n11;
        locals.var_pds_max_dn14 = assign59190_e92078_d_n14;
        locals.var_pds_max_rv = 0.0;

        let assign59200_e92081: f64 = (1.0 + 0.3);
        let assign59200_e92083: f64 = (assign59200_e92081 * locals.var_pds_max);
        let assign59200_e92085: f64 = if assign59200_e92083 > 0.03 { 1.0 } else { 0.0 };
        locals.var_guard1454 = assign59200_e92085;
        locals.var_guard1454_rv = 0.0;

        let (assign59210_e92106, assign59210_e92106_d_n0, assign59210_e92106_d_n2, assign59210_e92106_d_n4, assign59210_e92106_d_n5, assign59210_e92106_d_n6, assign59210_e92106_d_n7, assign59210_e92106_d_n8, assign59210_e92106_d_n9, assign59210_e92106_d_n10, assign59210_e92106_d_n11, assign59210_e92106_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1452 != 0.0)) && (locals.var_guard1453 != 0.0)) && (locals.var_guard1454 != 0.0)) {
        let assign59210_e92098: f64 = (1.0 + 0.3);
        let assign59210_e92100: f64 = (assign59210_e92098 * locals.var_pds_max);
        let assign59210_e92102: f64 = (assign59210_e92100 - locals.var_vds);
        let assign59210_e92104: f64 = (assign59210_e92102 - 0.03);
        (assign59210_e92104, ((assign59210_e92098 * locals.var_pds_max_dn0) - locals.var_vds_dn0), ((assign59210_e92098 * locals.var_pds_max_dn2) - locals.var_vds_dn2), ((assign59210_e92098 * locals.var_pds_max_dn4) - locals.var_vds_dn4), ((assign59210_e92098 * locals.var_pds_max_dn5) - locals.var_vds_dn5), ((assign59210_e92098 * locals.var_pds_max_dn6) - locals.var_vds_dn6), ((assign59210_e92098 * locals.var_pds_max_dn7) - locals.var_vds_dn7), ((assign59210_e92098 * locals.var_pds_max_dn8) - locals.var_vds_dn8), ((assign59210_e92098 * locals.var_pds_max_dn9) - locals.var_vds_dn9), ((assign59210_e92098 * locals.var_pds_max_dn10) - locals.var_vds_dn10), ((assign59210_e92098 * locals.var_pds_max_dn11) - locals.var_vds_dn11), ((assign59210_e92098 * locals.var_pds_max_dn14) - locals.var_vds_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign59210_e92106;
        locals.var_tmf1_dn0 = assign59210_e92106_d_n0;
        locals.var_tmf1_dn2 = assign59210_e92106_d_n2;
        locals.var_tmf1_dn4 = assign59210_e92106_d_n4;
        locals.var_tmf1_dn5 = assign59210_e92106_d_n5;
        locals.var_tmf1_dn6 = assign59210_e92106_d_n6;
        locals.var_tmf1_dn7 = assign59210_e92106_d_n7;
        locals.var_tmf1_dn8 = assign59210_e92106_d_n8;
        locals.var_tmf1_dn9 = assign59210_e92106_d_n9;
        locals.var_tmf1_dn10 = assign59210_e92106_d_n10;
        locals.var_tmf1_dn11 = assign59210_e92106_d_n11;
        locals.var_tmf1_dn14 = assign59210_e92106_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign59220_e92127, assign59220_e92127_d_n0, assign59220_e92127_d_n2, assign59220_e92127_d_n4, assign59220_e92127_d_n5, assign59220_e92127_d_n6, assign59220_e92127_d_n7, assign59220_e92127_d_n8, assign59220_e92127_d_n9, assign59220_e92127_d_n10, assign59220_e92127_d_n11, assign59220_e92127_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1452 != 0.0)) && (locals.var_guard1453 != 0.0)) && (locals.var_guard1454 != 0.0)) {
        let assign59220_e92120: f64 = (1.0 + 0.3);
        let assign59220_e92122: f64 = (assign59220_e92120 * locals.var_pds_max);
        let assign59220_e92123: f64 = (4.0 * assign59220_e92122);
        let assign59220_e92125: f64 = (assign59220_e92123 * 0.03);
        (assign59220_e92125, ((4.0 * (assign59220_e92120 * locals.var_pds_max_dn0)) * 0.03), ((4.0 * (assign59220_e92120 * locals.var_pds_max_dn2)) * 0.03), ((4.0 * (assign59220_e92120 * locals.var_pds_max_dn4)) * 0.03), ((4.0 * (assign59220_e92120 * locals.var_pds_max_dn5)) * 0.03), ((4.0 * (assign59220_e92120 * locals.var_pds_max_dn6)) * 0.03), ((4.0 * (assign59220_e92120 * locals.var_pds_max_dn7)) * 0.03), ((4.0 * (assign59220_e92120 * locals.var_pds_max_dn8)) * 0.03), ((4.0 * (assign59220_e92120 * locals.var_pds_max_dn9)) * 0.03), ((4.0 * (assign59220_e92120 * locals.var_pds_max_dn10)) * 0.03), ((4.0 * (assign59220_e92120 * locals.var_pds_max_dn11)) * 0.03), ((4.0 * (assign59220_e92120 * locals.var_pds_max_dn14)) * 0.03),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign59220_e92127;
        locals.var_tmf2_dn0 = assign59220_e92127_d_n0;
        locals.var_tmf2_dn2 = assign59220_e92127_d_n2;
        locals.var_tmf2_dn4 = assign59220_e92127_d_n4;
        locals.var_tmf2_dn5 = assign59220_e92127_d_n5;
        locals.var_tmf2_dn6 = assign59220_e92127_d_n6;
        locals.var_tmf2_dn7 = assign59220_e92127_d_n7;
        locals.var_tmf2_dn8 = assign59220_e92127_d_n8;
        locals.var_tmf2_dn9 = assign59220_e92127_d_n9;
        locals.var_tmf2_dn10 = assign59220_e92127_d_n10;
        locals.var_tmf2_dn11 = assign59220_e92127_d_n11;
        locals.var_tmf2_dn14 = assign59220_e92127_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign59230_e92146, assign59230_e92146_d_n0, assign59230_e92146_d_n2, assign59230_e92146_d_n4, assign59230_e92146_d_n5, assign59230_e92146_d_n6, assign59230_e92146_d_n7, assign59230_e92146_d_n8, assign59230_e92146_d_n9, assign59230_e92146_d_n10, assign59230_e92146_d_n11, assign59230_e92146_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1452 != 0.0)) && (locals.var_guard1453 != 0.0)) && (locals.var_guard1454 != 0.0)) {
        let (assign59230_e92144, assign59230_e92144_d_n0, assign59230_e92144_d_n2, assign59230_e92144_d_n4, assign59230_e92144_d_n5, assign59230_e92144_d_n6, assign59230_e92144_d_n7, assign59230_e92144_d_n8, assign59230_e92144_d_n9, assign59230_e92144_d_n10, assign59230_e92144_d_n11, assign59230_e92144_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign59230_e92143: f64 = (-locals.var_tmf2);
                (assign59230_e92143, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign59230_e92144, assign59230_e92144_d_n0, assign59230_e92144_d_n2, assign59230_e92144_d_n4, assign59230_e92144_d_n5, assign59230_e92144_d_n6, assign59230_e92144_d_n7, assign59230_e92144_d_n8, assign59230_e92144_d_n9, assign59230_e92144_d_n10, assign59230_e92144_d_n11, assign59230_e92144_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign59230_e92146;
        locals.var_tmf2_dn0 = assign59230_e92146_d_n0;
        locals.var_tmf2_dn2 = assign59230_e92146_d_n2;
        locals.var_tmf2_dn4 = assign59230_e92146_d_n4;
        locals.var_tmf2_dn5 = assign59230_e92146_d_n5;
        locals.var_tmf2_dn6 = assign59230_e92146_d_n6;
        locals.var_tmf2_dn7 = assign59230_e92146_d_n7;
        locals.var_tmf2_dn8 = assign59230_e92146_d_n8;
        locals.var_tmf2_dn9 = assign59230_e92146_d_n9;
        locals.var_tmf2_dn10 = assign59230_e92146_d_n10;
        locals.var_tmf2_dn11 = assign59230_e92146_d_n11;
        locals.var_tmf2_dn14 = assign59230_e92146_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign59240_e92164, assign59240_e92164_d_n0, assign59240_e92164_d_n2, assign59240_e92164_d_n4, assign59240_e92164_d_n5, assign59240_e92164_d_n6, assign59240_e92164_d_n7, assign59240_e92164_d_n8, assign59240_e92164_d_n9, assign59240_e92164_d_n10, assign59240_e92164_d_n11, assign59240_e92164_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1452 != 0.0)) && (locals.var_guard1453 != 0.0)) && (locals.var_guard1454 != 0.0)) {
        let assign59240_e92159: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign59240_e92161: f64 = (assign59240_e92159 + locals.var_tmf2);
        let assign59240_e92162: f64 = (assign59240_e92161).sqrt();
        (assign59240_e92162, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign59240_e92162)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign59240_e92162)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign59240_e92162)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign59240_e92162)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign59240_e92162)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign59240_e92162)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign59240_e92162)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign59240_e92162)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign59240_e92162)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign59240_e92162)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign59240_e92162)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign59240_e92164;
        locals.var_tmf2_dn0 = assign59240_e92164_d_n0;
        locals.var_tmf2_dn2 = assign59240_e92164_d_n2;
        locals.var_tmf2_dn4 = assign59240_e92164_d_n4;
        locals.var_tmf2_dn5 = assign59240_e92164_d_n5;
        locals.var_tmf2_dn6 = assign59240_e92164_d_n6;
        locals.var_tmf2_dn7 = assign59240_e92164_d_n7;
        locals.var_tmf2_dn8 = assign59240_e92164_d_n8;
        locals.var_tmf2_dn9 = assign59240_e92164_d_n9;
        locals.var_tmf2_dn10 = assign59240_e92164_d_n10;
        locals.var_tmf2_dn11 = assign59240_e92164_d_n11;
        locals.var_tmf2_dn14 = assign59240_e92164_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign59250_e92183, assign59250_e92183_d_n0, assign59250_e92183_d_n2, assign59250_e92183_d_n4, assign59250_e92183_d_n5, assign59250_e92183_d_n6, assign59250_e92183_d_n7, assign59250_e92183_d_n8, assign59250_e92183_d_n9, assign59250_e92183_d_n10, assign59250_e92183_d_n11, assign59250_e92183_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1452 != 0.0)) && (locals.var_guard1453 != 0.0)) && (locals.var_guard1454 != 0.0)) {
        let assign59250_e92179: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign59250_e92180: f64 = (1.0 + assign59250_e92179);
        let assign59250_e92181: f64 = (0.5 * assign59250_e92180);
        (assign59250_e92181, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign59250_e92183;
        locals.var_t1_dn0 = assign59250_e92183_d_n0;
        locals.var_t1_dn2 = assign59250_e92183_d_n2;
        locals.var_t1_dn4 = assign59250_e92183_d_n4;
        locals.var_t1_dn5 = assign59250_e92183_d_n5;
        locals.var_t1_dn6 = assign59250_e92183_d_n6;
        locals.var_t1_dn7 = assign59250_e92183_d_n7;
        locals.var_t1_dn8 = assign59250_e92183_d_n8;
        locals.var_t1_dn9 = assign59250_e92183_d_n9;
        locals.var_t1_dn10 = assign59250_e92183_d_n10;
        locals.var_t1_dn11 = assign59250_e92183_d_n11;
        locals.var_t1_dn14 = assign59250_e92183_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign59260_e92206, assign59260_e92206_d_n0, assign59260_e92206_d_n2, assign59260_e92206_d_n4, assign59260_e92206_d_n5, assign59260_e92206_d_n6, assign59260_e92206_d_n7, assign59260_e92206_d_n8, assign59260_e92206_d_n9, assign59260_e92206_d_n10, assign59260_e92206_d_n11, assign59260_e92206_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1452 != 0.0)) && (locals.var_guard1453 != 0.0)) && (locals.var_guard1454 != 0.0)) {
        let assign59260_e92196: f64 = (1.0 + 0.3);
        let assign59260_e92198: f64 = (assign59260_e92196 * locals.var_pds_max);
        let assign59260_e92202: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign59260_e92203: f64 = (0.5 * assign59260_e92202);
        let assign59260_e92204: f64 = (assign59260_e92198 - assign59260_e92203);
        (assign59260_e92204, ((assign59260_e92196 * locals.var_pds_max_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((assign59260_e92196 * locals.var_pds_max_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((assign59260_e92196 * locals.var_pds_max_dn4) - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), ((assign59260_e92196 * locals.var_pds_max_dn5) - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), ((assign59260_e92196 * locals.var_pds_max_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((assign59260_e92196 * locals.var_pds_max_dn7) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((assign59260_e92196 * locals.var_pds_max_dn8) - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), ((assign59260_e92196 * locals.var_pds_max_dn9) - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), ((assign59260_e92196 * locals.var_pds_max_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((assign59260_e92196 * locals.var_pds_max_dn11) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((assign59260_e92196 * locals.var_pds_max_dn14) - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn4, locals.var_pds_ini_dn5, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn8, locals.var_pds_ini_dn9, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn14,)
    }
};
        locals.var_pds_ini = assign59260_e92206;
        locals.var_pds_ini_dn0 = assign59260_e92206_d_n0;
        locals.var_pds_ini_dn2 = assign59260_e92206_d_n2;
        locals.var_pds_ini_dn4 = assign59260_e92206_d_n4;
        locals.var_pds_ini_dn5 = assign59260_e92206_d_n5;
        locals.var_pds_ini_dn6 = assign59260_e92206_d_n6;
        locals.var_pds_ini_dn7 = assign59260_e92206_d_n7;
        locals.var_pds_ini_dn8 = assign59260_e92206_d_n8;
        locals.var_pds_ini_dn9 = assign59260_e92206_d_n9;
        locals.var_pds_ini_dn10 = assign59260_e92206_d_n10;
        locals.var_pds_ini_dn11 = assign59260_e92206_d_n11;
        locals.var_pds_ini_dn14 = assign59260_e92206_d_n14;
        locals.var_pds_ini_rv = 0.0;

        let (assign59270_e92224, assign59270_e92224_d_n0, assign59270_e92224_d_n2, assign59270_e92224_d_n4, assign59270_e92224_d_n5, assign59270_e92224_d_n6, assign59270_e92224_d_n7, assign59270_e92224_d_n8, assign59270_e92224_d_n9, assign59270_e92224_d_n10, assign59270_e92224_d_n11, assign59270_e92224_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1452 != 0.0)) && (locals.var_guard1453 != 0.0)) && (locals.var_guard1454 == 0.0)) {
        let assign59270_e92220: f64 = (1.0 + 0.3);
        let assign59270_e92222: f64 = (assign59270_e92220 * locals.var_pds_max);
        (assign59270_e92222, (assign59270_e92220 * locals.var_pds_max_dn0), (assign59270_e92220 * locals.var_pds_max_dn2), (assign59270_e92220 * locals.var_pds_max_dn4), (assign59270_e92220 * locals.var_pds_max_dn5), (assign59270_e92220 * locals.var_pds_max_dn6), (assign59270_e92220 * locals.var_pds_max_dn7), (assign59270_e92220 * locals.var_pds_max_dn8), (assign59270_e92220 * locals.var_pds_max_dn9), (assign59270_e92220 * locals.var_pds_max_dn10), (assign59270_e92220 * locals.var_pds_max_dn11), (assign59270_e92220 * locals.var_pds_max_dn14),)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn4, locals.var_pds_ini_dn5, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn8, locals.var_pds_ini_dn9, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn14,)
    }
};
        locals.var_pds_ini = assign59270_e92224;
        locals.var_pds_ini_dn0 = assign59270_e92224_d_n0;
        locals.var_pds_ini_dn2 = assign59270_e92224_d_n2;
        locals.var_pds_ini_dn4 = assign59270_e92224_d_n4;
        locals.var_pds_ini_dn5 = assign59270_e92224_d_n5;
        locals.var_pds_ini_dn6 = assign59270_e92224_d_n6;
        locals.var_pds_ini_dn7 = assign59270_e92224_d_n7;
        locals.var_pds_ini_dn8 = assign59270_e92224_d_n8;
        locals.var_pds_ini_dn9 = assign59270_e92224_d_n9;
        locals.var_pds_ini_dn10 = assign59270_e92224_d_n10;
        locals.var_pds_ini_dn11 = assign59270_e92224_d_n11;
        locals.var_pds_ini_dn14 = assign59270_e92224_d_n14;
        locals.var_pds_ini_rv = 0.0;

        let (assign59280_e92240, assign59280_e92240_d_n0, assign59280_e92240_d_n2, assign59280_e92240_d_n4, assign59280_e92240_d_n5, assign59280_e92240_d_n6, assign59280_e92240_d_n7, assign59280_e92240_d_n8, assign59280_e92240_d_n9, assign59280_e92240_d_n10, assign59280_e92240_d_n11, assign59280_e92240_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1452 != 0.0)) && (locals.var_guard1453 != 0.0)) {
        let (assign59280_e92238, assign59280_e92238_d_n0, assign59280_e92238_d_n2, assign59280_e92238_d_n4, assign59280_e92238_d_n5, assign59280_e92238_d_n6, assign59280_e92238_d_n7, assign59280_e92238_d_n8, assign59280_e92238_d_n9, assign59280_e92238_d_n10, assign59280_e92238_d_n11, assign59280_e92238_d_n14,) = {
            if (locals.var_pds_ini <= locals.var_pds_max) {
                (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn4, locals.var_pds_ini_dn5, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn8, locals.var_pds_ini_dn9, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn14,)
            } else {
                (locals.var_pds_max, locals.var_pds_max_dn0, locals.var_pds_max_dn2, locals.var_pds_max_dn4, locals.var_pds_max_dn5, locals.var_pds_max_dn6, locals.var_pds_max_dn7, locals.var_pds_max_dn8, locals.var_pds_max_dn9, locals.var_pds_max_dn10, locals.var_pds_max_dn11, locals.var_pds_max_dn14,)
            }
        };
        (assign59280_e92238, assign59280_e92238_d_n0, assign59280_e92238_d_n2, assign59280_e92238_d_n4, assign59280_e92238_d_n5, assign59280_e92238_d_n6, assign59280_e92238_d_n7, assign59280_e92238_d_n8, assign59280_e92238_d_n9, assign59280_e92238_d_n10, assign59280_e92238_d_n11, assign59280_e92238_d_n14,)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn4, locals.var_pds_ini_dn5, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn8, locals.var_pds_ini_dn9, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn14,)
    }
};
        locals.var_pds_ini = assign59280_e92240;
        locals.var_pds_ini_dn0 = assign59280_e92240_d_n0;
        locals.var_pds_ini_dn2 = assign59280_e92240_d_n2;
        locals.var_pds_ini_dn4 = assign59280_e92240_d_n4;
        locals.var_pds_ini_dn5 = assign59280_e92240_d_n5;
        locals.var_pds_ini_dn6 = assign59280_e92240_d_n6;
        locals.var_pds_ini_dn7 = assign59280_e92240_d_n7;
        locals.var_pds_ini_dn8 = assign59280_e92240_d_n8;
        locals.var_pds_ini_dn9 = assign59280_e92240_d_n9;
        locals.var_pds_ini_dn10 = assign59280_e92240_d_n10;
        locals.var_pds_ini_dn11 = assign59280_e92240_d_n11;
        locals.var_pds_ini_dn14 = assign59280_e92240_d_n14;
        locals.var_pds_ini_rv = 0.0;

        let assign59290_e92243: f64 = if locals.var_pds_ini < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1455 = assign59290_e92243;
        locals.var_guard1455_rv = 0.0;

        let (assign59300_e92254, assign59300_e92254_d_n0, assign59300_e92254_d_n2, assign59300_e92254_d_n4, assign59300_e92254_d_n5, assign59300_e92254_d_n6, assign59300_e92254_d_n7, assign59300_e92254_d_n8, assign59300_e92254_d_n9, assign59300_e92254_d_n10, assign59300_e92254_d_n11, assign59300_e92254_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1452 != 0.0)) && (locals.var_guard1455 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn4, locals.var_pds_ini_dn5, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn8, locals.var_pds_ini_dn9, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn14,)
    }
};
        locals.var_pds_ini = assign59300_e92254;
        locals.var_pds_ini_dn0 = assign59300_e92254_d_n0;
        locals.var_pds_ini_dn2 = assign59300_e92254_d_n2;
        locals.var_pds_ini_dn4 = assign59300_e92254_d_n4;
        locals.var_pds_ini_dn5 = assign59300_e92254_d_n5;
        locals.var_pds_ini_dn6 = assign59300_e92254_d_n6;
        locals.var_pds_ini_dn7 = assign59300_e92254_d_n7;
        locals.var_pds_ini_dn8 = assign59300_e92254_d_n8;
        locals.var_pds_ini_dn9 = assign59300_e92254_d_n9;
        locals.var_pds_ini_dn10 = assign59300_e92254_d_n10;
        locals.var_pds_ini_dn11 = assign59300_e92254_d_n11;
        locals.var_pds_ini_dn14 = assign59300_e92254_d_n14;
        locals.var_pds_ini_rv = 0.0;

        let assign59310_e92257: f64 = if locals.var_pds_ini > locals.var_vds { 1.0 } else { 0.0 };
        locals.var_guard1456 = assign59310_e92257;
        locals.var_guard1456_rv = 0.0;

        let (assign59320_e92271, assign59320_e92271_d_n0, assign59320_e92271_d_n2, assign59320_e92271_d_n4, assign59320_e92271_d_n5, assign59320_e92271_d_n6, assign59320_e92271_d_n7, assign59320_e92271_d_n8, assign59320_e92271_d_n9, assign59320_e92271_d_n10, assign59320_e92271_d_n11, assign59320_e92271_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1452 != 0.0)) && (locals.var_guard1455 == 0.0)) && (locals.var_guard1456 != 0.0)) {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn14,)
    } else {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn4, locals.var_pds_ini_dn5, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn8, locals.var_pds_ini_dn9, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn14,)
    }
};
        locals.var_pds_ini = assign59320_e92271;
        locals.var_pds_ini_dn0 = assign59320_e92271_d_n0;
        locals.var_pds_ini_dn2 = assign59320_e92271_d_n2;
        locals.var_pds_ini_dn4 = assign59320_e92271_d_n4;
        locals.var_pds_ini_dn5 = assign59320_e92271_d_n5;
        locals.var_pds_ini_dn6 = assign59320_e92271_d_n6;
        locals.var_pds_ini_dn7 = assign59320_e92271_d_n7;
        locals.var_pds_ini_dn8 = assign59320_e92271_d_n8;
        locals.var_pds_ini_dn9 = assign59320_e92271_d_n9;
        locals.var_pds_ini_dn10 = assign59320_e92271_d_n10;
        locals.var_pds_ini_dn11 = assign59320_e92271_d_n11;
        locals.var_pds_ini_dn14 = assign59320_e92271_d_n14;
        locals.var_pds_ini_rv = 0.0;

        let (assign59340_e92289, assign59340_e92289_d_n0, assign59340_e92289_d_n2, assign59340_e92289_d_n4, assign59340_e92289_d_n5, assign59340_e92289_d_n6, assign59340_e92289_d_n7, assign59340_e92289_d_n8, assign59340_e92289_d_n9, assign59340_e92289_d_n10, assign59340_e92289_d_n11, assign59340_e92289_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1452 != 0.0)) {
        (locals.var_pds_ini, locals.var_pds_ini_dn0, locals.var_pds_ini_dn2, locals.var_pds_ini_dn4, locals.var_pds_ini_dn5, locals.var_pds_ini_dn6, locals.var_pds_ini_dn7, locals.var_pds_ini_dn8, locals.var_pds_ini_dn9, locals.var_pds_ini_dn10, locals.var_pds_ini_dn11, locals.var_pds_ini_dn14,)
    } else {
        (locals.var_pds, locals.var_pds_dn0, locals.var_pds_dn2, locals.var_pds_dn4, locals.var_pds_dn5, locals.var_pds_dn6, locals.var_pds_dn7, locals.var_pds_dn8, locals.var_pds_dn9, locals.var_pds_dn10, locals.var_pds_dn11, locals.var_pds_dn14,)
    }
};
        locals.var_pds = assign59340_e92289;
        locals.var_pds_dn0 = assign59340_e92289_d_n0;
        locals.var_pds_dn2 = assign59340_e92289_d_n2;
        locals.var_pds_dn4 = assign59340_e92289_d_n4;
        locals.var_pds_dn5 = assign59340_e92289_d_n5;
        locals.var_pds_dn6 = assign59340_e92289_d_n6;
        locals.var_pds_dn7 = assign59340_e92289_d_n7;
        locals.var_pds_dn8 = assign59340_e92289_d_n8;
        locals.var_pds_dn9 = assign59340_e92289_d_n9;
        locals.var_pds_dn10 = assign59340_e92289_d_n10;
        locals.var_pds_dn11 = assign59340_e92289_d_n11;
        locals.var_pds_dn14 = assign59340_e92289_d_n14;
        locals.var_pds_rv = 0.0;

        let (assign59350_e92300, assign59350_e92300_d_n0, assign59350_e92300_d_n2, assign59350_e92300_d_n4, assign59350_e92300_d_n5, assign59350_e92300_d_n6, assign59350_e92300_d_n7, assign59350_e92300_d_n8, assign59350_e92300_d_n9, assign59350_e92300_d_n10, assign59350_e92300_d_n11, assign59350_e92300_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1452 != 0.0)) {
        let assign59350_e92298: f64 = (locals.var_ps0 + locals.var_pds);
        (assign59350_e92298, (locals.var_ps0_dn0 + locals.var_pds_dn0), (locals.var_ps0_dn2 + locals.var_pds_dn2), (locals.var_ps0_dn4 + locals.var_pds_dn4), (locals.var_ps0_dn5 + locals.var_pds_dn5), (locals.var_ps0_dn6 + locals.var_pds_dn6), (locals.var_ps0_dn7 + locals.var_pds_dn7), (locals.var_ps0_dn8 + locals.var_pds_dn8), (locals.var_ps0_dn9 + locals.var_pds_dn9), (locals.var_ps0_dn10 + locals.var_pds_dn10), (locals.var_ps0_dn11 + locals.var_pds_dn11), (locals.var_ps0_dn14 + locals.var_pds_dn14),)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn4, locals.var_psl_dn5, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn8, locals.var_psl_dn9, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn14,)
    }
};
        locals.var_psl = assign59350_e92300;
        locals.var_psl_dn0 = assign59350_e92300_d_n0;
        locals.var_psl_dn2 = assign59350_e92300_d_n2;
        locals.var_psl_dn4 = assign59350_e92300_d_n4;
        locals.var_psl_dn5 = assign59350_e92300_d_n5;
        locals.var_psl_dn6 = assign59350_e92300_d_n6;
        locals.var_psl_dn7 = assign59350_e92300_d_n7;
        locals.var_psl_dn8 = assign59350_e92300_d_n8;
        locals.var_psl_dn9 = assign59350_e92300_d_n9;
        locals.var_psl_dn10 = assign59350_e92300_d_n10;
        locals.var_psl_dn11 = assign59350_e92300_d_n11;
        locals.var_psl_dn14 = assign59350_e92300_d_n14;
        locals.var_psl_rv = 0.0;

        let (assign59360_e92309,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1452 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign59360_e92309;
        locals.var_flg_conv_rv = 0.0;

        let (assign59370_e92318,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_start_of_loopl != 0.0)) {
        (0.0,)
    } else {
        (locals.var_start_of_loopl,)
    }
};
        locals.var_start_of_loopl = assign59370_e92318;
        locals.var_start_of_loopl_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_219(
        locals: &mut StampLocals,
    ) {
        let (assign59380_e92325,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_sl,)
    }
};
        locals.var_lp_sl = assign59380_e92325;
        locals.var_lp_sl_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_220(
        locals: &mut StampLocals,
    ) {
        let mut assign59390_loop_guard: usize = 0;
        while {
            let assign59390_cond_e92333: f64 = (40.0 + 1.0);
            let assign59390_cond_e92335: f64 = if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_lp_sl <= assign59390_cond_e92333)) { 1.0 } else { 0.0 };
            assign59390_cond_e92335 != 0.0
        } {
            assign59390_loop_guard += 1;
            assert!(assign59390_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign59390_body1_e92355, assign59390_body1_e92355_d_n0, assign59390_body1_e92355_d_n2, assign59390_body1_e92355_d_n4, assign59390_body1_e92355_d_n5, assign59390_body1_e92355_d_n6, assign59390_body1_e92355_d_n7, assign59390_body1_e92355_d_n8, assign59390_body1_e92355_d_n9, assign59390_body1_e92355_d_n10, assign59390_body1_e92355_d_n11, assign59390_body1_e92355_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign59390_body1_e92352: f64 = (locals.var_psl - locals.var_vbscl__blk439);
        let assign59390_body1_e92353: f64 = (locals.var_beta * assign59390_body1_e92352);
        (assign59390_body1_e92353, ((locals.var_beta_dn0 * assign59390_body1_e92352) + (locals.var_beta * (locals.var_psl_dn0 - locals.var_vbscl__blk439_dn0))), ((locals.var_beta_dn2 * assign59390_body1_e92352) + (locals.var_beta * (locals.var_psl_dn2 - locals.var_vbscl__blk439_dn2))), ((locals.var_beta_dn4 * assign59390_body1_e92352) + (locals.var_beta * (locals.var_psl_dn4 - locals.var_vbscl__blk439_dn4))), ((locals.var_beta_dn5 * assign59390_body1_e92352) + (locals.var_beta * (locals.var_psl_dn5 - locals.var_vbscl__blk439_dn5))), ((locals.var_beta_dn6 * assign59390_body1_e92352) + (locals.var_beta * (locals.var_psl_dn6 - locals.var_vbscl__blk439_dn6))), ((locals.var_beta_dn7 * assign59390_body1_e92352) + (locals.var_beta * (locals.var_psl_dn7 - locals.var_vbscl__blk439_dn7))), ((locals.var_beta_dn8 * assign59390_body1_e92352) + (locals.var_beta * (locals.var_psl_dn8 - locals.var_vbscl__blk439_dn8))), ((locals.var_beta_dn9 * assign59390_body1_e92352) + (locals.var_beta * (locals.var_psl_dn9 - locals.var_vbscl__blk439_dn9))), ((locals.var_beta_dn10 * assign59390_body1_e92352) + (locals.var_beta * (locals.var_psl_dn10 - locals.var_vbscl__blk439_dn10))), ((locals.var_beta_dn11 * assign59390_body1_e92352) + (locals.var_beta * (locals.var_psl_dn11 - locals.var_vbscl__blk439_dn11))), ((locals.var_beta_dn14 * assign59390_body1_e92352) + (locals.var_beta * (locals.var_psl_dn14 - locals.var_vbscl__blk439_dn14))),)
    } else {
        (locals.var_chi, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    }
};
            locals.var_chi = assign59390_body1_e92355;
            locals.var_chi_dn0 = assign59390_body1_e92355_d_n0;
            locals.var_chi_dn2 = assign59390_body1_e92355_d_n2;
            locals.var_chi_dn4 = assign59390_body1_e92355_d_n4;
            locals.var_chi_dn5 = assign59390_body1_e92355_d_n5;
            locals.var_chi_dn6 = assign59390_body1_e92355_d_n6;
            locals.var_chi_dn7 = assign59390_body1_e92355_d_n7;
            locals.var_chi_dn8 = assign59390_body1_e92355_d_n8;
            locals.var_chi_dn9 = assign59390_body1_e92355_d_n9;
            locals.var_chi_dn10 = assign59390_body1_e92355_d_n10;
            locals.var_chi_dn11 = assign59390_body1_e92355_d_n11;
            locals.var_chi_dn14 = assign59390_body1_e92355_d_n14;
            locals.var_chi_rv = 0.0;
            let assign59390_body2_e92358: f64 = if locals.var_chi < 5.0 { 1.0 } else { 0.0 };
            locals.var_guard1457 = assign59390_body2_e92358;
            locals.var_guard1457_rv = 0.0;
            let (assign59390_body3_e92382, assign59390_body3_e92382_d_n0, assign59390_body3_e92382_d_n2, assign59390_body3_e92382_d_n4, assign59390_body3_e92382_d_n5, assign59390_body3_e92382_d_n6, assign59390_body3_e92382_d_n7, assign59390_body3_e92382_d_n8, assign59390_body3_e92382_d_n9, assign59390_body3_e92382_d_n10, assign59390_body3_e92382_d_n11, assign59390_body3_e92382_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1457 != 0.0)) {
        let assign59390_body3_e92367: f64 = (locals.var_chi * locals.var_chi);
        let assign59390_body3_e92369: f64 = (assign59390_body3_e92367 * locals.var_chi);
        let assign59390_body3_e92373: f64 = (-0.07053654284009761);
        let assign59390_body3_e92376: f64 = (locals.var_chi * 0.006115288895133179);
        let assign59390_body3_e92377: f64 = (assign59390_body3_e92373 + assign59390_body3_e92376);
        let assign59390_body3_e92378: f64 = (locals.var_chi * assign59390_body3_e92377);
        let assign59390_body3_e92379: f64 = (0.29693154855771 + assign59390_body3_e92378);
        let assign59390_body3_e92380: f64 = (assign59390_body3_e92369 * assign59390_body3_e92379);
        (assign59390_body3_e92380, ((((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) * locals.var_chi) + (assign59390_body3_e92367 * locals.var_chi_dn0)) * assign59390_body3_e92379) + (assign59390_body3_e92369 * ((locals.var_chi_dn0 * assign59390_body3_e92377) + (locals.var_chi * (locals.var_chi_dn0 * 0.006115288895133179))))), ((((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) * locals.var_chi) + (assign59390_body3_e92367 * locals.var_chi_dn2)) * assign59390_body3_e92379) + (assign59390_body3_e92369 * ((locals.var_chi_dn2 * assign59390_body3_e92377) + (locals.var_chi * (locals.var_chi_dn2 * 0.006115288895133179))))), ((((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) * locals.var_chi) + (assign59390_body3_e92367 * locals.var_chi_dn4)) * assign59390_body3_e92379) + (assign59390_body3_e92369 * ((locals.var_chi_dn4 * assign59390_body3_e92377) + (locals.var_chi * (locals.var_chi_dn4 * 0.006115288895133179))))), ((((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) * locals.var_chi) + (assign59390_body3_e92367 * locals.var_chi_dn5)) * assign59390_body3_e92379) + (assign59390_body3_e92369 * ((locals.var_chi_dn5 * assign59390_body3_e92377) + (locals.var_chi * (locals.var_chi_dn5 * 0.006115288895133179))))), ((((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) * locals.var_chi) + (assign59390_body3_e92367 * locals.var_chi_dn6)) * assign59390_body3_e92379) + (assign59390_body3_e92369 * ((locals.var_chi_dn6 * assign59390_body3_e92377) + (locals.var_chi * (locals.var_chi_dn6 * 0.006115288895133179))))), ((((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) * locals.var_chi) + (assign59390_body3_e92367 * locals.var_chi_dn7)) * assign59390_body3_e92379) + (assign59390_body3_e92369 * ((locals.var_chi_dn7 * assign59390_body3_e92377) + (locals.var_chi * (locals.var_chi_dn7 * 0.006115288895133179))))), ((((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) * locals.var_chi) + (assign59390_body3_e92367 * locals.var_chi_dn8)) * assign59390_body3_e92379) + (assign59390_body3_e92369 * ((locals.var_chi_dn8 * assign59390_body3_e92377) + (locals.var_chi * (locals.var_chi_dn8 * 0.006115288895133179))))), ((((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) * locals.var_chi) + (assign59390_body3_e92367 * locals.var_chi_dn9)) * assign59390_body3_e92379) + (assign59390_body3_e92369 * ((locals.var_chi_dn9 * assign59390_body3_e92377) + (locals.var_chi * (locals.var_chi_dn9 * 0.006115288895133179))))), ((((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) * locals.var_chi) + (assign59390_body3_e92367 * locals.var_chi_dn10)) * assign59390_body3_e92379) + (assign59390_body3_e92369 * ((locals.var_chi_dn10 * assign59390_body3_e92377) + (locals.var_chi * (locals.var_chi_dn10 * 0.006115288895133179))))), ((((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) * locals.var_chi) + (assign59390_body3_e92367 * locals.var_chi_dn11)) * assign59390_body3_e92379) + (assign59390_body3_e92369 * ((locals.var_chi_dn11 * assign59390_body3_e92377) + (locals.var_chi * (locals.var_chi_dn11 * 0.006115288895133179))))), ((((((locals.var_chi_dn14 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn14)) * locals.var_chi) + (assign59390_body3_e92367 * locals.var_chi_dn14)) * assign59390_body3_e92379) + (assign59390_body3_e92369 * ((locals.var_chi_dn14 * assign59390_body3_e92377) + (locals.var_chi * (locals.var_chi_dn14 * 0.006115288895133179))))),)
    } else {
        (locals.var_fi, locals.var_fi_dn0, locals.var_fi_dn2, locals.var_fi_dn4, locals.var_fi_dn5, locals.var_fi_dn6, locals.var_fi_dn7, locals.var_fi_dn8, locals.var_fi_dn9, locals.var_fi_dn10, locals.var_fi_dn11, locals.var_fi_dn14,)
    }
};
            locals.var_fi = assign59390_body3_e92382;
            locals.var_fi_dn0 = assign59390_body3_e92382_d_n0;
            locals.var_fi_dn2 = assign59390_body3_e92382_d_n2;
            locals.var_fi_dn4 = assign59390_body3_e92382_d_n4;
            locals.var_fi_dn5 = assign59390_body3_e92382_d_n5;
            locals.var_fi_dn6 = assign59390_body3_e92382_d_n6;
            locals.var_fi_dn7 = assign59390_body3_e92382_d_n7;
            locals.var_fi_dn8 = assign59390_body3_e92382_d_n8;
            locals.var_fi_dn9 = assign59390_body3_e92382_d_n9;
            locals.var_fi_dn10 = assign59390_body3_e92382_d_n10;
            locals.var_fi_dn11 = assign59390_body3_e92382_d_n11;
            locals.var_fi_dn14 = assign59390_body3_e92382_d_n14;
            locals.var_fi_rv = 0.0;
            let (assign59390_body4_e92410, assign59390_body4_e92410_d_n0, assign59390_body4_e92410_d_n2, assign59390_body4_e92410_d_n4, assign59390_body4_e92410_d_n5, assign59390_body4_e92410_d_n6, assign59390_body4_e92410_d_n7, assign59390_body4_e92410_d_n8, assign59390_body4_e92410_d_n9, assign59390_body4_e92410_d_n10, assign59390_body4_e92410_d_n11, assign59390_body4_e92410_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1457 != 0.0)) {
        let assign59390_body4_e92391: f64 = (locals.var_chi * locals.var_chi);
        let assign59390_body4_e92394: f64 = (3.0 * 0.29693154855771);
        let assign59390_body4_e92398: f64 = (-0.07053654284009761);
        let assign59390_body4_e92399: f64 = (4.0 * assign59390_body4_e92398);
        let assign59390_body4_e92402: f64 = (locals.var_chi * 5.0);
        let assign59390_body4_e92404: f64 = (assign59390_body4_e92402 * 0.006115288895133179);
        let assign59390_body4_e92405: f64 = (assign59390_body4_e92399 + assign59390_body4_e92404);
        let assign59390_body4_e92406: f64 = (locals.var_chi * assign59390_body4_e92405);
        let assign59390_body4_e92407: f64 = (assign59390_body4_e92394 + assign59390_body4_e92406);
        let assign59390_body4_e92408: f64 = (assign59390_body4_e92391 * assign59390_body4_e92407);
        (assign59390_body4_e92408, ((((locals.var_chi_dn0 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn0)) * assign59390_body4_e92407) + (assign59390_body4_e92391 * ((locals.var_chi_dn0 * assign59390_body4_e92405) + (locals.var_chi * ((locals.var_chi_dn0 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn2 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn2)) * assign59390_body4_e92407) + (assign59390_body4_e92391 * ((locals.var_chi_dn2 * assign59390_body4_e92405) + (locals.var_chi * ((locals.var_chi_dn2 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn4 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn4)) * assign59390_body4_e92407) + (assign59390_body4_e92391 * ((locals.var_chi_dn4 * assign59390_body4_e92405) + (locals.var_chi * ((locals.var_chi_dn4 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn5 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn5)) * assign59390_body4_e92407) + (assign59390_body4_e92391 * ((locals.var_chi_dn5 * assign59390_body4_e92405) + (locals.var_chi * ((locals.var_chi_dn5 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn6 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn6)) * assign59390_body4_e92407) + (assign59390_body4_e92391 * ((locals.var_chi_dn6 * assign59390_body4_e92405) + (locals.var_chi * ((locals.var_chi_dn6 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn7 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn7)) * assign59390_body4_e92407) + (assign59390_body4_e92391 * ((locals.var_chi_dn7 * assign59390_body4_e92405) + (locals.var_chi * ((locals.var_chi_dn7 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn8 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn8)) * assign59390_body4_e92407) + (assign59390_body4_e92391 * ((locals.var_chi_dn8 * assign59390_body4_e92405) + (locals.var_chi * ((locals.var_chi_dn8 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn9 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn9)) * assign59390_body4_e92407) + (assign59390_body4_e92391 * ((locals.var_chi_dn9 * assign59390_body4_e92405) + (locals.var_chi * ((locals.var_chi_dn9 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn10 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn10)) * assign59390_body4_e92407) + (assign59390_body4_e92391 * ((locals.var_chi_dn10 * assign59390_body4_e92405) + (locals.var_chi * ((locals.var_chi_dn10 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn11 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn11)) * assign59390_body4_e92407) + (assign59390_body4_e92391 * ((locals.var_chi_dn11 * assign59390_body4_e92405) + (locals.var_chi * ((locals.var_chi_dn11 * 5.0) * 0.006115288895133179))))), ((((locals.var_chi_dn14 * locals.var_chi) + (locals.var_chi * locals.var_chi_dn14)) * assign59390_body4_e92407) + (assign59390_body4_e92391 * ((locals.var_chi_dn14 * assign59390_body4_e92405) + (locals.var_chi * ((locals.var_chi_dn14 * 5.0) * 0.006115288895133179))))),)
    } else {
        (locals.var_fi_dchi, locals.var_fi_dchi_dn0, locals.var_fi_dchi_dn2, locals.var_fi_dchi_dn4, locals.var_fi_dchi_dn5, locals.var_fi_dchi_dn6, locals.var_fi_dchi_dn7, locals.var_fi_dchi_dn8, locals.var_fi_dchi_dn9, locals.var_fi_dchi_dn10, locals.var_fi_dchi_dn11, locals.var_fi_dchi_dn14,)
    }
};
            locals.var_fi_dchi = assign59390_body4_e92410;
            locals.var_fi_dchi_dn0 = assign59390_body4_e92410_d_n0;
            locals.var_fi_dchi_dn2 = assign59390_body4_e92410_d_n2;
            locals.var_fi_dchi_dn4 = assign59390_body4_e92410_d_n4;
            locals.var_fi_dchi_dn5 = assign59390_body4_e92410_d_n5;
            locals.var_fi_dchi_dn6 = assign59390_body4_e92410_d_n6;
            locals.var_fi_dchi_dn7 = assign59390_body4_e92410_d_n7;
            locals.var_fi_dchi_dn8 = assign59390_body4_e92410_d_n8;
            locals.var_fi_dchi_dn9 = assign59390_body4_e92410_d_n9;
            locals.var_fi_dchi_dn10 = assign59390_body4_e92410_d_n10;
            locals.var_fi_dchi_dn11 = assign59390_body4_e92410_d_n11;
            locals.var_fi_dchi_dn14 = assign59390_body4_e92410_d_n14;
            locals.var_fi_dchi_rv = 0.0;
            let (assign59390_body5_e92421, assign59390_body5_e92421_d_n0, assign59390_body5_e92421_d_n2, assign59390_body5_e92421_d_n4, assign59390_body5_e92421_d_n5, assign59390_body5_e92421_d_n6, assign59390_body5_e92421_d_n7, assign59390_body5_e92421_d_n8, assign59390_body5_e92421_d_n9, assign59390_body5_e92421_d_n10, assign59390_body5_e92421_d_n11, assign59390_body5_e92421_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1457 != 0.0)) {
        let assign59390_body5_e92419: f64 = (locals.var_cnst1 * locals.var_exp_bvbsvds);
        (assign59390_body5_e92419, ((locals.var_cnst1_dn0 * locals.var_exp_bvbsvds) + (locals.var_cnst1 * locals.var_exp_bvbsvds_dn0)), ((locals.var_cnst1_dn2 * locals.var_exp_bvbsvds) + (locals.var_cnst1 * locals.var_exp_bvbsvds_dn2)), ((locals.var_cnst1_dn4 * locals.var_exp_bvbsvds) + (locals.var_cnst1 * locals.var_exp_bvbsvds_dn4)), ((locals.var_cnst1_dn5 * locals.var_exp_bvbsvds) + (locals.var_cnst1 * locals.var_exp_bvbsvds_dn5)), ((locals.var_cnst1_dn6 * locals.var_exp_bvbsvds) + (locals.var_cnst1 * locals.var_exp_bvbsvds_dn6)), ((locals.var_cnst1_dn7 * locals.var_exp_bvbsvds) + (locals.var_cnst1 * locals.var_exp_bvbsvds_dn7)), ((locals.var_cnst1_dn8 * locals.var_exp_bvbsvds) + (locals.var_cnst1 * locals.var_exp_bvbsvds_dn8)), ((locals.var_cnst1_dn9 * locals.var_exp_bvbsvds) + (locals.var_cnst1 * locals.var_exp_bvbsvds_dn9)), ((locals.var_cnst1_dn10 * locals.var_exp_bvbsvds) + (locals.var_cnst1 * locals.var_exp_bvbsvds_dn10)), ((locals.var_cnst1_dn11 * locals.var_exp_bvbsvds) + (locals.var_cnst1 * locals.var_exp_bvbsvds_dn11)), ((locals.var_cnst1_dn14 * locals.var_exp_bvbsvds) + (locals.var_cnst1 * locals.var_exp_bvbsvds_dn14)),)
    } else {
        (locals.var_cfs1, locals.var_cfs1_dn0, locals.var_cfs1_dn2, locals.var_cfs1_dn4, locals.var_cfs1_dn5, locals.var_cfs1_dn6, locals.var_cfs1_dn7, locals.var_cfs1_dn8, locals.var_cfs1_dn9, locals.var_cfs1_dn10, locals.var_cfs1_dn11, locals.var_cfs1_dn14,)
    }
};
            locals.var_cfs1 = assign59390_body5_e92421;
            locals.var_cfs1_dn0 = assign59390_body5_e92421_d_n0;
            locals.var_cfs1_dn2 = assign59390_body5_e92421_d_n2;
            locals.var_cfs1_dn4 = assign59390_body5_e92421_d_n4;
            locals.var_cfs1_dn5 = assign59390_body5_e92421_d_n5;
            locals.var_cfs1_dn6 = assign59390_body5_e92421_d_n6;
            locals.var_cfs1_dn7 = assign59390_body5_e92421_d_n7;
            locals.var_cfs1_dn8 = assign59390_body5_e92421_d_n8;
            locals.var_cfs1_dn9 = assign59390_body5_e92421_d_n9;
            locals.var_cfs1_dn10 = assign59390_body5_e92421_d_n10;
            locals.var_cfs1_dn11 = assign59390_body5_e92421_d_n11;
            locals.var_cfs1_dn14 = assign59390_body5_e92421_d_n14;
            locals.var_cfs1_rv = 0.0;
            let (assign59390_body6_e92434, assign59390_body6_e92434_d_n0, assign59390_body6_e92434_d_n2, assign59390_body6_e92434_d_n4, assign59390_body6_e92434_d_n5, assign59390_body6_e92434_d_n6, assign59390_body6_e92434_d_n7, assign59390_body6_e92434_d_n8, assign59390_body6_e92434_d_n9, assign59390_body6_e92434_d_n10, assign59390_body6_e92434_d_n11, assign59390_body6_e92434_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1457 != 0.0)) {
        let assign59390_body6_e92430: f64 = (locals.var_cfs1 * locals.var_fi);
        let assign59390_body6_e92432: f64 = (assign59390_body6_e92430 * locals.var_fi);
        (assign59390_body6_e92432, ((((locals.var_cfs1_dn0 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn0)) * locals.var_fi) + (assign59390_body6_e92430 * locals.var_fi_dn0)), ((((locals.var_cfs1_dn2 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn2)) * locals.var_fi) + (assign59390_body6_e92430 * locals.var_fi_dn2)), ((((locals.var_cfs1_dn4 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn4)) * locals.var_fi) + (assign59390_body6_e92430 * locals.var_fi_dn4)), ((((locals.var_cfs1_dn5 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn5)) * locals.var_fi) + (assign59390_body6_e92430 * locals.var_fi_dn5)), ((((locals.var_cfs1_dn6 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn6)) * locals.var_fi) + (assign59390_body6_e92430 * locals.var_fi_dn6)), ((((locals.var_cfs1_dn7 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn7)) * locals.var_fi) + (assign59390_body6_e92430 * locals.var_fi_dn7)), ((((locals.var_cfs1_dn8 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn8)) * locals.var_fi) + (assign59390_body6_e92430 * locals.var_fi_dn8)), ((((locals.var_cfs1_dn9 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn9)) * locals.var_fi) + (assign59390_body6_e92430 * locals.var_fi_dn9)), ((((locals.var_cfs1_dn10 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn10)) * locals.var_fi) + (assign59390_body6_e92430 * locals.var_fi_dn10)), ((((locals.var_cfs1_dn11 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn11)) * locals.var_fi) + (assign59390_body6_e92430 * locals.var_fi_dn11)), ((((locals.var_cfs1_dn14 * locals.var_fi) + (locals.var_cfs1 * locals.var_fi_dn14)) * locals.var_fi) + (assign59390_body6_e92430 * locals.var_fi_dn14)),)
    } else {
        (locals.var_fsl1, locals.var_fsl1_dn0, locals.var_fsl1_dn2, locals.var_fsl1_dn4, locals.var_fsl1_dn5, locals.var_fsl1_dn6, locals.var_fsl1_dn7, locals.var_fsl1_dn8, locals.var_fsl1_dn9, locals.var_fsl1_dn10, locals.var_fsl1_dn11, locals.var_fsl1_dn14,)
    }
};
            locals.var_fsl1 = assign59390_body6_e92434;
            locals.var_fsl1_dn0 = assign59390_body6_e92434_d_n0;
            locals.var_fsl1_dn2 = assign59390_body6_e92434_d_n2;
            locals.var_fsl1_dn4 = assign59390_body6_e92434_d_n4;
            locals.var_fsl1_dn5 = assign59390_body6_e92434_d_n5;
            locals.var_fsl1_dn6 = assign59390_body6_e92434_d_n6;
            locals.var_fsl1_dn7 = assign59390_body6_e92434_d_n7;
            locals.var_fsl1_dn8 = assign59390_body6_e92434_d_n8;
            locals.var_fsl1_dn9 = assign59390_body6_e92434_d_n9;
            locals.var_fsl1_dn10 = assign59390_body6_e92434_d_n10;
            locals.var_fsl1_dn11 = assign59390_body6_e92434_d_n11;
            locals.var_fsl1_dn14 = assign59390_body6_e92434_d_n14;
            locals.var_fsl1_rv = 0.0;
            let (assign59390_body7_e92451, assign59390_body7_e92451_d_n0, assign59390_body7_e92451_d_n2, assign59390_body7_e92451_d_n4, assign59390_body7_e92451_d_n5, assign59390_body7_e92451_d_n6, assign59390_body7_e92451_d_n7, assign59390_body7_e92451_d_n8, assign59390_body7_e92451_d_n9, assign59390_body7_e92451_d_n10, assign59390_body7_e92451_d_n11, assign59390_body7_e92451_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1457 != 0.0)) {
        let assign59390_body7_e92443: f64 = (locals.var_cfs1 * locals.var_beta);
        let assign59390_body7_e92445: f64 = (assign59390_body7_e92443 * 2.0);
        let assign59390_body7_e92447: f64 = (assign59390_body7_e92445 * locals.var_fi);
        let assign59390_body7_e92449: f64 = (assign59390_body7_e92447 * locals.var_fi_dchi);
        (assign59390_body7_e92449, (((((((locals.var_cfs1_dn0 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn0)) * 2.0) * locals.var_fi) + (assign59390_body7_e92445 * locals.var_fi_dn0)) * locals.var_fi_dchi) + (assign59390_body7_e92447 * locals.var_fi_dchi_dn0)), (((((((locals.var_cfs1_dn2 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn2)) * 2.0) * locals.var_fi) + (assign59390_body7_e92445 * locals.var_fi_dn2)) * locals.var_fi_dchi) + (assign59390_body7_e92447 * locals.var_fi_dchi_dn2)), (((((((locals.var_cfs1_dn4 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn4)) * 2.0) * locals.var_fi) + (assign59390_body7_e92445 * locals.var_fi_dn4)) * locals.var_fi_dchi) + (assign59390_body7_e92447 * locals.var_fi_dchi_dn4)), (((((((locals.var_cfs1_dn5 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn5)) * 2.0) * locals.var_fi) + (assign59390_body7_e92445 * locals.var_fi_dn5)) * locals.var_fi_dchi) + (assign59390_body7_e92447 * locals.var_fi_dchi_dn5)), (((((((locals.var_cfs1_dn6 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn6)) * 2.0) * locals.var_fi) + (assign59390_body7_e92445 * locals.var_fi_dn6)) * locals.var_fi_dchi) + (assign59390_body7_e92447 * locals.var_fi_dchi_dn6)), (((((((locals.var_cfs1_dn7 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn7)) * 2.0) * locals.var_fi) + (assign59390_body7_e92445 * locals.var_fi_dn7)) * locals.var_fi_dchi) + (assign59390_body7_e92447 * locals.var_fi_dchi_dn7)), (((((((locals.var_cfs1_dn8 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn8)) * 2.0) * locals.var_fi) + (assign59390_body7_e92445 * locals.var_fi_dn8)) * locals.var_fi_dchi) + (assign59390_body7_e92447 * locals.var_fi_dchi_dn8)), (((((((locals.var_cfs1_dn9 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn9)) * 2.0) * locals.var_fi) + (assign59390_body7_e92445 * locals.var_fi_dn9)) * locals.var_fi_dchi) + (assign59390_body7_e92447 * locals.var_fi_dchi_dn9)), (((((((locals.var_cfs1_dn10 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn10)) * 2.0) * locals.var_fi) + (assign59390_body7_e92445 * locals.var_fi_dn10)) * locals.var_fi_dchi) + (assign59390_body7_e92447 * locals.var_fi_dchi_dn10)), (((((((locals.var_cfs1_dn11 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn11)) * 2.0) * locals.var_fi) + (assign59390_body7_e92445 * locals.var_fi_dn11)) * locals.var_fi_dchi) + (assign59390_body7_e92447 * locals.var_fi_dchi_dn11)), (((((((locals.var_cfs1_dn14 * locals.var_beta) + (locals.var_cfs1 * locals.var_beta_dn14)) * 2.0) * locals.var_fi) + (assign59390_body7_e92445 * locals.var_fi_dn14)) * locals.var_fi_dchi) + (assign59390_body7_e92447 * locals.var_fi_dchi_dn14)),)
    } else {
        (locals.var_fsl1_dpsl, locals.var_fsl1_dpsl_dn0, locals.var_fsl1_dpsl_dn2, locals.var_fsl1_dpsl_dn4, locals.var_fsl1_dpsl_dn5, locals.var_fsl1_dpsl_dn6, locals.var_fsl1_dpsl_dn7, locals.var_fsl1_dpsl_dn8, locals.var_fsl1_dpsl_dn9, locals.var_fsl1_dpsl_dn10, locals.var_fsl1_dpsl_dn11, locals.var_fsl1_dpsl_dn14,)
    }
};
            locals.var_fsl1_dpsl = assign59390_body7_e92451;
            locals.var_fsl1_dpsl_dn0 = assign59390_body7_e92451_d_n0;
            locals.var_fsl1_dpsl_dn2 = assign59390_body7_e92451_d_n2;
            locals.var_fsl1_dpsl_dn4 = assign59390_body7_e92451_d_n4;
            locals.var_fsl1_dpsl_dn5 = assign59390_body7_e92451_d_n5;
            locals.var_fsl1_dpsl_dn6 = assign59390_body7_e92451_d_n6;
            locals.var_fsl1_dpsl_dn7 = assign59390_body7_e92451_d_n7;
            locals.var_fsl1_dpsl_dn8 = assign59390_body7_e92451_d_n8;
            locals.var_fsl1_dpsl_dn9 = assign59390_body7_e92451_d_n9;
            locals.var_fsl1_dpsl_dn10 = assign59390_body7_e92451_d_n10;
            locals.var_fsl1_dpsl_dn11 = assign59390_body7_e92451_d_n11;
            locals.var_fsl1_dpsl_dn14 = assign59390_body7_e92451_d_n14;
            locals.var_fsl1_dpsl_rv = 0.0;
            let (assign59390_body8_e92480, assign59390_body8_e92480_d_n0, assign59390_body8_e92480_d_n2, assign59390_body8_e92480_d_n4, assign59390_body8_e92480_d_n5, assign59390_body8_e92480_d_n6, assign59390_body8_e92480_d_n7, assign59390_body8_e92480_d_n8, assign59390_body8_e92480_d_n9, assign59390_body8_e92480_d_n10, assign59390_body8_e92480_d_n11, assign59390_body8_e92480_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1457 != 0.0)) {
        let assign59390_body8_e92462: f64 = (-0.117851130197758);
        let assign59390_body8_e92467: f64 = (-0.00163730162779191);
        let assign59390_body8_e92470: f64 = (locals.var_chi * 6.36964918866352e-5);
        let assign59390_body8_e92471: f64 = (assign59390_body8_e92467 + assign59390_body8_e92470);
        let assign59390_body8_e92472: f64 = (locals.var_chi * assign59390_body8_e92471);
        let assign59390_body8_e92473: f64 = (0.0178800506338833 + assign59390_body8_e92472);
        let assign59390_body8_e92474: f64 = (locals.var_chi * assign59390_body8_e92473);
        let assign59390_body8_e92475: f64 = (assign59390_body8_e92462 + assign59390_body8_e92474);
        let assign59390_body8_e92476: f64 = (locals.var_chi * assign59390_body8_e92475);
        let assign59390_body8_e92477: f64 = (0.707106781186548 + assign59390_body8_e92476);
        let assign59390_body8_e92478: f64 = (locals.var_chi * assign59390_body8_e92477);
        (assign59390_body8_e92478, ((locals.var_chi_dn0 * assign59390_body8_e92477) + (locals.var_chi * ((locals.var_chi_dn0 * assign59390_body8_e92475) + (locals.var_chi * ((locals.var_chi_dn0 * assign59390_body8_e92473) + (locals.var_chi * ((locals.var_chi_dn0 * assign59390_body8_e92471) + (locals.var_chi * (locals.var_chi_dn0 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn2 * assign59390_body8_e92477) + (locals.var_chi * ((locals.var_chi_dn2 * assign59390_body8_e92475) + (locals.var_chi * ((locals.var_chi_dn2 * assign59390_body8_e92473) + (locals.var_chi * ((locals.var_chi_dn2 * assign59390_body8_e92471) + (locals.var_chi * (locals.var_chi_dn2 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn4 * assign59390_body8_e92477) + (locals.var_chi * ((locals.var_chi_dn4 * assign59390_body8_e92475) + (locals.var_chi * ((locals.var_chi_dn4 * assign59390_body8_e92473) + (locals.var_chi * ((locals.var_chi_dn4 * assign59390_body8_e92471) + (locals.var_chi * (locals.var_chi_dn4 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn5 * assign59390_body8_e92477) + (locals.var_chi * ((locals.var_chi_dn5 * assign59390_body8_e92475) + (locals.var_chi * ((locals.var_chi_dn5 * assign59390_body8_e92473) + (locals.var_chi * ((locals.var_chi_dn5 * assign59390_body8_e92471) + (locals.var_chi * (locals.var_chi_dn5 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn6 * assign59390_body8_e92477) + (locals.var_chi * ((locals.var_chi_dn6 * assign59390_body8_e92475) + (locals.var_chi * ((locals.var_chi_dn6 * assign59390_body8_e92473) + (locals.var_chi * ((locals.var_chi_dn6 * assign59390_body8_e92471) + (locals.var_chi * (locals.var_chi_dn6 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn7 * assign59390_body8_e92477) + (locals.var_chi * ((locals.var_chi_dn7 * assign59390_body8_e92475) + (locals.var_chi * ((locals.var_chi_dn7 * assign59390_body8_e92473) + (locals.var_chi * ((locals.var_chi_dn7 * assign59390_body8_e92471) + (locals.var_chi * (locals.var_chi_dn7 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn8 * assign59390_body8_e92477) + (locals.var_chi * ((locals.var_chi_dn8 * assign59390_body8_e92475) + (locals.var_chi * ((locals.var_chi_dn8 * assign59390_body8_e92473) + (locals.var_chi * ((locals.var_chi_dn8 * assign59390_body8_e92471) + (locals.var_chi * (locals.var_chi_dn8 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn9 * assign59390_body8_e92477) + (locals.var_chi * ((locals.var_chi_dn9 * assign59390_body8_e92475) + (locals.var_chi * ((locals.var_chi_dn9 * assign59390_body8_e92473) + (locals.var_chi * ((locals.var_chi_dn9 * assign59390_body8_e92471) + (locals.var_chi * (locals.var_chi_dn9 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn10 * assign59390_body8_e92477) + (locals.var_chi * ((locals.var_chi_dn10 * assign59390_body8_e92475) + (locals.var_chi * ((locals.var_chi_dn10 * assign59390_body8_e92473) + (locals.var_chi * ((locals.var_chi_dn10 * assign59390_body8_e92471) + (locals.var_chi * (locals.var_chi_dn10 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn11 * assign59390_body8_e92477) + (locals.var_chi * ((locals.var_chi_dn11 * assign59390_body8_e92475) + (locals.var_chi * ((locals.var_chi_dn11 * assign59390_body8_e92473) + (locals.var_chi * ((locals.var_chi_dn11 * assign59390_body8_e92471) + (locals.var_chi * (locals.var_chi_dn11 * 6.36964918866352e-5))))))))), ((locals.var_chi_dn14 * assign59390_body8_e92477) + (locals.var_chi * ((locals.var_chi_dn14 * assign59390_body8_e92475) + (locals.var_chi * ((locals.var_chi_dn14 * assign59390_body8_e92473) + (locals.var_chi * ((locals.var_chi_dn14 * assign59390_body8_e92471) + (locals.var_chi * (locals.var_chi_dn14 * 6.36964918866352e-5))))))))),)
    } else {
        (locals.var_fb, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    }
};
            locals.var_fb = assign59390_body8_e92480;
            locals.var_fb_dn0 = assign59390_body8_e92480_d_n0;
            locals.var_fb_dn2 = assign59390_body8_e92480_d_n2;
            locals.var_fb_dn4 = assign59390_body8_e92480_d_n4;
            locals.var_fb_dn5 = assign59390_body8_e92480_d_n5;
            locals.var_fb_dn6 = assign59390_body8_e92480_d_n6;
            locals.var_fb_dn7 = assign59390_body8_e92480_d_n7;
            locals.var_fb_dn8 = assign59390_body8_e92480_d_n8;
            locals.var_fb_dn9 = assign59390_body8_e92480_d_n9;
            locals.var_fb_dn10 = assign59390_body8_e92480_d_n10;
            locals.var_fb_dn11 = assign59390_body8_e92480_d_n11;
            locals.var_fb_dn14 = assign59390_body8_e92480_d_n14;
            locals.var_fb_rv = 0.0;
            let (assign59390_body9_e92515, assign59390_body9_e92515_d_n0, assign59390_body9_e92515_d_n2, assign59390_body9_e92515_d_n4, assign59390_body9_e92515_d_n5, assign59390_body9_e92515_d_n6, assign59390_body9_e92515_d_n7, assign59390_body9_e92515_d_n8, assign59390_body9_e92515_d_n9, assign59390_body9_e92515_d_n10, assign59390_body9_e92515_d_n11, assign59390_body9_e92515_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1457 != 0.0)) {
        let assign59390_body9_e92491: f64 = (-0.117851130197758);
        let assign59390_body9_e92492: f64 = (2.0 * assign59390_body9_e92491);
        let assign59390_body9_e92496: f64 = (3.0 * 0.0178800506338833);
        let assign59390_body9_e92500: f64 = (-0.00163730162779191);
        let assign59390_body9_e92501: f64 = (4.0 * assign59390_body9_e92500);
        let assign59390_body9_e92504: f64 = (locals.var_chi * 5.0);
        let assign59390_body9_e92506: f64 = (assign59390_body9_e92504 * 6.36964918866352e-5);
        let assign59390_body9_e92507: f64 = (assign59390_body9_e92501 + assign59390_body9_e92506);
        let assign59390_body9_e92508: f64 = (locals.var_chi * assign59390_body9_e92507);
        let assign59390_body9_e92509: f64 = (assign59390_body9_e92496 + assign59390_body9_e92508);
        let assign59390_body9_e92510: f64 = (locals.var_chi * assign59390_body9_e92509);
        let assign59390_body9_e92511: f64 = (assign59390_body9_e92492 + assign59390_body9_e92510);
        let assign59390_body9_e92512: f64 = (locals.var_chi * assign59390_body9_e92511);
        let assign59390_body9_e92513: f64 = (0.707106781186548 + assign59390_body9_e92512);
        (assign59390_body9_e92513, ((locals.var_chi_dn0 * assign59390_body9_e92511) + (locals.var_chi * ((locals.var_chi_dn0 * assign59390_body9_e92509) + (locals.var_chi * ((locals.var_chi_dn0 * assign59390_body9_e92507) + (locals.var_chi * ((locals.var_chi_dn0 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn2 * assign59390_body9_e92511) + (locals.var_chi * ((locals.var_chi_dn2 * assign59390_body9_e92509) + (locals.var_chi * ((locals.var_chi_dn2 * assign59390_body9_e92507) + (locals.var_chi * ((locals.var_chi_dn2 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn4 * assign59390_body9_e92511) + (locals.var_chi * ((locals.var_chi_dn4 * assign59390_body9_e92509) + (locals.var_chi * ((locals.var_chi_dn4 * assign59390_body9_e92507) + (locals.var_chi * ((locals.var_chi_dn4 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn5 * assign59390_body9_e92511) + (locals.var_chi * ((locals.var_chi_dn5 * assign59390_body9_e92509) + (locals.var_chi * ((locals.var_chi_dn5 * assign59390_body9_e92507) + (locals.var_chi * ((locals.var_chi_dn5 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn6 * assign59390_body9_e92511) + (locals.var_chi * ((locals.var_chi_dn6 * assign59390_body9_e92509) + (locals.var_chi * ((locals.var_chi_dn6 * assign59390_body9_e92507) + (locals.var_chi * ((locals.var_chi_dn6 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn7 * assign59390_body9_e92511) + (locals.var_chi * ((locals.var_chi_dn7 * assign59390_body9_e92509) + (locals.var_chi * ((locals.var_chi_dn7 * assign59390_body9_e92507) + (locals.var_chi * ((locals.var_chi_dn7 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn8 * assign59390_body9_e92511) + (locals.var_chi * ((locals.var_chi_dn8 * assign59390_body9_e92509) + (locals.var_chi * ((locals.var_chi_dn8 * assign59390_body9_e92507) + (locals.var_chi * ((locals.var_chi_dn8 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn9 * assign59390_body9_e92511) + (locals.var_chi * ((locals.var_chi_dn9 * assign59390_body9_e92509) + (locals.var_chi * ((locals.var_chi_dn9 * assign59390_body9_e92507) + (locals.var_chi * ((locals.var_chi_dn9 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn10 * assign59390_body9_e92511) + (locals.var_chi * ((locals.var_chi_dn10 * assign59390_body9_e92509) + (locals.var_chi * ((locals.var_chi_dn10 * assign59390_body9_e92507) + (locals.var_chi * ((locals.var_chi_dn10 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn11 * assign59390_body9_e92511) + (locals.var_chi * ((locals.var_chi_dn11 * assign59390_body9_e92509) + (locals.var_chi * ((locals.var_chi_dn11 * assign59390_body9_e92507) + (locals.var_chi * ((locals.var_chi_dn11 * 5.0) * 6.36964918866352e-5))))))), ((locals.var_chi_dn14 * assign59390_body9_e92511) + (locals.var_chi * ((locals.var_chi_dn14 * assign59390_body9_e92509) + (locals.var_chi * ((locals.var_chi_dn14 * assign59390_body9_e92507) + (locals.var_chi * ((locals.var_chi_dn14 * 5.0) * 6.36964918866352e-5))))))),)
    } else {
        (locals.var_fb_dchi, locals.var_fb_dchi_dn0, locals.var_fb_dchi_dn2, locals.var_fb_dchi_dn4, locals.var_fb_dchi_dn5, locals.var_fb_dchi_dn6, locals.var_fb_dchi_dn7, locals.var_fb_dchi_dn8, locals.var_fb_dchi_dn9, locals.var_fb_dchi_dn10, locals.var_fb_dchi_dn11, locals.var_fb_dchi_dn14,)
    }
};
            locals.var_fb_dchi = assign59390_body9_e92515;
            locals.var_fb_dchi_dn0 = assign59390_body9_e92515_d_n0;
            locals.var_fb_dchi_dn2 = assign59390_body9_e92515_d_n2;
            locals.var_fb_dchi_dn4 = assign59390_body9_e92515_d_n4;
            locals.var_fb_dchi_dn5 = assign59390_body9_e92515_d_n5;
            locals.var_fb_dchi_dn6 = assign59390_body9_e92515_d_n6;
            locals.var_fb_dchi_dn7 = assign59390_body9_e92515_d_n7;
            locals.var_fb_dchi_dn8 = assign59390_body9_e92515_d_n8;
            locals.var_fb_dchi_dn9 = assign59390_body9_e92515_d_n9;
            locals.var_fb_dchi_dn10 = assign59390_body9_e92515_d_n10;
            locals.var_fb_dchi_dn11 = assign59390_body9_e92515_d_n11;
            locals.var_fb_dchi_dn14 = assign59390_body9_e92515_d_n14;
            locals.var_fb_dchi_rv = 0.0;
            let (assign59390_body10_e92529, assign59390_body10_e92529_d_n0, assign59390_body10_e92529_d_n2, assign59390_body10_e92529_d_n4, assign59390_body10_e92529_d_n5, assign59390_body10_e92529_d_n6, assign59390_body10_e92529_d_n7, assign59390_body10_e92529_d_n8, assign59390_body10_e92529_d_n9, assign59390_body10_e92529_d_n10, assign59390_body10_e92529_d_n11, assign59390_body10_e92529_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1457 != 0.0)) {
        let assign59390_body10_e92524: f64 = (locals.var_fb * locals.var_fb);
        let assign59390_body10_e92526: f64 = (assign59390_body10_e92524 + locals.var_fsl1);
        let assign59390_body10_e92527: f64 = (assign59390_body10_e92526).sqrt();
        (assign59390_body10_e92527, ((((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)) + locals.var_fsl1_dn0) / (2.0 * assign59390_body10_e92527)), ((((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)) + locals.var_fsl1_dn2) / (2.0 * assign59390_body10_e92527)), ((((locals.var_fb_dn4 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn4)) + locals.var_fsl1_dn4) / (2.0 * assign59390_body10_e92527)), ((((locals.var_fb_dn5 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn5)) + locals.var_fsl1_dn5) / (2.0 * assign59390_body10_e92527)), ((((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)) + locals.var_fsl1_dn6) / (2.0 * assign59390_body10_e92527)), ((((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)) + locals.var_fsl1_dn7) / (2.0 * assign59390_body10_e92527)), ((((locals.var_fb_dn8 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn8)) + locals.var_fsl1_dn8) / (2.0 * assign59390_body10_e92527)), ((((locals.var_fb_dn9 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn9)) + locals.var_fsl1_dn9) / (2.0 * assign59390_body10_e92527)), ((((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)) + locals.var_fsl1_dn10) / (2.0 * assign59390_body10_e92527)), ((((locals.var_fb_dn11 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn11)) + locals.var_fsl1_dn11) / (2.0 * assign59390_body10_e92527)), ((((locals.var_fb_dn14 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn14)) + locals.var_fsl1_dn14) / (2.0 * assign59390_body10_e92527)),)
    } else {
        (locals.var_fsl2, locals.var_fsl2_dn0, locals.var_fsl2_dn2, locals.var_fsl2_dn4, locals.var_fsl2_dn5, locals.var_fsl2_dn6, locals.var_fsl2_dn7, locals.var_fsl2_dn8, locals.var_fsl2_dn9, locals.var_fsl2_dn10, locals.var_fsl2_dn11, locals.var_fsl2_dn14,)
    }
};
            locals.var_fsl2 = assign59390_body10_e92529;
            locals.var_fsl2_dn0 = assign59390_body10_e92529_d_n0;
            locals.var_fsl2_dn2 = assign59390_body10_e92529_d_n2;
            locals.var_fsl2_dn4 = assign59390_body10_e92529_d_n4;
            locals.var_fsl2_dn5 = assign59390_body10_e92529_d_n5;
            locals.var_fsl2_dn6 = assign59390_body10_e92529_d_n6;
            locals.var_fsl2_dn7 = assign59390_body10_e92529_d_n7;
            locals.var_fsl2_dn8 = assign59390_body10_e92529_d_n8;
            locals.var_fsl2_dn9 = assign59390_body10_e92529_d_n9;
            locals.var_fsl2_dn10 = assign59390_body10_e92529_d_n10;
            locals.var_fsl2_dn11 = assign59390_body10_e92529_d_n11;
            locals.var_fsl2_dn14 = assign59390_body10_e92529_d_n14;
            locals.var_fsl2_rv = 0.0;
            let (assign59390_body11_e92550, assign59390_body11_e92550_d_n0, assign59390_body11_e92550_d_n2, assign59390_body11_e92550_d_n4, assign59390_body11_e92550_d_n5, assign59390_body11_e92550_d_n6, assign59390_body11_e92550_d_n7, assign59390_body11_e92550_d_n8, assign59390_body11_e92550_d_n9, assign59390_body11_e92550_d_n10, assign59390_body11_e92550_d_n11, assign59390_body11_e92550_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1457 != 0.0)) {
        let assign59390_body11_e92538: f64 = (locals.var_beta * locals.var_fb_dchi);
        let assign59390_body11_e92540: f64 = (assign59390_body11_e92538 * 2.0);
        let assign59390_body11_e92542: f64 = (assign59390_body11_e92540 * locals.var_fb);
        let assign59390_body11_e92544: f64 = (assign59390_body11_e92542 + locals.var_fsl1_dpsl);
        let assign59390_body11_e92547: f64 = (locals.var_fsl2 + locals.var_fsl2);
        let assign59390_body11_e92548: f64 = (assign59390_body11_e92544 / assign59390_body11_e92547);
        (assign59390_body11_e92548, (((((((((locals.var_beta_dn0 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn0)) * 2.0) * locals.var_fb) + (assign59390_body11_e92540 * locals.var_fb_dn0)) + locals.var_fsl1_dpsl_dn0) * assign59390_body11_e92547) - (assign59390_body11_e92544 * (locals.var_fsl2_dn0 + locals.var_fsl2_dn0))) / (assign59390_body11_e92547 * assign59390_body11_e92547)), (((((((((locals.var_beta_dn2 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn2)) * 2.0) * locals.var_fb) + (assign59390_body11_e92540 * locals.var_fb_dn2)) + locals.var_fsl1_dpsl_dn2) * assign59390_body11_e92547) - (assign59390_body11_e92544 * (locals.var_fsl2_dn2 + locals.var_fsl2_dn2))) / (assign59390_body11_e92547 * assign59390_body11_e92547)), (((((((((locals.var_beta_dn4 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn4)) * 2.0) * locals.var_fb) + (assign59390_body11_e92540 * locals.var_fb_dn4)) + locals.var_fsl1_dpsl_dn4) * assign59390_body11_e92547) - (assign59390_body11_e92544 * (locals.var_fsl2_dn4 + locals.var_fsl2_dn4))) / (assign59390_body11_e92547 * assign59390_body11_e92547)), (((((((((locals.var_beta_dn5 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn5)) * 2.0) * locals.var_fb) + (assign59390_body11_e92540 * locals.var_fb_dn5)) + locals.var_fsl1_dpsl_dn5) * assign59390_body11_e92547) - (assign59390_body11_e92544 * (locals.var_fsl2_dn5 + locals.var_fsl2_dn5))) / (assign59390_body11_e92547 * assign59390_body11_e92547)), (((((((((locals.var_beta_dn6 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn6)) * 2.0) * locals.var_fb) + (assign59390_body11_e92540 * locals.var_fb_dn6)) + locals.var_fsl1_dpsl_dn6) * assign59390_body11_e92547) - (assign59390_body11_e92544 * (locals.var_fsl2_dn6 + locals.var_fsl2_dn6))) / (assign59390_body11_e92547 * assign59390_body11_e92547)), (((((((((locals.var_beta_dn7 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn7)) * 2.0) * locals.var_fb) + (assign59390_body11_e92540 * locals.var_fb_dn7)) + locals.var_fsl1_dpsl_dn7) * assign59390_body11_e92547) - (assign59390_body11_e92544 * (locals.var_fsl2_dn7 + locals.var_fsl2_dn7))) / (assign59390_body11_e92547 * assign59390_body11_e92547)), (((((((((locals.var_beta_dn8 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn8)) * 2.0) * locals.var_fb) + (assign59390_body11_e92540 * locals.var_fb_dn8)) + locals.var_fsl1_dpsl_dn8) * assign59390_body11_e92547) - (assign59390_body11_e92544 * (locals.var_fsl2_dn8 + locals.var_fsl2_dn8))) / (assign59390_body11_e92547 * assign59390_body11_e92547)), (((((((((locals.var_beta_dn9 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn9)) * 2.0) * locals.var_fb) + (assign59390_body11_e92540 * locals.var_fb_dn9)) + locals.var_fsl1_dpsl_dn9) * assign59390_body11_e92547) - (assign59390_body11_e92544 * (locals.var_fsl2_dn9 + locals.var_fsl2_dn9))) / (assign59390_body11_e92547 * assign59390_body11_e92547)), (((((((((locals.var_beta_dn10 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn10)) * 2.0) * locals.var_fb) + (assign59390_body11_e92540 * locals.var_fb_dn10)) + locals.var_fsl1_dpsl_dn10) * assign59390_body11_e92547) - (assign59390_body11_e92544 * (locals.var_fsl2_dn10 + locals.var_fsl2_dn10))) / (assign59390_body11_e92547 * assign59390_body11_e92547)), (((((((((locals.var_beta_dn11 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn11)) * 2.0) * locals.var_fb) + (assign59390_body11_e92540 * locals.var_fb_dn11)) + locals.var_fsl1_dpsl_dn11) * assign59390_body11_e92547) - (assign59390_body11_e92544 * (locals.var_fsl2_dn11 + locals.var_fsl2_dn11))) / (assign59390_body11_e92547 * assign59390_body11_e92547)), (((((((((locals.var_beta_dn14 * locals.var_fb_dchi) + (locals.var_beta * locals.var_fb_dchi_dn14)) * 2.0) * locals.var_fb) + (assign59390_body11_e92540 * locals.var_fb_dn14)) + locals.var_fsl1_dpsl_dn14) * assign59390_body11_e92547) - (assign59390_body11_e92544 * (locals.var_fsl2_dn14 + locals.var_fsl2_dn14))) / (assign59390_body11_e92547 * assign59390_body11_e92547)),)
    } else {
        (locals.var_fsl2_dpsl, locals.var_fsl2_dpsl_dn0, locals.var_fsl2_dpsl_dn2, locals.var_fsl2_dpsl_dn4, locals.var_fsl2_dpsl_dn5, locals.var_fsl2_dpsl_dn6, locals.var_fsl2_dpsl_dn7, locals.var_fsl2_dpsl_dn8, locals.var_fsl2_dpsl_dn9, locals.var_fsl2_dpsl_dn10, locals.var_fsl2_dpsl_dn11, locals.var_fsl2_dpsl_dn14,)
    }
};
            locals.var_fsl2_dpsl = assign59390_body11_e92550;
            locals.var_fsl2_dpsl_dn0 = assign59390_body11_e92550_d_n0;
            locals.var_fsl2_dpsl_dn2 = assign59390_body11_e92550_d_n2;
            locals.var_fsl2_dpsl_dn4 = assign59390_body11_e92550_d_n4;
            locals.var_fsl2_dpsl_dn5 = assign59390_body11_e92550_d_n5;
            locals.var_fsl2_dpsl_dn6 = assign59390_body11_e92550_d_n6;
            locals.var_fsl2_dpsl_dn7 = assign59390_body11_e92550_d_n7;
            locals.var_fsl2_dpsl_dn8 = assign59390_body11_e92550_d_n8;
            locals.var_fsl2_dpsl_dn9 = assign59390_body11_e92550_d_n9;
            locals.var_fsl2_dpsl_dn10 = assign59390_body11_e92550_d_n10;
            locals.var_fsl2_dpsl_dn11 = assign59390_body11_e92550_d_n11;
            locals.var_fsl2_dpsl_dn14 = assign59390_body11_e92550_d_n14;
            locals.var_fsl2_dpsl_rv = 0.0;
            let (assign59390_body12_e92564, assign59390_body12_e92564_d_n0, assign59390_body12_e92564_d_n2, assign59390_body12_e92564_d_n4, assign59390_body12_e92564_d_n5, assign59390_body12_e92564_d_n6, assign59390_body12_e92564_d_n7, assign59390_body12_e92564_d_n8, assign59390_body12_e92564_d_n9, assign59390_body12_e92564_d_n10, assign59390_body12_e92564_d_n11, assign59390_body12_e92564_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1457 == 0.0)) {
        let assign59390_body12_e92561: f64 = (locals.var_psl - locals.var_vds);
        let assign59390_body12_e92562: f64 = (locals.var_beta * assign59390_body12_e92561);
        (assign59390_body12_e92562, ((locals.var_beta_dn0 * assign59390_body12_e92561) + (locals.var_beta * (locals.var_psl_dn0 - locals.var_vds_dn0))), ((locals.var_beta_dn2 * assign59390_body12_e92561) + (locals.var_beta * (locals.var_psl_dn2 - locals.var_vds_dn2))), ((locals.var_beta_dn4 * assign59390_body12_e92561) + (locals.var_beta * (locals.var_psl_dn4 - locals.var_vds_dn4))), ((locals.var_beta_dn5 * assign59390_body12_e92561) + (locals.var_beta * (locals.var_psl_dn5 - locals.var_vds_dn5))), ((locals.var_beta_dn6 * assign59390_body12_e92561) + (locals.var_beta * (locals.var_psl_dn6 - locals.var_vds_dn6))), ((locals.var_beta_dn7 * assign59390_body12_e92561) + (locals.var_beta * (locals.var_psl_dn7 - locals.var_vds_dn7))), ((locals.var_beta_dn8 * assign59390_body12_e92561) + (locals.var_beta * (locals.var_psl_dn8 - locals.var_vds_dn8))), ((locals.var_beta_dn9 * assign59390_body12_e92561) + (locals.var_beta * (locals.var_psl_dn9 - locals.var_vds_dn9))), ((locals.var_beta_dn10 * assign59390_body12_e92561) + (locals.var_beta * (locals.var_psl_dn10 - locals.var_vds_dn10))), ((locals.var_beta_dn11 * assign59390_body12_e92561) + (locals.var_beta * (locals.var_psl_dn11 - locals.var_vds_dn11))), ((locals.var_beta_dn14 * assign59390_body12_e92561) + (locals.var_beta * (locals.var_psl_dn14 - locals.var_vds_dn14))),)
    } else {
        (locals.var_rho, locals.var_rho_dn0, locals.var_rho_dn2, locals.var_rho_dn4, locals.var_rho_dn5, locals.var_rho_dn6, locals.var_rho_dn7, locals.var_rho_dn8, locals.var_rho_dn9, locals.var_rho_dn10, locals.var_rho_dn11, locals.var_rho_dn14,)
    }
};
            locals.var_rho = assign59390_body12_e92564;
            locals.var_rho_dn0 = assign59390_body12_e92564_d_n0;
            locals.var_rho_dn2 = assign59390_body12_e92564_d_n2;
            locals.var_rho_dn4 = assign59390_body12_e92564_d_n4;
            locals.var_rho_dn5 = assign59390_body12_e92564_d_n5;
            locals.var_rho_dn6 = assign59390_body12_e92564_d_n6;
            locals.var_rho_dn7 = assign59390_body12_e92564_d_n7;
            locals.var_rho_dn8 = assign59390_body12_e92564_d_n8;
            locals.var_rho_dn9 = assign59390_body12_e92564_d_n9;
            locals.var_rho_dn10 = assign59390_body12_e92564_d_n10;
            locals.var_rho_dn11 = assign59390_body12_e92564_d_n11;
            locals.var_rho_dn14 = assign59390_body12_e92564_d_n14;
            locals.var_rho_rv = 0.0;
            let (assign59390_body13_e92575, assign59390_body13_e92575_d_n0, assign59390_body13_e92575_d_n2, assign59390_body13_e92575_d_n4, assign59390_body13_e92575_d_n5, assign59390_body13_e92575_d_n6, assign59390_body13_e92575_d_n7, assign59390_body13_e92575_d_n8, assign59390_body13_e92575_d_n9, assign59390_body13_e92575_d_n10, assign59390_body13_e92575_d_n11, assign59390_body13_e92575_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1457 == 0.0)) {
        let assign59390_body13_e92573: f64 = (locals.var_rho).exp();
        (assign59390_body13_e92573, (assign59390_body13_e92573 * locals.var_rho_dn0), (assign59390_body13_e92573 * locals.var_rho_dn2), (assign59390_body13_e92573 * locals.var_rho_dn4), (assign59390_body13_e92573 * locals.var_rho_dn5), (assign59390_body13_e92573 * locals.var_rho_dn6), (assign59390_body13_e92573 * locals.var_rho_dn7), (assign59390_body13_e92573 * locals.var_rho_dn8), (assign59390_body13_e92573 * locals.var_rho_dn9), (assign59390_body13_e92573 * locals.var_rho_dn10), (assign59390_body13_e92573 * locals.var_rho_dn11), (assign59390_body13_e92573 * locals.var_rho_dn14),)
    } else {
        (locals.var_exp_rho, locals.var_exp_rho_dn0, locals.var_exp_rho_dn2, locals.var_exp_rho_dn4, locals.var_exp_rho_dn5, locals.var_exp_rho_dn6, locals.var_exp_rho_dn7, locals.var_exp_rho_dn8, locals.var_exp_rho_dn9, locals.var_exp_rho_dn10, locals.var_exp_rho_dn11, locals.var_exp_rho_dn14,)
    }
};
            locals.var_exp_rho = assign59390_body13_e92575;
            locals.var_exp_rho_dn0 = assign59390_body13_e92575_d_n0;
            locals.var_exp_rho_dn2 = assign59390_body13_e92575_d_n2;
            locals.var_exp_rho_dn4 = assign59390_body13_e92575_d_n4;
            locals.var_exp_rho_dn5 = assign59390_body13_e92575_d_n5;
            locals.var_exp_rho_dn6 = assign59390_body13_e92575_d_n6;
            locals.var_exp_rho_dn7 = assign59390_body13_e92575_d_n7;
            locals.var_exp_rho_dn8 = assign59390_body13_e92575_d_n8;
            locals.var_exp_rho_dn9 = assign59390_body13_e92575_d_n9;
            locals.var_exp_rho_dn10 = assign59390_body13_e92575_d_n10;
            locals.var_exp_rho_dn11 = assign59390_body13_e92575_d_n11;
            locals.var_exp_rho_dn14 = assign59390_body13_e92575_d_n14;
            locals.var_exp_rho_rv = 0.0;
            let (assign59390_body14_e92589, assign59390_body14_e92589_d_n0, assign59390_body14_e92589_d_n2, assign59390_body14_e92589_d_n4, assign59390_body14_e92589_d_n5, assign59390_body14_e92589_d_n6, assign59390_body14_e92589_d_n7, assign59390_body14_e92589_d_n8, assign59390_body14_e92589_d_n9, assign59390_body14_e92589_d_n10, assign59390_body14_e92589_d_n11, assign59390_body14_e92589_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1457 == 0.0)) {
        let assign59390_body14_e92586: f64 = (locals.var_exp_rho - locals.var_exp_bvbsvds);
        let assign59390_body14_e92587: f64 = (locals.var_cnst1 * assign59390_body14_e92586);
        (assign59390_body14_e92587, ((locals.var_cnst1_dn0 * assign59390_body14_e92586) + (locals.var_cnst1 * (locals.var_exp_rho_dn0 - locals.var_exp_bvbsvds_dn0))), ((locals.var_cnst1_dn2 * assign59390_body14_e92586) + (locals.var_cnst1 * (locals.var_exp_rho_dn2 - locals.var_exp_bvbsvds_dn2))), ((locals.var_cnst1_dn4 * assign59390_body14_e92586) + (locals.var_cnst1 * (locals.var_exp_rho_dn4 - locals.var_exp_bvbsvds_dn4))), ((locals.var_cnst1_dn5 * assign59390_body14_e92586) + (locals.var_cnst1 * (locals.var_exp_rho_dn5 - locals.var_exp_bvbsvds_dn5))), ((locals.var_cnst1_dn6 * assign59390_body14_e92586) + (locals.var_cnst1 * (locals.var_exp_rho_dn6 - locals.var_exp_bvbsvds_dn6))), ((locals.var_cnst1_dn7 * assign59390_body14_e92586) + (locals.var_cnst1 * (locals.var_exp_rho_dn7 - locals.var_exp_bvbsvds_dn7))), ((locals.var_cnst1_dn8 * assign59390_body14_e92586) + (locals.var_cnst1 * (locals.var_exp_rho_dn8 - locals.var_exp_bvbsvds_dn8))), ((locals.var_cnst1_dn9 * assign59390_body14_e92586) + (locals.var_cnst1 * (locals.var_exp_rho_dn9 - locals.var_exp_bvbsvds_dn9))), ((locals.var_cnst1_dn10 * assign59390_body14_e92586) + (locals.var_cnst1 * (locals.var_exp_rho_dn10 - locals.var_exp_bvbsvds_dn10))), ((locals.var_cnst1_dn11 * assign59390_body14_e92586) + (locals.var_cnst1 * (locals.var_exp_rho_dn11 - locals.var_exp_bvbsvds_dn11))), ((locals.var_cnst1_dn14 * assign59390_body14_e92586) + (locals.var_cnst1 * (locals.var_exp_rho_dn14 - locals.var_exp_bvbsvds_dn14))),)
    } else {
        (locals.var_fsl1, locals.var_fsl1_dn0, locals.var_fsl1_dn2, locals.var_fsl1_dn4, locals.var_fsl1_dn5, locals.var_fsl1_dn6, locals.var_fsl1_dn7, locals.var_fsl1_dn8, locals.var_fsl1_dn9, locals.var_fsl1_dn10, locals.var_fsl1_dn11, locals.var_fsl1_dn14,)
    }
};
            locals.var_fsl1 = assign59390_body14_e92589;
            locals.var_fsl1_dn0 = assign59390_body14_e92589_d_n0;
            locals.var_fsl1_dn2 = assign59390_body14_e92589_d_n2;
            locals.var_fsl1_dn4 = assign59390_body14_e92589_d_n4;
            locals.var_fsl1_dn5 = assign59390_body14_e92589_d_n5;
            locals.var_fsl1_dn6 = assign59390_body14_e92589_d_n6;
            locals.var_fsl1_dn7 = assign59390_body14_e92589_d_n7;
            locals.var_fsl1_dn8 = assign59390_body14_e92589_d_n8;
            locals.var_fsl1_dn9 = assign59390_body14_e92589_d_n9;
            locals.var_fsl1_dn10 = assign59390_body14_e92589_d_n10;
            locals.var_fsl1_dn11 = assign59390_body14_e92589_d_n11;
            locals.var_fsl1_dn14 = assign59390_body14_e92589_d_n14;
            locals.var_fsl1_rv = 0.0;
            let (assign59390_body15_e92603, assign59390_body15_e92603_d_n0, assign59390_body15_e92603_d_n2, assign59390_body15_e92603_d_n4, assign59390_body15_e92603_d_n5, assign59390_body15_e92603_d_n6, assign59390_body15_e92603_d_n7, assign59390_body15_e92603_d_n8, assign59390_body15_e92603_d_n9, assign59390_body15_e92603_d_n10, assign59390_body15_e92603_d_n11, assign59390_body15_e92603_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1457 == 0.0)) {
        let assign59390_body15_e92599: f64 = (locals.var_cnst1 * locals.var_beta);
        let assign59390_body15_e92601: f64 = (assign59390_body15_e92599 * locals.var_exp_rho);
        (assign59390_body15_e92601, ((((locals.var_cnst1_dn0 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn0)) * locals.var_exp_rho) + (assign59390_body15_e92599 * locals.var_exp_rho_dn0)), ((((locals.var_cnst1_dn2 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn2)) * locals.var_exp_rho) + (assign59390_body15_e92599 * locals.var_exp_rho_dn2)), ((((locals.var_cnst1_dn4 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn4)) * locals.var_exp_rho) + (assign59390_body15_e92599 * locals.var_exp_rho_dn4)), ((((locals.var_cnst1_dn5 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn5)) * locals.var_exp_rho) + (assign59390_body15_e92599 * locals.var_exp_rho_dn5)), ((((locals.var_cnst1_dn6 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn6)) * locals.var_exp_rho) + (assign59390_body15_e92599 * locals.var_exp_rho_dn6)), ((((locals.var_cnst1_dn7 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn7)) * locals.var_exp_rho) + (assign59390_body15_e92599 * locals.var_exp_rho_dn7)), ((((locals.var_cnst1_dn8 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn8)) * locals.var_exp_rho) + (assign59390_body15_e92599 * locals.var_exp_rho_dn8)), ((((locals.var_cnst1_dn9 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn9)) * locals.var_exp_rho) + (assign59390_body15_e92599 * locals.var_exp_rho_dn9)), ((((locals.var_cnst1_dn10 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn10)) * locals.var_exp_rho) + (assign59390_body15_e92599 * locals.var_exp_rho_dn10)), ((((locals.var_cnst1_dn11 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn11)) * locals.var_exp_rho) + (assign59390_body15_e92599 * locals.var_exp_rho_dn11)), ((((locals.var_cnst1_dn14 * locals.var_beta) + (locals.var_cnst1 * locals.var_beta_dn14)) * locals.var_exp_rho) + (assign59390_body15_e92599 * locals.var_exp_rho_dn14)),)
    } else {
        (locals.var_fsl1_dpsl, locals.var_fsl1_dpsl_dn0, locals.var_fsl1_dpsl_dn2, locals.var_fsl1_dpsl_dn4, locals.var_fsl1_dpsl_dn5, locals.var_fsl1_dpsl_dn6, locals.var_fsl1_dpsl_dn7, locals.var_fsl1_dpsl_dn8, locals.var_fsl1_dpsl_dn9, locals.var_fsl1_dpsl_dn10, locals.var_fsl1_dpsl_dn11, locals.var_fsl1_dpsl_dn14,)
    }
};
            locals.var_fsl1_dpsl = assign59390_body15_e92603;
            locals.var_fsl1_dpsl_dn0 = assign59390_body15_e92603_d_n0;
            locals.var_fsl1_dpsl_dn2 = assign59390_body15_e92603_d_n2;
            locals.var_fsl1_dpsl_dn4 = assign59390_body15_e92603_d_n4;
            locals.var_fsl1_dpsl_dn5 = assign59390_body15_e92603_d_n5;
            locals.var_fsl1_dpsl_dn6 = assign59390_body15_e92603_d_n6;
            locals.var_fsl1_dpsl_dn7 = assign59390_body15_e92603_d_n7;
            locals.var_fsl1_dpsl_dn8 = assign59390_body15_e92603_d_n8;
            locals.var_fsl1_dpsl_dn9 = assign59390_body15_e92603_d_n9;
            locals.var_fsl1_dpsl_dn10 = assign59390_body15_e92603_d_n10;
            locals.var_fsl1_dpsl_dn11 = assign59390_body15_e92603_d_n11;
            locals.var_fsl1_dpsl_dn14 = assign59390_body15_e92603_d_n14;
            locals.var_fsl1_dpsl_rv = 0.0;
            let (assign59390_body16_e92615, assign59390_body16_e92615_d_n0, assign59390_body16_e92615_d_n2, assign59390_body16_e92615_d_n4, assign59390_body16_e92615_d_n5, assign59390_body16_e92615_d_n6, assign59390_body16_e92615_d_n7, assign59390_body16_e92615_d_n8, assign59390_body16_e92615_d_n9, assign59390_body16_e92615_d_n10, assign59390_body16_e92615_d_n11, assign59390_body16_e92615_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1457 == 0.0)) {
        let assign59390_body16_e92613: f64 = (locals.var_chi - 1.0);
        (assign59390_body16_e92613, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    } else {
        (locals.var_xil, locals.var_xil_dn0, locals.var_xil_dn2, locals.var_xil_dn4, locals.var_xil_dn5, locals.var_xil_dn6, locals.var_xil_dn7, locals.var_xil_dn8, locals.var_xil_dn9, locals.var_xil_dn10, locals.var_xil_dn11, locals.var_xil_dn14,)
    }
};
            locals.var_xil = assign59390_body16_e92615;
            locals.var_xil_dn0 = assign59390_body16_e92615_d_n0;
            locals.var_xil_dn2 = assign59390_body16_e92615_d_n2;
            locals.var_xil_dn4 = assign59390_body16_e92615_d_n4;
            locals.var_xil_dn5 = assign59390_body16_e92615_d_n5;
            locals.var_xil_dn6 = assign59390_body16_e92615_d_n6;
            locals.var_xil_dn7 = assign59390_body16_e92615_d_n7;
            locals.var_xil_dn8 = assign59390_body16_e92615_d_n8;
            locals.var_xil_dn9 = assign59390_body16_e92615_d_n9;
            locals.var_xil_dn10 = assign59390_body16_e92615_d_n10;
            locals.var_xil_dn11 = assign59390_body16_e92615_d_n11;
            locals.var_xil_dn14 = assign59390_body16_e92615_d_n14;
            locals.var_xil_rv = 0.0;
            let (assign59390_body17_e92628, assign59390_body17_e92628_d_n0, assign59390_body17_e92628_d_n2, assign59390_body17_e92628_d_n4, assign59390_body17_e92628_d_n5, assign59390_body17_e92628_d_n6, assign59390_body17_e92628_d_n7, assign59390_body17_e92628_d_n8, assign59390_body17_e92628_d_n9, assign59390_body17_e92628_d_n10, assign59390_body17_e92628_d_n11, assign59390_body17_e92628_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1457 == 0.0)) {
        let assign59390_body17_e92625: f64 = (locals.var_xil + locals.var_fsl1);
        let assign59390_body17_e92626: f64 = (assign59390_body17_e92625).sqrt();
        (assign59390_body17_e92626, ((locals.var_xil_dn0 + locals.var_fsl1_dn0) / (2.0 * assign59390_body17_e92626)), ((locals.var_xil_dn2 + locals.var_fsl1_dn2) / (2.0 * assign59390_body17_e92626)), ((locals.var_xil_dn4 + locals.var_fsl1_dn4) / (2.0 * assign59390_body17_e92626)), ((locals.var_xil_dn5 + locals.var_fsl1_dn5) / (2.0 * assign59390_body17_e92626)), ((locals.var_xil_dn6 + locals.var_fsl1_dn6) / (2.0 * assign59390_body17_e92626)), ((locals.var_xil_dn7 + locals.var_fsl1_dn7) / (2.0 * assign59390_body17_e92626)), ((locals.var_xil_dn8 + locals.var_fsl1_dn8) / (2.0 * assign59390_body17_e92626)), ((locals.var_xil_dn9 + locals.var_fsl1_dn9) / (2.0 * assign59390_body17_e92626)), ((locals.var_xil_dn10 + locals.var_fsl1_dn10) / (2.0 * assign59390_body17_e92626)), ((locals.var_xil_dn11 + locals.var_fsl1_dn11) / (2.0 * assign59390_body17_e92626)), ((locals.var_xil_dn14 + locals.var_fsl1_dn14) / (2.0 * assign59390_body17_e92626)),)
    } else {
        (locals.var_fsl2, locals.var_fsl2_dn0, locals.var_fsl2_dn2, locals.var_fsl2_dn4, locals.var_fsl2_dn5, locals.var_fsl2_dn6, locals.var_fsl2_dn7, locals.var_fsl2_dn8, locals.var_fsl2_dn9, locals.var_fsl2_dn10, locals.var_fsl2_dn11, locals.var_fsl2_dn14,)
    }
};
            locals.var_fsl2 = assign59390_body17_e92628;
            locals.var_fsl2_dn0 = assign59390_body17_e92628_d_n0;
            locals.var_fsl2_dn2 = assign59390_body17_e92628_d_n2;
            locals.var_fsl2_dn4 = assign59390_body17_e92628_d_n4;
            locals.var_fsl2_dn5 = assign59390_body17_e92628_d_n5;
            locals.var_fsl2_dn6 = assign59390_body17_e92628_d_n6;
            locals.var_fsl2_dn7 = assign59390_body17_e92628_d_n7;
            locals.var_fsl2_dn8 = assign59390_body17_e92628_d_n8;
            locals.var_fsl2_dn9 = assign59390_body17_e92628_d_n9;
            locals.var_fsl2_dn10 = assign59390_body17_e92628_d_n10;
            locals.var_fsl2_dn11 = assign59390_body17_e92628_d_n11;
            locals.var_fsl2_dn14 = assign59390_body17_e92628_d_n14;
            locals.var_fsl2_rv = 0.0;
            let (assign59390_body18_e92644, assign59390_body18_e92644_d_n0, assign59390_body18_e92644_d_n2, assign59390_body18_e92644_d_n4, assign59390_body18_e92644_d_n5, assign59390_body18_e92644_d_n6, assign59390_body18_e92644_d_n7, assign59390_body18_e92644_d_n8, assign59390_body18_e92644_d_n9, assign59390_body18_e92644_d_n10, assign59390_body18_e92644_d_n11, assign59390_body18_e92644_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1457 == 0.0)) {
        let assign59390_body18_e92638: f64 = (locals.var_beta + locals.var_fsl1_dpsl);
        let assign59390_body18_e92641: f64 = (locals.var_fsl2 + locals.var_fsl2);
        let assign59390_body18_e92642: f64 = (assign59390_body18_e92638 / assign59390_body18_e92641);
        (assign59390_body18_e92642, ((((locals.var_beta_dn0 + locals.var_fsl1_dpsl_dn0) * assign59390_body18_e92641) - (assign59390_body18_e92638 * (locals.var_fsl2_dn0 + locals.var_fsl2_dn0))) / (assign59390_body18_e92641 * assign59390_body18_e92641)), ((((locals.var_beta_dn2 + locals.var_fsl1_dpsl_dn2) * assign59390_body18_e92641) - (assign59390_body18_e92638 * (locals.var_fsl2_dn2 + locals.var_fsl2_dn2))) / (assign59390_body18_e92641 * assign59390_body18_e92641)), ((((locals.var_beta_dn4 + locals.var_fsl1_dpsl_dn4) * assign59390_body18_e92641) - (assign59390_body18_e92638 * (locals.var_fsl2_dn4 + locals.var_fsl2_dn4))) / (assign59390_body18_e92641 * assign59390_body18_e92641)), ((((locals.var_beta_dn5 + locals.var_fsl1_dpsl_dn5) * assign59390_body18_e92641) - (assign59390_body18_e92638 * (locals.var_fsl2_dn5 + locals.var_fsl2_dn5))) / (assign59390_body18_e92641 * assign59390_body18_e92641)), ((((locals.var_beta_dn6 + locals.var_fsl1_dpsl_dn6) * assign59390_body18_e92641) - (assign59390_body18_e92638 * (locals.var_fsl2_dn6 + locals.var_fsl2_dn6))) / (assign59390_body18_e92641 * assign59390_body18_e92641)), ((((locals.var_beta_dn7 + locals.var_fsl1_dpsl_dn7) * assign59390_body18_e92641) - (assign59390_body18_e92638 * (locals.var_fsl2_dn7 + locals.var_fsl2_dn7))) / (assign59390_body18_e92641 * assign59390_body18_e92641)), ((((locals.var_beta_dn8 + locals.var_fsl1_dpsl_dn8) * assign59390_body18_e92641) - (assign59390_body18_e92638 * (locals.var_fsl2_dn8 + locals.var_fsl2_dn8))) / (assign59390_body18_e92641 * assign59390_body18_e92641)), ((((locals.var_beta_dn9 + locals.var_fsl1_dpsl_dn9) * assign59390_body18_e92641) - (assign59390_body18_e92638 * (locals.var_fsl2_dn9 + locals.var_fsl2_dn9))) / (assign59390_body18_e92641 * assign59390_body18_e92641)), ((((locals.var_beta_dn10 + locals.var_fsl1_dpsl_dn10) * assign59390_body18_e92641) - (assign59390_body18_e92638 * (locals.var_fsl2_dn10 + locals.var_fsl2_dn10))) / (assign59390_body18_e92641 * assign59390_body18_e92641)), ((((locals.var_beta_dn11 + locals.var_fsl1_dpsl_dn11) * assign59390_body18_e92641) - (assign59390_body18_e92638 * (locals.var_fsl2_dn11 + locals.var_fsl2_dn11))) / (assign59390_body18_e92641 * assign59390_body18_e92641)), ((((locals.var_beta_dn14 + locals.var_fsl1_dpsl_dn14) * assign59390_body18_e92641) - (assign59390_body18_e92638 * (locals.var_fsl2_dn14 + locals.var_fsl2_dn14))) / (assign59390_body18_e92641 * assign59390_body18_e92641)),)
    } else {
        (locals.var_fsl2_dpsl, locals.var_fsl2_dpsl_dn0, locals.var_fsl2_dpsl_dn2, locals.var_fsl2_dpsl_dn4, locals.var_fsl2_dpsl_dn5, locals.var_fsl2_dpsl_dn6, locals.var_fsl2_dpsl_dn7, locals.var_fsl2_dpsl_dn8, locals.var_fsl2_dpsl_dn9, locals.var_fsl2_dpsl_dn10, locals.var_fsl2_dpsl_dn11, locals.var_fsl2_dpsl_dn14,)
    }
};
            locals.var_fsl2_dpsl = assign59390_body18_e92644;
            locals.var_fsl2_dpsl_dn0 = assign59390_body18_e92644_d_n0;
            locals.var_fsl2_dpsl_dn2 = assign59390_body18_e92644_d_n2;
            locals.var_fsl2_dpsl_dn4 = assign59390_body18_e92644_d_n4;
            locals.var_fsl2_dpsl_dn5 = assign59390_body18_e92644_d_n5;
            locals.var_fsl2_dpsl_dn6 = assign59390_body18_e92644_d_n6;
            locals.var_fsl2_dpsl_dn7 = assign59390_body18_e92644_d_n7;
            locals.var_fsl2_dpsl_dn8 = assign59390_body18_e92644_d_n8;
            locals.var_fsl2_dpsl_dn9 = assign59390_body18_e92644_d_n9;
            locals.var_fsl2_dpsl_dn10 = assign59390_body18_e92644_d_n10;
            locals.var_fsl2_dpsl_dn11 = assign59390_body18_e92644_d_n11;
            locals.var_fsl2_dpsl_dn14 = assign59390_body18_e92644_d_n14;
            locals.var_fsl2_dpsl_rv = 0.0;
            let (assign59390_body19_e92657, assign59390_body19_e92657_d_n0, assign59390_body19_e92657_d_n2, assign59390_body19_e92657_d_n4, assign59390_body19_e92657_d_n5, assign59390_body19_e92657_d_n6, assign59390_body19_e92657_d_n7, assign59390_body19_e92657_d_n8, assign59390_body19_e92657_d_n9, assign59390_body19_e92657_d_n10, assign59390_body19_e92657_d_n11, assign59390_body19_e92657_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign59390_body19_e92651: f64 = (locals.var_vgp - locals.var_psl);
        let assign59390_body19_e92654: f64 = (locals.var_fac1 * locals.var_fsl2);
        let assign59390_body19_e92655: f64 = (assign59390_body19_e92651 - assign59390_body19_e92654);
        (assign59390_body19_e92655, ((locals.var_vgp_dn0 - locals.var_psl_dn0) - ((locals.var_fac1_dn0 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn0))), ((locals.var_vgp_dn2 - locals.var_psl_dn2) - ((locals.var_fac1_dn2 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn2))), ((locals.var_vgp_dn4 - locals.var_psl_dn4) - ((locals.var_fac1_dn4 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn4))), ((locals.var_vgp_dn5 - locals.var_psl_dn5) - ((locals.var_fac1_dn5 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn5))), ((locals.var_vgp_dn6 - locals.var_psl_dn6) - ((locals.var_fac1_dn6 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn6))), ((locals.var_vgp_dn7 - locals.var_psl_dn7) - ((locals.var_fac1_dn7 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn7))), ((locals.var_vgp_dn8 - locals.var_psl_dn8) - ((locals.var_fac1_dn8 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn8))), ((locals.var_vgp_dn9 - locals.var_psl_dn9) - ((locals.var_fac1_dn9 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn9))), ((locals.var_vgp_dn10 - locals.var_psl_dn10) - ((locals.var_fac1_dn10 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn10))), ((locals.var_vgp_dn11 - locals.var_psl_dn11) - ((locals.var_fac1_dn11 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn11))), ((locals.var_vgp_dn14 - locals.var_psl_dn14) - ((locals.var_fac1_dn14 * locals.var_fsl2) + (locals.var_fac1 * locals.var_fsl2_dn14))),)
    } else {
        (locals.var_fsl, locals.var_fsl_dn0, locals.var_fsl_dn2, locals.var_fsl_dn4, locals.var_fsl_dn5, locals.var_fsl_dn6, locals.var_fsl_dn7, locals.var_fsl_dn8, locals.var_fsl_dn9, locals.var_fsl_dn10, locals.var_fsl_dn11, locals.var_fsl_dn14,)
    }
};
            locals.var_fsl = assign59390_body19_e92657;
            locals.var_fsl_dn0 = assign59390_body19_e92657_d_n0;
            locals.var_fsl_dn2 = assign59390_body19_e92657_d_n2;
            locals.var_fsl_dn4 = assign59390_body19_e92657_d_n4;
            locals.var_fsl_dn5 = assign59390_body19_e92657_d_n5;
            locals.var_fsl_dn6 = assign59390_body19_e92657_d_n6;
            locals.var_fsl_dn7 = assign59390_body19_e92657_d_n7;
            locals.var_fsl_dn8 = assign59390_body19_e92657_d_n8;
            locals.var_fsl_dn9 = assign59390_body19_e92657_d_n9;
            locals.var_fsl_dn10 = assign59390_body19_e92657_d_n10;
            locals.var_fsl_dn11 = assign59390_body19_e92657_d_n11;
            locals.var_fsl_dn14 = assign59390_body19_e92657_d_n14;
            locals.var_fsl_rv = 0.0;
            let (assign59390_body20_e92669, assign59390_body20_e92669_d_n0, assign59390_body20_e92669_d_n2, assign59390_body20_e92669_d_n4, assign59390_body20_e92669_d_n5, assign59390_body20_e92669_d_n6, assign59390_body20_e92669_d_n7, assign59390_body20_e92669_d_n8, assign59390_body20_e92669_d_n9, assign59390_body20_e92669_d_n10, assign59390_body20_e92669_d_n11, assign59390_body20_e92669_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign59390_body20_e92663: f64 = (-1.0);
        let assign59390_body20_e92666: f64 = (locals.var_fac1 * locals.var_fsl2_dpsl);
        let assign59390_body20_e92667: f64 = (assign59390_body20_e92663 - assign59390_body20_e92666);
        (assign59390_body20_e92667, (-((locals.var_fac1_dn0 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn0))), (-((locals.var_fac1_dn2 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn2))), (-((locals.var_fac1_dn4 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn4))), (-((locals.var_fac1_dn5 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn5))), (-((locals.var_fac1_dn6 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn6))), (-((locals.var_fac1_dn7 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn7))), (-((locals.var_fac1_dn8 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn8))), (-((locals.var_fac1_dn9 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn9))), (-((locals.var_fac1_dn10 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn10))), (-((locals.var_fac1_dn11 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn11))), (-((locals.var_fac1_dn14 * locals.var_fsl2_dpsl) + (locals.var_fac1 * locals.var_fsl2_dpsl_dn14))),)
    } else {
        (locals.var_fsl_dpsl, locals.var_fsl_dpsl_dn0, locals.var_fsl_dpsl_dn2, locals.var_fsl_dpsl_dn4, locals.var_fsl_dpsl_dn5, locals.var_fsl_dpsl_dn6, locals.var_fsl_dpsl_dn7, locals.var_fsl_dpsl_dn8, locals.var_fsl_dpsl_dn9, locals.var_fsl_dpsl_dn10, locals.var_fsl_dpsl_dn11, locals.var_fsl_dpsl_dn14,)
    }
};
            locals.var_fsl_dpsl = assign59390_body20_e92669;
            locals.var_fsl_dpsl_dn0 = assign59390_body20_e92669_d_n0;
            locals.var_fsl_dpsl_dn2 = assign59390_body20_e92669_d_n2;
            locals.var_fsl_dpsl_dn4 = assign59390_body20_e92669_d_n4;
            locals.var_fsl_dpsl_dn5 = assign59390_body20_e92669_d_n5;
            locals.var_fsl_dpsl_dn6 = assign59390_body20_e92669_d_n6;
            locals.var_fsl_dpsl_dn7 = assign59390_body20_e92669_d_n7;
            locals.var_fsl_dpsl_dn8 = assign59390_body20_e92669_d_n8;
            locals.var_fsl_dpsl_dn9 = assign59390_body20_e92669_d_n9;
            locals.var_fsl_dpsl_dn10 = assign59390_body20_e92669_d_n10;
            locals.var_fsl_dpsl_dn11 = assign59390_body20_e92669_d_n11;
            locals.var_fsl_dpsl_dn14 = assign59390_body20_e92669_d_n14;
            locals.var_fsl_dpsl_rv = 0.0;
            let assign59390_body21_e92672: f64 = if locals.var_flg_conv == 1.0 { 1.0 } else { 0.0 };
            locals.var_guard1458 = assign59390_body21_e92672;
            locals.var_guard1458_rv = 0.0;
            let (assign59390_body22_e92681,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1458 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_brk2,)
    }
};
            locals.var_flg_brk2 = assign59390_body22_e92681;
            locals.var_flg_brk2_rv = 0.0;
            let assign59390_body23_e92684: f64 = if locals.var_flg_brk2 == 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1459 = assign59390_body23_e92684;
            locals.var_guard1459_rv = 0.0;
            let (assign59390_body24_e92696, assign59390_body24_e92696_d_n0, assign59390_body24_e92696_d_n2, assign59390_body24_e92696_d_n4, assign59390_body24_e92696_d_n5, assign59390_body24_e92696_d_n6, assign59390_body24_e92696_d_n7, assign59390_body24_e92696_d_n8, assign59390_body24_e92696_d_n9, assign59390_body24_e92696_d_n10, assign59390_body24_e92696_d_n11, assign59390_body24_e92696_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1459 != 0.0)) {
        let assign59390_body24_e92692: f64 = (-locals.var_fsl);
        let assign59390_body24_e92694: f64 = (assign59390_body24_e92692 / locals.var_fsl_dpsl);
        (assign59390_body24_e92694, ((((-locals.var_fsl_dn0) * locals.var_fsl_dpsl) - (assign59390_body24_e92692 * locals.var_fsl_dpsl_dn0)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn2) * locals.var_fsl_dpsl) - (assign59390_body24_e92692 * locals.var_fsl_dpsl_dn2)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn4) * locals.var_fsl_dpsl) - (assign59390_body24_e92692 * locals.var_fsl_dpsl_dn4)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn5) * locals.var_fsl_dpsl) - (assign59390_body24_e92692 * locals.var_fsl_dpsl_dn5)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn6) * locals.var_fsl_dpsl) - (assign59390_body24_e92692 * locals.var_fsl_dpsl_dn6)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn7) * locals.var_fsl_dpsl) - (assign59390_body24_e92692 * locals.var_fsl_dpsl_dn7)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn8) * locals.var_fsl_dpsl) - (assign59390_body24_e92692 * locals.var_fsl_dpsl_dn8)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn9) * locals.var_fsl_dpsl) - (assign59390_body24_e92692 * locals.var_fsl_dpsl_dn9)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn10) * locals.var_fsl_dpsl) - (assign59390_body24_e92692 * locals.var_fsl_dpsl_dn10)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn11) * locals.var_fsl_dpsl) - (assign59390_body24_e92692 * locals.var_fsl_dpsl_dn11)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)), ((((-locals.var_fsl_dn14) * locals.var_fsl_dpsl) - (assign59390_body24_e92692 * locals.var_fsl_dpsl_dn14)) / (locals.var_fsl_dpsl * locals.var_fsl_dpsl)),)
    } else {
        (locals.var_dpsl, locals.var_dpsl_dn0, locals.var_dpsl_dn2, locals.var_dpsl_dn4, locals.var_dpsl_dn5, locals.var_dpsl_dn6, locals.var_dpsl_dn7, locals.var_dpsl_dn8, locals.var_dpsl_dn9, locals.var_dpsl_dn10, locals.var_dpsl_dn11, locals.var_dpsl_dn14,)
    }
};
            locals.var_dpsl = assign59390_body24_e92696;
            locals.var_dpsl_dn0 = assign59390_body24_e92696_d_n0;
            locals.var_dpsl_dn2 = assign59390_body24_e92696_d_n2;
            locals.var_dpsl_dn4 = assign59390_body24_e92696_d_n4;
            locals.var_dpsl_dn5 = assign59390_body24_e92696_d_n5;
            locals.var_dpsl_dn6 = assign59390_body24_e92696_d_n6;
            locals.var_dpsl_dn7 = assign59390_body24_e92696_d_n7;
            locals.var_dpsl_dn8 = assign59390_body24_e92696_d_n8;
            locals.var_dpsl_dn9 = assign59390_body24_e92696_d_n9;
            locals.var_dpsl_dn10 = assign59390_body24_e92696_d_n10;
            locals.var_dpsl_dn11 = assign59390_body24_e92696_d_n11;
            locals.var_dpsl_dn14 = assign59390_body24_e92696_d_n14;
            locals.var_dpsl_rv = 0.0;
            let (assign59390_body25_e92718, assign59390_body25_e92718_d_n0, assign59390_body25_e92718_d_n2, assign59390_body25_e92718_d_n4, assign59390_body25_e92718_d_n5, assign59390_body25_e92718_d_n6, assign59390_body25_e92718_d_n7, assign59390_body25_e92718_d_n8, assign59390_body25_e92718_d_n9, assign59390_body25_e92718_d_n10, assign59390_body25_e92718_d_n11, assign59390_body25_e92718_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1459 != 0.0)) {
        let assign59390_body25_e92705: f64 = (0.5 * 0.1);
        let assign59390_body25_e92709: f64 = (locals.var_psl).abs();
        let (assign59390_body25_e92714, assign59390_body25_e92714_d_n0, assign59390_body25_e92714_d_n2, assign59390_body25_e92714_d_n4, assign59390_body25_e92714_d_n5, assign59390_body25_e92714_d_n6, assign59390_body25_e92714_d_n7, assign59390_body25_e92714_d_n8, assign59390_body25_e92714_d_n9, assign59390_body25_e92714_d_n10, assign59390_body25_e92714_d_n11, assign59390_body25_e92714_d_n14,) = {
            if (1.0 >= assign59390_body25_e92709) {
                (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign59390_body25_e92713: f64 = (locals.var_psl).abs();
                (assign59390_body25_e92713, if locals.var_psl >= 0.0 { locals.var_psl_dn0 } else { (-locals.var_psl_dn0) }, if locals.var_psl >= 0.0 { locals.var_psl_dn2 } else { (-locals.var_psl_dn2) }, if locals.var_psl >= 0.0 { locals.var_psl_dn4 } else { (-locals.var_psl_dn4) }, if locals.var_psl >= 0.0 { locals.var_psl_dn5 } else { (-locals.var_psl_dn5) }, if locals.var_psl >= 0.0 { locals.var_psl_dn6 } else { (-locals.var_psl_dn6) }, if locals.var_psl >= 0.0 { locals.var_psl_dn7 } else { (-locals.var_psl_dn7) }, if locals.var_psl >= 0.0 { locals.var_psl_dn8 } else { (-locals.var_psl_dn8) }, if locals.var_psl >= 0.0 { locals.var_psl_dn9 } else { (-locals.var_psl_dn9) }, if locals.var_psl >= 0.0 { locals.var_psl_dn10 } else { (-locals.var_psl_dn10) }, if locals.var_psl >= 0.0 { locals.var_psl_dn11 } else { (-locals.var_psl_dn11) }, if locals.var_psl >= 0.0 { locals.var_psl_dn14 } else { (-locals.var_psl_dn14) },)
            }
        };
        let assign59390_body25_e92715: f64 = (1.0 + assign59390_body25_e92714);
        let assign59390_body25_e92716: f64 = (assign59390_body25_e92705 * assign59390_body25_e92715);
        (assign59390_body25_e92716, (assign59390_body25_e92705 * assign59390_body25_e92714_d_n0), (assign59390_body25_e92705 * assign59390_body25_e92714_d_n2), (assign59390_body25_e92705 * assign59390_body25_e92714_d_n4), (assign59390_body25_e92705 * assign59390_body25_e92714_d_n5), (assign59390_body25_e92705 * assign59390_body25_e92714_d_n6), (assign59390_body25_e92705 * assign59390_body25_e92714_d_n7), (assign59390_body25_e92705 * assign59390_body25_e92714_d_n8), (assign59390_body25_e92705 * assign59390_body25_e92714_d_n9), (assign59390_body25_e92705 * assign59390_body25_e92714_d_n10), (assign59390_body25_e92705 * assign59390_body25_e92714_d_n11), (assign59390_body25_e92705 * assign59390_body25_e92714_d_n14),)
    } else {
        (locals.var_dplim, locals.var_dplim_dn0, locals.var_dplim_dn2, locals.var_dplim_dn4, locals.var_dplim_dn5, locals.var_dplim_dn6, locals.var_dplim_dn7, locals.var_dplim_dn8, locals.var_dplim_dn9, locals.var_dplim_dn10, locals.var_dplim_dn11, locals.var_dplim_dn14,)
    }
};
            locals.var_dplim = assign59390_body25_e92718;
            locals.var_dplim_dn0 = assign59390_body25_e92718_d_n0;
            locals.var_dplim_dn2 = assign59390_body25_e92718_d_n2;
            locals.var_dplim_dn4 = assign59390_body25_e92718_d_n4;
            locals.var_dplim_dn5 = assign59390_body25_e92718_d_n5;
            locals.var_dplim_dn6 = assign59390_body25_e92718_d_n6;
            locals.var_dplim_dn7 = assign59390_body25_e92718_d_n7;
            locals.var_dplim_dn8 = assign59390_body25_e92718_d_n8;
            locals.var_dplim_dn9 = assign59390_body25_e92718_d_n9;
            locals.var_dplim_dn10 = assign59390_body25_e92718_d_n10;
            locals.var_dplim_dn11 = assign59390_body25_e92718_d_n11;
            locals.var_dplim_dn14 = assign59390_body25_e92718_d_n14;
            locals.var_dplim_rv = 0.0;
            let assign59390_body26_e92720: f64 = (locals.var_dpsl).abs();
            let assign59390_body26_e92722: f64 = if assign59390_body26_e92720 > locals.var_dplim { 1.0 } else { 0.0 };
            locals.var_guard1460 = assign59390_body26_e92722;
            locals.var_guard1460_rv = 0.0;
            let (assign59390_body27_e92741, assign59390_body27_e92741_d_n0, assign59390_body27_e92741_d_n2, assign59390_body27_e92741_d_n4, assign59390_body27_e92741_d_n5, assign59390_body27_e92741_d_n6, assign59390_body27_e92741_d_n7, assign59390_body27_e92741_d_n8, assign59390_body27_e92741_d_n9, assign59390_body27_e92741_d_n10, assign59390_body27_e92741_d_n11, assign59390_body27_e92741_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1459 != 0.0)) && (locals.var_guard1460 != 0.0)) {
        let (assign59390_body27_e92738,) = {
            if (locals.var_dpsl >= 0.0) {
                (1.0,)
            } else {
                let assign59390_body27_e92737: f64 = (-1.0);
                (assign59390_body27_e92737,)
            }
        };
        let assign59390_body27_e92739: f64 = (locals.var_dplim * assign59390_body27_e92738);
        (assign59390_body27_e92739, (locals.var_dplim_dn0 * assign59390_body27_e92738), (locals.var_dplim_dn2 * assign59390_body27_e92738), (locals.var_dplim_dn4 * assign59390_body27_e92738), (locals.var_dplim_dn5 * assign59390_body27_e92738), (locals.var_dplim_dn6 * assign59390_body27_e92738), (locals.var_dplim_dn7 * assign59390_body27_e92738), (locals.var_dplim_dn8 * assign59390_body27_e92738), (locals.var_dplim_dn9 * assign59390_body27_e92738), (locals.var_dplim_dn10 * assign59390_body27_e92738), (locals.var_dplim_dn11 * assign59390_body27_e92738), (locals.var_dplim_dn14 * assign59390_body27_e92738),)
    } else {
        (locals.var_dpsl, locals.var_dpsl_dn0, locals.var_dpsl_dn2, locals.var_dpsl_dn4, locals.var_dpsl_dn5, locals.var_dpsl_dn6, locals.var_dpsl_dn7, locals.var_dpsl_dn8, locals.var_dpsl_dn9, locals.var_dpsl_dn10, locals.var_dpsl_dn11, locals.var_dpsl_dn14,)
    }
};
            locals.var_dpsl = assign59390_body27_e92741;
            locals.var_dpsl_dn0 = assign59390_body27_e92741_d_n0;
            locals.var_dpsl_dn2 = assign59390_body27_e92741_d_n2;
            locals.var_dpsl_dn4 = assign59390_body27_e92741_d_n4;
            locals.var_dpsl_dn5 = assign59390_body27_e92741_d_n5;
            locals.var_dpsl_dn6 = assign59390_body27_e92741_d_n6;
            locals.var_dpsl_dn7 = assign59390_body27_e92741_d_n7;
            locals.var_dpsl_dn8 = assign59390_body27_e92741_d_n8;
            locals.var_dpsl_dn9 = assign59390_body27_e92741_d_n9;
            locals.var_dpsl_dn10 = assign59390_body27_e92741_d_n10;
            locals.var_dpsl_dn11 = assign59390_body27_e92741_d_n11;
            locals.var_dpsl_dn14 = assign59390_body27_e92741_d_n14;
            locals.var_dpsl_rv = 0.0;
            let (assign59390_body28_e92752, assign59390_body28_e92752_d_n0, assign59390_body28_e92752_d_n2, assign59390_body28_e92752_d_n4, assign59390_body28_e92752_d_n5, assign59390_body28_e92752_d_n6, assign59390_body28_e92752_d_n7, assign59390_body28_e92752_d_n8, assign59390_body28_e92752_d_n9, assign59390_body28_e92752_d_n10, assign59390_body28_e92752_d_n11, assign59390_body28_e92752_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1459 != 0.0)) {
        let assign59390_body28_e92750: f64 = (locals.var_psl + locals.var_dpsl);
        (assign59390_body28_e92750, (locals.var_psl_dn0 + locals.var_dpsl_dn0), (locals.var_psl_dn2 + locals.var_dpsl_dn2), (locals.var_psl_dn4 + locals.var_dpsl_dn4), (locals.var_psl_dn5 + locals.var_dpsl_dn5), (locals.var_psl_dn6 + locals.var_dpsl_dn6), (locals.var_psl_dn7 + locals.var_dpsl_dn7), (locals.var_psl_dn8 + locals.var_dpsl_dn8), (locals.var_psl_dn9 + locals.var_dpsl_dn9), (locals.var_psl_dn10 + locals.var_dpsl_dn10), (locals.var_psl_dn11 + locals.var_dpsl_dn11), (locals.var_psl_dn14 + locals.var_dpsl_dn14),)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn4, locals.var_psl_dn5, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn8, locals.var_psl_dn9, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn14,)
    }
};
            locals.var_psl = assign59390_body28_e92752;
            locals.var_psl_dn0 = assign59390_body28_e92752_d_n0;
            locals.var_psl_dn2 = assign59390_body28_e92752_d_n2;
            locals.var_psl_dn4 = assign59390_body28_e92752_d_n4;
            locals.var_psl_dn5 = assign59390_body28_e92752_d_n5;
            locals.var_psl_dn6 = assign59390_body28_e92752_d_n6;
            locals.var_psl_dn7 = assign59390_body28_e92752_d_n7;
            locals.var_psl_dn8 = assign59390_body28_e92752_d_n8;
            locals.var_psl_dn9 = assign59390_body28_e92752_d_n9;
            locals.var_psl_dn10 = assign59390_body28_e92752_d_n10;
            locals.var_psl_dn11 = assign59390_body28_e92752_d_n11;
            locals.var_psl_dn14 = assign59390_body28_e92752_d_n14;
            locals.var_psl_rv = 0.0;
            let assign59390_body29_e92754: f64 = (locals.var_dpsl).abs();
            let assign59390_body29_e92758: f64 = (locals.var_fsl).abs();
            let assign59390_body29_e92761: f64 = if ((assign59390_body29_e92754 <= 1e-12) && (assign59390_body29_e92758 <= 1e-8)) { 1.0 } else { 0.0 };
            locals.var_guard1461 = assign59390_body29_e92761;
            locals.var_guard1461_rv = 0.0;
            let (assign59390_body30_e92772,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1459 != 0.0)) && (locals.var_guard1461 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign59390_body30_e92772;
            locals.var_flg_conv_rv = 0.0;
            let (assign59390_body31_e92783,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_flg_brk2 != 0.0)) {
        let assign59390_body31_e92781: f64 = (40.0 + 1.0);
        (assign59390_body31_e92781,)
    } else {
        (locals.var_lp_sl,)
    }
};
            locals.var_lp_sl = assign59390_body31_e92783;
            locals.var_lp_sl_rv = 0.0;
            let (assign59390_body32_e92790,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_brk2,)
    }
};
            locals.var_flg_brk2 = assign59390_body32_e92790;
            locals.var_flg_brk2_rv = 0.0;
            let (assign59390_body33_e92799,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign59390_body33_e92797: f64 = (locals.var_lp_sl + 1.0);
        (assign59390_body33_e92797,)
    } else {
        (locals.var_lp_sl,)
    }
};
            locals.var_lp_sl = assign59390_body33_e92799;
            locals.var_lp_sl_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_221(
        locals: &mut StampLocals,
    ) {
        let (assign59400_e92808,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign59400_e92806: f64 = (locals.var_lp_sl - 1.0);
        (assign59400_e92806,)
    } else {
        (locals.var_lp_sl,)
    }
};
        locals.var_lp_sl = assign59400_e92808;
        locals.var_lp_sl_rv = 0.0;

        let assign59420_e92814: f64 = if locals.var_chi < 5.0 { 1.0 } else { 0.0 };
        locals.var_guard1463 = assign59420_e92814;
        locals.var_guard1463_rv = 0.0;

        let (assign59430_e92829, assign59430_e92829_d_n0, assign59430_e92829_d_n2, assign59430_e92829_d_n4, assign59430_e92829_d_n5, assign59430_e92829_d_n6, assign59430_e92829_d_n7, assign59430_e92829_d_n8, assign59430_e92829_d_n9, assign59430_e92829_d_n10, assign59430_e92829_d_n11, assign59430_e92829_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign59430_e92823: f64 = (locals.var_fb * locals.var_fb);
        let assign59430_e92826: f64 = (10.0 * 2.220446049250313e-16);
        let assign59430_e92827: f64 = (assign59430_e92823 + assign59430_e92826);
        (assign59430_e92827, ((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)), ((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)), ((locals.var_fb_dn4 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn4)), ((locals.var_fb_dn5 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn5)), ((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)), ((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)), ((locals.var_fb_dn8 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn8)), ((locals.var_fb_dn9 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn9)), ((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)), ((locals.var_fb_dn11 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn11)), ((locals.var_fb_dn14 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn14)),)
    } else {
        (locals.var_xil, locals.var_xil_dn0, locals.var_xil_dn2, locals.var_xil_dn4, locals.var_xil_dn5, locals.var_xil_dn6, locals.var_xil_dn7, locals.var_xil_dn8, locals.var_xil_dn9, locals.var_xil_dn10, locals.var_xil_dn11, locals.var_xil_dn14,)
    }
};
        locals.var_xil = assign59430_e92829;
        locals.var_xil_dn0 = assign59430_e92829_d_n0;
        locals.var_xil_dn2 = assign59430_e92829_d_n2;
        locals.var_xil_dn4 = assign59430_e92829_d_n4;
        locals.var_xil_dn5 = assign59430_e92829_d_n5;
        locals.var_xil_dn6 = assign59430_e92829_d_n6;
        locals.var_xil_dn7 = assign59430_e92829_d_n7;
        locals.var_xil_dn8 = assign59430_e92829_d_n8;
        locals.var_xil_dn9 = assign59430_e92829_d_n9;
        locals.var_xil_dn10 = assign59430_e92829_d_n10;
        locals.var_xil_dn11 = assign59430_e92829_d_n11;
        locals.var_xil_dn14 = assign59430_e92829_d_n14;
        locals.var_xil_rv = 0.0;

        let (assign59440_e92842, assign59440_e92842_d_n0, assign59440_e92842_d_n2, assign59440_e92842_d_n4, assign59440_e92842_d_n5, assign59440_e92842_d_n6, assign59440_e92842_d_n7, assign59440_e92842_d_n8, assign59440_e92842_d_n9, assign59440_e92842_d_n10, assign59440_e92842_d_n11, assign59440_e92842_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign59440_e92839: f64 = (10.0 * 2.220446049250313e-16);
        let assign59440_e92840: f64 = (locals.var_fb + assign59440_e92839);
        (assign59440_e92840, locals.var_fb_dn0, locals.var_fb_dn2, locals.var_fb_dn4, locals.var_fb_dn5, locals.var_fb_dn6, locals.var_fb_dn7, locals.var_fb_dn8, locals.var_fb_dn9, locals.var_fb_dn10, locals.var_fb_dn11, locals.var_fb_dn14,)
    } else {
        (locals.var_xilp12, locals.var_xilp12_dn0, locals.var_xilp12_dn2, locals.var_xilp12_dn4, locals.var_xilp12_dn5, locals.var_xilp12_dn6, locals.var_xilp12_dn7, locals.var_xilp12_dn8, locals.var_xilp12_dn9, locals.var_xilp12_dn10, locals.var_xilp12_dn11, locals.var_xilp12_dn14,)
    }
};
        locals.var_xilp12 = assign59440_e92842;
        locals.var_xilp12_dn0 = assign59440_e92842_d_n0;
        locals.var_xilp12_dn2 = assign59440_e92842_d_n2;
        locals.var_xilp12_dn4 = assign59440_e92842_d_n4;
        locals.var_xilp12_dn5 = assign59440_e92842_d_n5;
        locals.var_xilp12_dn6 = assign59440_e92842_d_n6;
        locals.var_xilp12_dn7 = assign59440_e92842_d_n7;
        locals.var_xilp12_dn8 = assign59440_e92842_d_n8;
        locals.var_xilp12_dn9 = assign59440_e92842_d_n9;
        locals.var_xilp12_dn10 = assign59440_e92842_d_n10;
        locals.var_xilp12_dn11 = assign59440_e92842_d_n11;
        locals.var_xilp12_dn14 = assign59440_e92842_d_n14;
        locals.var_xilp12_rv = 0.0;

        let (assign59450_e92859, assign59450_e92859_d_n0, assign59450_e92859_d_n2, assign59450_e92859_d_n4, assign59450_e92859_d_n5, assign59450_e92859_d_n6, assign59450_e92859_d_n7, assign59450_e92859_d_n8, assign59450_e92859_d_n9, assign59450_e92859_d_n10, assign59450_e92859_d_n11, assign59450_e92859_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1463 != 0.0)) {
        let assign59450_e92851: f64 = (locals.var_fb * locals.var_fb);
        let assign59450_e92853: f64 = (assign59450_e92851 * locals.var_fb);
        let assign59450_e92856: f64 = (10.0 * 2.220446049250313e-16);
        let assign59450_e92857: f64 = (assign59450_e92853 + assign59450_e92856);
        (assign59450_e92857, ((((locals.var_fb_dn0 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn0)) * locals.var_fb) + (assign59450_e92851 * locals.var_fb_dn0)), ((((locals.var_fb_dn2 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn2)) * locals.var_fb) + (assign59450_e92851 * locals.var_fb_dn2)), ((((locals.var_fb_dn4 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn4)) * locals.var_fb) + (assign59450_e92851 * locals.var_fb_dn4)), ((((locals.var_fb_dn5 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn5)) * locals.var_fb) + (assign59450_e92851 * locals.var_fb_dn5)), ((((locals.var_fb_dn6 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn6)) * locals.var_fb) + (assign59450_e92851 * locals.var_fb_dn6)), ((((locals.var_fb_dn7 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn7)) * locals.var_fb) + (assign59450_e92851 * locals.var_fb_dn7)), ((((locals.var_fb_dn8 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn8)) * locals.var_fb) + (assign59450_e92851 * locals.var_fb_dn8)), ((((locals.var_fb_dn9 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn9)) * locals.var_fb) + (assign59450_e92851 * locals.var_fb_dn9)), ((((locals.var_fb_dn10 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn10)) * locals.var_fb) + (assign59450_e92851 * locals.var_fb_dn10)), ((((locals.var_fb_dn11 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn11)) * locals.var_fb) + (assign59450_e92851 * locals.var_fb_dn11)), ((((locals.var_fb_dn14 * locals.var_fb) + (locals.var_fb * locals.var_fb_dn14)) * locals.var_fb) + (assign59450_e92851 * locals.var_fb_dn14)),)
    } else {
        (locals.var_xilp32, locals.var_xilp32_dn0, locals.var_xilp32_dn2, locals.var_xilp32_dn4, locals.var_xilp32_dn5, locals.var_xilp32_dn6, locals.var_xilp32_dn7, locals.var_xilp32_dn8, locals.var_xilp32_dn9, locals.var_xilp32_dn10, locals.var_xilp32_dn11, locals.var_xilp32_dn14,)
    }
};
        locals.var_xilp32 = assign59450_e92859;
        locals.var_xilp32_dn0 = assign59450_e92859_d_n0;
        locals.var_xilp32_dn2 = assign59450_e92859_d_n2;
        locals.var_xilp32_dn4 = assign59450_e92859_d_n4;
        locals.var_xilp32_dn5 = assign59450_e92859_d_n5;
        locals.var_xilp32_dn6 = assign59450_e92859_d_n6;
        locals.var_xilp32_dn7 = assign59450_e92859_d_n7;
        locals.var_xilp32_dn8 = assign59450_e92859_d_n8;
        locals.var_xilp32_dn9 = assign59450_e92859_d_n9;
        locals.var_xilp32_dn10 = assign59450_e92859_d_n10;
        locals.var_xilp32_dn11 = assign59450_e92859_d_n11;
        locals.var_xilp32_dn14 = assign59450_e92859_d_n14;
        locals.var_xilp32_rv = 0.0;

        let (assign59460_e92871, assign59460_e92871_d_n0, assign59460_e92871_d_n2, assign59460_e92871_d_n4, assign59460_e92871_d_n5, assign59460_e92871_d_n6, assign59460_e92871_d_n7, assign59460_e92871_d_n8, assign59460_e92871_d_n9, assign59460_e92871_d_n10, assign59460_e92871_d_n11, assign59460_e92871_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1463 == 0.0)) {
        let assign59460_e92869: f64 = (locals.var_chi - 1.0);
        (assign59460_e92869, locals.var_chi_dn0, locals.var_chi_dn2, locals.var_chi_dn4, locals.var_chi_dn5, locals.var_chi_dn6, locals.var_chi_dn7, locals.var_chi_dn8, locals.var_chi_dn9, locals.var_chi_dn10, locals.var_chi_dn11, locals.var_chi_dn14,)
    } else {
        (locals.var_xil, locals.var_xil_dn0, locals.var_xil_dn2, locals.var_xil_dn4, locals.var_xil_dn5, locals.var_xil_dn6, locals.var_xil_dn7, locals.var_xil_dn8, locals.var_xil_dn9, locals.var_xil_dn10, locals.var_xil_dn11, locals.var_xil_dn14,)
    }
};
        locals.var_xil = assign59460_e92871;
        locals.var_xil_dn0 = assign59460_e92871_d_n0;
        locals.var_xil_dn2 = assign59460_e92871_d_n2;
        locals.var_xil_dn4 = assign59460_e92871_d_n4;
        locals.var_xil_dn5 = assign59460_e92871_d_n5;
        locals.var_xil_dn6 = assign59460_e92871_d_n6;
        locals.var_xil_dn7 = assign59460_e92871_d_n7;
        locals.var_xil_dn8 = assign59460_e92871_d_n8;
        locals.var_xil_dn9 = assign59460_e92871_d_n9;
        locals.var_xil_dn10 = assign59460_e92871_d_n10;
        locals.var_xil_dn11 = assign59460_e92871_d_n11;
        locals.var_xil_dn14 = assign59460_e92871_d_n14;
        locals.var_xil_rv = 0.0;

        let (assign59470_e92882, assign59470_e92882_d_n0, assign59470_e92882_d_n2, assign59470_e92882_d_n4, assign59470_e92882_d_n5, assign59470_e92882_d_n6, assign59470_e92882_d_n7, assign59470_e92882_d_n8, assign59470_e92882_d_n9, assign59470_e92882_d_n10, assign59470_e92882_d_n11, assign59470_e92882_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1463 == 0.0)) {
        let assign59470_e92880: f64 = (locals.var_xil).sqrt();
        (assign59470_e92880, (locals.var_xil_dn0 / (2.0 * assign59470_e92880)), (locals.var_xil_dn2 / (2.0 * assign59470_e92880)), (locals.var_xil_dn4 / (2.0 * assign59470_e92880)), (locals.var_xil_dn5 / (2.0 * assign59470_e92880)), (locals.var_xil_dn6 / (2.0 * assign59470_e92880)), (locals.var_xil_dn7 / (2.0 * assign59470_e92880)), (locals.var_xil_dn8 / (2.0 * assign59470_e92880)), (locals.var_xil_dn9 / (2.0 * assign59470_e92880)), (locals.var_xil_dn10 / (2.0 * assign59470_e92880)), (locals.var_xil_dn11 / (2.0 * assign59470_e92880)), (locals.var_xil_dn14 / (2.0 * assign59470_e92880)),)
    } else {
        (locals.var_xilp12, locals.var_xilp12_dn0, locals.var_xilp12_dn2, locals.var_xilp12_dn4, locals.var_xilp12_dn5, locals.var_xilp12_dn6, locals.var_xilp12_dn7, locals.var_xilp12_dn8, locals.var_xilp12_dn9, locals.var_xilp12_dn10, locals.var_xilp12_dn11, locals.var_xilp12_dn14,)
    }
};
        locals.var_xilp12 = assign59470_e92882;
        locals.var_xilp12_dn0 = assign59470_e92882_d_n0;
        locals.var_xilp12_dn2 = assign59470_e92882_d_n2;
        locals.var_xilp12_dn4 = assign59470_e92882_d_n4;
        locals.var_xilp12_dn5 = assign59470_e92882_d_n5;
        locals.var_xilp12_dn6 = assign59470_e92882_d_n6;
        locals.var_xilp12_dn7 = assign59470_e92882_d_n7;
        locals.var_xilp12_dn8 = assign59470_e92882_d_n8;
        locals.var_xilp12_dn9 = assign59470_e92882_d_n9;
        locals.var_xilp12_dn10 = assign59470_e92882_d_n10;
        locals.var_xilp12_dn11 = assign59470_e92882_d_n11;
        locals.var_xilp12_dn14 = assign59470_e92882_d_n14;
        locals.var_xilp12_rv = 0.0;

        let (assign59480_e92894, assign59480_e92894_d_n0, assign59480_e92894_d_n2, assign59480_e92894_d_n4, assign59480_e92894_d_n5, assign59480_e92894_d_n6, assign59480_e92894_d_n7, assign59480_e92894_d_n8, assign59480_e92894_d_n9, assign59480_e92894_d_n10, assign59480_e92894_d_n11, assign59480_e92894_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1463 == 0.0)) {
        let assign59480_e92892: f64 = (locals.var_xil * locals.var_xilp12);
        (assign59480_e92892, ((locals.var_xil_dn0 * locals.var_xilp12) + (locals.var_xil * locals.var_xilp12_dn0)), ((locals.var_xil_dn2 * locals.var_xilp12) + (locals.var_xil * locals.var_xilp12_dn2)), ((locals.var_xil_dn4 * locals.var_xilp12) + (locals.var_xil * locals.var_xilp12_dn4)), ((locals.var_xil_dn5 * locals.var_xilp12) + (locals.var_xil * locals.var_xilp12_dn5)), ((locals.var_xil_dn6 * locals.var_xilp12) + (locals.var_xil * locals.var_xilp12_dn6)), ((locals.var_xil_dn7 * locals.var_xilp12) + (locals.var_xil * locals.var_xilp12_dn7)), ((locals.var_xil_dn8 * locals.var_xilp12) + (locals.var_xil * locals.var_xilp12_dn8)), ((locals.var_xil_dn9 * locals.var_xilp12) + (locals.var_xil * locals.var_xilp12_dn9)), ((locals.var_xil_dn10 * locals.var_xilp12) + (locals.var_xil * locals.var_xilp12_dn10)), ((locals.var_xil_dn11 * locals.var_xilp12) + (locals.var_xil * locals.var_xilp12_dn11)), ((locals.var_xil_dn14 * locals.var_xilp12) + (locals.var_xil * locals.var_xilp12_dn14)),)
    } else {
        (locals.var_xilp32, locals.var_xilp32_dn0, locals.var_xilp32_dn2, locals.var_xilp32_dn4, locals.var_xilp32_dn5, locals.var_xilp32_dn6, locals.var_xilp32_dn7, locals.var_xilp32_dn8, locals.var_xilp32_dn9, locals.var_xilp32_dn10, locals.var_xilp32_dn11, locals.var_xilp32_dn14,)
    }
};
        locals.var_xilp32 = assign59480_e92894;
        locals.var_xilp32_dn0 = assign59480_e92894_d_n0;
        locals.var_xilp32_dn2 = assign59480_e92894_d_n2;
        locals.var_xilp32_dn4 = assign59480_e92894_d_n4;
        locals.var_xilp32_dn5 = assign59480_e92894_d_n5;
        locals.var_xilp32_dn6 = assign59480_e92894_d_n6;
        locals.var_xilp32_dn7 = assign59480_e92894_d_n7;
        locals.var_xilp32_dn8 = assign59480_e92894_d_n8;
        locals.var_xilp32_dn9 = assign59480_e92894_d_n9;
        locals.var_xilp32_dn10 = assign59480_e92894_d_n10;
        locals.var_xilp32_dn11 = assign59480_e92894_d_n11;
        locals.var_xilp32_dn14 = assign59480_e92894_d_n14;
        locals.var_xilp32_rv = 0.0;

        let (assign59490_e92903, assign59490_e92903_d_n0, assign59490_e92903_d_n2, assign59490_e92903_d_n4, assign59490_e92903_d_n5, assign59490_e92903_d_n6, assign59490_e92903_d_n7, assign59490_e92903_d_n8, assign59490_e92903_d_n9, assign59490_e92903_d_n10, assign59490_e92903_d_n11, assign59490_e92903_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign59490_e92901: f64 = (locals.var_psl - locals.var_ps0);
        (assign59490_e92901, (locals.var_psl_dn0 - locals.var_ps0_dn0), (locals.var_psl_dn2 - locals.var_ps0_dn2), (locals.var_psl_dn4 - locals.var_ps0_dn4), (locals.var_psl_dn5 - locals.var_ps0_dn5), (locals.var_psl_dn6 - locals.var_ps0_dn6), (locals.var_psl_dn7 - locals.var_ps0_dn7), (locals.var_psl_dn8 - locals.var_ps0_dn8), (locals.var_psl_dn9 - locals.var_ps0_dn9), (locals.var_psl_dn10 - locals.var_ps0_dn10), (locals.var_psl_dn11 - locals.var_ps0_dn11), (locals.var_psl_dn14 - locals.var_ps0_dn14),)
    } else {
        (locals.var_pds, locals.var_pds_dn0, locals.var_pds_dn2, locals.var_pds_dn4, locals.var_pds_dn5, locals.var_pds_dn6, locals.var_pds_dn7, locals.var_pds_dn8, locals.var_pds_dn9, locals.var_pds_dn10, locals.var_pds_dn11, locals.var_pds_dn14,)
    }
};
        locals.var_pds = assign59490_e92903;
        locals.var_pds_dn0 = assign59490_e92903_d_n0;
        locals.var_pds_dn2 = assign59490_e92903_d_n2;
        locals.var_pds_dn4 = assign59490_e92903_d_n4;
        locals.var_pds_dn5 = assign59490_e92903_d_n5;
        locals.var_pds_dn6 = assign59490_e92903_d_n6;
        locals.var_pds_dn7 = assign59490_e92903_d_n7;
        locals.var_pds_dn8 = assign59490_e92903_d_n8;
        locals.var_pds_dn9 = assign59490_e92903_d_n9;
        locals.var_pds_dn10 = assign59490_e92903_d_n10;
        locals.var_pds_dn11 = assign59490_e92903_d_n11;
        locals.var_pds_dn14 = assign59490_e92903_d_n14;
        locals.var_pds_rv = 0.0;

        let (assign59500_e92910, assign59500_e92910_d_n0, assign59500_e92910_d_n2, assign59500_e92910_d_n4, assign59500_e92910_d_n5, assign59500_e92910_d_n6, assign59500_e92910_d_n7, assign59500_e92910_d_n8, assign59500_e92910_d_n9, assign59500_e92910_d_n10, assign59500_e92910_d_n11, assign59500_e92910_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn4, locals.var_vdsorg_dn5, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn8, locals.var_vdsorg_dn9, locals.var_vdsorg_dn10, locals.var_vdsorg_dn11, locals.var_vdsorg_dn14,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn11, locals.var_vds_dn14,)
    }
};
        locals.var_vds = assign59500_e92910;
        locals.var_vds_dn0 = assign59500_e92910_d_n0;
        locals.var_vds_dn2 = assign59500_e92910_d_n2;
        locals.var_vds_dn4 = assign59500_e92910_d_n4;
        locals.var_vds_dn5 = assign59500_e92910_d_n5;
        locals.var_vds_dn6 = assign59500_e92910_d_n6;
        locals.var_vds_dn7 = assign59500_e92910_d_n7;
        locals.var_vds_dn8 = assign59500_e92910_d_n8;
        locals.var_vds_dn9 = assign59500_e92910_d_n9;
        locals.var_vds_dn10 = assign59500_e92910_d_n10;
        locals.var_vds_dn11 = assign59500_e92910_d_n11;
        locals.var_vds_dn14 = assign59500_e92910_d_n14;
        locals.var_vds_rv = 0.0;

        let (assign59510_e92919, assign59510_e92919_d_n0, assign59510_e92919_d_n2, assign59510_e92919_d_n4, assign59510_e92919_d_n5, assign59510_e92919_d_n6, assign59510_e92919_d_n7, assign59510_e92919_d_n8, assign59510_e92919_d_n9, assign59510_e92919_d_n10, assign59510_e92919_d_n11, assign59510_e92919_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign59510_e92917: f64 = (locals.var_beta / locals.var_xi0);
        (assign59510_e92917, (((locals.var_beta_dn0 * locals.var_xi0) - (locals.var_beta * locals.var_xi0_dn0)) / (locals.var_xi0 * locals.var_xi0)), (((locals.var_beta_dn2 * locals.var_xi0) - (locals.var_beta * locals.var_xi0_dn2)) / (locals.var_xi0 * locals.var_xi0)), (((locals.var_beta_dn4 * locals.var_xi0) - (locals.var_beta * locals.var_xi0_dn4)) / (locals.var_xi0 * locals.var_xi0)), (((locals.var_beta_dn5 * locals.var_xi0) - (locals.var_beta * locals.var_xi0_dn5)) / (locals.var_xi0 * locals.var_xi0)), (((locals.var_beta_dn6 * locals.var_xi0) - (locals.var_beta * locals.var_xi0_dn6)) / (locals.var_xi0 * locals.var_xi0)), (((locals.var_beta_dn7 * locals.var_xi0) - (locals.var_beta * locals.var_xi0_dn7)) / (locals.var_xi0 * locals.var_xi0)), (((locals.var_beta_dn8 * locals.var_xi0) - (locals.var_beta * locals.var_xi0_dn8)) / (locals.var_xi0 * locals.var_xi0)), (((locals.var_beta_dn9 * locals.var_xi0) - (locals.var_beta * locals.var_xi0_dn9)) / (locals.var_xi0 * locals.var_xi0)), (((locals.var_beta_dn10 * locals.var_xi0) - (locals.var_beta * locals.var_xi0_dn10)) / (locals.var_xi0 * locals.var_xi0)), (((locals.var_beta_dn11 * locals.var_xi0) - (locals.var_beta * locals.var_xi0_dn11)) / (locals.var_xi0 * locals.var_xi0)), (((locals.var_beta_dn14 * locals.var_xi0) - (locals.var_beta * locals.var_xi0_dn14)) / (locals.var_xi0 * locals.var_xi0)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign59510_e92919;
        locals.var_t1_dn0 = assign59510_e92919_d_n0;
        locals.var_t1_dn2 = assign59510_e92919_d_n2;
        locals.var_t1_dn4 = assign59510_e92919_d_n4;
        locals.var_t1_dn5 = assign59510_e92919_d_n5;
        locals.var_t1_dn6 = assign59510_e92919_d_n6;
        locals.var_t1_dn7 = assign59510_e92919_d_n7;
        locals.var_t1_dn8 = assign59510_e92919_d_n8;
        locals.var_t1_dn9 = assign59510_e92919_d_n9;
        locals.var_t1_dn10 = assign59510_e92919_d_n10;
        locals.var_t1_dn11 = assign59510_e92919_d_n11;
        locals.var_t1_dn14 = assign59510_e92919_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign59520_e92928, assign59520_e92928_d_n0, assign59520_e92928_d_n2, assign59520_e92928_d_n4, assign59520_e92928_d_n5, assign59520_e92928_d_n6, assign59520_e92928_d_n7, assign59520_e92928_d_n8, assign59520_e92928_d_n9, assign59520_e92928_d_n10, assign59520_e92928_d_n11, assign59520_e92928_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign59520_e92926: f64 = (locals.var_t1 * locals.var_pds);
        (assign59520_e92926, ((locals.var_t1_dn0 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn0)), ((locals.var_t1_dn2 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn2)), ((locals.var_t1_dn4 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn4)), ((locals.var_t1_dn5 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn5)), ((locals.var_t1_dn6 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn6)), ((locals.var_t1_dn7 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn7)), ((locals.var_t1_dn8 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn8)), ((locals.var_t1_dn9 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn9)), ((locals.var_t1_dn10 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn10)), ((locals.var_t1_dn11 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn11)), ((locals.var_t1_dn14 * locals.var_pds) + (locals.var_t1 * locals.var_pds_dn14)),)
    } else {
        (locals.var_eta, locals.var_eta_dn0, locals.var_eta_dn2, locals.var_eta_dn4, locals.var_eta_dn5, locals.var_eta_dn6, locals.var_eta_dn7, locals.var_eta_dn8, locals.var_eta_dn9, locals.var_eta_dn10, locals.var_eta_dn11, locals.var_eta_dn14,)
    }
};
        locals.var_eta = assign59520_e92928;
        locals.var_eta_dn0 = assign59520_e92928_d_n0;
        locals.var_eta_dn2 = assign59520_e92928_d_n2;
        locals.var_eta_dn4 = assign59520_e92928_d_n4;
        locals.var_eta_dn5 = assign59520_e92928_d_n5;
        locals.var_eta_dn6 = assign59520_e92928_d_n6;
        locals.var_eta_dn7 = assign59520_e92928_d_n7;
        locals.var_eta_dn8 = assign59520_e92928_d_n8;
        locals.var_eta_dn9 = assign59520_e92928_d_n9;
        locals.var_eta_dn10 = assign59520_e92928_d_n10;
        locals.var_eta_dn11 = assign59520_e92928_d_n11;
        locals.var_eta_dn14 = assign59520_e92928_d_n14;
        locals.var_eta_rv = 0.0;

        let (assign59530_e92937, assign59530_e92937_d_n0, assign59530_e92937_d_n2, assign59530_e92937_d_n4, assign59530_e92937_d_n5, assign59530_e92937_d_n6, assign59530_e92937_d_n7, assign59530_e92937_d_n8, assign59530_e92937_d_n9, assign59530_e92937_d_n10, assign59530_e92937_d_n11, assign59530_e92937_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign59530_e92935: f64 = (locals.var_eta + 1.0);
        (assign59530_e92935, locals.var_eta_dn0, locals.var_eta_dn2, locals.var_eta_dn4, locals.var_eta_dn5, locals.var_eta_dn6, locals.var_eta_dn7, locals.var_eta_dn8, locals.var_eta_dn9, locals.var_eta_dn10, locals.var_eta_dn11, locals.var_eta_dn14,)
    } else {
        (locals.var_eta1, locals.var_eta1_dn0, locals.var_eta1_dn2, locals.var_eta1_dn4, locals.var_eta1_dn5, locals.var_eta1_dn6, locals.var_eta1_dn7, locals.var_eta1_dn8, locals.var_eta1_dn9, locals.var_eta1_dn10, locals.var_eta1_dn11, locals.var_eta1_dn14,)
    }
};
        locals.var_eta1 = assign59530_e92937;
        locals.var_eta1_dn0 = assign59530_e92937_d_n0;
        locals.var_eta1_dn2 = assign59530_e92937_d_n2;
        locals.var_eta1_dn4 = assign59530_e92937_d_n4;
        locals.var_eta1_dn5 = assign59530_e92937_d_n5;
        locals.var_eta1_dn6 = assign59530_e92937_d_n6;
        locals.var_eta1_dn7 = assign59530_e92937_d_n7;
        locals.var_eta1_dn8 = assign59530_e92937_d_n8;
        locals.var_eta1_dn9 = assign59530_e92937_d_n9;
        locals.var_eta1_dn10 = assign59530_e92937_d_n10;
        locals.var_eta1_dn11 = assign59530_e92937_d_n11;
        locals.var_eta1_dn14 = assign59530_e92937_d_n14;
        locals.var_eta1_rv = 0.0;

        let (assign59540_e92945, assign59540_e92945_d_n0, assign59540_e92945_d_n2, assign59540_e92945_d_n4, assign59540_e92945_d_n5, assign59540_e92945_d_n6, assign59540_e92945_d_n7, assign59540_e92945_d_n8, assign59540_e92945_d_n9, assign59540_e92945_d_n10, assign59540_e92945_d_n11, assign59540_e92945_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign59540_e92943: f64 = (locals.var_eta1).sqrt();
        (assign59540_e92943, (locals.var_eta1_dn0 / (2.0 * assign59540_e92943)), (locals.var_eta1_dn2 / (2.0 * assign59540_e92943)), (locals.var_eta1_dn4 / (2.0 * assign59540_e92943)), (locals.var_eta1_dn5 / (2.0 * assign59540_e92943)), (locals.var_eta1_dn6 / (2.0 * assign59540_e92943)), (locals.var_eta1_dn7 / (2.0 * assign59540_e92943)), (locals.var_eta1_dn8 / (2.0 * assign59540_e92943)), (locals.var_eta1_dn9 / (2.0 * assign59540_e92943)), (locals.var_eta1_dn10 / (2.0 * assign59540_e92943)), (locals.var_eta1_dn11 / (2.0 * assign59540_e92943)), (locals.var_eta1_dn14 / (2.0 * assign59540_e92943)),)
    } else {
        (locals.var_eta1p12, locals.var_eta1p12_dn0, locals.var_eta1p12_dn2, locals.var_eta1p12_dn4, locals.var_eta1p12_dn5, locals.var_eta1p12_dn6, locals.var_eta1p12_dn7, locals.var_eta1p12_dn8, locals.var_eta1p12_dn9, locals.var_eta1p12_dn10, locals.var_eta1p12_dn11, locals.var_eta1p12_dn14,)
    }
};
        locals.var_eta1p12 = assign59540_e92945;
        locals.var_eta1p12_dn0 = assign59540_e92945_d_n0;
        locals.var_eta1p12_dn2 = assign59540_e92945_d_n2;
        locals.var_eta1p12_dn4 = assign59540_e92945_d_n4;
        locals.var_eta1p12_dn5 = assign59540_e92945_d_n5;
        locals.var_eta1p12_dn6 = assign59540_e92945_d_n6;
        locals.var_eta1p12_dn7 = assign59540_e92945_d_n7;
        locals.var_eta1p12_dn8 = assign59540_e92945_d_n8;
        locals.var_eta1p12_dn9 = assign59540_e92945_d_n9;
        locals.var_eta1p12_dn10 = assign59540_e92945_d_n10;
        locals.var_eta1p12_dn11 = assign59540_e92945_d_n11;
        locals.var_eta1p12_dn14 = assign59540_e92945_d_n14;
        locals.var_eta1p12_rv = 0.0;

        let (assign59550_e92954, assign59550_e92954_d_n0, assign59550_e92954_d_n2, assign59550_e92954_d_n4, assign59550_e92954_d_n5, assign59550_e92954_d_n6, assign59550_e92954_d_n7, assign59550_e92954_d_n8, assign59550_e92954_d_n9, assign59550_e92954_d_n10, assign59550_e92954_d_n11, assign59550_e92954_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign59550_e92952: f64 = (locals.var_eta1p12 * locals.var_eta1);
        (assign59550_e92952, ((locals.var_eta1p12_dn0 * locals.var_eta1) + (locals.var_eta1p12 * locals.var_eta1_dn0)), ((locals.var_eta1p12_dn2 * locals.var_eta1) + (locals.var_eta1p12 * locals.var_eta1_dn2)), ((locals.var_eta1p12_dn4 * locals.var_eta1) + (locals.var_eta1p12 * locals.var_eta1_dn4)), ((locals.var_eta1p12_dn5 * locals.var_eta1) + (locals.var_eta1p12 * locals.var_eta1_dn5)), ((locals.var_eta1p12_dn6 * locals.var_eta1) + (locals.var_eta1p12 * locals.var_eta1_dn6)), ((locals.var_eta1p12_dn7 * locals.var_eta1) + (locals.var_eta1p12 * locals.var_eta1_dn7)), ((locals.var_eta1p12_dn8 * locals.var_eta1) + (locals.var_eta1p12 * locals.var_eta1_dn8)), ((locals.var_eta1p12_dn9 * locals.var_eta1) + (locals.var_eta1p12 * locals.var_eta1_dn9)), ((locals.var_eta1p12_dn10 * locals.var_eta1) + (locals.var_eta1p12 * locals.var_eta1_dn10)), ((locals.var_eta1p12_dn11 * locals.var_eta1) + (locals.var_eta1p12 * locals.var_eta1_dn11)), ((locals.var_eta1p12_dn14 * locals.var_eta1) + (locals.var_eta1p12 * locals.var_eta1_dn14)),)
    } else {
        (locals.var_eta1p32, locals.var_eta1p32_dn0, locals.var_eta1p32_dn2, locals.var_eta1p32_dn4, locals.var_eta1p32_dn5, locals.var_eta1p32_dn6, locals.var_eta1p32_dn7, locals.var_eta1p32_dn8, locals.var_eta1p32_dn9, locals.var_eta1p32_dn10, locals.var_eta1p32_dn11, locals.var_eta1p32_dn14,)
    }
};
        locals.var_eta1p32 = assign59550_e92954;
        locals.var_eta1p32_dn0 = assign59550_e92954_d_n0;
        locals.var_eta1p32_dn2 = assign59550_e92954_d_n2;
        locals.var_eta1p32_dn4 = assign59550_e92954_d_n4;
        locals.var_eta1p32_dn5 = assign59550_e92954_d_n5;
        locals.var_eta1p32_dn6 = assign59550_e92954_d_n6;
        locals.var_eta1p32_dn7 = assign59550_e92954_d_n7;
        locals.var_eta1p32_dn8 = assign59550_e92954_d_n8;
        locals.var_eta1p32_dn9 = assign59550_e92954_d_n9;
        locals.var_eta1p32_dn10 = assign59550_e92954_d_n10;
        locals.var_eta1p32_dn11 = assign59550_e92954_d_n11;
        locals.var_eta1p32_dn14 = assign59550_e92954_d_n14;
        locals.var_eta1p32_rv = 0.0;

        let (assign59560_e92963, assign59560_e92963_d_n0, assign59560_e92963_d_n2, assign59560_e92963_d_n4, assign59560_e92963_d_n5, assign59560_e92963_d_n6, assign59560_e92963_d_n7, assign59560_e92963_d_n8, assign59560_e92963_d_n9, assign59560_e92963_d_n10, assign59560_e92963_d_n11, assign59560_e92963_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign59560_e92961: f64 = (locals.var_eta1p32 * locals.var_eta1);
        (assign59560_e92961, ((locals.var_eta1p32_dn0 * locals.var_eta1) + (locals.var_eta1p32 * locals.var_eta1_dn0)), ((locals.var_eta1p32_dn2 * locals.var_eta1) + (locals.var_eta1p32 * locals.var_eta1_dn2)), ((locals.var_eta1p32_dn4 * locals.var_eta1) + (locals.var_eta1p32 * locals.var_eta1_dn4)), ((locals.var_eta1p32_dn5 * locals.var_eta1) + (locals.var_eta1p32 * locals.var_eta1_dn5)), ((locals.var_eta1p32_dn6 * locals.var_eta1) + (locals.var_eta1p32 * locals.var_eta1_dn6)), ((locals.var_eta1p32_dn7 * locals.var_eta1) + (locals.var_eta1p32 * locals.var_eta1_dn7)), ((locals.var_eta1p32_dn8 * locals.var_eta1) + (locals.var_eta1p32 * locals.var_eta1_dn8)), ((locals.var_eta1p32_dn9 * locals.var_eta1) + (locals.var_eta1p32 * locals.var_eta1_dn9)), ((locals.var_eta1p32_dn10 * locals.var_eta1) + (locals.var_eta1p32 * locals.var_eta1_dn10)), ((locals.var_eta1p32_dn11 * locals.var_eta1) + (locals.var_eta1p32 * locals.var_eta1_dn11)), ((locals.var_eta1p32_dn14 * locals.var_eta1) + (locals.var_eta1p32 * locals.var_eta1_dn14)),)
    } else {
        (locals.var_eta1p52, locals.var_eta1p52_dn0, locals.var_eta1p52_dn2, locals.var_eta1p52_dn4, locals.var_eta1p52_dn5, locals.var_eta1p52_dn6, locals.var_eta1p52_dn7, locals.var_eta1p52_dn8, locals.var_eta1p52_dn9, locals.var_eta1p52_dn10, locals.var_eta1p52_dn11, locals.var_eta1p52_dn14,)
    }
};
        locals.var_eta1p52 = assign59560_e92963;
        locals.var_eta1p52_dn0 = assign59560_e92963_d_n0;
        locals.var_eta1p52_dn2 = assign59560_e92963_d_n2;
        locals.var_eta1p52_dn4 = assign59560_e92963_d_n4;
        locals.var_eta1p52_dn5 = assign59560_e92963_d_n5;
        locals.var_eta1p52_dn6 = assign59560_e92963_d_n6;
        locals.var_eta1p52_dn7 = assign59560_e92963_d_n7;
        locals.var_eta1p52_dn8 = assign59560_e92963_d_n8;
        locals.var_eta1p52_dn9 = assign59560_e92963_d_n9;
        locals.var_eta1p52_dn10 = assign59560_e92963_d_n10;
        locals.var_eta1p52_dn11 = assign59560_e92963_d_n11;
        locals.var_eta1p52_dn14 = assign59560_e92963_d_n14;
        locals.var_eta1p52_rv = 0.0;

        let (assign59570_e92974, assign59570_e92974_d_n0, assign59570_e92974_d_n2, assign59570_e92974_d_n4, assign59570_e92974_d_n5, assign59570_e92974_d_n6, assign59570_e92974_d_n7, assign59570_e92974_d_n8, assign59570_e92974_d_n9, assign59570_e92974_d_n10, assign59570_e92974_d_n11, assign59570_e92974_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign59570_e92971: f64 = (locals.var_eta1p12 + 1.0);
        let assign59570_e92972: f64 = (1.0 / assign59570_e92971);
        (assign59570_e92972, (-(locals.var_eta1p12_dn0 / (assign59570_e92971 * assign59570_e92971))), (-(locals.var_eta1p12_dn2 / (assign59570_e92971 * assign59570_e92971))), (-(locals.var_eta1p12_dn4 / (assign59570_e92971 * assign59570_e92971))), (-(locals.var_eta1p12_dn5 / (assign59570_e92971 * assign59570_e92971))), (-(locals.var_eta1p12_dn6 / (assign59570_e92971 * assign59570_e92971))), (-(locals.var_eta1p12_dn7 / (assign59570_e92971 * assign59570_e92971))), (-(locals.var_eta1p12_dn8 / (assign59570_e92971 * assign59570_e92971))), (-(locals.var_eta1p12_dn9 / (assign59570_e92971 * assign59570_e92971))), (-(locals.var_eta1p12_dn10 / (assign59570_e92971 * assign59570_e92971))), (-(locals.var_eta1p12_dn11 / (assign59570_e92971 * assign59570_e92971))), (-(locals.var_eta1p12_dn14 / (assign59570_e92971 * assign59570_e92971))),)
    } else {
        (locals.var_zeta12, locals.var_zeta12_dn0, locals.var_zeta12_dn2, locals.var_zeta12_dn4, locals.var_zeta12_dn5, locals.var_zeta12_dn6, locals.var_zeta12_dn7, locals.var_zeta12_dn8, locals.var_zeta12_dn9, locals.var_zeta12_dn10, locals.var_zeta12_dn11, locals.var_zeta12_dn14,)
    }
};
        locals.var_zeta12 = assign59570_e92974;
        locals.var_zeta12_dn0 = assign59570_e92974_d_n0;
        locals.var_zeta12_dn2 = assign59570_e92974_d_n2;
        locals.var_zeta12_dn4 = assign59570_e92974_d_n4;
        locals.var_zeta12_dn5 = assign59570_e92974_d_n5;
        locals.var_zeta12_dn6 = assign59570_e92974_d_n6;
        locals.var_zeta12_dn7 = assign59570_e92974_d_n7;
        locals.var_zeta12_dn8 = assign59570_e92974_d_n8;
        locals.var_zeta12_dn9 = assign59570_e92974_d_n9;
        locals.var_zeta12_dn10 = assign59570_e92974_d_n10;
        locals.var_zeta12_dn11 = assign59570_e92974_d_n11;
        locals.var_zeta12_dn14 = assign59570_e92974_d_n14;
        locals.var_zeta12_rv = 0.0;

        let (assign59580_e92985, assign59580_e92985_d_n0, assign59580_e92985_d_n2, assign59580_e92985_d_n4, assign59580_e92985_d_n5, assign59580_e92985_d_n6, assign59580_e92985_d_n7, assign59580_e92985_d_n8, assign59580_e92985_d_n9, assign59580_e92985_d_n10, assign59580_e92985_d_n11, assign59580_e92985_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign59580_e92982: f64 = (locals.var_eta1p32 + 1.0);
        let assign59580_e92983: f64 = (1.0 / assign59580_e92982);
        (assign59580_e92983, (-(locals.var_eta1p32_dn0 / (assign59580_e92982 * assign59580_e92982))), (-(locals.var_eta1p32_dn2 / (assign59580_e92982 * assign59580_e92982))), (-(locals.var_eta1p32_dn4 / (assign59580_e92982 * assign59580_e92982))), (-(locals.var_eta1p32_dn5 / (assign59580_e92982 * assign59580_e92982))), (-(locals.var_eta1p32_dn6 / (assign59580_e92982 * assign59580_e92982))), (-(locals.var_eta1p32_dn7 / (assign59580_e92982 * assign59580_e92982))), (-(locals.var_eta1p32_dn8 / (assign59580_e92982 * assign59580_e92982))), (-(locals.var_eta1p32_dn9 / (assign59580_e92982 * assign59580_e92982))), (-(locals.var_eta1p32_dn10 / (assign59580_e92982 * assign59580_e92982))), (-(locals.var_eta1p32_dn11 / (assign59580_e92982 * assign59580_e92982))), (-(locals.var_eta1p32_dn14 / (assign59580_e92982 * assign59580_e92982))),)
    } else {
        (locals.var_zeta32, locals.var_zeta32_dn0, locals.var_zeta32_dn2, locals.var_zeta32_dn4, locals.var_zeta32_dn5, locals.var_zeta32_dn6, locals.var_zeta32_dn7, locals.var_zeta32_dn8, locals.var_zeta32_dn9, locals.var_zeta32_dn10, locals.var_zeta32_dn11, locals.var_zeta32_dn14,)
    }
};
        locals.var_zeta32 = assign59580_e92985;
        locals.var_zeta32_dn0 = assign59580_e92985_d_n0;
        locals.var_zeta32_dn2 = assign59580_e92985_d_n2;
        locals.var_zeta32_dn4 = assign59580_e92985_d_n4;
        locals.var_zeta32_dn5 = assign59580_e92985_d_n5;
        locals.var_zeta32_dn6 = assign59580_e92985_d_n6;
        locals.var_zeta32_dn7 = assign59580_e92985_d_n7;
        locals.var_zeta32_dn8 = assign59580_e92985_d_n8;
        locals.var_zeta32_dn9 = assign59580_e92985_d_n9;
        locals.var_zeta32_dn10 = assign59580_e92985_d_n10;
        locals.var_zeta32_dn11 = assign59580_e92985_d_n11;
        locals.var_zeta32_dn14 = assign59580_e92985_d_n14;
        locals.var_zeta32_rv = 0.0;

        let (assign59590_e92996, assign59590_e92996_d_n0, assign59590_e92996_d_n2, assign59590_e92996_d_n4, assign59590_e92996_d_n5, assign59590_e92996_d_n6, assign59590_e92996_d_n7, assign59590_e92996_d_n8, assign59590_e92996_d_n9, assign59590_e92996_d_n10, assign59590_e92996_d_n11, assign59590_e92996_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign59590_e92993: f64 = (locals.var_eta1p52 + 1.0);
        let assign59590_e92994: f64 = (1.0 / assign59590_e92993);
        (assign59590_e92994, (-(locals.var_eta1p52_dn0 / (assign59590_e92993 * assign59590_e92993))), (-(locals.var_eta1p52_dn2 / (assign59590_e92993 * assign59590_e92993))), (-(locals.var_eta1p52_dn4 / (assign59590_e92993 * assign59590_e92993))), (-(locals.var_eta1p52_dn5 / (assign59590_e92993 * assign59590_e92993))), (-(locals.var_eta1p52_dn6 / (assign59590_e92993 * assign59590_e92993))), (-(locals.var_eta1p52_dn7 / (assign59590_e92993 * assign59590_e92993))), (-(locals.var_eta1p52_dn8 / (assign59590_e92993 * assign59590_e92993))), (-(locals.var_eta1p52_dn9 / (assign59590_e92993 * assign59590_e92993))), (-(locals.var_eta1p52_dn10 / (assign59590_e92993 * assign59590_e92993))), (-(locals.var_eta1p52_dn11 / (assign59590_e92993 * assign59590_e92993))), (-(locals.var_eta1p52_dn14 / (assign59590_e92993 * assign59590_e92993))),)
    } else {
        (locals.var_zeta52, locals.var_zeta52_dn0, locals.var_zeta52_dn2, locals.var_zeta52_dn4, locals.var_zeta52_dn5, locals.var_zeta52_dn6, locals.var_zeta52_dn7, locals.var_zeta52_dn8, locals.var_zeta52_dn9, locals.var_zeta52_dn10, locals.var_zeta52_dn11, locals.var_zeta52_dn14,)
    }
};
        locals.var_zeta52 = assign59590_e92996;
        locals.var_zeta52_dn0 = assign59590_e92996_d_n0;
        locals.var_zeta52_dn2 = assign59590_e92996_d_n2;
        locals.var_zeta52_dn4 = assign59590_e92996_d_n4;
        locals.var_zeta52_dn5 = assign59590_e92996_d_n5;
        locals.var_zeta52_dn6 = assign59590_e92996_d_n6;
        locals.var_zeta52_dn7 = assign59590_e92996_d_n7;
        locals.var_zeta52_dn8 = assign59590_e92996_d_n8;
        locals.var_zeta52_dn9 = assign59590_e92996_d_n9;
        locals.var_zeta52_dn10 = assign59590_e92996_d_n10;
        locals.var_zeta52_dn11 = assign59590_e92996_d_n11;
        locals.var_zeta52_dn14 = assign59590_e92996_d_n14;
        locals.var_zeta52_rv = 0.0;

        let (assign59600_e93005, assign59600_e93005_d_n0, assign59600_e93005_d_n2, assign59600_e93005_d_n4, assign59600_e93005_d_n5, assign59600_e93005_d_n6, assign59600_e93005_d_n7, assign59600_e93005_d_n8, assign59600_e93005_d_n9, assign59600_e93005_d_n10, assign59600_e93005_d_n11, assign59600_e93005_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign59600_e93003: f64 = (locals.var_zeta12 / locals.var_xi0p12);
        (assign59600_e93003, (((locals.var_zeta12_dn0 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn0)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn2 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn2)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn4 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn4)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn5 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn5)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn6 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn6)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn7 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn7)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn8 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn8)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn9 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn9)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn10 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn10)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn11 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn11)) / (locals.var_xi0p12 * locals.var_xi0p12)), (((locals.var_zeta12_dn14 * locals.var_xi0p12) - (locals.var_zeta12 * locals.var_xi0p12_dn14)) / (locals.var_xi0p12 * locals.var_xi0p12)),)
    } else {
        (locals.var_f00, locals.var_f00_dn0, locals.var_f00_dn2, locals.var_f00_dn4, locals.var_f00_dn5, locals.var_f00_dn6, locals.var_f00_dn7, locals.var_f00_dn8, locals.var_f00_dn9, locals.var_f00_dn10, locals.var_f00_dn11, locals.var_f00_dn14,)
    }
};
        locals.var_f00 = assign59600_e93005;
        locals.var_f00_dn0 = assign59600_e93005_d_n0;
        locals.var_f00_dn2 = assign59600_e93005_d_n2;
        locals.var_f00_dn4 = assign59600_e93005_d_n4;
        locals.var_f00_dn5 = assign59600_e93005_d_n5;
        locals.var_f00_dn6 = assign59600_e93005_d_n6;
        locals.var_f00_dn7 = assign59600_e93005_d_n7;
        locals.var_f00_dn8 = assign59600_e93005_d_n8;
        locals.var_f00_dn9 = assign59600_e93005_d_n9;
        locals.var_f00_dn10 = assign59600_e93005_d_n10;
        locals.var_f00_dn11 = assign59600_e93005_d_n11;
        locals.var_f00_dn14 = assign59600_e93005_d_n14;
        locals.var_f00_rv = 0.0;

        let (assign59610_e93018, assign59610_e93018_d_n0, assign59610_e93018_d_n2, assign59610_e93018_d_n4, assign59610_e93018_d_n5, assign59610_e93018_d_n6, assign59610_e93018_d_n7, assign59610_e93018_d_n8, assign59610_e93018_d_n9, assign59610_e93018_d_n10, assign59610_e93018_d_n11, assign59610_e93018_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign59610_e93014: f64 = (3.0 + locals.var_eta);
        let assign59610_e93015: f64 = (locals.var_eta * assign59610_e93014);
        let assign59610_e93016: f64 = (3.0 + assign59610_e93015);
        (assign59610_e93016, ((locals.var_eta_dn0 * assign59610_e93014) + (locals.var_eta * locals.var_eta_dn0)), ((locals.var_eta_dn2 * assign59610_e93014) + (locals.var_eta * locals.var_eta_dn2)), ((locals.var_eta_dn4 * assign59610_e93014) + (locals.var_eta * locals.var_eta_dn4)), ((locals.var_eta_dn5 * assign59610_e93014) + (locals.var_eta * locals.var_eta_dn5)), ((locals.var_eta_dn6 * assign59610_e93014) + (locals.var_eta * locals.var_eta_dn6)), ((locals.var_eta_dn7 * assign59610_e93014) + (locals.var_eta * locals.var_eta_dn7)), ((locals.var_eta_dn8 * assign59610_e93014) + (locals.var_eta * locals.var_eta_dn8)), ((locals.var_eta_dn9 * assign59610_e93014) + (locals.var_eta * locals.var_eta_dn9)), ((locals.var_eta_dn10 * assign59610_e93014) + (locals.var_eta * locals.var_eta_dn10)), ((locals.var_eta_dn11 * assign59610_e93014) + (locals.var_eta * locals.var_eta_dn11)), ((locals.var_eta_dn14 * assign59610_e93014) + (locals.var_eta * locals.var_eta_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign59610_e93018;
        locals.var_t1_dn0 = assign59610_e93018_d_n0;
        locals.var_t1_dn2 = assign59610_e93018_d_n2;
        locals.var_t1_dn4 = assign59610_e93018_d_n4;
        locals.var_t1_dn5 = assign59610_e93018_d_n5;
        locals.var_t1_dn6 = assign59610_e93018_d_n6;
        locals.var_t1_dn7 = assign59610_e93018_d_n7;
        locals.var_t1_dn8 = assign59610_e93018_d_n8;
        locals.var_t1_dn9 = assign59610_e93018_d_n9;
        locals.var_t1_dn10 = assign59610_e93018_d_n10;
        locals.var_t1_dn11 = assign59610_e93018_d_n11;
        locals.var_t1_dn14 = assign59610_e93018_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign59620_e93031, assign59620_e93031_d_n0, assign59620_e93031_d_n2, assign59620_e93031_d_n4, assign59620_e93031_d_n5, assign59620_e93031_d_n6, assign59620_e93031_d_n7, assign59620_e93031_d_n8, assign59620_e93031_d_n9, assign59620_e93031_d_n10, assign59620_e93031_d_n11, assign59620_e93031_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign59620_e93025: f64 = (0.6666666666666667 * locals.var_xi0p12);
        let assign59620_e93027: f64 = (assign59620_e93025 * locals.var_zeta32);
        let assign59620_e93029: f64 = (assign59620_e93027 * locals.var_t1);
        (assign59620_e93029, (((((0.6666666666666667 * locals.var_xi0p12_dn0) * locals.var_zeta32) + (assign59620_e93025 * locals.var_zeta32_dn0)) * locals.var_t1) + (assign59620_e93027 * locals.var_t1_dn0)), (((((0.6666666666666667 * locals.var_xi0p12_dn2) * locals.var_zeta32) + (assign59620_e93025 * locals.var_zeta32_dn2)) * locals.var_t1) + (assign59620_e93027 * locals.var_t1_dn2)), (((((0.6666666666666667 * locals.var_xi0p12_dn4) * locals.var_zeta32) + (assign59620_e93025 * locals.var_zeta32_dn4)) * locals.var_t1) + (assign59620_e93027 * locals.var_t1_dn4)), (((((0.6666666666666667 * locals.var_xi0p12_dn5) * locals.var_zeta32) + (assign59620_e93025 * locals.var_zeta32_dn5)) * locals.var_t1) + (assign59620_e93027 * locals.var_t1_dn5)), (((((0.6666666666666667 * locals.var_xi0p12_dn6) * locals.var_zeta32) + (assign59620_e93025 * locals.var_zeta32_dn6)) * locals.var_t1) + (assign59620_e93027 * locals.var_t1_dn6)), (((((0.6666666666666667 * locals.var_xi0p12_dn7) * locals.var_zeta32) + (assign59620_e93025 * locals.var_zeta32_dn7)) * locals.var_t1) + (assign59620_e93027 * locals.var_t1_dn7)), (((((0.6666666666666667 * locals.var_xi0p12_dn8) * locals.var_zeta32) + (assign59620_e93025 * locals.var_zeta32_dn8)) * locals.var_t1) + (assign59620_e93027 * locals.var_t1_dn8)), (((((0.6666666666666667 * locals.var_xi0p12_dn9) * locals.var_zeta32) + (assign59620_e93025 * locals.var_zeta32_dn9)) * locals.var_t1) + (assign59620_e93027 * locals.var_t1_dn9)), (((((0.6666666666666667 * locals.var_xi0p12_dn10) * locals.var_zeta32) + (assign59620_e93025 * locals.var_zeta32_dn10)) * locals.var_t1) + (assign59620_e93027 * locals.var_t1_dn10)), (((((0.6666666666666667 * locals.var_xi0p12_dn11) * locals.var_zeta32) + (assign59620_e93025 * locals.var_zeta32_dn11)) * locals.var_t1) + (assign59620_e93027 * locals.var_t1_dn11)), (((((0.6666666666666667 * locals.var_xi0p12_dn14) * locals.var_zeta32) + (assign59620_e93025 * locals.var_zeta32_dn14)) * locals.var_t1) + (assign59620_e93027 * locals.var_t1_dn14)),)
    } else {
        (locals.var_f10, locals.var_f10_dn0, locals.var_f10_dn2, locals.var_f10_dn4, locals.var_f10_dn5, locals.var_f10_dn6, locals.var_f10_dn7, locals.var_f10_dn8, locals.var_f10_dn9, locals.var_f10_dn10, locals.var_f10_dn11, locals.var_f10_dn14,)
    }
};
        locals.var_f10 = assign59620_e93031;
        locals.var_f10_dn0 = assign59620_e93031_d_n0;
        locals.var_f10_dn2 = assign59620_e93031_d_n2;
        locals.var_f10_dn4 = assign59620_e93031_d_n4;
        locals.var_f10_dn5 = assign59620_e93031_d_n5;
        locals.var_f10_dn6 = assign59620_e93031_d_n6;
        locals.var_f10_dn7 = assign59620_e93031_d_n7;
        locals.var_f10_dn8 = assign59620_e93031_d_n8;
        locals.var_f10_dn9 = assign59620_e93031_d_n9;
        locals.var_f10_dn10 = assign59620_e93031_d_n10;
        locals.var_f10_dn11 = assign59620_e93031_d_n11;
        locals.var_f10_dn14 = assign59620_e93031_d_n14;
        locals.var_f10_rv = 0.0;

        let (assign59630_e93052, assign59630_e93052_d_n0, assign59630_e93052_d_n2, assign59630_e93052_d_n4, assign59630_e93052_d_n5, assign59630_e93052_d_n6, assign59630_e93052_d_n7, assign59630_e93052_d_n8, assign59630_e93052_d_n9, assign59630_e93052_d_n10, assign59630_e93052_d_n11, assign59630_e93052_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign59630_e93044: f64 = (5.0 + locals.var_eta);
        let assign59630_e93045: f64 = (locals.var_eta * assign59630_e93044);
        let assign59630_e93046: f64 = (10.0 + assign59630_e93045);
        let assign59630_e93047: f64 = (locals.var_eta * assign59630_e93046);
        let assign59630_e93048: f64 = (10.0 + assign59630_e93047);
        let assign59630_e93049: f64 = (locals.var_eta * assign59630_e93048);
        let assign59630_e93050: f64 = (5.0 + assign59630_e93049);
        (assign59630_e93050, ((locals.var_eta_dn0 * assign59630_e93048) + (locals.var_eta * ((locals.var_eta_dn0 * assign59630_e93046) + (locals.var_eta * ((locals.var_eta_dn0 * assign59630_e93044) + (locals.var_eta * locals.var_eta_dn0)))))), ((locals.var_eta_dn2 * assign59630_e93048) + (locals.var_eta * ((locals.var_eta_dn2 * assign59630_e93046) + (locals.var_eta * ((locals.var_eta_dn2 * assign59630_e93044) + (locals.var_eta * locals.var_eta_dn2)))))), ((locals.var_eta_dn4 * assign59630_e93048) + (locals.var_eta * ((locals.var_eta_dn4 * assign59630_e93046) + (locals.var_eta * ((locals.var_eta_dn4 * assign59630_e93044) + (locals.var_eta * locals.var_eta_dn4)))))), ((locals.var_eta_dn5 * assign59630_e93048) + (locals.var_eta * ((locals.var_eta_dn5 * assign59630_e93046) + (locals.var_eta * ((locals.var_eta_dn5 * assign59630_e93044) + (locals.var_eta * locals.var_eta_dn5)))))), ((locals.var_eta_dn6 * assign59630_e93048) + (locals.var_eta * ((locals.var_eta_dn6 * assign59630_e93046) + (locals.var_eta * ((locals.var_eta_dn6 * assign59630_e93044) + (locals.var_eta * locals.var_eta_dn6)))))), ((locals.var_eta_dn7 * assign59630_e93048) + (locals.var_eta * ((locals.var_eta_dn7 * assign59630_e93046) + (locals.var_eta * ((locals.var_eta_dn7 * assign59630_e93044) + (locals.var_eta * locals.var_eta_dn7)))))), ((locals.var_eta_dn8 * assign59630_e93048) + (locals.var_eta * ((locals.var_eta_dn8 * assign59630_e93046) + (locals.var_eta * ((locals.var_eta_dn8 * assign59630_e93044) + (locals.var_eta * locals.var_eta_dn8)))))), ((locals.var_eta_dn9 * assign59630_e93048) + (locals.var_eta * ((locals.var_eta_dn9 * assign59630_e93046) + (locals.var_eta * ((locals.var_eta_dn9 * assign59630_e93044) + (locals.var_eta * locals.var_eta_dn9)))))), ((locals.var_eta_dn10 * assign59630_e93048) + (locals.var_eta * ((locals.var_eta_dn10 * assign59630_e93046) + (locals.var_eta * ((locals.var_eta_dn10 * assign59630_e93044) + (locals.var_eta * locals.var_eta_dn10)))))), ((locals.var_eta_dn11 * assign59630_e93048) + (locals.var_eta * ((locals.var_eta_dn11 * assign59630_e93046) + (locals.var_eta * ((locals.var_eta_dn11 * assign59630_e93044) + (locals.var_eta * locals.var_eta_dn11)))))), ((locals.var_eta_dn14 * assign59630_e93048) + (locals.var_eta * ((locals.var_eta_dn14 * assign59630_e93046) + (locals.var_eta * ((locals.var_eta_dn14 * assign59630_e93044) + (locals.var_eta * locals.var_eta_dn14)))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign59630_e93052;
        locals.var_t1_dn0 = assign59630_e93052_d_n0;
        locals.var_t1_dn2 = assign59630_e93052_d_n2;
        locals.var_t1_dn4 = assign59630_e93052_d_n4;
        locals.var_t1_dn5 = assign59630_e93052_d_n5;
        locals.var_t1_dn6 = assign59630_e93052_d_n6;
        locals.var_t1_dn7 = assign59630_e93052_d_n7;
        locals.var_t1_dn8 = assign59630_e93052_d_n8;
        locals.var_t1_dn9 = assign59630_e93052_d_n9;
        locals.var_t1_dn10 = assign59630_e93052_d_n10;
        locals.var_t1_dn11 = assign59630_e93052_d_n11;
        locals.var_t1_dn14 = assign59630_e93052_d_n14;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_222(
        locals: &mut StampLocals,
    ) {
        let (assign59640_e93069, assign59640_e93069_d_n0, assign59640_e93069_d_n2, assign59640_e93069_d_n4, assign59640_e93069_d_n5, assign59640_e93069_d_n6, assign59640_e93069_d_n7, assign59640_e93069_d_n8, assign59640_e93069_d_n9, assign59640_e93069_d_n10, assign59640_e93069_d_n11, assign59640_e93069_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign59640_e93060: f64 = (15.0 * locals.var_beta);
        let assign59640_e93061: f64 = (4.0 / assign59640_e93060);
        let assign59640_e93063: f64 = (assign59640_e93061 * locals.var_xi0p32);
        let assign59640_e93065: f64 = (assign59640_e93063 * locals.var_zeta52);
        let assign59640_e93067: f64 = (assign59640_e93065 * locals.var_t1);
        (assign59640_e93067, (((((((-((4.0 * (15.0 * locals.var_beta_dn0)) / (assign59640_e93060 * assign59640_e93060))) * locals.var_xi0p32) + (assign59640_e93061 * locals.var_xi0p32_dn0)) * locals.var_zeta52) + (assign59640_e93063 * locals.var_zeta52_dn0)) * locals.var_t1) + (assign59640_e93065 * locals.var_t1_dn0)), (((((((-((4.0 * (15.0 * locals.var_beta_dn2)) / (assign59640_e93060 * assign59640_e93060))) * locals.var_xi0p32) + (assign59640_e93061 * locals.var_xi0p32_dn2)) * locals.var_zeta52) + (assign59640_e93063 * locals.var_zeta52_dn2)) * locals.var_t1) + (assign59640_e93065 * locals.var_t1_dn2)), (((((((-((4.0 * (15.0 * locals.var_beta_dn4)) / (assign59640_e93060 * assign59640_e93060))) * locals.var_xi0p32) + (assign59640_e93061 * locals.var_xi0p32_dn4)) * locals.var_zeta52) + (assign59640_e93063 * locals.var_zeta52_dn4)) * locals.var_t1) + (assign59640_e93065 * locals.var_t1_dn4)), (((((((-((4.0 * (15.0 * locals.var_beta_dn5)) / (assign59640_e93060 * assign59640_e93060))) * locals.var_xi0p32) + (assign59640_e93061 * locals.var_xi0p32_dn5)) * locals.var_zeta52) + (assign59640_e93063 * locals.var_zeta52_dn5)) * locals.var_t1) + (assign59640_e93065 * locals.var_t1_dn5)), (((((((-((4.0 * (15.0 * locals.var_beta_dn6)) / (assign59640_e93060 * assign59640_e93060))) * locals.var_xi0p32) + (assign59640_e93061 * locals.var_xi0p32_dn6)) * locals.var_zeta52) + (assign59640_e93063 * locals.var_zeta52_dn6)) * locals.var_t1) + (assign59640_e93065 * locals.var_t1_dn6)), (((((((-((4.0 * (15.0 * locals.var_beta_dn7)) / (assign59640_e93060 * assign59640_e93060))) * locals.var_xi0p32) + (assign59640_e93061 * locals.var_xi0p32_dn7)) * locals.var_zeta52) + (assign59640_e93063 * locals.var_zeta52_dn7)) * locals.var_t1) + (assign59640_e93065 * locals.var_t1_dn7)), (((((((-((4.0 * (15.0 * locals.var_beta_dn8)) / (assign59640_e93060 * assign59640_e93060))) * locals.var_xi0p32) + (assign59640_e93061 * locals.var_xi0p32_dn8)) * locals.var_zeta52) + (assign59640_e93063 * locals.var_zeta52_dn8)) * locals.var_t1) + (assign59640_e93065 * locals.var_t1_dn8)), (((((((-((4.0 * (15.0 * locals.var_beta_dn9)) / (assign59640_e93060 * assign59640_e93060))) * locals.var_xi0p32) + (assign59640_e93061 * locals.var_xi0p32_dn9)) * locals.var_zeta52) + (assign59640_e93063 * locals.var_zeta52_dn9)) * locals.var_t1) + (assign59640_e93065 * locals.var_t1_dn9)), (((((((-((4.0 * (15.0 * locals.var_beta_dn10)) / (assign59640_e93060 * assign59640_e93060))) * locals.var_xi0p32) + (assign59640_e93061 * locals.var_xi0p32_dn10)) * locals.var_zeta52) + (assign59640_e93063 * locals.var_zeta52_dn10)) * locals.var_t1) + (assign59640_e93065 * locals.var_t1_dn10)), (((((((-((4.0 * (15.0 * locals.var_beta_dn11)) / (assign59640_e93060 * assign59640_e93060))) * locals.var_xi0p32) + (assign59640_e93061 * locals.var_xi0p32_dn11)) * locals.var_zeta52) + (assign59640_e93063 * locals.var_zeta52_dn11)) * locals.var_t1) + (assign59640_e93065 * locals.var_t1_dn11)), (((((((-((4.0 * (15.0 * locals.var_beta_dn14)) / (assign59640_e93060 * assign59640_e93060))) * locals.var_xi0p32) + (assign59640_e93061 * locals.var_xi0p32_dn14)) * locals.var_zeta52) + (assign59640_e93063 * locals.var_zeta52_dn14)) * locals.var_t1) + (assign59640_e93065 * locals.var_t1_dn14)),)
    } else {
        (locals.var_f30, locals.var_f30_dn0, locals.var_f30_dn2, locals.var_f30_dn4, locals.var_f30_dn5, locals.var_f30_dn6, locals.var_f30_dn7, locals.var_f30_dn8, locals.var_f30_dn9, locals.var_f30_dn10, locals.var_f30_dn11, locals.var_f30_dn14,)
    }
};
        locals.var_f30 = assign59640_e93069;
        locals.var_f30_dn0 = assign59640_e93069_d_n0;
        locals.var_f30_dn2 = assign59640_e93069_d_n2;
        locals.var_f30_dn4 = assign59640_e93069_d_n4;
        locals.var_f30_dn5 = assign59640_e93069_d_n5;
        locals.var_f30_dn6 = assign59640_e93069_d_n6;
        locals.var_f30_dn7 = assign59640_e93069_d_n7;
        locals.var_f30_dn8 = assign59640_e93069_d_n8;
        locals.var_f30_dn9 = assign59640_e93069_d_n9;
        locals.var_f30_dn10 = assign59640_e93069_d_n10;
        locals.var_f30_dn11 = assign59640_e93069_d_n11;
        locals.var_f30_dn14 = assign59640_e93069_d_n14;
        locals.var_f30_rv = 0.0;

        let (assign59650_e93086, assign59650_e93086_d_n0, assign59650_e93086_d_n2, assign59650_e93086_d_n4, assign59650_e93086_d_n5, assign59650_e93086_d_n6, assign59650_e93086_d_n7, assign59650_e93086_d_n8, assign59650_e93086_d_n9, assign59650_e93086_d_n10, assign59650_e93086_d_n11, assign59650_e93086_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign59650_e93076: f64 = (locals.var_ps0 * locals.var_f10);
        let assign59650_e93079: f64 = (0.6666666666666667 * locals.var_beta_inv);
        let assign59650_e93081: f64 = (assign59650_e93079 * locals.var_xilp32);
        let assign59650_e93082: f64 = (assign59650_e93076 + assign59650_e93081);
        let assign59650_e93084: f64 = (assign59650_e93082 - locals.var_f30);
        (assign59650_e93084, ((((locals.var_ps0_dn0 * locals.var_f10) + (locals.var_ps0 * locals.var_f10_dn0)) + (((0.6666666666666667 * locals.var_beta_inv_dn0) * locals.var_xilp32) + (assign59650_e93079 * locals.var_xilp32_dn0))) - locals.var_f30_dn0), ((((locals.var_ps0_dn2 * locals.var_f10) + (locals.var_ps0 * locals.var_f10_dn2)) + (((0.6666666666666667 * locals.var_beta_inv_dn2) * locals.var_xilp32) + (assign59650_e93079 * locals.var_xilp32_dn2))) - locals.var_f30_dn2), ((((locals.var_ps0_dn4 * locals.var_f10) + (locals.var_ps0 * locals.var_f10_dn4)) + (((0.6666666666666667 * locals.var_beta_inv_dn4) * locals.var_xilp32) + (assign59650_e93079 * locals.var_xilp32_dn4))) - locals.var_f30_dn4), ((((locals.var_ps0_dn5 * locals.var_f10) + (locals.var_ps0 * locals.var_f10_dn5)) + (((0.6666666666666667 * locals.var_beta_inv_dn5) * locals.var_xilp32) + (assign59650_e93079 * locals.var_xilp32_dn5))) - locals.var_f30_dn5), ((((locals.var_ps0_dn6 * locals.var_f10) + (locals.var_ps0 * locals.var_f10_dn6)) + (((0.6666666666666667 * locals.var_beta_inv_dn6) * locals.var_xilp32) + (assign59650_e93079 * locals.var_xilp32_dn6))) - locals.var_f30_dn6), ((((locals.var_ps0_dn7 * locals.var_f10) + (locals.var_ps0 * locals.var_f10_dn7)) + (((0.6666666666666667 * locals.var_beta_inv_dn7) * locals.var_xilp32) + (assign59650_e93079 * locals.var_xilp32_dn7))) - locals.var_f30_dn7), ((((locals.var_ps0_dn8 * locals.var_f10) + (locals.var_ps0 * locals.var_f10_dn8)) + (((0.6666666666666667 * locals.var_beta_inv_dn8) * locals.var_xilp32) + (assign59650_e93079 * locals.var_xilp32_dn8))) - locals.var_f30_dn8), ((((locals.var_ps0_dn9 * locals.var_f10) + (locals.var_ps0 * locals.var_f10_dn9)) + (((0.6666666666666667 * locals.var_beta_inv_dn9) * locals.var_xilp32) + (assign59650_e93079 * locals.var_xilp32_dn9))) - locals.var_f30_dn9), ((((locals.var_ps0_dn10 * locals.var_f10) + (locals.var_ps0 * locals.var_f10_dn10)) + (((0.6666666666666667 * locals.var_beta_inv_dn10) * locals.var_xilp32) + (assign59650_e93079 * locals.var_xilp32_dn10))) - locals.var_f30_dn10), ((((locals.var_ps0_dn11 * locals.var_f10) + (locals.var_ps0 * locals.var_f10_dn11)) + (((0.6666666666666667 * locals.var_beta_inv_dn11) * locals.var_xilp32) + (assign59650_e93079 * locals.var_xilp32_dn11))) - locals.var_f30_dn11), ((((locals.var_ps0_dn14 * locals.var_f10) + (locals.var_ps0 * locals.var_f10_dn14)) + (((0.6666666666666667 * locals.var_beta_inv_dn14) * locals.var_xilp32) + (assign59650_e93079 * locals.var_xilp32_dn14))) - locals.var_f30_dn14),)
    } else {
        (locals.var_f11, locals.var_f11_dn0, locals.var_f11_dn2, locals.var_f11_dn4, locals.var_f11_dn5, locals.var_f11_dn6, locals.var_f11_dn7, locals.var_f11_dn8, locals.var_f11_dn9, locals.var_f11_dn10, locals.var_f11_dn11, locals.var_f11_dn14,)
    }
};
        locals.var_f11 = assign59650_e93086;
        locals.var_f11_dn0 = assign59650_e93086_d_n0;
        locals.var_f11_dn2 = assign59650_e93086_d_n2;
        locals.var_f11_dn4 = assign59650_e93086_d_n4;
        locals.var_f11_dn5 = assign59650_e93086_d_n5;
        locals.var_f11_dn6 = assign59650_e93086_d_n6;
        locals.var_f11_dn7 = assign59650_e93086_d_n7;
        locals.var_f11_dn8 = assign59650_e93086_d_n8;
        locals.var_f11_dn9 = assign59650_e93086_d_n9;
        locals.var_f11_dn10 = assign59650_e93086_d_n10;
        locals.var_f11_dn11 = assign59650_e93086_d_n11;
        locals.var_f11_dn14 = assign59650_e93086_d_n14;
        locals.var_f11_rv = 0.0;

        let (assign59660_e93103, assign59660_e93103_d_n0, assign59660_e93103_d_n2, assign59660_e93103_d_n4, assign59660_e93103_d_n5, assign59660_e93103_d_n6, assign59660_e93103_d_n7, assign59660_e93103_d_n8, assign59660_e93103_d_n9, assign59660_e93103_d_n10, assign59660_e93103_d_n11, assign59660_e93103_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign59660_e93093: f64 = (locals.var_vgp + locals.var_beta_inv);
        let assign59660_e93097: f64 = (2.0 * locals.var_ps0);
        let assign59660_e93099: f64 = (assign59660_e93097 + locals.var_pds);
        let assign59660_e93100: f64 = (0.5 * assign59660_e93099);
        let assign59660_e93101: f64 = (assign59660_e93093 - assign59660_e93100);
        (assign59660_e93101, ((locals.var_vgp_dn0 + locals.var_beta_inv_dn0) - (0.5 * ((2.0 * locals.var_ps0_dn0) + locals.var_pds_dn0))), ((locals.var_vgp_dn2 + locals.var_beta_inv_dn2) - (0.5 * ((2.0 * locals.var_ps0_dn2) + locals.var_pds_dn2))), ((locals.var_vgp_dn4 + locals.var_beta_inv_dn4) - (0.5 * ((2.0 * locals.var_ps0_dn4) + locals.var_pds_dn4))), ((locals.var_vgp_dn5 + locals.var_beta_inv_dn5) - (0.5 * ((2.0 * locals.var_ps0_dn5) + locals.var_pds_dn5))), ((locals.var_vgp_dn6 + locals.var_beta_inv_dn6) - (0.5 * ((2.0 * locals.var_ps0_dn6) + locals.var_pds_dn6))), ((locals.var_vgp_dn7 + locals.var_beta_inv_dn7) - (0.5 * ((2.0 * locals.var_ps0_dn7) + locals.var_pds_dn7))), ((locals.var_vgp_dn8 + locals.var_beta_inv_dn8) - (0.5 * ((2.0 * locals.var_ps0_dn8) + locals.var_pds_dn8))), ((locals.var_vgp_dn9 + locals.var_beta_inv_dn9) - (0.5 * ((2.0 * locals.var_ps0_dn9) + locals.var_pds_dn9))), ((locals.var_vgp_dn10 + locals.var_beta_inv_dn10) - (0.5 * ((2.0 * locals.var_ps0_dn10) + locals.var_pds_dn10))), ((locals.var_vgp_dn11 + locals.var_beta_inv_dn11) - (0.5 * ((2.0 * locals.var_ps0_dn11) + locals.var_pds_dn11))), ((locals.var_vgp_dn14 + locals.var_beta_inv_dn14) - (0.5 * ((2.0 * locals.var_ps0_dn14) + locals.var_pds_dn14))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign59660_e93103;
        locals.var_t1_dn0 = assign59660_e93103_d_n0;
        locals.var_t1_dn2 = assign59660_e93103_d_n2;
        locals.var_t1_dn4 = assign59660_e93103_d_n4;
        locals.var_t1_dn5 = assign59660_e93103_d_n5;
        locals.var_t1_dn6 = assign59660_e93103_d_n6;
        locals.var_t1_dn7 = assign59660_e93103_d_n7;
        locals.var_t1_dn8 = assign59660_e93103_d_n8;
        locals.var_t1_dn9 = assign59660_e93103_d_n9;
        locals.var_t1_dn10 = assign59660_e93103_d_n10;
        locals.var_t1_dn11 = assign59660_e93103_d_n11;
        locals.var_t1_dn14 = assign59660_e93103_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign59670_e93113, assign59670_e93113_d_n0, assign59670_e93113_d_n2, assign59670_e93113_d_n4, assign59670_e93113_d_n5, assign59670_e93113_d_n6, assign59670_e93113_d_n7, assign59670_e93113_d_n8, assign59670_e93113_d_n9, assign59670_e93113_d_n10, assign59670_e93113_d_n11, assign59670_e93113_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign59670_e93109: f64 = (-locals.var_f10);
        let assign59670_e93111: f64 = (assign59670_e93109 + locals.var_f00);
        (assign59670_e93111, ((-locals.var_f10_dn0) + locals.var_f00_dn0), ((-locals.var_f10_dn2) + locals.var_f00_dn2), ((-locals.var_f10_dn4) + locals.var_f00_dn4), ((-locals.var_f10_dn5) + locals.var_f00_dn5), ((-locals.var_f10_dn6) + locals.var_f00_dn6), ((-locals.var_f10_dn7) + locals.var_f00_dn7), ((-locals.var_f10_dn8) + locals.var_f00_dn8), ((-locals.var_f10_dn9) + locals.var_f00_dn9), ((-locals.var_f10_dn10) + locals.var_f00_dn10), ((-locals.var_f10_dn11) + locals.var_f00_dn11), ((-locals.var_f10_dn14) + locals.var_f00_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign59670_e93113;
        locals.var_t2_dn0 = assign59670_e93113_d_n0;
        locals.var_t2_dn2 = assign59670_e93113_d_n2;
        locals.var_t2_dn4 = assign59670_e93113_d_n4;
        locals.var_t2_dn5 = assign59670_e93113_d_n5;
        locals.var_t2_dn6 = assign59670_e93113_d_n6;
        locals.var_t2_dn7 = assign59670_e93113_d_n7;
        locals.var_t2_dn8 = assign59670_e93113_d_n8;
        locals.var_t2_dn9 = assign59670_e93113_d_n9;
        locals.var_t2_dn10 = assign59670_e93113_d_n10;
        locals.var_t2_dn11 = assign59670_e93113_d_n11;
        locals.var_t2_dn14 = assign59670_e93113_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign59680_e93122, assign59680_e93122_d_n0, assign59680_e93122_d_n2, assign59680_e93122_d_n4, assign59680_e93122_d_n5, assign59680_e93122_d_n6, assign59680_e93122_d_n7, assign59680_e93122_d_n8, assign59680_e93122_d_n9, assign59680_e93122_d_n10, assign59680_e93122_d_n11, assign59680_e93122_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign59680_e93120: f64 = (locals.var_beta * locals.var_cox);
        (assign59680_e93120, ((locals.var_beta_dn0 * locals.var_cox) + (locals.var_beta * locals.var_cox_dn0)), ((locals.var_beta_dn2 * locals.var_cox) + (locals.var_beta * locals.var_cox_dn2)), ((locals.var_beta_dn4 * locals.var_cox) + (locals.var_beta * locals.var_cox_dn4)), ((locals.var_beta_dn5 * locals.var_cox) + (locals.var_beta * locals.var_cox_dn5)), ((locals.var_beta_dn6 * locals.var_cox) + (locals.var_beta * locals.var_cox_dn6)), ((locals.var_beta_dn7 * locals.var_cox) + (locals.var_beta * locals.var_cox_dn7)), ((locals.var_beta_dn8 * locals.var_cox) + (locals.var_beta * locals.var_cox_dn8)), ((locals.var_beta_dn9 * locals.var_cox) + (locals.var_beta * locals.var_cox_dn9)), ((locals.var_beta_dn10 * locals.var_cox) + (locals.var_beta * locals.var_cox_dn10)), ((locals.var_beta_dn11 * locals.var_cox) + (locals.var_beta * locals.var_cox_dn11)), ((locals.var_beta_dn14 * locals.var_cox) + (locals.var_beta * locals.var_cox_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign59680_e93122;
        locals.var_t3_dn0 = assign59680_e93122_d_n0;
        locals.var_t3_dn2 = assign59680_e93122_d_n2;
        locals.var_t3_dn4 = assign59680_e93122_d_n4;
        locals.var_t3_dn5 = assign59680_e93122_d_n5;
        locals.var_t3_dn6 = assign59680_e93122_d_n6;
        locals.var_t3_dn7 = assign59680_e93122_d_n7;
        locals.var_t3_dn8 = assign59680_e93122_d_n8;
        locals.var_t3_dn9 = assign59680_e93122_d_n9;
        locals.var_t3_dn10 = assign59680_e93122_d_n10;
        locals.var_t3_dn11 = assign59680_e93122_d_n11;
        locals.var_t3_dn14 = assign59680_e93122_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign59690_e93131, assign59690_e93131_d_n0, assign59690_e93131_d_n2, assign59690_e93131_d_n4, assign59690_e93131_d_n5, assign59690_e93131_d_n6, assign59690_e93131_d_n7, assign59690_e93131_d_n8, assign59690_e93131_d_n9, assign59690_e93131_d_n10, assign59690_e93131_d_n11, assign59690_e93131_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign59690_e93129: f64 = (locals.var_beta * locals.var_cnst0);
        (assign59690_e93129, ((locals.var_beta_dn0 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn0)), ((locals.var_beta_dn2 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn2)), ((locals.var_beta_dn4 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn4)), ((locals.var_beta_dn5 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn5)), ((locals.var_beta_dn6 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn6)), ((locals.var_beta_dn7 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn7)), ((locals.var_beta_dn8 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn8)), ((locals.var_beta_dn9 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn9)), ((locals.var_beta_dn10 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn10)), ((locals.var_beta_dn11 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn11)), ((locals.var_beta_dn14 * locals.var_cnst0) + (locals.var_beta * locals.var_cnst0_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign59690_e93131;
        locals.var_t4_dn0 = assign59690_e93131_d_n0;
        locals.var_t4_dn2 = assign59690_e93131_d_n2;
        locals.var_t4_dn4 = assign59690_e93131_d_n4;
        locals.var_t4_dn5 = assign59690_e93131_d_n5;
        locals.var_t4_dn6 = assign59690_e93131_d_n6;
        locals.var_t4_dn7 = assign59690_e93131_d_n7;
        locals.var_t4_dn8 = assign59690_e93131_d_n8;
        locals.var_t4_dn9 = assign59690_e93131_d_n9;
        locals.var_t4_dn10 = assign59690_e93131_d_n10;
        locals.var_t4_dn11 = assign59690_e93131_d_n11;
        locals.var_t4_dn14 = assign59690_e93131_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign59700_e93144, assign59700_e93144_d_n0, assign59700_e93144_d_n2, assign59700_e93144_d_n4, assign59700_e93144_d_n5, assign59700_e93144_d_n6, assign59700_e93144_d_n7, assign59700_e93144_d_n8, assign59700_e93144_d_n9, assign59700_e93144_d_n10, assign59700_e93144_d_n11, assign59700_e93144_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign59700_e93138: f64 = (locals.var_t3 * locals.var_t1);
        let assign59700_e93141: f64 = (locals.var_t4 * locals.var_t2);
        let assign59700_e93142: f64 = (assign59700_e93138 + assign59700_e93141);
        (assign59700_e93142, (((locals.var_t3_dn0 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn0)) + ((locals.var_t4_dn0 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn0))), (((locals.var_t3_dn2 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn2)) + ((locals.var_t4_dn2 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn2))), (((locals.var_t3_dn4 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn4)) + ((locals.var_t4_dn4 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn4))), (((locals.var_t3_dn5 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn5)) + ((locals.var_t4_dn5 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn5))), (((locals.var_t3_dn6 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn6)) + ((locals.var_t4_dn6 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn6))), (((locals.var_t3_dn7 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn7)) + ((locals.var_t4_dn7 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn7))), (((locals.var_t3_dn8 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn8)) + ((locals.var_t4_dn8 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn8))), (((locals.var_t3_dn9 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn9)) + ((locals.var_t4_dn9 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn9))), (((locals.var_t3_dn10 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn10)) + ((locals.var_t4_dn10 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn10))), (((locals.var_t3_dn11 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn11)) + ((locals.var_t4_dn11 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn11))), (((locals.var_t3_dn14 * locals.var_t1) + (locals.var_t3 * locals.var_t1_dn14)) + ((locals.var_t4_dn14 * locals.var_t2) + (locals.var_t4 * locals.var_t2_dn14))),)
    } else {
        (locals.var_fdd, locals.var_fdd_dn0, locals.var_fdd_dn2, locals.var_fdd_dn4, locals.var_fdd_dn5, locals.var_fdd_dn6, locals.var_fdd_dn7, locals.var_fdd_dn8, locals.var_fdd_dn9, locals.var_fdd_dn10, locals.var_fdd_dn11, locals.var_fdd_dn14,)
    }
};
        locals.var_fdd = assign59700_e93144;
        locals.var_fdd_dn0 = assign59700_e93144_d_n0;
        locals.var_fdd_dn2 = assign59700_e93144_d_n2;
        locals.var_fdd_dn4 = assign59700_e93144_d_n4;
        locals.var_fdd_dn5 = assign59700_e93144_d_n5;
        locals.var_fdd_dn6 = assign59700_e93144_d_n6;
        locals.var_fdd_dn7 = assign59700_e93144_d_n7;
        locals.var_fdd_dn8 = assign59700_e93144_d_n8;
        locals.var_fdd_dn9 = assign59700_e93144_d_n9;
        locals.var_fdd_dn10 = assign59700_e93144_d_n10;
        locals.var_fdd_dn11 = assign59700_e93144_d_n11;
        locals.var_fdd_dn14 = assign59700_e93144_d_n14;
        locals.var_fdd_rv = 0.0;

        let (assign59710_e93153, assign59710_e93153_d_n0, assign59710_e93153_d_n2, assign59710_e93153_d_n4, assign59710_e93153_d_n5, assign59710_e93153_d_n6, assign59710_e93153_d_n7, assign59710_e93153_d_n8, assign59710_e93153_d_n9, assign59710_e93153_d_n10, assign59710_e93153_d_n11, assign59710_e93153_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign59710_e93151: f64 = (locals.var_pds * locals.var_fdd);
        (assign59710_e93151, ((locals.var_pds_dn0 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn0)), ((locals.var_pds_dn2 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn2)), ((locals.var_pds_dn4 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn4)), ((locals.var_pds_dn5 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn5)), ((locals.var_pds_dn6 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn6)), ((locals.var_pds_dn7 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn7)), ((locals.var_pds_dn8 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn8)), ((locals.var_pds_dn9 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn9)), ((locals.var_pds_dn10 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn10)), ((locals.var_pds_dn11 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn11)), ((locals.var_pds_dn14 * locals.var_fdd) + (locals.var_pds * locals.var_fdd_dn14)),)
    } else {
        (locals.var_idd, locals.var_idd_dn0, locals.var_idd_dn2, locals.var_idd_dn4, locals.var_idd_dn5, locals.var_idd_dn6, locals.var_idd_dn7, locals.var_idd_dn8, locals.var_idd_dn9, locals.var_idd_dn10, locals.var_idd_dn11, locals.var_idd_dn14,)
    }
};
        locals.var_idd = assign59710_e93153;
        locals.var_idd_dn0 = assign59710_e93153_d_n0;
        locals.var_idd_dn2 = assign59710_e93153_d_n2;
        locals.var_idd_dn4 = assign59710_e93153_d_n4;
        locals.var_idd_dn5 = assign59710_e93153_d_n5;
        locals.var_idd_dn6 = assign59710_e93153_d_n6;
        locals.var_idd_dn7 = assign59710_e93153_d_n7;
        locals.var_idd_dn8 = assign59710_e93153_d_n8;
        locals.var_idd_dn9 = assign59710_e93153_d_n9;
        locals.var_idd_dn10 = assign59710_e93153_d_n10;
        locals.var_idd_dn11 = assign59710_e93153_d_n11;
        locals.var_idd_dn14 = assign59710_e93153_d_n14;
        locals.var_idd_rv = 0.0;

        let assign59720_e93156: f64 = if locals.var_flg_zone == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1464 = assign59720_e93156;
        locals.var_guard1464_rv = 0.0;

        let (assign59730_e93165,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1464 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_start_of_mobility,)
    }
};
        locals.var_start_of_mobility = assign59730_e93165;
        locals.var_start_of_mobility_rv = 0.0;

        let assign59740_e93168: f64 = if locals.var_start_of_mobility == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1465 = assign59740_e93168;
        locals.var_guard1465_rv = 0.0;

        let assign59750_e93172: f64 = (10.0 * 2.220446049250313e-16);
        let assign59750_e93177: f64 = (10.0 * 2.220446049250313e-16);
        let assign59750_e93179: f64 = if ((locals.var_uc_clm2 < assign59750_e93172) && (locals.var_uc_clm3 < assign59750_e93177)) { 1.0 } else { 0.0 };
        locals.var_guard1466 = assign59750_e93179;
        locals.var_guard1466_rv = 0.0;

        let (assign59760_e93190, assign59760_e93190_d_n0, assign59760_e93190_d_n2, assign59760_e93190_d_n4, assign59760_e93190_d_n5, assign59760_e93190_d_n6, assign59760_e93190_d_n7, assign59760_e93190_d_n8, assign59760_e93190_d_n9, assign59760_e93190_d_n10, assign59760_e93190_d_n11, assign59760_e93190_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn4, locals.var_lred_dn5, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn8, locals.var_lred_dn9, locals.var_lred_dn10, locals.var_lred_dn11, locals.var_lred_dn14,)
    }
};
        locals.var_lred = assign59760_e93190;
        locals.var_lred_dn0 = assign59760_e93190_d_n0;
        locals.var_lred_dn2 = assign59760_e93190_d_n2;
        locals.var_lred_dn4 = assign59760_e93190_d_n4;
        locals.var_lred_dn5 = assign59760_e93190_d_n5;
        locals.var_lred_dn6 = assign59760_e93190_d_n6;
        locals.var_lred_dn7 = assign59760_e93190_d_n7;
        locals.var_lred_dn8 = assign59760_e93190_d_n8;
        locals.var_lred_dn9 = assign59760_e93190_d_n9;
        locals.var_lred_dn10 = assign59760_e93190_d_n10;
        locals.var_lred_dn11 = assign59760_e93190_d_n11;
        locals.var_lred_dn14 = assign59760_e93190_d_n14;
        locals.var_lred_rv = 0.0;

        let (assign59770_e93201, assign59770_e93201_d_n0, assign59770_e93201_d_n2, assign59770_e93201_d_n4, assign59770_e93201_d_n5, assign59770_e93201_d_n6, assign59770_e93201_d_n7, assign59770_e93201_d_n8, assign59770_e93201_d_n9, assign59770_e93201_d_n10, assign59770_e93201_d_n11, assign59770_e93201_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 != 0.0)) {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn4, locals.var_psl_dn5, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn8, locals.var_psl_dn9, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn14,)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn14,)
    }
};
        locals.var_psdl = assign59770_e93201;
        locals.var_psdl_dn0 = assign59770_e93201_d_n0;
        locals.var_psdl_dn2 = assign59770_e93201_d_n2;
        locals.var_psdl_dn4 = assign59770_e93201_d_n4;
        locals.var_psdl_dn5 = assign59770_e93201_d_n5;
        locals.var_psdl_dn6 = assign59770_e93201_d_n6;
        locals.var_psdl_dn7 = assign59770_e93201_d_n7;
        locals.var_psdl_dn8 = assign59770_e93201_d_n8;
        locals.var_psdl_dn9 = assign59770_e93201_d_n9;
        locals.var_psdl_dn10 = assign59770_e93201_d_n10;
        locals.var_psdl_dn11 = assign59770_e93201_d_n11;
        locals.var_psdl_dn14 = assign59770_e93201_d_n14;
        locals.var_psdl_rv = 0.0;

        let assign59780_e93205: f64 = (locals.var_ps0 + locals.var_vds);
        let assign59780_e93208: f64 = (10.0 * 2.220446049250313e-16);
        let assign59780_e93209: f64 = (assign59780_e93205 - assign59780_e93208);
        let assign59780_e93212: f64 = (10.0 * 2.220446049250313e-16);
        let assign59780_e93213: f64 = (assign59780_e93209 - assign59780_e93212);
        let assign59780_e93217: f64 = (10.0 * 2.220446049250313e-16);
        let assign59780_e93220: f64 = if ((locals.var_psdl > assign59780_e93213) && (assign59780_e93217 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1467 = assign59780_e93220;
        locals.var_guard1467_rv = 0.0;

        let (assign59790_e93245, assign59790_e93245_d_n0, assign59790_e93245_d_n2, assign59790_e93245_d_n4, assign59790_e93245_d_n5, assign59790_e93245_d_n6, assign59790_e93245_d_n7, assign59790_e93245_d_n8, assign59790_e93245_d_n9, assign59790_e93245_d_n10, assign59790_e93245_d_n11, assign59790_e93245_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 != 0.0)) && (locals.var_guard1467 != 0.0)) {
        let assign59790_e93234: f64 = (locals.var_ps0 + locals.var_vds);
        let assign59790_e93237: f64 = (10.0 * 2.220446049250313e-16);
        let assign59790_e93238: f64 = (assign59790_e93234 - assign59790_e93237);
        let assign59790_e93239: f64 = (locals.var_psdl - assign59790_e93238);
        let assign59790_e93242: f64 = (10.0 * 2.220446049250313e-16);
        let assign59790_e93243: f64 = (assign59790_e93239 + assign59790_e93242);
        (assign59790_e93243, (locals.var_psdl_dn0 - (locals.var_ps0_dn0 + locals.var_vds_dn0)), (locals.var_psdl_dn2 - (locals.var_ps0_dn2 + locals.var_vds_dn2)), (locals.var_psdl_dn4 - (locals.var_ps0_dn4 + locals.var_vds_dn4)), (locals.var_psdl_dn5 - (locals.var_ps0_dn5 + locals.var_vds_dn5)), (locals.var_psdl_dn6 - (locals.var_ps0_dn6 + locals.var_vds_dn6)), (locals.var_psdl_dn7 - (locals.var_ps0_dn7 + locals.var_vds_dn7)), (locals.var_psdl_dn8 - (locals.var_ps0_dn8 + locals.var_vds_dn8)), (locals.var_psdl_dn9 - (locals.var_ps0_dn9 + locals.var_vds_dn9)), (locals.var_psdl_dn10 - (locals.var_ps0_dn10 + locals.var_vds_dn10)), (locals.var_psdl_dn11 - (locals.var_ps0_dn11 + locals.var_vds_dn11)), (locals.var_psdl_dn14 - (locals.var_ps0_dn14 + locals.var_vds_dn14)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign59790_e93245;
        locals.var_tmf1_dn0 = assign59790_e93245_d_n0;
        locals.var_tmf1_dn2 = assign59790_e93245_d_n2;
        locals.var_tmf1_dn4 = assign59790_e93245_d_n4;
        locals.var_tmf1_dn5 = assign59790_e93245_d_n5;
        locals.var_tmf1_dn6 = assign59790_e93245_d_n6;
        locals.var_tmf1_dn7 = assign59790_e93245_d_n7;
        locals.var_tmf1_dn8 = assign59790_e93245_d_n8;
        locals.var_tmf1_dn9 = assign59790_e93245_d_n9;
        locals.var_tmf1_dn10 = assign59790_e93245_d_n10;
        locals.var_tmf1_dn11 = assign59790_e93245_d_n11;
        locals.var_tmf1_dn14 = assign59790_e93245_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign59800_e93260, assign59800_e93260_d_n0, assign59800_e93260_d_n2, assign59800_e93260_d_n4, assign59800_e93260_d_n5, assign59800_e93260_d_n6, assign59800_e93260_d_n7, assign59800_e93260_d_n8, assign59800_e93260_d_n9, assign59800_e93260_d_n10, assign59800_e93260_d_n11, assign59800_e93260_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 != 0.0)) && (locals.var_guard1467 != 0.0)) {
        let assign59800_e93258: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign59800_e93258, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign59800_e93260;
        locals.var_x2_dn0 = assign59800_e93260_d_n0;
        locals.var_x2_dn2 = assign59800_e93260_d_n2;
        locals.var_x2_dn4 = assign59800_e93260_d_n4;
        locals.var_x2_dn5 = assign59800_e93260_d_n5;
        locals.var_x2_dn6 = assign59800_e93260_d_n6;
        locals.var_x2_dn7 = assign59800_e93260_d_n7;
        locals.var_x2_dn8 = assign59800_e93260_d_n8;
        locals.var_x2_dn9 = assign59800_e93260_d_n9;
        locals.var_x2_dn10 = assign59800_e93260_d_n10;
        locals.var_x2_dn11 = assign59800_e93260_d_n11;
        locals.var_x2_dn14 = assign59800_e93260_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign59810_e93279, assign59810_e93279_d_n0, assign59810_e93279_d_n2, assign59810_e93279_d_n4, assign59810_e93279_d_n5, assign59810_e93279_d_n6, assign59810_e93279_d_n7, assign59810_e93279_d_n8, assign59810_e93279_d_n9, assign59810_e93279_d_n10, assign59810_e93279_d_n11, assign59810_e93279_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 != 0.0)) && (locals.var_guard1467 != 0.0)) {
        let assign59810_e93273: f64 = (10.0 * 2.220446049250313e-16);
        let assign59810_e93276: f64 = (10.0 * 2.220446049250313e-16);
        let assign59810_e93277: f64 = (assign59810_e93273 * assign59810_e93276);
        (assign59810_e93277, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign59810_e93279;
        locals.var_xmax2_dn0 = assign59810_e93279_d_n0;
        locals.var_xmax2_dn2 = assign59810_e93279_d_n2;
        locals.var_xmax2_dn4 = assign59810_e93279_d_n4;
        locals.var_xmax2_dn5 = assign59810_e93279_d_n5;
        locals.var_xmax2_dn6 = assign59810_e93279_d_n6;
        locals.var_xmax2_dn7 = assign59810_e93279_d_n7;
        locals.var_xmax2_dn8 = assign59810_e93279_d_n8;
        locals.var_xmax2_dn9 = assign59810_e93279_d_n9;
        locals.var_xmax2_dn10 = assign59810_e93279_d_n10;
        locals.var_xmax2_dn11 = assign59810_e93279_d_n11;
        locals.var_xmax2_dn14 = assign59810_e93279_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign59820_e93292, assign59820_e93292_d_n0, assign59820_e93292_d_n2, assign59820_e93292_d_n4, assign59820_e93292_d_n5, assign59820_e93292_d_n6, assign59820_e93292_d_n7, assign59820_e93292_d_n8, assign59820_e93292_d_n9, assign59820_e93292_d_n10, assign59820_e93292_d_n11, assign59820_e93292_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 != 0.0)) && (locals.var_guard1467 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign59820_e93292;
        locals.var_xp_dn0 = assign59820_e93292_d_n0;
        locals.var_xp_dn2 = assign59820_e93292_d_n2;
        locals.var_xp_dn4 = assign59820_e93292_d_n4;
        locals.var_xp_dn5 = assign59820_e93292_d_n5;
        locals.var_xp_dn6 = assign59820_e93292_d_n6;
        locals.var_xp_dn7 = assign59820_e93292_d_n7;
        locals.var_xp_dn8 = assign59820_e93292_d_n8;
        locals.var_xp_dn9 = assign59820_e93292_d_n9;
        locals.var_xp_dn10 = assign59820_e93292_d_n10;
        locals.var_xp_dn11 = assign59820_e93292_d_n11;
        locals.var_xp_dn14 = assign59820_e93292_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign59830_e93305, assign59830_e93305_d_n0, assign59830_e93305_d_n2, assign59830_e93305_d_n4, assign59830_e93305_d_n5, assign59830_e93305_d_n6, assign59830_e93305_d_n7, assign59830_e93305_d_n8, assign59830_e93305_d_n9, assign59830_e93305_d_n10, assign59830_e93305_d_n11, assign59830_e93305_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 != 0.0)) && (locals.var_guard1467 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign59830_e93305;
        locals.var_xmp_dn0 = assign59830_e93305_d_n0;
        locals.var_xmp_dn2 = assign59830_e93305_d_n2;
        locals.var_xmp_dn4 = assign59830_e93305_d_n4;
        locals.var_xmp_dn5 = assign59830_e93305_d_n5;
        locals.var_xmp_dn6 = assign59830_e93305_d_n6;
        locals.var_xmp_dn7 = assign59830_e93305_d_n7;
        locals.var_xmp_dn8 = assign59830_e93305_d_n8;
        locals.var_xmp_dn9 = assign59830_e93305_d_n9;
        locals.var_xmp_dn10 = assign59830_e93305_d_n10;
        locals.var_xmp_dn11 = assign59830_e93305_d_n11;
        locals.var_xmp_dn14 = assign59830_e93305_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign59840_e93318,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 != 0.0)) && (locals.var_guard1467 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign59840_e93318;
        locals.var_m0_rv = 0.0;

        let (assign59850_e93331,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 != 0.0)) && (locals.var_guard1467 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign59850_e93331;
        locals.var_mm_rv = 0.0;

        let (assign59860_e93344, assign59860_e93344_d_n0, assign59860_e93344_d_n2, assign59860_e93344_d_n4, assign59860_e93344_d_n5, assign59860_e93344_d_n6, assign59860_e93344_d_n7, assign59860_e93344_d_n8, assign59860_e93344_d_n9, assign59860_e93344_d_n10, assign59860_e93344_d_n11, assign59860_e93344_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 != 0.0)) && (locals.var_guard1467 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign59860_e93344;
        locals.var_arg_dn0 = assign59860_e93344_d_n0;
        locals.var_arg_dn2 = assign59860_e93344_d_n2;
        locals.var_arg_dn4 = assign59860_e93344_d_n4;
        locals.var_arg_dn5 = assign59860_e93344_d_n5;
        locals.var_arg_dn6 = assign59860_e93344_d_n6;
        locals.var_arg_dn7 = assign59860_e93344_d_n7;
        locals.var_arg_dn8 = assign59860_e93344_d_n8;
        locals.var_arg_dn9 = assign59860_e93344_d_n9;
        locals.var_arg_dn10 = assign59860_e93344_d_n10;
        locals.var_arg_dn11 = assign59860_e93344_d_n11;
        locals.var_arg_dn14 = assign59860_e93344_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign59870_e93357, assign59870_e93357_d_n0, assign59870_e93357_d_n2, assign59870_e93357_d_n4, assign59870_e93357_d_n5, assign59870_e93357_d_n6, assign59870_e93357_d_n7, assign59870_e93357_d_n8, assign59870_e93357_d_n9, assign59870_e93357_d_n10, assign59870_e93357_d_n11, assign59870_e93357_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 != 0.0)) && (locals.var_guard1467 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign59870_e93357;
        locals.var_dnm_dn0 = assign59870_e93357_d_n0;
        locals.var_dnm_dn2 = assign59870_e93357_d_n2;
        locals.var_dnm_dn4 = assign59870_e93357_d_n4;
        locals.var_dnm_dn5 = assign59870_e93357_d_n5;
        locals.var_dnm_dn6 = assign59870_e93357_d_n6;
        locals.var_dnm_dn7 = assign59870_e93357_d_n7;
        locals.var_dnm_dn8 = assign59870_e93357_d_n8;
        locals.var_dnm_dn9 = assign59870_e93357_d_n9;
        locals.var_dnm_dn10 = assign59870_e93357_d_n10;
        locals.var_dnm_dn11 = assign59870_e93357_d_n11;
        locals.var_dnm_dn14 = assign59870_e93357_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign59880_e93372, assign59880_e93372_d_n0, assign59880_e93372_d_n2, assign59880_e93372_d_n4, assign59880_e93372_d_n5, assign59880_e93372_d_n6, assign59880_e93372_d_n7, assign59880_e93372_d_n8, assign59880_e93372_d_n9, assign59880_e93372_d_n10, assign59880_e93372_d_n11, assign59880_e93372_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 != 0.0)) && (locals.var_guard1467 != 0.0)) {
        let assign59880_e93370: f64 = (locals.var_xp * locals.var_x2);
        (assign59880_e93370, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign59880_e93372;
        locals.var_xp_dn0 = assign59880_e93372_d_n0;
        locals.var_xp_dn2 = assign59880_e93372_d_n2;
        locals.var_xp_dn4 = assign59880_e93372_d_n4;
        locals.var_xp_dn5 = assign59880_e93372_d_n5;
        locals.var_xp_dn6 = assign59880_e93372_d_n6;
        locals.var_xp_dn7 = assign59880_e93372_d_n7;
        locals.var_xp_dn8 = assign59880_e93372_d_n8;
        locals.var_xp_dn9 = assign59880_e93372_d_n9;
        locals.var_xp_dn10 = assign59880_e93372_d_n10;
        locals.var_xp_dn11 = assign59880_e93372_d_n11;
        locals.var_xp_dn14 = assign59880_e93372_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign59890_e93387, assign59890_e93387_d_n0, assign59890_e93387_d_n2, assign59890_e93387_d_n4, assign59890_e93387_d_n5, assign59890_e93387_d_n6, assign59890_e93387_d_n7, assign59890_e93387_d_n8, assign59890_e93387_d_n9, assign59890_e93387_d_n10, assign59890_e93387_d_n11, assign59890_e93387_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 != 0.0)) && (locals.var_guard1467 != 0.0)) {
        let assign59890_e93385: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign59890_e93385, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign59890_e93387;
        locals.var_xmp_dn0 = assign59890_e93387_d_n0;
        locals.var_xmp_dn2 = assign59890_e93387_d_n2;
        locals.var_xmp_dn4 = assign59890_e93387_d_n4;
        locals.var_xmp_dn5 = assign59890_e93387_d_n5;
        locals.var_xmp_dn6 = assign59890_e93387_d_n6;
        locals.var_xmp_dn7 = assign59890_e93387_d_n7;
        locals.var_xmp_dn8 = assign59890_e93387_d_n8;
        locals.var_xmp_dn9 = assign59890_e93387_d_n9;
        locals.var_xmp_dn10 = assign59890_e93387_d_n10;
        locals.var_xmp_dn11 = assign59890_e93387_d_n11;
        locals.var_xmp_dn14 = assign59890_e93387_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign59900_e93402, assign59900_e93402_d_n0, assign59900_e93402_d_n2, assign59900_e93402_d_n4, assign59900_e93402_d_n5, assign59900_e93402_d_n6, assign59900_e93402_d_n7, assign59900_e93402_d_n8, assign59900_e93402_d_n9, assign59900_e93402_d_n10, assign59900_e93402_d_n11, assign59900_e93402_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 != 0.0)) && (locals.var_guard1467 != 0.0)) {
        let assign59900_e93400: f64 = (locals.var_xp * locals.var_x2);
        (assign59900_e93400, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign59900_e93402;
        locals.var_xp_dn0 = assign59900_e93402_d_n0;
        locals.var_xp_dn2 = assign59900_e93402_d_n2;
        locals.var_xp_dn4 = assign59900_e93402_d_n4;
        locals.var_xp_dn5 = assign59900_e93402_d_n5;
        locals.var_xp_dn6 = assign59900_e93402_d_n6;
        locals.var_xp_dn7 = assign59900_e93402_d_n7;
        locals.var_xp_dn8 = assign59900_e93402_d_n8;
        locals.var_xp_dn9 = assign59900_e93402_d_n9;
        locals.var_xp_dn10 = assign59900_e93402_d_n10;
        locals.var_xp_dn11 = assign59900_e93402_d_n11;
        locals.var_xp_dn14 = assign59900_e93402_d_n14;
        locals.var_xp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_223(
        locals: &mut StampLocals,
    ) {
        let (assign59910_e93417, assign59910_e93417_d_n0, assign59910_e93417_d_n2, assign59910_e93417_d_n4, assign59910_e93417_d_n5, assign59910_e93417_d_n6, assign59910_e93417_d_n7, assign59910_e93417_d_n8, assign59910_e93417_d_n9, assign59910_e93417_d_n10, assign59910_e93417_d_n11, assign59910_e93417_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 != 0.0)) && (locals.var_guard1467 != 0.0)) {
        let assign59910_e93415: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign59910_e93415, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign59910_e93417;
        locals.var_xmp_dn0 = assign59910_e93417_d_n0;
        locals.var_xmp_dn2 = assign59910_e93417_d_n2;
        locals.var_xmp_dn4 = assign59910_e93417_d_n4;
        locals.var_xmp_dn5 = assign59910_e93417_d_n5;
        locals.var_xmp_dn6 = assign59910_e93417_d_n6;
        locals.var_xmp_dn7 = assign59910_e93417_d_n7;
        locals.var_xmp_dn8 = assign59910_e93417_d_n8;
        locals.var_xmp_dn9 = assign59910_e93417_d_n9;
        locals.var_xmp_dn10 = assign59910_e93417_d_n10;
        locals.var_xmp_dn11 = assign59910_e93417_d_n11;
        locals.var_xmp_dn14 = assign59910_e93417_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign59920_e93432, assign59920_e93432_d_n0, assign59920_e93432_d_n2, assign59920_e93432_d_n4, assign59920_e93432_d_n5, assign59920_e93432_d_n6, assign59920_e93432_d_n7, assign59920_e93432_d_n8, assign59920_e93432_d_n9, assign59920_e93432_d_n10, assign59920_e93432_d_n11, assign59920_e93432_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 != 0.0)) && (locals.var_guard1467 != 0.0)) {
        let assign59920_e93430: f64 = (locals.var_xp + locals.var_xmp);
        (assign59920_e93430, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign59920_e93432;
        locals.var_arg_dn0 = assign59920_e93432_d_n0;
        locals.var_arg_dn2 = assign59920_e93432_d_n2;
        locals.var_arg_dn4 = assign59920_e93432_d_n4;
        locals.var_arg_dn5 = assign59920_e93432_d_n5;
        locals.var_arg_dn6 = assign59920_e93432_d_n6;
        locals.var_arg_dn7 = assign59920_e93432_d_n7;
        locals.var_arg_dn8 = assign59920_e93432_d_n8;
        locals.var_arg_dn9 = assign59920_e93432_d_n9;
        locals.var_arg_dn10 = assign59920_e93432_d_n10;
        locals.var_arg_dn11 = assign59920_e93432_d_n11;
        locals.var_arg_dn14 = assign59920_e93432_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign59930_e93445, assign59930_e93445_d_n0, assign59930_e93445_d_n2, assign59930_e93445_d_n4, assign59930_e93445_d_n5, assign59930_e93445_d_n6, assign59930_e93445_d_n7, assign59930_e93445_d_n8, assign59930_e93445_d_n9, assign59930_e93445_d_n10, assign59930_e93445_d_n11, assign59930_e93445_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 != 0.0)) && (locals.var_guard1467 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign59930_e93445;
        locals.var_dnm_dn0 = assign59930_e93445_d_n0;
        locals.var_dnm_dn2 = assign59930_e93445_d_n2;
        locals.var_dnm_dn4 = assign59930_e93445_d_n4;
        locals.var_dnm_dn5 = assign59930_e93445_d_n5;
        locals.var_dnm_dn6 = assign59930_e93445_d_n6;
        locals.var_dnm_dn7 = assign59930_e93445_d_n7;
        locals.var_dnm_dn8 = assign59930_e93445_d_n8;
        locals.var_dnm_dn9 = assign59930_e93445_d_n9;
        locals.var_dnm_dn10 = assign59930_e93445_d_n10;
        locals.var_dnm_dn11 = assign59930_e93445_d_n11;
        locals.var_dnm_dn14 = assign59930_e93445_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign59940_e93460: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1468 = assign59940_e93460;
        locals.var_guard1468_rv = 0.0;

        let assign59950_e93463: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1469 = assign59950_e93463;
        locals.var_guard1469_rv = 0.0;

        let (assign59960_e93480,) = {
    if (((((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 != 0.0)) && (locals.var_guard1467 != 0.0)) && (locals.var_guard1468 != 0.0)) && (locals.var_guard1469 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign59960_e93480;
        locals.var_mm_rv = 0.0;

        let assign59970_e93483: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1470 = assign59970_e93483;
        locals.var_guard1470_rv = 0.0;

        let (assign59980_e93503,) = {
    if ((((((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 != 0.0)) && (locals.var_guard1467 != 0.0)) && (locals.var_guard1468 != 0.0)) && (locals.var_guard1469 == 0.0)) && (locals.var_guard1470 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign59980_e93503;
        locals.var_mm_rv = 0.0;

        let assign59990_e93506: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1471 = assign59990_e93506;
        locals.var_guard1471_rv = 0.0;

        let (assign60000_e93529,) = {
    if (((((((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 != 0.0)) && (locals.var_guard1467 != 0.0)) && (locals.var_guard1468 != 0.0)) && (locals.var_guard1469 == 0.0)) && (locals.var_guard1470 == 0.0)) && (locals.var_guard1471 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign60000_e93529;
        locals.var_mm_rv = 0.0;

        let assign60010_e93532: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1472 = assign60010_e93532;
        locals.var_guard1472_rv = 0.0;

        let (assign60020_e93558,) = {
    if ((((((((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 != 0.0)) && (locals.var_guard1467 != 0.0)) && (locals.var_guard1468 != 0.0)) && (locals.var_guard1469 == 0.0)) && (locals.var_guard1470 == 0.0)) && (locals.var_guard1471 == 0.0)) && (locals.var_guard1472 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign60020_e93558;
        locals.var_mm_rv = 0.0;

        let (assign60030_e93573,) = {
    if ((((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 != 0.0)) && (locals.var_guard1467 != 0.0)) && (locals.var_guard1468 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign60030_e93573;
        locals.var_m0_rv = 0.0;

        let mut assign60040_loop_guard: usize = 0;
        while {
            let assign60040_cond_e93589: f64 = if (((((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 != 0.0)) && (locals.var_guard1467 != 0.0)) && (locals.var_guard1468 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign60040_cond_e93589 != 0.0
        } {
            assign60040_loop_guard += 1;
            assert!(assign60040_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign60040_body0_e93605, assign60040_body0_e93605_d_n0, assign60040_body0_e93605_d_n2, assign60040_body0_e93605_d_n4, assign60040_body0_e93605_d_n5, assign60040_body0_e93605_d_n6, assign60040_body0_e93605_d_n7, assign60040_body0_e93605_d_n8, assign60040_body0_e93605_d_n9, assign60040_body0_e93605_d_n10, assign60040_body0_e93605_d_n11, assign60040_body0_e93605_d_n14,) = {
    if ((((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 != 0.0)) && (locals.var_guard1467 != 0.0)) && (locals.var_guard1468 != 0.0)) {
        let assign60040_body0_e93603: f64 = (locals.var_dnm).sqrt();
        (assign60040_body0_e93603, (locals.var_dnm_dn0 / (2.0 * assign60040_body0_e93603)), (locals.var_dnm_dn2 / (2.0 * assign60040_body0_e93603)), (locals.var_dnm_dn4 / (2.0 * assign60040_body0_e93603)), (locals.var_dnm_dn5 / (2.0 * assign60040_body0_e93603)), (locals.var_dnm_dn6 / (2.0 * assign60040_body0_e93603)), (locals.var_dnm_dn7 / (2.0 * assign60040_body0_e93603)), (locals.var_dnm_dn8 / (2.0 * assign60040_body0_e93603)), (locals.var_dnm_dn9 / (2.0 * assign60040_body0_e93603)), (locals.var_dnm_dn10 / (2.0 * assign60040_body0_e93603)), (locals.var_dnm_dn11 / (2.0 * assign60040_body0_e93603)), (locals.var_dnm_dn14 / (2.0 * assign60040_body0_e93603)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign60040_body0_e93605;
            locals.var_dnm_dn0 = assign60040_body0_e93605_d_n0;
            locals.var_dnm_dn2 = assign60040_body0_e93605_d_n2;
            locals.var_dnm_dn4 = assign60040_body0_e93605_d_n4;
            locals.var_dnm_dn5 = assign60040_body0_e93605_d_n5;
            locals.var_dnm_dn6 = assign60040_body0_e93605_d_n6;
            locals.var_dnm_dn7 = assign60040_body0_e93605_d_n7;
            locals.var_dnm_dn8 = assign60040_body0_e93605_d_n8;
            locals.var_dnm_dn9 = assign60040_body0_e93605_d_n9;
            locals.var_dnm_dn10 = assign60040_body0_e93605_d_n10;
            locals.var_dnm_dn11 = assign60040_body0_e93605_d_n11;
            locals.var_dnm_dn14 = assign60040_body0_e93605_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign60040_body1_e93622,) = {
    if ((((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 != 0.0)) && (locals.var_guard1467 != 0.0)) && (locals.var_guard1468 != 0.0)) {
        let assign60040_body1_e93620: f64 = (locals.var_m0 + 1.0);
        (assign60040_body1_e93620,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign60040_body1_e93622;
            locals.var_m0_rv = 0.0;
        }

        let (assign60050_e93649, assign60050_e93649_d_n0, assign60050_e93649_d_n2, assign60050_e93649_d_n4, assign60050_e93649_d_n5, assign60050_e93649_d_n6, assign60050_e93649_d_n7, assign60050_e93649_d_n8, assign60050_e93649_d_n9, assign60050_e93649_d_n10, assign60050_e93649_d_n11, assign60050_e93649_d_n14,) = {
    if ((((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 != 0.0)) && (locals.var_guard1467 != 0.0)) && (locals.var_guard1468 == 0.0)) {
        let (assign60050_e93647, assign60050_e93647_d_n0, assign60050_e93647_d_n2, assign60050_e93647_d_n4, assign60050_e93647_d_n5, assign60050_e93647_d_n6, assign60050_e93647_d_n7, assign60050_e93647_d_n8, assign60050_e93647_d_n9, assign60050_e93647_d_n10, assign60050_e93647_d_n11, assign60050_e93647_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign60050_e93644: f64 = (2.0 * 2.0);
                let assign60050_e93645: f64 = (1.0 / assign60050_e93644);
                let assign60050_e93646: f64 = (locals.var_dnm).powf(assign60050_e93645);
                (assign60050_e93646, if 0.0 == 0.0 && ((assign60050_e93645) as f64).is_finite() && ((assign60050_e93645) as f64).fract() == 0.0 { if assign60050_e93645 == 0.0 { 0.0 } else { (assign60050_e93645 * ((locals.var_dnm).powf(assign60050_e93645 - 1.0) * locals.var_dnm_dn0)) } } else { (assign60050_e93646 * (assign60050_e93645 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60050_e93645) as f64).is_finite() && ((assign60050_e93645) as f64).fract() == 0.0 { if assign60050_e93645 == 0.0 { 0.0 } else { (assign60050_e93645 * ((locals.var_dnm).powf(assign60050_e93645 - 1.0) * locals.var_dnm_dn2)) } } else { (assign60050_e93646 * (assign60050_e93645 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60050_e93645) as f64).is_finite() && ((assign60050_e93645) as f64).fract() == 0.0 { if assign60050_e93645 == 0.0 { 0.0 } else { (assign60050_e93645 * ((locals.var_dnm).powf(assign60050_e93645 - 1.0) * locals.var_dnm_dn4)) } } else { (assign60050_e93646 * (assign60050_e93645 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60050_e93645) as f64).is_finite() && ((assign60050_e93645) as f64).fract() == 0.0 { if assign60050_e93645 == 0.0 { 0.0 } else { (assign60050_e93645 * ((locals.var_dnm).powf(assign60050_e93645 - 1.0) * locals.var_dnm_dn5)) } } else { (assign60050_e93646 * (assign60050_e93645 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60050_e93645) as f64).is_finite() && ((assign60050_e93645) as f64).fract() == 0.0 { if assign60050_e93645 == 0.0 { 0.0 } else { (assign60050_e93645 * ((locals.var_dnm).powf(assign60050_e93645 - 1.0) * locals.var_dnm_dn6)) } } else { (assign60050_e93646 * (assign60050_e93645 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60050_e93645) as f64).is_finite() && ((assign60050_e93645) as f64).fract() == 0.0 { if assign60050_e93645 == 0.0 { 0.0 } else { (assign60050_e93645 * ((locals.var_dnm).powf(assign60050_e93645 - 1.0) * locals.var_dnm_dn7)) } } else { (assign60050_e93646 * (assign60050_e93645 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60050_e93645) as f64).is_finite() && ((assign60050_e93645) as f64).fract() == 0.0 { if assign60050_e93645 == 0.0 { 0.0 } else { (assign60050_e93645 * ((locals.var_dnm).powf(assign60050_e93645 - 1.0) * locals.var_dnm_dn8)) } } else { (assign60050_e93646 * (assign60050_e93645 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60050_e93645) as f64).is_finite() && ((assign60050_e93645) as f64).fract() == 0.0 { if assign60050_e93645 == 0.0 { 0.0 } else { (assign60050_e93645 * ((locals.var_dnm).powf(assign60050_e93645 - 1.0) * locals.var_dnm_dn9)) } } else { (assign60050_e93646 * (assign60050_e93645 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60050_e93645) as f64).is_finite() && ((assign60050_e93645) as f64).fract() == 0.0 { if assign60050_e93645 == 0.0 { 0.0 } else { (assign60050_e93645 * ((locals.var_dnm).powf(assign60050_e93645 - 1.0) * locals.var_dnm_dn10)) } } else { (assign60050_e93646 * (assign60050_e93645 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60050_e93645) as f64).is_finite() && ((assign60050_e93645) as f64).fract() == 0.0 { if assign60050_e93645 == 0.0 { 0.0 } else { (assign60050_e93645 * ((locals.var_dnm).powf(assign60050_e93645 - 1.0) * locals.var_dnm_dn11)) } } else { (assign60050_e93646 * (assign60050_e93645 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60050_e93645) as f64).is_finite() && ((assign60050_e93645) as f64).fract() == 0.0 { if assign60050_e93645 == 0.0 { 0.0 } else { (assign60050_e93645 * ((locals.var_dnm).powf(assign60050_e93645 - 1.0) * locals.var_dnm_dn14)) } } else { (assign60050_e93646 * (assign60050_e93645 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign60050_e93647, assign60050_e93647_d_n0, assign60050_e93647_d_n2, assign60050_e93647_d_n4, assign60050_e93647_d_n5, assign60050_e93647_d_n6, assign60050_e93647_d_n7, assign60050_e93647_d_n8, assign60050_e93647_d_n9, assign60050_e93647_d_n10, assign60050_e93647_d_n11, assign60050_e93647_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign60050_e93649;
        locals.var_dnm_dn0 = assign60050_e93649_d_n0;
        locals.var_dnm_dn2 = assign60050_e93649_d_n2;
        locals.var_dnm_dn4 = assign60050_e93649_d_n4;
        locals.var_dnm_dn5 = assign60050_e93649_d_n5;
        locals.var_dnm_dn6 = assign60050_e93649_d_n6;
        locals.var_dnm_dn7 = assign60050_e93649_d_n7;
        locals.var_dnm_dn8 = assign60050_e93649_d_n8;
        locals.var_dnm_dn9 = assign60050_e93649_d_n9;
        locals.var_dnm_dn10 = assign60050_e93649_d_n10;
        locals.var_dnm_dn11 = assign60050_e93649_d_n11;
        locals.var_dnm_dn14 = assign60050_e93649_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign60060_e93664, assign60060_e93664_d_n0, assign60060_e93664_d_n2, assign60060_e93664_d_n4, assign60060_e93664_d_n5, assign60060_e93664_d_n6, assign60060_e93664_d_n7, assign60060_e93664_d_n8, assign60060_e93664_d_n9, assign60060_e93664_d_n10, assign60060_e93664_d_n11, assign60060_e93664_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 != 0.0)) && (locals.var_guard1467 != 0.0)) {
        let assign60060_e93662: f64 = (1.0 / locals.var_dnm);
        (assign60060_e93662, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign60060_e93664;
        locals.var_dnm_dn0 = assign60060_e93664_d_n0;
        locals.var_dnm_dn2 = assign60060_e93664_d_n2;
        locals.var_dnm_dn4 = assign60060_e93664_d_n4;
        locals.var_dnm_dn5 = assign60060_e93664_d_n5;
        locals.var_dnm_dn6 = assign60060_e93664_d_n6;
        locals.var_dnm_dn7 = assign60060_e93664_d_n7;
        locals.var_dnm_dn8 = assign60060_e93664_d_n8;
        locals.var_dnm_dn9 = assign60060_e93664_d_n9;
        locals.var_dnm_dn10 = assign60060_e93664_d_n10;
        locals.var_dnm_dn11 = assign60060_e93664_d_n11;
        locals.var_dnm_dn14 = assign60060_e93664_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign60070_e93683, assign60070_e93683_d_n0, assign60070_e93683_d_n2, assign60070_e93683_d_n4, assign60070_e93683_d_n5, assign60070_e93683_d_n6, assign60070_e93683_d_n7, assign60070_e93683_d_n8, assign60070_e93683_d_n9, assign60070_e93683_d_n10, assign60070_e93683_d_n11, assign60070_e93683_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 != 0.0)) && (locals.var_guard1467 != 0.0)) {
        let assign60070_e93678: f64 = (10.0 * 2.220446049250313e-16);
        let assign60070_e93679: f64 = (locals.var_tmf1 * assign60070_e93678);
        let assign60070_e93681: f64 = (assign60070_e93679 * locals.var_dnm);
        (assign60070_e93681, (((locals.var_tmf1_dn0 * assign60070_e93678) * locals.var_dnm) + (assign60070_e93679 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * assign60070_e93678) * locals.var_dnm) + (assign60070_e93679 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * assign60070_e93678) * locals.var_dnm) + (assign60070_e93679 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * assign60070_e93678) * locals.var_dnm) + (assign60070_e93679 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * assign60070_e93678) * locals.var_dnm) + (assign60070_e93679 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * assign60070_e93678) * locals.var_dnm) + (assign60070_e93679 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * assign60070_e93678) * locals.var_dnm) + (assign60070_e93679 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * assign60070_e93678) * locals.var_dnm) + (assign60070_e93679 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * assign60070_e93678) * locals.var_dnm) + (assign60070_e93679 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * assign60070_e93678) * locals.var_dnm) + (assign60070_e93679 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * assign60070_e93678) * locals.var_dnm) + (assign60070_e93679 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign60070_e93683;
        locals.var_tmf0_dn0 = assign60070_e93683_d_n0;
        locals.var_tmf0_dn2 = assign60070_e93683_d_n2;
        locals.var_tmf0_dn4 = assign60070_e93683_d_n4;
        locals.var_tmf0_dn5 = assign60070_e93683_d_n5;
        locals.var_tmf0_dn6 = assign60070_e93683_d_n6;
        locals.var_tmf0_dn7 = assign60070_e93683_d_n7;
        locals.var_tmf0_dn8 = assign60070_e93683_d_n8;
        locals.var_tmf0_dn9 = assign60070_e93683_d_n9;
        locals.var_tmf0_dn10 = assign60070_e93683_d_n10;
        locals.var_tmf0_dn11 = assign60070_e93683_d_n11;
        locals.var_tmf0_dn14 = assign60070_e93683_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign60080_e93704, assign60080_e93704_d_n0, assign60080_e93704_d_n2, assign60080_e93704_d_n4, assign60080_e93704_d_n5, assign60080_e93704_d_n6, assign60080_e93704_d_n7, assign60080_e93704_d_n8, assign60080_e93704_d_n9, assign60080_e93704_d_n10, assign60080_e93704_d_n11, assign60080_e93704_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 != 0.0)) && (locals.var_guard1467 != 0.0)) {
        let assign60080_e93696: f64 = (10.0 * 2.220446049250313e-16);
        let assign60080_e93698: f64 = (assign60080_e93696 * locals.var_xmp);
        let assign60080_e93700: f64 = (assign60080_e93698 * locals.var_dnm);
        let assign60080_e93702: f64 = (assign60080_e93700 / locals.var_arg);
        (assign60080_e93702, ((((((assign60080_e93696 * locals.var_xmp_dn0) * locals.var_dnm) + (assign60080_e93698 * locals.var_dnm_dn0)) * locals.var_arg) - (assign60080_e93700 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((assign60080_e93696 * locals.var_xmp_dn2) * locals.var_dnm) + (assign60080_e93698 * locals.var_dnm_dn2)) * locals.var_arg) - (assign60080_e93700 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((assign60080_e93696 * locals.var_xmp_dn4) * locals.var_dnm) + (assign60080_e93698 * locals.var_dnm_dn4)) * locals.var_arg) - (assign60080_e93700 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((assign60080_e93696 * locals.var_xmp_dn5) * locals.var_dnm) + (assign60080_e93698 * locals.var_dnm_dn5)) * locals.var_arg) - (assign60080_e93700 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((assign60080_e93696 * locals.var_xmp_dn6) * locals.var_dnm) + (assign60080_e93698 * locals.var_dnm_dn6)) * locals.var_arg) - (assign60080_e93700 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((assign60080_e93696 * locals.var_xmp_dn7) * locals.var_dnm) + (assign60080_e93698 * locals.var_dnm_dn7)) * locals.var_arg) - (assign60080_e93700 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((assign60080_e93696 * locals.var_xmp_dn8) * locals.var_dnm) + (assign60080_e93698 * locals.var_dnm_dn8)) * locals.var_arg) - (assign60080_e93700 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((assign60080_e93696 * locals.var_xmp_dn9) * locals.var_dnm) + (assign60080_e93698 * locals.var_dnm_dn9)) * locals.var_arg) - (assign60080_e93700 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((assign60080_e93696 * locals.var_xmp_dn10) * locals.var_dnm) + (assign60080_e93698 * locals.var_dnm_dn10)) * locals.var_arg) - (assign60080_e93700 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((assign60080_e93696 * locals.var_xmp_dn11) * locals.var_dnm) + (assign60080_e93698 * locals.var_dnm_dn11)) * locals.var_arg) - (assign60080_e93700 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((assign60080_e93696 * locals.var_xmp_dn14) * locals.var_dnm) + (assign60080_e93698 * locals.var_dnm_dn14)) * locals.var_arg) - (assign60080_e93700 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign60080_e93704;
        locals.var_t0_dn0 = assign60080_e93704_d_n0;
        locals.var_t0_dn2 = assign60080_e93704_d_n2;
        locals.var_t0_dn4 = assign60080_e93704_d_n4;
        locals.var_t0_dn5 = assign60080_e93704_d_n5;
        locals.var_t0_dn6 = assign60080_e93704_d_n6;
        locals.var_t0_dn7 = assign60080_e93704_d_n7;
        locals.var_t0_dn8 = assign60080_e93704_d_n8;
        locals.var_t0_dn9 = assign60080_e93704_d_n9;
        locals.var_t0_dn10 = assign60080_e93704_d_n10;
        locals.var_t0_dn11 = assign60080_e93704_d_n11;
        locals.var_t0_dn14 = assign60080_e93704_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign60090_e93729, assign60090_e93729_d_n0, assign60090_e93729_d_n2, assign60090_e93729_d_n4, assign60090_e93729_d_n5, assign60090_e93729_d_n6, assign60090_e93729_d_n7, assign60090_e93729_d_n8, assign60090_e93729_d_n9, assign60090_e93729_d_n10, assign60090_e93729_d_n11, assign60090_e93729_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 != 0.0)) && (locals.var_guard1467 != 0.0)) {
        let assign60090_e93717: f64 = (locals.var_ps0 + locals.var_vds);
        let assign60090_e93720: f64 = (10.0 * 2.220446049250313e-16);
        let assign60090_e93721: f64 = (assign60090_e93717 - assign60090_e93720);
        let assign60090_e93724: f64 = (10.0 * 2.220446049250313e-16);
        let assign60090_e93725: f64 = (assign60090_e93721 - assign60090_e93724);
        let assign60090_e93727: f64 = (assign60090_e93725 + locals.var_tmf0);
        (assign60090_e93727, ((locals.var_ps0_dn0 + locals.var_vds_dn0) + locals.var_tmf0_dn0), ((locals.var_ps0_dn2 + locals.var_vds_dn2) + locals.var_tmf0_dn2), ((locals.var_ps0_dn4 + locals.var_vds_dn4) + locals.var_tmf0_dn4), ((locals.var_ps0_dn5 + locals.var_vds_dn5) + locals.var_tmf0_dn5), ((locals.var_ps0_dn6 + locals.var_vds_dn6) + locals.var_tmf0_dn6), ((locals.var_ps0_dn7 + locals.var_vds_dn7) + locals.var_tmf0_dn7), ((locals.var_ps0_dn8 + locals.var_vds_dn8) + locals.var_tmf0_dn8), ((locals.var_ps0_dn9 + locals.var_vds_dn9) + locals.var_tmf0_dn9), ((locals.var_ps0_dn10 + locals.var_vds_dn10) + locals.var_tmf0_dn10), ((locals.var_ps0_dn11 + locals.var_vds_dn11) + locals.var_tmf0_dn11), ((locals.var_ps0_dn14 + locals.var_vds_dn14) + locals.var_tmf0_dn14),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn14,)
    }
};
        locals.var_psdl = assign60090_e93729;
        locals.var_psdl_dn0 = assign60090_e93729_d_n0;
        locals.var_psdl_dn2 = assign60090_e93729_d_n2;
        locals.var_psdl_dn4 = assign60090_e93729_d_n4;
        locals.var_psdl_dn5 = assign60090_e93729_d_n5;
        locals.var_psdl_dn6 = assign60090_e93729_d_n6;
        locals.var_psdl_dn7 = assign60090_e93729_d_n7;
        locals.var_psdl_dn8 = assign60090_e93729_d_n8;
        locals.var_psdl_dn9 = assign60090_e93729_d_n9;
        locals.var_psdl_dn10 = assign60090_e93729_d_n10;
        locals.var_psdl_dn11 = assign60090_e93729_d_n11;
        locals.var_psdl_dn14 = assign60090_e93729_d_n14;
        locals.var_psdl_rv = 0.0;

        let (assign60100_e93742, assign60100_e93742_d_n0, assign60100_e93742_d_n2, assign60100_e93742_d_n4, assign60100_e93742_d_n5, assign60100_e93742_d_n6, assign60100_e93742_d_n7, assign60100_e93742_d_n8, assign60100_e93742_d_n9, assign60100_e93742_d_n10, assign60100_e93742_d_n11, assign60100_e93742_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 != 0.0)) && (locals.var_guard1467 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign60100_e93742;
        locals.var_t0_dn0 = assign60100_e93742_d_n0;
        locals.var_t0_dn2 = assign60100_e93742_d_n2;
        locals.var_t0_dn4 = assign60100_e93742_d_n4;
        locals.var_t0_dn5 = assign60100_e93742_d_n5;
        locals.var_t0_dn6 = assign60100_e93742_d_n6;
        locals.var_t0_dn7 = assign60100_e93742_d_n7;
        locals.var_t0_dn8 = assign60100_e93742_d_n8;
        locals.var_t0_dn9 = assign60100_e93742_d_n9;
        locals.var_t0_dn10 = assign60100_e93742_d_n10;
        locals.var_t0_dn11 = assign60100_e93742_d_n11;
        locals.var_t0_dn14 = assign60100_e93742_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign60110_e93756, assign60110_e93756_d_n0, assign60110_e93756_d_n2, assign60110_e93756_d_n4, assign60110_e93756_d_n5, assign60110_e93756_d_n6, assign60110_e93756_d_n7, assign60110_e93756_d_n8, assign60110_e93756_d_n9, assign60110_e93756_d_n10, assign60110_e93756_d_n11, assign60110_e93756_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 != 0.0)) && (locals.var_guard1467 == 0.0)) {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn14,)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn14,)
    }
};
        locals.var_psdl = assign60110_e93756;
        locals.var_psdl_dn0 = assign60110_e93756_d_n0;
        locals.var_psdl_dn2 = assign60110_e93756_d_n2;
        locals.var_psdl_dn4 = assign60110_e93756_d_n4;
        locals.var_psdl_dn5 = assign60110_e93756_d_n5;
        locals.var_psdl_dn6 = assign60110_e93756_d_n6;
        locals.var_psdl_dn7 = assign60110_e93756_d_n7;
        locals.var_psdl_dn8 = assign60110_e93756_d_n8;
        locals.var_psdl_dn9 = assign60110_e93756_d_n9;
        locals.var_psdl_dn10 = assign60110_e93756_d_n10;
        locals.var_psdl_dn11 = assign60110_e93756_d_n11;
        locals.var_psdl_dn14 = assign60110_e93756_d_n14;
        locals.var_psdl_rv = 0.0;

        let (assign60120_e93770, assign60120_e93770_d_n0, assign60120_e93770_d_n2, assign60120_e93770_d_n4, assign60120_e93770_d_n5, assign60120_e93770_d_n6, assign60120_e93770_d_n7, assign60120_e93770_d_n8, assign60120_e93770_d_n9, assign60120_e93770_d_n10, assign60120_e93770_d_n11, assign60120_e93770_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 != 0.0)) && (locals.var_guard1467 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign60120_e93770;
        locals.var_t0_dn0 = assign60120_e93770_d_n0;
        locals.var_t0_dn2 = assign60120_e93770_d_n2;
        locals.var_t0_dn4 = assign60120_e93770_d_n4;
        locals.var_t0_dn5 = assign60120_e93770_d_n5;
        locals.var_t0_dn6 = assign60120_e93770_d_n6;
        locals.var_t0_dn7 = assign60120_e93770_d_n7;
        locals.var_t0_dn8 = assign60120_e93770_d_n8;
        locals.var_t0_dn9 = assign60120_e93770_d_n9;
        locals.var_t0_dn10 = assign60120_e93770_d_n10;
        locals.var_t0_dn11 = assign60120_e93770_d_n11;
        locals.var_t0_dn14 = assign60120_e93770_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign60130_e93782, assign60130_e93782_d_n0, assign60130_e93782_d_n2, assign60130_e93782_d_n4, assign60130_e93782_d_n5, assign60130_e93782_d_n6, assign60130_e93782_d_n7, assign60130_e93782_d_n8, assign60130_e93782_d_n9, assign60130_e93782_d_n10, assign60130_e93782_d_n11, assign60130_e93782_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) {
        (locals.var_wdpl, locals.var_wdpl_dn0, locals.var_wdpl_dn2, locals.var_wdpl_dn4, locals.var_wdpl_dn5, locals.var_wdpl_dn6, locals.var_wdpl_dn7, locals.var_wdpl_dn8, locals.var_wdpl_dn9, locals.var_wdpl_dn10, locals.var_wdpl_dn11, locals.var_wdpl_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign60130_e93782;
        locals.var_t1_dn0 = assign60130_e93782_d_n0;
        locals.var_t1_dn2 = assign60130_e93782_d_n2;
        locals.var_t1_dn4 = assign60130_e93782_d_n4;
        locals.var_t1_dn5 = assign60130_e93782_d_n5;
        locals.var_t1_dn6 = assign60130_e93782_d_n6;
        locals.var_t1_dn7 = assign60130_e93782_d_n7;
        locals.var_t1_dn8 = assign60130_e93782_d_n8;
        locals.var_t1_dn9 = assign60130_e93782_d_n9;
        locals.var_t1_dn10 = assign60130_e93782_d_n10;
        locals.var_t1_dn11 = assign60130_e93782_d_n11;
        locals.var_t1_dn14 = assign60130_e93782_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign60140_e93797, assign60140_e93797_d_n0, assign60140_e93797_d_n2, assign60140_e93797_d_n4, assign60140_e93797_d_n5, assign60140_e93797_d_n6, assign60140_e93797_d_n7, assign60140_e93797_d_n8, assign60140_e93797_d_n9, assign60140_e93797_d_n10, assign60140_e93797_d_n11, assign60140_e93797_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) {
        let assign60140_e93794: f64 = (locals.var_psl - locals.var_vbscl__blk439);
        let assign60140_e93795: f64 = (assign60140_e93794).sqrt();
        (assign60140_e93795, ((locals.var_psl_dn0 - locals.var_vbscl__blk439_dn0) / (2.0 * assign60140_e93795)), ((locals.var_psl_dn2 - locals.var_vbscl__blk439_dn2) / (2.0 * assign60140_e93795)), ((locals.var_psl_dn4 - locals.var_vbscl__blk439_dn4) / (2.0 * assign60140_e93795)), ((locals.var_psl_dn5 - locals.var_vbscl__blk439_dn5) / (2.0 * assign60140_e93795)), ((locals.var_psl_dn6 - locals.var_vbscl__blk439_dn6) / (2.0 * assign60140_e93795)), ((locals.var_psl_dn7 - locals.var_vbscl__blk439_dn7) / (2.0 * assign60140_e93795)), ((locals.var_psl_dn8 - locals.var_vbscl__blk439_dn8) / (2.0 * assign60140_e93795)), ((locals.var_psl_dn9 - locals.var_vbscl__blk439_dn9) / (2.0 * assign60140_e93795)), ((locals.var_psl_dn10 - locals.var_vbscl__blk439_dn10) / (2.0 * assign60140_e93795)), ((locals.var_psl_dn11 - locals.var_vbscl__blk439_dn11) / (2.0 * assign60140_e93795)), ((locals.var_psl_dn14 - locals.var_vbscl__blk439_dn14) / (2.0 * assign60140_e93795)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign60140_e93797;
        locals.var_t8_dn0 = assign60140_e93797_d_n0;
        locals.var_t8_dn2 = assign60140_e93797_d_n2;
        locals.var_t8_dn4 = assign60140_e93797_d_n4;
        locals.var_t8_dn5 = assign60140_e93797_d_n5;
        locals.var_t8_dn6 = assign60140_e93797_d_n6;
        locals.var_t8_dn7 = assign60140_e93797_d_n7;
        locals.var_t8_dn8 = assign60140_e93797_d_n8;
        locals.var_t8_dn9 = assign60140_e93797_d_n9;
        locals.var_t8_dn10 = assign60140_e93797_d_n10;
        locals.var_t8_dn11 = assign60140_e93797_d_n11;
        locals.var_t8_dn14 = assign60140_e93797_d_n14;
        locals.var_t8_rv = 0.0;

        let (assign60150_e93811, assign60150_e93811_d_n0, assign60150_e93811_d_n2, assign60150_e93811_d_n4, assign60150_e93811_d_n5, assign60150_e93811_d_n6, assign60150_e93811_d_n7, assign60150_e93811_d_n8, assign60150_e93811_d_n9, assign60150_e93811_d_n10, assign60150_e93811_d_n11, assign60150_e93811_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) {
        let assign60150_e93809: f64 = (locals.var_t1 * locals.var_t8);
        (assign60150_e93809, ((locals.var_t1_dn0 * locals.var_t8) + (locals.var_t1 * locals.var_t8_dn0)), ((locals.var_t1_dn2 * locals.var_t8) + (locals.var_t1 * locals.var_t8_dn2)), ((locals.var_t1_dn4 * locals.var_t8) + (locals.var_t1 * locals.var_t8_dn4)), ((locals.var_t1_dn5 * locals.var_t8) + (locals.var_t1 * locals.var_t8_dn5)), ((locals.var_t1_dn6 * locals.var_t8) + (locals.var_t1 * locals.var_t8_dn6)), ((locals.var_t1_dn7 * locals.var_t8) + (locals.var_t1 * locals.var_t8_dn7)), ((locals.var_t1_dn8 * locals.var_t8) + (locals.var_t1 * locals.var_t8_dn8)), ((locals.var_t1_dn9 * locals.var_t8) + (locals.var_t1 * locals.var_t8_dn9)), ((locals.var_t1_dn10 * locals.var_t8) + (locals.var_t1 * locals.var_t8_dn10)), ((locals.var_t1_dn11 * locals.var_t8) + (locals.var_t1 * locals.var_t8_dn11)), ((locals.var_t1_dn14 * locals.var_t8) + (locals.var_t1 * locals.var_t8_dn14)),)
    } else {
        (locals.var_wd, locals.var_wd_dn0, locals.var_wd_dn2, locals.var_wd_dn4, locals.var_wd_dn5, locals.var_wd_dn6, locals.var_wd_dn7, locals.var_wd_dn8, locals.var_wd_dn9, locals.var_wd_dn10, locals.var_wd_dn11, locals.var_wd_dn14,)
    }
};
        locals.var_wd = assign60150_e93811;
        locals.var_wd_dn0 = assign60150_e93811_d_n0;
        locals.var_wd_dn2 = assign60150_e93811_d_n2;
        locals.var_wd_dn4 = assign60150_e93811_d_n4;
        locals.var_wd_dn5 = assign60150_e93811_d_n5;
        locals.var_wd_dn6 = assign60150_e93811_d_n6;
        locals.var_wd_dn7 = assign60150_e93811_d_n7;
        locals.var_wd_dn8 = assign60150_e93811_d_n8;
        locals.var_wd_dn9 = assign60150_e93811_d_n9;
        locals.var_wd_dn10 = assign60150_e93811_d_n10;
        locals.var_wd_dn11 = assign60150_e93811_d_n11;
        locals.var_wd_dn14 = assign60150_e93811_d_n14;
        locals.var_wd_rv = 0.0;

        let (assign60160_e93827, assign60160_e93827_d_n0, assign60160_e93827_d_n2, assign60160_e93827_d_n4, assign60160_e93827_d_n5, assign60160_e93827_d_n6, assign60160_e93827_d_n7, assign60160_e93827_d_n8, assign60160_e93827_d_n9, assign60160_e93827_d_n10, assign60160_e93827_d_n11, assign60160_e93827_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) {
        let assign60160_e93823: f64 = (0.5 * locals.var_t1);
        let assign60160_e93825: f64 = (assign60160_e93823 / locals.var_t8);
        (assign60160_e93825, ((((0.5 * locals.var_t1_dn0) * locals.var_t8) - (assign60160_e93823 * locals.var_t8_dn0)) / (locals.var_t8 * locals.var_t8)), ((((0.5 * locals.var_t1_dn2) * locals.var_t8) - (assign60160_e93823 * locals.var_t8_dn2)) / (locals.var_t8 * locals.var_t8)), ((((0.5 * locals.var_t1_dn4) * locals.var_t8) - (assign60160_e93823 * locals.var_t8_dn4)) / (locals.var_t8 * locals.var_t8)), ((((0.5 * locals.var_t1_dn5) * locals.var_t8) - (assign60160_e93823 * locals.var_t8_dn5)) / (locals.var_t8 * locals.var_t8)), ((((0.5 * locals.var_t1_dn6) * locals.var_t8) - (assign60160_e93823 * locals.var_t8_dn6)) / (locals.var_t8 * locals.var_t8)), ((((0.5 * locals.var_t1_dn7) * locals.var_t8) - (assign60160_e93823 * locals.var_t8_dn7)) / (locals.var_t8 * locals.var_t8)), ((((0.5 * locals.var_t1_dn8) * locals.var_t8) - (assign60160_e93823 * locals.var_t8_dn8)) / (locals.var_t8 * locals.var_t8)), ((((0.5 * locals.var_t1_dn9) * locals.var_t8) - (assign60160_e93823 * locals.var_t8_dn9)) / (locals.var_t8 * locals.var_t8)), ((((0.5 * locals.var_t1_dn10) * locals.var_t8) - (assign60160_e93823 * locals.var_t8_dn10)) / (locals.var_t8 * locals.var_t8)), ((((0.5 * locals.var_t1_dn11) * locals.var_t8) - (assign60160_e93823 * locals.var_t8_dn11)) / (locals.var_t8 * locals.var_t8)), ((((0.5 * locals.var_t1_dn14) * locals.var_t8) - (assign60160_e93823 * locals.var_t8_dn14)) / (locals.var_t8 * locals.var_t8)),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign60160_e93827;
        locals.var_t9_dn0 = assign60160_e93827_d_n0;
        locals.var_t9_dn2 = assign60160_e93827_d_n2;
        locals.var_t9_dn4 = assign60160_e93827_d_n4;
        locals.var_t9_dn5 = assign60160_e93827_d_n5;
        locals.var_t9_dn6 = assign60160_e93827_d_n6;
        locals.var_t9_dn7 = assign60160_e93827_d_n7;
        locals.var_t9_dn8 = assign60160_e93827_d_n8;
        locals.var_t9_dn9 = assign60160_e93827_d_n9;
        locals.var_t9_dn10 = assign60160_e93827_d_n10;
        locals.var_t9_dn11 = assign60160_e93827_d_n11;
        locals.var_t9_dn14 = assign60160_e93827_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign60170_e93841, assign60170_e93841_d_n0, assign60170_e93841_d_n2, assign60170_e93841_d_n4, assign60170_e93841_d_n5, assign60170_e93841_d_n6, assign60170_e93841_d_n7, assign60170_e93841_d_n8, assign60170_e93841_d_n9, assign60170_e93841_d_n10, assign60170_e93841_d_n11, assign60170_e93841_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) {
        let assign60170_e93839: f64 = (1.0 / locals.var_wd);
        (assign60170_e93839, (-(locals.var_wd_dn0 / (locals.var_wd * locals.var_wd))), (-(locals.var_wd_dn2 / (locals.var_wd * locals.var_wd))), (-(locals.var_wd_dn4 / (locals.var_wd * locals.var_wd))), (-(locals.var_wd_dn5 / (locals.var_wd * locals.var_wd))), (-(locals.var_wd_dn6 / (locals.var_wd * locals.var_wd))), (-(locals.var_wd_dn7 / (locals.var_wd * locals.var_wd))), (-(locals.var_wd_dn8 / (locals.var_wd * locals.var_wd))), (-(locals.var_wd_dn9 / (locals.var_wd * locals.var_wd))), (-(locals.var_wd_dn10 / (locals.var_wd * locals.var_wd))), (-(locals.var_wd_dn11 / (locals.var_wd * locals.var_wd))), (-(locals.var_wd_dn14 / (locals.var_wd * locals.var_wd))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign60170_e93841;
        locals.var_t0_dn0 = assign60170_e93841_d_n0;
        locals.var_t0_dn2 = assign60170_e93841_d_n2;
        locals.var_t0_dn4 = assign60170_e93841_d_n4;
        locals.var_t0_dn5 = assign60170_e93841_d_n5;
        locals.var_t0_dn6 = assign60170_e93841_d_n6;
        locals.var_t0_dn7 = assign60170_e93841_d_n7;
        locals.var_t0_dn8 = assign60170_e93841_d_n8;
        locals.var_t0_dn9 = assign60170_e93841_d_n9;
        locals.var_t0_dn10 = assign60170_e93841_d_n10;
        locals.var_t0_dn11 = assign60170_e93841_d_n11;
        locals.var_t0_dn14 = assign60170_e93841_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign60180_e93855, assign60180_e93855_d_n0, assign60180_e93855_d_n2, assign60180_e93855_d_n4, assign60180_e93855_d_n5, assign60180_e93855_d_n6, assign60180_e93855_d_n7, assign60180_e93855_d_n8, assign60180_e93855_d_n9, assign60180_e93855_d_n10, assign60180_e93855_d_n11, assign60180_e93855_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) {
        let assign60180_e93853: f64 = (locals.var_qn0 * locals.var_t0);
        (assign60180_e93853, ((locals.var_qn0_dn0 * locals.var_t0) + (locals.var_qn0 * locals.var_t0_dn0)), ((locals.var_qn0_dn2 * locals.var_t0) + (locals.var_qn0 * locals.var_t0_dn2)), ((locals.var_qn0_dn4 * locals.var_t0) + (locals.var_qn0 * locals.var_t0_dn4)), ((locals.var_qn0_dn5 * locals.var_t0) + (locals.var_qn0 * locals.var_t0_dn5)), ((locals.var_qn0_dn6 * locals.var_t0) + (locals.var_qn0 * locals.var_t0_dn6)), ((locals.var_qn0_dn7 * locals.var_t0) + (locals.var_qn0 * locals.var_t0_dn7)), ((locals.var_qn0_dn8 * locals.var_t0) + (locals.var_qn0 * locals.var_t0_dn8)), ((locals.var_qn0_dn9 * locals.var_t0) + (locals.var_qn0 * locals.var_t0_dn9)), ((locals.var_qn0_dn10 * locals.var_t0) + (locals.var_qn0 * locals.var_t0_dn10)), ((locals.var_qn0_dn11 * locals.var_t0) + (locals.var_qn0 * locals.var_t0_dn11)), ((locals.var_qn0_dn14 * locals.var_t0) + (locals.var_qn0 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign60180_e93855;
        locals.var_t1_dn0 = assign60180_e93855_d_n0;
        locals.var_t1_dn2 = assign60180_e93855_d_n2;
        locals.var_t1_dn4 = assign60180_e93855_d_n4;
        locals.var_t1_dn5 = assign60180_e93855_d_n5;
        locals.var_t1_dn6 = assign60180_e93855_d_n6;
        locals.var_t1_dn7 = assign60180_e93855_d_n7;
        locals.var_t1_dn8 = assign60180_e93855_d_n8;
        locals.var_t1_dn9 = assign60180_e93855_d_n9;
        locals.var_t1_dn10 = assign60180_e93855_d_n10;
        locals.var_t1_dn11 = assign60180_e93855_d_n11;
        locals.var_t1_dn14 = assign60180_e93855_d_n14;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_224(
        locals: &mut StampLocals,
    ) {
        let (assign60190_e93869, assign60190_e93869_d_n0, assign60190_e93869_d_n2, assign60190_e93869_d_n4, assign60190_e93869_d_n5, assign60190_e93869_d_n6, assign60190_e93869_d_n7, assign60190_e93869_d_n8, assign60190_e93869_d_n9, assign60190_e93869_d_n10, assign60190_e93869_d_n11, assign60190_e93869_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) {
        let assign60190_e93867: f64 = (locals.var_uc_clm3 * locals.var_t1);
        (assign60190_e93867, (locals.var_uc_clm3 * locals.var_t1_dn0), (locals.var_uc_clm3 * locals.var_t1_dn2), (locals.var_uc_clm3 * locals.var_t1_dn4), (locals.var_uc_clm3 * locals.var_t1_dn5), (locals.var_uc_clm3 * locals.var_t1_dn6), (locals.var_uc_clm3 * locals.var_t1_dn7), (locals.var_uc_clm3 * locals.var_t1_dn8), (locals.var_uc_clm3 * locals.var_t1_dn9), (locals.var_uc_clm3 * locals.var_t1_dn10), (locals.var_uc_clm3 * locals.var_t1_dn11), (locals.var_uc_clm3 * locals.var_t1_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign60190_e93869;
        locals.var_t2_dn0 = assign60190_e93869_d_n0;
        locals.var_t2_dn2 = assign60190_e93869_d_n2;
        locals.var_t2_dn4 = assign60190_e93869_d_n4;
        locals.var_t2_dn5 = assign60190_e93869_d_n5;
        locals.var_t2_dn6 = assign60190_e93869_d_n6;
        locals.var_t2_dn7 = assign60190_e93869_d_n7;
        locals.var_t2_dn8 = assign60190_e93869_d_n8;
        locals.var_t2_dn9 = assign60190_e93869_d_n9;
        locals.var_t2_dn10 = assign60190_e93869_d_n10;
        locals.var_t2_dn11 = assign60190_e93869_d_n11;
        locals.var_t2_dn14 = assign60190_e93869_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign60200_e93883, assign60200_e93883_d_n0, assign60200_e93883_d_n2, assign60200_e93883_d_n4, assign60200_e93883_d_n5, assign60200_e93883_d_n6, assign60200_e93883_d_n7, assign60200_e93883_d_n8, assign60200_e93883_d_n9, assign60200_e93883_d_n10, assign60200_e93883_d_n11, assign60200_e93883_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) {
        let assign60200_e93881: f64 = (locals.var_uc_clm3 * locals.var_t0);
        (assign60200_e93881, (locals.var_uc_clm3 * locals.var_t0_dn0), (locals.var_uc_clm3 * locals.var_t0_dn2), (locals.var_uc_clm3 * locals.var_t0_dn4), (locals.var_uc_clm3 * locals.var_t0_dn5), (locals.var_uc_clm3 * locals.var_t0_dn6), (locals.var_uc_clm3 * locals.var_t0_dn7), (locals.var_uc_clm3 * locals.var_t0_dn8), (locals.var_uc_clm3 * locals.var_t0_dn9), (locals.var_uc_clm3 * locals.var_t0_dn10), (locals.var_uc_clm3 * locals.var_t0_dn11), (locals.var_uc_clm3 * locals.var_t0_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign60200_e93883;
        locals.var_t3_dn0 = assign60200_e93883_d_n0;
        locals.var_t3_dn2 = assign60200_e93883_d_n2;
        locals.var_t3_dn4 = assign60200_e93883_d_n4;
        locals.var_t3_dn5 = assign60200_e93883_d_n5;
        locals.var_t3_dn6 = assign60200_e93883_d_n6;
        locals.var_t3_dn7 = assign60200_e93883_d_n7;
        locals.var_t3_dn8 = assign60200_e93883_d_n8;
        locals.var_t3_dn9 = assign60200_e93883_d_n9;
        locals.var_t3_dn10 = assign60200_e93883_d_n10;
        locals.var_t3_dn11 = assign60200_e93883_d_n11;
        locals.var_t3_dn14 = assign60200_e93883_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign60210_e93899, assign60210_e93899_d_n0, assign60210_e93899_d_n2, assign60210_e93899_d_n4, assign60210_e93899_d_n5, assign60210_e93899_d_n6, assign60210_e93899_d_n7, assign60210_e93899_d_n8, assign60210_e93899_d_n9, assign60210_e93899_d_n10, assign60210_e93899_d_n11, assign60210_e93899_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) {
        let assign60210_e93895: f64 = (locals.var_uc_clm2 * locals.var_q_nsub);
        let assign60210_e93897: f64 = (assign60210_e93895 + locals.var_t2);
        (assign60210_e93897, (((locals.var_uc_clm2_dn0 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn0)) + locals.var_t2_dn0), (((locals.var_uc_clm2_dn2 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn2)) + locals.var_t2_dn2), (((locals.var_uc_clm2_dn4 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn4)) + locals.var_t2_dn4), (((locals.var_uc_clm2_dn5 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn5)) + locals.var_t2_dn5), (((locals.var_uc_clm2_dn6 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn6)) + locals.var_t2_dn6), (((locals.var_uc_clm2_dn7 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn7)) + locals.var_t2_dn7), (((locals.var_uc_clm2_dn8 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn8)) + locals.var_t2_dn8), (((locals.var_uc_clm2_dn9 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn9)) + locals.var_t2_dn9), (((locals.var_uc_clm2_dn10 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn10)) + locals.var_t2_dn10), (((locals.var_uc_clm2_dn11 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn11)) + locals.var_t2_dn11), (((locals.var_uc_clm2_dn14 * locals.var_q_nsub) + (locals.var_uc_clm2 * locals.var_q_nsub_dn14)) + locals.var_t2_dn14),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign60210_e93899;
        locals.var_t5_dn0 = assign60210_e93899_d_n0;
        locals.var_t5_dn2 = assign60210_e93899_d_n2;
        locals.var_t5_dn4 = assign60210_e93899_d_n4;
        locals.var_t5_dn5 = assign60210_e93899_d_n5;
        locals.var_t5_dn6 = assign60210_e93899_d_n6;
        locals.var_t5_dn7 = assign60210_e93899_d_n7;
        locals.var_t5_dn8 = assign60210_e93899_d_n8;
        locals.var_t5_dn9 = assign60210_e93899_d_n9;
        locals.var_t5_dn10 = assign60210_e93899_d_n10;
        locals.var_t5_dn11 = assign60210_e93899_d_n11;
        locals.var_t5_dn14 = assign60210_e93899_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign60220_e93913, assign60220_e93913_d_n0, assign60220_e93913_d_n2, assign60220_e93913_d_n4, assign60220_e93913_d_n5, assign60220_e93913_d_n6, assign60220_e93913_d_n7, assign60220_e93913_d_n8, assign60220_e93913_d_n9, assign60220_e93913_d_n10, assign60220_e93913_d_n11, assign60220_e93913_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) {
        let assign60220_e93911: f64 = (1.0 / locals.var_t5);
        (assign60220_e93911, (-(locals.var_t5_dn0 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn2 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn4 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn5 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn6 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn7 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn8 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn9 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn10 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn11 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn14 / (locals.var_t5 * locals.var_t5))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign60220_e93913;
        locals.var_t1_dn0 = assign60220_e93913_d_n0;
        locals.var_t1_dn2 = assign60220_e93913_d_n2;
        locals.var_t1_dn4 = assign60220_e93913_d_n4;
        locals.var_t1_dn5 = assign60220_e93913_d_n5;
        locals.var_t1_dn6 = assign60220_e93913_d_n6;
        locals.var_t1_dn7 = assign60220_e93913_d_n7;
        locals.var_t1_dn8 = assign60220_e93913_d_n8;
        locals.var_t1_dn9 = assign60220_e93913_d_n9;
        locals.var_t1_dn10 = assign60220_e93913_d_n10;
        locals.var_t1_dn11 = assign60220_e93913_d_n11;
        locals.var_t1_dn14 = assign60220_e93913_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign60230_e93927, assign60230_e93927_d_n0, assign60230_e93927_d_n2, assign60230_e93927_d_n4, assign60230_e93927_d_n5, assign60230_e93927_d_n6, assign60230_e93927_d_n7, assign60230_e93927_d_n8, assign60230_e93927_d_n9, assign60230_e93927_d_n10, assign60230_e93927_d_n11, assign60230_e93927_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) {
        let assign60230_e93925: f64 = (1.034943e-10 * locals.var_t1);
        (assign60230_e93925, (1.034943e-10 * locals.var_t1_dn0), (1.034943e-10 * locals.var_t1_dn2), (1.034943e-10 * locals.var_t1_dn4), (1.034943e-10 * locals.var_t1_dn5), (1.034943e-10 * locals.var_t1_dn6), (1.034943e-10 * locals.var_t1_dn7), (1.034943e-10 * locals.var_t1_dn8), (1.034943e-10 * locals.var_t1_dn9), (1.034943e-10 * locals.var_t1_dn10), (1.034943e-10 * locals.var_t1_dn11), (1.034943e-10 * locals.var_t1_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign60230_e93927;
        locals.var_t4_dn0 = assign60230_e93927_d_n0;
        locals.var_t4_dn2 = assign60230_e93927_d_n2;
        locals.var_t4_dn4 = assign60230_e93927_d_n4;
        locals.var_t4_dn5 = assign60230_e93927_d_n5;
        locals.var_t4_dn6 = assign60230_e93927_d_n6;
        locals.var_t4_dn7 = assign60230_e93927_d_n7;
        locals.var_t4_dn8 = assign60230_e93927_d_n8;
        locals.var_t4_dn9 = assign60230_e93927_d_n9;
        locals.var_t4_dn10 = assign60230_e93927_d_n10;
        locals.var_t4_dn11 = assign60230_e93927_d_n11;
        locals.var_t4_dn14 = assign60230_e93927_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign60240_e93941, assign60240_e93941_d_n0, assign60240_e93941_d_n2, assign60240_e93941_d_n4, assign60240_e93941_d_n5, assign60240_e93941_d_n6, assign60240_e93941_d_n7, assign60240_e93941_d_n8, assign60240_e93941_d_n9, assign60240_e93941_d_n10, assign60240_e93941_d_n11, assign60240_e93941_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) {
        let assign60240_e93939: f64 = (1.0 - locals.var_uc_clm1);
        (assign60240_e93939, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign60240_e93941;
        locals.var_t1_dn0 = assign60240_e93941_d_n0;
        locals.var_t1_dn2 = assign60240_e93941_d_n2;
        locals.var_t1_dn4 = assign60240_e93941_d_n4;
        locals.var_t1_dn5 = assign60240_e93941_d_n5;
        locals.var_t1_dn6 = assign60240_e93941_d_n6;
        locals.var_t1_dn7 = assign60240_e93941_d_n7;
        locals.var_t1_dn8 = assign60240_e93941_d_n8;
        locals.var_t1_dn9 = assign60240_e93941_d_n9;
        locals.var_t1_dn10 = assign60240_e93941_d_n10;
        locals.var_t1_dn11 = assign60240_e93941_d_n11;
        locals.var_t1_dn14 = assign60240_e93941_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign60250_e93961, assign60250_e93961_d_n0, assign60250_e93961_d_n2, assign60250_e93961_d_n4, assign60250_e93961_d_n5, assign60250_e93961_d_n6, assign60250_e93961_d_n7, assign60250_e93961_d_n8, assign60250_e93961_d_n9, assign60250_e93961_d_n10, assign60250_e93961_d_n11, assign60250_e93961_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) {
        let assign60250_e93954: f64 = (locals.var_vds + locals.var_ps0);
        let assign60250_e93955: f64 = (locals.var_uc_clm1 * assign60250_e93954);
        let assign60250_e93958: f64 = (locals.var_t1 * locals.var_psl);
        let assign60250_e93959: f64 = (assign60250_e93955 + assign60250_e93958);
        (assign60250_e93959, ((locals.var_uc_clm1 * (locals.var_vds_dn0 + locals.var_ps0_dn0)) + ((locals.var_t1_dn0 * locals.var_psl) + (locals.var_t1 * locals.var_psl_dn0))), ((locals.var_uc_clm1 * (locals.var_vds_dn2 + locals.var_ps0_dn2)) + ((locals.var_t1_dn2 * locals.var_psl) + (locals.var_t1 * locals.var_psl_dn2))), ((locals.var_uc_clm1 * (locals.var_vds_dn4 + locals.var_ps0_dn4)) + ((locals.var_t1_dn4 * locals.var_psl) + (locals.var_t1 * locals.var_psl_dn4))), ((locals.var_uc_clm1 * (locals.var_vds_dn5 + locals.var_ps0_dn5)) + ((locals.var_t1_dn5 * locals.var_psl) + (locals.var_t1 * locals.var_psl_dn5))), ((locals.var_uc_clm1 * (locals.var_vds_dn6 + locals.var_ps0_dn6)) + ((locals.var_t1_dn6 * locals.var_psl) + (locals.var_t1 * locals.var_psl_dn6))), ((locals.var_uc_clm1 * (locals.var_vds_dn7 + locals.var_ps0_dn7)) + ((locals.var_t1_dn7 * locals.var_psl) + (locals.var_t1 * locals.var_psl_dn7))), ((locals.var_uc_clm1 * (locals.var_vds_dn8 + locals.var_ps0_dn8)) + ((locals.var_t1_dn8 * locals.var_psl) + (locals.var_t1 * locals.var_psl_dn8))), ((locals.var_uc_clm1 * (locals.var_vds_dn9 + locals.var_ps0_dn9)) + ((locals.var_t1_dn9 * locals.var_psl) + (locals.var_t1 * locals.var_psl_dn9))), ((locals.var_uc_clm1 * (locals.var_vds_dn10 + locals.var_ps0_dn10)) + ((locals.var_t1_dn10 * locals.var_psl) + (locals.var_t1 * locals.var_psl_dn10))), ((locals.var_uc_clm1 * (locals.var_vds_dn11 + locals.var_ps0_dn11)) + ((locals.var_t1_dn11 * locals.var_psl) + (locals.var_t1 * locals.var_psl_dn11))), ((locals.var_uc_clm1 * (locals.var_vds_dn14 + locals.var_ps0_dn14)) + ((locals.var_t1_dn14 * locals.var_psl) + (locals.var_t1 * locals.var_psl_dn14))),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn14,)
    }
};
        locals.var_psdl = assign60250_e93961;
        locals.var_psdl_dn0 = assign60250_e93961_d_n0;
        locals.var_psdl_dn2 = assign60250_e93961_d_n2;
        locals.var_psdl_dn4 = assign60250_e93961_d_n4;
        locals.var_psdl_dn5 = assign60250_e93961_d_n5;
        locals.var_psdl_dn6 = assign60250_e93961_d_n6;
        locals.var_psdl_dn7 = assign60250_e93961_d_n7;
        locals.var_psdl_dn8 = assign60250_e93961_d_n8;
        locals.var_psdl_dn9 = assign60250_e93961_d_n9;
        locals.var_psdl_dn10 = assign60250_e93961_d_n10;
        locals.var_psdl_dn11 = assign60250_e93961_d_n11;
        locals.var_psdl_dn14 = assign60250_e93961_d_n14;
        locals.var_psdl_rv = 0.0;

        let assign60260_e93965: f64 = (locals.var_ps0 + locals.var_vds);
        let assign60260_e93968: f64 = (10.0 * 2.220446049250313e-16);
        let assign60260_e93969: f64 = (assign60260_e93965 - assign60260_e93968);
        let assign60260_e93972: f64 = (10.0 * 2.220446049250313e-16);
        let assign60260_e93973: f64 = (assign60260_e93969 - assign60260_e93972);
        let assign60260_e93977: f64 = (10.0 * 2.220446049250313e-16);
        let assign60260_e93980: f64 = if ((locals.var_psdl > assign60260_e93973) && (assign60260_e93977 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1473 = assign60260_e93980;
        locals.var_guard1473_rv = 0.0;

        let (assign60270_e94006, assign60270_e94006_d_n0, assign60270_e94006_d_n2, assign60270_e94006_d_n4, assign60270_e94006_d_n5, assign60270_e94006_d_n6, assign60270_e94006_d_n7, assign60270_e94006_d_n8, assign60270_e94006_d_n9, assign60270_e94006_d_n10, assign60270_e94006_d_n11, assign60270_e94006_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) && (locals.var_guard1473 != 0.0)) {
        let assign60270_e93995: f64 = (locals.var_ps0 + locals.var_vds);
        let assign60270_e93998: f64 = (10.0 * 2.220446049250313e-16);
        let assign60270_e93999: f64 = (assign60270_e93995 - assign60270_e93998);
        let assign60270_e94000: f64 = (locals.var_psdl - assign60270_e93999);
        let assign60270_e94003: f64 = (10.0 * 2.220446049250313e-16);
        let assign60270_e94004: f64 = (assign60270_e94000 + assign60270_e94003);
        (assign60270_e94004, (locals.var_psdl_dn0 - (locals.var_ps0_dn0 + locals.var_vds_dn0)), (locals.var_psdl_dn2 - (locals.var_ps0_dn2 + locals.var_vds_dn2)), (locals.var_psdl_dn4 - (locals.var_ps0_dn4 + locals.var_vds_dn4)), (locals.var_psdl_dn5 - (locals.var_ps0_dn5 + locals.var_vds_dn5)), (locals.var_psdl_dn6 - (locals.var_ps0_dn6 + locals.var_vds_dn6)), (locals.var_psdl_dn7 - (locals.var_ps0_dn7 + locals.var_vds_dn7)), (locals.var_psdl_dn8 - (locals.var_ps0_dn8 + locals.var_vds_dn8)), (locals.var_psdl_dn9 - (locals.var_ps0_dn9 + locals.var_vds_dn9)), (locals.var_psdl_dn10 - (locals.var_ps0_dn10 + locals.var_vds_dn10)), (locals.var_psdl_dn11 - (locals.var_ps0_dn11 + locals.var_vds_dn11)), (locals.var_psdl_dn14 - (locals.var_ps0_dn14 + locals.var_vds_dn14)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign60270_e94006;
        locals.var_tmf1_dn0 = assign60270_e94006_d_n0;
        locals.var_tmf1_dn2 = assign60270_e94006_d_n2;
        locals.var_tmf1_dn4 = assign60270_e94006_d_n4;
        locals.var_tmf1_dn5 = assign60270_e94006_d_n5;
        locals.var_tmf1_dn6 = assign60270_e94006_d_n6;
        locals.var_tmf1_dn7 = assign60270_e94006_d_n7;
        locals.var_tmf1_dn8 = assign60270_e94006_d_n8;
        locals.var_tmf1_dn9 = assign60270_e94006_d_n9;
        locals.var_tmf1_dn10 = assign60270_e94006_d_n10;
        locals.var_tmf1_dn11 = assign60270_e94006_d_n11;
        locals.var_tmf1_dn14 = assign60270_e94006_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign60280_e94022, assign60280_e94022_d_n0, assign60280_e94022_d_n2, assign60280_e94022_d_n4, assign60280_e94022_d_n5, assign60280_e94022_d_n6, assign60280_e94022_d_n7, assign60280_e94022_d_n8, assign60280_e94022_d_n9, assign60280_e94022_d_n10, assign60280_e94022_d_n11, assign60280_e94022_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) && (locals.var_guard1473 != 0.0)) {
        let assign60280_e94020: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign60280_e94020, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign60280_e94022;
        locals.var_x2_dn0 = assign60280_e94022_d_n0;
        locals.var_x2_dn2 = assign60280_e94022_d_n2;
        locals.var_x2_dn4 = assign60280_e94022_d_n4;
        locals.var_x2_dn5 = assign60280_e94022_d_n5;
        locals.var_x2_dn6 = assign60280_e94022_d_n6;
        locals.var_x2_dn7 = assign60280_e94022_d_n7;
        locals.var_x2_dn8 = assign60280_e94022_d_n8;
        locals.var_x2_dn9 = assign60280_e94022_d_n9;
        locals.var_x2_dn10 = assign60280_e94022_d_n10;
        locals.var_x2_dn11 = assign60280_e94022_d_n11;
        locals.var_x2_dn14 = assign60280_e94022_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign60290_e94042, assign60290_e94042_d_n0, assign60290_e94042_d_n2, assign60290_e94042_d_n4, assign60290_e94042_d_n5, assign60290_e94042_d_n6, assign60290_e94042_d_n7, assign60290_e94042_d_n8, assign60290_e94042_d_n9, assign60290_e94042_d_n10, assign60290_e94042_d_n11, assign60290_e94042_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) && (locals.var_guard1473 != 0.0)) {
        let assign60290_e94036: f64 = (10.0 * 2.220446049250313e-16);
        let assign60290_e94039: f64 = (10.0 * 2.220446049250313e-16);
        let assign60290_e94040: f64 = (assign60290_e94036 * assign60290_e94039);
        (assign60290_e94040, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign60290_e94042;
        locals.var_xmax2_dn0 = assign60290_e94042_d_n0;
        locals.var_xmax2_dn2 = assign60290_e94042_d_n2;
        locals.var_xmax2_dn4 = assign60290_e94042_d_n4;
        locals.var_xmax2_dn5 = assign60290_e94042_d_n5;
        locals.var_xmax2_dn6 = assign60290_e94042_d_n6;
        locals.var_xmax2_dn7 = assign60290_e94042_d_n7;
        locals.var_xmax2_dn8 = assign60290_e94042_d_n8;
        locals.var_xmax2_dn9 = assign60290_e94042_d_n9;
        locals.var_xmax2_dn10 = assign60290_e94042_d_n10;
        locals.var_xmax2_dn11 = assign60290_e94042_d_n11;
        locals.var_xmax2_dn14 = assign60290_e94042_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign60300_e94056, assign60300_e94056_d_n0, assign60300_e94056_d_n2, assign60300_e94056_d_n4, assign60300_e94056_d_n5, assign60300_e94056_d_n6, assign60300_e94056_d_n7, assign60300_e94056_d_n8, assign60300_e94056_d_n9, assign60300_e94056_d_n10, assign60300_e94056_d_n11, assign60300_e94056_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) && (locals.var_guard1473 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign60300_e94056;
        locals.var_xp_dn0 = assign60300_e94056_d_n0;
        locals.var_xp_dn2 = assign60300_e94056_d_n2;
        locals.var_xp_dn4 = assign60300_e94056_d_n4;
        locals.var_xp_dn5 = assign60300_e94056_d_n5;
        locals.var_xp_dn6 = assign60300_e94056_d_n6;
        locals.var_xp_dn7 = assign60300_e94056_d_n7;
        locals.var_xp_dn8 = assign60300_e94056_d_n8;
        locals.var_xp_dn9 = assign60300_e94056_d_n9;
        locals.var_xp_dn10 = assign60300_e94056_d_n10;
        locals.var_xp_dn11 = assign60300_e94056_d_n11;
        locals.var_xp_dn14 = assign60300_e94056_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign60310_e94070, assign60310_e94070_d_n0, assign60310_e94070_d_n2, assign60310_e94070_d_n4, assign60310_e94070_d_n5, assign60310_e94070_d_n6, assign60310_e94070_d_n7, assign60310_e94070_d_n8, assign60310_e94070_d_n9, assign60310_e94070_d_n10, assign60310_e94070_d_n11, assign60310_e94070_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) && (locals.var_guard1473 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign60310_e94070;
        locals.var_xmp_dn0 = assign60310_e94070_d_n0;
        locals.var_xmp_dn2 = assign60310_e94070_d_n2;
        locals.var_xmp_dn4 = assign60310_e94070_d_n4;
        locals.var_xmp_dn5 = assign60310_e94070_d_n5;
        locals.var_xmp_dn6 = assign60310_e94070_d_n6;
        locals.var_xmp_dn7 = assign60310_e94070_d_n7;
        locals.var_xmp_dn8 = assign60310_e94070_d_n8;
        locals.var_xmp_dn9 = assign60310_e94070_d_n9;
        locals.var_xmp_dn10 = assign60310_e94070_d_n10;
        locals.var_xmp_dn11 = assign60310_e94070_d_n11;
        locals.var_xmp_dn14 = assign60310_e94070_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign60320_e94084,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) && (locals.var_guard1473 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign60320_e94084;
        locals.var_m0_rv = 0.0;

        let (assign60330_e94098,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) && (locals.var_guard1473 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign60330_e94098;
        locals.var_mm_rv = 0.0;

        let (assign60340_e94112, assign60340_e94112_d_n0, assign60340_e94112_d_n2, assign60340_e94112_d_n4, assign60340_e94112_d_n5, assign60340_e94112_d_n6, assign60340_e94112_d_n7, assign60340_e94112_d_n8, assign60340_e94112_d_n9, assign60340_e94112_d_n10, assign60340_e94112_d_n11, assign60340_e94112_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) && (locals.var_guard1473 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign60340_e94112;
        locals.var_arg_dn0 = assign60340_e94112_d_n0;
        locals.var_arg_dn2 = assign60340_e94112_d_n2;
        locals.var_arg_dn4 = assign60340_e94112_d_n4;
        locals.var_arg_dn5 = assign60340_e94112_d_n5;
        locals.var_arg_dn6 = assign60340_e94112_d_n6;
        locals.var_arg_dn7 = assign60340_e94112_d_n7;
        locals.var_arg_dn8 = assign60340_e94112_d_n8;
        locals.var_arg_dn9 = assign60340_e94112_d_n9;
        locals.var_arg_dn10 = assign60340_e94112_d_n10;
        locals.var_arg_dn11 = assign60340_e94112_d_n11;
        locals.var_arg_dn14 = assign60340_e94112_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign60350_e94126, assign60350_e94126_d_n0, assign60350_e94126_d_n2, assign60350_e94126_d_n4, assign60350_e94126_d_n5, assign60350_e94126_d_n6, assign60350_e94126_d_n7, assign60350_e94126_d_n8, assign60350_e94126_d_n9, assign60350_e94126_d_n10, assign60350_e94126_d_n11, assign60350_e94126_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) && (locals.var_guard1473 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign60350_e94126;
        locals.var_dnm_dn0 = assign60350_e94126_d_n0;
        locals.var_dnm_dn2 = assign60350_e94126_d_n2;
        locals.var_dnm_dn4 = assign60350_e94126_d_n4;
        locals.var_dnm_dn5 = assign60350_e94126_d_n5;
        locals.var_dnm_dn6 = assign60350_e94126_d_n6;
        locals.var_dnm_dn7 = assign60350_e94126_d_n7;
        locals.var_dnm_dn8 = assign60350_e94126_d_n8;
        locals.var_dnm_dn9 = assign60350_e94126_d_n9;
        locals.var_dnm_dn10 = assign60350_e94126_d_n10;
        locals.var_dnm_dn11 = assign60350_e94126_d_n11;
        locals.var_dnm_dn14 = assign60350_e94126_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign60360_e94142, assign60360_e94142_d_n0, assign60360_e94142_d_n2, assign60360_e94142_d_n4, assign60360_e94142_d_n5, assign60360_e94142_d_n6, assign60360_e94142_d_n7, assign60360_e94142_d_n8, assign60360_e94142_d_n9, assign60360_e94142_d_n10, assign60360_e94142_d_n11, assign60360_e94142_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) && (locals.var_guard1473 != 0.0)) {
        let assign60360_e94140: f64 = (locals.var_xp * locals.var_x2);
        (assign60360_e94140, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign60360_e94142;
        locals.var_xp_dn0 = assign60360_e94142_d_n0;
        locals.var_xp_dn2 = assign60360_e94142_d_n2;
        locals.var_xp_dn4 = assign60360_e94142_d_n4;
        locals.var_xp_dn5 = assign60360_e94142_d_n5;
        locals.var_xp_dn6 = assign60360_e94142_d_n6;
        locals.var_xp_dn7 = assign60360_e94142_d_n7;
        locals.var_xp_dn8 = assign60360_e94142_d_n8;
        locals.var_xp_dn9 = assign60360_e94142_d_n9;
        locals.var_xp_dn10 = assign60360_e94142_d_n10;
        locals.var_xp_dn11 = assign60360_e94142_d_n11;
        locals.var_xp_dn14 = assign60360_e94142_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign60370_e94158, assign60370_e94158_d_n0, assign60370_e94158_d_n2, assign60370_e94158_d_n4, assign60370_e94158_d_n5, assign60370_e94158_d_n6, assign60370_e94158_d_n7, assign60370_e94158_d_n8, assign60370_e94158_d_n9, assign60370_e94158_d_n10, assign60370_e94158_d_n11, assign60370_e94158_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) && (locals.var_guard1473 != 0.0)) {
        let assign60370_e94156: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign60370_e94156, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign60370_e94158;
        locals.var_xmp_dn0 = assign60370_e94158_d_n0;
        locals.var_xmp_dn2 = assign60370_e94158_d_n2;
        locals.var_xmp_dn4 = assign60370_e94158_d_n4;
        locals.var_xmp_dn5 = assign60370_e94158_d_n5;
        locals.var_xmp_dn6 = assign60370_e94158_d_n6;
        locals.var_xmp_dn7 = assign60370_e94158_d_n7;
        locals.var_xmp_dn8 = assign60370_e94158_d_n8;
        locals.var_xmp_dn9 = assign60370_e94158_d_n9;
        locals.var_xmp_dn10 = assign60370_e94158_d_n10;
        locals.var_xmp_dn11 = assign60370_e94158_d_n11;
        locals.var_xmp_dn14 = assign60370_e94158_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign60380_e94174, assign60380_e94174_d_n0, assign60380_e94174_d_n2, assign60380_e94174_d_n4, assign60380_e94174_d_n5, assign60380_e94174_d_n6, assign60380_e94174_d_n7, assign60380_e94174_d_n8, assign60380_e94174_d_n9, assign60380_e94174_d_n10, assign60380_e94174_d_n11, assign60380_e94174_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) && (locals.var_guard1473 != 0.0)) {
        let assign60380_e94172: f64 = (locals.var_xp * locals.var_x2);
        (assign60380_e94172, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign60380_e94174;
        locals.var_xp_dn0 = assign60380_e94174_d_n0;
        locals.var_xp_dn2 = assign60380_e94174_d_n2;
        locals.var_xp_dn4 = assign60380_e94174_d_n4;
        locals.var_xp_dn5 = assign60380_e94174_d_n5;
        locals.var_xp_dn6 = assign60380_e94174_d_n6;
        locals.var_xp_dn7 = assign60380_e94174_d_n7;
        locals.var_xp_dn8 = assign60380_e94174_d_n8;
        locals.var_xp_dn9 = assign60380_e94174_d_n9;
        locals.var_xp_dn10 = assign60380_e94174_d_n10;
        locals.var_xp_dn11 = assign60380_e94174_d_n11;
        locals.var_xp_dn14 = assign60380_e94174_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign60390_e94190, assign60390_e94190_d_n0, assign60390_e94190_d_n2, assign60390_e94190_d_n4, assign60390_e94190_d_n5, assign60390_e94190_d_n6, assign60390_e94190_d_n7, assign60390_e94190_d_n8, assign60390_e94190_d_n9, assign60390_e94190_d_n10, assign60390_e94190_d_n11, assign60390_e94190_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) && (locals.var_guard1473 != 0.0)) {
        let assign60390_e94188: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign60390_e94188, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign60390_e94190;
        locals.var_xmp_dn0 = assign60390_e94190_d_n0;
        locals.var_xmp_dn2 = assign60390_e94190_d_n2;
        locals.var_xmp_dn4 = assign60390_e94190_d_n4;
        locals.var_xmp_dn5 = assign60390_e94190_d_n5;
        locals.var_xmp_dn6 = assign60390_e94190_d_n6;
        locals.var_xmp_dn7 = assign60390_e94190_d_n7;
        locals.var_xmp_dn8 = assign60390_e94190_d_n8;
        locals.var_xmp_dn9 = assign60390_e94190_d_n9;
        locals.var_xmp_dn10 = assign60390_e94190_d_n10;
        locals.var_xmp_dn11 = assign60390_e94190_d_n11;
        locals.var_xmp_dn14 = assign60390_e94190_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign60400_e94206, assign60400_e94206_d_n0, assign60400_e94206_d_n2, assign60400_e94206_d_n4, assign60400_e94206_d_n5, assign60400_e94206_d_n6, assign60400_e94206_d_n7, assign60400_e94206_d_n8, assign60400_e94206_d_n9, assign60400_e94206_d_n10, assign60400_e94206_d_n11, assign60400_e94206_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) && (locals.var_guard1473 != 0.0)) {
        let assign60400_e94204: f64 = (locals.var_xp + locals.var_xmp);
        (assign60400_e94204, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign60400_e94206;
        locals.var_arg_dn0 = assign60400_e94206_d_n0;
        locals.var_arg_dn2 = assign60400_e94206_d_n2;
        locals.var_arg_dn4 = assign60400_e94206_d_n4;
        locals.var_arg_dn5 = assign60400_e94206_d_n5;
        locals.var_arg_dn6 = assign60400_e94206_d_n6;
        locals.var_arg_dn7 = assign60400_e94206_d_n7;
        locals.var_arg_dn8 = assign60400_e94206_d_n8;
        locals.var_arg_dn9 = assign60400_e94206_d_n9;
        locals.var_arg_dn10 = assign60400_e94206_d_n10;
        locals.var_arg_dn11 = assign60400_e94206_d_n11;
        locals.var_arg_dn14 = assign60400_e94206_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign60410_e94220, assign60410_e94220_d_n0, assign60410_e94220_d_n2, assign60410_e94220_d_n4, assign60410_e94220_d_n5, assign60410_e94220_d_n6, assign60410_e94220_d_n7, assign60410_e94220_d_n8, assign60410_e94220_d_n9, assign60410_e94220_d_n10, assign60410_e94220_d_n11, assign60410_e94220_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) && (locals.var_guard1473 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign60410_e94220;
        locals.var_dnm_dn0 = assign60410_e94220_d_n0;
        locals.var_dnm_dn2 = assign60410_e94220_d_n2;
        locals.var_dnm_dn4 = assign60410_e94220_d_n4;
        locals.var_dnm_dn5 = assign60410_e94220_d_n5;
        locals.var_dnm_dn6 = assign60410_e94220_d_n6;
        locals.var_dnm_dn7 = assign60410_e94220_d_n7;
        locals.var_dnm_dn8 = assign60410_e94220_d_n8;
        locals.var_dnm_dn9 = assign60410_e94220_d_n9;
        locals.var_dnm_dn10 = assign60410_e94220_d_n10;
        locals.var_dnm_dn11 = assign60410_e94220_d_n11;
        locals.var_dnm_dn14 = assign60410_e94220_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign60420_e94235: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1474 = assign60420_e94235;
        locals.var_guard1474_rv = 0.0;

        let assign60430_e94238: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1475 = assign60430_e94238;
        locals.var_guard1475_rv = 0.0;

        let (assign60440_e94256,) = {
    if (((((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) && (locals.var_guard1473 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1475 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign60440_e94256;
        locals.var_mm_rv = 0.0;

        let assign60450_e94259: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1476 = assign60450_e94259;
        locals.var_guard1476_rv = 0.0;

        let (assign60460_e94280,) = {
    if ((((((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) && (locals.var_guard1473 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1475 == 0.0)) && (locals.var_guard1476 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign60460_e94280;
        locals.var_mm_rv = 0.0;

        let assign60470_e94283: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1477 = assign60470_e94283;
        locals.var_guard1477_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_225(
        locals: &mut StampLocals,
    ) {
        let (assign60480_e94307,) = {
    if (((((((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) && (locals.var_guard1473 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1475 == 0.0)) && (locals.var_guard1476 == 0.0)) && (locals.var_guard1477 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign60480_e94307;
        locals.var_mm_rv = 0.0;

        let assign60490_e94310: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1478 = assign60490_e94310;
        locals.var_guard1478_rv = 0.0;

        let (assign60500_e94337,) = {
    if ((((((((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) && (locals.var_guard1473 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_guard1475 == 0.0)) && (locals.var_guard1476 == 0.0)) && (locals.var_guard1477 == 0.0)) && (locals.var_guard1478 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign60500_e94337;
        locals.var_mm_rv = 0.0;

        let (assign60510_e94353,) = {
    if ((((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) && (locals.var_guard1473 != 0.0)) && (locals.var_guard1474 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign60510_e94353;
        locals.var_m0_rv = 0.0;

        let mut assign60520_loop_guard: usize = 0;
        while {
            let assign60520_cond_e94370: f64 = if (((((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) && (locals.var_guard1473 != 0.0)) && (locals.var_guard1474 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign60520_cond_e94370 != 0.0
        } {
            assign60520_loop_guard += 1;
            assert!(assign60520_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign60520_body0_e94387, assign60520_body0_e94387_d_n0, assign60520_body0_e94387_d_n2, assign60520_body0_e94387_d_n4, assign60520_body0_e94387_d_n5, assign60520_body0_e94387_d_n6, assign60520_body0_e94387_d_n7, assign60520_body0_e94387_d_n8, assign60520_body0_e94387_d_n9, assign60520_body0_e94387_d_n10, assign60520_body0_e94387_d_n11, assign60520_body0_e94387_d_n14,) = {
    if ((((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) && (locals.var_guard1473 != 0.0)) && (locals.var_guard1474 != 0.0)) {
        let assign60520_body0_e94385: f64 = (locals.var_dnm).sqrt();
        (assign60520_body0_e94385, (locals.var_dnm_dn0 / (2.0 * assign60520_body0_e94385)), (locals.var_dnm_dn2 / (2.0 * assign60520_body0_e94385)), (locals.var_dnm_dn4 / (2.0 * assign60520_body0_e94385)), (locals.var_dnm_dn5 / (2.0 * assign60520_body0_e94385)), (locals.var_dnm_dn6 / (2.0 * assign60520_body0_e94385)), (locals.var_dnm_dn7 / (2.0 * assign60520_body0_e94385)), (locals.var_dnm_dn8 / (2.0 * assign60520_body0_e94385)), (locals.var_dnm_dn9 / (2.0 * assign60520_body0_e94385)), (locals.var_dnm_dn10 / (2.0 * assign60520_body0_e94385)), (locals.var_dnm_dn11 / (2.0 * assign60520_body0_e94385)), (locals.var_dnm_dn14 / (2.0 * assign60520_body0_e94385)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign60520_body0_e94387;
            locals.var_dnm_dn0 = assign60520_body0_e94387_d_n0;
            locals.var_dnm_dn2 = assign60520_body0_e94387_d_n2;
            locals.var_dnm_dn4 = assign60520_body0_e94387_d_n4;
            locals.var_dnm_dn5 = assign60520_body0_e94387_d_n5;
            locals.var_dnm_dn6 = assign60520_body0_e94387_d_n6;
            locals.var_dnm_dn7 = assign60520_body0_e94387_d_n7;
            locals.var_dnm_dn8 = assign60520_body0_e94387_d_n8;
            locals.var_dnm_dn9 = assign60520_body0_e94387_d_n9;
            locals.var_dnm_dn10 = assign60520_body0_e94387_d_n10;
            locals.var_dnm_dn11 = assign60520_body0_e94387_d_n11;
            locals.var_dnm_dn14 = assign60520_body0_e94387_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign60520_body1_e94405,) = {
    if ((((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) && (locals.var_guard1473 != 0.0)) && (locals.var_guard1474 != 0.0)) {
        let assign60520_body1_e94403: f64 = (locals.var_m0 + 1.0);
        (assign60520_body1_e94403,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign60520_body1_e94405;
            locals.var_m0_rv = 0.0;
        }

        let (assign60530_e94433, assign60530_e94433_d_n0, assign60530_e94433_d_n2, assign60530_e94433_d_n4, assign60530_e94433_d_n5, assign60530_e94433_d_n6, assign60530_e94433_d_n7, assign60530_e94433_d_n8, assign60530_e94433_d_n9, assign60530_e94433_d_n10, assign60530_e94433_d_n11, assign60530_e94433_d_n14,) = {
    if ((((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) && (locals.var_guard1473 != 0.0)) && (locals.var_guard1474 == 0.0)) {
        let (assign60530_e94431, assign60530_e94431_d_n0, assign60530_e94431_d_n2, assign60530_e94431_d_n4, assign60530_e94431_d_n5, assign60530_e94431_d_n6, assign60530_e94431_d_n7, assign60530_e94431_d_n8, assign60530_e94431_d_n9, assign60530_e94431_d_n10, assign60530_e94431_d_n11, assign60530_e94431_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign60530_e94428: f64 = (2.0 * 2.0);
                let assign60530_e94429: f64 = (1.0 / assign60530_e94428);
                let assign60530_e94430: f64 = (locals.var_dnm).powf(assign60530_e94429);
                (assign60530_e94430, if 0.0 == 0.0 && ((assign60530_e94429) as f64).is_finite() && ((assign60530_e94429) as f64).fract() == 0.0 { if assign60530_e94429 == 0.0 { 0.0 } else { (assign60530_e94429 * ((locals.var_dnm).powf(assign60530_e94429 - 1.0) * locals.var_dnm_dn0)) } } else { (assign60530_e94430 * (assign60530_e94429 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60530_e94429) as f64).is_finite() && ((assign60530_e94429) as f64).fract() == 0.0 { if assign60530_e94429 == 0.0 { 0.0 } else { (assign60530_e94429 * ((locals.var_dnm).powf(assign60530_e94429 - 1.0) * locals.var_dnm_dn2)) } } else { (assign60530_e94430 * (assign60530_e94429 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60530_e94429) as f64).is_finite() && ((assign60530_e94429) as f64).fract() == 0.0 { if assign60530_e94429 == 0.0 { 0.0 } else { (assign60530_e94429 * ((locals.var_dnm).powf(assign60530_e94429 - 1.0) * locals.var_dnm_dn4)) } } else { (assign60530_e94430 * (assign60530_e94429 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60530_e94429) as f64).is_finite() && ((assign60530_e94429) as f64).fract() == 0.0 { if assign60530_e94429 == 0.0 { 0.0 } else { (assign60530_e94429 * ((locals.var_dnm).powf(assign60530_e94429 - 1.0) * locals.var_dnm_dn5)) } } else { (assign60530_e94430 * (assign60530_e94429 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60530_e94429) as f64).is_finite() && ((assign60530_e94429) as f64).fract() == 0.0 { if assign60530_e94429 == 0.0 { 0.0 } else { (assign60530_e94429 * ((locals.var_dnm).powf(assign60530_e94429 - 1.0) * locals.var_dnm_dn6)) } } else { (assign60530_e94430 * (assign60530_e94429 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60530_e94429) as f64).is_finite() && ((assign60530_e94429) as f64).fract() == 0.0 { if assign60530_e94429 == 0.0 { 0.0 } else { (assign60530_e94429 * ((locals.var_dnm).powf(assign60530_e94429 - 1.0) * locals.var_dnm_dn7)) } } else { (assign60530_e94430 * (assign60530_e94429 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60530_e94429) as f64).is_finite() && ((assign60530_e94429) as f64).fract() == 0.0 { if assign60530_e94429 == 0.0 { 0.0 } else { (assign60530_e94429 * ((locals.var_dnm).powf(assign60530_e94429 - 1.0) * locals.var_dnm_dn8)) } } else { (assign60530_e94430 * (assign60530_e94429 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60530_e94429) as f64).is_finite() && ((assign60530_e94429) as f64).fract() == 0.0 { if assign60530_e94429 == 0.0 { 0.0 } else { (assign60530_e94429 * ((locals.var_dnm).powf(assign60530_e94429 - 1.0) * locals.var_dnm_dn9)) } } else { (assign60530_e94430 * (assign60530_e94429 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60530_e94429) as f64).is_finite() && ((assign60530_e94429) as f64).fract() == 0.0 { if assign60530_e94429 == 0.0 { 0.0 } else { (assign60530_e94429 * ((locals.var_dnm).powf(assign60530_e94429 - 1.0) * locals.var_dnm_dn10)) } } else { (assign60530_e94430 * (assign60530_e94429 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60530_e94429) as f64).is_finite() && ((assign60530_e94429) as f64).fract() == 0.0 { if assign60530_e94429 == 0.0 { 0.0 } else { (assign60530_e94429 * ((locals.var_dnm).powf(assign60530_e94429 - 1.0) * locals.var_dnm_dn11)) } } else { (assign60530_e94430 * (assign60530_e94429 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign60530_e94429) as f64).is_finite() && ((assign60530_e94429) as f64).fract() == 0.0 { if assign60530_e94429 == 0.0 { 0.0 } else { (assign60530_e94429 * ((locals.var_dnm).powf(assign60530_e94429 - 1.0) * locals.var_dnm_dn14)) } } else { (assign60530_e94430 * (assign60530_e94429 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign60530_e94431, assign60530_e94431_d_n0, assign60530_e94431_d_n2, assign60530_e94431_d_n4, assign60530_e94431_d_n5, assign60530_e94431_d_n6, assign60530_e94431_d_n7, assign60530_e94431_d_n8, assign60530_e94431_d_n9, assign60530_e94431_d_n10, assign60530_e94431_d_n11, assign60530_e94431_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign60530_e94433;
        locals.var_dnm_dn0 = assign60530_e94433_d_n0;
        locals.var_dnm_dn2 = assign60530_e94433_d_n2;
        locals.var_dnm_dn4 = assign60530_e94433_d_n4;
        locals.var_dnm_dn5 = assign60530_e94433_d_n5;
        locals.var_dnm_dn6 = assign60530_e94433_d_n6;
        locals.var_dnm_dn7 = assign60530_e94433_d_n7;
        locals.var_dnm_dn8 = assign60530_e94433_d_n8;
        locals.var_dnm_dn9 = assign60530_e94433_d_n9;
        locals.var_dnm_dn10 = assign60530_e94433_d_n10;
        locals.var_dnm_dn11 = assign60530_e94433_d_n11;
        locals.var_dnm_dn14 = assign60530_e94433_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign60540_e94449, assign60540_e94449_d_n0, assign60540_e94449_d_n2, assign60540_e94449_d_n4, assign60540_e94449_d_n5, assign60540_e94449_d_n6, assign60540_e94449_d_n7, assign60540_e94449_d_n8, assign60540_e94449_d_n9, assign60540_e94449_d_n10, assign60540_e94449_d_n11, assign60540_e94449_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) && (locals.var_guard1473 != 0.0)) {
        let assign60540_e94447: f64 = (1.0 / locals.var_dnm);
        (assign60540_e94447, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign60540_e94449;
        locals.var_dnm_dn0 = assign60540_e94449_d_n0;
        locals.var_dnm_dn2 = assign60540_e94449_d_n2;
        locals.var_dnm_dn4 = assign60540_e94449_d_n4;
        locals.var_dnm_dn5 = assign60540_e94449_d_n5;
        locals.var_dnm_dn6 = assign60540_e94449_d_n6;
        locals.var_dnm_dn7 = assign60540_e94449_d_n7;
        locals.var_dnm_dn8 = assign60540_e94449_d_n8;
        locals.var_dnm_dn9 = assign60540_e94449_d_n9;
        locals.var_dnm_dn10 = assign60540_e94449_d_n10;
        locals.var_dnm_dn11 = assign60540_e94449_d_n11;
        locals.var_dnm_dn14 = assign60540_e94449_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign60550_e94469, assign60550_e94469_d_n0, assign60550_e94469_d_n2, assign60550_e94469_d_n4, assign60550_e94469_d_n5, assign60550_e94469_d_n6, assign60550_e94469_d_n7, assign60550_e94469_d_n8, assign60550_e94469_d_n9, assign60550_e94469_d_n10, assign60550_e94469_d_n11, assign60550_e94469_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) && (locals.var_guard1473 != 0.0)) {
        let assign60550_e94464: f64 = (10.0 * 2.220446049250313e-16);
        let assign60550_e94465: f64 = (locals.var_tmf1 * assign60550_e94464);
        let assign60550_e94467: f64 = (assign60550_e94465 * locals.var_dnm);
        (assign60550_e94467, (((locals.var_tmf1_dn0 * assign60550_e94464) * locals.var_dnm) + (assign60550_e94465 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * assign60550_e94464) * locals.var_dnm) + (assign60550_e94465 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * assign60550_e94464) * locals.var_dnm) + (assign60550_e94465 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * assign60550_e94464) * locals.var_dnm) + (assign60550_e94465 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * assign60550_e94464) * locals.var_dnm) + (assign60550_e94465 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * assign60550_e94464) * locals.var_dnm) + (assign60550_e94465 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * assign60550_e94464) * locals.var_dnm) + (assign60550_e94465 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * assign60550_e94464) * locals.var_dnm) + (assign60550_e94465 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * assign60550_e94464) * locals.var_dnm) + (assign60550_e94465 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * assign60550_e94464) * locals.var_dnm) + (assign60550_e94465 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * assign60550_e94464) * locals.var_dnm) + (assign60550_e94465 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign60550_e94469;
        locals.var_tmf0_dn0 = assign60550_e94469_d_n0;
        locals.var_tmf0_dn2 = assign60550_e94469_d_n2;
        locals.var_tmf0_dn4 = assign60550_e94469_d_n4;
        locals.var_tmf0_dn5 = assign60550_e94469_d_n5;
        locals.var_tmf0_dn6 = assign60550_e94469_d_n6;
        locals.var_tmf0_dn7 = assign60550_e94469_d_n7;
        locals.var_tmf0_dn8 = assign60550_e94469_d_n8;
        locals.var_tmf0_dn9 = assign60550_e94469_d_n9;
        locals.var_tmf0_dn10 = assign60550_e94469_d_n10;
        locals.var_tmf0_dn11 = assign60550_e94469_d_n11;
        locals.var_tmf0_dn14 = assign60550_e94469_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign60560_e94491, assign60560_e94491_d_n0, assign60560_e94491_d_n2, assign60560_e94491_d_n4, assign60560_e94491_d_n5, assign60560_e94491_d_n6, assign60560_e94491_d_n7, assign60560_e94491_d_n8, assign60560_e94491_d_n9, assign60560_e94491_d_n10, assign60560_e94491_d_n11, assign60560_e94491_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) && (locals.var_guard1473 != 0.0)) {
        let assign60560_e94483: f64 = (10.0 * 2.220446049250313e-16);
        let assign60560_e94485: f64 = (assign60560_e94483 * locals.var_xmp);
        let assign60560_e94487: f64 = (assign60560_e94485 * locals.var_dnm);
        let assign60560_e94489: f64 = (assign60560_e94487 / locals.var_arg);
        (assign60560_e94489, ((((((assign60560_e94483 * locals.var_xmp_dn0) * locals.var_dnm) + (assign60560_e94485 * locals.var_dnm_dn0)) * locals.var_arg) - (assign60560_e94487 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((assign60560_e94483 * locals.var_xmp_dn2) * locals.var_dnm) + (assign60560_e94485 * locals.var_dnm_dn2)) * locals.var_arg) - (assign60560_e94487 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((assign60560_e94483 * locals.var_xmp_dn4) * locals.var_dnm) + (assign60560_e94485 * locals.var_dnm_dn4)) * locals.var_arg) - (assign60560_e94487 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((assign60560_e94483 * locals.var_xmp_dn5) * locals.var_dnm) + (assign60560_e94485 * locals.var_dnm_dn5)) * locals.var_arg) - (assign60560_e94487 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((assign60560_e94483 * locals.var_xmp_dn6) * locals.var_dnm) + (assign60560_e94485 * locals.var_dnm_dn6)) * locals.var_arg) - (assign60560_e94487 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((assign60560_e94483 * locals.var_xmp_dn7) * locals.var_dnm) + (assign60560_e94485 * locals.var_dnm_dn7)) * locals.var_arg) - (assign60560_e94487 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((assign60560_e94483 * locals.var_xmp_dn8) * locals.var_dnm) + (assign60560_e94485 * locals.var_dnm_dn8)) * locals.var_arg) - (assign60560_e94487 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((assign60560_e94483 * locals.var_xmp_dn9) * locals.var_dnm) + (assign60560_e94485 * locals.var_dnm_dn9)) * locals.var_arg) - (assign60560_e94487 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((assign60560_e94483 * locals.var_xmp_dn10) * locals.var_dnm) + (assign60560_e94485 * locals.var_dnm_dn10)) * locals.var_arg) - (assign60560_e94487 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((assign60560_e94483 * locals.var_xmp_dn11) * locals.var_dnm) + (assign60560_e94485 * locals.var_dnm_dn11)) * locals.var_arg) - (assign60560_e94487 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((assign60560_e94483 * locals.var_xmp_dn14) * locals.var_dnm) + (assign60560_e94485 * locals.var_dnm_dn14)) * locals.var_arg) - (assign60560_e94487 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign60560_e94491;
        locals.var_t0_dn0 = assign60560_e94491_d_n0;
        locals.var_t0_dn2 = assign60560_e94491_d_n2;
        locals.var_t0_dn4 = assign60560_e94491_d_n4;
        locals.var_t0_dn5 = assign60560_e94491_d_n5;
        locals.var_t0_dn6 = assign60560_e94491_d_n6;
        locals.var_t0_dn7 = assign60560_e94491_d_n7;
        locals.var_t0_dn8 = assign60560_e94491_d_n8;
        locals.var_t0_dn9 = assign60560_e94491_d_n9;
        locals.var_t0_dn10 = assign60560_e94491_d_n10;
        locals.var_t0_dn11 = assign60560_e94491_d_n11;
        locals.var_t0_dn14 = assign60560_e94491_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign60570_e94517, assign60570_e94517_d_n0, assign60570_e94517_d_n2, assign60570_e94517_d_n4, assign60570_e94517_d_n5, assign60570_e94517_d_n6, assign60570_e94517_d_n7, assign60570_e94517_d_n8, assign60570_e94517_d_n9, assign60570_e94517_d_n10, assign60570_e94517_d_n11, assign60570_e94517_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) && (locals.var_guard1473 != 0.0)) {
        let assign60570_e94505: f64 = (locals.var_ps0 + locals.var_vds);
        let assign60570_e94508: f64 = (10.0 * 2.220446049250313e-16);
        let assign60570_e94509: f64 = (assign60570_e94505 - assign60570_e94508);
        let assign60570_e94512: f64 = (10.0 * 2.220446049250313e-16);
        let assign60570_e94513: f64 = (assign60570_e94509 - assign60570_e94512);
        let assign60570_e94515: f64 = (assign60570_e94513 + locals.var_tmf0);
        (assign60570_e94515, ((locals.var_ps0_dn0 + locals.var_vds_dn0) + locals.var_tmf0_dn0), ((locals.var_ps0_dn2 + locals.var_vds_dn2) + locals.var_tmf0_dn2), ((locals.var_ps0_dn4 + locals.var_vds_dn4) + locals.var_tmf0_dn4), ((locals.var_ps0_dn5 + locals.var_vds_dn5) + locals.var_tmf0_dn5), ((locals.var_ps0_dn6 + locals.var_vds_dn6) + locals.var_tmf0_dn6), ((locals.var_ps0_dn7 + locals.var_vds_dn7) + locals.var_tmf0_dn7), ((locals.var_ps0_dn8 + locals.var_vds_dn8) + locals.var_tmf0_dn8), ((locals.var_ps0_dn9 + locals.var_vds_dn9) + locals.var_tmf0_dn9), ((locals.var_ps0_dn10 + locals.var_vds_dn10) + locals.var_tmf0_dn10), ((locals.var_ps0_dn11 + locals.var_vds_dn11) + locals.var_tmf0_dn11), ((locals.var_ps0_dn14 + locals.var_vds_dn14) + locals.var_tmf0_dn14),)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn14,)
    }
};
        locals.var_psdl = assign60570_e94517;
        locals.var_psdl_dn0 = assign60570_e94517_d_n0;
        locals.var_psdl_dn2 = assign60570_e94517_d_n2;
        locals.var_psdl_dn4 = assign60570_e94517_d_n4;
        locals.var_psdl_dn5 = assign60570_e94517_d_n5;
        locals.var_psdl_dn6 = assign60570_e94517_d_n6;
        locals.var_psdl_dn7 = assign60570_e94517_d_n7;
        locals.var_psdl_dn8 = assign60570_e94517_d_n8;
        locals.var_psdl_dn9 = assign60570_e94517_d_n9;
        locals.var_psdl_dn10 = assign60570_e94517_d_n10;
        locals.var_psdl_dn11 = assign60570_e94517_d_n11;
        locals.var_psdl_dn14 = assign60570_e94517_d_n14;
        locals.var_psdl_rv = 0.0;

        let (assign60580_e94531, assign60580_e94531_d_n0, assign60580_e94531_d_n2, assign60580_e94531_d_n4, assign60580_e94531_d_n5, assign60580_e94531_d_n6, assign60580_e94531_d_n7, assign60580_e94531_d_n8, assign60580_e94531_d_n9, assign60580_e94531_d_n10, assign60580_e94531_d_n11, assign60580_e94531_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) && (locals.var_guard1473 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign60580_e94531;
        locals.var_t0_dn0 = assign60580_e94531_d_n0;
        locals.var_t0_dn2 = assign60580_e94531_d_n2;
        locals.var_t0_dn4 = assign60580_e94531_d_n4;
        locals.var_t0_dn5 = assign60580_e94531_d_n5;
        locals.var_t0_dn6 = assign60580_e94531_d_n6;
        locals.var_t0_dn7 = assign60580_e94531_d_n7;
        locals.var_t0_dn8 = assign60580_e94531_d_n8;
        locals.var_t0_dn9 = assign60580_e94531_d_n9;
        locals.var_t0_dn10 = assign60580_e94531_d_n10;
        locals.var_t0_dn11 = assign60580_e94531_d_n11;
        locals.var_t0_dn14 = assign60580_e94531_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign60590_e94546, assign60590_e94546_d_n0, assign60590_e94546_d_n2, assign60590_e94546_d_n4, assign60590_e94546_d_n5, assign60590_e94546_d_n6, assign60590_e94546_d_n7, assign60590_e94546_d_n8, assign60590_e94546_d_n9, assign60590_e94546_d_n10, assign60590_e94546_d_n11, assign60590_e94546_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) && (locals.var_guard1473 == 0.0)) {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn14,)
    } else {
        (locals.var_psdl, locals.var_psdl_dn0, locals.var_psdl_dn2, locals.var_psdl_dn4, locals.var_psdl_dn5, locals.var_psdl_dn6, locals.var_psdl_dn7, locals.var_psdl_dn8, locals.var_psdl_dn9, locals.var_psdl_dn10, locals.var_psdl_dn11, locals.var_psdl_dn14,)
    }
};
        locals.var_psdl = assign60590_e94546;
        locals.var_psdl_dn0 = assign60590_e94546_d_n0;
        locals.var_psdl_dn2 = assign60590_e94546_d_n2;
        locals.var_psdl_dn4 = assign60590_e94546_d_n4;
        locals.var_psdl_dn5 = assign60590_e94546_d_n5;
        locals.var_psdl_dn6 = assign60590_e94546_d_n6;
        locals.var_psdl_dn7 = assign60590_e94546_d_n7;
        locals.var_psdl_dn8 = assign60590_e94546_d_n8;
        locals.var_psdl_dn9 = assign60590_e94546_d_n9;
        locals.var_psdl_dn10 = assign60590_e94546_d_n10;
        locals.var_psdl_dn11 = assign60590_e94546_d_n11;
        locals.var_psdl_dn14 = assign60590_e94546_d_n14;
        locals.var_psdl_rv = 0.0;

        let (assign60600_e94561, assign60600_e94561_d_n0, assign60600_e94561_d_n2, assign60600_e94561_d_n4, assign60600_e94561_d_n5, assign60600_e94561_d_n6, assign60600_e94561_d_n7, assign60600_e94561_d_n8, assign60600_e94561_d_n9, assign60600_e94561_d_n10, assign60600_e94561_d_n11, assign60600_e94561_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) && (locals.var_guard1473 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign60600_e94561;
        locals.var_t0_dn0 = assign60600_e94561_d_n0;
        locals.var_t0_dn2 = assign60600_e94561_d_n2;
        locals.var_t0_dn4 = assign60600_e94561_d_n4;
        locals.var_t0_dn5 = assign60600_e94561_d_n5;
        locals.var_t0_dn6 = assign60600_e94561_d_n6;
        locals.var_t0_dn7 = assign60600_e94561_d_n7;
        locals.var_t0_dn8 = assign60600_e94561_d_n8;
        locals.var_t0_dn9 = assign60600_e94561_d_n9;
        locals.var_t0_dn10 = assign60600_e94561_d_n10;
        locals.var_t0_dn11 = assign60600_e94561_d_n11;
        locals.var_t0_dn14 = assign60600_e94561_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign60610_e94575, assign60610_e94575_d_n0, assign60610_e94575_d_n2, assign60610_e94575_d_n4, assign60610_e94575_d_n5, assign60610_e94575_d_n6, assign60610_e94575_d_n7, assign60610_e94575_d_n8, assign60610_e94575_d_n9, assign60610_e94575_d_n10, assign60610_e94575_d_n11, assign60610_e94575_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) {
        let assign60610_e94573: f64 = (locals.var_psdl - locals.var_psl);
        (assign60610_e94573, (locals.var_psdl_dn0 - locals.var_psl_dn0), (locals.var_psdl_dn2 - locals.var_psl_dn2), (locals.var_psdl_dn4 - locals.var_psl_dn4), (locals.var_psdl_dn5 - locals.var_psl_dn5), (locals.var_psdl_dn6 - locals.var_psl_dn6), (locals.var_psdl_dn7 - locals.var_psl_dn7), (locals.var_psdl_dn8 - locals.var_psl_dn8), (locals.var_psdl_dn9 - locals.var_psl_dn9), (locals.var_psdl_dn10 - locals.var_psl_dn10), (locals.var_psdl_dn11 - locals.var_psl_dn11), (locals.var_psdl_dn14 - locals.var_psl_dn14),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign60610_e94575;
        locals.var_t6_dn0 = assign60610_e94575_d_n0;
        locals.var_t6_dn2 = assign60610_e94575_d_n2;
        locals.var_t6_dn4 = assign60610_e94575_d_n4;
        locals.var_t6_dn5 = assign60610_e94575_d_n5;
        locals.var_t6_dn6 = assign60610_e94575_d_n6;
        locals.var_t6_dn7 = assign60610_e94575_d_n7;
        locals.var_t6_dn8 = assign60610_e94575_d_n8;
        locals.var_t6_dn9 = assign60610_e94575_d_n9;
        locals.var_t6_dn10 = assign60610_e94575_d_n10;
        locals.var_t6_dn11 = assign60610_e94575_d_n11;
        locals.var_t6_dn14 = assign60610_e94575_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign60620_e94589, assign60620_e94589_d_n0, assign60620_e94589_d_n2, assign60620_e94589_d_n4, assign60620_e94589_d_n5, assign60620_e94589_d_n6, assign60620_e94589_d_n7, assign60620_e94589_d_n8, assign60620_e94589_d_n9, assign60620_e94589_d_n10, assign60620_e94589_d_n11, assign60620_e94589_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) {
        let assign60620_e94587: f64 = (locals.var_beta * locals.var_qn0);
        (assign60620_e94587, ((locals.var_beta_dn0 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn0)), ((locals.var_beta_dn2 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn2)), ((locals.var_beta_dn4 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn4)), ((locals.var_beta_dn5 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn5)), ((locals.var_beta_dn6 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn6)), ((locals.var_beta_dn7 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn7)), ((locals.var_beta_dn8 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn8)), ((locals.var_beta_dn9 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn9)), ((locals.var_beta_dn10 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn10)), ((locals.var_beta_dn11 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn11)), ((locals.var_beta_dn14 * locals.var_qn0) + (locals.var_beta * locals.var_qn0_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign60620_e94589;
        locals.var_t3_dn0 = assign60620_e94589_d_n0;
        locals.var_t3_dn2 = assign60620_e94589_d_n2;
        locals.var_t3_dn4 = assign60620_e94589_d_n4;
        locals.var_t3_dn5 = assign60620_e94589_d_n5;
        locals.var_t3_dn6 = assign60620_e94589_d_n6;
        locals.var_t3_dn7 = assign60620_e94589_d_n7;
        locals.var_t3_dn8 = assign60620_e94589_d_n8;
        locals.var_t3_dn9 = assign60620_e94589_d_n9;
        locals.var_t3_dn10 = assign60620_e94589_d_n10;
        locals.var_t3_dn11 = assign60620_e94589_d_n11;
        locals.var_t3_dn14 = assign60620_e94589_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign60630_e94603, assign60630_e94603_d_n0, assign60630_e94603_d_n2, assign60630_e94603_d_n4, assign60630_e94603_d_n5, assign60630_e94603_d_n6, assign60630_e94603_d_n7, assign60630_e94603_d_n8, assign60630_e94603_d_n9, assign60630_e94603_d_n10, assign60630_e94603_d_n11, assign60630_e94603_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) {
        let assign60630_e94601: f64 = (1.0 / locals.var_t3);
        (assign60630_e94601, (-(locals.var_t3_dn0 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn2 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn14 / (locals.var_t3 * locals.var_t3))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign60630_e94603;
        locals.var_t1_dn0 = assign60630_e94603_d_n0;
        locals.var_t1_dn2 = assign60630_e94603_d_n2;
        locals.var_t1_dn4 = assign60630_e94603_d_n4;
        locals.var_t1_dn5 = assign60630_e94603_d_n5;
        locals.var_t1_dn6 = assign60630_e94603_d_n6;
        locals.var_t1_dn7 = assign60630_e94603_d_n7;
        locals.var_t1_dn8 = assign60630_e94603_d_n8;
        locals.var_t1_dn9 = assign60630_e94603_d_n9;
        locals.var_t1_dn10 = assign60630_e94603_d_n10;
        locals.var_t1_dn11 = assign60630_e94603_d_n11;
        locals.var_t1_dn14 = assign60630_e94603_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign60640_e94623, assign60640_e94623_d_n0, assign60640_e94623_d_n2, assign60640_e94623_d_n4, assign60640_e94623_d_n5, assign60640_e94623_d_n6, assign60640_e94623_d_n7, assign60640_e94623_d_n8, assign60640_e94623_d_n9, assign60640_e94623_d_n10, assign60640_e94623_d_n11, assign60640_e94623_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) {
        let assign60640_e94616: f64 = (10.0 * 2.220446049250313e-16);
        let assign60640_e94617: f64 = (locals.var_pds + assign60640_e94616);
        let assign60640_e94619: f64 = (assign60640_e94617 * locals.var_fdd);
        let assign60640_e94621: f64 = (assign60640_e94619 * locals.var_t1);
        (assign60640_e94621, ((((locals.var_pds_dn0 * locals.var_fdd) + (assign60640_e94617 * locals.var_fdd_dn0)) * locals.var_t1) + (assign60640_e94619 * locals.var_t1_dn0)), ((((locals.var_pds_dn2 * locals.var_fdd) + (assign60640_e94617 * locals.var_fdd_dn2)) * locals.var_t1) + (assign60640_e94619 * locals.var_t1_dn2)), ((((locals.var_pds_dn4 * locals.var_fdd) + (assign60640_e94617 * locals.var_fdd_dn4)) * locals.var_t1) + (assign60640_e94619 * locals.var_t1_dn4)), ((((locals.var_pds_dn5 * locals.var_fdd) + (assign60640_e94617 * locals.var_fdd_dn5)) * locals.var_t1) + (assign60640_e94619 * locals.var_t1_dn5)), ((((locals.var_pds_dn6 * locals.var_fdd) + (assign60640_e94617 * locals.var_fdd_dn6)) * locals.var_t1) + (assign60640_e94619 * locals.var_t1_dn6)), ((((locals.var_pds_dn7 * locals.var_fdd) + (assign60640_e94617 * locals.var_fdd_dn7)) * locals.var_t1) + (assign60640_e94619 * locals.var_t1_dn7)), ((((locals.var_pds_dn8 * locals.var_fdd) + (assign60640_e94617 * locals.var_fdd_dn8)) * locals.var_t1) + (assign60640_e94619 * locals.var_t1_dn8)), ((((locals.var_pds_dn9 * locals.var_fdd) + (assign60640_e94617 * locals.var_fdd_dn9)) * locals.var_t1) + (assign60640_e94619 * locals.var_t1_dn9)), ((((locals.var_pds_dn10 * locals.var_fdd) + (assign60640_e94617 * locals.var_fdd_dn10)) * locals.var_t1) + (assign60640_e94619 * locals.var_t1_dn10)), ((((locals.var_pds_dn11 * locals.var_fdd) + (assign60640_e94617 * locals.var_fdd_dn11)) * locals.var_t1) + (assign60640_e94619 * locals.var_t1_dn11)), ((((locals.var_pds_dn14 * locals.var_fdd) + (assign60640_e94617 * locals.var_fdd_dn14)) * locals.var_t1) + (assign60640_e94619 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign60640_e94623;
        locals.var_t5_dn0 = assign60640_e94623_d_n0;
        locals.var_t5_dn2 = assign60640_e94623_d_n2;
        locals.var_t5_dn4 = assign60640_e94623_d_n4;
        locals.var_t5_dn5 = assign60640_e94623_d_n5;
        locals.var_t5_dn6 = assign60640_e94623_d_n6;
        locals.var_t5_dn7 = assign60640_e94623_d_n7;
        locals.var_t5_dn8 = assign60640_e94623_d_n8;
        locals.var_t5_dn9 = assign60640_e94623_d_n9;
        locals.var_t5_dn10 = assign60640_e94623_d_n10;
        locals.var_t5_dn11 = assign60640_e94623_d_n11;
        locals.var_t5_dn14 = assign60640_e94623_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign60650_e94637, assign60650_e94637_d_n0, assign60650_e94637_d_n2, assign60650_e94637_d_n4, assign60650_e94637_d_n5, assign60650_e94637_d_n6, assign60650_e94637_d_n7, assign60650_e94637_d_n8, assign60650_e94637_d_n9, assign60650_e94637_d_n10, assign60650_e94637_d_n11, assign60650_e94637_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) {
        let assign60650_e94635: f64 = (locals.var_t5 * locals.var_beta);
        (assign60650_e94635, ((locals.var_t5_dn0 * locals.var_beta) + (locals.var_t5 * locals.var_beta_dn0)), ((locals.var_t5_dn2 * locals.var_beta) + (locals.var_t5 * locals.var_beta_dn2)), ((locals.var_t5_dn4 * locals.var_beta) + (locals.var_t5 * locals.var_beta_dn4)), ((locals.var_t5_dn5 * locals.var_beta) + (locals.var_t5 * locals.var_beta_dn5)), ((locals.var_t5_dn6 * locals.var_beta) + (locals.var_t5 * locals.var_beta_dn6)), ((locals.var_t5_dn7 * locals.var_beta) + (locals.var_t5 * locals.var_beta_dn7)), ((locals.var_t5_dn8 * locals.var_beta) + (locals.var_t5 * locals.var_beta_dn8)), ((locals.var_t5_dn9 * locals.var_beta) + (locals.var_t5 * locals.var_beta_dn9)), ((locals.var_t5_dn10 * locals.var_beta) + (locals.var_t5 * locals.var_beta_dn10)), ((locals.var_t5_dn11 * locals.var_beta) + (locals.var_t5 * locals.var_beta_dn11)), ((locals.var_t5_dn14 * locals.var_beta) + (locals.var_t5 * locals.var_beta_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign60650_e94637;
        locals.var_t2_dn0 = assign60650_e94637_d_n0;
        locals.var_t2_dn2 = assign60650_e94637_d_n2;
        locals.var_t2_dn4 = assign60650_e94637_d_n4;
        locals.var_t2_dn5 = assign60650_e94637_d_n5;
        locals.var_t2_dn6 = assign60650_e94637_d_n6;
        locals.var_t2_dn7 = assign60650_e94637_d_n7;
        locals.var_t2_dn8 = assign60650_e94637_d_n8;
        locals.var_t2_dn9 = assign60650_e94637_d_n9;
        locals.var_t2_dn10 = assign60650_e94637_d_n10;
        locals.var_t2_dn11 = assign60650_e94637_d_n11;
        locals.var_t2_dn14 = assign60650_e94637_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign60660_e94651, assign60660_e94651_d_n0, assign60660_e94651_d_n2, assign60660_e94651_d_n4, assign60660_e94651_d_n5, assign60660_e94651_d_n6, assign60660_e94651_d_n7, assign60660_e94651_d_n8, assign60660_e94651_d_n9, assign60660_e94651_d_n10, assign60660_e94651_d_n11, assign60660_e94651_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) {
        let assign60660_e94649: f64 = (locals.var_q_nsub / 1.034943e-10);
        (assign60660_e94649, (locals.var_q_nsub_dn0 / 1.034943e-10), (locals.var_q_nsub_dn2 / 1.034943e-10), (locals.var_q_nsub_dn4 / 1.034943e-10), (locals.var_q_nsub_dn5 / 1.034943e-10), (locals.var_q_nsub_dn6 / 1.034943e-10), (locals.var_q_nsub_dn7 / 1.034943e-10), (locals.var_q_nsub_dn8 / 1.034943e-10), (locals.var_q_nsub_dn9 / 1.034943e-10), (locals.var_q_nsub_dn10 / 1.034943e-10), (locals.var_q_nsub_dn11 / 1.034943e-10), (locals.var_q_nsub_dn14 / 1.034943e-10),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign60660_e94651;
        locals.var_t10_dn0 = assign60660_e94651_d_n0;
        locals.var_t10_dn2 = assign60660_e94651_d_n2;
        locals.var_t10_dn4 = assign60660_e94651_d_n4;
        locals.var_t10_dn5 = assign60660_e94651_d_n5;
        locals.var_t10_dn6 = assign60660_e94651_d_n6;
        locals.var_t10_dn7 = assign60660_e94651_d_n7;
        locals.var_t10_dn8 = assign60660_e94651_d_n8;
        locals.var_t10_dn9 = assign60660_e94651_d_n9;
        locals.var_t10_dn10 = assign60660_e94651_d_n10;
        locals.var_t10_dn11 = assign60660_e94651_d_n11;
        locals.var_t10_dn14 = assign60660_e94651_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign60670_e94663, assign60670_e94663_d_n0, assign60670_e94663_d_n2, assign60670_e94663_d_n4, assign60670_e94663_d_n5, assign60670_e94663_d_n6, assign60670_e94663_d_n7, assign60670_e94663_d_n8, assign60670_e94663_d_n9, assign60670_e94663_d_n10, assign60670_e94663_d_n11, assign60670_e94663_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) {
        (100000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign60670_e94663;
        locals.var_t1_dn0 = assign60670_e94663_d_n0;
        locals.var_t1_dn2 = assign60670_e94663_d_n2;
        locals.var_t1_dn4 = assign60670_e94663_d_n4;
        locals.var_t1_dn5 = assign60670_e94663_d_n5;
        locals.var_t1_dn6 = assign60670_e94663_d_n6;
        locals.var_t1_dn7 = assign60670_e94663_d_n7;
        locals.var_t1_dn8 = assign60670_e94663_d_n8;
        locals.var_t1_dn9 = assign60670_e94663_d_n9;
        locals.var_t1_dn10 = assign60670_e94663_d_n10;
        locals.var_t1_dn11 = assign60670_e94663_d_n11;
        locals.var_t1_dn14 = assign60670_e94663_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign60680_e94677, assign60680_e94677_d_n0, assign60680_e94677_d_n2, assign60680_e94677_d_n4, assign60680_e94677_d_n5, assign60680_e94677_d_n6, assign60680_e94677_d_n7, assign60680_e94677_d_n8, assign60680_e94677_d_n9, assign60680_e94677_d_n10, assign60680_e94677_d_n11, assign60680_e94677_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) {
        let assign60680_e94675: f64 = (1.0 / locals.var_leff);
        (assign60680_e94675, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign60680_e94677;
        locals.var_t2_dn0 = assign60680_e94677_d_n0;
        locals.var_t2_dn2 = assign60680_e94677_d_n2;
        locals.var_t2_dn4 = assign60680_e94677_d_n4;
        locals.var_t2_dn5 = assign60680_e94677_d_n5;
        locals.var_t2_dn6 = assign60680_e94677_d_n6;
        locals.var_t2_dn7 = assign60680_e94677_d_n7;
        locals.var_t2_dn8 = assign60680_e94677_d_n8;
        locals.var_t2_dn9 = assign60680_e94677_d_n9;
        locals.var_t2_dn10 = assign60680_e94677_d_n10;
        locals.var_t2_dn11 = assign60680_e94677_d_n11;
        locals.var_t2_dn14 = assign60680_e94677_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign60690_e94705, assign60690_e94705_d_n0, assign60690_e94705_d_n2, assign60690_e94705_d_n4, assign60690_e94705_d_n5, assign60690_e94705_d_n6, assign60690_e94705_d_n7, assign60690_e94705_d_n8, assign60690_e94705_d_n9, assign60690_e94705_d_n10, assign60690_e94705_d_n11, assign60690_e94705_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) {
        let assign60690_e94689: f64 = (2.0 * locals.var_t5);
        let assign60690_e94692: f64 = (2.0 * locals.var_t10);
        let assign60690_e94694: f64 = (assign60690_e94692 * locals.var_t6);
        let assign60690_e94696: f64 = (assign60690_e94694 * locals.var_t4);
        let assign60690_e94697: f64 = (assign60690_e94689 + assign60690_e94696);
        let assign60690_e94700: f64 = (locals.var_t1 * locals.var_t4);
        let assign60690_e94701: f64 = (assign60690_e94697 + assign60690_e94700);
        let assign60690_e94703: f64 = (assign60690_e94701 * locals.var_t2);
        (assign60690_e94703, (((((2.0 * locals.var_t5_dn0) + (((((2.0 * locals.var_t10_dn0) * locals.var_t6) + (assign60690_e94692 * locals.var_t6_dn0)) * locals.var_t4) + (assign60690_e94694 * locals.var_t4_dn0))) + ((locals.var_t1_dn0 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn0))) * locals.var_t2) + (assign60690_e94701 * locals.var_t2_dn0)), (((((2.0 * locals.var_t5_dn2) + (((((2.0 * locals.var_t10_dn2) * locals.var_t6) + (assign60690_e94692 * locals.var_t6_dn2)) * locals.var_t4) + (assign60690_e94694 * locals.var_t4_dn2))) + ((locals.var_t1_dn2 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn2))) * locals.var_t2) + (assign60690_e94701 * locals.var_t2_dn2)), (((((2.0 * locals.var_t5_dn4) + (((((2.0 * locals.var_t10_dn4) * locals.var_t6) + (assign60690_e94692 * locals.var_t6_dn4)) * locals.var_t4) + (assign60690_e94694 * locals.var_t4_dn4))) + ((locals.var_t1_dn4 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn4))) * locals.var_t2) + (assign60690_e94701 * locals.var_t2_dn4)), (((((2.0 * locals.var_t5_dn5) + (((((2.0 * locals.var_t10_dn5) * locals.var_t6) + (assign60690_e94692 * locals.var_t6_dn5)) * locals.var_t4) + (assign60690_e94694 * locals.var_t4_dn5))) + ((locals.var_t1_dn5 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn5))) * locals.var_t2) + (assign60690_e94701 * locals.var_t2_dn5)), (((((2.0 * locals.var_t5_dn6) + (((((2.0 * locals.var_t10_dn6) * locals.var_t6) + (assign60690_e94692 * locals.var_t6_dn6)) * locals.var_t4) + (assign60690_e94694 * locals.var_t4_dn6))) + ((locals.var_t1_dn6 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn6))) * locals.var_t2) + (assign60690_e94701 * locals.var_t2_dn6)), (((((2.0 * locals.var_t5_dn7) + (((((2.0 * locals.var_t10_dn7) * locals.var_t6) + (assign60690_e94692 * locals.var_t6_dn7)) * locals.var_t4) + (assign60690_e94694 * locals.var_t4_dn7))) + ((locals.var_t1_dn7 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn7))) * locals.var_t2) + (assign60690_e94701 * locals.var_t2_dn7)), (((((2.0 * locals.var_t5_dn8) + (((((2.0 * locals.var_t10_dn8) * locals.var_t6) + (assign60690_e94692 * locals.var_t6_dn8)) * locals.var_t4) + (assign60690_e94694 * locals.var_t4_dn8))) + ((locals.var_t1_dn8 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn8))) * locals.var_t2) + (assign60690_e94701 * locals.var_t2_dn8)), (((((2.0 * locals.var_t5_dn9) + (((((2.0 * locals.var_t10_dn9) * locals.var_t6) + (assign60690_e94692 * locals.var_t6_dn9)) * locals.var_t4) + (assign60690_e94694 * locals.var_t4_dn9))) + ((locals.var_t1_dn9 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn9))) * locals.var_t2) + (assign60690_e94701 * locals.var_t2_dn9)), (((((2.0 * locals.var_t5_dn10) + (((((2.0 * locals.var_t10_dn10) * locals.var_t6) + (assign60690_e94692 * locals.var_t6_dn10)) * locals.var_t4) + (assign60690_e94694 * locals.var_t4_dn10))) + ((locals.var_t1_dn10 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn10))) * locals.var_t2) + (assign60690_e94701 * locals.var_t2_dn10)), (((((2.0 * locals.var_t5_dn11) + (((((2.0 * locals.var_t10_dn11) * locals.var_t6) + (assign60690_e94692 * locals.var_t6_dn11)) * locals.var_t4) + (assign60690_e94694 * locals.var_t4_dn11))) + ((locals.var_t1_dn11 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn11))) * locals.var_t2) + (assign60690_e94701 * locals.var_t2_dn11)), (((((2.0 * locals.var_t5_dn14) + (((((2.0 * locals.var_t10_dn14) * locals.var_t6) + (assign60690_e94692 * locals.var_t6_dn14)) * locals.var_t4) + (assign60690_e94694 * locals.var_t4_dn14))) + ((locals.var_t1_dn14 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn14))) * locals.var_t2) + (assign60690_e94701 * locals.var_t2_dn14)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn14,)
    }
};
        locals.var_t11 = assign60690_e94705;
        locals.var_t11_dn0 = assign60690_e94705_d_n0;
        locals.var_t11_dn2 = assign60690_e94705_d_n2;
        locals.var_t11_dn4 = assign60690_e94705_d_n4;
        locals.var_t11_dn5 = assign60690_e94705_d_n5;
        locals.var_t11_dn6 = assign60690_e94705_d_n6;
        locals.var_t11_dn7 = assign60690_e94705_d_n7;
        locals.var_t11_dn8 = assign60690_e94705_d_n8;
        locals.var_t11_dn9 = assign60690_e94705_d_n9;
        locals.var_t11_dn10 = assign60690_e94705_d_n10;
        locals.var_t11_dn11 = assign60690_e94705_d_n11;
        locals.var_t11_dn14 = assign60690_e94705_d_n14;
        locals.var_t11_rv = 0.0;

        let (assign60700_e94719, assign60700_e94719_d_n0, assign60700_e94719_d_n2, assign60700_e94719_d_n4, assign60700_e94719_d_n5, assign60700_e94719_d_n6, assign60700_e94719_d_n7, assign60700_e94719_d_n8, assign60700_e94719_d_n9, assign60700_e94719_d_n10, assign60700_e94719_d_n11, assign60700_e94719_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) {
        let assign60700_e94717: f64 = (locals.var_t2 * locals.var_t4);
        (assign60700_e94717, ((locals.var_t2_dn0 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn0)), ((locals.var_t2_dn2 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn2)), ((locals.var_t2_dn4 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn4)), ((locals.var_t2_dn5 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn5)), ((locals.var_t2_dn6 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn6)), ((locals.var_t2_dn7 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn7)), ((locals.var_t2_dn8 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn8)), ((locals.var_t2_dn9 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn9)), ((locals.var_t2_dn10 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn10)), ((locals.var_t2_dn11 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn11)), ((locals.var_t2_dn14 * locals.var_t4) + (locals.var_t2 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign60700_e94719;
        locals.var_t3_dn0 = assign60700_e94719_d_n0;
        locals.var_t3_dn2 = assign60700_e94719_d_n2;
        locals.var_t3_dn4 = assign60700_e94719_d_n4;
        locals.var_t3_dn5 = assign60700_e94719_d_n5;
        locals.var_t3_dn6 = assign60700_e94719_d_n6;
        locals.var_t3_dn7 = assign60700_e94719_d_n7;
        locals.var_t3_dn8 = assign60700_e94719_d_n8;
        locals.var_t3_dn9 = assign60700_e94719_d_n9;
        locals.var_t3_dn10 = assign60700_e94719_d_n10;
        locals.var_t3_dn11 = assign60700_e94719_d_n11;
        locals.var_t3_dn14 = assign60700_e94719_d_n14;
        locals.var_t3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_226(
        locals: &mut StampLocals,
    ) {
        let (assign60710_e94733, assign60710_e94733_d_n0, assign60710_e94733_d_n2, assign60710_e94733_d_n4, assign60710_e94733_d_n5, assign60710_e94733_d_n6, assign60710_e94733_d_n7, assign60710_e94733_d_n8, assign60710_e94733_d_n9, assign60710_e94733_d_n10, assign60710_e94733_d_n11, assign60710_e94733_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) {
        let assign60710_e94731: f64 = (locals.var_t11 * locals.var_t4);
        (assign60710_e94731, ((locals.var_t11_dn0 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn0)), ((locals.var_t11_dn2 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn2)), ((locals.var_t11_dn4 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn4)), ((locals.var_t11_dn5 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn5)), ((locals.var_t11_dn6 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn6)), ((locals.var_t11_dn7 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn7)), ((locals.var_t11_dn8 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn8)), ((locals.var_t11_dn9 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn9)), ((locals.var_t11_dn10 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn10)), ((locals.var_t11_dn11 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn11)), ((locals.var_t11_dn14 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign60710_e94733;
        locals.var_t7_dn0 = assign60710_e94733_d_n0;
        locals.var_t7_dn2 = assign60710_e94733_d_n2;
        locals.var_t7_dn4 = assign60710_e94733_d_n4;
        locals.var_t7_dn5 = assign60710_e94733_d_n5;
        locals.var_t7_dn6 = assign60710_e94733_d_n6;
        locals.var_t7_dn7 = assign60710_e94733_d_n7;
        locals.var_t7_dn8 = assign60710_e94733_d_n8;
        locals.var_t7_dn9 = assign60710_e94733_d_n9;
        locals.var_t7_dn10 = assign60710_e94733_d_n10;
        locals.var_t7_dn11 = assign60710_e94733_d_n11;
        locals.var_t7_dn14 = assign60710_e94733_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign60720_e94753, assign60720_e94753_d_n0, assign60720_e94753_d_n2, assign60720_e94753_d_n4, assign60720_e94753_d_n5, assign60720_e94753_d_n6, assign60720_e94753_d_n7, assign60720_e94753_d_n8, assign60720_e94753_d_n9, assign60720_e94753_d_n10, assign60720_e94753_d_n11, assign60720_e94753_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) {
        let assign60720_e94746: f64 = (2.0 * locals.var_t10);
        let assign60720_e94748: f64 = (assign60720_e94746 * locals.var_t6);
        let assign60720_e94750: f64 = (assign60720_e94748 + locals.var_t1);
        let assign60720_e94751: f64 = (4.0 * assign60720_e94750);
        (assign60720_e94751, (4.0 * ((((2.0 * locals.var_t10_dn0) * locals.var_t6) + (assign60720_e94746 * locals.var_t6_dn0)) + locals.var_t1_dn0)), (4.0 * ((((2.0 * locals.var_t10_dn2) * locals.var_t6) + (assign60720_e94746 * locals.var_t6_dn2)) + locals.var_t1_dn2)), (4.0 * ((((2.0 * locals.var_t10_dn4) * locals.var_t6) + (assign60720_e94746 * locals.var_t6_dn4)) + locals.var_t1_dn4)), (4.0 * ((((2.0 * locals.var_t10_dn5) * locals.var_t6) + (assign60720_e94746 * locals.var_t6_dn5)) + locals.var_t1_dn5)), (4.0 * ((((2.0 * locals.var_t10_dn6) * locals.var_t6) + (assign60720_e94746 * locals.var_t6_dn6)) + locals.var_t1_dn6)), (4.0 * ((((2.0 * locals.var_t10_dn7) * locals.var_t6) + (assign60720_e94746 * locals.var_t6_dn7)) + locals.var_t1_dn7)), (4.0 * ((((2.0 * locals.var_t10_dn8) * locals.var_t6) + (assign60720_e94746 * locals.var_t6_dn8)) + locals.var_t1_dn8)), (4.0 * ((((2.0 * locals.var_t10_dn9) * locals.var_t6) + (assign60720_e94746 * locals.var_t6_dn9)) + locals.var_t1_dn9)), (4.0 * ((((2.0 * locals.var_t10_dn10) * locals.var_t6) + (assign60720_e94746 * locals.var_t6_dn10)) + locals.var_t1_dn10)), (4.0 * ((((2.0 * locals.var_t10_dn11) * locals.var_t6) + (assign60720_e94746 * locals.var_t6_dn11)) + locals.var_t1_dn11)), (4.0 * ((((2.0 * locals.var_t10_dn14) * locals.var_t6) + (assign60720_e94746 * locals.var_t6_dn14)) + locals.var_t1_dn14)),)
    } else {
        (locals.var_t11, locals.var_t11_dn0, locals.var_t11_dn2, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn14,)
    }
};
        locals.var_t11 = assign60720_e94753;
        locals.var_t11_dn0 = assign60720_e94753_d_n0;
        locals.var_t11_dn2 = assign60720_e94753_d_n2;
        locals.var_t11_dn4 = assign60720_e94753_d_n4;
        locals.var_t11_dn5 = assign60720_e94753_d_n5;
        locals.var_t11_dn6 = assign60720_e94753_d_n6;
        locals.var_t11_dn7 = assign60720_e94753_d_n7;
        locals.var_t11_dn8 = assign60720_e94753_d_n8;
        locals.var_t11_dn9 = assign60720_e94753_d_n9;
        locals.var_t11_dn10 = assign60720_e94753_d_n10;
        locals.var_t11_dn11 = assign60720_e94753_d_n11;
        locals.var_t11_dn14 = assign60720_e94753_d_n14;
        locals.var_t11_rv = 0.0;

        let (assign60730_e94771, assign60730_e94771_d_n0, assign60730_e94771_d_n2, assign60730_e94771_d_n4, assign60730_e94771_d_n5, assign60730_e94771_d_n6, assign60730_e94771_d_n7, assign60730_e94771_d_n8, assign60730_e94771_d_n9, assign60730_e94771_d_n10, assign60730_e94771_d_n11, assign60730_e94771_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) {
        let assign60730_e94765: f64 = (8.0 * locals.var_t10);
        let assign60730_e94767: f64 = (assign60730_e94765 * locals.var_t4);
        let assign60730_e94769: f64 = (assign60730_e94767 * locals.var_t4);
        (assign60730_e94769, (((((8.0 * locals.var_t10_dn0) * locals.var_t4) + (assign60730_e94765 * locals.var_t4_dn0)) * locals.var_t4) + (assign60730_e94767 * locals.var_t4_dn0)), (((((8.0 * locals.var_t10_dn2) * locals.var_t4) + (assign60730_e94765 * locals.var_t4_dn2)) * locals.var_t4) + (assign60730_e94767 * locals.var_t4_dn2)), (((((8.0 * locals.var_t10_dn4) * locals.var_t4) + (assign60730_e94765 * locals.var_t4_dn4)) * locals.var_t4) + (assign60730_e94767 * locals.var_t4_dn4)), (((((8.0 * locals.var_t10_dn5) * locals.var_t4) + (assign60730_e94765 * locals.var_t4_dn5)) * locals.var_t4) + (assign60730_e94767 * locals.var_t4_dn5)), (((((8.0 * locals.var_t10_dn6) * locals.var_t4) + (assign60730_e94765 * locals.var_t4_dn6)) * locals.var_t4) + (assign60730_e94767 * locals.var_t4_dn6)), (((((8.0 * locals.var_t10_dn7) * locals.var_t4) + (assign60730_e94765 * locals.var_t4_dn7)) * locals.var_t4) + (assign60730_e94767 * locals.var_t4_dn7)), (((((8.0 * locals.var_t10_dn8) * locals.var_t4) + (assign60730_e94765 * locals.var_t4_dn8)) * locals.var_t4) + (assign60730_e94767 * locals.var_t4_dn8)), (((((8.0 * locals.var_t10_dn9) * locals.var_t4) + (assign60730_e94765 * locals.var_t4_dn9)) * locals.var_t4) + (assign60730_e94767 * locals.var_t4_dn9)), (((((8.0 * locals.var_t10_dn10) * locals.var_t4) + (assign60730_e94765 * locals.var_t4_dn10)) * locals.var_t4) + (assign60730_e94767 * locals.var_t4_dn10)), (((((8.0 * locals.var_t10_dn11) * locals.var_t4) + (assign60730_e94765 * locals.var_t4_dn11)) * locals.var_t4) + (assign60730_e94767 * locals.var_t4_dn11)), (((((8.0 * locals.var_t10_dn14) * locals.var_t4) + (assign60730_e94765 * locals.var_t4_dn14)) * locals.var_t4) + (assign60730_e94767 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign60730_e94771;
        locals.var_t1_dn0 = assign60730_e94771_d_n0;
        locals.var_t1_dn2 = assign60730_e94771_d_n2;
        locals.var_t1_dn4 = assign60730_e94771_d_n4;
        locals.var_t1_dn5 = assign60730_e94771_d_n5;
        locals.var_t1_dn6 = assign60730_e94771_d_n6;
        locals.var_t1_dn7 = assign60730_e94771_d_n7;
        locals.var_t1_dn8 = assign60730_e94771_d_n8;
        locals.var_t1_dn9 = assign60730_e94771_d_n9;
        locals.var_t1_dn10 = assign60730_e94771_d_n10;
        locals.var_t1_dn11 = assign60730_e94771_d_n11;
        locals.var_t1_dn14 = assign60730_e94771_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign60740_e94787, assign60740_e94787_d_n0, assign60740_e94787_d_n2, assign60740_e94787_d_n4, assign60740_e94787_d_n5, assign60740_e94787_d_n6, assign60740_e94787_d_n7, assign60740_e94787_d_n8, assign60740_e94787_d_n9, assign60740_e94787_d_n10, assign60740_e94787_d_n11, assign60740_e94787_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) {
        let assign60740_e94783: f64 = (2.0 * locals.var_t11);
        let assign60740_e94785: f64 = (assign60740_e94783 * locals.var_t4);
        (assign60740_e94785, (((2.0 * locals.var_t11_dn0) * locals.var_t4) + (assign60740_e94783 * locals.var_t4_dn0)), (((2.0 * locals.var_t11_dn2) * locals.var_t4) + (assign60740_e94783 * locals.var_t4_dn2)), (((2.0 * locals.var_t11_dn4) * locals.var_t4) + (assign60740_e94783 * locals.var_t4_dn4)), (((2.0 * locals.var_t11_dn5) * locals.var_t4) + (assign60740_e94783 * locals.var_t4_dn5)), (((2.0 * locals.var_t11_dn6) * locals.var_t4) + (assign60740_e94783 * locals.var_t4_dn6)), (((2.0 * locals.var_t11_dn7) * locals.var_t4) + (assign60740_e94783 * locals.var_t4_dn7)), (((2.0 * locals.var_t11_dn8) * locals.var_t4) + (assign60740_e94783 * locals.var_t4_dn8)), (((2.0 * locals.var_t11_dn9) * locals.var_t4) + (assign60740_e94783 * locals.var_t4_dn9)), (((2.0 * locals.var_t11_dn10) * locals.var_t4) + (assign60740_e94783 * locals.var_t4_dn10)), (((2.0 * locals.var_t11_dn11) * locals.var_t4) + (assign60740_e94783 * locals.var_t4_dn11)), (((2.0 * locals.var_t11_dn14) * locals.var_t4) + (assign60740_e94783 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign60740_e94787;
        locals.var_t2_dn0 = assign60740_e94787_d_n0;
        locals.var_t2_dn2 = assign60740_e94787_d_n2;
        locals.var_t2_dn4 = assign60740_e94787_d_n4;
        locals.var_t2_dn5 = assign60740_e94787_d_n5;
        locals.var_t2_dn6 = assign60740_e94787_d_n6;
        locals.var_t2_dn7 = assign60740_e94787_d_n7;
        locals.var_t2_dn8 = assign60740_e94787_d_n8;
        locals.var_t2_dn9 = assign60740_e94787_d_n9;
        locals.var_t2_dn10 = assign60740_e94787_d_n10;
        locals.var_t2_dn11 = assign60740_e94787_d_n11;
        locals.var_t2_dn14 = assign60740_e94787_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign60750_e94803, assign60750_e94803_d_n0, assign60750_e94803_d_n2, assign60750_e94803_d_n4, assign60750_e94803_d_n5, assign60750_e94803_d_n6, assign60750_e94803_d_n7, assign60750_e94803_d_n8, assign60750_e94803_d_n9, assign60750_e94803_d_n10, assign60750_e94803_d_n11, assign60750_e94803_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) {
        let assign60750_e94799: f64 = (locals.var_t11 * locals.var_t4);
        let assign60750_e94801: f64 = (assign60750_e94799 * locals.var_t4);
        (assign60750_e94801, ((((locals.var_t11_dn0 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn0)) * locals.var_t4) + (assign60750_e94799 * locals.var_t4_dn0)), ((((locals.var_t11_dn2 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn2)) * locals.var_t4) + (assign60750_e94799 * locals.var_t4_dn2)), ((((locals.var_t11_dn4 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn4)) * locals.var_t4) + (assign60750_e94799 * locals.var_t4_dn4)), ((((locals.var_t11_dn5 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn5)) * locals.var_t4) + (assign60750_e94799 * locals.var_t4_dn5)), ((((locals.var_t11_dn6 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn6)) * locals.var_t4) + (assign60750_e94799 * locals.var_t4_dn6)), ((((locals.var_t11_dn7 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn7)) * locals.var_t4) + (assign60750_e94799 * locals.var_t4_dn7)), ((((locals.var_t11_dn8 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn8)) * locals.var_t4) + (assign60750_e94799 * locals.var_t4_dn8)), ((((locals.var_t11_dn9 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn9)) * locals.var_t4) + (assign60750_e94799 * locals.var_t4_dn9)), ((((locals.var_t11_dn10 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn10)) * locals.var_t4) + (assign60750_e94799 * locals.var_t4_dn10)), ((((locals.var_t11_dn11 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn11)) * locals.var_t4) + (assign60750_e94799 * locals.var_t4_dn11)), ((((locals.var_t11_dn14 * locals.var_t4) + (locals.var_t11 * locals.var_t4_dn14)) * locals.var_t4) + (assign60750_e94799 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign60750_e94803;
        locals.var_t8_dn0 = assign60750_e94803_d_n0;
        locals.var_t8_dn2 = assign60750_e94803_d_n2;
        locals.var_t8_dn4 = assign60750_e94803_d_n4;
        locals.var_t8_dn5 = assign60750_e94803_d_n5;
        locals.var_t8_dn6 = assign60750_e94803_d_n6;
        locals.var_t8_dn7 = assign60750_e94803_d_n7;
        locals.var_t8_dn8 = assign60750_e94803_d_n8;
        locals.var_t8_dn9 = assign60750_e94803_d_n9;
        locals.var_t8_dn10 = assign60750_e94803_d_n10;
        locals.var_t8_dn11 = assign60750_e94803_d_n11;
        locals.var_t8_dn14 = assign60750_e94803_d_n14;
        locals.var_t8_rv = 0.0;

        let (assign60760_e94820, assign60760_e94820_d_n0, assign60760_e94820_d_n2, assign60760_e94820_d_n4, assign60760_e94820_d_n5, assign60760_e94820_d_n6, assign60760_e94820_d_n7, assign60760_e94820_d_n8, assign60760_e94820_d_n9, assign60760_e94820_d_n10, assign60760_e94820_d_n11, assign60760_e94820_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) {
        let assign60760_e94815: f64 = (locals.var_t7 * locals.var_t7);
        let assign60760_e94817: f64 = (assign60760_e94815 + locals.var_t8);
        let assign60760_e94818: f64 = (assign60760_e94817).sqrt();
        (assign60760_e94818, ((((locals.var_t7_dn0 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn0)) + locals.var_t8_dn0) / (2.0 * assign60760_e94818)), ((((locals.var_t7_dn2 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn2)) + locals.var_t8_dn2) / (2.0 * assign60760_e94818)), ((((locals.var_t7_dn4 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn4)) + locals.var_t8_dn4) / (2.0 * assign60760_e94818)), ((((locals.var_t7_dn5 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn5)) + locals.var_t8_dn5) / (2.0 * assign60760_e94818)), ((((locals.var_t7_dn6 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn6)) + locals.var_t8_dn6) / (2.0 * assign60760_e94818)), ((((locals.var_t7_dn7 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn7)) + locals.var_t8_dn7) / (2.0 * assign60760_e94818)), ((((locals.var_t7_dn8 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn8)) + locals.var_t8_dn8) / (2.0 * assign60760_e94818)), ((((locals.var_t7_dn9 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn9)) + locals.var_t8_dn9) / (2.0 * assign60760_e94818)), ((((locals.var_t7_dn10 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn10)) + locals.var_t8_dn10) / (2.0 * assign60760_e94818)), ((((locals.var_t7_dn11 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn11)) + locals.var_t8_dn11) / (2.0 * assign60760_e94818)), ((((locals.var_t7_dn14 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn14)) + locals.var_t8_dn14) / (2.0 * assign60760_e94818)),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign60760_e94820;
        locals.var_t9_dn0 = assign60760_e94820_d_n0;
        locals.var_t9_dn2 = assign60760_e94820_d_n2;
        locals.var_t9_dn4 = assign60760_e94820_d_n4;
        locals.var_t9_dn5 = assign60760_e94820_d_n5;
        locals.var_t9_dn6 = assign60760_e94820_d_n6;
        locals.var_t9_dn7 = assign60760_e94820_d_n7;
        locals.var_t9_dn8 = assign60760_e94820_d_n8;
        locals.var_t9_dn9 = assign60760_e94820_d_n9;
        locals.var_t9_dn10 = assign60760_e94820_d_n10;
        locals.var_t9_dn11 = assign60760_e94820_d_n11;
        locals.var_t9_dn14 = assign60760_e94820_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign60770_e94837, assign60770_e94837_d_n0, assign60770_e94837_d_n2, assign60770_e94837_d_n4, assign60770_e94837_d_n5, assign60770_e94837_d_n6, assign60770_e94837_d_n7, assign60770_e94837_d_n8, assign60770_e94837_d_n9, assign60770_e94837_d_n10, assign60770_e94837_d_n11, assign60770_e94837_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) {
        let assign60770_e94832: f64 = (-locals.var_t7);
        let assign60770_e94834: f64 = (assign60770_e94832 + locals.var_t9);
        let assign60770_e94835: f64 = (0.5 * assign60770_e94834);
        (assign60770_e94835, (0.5 * ((-locals.var_t7_dn0) + locals.var_t9_dn0)), (0.5 * ((-locals.var_t7_dn2) + locals.var_t9_dn2)), (0.5 * ((-locals.var_t7_dn4) + locals.var_t9_dn4)), (0.5 * ((-locals.var_t7_dn5) + locals.var_t9_dn5)), (0.5 * ((-locals.var_t7_dn6) + locals.var_t9_dn6)), (0.5 * ((-locals.var_t7_dn7) + locals.var_t9_dn7)), (0.5 * ((-locals.var_t7_dn8) + locals.var_t9_dn8)), (0.5 * ((-locals.var_t7_dn9) + locals.var_t9_dn9)), (0.5 * ((-locals.var_t7_dn10) + locals.var_t9_dn10)), (0.5 * ((-locals.var_t7_dn11) + locals.var_t9_dn11)), (0.5 * ((-locals.var_t7_dn14) + locals.var_t9_dn14)),)
    } else {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn4, locals.var_lred_dn5, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn8, locals.var_lred_dn9, locals.var_lred_dn10, locals.var_lred_dn11, locals.var_lred_dn14,)
    }
};
        locals.var_lred = assign60770_e94837;
        locals.var_lred_dn0 = assign60770_e94837_d_n0;
        locals.var_lred_dn2 = assign60770_e94837_d_n2;
        locals.var_lred_dn4 = assign60770_e94837_d_n4;
        locals.var_lred_dn5 = assign60770_e94837_d_n5;
        locals.var_lred_dn6 = assign60770_e94837_d_n6;
        locals.var_lred_dn7 = assign60770_e94837_d_n7;
        locals.var_lred_dn8 = assign60770_e94837_d_n8;
        locals.var_lred_dn9 = assign60770_e94837_d_n9;
        locals.var_lred_dn10 = assign60770_e94837_d_n10;
        locals.var_lred_dn11 = assign60770_e94837_d_n11;
        locals.var_lred_dn14 = assign60770_e94837_d_n14;
        locals.var_lred_rv = 0.0;

        let (assign60780_e94849, assign60780_e94849_d_n0, assign60780_e94849_d_n2, assign60780_e94849_d_n4, assign60780_e94849_d_n5, assign60780_e94849_d_n6, assign60780_e94849_d_n7, assign60780_e94849_d_n8, assign60780_e94849_d_n9, assign60780_e94849_d_n10, assign60780_e94849_d_n11, assign60780_e94849_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn4, locals.var_lred_dn5, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn8, locals.var_lred_dn9, locals.var_lred_dn10, locals.var_lred_dn11, locals.var_lred_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign60780_e94849;
        locals.var_t1_dn0 = assign60780_e94849_d_n0;
        locals.var_t1_dn2 = assign60780_e94849_d_n2;
        locals.var_t1_dn4 = assign60780_e94849_d_n4;
        locals.var_t1_dn5 = assign60780_e94849_d_n5;
        locals.var_t1_dn6 = assign60780_e94849_d_n6;
        locals.var_t1_dn7 = assign60780_e94849_d_n7;
        locals.var_t1_dn8 = assign60780_e94849_d_n8;
        locals.var_t1_dn9 = assign60780_e94849_d_n9;
        locals.var_t1_dn10 = assign60780_e94849_d_n10;
        locals.var_t1_dn11 = assign60780_e94849_d_n11;
        locals.var_t1_dn14 = assign60780_e94849_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign60790_e94863, assign60790_e94863_d_n0, assign60790_e94863_d_n2, assign60790_e94863_d_n4, assign60790_e94863_d_n5, assign60790_e94863_d_n6, assign60790_e94863_d_n7, assign60790_e94863_d_n8, assign60790_e94863_d_n9, assign60790_e94863_d_n10, assign60790_e94863_d_n11, assign60790_e94863_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1466 == 0.0)) {
        let assign60790_e94861: f64 = (locals.var_fmdvds * locals.var_t1);
        (assign60790_e94861, ((locals.var_fmdvds_dn0 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn0)), ((locals.var_fmdvds_dn2 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn2)), ((locals.var_fmdvds_dn4 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn4)), ((locals.var_fmdvds_dn5 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn5)), ((locals.var_fmdvds_dn6 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn6)), ((locals.var_fmdvds_dn7 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn7)), ((locals.var_fmdvds_dn8 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn8)), ((locals.var_fmdvds_dn9 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn9)), ((locals.var_fmdvds_dn10 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn10)), ((locals.var_fmdvds_dn11 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn11)), ((locals.var_fmdvds_dn14 * locals.var_t1) + (locals.var_fmdvds * locals.var_t1_dn14)),)
    } else {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn4, locals.var_lred_dn5, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn8, locals.var_lred_dn9, locals.var_lred_dn10, locals.var_lred_dn11, locals.var_lred_dn14,)
    }
};
        locals.var_lred = assign60790_e94863;
        locals.var_lred_dn0 = assign60790_e94863_d_n0;
        locals.var_lred_dn2 = assign60790_e94863_d_n2;
        locals.var_lred_dn4 = assign60790_e94863_d_n4;
        locals.var_lred_dn5 = assign60790_e94863_d_n5;
        locals.var_lred_dn6 = assign60790_e94863_d_n6;
        locals.var_lred_dn7 = assign60790_e94863_d_n7;
        locals.var_lred_dn8 = assign60790_e94863_d_n8;
        locals.var_lred_dn9 = assign60790_e94863_d_n9;
        locals.var_lred_dn10 = assign60790_e94863_d_n10;
        locals.var_lred_dn11 = assign60790_e94863_d_n11;
        locals.var_lred_dn14 = assign60790_e94863_d_n14;
        locals.var_lred_rv = 0.0;

        let (assign60800_e94874, assign60800_e94874_d_n0, assign60800_e94874_d_n2, assign60800_e94874_d_n4, assign60800_e94874_d_n5, assign60800_e94874_d_n6, assign60800_e94874_d_n7, assign60800_e94874_d_n8, assign60800_e94874_d_n9, assign60800_e94874_d_n10, assign60800_e94874_d_n11, assign60800_e94874_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign60800_e94872: f64 = (locals.var_lred * locals.var_clmmod);
        (assign60800_e94872, (locals.var_lred_dn0 * locals.var_clmmod), (locals.var_lred_dn2 * locals.var_clmmod), (locals.var_lred_dn4 * locals.var_clmmod), (locals.var_lred_dn5 * locals.var_clmmod), (locals.var_lred_dn6 * locals.var_clmmod), (locals.var_lred_dn7 * locals.var_clmmod), (locals.var_lred_dn8 * locals.var_clmmod), (locals.var_lred_dn9 * locals.var_clmmod), (locals.var_lred_dn10 * locals.var_clmmod), (locals.var_lred_dn11 * locals.var_clmmod), (locals.var_lred_dn14 * locals.var_clmmod),)
    } else {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn4, locals.var_lred_dn5, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn8, locals.var_lred_dn9, locals.var_lred_dn10, locals.var_lred_dn11, locals.var_lred_dn14,)
    }
};
        locals.var_lred = assign60800_e94874;
        locals.var_lred_dn0 = assign60800_e94874_d_n0;
        locals.var_lred_dn2 = assign60800_e94874_d_n2;
        locals.var_lred_dn4 = assign60800_e94874_d_n4;
        locals.var_lred_dn5 = assign60800_e94874_d_n5;
        locals.var_lred_dn6 = assign60800_e94874_d_n6;
        locals.var_lred_dn7 = assign60800_e94874_d_n7;
        locals.var_lred_dn8 = assign60800_e94874_d_n8;
        locals.var_lred_dn9 = assign60800_e94874_d_n9;
        locals.var_lred_dn10 = assign60800_e94874_d_n10;
        locals.var_lred_dn11 = assign60800_e94874_d_n11;
        locals.var_lred_dn14 = assign60800_e94874_d_n14;
        locals.var_lred_rv = 0.0;

        let (assign60810_e94885, assign60810_e94885_d_n0, assign60810_e94885_d_n2, assign60810_e94885_d_n4, assign60810_e94885_d_n5, assign60810_e94885_d_n6, assign60810_e94885_d_n7, assign60810_e94885_d_n8, assign60810_e94885_d_n9, assign60810_e94885_d_n10, assign60810_e94885_d_n11, assign60810_e94885_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign60810_e94883: f64 = (locals.var_vgp + locals.var_beta_inv);
        (assign60810_e94883, (locals.var_vgp_dn0 + locals.var_beta_inv_dn0), (locals.var_vgp_dn2 + locals.var_beta_inv_dn2), (locals.var_vgp_dn4 + locals.var_beta_inv_dn4), (locals.var_vgp_dn5 + locals.var_beta_inv_dn5), (locals.var_vgp_dn6 + locals.var_beta_inv_dn6), (locals.var_vgp_dn7 + locals.var_beta_inv_dn7), (locals.var_vgp_dn8 + locals.var_beta_inv_dn8), (locals.var_vgp_dn9 + locals.var_beta_inv_dn9), (locals.var_vgp_dn10 + locals.var_beta_inv_dn10), (locals.var_vgp_dn11 + locals.var_beta_inv_dn11), (locals.var_vgp_dn14 + locals.var_beta_inv_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign60810_e94885;
        locals.var_t1_dn0 = assign60810_e94885_d_n0;
        locals.var_t1_dn2 = assign60810_e94885_d_n2;
        locals.var_t1_dn4 = assign60810_e94885_d_n4;
        locals.var_t1_dn5 = assign60810_e94885_d_n5;
        locals.var_t1_dn6 = assign60810_e94885_d_n6;
        locals.var_t1_dn7 = assign60810_e94885_d_n7;
        locals.var_t1_dn8 = assign60810_e94885_d_n8;
        locals.var_t1_dn9 = assign60810_e94885_d_n9;
        locals.var_t1_dn10 = assign60810_e94885_d_n10;
        locals.var_t1_dn11 = assign60810_e94885_d_n11;
        locals.var_t1_dn14 = assign60810_e94885_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign60820_e94898, assign60820_e94898_d_n0, assign60820_e94898_d_n2, assign60820_e94898_d_n4, assign60820_e94898_d_n5, assign60820_e94898_d_n6, assign60820_e94898_d_n7, assign60820_e94898_d_n8, assign60820_e94898_d_n9, assign60820_e94898_d_n10, assign60820_e94898_d_n11, assign60820_e94898_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign60820_e94894: f64 = (locals.var_t1 * locals.var_f10);
        let assign60820_e94896: f64 = (assign60820_e94894 - locals.var_f11);
        (assign60820_e94896, (((locals.var_t1_dn0 * locals.var_f10) + (locals.var_t1 * locals.var_f10_dn0)) - locals.var_f11_dn0), (((locals.var_t1_dn2 * locals.var_f10) + (locals.var_t1 * locals.var_f10_dn2)) - locals.var_f11_dn2), (((locals.var_t1_dn4 * locals.var_f10) + (locals.var_t1 * locals.var_f10_dn4)) - locals.var_f11_dn4), (((locals.var_t1_dn5 * locals.var_f10) + (locals.var_t1 * locals.var_f10_dn5)) - locals.var_f11_dn5), (((locals.var_t1_dn6 * locals.var_f10) + (locals.var_t1 * locals.var_f10_dn6)) - locals.var_f11_dn6), (((locals.var_t1_dn7 * locals.var_f10) + (locals.var_t1 * locals.var_f10_dn7)) - locals.var_f11_dn7), (((locals.var_t1_dn8 * locals.var_f10) + (locals.var_t1 * locals.var_f10_dn8)) - locals.var_f11_dn8), (((locals.var_t1_dn9 * locals.var_f10) + (locals.var_t1 * locals.var_f10_dn9)) - locals.var_f11_dn9), (((locals.var_t1_dn10 * locals.var_f10) + (locals.var_t1 * locals.var_f10_dn10)) - locals.var_f11_dn10), (((locals.var_t1_dn11 * locals.var_f10) + (locals.var_t1 * locals.var_f10_dn11)) - locals.var_f11_dn11), (((locals.var_t1_dn14 * locals.var_f10) + (locals.var_t1 * locals.var_f10_dn14)) - locals.var_f11_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign60820_e94898;
        locals.var_t2_dn0 = assign60820_e94898_d_n0;
        locals.var_t2_dn2 = assign60820_e94898_d_n2;
        locals.var_t2_dn4 = assign60820_e94898_d_n4;
        locals.var_t2_dn5 = assign60820_e94898_d_n5;
        locals.var_t2_dn6 = assign60820_e94898_d_n6;
        locals.var_t2_dn7 = assign60820_e94898_d_n7;
        locals.var_t2_dn8 = assign60820_e94898_d_n8;
        locals.var_t2_dn9 = assign60820_e94898_d_n9;
        locals.var_t2_dn10 = assign60820_e94898_d_n10;
        locals.var_t2_dn11 = assign60820_e94898_d_n11;
        locals.var_t2_dn14 = assign60820_e94898_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign60830_e94925, assign60830_e94925_d_n0, assign60830_e94925_d_n2, assign60830_e94925_d_n4, assign60830_e94925_d_n5, assign60830_e94925_d_n6, assign60830_e94925_d_n7, assign60830_e94925_d_n8, assign60830_e94925_d_n9, assign60830_e94925_d_n10, assign60830_e94925_d_n11, assign60830_e94925_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign60830_e94910: f64 = (locals.var_xi0 + 1.0);
        let assign60830_e94911: f64 = (1.5 - assign60830_e94910);
        let assign60830_e94914: f64 = (0.5 * locals.var_beta);
        let assign60830_e94916: f64 = (assign60830_e94914 * locals.var_pds);
        let assign60830_e94917: f64 = (assign60830_e94911 - assign60830_e94916);
        let assign60830_e94918: f64 = (locals.var_cnst0 * assign60830_e94917);
        let assign60830_e94921: f64 = (locals.var_cox * locals.var_t2);
        let assign60830_e94922: f64 = (assign60830_e94918 + assign60830_e94921);
        let assign60830_e94923: f64 = (locals.var_cnst0 * assign60830_e94922);
        (assign60830_e94923, ((locals.var_cnst0_dn0 * assign60830_e94922) + (locals.var_cnst0 * (((locals.var_cnst0_dn0 * assign60830_e94917) + (locals.var_cnst0 * ((-locals.var_xi0_dn0) - (((0.5 * locals.var_beta_dn0) * locals.var_pds) + (assign60830_e94914 * locals.var_pds_dn0))))) + ((locals.var_cox_dn0 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn0))))), ((locals.var_cnst0_dn2 * assign60830_e94922) + (locals.var_cnst0 * (((locals.var_cnst0_dn2 * assign60830_e94917) + (locals.var_cnst0 * ((-locals.var_xi0_dn2) - (((0.5 * locals.var_beta_dn2) * locals.var_pds) + (assign60830_e94914 * locals.var_pds_dn2))))) + ((locals.var_cox_dn2 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn2))))), ((locals.var_cnst0_dn4 * assign60830_e94922) + (locals.var_cnst0 * (((locals.var_cnst0_dn4 * assign60830_e94917) + (locals.var_cnst0 * ((-locals.var_xi0_dn4) - (((0.5 * locals.var_beta_dn4) * locals.var_pds) + (assign60830_e94914 * locals.var_pds_dn4))))) + ((locals.var_cox_dn4 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn4))))), ((locals.var_cnst0_dn5 * assign60830_e94922) + (locals.var_cnst0 * (((locals.var_cnst0_dn5 * assign60830_e94917) + (locals.var_cnst0 * ((-locals.var_xi0_dn5) - (((0.5 * locals.var_beta_dn5) * locals.var_pds) + (assign60830_e94914 * locals.var_pds_dn5))))) + ((locals.var_cox_dn5 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn5))))), ((locals.var_cnst0_dn6 * assign60830_e94922) + (locals.var_cnst0 * (((locals.var_cnst0_dn6 * assign60830_e94917) + (locals.var_cnst0 * ((-locals.var_xi0_dn6) - (((0.5 * locals.var_beta_dn6) * locals.var_pds) + (assign60830_e94914 * locals.var_pds_dn6))))) + ((locals.var_cox_dn6 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn6))))), ((locals.var_cnst0_dn7 * assign60830_e94922) + (locals.var_cnst0 * (((locals.var_cnst0_dn7 * assign60830_e94917) + (locals.var_cnst0 * ((-locals.var_xi0_dn7) - (((0.5 * locals.var_beta_dn7) * locals.var_pds) + (assign60830_e94914 * locals.var_pds_dn7))))) + ((locals.var_cox_dn7 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn7))))), ((locals.var_cnst0_dn8 * assign60830_e94922) + (locals.var_cnst0 * (((locals.var_cnst0_dn8 * assign60830_e94917) + (locals.var_cnst0 * ((-locals.var_xi0_dn8) - (((0.5 * locals.var_beta_dn8) * locals.var_pds) + (assign60830_e94914 * locals.var_pds_dn8))))) + ((locals.var_cox_dn8 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn8))))), ((locals.var_cnst0_dn9 * assign60830_e94922) + (locals.var_cnst0 * (((locals.var_cnst0_dn9 * assign60830_e94917) + (locals.var_cnst0 * ((-locals.var_xi0_dn9) - (((0.5 * locals.var_beta_dn9) * locals.var_pds) + (assign60830_e94914 * locals.var_pds_dn9))))) + ((locals.var_cox_dn9 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn9))))), ((locals.var_cnst0_dn10 * assign60830_e94922) + (locals.var_cnst0 * (((locals.var_cnst0_dn10 * assign60830_e94917) + (locals.var_cnst0 * ((-locals.var_xi0_dn10) - (((0.5 * locals.var_beta_dn10) * locals.var_pds) + (assign60830_e94914 * locals.var_pds_dn10))))) + ((locals.var_cox_dn10 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn10))))), ((locals.var_cnst0_dn11 * assign60830_e94922) + (locals.var_cnst0 * (((locals.var_cnst0_dn11 * assign60830_e94917) + (locals.var_cnst0 * ((-locals.var_xi0_dn11) - (((0.5 * locals.var_beta_dn11) * locals.var_pds) + (assign60830_e94914 * locals.var_pds_dn11))))) + ((locals.var_cox_dn11 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn11))))), ((locals.var_cnst0_dn14 * assign60830_e94922) + (locals.var_cnst0 * (((locals.var_cnst0_dn14 * assign60830_e94917) + (locals.var_cnst0 * ((-locals.var_xi0_dn14) - (((0.5 * locals.var_beta_dn14) * locals.var_pds) + (assign60830_e94914 * locals.var_pds_dn14))))) + ((locals.var_cox_dn14 * locals.var_t2) + (locals.var_cox * locals.var_t2_dn14))))),)
    } else {
        (locals.var_qbnm, locals.var_qbnm_dn0, locals.var_qbnm_dn2, locals.var_qbnm_dn4, locals.var_qbnm_dn5, locals.var_qbnm_dn6, locals.var_qbnm_dn7, locals.var_qbnm_dn8, locals.var_qbnm_dn9, locals.var_qbnm_dn10, locals.var_qbnm_dn11, locals.var_qbnm_dn14,)
    }
};
        locals.var_qbnm = assign60830_e94925;
        locals.var_qbnm_dn0 = assign60830_e94925_d_n0;
        locals.var_qbnm_dn2 = assign60830_e94925_d_n2;
        locals.var_qbnm_dn4 = assign60830_e94925_d_n4;
        locals.var_qbnm_dn5 = assign60830_e94925_d_n5;
        locals.var_qbnm_dn6 = assign60830_e94925_d_n6;
        locals.var_qbnm_dn7 = assign60830_e94925_d_n7;
        locals.var_qbnm_dn8 = assign60830_e94925_d_n8;
        locals.var_qbnm_dn9 = assign60830_e94925_d_n9;
        locals.var_qbnm_dn10 = assign60830_e94925_d_n10;
        locals.var_qbnm_dn11 = assign60830_e94925_d_n11;
        locals.var_qbnm_dn14 = assign60830_e94925_d_n14;
        locals.var_qbnm_rv = 0.0;

        let (assign60840_e94934, assign60840_e94934_d_n0, assign60840_e94934_d_n2, assign60840_e94934_d_n4, assign60840_e94934_d_n5, assign60840_e94934_d_n6, assign60840_e94934_d_n7, assign60840_e94934_d_n8, assign60840_e94934_d_n9, assign60840_e94934_d_n10, assign60840_e94934_d_n11, assign60840_e94934_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        (locals.var_beta, locals.var_beta_dn0, locals.var_beta_dn2, locals.var_beta_dn4, locals.var_beta_dn5, locals.var_beta_dn6, locals.var_beta_dn7, locals.var_beta_dn8, locals.var_beta_dn9, locals.var_beta_dn10, locals.var_beta_dn11, locals.var_beta_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign60840_e94934;
        locals.var_t1_dn0 = assign60840_e94934_d_n0;
        locals.var_t1_dn2 = assign60840_e94934_d_n2;
        locals.var_t1_dn4 = assign60840_e94934_d_n4;
        locals.var_t1_dn5 = assign60840_e94934_d_n5;
        locals.var_t1_dn6 = assign60840_e94934_d_n6;
        locals.var_t1_dn7 = assign60840_e94934_d_n7;
        locals.var_t1_dn8 = assign60840_e94934_d_n8;
        locals.var_t1_dn9 = assign60840_e94934_d_n9;
        locals.var_t1_dn10 = assign60840_e94934_d_n10;
        locals.var_t1_dn11 = assign60840_e94934_d_n11;
        locals.var_t1_dn14 = assign60840_e94934_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign60850_e94947, assign60850_e94947_d_n0, assign60850_e94947_d_n2, assign60850_e94947_d_n4, assign60850_e94947_d_n5, assign60850_e94947_d_n6, assign60850_e94947_d_n7, assign60850_e94947_d_n8, assign60850_e94947_d_n9, assign60850_e94947_d_n10, assign60850_e94947_d_n11, assign60850_e94947_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign60850_e94943: f64 = (locals.var_t1 * locals.var_qbnm);
        let assign60850_e94945: f64 = (assign60850_e94943 / locals.var_fdd);
        (assign60850_e94945, (((((locals.var_t1_dn0 * locals.var_qbnm) + (locals.var_t1 * locals.var_qbnm_dn0)) * locals.var_fdd) - (assign60850_e94943 * locals.var_fdd_dn0)) / (locals.var_fdd * locals.var_fdd)), (((((locals.var_t1_dn2 * locals.var_qbnm) + (locals.var_t1 * locals.var_qbnm_dn2)) * locals.var_fdd) - (assign60850_e94943 * locals.var_fdd_dn2)) / (locals.var_fdd * locals.var_fdd)), (((((locals.var_t1_dn4 * locals.var_qbnm) + (locals.var_t1 * locals.var_qbnm_dn4)) * locals.var_fdd) - (assign60850_e94943 * locals.var_fdd_dn4)) / (locals.var_fdd * locals.var_fdd)), (((((locals.var_t1_dn5 * locals.var_qbnm) + (locals.var_t1 * locals.var_qbnm_dn5)) * locals.var_fdd) - (assign60850_e94943 * locals.var_fdd_dn5)) / (locals.var_fdd * locals.var_fdd)), (((((locals.var_t1_dn6 * locals.var_qbnm) + (locals.var_t1 * locals.var_qbnm_dn6)) * locals.var_fdd) - (assign60850_e94943 * locals.var_fdd_dn6)) / (locals.var_fdd * locals.var_fdd)), (((((locals.var_t1_dn7 * locals.var_qbnm) + (locals.var_t1 * locals.var_qbnm_dn7)) * locals.var_fdd) - (assign60850_e94943 * locals.var_fdd_dn7)) / (locals.var_fdd * locals.var_fdd)), (((((locals.var_t1_dn8 * locals.var_qbnm) + (locals.var_t1 * locals.var_qbnm_dn8)) * locals.var_fdd) - (assign60850_e94943 * locals.var_fdd_dn8)) / (locals.var_fdd * locals.var_fdd)), (((((locals.var_t1_dn9 * locals.var_qbnm) + (locals.var_t1 * locals.var_qbnm_dn9)) * locals.var_fdd) - (assign60850_e94943 * locals.var_fdd_dn9)) / (locals.var_fdd * locals.var_fdd)), (((((locals.var_t1_dn10 * locals.var_qbnm) + (locals.var_t1 * locals.var_qbnm_dn10)) * locals.var_fdd) - (assign60850_e94943 * locals.var_fdd_dn10)) / (locals.var_fdd * locals.var_fdd)), (((((locals.var_t1_dn11 * locals.var_qbnm) + (locals.var_t1 * locals.var_qbnm_dn11)) * locals.var_fdd) - (assign60850_e94943 * locals.var_fdd_dn11)) / (locals.var_fdd * locals.var_fdd)), (((((locals.var_t1_dn14 * locals.var_qbnm) + (locals.var_t1 * locals.var_qbnm_dn14)) * locals.var_fdd) - (assign60850_e94943 * locals.var_fdd_dn14)) / (locals.var_fdd * locals.var_fdd)),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn4, locals.var_qbu_dn5, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn8, locals.var_qbu_dn9, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn14,)
    }
};
        locals.var_qbu = assign60850_e94947;
        locals.var_qbu_dn0 = assign60850_e94947_d_n0;
        locals.var_qbu_dn2 = assign60850_e94947_d_n2;
        locals.var_qbu_dn4 = assign60850_e94947_d_n4;
        locals.var_qbu_dn5 = assign60850_e94947_d_n5;
        locals.var_qbu_dn6 = assign60850_e94947_d_n6;
        locals.var_qbu_dn7 = assign60850_e94947_d_n7;
        locals.var_qbu_dn8 = assign60850_e94947_d_n8;
        locals.var_qbu_dn9 = assign60850_e94947_d_n9;
        locals.var_qbu_dn10 = assign60850_e94947_d_n10;
        locals.var_qbu_dn11 = assign60850_e94947_d_n11;
        locals.var_qbu_dn14 = assign60850_e94947_d_n14;
        locals.var_qbu_rv = 0.0;

        let (assign60860_e94958, assign60860_e94958_d_n0, assign60860_e94958_d_n2, assign60860_e94958_d_n4, assign60860_e94958_d_n5, assign60860_e94958_d_n6, assign60860_e94958_d_n7, assign60860_e94958_d_n8, assign60860_e94958_d_n9, assign60860_e94958_d_n10, assign60860_e94958_d_n11, assign60860_e94958_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign60860_e94956: f64 = (2.0 * locals.var_fac1);
        (assign60860_e94956, (2.0 * locals.var_fac1_dn0), (2.0 * locals.var_fac1_dn2), (2.0 * locals.var_fac1_dn4), (2.0 * locals.var_fac1_dn5), (2.0 * locals.var_fac1_dn6), (2.0 * locals.var_fac1_dn7), (2.0 * locals.var_fac1_dn8), (2.0 * locals.var_fac1_dn9), (2.0 * locals.var_fac1_dn10), (2.0 * locals.var_fac1_dn11), (2.0 * locals.var_fac1_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign60860_e94958;
        locals.var_t1_dn0 = assign60860_e94958_d_n0;
        locals.var_t1_dn2 = assign60860_e94958_d_n2;
        locals.var_t1_dn4 = assign60860_e94958_d_n4;
        locals.var_t1_dn5 = assign60860_e94958_d_n5;
        locals.var_t1_dn6 = assign60860_e94958_d_n6;
        locals.var_t1_dn7 = assign60860_e94958_d_n7;
        locals.var_t1_dn8 = assign60860_e94958_d_n8;
        locals.var_t1_dn9 = assign60860_e94958_d_n9;
        locals.var_t1_dn10 = assign60860_e94958_d_n10;
        locals.var_t1_dn11 = assign60860_e94958_d_n11;
        locals.var_t1_dn14 = assign60860_e94958_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign60870_e94971, assign60870_e94971_d_n0, assign60870_e94971_d_n2, assign60870_e94971_d_n4, assign60870_e94971_d_n5, assign60870_e94971_d_n6, assign60870_e94971_d_n7, assign60870_e94971_d_n8, assign60870_e94971_d_n9, assign60870_e94971_d_n10, assign60870_e94971_d_n11, assign60870_e94971_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign60870_e94968: f64 = (locals.var_f10 - locals.var_xi0p12);
        let assign60870_e94969: f64 = (locals.var_t1 * assign60870_e94968);
        (assign60870_e94969, ((locals.var_t1_dn0 * assign60870_e94968) + (locals.var_t1 * (locals.var_f10_dn0 - locals.var_xi0p12_dn0))), ((locals.var_t1_dn2 * assign60870_e94968) + (locals.var_t1 * (locals.var_f10_dn2 - locals.var_xi0p12_dn2))), ((locals.var_t1_dn4 * assign60870_e94968) + (locals.var_t1 * (locals.var_f10_dn4 - locals.var_xi0p12_dn4))), ((locals.var_t1_dn5 * assign60870_e94968) + (locals.var_t1 * (locals.var_f10_dn5 - locals.var_xi0p12_dn5))), ((locals.var_t1_dn6 * assign60870_e94968) + (locals.var_t1 * (locals.var_f10_dn6 - locals.var_xi0p12_dn6))), ((locals.var_t1_dn7 * assign60870_e94968) + (locals.var_t1 * (locals.var_f10_dn7 - locals.var_xi0p12_dn7))), ((locals.var_t1_dn8 * assign60870_e94968) + (locals.var_t1 * (locals.var_f10_dn8 - locals.var_xi0p12_dn8))), ((locals.var_t1_dn9 * assign60870_e94968) + (locals.var_t1 * (locals.var_f10_dn9 - locals.var_xi0p12_dn9))), ((locals.var_t1_dn10 * assign60870_e94968) + (locals.var_t1 * (locals.var_f10_dn10 - locals.var_xi0p12_dn10))), ((locals.var_t1_dn11 * assign60870_e94968) + (locals.var_t1 * (locals.var_f10_dn11 - locals.var_xi0p12_dn11))), ((locals.var_t1_dn14 * assign60870_e94968) + (locals.var_t1 * (locals.var_f10_dn14 - locals.var_xi0p12_dn14))),)
    } else {
        (locals.var_dtpds, locals.var_dtpds_dn0, locals.var_dtpds_dn2, locals.var_dtpds_dn4, locals.var_dtpds_dn5, locals.var_dtpds_dn6, locals.var_dtpds_dn7, locals.var_dtpds_dn8, locals.var_dtpds_dn9, locals.var_dtpds_dn10, locals.var_dtpds_dn11, locals.var_dtpds_dn14,)
    }
};
        locals.var_dtpds = assign60870_e94971;
        locals.var_dtpds_dn0 = assign60870_e94971_d_n0;
        locals.var_dtpds_dn2 = assign60870_e94971_d_n2;
        locals.var_dtpds_dn4 = assign60870_e94971_d_n4;
        locals.var_dtpds_dn5 = assign60870_e94971_d_n5;
        locals.var_dtpds_dn6 = assign60870_e94971_d_n6;
        locals.var_dtpds_dn7 = assign60870_e94971_d_n7;
        locals.var_dtpds_dn8 = assign60870_e94971_d_n8;
        locals.var_dtpds_dn9 = assign60870_e94971_d_n9;
        locals.var_dtpds_dn10 = assign60870_e94971_d_n10;
        locals.var_dtpds_dn11 = assign60870_e94971_d_n11;
        locals.var_dtpds_dn14 = assign60870_e94971_d_n14;
        locals.var_dtpds_rv = 0.0;

        let (assign60880_e94984, assign60880_e94984_d_n0, assign60880_e94984_d_n2, assign60880_e94984_d_n4, assign60880_e94984_d_n5, assign60880_e94984_d_n6, assign60880_e94984_d_n7, assign60880_e94984_d_n8, assign60880_e94984_d_n9, assign60880_e94984_d_n10, assign60880_e94984_d_n11, assign60880_e94984_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign60880_e94981: f64 = (locals.var_f10 - locals.var_xi0p12);
        let assign60880_e94982: f64 = (2.0 * assign60880_e94981);
        (assign60880_e94982, (2.0 * (locals.var_f10_dn0 - locals.var_xi0p12_dn0)), (2.0 * (locals.var_f10_dn2 - locals.var_xi0p12_dn2)), (2.0 * (locals.var_f10_dn4 - locals.var_xi0p12_dn4)), (2.0 * (locals.var_f10_dn5 - locals.var_xi0p12_dn5)), (2.0 * (locals.var_f10_dn6 - locals.var_xi0p12_dn6)), (2.0 * (locals.var_f10_dn7 - locals.var_xi0p12_dn7)), (2.0 * (locals.var_f10_dn8 - locals.var_xi0p12_dn8)), (2.0 * (locals.var_f10_dn9 - locals.var_xi0p12_dn9)), (2.0 * (locals.var_f10_dn10 - locals.var_xi0p12_dn10)), (2.0 * (locals.var_f10_dn11 - locals.var_xi0p12_dn11)), (2.0 * (locals.var_f10_dn14 - locals.var_xi0p12_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign60880_e94984;
        locals.var_t2_dn0 = assign60880_e94984_d_n0;
        locals.var_t2_dn2 = assign60880_e94984_d_n2;
        locals.var_t2_dn4 = assign60880_e94984_d_n4;
        locals.var_t2_dn5 = assign60880_e94984_d_n5;
        locals.var_t2_dn6 = assign60880_e94984_d_n6;
        locals.var_t2_dn7 = assign60880_e94984_d_n7;
        locals.var_t2_dn8 = assign60880_e94984_d_n8;
        locals.var_t2_dn9 = assign60880_e94984_d_n9;
        locals.var_t2_dn10 = assign60880_e94984_d_n10;
        locals.var_t2_dn11 = assign60880_e94984_d_n11;
        locals.var_t2_dn14 = assign60880_e94984_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign60890_e94995, assign60890_e94995_d_n0, assign60890_e94995_d_n2, assign60890_e94995_d_n4, assign60890_e94995_d_n5, assign60890_e94995_d_n6, assign60890_e94995_d_n7, assign60890_e94995_d_n8, assign60890_e94995_d_n9, assign60890_e94995_d_n10, assign60890_e94995_d_n11, assign60890_e94995_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign60890_e94993: f64 = (locals.var_pds + locals.var_dtpds);
        (assign60890_e94993, (locals.var_pds_dn0 + locals.var_dtpds_dn0), (locals.var_pds_dn2 + locals.var_dtpds_dn2), (locals.var_pds_dn4 + locals.var_dtpds_dn4), (locals.var_pds_dn5 + locals.var_dtpds_dn5), (locals.var_pds_dn6 + locals.var_dtpds_dn6), (locals.var_pds_dn7 + locals.var_dtpds_dn7), (locals.var_pds_dn8 + locals.var_dtpds_dn8), (locals.var_pds_dn9 + locals.var_dtpds_dn9), (locals.var_pds_dn10 + locals.var_dtpds_dn10), (locals.var_pds_dn11 + locals.var_dtpds_dn11), (locals.var_pds_dn14 + locals.var_dtpds_dn14),)
    } else {
        (locals.var_achi, locals.var_achi_dn0, locals.var_achi_dn2, locals.var_achi_dn4, locals.var_achi_dn5, locals.var_achi_dn6, locals.var_achi_dn7, locals.var_achi_dn8, locals.var_achi_dn9, locals.var_achi_dn10, locals.var_achi_dn11, locals.var_achi_dn14,)
    }
};
        locals.var_achi = assign60890_e94995;
        locals.var_achi_dn0 = assign60890_e94995_d_n0;
        locals.var_achi_dn2 = assign60890_e94995_d_n2;
        locals.var_achi_dn4 = assign60890_e94995_d_n4;
        locals.var_achi_dn5 = assign60890_e94995_d_n5;
        locals.var_achi_dn6 = assign60890_e94995_d_n6;
        locals.var_achi_dn7 = assign60890_e94995_d_n7;
        locals.var_achi_dn8 = assign60890_e94995_d_n8;
        locals.var_achi_dn9 = assign60890_e94995_d_n9;
        locals.var_achi_dn10 = assign60890_e94995_d_n10;
        locals.var_achi_dn11 = assign60890_e94995_d_n11;
        locals.var_achi_dn14 = assign60890_e94995_d_n14;
        locals.var_achi_rv = 0.0;

        let (assign60900_e95006, assign60900_e95006_d_n0, assign60900_e95006_d_n2, assign60900_e95006_d_n4, assign60900_e95006_d_n5, assign60900_e95006_d_n6, assign60900_e95006_d_n7, assign60900_e95006_d_n8, assign60900_e95006_d_n9, assign60900_e95006_d_n10, assign60900_e95006_d_n11, assign60900_e95006_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign60900_e95004: f64 = (1.0 / locals.var_vgvt);
        (assign60900_e95004, (-(locals.var_vgvt_dn0 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn2 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn4 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn5 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn6 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn7 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn8 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn9 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn10 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn11 / (locals.var_vgvt * locals.var_vgvt))), (-(locals.var_vgvt_dn14 / (locals.var_vgvt * locals.var_vgvt))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign60900_e95006;
        locals.var_t1_dn0 = assign60900_e95006_d_n0;
        locals.var_t1_dn2 = assign60900_e95006_d_n2;
        locals.var_t1_dn4 = assign60900_e95006_d_n4;
        locals.var_t1_dn5 = assign60900_e95006_d_n5;
        locals.var_t1_dn6 = assign60900_e95006_d_n6;
        locals.var_t1_dn7 = assign60900_e95006_d_n7;
        locals.var_t1_dn8 = assign60900_e95006_d_n8;
        locals.var_t1_dn9 = assign60900_e95006_d_n9;
        locals.var_t1_dn10 = assign60900_e95006_d_n10;
        locals.var_t1_dn11 = assign60900_e95006_d_n11;
        locals.var_t1_dn14 = assign60900_e95006_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign60910_e95017, assign60910_e95017_d_n0, assign60910_e95017_d_n2, assign60910_e95017_d_n4, assign60910_e95017_d_n5, assign60910_e95017_d_n6, assign60910_e95017_d_n7, assign60910_e95017_d_n8, assign60910_e95017_d_n9, assign60910_e95017_d_n10, assign60910_e95017_d_n11, assign60910_e95017_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign60910_e95015: f64 = (locals.var_achi * locals.var_t1);
        (assign60910_e95015, ((locals.var_achi_dn0 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn0)), ((locals.var_achi_dn2 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn2)), ((locals.var_achi_dn4 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn4)), ((locals.var_achi_dn5 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn5)), ((locals.var_achi_dn6 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn6)), ((locals.var_achi_dn7 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn7)), ((locals.var_achi_dn8 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn8)), ((locals.var_achi_dn9 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn9)), ((locals.var_achi_dn10 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn10)), ((locals.var_achi_dn11 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn11)), ((locals.var_achi_dn14 * locals.var_t1) + (locals.var_achi * locals.var_t1_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign60910_e95017;
        locals.var_t2_dn0 = assign60910_e95017_d_n0;
        locals.var_t2_dn2 = assign60910_e95017_d_n2;
        locals.var_t2_dn4 = assign60910_e95017_d_n4;
        locals.var_t2_dn5 = assign60910_e95017_d_n5;
        locals.var_t2_dn6 = assign60910_e95017_d_n6;
        locals.var_t2_dn7 = assign60910_e95017_d_n7;
        locals.var_t2_dn8 = assign60910_e95017_d_n8;
        locals.var_t2_dn9 = assign60910_e95017_d_n9;
        locals.var_t2_dn10 = assign60910_e95017_d_n10;
        locals.var_t2_dn11 = assign60910_e95017_d_n11;
        locals.var_t2_dn14 = assign60910_e95017_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign60920_e95028, assign60920_e95028_d_n0, assign60920_e95028_d_n2, assign60920_e95028_d_n4, assign60920_e95028_d_n5, assign60920_e95028_d_n6, assign60920_e95028_d_n7, assign60920_e95028_d_n8, assign60920_e95028_d_n9, assign60920_e95028_d_n10, assign60920_e95028_d_n11, assign60920_e95028_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign60920_e95026: f64 = (1.0 - locals.var_t2);
        (assign60920_e95026, (-locals.var_t2_dn0), (-locals.var_t2_dn2), (-locals.var_t2_dn4), (-locals.var_t2_dn5), (-locals.var_t2_dn6), (-locals.var_t2_dn7), (-locals.var_t2_dn8), (-locals.var_t2_dn9), (-locals.var_t2_dn10), (-locals.var_t2_dn11), (-locals.var_t2_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign60920_e95028;
        locals.var_t3_dn0 = assign60920_e95028_d_n0;
        locals.var_t3_dn2 = assign60920_e95028_d_n2;
        locals.var_t3_dn4 = assign60920_e95028_d_n4;
        locals.var_t3_dn5 = assign60920_e95028_d_n5;
        locals.var_t3_dn6 = assign60920_e95028_d_n6;
        locals.var_t3_dn7 = assign60920_e95028_d_n7;
        locals.var_t3_dn8 = assign60920_e95028_d_n8;
        locals.var_t3_dn9 = assign60920_e95028_d_n9;
        locals.var_t3_dn10 = assign60920_e95028_d_n10;
        locals.var_t3_dn11 = assign60920_e95028_d_n11;
        locals.var_t3_dn14 = assign60920_e95028_d_n14;
        locals.var_t3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_227(
        locals: &mut StampLocals,
    ) {
        let (assign60930_e95039, assign60930_e95039_d_n0, assign60930_e95039_d_n2, assign60930_e95039_d_n4, assign60930_e95039_d_n5, assign60930_e95039_d_n6, assign60930_e95039_d_n7, assign60930_e95039_d_n8, assign60930_e95039_d_n9, assign60930_e95039_d_n10, assign60930_e95039_d_n11, assign60930_e95039_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign60930_e95037: f64 = (1.0 - locals.var_t3);
        (assign60930_e95037, (-locals.var_t3_dn0), (-locals.var_t3_dn2), (-locals.var_t3_dn4), (-locals.var_t3_dn5), (-locals.var_t3_dn6), (-locals.var_t3_dn7), (-locals.var_t3_dn8), (-locals.var_t3_dn9), (-locals.var_t3_dn10), (-locals.var_t3_dn11), (-locals.var_t3_dn14),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign60930_e95039;
        locals.var_tx_dn0 = assign60930_e95039_d_n0;
        locals.var_tx_dn2 = assign60930_e95039_d_n2;
        locals.var_tx_dn4 = assign60930_e95039_d_n4;
        locals.var_tx_dn5 = assign60930_e95039_d_n5;
        locals.var_tx_dn6 = assign60930_e95039_d_n6;
        locals.var_tx_dn7 = assign60930_e95039_d_n7;
        locals.var_tx_dn8 = assign60930_e95039_d_n8;
        locals.var_tx_dn9 = assign60930_e95039_d_n9;
        locals.var_tx_dn10 = assign60930_e95039_d_n10;
        locals.var_tx_dn11 = assign60930_e95039_d_n11;
        locals.var_tx_dn14 = assign60930_e95039_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign60940_e95050, assign60940_e95050_d_n0, assign60940_e95050_d_n2, assign60940_e95050_d_n4, assign60940_e95050_d_n5, assign60940_e95050_d_n6, assign60940_e95050_d_n7, assign60940_e95050_d_n8, assign60940_e95050_d_n9, assign60940_e95050_d_n10, assign60940_e95050_d_n11, assign60940_e95050_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign60940_e95048: f64 = (locals.var_tx * locals.var_tx);
        (assign60940_e95048, ((locals.var_tx_dn0 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn0)), ((locals.var_tx_dn2 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn2)), ((locals.var_tx_dn4 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn4)), ((locals.var_tx_dn5 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn5)), ((locals.var_tx_dn6 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn6)), ((locals.var_tx_dn7 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn7)), ((locals.var_tx_dn8 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn8)), ((locals.var_tx_dn9 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn9)), ((locals.var_tx_dn10 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn10)), ((locals.var_tx_dn11 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn11)), ((locals.var_tx_dn14 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign60940_e95050;
        locals.var_x2_dn0 = assign60940_e95050_d_n0;
        locals.var_x2_dn2 = assign60940_e95050_d_n2;
        locals.var_x2_dn4 = assign60940_e95050_d_n4;
        locals.var_x2_dn5 = assign60940_e95050_d_n5;
        locals.var_x2_dn6 = assign60940_e95050_d_n6;
        locals.var_x2_dn7 = assign60940_e95050_d_n7;
        locals.var_x2_dn8 = assign60940_e95050_d_n8;
        locals.var_x2_dn9 = assign60940_e95050_d_n9;
        locals.var_x2_dn10 = assign60940_e95050_d_n10;
        locals.var_x2_dn11 = assign60940_e95050_d_n11;
        locals.var_x2_dn14 = assign60940_e95050_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign60950_e95061, assign60950_e95061_d_n0, assign60950_e95061_d_n2, assign60950_e95061_d_n4, assign60950_e95061_d_n5, assign60950_e95061_d_n6, assign60950_e95061_d_n7, assign60950_e95061_d_n8, assign60950_e95061_d_n9, assign60950_e95061_d_n10, assign60950_e95061_d_n11, assign60950_e95061_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign60950_e95059: f64 = 1.0;
        (assign60950_e95059, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign60950_e95061;
        locals.var_xmax2_dn0 = assign60950_e95061_d_n0;
        locals.var_xmax2_dn2 = assign60950_e95061_d_n2;
        locals.var_xmax2_dn4 = assign60950_e95061_d_n4;
        locals.var_xmax2_dn5 = assign60950_e95061_d_n5;
        locals.var_xmax2_dn6 = assign60950_e95061_d_n6;
        locals.var_xmax2_dn7 = assign60950_e95061_d_n7;
        locals.var_xmax2_dn8 = assign60950_e95061_d_n8;
        locals.var_xmax2_dn9 = assign60950_e95061_d_n9;
        locals.var_xmax2_dn10 = assign60950_e95061_d_n10;
        locals.var_xmax2_dn11 = assign60950_e95061_d_n11;
        locals.var_xmax2_dn14 = assign60950_e95061_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign60960_e95070, assign60960_e95070_d_n0, assign60960_e95070_d_n2, assign60960_e95070_d_n4, assign60960_e95070_d_n5, assign60960_e95070_d_n6, assign60960_e95070_d_n7, assign60960_e95070_d_n8, assign60960_e95070_d_n9, assign60960_e95070_d_n10, assign60960_e95070_d_n11, assign60960_e95070_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign60960_e95070;
        locals.var_xp_dn0 = assign60960_e95070_d_n0;
        locals.var_xp_dn2 = assign60960_e95070_d_n2;
        locals.var_xp_dn4 = assign60960_e95070_d_n4;
        locals.var_xp_dn5 = assign60960_e95070_d_n5;
        locals.var_xp_dn6 = assign60960_e95070_d_n6;
        locals.var_xp_dn7 = assign60960_e95070_d_n7;
        locals.var_xp_dn8 = assign60960_e95070_d_n8;
        locals.var_xp_dn9 = assign60960_e95070_d_n9;
        locals.var_xp_dn10 = assign60960_e95070_d_n10;
        locals.var_xp_dn11 = assign60960_e95070_d_n11;
        locals.var_xp_dn14 = assign60960_e95070_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign60970_e95079, assign60970_e95079_d_n0, assign60970_e95079_d_n2, assign60970_e95079_d_n4, assign60970_e95079_d_n5, assign60970_e95079_d_n6, assign60970_e95079_d_n7, assign60970_e95079_d_n8, assign60970_e95079_d_n9, assign60970_e95079_d_n10, assign60970_e95079_d_n11, assign60970_e95079_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign60970_e95079;
        locals.var_xmp_dn0 = assign60970_e95079_d_n0;
        locals.var_xmp_dn2 = assign60970_e95079_d_n2;
        locals.var_xmp_dn4 = assign60970_e95079_d_n4;
        locals.var_xmp_dn5 = assign60970_e95079_d_n5;
        locals.var_xmp_dn6 = assign60970_e95079_d_n6;
        locals.var_xmp_dn7 = assign60970_e95079_d_n7;
        locals.var_xmp_dn8 = assign60970_e95079_d_n8;
        locals.var_xmp_dn9 = assign60970_e95079_d_n9;
        locals.var_xmp_dn10 = assign60970_e95079_d_n10;
        locals.var_xmp_dn11 = assign60970_e95079_d_n11;
        locals.var_xmp_dn14 = assign60970_e95079_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign60980_e95088,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign60980_e95088;
        locals.var_m0_rv = 0.0;

        let (assign60990_e95097,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign60990_e95097;
        locals.var_mm_rv = 0.0;

        let (assign61000_e95106, assign61000_e95106_d_n0, assign61000_e95106_d_n2, assign61000_e95106_d_n4, assign61000_e95106_d_n5, assign61000_e95106_d_n6, assign61000_e95106_d_n7, assign61000_e95106_d_n8, assign61000_e95106_d_n9, assign61000_e95106_d_n10, assign61000_e95106_d_n11, assign61000_e95106_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign61000_e95106;
        locals.var_arg_dn0 = assign61000_e95106_d_n0;
        locals.var_arg_dn2 = assign61000_e95106_d_n2;
        locals.var_arg_dn4 = assign61000_e95106_d_n4;
        locals.var_arg_dn5 = assign61000_e95106_d_n5;
        locals.var_arg_dn6 = assign61000_e95106_d_n6;
        locals.var_arg_dn7 = assign61000_e95106_d_n7;
        locals.var_arg_dn8 = assign61000_e95106_d_n8;
        locals.var_arg_dn9 = assign61000_e95106_d_n9;
        locals.var_arg_dn10 = assign61000_e95106_d_n10;
        locals.var_arg_dn11 = assign61000_e95106_d_n11;
        locals.var_arg_dn14 = assign61000_e95106_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign61010_e95115, assign61010_e95115_d_n0, assign61010_e95115_d_n2, assign61010_e95115_d_n4, assign61010_e95115_d_n5, assign61010_e95115_d_n6, assign61010_e95115_d_n7, assign61010_e95115_d_n8, assign61010_e95115_d_n9, assign61010_e95115_d_n10, assign61010_e95115_d_n11, assign61010_e95115_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign61010_e95115;
        locals.var_dnm_dn0 = assign61010_e95115_d_n0;
        locals.var_dnm_dn2 = assign61010_e95115_d_n2;
        locals.var_dnm_dn4 = assign61010_e95115_d_n4;
        locals.var_dnm_dn5 = assign61010_e95115_d_n5;
        locals.var_dnm_dn6 = assign61010_e95115_d_n6;
        locals.var_dnm_dn7 = assign61010_e95115_d_n7;
        locals.var_dnm_dn8 = assign61010_e95115_d_n8;
        locals.var_dnm_dn9 = assign61010_e95115_d_n9;
        locals.var_dnm_dn10 = assign61010_e95115_d_n10;
        locals.var_dnm_dn11 = assign61010_e95115_d_n11;
        locals.var_dnm_dn14 = assign61010_e95115_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign61020_e95126, assign61020_e95126_d_n0, assign61020_e95126_d_n2, assign61020_e95126_d_n4, assign61020_e95126_d_n5, assign61020_e95126_d_n6, assign61020_e95126_d_n7, assign61020_e95126_d_n8, assign61020_e95126_d_n9, assign61020_e95126_d_n10, assign61020_e95126_d_n11, assign61020_e95126_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign61020_e95124: f64 = (locals.var_xp * locals.var_x2);
        (assign61020_e95124, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign61020_e95126;
        locals.var_xp_dn0 = assign61020_e95126_d_n0;
        locals.var_xp_dn2 = assign61020_e95126_d_n2;
        locals.var_xp_dn4 = assign61020_e95126_d_n4;
        locals.var_xp_dn5 = assign61020_e95126_d_n5;
        locals.var_xp_dn6 = assign61020_e95126_d_n6;
        locals.var_xp_dn7 = assign61020_e95126_d_n7;
        locals.var_xp_dn8 = assign61020_e95126_d_n8;
        locals.var_xp_dn9 = assign61020_e95126_d_n9;
        locals.var_xp_dn10 = assign61020_e95126_d_n10;
        locals.var_xp_dn11 = assign61020_e95126_d_n11;
        locals.var_xp_dn14 = assign61020_e95126_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign61030_e95137, assign61030_e95137_d_n0, assign61030_e95137_d_n2, assign61030_e95137_d_n4, assign61030_e95137_d_n5, assign61030_e95137_d_n6, assign61030_e95137_d_n7, assign61030_e95137_d_n8, assign61030_e95137_d_n9, assign61030_e95137_d_n10, assign61030_e95137_d_n11, assign61030_e95137_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign61030_e95135: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign61030_e95135, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign61030_e95137;
        locals.var_xmp_dn0 = assign61030_e95137_d_n0;
        locals.var_xmp_dn2 = assign61030_e95137_d_n2;
        locals.var_xmp_dn4 = assign61030_e95137_d_n4;
        locals.var_xmp_dn5 = assign61030_e95137_d_n5;
        locals.var_xmp_dn6 = assign61030_e95137_d_n6;
        locals.var_xmp_dn7 = assign61030_e95137_d_n7;
        locals.var_xmp_dn8 = assign61030_e95137_d_n8;
        locals.var_xmp_dn9 = assign61030_e95137_d_n9;
        locals.var_xmp_dn10 = assign61030_e95137_d_n10;
        locals.var_xmp_dn11 = assign61030_e95137_d_n11;
        locals.var_xmp_dn14 = assign61030_e95137_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign61040_e95148, assign61040_e95148_d_n0, assign61040_e95148_d_n2, assign61040_e95148_d_n4, assign61040_e95148_d_n5, assign61040_e95148_d_n6, assign61040_e95148_d_n7, assign61040_e95148_d_n8, assign61040_e95148_d_n9, assign61040_e95148_d_n10, assign61040_e95148_d_n11, assign61040_e95148_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign61040_e95146: f64 = (locals.var_xp * locals.var_x2);
        (assign61040_e95146, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign61040_e95148;
        locals.var_xp_dn0 = assign61040_e95148_d_n0;
        locals.var_xp_dn2 = assign61040_e95148_d_n2;
        locals.var_xp_dn4 = assign61040_e95148_d_n4;
        locals.var_xp_dn5 = assign61040_e95148_d_n5;
        locals.var_xp_dn6 = assign61040_e95148_d_n6;
        locals.var_xp_dn7 = assign61040_e95148_d_n7;
        locals.var_xp_dn8 = assign61040_e95148_d_n8;
        locals.var_xp_dn9 = assign61040_e95148_d_n9;
        locals.var_xp_dn10 = assign61040_e95148_d_n10;
        locals.var_xp_dn11 = assign61040_e95148_d_n11;
        locals.var_xp_dn14 = assign61040_e95148_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign61050_e95159, assign61050_e95159_d_n0, assign61050_e95159_d_n2, assign61050_e95159_d_n4, assign61050_e95159_d_n5, assign61050_e95159_d_n6, assign61050_e95159_d_n7, assign61050_e95159_d_n8, assign61050_e95159_d_n9, assign61050_e95159_d_n10, assign61050_e95159_d_n11, assign61050_e95159_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign61050_e95157: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign61050_e95157, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign61050_e95159;
        locals.var_xmp_dn0 = assign61050_e95159_d_n0;
        locals.var_xmp_dn2 = assign61050_e95159_d_n2;
        locals.var_xmp_dn4 = assign61050_e95159_d_n4;
        locals.var_xmp_dn5 = assign61050_e95159_d_n5;
        locals.var_xmp_dn6 = assign61050_e95159_d_n6;
        locals.var_xmp_dn7 = assign61050_e95159_d_n7;
        locals.var_xmp_dn8 = assign61050_e95159_d_n8;
        locals.var_xmp_dn9 = assign61050_e95159_d_n9;
        locals.var_xmp_dn10 = assign61050_e95159_d_n10;
        locals.var_xmp_dn11 = assign61050_e95159_d_n11;
        locals.var_xmp_dn14 = assign61050_e95159_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign61060_e95170, assign61060_e95170_d_n0, assign61060_e95170_d_n2, assign61060_e95170_d_n4, assign61060_e95170_d_n5, assign61060_e95170_d_n6, assign61060_e95170_d_n7, assign61060_e95170_d_n8, assign61060_e95170_d_n9, assign61060_e95170_d_n10, assign61060_e95170_d_n11, assign61060_e95170_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign61060_e95168: f64 = (locals.var_xp * locals.var_x2);
        (assign61060_e95168, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign61060_e95170;
        locals.var_xp_dn0 = assign61060_e95170_d_n0;
        locals.var_xp_dn2 = assign61060_e95170_d_n2;
        locals.var_xp_dn4 = assign61060_e95170_d_n4;
        locals.var_xp_dn5 = assign61060_e95170_d_n5;
        locals.var_xp_dn6 = assign61060_e95170_d_n6;
        locals.var_xp_dn7 = assign61060_e95170_d_n7;
        locals.var_xp_dn8 = assign61060_e95170_d_n8;
        locals.var_xp_dn9 = assign61060_e95170_d_n9;
        locals.var_xp_dn10 = assign61060_e95170_d_n10;
        locals.var_xp_dn11 = assign61060_e95170_d_n11;
        locals.var_xp_dn14 = assign61060_e95170_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign61070_e95181, assign61070_e95181_d_n0, assign61070_e95181_d_n2, assign61070_e95181_d_n4, assign61070_e95181_d_n5, assign61070_e95181_d_n6, assign61070_e95181_d_n7, assign61070_e95181_d_n8, assign61070_e95181_d_n9, assign61070_e95181_d_n10, assign61070_e95181_d_n11, assign61070_e95181_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign61070_e95179: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign61070_e95179, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign61070_e95181;
        locals.var_xmp_dn0 = assign61070_e95181_d_n0;
        locals.var_xmp_dn2 = assign61070_e95181_d_n2;
        locals.var_xmp_dn4 = assign61070_e95181_d_n4;
        locals.var_xmp_dn5 = assign61070_e95181_d_n5;
        locals.var_xmp_dn6 = assign61070_e95181_d_n6;
        locals.var_xmp_dn7 = assign61070_e95181_d_n7;
        locals.var_xmp_dn8 = assign61070_e95181_d_n8;
        locals.var_xmp_dn9 = assign61070_e95181_d_n9;
        locals.var_xmp_dn10 = assign61070_e95181_d_n10;
        locals.var_xmp_dn11 = assign61070_e95181_d_n11;
        locals.var_xmp_dn14 = assign61070_e95181_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign61080_e95192, assign61080_e95192_d_n0, assign61080_e95192_d_n2, assign61080_e95192_d_n4, assign61080_e95192_d_n5, assign61080_e95192_d_n6, assign61080_e95192_d_n7, assign61080_e95192_d_n8, assign61080_e95192_d_n9, assign61080_e95192_d_n10, assign61080_e95192_d_n11, assign61080_e95192_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign61080_e95190: f64 = (locals.var_xp * locals.var_x2);
        (assign61080_e95190, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign61080_e95192;
        locals.var_xp_dn0 = assign61080_e95192_d_n0;
        locals.var_xp_dn2 = assign61080_e95192_d_n2;
        locals.var_xp_dn4 = assign61080_e95192_d_n4;
        locals.var_xp_dn5 = assign61080_e95192_d_n5;
        locals.var_xp_dn6 = assign61080_e95192_d_n6;
        locals.var_xp_dn7 = assign61080_e95192_d_n7;
        locals.var_xp_dn8 = assign61080_e95192_d_n8;
        locals.var_xp_dn9 = assign61080_e95192_d_n9;
        locals.var_xp_dn10 = assign61080_e95192_d_n10;
        locals.var_xp_dn11 = assign61080_e95192_d_n11;
        locals.var_xp_dn14 = assign61080_e95192_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign61090_e95203, assign61090_e95203_d_n0, assign61090_e95203_d_n2, assign61090_e95203_d_n4, assign61090_e95203_d_n5, assign61090_e95203_d_n6, assign61090_e95203_d_n7, assign61090_e95203_d_n8, assign61090_e95203_d_n9, assign61090_e95203_d_n10, assign61090_e95203_d_n11, assign61090_e95203_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign61090_e95201: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign61090_e95201, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign61090_e95203;
        locals.var_xmp_dn0 = assign61090_e95203_d_n0;
        locals.var_xmp_dn2 = assign61090_e95203_d_n2;
        locals.var_xmp_dn4 = assign61090_e95203_d_n4;
        locals.var_xmp_dn5 = assign61090_e95203_d_n5;
        locals.var_xmp_dn6 = assign61090_e95203_d_n6;
        locals.var_xmp_dn7 = assign61090_e95203_d_n7;
        locals.var_xmp_dn8 = assign61090_e95203_d_n8;
        locals.var_xmp_dn9 = assign61090_e95203_d_n9;
        locals.var_xmp_dn10 = assign61090_e95203_d_n10;
        locals.var_xmp_dn11 = assign61090_e95203_d_n11;
        locals.var_xmp_dn14 = assign61090_e95203_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign61100_e95214, assign61100_e95214_d_n0, assign61100_e95214_d_n2, assign61100_e95214_d_n4, assign61100_e95214_d_n5, assign61100_e95214_d_n6, assign61100_e95214_d_n7, assign61100_e95214_d_n8, assign61100_e95214_d_n9, assign61100_e95214_d_n10, assign61100_e95214_d_n11, assign61100_e95214_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign61100_e95212: f64 = (locals.var_xp + locals.var_xmp);
        (assign61100_e95212, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign61100_e95214;
        locals.var_arg_dn0 = assign61100_e95214_d_n0;
        locals.var_arg_dn2 = assign61100_e95214_d_n2;
        locals.var_arg_dn4 = assign61100_e95214_d_n4;
        locals.var_arg_dn5 = assign61100_e95214_d_n5;
        locals.var_arg_dn6 = assign61100_e95214_d_n6;
        locals.var_arg_dn7 = assign61100_e95214_d_n7;
        locals.var_arg_dn8 = assign61100_e95214_d_n8;
        locals.var_arg_dn9 = assign61100_e95214_d_n9;
        locals.var_arg_dn10 = assign61100_e95214_d_n10;
        locals.var_arg_dn11 = assign61100_e95214_d_n11;
        locals.var_arg_dn14 = assign61100_e95214_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign61110_e95223, assign61110_e95223_d_n0, assign61110_e95223_d_n2, assign61110_e95223_d_n4, assign61110_e95223_d_n5, assign61110_e95223_d_n6, assign61110_e95223_d_n7, assign61110_e95223_d_n8, assign61110_e95223_d_n9, assign61110_e95223_d_n10, assign61110_e95223_d_n11, assign61110_e95223_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign61110_e95223;
        locals.var_dnm_dn0 = assign61110_e95223_d_n0;
        locals.var_dnm_dn2 = assign61110_e95223_d_n2;
        locals.var_dnm_dn4 = assign61110_e95223_d_n4;
        locals.var_dnm_dn5 = assign61110_e95223_d_n5;
        locals.var_dnm_dn6 = assign61110_e95223_d_n6;
        locals.var_dnm_dn7 = assign61110_e95223_d_n7;
        locals.var_dnm_dn8 = assign61110_e95223_d_n8;
        locals.var_dnm_dn9 = assign61110_e95223_d_n9;
        locals.var_dnm_dn10 = assign61110_e95223_d_n10;
        locals.var_dnm_dn11 = assign61110_e95223_d_n11;
        locals.var_dnm_dn14 = assign61110_e95223_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign61120_e95238: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1479 = assign61120_e95238;
        locals.var_guard1479_rv = 0.0;

        let assign61130_e95241: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1480 = assign61130_e95241;
        locals.var_guard1480_rv = 0.0;

        let (assign61140_e95254,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1479 != 0.0)) && (locals.var_guard1480 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign61140_e95254;
        locals.var_mm_rv = 0.0;

        let assign61150_e95257: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1481 = assign61150_e95257;
        locals.var_guard1481_rv = 0.0;

        let (assign61160_e95273,) = {
    if ((((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1479 != 0.0)) && (locals.var_guard1480 == 0.0)) && (locals.var_guard1481 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign61160_e95273;
        locals.var_mm_rv = 0.0;

        let assign61170_e95276: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1482 = assign61170_e95276;
        locals.var_guard1482_rv = 0.0;

        let (assign61180_e95295,) = {
    if (((((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1479 != 0.0)) && (locals.var_guard1480 == 0.0)) && (locals.var_guard1481 == 0.0)) && (locals.var_guard1482 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign61180_e95295;
        locals.var_mm_rv = 0.0;

        let assign61190_e95298: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1483 = assign61190_e95298;
        locals.var_guard1483_rv = 0.0;

        let (assign61200_e95320,) = {
    if ((((((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1479 != 0.0)) && (locals.var_guard1480 == 0.0)) && (locals.var_guard1481 == 0.0)) && (locals.var_guard1482 == 0.0)) && (locals.var_guard1483 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign61200_e95320;
        locals.var_mm_rv = 0.0;

        let (assign61210_e95331,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1479 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign61210_e95331;
        locals.var_m0_rv = 0.0;

        let mut assign61220_loop_guard: usize = 0;
        while {
            let assign61220_cond_e95343: f64 = if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1479 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign61220_cond_e95343 != 0.0
        } {
            assign61220_loop_guard += 1;
            assert!(assign61220_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign61220_body0_e95355, assign61220_body0_e95355_d_n0, assign61220_body0_e95355_d_n2, assign61220_body0_e95355_d_n4, assign61220_body0_e95355_d_n5, assign61220_body0_e95355_d_n6, assign61220_body0_e95355_d_n7, assign61220_body0_e95355_d_n8, assign61220_body0_e95355_d_n9, assign61220_body0_e95355_d_n10, assign61220_body0_e95355_d_n11, assign61220_body0_e95355_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1479 != 0.0)) {
        let assign61220_body0_e95353: f64 = (locals.var_dnm).sqrt();
        (assign61220_body0_e95353, (locals.var_dnm_dn0 / (2.0 * assign61220_body0_e95353)), (locals.var_dnm_dn2 / (2.0 * assign61220_body0_e95353)), (locals.var_dnm_dn4 / (2.0 * assign61220_body0_e95353)), (locals.var_dnm_dn5 / (2.0 * assign61220_body0_e95353)), (locals.var_dnm_dn6 / (2.0 * assign61220_body0_e95353)), (locals.var_dnm_dn7 / (2.0 * assign61220_body0_e95353)), (locals.var_dnm_dn8 / (2.0 * assign61220_body0_e95353)), (locals.var_dnm_dn9 / (2.0 * assign61220_body0_e95353)), (locals.var_dnm_dn10 / (2.0 * assign61220_body0_e95353)), (locals.var_dnm_dn11 / (2.0 * assign61220_body0_e95353)), (locals.var_dnm_dn14 / (2.0 * assign61220_body0_e95353)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign61220_body0_e95355;
            locals.var_dnm_dn0 = assign61220_body0_e95355_d_n0;
            locals.var_dnm_dn2 = assign61220_body0_e95355_d_n2;
            locals.var_dnm_dn4 = assign61220_body0_e95355_d_n4;
            locals.var_dnm_dn5 = assign61220_body0_e95355_d_n5;
            locals.var_dnm_dn6 = assign61220_body0_e95355_d_n6;
            locals.var_dnm_dn7 = assign61220_body0_e95355_d_n7;
            locals.var_dnm_dn8 = assign61220_body0_e95355_d_n8;
            locals.var_dnm_dn9 = assign61220_body0_e95355_d_n9;
            locals.var_dnm_dn10 = assign61220_body0_e95355_d_n10;
            locals.var_dnm_dn11 = assign61220_body0_e95355_d_n11;
            locals.var_dnm_dn14 = assign61220_body0_e95355_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign61220_body1_e95368,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1479 != 0.0)) {
        let assign61220_body1_e95366: f64 = (locals.var_m0 + 1.0);
        (assign61220_body1_e95366,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign61220_body1_e95368;
            locals.var_m0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_228(
        locals: &mut StampLocals,
    ) {
        let (assign61230_e95391, assign61230_e95391_d_n0, assign61230_e95391_d_n2, assign61230_e95391_d_n4, assign61230_e95391_d_n5, assign61230_e95391_d_n6, assign61230_e95391_d_n7, assign61230_e95391_d_n8, assign61230_e95391_d_n9, assign61230_e95391_d_n10, assign61230_e95391_d_n11, assign61230_e95391_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1479 == 0.0)) {
        let (assign61230_e95389, assign61230_e95389_d_n0, assign61230_e95389_d_n2, assign61230_e95389_d_n4, assign61230_e95389_d_n5, assign61230_e95389_d_n6, assign61230_e95389_d_n7, assign61230_e95389_d_n8, assign61230_e95389_d_n9, assign61230_e95389_d_n10, assign61230_e95389_d_n11, assign61230_e95389_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign61230_e95386: f64 = (2.0 * 4.0);
                let assign61230_e95387: f64 = (1.0 / assign61230_e95386);
                let assign61230_e95388: f64 = (locals.var_dnm).powf(assign61230_e95387);
                (assign61230_e95388, if 0.0 == 0.0 && ((assign61230_e95387) as f64).is_finite() && ((assign61230_e95387) as f64).fract() == 0.0 { if assign61230_e95387 == 0.0 { 0.0 } else { (assign61230_e95387 * ((locals.var_dnm).powf(assign61230_e95387 - 1.0) * locals.var_dnm_dn0)) } } else { (assign61230_e95388 * (assign61230_e95387 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign61230_e95387) as f64).is_finite() && ((assign61230_e95387) as f64).fract() == 0.0 { if assign61230_e95387 == 0.0 { 0.0 } else { (assign61230_e95387 * ((locals.var_dnm).powf(assign61230_e95387 - 1.0) * locals.var_dnm_dn2)) } } else { (assign61230_e95388 * (assign61230_e95387 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign61230_e95387) as f64).is_finite() && ((assign61230_e95387) as f64).fract() == 0.0 { if assign61230_e95387 == 0.0 { 0.0 } else { (assign61230_e95387 * ((locals.var_dnm).powf(assign61230_e95387 - 1.0) * locals.var_dnm_dn4)) } } else { (assign61230_e95388 * (assign61230_e95387 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign61230_e95387) as f64).is_finite() && ((assign61230_e95387) as f64).fract() == 0.0 { if assign61230_e95387 == 0.0 { 0.0 } else { (assign61230_e95387 * ((locals.var_dnm).powf(assign61230_e95387 - 1.0) * locals.var_dnm_dn5)) } } else { (assign61230_e95388 * (assign61230_e95387 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign61230_e95387) as f64).is_finite() && ((assign61230_e95387) as f64).fract() == 0.0 { if assign61230_e95387 == 0.0 { 0.0 } else { (assign61230_e95387 * ((locals.var_dnm).powf(assign61230_e95387 - 1.0) * locals.var_dnm_dn6)) } } else { (assign61230_e95388 * (assign61230_e95387 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign61230_e95387) as f64).is_finite() && ((assign61230_e95387) as f64).fract() == 0.0 { if assign61230_e95387 == 0.0 { 0.0 } else { (assign61230_e95387 * ((locals.var_dnm).powf(assign61230_e95387 - 1.0) * locals.var_dnm_dn7)) } } else { (assign61230_e95388 * (assign61230_e95387 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign61230_e95387) as f64).is_finite() && ((assign61230_e95387) as f64).fract() == 0.0 { if assign61230_e95387 == 0.0 { 0.0 } else { (assign61230_e95387 * ((locals.var_dnm).powf(assign61230_e95387 - 1.0) * locals.var_dnm_dn8)) } } else { (assign61230_e95388 * (assign61230_e95387 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign61230_e95387) as f64).is_finite() && ((assign61230_e95387) as f64).fract() == 0.0 { if assign61230_e95387 == 0.0 { 0.0 } else { (assign61230_e95387 * ((locals.var_dnm).powf(assign61230_e95387 - 1.0) * locals.var_dnm_dn9)) } } else { (assign61230_e95388 * (assign61230_e95387 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign61230_e95387) as f64).is_finite() && ((assign61230_e95387) as f64).fract() == 0.0 { if assign61230_e95387 == 0.0 { 0.0 } else { (assign61230_e95387 * ((locals.var_dnm).powf(assign61230_e95387 - 1.0) * locals.var_dnm_dn10)) } } else { (assign61230_e95388 * (assign61230_e95387 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign61230_e95387) as f64).is_finite() && ((assign61230_e95387) as f64).fract() == 0.0 { if assign61230_e95387 == 0.0 { 0.0 } else { (assign61230_e95387 * ((locals.var_dnm).powf(assign61230_e95387 - 1.0) * locals.var_dnm_dn11)) } } else { (assign61230_e95388 * (assign61230_e95387 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign61230_e95387) as f64).is_finite() && ((assign61230_e95387) as f64).fract() == 0.0 { if assign61230_e95387 == 0.0 { 0.0 } else { (assign61230_e95387 * ((locals.var_dnm).powf(assign61230_e95387 - 1.0) * locals.var_dnm_dn14)) } } else { (assign61230_e95388 * (assign61230_e95387 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign61230_e95389, assign61230_e95389_d_n0, assign61230_e95389_d_n2, assign61230_e95389_d_n4, assign61230_e95389_d_n5, assign61230_e95389_d_n6, assign61230_e95389_d_n7, assign61230_e95389_d_n8, assign61230_e95389_d_n9, assign61230_e95389_d_n10, assign61230_e95389_d_n11, assign61230_e95389_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign61230_e95391;
        locals.var_dnm_dn0 = assign61230_e95391_d_n0;
        locals.var_dnm_dn2 = assign61230_e95391_d_n2;
        locals.var_dnm_dn4 = assign61230_e95391_d_n4;
        locals.var_dnm_dn5 = assign61230_e95391_d_n5;
        locals.var_dnm_dn6 = assign61230_e95391_d_n6;
        locals.var_dnm_dn7 = assign61230_e95391_d_n7;
        locals.var_dnm_dn8 = assign61230_e95391_d_n8;
        locals.var_dnm_dn9 = assign61230_e95391_d_n9;
        locals.var_dnm_dn10 = assign61230_e95391_d_n10;
        locals.var_dnm_dn11 = assign61230_e95391_d_n11;
        locals.var_dnm_dn14 = assign61230_e95391_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign61240_e95402, assign61240_e95402_d_n0, assign61240_e95402_d_n2, assign61240_e95402_d_n4, assign61240_e95402_d_n5, assign61240_e95402_d_n6, assign61240_e95402_d_n7, assign61240_e95402_d_n8, assign61240_e95402_d_n9, assign61240_e95402_d_n10, assign61240_e95402_d_n11, assign61240_e95402_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign61240_e95400: f64 = (1.0 / locals.var_dnm);
        (assign61240_e95400, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign61240_e95402;
        locals.var_dnm_dn0 = assign61240_e95402_d_n0;
        locals.var_dnm_dn2 = assign61240_e95402_d_n2;
        locals.var_dnm_dn4 = assign61240_e95402_d_n4;
        locals.var_dnm_dn5 = assign61240_e95402_d_n5;
        locals.var_dnm_dn6 = assign61240_e95402_d_n6;
        locals.var_dnm_dn7 = assign61240_e95402_d_n7;
        locals.var_dnm_dn8 = assign61240_e95402_d_n8;
        locals.var_dnm_dn9 = assign61240_e95402_d_n9;
        locals.var_dnm_dn10 = assign61240_e95402_d_n10;
        locals.var_dnm_dn11 = assign61240_e95402_d_n11;
        locals.var_dnm_dn14 = assign61240_e95402_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign61250_e95415, assign61250_e95415_d_n0, assign61250_e95415_d_n2, assign61250_e95415_d_n4, assign61250_e95415_d_n5, assign61250_e95415_d_n6, assign61250_e95415_d_n7, assign61250_e95415_d_n8, assign61250_e95415_d_n9, assign61250_e95415_d_n10, assign61250_e95415_d_n11, assign61250_e95415_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign61250_e95411: f64 = locals.var_tx;
        let assign61250_e95413: f64 = (assign61250_e95411 * locals.var_dnm);
        (assign61250_e95413, ((locals.var_tx_dn0 * locals.var_dnm) + (assign61250_e95411 * locals.var_dnm_dn0)), ((locals.var_tx_dn2 * locals.var_dnm) + (assign61250_e95411 * locals.var_dnm_dn2)), ((locals.var_tx_dn4 * locals.var_dnm) + (assign61250_e95411 * locals.var_dnm_dn4)), ((locals.var_tx_dn5 * locals.var_dnm) + (assign61250_e95411 * locals.var_dnm_dn5)), ((locals.var_tx_dn6 * locals.var_dnm) + (assign61250_e95411 * locals.var_dnm_dn6)), ((locals.var_tx_dn7 * locals.var_dnm) + (assign61250_e95411 * locals.var_dnm_dn7)), ((locals.var_tx_dn8 * locals.var_dnm) + (assign61250_e95411 * locals.var_dnm_dn8)), ((locals.var_tx_dn9 * locals.var_dnm) + (assign61250_e95411 * locals.var_dnm_dn9)), ((locals.var_tx_dn10 * locals.var_dnm) + (assign61250_e95411 * locals.var_dnm_dn10)), ((locals.var_tx_dn11 * locals.var_dnm) + (assign61250_e95411 * locals.var_dnm_dn11)), ((locals.var_tx_dn14 * locals.var_dnm) + (assign61250_e95411 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign61250_e95415;
        locals.var_ty_dn0 = assign61250_e95415_d_n0;
        locals.var_ty_dn2 = assign61250_e95415_d_n2;
        locals.var_ty_dn4 = assign61250_e95415_d_n4;
        locals.var_ty_dn5 = assign61250_e95415_d_n5;
        locals.var_ty_dn6 = assign61250_e95415_d_n6;
        locals.var_ty_dn7 = assign61250_e95415_d_n7;
        locals.var_ty_dn8 = assign61250_e95415_d_n8;
        locals.var_ty_dn9 = assign61250_e95415_d_n9;
        locals.var_ty_dn10 = assign61250_e95415_d_n10;
        locals.var_ty_dn11 = assign61250_e95415_d_n11;
        locals.var_ty_dn14 = assign61250_e95415_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign61260_e95430, assign61260_e95430_d_n0, assign61260_e95430_d_n2, assign61260_e95430_d_n4, assign61260_e95430_d_n5, assign61260_e95430_d_n6, assign61260_e95430_d_n7, assign61260_e95430_d_n8, assign61260_e95430_d_n9, assign61260_e95430_d_n10, assign61260_e95430_d_n11, assign61260_e95430_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign61260_e95424: f64 = locals.var_xmp;
        let assign61260_e95426: f64 = (assign61260_e95424 * locals.var_dnm);
        let assign61260_e95428: f64 = (assign61260_e95426 / locals.var_arg);
        (assign61260_e95428, (((((locals.var_xmp_dn0 * locals.var_dnm) + (assign61260_e95424 * locals.var_dnm_dn0)) * locals.var_arg) - (assign61260_e95426 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn2 * locals.var_dnm) + (assign61260_e95424 * locals.var_dnm_dn2)) * locals.var_arg) - (assign61260_e95426 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn4 * locals.var_dnm) + (assign61260_e95424 * locals.var_dnm_dn4)) * locals.var_arg) - (assign61260_e95426 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn5 * locals.var_dnm) + (assign61260_e95424 * locals.var_dnm_dn5)) * locals.var_arg) - (assign61260_e95426 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn6 * locals.var_dnm) + (assign61260_e95424 * locals.var_dnm_dn6)) * locals.var_arg) - (assign61260_e95426 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn7 * locals.var_dnm) + (assign61260_e95424 * locals.var_dnm_dn7)) * locals.var_arg) - (assign61260_e95426 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn8 * locals.var_dnm) + (assign61260_e95424 * locals.var_dnm_dn8)) * locals.var_arg) - (assign61260_e95426 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn9 * locals.var_dnm) + (assign61260_e95424 * locals.var_dnm_dn9)) * locals.var_arg) - (assign61260_e95426 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn10 * locals.var_dnm) + (assign61260_e95424 * locals.var_dnm_dn10)) * locals.var_arg) - (assign61260_e95426 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn11 * locals.var_dnm) + (assign61260_e95424 * locals.var_dnm_dn11)) * locals.var_arg) - (assign61260_e95426 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn14 * locals.var_dnm) + (assign61260_e95424 * locals.var_dnm_dn14)) * locals.var_arg) - (assign61260_e95426 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign61260_e95430;
        locals.var_t4_dn0 = assign61260_e95430_d_n0;
        locals.var_t4_dn2 = assign61260_e95430_d_n2;
        locals.var_t4_dn4 = assign61260_e95430_d_n4;
        locals.var_t4_dn5 = assign61260_e95430_d_n5;
        locals.var_t4_dn6 = assign61260_e95430_d_n6;
        locals.var_t4_dn7 = assign61260_e95430_d_n7;
        locals.var_t4_dn8 = assign61260_e95430_d_n8;
        locals.var_t4_dn9 = assign61260_e95430_d_n9;
        locals.var_t4_dn10 = assign61260_e95430_d_n10;
        locals.var_t4_dn11 = assign61260_e95430_d_n11;
        locals.var_t4_dn14 = assign61260_e95430_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign61270_e95441, assign61270_e95441_d_n0, assign61270_e95441_d_n2, assign61270_e95441_d_n4, assign61270_e95441_d_n5, assign61270_e95441_d_n6, assign61270_e95441_d_n7, assign61270_e95441_d_n8, assign61270_e95441_d_n9, assign61270_e95441_d_n10, assign61270_e95441_d_n11, assign61270_e95441_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign61270_e95439: f64 = (1.0 - locals.var_ty);
        (assign61270_e95439, (-locals.var_ty_dn0), (-locals.var_ty_dn2), (-locals.var_ty_dn4), (-locals.var_ty_dn5), (-locals.var_ty_dn6), (-locals.var_ty_dn7), (-locals.var_ty_dn8), (-locals.var_ty_dn9), (-locals.var_ty_dn10), (-locals.var_ty_dn11), (-locals.var_ty_dn14),)
    } else {
        (locals.var_alpha, locals.var_alpha_dn0, locals.var_alpha_dn2, locals.var_alpha_dn4, locals.var_alpha_dn5, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn8, locals.var_alpha_dn9, locals.var_alpha_dn10, locals.var_alpha_dn11, locals.var_alpha_dn14,)
    }
};
        locals.var_alpha = assign61270_e95441;
        locals.var_alpha_dn0 = assign61270_e95441_d_n0;
        locals.var_alpha_dn2 = assign61270_e95441_d_n2;
        locals.var_alpha_dn4 = assign61270_e95441_d_n4;
        locals.var_alpha_dn5 = assign61270_e95441_d_n5;
        locals.var_alpha_dn6 = assign61270_e95441_d_n6;
        locals.var_alpha_dn7 = assign61270_e95441_d_n7;
        locals.var_alpha_dn8 = assign61270_e95441_d_n8;
        locals.var_alpha_dn9 = assign61270_e95441_d_n9;
        locals.var_alpha_dn10 = assign61270_e95441_d_n10;
        locals.var_alpha_dn11 = assign61270_e95441_d_n11;
        locals.var_alpha_dn14 = assign61270_e95441_d_n14;
        locals.var_alpha_rv = 0.0;

        let (assign61280_e95456, assign61280_e95456_d_n0, assign61280_e95456_d_n2, assign61280_e95456_d_n4, assign61280_e95456_d_n5, assign61280_e95456_d_n6, assign61280_e95456_d_n7, assign61280_e95456_d_n8, assign61280_e95456_d_n9, assign61280_e95456_d_n10, assign61280_e95456_d_n11, assign61280_e95456_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign61280_e95452: f64 = (1.0 + locals.var_alpha);
        let assign61280_e95453: f64 = (locals.var_alpha * assign61280_e95452);
        let assign61280_e95454: f64 = (1.0 + assign61280_e95453);
        (assign61280_e95454, ((locals.var_alpha_dn0 * assign61280_e95452) + (locals.var_alpha * locals.var_alpha_dn0)), ((locals.var_alpha_dn2 * assign61280_e95452) + (locals.var_alpha * locals.var_alpha_dn2)), ((locals.var_alpha_dn4 * assign61280_e95452) + (locals.var_alpha * locals.var_alpha_dn4)), ((locals.var_alpha_dn5 * assign61280_e95452) + (locals.var_alpha * locals.var_alpha_dn5)), ((locals.var_alpha_dn6 * assign61280_e95452) + (locals.var_alpha * locals.var_alpha_dn6)), ((locals.var_alpha_dn7 * assign61280_e95452) + (locals.var_alpha * locals.var_alpha_dn7)), ((locals.var_alpha_dn8 * assign61280_e95452) + (locals.var_alpha * locals.var_alpha_dn8)), ((locals.var_alpha_dn9 * assign61280_e95452) + (locals.var_alpha * locals.var_alpha_dn9)), ((locals.var_alpha_dn10 * assign61280_e95452) + (locals.var_alpha * locals.var_alpha_dn10)), ((locals.var_alpha_dn11 * assign61280_e95452) + (locals.var_alpha * locals.var_alpha_dn11)), ((locals.var_alpha_dn14 * assign61280_e95452) + (locals.var_alpha * locals.var_alpha_dn14)),)
    } else {
        (locals.var_qinm, locals.var_qinm_dn0, locals.var_qinm_dn2, locals.var_qinm_dn4, locals.var_qinm_dn5, locals.var_qinm_dn6, locals.var_qinm_dn7, locals.var_qinm_dn8, locals.var_qinm_dn9, locals.var_qinm_dn10, locals.var_qinm_dn11, locals.var_qinm_dn14,)
    }
};
        locals.var_qinm = assign61280_e95456;
        locals.var_qinm_dn0 = assign61280_e95456_d_n0;
        locals.var_qinm_dn2 = assign61280_e95456_d_n2;
        locals.var_qinm_dn4 = assign61280_e95456_d_n4;
        locals.var_qinm_dn5 = assign61280_e95456_d_n5;
        locals.var_qinm_dn6 = assign61280_e95456_d_n6;
        locals.var_qinm_dn7 = assign61280_e95456_d_n7;
        locals.var_qinm_dn8 = assign61280_e95456_d_n8;
        locals.var_qinm_dn9 = assign61280_e95456_d_n9;
        locals.var_qinm_dn10 = assign61280_e95456_d_n10;
        locals.var_qinm_dn11 = assign61280_e95456_d_n11;
        locals.var_qinm_dn14 = assign61280_e95456_d_n14;
        locals.var_qinm_rv = 0.0;

        let assign61290_e95459: f64 = (1.0 + locals.var_alpha);
        let assign61290_e95462: f64 = (10.0 * 2.220446049250313e-16);
        let assign61290_e95465: f64 = (10.0 * 2.220446049250313e-16);
        let assign61290_e95466: f64 = (assign61290_e95462 + assign61290_e95465);
        let assign61290_e95470: f64 = (10.0 * 2.220446049250313e-16);
        let assign61290_e95473: f64 = if ((assign61290_e95459 < assign61290_e95466) && (assign61290_e95470 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1484 = assign61290_e95473;
        locals.var_guard1484_rv = 0.0;

        let (assign61300_e95494, assign61300_e95494_d_n0, assign61300_e95494_d_n2, assign61300_e95494_d_n4, assign61300_e95494_d_n5, assign61300_e95494_d_n6, assign61300_e95494_d_n7, assign61300_e95494_d_n8, assign61300_e95494_d_n9, assign61300_e95494_d_n10, assign61300_e95494_d_n11, assign61300_e95494_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1484 != 0.0)) {
        let assign61300_e95484: f64 = (10.0 * 2.220446049250313e-16);
        let assign61300_e95487: f64 = (10.0 * 2.220446049250313e-16);
        let assign61300_e95488: f64 = (assign61300_e95484 + assign61300_e95487);
        let assign61300_e95491: f64 = (1.0 + locals.var_alpha);
        let assign61300_e95492: f64 = (assign61300_e95488 - assign61300_e95491);
        (assign61300_e95492, (-locals.var_alpha_dn0), (-locals.var_alpha_dn2), (-locals.var_alpha_dn4), (-locals.var_alpha_dn5), (-locals.var_alpha_dn6), (-locals.var_alpha_dn7), (-locals.var_alpha_dn8), (-locals.var_alpha_dn9), (-locals.var_alpha_dn10), (-locals.var_alpha_dn11), (-locals.var_alpha_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign61300_e95494;
        locals.var_tmf1_dn0 = assign61300_e95494_d_n0;
        locals.var_tmf1_dn2 = assign61300_e95494_d_n2;
        locals.var_tmf1_dn4 = assign61300_e95494_d_n4;
        locals.var_tmf1_dn5 = assign61300_e95494_d_n5;
        locals.var_tmf1_dn6 = assign61300_e95494_d_n6;
        locals.var_tmf1_dn7 = assign61300_e95494_d_n7;
        locals.var_tmf1_dn8 = assign61300_e95494_d_n8;
        locals.var_tmf1_dn9 = assign61300_e95494_d_n9;
        locals.var_tmf1_dn10 = assign61300_e95494_d_n10;
        locals.var_tmf1_dn11 = assign61300_e95494_d_n11;
        locals.var_tmf1_dn14 = assign61300_e95494_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign61310_e95507, assign61310_e95507_d_n0, assign61310_e95507_d_n2, assign61310_e95507_d_n4, assign61310_e95507_d_n5, assign61310_e95507_d_n6, assign61310_e95507_d_n7, assign61310_e95507_d_n8, assign61310_e95507_d_n9, assign61310_e95507_d_n10, assign61310_e95507_d_n11, assign61310_e95507_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1484 != 0.0)) {
        let assign61310_e95505: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign61310_e95505, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign61310_e95507;
        locals.var_x2_dn0 = assign61310_e95507_d_n0;
        locals.var_x2_dn2 = assign61310_e95507_d_n2;
        locals.var_x2_dn4 = assign61310_e95507_d_n4;
        locals.var_x2_dn5 = assign61310_e95507_d_n5;
        locals.var_x2_dn6 = assign61310_e95507_d_n6;
        locals.var_x2_dn7 = assign61310_e95507_d_n7;
        locals.var_x2_dn8 = assign61310_e95507_d_n8;
        locals.var_x2_dn9 = assign61310_e95507_d_n9;
        locals.var_x2_dn10 = assign61310_e95507_d_n10;
        locals.var_x2_dn11 = assign61310_e95507_d_n11;
        locals.var_x2_dn14 = assign61310_e95507_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign61320_e95524, assign61320_e95524_d_n0, assign61320_e95524_d_n2, assign61320_e95524_d_n4, assign61320_e95524_d_n5, assign61320_e95524_d_n6, assign61320_e95524_d_n7, assign61320_e95524_d_n8, assign61320_e95524_d_n9, assign61320_e95524_d_n10, assign61320_e95524_d_n11, assign61320_e95524_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1484 != 0.0)) {
        let assign61320_e95518: f64 = (10.0 * 2.220446049250313e-16);
        let assign61320_e95521: f64 = (10.0 * 2.220446049250313e-16);
        let assign61320_e95522: f64 = (assign61320_e95518 * assign61320_e95521);
        (assign61320_e95522, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign61320_e95524;
        locals.var_xmax2_dn0 = assign61320_e95524_d_n0;
        locals.var_xmax2_dn2 = assign61320_e95524_d_n2;
        locals.var_xmax2_dn4 = assign61320_e95524_d_n4;
        locals.var_xmax2_dn5 = assign61320_e95524_d_n5;
        locals.var_xmax2_dn6 = assign61320_e95524_d_n6;
        locals.var_xmax2_dn7 = assign61320_e95524_d_n7;
        locals.var_xmax2_dn8 = assign61320_e95524_d_n8;
        locals.var_xmax2_dn9 = assign61320_e95524_d_n9;
        locals.var_xmax2_dn10 = assign61320_e95524_d_n10;
        locals.var_xmax2_dn11 = assign61320_e95524_d_n11;
        locals.var_xmax2_dn14 = assign61320_e95524_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign61330_e95535, assign61330_e95535_d_n0, assign61330_e95535_d_n2, assign61330_e95535_d_n4, assign61330_e95535_d_n5, assign61330_e95535_d_n6, assign61330_e95535_d_n7, assign61330_e95535_d_n8, assign61330_e95535_d_n9, assign61330_e95535_d_n10, assign61330_e95535_d_n11, assign61330_e95535_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1484 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign61330_e95535;
        locals.var_xp_dn0 = assign61330_e95535_d_n0;
        locals.var_xp_dn2 = assign61330_e95535_d_n2;
        locals.var_xp_dn4 = assign61330_e95535_d_n4;
        locals.var_xp_dn5 = assign61330_e95535_d_n5;
        locals.var_xp_dn6 = assign61330_e95535_d_n6;
        locals.var_xp_dn7 = assign61330_e95535_d_n7;
        locals.var_xp_dn8 = assign61330_e95535_d_n8;
        locals.var_xp_dn9 = assign61330_e95535_d_n9;
        locals.var_xp_dn10 = assign61330_e95535_d_n10;
        locals.var_xp_dn11 = assign61330_e95535_d_n11;
        locals.var_xp_dn14 = assign61330_e95535_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign61340_e95546, assign61340_e95546_d_n0, assign61340_e95546_d_n2, assign61340_e95546_d_n4, assign61340_e95546_d_n5, assign61340_e95546_d_n6, assign61340_e95546_d_n7, assign61340_e95546_d_n8, assign61340_e95546_d_n9, assign61340_e95546_d_n10, assign61340_e95546_d_n11, assign61340_e95546_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1484 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign61340_e95546;
        locals.var_xmp_dn0 = assign61340_e95546_d_n0;
        locals.var_xmp_dn2 = assign61340_e95546_d_n2;
        locals.var_xmp_dn4 = assign61340_e95546_d_n4;
        locals.var_xmp_dn5 = assign61340_e95546_d_n5;
        locals.var_xmp_dn6 = assign61340_e95546_d_n6;
        locals.var_xmp_dn7 = assign61340_e95546_d_n7;
        locals.var_xmp_dn8 = assign61340_e95546_d_n8;
        locals.var_xmp_dn9 = assign61340_e95546_d_n9;
        locals.var_xmp_dn10 = assign61340_e95546_d_n10;
        locals.var_xmp_dn11 = assign61340_e95546_d_n11;
        locals.var_xmp_dn14 = assign61340_e95546_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign61350_e95557,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1484 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign61350_e95557;
        locals.var_m0_rv = 0.0;

        let (assign61360_e95568,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1484 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign61360_e95568;
        locals.var_mm_rv = 0.0;

        let (assign61370_e95579, assign61370_e95579_d_n0, assign61370_e95579_d_n2, assign61370_e95579_d_n4, assign61370_e95579_d_n5, assign61370_e95579_d_n6, assign61370_e95579_d_n7, assign61370_e95579_d_n8, assign61370_e95579_d_n9, assign61370_e95579_d_n10, assign61370_e95579_d_n11, assign61370_e95579_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1484 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign61370_e95579;
        locals.var_arg_dn0 = assign61370_e95579_d_n0;
        locals.var_arg_dn2 = assign61370_e95579_d_n2;
        locals.var_arg_dn4 = assign61370_e95579_d_n4;
        locals.var_arg_dn5 = assign61370_e95579_d_n5;
        locals.var_arg_dn6 = assign61370_e95579_d_n6;
        locals.var_arg_dn7 = assign61370_e95579_d_n7;
        locals.var_arg_dn8 = assign61370_e95579_d_n8;
        locals.var_arg_dn9 = assign61370_e95579_d_n9;
        locals.var_arg_dn10 = assign61370_e95579_d_n10;
        locals.var_arg_dn11 = assign61370_e95579_d_n11;
        locals.var_arg_dn14 = assign61370_e95579_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign61380_e95590, assign61380_e95590_d_n0, assign61380_e95590_d_n2, assign61380_e95590_d_n4, assign61380_e95590_d_n5, assign61380_e95590_d_n6, assign61380_e95590_d_n7, assign61380_e95590_d_n8, assign61380_e95590_d_n9, assign61380_e95590_d_n10, assign61380_e95590_d_n11, assign61380_e95590_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1484 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign61380_e95590;
        locals.var_dnm_dn0 = assign61380_e95590_d_n0;
        locals.var_dnm_dn2 = assign61380_e95590_d_n2;
        locals.var_dnm_dn4 = assign61380_e95590_d_n4;
        locals.var_dnm_dn5 = assign61380_e95590_d_n5;
        locals.var_dnm_dn6 = assign61380_e95590_d_n6;
        locals.var_dnm_dn7 = assign61380_e95590_d_n7;
        locals.var_dnm_dn8 = assign61380_e95590_d_n8;
        locals.var_dnm_dn9 = assign61380_e95590_d_n9;
        locals.var_dnm_dn10 = assign61380_e95590_d_n10;
        locals.var_dnm_dn11 = assign61380_e95590_d_n11;
        locals.var_dnm_dn14 = assign61380_e95590_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign61390_e95603, assign61390_e95603_d_n0, assign61390_e95603_d_n2, assign61390_e95603_d_n4, assign61390_e95603_d_n5, assign61390_e95603_d_n6, assign61390_e95603_d_n7, assign61390_e95603_d_n8, assign61390_e95603_d_n9, assign61390_e95603_d_n10, assign61390_e95603_d_n11, assign61390_e95603_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1484 != 0.0)) {
        let assign61390_e95601: f64 = (locals.var_xp * locals.var_x2);
        (assign61390_e95601, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign61390_e95603;
        locals.var_xp_dn0 = assign61390_e95603_d_n0;
        locals.var_xp_dn2 = assign61390_e95603_d_n2;
        locals.var_xp_dn4 = assign61390_e95603_d_n4;
        locals.var_xp_dn5 = assign61390_e95603_d_n5;
        locals.var_xp_dn6 = assign61390_e95603_d_n6;
        locals.var_xp_dn7 = assign61390_e95603_d_n7;
        locals.var_xp_dn8 = assign61390_e95603_d_n8;
        locals.var_xp_dn9 = assign61390_e95603_d_n9;
        locals.var_xp_dn10 = assign61390_e95603_d_n10;
        locals.var_xp_dn11 = assign61390_e95603_d_n11;
        locals.var_xp_dn14 = assign61390_e95603_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign61400_e95616, assign61400_e95616_d_n0, assign61400_e95616_d_n2, assign61400_e95616_d_n4, assign61400_e95616_d_n5, assign61400_e95616_d_n6, assign61400_e95616_d_n7, assign61400_e95616_d_n8, assign61400_e95616_d_n9, assign61400_e95616_d_n10, assign61400_e95616_d_n11, assign61400_e95616_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1484 != 0.0)) {
        let assign61400_e95614: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign61400_e95614, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign61400_e95616;
        locals.var_xmp_dn0 = assign61400_e95616_d_n0;
        locals.var_xmp_dn2 = assign61400_e95616_d_n2;
        locals.var_xmp_dn4 = assign61400_e95616_d_n4;
        locals.var_xmp_dn5 = assign61400_e95616_d_n5;
        locals.var_xmp_dn6 = assign61400_e95616_d_n6;
        locals.var_xmp_dn7 = assign61400_e95616_d_n7;
        locals.var_xmp_dn8 = assign61400_e95616_d_n8;
        locals.var_xmp_dn9 = assign61400_e95616_d_n9;
        locals.var_xmp_dn10 = assign61400_e95616_d_n10;
        locals.var_xmp_dn11 = assign61400_e95616_d_n11;
        locals.var_xmp_dn14 = assign61400_e95616_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign61410_e95629, assign61410_e95629_d_n0, assign61410_e95629_d_n2, assign61410_e95629_d_n4, assign61410_e95629_d_n5, assign61410_e95629_d_n6, assign61410_e95629_d_n7, assign61410_e95629_d_n8, assign61410_e95629_d_n9, assign61410_e95629_d_n10, assign61410_e95629_d_n11, assign61410_e95629_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1484 != 0.0)) {
        let assign61410_e95627: f64 = (locals.var_xp * locals.var_x2);
        (assign61410_e95627, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign61410_e95629;
        locals.var_xp_dn0 = assign61410_e95629_d_n0;
        locals.var_xp_dn2 = assign61410_e95629_d_n2;
        locals.var_xp_dn4 = assign61410_e95629_d_n4;
        locals.var_xp_dn5 = assign61410_e95629_d_n5;
        locals.var_xp_dn6 = assign61410_e95629_d_n6;
        locals.var_xp_dn7 = assign61410_e95629_d_n7;
        locals.var_xp_dn8 = assign61410_e95629_d_n8;
        locals.var_xp_dn9 = assign61410_e95629_d_n9;
        locals.var_xp_dn10 = assign61410_e95629_d_n10;
        locals.var_xp_dn11 = assign61410_e95629_d_n11;
        locals.var_xp_dn14 = assign61410_e95629_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign61420_e95642, assign61420_e95642_d_n0, assign61420_e95642_d_n2, assign61420_e95642_d_n4, assign61420_e95642_d_n5, assign61420_e95642_d_n6, assign61420_e95642_d_n7, assign61420_e95642_d_n8, assign61420_e95642_d_n9, assign61420_e95642_d_n10, assign61420_e95642_d_n11, assign61420_e95642_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1484 != 0.0)) {
        let assign61420_e95640: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign61420_e95640, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign61420_e95642;
        locals.var_xmp_dn0 = assign61420_e95642_d_n0;
        locals.var_xmp_dn2 = assign61420_e95642_d_n2;
        locals.var_xmp_dn4 = assign61420_e95642_d_n4;
        locals.var_xmp_dn5 = assign61420_e95642_d_n5;
        locals.var_xmp_dn6 = assign61420_e95642_d_n6;
        locals.var_xmp_dn7 = assign61420_e95642_d_n7;
        locals.var_xmp_dn8 = assign61420_e95642_d_n8;
        locals.var_xmp_dn9 = assign61420_e95642_d_n9;
        locals.var_xmp_dn10 = assign61420_e95642_d_n10;
        locals.var_xmp_dn11 = assign61420_e95642_d_n11;
        locals.var_xmp_dn14 = assign61420_e95642_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign61430_e95655, assign61430_e95655_d_n0, assign61430_e95655_d_n2, assign61430_e95655_d_n4, assign61430_e95655_d_n5, assign61430_e95655_d_n6, assign61430_e95655_d_n7, assign61430_e95655_d_n8, assign61430_e95655_d_n9, assign61430_e95655_d_n10, assign61430_e95655_d_n11, assign61430_e95655_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1484 != 0.0)) {
        let assign61430_e95653: f64 = (locals.var_xp + locals.var_xmp);
        (assign61430_e95653, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign61430_e95655;
        locals.var_arg_dn0 = assign61430_e95655_d_n0;
        locals.var_arg_dn2 = assign61430_e95655_d_n2;
        locals.var_arg_dn4 = assign61430_e95655_d_n4;
        locals.var_arg_dn5 = assign61430_e95655_d_n5;
        locals.var_arg_dn6 = assign61430_e95655_d_n6;
        locals.var_arg_dn7 = assign61430_e95655_d_n7;
        locals.var_arg_dn8 = assign61430_e95655_d_n8;
        locals.var_arg_dn9 = assign61430_e95655_d_n9;
        locals.var_arg_dn10 = assign61430_e95655_d_n10;
        locals.var_arg_dn11 = assign61430_e95655_d_n11;
        locals.var_arg_dn14 = assign61430_e95655_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign61440_e95666, assign61440_e95666_d_n0, assign61440_e95666_d_n2, assign61440_e95666_d_n4, assign61440_e95666_d_n5, assign61440_e95666_d_n6, assign61440_e95666_d_n7, assign61440_e95666_d_n8, assign61440_e95666_d_n9, assign61440_e95666_d_n10, assign61440_e95666_d_n11, assign61440_e95666_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1484 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign61440_e95666;
        locals.var_dnm_dn0 = assign61440_e95666_d_n0;
        locals.var_dnm_dn2 = assign61440_e95666_d_n2;
        locals.var_dnm_dn4 = assign61440_e95666_d_n4;
        locals.var_dnm_dn5 = assign61440_e95666_d_n5;
        locals.var_dnm_dn6 = assign61440_e95666_d_n6;
        locals.var_dnm_dn7 = assign61440_e95666_d_n7;
        locals.var_dnm_dn8 = assign61440_e95666_d_n8;
        locals.var_dnm_dn9 = assign61440_e95666_d_n9;
        locals.var_dnm_dn10 = assign61440_e95666_d_n10;
        locals.var_dnm_dn11 = assign61440_e95666_d_n11;
        locals.var_dnm_dn14 = assign61440_e95666_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign61450_e95681: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1485 = assign61450_e95681;
        locals.var_guard1485_rv = 0.0;

        let assign61460_e95684: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1486 = assign61460_e95684;
        locals.var_guard1486_rv = 0.0;

        let (assign61470_e95699,) = {
    if ((((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1486 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign61470_e95699;
        locals.var_mm_rv = 0.0;

        let assign61480_e95702: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1487 = assign61480_e95702;
        locals.var_guard1487_rv = 0.0;

        let (assign61490_e95720,) = {
    if (((((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1486 == 0.0)) && (locals.var_guard1487 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign61490_e95720;
        locals.var_mm_rv = 0.0;

        let assign61500_e95723: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1488 = assign61500_e95723;
        locals.var_guard1488_rv = 0.0;

        let (assign61510_e95744,) = {
    if ((((((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1486 == 0.0)) && (locals.var_guard1487 == 0.0)) && (locals.var_guard1488 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign61510_e95744;
        locals.var_mm_rv = 0.0;

        let assign61520_e95747: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1489 = assign61520_e95747;
        locals.var_guard1489_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_229(
        locals: &mut StampLocals,
    ) {
        let (assign61530_e95771,) = {
    if (((((((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_guard1486 == 0.0)) && (locals.var_guard1487 == 0.0)) && (locals.var_guard1488 == 0.0)) && (locals.var_guard1489 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign61530_e95771;
        locals.var_mm_rv = 0.0;

        let (assign61540_e95784,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign61540_e95784;
        locals.var_m0_rv = 0.0;

        let mut assign61550_loop_guard: usize = 0;
        while {
            let assign61550_cond_e95798: f64 = if ((((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign61550_cond_e95798 != 0.0
        } {
            assign61550_loop_guard += 1;
            assert!(assign61550_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign61550_body0_e95812, assign61550_body0_e95812_d_n0, assign61550_body0_e95812_d_n2, assign61550_body0_e95812_d_n4, assign61550_body0_e95812_d_n5, assign61550_body0_e95812_d_n6, assign61550_body0_e95812_d_n7, assign61550_body0_e95812_d_n8, assign61550_body0_e95812_d_n9, assign61550_body0_e95812_d_n10, assign61550_body0_e95812_d_n11, assign61550_body0_e95812_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) {
        let assign61550_body0_e95810: f64 = (locals.var_dnm).sqrt();
        (assign61550_body0_e95810, (locals.var_dnm_dn0 / (2.0 * assign61550_body0_e95810)), (locals.var_dnm_dn2 / (2.0 * assign61550_body0_e95810)), (locals.var_dnm_dn4 / (2.0 * assign61550_body0_e95810)), (locals.var_dnm_dn5 / (2.0 * assign61550_body0_e95810)), (locals.var_dnm_dn6 / (2.0 * assign61550_body0_e95810)), (locals.var_dnm_dn7 / (2.0 * assign61550_body0_e95810)), (locals.var_dnm_dn8 / (2.0 * assign61550_body0_e95810)), (locals.var_dnm_dn9 / (2.0 * assign61550_body0_e95810)), (locals.var_dnm_dn10 / (2.0 * assign61550_body0_e95810)), (locals.var_dnm_dn11 / (2.0 * assign61550_body0_e95810)), (locals.var_dnm_dn14 / (2.0 * assign61550_body0_e95810)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign61550_body0_e95812;
            locals.var_dnm_dn0 = assign61550_body0_e95812_d_n0;
            locals.var_dnm_dn2 = assign61550_body0_e95812_d_n2;
            locals.var_dnm_dn4 = assign61550_body0_e95812_d_n4;
            locals.var_dnm_dn5 = assign61550_body0_e95812_d_n5;
            locals.var_dnm_dn6 = assign61550_body0_e95812_d_n6;
            locals.var_dnm_dn7 = assign61550_body0_e95812_d_n7;
            locals.var_dnm_dn8 = assign61550_body0_e95812_d_n8;
            locals.var_dnm_dn9 = assign61550_body0_e95812_d_n9;
            locals.var_dnm_dn10 = assign61550_body0_e95812_d_n10;
            locals.var_dnm_dn11 = assign61550_body0_e95812_d_n11;
            locals.var_dnm_dn14 = assign61550_body0_e95812_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign61550_body1_e95827,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 != 0.0)) {
        let assign61550_body1_e95825: f64 = (locals.var_m0 + 1.0);
        (assign61550_body1_e95825,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign61550_body1_e95827;
            locals.var_m0_rv = 0.0;
        }

        let (assign61560_e95852, assign61560_e95852_d_n0, assign61560_e95852_d_n2, assign61560_e95852_d_n4, assign61560_e95852_d_n5, assign61560_e95852_d_n6, assign61560_e95852_d_n7, assign61560_e95852_d_n8, assign61560_e95852_d_n9, assign61560_e95852_d_n10, assign61560_e95852_d_n11, assign61560_e95852_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1484 != 0.0)) && (locals.var_guard1485 == 0.0)) {
        let (assign61560_e95850, assign61560_e95850_d_n0, assign61560_e95850_d_n2, assign61560_e95850_d_n4, assign61560_e95850_d_n5, assign61560_e95850_d_n6, assign61560_e95850_d_n7, assign61560_e95850_d_n8, assign61560_e95850_d_n9, assign61560_e95850_d_n10, assign61560_e95850_d_n11, assign61560_e95850_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign61560_e95847: f64 = (2.0 * 2.0);
                let assign61560_e95848: f64 = (1.0 / assign61560_e95847);
                let assign61560_e95849: f64 = (locals.var_dnm).powf(assign61560_e95848);
                (assign61560_e95849, if 0.0 == 0.0 && ((assign61560_e95848) as f64).is_finite() && ((assign61560_e95848) as f64).fract() == 0.0 { if assign61560_e95848 == 0.0 { 0.0 } else { (assign61560_e95848 * ((locals.var_dnm).powf(assign61560_e95848 - 1.0) * locals.var_dnm_dn0)) } } else { (assign61560_e95849 * (assign61560_e95848 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign61560_e95848) as f64).is_finite() && ((assign61560_e95848) as f64).fract() == 0.0 { if assign61560_e95848 == 0.0 { 0.0 } else { (assign61560_e95848 * ((locals.var_dnm).powf(assign61560_e95848 - 1.0) * locals.var_dnm_dn2)) } } else { (assign61560_e95849 * (assign61560_e95848 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign61560_e95848) as f64).is_finite() && ((assign61560_e95848) as f64).fract() == 0.0 { if assign61560_e95848 == 0.0 { 0.0 } else { (assign61560_e95848 * ((locals.var_dnm).powf(assign61560_e95848 - 1.0) * locals.var_dnm_dn4)) } } else { (assign61560_e95849 * (assign61560_e95848 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign61560_e95848) as f64).is_finite() && ((assign61560_e95848) as f64).fract() == 0.0 { if assign61560_e95848 == 0.0 { 0.0 } else { (assign61560_e95848 * ((locals.var_dnm).powf(assign61560_e95848 - 1.0) * locals.var_dnm_dn5)) } } else { (assign61560_e95849 * (assign61560_e95848 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign61560_e95848) as f64).is_finite() && ((assign61560_e95848) as f64).fract() == 0.0 { if assign61560_e95848 == 0.0 { 0.0 } else { (assign61560_e95848 * ((locals.var_dnm).powf(assign61560_e95848 - 1.0) * locals.var_dnm_dn6)) } } else { (assign61560_e95849 * (assign61560_e95848 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign61560_e95848) as f64).is_finite() && ((assign61560_e95848) as f64).fract() == 0.0 { if assign61560_e95848 == 0.0 { 0.0 } else { (assign61560_e95848 * ((locals.var_dnm).powf(assign61560_e95848 - 1.0) * locals.var_dnm_dn7)) } } else { (assign61560_e95849 * (assign61560_e95848 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign61560_e95848) as f64).is_finite() && ((assign61560_e95848) as f64).fract() == 0.0 { if assign61560_e95848 == 0.0 { 0.0 } else { (assign61560_e95848 * ((locals.var_dnm).powf(assign61560_e95848 - 1.0) * locals.var_dnm_dn8)) } } else { (assign61560_e95849 * (assign61560_e95848 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign61560_e95848) as f64).is_finite() && ((assign61560_e95848) as f64).fract() == 0.0 { if assign61560_e95848 == 0.0 { 0.0 } else { (assign61560_e95848 * ((locals.var_dnm).powf(assign61560_e95848 - 1.0) * locals.var_dnm_dn9)) } } else { (assign61560_e95849 * (assign61560_e95848 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign61560_e95848) as f64).is_finite() && ((assign61560_e95848) as f64).fract() == 0.0 { if assign61560_e95848 == 0.0 { 0.0 } else { (assign61560_e95848 * ((locals.var_dnm).powf(assign61560_e95848 - 1.0) * locals.var_dnm_dn10)) } } else { (assign61560_e95849 * (assign61560_e95848 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign61560_e95848) as f64).is_finite() && ((assign61560_e95848) as f64).fract() == 0.0 { if assign61560_e95848 == 0.0 { 0.0 } else { (assign61560_e95848 * ((locals.var_dnm).powf(assign61560_e95848 - 1.0) * locals.var_dnm_dn11)) } } else { (assign61560_e95849 * (assign61560_e95848 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign61560_e95848) as f64).is_finite() && ((assign61560_e95848) as f64).fract() == 0.0 { if assign61560_e95848 == 0.0 { 0.0 } else { (assign61560_e95848 * ((locals.var_dnm).powf(assign61560_e95848 - 1.0) * locals.var_dnm_dn14)) } } else { (assign61560_e95849 * (assign61560_e95848 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign61560_e95850, assign61560_e95850_d_n0, assign61560_e95850_d_n2, assign61560_e95850_d_n4, assign61560_e95850_d_n5, assign61560_e95850_d_n6, assign61560_e95850_d_n7, assign61560_e95850_d_n8, assign61560_e95850_d_n9, assign61560_e95850_d_n10, assign61560_e95850_d_n11, assign61560_e95850_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign61560_e95852;
        locals.var_dnm_dn0 = assign61560_e95852_d_n0;
        locals.var_dnm_dn2 = assign61560_e95852_d_n2;
        locals.var_dnm_dn4 = assign61560_e95852_d_n4;
        locals.var_dnm_dn5 = assign61560_e95852_d_n5;
        locals.var_dnm_dn6 = assign61560_e95852_d_n6;
        locals.var_dnm_dn7 = assign61560_e95852_d_n7;
        locals.var_dnm_dn8 = assign61560_e95852_d_n8;
        locals.var_dnm_dn9 = assign61560_e95852_d_n9;
        locals.var_dnm_dn10 = assign61560_e95852_d_n10;
        locals.var_dnm_dn11 = assign61560_e95852_d_n11;
        locals.var_dnm_dn14 = assign61560_e95852_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign61570_e95865, assign61570_e95865_d_n0, assign61570_e95865_d_n2, assign61570_e95865_d_n4, assign61570_e95865_d_n5, assign61570_e95865_d_n6, assign61570_e95865_d_n7, assign61570_e95865_d_n8, assign61570_e95865_d_n9, assign61570_e95865_d_n10, assign61570_e95865_d_n11, assign61570_e95865_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1484 != 0.0)) {
        let assign61570_e95863: f64 = (1.0 / locals.var_dnm);
        (assign61570_e95863, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign61570_e95865;
        locals.var_dnm_dn0 = assign61570_e95865_d_n0;
        locals.var_dnm_dn2 = assign61570_e95865_d_n2;
        locals.var_dnm_dn4 = assign61570_e95865_d_n4;
        locals.var_dnm_dn5 = assign61570_e95865_d_n5;
        locals.var_dnm_dn6 = assign61570_e95865_d_n6;
        locals.var_dnm_dn7 = assign61570_e95865_d_n7;
        locals.var_dnm_dn8 = assign61570_e95865_d_n8;
        locals.var_dnm_dn9 = assign61570_e95865_d_n9;
        locals.var_dnm_dn10 = assign61570_e95865_d_n10;
        locals.var_dnm_dn11 = assign61570_e95865_d_n11;
        locals.var_dnm_dn14 = assign61570_e95865_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign61580_e95882, assign61580_e95882_d_n0, assign61580_e95882_d_n2, assign61580_e95882_d_n4, assign61580_e95882_d_n5, assign61580_e95882_d_n6, assign61580_e95882_d_n7, assign61580_e95882_d_n8, assign61580_e95882_d_n9, assign61580_e95882_d_n10, assign61580_e95882_d_n11, assign61580_e95882_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1484 != 0.0)) {
        let assign61580_e95877: f64 = (10.0 * 2.220446049250313e-16);
        let assign61580_e95878: f64 = (locals.var_tmf1 * assign61580_e95877);
        let assign61580_e95880: f64 = (assign61580_e95878 * locals.var_dnm);
        (assign61580_e95880, (((locals.var_tmf1_dn0 * assign61580_e95877) * locals.var_dnm) + (assign61580_e95878 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * assign61580_e95877) * locals.var_dnm) + (assign61580_e95878 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * assign61580_e95877) * locals.var_dnm) + (assign61580_e95878 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * assign61580_e95877) * locals.var_dnm) + (assign61580_e95878 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * assign61580_e95877) * locals.var_dnm) + (assign61580_e95878 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * assign61580_e95877) * locals.var_dnm) + (assign61580_e95878 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * assign61580_e95877) * locals.var_dnm) + (assign61580_e95878 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * assign61580_e95877) * locals.var_dnm) + (assign61580_e95878 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * assign61580_e95877) * locals.var_dnm) + (assign61580_e95878 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * assign61580_e95877) * locals.var_dnm) + (assign61580_e95878 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * assign61580_e95877) * locals.var_dnm) + (assign61580_e95878 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign61580_e95882;
        locals.var_tmf0_dn0 = assign61580_e95882_d_n0;
        locals.var_tmf0_dn2 = assign61580_e95882_d_n2;
        locals.var_tmf0_dn4 = assign61580_e95882_d_n4;
        locals.var_tmf0_dn5 = assign61580_e95882_d_n5;
        locals.var_tmf0_dn6 = assign61580_e95882_d_n6;
        locals.var_tmf0_dn7 = assign61580_e95882_d_n7;
        locals.var_tmf0_dn8 = assign61580_e95882_d_n8;
        locals.var_tmf0_dn9 = assign61580_e95882_d_n9;
        locals.var_tmf0_dn10 = assign61580_e95882_d_n10;
        locals.var_tmf0_dn11 = assign61580_e95882_d_n11;
        locals.var_tmf0_dn14 = assign61580_e95882_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign61590_e95901, assign61590_e95901_d_n0, assign61590_e95901_d_n2, assign61590_e95901_d_n4, assign61590_e95901_d_n5, assign61590_e95901_d_n6, assign61590_e95901_d_n7, assign61590_e95901_d_n8, assign61590_e95901_d_n9, assign61590_e95901_d_n10, assign61590_e95901_d_n11, assign61590_e95901_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1484 != 0.0)) {
        let assign61590_e95893: f64 = (10.0 * 2.220446049250313e-16);
        let assign61590_e95895: f64 = (assign61590_e95893 * locals.var_xmp);
        let assign61590_e95897: f64 = (assign61590_e95895 * locals.var_dnm);
        let assign61590_e95899: f64 = (assign61590_e95897 / locals.var_arg);
        (assign61590_e95899, ((((((assign61590_e95893 * locals.var_xmp_dn0) * locals.var_dnm) + (assign61590_e95895 * locals.var_dnm_dn0)) * locals.var_arg) - (assign61590_e95897 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((assign61590_e95893 * locals.var_xmp_dn2) * locals.var_dnm) + (assign61590_e95895 * locals.var_dnm_dn2)) * locals.var_arg) - (assign61590_e95897 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((assign61590_e95893 * locals.var_xmp_dn4) * locals.var_dnm) + (assign61590_e95895 * locals.var_dnm_dn4)) * locals.var_arg) - (assign61590_e95897 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((assign61590_e95893 * locals.var_xmp_dn5) * locals.var_dnm) + (assign61590_e95895 * locals.var_dnm_dn5)) * locals.var_arg) - (assign61590_e95897 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((assign61590_e95893 * locals.var_xmp_dn6) * locals.var_dnm) + (assign61590_e95895 * locals.var_dnm_dn6)) * locals.var_arg) - (assign61590_e95897 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((assign61590_e95893 * locals.var_xmp_dn7) * locals.var_dnm) + (assign61590_e95895 * locals.var_dnm_dn7)) * locals.var_arg) - (assign61590_e95897 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((assign61590_e95893 * locals.var_xmp_dn8) * locals.var_dnm) + (assign61590_e95895 * locals.var_dnm_dn8)) * locals.var_arg) - (assign61590_e95897 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((assign61590_e95893 * locals.var_xmp_dn9) * locals.var_dnm) + (assign61590_e95895 * locals.var_dnm_dn9)) * locals.var_arg) - (assign61590_e95897 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((assign61590_e95893 * locals.var_xmp_dn10) * locals.var_dnm) + (assign61590_e95895 * locals.var_dnm_dn10)) * locals.var_arg) - (assign61590_e95897 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((assign61590_e95893 * locals.var_xmp_dn11) * locals.var_dnm) + (assign61590_e95895 * locals.var_dnm_dn11)) * locals.var_arg) - (assign61590_e95897 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((assign61590_e95893 * locals.var_xmp_dn14) * locals.var_dnm) + (assign61590_e95895 * locals.var_dnm_dn14)) * locals.var_arg) - (assign61590_e95897 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign61590_e95901;
        locals.var_t0_dn0 = assign61590_e95901_d_n0;
        locals.var_t0_dn2 = assign61590_e95901_d_n2;
        locals.var_t0_dn4 = assign61590_e95901_d_n4;
        locals.var_t0_dn5 = assign61590_e95901_d_n5;
        locals.var_t0_dn6 = assign61590_e95901_d_n6;
        locals.var_t0_dn7 = assign61590_e95901_d_n7;
        locals.var_t0_dn8 = assign61590_e95901_d_n8;
        locals.var_t0_dn9 = assign61590_e95901_d_n9;
        locals.var_t0_dn10 = assign61590_e95901_d_n10;
        locals.var_t0_dn11 = assign61590_e95901_d_n11;
        locals.var_t0_dn14 = assign61590_e95901_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign61600_e95920, assign61600_e95920_d_n0, assign61600_e95920_d_n2, assign61600_e95920_d_n4, assign61600_e95920_d_n5, assign61600_e95920_d_n6, assign61600_e95920_d_n7, assign61600_e95920_d_n8, assign61600_e95920_d_n9, assign61600_e95920_d_n10, assign61600_e95920_d_n11, assign61600_e95920_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1484 != 0.0)) {
        let assign61600_e95912: f64 = (10.0 * 2.220446049250313e-16);
        let assign61600_e95915: f64 = (10.0 * 2.220446049250313e-16);
        let assign61600_e95916: f64 = (assign61600_e95912 + assign61600_e95915);
        let assign61600_e95918: f64 = (assign61600_e95916 - locals.var_tmf0);
        (assign61600_e95918, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_qidn, locals.var_qidn_dn0, locals.var_qidn_dn2, locals.var_qidn_dn4, locals.var_qidn_dn5, locals.var_qidn_dn6, locals.var_qidn_dn7, locals.var_qidn_dn8, locals.var_qidn_dn9, locals.var_qidn_dn10, locals.var_qidn_dn11, locals.var_qidn_dn14,)
    }
};
        locals.var_qidn = assign61600_e95920;
        locals.var_qidn_dn0 = assign61600_e95920_d_n0;
        locals.var_qidn_dn2 = assign61600_e95920_d_n2;
        locals.var_qidn_dn4 = assign61600_e95920_d_n4;
        locals.var_qidn_dn5 = assign61600_e95920_d_n5;
        locals.var_qidn_dn6 = assign61600_e95920_d_n6;
        locals.var_qidn_dn7 = assign61600_e95920_d_n7;
        locals.var_qidn_dn8 = assign61600_e95920_d_n8;
        locals.var_qidn_dn9 = assign61600_e95920_d_n9;
        locals.var_qidn_dn10 = assign61600_e95920_d_n10;
        locals.var_qidn_dn11 = assign61600_e95920_d_n11;
        locals.var_qidn_dn14 = assign61600_e95920_d_n14;
        locals.var_qidn_rv = 0.0;

        let (assign61610_e95931, assign61610_e95931_d_n0, assign61610_e95931_d_n2, assign61610_e95931_d_n4, assign61610_e95931_d_n5, assign61610_e95931_d_n6, assign61610_e95931_d_n7, assign61610_e95931_d_n8, assign61610_e95931_d_n9, assign61610_e95931_d_n10, assign61610_e95931_d_n11, assign61610_e95931_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1484 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign61610_e95931;
        locals.var_t0_dn0 = assign61610_e95931_d_n0;
        locals.var_t0_dn2 = assign61610_e95931_d_n2;
        locals.var_t0_dn4 = assign61610_e95931_d_n4;
        locals.var_t0_dn5 = assign61610_e95931_d_n5;
        locals.var_t0_dn6 = assign61610_e95931_d_n6;
        locals.var_t0_dn7 = assign61610_e95931_d_n7;
        locals.var_t0_dn8 = assign61610_e95931_d_n8;
        locals.var_t0_dn9 = assign61610_e95931_d_n9;
        locals.var_t0_dn10 = assign61610_e95931_d_n10;
        locals.var_t0_dn11 = assign61610_e95931_d_n11;
        locals.var_t0_dn14 = assign61610_e95931_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign61620_e95945, assign61620_e95945_d_n0, assign61620_e95945_d_n2, assign61620_e95945_d_n4, assign61620_e95945_d_n5, assign61620_e95945_d_n6, assign61620_e95945_d_n7, assign61620_e95945_d_n8, assign61620_e95945_d_n9, assign61620_e95945_d_n10, assign61620_e95945_d_n11, assign61620_e95945_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1484 == 0.0)) {
        let assign61620_e95943: f64 = (1.0 + locals.var_alpha);
        (assign61620_e95943, locals.var_alpha_dn0, locals.var_alpha_dn2, locals.var_alpha_dn4, locals.var_alpha_dn5, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn8, locals.var_alpha_dn9, locals.var_alpha_dn10, locals.var_alpha_dn11, locals.var_alpha_dn14,)
    } else {
        (locals.var_qidn, locals.var_qidn_dn0, locals.var_qidn_dn2, locals.var_qidn_dn4, locals.var_qidn_dn5, locals.var_qidn_dn6, locals.var_qidn_dn7, locals.var_qidn_dn8, locals.var_qidn_dn9, locals.var_qidn_dn10, locals.var_qidn_dn11, locals.var_qidn_dn14,)
    }
};
        locals.var_qidn = assign61620_e95945;
        locals.var_qidn_dn0 = assign61620_e95945_d_n0;
        locals.var_qidn_dn2 = assign61620_e95945_d_n2;
        locals.var_qidn_dn4 = assign61620_e95945_d_n4;
        locals.var_qidn_dn5 = assign61620_e95945_d_n5;
        locals.var_qidn_dn6 = assign61620_e95945_d_n6;
        locals.var_qidn_dn7 = assign61620_e95945_d_n7;
        locals.var_qidn_dn8 = assign61620_e95945_d_n8;
        locals.var_qidn_dn9 = assign61620_e95945_d_n9;
        locals.var_qidn_dn10 = assign61620_e95945_d_n10;
        locals.var_qidn_dn11 = assign61620_e95945_d_n11;
        locals.var_qidn_dn14 = assign61620_e95945_d_n14;
        locals.var_qidn_rv = 0.0;

        let (assign61630_e95957, assign61630_e95957_d_n0, assign61630_e95957_d_n2, assign61630_e95957_d_n4, assign61630_e95957_d_n5, assign61630_e95957_d_n6, assign61630_e95957_d_n7, assign61630_e95957_d_n8, assign61630_e95957_d_n9, assign61630_e95957_d_n10, assign61630_e95957_d_n11, assign61630_e95957_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1484 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign61630_e95957;
        locals.var_t0_dn0 = assign61630_e95957_d_n0;
        locals.var_t0_dn2 = assign61630_e95957_d_n2;
        locals.var_t0_dn4 = assign61630_e95957_d_n4;
        locals.var_t0_dn5 = assign61630_e95957_d_n5;
        locals.var_t0_dn6 = assign61630_e95957_d_n6;
        locals.var_t0_dn7 = assign61630_e95957_d_n7;
        locals.var_t0_dn8 = assign61630_e95957_d_n8;
        locals.var_t0_dn9 = assign61630_e95957_d_n9;
        locals.var_t0_dn10 = assign61630_e95957_d_n10;
        locals.var_t0_dn11 = assign61630_e95957_d_n11;
        locals.var_t0_dn14 = assign61630_e95957_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign61640_e95972, assign61640_e95972_d_n0, assign61640_e95972_d_n2, assign61640_e95972_d_n4, assign61640_e95972_d_n5, assign61640_e95972_d_n6, assign61640_e95972_d_n7, assign61640_e95972_d_n8, assign61640_e95972_d_n9, assign61640_e95972_d_n10, assign61640_e95972_d_n11, assign61640_e95972_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign61640_e95966: f64 = (0.6666666666666667 * locals.var_vgvt);
        let assign61640_e95968: f64 = (assign61640_e95966 * locals.var_qinm);
        let assign61640_e95970: f64 = (assign61640_e95968 / locals.var_qidn);
        (assign61640_e95970, ((((((0.6666666666666667 * locals.var_vgvt_dn0) * locals.var_qinm) + (assign61640_e95966 * locals.var_qinm_dn0)) * locals.var_qidn) - (assign61640_e95968 * locals.var_qidn_dn0)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn2) * locals.var_qinm) + (assign61640_e95966 * locals.var_qinm_dn2)) * locals.var_qidn) - (assign61640_e95968 * locals.var_qidn_dn2)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn4) * locals.var_qinm) + (assign61640_e95966 * locals.var_qinm_dn4)) * locals.var_qidn) - (assign61640_e95968 * locals.var_qidn_dn4)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn5) * locals.var_qinm) + (assign61640_e95966 * locals.var_qinm_dn5)) * locals.var_qidn) - (assign61640_e95968 * locals.var_qidn_dn5)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn6) * locals.var_qinm) + (assign61640_e95966 * locals.var_qinm_dn6)) * locals.var_qidn) - (assign61640_e95968 * locals.var_qidn_dn6)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn7) * locals.var_qinm) + (assign61640_e95966 * locals.var_qinm_dn7)) * locals.var_qidn) - (assign61640_e95968 * locals.var_qidn_dn7)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn8) * locals.var_qinm) + (assign61640_e95966 * locals.var_qinm_dn8)) * locals.var_qidn) - (assign61640_e95968 * locals.var_qidn_dn8)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn9) * locals.var_qinm) + (assign61640_e95966 * locals.var_qinm_dn9)) * locals.var_qidn) - (assign61640_e95968 * locals.var_qidn_dn9)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn10) * locals.var_qinm) + (assign61640_e95966 * locals.var_qinm_dn10)) * locals.var_qidn) - (assign61640_e95968 * locals.var_qidn_dn10)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn11) * locals.var_qinm) + (assign61640_e95966 * locals.var_qinm_dn11)) * locals.var_qidn) - (assign61640_e95968 * locals.var_qidn_dn11)) / (locals.var_qidn * locals.var_qidn)), ((((((0.6666666666666667 * locals.var_vgvt_dn14) * locals.var_qinm) + (assign61640_e95966 * locals.var_qinm_dn14)) * locals.var_qidn) - (assign61640_e95968 * locals.var_qidn_dn14)) / (locals.var_qidn * locals.var_qidn)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign61640_e95972;
        locals.var_t1_dn0 = assign61640_e95972_d_n0;
        locals.var_t1_dn2 = assign61640_e95972_d_n2;
        locals.var_t1_dn4 = assign61640_e95972_d_n4;
        locals.var_t1_dn5 = assign61640_e95972_d_n5;
        locals.var_t1_dn6 = assign61640_e95972_d_n6;
        locals.var_t1_dn7 = assign61640_e95972_d_n7;
        locals.var_t1_dn8 = assign61640_e95972_d_n8;
        locals.var_t1_dn9 = assign61640_e95972_d_n9;
        locals.var_t1_dn10 = assign61640_e95972_d_n10;
        locals.var_t1_dn11 = assign61640_e95972_d_n11;
        locals.var_t1_dn14 = assign61640_e95972_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign61650_e95983, assign61650_e95983_d_n0, assign61650_e95983_d_n2, assign61650_e95983_d_n4, assign61650_e95983_d_n5, assign61650_e95983_d_n6, assign61650_e95983_d_n7, assign61650_e95983_d_n8, assign61650_e95983_d_n9, assign61650_e95983_d_n10, assign61650_e95983_d_n11, assign61650_e95983_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign61650_e95981: f64 = (locals.var_t1 * locals.var_cox);
        (assign61650_e95981, ((locals.var_t1_dn0 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn0)), ((locals.var_t1_dn2 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn2)), ((locals.var_t1_dn4 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn4)), ((locals.var_t1_dn5 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn5)), ((locals.var_t1_dn6 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn6)), ((locals.var_t1_dn7 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn7)), ((locals.var_t1_dn8 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn8)), ((locals.var_t1_dn9 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn9)), ((locals.var_t1_dn10 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn10)), ((locals.var_t1_dn11 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn11)), ((locals.var_t1_dn14 * locals.var_cox) + (locals.var_t1 * locals.var_cox_dn14)),)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn4, locals.var_qiu_dn5, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn8, locals.var_qiu_dn9, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn14,)
    }
};
        locals.var_qiu = assign61650_e95983;
        locals.var_qiu_dn0 = assign61650_e95983_d_n0;
        locals.var_qiu_dn2 = assign61650_e95983_d_n2;
        locals.var_qiu_dn4 = assign61650_e95983_d_n4;
        locals.var_qiu_dn5 = assign61650_e95983_d_n5;
        locals.var_qiu_dn6 = assign61650_e95983_d_n6;
        locals.var_qiu_dn7 = assign61650_e95983_d_n7;
        locals.var_qiu_dn8 = assign61650_e95983_d_n8;
        locals.var_qiu_dn9 = assign61650_e95983_d_n9;
        locals.var_qiu_dn10 = assign61650_e95983_d_n10;
        locals.var_qiu_dn11 = assign61650_e95983_d_n11;
        locals.var_qiu_dn14 = assign61650_e95983_d_n14;
        locals.var_qiu_rv = 0.0;

        let (assign61660_e95994, assign61660_e95994_d_n0, assign61660_e95994_d_n2, assign61660_e95994_d_n4, assign61660_e95994_d_n5, assign61660_e95994_d_n6, assign61660_e95994_d_n7, assign61660_e95994_d_n8, assign61660_e95994_d_n9, assign61660_e95994_d_n10, assign61660_e95994_d_n11, assign61660_e95994_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign61660_e95992: f64 = (0.5 + locals.var_alpha);
        (assign61660_e95992, locals.var_alpha_dn0, locals.var_alpha_dn2, locals.var_alpha_dn4, locals.var_alpha_dn5, locals.var_alpha_dn6, locals.var_alpha_dn7, locals.var_alpha_dn8, locals.var_alpha_dn9, locals.var_alpha_dn10, locals.var_alpha_dn11, locals.var_alpha_dn14,)
    } else {
        (locals.var_qdnm, locals.var_qdnm_dn0, locals.var_qdnm_dn2, locals.var_qdnm_dn4, locals.var_qdnm_dn5, locals.var_qdnm_dn6, locals.var_qdnm_dn7, locals.var_qdnm_dn8, locals.var_qdnm_dn9, locals.var_qdnm_dn10, locals.var_qdnm_dn11, locals.var_qdnm_dn14,)
    }
};
        locals.var_qdnm = assign61660_e95994;
        locals.var_qdnm_dn0 = assign61660_e95994_d_n0;
        locals.var_qdnm_dn2 = assign61660_e95994_d_n2;
        locals.var_qdnm_dn4 = assign61660_e95994_d_n4;
        locals.var_qdnm_dn5 = assign61660_e95994_d_n5;
        locals.var_qdnm_dn6 = assign61660_e95994_d_n6;
        locals.var_qdnm_dn7 = assign61660_e95994_d_n7;
        locals.var_qdnm_dn8 = assign61660_e95994_d_n8;
        locals.var_qdnm_dn9 = assign61660_e95994_d_n9;
        locals.var_qdnm_dn10 = assign61660_e95994_d_n10;
        locals.var_qdnm_dn11 = assign61660_e95994_d_n11;
        locals.var_qdnm_dn14 = assign61660_e95994_d_n14;
        locals.var_qdnm_rv = 0.0;

        let (assign61670_e96005, assign61670_e96005_d_n0, assign61670_e96005_d_n2, assign61670_e96005_d_n4, assign61670_e96005_d_n5, assign61670_e96005_d_n6, assign61670_e96005_d_n7, assign61670_e96005_d_n8, assign61670_e96005_d_n9, assign61670_e96005_d_n10, assign61670_e96005_d_n11, assign61670_e96005_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign61670_e96003: f64 = (locals.var_qidn * locals.var_qinm);
        (assign61670_e96003, ((locals.var_qidn_dn0 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn0)), ((locals.var_qidn_dn2 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn2)), ((locals.var_qidn_dn4 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn4)), ((locals.var_qidn_dn5 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn5)), ((locals.var_qidn_dn6 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn6)), ((locals.var_qidn_dn7 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn7)), ((locals.var_qidn_dn8 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn8)), ((locals.var_qidn_dn9 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn9)), ((locals.var_qidn_dn10 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn10)), ((locals.var_qidn_dn11 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn11)), ((locals.var_qidn_dn14 * locals.var_qinm) + (locals.var_qidn * locals.var_qinm_dn14)),)
    } else {
        (locals.var_qddn, locals.var_qddn_dn0, locals.var_qddn_dn2, locals.var_qddn_dn4, locals.var_qddn_dn5, locals.var_qddn_dn6, locals.var_qddn_dn7, locals.var_qddn_dn8, locals.var_qddn_dn9, locals.var_qddn_dn10, locals.var_qddn_dn11, locals.var_qddn_dn14,)
    }
};
        locals.var_qddn = assign61670_e96005;
        locals.var_qddn_dn0 = assign61670_e96005_d_n0;
        locals.var_qddn_dn2 = assign61670_e96005_d_n2;
        locals.var_qddn_dn4 = assign61670_e96005_d_n4;
        locals.var_qddn_dn5 = assign61670_e96005_d_n5;
        locals.var_qddn_dn6 = assign61670_e96005_d_n6;
        locals.var_qddn_dn7 = assign61670_e96005_d_n7;
        locals.var_qddn_dn8 = assign61670_e96005_d_n8;
        locals.var_qddn_dn9 = assign61670_e96005_d_n9;
        locals.var_qddn_dn10 = assign61670_e96005_d_n10;
        locals.var_qddn_dn11 = assign61670_e96005_d_n11;
        locals.var_qddn_dn14 = assign61670_e96005_d_n14;
        locals.var_qddn_rv = 0.0;

        let (assign61680_e96018, assign61680_e96018_d_n0, assign61680_e96018_d_n2, assign61680_e96018_d_n4, assign61680_e96018_d_n5, assign61680_e96018_d_n6, assign61680_e96018_d_n7, assign61680_e96018_d_n8, assign61680_e96018_d_n9, assign61680_e96018_d_n10, assign61680_e96018_d_n11, assign61680_e96018_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign61680_e96014: f64 = (0.4 * locals.var_qdnm);
        let assign61680_e96016: f64 = (assign61680_e96014 / locals.var_qddn);
        (assign61680_e96016, ((((0.4 * locals.var_qdnm_dn0) * locals.var_qddn) - (assign61680_e96014 * locals.var_qddn_dn0)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn2) * locals.var_qddn) - (assign61680_e96014 * locals.var_qddn_dn2)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn4) * locals.var_qddn) - (assign61680_e96014 * locals.var_qddn_dn4)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn5) * locals.var_qddn) - (assign61680_e96014 * locals.var_qddn_dn5)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn6) * locals.var_qddn) - (assign61680_e96014 * locals.var_qddn_dn6)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn7) * locals.var_qddn) - (assign61680_e96014 * locals.var_qddn_dn7)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn8) * locals.var_qddn) - (assign61680_e96014 * locals.var_qddn_dn8)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn9) * locals.var_qddn) - (assign61680_e96014 * locals.var_qddn_dn9)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn10) * locals.var_qddn) - (assign61680_e96014 * locals.var_qddn_dn10)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn11) * locals.var_qddn) - (assign61680_e96014 * locals.var_qddn_dn11)) / (locals.var_qddn * locals.var_qddn)), ((((0.4 * locals.var_qdnm_dn14) * locals.var_qddn) - (assign61680_e96014 * locals.var_qddn_dn14)) / (locals.var_qddn * locals.var_qddn)),)
    } else {
        (locals.var_quot, locals.var_quot_dn0, locals.var_quot_dn2, locals.var_quot_dn4, locals.var_quot_dn5, locals.var_quot_dn6, locals.var_quot_dn7, locals.var_quot_dn8, locals.var_quot_dn9, locals.var_quot_dn10, locals.var_quot_dn11, locals.var_quot_dn14,)
    }
};
        locals.var_quot = assign61680_e96018;
        locals.var_quot_dn0 = assign61680_e96018_d_n0;
        locals.var_quot_dn2 = assign61680_e96018_d_n2;
        locals.var_quot_dn4 = assign61680_e96018_d_n4;
        locals.var_quot_dn5 = assign61680_e96018_d_n5;
        locals.var_quot_dn6 = assign61680_e96018_d_n6;
        locals.var_quot_dn7 = assign61680_e96018_d_n7;
        locals.var_quot_dn8 = assign61680_e96018_d_n8;
        locals.var_quot_dn9 = assign61680_e96018_d_n9;
        locals.var_quot_dn10 = assign61680_e96018_d_n10;
        locals.var_quot_dn11 = assign61680_e96018_d_n11;
        locals.var_quot_dn14 = assign61680_e96018_d_n14;
        locals.var_quot_rv = 0.0;

        let (assign61690_e96029, assign61690_e96029_d_n0, assign61690_e96029_d_n2, assign61690_e96029_d_n4, assign61690_e96029_d_n5, assign61690_e96029_d_n6, assign61690_e96029_d_n7, assign61690_e96029_d_n8, assign61690_e96029_d_n9, assign61690_e96029_d_n10, assign61690_e96029_d_n11, assign61690_e96029_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) {
        let assign61690_e96027: f64 = (0.6 - locals.var_quot);
        (assign61690_e96027, (-locals.var_quot_dn0), (-locals.var_quot_dn2), (-locals.var_quot_dn4), (-locals.var_quot_dn5), (-locals.var_quot_dn6), (-locals.var_quot_dn7), (-locals.var_quot_dn8), (-locals.var_quot_dn9), (-locals.var_quot_dn10), (-locals.var_quot_dn11), (-locals.var_quot_dn14),)
    } else {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn4, locals.var_qdrat_dn5, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn8, locals.var_qdrat_dn9, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn14,)
    }
};
        locals.var_qdrat = assign61690_e96029;
        locals.var_qdrat_dn0 = assign61690_e96029_d_n0;
        locals.var_qdrat_dn2 = assign61690_e96029_d_n2;
        locals.var_qdrat_dn4 = assign61690_e96029_d_n4;
        locals.var_qdrat_dn5 = assign61690_e96029_d_n5;
        locals.var_qdrat_dn6 = assign61690_e96029_d_n6;
        locals.var_qdrat_dn7 = assign61690_e96029_d_n7;
        locals.var_qdrat_dn8 = assign61690_e96029_d_n8;
        locals.var_qdrat_dn9 = assign61690_e96029_d_n9;
        locals.var_qdrat_dn10 = assign61690_e96029_d_n10;
        locals.var_qdrat_dn11 = assign61690_e96029_d_n11;
        locals.var_qdrat_dn14 = assign61690_e96029_d_n14;
        locals.var_qdrat_rv = 0.0;

        let assign61700_e96032: f64 = if locals.var_qdrat > 0.5 { 1.0 } else { 0.0 };
        locals.var_guard1490 = assign61700_e96032;
        locals.var_guard1490_rv = 0.0;

        let (assign61710_e96043, assign61710_e96043_d_n0, assign61710_e96043_d_n2, assign61710_e96043_d_n4, assign61710_e96043_d_n5, assign61710_e96043_d_n6, assign61710_e96043_d_n7, assign61710_e96043_d_n8, assign61710_e96043_d_n9, assign61710_e96043_d_n10, assign61710_e96043_d_n11, assign61710_e96043_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1490 != 0.0)) {
        (0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn4, locals.var_qdrat_dn5, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn8, locals.var_qdrat_dn9, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn14,)
    }
};
        locals.var_qdrat = assign61710_e96043;
        locals.var_qdrat_dn0 = assign61710_e96043_d_n0;
        locals.var_qdrat_dn2 = assign61710_e96043_d_n2;
        locals.var_qdrat_dn4 = assign61710_e96043_d_n4;
        locals.var_qdrat_dn5 = assign61710_e96043_d_n5;
        locals.var_qdrat_dn6 = assign61710_e96043_d_n6;
        locals.var_qdrat_dn7 = assign61710_e96043_d_n7;
        locals.var_qdrat_dn8 = assign61710_e96043_d_n8;
        locals.var_qdrat_dn9 = assign61710_e96043_d_n9;
        locals.var_qdrat_dn10 = assign61710_e96043_d_n10;
        locals.var_qdrat_dn11 = assign61710_e96043_d_n11;
        locals.var_qdrat_dn14 = assign61710_e96043_d_n14;
        locals.var_qdrat_rv = 0.0;

        let assign61720_e96046: f64 = if locals.var_flg_zone == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1491 = assign61720_e96046;
        locals.var_guard1491_rv = 0.0;

        let (assign61730_e96057, assign61730_e96057_d_n0, assign61730_e96057_d_n2, assign61730_e96057_d_n4, assign61730_e96057_d_n5, assign61730_e96057_d_n6, assign61730_e96057_d_n7, assign61730_e96057_d_n8, assign61730_e96057_d_n9, assign61730_e96057_d_n10, assign61730_e96057_d_n11, assign61730_e96057_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1491 != 0.0)) {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn4, locals.var_qbu_dn5, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn8, locals.var_qbu_dn9, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign61730_e96057;
        locals.var_t1_dn0 = assign61730_e96057_d_n0;
        locals.var_t1_dn2 = assign61730_e96057_d_n2;
        locals.var_t1_dn4 = assign61730_e96057_d_n4;
        locals.var_t1_dn5 = assign61730_e96057_d_n5;
        locals.var_t1_dn6 = assign61730_e96057_d_n6;
        locals.var_t1_dn7 = assign61730_e96057_d_n7;
        locals.var_t1_dn8 = assign61730_e96057_d_n8;
        locals.var_t1_dn9 = assign61730_e96057_d_n9;
        locals.var_t1_dn10 = assign61730_e96057_d_n10;
        locals.var_t1_dn11 = assign61730_e96057_d_n11;
        locals.var_t1_dn14 = assign61730_e96057_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign61740_e96076, assign61740_e96076_d_n0, assign61740_e96076_d_n2, assign61740_e96076_d_n4, assign61740_e96076_d_n5, assign61740_e96076_d_n6, assign61740_e96076_d_n7, assign61740_e96076_d_n8, assign61740_e96076_d_n9, assign61740_e96076_d_n10, assign61740_e96076_d_n11, assign61740_e96076_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1491 != 0.0)) {
        let assign61740_e96068: f64 = (locals.var_fd2 * locals.var_qbu);
        let assign61740_e96071: f64 = (1.0 - locals.var_fd2);
        let assign61740_e96073: f64 = (assign61740_e96071 * locals.var_qb0);
        let assign61740_e96074: f64 = (assign61740_e96068 + assign61740_e96073);
        (assign61740_e96074, (((locals.var_fd2_dn0 * locals.var_qbu) + (locals.var_fd2 * locals.var_qbu_dn0)) + (((-locals.var_fd2_dn0) * locals.var_qb0) + (assign61740_e96071 * locals.var_qb0_dn0))), (((locals.var_fd2_dn2 * locals.var_qbu) + (locals.var_fd2 * locals.var_qbu_dn2)) + (((-locals.var_fd2_dn2) * locals.var_qb0) + (assign61740_e96071 * locals.var_qb0_dn2))), (((locals.var_fd2_dn4 * locals.var_qbu) + (locals.var_fd2 * locals.var_qbu_dn4)) + (((-locals.var_fd2_dn4) * locals.var_qb0) + (assign61740_e96071 * locals.var_qb0_dn4))), (((locals.var_fd2_dn5 * locals.var_qbu) + (locals.var_fd2 * locals.var_qbu_dn5)) + (((-locals.var_fd2_dn5) * locals.var_qb0) + (assign61740_e96071 * locals.var_qb0_dn5))), (((locals.var_fd2_dn6 * locals.var_qbu) + (locals.var_fd2 * locals.var_qbu_dn6)) + (((-locals.var_fd2_dn6) * locals.var_qb0) + (assign61740_e96071 * locals.var_qb0_dn6))), (((locals.var_fd2_dn7 * locals.var_qbu) + (locals.var_fd2 * locals.var_qbu_dn7)) + (((-locals.var_fd2_dn7) * locals.var_qb0) + (assign61740_e96071 * locals.var_qb0_dn7))), (((locals.var_fd2_dn8 * locals.var_qbu) + (locals.var_fd2 * locals.var_qbu_dn8)) + (((-locals.var_fd2_dn8) * locals.var_qb0) + (assign61740_e96071 * locals.var_qb0_dn8))), (((locals.var_fd2_dn9 * locals.var_qbu) + (locals.var_fd2 * locals.var_qbu_dn9)) + (((-locals.var_fd2_dn9) * locals.var_qb0) + (assign61740_e96071 * locals.var_qb0_dn9))), (((locals.var_fd2_dn10 * locals.var_qbu) + (locals.var_fd2 * locals.var_qbu_dn10)) + (((-locals.var_fd2_dn10) * locals.var_qb0) + (assign61740_e96071 * locals.var_qb0_dn10))), (((locals.var_fd2_dn11 * locals.var_qbu) + (locals.var_fd2 * locals.var_qbu_dn11)) + (((-locals.var_fd2_dn11) * locals.var_qb0) + (assign61740_e96071 * locals.var_qb0_dn11))), (((locals.var_fd2_dn14 * locals.var_qbu) + (locals.var_fd2 * locals.var_qbu_dn14)) + (((-locals.var_fd2_dn14) * locals.var_qb0) + (assign61740_e96071 * locals.var_qb0_dn14))),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn4, locals.var_qbu_dn5, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn8, locals.var_qbu_dn9, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn14,)
    }
};
        locals.var_qbu = assign61740_e96076;
        locals.var_qbu_dn0 = assign61740_e96076_d_n0;
        locals.var_qbu_dn2 = assign61740_e96076_d_n2;
        locals.var_qbu_dn4 = assign61740_e96076_d_n4;
        locals.var_qbu_dn5 = assign61740_e96076_d_n5;
        locals.var_qbu_dn6 = assign61740_e96076_d_n6;
        locals.var_qbu_dn7 = assign61740_e96076_d_n7;
        locals.var_qbu_dn8 = assign61740_e96076_d_n8;
        locals.var_qbu_dn9 = assign61740_e96076_d_n9;
        locals.var_qbu_dn10 = assign61740_e96076_d_n10;
        locals.var_qbu_dn11 = assign61740_e96076_d_n11;
        locals.var_qbu_dn14 = assign61740_e96076_d_n14;
        locals.var_qbu_rv = 0.0;

        let assign61750_e96079: f64 = if locals.var_qbu < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1492 = assign61750_e96079;
        locals.var_guard1492_rv = 0.0;

        let (assign61760_e96092, assign61760_e96092_d_n0, assign61760_e96092_d_n2, assign61760_e96092_d_n4, assign61760_e96092_d_n5, assign61760_e96092_d_n6, assign61760_e96092_d_n7, assign61760_e96092_d_n8, assign61760_e96092_d_n9, assign61760_e96092_d_n10, assign61760_e96092_d_n11, assign61760_e96092_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1492 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn4, locals.var_qbu_dn5, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn8, locals.var_qbu_dn9, locals.var_qbu_dn10, locals.var_qbu_dn11, locals.var_qbu_dn14,)
    }
};
        locals.var_qbu = assign61760_e96092;
        locals.var_qbu_dn0 = assign61760_e96092_d_n0;
        locals.var_qbu_dn2 = assign61760_e96092_d_n2;
        locals.var_qbu_dn4 = assign61760_e96092_d_n4;
        locals.var_qbu_dn5 = assign61760_e96092_d_n5;
        locals.var_qbu_dn6 = assign61760_e96092_d_n6;
        locals.var_qbu_dn7 = assign61760_e96092_d_n7;
        locals.var_qbu_dn8 = assign61760_e96092_d_n8;
        locals.var_qbu_dn9 = assign61760_e96092_d_n9;
        locals.var_qbu_dn10 = assign61760_e96092_d_n10;
        locals.var_qbu_dn11 = assign61760_e96092_d_n11;
        locals.var_qbu_dn14 = assign61760_e96092_d_n14;
        locals.var_qbu_rv = 0.0;

        let (assign61770_e96103, assign61770_e96103_d_n0, assign61770_e96103_d_n2, assign61770_e96103_d_n4, assign61770_e96103_d_n5, assign61770_e96103_d_n6, assign61770_e96103_d_n7, assign61770_e96103_d_n8, assign61770_e96103_d_n9, assign61770_e96103_d_n10, assign61770_e96103_d_n11, assign61770_e96103_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1491 != 0.0)) {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn4, locals.var_qiu_dn5, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn8, locals.var_qiu_dn9, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign61770_e96103;
        locals.var_t1_dn0 = assign61770_e96103_d_n0;
        locals.var_t1_dn2 = assign61770_e96103_d_n2;
        locals.var_t1_dn4 = assign61770_e96103_d_n4;
        locals.var_t1_dn5 = assign61770_e96103_d_n5;
        locals.var_t1_dn6 = assign61770_e96103_d_n6;
        locals.var_t1_dn7 = assign61770_e96103_d_n7;
        locals.var_t1_dn8 = assign61770_e96103_d_n8;
        locals.var_t1_dn9 = assign61770_e96103_d_n9;
        locals.var_t1_dn10 = assign61770_e96103_d_n10;
        locals.var_t1_dn11 = assign61770_e96103_d_n11;
        locals.var_t1_dn14 = assign61770_e96103_d_n14;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_230(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign61780_e96122, assign61780_e96122_d_n0, assign61780_e96122_d_n2, assign61780_e96122_d_n4, assign61780_e96122_d_n5, assign61780_e96122_d_n6, assign61780_e96122_d_n7, assign61780_e96122_d_n8, assign61780_e96122_d_n9, assign61780_e96122_d_n10, assign61780_e96122_d_n11, assign61780_e96122_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1491 != 0.0)) {
        let assign61780_e96114: f64 = (locals.var_fd2 * locals.var_qiu);
        let assign61780_e96117: f64 = (1.0 - locals.var_fd2);
        let assign61780_e96119: f64 = (assign61780_e96117 * locals.var_qn0);
        let assign61780_e96120: f64 = (assign61780_e96114 + assign61780_e96119);
        (assign61780_e96120, (((locals.var_fd2_dn0 * locals.var_qiu) + (locals.var_fd2 * locals.var_qiu_dn0)) + (((-locals.var_fd2_dn0) * locals.var_qn0) + (assign61780_e96117 * locals.var_qn0_dn0))), (((locals.var_fd2_dn2 * locals.var_qiu) + (locals.var_fd2 * locals.var_qiu_dn2)) + (((-locals.var_fd2_dn2) * locals.var_qn0) + (assign61780_e96117 * locals.var_qn0_dn2))), (((locals.var_fd2_dn4 * locals.var_qiu) + (locals.var_fd2 * locals.var_qiu_dn4)) + (((-locals.var_fd2_dn4) * locals.var_qn0) + (assign61780_e96117 * locals.var_qn0_dn4))), (((locals.var_fd2_dn5 * locals.var_qiu) + (locals.var_fd2 * locals.var_qiu_dn5)) + (((-locals.var_fd2_dn5) * locals.var_qn0) + (assign61780_e96117 * locals.var_qn0_dn5))), (((locals.var_fd2_dn6 * locals.var_qiu) + (locals.var_fd2 * locals.var_qiu_dn6)) + (((-locals.var_fd2_dn6) * locals.var_qn0) + (assign61780_e96117 * locals.var_qn0_dn6))), (((locals.var_fd2_dn7 * locals.var_qiu) + (locals.var_fd2 * locals.var_qiu_dn7)) + (((-locals.var_fd2_dn7) * locals.var_qn0) + (assign61780_e96117 * locals.var_qn0_dn7))), (((locals.var_fd2_dn8 * locals.var_qiu) + (locals.var_fd2 * locals.var_qiu_dn8)) + (((-locals.var_fd2_dn8) * locals.var_qn0) + (assign61780_e96117 * locals.var_qn0_dn8))), (((locals.var_fd2_dn9 * locals.var_qiu) + (locals.var_fd2 * locals.var_qiu_dn9)) + (((-locals.var_fd2_dn9) * locals.var_qn0) + (assign61780_e96117 * locals.var_qn0_dn9))), (((locals.var_fd2_dn10 * locals.var_qiu) + (locals.var_fd2 * locals.var_qiu_dn10)) + (((-locals.var_fd2_dn10) * locals.var_qn0) + (assign61780_e96117 * locals.var_qn0_dn10))), (((locals.var_fd2_dn11 * locals.var_qiu) + (locals.var_fd2 * locals.var_qiu_dn11)) + (((-locals.var_fd2_dn11) * locals.var_qn0) + (assign61780_e96117 * locals.var_qn0_dn11))), (((locals.var_fd2_dn14 * locals.var_qiu) + (locals.var_fd2 * locals.var_qiu_dn14)) + (((-locals.var_fd2_dn14) * locals.var_qn0) + (assign61780_e96117 * locals.var_qn0_dn14))),)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn4, locals.var_qiu_dn5, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn8, locals.var_qiu_dn9, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn14,)
    }
};
        locals.var_qiu = assign61780_e96122;
        locals.var_qiu_dn0 = assign61780_e96122_d_n0;
        locals.var_qiu_dn2 = assign61780_e96122_d_n2;
        locals.var_qiu_dn4 = assign61780_e96122_d_n4;
        locals.var_qiu_dn5 = assign61780_e96122_d_n5;
        locals.var_qiu_dn6 = assign61780_e96122_d_n6;
        locals.var_qiu_dn7 = assign61780_e96122_d_n7;
        locals.var_qiu_dn8 = assign61780_e96122_d_n8;
        locals.var_qiu_dn9 = assign61780_e96122_d_n9;
        locals.var_qiu_dn10 = assign61780_e96122_d_n10;
        locals.var_qiu_dn11 = assign61780_e96122_d_n11;
        locals.var_qiu_dn14 = assign61780_e96122_d_n14;
        locals.var_qiu_rv = 0.0;

        let assign61790_e96125: f64 = if locals.var_qiu < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1493 = assign61790_e96125;
        locals.var_guard1493_rv = 0.0;

        let (assign61800_e96138, assign61800_e96138_d_n0, assign61800_e96138_d_n2, assign61800_e96138_d_n4, assign61800_e96138_d_n5, assign61800_e96138_d_n6, assign61800_e96138_d_n7, assign61800_e96138_d_n8, assign61800_e96138_d_n9, assign61800_e96138_d_n10, assign61800_e96138_d_n11, assign61800_e96138_d_n14,) = {
    if (((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1491 != 0.0)) && (locals.var_guard1493 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn4, locals.var_qiu_dn5, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn8, locals.var_qiu_dn9, locals.var_qiu_dn10, locals.var_qiu_dn11, locals.var_qiu_dn14,)
    }
};
        locals.var_qiu = assign61800_e96138;
        locals.var_qiu_dn0 = assign61800_e96138_d_n0;
        locals.var_qiu_dn2 = assign61800_e96138_d_n2;
        locals.var_qiu_dn4 = assign61800_e96138_d_n4;
        locals.var_qiu_dn5 = assign61800_e96138_d_n5;
        locals.var_qiu_dn6 = assign61800_e96138_d_n6;
        locals.var_qiu_dn7 = assign61800_e96138_d_n7;
        locals.var_qiu_dn8 = assign61800_e96138_d_n8;
        locals.var_qiu_dn9 = assign61800_e96138_d_n9;
        locals.var_qiu_dn10 = assign61800_e96138_d_n10;
        locals.var_qiu_dn11 = assign61800_e96138_d_n11;
        locals.var_qiu_dn14 = assign61800_e96138_d_n14;
        locals.var_qiu_rv = 0.0;

        let (assign61810_e96149, assign61810_e96149_d_n0, assign61810_e96149_d_n2, assign61810_e96149_d_n4, assign61810_e96149_d_n5, assign61810_e96149_d_n6, assign61810_e96149_d_n7, assign61810_e96149_d_n8, assign61810_e96149_d_n9, assign61810_e96149_d_n10, assign61810_e96149_d_n11, assign61810_e96149_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1491 != 0.0)) {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn4, locals.var_qdrat_dn5, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn8, locals.var_qdrat_dn9, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign61810_e96149;
        locals.var_t1_dn0 = assign61810_e96149_d_n0;
        locals.var_t1_dn2 = assign61810_e96149_d_n2;
        locals.var_t1_dn4 = assign61810_e96149_d_n4;
        locals.var_t1_dn5 = assign61810_e96149_d_n5;
        locals.var_t1_dn6 = assign61810_e96149_d_n6;
        locals.var_t1_dn7 = assign61810_e96149_d_n7;
        locals.var_t1_dn8 = assign61810_e96149_d_n8;
        locals.var_t1_dn9 = assign61810_e96149_d_n9;
        locals.var_t1_dn10 = assign61810_e96149_d_n10;
        locals.var_t1_dn11 = assign61810_e96149_d_n11;
        locals.var_t1_dn14 = assign61810_e96149_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign61820_e96168, assign61820_e96168_d_n0, assign61820_e96168_d_n2, assign61820_e96168_d_n4, assign61820_e96168_d_n5, assign61820_e96168_d_n6, assign61820_e96168_d_n7, assign61820_e96168_d_n8, assign61820_e96168_d_n9, assign61820_e96168_d_n10, assign61820_e96168_d_n11, assign61820_e96168_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1491 != 0.0)) {
        let assign61820_e96160: f64 = (locals.var_fd2 * locals.var_qdrat);
        let assign61820_e96163: f64 = (1.0 - locals.var_fd2);
        let assign61820_e96165: f64 = (assign61820_e96163 * 0.5);
        let assign61820_e96166: f64 = (assign61820_e96160 + assign61820_e96165);
        (assign61820_e96166, (((locals.var_fd2_dn0 * locals.var_qdrat) + (locals.var_fd2 * locals.var_qdrat_dn0)) + ((-locals.var_fd2_dn0) * 0.5)), (((locals.var_fd2_dn2 * locals.var_qdrat) + (locals.var_fd2 * locals.var_qdrat_dn2)) + ((-locals.var_fd2_dn2) * 0.5)), (((locals.var_fd2_dn4 * locals.var_qdrat) + (locals.var_fd2 * locals.var_qdrat_dn4)) + ((-locals.var_fd2_dn4) * 0.5)), (((locals.var_fd2_dn5 * locals.var_qdrat) + (locals.var_fd2 * locals.var_qdrat_dn5)) + ((-locals.var_fd2_dn5) * 0.5)), (((locals.var_fd2_dn6 * locals.var_qdrat) + (locals.var_fd2 * locals.var_qdrat_dn6)) + ((-locals.var_fd2_dn6) * 0.5)), (((locals.var_fd2_dn7 * locals.var_qdrat) + (locals.var_fd2 * locals.var_qdrat_dn7)) + ((-locals.var_fd2_dn7) * 0.5)), (((locals.var_fd2_dn8 * locals.var_qdrat) + (locals.var_fd2 * locals.var_qdrat_dn8)) + ((-locals.var_fd2_dn8) * 0.5)), (((locals.var_fd2_dn9 * locals.var_qdrat) + (locals.var_fd2 * locals.var_qdrat_dn9)) + ((-locals.var_fd2_dn9) * 0.5)), (((locals.var_fd2_dn10 * locals.var_qdrat) + (locals.var_fd2 * locals.var_qdrat_dn10)) + ((-locals.var_fd2_dn10) * 0.5)), (((locals.var_fd2_dn11 * locals.var_qdrat) + (locals.var_fd2 * locals.var_qdrat_dn11)) + ((-locals.var_fd2_dn11) * 0.5)), (((locals.var_fd2_dn14 * locals.var_qdrat) + (locals.var_fd2 * locals.var_qdrat_dn14)) + ((-locals.var_fd2_dn14) * 0.5)),)
    } else {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn4, locals.var_qdrat_dn5, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn8, locals.var_qdrat_dn9, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn14,)
    }
};
        locals.var_qdrat = assign61820_e96168;
        locals.var_qdrat_dn0 = assign61820_e96168_d_n0;
        locals.var_qdrat_dn2 = assign61820_e96168_d_n2;
        locals.var_qdrat_dn4 = assign61820_e96168_d_n4;
        locals.var_qdrat_dn5 = assign61820_e96168_d_n5;
        locals.var_qdrat_dn6 = assign61820_e96168_d_n6;
        locals.var_qdrat_dn7 = assign61820_e96168_d_n7;
        locals.var_qdrat_dn8 = assign61820_e96168_d_n8;
        locals.var_qdrat_dn9 = assign61820_e96168_d_n9;
        locals.var_qdrat_dn10 = assign61820_e96168_d_n10;
        locals.var_qdrat_dn11 = assign61820_e96168_d_n11;
        locals.var_qdrat_dn14 = assign61820_e96168_d_n14;
        locals.var_qdrat_rv = 0.0;

        let (assign61830_e96179, assign61830_e96179_d_n0, assign61830_e96179_d_n2, assign61830_e96179_d_n4, assign61830_e96179_d_n5, assign61830_e96179_d_n6, assign61830_e96179_d_n7, assign61830_e96179_d_n8, assign61830_e96179_d_n9, assign61830_e96179_d_n10, assign61830_e96179_d_n11, assign61830_e96179_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1491 != 0.0)) {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn4, locals.var_lred_dn5, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn8, locals.var_lred_dn9, locals.var_lred_dn10, locals.var_lred_dn11, locals.var_lred_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign61830_e96179;
        locals.var_t1_dn0 = assign61830_e96179_d_n0;
        locals.var_t1_dn2 = assign61830_e96179_d_n2;
        locals.var_t1_dn4 = assign61830_e96179_d_n4;
        locals.var_t1_dn5 = assign61830_e96179_d_n5;
        locals.var_t1_dn6 = assign61830_e96179_d_n6;
        locals.var_t1_dn7 = assign61830_e96179_d_n7;
        locals.var_t1_dn8 = assign61830_e96179_d_n8;
        locals.var_t1_dn9 = assign61830_e96179_d_n9;
        locals.var_t1_dn10 = assign61830_e96179_d_n10;
        locals.var_t1_dn11 = assign61830_e96179_d_n11;
        locals.var_t1_dn14 = assign61830_e96179_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign61840_e96192, assign61840_e96192_d_n0, assign61840_e96192_d_n2, assign61840_e96192_d_n4, assign61840_e96192_d_n5, assign61840_e96192_d_n6, assign61840_e96192_d_n7, assign61840_e96192_d_n8, assign61840_e96192_d_n9, assign61840_e96192_d_n10, assign61840_e96192_d_n11, assign61840_e96192_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1465 != 0.0)) && (locals.var_guard1491 != 0.0)) {
        let assign61840_e96190: f64 = (locals.var_fd2 * locals.var_lred);
        (assign61840_e96190, ((locals.var_fd2_dn0 * locals.var_lred) + (locals.var_fd2 * locals.var_lred_dn0)), ((locals.var_fd2_dn2 * locals.var_lred) + (locals.var_fd2 * locals.var_lred_dn2)), ((locals.var_fd2_dn4 * locals.var_lred) + (locals.var_fd2 * locals.var_lred_dn4)), ((locals.var_fd2_dn5 * locals.var_lred) + (locals.var_fd2 * locals.var_lred_dn5)), ((locals.var_fd2_dn6 * locals.var_lred) + (locals.var_fd2 * locals.var_lred_dn6)), ((locals.var_fd2_dn7 * locals.var_lred) + (locals.var_fd2 * locals.var_lred_dn7)), ((locals.var_fd2_dn8 * locals.var_lred) + (locals.var_fd2 * locals.var_lred_dn8)), ((locals.var_fd2_dn9 * locals.var_lred) + (locals.var_fd2 * locals.var_lred_dn9)), ((locals.var_fd2_dn10 * locals.var_lred) + (locals.var_fd2 * locals.var_lred_dn10)), ((locals.var_fd2_dn11 * locals.var_lred) + (locals.var_fd2 * locals.var_lred_dn11)), ((locals.var_fd2_dn14 * locals.var_lred) + (locals.var_fd2 * locals.var_lred_dn14)),)
    } else {
        (locals.var_lred, locals.var_lred_dn0, locals.var_lred_dn2, locals.var_lred_dn4, locals.var_lred_dn5, locals.var_lred_dn6, locals.var_lred_dn7, locals.var_lred_dn8, locals.var_lred_dn9, locals.var_lred_dn10, locals.var_lred_dn11, locals.var_lred_dn14,)
    }
};
        locals.var_lred = assign61840_e96192;
        locals.var_lred_dn0 = assign61840_e96192_d_n0;
        locals.var_lred_dn2 = assign61840_e96192_d_n2;
        locals.var_lred_dn4 = assign61840_e96192_d_n4;
        locals.var_lred_dn5 = assign61840_e96192_d_n5;
        locals.var_lred_dn6 = assign61840_e96192_d_n6;
        locals.var_lred_dn7 = assign61840_e96192_d_n7;
        locals.var_lred_dn8 = assign61840_e96192_d_n8;
        locals.var_lred_dn9 = assign61840_e96192_d_n9;
        locals.var_lred_dn10 = assign61840_e96192_d_n10;
        locals.var_lred_dn11 = assign61840_e96192_d_n11;
        locals.var_lred_dn14 = assign61840_e96192_d_n14;
        locals.var_lred_rv = 0.0;

        let (assign61850_e96201,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_start_of_mobility != 0.0)) {
        (0.0,)
    } else {
        (locals.var_start_of_mobility,)
    }
};
        locals.var_start_of_mobility = assign61850_e96201;
        locals.var_start_of_mobility_rv = 0.0;

        let (assign61860_e96210, assign61860_e96210_d_n0, assign61860_e96210_d_n2, assign61860_e96210_d_n4, assign61860_e96210_d_n5, assign61860_e96210_d_n6, assign61860_e96210_d_n7, assign61860_e96210_d_n8, assign61860_e96210_d_n9, assign61860_e96210_d_n10, assign61860_e96210_d_n11, assign61860_e96210_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign61860_e96208: f64 = (locals.var_leff - locals.var_lred);
        (assign61860_e96208, (-locals.var_lred_dn0), (-locals.var_lred_dn2), (-locals.var_lred_dn4), (-locals.var_lred_dn5), (-locals.var_lred_dn6), (-locals.var_lred_dn7), (-locals.var_lred_dn8), (-locals.var_lred_dn9), (-locals.var_lred_dn10), (-locals.var_lred_dn11), (-locals.var_lred_dn14),)
    } else {
        (locals.var_lch, locals.var_lch_dn0, locals.var_lch_dn2, locals.var_lch_dn4, locals.var_lch_dn5, locals.var_lch_dn6, locals.var_lch_dn7, locals.var_lch_dn8, locals.var_lch_dn9, locals.var_lch_dn10, locals.var_lch_dn11, locals.var_lch_dn14,)
    }
};
        locals.var_lch = assign61860_e96210;
        locals.var_lch_dn0 = assign61860_e96210_d_n0;
        locals.var_lch_dn2 = assign61860_e96210_d_n2;
        locals.var_lch_dn4 = assign61860_e96210_d_n4;
        locals.var_lch_dn5 = assign61860_e96210_d_n5;
        locals.var_lch_dn6 = assign61860_e96210_d_n6;
        locals.var_lch_dn7 = assign61860_e96210_d_n7;
        locals.var_lch_dn8 = assign61860_e96210_d_n8;
        locals.var_lch_dn9 = assign61860_e96210_d_n9;
        locals.var_lch_dn10 = assign61860_e96210_d_n10;
        locals.var_lch_dn11 = assign61860_e96210_d_n11;
        locals.var_lch_dn14 = assign61860_e96210_d_n14;
        locals.var_lch_rv = 0.0;

        let assign61870_e96213: f64 = if locals.var_lch < 1e-9 { 1.0 } else { 0.0 };
        locals.var_guard1494 = assign61870_e96213;
        locals.var_guard1494_rv = 0.0;

        let (assign61880_e96222, assign61880_e96222_d_n0, assign61880_e96222_d_n2, assign61880_e96222_d_n4, assign61880_e96222_d_n5, assign61880_e96222_d_n6, assign61880_e96222_d_n7, assign61880_e96222_d_n8, assign61880_e96222_d_n9, assign61880_e96222_d_n10, assign61880_e96222_d_n11, assign61880_e96222_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1494 != 0.0)) {
        (1e-9, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lch, locals.var_lch_dn0, locals.var_lch_dn2, locals.var_lch_dn4, locals.var_lch_dn5, locals.var_lch_dn6, locals.var_lch_dn7, locals.var_lch_dn8, locals.var_lch_dn9, locals.var_lch_dn10, locals.var_lch_dn11, locals.var_lch_dn14,)
    }
};
        locals.var_lch = assign61880_e96222;
        locals.var_lch_dn0 = assign61880_e96222_d_n0;
        locals.var_lch_dn2 = assign61880_e96222_d_n2;
        locals.var_lch_dn4 = assign61880_e96222_d_n4;
        locals.var_lch_dn5 = assign61880_e96222_d_n5;
        locals.var_lch_dn6 = assign61880_e96222_d_n6;
        locals.var_lch_dn7 = assign61880_e96222_d_n7;
        locals.var_lch_dn8 = assign61880_e96222_d_n8;
        locals.var_lch_dn9 = assign61880_e96222_d_n9;
        locals.var_lch_dn10 = assign61880_e96222_d_n10;
        locals.var_lch_dn11 = assign61880_e96222_d_n11;
        locals.var_lch_dn14 = assign61880_e96222_d_n14;
        locals.var_lch_rv = 0.0;

        let (assign61890_e96231, assign61890_e96231_d_n0, assign61890_e96231_d_n2, assign61890_e96231_d_n4, assign61890_e96231_d_n5, assign61890_e96231_d_n6, assign61890_e96231_d_n7, assign61890_e96231_d_n8, assign61890_e96231_d_n9, assign61890_e96231_d_n10, assign61890_e96231_d_n11, assign61890_e96231_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign61890_e96229: f64 = (locals.var_ndep_o_esi / 100.0);
        (assign61890_e96229, (locals.var_ndep_o_esi_dn0 / 100.0), (locals.var_ndep_o_esi_dn2 / 100.0), (locals.var_ndep_o_esi_dn4 / 100.0), (locals.var_ndep_o_esi_dn5 / 100.0), (locals.var_ndep_o_esi_dn6 / 100.0), (locals.var_ndep_o_esi_dn7 / 100.0), (locals.var_ndep_o_esi_dn8 / 100.0), (locals.var_ndep_o_esi_dn9 / 100.0), (locals.var_ndep_o_esi_dn10 / 100.0), (locals.var_ndep_o_esi_dn11 / 100.0), (locals.var_ndep_o_esi_dn14 / 100.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign61890_e96231;
        locals.var_t1_dn0 = assign61890_e96231_d_n0;
        locals.var_t1_dn2 = assign61890_e96231_d_n2;
        locals.var_t1_dn4 = assign61890_e96231_d_n4;
        locals.var_t1_dn5 = assign61890_e96231_d_n5;
        locals.var_t1_dn6 = assign61890_e96231_d_n6;
        locals.var_t1_dn7 = assign61890_e96231_d_n7;
        locals.var_t1_dn8 = assign61890_e96231_d_n8;
        locals.var_t1_dn9 = assign61890_e96231_d_n9;
        locals.var_t1_dn10 = assign61890_e96231_d_n10;
        locals.var_t1_dn11 = assign61890_e96231_d_n11;
        locals.var_t1_dn14 = assign61890_e96231_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign61900_e96240, assign61900_e96240_d_n0, assign61900_e96240_d_n2, assign61900_e96240_d_n4, assign61900_e96240_d_n5, assign61900_e96240_d_n6, assign61900_e96240_d_n7, assign61900_e96240_d_n8, assign61900_e96240_d_n9, assign61900_e96240_d_n10, assign61900_e96240_d_n11, assign61900_e96240_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign61900_e96238: f64 = (locals.var_ninv_o_esi / 100.0);
        (assign61900_e96238, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign61900_e96240;
        locals.var_t2_dn0 = assign61900_e96240_d_n0;
        locals.var_t2_dn2 = assign61900_e96240_d_n2;
        locals.var_t2_dn4 = assign61900_e96240_d_n4;
        locals.var_t2_dn5 = assign61900_e96240_d_n5;
        locals.var_t2_dn6 = assign61900_e96240_d_n6;
        locals.var_t2_dn7 = assign61900_e96240_d_n7;
        locals.var_t2_dn8 = assign61900_e96240_d_n8;
        locals.var_t2_dn9 = assign61900_e96240_d_n9;
        locals.var_t2_dn10 = assign61900_e96240_d_n10;
        locals.var_t2_dn11 = assign61900_e96240_d_n11;
        locals.var_t2_dn14 = assign61900_e96240_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign61910_e96247, assign61910_e96247_d_n0, assign61910_e96247_d_n2, assign61910_e96247_d_n4, assign61910_e96247_d_n5, assign61910_e96247_d_n6, assign61910_e96247_d_n7, assign61910_e96247_d_n8, assign61910_e96247_d_n9, assign61910_e96247_d_n10, assign61910_e96247_d_n11, assign61910_e96247_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        (locals.var_ninvde, locals.var_ninvde_dn0, locals.var_ninvde_dn2, locals.var_ninvde_dn4, locals.var_ninvde_dn5, locals.var_ninvde_dn6, locals.var_ninvde_dn7, locals.var_ninvde_dn8, locals.var_ninvde_dn9, locals.var_ninvde_dn10, locals.var_ninvde_dn11, locals.var_ninvde_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign61910_e96247;
        locals.var_t0_dn0 = assign61910_e96247_d_n0;
        locals.var_t0_dn2 = assign61910_e96247_d_n2;
        locals.var_t0_dn4 = assign61910_e96247_d_n4;
        locals.var_t0_dn5 = assign61910_e96247_d_n5;
        locals.var_t0_dn6 = assign61910_e96247_d_n6;
        locals.var_t0_dn7 = assign61910_e96247_d_n7;
        locals.var_t0_dn8 = assign61910_e96247_d_n8;
        locals.var_t0_dn9 = assign61910_e96247_d_n9;
        locals.var_t0_dn10 = assign61910_e96247_d_n10;
        locals.var_t0_dn11 = assign61910_e96247_d_n11;
        locals.var_t0_dn14 = assign61910_e96247_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign61920_e96260, assign61920_e96260_d_n0, assign61920_e96260_d_n2, assign61920_e96260_d_n4, assign61920_e96260_d_n5, assign61920_e96260_d_n6, assign61920_e96260_d_n7, assign61920_e96260_d_n8, assign61920_e96260_d_n9, assign61920_e96260_d_n10, assign61920_e96260_d_n11, assign61920_e96260_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign61920_e96255: f64 = (locals.var_psl - locals.var_ps0);
        let assign61920_e96257: f64 = (assign61920_e96255 * locals.var_t0);
        let assign61920_e96258: f64 = (1.0 + assign61920_e96257);
        (assign61920_e96258, (((locals.var_psl_dn0 - locals.var_ps0_dn0) * locals.var_t0) + (assign61920_e96255 * locals.var_t0_dn0)), (((locals.var_psl_dn2 - locals.var_ps0_dn2) * locals.var_t0) + (assign61920_e96255 * locals.var_t0_dn2)), (((locals.var_psl_dn4 - locals.var_ps0_dn4) * locals.var_t0) + (assign61920_e96255 * locals.var_t0_dn4)), (((locals.var_psl_dn5 - locals.var_ps0_dn5) * locals.var_t0) + (assign61920_e96255 * locals.var_t0_dn5)), (((locals.var_psl_dn6 - locals.var_ps0_dn6) * locals.var_t0) + (assign61920_e96255 * locals.var_t0_dn6)), (((locals.var_psl_dn7 - locals.var_ps0_dn7) * locals.var_t0) + (assign61920_e96255 * locals.var_t0_dn7)), (((locals.var_psl_dn8 - locals.var_ps0_dn8) * locals.var_t0) + (assign61920_e96255 * locals.var_t0_dn8)), (((locals.var_psl_dn9 - locals.var_ps0_dn9) * locals.var_t0) + (assign61920_e96255 * locals.var_t0_dn9)), (((locals.var_psl_dn10 - locals.var_ps0_dn10) * locals.var_t0) + (assign61920_e96255 * locals.var_t0_dn10)), (((locals.var_psl_dn11 - locals.var_ps0_dn11) * locals.var_t0) + (assign61920_e96255 * locals.var_t0_dn11)), (((locals.var_psl_dn14 - locals.var_ps0_dn14) * locals.var_t0) + (assign61920_e96255 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign61920_e96260;
        locals.var_t4_dn0 = assign61920_e96260_d_n0;
        locals.var_t4_dn2 = assign61920_e96260_d_n2;
        locals.var_t4_dn4 = assign61920_e96260_d_n4;
        locals.var_t4_dn5 = assign61920_e96260_d_n5;
        locals.var_t4_dn6 = assign61920_e96260_d_n6;
        locals.var_t4_dn7 = assign61920_e96260_d_n7;
        locals.var_t4_dn8 = assign61920_e96260_d_n8;
        locals.var_t4_dn9 = assign61920_e96260_d_n9;
        locals.var_t4_dn10 = assign61920_e96260_d_n10;
        locals.var_t4_dn11 = assign61920_e96260_d_n11;
        locals.var_t4_dn14 = assign61920_e96260_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign61930_e96273, assign61930_e96273_d_n0, assign61930_e96273_d_n2, assign61930_e96273_d_n4, assign61930_e96273_d_n5, assign61930_e96273_d_n6, assign61930_e96273_d_n7, assign61930_e96273_d_n8, assign61930_e96273_d_n9, assign61930_e96273_d_n10, assign61930_e96273_d_n11, assign61930_e96273_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign61930_e96267: f64 = (locals.var_t1 * locals.var_qbu);
        let assign61930_e96270: f64 = (locals.var_t2 * locals.var_qiu);
        let assign61930_e96271: f64 = (assign61930_e96267 + assign61930_e96270);
        (assign61930_e96271, (((locals.var_t1_dn0 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn0)) + ((locals.var_t2_dn0 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn0))), (((locals.var_t1_dn2 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn2)) + ((locals.var_t2_dn2 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn2))), (((locals.var_t1_dn4 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn4)) + ((locals.var_t2_dn4 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn4))), (((locals.var_t1_dn5 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn5)) + ((locals.var_t2_dn5 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn5))), (((locals.var_t1_dn6 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn6)) + ((locals.var_t2_dn6 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn6))), (((locals.var_t1_dn7 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn7)) + ((locals.var_t2_dn7 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn7))), (((locals.var_t1_dn8 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn8)) + ((locals.var_t2_dn8 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn8))), (((locals.var_t1_dn9 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn9)) + ((locals.var_t2_dn9 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn9))), (((locals.var_t1_dn10 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn10)) + ((locals.var_t2_dn10 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn10))), (((locals.var_t1_dn11 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn11)) + ((locals.var_t2_dn11 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn11))), (((locals.var_t1_dn14 * locals.var_qbu) + (locals.var_t1 * locals.var_qbu_dn14)) + ((locals.var_t2_dn14 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn14))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign61930_e96273;
        locals.var_t5_dn0 = assign61930_e96273_d_n0;
        locals.var_t5_dn2 = assign61930_e96273_d_n2;
        locals.var_t5_dn4 = assign61930_e96273_d_n4;
        locals.var_t5_dn5 = assign61930_e96273_d_n5;
        locals.var_t5_dn6 = assign61930_e96273_d_n6;
        locals.var_t5_dn7 = assign61930_e96273_d_n7;
        locals.var_t5_dn8 = assign61930_e96273_d_n8;
        locals.var_t5_dn9 = assign61930_e96273_d_n9;
        locals.var_t5_dn10 = assign61930_e96273_d_n10;
        locals.var_t5_dn11 = assign61930_e96273_d_n11;
        locals.var_t5_dn14 = assign61930_e96273_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign61940_e96282, assign61940_e96282_d_n0, assign61940_e96282_d_n2, assign61940_e96282_d_n4, assign61940_e96282_d_n5, assign61940_e96282_d_n6, assign61940_e96282_d_n7, assign61940_e96282_d_n8, assign61940_e96282_d_n9, assign61940_e96282_d_n10, assign61940_e96282_d_n11, assign61940_e96282_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign61940_e96280: f64 = (locals.var_t5 / locals.var_t4);
        (assign61940_e96280, (((locals.var_t5_dn0 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn2 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn4 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn5 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn6 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn7 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn8 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn9 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn10 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn11 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn14 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn14)) / (locals.var_t4 * locals.var_t4)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign61940_e96282;
        locals.var_t3_dn0 = assign61940_e96282_d_n0;
        locals.var_t3_dn2 = assign61940_e96282_d_n2;
        locals.var_t3_dn4 = assign61940_e96282_d_n4;
        locals.var_t3_dn5 = assign61940_e96282_d_n5;
        locals.var_t3_dn6 = assign61940_e96282_d_n6;
        locals.var_t3_dn7 = assign61940_e96282_d_n7;
        locals.var_t3_dn8 = assign61940_e96282_d_n8;
        locals.var_t3_dn9 = assign61940_e96282_d_n9;
        locals.var_t3_dn10 = assign61940_e96282_d_n10;
        locals.var_t3_dn11 = assign61940_e96282_d_n11;
        locals.var_t3_dn14 = assign61940_e96282_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign61950_e96295, assign61950_e96295_d_n0, assign61950_e96295_d_n2, assign61950_e96295_d_n4, assign61950_e96295_d_n5, assign61950_e96295_d_n6, assign61950_e96295_d_n7, assign61950_e96295_d_n8, assign61950_e96295_d_n9, assign61950_e96295_d_n10, assign61950_e96295_d_n11, assign61950_e96295_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign61950_e96291: f64 = (p.p166 * locals.var_vbsz__blk442);
        let assign61950_e96292: f64 = (1.0 + assign61950_e96291);
        let assign61950_e96293: f64 = (locals.var_t3 * assign61950_e96292);
        (assign61950_e96293, ((locals.var_t3_dn0 * assign61950_e96292) + (locals.var_t3 * (p.p166 * locals.var_vbsz__blk442_dn0))), ((locals.var_t3_dn2 * assign61950_e96292) + (locals.var_t3 * (p.p166 * locals.var_vbsz__blk442_dn2))), ((locals.var_t3_dn4 * assign61950_e96292) + (locals.var_t3 * (p.p166 * locals.var_vbsz__blk442_dn4))), ((locals.var_t3_dn5 * assign61950_e96292) + (locals.var_t3 * (p.p166 * locals.var_vbsz__blk442_dn5))), ((locals.var_t3_dn6 * assign61950_e96292) + (locals.var_t3 * (p.p166 * locals.var_vbsz__blk442_dn6))), ((locals.var_t3_dn7 * assign61950_e96292) + (locals.var_t3 * (p.p166 * locals.var_vbsz__blk442_dn7))), ((locals.var_t3_dn8 * assign61950_e96292) + (locals.var_t3 * (p.p166 * locals.var_vbsz__blk442_dn8))), ((locals.var_t3_dn9 * assign61950_e96292) + (locals.var_t3 * (p.p166 * locals.var_vbsz__blk442_dn9))), ((locals.var_t3_dn10 * assign61950_e96292) + (locals.var_t3 * (p.p166 * locals.var_vbsz__blk442_dn10))), ((locals.var_t3_dn11 * assign61950_e96292) + (locals.var_t3 * (p.p166 * locals.var_vbsz__blk442_dn11))), ((locals.var_t3_dn14 * assign61950_e96292) + (locals.var_t3 * (p.p166 * locals.var_vbsz__blk442_dn14))),)
    } else {
        (locals.var_eeff, locals.var_eeff_dn0, locals.var_eeff_dn2, locals.var_eeff_dn4, locals.var_eeff_dn5, locals.var_eeff_dn6, locals.var_eeff_dn7, locals.var_eeff_dn8, locals.var_eeff_dn9, locals.var_eeff_dn10, locals.var_eeff_dn11, locals.var_eeff_dn14,)
    }
};
        locals.var_eeff = assign61950_e96295;
        locals.var_eeff_dn0 = assign61950_e96295_d_n0;
        locals.var_eeff_dn2 = assign61950_e96295_d_n2;
        locals.var_eeff_dn4 = assign61950_e96295_d_n4;
        locals.var_eeff_dn5 = assign61950_e96295_d_n5;
        locals.var_eeff_dn6 = assign61950_e96295_d_n6;
        locals.var_eeff_dn7 = assign61950_e96295_d_n7;
        locals.var_eeff_dn8 = assign61950_e96295_d_n8;
        locals.var_eeff_dn9 = assign61950_e96295_d_n9;
        locals.var_eeff_dn10 = assign61950_e96295_d_n10;
        locals.var_eeff_dn11 = assign61950_e96295_d_n11;
        locals.var_eeff_dn14 = assign61950_e96295_d_n14;
        locals.var_eeff_rv = 0.0;

        let (assign61960_e96311, assign61960_e96311_d_n0, assign61960_e96311_d_n2, assign61960_e96311_d_n4, assign61960_e96311_d_n5, assign61960_e96311_d_n6, assign61960_e96311_d_n7, assign61960_e96311_d_n8, assign61960_e96311_d_n9, assign61960_e96311_d_n10, assign61960_e96311_d_n11, assign61960_e96311_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let (assign61960_e96309, assign61960_e96309_d_n0, assign61960_e96309_d_n2, assign61960_e96309_d_n4, assign61960_e96309_d_n5, assign61960_e96309_d_n6, assign61960_e96309_d_n7, assign61960_e96309_d_n8, assign61960_e96309_d_n9, assign61960_e96309_d_n10, assign61960_e96309_d_n11, assign61960_e96309_d_n14,) = {
            if (locals.var_eeff == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign61960_e96307: f64 = (p.p160 - 1.0);
                let assign61960_e96308: f64 = (locals.var_eeff).powf(assign61960_e96307);
                (assign61960_e96308, if 0.0 == 0.0 && ((assign61960_e96307) as f64).is_finite() && ((assign61960_e96307) as f64).fract() == 0.0 { if assign61960_e96307 == 0.0 { 0.0 } else { (assign61960_e96307 * ((locals.var_eeff).powf(assign61960_e96307 - 1.0) * locals.var_eeff_dn0)) } } else { (assign61960_e96308 * (assign61960_e96307 * (locals.var_eeff_dn0 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61960_e96307) as f64).is_finite() && ((assign61960_e96307) as f64).fract() == 0.0 { if assign61960_e96307 == 0.0 { 0.0 } else { (assign61960_e96307 * ((locals.var_eeff).powf(assign61960_e96307 - 1.0) * locals.var_eeff_dn2)) } } else { (assign61960_e96308 * (assign61960_e96307 * (locals.var_eeff_dn2 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61960_e96307) as f64).is_finite() && ((assign61960_e96307) as f64).fract() == 0.0 { if assign61960_e96307 == 0.0 { 0.0 } else { (assign61960_e96307 * ((locals.var_eeff).powf(assign61960_e96307 - 1.0) * locals.var_eeff_dn4)) } } else { (assign61960_e96308 * (assign61960_e96307 * (locals.var_eeff_dn4 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61960_e96307) as f64).is_finite() && ((assign61960_e96307) as f64).fract() == 0.0 { if assign61960_e96307 == 0.0 { 0.0 } else { (assign61960_e96307 * ((locals.var_eeff).powf(assign61960_e96307 - 1.0) * locals.var_eeff_dn5)) } } else { (assign61960_e96308 * (assign61960_e96307 * (locals.var_eeff_dn5 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61960_e96307) as f64).is_finite() && ((assign61960_e96307) as f64).fract() == 0.0 { if assign61960_e96307 == 0.0 { 0.0 } else { (assign61960_e96307 * ((locals.var_eeff).powf(assign61960_e96307 - 1.0) * locals.var_eeff_dn6)) } } else { (assign61960_e96308 * (assign61960_e96307 * (locals.var_eeff_dn6 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61960_e96307) as f64).is_finite() && ((assign61960_e96307) as f64).fract() == 0.0 { if assign61960_e96307 == 0.0 { 0.0 } else { (assign61960_e96307 * ((locals.var_eeff).powf(assign61960_e96307 - 1.0) * locals.var_eeff_dn7)) } } else { (assign61960_e96308 * (assign61960_e96307 * (locals.var_eeff_dn7 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61960_e96307) as f64).is_finite() && ((assign61960_e96307) as f64).fract() == 0.0 { if assign61960_e96307 == 0.0 { 0.0 } else { (assign61960_e96307 * ((locals.var_eeff).powf(assign61960_e96307 - 1.0) * locals.var_eeff_dn8)) } } else { (assign61960_e96308 * (assign61960_e96307 * (locals.var_eeff_dn8 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61960_e96307) as f64).is_finite() && ((assign61960_e96307) as f64).fract() == 0.0 { if assign61960_e96307 == 0.0 { 0.0 } else { (assign61960_e96307 * ((locals.var_eeff).powf(assign61960_e96307 - 1.0) * locals.var_eeff_dn9)) } } else { (assign61960_e96308 * (assign61960_e96307 * (locals.var_eeff_dn9 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61960_e96307) as f64).is_finite() && ((assign61960_e96307) as f64).fract() == 0.0 { if assign61960_e96307 == 0.0 { 0.0 } else { (assign61960_e96307 * ((locals.var_eeff).powf(assign61960_e96307 - 1.0) * locals.var_eeff_dn10)) } } else { (assign61960_e96308 * (assign61960_e96307 * (locals.var_eeff_dn10 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61960_e96307) as f64).is_finite() && ((assign61960_e96307) as f64).fract() == 0.0 { if assign61960_e96307 == 0.0 { 0.0 } else { (assign61960_e96307 * ((locals.var_eeff).powf(assign61960_e96307 - 1.0) * locals.var_eeff_dn11)) } } else { (assign61960_e96308 * (assign61960_e96307 * (locals.var_eeff_dn11 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61960_e96307) as f64).is_finite() && ((assign61960_e96307) as f64).fract() == 0.0 { if assign61960_e96307 == 0.0 { 0.0 } else { (assign61960_e96307 * ((locals.var_eeff).powf(assign61960_e96307 - 1.0) * locals.var_eeff_dn14)) } } else { (assign61960_e96308 * (assign61960_e96307 * (locals.var_eeff_dn14 / locals.var_eeff))) },)
            }
        };
        (assign61960_e96309, assign61960_e96309_d_n0, assign61960_e96309_d_n2, assign61960_e96309_d_n4, assign61960_e96309_d_n5, assign61960_e96309_d_n6, assign61960_e96309_d_n7, assign61960_e96309_d_n8, assign61960_e96309_d_n9, assign61960_e96309_d_n10, assign61960_e96309_d_n11, assign61960_e96309_d_n14,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign61960_e96311;
        locals.var_t5_dn0 = assign61960_e96311_d_n0;
        locals.var_t5_dn2 = assign61960_e96311_d_n2;
        locals.var_t5_dn4 = assign61960_e96311_d_n4;
        locals.var_t5_dn5 = assign61960_e96311_d_n5;
        locals.var_t5_dn6 = assign61960_e96311_d_n6;
        locals.var_t5_dn7 = assign61960_e96311_d_n7;
        locals.var_t5_dn8 = assign61960_e96311_d_n8;
        locals.var_t5_dn9 = assign61960_e96311_d_n9;
        locals.var_t5_dn10 = assign61960_e96311_d_n10;
        locals.var_t5_dn11 = assign61960_e96311_d_n11;
        locals.var_t5_dn14 = assign61960_e96311_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign61970_e96320, assign61970_e96320_d_n0, assign61970_e96320_d_n2, assign61970_e96320_d_n4, assign61970_e96320_d_n5, assign61970_e96320_d_n6, assign61970_e96320_d_n7, assign61970_e96320_d_n8, assign61970_e96320_d_n9, assign61970_e96320_d_n10, assign61970_e96320_d_n11, assign61970_e96320_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign61970_e96318: f64 = (locals.var_t5 * locals.var_eeff);
        (assign61970_e96318, ((locals.var_t5_dn0 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn0)), ((locals.var_t5_dn2 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn2)), ((locals.var_t5_dn4 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn4)), ((locals.var_t5_dn5 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn5)), ((locals.var_t5_dn6 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn6)), ((locals.var_t5_dn7 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn7)), ((locals.var_t5_dn8 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn8)), ((locals.var_t5_dn9 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn9)), ((locals.var_t5_dn10 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn10)), ((locals.var_t5_dn11 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn11)), ((locals.var_t5_dn14 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn14)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign61970_e96320;
        locals.var_t8_dn0 = assign61970_e96320_d_n0;
        locals.var_t8_dn2 = assign61970_e96320_d_n2;
        locals.var_t8_dn4 = assign61970_e96320_d_n4;
        locals.var_t8_dn5 = assign61970_e96320_d_n5;
        locals.var_t8_dn6 = assign61970_e96320_d_n6;
        locals.var_t8_dn7 = assign61970_e96320_d_n7;
        locals.var_t8_dn8 = assign61970_e96320_d_n8;
        locals.var_t8_dn9 = assign61970_e96320_d_n9;
        locals.var_t8_dn10 = assign61970_e96320_d_n10;
        locals.var_t8_dn11 = assign61970_e96320_d_n11;
        locals.var_t8_dn14 = assign61970_e96320_d_n14;
        locals.var_t8_rv = 0.0;

        let (assign61980_e96336, assign61980_e96336_d_n0, assign61980_e96336_d_n2, assign61980_e96336_d_n4, assign61980_e96336_d_n5, assign61980_e96336_d_n6, assign61980_e96336_d_n7, assign61980_e96336_d_n8, assign61980_e96336_d_n9, assign61980_e96336_d_n10, assign61980_e96336_d_n11, assign61980_e96336_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let (assign61980_e96334, assign61980_e96334_d_n0, assign61980_e96334_d_n2, assign61980_e96334_d_n4, assign61980_e96334_d_n5, assign61980_e96334_d_n6, assign61980_e96334_d_n7, assign61980_e96334_d_n8, assign61980_e96334_d_n9, assign61980_e96334_d_n10, assign61980_e96334_d_n11, assign61980_e96334_d_n14,) = {
            if (locals.var_eeff == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign61980_e96332: f64 = (locals.var_muesr - 1.0);
                let assign61980_e96333: f64 = (locals.var_eeff).powf(assign61980_e96332);
                (assign61980_e96333, if 0.0 == 0.0 && ((assign61980_e96332) as f64).is_finite() && ((assign61980_e96332) as f64).fract() == 0.0 { if assign61980_e96332 == 0.0 { 0.0 } else { (assign61980_e96332 * ((locals.var_eeff).powf(assign61980_e96332 - 1.0) * locals.var_eeff_dn0)) } } else { (assign61980_e96333 * (assign61980_e96332 * (locals.var_eeff_dn0 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61980_e96332) as f64).is_finite() && ((assign61980_e96332) as f64).fract() == 0.0 { if assign61980_e96332 == 0.0 { 0.0 } else { (assign61980_e96332 * ((locals.var_eeff).powf(assign61980_e96332 - 1.0) * locals.var_eeff_dn2)) } } else { (assign61980_e96333 * (assign61980_e96332 * (locals.var_eeff_dn2 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61980_e96332) as f64).is_finite() && ((assign61980_e96332) as f64).fract() == 0.0 { if assign61980_e96332 == 0.0 { 0.0 } else { (assign61980_e96332 * ((locals.var_eeff).powf(assign61980_e96332 - 1.0) * locals.var_eeff_dn4)) } } else { (assign61980_e96333 * (assign61980_e96332 * (locals.var_eeff_dn4 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61980_e96332) as f64).is_finite() && ((assign61980_e96332) as f64).fract() == 0.0 { if assign61980_e96332 == 0.0 { 0.0 } else { (assign61980_e96332 * ((locals.var_eeff).powf(assign61980_e96332 - 1.0) * locals.var_eeff_dn5)) } } else { (assign61980_e96333 * (assign61980_e96332 * (locals.var_eeff_dn5 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61980_e96332) as f64).is_finite() && ((assign61980_e96332) as f64).fract() == 0.0 { if assign61980_e96332 == 0.0 { 0.0 } else { (assign61980_e96332 * ((locals.var_eeff).powf(assign61980_e96332 - 1.0) * locals.var_eeff_dn6)) } } else { (assign61980_e96333 * (assign61980_e96332 * (locals.var_eeff_dn6 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61980_e96332) as f64).is_finite() && ((assign61980_e96332) as f64).fract() == 0.0 { if assign61980_e96332 == 0.0 { 0.0 } else { (assign61980_e96332 * ((locals.var_eeff).powf(assign61980_e96332 - 1.0) * locals.var_eeff_dn7)) } } else { (assign61980_e96333 * (assign61980_e96332 * (locals.var_eeff_dn7 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61980_e96332) as f64).is_finite() && ((assign61980_e96332) as f64).fract() == 0.0 { if assign61980_e96332 == 0.0 { 0.0 } else { (assign61980_e96332 * ((locals.var_eeff).powf(assign61980_e96332 - 1.0) * locals.var_eeff_dn8)) } } else { (assign61980_e96333 * (assign61980_e96332 * (locals.var_eeff_dn8 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61980_e96332) as f64).is_finite() && ((assign61980_e96332) as f64).fract() == 0.0 { if assign61980_e96332 == 0.0 { 0.0 } else { (assign61980_e96332 * ((locals.var_eeff).powf(assign61980_e96332 - 1.0) * locals.var_eeff_dn9)) } } else { (assign61980_e96333 * (assign61980_e96332 * (locals.var_eeff_dn9 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61980_e96332) as f64).is_finite() && ((assign61980_e96332) as f64).fract() == 0.0 { if assign61980_e96332 == 0.0 { 0.0 } else { (assign61980_e96332 * ((locals.var_eeff).powf(assign61980_e96332 - 1.0) * locals.var_eeff_dn10)) } } else { (assign61980_e96333 * (assign61980_e96332 * (locals.var_eeff_dn10 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61980_e96332) as f64).is_finite() && ((assign61980_e96332) as f64).fract() == 0.0 { if assign61980_e96332 == 0.0 { 0.0 } else { (assign61980_e96332 * ((locals.var_eeff).powf(assign61980_e96332 - 1.0) * locals.var_eeff_dn11)) } } else { (assign61980_e96333 * (assign61980_e96332 * (locals.var_eeff_dn11 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign61980_e96332) as f64).is_finite() && ((assign61980_e96332) as f64).fract() == 0.0 { if assign61980_e96332 == 0.0 { 0.0 } else { (assign61980_e96332 * ((locals.var_eeff).powf(assign61980_e96332 - 1.0) * locals.var_eeff_dn14)) } } else { (assign61980_e96333 * (assign61980_e96332 * (locals.var_eeff_dn14 / locals.var_eeff))) },)
            }
        };
        (assign61980_e96334, assign61980_e96334_d_n0, assign61980_e96334_d_n2, assign61980_e96334_d_n4, assign61980_e96334_d_n5, assign61980_e96334_d_n6, assign61980_e96334_d_n7, assign61980_e96334_d_n8, assign61980_e96334_d_n9, assign61980_e96334_d_n10, assign61980_e96334_d_n11, assign61980_e96334_d_n14,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign61980_e96336;
        locals.var_t7_dn0 = assign61980_e96336_d_n0;
        locals.var_t7_dn2 = assign61980_e96336_d_n2;
        locals.var_t7_dn4 = assign61980_e96336_d_n4;
        locals.var_t7_dn5 = assign61980_e96336_d_n5;
        locals.var_t7_dn6 = assign61980_e96336_d_n6;
        locals.var_t7_dn7 = assign61980_e96336_d_n7;
        locals.var_t7_dn8 = assign61980_e96336_d_n8;
        locals.var_t7_dn9 = assign61980_e96336_d_n9;
        locals.var_t7_dn10 = assign61980_e96336_d_n10;
        locals.var_t7_dn11 = assign61980_e96336_d_n11;
        locals.var_t7_dn14 = assign61980_e96336_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign61990_e96345, assign61990_e96345_d_n0, assign61990_e96345_d_n2, assign61990_e96345_d_n4, assign61990_e96345_d_n5, assign61990_e96345_d_n6, assign61990_e96345_d_n7, assign61990_e96345_d_n8, assign61990_e96345_d_n9, assign61990_e96345_d_n10, assign61990_e96345_d_n11, assign61990_e96345_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign61990_e96343: f64 = (locals.var_t7 * locals.var_eeff);
        (assign61990_e96343, ((locals.var_t7_dn0 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn0)), ((locals.var_t7_dn2 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn2)), ((locals.var_t7_dn4 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn4)), ((locals.var_t7_dn5 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn5)), ((locals.var_t7_dn6 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn6)), ((locals.var_t7_dn7 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn7)), ((locals.var_t7_dn8 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn8)), ((locals.var_t7_dn9 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn9)), ((locals.var_t7_dn10 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn10)), ((locals.var_t7_dn11 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn11)), ((locals.var_t7_dn14 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign61990_e96345;
        locals.var_t6_dn0 = assign61990_e96345_d_n0;
        locals.var_t6_dn2 = assign61990_e96345_d_n2;
        locals.var_t6_dn4 = assign61990_e96345_d_n4;
        locals.var_t6_dn5 = assign61990_e96345_d_n5;
        locals.var_t6_dn6 = assign61990_e96345_d_n6;
        locals.var_t6_dn7 = assign61990_e96345_d_n7;
        locals.var_t6_dn8 = assign61990_e96345_d_n8;
        locals.var_t6_dn9 = assign61990_e96345_d_n9;
        locals.var_t6_dn10 = assign61990_e96345_d_n10;
        locals.var_t6_dn11 = assign61990_e96345_d_n11;
        locals.var_t6_dn14 = assign61990_e96345_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign62000_e96354, assign62000_e96354_d_n0, assign62000_e96354_d_n2, assign62000_e96354_d_n4, assign62000_e96354_d_n5, assign62000_e96354_d_n6, assign62000_e96354_d_n7, assign62000_e96354_d_n8, assign62000_e96354_d_n9, assign62000_e96354_d_n10, assign62000_e96354_d_n11, assign62000_e96354_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign62000_e96352: f64 = (1.6021918e-19 * 10000.0);
        (assign62000_e96352, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign62000_e96354;
        locals.var_t9_dn0 = assign62000_e96354_d_n0;
        locals.var_t9_dn2 = assign62000_e96354_d_n2;
        locals.var_t9_dn4 = assign62000_e96354_d_n4;
        locals.var_t9_dn5 = assign62000_e96354_d_n5;
        locals.var_t9_dn6 = assign62000_e96354_d_n6;
        locals.var_t9_dn7 = assign62000_e96354_d_n7;
        locals.var_t9_dn8 = assign62000_e96354_d_n8;
        locals.var_t9_dn9 = assign62000_e96354_d_n9;
        locals.var_t9_dn10 = assign62000_e96354_d_n10;
        locals.var_t9_dn11 = assign62000_e96354_d_n11;
        locals.var_t9_dn14 = assign62000_e96354_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign62010_e96363, assign62010_e96363_d_n0, assign62010_e96363_d_n2, assign62010_e96363_d_n4, assign62010_e96363_d_n5, assign62010_e96363_d_n6, assign62010_e96363_d_n7, assign62010_e96363_d_n8, assign62010_e96363_d_n9, assign62010_e96363_d_n10, assign62010_e96363_d_n11, assign62010_e96363_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign62010_e96361: f64 = (locals.var_qiu / locals.var_t9);
        (assign62010_e96361, (((locals.var_qiu_dn0 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn0)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn2 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn2)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn4 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn4)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn5 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn5)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn6 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn6)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn7 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn7)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn8 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn8)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn9 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn9)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn10 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn10)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn11 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn11)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn14 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn14)) / (locals.var_t9 * locals.var_t9)),)
    } else {
        (locals.var_rns, locals.var_rns_dn0, locals.var_rns_dn2, locals.var_rns_dn4, locals.var_rns_dn5, locals.var_rns_dn6, locals.var_rns_dn7, locals.var_rns_dn8, locals.var_rns_dn9, locals.var_rns_dn10, locals.var_rns_dn11, locals.var_rns_dn14,)
    }
};
        locals.var_rns = assign62010_e96363;
        locals.var_rns_dn0 = assign62010_e96363_d_n0;
        locals.var_rns_dn2 = assign62010_e96363_d_n2;
        locals.var_rns_dn4 = assign62010_e96363_d_n4;
        locals.var_rns_dn5 = assign62010_e96363_d_n5;
        locals.var_rns_dn6 = assign62010_e96363_d_n6;
        locals.var_rns_dn7 = assign62010_e96363_d_n7;
        locals.var_rns_dn8 = assign62010_e96363_d_n8;
        locals.var_rns_dn9 = assign62010_e96363_d_n9;
        locals.var_rns_dn10 = assign62010_e96363_d_n10;
        locals.var_rns_dn11 = assign62010_e96363_d_n11;
        locals.var_rns_dn14 = assign62010_e96363_d_n14;
        locals.var_rns_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_231(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign62020_e96386, assign62020_e96386_d_n0, assign62020_e96386_d_n2, assign62020_e96386_d_n4, assign62020_e96386_d_n5, assign62020_e96386_d_n6, assign62020_e96386_d_n7, assign62020_e96386_d_n8, assign62020_e96386_d_n9, assign62020_e96386_d_n10, assign62020_e96386_d_n11, assign62020_e96386_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign62020_e96372: f64 = (locals.var_uc_muecb1 * locals.var_rns);
        let assign62020_e96374: f64 = (assign62020_e96372 / 100000000000.0);
        let assign62020_e96375: f64 = (locals.var_uc_muecb0 + assign62020_e96374);
        let assign62020_e96376: f64 = (1.0 / assign62020_e96375);
        let assign62020_e96379: f64 = (locals.var_mphn0 * locals.var_t8);
        let assign62020_e96380: f64 = (assign62020_e96376 + assign62020_e96379);
        let assign62020_e96383: f64 = (locals.var_t6 / locals.var_uc_muesr1);
        let assign62020_e96384: f64 = (assign62020_e96380 + assign62020_e96383);
        (assign62020_e96384, (((-(((locals.var_uc_muecb1 * locals.var_rns_dn0) / 100000000000.0) / (assign62020_e96375 * assign62020_e96375))) + ((locals.var_mphn0_dn0 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn0))) + (locals.var_t6_dn0 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn2) / 100000000000.0) / (assign62020_e96375 * assign62020_e96375))) + ((locals.var_mphn0_dn2 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn2))) + (locals.var_t6_dn2 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn4) / 100000000000.0) / (assign62020_e96375 * assign62020_e96375))) + ((locals.var_mphn0_dn4 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn4))) + (locals.var_t6_dn4 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn5) / 100000000000.0) / (assign62020_e96375 * assign62020_e96375))) + ((locals.var_mphn0_dn5 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn5))) + (locals.var_t6_dn5 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn6) / 100000000000.0) / (assign62020_e96375 * assign62020_e96375))) + ((locals.var_mphn0_dn6 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn6))) + (locals.var_t6_dn6 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn7) / 100000000000.0) / (assign62020_e96375 * assign62020_e96375))) + ((locals.var_mphn0_dn7 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn7))) + (locals.var_t6_dn7 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn8) / 100000000000.0) / (assign62020_e96375 * assign62020_e96375))) + ((locals.var_mphn0_dn8 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn8))) + (locals.var_t6_dn8 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn9) / 100000000000.0) / (assign62020_e96375 * assign62020_e96375))) + ((locals.var_mphn0_dn9 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn9))) + (locals.var_t6_dn9 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn10) / 100000000000.0) / (assign62020_e96375 * assign62020_e96375))) + ((locals.var_mphn0_dn10 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn10))) + (locals.var_t6_dn10 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn11) / 100000000000.0) / (assign62020_e96375 * assign62020_e96375))) + ((locals.var_mphn0_dn11 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn11))) + (locals.var_t6_dn11 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn14) / 100000000000.0) / (assign62020_e96375 * assign62020_e96375))) + ((locals.var_mphn0_dn14 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn14))) + (locals.var_t6_dn14 / locals.var_uc_muesr1)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign62020_e96386;
        locals.var_t1_dn0 = assign62020_e96386_d_n0;
        locals.var_t1_dn2 = assign62020_e96386_d_n2;
        locals.var_t1_dn4 = assign62020_e96386_d_n4;
        locals.var_t1_dn5 = assign62020_e96386_d_n5;
        locals.var_t1_dn6 = assign62020_e96386_d_n6;
        locals.var_t1_dn7 = assign62020_e96386_d_n7;
        locals.var_t1_dn8 = assign62020_e96386_d_n8;
        locals.var_t1_dn9 = assign62020_e96386_d_n9;
        locals.var_t1_dn10 = assign62020_e96386_d_n10;
        locals.var_t1_dn11 = assign62020_e96386_d_n11;
        locals.var_t1_dn14 = assign62020_e96386_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign62030_e96395, assign62030_e96395_d_n0, assign62030_e96395_d_n2, assign62030_e96395_d_n4, assign62030_e96395_d_n5, assign62030_e96395_d_n6, assign62030_e96395_d_n7, assign62030_e96395_d_n8, assign62030_e96395_d_n9, assign62030_e96395_d_n10, assign62030_e96395_d_n11, assign62030_e96395_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign62030_e96393: f64 = (1.0 / locals.var_t1);
        (assign62030_e96393, (-(locals.var_t1_dn0 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn2 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn4 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn5 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn6 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn7 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn8 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn9 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn10 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn11 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn14 / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn11, locals.var_muun_dn14,)
    }
};
        locals.var_muun = assign62030_e96395;
        locals.var_muun_dn0 = assign62030_e96395_d_n0;
        locals.var_muun_dn2 = assign62030_e96395_d_n2;
        locals.var_muun_dn4 = assign62030_e96395_d_n4;
        locals.var_muun_dn5 = assign62030_e96395_d_n5;
        locals.var_muun_dn6 = assign62030_e96395_d_n6;
        locals.var_muun_dn7 = assign62030_e96395_d_n7;
        locals.var_muun_dn8 = assign62030_e96395_d_n8;
        locals.var_muun_dn9 = assign62030_e96395_d_n9;
        locals.var_muun_dn10 = assign62030_e96395_d_n10;
        locals.var_muun_dn11 = assign62030_e96395_d_n11;
        locals.var_muun_dn14 = assign62030_e96395_d_n14;
        locals.var_muun_rv = 0.0;

        let (assign62040_e96404, assign62040_e96404_d_n0, assign62040_e96404_d_n2, assign62040_e96404_d_n4, assign62040_e96404_d_n5, assign62040_e96404_d_n6, assign62040_e96404_d_n7, assign62040_e96404_d_n8, assign62040_e96404_d_n9, assign62040_e96404_d_n10, assign62040_e96404_d_n11, assign62040_e96404_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign62040_e96402: f64 = (locals.var_muun / 10000.0);
        (assign62040_e96402, (locals.var_muun_dn0 / 10000.0), (locals.var_muun_dn2 / 10000.0), (locals.var_muun_dn4 / 10000.0), (locals.var_muun_dn5 / 10000.0), (locals.var_muun_dn6 / 10000.0), (locals.var_muun_dn7 / 10000.0), (locals.var_muun_dn8 / 10000.0), (locals.var_muun_dn9 / 10000.0), (locals.var_muun_dn10 / 10000.0), (locals.var_muun_dn11 / 10000.0), (locals.var_muun_dn14 / 10000.0),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn11, locals.var_muun_dn14,)
    }
};
        locals.var_muun = assign62040_e96404;
        locals.var_muun_dn0 = assign62040_e96404_d_n0;
        locals.var_muun_dn2 = assign62040_e96404_d_n2;
        locals.var_muun_dn4 = assign62040_e96404_d_n4;
        locals.var_muun_dn5 = assign62040_e96404_d_n5;
        locals.var_muun_dn6 = assign62040_e96404_d_n6;
        locals.var_muun_dn7 = assign62040_e96404_d_n7;
        locals.var_muun_dn8 = assign62040_e96404_d_n8;
        locals.var_muun_dn9 = assign62040_e96404_d_n9;
        locals.var_muun_dn10 = assign62040_e96404_d_n10;
        locals.var_muun_dn11 = assign62040_e96404_d_n11;
        locals.var_muun_dn14 = assign62040_e96404_d_n14;
        locals.var_muun_rv = 0.0;

        let (assign62050_e96417, assign62050_e96417_d_n0, assign62050_e96417_d_n2, assign62050_e96417_d_n4, assign62050_e96417_d_n5, assign62050_e96417_d_n6, assign62050_e96417_d_n7, assign62050_e96417_d_n8, assign62050_e96417_d_n9, assign62050_e96417_d_n10, assign62050_e96417_d_n11, assign62050_e96417_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign62050_e96412: f64 = (locals.var_qn0 + 1e-25);
        let assign62050_e96413: f64 = (locals.var_beta * assign62050_e96412);
        let assign62050_e96415: f64 = (assign62050_e96413 * locals.var_lch);
        (assign62050_e96415, ((((locals.var_beta_dn0 * assign62050_e96412) + (locals.var_beta * locals.var_qn0_dn0)) * locals.var_lch) + (assign62050_e96413 * locals.var_lch_dn0)), ((((locals.var_beta_dn2 * assign62050_e96412) + (locals.var_beta * locals.var_qn0_dn2)) * locals.var_lch) + (assign62050_e96413 * locals.var_lch_dn2)), ((((locals.var_beta_dn4 * assign62050_e96412) + (locals.var_beta * locals.var_qn0_dn4)) * locals.var_lch) + (assign62050_e96413 * locals.var_lch_dn4)), ((((locals.var_beta_dn5 * assign62050_e96412) + (locals.var_beta * locals.var_qn0_dn5)) * locals.var_lch) + (assign62050_e96413 * locals.var_lch_dn5)), ((((locals.var_beta_dn6 * assign62050_e96412) + (locals.var_beta * locals.var_qn0_dn6)) * locals.var_lch) + (assign62050_e96413 * locals.var_lch_dn6)), ((((locals.var_beta_dn7 * assign62050_e96412) + (locals.var_beta * locals.var_qn0_dn7)) * locals.var_lch) + (assign62050_e96413 * locals.var_lch_dn7)), ((((locals.var_beta_dn8 * assign62050_e96412) + (locals.var_beta * locals.var_qn0_dn8)) * locals.var_lch) + (assign62050_e96413 * locals.var_lch_dn8)), ((((locals.var_beta_dn9 * assign62050_e96412) + (locals.var_beta * locals.var_qn0_dn9)) * locals.var_lch) + (assign62050_e96413 * locals.var_lch_dn9)), ((((locals.var_beta_dn10 * assign62050_e96412) + (locals.var_beta * locals.var_qn0_dn10)) * locals.var_lch) + (assign62050_e96413 * locals.var_lch_dn10)), ((((locals.var_beta_dn11 * assign62050_e96412) + (locals.var_beta * locals.var_qn0_dn11)) * locals.var_lch) + (assign62050_e96413 * locals.var_lch_dn11)), ((((locals.var_beta_dn14 * assign62050_e96412) + (locals.var_beta * locals.var_qn0_dn14)) * locals.var_lch) + (assign62050_e96413 * locals.var_lch_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign62050_e96417;
        locals.var_t2_dn0 = assign62050_e96417_d_n0;
        locals.var_t2_dn2 = assign62050_e96417_d_n2;
        locals.var_t2_dn4 = assign62050_e96417_d_n4;
        locals.var_t2_dn5 = assign62050_e96417_d_n5;
        locals.var_t2_dn6 = assign62050_e96417_d_n6;
        locals.var_t2_dn7 = assign62050_e96417_d_n7;
        locals.var_t2_dn8 = assign62050_e96417_d_n8;
        locals.var_t2_dn9 = assign62050_e96417_d_n9;
        locals.var_t2_dn10 = assign62050_e96417_d_n10;
        locals.var_t2_dn11 = assign62050_e96417_d_n11;
        locals.var_t2_dn14 = assign62050_e96417_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign62060_e96426, assign62060_e96426_d_n0, assign62060_e96426_d_n2, assign62060_e96426_d_n4, assign62060_e96426_d_n5, assign62060_e96426_d_n6, assign62060_e96426_d_n7, assign62060_e96426_d_n8, assign62060_e96426_d_n9, assign62060_e96426_d_n10, assign62060_e96426_d_n11, assign62060_e96426_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign62060_e96424: f64 = (1.0 / locals.var_t2);
        (assign62060_e96424, (-(locals.var_t2_dn0 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn2 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn4 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn5 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn6 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn7 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn8 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn9 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn10 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn11 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn14 / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign62060_e96426;
        locals.var_t1_dn0 = assign62060_e96426_d_n0;
        locals.var_t1_dn2 = assign62060_e96426_d_n2;
        locals.var_t1_dn4 = assign62060_e96426_d_n4;
        locals.var_t1_dn5 = assign62060_e96426_d_n5;
        locals.var_t1_dn6 = assign62060_e96426_d_n6;
        locals.var_t1_dn7 = assign62060_e96426_d_n7;
        locals.var_t1_dn8 = assign62060_e96426_d_n8;
        locals.var_t1_dn9 = assign62060_e96426_d_n9;
        locals.var_t1_dn10 = assign62060_e96426_d_n10;
        locals.var_t1_dn11 = assign62060_e96426_d_n11;
        locals.var_t1_dn14 = assign62060_e96426_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign62070_e96435, assign62070_e96435_d_n0, assign62070_e96435_d_n2, assign62070_e96435_d_n4, assign62070_e96435_d_n5, assign62070_e96435_d_n6, assign62070_e96435_d_n7, assign62070_e96435_d_n8, assign62070_e96435_d_n9, assign62070_e96435_d_n10, assign62070_e96435_d_n11, assign62070_e96435_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign62070_e96433: f64 = (locals.var_t1 * locals.var_t1);
        (assign62070_e96433, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)), ((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign62070_e96435;
        locals.var_t3_dn0 = assign62070_e96435_d_n0;
        locals.var_t3_dn2 = assign62070_e96435_d_n2;
        locals.var_t3_dn4 = assign62070_e96435_d_n4;
        locals.var_t3_dn5 = assign62070_e96435_d_n5;
        locals.var_t3_dn6 = assign62070_e96435_d_n6;
        locals.var_t3_dn7 = assign62070_e96435_d_n7;
        locals.var_t3_dn8 = assign62070_e96435_d_n8;
        locals.var_t3_dn9 = assign62070_e96435_d_n9;
        locals.var_t3_dn10 = assign62070_e96435_d_n10;
        locals.var_t3_dn11 = assign62070_e96435_d_n11;
        locals.var_t3_dn14 = assign62070_e96435_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign62080_e96445, assign62080_e96445_d_n0, assign62080_e96445_d_n2, assign62080_e96445_d_n4, assign62080_e96445_d_n5, assign62080_e96445_d_n6, assign62080_e96445_d_n7, assign62080_e96445_d_n8, assign62080_e96445_d_n9, assign62080_e96445_d_n10, assign62080_e96445_d_n11, assign62080_e96445_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign62080_e96441: f64 = (-locals.var_beta);
        let assign62080_e96443: f64 = (assign62080_e96441 * locals.var_t3);
        (assign62080_e96443, (((-locals.var_beta_dn0) * locals.var_t3) + (assign62080_e96441 * locals.var_t3_dn0)), (((-locals.var_beta_dn2) * locals.var_t3) + (assign62080_e96441 * locals.var_t3_dn2)), (((-locals.var_beta_dn4) * locals.var_t3) + (assign62080_e96441 * locals.var_t3_dn4)), (((-locals.var_beta_dn5) * locals.var_t3) + (assign62080_e96441 * locals.var_t3_dn5)), (((-locals.var_beta_dn6) * locals.var_t3) + (assign62080_e96441 * locals.var_t3_dn6)), (((-locals.var_beta_dn7) * locals.var_t3) + (assign62080_e96441 * locals.var_t3_dn7)), (((-locals.var_beta_dn8) * locals.var_t3) + (assign62080_e96441 * locals.var_t3_dn8)), (((-locals.var_beta_dn9) * locals.var_t3) + (assign62080_e96441 * locals.var_t3_dn9)), (((-locals.var_beta_dn10) * locals.var_t3) + (assign62080_e96441 * locals.var_t3_dn10)), (((-locals.var_beta_dn11) * locals.var_t3) + (assign62080_e96441 * locals.var_t3_dn11)), (((-locals.var_beta_dn14) * locals.var_t3) + (assign62080_e96441 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign62080_e96445;
        locals.var_t4_dn0 = assign62080_e96445_d_n0;
        locals.var_t4_dn2 = assign62080_e96445_d_n2;
        locals.var_t4_dn4 = assign62080_e96445_d_n4;
        locals.var_t4_dn5 = assign62080_e96445_d_n5;
        locals.var_t4_dn6 = assign62080_e96445_d_n6;
        locals.var_t4_dn7 = assign62080_e96445_d_n7;
        locals.var_t4_dn8 = assign62080_e96445_d_n8;
        locals.var_t4_dn9 = assign62080_e96445_d_n9;
        locals.var_t4_dn10 = assign62080_e96445_d_n10;
        locals.var_t4_dn11 = assign62080_e96445_d_n11;
        locals.var_t4_dn14 = assign62080_e96445_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign62090_e96454, assign62090_e96454_d_n0, assign62090_e96454_d_n2, assign62090_e96454_d_n4, assign62090_e96454_d_n5, assign62090_e96454_d_n6, assign62090_e96454_d_n7, assign62090_e96454_d_n8, assign62090_e96454_d_n9, assign62090_e96454_d_n10, assign62090_e96454_d_n11, assign62090_e96454_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign62090_e96452: f64 = (locals.var_t4 * locals.var_lch);
        (assign62090_e96452, ((locals.var_t4_dn0 * locals.var_lch) + (locals.var_t4 * locals.var_lch_dn0)), ((locals.var_t4_dn2 * locals.var_lch) + (locals.var_t4 * locals.var_lch_dn2)), ((locals.var_t4_dn4 * locals.var_lch) + (locals.var_t4 * locals.var_lch_dn4)), ((locals.var_t4_dn5 * locals.var_lch) + (locals.var_t4 * locals.var_lch_dn5)), ((locals.var_t4_dn6 * locals.var_lch) + (locals.var_t4 * locals.var_lch_dn6)), ((locals.var_t4_dn7 * locals.var_lch) + (locals.var_t4 * locals.var_lch_dn7)), ((locals.var_t4_dn8 * locals.var_lch) + (locals.var_t4 * locals.var_lch_dn8)), ((locals.var_t4_dn9 * locals.var_lch) + (locals.var_t4 * locals.var_lch_dn9)), ((locals.var_t4_dn10 * locals.var_lch) + (locals.var_t4 * locals.var_lch_dn10)), ((locals.var_t4_dn11 * locals.var_lch) + (locals.var_t4 * locals.var_lch_dn11)), ((locals.var_t4_dn14 * locals.var_lch) + (locals.var_t4 * locals.var_lch_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign62090_e96454;
        locals.var_t5_dn0 = assign62090_e96454_d_n0;
        locals.var_t5_dn2 = assign62090_e96454_d_n2;
        locals.var_t5_dn4 = assign62090_e96454_d_n4;
        locals.var_t5_dn5 = assign62090_e96454_d_n5;
        locals.var_t5_dn6 = assign62090_e96454_d_n6;
        locals.var_t5_dn7 = assign62090_e96454_d_n7;
        locals.var_t5_dn8 = assign62090_e96454_d_n8;
        locals.var_t5_dn9 = assign62090_e96454_d_n9;
        locals.var_t5_dn10 = assign62090_e96454_d_n10;
        locals.var_t5_dn11 = assign62090_e96454_d_n11;
        locals.var_t5_dn14 = assign62090_e96454_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign62100_e96465, assign62100_e96465_d_n0, assign62100_e96465_d_n2, assign62100_e96465_d_n4, assign62100_e96465_d_n5, assign62100_e96465_d_n6, assign62100_e96465_d_n7, assign62100_e96465_d_n8, assign62100_e96465_d_n9, assign62100_e96465_d_n10, assign62100_e96465_d_n11, assign62100_e96465_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign62100_e96462: f64 = (locals.var_qn0 + 1e-25);
        let assign62100_e96463: f64 = (locals.var_t4 * assign62100_e96462);
        (assign62100_e96463, ((locals.var_t4_dn0 * assign62100_e96462) + (locals.var_t4 * locals.var_qn0_dn0)), ((locals.var_t4_dn2 * assign62100_e96462) + (locals.var_t4 * locals.var_qn0_dn2)), ((locals.var_t4_dn4 * assign62100_e96462) + (locals.var_t4 * locals.var_qn0_dn4)), ((locals.var_t4_dn5 * assign62100_e96462) + (locals.var_t4 * locals.var_qn0_dn5)), ((locals.var_t4_dn6 * assign62100_e96462) + (locals.var_t4 * locals.var_qn0_dn6)), ((locals.var_t4_dn7 * assign62100_e96462) + (locals.var_t4 * locals.var_qn0_dn7)), ((locals.var_t4_dn8 * assign62100_e96462) + (locals.var_t4 * locals.var_qn0_dn8)), ((locals.var_t4_dn9 * assign62100_e96462) + (locals.var_t4 * locals.var_qn0_dn9)), ((locals.var_t4_dn10 * assign62100_e96462) + (locals.var_t4 * locals.var_qn0_dn10)), ((locals.var_t4_dn11 * assign62100_e96462) + (locals.var_t4 * locals.var_qn0_dn11)), ((locals.var_t4_dn14 * assign62100_e96462) + (locals.var_t4 * locals.var_qn0_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign62100_e96465;
        locals.var_t6_dn0 = assign62100_e96465_d_n0;
        locals.var_t6_dn2 = assign62100_e96465_d_n2;
        locals.var_t6_dn4 = assign62100_e96465_d_n4;
        locals.var_t6_dn5 = assign62100_e96465_d_n5;
        locals.var_t6_dn6 = assign62100_e96465_d_n6;
        locals.var_t6_dn7 = assign62100_e96465_d_n7;
        locals.var_t6_dn8 = assign62100_e96465_d_n8;
        locals.var_t6_dn9 = assign62100_e96465_d_n9;
        locals.var_t6_dn10 = assign62100_e96465_d_n10;
        locals.var_t6_dn11 = assign62100_e96465_d_n11;
        locals.var_t6_dn14 = assign62100_e96465_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign62110_e96480, assign62110_e96480_d_n0, assign62110_e96480_d_n2, assign62110_e96480_d_n4, assign62110_e96480_d_n5, assign62110_e96480_d_n6, assign62110_e96480_d_n7, assign62110_e96480_d_n8, assign62110_e96480_d_n9, assign62110_e96480_d_n10, assign62110_e96480_d_n11, assign62110_e96480_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign62110_e96473: f64 = (10.0 * 2.220446049250313e-16);
        let assign62110_e96474: f64 = (locals.var_pds + assign62110_e96473);
        let assign62110_e96476: f64 = (assign62110_e96474 * locals.var_fdd);
        let assign62110_e96478: f64 = (assign62110_e96476 * locals.var_t1);
        (assign62110_e96478, ((((locals.var_pds_dn0 * locals.var_fdd) + (assign62110_e96474 * locals.var_fdd_dn0)) * locals.var_t1) + (assign62110_e96476 * locals.var_t1_dn0)), ((((locals.var_pds_dn2 * locals.var_fdd) + (assign62110_e96474 * locals.var_fdd_dn2)) * locals.var_t1) + (assign62110_e96476 * locals.var_t1_dn2)), ((((locals.var_pds_dn4 * locals.var_fdd) + (assign62110_e96474 * locals.var_fdd_dn4)) * locals.var_t1) + (assign62110_e96476 * locals.var_t1_dn4)), ((((locals.var_pds_dn5 * locals.var_fdd) + (assign62110_e96474 * locals.var_fdd_dn5)) * locals.var_t1) + (assign62110_e96476 * locals.var_t1_dn5)), ((((locals.var_pds_dn6 * locals.var_fdd) + (assign62110_e96474 * locals.var_fdd_dn6)) * locals.var_t1) + (assign62110_e96476 * locals.var_t1_dn6)), ((((locals.var_pds_dn7 * locals.var_fdd) + (assign62110_e96474 * locals.var_fdd_dn7)) * locals.var_t1) + (assign62110_e96476 * locals.var_t1_dn7)), ((((locals.var_pds_dn8 * locals.var_fdd) + (assign62110_e96474 * locals.var_fdd_dn8)) * locals.var_t1) + (assign62110_e96476 * locals.var_t1_dn8)), ((((locals.var_pds_dn9 * locals.var_fdd) + (assign62110_e96474 * locals.var_fdd_dn9)) * locals.var_t1) + (assign62110_e96476 * locals.var_t1_dn9)), ((((locals.var_pds_dn10 * locals.var_fdd) + (assign62110_e96474 * locals.var_fdd_dn10)) * locals.var_t1) + (assign62110_e96476 * locals.var_t1_dn10)), ((((locals.var_pds_dn11 * locals.var_fdd) + (assign62110_e96474 * locals.var_fdd_dn11)) * locals.var_t1) + (assign62110_e96476 * locals.var_t1_dn11)), ((((locals.var_pds_dn14 * locals.var_fdd) + (assign62110_e96474 * locals.var_fdd_dn14)) * locals.var_t1) + (assign62110_e96476 * locals.var_t1_dn14)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign62110_e96480;
        locals.var_ty_dn0 = assign62110_e96480_d_n0;
        locals.var_ty_dn2 = assign62110_e96480_d_n2;
        locals.var_ty_dn4 = assign62110_e96480_d_n4;
        locals.var_ty_dn5 = assign62110_e96480_d_n5;
        locals.var_ty_dn6 = assign62110_e96480_d_n6;
        locals.var_ty_dn7 = assign62110_e96480_d_n7;
        locals.var_ty_dn8 = assign62110_e96480_d_n8;
        locals.var_ty_dn9 = assign62110_e96480_d_n9;
        locals.var_ty_dn10 = assign62110_e96480_d_n10;
        locals.var_ty_dn11 = assign62110_e96480_d_n11;
        locals.var_ty_dn14 = assign62110_e96480_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign62120_e96491, assign62120_e96491_d_n0, assign62120_e96491_d_n2, assign62120_e96491_d_n4, assign62120_e96491_d_n5, assign62120_e96491_d_n6, assign62120_e96491_d_n7, assign62120_e96491_d_n8, assign62120_e96491_d_n9, assign62120_e96491_d_n10, assign62120_e96491_d_n11, assign62120_e96491_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign62120_e96487: f64 = (0.2 * locals.var_vmaxe);
        let assign62120_e96489: f64 = (assign62120_e96487 / locals.var_muun);
        (assign62120_e96489, ((((0.2 * locals.var_vmaxe_dn0) * locals.var_muun) - (assign62120_e96487 * locals.var_muun_dn0)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn2) * locals.var_muun) - (assign62120_e96487 * locals.var_muun_dn2)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn4) * locals.var_muun) - (assign62120_e96487 * locals.var_muun_dn4)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn5) * locals.var_muun) - (assign62120_e96487 * locals.var_muun_dn5)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn6) * locals.var_muun) - (assign62120_e96487 * locals.var_muun_dn6)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn7) * locals.var_muun) - (assign62120_e96487 * locals.var_muun_dn7)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn8) * locals.var_muun) - (assign62120_e96487 * locals.var_muun_dn8)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn9) * locals.var_muun) - (assign62120_e96487 * locals.var_muun_dn9)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn10) * locals.var_muun) - (assign62120_e96487 * locals.var_muun_dn10)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn11) * locals.var_muun) - (assign62120_e96487 * locals.var_muun_dn11)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn14) * locals.var_muun) - (assign62120_e96487 * locals.var_muun_dn14)) / (locals.var_muun * locals.var_muun)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign62120_e96491;
        locals.var_t2_dn0 = assign62120_e96491_d_n0;
        locals.var_t2_dn2 = assign62120_e96491_d_n2;
        locals.var_t2_dn4 = assign62120_e96491_d_n4;
        locals.var_t2_dn5 = assign62120_e96491_d_n5;
        locals.var_t2_dn6 = assign62120_e96491_d_n6;
        locals.var_t2_dn7 = assign62120_e96491_d_n7;
        locals.var_t2_dn8 = assign62120_e96491_d_n8;
        locals.var_t2_dn9 = assign62120_e96491_d_n9;
        locals.var_t2_dn10 = assign62120_e96491_d_n10;
        locals.var_t2_dn11 = assign62120_e96491_d_n11;
        locals.var_t2_dn14 = assign62120_e96491_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign62130_e96501, assign62130_e96501_d_n0, assign62130_e96501_d_n2, assign62130_e96501_d_n4, assign62130_e96501_d_n5, assign62130_e96501_d_n6, assign62130_e96501_d_n7, assign62130_e96501_d_n8, assign62130_e96501_d_n9, assign62130_e96501_d_n10, assign62130_e96501_d_n11, assign62130_e96501_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign62130_e96497: f64 = (-locals.var_t2);
        let assign62130_e96499: f64 = (assign62130_e96497 / locals.var_muun);
        (assign62130_e96499, ((((-locals.var_t2_dn0) * locals.var_muun) - (assign62130_e96497 * locals.var_muun_dn0)) / (locals.var_muun * locals.var_muun)), ((((-locals.var_t2_dn2) * locals.var_muun) - (assign62130_e96497 * locals.var_muun_dn2)) / (locals.var_muun * locals.var_muun)), ((((-locals.var_t2_dn4) * locals.var_muun) - (assign62130_e96497 * locals.var_muun_dn4)) / (locals.var_muun * locals.var_muun)), ((((-locals.var_t2_dn5) * locals.var_muun) - (assign62130_e96497 * locals.var_muun_dn5)) / (locals.var_muun * locals.var_muun)), ((((-locals.var_t2_dn6) * locals.var_muun) - (assign62130_e96497 * locals.var_muun_dn6)) / (locals.var_muun * locals.var_muun)), ((((-locals.var_t2_dn7) * locals.var_muun) - (assign62130_e96497 * locals.var_muun_dn7)) / (locals.var_muun * locals.var_muun)), ((((-locals.var_t2_dn8) * locals.var_muun) - (assign62130_e96497 * locals.var_muun_dn8)) / (locals.var_muun * locals.var_muun)), ((((-locals.var_t2_dn9) * locals.var_muun) - (assign62130_e96497 * locals.var_muun_dn9)) / (locals.var_muun * locals.var_muun)), ((((-locals.var_t2_dn10) * locals.var_muun) - (assign62130_e96497 * locals.var_muun_dn10)) / (locals.var_muun * locals.var_muun)), ((((-locals.var_t2_dn11) * locals.var_muun) - (assign62130_e96497 * locals.var_muun_dn11)) / (locals.var_muun * locals.var_muun)), ((((-locals.var_t2_dn14) * locals.var_muun) - (assign62130_e96497 * locals.var_muun_dn14)) / (locals.var_muun * locals.var_muun)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign62130_e96501;
        locals.var_t3_dn0 = assign62130_e96501_d_n0;
        locals.var_t3_dn2 = assign62130_e96501_d_n2;
        locals.var_t3_dn4 = assign62130_e96501_d_n4;
        locals.var_t3_dn5 = assign62130_e96501_d_n5;
        locals.var_t3_dn6 = assign62130_e96501_d_n6;
        locals.var_t3_dn7 = assign62130_e96501_d_n7;
        locals.var_t3_dn8 = assign62130_e96501_d_n8;
        locals.var_t3_dn9 = assign62130_e96501_d_n9;
        locals.var_t3_dn10 = assign62130_e96501_d_n10;
        locals.var_t3_dn11 = assign62130_e96501_d_n11;
        locals.var_t3_dn14 = assign62130_e96501_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign62140_e96515, assign62140_e96515_d_n0, assign62140_e96515_d_n2, assign62140_e96515_d_n4, assign62140_e96515_d_n5, assign62140_e96515_d_n6, assign62140_e96515_d_n7, assign62140_e96515_d_n8, assign62140_e96515_d_n9, assign62140_e96515_d_n10, assign62140_e96515_d_n11, assign62140_e96515_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign62140_e96508: f64 = (locals.var_ty * locals.var_ty);
        let assign62140_e96511: f64 = (locals.var_t2 * locals.var_t2);
        let assign62140_e96512: f64 = (assign62140_e96508 + assign62140_e96511);
        let assign62140_e96513: f64 = (assign62140_e96512).sqrt();
        (assign62140_e96513, ((((locals.var_ty_dn0 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn0)) + ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0))) / (2.0 * assign62140_e96513)), ((((locals.var_ty_dn2 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn2)) + ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2))) / (2.0 * assign62140_e96513)), ((((locals.var_ty_dn4 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn4)) + ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4))) / (2.0 * assign62140_e96513)), ((((locals.var_ty_dn5 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn5)) + ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5))) / (2.0 * assign62140_e96513)), ((((locals.var_ty_dn6 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn6)) + ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6))) / (2.0 * assign62140_e96513)), ((((locals.var_ty_dn7 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn7)) + ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7))) / (2.0 * assign62140_e96513)), ((((locals.var_ty_dn8 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn8)) + ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8))) / (2.0 * assign62140_e96513)), ((((locals.var_ty_dn9 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn9)) + ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9))) / (2.0 * assign62140_e96513)), ((((locals.var_ty_dn10 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn10)) + ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10))) / (2.0 * assign62140_e96513)), ((((locals.var_ty_dn11 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn11)) + ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11))) / (2.0 * assign62140_e96513)), ((((locals.var_ty_dn14 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn14)) + ((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14))) / (2.0 * assign62140_e96513)),)
    } else {
        (locals.var_ey, locals.var_ey_dn0, locals.var_ey_dn2, locals.var_ey_dn4, locals.var_ey_dn5, locals.var_ey_dn6, locals.var_ey_dn7, locals.var_ey_dn8, locals.var_ey_dn9, locals.var_ey_dn10, locals.var_ey_dn11, locals.var_ey_dn14,)
    }
};
        locals.var_ey = assign62140_e96515;
        locals.var_ey_dn0 = assign62140_e96515_d_n0;
        locals.var_ey_dn2 = assign62140_e96515_d_n2;
        locals.var_ey_dn4 = assign62140_e96515_d_n4;
        locals.var_ey_dn5 = assign62140_e96515_d_n5;
        locals.var_ey_dn6 = assign62140_e96515_d_n6;
        locals.var_ey_dn7 = assign62140_e96515_d_n7;
        locals.var_ey_dn8 = assign62140_e96515_d_n8;
        locals.var_ey_dn9 = assign62140_e96515_d_n9;
        locals.var_ey_dn10 = assign62140_e96515_d_n10;
        locals.var_ey_dn11 = assign62140_e96515_d_n11;
        locals.var_ey_dn14 = assign62140_e96515_d_n14;
        locals.var_ey_rv = 0.0;

        let (assign62150_e96524, assign62150_e96524_d_n0, assign62150_e96524_d_n2, assign62150_e96524_d_n4, assign62150_e96524_d_n5, assign62150_e96524_d_n6, assign62150_e96524_d_n7, assign62150_e96524_d_n8, assign62150_e96524_d_n9, assign62150_e96524_d_n10, assign62150_e96524_d_n11, assign62150_e96524_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign62150_e96522: f64 = (1.0 / locals.var_ey);
        (assign62150_e96522, (-(locals.var_ey_dn0 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn2 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn4 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn5 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn6 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn7 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn8 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn9 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn10 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn11 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn14 / (locals.var_ey * locals.var_ey))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign62150_e96524;
        locals.var_t4_dn0 = assign62150_e96524_d_n0;
        locals.var_t4_dn2 = assign62150_e96524_d_n2;
        locals.var_t4_dn4 = assign62150_e96524_d_n4;
        locals.var_t4_dn5 = assign62150_e96524_d_n5;
        locals.var_t4_dn6 = assign62150_e96524_d_n6;
        locals.var_t4_dn7 = assign62150_e96524_d_n7;
        locals.var_t4_dn8 = assign62150_e96524_d_n8;
        locals.var_t4_dn9 = assign62150_e96524_d_n9;
        locals.var_t4_dn10 = assign62150_e96524_d_n10;
        locals.var_t4_dn11 = assign62150_e96524_d_n11;
        locals.var_t4_dn14 = assign62150_e96524_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign62160_e96533, assign62160_e96533_d_n0, assign62160_e96533_d_n2, assign62160_e96533_d_n4, assign62160_e96533_d_n5, assign62160_e96533_d_n6, assign62160_e96533_d_n7, assign62160_e96533_d_n8, assign62160_e96533_d_n9, assign62160_e96533_d_n10, assign62160_e96533_d_n11, assign62160_e96533_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign62160_e96531: f64 = (locals.var_muun * locals.var_ey);
        (assign62160_e96531, ((locals.var_muun_dn0 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn0)), ((locals.var_muun_dn2 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn2)), ((locals.var_muun_dn4 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn4)), ((locals.var_muun_dn5 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn5)), ((locals.var_muun_dn6 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn6)), ((locals.var_muun_dn7 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn7)), ((locals.var_muun_dn8 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn8)), ((locals.var_muun_dn9 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn9)), ((locals.var_muun_dn10 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn10)), ((locals.var_muun_dn11 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn11)), ((locals.var_muun_dn14 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn14)),)
    } else {
        (locals.var_em, locals.var_em_dn0, locals.var_em_dn2, locals.var_em_dn4, locals.var_em_dn5, locals.var_em_dn6, locals.var_em_dn7, locals.var_em_dn8, locals.var_em_dn9, locals.var_em_dn10, locals.var_em_dn11, locals.var_em_dn14,)
    }
};
        locals.var_em = assign62160_e96533;
        locals.var_em_dn0 = assign62160_e96533_d_n0;
        locals.var_em_dn2 = assign62160_e96533_d_n2;
        locals.var_em_dn4 = assign62160_e96533_d_n4;
        locals.var_em_dn5 = assign62160_e96533_d_n5;
        locals.var_em_dn6 = assign62160_e96533_d_n6;
        locals.var_em_dn7 = assign62160_e96533_d_n7;
        locals.var_em_dn8 = assign62160_e96533_d_n8;
        locals.var_em_dn9 = assign62160_e96533_d_n9;
        locals.var_em_dn10 = assign62160_e96533_d_n10;
        locals.var_em_dn11 = assign62160_e96533_d_n11;
        locals.var_em_dn14 = assign62160_e96533_d_n14;
        locals.var_em_rv = 0.0;

        let (assign62170_e96542, assign62170_e96542_d_n0, assign62170_e96542_d_n2, assign62170_e96542_d_n4, assign62170_e96542_d_n5, assign62170_e96542_d_n6, assign62170_e96542_d_n7, assign62170_e96542_d_n8, assign62170_e96542_d_n9, assign62170_e96542_d_n10, assign62170_e96542_d_n11, assign62170_e96542_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign62170_e96540: f64 = (locals.var_em / locals.var_vmaxe);
        (assign62170_e96540, (((locals.var_em_dn0 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn0)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn2 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn2)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn4 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn4)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn5 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn5)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn6 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn6)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn7 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn7)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn8 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn8)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn9 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn9)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn10 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn10)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn11 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn11)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn14 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn14)) / (locals.var_vmaxe * locals.var_vmaxe)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign62170_e96542;
        locals.var_t1_dn0 = assign62170_e96542_d_n0;
        locals.var_t1_dn2 = assign62170_e96542_d_n2;
        locals.var_t1_dn4 = assign62170_e96542_d_n4;
        locals.var_t1_dn5 = assign62170_e96542_d_n5;
        locals.var_t1_dn6 = assign62170_e96542_d_n6;
        locals.var_t1_dn7 = assign62170_e96542_d_n7;
        locals.var_t1_dn8 = assign62170_e96542_d_n8;
        locals.var_t1_dn9 = assign62170_e96542_d_n9;
        locals.var_t1_dn10 = assign62170_e96542_d_n10;
        locals.var_t1_dn11 = assign62170_e96542_d_n11;
        locals.var_t1_dn14 = assign62170_e96542_d_n14;
        locals.var_t1_rv = 0.0;

        let assign62180_e96546: f64 = (10.0 * 2.220446049250313e-16);
        let assign62180_e96547: f64 = (1.0 - assign62180_e96546);
        let assign62180_e96554: f64 = (10.0 * 2.220446049250313e-16);
        let assign62180_e96555: f64 = (1.0 + assign62180_e96554);
        let assign62180_e96557: f64 = if ((assign62180_e96547 <= p.p178) && (p.p178 <= assign62180_e96555)) { 1.0 } else { 0.0 };
        locals.var_guard1495 = assign62180_e96557;
        locals.var_guard1495_rv = 0.0;

        let (assign62190_e96566, assign62190_e96566_d_n0, assign62190_e96566_d_n2, assign62190_e96566_d_n4, assign62190_e96566_d_n5, assign62190_e96566_d_n6, assign62190_e96566_d_n7, assign62190_e96566_d_n8, assign62190_e96566_d_n9, assign62190_e96566_d_n10, assign62190_e96566_d_n11, assign62190_e96566_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1495 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign62190_e96566;
        locals.var_t3_dn0 = assign62190_e96566_d_n0;
        locals.var_t3_dn2 = assign62190_e96566_d_n2;
        locals.var_t3_dn4 = assign62190_e96566_d_n4;
        locals.var_t3_dn5 = assign62190_e96566_d_n5;
        locals.var_t3_dn6 = assign62190_e96566_d_n6;
        locals.var_t3_dn7 = assign62190_e96566_d_n7;
        locals.var_t3_dn8 = assign62190_e96566_d_n8;
        locals.var_t3_dn9 = assign62190_e96566_d_n9;
        locals.var_t3_dn10 = assign62190_e96566_d_n10;
        locals.var_t3_dn11 = assign62190_e96566_d_n11;
        locals.var_t3_dn14 = assign62190_e96566_d_n14;
        locals.var_t3_rv = 0.0;

        let assign62200_e96570: f64 = (10.0 * 2.220446049250313e-16);
        let assign62200_e96571: f64 = (2.0 - assign62200_e96570);
        let assign62200_e96578: f64 = (10.0 * 2.220446049250313e-16);
        let assign62200_e96579: f64 = (2.0 + assign62200_e96578);
        let assign62200_e96581: f64 = if ((assign62200_e96571 <= p.p178) && (p.p178 <= assign62200_e96579)) { 1.0 } else { 0.0 };
        locals.var_guard1496 = assign62200_e96581;
        locals.var_guard1496_rv = 0.0;

        let (assign62210_e96593, assign62210_e96593_d_n0, assign62210_e96593_d_n2, assign62210_e96593_d_n4, assign62210_e96593_d_n5, assign62210_e96593_d_n6, assign62210_e96593_d_n7, assign62210_e96593_d_n8, assign62210_e96593_d_n9, assign62210_e96593_d_n10, assign62210_e96593_d_n11, assign62210_e96593_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1495 == 0.0)) && (locals.var_guard1496 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign62210_e96593;
        locals.var_t3_dn0 = assign62210_e96593_d_n0;
        locals.var_t3_dn2 = assign62210_e96593_d_n2;
        locals.var_t3_dn4 = assign62210_e96593_d_n4;
        locals.var_t3_dn5 = assign62210_e96593_d_n5;
        locals.var_t3_dn6 = assign62210_e96593_d_n6;
        locals.var_t3_dn7 = assign62210_e96593_d_n7;
        locals.var_t3_dn8 = assign62210_e96593_d_n8;
        locals.var_t3_dn9 = assign62210_e96593_d_n9;
        locals.var_t3_dn10 = assign62210_e96593_d_n10;
        locals.var_t3_dn11 = assign62210_e96593_d_n11;
        locals.var_t3_dn14 = assign62210_e96593_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign62220_e96615, assign62220_e96615_d_n0, assign62220_e96615_d_n2, assign62220_e96615_d_n4, assign62220_e96615_d_n5, assign62220_e96615_d_n6, assign62220_e96615_d_n7, assign62220_e96615_d_n8, assign62220_e96615_d_n9, assign62220_e96615_d_n10, assign62220_e96615_d_n11, assign62220_e96615_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1495 == 0.0)) && (locals.var_guard1496 == 0.0)) {
        let (assign62220_e96613, assign62220_e96613_d_n0, assign62220_e96613_d_n2, assign62220_e96613_d_n4, assign62220_e96613_d_n5, assign62220_e96613_d_n6, assign62220_e96613_d_n7, assign62220_e96613_d_n8, assign62220_e96613_d_n9, assign62220_e96613_d_n10, assign62220_e96613_d_n11, assign62220_e96613_d_n14,) = {
            if (locals.var_t1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign62220_e96611: f64 = (p.p178 - 1.0);
                let assign62220_e96612: f64 = (locals.var_t1).powf(assign62220_e96611);
                (assign62220_e96612, if 0.0 == 0.0 && ((assign62220_e96611) as f64).is_finite() && ((assign62220_e96611) as f64).fract() == 0.0 { if assign62220_e96611 == 0.0 { 0.0 } else { (assign62220_e96611 * ((locals.var_t1).powf(assign62220_e96611 - 1.0) * locals.var_t1_dn0)) } } else { (assign62220_e96612 * (assign62220_e96611 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign62220_e96611) as f64).is_finite() && ((assign62220_e96611) as f64).fract() == 0.0 { if assign62220_e96611 == 0.0 { 0.0 } else { (assign62220_e96611 * ((locals.var_t1).powf(assign62220_e96611 - 1.0) * locals.var_t1_dn2)) } } else { (assign62220_e96612 * (assign62220_e96611 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign62220_e96611) as f64).is_finite() && ((assign62220_e96611) as f64).fract() == 0.0 { if assign62220_e96611 == 0.0 { 0.0 } else { (assign62220_e96611 * ((locals.var_t1).powf(assign62220_e96611 - 1.0) * locals.var_t1_dn4)) } } else { (assign62220_e96612 * (assign62220_e96611 * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign62220_e96611) as f64).is_finite() && ((assign62220_e96611) as f64).fract() == 0.0 { if assign62220_e96611 == 0.0 { 0.0 } else { (assign62220_e96611 * ((locals.var_t1).powf(assign62220_e96611 - 1.0) * locals.var_t1_dn5)) } } else { (assign62220_e96612 * (assign62220_e96611 * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign62220_e96611) as f64).is_finite() && ((assign62220_e96611) as f64).fract() == 0.0 { if assign62220_e96611 == 0.0 { 0.0 } else { (assign62220_e96611 * ((locals.var_t1).powf(assign62220_e96611 - 1.0) * locals.var_t1_dn6)) } } else { (assign62220_e96612 * (assign62220_e96611 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign62220_e96611) as f64).is_finite() && ((assign62220_e96611) as f64).fract() == 0.0 { if assign62220_e96611 == 0.0 { 0.0 } else { (assign62220_e96611 * ((locals.var_t1).powf(assign62220_e96611 - 1.0) * locals.var_t1_dn7)) } } else { (assign62220_e96612 * (assign62220_e96611 * (locals.var_t1_dn7 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign62220_e96611) as f64).is_finite() && ((assign62220_e96611) as f64).fract() == 0.0 { if assign62220_e96611 == 0.0 { 0.0 } else { (assign62220_e96611 * ((locals.var_t1).powf(assign62220_e96611 - 1.0) * locals.var_t1_dn8)) } } else { (assign62220_e96612 * (assign62220_e96611 * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign62220_e96611) as f64).is_finite() && ((assign62220_e96611) as f64).fract() == 0.0 { if assign62220_e96611 == 0.0 { 0.0 } else { (assign62220_e96611 * ((locals.var_t1).powf(assign62220_e96611 - 1.0) * locals.var_t1_dn9)) } } else { (assign62220_e96612 * (assign62220_e96611 * (locals.var_t1_dn9 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign62220_e96611) as f64).is_finite() && ((assign62220_e96611) as f64).fract() == 0.0 { if assign62220_e96611 == 0.0 { 0.0 } else { (assign62220_e96611 * ((locals.var_t1).powf(assign62220_e96611 - 1.0) * locals.var_t1_dn10)) } } else { (assign62220_e96612 * (assign62220_e96611 * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign62220_e96611) as f64).is_finite() && ((assign62220_e96611) as f64).fract() == 0.0 { if assign62220_e96611 == 0.0 { 0.0 } else { (assign62220_e96611 * ((locals.var_t1).powf(assign62220_e96611 - 1.0) * locals.var_t1_dn11)) } } else { (assign62220_e96612 * (assign62220_e96611 * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign62220_e96611) as f64).is_finite() && ((assign62220_e96611) as f64).fract() == 0.0 { if assign62220_e96611 == 0.0 { 0.0 } else { (assign62220_e96611 * ((locals.var_t1).powf(assign62220_e96611 - 1.0) * locals.var_t1_dn14)) } } else { (assign62220_e96612 * (assign62220_e96611 * (locals.var_t1_dn14 / locals.var_t1))) },)
            }
        };
        (assign62220_e96613, assign62220_e96613_d_n0, assign62220_e96613_d_n2, assign62220_e96613_d_n4, assign62220_e96613_d_n5, assign62220_e96613_d_n6, assign62220_e96613_d_n7, assign62220_e96613_d_n8, assign62220_e96613_d_n9, assign62220_e96613_d_n10, assign62220_e96613_d_n11, assign62220_e96613_d_n14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign62220_e96615;
        locals.var_t3_dn0 = assign62220_e96615_d_n0;
        locals.var_t3_dn2 = assign62220_e96615_d_n2;
        locals.var_t3_dn4 = assign62220_e96615_d_n4;
        locals.var_t3_dn5 = assign62220_e96615_d_n5;
        locals.var_t3_dn6 = assign62220_e96615_d_n6;
        locals.var_t3_dn7 = assign62220_e96615_d_n7;
        locals.var_t3_dn8 = assign62220_e96615_d_n8;
        locals.var_t3_dn9 = assign62220_e96615_d_n9;
        locals.var_t3_dn10 = assign62220_e96615_d_n10;
        locals.var_t3_dn11 = assign62220_e96615_d_n11;
        locals.var_t3_dn14 = assign62220_e96615_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign62230_e96624, assign62230_e96624_d_n0, assign62230_e96624_d_n2, assign62230_e96624_d_n4, assign62230_e96624_d_n5, assign62230_e96624_d_n6, assign62230_e96624_d_n7, assign62230_e96624_d_n8, assign62230_e96624_d_n9, assign62230_e96624_d_n10, assign62230_e96624_d_n11, assign62230_e96624_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign62230_e96622: f64 = (locals.var_t1 * locals.var_t3);
        (assign62230_e96622, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)), ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)), ((locals.var_t1_dn9 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn9)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)), ((locals.var_t1_dn14 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign62230_e96624;
        locals.var_t2_dn0 = assign62230_e96624_d_n0;
        locals.var_t2_dn2 = assign62230_e96624_d_n2;
        locals.var_t2_dn4 = assign62230_e96624_d_n4;
        locals.var_t2_dn5 = assign62230_e96624_d_n5;
        locals.var_t2_dn6 = assign62230_e96624_d_n6;
        locals.var_t2_dn7 = assign62230_e96624_d_n7;
        locals.var_t2_dn8 = assign62230_e96624_d_n8;
        locals.var_t2_dn9 = assign62230_e96624_d_n9;
        locals.var_t2_dn10 = assign62230_e96624_d_n10;
        locals.var_t2_dn11 = assign62230_e96624_d_n11;
        locals.var_t2_dn14 = assign62230_e96624_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign62240_e96633, assign62240_e96633_d_n0, assign62240_e96633_d_n2, assign62240_e96633_d_n4, assign62240_e96633_d_n5, assign62240_e96633_d_n6, assign62240_e96633_d_n7, assign62240_e96633_d_n8, assign62240_e96633_d_n9, assign62240_e96633_d_n10, assign62240_e96633_d_n11, assign62240_e96633_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign62240_e96631: f64 = (1.0 + locals.var_t2);
        (assign62240_e96631, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign62240_e96633;
        locals.var_t4_dn0 = assign62240_e96633_d_n0;
        locals.var_t4_dn2 = assign62240_e96633_d_n2;
        locals.var_t4_dn4 = assign62240_e96633_d_n4;
        locals.var_t4_dn5 = assign62240_e96633_d_n5;
        locals.var_t4_dn6 = assign62240_e96633_d_n6;
        locals.var_t4_dn7 = assign62240_e96633_d_n7;
        locals.var_t4_dn8 = assign62240_e96633_d_n8;
        locals.var_t4_dn9 = assign62240_e96633_d_n9;
        locals.var_t4_dn10 = assign62240_e96633_d_n10;
        locals.var_t4_dn11 = assign62240_e96633_d_n11;
        locals.var_t4_dn14 = assign62240_e96633_d_n14;
        locals.var_t4_rv = 0.0;

        let assign62250_e96637: f64 = (10.0 * 2.220446049250313e-16);
        let assign62250_e96638: f64 = (1.0 - assign62250_e96637);
        let assign62250_e96645: f64 = (10.0 * 2.220446049250313e-16);
        let assign62250_e96646: f64 = (1.0 + assign62250_e96645);
        let assign62250_e96648: f64 = if ((assign62250_e96638 <= p.p178) && (p.p178 <= assign62250_e96646)) { 1.0 } else { 0.0 };
        locals.var_guard1497 = assign62250_e96648;
        locals.var_guard1497_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_232(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign62260_e96659, assign62260_e96659_d_n0, assign62260_e96659_d_n2, assign62260_e96659_d_n4, assign62260_e96659_d_n5, assign62260_e96659_d_n6, assign62260_e96659_d_n7, assign62260_e96659_d_n8, assign62260_e96659_d_n9, assign62260_e96659_d_n10, assign62260_e96659_d_n11, assign62260_e96659_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1497 != 0.0)) {
        let assign62260_e96657: f64 = (1.0 / locals.var_t4);
        (assign62260_e96657, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn14 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign62260_e96659;
        locals.var_t5_dn0 = assign62260_e96659_d_n0;
        locals.var_t5_dn2 = assign62260_e96659_d_n2;
        locals.var_t5_dn4 = assign62260_e96659_d_n4;
        locals.var_t5_dn5 = assign62260_e96659_d_n5;
        locals.var_t5_dn6 = assign62260_e96659_d_n6;
        locals.var_t5_dn7 = assign62260_e96659_d_n7;
        locals.var_t5_dn8 = assign62260_e96659_d_n8;
        locals.var_t5_dn9 = assign62260_e96659_d_n9;
        locals.var_t5_dn10 = assign62260_e96659_d_n10;
        locals.var_t5_dn11 = assign62260_e96659_d_n11;
        locals.var_t5_dn14 = assign62260_e96659_d_n14;
        locals.var_t5_rv = 0.0;

        let assign62270_e96663: f64 = (10.0 * 2.220446049250313e-16);
        let assign62270_e96664: f64 = (2.0 - assign62270_e96663);
        let assign62270_e96671: f64 = (10.0 * 2.220446049250313e-16);
        let assign62270_e96672: f64 = (2.0 + assign62270_e96671);
        let assign62270_e96674: f64 = if ((assign62270_e96664 <= p.p178) && (p.p178 <= assign62270_e96672)) { 1.0 } else { 0.0 };
        locals.var_guard1498 = assign62270_e96674;
        locals.var_guard1498_rv = 0.0;

        let (assign62280_e96689, assign62280_e96689_d_n0, assign62280_e96689_d_n2, assign62280_e96689_d_n4, assign62280_e96689_d_n5, assign62280_e96689_d_n6, assign62280_e96689_d_n7, assign62280_e96689_d_n8, assign62280_e96689_d_n9, assign62280_e96689_d_n10, assign62280_e96689_d_n11, assign62280_e96689_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1497 == 0.0)) && (locals.var_guard1498 != 0.0)) {
        let assign62280_e96686: f64 = (locals.var_t4).sqrt();
        let assign62280_e96687: f64 = (1.0 / assign62280_e96686);
        (assign62280_e96687, (-((locals.var_t4_dn0 / (2.0 * assign62280_e96686)) / (assign62280_e96686 * assign62280_e96686))), (-((locals.var_t4_dn2 / (2.0 * assign62280_e96686)) / (assign62280_e96686 * assign62280_e96686))), (-((locals.var_t4_dn4 / (2.0 * assign62280_e96686)) / (assign62280_e96686 * assign62280_e96686))), (-((locals.var_t4_dn5 / (2.0 * assign62280_e96686)) / (assign62280_e96686 * assign62280_e96686))), (-((locals.var_t4_dn6 / (2.0 * assign62280_e96686)) / (assign62280_e96686 * assign62280_e96686))), (-((locals.var_t4_dn7 / (2.0 * assign62280_e96686)) / (assign62280_e96686 * assign62280_e96686))), (-((locals.var_t4_dn8 / (2.0 * assign62280_e96686)) / (assign62280_e96686 * assign62280_e96686))), (-((locals.var_t4_dn9 / (2.0 * assign62280_e96686)) / (assign62280_e96686 * assign62280_e96686))), (-((locals.var_t4_dn10 / (2.0 * assign62280_e96686)) / (assign62280_e96686 * assign62280_e96686))), (-((locals.var_t4_dn11 / (2.0 * assign62280_e96686)) / (assign62280_e96686 * assign62280_e96686))), (-((locals.var_t4_dn14 / (2.0 * assign62280_e96686)) / (assign62280_e96686 * assign62280_e96686))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign62280_e96689;
        locals.var_t5_dn0 = assign62280_e96689_d_n0;
        locals.var_t5_dn2 = assign62280_e96689_d_n2;
        locals.var_t5_dn4 = assign62280_e96689_d_n4;
        locals.var_t5_dn5 = assign62280_e96689_d_n5;
        locals.var_t5_dn6 = assign62280_e96689_d_n6;
        locals.var_t5_dn7 = assign62280_e96689_d_n7;
        locals.var_t5_dn8 = assign62280_e96689_d_n8;
        locals.var_t5_dn9 = assign62280_e96689_d_n9;
        locals.var_t5_dn10 = assign62280_e96689_d_n10;
        locals.var_t5_dn11 = assign62280_e96689_d_n11;
        locals.var_t5_dn14 = assign62280_e96689_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign62290_e96714, assign62290_e96714_d_n0, assign62290_e96714_d_n2, assign62290_e96714_d_n4, assign62290_e96714_d_n5, assign62290_e96714_d_n6, assign62290_e96714_d_n7, assign62290_e96714_d_n8, assign62290_e96714_d_n9, assign62290_e96714_d_n10, assign62290_e96714_d_n11, assign62290_e96714_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1497 == 0.0)) && (locals.var_guard1498 == 0.0)) {
        let (assign62290_e96712, assign62290_e96712_d_n0, assign62290_e96712_d_n2, assign62290_e96712_d_n4, assign62290_e96712_d_n5, assign62290_e96712_d_n6, assign62290_e96712_d_n7, assign62290_e96712_d_n8, assign62290_e96712_d_n9, assign62290_e96712_d_n10, assign62290_e96712_d_n11, assign62290_e96712_d_n14,) = {
            if (locals.var_t4 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign62290_e96706: f64 = (-1.0);
                let assign62290_e96708: f64 = (assign62290_e96706 / p.p178);
                let assign62290_e96710: f64 = (assign62290_e96708 - 1.0);
                let assign62290_e96711: f64 = (locals.var_t4).powf(assign62290_e96710);
                (assign62290_e96711, if 0.0 == 0.0 && ((assign62290_e96710) as f64).is_finite() && ((assign62290_e96710) as f64).fract() == 0.0 { if assign62290_e96710 == 0.0 { 0.0 } else { (assign62290_e96710 * ((locals.var_t4).powf(assign62290_e96710 - 1.0) * locals.var_t4_dn0)) } } else { (assign62290_e96711 * (assign62290_e96710 * (locals.var_t4_dn0 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign62290_e96710) as f64).is_finite() && ((assign62290_e96710) as f64).fract() == 0.0 { if assign62290_e96710 == 0.0 { 0.0 } else { (assign62290_e96710 * ((locals.var_t4).powf(assign62290_e96710 - 1.0) * locals.var_t4_dn2)) } } else { (assign62290_e96711 * (assign62290_e96710 * (locals.var_t4_dn2 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign62290_e96710) as f64).is_finite() && ((assign62290_e96710) as f64).fract() == 0.0 { if assign62290_e96710 == 0.0 { 0.0 } else { (assign62290_e96710 * ((locals.var_t4).powf(assign62290_e96710 - 1.0) * locals.var_t4_dn4)) } } else { (assign62290_e96711 * (assign62290_e96710 * (locals.var_t4_dn4 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign62290_e96710) as f64).is_finite() && ((assign62290_e96710) as f64).fract() == 0.0 { if assign62290_e96710 == 0.0 { 0.0 } else { (assign62290_e96710 * ((locals.var_t4).powf(assign62290_e96710 - 1.0) * locals.var_t4_dn5)) } } else { (assign62290_e96711 * (assign62290_e96710 * (locals.var_t4_dn5 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign62290_e96710) as f64).is_finite() && ((assign62290_e96710) as f64).fract() == 0.0 { if assign62290_e96710 == 0.0 { 0.0 } else { (assign62290_e96710 * ((locals.var_t4).powf(assign62290_e96710 - 1.0) * locals.var_t4_dn6)) } } else { (assign62290_e96711 * (assign62290_e96710 * (locals.var_t4_dn6 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign62290_e96710) as f64).is_finite() && ((assign62290_e96710) as f64).fract() == 0.0 { if assign62290_e96710 == 0.0 { 0.0 } else { (assign62290_e96710 * ((locals.var_t4).powf(assign62290_e96710 - 1.0) * locals.var_t4_dn7)) } } else { (assign62290_e96711 * (assign62290_e96710 * (locals.var_t4_dn7 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign62290_e96710) as f64).is_finite() && ((assign62290_e96710) as f64).fract() == 0.0 { if assign62290_e96710 == 0.0 { 0.0 } else { (assign62290_e96710 * ((locals.var_t4).powf(assign62290_e96710 - 1.0) * locals.var_t4_dn8)) } } else { (assign62290_e96711 * (assign62290_e96710 * (locals.var_t4_dn8 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign62290_e96710) as f64).is_finite() && ((assign62290_e96710) as f64).fract() == 0.0 { if assign62290_e96710 == 0.0 { 0.0 } else { (assign62290_e96710 * ((locals.var_t4).powf(assign62290_e96710 - 1.0) * locals.var_t4_dn9)) } } else { (assign62290_e96711 * (assign62290_e96710 * (locals.var_t4_dn9 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign62290_e96710) as f64).is_finite() && ((assign62290_e96710) as f64).fract() == 0.0 { if assign62290_e96710 == 0.0 { 0.0 } else { (assign62290_e96710 * ((locals.var_t4).powf(assign62290_e96710 - 1.0) * locals.var_t4_dn10)) } } else { (assign62290_e96711 * (assign62290_e96710 * (locals.var_t4_dn10 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign62290_e96710) as f64).is_finite() && ((assign62290_e96710) as f64).fract() == 0.0 { if assign62290_e96710 == 0.0 { 0.0 } else { (assign62290_e96710 * ((locals.var_t4).powf(assign62290_e96710 - 1.0) * locals.var_t4_dn11)) } } else { (assign62290_e96711 * (assign62290_e96710 * (locals.var_t4_dn11 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign62290_e96710) as f64).is_finite() && ((assign62290_e96710) as f64).fract() == 0.0 { if assign62290_e96710 == 0.0 { 0.0 } else { (assign62290_e96710 * ((locals.var_t4).powf(assign62290_e96710 - 1.0) * locals.var_t4_dn14)) } } else { (assign62290_e96711 * (assign62290_e96710 * (locals.var_t4_dn14 / locals.var_t4))) },)
            }
        };
        (assign62290_e96712, assign62290_e96712_d_n0, assign62290_e96712_d_n2, assign62290_e96712_d_n4, assign62290_e96712_d_n5, assign62290_e96712_d_n6, assign62290_e96712_d_n7, assign62290_e96712_d_n8, assign62290_e96712_d_n9, assign62290_e96712_d_n10, assign62290_e96712_d_n11, assign62290_e96712_d_n14,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign62290_e96714;
        locals.var_t6_dn0 = assign62290_e96714_d_n0;
        locals.var_t6_dn2 = assign62290_e96714_d_n2;
        locals.var_t6_dn4 = assign62290_e96714_d_n4;
        locals.var_t6_dn5 = assign62290_e96714_d_n5;
        locals.var_t6_dn6 = assign62290_e96714_d_n6;
        locals.var_t6_dn7 = assign62290_e96714_d_n7;
        locals.var_t6_dn8 = assign62290_e96714_d_n8;
        locals.var_t6_dn9 = assign62290_e96714_d_n9;
        locals.var_t6_dn10 = assign62290_e96714_d_n10;
        locals.var_t6_dn11 = assign62290_e96714_d_n11;
        locals.var_t6_dn14 = assign62290_e96714_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign62300_e96729, assign62300_e96729_d_n0, assign62300_e96729_d_n2, assign62300_e96729_d_n4, assign62300_e96729_d_n5, assign62300_e96729_d_n6, assign62300_e96729_d_n7, assign62300_e96729_d_n8, assign62300_e96729_d_n9, assign62300_e96729_d_n10, assign62300_e96729_d_n11, assign62300_e96729_d_n14,) = {
    if ((((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1497 == 0.0)) && (locals.var_guard1498 == 0.0)) {
        let assign62300_e96727: f64 = (locals.var_t4 * locals.var_t6);
        (assign62300_e96727, ((locals.var_t4_dn0 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn0)), ((locals.var_t4_dn2 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn2)), ((locals.var_t4_dn4 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn4)), ((locals.var_t4_dn5 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn5)), ((locals.var_t4_dn6 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn6)), ((locals.var_t4_dn7 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn7)), ((locals.var_t4_dn8 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn8)), ((locals.var_t4_dn9 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn9)), ((locals.var_t4_dn10 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn10)), ((locals.var_t4_dn11 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn11)), ((locals.var_t4_dn14 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign62300_e96729;
        locals.var_t5_dn0 = assign62300_e96729_d_n0;
        locals.var_t5_dn2 = assign62300_e96729_d_n2;
        locals.var_t5_dn4 = assign62300_e96729_d_n4;
        locals.var_t5_dn5 = assign62300_e96729_d_n5;
        locals.var_t5_dn6 = assign62300_e96729_d_n6;
        locals.var_t5_dn7 = assign62300_e96729_d_n7;
        locals.var_t5_dn8 = assign62300_e96729_d_n8;
        locals.var_t5_dn9 = assign62300_e96729_d_n9;
        locals.var_t5_dn10 = assign62300_e96729_d_n10;
        locals.var_t5_dn11 = assign62300_e96729_d_n11;
        locals.var_t5_dn14 = assign62300_e96729_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign62310_e96738, assign62310_e96738_d_n0, assign62310_e96738_d_n2, assign62310_e96738_d_n4, assign62310_e96738_d_n5, assign62310_e96738_d_n6, assign62310_e96738_d_n7, assign62310_e96738_d_n8, assign62310_e96738_d_n9, assign62310_e96738_d_n10, assign62310_e96738_d_n11, assign62310_e96738_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign62310_e96736: f64 = (locals.var_muun * locals.var_t5);
        (assign62310_e96736, ((locals.var_muun_dn0 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn0)), ((locals.var_muun_dn2 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn2)), ((locals.var_muun_dn4 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn4)), ((locals.var_muun_dn5 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn5)), ((locals.var_muun_dn6 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn6)), ((locals.var_muun_dn7 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn7)), ((locals.var_muun_dn8 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn8)), ((locals.var_muun_dn9 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn9)), ((locals.var_muun_dn10 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn10)), ((locals.var_muun_dn11 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn11)), ((locals.var_muun_dn14 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn14)),)
    } else {
        (locals.var_mu, locals.var_mu_dn0, locals.var_mu_dn2, locals.var_mu_dn4, locals.var_mu_dn5, locals.var_mu_dn6, locals.var_mu_dn7, locals.var_mu_dn8, locals.var_mu_dn9, locals.var_mu_dn10, locals.var_mu_dn11, locals.var_mu_dn14,)
    }
};
        locals.var_mu = assign62310_e96738;
        locals.var_mu_dn0 = assign62310_e96738_d_n0;
        locals.var_mu_dn2 = assign62310_e96738_d_n2;
        locals.var_mu_dn4 = assign62310_e96738_d_n4;
        locals.var_mu_dn5 = assign62310_e96738_d_n5;
        locals.var_mu_dn6 = assign62310_e96738_d_n6;
        locals.var_mu_dn7 = assign62310_e96738_d_n7;
        locals.var_mu_dn8 = assign62310_e96738_d_n8;
        locals.var_mu_dn9 = assign62310_e96738_d_n9;
        locals.var_mu_dn10 = assign62310_e96738_d_n10;
        locals.var_mu_dn11 = assign62310_e96738_d_n11;
        locals.var_mu_dn14 = assign62310_e96738_d_n14;
        locals.var_mu_rv = 0.0;

        let (assign62320_e96749, assign62320_e96749_d_n0, assign62320_e96749_d_n2, assign62320_e96749_d_n4, assign62320_e96749_d_n5, assign62320_e96749_d_n6, assign62320_e96749_d_n7, assign62320_e96749_d_n8, assign62320_e96749_d_n9, assign62320_e96749_d_n10, assign62320_e96749_d_n11, assign62320_e96749_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign62320_e96745: f64 = (locals.var_weff_nf * locals.var_beta_inv);
        let assign62320_e96747: f64 = (assign62320_e96745 / locals.var_lch);
        (assign62320_e96747, ((((locals.var_weff_nf * locals.var_beta_inv_dn0) * locals.var_lch) - (assign62320_e96745 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn2) * locals.var_lch) - (assign62320_e96745 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn4) * locals.var_lch) - (assign62320_e96745 * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn5) * locals.var_lch) - (assign62320_e96745 * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn6) * locals.var_lch) - (assign62320_e96745 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn7) * locals.var_lch) - (assign62320_e96745 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn8) * locals.var_lch) - (assign62320_e96745 * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn9) * locals.var_lch) - (assign62320_e96745 * locals.var_lch_dn9)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn10) * locals.var_lch) - (assign62320_e96745 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn11) * locals.var_lch) - (assign62320_e96745 * locals.var_lch_dn11)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn14) * locals.var_lch) - (assign62320_e96745 * locals.var_lch_dn14)) / (locals.var_lch * locals.var_lch)),)
    } else {
        (locals.var_betawl, locals.var_betawl_dn0, locals.var_betawl_dn2, locals.var_betawl_dn4, locals.var_betawl_dn5, locals.var_betawl_dn6, locals.var_betawl_dn7, locals.var_betawl_dn8, locals.var_betawl_dn9, locals.var_betawl_dn10, locals.var_betawl_dn11, locals.var_betawl_dn14,)
    }
};
        locals.var_betawl = assign62320_e96749;
        locals.var_betawl_dn0 = assign62320_e96749_d_n0;
        locals.var_betawl_dn2 = assign62320_e96749_d_n2;
        locals.var_betawl_dn4 = assign62320_e96749_d_n4;
        locals.var_betawl_dn5 = assign62320_e96749_d_n5;
        locals.var_betawl_dn6 = assign62320_e96749_d_n6;
        locals.var_betawl_dn7 = assign62320_e96749_d_n7;
        locals.var_betawl_dn8 = assign62320_e96749_d_n8;
        locals.var_betawl_dn9 = assign62320_e96749_d_n9;
        locals.var_betawl_dn10 = assign62320_e96749_d_n10;
        locals.var_betawl_dn11 = assign62320_e96749_d_n11;
        locals.var_betawl_dn14 = assign62320_e96749_d_n14;
        locals.var_betawl_rv = 0.0;

        let (assign62330_e96759, assign62330_e96759_d_n0, assign62330_e96759_d_n2, assign62330_e96759_d_n4, assign62330_e96759_d_n5, assign62330_e96759_d_n6, assign62330_e96759_d_n7, assign62330_e96759_d_n8, assign62330_e96759_d_n9, assign62330_e96759_d_n10, assign62330_e96759_d_n11, assign62330_e96759_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign62330_e96755: f64 = (-locals.var_betawl);
        let assign62330_e96757: f64 = (assign62330_e96755 / locals.var_lch);
        (assign62330_e96757, ((((-locals.var_betawl_dn0) * locals.var_lch) - (assign62330_e96755 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)), ((((-locals.var_betawl_dn2) * locals.var_lch) - (assign62330_e96755 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)), ((((-locals.var_betawl_dn4) * locals.var_lch) - (assign62330_e96755 * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)), ((((-locals.var_betawl_dn5) * locals.var_lch) - (assign62330_e96755 * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)), ((((-locals.var_betawl_dn6) * locals.var_lch) - (assign62330_e96755 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)), ((((-locals.var_betawl_dn7) * locals.var_lch) - (assign62330_e96755 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)), ((((-locals.var_betawl_dn8) * locals.var_lch) - (assign62330_e96755 * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)), ((((-locals.var_betawl_dn9) * locals.var_lch) - (assign62330_e96755 * locals.var_lch_dn9)) / (locals.var_lch * locals.var_lch)), ((((-locals.var_betawl_dn10) * locals.var_lch) - (assign62330_e96755 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)), ((((-locals.var_betawl_dn11) * locals.var_lch) - (assign62330_e96755 * locals.var_lch_dn11)) / (locals.var_lch * locals.var_lch)), ((((-locals.var_betawl_dn14) * locals.var_lch) - (assign62330_e96755 * locals.var_lch_dn14)) / (locals.var_lch * locals.var_lch)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign62330_e96759;
        locals.var_t1_dn0 = assign62330_e96759_d_n0;
        locals.var_t1_dn2 = assign62330_e96759_d_n2;
        locals.var_t1_dn4 = assign62330_e96759_d_n4;
        locals.var_t1_dn5 = assign62330_e96759_d_n5;
        locals.var_t1_dn6 = assign62330_e96759_d_n6;
        locals.var_t1_dn7 = assign62330_e96759_d_n7;
        locals.var_t1_dn8 = assign62330_e96759_d_n8;
        locals.var_t1_dn9 = assign62330_e96759_d_n9;
        locals.var_t1_dn10 = assign62330_e96759_d_n10;
        locals.var_t1_dn11 = assign62330_e96759_d_n11;
        locals.var_t1_dn14 = assign62330_e96759_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign62340_e96770, assign62340_e96770_d_n0, assign62340_e96770_d_n2, assign62340_e96770_d_n4, assign62340_e96770_d_n5, assign62340_e96770_d_n6, assign62340_e96770_d_n7, assign62340_e96770_d_n8, assign62340_e96770_d_n9, assign62340_e96770_d_n10, assign62340_e96770_d_n11, assign62340_e96770_d_n14,) = {
    if ((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) {
        let assign62340_e96766: f64 = (locals.var_betawl * locals.var_idd);
        let assign62340_e96768: f64 = (assign62340_e96766 * locals.var_mu);
        (assign62340_e96768, ((((locals.var_betawl_dn0 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn0)) * locals.var_mu) + (assign62340_e96766 * locals.var_mu_dn0)), ((((locals.var_betawl_dn2 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn2)) * locals.var_mu) + (assign62340_e96766 * locals.var_mu_dn2)), ((((locals.var_betawl_dn4 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn4)) * locals.var_mu) + (assign62340_e96766 * locals.var_mu_dn4)), ((((locals.var_betawl_dn5 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn5)) * locals.var_mu) + (assign62340_e96766 * locals.var_mu_dn5)), ((((locals.var_betawl_dn6 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn6)) * locals.var_mu) + (assign62340_e96766 * locals.var_mu_dn6)), ((((locals.var_betawl_dn7 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn7)) * locals.var_mu) + (assign62340_e96766 * locals.var_mu_dn7)), ((((locals.var_betawl_dn8 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn8)) * locals.var_mu) + (assign62340_e96766 * locals.var_mu_dn8)), ((((locals.var_betawl_dn9 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn9)) * locals.var_mu) + (assign62340_e96766 * locals.var_mu_dn9)), ((((locals.var_betawl_dn10 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn10)) * locals.var_mu) + (assign62340_e96766 * locals.var_mu_dn10)), ((((locals.var_betawl_dn11 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn11)) * locals.var_mu) + (assign62340_e96766 * locals.var_mu_dn11)), ((((locals.var_betawl_dn14 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn14)) * locals.var_mu) + (assign62340_e96766 * locals.var_mu_dn14)),)
    } else {
        (locals.var_ids0, locals.var_ids0_dn0, locals.var_ids0_dn2, locals.var_ids0_dn4, locals.var_ids0_dn5, locals.var_ids0_dn6, locals.var_ids0_dn7, locals.var_ids0_dn8, locals.var_ids0_dn9, locals.var_ids0_dn10, locals.var_ids0_dn11, locals.var_ids0_dn14,)
    }
};
        locals.var_ids0 = assign62340_e96770;
        locals.var_ids0_dn0 = assign62340_e96770_d_n0;
        locals.var_ids0_dn2 = assign62340_e96770_d_n2;
        locals.var_ids0_dn4 = assign62340_e96770_d_n4;
        locals.var_ids0_dn5 = assign62340_e96770_d_n5;
        locals.var_ids0_dn6 = assign62340_e96770_d_n6;
        locals.var_ids0_dn7 = assign62340_e96770_d_n7;
        locals.var_ids0_dn8 = assign62340_e96770_d_n8;
        locals.var_ids0_dn9 = assign62340_e96770_d_n9;
        locals.var_ids0_dn10 = assign62340_e96770_d_n10;
        locals.var_ids0_dn11 = assign62340_e96770_d_n11;
        locals.var_ids0_dn14 = assign62340_e96770_d_n14;
        locals.var_ids0_rv = 0.0;

        let assign62350_e96773: f64 = if p.p283 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1499 = assign62350_e96773;
        locals.var_guard1499_rv = 0.0;

        let (assign62360_e96786, assign62360_e96786_d_n0, assign62360_e96786_d_n2, assign62360_e96786_d_n4, assign62360_e96786_d_n5, assign62360_e96786_d_n6, assign62360_e96786_d_n7, assign62360_e96786_d_n8, assign62360_e96786_d_n9, assign62360_e96786_d_n10, assign62360_e96786_d_n11, assign62360_e96786_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign62360_e96783: f64 = (locals.var_vds - locals.var_pds);
        let assign62360_e96784: f64 = (0.5 * assign62360_e96783);
        (assign62360_e96784, (0.5 * (locals.var_vds_dn0 - locals.var_pds_dn0)), (0.5 * (locals.var_vds_dn2 - locals.var_pds_dn2)), (0.5 * (locals.var_vds_dn4 - locals.var_pds_dn4)), (0.5 * (locals.var_vds_dn5 - locals.var_pds_dn5)), (0.5 * (locals.var_vds_dn6 - locals.var_pds_dn6)), (0.5 * (locals.var_vds_dn7 - locals.var_pds_dn7)), (0.5 * (locals.var_vds_dn8 - locals.var_pds_dn8)), (0.5 * (locals.var_vds_dn9 - locals.var_pds_dn9)), (0.5 * (locals.var_vds_dn10 - locals.var_pds_dn10)), (0.5 * (locals.var_vds_dn11 - locals.var_pds_dn11)), (0.5 * (locals.var_vds_dn14 - locals.var_pds_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign62360_e96786;
        locals.var_t1_dn0 = assign62360_e96786_d_n0;
        locals.var_t1_dn2 = assign62360_e96786_d_n2;
        locals.var_t1_dn4 = assign62360_e96786_d_n4;
        locals.var_t1_dn5 = assign62360_e96786_d_n5;
        locals.var_t1_dn6 = assign62360_e96786_d_n6;
        locals.var_t1_dn7 = assign62360_e96786_d_n7;
        locals.var_t1_dn8 = assign62360_e96786_d_n8;
        locals.var_t1_dn9 = assign62360_e96786_d_n9;
        locals.var_t1_dn10 = assign62360_e96786_d_n10;
        locals.var_t1_dn11 = assign62360_e96786_d_n11;
        locals.var_t1_dn14 = assign62360_e96786_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign62370_e96799, assign62370_e96799_d_n0, assign62370_e96799_d_n2, assign62370_e96799_d_n4, assign62370_e96799_d_n5, assign62370_e96799_d_n6, assign62370_e96799_d_n7, assign62370_e96799_d_n8, assign62370_e96799_d_n9, assign62370_e96799_d_n10, assign62370_e96799_d_n11, assign62370_e96799_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign62370_e96795: f64 = (2.0 * locals.var_t1);
        let assign62370_e96797: f64 = (assign62370_e96795 / 0.01);
        (assign62370_e96797, ((2.0 * locals.var_t1_dn0) / 0.01), ((2.0 * locals.var_t1_dn2) / 0.01), ((2.0 * locals.var_t1_dn4) / 0.01), ((2.0 * locals.var_t1_dn5) / 0.01), ((2.0 * locals.var_t1_dn6) / 0.01), ((2.0 * locals.var_t1_dn7) / 0.01), ((2.0 * locals.var_t1_dn8) / 0.01), ((2.0 * locals.var_t1_dn9) / 0.01), ((2.0 * locals.var_t1_dn10) / 0.01), ((2.0 * locals.var_t1_dn11) / 0.01), ((2.0 * locals.var_t1_dn14) / 0.01),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign62370_e96799;
        locals.var_tmf1_dn0 = assign62370_e96799_d_n0;
        locals.var_tmf1_dn2 = assign62370_e96799_d_n2;
        locals.var_tmf1_dn4 = assign62370_e96799_d_n4;
        locals.var_tmf1_dn5 = assign62370_e96799_d_n5;
        locals.var_tmf1_dn6 = assign62370_e96799_d_n6;
        locals.var_tmf1_dn7 = assign62370_e96799_d_n7;
        locals.var_tmf1_dn8 = assign62370_e96799_d_n8;
        locals.var_tmf1_dn9 = assign62370_e96799_d_n9;
        locals.var_tmf1_dn10 = assign62370_e96799_d_n10;
        locals.var_tmf1_dn11 = assign62370_e96799_d_n11;
        locals.var_tmf1_dn14 = assign62370_e96799_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign62380_e96844, assign62380_e96844_d_n0, assign62380_e96844_d_n2, assign62380_e96844_d_n4, assign62380_e96844_d_n5, assign62380_e96844_d_n6, assign62380_e96844_d_n7, assign62380_e96844_d_n8, assign62380_e96844_d_n9, assign62380_e96844_d_n10, assign62380_e96844_d_n11, assign62380_e96844_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign62380_e96810: f64 = (1.0 / 2.0);
        let assign62380_e96814: f64 = (1.0 / 6.0);
        let assign62380_e96818: f64 = (1.0 / 24.0);
        let assign62380_e96822: f64 = (1.0 / 120.0);
        let assign62380_e96826: f64 = (1.0 / 720.0);
        let assign62380_e96830: f64 = (1.0 / 5040.0);
        let assign62380_e96831: f64 = (locals.var_tmf1 * assign62380_e96830);
        let assign62380_e96832: f64 = (assign62380_e96826 + assign62380_e96831);
        let assign62380_e96833: f64 = (locals.var_tmf1 * assign62380_e96832);
        let assign62380_e96834: f64 = (assign62380_e96822 + assign62380_e96833);
        let assign62380_e96835: f64 = (locals.var_tmf1 * assign62380_e96834);
        let assign62380_e96836: f64 = (assign62380_e96818 + assign62380_e96835);
        let assign62380_e96837: f64 = (locals.var_tmf1 * assign62380_e96836);
        let assign62380_e96838: f64 = (assign62380_e96814 + assign62380_e96837);
        let assign62380_e96839: f64 = (locals.var_tmf1 * assign62380_e96838);
        let assign62380_e96840: f64 = (assign62380_e96810 + assign62380_e96839);
        let assign62380_e96841: f64 = (locals.var_tmf1 * assign62380_e96840);
        let assign62380_e96842: f64 = (1.0 + assign62380_e96841);
        (assign62380_e96842, ((locals.var_tmf1_dn0 * assign62380_e96840) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign62380_e96838) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign62380_e96836) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign62380_e96834) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign62380_e96832) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign62380_e96830))))))))))), ((locals.var_tmf1_dn2 * assign62380_e96840) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign62380_e96838) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign62380_e96836) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign62380_e96834) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign62380_e96832) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign62380_e96830))))))))))), ((locals.var_tmf1_dn4 * assign62380_e96840) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign62380_e96838) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign62380_e96836) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign62380_e96834) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign62380_e96832) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign62380_e96830))))))))))), ((locals.var_tmf1_dn5 * assign62380_e96840) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign62380_e96838) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign62380_e96836) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign62380_e96834) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign62380_e96832) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign62380_e96830))))))))))), ((locals.var_tmf1_dn6 * assign62380_e96840) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign62380_e96838) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign62380_e96836) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign62380_e96834) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign62380_e96832) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign62380_e96830))))))))))), ((locals.var_tmf1_dn7 * assign62380_e96840) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign62380_e96838) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign62380_e96836) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign62380_e96834) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign62380_e96832) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign62380_e96830))))))))))), ((locals.var_tmf1_dn8 * assign62380_e96840) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign62380_e96838) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign62380_e96836) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign62380_e96834) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign62380_e96832) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign62380_e96830))))))))))), ((locals.var_tmf1_dn9 * assign62380_e96840) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign62380_e96838) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign62380_e96836) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign62380_e96834) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign62380_e96832) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign62380_e96830))))))))))), ((locals.var_tmf1_dn10 * assign62380_e96840) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign62380_e96838) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign62380_e96836) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign62380_e96834) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign62380_e96832) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign62380_e96830))))))))))), ((locals.var_tmf1_dn11 * assign62380_e96840) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign62380_e96838) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign62380_e96836) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign62380_e96834) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign62380_e96832) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign62380_e96830))))))))))), ((locals.var_tmf1_dn14 * assign62380_e96840) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign62380_e96838) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign62380_e96836) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign62380_e96834) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign62380_e96832) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign62380_e96830))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign62380_e96844;
        locals.var_tmf2_dn0 = assign62380_e96844_d_n0;
        locals.var_tmf2_dn2 = assign62380_e96844_d_n2;
        locals.var_tmf2_dn4 = assign62380_e96844_d_n4;
        locals.var_tmf2_dn5 = assign62380_e96844_d_n5;
        locals.var_tmf2_dn6 = assign62380_e96844_d_n6;
        locals.var_tmf2_dn7 = assign62380_e96844_d_n7;
        locals.var_tmf2_dn8 = assign62380_e96844_d_n8;
        locals.var_tmf2_dn9 = assign62380_e96844_d_n9;
        locals.var_tmf2_dn10 = assign62380_e96844_d_n10;
        locals.var_tmf2_dn11 = assign62380_e96844_d_n11;
        locals.var_tmf2_dn14 = assign62380_e96844_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign62390_e96885, assign62390_e96885_d_n0, assign62390_e96885_d_n2, assign62390_e96885_d_n4, assign62390_e96885_d_n5, assign62390_e96885_d_n6, assign62390_e96885_d_n7, assign62390_e96885_d_n8, assign62390_e96885_d_n9, assign62390_e96885_d_n10, assign62390_e96885_d_n11, assign62390_e96885_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign62390_e96853: f64 = (1.0 / 2.0);
        let assign62390_e96857: f64 = (1.0 / 3.0);
        let assign62390_e96861: f64 = (1.0 / 8.0);
        let assign62390_e96865: f64 = (1.0 / 30.0);
        let assign62390_e96869: f64 = (1.0 / 144.0);
        let assign62390_e96873: f64 = (1.0 / 840.0);
        let assign62390_e96874: f64 = (locals.var_tmf1 * assign62390_e96873);
        let assign62390_e96875: f64 = (assign62390_e96869 + assign62390_e96874);
        let assign62390_e96876: f64 = (locals.var_tmf1 * assign62390_e96875);
        let assign62390_e96877: f64 = (assign62390_e96865 + assign62390_e96876);
        let assign62390_e96878: f64 = (locals.var_tmf1 * assign62390_e96877);
        let assign62390_e96879: f64 = (assign62390_e96861 + assign62390_e96878);
        let assign62390_e96880: f64 = (locals.var_tmf1 * assign62390_e96879);
        let assign62390_e96881: f64 = (assign62390_e96857 + assign62390_e96880);
        let assign62390_e96882: f64 = (locals.var_tmf1 * assign62390_e96881);
        let assign62390_e96883: f64 = (assign62390_e96853 + assign62390_e96882);
        (assign62390_e96883, ((locals.var_tmf1_dn0 * assign62390_e96881) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign62390_e96879) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign62390_e96877) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign62390_e96875) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign62390_e96873))))))))), ((locals.var_tmf1_dn2 * assign62390_e96881) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign62390_e96879) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign62390_e96877) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign62390_e96875) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign62390_e96873))))))))), ((locals.var_tmf1_dn4 * assign62390_e96881) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign62390_e96879) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign62390_e96877) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign62390_e96875) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign62390_e96873))))))))), ((locals.var_tmf1_dn5 * assign62390_e96881) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign62390_e96879) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign62390_e96877) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign62390_e96875) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign62390_e96873))))))))), ((locals.var_tmf1_dn6 * assign62390_e96881) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign62390_e96879) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign62390_e96877) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign62390_e96875) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign62390_e96873))))))))), ((locals.var_tmf1_dn7 * assign62390_e96881) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign62390_e96879) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign62390_e96877) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign62390_e96875) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign62390_e96873))))))))), ((locals.var_tmf1_dn8 * assign62390_e96881) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign62390_e96879) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign62390_e96877) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign62390_e96875) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign62390_e96873))))))))), ((locals.var_tmf1_dn9 * assign62390_e96881) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign62390_e96879) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign62390_e96877) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign62390_e96875) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign62390_e96873))))))))), ((locals.var_tmf1_dn10 * assign62390_e96881) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign62390_e96879) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign62390_e96877) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign62390_e96875) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign62390_e96873))))))))), ((locals.var_tmf1_dn11 * assign62390_e96881) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign62390_e96879) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign62390_e96877) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign62390_e96875) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign62390_e96873))))))))), ((locals.var_tmf1_dn14 * assign62390_e96881) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign62390_e96879) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign62390_e96877) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign62390_e96875) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign62390_e96873))))))))),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn14,)
    }
};
        locals.var_tmf3 = assign62390_e96885;
        locals.var_tmf3_dn0 = assign62390_e96885_d_n0;
        locals.var_tmf3_dn2 = assign62390_e96885_d_n2;
        locals.var_tmf3_dn4 = assign62390_e96885_d_n4;
        locals.var_tmf3_dn5 = assign62390_e96885_d_n5;
        locals.var_tmf3_dn6 = assign62390_e96885_d_n6;
        locals.var_tmf3_dn7 = assign62390_e96885_d_n7;
        locals.var_tmf3_dn8 = assign62390_e96885_d_n8;
        locals.var_tmf3_dn9 = assign62390_e96885_d_n9;
        locals.var_tmf3_dn10 = assign62390_e96885_d_n10;
        locals.var_tmf3_dn11 = assign62390_e96885_d_n11;
        locals.var_tmf3_dn14 = assign62390_e96885_d_n14;
        locals.var_tmf3_rv = 0.0;

        let (assign62400_e96896, assign62400_e96896_d_n0, assign62400_e96896_d_n2, assign62400_e96896_d_n4, assign62400_e96896_d_n5, assign62400_e96896_d_n6, assign62400_e96896_d_n7, assign62400_e96896_d_n8, assign62400_e96896_d_n9, assign62400_e96896_d_n10, assign62400_e96896_d_n11, assign62400_e96896_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign62400_e96894: f64 = (0.01 / locals.var_tmf2);
        (assign62400_e96894, (-((0.01 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign62400_e96896;
        locals.var_t6_dn0 = assign62400_e96896_d_n0;
        locals.var_t6_dn2 = assign62400_e96896_d_n2;
        locals.var_t6_dn4 = assign62400_e96896_d_n4;
        locals.var_t6_dn5 = assign62400_e96896_d_n5;
        locals.var_t6_dn6 = assign62400_e96896_d_n6;
        locals.var_t6_dn7 = assign62400_e96896_d_n7;
        locals.var_t6_dn8 = assign62400_e96896_d_n8;
        locals.var_t6_dn9 = assign62400_e96896_d_n9;
        locals.var_t6_dn10 = assign62400_e96896_d_n10;
        locals.var_t6_dn11 = assign62400_e96896_d_n11;
        locals.var_t6_dn14 = assign62400_e96896_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign62410_e96912, assign62410_e96912_d_n0, assign62410_e96912_d_n2, assign62410_e96912_d_n4, assign62410_e96912_d_n5, assign62410_e96912_d_n6, assign62410_e96912_d_n7, assign62410_e96912_d_n8, assign62410_e96912_d_n9, assign62410_e96912_d_n10, assign62410_e96912_d_n11, assign62410_e96912_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign62410_e96904: f64 = (-2.0);
        let assign62410_e96906: f64 = (assign62410_e96904 * locals.var_tmf3);
        let assign62410_e96909: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign62410_e96910: f64 = (assign62410_e96906 / assign62410_e96909);
        (assign62410_e96910, ((((assign62410_e96904 * locals.var_tmf3_dn0) * assign62410_e96909) - (assign62410_e96906 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign62410_e96909 * assign62410_e96909)), ((((assign62410_e96904 * locals.var_tmf3_dn2) * assign62410_e96909) - (assign62410_e96906 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign62410_e96909 * assign62410_e96909)), ((((assign62410_e96904 * locals.var_tmf3_dn4) * assign62410_e96909) - (assign62410_e96906 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign62410_e96909 * assign62410_e96909)), ((((assign62410_e96904 * locals.var_tmf3_dn5) * assign62410_e96909) - (assign62410_e96906 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign62410_e96909 * assign62410_e96909)), ((((assign62410_e96904 * locals.var_tmf3_dn6) * assign62410_e96909) - (assign62410_e96906 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign62410_e96909 * assign62410_e96909)), ((((assign62410_e96904 * locals.var_tmf3_dn7) * assign62410_e96909) - (assign62410_e96906 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign62410_e96909 * assign62410_e96909)), ((((assign62410_e96904 * locals.var_tmf3_dn8) * assign62410_e96909) - (assign62410_e96906 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign62410_e96909 * assign62410_e96909)), ((((assign62410_e96904 * locals.var_tmf3_dn9) * assign62410_e96909) - (assign62410_e96906 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign62410_e96909 * assign62410_e96909)), ((((assign62410_e96904 * locals.var_tmf3_dn10) * assign62410_e96909) - (assign62410_e96906 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign62410_e96909 * assign62410_e96909)), ((((assign62410_e96904 * locals.var_tmf3_dn11) * assign62410_e96909) - (assign62410_e96906 * ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)))) / (assign62410_e96909 * assign62410_e96909)), ((((assign62410_e96904 * locals.var_tmf3_dn14) * assign62410_e96909) - (assign62410_e96906 * ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)))) / (assign62410_e96909 * assign62410_e96909)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign62410_e96912;
        locals.var_t2_dn0 = assign62410_e96912_d_n0;
        locals.var_t2_dn2 = assign62410_e96912_d_n2;
        locals.var_t2_dn4 = assign62410_e96912_d_n4;
        locals.var_t2_dn5 = assign62410_e96912_d_n5;
        locals.var_t2_dn6 = assign62410_e96912_d_n6;
        locals.var_t2_dn7 = assign62410_e96912_d_n7;
        locals.var_t2_dn8 = assign62410_e96912_d_n8;
        locals.var_t2_dn9 = assign62410_e96912_d_n9;
        locals.var_t2_dn10 = assign62410_e96912_d_n10;
        locals.var_t2_dn11 = assign62410_e96912_d_n11;
        locals.var_t2_dn14 = assign62410_e96912_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign62420_e96923, assign62420_e96923_d_n0, assign62420_e96923_d_n2, assign62420_e96923_d_n4, assign62420_e96923_d_n5, assign62420_e96923_d_n6, assign62420_e96923_d_n7, assign62420_e96923_d_n8, assign62420_e96923_d_n9, assign62420_e96923_d_n10, assign62420_e96923_d_n11, assign62420_e96923_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign62420_e96921: f64 = (locals.var_t2 * 0.5);
        (assign62420_e96921, (locals.var_t2_dn0 * 0.5), (locals.var_t2_dn2 * 0.5), (locals.var_t2_dn4 * 0.5), (locals.var_t2_dn5 * 0.5), (locals.var_t2_dn6 * 0.5), (locals.var_t2_dn7 * 0.5), (locals.var_t2_dn8 * 0.5), (locals.var_t2_dn9 * 0.5), (locals.var_t2_dn10 * 0.5), (locals.var_t2_dn11 * 0.5), (locals.var_t2_dn14 * 0.5),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign62420_e96923;
        locals.var_t2_dn0 = assign62420_e96923_d_n0;
        locals.var_t2_dn2 = assign62420_e96923_d_n2;
        locals.var_t2_dn4 = assign62420_e96923_d_n4;
        locals.var_t2_dn5 = assign62420_e96923_d_n5;
        locals.var_t2_dn6 = assign62420_e96923_d_n6;
        locals.var_t2_dn7 = assign62420_e96923_d_n7;
        locals.var_t2_dn8 = assign62420_e96923_d_n8;
        locals.var_t2_dn9 = assign62420_e96923_d_n9;
        locals.var_t2_dn10 = assign62420_e96923_d_n10;
        locals.var_t2_dn11 = assign62420_e96923_d_n11;
        locals.var_t2_dn14 = assign62420_e96923_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign62430_e96936, assign62430_e96936_d_n0, assign62430_e96936_d_n2, assign62430_e96936_d_n4, assign62430_e96936_d_n5, assign62430_e96936_d_n6, assign62430_e96936_d_n7, assign62430_e96936_d_n8, assign62430_e96936_d_n9, assign62430_e96936_d_n10, assign62430_e96936_d_n11, assign62430_e96936_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign62430_e96933: f64 = (locals.var_ps0 + locals.var_t6);
        let assign62430_e96934: f64 = (1.1 - assign62430_e96933);
        (assign62430_e96934, (-(locals.var_ps0_dn0 + locals.var_t6_dn0)), (-(locals.var_ps0_dn2 + locals.var_t6_dn2)), (-(locals.var_ps0_dn4 + locals.var_t6_dn4)), (-(locals.var_ps0_dn5 + locals.var_t6_dn5)), (-(locals.var_ps0_dn6 + locals.var_t6_dn6)), (-(locals.var_ps0_dn7 + locals.var_t6_dn7)), (-(locals.var_ps0_dn8 + locals.var_t6_dn8)), (-(locals.var_ps0_dn9 + locals.var_t6_dn9)), (-(locals.var_ps0_dn10 + locals.var_t6_dn10)), (-(locals.var_ps0_dn11 + locals.var_t6_dn11)), (-(locals.var_ps0_dn14 + locals.var_t6_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign62430_e96936;
        locals.var_t1_dn0 = assign62430_e96936_d_n0;
        locals.var_t1_dn2 = assign62430_e96936_d_n2;
        locals.var_t1_dn4 = assign62430_e96936_d_n4;
        locals.var_t1_dn5 = assign62430_e96936_d_n5;
        locals.var_t1_dn6 = assign62430_e96936_d_n6;
        locals.var_t1_dn7 = assign62430_e96936_d_n7;
        locals.var_t1_dn8 = assign62430_e96936_d_n8;
        locals.var_t1_dn9 = assign62430_e96936_d_n9;
        locals.var_t1_dn10 = assign62430_e96936_d_n10;
        locals.var_t1_dn11 = assign62430_e96936_d_n11;
        locals.var_t1_dn14 = assign62430_e96936_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign62440_e96954, assign62440_e96954_d_n0, assign62440_e96954_d_n2, assign62440_e96954_d_n4, assign62440_e96954_d_n5, assign62440_e96954_d_n6, assign62440_e96954_d_n7, assign62440_e96954_d_n8, assign62440_e96954_d_n9, assign62440_e96954_d_n10, assign62440_e96954_d_n11, assign62440_e96954_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign62440_e96945: f64 = (locals.var_t1 * locals.var_t1);
        let assign62440_e96948: f64 = (4.0 * 0.05);
        let assign62440_e96950: f64 = (assign62440_e96948 * 0.05);
        let assign62440_e96951: f64 = (assign62440_e96945 + assign62440_e96950);
        let assign62440_e96952: f64 = (assign62440_e96951).sqrt();
        (assign62440_e96952, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign62440_e96952)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign62440_e96952)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign62440_e96952)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign62440_e96952)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign62440_e96952)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign62440_e96952)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign62440_e96952)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign62440_e96952)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign62440_e96952)), (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign62440_e96952)), (((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) / (2.0 * assign62440_e96952)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign62440_e96954;
        locals.var_tmf2_dn0 = assign62440_e96954_d_n0;
        locals.var_tmf2_dn2 = assign62440_e96954_d_n2;
        locals.var_tmf2_dn4 = assign62440_e96954_d_n4;
        locals.var_tmf2_dn5 = assign62440_e96954_d_n5;
        locals.var_tmf2_dn6 = assign62440_e96954_d_n6;
        locals.var_tmf2_dn7 = assign62440_e96954_d_n7;
        locals.var_tmf2_dn8 = assign62440_e96954_d_n8;
        locals.var_tmf2_dn9 = assign62440_e96954_d_n9;
        locals.var_tmf2_dn10 = assign62440_e96954_d_n10;
        locals.var_tmf2_dn11 = assign62440_e96954_d_n11;
        locals.var_tmf2_dn14 = assign62440_e96954_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign62450_e96969, assign62450_e96969_d_n0, assign62450_e96969_d_n2, assign62450_e96969_d_n4, assign62450_e96969_d_n5, assign62450_e96969_d_n6, assign62450_e96969_d_n7, assign62450_e96969_d_n8, assign62450_e96969_d_n9, assign62450_e96969_d_n10, assign62450_e96969_d_n11, assign62450_e96969_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign62450_e96965: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign62450_e96966: f64 = (1.0 + assign62450_e96965);
        let assign62450_e96967: f64 = (0.5 * assign62450_e96966);
        (assign62450_e96967, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn7 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn9 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn11 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn14 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign62450_e96969;
        locals.var_t0_dn0 = assign62450_e96969_d_n0;
        locals.var_t0_dn2 = assign62450_e96969_d_n2;
        locals.var_t0_dn4 = assign62450_e96969_d_n4;
        locals.var_t0_dn5 = assign62450_e96969_d_n5;
        locals.var_t0_dn6 = assign62450_e96969_d_n6;
        locals.var_t0_dn7 = assign62450_e96969_d_n7;
        locals.var_t0_dn8 = assign62450_e96969_d_n8;
        locals.var_t0_dn9 = assign62450_e96969_d_n9;
        locals.var_t0_dn10 = assign62450_e96969_d_n10;
        locals.var_t0_dn11 = assign62450_e96969_d_n11;
        locals.var_t0_dn14 = assign62450_e96969_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign62460_e96982, assign62460_e96982_d_n0, assign62460_e96982_d_n2, assign62460_e96982_d_n4, assign62460_e96982_d_n5, assign62460_e96982_d_n6, assign62460_e96982_d_n7, assign62460_e96982_d_n8, assign62460_e96982_d_n9, assign62460_e96982_d_n10, assign62460_e96982_d_n11, assign62460_e96982_d_n14,) = {
    if (((locals.var_guard447 == 0.0) && (locals.var_guard1434 != 0.0)) && (locals.var_guard1499 != 0.0)) {
        let assign62460_e96979: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign62460_e96980: f64 = (0.5 * assign62460_e96979);
        (assign62460_e96980, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign62460_e96982;
        locals.var_t2_dn0 = assign62460_e96982_d_n0;
        locals.var_t2_dn2 = assign62460_e96982_d_n2;
        locals.var_t2_dn4 = assign62460_e96982_d_n4;
        locals.var_t2_dn5 = assign62460_e96982_d_n5;
        locals.var_t2_dn6 = assign62460_e96982_d_n6;
        locals.var_t2_dn7 = assign62460_e96982_d_n7;
        locals.var_t2_dn8 = assign62460_e96982_d_n8;
        locals.var_t2_dn9 = assign62460_e96982_d_n9;
        locals.var_t2_dn10 = assign62460_e96982_d_n10;
        locals.var_t2_dn11 = assign62460_e96982_d_n11;
        locals.var_t2_dn14 = assign62460_e96982_d_n14;
        locals.var_t2_rv = 0.0;

        let assign62470_e96985: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1500 = assign62470_e96985;
        locals.var_guard1500_rv = 0.0;

    }
}
