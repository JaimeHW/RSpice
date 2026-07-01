#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_205(
        p: &Parameters,
        var_cbox_1: f64,
        var_cboxwl: f64,
        var_devsign: f64,
        var_guard492: f64,
        var_vbs: f64,
        var_vbs_dn10: f64,
        var_vbs_dn6: f64,
        var_vbs_dn7: f64,
        var_ved: f64,
        var_ved_dn10: f64,
        var_ved_dn3: f64,
        var_ved_dn6: f64,
        var_ved_dn7: f64,
        var_ves_1: f64,
        var_ves_1_dn10: f64,
        var_ves_1_dn3: f64,
        var_ves_1_dn6: f64,
        var_ves_1_dn7: f64,
        var_vesfb: f64,
        var_vesfb_dn10: f64,
        var_vesfb_dn11: f64,
        var_vesfb_dn3: f64,
        var_vesfb_dn4: f64,
        var_vesfb_dn5: f64,
        var_vesfb_dn6: f64,
        var_vesfb_dn7: f64,
        var_vesfb_dn8: f64,
        var_vesfb_dn9: f64,
        var_cdbox_slot: &mut f64,
        var_cdbox_rv_slot: &mut f64,
        var_cdesw_slot: &mut f64,
        var_cdesw_dn10_slot: &mut f64,
        var_cdesw_dn11_slot: &mut f64,
        var_cdesw_dn3_slot: &mut f64,
        var_cdesw_dn4_slot: &mut f64,
        var_cdesw_dn5_slot: &mut f64,
        var_cdesw_dn6_slot: &mut f64,
        var_cdesw_dn7_slot: &mut f64,
        var_cdesw_dn8_slot: &mut f64,
        var_cdesw_dn9_slot: &mut f64,
        var_cdesw_rv_slot: &mut f64,
        var_cdmin_slot: &mut f64,
        var_cdmin_rv_slot: &mut f64,
        var_csbox_slot: &mut f64,
        var_csbox_rv_slot: &mut f64,
        var_csesw_slot: &mut f64,
        var_csesw_dn10_slot: &mut f64,
        var_csesw_dn11_slot: &mut f64,
        var_csesw_dn3_slot: &mut f64,
        var_csesw_dn4_slot: &mut f64,
        var_csesw_dn5_slot: &mut f64,
        var_csesw_dn6_slot: &mut f64,
        var_csesw_dn7_slot: &mut f64,
        var_csesw_dn8_slot: &mut f64,
        var_csesw_dn9_slot: &mut f64,
        var_csesw_rv_slot: &mut f64,
        var_csmin_slot: &mut f64,
        var_csmin_rv_slot: &mut f64,
        var_guard864_slot: &mut f64,
        var_guard864_rv_slot: &mut f64,
        var_guard865_slot: &mut f64,
        var_guard865_rv_slot: &mut f64,
        var_guard866_slot: &mut f64,
        var_guard866_rv_slot: &mut f64,
        var_qde_slot: &mut f64,
        var_qde_dn10_slot: &mut f64,
        var_qde_dn11_slot: &mut f64,
        var_qde_dn3_slot: &mut f64,
        var_qde_dn4_slot: &mut f64,
        var_qde_dn5_slot: &mut f64,
        var_qde_dn6_slot: &mut f64,
        var_qde_dn7_slot: &mut f64,
        var_qde_dn8_slot: &mut f64,
        var_qde_dn9_slot: &mut f64,
        var_qde_rv_slot: &mut f64,
        var_qe1_slot: &mut f64,
        var_qe1_dn10_slot: &mut f64,
        var_qe1_dn11_slot: &mut f64,
        var_qe1_dn3_slot: &mut f64,
        var_qe1_dn4_slot: &mut f64,
        var_qe1_dn5_slot: &mut f64,
        var_qe1_dn6_slot: &mut f64,
        var_qe1_dn7_slot: &mut f64,
        var_qe1_dn8_slot: &mut f64,
        var_qe1_dn9_slot: &mut f64,
        var_qe1_rv_slot: &mut f64,
        var_qse_slot: &mut f64,
        var_qse_dn10_slot: &mut f64,
        var_qse_dn11_slot: &mut f64,
        var_qse_dn3_slot: &mut f64,
        var_qse_dn4_slot: &mut f64,
        var_qse_dn5_slot: &mut f64,
        var_qse_dn6_slot: &mut f64,
        var_qse_dn7_slot: &mut f64,
        var_qse_dn8_slot: &mut f64,
        var_qse_dn9_slot: &mut f64,
        var_qse_rv_slot: &mut f64,
        var_qsub_slot: &mut f64,
        var_qsub_dn10_slot: &mut f64,
        var_qsub_dn11_slot: &mut f64,
        var_qsub_dn3_slot: &mut f64,
        var_qsub_dn4_slot: &mut f64,
        var_qsub_dn5_slot: &mut f64,
        var_qsub_dn6_slot: &mut f64,
        var_qsub_dn7_slot: &mut f64,
        var_qsub_dn8_slot: &mut f64,
        var_qsub_dn9_slot: &mut f64,
        var_qsub_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t10_slot: &mut f64,
        var_t10_dn10_slot: &mut f64,
        var_t10_dn11_slot: &mut f64,
        var_t10_dn3_slot: &mut f64,
        var_t10_dn4_slot: &mut f64,
        var_t10_dn5_slot: &mut f64,
        var_t10_dn6_slot: &mut f64,
        var_t10_dn7_slot: &mut f64,
        var_t10_dn8_slot: &mut f64,
        var_t10_dn9_slot: &mut f64,
        var_t10_rv_slot: &mut f64,
        var_t11_slot: &mut f64,
        var_t11_dn10_slot: &mut f64,
        var_t11_dn11_slot: &mut f64,
        var_t11_dn3_slot: &mut f64,
        var_t11_dn4_slot: &mut f64,
        var_t11_dn5_slot: &mut f64,
        var_t11_dn6_slot: &mut f64,
        var_t11_dn7_slot: &mut f64,
        var_t11_dn8_slot: &mut f64,
        var_t11_dn9_slot: &mut f64,
        var_t11_rv_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_dn9_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
    ) {
        let mut var_cdbox: f64 = *var_cdbox_slot;
        let mut var_cdbox_rv: f64 = *var_cdbox_rv_slot;
        let mut var_cdesw: f64 = *var_cdesw_slot;
        let mut var_cdesw_dn10: f64 = *var_cdesw_dn10_slot;
        let mut var_cdesw_dn11: f64 = *var_cdesw_dn11_slot;
        let mut var_cdesw_dn3: f64 = *var_cdesw_dn3_slot;
        let mut var_cdesw_dn4: f64 = *var_cdesw_dn4_slot;
        let mut var_cdesw_dn5: f64 = *var_cdesw_dn5_slot;
        let mut var_cdesw_dn6: f64 = *var_cdesw_dn6_slot;
        let mut var_cdesw_dn7: f64 = *var_cdesw_dn7_slot;
        let mut var_cdesw_dn8: f64 = *var_cdesw_dn8_slot;
        let mut var_cdesw_dn9: f64 = *var_cdesw_dn9_slot;
        let mut var_cdesw_rv: f64 = *var_cdesw_rv_slot;
        let mut var_cdmin: f64 = *var_cdmin_slot;
        let mut var_cdmin_rv: f64 = *var_cdmin_rv_slot;
        let mut var_csbox: f64 = *var_csbox_slot;
        let mut var_csbox_rv: f64 = *var_csbox_rv_slot;
        let mut var_csesw: f64 = *var_csesw_slot;
        let mut var_csesw_dn10: f64 = *var_csesw_dn10_slot;
        let mut var_csesw_dn11: f64 = *var_csesw_dn11_slot;
        let mut var_csesw_dn3: f64 = *var_csesw_dn3_slot;
        let mut var_csesw_dn4: f64 = *var_csesw_dn4_slot;
        let mut var_csesw_dn5: f64 = *var_csesw_dn5_slot;
        let mut var_csesw_dn6: f64 = *var_csesw_dn6_slot;
        let mut var_csesw_dn7: f64 = *var_csesw_dn7_slot;
        let mut var_csesw_dn8: f64 = *var_csesw_dn8_slot;
        let mut var_csesw_dn9: f64 = *var_csesw_dn9_slot;
        let mut var_csesw_rv: f64 = *var_csesw_rv_slot;
        let mut var_csmin: f64 = *var_csmin_slot;
        let mut var_csmin_rv: f64 = *var_csmin_rv_slot;
        let mut var_guard864: f64 = *var_guard864_slot;
        let mut var_guard864_rv: f64 = *var_guard864_rv_slot;
        let mut var_guard865: f64 = *var_guard865_slot;
        let mut var_guard865_rv: f64 = *var_guard865_rv_slot;
        let mut var_guard866: f64 = *var_guard866_slot;
        let mut var_guard866_rv: f64 = *var_guard866_rv_slot;
        let mut var_qde: f64 = *var_qde_slot;
        let mut var_qde_dn10: f64 = *var_qde_dn10_slot;
        let mut var_qde_dn11: f64 = *var_qde_dn11_slot;
        let mut var_qde_dn3: f64 = *var_qde_dn3_slot;
        let mut var_qde_dn4: f64 = *var_qde_dn4_slot;
        let mut var_qde_dn5: f64 = *var_qde_dn5_slot;
        let mut var_qde_dn6: f64 = *var_qde_dn6_slot;
        let mut var_qde_dn7: f64 = *var_qde_dn7_slot;
        let mut var_qde_dn8: f64 = *var_qde_dn8_slot;
        let mut var_qde_dn9: f64 = *var_qde_dn9_slot;
        let mut var_qde_rv: f64 = *var_qde_rv_slot;
        let mut var_qe1: f64 = *var_qe1_slot;
        let mut var_qe1_dn10: f64 = *var_qe1_dn10_slot;
        let mut var_qe1_dn11: f64 = *var_qe1_dn11_slot;
        let mut var_qe1_dn3: f64 = *var_qe1_dn3_slot;
        let mut var_qe1_dn4: f64 = *var_qe1_dn4_slot;
        let mut var_qe1_dn5: f64 = *var_qe1_dn5_slot;
        let mut var_qe1_dn6: f64 = *var_qe1_dn6_slot;
        let mut var_qe1_dn7: f64 = *var_qe1_dn7_slot;
        let mut var_qe1_dn8: f64 = *var_qe1_dn8_slot;
        let mut var_qe1_dn9: f64 = *var_qe1_dn9_slot;
        let mut var_qe1_rv: f64 = *var_qe1_rv_slot;
        let mut var_qse: f64 = *var_qse_slot;
        let mut var_qse_dn10: f64 = *var_qse_dn10_slot;
        let mut var_qse_dn11: f64 = *var_qse_dn11_slot;
        let mut var_qse_dn3: f64 = *var_qse_dn3_slot;
        let mut var_qse_dn4: f64 = *var_qse_dn4_slot;
        let mut var_qse_dn5: f64 = *var_qse_dn5_slot;
        let mut var_qse_dn6: f64 = *var_qse_dn6_slot;
        let mut var_qse_dn7: f64 = *var_qse_dn7_slot;
        let mut var_qse_dn8: f64 = *var_qse_dn8_slot;
        let mut var_qse_dn9: f64 = *var_qse_dn9_slot;
        let mut var_qse_rv: f64 = *var_qse_rv_slot;
        let mut var_qsub: f64 = *var_qsub_slot;
        let mut var_qsub_dn10: f64 = *var_qsub_dn10_slot;
        let mut var_qsub_dn11: f64 = *var_qsub_dn11_slot;
        let mut var_qsub_dn3: f64 = *var_qsub_dn3_slot;
        let mut var_qsub_dn4: f64 = *var_qsub_dn4_slot;
        let mut var_qsub_dn5: f64 = *var_qsub_dn5_slot;
        let mut var_qsub_dn6: f64 = *var_qsub_dn6_slot;
        let mut var_qsub_dn7: f64 = *var_qsub_dn7_slot;
        let mut var_qsub_dn8: f64 = *var_qsub_dn8_slot;
        let mut var_qsub_dn9: f64 = *var_qsub_dn9_slot;
        let mut var_qsub_rv: f64 = *var_qsub_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t10: f64 = *var_t10_slot;
        let mut var_t10_dn10: f64 = *var_t10_dn10_slot;
        let mut var_t10_dn11: f64 = *var_t10_dn11_slot;
        let mut var_t10_dn3: f64 = *var_t10_dn3_slot;
        let mut var_t10_dn4: f64 = *var_t10_dn4_slot;
        let mut var_t10_dn5: f64 = *var_t10_dn5_slot;
        let mut var_t10_dn6: f64 = *var_t10_dn6_slot;
        let mut var_t10_dn7: f64 = *var_t10_dn7_slot;
        let mut var_t10_dn8: f64 = *var_t10_dn8_slot;
        let mut var_t10_dn9: f64 = *var_t10_dn9_slot;
        let mut var_t10_rv: f64 = *var_t10_rv_slot;
        let mut var_t11: f64 = *var_t11_slot;
        let mut var_t11_dn10: f64 = *var_t11_dn10_slot;
        let mut var_t11_dn11: f64 = *var_t11_dn11_slot;
        let mut var_t11_dn3: f64 = *var_t11_dn3_slot;
        let mut var_t11_dn4: f64 = *var_t11_dn4_slot;
        let mut var_t11_dn5: f64 = *var_t11_dn5_slot;
        let mut var_t11_dn6: f64 = *var_t11_dn6_slot;
        let mut var_t11_dn7: f64 = *var_t11_dn7_slot;
        let mut var_t11_dn8: f64 = *var_t11_dn8_slot;
        let mut var_t11_dn9: f64 = *var_t11_dn9_slot;
        let mut var_t11_rv: f64 = *var_t11_rv_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_dn9: f64 = *var_t3_dn9_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;

        let (assign58780_e95754, assign58780_e95754_d_n3, assign58780_e95754_d_n4, assign58780_e95754_d_n5, assign58780_e95754_d_n6, assign58780_e95754_d_n7, assign58780_e95754_d_n8, assign58780_e95754_d_n9, assign58780_e95754_d_n10, assign58780_e95754_d_n11,) = {
    if (var_guard492 == 0.0) {
        let assign58780_e95751: f64 = (var_vesfb - var_vbs);
        let assign58780_e95752: f64 = (var_cboxwl * assign58780_e95751);
        (assign58780_e95752, (var_cboxwl * var_vesfb_dn3), (var_cboxwl * var_vesfb_dn4), (var_cboxwl * var_vesfb_dn5), (var_cboxwl * (var_vesfb_dn6 - var_vbs_dn6)), (var_cboxwl * (var_vesfb_dn7 - var_vbs_dn7)), (var_cboxwl * var_vesfb_dn8), (var_cboxwl * var_vesfb_dn9), (var_cboxwl * (var_vesfb_dn10 - var_vbs_dn10)), (var_cboxwl * var_vesfb_dn11),)
    } else {
        (var_qe1, var_qe1_dn3, var_qe1_dn4, var_qe1_dn5, var_qe1_dn6, var_qe1_dn7, var_qe1_dn8, var_qe1_dn9, var_qe1_dn10, var_qe1_dn11,)
    }
};
        var_qe1 = assign58780_e95754;
        var_qe1_dn3 = assign58780_e95754_d_n3;
        var_qe1_dn4 = assign58780_e95754_d_n4;
        var_qe1_dn5 = assign58780_e95754_d_n5;
        var_qe1_dn6 = assign58780_e95754_d_n6;
        var_qe1_dn7 = assign58780_e95754_d_n7;
        var_qe1_dn8 = assign58780_e95754_d_n8;
        var_qe1_dn9 = assign58780_e95754_d_n9;
        var_qe1_dn10 = assign58780_e95754_d_n10;
        var_qe1_dn11 = assign58780_e95754_d_n11;
        var_qe1_rv = 0.0;

        let (assign58790_e95759, assign58790_e95759_d_n3, assign58790_e95759_d_n4, assign58790_e95759_d_n5, assign58790_e95759_d_n6, assign58790_e95759_d_n7, assign58790_e95759_d_n8, assign58790_e95759_d_n9, assign58790_e95759_d_n10, assign58790_e95759_d_n11,) = {
    if (var_guard492 == 0.0) {
        (var_qe1, var_qe1_dn3, var_qe1_dn4, var_qe1_dn5, var_qe1_dn6, var_qe1_dn7, var_qe1_dn8, var_qe1_dn9, var_qe1_dn10, var_qe1_dn11,)
    } else {
        (var_qsub, var_qsub_dn3, var_qsub_dn4, var_qsub_dn5, var_qsub_dn6, var_qsub_dn7, var_qsub_dn8, var_qsub_dn9, var_qsub_dn10, var_qsub_dn11,)
    }
};
        var_qsub = assign58790_e95759;
        var_qsub_dn3 = assign58790_e95759_d_n3;
        var_qsub_dn4 = assign58790_e95759_d_n4;
        var_qsub_dn5 = assign58790_e95759_d_n5;
        var_qsub_dn6 = assign58790_e95759_d_n6;
        var_qsub_dn7 = assign58790_e95759_d_n7;
        var_qsub_dn8 = assign58790_e95759_d_n8;
        var_qsub_dn9 = assign58790_e95759_d_n9;
        var_qsub_dn10 = assign58790_e95759_d_n10;
        var_qsub_dn11 = assign58790_e95759_d_n11;
        var_qsub_rv = 0.0;

        let (assign58800_e95775, assign58800_e95775_d_n3, assign58800_e95775_d_n4, assign58800_e95775_d_n5, assign58800_e95775_d_n6, assign58800_e95775_d_n7, assign58800_e95775_d_n8, assign58800_e95775_d_n9, assign58800_e95775_d_n10, assign58800_e95775_d_n11,) = {
    if (var_guard492 == 0.0) {
        let assign58800_e95767: f64 = (p.p74 / p.p75);
        let assign58800_e95768: f64 = (1.0 + assign58800_e95767);
        let assign58800_e95769: f64 = (p.p871 * assign58800_e95768);
        let assign58800_e95771: f64 = (assign58800_e95769).max(1e-38);
        let assign58800_e95772: f64 = (assign58800_e95771).ln();
        let assign58800_e95773: f64 = (p.p1395 * assign58800_e95772);
        (assign58800_e95773, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11,)
    }
};
        var_t0 = assign58800_e95775;
        var_t0_dn3 = assign58800_e95775_d_n3;
        var_t0_dn4 = assign58800_e95775_d_n4;
        var_t0_dn5 = assign58800_e95775_d_n5;
        var_t0_dn6 = assign58800_e95775_d_n6;
        var_t0_dn7 = assign58800_e95775_d_n7;
        var_t0_dn8 = assign58800_e95775_d_n8;
        var_t0_dn9 = assign58800_e95775_d_n9;
        var_t0_dn10 = assign58800_e95775_d_n10;
        var_t0_dn11 = assign58800_e95775_d_n11;
        var_t0_rv = 0.0;

        let (assign58810_e95782, assign58810_e95782_d_n3, assign58810_e95782_d_n4, assign58810_e95782_d_n5, assign58810_e95782_d_n6, assign58810_e95782_d_n7, assign58810_e95782_d_n8, assign58810_e95782_d_n9, assign58810_e95782_d_n10, assign58810_e95782_d_n11,) = {
    if (var_guard492 == 0.0) {
        let assign58810_e95780: f64 = (p.p19 - p.p1);
        (assign58810_e95780, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11,)
    }
};
        var_t1 = assign58810_e95782;
        var_t1_dn3 = assign58810_e95782_d_n3;
        var_t1_dn4 = assign58810_e95782_d_n4;
        var_t1_dn5 = assign58810_e95782_d_n5;
        var_t1_dn6 = assign58810_e95782_d_n6;
        var_t1_dn7 = assign58810_e95782_d_n7;
        var_t1_dn8 = assign58810_e95782_d_n8;
        var_t1_dn9 = assign58810_e95782_d_n9;
        var_t1_dn10 = assign58810_e95782_d_n10;
        var_t1_dn11 = assign58810_e95782_d_n11;
        var_t1_rv = 0.0;

        let assign58820_e95785: f64 = if var_t1 > 0.0 { 1.0 } else { 0.0 };
        var_guard864 = assign58820_e95785;
        var_guard864_rv = 0.0;

        let (assign58830_e95794, assign58830_e95794_d_n3, assign58830_e95794_d_n4, assign58830_e95794_d_n5, assign58830_e95794_d_n6, assign58830_e95794_d_n7, assign58830_e95794_d_n8, assign58830_e95794_d_n9, assign58830_e95794_d_n10, assign58830_e95794_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard864 != 0.0)) {
        let assign58830_e95792: f64 = (var_t0 * var_t1);
        (assign58830_e95792, ((var_t0_dn3 * var_t1) + (var_t0 * var_t1_dn3)), ((var_t0_dn4 * var_t1) + (var_t0 * var_t1_dn4)), ((var_t0_dn5 * var_t1) + (var_t0 * var_t1_dn5)), ((var_t0_dn6 * var_t1) + (var_t0 * var_t1_dn6)), ((var_t0_dn7 * var_t1) + (var_t0 * var_t1_dn7)), ((var_t0_dn8 * var_t1) + (var_t0 * var_t1_dn8)), ((var_t0_dn9 * var_t1) + (var_t0 * var_t1_dn9)), ((var_t0_dn10 * var_t1) + (var_t0 * var_t1_dn10)), ((var_t0_dn11 * var_t1) + (var_t0 * var_t1_dn11)),)
    } else {
        (var_csesw, var_csesw_dn3, var_csesw_dn4, var_csesw_dn5, var_csesw_dn6, var_csesw_dn7, var_csesw_dn8, var_csesw_dn9, var_csesw_dn10, var_csesw_dn11,)
    }
};
        var_csesw = assign58830_e95794;
        var_csesw_dn3 = assign58830_e95794_d_n3;
        var_csesw_dn4 = assign58830_e95794_d_n4;
        var_csesw_dn5 = assign58830_e95794_d_n5;
        var_csesw_dn6 = assign58830_e95794_d_n6;
        var_csesw_dn7 = assign58830_e95794_d_n7;
        var_csesw_dn8 = assign58830_e95794_d_n8;
        var_csesw_dn9 = assign58830_e95794_d_n9;
        var_csesw_dn10 = assign58830_e95794_d_n10;
        var_csesw_dn11 = assign58830_e95794_d_n11;
        var_csesw_rv = 0.0;

        let (assign58840_e95802, assign58840_e95802_d_n3, assign58840_e95802_d_n4, assign58840_e95802_d_n5, assign58840_e95802_d_n6, assign58840_e95802_d_n7, assign58840_e95802_d_n8, assign58840_e95802_d_n9, assign58840_e95802_d_n10, assign58840_e95802_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard864 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_csesw, var_csesw_dn3, var_csesw_dn4, var_csesw_dn5, var_csesw_dn6, var_csesw_dn7, var_csesw_dn8, var_csesw_dn9, var_csesw_dn10, var_csesw_dn11,)
    }
};
        var_csesw = assign58840_e95802;
        var_csesw_dn3 = assign58840_e95802_d_n3;
        var_csesw_dn4 = assign58840_e95802_d_n4;
        var_csesw_dn5 = assign58840_e95802_d_n5;
        var_csesw_dn6 = assign58840_e95802_d_n6;
        var_csesw_dn7 = assign58840_e95802_d_n7;
        var_csesw_dn8 = assign58840_e95802_d_n8;
        var_csesw_dn9 = assign58840_e95802_d_n9;
        var_csesw_dn10 = assign58840_e95802_d_n10;
        var_csesw_dn11 = assign58840_e95802_d_n11;
        var_csesw_rv = 0.0;

        let (assign58850_e95809, assign58850_e95809_d_n3, assign58850_e95809_d_n4, assign58850_e95809_d_n5, assign58850_e95809_d_n6, assign58850_e95809_d_n7, assign58850_e95809_d_n8, assign58850_e95809_d_n9, assign58850_e95809_d_n10, assign58850_e95809_d_n11,) = {
    if (var_guard492 == 0.0) {
        let assign58850_e95807: f64 = (p.p20 - p.p1);
        (assign58850_e95807, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11,)
    }
};
        var_t1 = assign58850_e95809;
        var_t1_dn3 = assign58850_e95809_d_n3;
        var_t1_dn4 = assign58850_e95809_d_n4;
        var_t1_dn5 = assign58850_e95809_d_n5;
        var_t1_dn6 = assign58850_e95809_d_n6;
        var_t1_dn7 = assign58850_e95809_d_n7;
        var_t1_dn8 = assign58850_e95809_d_n8;
        var_t1_dn9 = assign58850_e95809_d_n9;
        var_t1_dn10 = assign58850_e95809_d_n10;
        var_t1_dn11 = assign58850_e95809_d_n11;
        var_t1_rv = 0.0;

        let assign58860_e95812: f64 = if var_t1 > 0.0 { 1.0 } else { 0.0 };
        var_guard865 = assign58860_e95812;
        var_guard865_rv = 0.0;

        let (assign58870_e95821, assign58870_e95821_d_n3, assign58870_e95821_d_n4, assign58870_e95821_d_n5, assign58870_e95821_d_n6, assign58870_e95821_d_n7, assign58870_e95821_d_n8, assign58870_e95821_d_n9, assign58870_e95821_d_n10, assign58870_e95821_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard865 != 0.0)) {
        let assign58870_e95819: f64 = (var_t0 * var_t1);
        (assign58870_e95819, ((var_t0_dn3 * var_t1) + (var_t0 * var_t1_dn3)), ((var_t0_dn4 * var_t1) + (var_t0 * var_t1_dn4)), ((var_t0_dn5 * var_t1) + (var_t0 * var_t1_dn5)), ((var_t0_dn6 * var_t1) + (var_t0 * var_t1_dn6)), ((var_t0_dn7 * var_t1) + (var_t0 * var_t1_dn7)), ((var_t0_dn8 * var_t1) + (var_t0 * var_t1_dn8)), ((var_t0_dn9 * var_t1) + (var_t0 * var_t1_dn9)), ((var_t0_dn10 * var_t1) + (var_t0 * var_t1_dn10)), ((var_t0_dn11 * var_t1) + (var_t0 * var_t1_dn11)),)
    } else {
        (var_cdesw, var_cdesw_dn3, var_cdesw_dn4, var_cdesw_dn5, var_cdesw_dn6, var_cdesw_dn7, var_cdesw_dn8, var_cdesw_dn9, var_cdesw_dn10, var_cdesw_dn11,)
    }
};
        var_cdesw = assign58870_e95821;
        var_cdesw_dn3 = assign58870_e95821_d_n3;
        var_cdesw_dn4 = assign58870_e95821_d_n4;
        var_cdesw_dn5 = assign58870_e95821_d_n5;
        var_cdesw_dn6 = assign58870_e95821_d_n6;
        var_cdesw_dn7 = assign58870_e95821_d_n7;
        var_cdesw_dn8 = assign58870_e95821_d_n8;
        var_cdesw_dn9 = assign58870_e95821_d_n9;
        var_cdesw_dn10 = assign58870_e95821_d_n10;
        var_cdesw_dn11 = assign58870_e95821_d_n11;
        var_cdesw_rv = 0.0;

        let (assign58880_e95829, assign58880_e95829_d_n3, assign58880_e95829_d_n4, assign58880_e95829_d_n5, assign58880_e95829_d_n6, assign58880_e95829_d_n7, assign58880_e95829_d_n8, assign58880_e95829_d_n9, assign58880_e95829_d_n10, assign58880_e95829_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard865 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cdesw, var_cdesw_dn3, var_cdesw_dn4, var_cdesw_dn5, var_cdesw_dn6, var_cdesw_dn7, var_cdesw_dn8, var_cdesw_dn9, var_cdesw_dn10, var_cdesw_dn11,)
    }
};
        var_cdesw = assign58880_e95829;
        var_cdesw_dn3 = assign58880_e95829_d_n3;
        var_cdesw_dn4 = assign58880_e95829_d_n4;
        var_cdesw_dn5 = assign58880_e95829_d_n5;
        var_cdesw_dn6 = assign58880_e95829_d_n6;
        var_cdesw_dn7 = assign58880_e95829_d_n7;
        var_cdesw_dn8 = assign58880_e95829_d_n8;
        var_cdesw_dn9 = assign58880_e95829_d_n9;
        var_cdesw_dn10 = assign58880_e95829_d_n10;
        var_cdesw_dn11 = assign58880_e95829_d_n11;
        var_cdesw_rv = 0.0;

        let (assign58890_e95836,) = {
    if (var_guard492 == 0.0) {
        let assign58890_e95834: f64 = (var_cbox_1 * p.p17);
        (assign58890_e95834,)
    } else {
        (var_csbox,)
    }
};
        var_csbox = assign58890_e95836;
        var_csbox_rv = 0.0;

        let (assign58900_e95843,) = {
    if (var_guard492 == 0.0) {
        let assign58900_e95841: f64 = (p.p1396 * p.p17);
        (assign58900_e95841,)
    } else {
        (var_csmin,)
    }
};
        var_csmin = assign58900_e95843;
        var_csmin_rv = 0.0;

        let (assign58910_e95850,) = {
    if (var_guard492 == 0.0) {
        let assign58910_e95848: f64 = (var_cbox_1 * p.p18);
        (assign58910_e95848,)
    } else {
        (var_cdbox,)
    }
};
        var_cdbox = assign58910_e95850;
        var_cdbox_rv = 0.0;

        let (assign58920_e95857,) = {
    if (var_guard492 == 0.0) {
        let assign58920_e95855: f64 = (p.p1396 * p.p18);
        (assign58920_e95855,)
    } else {
        (var_cdmin,)
    }
};
        var_cdmin = assign58920_e95857;
        var_cdmin_rv = 0.0;

        let (assign58930_e95865, assign58930_e95865_d_n3, assign58930_e95865_d_n4, assign58930_e95865_d_n5, assign58930_e95865_d_n6, assign58930_e95865_d_n7, assign58930_e95865_d_n8, assign58930_e95865_d_n9, assign58930_e95865_d_n10, assign58930_e95865_d_n11,) = {
    if (var_guard492 == 0.0) {
        let assign58930_e95861: f64 = (-var_devsign);
        let assign58930_e95863: f64 = (assign58930_e95861 * var_ves_1);
        (assign58930_e95863, (assign58930_e95861 * var_ves_1_dn3), 0.0, 0.0, (assign58930_e95861 * var_ves_1_dn6), (assign58930_e95861 * var_ves_1_dn7), 0.0, 0.0, (assign58930_e95861 * var_ves_1_dn10), 0.0,)
    } else {
        (var_t10, var_t10_dn3, var_t10_dn4, var_t10_dn5, var_t10_dn6, var_t10_dn7, var_t10_dn8, var_t10_dn9, var_t10_dn10, var_t10_dn11,)
    }
};
        var_t10 = assign58930_e95865;
        var_t10_dn3 = assign58930_e95865_d_n3;
        var_t10_dn4 = assign58930_e95865_d_n4;
        var_t10_dn5 = assign58930_e95865_d_n5;
        var_t10_dn6 = assign58930_e95865_d_n6;
        var_t10_dn7 = assign58930_e95865_d_n7;
        var_t10_dn8 = assign58930_e95865_d_n8;
        var_t10_dn9 = assign58930_e95865_d_n9;
        var_t10_dn10 = assign58930_e95865_d_n10;
        var_t10_dn11 = assign58930_e95865_d_n11;
        var_t10_rv = 0.0;

        let (assign58940_e95873, assign58940_e95873_d_n3, assign58940_e95873_d_n4, assign58940_e95873_d_n5, assign58940_e95873_d_n6, assign58940_e95873_d_n7, assign58940_e95873_d_n8, assign58940_e95873_d_n9, assign58940_e95873_d_n10, assign58940_e95873_d_n11,) = {
    if (var_guard492 == 0.0) {
        let assign58940_e95869: f64 = (-var_devsign);
        let assign58940_e95871: f64 = (assign58940_e95869 * var_ved);
        (assign58940_e95871, (assign58940_e95869 * var_ved_dn3), 0.0, 0.0, (assign58940_e95869 * var_ved_dn6), (assign58940_e95869 * var_ved_dn7), 0.0, 0.0, (assign58940_e95869 * var_ved_dn10), 0.0,)
    } else {
        (var_t11, var_t11_dn3, var_t11_dn4, var_t11_dn5, var_t11_dn6, var_t11_dn7, var_t11_dn8, var_t11_dn9, var_t11_dn10, var_t11_dn11,)
    }
};
        var_t11 = assign58940_e95873;
        var_t11_dn3 = assign58940_e95873_d_n3;
        var_t11_dn4 = assign58940_e95873_d_n4;
        var_t11_dn5 = assign58940_e95873_d_n5;
        var_t11_dn6 = assign58940_e95873_d_n6;
        var_t11_dn7 = assign58940_e95873_d_n7;
        var_t11_dn8 = assign58940_e95873_d_n8;
        var_t11_dn9 = assign58940_e95873_d_n9;
        var_t11_dn10 = assign58940_e95873_d_n10;
        var_t11_dn11 = assign58940_e95873_d_n11;
        var_t11_rv = 0.0;

        let assign58950_e95876: f64 = if p.p1396 != 0.0 { 1.0 } else { 0.0 };
        var_guard866 = assign58950_e95876;
        var_guard866_rv = 0.0;

        let (assign58960_e95890, assign58960_e95890_d_n3, assign58960_e95890_d_n4, assign58960_e95890_d_n5, assign58960_e95890_d_n6, assign58960_e95890_d_n7, assign58960_e95890_d_n8, assign58960_e95890_d_n9, assign58960_e95890_d_n10, assign58960_e95890_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard866 != 0.0)) {
        let assign58960_e95882: f64 = (-0.5);
        let assign58960_e95885: f64 = (var_cdbox - var_cdmin);
        let assign58960_e95886: f64 = (assign58960_e95882 * assign58960_e95885);
        let assign58960_e95888: f64 = (assign58960_e95886 / p.p1399);
        (assign58960_e95888, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11,)
    }
};
        var_t1 = assign58960_e95890;
        var_t1_dn3 = assign58960_e95890_d_n3;
        var_t1_dn4 = assign58960_e95890_d_n4;
        var_t1_dn5 = assign58960_e95890_d_n5;
        var_t1_dn6 = assign58960_e95890_d_n6;
        var_t1_dn7 = assign58960_e95890_d_n7;
        var_t1_dn8 = assign58960_e95890_d_n8;
        var_t1_dn9 = assign58960_e95890_d_n9;
        var_t1_dn10 = assign58960_e95890_d_n10;
        var_t1_dn11 = assign58960_e95890_d_n11;
        var_t1_rv = 0.0;

        let (assign58970_e95906, assign58970_e95906_d_n3, assign58970_e95906_d_n4, assign58970_e95906_d_n5, assign58970_e95906_d_n6, assign58970_e95906_d_n7, assign58970_e95906_d_n8, assign58970_e95906_d_n9, assign58970_e95906_d_n10, assign58970_e95906_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard866 != 0.0)) {
        let assign58970_e95896: f64 = (-p.p1399);
        let assign58970_e95898: f64 = (assign58970_e95896 * var_t11);
        let assign58970_e95900: f64 = (assign58970_e95898 + p.p1400);
        let assign58970_e95901: f64 = (assign58970_e95900).cosh();
        let assign58970_e95903: f64 = (assign58970_e95901).max(1e-38);
        let assign58970_e95904: f64 = (assign58970_e95903).ln();
        (assign58970_e95904, (if assign58970_e95901 >= 1e-38 { ((assign58970_e95900).sinh() * (assign58970_e95896 * var_t11_dn3)) } else { 0.0 } / assign58970_e95903), (if assign58970_e95901 >= 1e-38 { ((assign58970_e95900).sinh() * (assign58970_e95896 * var_t11_dn4)) } else { 0.0 } / assign58970_e95903), (if assign58970_e95901 >= 1e-38 { ((assign58970_e95900).sinh() * (assign58970_e95896 * var_t11_dn5)) } else { 0.0 } / assign58970_e95903), (if assign58970_e95901 >= 1e-38 { ((assign58970_e95900).sinh() * (assign58970_e95896 * var_t11_dn6)) } else { 0.0 } / assign58970_e95903), (if assign58970_e95901 >= 1e-38 { ((assign58970_e95900).sinh() * (assign58970_e95896 * var_t11_dn7)) } else { 0.0 } / assign58970_e95903), (if assign58970_e95901 >= 1e-38 { ((assign58970_e95900).sinh() * (assign58970_e95896 * var_t11_dn8)) } else { 0.0 } / assign58970_e95903), (if assign58970_e95901 >= 1e-38 { ((assign58970_e95900).sinh() * (assign58970_e95896 * var_t11_dn9)) } else { 0.0 } / assign58970_e95903), (if assign58970_e95901 >= 1e-38 { ((assign58970_e95900).sinh() * (assign58970_e95896 * var_t11_dn10)) } else { 0.0 } / assign58970_e95903), (if assign58970_e95901 >= 1e-38 { ((assign58970_e95900).sinh() * (assign58970_e95896 * var_t11_dn11)) } else { 0.0 } / assign58970_e95903),)
    } else {
        (var_t2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11,)
    }
};
        var_t2 = assign58970_e95906;
        var_t2_dn3 = assign58970_e95906_d_n3;
        var_t2_dn4 = assign58970_e95906_d_n4;
        var_t2_dn5 = assign58970_e95906_d_n5;
        var_t2_dn6 = assign58970_e95906_d_n6;
        var_t2_dn7 = assign58970_e95906_d_n7;
        var_t2_dn8 = assign58970_e95906_d_n8;
        var_t2_dn9 = assign58970_e95906_d_n9;
        var_t2_dn10 = assign58970_e95906_d_n10;
        var_t2_dn11 = assign58970_e95906_d_n11;
        var_t2_rv = 0.0;

        let (assign58980_e95919, assign58980_e95919_d_n3, assign58980_e95919_d_n4, assign58980_e95919_d_n5, assign58980_e95919_d_n6, assign58980_e95919_d_n7, assign58980_e95919_d_n8, assign58980_e95919_d_n9, assign58980_e95919_d_n10, assign58980_e95919_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard866 != 0.0)) {
        let assign58980_e95914: f64 = (var_cdbox + var_cdmin);
        let assign58980_e95915: f64 = (0.5 * assign58980_e95914);
        let assign58980_e95917: f64 = (assign58980_e95915 * var_t11);
        (assign58980_e95917, (assign58980_e95915 * var_t11_dn3), (assign58980_e95915 * var_t11_dn4), (assign58980_e95915 * var_t11_dn5), (assign58980_e95915 * var_t11_dn6), (assign58980_e95915 * var_t11_dn7), (assign58980_e95915 * var_t11_dn8), (assign58980_e95915 * var_t11_dn9), (assign58980_e95915 * var_t11_dn10), (assign58980_e95915 * var_t11_dn11),)
    } else {
        (var_t3, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11,)
    }
};
        var_t3 = assign58980_e95919;
        var_t3_dn3 = assign58980_e95919_d_n3;
        var_t3_dn4 = assign58980_e95919_d_n4;
        var_t3_dn5 = assign58980_e95919_d_n5;
        var_t3_dn6 = assign58980_e95919_d_n6;
        var_t3_dn7 = assign58980_e95919_d_n7;
        var_t3_dn8 = assign58980_e95919_d_n8;
        var_t3_dn9 = assign58980_e95919_d_n9;
        var_t3_dn10 = assign58980_e95919_d_n10;
        var_t3_dn11 = assign58980_e95919_d_n11;
        var_t3_rv = 0.0;

        let (assign58990_e95930, assign58990_e95930_d_n3, assign58990_e95930_d_n4, assign58990_e95930_d_n5, assign58990_e95930_d_n6, assign58990_e95930_d_n7, assign58990_e95930_d_n8, assign58990_e95930_d_n9, assign58990_e95930_d_n10, assign58990_e95930_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard866 != 0.0)) {
        let assign58990_e95926: f64 = (var_t1 * var_t2);
        let assign58990_e95928: f64 = (assign58990_e95926 + var_t3);
        (assign58990_e95928, (((var_t1_dn3 * var_t2) + (var_t1 * var_t2_dn3)) + var_t3_dn3), (((var_t1_dn4 * var_t2) + (var_t1 * var_t2_dn4)) + var_t3_dn4), (((var_t1_dn5 * var_t2) + (var_t1 * var_t2_dn5)) + var_t3_dn5), (((var_t1_dn6 * var_t2) + (var_t1 * var_t2_dn6)) + var_t3_dn6), (((var_t1_dn7 * var_t2) + (var_t1 * var_t2_dn7)) + var_t3_dn7), (((var_t1_dn8 * var_t2) + (var_t1 * var_t2_dn8)) + var_t3_dn8), (((var_t1_dn9 * var_t2) + (var_t1 * var_t2_dn9)) + var_t3_dn9), (((var_t1_dn10 * var_t2) + (var_t1 * var_t2_dn10)) + var_t3_dn10), (((var_t1_dn11 * var_t2) + (var_t1 * var_t2_dn11)) + var_t3_dn11),)
    } else {
        (var_qde, var_qde_dn3, var_qde_dn4, var_qde_dn5, var_qde_dn6, var_qde_dn7, var_qde_dn8, var_qde_dn9, var_qde_dn10, var_qde_dn11,)
    }
};
        var_qde = assign58990_e95930;
        var_qde_dn3 = assign58990_e95930_d_n3;
        var_qde_dn4 = assign58990_e95930_d_n4;
        var_qde_dn5 = assign58990_e95930_d_n5;
        var_qde_dn6 = assign58990_e95930_d_n6;
        var_qde_dn7 = assign58990_e95930_d_n7;
        var_qde_dn8 = assign58990_e95930_d_n8;
        var_qde_dn9 = assign58990_e95930_d_n9;
        var_qde_dn10 = assign58990_e95930_d_n10;
        var_qde_dn11 = assign58990_e95930_d_n11;
        var_qde_rv = 0.0;

        let (assign59000_e95944, assign59000_e95944_d_n3, assign59000_e95944_d_n4, assign59000_e95944_d_n5, assign59000_e95944_d_n6, assign59000_e95944_d_n7, assign59000_e95944_d_n8, assign59000_e95944_d_n9, assign59000_e95944_d_n10, assign59000_e95944_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard866 != 0.0)) {
        let assign59000_e95936: f64 = (-0.5);
        let assign59000_e95939: f64 = (var_csbox - var_csmin);
        let assign59000_e95940: f64 = (assign59000_e95936 * assign59000_e95939);
        let assign59000_e95942: f64 = (assign59000_e95940 / p.p1397);
        (assign59000_e95942, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11,)
    }
};
        var_t1 = assign59000_e95944;
        var_t1_dn3 = assign59000_e95944_d_n3;
        var_t1_dn4 = assign59000_e95944_d_n4;
        var_t1_dn5 = assign59000_e95944_d_n5;
        var_t1_dn6 = assign59000_e95944_d_n6;
        var_t1_dn7 = assign59000_e95944_d_n7;
        var_t1_dn8 = assign59000_e95944_d_n8;
        var_t1_dn9 = assign59000_e95944_d_n9;
        var_t1_dn10 = assign59000_e95944_d_n10;
        var_t1_dn11 = assign59000_e95944_d_n11;
        var_t1_rv = 0.0;

        let (assign59010_e95960, assign59010_e95960_d_n3, assign59010_e95960_d_n4, assign59010_e95960_d_n5, assign59010_e95960_d_n6, assign59010_e95960_d_n7, assign59010_e95960_d_n8, assign59010_e95960_d_n9, assign59010_e95960_d_n10, assign59010_e95960_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard866 != 0.0)) {
        let assign59010_e95950: f64 = (-p.p1397);
        let assign59010_e95952: f64 = (assign59010_e95950 * var_t10);
        let assign59010_e95954: f64 = (assign59010_e95952 + p.p1398);
        let assign59010_e95955: f64 = (assign59010_e95954).cosh();
        let assign59010_e95957: f64 = (assign59010_e95955).max(1e-38);
        let assign59010_e95958: f64 = (assign59010_e95957).ln();
        (assign59010_e95958, (if assign59010_e95955 >= 1e-38 { ((assign59010_e95954).sinh() * (assign59010_e95950 * var_t10_dn3)) } else { 0.0 } / assign59010_e95957), (if assign59010_e95955 >= 1e-38 { ((assign59010_e95954).sinh() * (assign59010_e95950 * var_t10_dn4)) } else { 0.0 } / assign59010_e95957), (if assign59010_e95955 >= 1e-38 { ((assign59010_e95954).sinh() * (assign59010_e95950 * var_t10_dn5)) } else { 0.0 } / assign59010_e95957), (if assign59010_e95955 >= 1e-38 { ((assign59010_e95954).sinh() * (assign59010_e95950 * var_t10_dn6)) } else { 0.0 } / assign59010_e95957), (if assign59010_e95955 >= 1e-38 { ((assign59010_e95954).sinh() * (assign59010_e95950 * var_t10_dn7)) } else { 0.0 } / assign59010_e95957), (if assign59010_e95955 >= 1e-38 { ((assign59010_e95954).sinh() * (assign59010_e95950 * var_t10_dn8)) } else { 0.0 } / assign59010_e95957), (if assign59010_e95955 >= 1e-38 { ((assign59010_e95954).sinh() * (assign59010_e95950 * var_t10_dn9)) } else { 0.0 } / assign59010_e95957), (if assign59010_e95955 >= 1e-38 { ((assign59010_e95954).sinh() * (assign59010_e95950 * var_t10_dn10)) } else { 0.0 } / assign59010_e95957), (if assign59010_e95955 >= 1e-38 { ((assign59010_e95954).sinh() * (assign59010_e95950 * var_t10_dn11)) } else { 0.0 } / assign59010_e95957),)
    } else {
        (var_t2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11,)
    }
};
        var_t2 = assign59010_e95960;
        var_t2_dn3 = assign59010_e95960_d_n3;
        var_t2_dn4 = assign59010_e95960_d_n4;
        var_t2_dn5 = assign59010_e95960_d_n5;
        var_t2_dn6 = assign59010_e95960_d_n6;
        var_t2_dn7 = assign59010_e95960_d_n7;
        var_t2_dn8 = assign59010_e95960_d_n8;
        var_t2_dn9 = assign59010_e95960_d_n9;
        var_t2_dn10 = assign59010_e95960_d_n10;
        var_t2_dn11 = assign59010_e95960_d_n11;
        var_t2_rv = 0.0;

        let (assign59020_e95973, assign59020_e95973_d_n3, assign59020_e95973_d_n4, assign59020_e95973_d_n5, assign59020_e95973_d_n6, assign59020_e95973_d_n7, assign59020_e95973_d_n8, assign59020_e95973_d_n9, assign59020_e95973_d_n10, assign59020_e95973_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard866 != 0.0)) {
        let assign59020_e95968: f64 = (var_csbox + var_csmin);
        let assign59020_e95969: f64 = (0.5 * assign59020_e95968);
        let assign59020_e95971: f64 = (assign59020_e95969 * var_t10);
        (assign59020_e95971, (assign59020_e95969 * var_t10_dn3), (assign59020_e95969 * var_t10_dn4), (assign59020_e95969 * var_t10_dn5), (assign59020_e95969 * var_t10_dn6), (assign59020_e95969 * var_t10_dn7), (assign59020_e95969 * var_t10_dn8), (assign59020_e95969 * var_t10_dn9), (assign59020_e95969 * var_t10_dn10), (assign59020_e95969 * var_t10_dn11),)
    } else {
        (var_t3, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11,)
    }
};
        var_t3 = assign59020_e95973;
        var_t3_dn3 = assign59020_e95973_d_n3;
        var_t3_dn4 = assign59020_e95973_d_n4;
        var_t3_dn5 = assign59020_e95973_d_n5;
        var_t3_dn6 = assign59020_e95973_d_n6;
        var_t3_dn7 = assign59020_e95973_d_n7;
        var_t3_dn8 = assign59020_e95973_d_n8;
        var_t3_dn9 = assign59020_e95973_d_n9;
        var_t3_dn10 = assign59020_e95973_d_n10;
        var_t3_dn11 = assign59020_e95973_d_n11;
        var_t3_rv = 0.0;

        let (assign59030_e95984, assign59030_e95984_d_n3, assign59030_e95984_d_n4, assign59030_e95984_d_n5, assign59030_e95984_d_n6, assign59030_e95984_d_n7, assign59030_e95984_d_n8, assign59030_e95984_d_n9, assign59030_e95984_d_n10, assign59030_e95984_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard866 != 0.0)) {
        let assign59030_e95980: f64 = (var_t1 * var_t2);
        let assign59030_e95982: f64 = (assign59030_e95980 + var_t3);
        (assign59030_e95982, (((var_t1_dn3 * var_t2) + (var_t1 * var_t2_dn3)) + var_t3_dn3), (((var_t1_dn4 * var_t2) + (var_t1 * var_t2_dn4)) + var_t3_dn4), (((var_t1_dn5 * var_t2) + (var_t1 * var_t2_dn5)) + var_t3_dn5), (((var_t1_dn6 * var_t2) + (var_t1 * var_t2_dn6)) + var_t3_dn6), (((var_t1_dn7 * var_t2) + (var_t1 * var_t2_dn7)) + var_t3_dn7), (((var_t1_dn8 * var_t2) + (var_t1 * var_t2_dn8)) + var_t3_dn8), (((var_t1_dn9 * var_t2) + (var_t1 * var_t2_dn9)) + var_t3_dn9), (((var_t1_dn10 * var_t2) + (var_t1 * var_t2_dn10)) + var_t3_dn10), (((var_t1_dn11 * var_t2) + (var_t1 * var_t2_dn11)) + var_t3_dn11),)
    } else {
        (var_qse, var_qse_dn3, var_qse_dn4, var_qse_dn5, var_qse_dn6, var_qse_dn7, var_qse_dn8, var_qse_dn9, var_qse_dn10, var_qse_dn11,)
    }
};
        var_qse = assign59030_e95984;
        var_qse_dn3 = assign59030_e95984_d_n3;
        var_qse_dn4 = assign59030_e95984_d_n4;
        var_qse_dn5 = assign59030_e95984_d_n5;
        var_qse_dn6 = assign59030_e95984_d_n6;
        var_qse_dn7 = assign59030_e95984_d_n7;
        var_qse_dn8 = assign59030_e95984_d_n8;
        var_qse_dn9 = assign59030_e95984_d_n9;
        var_qse_dn10 = assign59030_e95984_d_n10;
        var_qse_dn11 = assign59030_e95984_d_n11;
        var_qse_rv = 0.0;

        let (assign59040_e95994, assign59040_e95994_d_n3, assign59040_e95994_d_n4, assign59040_e95994_d_n5, assign59040_e95994_d_n6, assign59040_e95994_d_n7, assign59040_e95994_d_n8, assign59040_e95994_d_n9, assign59040_e95994_d_n10, assign59040_e95994_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard866 == 0.0)) {
        let assign59040_e95992: f64 = (var_csbox * var_t10);
        (assign59040_e95992, (var_csbox * var_t10_dn3), (var_csbox * var_t10_dn4), (var_csbox * var_t10_dn5), (var_csbox * var_t10_dn6), (var_csbox * var_t10_dn7), (var_csbox * var_t10_dn8), (var_csbox * var_t10_dn9), (var_csbox * var_t10_dn10), (var_csbox * var_t10_dn11),)
    } else {
        (var_qse, var_qse_dn3, var_qse_dn4, var_qse_dn5, var_qse_dn6, var_qse_dn7, var_qse_dn8, var_qse_dn9, var_qse_dn10, var_qse_dn11,)
    }
};
        var_qse = assign59040_e95994;
        var_qse_dn3 = assign59040_e95994_d_n3;
        var_qse_dn4 = assign59040_e95994_d_n4;
        var_qse_dn5 = assign59040_e95994_d_n5;
        var_qse_dn6 = assign59040_e95994_d_n6;
        var_qse_dn7 = assign59040_e95994_d_n7;
        var_qse_dn8 = assign59040_e95994_d_n8;
        var_qse_dn9 = assign59040_e95994_d_n9;
        var_qse_dn10 = assign59040_e95994_d_n10;
        var_qse_dn11 = assign59040_e95994_d_n11;
        var_qse_rv = 0.0;

        let (assign59050_e96004, assign59050_e96004_d_n3, assign59050_e96004_d_n4, assign59050_e96004_d_n5, assign59050_e96004_d_n6, assign59050_e96004_d_n7, assign59050_e96004_d_n8, assign59050_e96004_d_n9, assign59050_e96004_d_n10, assign59050_e96004_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard866 == 0.0)) {
        let assign59050_e96002: f64 = (var_cdbox * var_t11);
        (assign59050_e96002, (var_cdbox * var_t11_dn3), (var_cdbox * var_t11_dn4), (var_cdbox * var_t11_dn5), (var_cdbox * var_t11_dn6), (var_cdbox * var_t11_dn7), (var_cdbox * var_t11_dn8), (var_cdbox * var_t11_dn9), (var_cdbox * var_t11_dn10), (var_cdbox * var_t11_dn11),)
    } else {
        (var_qde, var_qde_dn3, var_qde_dn4, var_qde_dn5, var_qde_dn6, var_qde_dn7, var_qde_dn8, var_qde_dn9, var_qde_dn10, var_qde_dn11,)
    }
};
        var_qde = assign59050_e96004;
        var_qde_dn3 = assign59050_e96004_d_n3;
        var_qde_dn4 = assign59050_e96004_d_n4;
        var_qde_dn5 = assign59050_e96004_d_n5;
        var_qde_dn6 = assign59050_e96004_d_n6;
        var_qde_dn7 = assign59050_e96004_d_n7;
        var_qde_dn8 = assign59050_e96004_d_n8;
        var_qde_dn9 = assign59050_e96004_d_n9;
        var_qde_dn10 = assign59050_e96004_d_n10;
        var_qde_dn11 = assign59050_e96004_d_n11;
        var_qde_rv = 0.0;

        *var_cdbox_slot = var_cdbox;
        *var_cdbox_rv_slot = var_cdbox_rv;
        *var_cdesw_slot = var_cdesw;
        *var_cdesw_dn10_slot = var_cdesw_dn10;
        *var_cdesw_dn11_slot = var_cdesw_dn11;
        *var_cdesw_dn3_slot = var_cdesw_dn3;
        *var_cdesw_dn4_slot = var_cdesw_dn4;
        *var_cdesw_dn5_slot = var_cdesw_dn5;
        *var_cdesw_dn6_slot = var_cdesw_dn6;
        *var_cdesw_dn7_slot = var_cdesw_dn7;
        *var_cdesw_dn8_slot = var_cdesw_dn8;
        *var_cdesw_dn9_slot = var_cdesw_dn9;
        *var_cdesw_rv_slot = var_cdesw_rv;
        *var_cdmin_slot = var_cdmin;
        *var_cdmin_rv_slot = var_cdmin_rv;
        *var_csbox_slot = var_csbox;
        *var_csbox_rv_slot = var_csbox_rv;
        *var_csesw_slot = var_csesw;
        *var_csesw_dn10_slot = var_csesw_dn10;
        *var_csesw_dn11_slot = var_csesw_dn11;
        *var_csesw_dn3_slot = var_csesw_dn3;
        *var_csesw_dn4_slot = var_csesw_dn4;
        *var_csesw_dn5_slot = var_csesw_dn5;
        *var_csesw_dn6_slot = var_csesw_dn6;
        *var_csesw_dn7_slot = var_csesw_dn7;
        *var_csesw_dn8_slot = var_csesw_dn8;
        *var_csesw_dn9_slot = var_csesw_dn9;
        *var_csesw_rv_slot = var_csesw_rv;
        *var_csmin_slot = var_csmin;
        *var_csmin_rv_slot = var_csmin_rv;
        *var_guard864_slot = var_guard864;
        *var_guard864_rv_slot = var_guard864_rv;
        *var_guard865_slot = var_guard865;
        *var_guard865_rv_slot = var_guard865_rv;
        *var_guard866_slot = var_guard866;
        *var_guard866_rv_slot = var_guard866_rv;
        *var_qde_slot = var_qde;
        *var_qde_dn10_slot = var_qde_dn10;
        *var_qde_dn11_slot = var_qde_dn11;
        *var_qde_dn3_slot = var_qde_dn3;
        *var_qde_dn4_slot = var_qde_dn4;
        *var_qde_dn5_slot = var_qde_dn5;
        *var_qde_dn6_slot = var_qde_dn6;
        *var_qde_dn7_slot = var_qde_dn7;
        *var_qde_dn8_slot = var_qde_dn8;
        *var_qde_dn9_slot = var_qde_dn9;
        *var_qde_rv_slot = var_qde_rv;
        *var_qe1_slot = var_qe1;
        *var_qe1_dn10_slot = var_qe1_dn10;
        *var_qe1_dn11_slot = var_qe1_dn11;
        *var_qe1_dn3_slot = var_qe1_dn3;
        *var_qe1_dn4_slot = var_qe1_dn4;
        *var_qe1_dn5_slot = var_qe1_dn5;
        *var_qe1_dn6_slot = var_qe1_dn6;
        *var_qe1_dn7_slot = var_qe1_dn7;
        *var_qe1_dn8_slot = var_qe1_dn8;
        *var_qe1_dn9_slot = var_qe1_dn9;
        *var_qe1_rv_slot = var_qe1_rv;
        *var_qse_slot = var_qse;
        *var_qse_dn10_slot = var_qse_dn10;
        *var_qse_dn11_slot = var_qse_dn11;
        *var_qse_dn3_slot = var_qse_dn3;
        *var_qse_dn4_slot = var_qse_dn4;
        *var_qse_dn5_slot = var_qse_dn5;
        *var_qse_dn6_slot = var_qse_dn6;
        *var_qse_dn7_slot = var_qse_dn7;
        *var_qse_dn8_slot = var_qse_dn8;
        *var_qse_dn9_slot = var_qse_dn9;
        *var_qse_rv_slot = var_qse_rv;
        *var_qsub_slot = var_qsub;
        *var_qsub_dn10_slot = var_qsub_dn10;
        *var_qsub_dn11_slot = var_qsub_dn11;
        *var_qsub_dn3_slot = var_qsub_dn3;
        *var_qsub_dn4_slot = var_qsub_dn4;
        *var_qsub_dn5_slot = var_qsub_dn5;
        *var_qsub_dn6_slot = var_qsub_dn6;
        *var_qsub_dn7_slot = var_qsub_dn7;
        *var_qsub_dn8_slot = var_qsub_dn8;
        *var_qsub_dn9_slot = var_qsub_dn9;
        *var_qsub_rv_slot = var_qsub_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_t0_rv_slot = var_t0_rv;
        *var_t1_slot = var_t1;
        *var_t10_slot = var_t10;
        *var_t10_dn10_slot = var_t10_dn10;
        *var_t10_dn11_slot = var_t10_dn11;
        *var_t10_dn3_slot = var_t10_dn3;
        *var_t10_dn4_slot = var_t10_dn4;
        *var_t10_dn5_slot = var_t10_dn5;
        *var_t10_dn6_slot = var_t10_dn6;
        *var_t10_dn7_slot = var_t10_dn7;
        *var_t10_dn8_slot = var_t10_dn8;
        *var_t10_dn9_slot = var_t10_dn9;
        *var_t10_rv_slot = var_t10_rv;
        *var_t11_slot = var_t11;
        *var_t11_dn10_slot = var_t11_dn10;
        *var_t11_dn11_slot = var_t11_dn11;
        *var_t11_dn3_slot = var_t11_dn3;
        *var_t11_dn4_slot = var_t11_dn4;
        *var_t11_dn5_slot = var_t11_dn5;
        *var_t11_dn6_slot = var_t11_dn6;
        *var_t11_dn7_slot = var_t11_dn7;
        *var_t11_dn8_slot = var_t11_dn8;
        *var_t11_dn9_slot = var_t11_dn9;
        *var_t11_rv_slot = var_t11_rv;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t1_rv_slot = var_t1_rv;
        *var_t2_slot = var_t2;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_t2_rv_slot = var_t2_rv;
        *var_t3_slot = var_t3;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn3_slot = var_t3_dn3;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_dn9_slot = var_t3_dn9;
        *var_t3_rv_slot = var_t3_rv;
    }

    pub(super) fn stamp_reactive_block_206(
        p: &Parameters,
        var_cdesw: f64,
        var_cdesw_dn10: f64,
        var_cdesw_dn11: f64,
        var_cdesw_dn3: f64,
        var_cdesw_dn4: f64,
        var_cdesw_dn5: f64,
        var_cdesw_dn6: f64,
        var_cdesw_dn7: f64,
        var_cdesw_dn8: f64,
        var_cdesw_dn9: f64,
        var_cdscbedge_i: f64,
        var_cdscdedge_a: f64,
        var_cdscdedge_a_dn10: f64,
        var_cdscdedge_a_dn11: f64,
        var_cdscdedge_a_dn3: f64,
        var_cdscdedge_a_dn4: f64,
        var_cdscdedge_a_dn5: f64,
        var_cdscdedge_a_dn6: f64,
        var_cdscdedge_a_dn7: f64,
        var_cdscdedge_a_dn8: f64,
        var_cdscdedge_a_dn9: f64,
        var_citedge_i: f64,
        var_cox: f64,
        var_csesw: f64,
        var_csesw_dn10: f64,
        var_csesw_dn11: f64,
        var_csesw_dn3: f64,
        var_csesw_dn4: f64,
        var_csesw_dn5: f64,
        var_csesw_dn6: f64,
        var_csesw_dn7: f64,
        var_csesw_dn8: f64,
        var_csesw_dn9: f64,
        var_epssi: f64,
        var_eta0edge_i: f64,
        var_eta0edge_i_dn10: f64,
        var_eta0edge_i_dn11: f64,
        var_eta0edge_i_dn3: f64,
        var_eta0edge_i_dn4: f64,
        var_eta0edge_i_dn5: f64,
        var_eta0edge_i_dn6: f64,
        var_eta0edge_i_dn7: f64,
        var_eta0edge_i_dn8: f64,
        var_eta0edge_i_dn9: f64,
        var_etabedge_i: f64,
        var_guard492: f64,
        var_kt1edge_i: f64,
        var_kt1expedge_i: f64,
        var_kt1ledge_i: f64,
        var_kt2edge_i: f64,
        var_leff: f64,
        var_litl: f64,
        var_ndepedge_i: f64,
        var_nfactoredge_i: f64,
        var_ni: f64,
        var_ni_dn10: f64,
        var_ni_dn11: f64,
        var_ni_dn3: f64,
        var_ni_dn4: f64,
        var_ni_dn5: f64,
        var_ni_dn6: f64,
        var_ni_dn7: f64,
        var_ni_dn8: f64,
        var_ni_dn9: f64,
        var_phin_i: f64,
        var_t10: f64,
        var_t10_dn10: f64,
        var_t10_dn11: f64,
        var_t10_dn3: f64,
        var_t10_dn4: f64,
        var_t10_dn5: f64,
        var_t10_dn6: f64,
        var_t10_dn7: f64,
        var_t10_dn8: f64,
        var_t10_dn9: f64,
        var_t11: f64,
        var_t11_dn10: f64,
        var_t11_dn11: f64,
        var_t11_dn3: f64,
        var_t11_dn4: f64,
        var_t11_dn5: f64,
        var_t11_dn6: f64,
        var_t11_dn7: f64,
        var_t11_dn8: f64,
        var_t11_dn9: f64,
        var_teta0edge_i: f64,
        var_tnfactoredge_i: f64,
        var_tratio: f64,
        var_tratio_dn4: f64,
        var_tratio_dn5: f64,
        var_vbsx: f64,
        var_vbsx_dn10: f64,
        var_vbsx_dn11: f64,
        var_vbsx_dn3: f64,
        var_vbsx_dn4: f64,
        var_vbsx_dn5: f64,
        var_vbsx_dn6: f64,
        var_vbsx_dn7: f64,
        var_vbsx_dn8: f64,
        var_vbsx_dn9: f64,
        var_vdsx: f64,
        var_vdsx_dn10: f64,
        var_vdsx_dn11: f64,
        var_vdsx_dn3: f64,
        var_vdsx_dn4: f64,
        var_vdsx_dn5: f64,
        var_vdsx_dn6: f64,
        var_vdsx_dn7: f64,
        var_vdsx_dn8: f64,
        var_vdsx_dn9: f64,
        var_vfb_i: f64,
        var_vfb_i_dn10: f64,
        var_vfb_i_dn11: f64,
        var_vfb_i_dn3: f64,
        var_vfb_i_dn4: f64,
        var_vfb_i_dn5: f64,
        var_vfb_i_dn6: f64,
        var_vfb_i_dn7: f64,
        var_vfb_i_dn8: f64,
        var_vfb_i_dn9: f64,
        var_vg: f64,
        var_vg_dn10: f64,
        var_vg_dn8: f64,
        var_vs: f64,
        var_vs_dn10: f64,
        var_vs_dn6: f64,
        var_vs_dn7: f64,
        var_vt: f64,
        var_vt_dn4: f64,
        var_vt_dn5: f64,
        var_cdep_slot: &mut f64,
        var_cdep_dn10_slot: &mut f64,
        var_cdep_dn11_slot: &mut f64,
        var_cdep_dn3_slot: &mut f64,
        var_cdep_dn4_slot: &mut f64,
        var_cdep_dn5_slot: &mut f64,
        var_cdep_dn6_slot: &mut f64,
        var_cdep_dn7_slot: &mut f64,
        var_cdep_dn8_slot: &mut f64,
        var_cdep_dn9_slot: &mut f64,
        var_cdep_rv_slot: &mut f64,
        var_cdsc_slot: &mut f64,
        var_cdsc_dn10_slot: &mut f64,
        var_cdsc_dn11_slot: &mut f64,
        var_cdsc_dn3_slot: &mut f64,
        var_cdsc_dn4_slot: &mut f64,
        var_cdsc_dn5_slot: &mut f64,
        var_cdsc_dn6_slot: &mut f64,
        var_cdsc_dn7_slot: &mut f64,
        var_cdsc_dn8_slot: &mut f64,
        var_cdsc_dn9_slot: &mut f64,
        var_cdsc_rv_slot: &mut f64,
        var_dvth_dibl_1_slot: &mut f64,
        var_dvth_dibl_1_dn10_slot: &mut f64,
        var_dvth_dibl_1_dn11_slot: &mut f64,
        var_dvth_dibl_1_dn3_slot: &mut f64,
        var_dvth_dibl_1_dn4_slot: &mut f64,
        var_dvth_dibl_1_dn5_slot: &mut f64,
        var_dvth_dibl_1_dn6_slot: &mut f64,
        var_dvth_dibl_1_dn7_slot: &mut f64,
        var_dvth_dibl_1_dn8_slot: &mut f64,
        var_dvth_dibl_1_dn9_slot: &mut f64,
        var_dvth_dibl_1_rv_slot: &mut f64,
        var_dvth_temp_slot: &mut f64,
        var_dvth_temp_dn10_slot: &mut f64,
        var_dvth_temp_dn11_slot: &mut f64,
        var_dvth_temp_dn3_slot: &mut f64,
        var_dvth_temp_dn4_slot: &mut f64,
        var_dvth_temp_dn5_slot: &mut f64,
        var_dvth_temp_dn6_slot: &mut f64,
        var_dvth_temp_dn7_slot: &mut f64,
        var_dvth_temp_dn8_slot: &mut f64,
        var_dvth_temp_dn9_slot: &mut f64,
        var_dvth_temp_rv_slot: &mut f64,
        var_eta0edge_t_slot: &mut f64,
        var_eta0edge_t_dn10_slot: &mut f64,
        var_eta0edge_t_dn11_slot: &mut f64,
        var_eta0edge_t_dn3_slot: &mut f64,
        var_eta0edge_t_dn4_slot: &mut f64,
        var_eta0edge_t_dn5_slot: &mut f64,
        var_eta0edge_t_dn6_slot: &mut f64,
        var_eta0edge_t_dn7_slot: &mut f64,
        var_eta0edge_t_dn8_slot: &mut f64,
        var_eta0edge_t_dn9_slot: &mut f64,
        var_eta0edge_t_rv_slot: &mut f64,
        var_guard867_slot: &mut f64,
        var_guard867_rv_slot: &mut f64,
        var_guard868_slot: &mut f64,
        var_guard868_rv_slot: &mut f64,
        var_inv_nvt_slot: &mut f64,
        var_inv_nvt_dn10_slot: &mut f64,
        var_inv_nvt_dn11_slot: &mut f64,
        var_inv_nvt_dn3_slot: &mut f64,
        var_inv_nvt_dn4_slot: &mut f64,
        var_inv_nvt_dn5_slot: &mut f64,
        var_inv_nvt_dn6_slot: &mut f64,
        var_inv_nvt_dn7_slot: &mut f64,
        var_inv_nvt_dn8_slot: &mut f64,
        var_inv_nvt_dn9_slot: &mut f64,
        var_inv_nvt_rv_slot: &mut f64,
        var_litl_edge_slot: &mut f64,
        var_litl_edge_dn10_slot: &mut f64,
        var_litl_edge_dn11_slot: &mut f64,
        var_litl_edge_dn3_slot: &mut f64,
        var_litl_edge_dn4_slot: &mut f64,
        var_litl_edge_dn5_slot: &mut f64,
        var_litl_edge_dn6_slot: &mut f64,
        var_litl_edge_dn7_slot: &mut f64,
        var_litl_edge_dn8_slot: &mut f64,
        var_litl_edge_dn9_slot: &mut f64,
        var_litl_edge_rv_slot: &mut f64,
        var_n_slot: &mut f64,
        var_n_dn10_slot: &mut f64,
        var_n_dn11_slot: &mut f64,
        var_n_dn3_slot: &mut f64,
        var_n_dn4_slot: &mut f64,
        var_n_dn5_slot: &mut f64,
        var_n_dn6_slot: &mut f64,
        var_n_dn7_slot: &mut f64,
        var_n_dn8_slot: &mut f64,
        var_n_dn9_slot: &mut f64,
        var_n_rv_slot: &mut f64,
        var_nfactoredge_t_slot: &mut f64,
        var_nfactoredge_t_dn4_slot: &mut f64,
        var_nfactoredge_t_dn5_slot: &mut f64,
        var_nfactoredge_t_rv_slot: &mut f64,
        var_nvt_slot: &mut f64,
        var_nvt_dn10_slot: &mut f64,
        var_nvt_dn11_slot: &mut f64,
        var_nvt_dn3_slot: &mut f64,
        var_nvt_dn4_slot: &mut f64,
        var_nvt_dn5_slot: &mut f64,
        var_nvt_dn6_slot: &mut f64,
        var_nvt_dn7_slot: &mut f64,
        var_nvt_dn8_slot: &mut f64,
        var_nvt_dn9_slot: &mut f64,
        var_nvt_rv_slot: &mut f64,
        var_phib_edge_slot: &mut f64,
        var_phib_edge_dn10_slot: &mut f64,
        var_phib_edge_dn11_slot: &mut f64,
        var_phib_edge_dn3_slot: &mut f64,
        var_phib_edge_dn4_slot: &mut f64,
        var_phib_edge_dn5_slot: &mut f64,
        var_phib_edge_dn6_slot: &mut f64,
        var_phib_edge_dn7_slot: &mut f64,
        var_phib_edge_dn8_slot: &mut f64,
        var_phib_edge_dn9_slot: &mut f64,
        var_phib_edge_rv_slot: &mut f64,
        var_phist_slot: &mut f64,
        var_phist_dn10_slot: &mut f64,
        var_phist_dn11_slot: &mut f64,
        var_phist_dn3_slot: &mut f64,
        var_phist_dn4_slot: &mut f64,
        var_phist_dn5_slot: &mut f64,
        var_phist_dn6_slot: &mut f64,
        var_phist_dn7_slot: &mut f64,
        var_phist_dn8_slot: &mut f64,
        var_phist_dn9_slot: &mut f64,
        var_phist_rv_slot: &mut f64,
        var_phistvbs_slot: &mut f64,
        var_phistvbs_dn10_slot: &mut f64,
        var_phistvbs_dn11_slot: &mut f64,
        var_phistvbs_dn3_slot: &mut f64,
        var_phistvbs_dn4_slot: &mut f64,
        var_phistvbs_dn5_slot: &mut f64,
        var_phistvbs_dn6_slot: &mut f64,
        var_phistvbs_dn7_slot: &mut f64,
        var_phistvbs_dn8_slot: &mut f64,
        var_phistvbs_dn9_slot: &mut f64,
        var_phistvbs_rv_slot: &mut f64,
        var_qde_slot: &mut f64,
        var_qde_dn10_slot: &mut f64,
        var_qde_dn11_slot: &mut f64,
        var_qde_dn3_slot: &mut f64,
        var_qde_dn4_slot: &mut f64,
        var_qde_dn5_slot: &mut f64,
        var_qde_dn6_slot: &mut f64,
        var_qde_dn7_slot: &mut f64,
        var_qde_dn8_slot: &mut f64,
        var_qde_dn9_slot: &mut f64,
        var_qde_rv_slot: &mut f64,
        var_qse_slot: &mut f64,
        var_qse_dn10_slot: &mut f64,
        var_qse_dn11_slot: &mut f64,
        var_qse_dn3_slot: &mut f64,
        var_qse_dn4_slot: &mut f64,
        var_qse_dn5_slot: &mut f64,
        var_qse_dn6_slot: &mut f64,
        var_qse_dn7_slot: &mut f64,
        var_qse_dn8_slot: &mut f64,
        var_qse_dn9_slot: &mut f64,
        var_qse_rv_slot: &mut f64,
        var_sqrtphistvbs_slot: &mut f64,
        var_sqrtphistvbs_dn10_slot: &mut f64,
        var_sqrtphistvbs_dn11_slot: &mut f64,
        var_sqrtphistvbs_dn3_slot: &mut f64,
        var_sqrtphistvbs_dn4_slot: &mut f64,
        var_sqrtphistvbs_dn5_slot: &mut f64,
        var_sqrtphistvbs_dn6_slot: &mut f64,
        var_sqrtphistvbs_dn7_slot: &mut f64,
        var_sqrtphistvbs_dn8_slot: &mut f64,
        var_sqrtphistvbs_dn9_slot: &mut f64,
        var_sqrtphistvbs_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t1dep_slot: &mut f64,
        var_t1dep_dn10_slot: &mut f64,
        var_t1dep_dn11_slot: &mut f64,
        var_t1dep_dn3_slot: &mut f64,
        var_t1dep_dn4_slot: &mut f64,
        var_t1dep_dn5_slot: &mut f64,
        var_t1dep_dn6_slot: &mut f64,
        var_t1dep_dn7_slot: &mut f64,
        var_t1dep_dn8_slot: &mut f64,
        var_t1dep_dn9_slot: &mut f64,
        var_t1dep_rv_slot: &mut f64,
        var_vfb_slot: &mut f64,
        var_vfb_dn10_slot: &mut f64,
        var_vfb_dn11_slot: &mut f64,
        var_vfb_dn3_slot: &mut f64,
        var_vfb_dn4_slot: &mut f64,
        var_vfb_dn5_slot: &mut f64,
        var_vfb_dn6_slot: &mut f64,
        var_vfb_dn7_slot: &mut f64,
        var_vfb_dn8_slot: &mut f64,
        var_vfb_dn9_slot: &mut f64,
        var_vfb_rv_slot: &mut f64,
        var_vg_1_slot: &mut f64,
        var_vg_1_dn10_slot: &mut f64,
        var_vg_1_dn11_slot: &mut f64,
        var_vg_1_dn3_slot: &mut f64,
        var_vg_1_dn4_slot: &mut f64,
        var_vg_1_dn5_slot: &mut f64,
        var_vg_1_dn6_slot: &mut f64,
        var_vg_1_dn7_slot: &mut f64,
        var_vg_1_dn8_slot: &mut f64,
        var_vg_1_dn9_slot: &mut f64,
        var_vg_1_rv_slot: &mut f64,
        var_vs_1_slot: &mut f64,
        var_vs_1_dn10_slot: &mut f64,
        var_vs_1_dn11_slot: &mut f64,
        var_vs_1_dn3_slot: &mut f64,
        var_vs_1_dn4_slot: &mut f64,
        var_vs_1_dn5_slot: &mut f64,
        var_vs_1_dn6_slot: &mut f64,
        var_vs_1_dn7_slot: &mut f64,
        var_vs_1_dn8_slot: &mut f64,
        var_vs_1_dn9_slot: &mut f64,
        var_vs_1_rv_slot: &mut f64,
        var_xdep_slot: &mut f64,
        var_xdep_dn10_slot: &mut f64,
        var_xdep_dn11_slot: &mut f64,
        var_xdep_dn3_slot: &mut f64,
        var_xdep_dn4_slot: &mut f64,
        var_xdep_dn5_slot: &mut f64,
        var_xdep_dn6_slot: &mut f64,
        var_xdep_dn7_slot: &mut f64,
        var_xdep_dn8_slot: &mut f64,
        var_xdep_dn9_slot: &mut f64,
        var_xdep_rv_slot: &mut f64,
    ) {
        let mut var_cdep: f64 = *var_cdep_slot;
        let mut var_cdep_dn10: f64 = *var_cdep_dn10_slot;
        let mut var_cdep_dn11: f64 = *var_cdep_dn11_slot;
        let mut var_cdep_dn3: f64 = *var_cdep_dn3_slot;
        let mut var_cdep_dn4: f64 = *var_cdep_dn4_slot;
        let mut var_cdep_dn5: f64 = *var_cdep_dn5_slot;
        let mut var_cdep_dn6: f64 = *var_cdep_dn6_slot;
        let mut var_cdep_dn7: f64 = *var_cdep_dn7_slot;
        let mut var_cdep_dn8: f64 = *var_cdep_dn8_slot;
        let mut var_cdep_dn9: f64 = *var_cdep_dn9_slot;
        let mut var_cdep_rv: f64 = *var_cdep_rv_slot;
        let mut var_cdsc: f64 = *var_cdsc_slot;
        let mut var_cdsc_dn10: f64 = *var_cdsc_dn10_slot;
        let mut var_cdsc_dn11: f64 = *var_cdsc_dn11_slot;
        let mut var_cdsc_dn3: f64 = *var_cdsc_dn3_slot;
        let mut var_cdsc_dn4: f64 = *var_cdsc_dn4_slot;
        let mut var_cdsc_dn5: f64 = *var_cdsc_dn5_slot;
        let mut var_cdsc_dn6: f64 = *var_cdsc_dn6_slot;
        let mut var_cdsc_dn7: f64 = *var_cdsc_dn7_slot;
        let mut var_cdsc_dn8: f64 = *var_cdsc_dn8_slot;
        let mut var_cdsc_dn9: f64 = *var_cdsc_dn9_slot;
        let mut var_cdsc_rv: f64 = *var_cdsc_rv_slot;
        let mut var_dvth_dibl_1: f64 = *var_dvth_dibl_1_slot;
        let mut var_dvth_dibl_1_dn10: f64 = *var_dvth_dibl_1_dn10_slot;
        let mut var_dvth_dibl_1_dn11: f64 = *var_dvth_dibl_1_dn11_slot;
        let mut var_dvth_dibl_1_dn3: f64 = *var_dvth_dibl_1_dn3_slot;
        let mut var_dvth_dibl_1_dn4: f64 = *var_dvth_dibl_1_dn4_slot;
        let mut var_dvth_dibl_1_dn5: f64 = *var_dvth_dibl_1_dn5_slot;
        let mut var_dvth_dibl_1_dn6: f64 = *var_dvth_dibl_1_dn6_slot;
        let mut var_dvth_dibl_1_dn7: f64 = *var_dvth_dibl_1_dn7_slot;
        let mut var_dvth_dibl_1_dn8: f64 = *var_dvth_dibl_1_dn8_slot;
        let mut var_dvth_dibl_1_dn9: f64 = *var_dvth_dibl_1_dn9_slot;
        let mut var_dvth_dibl_1_rv: f64 = *var_dvth_dibl_1_rv_slot;
        let mut var_dvth_temp: f64 = *var_dvth_temp_slot;
        let mut var_dvth_temp_dn10: f64 = *var_dvth_temp_dn10_slot;
        let mut var_dvth_temp_dn11: f64 = *var_dvth_temp_dn11_slot;
        let mut var_dvth_temp_dn3: f64 = *var_dvth_temp_dn3_slot;
        let mut var_dvth_temp_dn4: f64 = *var_dvth_temp_dn4_slot;
        let mut var_dvth_temp_dn5: f64 = *var_dvth_temp_dn5_slot;
        let mut var_dvth_temp_dn6: f64 = *var_dvth_temp_dn6_slot;
        let mut var_dvth_temp_dn7: f64 = *var_dvth_temp_dn7_slot;
        let mut var_dvth_temp_dn8: f64 = *var_dvth_temp_dn8_slot;
        let mut var_dvth_temp_dn9: f64 = *var_dvth_temp_dn9_slot;
        let mut var_dvth_temp_rv: f64 = *var_dvth_temp_rv_slot;
        let mut var_eta0edge_t: f64 = *var_eta0edge_t_slot;
        let mut var_eta0edge_t_dn10: f64 = *var_eta0edge_t_dn10_slot;
        let mut var_eta0edge_t_dn11: f64 = *var_eta0edge_t_dn11_slot;
        let mut var_eta0edge_t_dn3: f64 = *var_eta0edge_t_dn3_slot;
        let mut var_eta0edge_t_dn4: f64 = *var_eta0edge_t_dn4_slot;
        let mut var_eta0edge_t_dn5: f64 = *var_eta0edge_t_dn5_slot;
        let mut var_eta0edge_t_dn6: f64 = *var_eta0edge_t_dn6_slot;
        let mut var_eta0edge_t_dn7: f64 = *var_eta0edge_t_dn7_slot;
        let mut var_eta0edge_t_dn8: f64 = *var_eta0edge_t_dn8_slot;
        let mut var_eta0edge_t_dn9: f64 = *var_eta0edge_t_dn9_slot;
        let mut var_eta0edge_t_rv: f64 = *var_eta0edge_t_rv_slot;
        let mut var_guard867: f64 = *var_guard867_slot;
        let mut var_guard867_rv: f64 = *var_guard867_rv_slot;
        let mut var_guard868: f64 = *var_guard868_slot;
        let mut var_guard868_rv: f64 = *var_guard868_rv_slot;
        let mut var_inv_nvt: f64 = *var_inv_nvt_slot;
        let mut var_inv_nvt_dn10: f64 = *var_inv_nvt_dn10_slot;
        let mut var_inv_nvt_dn11: f64 = *var_inv_nvt_dn11_slot;
        let mut var_inv_nvt_dn3: f64 = *var_inv_nvt_dn3_slot;
        let mut var_inv_nvt_dn4: f64 = *var_inv_nvt_dn4_slot;
        let mut var_inv_nvt_dn5: f64 = *var_inv_nvt_dn5_slot;
        let mut var_inv_nvt_dn6: f64 = *var_inv_nvt_dn6_slot;
        let mut var_inv_nvt_dn7: f64 = *var_inv_nvt_dn7_slot;
        let mut var_inv_nvt_dn8: f64 = *var_inv_nvt_dn8_slot;
        let mut var_inv_nvt_dn9: f64 = *var_inv_nvt_dn9_slot;
        let mut var_inv_nvt_rv: f64 = *var_inv_nvt_rv_slot;
        let mut var_litl_edge: f64 = *var_litl_edge_slot;
        let mut var_litl_edge_dn10: f64 = *var_litl_edge_dn10_slot;
        let mut var_litl_edge_dn11: f64 = *var_litl_edge_dn11_slot;
        let mut var_litl_edge_dn3: f64 = *var_litl_edge_dn3_slot;
        let mut var_litl_edge_dn4: f64 = *var_litl_edge_dn4_slot;
        let mut var_litl_edge_dn5: f64 = *var_litl_edge_dn5_slot;
        let mut var_litl_edge_dn6: f64 = *var_litl_edge_dn6_slot;
        let mut var_litl_edge_dn7: f64 = *var_litl_edge_dn7_slot;
        let mut var_litl_edge_dn8: f64 = *var_litl_edge_dn8_slot;
        let mut var_litl_edge_dn9: f64 = *var_litl_edge_dn9_slot;
        let mut var_litl_edge_rv: f64 = *var_litl_edge_rv_slot;
        let mut var_n: f64 = *var_n_slot;
        let mut var_n_dn10: f64 = *var_n_dn10_slot;
        let mut var_n_dn11: f64 = *var_n_dn11_slot;
        let mut var_n_dn3: f64 = *var_n_dn3_slot;
        let mut var_n_dn4: f64 = *var_n_dn4_slot;
        let mut var_n_dn5: f64 = *var_n_dn5_slot;
        let mut var_n_dn6: f64 = *var_n_dn6_slot;
        let mut var_n_dn7: f64 = *var_n_dn7_slot;
        let mut var_n_dn8: f64 = *var_n_dn8_slot;
        let mut var_n_dn9: f64 = *var_n_dn9_slot;
        let mut var_n_rv: f64 = *var_n_rv_slot;
        let mut var_nfactoredge_t: f64 = *var_nfactoredge_t_slot;
        let mut var_nfactoredge_t_dn4: f64 = *var_nfactoredge_t_dn4_slot;
        let mut var_nfactoredge_t_dn5: f64 = *var_nfactoredge_t_dn5_slot;
        let mut var_nfactoredge_t_rv: f64 = *var_nfactoredge_t_rv_slot;
        let mut var_nvt: f64 = *var_nvt_slot;
        let mut var_nvt_dn10: f64 = *var_nvt_dn10_slot;
        let mut var_nvt_dn11: f64 = *var_nvt_dn11_slot;
        let mut var_nvt_dn3: f64 = *var_nvt_dn3_slot;
        let mut var_nvt_dn4: f64 = *var_nvt_dn4_slot;
        let mut var_nvt_dn5: f64 = *var_nvt_dn5_slot;
        let mut var_nvt_dn6: f64 = *var_nvt_dn6_slot;
        let mut var_nvt_dn7: f64 = *var_nvt_dn7_slot;
        let mut var_nvt_dn8: f64 = *var_nvt_dn8_slot;
        let mut var_nvt_dn9: f64 = *var_nvt_dn9_slot;
        let mut var_nvt_rv: f64 = *var_nvt_rv_slot;
        let mut var_phib_edge: f64 = *var_phib_edge_slot;
        let mut var_phib_edge_dn10: f64 = *var_phib_edge_dn10_slot;
        let mut var_phib_edge_dn11: f64 = *var_phib_edge_dn11_slot;
        let mut var_phib_edge_dn3: f64 = *var_phib_edge_dn3_slot;
        let mut var_phib_edge_dn4: f64 = *var_phib_edge_dn4_slot;
        let mut var_phib_edge_dn5: f64 = *var_phib_edge_dn5_slot;
        let mut var_phib_edge_dn6: f64 = *var_phib_edge_dn6_slot;
        let mut var_phib_edge_dn7: f64 = *var_phib_edge_dn7_slot;
        let mut var_phib_edge_dn8: f64 = *var_phib_edge_dn8_slot;
        let mut var_phib_edge_dn9: f64 = *var_phib_edge_dn9_slot;
        let mut var_phib_edge_rv: f64 = *var_phib_edge_rv_slot;
        let mut var_phist: f64 = *var_phist_slot;
        let mut var_phist_dn10: f64 = *var_phist_dn10_slot;
        let mut var_phist_dn11: f64 = *var_phist_dn11_slot;
        let mut var_phist_dn3: f64 = *var_phist_dn3_slot;
        let mut var_phist_dn4: f64 = *var_phist_dn4_slot;
        let mut var_phist_dn5: f64 = *var_phist_dn5_slot;
        let mut var_phist_dn6: f64 = *var_phist_dn6_slot;
        let mut var_phist_dn7: f64 = *var_phist_dn7_slot;
        let mut var_phist_dn8: f64 = *var_phist_dn8_slot;
        let mut var_phist_dn9: f64 = *var_phist_dn9_slot;
        let mut var_phist_rv: f64 = *var_phist_rv_slot;
        let mut var_phistvbs: f64 = *var_phistvbs_slot;
        let mut var_phistvbs_dn10: f64 = *var_phistvbs_dn10_slot;
        let mut var_phistvbs_dn11: f64 = *var_phistvbs_dn11_slot;
        let mut var_phistvbs_dn3: f64 = *var_phistvbs_dn3_slot;
        let mut var_phistvbs_dn4: f64 = *var_phistvbs_dn4_slot;
        let mut var_phistvbs_dn5: f64 = *var_phistvbs_dn5_slot;
        let mut var_phistvbs_dn6: f64 = *var_phistvbs_dn6_slot;
        let mut var_phistvbs_dn7: f64 = *var_phistvbs_dn7_slot;
        let mut var_phistvbs_dn8: f64 = *var_phistvbs_dn8_slot;
        let mut var_phistvbs_dn9: f64 = *var_phistvbs_dn9_slot;
        let mut var_phistvbs_rv: f64 = *var_phistvbs_rv_slot;
        let mut var_qde: f64 = *var_qde_slot;
        let mut var_qde_dn10: f64 = *var_qde_dn10_slot;
        let mut var_qde_dn11: f64 = *var_qde_dn11_slot;
        let mut var_qde_dn3: f64 = *var_qde_dn3_slot;
        let mut var_qde_dn4: f64 = *var_qde_dn4_slot;
        let mut var_qde_dn5: f64 = *var_qde_dn5_slot;
        let mut var_qde_dn6: f64 = *var_qde_dn6_slot;
        let mut var_qde_dn7: f64 = *var_qde_dn7_slot;
        let mut var_qde_dn8: f64 = *var_qde_dn8_slot;
        let mut var_qde_dn9: f64 = *var_qde_dn9_slot;
        let mut var_qde_rv: f64 = *var_qde_rv_slot;
        let mut var_qse: f64 = *var_qse_slot;
        let mut var_qse_dn10: f64 = *var_qse_dn10_slot;
        let mut var_qse_dn11: f64 = *var_qse_dn11_slot;
        let mut var_qse_dn3: f64 = *var_qse_dn3_slot;
        let mut var_qse_dn4: f64 = *var_qse_dn4_slot;
        let mut var_qse_dn5: f64 = *var_qse_dn5_slot;
        let mut var_qse_dn6: f64 = *var_qse_dn6_slot;
        let mut var_qse_dn7: f64 = *var_qse_dn7_slot;
        let mut var_qse_dn8: f64 = *var_qse_dn8_slot;
        let mut var_qse_dn9: f64 = *var_qse_dn9_slot;
        let mut var_qse_rv: f64 = *var_qse_rv_slot;
        let mut var_sqrtphistvbs: f64 = *var_sqrtphistvbs_slot;
        let mut var_sqrtphistvbs_dn10: f64 = *var_sqrtphistvbs_dn10_slot;
        let mut var_sqrtphistvbs_dn11: f64 = *var_sqrtphistvbs_dn11_slot;
        let mut var_sqrtphistvbs_dn3: f64 = *var_sqrtphistvbs_dn3_slot;
        let mut var_sqrtphistvbs_dn4: f64 = *var_sqrtphistvbs_dn4_slot;
        let mut var_sqrtphistvbs_dn5: f64 = *var_sqrtphistvbs_dn5_slot;
        let mut var_sqrtphistvbs_dn6: f64 = *var_sqrtphistvbs_dn6_slot;
        let mut var_sqrtphistvbs_dn7: f64 = *var_sqrtphistvbs_dn7_slot;
        let mut var_sqrtphistvbs_dn8: f64 = *var_sqrtphistvbs_dn8_slot;
        let mut var_sqrtphistvbs_dn9: f64 = *var_sqrtphistvbs_dn9_slot;
        let mut var_sqrtphistvbs_rv: f64 = *var_sqrtphistvbs_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t1dep: f64 = *var_t1dep_slot;
        let mut var_t1dep_dn10: f64 = *var_t1dep_dn10_slot;
        let mut var_t1dep_dn11: f64 = *var_t1dep_dn11_slot;
        let mut var_t1dep_dn3: f64 = *var_t1dep_dn3_slot;
        let mut var_t1dep_dn4: f64 = *var_t1dep_dn4_slot;
        let mut var_t1dep_dn5: f64 = *var_t1dep_dn5_slot;
        let mut var_t1dep_dn6: f64 = *var_t1dep_dn6_slot;
        let mut var_t1dep_dn7: f64 = *var_t1dep_dn7_slot;
        let mut var_t1dep_dn8: f64 = *var_t1dep_dn8_slot;
        let mut var_t1dep_dn9: f64 = *var_t1dep_dn9_slot;
        let mut var_t1dep_rv: f64 = *var_t1dep_rv_slot;
        let mut var_vfb: f64 = *var_vfb_slot;
        let mut var_vfb_dn10: f64 = *var_vfb_dn10_slot;
        let mut var_vfb_dn11: f64 = *var_vfb_dn11_slot;
        let mut var_vfb_dn3: f64 = *var_vfb_dn3_slot;
        let mut var_vfb_dn4: f64 = *var_vfb_dn4_slot;
        let mut var_vfb_dn5: f64 = *var_vfb_dn5_slot;
        let mut var_vfb_dn6: f64 = *var_vfb_dn6_slot;
        let mut var_vfb_dn7: f64 = *var_vfb_dn7_slot;
        let mut var_vfb_dn8: f64 = *var_vfb_dn8_slot;
        let mut var_vfb_dn9: f64 = *var_vfb_dn9_slot;
        let mut var_vfb_rv: f64 = *var_vfb_rv_slot;
        let mut var_vg_1: f64 = *var_vg_1_slot;
        let mut var_vg_1_dn10: f64 = *var_vg_1_dn10_slot;
        let mut var_vg_1_dn11: f64 = *var_vg_1_dn11_slot;
        let mut var_vg_1_dn3: f64 = *var_vg_1_dn3_slot;
        let mut var_vg_1_dn4: f64 = *var_vg_1_dn4_slot;
        let mut var_vg_1_dn5: f64 = *var_vg_1_dn5_slot;
        let mut var_vg_1_dn6: f64 = *var_vg_1_dn6_slot;
        let mut var_vg_1_dn7: f64 = *var_vg_1_dn7_slot;
        let mut var_vg_1_dn8: f64 = *var_vg_1_dn8_slot;
        let mut var_vg_1_dn9: f64 = *var_vg_1_dn9_slot;
        let mut var_vg_1_rv: f64 = *var_vg_1_rv_slot;
        let mut var_vs_1: f64 = *var_vs_1_slot;
        let mut var_vs_1_dn10: f64 = *var_vs_1_dn10_slot;
        let mut var_vs_1_dn11: f64 = *var_vs_1_dn11_slot;
        let mut var_vs_1_dn3: f64 = *var_vs_1_dn3_slot;
        let mut var_vs_1_dn4: f64 = *var_vs_1_dn4_slot;
        let mut var_vs_1_dn5: f64 = *var_vs_1_dn5_slot;
        let mut var_vs_1_dn6: f64 = *var_vs_1_dn6_slot;
        let mut var_vs_1_dn7: f64 = *var_vs_1_dn7_slot;
        let mut var_vs_1_dn8: f64 = *var_vs_1_dn8_slot;
        let mut var_vs_1_dn9: f64 = *var_vs_1_dn9_slot;
        let mut var_vs_1_rv: f64 = *var_vs_1_rv_slot;
        let mut var_xdep: f64 = *var_xdep_slot;
        let mut var_xdep_dn10: f64 = *var_xdep_dn10_slot;
        let mut var_xdep_dn11: f64 = *var_xdep_dn11_slot;
        let mut var_xdep_dn3: f64 = *var_xdep_dn3_slot;
        let mut var_xdep_dn4: f64 = *var_xdep_dn4_slot;
        let mut var_xdep_dn5: f64 = *var_xdep_dn5_slot;
        let mut var_xdep_dn6: f64 = *var_xdep_dn6_slot;
        let mut var_xdep_dn7: f64 = *var_xdep_dn7_slot;
        let mut var_xdep_dn8: f64 = *var_xdep_dn8_slot;
        let mut var_xdep_dn9: f64 = *var_xdep_dn9_slot;
        let mut var_xdep_rv: f64 = *var_xdep_rv_slot;

        let (assign59060_e96013, assign59060_e96013_d_n3, assign59060_e96013_d_n4, assign59060_e96013_d_n5, assign59060_e96013_d_n6, assign59060_e96013_d_n7, assign59060_e96013_d_n8, assign59060_e96013_d_n9, assign59060_e96013_d_n10, assign59060_e96013_d_n11,) = {
    if (var_guard492 == 0.0) {
        let assign59060_e96010: f64 = (var_csesw * var_t10);
        let assign59060_e96011: f64 = (var_qse + assign59060_e96010);
        (assign59060_e96011, (var_qse_dn3 + ((var_csesw_dn3 * var_t10) + (var_csesw * var_t10_dn3))), (var_qse_dn4 + ((var_csesw_dn4 * var_t10) + (var_csesw * var_t10_dn4))), (var_qse_dn5 + ((var_csesw_dn5 * var_t10) + (var_csesw * var_t10_dn5))), (var_qse_dn6 + ((var_csesw_dn6 * var_t10) + (var_csesw * var_t10_dn6))), (var_qse_dn7 + ((var_csesw_dn7 * var_t10) + (var_csesw * var_t10_dn7))), (var_qse_dn8 + ((var_csesw_dn8 * var_t10) + (var_csesw * var_t10_dn8))), (var_qse_dn9 + ((var_csesw_dn9 * var_t10) + (var_csesw * var_t10_dn9))), (var_qse_dn10 + ((var_csesw_dn10 * var_t10) + (var_csesw * var_t10_dn10))), (var_qse_dn11 + ((var_csesw_dn11 * var_t10) + (var_csesw * var_t10_dn11))),)
    } else {
        (var_qse, var_qse_dn3, var_qse_dn4, var_qse_dn5, var_qse_dn6, var_qse_dn7, var_qse_dn8, var_qse_dn9, var_qse_dn10, var_qse_dn11,)
    }
};
        var_qse = assign59060_e96013;
        var_qse_dn3 = assign59060_e96013_d_n3;
        var_qse_dn4 = assign59060_e96013_d_n4;
        var_qse_dn5 = assign59060_e96013_d_n5;
        var_qse_dn6 = assign59060_e96013_d_n6;
        var_qse_dn7 = assign59060_e96013_d_n7;
        var_qse_dn8 = assign59060_e96013_d_n8;
        var_qse_dn9 = assign59060_e96013_d_n9;
        var_qse_dn10 = assign59060_e96013_d_n10;
        var_qse_dn11 = assign59060_e96013_d_n11;
        var_qse_rv = 0.0;

        let (assign59070_e96022, assign59070_e96022_d_n3, assign59070_e96022_d_n4, assign59070_e96022_d_n5, assign59070_e96022_d_n6, assign59070_e96022_d_n7, assign59070_e96022_d_n8, assign59070_e96022_d_n9, assign59070_e96022_d_n10, assign59070_e96022_d_n11,) = {
    if (var_guard492 == 0.0) {
        let assign59070_e96019: f64 = (var_cdesw * var_t11);
        let assign59070_e96020: f64 = (var_qde + assign59070_e96019);
        (assign59070_e96020, (var_qde_dn3 + ((var_cdesw_dn3 * var_t11) + (var_cdesw * var_t11_dn3))), (var_qde_dn4 + ((var_cdesw_dn4 * var_t11) + (var_cdesw * var_t11_dn4))), (var_qde_dn5 + ((var_cdesw_dn5 * var_t11) + (var_cdesw * var_t11_dn5))), (var_qde_dn6 + ((var_cdesw_dn6 * var_t11) + (var_cdesw * var_t11_dn6))), (var_qde_dn7 + ((var_cdesw_dn7 * var_t11) + (var_cdesw * var_t11_dn7))), (var_qde_dn8 + ((var_cdesw_dn8 * var_t11) + (var_cdesw * var_t11_dn8))), (var_qde_dn9 + ((var_cdesw_dn9 * var_t11) + (var_cdesw * var_t11_dn9))), (var_qde_dn10 + ((var_cdesw_dn10 * var_t11) + (var_cdesw * var_t11_dn10))), (var_qde_dn11 + ((var_cdesw_dn11 * var_t11) + (var_cdesw * var_t11_dn11))),)
    } else {
        (var_qde, var_qde_dn3, var_qde_dn4, var_qde_dn5, var_qde_dn6, var_qde_dn7, var_qde_dn8, var_qde_dn9, var_qde_dn10, var_qde_dn11,)
    }
};
        var_qde = assign59070_e96022;
        var_qde_dn3 = assign59070_e96022_d_n3;
        var_qde_dn4 = assign59070_e96022_d_n4;
        var_qde_dn5 = assign59070_e96022_d_n5;
        var_qde_dn6 = assign59070_e96022_d_n6;
        var_qde_dn7 = assign59070_e96022_d_n7;
        var_qde_dn8 = assign59070_e96022_d_n8;
        var_qde_dn9 = assign59070_e96022_d_n9;
        var_qde_dn10 = assign59070_e96022_d_n10;
        var_qde_dn11 = assign59070_e96022_d_n11;
        var_qde_rv = 0.0;

        let assign59080_e96025: f64 = if p.p27 == 1.0 { 1.0 } else { 0.0 };
        var_guard867 = assign59080_e96025;
        var_guard867_rv = 0.0;

        let (assign59090_e96037, assign59090_e96037_d_n3, assign59090_e96037_d_n4, assign59090_e96037_d_n5, assign59090_e96037_d_n6, assign59090_e96037_d_n7, assign59090_e96037_d_n8, assign59090_e96037_d_n9, assign59090_e96037_d_n10, assign59090_e96037_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59090_e96032: f64 = (var_ndepedge_i / var_ni);
        let assign59090_e96034: f64 = (assign59090_e96032).max(1e-38);
        let assign59090_e96035: f64 = (assign59090_e96034).ln();
        (assign59090_e96035, (if assign59090_e96032 >= 1e-38 { (-((var_ndepedge_i * var_ni_dn3) / (var_ni * var_ni))) } else { 0.0 } / assign59090_e96034), (if assign59090_e96032 >= 1e-38 { (-((var_ndepedge_i * var_ni_dn4) / (var_ni * var_ni))) } else { 0.0 } / assign59090_e96034), (if assign59090_e96032 >= 1e-38 { (-((var_ndepedge_i * var_ni_dn5) / (var_ni * var_ni))) } else { 0.0 } / assign59090_e96034), (if assign59090_e96032 >= 1e-38 { (-((var_ndepedge_i * var_ni_dn6) / (var_ni * var_ni))) } else { 0.0 } / assign59090_e96034), (if assign59090_e96032 >= 1e-38 { (-((var_ndepedge_i * var_ni_dn7) / (var_ni * var_ni))) } else { 0.0 } / assign59090_e96034), (if assign59090_e96032 >= 1e-38 { (-((var_ndepedge_i * var_ni_dn8) / (var_ni * var_ni))) } else { 0.0 } / assign59090_e96034), (if assign59090_e96032 >= 1e-38 { (-((var_ndepedge_i * var_ni_dn9) / (var_ni * var_ni))) } else { 0.0 } / assign59090_e96034), (if assign59090_e96032 >= 1e-38 { (-((var_ndepedge_i * var_ni_dn10) / (var_ni * var_ni))) } else { 0.0 } / assign59090_e96034), (if assign59090_e96032 >= 1e-38 { (-((var_ndepedge_i * var_ni_dn11) / (var_ni * var_ni))) } else { 0.0 } / assign59090_e96034),)
    } else {
        (var_phib_edge, var_phib_edge_dn3, var_phib_edge_dn4, var_phib_edge_dn5, var_phib_edge_dn6, var_phib_edge_dn7, var_phib_edge_dn8, var_phib_edge_dn9, var_phib_edge_dn10, var_phib_edge_dn11,)
    }
};
        var_phib_edge = assign59090_e96037;
        var_phib_edge_dn3 = assign59090_e96037_d_n3;
        var_phib_edge_dn4 = assign59090_e96037_d_n4;
        var_phib_edge_dn5 = assign59090_e96037_d_n5;
        var_phib_edge_dn6 = assign59090_e96037_d_n6;
        var_phib_edge_dn7 = assign59090_e96037_d_n7;
        var_phib_edge_dn8 = assign59090_e96037_d_n8;
        var_phib_edge_dn9 = assign59090_e96037_d_n9;
        var_phib_edge_dn10 = assign59090_e96037_d_n10;
        var_phib_edge_dn11 = assign59090_e96037_d_n11;
        var_phib_edge_rv = 0.0;

        let (assign59100_e96052, assign59100_e96052_d_n3, assign59100_e96052_d_n4, assign59100_e96052_d_n5, assign59100_e96052_d_n6, assign59100_e96052_d_n7, assign59100_e96052_d_n8, assign59100_e96052_d_n9, assign59100_e96052_d_n10, assign59100_e96052_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59100_e96045: f64 = (var_vt * var_phib_edge);
        let assign59100_e96046: f64 = (0.4 + assign59100_e96045);
        let assign59100_e96048: f64 = (assign59100_e96046 + var_phin_i);
        let assign59100_e96050: f64 = (assign59100_e96048).max(0.4);
        (assign59100_e96050, if assign59100_e96048 >= 0.4 { (var_vt * var_phib_edge_dn3) } else { 0.0 }, if assign59100_e96048 >= 0.4 { ((var_vt_dn4 * var_phib_edge) + (var_vt * var_phib_edge_dn4)) } else { 0.0 }, if assign59100_e96048 >= 0.4 { ((var_vt_dn5 * var_phib_edge) + (var_vt * var_phib_edge_dn5)) } else { 0.0 }, if assign59100_e96048 >= 0.4 { (var_vt * var_phib_edge_dn6) } else { 0.0 }, if assign59100_e96048 >= 0.4 { (var_vt * var_phib_edge_dn7) } else { 0.0 }, if assign59100_e96048 >= 0.4 { (var_vt * var_phib_edge_dn8) } else { 0.0 }, if assign59100_e96048 >= 0.4 { (var_vt * var_phib_edge_dn9) } else { 0.0 }, if assign59100_e96048 >= 0.4 { (var_vt * var_phib_edge_dn10) } else { 0.0 }, if assign59100_e96048 >= 0.4 { (var_vt * var_phib_edge_dn11) } else { 0.0 },)
    } else {
        (var_phist, var_phist_dn3, var_phist_dn4, var_phist_dn5, var_phist_dn6, var_phist_dn7, var_phist_dn8, var_phist_dn9, var_phist_dn10, var_phist_dn11,)
    }
};
        var_phist = assign59100_e96052;
        var_phist_dn3 = assign59100_e96052_d_n3;
        var_phist_dn4 = assign59100_e96052_d_n4;
        var_phist_dn5 = assign59100_e96052_d_n5;
        var_phist_dn6 = assign59100_e96052_d_n6;
        var_phist_dn7 = assign59100_e96052_d_n7;
        var_phist_dn8 = assign59100_e96052_d_n8;
        var_phist_dn9 = assign59100_e96052_d_n9;
        var_phist_dn10 = assign59100_e96052_d_n10;
        var_phist_dn11 = assign59100_e96052_d_n11;
        var_phist_rv = 0.0;

        let (assign59110_e96066, assign59110_e96066_d_n3, assign59110_e96066_d_n4, assign59110_e96066_d_n5, assign59110_e96066_d_n6, assign59110_e96066_d_n7, assign59110_e96066_d_n8, assign59110_e96066_d_n9, assign59110_e96066_d_n10, assign59110_e96066_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59110_e96059: f64 = (2.0 * var_epssi);
        let assign59110_e96062: f64 = (1.602176462e-19 * var_ndepedge_i);
        let assign59110_e96063: f64 = (assign59110_e96059 / assign59110_e96062);
        let assign59110_e96064: f64 = (assign59110_e96063).sqrt();
        (assign59110_e96064, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1dep, var_t1dep_dn3, var_t1dep_dn4, var_t1dep_dn5, var_t1dep_dn6, var_t1dep_dn7, var_t1dep_dn8, var_t1dep_dn9, var_t1dep_dn10, var_t1dep_dn11,)
    }
};
        var_t1dep = assign59110_e96066;
        var_t1dep_dn3 = assign59110_e96066_d_n3;
        var_t1dep_dn4 = assign59110_e96066_d_n4;
        var_t1dep_dn5 = assign59110_e96066_d_n5;
        var_t1dep_dn6 = assign59110_e96066_d_n6;
        var_t1dep_dn7 = assign59110_e96066_d_n7;
        var_t1dep_dn8 = assign59110_e96066_d_n8;
        var_t1dep_dn9 = assign59110_e96066_d_n9;
        var_t1dep_dn10 = assign59110_e96066_d_n10;
        var_t1dep_dn11 = assign59110_e96066_d_n11;
        var_t1dep_rv = 0.0;

        let (assign59120_e96106, assign59120_e96106_d_n4, assign59120_e96106_d_n5,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59120_e96077: f64 = (var_tratio - 1.0);
        let assign59120_e96078: f64 = (var_tnfactoredge_i * assign59120_e96077);
        let assign59120_e96079: f64 = (1.0 + assign59120_e96078);
        let assign59120_e96084: f64 = (var_tratio - 1.0);
        let assign59120_e96085: f64 = (var_tnfactoredge_i * assign59120_e96084);
        let assign59120_e96086: f64 = (1.0 + assign59120_e96085);
        let assign59120_e96091: f64 = (var_tratio - 1.0);
        let assign59120_e96092: f64 = (var_tnfactoredge_i * assign59120_e96091);
        let assign59120_e96093: f64 = (1.0 + assign59120_e96092);
        let assign59120_e96094: f64 = (assign59120_e96086 * assign59120_e96093);
        let assign59120_e96097: f64 = (4.0 * 0.001);
        let assign59120_e96099: f64 = (assign59120_e96097 * 0.001);
        let assign59120_e96100: f64 = (assign59120_e96094 + assign59120_e96099);
        let assign59120_e96101: f64 = (assign59120_e96100).sqrt();
        let assign59120_e96102: f64 = (assign59120_e96079 + assign59120_e96101);
        let assign59120_e96103: f64 = (0.5 * assign59120_e96102);
        let assign59120_e96104: f64 = (var_nfactoredge_i * assign59120_e96103);
        (assign59120_e96104, (var_nfactoredge_i * (0.5 * ((var_tnfactoredge_i * var_tratio_dn4) + ((((var_tnfactoredge_i * var_tratio_dn4) * assign59120_e96093) + (assign59120_e96086 * (var_tnfactoredge_i * var_tratio_dn4))) / (2.0 * assign59120_e96101))))), (var_nfactoredge_i * (0.5 * ((var_tnfactoredge_i * var_tratio_dn5) + ((((var_tnfactoredge_i * var_tratio_dn5) * assign59120_e96093) + (assign59120_e96086 * (var_tnfactoredge_i * var_tratio_dn5))) / (2.0 * assign59120_e96101))))),)
    } else {
        (var_nfactoredge_t, var_nfactoredge_t_dn4, var_nfactoredge_t_dn5,)
    }
};
        var_nfactoredge_t = assign59120_e96106;
        var_nfactoredge_t_dn4 = assign59120_e96106_d_n4;
        var_nfactoredge_t_dn5 = assign59120_e96106_d_n5;
        var_nfactoredge_t_rv = 0.0;

        let (assign59130_e96121, assign59130_e96121_d_n3, assign59130_e96121_d_n4, assign59130_e96121_d_n5, assign59130_e96121_d_n6, assign59130_e96121_d_n7, assign59130_e96121_d_n8, assign59130_e96121_d_n9, assign59130_e96121_d_n10, assign59130_e96121_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59130_e96116: f64 = (var_tratio - 1.0);
        let assign59130_e96117: f64 = (var_teta0edge_i * assign59130_e96116);
        let assign59130_e96118: f64 = (1.0 + assign59130_e96117);
        let assign59130_e96119: f64 = (var_eta0edge_i * assign59130_e96118);
        (assign59130_e96119, (var_eta0edge_i_dn3 * assign59130_e96118), ((var_eta0edge_i_dn4 * assign59130_e96118) + (var_eta0edge_i * (var_teta0edge_i * var_tratio_dn4))), ((var_eta0edge_i_dn5 * assign59130_e96118) + (var_eta0edge_i * (var_teta0edge_i * var_tratio_dn5))), (var_eta0edge_i_dn6 * assign59130_e96118), (var_eta0edge_i_dn7 * assign59130_e96118), (var_eta0edge_i_dn8 * assign59130_e96118), (var_eta0edge_i_dn9 * assign59130_e96118), (var_eta0edge_i_dn10 * assign59130_e96118), (var_eta0edge_i_dn11 * assign59130_e96118),)
    } else {
        (var_eta0edge_t, var_eta0edge_t_dn3, var_eta0edge_t_dn4, var_eta0edge_t_dn5, var_eta0edge_t_dn6, var_eta0edge_t_dn7, var_eta0edge_t_dn8, var_eta0edge_t_dn9, var_eta0edge_t_dn10, var_eta0edge_t_dn11,)
    }
};
        var_eta0edge_t = assign59130_e96121;
        var_eta0edge_t_dn3 = assign59130_e96121_d_n3;
        var_eta0edge_t_dn4 = assign59130_e96121_d_n4;
        var_eta0edge_t_dn5 = assign59130_e96121_d_n5;
        var_eta0edge_t_dn6 = assign59130_e96121_d_n6;
        var_eta0edge_t_dn7 = assign59130_e96121_d_n7;
        var_eta0edge_t_dn8 = assign59130_e96121_d_n8;
        var_eta0edge_t_dn9 = assign59130_e96121_d_n9;
        var_eta0edge_t_dn10 = assign59130_e96121_d_n10;
        var_eta0edge_t_dn11 = assign59130_e96121_d_n11;
        var_eta0edge_t_rv = 0.0;

        let (assign59140_e96153, assign59140_e96153_d_n3, assign59140_e96153_d_n4, assign59140_e96153_d_n5, assign59140_e96153_d_n6, assign59140_e96153_d_n7, assign59140_e96153_d_n8, assign59140_e96153_d_n9, assign59140_e96153_d_n10, assign59140_e96153_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59140_e96129: f64 = (var_phist - var_vbsx);
        let assign59140_e96131: f64 = (assign59140_e96129 + 0.05);
        let assign59140_e96134: f64 = (var_phist - var_vbsx);
        let assign59140_e96136: f64 = (assign59140_e96134 - 0.05);
        let assign59140_e96139: f64 = (var_phist - var_vbsx);
        let assign59140_e96141: f64 = (assign59140_e96139 - 0.05);
        let assign59140_e96142: f64 = (assign59140_e96136 * assign59140_e96141);
        let assign59140_e96145: f64 = (0.25 * 0.1);
        let assign59140_e96147: f64 = (assign59140_e96145 * 0.1);
        let assign59140_e96148: f64 = (assign59140_e96142 + assign59140_e96147);
        let assign59140_e96149: f64 = (assign59140_e96148).sqrt();
        let assign59140_e96150: f64 = (assign59140_e96131 + assign59140_e96149);
        let assign59140_e96151: f64 = (0.5 * assign59140_e96150);
        (assign59140_e96151, (0.5 * ((var_phist_dn3 - var_vbsx_dn3) + ((((var_phist_dn3 - var_vbsx_dn3) * assign59140_e96141) + (assign59140_e96136 * (var_phist_dn3 - var_vbsx_dn3))) / (2.0 * assign59140_e96149)))), (0.5 * ((var_phist_dn4 - var_vbsx_dn4) + ((((var_phist_dn4 - var_vbsx_dn4) * assign59140_e96141) + (assign59140_e96136 * (var_phist_dn4 - var_vbsx_dn4))) / (2.0 * assign59140_e96149)))), (0.5 * ((var_phist_dn5 - var_vbsx_dn5) + ((((var_phist_dn5 - var_vbsx_dn5) * assign59140_e96141) + (assign59140_e96136 * (var_phist_dn5 - var_vbsx_dn5))) / (2.0 * assign59140_e96149)))), (0.5 * ((var_phist_dn6 - var_vbsx_dn6) + ((((var_phist_dn6 - var_vbsx_dn6) * assign59140_e96141) + (assign59140_e96136 * (var_phist_dn6 - var_vbsx_dn6))) / (2.0 * assign59140_e96149)))), (0.5 * ((var_phist_dn7 - var_vbsx_dn7) + ((((var_phist_dn7 - var_vbsx_dn7) * assign59140_e96141) + (assign59140_e96136 * (var_phist_dn7 - var_vbsx_dn7))) / (2.0 * assign59140_e96149)))), (0.5 * ((var_phist_dn8 - var_vbsx_dn8) + ((((var_phist_dn8 - var_vbsx_dn8) * assign59140_e96141) + (assign59140_e96136 * (var_phist_dn8 - var_vbsx_dn8))) / (2.0 * assign59140_e96149)))), (0.5 * ((var_phist_dn9 - var_vbsx_dn9) + ((((var_phist_dn9 - var_vbsx_dn9) * assign59140_e96141) + (assign59140_e96136 * (var_phist_dn9 - var_vbsx_dn9))) / (2.0 * assign59140_e96149)))), (0.5 * ((var_phist_dn10 - var_vbsx_dn10) + ((((var_phist_dn10 - var_vbsx_dn10) * assign59140_e96141) + (assign59140_e96136 * (var_phist_dn10 - var_vbsx_dn10))) / (2.0 * assign59140_e96149)))), (0.5 * ((var_phist_dn11 - var_vbsx_dn11) + ((((var_phist_dn11 - var_vbsx_dn11) * assign59140_e96141) + (assign59140_e96136 * (var_phist_dn11 - var_vbsx_dn11))) / (2.0 * assign59140_e96149)))),)
    } else {
        (var_phistvbs, var_phistvbs_dn3, var_phistvbs_dn4, var_phistvbs_dn5, var_phistvbs_dn6, var_phistvbs_dn7, var_phistvbs_dn8, var_phistvbs_dn9, var_phistvbs_dn10, var_phistvbs_dn11,)
    }
};
        var_phistvbs = assign59140_e96153;
        var_phistvbs_dn3 = assign59140_e96153_d_n3;
        var_phistvbs_dn4 = assign59140_e96153_d_n4;
        var_phistvbs_dn5 = assign59140_e96153_d_n5;
        var_phistvbs_dn6 = assign59140_e96153_d_n6;
        var_phistvbs_dn7 = assign59140_e96153_d_n7;
        var_phistvbs_dn8 = assign59140_e96153_d_n8;
        var_phistvbs_dn9 = assign59140_e96153_d_n9;
        var_phistvbs_dn10 = assign59140_e96153_d_n10;
        var_phistvbs_dn11 = assign59140_e96153_d_n11;
        var_phistvbs_rv = 0.0;

        let (assign59150_e96161, assign59150_e96161_d_n3, assign59150_e96161_d_n4, assign59150_e96161_d_n5, assign59150_e96161_d_n6, assign59150_e96161_d_n7, assign59150_e96161_d_n8, assign59150_e96161_d_n9, assign59150_e96161_d_n10, assign59150_e96161_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59150_e96159: f64 = (var_phistvbs).sqrt();
        (assign59150_e96159, (var_phistvbs_dn3 / (2.0 * assign59150_e96159)), (var_phistvbs_dn4 / (2.0 * assign59150_e96159)), (var_phistvbs_dn5 / (2.0 * assign59150_e96159)), (var_phistvbs_dn6 / (2.0 * assign59150_e96159)), (var_phistvbs_dn7 / (2.0 * assign59150_e96159)), (var_phistvbs_dn8 / (2.0 * assign59150_e96159)), (var_phistvbs_dn9 / (2.0 * assign59150_e96159)), (var_phistvbs_dn10 / (2.0 * assign59150_e96159)), (var_phistvbs_dn11 / (2.0 * assign59150_e96159)),)
    } else {
        (var_sqrtphistvbs, var_sqrtphistvbs_dn3, var_sqrtphistvbs_dn4, var_sqrtphistvbs_dn5, var_sqrtphistvbs_dn6, var_sqrtphistvbs_dn7, var_sqrtphistvbs_dn8, var_sqrtphistvbs_dn9, var_sqrtphistvbs_dn10, var_sqrtphistvbs_dn11,)
    }
};
        var_sqrtphistvbs = assign59150_e96161;
        var_sqrtphistvbs_dn3 = assign59150_e96161_d_n3;
        var_sqrtphistvbs_dn4 = assign59150_e96161_d_n4;
        var_sqrtphistvbs_dn5 = assign59150_e96161_d_n5;
        var_sqrtphistvbs_dn6 = assign59150_e96161_d_n6;
        var_sqrtphistvbs_dn7 = assign59150_e96161_d_n7;
        var_sqrtphistvbs_dn8 = assign59150_e96161_d_n8;
        var_sqrtphistvbs_dn9 = assign59150_e96161_d_n9;
        var_sqrtphistvbs_dn10 = assign59150_e96161_d_n10;
        var_sqrtphistvbs_dn11 = assign59150_e96161_d_n11;
        var_sqrtphistvbs_rv = 0.0;

        let (assign59160_e96170, assign59160_e96170_d_n3, assign59160_e96170_d_n4, assign59160_e96170_d_n5, assign59160_e96170_d_n6, assign59160_e96170_d_n7, assign59160_e96170_d_n8, assign59160_e96170_d_n9, assign59160_e96170_d_n10, assign59160_e96170_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59160_e96168: f64 = (var_t1dep * var_sqrtphistvbs);
        (assign59160_e96168, ((var_t1dep_dn3 * var_sqrtphistvbs) + (var_t1dep * var_sqrtphistvbs_dn3)), ((var_t1dep_dn4 * var_sqrtphistvbs) + (var_t1dep * var_sqrtphistvbs_dn4)), ((var_t1dep_dn5 * var_sqrtphistvbs) + (var_t1dep * var_sqrtphistvbs_dn5)), ((var_t1dep_dn6 * var_sqrtphistvbs) + (var_t1dep * var_sqrtphistvbs_dn6)), ((var_t1dep_dn7 * var_sqrtphistvbs) + (var_t1dep * var_sqrtphistvbs_dn7)), ((var_t1dep_dn8 * var_sqrtphistvbs) + (var_t1dep * var_sqrtphistvbs_dn8)), ((var_t1dep_dn9 * var_sqrtphistvbs) + (var_t1dep * var_sqrtphistvbs_dn9)), ((var_t1dep_dn10 * var_sqrtphistvbs) + (var_t1dep * var_sqrtphistvbs_dn10)), ((var_t1dep_dn11 * var_sqrtphistvbs) + (var_t1dep * var_sqrtphistvbs_dn11)),)
    } else {
        (var_xdep, var_xdep_dn3, var_xdep_dn4, var_xdep_dn5, var_xdep_dn6, var_xdep_dn7, var_xdep_dn8, var_xdep_dn9, var_xdep_dn10, var_xdep_dn11,)
    }
};
        var_xdep = assign59160_e96170;
        var_xdep_dn3 = assign59160_e96170_d_n3;
        var_xdep_dn4 = assign59160_e96170_d_n4;
        var_xdep_dn5 = assign59160_e96170_d_n5;
        var_xdep_dn6 = assign59160_e96170_d_n6;
        var_xdep_dn7 = assign59160_e96170_d_n7;
        var_xdep_dn8 = assign59160_e96170_d_n8;
        var_xdep_dn9 = assign59160_e96170_d_n9;
        var_xdep_dn10 = assign59160_e96170_d_n10;
        var_xdep_dn11 = assign59160_e96170_d_n11;
        var_xdep_rv = 0.0;

        let (assign59170_e96179, assign59170_e96179_d_n3, assign59170_e96179_d_n4, assign59170_e96179_d_n5, assign59170_e96179_d_n6, assign59170_e96179_d_n7, assign59170_e96179_d_n8, assign59170_e96179_d_n9, assign59170_e96179_d_n10, assign59170_e96179_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59170_e96177: f64 = (var_epssi / var_xdep);
        (assign59170_e96177, (-((var_epssi * var_xdep_dn3) / (var_xdep * var_xdep))), (-((var_epssi * var_xdep_dn4) / (var_xdep * var_xdep))), (-((var_epssi * var_xdep_dn5) / (var_xdep * var_xdep))), (-((var_epssi * var_xdep_dn6) / (var_xdep * var_xdep))), (-((var_epssi * var_xdep_dn7) / (var_xdep * var_xdep))), (-((var_epssi * var_xdep_dn8) / (var_xdep * var_xdep))), (-((var_epssi * var_xdep_dn9) / (var_xdep * var_xdep))), (-((var_epssi * var_xdep_dn10) / (var_xdep * var_xdep))), (-((var_epssi * var_xdep_dn11) / (var_xdep * var_xdep))),)
    } else {
        (var_cdep, var_cdep_dn3, var_cdep_dn4, var_cdep_dn5, var_cdep_dn6, var_cdep_dn7, var_cdep_dn8, var_cdep_dn9, var_cdep_dn10, var_cdep_dn11,)
    }
};
        var_cdep = assign59170_e96179;
        var_cdep_dn3 = assign59170_e96179_d_n3;
        var_cdep_dn4 = assign59170_e96179_d_n4;
        var_cdep_dn5 = assign59170_e96179_d_n5;
        var_cdep_dn6 = assign59170_e96179_d_n6;
        var_cdep_dn7 = assign59170_e96179_d_n7;
        var_cdep_dn8 = assign59170_e96179_d_n8;
        var_cdep_dn9 = assign59170_e96179_d_n9;
        var_cdep_dn10 = assign59170_e96179_d_n10;
        var_cdep_dn11 = assign59170_e96179_d_n11;
        var_cdep_rv = 0.0;

        let (assign59180_e96196, assign59180_e96196_d_n3, assign59180_e96196_d_n4, assign59180_e96196_d_n5, assign59180_e96196_d_n6, assign59180_e96196_d_n7, assign59180_e96196_d_n8, assign59180_e96196_d_n9, assign59180_e96196_d_n10, assign59180_e96196_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59180_e96186: f64 = (var_citedge_i + var_nfactoredge_t);
        let assign59180_e96189: f64 = (var_cdscdedge_a * var_vdsx);
        let assign59180_e96190: f64 = (assign59180_e96186 + assign59180_e96189);
        let assign59180_e96193: f64 = (var_cdscbedge_i * var_vbsx);
        let assign59180_e96194: f64 = (assign59180_e96190 - assign59180_e96193);
        (assign59180_e96194, (((var_cdscdedge_a_dn3 * var_vdsx) + (var_cdscdedge_a * var_vdsx_dn3)) - (var_cdscbedge_i * var_vbsx_dn3)), ((var_nfactoredge_t_dn4 + ((var_cdscdedge_a_dn4 * var_vdsx) + (var_cdscdedge_a * var_vdsx_dn4))) - (var_cdscbedge_i * var_vbsx_dn4)), ((var_nfactoredge_t_dn5 + ((var_cdscdedge_a_dn5 * var_vdsx) + (var_cdscdedge_a * var_vdsx_dn5))) - (var_cdscbedge_i * var_vbsx_dn5)), (((var_cdscdedge_a_dn6 * var_vdsx) + (var_cdscdedge_a * var_vdsx_dn6)) - (var_cdscbedge_i * var_vbsx_dn6)), (((var_cdscdedge_a_dn7 * var_vdsx) + (var_cdscdedge_a * var_vdsx_dn7)) - (var_cdscbedge_i * var_vbsx_dn7)), (((var_cdscdedge_a_dn8 * var_vdsx) + (var_cdscdedge_a * var_vdsx_dn8)) - (var_cdscbedge_i * var_vbsx_dn8)), (((var_cdscdedge_a_dn9 * var_vdsx) + (var_cdscdedge_a * var_vdsx_dn9)) - (var_cdscbedge_i * var_vbsx_dn9)), (((var_cdscdedge_a_dn10 * var_vdsx) + (var_cdscdedge_a * var_vdsx_dn10)) - (var_cdscbedge_i * var_vbsx_dn10)), (((var_cdscdedge_a_dn11 * var_vdsx) + (var_cdscdedge_a * var_vdsx_dn11)) - (var_cdscbedge_i * var_vbsx_dn11)),)
    } else {
        (var_cdsc, var_cdsc_dn3, var_cdsc_dn4, var_cdsc_dn5, var_cdsc_dn6, var_cdsc_dn7, var_cdsc_dn8, var_cdsc_dn9, var_cdsc_dn10, var_cdsc_dn11,)
    }
};
        var_cdsc = assign59180_e96196;
        var_cdsc_dn3 = assign59180_e96196_d_n3;
        var_cdsc_dn4 = assign59180_e96196_d_n4;
        var_cdsc_dn5 = assign59180_e96196_d_n5;
        var_cdsc_dn6 = assign59180_e96196_d_n6;
        var_cdsc_dn7 = assign59180_e96196_d_n7;
        var_cdsc_dn8 = assign59180_e96196_d_n8;
        var_cdsc_dn9 = assign59180_e96196_d_n9;
        var_cdsc_dn10 = assign59180_e96196_d_n10;
        var_cdsc_dn11 = assign59180_e96196_d_n11;
        var_cdsc_rv = 0.0;

        let (assign59190_e96207, assign59190_e96207_d_n3, assign59190_e96207_d_n4, assign59190_e96207_d_n5, assign59190_e96207_d_n6, assign59190_e96207_d_n7, assign59190_e96207_d_n8, assign59190_e96207_d_n9, assign59190_e96207_d_n10, assign59190_e96207_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59190_e96204: f64 = (var_cdsc / var_cox);
        let assign59190_e96205: f64 = (1.0 + assign59190_e96204);
        (assign59190_e96205, (var_cdsc_dn3 / var_cox), (var_cdsc_dn4 / var_cox), (var_cdsc_dn5 / var_cox), (var_cdsc_dn6 / var_cox), (var_cdsc_dn7 / var_cox), (var_cdsc_dn8 / var_cox), (var_cdsc_dn9 / var_cox), (var_cdsc_dn10 / var_cox), (var_cdsc_dn11 / var_cox),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11,)
    }
};
        var_t1 = assign59190_e96207;
        var_t1_dn3 = assign59190_e96207_d_n3;
        var_t1_dn4 = assign59190_e96207_d_n4;
        var_t1_dn5 = assign59190_e96207_d_n5;
        var_t1_dn6 = assign59190_e96207_d_n6;
        var_t1_dn7 = assign59190_e96207_d_n7;
        var_t1_dn8 = assign59190_e96207_d_n8;
        var_t1_dn9 = assign59190_e96207_d_n9;
        var_t1_dn10 = assign59190_e96207_d_n10;
        var_t1_dn11 = assign59190_e96207_d_n11;
        var_t1_rv = 0.0;

        let (assign59200_e96233, assign59200_e96233_d_n3, assign59200_e96233_d_n4, assign59200_e96233_d_n5, assign59200_e96233_d_n6, assign59200_e96233_d_n7, assign59200_e96233_d_n8, assign59200_e96233_d_n9, assign59200_e96233_d_n10, assign59200_e96233_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59200_e96215: f64 = (var_t1 + 1.0);
        let assign59200_e96218: f64 = (var_t1 - 1.0);
        let assign59200_e96221: f64 = (var_t1 - 1.0);
        let assign59200_e96222: f64 = (assign59200_e96218 * assign59200_e96221);
        let assign59200_e96225: f64 = (0.25 * 0.05);
        let assign59200_e96227: f64 = (assign59200_e96225 * 0.05);
        let assign59200_e96228: f64 = (assign59200_e96222 + assign59200_e96227);
        let assign59200_e96229: f64 = (assign59200_e96228).sqrt();
        let assign59200_e96230: f64 = (assign59200_e96215 + assign59200_e96229);
        let assign59200_e96231: f64 = (0.5 * assign59200_e96230);
        (assign59200_e96231, (0.5 * (var_t1_dn3 + (((var_t1_dn3 * assign59200_e96221) + (assign59200_e96218 * var_t1_dn3)) / (2.0 * assign59200_e96229)))), (0.5 * (var_t1_dn4 + (((var_t1_dn4 * assign59200_e96221) + (assign59200_e96218 * var_t1_dn4)) / (2.0 * assign59200_e96229)))), (0.5 * (var_t1_dn5 + (((var_t1_dn5 * assign59200_e96221) + (assign59200_e96218 * var_t1_dn5)) / (2.0 * assign59200_e96229)))), (0.5 * (var_t1_dn6 + (((var_t1_dn6 * assign59200_e96221) + (assign59200_e96218 * var_t1_dn6)) / (2.0 * assign59200_e96229)))), (0.5 * (var_t1_dn7 + (((var_t1_dn7 * assign59200_e96221) + (assign59200_e96218 * var_t1_dn7)) / (2.0 * assign59200_e96229)))), (0.5 * (var_t1_dn8 + (((var_t1_dn8 * assign59200_e96221) + (assign59200_e96218 * var_t1_dn8)) / (2.0 * assign59200_e96229)))), (0.5 * (var_t1_dn9 + (((var_t1_dn9 * assign59200_e96221) + (assign59200_e96218 * var_t1_dn9)) / (2.0 * assign59200_e96229)))), (0.5 * (var_t1_dn10 + (((var_t1_dn10 * assign59200_e96221) + (assign59200_e96218 * var_t1_dn10)) / (2.0 * assign59200_e96229)))), (0.5 * (var_t1_dn11 + (((var_t1_dn11 * assign59200_e96221) + (assign59200_e96218 * var_t1_dn11)) / (2.0 * assign59200_e96229)))),)
    } else {
        (var_n, var_n_dn3, var_n_dn4, var_n_dn5, var_n_dn6, var_n_dn7, var_n_dn8, var_n_dn9, var_n_dn10, var_n_dn11,)
    }
};
        var_n = assign59200_e96233;
        var_n_dn3 = assign59200_e96233_d_n3;
        var_n_dn4 = assign59200_e96233_d_n4;
        var_n_dn5 = assign59200_e96233_d_n5;
        var_n_dn6 = assign59200_e96233_d_n6;
        var_n_dn7 = assign59200_e96233_d_n7;
        var_n_dn8 = assign59200_e96233_d_n8;
        var_n_dn9 = assign59200_e96233_d_n9;
        var_n_dn10 = assign59200_e96233_d_n10;
        var_n_dn11 = assign59200_e96233_d_n11;
        var_n_rv = 0.0;

        let (assign59210_e96242, assign59210_e96242_d_n3, assign59210_e96242_d_n4, assign59210_e96242_d_n5, assign59210_e96242_d_n6, assign59210_e96242_d_n7, assign59210_e96242_d_n8, assign59210_e96242_d_n9, assign59210_e96242_d_n10, assign59210_e96242_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59210_e96240: f64 = (var_n * var_vt);
        (assign59210_e96240, (var_n_dn3 * var_vt), ((var_n_dn4 * var_vt) + (var_n * var_vt_dn4)), ((var_n_dn5 * var_vt) + (var_n * var_vt_dn5)), (var_n_dn6 * var_vt), (var_n_dn7 * var_vt), (var_n_dn8 * var_vt), (var_n_dn9 * var_vt), (var_n_dn10 * var_vt), (var_n_dn11 * var_vt),)
    } else {
        (var_nvt, var_nvt_dn3, var_nvt_dn4, var_nvt_dn5, var_nvt_dn6, var_nvt_dn7, var_nvt_dn8, var_nvt_dn9, var_nvt_dn10, var_nvt_dn11,)
    }
};
        var_nvt = assign59210_e96242;
        var_nvt_dn3 = assign59210_e96242_d_n3;
        var_nvt_dn4 = assign59210_e96242_d_n4;
        var_nvt_dn5 = assign59210_e96242_d_n5;
        var_nvt_dn6 = assign59210_e96242_d_n6;
        var_nvt_dn7 = assign59210_e96242_d_n7;
        var_nvt_dn8 = assign59210_e96242_d_n8;
        var_nvt_dn9 = assign59210_e96242_d_n9;
        var_nvt_dn10 = assign59210_e96242_d_n10;
        var_nvt_dn11 = assign59210_e96242_d_n11;
        var_nvt_rv = 0.0;

        let (assign59220_e96251, assign59220_e96251_d_n3, assign59220_e96251_d_n4, assign59220_e96251_d_n5, assign59220_e96251_d_n6, assign59220_e96251_d_n7, assign59220_e96251_d_n8, assign59220_e96251_d_n9, assign59220_e96251_d_n10, assign59220_e96251_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59220_e96249: f64 = (1.0 / var_nvt);
        (assign59220_e96249, (-(var_nvt_dn3 / (var_nvt * var_nvt))), (-(var_nvt_dn4 / (var_nvt * var_nvt))), (-(var_nvt_dn5 / (var_nvt * var_nvt))), (-(var_nvt_dn6 / (var_nvt * var_nvt))), (-(var_nvt_dn7 / (var_nvt * var_nvt))), (-(var_nvt_dn8 / (var_nvt * var_nvt))), (-(var_nvt_dn9 / (var_nvt * var_nvt))), (-(var_nvt_dn10 / (var_nvt * var_nvt))), (-(var_nvt_dn11 / (var_nvt * var_nvt))),)
    } else {
        (var_inv_nvt, var_inv_nvt_dn3, var_inv_nvt_dn4, var_inv_nvt_dn5, var_inv_nvt_dn6, var_inv_nvt_dn7, var_inv_nvt_dn8, var_inv_nvt_dn9, var_inv_nvt_dn10, var_inv_nvt_dn11,)
    }
};
        var_inv_nvt = assign59220_e96251;
        var_inv_nvt_dn3 = assign59220_e96251_d_n3;
        var_inv_nvt_dn4 = assign59220_e96251_d_n4;
        var_inv_nvt_dn5 = assign59220_e96251_d_n5;
        var_inv_nvt_dn6 = assign59220_e96251_d_n6;
        var_inv_nvt_dn7 = assign59220_e96251_d_n7;
        var_inv_nvt_dn8 = assign59220_e96251_d_n8;
        var_inv_nvt_dn9 = assign59220_e96251_d_n9;
        var_inv_nvt_dn10 = assign59220_e96251_d_n10;
        var_inv_nvt_dn11 = assign59220_e96251_d_n11;
        var_inv_nvt_rv = 0.0;

        let (assign59230_e96260, assign59230_e96260_d_n3, assign59230_e96260_d_n4, assign59230_e96260_d_n5, assign59230_e96260_d_n6, assign59230_e96260_d_n7, assign59230_e96260_d_n8, assign59230_e96260_d_n9, assign59230_e96260_d_n10, assign59230_e96260_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59230_e96258: f64 = (var_vg * var_inv_nvt);
        (assign59230_e96258, (var_vg * var_inv_nvt_dn3), (var_vg * var_inv_nvt_dn4), (var_vg * var_inv_nvt_dn5), (var_vg * var_inv_nvt_dn6), (var_vg * var_inv_nvt_dn7), ((var_vg_dn8 * var_inv_nvt) + (var_vg * var_inv_nvt_dn8)), (var_vg * var_inv_nvt_dn9), ((var_vg_dn10 * var_inv_nvt) + (var_vg * var_inv_nvt_dn10)), (var_vg * var_inv_nvt_dn11),)
    } else {
        (var_vg_1, var_vg_1_dn3, var_vg_1_dn4, var_vg_1_dn5, var_vg_1_dn6, var_vg_1_dn7, var_vg_1_dn8, var_vg_1_dn9, var_vg_1_dn10, var_vg_1_dn11,)
    }
};
        var_vg_1 = assign59230_e96260;
        var_vg_1_dn3 = assign59230_e96260_d_n3;
        var_vg_1_dn4 = assign59230_e96260_d_n4;
        var_vg_1_dn5 = assign59230_e96260_d_n5;
        var_vg_1_dn6 = assign59230_e96260_d_n6;
        var_vg_1_dn7 = assign59230_e96260_d_n7;
        var_vg_1_dn8 = assign59230_e96260_d_n8;
        var_vg_1_dn9 = assign59230_e96260_d_n9;
        var_vg_1_dn10 = assign59230_e96260_d_n10;
        var_vg_1_dn11 = assign59230_e96260_d_n11;
        var_vg_1_rv = 0.0;

        let (assign59240_e96269, assign59240_e96269_d_n3, assign59240_e96269_d_n4, assign59240_e96269_d_n5, assign59240_e96269_d_n6, assign59240_e96269_d_n7, assign59240_e96269_d_n8, assign59240_e96269_d_n9, assign59240_e96269_d_n10, assign59240_e96269_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59240_e96267: f64 = (var_vs * var_inv_nvt);
        (assign59240_e96267, (var_vs * var_inv_nvt_dn3), (var_vs * var_inv_nvt_dn4), (var_vs * var_inv_nvt_dn5), ((var_vs_dn6 * var_inv_nvt) + (var_vs * var_inv_nvt_dn6)), ((var_vs_dn7 * var_inv_nvt) + (var_vs * var_inv_nvt_dn7)), (var_vs * var_inv_nvt_dn8), (var_vs * var_inv_nvt_dn9), ((var_vs_dn10 * var_inv_nvt) + (var_vs * var_inv_nvt_dn10)), (var_vs * var_inv_nvt_dn11),)
    } else {
        (var_vs_1, var_vs_1_dn3, var_vs_1_dn4, var_vs_1_dn5, var_vs_1_dn6, var_vs_1_dn7, var_vs_1_dn8, var_vs_1_dn9, var_vs_1_dn10, var_vs_1_dn11,)
    }
};
        var_vs_1 = assign59240_e96269;
        var_vs_1_dn3 = assign59240_e96269_d_n3;
        var_vs_1_dn4 = assign59240_e96269_d_n4;
        var_vs_1_dn5 = assign59240_e96269_d_n5;
        var_vs_1_dn6 = assign59240_e96269_d_n6;
        var_vs_1_dn7 = assign59240_e96269_d_n7;
        var_vs_1_dn8 = assign59240_e96269_d_n8;
        var_vs_1_dn9 = assign59240_e96269_d_n9;
        var_vs_1_dn10 = assign59240_e96269_d_n10;
        var_vs_1_dn11 = assign59240_e96269_d_n11;
        var_vs_1_rv = 0.0;

        let (assign59250_e96278, assign59250_e96278_d_n3, assign59250_e96278_d_n4, assign59250_e96278_d_n5, assign59250_e96278_d_n6, assign59250_e96278_d_n7, assign59250_e96278_d_n8, assign59250_e96278_d_n9, assign59250_e96278_d_n10, assign59250_e96278_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59250_e96276: f64 = (var_vfb_i * var_inv_nvt);
        (assign59250_e96276, ((var_vfb_i_dn3 * var_inv_nvt) + (var_vfb_i * var_inv_nvt_dn3)), ((var_vfb_i_dn4 * var_inv_nvt) + (var_vfb_i * var_inv_nvt_dn4)), ((var_vfb_i_dn5 * var_inv_nvt) + (var_vfb_i * var_inv_nvt_dn5)), ((var_vfb_i_dn6 * var_inv_nvt) + (var_vfb_i * var_inv_nvt_dn6)), ((var_vfb_i_dn7 * var_inv_nvt) + (var_vfb_i * var_inv_nvt_dn7)), ((var_vfb_i_dn8 * var_inv_nvt) + (var_vfb_i * var_inv_nvt_dn8)), ((var_vfb_i_dn9 * var_inv_nvt) + (var_vfb_i * var_inv_nvt_dn9)), ((var_vfb_i_dn10 * var_inv_nvt) + (var_vfb_i * var_inv_nvt_dn10)), ((var_vfb_i_dn11 * var_inv_nvt) + (var_vfb_i * var_inv_nvt_dn11)),)
    } else {
        (var_vfb, var_vfb_dn3, var_vfb_dn4, var_vfb_dn5, var_vfb_dn6, var_vfb_dn7, var_vfb_dn8, var_vfb_dn9, var_vfb_dn10, var_vfb_dn11,)
    }
};
        var_vfb = assign59250_e96278;
        var_vfb_dn3 = assign59250_e96278_d_n3;
        var_vfb_dn4 = assign59250_e96278_d_n4;
        var_vfb_dn5 = assign59250_e96278_d_n5;
        var_vfb_dn6 = assign59250_e96278_d_n6;
        var_vfb_dn7 = assign59250_e96278_d_n7;
        var_vfb_dn8 = assign59250_e96278_d_n8;
        var_vfb_dn9 = assign59250_e96278_d_n9;
        var_vfb_dn10 = assign59250_e96278_d_n10;
        var_vfb_dn11 = assign59250_e96278_d_n11;
        var_vfb_rv = 0.0;

        let (assign59260_e96292, assign59260_e96292_d_n3, assign59260_e96292_d_n4, assign59260_e96292_d_n5, assign59260_e96292_d_n6, assign59260_e96292_d_n7, assign59260_e96292_d_n8, assign59260_e96292_d_n9, assign59260_e96292_d_n10, assign59260_e96292_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59260_e96286: f64 = (var_etabedge_i * var_vbsx);
        let assign59260_e96287: f64 = (var_eta0edge_t + assign59260_e96286);
        let assign59260_e96288: f64 = (-assign59260_e96287);
        let assign59260_e96290: f64 = (assign59260_e96288 * var_vdsx);
        (assign59260_e96290, (((-(var_eta0edge_t_dn3 + (var_etabedge_i * var_vbsx_dn3))) * var_vdsx) + (assign59260_e96288 * var_vdsx_dn3)), (((-(var_eta0edge_t_dn4 + (var_etabedge_i * var_vbsx_dn4))) * var_vdsx) + (assign59260_e96288 * var_vdsx_dn4)), (((-(var_eta0edge_t_dn5 + (var_etabedge_i * var_vbsx_dn5))) * var_vdsx) + (assign59260_e96288 * var_vdsx_dn5)), (((-(var_eta0edge_t_dn6 + (var_etabedge_i * var_vbsx_dn6))) * var_vdsx) + (assign59260_e96288 * var_vdsx_dn6)), (((-(var_eta0edge_t_dn7 + (var_etabedge_i * var_vbsx_dn7))) * var_vdsx) + (assign59260_e96288 * var_vdsx_dn7)), (((-(var_eta0edge_t_dn8 + (var_etabedge_i * var_vbsx_dn8))) * var_vdsx) + (assign59260_e96288 * var_vdsx_dn8)), (((-(var_eta0edge_t_dn9 + (var_etabedge_i * var_vbsx_dn9))) * var_vdsx) + (assign59260_e96288 * var_vdsx_dn9)), (((-(var_eta0edge_t_dn10 + (var_etabedge_i * var_vbsx_dn10))) * var_vdsx) + (assign59260_e96288 * var_vdsx_dn10)), (((-(var_eta0edge_t_dn11 + (var_etabedge_i * var_vbsx_dn11))) * var_vdsx) + (assign59260_e96288 * var_vdsx_dn11)),)
    } else {
        (var_dvth_dibl_1, var_dvth_dibl_1_dn3, var_dvth_dibl_1_dn4, var_dvth_dibl_1_dn5, var_dvth_dibl_1_dn6, var_dvth_dibl_1_dn7, var_dvth_dibl_1_dn8, var_dvth_dibl_1_dn9, var_dvth_dibl_1_dn10, var_dvth_dibl_1_dn11,)
    }
};
        var_dvth_dibl_1 = assign59260_e96292;
        var_dvth_dibl_1_dn3 = assign59260_e96292_d_n3;
        var_dvth_dibl_1_dn4 = assign59260_e96292_d_n4;
        var_dvth_dibl_1_dn5 = assign59260_e96292_d_n5;
        var_dvth_dibl_1_dn6 = assign59260_e96292_d_n6;
        var_dvth_dibl_1_dn7 = assign59260_e96292_d_n7;
        var_dvth_dibl_1_dn8 = assign59260_e96292_d_n8;
        var_dvth_dibl_1_dn9 = assign59260_e96292_d_n9;
        var_dvth_dibl_1_dn10 = assign59260_e96292_d_n10;
        var_dvth_dibl_1_dn11 = assign59260_e96292_d_n11;
        var_dvth_dibl_1_rv = 0.0;

        let (assign59270_e96313, assign59270_e96313_d_n3, assign59270_e96313_d_n4, assign59270_e96313_d_n5, assign59270_e96313_d_n6, assign59270_e96313_d_n7, assign59270_e96313_d_n8, assign59270_e96313_d_n9, assign59270_e96313_d_n10, assign59270_e96313_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59270_e96300: f64 = (var_kt1ledge_i / var_leff);
        let assign59270_e96301: f64 = (var_kt1edge_i + assign59270_e96300);
        let assign59270_e96304: f64 = (var_kt2edge_i * var_vbsx);
        let assign59270_e96305: f64 = (assign59270_e96301 + assign59270_e96304);
        let assign59270_e96308: f64 = (var_tratio).powf(var_kt1expedge_i);
        let assign59270_e96310: f64 = (assign59270_e96308 - 1.0);
        let assign59270_e96311: f64 = (assign59270_e96305 * assign59270_e96310);
        (assign59270_e96311, ((var_kt2edge_i * var_vbsx_dn3) * assign59270_e96310), (((var_kt2edge_i * var_vbsx_dn4) * assign59270_e96310) + (assign59270_e96305 * if 0.0 == 0.0 && ((var_kt1expedge_i) as f64).is_finite() && ((var_kt1expedge_i) as f64).fract() == 0.0 { if var_kt1expedge_i == 0.0 { 0.0 } else { (var_kt1expedge_i * ((var_tratio).powf(var_kt1expedge_i - 1.0) * var_tratio_dn4)) } } else { (assign59270_e96308 * (var_kt1expedge_i * (var_tratio_dn4 / var_tratio))) })), (((var_kt2edge_i * var_vbsx_dn5) * assign59270_e96310) + (assign59270_e96305 * if 0.0 == 0.0 && ((var_kt1expedge_i) as f64).is_finite() && ((var_kt1expedge_i) as f64).fract() == 0.0 { if var_kt1expedge_i == 0.0 { 0.0 } else { (var_kt1expedge_i * ((var_tratio).powf(var_kt1expedge_i - 1.0) * var_tratio_dn5)) } } else { (assign59270_e96308 * (var_kt1expedge_i * (var_tratio_dn5 / var_tratio))) })), ((var_kt2edge_i * var_vbsx_dn6) * assign59270_e96310), ((var_kt2edge_i * var_vbsx_dn7) * assign59270_e96310), ((var_kt2edge_i * var_vbsx_dn8) * assign59270_e96310), ((var_kt2edge_i * var_vbsx_dn9) * assign59270_e96310), ((var_kt2edge_i * var_vbsx_dn10) * assign59270_e96310), ((var_kt2edge_i * var_vbsx_dn11) * assign59270_e96310),)
    } else {
        (var_dvth_temp, var_dvth_temp_dn3, var_dvth_temp_dn4, var_dvth_temp_dn5, var_dvth_temp_dn6, var_dvth_temp_dn7, var_dvth_temp_dn8, var_dvth_temp_dn9, var_dvth_temp_dn10, var_dvth_temp_dn11,)
    }
};
        var_dvth_temp = assign59270_e96313;
        var_dvth_temp_dn3 = assign59270_e96313_d_n3;
        var_dvth_temp_dn4 = assign59270_e96313_d_n4;
        var_dvth_temp_dn5 = assign59270_e96313_d_n5;
        var_dvth_temp_dn6 = assign59270_e96313_d_n6;
        var_dvth_temp_dn7 = assign59270_e96313_d_n7;
        var_dvth_temp_dn8 = assign59270_e96313_d_n8;
        var_dvth_temp_dn9 = assign59270_e96313_d_n9;
        var_dvth_temp_dn10 = assign59270_e96313_d_n10;
        var_dvth_temp_dn11 = assign59270_e96313_d_n11;
        var_dvth_temp_rv = 0.0;

        let (assign59280_e96326, assign59280_e96326_d_n3, assign59280_e96326_d_n4, assign59280_e96326_d_n5, assign59280_e96326_d_n6, assign59280_e96326_d_n7, assign59280_e96326_d_n8, assign59280_e96326_d_n9, assign59280_e96326_d_n10, assign59280_e96326_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59280_e96322: f64 = (p.p1264 * var_vbsx);
        let assign59280_e96323: f64 = (1.0 + assign59280_e96322);
        let assign59280_e96324: f64 = (var_litl * assign59280_e96323);
        (assign59280_e96324, (var_litl * (p.p1264 * var_vbsx_dn3)), (var_litl * (p.p1264 * var_vbsx_dn4)), (var_litl * (p.p1264 * var_vbsx_dn5)), (var_litl * (p.p1264 * var_vbsx_dn6)), (var_litl * (p.p1264 * var_vbsx_dn7)), (var_litl * (p.p1264 * var_vbsx_dn8)), (var_litl * (p.p1264 * var_vbsx_dn9)), (var_litl * (p.p1264 * var_vbsx_dn10)), (var_litl * (p.p1264 * var_vbsx_dn11)),)
    } else {
        (var_litl_edge, var_litl_edge_dn3, var_litl_edge_dn4, var_litl_edge_dn5, var_litl_edge_dn6, var_litl_edge_dn7, var_litl_edge_dn8, var_litl_edge_dn9, var_litl_edge_dn10, var_litl_edge_dn11,)
    }
};
        var_litl_edge = assign59280_e96326;
        var_litl_edge_dn3 = assign59280_e96326_d_n3;
        var_litl_edge_dn4 = assign59280_e96326_d_n4;
        var_litl_edge_dn5 = assign59280_e96326_d_n5;
        var_litl_edge_dn6 = assign59280_e96326_d_n6;
        var_litl_edge_dn7 = assign59280_e96326_d_n7;
        var_litl_edge_dn8 = assign59280_e96326_d_n8;
        var_litl_edge_dn9 = assign59280_e96326_d_n9;
        var_litl_edge_dn10 = assign59280_e96326_d_n10;
        var_litl_edge_dn11 = assign59280_e96326_d_n11;
        var_litl_edge_rv = 0.0;

        let assign59290_e96329: f64 = if var_litl_edge > 0.0 { 1.0 } else { 0.0 };
        var_guard868 = assign59290_e96329;
        var_guard868_rv = 0.0;

        *var_cdep_slot = var_cdep;
        *var_cdep_dn10_slot = var_cdep_dn10;
        *var_cdep_dn11_slot = var_cdep_dn11;
        *var_cdep_dn3_slot = var_cdep_dn3;
        *var_cdep_dn4_slot = var_cdep_dn4;
        *var_cdep_dn5_slot = var_cdep_dn5;
        *var_cdep_dn6_slot = var_cdep_dn6;
        *var_cdep_dn7_slot = var_cdep_dn7;
        *var_cdep_dn8_slot = var_cdep_dn8;
        *var_cdep_dn9_slot = var_cdep_dn9;
        *var_cdep_rv_slot = var_cdep_rv;
        *var_cdsc_slot = var_cdsc;
        *var_cdsc_dn10_slot = var_cdsc_dn10;
        *var_cdsc_dn11_slot = var_cdsc_dn11;
        *var_cdsc_dn3_slot = var_cdsc_dn3;
        *var_cdsc_dn4_slot = var_cdsc_dn4;
        *var_cdsc_dn5_slot = var_cdsc_dn5;
        *var_cdsc_dn6_slot = var_cdsc_dn6;
        *var_cdsc_dn7_slot = var_cdsc_dn7;
        *var_cdsc_dn8_slot = var_cdsc_dn8;
        *var_cdsc_dn9_slot = var_cdsc_dn9;
        *var_cdsc_rv_slot = var_cdsc_rv;
        *var_dvth_dibl_1_slot = var_dvth_dibl_1;
        *var_dvth_dibl_1_dn10_slot = var_dvth_dibl_1_dn10;
        *var_dvth_dibl_1_dn11_slot = var_dvth_dibl_1_dn11;
        *var_dvth_dibl_1_dn3_slot = var_dvth_dibl_1_dn3;
        *var_dvth_dibl_1_dn4_slot = var_dvth_dibl_1_dn4;
        *var_dvth_dibl_1_dn5_slot = var_dvth_dibl_1_dn5;
        *var_dvth_dibl_1_dn6_slot = var_dvth_dibl_1_dn6;
        *var_dvth_dibl_1_dn7_slot = var_dvth_dibl_1_dn7;
        *var_dvth_dibl_1_dn8_slot = var_dvth_dibl_1_dn8;
        *var_dvth_dibl_1_dn9_slot = var_dvth_dibl_1_dn9;
        *var_dvth_dibl_1_rv_slot = var_dvth_dibl_1_rv;
        *var_dvth_temp_slot = var_dvth_temp;
        *var_dvth_temp_dn10_slot = var_dvth_temp_dn10;
        *var_dvth_temp_dn11_slot = var_dvth_temp_dn11;
        *var_dvth_temp_dn3_slot = var_dvth_temp_dn3;
        *var_dvth_temp_dn4_slot = var_dvth_temp_dn4;
        *var_dvth_temp_dn5_slot = var_dvth_temp_dn5;
        *var_dvth_temp_dn6_slot = var_dvth_temp_dn6;
        *var_dvth_temp_dn7_slot = var_dvth_temp_dn7;
        *var_dvth_temp_dn8_slot = var_dvth_temp_dn8;
        *var_dvth_temp_dn9_slot = var_dvth_temp_dn9;
        *var_dvth_temp_rv_slot = var_dvth_temp_rv;
        *var_eta0edge_t_slot = var_eta0edge_t;
        *var_eta0edge_t_dn10_slot = var_eta0edge_t_dn10;
        *var_eta0edge_t_dn11_slot = var_eta0edge_t_dn11;
        *var_eta0edge_t_dn3_slot = var_eta0edge_t_dn3;
        *var_eta0edge_t_dn4_slot = var_eta0edge_t_dn4;
        *var_eta0edge_t_dn5_slot = var_eta0edge_t_dn5;
        *var_eta0edge_t_dn6_slot = var_eta0edge_t_dn6;
        *var_eta0edge_t_dn7_slot = var_eta0edge_t_dn7;
        *var_eta0edge_t_dn8_slot = var_eta0edge_t_dn8;
        *var_eta0edge_t_dn9_slot = var_eta0edge_t_dn9;
        *var_eta0edge_t_rv_slot = var_eta0edge_t_rv;
        *var_guard867_slot = var_guard867;
        *var_guard867_rv_slot = var_guard867_rv;
        *var_guard868_slot = var_guard868;
        *var_guard868_rv_slot = var_guard868_rv;
        *var_inv_nvt_slot = var_inv_nvt;
        *var_inv_nvt_dn10_slot = var_inv_nvt_dn10;
        *var_inv_nvt_dn11_slot = var_inv_nvt_dn11;
        *var_inv_nvt_dn3_slot = var_inv_nvt_dn3;
        *var_inv_nvt_dn4_slot = var_inv_nvt_dn4;
        *var_inv_nvt_dn5_slot = var_inv_nvt_dn5;
        *var_inv_nvt_dn6_slot = var_inv_nvt_dn6;
        *var_inv_nvt_dn7_slot = var_inv_nvt_dn7;
        *var_inv_nvt_dn8_slot = var_inv_nvt_dn8;
        *var_inv_nvt_dn9_slot = var_inv_nvt_dn9;
        *var_inv_nvt_rv_slot = var_inv_nvt_rv;
        *var_litl_edge_slot = var_litl_edge;
        *var_litl_edge_dn10_slot = var_litl_edge_dn10;
        *var_litl_edge_dn11_slot = var_litl_edge_dn11;
        *var_litl_edge_dn3_slot = var_litl_edge_dn3;
        *var_litl_edge_dn4_slot = var_litl_edge_dn4;
        *var_litl_edge_dn5_slot = var_litl_edge_dn5;
        *var_litl_edge_dn6_slot = var_litl_edge_dn6;
        *var_litl_edge_dn7_slot = var_litl_edge_dn7;
        *var_litl_edge_dn8_slot = var_litl_edge_dn8;
        *var_litl_edge_dn9_slot = var_litl_edge_dn9;
        *var_litl_edge_rv_slot = var_litl_edge_rv;
        *var_n_slot = var_n;
        *var_n_dn10_slot = var_n_dn10;
        *var_n_dn11_slot = var_n_dn11;
        *var_n_dn3_slot = var_n_dn3;
        *var_n_dn4_slot = var_n_dn4;
        *var_n_dn5_slot = var_n_dn5;
        *var_n_dn6_slot = var_n_dn6;
        *var_n_dn7_slot = var_n_dn7;
        *var_n_dn8_slot = var_n_dn8;
        *var_n_dn9_slot = var_n_dn9;
        *var_n_rv_slot = var_n_rv;
        *var_nfactoredge_t_slot = var_nfactoredge_t;
        *var_nfactoredge_t_dn4_slot = var_nfactoredge_t_dn4;
        *var_nfactoredge_t_dn5_slot = var_nfactoredge_t_dn5;
        *var_nfactoredge_t_rv_slot = var_nfactoredge_t_rv;
        *var_nvt_slot = var_nvt;
        *var_nvt_dn10_slot = var_nvt_dn10;
        *var_nvt_dn11_slot = var_nvt_dn11;
        *var_nvt_dn3_slot = var_nvt_dn3;
        *var_nvt_dn4_slot = var_nvt_dn4;
        *var_nvt_dn5_slot = var_nvt_dn5;
        *var_nvt_dn6_slot = var_nvt_dn6;
        *var_nvt_dn7_slot = var_nvt_dn7;
        *var_nvt_dn8_slot = var_nvt_dn8;
        *var_nvt_dn9_slot = var_nvt_dn9;
        *var_nvt_rv_slot = var_nvt_rv;
        *var_phib_edge_slot = var_phib_edge;
        *var_phib_edge_dn10_slot = var_phib_edge_dn10;
        *var_phib_edge_dn11_slot = var_phib_edge_dn11;
        *var_phib_edge_dn3_slot = var_phib_edge_dn3;
        *var_phib_edge_dn4_slot = var_phib_edge_dn4;
        *var_phib_edge_dn5_slot = var_phib_edge_dn5;
        *var_phib_edge_dn6_slot = var_phib_edge_dn6;
        *var_phib_edge_dn7_slot = var_phib_edge_dn7;
        *var_phib_edge_dn8_slot = var_phib_edge_dn8;
        *var_phib_edge_dn9_slot = var_phib_edge_dn9;
        *var_phib_edge_rv_slot = var_phib_edge_rv;
        *var_phist_slot = var_phist;
        *var_phist_dn10_slot = var_phist_dn10;
        *var_phist_dn11_slot = var_phist_dn11;
        *var_phist_dn3_slot = var_phist_dn3;
        *var_phist_dn4_slot = var_phist_dn4;
        *var_phist_dn5_slot = var_phist_dn5;
        *var_phist_dn6_slot = var_phist_dn6;
        *var_phist_dn7_slot = var_phist_dn7;
        *var_phist_dn8_slot = var_phist_dn8;
        *var_phist_dn9_slot = var_phist_dn9;
        *var_phist_rv_slot = var_phist_rv;
        *var_phistvbs_slot = var_phistvbs;
        *var_phistvbs_dn10_slot = var_phistvbs_dn10;
        *var_phistvbs_dn11_slot = var_phistvbs_dn11;
        *var_phistvbs_dn3_slot = var_phistvbs_dn3;
        *var_phistvbs_dn4_slot = var_phistvbs_dn4;
        *var_phistvbs_dn5_slot = var_phistvbs_dn5;
        *var_phistvbs_dn6_slot = var_phistvbs_dn6;
        *var_phistvbs_dn7_slot = var_phistvbs_dn7;
        *var_phistvbs_dn8_slot = var_phistvbs_dn8;
        *var_phistvbs_dn9_slot = var_phistvbs_dn9;
        *var_phistvbs_rv_slot = var_phistvbs_rv;
        *var_qde_slot = var_qde;
        *var_qde_dn10_slot = var_qde_dn10;
        *var_qde_dn11_slot = var_qde_dn11;
        *var_qde_dn3_slot = var_qde_dn3;
        *var_qde_dn4_slot = var_qde_dn4;
        *var_qde_dn5_slot = var_qde_dn5;
        *var_qde_dn6_slot = var_qde_dn6;
        *var_qde_dn7_slot = var_qde_dn7;
        *var_qde_dn8_slot = var_qde_dn8;
        *var_qde_dn9_slot = var_qde_dn9;
        *var_qde_rv_slot = var_qde_rv;
        *var_qse_slot = var_qse;
        *var_qse_dn10_slot = var_qse_dn10;
        *var_qse_dn11_slot = var_qse_dn11;
        *var_qse_dn3_slot = var_qse_dn3;
        *var_qse_dn4_slot = var_qse_dn4;
        *var_qse_dn5_slot = var_qse_dn5;
        *var_qse_dn6_slot = var_qse_dn6;
        *var_qse_dn7_slot = var_qse_dn7;
        *var_qse_dn8_slot = var_qse_dn8;
        *var_qse_dn9_slot = var_qse_dn9;
        *var_qse_rv_slot = var_qse_rv;
        *var_sqrtphistvbs_slot = var_sqrtphistvbs;
        *var_sqrtphistvbs_dn10_slot = var_sqrtphistvbs_dn10;
        *var_sqrtphistvbs_dn11_slot = var_sqrtphistvbs_dn11;
        *var_sqrtphistvbs_dn3_slot = var_sqrtphistvbs_dn3;
        *var_sqrtphistvbs_dn4_slot = var_sqrtphistvbs_dn4;
        *var_sqrtphistvbs_dn5_slot = var_sqrtphistvbs_dn5;
        *var_sqrtphistvbs_dn6_slot = var_sqrtphistvbs_dn6;
        *var_sqrtphistvbs_dn7_slot = var_sqrtphistvbs_dn7;
        *var_sqrtphistvbs_dn8_slot = var_sqrtphistvbs_dn8;
        *var_sqrtphistvbs_dn9_slot = var_sqrtphistvbs_dn9;
        *var_sqrtphistvbs_rv_slot = var_sqrtphistvbs_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t1_rv_slot = var_t1_rv;
        *var_t1dep_slot = var_t1dep;
        *var_t1dep_dn10_slot = var_t1dep_dn10;
        *var_t1dep_dn11_slot = var_t1dep_dn11;
        *var_t1dep_dn3_slot = var_t1dep_dn3;
        *var_t1dep_dn4_slot = var_t1dep_dn4;
        *var_t1dep_dn5_slot = var_t1dep_dn5;
        *var_t1dep_dn6_slot = var_t1dep_dn6;
        *var_t1dep_dn7_slot = var_t1dep_dn7;
        *var_t1dep_dn8_slot = var_t1dep_dn8;
        *var_t1dep_dn9_slot = var_t1dep_dn9;
        *var_t1dep_rv_slot = var_t1dep_rv;
        *var_vfb_slot = var_vfb;
        *var_vfb_dn10_slot = var_vfb_dn10;
        *var_vfb_dn11_slot = var_vfb_dn11;
        *var_vfb_dn3_slot = var_vfb_dn3;
        *var_vfb_dn4_slot = var_vfb_dn4;
        *var_vfb_dn5_slot = var_vfb_dn5;
        *var_vfb_dn6_slot = var_vfb_dn6;
        *var_vfb_dn7_slot = var_vfb_dn7;
        *var_vfb_dn8_slot = var_vfb_dn8;
        *var_vfb_dn9_slot = var_vfb_dn9;
        *var_vfb_rv_slot = var_vfb_rv;
        *var_vg_1_slot = var_vg_1;
        *var_vg_1_dn10_slot = var_vg_1_dn10;
        *var_vg_1_dn11_slot = var_vg_1_dn11;
        *var_vg_1_dn3_slot = var_vg_1_dn3;
        *var_vg_1_dn4_slot = var_vg_1_dn4;
        *var_vg_1_dn5_slot = var_vg_1_dn5;
        *var_vg_1_dn6_slot = var_vg_1_dn6;
        *var_vg_1_dn7_slot = var_vg_1_dn7;
        *var_vg_1_dn8_slot = var_vg_1_dn8;
        *var_vg_1_dn9_slot = var_vg_1_dn9;
        *var_vg_1_rv_slot = var_vg_1_rv;
        *var_vs_1_slot = var_vs_1;
        *var_vs_1_dn10_slot = var_vs_1_dn10;
        *var_vs_1_dn11_slot = var_vs_1_dn11;
        *var_vs_1_dn3_slot = var_vs_1_dn3;
        *var_vs_1_dn4_slot = var_vs_1_dn4;
        *var_vs_1_dn5_slot = var_vs_1_dn5;
        *var_vs_1_dn6_slot = var_vs_1_dn6;
        *var_vs_1_dn7_slot = var_vs_1_dn7;
        *var_vs_1_dn8_slot = var_vs_1_dn8;
        *var_vs_1_dn9_slot = var_vs_1_dn9;
        *var_vs_1_rv_slot = var_vs_1_rv;
        *var_xdep_slot = var_xdep;
        *var_xdep_dn10_slot = var_xdep_dn10;
        *var_xdep_dn11_slot = var_xdep_dn11;
        *var_xdep_dn3_slot = var_xdep_dn3;
        *var_xdep_dn4_slot = var_xdep_dn4;
        *var_xdep_dn5_slot = var_xdep_dn5;
        *var_xdep_dn6_slot = var_xdep_dn6;
        *var_xdep_dn7_slot = var_xdep_dn7;
        *var_xdep_dn8_slot = var_xdep_dn8;
        *var_xdep_dn9_slot = var_xdep_dn9;
        *var_xdep_rv_slot = var_xdep_rv;
    }

    pub(super) fn stamp_reactive_block_207(
        p: &Parameters,
        var_cox: f64,
        var_dvth_dibl_1: f64,
        var_dvth_dibl_1_dn10: f64,
        var_dvth_dibl_1_dn11: f64,
        var_dvth_dibl_1_dn3: f64,
        var_dvth_dibl_1_dn4: f64,
        var_dvth_dibl_1_dn5: f64,
        var_dvth_dibl_1_dn6: f64,
        var_dvth_dibl_1_dn7: f64,
        var_dvth_dibl_1_dn8: f64,
        var_dvth_dibl_1_dn9: f64,
        var_dvth_temp: f64,
        var_dvth_temp_dn10: f64,
        var_dvth_temp_dn11: f64,
        var_dvth_temp_dn3: f64,
        var_dvth_temp_dn4: f64,
        var_dvth_temp_dn5: f64,
        var_dvth_temp_dn6: f64,
        var_dvth_temp_dn7: f64,
        var_dvth_temp_dn8: f64,
        var_dvth_temp_dn9: f64,
        var_epssi: f64,
        var_guard492: f64,
        var_guard867: f64,
        var_guard868: f64,
        var_inv_nvt: f64,
        var_inv_nvt_dn10: f64,
        var_inv_nvt_dn11: f64,
        var_inv_nvt_dn3: f64,
        var_inv_nvt_dn4: f64,
        var_inv_nvt_dn5: f64,
        var_inv_nvt_dn6: f64,
        var_inv_nvt_dn7: f64,
        var_inv_nvt_dn8: f64,
        var_inv_nvt_dn9: f64,
        var_k2edge_i: f64,
        var_k2edge_i_dn10: f64,
        var_k2edge_i_dn11: f64,
        var_k2edge_i_dn3: f64,
        var_k2edge_i_dn4: f64,
        var_k2edge_i_dn5: f64,
        var_k2edge_i_dn6: f64,
        var_k2edge_i_dn7: f64,
        var_k2edge_i_dn8: f64,
        var_k2edge_i_dn9: f64,
        var_leff: f64,
        var_litl_edge: f64,
        var_litl_edge_dn10: f64,
        var_litl_edge_dn11: f64,
        var_litl_edge_dn3: f64,
        var_litl_edge_dn4: f64,
        var_litl_edge_dn5: f64,
        var_litl_edge_dn6: f64,
        var_litl_edge_dn7: f64,
        var_litl_edge_dn8: f64,
        var_litl_edge_dn9: f64,
        var_n: f64,
        var_n_dn10: f64,
        var_n_dn11: f64,
        var_n_dn3: f64,
        var_n_dn4: f64,
        var_n_dn5: f64,
        var_n_dn6: f64,
        var_n_dn7: f64,
        var_n_dn8: f64,
        var_n_dn9: f64,
        var_ndepedge_i: f64,
        var_phib_edge: f64,
        var_phib_edge_dn10: f64,
        var_phib_edge_dn11: f64,
        var_phib_edge_dn3: f64,
        var_phib_edge_dn4: f64,
        var_phib_edge_dn5: f64,
        var_phib_edge_dn6: f64,
        var_phib_edge_dn7: f64,
        var_phib_edge_dn8: f64,
        var_phib_edge_dn9: f64,
        var_phist: f64,
        var_phist_dn10: f64,
        var_phist_dn11: f64,
        var_phist_dn3: f64,
        var_phist_dn4: f64,
        var_phist_dn5: f64,
        var_phist_dn6: f64,
        var_phist_dn7: f64,
        var_phist_dn8: f64,
        var_phist_dn9: f64,
        var_vbi_edge: f64,
        var_vbi_edge_dn10: f64,
        var_vbi_edge_dn11: f64,
        var_vbi_edge_dn3: f64,
        var_vbi_edge_dn4: f64,
        var_vbi_edge_dn5: f64,
        var_vbi_edge_dn6: f64,
        var_vbi_edge_dn7: f64,
        var_vbi_edge_dn8: f64,
        var_vbi_edge_dn9: f64,
        var_vbsx: f64,
        var_vbsx_dn10: f64,
        var_vbsx_dn11: f64,
        var_vbsx_dn3: f64,
        var_vbsx_dn4: f64,
        var_vbsx_dn5: f64,
        var_vbsx_dn6: f64,
        var_vbsx_dn7: f64,
        var_vbsx_dn8: f64,
        var_vbsx_dn9: f64,
        var_vfb: f64,
        var_vfb_dn10: f64,
        var_vfb_dn11: f64,
        var_vfb_dn3: f64,
        var_vfb_dn4: f64,
        var_vfb_dn5: f64,
        var_vfb_dn6: f64,
        var_vfb_dn7: f64,
        var_vfb_dn8: f64,
        var_vfb_dn9: f64,
        var_vg_1: f64,
        var_vg_1_dn10: f64,
        var_vg_1_dn11: f64,
        var_vg_1_dn3: f64,
        var_vg_1_dn4: f64,
        var_vg_1_dn5: f64,
        var_vg_1_dn6: f64,
        var_vg_1_dn7: f64,
        var_vg_1_dn8: f64,
        var_vg_1_dn9: f64,
        var_vth0_stress_edge: f64,
        var_vth0_stress_edge_dn10: f64,
        var_vth0_stress_edge_dn11: f64,
        var_vth0_stress_edge_dn3: f64,
        var_vth0_stress_edge_dn4: f64,
        var_vth0_stress_edge_dn5: f64,
        var_vth0_stress_edge_dn6: f64,
        var_vth0_stress_edge_dn7: f64,
        var_vth0_stress_edge_dn8: f64,
        var_vth0_stress_edge_dn9: f64,
        var_vth0_well_edge: f64,
        var_vth0_well_edge_dn10: f64,
        var_vth0_well_edge_dn11: f64,
        var_vth0_well_edge_dn3: f64,
        var_vth0_well_edge_dn4: f64,
        var_vth0_well_edge_dn5: f64,
        var_vth0_well_edge_dn6: f64,
        var_vth0_well_edge_dn7: f64,
        var_vth0_well_edge_dn8: f64,
        var_vth0_well_edge_dn9: f64,
        var_dgammaedge_i_slot: &mut f64,
        var_dgammaedge_i_rv_slot: &mut f64,
        var_dvth_sce_slot: &mut f64,
        var_dvth_sce_dn10_slot: &mut f64,
        var_dvth_sce_dn11_slot: &mut f64,
        var_dvth_sce_dn3_slot: &mut f64,
        var_dvth_sce_dn4_slot: &mut f64,
        var_dvth_sce_dn5_slot: &mut f64,
        var_dvth_sce_dn6_slot: &mut f64,
        var_dvth_sce_dn7_slot: &mut f64,
        var_dvth_sce_dn8_slot: &mut f64,
        var_dvth_sce_dn9_slot: &mut f64,
        var_dvth_sce_rv_slot: &mut f64,
        var_gam_edge_slot: &mut f64,
        var_gam_edge_dn10_slot: &mut f64,
        var_gam_edge_dn11_slot: &mut f64,
        var_gam_edge_dn3_slot: &mut f64,
        var_gam_edge_dn4_slot: &mut f64,
        var_gam_edge_dn5_slot: &mut f64,
        var_gam_edge_dn6_slot: &mut f64,
        var_gam_edge_dn7_slot: &mut f64,
        var_gam_edge_dn8_slot: &mut f64,
        var_gam_edge_dn9_slot: &mut f64,
        var_gam_edge_rv_slot: &mut f64,
        var_gammapd_slot: &mut f64,
        var_gammapd_dn10_slot: &mut f64,
        var_gammapd_dn11_slot: &mut f64,
        var_gammapd_dn3_slot: &mut f64,
        var_gammapd_dn4_slot: &mut f64,
        var_gammapd_dn5_slot: &mut f64,
        var_gammapd_dn6_slot: &mut f64,
        var_gammapd_dn7_slot: &mut f64,
        var_gammapd_dn8_slot: &mut f64,
        var_gammapd_dn9_slot: &mut f64,
        var_gammapd_rv_slot: &mut f64,
        var_guard869_slot: &mut f64,
        var_guard869_rv_slot: &mut f64,
        var_guard870_slot: &mut f64,
        var_guard870_rv_slot: &mut f64,
        var_phib_n_edge_slot: &mut f64,
        var_phib_n_edge_dn10_slot: &mut f64,
        var_phib_n_edge_dn11_slot: &mut f64,
        var_phib_n_edge_dn3_slot: &mut f64,
        var_phib_n_edge_dn4_slot: &mut f64,
        var_phib_n_edge_dn5_slot: &mut f64,
        var_phib_n_edge_dn6_slot: &mut f64,
        var_phib_n_edge_dn7_slot: &mut f64,
        var_phib_n_edge_dn8_slot: &mut f64,
        var_phib_n_edge_dn9_slot: &mut f64,
        var_phib_n_edge_rv_slot: &mut f64,
        var_psip_slot: &mut f64,
        var_psip_dn10_slot: &mut f64,
        var_psip_dn11_slot: &mut f64,
        var_psip_dn3_slot: &mut f64,
        var_psip_dn4_slot: &mut f64,
        var_psip_dn5_slot: &mut f64,
        var_psip_dn6_slot: &mut f64,
        var_psip_dn7_slot: &mut f64,
        var_psip_dn8_slot: &mut f64,
        var_psip_dn9_slot: &mut f64,
        var_psip_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_dn9_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_theta_sce_edge_slot: &mut f64,
        var_theta_sce_edge_dn10_slot: &mut f64,
        var_theta_sce_edge_dn11_slot: &mut f64,
        var_theta_sce_edge_dn3_slot: &mut f64,
        var_theta_sce_edge_dn4_slot: &mut f64,
        var_theta_sce_edge_dn5_slot: &mut f64,
        var_theta_sce_edge_dn6_slot: &mut f64,
        var_theta_sce_edge_dn7_slot: &mut f64,
        var_theta_sce_edge_dn8_slot: &mut f64,
        var_theta_sce_edge_dn9_slot: &mut f64,
        var_theta_sce_edge_rv_slot: &mut f64,
        var_vgfb_slot: &mut f64,
        var_vgfb_dn10_slot: &mut f64,
        var_vgfb_dn11_slot: &mut f64,
        var_vgfb_dn3_slot: &mut f64,
        var_vgfb_dn4_slot: &mut f64,
        var_vgfb_dn5_slot: &mut f64,
        var_vgfb_dn6_slot: &mut f64,
        var_vgfb_dn7_slot: &mut f64,
        var_vgfb_dn8_slot: &mut f64,
        var_vgfb_dn9_slot: &mut f64,
        var_vgfb_rv_slot: &mut f64,
        var_vgfbpd_slot: &mut f64,
        var_vgfbpd_dn10_slot: &mut f64,
        var_vgfbpd_dn11_slot: &mut f64,
        var_vgfbpd_dn3_slot: &mut f64,
        var_vgfbpd_dn4_slot: &mut f64,
        var_vgfbpd_dn5_slot: &mut f64,
        var_vgfbpd_dn6_slot: &mut f64,
        var_vgfbpd_dn7_slot: &mut f64,
        var_vgfbpd_dn8_slot: &mut f64,
        var_vgfbpd_dn9_slot: &mut f64,
        var_vgfbpd_rv_slot: &mut f64,
        var_vth_shift_slot: &mut f64,
        var_vth_shift_dn10_slot: &mut f64,
        var_vth_shift_dn11_slot: &mut f64,
        var_vth_shift_dn3_slot: &mut f64,
        var_vth_shift_dn4_slot: &mut f64,
        var_vth_shift_dn5_slot: &mut f64,
        var_vth_shift_dn6_slot: &mut f64,
        var_vth_shift_dn7_slot: &mut f64,
        var_vth_shift_dn8_slot: &mut f64,
        var_vth_shift_dn9_slot: &mut f64,
        var_vth_shift_rv_slot: &mut f64,
    ) {
        let mut var_dgammaedge_i: f64 = *var_dgammaedge_i_slot;
        let mut var_dgammaedge_i_rv: f64 = *var_dgammaedge_i_rv_slot;
        let mut var_dvth_sce: f64 = *var_dvth_sce_slot;
        let mut var_dvth_sce_dn10: f64 = *var_dvth_sce_dn10_slot;
        let mut var_dvth_sce_dn11: f64 = *var_dvth_sce_dn11_slot;
        let mut var_dvth_sce_dn3: f64 = *var_dvth_sce_dn3_slot;
        let mut var_dvth_sce_dn4: f64 = *var_dvth_sce_dn4_slot;
        let mut var_dvth_sce_dn5: f64 = *var_dvth_sce_dn5_slot;
        let mut var_dvth_sce_dn6: f64 = *var_dvth_sce_dn6_slot;
        let mut var_dvth_sce_dn7: f64 = *var_dvth_sce_dn7_slot;
        let mut var_dvth_sce_dn8: f64 = *var_dvth_sce_dn8_slot;
        let mut var_dvth_sce_dn9: f64 = *var_dvth_sce_dn9_slot;
        let mut var_dvth_sce_rv: f64 = *var_dvth_sce_rv_slot;
        let mut var_gam_edge: f64 = *var_gam_edge_slot;
        let mut var_gam_edge_dn10: f64 = *var_gam_edge_dn10_slot;
        let mut var_gam_edge_dn11: f64 = *var_gam_edge_dn11_slot;
        let mut var_gam_edge_dn3: f64 = *var_gam_edge_dn3_slot;
        let mut var_gam_edge_dn4: f64 = *var_gam_edge_dn4_slot;
        let mut var_gam_edge_dn5: f64 = *var_gam_edge_dn5_slot;
        let mut var_gam_edge_dn6: f64 = *var_gam_edge_dn6_slot;
        let mut var_gam_edge_dn7: f64 = *var_gam_edge_dn7_slot;
        let mut var_gam_edge_dn8: f64 = *var_gam_edge_dn8_slot;
        let mut var_gam_edge_dn9: f64 = *var_gam_edge_dn9_slot;
        let mut var_gam_edge_rv: f64 = *var_gam_edge_rv_slot;
        let mut var_gammapd: f64 = *var_gammapd_slot;
        let mut var_gammapd_dn10: f64 = *var_gammapd_dn10_slot;
        let mut var_gammapd_dn11: f64 = *var_gammapd_dn11_slot;
        let mut var_gammapd_dn3: f64 = *var_gammapd_dn3_slot;
        let mut var_gammapd_dn4: f64 = *var_gammapd_dn4_slot;
        let mut var_gammapd_dn5: f64 = *var_gammapd_dn5_slot;
        let mut var_gammapd_dn6: f64 = *var_gammapd_dn6_slot;
        let mut var_gammapd_dn7: f64 = *var_gammapd_dn7_slot;
        let mut var_gammapd_dn8: f64 = *var_gammapd_dn8_slot;
        let mut var_gammapd_dn9: f64 = *var_gammapd_dn9_slot;
        let mut var_gammapd_rv: f64 = *var_gammapd_rv_slot;
        let mut var_guard869: f64 = *var_guard869_slot;
        let mut var_guard869_rv: f64 = *var_guard869_rv_slot;
        let mut var_guard870: f64 = *var_guard870_slot;
        let mut var_guard870_rv: f64 = *var_guard870_rv_slot;
        let mut var_phib_n_edge: f64 = *var_phib_n_edge_slot;
        let mut var_phib_n_edge_dn10: f64 = *var_phib_n_edge_dn10_slot;
        let mut var_phib_n_edge_dn11: f64 = *var_phib_n_edge_dn11_slot;
        let mut var_phib_n_edge_dn3: f64 = *var_phib_n_edge_dn3_slot;
        let mut var_phib_n_edge_dn4: f64 = *var_phib_n_edge_dn4_slot;
        let mut var_phib_n_edge_dn5: f64 = *var_phib_n_edge_dn5_slot;
        let mut var_phib_n_edge_dn6: f64 = *var_phib_n_edge_dn6_slot;
        let mut var_phib_n_edge_dn7: f64 = *var_phib_n_edge_dn7_slot;
        let mut var_phib_n_edge_dn8: f64 = *var_phib_n_edge_dn8_slot;
        let mut var_phib_n_edge_dn9: f64 = *var_phib_n_edge_dn9_slot;
        let mut var_phib_n_edge_rv: f64 = *var_phib_n_edge_rv_slot;
        let mut var_psip: f64 = *var_psip_slot;
        let mut var_psip_dn10: f64 = *var_psip_dn10_slot;
        let mut var_psip_dn11: f64 = *var_psip_dn11_slot;
        let mut var_psip_dn3: f64 = *var_psip_dn3_slot;
        let mut var_psip_dn4: f64 = *var_psip_dn4_slot;
        let mut var_psip_dn5: f64 = *var_psip_dn5_slot;
        let mut var_psip_dn6: f64 = *var_psip_dn6_slot;
        let mut var_psip_dn7: f64 = *var_psip_dn7_slot;
        let mut var_psip_dn8: f64 = *var_psip_dn8_slot;
        let mut var_psip_dn9: f64 = *var_psip_dn9_slot;
        let mut var_psip_rv: f64 = *var_psip_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_dn9: f64 = *var_t3_dn9_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_theta_sce_edge: f64 = *var_theta_sce_edge_slot;
        let mut var_theta_sce_edge_dn10: f64 = *var_theta_sce_edge_dn10_slot;
        let mut var_theta_sce_edge_dn11: f64 = *var_theta_sce_edge_dn11_slot;
        let mut var_theta_sce_edge_dn3: f64 = *var_theta_sce_edge_dn3_slot;
        let mut var_theta_sce_edge_dn4: f64 = *var_theta_sce_edge_dn4_slot;
        let mut var_theta_sce_edge_dn5: f64 = *var_theta_sce_edge_dn5_slot;
        let mut var_theta_sce_edge_dn6: f64 = *var_theta_sce_edge_dn6_slot;
        let mut var_theta_sce_edge_dn7: f64 = *var_theta_sce_edge_dn7_slot;
        let mut var_theta_sce_edge_dn8: f64 = *var_theta_sce_edge_dn8_slot;
        let mut var_theta_sce_edge_dn9: f64 = *var_theta_sce_edge_dn9_slot;
        let mut var_theta_sce_edge_rv: f64 = *var_theta_sce_edge_rv_slot;
        let mut var_vgfb: f64 = *var_vgfb_slot;
        let mut var_vgfb_dn10: f64 = *var_vgfb_dn10_slot;
        let mut var_vgfb_dn11: f64 = *var_vgfb_dn11_slot;
        let mut var_vgfb_dn3: f64 = *var_vgfb_dn3_slot;
        let mut var_vgfb_dn4: f64 = *var_vgfb_dn4_slot;
        let mut var_vgfb_dn5: f64 = *var_vgfb_dn5_slot;
        let mut var_vgfb_dn6: f64 = *var_vgfb_dn6_slot;
        let mut var_vgfb_dn7: f64 = *var_vgfb_dn7_slot;
        let mut var_vgfb_dn8: f64 = *var_vgfb_dn8_slot;
        let mut var_vgfb_dn9: f64 = *var_vgfb_dn9_slot;
        let mut var_vgfb_rv: f64 = *var_vgfb_rv_slot;
        let mut var_vgfbpd: f64 = *var_vgfbpd_slot;
        let mut var_vgfbpd_dn10: f64 = *var_vgfbpd_dn10_slot;
        let mut var_vgfbpd_dn11: f64 = *var_vgfbpd_dn11_slot;
        let mut var_vgfbpd_dn3: f64 = *var_vgfbpd_dn3_slot;
        let mut var_vgfbpd_dn4: f64 = *var_vgfbpd_dn4_slot;
        let mut var_vgfbpd_dn5: f64 = *var_vgfbpd_dn5_slot;
        let mut var_vgfbpd_dn6: f64 = *var_vgfbpd_dn6_slot;
        let mut var_vgfbpd_dn7: f64 = *var_vgfbpd_dn7_slot;
        let mut var_vgfbpd_dn8: f64 = *var_vgfbpd_dn8_slot;
        let mut var_vgfbpd_dn9: f64 = *var_vgfbpd_dn9_slot;
        let mut var_vgfbpd_rv: f64 = *var_vgfbpd_rv_slot;
        let mut var_vth_shift: f64 = *var_vth_shift_slot;
        let mut var_vth_shift_dn10: f64 = *var_vth_shift_dn10_slot;
        let mut var_vth_shift_dn11: f64 = *var_vth_shift_dn11_slot;
        let mut var_vth_shift_dn3: f64 = *var_vth_shift_dn3_slot;
        let mut var_vth_shift_dn4: f64 = *var_vth_shift_dn4_slot;
        let mut var_vth_shift_dn5: f64 = *var_vth_shift_dn5_slot;
        let mut var_vth_shift_dn6: f64 = *var_vth_shift_dn6_slot;
        let mut var_vth_shift_dn7: f64 = *var_vth_shift_dn7_slot;
        let mut var_vth_shift_dn8: f64 = *var_vth_shift_dn8_slot;
        let mut var_vth_shift_dn9: f64 = *var_vth_shift_dn9_slot;
        let mut var_vth_shift_rv: f64 = *var_vth_shift_rv_slot;

        let (assign59300_e96342, assign59300_e96342_d_n3, assign59300_e96342_d_n4, assign59300_e96342_d_n5, assign59300_e96342_d_n6, assign59300_e96342_d_n7, assign59300_e96342_d_n8, assign59300_e96342_d_n9, assign59300_e96342_d_n10, assign59300_e96342_d_n11,) = {
    if (((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard868 != 0.0)) {
        let assign59300_e96338: f64 = (p.p1263 * var_leff);
        let assign59300_e96340: f64 = (assign59300_e96338 / var_litl_edge);
        (assign59300_e96340, (-((assign59300_e96338 * var_litl_edge_dn3) / (var_litl_edge * var_litl_edge))), (-((assign59300_e96338 * var_litl_edge_dn4) / (var_litl_edge * var_litl_edge))), (-((assign59300_e96338 * var_litl_edge_dn5) / (var_litl_edge * var_litl_edge))), (-((assign59300_e96338 * var_litl_edge_dn6) / (var_litl_edge * var_litl_edge))), (-((assign59300_e96338 * var_litl_edge_dn7) / (var_litl_edge * var_litl_edge))), (-((assign59300_e96338 * var_litl_edge_dn8) / (var_litl_edge * var_litl_edge))), (-((assign59300_e96338 * var_litl_edge_dn9) / (var_litl_edge * var_litl_edge))), (-((assign59300_e96338 * var_litl_edge_dn10) / (var_litl_edge * var_litl_edge))), (-((assign59300_e96338 * var_litl_edge_dn11) / (var_litl_edge * var_litl_edge))),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11,)
    }
};
        var_t0 = assign59300_e96342;
        var_t0_dn3 = assign59300_e96342_d_n3;
        var_t0_dn4 = assign59300_e96342_d_n4;
        var_t0_dn5 = assign59300_e96342_d_n5;
        var_t0_dn6 = assign59300_e96342_d_n6;
        var_t0_dn7 = assign59300_e96342_d_n7;
        var_t0_dn8 = assign59300_e96342_d_n8;
        var_t0_dn9 = assign59300_e96342_d_n9;
        var_t0_dn10 = assign59300_e96342_d_n10;
        var_t0_dn11 = assign59300_e96342_d_n11;
        var_t0_rv = 0.0;

        let assign59310_e96345: f64 = if var_t0 < 40.0 { 1.0 } else { 0.0 };
        var_guard869 = assign59310_e96345;
        var_guard869_rv = 0.0;

        let (assign59320_e96363, assign59320_e96363_d_n3, assign59320_e96363_d_n4, assign59320_e96363_d_n5, assign59320_e96363_d_n6, assign59320_e96363_d_n7, assign59320_e96363_d_n8, assign59320_e96363_d_n9, assign59320_e96363_d_n10, assign59320_e96363_d_n11,) = {
    if ((((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard868 != 0.0)) && (var_guard869 != 0.0)) {
        let assign59320_e96356: f64 = (0.5 * p.p1262);
        let assign59320_e96358: f64 = (var_t0).cosh();
        let assign59320_e96360: f64 = (assign59320_e96358 - 1.0);
        let assign59320_e96361: f64 = (assign59320_e96356 / assign59320_e96360);
        (assign59320_e96361, (-((assign59320_e96356 * ((var_t0).sinh() * var_t0_dn3)) / (assign59320_e96360 * assign59320_e96360))), (-((assign59320_e96356 * ((var_t0).sinh() * var_t0_dn4)) / (assign59320_e96360 * assign59320_e96360))), (-((assign59320_e96356 * ((var_t0).sinh() * var_t0_dn5)) / (assign59320_e96360 * assign59320_e96360))), (-((assign59320_e96356 * ((var_t0).sinh() * var_t0_dn6)) / (assign59320_e96360 * assign59320_e96360))), (-((assign59320_e96356 * ((var_t0).sinh() * var_t0_dn7)) / (assign59320_e96360 * assign59320_e96360))), (-((assign59320_e96356 * ((var_t0).sinh() * var_t0_dn8)) / (assign59320_e96360 * assign59320_e96360))), (-((assign59320_e96356 * ((var_t0).sinh() * var_t0_dn9)) / (assign59320_e96360 * assign59320_e96360))), (-((assign59320_e96356 * ((var_t0).sinh() * var_t0_dn10)) / (assign59320_e96360 * assign59320_e96360))), (-((assign59320_e96356 * ((var_t0).sinh() * var_t0_dn11)) / (assign59320_e96360 * assign59320_e96360))),)
    } else {
        (var_theta_sce_edge, var_theta_sce_edge_dn3, var_theta_sce_edge_dn4, var_theta_sce_edge_dn5, var_theta_sce_edge_dn6, var_theta_sce_edge_dn7, var_theta_sce_edge_dn8, var_theta_sce_edge_dn9, var_theta_sce_edge_dn10, var_theta_sce_edge_dn11,)
    }
};
        var_theta_sce_edge = assign59320_e96363;
        var_theta_sce_edge_dn3 = assign59320_e96363_d_n3;
        var_theta_sce_edge_dn4 = assign59320_e96363_d_n4;
        var_theta_sce_edge_dn5 = assign59320_e96363_d_n5;
        var_theta_sce_edge_dn6 = assign59320_e96363_d_n6;
        var_theta_sce_edge_dn7 = assign59320_e96363_d_n7;
        var_theta_sce_edge_dn8 = assign59320_e96363_d_n8;
        var_theta_sce_edge_dn9 = assign59320_e96363_d_n9;
        var_theta_sce_edge_dn10 = assign59320_e96363_d_n10;
        var_theta_sce_edge_dn11 = assign59320_e96363_d_n11;
        var_theta_sce_edge_rv = 0.0;

        let (assign59330_e96379, assign59330_e96379_d_n3, assign59330_e96379_d_n4, assign59330_e96379_d_n5, assign59330_e96379_d_n6, assign59330_e96379_d_n7, assign59330_e96379_d_n8, assign59330_e96379_d_n9, assign59330_e96379_d_n10, assign59330_e96379_d_n11,) = {
    if ((((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard868 != 0.0)) && (var_guard869 == 0.0)) {
        let assign59330_e96375: f64 = (-var_t0);
        let assign59330_e96376: f64 = { let limited_exp_arg = assign59330_e96375; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign59330_e96377: f64 = (p.p1262 * assign59330_e96376);
        (assign59330_e96377, (p.p1262 * ({ let limited_exp_arg = assign59330_e96375; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t0_dn3))), (p.p1262 * ({ let limited_exp_arg = assign59330_e96375; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t0_dn4))), (p.p1262 * ({ let limited_exp_arg = assign59330_e96375; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t0_dn5))), (p.p1262 * ({ let limited_exp_arg = assign59330_e96375; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t0_dn6))), (p.p1262 * ({ let limited_exp_arg = assign59330_e96375; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t0_dn7))), (p.p1262 * ({ let limited_exp_arg = assign59330_e96375; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t0_dn8))), (p.p1262 * ({ let limited_exp_arg = assign59330_e96375; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t0_dn9))), (p.p1262 * ({ let limited_exp_arg = assign59330_e96375; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t0_dn10))), (p.p1262 * ({ let limited_exp_arg = assign59330_e96375; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t0_dn11))),)
    } else {
        (var_theta_sce_edge, var_theta_sce_edge_dn3, var_theta_sce_edge_dn4, var_theta_sce_edge_dn5, var_theta_sce_edge_dn6, var_theta_sce_edge_dn7, var_theta_sce_edge_dn8, var_theta_sce_edge_dn9, var_theta_sce_edge_dn10, var_theta_sce_edge_dn11,)
    }
};
        var_theta_sce_edge = assign59330_e96379;
        var_theta_sce_edge_dn3 = assign59330_e96379_d_n3;
        var_theta_sce_edge_dn4 = assign59330_e96379_d_n4;
        var_theta_sce_edge_dn5 = assign59330_e96379_d_n5;
        var_theta_sce_edge_dn6 = assign59330_e96379_d_n6;
        var_theta_sce_edge_dn7 = assign59330_e96379_d_n7;
        var_theta_sce_edge_dn8 = assign59330_e96379_d_n8;
        var_theta_sce_edge_dn9 = assign59330_e96379_d_n9;
        var_theta_sce_edge_dn10 = assign59330_e96379_d_n10;
        var_theta_sce_edge_dn11 = assign59330_e96379_d_n11;
        var_theta_sce_edge_rv = 0.0;

        let (assign59340_e96389, assign59340_e96389_d_n3, assign59340_e96389_d_n4, assign59340_e96389_d_n5, assign59340_e96389_d_n6, assign59340_e96389_d_n7, assign59340_e96389_d_n8, assign59340_e96389_d_n9, assign59340_e96389_d_n10, assign59340_e96389_d_n11,) = {
    if (((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard868 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_theta_sce_edge, var_theta_sce_edge_dn3, var_theta_sce_edge_dn4, var_theta_sce_edge_dn5, var_theta_sce_edge_dn6, var_theta_sce_edge_dn7, var_theta_sce_edge_dn8, var_theta_sce_edge_dn9, var_theta_sce_edge_dn10, var_theta_sce_edge_dn11,)
    }
};
        var_theta_sce_edge = assign59340_e96389;
        var_theta_sce_edge_dn3 = assign59340_e96389_d_n3;
        var_theta_sce_edge_dn4 = assign59340_e96389_d_n4;
        var_theta_sce_edge_dn5 = assign59340_e96389_d_n5;
        var_theta_sce_edge_dn6 = assign59340_e96389_d_n6;
        var_theta_sce_edge_dn7 = assign59340_e96389_d_n7;
        var_theta_sce_edge_dn8 = assign59340_e96389_d_n8;
        var_theta_sce_edge_dn9 = assign59340_e96389_d_n9;
        var_theta_sce_edge_dn10 = assign59340_e96389_d_n10;
        var_theta_sce_edge_dn11 = assign59340_e96389_d_n11;
        var_theta_sce_edge_rv = 0.0;

        let (assign59350_e96400, assign59350_e96400_d_n3, assign59350_e96400_d_n4, assign59350_e96400_d_n5, assign59350_e96400_d_n6, assign59350_e96400_d_n7, assign59350_e96400_d_n8, assign59350_e96400_d_n9, assign59350_e96400_d_n10, assign59350_e96400_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59350_e96397: f64 = (var_vbi_edge - var_phist);
        let assign59350_e96398: f64 = (var_theta_sce_edge * assign59350_e96397);
        (assign59350_e96398, ((var_theta_sce_edge_dn3 * assign59350_e96397) + (var_theta_sce_edge * (var_vbi_edge_dn3 - var_phist_dn3))), ((var_theta_sce_edge_dn4 * assign59350_e96397) + (var_theta_sce_edge * (var_vbi_edge_dn4 - var_phist_dn4))), ((var_theta_sce_edge_dn5 * assign59350_e96397) + (var_theta_sce_edge * (var_vbi_edge_dn5 - var_phist_dn5))), ((var_theta_sce_edge_dn6 * assign59350_e96397) + (var_theta_sce_edge * (var_vbi_edge_dn6 - var_phist_dn6))), ((var_theta_sce_edge_dn7 * assign59350_e96397) + (var_theta_sce_edge * (var_vbi_edge_dn7 - var_phist_dn7))), ((var_theta_sce_edge_dn8 * assign59350_e96397) + (var_theta_sce_edge * (var_vbi_edge_dn8 - var_phist_dn8))), ((var_theta_sce_edge_dn9 * assign59350_e96397) + (var_theta_sce_edge * (var_vbi_edge_dn9 - var_phist_dn9))), ((var_theta_sce_edge_dn10 * assign59350_e96397) + (var_theta_sce_edge * (var_vbi_edge_dn10 - var_phist_dn10))), ((var_theta_sce_edge_dn11 * assign59350_e96397) + (var_theta_sce_edge * (var_vbi_edge_dn11 - var_phist_dn11))),)
    } else {
        (var_dvth_sce, var_dvth_sce_dn3, var_dvth_sce_dn4, var_dvth_sce_dn5, var_dvth_sce_dn6, var_dvth_sce_dn7, var_dvth_sce_dn8, var_dvth_sce_dn9, var_dvth_sce_dn10, var_dvth_sce_dn11,)
    }
};
        var_dvth_sce = assign59350_e96400;
        var_dvth_sce_dn3 = assign59350_e96400_d_n3;
        var_dvth_sce_dn4 = assign59350_e96400_d_n4;
        var_dvth_sce_dn5 = assign59350_e96400_d_n5;
        var_dvth_sce_dn6 = assign59350_e96400_d_n6;
        var_dvth_sce_dn7 = assign59350_e96400_d_n7;
        var_dvth_sce_dn8 = assign59350_e96400_d_n8;
        var_dvth_sce_dn9 = assign59350_e96400_d_n9;
        var_dvth_sce_dn10 = assign59350_e96400_d_n10;
        var_dvth_sce_dn11 = assign59350_e96400_d_n11;
        var_dvth_sce_rv = 0.0;

        let (assign59360_e96421, assign59360_e96421_d_n3, assign59360_e96421_d_n4, assign59360_e96421_d_n5, assign59360_e96421_d_n6, assign59360_e96421_d_n7, assign59360_e96421_d_n8, assign59360_e96421_d_n9, assign59360_e96421_d_n10, assign59360_e96421_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59360_e96407: f64 = (var_dvth_dibl_1 - var_dvth_temp);
        let assign59360_e96409: f64 = (assign59360_e96407 + var_dvth_sce);
        let assign59360_e96411: f64 = (assign59360_e96409 + p.p1151);
        let assign59360_e96413: f64 = (assign59360_e96411 + var_vth0_stress_edge);
        let assign59360_e96416: f64 = (var_k2edge_i * var_vbsx);
        let assign59360_e96417: f64 = (assign59360_e96413 - assign59360_e96416);
        let assign59360_e96419: f64 = (assign59360_e96417 + var_vth0_well_edge);
        (assign59360_e96419, (((((var_dvth_dibl_1_dn3 - var_dvth_temp_dn3) + var_dvth_sce_dn3) + var_vth0_stress_edge_dn3) - ((var_k2edge_i_dn3 * var_vbsx) + (var_k2edge_i * var_vbsx_dn3))) + var_vth0_well_edge_dn3), (((((var_dvth_dibl_1_dn4 - var_dvth_temp_dn4) + var_dvth_sce_dn4) + var_vth0_stress_edge_dn4) - ((var_k2edge_i_dn4 * var_vbsx) + (var_k2edge_i * var_vbsx_dn4))) + var_vth0_well_edge_dn4), (((((var_dvth_dibl_1_dn5 - var_dvth_temp_dn5) + var_dvth_sce_dn5) + var_vth0_stress_edge_dn5) - ((var_k2edge_i_dn5 * var_vbsx) + (var_k2edge_i * var_vbsx_dn5))) + var_vth0_well_edge_dn5), (((((var_dvth_dibl_1_dn6 - var_dvth_temp_dn6) + var_dvth_sce_dn6) + var_vth0_stress_edge_dn6) - ((var_k2edge_i_dn6 * var_vbsx) + (var_k2edge_i * var_vbsx_dn6))) + var_vth0_well_edge_dn6), (((((var_dvth_dibl_1_dn7 - var_dvth_temp_dn7) + var_dvth_sce_dn7) + var_vth0_stress_edge_dn7) - ((var_k2edge_i_dn7 * var_vbsx) + (var_k2edge_i * var_vbsx_dn7))) + var_vth0_well_edge_dn7), (((((var_dvth_dibl_1_dn8 - var_dvth_temp_dn8) + var_dvth_sce_dn8) + var_vth0_stress_edge_dn8) - ((var_k2edge_i_dn8 * var_vbsx) + (var_k2edge_i * var_vbsx_dn8))) + var_vth0_well_edge_dn8), (((((var_dvth_dibl_1_dn9 - var_dvth_temp_dn9) + var_dvth_sce_dn9) + var_vth0_stress_edge_dn9) - ((var_k2edge_i_dn9 * var_vbsx) + (var_k2edge_i * var_vbsx_dn9))) + var_vth0_well_edge_dn9), (((((var_dvth_dibl_1_dn10 - var_dvth_temp_dn10) + var_dvth_sce_dn10) + var_vth0_stress_edge_dn10) - ((var_k2edge_i_dn10 * var_vbsx) + (var_k2edge_i * var_vbsx_dn10))) + var_vth0_well_edge_dn10), (((((var_dvth_dibl_1_dn11 - var_dvth_temp_dn11) + var_dvth_sce_dn11) + var_vth0_stress_edge_dn11) - ((var_k2edge_i_dn11 * var_vbsx) + (var_k2edge_i * var_vbsx_dn11))) + var_vth0_well_edge_dn11),)
    } else {
        (var_vth_shift, var_vth_shift_dn3, var_vth_shift_dn4, var_vth_shift_dn5, var_vth_shift_dn6, var_vth_shift_dn7, var_vth_shift_dn8, var_vth_shift_dn9, var_vth_shift_dn10, var_vth_shift_dn11,)
    }
};
        var_vth_shift = assign59360_e96421;
        var_vth_shift_dn3 = assign59360_e96421_d_n3;
        var_vth_shift_dn4 = assign59360_e96421_d_n4;
        var_vth_shift_dn5 = assign59360_e96421_d_n5;
        var_vth_shift_dn6 = assign59360_e96421_d_n6;
        var_vth_shift_dn7 = assign59360_e96421_d_n7;
        var_vth_shift_dn8 = assign59360_e96421_d_n8;
        var_vth_shift_dn9 = assign59360_e96421_d_n9;
        var_vth_shift_dn10 = assign59360_e96421_d_n10;
        var_vth_shift_dn11 = assign59360_e96421_d_n11;
        var_vth_shift_rv = 0.0;

        let (assign59370_e96434, assign59370_e96434_d_n3, assign59370_e96434_d_n4, assign59370_e96434_d_n5, assign59370_e96434_d_n6, assign59370_e96434_d_n7, assign59370_e96434_d_n8, assign59370_e96434_d_n9, assign59370_e96434_d_n10, assign59370_e96434_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59370_e96428: f64 = (var_vg_1 - var_vfb);
        let assign59370_e96431: f64 = (var_vth_shift * var_inv_nvt);
        let assign59370_e96432: f64 = (assign59370_e96428 - assign59370_e96431);
        (assign59370_e96432, ((var_vg_1_dn3 - var_vfb_dn3) - ((var_vth_shift_dn3 * var_inv_nvt) + (var_vth_shift * var_inv_nvt_dn3))), ((var_vg_1_dn4 - var_vfb_dn4) - ((var_vth_shift_dn4 * var_inv_nvt) + (var_vth_shift * var_inv_nvt_dn4))), ((var_vg_1_dn5 - var_vfb_dn5) - ((var_vth_shift_dn5 * var_inv_nvt) + (var_vth_shift * var_inv_nvt_dn5))), ((var_vg_1_dn6 - var_vfb_dn6) - ((var_vth_shift_dn6 * var_inv_nvt) + (var_vth_shift * var_inv_nvt_dn6))), ((var_vg_1_dn7 - var_vfb_dn7) - ((var_vth_shift_dn7 * var_inv_nvt) + (var_vth_shift * var_inv_nvt_dn7))), ((var_vg_1_dn8 - var_vfb_dn8) - ((var_vth_shift_dn8 * var_inv_nvt) + (var_vth_shift * var_inv_nvt_dn8))), ((var_vg_1_dn9 - var_vfb_dn9) - ((var_vth_shift_dn9 * var_inv_nvt) + (var_vth_shift * var_inv_nvt_dn9))), ((var_vg_1_dn10 - var_vfb_dn10) - ((var_vth_shift_dn10 * var_inv_nvt) + (var_vth_shift * var_inv_nvt_dn10))), ((var_vg_1_dn11 - var_vfb_dn11) - ((var_vth_shift_dn11 * var_inv_nvt) + (var_vth_shift * var_inv_nvt_dn11))),)
    } else {
        (var_vgfb, var_vgfb_dn3, var_vgfb_dn4, var_vgfb_dn5, var_vgfb_dn6, var_vgfb_dn7, var_vgfb_dn8, var_vgfb_dn9, var_vgfb_dn10, var_vgfb_dn11,)
    }
};
        var_vgfb = assign59370_e96434;
        var_vgfb_dn3 = assign59370_e96434_d_n3;
        var_vgfb_dn4 = assign59370_e96434_d_n4;
        var_vgfb_dn5 = assign59370_e96434_d_n5;
        var_vgfb_dn6 = assign59370_e96434_d_n6;
        var_vgfb_dn7 = assign59370_e96434_d_n7;
        var_vgfb_dn8 = assign59370_e96434_d_n8;
        var_vgfb_dn9 = assign59370_e96434_d_n9;
        var_vgfb_dn10 = assign59370_e96434_d_n10;
        var_vgfb_dn11 = assign59370_e96434_d_n11;
        var_vgfb_rv = 0.0;

        let (assign59380_e96450,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59380_e96444: f64 = (-p.p1150);
        let assign59380_e96445: f64 = (var_leff).powf(assign59380_e96444);
        let assign59380_e96446: f64 = (p.p1149 * assign59380_e96445);
        let assign59380_e96447: f64 = (1.0 + assign59380_e96446);
        let assign59380_e96448: f64 = (p.p1148 * assign59380_e96447);
        (assign59380_e96448,)
    } else {
        (var_dgammaedge_i,)
    }
};
        var_dgammaedge_i = assign59380_e96450;
        var_dgammaedge_i_rv = 0.0;

        let (assign59390_e96468, assign59390_e96468_d_n3, assign59390_e96468_d_n4, assign59390_e96468_d_n5, assign59390_e96468_d_n6, assign59390_e96468_d_n7, assign59390_e96468_d_n8, assign59390_e96468_d_n9, assign59390_e96468_d_n10, assign59390_e96468_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59390_e96457: f64 = (2.0 * 1.602176462e-19);
        let assign59390_e96459: f64 = (assign59390_e96457 * var_epssi);
        let assign59390_e96461: f64 = (assign59390_e96459 * var_ndepedge_i);
        let assign59390_e96463: f64 = (assign59390_e96461 * var_inv_nvt);
        let assign59390_e96464: f64 = (assign59390_e96463).sqrt();
        let assign59390_e96466: f64 = (assign59390_e96464 / var_cox);
        (assign59390_e96466, (((assign59390_e96461 * var_inv_nvt_dn3) / (2.0 * assign59390_e96464)) / var_cox), (((assign59390_e96461 * var_inv_nvt_dn4) / (2.0 * assign59390_e96464)) / var_cox), (((assign59390_e96461 * var_inv_nvt_dn5) / (2.0 * assign59390_e96464)) / var_cox), (((assign59390_e96461 * var_inv_nvt_dn6) / (2.0 * assign59390_e96464)) / var_cox), (((assign59390_e96461 * var_inv_nvt_dn7) / (2.0 * assign59390_e96464)) / var_cox), (((assign59390_e96461 * var_inv_nvt_dn8) / (2.0 * assign59390_e96464)) / var_cox), (((assign59390_e96461 * var_inv_nvt_dn9) / (2.0 * assign59390_e96464)) / var_cox), (((assign59390_e96461 * var_inv_nvt_dn10) / (2.0 * assign59390_e96464)) / var_cox), (((assign59390_e96461 * var_inv_nvt_dn11) / (2.0 * assign59390_e96464)) / var_cox),)
    } else {
        (var_gam_edge, var_gam_edge_dn3, var_gam_edge_dn4, var_gam_edge_dn5, var_gam_edge_dn6, var_gam_edge_dn7, var_gam_edge_dn8, var_gam_edge_dn9, var_gam_edge_dn10, var_gam_edge_dn11,)
    }
};
        var_gam_edge = assign59390_e96468;
        var_gam_edge_dn3 = assign59390_e96468_d_n3;
        var_gam_edge_dn4 = assign59390_e96468_d_n4;
        var_gam_edge_dn5 = assign59390_e96468_d_n5;
        var_gam_edge_dn6 = assign59390_e96468_d_n6;
        var_gam_edge_dn7 = assign59390_e96468_d_n7;
        var_gam_edge_dn8 = assign59390_e96468_d_n8;
        var_gam_edge_dn9 = assign59390_e96468_d_n9;
        var_gam_edge_dn10 = assign59390_e96468_d_n10;
        var_gam_edge_dn11 = assign59390_e96468_d_n11;
        var_gam_edge_rv = 0.0;

        let (assign59400_e96479, assign59400_e96479_d_n3, assign59400_e96479_d_n4, assign59400_e96479_d_n5, assign59400_e96479_d_n6, assign59400_e96479_d_n7, assign59400_e96479_d_n8, assign59400_e96479_d_n9, assign59400_e96479_d_n10, assign59400_e96479_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59400_e96476: f64 = (1.0 + var_dgammaedge_i);
        let assign59400_e96477: f64 = (var_gam_edge * assign59400_e96476);
        (assign59400_e96477, (var_gam_edge_dn3 * assign59400_e96476), (var_gam_edge_dn4 * assign59400_e96476), (var_gam_edge_dn5 * assign59400_e96476), (var_gam_edge_dn6 * assign59400_e96476), (var_gam_edge_dn7 * assign59400_e96476), (var_gam_edge_dn8 * assign59400_e96476), (var_gam_edge_dn9 * assign59400_e96476), (var_gam_edge_dn10 * assign59400_e96476), (var_gam_edge_dn11 * assign59400_e96476),)
    } else {
        (var_gam_edge, var_gam_edge_dn3, var_gam_edge_dn4, var_gam_edge_dn5, var_gam_edge_dn6, var_gam_edge_dn7, var_gam_edge_dn8, var_gam_edge_dn9, var_gam_edge_dn10, var_gam_edge_dn11,)
    }
};
        var_gam_edge = assign59400_e96479;
        var_gam_edge_dn3 = assign59400_e96479_d_n3;
        var_gam_edge_dn4 = assign59400_e96479_d_n4;
        var_gam_edge_dn5 = assign59400_e96479_d_n5;
        var_gam_edge_dn6 = assign59400_e96479_d_n6;
        var_gam_edge_dn7 = assign59400_e96479_d_n7;
        var_gam_edge_dn8 = assign59400_e96479_d_n8;
        var_gam_edge_dn9 = assign59400_e96479_d_n9;
        var_gam_edge_dn10 = assign59400_e96479_d_n10;
        var_gam_edge_dn11 = assign59400_e96479_d_n11;
        var_gam_edge_rv = 0.0;

        let (assign59410_e96488, assign59410_e96488_d_n3, assign59410_e96488_d_n4, assign59410_e96488_d_n5, assign59410_e96488_d_n6, assign59410_e96488_d_n7, assign59410_e96488_d_n8, assign59410_e96488_d_n9, assign59410_e96488_d_n10, assign59410_e96488_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59410_e96486: f64 = (var_phib_edge / var_n);
        (assign59410_e96486, (((var_phib_edge_dn3 * var_n) - (var_phib_edge * var_n_dn3)) / (var_n * var_n)), (((var_phib_edge_dn4 * var_n) - (var_phib_edge * var_n_dn4)) / (var_n * var_n)), (((var_phib_edge_dn5 * var_n) - (var_phib_edge * var_n_dn5)) / (var_n * var_n)), (((var_phib_edge_dn6 * var_n) - (var_phib_edge * var_n_dn6)) / (var_n * var_n)), (((var_phib_edge_dn7 * var_n) - (var_phib_edge * var_n_dn7)) / (var_n * var_n)), (((var_phib_edge_dn8 * var_n) - (var_phib_edge * var_n_dn8)) / (var_n * var_n)), (((var_phib_edge_dn9 * var_n) - (var_phib_edge * var_n_dn9)) / (var_n * var_n)), (((var_phib_edge_dn10 * var_n) - (var_phib_edge * var_n_dn10)) / (var_n * var_n)), (((var_phib_edge_dn11 * var_n) - (var_phib_edge * var_n_dn11)) / (var_n * var_n)),)
    } else {
        (var_phib_n_edge, var_phib_n_edge_dn3, var_phib_n_edge_dn4, var_phib_n_edge_dn5, var_phib_n_edge_dn6, var_phib_n_edge_dn7, var_phib_n_edge_dn8, var_phib_n_edge_dn9, var_phib_n_edge_dn10, var_phib_n_edge_dn11,)
    }
};
        var_phib_n_edge = assign59410_e96488;
        var_phib_n_edge_dn3 = assign59410_e96488_d_n3;
        var_phib_n_edge_dn4 = assign59410_e96488_d_n4;
        var_phib_n_edge_dn5 = assign59410_e96488_d_n5;
        var_phib_n_edge_dn6 = assign59410_e96488_d_n6;
        var_phib_n_edge_dn7 = assign59410_e96488_d_n7;
        var_phib_n_edge_dn8 = assign59410_e96488_d_n8;
        var_phib_n_edge_dn9 = assign59410_e96488_d_n9;
        var_phib_n_edge_dn10 = assign59410_e96488_d_n10;
        var_phib_n_edge_dn11 = assign59410_e96488_d_n11;
        var_phib_n_edge_rv = 0.0;

        let (assign59420_e96497, assign59420_e96497_d_n3, assign59420_e96497_d_n4, assign59420_e96497_d_n5, assign59420_e96497_d_n6, assign59420_e96497_d_n7, assign59420_e96497_d_n8, assign59420_e96497_d_n9, assign59420_e96497_d_n10, assign59420_e96497_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59420_e96495: f64 = 1.0;
        (assign59420_e96495, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11,)
    }
};
        var_t1 = assign59420_e96497;
        var_t1_dn3 = assign59420_e96497_d_n3;
        var_t1_dn4 = assign59420_e96497_d_n4;
        var_t1_dn5 = assign59420_e96497_d_n5;
        var_t1_dn6 = assign59420_e96497_d_n6;
        var_t1_dn7 = assign59420_e96497_d_n7;
        var_t1_dn8 = assign59420_e96497_d_n8;
        var_t1_dn9 = assign59420_e96497_d_n9;
        var_t1_dn10 = assign59420_e96497_d_n10;
        var_t1_dn11 = assign59420_e96497_d_n11;
        var_t1_rv = 0.0;

        let (assign59430_e96506, assign59430_e96506_d_n3, assign59430_e96506_d_n4, assign59430_e96506_d_n5, assign59430_e96506_d_n6, assign59430_e96506_d_n7, assign59430_e96506_d_n8, assign59430_e96506_d_n9, assign59430_e96506_d_n10, assign59430_e96506_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59430_e96504: f64 = (var_vgfb / var_t1);
        (assign59430_e96504, (((var_vgfb_dn3 * var_t1) - (var_vgfb * var_t1_dn3)) / (var_t1 * var_t1)), (((var_vgfb_dn4 * var_t1) - (var_vgfb * var_t1_dn4)) / (var_t1 * var_t1)), (((var_vgfb_dn5 * var_t1) - (var_vgfb * var_t1_dn5)) / (var_t1 * var_t1)), (((var_vgfb_dn6 * var_t1) - (var_vgfb * var_t1_dn6)) / (var_t1 * var_t1)), (((var_vgfb_dn7 * var_t1) - (var_vgfb * var_t1_dn7)) / (var_t1 * var_t1)), (((var_vgfb_dn8 * var_t1) - (var_vgfb * var_t1_dn8)) / (var_t1 * var_t1)), (((var_vgfb_dn9 * var_t1) - (var_vgfb * var_t1_dn9)) / (var_t1 * var_t1)), (((var_vgfb_dn10 * var_t1) - (var_vgfb * var_t1_dn10)) / (var_t1 * var_t1)), (((var_vgfb_dn11 * var_t1) - (var_vgfb * var_t1_dn11)) / (var_t1 * var_t1)),)
    } else {
        (var_vgfbpd, var_vgfbpd_dn3, var_vgfbpd_dn4, var_vgfbpd_dn5, var_vgfbpd_dn6, var_vgfbpd_dn7, var_vgfbpd_dn8, var_vgfbpd_dn9, var_vgfbpd_dn10, var_vgfbpd_dn11,)
    }
};
        var_vgfbpd = assign59430_e96506;
        var_vgfbpd_dn3 = assign59430_e96506_d_n3;
        var_vgfbpd_dn4 = assign59430_e96506_d_n4;
        var_vgfbpd_dn5 = assign59430_e96506_d_n5;
        var_vgfbpd_dn6 = assign59430_e96506_d_n6;
        var_vgfbpd_dn7 = assign59430_e96506_d_n7;
        var_vgfbpd_dn8 = assign59430_e96506_d_n8;
        var_vgfbpd_dn9 = assign59430_e96506_d_n9;
        var_vgfbpd_dn10 = assign59430_e96506_d_n10;
        var_vgfbpd_dn11 = assign59430_e96506_d_n11;
        var_vgfbpd_rv = 0.0;

        let (assign59440_e96515, assign59440_e96515_d_n3, assign59440_e96515_d_n4, assign59440_e96515_d_n5, assign59440_e96515_d_n6, assign59440_e96515_d_n7, assign59440_e96515_d_n8, assign59440_e96515_d_n9, assign59440_e96515_d_n10, assign59440_e96515_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59440_e96513: f64 = (var_gam_edge / var_t1);
        (assign59440_e96513, (((var_gam_edge_dn3 * var_t1) - (var_gam_edge * var_t1_dn3)) / (var_t1 * var_t1)), (((var_gam_edge_dn4 * var_t1) - (var_gam_edge * var_t1_dn4)) / (var_t1 * var_t1)), (((var_gam_edge_dn5 * var_t1) - (var_gam_edge * var_t1_dn5)) / (var_t1 * var_t1)), (((var_gam_edge_dn6 * var_t1) - (var_gam_edge * var_t1_dn6)) / (var_t1 * var_t1)), (((var_gam_edge_dn7 * var_t1) - (var_gam_edge * var_t1_dn7)) / (var_t1 * var_t1)), (((var_gam_edge_dn8 * var_t1) - (var_gam_edge * var_t1_dn8)) / (var_t1 * var_t1)), (((var_gam_edge_dn9 * var_t1) - (var_gam_edge * var_t1_dn9)) / (var_t1 * var_t1)), (((var_gam_edge_dn10 * var_t1) - (var_gam_edge * var_t1_dn10)) / (var_t1 * var_t1)), (((var_gam_edge_dn11 * var_t1) - (var_gam_edge * var_t1_dn11)) / (var_t1 * var_t1)),)
    } else {
        (var_gammapd, var_gammapd_dn3, var_gammapd_dn4, var_gammapd_dn5, var_gammapd_dn6, var_gammapd_dn7, var_gammapd_dn8, var_gammapd_dn9, var_gammapd_dn10, var_gammapd_dn11,)
    }
};
        var_gammapd = assign59440_e96515;
        var_gammapd_dn3 = assign59440_e96515_d_n3;
        var_gammapd_dn4 = assign59440_e96515_d_n4;
        var_gammapd_dn5 = assign59440_e96515_d_n5;
        var_gammapd_dn6 = assign59440_e96515_d_n6;
        var_gammapd_dn7 = assign59440_e96515_d_n7;
        var_gammapd_dn8 = assign59440_e96515_d_n8;
        var_gammapd_dn9 = assign59440_e96515_d_n9;
        var_gammapd_dn10 = assign59440_e96515_d_n10;
        var_gammapd_dn11 = assign59440_e96515_d_n11;
        var_gammapd_rv = 0.0;

        let (assign59450_e96532, assign59450_e96532_d_n3, assign59450_e96532_d_n4, assign59450_e96532_d_n5, assign59450_e96532_d_n6, assign59450_e96532_d_n7, assign59450_e96532_d_n8, assign59450_e96532_d_n9, assign59450_e96532_d_n10, assign59450_e96532_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59450_e96522: f64 = (0.5 * var_vgfbpd);
        let assign59450_e96527: f64 = (var_gammapd / 1.4142135623730951);
        let assign59450_e96528: f64 = (1.0 + assign59450_e96527);
        let assign59450_e96529: f64 = (3.0 * assign59450_e96528);
        let assign59450_e96530: f64 = (assign59450_e96522 - assign59450_e96529);
        (assign59450_e96530, ((0.5 * var_vgfbpd_dn3) - (3.0 * (var_gammapd_dn3 / 1.4142135623730951))), ((0.5 * var_vgfbpd_dn4) - (3.0 * (var_gammapd_dn4 / 1.4142135623730951))), ((0.5 * var_vgfbpd_dn5) - (3.0 * (var_gammapd_dn5 / 1.4142135623730951))), ((0.5 * var_vgfbpd_dn6) - (3.0 * (var_gammapd_dn6 / 1.4142135623730951))), ((0.5 * var_vgfbpd_dn7) - (3.0 * (var_gammapd_dn7 / 1.4142135623730951))), ((0.5 * var_vgfbpd_dn8) - (3.0 * (var_gammapd_dn8 / 1.4142135623730951))), ((0.5 * var_vgfbpd_dn9) - (3.0 * (var_gammapd_dn9 / 1.4142135623730951))), ((0.5 * var_vgfbpd_dn10) - (3.0 * (var_gammapd_dn10 / 1.4142135623730951))), ((0.5 * var_vgfbpd_dn11) - (3.0 * (var_gammapd_dn11 / 1.4142135623730951))),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11,)
    }
};
        var_t1 = assign59450_e96532;
        var_t1_dn3 = assign59450_e96532_d_n3;
        var_t1_dn4 = assign59450_e96532_d_n4;
        var_t1_dn5 = assign59450_e96532_d_n5;
        var_t1_dn6 = assign59450_e96532_d_n6;
        var_t1_dn7 = assign59450_e96532_d_n7;
        var_t1_dn8 = assign59450_e96532_d_n8;
        var_t1_dn9 = assign59450_e96532_d_n9;
        var_t1_dn10 = assign59450_e96532_d_n10;
        var_t1_dn11 = assign59450_e96532_d_n11;
        var_t1_rv = 0.0;

        let (assign59460_e96548, assign59460_e96548_d_n3, assign59460_e96548_d_n4, assign59460_e96548_d_n5, assign59460_e96548_d_n6, assign59460_e96548_d_n7, assign59460_e96548_d_n8, assign59460_e96548_d_n9, assign59460_e96548_d_n10, assign59460_e96548_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59460_e96540: f64 = (var_t1 * var_t1);
        let assign59460_e96543: f64 = (6.0 * var_vgfbpd);
        let assign59460_e96544: f64 = (assign59460_e96540 + assign59460_e96543);
        let assign59460_e96545: f64 = (assign59460_e96544).sqrt();
        let assign59460_e96546: f64 = (var_t1 + assign59460_e96545);
        (assign59460_e96546, (var_t1_dn3 + ((((var_t1_dn3 * var_t1) + (var_t1 * var_t1_dn3)) + (6.0 * var_vgfbpd_dn3)) / (2.0 * assign59460_e96545))), (var_t1_dn4 + ((((var_t1_dn4 * var_t1) + (var_t1 * var_t1_dn4)) + (6.0 * var_vgfbpd_dn4)) / (2.0 * assign59460_e96545))), (var_t1_dn5 + ((((var_t1_dn5 * var_t1) + (var_t1 * var_t1_dn5)) + (6.0 * var_vgfbpd_dn5)) / (2.0 * assign59460_e96545))), (var_t1_dn6 + ((((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6)) + (6.0 * var_vgfbpd_dn6)) / (2.0 * assign59460_e96545))), (var_t1_dn7 + ((((var_t1_dn7 * var_t1) + (var_t1 * var_t1_dn7)) + (6.0 * var_vgfbpd_dn7)) / (2.0 * assign59460_e96545))), (var_t1_dn8 + ((((var_t1_dn8 * var_t1) + (var_t1 * var_t1_dn8)) + (6.0 * var_vgfbpd_dn8)) / (2.0 * assign59460_e96545))), (var_t1_dn9 + ((((var_t1_dn9 * var_t1) + (var_t1 * var_t1_dn9)) + (6.0 * var_vgfbpd_dn9)) / (2.0 * assign59460_e96545))), (var_t1_dn10 + ((((var_t1_dn10 * var_t1) + (var_t1 * var_t1_dn10)) + (6.0 * var_vgfbpd_dn10)) / (2.0 * assign59460_e96545))), (var_t1_dn11 + ((((var_t1_dn11 * var_t1) + (var_t1 * var_t1_dn11)) + (6.0 * var_vgfbpd_dn11)) / (2.0 * assign59460_e96545))),)
    } else {
        (var_t2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11,)
    }
};
        var_t2 = assign59460_e96548;
        var_t2_dn3 = assign59460_e96548_d_n3;
        var_t2_dn4 = assign59460_e96548_d_n4;
        var_t2_dn5 = assign59460_e96548_d_n5;
        var_t2_dn6 = assign59460_e96548_d_n6;
        var_t2_dn7 = assign59460_e96548_d_n7;
        var_t2_dn8 = assign59460_e96548_d_n8;
        var_t2_dn9 = assign59460_e96548_d_n9;
        var_t2_dn10 = assign59460_e96548_d_n10;
        var_t2_dn11 = assign59460_e96548_d_n11;
        var_t2_rv = 0.0;

        let assign59470_e96551: f64 = if var_vgfbpd < 0.0 { 1.0 } else { 0.0 };
        var_guard870 = assign59470_e96551;
        var_guard870_rv = 0.0;

        let (assign59480_e96564, assign59480_e96564_d_n3, assign59480_e96564_d_n4, assign59480_e96564_d_n5, assign59480_e96564_d_n6, assign59480_e96564_d_n7, assign59480_e96564_d_n8, assign59480_e96564_d_n9, assign59480_e96564_d_n10, assign59480_e96564_d_n11,) = {
    if (((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard870 != 0.0)) {
        let assign59480_e96560: f64 = (var_vgfbpd - var_t2);
        let assign59480_e96562: f64 = (assign59480_e96560 / var_gammapd);
        (assign59480_e96562, ((((var_vgfbpd_dn3 - var_t2_dn3) * var_gammapd) - (assign59480_e96560 * var_gammapd_dn3)) / (var_gammapd * var_gammapd)), ((((var_vgfbpd_dn4 - var_t2_dn4) * var_gammapd) - (assign59480_e96560 * var_gammapd_dn4)) / (var_gammapd * var_gammapd)), ((((var_vgfbpd_dn5 - var_t2_dn5) * var_gammapd) - (assign59480_e96560 * var_gammapd_dn5)) / (var_gammapd * var_gammapd)), ((((var_vgfbpd_dn6 - var_t2_dn6) * var_gammapd) - (assign59480_e96560 * var_gammapd_dn6)) / (var_gammapd * var_gammapd)), ((((var_vgfbpd_dn7 - var_t2_dn7) * var_gammapd) - (assign59480_e96560 * var_gammapd_dn7)) / (var_gammapd * var_gammapd)), ((((var_vgfbpd_dn8 - var_t2_dn8) * var_gammapd) - (assign59480_e96560 * var_gammapd_dn8)) / (var_gammapd * var_gammapd)), ((((var_vgfbpd_dn9 - var_t2_dn9) * var_gammapd) - (assign59480_e96560 * var_gammapd_dn9)) / (var_gammapd * var_gammapd)), ((((var_vgfbpd_dn10 - var_t2_dn10) * var_gammapd) - (assign59480_e96560 * var_gammapd_dn10)) / (var_gammapd * var_gammapd)), ((((var_vgfbpd_dn11 - var_t2_dn11) * var_gammapd) - (assign59480_e96560 * var_gammapd_dn11)) / (var_gammapd * var_gammapd)),)
    } else {
        (var_t3, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11,)
    }
};
        var_t3 = assign59480_e96564;
        var_t3_dn3 = assign59480_e96564_d_n3;
        var_t3_dn4 = assign59480_e96564_d_n4;
        var_t3_dn5 = assign59480_e96564_d_n5;
        var_t3_dn6 = assign59480_e96564_d_n6;
        var_t3_dn7 = assign59480_e96564_d_n7;
        var_t3_dn8 = assign59480_e96564_d_n8;
        var_t3_dn9 = assign59480_e96564_d_n9;
        var_t3_dn10 = assign59480_e96564_d_n10;
        var_t3_dn11 = assign59480_e96564_d_n11;
        var_t3_rv = 0.0;

        let (assign59490_e96583, assign59490_e96583_d_n3, assign59490_e96583_d_n4, assign59490_e96583_d_n5, assign59490_e96583_d_n6, assign59490_e96583_d_n7, assign59490_e96583_d_n8, assign59490_e96583_d_n9, assign59490_e96583_d_n10, assign59490_e96583_d_n11,) = {
    if (((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard870 != 0.0)) {
        let assign59490_e96573: f64 = (1.0 - var_t2);
        let assign59490_e96576: f64 = (var_t3 * var_t3);
        let assign59490_e96577: f64 = (assign59490_e96573 + assign59490_e96576);
        let assign59490_e96579: f64 = (assign59490_e96577).max(1e-38);
        let assign59490_e96580: f64 = (assign59490_e96579).ln();
        let assign59490_e96581: f64 = (-assign59490_e96580);
        (assign59490_e96581, (-(if assign59490_e96577 >= 1e-38 { ((-var_t2_dn3) + ((var_t3_dn3 * var_t3) + (var_t3 * var_t3_dn3))) } else { 0.0 } / assign59490_e96579)), (-(if assign59490_e96577 >= 1e-38 { ((-var_t2_dn4) + ((var_t3_dn4 * var_t3) + (var_t3 * var_t3_dn4))) } else { 0.0 } / assign59490_e96579)), (-(if assign59490_e96577 >= 1e-38 { ((-var_t2_dn5) + ((var_t3_dn5 * var_t3) + (var_t3 * var_t3_dn5))) } else { 0.0 } / assign59490_e96579)), (-(if assign59490_e96577 >= 1e-38 { ((-var_t2_dn6) + ((var_t3_dn6 * var_t3) + (var_t3 * var_t3_dn6))) } else { 0.0 } / assign59490_e96579)), (-(if assign59490_e96577 >= 1e-38 { ((-var_t2_dn7) + ((var_t3_dn7 * var_t3) + (var_t3 * var_t3_dn7))) } else { 0.0 } / assign59490_e96579)), (-(if assign59490_e96577 >= 1e-38 { ((-var_t2_dn8) + ((var_t3_dn8 * var_t3) + (var_t3 * var_t3_dn8))) } else { 0.0 } / assign59490_e96579)), (-(if assign59490_e96577 >= 1e-38 { ((-var_t2_dn9) + ((var_t3_dn9 * var_t3) + (var_t3 * var_t3_dn9))) } else { 0.0 } / assign59490_e96579)), (-(if assign59490_e96577 >= 1e-38 { ((-var_t2_dn10) + ((var_t3_dn10 * var_t3) + (var_t3 * var_t3_dn10))) } else { 0.0 } / assign59490_e96579)), (-(if assign59490_e96577 >= 1e-38 { ((-var_t2_dn11) + ((var_t3_dn11 * var_t3) + (var_t3 * var_t3_dn11))) } else { 0.0 } / assign59490_e96579)),)
    } else {
        (var_psip, var_psip_dn3, var_psip_dn4, var_psip_dn5, var_psip_dn6, var_psip_dn7, var_psip_dn8, var_psip_dn9, var_psip_dn10, var_psip_dn11,)
    }
};
        var_psip = assign59490_e96583;
        var_psip_dn3 = assign59490_e96583_d_n3;
        var_psip_dn4 = assign59490_e96583_d_n4;
        var_psip_dn5 = assign59490_e96583_d_n5;
        var_psip_dn6 = assign59490_e96583_d_n6;
        var_psip_dn7 = assign59490_e96583_d_n7;
        var_psip_dn8 = assign59490_e96583_d_n8;
        var_psip_dn9 = assign59490_e96583_d_n9;
        var_psip_dn10 = assign59490_e96583_d_n10;
        var_psip_dn11 = assign59490_e96583_d_n11;
        var_psip_rv = 0.0;

        let (assign59500_e96595, assign59500_e96595_d_n3, assign59500_e96595_d_n4, assign59500_e96595_d_n5, assign59500_e96595_d_n6, assign59500_e96595_d_n7, assign59500_e96595_d_n8, assign59500_e96595_d_n9, assign59500_e96595_d_n10, assign59500_e96595_d_n11,) = {
    if (((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard870 == 0.0)) {
        let assign59500_e96592: f64 = (-var_t2);
        let assign59500_e96593: f64 = { let limited_exp_arg = assign59500_e96592; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign59500_e96593, ({ let limited_exp_arg = assign59500_e96592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_dn3)), ({ let limited_exp_arg = assign59500_e96592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_dn4)), ({ let limited_exp_arg = assign59500_e96592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_dn5)), ({ let limited_exp_arg = assign59500_e96592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_dn6)), ({ let limited_exp_arg = assign59500_e96592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_dn7)), ({ let limited_exp_arg = assign59500_e96592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_dn8)), ({ let limited_exp_arg = assign59500_e96592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_dn9)), ({ let limited_exp_arg = assign59500_e96592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_dn10)), ({ let limited_exp_arg = assign59500_e96592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-var_t2_dn11)),)
    } else {
        (var_t3, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11,)
    }
};
        var_t3 = assign59500_e96595;
        var_t3_dn3 = assign59500_e96595_d_n3;
        var_t3_dn4 = assign59500_e96595_d_n4;
        var_t3_dn5 = assign59500_e96595_d_n5;
        var_t3_dn6 = assign59500_e96595_d_n6;
        var_t3_dn7 = assign59500_e96595_d_n7;
        var_t3_dn8 = assign59500_e96595_d_n8;
        var_t3_dn9 = assign59500_e96595_d_n9;
        var_t3_dn10 = assign59500_e96595_d_n10;
        var_t3_dn11 = assign59500_e96595_d_n11;
        var_t3_rv = 0.0;

        let (assign59510_e96607, assign59510_e96607_d_n3, assign59510_e96607_d_n4, assign59510_e96607_d_n5, assign59510_e96607_d_n6, assign59510_e96607_d_n7, assign59510_e96607_d_n8, assign59510_e96607_d_n9, assign59510_e96607_d_n10, assign59510_e96607_d_n11,) = {
    if (((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard870 == 0.0)) {
        let assign59510_e96605: f64 = (0.5 * var_gammapd);
        (assign59510_e96605, (0.5 * var_gammapd_dn3), (0.5 * var_gammapd_dn4), (0.5 * var_gammapd_dn5), (0.5 * var_gammapd_dn6), (0.5 * var_gammapd_dn7), (0.5 * var_gammapd_dn8), (0.5 * var_gammapd_dn9), (0.5 * var_gammapd_dn10), (0.5 * var_gammapd_dn11),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11,)
    }
};
        var_t1 = assign59510_e96607;
        var_t1_dn3 = assign59510_e96607_d_n3;
        var_t1_dn4 = assign59510_e96607_d_n4;
        var_t1_dn5 = assign59510_e96607_d_n5;
        var_t1_dn6 = assign59510_e96607_d_n6;
        var_t1_dn7 = assign59510_e96607_d_n7;
        var_t1_dn8 = assign59510_e96607_d_n8;
        var_t1_dn9 = assign59510_e96607_d_n9;
        var_t1_dn10 = assign59510_e96607_d_n10;
        var_t1_dn11 = assign59510_e96607_d_n11;
        var_t1_rv = 0.0;

        let (assign59520_e96628, assign59520_e96628_d_n3, assign59520_e96628_d_n4, assign59520_e96628_d_n5, assign59520_e96628_d_n6, assign59520_e96628_d_n7, assign59520_e96628_d_n8, assign59520_e96628_d_n9, assign59520_e96628_d_n10, assign59520_e96628_d_n11,) = {
    if (((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard870 == 0.0)) {
        let assign59520_e96617: f64 = (var_vgfbpd - 1.0);
        let assign59520_e96619: f64 = (assign59520_e96617 + var_t3);
        let assign59520_e96622: f64 = (var_t1 * var_t1);
        let assign59520_e96623: f64 = (assign59520_e96619 + assign59520_e96622);
        let assign59520_e96624: f64 = (assign59520_e96623).sqrt();
        let assign59520_e96626: f64 = (assign59520_e96624 - var_t1);
        (assign59520_e96626, ((((var_vgfbpd_dn3 + var_t3_dn3) + ((var_t1_dn3 * var_t1) + (var_t1 * var_t1_dn3))) / (2.0 * assign59520_e96624)) - var_t1_dn3), ((((var_vgfbpd_dn4 + var_t3_dn4) + ((var_t1_dn4 * var_t1) + (var_t1 * var_t1_dn4))) / (2.0 * assign59520_e96624)) - var_t1_dn4), ((((var_vgfbpd_dn5 + var_t3_dn5) + ((var_t1_dn5 * var_t1) + (var_t1 * var_t1_dn5))) / (2.0 * assign59520_e96624)) - var_t1_dn5), ((((var_vgfbpd_dn6 + var_t3_dn6) + ((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6))) / (2.0 * assign59520_e96624)) - var_t1_dn6), ((((var_vgfbpd_dn7 + var_t3_dn7) + ((var_t1_dn7 * var_t1) + (var_t1 * var_t1_dn7))) / (2.0 * assign59520_e96624)) - var_t1_dn7), ((((var_vgfbpd_dn8 + var_t3_dn8) + ((var_t1_dn8 * var_t1) + (var_t1 * var_t1_dn8))) / (2.0 * assign59520_e96624)) - var_t1_dn8), ((((var_vgfbpd_dn9 + var_t3_dn9) + ((var_t1_dn9 * var_t1) + (var_t1 * var_t1_dn9))) / (2.0 * assign59520_e96624)) - var_t1_dn9), ((((var_vgfbpd_dn10 + var_t3_dn10) + ((var_t1_dn10 * var_t1) + (var_t1 * var_t1_dn10))) / (2.0 * assign59520_e96624)) - var_t1_dn10), ((((var_vgfbpd_dn11 + var_t3_dn11) + ((var_t1_dn11 * var_t1) + (var_t1 * var_t1_dn11))) / (2.0 * assign59520_e96624)) - var_t1_dn11),)
    } else {
        (var_t2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11,)
    }
};
        var_t2 = assign59520_e96628;
        var_t2_dn3 = assign59520_e96628_d_n3;
        var_t2_dn4 = assign59520_e96628_d_n4;
        var_t2_dn5 = assign59520_e96628_d_n5;
        var_t2_dn6 = assign59520_e96628_d_n6;
        var_t2_dn7 = assign59520_e96628_d_n7;
        var_t2_dn8 = assign59520_e96628_d_n8;
        var_t2_dn9 = assign59520_e96628_d_n9;
        var_t2_dn10 = assign59520_e96628_d_n10;
        var_t2_dn11 = assign59520_e96628_d_n11;
        var_t2_rv = 0.0;

        let (assign59530_e96644, assign59530_e96644_d_n3, assign59530_e96644_d_n4, assign59530_e96644_d_n5, assign59530_e96644_d_n6, assign59530_e96644_d_n7, assign59530_e96644_d_n8, assign59530_e96644_d_n9, assign59530_e96644_d_n10, assign59530_e96644_d_n11,) = {
    if (((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard870 == 0.0)) {
        let assign59530_e96638: f64 = (var_t2 * var_t2);
        let assign59530_e96640: f64 = (assign59530_e96638 + 1.0);
        let assign59530_e96642: f64 = (assign59530_e96640 - var_t3);
        (assign59530_e96642, (((var_t2_dn3 * var_t2) + (var_t2 * var_t2_dn3)) - var_t3_dn3), (((var_t2_dn4 * var_t2) + (var_t2 * var_t2_dn4)) - var_t3_dn4), (((var_t2_dn5 * var_t2) + (var_t2 * var_t2_dn5)) - var_t3_dn5), (((var_t2_dn6 * var_t2) + (var_t2 * var_t2_dn6)) - var_t3_dn6), (((var_t2_dn7 * var_t2) + (var_t2 * var_t2_dn7)) - var_t3_dn7), (((var_t2_dn8 * var_t2) + (var_t2 * var_t2_dn8)) - var_t3_dn8), (((var_t2_dn9 * var_t2) + (var_t2 * var_t2_dn9)) - var_t3_dn9), (((var_t2_dn10 * var_t2) + (var_t2 * var_t2_dn10)) - var_t3_dn10), (((var_t2_dn11 * var_t2) + (var_t2 * var_t2_dn11)) - var_t3_dn11),)
    } else {
        (var_psip, var_psip_dn3, var_psip_dn4, var_psip_dn5, var_psip_dn6, var_psip_dn7, var_psip_dn8, var_psip_dn9, var_psip_dn10, var_psip_dn11,)
    }
};
        var_psip = assign59530_e96644;
        var_psip_dn3 = assign59530_e96644_d_n3;
        var_psip_dn4 = assign59530_e96644_d_n4;
        var_psip_dn5 = assign59530_e96644_d_n5;
        var_psip_dn6 = assign59530_e96644_d_n6;
        var_psip_dn7 = assign59530_e96644_d_n7;
        var_psip_dn8 = assign59530_e96644_d_n8;
        var_psip_dn9 = assign59530_e96644_d_n9;
        var_psip_dn10 = assign59530_e96644_d_n10;
        var_psip_dn11 = assign59530_e96644_d_n11;
        var_psip_rv = 0.0;

        *var_dgammaedge_i_slot = var_dgammaedge_i;
        *var_dgammaedge_i_rv_slot = var_dgammaedge_i_rv;
        *var_dvth_sce_slot = var_dvth_sce;
        *var_dvth_sce_dn10_slot = var_dvth_sce_dn10;
        *var_dvth_sce_dn11_slot = var_dvth_sce_dn11;
        *var_dvth_sce_dn3_slot = var_dvth_sce_dn3;
        *var_dvth_sce_dn4_slot = var_dvth_sce_dn4;
        *var_dvth_sce_dn5_slot = var_dvth_sce_dn5;
        *var_dvth_sce_dn6_slot = var_dvth_sce_dn6;
        *var_dvth_sce_dn7_slot = var_dvth_sce_dn7;
        *var_dvth_sce_dn8_slot = var_dvth_sce_dn8;
        *var_dvth_sce_dn9_slot = var_dvth_sce_dn9;
        *var_dvth_sce_rv_slot = var_dvth_sce_rv;
        *var_gam_edge_slot = var_gam_edge;
        *var_gam_edge_dn10_slot = var_gam_edge_dn10;
        *var_gam_edge_dn11_slot = var_gam_edge_dn11;
        *var_gam_edge_dn3_slot = var_gam_edge_dn3;
        *var_gam_edge_dn4_slot = var_gam_edge_dn4;
        *var_gam_edge_dn5_slot = var_gam_edge_dn5;
        *var_gam_edge_dn6_slot = var_gam_edge_dn6;
        *var_gam_edge_dn7_slot = var_gam_edge_dn7;
        *var_gam_edge_dn8_slot = var_gam_edge_dn8;
        *var_gam_edge_dn9_slot = var_gam_edge_dn9;
        *var_gam_edge_rv_slot = var_gam_edge_rv;
        *var_gammapd_slot = var_gammapd;
        *var_gammapd_dn10_slot = var_gammapd_dn10;
        *var_gammapd_dn11_slot = var_gammapd_dn11;
        *var_gammapd_dn3_slot = var_gammapd_dn3;
        *var_gammapd_dn4_slot = var_gammapd_dn4;
        *var_gammapd_dn5_slot = var_gammapd_dn5;
        *var_gammapd_dn6_slot = var_gammapd_dn6;
        *var_gammapd_dn7_slot = var_gammapd_dn7;
        *var_gammapd_dn8_slot = var_gammapd_dn8;
        *var_gammapd_dn9_slot = var_gammapd_dn9;
        *var_gammapd_rv_slot = var_gammapd_rv;
        *var_guard869_slot = var_guard869;
        *var_guard869_rv_slot = var_guard869_rv;
        *var_guard870_slot = var_guard870;
        *var_guard870_rv_slot = var_guard870_rv;
        *var_phib_n_edge_slot = var_phib_n_edge;
        *var_phib_n_edge_dn10_slot = var_phib_n_edge_dn10;
        *var_phib_n_edge_dn11_slot = var_phib_n_edge_dn11;
        *var_phib_n_edge_dn3_slot = var_phib_n_edge_dn3;
        *var_phib_n_edge_dn4_slot = var_phib_n_edge_dn4;
        *var_phib_n_edge_dn5_slot = var_phib_n_edge_dn5;
        *var_phib_n_edge_dn6_slot = var_phib_n_edge_dn6;
        *var_phib_n_edge_dn7_slot = var_phib_n_edge_dn7;
        *var_phib_n_edge_dn8_slot = var_phib_n_edge_dn8;
        *var_phib_n_edge_dn9_slot = var_phib_n_edge_dn9;
        *var_phib_n_edge_rv_slot = var_phib_n_edge_rv;
        *var_psip_slot = var_psip;
        *var_psip_dn10_slot = var_psip_dn10;
        *var_psip_dn11_slot = var_psip_dn11;
        *var_psip_dn3_slot = var_psip_dn3;
        *var_psip_dn4_slot = var_psip_dn4;
        *var_psip_dn5_slot = var_psip_dn5;
        *var_psip_dn6_slot = var_psip_dn6;
        *var_psip_dn7_slot = var_psip_dn7;
        *var_psip_dn8_slot = var_psip_dn8;
        *var_psip_dn9_slot = var_psip_dn9;
        *var_psip_rv_slot = var_psip_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_t0_rv_slot = var_t0_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t1_rv_slot = var_t1_rv;
        *var_t2_slot = var_t2;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_t2_rv_slot = var_t2_rv;
        *var_t3_slot = var_t3;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn3_slot = var_t3_dn3;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_dn9_slot = var_t3_dn9;
        *var_t3_rv_slot = var_t3_rv;
        *var_theta_sce_edge_slot = var_theta_sce_edge;
        *var_theta_sce_edge_dn10_slot = var_theta_sce_edge_dn10;
        *var_theta_sce_edge_dn11_slot = var_theta_sce_edge_dn11;
        *var_theta_sce_edge_dn3_slot = var_theta_sce_edge_dn3;
        *var_theta_sce_edge_dn4_slot = var_theta_sce_edge_dn4;
        *var_theta_sce_edge_dn5_slot = var_theta_sce_edge_dn5;
        *var_theta_sce_edge_dn6_slot = var_theta_sce_edge_dn6;
        *var_theta_sce_edge_dn7_slot = var_theta_sce_edge_dn7;
        *var_theta_sce_edge_dn8_slot = var_theta_sce_edge_dn8;
        *var_theta_sce_edge_dn9_slot = var_theta_sce_edge_dn9;
        *var_theta_sce_edge_rv_slot = var_theta_sce_edge_rv;
        *var_vgfb_slot = var_vgfb;
        *var_vgfb_dn10_slot = var_vgfb_dn10;
        *var_vgfb_dn11_slot = var_vgfb_dn11;
        *var_vgfb_dn3_slot = var_vgfb_dn3;
        *var_vgfb_dn4_slot = var_vgfb_dn4;
        *var_vgfb_dn5_slot = var_vgfb_dn5;
        *var_vgfb_dn6_slot = var_vgfb_dn6;
        *var_vgfb_dn7_slot = var_vgfb_dn7;
        *var_vgfb_dn8_slot = var_vgfb_dn8;
        *var_vgfb_dn9_slot = var_vgfb_dn9;
        *var_vgfb_rv_slot = var_vgfb_rv;
        *var_vgfbpd_slot = var_vgfbpd;
        *var_vgfbpd_dn10_slot = var_vgfbpd_dn10;
        *var_vgfbpd_dn11_slot = var_vgfbpd_dn11;
        *var_vgfbpd_dn3_slot = var_vgfbpd_dn3;
        *var_vgfbpd_dn4_slot = var_vgfbpd_dn4;
        *var_vgfbpd_dn5_slot = var_vgfbpd_dn5;
        *var_vgfbpd_dn6_slot = var_vgfbpd_dn6;
        *var_vgfbpd_dn7_slot = var_vgfbpd_dn7;
        *var_vgfbpd_dn8_slot = var_vgfbpd_dn8;
        *var_vgfbpd_dn9_slot = var_vgfbpd_dn9;
        *var_vgfbpd_rv_slot = var_vgfbpd_rv;
        *var_vth_shift_slot = var_vth_shift;
        *var_vth_shift_dn10_slot = var_vth_shift_dn10;
        *var_vth_shift_dn11_slot = var_vth_shift_dn11;
        *var_vth_shift_dn3_slot = var_vth_shift_dn3;
        *var_vth_shift_dn4_slot = var_vth_shift_dn4;
        *var_vth_shift_dn5_slot = var_vth_shift_dn5;
        *var_vth_shift_dn6_slot = var_vth_shift_dn6;
        *var_vth_shift_dn7_slot = var_vth_shift_dn7;
        *var_vth_shift_dn8_slot = var_vth_shift_dn8;
        *var_vth_shift_dn9_slot = var_vth_shift_dn9;
        *var_vth_shift_rv_slot = var_vth_shift_rv;
    }

    pub(super) fn stamp_reactive_block_208(
        var_gam_edge: f64,
        var_gam_edge_dn10: f64,
        var_gam_edge_dn11: f64,
        var_gam_edge_dn3: f64,
        var_gam_edge_dn4: f64,
        var_gam_edge_dn5: f64,
        var_gam_edge_dn6: f64,
        var_gam_edge_dn7: f64,
        var_gam_edge_dn8: f64,
        var_gam_edge_dn9: f64,
        var_guard492: f64,
        var_guard867: f64,
        var_phib_n_edge: f64,
        var_phib_n_edge_dn10: f64,
        var_phib_n_edge_dn11: f64,
        var_phib_n_edge_dn3: f64,
        var_phib_n_edge_dn4: f64,
        var_phib_n_edge_dn5: f64,
        var_phib_n_edge_dn6: f64,
        var_phib_n_edge_dn7: f64,
        var_phib_n_edge_dn8: f64,
        var_phib_n_edge_dn9: f64,
        var_psip: f64,
        var_psip_dn10: f64,
        var_psip_dn11: f64,
        var_psip_dn3: f64,
        var_psip_dn4: f64,
        var_psip_dn5: f64,
        var_psip_dn6: f64,
        var_psip_dn7: f64,
        var_psip_dn8: f64,
        var_psip_dn9: f64,
        var_vs_1: f64,
        var_vs_1_dn10: f64,
        var_vs_1_dn11: f64,
        var_vs_1_dn3: f64,
        var_vs_1_dn4: f64,
        var_vs_1_dn5: f64,
        var_vs_1_dn6: f64,
        var_vs_1_dn7: f64,
        var_vs_1_dn8: f64,
        var_vs_1_dn9: f64,
        var_guard871_slot: &mut f64,
        var_guard871_rv_slot: &mut f64,
        var_guard872_slot: &mut f64,
        var_guard872_rv_slot: &mut f64,
        var_guard873_slot: &mut f64,
        var_guard873_rv_slot: &mut f64,
        var_qs_edge_slot: &mut f64,
        var_qs_edge_dn10_slot: &mut f64,
        var_qs_edge_dn11_slot: &mut f64,
        var_qs_edge_dn3_slot: &mut f64,
        var_qs_edge_dn4_slot: &mut f64,
        var_qs_edge_dn5_slot: &mut f64,
        var_qs_edge_dn6_slot: &mut f64,
        var_qs_edge_dn7_slot: &mut f64,
        var_qs_edge_dn8_slot: &mut f64,
        var_qs_edge_dn9_slot: &mut f64,
        var_qs_edge_rv_slot: &mut f64,
        var_sqrtpsip_slot: &mut f64,
        var_sqrtpsip_dn10_slot: &mut f64,
        var_sqrtpsip_dn11_slot: &mut f64,
        var_sqrtpsip_dn3_slot: &mut f64,
        var_sqrtpsip_dn4_slot: &mut f64,
        var_sqrtpsip_dn5_slot: &mut f64,
        var_sqrtpsip_dn6_slot: &mut f64,
        var_sqrtpsip_dn7_slot: &mut f64,
        var_sqrtpsip_dn8_slot: &mut f64,
        var_sqrtpsip_dn9_slot: &mut f64,
        var_sqrtpsip_rv_slot: &mut f64,
        var_sqrtpsisa_slot: &mut f64,
        var_sqrtpsisa_dn10_slot: &mut f64,
        var_sqrtpsisa_dn11_slot: &mut f64,
        var_sqrtpsisa_dn3_slot: &mut f64,
        var_sqrtpsisa_dn4_slot: &mut f64,
        var_sqrtpsisa_dn5_slot: &mut f64,
        var_sqrtpsisa_dn6_slot: &mut f64,
        var_sqrtpsisa_dn7_slot: &mut f64,
        var_sqrtpsisa_dn8_slot: &mut f64,
        var_sqrtpsisa_dn9_slot: &mut f64,
        var_sqrtpsisa_rv_slot: &mut f64,
        var_sqrtpsisainv_slot: &mut f64,
        var_sqrtpsisainv_dn10_slot: &mut f64,
        var_sqrtpsisainv_dn11_slot: &mut f64,
        var_sqrtpsisainv_dn3_slot: &mut f64,
        var_sqrtpsisainv_dn4_slot: &mut f64,
        var_sqrtpsisainv_dn5_slot: &mut f64,
        var_sqrtpsisainv_dn6_slot: &mut f64,
        var_sqrtpsisainv_dn7_slot: &mut f64,
        var_sqrtpsisainv_dn8_slot: &mut f64,
        var_sqrtpsisainv_dn9_slot: &mut f64,
        var_sqrtpsisainv_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_dn9_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn10_slot: &mut f64,
        var_t4_dn11_slot: &mut f64,
        var_t4_dn3_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t4_dn9_slot: &mut f64,
        var_t4_rv_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_dn10_slot: &mut f64,
        var_t5_dn11_slot: &mut f64,
        var_t5_dn3_slot: &mut f64,
        var_t5_dn4_slot: &mut f64,
        var_t5_dn5_slot: &mut f64,
        var_t5_dn6_slot: &mut f64,
        var_t5_dn7_slot: &mut f64,
        var_t5_dn8_slot: &mut f64,
        var_t5_dn9_slot: &mut f64,
        var_t5_rv_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_dn10_slot: &mut f64,
        var_t6_dn11_slot: &mut f64,
        var_t6_dn3_slot: &mut f64,
        var_t6_dn4_slot: &mut f64,
        var_t6_dn5_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn7_slot: &mut f64,
        var_t6_dn8_slot: &mut f64,
        var_t6_dn9_slot: &mut f64,
        var_t6_rv_slot: &mut f64,
        var_t8_slot: &mut f64,
        var_t8_dn10_slot: &mut f64,
        var_t8_dn11_slot: &mut f64,
        var_t8_dn3_slot: &mut f64,
        var_t8_dn4_slot: &mut f64,
        var_t8_dn5_slot: &mut f64,
        var_t8_dn6_slot: &mut f64,
        var_t8_dn7_slot: &mut f64,
        var_t8_dn8_slot: &mut f64,
        var_t8_dn9_slot: &mut f64,
        var_t8_rv_slot: &mut f64,
    ) {
        let mut var_guard871: f64 = *var_guard871_slot;
        let mut var_guard871_rv: f64 = *var_guard871_rv_slot;
        let mut var_guard872: f64 = *var_guard872_slot;
        let mut var_guard872_rv: f64 = *var_guard872_rv_slot;
        let mut var_guard873: f64 = *var_guard873_slot;
        let mut var_guard873_rv: f64 = *var_guard873_rv_slot;
        let mut var_qs_edge: f64 = *var_qs_edge_slot;
        let mut var_qs_edge_dn10: f64 = *var_qs_edge_dn10_slot;
        let mut var_qs_edge_dn11: f64 = *var_qs_edge_dn11_slot;
        let mut var_qs_edge_dn3: f64 = *var_qs_edge_dn3_slot;
        let mut var_qs_edge_dn4: f64 = *var_qs_edge_dn4_slot;
        let mut var_qs_edge_dn5: f64 = *var_qs_edge_dn5_slot;
        let mut var_qs_edge_dn6: f64 = *var_qs_edge_dn6_slot;
        let mut var_qs_edge_dn7: f64 = *var_qs_edge_dn7_slot;
        let mut var_qs_edge_dn8: f64 = *var_qs_edge_dn8_slot;
        let mut var_qs_edge_dn9: f64 = *var_qs_edge_dn9_slot;
        let mut var_qs_edge_rv: f64 = *var_qs_edge_rv_slot;
        let mut var_sqrtpsip: f64 = *var_sqrtpsip_slot;
        let mut var_sqrtpsip_dn10: f64 = *var_sqrtpsip_dn10_slot;
        let mut var_sqrtpsip_dn11: f64 = *var_sqrtpsip_dn11_slot;
        let mut var_sqrtpsip_dn3: f64 = *var_sqrtpsip_dn3_slot;
        let mut var_sqrtpsip_dn4: f64 = *var_sqrtpsip_dn4_slot;
        let mut var_sqrtpsip_dn5: f64 = *var_sqrtpsip_dn5_slot;
        let mut var_sqrtpsip_dn6: f64 = *var_sqrtpsip_dn6_slot;
        let mut var_sqrtpsip_dn7: f64 = *var_sqrtpsip_dn7_slot;
        let mut var_sqrtpsip_dn8: f64 = *var_sqrtpsip_dn8_slot;
        let mut var_sqrtpsip_dn9: f64 = *var_sqrtpsip_dn9_slot;
        let mut var_sqrtpsip_rv: f64 = *var_sqrtpsip_rv_slot;
        let mut var_sqrtpsisa: f64 = *var_sqrtpsisa_slot;
        let mut var_sqrtpsisa_dn10: f64 = *var_sqrtpsisa_dn10_slot;
        let mut var_sqrtpsisa_dn11: f64 = *var_sqrtpsisa_dn11_slot;
        let mut var_sqrtpsisa_dn3: f64 = *var_sqrtpsisa_dn3_slot;
        let mut var_sqrtpsisa_dn4: f64 = *var_sqrtpsisa_dn4_slot;
        let mut var_sqrtpsisa_dn5: f64 = *var_sqrtpsisa_dn5_slot;
        let mut var_sqrtpsisa_dn6: f64 = *var_sqrtpsisa_dn6_slot;
        let mut var_sqrtpsisa_dn7: f64 = *var_sqrtpsisa_dn7_slot;
        let mut var_sqrtpsisa_dn8: f64 = *var_sqrtpsisa_dn8_slot;
        let mut var_sqrtpsisa_dn9: f64 = *var_sqrtpsisa_dn9_slot;
        let mut var_sqrtpsisa_rv: f64 = *var_sqrtpsisa_rv_slot;
        let mut var_sqrtpsisainv: f64 = *var_sqrtpsisainv_slot;
        let mut var_sqrtpsisainv_dn10: f64 = *var_sqrtpsisainv_dn10_slot;
        let mut var_sqrtpsisainv_dn11: f64 = *var_sqrtpsisainv_dn11_slot;
        let mut var_sqrtpsisainv_dn3: f64 = *var_sqrtpsisainv_dn3_slot;
        let mut var_sqrtpsisainv_dn4: f64 = *var_sqrtpsisainv_dn4_slot;
        let mut var_sqrtpsisainv_dn5: f64 = *var_sqrtpsisainv_dn5_slot;
        let mut var_sqrtpsisainv_dn6: f64 = *var_sqrtpsisainv_dn6_slot;
        let mut var_sqrtpsisainv_dn7: f64 = *var_sqrtpsisainv_dn7_slot;
        let mut var_sqrtpsisainv_dn8: f64 = *var_sqrtpsisainv_dn8_slot;
        let mut var_sqrtpsisainv_dn9: f64 = *var_sqrtpsisainv_dn9_slot;
        let mut var_sqrtpsisainv_rv: f64 = *var_sqrtpsisainv_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_dn9: f64 = *var_t3_dn9_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn10: f64 = *var_t4_dn10_slot;
        let mut var_t4_dn11: f64 = *var_t4_dn11_slot;
        let mut var_t4_dn3: f64 = *var_t4_dn3_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t4_dn9: f64 = *var_t4_dn9_slot;
        let mut var_t4_rv: f64 = *var_t4_rv_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_dn10: f64 = *var_t5_dn10_slot;
        let mut var_t5_dn11: f64 = *var_t5_dn11_slot;
        let mut var_t5_dn3: f64 = *var_t5_dn3_slot;
        let mut var_t5_dn4: f64 = *var_t5_dn4_slot;
        let mut var_t5_dn5: f64 = *var_t5_dn5_slot;
        let mut var_t5_dn6: f64 = *var_t5_dn6_slot;
        let mut var_t5_dn7: f64 = *var_t5_dn7_slot;
        let mut var_t5_dn8: f64 = *var_t5_dn8_slot;
        let mut var_t5_dn9: f64 = *var_t5_dn9_slot;
        let mut var_t5_rv: f64 = *var_t5_rv_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_dn10: f64 = *var_t6_dn10_slot;
        let mut var_t6_dn11: f64 = *var_t6_dn11_slot;
        let mut var_t6_dn3: f64 = *var_t6_dn3_slot;
        let mut var_t6_dn4: f64 = *var_t6_dn4_slot;
        let mut var_t6_dn5: f64 = *var_t6_dn5_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn7: f64 = *var_t6_dn7_slot;
        let mut var_t6_dn8: f64 = *var_t6_dn8_slot;
        let mut var_t6_dn9: f64 = *var_t6_dn9_slot;
        let mut var_t6_rv: f64 = *var_t6_rv_slot;
        let mut var_t8: f64 = *var_t8_slot;
        let mut var_t8_dn10: f64 = *var_t8_dn10_slot;
        let mut var_t8_dn11: f64 = *var_t8_dn11_slot;
        let mut var_t8_dn3: f64 = *var_t8_dn3_slot;
        let mut var_t8_dn4: f64 = *var_t8_dn4_slot;
        let mut var_t8_dn5: f64 = *var_t8_dn5_slot;
        let mut var_t8_dn6: f64 = *var_t8_dn6_slot;
        let mut var_t8_dn7: f64 = *var_t8_dn7_slot;
        let mut var_t8_dn8: f64 = *var_t8_dn8_slot;
        let mut var_t8_dn9: f64 = *var_t8_dn9_slot;
        let mut var_t8_rv: f64 = *var_t8_rv_slot;

        let (assign59540_e96670, assign59540_e96670_d_n3, assign59540_e96670_d_n4, assign59540_e96670_d_n5, assign59540_e96670_d_n6, assign59540_e96670_d_n7, assign59540_e96670_d_n8, assign59540_e96670_d_n9, assign59540_e96670_d_n10, assign59540_e96670_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59540_e96652: f64 = (var_psip + 1.0);
        let assign59540_e96655: f64 = (var_psip - 1.0);
        let assign59540_e96658: f64 = (var_psip - 1.0);
        let assign59540_e96659: f64 = (assign59540_e96655 * assign59540_e96658);
        let assign59540_e96662: f64 = (0.25 * 2.0);
        let assign59540_e96664: f64 = (assign59540_e96662 * 2.0);
        let assign59540_e96665: f64 = (assign59540_e96659 + assign59540_e96664);
        let assign59540_e96666: f64 = (assign59540_e96665).sqrt();
        let assign59540_e96667: f64 = (assign59540_e96652 + assign59540_e96666);
        let assign59540_e96668: f64 = (0.5 * assign59540_e96667);
        (assign59540_e96668, (0.5 * (var_psip_dn3 + (((var_psip_dn3 * assign59540_e96658) + (assign59540_e96655 * var_psip_dn3)) / (2.0 * assign59540_e96666)))), (0.5 * (var_psip_dn4 + (((var_psip_dn4 * assign59540_e96658) + (assign59540_e96655 * var_psip_dn4)) / (2.0 * assign59540_e96666)))), (0.5 * (var_psip_dn5 + (((var_psip_dn5 * assign59540_e96658) + (assign59540_e96655 * var_psip_dn5)) / (2.0 * assign59540_e96666)))), (0.5 * (var_psip_dn6 + (((var_psip_dn6 * assign59540_e96658) + (assign59540_e96655 * var_psip_dn6)) / (2.0 * assign59540_e96666)))), (0.5 * (var_psip_dn7 + (((var_psip_dn7 * assign59540_e96658) + (assign59540_e96655 * var_psip_dn7)) / (2.0 * assign59540_e96666)))), (0.5 * (var_psip_dn8 + (((var_psip_dn8 * assign59540_e96658) + (assign59540_e96655 * var_psip_dn8)) / (2.0 * assign59540_e96666)))), (0.5 * (var_psip_dn9 + (((var_psip_dn9 * assign59540_e96658) + (assign59540_e96655 * var_psip_dn9)) / (2.0 * assign59540_e96666)))), (0.5 * (var_psip_dn10 + (((var_psip_dn10 * assign59540_e96658) + (assign59540_e96655 * var_psip_dn10)) / (2.0 * assign59540_e96666)))), (0.5 * (var_psip_dn11 + (((var_psip_dn11 * assign59540_e96658) + (assign59540_e96655 * var_psip_dn11)) / (2.0 * assign59540_e96666)))),)
    } else {
        (var_t8, var_t8_dn3, var_t8_dn4, var_t8_dn5, var_t8_dn6, var_t8_dn7, var_t8_dn8, var_t8_dn9, var_t8_dn10, var_t8_dn11,)
    }
};
        var_t8 = assign59540_e96670;
        var_t8_dn3 = assign59540_e96670_d_n3;
        var_t8_dn4 = assign59540_e96670_d_n4;
        var_t8_dn5 = assign59540_e96670_d_n5;
        var_t8_dn6 = assign59540_e96670_d_n6;
        var_t8_dn7 = assign59540_e96670_d_n7;
        var_t8_dn8 = assign59540_e96670_d_n8;
        var_t8_dn9 = assign59540_e96670_d_n9;
        var_t8_dn10 = assign59540_e96670_d_n10;
        var_t8_dn11 = assign59540_e96670_d_n11;
        var_t8_rv = 0.0;

        let (assign59550_e96678, assign59550_e96678_d_n3, assign59550_e96678_d_n4, assign59550_e96678_d_n5, assign59550_e96678_d_n6, assign59550_e96678_d_n7, assign59550_e96678_d_n8, assign59550_e96678_d_n9, assign59550_e96678_d_n10, assign59550_e96678_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59550_e96676: f64 = (var_t8).sqrt();
        (assign59550_e96676, (var_t8_dn3 / (2.0 * assign59550_e96676)), (var_t8_dn4 / (2.0 * assign59550_e96676)), (var_t8_dn5 / (2.0 * assign59550_e96676)), (var_t8_dn6 / (2.0 * assign59550_e96676)), (var_t8_dn7 / (2.0 * assign59550_e96676)), (var_t8_dn8 / (2.0 * assign59550_e96676)), (var_t8_dn9 / (2.0 * assign59550_e96676)), (var_t8_dn10 / (2.0 * assign59550_e96676)), (var_t8_dn11 / (2.0 * assign59550_e96676)),)
    } else {
        (var_sqrtpsip, var_sqrtpsip_dn3, var_sqrtpsip_dn4, var_sqrtpsip_dn5, var_sqrtpsip_dn6, var_sqrtpsip_dn7, var_sqrtpsip_dn8, var_sqrtpsip_dn9, var_sqrtpsip_dn10, var_sqrtpsip_dn11,)
    }
};
        var_sqrtpsip = assign59550_e96678;
        var_sqrtpsip_dn3 = assign59550_e96678_d_n3;
        var_sqrtpsip_dn4 = assign59550_e96678_d_n4;
        var_sqrtpsip_dn5 = assign59550_e96678_d_n5;
        var_sqrtpsip_dn6 = assign59550_e96678_d_n6;
        var_sqrtpsip_dn7 = assign59550_e96678_d_n7;
        var_sqrtpsip_dn8 = assign59550_e96678_d_n8;
        var_sqrtpsip_dn9 = assign59550_e96678_d_n9;
        var_sqrtpsip_dn10 = assign59550_e96678_d_n10;
        var_sqrtpsip_dn11 = assign59550_e96678_d_n11;
        var_sqrtpsip_rv = 0.0;

        let (assign59560_e96693, assign59560_e96693_d_n3, assign59560_e96693_d_n4, assign59560_e96693_d_n5, assign59560_e96693_d_n6, assign59560_e96693_d_n7, assign59560_e96693_d_n8, assign59560_e96693_d_n9, assign59560_e96693_d_n10, assign59560_e96693_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59560_e96687: f64 = (2.0 * var_sqrtpsip);
        let assign59560_e96688: f64 = (var_gam_edge / assign59560_e96687);
        let assign59560_e96689: f64 = (1.0 + assign59560_e96688);
        let assign59560_e96691: f64 = (assign59560_e96689 / var_gam_edge);
        (assign59560_e96691, ((((((var_gam_edge_dn3 * assign59560_e96687) - (var_gam_edge * (2.0 * var_sqrtpsip_dn3))) / (assign59560_e96687 * assign59560_e96687)) * var_gam_edge) - (assign59560_e96689 * var_gam_edge_dn3)) / (var_gam_edge * var_gam_edge)), ((((((var_gam_edge_dn4 * assign59560_e96687) - (var_gam_edge * (2.0 * var_sqrtpsip_dn4))) / (assign59560_e96687 * assign59560_e96687)) * var_gam_edge) - (assign59560_e96689 * var_gam_edge_dn4)) / (var_gam_edge * var_gam_edge)), ((((((var_gam_edge_dn5 * assign59560_e96687) - (var_gam_edge * (2.0 * var_sqrtpsip_dn5))) / (assign59560_e96687 * assign59560_e96687)) * var_gam_edge) - (assign59560_e96689 * var_gam_edge_dn5)) / (var_gam_edge * var_gam_edge)), ((((((var_gam_edge_dn6 * assign59560_e96687) - (var_gam_edge * (2.0 * var_sqrtpsip_dn6))) / (assign59560_e96687 * assign59560_e96687)) * var_gam_edge) - (assign59560_e96689 * var_gam_edge_dn6)) / (var_gam_edge * var_gam_edge)), ((((((var_gam_edge_dn7 * assign59560_e96687) - (var_gam_edge * (2.0 * var_sqrtpsip_dn7))) / (assign59560_e96687 * assign59560_e96687)) * var_gam_edge) - (assign59560_e96689 * var_gam_edge_dn7)) / (var_gam_edge * var_gam_edge)), ((((((var_gam_edge_dn8 * assign59560_e96687) - (var_gam_edge * (2.0 * var_sqrtpsip_dn8))) / (assign59560_e96687 * assign59560_e96687)) * var_gam_edge) - (assign59560_e96689 * var_gam_edge_dn8)) / (var_gam_edge * var_gam_edge)), ((((((var_gam_edge_dn9 * assign59560_e96687) - (var_gam_edge * (2.0 * var_sqrtpsip_dn9))) / (assign59560_e96687 * assign59560_e96687)) * var_gam_edge) - (assign59560_e96689 * var_gam_edge_dn9)) / (var_gam_edge * var_gam_edge)), ((((((var_gam_edge_dn10 * assign59560_e96687) - (var_gam_edge * (2.0 * var_sqrtpsip_dn10))) / (assign59560_e96687 * assign59560_e96687)) * var_gam_edge) - (assign59560_e96689 * var_gam_edge_dn10)) / (var_gam_edge * var_gam_edge)), ((((((var_gam_edge_dn11 * assign59560_e96687) - (var_gam_edge * (2.0 * var_sqrtpsip_dn11))) / (assign59560_e96687 * assign59560_e96687)) * var_gam_edge) - (assign59560_e96689 * var_gam_edge_dn11)) / (var_gam_edge * var_gam_edge)),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11,)
    }
};
        var_t0 = assign59560_e96693;
        var_t0_dn3 = assign59560_e96693_d_n3;
        var_t0_dn4 = assign59560_e96693_d_n4;
        var_t0_dn5 = assign59560_e96693_d_n5;
        var_t0_dn6 = assign59560_e96693_d_n6;
        var_t0_dn7 = assign59560_e96693_d_n7;
        var_t0_dn8 = assign59560_e96693_d_n8;
        var_t0_dn9 = assign59560_e96693_d_n9;
        var_t0_dn10 = assign59560_e96693_d_n10;
        var_t0_dn11 = assign59560_e96693_d_n11;
        var_t0_rv = 0.0;

        let (assign59570_e96706, assign59570_e96706_d_n3, assign59570_e96706_d_n4, assign59570_e96706_d_n5, assign59570_e96706_d_n6, assign59570_e96706_d_n7, assign59570_e96706_d_n8, assign59570_e96706_d_n9, assign59570_e96706_d_n10, assign59570_e96706_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59570_e96701: f64 = (2.0 * var_phib_n_edge);
        let assign59570_e96702: f64 = (var_psip - assign59570_e96701);
        let assign59570_e96704: f64 = (assign59570_e96702 - var_vs_1);
        (assign59570_e96704, ((var_psip_dn3 - (2.0 * var_phib_n_edge_dn3)) - var_vs_1_dn3), ((var_psip_dn4 - (2.0 * var_phib_n_edge_dn4)) - var_vs_1_dn4), ((var_psip_dn5 - (2.0 * var_phib_n_edge_dn5)) - var_vs_1_dn5), ((var_psip_dn6 - (2.0 * var_phib_n_edge_dn6)) - var_vs_1_dn6), ((var_psip_dn7 - (2.0 * var_phib_n_edge_dn7)) - var_vs_1_dn7), ((var_psip_dn8 - (2.0 * var_phib_n_edge_dn8)) - var_vs_1_dn8), ((var_psip_dn9 - (2.0 * var_phib_n_edge_dn9)) - var_vs_1_dn9), ((var_psip_dn10 - (2.0 * var_phib_n_edge_dn10)) - var_vs_1_dn10), ((var_psip_dn11 - (2.0 * var_phib_n_edge_dn11)) - var_vs_1_dn11),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11,)
    }
};
        var_t1 = assign59570_e96706;
        var_t1_dn3 = assign59570_e96706_d_n3;
        var_t1_dn4 = assign59570_e96706_d_n4;
        var_t1_dn5 = assign59570_e96706_d_n5;
        var_t1_dn6 = assign59570_e96706_d_n6;
        var_t1_dn7 = assign59570_e96706_d_n7;
        var_t1_dn8 = assign59570_e96706_d_n8;
        var_t1_dn9 = assign59570_e96706_d_n9;
        var_t1_dn10 = assign59570_e96706_d_n10;
        var_t1_dn11 = assign59570_e96706_d_n11;
        var_t1_rv = 0.0;

        let (assign59580_e96722, assign59580_e96722_d_n3, assign59580_e96722_d_n4, assign59580_e96722_d_n5, assign59580_e96722_d_n6, assign59580_e96722_d_n7, assign59580_e96722_d_n8, assign59580_e96722_d_n9, assign59580_e96722_d_n10, assign59580_e96722_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59580_e96714: f64 = (4.0 * var_t0);
        let assign59580_e96716: f64 = (assign59580_e96714 * var_sqrtpsip);
        let assign59580_e96718: f64 = (assign59580_e96716).max(1e-38);
        let assign59580_e96719: f64 = (assign59580_e96718).ln();
        let assign59580_e96720: f64 = (var_t1 - assign59580_e96719);
        (assign59580_e96720, (var_t1_dn3 - (if assign59580_e96716 >= 1e-38 { (((4.0 * var_t0_dn3) * var_sqrtpsip) + (assign59580_e96714 * var_sqrtpsip_dn3)) } else { 0.0 } / assign59580_e96718)), (var_t1_dn4 - (if assign59580_e96716 >= 1e-38 { (((4.0 * var_t0_dn4) * var_sqrtpsip) + (assign59580_e96714 * var_sqrtpsip_dn4)) } else { 0.0 } / assign59580_e96718)), (var_t1_dn5 - (if assign59580_e96716 >= 1e-38 { (((4.0 * var_t0_dn5) * var_sqrtpsip) + (assign59580_e96714 * var_sqrtpsip_dn5)) } else { 0.0 } / assign59580_e96718)), (var_t1_dn6 - (if assign59580_e96716 >= 1e-38 { (((4.0 * var_t0_dn6) * var_sqrtpsip) + (assign59580_e96714 * var_sqrtpsip_dn6)) } else { 0.0 } / assign59580_e96718)), (var_t1_dn7 - (if assign59580_e96716 >= 1e-38 { (((4.0 * var_t0_dn7) * var_sqrtpsip) + (assign59580_e96714 * var_sqrtpsip_dn7)) } else { 0.0 } / assign59580_e96718)), (var_t1_dn8 - (if assign59580_e96716 >= 1e-38 { (((4.0 * var_t0_dn8) * var_sqrtpsip) + (assign59580_e96714 * var_sqrtpsip_dn8)) } else { 0.0 } / assign59580_e96718)), (var_t1_dn9 - (if assign59580_e96716 >= 1e-38 { (((4.0 * var_t0_dn9) * var_sqrtpsip) + (assign59580_e96714 * var_sqrtpsip_dn9)) } else { 0.0 } / assign59580_e96718)), (var_t1_dn10 - (if assign59580_e96716 >= 1e-38 { (((4.0 * var_t0_dn10) * var_sqrtpsip) + (assign59580_e96714 * var_sqrtpsip_dn10)) } else { 0.0 } / assign59580_e96718)), (var_t1_dn11 - (if assign59580_e96716 >= 1e-38 { (((4.0 * var_t0_dn11) * var_sqrtpsip) + (assign59580_e96714 * var_sqrtpsip_dn11)) } else { 0.0 } / assign59580_e96718)),)
    } else {
        (var_t2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11,)
    }
};
        var_t2 = assign59580_e96722;
        var_t2_dn3 = assign59580_e96722_d_n3;
        var_t2_dn4 = assign59580_e96722_d_n4;
        var_t2_dn5 = assign59580_e96722_d_n5;
        var_t2_dn6 = assign59580_e96722_d_n6;
        var_t2_dn7 = assign59580_e96722_d_n7;
        var_t2_dn8 = assign59580_e96722_d_n8;
        var_t2_dn9 = assign59580_e96722_d_n9;
        var_t2_dn10 = assign59580_e96722_d_n10;
        var_t2_dn11 = assign59580_e96722_d_n11;
        var_t2_rv = 0.0;

        let (assign59590_e96742, assign59590_e96742_d_n3, assign59590_e96742_d_n4, assign59590_e96742_d_n5, assign59590_e96742_d_n6, assign59590_e96742_d_n7, assign59590_e96742_d_n8, assign59590_e96742_d_n9, assign59590_e96742_d_n10, assign59590_e96742_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59590_e96730: f64 = (var_t2 - 0.201491);
        let assign59590_e96734: f64 = (var_t2 + 0.402982);
        let assign59590_e96735: f64 = (var_t2 * assign59590_e96734);
        let assign59590_e96737: f64 = (assign59590_e96735 + 2.446562);
        let assign59590_e96738: f64 = (assign59590_e96737).sqrt();
        let assign59590_e96739: f64 = (assign59590_e96730 - assign59590_e96738);
        let assign59590_e96740: f64 = (0.5 * assign59590_e96739);
        (assign59590_e96740, (0.5 * (var_t2_dn3 - (((var_t2_dn3 * assign59590_e96734) + (var_t2 * var_t2_dn3)) / (2.0 * assign59590_e96738)))), (0.5 * (var_t2_dn4 - (((var_t2_dn4 * assign59590_e96734) + (var_t2 * var_t2_dn4)) / (2.0 * assign59590_e96738)))), (0.5 * (var_t2_dn5 - (((var_t2_dn5 * assign59590_e96734) + (var_t2 * var_t2_dn5)) / (2.0 * assign59590_e96738)))), (0.5 * (var_t2_dn6 - (((var_t2_dn6 * assign59590_e96734) + (var_t2 * var_t2_dn6)) / (2.0 * assign59590_e96738)))), (0.5 * (var_t2_dn7 - (((var_t2_dn7 * assign59590_e96734) + (var_t2 * var_t2_dn7)) / (2.0 * assign59590_e96738)))), (0.5 * (var_t2_dn8 - (((var_t2_dn8 * assign59590_e96734) + (var_t2 * var_t2_dn8)) / (2.0 * assign59590_e96738)))), (0.5 * (var_t2_dn9 - (((var_t2_dn9 * assign59590_e96734) + (var_t2 * var_t2_dn9)) / (2.0 * assign59590_e96738)))), (0.5 * (var_t2_dn10 - (((var_t2_dn10 * assign59590_e96734) + (var_t2 * var_t2_dn10)) / (2.0 * assign59590_e96738)))), (0.5 * (var_t2_dn11 - (((var_t2_dn11 * assign59590_e96734) + (var_t2 * var_t2_dn11)) / (2.0 * assign59590_e96738)))),)
    } else {
        (var_t8, var_t8_dn3, var_t8_dn4, var_t8_dn5, var_t8_dn6, var_t8_dn7, var_t8_dn8, var_t8_dn9, var_t8_dn10, var_t8_dn11,)
    }
};
        var_t8 = assign59590_e96742;
        var_t8_dn3 = assign59590_e96742_d_n3;
        var_t8_dn4 = assign59590_e96742_d_n4;
        var_t8_dn5 = assign59590_e96742_d_n5;
        var_t8_dn6 = assign59590_e96742_d_n6;
        var_t8_dn7 = assign59590_e96742_d_n7;
        var_t8_dn8 = assign59590_e96742_d_n8;
        var_t8_dn9 = assign59590_e96742_d_n9;
        var_t8_dn10 = assign59590_e96742_d_n10;
        var_t8_dn11 = assign59590_e96742_d_n11;
        var_t8_rv = 0.0;

        let (assign59600_e96749, assign59600_e96749_d_n3, assign59600_e96749_d_n4, assign59600_e96749_d_n5, assign59600_e96749_d_n6, assign59600_e96749_d_n7, assign59600_e96749_d_n8, assign59600_e96749_d_n9, assign59600_e96749_d_n10, assign59600_e96749_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        (var_sqrtpsip, var_sqrtpsip_dn3, var_sqrtpsip_dn4, var_sqrtpsip_dn5, var_sqrtpsip_dn6, var_sqrtpsip_dn7, var_sqrtpsip_dn8, var_sqrtpsip_dn9, var_sqrtpsip_dn10, var_sqrtpsip_dn11,)
    } else {
        (var_sqrtpsisa, var_sqrtpsisa_dn3, var_sqrtpsisa_dn4, var_sqrtpsisa_dn5, var_sqrtpsisa_dn6, var_sqrtpsisa_dn7, var_sqrtpsisa_dn8, var_sqrtpsisa_dn9, var_sqrtpsisa_dn10, var_sqrtpsisa_dn11,)
    }
};
        var_sqrtpsisa = assign59600_e96749;
        var_sqrtpsisa_dn3 = assign59600_e96749_d_n3;
        var_sqrtpsisa_dn4 = assign59600_e96749_d_n4;
        var_sqrtpsisa_dn5 = assign59600_e96749_d_n5;
        var_sqrtpsisa_dn6 = assign59600_e96749_d_n6;
        var_sqrtpsisa_dn7 = assign59600_e96749_d_n7;
        var_sqrtpsisa_dn8 = assign59600_e96749_d_n8;
        var_sqrtpsisa_dn9 = assign59600_e96749_d_n9;
        var_sqrtpsisa_dn10 = assign59600_e96749_d_n10;
        var_sqrtpsisa_dn11 = assign59600_e96749_d_n11;
        var_sqrtpsisa_rv = 0.0;

        let assign59610_e96752: f64 = (-68.0);
        let assign59610_e96753: f64 = if var_t8 <= assign59610_e96752 { 1.0 } else { 0.0 };
        var_guard871 = assign59610_e96753;
        var_guard871_rv = 0.0;

        let (assign59620_e96763, assign59620_e96763_d_n3, assign59620_e96763_d_n4, assign59620_e96763_d_n5, assign59620_e96763_d_n6, assign59620_e96763_d_n7, assign59620_e96763_d_n8, assign59620_e96763_d_n9, assign59620_e96763_d_n10, assign59620_e96763_d_n11,) = {
    if (((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard871 != 0.0)) {
        let assign59620_e96761: f64 = (-100.0);
        (assign59620_e96761, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t4, var_t4_dn3, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_dn9, var_t4_dn10, var_t4_dn11,)
    }
};
        var_t4 = assign59620_e96763;
        var_t4_dn3 = assign59620_e96763_d_n3;
        var_t4_dn4 = assign59620_e96763_d_n4;
        var_t4_dn5 = assign59620_e96763_d_n5;
        var_t4_dn6 = assign59620_e96763_d_n6;
        var_t4_dn7 = assign59620_e96763_d_n7;
        var_t4_dn8 = assign59620_e96763_d_n8;
        var_t4_dn9 = assign59620_e96763_d_n9;
        var_t4_dn10 = assign59620_e96763_d_n10;
        var_t4_dn11 = assign59620_e96763_d_n11;
        var_t4_rv = 0.0;

        let (assign59630_e96772, assign59630_e96772_d_n3, assign59630_e96772_d_n4, assign59630_e96772_d_n5, assign59630_e96772_d_n6, assign59630_e96772_d_n7, assign59630_e96772_d_n8, assign59630_e96772_d_n9, assign59630_e96772_d_n10, assign59630_e96772_d_n11,) = {
    if (((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard871 != 0.0)) {
        (20.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t5, var_t5_dn3, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8, var_t5_dn9, var_t5_dn10, var_t5_dn11,)
    }
};
        var_t5 = assign59630_e96772;
        var_t5_dn3 = assign59630_e96772_d_n3;
        var_t5_dn4 = assign59630_e96772_d_n4;
        var_t5_dn5 = assign59630_e96772_d_n5;
        var_t5_dn6 = assign59630_e96772_d_n6;
        var_t5_dn7 = assign59630_e96772_d_n7;
        var_t5_dn8 = assign59630_e96772_d_n8;
        var_t5_dn9 = assign59630_e96772_d_n9;
        var_t5_dn10 = assign59630_e96772_d_n10;
        var_t5_dn11 = assign59630_e96772_d_n11;
        var_t5_rv = 0.0;

        let assign59640_e96777: f64 = (0.5 * var_t5);
        let assign59640_e96778: f64 = (var_t4 - assign59640_e96777);
        let assign59640_e96779: f64 = if var_t8 < assign59640_e96778 { 1.0 } else { 0.0 };
        var_guard872 = assign59640_e96779;
        var_guard872_rv = 0.0;

        let (assign59650_e96791, assign59650_e96791_d_n3, assign59650_e96791_d_n4, assign59650_e96791_d_n5, assign59650_e96791_d_n6, assign59650_e96791_d_n7, assign59650_e96791_d_n8, assign59650_e96791_d_n9, assign59650_e96791_d_n10, assign59650_e96791_d_n11,) = {
    if ((((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard871 != 0.0)) && (var_guard872 != 0.0)) {
        let assign59650_e96789: f64 = { let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign59650_e96789, ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn3), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn4), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn5), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn6), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn7), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn8), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn9), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn10), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn11),)
    } else {
        (var_t3, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11,)
    }
};
        var_t3 = assign59650_e96791;
        var_t3_dn3 = assign59650_e96791_d_n3;
        var_t3_dn4 = assign59650_e96791_d_n4;
        var_t3_dn5 = assign59650_e96791_d_n5;
        var_t3_dn6 = assign59650_e96791_d_n6;
        var_t3_dn7 = assign59650_e96791_d_n7;
        var_t3_dn8 = assign59650_e96791_d_n8;
        var_t3_dn9 = assign59650_e96791_d_n9;
        var_t3_dn10 = assign59650_e96791_d_n10;
        var_t3_dn11 = assign59650_e96791_d_n11;
        var_t3_rv = 0.0;

        let assign59660_e96796: f64 = (0.5 * var_t5);
        let assign59660_e96797: f64 = (var_t4 + assign59660_e96796);
        let assign59660_e96798: f64 = if var_t8 > assign59660_e96797 { 1.0 } else { 0.0 };
        var_guard873 = assign59660_e96798;
        var_guard873_rv = 0.0;

        let (assign59670_e96813, assign59670_e96813_d_n3, assign59670_e96813_d_n4, assign59670_e96813_d_n5, assign59670_e96813_d_n6, assign59670_e96813_d_n7, assign59670_e96813_d_n8, assign59670_e96813_d_n9, assign59670_e96813_d_n10, assign59670_e96813_d_n11,) = {
    if (((((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard871 != 0.0)) && (var_guard872 == 0.0)) && (var_guard873 != 0.0)) {
        let assign59670_e96811: f64 = { let limited_exp_arg = var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign59670_e96811, ({ let limited_exp_arg = var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t8_dn3), ({ let limited_exp_arg = var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t8_dn4), ({ let limited_exp_arg = var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t8_dn5), ({ let limited_exp_arg = var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t8_dn6), ({ let limited_exp_arg = var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t8_dn7), ({ let limited_exp_arg = var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t8_dn8), ({ let limited_exp_arg = var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t8_dn9), ({ let limited_exp_arg = var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t8_dn10), ({ let limited_exp_arg = var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t8_dn11),)
    } else {
        (var_t3, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11,)
    }
};
        var_t3 = assign59670_e96813;
        var_t3_dn3 = assign59670_e96813_d_n3;
        var_t3_dn4 = assign59670_e96813_d_n4;
        var_t3_dn5 = assign59670_e96813_d_n5;
        var_t3_dn6 = assign59670_e96813_d_n6;
        var_t3_dn7 = assign59670_e96813_d_n7;
        var_t3_dn8 = assign59670_e96813_d_n8;
        var_t3_dn9 = assign59670_e96813_d_n9;
        var_t3_dn10 = assign59670_e96813_d_n10;
        var_t3_dn11 = assign59670_e96813_d_n11;
        var_t3_rv = 0.0;

        let (assign59680_e96832, assign59680_e96832_d_n3, assign59680_e96832_d_n4, assign59680_e96832_d_n5, assign59680_e96832_d_n6, assign59680_e96832_d_n7, assign59680_e96832_d_n8, assign59680_e96832_d_n9, assign59680_e96832_d_n10, assign59680_e96832_d_n11,) = {
    if (((((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard871 != 0.0)) && (var_guard872 == 0.0)) && (var_guard873 == 0.0)) {
        let assign59680_e96828: f64 = (var_t8 - var_t4);
        let assign59680_e96830: f64 = (assign59680_e96828 / var_t5);
        (assign59680_e96830, ((((var_t8_dn3 - var_t4_dn3) * var_t5) - (assign59680_e96828 * var_t5_dn3)) / (var_t5 * var_t5)), ((((var_t8_dn4 - var_t4_dn4) * var_t5) - (assign59680_e96828 * var_t5_dn4)) / (var_t5 * var_t5)), ((((var_t8_dn5 - var_t4_dn5) * var_t5) - (assign59680_e96828 * var_t5_dn5)) / (var_t5 * var_t5)), ((((var_t8_dn6 - var_t4_dn6) * var_t5) - (assign59680_e96828 * var_t5_dn6)) / (var_t5 * var_t5)), ((((var_t8_dn7 - var_t4_dn7) * var_t5) - (assign59680_e96828 * var_t5_dn7)) / (var_t5 * var_t5)), ((((var_t8_dn8 - var_t4_dn8) * var_t5) - (assign59680_e96828 * var_t5_dn8)) / (var_t5 * var_t5)), ((((var_t8_dn9 - var_t4_dn9) * var_t5) - (assign59680_e96828 * var_t5_dn9)) / (var_t5 * var_t5)), ((((var_t8_dn10 - var_t4_dn10) * var_t5) - (assign59680_e96828 * var_t5_dn10)) / (var_t5 * var_t5)), ((((var_t8_dn11 - var_t4_dn11) * var_t5) - (assign59680_e96828 * var_t5_dn11)) / (var_t5 * var_t5)),)
    } else {
        (var_t2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11,)
    }
};
        var_t2 = assign59680_e96832;
        var_t2_dn3 = assign59680_e96832_d_n3;
        var_t2_dn4 = assign59680_e96832_d_n4;
        var_t2_dn5 = assign59680_e96832_d_n5;
        var_t2_dn6 = assign59680_e96832_d_n6;
        var_t2_dn7 = assign59680_e96832_d_n7;
        var_t2_dn8 = assign59680_e96832_d_n8;
        var_t2_dn9 = assign59680_e96832_d_n9;
        var_t2_dn10 = assign59680_e96832_d_n10;
        var_t2_dn11 = assign59680_e96832_d_n11;
        var_t2_rv = 0.0;

        let (assign59690_e96849, assign59690_e96849_d_n3, assign59690_e96849_d_n4, assign59690_e96849_d_n5, assign59690_e96849_d_n6, assign59690_e96849_d_n7, assign59690_e96849_d_n8, assign59690_e96849_d_n9, assign59690_e96849_d_n10, assign59690_e96849_d_n11,) = {
    if (((((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard871 != 0.0)) && (var_guard872 == 0.0)) && (var_guard873 == 0.0)) {
        let assign59690_e96847: f64 = (var_t2 * var_t2);
        (assign59690_e96847, ((var_t2_dn3 * var_t2) + (var_t2 * var_t2_dn3)), ((var_t2_dn4 * var_t2) + (var_t2 * var_t2_dn4)), ((var_t2_dn5 * var_t2) + (var_t2 * var_t2_dn5)), ((var_t2_dn6 * var_t2) + (var_t2 * var_t2_dn6)), ((var_t2_dn7 * var_t2) + (var_t2 * var_t2_dn7)), ((var_t2_dn8 * var_t2) + (var_t2 * var_t2_dn8)), ((var_t2_dn9 * var_t2) + (var_t2 * var_t2_dn9)), ((var_t2_dn10 * var_t2) + (var_t2 * var_t2_dn10)), ((var_t2_dn11 * var_t2) + (var_t2 * var_t2_dn11)),)
    } else {
        (var_t6, var_t6_dn3, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_dn9, var_t6_dn10, var_t6_dn11,)
    }
};
        var_t6 = assign59690_e96849;
        var_t6_dn3 = assign59690_e96849_d_n3;
        var_t6_dn4 = assign59690_e96849_d_n4;
        var_t6_dn5 = assign59690_e96849_d_n5;
        var_t6_dn6 = assign59690_e96849_d_n6;
        var_t6_dn7 = assign59690_e96849_d_n7;
        var_t6_dn8 = assign59690_e96849_d_n8;
        var_t6_dn9 = assign59690_e96849_d_n9;
        var_t6_dn10 = assign59690_e96849_d_n10;
        var_t6_dn11 = assign59690_e96849_d_n11;
        var_t6_rv = 0.0;

        let (assign59700_e96887, assign59700_e96887_d_n3, assign59700_e96887_d_n4, assign59700_e96887_d_n5, assign59700_e96887_d_n6, assign59700_e96887_d_n7, assign59700_e96887_d_n8, assign59700_e96887_d_n9, assign59700_e96887_d_n10, assign59700_e96887_d_n11,) = {
    if (((((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard871 != 0.0)) && (var_guard872 == 0.0)) && (var_guard873 == 0.0)) {
        let assign59700_e96866: f64 = (5.0 / 64.0);
        let assign59700_e96869: f64 = (0.5 * var_t2);
        let assign59700_e96870: f64 = (assign59700_e96866 + assign59700_e96869);
        let assign59700_e96874: f64 = (15.0 / 16.0);
        let assign59700_e96878: f64 = (1.25 - var_t6);
        let assign59700_e96879: f64 = (var_t6 * assign59700_e96878);
        let assign59700_e96880: f64 = (assign59700_e96874 - assign59700_e96879);
        let assign59700_e96881: f64 = (var_t6 * assign59700_e96880);
        let assign59700_e96882: f64 = (assign59700_e96870 + assign59700_e96881);
        let assign59700_e96883: f64 = (var_t5 * assign59700_e96882);
        let assign59700_e96884: f64 = (var_t4 + assign59700_e96883);
        let assign59700_e96885: f64 = { let limited_exp_arg = assign59700_e96884; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign59700_e96885, ({ let limited_exp_arg = assign59700_e96884; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t4_dn3 + ((var_t5_dn3 * assign59700_e96882) + (var_t5 * ((0.5 * var_t2_dn3) + ((var_t6_dn3 * assign59700_e96880) + (var_t6 * (-((var_t6_dn3 * assign59700_e96878) + (var_t6 * (-var_t6_dn3))))))))))), ({ let limited_exp_arg = assign59700_e96884; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t4_dn4 + ((var_t5_dn4 * assign59700_e96882) + (var_t5 * ((0.5 * var_t2_dn4) + ((var_t6_dn4 * assign59700_e96880) + (var_t6 * (-((var_t6_dn4 * assign59700_e96878) + (var_t6 * (-var_t6_dn4))))))))))), ({ let limited_exp_arg = assign59700_e96884; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t4_dn5 + ((var_t5_dn5 * assign59700_e96882) + (var_t5 * ((0.5 * var_t2_dn5) + ((var_t6_dn5 * assign59700_e96880) + (var_t6 * (-((var_t6_dn5 * assign59700_e96878) + (var_t6 * (-var_t6_dn5))))))))))), ({ let limited_exp_arg = assign59700_e96884; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t4_dn6 + ((var_t5_dn6 * assign59700_e96882) + (var_t5 * ((0.5 * var_t2_dn6) + ((var_t6_dn6 * assign59700_e96880) + (var_t6 * (-((var_t6_dn6 * assign59700_e96878) + (var_t6 * (-var_t6_dn6))))))))))), ({ let limited_exp_arg = assign59700_e96884; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t4_dn7 + ((var_t5_dn7 * assign59700_e96882) + (var_t5 * ((0.5 * var_t2_dn7) + ((var_t6_dn7 * assign59700_e96880) + (var_t6 * (-((var_t6_dn7 * assign59700_e96878) + (var_t6 * (-var_t6_dn7))))))))))), ({ let limited_exp_arg = assign59700_e96884; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t4_dn8 + ((var_t5_dn8 * assign59700_e96882) + (var_t5 * ((0.5 * var_t2_dn8) + ((var_t6_dn8 * assign59700_e96880) + (var_t6 * (-((var_t6_dn8 * assign59700_e96878) + (var_t6 * (-var_t6_dn8))))))))))), ({ let limited_exp_arg = assign59700_e96884; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t4_dn9 + ((var_t5_dn9 * assign59700_e96882) + (var_t5 * ((0.5 * var_t2_dn9) + ((var_t6_dn9 * assign59700_e96880) + (var_t6 * (-((var_t6_dn9 * assign59700_e96878) + (var_t6 * (-var_t6_dn9))))))))))), ({ let limited_exp_arg = assign59700_e96884; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t4_dn10 + ((var_t5_dn10 * assign59700_e96882) + (var_t5 * ((0.5 * var_t2_dn10) + ((var_t6_dn10 * assign59700_e96880) + (var_t6 * (-((var_t6_dn10 * assign59700_e96878) + (var_t6 * (-var_t6_dn10))))))))))), ({ let limited_exp_arg = assign59700_e96884; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t4_dn11 + ((var_t5_dn11 * assign59700_e96882) + (var_t5 * ((0.5 * var_t2_dn11) + ((var_t6_dn11 * assign59700_e96880) + (var_t6 * (-((var_t6_dn11 * assign59700_e96878) + (var_t6 * (-var_t6_dn11))))))))))),)
    } else {
        (var_t3, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11,)
    }
};
        var_t3 = assign59700_e96887;
        var_t3_dn3 = assign59700_e96887_d_n3;
        var_t3_dn4 = assign59700_e96887_d_n4;
        var_t3_dn5 = assign59700_e96887_d_n5;
        var_t3_dn6 = assign59700_e96887_d_n6;
        var_t3_dn7 = assign59700_e96887_d_n7;
        var_t3_dn8 = assign59700_e96887_d_n8;
        var_t3_dn9 = assign59700_e96887_d_n9;
        var_t3_dn10 = assign59700_e96887_d_n10;
        var_t3_dn11 = assign59700_e96887_d_n11;
        var_t3_rv = 0.0;

        let (assign59710_e96919, assign59710_e96919_d_n3, assign59710_e96919_d_n4, assign59710_e96919_d_n5, assign59710_e96919_d_n6, assign59710_e96919_d_n7, assign59710_e96919_d_n8, assign59710_e96919_d_n9, assign59710_e96919_d_n10, assign59710_e96919_d_n11,) = {
    if (((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard871 != 0.0)) {
        let assign59710_e96897: f64 = (1.0 + var_t1);
        let assign59710_e96899: f64 = (assign59710_e96897 - var_t8);
        let assign59710_e96902: f64 = (2.0 * var_t0);
        let assign59710_e96905: f64 = (var_t3 * 2.0);
        let assign59710_e96907: f64 = (assign59710_e96905 * var_t0);
        let assign59710_e96910: f64 = (2.0 * var_sqrtpsisa);
        let assign59710_e96911: f64 = (assign59710_e96907 + assign59710_e96910);
        let assign59710_e96912: f64 = (assign59710_e96902 * assign59710_e96911);
        let assign59710_e96914: f64 = (assign59710_e96912).max(1e-38);
        let assign59710_e96915: f64 = (assign59710_e96914).ln();
        let assign59710_e96916: f64 = (assign59710_e96899 - assign59710_e96915);
        let assign59710_e96917: f64 = (var_t3 * assign59710_e96916);
        (assign59710_e96917, ((var_t3_dn3 * assign59710_e96916) + (var_t3 * ((var_t1_dn3 - var_t8_dn3) - (if assign59710_e96912 >= 1e-38 { (((2.0 * var_t0_dn3) * assign59710_e96911) + (assign59710_e96902 * ((((var_t3_dn3 * 2.0) * var_t0) + (assign59710_e96905 * var_t0_dn3)) + (2.0 * var_sqrtpsisa_dn3)))) } else { 0.0 } / assign59710_e96914)))), ((var_t3_dn4 * assign59710_e96916) + (var_t3 * ((var_t1_dn4 - var_t8_dn4) - (if assign59710_e96912 >= 1e-38 { (((2.0 * var_t0_dn4) * assign59710_e96911) + (assign59710_e96902 * ((((var_t3_dn4 * 2.0) * var_t0) + (assign59710_e96905 * var_t0_dn4)) + (2.0 * var_sqrtpsisa_dn4)))) } else { 0.0 } / assign59710_e96914)))), ((var_t3_dn5 * assign59710_e96916) + (var_t3 * ((var_t1_dn5 - var_t8_dn5) - (if assign59710_e96912 >= 1e-38 { (((2.0 * var_t0_dn5) * assign59710_e96911) + (assign59710_e96902 * ((((var_t3_dn5 * 2.0) * var_t0) + (assign59710_e96905 * var_t0_dn5)) + (2.0 * var_sqrtpsisa_dn5)))) } else { 0.0 } / assign59710_e96914)))), ((var_t3_dn6 * assign59710_e96916) + (var_t3 * ((var_t1_dn6 - var_t8_dn6) - (if assign59710_e96912 >= 1e-38 { (((2.0 * var_t0_dn6) * assign59710_e96911) + (assign59710_e96902 * ((((var_t3_dn6 * 2.0) * var_t0) + (assign59710_e96905 * var_t0_dn6)) + (2.0 * var_sqrtpsisa_dn6)))) } else { 0.0 } / assign59710_e96914)))), ((var_t3_dn7 * assign59710_e96916) + (var_t3 * ((var_t1_dn7 - var_t8_dn7) - (if assign59710_e96912 >= 1e-38 { (((2.0 * var_t0_dn7) * assign59710_e96911) + (assign59710_e96902 * ((((var_t3_dn7 * 2.0) * var_t0) + (assign59710_e96905 * var_t0_dn7)) + (2.0 * var_sqrtpsisa_dn7)))) } else { 0.0 } / assign59710_e96914)))), ((var_t3_dn8 * assign59710_e96916) + (var_t3 * ((var_t1_dn8 - var_t8_dn8) - (if assign59710_e96912 >= 1e-38 { (((2.0 * var_t0_dn8) * assign59710_e96911) + (assign59710_e96902 * ((((var_t3_dn8 * 2.0) * var_t0) + (assign59710_e96905 * var_t0_dn8)) + (2.0 * var_sqrtpsisa_dn8)))) } else { 0.0 } / assign59710_e96914)))), ((var_t3_dn9 * assign59710_e96916) + (var_t3 * ((var_t1_dn9 - var_t8_dn9) - (if assign59710_e96912 >= 1e-38 { (((2.0 * var_t0_dn9) * assign59710_e96911) + (assign59710_e96902 * ((((var_t3_dn9 * 2.0) * var_t0) + (assign59710_e96905 * var_t0_dn9)) + (2.0 * var_sqrtpsisa_dn9)))) } else { 0.0 } / assign59710_e96914)))), ((var_t3_dn10 * assign59710_e96916) + (var_t3 * ((var_t1_dn10 - var_t8_dn10) - (if assign59710_e96912 >= 1e-38 { (((2.0 * var_t0_dn10) * assign59710_e96911) + (assign59710_e96902 * ((((var_t3_dn10 * 2.0) * var_t0) + (assign59710_e96905 * var_t0_dn10)) + (2.0 * var_sqrtpsisa_dn10)))) } else { 0.0 } / assign59710_e96914)))), ((var_t3_dn11 * assign59710_e96916) + (var_t3 * ((var_t1_dn11 - var_t8_dn11) - (if assign59710_e96912 >= 1e-38 { (((2.0 * var_t0_dn11) * assign59710_e96911) + (assign59710_e96902 * ((((var_t3_dn11 * 2.0) * var_t0) + (assign59710_e96905 * var_t0_dn11)) + (2.0 * var_sqrtpsisa_dn11)))) } else { 0.0 } / assign59710_e96914)))),)
    } else {
        (var_qs_edge, var_qs_edge_dn3, var_qs_edge_dn4, var_qs_edge_dn5, var_qs_edge_dn6, var_qs_edge_dn7, var_qs_edge_dn8, var_qs_edge_dn9, var_qs_edge_dn10, var_qs_edge_dn11,)
    }
};
        var_qs_edge = assign59710_e96919;
        var_qs_edge_dn3 = assign59710_e96919_d_n3;
        var_qs_edge_dn4 = assign59710_e96919_d_n4;
        var_qs_edge_dn5 = assign59710_e96919_d_n5;
        var_qs_edge_dn6 = assign59710_e96919_d_n6;
        var_qs_edge_dn7 = assign59710_e96919_d_n7;
        var_qs_edge_dn8 = assign59710_e96919_d_n8;
        var_qs_edge_dn9 = assign59710_e96919_d_n9;
        var_qs_edge_dn10 = assign59710_e96919_d_n10;
        var_qs_edge_dn11 = assign59710_e96919_d_n11;
        var_qs_edge_rv = 0.0;

        let (assign59720_e96930, assign59720_e96930_d_n3, assign59720_e96930_d_n4, assign59720_e96930_d_n5, assign59720_e96930_d_n6, assign59720_e96930_d_n7, assign59720_e96930_d_n8, assign59720_e96930_d_n9, assign59720_e96930_d_n10, assign59720_e96930_d_n11,) = {
    if (((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard871 == 0.0)) {
        let assign59720_e96928: f64 = { let limited_exp_arg = var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign59720_e96928, ({ let limited_exp_arg = var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t8_dn3), ({ let limited_exp_arg = var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t8_dn4), ({ let limited_exp_arg = var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t8_dn5), ({ let limited_exp_arg = var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t8_dn6), ({ let limited_exp_arg = var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t8_dn7), ({ let limited_exp_arg = var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t8_dn8), ({ let limited_exp_arg = var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t8_dn9), ({ let limited_exp_arg = var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t8_dn10), ({ let limited_exp_arg = var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t8_dn11),)
    } else {
        (var_t3, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11,)
    }
};
        var_t3 = assign59720_e96930;
        var_t3_dn3 = assign59720_e96930_d_n3;
        var_t3_dn4 = assign59720_e96930_d_n4;
        var_t3_dn5 = assign59720_e96930_d_n5;
        var_t3_dn6 = assign59720_e96930_d_n6;
        var_t3_dn7 = assign59720_e96930_d_n7;
        var_t3_dn8 = assign59720_e96930_d_n8;
        var_t3_dn9 = assign59720_e96930_d_n9;
        var_t3_dn10 = assign59720_e96930_d_n10;
        var_t3_dn11 = assign59720_e96930_d_n11;
        var_t3_rv = 0.0;

        let (assign59730_e96942, assign59730_e96942_d_n3, assign59730_e96942_d_n4, assign59730_e96942_d_n5, assign59730_e96942_d_n6, assign59730_e96942_d_n7, assign59730_e96942_d_n8, assign59730_e96942_d_n9, assign59730_e96942_d_n10, assign59730_e96942_d_n11,) = {
    if (((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard871 == 0.0)) {
        let assign59730_e96940: f64 = (1.0 / var_sqrtpsisa);
        (assign59730_e96940, (-(var_sqrtpsisa_dn3 / (var_sqrtpsisa * var_sqrtpsisa))), (-(var_sqrtpsisa_dn4 / (var_sqrtpsisa * var_sqrtpsisa))), (-(var_sqrtpsisa_dn5 / (var_sqrtpsisa * var_sqrtpsisa))), (-(var_sqrtpsisa_dn6 / (var_sqrtpsisa * var_sqrtpsisa))), (-(var_sqrtpsisa_dn7 / (var_sqrtpsisa * var_sqrtpsisa))), (-(var_sqrtpsisa_dn8 / (var_sqrtpsisa * var_sqrtpsisa))), (-(var_sqrtpsisa_dn9 / (var_sqrtpsisa * var_sqrtpsisa))), (-(var_sqrtpsisa_dn10 / (var_sqrtpsisa * var_sqrtpsisa))), (-(var_sqrtpsisa_dn11 / (var_sqrtpsisa * var_sqrtpsisa))),)
    } else {
        (var_sqrtpsisainv, var_sqrtpsisainv_dn3, var_sqrtpsisainv_dn4, var_sqrtpsisainv_dn5, var_sqrtpsisainv_dn6, var_sqrtpsisainv_dn7, var_sqrtpsisainv_dn8, var_sqrtpsisainv_dn9, var_sqrtpsisainv_dn10, var_sqrtpsisainv_dn11,)
    }
};
        var_sqrtpsisainv = assign59730_e96942;
        var_sqrtpsisainv_dn3 = assign59730_e96942_d_n3;
        var_sqrtpsisainv_dn4 = assign59730_e96942_d_n4;
        var_sqrtpsisainv_dn5 = assign59730_e96942_d_n5;
        var_sqrtpsisainv_dn6 = assign59730_e96942_d_n6;
        var_sqrtpsisainv_dn7 = assign59730_e96942_d_n7;
        var_sqrtpsisainv_dn8 = assign59730_e96942_d_n8;
        var_sqrtpsisainv_dn9 = assign59730_e96942_d_n9;
        var_sqrtpsisainv_dn10 = assign59730_e96942_d_n10;
        var_sqrtpsisainv_dn11 = assign59730_e96942_d_n11;
        var_sqrtpsisainv_rv = 0.0;

        let (assign59740_e96975, assign59740_e96975_d_n3, assign59740_e96975_d_n4, assign59740_e96975_d_n5, assign59740_e96975_d_n6, assign59740_e96975_d_n7, assign59740_e96975_d_n8, assign59740_e96975_d_n9, assign59740_e96975_d_n10, assign59740_e96975_d_n11,) = {
    if (((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard871 == 0.0)) {
        let assign59740_e96952: f64 = (2.0 * var_t3);
        let assign59740_e96955: f64 = (var_t3 * 2.0);
        let assign59740_e96957: f64 = (assign59740_e96955 * var_t0);
        let assign59740_e96960: f64 = (var_t3 * 2.0);
        let assign59740_e96962: f64 = (assign59740_e96960 * var_t0);
        let assign59740_e96965: f64 = (2.0 * var_sqrtpsisa);
        let assign59740_e96966: f64 = (assign59740_e96962 + assign59740_e96965);
        let assign59740_e96967: f64 = (assign59740_e96957 * assign59740_e96966);
        let assign59740_e96969: f64 = (assign59740_e96967).max(1e-38);
        let assign59740_e96970: f64 = (assign59740_e96969).ln();
        let assign59740_e96971: f64 = (assign59740_e96952 + assign59740_e96970);
        let assign59740_e96973: f64 = (assign59740_e96971 - var_t1);
        (assign59740_e96973, (((2.0 * var_t3_dn3) + (if assign59740_e96967 >= 1e-38 { (((((var_t3_dn3 * 2.0) * var_t0) + (assign59740_e96955 * var_t0_dn3)) * assign59740_e96966) + (assign59740_e96957 * ((((var_t3_dn3 * 2.0) * var_t0) + (assign59740_e96960 * var_t0_dn3)) + (2.0 * var_sqrtpsisa_dn3)))) } else { 0.0 } / assign59740_e96969)) - var_t1_dn3), (((2.0 * var_t3_dn4) + (if assign59740_e96967 >= 1e-38 { (((((var_t3_dn4 * 2.0) * var_t0) + (assign59740_e96955 * var_t0_dn4)) * assign59740_e96966) + (assign59740_e96957 * ((((var_t3_dn4 * 2.0) * var_t0) + (assign59740_e96960 * var_t0_dn4)) + (2.0 * var_sqrtpsisa_dn4)))) } else { 0.0 } / assign59740_e96969)) - var_t1_dn4), (((2.0 * var_t3_dn5) + (if assign59740_e96967 >= 1e-38 { (((((var_t3_dn5 * 2.0) * var_t0) + (assign59740_e96955 * var_t0_dn5)) * assign59740_e96966) + (assign59740_e96957 * ((((var_t3_dn5 * 2.0) * var_t0) + (assign59740_e96960 * var_t0_dn5)) + (2.0 * var_sqrtpsisa_dn5)))) } else { 0.0 } / assign59740_e96969)) - var_t1_dn5), (((2.0 * var_t3_dn6) + (if assign59740_e96967 >= 1e-38 { (((((var_t3_dn6 * 2.0) * var_t0) + (assign59740_e96955 * var_t0_dn6)) * assign59740_e96966) + (assign59740_e96957 * ((((var_t3_dn6 * 2.0) * var_t0) + (assign59740_e96960 * var_t0_dn6)) + (2.0 * var_sqrtpsisa_dn6)))) } else { 0.0 } / assign59740_e96969)) - var_t1_dn6), (((2.0 * var_t3_dn7) + (if assign59740_e96967 >= 1e-38 { (((((var_t3_dn7 * 2.0) * var_t0) + (assign59740_e96955 * var_t0_dn7)) * assign59740_e96966) + (assign59740_e96957 * ((((var_t3_dn7 * 2.0) * var_t0) + (assign59740_e96960 * var_t0_dn7)) + (2.0 * var_sqrtpsisa_dn7)))) } else { 0.0 } / assign59740_e96969)) - var_t1_dn7), (((2.0 * var_t3_dn8) + (if assign59740_e96967 >= 1e-38 { (((((var_t3_dn8 * 2.0) * var_t0) + (assign59740_e96955 * var_t0_dn8)) * assign59740_e96966) + (assign59740_e96957 * ((((var_t3_dn8 * 2.0) * var_t0) + (assign59740_e96960 * var_t0_dn8)) + (2.0 * var_sqrtpsisa_dn8)))) } else { 0.0 } / assign59740_e96969)) - var_t1_dn8), (((2.0 * var_t3_dn9) + (if assign59740_e96967 >= 1e-38 { (((((var_t3_dn9 * 2.0) * var_t0) + (assign59740_e96955 * var_t0_dn9)) * assign59740_e96966) + (assign59740_e96957 * ((((var_t3_dn9 * 2.0) * var_t0) + (assign59740_e96960 * var_t0_dn9)) + (2.0 * var_sqrtpsisa_dn9)))) } else { 0.0 } / assign59740_e96969)) - var_t1_dn9), (((2.0 * var_t3_dn10) + (if assign59740_e96967 >= 1e-38 { (((((var_t3_dn10 * 2.0) * var_t0) + (assign59740_e96955 * var_t0_dn10)) * assign59740_e96966) + (assign59740_e96957 * ((((var_t3_dn10 * 2.0) * var_t0) + (assign59740_e96960 * var_t0_dn10)) + (2.0 * var_sqrtpsisa_dn10)))) } else { 0.0 } / assign59740_e96969)) - var_t1_dn10), (((2.0 * var_t3_dn11) + (if assign59740_e96967 >= 1e-38 { (((((var_t3_dn11 * 2.0) * var_t0) + (assign59740_e96955 * var_t0_dn11)) * assign59740_e96966) + (assign59740_e96957 * ((((var_t3_dn11 * 2.0) * var_t0) + (assign59740_e96960 * var_t0_dn11)) + (2.0 * var_sqrtpsisa_dn11)))) } else { 0.0 } / assign59740_e96969)) - var_t1_dn11),)
    } else {
        (var_t4, var_t4_dn3, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_dn9, var_t4_dn10, var_t4_dn11,)
    }
};
        var_t4 = assign59740_e96975;
        var_t4_dn3 = assign59740_e96975_d_n3;
        var_t4_dn4 = assign59740_e96975_d_n4;
        var_t4_dn5 = assign59740_e96975_d_n5;
        var_t4_dn6 = assign59740_e96975_d_n6;
        var_t4_dn7 = assign59740_e96975_d_n7;
        var_t4_dn8 = assign59740_e96975_d_n8;
        var_t4_dn9 = assign59740_e96975_d_n9;
        var_t4_dn10 = assign59740_e96975_d_n10;
        var_t4_dn11 = assign59740_e96975_d_n11;
        var_t4_rv = 0.0;

        let (assign59750_e96999, assign59750_e96999_d_n3, assign59750_e96999_d_n4, assign59750_e96999_d_n5, assign59750_e96999_d_n6, assign59750_e96999_d_n7, assign59750_e96999_d_n8, assign59750_e96999_d_n9, assign59750_e96999_d_n10, assign59750_e96999_d_n11,) = {
    if (((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard871 == 0.0)) {
        let assign59750_e96986: f64 = (1.0 / var_t3);
        let assign59750_e96987: f64 = (2.0 + assign59750_e96986);
        let assign59750_e96990: f64 = (var_t0 + var_sqrtpsisainv);
        let assign59750_e96993: f64 = (var_t0 * var_t3);
        let assign59750_e96995: f64 = (assign59750_e96993 + var_sqrtpsisa);
        let assign59750_e96996: f64 = (assign59750_e96990 / assign59750_e96995);
        let assign59750_e96997: f64 = (assign59750_e96987 + assign59750_e96996);
        (assign59750_e96997, ((-(var_t3_dn3 / (var_t3 * var_t3))) + ((((var_t0_dn3 + var_sqrtpsisainv_dn3) * assign59750_e96995) - (assign59750_e96990 * (((var_t0_dn3 * var_t3) + (var_t0 * var_t3_dn3)) + var_sqrtpsisa_dn3))) / (assign59750_e96995 * assign59750_e96995))), ((-(var_t3_dn4 / (var_t3 * var_t3))) + ((((var_t0_dn4 + var_sqrtpsisainv_dn4) * assign59750_e96995) - (assign59750_e96990 * (((var_t0_dn4 * var_t3) + (var_t0 * var_t3_dn4)) + var_sqrtpsisa_dn4))) / (assign59750_e96995 * assign59750_e96995))), ((-(var_t3_dn5 / (var_t3 * var_t3))) + ((((var_t0_dn5 + var_sqrtpsisainv_dn5) * assign59750_e96995) - (assign59750_e96990 * (((var_t0_dn5 * var_t3) + (var_t0 * var_t3_dn5)) + var_sqrtpsisa_dn5))) / (assign59750_e96995 * assign59750_e96995))), ((-(var_t3_dn6 / (var_t3 * var_t3))) + ((((var_t0_dn6 + var_sqrtpsisainv_dn6) * assign59750_e96995) - (assign59750_e96990 * (((var_t0_dn6 * var_t3) + (var_t0 * var_t3_dn6)) + var_sqrtpsisa_dn6))) / (assign59750_e96995 * assign59750_e96995))), ((-(var_t3_dn7 / (var_t3 * var_t3))) + ((((var_t0_dn7 + var_sqrtpsisainv_dn7) * assign59750_e96995) - (assign59750_e96990 * (((var_t0_dn7 * var_t3) + (var_t0 * var_t3_dn7)) + var_sqrtpsisa_dn7))) / (assign59750_e96995 * assign59750_e96995))), ((-(var_t3_dn8 / (var_t3 * var_t3))) + ((((var_t0_dn8 + var_sqrtpsisainv_dn8) * assign59750_e96995) - (assign59750_e96990 * (((var_t0_dn8 * var_t3) + (var_t0 * var_t3_dn8)) + var_sqrtpsisa_dn8))) / (assign59750_e96995 * assign59750_e96995))), ((-(var_t3_dn9 / (var_t3 * var_t3))) + ((((var_t0_dn9 + var_sqrtpsisainv_dn9) * assign59750_e96995) - (assign59750_e96990 * (((var_t0_dn9 * var_t3) + (var_t0 * var_t3_dn9)) + var_sqrtpsisa_dn9))) / (assign59750_e96995 * assign59750_e96995))), ((-(var_t3_dn10 / (var_t3 * var_t3))) + ((((var_t0_dn10 + var_sqrtpsisainv_dn10) * assign59750_e96995) - (assign59750_e96990 * (((var_t0_dn10 * var_t3) + (var_t0 * var_t3_dn10)) + var_sqrtpsisa_dn10))) / (assign59750_e96995 * assign59750_e96995))), ((-(var_t3_dn11 / (var_t3 * var_t3))) + ((((var_t0_dn11 + var_sqrtpsisainv_dn11) * assign59750_e96995) - (assign59750_e96990 * (((var_t0_dn11 * var_t3) + (var_t0 * var_t3_dn11)) + var_sqrtpsisa_dn11))) / (assign59750_e96995 * assign59750_e96995))),)
    } else {
        (var_t5, var_t5_dn3, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8, var_t5_dn9, var_t5_dn10, var_t5_dn11,)
    }
};
        var_t5 = assign59750_e96999;
        var_t5_dn3 = assign59750_e96999_d_n3;
        var_t5_dn4 = assign59750_e96999_d_n4;
        var_t5_dn5 = assign59750_e96999_d_n5;
        var_t5_dn6 = assign59750_e96999_d_n6;
        var_t5_dn7 = assign59750_e96999_d_n7;
        var_t5_dn8 = assign59750_e96999_d_n8;
        var_t5_dn9 = assign59750_e96999_d_n9;
        var_t5_dn10 = assign59750_e96999_d_n10;
        var_t5_dn11 = assign59750_e96999_d_n11;
        var_t5_rv = 0.0;

        let (assign59760_e97013, assign59760_e97013_d_n3, assign59760_e97013_d_n4, assign59760_e97013_d_n5, assign59760_e97013_d_n6, assign59760_e97013_d_n7, assign59760_e97013_d_n8, assign59760_e97013_d_n9, assign59760_e97013_d_n10, assign59760_e97013_d_n11,) = {
    if (((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard871 == 0.0)) {
        let assign59760_e97010: f64 = (var_t4 / var_t5);
        let assign59760_e97011: f64 = (var_t3 - assign59760_e97010);
        (assign59760_e97011, (var_t3_dn3 - (((var_t4_dn3 * var_t5) - (var_t4 * var_t5_dn3)) / (var_t5 * var_t5))), (var_t3_dn4 - (((var_t4_dn4 * var_t5) - (var_t4 * var_t5_dn4)) / (var_t5 * var_t5))), (var_t3_dn5 - (((var_t4_dn5 * var_t5) - (var_t4 * var_t5_dn5)) / (var_t5 * var_t5))), (var_t3_dn6 - (((var_t4_dn6 * var_t5) - (var_t4 * var_t5_dn6)) / (var_t5 * var_t5))), (var_t3_dn7 - (((var_t4_dn7 * var_t5) - (var_t4 * var_t5_dn7)) / (var_t5 * var_t5))), (var_t3_dn8 - (((var_t4_dn8 * var_t5) - (var_t4 * var_t5_dn8)) / (var_t5 * var_t5))), (var_t3_dn9 - (((var_t4_dn9 * var_t5) - (var_t4 * var_t5_dn9)) / (var_t5 * var_t5))), (var_t3_dn10 - (((var_t4_dn10 * var_t5) - (var_t4 * var_t5_dn10)) / (var_t5 * var_t5))), (var_t3_dn11 - (((var_t4_dn11 * var_t5) - (var_t4 * var_t5_dn11)) / (var_t5 * var_t5))),)
    } else {
        (var_t3, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11,)
    }
};
        var_t3 = assign59760_e97013;
        var_t3_dn3 = assign59760_e97013_d_n3;
        var_t3_dn4 = assign59760_e97013_d_n4;
        var_t3_dn5 = assign59760_e97013_d_n5;
        var_t3_dn6 = assign59760_e97013_d_n6;
        var_t3_dn7 = assign59760_e97013_d_n7;
        var_t3_dn8 = assign59760_e97013_d_n8;
        var_t3_dn9 = assign59760_e97013_d_n9;
        var_t3_dn10 = assign59760_e97013_d_n10;
        var_t3_dn11 = assign59760_e97013_d_n11;
        var_t3_rv = 0.0;

        let (assign59770_e97046, assign59770_e97046_d_n3, assign59770_e97046_d_n4, assign59770_e97046_d_n5, assign59770_e97046_d_n6, assign59770_e97046_d_n7, assign59770_e97046_d_n8, assign59770_e97046_d_n9, assign59770_e97046_d_n10, assign59770_e97046_d_n11,) = {
    if (((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard871 == 0.0)) {
        let assign59770_e97023: f64 = (2.0 * var_t3);
        let assign59770_e97026: f64 = (var_t3 * 2.0);
        let assign59770_e97028: f64 = (assign59770_e97026 * var_t0);
        let assign59770_e97031: f64 = (var_t3 * 2.0);
        let assign59770_e97033: f64 = (assign59770_e97031 * var_t0);
        let assign59770_e97036: f64 = (2.0 * var_sqrtpsisa);
        let assign59770_e97037: f64 = (assign59770_e97033 + assign59770_e97036);
        let assign59770_e97038: f64 = (assign59770_e97028 * assign59770_e97037);
        let assign59770_e97040: f64 = (assign59770_e97038).max(1e-38);
        let assign59770_e97041: f64 = (assign59770_e97040).ln();
        let assign59770_e97042: f64 = (assign59770_e97023 + assign59770_e97041);
        let assign59770_e97044: f64 = (assign59770_e97042 - var_t1);
        (assign59770_e97044, (((2.0 * var_t3_dn3) + (if assign59770_e97038 >= 1e-38 { (((((var_t3_dn3 * 2.0) * var_t0) + (assign59770_e97026 * var_t0_dn3)) * assign59770_e97037) + (assign59770_e97028 * ((((var_t3_dn3 * 2.0) * var_t0) + (assign59770_e97031 * var_t0_dn3)) + (2.0 * var_sqrtpsisa_dn3)))) } else { 0.0 } / assign59770_e97040)) - var_t1_dn3), (((2.0 * var_t3_dn4) + (if assign59770_e97038 >= 1e-38 { (((((var_t3_dn4 * 2.0) * var_t0) + (assign59770_e97026 * var_t0_dn4)) * assign59770_e97037) + (assign59770_e97028 * ((((var_t3_dn4 * 2.0) * var_t0) + (assign59770_e97031 * var_t0_dn4)) + (2.0 * var_sqrtpsisa_dn4)))) } else { 0.0 } / assign59770_e97040)) - var_t1_dn4), (((2.0 * var_t3_dn5) + (if assign59770_e97038 >= 1e-38 { (((((var_t3_dn5 * 2.0) * var_t0) + (assign59770_e97026 * var_t0_dn5)) * assign59770_e97037) + (assign59770_e97028 * ((((var_t3_dn5 * 2.0) * var_t0) + (assign59770_e97031 * var_t0_dn5)) + (2.0 * var_sqrtpsisa_dn5)))) } else { 0.0 } / assign59770_e97040)) - var_t1_dn5), (((2.0 * var_t3_dn6) + (if assign59770_e97038 >= 1e-38 { (((((var_t3_dn6 * 2.0) * var_t0) + (assign59770_e97026 * var_t0_dn6)) * assign59770_e97037) + (assign59770_e97028 * ((((var_t3_dn6 * 2.0) * var_t0) + (assign59770_e97031 * var_t0_dn6)) + (2.0 * var_sqrtpsisa_dn6)))) } else { 0.0 } / assign59770_e97040)) - var_t1_dn6), (((2.0 * var_t3_dn7) + (if assign59770_e97038 >= 1e-38 { (((((var_t3_dn7 * 2.0) * var_t0) + (assign59770_e97026 * var_t0_dn7)) * assign59770_e97037) + (assign59770_e97028 * ((((var_t3_dn7 * 2.0) * var_t0) + (assign59770_e97031 * var_t0_dn7)) + (2.0 * var_sqrtpsisa_dn7)))) } else { 0.0 } / assign59770_e97040)) - var_t1_dn7), (((2.0 * var_t3_dn8) + (if assign59770_e97038 >= 1e-38 { (((((var_t3_dn8 * 2.0) * var_t0) + (assign59770_e97026 * var_t0_dn8)) * assign59770_e97037) + (assign59770_e97028 * ((((var_t3_dn8 * 2.0) * var_t0) + (assign59770_e97031 * var_t0_dn8)) + (2.0 * var_sqrtpsisa_dn8)))) } else { 0.0 } / assign59770_e97040)) - var_t1_dn8), (((2.0 * var_t3_dn9) + (if assign59770_e97038 >= 1e-38 { (((((var_t3_dn9 * 2.0) * var_t0) + (assign59770_e97026 * var_t0_dn9)) * assign59770_e97037) + (assign59770_e97028 * ((((var_t3_dn9 * 2.0) * var_t0) + (assign59770_e97031 * var_t0_dn9)) + (2.0 * var_sqrtpsisa_dn9)))) } else { 0.0 } / assign59770_e97040)) - var_t1_dn9), (((2.0 * var_t3_dn10) + (if assign59770_e97038 >= 1e-38 { (((((var_t3_dn10 * 2.0) * var_t0) + (assign59770_e97026 * var_t0_dn10)) * assign59770_e97037) + (assign59770_e97028 * ((((var_t3_dn10 * 2.0) * var_t0) + (assign59770_e97031 * var_t0_dn10)) + (2.0 * var_sqrtpsisa_dn10)))) } else { 0.0 } / assign59770_e97040)) - var_t1_dn10), (((2.0 * var_t3_dn11) + (if assign59770_e97038 >= 1e-38 { (((((var_t3_dn11 * 2.0) * var_t0) + (assign59770_e97026 * var_t0_dn11)) * assign59770_e97037) + (assign59770_e97028 * ((((var_t3_dn11 * 2.0) * var_t0) + (assign59770_e97031 * var_t0_dn11)) + (2.0 * var_sqrtpsisa_dn11)))) } else { 0.0 } / assign59770_e97040)) - var_t1_dn11),)
    } else {
        (var_t4, var_t4_dn3, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_dn9, var_t4_dn10, var_t4_dn11,)
    }
};
        var_t4 = assign59770_e97046;
        var_t4_dn3 = assign59770_e97046_d_n3;
        var_t4_dn4 = assign59770_e97046_d_n4;
        var_t4_dn5 = assign59770_e97046_d_n5;
        var_t4_dn6 = assign59770_e97046_d_n6;
        var_t4_dn7 = assign59770_e97046_d_n7;
        var_t4_dn8 = assign59770_e97046_d_n8;
        var_t4_dn9 = assign59770_e97046_d_n9;
        var_t4_dn10 = assign59770_e97046_d_n10;
        var_t4_dn11 = assign59770_e97046_d_n11;
        var_t4_rv = 0.0;

        *var_guard871_slot = var_guard871;
        *var_guard871_rv_slot = var_guard871_rv;
        *var_guard872_slot = var_guard872;
        *var_guard872_rv_slot = var_guard872_rv;
        *var_guard873_slot = var_guard873;
        *var_guard873_rv_slot = var_guard873_rv;
        *var_qs_edge_slot = var_qs_edge;
        *var_qs_edge_dn10_slot = var_qs_edge_dn10;
        *var_qs_edge_dn11_slot = var_qs_edge_dn11;
        *var_qs_edge_dn3_slot = var_qs_edge_dn3;
        *var_qs_edge_dn4_slot = var_qs_edge_dn4;
        *var_qs_edge_dn5_slot = var_qs_edge_dn5;
        *var_qs_edge_dn6_slot = var_qs_edge_dn6;
        *var_qs_edge_dn7_slot = var_qs_edge_dn7;
        *var_qs_edge_dn8_slot = var_qs_edge_dn8;
        *var_qs_edge_dn9_slot = var_qs_edge_dn9;
        *var_qs_edge_rv_slot = var_qs_edge_rv;
        *var_sqrtpsip_slot = var_sqrtpsip;
        *var_sqrtpsip_dn10_slot = var_sqrtpsip_dn10;
        *var_sqrtpsip_dn11_slot = var_sqrtpsip_dn11;
        *var_sqrtpsip_dn3_slot = var_sqrtpsip_dn3;
        *var_sqrtpsip_dn4_slot = var_sqrtpsip_dn4;
        *var_sqrtpsip_dn5_slot = var_sqrtpsip_dn5;
        *var_sqrtpsip_dn6_slot = var_sqrtpsip_dn6;
        *var_sqrtpsip_dn7_slot = var_sqrtpsip_dn7;
        *var_sqrtpsip_dn8_slot = var_sqrtpsip_dn8;
        *var_sqrtpsip_dn9_slot = var_sqrtpsip_dn9;
        *var_sqrtpsip_rv_slot = var_sqrtpsip_rv;
        *var_sqrtpsisa_slot = var_sqrtpsisa;
        *var_sqrtpsisa_dn10_slot = var_sqrtpsisa_dn10;
        *var_sqrtpsisa_dn11_slot = var_sqrtpsisa_dn11;
        *var_sqrtpsisa_dn3_slot = var_sqrtpsisa_dn3;
        *var_sqrtpsisa_dn4_slot = var_sqrtpsisa_dn4;
        *var_sqrtpsisa_dn5_slot = var_sqrtpsisa_dn5;
        *var_sqrtpsisa_dn6_slot = var_sqrtpsisa_dn6;
        *var_sqrtpsisa_dn7_slot = var_sqrtpsisa_dn7;
        *var_sqrtpsisa_dn8_slot = var_sqrtpsisa_dn8;
        *var_sqrtpsisa_dn9_slot = var_sqrtpsisa_dn9;
        *var_sqrtpsisa_rv_slot = var_sqrtpsisa_rv;
        *var_sqrtpsisainv_slot = var_sqrtpsisainv;
        *var_sqrtpsisainv_dn10_slot = var_sqrtpsisainv_dn10;
        *var_sqrtpsisainv_dn11_slot = var_sqrtpsisainv_dn11;
        *var_sqrtpsisainv_dn3_slot = var_sqrtpsisainv_dn3;
        *var_sqrtpsisainv_dn4_slot = var_sqrtpsisainv_dn4;
        *var_sqrtpsisainv_dn5_slot = var_sqrtpsisainv_dn5;
        *var_sqrtpsisainv_dn6_slot = var_sqrtpsisainv_dn6;
        *var_sqrtpsisainv_dn7_slot = var_sqrtpsisainv_dn7;
        *var_sqrtpsisainv_dn8_slot = var_sqrtpsisainv_dn8;
        *var_sqrtpsisainv_dn9_slot = var_sqrtpsisainv_dn9;
        *var_sqrtpsisainv_rv_slot = var_sqrtpsisainv_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_t0_rv_slot = var_t0_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t1_rv_slot = var_t1_rv;
        *var_t2_slot = var_t2;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_t2_rv_slot = var_t2_rv;
        *var_t3_slot = var_t3;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn3_slot = var_t3_dn3;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_dn9_slot = var_t3_dn9;
        *var_t3_rv_slot = var_t3_rv;
        *var_t4_slot = var_t4;
        *var_t4_dn10_slot = var_t4_dn10;
        *var_t4_dn11_slot = var_t4_dn11;
        *var_t4_dn3_slot = var_t4_dn3;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t4_dn9_slot = var_t4_dn9;
        *var_t4_rv_slot = var_t4_rv;
        *var_t5_slot = var_t5;
        *var_t5_dn10_slot = var_t5_dn10;
        *var_t5_dn11_slot = var_t5_dn11;
        *var_t5_dn3_slot = var_t5_dn3;
        *var_t5_dn4_slot = var_t5_dn4;
        *var_t5_dn5_slot = var_t5_dn5;
        *var_t5_dn6_slot = var_t5_dn6;
        *var_t5_dn7_slot = var_t5_dn7;
        *var_t5_dn8_slot = var_t5_dn8;
        *var_t5_dn9_slot = var_t5_dn9;
        *var_t5_rv_slot = var_t5_rv;
        *var_t6_slot = var_t6;
        *var_t6_dn10_slot = var_t6_dn10;
        *var_t6_dn11_slot = var_t6_dn11;
        *var_t6_dn3_slot = var_t6_dn3;
        *var_t6_dn4_slot = var_t6_dn4;
        *var_t6_dn5_slot = var_t6_dn5;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn7_slot = var_t6_dn7;
        *var_t6_dn8_slot = var_t6_dn8;
        *var_t6_dn9_slot = var_t6_dn9;
        *var_t6_rv_slot = var_t6_rv;
        *var_t8_slot = var_t8;
        *var_t8_dn10_slot = var_t8_dn10;
        *var_t8_dn11_slot = var_t8_dn11;
        *var_t8_dn3_slot = var_t8_dn3;
        *var_t8_dn4_slot = var_t8_dn4;
        *var_t8_dn5_slot = var_t8_dn5;
        *var_t8_dn6_slot = var_t8_dn6;
        *var_t8_dn7_slot = var_t8_dn7;
        *var_t8_dn8_slot = var_t8_dn8;
        *var_t8_dn9_slot = var_t8_dn9;
        *var_t8_rv_slot = var_t8_rv;
    }

    pub(super) fn stamp_reactive_block_209(
        var_delta_t: f64,
        var_delta_t_dn10: f64,
        var_delta_t_dn11: f64,
        var_delta_t_dn3: f64,
        var_delta_t_dn4: f64,
        var_delta_t_dn5: f64,
        var_delta_t_dn6: f64,
        var_delta_t_dn7: f64,
        var_delta_t_dn8: f64,
        var_delta_t_dn9: f64,
        var_gam_edge: f64,
        var_gam_edge_dn10: f64,
        var_gam_edge_dn11: f64,
        var_gam_edge_dn3: f64,
        var_gam_edge_dn4: f64,
        var_gam_edge_dn5: f64,
        var_gam_edge_dn6: f64,
        var_gam_edge_dn7: f64,
        var_gam_edge_dn8: f64,
        var_gam_edge_dn9: f64,
        var_guard492: f64,
        var_guard867: f64,
        var_guard871: f64,
        var_inv_nvt: f64,
        var_inv_nvt_dn10: f64,
        var_inv_nvt_dn11: f64,
        var_inv_nvt_dn3: f64,
        var_inv_nvt_dn4: f64,
        var_inv_nvt_dn5: f64,
        var_inv_nvt_dn6: f64,
        var_inv_nvt_dn7: f64,
        var_inv_nvt_dn8: f64,
        var_inv_nvt_dn9: f64,
        var_nvt: f64,
        var_nvt_dn10: f64,
        var_nvt_dn11: f64,
        var_nvt_dn3: f64,
        var_nvt_dn4: f64,
        var_nvt_dn5: f64,
        var_nvt_dn6: f64,
        var_nvt_dn7: f64,
        var_nvt_dn8: f64,
        var_nvt_dn9: f64,
        var_phib_n_edge: f64,
        var_phib_n_edge_dn10: f64,
        var_phib_n_edge_dn11: f64,
        var_phib_n_edge_dn3: f64,
        var_phib_n_edge_dn4: f64,
        var_phib_n_edge_dn5: f64,
        var_phib_n_edge_dn6: f64,
        var_phib_n_edge_dn7: f64,
        var_phib_n_edge_dn8: f64,
        var_phib_n_edge_dn9: f64,
        var_psip: f64,
        var_psip_dn10: f64,
        var_psip_dn11: f64,
        var_psip_dn3: f64,
        var_psip_dn4: f64,
        var_psip_dn5: f64,
        var_psip_dn6: f64,
        var_psip_dn7: f64,
        var_psip_dn8: f64,
        var_psip_dn9: f64,
        var_sqrtpsisainv: f64,
        var_sqrtpsisainv_dn10: f64,
        var_sqrtpsisainv_dn11: f64,
        var_sqrtpsisainv_dn3: f64,
        var_sqrtpsisainv_dn4: f64,
        var_sqrtpsisainv_dn5: f64,
        var_sqrtpsisainv_dn6: f64,
        var_sqrtpsisainv_dn7: f64,
        var_sqrtpsisainv_dn8: f64,
        var_sqrtpsisainv_dn9: f64,
        var_t3: f64,
        var_t3_dn10: f64,
        var_t3_dn11: f64,
        var_t3_dn3: f64,
        var_t3_dn4: f64,
        var_t3_dn5: f64,
        var_t3_dn6: f64,
        var_t3_dn7: f64,
        var_t3_dn8: f64,
        var_t3_dn9: f64,
        var_vds: f64,
        var_vds_dn10: f64,
        var_vds_dn6: f64,
        var_vds_dn7: f64,
        var_vs: f64,
        var_vs_dn10: f64,
        var_vs_dn6: f64,
        var_vs_dn7: f64,
        var_guard874_slot: &mut f64,
        var_guard874_rv_slot: &mut f64,
        var_guard875_slot: &mut f64,
        var_guard875_rv_slot: &mut f64,
        var_qs_edge_slot: &mut f64,
        var_qs_edge_dn10_slot: &mut f64,
        var_qs_edge_dn11_slot: &mut f64,
        var_qs_edge_dn3_slot: &mut f64,
        var_qs_edge_dn4_slot: &mut f64,
        var_qs_edge_dn5_slot: &mut f64,
        var_qs_edge_dn6_slot: &mut f64,
        var_qs_edge_dn7_slot: &mut f64,
        var_qs_edge_dn8_slot: &mut f64,
        var_qs_edge_dn9_slot: &mut f64,
        var_qs_edge_rv_slot: &mut f64,
        var_sqrtpsip_slot: &mut f64,
        var_sqrtpsip_dn10_slot: &mut f64,
        var_sqrtpsip_dn11_slot: &mut f64,
        var_sqrtpsip_dn3_slot: &mut f64,
        var_sqrtpsip_dn4_slot: &mut f64,
        var_sqrtpsip_dn5_slot: &mut f64,
        var_sqrtpsip_dn6_slot: &mut f64,
        var_sqrtpsip_dn7_slot: &mut f64,
        var_sqrtpsip_dn8_slot: &mut f64,
        var_sqrtpsip_dn9_slot: &mut f64,
        var_sqrtpsip_rv_slot: &mut f64,
        var_sqrtpsisa_slot: &mut f64,
        var_sqrtpsisa_dn10_slot: &mut f64,
        var_sqrtpsisa_dn11_slot: &mut f64,
        var_sqrtpsisa_dn3_slot: &mut f64,
        var_sqrtpsisa_dn4_slot: &mut f64,
        var_sqrtpsisa_dn5_slot: &mut f64,
        var_sqrtpsisa_dn6_slot: &mut f64,
        var_sqrtpsisa_dn7_slot: &mut f64,
        var_sqrtpsisa_dn8_slot: &mut f64,
        var_sqrtpsisa_dn9_slot: &mut f64,
        var_sqrtpsisa_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn10_slot: &mut f64,
        var_t4_dn11_slot: &mut f64,
        var_t4_dn3_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t4_dn9_slot: &mut f64,
        var_t4_rv_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_dn10_slot: &mut f64,
        var_t5_dn11_slot: &mut f64,
        var_t5_dn3_slot: &mut f64,
        var_t5_dn4_slot: &mut f64,
        var_t5_dn5_slot: &mut f64,
        var_t5_dn6_slot: &mut f64,
        var_t5_dn7_slot: &mut f64,
        var_t5_dn8_slot: &mut f64,
        var_t5_dn9_slot: &mut f64,
        var_t5_rv_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_dn10_slot: &mut f64,
        var_t6_dn11_slot: &mut f64,
        var_t6_dn3_slot: &mut f64,
        var_t6_dn4_slot: &mut f64,
        var_t6_dn5_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn7_slot: &mut f64,
        var_t6_dn8_slot: &mut f64,
        var_t6_dn9_slot: &mut f64,
        var_t6_rv_slot: &mut f64,
        var_t7_slot: &mut f64,
        var_t7_dn10_slot: &mut f64,
        var_t7_dn11_slot: &mut f64,
        var_t7_dn3_slot: &mut f64,
        var_t7_dn4_slot: &mut f64,
        var_t7_dn5_slot: &mut f64,
        var_t7_dn6_slot: &mut f64,
        var_t7_dn7_slot: &mut f64,
        var_t7_dn8_slot: &mut f64,
        var_t7_dn9_slot: &mut f64,
        var_t7_rv_slot: &mut f64,
        var_t8_slot: &mut f64,
        var_t8_dn10_slot: &mut f64,
        var_t8_dn11_slot: &mut f64,
        var_t8_dn3_slot: &mut f64,
        var_t8_dn4_slot: &mut f64,
        var_t8_dn5_slot: &mut f64,
        var_t8_dn6_slot: &mut f64,
        var_t8_dn7_slot: &mut f64,
        var_t8_dn8_slot: &mut f64,
        var_t8_dn9_slot: &mut f64,
        var_t8_rv_slot: &mut f64,
        var_vdeff_slot: &mut f64,
        var_vdeff_dn10_slot: &mut f64,
        var_vdeff_dn11_slot: &mut f64,
        var_vdeff_dn3_slot: &mut f64,
        var_vdeff_dn4_slot: &mut f64,
        var_vdeff_dn5_slot: &mut f64,
        var_vdeff_dn6_slot: &mut f64,
        var_vdeff_dn7_slot: &mut f64,
        var_vdeff_dn8_slot: &mut f64,
        var_vdeff_dn9_slot: &mut f64,
        var_vdeff_rv_slot: &mut f64,
        var_vdsatedge_slot: &mut f64,
        var_vdsatedge_1_slot: &mut f64,
        var_vdsatedge_1_dn10_slot: &mut f64,
        var_vdsatedge_1_dn11_slot: &mut f64,
        var_vdsatedge_1_dn3_slot: &mut f64,
        var_vdsatedge_1_dn4_slot: &mut f64,
        var_vdsatedge_1_dn5_slot: &mut f64,
        var_vdsatedge_1_dn6_slot: &mut f64,
        var_vdsatedge_1_dn7_slot: &mut f64,
        var_vdsatedge_1_dn8_slot: &mut f64,
        var_vdsatedge_1_dn9_slot: &mut f64,
        var_vdsatedge_1_rv_slot: &mut f64,
        var_vdsatedge_dn10_slot: &mut f64,
        var_vdsatedge_dn11_slot: &mut f64,
        var_vdsatedge_dn3_slot: &mut f64,
        var_vdsatedge_dn4_slot: &mut f64,
        var_vdsatedge_dn5_slot: &mut f64,
        var_vdsatedge_dn6_slot: &mut f64,
        var_vdsatedge_dn7_slot: &mut f64,
        var_vdsatedge_dn8_slot: &mut f64,
        var_vdsatedge_dn9_slot: &mut f64,
        var_vdsatedge_rv_slot: &mut f64,
        var_vdseff_slot: &mut f64,
        var_vdseff_dn10_slot: &mut f64,
        var_vdseff_dn11_slot: &mut f64,
        var_vdseff_dn3_slot: &mut f64,
        var_vdseff_dn4_slot: &mut f64,
        var_vdseff_dn5_slot: &mut f64,
        var_vdseff_dn6_slot: &mut f64,
        var_vdseff_dn7_slot: &mut f64,
        var_vdseff_dn8_slot: &mut f64,
        var_vdseff_dn9_slot: &mut f64,
        var_vdseff_rv_slot: &mut f64,
        var_vdssate_slot: &mut f64,
        var_vdssate_dn10_slot: &mut f64,
        var_vdssate_dn11_slot: &mut f64,
        var_vdssate_dn3_slot: &mut f64,
        var_vdssate_dn4_slot: &mut f64,
        var_vdssate_dn5_slot: &mut f64,
        var_vdssate_dn6_slot: &mut f64,
        var_vdssate_dn7_slot: &mut f64,
        var_vdssate_dn8_slot: &mut f64,
        var_vdssate_dn9_slot: &mut f64,
        var_vdssate_rv_slot: &mut f64,
    ) {
        let mut var_guard874: f64 = *var_guard874_slot;
        let mut var_guard874_rv: f64 = *var_guard874_rv_slot;
        let mut var_guard875: f64 = *var_guard875_slot;
        let mut var_guard875_rv: f64 = *var_guard875_rv_slot;
        let mut var_qs_edge: f64 = *var_qs_edge_slot;
        let mut var_qs_edge_dn10: f64 = *var_qs_edge_dn10_slot;
        let mut var_qs_edge_dn11: f64 = *var_qs_edge_dn11_slot;
        let mut var_qs_edge_dn3: f64 = *var_qs_edge_dn3_slot;
        let mut var_qs_edge_dn4: f64 = *var_qs_edge_dn4_slot;
        let mut var_qs_edge_dn5: f64 = *var_qs_edge_dn5_slot;
        let mut var_qs_edge_dn6: f64 = *var_qs_edge_dn6_slot;
        let mut var_qs_edge_dn7: f64 = *var_qs_edge_dn7_slot;
        let mut var_qs_edge_dn8: f64 = *var_qs_edge_dn8_slot;
        let mut var_qs_edge_dn9: f64 = *var_qs_edge_dn9_slot;
        let mut var_qs_edge_rv: f64 = *var_qs_edge_rv_slot;
        let mut var_sqrtpsip: f64 = *var_sqrtpsip_slot;
        let mut var_sqrtpsip_dn10: f64 = *var_sqrtpsip_dn10_slot;
        let mut var_sqrtpsip_dn11: f64 = *var_sqrtpsip_dn11_slot;
        let mut var_sqrtpsip_dn3: f64 = *var_sqrtpsip_dn3_slot;
        let mut var_sqrtpsip_dn4: f64 = *var_sqrtpsip_dn4_slot;
        let mut var_sqrtpsip_dn5: f64 = *var_sqrtpsip_dn5_slot;
        let mut var_sqrtpsip_dn6: f64 = *var_sqrtpsip_dn6_slot;
        let mut var_sqrtpsip_dn7: f64 = *var_sqrtpsip_dn7_slot;
        let mut var_sqrtpsip_dn8: f64 = *var_sqrtpsip_dn8_slot;
        let mut var_sqrtpsip_dn9: f64 = *var_sqrtpsip_dn9_slot;
        let mut var_sqrtpsip_rv: f64 = *var_sqrtpsip_rv_slot;
        let mut var_sqrtpsisa: f64 = *var_sqrtpsisa_slot;
        let mut var_sqrtpsisa_dn10: f64 = *var_sqrtpsisa_dn10_slot;
        let mut var_sqrtpsisa_dn11: f64 = *var_sqrtpsisa_dn11_slot;
        let mut var_sqrtpsisa_dn3: f64 = *var_sqrtpsisa_dn3_slot;
        let mut var_sqrtpsisa_dn4: f64 = *var_sqrtpsisa_dn4_slot;
        let mut var_sqrtpsisa_dn5: f64 = *var_sqrtpsisa_dn5_slot;
        let mut var_sqrtpsisa_dn6: f64 = *var_sqrtpsisa_dn6_slot;
        let mut var_sqrtpsisa_dn7: f64 = *var_sqrtpsisa_dn7_slot;
        let mut var_sqrtpsisa_dn8: f64 = *var_sqrtpsisa_dn8_slot;
        let mut var_sqrtpsisa_dn9: f64 = *var_sqrtpsisa_dn9_slot;
        let mut var_sqrtpsisa_rv: f64 = *var_sqrtpsisa_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn10: f64 = *var_t4_dn10_slot;
        let mut var_t4_dn11: f64 = *var_t4_dn11_slot;
        let mut var_t4_dn3: f64 = *var_t4_dn3_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t4_dn9: f64 = *var_t4_dn9_slot;
        let mut var_t4_rv: f64 = *var_t4_rv_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_dn10: f64 = *var_t5_dn10_slot;
        let mut var_t5_dn11: f64 = *var_t5_dn11_slot;
        let mut var_t5_dn3: f64 = *var_t5_dn3_slot;
        let mut var_t5_dn4: f64 = *var_t5_dn4_slot;
        let mut var_t5_dn5: f64 = *var_t5_dn5_slot;
        let mut var_t5_dn6: f64 = *var_t5_dn6_slot;
        let mut var_t5_dn7: f64 = *var_t5_dn7_slot;
        let mut var_t5_dn8: f64 = *var_t5_dn8_slot;
        let mut var_t5_dn9: f64 = *var_t5_dn9_slot;
        let mut var_t5_rv: f64 = *var_t5_rv_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_dn10: f64 = *var_t6_dn10_slot;
        let mut var_t6_dn11: f64 = *var_t6_dn11_slot;
        let mut var_t6_dn3: f64 = *var_t6_dn3_slot;
        let mut var_t6_dn4: f64 = *var_t6_dn4_slot;
        let mut var_t6_dn5: f64 = *var_t6_dn5_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn7: f64 = *var_t6_dn7_slot;
        let mut var_t6_dn8: f64 = *var_t6_dn8_slot;
        let mut var_t6_dn9: f64 = *var_t6_dn9_slot;
        let mut var_t6_rv: f64 = *var_t6_rv_slot;
        let mut var_t7: f64 = *var_t7_slot;
        let mut var_t7_dn10: f64 = *var_t7_dn10_slot;
        let mut var_t7_dn11: f64 = *var_t7_dn11_slot;
        let mut var_t7_dn3: f64 = *var_t7_dn3_slot;
        let mut var_t7_dn4: f64 = *var_t7_dn4_slot;
        let mut var_t7_dn5: f64 = *var_t7_dn5_slot;
        let mut var_t7_dn6: f64 = *var_t7_dn6_slot;
        let mut var_t7_dn7: f64 = *var_t7_dn7_slot;
        let mut var_t7_dn8: f64 = *var_t7_dn8_slot;
        let mut var_t7_dn9: f64 = *var_t7_dn9_slot;
        let mut var_t7_rv: f64 = *var_t7_rv_slot;
        let mut var_t8: f64 = *var_t8_slot;
        let mut var_t8_dn10: f64 = *var_t8_dn10_slot;
        let mut var_t8_dn11: f64 = *var_t8_dn11_slot;
        let mut var_t8_dn3: f64 = *var_t8_dn3_slot;
        let mut var_t8_dn4: f64 = *var_t8_dn4_slot;
        let mut var_t8_dn5: f64 = *var_t8_dn5_slot;
        let mut var_t8_dn6: f64 = *var_t8_dn6_slot;
        let mut var_t8_dn7: f64 = *var_t8_dn7_slot;
        let mut var_t8_dn8: f64 = *var_t8_dn8_slot;
        let mut var_t8_dn9: f64 = *var_t8_dn9_slot;
        let mut var_t8_rv: f64 = *var_t8_rv_slot;
        let mut var_vdeff: f64 = *var_vdeff_slot;
        let mut var_vdeff_dn10: f64 = *var_vdeff_dn10_slot;
        let mut var_vdeff_dn11: f64 = *var_vdeff_dn11_slot;
        let mut var_vdeff_dn3: f64 = *var_vdeff_dn3_slot;
        let mut var_vdeff_dn4: f64 = *var_vdeff_dn4_slot;
        let mut var_vdeff_dn5: f64 = *var_vdeff_dn5_slot;
        let mut var_vdeff_dn6: f64 = *var_vdeff_dn6_slot;
        let mut var_vdeff_dn7: f64 = *var_vdeff_dn7_slot;
        let mut var_vdeff_dn8: f64 = *var_vdeff_dn8_slot;
        let mut var_vdeff_dn9: f64 = *var_vdeff_dn9_slot;
        let mut var_vdeff_rv: f64 = *var_vdeff_rv_slot;
        let mut var_vdsatedge: f64 = *var_vdsatedge_slot;
        let mut var_vdsatedge_1: f64 = *var_vdsatedge_1_slot;
        let mut var_vdsatedge_1_dn10: f64 = *var_vdsatedge_1_dn10_slot;
        let mut var_vdsatedge_1_dn11: f64 = *var_vdsatedge_1_dn11_slot;
        let mut var_vdsatedge_1_dn3: f64 = *var_vdsatedge_1_dn3_slot;
        let mut var_vdsatedge_1_dn4: f64 = *var_vdsatedge_1_dn4_slot;
        let mut var_vdsatedge_1_dn5: f64 = *var_vdsatedge_1_dn5_slot;
        let mut var_vdsatedge_1_dn6: f64 = *var_vdsatedge_1_dn6_slot;
        let mut var_vdsatedge_1_dn7: f64 = *var_vdsatedge_1_dn7_slot;
        let mut var_vdsatedge_1_dn8: f64 = *var_vdsatedge_1_dn8_slot;
        let mut var_vdsatedge_1_dn9: f64 = *var_vdsatedge_1_dn9_slot;
        let mut var_vdsatedge_1_rv: f64 = *var_vdsatedge_1_rv_slot;
        let mut var_vdsatedge_dn10: f64 = *var_vdsatedge_dn10_slot;
        let mut var_vdsatedge_dn11: f64 = *var_vdsatedge_dn11_slot;
        let mut var_vdsatedge_dn3: f64 = *var_vdsatedge_dn3_slot;
        let mut var_vdsatedge_dn4: f64 = *var_vdsatedge_dn4_slot;
        let mut var_vdsatedge_dn5: f64 = *var_vdsatedge_dn5_slot;
        let mut var_vdsatedge_dn6: f64 = *var_vdsatedge_dn6_slot;
        let mut var_vdsatedge_dn7: f64 = *var_vdsatedge_dn7_slot;
        let mut var_vdsatedge_dn8: f64 = *var_vdsatedge_dn8_slot;
        let mut var_vdsatedge_dn9: f64 = *var_vdsatedge_dn9_slot;
        let mut var_vdsatedge_rv: f64 = *var_vdsatedge_rv_slot;
        let mut var_vdseff: f64 = *var_vdseff_slot;
        let mut var_vdseff_dn10: f64 = *var_vdseff_dn10_slot;
        let mut var_vdseff_dn11: f64 = *var_vdseff_dn11_slot;
        let mut var_vdseff_dn3: f64 = *var_vdseff_dn3_slot;
        let mut var_vdseff_dn4: f64 = *var_vdseff_dn4_slot;
        let mut var_vdseff_dn5: f64 = *var_vdseff_dn5_slot;
        let mut var_vdseff_dn6: f64 = *var_vdseff_dn6_slot;
        let mut var_vdseff_dn7: f64 = *var_vdseff_dn7_slot;
        let mut var_vdseff_dn8: f64 = *var_vdseff_dn8_slot;
        let mut var_vdseff_dn9: f64 = *var_vdseff_dn9_slot;
        let mut var_vdseff_rv: f64 = *var_vdseff_rv_slot;
        let mut var_vdssate: f64 = *var_vdssate_slot;
        let mut var_vdssate_dn10: f64 = *var_vdssate_dn10_slot;
        let mut var_vdssate_dn11: f64 = *var_vdssate_dn11_slot;
        let mut var_vdssate_dn3: f64 = *var_vdssate_dn3_slot;
        let mut var_vdssate_dn4: f64 = *var_vdssate_dn4_slot;
        let mut var_vdssate_dn5: f64 = *var_vdssate_dn5_slot;
        let mut var_vdssate_dn6: f64 = *var_vdssate_dn6_slot;
        let mut var_vdssate_dn7: f64 = *var_vdssate_dn7_slot;
        let mut var_vdssate_dn8: f64 = *var_vdssate_dn8_slot;
        let mut var_vdssate_dn9: f64 = *var_vdssate_dn9_slot;
        let mut var_vdssate_rv: f64 = *var_vdssate_rv_slot;

        let (assign59780_e97070, assign59780_e97070_d_n3, assign59780_e97070_d_n4, assign59780_e97070_d_n5, assign59780_e97070_d_n6, assign59780_e97070_d_n7, assign59780_e97070_d_n8, assign59780_e97070_d_n9, assign59780_e97070_d_n10, assign59780_e97070_d_n11,) = {
    if (((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard871 == 0.0)) {
        let assign59780_e97057: f64 = (1.0 / var_t3);
        let assign59780_e97058: f64 = (2.0 + assign59780_e97057);
        let assign59780_e97061: f64 = (var_t0 + var_sqrtpsisainv);
        let assign59780_e97064: f64 = (var_t0 * var_t3);
        let assign59780_e97066: f64 = (assign59780_e97064 + var_sqrtpsisa);
        let assign59780_e97067: f64 = (assign59780_e97061 / assign59780_e97066);
        let assign59780_e97068: f64 = (assign59780_e97058 + assign59780_e97067);
        (assign59780_e97068, ((-(var_t3_dn3 / (var_t3 * var_t3))) + ((((var_t0_dn3 + var_sqrtpsisainv_dn3) * assign59780_e97066) - (assign59780_e97061 * (((var_t0_dn3 * var_t3) + (var_t0 * var_t3_dn3)) + var_sqrtpsisa_dn3))) / (assign59780_e97066 * assign59780_e97066))), ((-(var_t3_dn4 / (var_t3 * var_t3))) + ((((var_t0_dn4 + var_sqrtpsisainv_dn4) * assign59780_e97066) - (assign59780_e97061 * (((var_t0_dn4 * var_t3) + (var_t0 * var_t3_dn4)) + var_sqrtpsisa_dn4))) / (assign59780_e97066 * assign59780_e97066))), ((-(var_t3_dn5 / (var_t3 * var_t3))) + ((((var_t0_dn5 + var_sqrtpsisainv_dn5) * assign59780_e97066) - (assign59780_e97061 * (((var_t0_dn5 * var_t3) + (var_t0 * var_t3_dn5)) + var_sqrtpsisa_dn5))) / (assign59780_e97066 * assign59780_e97066))), ((-(var_t3_dn6 / (var_t3 * var_t3))) + ((((var_t0_dn6 + var_sqrtpsisainv_dn6) * assign59780_e97066) - (assign59780_e97061 * (((var_t0_dn6 * var_t3) + (var_t0 * var_t3_dn6)) + var_sqrtpsisa_dn6))) / (assign59780_e97066 * assign59780_e97066))), ((-(var_t3_dn7 / (var_t3 * var_t3))) + ((((var_t0_dn7 + var_sqrtpsisainv_dn7) * assign59780_e97066) - (assign59780_e97061 * (((var_t0_dn7 * var_t3) + (var_t0 * var_t3_dn7)) + var_sqrtpsisa_dn7))) / (assign59780_e97066 * assign59780_e97066))), ((-(var_t3_dn8 / (var_t3 * var_t3))) + ((((var_t0_dn8 + var_sqrtpsisainv_dn8) * assign59780_e97066) - (assign59780_e97061 * (((var_t0_dn8 * var_t3) + (var_t0 * var_t3_dn8)) + var_sqrtpsisa_dn8))) / (assign59780_e97066 * assign59780_e97066))), ((-(var_t3_dn9 / (var_t3 * var_t3))) + ((((var_t0_dn9 + var_sqrtpsisainv_dn9) * assign59780_e97066) - (assign59780_e97061 * (((var_t0_dn9 * var_t3) + (var_t0 * var_t3_dn9)) + var_sqrtpsisa_dn9))) / (assign59780_e97066 * assign59780_e97066))), ((-(var_t3_dn10 / (var_t3 * var_t3))) + ((((var_t0_dn10 + var_sqrtpsisainv_dn10) * assign59780_e97066) - (assign59780_e97061 * (((var_t0_dn10 * var_t3) + (var_t0 * var_t3_dn10)) + var_sqrtpsisa_dn10))) / (assign59780_e97066 * assign59780_e97066))), ((-(var_t3_dn11 / (var_t3 * var_t3))) + ((((var_t0_dn11 + var_sqrtpsisainv_dn11) * assign59780_e97066) - (assign59780_e97061 * (((var_t0_dn11 * var_t3) + (var_t0 * var_t3_dn11)) + var_sqrtpsisa_dn11))) / (assign59780_e97066 * assign59780_e97066))),)
    } else {
        (var_t5, var_t5_dn3, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8, var_t5_dn9, var_t5_dn10, var_t5_dn11,)
    }
};
        var_t5 = assign59780_e97070;
        var_t5_dn3 = assign59780_e97070_d_n3;
        var_t5_dn4 = assign59780_e97070_d_n4;
        var_t5_dn5 = assign59780_e97070_d_n5;
        var_t5_dn6 = assign59780_e97070_d_n6;
        var_t5_dn7 = assign59780_e97070_d_n7;
        var_t5_dn8 = assign59780_e97070_d_n8;
        var_t5_dn9 = assign59780_e97070_d_n9;
        var_t5_dn10 = assign59780_e97070_d_n10;
        var_t5_dn11 = assign59780_e97070_d_n11;
        var_t5_rv = 0.0;

        let (assign59790_e97098, assign59790_e97098_d_n3, assign59790_e97098_d_n4, assign59790_e97098_d_n5, assign59790_e97098_d_n6, assign59790_e97098_d_n7, assign59790_e97098_d_n8, assign59790_e97098_d_n9, assign59790_e97098_d_n10, assign59790_e97098_d_n11,) = {
    if (((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard871 == 0.0)) {
        let assign59790_e97080: f64 = (var_t0 + var_sqrtpsisainv);
        let assign59790_e97083: f64 = (var_t0 * var_t3);
        let assign59790_e97085: f64 = (assign59790_e97083 + var_sqrtpsisa);
        let assign59790_e97086: f64 = (assign59790_e97080 / assign59790_e97085);
        let assign59790_e97089: f64 = (var_t0 + var_sqrtpsisainv);
        let assign59790_e97092: f64 = (var_t0 * var_t3);
        let assign59790_e97094: f64 = (assign59790_e97092 + var_sqrtpsisa);
        let assign59790_e97095: f64 = (assign59790_e97089 / assign59790_e97094);
        let assign59790_e97096: f64 = (assign59790_e97086 * assign59790_e97095);
        (assign59790_e97096, ((((((var_t0_dn3 + var_sqrtpsisainv_dn3) * assign59790_e97085) - (assign59790_e97080 * (((var_t0_dn3 * var_t3) + (var_t0 * var_t3_dn3)) + var_sqrtpsisa_dn3))) / (assign59790_e97085 * assign59790_e97085)) * assign59790_e97095) + (assign59790_e97086 * ((((var_t0_dn3 + var_sqrtpsisainv_dn3) * assign59790_e97094) - (assign59790_e97089 * (((var_t0_dn3 * var_t3) + (var_t0 * var_t3_dn3)) + var_sqrtpsisa_dn3))) / (assign59790_e97094 * assign59790_e97094)))), ((((((var_t0_dn4 + var_sqrtpsisainv_dn4) * assign59790_e97085) - (assign59790_e97080 * (((var_t0_dn4 * var_t3) + (var_t0 * var_t3_dn4)) + var_sqrtpsisa_dn4))) / (assign59790_e97085 * assign59790_e97085)) * assign59790_e97095) + (assign59790_e97086 * ((((var_t0_dn4 + var_sqrtpsisainv_dn4) * assign59790_e97094) - (assign59790_e97089 * (((var_t0_dn4 * var_t3) + (var_t0 * var_t3_dn4)) + var_sqrtpsisa_dn4))) / (assign59790_e97094 * assign59790_e97094)))), ((((((var_t0_dn5 + var_sqrtpsisainv_dn5) * assign59790_e97085) - (assign59790_e97080 * (((var_t0_dn5 * var_t3) + (var_t0 * var_t3_dn5)) + var_sqrtpsisa_dn5))) / (assign59790_e97085 * assign59790_e97085)) * assign59790_e97095) + (assign59790_e97086 * ((((var_t0_dn5 + var_sqrtpsisainv_dn5) * assign59790_e97094) - (assign59790_e97089 * (((var_t0_dn5 * var_t3) + (var_t0 * var_t3_dn5)) + var_sqrtpsisa_dn5))) / (assign59790_e97094 * assign59790_e97094)))), ((((((var_t0_dn6 + var_sqrtpsisainv_dn6) * assign59790_e97085) - (assign59790_e97080 * (((var_t0_dn6 * var_t3) + (var_t0 * var_t3_dn6)) + var_sqrtpsisa_dn6))) / (assign59790_e97085 * assign59790_e97085)) * assign59790_e97095) + (assign59790_e97086 * ((((var_t0_dn6 + var_sqrtpsisainv_dn6) * assign59790_e97094) - (assign59790_e97089 * (((var_t0_dn6 * var_t3) + (var_t0 * var_t3_dn6)) + var_sqrtpsisa_dn6))) / (assign59790_e97094 * assign59790_e97094)))), ((((((var_t0_dn7 + var_sqrtpsisainv_dn7) * assign59790_e97085) - (assign59790_e97080 * (((var_t0_dn7 * var_t3) + (var_t0 * var_t3_dn7)) + var_sqrtpsisa_dn7))) / (assign59790_e97085 * assign59790_e97085)) * assign59790_e97095) + (assign59790_e97086 * ((((var_t0_dn7 + var_sqrtpsisainv_dn7) * assign59790_e97094) - (assign59790_e97089 * (((var_t0_dn7 * var_t3) + (var_t0 * var_t3_dn7)) + var_sqrtpsisa_dn7))) / (assign59790_e97094 * assign59790_e97094)))), ((((((var_t0_dn8 + var_sqrtpsisainv_dn8) * assign59790_e97085) - (assign59790_e97080 * (((var_t0_dn8 * var_t3) + (var_t0 * var_t3_dn8)) + var_sqrtpsisa_dn8))) / (assign59790_e97085 * assign59790_e97085)) * assign59790_e97095) + (assign59790_e97086 * ((((var_t0_dn8 + var_sqrtpsisainv_dn8) * assign59790_e97094) - (assign59790_e97089 * (((var_t0_dn8 * var_t3) + (var_t0 * var_t3_dn8)) + var_sqrtpsisa_dn8))) / (assign59790_e97094 * assign59790_e97094)))), ((((((var_t0_dn9 + var_sqrtpsisainv_dn9) * assign59790_e97085) - (assign59790_e97080 * (((var_t0_dn9 * var_t3) + (var_t0 * var_t3_dn9)) + var_sqrtpsisa_dn9))) / (assign59790_e97085 * assign59790_e97085)) * assign59790_e97095) + (assign59790_e97086 * ((((var_t0_dn9 + var_sqrtpsisainv_dn9) * assign59790_e97094) - (assign59790_e97089 * (((var_t0_dn9 * var_t3) + (var_t0 * var_t3_dn9)) + var_sqrtpsisa_dn9))) / (assign59790_e97094 * assign59790_e97094)))), ((((((var_t0_dn10 + var_sqrtpsisainv_dn10) * assign59790_e97085) - (assign59790_e97080 * (((var_t0_dn10 * var_t3) + (var_t0 * var_t3_dn10)) + var_sqrtpsisa_dn10))) / (assign59790_e97085 * assign59790_e97085)) * assign59790_e97095) + (assign59790_e97086 * ((((var_t0_dn10 + var_sqrtpsisainv_dn10) * assign59790_e97094) - (assign59790_e97089 * (((var_t0_dn10 * var_t3) + (var_t0 * var_t3_dn10)) + var_sqrtpsisa_dn10))) / (assign59790_e97094 * assign59790_e97094)))), ((((((var_t0_dn11 + var_sqrtpsisainv_dn11) * assign59790_e97085) - (assign59790_e97080 * (((var_t0_dn11 * var_t3) + (var_t0 * var_t3_dn11)) + var_sqrtpsisa_dn11))) / (assign59790_e97085 * assign59790_e97085)) * assign59790_e97095) + (assign59790_e97086 * ((((var_t0_dn11 + var_sqrtpsisainv_dn11) * assign59790_e97094) - (assign59790_e97089 * (((var_t0_dn11 * var_t3) + (var_t0 * var_t3_dn11)) + var_sqrtpsisa_dn11))) / (assign59790_e97094 * assign59790_e97094)))),)
    } else {
        (var_t6, var_t6_dn3, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_dn9, var_t6_dn10, var_t6_dn11,)
    }
};
        var_t6 = assign59790_e97098;
        var_t6_dn3 = assign59790_e97098_d_n3;
        var_t6_dn4 = assign59790_e97098_d_n4;
        var_t6_dn5 = assign59790_e97098_d_n5;
        var_t6_dn6 = assign59790_e97098_d_n6;
        var_t6_dn7 = assign59790_e97098_d_n7;
        var_t6_dn8 = assign59790_e97098_d_n8;
        var_t6_dn9 = assign59790_e97098_d_n9;
        var_t6_dn10 = assign59790_e97098_d_n10;
        var_t6_dn11 = assign59790_e97098_d_n11;
        var_t6_rv = 0.0;

        let (assign59800_e97131, assign59800_e97131_d_n3, assign59800_e97131_d_n4, assign59800_e97131_d_n5, assign59800_e97131_d_n6, assign59800_e97131_d_n7, assign59800_e97131_d_n8, assign59800_e97131_d_n9, assign59800_e97131_d_n10, assign59800_e97131_d_n11,) = {
    if (((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard871 == 0.0)) {
        let __rspice_inv_cse_0: f64 = 1.0 / var_t3;
        let assign59800_e97108: f64 = (1.0 * __rspice_inv_cse_0);
        let assign59800_e97111: f64 = (1.0 * __rspice_inv_cse_0);
        let assign59800_e97112: f64 = (assign59800_e97108 * assign59800_e97111);
        let assign59800_e97113: f64 = (-assign59800_e97112);
        let assign59800_e97117: f64 = (var_sqrtpsisa * var_sqrtpsisa);
        let assign59800_e97119: f64 = (assign59800_e97117 * var_sqrtpsisa);
        let assign59800_e97122: f64 = (var_t0 * var_t3);
        let assign59800_e97124: f64 = (assign59800_e97122 + var_sqrtpsisa);
        let assign59800_e97125: f64 = (assign59800_e97119 * assign59800_e97124);
        let assign59800_e97126: f64 = (1.0 / assign59800_e97125);
        let assign59800_e97127: f64 = (assign59800_e97113 - assign59800_e97126);
        let assign59800_e97129: f64 = (assign59800_e97127 - var_t6);
        (assign59800_e97129, (((-(((-(var_t3_dn3 / (var_t3 * var_t3))) * assign59800_e97111) + (assign59800_e97108 * (-(var_t3_dn3 / (var_t3 * var_t3)))))) - (-(((((((var_sqrtpsisa_dn3 * var_sqrtpsisa) + (var_sqrtpsisa * var_sqrtpsisa_dn3)) * var_sqrtpsisa) + (assign59800_e97117 * var_sqrtpsisa_dn3)) * assign59800_e97124) + (assign59800_e97119 * (((var_t0_dn3 * var_t3) + (var_t0 * var_t3_dn3)) + var_sqrtpsisa_dn3))) / (assign59800_e97125 * assign59800_e97125)))) - var_t6_dn3), (((-(((-(var_t3_dn4 / (var_t3 * var_t3))) * assign59800_e97111) + (assign59800_e97108 * (-(var_t3_dn4 / (var_t3 * var_t3)))))) - (-(((((((var_sqrtpsisa_dn4 * var_sqrtpsisa) + (var_sqrtpsisa * var_sqrtpsisa_dn4)) * var_sqrtpsisa) + (assign59800_e97117 * var_sqrtpsisa_dn4)) * assign59800_e97124) + (assign59800_e97119 * (((var_t0_dn4 * var_t3) + (var_t0 * var_t3_dn4)) + var_sqrtpsisa_dn4))) / (assign59800_e97125 * assign59800_e97125)))) - var_t6_dn4), (((-(((-(var_t3_dn5 / (var_t3 * var_t3))) * assign59800_e97111) + (assign59800_e97108 * (-(var_t3_dn5 / (var_t3 * var_t3)))))) - (-(((((((var_sqrtpsisa_dn5 * var_sqrtpsisa) + (var_sqrtpsisa * var_sqrtpsisa_dn5)) * var_sqrtpsisa) + (assign59800_e97117 * var_sqrtpsisa_dn5)) * assign59800_e97124) + (assign59800_e97119 * (((var_t0_dn5 * var_t3) + (var_t0 * var_t3_dn5)) + var_sqrtpsisa_dn5))) / (assign59800_e97125 * assign59800_e97125)))) - var_t6_dn5), (((-(((-(var_t3_dn6 / (var_t3 * var_t3))) * assign59800_e97111) + (assign59800_e97108 * (-(var_t3_dn6 / (var_t3 * var_t3)))))) - (-(((((((var_sqrtpsisa_dn6 * var_sqrtpsisa) + (var_sqrtpsisa * var_sqrtpsisa_dn6)) * var_sqrtpsisa) + (assign59800_e97117 * var_sqrtpsisa_dn6)) * assign59800_e97124) + (assign59800_e97119 * (((var_t0_dn6 * var_t3) + (var_t0 * var_t3_dn6)) + var_sqrtpsisa_dn6))) / (assign59800_e97125 * assign59800_e97125)))) - var_t6_dn6), (((-(((-(var_t3_dn7 / (var_t3 * var_t3))) * assign59800_e97111) + (assign59800_e97108 * (-(var_t3_dn7 / (var_t3 * var_t3)))))) - (-(((((((var_sqrtpsisa_dn7 * var_sqrtpsisa) + (var_sqrtpsisa * var_sqrtpsisa_dn7)) * var_sqrtpsisa) + (assign59800_e97117 * var_sqrtpsisa_dn7)) * assign59800_e97124) + (assign59800_e97119 * (((var_t0_dn7 * var_t3) + (var_t0 * var_t3_dn7)) + var_sqrtpsisa_dn7))) / (assign59800_e97125 * assign59800_e97125)))) - var_t6_dn7), (((-(((-(var_t3_dn8 / (var_t3 * var_t3))) * assign59800_e97111) + (assign59800_e97108 * (-(var_t3_dn8 / (var_t3 * var_t3)))))) - (-(((((((var_sqrtpsisa_dn8 * var_sqrtpsisa) + (var_sqrtpsisa * var_sqrtpsisa_dn8)) * var_sqrtpsisa) + (assign59800_e97117 * var_sqrtpsisa_dn8)) * assign59800_e97124) + (assign59800_e97119 * (((var_t0_dn8 * var_t3) + (var_t0 * var_t3_dn8)) + var_sqrtpsisa_dn8))) / (assign59800_e97125 * assign59800_e97125)))) - var_t6_dn8), (((-(((-(var_t3_dn9 / (var_t3 * var_t3))) * assign59800_e97111) + (assign59800_e97108 * (-(var_t3_dn9 / (var_t3 * var_t3)))))) - (-(((((((var_sqrtpsisa_dn9 * var_sqrtpsisa) + (var_sqrtpsisa * var_sqrtpsisa_dn9)) * var_sqrtpsisa) + (assign59800_e97117 * var_sqrtpsisa_dn9)) * assign59800_e97124) + (assign59800_e97119 * (((var_t0_dn9 * var_t3) + (var_t0 * var_t3_dn9)) + var_sqrtpsisa_dn9))) / (assign59800_e97125 * assign59800_e97125)))) - var_t6_dn9), (((-(((-(var_t3_dn10 / (var_t3 * var_t3))) * assign59800_e97111) + (assign59800_e97108 * (-(var_t3_dn10 / (var_t3 * var_t3)))))) - (-(((((((var_sqrtpsisa_dn10 * var_sqrtpsisa) + (var_sqrtpsisa * var_sqrtpsisa_dn10)) * var_sqrtpsisa) + (assign59800_e97117 * var_sqrtpsisa_dn10)) * assign59800_e97124) + (assign59800_e97119 * (((var_t0_dn10 * var_t3) + (var_t0 * var_t3_dn10)) + var_sqrtpsisa_dn10))) / (assign59800_e97125 * assign59800_e97125)))) - var_t6_dn10), (((-(((-(var_t3_dn11 / (var_t3 * var_t3))) * assign59800_e97111) + (assign59800_e97108 * (-(var_t3_dn11 / (var_t3 * var_t3)))))) - (-(((((((var_sqrtpsisa_dn11 * var_sqrtpsisa) + (var_sqrtpsisa * var_sqrtpsisa_dn11)) * var_sqrtpsisa) + (assign59800_e97117 * var_sqrtpsisa_dn11)) * assign59800_e97124) + (assign59800_e97119 * (((var_t0_dn11 * var_t3) + (var_t0 * var_t3_dn11)) + var_sqrtpsisa_dn11))) / (assign59800_e97125 * assign59800_e97125)))) - var_t6_dn11),)
    } else {
        (var_t7, var_t7_dn3, var_t7_dn4, var_t7_dn5, var_t7_dn6, var_t7_dn7, var_t7_dn8, var_t7_dn9, var_t7_dn10, var_t7_dn11,)
    }
};
        var_t7 = assign59800_e97131;
        var_t7_dn3 = assign59800_e97131_d_n3;
        var_t7_dn4 = assign59800_e97131_d_n4;
        var_t7_dn5 = assign59800_e97131_d_n5;
        var_t7_dn6 = assign59800_e97131_d_n6;
        var_t7_dn7 = assign59800_e97131_d_n7;
        var_t7_dn8 = assign59800_e97131_d_n8;
        var_t7_dn9 = assign59800_e97131_d_n9;
        var_t7_dn10 = assign59800_e97131_d_n10;
        var_t7_dn11 = assign59800_e97131_d_n11;
        var_t7_rv = 0.0;

        let (assign59810_e97157, assign59810_e97157_d_n3, assign59810_e97157_d_n4, assign59810_e97157_d_n5, assign59810_e97157_d_n6, assign59810_e97157_d_n7, assign59810_e97157_d_n8, assign59810_e97157_d_n9, assign59810_e97157_d_n10, assign59810_e97157_d_n11,) = {
    if (((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard871 == 0.0)) {
        let assign59810_e97142: f64 = (var_t4 / var_t5);
        let assign59810_e97146: f64 = (var_t4 * var_t7);
        let assign59810_e97149: f64 = (2.0 * var_t5);
        let assign59810_e97151: f64 = (assign59810_e97149 * var_t5);
        let assign59810_e97152: f64 = (assign59810_e97146 / assign59810_e97151);
        let assign59810_e97153: f64 = (1.0 + assign59810_e97152);
        let assign59810_e97154: f64 = (assign59810_e97142 * assign59810_e97153);
        let assign59810_e97155: f64 = (var_t3 - assign59810_e97154);
        (assign59810_e97155, (var_t3_dn3 - (((((var_t4_dn3 * var_t5) - (var_t4 * var_t5_dn3)) / (var_t5 * var_t5)) * assign59810_e97153) + (assign59810_e97142 * (((((var_t4_dn3 * var_t7) + (var_t4 * var_t7_dn3)) * assign59810_e97151) - (assign59810_e97146 * (((2.0 * var_t5_dn3) * var_t5) + (assign59810_e97149 * var_t5_dn3)))) / (assign59810_e97151 * assign59810_e97151))))), (var_t3_dn4 - (((((var_t4_dn4 * var_t5) - (var_t4 * var_t5_dn4)) / (var_t5 * var_t5)) * assign59810_e97153) + (assign59810_e97142 * (((((var_t4_dn4 * var_t7) + (var_t4 * var_t7_dn4)) * assign59810_e97151) - (assign59810_e97146 * (((2.0 * var_t5_dn4) * var_t5) + (assign59810_e97149 * var_t5_dn4)))) / (assign59810_e97151 * assign59810_e97151))))), (var_t3_dn5 - (((((var_t4_dn5 * var_t5) - (var_t4 * var_t5_dn5)) / (var_t5 * var_t5)) * assign59810_e97153) + (assign59810_e97142 * (((((var_t4_dn5 * var_t7) + (var_t4 * var_t7_dn5)) * assign59810_e97151) - (assign59810_e97146 * (((2.0 * var_t5_dn5) * var_t5) + (assign59810_e97149 * var_t5_dn5)))) / (assign59810_e97151 * assign59810_e97151))))), (var_t3_dn6 - (((((var_t4_dn6 * var_t5) - (var_t4 * var_t5_dn6)) / (var_t5 * var_t5)) * assign59810_e97153) + (assign59810_e97142 * (((((var_t4_dn6 * var_t7) + (var_t4 * var_t7_dn6)) * assign59810_e97151) - (assign59810_e97146 * (((2.0 * var_t5_dn6) * var_t5) + (assign59810_e97149 * var_t5_dn6)))) / (assign59810_e97151 * assign59810_e97151))))), (var_t3_dn7 - (((((var_t4_dn7 * var_t5) - (var_t4 * var_t5_dn7)) / (var_t5 * var_t5)) * assign59810_e97153) + (assign59810_e97142 * (((((var_t4_dn7 * var_t7) + (var_t4 * var_t7_dn7)) * assign59810_e97151) - (assign59810_e97146 * (((2.0 * var_t5_dn7) * var_t5) + (assign59810_e97149 * var_t5_dn7)))) / (assign59810_e97151 * assign59810_e97151))))), (var_t3_dn8 - (((((var_t4_dn8 * var_t5) - (var_t4 * var_t5_dn8)) / (var_t5 * var_t5)) * assign59810_e97153) + (assign59810_e97142 * (((((var_t4_dn8 * var_t7) + (var_t4 * var_t7_dn8)) * assign59810_e97151) - (assign59810_e97146 * (((2.0 * var_t5_dn8) * var_t5) + (assign59810_e97149 * var_t5_dn8)))) / (assign59810_e97151 * assign59810_e97151))))), (var_t3_dn9 - (((((var_t4_dn9 * var_t5) - (var_t4 * var_t5_dn9)) / (var_t5 * var_t5)) * assign59810_e97153) + (assign59810_e97142 * (((((var_t4_dn9 * var_t7) + (var_t4 * var_t7_dn9)) * assign59810_e97151) - (assign59810_e97146 * (((2.0 * var_t5_dn9) * var_t5) + (assign59810_e97149 * var_t5_dn9)))) / (assign59810_e97151 * assign59810_e97151))))), (var_t3_dn10 - (((((var_t4_dn10 * var_t5) - (var_t4 * var_t5_dn10)) / (var_t5 * var_t5)) * assign59810_e97153) + (assign59810_e97142 * (((((var_t4_dn10 * var_t7) + (var_t4 * var_t7_dn10)) * assign59810_e97151) - (assign59810_e97146 * (((2.0 * var_t5_dn10) * var_t5) + (assign59810_e97149 * var_t5_dn10)))) / (assign59810_e97151 * assign59810_e97151))))), (var_t3_dn11 - (((((var_t4_dn11 * var_t5) - (var_t4 * var_t5_dn11)) / (var_t5 * var_t5)) * assign59810_e97153) + (assign59810_e97142 * (((((var_t4_dn11 * var_t7) + (var_t4 * var_t7_dn11)) * assign59810_e97151) - (assign59810_e97146 * (((2.0 * var_t5_dn11) * var_t5) + (assign59810_e97149 * var_t5_dn11)))) / (assign59810_e97151 * assign59810_e97151))))),)
    } else {
        (var_qs_edge, var_qs_edge_dn3, var_qs_edge_dn4, var_qs_edge_dn5, var_qs_edge_dn6, var_qs_edge_dn7, var_qs_edge_dn8, var_qs_edge_dn9, var_qs_edge_dn10, var_qs_edge_dn11,)
    }
};
        var_qs_edge = assign59810_e97157;
        var_qs_edge_dn3 = assign59810_e97157_d_n3;
        var_qs_edge_dn4 = assign59810_e97157_d_n4;
        var_qs_edge_dn5 = assign59810_e97157_d_n5;
        var_qs_edge_dn6 = assign59810_e97157_d_n6;
        var_qs_edge_dn7 = assign59810_e97157_d_n7;
        var_qs_edge_dn8 = assign59810_e97157_d_n8;
        var_qs_edge_dn9 = assign59810_e97157_d_n9;
        var_qs_edge_dn10 = assign59810_e97157_d_n10;
        var_qs_edge_dn11 = assign59810_e97157_d_n11;
        var_qs_edge_rv = 0.0;

        let (assign59820_e97172, assign59820_e97172_d_n3, assign59820_e97172_d_n4, assign59820_e97172_d_n5, assign59820_e97172_d_n6, assign59820_e97172_d_n7, assign59820_e97172_d_n8, assign59820_e97172_d_n9, assign59820_e97172_d_n10, assign59820_e97172_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59820_e97164: f64 = (2.0 * var_nvt);
        let assign59820_e97166: f64 = (assign59820_e97164 * var_qs_edge);
        let assign59820_e97169: f64 = (2.0 * var_nvt);
        let assign59820_e97170: f64 = (assign59820_e97166 + assign59820_e97169);
        (assign59820_e97170, ((((2.0 * var_nvt_dn3) * var_qs_edge) + (assign59820_e97164 * var_qs_edge_dn3)) + (2.0 * var_nvt_dn3)), ((((2.0 * var_nvt_dn4) * var_qs_edge) + (assign59820_e97164 * var_qs_edge_dn4)) + (2.0 * var_nvt_dn4)), ((((2.0 * var_nvt_dn5) * var_qs_edge) + (assign59820_e97164 * var_qs_edge_dn5)) + (2.0 * var_nvt_dn5)), ((((2.0 * var_nvt_dn6) * var_qs_edge) + (assign59820_e97164 * var_qs_edge_dn6)) + (2.0 * var_nvt_dn6)), ((((2.0 * var_nvt_dn7) * var_qs_edge) + (assign59820_e97164 * var_qs_edge_dn7)) + (2.0 * var_nvt_dn7)), ((((2.0 * var_nvt_dn8) * var_qs_edge) + (assign59820_e97164 * var_qs_edge_dn8)) + (2.0 * var_nvt_dn8)), ((((2.0 * var_nvt_dn9) * var_qs_edge) + (assign59820_e97164 * var_qs_edge_dn9)) + (2.0 * var_nvt_dn9)), ((((2.0 * var_nvt_dn10) * var_qs_edge) + (assign59820_e97164 * var_qs_edge_dn10)) + (2.0 * var_nvt_dn10)), ((((2.0 * var_nvt_dn11) * var_qs_edge) + (assign59820_e97164 * var_qs_edge_dn11)) + (2.0 * var_nvt_dn11)),)
    } else {
        (var_vdsatedge, var_vdsatedge_dn3, var_vdsatedge_dn4, var_vdsatedge_dn5, var_vdsatedge_dn6, var_vdsatedge_dn7, var_vdsatedge_dn8, var_vdsatedge_dn9, var_vdsatedge_dn10, var_vdsatedge_dn11,)
    }
};
        var_vdsatedge = assign59820_e97172;
        var_vdsatedge_dn3 = assign59820_e97172_d_n3;
        var_vdsatedge_dn4 = assign59820_e97172_d_n4;
        var_vdsatedge_dn5 = assign59820_e97172_d_n5;
        var_vdsatedge_dn6 = assign59820_e97172_d_n6;
        var_vdsatedge_dn7 = assign59820_e97172_d_n7;
        var_vdsatedge_dn8 = assign59820_e97172_d_n8;
        var_vdsatedge_dn9 = assign59820_e97172_d_n9;
        var_vdsatedge_dn10 = assign59820_e97172_d_n10;
        var_vdsatedge_dn11 = assign59820_e97172_d_n11;
        var_vdsatedge_rv = 0.0;

        let (assign59830_e97179, assign59830_e97179_d_n3, assign59830_e97179_d_n4, assign59830_e97179_d_n5, assign59830_e97179_d_n6, assign59830_e97179_d_n7, assign59830_e97179_d_n8, assign59830_e97179_d_n9, assign59830_e97179_d_n10, assign59830_e97179_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        (var_vdsatedge, var_vdsatedge_dn3, var_vdsatedge_dn4, var_vdsatedge_dn5, var_vdsatedge_dn6, var_vdsatedge_dn7, var_vdsatedge_dn8, var_vdsatedge_dn9, var_vdsatedge_dn10, var_vdsatedge_dn11,)
    } else {
        (var_vdsatedge_1, var_vdsatedge_1_dn3, var_vdsatedge_1_dn4, var_vdsatedge_1_dn5, var_vdsatedge_1_dn6, var_vdsatedge_1_dn7, var_vdsatedge_1_dn8, var_vdsatedge_1_dn9, var_vdsatedge_1_dn10, var_vdsatedge_1_dn11,)
    }
};
        var_vdsatedge_1 = assign59830_e97179;
        var_vdsatedge_1_dn3 = assign59830_e97179_d_n3;
        var_vdsatedge_1_dn4 = assign59830_e97179_d_n4;
        var_vdsatedge_1_dn5 = assign59830_e97179_d_n5;
        var_vdsatedge_1_dn6 = assign59830_e97179_d_n6;
        var_vdsatedge_1_dn7 = assign59830_e97179_d_n7;
        var_vdsatedge_1_dn8 = assign59830_e97179_d_n8;
        var_vdsatedge_1_dn9 = assign59830_e97179_d_n9;
        var_vdsatedge_1_dn10 = assign59830_e97179_d_n10;
        var_vdsatedge_1_dn11 = assign59830_e97179_d_n11;
        var_vdsatedge_1_rv = 0.0;

        let (assign59840_e97188, assign59840_e97188_d_n3, assign59840_e97188_d_n4, assign59840_e97188_d_n5, assign59840_e97188_d_n6, assign59840_e97188_d_n7, assign59840_e97188_d_n8, assign59840_e97188_d_n9, assign59840_e97188_d_n10, assign59840_e97188_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59840_e97186: f64 = (var_vdsatedge_1 + var_vs);
        (assign59840_e97186, var_vdsatedge_1_dn3, var_vdsatedge_1_dn4, var_vdsatedge_1_dn5, (var_vdsatedge_1_dn6 + var_vs_dn6), (var_vdsatedge_1_dn7 + var_vs_dn7), var_vdsatedge_1_dn8, var_vdsatedge_1_dn9, (var_vdsatedge_1_dn10 + var_vs_dn10), var_vdsatedge_1_dn11,)
    } else {
        (var_vdsatedge_1, var_vdsatedge_1_dn3, var_vdsatedge_1_dn4, var_vdsatedge_1_dn5, var_vdsatedge_1_dn6, var_vdsatedge_1_dn7, var_vdsatedge_1_dn8, var_vdsatedge_1_dn9, var_vdsatedge_1_dn10, var_vdsatedge_1_dn11,)
    }
};
        var_vdsatedge_1 = assign59840_e97188;
        var_vdsatedge_1_dn3 = assign59840_e97188_d_n3;
        var_vdsatedge_1_dn4 = assign59840_e97188_d_n4;
        var_vdsatedge_1_dn5 = assign59840_e97188_d_n5;
        var_vdsatedge_1_dn6 = assign59840_e97188_d_n6;
        var_vdsatedge_1_dn7 = assign59840_e97188_d_n7;
        var_vdsatedge_1_dn8 = assign59840_e97188_d_n8;
        var_vdsatedge_1_dn9 = assign59840_e97188_d_n9;
        var_vdsatedge_1_dn10 = assign59840_e97188_d_n10;
        var_vdsatedge_1_dn11 = assign59840_e97188_d_n11;
        var_vdsatedge_1_rv = 0.0;

        let (assign59850_e97220, assign59850_e97220_d_n3, assign59850_e97220_d_n4, assign59850_e97220_d_n5, assign59850_e97220_d_n6, assign59850_e97220_d_n7, assign59850_e97220_d_n8, assign59850_e97220_d_n9, assign59850_e97220_d_n10, assign59850_e97220_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59850_e97196: f64 = (var_vdsatedge_1 - var_vs);
        let assign59850_e97198: f64 = assign59850_e97196;
        let assign59850_e97201: f64 = (var_vdsatedge_1 - var_vs);
        let assign59850_e97203: f64 = assign59850_e97201;
        let assign59850_e97206: f64 = (var_vdsatedge_1 - var_vs);
        let assign59850_e97208: f64 = assign59850_e97206;
        let assign59850_e97209: f64 = (assign59850_e97203 * assign59850_e97208);
        let assign59850_e97212: f64 = (0.25 * 0.001);
        let assign59850_e97214: f64 = (assign59850_e97212 * 0.001);
        let assign59850_e97215: f64 = (assign59850_e97209 + assign59850_e97214);
        let assign59850_e97216: f64 = (assign59850_e97215).sqrt();
        let assign59850_e97217: f64 = (assign59850_e97198 + assign59850_e97216);
        let assign59850_e97218: f64 = (0.5 * assign59850_e97217);
        (assign59850_e97218, (0.5 * (var_vdsatedge_1_dn3 + (((var_vdsatedge_1_dn3 * assign59850_e97208) + (assign59850_e97203 * var_vdsatedge_1_dn3)) / (2.0 * assign59850_e97216)))), (0.5 * (var_vdsatedge_1_dn4 + (((var_vdsatedge_1_dn4 * assign59850_e97208) + (assign59850_e97203 * var_vdsatedge_1_dn4)) / (2.0 * assign59850_e97216)))), (0.5 * (var_vdsatedge_1_dn5 + (((var_vdsatedge_1_dn5 * assign59850_e97208) + (assign59850_e97203 * var_vdsatedge_1_dn5)) / (2.0 * assign59850_e97216)))), (0.5 * ((var_vdsatedge_1_dn6 - var_vs_dn6) + ((((var_vdsatedge_1_dn6 - var_vs_dn6) * assign59850_e97208) + (assign59850_e97203 * (var_vdsatedge_1_dn6 - var_vs_dn6))) / (2.0 * assign59850_e97216)))), (0.5 * ((var_vdsatedge_1_dn7 - var_vs_dn7) + ((((var_vdsatedge_1_dn7 - var_vs_dn7) * assign59850_e97208) + (assign59850_e97203 * (var_vdsatedge_1_dn7 - var_vs_dn7))) / (2.0 * assign59850_e97216)))), (0.5 * (var_vdsatedge_1_dn8 + (((var_vdsatedge_1_dn8 * assign59850_e97208) + (assign59850_e97203 * var_vdsatedge_1_dn8)) / (2.0 * assign59850_e97216)))), (0.5 * (var_vdsatedge_1_dn9 + (((var_vdsatedge_1_dn9 * assign59850_e97208) + (assign59850_e97203 * var_vdsatedge_1_dn9)) / (2.0 * assign59850_e97216)))), (0.5 * ((var_vdsatedge_1_dn10 - var_vs_dn10) + ((((var_vdsatedge_1_dn10 - var_vs_dn10) * assign59850_e97208) + (assign59850_e97203 * (var_vdsatedge_1_dn10 - var_vs_dn10))) / (2.0 * assign59850_e97216)))), (0.5 * (var_vdsatedge_1_dn11 + (((var_vdsatedge_1_dn11 * assign59850_e97208) + (assign59850_e97203 * var_vdsatedge_1_dn11)) / (2.0 * assign59850_e97216)))),)
    } else {
        (var_vdssate, var_vdssate_dn3, var_vdssate_dn4, var_vdssate_dn5, var_vdssate_dn6, var_vdssate_dn7, var_vdssate_dn8, var_vdssate_dn9, var_vdssate_dn10, var_vdssate_dn11,)
    }
};
        var_vdssate = assign59850_e97220;
        var_vdssate_dn3 = assign59850_e97220_d_n3;
        var_vdssate_dn4 = assign59850_e97220_d_n4;
        var_vdssate_dn5 = assign59850_e97220_d_n5;
        var_vdssate_dn6 = assign59850_e97220_d_n6;
        var_vdssate_dn7 = assign59850_e97220_d_n7;
        var_vdssate_dn8 = assign59850_e97220_d_n8;
        var_vdssate_dn9 = assign59850_e97220_d_n9;
        var_vdssate_dn10 = assign59850_e97220_d_n10;
        var_vdssate_dn11 = assign59850_e97220_d_n11;
        var_vdssate_rv = 0.0;

        let (assign59860_e97235, assign59860_e97235_d_n3, assign59860_e97235_d_n4, assign59860_e97235_d_n5, assign59860_e97235_d_n6, assign59860_e97235_d_n7, assign59860_e97235_d_n8, assign59860_e97235_d_n9, assign59860_e97235_d_n10, assign59860_e97235_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59860_e97227: f64 = (var_vds / var_vdssate);
        let assign59860_e97229: f64 = (assign59860_e97227 + 1e-6);
        let assign59860_e97232: f64 = (1.0 / var_delta_t);
        let assign59860_e97233: f64 = (assign59860_e97229).powf(assign59860_e97232);
        (assign59860_e97233, if (-(var_delta_t_dn3 / (var_delta_t * var_delta_t))) == 0.0 && ((assign59860_e97232) as f64).is_finite() && ((assign59860_e97232) as f64).fract() == 0.0 { if assign59860_e97232 == 0.0 { 0.0 } else { (assign59860_e97232 * ((assign59860_e97229).powf(assign59860_e97232 - 1.0) * (-((var_vds * var_vdssate_dn3) / (var_vdssate * var_vdssate))))) } } else { (assign59860_e97233 * (((-(var_delta_t_dn3 / (var_delta_t * var_delta_t))) * (assign59860_e97229).ln()) + (assign59860_e97232 * ((-((var_vds * var_vdssate_dn3) / (var_vdssate * var_vdssate))) / assign59860_e97229)))) }, if (-(var_delta_t_dn4 / (var_delta_t * var_delta_t))) == 0.0 && ((assign59860_e97232) as f64).is_finite() && ((assign59860_e97232) as f64).fract() == 0.0 { if assign59860_e97232 == 0.0 { 0.0 } else { (assign59860_e97232 * ((assign59860_e97229).powf(assign59860_e97232 - 1.0) * (-((var_vds * var_vdssate_dn4) / (var_vdssate * var_vdssate))))) } } else { (assign59860_e97233 * (((-(var_delta_t_dn4 / (var_delta_t * var_delta_t))) * (assign59860_e97229).ln()) + (assign59860_e97232 * ((-((var_vds * var_vdssate_dn4) / (var_vdssate * var_vdssate))) / assign59860_e97229)))) }, if (-(var_delta_t_dn5 / (var_delta_t * var_delta_t))) == 0.0 && ((assign59860_e97232) as f64).is_finite() && ((assign59860_e97232) as f64).fract() == 0.0 { if assign59860_e97232 == 0.0 { 0.0 } else { (assign59860_e97232 * ((assign59860_e97229).powf(assign59860_e97232 - 1.0) * (-((var_vds * var_vdssate_dn5) / (var_vdssate * var_vdssate))))) } } else { (assign59860_e97233 * (((-(var_delta_t_dn5 / (var_delta_t * var_delta_t))) * (assign59860_e97229).ln()) + (assign59860_e97232 * ((-((var_vds * var_vdssate_dn5) / (var_vdssate * var_vdssate))) / assign59860_e97229)))) }, if (-(var_delta_t_dn6 / (var_delta_t * var_delta_t))) == 0.0 && ((assign59860_e97232) as f64).is_finite() && ((assign59860_e97232) as f64).fract() == 0.0 { if assign59860_e97232 == 0.0 { 0.0 } else { (assign59860_e97232 * ((assign59860_e97229).powf(assign59860_e97232 - 1.0) * (((var_vds_dn6 * var_vdssate) - (var_vds * var_vdssate_dn6)) / (var_vdssate * var_vdssate)))) } } else { (assign59860_e97233 * (((-(var_delta_t_dn6 / (var_delta_t * var_delta_t))) * (assign59860_e97229).ln()) + (assign59860_e97232 * ((((var_vds_dn6 * var_vdssate) - (var_vds * var_vdssate_dn6)) / (var_vdssate * var_vdssate)) / assign59860_e97229)))) }, if (-(var_delta_t_dn7 / (var_delta_t * var_delta_t))) == 0.0 && ((assign59860_e97232) as f64).is_finite() && ((assign59860_e97232) as f64).fract() == 0.0 { if assign59860_e97232 == 0.0 { 0.0 } else { (assign59860_e97232 * ((assign59860_e97229).powf(assign59860_e97232 - 1.0) * (((var_vds_dn7 * var_vdssate) - (var_vds * var_vdssate_dn7)) / (var_vdssate * var_vdssate)))) } } else { (assign59860_e97233 * (((-(var_delta_t_dn7 / (var_delta_t * var_delta_t))) * (assign59860_e97229).ln()) + (assign59860_e97232 * ((((var_vds_dn7 * var_vdssate) - (var_vds * var_vdssate_dn7)) / (var_vdssate * var_vdssate)) / assign59860_e97229)))) }, if (-(var_delta_t_dn8 / (var_delta_t * var_delta_t))) == 0.0 && ((assign59860_e97232) as f64).is_finite() && ((assign59860_e97232) as f64).fract() == 0.0 { if assign59860_e97232 == 0.0 { 0.0 } else { (assign59860_e97232 * ((assign59860_e97229).powf(assign59860_e97232 - 1.0) * (-((var_vds * var_vdssate_dn8) / (var_vdssate * var_vdssate))))) } } else { (assign59860_e97233 * (((-(var_delta_t_dn8 / (var_delta_t * var_delta_t))) * (assign59860_e97229).ln()) + (assign59860_e97232 * ((-((var_vds * var_vdssate_dn8) / (var_vdssate * var_vdssate))) / assign59860_e97229)))) }, if (-(var_delta_t_dn9 / (var_delta_t * var_delta_t))) == 0.0 && ((assign59860_e97232) as f64).is_finite() && ((assign59860_e97232) as f64).fract() == 0.0 { if assign59860_e97232 == 0.0 { 0.0 } else { (assign59860_e97232 * ((assign59860_e97229).powf(assign59860_e97232 - 1.0) * (-((var_vds * var_vdssate_dn9) / (var_vdssate * var_vdssate))))) } } else { (assign59860_e97233 * (((-(var_delta_t_dn9 / (var_delta_t * var_delta_t))) * (assign59860_e97229).ln()) + (assign59860_e97232 * ((-((var_vds * var_vdssate_dn9) / (var_vdssate * var_vdssate))) / assign59860_e97229)))) }, if (-(var_delta_t_dn10 / (var_delta_t * var_delta_t))) == 0.0 && ((assign59860_e97232) as f64).is_finite() && ((assign59860_e97232) as f64).fract() == 0.0 { if assign59860_e97232 == 0.0 { 0.0 } else { (assign59860_e97232 * ((assign59860_e97229).powf(assign59860_e97232 - 1.0) * (((var_vds_dn10 * var_vdssate) - (var_vds * var_vdssate_dn10)) / (var_vdssate * var_vdssate)))) } } else { (assign59860_e97233 * (((-(var_delta_t_dn10 / (var_delta_t * var_delta_t))) * (assign59860_e97229).ln()) + (assign59860_e97232 * ((((var_vds_dn10 * var_vdssate) - (var_vds * var_vdssate_dn10)) / (var_vdssate * var_vdssate)) / assign59860_e97229)))) }, if (-(var_delta_t_dn11 / (var_delta_t * var_delta_t))) == 0.0 && ((assign59860_e97232) as f64).is_finite() && ((assign59860_e97232) as f64).fract() == 0.0 { if assign59860_e97232 == 0.0 { 0.0 } else { (assign59860_e97232 * ((assign59860_e97229).powf(assign59860_e97232 - 1.0) * (-((var_vds * var_vdssate_dn11) / (var_vdssate * var_vdssate))))) } } else { (assign59860_e97233 * (((-(var_delta_t_dn11 / (var_delta_t * var_delta_t))) * (assign59860_e97229).ln()) + (assign59860_e97232 * ((-((var_vds * var_vdssate_dn11) / (var_vdssate * var_vdssate))) / assign59860_e97229)))) },)
    } else {
        (var_t7, var_t7_dn3, var_t7_dn4, var_t7_dn5, var_t7_dn6, var_t7_dn7, var_t7_dn8, var_t7_dn9, var_t7_dn10, var_t7_dn11,)
    }
};
        var_t7 = assign59860_e97235;
        var_t7_dn3 = assign59860_e97235_d_n3;
        var_t7_dn4 = assign59860_e97235_d_n4;
        var_t7_dn5 = assign59860_e97235_d_n5;
        var_t7_dn6 = assign59860_e97235_d_n6;
        var_t7_dn7 = assign59860_e97235_d_n7;
        var_t7_dn8 = assign59860_e97235_d_n8;
        var_t7_dn9 = assign59860_e97235_d_n9;
        var_t7_dn10 = assign59860_e97235_d_n10;
        var_t7_dn11 = assign59860_e97235_d_n11;
        var_t7_rv = 0.0;

        let (assign59870_e97247, assign59870_e97247_d_n3, assign59870_e97247_d_n4, assign59870_e97247_d_n5, assign59870_e97247_d_n6, assign59870_e97247_d_n7, assign59870_e97247_d_n8, assign59870_e97247_d_n9, assign59870_e97247_d_n10, assign59870_e97247_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59870_e97242: f64 = (1.0 + var_t7);
        let assign59870_e97244: f64 = (-var_delta_t);
        let assign59870_e97245: f64 = (assign59870_e97242).powf(assign59870_e97244);
        (assign59870_e97245, if (-var_delta_t_dn3) == 0.0 && ((assign59870_e97244) as f64).is_finite() && ((assign59870_e97244) as f64).fract() == 0.0 { if assign59870_e97244 == 0.0 { 0.0 } else { (assign59870_e97244 * ((assign59870_e97242).powf(assign59870_e97244 - 1.0) * var_t7_dn3)) } } else { (assign59870_e97245 * (((-var_delta_t_dn3) * (assign59870_e97242).ln()) + (assign59870_e97244 * (var_t7_dn3 / assign59870_e97242)))) }, if (-var_delta_t_dn4) == 0.0 && ((assign59870_e97244) as f64).is_finite() && ((assign59870_e97244) as f64).fract() == 0.0 { if assign59870_e97244 == 0.0 { 0.0 } else { (assign59870_e97244 * ((assign59870_e97242).powf(assign59870_e97244 - 1.0) * var_t7_dn4)) } } else { (assign59870_e97245 * (((-var_delta_t_dn4) * (assign59870_e97242).ln()) + (assign59870_e97244 * (var_t7_dn4 / assign59870_e97242)))) }, if (-var_delta_t_dn5) == 0.0 && ((assign59870_e97244) as f64).is_finite() && ((assign59870_e97244) as f64).fract() == 0.0 { if assign59870_e97244 == 0.0 { 0.0 } else { (assign59870_e97244 * ((assign59870_e97242).powf(assign59870_e97244 - 1.0) * var_t7_dn5)) } } else { (assign59870_e97245 * (((-var_delta_t_dn5) * (assign59870_e97242).ln()) + (assign59870_e97244 * (var_t7_dn5 / assign59870_e97242)))) }, if (-var_delta_t_dn6) == 0.0 && ((assign59870_e97244) as f64).is_finite() && ((assign59870_e97244) as f64).fract() == 0.0 { if assign59870_e97244 == 0.0 { 0.0 } else { (assign59870_e97244 * ((assign59870_e97242).powf(assign59870_e97244 - 1.0) * var_t7_dn6)) } } else { (assign59870_e97245 * (((-var_delta_t_dn6) * (assign59870_e97242).ln()) + (assign59870_e97244 * (var_t7_dn6 / assign59870_e97242)))) }, if (-var_delta_t_dn7) == 0.0 && ((assign59870_e97244) as f64).is_finite() && ((assign59870_e97244) as f64).fract() == 0.0 { if assign59870_e97244 == 0.0 { 0.0 } else { (assign59870_e97244 * ((assign59870_e97242).powf(assign59870_e97244 - 1.0) * var_t7_dn7)) } } else { (assign59870_e97245 * (((-var_delta_t_dn7) * (assign59870_e97242).ln()) + (assign59870_e97244 * (var_t7_dn7 / assign59870_e97242)))) }, if (-var_delta_t_dn8) == 0.0 && ((assign59870_e97244) as f64).is_finite() && ((assign59870_e97244) as f64).fract() == 0.0 { if assign59870_e97244 == 0.0 { 0.0 } else { (assign59870_e97244 * ((assign59870_e97242).powf(assign59870_e97244 - 1.0) * var_t7_dn8)) } } else { (assign59870_e97245 * (((-var_delta_t_dn8) * (assign59870_e97242).ln()) + (assign59870_e97244 * (var_t7_dn8 / assign59870_e97242)))) }, if (-var_delta_t_dn9) == 0.0 && ((assign59870_e97244) as f64).is_finite() && ((assign59870_e97244) as f64).fract() == 0.0 { if assign59870_e97244 == 0.0 { 0.0 } else { (assign59870_e97244 * ((assign59870_e97242).powf(assign59870_e97244 - 1.0) * var_t7_dn9)) } } else { (assign59870_e97245 * (((-var_delta_t_dn9) * (assign59870_e97242).ln()) + (assign59870_e97244 * (var_t7_dn9 / assign59870_e97242)))) }, if (-var_delta_t_dn10) == 0.0 && ((assign59870_e97244) as f64).is_finite() && ((assign59870_e97244) as f64).fract() == 0.0 { if assign59870_e97244 == 0.0 { 0.0 } else { (assign59870_e97244 * ((assign59870_e97242).powf(assign59870_e97244 - 1.0) * var_t7_dn10)) } } else { (assign59870_e97245 * (((-var_delta_t_dn10) * (assign59870_e97242).ln()) + (assign59870_e97244 * (var_t7_dn10 / assign59870_e97242)))) }, if (-var_delta_t_dn11) == 0.0 && ((assign59870_e97244) as f64).is_finite() && ((assign59870_e97244) as f64).fract() == 0.0 { if assign59870_e97244 == 0.0 { 0.0 } else { (assign59870_e97244 * ((assign59870_e97242).powf(assign59870_e97244 - 1.0) * var_t7_dn11)) } } else { (assign59870_e97245 * (((-var_delta_t_dn11) * (assign59870_e97242).ln()) + (assign59870_e97244 * (var_t7_dn11 / assign59870_e97242)))) },)
    } else {
        (var_t8, var_t8_dn3, var_t8_dn4, var_t8_dn5, var_t8_dn6, var_t8_dn7, var_t8_dn8, var_t8_dn9, var_t8_dn10, var_t8_dn11,)
    }
};
        var_t8 = assign59870_e97247;
        var_t8_dn3 = assign59870_e97247_d_n3;
        var_t8_dn4 = assign59870_e97247_d_n4;
        var_t8_dn5 = assign59870_e97247_d_n5;
        var_t8_dn6 = assign59870_e97247_d_n6;
        var_t8_dn7 = assign59870_e97247_d_n7;
        var_t8_dn8 = assign59870_e97247_d_n8;
        var_t8_dn9 = assign59870_e97247_d_n9;
        var_t8_dn10 = assign59870_e97247_d_n10;
        var_t8_dn11 = assign59870_e97247_d_n11;
        var_t8_rv = 0.0;

        let (assign59880_e97256, assign59880_e97256_d_n3, assign59880_e97256_d_n4, assign59880_e97256_d_n5, assign59880_e97256_d_n6, assign59880_e97256_d_n7, assign59880_e97256_d_n8, assign59880_e97256_d_n9, assign59880_e97256_d_n10, assign59880_e97256_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59880_e97254: f64 = (var_vds * var_t8);
        (assign59880_e97254, (var_vds * var_t8_dn3), (var_vds * var_t8_dn4), (var_vds * var_t8_dn5), ((var_vds_dn6 * var_t8) + (var_vds * var_t8_dn6)), ((var_vds_dn7 * var_t8) + (var_vds * var_t8_dn7)), (var_vds * var_t8_dn8), (var_vds * var_t8_dn9), ((var_vds_dn10 * var_t8) + (var_vds * var_t8_dn10)), (var_vds * var_t8_dn11),)
    } else {
        (var_vdseff, var_vdseff_dn3, var_vdseff_dn4, var_vdseff_dn5, var_vdseff_dn6, var_vdseff_dn7, var_vdseff_dn8, var_vdseff_dn9, var_vdseff_dn10, var_vdseff_dn11,)
    }
};
        var_vdseff = assign59880_e97256;
        var_vdseff_dn3 = assign59880_e97256_d_n3;
        var_vdseff_dn4 = assign59880_e97256_d_n4;
        var_vdseff_dn5 = assign59880_e97256_d_n5;
        var_vdseff_dn6 = assign59880_e97256_d_n6;
        var_vdseff_dn7 = assign59880_e97256_d_n7;
        var_vdseff_dn8 = assign59880_e97256_d_n8;
        var_vdseff_dn9 = assign59880_e97256_d_n9;
        var_vdseff_dn10 = assign59880_e97256_d_n10;
        var_vdseff_dn11 = assign59880_e97256_d_n11;
        var_vdseff_rv = 0.0;

        let (assign59890_e97267, assign59890_e97267_d_n3, assign59890_e97267_d_n4, assign59890_e97267_d_n5, assign59890_e97267_d_n6, assign59890_e97267_d_n7, assign59890_e97267_d_n8, assign59890_e97267_d_n9, assign59890_e97267_d_n10, assign59890_e97267_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59890_e97263: f64 = (var_vdseff + var_vs);
        let assign59890_e97265: f64 = (assign59890_e97263 * var_inv_nvt);
        (assign59890_e97265, ((var_vdseff_dn3 * var_inv_nvt) + (assign59890_e97263 * var_inv_nvt_dn3)), ((var_vdseff_dn4 * var_inv_nvt) + (assign59890_e97263 * var_inv_nvt_dn4)), ((var_vdseff_dn5 * var_inv_nvt) + (assign59890_e97263 * var_inv_nvt_dn5)), (((var_vdseff_dn6 + var_vs_dn6) * var_inv_nvt) + (assign59890_e97263 * var_inv_nvt_dn6)), (((var_vdseff_dn7 + var_vs_dn7) * var_inv_nvt) + (assign59890_e97263 * var_inv_nvt_dn7)), ((var_vdseff_dn8 * var_inv_nvt) + (assign59890_e97263 * var_inv_nvt_dn8)), ((var_vdseff_dn9 * var_inv_nvt) + (assign59890_e97263 * var_inv_nvt_dn9)), (((var_vdseff_dn10 + var_vs_dn10) * var_inv_nvt) + (assign59890_e97263 * var_inv_nvt_dn10)), ((var_vdseff_dn11 * var_inv_nvt) + (assign59890_e97263 * var_inv_nvt_dn11)),)
    } else {
        (var_vdeff, var_vdeff_dn3, var_vdeff_dn4, var_vdeff_dn5, var_vdeff_dn6, var_vdeff_dn7, var_vdeff_dn8, var_vdeff_dn9, var_vdeff_dn10, var_vdeff_dn11,)
    }
};
        var_vdeff = assign59890_e97267;
        var_vdeff_dn3 = assign59890_e97267_d_n3;
        var_vdeff_dn4 = assign59890_e97267_d_n4;
        var_vdeff_dn5 = assign59890_e97267_d_n5;
        var_vdeff_dn6 = assign59890_e97267_d_n6;
        var_vdeff_dn7 = assign59890_e97267_d_n7;
        var_vdeff_dn8 = assign59890_e97267_d_n8;
        var_vdeff_dn9 = assign59890_e97267_d_n9;
        var_vdeff_dn10 = assign59890_e97267_d_n10;
        var_vdeff_dn11 = assign59890_e97267_d_n11;
        var_vdeff_rv = 0.0;

        let (assign59900_e97293, assign59900_e97293_d_n3, assign59900_e97293_d_n4, assign59900_e97293_d_n5, assign59900_e97293_d_n6, assign59900_e97293_d_n7, assign59900_e97293_d_n8, assign59900_e97293_d_n9, assign59900_e97293_d_n10, assign59900_e97293_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59900_e97275: f64 = (var_psip + 1.0);
        let assign59900_e97278: f64 = (var_psip - 1.0);
        let assign59900_e97281: f64 = (var_psip - 1.0);
        let assign59900_e97282: f64 = (assign59900_e97278 * assign59900_e97281);
        let assign59900_e97285: f64 = (0.25 * 2.0);
        let assign59900_e97287: f64 = (assign59900_e97285 * 2.0);
        let assign59900_e97288: f64 = (assign59900_e97282 + assign59900_e97287);
        let assign59900_e97289: f64 = (assign59900_e97288).sqrt();
        let assign59900_e97290: f64 = (assign59900_e97275 + assign59900_e97289);
        let assign59900_e97291: f64 = (0.5 * assign59900_e97290);
        (assign59900_e97291, (0.5 * (var_psip_dn3 + (((var_psip_dn3 * assign59900_e97281) + (assign59900_e97278 * var_psip_dn3)) / (2.0 * assign59900_e97289)))), (0.5 * (var_psip_dn4 + (((var_psip_dn4 * assign59900_e97281) + (assign59900_e97278 * var_psip_dn4)) / (2.0 * assign59900_e97289)))), (0.5 * (var_psip_dn5 + (((var_psip_dn5 * assign59900_e97281) + (assign59900_e97278 * var_psip_dn5)) / (2.0 * assign59900_e97289)))), (0.5 * (var_psip_dn6 + (((var_psip_dn6 * assign59900_e97281) + (assign59900_e97278 * var_psip_dn6)) / (2.0 * assign59900_e97289)))), (0.5 * (var_psip_dn7 + (((var_psip_dn7 * assign59900_e97281) + (assign59900_e97278 * var_psip_dn7)) / (2.0 * assign59900_e97289)))), (0.5 * (var_psip_dn8 + (((var_psip_dn8 * assign59900_e97281) + (assign59900_e97278 * var_psip_dn8)) / (2.0 * assign59900_e97289)))), (0.5 * (var_psip_dn9 + (((var_psip_dn9 * assign59900_e97281) + (assign59900_e97278 * var_psip_dn9)) / (2.0 * assign59900_e97289)))), (0.5 * (var_psip_dn10 + (((var_psip_dn10 * assign59900_e97281) + (assign59900_e97278 * var_psip_dn10)) / (2.0 * assign59900_e97289)))), (0.5 * (var_psip_dn11 + (((var_psip_dn11 * assign59900_e97281) + (assign59900_e97278 * var_psip_dn11)) / (2.0 * assign59900_e97289)))),)
    } else {
        (var_t8, var_t8_dn3, var_t8_dn4, var_t8_dn5, var_t8_dn6, var_t8_dn7, var_t8_dn8, var_t8_dn9, var_t8_dn10, var_t8_dn11,)
    }
};
        var_t8 = assign59900_e97293;
        var_t8_dn3 = assign59900_e97293_d_n3;
        var_t8_dn4 = assign59900_e97293_d_n4;
        var_t8_dn5 = assign59900_e97293_d_n5;
        var_t8_dn6 = assign59900_e97293_d_n6;
        var_t8_dn7 = assign59900_e97293_d_n7;
        var_t8_dn8 = assign59900_e97293_d_n8;
        var_t8_dn9 = assign59900_e97293_d_n9;
        var_t8_dn10 = assign59900_e97293_d_n10;
        var_t8_dn11 = assign59900_e97293_d_n11;
        var_t8_rv = 0.0;

        let (assign59910_e97301, assign59910_e97301_d_n3, assign59910_e97301_d_n4, assign59910_e97301_d_n5, assign59910_e97301_d_n6, assign59910_e97301_d_n7, assign59910_e97301_d_n8, assign59910_e97301_d_n9, assign59910_e97301_d_n10, assign59910_e97301_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59910_e97299: f64 = (var_t8).sqrt();
        (assign59910_e97299, (var_t8_dn3 / (2.0 * assign59910_e97299)), (var_t8_dn4 / (2.0 * assign59910_e97299)), (var_t8_dn5 / (2.0 * assign59910_e97299)), (var_t8_dn6 / (2.0 * assign59910_e97299)), (var_t8_dn7 / (2.0 * assign59910_e97299)), (var_t8_dn8 / (2.0 * assign59910_e97299)), (var_t8_dn9 / (2.0 * assign59910_e97299)), (var_t8_dn10 / (2.0 * assign59910_e97299)), (var_t8_dn11 / (2.0 * assign59910_e97299)),)
    } else {
        (var_sqrtpsip, var_sqrtpsip_dn3, var_sqrtpsip_dn4, var_sqrtpsip_dn5, var_sqrtpsip_dn6, var_sqrtpsip_dn7, var_sqrtpsip_dn8, var_sqrtpsip_dn9, var_sqrtpsip_dn10, var_sqrtpsip_dn11,)
    }
};
        var_sqrtpsip = assign59910_e97301;
        var_sqrtpsip_dn3 = assign59910_e97301_d_n3;
        var_sqrtpsip_dn4 = assign59910_e97301_d_n4;
        var_sqrtpsip_dn5 = assign59910_e97301_d_n5;
        var_sqrtpsip_dn6 = assign59910_e97301_d_n6;
        var_sqrtpsip_dn7 = assign59910_e97301_d_n7;
        var_sqrtpsip_dn8 = assign59910_e97301_d_n8;
        var_sqrtpsip_dn9 = assign59910_e97301_d_n9;
        var_sqrtpsip_dn10 = assign59910_e97301_d_n10;
        var_sqrtpsip_dn11 = assign59910_e97301_d_n11;
        var_sqrtpsip_rv = 0.0;

        let (assign59920_e97316, assign59920_e97316_d_n3, assign59920_e97316_d_n4, assign59920_e97316_d_n5, assign59920_e97316_d_n6, assign59920_e97316_d_n7, assign59920_e97316_d_n8, assign59920_e97316_d_n9, assign59920_e97316_d_n10, assign59920_e97316_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59920_e97310: f64 = (2.0 * var_sqrtpsip);
        let assign59920_e97311: f64 = (var_gam_edge / assign59920_e97310);
        let assign59920_e97312: f64 = (1.0 + assign59920_e97311);
        let assign59920_e97314: f64 = (assign59920_e97312 / var_gam_edge);
        (assign59920_e97314, ((((((var_gam_edge_dn3 * assign59920_e97310) - (var_gam_edge * (2.0 * var_sqrtpsip_dn3))) / (assign59920_e97310 * assign59920_e97310)) * var_gam_edge) - (assign59920_e97312 * var_gam_edge_dn3)) / (var_gam_edge * var_gam_edge)), ((((((var_gam_edge_dn4 * assign59920_e97310) - (var_gam_edge * (2.0 * var_sqrtpsip_dn4))) / (assign59920_e97310 * assign59920_e97310)) * var_gam_edge) - (assign59920_e97312 * var_gam_edge_dn4)) / (var_gam_edge * var_gam_edge)), ((((((var_gam_edge_dn5 * assign59920_e97310) - (var_gam_edge * (2.0 * var_sqrtpsip_dn5))) / (assign59920_e97310 * assign59920_e97310)) * var_gam_edge) - (assign59920_e97312 * var_gam_edge_dn5)) / (var_gam_edge * var_gam_edge)), ((((((var_gam_edge_dn6 * assign59920_e97310) - (var_gam_edge * (2.0 * var_sqrtpsip_dn6))) / (assign59920_e97310 * assign59920_e97310)) * var_gam_edge) - (assign59920_e97312 * var_gam_edge_dn6)) / (var_gam_edge * var_gam_edge)), ((((((var_gam_edge_dn7 * assign59920_e97310) - (var_gam_edge * (2.0 * var_sqrtpsip_dn7))) / (assign59920_e97310 * assign59920_e97310)) * var_gam_edge) - (assign59920_e97312 * var_gam_edge_dn7)) / (var_gam_edge * var_gam_edge)), ((((((var_gam_edge_dn8 * assign59920_e97310) - (var_gam_edge * (2.0 * var_sqrtpsip_dn8))) / (assign59920_e97310 * assign59920_e97310)) * var_gam_edge) - (assign59920_e97312 * var_gam_edge_dn8)) / (var_gam_edge * var_gam_edge)), ((((((var_gam_edge_dn9 * assign59920_e97310) - (var_gam_edge * (2.0 * var_sqrtpsip_dn9))) / (assign59920_e97310 * assign59920_e97310)) * var_gam_edge) - (assign59920_e97312 * var_gam_edge_dn9)) / (var_gam_edge * var_gam_edge)), ((((((var_gam_edge_dn10 * assign59920_e97310) - (var_gam_edge * (2.0 * var_sqrtpsip_dn10))) / (assign59920_e97310 * assign59920_e97310)) * var_gam_edge) - (assign59920_e97312 * var_gam_edge_dn10)) / (var_gam_edge * var_gam_edge)), ((((((var_gam_edge_dn11 * assign59920_e97310) - (var_gam_edge * (2.0 * var_sqrtpsip_dn11))) / (assign59920_e97310 * assign59920_e97310)) * var_gam_edge) - (assign59920_e97312 * var_gam_edge_dn11)) / (var_gam_edge * var_gam_edge)),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11,)
    }
};
        var_t0 = assign59920_e97316;
        var_t0_dn3 = assign59920_e97316_d_n3;
        var_t0_dn4 = assign59920_e97316_d_n4;
        var_t0_dn5 = assign59920_e97316_d_n5;
        var_t0_dn6 = assign59920_e97316_d_n6;
        var_t0_dn7 = assign59920_e97316_d_n7;
        var_t0_dn8 = assign59920_e97316_d_n8;
        var_t0_dn9 = assign59920_e97316_d_n9;
        var_t0_dn10 = assign59920_e97316_d_n10;
        var_t0_dn11 = assign59920_e97316_d_n11;
        var_t0_rv = 0.0;

        let (assign59930_e97329, assign59930_e97329_d_n3, assign59930_e97329_d_n4, assign59930_e97329_d_n5, assign59930_e97329_d_n6, assign59930_e97329_d_n7, assign59930_e97329_d_n8, assign59930_e97329_d_n9, assign59930_e97329_d_n10, assign59930_e97329_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59930_e97324: f64 = (2.0 * var_phib_n_edge);
        let assign59930_e97325: f64 = (var_psip - assign59930_e97324);
        let assign59930_e97327: f64 = (assign59930_e97325 - var_vdeff);
        (assign59930_e97327, ((var_psip_dn3 - (2.0 * var_phib_n_edge_dn3)) - var_vdeff_dn3), ((var_psip_dn4 - (2.0 * var_phib_n_edge_dn4)) - var_vdeff_dn4), ((var_psip_dn5 - (2.0 * var_phib_n_edge_dn5)) - var_vdeff_dn5), ((var_psip_dn6 - (2.0 * var_phib_n_edge_dn6)) - var_vdeff_dn6), ((var_psip_dn7 - (2.0 * var_phib_n_edge_dn7)) - var_vdeff_dn7), ((var_psip_dn8 - (2.0 * var_phib_n_edge_dn8)) - var_vdeff_dn8), ((var_psip_dn9 - (2.0 * var_phib_n_edge_dn9)) - var_vdeff_dn9), ((var_psip_dn10 - (2.0 * var_phib_n_edge_dn10)) - var_vdeff_dn10), ((var_psip_dn11 - (2.0 * var_phib_n_edge_dn11)) - var_vdeff_dn11),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11,)
    }
};
        var_t1 = assign59930_e97329;
        var_t1_dn3 = assign59930_e97329_d_n3;
        var_t1_dn4 = assign59930_e97329_d_n4;
        var_t1_dn5 = assign59930_e97329_d_n5;
        var_t1_dn6 = assign59930_e97329_d_n6;
        var_t1_dn7 = assign59930_e97329_d_n7;
        var_t1_dn8 = assign59930_e97329_d_n8;
        var_t1_dn9 = assign59930_e97329_d_n9;
        var_t1_dn10 = assign59930_e97329_d_n10;
        var_t1_dn11 = assign59930_e97329_d_n11;
        var_t1_rv = 0.0;

        let (assign59940_e97345, assign59940_e97345_d_n3, assign59940_e97345_d_n4, assign59940_e97345_d_n5, assign59940_e97345_d_n6, assign59940_e97345_d_n7, assign59940_e97345_d_n8, assign59940_e97345_d_n9, assign59940_e97345_d_n10, assign59940_e97345_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59940_e97337: f64 = (4.0 * var_t0);
        let assign59940_e97339: f64 = (assign59940_e97337 * var_sqrtpsip);
        let assign59940_e97341: f64 = (assign59940_e97339).max(1e-38);
        let assign59940_e97342: f64 = (assign59940_e97341).ln();
        let assign59940_e97343: f64 = (var_t1 - assign59940_e97342);
        (assign59940_e97343, (var_t1_dn3 - (if assign59940_e97339 >= 1e-38 { (((4.0 * var_t0_dn3) * var_sqrtpsip) + (assign59940_e97337 * var_sqrtpsip_dn3)) } else { 0.0 } / assign59940_e97341)), (var_t1_dn4 - (if assign59940_e97339 >= 1e-38 { (((4.0 * var_t0_dn4) * var_sqrtpsip) + (assign59940_e97337 * var_sqrtpsip_dn4)) } else { 0.0 } / assign59940_e97341)), (var_t1_dn5 - (if assign59940_e97339 >= 1e-38 { (((4.0 * var_t0_dn5) * var_sqrtpsip) + (assign59940_e97337 * var_sqrtpsip_dn5)) } else { 0.0 } / assign59940_e97341)), (var_t1_dn6 - (if assign59940_e97339 >= 1e-38 { (((4.0 * var_t0_dn6) * var_sqrtpsip) + (assign59940_e97337 * var_sqrtpsip_dn6)) } else { 0.0 } / assign59940_e97341)), (var_t1_dn7 - (if assign59940_e97339 >= 1e-38 { (((4.0 * var_t0_dn7) * var_sqrtpsip) + (assign59940_e97337 * var_sqrtpsip_dn7)) } else { 0.0 } / assign59940_e97341)), (var_t1_dn8 - (if assign59940_e97339 >= 1e-38 { (((4.0 * var_t0_dn8) * var_sqrtpsip) + (assign59940_e97337 * var_sqrtpsip_dn8)) } else { 0.0 } / assign59940_e97341)), (var_t1_dn9 - (if assign59940_e97339 >= 1e-38 { (((4.0 * var_t0_dn9) * var_sqrtpsip) + (assign59940_e97337 * var_sqrtpsip_dn9)) } else { 0.0 } / assign59940_e97341)), (var_t1_dn10 - (if assign59940_e97339 >= 1e-38 { (((4.0 * var_t0_dn10) * var_sqrtpsip) + (assign59940_e97337 * var_sqrtpsip_dn10)) } else { 0.0 } / assign59940_e97341)), (var_t1_dn11 - (if assign59940_e97339 >= 1e-38 { (((4.0 * var_t0_dn11) * var_sqrtpsip) + (assign59940_e97337 * var_sqrtpsip_dn11)) } else { 0.0 } / assign59940_e97341)),)
    } else {
        (var_t2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11,)
    }
};
        var_t2 = assign59940_e97345;
        var_t2_dn3 = assign59940_e97345_d_n3;
        var_t2_dn4 = assign59940_e97345_d_n4;
        var_t2_dn5 = assign59940_e97345_d_n5;
        var_t2_dn6 = assign59940_e97345_d_n6;
        var_t2_dn7 = assign59940_e97345_d_n7;
        var_t2_dn8 = assign59940_e97345_d_n8;
        var_t2_dn9 = assign59940_e97345_d_n9;
        var_t2_dn10 = assign59940_e97345_d_n10;
        var_t2_dn11 = assign59940_e97345_d_n11;
        var_t2_rv = 0.0;

        let (assign59950_e97365, assign59950_e97365_d_n3, assign59950_e97365_d_n4, assign59950_e97365_d_n5, assign59950_e97365_d_n6, assign59950_e97365_d_n7, assign59950_e97365_d_n8, assign59950_e97365_d_n9, assign59950_e97365_d_n10, assign59950_e97365_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign59950_e97353: f64 = (var_t2 - 0.201491);
        let assign59950_e97357: f64 = (var_t2 + 0.402982);
        let assign59950_e97358: f64 = (var_t2 * assign59950_e97357);
        let assign59950_e97360: f64 = (assign59950_e97358 + 2.446562);
        let assign59950_e97361: f64 = (assign59950_e97360).sqrt();
        let assign59950_e97362: f64 = (assign59950_e97353 - assign59950_e97361);
        let assign59950_e97363: f64 = (0.5 * assign59950_e97362);
        (assign59950_e97363, (0.5 * (var_t2_dn3 - (((var_t2_dn3 * assign59950_e97357) + (var_t2 * var_t2_dn3)) / (2.0 * assign59950_e97361)))), (0.5 * (var_t2_dn4 - (((var_t2_dn4 * assign59950_e97357) + (var_t2 * var_t2_dn4)) / (2.0 * assign59950_e97361)))), (0.5 * (var_t2_dn5 - (((var_t2_dn5 * assign59950_e97357) + (var_t2 * var_t2_dn5)) / (2.0 * assign59950_e97361)))), (0.5 * (var_t2_dn6 - (((var_t2_dn6 * assign59950_e97357) + (var_t2 * var_t2_dn6)) / (2.0 * assign59950_e97361)))), (0.5 * (var_t2_dn7 - (((var_t2_dn7 * assign59950_e97357) + (var_t2 * var_t2_dn7)) / (2.0 * assign59950_e97361)))), (0.5 * (var_t2_dn8 - (((var_t2_dn8 * assign59950_e97357) + (var_t2 * var_t2_dn8)) / (2.0 * assign59950_e97361)))), (0.5 * (var_t2_dn9 - (((var_t2_dn9 * assign59950_e97357) + (var_t2 * var_t2_dn9)) / (2.0 * assign59950_e97361)))), (0.5 * (var_t2_dn10 - (((var_t2_dn10 * assign59950_e97357) + (var_t2 * var_t2_dn10)) / (2.0 * assign59950_e97361)))), (0.5 * (var_t2_dn11 - (((var_t2_dn11 * assign59950_e97357) + (var_t2 * var_t2_dn11)) / (2.0 * assign59950_e97361)))),)
    } else {
        (var_t8, var_t8_dn3, var_t8_dn4, var_t8_dn5, var_t8_dn6, var_t8_dn7, var_t8_dn8, var_t8_dn9, var_t8_dn10, var_t8_dn11,)
    }
};
        var_t8 = assign59950_e97365;
        var_t8_dn3 = assign59950_e97365_d_n3;
        var_t8_dn4 = assign59950_e97365_d_n4;
        var_t8_dn5 = assign59950_e97365_d_n5;
        var_t8_dn6 = assign59950_e97365_d_n6;
        var_t8_dn7 = assign59950_e97365_d_n7;
        var_t8_dn8 = assign59950_e97365_d_n8;
        var_t8_dn9 = assign59950_e97365_d_n9;
        var_t8_dn10 = assign59950_e97365_d_n10;
        var_t8_dn11 = assign59950_e97365_d_n11;
        var_t8_rv = 0.0;

        let (assign59960_e97372, assign59960_e97372_d_n3, assign59960_e97372_d_n4, assign59960_e97372_d_n5, assign59960_e97372_d_n6, assign59960_e97372_d_n7, assign59960_e97372_d_n8, assign59960_e97372_d_n9, assign59960_e97372_d_n10, assign59960_e97372_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        (var_sqrtpsip, var_sqrtpsip_dn3, var_sqrtpsip_dn4, var_sqrtpsip_dn5, var_sqrtpsip_dn6, var_sqrtpsip_dn7, var_sqrtpsip_dn8, var_sqrtpsip_dn9, var_sqrtpsip_dn10, var_sqrtpsip_dn11,)
    } else {
        (var_sqrtpsisa, var_sqrtpsisa_dn3, var_sqrtpsisa_dn4, var_sqrtpsisa_dn5, var_sqrtpsisa_dn6, var_sqrtpsisa_dn7, var_sqrtpsisa_dn8, var_sqrtpsisa_dn9, var_sqrtpsisa_dn10, var_sqrtpsisa_dn11,)
    }
};
        var_sqrtpsisa = assign59960_e97372;
        var_sqrtpsisa_dn3 = assign59960_e97372_d_n3;
        var_sqrtpsisa_dn4 = assign59960_e97372_d_n4;
        var_sqrtpsisa_dn5 = assign59960_e97372_d_n5;
        var_sqrtpsisa_dn6 = assign59960_e97372_d_n6;
        var_sqrtpsisa_dn7 = assign59960_e97372_d_n7;
        var_sqrtpsisa_dn8 = assign59960_e97372_d_n8;
        var_sqrtpsisa_dn9 = assign59960_e97372_d_n9;
        var_sqrtpsisa_dn10 = assign59960_e97372_d_n10;
        var_sqrtpsisa_dn11 = assign59960_e97372_d_n11;
        var_sqrtpsisa_rv = 0.0;

        let assign59970_e97375: f64 = (-68.0);
        let assign59970_e97376: f64 = if var_t8 <= assign59970_e97375 { 1.0 } else { 0.0 };
        var_guard874 = assign59970_e97376;
        var_guard874_rv = 0.0;

        let (assign59980_e97386, assign59980_e97386_d_n3, assign59980_e97386_d_n4, assign59980_e97386_d_n5, assign59980_e97386_d_n6, assign59980_e97386_d_n7, assign59980_e97386_d_n8, assign59980_e97386_d_n9, assign59980_e97386_d_n10, assign59980_e97386_d_n11,) = {
    if (((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard874 != 0.0)) {
        let assign59980_e97384: f64 = (-100.0);
        (assign59980_e97384, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t4, var_t4_dn3, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_dn9, var_t4_dn10, var_t4_dn11,)
    }
};
        var_t4 = assign59980_e97386;
        var_t4_dn3 = assign59980_e97386_d_n3;
        var_t4_dn4 = assign59980_e97386_d_n4;
        var_t4_dn5 = assign59980_e97386_d_n5;
        var_t4_dn6 = assign59980_e97386_d_n6;
        var_t4_dn7 = assign59980_e97386_d_n7;
        var_t4_dn8 = assign59980_e97386_d_n8;
        var_t4_dn9 = assign59980_e97386_d_n9;
        var_t4_dn10 = assign59980_e97386_d_n10;
        var_t4_dn11 = assign59980_e97386_d_n11;
        var_t4_rv = 0.0;

        let (assign59990_e97395, assign59990_e97395_d_n3, assign59990_e97395_d_n4, assign59990_e97395_d_n5, assign59990_e97395_d_n6, assign59990_e97395_d_n7, assign59990_e97395_d_n8, assign59990_e97395_d_n9, assign59990_e97395_d_n10, assign59990_e97395_d_n11,) = {
    if (((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard874 != 0.0)) {
        (20.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t5, var_t5_dn3, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8, var_t5_dn9, var_t5_dn10, var_t5_dn11,)
    }
};
        var_t5 = assign59990_e97395;
        var_t5_dn3 = assign59990_e97395_d_n3;
        var_t5_dn4 = assign59990_e97395_d_n4;
        var_t5_dn5 = assign59990_e97395_d_n5;
        var_t5_dn6 = assign59990_e97395_d_n6;
        var_t5_dn7 = assign59990_e97395_d_n7;
        var_t5_dn8 = assign59990_e97395_d_n8;
        var_t5_dn9 = assign59990_e97395_d_n9;
        var_t5_dn10 = assign59990_e97395_d_n10;
        var_t5_dn11 = assign59990_e97395_d_n11;
        var_t5_rv = 0.0;

        let assign60000_e97400: f64 = (0.5 * var_t5);
        let assign60000_e97401: f64 = (var_t4 - assign60000_e97400);
        let assign60000_e97402: f64 = if var_t8 < assign60000_e97401 { 1.0 } else { 0.0 };
        var_guard875 = assign60000_e97402;
        var_guard875_rv = 0.0;

        *var_guard874_slot = var_guard874;
        *var_guard874_rv_slot = var_guard874_rv;
        *var_guard875_slot = var_guard875;
        *var_guard875_rv_slot = var_guard875_rv;
        *var_qs_edge_slot = var_qs_edge;
        *var_qs_edge_dn10_slot = var_qs_edge_dn10;
        *var_qs_edge_dn11_slot = var_qs_edge_dn11;
        *var_qs_edge_dn3_slot = var_qs_edge_dn3;
        *var_qs_edge_dn4_slot = var_qs_edge_dn4;
        *var_qs_edge_dn5_slot = var_qs_edge_dn5;
        *var_qs_edge_dn6_slot = var_qs_edge_dn6;
        *var_qs_edge_dn7_slot = var_qs_edge_dn7;
        *var_qs_edge_dn8_slot = var_qs_edge_dn8;
        *var_qs_edge_dn9_slot = var_qs_edge_dn9;
        *var_qs_edge_rv_slot = var_qs_edge_rv;
        *var_sqrtpsip_slot = var_sqrtpsip;
        *var_sqrtpsip_dn10_slot = var_sqrtpsip_dn10;
        *var_sqrtpsip_dn11_slot = var_sqrtpsip_dn11;
        *var_sqrtpsip_dn3_slot = var_sqrtpsip_dn3;
        *var_sqrtpsip_dn4_slot = var_sqrtpsip_dn4;
        *var_sqrtpsip_dn5_slot = var_sqrtpsip_dn5;
        *var_sqrtpsip_dn6_slot = var_sqrtpsip_dn6;
        *var_sqrtpsip_dn7_slot = var_sqrtpsip_dn7;
        *var_sqrtpsip_dn8_slot = var_sqrtpsip_dn8;
        *var_sqrtpsip_dn9_slot = var_sqrtpsip_dn9;
        *var_sqrtpsip_rv_slot = var_sqrtpsip_rv;
        *var_sqrtpsisa_slot = var_sqrtpsisa;
        *var_sqrtpsisa_dn10_slot = var_sqrtpsisa_dn10;
        *var_sqrtpsisa_dn11_slot = var_sqrtpsisa_dn11;
        *var_sqrtpsisa_dn3_slot = var_sqrtpsisa_dn3;
        *var_sqrtpsisa_dn4_slot = var_sqrtpsisa_dn4;
        *var_sqrtpsisa_dn5_slot = var_sqrtpsisa_dn5;
        *var_sqrtpsisa_dn6_slot = var_sqrtpsisa_dn6;
        *var_sqrtpsisa_dn7_slot = var_sqrtpsisa_dn7;
        *var_sqrtpsisa_dn8_slot = var_sqrtpsisa_dn8;
        *var_sqrtpsisa_dn9_slot = var_sqrtpsisa_dn9;
        *var_sqrtpsisa_rv_slot = var_sqrtpsisa_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_t0_rv_slot = var_t0_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t1_rv_slot = var_t1_rv;
        *var_t2_slot = var_t2;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_t2_rv_slot = var_t2_rv;
        *var_t4_slot = var_t4;
        *var_t4_dn10_slot = var_t4_dn10;
        *var_t4_dn11_slot = var_t4_dn11;
        *var_t4_dn3_slot = var_t4_dn3;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t4_dn9_slot = var_t4_dn9;
        *var_t4_rv_slot = var_t4_rv;
        *var_t5_slot = var_t5;
        *var_t5_dn10_slot = var_t5_dn10;
        *var_t5_dn11_slot = var_t5_dn11;
        *var_t5_dn3_slot = var_t5_dn3;
        *var_t5_dn4_slot = var_t5_dn4;
        *var_t5_dn5_slot = var_t5_dn5;
        *var_t5_dn6_slot = var_t5_dn6;
        *var_t5_dn7_slot = var_t5_dn7;
        *var_t5_dn8_slot = var_t5_dn8;
        *var_t5_dn9_slot = var_t5_dn9;
        *var_t5_rv_slot = var_t5_rv;
        *var_t6_slot = var_t6;
        *var_t6_dn10_slot = var_t6_dn10;
        *var_t6_dn11_slot = var_t6_dn11;
        *var_t6_dn3_slot = var_t6_dn3;
        *var_t6_dn4_slot = var_t6_dn4;
        *var_t6_dn5_slot = var_t6_dn5;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn7_slot = var_t6_dn7;
        *var_t6_dn8_slot = var_t6_dn8;
        *var_t6_dn9_slot = var_t6_dn9;
        *var_t6_rv_slot = var_t6_rv;
        *var_t7_slot = var_t7;
        *var_t7_dn10_slot = var_t7_dn10;
        *var_t7_dn11_slot = var_t7_dn11;
        *var_t7_dn3_slot = var_t7_dn3;
        *var_t7_dn4_slot = var_t7_dn4;
        *var_t7_dn5_slot = var_t7_dn5;
        *var_t7_dn6_slot = var_t7_dn6;
        *var_t7_dn7_slot = var_t7_dn7;
        *var_t7_dn8_slot = var_t7_dn8;
        *var_t7_dn9_slot = var_t7_dn9;
        *var_t7_rv_slot = var_t7_rv;
        *var_t8_slot = var_t8;
        *var_t8_dn10_slot = var_t8_dn10;
        *var_t8_dn11_slot = var_t8_dn11;
        *var_t8_dn3_slot = var_t8_dn3;
        *var_t8_dn4_slot = var_t8_dn4;
        *var_t8_dn5_slot = var_t8_dn5;
        *var_t8_dn6_slot = var_t8_dn6;
        *var_t8_dn7_slot = var_t8_dn7;
        *var_t8_dn8_slot = var_t8_dn8;
        *var_t8_dn9_slot = var_t8_dn9;
        *var_t8_rv_slot = var_t8_rv;
        *var_vdeff_slot = var_vdeff;
        *var_vdeff_dn10_slot = var_vdeff_dn10;
        *var_vdeff_dn11_slot = var_vdeff_dn11;
        *var_vdeff_dn3_slot = var_vdeff_dn3;
        *var_vdeff_dn4_slot = var_vdeff_dn4;
        *var_vdeff_dn5_slot = var_vdeff_dn5;
        *var_vdeff_dn6_slot = var_vdeff_dn6;
        *var_vdeff_dn7_slot = var_vdeff_dn7;
        *var_vdeff_dn8_slot = var_vdeff_dn8;
        *var_vdeff_dn9_slot = var_vdeff_dn9;
        *var_vdeff_rv_slot = var_vdeff_rv;
        *var_vdsatedge_slot = var_vdsatedge;
        *var_vdsatedge_1_slot = var_vdsatedge_1;
        *var_vdsatedge_1_dn10_slot = var_vdsatedge_1_dn10;
        *var_vdsatedge_1_dn11_slot = var_vdsatedge_1_dn11;
        *var_vdsatedge_1_dn3_slot = var_vdsatedge_1_dn3;
        *var_vdsatedge_1_dn4_slot = var_vdsatedge_1_dn4;
        *var_vdsatedge_1_dn5_slot = var_vdsatedge_1_dn5;
        *var_vdsatedge_1_dn6_slot = var_vdsatedge_1_dn6;
        *var_vdsatedge_1_dn7_slot = var_vdsatedge_1_dn7;
        *var_vdsatedge_1_dn8_slot = var_vdsatedge_1_dn8;
        *var_vdsatedge_1_dn9_slot = var_vdsatedge_1_dn9;
        *var_vdsatedge_1_rv_slot = var_vdsatedge_1_rv;
        *var_vdsatedge_dn10_slot = var_vdsatedge_dn10;
        *var_vdsatedge_dn11_slot = var_vdsatedge_dn11;
        *var_vdsatedge_dn3_slot = var_vdsatedge_dn3;
        *var_vdsatedge_dn4_slot = var_vdsatedge_dn4;
        *var_vdsatedge_dn5_slot = var_vdsatedge_dn5;
        *var_vdsatedge_dn6_slot = var_vdsatedge_dn6;
        *var_vdsatedge_dn7_slot = var_vdsatedge_dn7;
        *var_vdsatedge_dn8_slot = var_vdsatedge_dn8;
        *var_vdsatedge_dn9_slot = var_vdsatedge_dn9;
        *var_vdsatedge_rv_slot = var_vdsatedge_rv;
        *var_vdseff_slot = var_vdseff;
        *var_vdseff_dn10_slot = var_vdseff_dn10;
        *var_vdseff_dn11_slot = var_vdseff_dn11;
        *var_vdseff_dn3_slot = var_vdseff_dn3;
        *var_vdseff_dn4_slot = var_vdseff_dn4;
        *var_vdseff_dn5_slot = var_vdseff_dn5;
        *var_vdseff_dn6_slot = var_vdseff_dn6;
        *var_vdseff_dn7_slot = var_vdseff_dn7;
        *var_vdseff_dn8_slot = var_vdseff_dn8;
        *var_vdseff_dn9_slot = var_vdseff_dn9;
        *var_vdseff_rv_slot = var_vdseff_rv;
        *var_vdssate_slot = var_vdssate;
        *var_vdssate_dn10_slot = var_vdssate_dn10;
        *var_vdssate_dn11_slot = var_vdssate_dn11;
        *var_vdssate_dn3_slot = var_vdssate_dn3;
        *var_vdssate_dn4_slot = var_vdssate_dn4;
        *var_vdssate_dn5_slot = var_vdssate_dn5;
        *var_vdssate_dn6_slot = var_vdssate_dn6;
        *var_vdssate_dn7_slot = var_vdssate_dn7;
        *var_vdssate_dn8_slot = var_vdssate_dn8;
        *var_vdssate_dn9_slot = var_vdssate_dn9;
        *var_vdssate_rv_slot = var_vdssate_rv;
    }

    pub(super) fn stamp_reactive_block_210(
        var_guard492: f64,
        var_guard867: f64,
        var_guard874: f64,
        var_guard875: f64,
        var_psip: f64,
        var_psip_dn10: f64,
        var_psip_dn11: f64,
        var_psip_dn3: f64,
        var_psip_dn4: f64,
        var_psip_dn5: f64,
        var_psip_dn6: f64,
        var_psip_dn7: f64,
        var_psip_dn8: f64,
        var_psip_dn9: f64,
        var_qs_edge: f64,
        var_qs_edge_dn10: f64,
        var_qs_edge_dn11: f64,
        var_qs_edge_dn3: f64,
        var_qs_edge_dn4: f64,
        var_qs_edge_dn5: f64,
        var_qs_edge_dn6: f64,
        var_qs_edge_dn7: f64,
        var_qs_edge_dn8: f64,
        var_qs_edge_dn9: f64,
        var_sqrtpsisa: f64,
        var_sqrtpsisa_dn10: f64,
        var_sqrtpsisa_dn11: f64,
        var_sqrtpsisa_dn3: f64,
        var_sqrtpsisa_dn4: f64,
        var_sqrtpsisa_dn5: f64,
        var_sqrtpsisa_dn6: f64,
        var_sqrtpsisa_dn7: f64,
        var_sqrtpsisa_dn8: f64,
        var_sqrtpsisa_dn9: f64,
        var_t1: f64,
        var_t1_dn10: f64,
        var_t1_dn11: f64,
        var_t1_dn3: f64,
        var_t1_dn4: f64,
        var_t1_dn5: f64,
        var_t1_dn6: f64,
        var_t1_dn7: f64,
        var_t1_dn8: f64,
        var_t1_dn9: f64,
        var_t8: f64,
        var_t8_dn10: f64,
        var_t8_dn11: f64,
        var_t8_dn3: f64,
        var_t8_dn4: f64,
        var_t8_dn5: f64,
        var_t8_dn6: f64,
        var_t8_dn7: f64,
        var_t8_dn8: f64,
        var_t8_dn9: f64,
        var_guard876_slot: &mut f64,
        var_guard876_rv_slot: &mut f64,
        var_psiavg_slot: &mut f64,
        var_psiavg_dn10_slot: &mut f64,
        var_psiavg_dn11_slot: &mut f64,
        var_psiavg_dn3_slot: &mut f64,
        var_psiavg_dn4_slot: &mut f64,
        var_psiavg_dn5_slot: &mut f64,
        var_psiavg_dn6_slot: &mut f64,
        var_psiavg_dn7_slot: &mut f64,
        var_psiavg_dn8_slot: &mut f64,
        var_psiavg_dn9_slot: &mut f64,
        var_psiavg_rv_slot: &mut f64,
        var_psipclamp_slot: &mut f64,
        var_psipclamp_dn10_slot: &mut f64,
        var_psipclamp_dn11_slot: &mut f64,
        var_psipclamp_dn3_slot: &mut f64,
        var_psipclamp_dn4_slot: &mut f64,
        var_psipclamp_dn5_slot: &mut f64,
        var_psipclamp_dn6_slot: &mut f64,
        var_psipclamp_dn7_slot: &mut f64,
        var_psipclamp_dn8_slot: &mut f64,
        var_psipclamp_dn9_slot: &mut f64,
        var_psipclamp_rv_slot: &mut f64,
        var_qdeff_edge_slot: &mut f64,
        var_qdeff_edge_dn10_slot: &mut f64,
        var_qdeff_edge_dn11_slot: &mut f64,
        var_qdeff_edge_dn3_slot: &mut f64,
        var_qdeff_edge_dn4_slot: &mut f64,
        var_qdeff_edge_dn5_slot: &mut f64,
        var_qdeff_edge_dn6_slot: &mut f64,
        var_qdeff_edge_dn7_slot: &mut f64,
        var_qdeff_edge_dn8_slot: &mut f64,
        var_qdeff_edge_dn9_slot: &mut f64,
        var_qdeff_edge_rv_slot: &mut f64,
        var_sqrtpsip_slot: &mut f64,
        var_sqrtpsip_dn10_slot: &mut f64,
        var_sqrtpsip_dn11_slot: &mut f64,
        var_sqrtpsip_dn3_slot: &mut f64,
        var_sqrtpsip_dn4_slot: &mut f64,
        var_sqrtpsip_dn5_slot: &mut f64,
        var_sqrtpsip_dn6_slot: &mut f64,
        var_sqrtpsip_dn7_slot: &mut f64,
        var_sqrtpsip_dn8_slot: &mut f64,
        var_sqrtpsip_dn9_slot: &mut f64,
        var_sqrtpsip_rv_slot: &mut f64,
        var_sqrtpsisainv_slot: &mut f64,
        var_sqrtpsisainv_dn10_slot: &mut f64,
        var_sqrtpsisainv_dn11_slot: &mut f64,
        var_sqrtpsisainv_dn3_slot: &mut f64,
        var_sqrtpsisainv_dn4_slot: &mut f64,
        var_sqrtpsisainv_dn5_slot: &mut f64,
        var_sqrtpsisainv_dn6_slot: &mut f64,
        var_sqrtpsisainv_dn7_slot: &mut f64,
        var_sqrtpsisainv_dn8_slot: &mut f64,
        var_sqrtpsisainv_dn9_slot: &mut f64,
        var_sqrtpsisainv_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_dn9_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn10_slot: &mut f64,
        var_t4_dn11_slot: &mut f64,
        var_t4_dn3_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t4_dn9_slot: &mut f64,
        var_t4_rv_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_dn10_slot: &mut f64,
        var_t5_dn11_slot: &mut f64,
        var_t5_dn3_slot: &mut f64,
        var_t5_dn4_slot: &mut f64,
        var_t5_dn5_slot: &mut f64,
        var_t5_dn6_slot: &mut f64,
        var_t5_dn7_slot: &mut f64,
        var_t5_dn8_slot: &mut f64,
        var_t5_dn9_slot: &mut f64,
        var_t5_rv_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_dn10_slot: &mut f64,
        var_t6_dn11_slot: &mut f64,
        var_t6_dn3_slot: &mut f64,
        var_t6_dn4_slot: &mut f64,
        var_t6_dn5_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn7_slot: &mut f64,
        var_t6_dn8_slot: &mut f64,
        var_t6_dn9_slot: &mut f64,
        var_t6_rv_slot: &mut f64,
        var_t7_slot: &mut f64,
        var_t7_dn10_slot: &mut f64,
        var_t7_dn11_slot: &mut f64,
        var_t7_dn3_slot: &mut f64,
        var_t7_dn4_slot: &mut f64,
        var_t7_dn5_slot: &mut f64,
        var_t7_dn6_slot: &mut f64,
        var_t7_dn7_slot: &mut f64,
        var_t7_dn8_slot: &mut f64,
        var_t7_dn9_slot: &mut f64,
        var_t7_rv_slot: &mut f64,
    ) {
        let mut var_guard876: f64 = *var_guard876_slot;
        let mut var_guard876_rv: f64 = *var_guard876_rv_slot;
        let mut var_psiavg: f64 = *var_psiavg_slot;
        let mut var_psiavg_dn10: f64 = *var_psiavg_dn10_slot;
        let mut var_psiavg_dn11: f64 = *var_psiavg_dn11_slot;
        let mut var_psiavg_dn3: f64 = *var_psiavg_dn3_slot;
        let mut var_psiavg_dn4: f64 = *var_psiavg_dn4_slot;
        let mut var_psiavg_dn5: f64 = *var_psiavg_dn5_slot;
        let mut var_psiavg_dn6: f64 = *var_psiavg_dn6_slot;
        let mut var_psiavg_dn7: f64 = *var_psiavg_dn7_slot;
        let mut var_psiavg_dn8: f64 = *var_psiavg_dn8_slot;
        let mut var_psiavg_dn9: f64 = *var_psiavg_dn9_slot;
        let mut var_psiavg_rv: f64 = *var_psiavg_rv_slot;
        let mut var_psipclamp: f64 = *var_psipclamp_slot;
        let mut var_psipclamp_dn10: f64 = *var_psipclamp_dn10_slot;
        let mut var_psipclamp_dn11: f64 = *var_psipclamp_dn11_slot;
        let mut var_psipclamp_dn3: f64 = *var_psipclamp_dn3_slot;
        let mut var_psipclamp_dn4: f64 = *var_psipclamp_dn4_slot;
        let mut var_psipclamp_dn5: f64 = *var_psipclamp_dn5_slot;
        let mut var_psipclamp_dn6: f64 = *var_psipclamp_dn6_slot;
        let mut var_psipclamp_dn7: f64 = *var_psipclamp_dn7_slot;
        let mut var_psipclamp_dn8: f64 = *var_psipclamp_dn8_slot;
        let mut var_psipclamp_dn9: f64 = *var_psipclamp_dn9_slot;
        let mut var_psipclamp_rv: f64 = *var_psipclamp_rv_slot;
        let mut var_qdeff_edge: f64 = *var_qdeff_edge_slot;
        let mut var_qdeff_edge_dn10: f64 = *var_qdeff_edge_dn10_slot;
        let mut var_qdeff_edge_dn11: f64 = *var_qdeff_edge_dn11_slot;
        let mut var_qdeff_edge_dn3: f64 = *var_qdeff_edge_dn3_slot;
        let mut var_qdeff_edge_dn4: f64 = *var_qdeff_edge_dn4_slot;
        let mut var_qdeff_edge_dn5: f64 = *var_qdeff_edge_dn5_slot;
        let mut var_qdeff_edge_dn6: f64 = *var_qdeff_edge_dn6_slot;
        let mut var_qdeff_edge_dn7: f64 = *var_qdeff_edge_dn7_slot;
        let mut var_qdeff_edge_dn8: f64 = *var_qdeff_edge_dn8_slot;
        let mut var_qdeff_edge_dn9: f64 = *var_qdeff_edge_dn9_slot;
        let mut var_qdeff_edge_rv: f64 = *var_qdeff_edge_rv_slot;
        let mut var_sqrtpsip: f64 = *var_sqrtpsip_slot;
        let mut var_sqrtpsip_dn10: f64 = *var_sqrtpsip_dn10_slot;
        let mut var_sqrtpsip_dn11: f64 = *var_sqrtpsip_dn11_slot;
        let mut var_sqrtpsip_dn3: f64 = *var_sqrtpsip_dn3_slot;
        let mut var_sqrtpsip_dn4: f64 = *var_sqrtpsip_dn4_slot;
        let mut var_sqrtpsip_dn5: f64 = *var_sqrtpsip_dn5_slot;
        let mut var_sqrtpsip_dn6: f64 = *var_sqrtpsip_dn6_slot;
        let mut var_sqrtpsip_dn7: f64 = *var_sqrtpsip_dn7_slot;
        let mut var_sqrtpsip_dn8: f64 = *var_sqrtpsip_dn8_slot;
        let mut var_sqrtpsip_dn9: f64 = *var_sqrtpsip_dn9_slot;
        let mut var_sqrtpsip_rv: f64 = *var_sqrtpsip_rv_slot;
        let mut var_sqrtpsisainv: f64 = *var_sqrtpsisainv_slot;
        let mut var_sqrtpsisainv_dn10: f64 = *var_sqrtpsisainv_dn10_slot;
        let mut var_sqrtpsisainv_dn11: f64 = *var_sqrtpsisainv_dn11_slot;
        let mut var_sqrtpsisainv_dn3: f64 = *var_sqrtpsisainv_dn3_slot;
        let mut var_sqrtpsisainv_dn4: f64 = *var_sqrtpsisainv_dn4_slot;
        let mut var_sqrtpsisainv_dn5: f64 = *var_sqrtpsisainv_dn5_slot;
        let mut var_sqrtpsisainv_dn6: f64 = *var_sqrtpsisainv_dn6_slot;
        let mut var_sqrtpsisainv_dn7: f64 = *var_sqrtpsisainv_dn7_slot;
        let mut var_sqrtpsisainv_dn8: f64 = *var_sqrtpsisainv_dn8_slot;
        let mut var_sqrtpsisainv_dn9: f64 = *var_sqrtpsisainv_dn9_slot;
        let mut var_sqrtpsisainv_rv: f64 = *var_sqrtpsisainv_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_dn9: f64 = *var_t3_dn9_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn10: f64 = *var_t4_dn10_slot;
        let mut var_t4_dn11: f64 = *var_t4_dn11_slot;
        let mut var_t4_dn3: f64 = *var_t4_dn3_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t4_dn9: f64 = *var_t4_dn9_slot;
        let mut var_t4_rv: f64 = *var_t4_rv_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_dn10: f64 = *var_t5_dn10_slot;
        let mut var_t5_dn11: f64 = *var_t5_dn11_slot;
        let mut var_t5_dn3: f64 = *var_t5_dn3_slot;
        let mut var_t5_dn4: f64 = *var_t5_dn4_slot;
        let mut var_t5_dn5: f64 = *var_t5_dn5_slot;
        let mut var_t5_dn6: f64 = *var_t5_dn6_slot;
        let mut var_t5_dn7: f64 = *var_t5_dn7_slot;
        let mut var_t5_dn8: f64 = *var_t5_dn8_slot;
        let mut var_t5_dn9: f64 = *var_t5_dn9_slot;
        let mut var_t5_rv: f64 = *var_t5_rv_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_dn10: f64 = *var_t6_dn10_slot;
        let mut var_t6_dn11: f64 = *var_t6_dn11_slot;
        let mut var_t6_dn3: f64 = *var_t6_dn3_slot;
        let mut var_t6_dn4: f64 = *var_t6_dn4_slot;
        let mut var_t6_dn5: f64 = *var_t6_dn5_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn7: f64 = *var_t6_dn7_slot;
        let mut var_t6_dn8: f64 = *var_t6_dn8_slot;
        let mut var_t6_dn9: f64 = *var_t6_dn9_slot;
        let mut var_t6_rv: f64 = *var_t6_rv_slot;
        let mut var_t7: f64 = *var_t7_slot;
        let mut var_t7_dn10: f64 = *var_t7_dn10_slot;
        let mut var_t7_dn11: f64 = *var_t7_dn11_slot;
        let mut var_t7_dn3: f64 = *var_t7_dn3_slot;
        let mut var_t7_dn4: f64 = *var_t7_dn4_slot;
        let mut var_t7_dn5: f64 = *var_t7_dn5_slot;
        let mut var_t7_dn6: f64 = *var_t7_dn6_slot;
        let mut var_t7_dn7: f64 = *var_t7_dn7_slot;
        let mut var_t7_dn8: f64 = *var_t7_dn8_slot;
        let mut var_t7_dn9: f64 = *var_t7_dn9_slot;
        let mut var_t7_rv: f64 = *var_t7_rv_slot;

        let (assign60010_e97414, assign60010_e97414_d_n3, assign60010_e97414_d_n4, assign60010_e97414_d_n5, assign60010_e97414_d_n6, assign60010_e97414_d_n7, assign60010_e97414_d_n8, assign60010_e97414_d_n9, assign60010_e97414_d_n10, assign60010_e97414_d_n11,) = {
    if ((((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard874 != 0.0)) && (var_guard875 != 0.0)) {
        let assign60010_e97412: f64 = { let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign60010_e97412, ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn3), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn4), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn5), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn6), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn7), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn8), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn9), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn10), ({ let limited_exp_arg = var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t4_dn11),)
    } else {
        (var_t3, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11,)
    }
};
        var_t3 = assign60010_e97414;
        var_t3_dn3 = assign60010_e97414_d_n3;
        var_t3_dn4 = assign60010_e97414_d_n4;
        var_t3_dn5 = assign60010_e97414_d_n5;
        var_t3_dn6 = assign60010_e97414_d_n6;
        var_t3_dn7 = assign60010_e97414_d_n7;
        var_t3_dn8 = assign60010_e97414_d_n8;
        var_t3_dn9 = assign60010_e97414_d_n9;
        var_t3_dn10 = assign60010_e97414_d_n10;
        var_t3_dn11 = assign60010_e97414_d_n11;
        var_t3_rv = 0.0;

        let assign60020_e97419: f64 = (0.5 * var_t5);
        let assign60020_e97420: f64 = (var_t4 + assign60020_e97419);
        let assign60020_e97421: f64 = if var_t8 > assign60020_e97420 { 1.0 } else { 0.0 };
        var_guard876 = assign60020_e97421;
        var_guard876_rv = 0.0;

        let (assign60030_e97436, assign60030_e97436_d_n3, assign60030_e97436_d_n4, assign60030_e97436_d_n5, assign60030_e97436_d_n6, assign60030_e97436_d_n7, assign60030_e97436_d_n8, assign60030_e97436_d_n9, assign60030_e97436_d_n10, assign60030_e97436_d_n11,) = {
    if (((((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard874 != 0.0)) && (var_guard875 == 0.0)) && (var_guard876 != 0.0)) {
        let assign60030_e97434: f64 = { let limited_exp_arg = var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign60030_e97434, ({ let limited_exp_arg = var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t8_dn3), ({ let limited_exp_arg = var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t8_dn4), ({ let limited_exp_arg = var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t8_dn5), ({ let limited_exp_arg = var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t8_dn6), ({ let limited_exp_arg = var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t8_dn7), ({ let limited_exp_arg = var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t8_dn8), ({ let limited_exp_arg = var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t8_dn9), ({ let limited_exp_arg = var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t8_dn10), ({ let limited_exp_arg = var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t8_dn11),)
    } else {
        (var_t3, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11,)
    }
};
        var_t3 = assign60030_e97436;
        var_t3_dn3 = assign60030_e97436_d_n3;
        var_t3_dn4 = assign60030_e97436_d_n4;
        var_t3_dn5 = assign60030_e97436_d_n5;
        var_t3_dn6 = assign60030_e97436_d_n6;
        var_t3_dn7 = assign60030_e97436_d_n7;
        var_t3_dn8 = assign60030_e97436_d_n8;
        var_t3_dn9 = assign60030_e97436_d_n9;
        var_t3_dn10 = assign60030_e97436_d_n10;
        var_t3_dn11 = assign60030_e97436_d_n11;
        var_t3_rv = 0.0;

        let (assign60040_e97455, assign60040_e97455_d_n3, assign60040_e97455_d_n4, assign60040_e97455_d_n5, assign60040_e97455_d_n6, assign60040_e97455_d_n7, assign60040_e97455_d_n8, assign60040_e97455_d_n9, assign60040_e97455_d_n10, assign60040_e97455_d_n11,) = {
    if (((((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard874 != 0.0)) && (var_guard875 == 0.0)) && (var_guard876 == 0.0)) {
        let assign60040_e97451: f64 = (var_t8 - var_t4);
        let assign60040_e97453: f64 = (assign60040_e97451 / var_t5);
        (assign60040_e97453, ((((var_t8_dn3 - var_t4_dn3) * var_t5) - (assign60040_e97451 * var_t5_dn3)) / (var_t5 * var_t5)), ((((var_t8_dn4 - var_t4_dn4) * var_t5) - (assign60040_e97451 * var_t5_dn4)) / (var_t5 * var_t5)), ((((var_t8_dn5 - var_t4_dn5) * var_t5) - (assign60040_e97451 * var_t5_dn5)) / (var_t5 * var_t5)), ((((var_t8_dn6 - var_t4_dn6) * var_t5) - (assign60040_e97451 * var_t5_dn6)) / (var_t5 * var_t5)), ((((var_t8_dn7 - var_t4_dn7) * var_t5) - (assign60040_e97451 * var_t5_dn7)) / (var_t5 * var_t5)), ((((var_t8_dn8 - var_t4_dn8) * var_t5) - (assign60040_e97451 * var_t5_dn8)) / (var_t5 * var_t5)), ((((var_t8_dn9 - var_t4_dn9) * var_t5) - (assign60040_e97451 * var_t5_dn9)) / (var_t5 * var_t5)), ((((var_t8_dn10 - var_t4_dn10) * var_t5) - (assign60040_e97451 * var_t5_dn10)) / (var_t5 * var_t5)), ((((var_t8_dn11 - var_t4_dn11) * var_t5) - (assign60040_e97451 * var_t5_dn11)) / (var_t5 * var_t5)),)
    } else {
        (var_t2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11,)
    }
};
        var_t2 = assign60040_e97455;
        var_t2_dn3 = assign60040_e97455_d_n3;
        var_t2_dn4 = assign60040_e97455_d_n4;
        var_t2_dn5 = assign60040_e97455_d_n5;
        var_t2_dn6 = assign60040_e97455_d_n6;
        var_t2_dn7 = assign60040_e97455_d_n7;
        var_t2_dn8 = assign60040_e97455_d_n8;
        var_t2_dn9 = assign60040_e97455_d_n9;
        var_t2_dn10 = assign60040_e97455_d_n10;
        var_t2_dn11 = assign60040_e97455_d_n11;
        var_t2_rv = 0.0;

        let (assign60050_e97472, assign60050_e97472_d_n3, assign60050_e97472_d_n4, assign60050_e97472_d_n5, assign60050_e97472_d_n6, assign60050_e97472_d_n7, assign60050_e97472_d_n8, assign60050_e97472_d_n9, assign60050_e97472_d_n10, assign60050_e97472_d_n11,) = {
    if (((((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard874 != 0.0)) && (var_guard875 == 0.0)) && (var_guard876 == 0.0)) {
        let assign60050_e97470: f64 = (var_t2 * var_t2);
        (assign60050_e97470, ((var_t2_dn3 * var_t2) + (var_t2 * var_t2_dn3)), ((var_t2_dn4 * var_t2) + (var_t2 * var_t2_dn4)), ((var_t2_dn5 * var_t2) + (var_t2 * var_t2_dn5)), ((var_t2_dn6 * var_t2) + (var_t2 * var_t2_dn6)), ((var_t2_dn7 * var_t2) + (var_t2 * var_t2_dn7)), ((var_t2_dn8 * var_t2) + (var_t2 * var_t2_dn8)), ((var_t2_dn9 * var_t2) + (var_t2 * var_t2_dn9)), ((var_t2_dn10 * var_t2) + (var_t2 * var_t2_dn10)), ((var_t2_dn11 * var_t2) + (var_t2 * var_t2_dn11)),)
    } else {
        (var_t6, var_t6_dn3, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_dn9, var_t6_dn10, var_t6_dn11,)
    }
};
        var_t6 = assign60050_e97472;
        var_t6_dn3 = assign60050_e97472_d_n3;
        var_t6_dn4 = assign60050_e97472_d_n4;
        var_t6_dn5 = assign60050_e97472_d_n5;
        var_t6_dn6 = assign60050_e97472_d_n6;
        var_t6_dn7 = assign60050_e97472_d_n7;
        var_t6_dn8 = assign60050_e97472_d_n8;
        var_t6_dn9 = assign60050_e97472_d_n9;
        var_t6_dn10 = assign60050_e97472_d_n10;
        var_t6_dn11 = assign60050_e97472_d_n11;
        var_t6_rv = 0.0;

        let (assign60060_e97510, assign60060_e97510_d_n3, assign60060_e97510_d_n4, assign60060_e97510_d_n5, assign60060_e97510_d_n6, assign60060_e97510_d_n7, assign60060_e97510_d_n8, assign60060_e97510_d_n9, assign60060_e97510_d_n10, assign60060_e97510_d_n11,) = {
    if (((((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard874 != 0.0)) && (var_guard875 == 0.0)) && (var_guard876 == 0.0)) {
        let assign60060_e97489: f64 = (5.0 / 64.0);
        let assign60060_e97492: f64 = (0.5 * var_t2);
        let assign60060_e97493: f64 = (assign60060_e97489 + assign60060_e97492);
        let assign60060_e97497: f64 = (15.0 / 16.0);
        let assign60060_e97501: f64 = (1.25 - var_t6);
        let assign60060_e97502: f64 = (var_t6 * assign60060_e97501);
        let assign60060_e97503: f64 = (assign60060_e97497 - assign60060_e97502);
        let assign60060_e97504: f64 = (var_t6 * assign60060_e97503);
        let assign60060_e97505: f64 = (assign60060_e97493 + assign60060_e97504);
        let assign60060_e97506: f64 = (var_t5 * assign60060_e97505);
        let assign60060_e97507: f64 = (var_t4 + assign60060_e97506);
        let assign60060_e97508: f64 = { let limited_exp_arg = assign60060_e97507; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign60060_e97508, ({ let limited_exp_arg = assign60060_e97507; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t4_dn3 + ((var_t5_dn3 * assign60060_e97505) + (var_t5 * ((0.5 * var_t2_dn3) + ((var_t6_dn3 * assign60060_e97503) + (var_t6 * (-((var_t6_dn3 * assign60060_e97501) + (var_t6 * (-var_t6_dn3))))))))))), ({ let limited_exp_arg = assign60060_e97507; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t4_dn4 + ((var_t5_dn4 * assign60060_e97505) + (var_t5 * ((0.5 * var_t2_dn4) + ((var_t6_dn4 * assign60060_e97503) + (var_t6 * (-((var_t6_dn4 * assign60060_e97501) + (var_t6 * (-var_t6_dn4))))))))))), ({ let limited_exp_arg = assign60060_e97507; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t4_dn5 + ((var_t5_dn5 * assign60060_e97505) + (var_t5 * ((0.5 * var_t2_dn5) + ((var_t6_dn5 * assign60060_e97503) + (var_t6 * (-((var_t6_dn5 * assign60060_e97501) + (var_t6 * (-var_t6_dn5))))))))))), ({ let limited_exp_arg = assign60060_e97507; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t4_dn6 + ((var_t5_dn6 * assign60060_e97505) + (var_t5 * ((0.5 * var_t2_dn6) + ((var_t6_dn6 * assign60060_e97503) + (var_t6 * (-((var_t6_dn6 * assign60060_e97501) + (var_t6 * (-var_t6_dn6))))))))))), ({ let limited_exp_arg = assign60060_e97507; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t4_dn7 + ((var_t5_dn7 * assign60060_e97505) + (var_t5 * ((0.5 * var_t2_dn7) + ((var_t6_dn7 * assign60060_e97503) + (var_t6 * (-((var_t6_dn7 * assign60060_e97501) + (var_t6 * (-var_t6_dn7))))))))))), ({ let limited_exp_arg = assign60060_e97507; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t4_dn8 + ((var_t5_dn8 * assign60060_e97505) + (var_t5 * ((0.5 * var_t2_dn8) + ((var_t6_dn8 * assign60060_e97503) + (var_t6 * (-((var_t6_dn8 * assign60060_e97501) + (var_t6 * (-var_t6_dn8))))))))))), ({ let limited_exp_arg = assign60060_e97507; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t4_dn9 + ((var_t5_dn9 * assign60060_e97505) + (var_t5 * ((0.5 * var_t2_dn9) + ((var_t6_dn9 * assign60060_e97503) + (var_t6 * (-((var_t6_dn9 * assign60060_e97501) + (var_t6 * (-var_t6_dn9))))))))))), ({ let limited_exp_arg = assign60060_e97507; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t4_dn10 + ((var_t5_dn10 * assign60060_e97505) + (var_t5 * ((0.5 * var_t2_dn10) + ((var_t6_dn10 * assign60060_e97503) + (var_t6 * (-((var_t6_dn10 * assign60060_e97501) + (var_t6 * (-var_t6_dn10))))))))))), ({ let limited_exp_arg = assign60060_e97507; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t4_dn11 + ((var_t5_dn11 * assign60060_e97505) + (var_t5 * ((0.5 * var_t2_dn11) + ((var_t6_dn11 * assign60060_e97503) + (var_t6 * (-((var_t6_dn11 * assign60060_e97501) + (var_t6 * (-var_t6_dn11))))))))))),)
    } else {
        (var_t3, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11,)
    }
};
        var_t3 = assign60060_e97510;
        var_t3_dn3 = assign60060_e97510_d_n3;
        var_t3_dn4 = assign60060_e97510_d_n4;
        var_t3_dn5 = assign60060_e97510_d_n5;
        var_t3_dn6 = assign60060_e97510_d_n6;
        var_t3_dn7 = assign60060_e97510_d_n7;
        var_t3_dn8 = assign60060_e97510_d_n8;
        var_t3_dn9 = assign60060_e97510_d_n9;
        var_t3_dn10 = assign60060_e97510_d_n10;
        var_t3_dn11 = assign60060_e97510_d_n11;
        var_t3_rv = 0.0;

        let (assign60070_e97542, assign60070_e97542_d_n3, assign60070_e97542_d_n4, assign60070_e97542_d_n5, assign60070_e97542_d_n6, assign60070_e97542_d_n7, assign60070_e97542_d_n8, assign60070_e97542_d_n9, assign60070_e97542_d_n10, assign60070_e97542_d_n11,) = {
    if (((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard874 != 0.0)) {
        let assign60070_e97520: f64 = (1.0 + var_t1);
        let assign60070_e97522: f64 = (assign60070_e97520 - var_t8);
        let assign60070_e97525: f64 = (2.0 * var_t0);
        let assign60070_e97528: f64 = (var_t3 * 2.0);
        let assign60070_e97530: f64 = (assign60070_e97528 * var_t0);
        let assign60070_e97533: f64 = (2.0 * var_sqrtpsisa);
        let assign60070_e97534: f64 = (assign60070_e97530 + assign60070_e97533);
        let assign60070_e97535: f64 = (assign60070_e97525 * assign60070_e97534);
        let assign60070_e97537: f64 = (assign60070_e97535).max(1e-38);
        let assign60070_e97538: f64 = (assign60070_e97537).ln();
        let assign60070_e97539: f64 = (assign60070_e97522 - assign60070_e97538);
        let assign60070_e97540: f64 = (var_t3 * assign60070_e97539);
        (assign60070_e97540, ((var_t3_dn3 * assign60070_e97539) + (var_t3 * ((var_t1_dn3 - var_t8_dn3) - (if assign60070_e97535 >= 1e-38 { (((2.0 * var_t0_dn3) * assign60070_e97534) + (assign60070_e97525 * ((((var_t3_dn3 * 2.0) * var_t0) + (assign60070_e97528 * var_t0_dn3)) + (2.0 * var_sqrtpsisa_dn3)))) } else { 0.0 } / assign60070_e97537)))), ((var_t3_dn4 * assign60070_e97539) + (var_t3 * ((var_t1_dn4 - var_t8_dn4) - (if assign60070_e97535 >= 1e-38 { (((2.0 * var_t0_dn4) * assign60070_e97534) + (assign60070_e97525 * ((((var_t3_dn4 * 2.0) * var_t0) + (assign60070_e97528 * var_t0_dn4)) + (2.0 * var_sqrtpsisa_dn4)))) } else { 0.0 } / assign60070_e97537)))), ((var_t3_dn5 * assign60070_e97539) + (var_t3 * ((var_t1_dn5 - var_t8_dn5) - (if assign60070_e97535 >= 1e-38 { (((2.0 * var_t0_dn5) * assign60070_e97534) + (assign60070_e97525 * ((((var_t3_dn5 * 2.0) * var_t0) + (assign60070_e97528 * var_t0_dn5)) + (2.0 * var_sqrtpsisa_dn5)))) } else { 0.0 } / assign60070_e97537)))), ((var_t3_dn6 * assign60070_e97539) + (var_t3 * ((var_t1_dn6 - var_t8_dn6) - (if assign60070_e97535 >= 1e-38 { (((2.0 * var_t0_dn6) * assign60070_e97534) + (assign60070_e97525 * ((((var_t3_dn6 * 2.0) * var_t0) + (assign60070_e97528 * var_t0_dn6)) + (2.0 * var_sqrtpsisa_dn6)))) } else { 0.0 } / assign60070_e97537)))), ((var_t3_dn7 * assign60070_e97539) + (var_t3 * ((var_t1_dn7 - var_t8_dn7) - (if assign60070_e97535 >= 1e-38 { (((2.0 * var_t0_dn7) * assign60070_e97534) + (assign60070_e97525 * ((((var_t3_dn7 * 2.0) * var_t0) + (assign60070_e97528 * var_t0_dn7)) + (2.0 * var_sqrtpsisa_dn7)))) } else { 0.0 } / assign60070_e97537)))), ((var_t3_dn8 * assign60070_e97539) + (var_t3 * ((var_t1_dn8 - var_t8_dn8) - (if assign60070_e97535 >= 1e-38 { (((2.0 * var_t0_dn8) * assign60070_e97534) + (assign60070_e97525 * ((((var_t3_dn8 * 2.0) * var_t0) + (assign60070_e97528 * var_t0_dn8)) + (2.0 * var_sqrtpsisa_dn8)))) } else { 0.0 } / assign60070_e97537)))), ((var_t3_dn9 * assign60070_e97539) + (var_t3 * ((var_t1_dn9 - var_t8_dn9) - (if assign60070_e97535 >= 1e-38 { (((2.0 * var_t0_dn9) * assign60070_e97534) + (assign60070_e97525 * ((((var_t3_dn9 * 2.0) * var_t0) + (assign60070_e97528 * var_t0_dn9)) + (2.0 * var_sqrtpsisa_dn9)))) } else { 0.0 } / assign60070_e97537)))), ((var_t3_dn10 * assign60070_e97539) + (var_t3 * ((var_t1_dn10 - var_t8_dn10) - (if assign60070_e97535 >= 1e-38 { (((2.0 * var_t0_dn10) * assign60070_e97534) + (assign60070_e97525 * ((((var_t3_dn10 * 2.0) * var_t0) + (assign60070_e97528 * var_t0_dn10)) + (2.0 * var_sqrtpsisa_dn10)))) } else { 0.0 } / assign60070_e97537)))), ((var_t3_dn11 * assign60070_e97539) + (var_t3 * ((var_t1_dn11 - var_t8_dn11) - (if assign60070_e97535 >= 1e-38 { (((2.0 * var_t0_dn11) * assign60070_e97534) + (assign60070_e97525 * ((((var_t3_dn11 * 2.0) * var_t0) + (assign60070_e97528 * var_t0_dn11)) + (2.0 * var_sqrtpsisa_dn11)))) } else { 0.0 } / assign60070_e97537)))),)
    } else {
        (var_qdeff_edge, var_qdeff_edge_dn3, var_qdeff_edge_dn4, var_qdeff_edge_dn5, var_qdeff_edge_dn6, var_qdeff_edge_dn7, var_qdeff_edge_dn8, var_qdeff_edge_dn9, var_qdeff_edge_dn10, var_qdeff_edge_dn11,)
    }
};
        var_qdeff_edge = assign60070_e97542;
        var_qdeff_edge_dn3 = assign60070_e97542_d_n3;
        var_qdeff_edge_dn4 = assign60070_e97542_d_n4;
        var_qdeff_edge_dn5 = assign60070_e97542_d_n5;
        var_qdeff_edge_dn6 = assign60070_e97542_d_n6;
        var_qdeff_edge_dn7 = assign60070_e97542_d_n7;
        var_qdeff_edge_dn8 = assign60070_e97542_d_n8;
        var_qdeff_edge_dn9 = assign60070_e97542_d_n9;
        var_qdeff_edge_dn10 = assign60070_e97542_d_n10;
        var_qdeff_edge_dn11 = assign60070_e97542_d_n11;
        var_qdeff_edge_rv = 0.0;

        let (assign60080_e97553, assign60080_e97553_d_n3, assign60080_e97553_d_n4, assign60080_e97553_d_n5, assign60080_e97553_d_n6, assign60080_e97553_d_n7, assign60080_e97553_d_n8, assign60080_e97553_d_n9, assign60080_e97553_d_n10, assign60080_e97553_d_n11,) = {
    if (((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard874 == 0.0)) {
        let assign60080_e97551: f64 = { let limited_exp_arg = var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign60080_e97551, ({ let limited_exp_arg = var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t8_dn3), ({ let limited_exp_arg = var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t8_dn4), ({ let limited_exp_arg = var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t8_dn5), ({ let limited_exp_arg = var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t8_dn6), ({ let limited_exp_arg = var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t8_dn7), ({ let limited_exp_arg = var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t8_dn8), ({ let limited_exp_arg = var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t8_dn9), ({ let limited_exp_arg = var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t8_dn10), ({ let limited_exp_arg = var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_t8_dn11),)
    } else {
        (var_t3, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11,)
    }
};
        var_t3 = assign60080_e97553;
        var_t3_dn3 = assign60080_e97553_d_n3;
        var_t3_dn4 = assign60080_e97553_d_n4;
        var_t3_dn5 = assign60080_e97553_d_n5;
        var_t3_dn6 = assign60080_e97553_d_n6;
        var_t3_dn7 = assign60080_e97553_d_n7;
        var_t3_dn8 = assign60080_e97553_d_n8;
        var_t3_dn9 = assign60080_e97553_d_n9;
        var_t3_dn10 = assign60080_e97553_d_n10;
        var_t3_dn11 = assign60080_e97553_d_n11;
        var_t3_rv = 0.0;

        let (assign60090_e97565, assign60090_e97565_d_n3, assign60090_e97565_d_n4, assign60090_e97565_d_n5, assign60090_e97565_d_n6, assign60090_e97565_d_n7, assign60090_e97565_d_n8, assign60090_e97565_d_n9, assign60090_e97565_d_n10, assign60090_e97565_d_n11,) = {
    if (((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard874 == 0.0)) {
        let assign60090_e97563: f64 = (1.0 / var_sqrtpsisa);
        (assign60090_e97563, (-(var_sqrtpsisa_dn3 / (var_sqrtpsisa * var_sqrtpsisa))), (-(var_sqrtpsisa_dn4 / (var_sqrtpsisa * var_sqrtpsisa))), (-(var_sqrtpsisa_dn5 / (var_sqrtpsisa * var_sqrtpsisa))), (-(var_sqrtpsisa_dn6 / (var_sqrtpsisa * var_sqrtpsisa))), (-(var_sqrtpsisa_dn7 / (var_sqrtpsisa * var_sqrtpsisa))), (-(var_sqrtpsisa_dn8 / (var_sqrtpsisa * var_sqrtpsisa))), (-(var_sqrtpsisa_dn9 / (var_sqrtpsisa * var_sqrtpsisa))), (-(var_sqrtpsisa_dn10 / (var_sqrtpsisa * var_sqrtpsisa))), (-(var_sqrtpsisa_dn11 / (var_sqrtpsisa * var_sqrtpsisa))),)
    } else {
        (var_sqrtpsisainv, var_sqrtpsisainv_dn3, var_sqrtpsisainv_dn4, var_sqrtpsisainv_dn5, var_sqrtpsisainv_dn6, var_sqrtpsisainv_dn7, var_sqrtpsisainv_dn8, var_sqrtpsisainv_dn9, var_sqrtpsisainv_dn10, var_sqrtpsisainv_dn11,)
    }
};
        var_sqrtpsisainv = assign60090_e97565;
        var_sqrtpsisainv_dn3 = assign60090_e97565_d_n3;
        var_sqrtpsisainv_dn4 = assign60090_e97565_d_n4;
        var_sqrtpsisainv_dn5 = assign60090_e97565_d_n5;
        var_sqrtpsisainv_dn6 = assign60090_e97565_d_n6;
        var_sqrtpsisainv_dn7 = assign60090_e97565_d_n7;
        var_sqrtpsisainv_dn8 = assign60090_e97565_d_n8;
        var_sqrtpsisainv_dn9 = assign60090_e97565_d_n9;
        var_sqrtpsisainv_dn10 = assign60090_e97565_d_n10;
        var_sqrtpsisainv_dn11 = assign60090_e97565_d_n11;
        var_sqrtpsisainv_rv = 0.0;

        let (assign60100_e97598, assign60100_e97598_d_n3, assign60100_e97598_d_n4, assign60100_e97598_d_n5, assign60100_e97598_d_n6, assign60100_e97598_d_n7, assign60100_e97598_d_n8, assign60100_e97598_d_n9, assign60100_e97598_d_n10, assign60100_e97598_d_n11,) = {
    if (((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard874 == 0.0)) {
        let assign60100_e97575: f64 = (2.0 * var_t3);
        let assign60100_e97578: f64 = (var_t3 * 2.0);
        let assign60100_e97580: f64 = (assign60100_e97578 * var_t0);
        let assign60100_e97583: f64 = (var_t3 * 2.0);
        let assign60100_e97585: f64 = (assign60100_e97583 * var_t0);
        let assign60100_e97588: f64 = (2.0 * var_sqrtpsisa);
        let assign60100_e97589: f64 = (assign60100_e97585 + assign60100_e97588);
        let assign60100_e97590: f64 = (assign60100_e97580 * assign60100_e97589);
        let assign60100_e97592: f64 = (assign60100_e97590).max(1e-38);
        let assign60100_e97593: f64 = (assign60100_e97592).ln();
        let assign60100_e97594: f64 = (assign60100_e97575 + assign60100_e97593);
        let assign60100_e97596: f64 = (assign60100_e97594 - var_t1);
        (assign60100_e97596, (((2.0 * var_t3_dn3) + (if assign60100_e97590 >= 1e-38 { (((((var_t3_dn3 * 2.0) * var_t0) + (assign60100_e97578 * var_t0_dn3)) * assign60100_e97589) + (assign60100_e97580 * ((((var_t3_dn3 * 2.0) * var_t0) + (assign60100_e97583 * var_t0_dn3)) + (2.0 * var_sqrtpsisa_dn3)))) } else { 0.0 } / assign60100_e97592)) - var_t1_dn3), (((2.0 * var_t3_dn4) + (if assign60100_e97590 >= 1e-38 { (((((var_t3_dn4 * 2.0) * var_t0) + (assign60100_e97578 * var_t0_dn4)) * assign60100_e97589) + (assign60100_e97580 * ((((var_t3_dn4 * 2.0) * var_t0) + (assign60100_e97583 * var_t0_dn4)) + (2.0 * var_sqrtpsisa_dn4)))) } else { 0.0 } / assign60100_e97592)) - var_t1_dn4), (((2.0 * var_t3_dn5) + (if assign60100_e97590 >= 1e-38 { (((((var_t3_dn5 * 2.0) * var_t0) + (assign60100_e97578 * var_t0_dn5)) * assign60100_e97589) + (assign60100_e97580 * ((((var_t3_dn5 * 2.0) * var_t0) + (assign60100_e97583 * var_t0_dn5)) + (2.0 * var_sqrtpsisa_dn5)))) } else { 0.0 } / assign60100_e97592)) - var_t1_dn5), (((2.0 * var_t3_dn6) + (if assign60100_e97590 >= 1e-38 { (((((var_t3_dn6 * 2.0) * var_t0) + (assign60100_e97578 * var_t0_dn6)) * assign60100_e97589) + (assign60100_e97580 * ((((var_t3_dn6 * 2.0) * var_t0) + (assign60100_e97583 * var_t0_dn6)) + (2.0 * var_sqrtpsisa_dn6)))) } else { 0.0 } / assign60100_e97592)) - var_t1_dn6), (((2.0 * var_t3_dn7) + (if assign60100_e97590 >= 1e-38 { (((((var_t3_dn7 * 2.0) * var_t0) + (assign60100_e97578 * var_t0_dn7)) * assign60100_e97589) + (assign60100_e97580 * ((((var_t3_dn7 * 2.0) * var_t0) + (assign60100_e97583 * var_t0_dn7)) + (2.0 * var_sqrtpsisa_dn7)))) } else { 0.0 } / assign60100_e97592)) - var_t1_dn7), (((2.0 * var_t3_dn8) + (if assign60100_e97590 >= 1e-38 { (((((var_t3_dn8 * 2.0) * var_t0) + (assign60100_e97578 * var_t0_dn8)) * assign60100_e97589) + (assign60100_e97580 * ((((var_t3_dn8 * 2.0) * var_t0) + (assign60100_e97583 * var_t0_dn8)) + (2.0 * var_sqrtpsisa_dn8)))) } else { 0.0 } / assign60100_e97592)) - var_t1_dn8), (((2.0 * var_t3_dn9) + (if assign60100_e97590 >= 1e-38 { (((((var_t3_dn9 * 2.0) * var_t0) + (assign60100_e97578 * var_t0_dn9)) * assign60100_e97589) + (assign60100_e97580 * ((((var_t3_dn9 * 2.0) * var_t0) + (assign60100_e97583 * var_t0_dn9)) + (2.0 * var_sqrtpsisa_dn9)))) } else { 0.0 } / assign60100_e97592)) - var_t1_dn9), (((2.0 * var_t3_dn10) + (if assign60100_e97590 >= 1e-38 { (((((var_t3_dn10 * 2.0) * var_t0) + (assign60100_e97578 * var_t0_dn10)) * assign60100_e97589) + (assign60100_e97580 * ((((var_t3_dn10 * 2.0) * var_t0) + (assign60100_e97583 * var_t0_dn10)) + (2.0 * var_sqrtpsisa_dn10)))) } else { 0.0 } / assign60100_e97592)) - var_t1_dn10), (((2.0 * var_t3_dn11) + (if assign60100_e97590 >= 1e-38 { (((((var_t3_dn11 * 2.0) * var_t0) + (assign60100_e97578 * var_t0_dn11)) * assign60100_e97589) + (assign60100_e97580 * ((((var_t3_dn11 * 2.0) * var_t0) + (assign60100_e97583 * var_t0_dn11)) + (2.0 * var_sqrtpsisa_dn11)))) } else { 0.0 } / assign60100_e97592)) - var_t1_dn11),)
    } else {
        (var_t4, var_t4_dn3, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_dn9, var_t4_dn10, var_t4_dn11,)
    }
};
        var_t4 = assign60100_e97598;
        var_t4_dn3 = assign60100_e97598_d_n3;
        var_t4_dn4 = assign60100_e97598_d_n4;
        var_t4_dn5 = assign60100_e97598_d_n5;
        var_t4_dn6 = assign60100_e97598_d_n6;
        var_t4_dn7 = assign60100_e97598_d_n7;
        var_t4_dn8 = assign60100_e97598_d_n8;
        var_t4_dn9 = assign60100_e97598_d_n9;
        var_t4_dn10 = assign60100_e97598_d_n10;
        var_t4_dn11 = assign60100_e97598_d_n11;
        var_t4_rv = 0.0;

        let (assign60110_e97622, assign60110_e97622_d_n3, assign60110_e97622_d_n4, assign60110_e97622_d_n5, assign60110_e97622_d_n6, assign60110_e97622_d_n7, assign60110_e97622_d_n8, assign60110_e97622_d_n9, assign60110_e97622_d_n10, assign60110_e97622_d_n11,) = {
    if (((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard874 == 0.0)) {
        let assign60110_e97609: f64 = (1.0 / var_t3);
        let assign60110_e97610: f64 = (2.0 + assign60110_e97609);
        let assign60110_e97613: f64 = (var_t0 + var_sqrtpsisainv);
        let assign60110_e97616: f64 = (var_t0 * var_t3);
        let assign60110_e97618: f64 = (assign60110_e97616 + var_sqrtpsisa);
        let assign60110_e97619: f64 = (assign60110_e97613 / assign60110_e97618);
        let assign60110_e97620: f64 = (assign60110_e97610 + assign60110_e97619);
        (assign60110_e97620, ((-(var_t3_dn3 / (var_t3 * var_t3))) + ((((var_t0_dn3 + var_sqrtpsisainv_dn3) * assign60110_e97618) - (assign60110_e97613 * (((var_t0_dn3 * var_t3) + (var_t0 * var_t3_dn3)) + var_sqrtpsisa_dn3))) / (assign60110_e97618 * assign60110_e97618))), ((-(var_t3_dn4 / (var_t3 * var_t3))) + ((((var_t0_dn4 + var_sqrtpsisainv_dn4) * assign60110_e97618) - (assign60110_e97613 * (((var_t0_dn4 * var_t3) + (var_t0 * var_t3_dn4)) + var_sqrtpsisa_dn4))) / (assign60110_e97618 * assign60110_e97618))), ((-(var_t3_dn5 / (var_t3 * var_t3))) + ((((var_t0_dn5 + var_sqrtpsisainv_dn5) * assign60110_e97618) - (assign60110_e97613 * (((var_t0_dn5 * var_t3) + (var_t0 * var_t3_dn5)) + var_sqrtpsisa_dn5))) / (assign60110_e97618 * assign60110_e97618))), ((-(var_t3_dn6 / (var_t3 * var_t3))) + ((((var_t0_dn6 + var_sqrtpsisainv_dn6) * assign60110_e97618) - (assign60110_e97613 * (((var_t0_dn6 * var_t3) + (var_t0 * var_t3_dn6)) + var_sqrtpsisa_dn6))) / (assign60110_e97618 * assign60110_e97618))), ((-(var_t3_dn7 / (var_t3 * var_t3))) + ((((var_t0_dn7 + var_sqrtpsisainv_dn7) * assign60110_e97618) - (assign60110_e97613 * (((var_t0_dn7 * var_t3) + (var_t0 * var_t3_dn7)) + var_sqrtpsisa_dn7))) / (assign60110_e97618 * assign60110_e97618))), ((-(var_t3_dn8 / (var_t3 * var_t3))) + ((((var_t0_dn8 + var_sqrtpsisainv_dn8) * assign60110_e97618) - (assign60110_e97613 * (((var_t0_dn8 * var_t3) + (var_t0 * var_t3_dn8)) + var_sqrtpsisa_dn8))) / (assign60110_e97618 * assign60110_e97618))), ((-(var_t3_dn9 / (var_t3 * var_t3))) + ((((var_t0_dn9 + var_sqrtpsisainv_dn9) * assign60110_e97618) - (assign60110_e97613 * (((var_t0_dn9 * var_t3) + (var_t0 * var_t3_dn9)) + var_sqrtpsisa_dn9))) / (assign60110_e97618 * assign60110_e97618))), ((-(var_t3_dn10 / (var_t3 * var_t3))) + ((((var_t0_dn10 + var_sqrtpsisainv_dn10) * assign60110_e97618) - (assign60110_e97613 * (((var_t0_dn10 * var_t3) + (var_t0 * var_t3_dn10)) + var_sqrtpsisa_dn10))) / (assign60110_e97618 * assign60110_e97618))), ((-(var_t3_dn11 / (var_t3 * var_t3))) + ((((var_t0_dn11 + var_sqrtpsisainv_dn11) * assign60110_e97618) - (assign60110_e97613 * (((var_t0_dn11 * var_t3) + (var_t0 * var_t3_dn11)) + var_sqrtpsisa_dn11))) / (assign60110_e97618 * assign60110_e97618))),)
    } else {
        (var_t5, var_t5_dn3, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8, var_t5_dn9, var_t5_dn10, var_t5_dn11,)
    }
};
        var_t5 = assign60110_e97622;
        var_t5_dn3 = assign60110_e97622_d_n3;
        var_t5_dn4 = assign60110_e97622_d_n4;
        var_t5_dn5 = assign60110_e97622_d_n5;
        var_t5_dn6 = assign60110_e97622_d_n6;
        var_t5_dn7 = assign60110_e97622_d_n7;
        var_t5_dn8 = assign60110_e97622_d_n8;
        var_t5_dn9 = assign60110_e97622_d_n9;
        var_t5_dn10 = assign60110_e97622_d_n10;
        var_t5_dn11 = assign60110_e97622_d_n11;
        var_t5_rv = 0.0;

        let (assign60120_e97636, assign60120_e97636_d_n3, assign60120_e97636_d_n4, assign60120_e97636_d_n5, assign60120_e97636_d_n6, assign60120_e97636_d_n7, assign60120_e97636_d_n8, assign60120_e97636_d_n9, assign60120_e97636_d_n10, assign60120_e97636_d_n11,) = {
    if (((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard874 == 0.0)) {
        let assign60120_e97633: f64 = (var_t4 / var_t5);
        let assign60120_e97634: f64 = (var_t3 - assign60120_e97633);
        (assign60120_e97634, (var_t3_dn3 - (((var_t4_dn3 * var_t5) - (var_t4 * var_t5_dn3)) / (var_t5 * var_t5))), (var_t3_dn4 - (((var_t4_dn4 * var_t5) - (var_t4 * var_t5_dn4)) / (var_t5 * var_t5))), (var_t3_dn5 - (((var_t4_dn5 * var_t5) - (var_t4 * var_t5_dn5)) / (var_t5 * var_t5))), (var_t3_dn6 - (((var_t4_dn6 * var_t5) - (var_t4 * var_t5_dn6)) / (var_t5 * var_t5))), (var_t3_dn7 - (((var_t4_dn7 * var_t5) - (var_t4 * var_t5_dn7)) / (var_t5 * var_t5))), (var_t3_dn8 - (((var_t4_dn8 * var_t5) - (var_t4 * var_t5_dn8)) / (var_t5 * var_t5))), (var_t3_dn9 - (((var_t4_dn9 * var_t5) - (var_t4 * var_t5_dn9)) / (var_t5 * var_t5))), (var_t3_dn10 - (((var_t4_dn10 * var_t5) - (var_t4 * var_t5_dn10)) / (var_t5 * var_t5))), (var_t3_dn11 - (((var_t4_dn11 * var_t5) - (var_t4 * var_t5_dn11)) / (var_t5 * var_t5))),)
    } else {
        (var_t3, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11,)
    }
};
        var_t3 = assign60120_e97636;
        var_t3_dn3 = assign60120_e97636_d_n3;
        var_t3_dn4 = assign60120_e97636_d_n4;
        var_t3_dn5 = assign60120_e97636_d_n5;
        var_t3_dn6 = assign60120_e97636_d_n6;
        var_t3_dn7 = assign60120_e97636_d_n7;
        var_t3_dn8 = assign60120_e97636_d_n8;
        var_t3_dn9 = assign60120_e97636_d_n9;
        var_t3_dn10 = assign60120_e97636_d_n10;
        var_t3_dn11 = assign60120_e97636_d_n11;
        var_t3_rv = 0.0;

        let (assign60130_e97669, assign60130_e97669_d_n3, assign60130_e97669_d_n4, assign60130_e97669_d_n5, assign60130_e97669_d_n6, assign60130_e97669_d_n7, assign60130_e97669_d_n8, assign60130_e97669_d_n9, assign60130_e97669_d_n10, assign60130_e97669_d_n11,) = {
    if (((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard874 == 0.0)) {
        let assign60130_e97646: f64 = (2.0 * var_t3);
        let assign60130_e97649: f64 = (var_t3 * 2.0);
        let assign60130_e97651: f64 = (assign60130_e97649 * var_t0);
        let assign60130_e97654: f64 = (var_t3 * 2.0);
        let assign60130_e97656: f64 = (assign60130_e97654 * var_t0);
        let assign60130_e97659: f64 = (2.0 * var_sqrtpsisa);
        let assign60130_e97660: f64 = (assign60130_e97656 + assign60130_e97659);
        let assign60130_e97661: f64 = (assign60130_e97651 * assign60130_e97660);
        let assign60130_e97663: f64 = (assign60130_e97661).max(1e-38);
        let assign60130_e97664: f64 = (assign60130_e97663).ln();
        let assign60130_e97665: f64 = (assign60130_e97646 + assign60130_e97664);
        let assign60130_e97667: f64 = (assign60130_e97665 - var_t1);
        (assign60130_e97667, (((2.0 * var_t3_dn3) + (if assign60130_e97661 >= 1e-38 { (((((var_t3_dn3 * 2.0) * var_t0) + (assign60130_e97649 * var_t0_dn3)) * assign60130_e97660) + (assign60130_e97651 * ((((var_t3_dn3 * 2.0) * var_t0) + (assign60130_e97654 * var_t0_dn3)) + (2.0 * var_sqrtpsisa_dn3)))) } else { 0.0 } / assign60130_e97663)) - var_t1_dn3), (((2.0 * var_t3_dn4) + (if assign60130_e97661 >= 1e-38 { (((((var_t3_dn4 * 2.0) * var_t0) + (assign60130_e97649 * var_t0_dn4)) * assign60130_e97660) + (assign60130_e97651 * ((((var_t3_dn4 * 2.0) * var_t0) + (assign60130_e97654 * var_t0_dn4)) + (2.0 * var_sqrtpsisa_dn4)))) } else { 0.0 } / assign60130_e97663)) - var_t1_dn4), (((2.0 * var_t3_dn5) + (if assign60130_e97661 >= 1e-38 { (((((var_t3_dn5 * 2.0) * var_t0) + (assign60130_e97649 * var_t0_dn5)) * assign60130_e97660) + (assign60130_e97651 * ((((var_t3_dn5 * 2.0) * var_t0) + (assign60130_e97654 * var_t0_dn5)) + (2.0 * var_sqrtpsisa_dn5)))) } else { 0.0 } / assign60130_e97663)) - var_t1_dn5), (((2.0 * var_t3_dn6) + (if assign60130_e97661 >= 1e-38 { (((((var_t3_dn6 * 2.0) * var_t0) + (assign60130_e97649 * var_t0_dn6)) * assign60130_e97660) + (assign60130_e97651 * ((((var_t3_dn6 * 2.0) * var_t0) + (assign60130_e97654 * var_t0_dn6)) + (2.0 * var_sqrtpsisa_dn6)))) } else { 0.0 } / assign60130_e97663)) - var_t1_dn6), (((2.0 * var_t3_dn7) + (if assign60130_e97661 >= 1e-38 { (((((var_t3_dn7 * 2.0) * var_t0) + (assign60130_e97649 * var_t0_dn7)) * assign60130_e97660) + (assign60130_e97651 * ((((var_t3_dn7 * 2.0) * var_t0) + (assign60130_e97654 * var_t0_dn7)) + (2.0 * var_sqrtpsisa_dn7)))) } else { 0.0 } / assign60130_e97663)) - var_t1_dn7), (((2.0 * var_t3_dn8) + (if assign60130_e97661 >= 1e-38 { (((((var_t3_dn8 * 2.0) * var_t0) + (assign60130_e97649 * var_t0_dn8)) * assign60130_e97660) + (assign60130_e97651 * ((((var_t3_dn8 * 2.0) * var_t0) + (assign60130_e97654 * var_t0_dn8)) + (2.0 * var_sqrtpsisa_dn8)))) } else { 0.0 } / assign60130_e97663)) - var_t1_dn8), (((2.0 * var_t3_dn9) + (if assign60130_e97661 >= 1e-38 { (((((var_t3_dn9 * 2.0) * var_t0) + (assign60130_e97649 * var_t0_dn9)) * assign60130_e97660) + (assign60130_e97651 * ((((var_t3_dn9 * 2.0) * var_t0) + (assign60130_e97654 * var_t0_dn9)) + (2.0 * var_sqrtpsisa_dn9)))) } else { 0.0 } / assign60130_e97663)) - var_t1_dn9), (((2.0 * var_t3_dn10) + (if assign60130_e97661 >= 1e-38 { (((((var_t3_dn10 * 2.0) * var_t0) + (assign60130_e97649 * var_t0_dn10)) * assign60130_e97660) + (assign60130_e97651 * ((((var_t3_dn10 * 2.0) * var_t0) + (assign60130_e97654 * var_t0_dn10)) + (2.0 * var_sqrtpsisa_dn10)))) } else { 0.0 } / assign60130_e97663)) - var_t1_dn10), (((2.0 * var_t3_dn11) + (if assign60130_e97661 >= 1e-38 { (((((var_t3_dn11 * 2.0) * var_t0) + (assign60130_e97649 * var_t0_dn11)) * assign60130_e97660) + (assign60130_e97651 * ((((var_t3_dn11 * 2.0) * var_t0) + (assign60130_e97654 * var_t0_dn11)) + (2.0 * var_sqrtpsisa_dn11)))) } else { 0.0 } / assign60130_e97663)) - var_t1_dn11),)
    } else {
        (var_t4, var_t4_dn3, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_dn9, var_t4_dn10, var_t4_dn11,)
    }
};
        var_t4 = assign60130_e97669;
        var_t4_dn3 = assign60130_e97669_d_n3;
        var_t4_dn4 = assign60130_e97669_d_n4;
        var_t4_dn5 = assign60130_e97669_d_n5;
        var_t4_dn6 = assign60130_e97669_d_n6;
        var_t4_dn7 = assign60130_e97669_d_n7;
        var_t4_dn8 = assign60130_e97669_d_n8;
        var_t4_dn9 = assign60130_e97669_d_n9;
        var_t4_dn10 = assign60130_e97669_d_n10;
        var_t4_dn11 = assign60130_e97669_d_n11;
        var_t4_rv = 0.0;

        let (assign60140_e97693, assign60140_e97693_d_n3, assign60140_e97693_d_n4, assign60140_e97693_d_n5, assign60140_e97693_d_n6, assign60140_e97693_d_n7, assign60140_e97693_d_n8, assign60140_e97693_d_n9, assign60140_e97693_d_n10, assign60140_e97693_d_n11,) = {
    if (((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard874 == 0.0)) {
        let assign60140_e97680: f64 = (1.0 / var_t3);
        let assign60140_e97681: f64 = (2.0 + assign60140_e97680);
        let assign60140_e97684: f64 = (var_t0 + var_sqrtpsisainv);
        let assign60140_e97687: f64 = (var_t0 * var_t3);
        let assign60140_e97689: f64 = (assign60140_e97687 + var_sqrtpsisa);
        let assign60140_e97690: f64 = (assign60140_e97684 / assign60140_e97689);
        let assign60140_e97691: f64 = (assign60140_e97681 + assign60140_e97690);
        (assign60140_e97691, ((-(var_t3_dn3 / (var_t3 * var_t3))) + ((((var_t0_dn3 + var_sqrtpsisainv_dn3) * assign60140_e97689) - (assign60140_e97684 * (((var_t0_dn3 * var_t3) + (var_t0 * var_t3_dn3)) + var_sqrtpsisa_dn3))) / (assign60140_e97689 * assign60140_e97689))), ((-(var_t3_dn4 / (var_t3 * var_t3))) + ((((var_t0_dn4 + var_sqrtpsisainv_dn4) * assign60140_e97689) - (assign60140_e97684 * (((var_t0_dn4 * var_t3) + (var_t0 * var_t3_dn4)) + var_sqrtpsisa_dn4))) / (assign60140_e97689 * assign60140_e97689))), ((-(var_t3_dn5 / (var_t3 * var_t3))) + ((((var_t0_dn5 + var_sqrtpsisainv_dn5) * assign60140_e97689) - (assign60140_e97684 * (((var_t0_dn5 * var_t3) + (var_t0 * var_t3_dn5)) + var_sqrtpsisa_dn5))) / (assign60140_e97689 * assign60140_e97689))), ((-(var_t3_dn6 / (var_t3 * var_t3))) + ((((var_t0_dn6 + var_sqrtpsisainv_dn6) * assign60140_e97689) - (assign60140_e97684 * (((var_t0_dn6 * var_t3) + (var_t0 * var_t3_dn6)) + var_sqrtpsisa_dn6))) / (assign60140_e97689 * assign60140_e97689))), ((-(var_t3_dn7 / (var_t3 * var_t3))) + ((((var_t0_dn7 + var_sqrtpsisainv_dn7) * assign60140_e97689) - (assign60140_e97684 * (((var_t0_dn7 * var_t3) + (var_t0 * var_t3_dn7)) + var_sqrtpsisa_dn7))) / (assign60140_e97689 * assign60140_e97689))), ((-(var_t3_dn8 / (var_t3 * var_t3))) + ((((var_t0_dn8 + var_sqrtpsisainv_dn8) * assign60140_e97689) - (assign60140_e97684 * (((var_t0_dn8 * var_t3) + (var_t0 * var_t3_dn8)) + var_sqrtpsisa_dn8))) / (assign60140_e97689 * assign60140_e97689))), ((-(var_t3_dn9 / (var_t3 * var_t3))) + ((((var_t0_dn9 + var_sqrtpsisainv_dn9) * assign60140_e97689) - (assign60140_e97684 * (((var_t0_dn9 * var_t3) + (var_t0 * var_t3_dn9)) + var_sqrtpsisa_dn9))) / (assign60140_e97689 * assign60140_e97689))), ((-(var_t3_dn10 / (var_t3 * var_t3))) + ((((var_t0_dn10 + var_sqrtpsisainv_dn10) * assign60140_e97689) - (assign60140_e97684 * (((var_t0_dn10 * var_t3) + (var_t0 * var_t3_dn10)) + var_sqrtpsisa_dn10))) / (assign60140_e97689 * assign60140_e97689))), ((-(var_t3_dn11 / (var_t3 * var_t3))) + ((((var_t0_dn11 + var_sqrtpsisainv_dn11) * assign60140_e97689) - (assign60140_e97684 * (((var_t0_dn11 * var_t3) + (var_t0 * var_t3_dn11)) + var_sqrtpsisa_dn11))) / (assign60140_e97689 * assign60140_e97689))),)
    } else {
        (var_t5, var_t5_dn3, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8, var_t5_dn9, var_t5_dn10, var_t5_dn11,)
    }
};
        var_t5 = assign60140_e97693;
        var_t5_dn3 = assign60140_e97693_d_n3;
        var_t5_dn4 = assign60140_e97693_d_n4;
        var_t5_dn5 = assign60140_e97693_d_n5;
        var_t5_dn6 = assign60140_e97693_d_n6;
        var_t5_dn7 = assign60140_e97693_d_n7;
        var_t5_dn8 = assign60140_e97693_d_n8;
        var_t5_dn9 = assign60140_e97693_d_n9;
        var_t5_dn10 = assign60140_e97693_d_n10;
        var_t5_dn11 = assign60140_e97693_d_n11;
        var_t5_rv = 0.0;

        let (assign60150_e97721, assign60150_e97721_d_n3, assign60150_e97721_d_n4, assign60150_e97721_d_n5, assign60150_e97721_d_n6, assign60150_e97721_d_n7, assign60150_e97721_d_n8, assign60150_e97721_d_n9, assign60150_e97721_d_n10, assign60150_e97721_d_n11,) = {
    if (((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard874 == 0.0)) {
        let assign60150_e97703: f64 = (var_t0 + var_sqrtpsisainv);
        let assign60150_e97706: f64 = (var_t0 * var_t3);
        let assign60150_e97708: f64 = (assign60150_e97706 + var_sqrtpsisa);
        let assign60150_e97709: f64 = (assign60150_e97703 / assign60150_e97708);
        let assign60150_e97712: f64 = (var_t0 + var_sqrtpsisainv);
        let assign60150_e97715: f64 = (var_t0 * var_t3);
        let assign60150_e97717: f64 = (assign60150_e97715 + var_sqrtpsisa);
        let assign60150_e97718: f64 = (assign60150_e97712 / assign60150_e97717);
        let assign60150_e97719: f64 = (assign60150_e97709 * assign60150_e97718);
        (assign60150_e97719, ((((((var_t0_dn3 + var_sqrtpsisainv_dn3) * assign60150_e97708) - (assign60150_e97703 * (((var_t0_dn3 * var_t3) + (var_t0 * var_t3_dn3)) + var_sqrtpsisa_dn3))) / (assign60150_e97708 * assign60150_e97708)) * assign60150_e97718) + (assign60150_e97709 * ((((var_t0_dn3 + var_sqrtpsisainv_dn3) * assign60150_e97717) - (assign60150_e97712 * (((var_t0_dn3 * var_t3) + (var_t0 * var_t3_dn3)) + var_sqrtpsisa_dn3))) / (assign60150_e97717 * assign60150_e97717)))), ((((((var_t0_dn4 + var_sqrtpsisainv_dn4) * assign60150_e97708) - (assign60150_e97703 * (((var_t0_dn4 * var_t3) + (var_t0 * var_t3_dn4)) + var_sqrtpsisa_dn4))) / (assign60150_e97708 * assign60150_e97708)) * assign60150_e97718) + (assign60150_e97709 * ((((var_t0_dn4 + var_sqrtpsisainv_dn4) * assign60150_e97717) - (assign60150_e97712 * (((var_t0_dn4 * var_t3) + (var_t0 * var_t3_dn4)) + var_sqrtpsisa_dn4))) / (assign60150_e97717 * assign60150_e97717)))), ((((((var_t0_dn5 + var_sqrtpsisainv_dn5) * assign60150_e97708) - (assign60150_e97703 * (((var_t0_dn5 * var_t3) + (var_t0 * var_t3_dn5)) + var_sqrtpsisa_dn5))) / (assign60150_e97708 * assign60150_e97708)) * assign60150_e97718) + (assign60150_e97709 * ((((var_t0_dn5 + var_sqrtpsisainv_dn5) * assign60150_e97717) - (assign60150_e97712 * (((var_t0_dn5 * var_t3) + (var_t0 * var_t3_dn5)) + var_sqrtpsisa_dn5))) / (assign60150_e97717 * assign60150_e97717)))), ((((((var_t0_dn6 + var_sqrtpsisainv_dn6) * assign60150_e97708) - (assign60150_e97703 * (((var_t0_dn6 * var_t3) + (var_t0 * var_t3_dn6)) + var_sqrtpsisa_dn6))) / (assign60150_e97708 * assign60150_e97708)) * assign60150_e97718) + (assign60150_e97709 * ((((var_t0_dn6 + var_sqrtpsisainv_dn6) * assign60150_e97717) - (assign60150_e97712 * (((var_t0_dn6 * var_t3) + (var_t0 * var_t3_dn6)) + var_sqrtpsisa_dn6))) / (assign60150_e97717 * assign60150_e97717)))), ((((((var_t0_dn7 + var_sqrtpsisainv_dn7) * assign60150_e97708) - (assign60150_e97703 * (((var_t0_dn7 * var_t3) + (var_t0 * var_t3_dn7)) + var_sqrtpsisa_dn7))) / (assign60150_e97708 * assign60150_e97708)) * assign60150_e97718) + (assign60150_e97709 * ((((var_t0_dn7 + var_sqrtpsisainv_dn7) * assign60150_e97717) - (assign60150_e97712 * (((var_t0_dn7 * var_t3) + (var_t0 * var_t3_dn7)) + var_sqrtpsisa_dn7))) / (assign60150_e97717 * assign60150_e97717)))), ((((((var_t0_dn8 + var_sqrtpsisainv_dn8) * assign60150_e97708) - (assign60150_e97703 * (((var_t0_dn8 * var_t3) + (var_t0 * var_t3_dn8)) + var_sqrtpsisa_dn8))) / (assign60150_e97708 * assign60150_e97708)) * assign60150_e97718) + (assign60150_e97709 * ((((var_t0_dn8 + var_sqrtpsisainv_dn8) * assign60150_e97717) - (assign60150_e97712 * (((var_t0_dn8 * var_t3) + (var_t0 * var_t3_dn8)) + var_sqrtpsisa_dn8))) / (assign60150_e97717 * assign60150_e97717)))), ((((((var_t0_dn9 + var_sqrtpsisainv_dn9) * assign60150_e97708) - (assign60150_e97703 * (((var_t0_dn9 * var_t3) + (var_t0 * var_t3_dn9)) + var_sqrtpsisa_dn9))) / (assign60150_e97708 * assign60150_e97708)) * assign60150_e97718) + (assign60150_e97709 * ((((var_t0_dn9 + var_sqrtpsisainv_dn9) * assign60150_e97717) - (assign60150_e97712 * (((var_t0_dn9 * var_t3) + (var_t0 * var_t3_dn9)) + var_sqrtpsisa_dn9))) / (assign60150_e97717 * assign60150_e97717)))), ((((((var_t0_dn10 + var_sqrtpsisainv_dn10) * assign60150_e97708) - (assign60150_e97703 * (((var_t0_dn10 * var_t3) + (var_t0 * var_t3_dn10)) + var_sqrtpsisa_dn10))) / (assign60150_e97708 * assign60150_e97708)) * assign60150_e97718) + (assign60150_e97709 * ((((var_t0_dn10 + var_sqrtpsisainv_dn10) * assign60150_e97717) - (assign60150_e97712 * (((var_t0_dn10 * var_t3) + (var_t0 * var_t3_dn10)) + var_sqrtpsisa_dn10))) / (assign60150_e97717 * assign60150_e97717)))), ((((((var_t0_dn11 + var_sqrtpsisainv_dn11) * assign60150_e97708) - (assign60150_e97703 * (((var_t0_dn11 * var_t3) + (var_t0 * var_t3_dn11)) + var_sqrtpsisa_dn11))) / (assign60150_e97708 * assign60150_e97708)) * assign60150_e97718) + (assign60150_e97709 * ((((var_t0_dn11 + var_sqrtpsisainv_dn11) * assign60150_e97717) - (assign60150_e97712 * (((var_t0_dn11 * var_t3) + (var_t0 * var_t3_dn11)) + var_sqrtpsisa_dn11))) / (assign60150_e97717 * assign60150_e97717)))),)
    } else {
        (var_t6, var_t6_dn3, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_dn9, var_t6_dn10, var_t6_dn11,)
    }
};
        var_t6 = assign60150_e97721;
        var_t6_dn3 = assign60150_e97721_d_n3;
        var_t6_dn4 = assign60150_e97721_d_n4;
        var_t6_dn5 = assign60150_e97721_d_n5;
        var_t6_dn6 = assign60150_e97721_d_n6;
        var_t6_dn7 = assign60150_e97721_d_n7;
        var_t6_dn8 = assign60150_e97721_d_n8;
        var_t6_dn9 = assign60150_e97721_d_n9;
        var_t6_dn10 = assign60150_e97721_d_n10;
        var_t6_dn11 = assign60150_e97721_d_n11;
        var_t6_rv = 0.0;

        let (assign60160_e97754, assign60160_e97754_d_n3, assign60160_e97754_d_n4, assign60160_e97754_d_n5, assign60160_e97754_d_n6, assign60160_e97754_d_n7, assign60160_e97754_d_n8, assign60160_e97754_d_n9, assign60160_e97754_d_n10, assign60160_e97754_d_n11,) = {
    if (((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard874 == 0.0)) {
        let __rspice_inv_cse_0: f64 = 1.0 / var_t3;
        let assign60160_e97731: f64 = (1.0 * __rspice_inv_cse_0);
        let assign60160_e97734: f64 = (1.0 * __rspice_inv_cse_0);
        let assign60160_e97735: f64 = (assign60160_e97731 * assign60160_e97734);
        let assign60160_e97736: f64 = (-assign60160_e97735);
        let assign60160_e97740: f64 = (var_sqrtpsisa * var_sqrtpsisa);
        let assign60160_e97742: f64 = (assign60160_e97740 * var_sqrtpsisa);
        let assign60160_e97745: f64 = (var_t0 * var_t3);
        let assign60160_e97747: f64 = (assign60160_e97745 + var_sqrtpsisa);
        let assign60160_e97748: f64 = (assign60160_e97742 * assign60160_e97747);
        let assign60160_e97749: f64 = (1.0 / assign60160_e97748);
        let assign60160_e97750: f64 = (assign60160_e97736 - assign60160_e97749);
        let assign60160_e97752: f64 = (assign60160_e97750 - var_t6);
        (assign60160_e97752, (((-(((-(var_t3_dn3 / (var_t3 * var_t3))) * assign60160_e97734) + (assign60160_e97731 * (-(var_t3_dn3 / (var_t3 * var_t3)))))) - (-(((((((var_sqrtpsisa_dn3 * var_sqrtpsisa) + (var_sqrtpsisa * var_sqrtpsisa_dn3)) * var_sqrtpsisa) + (assign60160_e97740 * var_sqrtpsisa_dn3)) * assign60160_e97747) + (assign60160_e97742 * (((var_t0_dn3 * var_t3) + (var_t0 * var_t3_dn3)) + var_sqrtpsisa_dn3))) / (assign60160_e97748 * assign60160_e97748)))) - var_t6_dn3), (((-(((-(var_t3_dn4 / (var_t3 * var_t3))) * assign60160_e97734) + (assign60160_e97731 * (-(var_t3_dn4 / (var_t3 * var_t3)))))) - (-(((((((var_sqrtpsisa_dn4 * var_sqrtpsisa) + (var_sqrtpsisa * var_sqrtpsisa_dn4)) * var_sqrtpsisa) + (assign60160_e97740 * var_sqrtpsisa_dn4)) * assign60160_e97747) + (assign60160_e97742 * (((var_t0_dn4 * var_t3) + (var_t0 * var_t3_dn4)) + var_sqrtpsisa_dn4))) / (assign60160_e97748 * assign60160_e97748)))) - var_t6_dn4), (((-(((-(var_t3_dn5 / (var_t3 * var_t3))) * assign60160_e97734) + (assign60160_e97731 * (-(var_t3_dn5 / (var_t3 * var_t3)))))) - (-(((((((var_sqrtpsisa_dn5 * var_sqrtpsisa) + (var_sqrtpsisa * var_sqrtpsisa_dn5)) * var_sqrtpsisa) + (assign60160_e97740 * var_sqrtpsisa_dn5)) * assign60160_e97747) + (assign60160_e97742 * (((var_t0_dn5 * var_t3) + (var_t0 * var_t3_dn5)) + var_sqrtpsisa_dn5))) / (assign60160_e97748 * assign60160_e97748)))) - var_t6_dn5), (((-(((-(var_t3_dn6 / (var_t3 * var_t3))) * assign60160_e97734) + (assign60160_e97731 * (-(var_t3_dn6 / (var_t3 * var_t3)))))) - (-(((((((var_sqrtpsisa_dn6 * var_sqrtpsisa) + (var_sqrtpsisa * var_sqrtpsisa_dn6)) * var_sqrtpsisa) + (assign60160_e97740 * var_sqrtpsisa_dn6)) * assign60160_e97747) + (assign60160_e97742 * (((var_t0_dn6 * var_t3) + (var_t0 * var_t3_dn6)) + var_sqrtpsisa_dn6))) / (assign60160_e97748 * assign60160_e97748)))) - var_t6_dn6), (((-(((-(var_t3_dn7 / (var_t3 * var_t3))) * assign60160_e97734) + (assign60160_e97731 * (-(var_t3_dn7 / (var_t3 * var_t3)))))) - (-(((((((var_sqrtpsisa_dn7 * var_sqrtpsisa) + (var_sqrtpsisa * var_sqrtpsisa_dn7)) * var_sqrtpsisa) + (assign60160_e97740 * var_sqrtpsisa_dn7)) * assign60160_e97747) + (assign60160_e97742 * (((var_t0_dn7 * var_t3) + (var_t0 * var_t3_dn7)) + var_sqrtpsisa_dn7))) / (assign60160_e97748 * assign60160_e97748)))) - var_t6_dn7), (((-(((-(var_t3_dn8 / (var_t3 * var_t3))) * assign60160_e97734) + (assign60160_e97731 * (-(var_t3_dn8 / (var_t3 * var_t3)))))) - (-(((((((var_sqrtpsisa_dn8 * var_sqrtpsisa) + (var_sqrtpsisa * var_sqrtpsisa_dn8)) * var_sqrtpsisa) + (assign60160_e97740 * var_sqrtpsisa_dn8)) * assign60160_e97747) + (assign60160_e97742 * (((var_t0_dn8 * var_t3) + (var_t0 * var_t3_dn8)) + var_sqrtpsisa_dn8))) / (assign60160_e97748 * assign60160_e97748)))) - var_t6_dn8), (((-(((-(var_t3_dn9 / (var_t3 * var_t3))) * assign60160_e97734) + (assign60160_e97731 * (-(var_t3_dn9 / (var_t3 * var_t3)))))) - (-(((((((var_sqrtpsisa_dn9 * var_sqrtpsisa) + (var_sqrtpsisa * var_sqrtpsisa_dn9)) * var_sqrtpsisa) + (assign60160_e97740 * var_sqrtpsisa_dn9)) * assign60160_e97747) + (assign60160_e97742 * (((var_t0_dn9 * var_t3) + (var_t0 * var_t3_dn9)) + var_sqrtpsisa_dn9))) / (assign60160_e97748 * assign60160_e97748)))) - var_t6_dn9), (((-(((-(var_t3_dn10 / (var_t3 * var_t3))) * assign60160_e97734) + (assign60160_e97731 * (-(var_t3_dn10 / (var_t3 * var_t3)))))) - (-(((((((var_sqrtpsisa_dn10 * var_sqrtpsisa) + (var_sqrtpsisa * var_sqrtpsisa_dn10)) * var_sqrtpsisa) + (assign60160_e97740 * var_sqrtpsisa_dn10)) * assign60160_e97747) + (assign60160_e97742 * (((var_t0_dn10 * var_t3) + (var_t0 * var_t3_dn10)) + var_sqrtpsisa_dn10))) / (assign60160_e97748 * assign60160_e97748)))) - var_t6_dn10), (((-(((-(var_t3_dn11 / (var_t3 * var_t3))) * assign60160_e97734) + (assign60160_e97731 * (-(var_t3_dn11 / (var_t3 * var_t3)))))) - (-(((((((var_sqrtpsisa_dn11 * var_sqrtpsisa) + (var_sqrtpsisa * var_sqrtpsisa_dn11)) * var_sqrtpsisa) + (assign60160_e97740 * var_sqrtpsisa_dn11)) * assign60160_e97747) + (assign60160_e97742 * (((var_t0_dn11 * var_t3) + (var_t0 * var_t3_dn11)) + var_sqrtpsisa_dn11))) / (assign60160_e97748 * assign60160_e97748)))) - var_t6_dn11),)
    } else {
        (var_t7, var_t7_dn3, var_t7_dn4, var_t7_dn5, var_t7_dn6, var_t7_dn7, var_t7_dn8, var_t7_dn9, var_t7_dn10, var_t7_dn11,)
    }
};
        var_t7 = assign60160_e97754;
        var_t7_dn3 = assign60160_e97754_d_n3;
        var_t7_dn4 = assign60160_e97754_d_n4;
        var_t7_dn5 = assign60160_e97754_d_n5;
        var_t7_dn6 = assign60160_e97754_d_n6;
        var_t7_dn7 = assign60160_e97754_d_n7;
        var_t7_dn8 = assign60160_e97754_d_n8;
        var_t7_dn9 = assign60160_e97754_d_n9;
        var_t7_dn10 = assign60160_e97754_d_n10;
        var_t7_dn11 = assign60160_e97754_d_n11;
        var_t7_rv = 0.0;

        let (assign60170_e97780, assign60170_e97780_d_n3, assign60170_e97780_d_n4, assign60170_e97780_d_n5, assign60170_e97780_d_n6, assign60170_e97780_d_n7, assign60170_e97780_d_n8, assign60170_e97780_d_n9, assign60170_e97780_d_n10, assign60170_e97780_d_n11,) = {
    if (((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard874 == 0.0)) {
        let assign60170_e97765: f64 = (var_t4 / var_t5);
        let assign60170_e97769: f64 = (var_t4 * var_t7);
        let assign60170_e97772: f64 = (2.0 * var_t5);
        let assign60170_e97774: f64 = (assign60170_e97772 * var_t5);
        let assign60170_e97775: f64 = (assign60170_e97769 / assign60170_e97774);
        let assign60170_e97776: f64 = (1.0 + assign60170_e97775);
        let assign60170_e97777: f64 = (assign60170_e97765 * assign60170_e97776);
        let assign60170_e97778: f64 = (var_t3 - assign60170_e97777);
        (assign60170_e97778, (var_t3_dn3 - (((((var_t4_dn3 * var_t5) - (var_t4 * var_t5_dn3)) / (var_t5 * var_t5)) * assign60170_e97776) + (assign60170_e97765 * (((((var_t4_dn3 * var_t7) + (var_t4 * var_t7_dn3)) * assign60170_e97774) - (assign60170_e97769 * (((2.0 * var_t5_dn3) * var_t5) + (assign60170_e97772 * var_t5_dn3)))) / (assign60170_e97774 * assign60170_e97774))))), (var_t3_dn4 - (((((var_t4_dn4 * var_t5) - (var_t4 * var_t5_dn4)) / (var_t5 * var_t5)) * assign60170_e97776) + (assign60170_e97765 * (((((var_t4_dn4 * var_t7) + (var_t4 * var_t7_dn4)) * assign60170_e97774) - (assign60170_e97769 * (((2.0 * var_t5_dn4) * var_t5) + (assign60170_e97772 * var_t5_dn4)))) / (assign60170_e97774 * assign60170_e97774))))), (var_t3_dn5 - (((((var_t4_dn5 * var_t5) - (var_t4 * var_t5_dn5)) / (var_t5 * var_t5)) * assign60170_e97776) + (assign60170_e97765 * (((((var_t4_dn5 * var_t7) + (var_t4 * var_t7_dn5)) * assign60170_e97774) - (assign60170_e97769 * (((2.0 * var_t5_dn5) * var_t5) + (assign60170_e97772 * var_t5_dn5)))) / (assign60170_e97774 * assign60170_e97774))))), (var_t3_dn6 - (((((var_t4_dn6 * var_t5) - (var_t4 * var_t5_dn6)) / (var_t5 * var_t5)) * assign60170_e97776) + (assign60170_e97765 * (((((var_t4_dn6 * var_t7) + (var_t4 * var_t7_dn6)) * assign60170_e97774) - (assign60170_e97769 * (((2.0 * var_t5_dn6) * var_t5) + (assign60170_e97772 * var_t5_dn6)))) / (assign60170_e97774 * assign60170_e97774))))), (var_t3_dn7 - (((((var_t4_dn7 * var_t5) - (var_t4 * var_t5_dn7)) / (var_t5 * var_t5)) * assign60170_e97776) + (assign60170_e97765 * (((((var_t4_dn7 * var_t7) + (var_t4 * var_t7_dn7)) * assign60170_e97774) - (assign60170_e97769 * (((2.0 * var_t5_dn7) * var_t5) + (assign60170_e97772 * var_t5_dn7)))) / (assign60170_e97774 * assign60170_e97774))))), (var_t3_dn8 - (((((var_t4_dn8 * var_t5) - (var_t4 * var_t5_dn8)) / (var_t5 * var_t5)) * assign60170_e97776) + (assign60170_e97765 * (((((var_t4_dn8 * var_t7) + (var_t4 * var_t7_dn8)) * assign60170_e97774) - (assign60170_e97769 * (((2.0 * var_t5_dn8) * var_t5) + (assign60170_e97772 * var_t5_dn8)))) / (assign60170_e97774 * assign60170_e97774))))), (var_t3_dn9 - (((((var_t4_dn9 * var_t5) - (var_t4 * var_t5_dn9)) / (var_t5 * var_t5)) * assign60170_e97776) + (assign60170_e97765 * (((((var_t4_dn9 * var_t7) + (var_t4 * var_t7_dn9)) * assign60170_e97774) - (assign60170_e97769 * (((2.0 * var_t5_dn9) * var_t5) + (assign60170_e97772 * var_t5_dn9)))) / (assign60170_e97774 * assign60170_e97774))))), (var_t3_dn10 - (((((var_t4_dn10 * var_t5) - (var_t4 * var_t5_dn10)) / (var_t5 * var_t5)) * assign60170_e97776) + (assign60170_e97765 * (((((var_t4_dn10 * var_t7) + (var_t4 * var_t7_dn10)) * assign60170_e97774) - (assign60170_e97769 * (((2.0 * var_t5_dn10) * var_t5) + (assign60170_e97772 * var_t5_dn10)))) / (assign60170_e97774 * assign60170_e97774))))), (var_t3_dn11 - (((((var_t4_dn11 * var_t5) - (var_t4 * var_t5_dn11)) / (var_t5 * var_t5)) * assign60170_e97776) + (assign60170_e97765 * (((((var_t4_dn11 * var_t7) + (var_t4 * var_t7_dn11)) * assign60170_e97774) - (assign60170_e97769 * (((2.0 * var_t5_dn11) * var_t5) + (assign60170_e97772 * var_t5_dn11)))) / (assign60170_e97774 * assign60170_e97774))))),)
    } else {
        (var_qdeff_edge, var_qdeff_edge_dn3, var_qdeff_edge_dn4, var_qdeff_edge_dn5, var_qdeff_edge_dn6, var_qdeff_edge_dn7, var_qdeff_edge_dn8, var_qdeff_edge_dn9, var_qdeff_edge_dn10, var_qdeff_edge_dn11,)
    }
};
        var_qdeff_edge = assign60170_e97780;
        var_qdeff_edge_dn3 = assign60170_e97780_d_n3;
        var_qdeff_edge_dn4 = assign60170_e97780_d_n4;
        var_qdeff_edge_dn5 = assign60170_e97780_d_n5;
        var_qdeff_edge_dn6 = assign60170_e97780_d_n6;
        var_qdeff_edge_dn7 = assign60170_e97780_d_n7;
        var_qdeff_edge_dn8 = assign60170_e97780_d_n8;
        var_qdeff_edge_dn9 = assign60170_e97780_d_n9;
        var_qdeff_edge_dn10 = assign60170_e97780_d_n10;
        var_qdeff_edge_dn11 = assign60170_e97780_d_n11;
        var_qdeff_edge_rv = 0.0;

        let (assign60180_e97806, assign60180_e97806_d_n3, assign60180_e97806_d_n4, assign60180_e97806_d_n5, assign60180_e97806_d_n6, assign60180_e97806_d_n7, assign60180_e97806_d_n8, assign60180_e97806_d_n9, assign60180_e97806_d_n10, assign60180_e97806_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign60180_e97788: f64 = (var_psip + 1.0);
        let assign60180_e97791: f64 = (var_psip - 1.0);
        let assign60180_e97794: f64 = (var_psip - 1.0);
        let assign60180_e97795: f64 = (assign60180_e97791 * assign60180_e97794);
        let assign60180_e97798: f64 = (0.25 * 2.0);
        let assign60180_e97800: f64 = (assign60180_e97798 * 2.0);
        let assign60180_e97801: f64 = (assign60180_e97795 + assign60180_e97800);
        let assign60180_e97802: f64 = (assign60180_e97801).sqrt();
        let assign60180_e97803: f64 = (assign60180_e97788 + assign60180_e97802);
        let assign60180_e97804: f64 = (0.5 * assign60180_e97803);
        (assign60180_e97804, (0.5 * (var_psip_dn3 + (((var_psip_dn3 * assign60180_e97794) + (assign60180_e97791 * var_psip_dn3)) / (2.0 * assign60180_e97802)))), (0.5 * (var_psip_dn4 + (((var_psip_dn4 * assign60180_e97794) + (assign60180_e97791 * var_psip_dn4)) / (2.0 * assign60180_e97802)))), (0.5 * (var_psip_dn5 + (((var_psip_dn5 * assign60180_e97794) + (assign60180_e97791 * var_psip_dn5)) / (2.0 * assign60180_e97802)))), (0.5 * (var_psip_dn6 + (((var_psip_dn6 * assign60180_e97794) + (assign60180_e97791 * var_psip_dn6)) / (2.0 * assign60180_e97802)))), (0.5 * (var_psip_dn7 + (((var_psip_dn7 * assign60180_e97794) + (assign60180_e97791 * var_psip_dn7)) / (2.0 * assign60180_e97802)))), (0.5 * (var_psip_dn8 + (((var_psip_dn8 * assign60180_e97794) + (assign60180_e97791 * var_psip_dn8)) / (2.0 * assign60180_e97802)))), (0.5 * (var_psip_dn9 + (((var_psip_dn9 * assign60180_e97794) + (assign60180_e97791 * var_psip_dn9)) / (2.0 * assign60180_e97802)))), (0.5 * (var_psip_dn10 + (((var_psip_dn10 * assign60180_e97794) + (assign60180_e97791 * var_psip_dn10)) / (2.0 * assign60180_e97802)))), (0.5 * (var_psip_dn11 + (((var_psip_dn11 * assign60180_e97794) + (assign60180_e97791 * var_psip_dn11)) / (2.0 * assign60180_e97802)))),)
    } else {
        (var_psipclamp, var_psipclamp_dn3, var_psipclamp_dn4, var_psipclamp_dn5, var_psipclamp_dn6, var_psipclamp_dn7, var_psipclamp_dn8, var_psipclamp_dn9, var_psipclamp_dn10, var_psipclamp_dn11,)
    }
};
        var_psipclamp = assign60180_e97806;
        var_psipclamp_dn3 = assign60180_e97806_d_n3;
        var_psipclamp_dn4 = assign60180_e97806_d_n4;
        var_psipclamp_dn5 = assign60180_e97806_d_n5;
        var_psipclamp_dn6 = assign60180_e97806_d_n6;
        var_psipclamp_dn7 = assign60180_e97806_d_n7;
        var_psipclamp_dn8 = assign60180_e97806_d_n8;
        var_psipclamp_dn9 = assign60180_e97806_d_n9;
        var_psipclamp_dn10 = assign60180_e97806_d_n10;
        var_psipclamp_dn11 = assign60180_e97806_d_n11;
        var_psipclamp_rv = 0.0;

        let (assign60190_e97814, assign60190_e97814_d_n3, assign60190_e97814_d_n4, assign60190_e97814_d_n5, assign60190_e97814_d_n6, assign60190_e97814_d_n7, assign60190_e97814_d_n8, assign60190_e97814_d_n9, assign60190_e97814_d_n10, assign60190_e97814_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign60190_e97812: f64 = (var_psipclamp).sqrt();
        (assign60190_e97812, (var_psipclamp_dn3 / (2.0 * assign60190_e97812)), (var_psipclamp_dn4 / (2.0 * assign60190_e97812)), (var_psipclamp_dn5 / (2.0 * assign60190_e97812)), (var_psipclamp_dn6 / (2.0 * assign60190_e97812)), (var_psipclamp_dn7 / (2.0 * assign60190_e97812)), (var_psipclamp_dn8 / (2.0 * assign60190_e97812)), (var_psipclamp_dn9 / (2.0 * assign60190_e97812)), (var_psipclamp_dn10 / (2.0 * assign60190_e97812)), (var_psipclamp_dn11 / (2.0 * assign60190_e97812)),)
    } else {
        (var_sqrtpsip, var_sqrtpsip_dn3, var_sqrtpsip_dn4, var_sqrtpsip_dn5, var_sqrtpsip_dn6, var_sqrtpsip_dn7, var_sqrtpsip_dn8, var_sqrtpsip_dn9, var_sqrtpsip_dn10, var_sqrtpsip_dn11,)
    }
};
        var_sqrtpsip = assign60190_e97814;
        var_sqrtpsip_dn3 = assign60190_e97814_d_n3;
        var_sqrtpsip_dn4 = assign60190_e97814_d_n4;
        var_sqrtpsip_dn5 = assign60190_e97814_d_n5;
        var_sqrtpsip_dn6 = assign60190_e97814_d_n6;
        var_sqrtpsip_dn7 = assign60190_e97814_d_n7;
        var_sqrtpsip_dn8 = assign60190_e97814_d_n8;
        var_sqrtpsip_dn9 = assign60190_e97814_d_n9;
        var_sqrtpsip_dn10 = assign60190_e97814_d_n10;
        var_sqrtpsip_dn11 = assign60190_e97814_d_n11;
        var_sqrtpsip_rv = 0.0;

        let (assign60200_e97827, assign60200_e97827_d_n3, assign60200_e97827_d_n4, assign60200_e97827_d_n5, assign60200_e97827_d_n6, assign60200_e97827_d_n7, assign60200_e97827_d_n8, assign60200_e97827_d_n9, assign60200_e97827_d_n10, assign60200_e97827_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign60200_e97821: f64 = (var_psip - var_qs_edge);
        let assign60200_e97823: f64 = (assign60200_e97821 - var_qdeff_edge);
        let assign60200_e97825: f64 = (assign60200_e97823 - 1.0);
        (assign60200_e97825, ((var_psip_dn3 - var_qs_edge_dn3) - var_qdeff_edge_dn3), ((var_psip_dn4 - var_qs_edge_dn4) - var_qdeff_edge_dn4), ((var_psip_dn5 - var_qs_edge_dn5) - var_qdeff_edge_dn5), ((var_psip_dn6 - var_qs_edge_dn6) - var_qdeff_edge_dn6), ((var_psip_dn7 - var_qs_edge_dn7) - var_qdeff_edge_dn7), ((var_psip_dn8 - var_qs_edge_dn8) - var_qdeff_edge_dn8), ((var_psip_dn9 - var_qs_edge_dn9) - var_qdeff_edge_dn9), ((var_psip_dn10 - var_qs_edge_dn10) - var_qdeff_edge_dn10), ((var_psip_dn11 - var_qs_edge_dn11) - var_qdeff_edge_dn11),)
    } else {
        (var_psiavg, var_psiavg_dn3, var_psiavg_dn4, var_psiavg_dn5, var_psiavg_dn6, var_psiavg_dn7, var_psiavg_dn8, var_psiavg_dn9, var_psiavg_dn10, var_psiavg_dn11,)
    }
};
        var_psiavg = assign60200_e97827;
        var_psiavg_dn3 = assign60200_e97827_d_n3;
        var_psiavg_dn4 = assign60200_e97827_d_n4;
        var_psiavg_dn5 = assign60200_e97827_d_n5;
        var_psiavg_dn6 = assign60200_e97827_d_n6;
        var_psiavg_dn7 = assign60200_e97827_d_n7;
        var_psiavg_dn8 = assign60200_e97827_d_n8;
        var_psiavg_dn9 = assign60200_e97827_d_n9;
        var_psiavg_dn10 = assign60200_e97827_d_n10;
        var_psiavg_dn11 = assign60200_e97827_d_n11;
        var_psiavg_rv = 0.0;

        let (assign60210_e97853, assign60210_e97853_d_n3, assign60210_e97853_d_n4, assign60210_e97853_d_n5, assign60210_e97853_d_n6, assign60210_e97853_d_n7, assign60210_e97853_d_n8, assign60210_e97853_d_n9, assign60210_e97853_d_n10, assign60210_e97853_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign60210_e97835: f64 = (var_psiavg + 1.0);
        let assign60210_e97838: f64 = (var_psiavg - 1.0);
        let assign60210_e97841: f64 = (var_psiavg - 1.0);
        let assign60210_e97842: f64 = (assign60210_e97838 * assign60210_e97841);
        let assign60210_e97845: f64 = (0.25 * 2.0);
        let assign60210_e97847: f64 = (assign60210_e97845 * 2.0);
        let assign60210_e97848: f64 = (assign60210_e97842 + assign60210_e97847);
        let assign60210_e97849: f64 = (assign60210_e97848).sqrt();
        let assign60210_e97850: f64 = (assign60210_e97835 + assign60210_e97849);
        let assign60210_e97851: f64 = (0.5 * assign60210_e97850);
        (assign60210_e97851, (0.5 * (var_psiavg_dn3 + (((var_psiavg_dn3 * assign60210_e97841) + (assign60210_e97838 * var_psiavg_dn3)) / (2.0 * assign60210_e97849)))), (0.5 * (var_psiavg_dn4 + (((var_psiavg_dn4 * assign60210_e97841) + (assign60210_e97838 * var_psiavg_dn4)) / (2.0 * assign60210_e97849)))), (0.5 * (var_psiavg_dn5 + (((var_psiavg_dn5 * assign60210_e97841) + (assign60210_e97838 * var_psiavg_dn5)) / (2.0 * assign60210_e97849)))), (0.5 * (var_psiavg_dn6 + (((var_psiavg_dn6 * assign60210_e97841) + (assign60210_e97838 * var_psiavg_dn6)) / (2.0 * assign60210_e97849)))), (0.5 * (var_psiavg_dn7 + (((var_psiavg_dn7 * assign60210_e97841) + (assign60210_e97838 * var_psiavg_dn7)) / (2.0 * assign60210_e97849)))), (0.5 * (var_psiavg_dn8 + (((var_psiavg_dn8 * assign60210_e97841) + (assign60210_e97838 * var_psiavg_dn8)) / (2.0 * assign60210_e97849)))), (0.5 * (var_psiavg_dn9 + (((var_psiavg_dn9 * assign60210_e97841) + (assign60210_e97838 * var_psiavg_dn9)) / (2.0 * assign60210_e97849)))), (0.5 * (var_psiavg_dn10 + (((var_psiavg_dn10 * assign60210_e97841) + (assign60210_e97838 * var_psiavg_dn10)) / (2.0 * assign60210_e97849)))), (0.5 * (var_psiavg_dn11 + (((var_psiavg_dn11 * assign60210_e97841) + (assign60210_e97838 * var_psiavg_dn11)) / (2.0 * assign60210_e97849)))),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11,)
    }
};
        var_t0 = assign60210_e97853;
        var_t0_dn3 = assign60210_e97853_d_n3;
        var_t0_dn4 = assign60210_e97853_d_n4;
        var_t0_dn5 = assign60210_e97853_d_n5;
        var_t0_dn6 = assign60210_e97853_d_n6;
        var_t0_dn7 = assign60210_e97853_d_n7;
        var_t0_dn8 = assign60210_e97853_d_n8;
        var_t0_dn9 = assign60210_e97853_d_n9;
        var_t0_dn10 = assign60210_e97853_d_n10;
        var_t0_dn11 = assign60210_e97853_d_n11;
        var_t0_rv = 0.0;

        *var_guard876_slot = var_guard876;
        *var_guard876_rv_slot = var_guard876_rv;
        *var_psiavg_slot = var_psiavg;
        *var_psiavg_dn10_slot = var_psiavg_dn10;
        *var_psiavg_dn11_slot = var_psiavg_dn11;
        *var_psiavg_dn3_slot = var_psiavg_dn3;
        *var_psiavg_dn4_slot = var_psiavg_dn4;
        *var_psiavg_dn5_slot = var_psiavg_dn5;
        *var_psiavg_dn6_slot = var_psiavg_dn6;
        *var_psiavg_dn7_slot = var_psiavg_dn7;
        *var_psiavg_dn8_slot = var_psiavg_dn8;
        *var_psiavg_dn9_slot = var_psiavg_dn9;
        *var_psiavg_rv_slot = var_psiavg_rv;
        *var_psipclamp_slot = var_psipclamp;
        *var_psipclamp_dn10_slot = var_psipclamp_dn10;
        *var_psipclamp_dn11_slot = var_psipclamp_dn11;
        *var_psipclamp_dn3_slot = var_psipclamp_dn3;
        *var_psipclamp_dn4_slot = var_psipclamp_dn4;
        *var_psipclamp_dn5_slot = var_psipclamp_dn5;
        *var_psipclamp_dn6_slot = var_psipclamp_dn6;
        *var_psipclamp_dn7_slot = var_psipclamp_dn7;
        *var_psipclamp_dn8_slot = var_psipclamp_dn8;
        *var_psipclamp_dn9_slot = var_psipclamp_dn9;
        *var_psipclamp_rv_slot = var_psipclamp_rv;
        *var_qdeff_edge_slot = var_qdeff_edge;
        *var_qdeff_edge_dn10_slot = var_qdeff_edge_dn10;
        *var_qdeff_edge_dn11_slot = var_qdeff_edge_dn11;
        *var_qdeff_edge_dn3_slot = var_qdeff_edge_dn3;
        *var_qdeff_edge_dn4_slot = var_qdeff_edge_dn4;
        *var_qdeff_edge_dn5_slot = var_qdeff_edge_dn5;
        *var_qdeff_edge_dn6_slot = var_qdeff_edge_dn6;
        *var_qdeff_edge_dn7_slot = var_qdeff_edge_dn7;
        *var_qdeff_edge_dn8_slot = var_qdeff_edge_dn8;
        *var_qdeff_edge_dn9_slot = var_qdeff_edge_dn9;
        *var_qdeff_edge_rv_slot = var_qdeff_edge_rv;
        *var_sqrtpsip_slot = var_sqrtpsip;
        *var_sqrtpsip_dn10_slot = var_sqrtpsip_dn10;
        *var_sqrtpsip_dn11_slot = var_sqrtpsip_dn11;
        *var_sqrtpsip_dn3_slot = var_sqrtpsip_dn3;
        *var_sqrtpsip_dn4_slot = var_sqrtpsip_dn4;
        *var_sqrtpsip_dn5_slot = var_sqrtpsip_dn5;
        *var_sqrtpsip_dn6_slot = var_sqrtpsip_dn6;
        *var_sqrtpsip_dn7_slot = var_sqrtpsip_dn7;
        *var_sqrtpsip_dn8_slot = var_sqrtpsip_dn8;
        *var_sqrtpsip_dn9_slot = var_sqrtpsip_dn9;
        *var_sqrtpsip_rv_slot = var_sqrtpsip_rv;
        *var_sqrtpsisainv_slot = var_sqrtpsisainv;
        *var_sqrtpsisainv_dn10_slot = var_sqrtpsisainv_dn10;
        *var_sqrtpsisainv_dn11_slot = var_sqrtpsisainv_dn11;
        *var_sqrtpsisainv_dn3_slot = var_sqrtpsisainv_dn3;
        *var_sqrtpsisainv_dn4_slot = var_sqrtpsisainv_dn4;
        *var_sqrtpsisainv_dn5_slot = var_sqrtpsisainv_dn5;
        *var_sqrtpsisainv_dn6_slot = var_sqrtpsisainv_dn6;
        *var_sqrtpsisainv_dn7_slot = var_sqrtpsisainv_dn7;
        *var_sqrtpsisainv_dn8_slot = var_sqrtpsisainv_dn8;
        *var_sqrtpsisainv_dn9_slot = var_sqrtpsisainv_dn9;
        *var_sqrtpsisainv_rv_slot = var_sqrtpsisainv_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_t0_rv_slot = var_t0_rv;
        *var_t2_slot = var_t2;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_t2_rv_slot = var_t2_rv;
        *var_t3_slot = var_t3;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn3_slot = var_t3_dn3;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_dn9_slot = var_t3_dn9;
        *var_t3_rv_slot = var_t3_rv;
        *var_t4_slot = var_t4;
        *var_t4_dn10_slot = var_t4_dn10;
        *var_t4_dn11_slot = var_t4_dn11;
        *var_t4_dn3_slot = var_t4_dn3;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t4_dn9_slot = var_t4_dn9;
        *var_t4_rv_slot = var_t4_rv;
        *var_t5_slot = var_t5;
        *var_t5_dn10_slot = var_t5_dn10;
        *var_t5_dn11_slot = var_t5_dn11;
        *var_t5_dn3_slot = var_t5_dn3;
        *var_t5_dn4_slot = var_t5_dn4;
        *var_t5_dn5_slot = var_t5_dn5;
        *var_t5_dn6_slot = var_t5_dn6;
        *var_t5_dn7_slot = var_t5_dn7;
        *var_t5_dn8_slot = var_t5_dn8;
        *var_t5_dn9_slot = var_t5_dn9;
        *var_t5_rv_slot = var_t5_rv;
        *var_t6_slot = var_t6;
        *var_t6_dn10_slot = var_t6_dn10;
        *var_t6_dn11_slot = var_t6_dn11;
        *var_t6_dn3_slot = var_t6_dn3;
        *var_t6_dn4_slot = var_t6_dn4;
        *var_t6_dn5_slot = var_t6_dn5;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn7_slot = var_t6_dn7;
        *var_t6_dn8_slot = var_t6_dn8;
        *var_t6_dn9_slot = var_t6_dn9;
        *var_t6_rv_slot = var_t6_rv;
        *var_t7_slot = var_t7;
        *var_t7_dn10_slot = var_t7_dn10;
        *var_t7_dn11_slot = var_t7_dn11;
        *var_t7_dn3_slot = var_t7_dn3;
        *var_t7_dn4_slot = var_t7_dn4;
        *var_t7_dn5_slot = var_t7_dn5;
        *var_t7_dn6_slot = var_t7_dn6;
        *var_t7_dn7_slot = var_t7_dn7;
        *var_t7_dn8_slot = var_t7_dn8;
        *var_t7_dn9_slot = var_t7_dn9;
        *var_t7_rv_slot = var_t7_rv;
    }

    pub(super) fn stamp_reactive_block_211(
        p: &Parameters,
        var_cdep: f64,
        var_cdep_dn10: f64,
        var_cdep_dn11: f64,
        var_cdep_dn3: f64,
        var_cdep_dn4: f64,
        var_cdep_dn5: f64,
        var_cdep_dn6: f64,
        var_cdep_dn7: f64,
        var_cdep_dn8: f64,
        var_cdep_dn9: f64,
        var_citedge_i: f64,
        var_cox: f64,
        var_delclm: f64,
        var_delclm_dn10: f64,
        var_delclm_dn11: f64,
        var_delclm_dn3: f64,
        var_delclm_dn4: f64,
        var_delclm_dn5: f64,
        var_delclm_dn6: f64,
        var_delclm_dn7: f64,
        var_delclm_dn8: f64,
        var_delclm_dn9: f64,
        var_gam_edge: f64,
        var_gam_edge_dn10: f64,
        var_gam_edge_dn11: f64,
        var_gam_edge_dn3: f64,
        var_gam_edge_dn4: f64,
        var_gam_edge_dn5: f64,
        var_gam_edge_dn6: f64,
        var_gam_edge_dn7: f64,
        var_gam_edge_dn8: f64,
        var_gam_edge_dn9: f64,
        var_guard492: f64,
        var_guard867: f64,
        var_leff: f64,
        var_lintnoi_i: f64,
        var_moc: f64,
        var_moc_dn10: f64,
        var_moc_dn11: f64,
        var_moc_dn3: f64,
        var_moc_dn4: f64,
        var_moc_dn5: f64,
        var_moc_dn6: f64,
        var_moc_dn7: f64,
        var_moc_dn8: f64,
        var_moc_dn9: f64,
        var_nvt: f64,
        var_nvt_dn10: f64,
        var_nvt_dn11: f64,
        var_nvt_dn3: f64,
        var_nvt_dn4: f64,
        var_nvt_dn5: f64,
        var_nvt_dn6: f64,
        var_nvt_dn7: f64,
        var_nvt_dn8: f64,
        var_nvt_dn9: f64,
        var_qdeff_edge: f64,
        var_qdeff_edge_dn10: f64,
        var_qdeff_edge_dn11: f64,
        var_qdeff_edge_dn3: f64,
        var_qdeff_edge_dn4: f64,
        var_qdeff_edge_dn5: f64,
        var_qdeff_edge_dn6: f64,
        var_qdeff_edge_dn7: f64,
        var_qdeff_edge_dn8: f64,
        var_qdeff_edge_dn9: f64,
        var_qs_edge: f64,
        var_qs_edge_dn10: f64,
        var_qs_edge_dn11: f64,
        var_qs_edge_dn3: f64,
        var_qs_edge_dn4: f64,
        var_qs_edge_dn5: f64,
        var_qs_edge_dn6: f64,
        var_qs_edge_dn7: f64,
        var_qs_edge_dn8: f64,
        var_qs_edge_dn9: f64,
        var_sqrtpsip: f64,
        var_sqrtpsip_dn10: f64,
        var_sqrtpsip_dn11: f64,
        var_sqrtpsip_dn3: f64,
        var_sqrtpsip_dn4: f64,
        var_sqrtpsip_dn5: f64,
        var_sqrtpsip_dn6: f64,
        var_sqrtpsip_dn7: f64,
        var_sqrtpsip_dn8: f64,
        var_sqrtpsip_dn9: f64,
        var_t0: f64,
        var_t0_dn10: f64,
        var_t0_dn11: f64,
        var_t0_dn3: f64,
        var_t0_dn4: f64,
        var_t0_dn5: f64,
        var_t0_dn6: f64,
        var_t0_dn7: f64,
        var_t0_dn8: f64,
        var_t0_dn9: f64,
        var_ueff: f64,
        var_ueff_dn10: f64,
        var_ueff_dn11: f64,
        var_ueff_dn3: f64,
        var_ueff_dn4: f64,
        var_ueff_dn5: f64,
        var_ueff_dn6: f64,
        var_ueff_dn7: f64,
        var_ueff_dn8: f64,
        var_ueff_dn9: f64,
        var_vt: f64,
        var_vt_dn4: f64,
        var_vt_dn5: f64,
        var_ids_slot: &mut f64,
        var_ids_dn10_slot: &mut f64,
        var_ids_dn11_slot: &mut f64,
        var_ids_dn3_slot: &mut f64,
        var_ids_dn4_slot: &mut f64,
        var_ids_dn5_slot: &mut f64,
        var_ids_dn6_slot: &mut f64,
        var_ids_dn7_slot: &mut f64,
        var_ids_dn8_slot: &mut f64,
        var_ids_dn9_slot: &mut f64,
        var_ids_edge_slot: &mut f64,
        var_ids_edge_dn10_slot: &mut f64,
        var_ids_edge_dn11_slot: &mut f64,
        var_ids_edge_dn3_slot: &mut f64,
        var_ids_edge_dn4_slot: &mut f64,
        var_ids_edge_dn5_slot: &mut f64,
        var_ids_edge_dn6_slot: &mut f64,
        var_ids_edge_dn7_slot: &mut f64,
        var_ids_edge_dn8_slot: &mut f64,
        var_ids_edge_dn9_slot: &mut f64,
        var_ids_edge_rv_slot: &mut f64,
        var_ids_rv_slot: &mut f64,
        var_leffnoi_edge_slot: &mut f64,
        var_leffnoi_edge_rv_slot: &mut f64,
        var_leffnoisq_edge_slot: &mut f64,
        var_leffnoisq_edge_rv_slot: &mut f64,
        var_n0_slot: &mut f64,
        var_n0_dn10_slot: &mut f64,
        var_n0_dn11_slot: &mut f64,
        var_n0_dn3_slot: &mut f64,
        var_n0_dn4_slot: &mut f64,
        var_n0_dn5_slot: &mut f64,
        var_n0_dn6_slot: &mut f64,
        var_n0_dn7_slot: &mut f64,
        var_n0_dn8_slot: &mut f64,
        var_n0_dn9_slot: &mut f64,
        var_n0_rv_slot: &mut f64,
        var_nl_slot: &mut f64,
        var_nl_dn10_slot: &mut f64,
        var_nl_dn11_slot: &mut f64,
        var_nl_dn3_slot: &mut f64,
        var_nl_dn4_slot: &mut f64,
        var_nl_dn5_slot: &mut f64,
        var_nl_dn6_slot: &mut f64,
        var_nl_dn7_slot: &mut f64,
        var_nl_dn8_slot: &mut f64,
        var_nl_dn9_slot: &mut f64,
        var_nl_rv_slot: &mut f64,
        var_noia_edge_slot: &mut f64,
        var_noia_edge_rv_slot: &mut f64,
        var_noib_edge_slot: &mut f64,
        var_noib_edge_rv_slot: &mut f64,
        var_noic_edge_slot: &mut f64,
        var_noic_edge_rv_slot: &mut f64,
        var_nq_edge_slot: &mut f64,
        var_nq_edge_dn10_slot: &mut f64,
        var_nq_edge_dn11_slot: &mut f64,
        var_nq_edge_dn3_slot: &mut f64,
        var_nq_edge_dn4_slot: &mut f64,
        var_nq_edge_dn5_slot: &mut f64,
        var_nq_edge_dn6_slot: &mut f64,
        var_nq_edge_dn7_slot: &mut f64,
        var_nq_edge_dn8_slot: &mut f64,
        var_nq_edge_dn9_slot: &mut f64,
        var_nq_edge_rv_slot: &mut f64,
        var_nstar_slot: &mut f64,
        var_nstar_dn10_slot: &mut f64,
        var_nstar_dn11_slot: &mut f64,
        var_nstar_dn3_slot: &mut f64,
        var_nstar_dn4_slot: &mut f64,
        var_nstar_dn5_slot: &mut f64,
        var_nstar_dn6_slot: &mut f64,
        var_nstar_dn7_slot: &mut f64,
        var_nstar_dn8_slot: &mut f64,
        var_nstar_dn9_slot: &mut f64,
        var_nstar_rv_slot: &mut f64,
        var_ssi_slot: &mut f64,
        var_ssi_dn10_slot: &mut f64,
        var_ssi_dn11_slot: &mut f64,
        var_ssi_dn3_slot: &mut f64,
        var_ssi_dn4_slot: &mut f64,
        var_ssi_dn5_slot: &mut f64,
        var_ssi_dn6_slot: &mut f64,
        var_ssi_dn7_slot: &mut f64,
        var_ssi_dn8_slot: &mut f64,
        var_ssi_dn9_slot: &mut f64,
        var_ssi_rv_slot: &mut f64,
        var_swi_slot: &mut f64,
        var_swi_dn10_slot: &mut f64,
        var_swi_dn11_slot: &mut f64,
        var_swi_dn3_slot: &mut f64,
        var_swi_dn4_slot: &mut f64,
        var_swi_dn5_slot: &mut f64,
        var_swi_dn6_slot: &mut f64,
        var_swi_dn7_slot: &mut f64,
        var_swi_dn8_slot: &mut f64,
        var_swi_dn9_slot: &mut f64,
        var_swi_rv_slot: &mut f64,
        var_t0a_slot: &mut f64,
        var_t0a_dn10_slot: &mut f64,
        var_t0a_dn11_slot: &mut f64,
        var_t0a_dn3_slot: &mut f64,
        var_t0a_dn4_slot: &mut f64,
        var_t0a_dn5_slot: &mut f64,
        var_t0a_dn6_slot: &mut f64,
        var_t0a_dn7_slot: &mut f64,
        var_t0a_dn8_slot: &mut f64,
        var_t0a_dn9_slot: &mut f64,
        var_t0a_rv_slot: &mut f64,
        var_t0b_slot: &mut f64,
        var_t0b_dn10_slot: &mut f64,
        var_t0b_dn11_slot: &mut f64,
        var_t0b_dn3_slot: &mut f64,
        var_t0b_dn4_slot: &mut f64,
        var_t0b_dn5_slot: &mut f64,
        var_t0b_dn6_slot: &mut f64,
        var_t0b_dn7_slot: &mut f64,
        var_t0b_dn8_slot: &mut f64,
        var_t0b_dn9_slot: &mut f64,
        var_t0b_rv_slot: &mut f64,
        var_t0c_slot: &mut f64,
        var_t0c_dn10_slot: &mut f64,
        var_t0c_dn11_slot: &mut f64,
        var_t0c_dn3_slot: &mut f64,
        var_t0c_dn4_slot: &mut f64,
        var_t0c_dn5_slot: &mut f64,
        var_t0c_dn6_slot: &mut f64,
        var_t0c_dn7_slot: &mut f64,
        var_t0c_dn8_slot: &mut f64,
        var_t0c_dn9_slot: &mut f64,
        var_t0c_rv_slot: &mut f64,
        var_t0d_slot: &mut f64,
        var_t0d_dn10_slot: &mut f64,
        var_t0d_dn11_slot: &mut f64,
        var_t0d_dn3_slot: &mut f64,
        var_t0d_dn4_slot: &mut f64,
        var_t0d_dn5_slot: &mut f64,
        var_t0d_dn6_slot: &mut f64,
        var_t0d_dn7_slot: &mut f64,
        var_t0d_dn8_slot: &mut f64,
        var_t0d_dn9_slot: &mut f64,
        var_t0d_rv_slot: &mut f64,
        var_t0e_slot: &mut f64,
        var_t0e_dn4_slot: &mut f64,
        var_t0e_dn5_slot: &mut f64,
        var_t0e_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_dn9_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn10_slot: &mut f64,
        var_t4_dn11_slot: &mut f64,
        var_t4_dn3_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t4_dn9_slot: &mut f64,
        var_t4_rv_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_dn10_slot: &mut f64,
        var_t5_dn11_slot: &mut f64,
        var_t5_dn3_slot: &mut f64,
        var_t5_dn4_slot: &mut f64,
        var_t5_dn5_slot: &mut f64,
        var_t5_dn6_slot: &mut f64,
        var_t5_dn7_slot: &mut f64,
        var_t5_dn8_slot: &mut f64,
        var_t5_dn9_slot: &mut f64,
        var_t5_rv_slot: &mut f64,
    ) {
        let mut var_ids: f64 = *var_ids_slot;
        let mut var_ids_dn10: f64 = *var_ids_dn10_slot;
        let mut var_ids_dn11: f64 = *var_ids_dn11_slot;
        let mut var_ids_dn3: f64 = *var_ids_dn3_slot;
        let mut var_ids_dn4: f64 = *var_ids_dn4_slot;
        let mut var_ids_dn5: f64 = *var_ids_dn5_slot;
        let mut var_ids_dn6: f64 = *var_ids_dn6_slot;
        let mut var_ids_dn7: f64 = *var_ids_dn7_slot;
        let mut var_ids_dn8: f64 = *var_ids_dn8_slot;
        let mut var_ids_dn9: f64 = *var_ids_dn9_slot;
        let mut var_ids_edge: f64 = *var_ids_edge_slot;
        let mut var_ids_edge_dn10: f64 = *var_ids_edge_dn10_slot;
        let mut var_ids_edge_dn11: f64 = *var_ids_edge_dn11_slot;
        let mut var_ids_edge_dn3: f64 = *var_ids_edge_dn3_slot;
        let mut var_ids_edge_dn4: f64 = *var_ids_edge_dn4_slot;
        let mut var_ids_edge_dn5: f64 = *var_ids_edge_dn5_slot;
        let mut var_ids_edge_dn6: f64 = *var_ids_edge_dn6_slot;
        let mut var_ids_edge_dn7: f64 = *var_ids_edge_dn7_slot;
        let mut var_ids_edge_dn8: f64 = *var_ids_edge_dn8_slot;
        let mut var_ids_edge_dn9: f64 = *var_ids_edge_dn9_slot;
        let mut var_ids_edge_rv: f64 = *var_ids_edge_rv_slot;
        let mut var_ids_rv: f64 = *var_ids_rv_slot;
        let mut var_leffnoi_edge: f64 = *var_leffnoi_edge_slot;
        let mut var_leffnoi_edge_rv: f64 = *var_leffnoi_edge_rv_slot;
        let mut var_leffnoisq_edge: f64 = *var_leffnoisq_edge_slot;
        let mut var_leffnoisq_edge_rv: f64 = *var_leffnoisq_edge_rv_slot;
        let mut var_n0: f64 = *var_n0_slot;
        let mut var_n0_dn10: f64 = *var_n0_dn10_slot;
        let mut var_n0_dn11: f64 = *var_n0_dn11_slot;
        let mut var_n0_dn3: f64 = *var_n0_dn3_slot;
        let mut var_n0_dn4: f64 = *var_n0_dn4_slot;
        let mut var_n0_dn5: f64 = *var_n0_dn5_slot;
        let mut var_n0_dn6: f64 = *var_n0_dn6_slot;
        let mut var_n0_dn7: f64 = *var_n0_dn7_slot;
        let mut var_n0_dn8: f64 = *var_n0_dn8_slot;
        let mut var_n0_dn9: f64 = *var_n0_dn9_slot;
        let mut var_n0_rv: f64 = *var_n0_rv_slot;
        let mut var_nl: f64 = *var_nl_slot;
        let mut var_nl_dn10: f64 = *var_nl_dn10_slot;
        let mut var_nl_dn11: f64 = *var_nl_dn11_slot;
        let mut var_nl_dn3: f64 = *var_nl_dn3_slot;
        let mut var_nl_dn4: f64 = *var_nl_dn4_slot;
        let mut var_nl_dn5: f64 = *var_nl_dn5_slot;
        let mut var_nl_dn6: f64 = *var_nl_dn6_slot;
        let mut var_nl_dn7: f64 = *var_nl_dn7_slot;
        let mut var_nl_dn8: f64 = *var_nl_dn8_slot;
        let mut var_nl_dn9: f64 = *var_nl_dn9_slot;
        let mut var_nl_rv: f64 = *var_nl_rv_slot;
        let mut var_noia_edge: f64 = *var_noia_edge_slot;
        let mut var_noia_edge_rv: f64 = *var_noia_edge_rv_slot;
        let mut var_noib_edge: f64 = *var_noib_edge_slot;
        let mut var_noib_edge_rv: f64 = *var_noib_edge_rv_slot;
        let mut var_noic_edge: f64 = *var_noic_edge_slot;
        let mut var_noic_edge_rv: f64 = *var_noic_edge_rv_slot;
        let mut var_nq_edge: f64 = *var_nq_edge_slot;
        let mut var_nq_edge_dn10: f64 = *var_nq_edge_dn10_slot;
        let mut var_nq_edge_dn11: f64 = *var_nq_edge_dn11_slot;
        let mut var_nq_edge_dn3: f64 = *var_nq_edge_dn3_slot;
        let mut var_nq_edge_dn4: f64 = *var_nq_edge_dn4_slot;
        let mut var_nq_edge_dn5: f64 = *var_nq_edge_dn5_slot;
        let mut var_nq_edge_dn6: f64 = *var_nq_edge_dn6_slot;
        let mut var_nq_edge_dn7: f64 = *var_nq_edge_dn7_slot;
        let mut var_nq_edge_dn8: f64 = *var_nq_edge_dn8_slot;
        let mut var_nq_edge_dn9: f64 = *var_nq_edge_dn9_slot;
        let mut var_nq_edge_rv: f64 = *var_nq_edge_rv_slot;
        let mut var_nstar: f64 = *var_nstar_slot;
        let mut var_nstar_dn10: f64 = *var_nstar_dn10_slot;
        let mut var_nstar_dn11: f64 = *var_nstar_dn11_slot;
        let mut var_nstar_dn3: f64 = *var_nstar_dn3_slot;
        let mut var_nstar_dn4: f64 = *var_nstar_dn4_slot;
        let mut var_nstar_dn5: f64 = *var_nstar_dn5_slot;
        let mut var_nstar_dn6: f64 = *var_nstar_dn6_slot;
        let mut var_nstar_dn7: f64 = *var_nstar_dn7_slot;
        let mut var_nstar_dn8: f64 = *var_nstar_dn8_slot;
        let mut var_nstar_dn9: f64 = *var_nstar_dn9_slot;
        let mut var_nstar_rv: f64 = *var_nstar_rv_slot;
        let mut var_ssi: f64 = *var_ssi_slot;
        let mut var_ssi_dn10: f64 = *var_ssi_dn10_slot;
        let mut var_ssi_dn11: f64 = *var_ssi_dn11_slot;
        let mut var_ssi_dn3: f64 = *var_ssi_dn3_slot;
        let mut var_ssi_dn4: f64 = *var_ssi_dn4_slot;
        let mut var_ssi_dn5: f64 = *var_ssi_dn5_slot;
        let mut var_ssi_dn6: f64 = *var_ssi_dn6_slot;
        let mut var_ssi_dn7: f64 = *var_ssi_dn7_slot;
        let mut var_ssi_dn8: f64 = *var_ssi_dn8_slot;
        let mut var_ssi_dn9: f64 = *var_ssi_dn9_slot;
        let mut var_ssi_rv: f64 = *var_ssi_rv_slot;
        let mut var_swi: f64 = *var_swi_slot;
        let mut var_swi_dn10: f64 = *var_swi_dn10_slot;
        let mut var_swi_dn11: f64 = *var_swi_dn11_slot;
        let mut var_swi_dn3: f64 = *var_swi_dn3_slot;
        let mut var_swi_dn4: f64 = *var_swi_dn4_slot;
        let mut var_swi_dn5: f64 = *var_swi_dn5_slot;
        let mut var_swi_dn6: f64 = *var_swi_dn6_slot;
        let mut var_swi_dn7: f64 = *var_swi_dn7_slot;
        let mut var_swi_dn8: f64 = *var_swi_dn8_slot;
        let mut var_swi_dn9: f64 = *var_swi_dn9_slot;
        let mut var_swi_rv: f64 = *var_swi_rv_slot;
        let mut var_t0a: f64 = *var_t0a_slot;
        let mut var_t0a_dn10: f64 = *var_t0a_dn10_slot;
        let mut var_t0a_dn11: f64 = *var_t0a_dn11_slot;
        let mut var_t0a_dn3: f64 = *var_t0a_dn3_slot;
        let mut var_t0a_dn4: f64 = *var_t0a_dn4_slot;
        let mut var_t0a_dn5: f64 = *var_t0a_dn5_slot;
        let mut var_t0a_dn6: f64 = *var_t0a_dn6_slot;
        let mut var_t0a_dn7: f64 = *var_t0a_dn7_slot;
        let mut var_t0a_dn8: f64 = *var_t0a_dn8_slot;
        let mut var_t0a_dn9: f64 = *var_t0a_dn9_slot;
        let mut var_t0a_rv: f64 = *var_t0a_rv_slot;
        let mut var_t0b: f64 = *var_t0b_slot;
        let mut var_t0b_dn10: f64 = *var_t0b_dn10_slot;
        let mut var_t0b_dn11: f64 = *var_t0b_dn11_slot;
        let mut var_t0b_dn3: f64 = *var_t0b_dn3_slot;
        let mut var_t0b_dn4: f64 = *var_t0b_dn4_slot;
        let mut var_t0b_dn5: f64 = *var_t0b_dn5_slot;
        let mut var_t0b_dn6: f64 = *var_t0b_dn6_slot;
        let mut var_t0b_dn7: f64 = *var_t0b_dn7_slot;
        let mut var_t0b_dn8: f64 = *var_t0b_dn8_slot;
        let mut var_t0b_dn9: f64 = *var_t0b_dn9_slot;
        let mut var_t0b_rv: f64 = *var_t0b_rv_slot;
        let mut var_t0c: f64 = *var_t0c_slot;
        let mut var_t0c_dn10: f64 = *var_t0c_dn10_slot;
        let mut var_t0c_dn11: f64 = *var_t0c_dn11_slot;
        let mut var_t0c_dn3: f64 = *var_t0c_dn3_slot;
        let mut var_t0c_dn4: f64 = *var_t0c_dn4_slot;
        let mut var_t0c_dn5: f64 = *var_t0c_dn5_slot;
        let mut var_t0c_dn6: f64 = *var_t0c_dn6_slot;
        let mut var_t0c_dn7: f64 = *var_t0c_dn7_slot;
        let mut var_t0c_dn8: f64 = *var_t0c_dn8_slot;
        let mut var_t0c_dn9: f64 = *var_t0c_dn9_slot;
        let mut var_t0c_rv: f64 = *var_t0c_rv_slot;
        let mut var_t0d: f64 = *var_t0d_slot;
        let mut var_t0d_dn10: f64 = *var_t0d_dn10_slot;
        let mut var_t0d_dn11: f64 = *var_t0d_dn11_slot;
        let mut var_t0d_dn3: f64 = *var_t0d_dn3_slot;
        let mut var_t0d_dn4: f64 = *var_t0d_dn4_slot;
        let mut var_t0d_dn5: f64 = *var_t0d_dn5_slot;
        let mut var_t0d_dn6: f64 = *var_t0d_dn6_slot;
        let mut var_t0d_dn7: f64 = *var_t0d_dn7_slot;
        let mut var_t0d_dn8: f64 = *var_t0d_dn8_slot;
        let mut var_t0d_dn9: f64 = *var_t0d_dn9_slot;
        let mut var_t0d_rv: f64 = *var_t0d_rv_slot;
        let mut var_t0e: f64 = *var_t0e_slot;
        let mut var_t0e_dn4: f64 = *var_t0e_dn4_slot;
        let mut var_t0e_dn5: f64 = *var_t0e_dn5_slot;
        let mut var_t0e_rv: f64 = *var_t0e_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_dn9: f64 = *var_t3_dn9_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn10: f64 = *var_t4_dn10_slot;
        let mut var_t4_dn11: f64 = *var_t4_dn11_slot;
        let mut var_t4_dn3: f64 = *var_t4_dn3_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t4_dn9: f64 = *var_t4_dn9_slot;
        let mut var_t4_rv: f64 = *var_t4_rv_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_dn10: f64 = *var_t5_dn10_slot;
        let mut var_t5_dn11: f64 = *var_t5_dn11_slot;
        let mut var_t5_dn3: f64 = *var_t5_dn3_slot;
        let mut var_t5_dn4: f64 = *var_t5_dn4_slot;
        let mut var_t5_dn5: f64 = *var_t5_dn5_slot;
        let mut var_t5_dn6: f64 = *var_t5_dn6_slot;
        let mut var_t5_dn7: f64 = *var_t5_dn7_slot;
        let mut var_t5_dn8: f64 = *var_t5_dn8_slot;
        let mut var_t5_dn9: f64 = *var_t5_dn9_slot;
        let mut var_t5_rv: f64 = *var_t5_rv_slot;

        let (assign60220_e97861, assign60220_e97861_d_n3, assign60220_e97861_d_n4, assign60220_e97861_d_n5, assign60220_e97861_d_n6, assign60220_e97861_d_n7, assign60220_e97861_d_n8, assign60220_e97861_d_n9, assign60220_e97861_d_n10, assign60220_e97861_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign60220_e97859: f64 = (var_t0).sqrt();
        (assign60220_e97859, (var_t0_dn3 / (2.0 * assign60220_e97859)), (var_t0_dn4 / (2.0 * assign60220_e97859)), (var_t0_dn5 / (2.0 * assign60220_e97859)), (var_t0_dn6 / (2.0 * assign60220_e97859)), (var_t0_dn7 / (2.0 * assign60220_e97859)), (var_t0_dn8 / (2.0 * assign60220_e97859)), (var_t0_dn9 / (2.0 * assign60220_e97859)), (var_t0_dn10 / (2.0 * assign60220_e97859)), (var_t0_dn11 / (2.0 * assign60220_e97859)),)
    } else {
        (var_t2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11,)
    }
};
        var_t2 = assign60220_e97861;
        var_t2_dn3 = assign60220_e97861_d_n3;
        var_t2_dn4 = assign60220_e97861_d_n4;
        var_t2_dn5 = assign60220_e97861_d_n5;
        var_t2_dn6 = assign60220_e97861_d_n6;
        var_t2_dn7 = assign60220_e97861_d_n7;
        var_t2_dn8 = assign60220_e97861_d_n8;
        var_t2_dn9 = assign60220_e97861_d_n9;
        var_t2_dn10 = assign60220_e97861_d_n10;
        var_t2_dn11 = assign60220_e97861_d_n11;
        var_t2_rv = 0.0;

        let (assign60230_e97874, assign60230_e97874_d_n3, assign60230_e97874_d_n4, assign60230_e97874_d_n5, assign60230_e97874_d_n6, assign60230_e97874_d_n7, assign60230_e97874_d_n8, assign60230_e97874_d_n9, assign60230_e97874_d_n10, assign60230_e97874_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign60230_e97870: f64 = (var_sqrtpsip + var_t2);
        let assign60230_e97871: f64 = (var_gam_edge / assign60230_e97870);
        let assign60230_e97872: f64 = (1.0 + assign60230_e97871);
        (assign60230_e97872, (((var_gam_edge_dn3 * assign60230_e97870) - (var_gam_edge * (var_sqrtpsip_dn3 + var_t2_dn3))) / (assign60230_e97870 * assign60230_e97870)), (((var_gam_edge_dn4 * assign60230_e97870) - (var_gam_edge * (var_sqrtpsip_dn4 + var_t2_dn4))) / (assign60230_e97870 * assign60230_e97870)), (((var_gam_edge_dn5 * assign60230_e97870) - (var_gam_edge * (var_sqrtpsip_dn5 + var_t2_dn5))) / (assign60230_e97870 * assign60230_e97870)), (((var_gam_edge_dn6 * assign60230_e97870) - (var_gam_edge * (var_sqrtpsip_dn6 + var_t2_dn6))) / (assign60230_e97870 * assign60230_e97870)), (((var_gam_edge_dn7 * assign60230_e97870) - (var_gam_edge * (var_sqrtpsip_dn7 + var_t2_dn7))) / (assign60230_e97870 * assign60230_e97870)), (((var_gam_edge_dn8 * assign60230_e97870) - (var_gam_edge * (var_sqrtpsip_dn8 + var_t2_dn8))) / (assign60230_e97870 * assign60230_e97870)), (((var_gam_edge_dn9 * assign60230_e97870) - (var_gam_edge * (var_sqrtpsip_dn9 + var_t2_dn9))) / (assign60230_e97870 * assign60230_e97870)), (((var_gam_edge_dn10 * assign60230_e97870) - (var_gam_edge * (var_sqrtpsip_dn10 + var_t2_dn10))) / (assign60230_e97870 * assign60230_e97870)), (((var_gam_edge_dn11 * assign60230_e97870) - (var_gam_edge * (var_sqrtpsip_dn11 + var_t2_dn11))) / (assign60230_e97870 * assign60230_e97870)),)
    } else {
        (var_nq_edge, var_nq_edge_dn3, var_nq_edge_dn4, var_nq_edge_dn5, var_nq_edge_dn6, var_nq_edge_dn7, var_nq_edge_dn8, var_nq_edge_dn9, var_nq_edge_dn10, var_nq_edge_dn11,)
    }
};
        var_nq_edge = assign60230_e97874;
        var_nq_edge_dn3 = assign60230_e97874_d_n3;
        var_nq_edge_dn4 = assign60230_e97874_d_n4;
        var_nq_edge_dn5 = assign60230_e97874_d_n5;
        var_nq_edge_dn6 = assign60230_e97874_d_n6;
        var_nq_edge_dn7 = assign60230_e97874_d_n7;
        var_nq_edge_dn8 = assign60230_e97874_d_n8;
        var_nq_edge_dn9 = assign60230_e97874_d_n9;
        var_nq_edge_dn10 = assign60230_e97874_d_n10;
        var_nq_edge_dn11 = assign60230_e97874_d_n11;
        var_nq_edge_rv = 0.0;

        let (assign60240_e97909, assign60240_e97909_d_n3, assign60240_e97909_d_n4, assign60240_e97909_d_n5, assign60240_e97909_d_n6, assign60240_e97909_d_n7, assign60240_e97909_d_n8, assign60240_e97909_d_n9, assign60240_e97909_d_n10, assign60240_e97909_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign60240_e97881: f64 = (2.0 * p.p2);
        let assign60240_e97883: f64 = (assign60240_e97881 * var_nq_edge);
        let assign60240_e97885: f64 = (assign60240_e97883 * var_ueff);
        let assign60240_e97887: f64 = (assign60240_e97885 * p.p1147);
        let assign60240_e97889: f64 = (assign60240_e97887 / var_leff);
        let assign60240_e97891: f64 = (assign60240_e97889 * var_cox);
        let assign60240_e97893: f64 = (assign60240_e97891 * var_nvt);
        let assign60240_e97895: f64 = (assign60240_e97893 * var_nvt);
        let assign60240_e97898: f64 = (var_qs_edge - var_qdeff_edge);
        let assign60240_e97901: f64 = (1.0 + var_qs_edge);
        let assign60240_e97903: f64 = (assign60240_e97901 + var_qdeff_edge);
        let assign60240_e97904: f64 = (assign60240_e97898 * assign60240_e97903);
        let assign60240_e97905: f64 = (assign60240_e97895 * assign60240_e97904);
        let assign60240_e97907: f64 = (assign60240_e97905 * var_moc);
        (assign60240_e97907, ((((((((((((((assign60240_e97881 * var_nq_edge_dn3) * var_ueff) + (assign60240_e97883 * var_ueff_dn3)) * p.p1147) / var_leff) * var_cox) * var_nvt) + (assign60240_e97891 * var_nvt_dn3)) * var_nvt) + (assign60240_e97893 * var_nvt_dn3)) * assign60240_e97904) + (assign60240_e97895 * (((var_qs_edge_dn3 - var_qdeff_edge_dn3) * assign60240_e97903) + (assign60240_e97898 * (var_qs_edge_dn3 + var_qdeff_edge_dn3))))) * var_moc) + (assign60240_e97905 * var_moc_dn3)), ((((((((((((((assign60240_e97881 * var_nq_edge_dn4) * var_ueff) + (assign60240_e97883 * var_ueff_dn4)) * p.p1147) / var_leff) * var_cox) * var_nvt) + (assign60240_e97891 * var_nvt_dn4)) * var_nvt) + (assign60240_e97893 * var_nvt_dn4)) * assign60240_e97904) + (assign60240_e97895 * (((var_qs_edge_dn4 - var_qdeff_edge_dn4) * assign60240_e97903) + (assign60240_e97898 * (var_qs_edge_dn4 + var_qdeff_edge_dn4))))) * var_moc) + (assign60240_e97905 * var_moc_dn4)), ((((((((((((((assign60240_e97881 * var_nq_edge_dn5) * var_ueff) + (assign60240_e97883 * var_ueff_dn5)) * p.p1147) / var_leff) * var_cox) * var_nvt) + (assign60240_e97891 * var_nvt_dn5)) * var_nvt) + (assign60240_e97893 * var_nvt_dn5)) * assign60240_e97904) + (assign60240_e97895 * (((var_qs_edge_dn5 - var_qdeff_edge_dn5) * assign60240_e97903) + (assign60240_e97898 * (var_qs_edge_dn5 + var_qdeff_edge_dn5))))) * var_moc) + (assign60240_e97905 * var_moc_dn5)), ((((((((((((((assign60240_e97881 * var_nq_edge_dn6) * var_ueff) + (assign60240_e97883 * var_ueff_dn6)) * p.p1147) / var_leff) * var_cox) * var_nvt) + (assign60240_e97891 * var_nvt_dn6)) * var_nvt) + (assign60240_e97893 * var_nvt_dn6)) * assign60240_e97904) + (assign60240_e97895 * (((var_qs_edge_dn6 - var_qdeff_edge_dn6) * assign60240_e97903) + (assign60240_e97898 * (var_qs_edge_dn6 + var_qdeff_edge_dn6))))) * var_moc) + (assign60240_e97905 * var_moc_dn6)), ((((((((((((((assign60240_e97881 * var_nq_edge_dn7) * var_ueff) + (assign60240_e97883 * var_ueff_dn7)) * p.p1147) / var_leff) * var_cox) * var_nvt) + (assign60240_e97891 * var_nvt_dn7)) * var_nvt) + (assign60240_e97893 * var_nvt_dn7)) * assign60240_e97904) + (assign60240_e97895 * (((var_qs_edge_dn7 - var_qdeff_edge_dn7) * assign60240_e97903) + (assign60240_e97898 * (var_qs_edge_dn7 + var_qdeff_edge_dn7))))) * var_moc) + (assign60240_e97905 * var_moc_dn7)), ((((((((((((((assign60240_e97881 * var_nq_edge_dn8) * var_ueff) + (assign60240_e97883 * var_ueff_dn8)) * p.p1147) / var_leff) * var_cox) * var_nvt) + (assign60240_e97891 * var_nvt_dn8)) * var_nvt) + (assign60240_e97893 * var_nvt_dn8)) * assign60240_e97904) + (assign60240_e97895 * (((var_qs_edge_dn8 - var_qdeff_edge_dn8) * assign60240_e97903) + (assign60240_e97898 * (var_qs_edge_dn8 + var_qdeff_edge_dn8))))) * var_moc) + (assign60240_e97905 * var_moc_dn8)), ((((((((((((((assign60240_e97881 * var_nq_edge_dn9) * var_ueff) + (assign60240_e97883 * var_ueff_dn9)) * p.p1147) / var_leff) * var_cox) * var_nvt) + (assign60240_e97891 * var_nvt_dn9)) * var_nvt) + (assign60240_e97893 * var_nvt_dn9)) * assign60240_e97904) + (assign60240_e97895 * (((var_qs_edge_dn9 - var_qdeff_edge_dn9) * assign60240_e97903) + (assign60240_e97898 * (var_qs_edge_dn9 + var_qdeff_edge_dn9))))) * var_moc) + (assign60240_e97905 * var_moc_dn9)), ((((((((((((((assign60240_e97881 * var_nq_edge_dn10) * var_ueff) + (assign60240_e97883 * var_ueff_dn10)) * p.p1147) / var_leff) * var_cox) * var_nvt) + (assign60240_e97891 * var_nvt_dn10)) * var_nvt) + (assign60240_e97893 * var_nvt_dn10)) * assign60240_e97904) + (assign60240_e97895 * (((var_qs_edge_dn10 - var_qdeff_edge_dn10) * assign60240_e97903) + (assign60240_e97898 * (var_qs_edge_dn10 + var_qdeff_edge_dn10))))) * var_moc) + (assign60240_e97905 * var_moc_dn10)), ((((((((((((((assign60240_e97881 * var_nq_edge_dn11) * var_ueff) + (assign60240_e97883 * var_ueff_dn11)) * p.p1147) / var_leff) * var_cox) * var_nvt) + (assign60240_e97891 * var_nvt_dn11)) * var_nvt) + (assign60240_e97893 * var_nvt_dn11)) * assign60240_e97904) + (assign60240_e97895 * (((var_qs_edge_dn11 - var_qdeff_edge_dn11) * assign60240_e97903) + (assign60240_e97898 * (var_qs_edge_dn11 + var_qdeff_edge_dn11))))) * var_moc) + (assign60240_e97905 * var_moc_dn11)),)
    } else {
        (var_ids_edge, var_ids_edge_dn3, var_ids_edge_dn4, var_ids_edge_dn5, var_ids_edge_dn6, var_ids_edge_dn7, var_ids_edge_dn8, var_ids_edge_dn9, var_ids_edge_dn10, var_ids_edge_dn11,)
    }
};
        var_ids_edge = assign60240_e97909;
        var_ids_edge_dn3 = assign60240_e97909_d_n3;
        var_ids_edge_dn4 = assign60240_e97909_d_n4;
        var_ids_edge_dn5 = assign60240_e97909_d_n5;
        var_ids_edge_dn6 = assign60240_e97909_d_n6;
        var_ids_edge_dn7 = assign60240_e97909_d_n7;
        var_ids_edge_dn8 = assign60240_e97909_d_n8;
        var_ids_edge_dn9 = assign60240_e97909_d_n9;
        var_ids_edge_dn10 = assign60240_e97909_d_n10;
        var_ids_edge_dn11 = assign60240_e97909_d_n11;
        var_ids_edge_rv = 0.0;

        let (assign60250_e97918, assign60250_e97918_d_n3, assign60250_e97918_d_n4, assign60250_e97918_d_n5, assign60250_e97918_d_n6, assign60250_e97918_d_n7, assign60250_e97918_d_n8, assign60250_e97918_d_n9, assign60250_e97918_d_n10, assign60250_e97918_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign60250_e97916: f64 = (var_ids_edge + var_ids);
        (assign60250_e97916, (var_ids_edge_dn3 + var_ids_dn3), (var_ids_edge_dn4 + var_ids_dn4), (var_ids_edge_dn5 + var_ids_dn5), (var_ids_edge_dn6 + var_ids_dn6), (var_ids_edge_dn7 + var_ids_dn7), (var_ids_edge_dn8 + var_ids_dn8), (var_ids_edge_dn9 + var_ids_dn9), (var_ids_edge_dn10 + var_ids_dn10), (var_ids_edge_dn11 + var_ids_dn11),)
    } else {
        (var_ids, var_ids_dn3, var_ids_dn4, var_ids_dn5, var_ids_dn6, var_ids_dn7, var_ids_dn8, var_ids_dn9, var_ids_dn10, var_ids_dn11,)
    }
};
        var_ids = assign60250_e97918;
        var_ids_dn3 = assign60250_e97918_d_n3;
        var_ids_dn4 = assign60250_e97918_d_n4;
        var_ids_dn5 = assign60250_e97918_d_n5;
        var_ids_dn6 = assign60250_e97918_d_n6;
        var_ids_dn7 = assign60250_e97918_d_n7;
        var_ids_dn8 = assign60250_e97918_d_n8;
        var_ids_dn9 = assign60250_e97918_d_n9;
        var_ids_dn10 = assign60250_e97918_d_n10;
        var_ids_dn11 = assign60250_e97918_d_n11;
        var_ids_rv = 0.0;

        let (assign60260_e97927,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign60260_e97925: f64 = (p.p1012 * p.p1316);
        (assign60260_e97925,)
    } else {
        (var_noia_edge,)
    }
};
        var_noia_edge = assign60260_e97927;
        var_noia_edge_rv = 0.0;

        let (assign60270_e97936,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign60270_e97934: f64 = (p.p1013 * p.p1316);
        (assign60270_e97934,)
    } else {
        (var_noib_edge,)
    }
};
        var_noib_edge = assign60270_e97936;
        var_noib_edge_rv = 0.0;

        let (assign60280_e97945,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign60280_e97943: f64 = (p.p1014 * p.p1316);
        (assign60280_e97943,)
    } else {
        (var_noic_edge,)
    }
};
        var_noic_edge = assign60280_e97945;
        var_noic_edge_rv = 0.0;

        let (assign60290_e97956,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign60290_e97953: f64 = (2.0 * var_lintnoi_i);
        let assign60290_e97954: f64 = (var_leff - assign60290_e97953);
        (assign60290_e97954,)
    } else {
        (var_leffnoi_edge,)
    }
};
        var_leffnoi_edge = assign60290_e97956;
        var_leffnoi_edge_rv = 0.0;

        let (assign60300_e97965,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign60300_e97963: f64 = (var_leffnoi_edge * var_leffnoi_edge);
        (assign60300_e97963,)
    } else {
        (var_leffnoisq_edge,)
    }
};
        var_leffnoisq_edge = assign60300_e97965;
        var_leffnoisq_edge_rv = 0.0;

        let (assign60310_e97980, assign60310_e97980_d_n3, assign60310_e97980_d_n4, assign60310_e97980_d_n5, assign60310_e97980_d_n6, assign60310_e97980_d_n7, assign60310_e97980_d_n8, assign60310_e97980_d_n9, assign60310_e97980_d_n10, assign60310_e97980_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign60310_e97972: f64 = (var_vt / 1.602176462e-19);
        let assign60310_e97975: f64 = (var_cox + var_cdep);
        let assign60310_e97977: f64 = (assign60310_e97975 + var_citedge_i);
        let assign60310_e97978: f64 = (assign60310_e97972 * assign60310_e97977);
        (assign60310_e97978, (assign60310_e97972 * var_cdep_dn3), (((var_vt_dn4 / 1.602176462e-19) * assign60310_e97977) + (assign60310_e97972 * var_cdep_dn4)), (((var_vt_dn5 / 1.602176462e-19) * assign60310_e97977) + (assign60310_e97972 * var_cdep_dn5)), (assign60310_e97972 * var_cdep_dn6), (assign60310_e97972 * var_cdep_dn7), (assign60310_e97972 * var_cdep_dn8), (assign60310_e97972 * var_cdep_dn9), (assign60310_e97972 * var_cdep_dn10), (assign60310_e97972 * var_cdep_dn11),)
    } else {
        (var_nstar, var_nstar_dn3, var_nstar_dn4, var_nstar_dn5, var_nstar_dn6, var_nstar_dn7, var_nstar_dn8, var_nstar_dn9, var_nstar_dn10, var_nstar_dn11,)
    }
};
        var_nstar = assign60310_e97980;
        var_nstar_dn3 = assign60310_e97980_d_n3;
        var_nstar_dn4 = assign60310_e97980_d_n4;
        var_nstar_dn5 = assign60310_e97980_d_n5;
        var_nstar_dn6 = assign60310_e97980_d_n6;
        var_nstar_dn7 = assign60310_e97980_d_n7;
        var_nstar_dn8 = assign60310_e97980_d_n8;
        var_nstar_dn9 = assign60310_e97980_d_n9;
        var_nstar_dn10 = assign60310_e97980_d_n10;
        var_nstar_dn11 = assign60310_e97980_d_n11;
        var_nstar_rv = 0.0;

        let (assign60320_e97997, assign60320_e97997_d_n3, assign60320_e97997_d_n4, assign60320_e97997_d_n5, assign60320_e97997_d_n6, assign60320_e97997_d_n7, assign60320_e97997_d_n8, assign60320_e97997_d_n9, assign60320_e97997_d_n10, assign60320_e97997_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign60320_e97987: f64 = (2.0 * var_nq_edge);
        let assign60320_e97989: f64 = (assign60320_e97987 * var_cox);
        let assign60320_e97991: f64 = (assign60320_e97989 * var_vt);
        let assign60320_e97993: f64 = (assign60320_e97991 * var_qdeff_edge);
        let assign60320_e97995: f64 = (assign60320_e97993 / 1.602176462e-19);
        (assign60320_e97995, ((((((2.0 * var_nq_edge_dn3) * var_cox) * var_vt) * var_qdeff_edge) + (assign60320_e97991 * var_qdeff_edge_dn3)) / 1.602176462e-19), (((((((2.0 * var_nq_edge_dn4) * var_cox) * var_vt) + (assign60320_e97989 * var_vt_dn4)) * var_qdeff_edge) + (assign60320_e97991 * var_qdeff_edge_dn4)) / 1.602176462e-19), (((((((2.0 * var_nq_edge_dn5) * var_cox) * var_vt) + (assign60320_e97989 * var_vt_dn5)) * var_qdeff_edge) + (assign60320_e97991 * var_qdeff_edge_dn5)) / 1.602176462e-19), ((((((2.0 * var_nq_edge_dn6) * var_cox) * var_vt) * var_qdeff_edge) + (assign60320_e97991 * var_qdeff_edge_dn6)) / 1.602176462e-19), ((((((2.0 * var_nq_edge_dn7) * var_cox) * var_vt) * var_qdeff_edge) + (assign60320_e97991 * var_qdeff_edge_dn7)) / 1.602176462e-19), ((((((2.0 * var_nq_edge_dn8) * var_cox) * var_vt) * var_qdeff_edge) + (assign60320_e97991 * var_qdeff_edge_dn8)) / 1.602176462e-19), ((((((2.0 * var_nq_edge_dn9) * var_cox) * var_vt) * var_qdeff_edge) + (assign60320_e97991 * var_qdeff_edge_dn9)) / 1.602176462e-19), ((((((2.0 * var_nq_edge_dn10) * var_cox) * var_vt) * var_qdeff_edge) + (assign60320_e97991 * var_qdeff_edge_dn10)) / 1.602176462e-19), ((((((2.0 * var_nq_edge_dn11) * var_cox) * var_vt) * var_qdeff_edge) + (assign60320_e97991 * var_qdeff_edge_dn11)) / 1.602176462e-19),)
    } else {
        (var_nl, var_nl_dn3, var_nl_dn4, var_nl_dn5, var_nl_dn6, var_nl_dn7, var_nl_dn8, var_nl_dn9, var_nl_dn10, var_nl_dn11,)
    }
};
        var_nl = assign60320_e97997;
        var_nl_dn3 = assign60320_e97997_d_n3;
        var_nl_dn4 = assign60320_e97997_d_n4;
        var_nl_dn5 = assign60320_e97997_d_n5;
        var_nl_dn6 = assign60320_e97997_d_n6;
        var_nl_dn7 = assign60320_e97997_d_n7;
        var_nl_dn8 = assign60320_e97997_d_n8;
        var_nl_dn9 = assign60320_e97997_d_n9;
        var_nl_dn10 = assign60320_e97997_d_n10;
        var_nl_dn11 = assign60320_e97997_d_n11;
        var_nl_rv = 0.0;

        let (assign60330_e98015, assign60330_e98015_d_n3, assign60330_e98015_d_n4, assign60330_e98015_d_n5, assign60330_e98015_d_n6, assign60330_e98015_d_n7, assign60330_e98015_d_n8, assign60330_e98015_d_n9, assign60330_e98015_d_n10, assign60330_e98015_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign60330_e98004: f64 = (1.602176462e-19 * 1.602176462e-19);
        let assign60330_e98006: f64 = (assign60330_e98004 * 1.602176462e-19);
        let assign60330_e98008: f64 = (assign60330_e98006 * var_vt);
        let assign60330_e98010: f64 = (var_ids_edge).abs();
        let assign60330_e98011: f64 = (assign60330_e98008 * assign60330_e98010);
        let assign60330_e98013: f64 = (assign60330_e98011 * var_ueff);
        (assign60330_e98013, (((assign60330_e98008 * if var_ids_edge >= 0.0 { var_ids_edge_dn3 } else { (-var_ids_edge_dn3) }) * var_ueff) + (assign60330_e98011 * var_ueff_dn3)), (((((assign60330_e98006 * var_vt_dn4) * assign60330_e98010) + (assign60330_e98008 * if var_ids_edge >= 0.0 { var_ids_edge_dn4 } else { (-var_ids_edge_dn4) })) * var_ueff) + (assign60330_e98011 * var_ueff_dn4)), (((((assign60330_e98006 * var_vt_dn5) * assign60330_e98010) + (assign60330_e98008 * if var_ids_edge >= 0.0 { var_ids_edge_dn5 } else { (-var_ids_edge_dn5) })) * var_ueff) + (assign60330_e98011 * var_ueff_dn5)), (((assign60330_e98008 * if var_ids_edge >= 0.0 { var_ids_edge_dn6 } else { (-var_ids_edge_dn6) }) * var_ueff) + (assign60330_e98011 * var_ueff_dn6)), (((assign60330_e98008 * if var_ids_edge >= 0.0 { var_ids_edge_dn7 } else { (-var_ids_edge_dn7) }) * var_ueff) + (assign60330_e98011 * var_ueff_dn7)), (((assign60330_e98008 * if var_ids_edge >= 0.0 { var_ids_edge_dn8 } else { (-var_ids_edge_dn8) }) * var_ueff) + (assign60330_e98011 * var_ueff_dn8)), (((assign60330_e98008 * if var_ids_edge >= 0.0 { var_ids_edge_dn9 } else { (-var_ids_edge_dn9) }) * var_ueff) + (assign60330_e98011 * var_ueff_dn9)), (((assign60330_e98008 * if var_ids_edge >= 0.0 { var_ids_edge_dn10 } else { (-var_ids_edge_dn10) }) * var_ueff) + (assign60330_e98011 * var_ueff_dn10)), (((assign60330_e98008 * if var_ids_edge >= 0.0 { var_ids_edge_dn11 } else { (-var_ids_edge_dn11) }) * var_ueff) + (assign60330_e98011 * var_ueff_dn11)),)
    } else {
        (var_t0a, var_t0a_dn3, var_t0a_dn4, var_t0a_dn5, var_t0a_dn6, var_t0a_dn7, var_t0a_dn8, var_t0a_dn9, var_t0a_dn10, var_t0a_dn11,)
    }
};
        var_t0a = assign60330_e98015;
        var_t0a_dn3 = assign60330_e98015_d_n3;
        var_t0a_dn4 = assign60330_e98015_d_n4;
        var_t0a_dn5 = assign60330_e98015_d_n5;
        var_t0a_dn6 = assign60330_e98015_d_n6;
        var_t0a_dn7 = assign60330_e98015_d_n7;
        var_t0a_dn8 = assign60330_e98015_d_n8;
        var_t0a_dn9 = assign60330_e98015_d_n9;
        var_t0a_dn10 = assign60330_e98015_d_n10;
        var_t0a_dn11 = assign60330_e98015_d_n11;
        var_t0a_rv = 0.0;

        let (assign60340_e98028, assign60340_e98028_d_n3, assign60340_e98028_d_n4, assign60340_e98028_d_n5, assign60340_e98028_d_n6, assign60340_e98028_d_n7, assign60340_e98028_d_n8, assign60340_e98028_d_n9, assign60340_e98028_d_n10, assign60340_e98028_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign60340_e98022: f64 = (1.602176462e-19 * var_vt);
        let assign60340_e98024: f64 = (assign60340_e98022 * var_ids_edge);
        let assign60340_e98026: f64 = (assign60340_e98024 * var_ids_edge);
        (assign60340_e98026, (((assign60340_e98022 * var_ids_edge_dn3) * var_ids_edge) + (assign60340_e98024 * var_ids_edge_dn3)), (((((1.602176462e-19 * var_vt_dn4) * var_ids_edge) + (assign60340_e98022 * var_ids_edge_dn4)) * var_ids_edge) + (assign60340_e98024 * var_ids_edge_dn4)), (((((1.602176462e-19 * var_vt_dn5) * var_ids_edge) + (assign60340_e98022 * var_ids_edge_dn5)) * var_ids_edge) + (assign60340_e98024 * var_ids_edge_dn5)), (((assign60340_e98022 * var_ids_edge_dn6) * var_ids_edge) + (assign60340_e98024 * var_ids_edge_dn6)), (((assign60340_e98022 * var_ids_edge_dn7) * var_ids_edge) + (assign60340_e98024 * var_ids_edge_dn7)), (((assign60340_e98022 * var_ids_edge_dn8) * var_ids_edge) + (assign60340_e98024 * var_ids_edge_dn8)), (((assign60340_e98022 * var_ids_edge_dn9) * var_ids_edge) + (assign60340_e98024 * var_ids_edge_dn9)), (((assign60340_e98022 * var_ids_edge_dn10) * var_ids_edge) + (assign60340_e98024 * var_ids_edge_dn10)), (((assign60340_e98022 * var_ids_edge_dn11) * var_ids_edge) + (assign60340_e98024 * var_ids_edge_dn11)),)
    } else {
        (var_t0b, var_t0b_dn3, var_t0b_dn4, var_t0b_dn5, var_t0b_dn6, var_t0b_dn7, var_t0b_dn8, var_t0b_dn9, var_t0b_dn10, var_t0b_dn11,)
    }
};
        var_t0b = assign60340_e98028;
        var_t0b_dn3 = assign60340_e98028_d_n3;
        var_t0b_dn4 = assign60340_e98028_d_n4;
        var_t0b_dn5 = assign60340_e98028_d_n5;
        var_t0b_dn6 = assign60340_e98028_d_n6;
        var_t0b_dn7 = assign60340_e98028_d_n7;
        var_t0b_dn8 = assign60340_e98028_d_n8;
        var_t0b_dn9 = assign60340_e98028_d_n9;
        var_t0b_dn10 = assign60340_e98028_d_n10;
        var_t0b_dn11 = assign60340_e98028_d_n11;
        var_t0b_rv = 0.0;

        let (assign60350_e98045, assign60350_e98045_d_n3, assign60350_e98045_d_n4, assign60350_e98045_d_n5, assign60350_e98045_d_n6, assign60350_e98045_d_n7, assign60350_e98045_d_n8, assign60350_e98045_d_n9, assign60350_e98045_d_n10, assign60350_e98045_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign60350_e98036: f64 = (var_noib_edge * var_nl);
        let assign60350_e98037: f64 = (var_noia_edge + assign60350_e98036);
        let assign60350_e98040: f64 = (var_noic_edge * var_nl);
        let assign60350_e98042: f64 = (assign60350_e98040 * var_nl);
        let assign60350_e98043: f64 = (assign60350_e98037 + assign60350_e98042);
        (assign60350_e98043, ((var_noib_edge * var_nl_dn3) + (((var_noic_edge * var_nl_dn3) * var_nl) + (assign60350_e98040 * var_nl_dn3))), ((var_noib_edge * var_nl_dn4) + (((var_noic_edge * var_nl_dn4) * var_nl) + (assign60350_e98040 * var_nl_dn4))), ((var_noib_edge * var_nl_dn5) + (((var_noic_edge * var_nl_dn5) * var_nl) + (assign60350_e98040 * var_nl_dn5))), ((var_noib_edge * var_nl_dn6) + (((var_noic_edge * var_nl_dn6) * var_nl) + (assign60350_e98040 * var_nl_dn6))), ((var_noib_edge * var_nl_dn7) + (((var_noic_edge * var_nl_dn7) * var_nl) + (assign60350_e98040 * var_nl_dn7))), ((var_noib_edge * var_nl_dn8) + (((var_noic_edge * var_nl_dn8) * var_nl) + (assign60350_e98040 * var_nl_dn8))), ((var_noib_edge * var_nl_dn9) + (((var_noic_edge * var_nl_dn9) * var_nl) + (assign60350_e98040 * var_nl_dn9))), ((var_noib_edge * var_nl_dn10) + (((var_noic_edge * var_nl_dn10) * var_nl) + (assign60350_e98040 * var_nl_dn10))), ((var_noib_edge * var_nl_dn11) + (((var_noic_edge * var_nl_dn11) * var_nl) + (assign60350_e98040 * var_nl_dn11))),)
    } else {
        (var_t0c, var_t0c_dn3, var_t0c_dn4, var_t0c_dn5, var_t0c_dn6, var_t0c_dn7, var_t0c_dn8, var_t0c_dn9, var_t0c_dn10, var_t0c_dn11,)
    }
};
        var_t0c = assign60350_e98045;
        var_t0c_dn3 = assign60350_e98045_d_n3;
        var_t0c_dn4 = assign60350_e98045_d_n4;
        var_t0c_dn5 = assign60350_e98045_d_n5;
        var_t0c_dn6 = assign60350_e98045_d_n6;
        var_t0c_dn7 = assign60350_e98045_d_n7;
        var_t0c_dn8 = assign60350_e98045_d_n8;
        var_t0c_dn9 = assign60350_e98045_d_n9;
        var_t0c_dn10 = assign60350_e98045_d_n10;
        var_t0c_dn11 = assign60350_e98045_d_n11;
        var_t0c_rv = 0.0;

        let (assign60360_e98058, assign60360_e98058_d_n3, assign60360_e98058_d_n4, assign60360_e98058_d_n5, assign60360_e98058_d_n6, assign60360_e98058_d_n7, assign60360_e98058_d_n8, assign60360_e98058_d_n9, assign60360_e98058_d_n10, assign60360_e98058_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign60360_e98052: f64 = (var_nl + var_nstar);
        let assign60360_e98055: f64 = (var_nl + var_nstar);
        let assign60360_e98056: f64 = (assign60360_e98052 * assign60360_e98055);
        (assign60360_e98056, (((var_nl_dn3 + var_nstar_dn3) * assign60360_e98055) + (assign60360_e98052 * (var_nl_dn3 + var_nstar_dn3))), (((var_nl_dn4 + var_nstar_dn4) * assign60360_e98055) + (assign60360_e98052 * (var_nl_dn4 + var_nstar_dn4))), (((var_nl_dn5 + var_nstar_dn5) * assign60360_e98055) + (assign60360_e98052 * (var_nl_dn5 + var_nstar_dn5))), (((var_nl_dn6 + var_nstar_dn6) * assign60360_e98055) + (assign60360_e98052 * (var_nl_dn6 + var_nstar_dn6))), (((var_nl_dn7 + var_nstar_dn7) * assign60360_e98055) + (assign60360_e98052 * (var_nl_dn7 + var_nstar_dn7))), (((var_nl_dn8 + var_nstar_dn8) * assign60360_e98055) + (assign60360_e98052 * (var_nl_dn8 + var_nstar_dn8))), (((var_nl_dn9 + var_nstar_dn9) * assign60360_e98055) + (assign60360_e98052 * (var_nl_dn9 + var_nstar_dn9))), (((var_nl_dn10 + var_nstar_dn10) * assign60360_e98055) + (assign60360_e98052 * (var_nl_dn10 + var_nstar_dn10))), (((var_nl_dn11 + var_nstar_dn11) * assign60360_e98055) + (assign60360_e98052 * (var_nl_dn11 + var_nstar_dn11))),)
    } else {
        (var_t0d, var_t0d_dn3, var_t0d_dn4, var_t0d_dn5, var_t0d_dn6, var_t0d_dn7, var_t0d_dn8, var_t0d_dn9, var_t0d_dn10, var_t0d_dn11,)
    }
};
        var_t0d = assign60360_e98058;
        var_t0d_dn3 = assign60360_e98058_d_n3;
        var_t0d_dn4 = assign60360_e98058_d_n4;
        var_t0d_dn5 = assign60360_e98058_d_n5;
        var_t0d_dn6 = assign60360_e98058_d_n6;
        var_t0d_dn7 = assign60360_e98058_d_n7;
        var_t0d_dn8 = assign60360_e98058_d_n8;
        var_t0d_dn9 = assign60360_e98058_d_n9;
        var_t0d_dn10 = assign60360_e98058_d_n10;
        var_t0d_dn11 = assign60360_e98058_d_n11;
        var_t0d_rv = 0.0;

        let (assign60370_e98069, assign60370_e98069_d_n4, assign60370_e98069_d_n5,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign60370_e98065: f64 = (var_noia_edge * 1.602176462e-19);
        let assign60370_e98067: f64 = (assign60370_e98065 * var_vt);
        (assign60370_e98067, (assign60370_e98065 * var_vt_dn4), (assign60370_e98065 * var_vt_dn5),)
    } else {
        (var_t0e, var_t0e_dn4, var_t0e_dn5,)
    }
};
        var_t0e = assign60370_e98069;
        var_t0e_dn4 = assign60370_e98069_d_n4;
        var_t0e_dn5 = assign60370_e98069_d_n5;
        var_t0e_rv = 0.0;

        let (assign60380_e98086, assign60380_e98086_d_n3, assign60380_e98086_d_n4, assign60380_e98086_d_n5, assign60380_e98086_d_n6, assign60380_e98086_d_n7, assign60380_e98086_d_n8, assign60380_e98086_d_n9, assign60380_e98086_d_n10, assign60380_e98086_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign60380_e98076: f64 = (2.0 * var_nq_edge);
        let assign60380_e98078: f64 = (assign60380_e98076 * var_cox);
        let assign60380_e98080: f64 = (assign60380_e98078 * var_vt);
        let assign60380_e98082: f64 = (assign60380_e98080 * var_qs_edge);
        let assign60380_e98084: f64 = (assign60380_e98082 / 1.602176462e-19);
        (assign60380_e98084, ((((((2.0 * var_nq_edge_dn3) * var_cox) * var_vt) * var_qs_edge) + (assign60380_e98080 * var_qs_edge_dn3)) / 1.602176462e-19), (((((((2.0 * var_nq_edge_dn4) * var_cox) * var_vt) + (assign60380_e98078 * var_vt_dn4)) * var_qs_edge) + (assign60380_e98080 * var_qs_edge_dn4)) / 1.602176462e-19), (((((((2.0 * var_nq_edge_dn5) * var_cox) * var_vt) + (assign60380_e98078 * var_vt_dn5)) * var_qs_edge) + (assign60380_e98080 * var_qs_edge_dn5)) / 1.602176462e-19), ((((((2.0 * var_nq_edge_dn6) * var_cox) * var_vt) * var_qs_edge) + (assign60380_e98080 * var_qs_edge_dn6)) / 1.602176462e-19), ((((((2.0 * var_nq_edge_dn7) * var_cox) * var_vt) * var_qs_edge) + (assign60380_e98080 * var_qs_edge_dn7)) / 1.602176462e-19), ((((((2.0 * var_nq_edge_dn8) * var_cox) * var_vt) * var_qs_edge) + (assign60380_e98080 * var_qs_edge_dn8)) / 1.602176462e-19), ((((((2.0 * var_nq_edge_dn9) * var_cox) * var_vt) * var_qs_edge) + (assign60380_e98080 * var_qs_edge_dn9)) / 1.602176462e-19), ((((((2.0 * var_nq_edge_dn10) * var_cox) * var_vt) * var_qs_edge) + (assign60380_e98080 * var_qs_edge_dn10)) / 1.602176462e-19), ((((((2.0 * var_nq_edge_dn11) * var_cox) * var_vt) * var_qs_edge) + (assign60380_e98080 * var_qs_edge_dn11)) / 1.602176462e-19),)
    } else {
        (var_n0, var_n0_dn3, var_n0_dn4, var_n0_dn5, var_n0_dn6, var_n0_dn7, var_n0_dn8, var_n0_dn9, var_n0_dn10, var_n0_dn11,)
    }
};
        var_n0 = assign60380_e98086;
        var_n0_dn3 = assign60380_e98086_d_n3;
        var_n0_dn4 = assign60380_e98086_d_n4;
        var_n0_dn5 = assign60380_e98086_d_n5;
        var_n0_dn6 = assign60380_e98086_d_n6;
        var_n0_dn7 = assign60380_e98086_d_n7;
        var_n0_dn8 = assign60380_e98086_d_n8;
        var_n0_dn9 = assign60380_e98086_d_n9;
        var_n0_dn10 = assign60380_e98086_d_n10;
        var_n0_dn11 = assign60380_e98086_d_n11;
        var_n0_rv = 0.0;

        let (assign60390_e98104, assign60390_e98104_d_n3, assign60390_e98104_d_n4, assign60390_e98104_d_n5, assign60390_e98104_d_n6, assign60390_e98104_d_n7, assign60390_e98104_d_n8, assign60390_e98104_d_n9, assign60390_e98104_d_n10, assign60390_e98104_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign60390_e98094: f64 = (var_n0 + var_nstar);
        let assign60390_e98097: f64 = (var_nl + var_nstar);
        let assign60390_e98098: f64 = (assign60390_e98094 / assign60390_e98097);
        let assign60390_e98100: f64 = (assign60390_e98098).max(1e-38);
        let assign60390_e98101: f64 = (assign60390_e98100).ln();
        let assign60390_e98102: f64 = (var_noia_edge * assign60390_e98101);
        (assign60390_e98102, (var_noia_edge * (if assign60390_e98098 >= 1e-38 { ((((var_n0_dn3 + var_nstar_dn3) * assign60390_e98097) - (assign60390_e98094 * (var_nl_dn3 + var_nstar_dn3))) / (assign60390_e98097 * assign60390_e98097)) } else { 0.0 } / assign60390_e98100)), (var_noia_edge * (if assign60390_e98098 >= 1e-38 { ((((var_n0_dn4 + var_nstar_dn4) * assign60390_e98097) - (assign60390_e98094 * (var_nl_dn4 + var_nstar_dn4))) / (assign60390_e98097 * assign60390_e98097)) } else { 0.0 } / assign60390_e98100)), (var_noia_edge * (if assign60390_e98098 >= 1e-38 { ((((var_n0_dn5 + var_nstar_dn5) * assign60390_e98097) - (assign60390_e98094 * (var_nl_dn5 + var_nstar_dn5))) / (assign60390_e98097 * assign60390_e98097)) } else { 0.0 } / assign60390_e98100)), (var_noia_edge * (if assign60390_e98098 >= 1e-38 { ((((var_n0_dn6 + var_nstar_dn6) * assign60390_e98097) - (assign60390_e98094 * (var_nl_dn6 + var_nstar_dn6))) / (assign60390_e98097 * assign60390_e98097)) } else { 0.0 } / assign60390_e98100)), (var_noia_edge * (if assign60390_e98098 >= 1e-38 { ((((var_n0_dn7 + var_nstar_dn7) * assign60390_e98097) - (assign60390_e98094 * (var_nl_dn7 + var_nstar_dn7))) / (assign60390_e98097 * assign60390_e98097)) } else { 0.0 } / assign60390_e98100)), (var_noia_edge * (if assign60390_e98098 >= 1e-38 { ((((var_n0_dn8 + var_nstar_dn8) * assign60390_e98097) - (assign60390_e98094 * (var_nl_dn8 + var_nstar_dn8))) / (assign60390_e98097 * assign60390_e98097)) } else { 0.0 } / assign60390_e98100)), (var_noia_edge * (if assign60390_e98098 >= 1e-38 { ((((var_n0_dn9 + var_nstar_dn9) * assign60390_e98097) - (assign60390_e98094 * (var_nl_dn9 + var_nstar_dn9))) / (assign60390_e98097 * assign60390_e98097)) } else { 0.0 } / assign60390_e98100)), (var_noia_edge * (if assign60390_e98098 >= 1e-38 { ((((var_n0_dn10 + var_nstar_dn10) * assign60390_e98097) - (assign60390_e98094 * (var_nl_dn10 + var_nstar_dn10))) / (assign60390_e98097 * assign60390_e98097)) } else { 0.0 } / assign60390_e98100)), (var_noia_edge * (if assign60390_e98098 >= 1e-38 { ((((var_n0_dn11 + var_nstar_dn11) * assign60390_e98097) - (assign60390_e98094 * (var_nl_dn11 + var_nstar_dn11))) / (assign60390_e98097 * assign60390_e98097)) } else { 0.0 } / assign60390_e98100)),)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11,)
    }
};
        var_t1 = assign60390_e98104;
        var_t1_dn3 = assign60390_e98104_d_n3;
        var_t1_dn4 = assign60390_e98104_d_n4;
        var_t1_dn5 = assign60390_e98104_d_n5;
        var_t1_dn6 = assign60390_e98104_d_n6;
        var_t1_dn7 = assign60390_e98104_d_n7;
        var_t1_dn8 = assign60390_e98104_d_n8;
        var_t1_dn9 = assign60390_e98104_d_n9;
        var_t1_dn10 = assign60390_e98104_d_n10;
        var_t1_dn11 = assign60390_e98104_d_n11;
        var_t1_rv = 0.0;

        let (assign60400_e98115, assign60400_e98115_d_n3, assign60400_e98115_d_n4, assign60400_e98115_d_n5, assign60400_e98115_d_n6, assign60400_e98115_d_n7, assign60400_e98115_d_n8, assign60400_e98115_d_n9, assign60400_e98115_d_n10, assign60400_e98115_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign60400_e98112: f64 = (var_n0 - var_nl);
        let assign60400_e98113: f64 = (var_noib_edge * assign60400_e98112);
        (assign60400_e98113, (var_noib_edge * (var_n0_dn3 - var_nl_dn3)), (var_noib_edge * (var_n0_dn4 - var_nl_dn4)), (var_noib_edge * (var_n0_dn5 - var_nl_dn5)), (var_noib_edge * (var_n0_dn6 - var_nl_dn6)), (var_noib_edge * (var_n0_dn7 - var_nl_dn7)), (var_noib_edge * (var_n0_dn8 - var_nl_dn8)), (var_noib_edge * (var_n0_dn9 - var_nl_dn9)), (var_noib_edge * (var_n0_dn10 - var_nl_dn10)), (var_noib_edge * (var_n0_dn11 - var_nl_dn11)),)
    } else {
        (var_t2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11,)
    }
};
        var_t2 = assign60400_e98115;
        var_t2_dn3 = assign60400_e98115_d_n3;
        var_t2_dn4 = assign60400_e98115_d_n4;
        var_t2_dn5 = assign60400_e98115_d_n5;
        var_t2_dn6 = assign60400_e98115_d_n6;
        var_t2_dn7 = assign60400_e98115_d_n7;
        var_t2_dn8 = assign60400_e98115_d_n8;
        var_t2_dn9 = assign60400_e98115_d_n9;
        var_t2_dn10 = assign60400_e98115_d_n10;
        var_t2_dn11 = assign60400_e98115_d_n11;
        var_t2_rv = 0.0;

        let (assign60410_e98132, assign60410_e98132_d_n3, assign60410_e98132_d_n4, assign60410_e98132_d_n5, assign60410_e98132_d_n6, assign60410_e98132_d_n7, assign60410_e98132_d_n8, assign60410_e98132_d_n9, assign60410_e98132_d_n10, assign60410_e98132_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign60410_e98122: f64 = (0.5 * var_noic_edge);
        let assign60410_e98125: f64 = (var_n0 * var_n0);
        let assign60410_e98128: f64 = (var_nl * var_nl);
        let assign60410_e98129: f64 = (assign60410_e98125 - assign60410_e98128);
        let assign60410_e98130: f64 = (assign60410_e98122 * assign60410_e98129);
        (assign60410_e98130, (assign60410_e98122 * (((var_n0_dn3 * var_n0) + (var_n0 * var_n0_dn3)) - ((var_nl_dn3 * var_nl) + (var_nl * var_nl_dn3)))), (assign60410_e98122 * (((var_n0_dn4 * var_n0) + (var_n0 * var_n0_dn4)) - ((var_nl_dn4 * var_nl) + (var_nl * var_nl_dn4)))), (assign60410_e98122 * (((var_n0_dn5 * var_n0) + (var_n0 * var_n0_dn5)) - ((var_nl_dn5 * var_nl) + (var_nl * var_nl_dn5)))), (assign60410_e98122 * (((var_n0_dn6 * var_n0) + (var_n0 * var_n0_dn6)) - ((var_nl_dn6 * var_nl) + (var_nl * var_nl_dn6)))), (assign60410_e98122 * (((var_n0_dn7 * var_n0) + (var_n0 * var_n0_dn7)) - ((var_nl_dn7 * var_nl) + (var_nl * var_nl_dn7)))), (assign60410_e98122 * (((var_n0_dn8 * var_n0) + (var_n0 * var_n0_dn8)) - ((var_nl_dn8 * var_nl) + (var_nl * var_nl_dn8)))), (assign60410_e98122 * (((var_n0_dn9 * var_n0) + (var_n0 * var_n0_dn9)) - ((var_nl_dn9 * var_nl) + (var_nl * var_nl_dn9)))), (assign60410_e98122 * (((var_n0_dn10 * var_n0) + (var_n0 * var_n0_dn10)) - ((var_nl_dn10 * var_nl) + (var_nl * var_nl_dn10)))), (assign60410_e98122 * (((var_n0_dn11 * var_n0) + (var_n0 * var_n0_dn11)) - ((var_nl_dn11 * var_nl) + (var_nl * var_nl_dn11)))),)
    } else {
        (var_t3, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11,)
    }
};
        var_t3 = assign60410_e98132;
        var_t3_dn3 = assign60410_e98132_d_n3;
        var_t3_dn4 = assign60410_e98132_d_n4;
        var_t3_dn5 = assign60410_e98132_d_n5;
        var_t3_dn6 = assign60410_e98132_d_n6;
        var_t3_dn7 = assign60410_e98132_d_n7;
        var_t3_dn8 = assign60410_e98132_d_n8;
        var_t3_dn9 = assign60410_e98132_d_n9;
        var_t3_dn10 = assign60410_e98132_d_n10;
        var_t3_dn11 = assign60410_e98132_d_n11;
        var_t3_rv = 0.0;

        let (assign60420_e98145, assign60420_e98145_d_n3, assign60420_e98145_d_n4, assign60420_e98145_d_n5, assign60420_e98145_d_n6, assign60420_e98145_d_n7, assign60420_e98145_d_n8, assign60420_e98145_d_n9, assign60420_e98145_d_n10, assign60420_e98145_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign60420_e98139: f64 = (10000000000.0 * var_leffnoisq_edge);
        let assign60420_e98141: f64 = (assign60420_e98139 * p.p1147);
        let assign60420_e98143: f64 = (assign60420_e98141 * p.p2);
        (assign60420_e98143, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t4, var_t4_dn3, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_dn9, var_t4_dn10, var_t4_dn11,)
    }
};
        var_t4 = assign60420_e98145;
        var_t4_dn3 = assign60420_e98145_d_n3;
        var_t4_dn4 = assign60420_e98145_d_n4;
        var_t4_dn5 = assign60420_e98145_d_n5;
        var_t4_dn6 = assign60420_e98145_d_n6;
        var_t4_dn7 = assign60420_e98145_d_n7;
        var_t4_dn8 = assign60420_e98145_d_n8;
        var_t4_dn9 = assign60420_e98145_d_n9;
        var_t4_dn10 = assign60420_e98145_d_n10;
        var_t4_dn11 = assign60420_e98145_d_n11;
        var_t4_rv = 0.0;

        let (assign60430_e98170, assign60430_e98170_d_n3, assign60430_e98170_d_n4, assign60430_e98170_d_n5, assign60430_e98170_d_n6, assign60430_e98170_d_n7, assign60430_e98170_d_n8, assign60430_e98170_d_n9, assign60430_e98170_d_n10, assign60430_e98170_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign60430_e98152: f64 = (var_t0a / var_t0);
        let assign60430_e98155: f64 = (var_t1 + var_t2);
        let assign60430_e98157: f64 = (assign60430_e98155 + var_t3);
        let assign60430_e98158: f64 = (assign60430_e98152 * assign60430_e98157);
        let assign60430_e98161: f64 = (var_t0b / var_t4);
        let assign60430_e98163: f64 = (assign60430_e98161 * var_delclm);
        let assign60430_e98165: f64 = (assign60430_e98163 * var_t0c);
        let assign60430_e98167: f64 = (assign60430_e98165 / var_t0d);
        let assign60430_e98168: f64 = (assign60430_e98158 + assign60430_e98167);
        (assign60430_e98168, ((((((var_t0a_dn3 * var_t0) - (var_t0a * var_t0_dn3)) / (var_t0 * var_t0)) * assign60430_e98157) + (assign60430_e98152 * ((var_t1_dn3 + var_t2_dn3) + var_t3_dn3))) + ((((((((((var_t0b_dn3 * var_t4) - (var_t0b * var_t4_dn3)) / (var_t4 * var_t4)) * var_delclm) + (assign60430_e98161 * var_delclm_dn3)) * var_t0c) + (assign60430_e98163 * var_t0c_dn3)) * var_t0d) - (assign60430_e98165 * var_t0d_dn3)) / (var_t0d * var_t0d))), ((((((var_t0a_dn4 * var_t0) - (var_t0a * var_t0_dn4)) / (var_t0 * var_t0)) * assign60430_e98157) + (assign60430_e98152 * ((var_t1_dn4 + var_t2_dn4) + var_t3_dn4))) + ((((((((((var_t0b_dn4 * var_t4) - (var_t0b * var_t4_dn4)) / (var_t4 * var_t4)) * var_delclm) + (assign60430_e98161 * var_delclm_dn4)) * var_t0c) + (assign60430_e98163 * var_t0c_dn4)) * var_t0d) - (assign60430_e98165 * var_t0d_dn4)) / (var_t0d * var_t0d))), ((((((var_t0a_dn5 * var_t0) - (var_t0a * var_t0_dn5)) / (var_t0 * var_t0)) * assign60430_e98157) + (assign60430_e98152 * ((var_t1_dn5 + var_t2_dn5) + var_t3_dn5))) + ((((((((((var_t0b_dn5 * var_t4) - (var_t0b * var_t4_dn5)) / (var_t4 * var_t4)) * var_delclm) + (assign60430_e98161 * var_delclm_dn5)) * var_t0c) + (assign60430_e98163 * var_t0c_dn5)) * var_t0d) - (assign60430_e98165 * var_t0d_dn5)) / (var_t0d * var_t0d))), ((((((var_t0a_dn6 * var_t0) - (var_t0a * var_t0_dn6)) / (var_t0 * var_t0)) * assign60430_e98157) + (assign60430_e98152 * ((var_t1_dn6 + var_t2_dn6) + var_t3_dn6))) + ((((((((((var_t0b_dn6 * var_t4) - (var_t0b * var_t4_dn6)) / (var_t4 * var_t4)) * var_delclm) + (assign60430_e98161 * var_delclm_dn6)) * var_t0c) + (assign60430_e98163 * var_t0c_dn6)) * var_t0d) - (assign60430_e98165 * var_t0d_dn6)) / (var_t0d * var_t0d))), ((((((var_t0a_dn7 * var_t0) - (var_t0a * var_t0_dn7)) / (var_t0 * var_t0)) * assign60430_e98157) + (assign60430_e98152 * ((var_t1_dn7 + var_t2_dn7) + var_t3_dn7))) + ((((((((((var_t0b_dn7 * var_t4) - (var_t0b * var_t4_dn7)) / (var_t4 * var_t4)) * var_delclm) + (assign60430_e98161 * var_delclm_dn7)) * var_t0c) + (assign60430_e98163 * var_t0c_dn7)) * var_t0d) - (assign60430_e98165 * var_t0d_dn7)) / (var_t0d * var_t0d))), ((((((var_t0a_dn8 * var_t0) - (var_t0a * var_t0_dn8)) / (var_t0 * var_t0)) * assign60430_e98157) + (assign60430_e98152 * ((var_t1_dn8 + var_t2_dn8) + var_t3_dn8))) + ((((((((((var_t0b_dn8 * var_t4) - (var_t0b * var_t4_dn8)) / (var_t4 * var_t4)) * var_delclm) + (assign60430_e98161 * var_delclm_dn8)) * var_t0c) + (assign60430_e98163 * var_t0c_dn8)) * var_t0d) - (assign60430_e98165 * var_t0d_dn8)) / (var_t0d * var_t0d))), ((((((var_t0a_dn9 * var_t0) - (var_t0a * var_t0_dn9)) / (var_t0 * var_t0)) * assign60430_e98157) + (assign60430_e98152 * ((var_t1_dn9 + var_t2_dn9) + var_t3_dn9))) + ((((((((((var_t0b_dn9 * var_t4) - (var_t0b * var_t4_dn9)) / (var_t4 * var_t4)) * var_delclm) + (assign60430_e98161 * var_delclm_dn9)) * var_t0c) + (assign60430_e98163 * var_t0c_dn9)) * var_t0d) - (assign60430_e98165 * var_t0d_dn9)) / (var_t0d * var_t0d))), ((((((var_t0a_dn10 * var_t0) - (var_t0a * var_t0_dn10)) / (var_t0 * var_t0)) * assign60430_e98157) + (assign60430_e98152 * ((var_t1_dn10 + var_t2_dn10) + var_t3_dn10))) + ((((((((((var_t0b_dn10 * var_t4) - (var_t0b * var_t4_dn10)) / (var_t4 * var_t4)) * var_delclm) + (assign60430_e98161 * var_delclm_dn10)) * var_t0c) + (assign60430_e98163 * var_t0c_dn10)) * var_t0d) - (assign60430_e98165 * var_t0d_dn10)) / (var_t0d * var_t0d))), ((((((var_t0a_dn11 * var_t0) - (var_t0a * var_t0_dn11)) / (var_t0 * var_t0)) * assign60430_e98157) + (assign60430_e98152 * ((var_t1_dn11 + var_t2_dn11) + var_t3_dn11))) + ((((((((((var_t0b_dn11 * var_t4) - (var_t0b * var_t4_dn11)) / (var_t4 * var_t4)) * var_delclm) + (assign60430_e98161 * var_delclm_dn11)) * var_t0c) + (assign60430_e98163 * var_t0c_dn11)) * var_t0d) - (assign60430_e98165 * var_t0d_dn11)) / (var_t0d * var_t0d))),)
    } else {
        (var_ssi, var_ssi_dn3, var_ssi_dn4, var_ssi_dn5, var_ssi_dn6, var_ssi_dn7, var_ssi_dn8, var_ssi_dn9, var_ssi_dn10, var_ssi_dn11,)
    }
};
        var_ssi = assign60430_e98170;
        var_ssi_dn3 = assign60430_e98170_d_n3;
        var_ssi_dn4 = assign60430_e98170_d_n4;
        var_ssi_dn5 = assign60430_e98170_d_n5;
        var_ssi_dn6 = assign60430_e98170_d_n6;
        var_ssi_dn7 = assign60430_e98170_d_n7;
        var_ssi_dn8 = assign60430_e98170_d_n8;
        var_ssi_dn9 = assign60430_e98170_d_n9;
        var_ssi_dn10 = assign60430_e98170_d_n10;
        var_ssi_dn11 = assign60430_e98170_d_n11;
        var_ssi_rv = 0.0;

        let (assign60440_e98187, assign60440_e98187_d_n3, assign60440_e98187_d_n4, assign60440_e98187_d_n5, assign60440_e98187_d_n6, assign60440_e98187_d_n7, assign60440_e98187_d_n8, assign60440_e98187_d_n9, assign60440_e98187_d_n10, assign60440_e98187_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign60440_e98177: f64 = (p.p1147 * p.p2);
        let assign60440_e98179: f64 = (assign60440_e98177 * var_leffnoi_edge);
        let assign60440_e98181: f64 = (assign60440_e98179 * 10000000000.0);
        let assign60440_e98183: f64 = (assign60440_e98181 * var_nstar);
        let assign60440_e98185: f64 = (assign60440_e98183 * var_nstar);
        (assign60440_e98185, (((assign60440_e98181 * var_nstar_dn3) * var_nstar) + (assign60440_e98183 * var_nstar_dn3)), (((assign60440_e98181 * var_nstar_dn4) * var_nstar) + (assign60440_e98183 * var_nstar_dn4)), (((assign60440_e98181 * var_nstar_dn5) * var_nstar) + (assign60440_e98183 * var_nstar_dn5)), (((assign60440_e98181 * var_nstar_dn6) * var_nstar) + (assign60440_e98183 * var_nstar_dn6)), (((assign60440_e98181 * var_nstar_dn7) * var_nstar) + (assign60440_e98183 * var_nstar_dn7)), (((assign60440_e98181 * var_nstar_dn8) * var_nstar) + (assign60440_e98183 * var_nstar_dn8)), (((assign60440_e98181 * var_nstar_dn9) * var_nstar) + (assign60440_e98183 * var_nstar_dn9)), (((assign60440_e98181 * var_nstar_dn10) * var_nstar) + (assign60440_e98183 * var_nstar_dn10)), (((assign60440_e98181 * var_nstar_dn11) * var_nstar) + (assign60440_e98183 * var_nstar_dn11)),)
    } else {
        (var_t5, var_t5_dn3, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8, var_t5_dn9, var_t5_dn10, var_t5_dn11,)
    }
};
        var_t5 = assign60440_e98187;
        var_t5_dn3 = assign60440_e98187_d_n3;
        var_t5_dn4 = assign60440_e98187_d_n4;
        var_t5_dn5 = assign60440_e98187_d_n5;
        var_t5_dn6 = assign60440_e98187_d_n6;
        var_t5_dn7 = assign60440_e98187_d_n7;
        var_t5_dn8 = assign60440_e98187_d_n8;
        var_t5_dn9 = assign60440_e98187_d_n9;
        var_t5_dn10 = assign60440_e98187_d_n10;
        var_t5_dn11 = assign60440_e98187_d_n11;
        var_t5_rv = 0.0;

        let (assign60450_e98200, assign60450_e98200_d_n3, assign60450_e98200_d_n4, assign60450_e98200_d_n5, assign60450_e98200_d_n6, assign60450_e98200_d_n7, assign60450_e98200_d_n8, assign60450_e98200_d_n9, assign60450_e98200_d_n10, assign60450_e98200_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign60450_e98194: f64 = (var_t0e / var_t5);
        let assign60450_e98196: f64 = (assign60450_e98194 * var_ids_edge);
        let assign60450_e98198: f64 = (assign60450_e98196 * var_ids_edge);
        (assign60450_e98198, (((((-((var_t0e * var_t5_dn3) / (var_t5 * var_t5))) * var_ids_edge) + (assign60450_e98194 * var_ids_edge_dn3)) * var_ids_edge) + (assign60450_e98196 * var_ids_edge_dn3)), (((((((var_t0e_dn4 * var_t5) - (var_t0e * var_t5_dn4)) / (var_t5 * var_t5)) * var_ids_edge) + (assign60450_e98194 * var_ids_edge_dn4)) * var_ids_edge) + (assign60450_e98196 * var_ids_edge_dn4)), (((((((var_t0e_dn5 * var_t5) - (var_t0e * var_t5_dn5)) / (var_t5 * var_t5)) * var_ids_edge) + (assign60450_e98194 * var_ids_edge_dn5)) * var_ids_edge) + (assign60450_e98196 * var_ids_edge_dn5)), (((((-((var_t0e * var_t5_dn6) / (var_t5 * var_t5))) * var_ids_edge) + (assign60450_e98194 * var_ids_edge_dn6)) * var_ids_edge) + (assign60450_e98196 * var_ids_edge_dn6)), (((((-((var_t0e * var_t5_dn7) / (var_t5 * var_t5))) * var_ids_edge) + (assign60450_e98194 * var_ids_edge_dn7)) * var_ids_edge) + (assign60450_e98196 * var_ids_edge_dn7)), (((((-((var_t0e * var_t5_dn8) / (var_t5 * var_t5))) * var_ids_edge) + (assign60450_e98194 * var_ids_edge_dn8)) * var_ids_edge) + (assign60450_e98196 * var_ids_edge_dn8)), (((((-((var_t0e * var_t5_dn9) / (var_t5 * var_t5))) * var_ids_edge) + (assign60450_e98194 * var_ids_edge_dn9)) * var_ids_edge) + (assign60450_e98196 * var_ids_edge_dn9)), (((((-((var_t0e * var_t5_dn10) / (var_t5 * var_t5))) * var_ids_edge) + (assign60450_e98194 * var_ids_edge_dn10)) * var_ids_edge) + (assign60450_e98196 * var_ids_edge_dn10)), (((((-((var_t0e * var_t5_dn11) / (var_t5 * var_t5))) * var_ids_edge) + (assign60450_e98194 * var_ids_edge_dn11)) * var_ids_edge) + (assign60450_e98196 * var_ids_edge_dn11)),)
    } else {
        (var_swi, var_swi_dn3, var_swi_dn4, var_swi_dn5, var_swi_dn6, var_swi_dn7, var_swi_dn8, var_swi_dn9, var_swi_dn10, var_swi_dn11,)
    }
};
        var_swi = assign60450_e98200;
        var_swi_dn3 = assign60450_e98200_d_n3;
        var_swi_dn4 = assign60450_e98200_d_n4;
        var_swi_dn5 = assign60450_e98200_d_n5;
        var_swi_dn6 = assign60450_e98200_d_n6;
        var_swi_dn7 = assign60450_e98200_d_n7;
        var_swi_dn8 = assign60450_e98200_d_n8;
        var_swi_dn9 = assign60450_e98200_d_n9;
        var_swi_dn10 = assign60450_e98200_d_n10;
        var_swi_dn11 = assign60450_e98200_d_n11;
        var_swi_rv = 0.0;

        *var_ids_slot = var_ids;
        *var_ids_dn10_slot = var_ids_dn10;
        *var_ids_dn11_slot = var_ids_dn11;
        *var_ids_dn3_slot = var_ids_dn3;
        *var_ids_dn4_slot = var_ids_dn4;
        *var_ids_dn5_slot = var_ids_dn5;
        *var_ids_dn6_slot = var_ids_dn6;
        *var_ids_dn7_slot = var_ids_dn7;
        *var_ids_dn8_slot = var_ids_dn8;
        *var_ids_dn9_slot = var_ids_dn9;
        *var_ids_edge_slot = var_ids_edge;
        *var_ids_edge_dn10_slot = var_ids_edge_dn10;
        *var_ids_edge_dn11_slot = var_ids_edge_dn11;
        *var_ids_edge_dn3_slot = var_ids_edge_dn3;
        *var_ids_edge_dn4_slot = var_ids_edge_dn4;
        *var_ids_edge_dn5_slot = var_ids_edge_dn5;
        *var_ids_edge_dn6_slot = var_ids_edge_dn6;
        *var_ids_edge_dn7_slot = var_ids_edge_dn7;
        *var_ids_edge_dn8_slot = var_ids_edge_dn8;
        *var_ids_edge_dn9_slot = var_ids_edge_dn9;
        *var_ids_edge_rv_slot = var_ids_edge_rv;
        *var_ids_rv_slot = var_ids_rv;
        *var_leffnoi_edge_slot = var_leffnoi_edge;
        *var_leffnoi_edge_rv_slot = var_leffnoi_edge_rv;
        *var_leffnoisq_edge_slot = var_leffnoisq_edge;
        *var_leffnoisq_edge_rv_slot = var_leffnoisq_edge_rv;
        *var_n0_slot = var_n0;
        *var_n0_dn10_slot = var_n0_dn10;
        *var_n0_dn11_slot = var_n0_dn11;
        *var_n0_dn3_slot = var_n0_dn3;
        *var_n0_dn4_slot = var_n0_dn4;
        *var_n0_dn5_slot = var_n0_dn5;
        *var_n0_dn6_slot = var_n0_dn6;
        *var_n0_dn7_slot = var_n0_dn7;
        *var_n0_dn8_slot = var_n0_dn8;
        *var_n0_dn9_slot = var_n0_dn9;
        *var_n0_rv_slot = var_n0_rv;
        *var_nl_slot = var_nl;
        *var_nl_dn10_slot = var_nl_dn10;
        *var_nl_dn11_slot = var_nl_dn11;
        *var_nl_dn3_slot = var_nl_dn3;
        *var_nl_dn4_slot = var_nl_dn4;
        *var_nl_dn5_slot = var_nl_dn5;
        *var_nl_dn6_slot = var_nl_dn6;
        *var_nl_dn7_slot = var_nl_dn7;
        *var_nl_dn8_slot = var_nl_dn8;
        *var_nl_dn9_slot = var_nl_dn9;
        *var_nl_rv_slot = var_nl_rv;
        *var_noia_edge_slot = var_noia_edge;
        *var_noia_edge_rv_slot = var_noia_edge_rv;
        *var_noib_edge_slot = var_noib_edge;
        *var_noib_edge_rv_slot = var_noib_edge_rv;
        *var_noic_edge_slot = var_noic_edge;
        *var_noic_edge_rv_slot = var_noic_edge_rv;
        *var_nq_edge_slot = var_nq_edge;
        *var_nq_edge_dn10_slot = var_nq_edge_dn10;
        *var_nq_edge_dn11_slot = var_nq_edge_dn11;
        *var_nq_edge_dn3_slot = var_nq_edge_dn3;
        *var_nq_edge_dn4_slot = var_nq_edge_dn4;
        *var_nq_edge_dn5_slot = var_nq_edge_dn5;
        *var_nq_edge_dn6_slot = var_nq_edge_dn6;
        *var_nq_edge_dn7_slot = var_nq_edge_dn7;
        *var_nq_edge_dn8_slot = var_nq_edge_dn8;
        *var_nq_edge_dn9_slot = var_nq_edge_dn9;
        *var_nq_edge_rv_slot = var_nq_edge_rv;
        *var_nstar_slot = var_nstar;
        *var_nstar_dn10_slot = var_nstar_dn10;
        *var_nstar_dn11_slot = var_nstar_dn11;
        *var_nstar_dn3_slot = var_nstar_dn3;
        *var_nstar_dn4_slot = var_nstar_dn4;
        *var_nstar_dn5_slot = var_nstar_dn5;
        *var_nstar_dn6_slot = var_nstar_dn6;
        *var_nstar_dn7_slot = var_nstar_dn7;
        *var_nstar_dn8_slot = var_nstar_dn8;
        *var_nstar_dn9_slot = var_nstar_dn9;
        *var_nstar_rv_slot = var_nstar_rv;
        *var_ssi_slot = var_ssi;
        *var_ssi_dn10_slot = var_ssi_dn10;
        *var_ssi_dn11_slot = var_ssi_dn11;
        *var_ssi_dn3_slot = var_ssi_dn3;
        *var_ssi_dn4_slot = var_ssi_dn4;
        *var_ssi_dn5_slot = var_ssi_dn5;
        *var_ssi_dn6_slot = var_ssi_dn6;
        *var_ssi_dn7_slot = var_ssi_dn7;
        *var_ssi_dn8_slot = var_ssi_dn8;
        *var_ssi_dn9_slot = var_ssi_dn9;
        *var_ssi_rv_slot = var_ssi_rv;
        *var_swi_slot = var_swi;
        *var_swi_dn10_slot = var_swi_dn10;
        *var_swi_dn11_slot = var_swi_dn11;
        *var_swi_dn3_slot = var_swi_dn3;
        *var_swi_dn4_slot = var_swi_dn4;
        *var_swi_dn5_slot = var_swi_dn5;
        *var_swi_dn6_slot = var_swi_dn6;
        *var_swi_dn7_slot = var_swi_dn7;
        *var_swi_dn8_slot = var_swi_dn8;
        *var_swi_dn9_slot = var_swi_dn9;
        *var_swi_rv_slot = var_swi_rv;
        *var_t0a_slot = var_t0a;
        *var_t0a_dn10_slot = var_t0a_dn10;
        *var_t0a_dn11_slot = var_t0a_dn11;
        *var_t0a_dn3_slot = var_t0a_dn3;
        *var_t0a_dn4_slot = var_t0a_dn4;
        *var_t0a_dn5_slot = var_t0a_dn5;
        *var_t0a_dn6_slot = var_t0a_dn6;
        *var_t0a_dn7_slot = var_t0a_dn7;
        *var_t0a_dn8_slot = var_t0a_dn8;
        *var_t0a_dn9_slot = var_t0a_dn9;
        *var_t0a_rv_slot = var_t0a_rv;
        *var_t0b_slot = var_t0b;
        *var_t0b_dn10_slot = var_t0b_dn10;
        *var_t0b_dn11_slot = var_t0b_dn11;
        *var_t0b_dn3_slot = var_t0b_dn3;
        *var_t0b_dn4_slot = var_t0b_dn4;
        *var_t0b_dn5_slot = var_t0b_dn5;
        *var_t0b_dn6_slot = var_t0b_dn6;
        *var_t0b_dn7_slot = var_t0b_dn7;
        *var_t0b_dn8_slot = var_t0b_dn8;
        *var_t0b_dn9_slot = var_t0b_dn9;
        *var_t0b_rv_slot = var_t0b_rv;
        *var_t0c_slot = var_t0c;
        *var_t0c_dn10_slot = var_t0c_dn10;
        *var_t0c_dn11_slot = var_t0c_dn11;
        *var_t0c_dn3_slot = var_t0c_dn3;
        *var_t0c_dn4_slot = var_t0c_dn4;
        *var_t0c_dn5_slot = var_t0c_dn5;
        *var_t0c_dn6_slot = var_t0c_dn6;
        *var_t0c_dn7_slot = var_t0c_dn7;
        *var_t0c_dn8_slot = var_t0c_dn8;
        *var_t0c_dn9_slot = var_t0c_dn9;
        *var_t0c_rv_slot = var_t0c_rv;
        *var_t0d_slot = var_t0d;
        *var_t0d_dn10_slot = var_t0d_dn10;
        *var_t0d_dn11_slot = var_t0d_dn11;
        *var_t0d_dn3_slot = var_t0d_dn3;
        *var_t0d_dn4_slot = var_t0d_dn4;
        *var_t0d_dn5_slot = var_t0d_dn5;
        *var_t0d_dn6_slot = var_t0d_dn6;
        *var_t0d_dn7_slot = var_t0d_dn7;
        *var_t0d_dn8_slot = var_t0d_dn8;
        *var_t0d_dn9_slot = var_t0d_dn9;
        *var_t0d_rv_slot = var_t0d_rv;
        *var_t0e_slot = var_t0e;
        *var_t0e_dn4_slot = var_t0e_dn4;
        *var_t0e_dn5_slot = var_t0e_dn5;
        *var_t0e_rv_slot = var_t0e_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t1_rv_slot = var_t1_rv;
        *var_t2_slot = var_t2;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_t2_rv_slot = var_t2_rv;
        *var_t3_slot = var_t3;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn3_slot = var_t3_dn3;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_dn9_slot = var_t3_dn9;
        *var_t3_rv_slot = var_t3_rv;
        *var_t4_slot = var_t4;
        *var_t4_dn10_slot = var_t4_dn10;
        *var_t4_dn11_slot = var_t4_dn11;
        *var_t4_dn3_slot = var_t4_dn3;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t4_dn9_slot = var_t4_dn9;
        *var_t4_rv_slot = var_t4_rv;
        *var_t5_slot = var_t5;
        *var_t5_dn10_slot = var_t5_dn10;
        *var_t5_dn11_slot = var_t5_dn11;
        *var_t5_dn3_slot = var_t5_dn3;
        *var_t5_dn4_slot = var_t5_dn4;
        *var_t5_dn5_slot = var_t5_dn5;
        *var_t5_dn6_slot = var_t5_dn6;
        *var_t5_dn7_slot = var_t5_dn7;
        *var_t5_dn8_slot = var_t5_dn8;
        *var_t5_dn9_slot = var_t5_dn9;
        *var_t5_rv_slot = var_t5_rv;
    }

    pub(super) fn stamp_reactive_block_212(
        p: &Parameters,
        var_devsign: f64,
        var_guard492: f64,
        var_guard867: f64,
        var_leff: f64,
        var_qbdj: f64,
        var_qbdj_dn10: f64,
        var_qbdj_dn11: f64,
        var_qbdj_dn3: f64,
        var_qbdj_dn4: f64,
        var_qbdj_dn5: f64,
        var_qbdj_dn6: f64,
        var_qbdj_dn7: f64,
        var_qbdj_dn8: f64,
        var_qbdj_dn9: f64,
        var_qbi: f64,
        var_qbi_dn10: f64,
        var_qbi_dn11: f64,
        var_qbi_dn3: f64,
        var_qbi_dn4: f64,
        var_qbi_dn5: f64,
        var_qbi_dn6: f64,
        var_qbi_dn7: f64,
        var_qbi_dn8: f64,
        var_qbi_dn9: f64,
        var_qbsj: f64,
        var_qbsj_dn10: f64,
        var_qbsj_dn11: f64,
        var_qbsj_dn3: f64,
        var_qbsj_dn4: f64,
        var_qbsj_dn5: f64,
        var_qbsj_dn6: f64,
        var_qbsj_dn7: f64,
        var_qbsj_dn8: f64,
        var_qbsj_dn9: f64,
        var_qdeff_edge: f64,
        var_qdeff_edge_dn10: f64,
        var_qdeff_edge_dn11: f64,
        var_qdeff_edge_dn3: f64,
        var_qdeff_edge_dn4: f64,
        var_qdeff_edge_dn5: f64,
        var_qdeff_edge_dn6: f64,
        var_qdeff_edge_dn7: f64,
        var_qdeff_edge_dn8: f64,
        var_qdeff_edge_dn9: f64,
        var_qdi: f64,
        var_qdi_dn10: f64,
        var_qdi_dn11: f64,
        var_qdi_dn3: f64,
        var_qdi_dn4: f64,
        var_qdi_dn5: f64,
        var_qdi_dn6: f64,
        var_qdi_dn7: f64,
        var_qdi_dn8: f64,
        var_qdi_dn9: f64,
        var_qdiagbcp2: f64,
        var_qdiagbcp2_dn10: f64,
        var_qdiagbcp2_dn11: f64,
        var_qdiagbcp2_dn3: f64,
        var_qdiagbcp2_dn4: f64,
        var_qdiagbcp2_dn5: f64,
        var_qdiagbcp2_dn6: f64,
        var_qdiagbcp2_dn7: f64,
        var_qdiagbcp2_dn8: f64,
        var_qdiagbcp2_dn9: f64,
        var_qdim: f64,
        var_qdim_dn10: f64,
        var_qdim_dn11: f64,
        var_qdim_dn3: f64,
        var_qdim_dn4: f64,
        var_qdim_dn5: f64,
        var_qdim_dn6: f64,
        var_qdim_dn7: f64,
        var_qdim_dn8: f64,
        var_qdim_dn9: f64,
        var_qgi: f64,
        var_qgi_dn10: f64,
        var_qgi_dn11: f64,
        var_qgi_dn3: f64,
        var_qgi_dn4: f64,
        var_qgi_dn5: f64,
        var_qgi_dn6: f64,
        var_qgi_dn7: f64,
        var_qgi_dn8: f64,
        var_qgi_dn9: f64,
        var_qgiagbcp2: f64,
        var_qgiagbcp2_dn10: f64,
        var_qgiagbcp2_dn11: f64,
        var_qgiagbcp2_dn3: f64,
        var_qgiagbcp2_dn4: f64,
        var_qgiagbcp2_dn5: f64,
        var_qgiagbcp2_dn6: f64,
        var_qgiagbcp2_dn7: f64,
        var_qgiagbcp2_dn8: f64,
        var_qgiagbcp2_dn9: f64,
        var_qgim: f64,
        var_qgim_dn10: f64,
        var_qgim_dn11: f64,
        var_qgim_dn3: f64,
        var_qgim_dn4: f64,
        var_qgim_dn5: f64,
        var_qgim_dn6: f64,
        var_qgim_dn7: f64,
        var_qgim_dn8: f64,
        var_qgim_dn9: f64,
        var_qovb: f64,
        var_qovb_dn10: f64,
        var_qovb_dn9: f64,
        var_qovd: f64,
        var_qovd_dn10: f64,
        var_qovd_dn11: f64,
        var_qovd_dn3: f64,
        var_qovd_dn4: f64,
        var_qovd_dn5: f64,
        var_qovd_dn6: f64,
        var_qovd_dn7: f64,
        var_qovd_dn8: f64,
        var_qovd_dn9: f64,
        var_qovg: f64,
        var_qovg_dn10: f64,
        var_qovg_dn11: f64,
        var_qovg_dn3: f64,
        var_qovg_dn4: f64,
        var_qovg_dn5: f64,
        var_qovg_dn6: f64,
        var_qovg_dn7: f64,
        var_qovg_dn8: f64,
        var_qovg_dn9: f64,
        var_qovs: f64,
        var_qovs_dn10: f64,
        var_qovs_dn11: f64,
        var_qovs_dn3: f64,
        var_qovs_dn4: f64,
        var_qovs_dn5: f64,
        var_qovs_dn6: f64,
        var_qovs_dn7: f64,
        var_qovs_dn8: f64,
        var_qovs_dn9: f64,
        var_qs_edge: f64,
        var_qs_edge_dn10: f64,
        var_qs_edge_dn11: f64,
        var_qs_edge_dn3: f64,
        var_qs_edge_dn4: f64,
        var_qs_edge_dn5: f64,
        var_qs_edge_dn6: f64,
        var_qs_edge_dn7: f64,
        var_qs_edge_dn8: f64,
        var_qs_edge_dn9: f64,
        var_qsi: f64,
        var_qsi_dn10: f64,
        var_qsi_dn11: f64,
        var_qsi_dn3: f64,
        var_qsi_dn4: f64,
        var_qsi_dn5: f64,
        var_qsi_dn6: f64,
        var_qsi_dn7: f64,
        var_qsi_dn8: f64,
        var_qsi_dn9: f64,
        var_qsiagbcp2: f64,
        var_qsiagbcp2_dn10: f64,
        var_qsiagbcp2_dn11: f64,
        var_qsiagbcp2_dn3: f64,
        var_qsiagbcp2_dn4: f64,
        var_qsiagbcp2_dn5: f64,
        var_qsiagbcp2_dn6: f64,
        var_qsiagbcp2_dn7: f64,
        var_qsiagbcp2_dn8: f64,
        var_qsiagbcp2_dn9: f64,
        var_qsim: f64,
        var_qsim_dn10: f64,
        var_qsim_dn11: f64,
        var_qsim_dn3: f64,
        var_qsim_dn4: f64,
        var_qsim_dn5: f64,
        var_qsim_dn6: f64,
        var_qsim_dn7: f64,
        var_qsim_dn8: f64,
        var_qsim_dn9: f64,
        var_sigvds: f64,
        var_ssi: f64,
        var_ssi_dn10: f64,
        var_ssi_dn11: f64,
        var_ssi_dn3: f64,
        var_ssi_dn4: f64,
        var_ssi_dn5: f64,
        var_ssi_dn6: f64,
        var_ssi_dn7: f64,
        var_ssi_dn8: f64,
        var_ssi_dn9: f64,
        var_swi: f64,
        var_swi_dn10: f64,
        var_swi_dn11: f64,
        var_swi_dn3: f64,
        var_swi_dn4: f64,
        var_swi_dn5: f64,
        var_swi_dn6: f64,
        var_swi_dn7: f64,
        var_swi_dn8: f64,
        var_swi_dn9: f64,
        var_weff: f64,
        var_guard877_slot: &mut f64,
        var_guard877_rv_slot: &mut f64,
        var_guard878_slot: &mut f64,
        var_guard878_rv_slot: &mut f64,
        var_guard893_slot: &mut f64,
        var_guard893_rv_slot: &mut f64,
        var_leff_1_slot: &mut f64,
        var_leff_1_rv_slot: &mut f64,
        var_qb_2_slot: &mut f64,
        var_qb_2_dn10_slot: &mut f64,
        var_qb_2_dn11_slot: &mut f64,
        var_qb_2_dn3_slot: &mut f64,
        var_qb_2_dn4_slot: &mut f64,
        var_qb_2_dn5_slot: &mut f64,
        var_qb_2_dn6_slot: &mut f64,
        var_qb_2_dn7_slot: &mut f64,
        var_qb_2_dn8_slot: &mut f64,
        var_qb_2_dn9_slot: &mut f64,
        var_qb_2_rv_slot: &mut f64,
        var_qd_1_slot: &mut f64,
        var_qd_1_dn10_slot: &mut f64,
        var_qd_1_dn11_slot: &mut f64,
        var_qd_1_dn3_slot: &mut f64,
        var_qd_1_dn4_slot: &mut f64,
        var_qd_1_dn5_slot: &mut f64,
        var_qd_1_dn6_slot: &mut f64,
        var_qd_1_dn7_slot: &mut f64,
        var_qd_1_dn8_slot: &mut f64,
        var_qd_1_dn9_slot: &mut f64,
        var_qd_1_rv_slot: &mut f64,
        var_qdi_1_slot: &mut f64,
        var_qdi_1_dn10_slot: &mut f64,
        var_qdi_1_dn11_slot: &mut f64,
        var_qdi_1_dn3_slot: &mut f64,
        var_qdi_1_dn4_slot: &mut f64,
        var_qdi_1_dn5_slot: &mut f64,
        var_qdi_1_dn6_slot: &mut f64,
        var_qdi_1_dn7_slot: &mut f64,
        var_qdi_1_dn8_slot: &mut f64,
        var_qdi_1_dn9_slot: &mut f64,
        var_qdi_1_rv_slot: &mut f64,
        var_qdiagbcp2_1_slot: &mut f64,
        var_qdiagbcp2_1_dn10_slot: &mut f64,
        var_qdiagbcp2_1_dn11_slot: &mut f64,
        var_qdiagbcp2_1_dn3_slot: &mut f64,
        var_qdiagbcp2_1_dn4_slot: &mut f64,
        var_qdiagbcp2_1_dn5_slot: &mut f64,
        var_qdiagbcp2_1_dn6_slot: &mut f64,
        var_qdiagbcp2_1_dn7_slot: &mut f64,
        var_qdiagbcp2_1_dn8_slot: &mut f64,
        var_qdiagbcp2_1_dn9_slot: &mut f64,
        var_qdiagbcp2_1_rv_slot: &mut f64,
        var_qdim_1_slot: &mut f64,
        var_qdim_1_dn10_slot: &mut f64,
        var_qdim_1_dn11_slot: &mut f64,
        var_qdim_1_dn3_slot: &mut f64,
        var_qdim_1_dn4_slot: &mut f64,
        var_qdim_1_dn5_slot: &mut f64,
        var_qdim_1_dn6_slot: &mut f64,
        var_qdim_1_dn7_slot: &mut f64,
        var_qdim_1_dn8_slot: &mut f64,
        var_qdim_1_dn9_slot: &mut f64,
        var_qdim_1_rv_slot: &mut f64,
        var_qg_slot: &mut f64,
        var_qg_dn10_slot: &mut f64,
        var_qg_dn11_slot: &mut f64,
        var_qg_dn3_slot: &mut f64,
        var_qg_dn4_slot: &mut f64,
        var_qg_dn5_slot: &mut f64,
        var_qg_dn6_slot: &mut f64,
        var_qg_dn7_slot: &mut f64,
        var_qg_dn8_slot: &mut f64,
        var_qg_dn9_slot: &mut f64,
        var_qg_rv_slot: &mut f64,
        var_qgiagbcp2_1_slot: &mut f64,
        var_qgiagbcp2_1_dn10_slot: &mut f64,
        var_qgiagbcp2_1_dn11_slot: &mut f64,
        var_qgiagbcp2_1_dn3_slot: &mut f64,
        var_qgiagbcp2_1_dn4_slot: &mut f64,
        var_qgiagbcp2_1_dn5_slot: &mut f64,
        var_qgiagbcp2_1_dn6_slot: &mut f64,
        var_qgiagbcp2_1_dn7_slot: &mut f64,
        var_qgiagbcp2_1_dn8_slot: &mut f64,
        var_qgiagbcp2_1_dn9_slot: &mut f64,
        var_qgiagbcp2_1_rv_slot: &mut f64,
        var_qgim_1_slot: &mut f64,
        var_qgim_1_dn10_slot: &mut f64,
        var_qgim_1_dn11_slot: &mut f64,
        var_qgim_1_dn3_slot: &mut f64,
        var_qgim_1_dn4_slot: &mut f64,
        var_qgim_1_dn5_slot: &mut f64,
        var_qgim_1_dn6_slot: &mut f64,
        var_qgim_1_dn7_slot: &mut f64,
        var_qgim_1_dn8_slot: &mut f64,
        var_qgim_1_dn9_slot: &mut f64,
        var_qgim_1_rv_slot: &mut f64,
        var_qs_2_slot: &mut f64,
        var_qs_2_dn10_slot: &mut f64,
        var_qs_2_dn11_slot: &mut f64,
        var_qs_2_dn3_slot: &mut f64,
        var_qs_2_dn4_slot: &mut f64,
        var_qs_2_dn5_slot: &mut f64,
        var_qs_2_dn6_slot: &mut f64,
        var_qs_2_dn7_slot: &mut f64,
        var_qs_2_dn8_slot: &mut f64,
        var_qs_2_dn9_slot: &mut f64,
        var_qs_2_rv_slot: &mut f64,
        var_qsi_1_slot: &mut f64,
        var_qsi_1_dn10_slot: &mut f64,
        var_qsi_1_dn11_slot: &mut f64,
        var_qsi_1_dn3_slot: &mut f64,
        var_qsi_1_dn4_slot: &mut f64,
        var_qsi_1_dn5_slot: &mut f64,
        var_qsi_1_dn6_slot: &mut f64,
        var_qsi_1_dn7_slot: &mut f64,
        var_qsi_1_dn8_slot: &mut f64,
        var_qsi_1_dn9_slot: &mut f64,
        var_qsi_1_rv_slot: &mut f64,
        var_qsiagbcp2_1_slot: &mut f64,
        var_qsiagbcp2_1_dn10_slot: &mut f64,
        var_qsiagbcp2_1_dn11_slot: &mut f64,
        var_qsiagbcp2_1_dn3_slot: &mut f64,
        var_qsiagbcp2_1_dn4_slot: &mut f64,
        var_qsiagbcp2_1_dn5_slot: &mut f64,
        var_qsiagbcp2_1_dn6_slot: &mut f64,
        var_qsiagbcp2_1_dn7_slot: &mut f64,
        var_qsiagbcp2_1_dn8_slot: &mut f64,
        var_qsiagbcp2_1_dn9_slot: &mut f64,
        var_qsiagbcp2_1_rv_slot: &mut f64,
        var_qsim_1_slot: &mut f64,
        var_qsim_1_dn10_slot: &mut f64,
        var_qsim_1_dn11_slot: &mut f64,
        var_qsim_1_dn3_slot: &mut f64,
        var_qsim_1_dn4_slot: &mut f64,
        var_qsim_1_dn5_slot: &mut f64,
        var_qsim_1_dn6_slot: &mut f64,
        var_qsim_1_dn7_slot: &mut f64,
        var_qsim_1_dn8_slot: &mut f64,
        var_qsim_1_dn9_slot: &mut f64,
        var_qsim_1_rv_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_dn10_slot: &mut f64,
        var_t6_dn11_slot: &mut f64,
        var_t6_dn3_slot: &mut f64,
        var_t6_dn4_slot: &mut f64,
        var_t6_dn5_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn7_slot: &mut f64,
        var_t6_dn8_slot: &mut f64,
        var_t6_dn9_slot: &mut f64,
        var_t6_rv_slot: &mut f64,
        var_t7_slot: &mut f64,
        var_t7_dn10_slot: &mut f64,
        var_t7_dn11_slot: &mut f64,
        var_t7_dn3_slot: &mut f64,
        var_t7_dn4_slot: &mut f64,
        var_t7_dn5_slot: &mut f64,
        var_t7_dn6_slot: &mut f64,
        var_t7_dn7_slot: &mut f64,
        var_t7_dn8_slot: &mut f64,
        var_t7_dn9_slot: &mut f64,
        var_t7_rv_slot: &mut f64,
        var_t8_slot: &mut f64,
        var_t8_dn10_slot: &mut f64,
        var_t8_dn11_slot: &mut f64,
        var_t8_dn3_slot: &mut f64,
        var_t8_dn4_slot: &mut f64,
        var_t8_dn5_slot: &mut f64,
        var_t8_dn6_slot: &mut f64,
        var_t8_dn7_slot: &mut f64,
        var_t8_dn8_slot: &mut f64,
        var_t8_dn9_slot: &mut f64,
        var_t8_rv_slot: &mut f64,
        var_weff_1_slot: &mut f64,
        var_weff_1_rv_slot: &mut f64,
    ) {
        let mut var_guard877: f64 = *var_guard877_slot;
        let mut var_guard877_rv: f64 = *var_guard877_rv_slot;
        let mut var_guard878: f64 = *var_guard878_slot;
        let mut var_guard878_rv: f64 = *var_guard878_rv_slot;
        let mut var_guard893: f64 = *var_guard893_slot;
        let mut var_guard893_rv: f64 = *var_guard893_rv_slot;
        let mut var_leff_1: f64 = *var_leff_1_slot;
        let mut var_leff_1_rv: f64 = *var_leff_1_rv_slot;
        let mut var_qb_2: f64 = *var_qb_2_slot;
        let mut var_qb_2_dn10: f64 = *var_qb_2_dn10_slot;
        let mut var_qb_2_dn11: f64 = *var_qb_2_dn11_slot;
        let mut var_qb_2_dn3: f64 = *var_qb_2_dn3_slot;
        let mut var_qb_2_dn4: f64 = *var_qb_2_dn4_slot;
        let mut var_qb_2_dn5: f64 = *var_qb_2_dn5_slot;
        let mut var_qb_2_dn6: f64 = *var_qb_2_dn6_slot;
        let mut var_qb_2_dn7: f64 = *var_qb_2_dn7_slot;
        let mut var_qb_2_dn8: f64 = *var_qb_2_dn8_slot;
        let mut var_qb_2_dn9: f64 = *var_qb_2_dn9_slot;
        let mut var_qb_2_rv: f64 = *var_qb_2_rv_slot;
        let mut var_qd_1: f64 = *var_qd_1_slot;
        let mut var_qd_1_dn10: f64 = *var_qd_1_dn10_slot;
        let mut var_qd_1_dn11: f64 = *var_qd_1_dn11_slot;
        let mut var_qd_1_dn3: f64 = *var_qd_1_dn3_slot;
        let mut var_qd_1_dn4: f64 = *var_qd_1_dn4_slot;
        let mut var_qd_1_dn5: f64 = *var_qd_1_dn5_slot;
        let mut var_qd_1_dn6: f64 = *var_qd_1_dn6_slot;
        let mut var_qd_1_dn7: f64 = *var_qd_1_dn7_slot;
        let mut var_qd_1_dn8: f64 = *var_qd_1_dn8_slot;
        let mut var_qd_1_dn9: f64 = *var_qd_1_dn9_slot;
        let mut var_qd_1_rv: f64 = *var_qd_1_rv_slot;
        let mut var_qdi_1: f64 = *var_qdi_1_slot;
        let mut var_qdi_1_dn10: f64 = *var_qdi_1_dn10_slot;
        let mut var_qdi_1_dn11: f64 = *var_qdi_1_dn11_slot;
        let mut var_qdi_1_dn3: f64 = *var_qdi_1_dn3_slot;
        let mut var_qdi_1_dn4: f64 = *var_qdi_1_dn4_slot;
        let mut var_qdi_1_dn5: f64 = *var_qdi_1_dn5_slot;
        let mut var_qdi_1_dn6: f64 = *var_qdi_1_dn6_slot;
        let mut var_qdi_1_dn7: f64 = *var_qdi_1_dn7_slot;
        let mut var_qdi_1_dn8: f64 = *var_qdi_1_dn8_slot;
        let mut var_qdi_1_dn9: f64 = *var_qdi_1_dn9_slot;
        let mut var_qdi_1_rv: f64 = *var_qdi_1_rv_slot;
        let mut var_qdiagbcp2_1: f64 = *var_qdiagbcp2_1_slot;
        let mut var_qdiagbcp2_1_dn10: f64 = *var_qdiagbcp2_1_dn10_slot;
        let mut var_qdiagbcp2_1_dn11: f64 = *var_qdiagbcp2_1_dn11_slot;
        let mut var_qdiagbcp2_1_dn3: f64 = *var_qdiagbcp2_1_dn3_slot;
        let mut var_qdiagbcp2_1_dn4: f64 = *var_qdiagbcp2_1_dn4_slot;
        let mut var_qdiagbcp2_1_dn5: f64 = *var_qdiagbcp2_1_dn5_slot;
        let mut var_qdiagbcp2_1_dn6: f64 = *var_qdiagbcp2_1_dn6_slot;
        let mut var_qdiagbcp2_1_dn7: f64 = *var_qdiagbcp2_1_dn7_slot;
        let mut var_qdiagbcp2_1_dn8: f64 = *var_qdiagbcp2_1_dn8_slot;
        let mut var_qdiagbcp2_1_dn9: f64 = *var_qdiagbcp2_1_dn9_slot;
        let mut var_qdiagbcp2_1_rv: f64 = *var_qdiagbcp2_1_rv_slot;
        let mut var_qdim_1: f64 = *var_qdim_1_slot;
        let mut var_qdim_1_dn10: f64 = *var_qdim_1_dn10_slot;
        let mut var_qdim_1_dn11: f64 = *var_qdim_1_dn11_slot;
        let mut var_qdim_1_dn3: f64 = *var_qdim_1_dn3_slot;
        let mut var_qdim_1_dn4: f64 = *var_qdim_1_dn4_slot;
        let mut var_qdim_1_dn5: f64 = *var_qdim_1_dn5_slot;
        let mut var_qdim_1_dn6: f64 = *var_qdim_1_dn6_slot;
        let mut var_qdim_1_dn7: f64 = *var_qdim_1_dn7_slot;
        let mut var_qdim_1_dn8: f64 = *var_qdim_1_dn8_slot;
        let mut var_qdim_1_dn9: f64 = *var_qdim_1_dn9_slot;
        let mut var_qdim_1_rv: f64 = *var_qdim_1_rv_slot;
        let mut var_qg: f64 = *var_qg_slot;
        let mut var_qg_dn10: f64 = *var_qg_dn10_slot;
        let mut var_qg_dn11: f64 = *var_qg_dn11_slot;
        let mut var_qg_dn3: f64 = *var_qg_dn3_slot;
        let mut var_qg_dn4: f64 = *var_qg_dn4_slot;
        let mut var_qg_dn5: f64 = *var_qg_dn5_slot;
        let mut var_qg_dn6: f64 = *var_qg_dn6_slot;
        let mut var_qg_dn7: f64 = *var_qg_dn7_slot;
        let mut var_qg_dn8: f64 = *var_qg_dn8_slot;
        let mut var_qg_dn9: f64 = *var_qg_dn9_slot;
        let mut var_qg_rv: f64 = *var_qg_rv_slot;
        let mut var_qgiagbcp2_1: f64 = *var_qgiagbcp2_1_slot;
        let mut var_qgiagbcp2_1_dn10: f64 = *var_qgiagbcp2_1_dn10_slot;
        let mut var_qgiagbcp2_1_dn11: f64 = *var_qgiagbcp2_1_dn11_slot;
        let mut var_qgiagbcp2_1_dn3: f64 = *var_qgiagbcp2_1_dn3_slot;
        let mut var_qgiagbcp2_1_dn4: f64 = *var_qgiagbcp2_1_dn4_slot;
        let mut var_qgiagbcp2_1_dn5: f64 = *var_qgiagbcp2_1_dn5_slot;
        let mut var_qgiagbcp2_1_dn6: f64 = *var_qgiagbcp2_1_dn6_slot;
        let mut var_qgiagbcp2_1_dn7: f64 = *var_qgiagbcp2_1_dn7_slot;
        let mut var_qgiagbcp2_1_dn8: f64 = *var_qgiagbcp2_1_dn8_slot;
        let mut var_qgiagbcp2_1_dn9: f64 = *var_qgiagbcp2_1_dn9_slot;
        let mut var_qgiagbcp2_1_rv: f64 = *var_qgiagbcp2_1_rv_slot;
        let mut var_qgim_1: f64 = *var_qgim_1_slot;
        let mut var_qgim_1_dn10: f64 = *var_qgim_1_dn10_slot;
        let mut var_qgim_1_dn11: f64 = *var_qgim_1_dn11_slot;
        let mut var_qgim_1_dn3: f64 = *var_qgim_1_dn3_slot;
        let mut var_qgim_1_dn4: f64 = *var_qgim_1_dn4_slot;
        let mut var_qgim_1_dn5: f64 = *var_qgim_1_dn5_slot;
        let mut var_qgim_1_dn6: f64 = *var_qgim_1_dn6_slot;
        let mut var_qgim_1_dn7: f64 = *var_qgim_1_dn7_slot;
        let mut var_qgim_1_dn8: f64 = *var_qgim_1_dn8_slot;
        let mut var_qgim_1_dn9: f64 = *var_qgim_1_dn9_slot;
        let mut var_qgim_1_rv: f64 = *var_qgim_1_rv_slot;
        let mut var_qs_2: f64 = *var_qs_2_slot;
        let mut var_qs_2_dn10: f64 = *var_qs_2_dn10_slot;
        let mut var_qs_2_dn11: f64 = *var_qs_2_dn11_slot;
        let mut var_qs_2_dn3: f64 = *var_qs_2_dn3_slot;
        let mut var_qs_2_dn4: f64 = *var_qs_2_dn4_slot;
        let mut var_qs_2_dn5: f64 = *var_qs_2_dn5_slot;
        let mut var_qs_2_dn6: f64 = *var_qs_2_dn6_slot;
        let mut var_qs_2_dn7: f64 = *var_qs_2_dn7_slot;
        let mut var_qs_2_dn8: f64 = *var_qs_2_dn8_slot;
        let mut var_qs_2_dn9: f64 = *var_qs_2_dn9_slot;
        let mut var_qs_2_rv: f64 = *var_qs_2_rv_slot;
        let mut var_qsi_1: f64 = *var_qsi_1_slot;
        let mut var_qsi_1_dn10: f64 = *var_qsi_1_dn10_slot;
        let mut var_qsi_1_dn11: f64 = *var_qsi_1_dn11_slot;
        let mut var_qsi_1_dn3: f64 = *var_qsi_1_dn3_slot;
        let mut var_qsi_1_dn4: f64 = *var_qsi_1_dn4_slot;
        let mut var_qsi_1_dn5: f64 = *var_qsi_1_dn5_slot;
        let mut var_qsi_1_dn6: f64 = *var_qsi_1_dn6_slot;
        let mut var_qsi_1_dn7: f64 = *var_qsi_1_dn7_slot;
        let mut var_qsi_1_dn8: f64 = *var_qsi_1_dn8_slot;
        let mut var_qsi_1_dn9: f64 = *var_qsi_1_dn9_slot;
        let mut var_qsi_1_rv: f64 = *var_qsi_1_rv_slot;
        let mut var_qsiagbcp2_1: f64 = *var_qsiagbcp2_1_slot;
        let mut var_qsiagbcp2_1_dn10: f64 = *var_qsiagbcp2_1_dn10_slot;
        let mut var_qsiagbcp2_1_dn11: f64 = *var_qsiagbcp2_1_dn11_slot;
        let mut var_qsiagbcp2_1_dn3: f64 = *var_qsiagbcp2_1_dn3_slot;
        let mut var_qsiagbcp2_1_dn4: f64 = *var_qsiagbcp2_1_dn4_slot;
        let mut var_qsiagbcp2_1_dn5: f64 = *var_qsiagbcp2_1_dn5_slot;
        let mut var_qsiagbcp2_1_dn6: f64 = *var_qsiagbcp2_1_dn6_slot;
        let mut var_qsiagbcp2_1_dn7: f64 = *var_qsiagbcp2_1_dn7_slot;
        let mut var_qsiagbcp2_1_dn8: f64 = *var_qsiagbcp2_1_dn8_slot;
        let mut var_qsiagbcp2_1_dn9: f64 = *var_qsiagbcp2_1_dn9_slot;
        let mut var_qsiagbcp2_1_rv: f64 = *var_qsiagbcp2_1_rv_slot;
        let mut var_qsim_1: f64 = *var_qsim_1_slot;
        let mut var_qsim_1_dn10: f64 = *var_qsim_1_dn10_slot;
        let mut var_qsim_1_dn11: f64 = *var_qsim_1_dn11_slot;
        let mut var_qsim_1_dn3: f64 = *var_qsim_1_dn3_slot;
        let mut var_qsim_1_dn4: f64 = *var_qsim_1_dn4_slot;
        let mut var_qsim_1_dn5: f64 = *var_qsim_1_dn5_slot;
        let mut var_qsim_1_dn6: f64 = *var_qsim_1_dn6_slot;
        let mut var_qsim_1_dn7: f64 = *var_qsim_1_dn7_slot;
        let mut var_qsim_1_dn8: f64 = *var_qsim_1_dn8_slot;
        let mut var_qsim_1_dn9: f64 = *var_qsim_1_dn9_slot;
        let mut var_qsim_1_rv: f64 = *var_qsim_1_rv_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_dn10: f64 = *var_t6_dn10_slot;
        let mut var_t6_dn11: f64 = *var_t6_dn11_slot;
        let mut var_t6_dn3: f64 = *var_t6_dn3_slot;
        let mut var_t6_dn4: f64 = *var_t6_dn4_slot;
        let mut var_t6_dn5: f64 = *var_t6_dn5_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn7: f64 = *var_t6_dn7_slot;
        let mut var_t6_dn8: f64 = *var_t6_dn8_slot;
        let mut var_t6_dn9: f64 = *var_t6_dn9_slot;
        let mut var_t6_rv: f64 = *var_t6_rv_slot;
        let mut var_t7: f64 = *var_t7_slot;
        let mut var_t7_dn10: f64 = *var_t7_dn10_slot;
        let mut var_t7_dn11: f64 = *var_t7_dn11_slot;
        let mut var_t7_dn3: f64 = *var_t7_dn3_slot;
        let mut var_t7_dn4: f64 = *var_t7_dn4_slot;
        let mut var_t7_dn5: f64 = *var_t7_dn5_slot;
        let mut var_t7_dn6: f64 = *var_t7_dn6_slot;
        let mut var_t7_dn7: f64 = *var_t7_dn7_slot;
        let mut var_t7_dn8: f64 = *var_t7_dn8_slot;
        let mut var_t7_dn9: f64 = *var_t7_dn9_slot;
        let mut var_t7_rv: f64 = *var_t7_rv_slot;
        let mut var_t8: f64 = *var_t8_slot;
        let mut var_t8_dn10: f64 = *var_t8_dn10_slot;
        let mut var_t8_dn11: f64 = *var_t8_dn11_slot;
        let mut var_t8_dn3: f64 = *var_t8_dn3_slot;
        let mut var_t8_dn4: f64 = *var_t8_dn4_slot;
        let mut var_t8_dn5: f64 = *var_t8_dn5_slot;
        let mut var_t8_dn6: f64 = *var_t8_dn6_slot;
        let mut var_t8_dn7: f64 = *var_t8_dn7_slot;
        let mut var_t8_dn8: f64 = *var_t8_dn8_slot;
        let mut var_t8_dn9: f64 = *var_t8_dn9_slot;
        let mut var_t8_rv: f64 = *var_t8_rv_slot;
        let mut var_weff_1: f64 = *var_weff_1_slot;
        let mut var_weff_1_rv: f64 = *var_weff_1_rv_slot;

        let (assign60460_e98209, assign60460_e98209_d_n3, assign60460_e98209_d_n4, assign60460_e98209_d_n5, assign60460_e98209_d_n6, assign60460_e98209_d_n7, assign60460_e98209_d_n8, assign60460_e98209_d_n9, assign60460_e98209_d_n10, assign60460_e98209_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard867 != 0.0)) {
        let assign60460_e98207: f64 = (var_swi + var_ssi);
        (assign60460_e98207, (var_swi_dn3 + var_ssi_dn3), (var_swi_dn4 + var_ssi_dn4), (var_swi_dn5 + var_ssi_dn5), (var_swi_dn6 + var_ssi_dn6), (var_swi_dn7 + var_ssi_dn7), (var_swi_dn8 + var_ssi_dn8), (var_swi_dn9 + var_ssi_dn9), (var_swi_dn10 + var_ssi_dn10), (var_swi_dn11 + var_ssi_dn11),)
    } else {
        (var_t6, var_t6_dn3, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_dn9, var_t6_dn10, var_t6_dn11,)
    }
};
        var_t6 = assign60460_e98209;
        var_t6_dn3 = assign60460_e98209_d_n3;
        var_t6_dn4 = assign60460_e98209_d_n4;
        var_t6_dn5 = assign60460_e98209_d_n5;
        var_t6_dn6 = assign60460_e98209_d_n6;
        var_t6_dn7 = assign60460_e98209_d_n7;
        var_t6_dn8 = assign60460_e98209_d_n8;
        var_t6_dn9 = assign60460_e98209_d_n9;
        var_t6_dn10 = assign60460_e98209_d_n10;
        var_t6_dn11 = assign60460_e98209_d_n11;
        var_t6_rv = 0.0;

        let assign60470_e98212: f64 = if var_t6 > 0.0 { 1.0 } else { 0.0 };
        var_guard877 = assign60470_e98212;
        var_guard877_rv = 0.0;

        let (assign60480_e98225, assign60480_e98225_d_n3, assign60480_e98225_d_n4, assign60480_e98225_d_n5, assign60480_e98225_d_n6, assign60480_e98225_d_n7, assign60480_e98225_d_n8, assign60480_e98225_d_n9, assign60480_e98225_d_n10, assign60480_e98225_d_n11,) = {
    if (((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard877 != 0.0)) {
        let assign60480_e98221: f64 = (var_ssi * var_swi);
        let assign60480_e98223: f64 = (assign60480_e98221 / var_t6);
        (assign60480_e98223, (((((var_ssi_dn3 * var_swi) + (var_ssi * var_swi_dn3)) * var_t6) - (assign60480_e98221 * var_t6_dn3)) / (var_t6 * var_t6)), (((((var_ssi_dn4 * var_swi) + (var_ssi * var_swi_dn4)) * var_t6) - (assign60480_e98221 * var_t6_dn4)) / (var_t6 * var_t6)), (((((var_ssi_dn5 * var_swi) + (var_ssi * var_swi_dn5)) * var_t6) - (assign60480_e98221 * var_t6_dn5)) / (var_t6 * var_t6)), (((((var_ssi_dn6 * var_swi) + (var_ssi * var_swi_dn6)) * var_t6) - (assign60480_e98221 * var_t6_dn6)) / (var_t6 * var_t6)), (((((var_ssi_dn7 * var_swi) + (var_ssi * var_swi_dn7)) * var_t6) - (assign60480_e98221 * var_t6_dn7)) / (var_t6 * var_t6)), (((((var_ssi_dn8 * var_swi) + (var_ssi * var_swi_dn8)) * var_t6) - (assign60480_e98221 * var_t6_dn8)) / (var_t6 * var_t6)), (((((var_ssi_dn9 * var_swi) + (var_ssi * var_swi_dn9)) * var_t6) - (assign60480_e98221 * var_t6_dn9)) / (var_t6 * var_t6)), (((((var_ssi_dn10 * var_swi) + (var_ssi * var_swi_dn10)) * var_t6) - (assign60480_e98221 * var_t6_dn10)) / (var_t6 * var_t6)), (((((var_ssi_dn11 * var_swi) + (var_ssi * var_swi_dn11)) * var_t6) - (assign60480_e98221 * var_t6_dn11)) / (var_t6 * var_t6)),)
    } else {
        (var_t7, var_t7_dn3, var_t7_dn4, var_t7_dn5, var_t7_dn6, var_t7_dn7, var_t7_dn8, var_t7_dn9, var_t7_dn10, var_t7_dn11,)
    }
};
        var_t7 = assign60480_e98225;
        var_t7_dn3 = assign60480_e98225_d_n3;
        var_t7_dn4 = assign60480_e98225_d_n4;
        var_t7_dn5 = assign60480_e98225_d_n5;
        var_t7_dn6 = assign60480_e98225_d_n6;
        var_t7_dn7 = assign60480_e98225_d_n7;
        var_t7_dn8 = assign60480_e98225_d_n8;
        var_t7_dn9 = assign60480_e98225_d_n9;
        var_t7_dn10 = assign60480_e98225_d_n10;
        var_t7_dn11 = assign60480_e98225_d_n11;
        var_t7_rv = 0.0;

        let (assign60490_e98242, assign60490_e98242_d_n3, assign60490_e98242_d_n4, assign60490_e98242_d_n5, assign60490_e98242_d_n6, assign60490_e98242_d_n7, assign60490_e98242_d_n8, assign60490_e98242_d_n9, assign60490_e98242_d_n10, assign60490_e98242_d_n11,) = {
    if (((var_guard492 == 0.0) && (var_guard867 != 0.0)) && (var_guard877 != 0.0)) {
        let assign60490_e98236: f64 = (var_qs_edge - var_qdeff_edge);
        let assign60490_e98238: f64 = (assign60490_e98236).powf(p.p1318);
        let assign60490_e98239: f64 = (p.p1317 * assign60490_e98238);
        let assign60490_e98240: f64 = (1.0 + assign60490_e98239);
        (assign60490_e98240, (p.p1317 * if 0.0 == 0.0 && ((p.p1318) as f64).is_finite() && ((p.p1318) as f64).fract() == 0.0 { if p.p1318 == 0.0 { 0.0 } else { (p.p1318 * ((assign60490_e98236).powf(p.p1318 - 1.0) * (var_qs_edge_dn3 - var_qdeff_edge_dn3))) } } else { (assign60490_e98238 * (p.p1318 * ((var_qs_edge_dn3 - var_qdeff_edge_dn3) / assign60490_e98236))) }), (p.p1317 * if 0.0 == 0.0 && ((p.p1318) as f64).is_finite() && ((p.p1318) as f64).fract() == 0.0 { if p.p1318 == 0.0 { 0.0 } else { (p.p1318 * ((assign60490_e98236).powf(p.p1318 - 1.0) * (var_qs_edge_dn4 - var_qdeff_edge_dn4))) } } else { (assign60490_e98238 * (p.p1318 * ((var_qs_edge_dn4 - var_qdeff_edge_dn4) / assign60490_e98236))) }), (p.p1317 * if 0.0 == 0.0 && ((p.p1318) as f64).is_finite() && ((p.p1318) as f64).fract() == 0.0 { if p.p1318 == 0.0 { 0.0 } else { (p.p1318 * ((assign60490_e98236).powf(p.p1318 - 1.0) * (var_qs_edge_dn5 - var_qdeff_edge_dn5))) } } else { (assign60490_e98238 * (p.p1318 * ((var_qs_edge_dn5 - var_qdeff_edge_dn5) / assign60490_e98236))) }), (p.p1317 * if 0.0 == 0.0 && ((p.p1318) as f64).is_finite() && ((p.p1318) as f64).fract() == 0.0 { if p.p1318 == 0.0 { 0.0 } else { (p.p1318 * ((assign60490_e98236).powf(p.p1318 - 1.0) * (var_qs_edge_dn6 - var_qdeff_edge_dn6))) } } else { (assign60490_e98238 * (p.p1318 * ((var_qs_edge_dn6 - var_qdeff_edge_dn6) / assign60490_e98236))) }), (p.p1317 * if 0.0 == 0.0 && ((p.p1318) as f64).is_finite() && ((p.p1318) as f64).fract() == 0.0 { if p.p1318 == 0.0 { 0.0 } else { (p.p1318 * ((assign60490_e98236).powf(p.p1318 - 1.0) * (var_qs_edge_dn7 - var_qdeff_edge_dn7))) } } else { (assign60490_e98238 * (p.p1318 * ((var_qs_edge_dn7 - var_qdeff_edge_dn7) / assign60490_e98236))) }), (p.p1317 * if 0.0 == 0.0 && ((p.p1318) as f64).is_finite() && ((p.p1318) as f64).fract() == 0.0 { if p.p1318 == 0.0 { 0.0 } else { (p.p1318 * ((assign60490_e98236).powf(p.p1318 - 1.0) * (var_qs_edge_dn8 - var_qdeff_edge_dn8))) } } else { (assign60490_e98238 * (p.p1318 * ((var_qs_edge_dn8 - var_qdeff_edge_dn8) / assign60490_e98236))) }), (p.p1317 * if 0.0 == 0.0 && ((p.p1318) as f64).is_finite() && ((p.p1318) as f64).fract() == 0.0 { if p.p1318 == 0.0 { 0.0 } else { (p.p1318 * ((assign60490_e98236).powf(p.p1318 - 1.0) * (var_qs_edge_dn9 - var_qdeff_edge_dn9))) } } else { (assign60490_e98238 * (p.p1318 * ((var_qs_edge_dn9 - var_qdeff_edge_dn9) / assign60490_e98236))) }), (p.p1317 * if 0.0 == 0.0 && ((p.p1318) as f64).is_finite() && ((p.p1318) as f64).fract() == 0.0 { if p.p1318 == 0.0 { 0.0 } else { (p.p1318 * ((assign60490_e98236).powf(p.p1318 - 1.0) * (var_qs_edge_dn10 - var_qdeff_edge_dn10))) } } else { (assign60490_e98238 * (p.p1318 * ((var_qs_edge_dn10 - var_qdeff_edge_dn10) / assign60490_e98236))) }), (p.p1317 * if 0.0 == 0.0 && ((p.p1318) as f64).is_finite() && ((p.p1318) as f64).fract() == 0.0 { if p.p1318 == 0.0 { 0.0 } else { (p.p1318 * ((assign60490_e98236).powf(p.p1318 - 1.0) * (var_qs_edge_dn11 - var_qdeff_edge_dn11))) } } else { (assign60490_e98238 * (p.p1318 * ((var_qs_edge_dn11 - var_qdeff_edge_dn11) / assign60490_e98236))) }),)
    } else {
        (var_t8, var_t8_dn3, var_t8_dn4, var_t8_dn5, var_t8_dn6, var_t8_dn7, var_t8_dn8, var_t8_dn9, var_t8_dn10, var_t8_dn11,)
    }
};
        var_t8 = assign60490_e98242;
        var_t8_dn3 = assign60490_e98242_d_n3;
        var_t8_dn4 = assign60490_e98242_d_n4;
        var_t8_dn5 = assign60490_e98242_d_n5;
        var_t8_dn6 = assign60490_e98242_d_n6;
        var_t8_dn7 = assign60490_e98242_d_n7;
        var_t8_dn8 = assign60490_e98242_d_n8;
        var_t8_dn9 = assign60490_e98242_d_n9;
        var_t8_dn10 = assign60490_e98242_d_n10;
        var_t8_dn11 = assign60490_e98242_d_n11;
        var_t8_rv = 0.0;

        let (assign60520_e98276, assign60520_e98276_d_n3, assign60520_e98276_d_n4, assign60520_e98276_d_n5, assign60520_e98276_d_n6, assign60520_e98276_d_n7, assign60520_e98276_d_n8, assign60520_e98276_d_n9, assign60520_e98276_d_n10, assign60520_e98276_d_n11,) = {
    if (var_guard492 == 0.0) {
        let assign60520_e98269: f64 = (var_qbi + var_qovb);
        let assign60520_e98271: f64 = (assign60520_e98269 + var_qbsj);
        let assign60520_e98273: f64 = (assign60520_e98271 + var_qbdj);
        let assign60520_e98274: f64 = (var_devsign * assign60520_e98273);
        (assign60520_e98274, (var_devsign * ((var_qbi_dn3 + var_qbsj_dn3) + var_qbdj_dn3)), (var_devsign * ((var_qbi_dn4 + var_qbsj_dn4) + var_qbdj_dn4)), (var_devsign * ((var_qbi_dn5 + var_qbsj_dn5) + var_qbdj_dn5)), (var_devsign * ((var_qbi_dn6 + var_qbsj_dn6) + var_qbdj_dn6)), (var_devsign * ((var_qbi_dn7 + var_qbsj_dn7) + var_qbdj_dn7)), (var_devsign * ((var_qbi_dn8 + var_qbsj_dn8) + var_qbdj_dn8)), (var_devsign * (((var_qbi_dn9 + var_qovb_dn9) + var_qbsj_dn9) + var_qbdj_dn9)), (var_devsign * (((var_qbi_dn10 + var_qovb_dn10) + var_qbsj_dn10) + var_qbdj_dn10)), (var_devsign * ((var_qbi_dn11 + var_qbsj_dn11) + var_qbdj_dn11)),)
    } else {
        (var_qb_2, var_qb_2_dn3, var_qb_2_dn4, var_qb_2_dn5, var_qb_2_dn6, var_qb_2_dn7, var_qb_2_dn8, var_qb_2_dn9, var_qb_2_dn10, var_qb_2_dn11,)
    }
};
        var_qb_2 = assign60520_e98276;
        var_qb_2_dn3 = assign60520_e98276_d_n3;
        var_qb_2_dn4 = assign60520_e98276_d_n4;
        var_qb_2_dn5 = assign60520_e98276_d_n5;
        var_qb_2_dn6 = assign60520_e98276_d_n6;
        var_qb_2_dn7 = assign60520_e98276_d_n7;
        var_qb_2_dn8 = assign60520_e98276_d_n8;
        var_qb_2_dn9 = assign60520_e98276_d_n9;
        var_qb_2_dn10 = assign60520_e98276_d_n10;
        var_qb_2_dn11 = assign60520_e98276_d_n11;
        var_qb_2_rv = 0.0;

        let assign60530_e98279: f64 = if var_sigvds > 0.0 { 1.0 } else { 0.0 };
        var_guard878 = assign60530_e98279;
        var_guard878_rv = 0.0;

        let (assign60540_e98288, assign60540_e98288_d_n3, assign60540_e98288_d_n4, assign60540_e98288_d_n5, assign60540_e98288_d_n6, assign60540_e98288_d_n7, assign60540_e98288_d_n8, assign60540_e98288_d_n9, assign60540_e98288_d_n10, assign60540_e98288_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard878 != 0.0)) {
        let assign60540_e98286: f64 = (var_devsign * var_qsi);
        (assign60540_e98286, (var_devsign * var_qsi_dn3), (var_devsign * var_qsi_dn4), (var_devsign * var_qsi_dn5), (var_devsign * var_qsi_dn6), (var_devsign * var_qsi_dn7), (var_devsign * var_qsi_dn8), (var_devsign * var_qsi_dn9), (var_devsign * var_qsi_dn10), (var_devsign * var_qsi_dn11),)
    } else {
        (var_qsi_1, var_qsi_1_dn3, var_qsi_1_dn4, var_qsi_1_dn5, var_qsi_1_dn6, var_qsi_1_dn7, var_qsi_1_dn8, var_qsi_1_dn9, var_qsi_1_dn10, var_qsi_1_dn11,)
    }
};
        var_qsi_1 = assign60540_e98288;
        var_qsi_1_dn3 = assign60540_e98288_d_n3;
        var_qsi_1_dn4 = assign60540_e98288_d_n4;
        var_qsi_1_dn5 = assign60540_e98288_d_n5;
        var_qsi_1_dn6 = assign60540_e98288_d_n6;
        var_qsi_1_dn7 = assign60540_e98288_d_n7;
        var_qsi_1_dn8 = assign60540_e98288_d_n8;
        var_qsi_1_dn9 = assign60540_e98288_d_n9;
        var_qsi_1_dn10 = assign60540_e98288_d_n10;
        var_qsi_1_dn11 = assign60540_e98288_d_n11;
        var_qsi_1_rv = 0.0;

        let (assign60550_e98297, assign60550_e98297_d_n3, assign60550_e98297_d_n4, assign60550_e98297_d_n5, assign60550_e98297_d_n6, assign60550_e98297_d_n7, assign60550_e98297_d_n8, assign60550_e98297_d_n9, assign60550_e98297_d_n10, assign60550_e98297_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard878 != 0.0)) {
        let assign60550_e98295: f64 = (var_devsign * var_qsim);
        (assign60550_e98295, (var_devsign * var_qsim_dn3), (var_devsign * var_qsim_dn4), (var_devsign * var_qsim_dn5), (var_devsign * var_qsim_dn6), (var_devsign * var_qsim_dn7), (var_devsign * var_qsim_dn8), (var_devsign * var_qsim_dn9), (var_devsign * var_qsim_dn10), (var_devsign * var_qsim_dn11),)
    } else {
        (var_qsim_1, var_qsim_1_dn3, var_qsim_1_dn4, var_qsim_1_dn5, var_qsim_1_dn6, var_qsim_1_dn7, var_qsim_1_dn8, var_qsim_1_dn9, var_qsim_1_dn10, var_qsim_1_dn11,)
    }
};
        var_qsim_1 = assign60550_e98297;
        var_qsim_1_dn3 = assign60550_e98297_d_n3;
        var_qsim_1_dn4 = assign60550_e98297_d_n4;
        var_qsim_1_dn5 = assign60550_e98297_d_n5;
        var_qsim_1_dn6 = assign60550_e98297_d_n6;
        var_qsim_1_dn7 = assign60550_e98297_d_n7;
        var_qsim_1_dn8 = assign60550_e98297_d_n8;
        var_qsim_1_dn9 = assign60550_e98297_d_n9;
        var_qsim_1_dn10 = assign60550_e98297_d_n10;
        var_qsim_1_dn11 = assign60550_e98297_d_n11;
        var_qsim_1_rv = 0.0;

        let (assign60560_e98306, assign60560_e98306_d_n3, assign60560_e98306_d_n4, assign60560_e98306_d_n5, assign60560_e98306_d_n6, assign60560_e98306_d_n7, assign60560_e98306_d_n8, assign60560_e98306_d_n9, assign60560_e98306_d_n10, assign60560_e98306_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard878 != 0.0)) {
        let assign60560_e98304: f64 = (var_devsign * var_qsiagbcp2);
        (assign60560_e98304, (var_devsign * var_qsiagbcp2_dn3), (var_devsign * var_qsiagbcp2_dn4), (var_devsign * var_qsiagbcp2_dn5), (var_devsign * var_qsiagbcp2_dn6), (var_devsign * var_qsiagbcp2_dn7), (var_devsign * var_qsiagbcp2_dn8), (var_devsign * var_qsiagbcp2_dn9), (var_devsign * var_qsiagbcp2_dn10), (var_devsign * var_qsiagbcp2_dn11),)
    } else {
        (var_qsiagbcp2_1, var_qsiagbcp2_1_dn3, var_qsiagbcp2_1_dn4, var_qsiagbcp2_1_dn5, var_qsiagbcp2_1_dn6, var_qsiagbcp2_1_dn7, var_qsiagbcp2_1_dn8, var_qsiagbcp2_1_dn9, var_qsiagbcp2_1_dn10, var_qsiagbcp2_1_dn11,)
    }
};
        var_qsiagbcp2_1 = assign60560_e98306;
        var_qsiagbcp2_1_dn3 = assign60560_e98306_d_n3;
        var_qsiagbcp2_1_dn4 = assign60560_e98306_d_n4;
        var_qsiagbcp2_1_dn5 = assign60560_e98306_d_n5;
        var_qsiagbcp2_1_dn6 = assign60560_e98306_d_n6;
        var_qsiagbcp2_1_dn7 = assign60560_e98306_d_n7;
        var_qsiagbcp2_1_dn8 = assign60560_e98306_d_n8;
        var_qsiagbcp2_1_dn9 = assign60560_e98306_d_n9;
        var_qsiagbcp2_1_dn10 = assign60560_e98306_d_n10;
        var_qsiagbcp2_1_dn11 = assign60560_e98306_d_n11;
        var_qsiagbcp2_1_rv = 0.0;

        let (assign60570_e98315, assign60570_e98315_d_n3, assign60570_e98315_d_n4, assign60570_e98315_d_n5, assign60570_e98315_d_n6, assign60570_e98315_d_n7, assign60570_e98315_d_n8, assign60570_e98315_d_n9, assign60570_e98315_d_n10, assign60570_e98315_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard878 != 0.0)) {
        let assign60570_e98313: f64 = (var_devsign * var_qdi);
        (assign60570_e98313, (var_devsign * var_qdi_dn3), (var_devsign * var_qdi_dn4), (var_devsign * var_qdi_dn5), (var_devsign * var_qdi_dn6), (var_devsign * var_qdi_dn7), (var_devsign * var_qdi_dn8), (var_devsign * var_qdi_dn9), (var_devsign * var_qdi_dn10), (var_devsign * var_qdi_dn11),)
    } else {
        (var_qdi_1, var_qdi_1_dn3, var_qdi_1_dn4, var_qdi_1_dn5, var_qdi_1_dn6, var_qdi_1_dn7, var_qdi_1_dn8, var_qdi_1_dn9, var_qdi_1_dn10, var_qdi_1_dn11,)
    }
};
        var_qdi_1 = assign60570_e98315;
        var_qdi_1_dn3 = assign60570_e98315_d_n3;
        var_qdi_1_dn4 = assign60570_e98315_d_n4;
        var_qdi_1_dn5 = assign60570_e98315_d_n5;
        var_qdi_1_dn6 = assign60570_e98315_d_n6;
        var_qdi_1_dn7 = assign60570_e98315_d_n7;
        var_qdi_1_dn8 = assign60570_e98315_d_n8;
        var_qdi_1_dn9 = assign60570_e98315_d_n9;
        var_qdi_1_dn10 = assign60570_e98315_d_n10;
        var_qdi_1_dn11 = assign60570_e98315_d_n11;
        var_qdi_1_rv = 0.0;

        let (assign60580_e98324, assign60580_e98324_d_n3, assign60580_e98324_d_n4, assign60580_e98324_d_n5, assign60580_e98324_d_n6, assign60580_e98324_d_n7, assign60580_e98324_d_n8, assign60580_e98324_d_n9, assign60580_e98324_d_n10, assign60580_e98324_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard878 != 0.0)) {
        let assign60580_e98322: f64 = (var_devsign * var_qdim);
        (assign60580_e98322, (var_devsign * var_qdim_dn3), (var_devsign * var_qdim_dn4), (var_devsign * var_qdim_dn5), (var_devsign * var_qdim_dn6), (var_devsign * var_qdim_dn7), (var_devsign * var_qdim_dn8), (var_devsign * var_qdim_dn9), (var_devsign * var_qdim_dn10), (var_devsign * var_qdim_dn11),)
    } else {
        (var_qdim_1, var_qdim_1_dn3, var_qdim_1_dn4, var_qdim_1_dn5, var_qdim_1_dn6, var_qdim_1_dn7, var_qdim_1_dn8, var_qdim_1_dn9, var_qdim_1_dn10, var_qdim_1_dn11,)
    }
};
        var_qdim_1 = assign60580_e98324;
        var_qdim_1_dn3 = assign60580_e98324_d_n3;
        var_qdim_1_dn4 = assign60580_e98324_d_n4;
        var_qdim_1_dn5 = assign60580_e98324_d_n5;
        var_qdim_1_dn6 = assign60580_e98324_d_n6;
        var_qdim_1_dn7 = assign60580_e98324_d_n7;
        var_qdim_1_dn8 = assign60580_e98324_d_n8;
        var_qdim_1_dn9 = assign60580_e98324_d_n9;
        var_qdim_1_dn10 = assign60580_e98324_d_n10;
        var_qdim_1_dn11 = assign60580_e98324_d_n11;
        var_qdim_1_rv = 0.0;

        let (assign60590_e98333, assign60590_e98333_d_n3, assign60590_e98333_d_n4, assign60590_e98333_d_n5, assign60590_e98333_d_n6, assign60590_e98333_d_n7, assign60590_e98333_d_n8, assign60590_e98333_d_n9, assign60590_e98333_d_n10, assign60590_e98333_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard878 != 0.0)) {
        let assign60590_e98331: f64 = (var_devsign * var_qdiagbcp2);
        (assign60590_e98331, (var_devsign * var_qdiagbcp2_dn3), (var_devsign * var_qdiagbcp2_dn4), (var_devsign * var_qdiagbcp2_dn5), (var_devsign * var_qdiagbcp2_dn6), (var_devsign * var_qdiagbcp2_dn7), (var_devsign * var_qdiagbcp2_dn8), (var_devsign * var_qdiagbcp2_dn9), (var_devsign * var_qdiagbcp2_dn10), (var_devsign * var_qdiagbcp2_dn11),)
    } else {
        (var_qdiagbcp2_1, var_qdiagbcp2_1_dn3, var_qdiagbcp2_1_dn4, var_qdiagbcp2_1_dn5, var_qdiagbcp2_1_dn6, var_qdiagbcp2_1_dn7, var_qdiagbcp2_1_dn8, var_qdiagbcp2_1_dn9, var_qdiagbcp2_1_dn10, var_qdiagbcp2_1_dn11,)
    }
};
        var_qdiagbcp2_1 = assign60590_e98333;
        var_qdiagbcp2_1_dn3 = assign60590_e98333_d_n3;
        var_qdiagbcp2_1_dn4 = assign60590_e98333_d_n4;
        var_qdiagbcp2_1_dn5 = assign60590_e98333_d_n5;
        var_qdiagbcp2_1_dn6 = assign60590_e98333_d_n6;
        var_qdiagbcp2_1_dn7 = assign60590_e98333_d_n7;
        var_qdiagbcp2_1_dn8 = assign60590_e98333_d_n8;
        var_qdiagbcp2_1_dn9 = assign60590_e98333_d_n9;
        var_qdiagbcp2_1_dn10 = assign60590_e98333_d_n10;
        var_qdiagbcp2_1_dn11 = assign60590_e98333_d_n11;
        var_qdiagbcp2_1_rv = 0.0;

        let (assign60600_e98346, assign60600_e98346_d_n3, assign60600_e98346_d_n4, assign60600_e98346_d_n5, assign60600_e98346_d_n6, assign60600_e98346_d_n7, assign60600_e98346_d_n8, assign60600_e98346_d_n9, assign60600_e98346_d_n10, assign60600_e98346_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard878 != 0.0)) {
        let assign60600_e98341: f64 = (var_qsi + var_qovs);
        let assign60600_e98343: f64 = (assign60600_e98341 - var_qbsj);
        let assign60600_e98344: f64 = (var_devsign * assign60600_e98343);
        (assign60600_e98344, (var_devsign * ((var_qsi_dn3 + var_qovs_dn3) - var_qbsj_dn3)), (var_devsign * ((var_qsi_dn4 + var_qovs_dn4) - var_qbsj_dn4)), (var_devsign * ((var_qsi_dn5 + var_qovs_dn5) - var_qbsj_dn5)), (var_devsign * ((var_qsi_dn6 + var_qovs_dn6) - var_qbsj_dn6)), (var_devsign * ((var_qsi_dn7 + var_qovs_dn7) - var_qbsj_dn7)), (var_devsign * ((var_qsi_dn8 + var_qovs_dn8) - var_qbsj_dn8)), (var_devsign * ((var_qsi_dn9 + var_qovs_dn9) - var_qbsj_dn9)), (var_devsign * ((var_qsi_dn10 + var_qovs_dn10) - var_qbsj_dn10)), (var_devsign * ((var_qsi_dn11 + var_qovs_dn11) - var_qbsj_dn11)),)
    } else {
        (var_qs_2, var_qs_2_dn3, var_qs_2_dn4, var_qs_2_dn5, var_qs_2_dn6, var_qs_2_dn7, var_qs_2_dn8, var_qs_2_dn9, var_qs_2_dn10, var_qs_2_dn11,)
    }
};
        var_qs_2 = assign60600_e98346;
        var_qs_2_dn3 = assign60600_e98346_d_n3;
        var_qs_2_dn4 = assign60600_e98346_d_n4;
        var_qs_2_dn5 = assign60600_e98346_d_n5;
        var_qs_2_dn6 = assign60600_e98346_d_n6;
        var_qs_2_dn7 = assign60600_e98346_d_n7;
        var_qs_2_dn8 = assign60600_e98346_d_n8;
        var_qs_2_dn9 = assign60600_e98346_d_n9;
        var_qs_2_dn10 = assign60600_e98346_d_n10;
        var_qs_2_dn11 = assign60600_e98346_d_n11;
        var_qs_2_rv = 0.0;

        let (assign60610_e98359, assign60610_e98359_d_n3, assign60610_e98359_d_n4, assign60610_e98359_d_n5, assign60610_e98359_d_n6, assign60610_e98359_d_n7, assign60610_e98359_d_n8, assign60610_e98359_d_n9, assign60610_e98359_d_n10, assign60610_e98359_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard878 != 0.0)) {
        let assign60610_e98354: f64 = (var_qdi + var_qovd);
        let assign60610_e98356: f64 = (assign60610_e98354 - var_qbdj);
        let assign60610_e98357: f64 = (var_devsign * assign60610_e98356);
        (assign60610_e98357, (var_devsign * ((var_qdi_dn3 + var_qovd_dn3) - var_qbdj_dn3)), (var_devsign * ((var_qdi_dn4 + var_qovd_dn4) - var_qbdj_dn4)), (var_devsign * ((var_qdi_dn5 + var_qovd_dn5) - var_qbdj_dn5)), (var_devsign * ((var_qdi_dn6 + var_qovd_dn6) - var_qbdj_dn6)), (var_devsign * ((var_qdi_dn7 + var_qovd_dn7) - var_qbdj_dn7)), (var_devsign * ((var_qdi_dn8 + var_qovd_dn8) - var_qbdj_dn8)), (var_devsign * ((var_qdi_dn9 + var_qovd_dn9) - var_qbdj_dn9)), (var_devsign * ((var_qdi_dn10 + var_qovd_dn10) - var_qbdj_dn10)), (var_devsign * ((var_qdi_dn11 + var_qovd_dn11) - var_qbdj_dn11)),)
    } else {
        (var_qd_1, var_qd_1_dn3, var_qd_1_dn4, var_qd_1_dn5, var_qd_1_dn6, var_qd_1_dn7, var_qd_1_dn8, var_qd_1_dn9, var_qd_1_dn10, var_qd_1_dn11,)
    }
};
        var_qd_1 = assign60610_e98359;
        var_qd_1_dn3 = assign60610_e98359_d_n3;
        var_qd_1_dn4 = assign60610_e98359_d_n4;
        var_qd_1_dn5 = assign60610_e98359_d_n5;
        var_qd_1_dn6 = assign60610_e98359_d_n6;
        var_qd_1_dn7 = assign60610_e98359_d_n7;
        var_qd_1_dn8 = assign60610_e98359_d_n8;
        var_qd_1_dn9 = assign60610_e98359_d_n9;
        var_qd_1_dn10 = assign60610_e98359_d_n10;
        var_qd_1_dn11 = assign60610_e98359_d_n11;
        var_qd_1_rv = 0.0;

        let (assign60620_e98369, assign60620_e98369_d_n3, assign60620_e98369_d_n4, assign60620_e98369_d_n5, assign60620_e98369_d_n6, assign60620_e98369_d_n7, assign60620_e98369_d_n8, assign60620_e98369_d_n9, assign60620_e98369_d_n10, assign60620_e98369_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard878 == 0.0)) {
        let assign60620_e98367: f64 = (var_devsign * var_qdi);
        (assign60620_e98367, (var_devsign * var_qdi_dn3), (var_devsign * var_qdi_dn4), (var_devsign * var_qdi_dn5), (var_devsign * var_qdi_dn6), (var_devsign * var_qdi_dn7), (var_devsign * var_qdi_dn8), (var_devsign * var_qdi_dn9), (var_devsign * var_qdi_dn10), (var_devsign * var_qdi_dn11),)
    } else {
        (var_qsi_1, var_qsi_1_dn3, var_qsi_1_dn4, var_qsi_1_dn5, var_qsi_1_dn6, var_qsi_1_dn7, var_qsi_1_dn8, var_qsi_1_dn9, var_qsi_1_dn10, var_qsi_1_dn11,)
    }
};
        var_qsi_1 = assign60620_e98369;
        var_qsi_1_dn3 = assign60620_e98369_d_n3;
        var_qsi_1_dn4 = assign60620_e98369_d_n4;
        var_qsi_1_dn5 = assign60620_e98369_d_n5;
        var_qsi_1_dn6 = assign60620_e98369_d_n6;
        var_qsi_1_dn7 = assign60620_e98369_d_n7;
        var_qsi_1_dn8 = assign60620_e98369_d_n8;
        var_qsi_1_dn9 = assign60620_e98369_d_n9;
        var_qsi_1_dn10 = assign60620_e98369_d_n10;
        var_qsi_1_dn11 = assign60620_e98369_d_n11;
        var_qsi_1_rv = 0.0;

        let (assign60630_e98379, assign60630_e98379_d_n3, assign60630_e98379_d_n4, assign60630_e98379_d_n5, assign60630_e98379_d_n6, assign60630_e98379_d_n7, assign60630_e98379_d_n8, assign60630_e98379_d_n9, assign60630_e98379_d_n10, assign60630_e98379_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard878 == 0.0)) {
        let assign60630_e98377: f64 = (var_devsign * var_qdim);
        (assign60630_e98377, (var_devsign * var_qdim_dn3), (var_devsign * var_qdim_dn4), (var_devsign * var_qdim_dn5), (var_devsign * var_qdim_dn6), (var_devsign * var_qdim_dn7), (var_devsign * var_qdim_dn8), (var_devsign * var_qdim_dn9), (var_devsign * var_qdim_dn10), (var_devsign * var_qdim_dn11),)
    } else {
        (var_qsim_1, var_qsim_1_dn3, var_qsim_1_dn4, var_qsim_1_dn5, var_qsim_1_dn6, var_qsim_1_dn7, var_qsim_1_dn8, var_qsim_1_dn9, var_qsim_1_dn10, var_qsim_1_dn11,)
    }
};
        var_qsim_1 = assign60630_e98379;
        var_qsim_1_dn3 = assign60630_e98379_d_n3;
        var_qsim_1_dn4 = assign60630_e98379_d_n4;
        var_qsim_1_dn5 = assign60630_e98379_d_n5;
        var_qsim_1_dn6 = assign60630_e98379_d_n6;
        var_qsim_1_dn7 = assign60630_e98379_d_n7;
        var_qsim_1_dn8 = assign60630_e98379_d_n8;
        var_qsim_1_dn9 = assign60630_e98379_d_n9;
        var_qsim_1_dn10 = assign60630_e98379_d_n10;
        var_qsim_1_dn11 = assign60630_e98379_d_n11;
        var_qsim_1_rv = 0.0;

        let (assign60640_e98389, assign60640_e98389_d_n3, assign60640_e98389_d_n4, assign60640_e98389_d_n5, assign60640_e98389_d_n6, assign60640_e98389_d_n7, assign60640_e98389_d_n8, assign60640_e98389_d_n9, assign60640_e98389_d_n10, assign60640_e98389_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard878 == 0.0)) {
        let assign60640_e98387: f64 = (var_devsign * var_qdiagbcp2);
        (assign60640_e98387, (var_devsign * var_qdiagbcp2_dn3), (var_devsign * var_qdiagbcp2_dn4), (var_devsign * var_qdiagbcp2_dn5), (var_devsign * var_qdiagbcp2_dn6), (var_devsign * var_qdiagbcp2_dn7), (var_devsign * var_qdiagbcp2_dn8), (var_devsign * var_qdiagbcp2_dn9), (var_devsign * var_qdiagbcp2_dn10), (var_devsign * var_qdiagbcp2_dn11),)
    } else {
        (var_qsiagbcp2_1, var_qsiagbcp2_1_dn3, var_qsiagbcp2_1_dn4, var_qsiagbcp2_1_dn5, var_qsiagbcp2_1_dn6, var_qsiagbcp2_1_dn7, var_qsiagbcp2_1_dn8, var_qsiagbcp2_1_dn9, var_qsiagbcp2_1_dn10, var_qsiagbcp2_1_dn11,)
    }
};
        var_qsiagbcp2_1 = assign60640_e98389;
        var_qsiagbcp2_1_dn3 = assign60640_e98389_d_n3;
        var_qsiagbcp2_1_dn4 = assign60640_e98389_d_n4;
        var_qsiagbcp2_1_dn5 = assign60640_e98389_d_n5;
        var_qsiagbcp2_1_dn6 = assign60640_e98389_d_n6;
        var_qsiagbcp2_1_dn7 = assign60640_e98389_d_n7;
        var_qsiagbcp2_1_dn8 = assign60640_e98389_d_n8;
        var_qsiagbcp2_1_dn9 = assign60640_e98389_d_n9;
        var_qsiagbcp2_1_dn10 = assign60640_e98389_d_n10;
        var_qsiagbcp2_1_dn11 = assign60640_e98389_d_n11;
        var_qsiagbcp2_1_rv = 0.0;

        let (assign60650_e98399, assign60650_e98399_d_n3, assign60650_e98399_d_n4, assign60650_e98399_d_n5, assign60650_e98399_d_n6, assign60650_e98399_d_n7, assign60650_e98399_d_n8, assign60650_e98399_d_n9, assign60650_e98399_d_n10, assign60650_e98399_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard878 == 0.0)) {
        let assign60650_e98397: f64 = (var_devsign * var_qsi);
        (assign60650_e98397, (var_devsign * var_qsi_dn3), (var_devsign * var_qsi_dn4), (var_devsign * var_qsi_dn5), (var_devsign * var_qsi_dn6), (var_devsign * var_qsi_dn7), (var_devsign * var_qsi_dn8), (var_devsign * var_qsi_dn9), (var_devsign * var_qsi_dn10), (var_devsign * var_qsi_dn11),)
    } else {
        (var_qdi_1, var_qdi_1_dn3, var_qdi_1_dn4, var_qdi_1_dn5, var_qdi_1_dn6, var_qdi_1_dn7, var_qdi_1_dn8, var_qdi_1_dn9, var_qdi_1_dn10, var_qdi_1_dn11,)
    }
};
        var_qdi_1 = assign60650_e98399;
        var_qdi_1_dn3 = assign60650_e98399_d_n3;
        var_qdi_1_dn4 = assign60650_e98399_d_n4;
        var_qdi_1_dn5 = assign60650_e98399_d_n5;
        var_qdi_1_dn6 = assign60650_e98399_d_n6;
        var_qdi_1_dn7 = assign60650_e98399_d_n7;
        var_qdi_1_dn8 = assign60650_e98399_d_n8;
        var_qdi_1_dn9 = assign60650_e98399_d_n9;
        var_qdi_1_dn10 = assign60650_e98399_d_n10;
        var_qdi_1_dn11 = assign60650_e98399_d_n11;
        var_qdi_1_rv = 0.0;

        let (assign60660_e98409, assign60660_e98409_d_n3, assign60660_e98409_d_n4, assign60660_e98409_d_n5, assign60660_e98409_d_n6, assign60660_e98409_d_n7, assign60660_e98409_d_n8, assign60660_e98409_d_n9, assign60660_e98409_d_n10, assign60660_e98409_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard878 == 0.0)) {
        let assign60660_e98407: f64 = (var_devsign * var_qsim);
        (assign60660_e98407, (var_devsign * var_qsim_dn3), (var_devsign * var_qsim_dn4), (var_devsign * var_qsim_dn5), (var_devsign * var_qsim_dn6), (var_devsign * var_qsim_dn7), (var_devsign * var_qsim_dn8), (var_devsign * var_qsim_dn9), (var_devsign * var_qsim_dn10), (var_devsign * var_qsim_dn11),)
    } else {
        (var_qdim_1, var_qdim_1_dn3, var_qdim_1_dn4, var_qdim_1_dn5, var_qdim_1_dn6, var_qdim_1_dn7, var_qdim_1_dn8, var_qdim_1_dn9, var_qdim_1_dn10, var_qdim_1_dn11,)
    }
};
        var_qdim_1 = assign60660_e98409;
        var_qdim_1_dn3 = assign60660_e98409_d_n3;
        var_qdim_1_dn4 = assign60660_e98409_d_n4;
        var_qdim_1_dn5 = assign60660_e98409_d_n5;
        var_qdim_1_dn6 = assign60660_e98409_d_n6;
        var_qdim_1_dn7 = assign60660_e98409_d_n7;
        var_qdim_1_dn8 = assign60660_e98409_d_n8;
        var_qdim_1_dn9 = assign60660_e98409_d_n9;
        var_qdim_1_dn10 = assign60660_e98409_d_n10;
        var_qdim_1_dn11 = assign60660_e98409_d_n11;
        var_qdim_1_rv = 0.0;

        let (assign60670_e98419, assign60670_e98419_d_n3, assign60670_e98419_d_n4, assign60670_e98419_d_n5, assign60670_e98419_d_n6, assign60670_e98419_d_n7, assign60670_e98419_d_n8, assign60670_e98419_d_n9, assign60670_e98419_d_n10, assign60670_e98419_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard878 == 0.0)) {
        let assign60670_e98417: f64 = (var_devsign * var_qsiagbcp2);
        (assign60670_e98417, (var_devsign * var_qsiagbcp2_dn3), (var_devsign * var_qsiagbcp2_dn4), (var_devsign * var_qsiagbcp2_dn5), (var_devsign * var_qsiagbcp2_dn6), (var_devsign * var_qsiagbcp2_dn7), (var_devsign * var_qsiagbcp2_dn8), (var_devsign * var_qsiagbcp2_dn9), (var_devsign * var_qsiagbcp2_dn10), (var_devsign * var_qsiagbcp2_dn11),)
    } else {
        (var_qdiagbcp2_1, var_qdiagbcp2_1_dn3, var_qdiagbcp2_1_dn4, var_qdiagbcp2_1_dn5, var_qdiagbcp2_1_dn6, var_qdiagbcp2_1_dn7, var_qdiagbcp2_1_dn8, var_qdiagbcp2_1_dn9, var_qdiagbcp2_1_dn10, var_qdiagbcp2_1_dn11,)
    }
};
        var_qdiagbcp2_1 = assign60670_e98419;
        var_qdiagbcp2_1_dn3 = assign60670_e98419_d_n3;
        var_qdiagbcp2_1_dn4 = assign60670_e98419_d_n4;
        var_qdiagbcp2_1_dn5 = assign60670_e98419_d_n5;
        var_qdiagbcp2_1_dn6 = assign60670_e98419_d_n6;
        var_qdiagbcp2_1_dn7 = assign60670_e98419_d_n7;
        var_qdiagbcp2_1_dn8 = assign60670_e98419_d_n8;
        var_qdiagbcp2_1_dn9 = assign60670_e98419_d_n9;
        var_qdiagbcp2_1_dn10 = assign60670_e98419_d_n10;
        var_qdiagbcp2_1_dn11 = assign60670_e98419_d_n11;
        var_qdiagbcp2_1_rv = 0.0;

        let (assign60680_e98433, assign60680_e98433_d_n3, assign60680_e98433_d_n4, assign60680_e98433_d_n5, assign60680_e98433_d_n6, assign60680_e98433_d_n7, assign60680_e98433_d_n8, assign60680_e98433_d_n9, assign60680_e98433_d_n10, assign60680_e98433_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard878 == 0.0)) {
        let assign60680_e98428: f64 = (var_qdi + var_qovs);
        let assign60680_e98430: f64 = (assign60680_e98428 - var_qbsj);
        let assign60680_e98431: f64 = (var_devsign * assign60680_e98430);
        (assign60680_e98431, (var_devsign * ((var_qdi_dn3 + var_qovs_dn3) - var_qbsj_dn3)), (var_devsign * ((var_qdi_dn4 + var_qovs_dn4) - var_qbsj_dn4)), (var_devsign * ((var_qdi_dn5 + var_qovs_dn5) - var_qbsj_dn5)), (var_devsign * ((var_qdi_dn6 + var_qovs_dn6) - var_qbsj_dn6)), (var_devsign * ((var_qdi_dn7 + var_qovs_dn7) - var_qbsj_dn7)), (var_devsign * ((var_qdi_dn8 + var_qovs_dn8) - var_qbsj_dn8)), (var_devsign * ((var_qdi_dn9 + var_qovs_dn9) - var_qbsj_dn9)), (var_devsign * ((var_qdi_dn10 + var_qovs_dn10) - var_qbsj_dn10)), (var_devsign * ((var_qdi_dn11 + var_qovs_dn11) - var_qbsj_dn11)),)
    } else {
        (var_qs_2, var_qs_2_dn3, var_qs_2_dn4, var_qs_2_dn5, var_qs_2_dn6, var_qs_2_dn7, var_qs_2_dn8, var_qs_2_dn9, var_qs_2_dn10, var_qs_2_dn11,)
    }
};
        var_qs_2 = assign60680_e98433;
        var_qs_2_dn3 = assign60680_e98433_d_n3;
        var_qs_2_dn4 = assign60680_e98433_d_n4;
        var_qs_2_dn5 = assign60680_e98433_d_n5;
        var_qs_2_dn6 = assign60680_e98433_d_n6;
        var_qs_2_dn7 = assign60680_e98433_d_n7;
        var_qs_2_dn8 = assign60680_e98433_d_n8;
        var_qs_2_dn9 = assign60680_e98433_d_n9;
        var_qs_2_dn10 = assign60680_e98433_d_n10;
        var_qs_2_dn11 = assign60680_e98433_d_n11;
        var_qs_2_rv = 0.0;

        let (assign60690_e98447, assign60690_e98447_d_n3, assign60690_e98447_d_n4, assign60690_e98447_d_n5, assign60690_e98447_d_n6, assign60690_e98447_d_n7, assign60690_e98447_d_n8, assign60690_e98447_d_n9, assign60690_e98447_d_n10, assign60690_e98447_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard878 == 0.0)) {
        let assign60690_e98442: f64 = (var_qsi + var_qovd);
        let assign60690_e98444: f64 = (assign60690_e98442 - var_qbdj);
        let assign60690_e98445: f64 = (var_devsign * assign60690_e98444);
        (assign60690_e98445, (var_devsign * ((var_qsi_dn3 + var_qovd_dn3) - var_qbdj_dn3)), (var_devsign * ((var_qsi_dn4 + var_qovd_dn4) - var_qbdj_dn4)), (var_devsign * ((var_qsi_dn5 + var_qovd_dn5) - var_qbdj_dn5)), (var_devsign * ((var_qsi_dn6 + var_qovd_dn6) - var_qbdj_dn6)), (var_devsign * ((var_qsi_dn7 + var_qovd_dn7) - var_qbdj_dn7)), (var_devsign * ((var_qsi_dn8 + var_qovd_dn8) - var_qbdj_dn8)), (var_devsign * ((var_qsi_dn9 + var_qovd_dn9) - var_qbdj_dn9)), (var_devsign * ((var_qsi_dn10 + var_qovd_dn10) - var_qbdj_dn10)), (var_devsign * ((var_qsi_dn11 + var_qovd_dn11) - var_qbdj_dn11)),)
    } else {
        (var_qd_1, var_qd_1_dn3, var_qd_1_dn4, var_qd_1_dn5, var_qd_1_dn6, var_qd_1_dn7, var_qd_1_dn8, var_qd_1_dn9, var_qd_1_dn10, var_qd_1_dn11,)
    }
};
        var_qd_1 = assign60690_e98447;
        var_qd_1_dn3 = assign60690_e98447_d_n3;
        var_qd_1_dn4 = assign60690_e98447_d_n4;
        var_qd_1_dn5 = assign60690_e98447_d_n5;
        var_qd_1_dn6 = assign60690_e98447_d_n6;
        var_qd_1_dn7 = assign60690_e98447_d_n7;
        var_qd_1_dn8 = assign60690_e98447_d_n8;
        var_qd_1_dn9 = assign60690_e98447_d_n9;
        var_qd_1_dn10 = assign60690_e98447_d_n10;
        var_qd_1_dn11 = assign60690_e98447_d_n11;
        var_qd_1_rv = 0.0;

        let (assign60700_e98456, assign60700_e98456_d_n3, assign60700_e98456_d_n4, assign60700_e98456_d_n5, assign60700_e98456_d_n6, assign60700_e98456_d_n7, assign60700_e98456_d_n8, assign60700_e98456_d_n9, assign60700_e98456_d_n10, assign60700_e98456_d_n11,) = {
    if (var_guard492 == 0.0) {
        let assign60700_e98453: f64 = (var_qgi + var_qovg);
        let assign60700_e98454: f64 = (var_devsign * assign60700_e98453);
        (assign60700_e98454, (var_devsign * (var_qgi_dn3 + var_qovg_dn3)), (var_devsign * (var_qgi_dn4 + var_qovg_dn4)), (var_devsign * (var_qgi_dn5 + var_qovg_dn5)), (var_devsign * (var_qgi_dn6 + var_qovg_dn6)), (var_devsign * (var_qgi_dn7 + var_qovg_dn7)), (var_devsign * (var_qgi_dn8 + var_qovg_dn8)), (var_devsign * (var_qgi_dn9 + var_qovg_dn9)), (var_devsign * (var_qgi_dn10 + var_qovg_dn10)), (var_devsign * (var_qgi_dn11 + var_qovg_dn11)),)
    } else {
        (var_qg, var_qg_dn3, var_qg_dn4, var_qg_dn5, var_qg_dn6, var_qg_dn7, var_qg_dn8, var_qg_dn9, var_qg_dn10, var_qg_dn11,)
    }
};
        var_qg = assign60700_e98456;
        var_qg_dn3 = assign60700_e98456_d_n3;
        var_qg_dn4 = assign60700_e98456_d_n4;
        var_qg_dn5 = assign60700_e98456_d_n5;
        var_qg_dn6 = assign60700_e98456_d_n6;
        var_qg_dn7 = assign60700_e98456_d_n7;
        var_qg_dn8 = assign60700_e98456_d_n8;
        var_qg_dn9 = assign60700_e98456_d_n9;
        var_qg_dn10 = assign60700_e98456_d_n10;
        var_qg_dn11 = assign60700_e98456_d_n11;
        var_qg_rv = 0.0;

        let (assign60730_e98477, assign60730_e98477_d_n3, assign60730_e98477_d_n4, assign60730_e98477_d_n5, assign60730_e98477_d_n6, assign60730_e98477_d_n7, assign60730_e98477_d_n8, assign60730_e98477_d_n9, assign60730_e98477_d_n10, assign60730_e98477_d_n11,) = {
    if (var_guard492 == 0.0) {
        let assign60730_e98475: f64 = (var_devsign * var_qgim);
        (assign60730_e98475, (var_devsign * var_qgim_dn3), (var_devsign * var_qgim_dn4), (var_devsign * var_qgim_dn5), (var_devsign * var_qgim_dn6), (var_devsign * var_qgim_dn7), (var_devsign * var_qgim_dn8), (var_devsign * var_qgim_dn9), (var_devsign * var_qgim_dn10), (var_devsign * var_qgim_dn11),)
    } else {
        (var_qgim_1, var_qgim_1_dn3, var_qgim_1_dn4, var_qgim_1_dn5, var_qgim_1_dn6, var_qgim_1_dn7, var_qgim_1_dn8, var_qgim_1_dn9, var_qgim_1_dn10, var_qgim_1_dn11,)
    }
};
        var_qgim_1 = assign60730_e98477;
        var_qgim_1_dn3 = assign60730_e98477_d_n3;
        var_qgim_1_dn4 = assign60730_e98477_d_n4;
        var_qgim_1_dn5 = assign60730_e98477_d_n5;
        var_qgim_1_dn6 = assign60730_e98477_d_n6;
        var_qgim_1_dn7 = assign60730_e98477_d_n7;
        var_qgim_1_dn8 = assign60730_e98477_d_n8;
        var_qgim_1_dn9 = assign60730_e98477_d_n9;
        var_qgim_1_dn10 = assign60730_e98477_d_n10;
        var_qgim_1_dn11 = assign60730_e98477_d_n11;
        var_qgim_1_rv = 0.0;

        let (assign60740_e98484, assign60740_e98484_d_n3, assign60740_e98484_d_n4, assign60740_e98484_d_n5, assign60740_e98484_d_n6, assign60740_e98484_d_n7, assign60740_e98484_d_n8, assign60740_e98484_d_n9, assign60740_e98484_d_n10, assign60740_e98484_d_n11,) = {
    if (var_guard492 == 0.0) {
        let assign60740_e98482: f64 = (var_devsign * var_qgiagbcp2);
        (assign60740_e98482, (var_devsign * var_qgiagbcp2_dn3), (var_devsign * var_qgiagbcp2_dn4), (var_devsign * var_qgiagbcp2_dn5), (var_devsign * var_qgiagbcp2_dn6), (var_devsign * var_qgiagbcp2_dn7), (var_devsign * var_qgiagbcp2_dn8), (var_devsign * var_qgiagbcp2_dn9), (var_devsign * var_qgiagbcp2_dn10), (var_devsign * var_qgiagbcp2_dn11),)
    } else {
        (var_qgiagbcp2_1, var_qgiagbcp2_1_dn3, var_qgiagbcp2_1_dn4, var_qgiagbcp2_1_dn5, var_qgiagbcp2_1_dn6, var_qgiagbcp2_1_dn7, var_qgiagbcp2_1_dn8, var_qgiagbcp2_1_dn9, var_qgiagbcp2_1_dn10, var_qgiagbcp2_1_dn11,)
    }
};
        var_qgiagbcp2_1 = assign60740_e98484;
        var_qgiagbcp2_1_dn3 = assign60740_e98484_d_n3;
        var_qgiagbcp2_1_dn4 = assign60740_e98484_d_n4;
        var_qgiagbcp2_1_dn5 = assign60740_e98484_d_n5;
        var_qgiagbcp2_1_dn6 = assign60740_e98484_d_n6;
        var_qgiagbcp2_1_dn7 = assign60740_e98484_d_n7;
        var_qgiagbcp2_1_dn8 = assign60740_e98484_d_n8;
        var_qgiagbcp2_1_dn9 = assign60740_e98484_d_n9;
        var_qgiagbcp2_1_dn10 = assign60740_e98484_d_n10;
        var_qgiagbcp2_1_dn11 = assign60740_e98484_d_n11;
        var_qgiagbcp2_1_rv = 0.0;

        var_weff_1 = var_weff;
        var_weff_1_rv = 0.0;

        var_leff_1 = var_leff;
        var_leff_1_rv = 0.0;

        let assign61510_e98885: f64 = if ((p.p41 != 0.0) && (p.p1099 > 0.0)) { 1.0 } else { 0.0 };
        var_guard893 = assign61510_e98885;
        var_guard893_rv = 0.0;

        *var_guard877_slot = var_guard877;
        *var_guard877_rv_slot = var_guard877_rv;
        *var_guard878_slot = var_guard878;
        *var_guard878_rv_slot = var_guard878_rv;
        *var_guard893_slot = var_guard893;
        *var_guard893_rv_slot = var_guard893_rv;
        *var_leff_1_slot = var_leff_1;
        *var_leff_1_rv_slot = var_leff_1_rv;
        *var_qb_2_slot = var_qb_2;
        *var_qb_2_dn10_slot = var_qb_2_dn10;
        *var_qb_2_dn11_slot = var_qb_2_dn11;
        *var_qb_2_dn3_slot = var_qb_2_dn3;
        *var_qb_2_dn4_slot = var_qb_2_dn4;
        *var_qb_2_dn5_slot = var_qb_2_dn5;
        *var_qb_2_dn6_slot = var_qb_2_dn6;
        *var_qb_2_dn7_slot = var_qb_2_dn7;
        *var_qb_2_dn8_slot = var_qb_2_dn8;
        *var_qb_2_dn9_slot = var_qb_2_dn9;
        *var_qb_2_rv_slot = var_qb_2_rv;
        *var_qd_1_slot = var_qd_1;
        *var_qd_1_dn10_slot = var_qd_1_dn10;
        *var_qd_1_dn11_slot = var_qd_1_dn11;
        *var_qd_1_dn3_slot = var_qd_1_dn3;
        *var_qd_1_dn4_slot = var_qd_1_dn4;
        *var_qd_1_dn5_slot = var_qd_1_dn5;
        *var_qd_1_dn6_slot = var_qd_1_dn6;
        *var_qd_1_dn7_slot = var_qd_1_dn7;
        *var_qd_1_dn8_slot = var_qd_1_dn8;
        *var_qd_1_dn9_slot = var_qd_1_dn9;
        *var_qd_1_rv_slot = var_qd_1_rv;
        *var_qdi_1_slot = var_qdi_1;
        *var_qdi_1_dn10_slot = var_qdi_1_dn10;
        *var_qdi_1_dn11_slot = var_qdi_1_dn11;
        *var_qdi_1_dn3_slot = var_qdi_1_dn3;
        *var_qdi_1_dn4_slot = var_qdi_1_dn4;
        *var_qdi_1_dn5_slot = var_qdi_1_dn5;
        *var_qdi_1_dn6_slot = var_qdi_1_dn6;
        *var_qdi_1_dn7_slot = var_qdi_1_dn7;
        *var_qdi_1_dn8_slot = var_qdi_1_dn8;
        *var_qdi_1_dn9_slot = var_qdi_1_dn9;
        *var_qdi_1_rv_slot = var_qdi_1_rv;
        *var_qdiagbcp2_1_slot = var_qdiagbcp2_1;
        *var_qdiagbcp2_1_dn10_slot = var_qdiagbcp2_1_dn10;
        *var_qdiagbcp2_1_dn11_slot = var_qdiagbcp2_1_dn11;
        *var_qdiagbcp2_1_dn3_slot = var_qdiagbcp2_1_dn3;
        *var_qdiagbcp2_1_dn4_slot = var_qdiagbcp2_1_dn4;
        *var_qdiagbcp2_1_dn5_slot = var_qdiagbcp2_1_dn5;
        *var_qdiagbcp2_1_dn6_slot = var_qdiagbcp2_1_dn6;
        *var_qdiagbcp2_1_dn7_slot = var_qdiagbcp2_1_dn7;
        *var_qdiagbcp2_1_dn8_slot = var_qdiagbcp2_1_dn8;
        *var_qdiagbcp2_1_dn9_slot = var_qdiagbcp2_1_dn9;
        *var_qdiagbcp2_1_rv_slot = var_qdiagbcp2_1_rv;
        *var_qdim_1_slot = var_qdim_1;
        *var_qdim_1_dn10_slot = var_qdim_1_dn10;
        *var_qdim_1_dn11_slot = var_qdim_1_dn11;
        *var_qdim_1_dn3_slot = var_qdim_1_dn3;
        *var_qdim_1_dn4_slot = var_qdim_1_dn4;
        *var_qdim_1_dn5_slot = var_qdim_1_dn5;
        *var_qdim_1_dn6_slot = var_qdim_1_dn6;
        *var_qdim_1_dn7_slot = var_qdim_1_dn7;
        *var_qdim_1_dn8_slot = var_qdim_1_dn8;
        *var_qdim_1_dn9_slot = var_qdim_1_dn9;
        *var_qdim_1_rv_slot = var_qdim_1_rv;
        *var_qg_slot = var_qg;
        *var_qg_dn10_slot = var_qg_dn10;
        *var_qg_dn11_slot = var_qg_dn11;
        *var_qg_dn3_slot = var_qg_dn3;
        *var_qg_dn4_slot = var_qg_dn4;
        *var_qg_dn5_slot = var_qg_dn5;
        *var_qg_dn6_slot = var_qg_dn6;
        *var_qg_dn7_slot = var_qg_dn7;
        *var_qg_dn8_slot = var_qg_dn8;
        *var_qg_dn9_slot = var_qg_dn9;
        *var_qg_rv_slot = var_qg_rv;
        *var_qgiagbcp2_1_slot = var_qgiagbcp2_1;
        *var_qgiagbcp2_1_dn10_slot = var_qgiagbcp2_1_dn10;
        *var_qgiagbcp2_1_dn11_slot = var_qgiagbcp2_1_dn11;
        *var_qgiagbcp2_1_dn3_slot = var_qgiagbcp2_1_dn3;
        *var_qgiagbcp2_1_dn4_slot = var_qgiagbcp2_1_dn4;
        *var_qgiagbcp2_1_dn5_slot = var_qgiagbcp2_1_dn5;
        *var_qgiagbcp2_1_dn6_slot = var_qgiagbcp2_1_dn6;
        *var_qgiagbcp2_1_dn7_slot = var_qgiagbcp2_1_dn7;
        *var_qgiagbcp2_1_dn8_slot = var_qgiagbcp2_1_dn8;
        *var_qgiagbcp2_1_dn9_slot = var_qgiagbcp2_1_dn9;
        *var_qgiagbcp2_1_rv_slot = var_qgiagbcp2_1_rv;
        *var_qgim_1_slot = var_qgim_1;
        *var_qgim_1_dn10_slot = var_qgim_1_dn10;
        *var_qgim_1_dn11_slot = var_qgim_1_dn11;
        *var_qgim_1_dn3_slot = var_qgim_1_dn3;
        *var_qgim_1_dn4_slot = var_qgim_1_dn4;
        *var_qgim_1_dn5_slot = var_qgim_1_dn5;
        *var_qgim_1_dn6_slot = var_qgim_1_dn6;
        *var_qgim_1_dn7_slot = var_qgim_1_dn7;
        *var_qgim_1_dn8_slot = var_qgim_1_dn8;
        *var_qgim_1_dn9_slot = var_qgim_1_dn9;
        *var_qgim_1_rv_slot = var_qgim_1_rv;
        *var_qs_2_slot = var_qs_2;
        *var_qs_2_dn10_slot = var_qs_2_dn10;
        *var_qs_2_dn11_slot = var_qs_2_dn11;
        *var_qs_2_dn3_slot = var_qs_2_dn3;
        *var_qs_2_dn4_slot = var_qs_2_dn4;
        *var_qs_2_dn5_slot = var_qs_2_dn5;
        *var_qs_2_dn6_slot = var_qs_2_dn6;
        *var_qs_2_dn7_slot = var_qs_2_dn7;
        *var_qs_2_dn8_slot = var_qs_2_dn8;
        *var_qs_2_dn9_slot = var_qs_2_dn9;
        *var_qs_2_rv_slot = var_qs_2_rv;
        *var_qsi_1_slot = var_qsi_1;
        *var_qsi_1_dn10_slot = var_qsi_1_dn10;
        *var_qsi_1_dn11_slot = var_qsi_1_dn11;
        *var_qsi_1_dn3_slot = var_qsi_1_dn3;
        *var_qsi_1_dn4_slot = var_qsi_1_dn4;
        *var_qsi_1_dn5_slot = var_qsi_1_dn5;
        *var_qsi_1_dn6_slot = var_qsi_1_dn6;
        *var_qsi_1_dn7_slot = var_qsi_1_dn7;
        *var_qsi_1_dn8_slot = var_qsi_1_dn8;
        *var_qsi_1_dn9_slot = var_qsi_1_dn9;
        *var_qsi_1_rv_slot = var_qsi_1_rv;
        *var_qsiagbcp2_1_slot = var_qsiagbcp2_1;
        *var_qsiagbcp2_1_dn10_slot = var_qsiagbcp2_1_dn10;
        *var_qsiagbcp2_1_dn11_slot = var_qsiagbcp2_1_dn11;
        *var_qsiagbcp2_1_dn3_slot = var_qsiagbcp2_1_dn3;
        *var_qsiagbcp2_1_dn4_slot = var_qsiagbcp2_1_dn4;
        *var_qsiagbcp2_1_dn5_slot = var_qsiagbcp2_1_dn5;
        *var_qsiagbcp2_1_dn6_slot = var_qsiagbcp2_1_dn6;
        *var_qsiagbcp2_1_dn7_slot = var_qsiagbcp2_1_dn7;
        *var_qsiagbcp2_1_dn8_slot = var_qsiagbcp2_1_dn8;
        *var_qsiagbcp2_1_dn9_slot = var_qsiagbcp2_1_dn9;
        *var_qsiagbcp2_1_rv_slot = var_qsiagbcp2_1_rv;
        *var_qsim_1_slot = var_qsim_1;
        *var_qsim_1_dn10_slot = var_qsim_1_dn10;
        *var_qsim_1_dn11_slot = var_qsim_1_dn11;
        *var_qsim_1_dn3_slot = var_qsim_1_dn3;
        *var_qsim_1_dn4_slot = var_qsim_1_dn4;
        *var_qsim_1_dn5_slot = var_qsim_1_dn5;
        *var_qsim_1_dn6_slot = var_qsim_1_dn6;
        *var_qsim_1_dn7_slot = var_qsim_1_dn7;
        *var_qsim_1_dn8_slot = var_qsim_1_dn8;
        *var_qsim_1_dn9_slot = var_qsim_1_dn9;
        *var_qsim_1_rv_slot = var_qsim_1_rv;
        *var_t6_slot = var_t6;
        *var_t6_dn10_slot = var_t6_dn10;
        *var_t6_dn11_slot = var_t6_dn11;
        *var_t6_dn3_slot = var_t6_dn3;
        *var_t6_dn4_slot = var_t6_dn4;
        *var_t6_dn5_slot = var_t6_dn5;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn7_slot = var_t6_dn7;
        *var_t6_dn8_slot = var_t6_dn8;
        *var_t6_dn9_slot = var_t6_dn9;
        *var_t6_rv_slot = var_t6_rv;
        *var_t7_slot = var_t7;
        *var_t7_dn10_slot = var_t7_dn10;
        *var_t7_dn11_slot = var_t7_dn11;
        *var_t7_dn3_slot = var_t7_dn3;
        *var_t7_dn4_slot = var_t7_dn4;
        *var_t7_dn5_slot = var_t7_dn5;
        *var_t7_dn6_slot = var_t7_dn6;
        *var_t7_dn7_slot = var_t7_dn7;
        *var_t7_dn8_slot = var_t7_dn8;
        *var_t7_dn9_slot = var_t7_dn9;
        *var_t7_rv_slot = var_t7_rv;
        *var_t8_slot = var_t8;
        *var_t8_dn10_slot = var_t8_dn10;
        *var_t8_dn11_slot = var_t8_dn11;
        *var_t8_dn3_slot = var_t8_dn3;
        *var_t8_dn4_slot = var_t8_dn4;
        *var_t8_dn5_slot = var_t8_dn5;
        *var_t8_dn6_slot = var_t8_dn6;
        *var_t8_dn7_slot = var_t8_dn7;
        *var_t8_dn8_slot = var_t8_dn8;
        *var_t8_dn9_slot = var_t8_dn9;
        *var_t8_rv_slot = var_t8_rv;
        *var_weff_1_slot = var_weff_1;
        *var_weff_1_rv_slot = var_weff_1_rv;
    }

    pub(super) fn stamp_reactive_block_213(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_devsign: f64,
        var_guard893: f64,
        var_ids: f64,
        var_ids_dn10: f64,
        var_ids_dn11: f64,
        var_ids_dn3: f64,
        var_ids_dn4: f64,
        var_ids_dn5: f64,
        var_ids_dn6: f64,
        var_ids_dn7: f64,
        var_ids_dn8: f64,
        var_ids_dn9: f64,
        var_leff_1: f64,
        var_neff_i: f64,
        var_qbdj: f64,
        var_qbdj_dn10: f64,
        var_qbdj_dn11: f64,
        var_qbdj_dn3: f64,
        var_qbdj_dn4: f64,
        var_qbdj_dn5: f64,
        var_qbdj_dn6: f64,
        var_qbdj_dn7: f64,
        var_qbdj_dn8: f64,
        var_qbdj_dn9: f64,
        var_qbi: f64,
        var_qbi_dn10: f64,
        var_qbi_dn11: f64,
        var_qbi_dn3: f64,
        var_qbi_dn4: f64,
        var_qbi_dn5: f64,
        var_qbi_dn6: f64,
        var_qbi_dn7: f64,
        var_qbi_dn8: f64,
        var_qbi_dn9: f64,
        var_qbsj: f64,
        var_qbsj_dn10: f64,
        var_qbsj_dn11: f64,
        var_qbsj_dn3: f64,
        var_qbsj_dn4: f64,
        var_qbsj_dn5: f64,
        var_qbsj_dn6: f64,
        var_qbsj_dn7: f64,
        var_qbsj_dn8: f64,
        var_qbsj_dn9: f64,
        var_qsub: f64,
        var_qsub_dn10: f64,
        var_qsub_dn11: f64,
        var_qsub_dn3: f64,
        var_qsub_dn4: f64,
        var_qsub_dn5: f64,
        var_qsub_dn6: f64,
        var_qsub_dn7: f64,
        var_qsub_dn8: f64,
        var_qsub_dn9: f64,
        var_rdrain: f64,
        var_rdrain_dn10: f64,
        var_rdrain_dn11: f64,
        var_rdrain_dn3: f64,
        var_rdrain_dn4: f64,
        var_rdrain_dn5: f64,
        var_rdrain_dn6: f64,
        var_rdrain_dn7: f64,
        var_rdrain_dn8: f64,
        var_rdrain_dn9: f64,
        var_rdraingeo: f64,
        var_rsource: f64,
        var_rsource_dn10: f64,
        var_rsource_dn11: f64,
        var_rsource_dn3: f64,
        var_rsource_dn4: f64,
        var_rsource_dn5: f64,
        var_rsource_dn6: f64,
        var_rsource_dn7: f64,
        var_rsource_dn8: f64,
        var_rsource_dn9: f64,
        var_rsourcegeo: f64,
        var_sigvds: f64,
        var_tratio: f64,
        var_tratio_dn4: f64,
        var_tratio_dn5: f64,
        var_ub_i: f64,
        var_ubte_i: f64,
        var_weff_1: f64,
        var_gbodyint_slot: &mut f64,
        var_gbodyint_dn10_slot: &mut f64,
        var_gbodyint_dn11_slot: &mut f64,
        var_gbodyint_dn3_slot: &mut f64,
        var_gbodyint_dn4_slot: &mut f64,
        var_gbodyint_dn5_slot: &mut f64,
        var_gbodyint_dn6_slot: &mut f64,
        var_gbodyint_dn7_slot: &mut f64,
        var_gbodyint_dn8_slot: &mut f64,
        var_gbodyint_dn9_slot: &mut f64,
        var_gbodyint_rv_slot: &mut f64,
        var_guard894_slot: &mut f64,
        var_guard894_rv_slot: &mut f64,
        var_guard895_slot: &mut f64,
        var_guard895_rv_slot: &mut f64,
        var_guard896_slot: &mut f64,
        var_guard896_rv_slot: &mut f64,
        var_guard897_slot: &mut f64,
        var_guard897_rv_slot: &mut f64,
        var_guard900_slot: &mut f64,
        var_guard900_rv_slot: &mut f64,
        var_guard901_slot: &mut f64,
        var_guard901_rv_slot: &mut f64,
        var_guard902_slot: &mut f64,
        var_guard902_rv_slot: &mut f64,
        var_guard903_slot: &mut f64,
        var_guard903_rv_slot: &mut f64,
        var_guard904_slot: &mut f64,
        var_guard904_rv_slot: &mut f64,
        var_guard905_slot: &mut f64,
        var_guard905_rv_slot: &mut f64,
        var_guard906_slot: &mut f64,
        var_guard906_rv_slot: &mut f64,
        var_guard908_slot: &mut f64,
        var_guard908_rv_slot: &mut f64,
        var_pdiss_slot: &mut f64,
        var_pdiss_dn0_slot: &mut f64,
        var_pdiss_dn10_slot: &mut f64,
        var_pdiss_dn11_slot: &mut f64,
        var_pdiss_dn2_slot: &mut f64,
        var_pdiss_dn3_slot: &mut f64,
        var_pdiss_dn4_slot: &mut f64,
        var_pdiss_dn5_slot: &mut f64,
        var_pdiss_dn6_slot: &mut f64,
        var_pdiss_dn7_slot: &mut f64,
        var_pdiss_dn8_slot: &mut f64,
        var_pdiss_dn9_slot: &mut f64,
        var_pdiss_rv_slot: &mut f64,
        var_qb1_slot: &mut f64,
        var_qb1_dn10_slot: &mut f64,
        var_qb1_dn11_slot: &mut f64,
        var_qb1_dn3_slot: &mut f64,
        var_qb1_dn4_slot: &mut f64,
        var_qb1_dn5_slot: &mut f64,
        var_qb1_dn6_slot: &mut f64,
        var_qb1_dn7_slot: &mut f64,
        var_qb1_dn8_slot: &mut f64,
        var_qb1_dn9_slot: &mut f64,
        var_qb1_rv_slot: &mut f64,
        var_qbody_slot: &mut f64,
        var_qbody_dn10_slot: &mut f64,
        var_qbody_dn11_slot: &mut f64,
        var_qbody_dn3_slot: &mut f64,
        var_qbody_dn4_slot: &mut f64,
        var_qbody_dn5_slot: &mut f64,
        var_qbody_dn6_slot: &mut f64,
        var_qbody_dn7_slot: &mut f64,
        var_qbody_dn8_slot: &mut f64,
        var_qbody_dn9_slot: &mut f64,
        var_qbody_rv_slot: &mut f64,
        var_rbodyext_slot: &mut f64,
        var_rbodyext_rv_slot: &mut f64,
        var_rbodyint_slot: &mut f64,
        var_rbodyint_dn10_slot: &mut f64,
        var_rbodyint_dn11_slot: &mut f64,
        var_rbodyint_dn3_slot: &mut f64,
        var_rbodyint_dn4_slot: &mut f64,
        var_rbodyint_dn5_slot: &mut f64,
        var_rbodyint_dn6_slot: &mut f64,
        var_rbodyint_dn7_slot: &mut f64,
        var_rbodyint_dn8_slot: &mut f64,
        var_rbodyint_dn9_slot: &mut f64,
        var_rbodyint_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_ub_t_slot: &mut f64,
        var_ub_t_dn4_slot: &mut f64,
        var_ub_t_dn5_slot: &mut f64,
        var_ub_t_rv_slot: &mut f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let mut var_gbodyint: f64 = *var_gbodyint_slot;
        let mut var_gbodyint_dn10: f64 = *var_gbodyint_dn10_slot;
        let mut var_gbodyint_dn11: f64 = *var_gbodyint_dn11_slot;
        let mut var_gbodyint_dn3: f64 = *var_gbodyint_dn3_slot;
        let mut var_gbodyint_dn4: f64 = *var_gbodyint_dn4_slot;
        let mut var_gbodyint_dn5: f64 = *var_gbodyint_dn5_slot;
        let mut var_gbodyint_dn6: f64 = *var_gbodyint_dn6_slot;
        let mut var_gbodyint_dn7: f64 = *var_gbodyint_dn7_slot;
        let mut var_gbodyint_dn8: f64 = *var_gbodyint_dn8_slot;
        let mut var_gbodyint_dn9: f64 = *var_gbodyint_dn9_slot;
        let mut var_gbodyint_rv: f64 = *var_gbodyint_rv_slot;
        let mut var_guard894: f64 = *var_guard894_slot;
        let mut var_guard894_rv: f64 = *var_guard894_rv_slot;
        let mut var_guard895: f64 = *var_guard895_slot;
        let mut var_guard895_rv: f64 = *var_guard895_rv_slot;
        let mut var_guard896: f64 = *var_guard896_slot;
        let mut var_guard896_rv: f64 = *var_guard896_rv_slot;
        let mut var_guard897: f64 = *var_guard897_slot;
        let mut var_guard897_rv: f64 = *var_guard897_rv_slot;
        let mut var_guard900: f64 = *var_guard900_slot;
        let mut var_guard900_rv: f64 = *var_guard900_rv_slot;
        let mut var_guard901: f64 = *var_guard901_slot;
        let mut var_guard901_rv: f64 = *var_guard901_rv_slot;
        let mut var_guard902: f64 = *var_guard902_slot;
        let mut var_guard902_rv: f64 = *var_guard902_rv_slot;
        let mut var_guard903: f64 = *var_guard903_slot;
        let mut var_guard903_rv: f64 = *var_guard903_rv_slot;
        let mut var_guard904: f64 = *var_guard904_slot;
        let mut var_guard904_rv: f64 = *var_guard904_rv_slot;
        let mut var_guard905: f64 = *var_guard905_slot;
        let mut var_guard905_rv: f64 = *var_guard905_rv_slot;
        let mut var_guard906: f64 = *var_guard906_slot;
        let mut var_guard906_rv: f64 = *var_guard906_rv_slot;
        let mut var_guard908: f64 = *var_guard908_slot;
        let mut var_guard908_rv: f64 = *var_guard908_rv_slot;
        let mut var_pdiss: f64 = *var_pdiss_slot;
        let mut var_pdiss_dn0: f64 = *var_pdiss_dn0_slot;
        let mut var_pdiss_dn10: f64 = *var_pdiss_dn10_slot;
        let mut var_pdiss_dn11: f64 = *var_pdiss_dn11_slot;
        let mut var_pdiss_dn2: f64 = *var_pdiss_dn2_slot;
        let mut var_pdiss_dn3: f64 = *var_pdiss_dn3_slot;
        let mut var_pdiss_dn4: f64 = *var_pdiss_dn4_slot;
        let mut var_pdiss_dn5: f64 = *var_pdiss_dn5_slot;
        let mut var_pdiss_dn6: f64 = *var_pdiss_dn6_slot;
        let mut var_pdiss_dn7: f64 = *var_pdiss_dn7_slot;
        let mut var_pdiss_dn8: f64 = *var_pdiss_dn8_slot;
        let mut var_pdiss_dn9: f64 = *var_pdiss_dn9_slot;
        let mut var_pdiss_rv: f64 = *var_pdiss_rv_slot;
        let mut var_qb1: f64 = *var_qb1_slot;
        let mut var_qb1_dn10: f64 = *var_qb1_dn10_slot;
        let mut var_qb1_dn11: f64 = *var_qb1_dn11_slot;
        let mut var_qb1_dn3: f64 = *var_qb1_dn3_slot;
        let mut var_qb1_dn4: f64 = *var_qb1_dn4_slot;
        let mut var_qb1_dn5: f64 = *var_qb1_dn5_slot;
        let mut var_qb1_dn6: f64 = *var_qb1_dn6_slot;
        let mut var_qb1_dn7: f64 = *var_qb1_dn7_slot;
        let mut var_qb1_dn8: f64 = *var_qb1_dn8_slot;
        let mut var_qb1_dn9: f64 = *var_qb1_dn9_slot;
        let mut var_qb1_rv: f64 = *var_qb1_rv_slot;
        let mut var_qbody: f64 = *var_qbody_slot;
        let mut var_qbody_dn10: f64 = *var_qbody_dn10_slot;
        let mut var_qbody_dn11: f64 = *var_qbody_dn11_slot;
        let mut var_qbody_dn3: f64 = *var_qbody_dn3_slot;
        let mut var_qbody_dn4: f64 = *var_qbody_dn4_slot;
        let mut var_qbody_dn5: f64 = *var_qbody_dn5_slot;
        let mut var_qbody_dn6: f64 = *var_qbody_dn6_slot;
        let mut var_qbody_dn7: f64 = *var_qbody_dn7_slot;
        let mut var_qbody_dn8: f64 = *var_qbody_dn8_slot;
        let mut var_qbody_dn9: f64 = *var_qbody_dn9_slot;
        let mut var_qbody_rv: f64 = *var_qbody_rv_slot;
        let mut var_rbodyext: f64 = *var_rbodyext_slot;
        let mut var_rbodyext_rv: f64 = *var_rbodyext_rv_slot;
        let mut var_rbodyint: f64 = *var_rbodyint_slot;
        let mut var_rbodyint_dn10: f64 = *var_rbodyint_dn10_slot;
        let mut var_rbodyint_dn11: f64 = *var_rbodyint_dn11_slot;
        let mut var_rbodyint_dn3: f64 = *var_rbodyint_dn3_slot;
        let mut var_rbodyint_dn4: f64 = *var_rbodyint_dn4_slot;
        let mut var_rbodyint_dn5: f64 = *var_rbodyint_dn5_slot;
        let mut var_rbodyint_dn6: f64 = *var_rbodyint_dn6_slot;
        let mut var_rbodyint_dn7: f64 = *var_rbodyint_dn7_slot;
        let mut var_rbodyint_dn8: f64 = *var_rbodyint_dn8_slot;
        let mut var_rbodyint_dn9: f64 = *var_rbodyint_dn9_slot;
        let mut var_rbodyint_rv: f64 = *var_rbodyint_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_ub_t: f64 = *var_ub_t_slot;
        let mut var_ub_t_dn4: f64 = *var_ub_t_dn4_slot;
        let mut var_ub_t_dn5: f64 = *var_ub_t_dn5_slot;
        let mut var_ub_t_rv: f64 = *var_ub_t_rv_slot;

        let (assign61520_e98895, assign61520_e98895_d_n0, assign61520_e98895_d_n2, assign61520_e98895_d_n3, assign61520_e98895_d_n4, assign61520_e98895_d_n5, assign61520_e98895_d_n6, assign61520_e98895_d_n7, assign61520_e98895_d_n8, assign61520_e98895_d_n9, assign61520_e98895_d_n10, assign61520_e98895_d_n11,) = {
    if (var_guard893 != 0.0) {
        let assign61520_e98889: f64 = (var_devsign * var_sigvds);
        let assign61520_e98891: f64 = (assign61520_e98889 * var_ids);
        let assign61520_e98893: f64 = (assign61520_e98891 * (nv6 - nv7));
        (assign61520_e98893, 0.0, 0.0, ((assign61520_e98889 * var_ids_dn3) * (nv6 - nv7)), ((assign61520_e98889 * var_ids_dn4) * (nv6 - nv7)), ((assign61520_e98889 * var_ids_dn5) * (nv6 - nv7)), (((assign61520_e98889 * var_ids_dn6) * (nv6 - nv7)) + assign61520_e98891), (((assign61520_e98889 * var_ids_dn7) * (nv6 - nv7)) + (-assign61520_e98891)), ((assign61520_e98889 * var_ids_dn8) * (nv6 - nv7)), ((assign61520_e98889 * var_ids_dn9) * (nv6 - nv7)), ((assign61520_e98889 * var_ids_dn10) * (nv6 - nv7)), ((assign61520_e98889 * var_ids_dn11) * (nv6 - nv7)),)
    } else {
        (var_pdiss, var_pdiss_dn0, var_pdiss_dn2, var_pdiss_dn3, var_pdiss_dn4, var_pdiss_dn5, var_pdiss_dn6, var_pdiss_dn7, var_pdiss_dn8, var_pdiss_dn9, var_pdiss_dn10, var_pdiss_dn11,)
    }
};
        var_pdiss = assign61520_e98895;
        var_pdiss_dn0 = assign61520_e98895_d_n0;
        var_pdiss_dn2 = assign61520_e98895_d_n2;
        var_pdiss_dn3 = assign61520_e98895_d_n3;
        var_pdiss_dn4 = assign61520_e98895_d_n4;
        var_pdiss_dn5 = assign61520_e98895_d_n5;
        var_pdiss_dn6 = assign61520_e98895_d_n6;
        var_pdiss_dn7 = assign61520_e98895_d_n7;
        var_pdiss_dn8 = assign61520_e98895_d_n8;
        var_pdiss_dn9 = assign61520_e98895_d_n9;
        var_pdiss_dn10 = assign61520_e98895_d_n10;
        var_pdiss_dn11 = assign61520_e98895_d_n11;
        var_pdiss_rv = 0.0;

        let assign61530_e98902: f64 = if ((p.p33 != 2.0) && (var_rdraingeo > 0.0)) { 1.0 } else { 0.0 };
        var_guard894 = assign61530_e98902;
        var_guard894_rv = 0.0;

        let (assign61540_e98914, assign61540_e98914_d_n0, assign61540_e98914_d_n2, assign61540_e98914_d_n3, assign61540_e98914_d_n4, assign61540_e98914_d_n5, assign61540_e98914_d_n6, assign61540_e98914_d_n7, assign61540_e98914_d_n8, assign61540_e98914_d_n9, assign61540_e98914_d_n10, assign61540_e98914_d_n11,) = {
    if ((var_guard893 != 0.0) && (var_guard894 != 0.0)) {
        let assign61540_e98909: f64 = ((nv0 - nv6) * (nv0 - nv6));
        let assign61540_e98911: f64 = (assign61540_e98909 / var_rdrain);
        let assign61540_e98912: f64 = (var_pdiss + assign61540_e98911);
        (assign61540_e98912, (var_pdiss_dn0 + (((nv0 - nv6) + (nv0 - nv6)) / var_rdrain)), var_pdiss_dn2, (var_pdiss_dn3 + (-((assign61540_e98909 * var_rdrain_dn3) / (var_rdrain * var_rdrain)))), (var_pdiss_dn4 + (-((assign61540_e98909 * var_rdrain_dn4) / (var_rdrain * var_rdrain)))), (var_pdiss_dn5 + (-((assign61540_e98909 * var_rdrain_dn5) / (var_rdrain * var_rdrain)))), (var_pdiss_dn6 + (((((-(nv0 - nv6)) + (-(nv0 - nv6))) * var_rdrain) - (assign61540_e98909 * var_rdrain_dn6)) / (var_rdrain * var_rdrain))), (var_pdiss_dn7 + (-((assign61540_e98909 * var_rdrain_dn7) / (var_rdrain * var_rdrain)))), (var_pdiss_dn8 + (-((assign61540_e98909 * var_rdrain_dn8) / (var_rdrain * var_rdrain)))), (var_pdiss_dn9 + (-((assign61540_e98909 * var_rdrain_dn9) / (var_rdrain * var_rdrain)))), (var_pdiss_dn10 + (-((assign61540_e98909 * var_rdrain_dn10) / (var_rdrain * var_rdrain)))), (var_pdiss_dn11 + (-((assign61540_e98909 * var_rdrain_dn11) / (var_rdrain * var_rdrain)))),)
    } else {
        (var_pdiss, var_pdiss_dn0, var_pdiss_dn2, var_pdiss_dn3, var_pdiss_dn4, var_pdiss_dn5, var_pdiss_dn6, var_pdiss_dn7, var_pdiss_dn8, var_pdiss_dn9, var_pdiss_dn10, var_pdiss_dn11,)
    }
};
        var_pdiss = assign61540_e98914;
        var_pdiss_dn0 = assign61540_e98914_d_n0;
        var_pdiss_dn2 = assign61540_e98914_d_n2;
        var_pdiss_dn3 = assign61540_e98914_d_n3;
        var_pdiss_dn4 = assign61540_e98914_d_n4;
        var_pdiss_dn5 = assign61540_e98914_d_n5;
        var_pdiss_dn6 = assign61540_e98914_d_n6;
        var_pdiss_dn7 = assign61540_e98914_d_n7;
        var_pdiss_dn8 = assign61540_e98914_d_n8;
        var_pdiss_dn9 = assign61540_e98914_d_n9;
        var_pdiss_dn10 = assign61540_e98914_d_n10;
        var_pdiss_dn11 = assign61540_e98914_d_n11;
        var_pdiss_rv = 0.0;

        let assign61550_e98921: f64 = if ((p.p33 != 2.0) && (var_rsourcegeo > 0.0)) { 1.0 } else { 0.0 };
        var_guard895 = assign61550_e98921;
        var_guard895_rv = 0.0;

        let (assign61560_e98933, assign61560_e98933_d_n0, assign61560_e98933_d_n2, assign61560_e98933_d_n3, assign61560_e98933_d_n4, assign61560_e98933_d_n5, assign61560_e98933_d_n6, assign61560_e98933_d_n7, assign61560_e98933_d_n8, assign61560_e98933_d_n9, assign61560_e98933_d_n10, assign61560_e98933_d_n11,) = {
    if ((var_guard893 != 0.0) && (var_guard895 != 0.0)) {
        let assign61560_e98928: f64 = ((nv2 - nv7) * (nv2 - nv7));
        let assign61560_e98930: f64 = (assign61560_e98928 / var_rsource);
        let assign61560_e98931: f64 = (var_pdiss + assign61560_e98930);
        (assign61560_e98931, var_pdiss_dn0, (var_pdiss_dn2 + (((nv2 - nv7) + (nv2 - nv7)) / var_rsource)), (var_pdiss_dn3 + (-((assign61560_e98928 * var_rsource_dn3) / (var_rsource * var_rsource)))), (var_pdiss_dn4 + (-((assign61560_e98928 * var_rsource_dn4) / (var_rsource * var_rsource)))), (var_pdiss_dn5 + (-((assign61560_e98928 * var_rsource_dn5) / (var_rsource * var_rsource)))), (var_pdiss_dn6 + (-((assign61560_e98928 * var_rsource_dn6) / (var_rsource * var_rsource)))), (var_pdiss_dn7 + (((((-(nv2 - nv7)) + (-(nv2 - nv7))) * var_rsource) - (assign61560_e98928 * var_rsource_dn7)) / (var_rsource * var_rsource))), (var_pdiss_dn8 + (-((assign61560_e98928 * var_rsource_dn8) / (var_rsource * var_rsource)))), (var_pdiss_dn9 + (-((assign61560_e98928 * var_rsource_dn9) / (var_rsource * var_rsource)))), (var_pdiss_dn10 + (-((assign61560_e98928 * var_rsource_dn10) / (var_rsource * var_rsource)))), (var_pdiss_dn11 + (-((assign61560_e98928 * var_rsource_dn11) / (var_rsource * var_rsource)))),)
    } else {
        (var_pdiss, var_pdiss_dn0, var_pdiss_dn2, var_pdiss_dn3, var_pdiss_dn4, var_pdiss_dn5, var_pdiss_dn6, var_pdiss_dn7, var_pdiss_dn8, var_pdiss_dn9, var_pdiss_dn10, var_pdiss_dn11,)
    }
};
        var_pdiss = assign61560_e98933;
        var_pdiss_dn0 = assign61560_e98933_d_n0;
        var_pdiss_dn2 = assign61560_e98933_d_n2;
        var_pdiss_dn3 = assign61560_e98933_d_n3;
        var_pdiss_dn4 = assign61560_e98933_d_n4;
        var_pdiss_dn5 = assign61560_e98933_d_n5;
        var_pdiss_dn6 = assign61560_e98933_d_n6;
        var_pdiss_dn7 = assign61560_e98933_d_n7;
        var_pdiss_dn8 = assign61560_e98933_d_n8;
        var_pdiss_dn9 = assign61560_e98933_d_n9;
        var_pdiss_dn10 = assign61560_e98933_d_n10;
        var_pdiss_dn11 = assign61560_e98933_d_n11;
        var_pdiss_rv = 0.0;

        let assign61570_e98938: f64 = if ((p.p40 != 0.0) && (!true)) { 1.0 } else { 0.0 };
        var_guard896 = assign61570_e98938;
        var_guard896_rv = 0.0;

        let assign61580_e98940: f64 = 1.0;
        var_guard897 = assign61580_e98940;
        var_guard897_rv = 0.0;

        let assign61610_e98950: f64 = (p.p1359 * p.p1358);
        var_rbodyext = assign61610_e98950;
        var_rbodyext_rv = 0.0;

        let assign61620_e98958: f64 = if ((p.p43 == 0.0) || (!true)) { 1.0 } else { 0.0 };
        var_guard900 = assign61620_e98958;
        var_guard900_rv = 0.0;

        let assign61630_e98963: f64 = if ((p.p40 != 0.0) && (!true)) { 1.0 } else { 0.0 };
        var_guard901 = assign61630_e98963;
        var_guard901_rv = 0.0;

        let assign61640_e98966: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        var_guard902 = assign61640_e98966;
        var_guard902_rv = 0.0;

        let (assign61650_e98994, assign61650_e98994_d_n3, assign61650_e98994_d_n4, assign61650_e98994_d_n5, assign61650_e98994_d_n6, assign61650_e98994_d_n7, assign61650_e98994_d_n8, assign61650_e98994_d_n9, assign61650_e98994_d_n10, assign61650_e98994_d_n11,) = {
    if (((var_guard900 == 0.0) && (var_guard901 == 0.0)) && (var_guard902 != 0.0)) {
        let assign61650_e98976: f64 = (p.p1357 * p.p1356);
        let assign61650_e98978: f64 = (assign61650_e98976 * p.p1360);
        let assign61650_e98981: f64 = (2.0 * p.p1356);
        let assign61650_e98984: f64 = (p.p1360 * var_leff_1);
        let assign61650_e98985: f64 = (assign61650_e98981 + assign61650_e98984);
        let assign61650_e98986: f64 = (assign61650_e98978 / assign61650_e98985);
        let assign61650_e98988: f64 = (assign61650_e98986 * var_weff_1);
        let assign61650_e98990: f64 = (assign61650_e98988 / p.p1373);
        let assign61650_e98992: f64 = (assign61650_e98990 / p.p2);
        (assign61650_e98992, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rbodyint, var_rbodyint_dn3, var_rbodyint_dn4, var_rbodyint_dn5, var_rbodyint_dn6, var_rbodyint_dn7, var_rbodyint_dn8, var_rbodyint_dn9, var_rbodyint_dn10, var_rbodyint_dn11,)
    }
};
        var_rbodyint = assign61650_e98994;
        var_rbodyint_dn3 = assign61650_e98994_d_n3;
        var_rbodyint_dn4 = assign61650_e98994_d_n4;
        var_rbodyint_dn5 = assign61650_e98994_d_n5;
        var_rbodyint_dn6 = assign61650_e98994_d_n6;
        var_rbodyint_dn7 = assign61650_e98994_d_n7;
        var_rbodyint_dn8 = assign61650_e98994_d_n8;
        var_rbodyint_dn9 = assign61650_e98994_d_n9;
        var_rbodyint_dn10 = assign61650_e98994_d_n10;
        var_rbodyint_dn11 = assign61650_e98994_d_n11;
        var_rbodyint_rv = 0.0;

        let assign61660_e98997: f64 = if var_rbodyint < 0.001 { 1.0 } else { 0.0 };
        var_guard903 = assign61660_e98997;
        var_guard903_rv = 0.0;

        let assign61670_e99000: f64 = if var_rbodyext <= 0.001 { 1.0 } else { 0.0 };
        var_guard904 = assign61670_e99000;
        var_guard904_rv = 0.0;

        let (assign61680_e99016, assign61680_e99016_d_n3, assign61680_e99016_d_n4, assign61680_e99016_d_n5, assign61680_e99016_d_n6, assign61680_e99016_d_n7, assign61680_e99016_d_n8, assign61680_e99016_d_n9, assign61680_e99016_d_n10, assign61680_e99016_d_n11,) = {
    if (((((var_guard900 == 0.0) && (var_guard901 == 0.0)) && (var_guard902 != 0.0)) && (var_guard903 != 0.0)) && (var_guard904 != 0.0)) {
        let assign61680_e99014: f64 = (1.0 / 0.001);
        (assign61680_e99014, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11,)
    }
};
        var_t0 = assign61680_e99016;
        var_t0_dn3 = assign61680_e99016_d_n3;
        var_t0_dn4 = assign61680_e99016_d_n4;
        var_t0_dn5 = assign61680_e99016_d_n5;
        var_t0_dn6 = assign61680_e99016_d_n6;
        var_t0_dn7 = assign61680_e99016_d_n7;
        var_t0_dn8 = assign61680_e99016_d_n8;
        var_t0_dn9 = assign61680_e99016_d_n9;
        var_t0_dn10 = assign61680_e99016_d_n10;
        var_t0_dn11 = assign61680_e99016_d_n11;
        var_t0_rv = 0.0;

        let (assign61690_e99033, assign61690_e99033_d_n3, assign61690_e99033_d_n4, assign61690_e99033_d_n5, assign61690_e99033_d_n6, assign61690_e99033_d_n7, assign61690_e99033_d_n8, assign61690_e99033_d_n9, assign61690_e99033_d_n10, assign61690_e99033_d_n11,) = {
    if (((((var_guard900 == 0.0) && (var_guard901 == 0.0)) && (var_guard902 != 0.0)) && (var_guard903 != 0.0)) && (var_guard904 == 0.0)) {
        let assign61690_e99031: f64 = (1.0 / var_rbodyext);
        (assign61690_e99031, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11,)
    }
};
        var_t0 = assign61690_e99033;
        var_t0_dn3 = assign61690_e99033_d_n3;
        var_t0_dn4 = assign61690_e99033_d_n4;
        var_t0_dn5 = assign61690_e99033_d_n5;
        var_t0_dn6 = assign61690_e99033_d_n6;
        var_t0_dn7 = assign61690_e99033_d_n7;
        var_t0_dn8 = assign61690_e99033_d_n8;
        var_t0_dn9 = assign61690_e99033_d_n9;
        var_t0_dn10 = assign61690_e99033_d_n10;
        var_t0_dn11 = assign61690_e99033_d_n11;
        var_t0_rv = 0.0;

        let (assign61720_e99077, assign61720_e99077_d_n4, assign61720_e99077_d_n5,) = {
    if (((var_guard900 == 0.0) && (var_guard901 == 0.0)) && (var_guard902 == 0.0)) {
        let assign61720_e99074: f64 = (var_tratio).powf(var_ubte_i);
        let assign61720_e99075: f64 = (var_ub_i * assign61720_e99074);
        (assign61720_e99075, (var_ub_i * if 0.0 == 0.0 && ((var_ubte_i) as f64).is_finite() && ((var_ubte_i) as f64).fract() == 0.0 { if var_ubte_i == 0.0 { 0.0 } else { (var_ubte_i * ((var_tratio).powf(var_ubte_i - 1.0) * var_tratio_dn4)) } } else { (assign61720_e99074 * (var_ubte_i * (var_tratio_dn4 / var_tratio))) }), (var_ub_i * if 0.0 == 0.0 && ((var_ubte_i) as f64).is_finite() && ((var_ubte_i) as f64).fract() == 0.0 { if var_ubte_i == 0.0 { 0.0 } else { (var_ubte_i * ((var_tratio).powf(var_ubte_i - 1.0) * var_tratio_dn5)) } } else { (assign61720_e99074 * (var_ubte_i * (var_tratio_dn5 / var_tratio))) }),)
    } else {
        (var_ub_t, var_ub_t_dn4, var_ub_t_dn5,)
    }
};
        var_ub_t = assign61720_e99077;
        var_ub_t_dn4 = assign61720_e99077_d_n4;
        var_ub_t_dn5 = assign61720_e99077_d_n5;
        var_ub_t_rv = 0.0;

        let (assign61730_e99095, assign61730_e99095_d_n3, assign61730_e99095_d_n4, assign61730_e99095_d_n5, assign61730_e99095_d_n6, assign61730_e99095_d_n7, assign61730_e99095_d_n8, assign61730_e99095_d_n9, assign61730_e99095_d_n10, assign61730_e99095_d_n11,) = {
    if (((var_guard900 == 0.0) && (var_guard901 == 0.0)) && (var_guard902 == 0.0)) {
        let assign61730_e99088: f64 = (var_qbi + var_qbsj);
        let assign61730_e99090: f64 = (assign61730_e99088 + var_qbdj);
        let assign61730_e99091: f64 = (-assign61730_e99090);
        let assign61730_e99093: f64 = (assign61730_e99091 + var_qsub);
        (assign61730_e99093, ((-((var_qbi_dn3 + var_qbsj_dn3) + var_qbdj_dn3)) + var_qsub_dn3), ((-((var_qbi_dn4 + var_qbsj_dn4) + var_qbdj_dn4)) + var_qsub_dn4), ((-((var_qbi_dn5 + var_qbsj_dn5) + var_qbdj_dn5)) + var_qsub_dn5), ((-((var_qbi_dn6 + var_qbsj_dn6) + var_qbdj_dn6)) + var_qsub_dn6), ((-((var_qbi_dn7 + var_qbsj_dn7) + var_qbdj_dn7)) + var_qsub_dn7), ((-((var_qbi_dn8 + var_qbsj_dn8) + var_qbdj_dn8)) + var_qsub_dn8), ((-((var_qbi_dn9 + var_qbsj_dn9) + var_qbdj_dn9)) + var_qsub_dn9), ((-((var_qbi_dn10 + var_qbsj_dn10) + var_qbdj_dn10)) + var_qsub_dn10), ((-((var_qbi_dn11 + var_qbsj_dn11) + var_qbdj_dn11)) + var_qsub_dn11),)
    } else {
        (var_qb1, var_qb1_dn3, var_qb1_dn4, var_qb1_dn5, var_qb1_dn6, var_qb1_dn7, var_qb1_dn8, var_qb1_dn9, var_qb1_dn10, var_qb1_dn11,)
    }
};
        var_qb1 = assign61730_e99095;
        var_qb1_dn3 = assign61730_e99095_d_n3;
        var_qb1_dn4 = assign61730_e99095_d_n4;
        var_qb1_dn5 = assign61730_e99095_d_n5;
        var_qb1_dn6 = assign61730_e99095_d_n6;
        var_qb1_dn7 = assign61730_e99095_d_n7;
        var_qb1_dn8 = assign61730_e99095_d_n8;
        var_qb1_dn9 = assign61730_e99095_d_n9;
        var_qb1_dn10 = assign61730_e99095_d_n10;
        var_qb1_dn11 = assign61730_e99095_d_n11;
        var_qb1_rv = 0.0;

        let (assign61740_e99116, assign61740_e99116_d_n3, assign61740_e99116_d_n4, assign61740_e99116_d_n5, assign61740_e99116_d_n6, assign61740_e99116_d_n7, assign61740_e99116_d_n8, assign61740_e99116_d_n9, assign61740_e99116_d_n10, assign61740_e99116_d_n11,) = {
    if (((var_guard900 == 0.0) && (var_guard901 == 0.0)) && (var_guard902 == 0.0)) {
        let assign61740_e99106: f64 = (1.602176462e-19 * var_neff_i);
        let assign61740_e99108: f64 = (assign61740_e99106 * p.p74);
        let assign61740_e99110: f64 = (assign61740_e99108 * var_weff_1);
        let assign61740_e99112: f64 = (assign61740_e99110 * var_leff_1);
        let assign61740_e99114: f64 = (assign61740_e99112 - var_qb1);
        (assign61740_e99114, (-var_qb1_dn3), (-var_qb1_dn4), (-var_qb1_dn5), (-var_qb1_dn6), (-var_qb1_dn7), (-var_qb1_dn8), (-var_qb1_dn9), (-var_qb1_dn10), (-var_qb1_dn11),)
    } else {
        (var_qbody, var_qbody_dn3, var_qbody_dn4, var_qbody_dn5, var_qbody_dn6, var_qbody_dn7, var_qbody_dn8, var_qbody_dn9, var_qbody_dn10, var_qbody_dn11,)
    }
};
        var_qbody = assign61740_e99116;
        var_qbody_dn3 = assign61740_e99116_d_n3;
        var_qbody_dn4 = assign61740_e99116_d_n4;
        var_qbody_dn5 = assign61740_e99116_d_n5;
        var_qbody_dn6 = assign61740_e99116_d_n6;
        var_qbody_dn7 = assign61740_e99116_d_n7;
        var_qbody_dn8 = assign61740_e99116_d_n8;
        var_qbody_dn9 = assign61740_e99116_d_n9;
        var_qbody_dn10 = assign61740_e99116_d_n10;
        var_qbody_dn11 = assign61740_e99116_d_n11;
        var_qbody_rv = 0.0;

        let (assign61750_e99129, assign61750_e99129_d_n3, assign61750_e99129_d_n4, assign61750_e99129_d_n5, assign61750_e99129_d_n6, assign61750_e99129_d_n7, assign61750_e99129_d_n8, assign61750_e99129_d_n9, assign61750_e99129_d_n10, assign61750_e99129_d_n11,) = {
    if (((var_guard900 == 0.0) && (var_guard901 == 0.0)) && (var_guard902 == 0.0)) {
        let assign61750_e99127: f64 = (var_ub_t * var_qbody);
        (assign61750_e99127, (var_ub_t * var_qbody_dn3), ((var_ub_t_dn4 * var_qbody) + (var_ub_t * var_qbody_dn4)), ((var_ub_t_dn5 * var_qbody) + (var_ub_t * var_qbody_dn5)), (var_ub_t * var_qbody_dn6), (var_ub_t * var_qbody_dn7), (var_ub_t * var_qbody_dn8), (var_ub_t * var_qbody_dn9), (var_ub_t * var_qbody_dn10), (var_ub_t * var_qbody_dn11),)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11,)
    }
};
        var_t0 = assign61750_e99129;
        var_t0_dn3 = assign61750_e99129_d_n3;
        var_t0_dn4 = assign61750_e99129_d_n4;
        var_t0_dn5 = assign61750_e99129_d_n5;
        var_t0_dn6 = assign61750_e99129_d_n6;
        var_t0_dn7 = assign61750_e99129_d_n7;
        var_t0_dn8 = assign61750_e99129_d_n8;
        var_t0_dn9 = assign61750_e99129_d_n9;
        var_t0_dn10 = assign61750_e99129_d_n10;
        var_t0_dn11 = assign61750_e99129_d_n11;
        var_t0_rv = 0.0;

        let (assign61760_e99142, assign61760_e99142_d_n3, assign61760_e99142_d_n4, assign61760_e99142_d_n5, assign61760_e99142_d_n6, assign61760_e99142_d_n7, assign61760_e99142_d_n8, assign61760_e99142_d_n9, assign61760_e99142_d_n10, assign61760_e99142_d_n11,) = {
    if (((var_guard900 == 0.0) && (var_guard901 == 0.0)) && (var_guard902 == 0.0)) {
        let assign61760_e99140: f64 = (var_weff_1 * var_weff_1);
        (assign61760_e99140, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11,)
    }
};
        var_t1 = assign61760_e99142;
        var_t1_dn3 = assign61760_e99142_d_n3;
        var_t1_dn4 = assign61760_e99142_d_n4;
        var_t1_dn5 = assign61760_e99142_d_n5;
        var_t1_dn6 = assign61760_e99142_d_n6;
        var_t1_dn7 = assign61760_e99142_d_n7;
        var_t1_dn8 = assign61760_e99142_d_n8;
        var_t1_dn9 = assign61760_e99142_d_n9;
        var_t1_dn10 = assign61760_e99142_d_n10;
        var_t1_dn11 = assign61760_e99142_d_n11;
        var_t1_rv = 0.0;

        let (assign61770_e99157, assign61770_e99157_d_n3, assign61770_e99157_d_n4, assign61770_e99157_d_n5, assign61770_e99157_d_n6, assign61770_e99157_d_n7, assign61770_e99157_d_n8, assign61770_e99157_d_n9, assign61770_e99157_d_n10, assign61770_e99157_d_n11,) = {
    if (((var_guard900 == 0.0) && (var_guard901 == 0.0)) && (var_guard902 == 0.0)) {
        let assign61770_e99153: f64 = (p.p2 * var_t0);
        let assign61770_e99155: f64 = (assign61770_e99153 / var_t1);
        (assign61770_e99155, ((((p.p2 * var_t0_dn3) * var_t1) - (assign61770_e99153 * var_t1_dn3)) / (var_t1 * var_t1)), ((((p.p2 * var_t0_dn4) * var_t1) - (assign61770_e99153 * var_t1_dn4)) / (var_t1 * var_t1)), ((((p.p2 * var_t0_dn5) * var_t1) - (assign61770_e99153 * var_t1_dn5)) / (var_t1 * var_t1)), ((((p.p2 * var_t0_dn6) * var_t1) - (assign61770_e99153 * var_t1_dn6)) / (var_t1 * var_t1)), ((((p.p2 * var_t0_dn7) * var_t1) - (assign61770_e99153 * var_t1_dn7)) / (var_t1 * var_t1)), ((((p.p2 * var_t0_dn8) * var_t1) - (assign61770_e99153 * var_t1_dn8)) / (var_t1 * var_t1)), ((((p.p2 * var_t0_dn9) * var_t1) - (assign61770_e99153 * var_t1_dn9)) / (var_t1 * var_t1)), ((((p.p2 * var_t0_dn10) * var_t1) - (assign61770_e99153 * var_t1_dn10)) / (var_t1 * var_t1)), ((((p.p2 * var_t0_dn11) * var_t1) - (assign61770_e99153 * var_t1_dn11)) / (var_t1 * var_t1)),)
    } else {
        (var_gbodyint, var_gbodyint_dn3, var_gbodyint_dn4, var_gbodyint_dn5, var_gbodyint_dn6, var_gbodyint_dn7, var_gbodyint_dn8, var_gbodyint_dn9, var_gbodyint_dn10, var_gbodyint_dn11,)
    }
};
        var_gbodyint = assign61770_e99157;
        var_gbodyint_dn3 = assign61770_e99157_d_n3;
        var_gbodyint_dn4 = assign61770_e99157_d_n4;
        var_gbodyint_dn5 = assign61770_e99157_d_n5;
        var_gbodyint_dn6 = assign61770_e99157_d_n6;
        var_gbodyint_dn7 = assign61770_e99157_d_n7;
        var_gbodyint_dn8 = assign61770_e99157_d_n8;
        var_gbodyint_dn9 = assign61770_e99157_d_n9;
        var_gbodyint_dn10 = assign61770_e99157_d_n10;
        var_gbodyint_dn11 = assign61770_e99157_d_n11;
        var_gbodyint_rv = 0.0;

        let (assign61780_e99170, assign61780_e99170_d_n3, assign61780_e99170_d_n4, assign61780_e99170_d_n5, assign61780_e99170_d_n6, assign61780_e99170_d_n7, assign61780_e99170_d_n8, assign61780_e99170_d_n9, assign61780_e99170_d_n10, assign61780_e99170_d_n11,) = {
    if (((var_guard900 == 0.0) && (var_guard901 == 0.0)) && (var_guard902 == 0.0)) {
        let assign61780_e99168: f64 = (1.0 / var_gbodyint);
        (assign61780_e99168, (-(var_gbodyint_dn3 / (var_gbodyint * var_gbodyint))), (-(var_gbodyint_dn4 / (var_gbodyint * var_gbodyint))), (-(var_gbodyint_dn5 / (var_gbodyint * var_gbodyint))), (-(var_gbodyint_dn6 / (var_gbodyint * var_gbodyint))), (-(var_gbodyint_dn7 / (var_gbodyint * var_gbodyint))), (-(var_gbodyint_dn8 / (var_gbodyint * var_gbodyint))), (-(var_gbodyint_dn9 / (var_gbodyint * var_gbodyint))), (-(var_gbodyint_dn10 / (var_gbodyint * var_gbodyint))), (-(var_gbodyint_dn11 / (var_gbodyint * var_gbodyint))),)
    } else {
        (var_rbodyint, var_rbodyint_dn3, var_rbodyint_dn4, var_rbodyint_dn5, var_rbodyint_dn6, var_rbodyint_dn7, var_rbodyint_dn8, var_rbodyint_dn9, var_rbodyint_dn10, var_rbodyint_dn11,)
    }
};
        var_rbodyint = assign61780_e99170;
        var_rbodyint_dn3 = assign61780_e99170_d_n3;
        var_rbodyint_dn4 = assign61780_e99170_d_n4;
        var_rbodyint_dn5 = assign61780_e99170_d_n5;
        var_rbodyint_dn6 = assign61780_e99170_d_n6;
        var_rbodyint_dn7 = assign61780_e99170_d_n7;
        var_rbodyint_dn8 = assign61780_e99170_d_n8;
        var_rbodyint_dn9 = assign61780_e99170_d_n9;
        var_rbodyint_dn10 = assign61780_e99170_d_n10;
        var_rbodyint_dn11 = assign61780_e99170_d_n11;
        var_rbodyint_rv = 0.0;

        let assign61790_e99173: f64 = if var_rbodyint < 0.001 { 1.0 } else { 0.0 };
        var_guard905 = assign61790_e99173;
        var_guard905_rv = 0.0;

        let assign61800_e99176: f64 = if var_rbodyext <= 0.001 { 1.0 } else { 0.0 };
        var_guard906 = assign61800_e99176;
        var_guard906_rv = 0.0;

        let (assign61810_e99193, assign61810_e99193_d_n3, assign61810_e99193_d_n4, assign61810_e99193_d_n5, assign61810_e99193_d_n6, assign61810_e99193_d_n7, assign61810_e99193_d_n8, assign61810_e99193_d_n9, assign61810_e99193_d_n10, assign61810_e99193_d_n11,) = {
    if (((((var_guard900 == 0.0) && (var_guard901 == 0.0)) && (var_guard902 == 0.0)) && (var_guard905 != 0.0)) && (var_guard906 != 0.0)) {
        let assign61810_e99191: f64 = (1.0 / 0.001);
        (assign61810_e99191, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11,)
    }
};
        var_t0 = assign61810_e99193;
        var_t0_dn3 = assign61810_e99193_d_n3;
        var_t0_dn4 = assign61810_e99193_d_n4;
        var_t0_dn5 = assign61810_e99193_d_n5;
        var_t0_dn6 = assign61810_e99193_d_n6;
        var_t0_dn7 = assign61810_e99193_d_n7;
        var_t0_dn8 = assign61810_e99193_d_n8;
        var_t0_dn9 = assign61810_e99193_d_n9;
        var_t0_dn10 = assign61810_e99193_d_n10;
        var_t0_dn11 = assign61810_e99193_d_n11;
        var_t0_rv = 0.0;

        let (assign61820_e99211, assign61820_e99211_d_n3, assign61820_e99211_d_n4, assign61820_e99211_d_n5, assign61820_e99211_d_n6, assign61820_e99211_d_n7, assign61820_e99211_d_n8, assign61820_e99211_d_n9, assign61820_e99211_d_n10, assign61820_e99211_d_n11,) = {
    if (((((var_guard900 == 0.0) && (var_guard901 == 0.0)) && (var_guard902 == 0.0)) && (var_guard905 != 0.0)) && (var_guard906 == 0.0)) {
        let assign61820_e99209: f64 = (1.0 / var_rbodyext);
        (assign61820_e99209, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11,)
    }
};
        var_t0 = assign61820_e99211;
        var_t0_dn3 = assign61820_e99211_d_n3;
        var_t0_dn4 = assign61820_e99211_d_n4;
        var_t0_dn5 = assign61820_e99211_d_n5;
        var_t0_dn6 = assign61820_e99211_d_n6;
        var_t0_dn7 = assign61820_e99211_d_n7;
        var_t0_dn8 = assign61820_e99211_d_n8;
        var_t0_dn9 = assign61820_e99211_d_n9;
        var_t0_dn10 = assign61820_e99211_d_n10;
        var_t0_dn11 = assign61820_e99211_d_n11;
        var_t0_rv = 0.0;

        let assign61870_e99254: f64 = if p.p1374 < 0.001 { 1.0 } else { 0.0 };
        var_guard908 = assign61870_e99254;
        var_guard908_rv = 0.0;

        let (assign61880_e99260, assign61880_e99260_d_n3, assign61880_e99260_d_n4, assign61880_e99260_d_n5, assign61880_e99260_d_n6, assign61880_e99260_d_n7, assign61880_e99260_d_n8, assign61880_e99260_d_n9, assign61880_e99260_d_n10, assign61880_e99260_d_n11,) = {
    if (var_guard908 != 0.0) {
        let assign61880_e99258: f64 = (1.0 / 0.001);
        (assign61880_e99258, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn11,)
    }
};
        var_t0 = assign61880_e99260;
        var_t0_dn3 = assign61880_e99260_d_n3;
        var_t0_dn4 = assign61880_e99260_d_n4;
        var_t0_dn5 = assign61880_e99260_d_n5;
        var_t0_dn6 = assign61880_e99260_d_n6;
        var_t0_dn7 = assign61880_e99260_d_n7;
        var_t0_dn8 = assign61880_e99260_d_n8;
        var_t0_dn9 = assign61880_e99260_d_n9;
        var_t0_dn10 = assign61880_e99260_d_n10;
        var_t0_dn11 = assign61880_e99260_d_n11;
        var_t0_rv = 0.0;

        *var_gbodyint_slot = var_gbodyint;
        *var_gbodyint_dn10_slot = var_gbodyint_dn10;
        *var_gbodyint_dn11_slot = var_gbodyint_dn11;
        *var_gbodyint_dn3_slot = var_gbodyint_dn3;
        *var_gbodyint_dn4_slot = var_gbodyint_dn4;
        *var_gbodyint_dn5_slot = var_gbodyint_dn5;
        *var_gbodyint_dn6_slot = var_gbodyint_dn6;
        *var_gbodyint_dn7_slot = var_gbodyint_dn7;
        *var_gbodyint_dn8_slot = var_gbodyint_dn8;
        *var_gbodyint_dn9_slot = var_gbodyint_dn9;
        *var_gbodyint_rv_slot = var_gbodyint_rv;
        *var_guard894_slot = var_guard894;
        *var_guard894_rv_slot = var_guard894_rv;
        *var_guard895_slot = var_guard895;
        *var_guard895_rv_slot = var_guard895_rv;
        *var_guard896_slot = var_guard896;
        *var_guard896_rv_slot = var_guard896_rv;
        *var_guard897_slot = var_guard897;
        *var_guard897_rv_slot = var_guard897_rv;
        *var_guard900_slot = var_guard900;
        *var_guard900_rv_slot = var_guard900_rv;
        *var_guard901_slot = var_guard901;
        *var_guard901_rv_slot = var_guard901_rv;
        *var_guard902_slot = var_guard902;
        *var_guard902_rv_slot = var_guard902_rv;
        *var_guard903_slot = var_guard903;
        *var_guard903_rv_slot = var_guard903_rv;
        *var_guard904_slot = var_guard904;
        *var_guard904_rv_slot = var_guard904_rv;
        *var_guard905_slot = var_guard905;
        *var_guard905_rv_slot = var_guard905_rv;
        *var_guard906_slot = var_guard906;
        *var_guard906_rv_slot = var_guard906_rv;
        *var_guard908_slot = var_guard908;
        *var_guard908_rv_slot = var_guard908_rv;
        *var_pdiss_slot = var_pdiss;
        *var_pdiss_dn0_slot = var_pdiss_dn0;
        *var_pdiss_dn10_slot = var_pdiss_dn10;
        *var_pdiss_dn11_slot = var_pdiss_dn11;
        *var_pdiss_dn2_slot = var_pdiss_dn2;
        *var_pdiss_dn3_slot = var_pdiss_dn3;
        *var_pdiss_dn4_slot = var_pdiss_dn4;
        *var_pdiss_dn5_slot = var_pdiss_dn5;
        *var_pdiss_dn6_slot = var_pdiss_dn6;
        *var_pdiss_dn7_slot = var_pdiss_dn7;
        *var_pdiss_dn8_slot = var_pdiss_dn8;
        *var_pdiss_dn9_slot = var_pdiss_dn9;
        *var_pdiss_rv_slot = var_pdiss_rv;
        *var_qb1_slot = var_qb1;
        *var_qb1_dn10_slot = var_qb1_dn10;
        *var_qb1_dn11_slot = var_qb1_dn11;
        *var_qb1_dn3_slot = var_qb1_dn3;
        *var_qb1_dn4_slot = var_qb1_dn4;
        *var_qb1_dn5_slot = var_qb1_dn5;
        *var_qb1_dn6_slot = var_qb1_dn6;
        *var_qb1_dn7_slot = var_qb1_dn7;
        *var_qb1_dn8_slot = var_qb1_dn8;
        *var_qb1_dn9_slot = var_qb1_dn9;
        *var_qb1_rv_slot = var_qb1_rv;
        *var_qbody_slot = var_qbody;
        *var_qbody_dn10_slot = var_qbody_dn10;
        *var_qbody_dn11_slot = var_qbody_dn11;
        *var_qbody_dn3_slot = var_qbody_dn3;
        *var_qbody_dn4_slot = var_qbody_dn4;
        *var_qbody_dn5_slot = var_qbody_dn5;
        *var_qbody_dn6_slot = var_qbody_dn6;
        *var_qbody_dn7_slot = var_qbody_dn7;
        *var_qbody_dn8_slot = var_qbody_dn8;
        *var_qbody_dn9_slot = var_qbody_dn9;
        *var_qbody_rv_slot = var_qbody_rv;
        *var_rbodyext_slot = var_rbodyext;
        *var_rbodyext_rv_slot = var_rbodyext_rv;
        *var_rbodyint_slot = var_rbodyint;
        *var_rbodyint_dn10_slot = var_rbodyint_dn10;
        *var_rbodyint_dn11_slot = var_rbodyint_dn11;
        *var_rbodyint_dn3_slot = var_rbodyint_dn3;
        *var_rbodyint_dn4_slot = var_rbodyint_dn4;
        *var_rbodyint_dn5_slot = var_rbodyint_dn5;
        *var_rbodyint_dn6_slot = var_rbodyint_dn6;
        *var_rbodyint_dn7_slot = var_rbodyint_dn7;
        *var_rbodyint_dn8_slot = var_rbodyint_dn8;
        *var_rbodyint_dn9_slot = var_rbodyint_dn9;
        *var_rbodyint_rv_slot = var_rbodyint_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_t0_rv_slot = var_t0_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t1_rv_slot = var_t1_rv;
        *var_ub_t_slot = var_ub_t;
        *var_ub_t_dn4_slot = var_ub_t_dn4;
        *var_ub_t_dn5_slot = var_ub_t_dn5;
        *var_ub_t_rv_slot = var_ub_t_rv;
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
        var_cox: f64,
        var_devsign: f64,
        var_guard492: f64,
        var_guard665: f64,
        var_guard666: f64,
        var_guard819: f64,
        var_guard836: f64,
        var_guard837: f64,
        var_issl: f64,
        var_issl_dn10: f64,
        var_issl_dn11: f64,
        var_issl_dn3: f64,
        var_issl_dn4: f64,
        var_issl_dn5: f64,
        var_issl_dn6: f64,
        var_issl_dn7: f64,
        var_issl_dn8: f64,
        var_issl_dn9: f64,
        var_leff: f64,
        var_mig: f64,
        var_mig_dn10: f64,
        var_mig_dn11: f64,
        var_mig_dn3: f64,
        var_mig_dn4: f64,
        var_mig_dn5: f64,
        var_mig_dn6: f64,
        var_mig_dn7: f64,
        var_mig_dn8: f64,
        var_mig_dn9: f64,
        var_sigvds: f64,
        var_sqid: f64,
        var_sqid_dn10: f64,
        var_sqid_dn11: f64,
        var_sqid_dn3: f64,
        var_sqid_dn4: f64,
        var_sqid_dn5: f64,
        var_sqid_dn6: f64,
        var_sqid_dn7: f64,
        var_sqid_dn8: f64,
        var_sqid_dn9: f64,
        var_sqig: f64,
        var_sqig_dn10: f64,
        var_sqig_dn11: f64,
        var_sqig_dn3: f64,
        var_sqig_dn4: f64,
        var_sqig_dn5: f64,
        var_sqig_dn6: f64,
        var_sqig_dn7: f64,
        var_sqig_dn8: f64,
        var_sqig_dn9: f64,
        var_weff: f64,
    ) {
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq6_e1526, eq6_e1526_d_n3, eq6_e1526_d_n4, eq6_e1526_d_n5, eq6_e1526_d_n6, eq6_e1526_d_n7, eq6_e1526_d_n8, eq6_e1526_d_n9, eq6_e1526_d_n10, eq6_e1526_d_n11, eq6_e1526_d_n13,) = {
    if ((var_guard492 != 0.0) && ((var_guard666 != 0.0) && (var_guard665 == 0.0))) {
        let eq6_e1522: f64 = (-var_sqig);
        let eq6_e1524: f64 = (eq6_e1522 * (nv13 - 0.0));
        let eq6_e1524_d_n3: f64 = ((-var_sqig_dn3) * (nv13 - 0.0));
        let eq6_e1524_d_n4: f64 = ((-var_sqig_dn4) * (nv13 - 0.0));
        let eq6_e1524_d_n5: f64 = ((-var_sqig_dn5) * (nv13 - 0.0));
        let eq6_e1524_d_n6: f64 = ((-var_sqig_dn6) * (nv13 - 0.0));
        let eq6_e1524_d_n7: f64 = ((-var_sqig_dn7) * (nv13 - 0.0));
        let eq6_e1524_d_n8: f64 = ((-var_sqig_dn8) * (nv13 - 0.0));
        let eq6_e1524_d_n9: f64 = ((-var_sqig_dn9) * (nv13 - 0.0));
        let eq6_e1524_d_n10: f64 = ((-var_sqig_dn10) * (nv13 - 0.0));
        let eq6_e1524_d_n11: f64 = ((-var_sqig_dn11) * (nv13 - 0.0));
        (eq6_e1524, eq6_e1524_d_n3, eq6_e1524_d_n4, eq6_e1524_d_n5, eq6_e1524_d_n6, eq6_e1524_d_n7, eq6_e1524_d_n8, eq6_e1524_d_n9, eq6_e1524_d_n10, eq6_e1524_d_n11, eq6_e1522,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e1526;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(12),
            None,
            multiplicity * (eq6_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 13],
            [multiplicity * (eq6_e1526_d_n3), multiplicity * (eq6_e1526_d_n4), multiplicity * (eq6_e1526_d_n5), multiplicity * (eq6_e1526_d_n6), multiplicity * (eq6_e1526_d_n7), multiplicity * (eq6_e1526_d_n8), multiplicity * (eq6_e1526_d_n9), multiplicity * (eq6_e1526_d_n10), multiplicity * (eq6_e1526_d_n11), multiplicity * (eq6_e1526_d_n13)],
            [],
            [],
            1.0,
        );
        let (eq7_e1546, eq7_e1546_d_n3, eq7_e1546_d_n4, eq7_e1546_d_n5, eq7_e1546_d_n6, eq7_e1546_d_n7, eq7_e1546_d_n8, eq7_e1546_d_n9, eq7_e1546_d_n10, eq7_e1546_d_n11, eq7_e1546_d_n12,) = {
    if ((var_guard492 != 0.0) && ((var_guard666 != 0.0) && (var_guard665 == 0.0))) {
        let eq7_e1535: f64 = (var_mig * var_cox);
        let eq7_e1535_d_n3: f64 = (var_mig_dn3 * var_cox);
        let eq7_e1535_d_n4: f64 = (var_mig_dn4 * var_cox);
        let eq7_e1535_d_n5: f64 = (var_mig_dn5 * var_cox);
        let eq7_e1535_d_n6: f64 = (var_mig_dn6 * var_cox);
        let eq7_e1535_d_n7: f64 = (var_mig_dn7 * var_cox);
        let eq7_e1535_d_n8: f64 = (var_mig_dn8 * var_cox);
        let eq7_e1535_d_n9: f64 = (var_mig_dn9 * var_cox);
        let eq7_e1535_d_n10: f64 = (var_mig_dn10 * var_cox);
        let eq7_e1535_d_n11: f64 = (var_mig_dn11 * var_cox);
        let eq7_e1537: f64 = (eq7_e1535 * var_weff);
        let eq7_e1537_d_n3: f64 = (eq7_e1535_d_n3 * var_weff);
        let eq7_e1537_d_n4: f64 = (eq7_e1535_d_n4 * var_weff);
        let eq7_e1537_d_n5: f64 = (eq7_e1535_d_n5 * var_weff);
        let eq7_e1537_d_n6: f64 = (eq7_e1535_d_n6 * var_weff);
        let eq7_e1537_d_n7: f64 = (eq7_e1535_d_n7 * var_weff);
        let eq7_e1537_d_n8: f64 = (eq7_e1535_d_n8 * var_weff);
        let eq7_e1537_d_n9: f64 = (eq7_e1535_d_n9 * var_weff);
        let eq7_e1537_d_n10: f64 = (eq7_e1535_d_n10 * var_weff);
        let eq7_e1537_d_n11: f64 = (eq7_e1535_d_n11 * var_weff);
        let eq7_e1539: f64 = (eq7_e1537 * p.p2);
        let eq7_e1539_d_n3: f64 = (eq7_e1537_d_n3 * p.p2);
        let eq7_e1539_d_n4: f64 = (eq7_e1537_d_n4 * p.p2);
        let eq7_e1539_d_n5: f64 = (eq7_e1537_d_n5 * p.p2);
        let eq7_e1539_d_n6: f64 = (eq7_e1537_d_n6 * p.p2);
        let eq7_e1539_d_n7: f64 = (eq7_e1537_d_n7 * p.p2);
        let eq7_e1539_d_n8: f64 = (eq7_e1537_d_n8 * p.p2);
        let eq7_e1539_d_n9: f64 = (eq7_e1537_d_n9 * p.p2);
        let eq7_e1539_d_n10: f64 = (eq7_e1537_d_n10 * p.p2);
        let eq7_e1539_d_n11: f64 = (eq7_e1537_d_n11 * p.p2);
        let eq7_e1541: f64 = (eq7_e1539 * var_leff);
        let eq7_e1541_d_n3: f64 = (eq7_e1539_d_n3 * var_leff);
        let eq7_e1541_d_n4: f64 = (eq7_e1539_d_n4 * var_leff);
        let eq7_e1541_d_n5: f64 = (eq7_e1539_d_n5 * var_leff);
        let eq7_e1541_d_n6: f64 = (eq7_e1539_d_n6 * var_leff);
        let eq7_e1541_d_n7: f64 = (eq7_e1539_d_n7 * var_leff);
        let eq7_e1541_d_n8: f64 = (eq7_e1539_d_n8 * var_leff);
        let eq7_e1541_d_n9: f64 = (eq7_e1539_d_n9 * var_leff);
        let eq7_e1541_d_n10: f64 = (eq7_e1539_d_n10 * var_leff);
        let eq7_e1541_d_n11: f64 = (eq7_e1539_d_n11 * var_leff);
        let eq7_e1543: f64 = (eq7_e1541 * (nv12 - 0.0));
        let eq7_e1543_d_n3: f64 = (eq7_e1541_d_n3 * (nv12 - 0.0));
        let eq7_e1543_d_n4: f64 = (eq7_e1541_d_n4 * (nv12 - 0.0));
        let eq7_e1543_d_n5: f64 = (eq7_e1541_d_n5 * (nv12 - 0.0));
        let eq7_e1543_d_n6: f64 = (eq7_e1541_d_n6 * (nv12 - 0.0));
        let eq7_e1543_d_n7: f64 = (eq7_e1541_d_n7 * (nv12 - 0.0));
        let eq7_e1543_d_n8: f64 = (eq7_e1541_d_n8 * (nv12 - 0.0));
        let eq7_e1543_d_n9: f64 = (eq7_e1541_d_n9 * (nv12 - 0.0));
        let eq7_e1543_d_n10: f64 = (eq7_e1541_d_n10 * (nv12 - 0.0));
        let eq7_e1543_d_n11: f64 = (eq7_e1541_d_n11 * (nv12 - 0.0));
        let eq7_e1544: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, eq7_e1543);
        (eq7_e1544, (eq7_e1543_d_n3 * ddt_scale), (eq7_e1543_d_n4 * ddt_scale), (eq7_e1543_d_n5 * ddt_scale), (eq7_e1543_d_n6 * ddt_scale), (eq7_e1543_d_n7 * ddt_scale), (eq7_e1543_d_n8 * ddt_scale), (eq7_e1543_d_n9 * ddt_scale), (eq7_e1543_d_n10 * ddt_scale), (eq7_e1543_d_n11 * ddt_scale), (eq7_e1541 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e1546;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(12),
            None,
            multiplicity * (eq7_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq7_e1546_d_n3), multiplicity * (eq7_e1546_d_n4), multiplicity * (eq7_e1546_d_n5), multiplicity * (eq7_e1546_d_n6), multiplicity * (eq7_e1546_d_n7), multiplicity * (eq7_e1546_d_n8), multiplicity * (eq7_e1546_d_n9), multiplicity * (eq7_e1546_d_n10), multiplicity * (eq7_e1546_d_n11), multiplicity * (eq7_e1546_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq9_e1574, eq9_e1574_d_n3, eq9_e1574_d_n4, eq9_e1574_d_n5, eq9_e1574_d_n6, eq9_e1574_d_n7, eq9_e1574_d_n8, eq9_e1574_d_n9, eq9_e1574_d_n10, eq9_e1574_d_n11, eq9_e1574_d_n13,) = {
    if ((var_guard492 != 0.0) && ((var_guard666 != 0.0) && (var_guard665 == 0.0))) {
        let eq9_e1572: f64 = (var_sqid * (nv13 - 0.0));
        let eq9_e1572_d_n3: f64 = (var_sqid_dn3 * (nv13 - 0.0));
        let eq9_e1572_d_n4: f64 = (var_sqid_dn4 * (nv13 - 0.0));
        let eq9_e1572_d_n5: f64 = (var_sqid_dn5 * (nv13 - 0.0));
        let eq9_e1572_d_n6: f64 = (var_sqid_dn6 * (nv13 - 0.0));
        let eq9_e1572_d_n7: f64 = (var_sqid_dn7 * (nv13 - 0.0));
        let eq9_e1572_d_n8: f64 = (var_sqid_dn8 * (nv13 - 0.0));
        let eq9_e1572_d_n9: f64 = (var_sqid_dn9 * (nv13 - 0.0));
        let eq9_e1572_d_n10: f64 = (var_sqid_dn10 * (nv13 - 0.0));
        let eq9_e1572_d_n11: f64 = (var_sqid_dn11 * (nv13 - 0.0));
        (eq9_e1572, eq9_e1572_d_n3, eq9_e1572_d_n4, eq9_e1572_d_n5, eq9_e1572_d_n6, eq9_e1572_d_n7, eq9_e1572_d_n8, eq9_e1572_d_n9, eq9_e1572_d_n10, eq9_e1572_d_n11, var_sqid,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e1574;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq9_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 13],
            [multiplicity * (eq9_e1574_d_n3), multiplicity * (eq9_e1574_d_n4), multiplicity * (eq9_e1574_d_n5), multiplicity * (eq9_e1574_d_n6), multiplicity * (eq9_e1574_d_n7), multiplicity * (eq9_e1574_d_n8), multiplicity * (eq9_e1574_d_n9), multiplicity * (eq9_e1574_d_n10), multiplicity * (eq9_e1574_d_n11), multiplicity * (eq9_e1574_d_n13)],
            [],
            [],
            1.0,
        );
        let (eq10_e1600, eq10_e1600_d_n3, eq10_e1600_d_n4, eq10_e1600_d_n5, eq10_e1600_d_n6, eq10_e1600_d_n7, eq10_e1600_d_n8, eq10_e1600_d_n9, eq10_e1600_d_n10, eq10_e1600_d_n11, eq10_e1600_d_n12,) = {
    if ((var_guard492 != 0.0) && ((var_guard666 != 0.0) && (var_guard665 == 0.0))) {
        let eq10_e1584: f64 = (1.0 + var_sigvds);
        let eq10_e1586: f64 = (eq10_e1584 * var_mig);
        let eq10_e1586_d_n3: f64 = (eq10_e1584 * var_mig_dn3);
        let eq10_e1586_d_n4: f64 = (eq10_e1584 * var_mig_dn4);
        let eq10_e1586_d_n5: f64 = (eq10_e1584 * var_mig_dn5);
        let eq10_e1586_d_n6: f64 = (eq10_e1584 * var_mig_dn6);
        let eq10_e1586_d_n7: f64 = (eq10_e1584 * var_mig_dn7);
        let eq10_e1586_d_n8: f64 = (eq10_e1584 * var_mig_dn8);
        let eq10_e1586_d_n9: f64 = (eq10_e1584 * var_mig_dn9);
        let eq10_e1586_d_n10: f64 = (eq10_e1584 * var_mig_dn10);
        let eq10_e1586_d_n11: f64 = (eq10_e1584 * var_mig_dn11);
        let eq10_e1588: f64 = (eq10_e1586 * var_cox);
        let eq10_e1588_d_n3: f64 = (eq10_e1586_d_n3 * var_cox);
        let eq10_e1588_d_n4: f64 = (eq10_e1586_d_n4 * var_cox);
        let eq10_e1588_d_n5: f64 = (eq10_e1586_d_n5 * var_cox);
        let eq10_e1588_d_n6: f64 = (eq10_e1586_d_n6 * var_cox);
        let eq10_e1588_d_n7: f64 = (eq10_e1586_d_n7 * var_cox);
        let eq10_e1588_d_n8: f64 = (eq10_e1586_d_n8 * var_cox);
        let eq10_e1588_d_n9: f64 = (eq10_e1586_d_n9 * var_cox);
        let eq10_e1588_d_n10: f64 = (eq10_e1586_d_n10 * var_cox);
        let eq10_e1588_d_n11: f64 = (eq10_e1586_d_n11 * var_cox);
        let eq10_e1590: f64 = (eq10_e1588 * var_weff);
        let eq10_e1590_d_n3: f64 = (eq10_e1588_d_n3 * var_weff);
        let eq10_e1590_d_n4: f64 = (eq10_e1588_d_n4 * var_weff);
        let eq10_e1590_d_n5: f64 = (eq10_e1588_d_n5 * var_weff);
        let eq10_e1590_d_n6: f64 = (eq10_e1588_d_n6 * var_weff);
        let eq10_e1590_d_n7: f64 = (eq10_e1588_d_n7 * var_weff);
        let eq10_e1590_d_n8: f64 = (eq10_e1588_d_n8 * var_weff);
        let eq10_e1590_d_n9: f64 = (eq10_e1588_d_n9 * var_weff);
        let eq10_e1590_d_n10: f64 = (eq10_e1588_d_n10 * var_weff);
        let eq10_e1590_d_n11: f64 = (eq10_e1588_d_n11 * var_weff);
        let eq10_e1592: f64 = (eq10_e1590 * p.p2);
        let eq10_e1592_d_n3: f64 = (eq10_e1590_d_n3 * p.p2);
        let eq10_e1592_d_n4: f64 = (eq10_e1590_d_n4 * p.p2);
        let eq10_e1592_d_n5: f64 = (eq10_e1590_d_n5 * p.p2);
        let eq10_e1592_d_n6: f64 = (eq10_e1590_d_n6 * p.p2);
        let eq10_e1592_d_n7: f64 = (eq10_e1590_d_n7 * p.p2);
        let eq10_e1592_d_n8: f64 = (eq10_e1590_d_n8 * p.p2);
        let eq10_e1592_d_n9: f64 = (eq10_e1590_d_n9 * p.p2);
        let eq10_e1592_d_n10: f64 = (eq10_e1590_d_n10 * p.p2);
        let eq10_e1592_d_n11: f64 = (eq10_e1590_d_n11 * p.p2);
        let eq10_e1594: f64 = (eq10_e1592 * var_leff);
        let eq10_e1594_d_n3: f64 = (eq10_e1592_d_n3 * var_leff);
        let eq10_e1594_d_n4: f64 = (eq10_e1592_d_n4 * var_leff);
        let eq10_e1594_d_n5: f64 = (eq10_e1592_d_n5 * var_leff);
        let eq10_e1594_d_n6: f64 = (eq10_e1592_d_n6 * var_leff);
        let eq10_e1594_d_n7: f64 = (eq10_e1592_d_n7 * var_leff);
        let eq10_e1594_d_n8: f64 = (eq10_e1592_d_n8 * var_leff);
        let eq10_e1594_d_n9: f64 = (eq10_e1592_d_n9 * var_leff);
        let eq10_e1594_d_n10: f64 = (eq10_e1592_d_n10 * var_leff);
        let eq10_e1594_d_n11: f64 = (eq10_e1592_d_n11 * var_leff);
        let eq10_e1596: f64 = (eq10_e1594 * (nv12 - 0.0));
        let eq10_e1596_d_n3: f64 = (eq10_e1594_d_n3 * (nv12 - 0.0));
        let eq10_e1596_d_n4: f64 = (eq10_e1594_d_n4 * (nv12 - 0.0));
        let eq10_e1596_d_n5: f64 = (eq10_e1594_d_n5 * (nv12 - 0.0));
        let eq10_e1596_d_n6: f64 = (eq10_e1594_d_n6 * (nv12 - 0.0));
        let eq10_e1596_d_n7: f64 = (eq10_e1594_d_n7 * (nv12 - 0.0));
        let eq10_e1596_d_n8: f64 = (eq10_e1594_d_n8 * (nv12 - 0.0));
        let eq10_e1596_d_n9: f64 = (eq10_e1594_d_n9 * (nv12 - 0.0));
        let eq10_e1596_d_n10: f64 = (eq10_e1594_d_n10 * (nv12 - 0.0));
        let eq10_e1596_d_n11: f64 = (eq10_e1594_d_n11 * (nv12 - 0.0));
        let eq10_e1597: f64 = (0.5 * eq10_e1596);
        let eq10_e1597_d_n3: f64 = (0.5 * eq10_e1596_d_n3);
        let eq10_e1597_d_n4: f64 = (0.5 * eq10_e1596_d_n4);
        let eq10_e1597_d_n5: f64 = (0.5 * eq10_e1596_d_n5);
        let eq10_e1597_d_n6: f64 = (0.5 * eq10_e1596_d_n6);
        let eq10_e1597_d_n7: f64 = (0.5 * eq10_e1596_d_n7);
        let eq10_e1597_d_n8: f64 = (0.5 * eq10_e1596_d_n8);
        let eq10_e1597_d_n9: f64 = (0.5 * eq10_e1596_d_n9);
        let eq10_e1597_d_n10: f64 = (0.5 * eq10_e1596_d_n10);
        let eq10_e1597_d_n11: f64 = (0.5 * eq10_e1596_d_n11);
        let eq10_e1597_d_n12: f64 = (0.5 * eq10_e1594);
        let eq10_e1598: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq10_e1597);
        (eq10_e1598, (eq10_e1597_d_n3 * ddt_scale), (eq10_e1597_d_n4 * ddt_scale), (eq10_e1597_d_n5 * ddt_scale), (eq10_e1597_d_n6 * ddt_scale), (eq10_e1597_d_n7 * ddt_scale), (eq10_e1597_d_n8 * ddt_scale), (eq10_e1597_d_n9 * ddt_scale), (eq10_e1597_d_n10 * ddt_scale), (eq10_e1597_d_n11 * ddt_scale), (eq10_e1597_d_n12 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_value: f64 = eq10_e1600;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(8),
            Some(7),
            multiplicity * (eq10_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq10_e1600_d_n3), multiplicity * (eq10_e1600_d_n4), multiplicity * (eq10_e1600_d_n5), multiplicity * (eq10_e1600_d_n6), multiplicity * (eq10_e1600_d_n7), multiplicity * (eq10_e1600_d_n8), multiplicity * (eq10_e1600_d_n9), multiplicity * (eq10_e1600_d_n10), multiplicity * (eq10_e1600_d_n11), multiplicity * (eq10_e1600_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq11_e1626, eq11_e1626_d_n3, eq11_e1626_d_n4, eq11_e1626_d_n5, eq11_e1626_d_n6, eq11_e1626_d_n7, eq11_e1626_d_n8, eq11_e1626_d_n9, eq11_e1626_d_n10, eq11_e1626_d_n11, eq11_e1626_d_n12,) = {
    if ((var_guard492 != 0.0) && ((var_guard666 != 0.0) && (var_guard665 == 0.0))) {
        let eq11_e1610: f64 = (1.0 - var_sigvds);
        let eq11_e1612: f64 = (eq11_e1610 * var_mig);
        let eq11_e1612_d_n3: f64 = (eq11_e1610 * var_mig_dn3);
        let eq11_e1612_d_n4: f64 = (eq11_e1610 * var_mig_dn4);
        let eq11_e1612_d_n5: f64 = (eq11_e1610 * var_mig_dn5);
        let eq11_e1612_d_n6: f64 = (eq11_e1610 * var_mig_dn6);
        let eq11_e1612_d_n7: f64 = (eq11_e1610 * var_mig_dn7);
        let eq11_e1612_d_n8: f64 = (eq11_e1610 * var_mig_dn8);
        let eq11_e1612_d_n9: f64 = (eq11_e1610 * var_mig_dn9);
        let eq11_e1612_d_n10: f64 = (eq11_e1610 * var_mig_dn10);
        let eq11_e1612_d_n11: f64 = (eq11_e1610 * var_mig_dn11);
        let eq11_e1614: f64 = (eq11_e1612 * var_cox);
        let eq11_e1614_d_n3: f64 = (eq11_e1612_d_n3 * var_cox);
        let eq11_e1614_d_n4: f64 = (eq11_e1612_d_n4 * var_cox);
        let eq11_e1614_d_n5: f64 = (eq11_e1612_d_n5 * var_cox);
        let eq11_e1614_d_n6: f64 = (eq11_e1612_d_n6 * var_cox);
        let eq11_e1614_d_n7: f64 = (eq11_e1612_d_n7 * var_cox);
        let eq11_e1614_d_n8: f64 = (eq11_e1612_d_n8 * var_cox);
        let eq11_e1614_d_n9: f64 = (eq11_e1612_d_n9 * var_cox);
        let eq11_e1614_d_n10: f64 = (eq11_e1612_d_n10 * var_cox);
        let eq11_e1614_d_n11: f64 = (eq11_e1612_d_n11 * var_cox);
        let eq11_e1616: f64 = (eq11_e1614 * var_weff);
        let eq11_e1616_d_n3: f64 = (eq11_e1614_d_n3 * var_weff);
        let eq11_e1616_d_n4: f64 = (eq11_e1614_d_n4 * var_weff);
        let eq11_e1616_d_n5: f64 = (eq11_e1614_d_n5 * var_weff);
        let eq11_e1616_d_n6: f64 = (eq11_e1614_d_n6 * var_weff);
        let eq11_e1616_d_n7: f64 = (eq11_e1614_d_n7 * var_weff);
        let eq11_e1616_d_n8: f64 = (eq11_e1614_d_n8 * var_weff);
        let eq11_e1616_d_n9: f64 = (eq11_e1614_d_n9 * var_weff);
        let eq11_e1616_d_n10: f64 = (eq11_e1614_d_n10 * var_weff);
        let eq11_e1616_d_n11: f64 = (eq11_e1614_d_n11 * var_weff);
        let eq11_e1618: f64 = (eq11_e1616 * p.p2);
        let eq11_e1618_d_n3: f64 = (eq11_e1616_d_n3 * p.p2);
        let eq11_e1618_d_n4: f64 = (eq11_e1616_d_n4 * p.p2);
        let eq11_e1618_d_n5: f64 = (eq11_e1616_d_n5 * p.p2);
        let eq11_e1618_d_n6: f64 = (eq11_e1616_d_n6 * p.p2);
        let eq11_e1618_d_n7: f64 = (eq11_e1616_d_n7 * p.p2);
        let eq11_e1618_d_n8: f64 = (eq11_e1616_d_n8 * p.p2);
        let eq11_e1618_d_n9: f64 = (eq11_e1616_d_n9 * p.p2);
        let eq11_e1618_d_n10: f64 = (eq11_e1616_d_n10 * p.p2);
        let eq11_e1618_d_n11: f64 = (eq11_e1616_d_n11 * p.p2);
        let eq11_e1620: f64 = (eq11_e1618 * var_leff);
        let eq11_e1620_d_n3: f64 = (eq11_e1618_d_n3 * var_leff);
        let eq11_e1620_d_n4: f64 = (eq11_e1618_d_n4 * var_leff);
        let eq11_e1620_d_n5: f64 = (eq11_e1618_d_n5 * var_leff);
        let eq11_e1620_d_n6: f64 = (eq11_e1618_d_n6 * var_leff);
        let eq11_e1620_d_n7: f64 = (eq11_e1618_d_n7 * var_leff);
        let eq11_e1620_d_n8: f64 = (eq11_e1618_d_n8 * var_leff);
        let eq11_e1620_d_n9: f64 = (eq11_e1618_d_n9 * var_leff);
        let eq11_e1620_d_n10: f64 = (eq11_e1618_d_n10 * var_leff);
        let eq11_e1620_d_n11: f64 = (eq11_e1618_d_n11 * var_leff);
        let eq11_e1622: f64 = (eq11_e1620 * (nv12 - 0.0));
        let eq11_e1622_d_n3: f64 = (eq11_e1620_d_n3 * (nv12 - 0.0));
        let eq11_e1622_d_n4: f64 = (eq11_e1620_d_n4 * (nv12 - 0.0));
        let eq11_e1622_d_n5: f64 = (eq11_e1620_d_n5 * (nv12 - 0.0));
        let eq11_e1622_d_n6: f64 = (eq11_e1620_d_n6 * (nv12 - 0.0));
        let eq11_e1622_d_n7: f64 = (eq11_e1620_d_n7 * (nv12 - 0.0));
        let eq11_e1622_d_n8: f64 = (eq11_e1620_d_n8 * (nv12 - 0.0));
        let eq11_e1622_d_n9: f64 = (eq11_e1620_d_n9 * (nv12 - 0.0));
        let eq11_e1622_d_n10: f64 = (eq11_e1620_d_n10 * (nv12 - 0.0));
        let eq11_e1622_d_n11: f64 = (eq11_e1620_d_n11 * (nv12 - 0.0));
        let eq11_e1623: f64 = (0.5 * eq11_e1622);
        let eq11_e1623_d_n3: f64 = (0.5 * eq11_e1622_d_n3);
        let eq11_e1623_d_n4: f64 = (0.5 * eq11_e1622_d_n4);
        let eq11_e1623_d_n5: f64 = (0.5 * eq11_e1622_d_n5);
        let eq11_e1623_d_n6: f64 = (0.5 * eq11_e1622_d_n6);
        let eq11_e1623_d_n7: f64 = (0.5 * eq11_e1622_d_n7);
        let eq11_e1623_d_n8: f64 = (0.5 * eq11_e1622_d_n8);
        let eq11_e1623_d_n9: f64 = (0.5 * eq11_e1622_d_n9);
        let eq11_e1623_d_n10: f64 = (0.5 * eq11_e1622_d_n10);
        let eq11_e1623_d_n11: f64 = (0.5 * eq11_e1622_d_n11);
        let eq11_e1623_d_n12: f64 = (0.5 * eq11_e1620);
        let eq11_e1624: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq11_e1623);
        (eq11_e1624, (eq11_e1623_d_n3 * ddt_scale), (eq11_e1623_d_n4 * ddt_scale), (eq11_e1623_d_n5 * ddt_scale), (eq11_e1623_d_n6 * ddt_scale), (eq11_e1623_d_n7 * ddt_scale), (eq11_e1623_d_n8 * ddt_scale), (eq11_e1623_d_n9 * ddt_scale), (eq11_e1623_d_n10 * ddt_scale), (eq11_e1623_d_n11 * ddt_scale), (eq11_e1623_d_n12 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e1626;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(8),
            Some(6),
            multiplicity * (eq11_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq11_e1626_d_n3), multiplicity * (eq11_e1626_d_n4), multiplicity * (eq11_e1626_d_n5), multiplicity * (eq11_e1626_d_n6), multiplicity * (eq11_e1626_d_n7), multiplicity * (eq11_e1626_d_n8), multiplicity * (eq11_e1626_d_n9), multiplicity * (eq11_e1626_d_n10), multiplicity * (eq11_e1626_d_n11), multiplicity * (eq11_e1626_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq17_e1686, eq17_e1686_d_n3, eq17_e1686_d_n4, eq17_e1686_d_n5, eq17_e1686_d_n6, eq17_e1686_d_n7, eq17_e1686_d_n8, eq17_e1686_d_n9, eq17_e1686_d_n10, eq17_e1686_d_n11,) = {
    if ((var_guard492 == 0.0) && (var_guard819 != 0.0)) {
        let eq17_e1684: f64 = (var_devsign * var_issl);
        let eq17_e1684_d_n3: f64 = (var_devsign * var_issl_dn3);
        let eq17_e1684_d_n4: f64 = (var_devsign * var_issl_dn4);
        let eq17_e1684_d_n5: f64 = (var_devsign * var_issl_dn5);
        let eq17_e1684_d_n6: f64 = (var_devsign * var_issl_dn6);
        let eq17_e1684_d_n7: f64 = (var_devsign * var_issl_dn7);
        let eq17_e1684_d_n8: f64 = (var_devsign * var_issl_dn8);
        let eq17_e1684_d_n9: f64 = (var_devsign * var_issl_dn9);
        let eq17_e1684_d_n10: f64 = (var_devsign * var_issl_dn10);
        let eq17_e1684_d_n11: f64 = (var_devsign * var_issl_dn11);
        (eq17_e1684, eq17_e1684_d_n3, eq17_e1684_d_n4, eq17_e1684_d_n5, eq17_e1684_d_n6, eq17_e1684_d_n7, eq17_e1684_d_n8, eq17_e1684_d_n9, eq17_e1684_d_n10, eq17_e1684_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq17_value: f64 = eq17_e1686;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq17_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (eq17_e1686_d_n3), multiplicity * (eq17_e1686_d_n4), multiplicity * (eq17_e1686_d_n5), multiplicity * (eq17_e1686_d_n6), multiplicity * (eq17_e1686_d_n7), multiplicity * (eq17_e1686_d_n8), multiplicity * (eq17_e1686_d_n9), multiplicity * (eq17_e1686_d_n10), multiplicity * (eq17_e1686_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq23_e1763, eq23_e1763_d_n3, eq23_e1763_d_n4, eq23_e1763_d_n5, eq23_e1763_d_n6, eq23_e1763_d_n7, eq23_e1763_d_n8, eq23_e1763_d_n9, eq23_e1763_d_n10, eq23_e1763_d_n11, eq23_e1763_d_n13,) = {
    if ((var_guard492 == 0.0) && ((var_guard837 != 0.0) && (var_guard836 == 0.0))) {
        let eq23_e1759: f64 = (-var_sqig);
        let eq23_e1761: f64 = (eq23_e1759 * (nv13 - 0.0));
        let eq23_e1761_d_n3: f64 = ((-var_sqig_dn3) * (nv13 - 0.0));
        let eq23_e1761_d_n4: f64 = ((-var_sqig_dn4) * (nv13 - 0.0));
        let eq23_e1761_d_n5: f64 = ((-var_sqig_dn5) * (nv13 - 0.0));
        let eq23_e1761_d_n6: f64 = ((-var_sqig_dn6) * (nv13 - 0.0));
        let eq23_e1761_d_n7: f64 = ((-var_sqig_dn7) * (nv13 - 0.0));
        let eq23_e1761_d_n8: f64 = ((-var_sqig_dn8) * (nv13 - 0.0));
        let eq23_e1761_d_n9: f64 = ((-var_sqig_dn9) * (nv13 - 0.0));
        let eq23_e1761_d_n10: f64 = ((-var_sqig_dn10) * (nv13 - 0.0));
        let eq23_e1761_d_n11: f64 = ((-var_sqig_dn11) * (nv13 - 0.0));
        (eq23_e1761, eq23_e1761_d_n3, eq23_e1761_d_n4, eq23_e1761_d_n5, eq23_e1761_d_n6, eq23_e1761_d_n7, eq23_e1761_d_n8, eq23_e1761_d_n9, eq23_e1761_d_n10, eq23_e1761_d_n11, eq23_e1759,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq23_value: f64 = eq23_e1763;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(12),
            None,
            multiplicity * (eq23_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 13],
            [multiplicity * (eq23_e1763_d_n3), multiplicity * (eq23_e1763_d_n4), multiplicity * (eq23_e1763_d_n5), multiplicity * (eq23_e1763_d_n6), multiplicity * (eq23_e1763_d_n7), multiplicity * (eq23_e1763_d_n8), multiplicity * (eq23_e1763_d_n9), multiplicity * (eq23_e1763_d_n10), multiplicity * (eq23_e1763_d_n11), multiplicity * (eq23_e1763_d_n13)],
            [],
            [],
            1.0,
        );
        let (eq24_e1784, eq24_e1784_d_n3, eq24_e1784_d_n4, eq24_e1784_d_n5, eq24_e1784_d_n6, eq24_e1784_d_n7, eq24_e1784_d_n8, eq24_e1784_d_n9, eq24_e1784_d_n10, eq24_e1784_d_n11, eq24_e1784_d_n12,) = {
    if ((var_guard492 == 0.0) && ((var_guard837 != 0.0) && (var_guard836 == 0.0))) {
        let eq24_e1773: f64 = (var_mig * var_cox);
        let eq24_e1773_d_n3: f64 = (var_mig_dn3 * var_cox);
        let eq24_e1773_d_n4: f64 = (var_mig_dn4 * var_cox);
        let eq24_e1773_d_n5: f64 = (var_mig_dn5 * var_cox);
        let eq24_e1773_d_n6: f64 = (var_mig_dn6 * var_cox);
        let eq24_e1773_d_n7: f64 = (var_mig_dn7 * var_cox);
        let eq24_e1773_d_n8: f64 = (var_mig_dn8 * var_cox);
        let eq24_e1773_d_n9: f64 = (var_mig_dn9 * var_cox);
        let eq24_e1773_d_n10: f64 = (var_mig_dn10 * var_cox);
        let eq24_e1773_d_n11: f64 = (var_mig_dn11 * var_cox);
        let eq24_e1775: f64 = (eq24_e1773 * var_weff);
        let eq24_e1775_d_n3: f64 = (eq24_e1773_d_n3 * var_weff);
        let eq24_e1775_d_n4: f64 = (eq24_e1773_d_n4 * var_weff);
        let eq24_e1775_d_n5: f64 = (eq24_e1773_d_n5 * var_weff);
        let eq24_e1775_d_n6: f64 = (eq24_e1773_d_n6 * var_weff);
        let eq24_e1775_d_n7: f64 = (eq24_e1773_d_n7 * var_weff);
        let eq24_e1775_d_n8: f64 = (eq24_e1773_d_n8 * var_weff);
        let eq24_e1775_d_n9: f64 = (eq24_e1773_d_n9 * var_weff);
        let eq24_e1775_d_n10: f64 = (eq24_e1773_d_n10 * var_weff);
        let eq24_e1775_d_n11: f64 = (eq24_e1773_d_n11 * var_weff);
        let eq24_e1777: f64 = (eq24_e1775 * p.p2);
        let eq24_e1777_d_n3: f64 = (eq24_e1775_d_n3 * p.p2);
        let eq24_e1777_d_n4: f64 = (eq24_e1775_d_n4 * p.p2);
        let eq24_e1777_d_n5: f64 = (eq24_e1775_d_n5 * p.p2);
        let eq24_e1777_d_n6: f64 = (eq24_e1775_d_n6 * p.p2);
        let eq24_e1777_d_n7: f64 = (eq24_e1775_d_n7 * p.p2);
        let eq24_e1777_d_n8: f64 = (eq24_e1775_d_n8 * p.p2);
        let eq24_e1777_d_n9: f64 = (eq24_e1775_d_n9 * p.p2);
        let eq24_e1777_d_n10: f64 = (eq24_e1775_d_n10 * p.p2);
        let eq24_e1777_d_n11: f64 = (eq24_e1775_d_n11 * p.p2);
        let eq24_e1779: f64 = (eq24_e1777 * var_leff);
        let eq24_e1779_d_n3: f64 = (eq24_e1777_d_n3 * var_leff);
        let eq24_e1779_d_n4: f64 = (eq24_e1777_d_n4 * var_leff);
        let eq24_e1779_d_n5: f64 = (eq24_e1777_d_n5 * var_leff);
        let eq24_e1779_d_n6: f64 = (eq24_e1777_d_n6 * var_leff);
        let eq24_e1779_d_n7: f64 = (eq24_e1777_d_n7 * var_leff);
        let eq24_e1779_d_n8: f64 = (eq24_e1777_d_n8 * var_leff);
        let eq24_e1779_d_n9: f64 = (eq24_e1777_d_n9 * var_leff);
        let eq24_e1779_d_n10: f64 = (eq24_e1777_d_n10 * var_leff);
        let eq24_e1779_d_n11: f64 = (eq24_e1777_d_n11 * var_leff);
        let eq24_e1781: f64 = (eq24_e1779 * (nv12 - 0.0));
        let eq24_e1781_d_n3: f64 = (eq24_e1779_d_n3 * (nv12 - 0.0));
        let eq24_e1781_d_n4: f64 = (eq24_e1779_d_n4 * (nv12 - 0.0));
        let eq24_e1781_d_n5: f64 = (eq24_e1779_d_n5 * (nv12 - 0.0));
        let eq24_e1781_d_n6: f64 = (eq24_e1779_d_n6 * (nv12 - 0.0));
        let eq24_e1781_d_n7: f64 = (eq24_e1779_d_n7 * (nv12 - 0.0));
        let eq24_e1781_d_n8: f64 = (eq24_e1779_d_n8 * (nv12 - 0.0));
        let eq24_e1781_d_n9: f64 = (eq24_e1779_d_n9 * (nv12 - 0.0));
        let eq24_e1781_d_n10: f64 = (eq24_e1779_d_n10 * (nv12 - 0.0));
        let eq24_e1781_d_n11: f64 = (eq24_e1779_d_n11 * (nv12 - 0.0));
        let eq24_e1782: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq24_e1781);
        (eq24_e1782, (eq24_e1781_d_n3 * ddt_scale), (eq24_e1781_d_n4 * ddt_scale), (eq24_e1781_d_n5 * ddt_scale), (eq24_e1781_d_n6 * ddt_scale), (eq24_e1781_d_n7 * ddt_scale), (eq24_e1781_d_n8 * ddt_scale), (eq24_e1781_d_n9 * ddt_scale), (eq24_e1781_d_n10 * ddt_scale), (eq24_e1781_d_n11 * ddt_scale), (eq24_e1779 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e1784;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(12),
            None,
            multiplicity * (eq24_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq24_e1784_d_n3), multiplicity * (eq24_e1784_d_n4), multiplicity * (eq24_e1784_d_n5), multiplicity * (eq24_e1784_d_n6), multiplicity * (eq24_e1784_d_n7), multiplicity * (eq24_e1784_d_n8), multiplicity * (eq24_e1784_d_n9), multiplicity * (eq24_e1784_d_n10), multiplicity * (eq24_e1784_d_n11), multiplicity * (eq24_e1784_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq26_e1814, eq26_e1814_d_n3, eq26_e1814_d_n4, eq26_e1814_d_n5, eq26_e1814_d_n6, eq26_e1814_d_n7, eq26_e1814_d_n8, eq26_e1814_d_n9, eq26_e1814_d_n10, eq26_e1814_d_n11, eq26_e1814_d_n13,) = {
    if ((var_guard492 == 0.0) && ((var_guard837 != 0.0) && (var_guard836 == 0.0))) {
        let eq26_e1812: f64 = (var_sqid * (nv13 - 0.0));
        let eq26_e1812_d_n3: f64 = (var_sqid_dn3 * (nv13 - 0.0));
        let eq26_e1812_d_n4: f64 = (var_sqid_dn4 * (nv13 - 0.0));
        let eq26_e1812_d_n5: f64 = (var_sqid_dn5 * (nv13 - 0.0));
        let eq26_e1812_d_n6: f64 = (var_sqid_dn6 * (nv13 - 0.0));
        let eq26_e1812_d_n7: f64 = (var_sqid_dn7 * (nv13 - 0.0));
        let eq26_e1812_d_n8: f64 = (var_sqid_dn8 * (nv13 - 0.0));
        let eq26_e1812_d_n9: f64 = (var_sqid_dn9 * (nv13 - 0.0));
        let eq26_e1812_d_n10: f64 = (var_sqid_dn10 * (nv13 - 0.0));
        let eq26_e1812_d_n11: f64 = (var_sqid_dn11 * (nv13 - 0.0));
        (eq26_e1812, eq26_e1812_d_n3, eq26_e1812_d_n4, eq26_e1812_d_n5, eq26_e1812_d_n6, eq26_e1812_d_n7, eq26_e1812_d_n8, eq26_e1812_d_n9, eq26_e1812_d_n10, eq26_e1812_d_n11, var_sqid,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq26_value: f64 = eq26_e1814;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq26_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 13],
            [multiplicity * (eq26_e1814_d_n3), multiplicity * (eq26_e1814_d_n4), multiplicity * (eq26_e1814_d_n5), multiplicity * (eq26_e1814_d_n6), multiplicity * (eq26_e1814_d_n7), multiplicity * (eq26_e1814_d_n8), multiplicity * (eq26_e1814_d_n9), multiplicity * (eq26_e1814_d_n10), multiplicity * (eq26_e1814_d_n11), multiplicity * (eq26_e1814_d_n13)],
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
        var_cox: f64,
        var_devsign: f64,
        var_guard492: f64,
        var_guard836: f64,
        var_guard837: f64,
        var_guard881: f64,
        var_guard882: f64,
        var_guard883: f64,
        var_ids: f64,
        var_ids_dn10: f64,
        var_ids_dn11: f64,
        var_ids_dn3: f64,
        var_ids_dn4: f64,
        var_ids_dn5: f64,
        var_ids_dn6: f64,
        var_ids_dn7: f64,
        var_ids_dn8: f64,
        var_ids_dn9: f64,
        var_ig_agbcp2: f64,
        var_ig_agbcp2_dn10: f64,
        var_ig_agbcp2_dn11: f64,
        var_ig_agbcp2_dn3: f64,
        var_ig_agbcp2_dn4: f64,
        var_ig_agbcp2_dn5: f64,
        var_ig_agbcp2_dn6: f64,
        var_ig_agbcp2_dn7: f64,
        var_ig_agbcp2_dn8: f64,
        var_ig_agbcp2_dn9: f64,
        var_igb_1: f64,
        var_igb_1_dn10: f64,
        var_igb_1_dn11: f64,
        var_igb_1_dn3: f64,
        var_igb_1_dn4: f64,
        var_igb_1_dn5: f64,
        var_igb_1_dn6: f64,
        var_igb_1_dn7: f64,
        var_igb_1_dn8: f64,
        var_igb_1_dn9: f64,
        var_igcd_1: f64,
        var_igcd_1_dn10: f64,
        var_igcd_1_dn11: f64,
        var_igcd_1_dn3: f64,
        var_igcd_1_dn4: f64,
        var_igcd_1_dn5: f64,
        var_igcd_1_dn6: f64,
        var_igcd_1_dn7: f64,
        var_igcd_1_dn8: f64,
        var_igcd_1_dn9: f64,
        var_igcs_1: f64,
        var_igcs_1_dn10: f64,
        var_igcs_1_dn11: f64,
        var_igcs_1_dn3: f64,
        var_igcs_1_dn4: f64,
        var_igcs_1_dn5: f64,
        var_igcs_1_dn6: f64,
        var_igcs_1_dn7: f64,
        var_igcs_1_dn8: f64,
        var_igcs_1_dn9: f64,
        var_igd_1: f64,
        var_igd_1_dn10: f64,
        var_igd_1_dn11: f64,
        var_igd_1_dn3: f64,
        var_igd_1_dn4: f64,
        var_igd_1_dn5: f64,
        var_igd_1_dn6: f64,
        var_igd_1_dn7: f64,
        var_igd_1_dn8: f64,
        var_igd_1_dn9: f64,
        var_igidl_1: f64,
        var_igidl_1_dn10: f64,
        var_igidl_1_dn11: f64,
        var_igidl_1_dn3: f64,
        var_igidl_1_dn4: f64,
        var_igidl_1_dn5: f64,
        var_igidl_1_dn6: f64,
        var_igidl_1_dn7: f64,
        var_igidl_1_dn8: f64,
        var_igidl_1_dn9: f64,
        var_igisl_1: f64,
        var_igisl_1_dn10: f64,
        var_igisl_1_dn11: f64,
        var_igisl_1_dn3: f64,
        var_igisl_1_dn4: f64,
        var_igisl_1_dn5: f64,
        var_igisl_1_dn6: f64,
        var_igisl_1_dn7: f64,
        var_igisl_1_dn8: f64,
        var_igisl_1_dn9: f64,
        var_igs_1: f64,
        var_igs_1_dn10: f64,
        var_igs_1_dn11: f64,
        var_igs_1_dn3: f64,
        var_igs_1_dn4: f64,
        var_igs_1_dn5: f64,
        var_igs_1_dn6: f64,
        var_igs_1_dn7: f64,
        var_igs_1_dn8: f64,
        var_igs_1_dn9: f64,
        var_isub: f64,
        var_isub_dn10: f64,
        var_isub_dn11: f64,
        var_isub_dn3: f64,
        var_isub_dn4: f64,
        var_isub_dn5: f64,
        var_isub_dn6: f64,
        var_isub_dn7: f64,
        var_isub_dn8: f64,
        var_isub_dn9: f64,
        var_leff: f64,
        var_mig: f64,
        var_mig_dn10: f64,
        var_mig_dn11: f64,
        var_mig_dn3: f64,
        var_mig_dn4: f64,
        var_mig_dn5: f64,
        var_mig_dn6: f64,
        var_mig_dn7: f64,
        var_mig_dn8: f64,
        var_mig_dn9: f64,
        var_qde: f64,
        var_qde_dn10: f64,
        var_qde_dn11: f64,
        var_qde_dn3: f64,
        var_qde_dn4: f64,
        var_qde_dn5: f64,
        var_qde_dn6: f64,
        var_qde_dn7: f64,
        var_qde_dn8: f64,
        var_qde_dn9: f64,
        var_qdiagbcp2_1: f64,
        var_qdiagbcp2_1_dn10: f64,
        var_qdiagbcp2_1_dn11: f64,
        var_qdiagbcp2_1_dn3: f64,
        var_qdiagbcp2_1_dn4: f64,
        var_qdiagbcp2_1_dn5: f64,
        var_qdiagbcp2_1_dn6: f64,
        var_qdiagbcp2_1_dn7: f64,
        var_qdiagbcp2_1_dn8: f64,
        var_qdiagbcp2_1_dn9: f64,
        var_qdim_1: f64,
        var_qdim_1_dn10: f64,
        var_qdim_1_dn11: f64,
        var_qdim_1_dn3: f64,
        var_qdim_1_dn4: f64,
        var_qdim_1_dn5: f64,
        var_qdim_1_dn6: f64,
        var_qdim_1_dn7: f64,
        var_qdim_1_dn8: f64,
        var_qdim_1_dn9: f64,
        var_qgiagbcp2_1: f64,
        var_qgiagbcp2_1_dn10: f64,
        var_qgiagbcp2_1_dn11: f64,
        var_qgiagbcp2_1_dn3: f64,
        var_qgiagbcp2_1_dn4: f64,
        var_qgiagbcp2_1_dn5: f64,
        var_qgiagbcp2_1_dn6: f64,
        var_qgiagbcp2_1_dn7: f64,
        var_qgiagbcp2_1_dn8: f64,
        var_qgiagbcp2_1_dn9: f64,
        var_qgim_1: f64,
        var_qgim_1_dn10: f64,
        var_qgim_1_dn11: f64,
        var_qgim_1_dn3: f64,
        var_qgim_1_dn4: f64,
        var_qgim_1_dn5: f64,
        var_qgim_1_dn6: f64,
        var_qgim_1_dn7: f64,
        var_qgim_1_dn8: f64,
        var_qgim_1_dn9: f64,
        var_qovd: f64,
        var_qovd_dn10: f64,
        var_qovd_dn11: f64,
        var_qovd_dn3: f64,
        var_qovd_dn4: f64,
        var_qovd_dn5: f64,
        var_qovd_dn6: f64,
        var_qovd_dn7: f64,
        var_qovd_dn8: f64,
        var_qovd_dn9: f64,
        var_qovs: f64,
        var_qovs_dn10: f64,
        var_qovs_dn11: f64,
        var_qovs_dn3: f64,
        var_qovs_dn4: f64,
        var_qovs_dn5: f64,
        var_qovs_dn6: f64,
        var_qovs_dn7: f64,
        var_qovs_dn8: f64,
        var_qovs_dn9: f64,
        var_qse: f64,
        var_qse_dn10: f64,
        var_qse_dn11: f64,
        var_qse_dn3: f64,
        var_qse_dn4: f64,
        var_qse_dn5: f64,
        var_qse_dn6: f64,
        var_qse_dn7: f64,
        var_qse_dn8: f64,
        var_qse_dn9: f64,
        var_qsiagbcp2_1: f64,
        var_qsiagbcp2_1_dn10: f64,
        var_qsiagbcp2_1_dn11: f64,
        var_qsiagbcp2_1_dn3: f64,
        var_qsiagbcp2_1_dn4: f64,
        var_qsiagbcp2_1_dn5: f64,
        var_qsiagbcp2_1_dn6: f64,
        var_qsiagbcp2_1_dn7: f64,
        var_qsiagbcp2_1_dn8: f64,
        var_qsiagbcp2_1_dn9: f64,
        var_qsim_1: f64,
        var_qsim_1_dn10: f64,
        var_qsim_1_dn11: f64,
        var_qsim_1_dn3: f64,
        var_qsim_1_dn4: f64,
        var_qsim_1_dn5: f64,
        var_qsim_1_dn6: f64,
        var_qsim_1_dn7: f64,
        var_qsim_1_dn8: f64,
        var_qsim_1_dn9: f64,
        var_qsub: f64,
        var_qsub_dn10: f64,
        var_qsub_dn11: f64,
        var_qsub_dn3: f64,
        var_qsub_dn4: f64,
        var_qsub_dn5: f64,
        var_qsub_dn6: f64,
        var_qsub_dn7: f64,
        var_qsub_dn8: f64,
        var_qsub_dn9: f64,
        var_sigvds: f64,
        var_weff: f64,
    ) {
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq27_e1841, eq27_e1841_d_n3, eq27_e1841_d_n4, eq27_e1841_d_n5, eq27_e1841_d_n6, eq27_e1841_d_n7, eq27_e1841_d_n8, eq27_e1841_d_n9, eq27_e1841_d_n10, eq27_e1841_d_n11, eq27_e1841_d_n12,) = {
    if ((var_guard492 == 0.0) && ((var_guard837 != 0.0) && (var_guard836 == 0.0))) {
        let eq27_e1825: f64 = (1.0 + var_sigvds);
        let eq27_e1827: f64 = (eq27_e1825 * var_mig);
        let eq27_e1827_d_n3: f64 = (eq27_e1825 * var_mig_dn3);
        let eq27_e1827_d_n4: f64 = (eq27_e1825 * var_mig_dn4);
        let eq27_e1827_d_n5: f64 = (eq27_e1825 * var_mig_dn5);
        let eq27_e1827_d_n6: f64 = (eq27_e1825 * var_mig_dn6);
        let eq27_e1827_d_n7: f64 = (eq27_e1825 * var_mig_dn7);
        let eq27_e1827_d_n8: f64 = (eq27_e1825 * var_mig_dn8);
        let eq27_e1827_d_n9: f64 = (eq27_e1825 * var_mig_dn9);
        let eq27_e1827_d_n10: f64 = (eq27_e1825 * var_mig_dn10);
        let eq27_e1827_d_n11: f64 = (eq27_e1825 * var_mig_dn11);
        let eq27_e1829: f64 = (eq27_e1827 * var_cox);
        let eq27_e1829_d_n3: f64 = (eq27_e1827_d_n3 * var_cox);
        let eq27_e1829_d_n4: f64 = (eq27_e1827_d_n4 * var_cox);
        let eq27_e1829_d_n5: f64 = (eq27_e1827_d_n5 * var_cox);
        let eq27_e1829_d_n6: f64 = (eq27_e1827_d_n6 * var_cox);
        let eq27_e1829_d_n7: f64 = (eq27_e1827_d_n7 * var_cox);
        let eq27_e1829_d_n8: f64 = (eq27_e1827_d_n8 * var_cox);
        let eq27_e1829_d_n9: f64 = (eq27_e1827_d_n9 * var_cox);
        let eq27_e1829_d_n10: f64 = (eq27_e1827_d_n10 * var_cox);
        let eq27_e1829_d_n11: f64 = (eq27_e1827_d_n11 * var_cox);
        let eq27_e1831: f64 = (eq27_e1829 * var_weff);
        let eq27_e1831_d_n3: f64 = (eq27_e1829_d_n3 * var_weff);
        let eq27_e1831_d_n4: f64 = (eq27_e1829_d_n4 * var_weff);
        let eq27_e1831_d_n5: f64 = (eq27_e1829_d_n5 * var_weff);
        let eq27_e1831_d_n6: f64 = (eq27_e1829_d_n6 * var_weff);
        let eq27_e1831_d_n7: f64 = (eq27_e1829_d_n7 * var_weff);
        let eq27_e1831_d_n8: f64 = (eq27_e1829_d_n8 * var_weff);
        let eq27_e1831_d_n9: f64 = (eq27_e1829_d_n9 * var_weff);
        let eq27_e1831_d_n10: f64 = (eq27_e1829_d_n10 * var_weff);
        let eq27_e1831_d_n11: f64 = (eq27_e1829_d_n11 * var_weff);
        let eq27_e1833: f64 = (eq27_e1831 * p.p2);
        let eq27_e1833_d_n3: f64 = (eq27_e1831_d_n3 * p.p2);
        let eq27_e1833_d_n4: f64 = (eq27_e1831_d_n4 * p.p2);
        let eq27_e1833_d_n5: f64 = (eq27_e1831_d_n5 * p.p2);
        let eq27_e1833_d_n6: f64 = (eq27_e1831_d_n6 * p.p2);
        let eq27_e1833_d_n7: f64 = (eq27_e1831_d_n7 * p.p2);
        let eq27_e1833_d_n8: f64 = (eq27_e1831_d_n8 * p.p2);
        let eq27_e1833_d_n9: f64 = (eq27_e1831_d_n9 * p.p2);
        let eq27_e1833_d_n10: f64 = (eq27_e1831_d_n10 * p.p2);
        let eq27_e1833_d_n11: f64 = (eq27_e1831_d_n11 * p.p2);
        let eq27_e1835: f64 = (eq27_e1833 * var_leff);
        let eq27_e1835_d_n3: f64 = (eq27_e1833_d_n3 * var_leff);
        let eq27_e1835_d_n4: f64 = (eq27_e1833_d_n4 * var_leff);
        let eq27_e1835_d_n5: f64 = (eq27_e1833_d_n5 * var_leff);
        let eq27_e1835_d_n6: f64 = (eq27_e1833_d_n6 * var_leff);
        let eq27_e1835_d_n7: f64 = (eq27_e1833_d_n7 * var_leff);
        let eq27_e1835_d_n8: f64 = (eq27_e1833_d_n8 * var_leff);
        let eq27_e1835_d_n9: f64 = (eq27_e1833_d_n9 * var_leff);
        let eq27_e1835_d_n10: f64 = (eq27_e1833_d_n10 * var_leff);
        let eq27_e1835_d_n11: f64 = (eq27_e1833_d_n11 * var_leff);
        let eq27_e1837: f64 = (eq27_e1835 * (nv12 - 0.0));
        let eq27_e1837_d_n3: f64 = (eq27_e1835_d_n3 * (nv12 - 0.0));
        let eq27_e1837_d_n4: f64 = (eq27_e1835_d_n4 * (nv12 - 0.0));
        let eq27_e1837_d_n5: f64 = (eq27_e1835_d_n5 * (nv12 - 0.0));
        let eq27_e1837_d_n6: f64 = (eq27_e1835_d_n6 * (nv12 - 0.0));
        let eq27_e1837_d_n7: f64 = (eq27_e1835_d_n7 * (nv12 - 0.0));
        let eq27_e1837_d_n8: f64 = (eq27_e1835_d_n8 * (nv12 - 0.0));
        let eq27_e1837_d_n9: f64 = (eq27_e1835_d_n9 * (nv12 - 0.0));
        let eq27_e1837_d_n10: f64 = (eq27_e1835_d_n10 * (nv12 - 0.0));
        let eq27_e1837_d_n11: f64 = (eq27_e1835_d_n11 * (nv12 - 0.0));
        let eq27_e1838: f64 = (0.5 * eq27_e1837);
        let eq27_e1838_d_n3: f64 = (0.5 * eq27_e1837_d_n3);
        let eq27_e1838_d_n4: f64 = (0.5 * eq27_e1837_d_n4);
        let eq27_e1838_d_n5: f64 = (0.5 * eq27_e1837_d_n5);
        let eq27_e1838_d_n6: f64 = (0.5 * eq27_e1837_d_n6);
        let eq27_e1838_d_n7: f64 = (0.5 * eq27_e1837_d_n7);
        let eq27_e1838_d_n8: f64 = (0.5 * eq27_e1837_d_n8);
        let eq27_e1838_d_n9: f64 = (0.5 * eq27_e1837_d_n9);
        let eq27_e1838_d_n10: f64 = (0.5 * eq27_e1837_d_n10);
        let eq27_e1838_d_n11: f64 = (0.5 * eq27_e1837_d_n11);
        let eq27_e1838_d_n12: f64 = (0.5 * eq27_e1835);
        let eq27_e1839: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq27_e1838);
        (eq27_e1839, (eq27_e1838_d_n3 * ddt_scale), (eq27_e1838_d_n4 * ddt_scale), (eq27_e1838_d_n5 * ddt_scale), (eq27_e1838_d_n6 * ddt_scale), (eq27_e1838_d_n7 * ddt_scale), (eq27_e1838_d_n8 * ddt_scale), (eq27_e1838_d_n9 * ddt_scale), (eq27_e1838_d_n10 * ddt_scale), (eq27_e1838_d_n11 * ddt_scale), (eq27_e1838_d_n12 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e1841;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(8),
            Some(7),
            multiplicity * (eq27_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq27_e1841_d_n3), multiplicity * (eq27_e1841_d_n4), multiplicity * (eq27_e1841_d_n5), multiplicity * (eq27_e1841_d_n6), multiplicity * (eq27_e1841_d_n7), multiplicity * (eq27_e1841_d_n8), multiplicity * (eq27_e1841_d_n9), multiplicity * (eq27_e1841_d_n10), multiplicity * (eq27_e1841_d_n11), multiplicity * (eq27_e1841_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq28_e1868, eq28_e1868_d_n3, eq28_e1868_d_n4, eq28_e1868_d_n5, eq28_e1868_d_n6, eq28_e1868_d_n7, eq28_e1868_d_n8, eq28_e1868_d_n9, eq28_e1868_d_n10, eq28_e1868_d_n11, eq28_e1868_d_n12,) = {
    if ((var_guard492 == 0.0) && ((var_guard837 != 0.0) && (var_guard836 == 0.0))) {
        let eq28_e1852: f64 = (1.0 - var_sigvds);
        let eq28_e1854: f64 = (eq28_e1852 * var_mig);
        let eq28_e1854_d_n3: f64 = (eq28_e1852 * var_mig_dn3);
        let eq28_e1854_d_n4: f64 = (eq28_e1852 * var_mig_dn4);
        let eq28_e1854_d_n5: f64 = (eq28_e1852 * var_mig_dn5);
        let eq28_e1854_d_n6: f64 = (eq28_e1852 * var_mig_dn6);
        let eq28_e1854_d_n7: f64 = (eq28_e1852 * var_mig_dn7);
        let eq28_e1854_d_n8: f64 = (eq28_e1852 * var_mig_dn8);
        let eq28_e1854_d_n9: f64 = (eq28_e1852 * var_mig_dn9);
        let eq28_e1854_d_n10: f64 = (eq28_e1852 * var_mig_dn10);
        let eq28_e1854_d_n11: f64 = (eq28_e1852 * var_mig_dn11);
        let eq28_e1856: f64 = (eq28_e1854 * var_cox);
        let eq28_e1856_d_n3: f64 = (eq28_e1854_d_n3 * var_cox);
        let eq28_e1856_d_n4: f64 = (eq28_e1854_d_n4 * var_cox);
        let eq28_e1856_d_n5: f64 = (eq28_e1854_d_n5 * var_cox);
        let eq28_e1856_d_n6: f64 = (eq28_e1854_d_n6 * var_cox);
        let eq28_e1856_d_n7: f64 = (eq28_e1854_d_n7 * var_cox);
        let eq28_e1856_d_n8: f64 = (eq28_e1854_d_n8 * var_cox);
        let eq28_e1856_d_n9: f64 = (eq28_e1854_d_n9 * var_cox);
        let eq28_e1856_d_n10: f64 = (eq28_e1854_d_n10 * var_cox);
        let eq28_e1856_d_n11: f64 = (eq28_e1854_d_n11 * var_cox);
        let eq28_e1858: f64 = (eq28_e1856 * var_weff);
        let eq28_e1858_d_n3: f64 = (eq28_e1856_d_n3 * var_weff);
        let eq28_e1858_d_n4: f64 = (eq28_e1856_d_n4 * var_weff);
        let eq28_e1858_d_n5: f64 = (eq28_e1856_d_n5 * var_weff);
        let eq28_e1858_d_n6: f64 = (eq28_e1856_d_n6 * var_weff);
        let eq28_e1858_d_n7: f64 = (eq28_e1856_d_n7 * var_weff);
        let eq28_e1858_d_n8: f64 = (eq28_e1856_d_n8 * var_weff);
        let eq28_e1858_d_n9: f64 = (eq28_e1856_d_n9 * var_weff);
        let eq28_e1858_d_n10: f64 = (eq28_e1856_d_n10 * var_weff);
        let eq28_e1858_d_n11: f64 = (eq28_e1856_d_n11 * var_weff);
        let eq28_e1860: f64 = (eq28_e1858 * p.p2);
        let eq28_e1860_d_n3: f64 = (eq28_e1858_d_n3 * p.p2);
        let eq28_e1860_d_n4: f64 = (eq28_e1858_d_n4 * p.p2);
        let eq28_e1860_d_n5: f64 = (eq28_e1858_d_n5 * p.p2);
        let eq28_e1860_d_n6: f64 = (eq28_e1858_d_n6 * p.p2);
        let eq28_e1860_d_n7: f64 = (eq28_e1858_d_n7 * p.p2);
        let eq28_e1860_d_n8: f64 = (eq28_e1858_d_n8 * p.p2);
        let eq28_e1860_d_n9: f64 = (eq28_e1858_d_n9 * p.p2);
        let eq28_e1860_d_n10: f64 = (eq28_e1858_d_n10 * p.p2);
        let eq28_e1860_d_n11: f64 = (eq28_e1858_d_n11 * p.p2);
        let eq28_e1862: f64 = (eq28_e1860 * var_leff);
        let eq28_e1862_d_n3: f64 = (eq28_e1860_d_n3 * var_leff);
        let eq28_e1862_d_n4: f64 = (eq28_e1860_d_n4 * var_leff);
        let eq28_e1862_d_n5: f64 = (eq28_e1860_d_n5 * var_leff);
        let eq28_e1862_d_n6: f64 = (eq28_e1860_d_n6 * var_leff);
        let eq28_e1862_d_n7: f64 = (eq28_e1860_d_n7 * var_leff);
        let eq28_e1862_d_n8: f64 = (eq28_e1860_d_n8 * var_leff);
        let eq28_e1862_d_n9: f64 = (eq28_e1860_d_n9 * var_leff);
        let eq28_e1862_d_n10: f64 = (eq28_e1860_d_n10 * var_leff);
        let eq28_e1862_d_n11: f64 = (eq28_e1860_d_n11 * var_leff);
        let eq28_e1864: f64 = (eq28_e1862 * (nv12 - 0.0));
        let eq28_e1864_d_n3: f64 = (eq28_e1862_d_n3 * (nv12 - 0.0));
        let eq28_e1864_d_n4: f64 = (eq28_e1862_d_n4 * (nv12 - 0.0));
        let eq28_e1864_d_n5: f64 = (eq28_e1862_d_n5 * (nv12 - 0.0));
        let eq28_e1864_d_n6: f64 = (eq28_e1862_d_n6 * (nv12 - 0.0));
        let eq28_e1864_d_n7: f64 = (eq28_e1862_d_n7 * (nv12 - 0.0));
        let eq28_e1864_d_n8: f64 = (eq28_e1862_d_n8 * (nv12 - 0.0));
        let eq28_e1864_d_n9: f64 = (eq28_e1862_d_n9 * (nv12 - 0.0));
        let eq28_e1864_d_n10: f64 = (eq28_e1862_d_n10 * (nv12 - 0.0));
        let eq28_e1864_d_n11: f64 = (eq28_e1862_d_n11 * (nv12 - 0.0));
        let eq28_e1865: f64 = (0.5 * eq28_e1864);
        let eq28_e1865_d_n3: f64 = (0.5 * eq28_e1864_d_n3);
        let eq28_e1865_d_n4: f64 = (0.5 * eq28_e1864_d_n4);
        let eq28_e1865_d_n5: f64 = (0.5 * eq28_e1864_d_n5);
        let eq28_e1865_d_n6: f64 = (0.5 * eq28_e1864_d_n6);
        let eq28_e1865_d_n7: f64 = (0.5 * eq28_e1864_d_n7);
        let eq28_e1865_d_n8: f64 = (0.5 * eq28_e1864_d_n8);
        let eq28_e1865_d_n9: f64 = (0.5 * eq28_e1864_d_n9);
        let eq28_e1865_d_n10: f64 = (0.5 * eq28_e1864_d_n10);
        let eq28_e1865_d_n11: f64 = (0.5 * eq28_e1864_d_n11);
        let eq28_e1865_d_n12: f64 = (0.5 * eq28_e1862);
        let eq28_e1866: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq28_e1865);
        (eq28_e1866, (eq28_e1865_d_n3 * ddt_scale), (eq28_e1865_d_n4 * ddt_scale), (eq28_e1865_d_n5 * ddt_scale), (eq28_e1865_d_n6 * ddt_scale), (eq28_e1865_d_n7 * ddt_scale), (eq28_e1865_d_n8 * ddt_scale), (eq28_e1865_d_n9 * ddt_scale), (eq28_e1865_d_n10 * ddt_scale), (eq28_e1865_d_n11 * ddt_scale), (eq28_e1865_d_n12 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_value: f64 = eq28_e1868;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(8),
            Some(6),
            multiplicity * (eq28_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq28_e1868_d_n3), multiplicity * (eq28_e1868_d_n4), multiplicity * (eq28_e1868_d_n5), multiplicity * (eq28_e1868_d_n6), multiplicity * (eq28_e1868_d_n7), multiplicity * (eq28_e1868_d_n8), multiplicity * (eq28_e1868_d_n9), multiplicity * (eq28_e1868_d_n10), multiplicity * (eq28_e1868_d_n11), multiplicity * (eq28_e1868_d_n12)],
            [],
            [],
            1.0,
        );
        let eq35_e1938: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, var_qgim_1);
        let eq35_value: f64 = eq35_e1938;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(8),
            Some(10),
            multiplicity * (eq35_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * ((var_qgim_1_dn3 * ddt_scale)), multiplicity * ((var_qgim_1_dn4 * ddt_scale)), multiplicity * ((var_qgim_1_dn5 * ddt_scale)), multiplicity * ((var_qgim_1_dn6 * ddt_scale)), multiplicity * ((var_qgim_1_dn7 * ddt_scale)), multiplicity * ((var_qgim_1_dn8 * ddt_scale)), multiplicity * ((var_qgim_1_dn9 * ddt_scale)), multiplicity * ((var_qgim_1_dn10 * ddt_scale)), multiplicity * ((var_qgim_1_dn11 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq36_e1940: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, var_qgiagbcp2_1);
        let eq36_value: f64 = eq36_e1940;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(8),
            Some(11),
            multiplicity * (eq36_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * ((var_qgiagbcp2_1_dn3 * ddt_scale)), multiplicity * ((var_qgiagbcp2_1_dn4 * ddt_scale)), multiplicity * ((var_qgiagbcp2_1_dn5 * ddt_scale)), multiplicity * ((var_qgiagbcp2_1_dn6 * ddt_scale)), multiplicity * ((var_qgiagbcp2_1_dn7 * ddt_scale)), multiplicity * ((var_qgiagbcp2_1_dn8 * ddt_scale)), multiplicity * ((var_qgiagbcp2_1_dn9 * ddt_scale)), multiplicity * ((var_qgiagbcp2_1_dn10 * ddt_scale)), multiplicity * ((var_qgiagbcp2_1_dn11 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq37_e1942: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, var_qsim_1);
        let eq37_value: f64 = eq37_e1942;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(7),
            Some(10),
            multiplicity * (eq37_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * ((var_qsim_1_dn3 * ddt_scale)), multiplicity * ((var_qsim_1_dn4 * ddt_scale)), multiplicity * ((var_qsim_1_dn5 * ddt_scale)), multiplicity * ((var_qsim_1_dn6 * ddt_scale)), multiplicity * ((var_qsim_1_dn7 * ddt_scale)), multiplicity * ((var_qsim_1_dn8 * ddt_scale)), multiplicity * ((var_qsim_1_dn9 * ddt_scale)), multiplicity * ((var_qsim_1_dn10 * ddt_scale)), multiplicity * ((var_qsim_1_dn11 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq38_e1944: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, var_qsiagbcp2_1);
        let eq38_value: f64 = eq38_e1944;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(7),
            Some(11),
            multiplicity * (eq38_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * ((var_qsiagbcp2_1_dn3 * ddt_scale)), multiplicity * ((var_qsiagbcp2_1_dn4 * ddt_scale)), multiplicity * ((var_qsiagbcp2_1_dn5 * ddt_scale)), multiplicity * ((var_qsiagbcp2_1_dn6 * ddt_scale)), multiplicity * ((var_qsiagbcp2_1_dn7 * ddt_scale)), multiplicity * ((var_qsiagbcp2_1_dn8 * ddt_scale)), multiplicity * ((var_qsiagbcp2_1_dn9 * ddt_scale)), multiplicity * ((var_qsiagbcp2_1_dn10 * ddt_scale)), multiplicity * ((var_qsiagbcp2_1_dn11 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq39_e1946: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, var_qdim_1);
        let eq39_value: f64 = eq39_e1946;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(6),
            Some(10),
            multiplicity * (eq39_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * ((var_qdim_1_dn3 * ddt_scale)), multiplicity * ((var_qdim_1_dn4 * ddt_scale)), multiplicity * ((var_qdim_1_dn5 * ddt_scale)), multiplicity * ((var_qdim_1_dn6 * ddt_scale)), multiplicity * ((var_qdim_1_dn7 * ddt_scale)), multiplicity * ((var_qdim_1_dn8 * ddt_scale)), multiplicity * ((var_qdim_1_dn9 * ddt_scale)), multiplicity * ((var_qdim_1_dn10 * ddt_scale)), multiplicity * ((var_qdim_1_dn11 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq40_e1948: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, var_qdiagbcp2_1);
        let eq40_value: f64 = eq40_e1948;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(6),
            Some(11),
            multiplicity * (eq40_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * ((var_qdiagbcp2_1_dn3 * ddt_scale)), multiplicity * ((var_qdiagbcp2_1_dn4 * ddt_scale)), multiplicity * ((var_qdiagbcp2_1_dn5 * ddt_scale)), multiplicity * ((var_qdiagbcp2_1_dn6 * ddt_scale)), multiplicity * ((var_qdiagbcp2_1_dn7 * ddt_scale)), multiplicity * ((var_qdiagbcp2_1_dn8 * ddt_scale)), multiplicity * ((var_qdiagbcp2_1_dn9 * ddt_scale)), multiplicity * ((var_qdiagbcp2_1_dn10 * ddt_scale)), multiplicity * ((var_qdiagbcp2_1_dn11 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq41_e1950: f64 = (-var_devsign);
        let eq41_e1952: f64 = (eq41_e1950 * var_qovs);
        let eq41_e1952_d_n3: f64 = (eq41_e1950 * var_qovs_dn3);
        let eq41_e1952_d_n4: f64 = (eq41_e1950 * var_qovs_dn4);
        let eq41_e1952_d_n5: f64 = (eq41_e1950 * var_qovs_dn5);
        let eq41_e1952_d_n6: f64 = (eq41_e1950 * var_qovs_dn6);
        let eq41_e1952_d_n7: f64 = (eq41_e1950 * var_qovs_dn7);
        let eq41_e1952_d_n8: f64 = (eq41_e1950 * var_qovs_dn8);
        let eq41_e1952_d_n9: f64 = (eq41_e1950 * var_qovs_dn9);
        let eq41_e1952_d_n10: f64 = (eq41_e1950 * var_qovs_dn10);
        let eq41_e1952_d_n11: f64 = (eq41_e1950 * var_qovs_dn11);
        let eq41_e1953: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, eq41_e1952);
        let eq41_value: f64 = eq41_e1953;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(9),
            Some(7),
            multiplicity * (eq41_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * ((eq41_e1952_d_n3 * ddt_scale)), multiplicity * ((eq41_e1952_d_n4 * ddt_scale)), multiplicity * ((eq41_e1952_d_n5 * ddt_scale)), multiplicity * ((eq41_e1952_d_n6 * ddt_scale)), multiplicity * ((eq41_e1952_d_n7 * ddt_scale)), multiplicity * ((eq41_e1952_d_n8 * ddt_scale)), multiplicity * ((eq41_e1952_d_n9 * ddt_scale)), multiplicity * ((eq41_e1952_d_n10 * ddt_scale)), multiplicity * ((eq41_e1952_d_n11 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq42_e1955: f64 = (-var_devsign);
        let eq42_e1957: f64 = (eq42_e1955 * var_qovd);
        let eq42_e1957_d_n3: f64 = (eq42_e1955 * var_qovd_dn3);
        let eq42_e1957_d_n4: f64 = (eq42_e1955 * var_qovd_dn4);
        let eq42_e1957_d_n5: f64 = (eq42_e1955 * var_qovd_dn5);
        let eq42_e1957_d_n6: f64 = (eq42_e1955 * var_qovd_dn6);
        let eq42_e1957_d_n7: f64 = (eq42_e1955 * var_qovd_dn7);
        let eq42_e1957_d_n8: f64 = (eq42_e1955 * var_qovd_dn8);
        let eq42_e1957_d_n9: f64 = (eq42_e1955 * var_qovd_dn9);
        let eq42_e1957_d_n10: f64 = (eq42_e1955 * var_qovd_dn10);
        let eq42_e1957_d_n11: f64 = (eq42_e1955 * var_qovd_dn11);
        let eq42_e1958: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, eq42_e1957);
        let eq42_value: f64 = eq42_e1958;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(9),
            Some(6),
            multiplicity * (eq42_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * ((eq42_e1957_d_n3 * ddt_scale)), multiplicity * ((eq42_e1957_d_n4 * ddt_scale)), multiplicity * ((eq42_e1957_d_n5 * ddt_scale)), multiplicity * ((eq42_e1957_d_n6 * ddt_scale)), multiplicity * ((eq42_e1957_d_n7 * ddt_scale)), multiplicity * ((eq42_e1957_d_n8 * ddt_scale)), multiplicity * ((eq42_e1957_d_n9 * ddt_scale)), multiplicity * ((eq42_e1957_d_n10 * ddt_scale)), multiplicity * ((eq42_e1957_d_n11 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq44_e1966: f64 = (var_devsign * var_ig_agbcp2);
        let eq44_e1966_d_n3: f64 = (var_devsign * var_ig_agbcp2_dn3);
        let eq44_e1966_d_n4: f64 = (var_devsign * var_ig_agbcp2_dn4);
        let eq44_e1966_d_n5: f64 = (var_devsign * var_ig_agbcp2_dn5);
        let eq44_e1966_d_n6: f64 = (var_devsign * var_ig_agbcp2_dn6);
        let eq44_e1966_d_n7: f64 = (var_devsign * var_ig_agbcp2_dn7);
        let eq44_e1966_d_n8: f64 = (var_devsign * var_ig_agbcp2_dn8);
        let eq44_e1966_d_n9: f64 = (var_devsign * var_ig_agbcp2_dn9);
        let eq44_e1966_d_n10: f64 = (var_devsign * var_ig_agbcp2_dn10);
        let eq44_e1966_d_n11: f64 = (var_devsign * var_ig_agbcp2_dn11);
        let eq44_value: f64 = eq44_e1966;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(8),
            Some(11),
            multiplicity * (eq44_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (eq44_e1966_d_n3), multiplicity * (eq44_e1966_d_n4), multiplicity * (eq44_e1966_d_n5), multiplicity * (eq44_e1966_d_n6), multiplicity * (eq44_e1966_d_n7), multiplicity * (eq44_e1966_d_n8), multiplicity * (eq44_e1966_d_n9), multiplicity * (eq44_e1966_d_n10), multiplicity * (eq44_e1966_d_n11)],
            [],
            [],
            1.0,
        );
        let eq45_e1969: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, var_qsub);
        let eq45_e1970: f64 = (var_devsign * eq45_e1969);
        let eq45_e1970_d_n3: f64 = (var_devsign * (var_qsub_dn3 * ddt_scale));
        let eq45_e1970_d_n4: f64 = (var_devsign * (var_qsub_dn4 * ddt_scale));
        let eq45_e1970_d_n5: f64 = (var_devsign * (var_qsub_dn5 * ddt_scale));
        let eq45_e1970_d_n6: f64 = (var_devsign * (var_qsub_dn6 * ddt_scale));
        let eq45_e1970_d_n7: f64 = (var_devsign * (var_qsub_dn7 * ddt_scale));
        let eq45_e1970_d_n8: f64 = (var_devsign * (var_qsub_dn8 * ddt_scale));
        let eq45_e1970_d_n9: f64 = (var_devsign * (var_qsub_dn9 * ddt_scale));
        let eq45_e1970_d_n10: f64 = (var_devsign * (var_qsub_dn10 * ddt_scale));
        let eq45_e1970_d_n11: f64 = (var_devsign * (var_qsub_dn11 * ddt_scale));
        let eq45_value: f64 = eq45_e1970;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(3),
            Some(10),
            multiplicity * (eq45_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (eq45_e1970_d_n3), multiplicity * (eq45_e1970_d_n4), multiplicity * (eq45_e1970_d_n5), multiplicity * (eq45_e1970_d_n6), multiplicity * (eq45_e1970_d_n7), multiplicity * (eq45_e1970_d_n8), multiplicity * (eq45_e1970_d_n9), multiplicity * (eq45_e1970_d_n10), multiplicity * (eq45_e1970_d_n11)],
            [],
            [],
            1.0,
        );
        let eq46_e1972: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, var_qde);
        let eq46_value: f64 = eq46_e1972;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(6),
            Some(3),
            multiplicity * (eq46_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * ((var_qde_dn3 * ddt_scale)), multiplicity * ((var_qde_dn4 * ddt_scale)), multiplicity * ((var_qde_dn5 * ddt_scale)), multiplicity * ((var_qde_dn6 * ddt_scale)), multiplicity * ((var_qde_dn7 * ddt_scale)), multiplicity * ((var_qde_dn8 * ddt_scale)), multiplicity * ((var_qde_dn9 * ddt_scale)), multiplicity * ((var_qde_dn10 * ddt_scale)), multiplicity * ((var_qde_dn11 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq47_e1974: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 17, var_qse);
        let eq47_value: f64 = eq47_e1974;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(7),
            Some(3),
            multiplicity * (eq47_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * ((var_qse_dn3 * ddt_scale)), multiplicity * ((var_qse_dn4 * ddt_scale)), multiplicity * ((var_qse_dn5 * ddt_scale)), multiplicity * ((var_qse_dn6 * ddt_scale)), multiplicity * ((var_qse_dn7 * ddt_scale)), multiplicity * ((var_qse_dn8 * ddt_scale)), multiplicity * ((var_qse_dn9 * ddt_scale)), multiplicity * ((var_qse_dn10 * ddt_scale)), multiplicity * ((var_qse_dn11 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq48_e1977: f64 = (var_devsign * var_sigvds);
        let eq48_e1979: f64 = (eq48_e1977 * var_ids);
        let eq48_e1979_d_n3: f64 = (eq48_e1977 * var_ids_dn3);
        let eq48_e1979_d_n4: f64 = (eq48_e1977 * var_ids_dn4);
        let eq48_e1979_d_n5: f64 = (eq48_e1977 * var_ids_dn5);
        let eq48_e1979_d_n6: f64 = (eq48_e1977 * var_ids_dn6);
        let eq48_e1979_d_n7: f64 = (eq48_e1977 * var_ids_dn7);
        let eq48_e1979_d_n8: f64 = (eq48_e1977 * var_ids_dn8);
        let eq48_e1979_d_n9: f64 = (eq48_e1977 * var_ids_dn9);
        let eq48_e1979_d_n10: f64 = (eq48_e1977 * var_ids_dn10);
        let eq48_e1979_d_n11: f64 = (eq48_e1977 * var_ids_dn11);
        let eq48_value: f64 = eq48_e1979;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq48_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (eq48_e1979_d_n3), multiplicity * (eq48_e1979_d_n4), multiplicity * (eq48_e1979_d_n5), multiplicity * (eq48_e1979_d_n6), multiplicity * (eq48_e1979_d_n7), multiplicity * (eq48_e1979_d_n8), multiplicity * (eq48_e1979_d_n9), multiplicity * (eq48_e1979_d_n10), multiplicity * (eq48_e1979_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq49_e1983, eq49_e1983_d_n3, eq49_e1983_d_n4, eq49_e1983_d_n5, eq49_e1983_d_n6, eq49_e1983_d_n7, eq49_e1983_d_n8, eq49_e1983_d_n9, eq49_e1983_d_n10, eq49_e1983_d_n11,) = {
    if (var_guard881 != 0.0) {
        (var_igb_1, var_igb_1_dn3, var_igb_1_dn4, var_igb_1_dn5, var_igb_1_dn6, var_igb_1_dn7, var_igb_1_dn8, var_igb_1_dn9, var_igb_1_dn10, var_igb_1_dn11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq49_value: f64 = eq49_e1983;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(8),
            Some(10),
            multiplicity * (eq49_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (eq49_e1983_d_n3), multiplicity * (eq49_e1983_d_n4), multiplicity * (eq49_e1983_d_n5), multiplicity * (eq49_e1983_d_n6), multiplicity * (eq49_e1983_d_n7), multiplicity * (eq49_e1983_d_n8), multiplicity * (eq49_e1983_d_n9), multiplicity * (eq49_e1983_d_n10), multiplicity * (eq49_e1983_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq50_e1989, eq50_e1989_d_n3, eq50_e1989_d_n4, eq50_e1989_d_n5, eq50_e1989_d_n6, eq50_e1989_d_n7, eq50_e1989_d_n8, eq50_e1989_d_n9, eq50_e1989_d_n10, eq50_e1989_d_n11,) = {
    if (var_guard882 != 0.0) {
        let eq50_e1987: f64 = (var_igs_1 + var_igcs_1);
        let eq50_e1987_d_n3: f64 = (var_igs_1_dn3 + var_igcs_1_dn3);
        let eq50_e1987_d_n4: f64 = (var_igs_1_dn4 + var_igcs_1_dn4);
        let eq50_e1987_d_n5: f64 = (var_igs_1_dn5 + var_igcs_1_dn5);
        let eq50_e1987_d_n6: f64 = (var_igs_1_dn6 + var_igcs_1_dn6);
        let eq50_e1987_d_n7: f64 = (var_igs_1_dn7 + var_igcs_1_dn7);
        let eq50_e1987_d_n8: f64 = (var_igs_1_dn8 + var_igcs_1_dn8);
        let eq50_e1987_d_n9: f64 = (var_igs_1_dn9 + var_igcs_1_dn9);
        let eq50_e1987_d_n10: f64 = (var_igs_1_dn10 + var_igcs_1_dn10);
        let eq50_e1987_d_n11: f64 = (var_igs_1_dn11 + var_igcs_1_dn11);
        (eq50_e1987, eq50_e1987_d_n3, eq50_e1987_d_n4, eq50_e1987_d_n5, eq50_e1987_d_n6, eq50_e1987_d_n7, eq50_e1987_d_n8, eq50_e1987_d_n9, eq50_e1987_d_n10, eq50_e1987_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_value: f64 = eq50_e1989;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(8),
            Some(7),
            multiplicity * (eq50_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (eq50_e1989_d_n3), multiplicity * (eq50_e1989_d_n4), multiplicity * (eq50_e1989_d_n5), multiplicity * (eq50_e1989_d_n6), multiplicity * (eq50_e1989_d_n7), multiplicity * (eq50_e1989_d_n8), multiplicity * (eq50_e1989_d_n9), multiplicity * (eq50_e1989_d_n10), multiplicity * (eq50_e1989_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq51_e1995, eq51_e1995_d_n3, eq51_e1995_d_n4, eq51_e1995_d_n5, eq51_e1995_d_n6, eq51_e1995_d_n7, eq51_e1995_d_n8, eq51_e1995_d_n9, eq51_e1995_d_n10, eq51_e1995_d_n11,) = {
    if (var_guard882 != 0.0) {
        let eq51_e1993: f64 = (var_igd_1 + var_igcd_1);
        let eq51_e1993_d_n3: f64 = (var_igd_1_dn3 + var_igcd_1_dn3);
        let eq51_e1993_d_n4: f64 = (var_igd_1_dn4 + var_igcd_1_dn4);
        let eq51_e1993_d_n5: f64 = (var_igd_1_dn5 + var_igcd_1_dn5);
        let eq51_e1993_d_n6: f64 = (var_igd_1_dn6 + var_igcd_1_dn6);
        let eq51_e1993_d_n7: f64 = (var_igd_1_dn7 + var_igcd_1_dn7);
        let eq51_e1993_d_n8: f64 = (var_igd_1_dn8 + var_igcd_1_dn8);
        let eq51_e1993_d_n9: f64 = (var_igd_1_dn9 + var_igcd_1_dn9);
        let eq51_e1993_d_n10: f64 = (var_igd_1_dn10 + var_igcd_1_dn10);
        let eq51_e1993_d_n11: f64 = (var_igd_1_dn11 + var_igcd_1_dn11);
        (eq51_e1993, eq51_e1993_d_n3, eq51_e1993_d_n4, eq51_e1993_d_n5, eq51_e1993_d_n6, eq51_e1993_d_n7, eq51_e1993_d_n8, eq51_e1993_d_n9, eq51_e1993_d_n10, eq51_e1993_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e1995;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(8),
            Some(6),
            multiplicity * (eq51_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (eq51_e1995_d_n3), multiplicity * (eq51_e1995_d_n4), multiplicity * (eq51_e1995_d_n5), multiplicity * (eq51_e1995_d_n6), multiplicity * (eq51_e1995_d_n7), multiplicity * (eq51_e1995_d_n8), multiplicity * (eq51_e1995_d_n9), multiplicity * (eq51_e1995_d_n10), multiplicity * (eq51_e1995_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq52_e2001, eq52_e2001_d_n3, eq52_e2001_d_n4, eq52_e2001_d_n5, eq52_e2001_d_n6, eq52_e2001_d_n7, eq52_e2001_d_n8, eq52_e2001_d_n9, eq52_e2001_d_n10, eq52_e2001_d_n11,) = {
    if (var_guard883 != 0.0) {
        let eq52_e1999: f64 = (var_isub + var_igidl_1);
        let eq52_e1999_d_n3: f64 = (var_isub_dn3 + var_igidl_1_dn3);
        let eq52_e1999_d_n4: f64 = (var_isub_dn4 + var_igidl_1_dn4);
        let eq52_e1999_d_n5: f64 = (var_isub_dn5 + var_igidl_1_dn5);
        let eq52_e1999_d_n6: f64 = (var_isub_dn6 + var_igidl_1_dn6);
        let eq52_e1999_d_n7: f64 = (var_isub_dn7 + var_igidl_1_dn7);
        let eq52_e1999_d_n8: f64 = (var_isub_dn8 + var_igidl_1_dn8);
        let eq52_e1999_d_n9: f64 = (var_isub_dn9 + var_igidl_1_dn9);
        let eq52_e1999_d_n10: f64 = (var_isub_dn10 + var_igidl_1_dn10);
        let eq52_e1999_d_n11: f64 = (var_isub_dn11 + var_igidl_1_dn11);
        (eq52_e1999, eq52_e1999_d_n3, eq52_e1999_d_n4, eq52_e1999_d_n5, eq52_e1999_d_n6, eq52_e1999_d_n7, eq52_e1999_d_n8, eq52_e1999_d_n9, eq52_e1999_d_n10, eq52_e1999_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq52_value: f64 = eq52_e2001;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(6),
            Some(10),
            multiplicity * (eq52_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (eq52_e2001_d_n3), multiplicity * (eq52_e2001_d_n4), multiplicity * (eq52_e2001_d_n5), multiplicity * (eq52_e2001_d_n6), multiplicity * (eq52_e2001_d_n7), multiplicity * (eq52_e2001_d_n8), multiplicity * (eq52_e2001_d_n9), multiplicity * (eq52_e2001_d_n10), multiplicity * (eq52_e2001_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq53_e2005, eq53_e2005_d_n3, eq53_e2005_d_n4, eq53_e2005_d_n5, eq53_e2005_d_n6, eq53_e2005_d_n7, eq53_e2005_d_n8, eq53_e2005_d_n9, eq53_e2005_d_n10, eq53_e2005_d_n11,) = {
    if (var_guard883 != 0.0) {
        (var_igisl_1, var_igisl_1_dn3, var_igisl_1_dn4, var_igisl_1_dn5, var_igisl_1_dn6, var_igisl_1_dn7, var_igisl_1_dn8, var_igisl_1_dn9, var_igisl_1_dn10, var_igisl_1_dn11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_value: f64 = eq53_e2005;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(7),
            Some(10),
            multiplicity * (eq53_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (eq53_e2005_d_n3), multiplicity * (eq53_e2005_d_n4), multiplicity * (eq53_e2005_d_n5), multiplicity * (eq53_e2005_d_n6), multiplicity * (eq53_e2005_d_n7), multiplicity * (eq53_e2005_d_n8), multiplicity * (eq53_e2005_d_n9), multiplicity * (eq53_e2005_d_n10), multiplicity * (eq53_e2005_d_n11)],
            [],
            [],
            1.0,
        );
    }

    pub(super) fn stamp_transient_equations_block_2(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
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
        var_cth: f64,
        var_deltemp1: f64,
        var_deltemp1_dn4: f64,
        var_deltemp1_dn5: f64,
        var_devsign: f64,
        var_gbody: f64,
        var_gbody_dn10: f64,
        var_gbody_dn11: f64,
        var_gbody_dn3: f64,
        var_gbody_dn4: f64,
        var_gbody_dn5: f64,
        var_gbody_dn6: f64,
        var_gbody_dn7: f64,
        var_gbody_dn8: f64,
        var_gbody_dn9: f64,
        var_gbodyagbcp2: f64,
        var_gbodyagbcp2_dn10: f64,
        var_gbodyagbcp2_dn11: f64,
        var_gbodyagbcp2_dn3: f64,
        var_gbodyagbcp2_dn4: f64,
        var_gbodyagbcp2_dn5: f64,
        var_gbodyagbcp2_dn6: f64,
        var_gbodyagbcp2_dn7: f64,
        var_gbodyagbcp2_dn8: f64,
        var_gbodyagbcp2_dn9: f64,
        var_gcrg: f64,
        var_gcrg_dn10: f64,
        var_gcrg_dn11: f64,
        var_gcrg_dn3: f64,
        var_gcrg_dn4: f64,
        var_gcrg_dn5: f64,
        var_gcrg_dn6: f64,
        var_gcrg_dn7: f64,
        var_gcrg_dn8: f64,
        var_gcrg_dn9: f64,
        var_gdpr: f64,
        var_gdpr_dn10: f64,
        var_gdpr_dn11: f64,
        var_gdpr_dn3: f64,
        var_gdpr_dn4: f64,
        var_gdpr_dn5: f64,
        var_gdpr_dn6: f64,
        var_gdpr_dn7: f64,
        var_gdpr_dn8: f64,
        var_gdpr_dn9: f64,
        var_ggate: f64,
        var_ggate_dn10: f64,
        var_ggate_dn11: f64,
        var_ggate_dn3: f64,
        var_ggate_dn4: f64,
        var_ggate_dn5: f64,
        var_ggate_dn6: f64,
        var_ggate_dn7: f64,
        var_ggate_dn8: f64,
        var_ggate_dn9: f64,
        var_gmin: f64,
        var_gspr: f64,
        var_gspr_dn10: f64,
        var_gspr_dn11: f64,
        var_gspr_dn3: f64,
        var_gspr_dn4: f64,
        var_gspr_dn5: f64,
        var_gspr_dn6: f64,
        var_gspr_dn7: f64,
        var_gspr_dn8: f64,
        var_gspr_dn9: f64,
        var_gth: f64,
        var_guard883: f64,
        var_guard884: f64,
        var_guard888: f64,
        var_guard890: f64,
        var_guard892: f64,
        var_guard893: f64,
        var_guard896: f64,
        var_guard897: f64,
        var_guard909: f64,
        var_guard910: f64,
        var_ibd: f64,
        var_ibd_dn10: f64,
        var_ibd_dn11: f64,
        var_ibd_dn3: f64,
        var_ibd_dn4: f64,
        var_ibd_dn5: f64,
        var_ibd_dn6: f64,
        var_ibd_dn7: f64,
        var_ibd_dn8: f64,
        var_ibd_dn9: f64,
        var_ibs: f64,
        var_ibs_dn10: f64,
        var_ibs_dn11: f64,
        var_ibs_dn3: f64,
        var_ibs_dn4: f64,
        var_ibs_dn5: f64,
        var_ibs_dn6: f64,
        var_ibs_dn7: f64,
        var_ibs_dn8: f64,
        var_ibs_dn9: f64,
        var_igidl_1: f64,
        var_igidl_1_dn10: f64,
        var_igidl_1_dn11: f64,
        var_igidl_1_dn3: f64,
        var_igidl_1_dn4: f64,
        var_igidl_1_dn5: f64,
        var_igidl_1_dn6: f64,
        var_igidl_1_dn7: f64,
        var_igidl_1_dn8: f64,
        var_igidl_1_dn9: f64,
        var_igisl_1: f64,
        var_igisl_1_dn10: f64,
        var_igisl_1_dn11: f64,
        var_igisl_1_dn3: f64,
        var_igisl_1_dn4: f64,
        var_igisl_1_dn5: f64,
        var_igisl_1_dn6: f64,
        var_igisl_1_dn7: f64,
        var_igisl_1_dn8: f64,
        var_igisl_1_dn9: f64,
        var_isub: f64,
        var_isub_dn10: f64,
        var_isub_dn11: f64,
        var_isub_dn3: f64,
        var_isub_dn4: f64,
        var_isub_dn5: f64,
        var_isub_dn6: f64,
        var_isub_dn7: f64,
        var_isub_dn8: f64,
        var_isub_dn9: f64,
        var_pdiss: f64,
        var_pdiss_dn0: f64,
        var_pdiss_dn10: f64,
        var_pdiss_dn11: f64,
        var_pdiss_dn2: f64,
        var_pdiss_dn3: f64,
        var_pdiss_dn4: f64,
        var_pdiss_dn5: f64,
        var_pdiss_dn6: f64,
        var_pdiss_dn7: f64,
        var_pdiss_dn8: f64,
        var_pdiss_dn9: f64,
        var_qbdj: f64,
        var_qbdj_dn10: f64,
        var_qbdj_dn11: f64,
        var_qbdj_dn3: f64,
        var_qbdj_dn4: f64,
        var_qbdj_dn5: f64,
        var_qbdj_dn6: f64,
        var_qbdj_dn7: f64,
        var_qbdj_dn8: f64,
        var_qbdj_dn9: f64,
        var_qbsj: f64,
        var_qbsj_dn10: f64,
        var_qbsj_dn11: f64,
        var_qbsj_dn3: f64,
        var_qbsj_dn4: f64,
        var_qbsj_dn5: f64,
        var_qbsj_dn6: f64,
        var_qbsj_dn7: f64,
        var_qbsj_dn8: f64,
        var_qbsj_dn9: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let (eq54_e2010, eq54_e2010_d_n3, eq54_e2010_d_n4, eq54_e2010_d_n5, eq54_e2010_d_n6, eq54_e2010_d_n7, eq54_e2010_d_n8, eq54_e2010_d_n9, eq54_e2010_d_n10, eq54_e2010_d_n11,) = {
    if (var_guard883 == 0.0) {
        (var_igidl_1, var_igidl_1_dn3, var_igidl_1_dn4, var_igidl_1_dn5, var_igidl_1_dn6, var_igidl_1_dn7, var_igidl_1_dn8, var_igidl_1_dn9, var_igidl_1_dn10, var_igidl_1_dn11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq54_value: f64 = eq54_e2010;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(6),
            Some(10),
            multiplicity * (eq54_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (eq54_e2010_d_n3), multiplicity * (eq54_e2010_d_n4), multiplicity * (eq54_e2010_d_n5), multiplicity * (eq54_e2010_d_n6), multiplicity * (eq54_e2010_d_n7), multiplicity * (eq54_e2010_d_n8), multiplicity * (eq54_e2010_d_n9), multiplicity * (eq54_e2010_d_n10), multiplicity * (eq54_e2010_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq55_e2017, eq55_e2017_d_n3, eq55_e2017_d_n4, eq55_e2017_d_n5, eq55_e2017_d_n6, eq55_e2017_d_n7, eq55_e2017_d_n8, eq55_e2017_d_n9, eq55_e2017_d_n10, eq55_e2017_d_n11,) = {
    if (var_guard883 == 0.0) {
        let eq55_e2015: f64 = (var_isub + var_igisl_1);
        let eq55_e2015_d_n3: f64 = (var_isub_dn3 + var_igisl_1_dn3);
        let eq55_e2015_d_n4: f64 = (var_isub_dn4 + var_igisl_1_dn4);
        let eq55_e2015_d_n5: f64 = (var_isub_dn5 + var_igisl_1_dn5);
        let eq55_e2015_d_n6: f64 = (var_isub_dn6 + var_igisl_1_dn6);
        let eq55_e2015_d_n7: f64 = (var_isub_dn7 + var_igisl_1_dn7);
        let eq55_e2015_d_n8: f64 = (var_isub_dn8 + var_igisl_1_dn8);
        let eq55_e2015_d_n9: f64 = (var_isub_dn9 + var_igisl_1_dn9);
        let eq55_e2015_d_n10: f64 = (var_isub_dn10 + var_igisl_1_dn10);
        let eq55_e2015_d_n11: f64 = (var_isub_dn11 + var_igisl_1_dn11);
        (eq55_e2015, eq55_e2015_d_n3, eq55_e2015_d_n4, eq55_e2015_d_n5, eq55_e2015_d_n6, eq55_e2015_d_n7, eq55_e2015_d_n8, eq55_e2015_d_n9, eq55_e2015_d_n10, eq55_e2015_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e2017;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(7),
            Some(10),
            multiplicity * (eq55_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (eq55_e2017_d_n3), multiplicity * (eq55_e2017_d_n4), multiplicity * (eq55_e2017_d_n5), multiplicity * (eq55_e2017_d_n6), multiplicity * (eq55_e2017_d_n7), multiplicity * (eq55_e2017_d_n8), multiplicity * (eq55_e2017_d_n9), multiplicity * (eq55_e2017_d_n10), multiplicity * (eq55_e2017_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq57_e2028, eq57_e2028_d_n1, eq57_e2028_d_n3, eq57_e2028_d_n4, eq57_e2028_d_n5, eq57_e2028_d_n6, eq57_e2028_d_n7, eq57_e2028_d_n8, eq57_e2028_d_n9, eq57_e2028_d_n10, eq57_e2028_d_n11,) = {
    if (var_guard884 == 0.0) {
        let eq57_e2026: f64 = ((nv1 - nv9) * var_ggate);
        let eq57_e2026_d_n3: f64 = ((nv1 - nv9) * var_ggate_dn3);
        let eq57_e2026_d_n4: f64 = ((nv1 - nv9) * var_ggate_dn4);
        let eq57_e2026_d_n5: f64 = ((nv1 - nv9) * var_ggate_dn5);
        let eq57_e2026_d_n6: f64 = ((nv1 - nv9) * var_ggate_dn6);
        let eq57_e2026_d_n7: f64 = ((nv1 - nv9) * var_ggate_dn7);
        let eq57_e2026_d_n8: f64 = ((nv1 - nv9) * var_ggate_dn8);
        let eq57_e2026_d_n9: f64 = ((-var_ggate) + ((nv1 - nv9) * var_ggate_dn9));
        let eq57_e2026_d_n10: f64 = ((nv1 - nv9) * var_ggate_dn10);
        let eq57_e2026_d_n11: f64 = ((nv1 - nv9) * var_ggate_dn11);
        (eq57_e2026, var_ggate, eq57_e2026_d_n3, eq57_e2026_d_n4, eq57_e2026_d_n5, eq57_e2026_d_n6, eq57_e2026_d_n7, eq57_e2026_d_n8, eq57_e2026_d_n9, eq57_e2026_d_n10, eq57_e2026_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e2028;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(1),
            Some(9),
            multiplicity * (eq57_value),
            [1, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (eq57_e2028_d_n1), multiplicity * (eq57_e2028_d_n3), multiplicity * (eq57_e2028_d_n4), multiplicity * (eq57_e2028_d_n5), multiplicity * (eq57_e2028_d_n6), multiplicity * (eq57_e2028_d_n7), multiplicity * (eq57_e2028_d_n8), multiplicity * (eq57_e2028_d_n9), multiplicity * (eq57_e2028_d_n10), multiplicity * (eq57_e2028_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq59_e2043, eq59_e2043_d_n0, eq59_e2043_d_n3, eq59_e2043_d_n4, eq59_e2043_d_n5, eq59_e2043_d_n6, eq59_e2043_d_n7, eq59_e2043_d_n8, eq59_e2043_d_n9, eq59_e2043_d_n10, eq59_e2043_d_n11,) = {
    if (var_guard888 != 0.0) {
        let eq59_e2041: f64 = ((nv0 - nv6) * var_gdpr);
        let eq59_e2041_d_n3: f64 = ((nv0 - nv6) * var_gdpr_dn3);
        let eq59_e2041_d_n4: f64 = ((nv0 - nv6) * var_gdpr_dn4);
        let eq59_e2041_d_n5: f64 = ((nv0 - nv6) * var_gdpr_dn5);
        let eq59_e2041_d_n6: f64 = ((-var_gdpr) + ((nv0 - nv6) * var_gdpr_dn6));
        let eq59_e2041_d_n7: f64 = ((nv0 - nv6) * var_gdpr_dn7);
        let eq59_e2041_d_n8: f64 = ((nv0 - nv6) * var_gdpr_dn8);
        let eq59_e2041_d_n9: f64 = ((nv0 - nv6) * var_gdpr_dn9);
        let eq59_e2041_d_n10: f64 = ((nv0 - nv6) * var_gdpr_dn10);
        let eq59_e2041_d_n11: f64 = ((nv0 - nv6) * var_gdpr_dn11);
        (eq59_e2041, var_gdpr, eq59_e2041_d_n3, eq59_e2041_d_n4, eq59_e2041_d_n5, eq59_e2041_d_n6, eq59_e2041_d_n7, eq59_e2041_d_n8, eq59_e2041_d_n9, eq59_e2041_d_n10, eq59_e2041_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq59_value: f64 = eq59_e2043;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(0),
            Some(6),
            multiplicity * (eq59_value),
            [0, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (eq59_e2043_d_n0), multiplicity * (eq59_e2043_d_n3), multiplicity * (eq59_e2043_d_n4), multiplicity * (eq59_e2043_d_n5), multiplicity * (eq59_e2043_d_n6), multiplicity * (eq59_e2043_d_n7), multiplicity * (eq59_e2043_d_n8), multiplicity * (eq59_e2043_d_n9), multiplicity * (eq59_e2043_d_n10), multiplicity * (eq59_e2043_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq60_e2048,) = {
    if (var_guard888 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq60_value: f64 = eq60_e2048;
        stamper.stamp_potential_const_local(
            3,
            eq60_value,
        );
        let (eq62_e2062, eq62_e2062_d_n2, eq62_e2062_d_n3, eq62_e2062_d_n4, eq62_e2062_d_n5, eq62_e2062_d_n6, eq62_e2062_d_n7, eq62_e2062_d_n8, eq62_e2062_d_n9, eq62_e2062_d_n10, eq62_e2062_d_n11,) = {
    if (var_guard890 != 0.0) {
        let eq62_e2060: f64 = ((nv2 - nv7) * var_gspr);
        let eq62_e2060_d_n3: f64 = ((nv2 - nv7) * var_gspr_dn3);
        let eq62_e2060_d_n4: f64 = ((nv2 - nv7) * var_gspr_dn4);
        let eq62_e2060_d_n5: f64 = ((nv2 - nv7) * var_gspr_dn5);
        let eq62_e2060_d_n6: f64 = ((nv2 - nv7) * var_gspr_dn6);
        let eq62_e2060_d_n7: f64 = ((-var_gspr) + ((nv2 - nv7) * var_gspr_dn7));
        let eq62_e2060_d_n8: f64 = ((nv2 - nv7) * var_gspr_dn8);
        let eq62_e2060_d_n9: f64 = ((nv2 - nv7) * var_gspr_dn9);
        let eq62_e2060_d_n10: f64 = ((nv2 - nv7) * var_gspr_dn10);
        let eq62_e2060_d_n11: f64 = ((nv2 - nv7) * var_gspr_dn11);
        (eq62_e2060, var_gspr, eq62_e2060_d_n3, eq62_e2060_d_n4, eq62_e2060_d_n5, eq62_e2060_d_n6, eq62_e2060_d_n7, eq62_e2060_d_n8, eq62_e2060_d_n9, eq62_e2060_d_n10, eq62_e2060_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_value: f64 = eq62_e2062;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(2),
            Some(7),
            multiplicity * (eq62_value),
            [2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (eq62_e2062_d_n2), multiplicity * (eq62_e2062_d_n3), multiplicity * (eq62_e2062_d_n4), multiplicity * (eq62_e2062_d_n5), multiplicity * (eq62_e2062_d_n6), multiplicity * (eq62_e2062_d_n7), multiplicity * (eq62_e2062_d_n8), multiplicity * (eq62_e2062_d_n9), multiplicity * (eq62_e2062_d_n10), multiplicity * (eq62_e2062_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq63_e2067,) = {
    if (var_guard890 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq63_value: f64 = eq63_e2067;
        stamper.stamp_potential_const_local(
            4,
            eq63_value,
        );
        let (eq65_e2081, eq65_e2081_d_n3, eq65_e2081_d_n4, eq65_e2081_d_n5, eq65_e2081_d_n6, eq65_e2081_d_n7, eq65_e2081_d_n8, eq65_e2081_d_n9, eq65_e2081_d_n10, eq65_e2081_d_n11,) = {
    if (var_guard892 != 0.0) {
        let eq65_e2079: f64 = ((nv9 - nv8) * var_gcrg);
        let eq65_e2079_d_n3: f64 = ((nv9 - nv8) * var_gcrg_dn3);
        let eq65_e2079_d_n4: f64 = ((nv9 - nv8) * var_gcrg_dn4);
        let eq65_e2079_d_n5: f64 = ((nv9 - nv8) * var_gcrg_dn5);
        let eq65_e2079_d_n6: f64 = ((nv9 - nv8) * var_gcrg_dn6);
        let eq65_e2079_d_n7: f64 = ((nv9 - nv8) * var_gcrg_dn7);
        let eq65_e2079_d_n8: f64 = ((-var_gcrg) + ((nv9 - nv8) * var_gcrg_dn8));
        let eq65_e2079_d_n9: f64 = (var_gcrg + ((nv9 - nv8) * var_gcrg_dn9));
        let eq65_e2079_d_n10: f64 = ((nv9 - nv8) * var_gcrg_dn10);
        let eq65_e2079_d_n11: f64 = ((nv9 - nv8) * var_gcrg_dn11);
        (eq65_e2079, eq65_e2079_d_n3, eq65_e2079_d_n4, eq65_e2079_d_n5, eq65_e2079_d_n6, eq65_e2079_d_n7, eq65_e2079_d_n8, eq65_e2079_d_n9, eq65_e2079_d_n10, eq65_e2079_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq65_value: f64 = eq65_e2081;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(9),
            Some(8),
            multiplicity * (eq65_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (eq65_e2081_d_n3), multiplicity * (eq65_e2081_d_n4), multiplicity * (eq65_e2081_d_n5), multiplicity * (eq65_e2081_d_n6), multiplicity * (eq65_e2081_d_n7), multiplicity * (eq65_e2081_d_n8), multiplicity * (eq65_e2081_d_n9), multiplicity * (eq65_e2081_d_n10), multiplicity * (eq65_e2081_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq67_e2103, eq67_e2103_d_n0, eq67_e2103_d_n2, eq67_e2103_d_n3, eq67_e2103_d_n4, eq67_e2103_d_n5, eq67_e2103_d_n6, eq67_e2103_d_n7, eq67_e2103_d_n8, eq67_e2103_d_n9, eq67_e2103_d_n10, eq67_e2103_d_n11,) = {
    if (((var_guard893 != 0.0) && (var_guard896 != 0.0)) && (var_guard897 != 0.0)) {
        let eq67_e2094: f64 = (var_deltemp1 * var_gth);
        let eq67_e2094_d_n4: f64 = (var_deltemp1_dn4 * var_gth);
        let eq67_e2094_d_n5: f64 = (var_deltemp1_dn5 * var_gth);
        let eq67_e2097: f64 = (var_deltemp1 * var_cth);
        let eq67_e2097_d_n4: f64 = (var_deltemp1_dn4 * var_cth);
        let eq67_e2097_d_n5: f64 = (var_deltemp1_dn5 * var_cth);
        let eq67_e2098: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 18, eq67_e2097);
        let eq67_e2099: f64 = (eq67_e2094 + eq67_e2098);
        let eq67_e2099_d_n4: f64 = (eq67_e2094_d_n4 + (eq67_e2097_d_n4 * ddt_scale));
        let eq67_e2099_d_n5: f64 = (eq67_e2094_d_n5 + (eq67_e2097_d_n5 * ddt_scale));
        let eq67_e2101: f64 = (eq67_e2099 - var_pdiss);
        let eq67_e2101_d_n4: f64 = (eq67_e2099_d_n4 - var_pdiss_dn4);
        let eq67_e2101_d_n5: f64 = (eq67_e2099_d_n5 - var_pdiss_dn5);
        (eq67_e2101, (-var_pdiss_dn0), (-var_pdiss_dn2), (-var_pdiss_dn3), eq67_e2101_d_n4, eq67_e2101_d_n5, (-var_pdiss_dn6), (-var_pdiss_dn7), (-var_pdiss_dn8), (-var_pdiss_dn9), (-var_pdiss_dn10), (-var_pdiss_dn11),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq67_value: f64 = eq67_e2103;
        let eq67_node_derivative_indices: [usize; 11] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq67_node_derivatives: [f64; 11] = [eq67_e2103_d_n0, eq67_e2103_d_n2, eq67_e2103_d_n3, eq67_e2103_d_n4, eq67_e2103_d_n5, eq67_e2103_d_n6, eq67_e2103_d_n7, eq67_e2103_d_n8, eq67_e2103_d_n9, eq67_e2103_d_n10, eq67_e2103_d_n11];
        let eq67_branch_derivative_indices: [usize; 0] = [];
        let eq67_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(4),
            None,
            multiplicity * (eq67_value),
            &eq67_node_derivative_indices,
            &eq67_node_derivatives,
            &eq67_branch_derivative_indices,
            &eq67_branch_derivatives,
            multiplicity,
        );
        let (eq68_e2121, eq68_e2121_d_n0, eq68_e2121_d_n2, eq68_e2121_d_n3, eq68_e2121_d_n4, eq68_e2121_d_n5, eq68_e2121_d_n6, eq68_e2121_d_n7, eq68_e2121_d_n8, eq68_e2121_d_n9, eq68_e2121_d_n10, eq68_e2121_d_n11,) = {
    if (((var_guard893 != 0.0) && (var_guard896 != 0.0)) && (var_guard897 == 0.0)) {
        let eq68_e2112: f64 = (var_deltemp1 * var_gth);
        let eq68_e2112_d_n4: f64 = (var_deltemp1_dn4 * var_gth);
        let eq68_e2112_d_n5: f64 = (var_deltemp1_dn5 * var_gth);
        let eq68_e2115: f64 = (var_deltemp1 * var_cth);
        let eq68_e2115_d_n4: f64 = (var_deltemp1_dn4 * var_cth);
        let eq68_e2115_d_n5: f64 = (var_deltemp1_dn5 * var_cth);
        let eq68_e2116: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 19, eq68_e2115);
        let eq68_e2117: f64 = (eq68_e2112 + eq68_e2116);
        let eq68_e2117_d_n4: f64 = (eq68_e2112_d_n4 + (eq68_e2115_d_n4 * ddt_scale));
        let eq68_e2117_d_n5: f64 = (eq68_e2112_d_n5 + (eq68_e2115_d_n5 * ddt_scale));
        let eq68_e2119: f64 = (eq68_e2117 - var_pdiss);
        let eq68_e2119_d_n4: f64 = (eq68_e2117_d_n4 - var_pdiss_dn4);
        let eq68_e2119_d_n5: f64 = (eq68_e2117_d_n5 - var_pdiss_dn5);
        (eq68_e2119, (-var_pdiss_dn0), (-var_pdiss_dn2), (-var_pdiss_dn3), eq68_e2119_d_n4, eq68_e2119_d_n5, (-var_pdiss_dn6), (-var_pdiss_dn7), (-var_pdiss_dn8), (-var_pdiss_dn9), (-var_pdiss_dn10), (-var_pdiss_dn11),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq68_value: f64 = eq68_e2121;
        let eq68_node_derivative_indices: [usize; 11] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq68_node_derivatives: [f64; 11] = [eq68_e2121_d_n0, eq68_e2121_d_n2, eq68_e2121_d_n3, eq68_e2121_d_n4, eq68_e2121_d_n5, eq68_e2121_d_n6, eq68_e2121_d_n7, eq68_e2121_d_n8, eq68_e2121_d_n9, eq68_e2121_d_n10, eq68_e2121_d_n11];
        let eq68_branch_derivative_indices: [usize; 0] = [];
        let eq68_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            None,
            multiplicity * (eq68_value),
            &eq68_node_derivative_indices,
            &eq68_node_derivatives,
            &eq68_branch_derivative_indices,
            &eq68_branch_derivatives,
            multiplicity,
        );
        let (eq69_e2137, eq69_e2137_d_n0, eq69_e2137_d_n2, eq69_e2137_d_n3, eq69_e2137_d_n4, eq69_e2137_d_n5, eq69_e2137_d_n6, eq69_e2137_d_n7, eq69_e2137_d_n8, eq69_e2137_d_n9, eq69_e2137_d_n10, eq69_e2137_d_n11,) = {
    if ((var_guard893 != 0.0) && (var_guard896 == 0.0)) {
        let eq69_e2128: f64 = (var_deltemp1 * var_gth);
        let eq69_e2128_d_n4: f64 = (var_deltemp1_dn4 * var_gth);
        let eq69_e2128_d_n5: f64 = (var_deltemp1_dn5 * var_gth);
        let eq69_e2131: f64 = (var_deltemp1 * var_cth);
        let eq69_e2131_d_n4: f64 = (var_deltemp1_dn4 * var_cth);
        let eq69_e2131_d_n5: f64 = (var_deltemp1_dn5 * var_cth);
        let eq69_e2132: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 20, eq69_e2131);
        let eq69_e2133: f64 = (eq69_e2128 + eq69_e2132);
        let eq69_e2133_d_n4: f64 = (eq69_e2128_d_n4 + (eq69_e2131_d_n4 * ddt_scale));
        let eq69_e2133_d_n5: f64 = (eq69_e2128_d_n5 + (eq69_e2131_d_n5 * ddt_scale));
        let eq69_e2135: f64 = (eq69_e2133 - var_pdiss);
        let eq69_e2135_d_n4: f64 = (eq69_e2133_d_n4 - var_pdiss_dn4);
        let eq69_e2135_d_n5: f64 = (eq69_e2133_d_n5 - var_pdiss_dn5);
        (eq69_e2135, (-var_pdiss_dn0), (-var_pdiss_dn2), (-var_pdiss_dn3), eq69_e2135_d_n4, eq69_e2135_d_n5, (-var_pdiss_dn6), (-var_pdiss_dn7), (-var_pdiss_dn8), (-var_pdiss_dn9), (-var_pdiss_dn10), (-var_pdiss_dn11),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq69_value: f64 = eq69_e2137;
        let eq69_node_derivative_indices: [usize; 11] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq69_node_derivatives: [f64; 11] = [eq69_e2137_d_n0, eq69_e2137_d_n2, eq69_e2137_d_n3, eq69_e2137_d_n4, eq69_e2137_d_n5, eq69_e2137_d_n6, eq69_e2137_d_n7, eq69_e2137_d_n8, eq69_e2137_d_n9, eq69_e2137_d_n10, eq69_e2137_d_n11];
        let eq69_branch_derivative_indices: [usize; 0] = [];
        let eq69_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            None,
            multiplicity * (eq69_value),
            &eq69_node_derivative_indices,
            &eq69_node_derivatives,
            &eq69_branch_derivative_indices,
            &eq69_branch_derivatives,
            multiplicity,
        );
        let (eq76_e2187, eq76_e2187_d_n3, eq76_e2187_d_n4, eq76_e2187_d_n5, eq76_e2187_d_n6, eq76_e2187_d_n7, eq76_e2187_d_n8, eq76_e2187_d_n9, eq76_e2187_d_n10, eq76_e2187_d_n11,) = {
    if ((var_guard909 != 0.0) && (var_guard910 != 0.0)) {
        let eq76_e2185: f64 = ((nv4 - nv10) * var_gbody);
        let eq76_e2185_d_n3: f64 = ((nv4 - nv10) * var_gbody_dn3);
        let eq76_e2185_d_n4: f64 = (var_gbody + ((nv4 - nv10) * var_gbody_dn4));
        let eq76_e2185_d_n5: f64 = ((nv4 - nv10) * var_gbody_dn5);
        let eq76_e2185_d_n6: f64 = ((nv4 - nv10) * var_gbody_dn6);
        let eq76_e2185_d_n7: f64 = ((nv4 - nv10) * var_gbody_dn7);
        let eq76_e2185_d_n8: f64 = ((nv4 - nv10) * var_gbody_dn8);
        let eq76_e2185_d_n9: f64 = ((nv4 - nv10) * var_gbody_dn9);
        let eq76_e2185_d_n10: f64 = ((-var_gbody) + ((nv4 - nv10) * var_gbody_dn10));
        let eq76_e2185_d_n11: f64 = ((nv4 - nv10) * var_gbody_dn11);
        (eq76_e2185, eq76_e2185_d_n3, eq76_e2185_d_n4, eq76_e2185_d_n5, eq76_e2185_d_n6, eq76_e2185_d_n7, eq76_e2185_d_n8, eq76_e2185_d_n9, eq76_e2185_d_n10, eq76_e2185_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq76_value: f64 = eq76_e2187;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(4),
            Some(10),
            multiplicity * (eq76_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (eq76_e2187_d_n3), multiplicity * (eq76_e2187_d_n4), multiplicity * (eq76_e2187_d_n5), multiplicity * (eq76_e2187_d_n6), multiplicity * (eq76_e2187_d_n7), multiplicity * (eq76_e2187_d_n8), multiplicity * (eq76_e2187_d_n9), multiplicity * (eq76_e2187_d_n10), multiplicity * (eq76_e2187_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq77_e2195, eq77_e2195_d_n3, eq77_e2195_d_n4, eq77_e2195_d_n5, eq77_e2195_d_n6, eq77_e2195_d_n7, eq77_e2195_d_n8, eq77_e2195_d_n9, eq77_e2195_d_n10, eq77_e2195_d_n11,) = {
    if ((var_guard909 != 0.0) && (var_guard910 != 0.0)) {
        let eq77_e2193: f64 = ((nv4 - nv11) * var_gbodyagbcp2);
        let eq77_e2193_d_n3: f64 = ((nv4 - nv11) * var_gbodyagbcp2_dn3);
        let eq77_e2193_d_n4: f64 = (var_gbodyagbcp2 + ((nv4 - nv11) * var_gbodyagbcp2_dn4));
        let eq77_e2193_d_n5: f64 = ((nv4 - nv11) * var_gbodyagbcp2_dn5);
        let eq77_e2193_d_n6: f64 = ((nv4 - nv11) * var_gbodyagbcp2_dn6);
        let eq77_e2193_d_n7: f64 = ((nv4 - nv11) * var_gbodyagbcp2_dn7);
        let eq77_e2193_d_n8: f64 = ((nv4 - nv11) * var_gbodyagbcp2_dn8);
        let eq77_e2193_d_n9: f64 = ((nv4 - nv11) * var_gbodyagbcp2_dn9);
        let eq77_e2193_d_n10: f64 = ((nv4 - nv11) * var_gbodyagbcp2_dn10);
        let eq77_e2193_d_n11: f64 = ((-var_gbodyagbcp2) + ((nv4 - nv11) * var_gbodyagbcp2_dn11));
        (eq77_e2193, eq77_e2193_d_n3, eq77_e2193_d_n4, eq77_e2193_d_n5, eq77_e2193_d_n6, eq77_e2193_d_n7, eq77_e2193_d_n8, eq77_e2193_d_n9, eq77_e2193_d_n10, eq77_e2193_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq77_value: f64 = eq77_e2195;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(4),
            Some(11),
            multiplicity * (eq77_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (eq77_e2195_d_n3), multiplicity * (eq77_e2195_d_n4), multiplicity * (eq77_e2195_d_n5), multiplicity * (eq77_e2195_d_n6), multiplicity * (eq77_e2195_d_n7), multiplicity * (eq77_e2195_d_n8), multiplicity * (eq77_e2195_d_n9), multiplicity * (eq77_e2195_d_n10), multiplicity * (eq77_e2195_d_n11)],
            [],
            [],
            1.0,
        );
        let eq78_e2198: f64 = (var_devsign * var_ibs);
        let eq78_e2198_d_n3: f64 = (var_devsign * var_ibs_dn3);
        let eq78_e2198_d_n4: f64 = (var_devsign * var_ibs_dn4);
        let eq78_e2198_d_n5: f64 = (var_devsign * var_ibs_dn5);
        let eq78_e2198_d_n6: f64 = (var_devsign * var_ibs_dn6);
        let eq78_e2198_d_n7: f64 = (var_devsign * var_ibs_dn7);
        let eq78_e2198_d_n8: f64 = (var_devsign * var_ibs_dn8);
        let eq78_e2198_d_n9: f64 = (var_devsign * var_ibs_dn9);
        let eq78_e2198_d_n10: f64 = (var_devsign * var_ibs_dn10);
        let eq78_e2198_d_n11: f64 = (var_devsign * var_ibs_dn11);
        let eq78_e2201: f64 = ((nv10 - nv7) * var_gmin);
        let eq78_e2202: f64 = (eq78_e2198 + eq78_e2201);
        let eq78_e2202_d_n7: f64 = (eq78_e2198_d_n7 + (-var_gmin));
        let eq78_e2202_d_n10: f64 = (eq78_e2198_d_n10 + var_gmin);
        let eq78_value: f64 = eq78_e2202;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(10),
            Some(7),
            multiplicity * (eq78_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (eq78_e2198_d_n3), multiplicity * (eq78_e2198_d_n4), multiplicity * (eq78_e2198_d_n5), multiplicity * (eq78_e2198_d_n6), multiplicity * (eq78_e2202_d_n7), multiplicity * (eq78_e2198_d_n8), multiplicity * (eq78_e2198_d_n9), multiplicity * (eq78_e2202_d_n10), multiplicity * (eq78_e2198_d_n11)],
            [],
            [],
            1.0,
        );
        let eq79_e2205: f64 = (var_devsign * var_ibd);
        let eq79_e2205_d_n3: f64 = (var_devsign * var_ibd_dn3);
        let eq79_e2205_d_n4: f64 = (var_devsign * var_ibd_dn4);
        let eq79_e2205_d_n5: f64 = (var_devsign * var_ibd_dn5);
        let eq79_e2205_d_n6: f64 = (var_devsign * var_ibd_dn6);
        let eq79_e2205_d_n7: f64 = (var_devsign * var_ibd_dn7);
        let eq79_e2205_d_n8: f64 = (var_devsign * var_ibd_dn8);
        let eq79_e2205_d_n9: f64 = (var_devsign * var_ibd_dn9);
        let eq79_e2205_d_n10: f64 = (var_devsign * var_ibd_dn10);
        let eq79_e2205_d_n11: f64 = (var_devsign * var_ibd_dn11);
        let eq79_e2208: f64 = ((nv10 - nv6) * var_gmin);
        let eq79_e2209: f64 = (eq79_e2205 + eq79_e2208);
        let eq79_e2209_d_n6: f64 = (eq79_e2205_d_n6 + (-var_gmin));
        let eq79_e2209_d_n10: f64 = (eq79_e2205_d_n10 + var_gmin);
        let eq79_value: f64 = eq79_e2209;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(10),
            Some(6),
            multiplicity * (eq79_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (eq79_e2205_d_n3), multiplicity * (eq79_e2205_d_n4), multiplicity * (eq79_e2205_d_n5), multiplicity * (eq79_e2209_d_n6), multiplicity * (eq79_e2205_d_n7), multiplicity * (eq79_e2205_d_n8), multiplicity * (eq79_e2205_d_n9), multiplicity * (eq79_e2209_d_n10), multiplicity * (eq79_e2205_d_n11)],
            [],
            [],
            1.0,
        );
        let eq80_e2212: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 21, var_qbsj);
        let eq80_e2213: f64 = (var_devsign * eq80_e2212);
        let eq80_e2213_d_n3: f64 = (var_devsign * (var_qbsj_dn3 * ddt_scale));
        let eq80_e2213_d_n4: f64 = (var_devsign * (var_qbsj_dn4 * ddt_scale));
        let eq80_e2213_d_n5: f64 = (var_devsign * (var_qbsj_dn5 * ddt_scale));
        let eq80_e2213_d_n6: f64 = (var_devsign * (var_qbsj_dn6 * ddt_scale));
        let eq80_e2213_d_n7: f64 = (var_devsign * (var_qbsj_dn7 * ddt_scale));
        let eq80_e2213_d_n8: f64 = (var_devsign * (var_qbsj_dn8 * ddt_scale));
        let eq80_e2213_d_n9: f64 = (var_devsign * (var_qbsj_dn9 * ddt_scale));
        let eq80_e2213_d_n10: f64 = (var_devsign * (var_qbsj_dn10 * ddt_scale));
        let eq80_e2213_d_n11: f64 = (var_devsign * (var_qbsj_dn11 * ddt_scale));
        let eq80_value: f64 = eq80_e2213;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(10),
            Some(7),
            multiplicity * (eq80_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (eq80_e2213_d_n3), multiplicity * (eq80_e2213_d_n4), multiplicity * (eq80_e2213_d_n5), multiplicity * (eq80_e2213_d_n6), multiplicity * (eq80_e2213_d_n7), multiplicity * (eq80_e2213_d_n8), multiplicity * (eq80_e2213_d_n9), multiplicity * (eq80_e2213_d_n10), multiplicity * (eq80_e2213_d_n11)],
            [],
            [],
            1.0,
        );
        let eq81_e2216: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 22, var_qbdj);
        let eq81_e2217: f64 = (var_devsign * eq81_e2216);
        let eq81_e2217_d_n3: f64 = (var_devsign * (var_qbdj_dn3 * ddt_scale));
        let eq81_e2217_d_n4: f64 = (var_devsign * (var_qbdj_dn4 * ddt_scale));
        let eq81_e2217_d_n5: f64 = (var_devsign * (var_qbdj_dn5 * ddt_scale));
        let eq81_e2217_d_n6: f64 = (var_devsign * (var_qbdj_dn6 * ddt_scale));
        let eq81_e2217_d_n7: f64 = (var_devsign * (var_qbdj_dn7 * ddt_scale));
        let eq81_e2217_d_n8: f64 = (var_devsign * (var_qbdj_dn8 * ddt_scale));
        let eq81_e2217_d_n9: f64 = (var_devsign * (var_qbdj_dn9 * ddt_scale));
        let eq81_e2217_d_n10: f64 = (var_devsign * (var_qbdj_dn10 * ddt_scale));
        let eq81_e2217_d_n11: f64 = (var_devsign * (var_qbdj_dn11 * ddt_scale));
        let eq81_value: f64 = eq81_e2217;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(10),
            Some(6),
            multiplicity * (eq81_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11],
            [multiplicity * (eq81_e2217_d_n3), multiplicity * (eq81_e2217_d_n4), multiplicity * (eq81_e2217_d_n5), multiplicity * (eq81_e2217_d_n6), multiplicity * (eq81_e2217_d_n7), multiplicity * (eq81_e2217_d_n8), multiplicity * (eq81_e2217_d_n9), multiplicity * (eq81_e2217_d_n10), multiplicity * (eq81_e2217_d_n11)],
            [],
            [],
            1.0,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        var_cox: f64,
        var_guard492: f64,
        var_guard665: f64,
        var_guard666: f64,
        var_guard836: f64,
        var_guard837: f64,
        var_leff: f64,
        var_mig: f64,
        var_mig_dn10: f64,
        var_mig_dn11: f64,
        var_mig_dn3: f64,
        var_mig_dn4: f64,
        var_mig_dn5: f64,
        var_mig_dn6: f64,
        var_mig_dn7: f64,
        var_mig_dn8: f64,
        var_mig_dn9: f64,
        var_sigvds: f64,
        var_weff: f64,
    ) {
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq7_e1546, eq7_e1546_d_n3, eq7_e1546_d_n4, eq7_e1546_d_n5, eq7_e1546_d_n6, eq7_e1546_d_n7, eq7_e1546_d_n8, eq7_e1546_d_n9, eq7_e1546_d_n10, eq7_e1546_d_n11, eq7_e1546_d_n12, eq7_e1546_q,) = {
    if ((var_guard492 != 0.0) && ((var_guard666 != 0.0) && (var_guard665 == 0.0))) {
        let eq7_e1535: f64 = (var_mig * var_cox);
        let eq7_e1535_d_n3: f64 = (var_mig_dn3 * var_cox);
        let eq7_e1535_d_n4: f64 = (var_mig_dn4 * var_cox);
        let eq7_e1535_d_n5: f64 = (var_mig_dn5 * var_cox);
        let eq7_e1535_d_n6: f64 = (var_mig_dn6 * var_cox);
        let eq7_e1535_d_n7: f64 = (var_mig_dn7 * var_cox);
        let eq7_e1535_d_n8: f64 = (var_mig_dn8 * var_cox);
        let eq7_e1535_d_n9: f64 = (var_mig_dn9 * var_cox);
        let eq7_e1535_d_n10: f64 = (var_mig_dn10 * var_cox);
        let eq7_e1535_d_n11: f64 = (var_mig_dn11 * var_cox);
        let eq7_e1537: f64 = (eq7_e1535 * var_weff);
        let eq7_e1537_d_n3: f64 = (eq7_e1535_d_n3 * var_weff);
        let eq7_e1537_d_n4: f64 = (eq7_e1535_d_n4 * var_weff);
        let eq7_e1537_d_n5: f64 = (eq7_e1535_d_n5 * var_weff);
        let eq7_e1537_d_n6: f64 = (eq7_e1535_d_n6 * var_weff);
        let eq7_e1537_d_n7: f64 = (eq7_e1535_d_n7 * var_weff);
        let eq7_e1537_d_n8: f64 = (eq7_e1535_d_n8 * var_weff);
        let eq7_e1537_d_n9: f64 = (eq7_e1535_d_n9 * var_weff);
        let eq7_e1537_d_n10: f64 = (eq7_e1535_d_n10 * var_weff);
        let eq7_e1537_d_n11: f64 = (eq7_e1535_d_n11 * var_weff);
        let eq7_e1539: f64 = (eq7_e1537 * p.p2);
        let eq7_e1539_d_n3: f64 = (eq7_e1537_d_n3 * p.p2);
        let eq7_e1539_d_n4: f64 = (eq7_e1537_d_n4 * p.p2);
        let eq7_e1539_d_n5: f64 = (eq7_e1537_d_n5 * p.p2);
        let eq7_e1539_d_n6: f64 = (eq7_e1537_d_n6 * p.p2);
        let eq7_e1539_d_n7: f64 = (eq7_e1537_d_n7 * p.p2);
        let eq7_e1539_d_n8: f64 = (eq7_e1537_d_n8 * p.p2);
        let eq7_e1539_d_n9: f64 = (eq7_e1537_d_n9 * p.p2);
        let eq7_e1539_d_n10: f64 = (eq7_e1537_d_n10 * p.p2);
        let eq7_e1539_d_n11: f64 = (eq7_e1537_d_n11 * p.p2);
        let eq7_e1541: f64 = (eq7_e1539 * var_leff);
        let eq7_e1541_d_n3: f64 = (eq7_e1539_d_n3 * var_leff);
        let eq7_e1541_d_n4: f64 = (eq7_e1539_d_n4 * var_leff);
        let eq7_e1541_d_n5: f64 = (eq7_e1539_d_n5 * var_leff);
        let eq7_e1541_d_n6: f64 = (eq7_e1539_d_n6 * var_leff);
        let eq7_e1541_d_n7: f64 = (eq7_e1539_d_n7 * var_leff);
        let eq7_e1541_d_n8: f64 = (eq7_e1539_d_n8 * var_leff);
        let eq7_e1541_d_n9: f64 = (eq7_e1539_d_n9 * var_leff);
        let eq7_e1541_d_n10: f64 = (eq7_e1539_d_n10 * var_leff);
        let eq7_e1541_d_n11: f64 = (eq7_e1539_d_n11 * var_leff);
        let eq7_e1543: f64 = (eq7_e1541 * (nv12 - 0.0));
        let eq7_e1543_d_n3: f64 = (eq7_e1541_d_n3 * (nv12 - 0.0));
        let eq7_e1543_d_n4: f64 = (eq7_e1541_d_n4 * (nv12 - 0.0));
        let eq7_e1543_d_n5: f64 = (eq7_e1541_d_n5 * (nv12 - 0.0));
        let eq7_e1543_d_n6: f64 = (eq7_e1541_d_n6 * (nv12 - 0.0));
        let eq7_e1543_d_n7: f64 = (eq7_e1541_d_n7 * (nv12 - 0.0));
        let eq7_e1543_d_n8: f64 = (eq7_e1541_d_n8 * (nv12 - 0.0));
        let eq7_e1543_d_n9: f64 = (eq7_e1541_d_n9 * (nv12 - 0.0));
        let eq7_e1543_d_n10: f64 = (eq7_e1541_d_n10 * (nv12 - 0.0));
        let eq7_e1543_d_n11: f64 = (eq7_e1541_d_n11 * (nv12 - 0.0));
        let eq7_e1544_q: f64 = eq7_e1543;
        (eq7_e1543, eq7_e1543_d_n3, eq7_e1543_d_n4, eq7_e1543_d_n5, eq7_e1543_d_n6, eq7_e1543_d_n7, eq7_e1543_d_n8, eq7_e1543_d_n9, eq7_e1543_d_n10, eq7_e1543_d_n11, eq7_e1541, eq7_e1544_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, eq7_e1546_d_n3, eq7_e1546_d_n4, eq7_e1546_d_n5, eq7_e1546_d_n6, eq7_e1546_d_n7, eq7_e1546_d_n8, eq7_e1546_d_n9, eq7_e1546_d_n10, eq7_e1546_d_n11, eq7_e1546_d_n12, 0.0];
        let eq7_reactive_branch_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            None,
            nodes,
            &eq7_reactive_node_derivatives,
            branches,
            &eq7_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq10_e1600, eq10_e1600_d_n3, eq10_e1600_d_n4, eq10_e1600_d_n5, eq10_e1600_d_n6, eq10_e1600_d_n7, eq10_e1600_d_n8, eq10_e1600_d_n9, eq10_e1600_d_n10, eq10_e1600_d_n11, eq10_e1600_d_n12, eq10_e1600_q,) = {
    if ((var_guard492 != 0.0) && ((var_guard666 != 0.0) && (var_guard665 == 0.0))) {
        let eq10_e1584: f64 = (1.0 + var_sigvds);
        let eq10_e1586: f64 = (eq10_e1584 * var_mig);
        let eq10_e1586_d_n3: f64 = (eq10_e1584 * var_mig_dn3);
        let eq10_e1586_d_n4: f64 = (eq10_e1584 * var_mig_dn4);
        let eq10_e1586_d_n5: f64 = (eq10_e1584 * var_mig_dn5);
        let eq10_e1586_d_n6: f64 = (eq10_e1584 * var_mig_dn6);
        let eq10_e1586_d_n7: f64 = (eq10_e1584 * var_mig_dn7);
        let eq10_e1586_d_n8: f64 = (eq10_e1584 * var_mig_dn8);
        let eq10_e1586_d_n9: f64 = (eq10_e1584 * var_mig_dn9);
        let eq10_e1586_d_n10: f64 = (eq10_e1584 * var_mig_dn10);
        let eq10_e1586_d_n11: f64 = (eq10_e1584 * var_mig_dn11);
        let eq10_e1588: f64 = (eq10_e1586 * var_cox);
        let eq10_e1588_d_n3: f64 = (eq10_e1586_d_n3 * var_cox);
        let eq10_e1588_d_n4: f64 = (eq10_e1586_d_n4 * var_cox);
        let eq10_e1588_d_n5: f64 = (eq10_e1586_d_n5 * var_cox);
        let eq10_e1588_d_n6: f64 = (eq10_e1586_d_n6 * var_cox);
        let eq10_e1588_d_n7: f64 = (eq10_e1586_d_n7 * var_cox);
        let eq10_e1588_d_n8: f64 = (eq10_e1586_d_n8 * var_cox);
        let eq10_e1588_d_n9: f64 = (eq10_e1586_d_n9 * var_cox);
        let eq10_e1588_d_n10: f64 = (eq10_e1586_d_n10 * var_cox);
        let eq10_e1588_d_n11: f64 = (eq10_e1586_d_n11 * var_cox);
        let eq10_e1590: f64 = (eq10_e1588 * var_weff);
        let eq10_e1590_d_n3: f64 = (eq10_e1588_d_n3 * var_weff);
        let eq10_e1590_d_n4: f64 = (eq10_e1588_d_n4 * var_weff);
        let eq10_e1590_d_n5: f64 = (eq10_e1588_d_n5 * var_weff);
        let eq10_e1590_d_n6: f64 = (eq10_e1588_d_n6 * var_weff);
        let eq10_e1590_d_n7: f64 = (eq10_e1588_d_n7 * var_weff);
        let eq10_e1590_d_n8: f64 = (eq10_e1588_d_n8 * var_weff);
        let eq10_e1590_d_n9: f64 = (eq10_e1588_d_n9 * var_weff);
        let eq10_e1590_d_n10: f64 = (eq10_e1588_d_n10 * var_weff);
        let eq10_e1590_d_n11: f64 = (eq10_e1588_d_n11 * var_weff);
        let eq10_e1592: f64 = (eq10_e1590 * p.p2);
        let eq10_e1592_d_n3: f64 = (eq10_e1590_d_n3 * p.p2);
        let eq10_e1592_d_n4: f64 = (eq10_e1590_d_n4 * p.p2);
        let eq10_e1592_d_n5: f64 = (eq10_e1590_d_n5 * p.p2);
        let eq10_e1592_d_n6: f64 = (eq10_e1590_d_n6 * p.p2);
        let eq10_e1592_d_n7: f64 = (eq10_e1590_d_n7 * p.p2);
        let eq10_e1592_d_n8: f64 = (eq10_e1590_d_n8 * p.p2);
        let eq10_e1592_d_n9: f64 = (eq10_e1590_d_n9 * p.p2);
        let eq10_e1592_d_n10: f64 = (eq10_e1590_d_n10 * p.p2);
        let eq10_e1592_d_n11: f64 = (eq10_e1590_d_n11 * p.p2);
        let eq10_e1594: f64 = (eq10_e1592 * var_leff);
        let eq10_e1594_d_n3: f64 = (eq10_e1592_d_n3 * var_leff);
        let eq10_e1594_d_n4: f64 = (eq10_e1592_d_n4 * var_leff);
        let eq10_e1594_d_n5: f64 = (eq10_e1592_d_n5 * var_leff);
        let eq10_e1594_d_n6: f64 = (eq10_e1592_d_n6 * var_leff);
        let eq10_e1594_d_n7: f64 = (eq10_e1592_d_n7 * var_leff);
        let eq10_e1594_d_n8: f64 = (eq10_e1592_d_n8 * var_leff);
        let eq10_e1594_d_n9: f64 = (eq10_e1592_d_n9 * var_leff);
        let eq10_e1594_d_n10: f64 = (eq10_e1592_d_n10 * var_leff);
        let eq10_e1594_d_n11: f64 = (eq10_e1592_d_n11 * var_leff);
        let eq10_e1596: f64 = (eq10_e1594 * (nv12 - 0.0));
        let eq10_e1596_d_n3: f64 = (eq10_e1594_d_n3 * (nv12 - 0.0));
        let eq10_e1596_d_n4: f64 = (eq10_e1594_d_n4 * (nv12 - 0.0));
        let eq10_e1596_d_n5: f64 = (eq10_e1594_d_n5 * (nv12 - 0.0));
        let eq10_e1596_d_n6: f64 = (eq10_e1594_d_n6 * (nv12 - 0.0));
        let eq10_e1596_d_n7: f64 = (eq10_e1594_d_n7 * (nv12 - 0.0));
        let eq10_e1596_d_n8: f64 = (eq10_e1594_d_n8 * (nv12 - 0.0));
        let eq10_e1596_d_n9: f64 = (eq10_e1594_d_n9 * (nv12 - 0.0));
        let eq10_e1596_d_n10: f64 = (eq10_e1594_d_n10 * (nv12 - 0.0));
        let eq10_e1596_d_n11: f64 = (eq10_e1594_d_n11 * (nv12 - 0.0));
        let eq10_e1597: f64 = (0.5 * eq10_e1596);
        let eq10_e1597_d_n3: f64 = (0.5 * eq10_e1596_d_n3);
        let eq10_e1597_d_n4: f64 = (0.5 * eq10_e1596_d_n4);
        let eq10_e1597_d_n5: f64 = (0.5 * eq10_e1596_d_n5);
        let eq10_e1597_d_n6: f64 = (0.5 * eq10_e1596_d_n6);
        let eq10_e1597_d_n7: f64 = (0.5 * eq10_e1596_d_n7);
        let eq10_e1597_d_n8: f64 = (0.5 * eq10_e1596_d_n8);
        let eq10_e1597_d_n9: f64 = (0.5 * eq10_e1596_d_n9);
        let eq10_e1597_d_n10: f64 = (0.5 * eq10_e1596_d_n10);
        let eq10_e1597_d_n11: f64 = (0.5 * eq10_e1596_d_n11);
        let eq10_e1597_d_n12: f64 = (0.5 * eq10_e1594);
        let eq10_e1598_q: f64 = eq10_e1597;
        (eq10_e1597, eq10_e1597_d_n3, eq10_e1597_d_n4, eq10_e1597_d_n5, eq10_e1597_d_n6, eq10_e1597_d_n7, eq10_e1597_d_n8, eq10_e1597_d_n9, eq10_e1597_d_n10, eq10_e1597_d_n11, eq10_e1597_d_n12, eq10_e1598_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, eq10_e1600_d_n3, eq10_e1600_d_n4, eq10_e1600_d_n5, eq10_e1600_d_n6, eq10_e1600_d_n7, eq10_e1600_d_n8, eq10_e1600_d_n9, eq10_e1600_d_n10, eq10_e1600_d_n11, eq10_e1600_d_n12, 0.0];
        let eq10_reactive_branch_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            nodes,
            &eq10_reactive_node_derivatives,
            branches,
            &eq10_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq11_e1626, eq11_e1626_d_n3, eq11_e1626_d_n4, eq11_e1626_d_n5, eq11_e1626_d_n6, eq11_e1626_d_n7, eq11_e1626_d_n8, eq11_e1626_d_n9, eq11_e1626_d_n10, eq11_e1626_d_n11, eq11_e1626_d_n12, eq11_e1626_q,) = {
    if ((var_guard492 != 0.0) && ((var_guard666 != 0.0) && (var_guard665 == 0.0))) {
        let eq11_e1610: f64 = (1.0 - var_sigvds);
        let eq11_e1612: f64 = (eq11_e1610 * var_mig);
        let eq11_e1612_d_n3: f64 = (eq11_e1610 * var_mig_dn3);
        let eq11_e1612_d_n4: f64 = (eq11_e1610 * var_mig_dn4);
        let eq11_e1612_d_n5: f64 = (eq11_e1610 * var_mig_dn5);
        let eq11_e1612_d_n6: f64 = (eq11_e1610 * var_mig_dn6);
        let eq11_e1612_d_n7: f64 = (eq11_e1610 * var_mig_dn7);
        let eq11_e1612_d_n8: f64 = (eq11_e1610 * var_mig_dn8);
        let eq11_e1612_d_n9: f64 = (eq11_e1610 * var_mig_dn9);
        let eq11_e1612_d_n10: f64 = (eq11_e1610 * var_mig_dn10);
        let eq11_e1612_d_n11: f64 = (eq11_e1610 * var_mig_dn11);
        let eq11_e1614: f64 = (eq11_e1612 * var_cox);
        let eq11_e1614_d_n3: f64 = (eq11_e1612_d_n3 * var_cox);
        let eq11_e1614_d_n4: f64 = (eq11_e1612_d_n4 * var_cox);
        let eq11_e1614_d_n5: f64 = (eq11_e1612_d_n5 * var_cox);
        let eq11_e1614_d_n6: f64 = (eq11_e1612_d_n6 * var_cox);
        let eq11_e1614_d_n7: f64 = (eq11_e1612_d_n7 * var_cox);
        let eq11_e1614_d_n8: f64 = (eq11_e1612_d_n8 * var_cox);
        let eq11_e1614_d_n9: f64 = (eq11_e1612_d_n9 * var_cox);
        let eq11_e1614_d_n10: f64 = (eq11_e1612_d_n10 * var_cox);
        let eq11_e1614_d_n11: f64 = (eq11_e1612_d_n11 * var_cox);
        let eq11_e1616: f64 = (eq11_e1614 * var_weff);
        let eq11_e1616_d_n3: f64 = (eq11_e1614_d_n3 * var_weff);
        let eq11_e1616_d_n4: f64 = (eq11_e1614_d_n4 * var_weff);
        let eq11_e1616_d_n5: f64 = (eq11_e1614_d_n5 * var_weff);
        let eq11_e1616_d_n6: f64 = (eq11_e1614_d_n6 * var_weff);
        let eq11_e1616_d_n7: f64 = (eq11_e1614_d_n7 * var_weff);
        let eq11_e1616_d_n8: f64 = (eq11_e1614_d_n8 * var_weff);
        let eq11_e1616_d_n9: f64 = (eq11_e1614_d_n9 * var_weff);
        let eq11_e1616_d_n10: f64 = (eq11_e1614_d_n10 * var_weff);
        let eq11_e1616_d_n11: f64 = (eq11_e1614_d_n11 * var_weff);
        let eq11_e1618: f64 = (eq11_e1616 * p.p2);
        let eq11_e1618_d_n3: f64 = (eq11_e1616_d_n3 * p.p2);
        let eq11_e1618_d_n4: f64 = (eq11_e1616_d_n4 * p.p2);
        let eq11_e1618_d_n5: f64 = (eq11_e1616_d_n5 * p.p2);
        let eq11_e1618_d_n6: f64 = (eq11_e1616_d_n6 * p.p2);
        let eq11_e1618_d_n7: f64 = (eq11_e1616_d_n7 * p.p2);
        let eq11_e1618_d_n8: f64 = (eq11_e1616_d_n8 * p.p2);
        let eq11_e1618_d_n9: f64 = (eq11_e1616_d_n9 * p.p2);
        let eq11_e1618_d_n10: f64 = (eq11_e1616_d_n10 * p.p2);
        let eq11_e1618_d_n11: f64 = (eq11_e1616_d_n11 * p.p2);
        let eq11_e1620: f64 = (eq11_e1618 * var_leff);
        let eq11_e1620_d_n3: f64 = (eq11_e1618_d_n3 * var_leff);
        let eq11_e1620_d_n4: f64 = (eq11_e1618_d_n4 * var_leff);
        let eq11_e1620_d_n5: f64 = (eq11_e1618_d_n5 * var_leff);
        let eq11_e1620_d_n6: f64 = (eq11_e1618_d_n6 * var_leff);
        let eq11_e1620_d_n7: f64 = (eq11_e1618_d_n7 * var_leff);
        let eq11_e1620_d_n8: f64 = (eq11_e1618_d_n8 * var_leff);
        let eq11_e1620_d_n9: f64 = (eq11_e1618_d_n9 * var_leff);
        let eq11_e1620_d_n10: f64 = (eq11_e1618_d_n10 * var_leff);
        let eq11_e1620_d_n11: f64 = (eq11_e1618_d_n11 * var_leff);
        let eq11_e1622: f64 = (eq11_e1620 * (nv12 - 0.0));
        let eq11_e1622_d_n3: f64 = (eq11_e1620_d_n3 * (nv12 - 0.0));
        let eq11_e1622_d_n4: f64 = (eq11_e1620_d_n4 * (nv12 - 0.0));
        let eq11_e1622_d_n5: f64 = (eq11_e1620_d_n5 * (nv12 - 0.0));
        let eq11_e1622_d_n6: f64 = (eq11_e1620_d_n6 * (nv12 - 0.0));
        let eq11_e1622_d_n7: f64 = (eq11_e1620_d_n7 * (nv12 - 0.0));
        let eq11_e1622_d_n8: f64 = (eq11_e1620_d_n8 * (nv12 - 0.0));
        let eq11_e1622_d_n9: f64 = (eq11_e1620_d_n9 * (nv12 - 0.0));
        let eq11_e1622_d_n10: f64 = (eq11_e1620_d_n10 * (nv12 - 0.0));
        let eq11_e1622_d_n11: f64 = (eq11_e1620_d_n11 * (nv12 - 0.0));
        let eq11_e1623: f64 = (0.5 * eq11_e1622);
        let eq11_e1623_d_n3: f64 = (0.5 * eq11_e1622_d_n3);
        let eq11_e1623_d_n4: f64 = (0.5 * eq11_e1622_d_n4);
        let eq11_e1623_d_n5: f64 = (0.5 * eq11_e1622_d_n5);
        let eq11_e1623_d_n6: f64 = (0.5 * eq11_e1622_d_n6);
        let eq11_e1623_d_n7: f64 = (0.5 * eq11_e1622_d_n7);
        let eq11_e1623_d_n8: f64 = (0.5 * eq11_e1622_d_n8);
        let eq11_e1623_d_n9: f64 = (0.5 * eq11_e1622_d_n9);
        let eq11_e1623_d_n10: f64 = (0.5 * eq11_e1622_d_n10);
        let eq11_e1623_d_n11: f64 = (0.5 * eq11_e1622_d_n11);
        let eq11_e1623_d_n12: f64 = (0.5 * eq11_e1620);
        let eq11_e1624_q: f64 = eq11_e1623;
        (eq11_e1623, eq11_e1623_d_n3, eq11_e1623_d_n4, eq11_e1623_d_n5, eq11_e1623_d_n6, eq11_e1623_d_n7, eq11_e1623_d_n8, eq11_e1623_d_n9, eq11_e1623_d_n10, eq11_e1623_d_n11, eq11_e1623_d_n12, eq11_e1624_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, eq11_e1626_d_n3, eq11_e1626_d_n4, eq11_e1626_d_n5, eq11_e1626_d_n6, eq11_e1626_d_n7, eq11_e1626_d_n8, eq11_e1626_d_n9, eq11_e1626_d_n10, eq11_e1626_d_n11, eq11_e1626_d_n12, 0.0];
        let eq11_reactive_branch_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq11_reactive_node_derivatives,
            branches,
            &eq11_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq24_e1784, eq24_e1784_d_n3, eq24_e1784_d_n4, eq24_e1784_d_n5, eq24_e1784_d_n6, eq24_e1784_d_n7, eq24_e1784_d_n8, eq24_e1784_d_n9, eq24_e1784_d_n10, eq24_e1784_d_n11, eq24_e1784_d_n12, eq24_e1784_q,) = {
    if ((var_guard492 == 0.0) && ((var_guard837 != 0.0) && (var_guard836 == 0.0))) {
        let eq24_e1773: f64 = (var_mig * var_cox);
        let eq24_e1773_d_n3: f64 = (var_mig_dn3 * var_cox);
        let eq24_e1773_d_n4: f64 = (var_mig_dn4 * var_cox);
        let eq24_e1773_d_n5: f64 = (var_mig_dn5 * var_cox);
        let eq24_e1773_d_n6: f64 = (var_mig_dn6 * var_cox);
        let eq24_e1773_d_n7: f64 = (var_mig_dn7 * var_cox);
        let eq24_e1773_d_n8: f64 = (var_mig_dn8 * var_cox);
        let eq24_e1773_d_n9: f64 = (var_mig_dn9 * var_cox);
        let eq24_e1773_d_n10: f64 = (var_mig_dn10 * var_cox);
        let eq24_e1773_d_n11: f64 = (var_mig_dn11 * var_cox);
        let eq24_e1775: f64 = (eq24_e1773 * var_weff);
        let eq24_e1775_d_n3: f64 = (eq24_e1773_d_n3 * var_weff);
        let eq24_e1775_d_n4: f64 = (eq24_e1773_d_n4 * var_weff);
        let eq24_e1775_d_n5: f64 = (eq24_e1773_d_n5 * var_weff);
        let eq24_e1775_d_n6: f64 = (eq24_e1773_d_n6 * var_weff);
        let eq24_e1775_d_n7: f64 = (eq24_e1773_d_n7 * var_weff);
        let eq24_e1775_d_n8: f64 = (eq24_e1773_d_n8 * var_weff);
        let eq24_e1775_d_n9: f64 = (eq24_e1773_d_n9 * var_weff);
        let eq24_e1775_d_n10: f64 = (eq24_e1773_d_n10 * var_weff);
        let eq24_e1775_d_n11: f64 = (eq24_e1773_d_n11 * var_weff);
        let eq24_e1777: f64 = (eq24_e1775 * p.p2);
        let eq24_e1777_d_n3: f64 = (eq24_e1775_d_n3 * p.p2);
        let eq24_e1777_d_n4: f64 = (eq24_e1775_d_n4 * p.p2);
        let eq24_e1777_d_n5: f64 = (eq24_e1775_d_n5 * p.p2);
        let eq24_e1777_d_n6: f64 = (eq24_e1775_d_n6 * p.p2);
        let eq24_e1777_d_n7: f64 = (eq24_e1775_d_n7 * p.p2);
        let eq24_e1777_d_n8: f64 = (eq24_e1775_d_n8 * p.p2);
        let eq24_e1777_d_n9: f64 = (eq24_e1775_d_n9 * p.p2);
        let eq24_e1777_d_n10: f64 = (eq24_e1775_d_n10 * p.p2);
        let eq24_e1777_d_n11: f64 = (eq24_e1775_d_n11 * p.p2);
        let eq24_e1779: f64 = (eq24_e1777 * var_leff);
        let eq24_e1779_d_n3: f64 = (eq24_e1777_d_n3 * var_leff);
        let eq24_e1779_d_n4: f64 = (eq24_e1777_d_n4 * var_leff);
        let eq24_e1779_d_n5: f64 = (eq24_e1777_d_n5 * var_leff);
        let eq24_e1779_d_n6: f64 = (eq24_e1777_d_n6 * var_leff);
        let eq24_e1779_d_n7: f64 = (eq24_e1777_d_n7 * var_leff);
        let eq24_e1779_d_n8: f64 = (eq24_e1777_d_n8 * var_leff);
        let eq24_e1779_d_n9: f64 = (eq24_e1777_d_n9 * var_leff);
        let eq24_e1779_d_n10: f64 = (eq24_e1777_d_n10 * var_leff);
        let eq24_e1779_d_n11: f64 = (eq24_e1777_d_n11 * var_leff);
        let eq24_e1781: f64 = (eq24_e1779 * (nv12 - 0.0));
        let eq24_e1781_d_n3: f64 = (eq24_e1779_d_n3 * (nv12 - 0.0));
        let eq24_e1781_d_n4: f64 = (eq24_e1779_d_n4 * (nv12 - 0.0));
        let eq24_e1781_d_n5: f64 = (eq24_e1779_d_n5 * (nv12 - 0.0));
        let eq24_e1781_d_n6: f64 = (eq24_e1779_d_n6 * (nv12 - 0.0));
        let eq24_e1781_d_n7: f64 = (eq24_e1779_d_n7 * (nv12 - 0.0));
        let eq24_e1781_d_n8: f64 = (eq24_e1779_d_n8 * (nv12 - 0.0));
        let eq24_e1781_d_n9: f64 = (eq24_e1779_d_n9 * (nv12 - 0.0));
        let eq24_e1781_d_n10: f64 = (eq24_e1779_d_n10 * (nv12 - 0.0));
        let eq24_e1781_d_n11: f64 = (eq24_e1779_d_n11 * (nv12 - 0.0));
        let eq24_e1782_q: f64 = eq24_e1781;
        (eq24_e1781, eq24_e1781_d_n3, eq24_e1781_d_n4, eq24_e1781_d_n5, eq24_e1781_d_n6, eq24_e1781_d_n7, eq24_e1781_d_n8, eq24_e1781_d_n9, eq24_e1781_d_n10, eq24_e1781_d_n11, eq24_e1779, eq24_e1782_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq24_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, eq24_e1784_d_n3, eq24_e1784_d_n4, eq24_e1784_d_n5, eq24_e1784_d_n6, eq24_e1784_d_n7, eq24_e1784_d_n8, eq24_e1784_d_n9, eq24_e1784_d_n10, eq24_e1784_d_n11, eq24_e1784_d_n12, 0.0];
        let eq24_reactive_branch_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            None,
            nodes,
            &eq24_reactive_node_derivatives,
            branches,
            &eq24_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq27_e1841, eq27_e1841_d_n3, eq27_e1841_d_n4, eq27_e1841_d_n5, eq27_e1841_d_n6, eq27_e1841_d_n7, eq27_e1841_d_n8, eq27_e1841_d_n9, eq27_e1841_d_n10, eq27_e1841_d_n11, eq27_e1841_d_n12, eq27_e1841_q,) = {
    if ((var_guard492 == 0.0) && ((var_guard837 != 0.0) && (var_guard836 == 0.0))) {
        let eq27_e1825: f64 = (1.0 + var_sigvds);
        let eq27_e1827: f64 = (eq27_e1825 * var_mig);
        let eq27_e1827_d_n3: f64 = (eq27_e1825 * var_mig_dn3);
        let eq27_e1827_d_n4: f64 = (eq27_e1825 * var_mig_dn4);
        let eq27_e1827_d_n5: f64 = (eq27_e1825 * var_mig_dn5);
        let eq27_e1827_d_n6: f64 = (eq27_e1825 * var_mig_dn6);
        let eq27_e1827_d_n7: f64 = (eq27_e1825 * var_mig_dn7);
        let eq27_e1827_d_n8: f64 = (eq27_e1825 * var_mig_dn8);
        let eq27_e1827_d_n9: f64 = (eq27_e1825 * var_mig_dn9);
        let eq27_e1827_d_n10: f64 = (eq27_e1825 * var_mig_dn10);
        let eq27_e1827_d_n11: f64 = (eq27_e1825 * var_mig_dn11);
        let eq27_e1829: f64 = (eq27_e1827 * var_cox);
        let eq27_e1829_d_n3: f64 = (eq27_e1827_d_n3 * var_cox);
        let eq27_e1829_d_n4: f64 = (eq27_e1827_d_n4 * var_cox);
        let eq27_e1829_d_n5: f64 = (eq27_e1827_d_n5 * var_cox);
        let eq27_e1829_d_n6: f64 = (eq27_e1827_d_n6 * var_cox);
        let eq27_e1829_d_n7: f64 = (eq27_e1827_d_n7 * var_cox);
        let eq27_e1829_d_n8: f64 = (eq27_e1827_d_n8 * var_cox);
        let eq27_e1829_d_n9: f64 = (eq27_e1827_d_n9 * var_cox);
        let eq27_e1829_d_n10: f64 = (eq27_e1827_d_n10 * var_cox);
        let eq27_e1829_d_n11: f64 = (eq27_e1827_d_n11 * var_cox);
        let eq27_e1831: f64 = (eq27_e1829 * var_weff);
        let eq27_e1831_d_n3: f64 = (eq27_e1829_d_n3 * var_weff);
        let eq27_e1831_d_n4: f64 = (eq27_e1829_d_n4 * var_weff);
        let eq27_e1831_d_n5: f64 = (eq27_e1829_d_n5 * var_weff);
        let eq27_e1831_d_n6: f64 = (eq27_e1829_d_n6 * var_weff);
        let eq27_e1831_d_n7: f64 = (eq27_e1829_d_n7 * var_weff);
        let eq27_e1831_d_n8: f64 = (eq27_e1829_d_n8 * var_weff);
        let eq27_e1831_d_n9: f64 = (eq27_e1829_d_n9 * var_weff);
        let eq27_e1831_d_n10: f64 = (eq27_e1829_d_n10 * var_weff);
        let eq27_e1831_d_n11: f64 = (eq27_e1829_d_n11 * var_weff);
        let eq27_e1833: f64 = (eq27_e1831 * p.p2);
        let eq27_e1833_d_n3: f64 = (eq27_e1831_d_n3 * p.p2);
        let eq27_e1833_d_n4: f64 = (eq27_e1831_d_n4 * p.p2);
        let eq27_e1833_d_n5: f64 = (eq27_e1831_d_n5 * p.p2);
        let eq27_e1833_d_n6: f64 = (eq27_e1831_d_n6 * p.p2);
        let eq27_e1833_d_n7: f64 = (eq27_e1831_d_n7 * p.p2);
        let eq27_e1833_d_n8: f64 = (eq27_e1831_d_n8 * p.p2);
        let eq27_e1833_d_n9: f64 = (eq27_e1831_d_n9 * p.p2);
        let eq27_e1833_d_n10: f64 = (eq27_e1831_d_n10 * p.p2);
        let eq27_e1833_d_n11: f64 = (eq27_e1831_d_n11 * p.p2);
        let eq27_e1835: f64 = (eq27_e1833 * var_leff);
        let eq27_e1835_d_n3: f64 = (eq27_e1833_d_n3 * var_leff);
        let eq27_e1835_d_n4: f64 = (eq27_e1833_d_n4 * var_leff);
        let eq27_e1835_d_n5: f64 = (eq27_e1833_d_n5 * var_leff);
        let eq27_e1835_d_n6: f64 = (eq27_e1833_d_n6 * var_leff);
        let eq27_e1835_d_n7: f64 = (eq27_e1833_d_n7 * var_leff);
        let eq27_e1835_d_n8: f64 = (eq27_e1833_d_n8 * var_leff);
        let eq27_e1835_d_n9: f64 = (eq27_e1833_d_n9 * var_leff);
        let eq27_e1835_d_n10: f64 = (eq27_e1833_d_n10 * var_leff);
        let eq27_e1835_d_n11: f64 = (eq27_e1833_d_n11 * var_leff);
        let eq27_e1837: f64 = (eq27_e1835 * (nv12 - 0.0));
        let eq27_e1837_d_n3: f64 = (eq27_e1835_d_n3 * (nv12 - 0.0));
        let eq27_e1837_d_n4: f64 = (eq27_e1835_d_n4 * (nv12 - 0.0));
        let eq27_e1837_d_n5: f64 = (eq27_e1835_d_n5 * (nv12 - 0.0));
        let eq27_e1837_d_n6: f64 = (eq27_e1835_d_n6 * (nv12 - 0.0));
        let eq27_e1837_d_n7: f64 = (eq27_e1835_d_n7 * (nv12 - 0.0));
        let eq27_e1837_d_n8: f64 = (eq27_e1835_d_n8 * (nv12 - 0.0));
        let eq27_e1837_d_n9: f64 = (eq27_e1835_d_n9 * (nv12 - 0.0));
        let eq27_e1837_d_n10: f64 = (eq27_e1835_d_n10 * (nv12 - 0.0));
        let eq27_e1837_d_n11: f64 = (eq27_e1835_d_n11 * (nv12 - 0.0));
        let eq27_e1838: f64 = (0.5 * eq27_e1837);
        let eq27_e1838_d_n3: f64 = (0.5 * eq27_e1837_d_n3);
        let eq27_e1838_d_n4: f64 = (0.5 * eq27_e1837_d_n4);
        let eq27_e1838_d_n5: f64 = (0.5 * eq27_e1837_d_n5);
        let eq27_e1838_d_n6: f64 = (0.5 * eq27_e1837_d_n6);
        let eq27_e1838_d_n7: f64 = (0.5 * eq27_e1837_d_n7);
        let eq27_e1838_d_n8: f64 = (0.5 * eq27_e1837_d_n8);
        let eq27_e1838_d_n9: f64 = (0.5 * eq27_e1837_d_n9);
        let eq27_e1838_d_n10: f64 = (0.5 * eq27_e1837_d_n10);
        let eq27_e1838_d_n11: f64 = (0.5 * eq27_e1837_d_n11);
        let eq27_e1838_d_n12: f64 = (0.5 * eq27_e1835);
        let eq27_e1839_q: f64 = eq27_e1838;
        (eq27_e1838, eq27_e1838_d_n3, eq27_e1838_d_n4, eq27_e1838_d_n5, eq27_e1838_d_n6, eq27_e1838_d_n7, eq27_e1838_d_n8, eq27_e1838_d_n9, eq27_e1838_d_n10, eq27_e1838_d_n11, eq27_e1838_d_n12, eq27_e1839_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, eq27_e1841_d_n3, eq27_e1841_d_n4, eq27_e1841_d_n5, eq27_e1841_d_n6, eq27_e1841_d_n7, eq27_e1841_d_n8, eq27_e1841_d_n9, eq27_e1841_d_n10, eq27_e1841_d_n11, eq27_e1841_d_n12, 0.0];
        let eq27_reactive_branch_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            nodes,
            &eq27_reactive_node_derivatives,
            branches,
            &eq27_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq28_e1868, eq28_e1868_d_n3, eq28_e1868_d_n4, eq28_e1868_d_n5, eq28_e1868_d_n6, eq28_e1868_d_n7, eq28_e1868_d_n8, eq28_e1868_d_n9, eq28_e1868_d_n10, eq28_e1868_d_n11, eq28_e1868_d_n12, eq28_e1868_q,) = {
    if ((var_guard492 == 0.0) && ((var_guard837 != 0.0) && (var_guard836 == 0.0))) {
        let eq28_e1852: f64 = (1.0 - var_sigvds);
        let eq28_e1854: f64 = (eq28_e1852 * var_mig);
        let eq28_e1854_d_n3: f64 = (eq28_e1852 * var_mig_dn3);
        let eq28_e1854_d_n4: f64 = (eq28_e1852 * var_mig_dn4);
        let eq28_e1854_d_n5: f64 = (eq28_e1852 * var_mig_dn5);
        let eq28_e1854_d_n6: f64 = (eq28_e1852 * var_mig_dn6);
        let eq28_e1854_d_n7: f64 = (eq28_e1852 * var_mig_dn7);
        let eq28_e1854_d_n8: f64 = (eq28_e1852 * var_mig_dn8);
        let eq28_e1854_d_n9: f64 = (eq28_e1852 * var_mig_dn9);
        let eq28_e1854_d_n10: f64 = (eq28_e1852 * var_mig_dn10);
        let eq28_e1854_d_n11: f64 = (eq28_e1852 * var_mig_dn11);
        let eq28_e1856: f64 = (eq28_e1854 * var_cox);
        let eq28_e1856_d_n3: f64 = (eq28_e1854_d_n3 * var_cox);
        let eq28_e1856_d_n4: f64 = (eq28_e1854_d_n4 * var_cox);
        let eq28_e1856_d_n5: f64 = (eq28_e1854_d_n5 * var_cox);
        let eq28_e1856_d_n6: f64 = (eq28_e1854_d_n6 * var_cox);
        let eq28_e1856_d_n7: f64 = (eq28_e1854_d_n7 * var_cox);
        let eq28_e1856_d_n8: f64 = (eq28_e1854_d_n8 * var_cox);
        let eq28_e1856_d_n9: f64 = (eq28_e1854_d_n9 * var_cox);
        let eq28_e1856_d_n10: f64 = (eq28_e1854_d_n10 * var_cox);
        let eq28_e1856_d_n11: f64 = (eq28_e1854_d_n11 * var_cox);
        let eq28_e1858: f64 = (eq28_e1856 * var_weff);
        let eq28_e1858_d_n3: f64 = (eq28_e1856_d_n3 * var_weff);
        let eq28_e1858_d_n4: f64 = (eq28_e1856_d_n4 * var_weff);
        let eq28_e1858_d_n5: f64 = (eq28_e1856_d_n5 * var_weff);
        let eq28_e1858_d_n6: f64 = (eq28_e1856_d_n6 * var_weff);
        let eq28_e1858_d_n7: f64 = (eq28_e1856_d_n7 * var_weff);
        let eq28_e1858_d_n8: f64 = (eq28_e1856_d_n8 * var_weff);
        let eq28_e1858_d_n9: f64 = (eq28_e1856_d_n9 * var_weff);
        let eq28_e1858_d_n10: f64 = (eq28_e1856_d_n10 * var_weff);
        let eq28_e1858_d_n11: f64 = (eq28_e1856_d_n11 * var_weff);
        let eq28_e1860: f64 = (eq28_e1858 * p.p2);
        let eq28_e1860_d_n3: f64 = (eq28_e1858_d_n3 * p.p2);
        let eq28_e1860_d_n4: f64 = (eq28_e1858_d_n4 * p.p2);
        let eq28_e1860_d_n5: f64 = (eq28_e1858_d_n5 * p.p2);
        let eq28_e1860_d_n6: f64 = (eq28_e1858_d_n6 * p.p2);
        let eq28_e1860_d_n7: f64 = (eq28_e1858_d_n7 * p.p2);
        let eq28_e1860_d_n8: f64 = (eq28_e1858_d_n8 * p.p2);
        let eq28_e1860_d_n9: f64 = (eq28_e1858_d_n9 * p.p2);
        let eq28_e1860_d_n10: f64 = (eq28_e1858_d_n10 * p.p2);
        let eq28_e1860_d_n11: f64 = (eq28_e1858_d_n11 * p.p2);
        let eq28_e1862: f64 = (eq28_e1860 * var_leff);
        let eq28_e1862_d_n3: f64 = (eq28_e1860_d_n3 * var_leff);
        let eq28_e1862_d_n4: f64 = (eq28_e1860_d_n4 * var_leff);
        let eq28_e1862_d_n5: f64 = (eq28_e1860_d_n5 * var_leff);
        let eq28_e1862_d_n6: f64 = (eq28_e1860_d_n6 * var_leff);
        let eq28_e1862_d_n7: f64 = (eq28_e1860_d_n7 * var_leff);
        let eq28_e1862_d_n8: f64 = (eq28_e1860_d_n8 * var_leff);
        let eq28_e1862_d_n9: f64 = (eq28_e1860_d_n9 * var_leff);
        let eq28_e1862_d_n10: f64 = (eq28_e1860_d_n10 * var_leff);
        let eq28_e1862_d_n11: f64 = (eq28_e1860_d_n11 * var_leff);
        let eq28_e1864: f64 = (eq28_e1862 * (nv12 - 0.0));
        let eq28_e1864_d_n3: f64 = (eq28_e1862_d_n3 * (nv12 - 0.0));
        let eq28_e1864_d_n4: f64 = (eq28_e1862_d_n4 * (nv12 - 0.0));
        let eq28_e1864_d_n5: f64 = (eq28_e1862_d_n5 * (nv12 - 0.0));
        let eq28_e1864_d_n6: f64 = (eq28_e1862_d_n6 * (nv12 - 0.0));
        let eq28_e1864_d_n7: f64 = (eq28_e1862_d_n7 * (nv12 - 0.0));
        let eq28_e1864_d_n8: f64 = (eq28_e1862_d_n8 * (nv12 - 0.0));
        let eq28_e1864_d_n9: f64 = (eq28_e1862_d_n9 * (nv12 - 0.0));
        let eq28_e1864_d_n10: f64 = (eq28_e1862_d_n10 * (nv12 - 0.0));
        let eq28_e1864_d_n11: f64 = (eq28_e1862_d_n11 * (nv12 - 0.0));
        let eq28_e1865: f64 = (0.5 * eq28_e1864);
        let eq28_e1865_d_n3: f64 = (0.5 * eq28_e1864_d_n3);
        let eq28_e1865_d_n4: f64 = (0.5 * eq28_e1864_d_n4);
        let eq28_e1865_d_n5: f64 = (0.5 * eq28_e1864_d_n5);
        let eq28_e1865_d_n6: f64 = (0.5 * eq28_e1864_d_n6);
        let eq28_e1865_d_n7: f64 = (0.5 * eq28_e1864_d_n7);
        let eq28_e1865_d_n8: f64 = (0.5 * eq28_e1864_d_n8);
        let eq28_e1865_d_n9: f64 = (0.5 * eq28_e1864_d_n9);
        let eq28_e1865_d_n10: f64 = (0.5 * eq28_e1864_d_n10);
        let eq28_e1865_d_n11: f64 = (0.5 * eq28_e1864_d_n11);
        let eq28_e1865_d_n12: f64 = (0.5 * eq28_e1862);
        let eq28_e1866_q: f64 = eq28_e1865;
        (eq28_e1865, eq28_e1865_d_n3, eq28_e1865_d_n4, eq28_e1865_d_n5, eq28_e1865_d_n6, eq28_e1865_d_n7, eq28_e1865_d_n8, eq28_e1865_d_n9, eq28_e1865_d_n10, eq28_e1865_d_n11, eq28_e1865_d_n12, eq28_e1866_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, eq28_e1868_d_n3, eq28_e1868_d_n4, eq28_e1868_d_n5, eq28_e1868_d_n6, eq28_e1868_d_n7, eq28_e1868_d_n8, eq28_e1868_d_n9, eq28_e1868_d_n10, eq28_e1868_d_n11, eq28_e1868_d_n12, 0.0];
        let eq28_reactive_branch_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq28_reactive_node_derivatives,
            branches,
            &eq28_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_1(
        stamper: &mut GeneratedReactiveStamper<'_>,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        var_cth: f64,
        var_deltemp1: f64,
        var_deltemp1_dn4: f64,
        var_deltemp1_dn5: f64,
        var_devsign: f64,
        var_gth: f64,
        var_guard893: f64,
        var_guard896: f64,
        var_guard897: f64,
        var_pdiss: f64,
        var_pdiss_dn0: f64,
        var_pdiss_dn10: f64,
        var_pdiss_dn11: f64,
        var_pdiss_dn2: f64,
        var_pdiss_dn3: f64,
        var_pdiss_dn4: f64,
        var_pdiss_dn5: f64,
        var_pdiss_dn6: f64,
        var_pdiss_dn7: f64,
        var_pdiss_dn8: f64,
        var_pdiss_dn9: f64,
        var_qbdj: f64,
        var_qbdj_dn10: f64,
        var_qbdj_dn11: f64,
        var_qbdj_dn3: f64,
        var_qbdj_dn4: f64,
        var_qbdj_dn5: f64,
        var_qbdj_dn6: f64,
        var_qbdj_dn7: f64,
        var_qbdj_dn8: f64,
        var_qbdj_dn9: f64,
        var_qbsj: f64,
        var_qbsj_dn10: f64,
        var_qbsj_dn11: f64,
        var_qbsj_dn3: f64,
        var_qbsj_dn4: f64,
        var_qbsj_dn5: f64,
        var_qbsj_dn6: f64,
        var_qbsj_dn7: f64,
        var_qbsj_dn8: f64,
        var_qbsj_dn9: f64,
        var_qde: f64,
        var_qde_dn10: f64,
        var_qde_dn11: f64,
        var_qde_dn3: f64,
        var_qde_dn4: f64,
        var_qde_dn5: f64,
        var_qde_dn6: f64,
        var_qde_dn7: f64,
        var_qde_dn8: f64,
        var_qde_dn9: f64,
        var_qdiagbcp2_1: f64,
        var_qdiagbcp2_1_dn10: f64,
        var_qdiagbcp2_1_dn11: f64,
        var_qdiagbcp2_1_dn3: f64,
        var_qdiagbcp2_1_dn4: f64,
        var_qdiagbcp2_1_dn5: f64,
        var_qdiagbcp2_1_dn6: f64,
        var_qdiagbcp2_1_dn7: f64,
        var_qdiagbcp2_1_dn8: f64,
        var_qdiagbcp2_1_dn9: f64,
        var_qdim_1: f64,
        var_qdim_1_dn10: f64,
        var_qdim_1_dn11: f64,
        var_qdim_1_dn3: f64,
        var_qdim_1_dn4: f64,
        var_qdim_1_dn5: f64,
        var_qdim_1_dn6: f64,
        var_qdim_1_dn7: f64,
        var_qdim_1_dn8: f64,
        var_qdim_1_dn9: f64,
        var_qgiagbcp2_1: f64,
        var_qgiagbcp2_1_dn10: f64,
        var_qgiagbcp2_1_dn11: f64,
        var_qgiagbcp2_1_dn3: f64,
        var_qgiagbcp2_1_dn4: f64,
        var_qgiagbcp2_1_dn5: f64,
        var_qgiagbcp2_1_dn6: f64,
        var_qgiagbcp2_1_dn7: f64,
        var_qgiagbcp2_1_dn8: f64,
        var_qgiagbcp2_1_dn9: f64,
        var_qgim_1: f64,
        var_qgim_1_dn10: f64,
        var_qgim_1_dn11: f64,
        var_qgim_1_dn3: f64,
        var_qgim_1_dn4: f64,
        var_qgim_1_dn5: f64,
        var_qgim_1_dn6: f64,
        var_qgim_1_dn7: f64,
        var_qgim_1_dn8: f64,
        var_qgim_1_dn9: f64,
        var_qovd: f64,
        var_qovd_dn10: f64,
        var_qovd_dn11: f64,
        var_qovd_dn3: f64,
        var_qovd_dn4: f64,
        var_qovd_dn5: f64,
        var_qovd_dn6: f64,
        var_qovd_dn7: f64,
        var_qovd_dn8: f64,
        var_qovd_dn9: f64,
        var_qovs: f64,
        var_qovs_dn10: f64,
        var_qovs_dn11: f64,
        var_qovs_dn3: f64,
        var_qovs_dn4: f64,
        var_qovs_dn5: f64,
        var_qovs_dn6: f64,
        var_qovs_dn7: f64,
        var_qovs_dn8: f64,
        var_qovs_dn9: f64,
        var_qse: f64,
        var_qse_dn10: f64,
        var_qse_dn11: f64,
        var_qse_dn3: f64,
        var_qse_dn4: f64,
        var_qse_dn5: f64,
        var_qse_dn6: f64,
        var_qse_dn7: f64,
        var_qse_dn8: f64,
        var_qse_dn9: f64,
        var_qsiagbcp2_1: f64,
        var_qsiagbcp2_1_dn10: f64,
        var_qsiagbcp2_1_dn11: f64,
        var_qsiagbcp2_1_dn3: f64,
        var_qsiagbcp2_1_dn4: f64,
        var_qsiagbcp2_1_dn5: f64,
        var_qsiagbcp2_1_dn6: f64,
        var_qsiagbcp2_1_dn7: f64,
        var_qsiagbcp2_1_dn8: f64,
        var_qsiagbcp2_1_dn9: f64,
        var_qsim_1: f64,
        var_qsim_1_dn10: f64,
        var_qsim_1_dn11: f64,
        var_qsim_1_dn3: f64,
        var_qsim_1_dn4: f64,
        var_qsim_1_dn5: f64,
        var_qsim_1_dn6: f64,
        var_qsim_1_dn7: f64,
        var_qsim_1_dn8: f64,
        var_qsim_1_dn9: f64,
        var_qsub: f64,
        var_qsub_dn10: f64,
        var_qsub_dn11: f64,
        var_qsub_dn3: f64,
        var_qsub_dn4: f64,
        var_qsub_dn5: f64,
        var_qsub_dn6: f64,
        var_qsub_dn7: f64,
        var_qsub_dn8: f64,
        var_qsub_dn9: f64,
    ) {
        let eq35_e1938_q: f64 = var_qgim_1;
        let eq35_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, var_qgim_1_dn3, var_qgim_1_dn4, var_qgim_1_dn5, var_qgim_1_dn6, var_qgim_1_dn7, var_qgim_1_dn8, var_qgim_1_dn9, var_qgim_1_dn10, var_qgim_1_dn11, 0.0, 0.0];
        let eq35_reactive_branch_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[10]),
            nodes,
            &eq35_reactive_node_derivatives,
            branches,
            &eq35_reactive_branch_derivatives,
            multiplicity,
        );
        let eq36_e1940_q: f64 = var_qgiagbcp2_1;
        let eq36_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, var_qgiagbcp2_1_dn3, var_qgiagbcp2_1_dn4, var_qgiagbcp2_1_dn5, var_qgiagbcp2_1_dn6, var_qgiagbcp2_1_dn7, var_qgiagbcp2_1_dn8, var_qgiagbcp2_1_dn9, var_qgiagbcp2_1_dn10, var_qgiagbcp2_1_dn11, 0.0, 0.0];
        let eq36_reactive_branch_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[11]),
            nodes,
            &eq36_reactive_node_derivatives,
            branches,
            &eq36_reactive_branch_derivatives,
            multiplicity,
        );
        let eq37_e1942_q: f64 = var_qsim_1;
        let eq37_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, var_qsim_1_dn3, var_qsim_1_dn4, var_qsim_1_dn5, var_qsim_1_dn6, var_qsim_1_dn7, var_qsim_1_dn8, var_qsim_1_dn9, var_qsim_1_dn10, var_qsim_1_dn11, 0.0, 0.0];
        let eq37_reactive_branch_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[10]),
            nodes,
            &eq37_reactive_node_derivatives,
            branches,
            &eq37_reactive_branch_derivatives,
            multiplicity,
        );
        let eq38_e1944_q: f64 = var_qsiagbcp2_1;
        let eq38_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, var_qsiagbcp2_1_dn3, var_qsiagbcp2_1_dn4, var_qsiagbcp2_1_dn5, var_qsiagbcp2_1_dn6, var_qsiagbcp2_1_dn7, var_qsiagbcp2_1_dn8, var_qsiagbcp2_1_dn9, var_qsiagbcp2_1_dn10, var_qsiagbcp2_1_dn11, 0.0, 0.0];
        let eq38_reactive_branch_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[11]),
            nodes,
            &eq38_reactive_node_derivatives,
            branches,
            &eq38_reactive_branch_derivatives,
            multiplicity,
        );
        let eq39_e1946_q: f64 = var_qdim_1;
        let eq39_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, var_qdim_1_dn3, var_qdim_1_dn4, var_qdim_1_dn5, var_qdim_1_dn6, var_qdim_1_dn7, var_qdim_1_dn8, var_qdim_1_dn9, var_qdim_1_dn10, var_qdim_1_dn11, 0.0, 0.0];
        let eq39_reactive_branch_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[10]),
            nodes,
            &eq39_reactive_node_derivatives,
            branches,
            &eq39_reactive_branch_derivatives,
            multiplicity,
        );
        let eq40_e1948_q: f64 = var_qdiagbcp2_1;
        let eq40_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, var_qdiagbcp2_1_dn3, var_qdiagbcp2_1_dn4, var_qdiagbcp2_1_dn5, var_qdiagbcp2_1_dn6, var_qdiagbcp2_1_dn7, var_qdiagbcp2_1_dn8, var_qdiagbcp2_1_dn9, var_qdiagbcp2_1_dn10, var_qdiagbcp2_1_dn11, 0.0, 0.0];
        let eq40_reactive_branch_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[11]),
            nodes,
            &eq40_reactive_node_derivatives,
            branches,
            &eq40_reactive_branch_derivatives,
            multiplicity,
        );
        let eq41_e1950: f64 = (-var_devsign);
        let eq41_e1952: f64 = (eq41_e1950 * var_qovs);
        let eq41_e1952_d_n3: f64 = (eq41_e1950 * var_qovs_dn3);
        let eq41_e1952_d_n4: f64 = (eq41_e1950 * var_qovs_dn4);
        let eq41_e1952_d_n5: f64 = (eq41_e1950 * var_qovs_dn5);
        let eq41_e1952_d_n6: f64 = (eq41_e1950 * var_qovs_dn6);
        let eq41_e1952_d_n7: f64 = (eq41_e1950 * var_qovs_dn7);
        let eq41_e1952_d_n8: f64 = (eq41_e1950 * var_qovs_dn8);
        let eq41_e1952_d_n9: f64 = (eq41_e1950 * var_qovs_dn9);
        let eq41_e1952_d_n10: f64 = (eq41_e1950 * var_qovs_dn10);
        let eq41_e1952_d_n11: f64 = (eq41_e1950 * var_qovs_dn11);
        let eq41_e1953_q: f64 = eq41_e1952;
        let eq41_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, eq41_e1952_d_n3, eq41_e1952_d_n4, eq41_e1952_d_n5, eq41_e1952_d_n6, eq41_e1952_d_n7, eq41_e1952_d_n8, eq41_e1952_d_n9, eq41_e1952_d_n10, eq41_e1952_d_n11, 0.0, 0.0];
        let eq41_reactive_branch_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq41_reactive_node_derivatives,
            branches,
            &eq41_reactive_branch_derivatives,
            multiplicity,
        );
        let eq42_e1955: f64 = (-var_devsign);
        let eq42_e1957: f64 = (eq42_e1955 * var_qovd);
        let eq42_e1957_d_n3: f64 = (eq42_e1955 * var_qovd_dn3);
        let eq42_e1957_d_n4: f64 = (eq42_e1955 * var_qovd_dn4);
        let eq42_e1957_d_n5: f64 = (eq42_e1955 * var_qovd_dn5);
        let eq42_e1957_d_n6: f64 = (eq42_e1955 * var_qovd_dn6);
        let eq42_e1957_d_n7: f64 = (eq42_e1955 * var_qovd_dn7);
        let eq42_e1957_d_n8: f64 = (eq42_e1955 * var_qovd_dn8);
        let eq42_e1957_d_n9: f64 = (eq42_e1955 * var_qovd_dn9);
        let eq42_e1957_d_n10: f64 = (eq42_e1955 * var_qovd_dn10);
        let eq42_e1957_d_n11: f64 = (eq42_e1955 * var_qovd_dn11);
        let eq42_e1958_q: f64 = eq42_e1957;
        let eq42_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, eq42_e1957_d_n3, eq42_e1957_d_n4, eq42_e1957_d_n5, eq42_e1957_d_n6, eq42_e1957_d_n7, eq42_e1957_d_n8, eq42_e1957_d_n9, eq42_e1957_d_n10, eq42_e1957_d_n11, 0.0, 0.0];
        let eq42_reactive_branch_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[6]),
            nodes,
            &eq42_reactive_node_derivatives,
            branches,
            &eq42_reactive_branch_derivatives,
            multiplicity,
        );
        let eq45_e1969_q: f64 = var_qsub;
        let eq45_e1970: f64 = (var_devsign * var_qsub);
        let eq45_e1970_d_n3: f64 = (var_devsign * var_qsub_dn3);
        let eq45_e1970_d_n4: f64 = (var_devsign * var_qsub_dn4);
        let eq45_e1970_d_n5: f64 = (var_devsign * var_qsub_dn5);
        let eq45_e1970_d_n6: f64 = (var_devsign * var_qsub_dn6);
        let eq45_e1970_d_n7: f64 = (var_devsign * var_qsub_dn7);
        let eq45_e1970_d_n8: f64 = (var_devsign * var_qsub_dn8);
        let eq45_e1970_d_n9: f64 = (var_devsign * var_qsub_dn9);
        let eq45_e1970_d_n10: f64 = (var_devsign * var_qsub_dn10);
        let eq45_e1970_d_n11: f64 = (var_devsign * var_qsub_dn11);
        let eq45_e1970_q: f64 = (var_devsign * eq45_e1969_q);
        let eq45_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, eq45_e1970_d_n3, eq45_e1970_d_n4, eq45_e1970_d_n5, eq45_e1970_d_n6, eq45_e1970_d_n7, eq45_e1970_d_n8, eq45_e1970_d_n9, eq45_e1970_d_n10, eq45_e1970_d_n11, 0.0, 0.0];
        let eq45_reactive_branch_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[10]),
            nodes,
            &eq45_reactive_node_derivatives,
            branches,
            &eq45_reactive_branch_derivatives,
            multiplicity,
        );
        let eq46_e1972_q: f64 = var_qde;
        let eq46_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, var_qde_dn3, var_qde_dn4, var_qde_dn5, var_qde_dn6, var_qde_dn7, var_qde_dn8, var_qde_dn9, var_qde_dn10, var_qde_dn11, 0.0, 0.0];
        let eq46_reactive_branch_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[3]),
            nodes,
            &eq46_reactive_node_derivatives,
            branches,
            &eq46_reactive_branch_derivatives,
            multiplicity,
        );
        let eq47_e1974_q: f64 = var_qse;
        let eq47_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, var_qse_dn3, var_qse_dn4, var_qse_dn5, var_qse_dn6, var_qse_dn7, var_qse_dn8, var_qse_dn9, var_qse_dn10, var_qse_dn11, 0.0, 0.0];
        let eq47_reactive_branch_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[3]),
            nodes,
            &eq47_reactive_node_derivatives,
            branches,
            &eq47_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq67_e2103, eq67_e2103_d_n0, eq67_e2103_d_n2, eq67_e2103_d_n3, eq67_e2103_d_n4, eq67_e2103_d_n5, eq67_e2103_d_n6, eq67_e2103_d_n7, eq67_e2103_d_n8, eq67_e2103_d_n9, eq67_e2103_d_n10, eq67_e2103_d_n11, eq67_e2103_q, eq67_e2103_q_d_n4, eq67_e2103_q_d_n5,) = {
    if (((var_guard893 != 0.0) && (var_guard896 != 0.0)) && (var_guard897 != 0.0)) {
        let eq67_e2094: f64 = (var_deltemp1 * var_gth);
        let eq67_e2094_d_n4: f64 = (var_deltemp1_dn4 * var_gth);
        let eq67_e2094_d_n5: f64 = (var_deltemp1_dn5 * var_gth);
        let eq67_e2097: f64 = (var_deltemp1 * var_cth);
        let eq67_e2097_d_n4: f64 = (var_deltemp1_dn4 * var_cth);
        let eq67_e2097_d_n5: f64 = (var_deltemp1_dn5 * var_cth);
        let eq67_e2098_q: f64 = eq67_e2097;
        let eq67_e2099: f64 = (eq67_e2094 + eq67_e2097);
        let eq67_e2099_d_n4: f64 = (eq67_e2094_d_n4 + eq67_e2097_d_n4);
        let eq67_e2099_d_n5: f64 = (eq67_e2094_d_n5 + eq67_e2097_d_n5);
        let eq67_e2099_q: f64 = eq67_e2098_q;
        let eq67_e2101: f64 = (eq67_e2099 - var_pdiss);
        let eq67_e2101_d_n4: f64 = (eq67_e2099_d_n4 - var_pdiss_dn4);
        let eq67_e2101_d_n5: f64 = (eq67_e2099_d_n5 - var_pdiss_dn5);
        let eq67_e2101_q: f64 = eq67_e2099_q;
        (eq67_e2101, (-var_pdiss_dn0), (-var_pdiss_dn2), (-var_pdiss_dn3), eq67_e2101_d_n4, eq67_e2101_d_n5, (-var_pdiss_dn6), (-var_pdiss_dn7), (-var_pdiss_dn8), (-var_pdiss_dn9), (-var_pdiss_dn10), (-var_pdiss_dn11), eq67_e2101_q, eq67_e2097_d_n4, eq67_e2097_d_n5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node2(
            Some(nodes[4]),
            None,
            nodes[4],
            multiplicity * (eq67_e2103_q_d_n4),
            nodes[5],
            multiplicity * (eq67_e2103_q_d_n5),
        );
        let (eq68_e2121, eq68_e2121_d_n0, eq68_e2121_d_n2, eq68_e2121_d_n3, eq68_e2121_d_n4, eq68_e2121_d_n5, eq68_e2121_d_n6, eq68_e2121_d_n7, eq68_e2121_d_n8, eq68_e2121_d_n9, eq68_e2121_d_n10, eq68_e2121_d_n11, eq68_e2121_q, eq68_e2121_q_d_n4, eq68_e2121_q_d_n5,) = {
    if (((var_guard893 != 0.0) && (var_guard896 != 0.0)) && (var_guard897 == 0.0)) {
        let eq68_e2112: f64 = (var_deltemp1 * var_gth);
        let eq68_e2112_d_n4: f64 = (var_deltemp1_dn4 * var_gth);
        let eq68_e2112_d_n5: f64 = (var_deltemp1_dn5 * var_gth);
        let eq68_e2115: f64 = (var_deltemp1 * var_cth);
        let eq68_e2115_d_n4: f64 = (var_deltemp1_dn4 * var_cth);
        let eq68_e2115_d_n5: f64 = (var_deltemp1_dn5 * var_cth);
        let eq68_e2116_q: f64 = eq68_e2115;
        let eq68_e2117: f64 = (eq68_e2112 + eq68_e2115);
        let eq68_e2117_d_n4: f64 = (eq68_e2112_d_n4 + eq68_e2115_d_n4);
        let eq68_e2117_d_n5: f64 = (eq68_e2112_d_n5 + eq68_e2115_d_n5);
        let eq68_e2117_q: f64 = eq68_e2116_q;
        let eq68_e2119: f64 = (eq68_e2117 - var_pdiss);
        let eq68_e2119_d_n4: f64 = (eq68_e2117_d_n4 - var_pdiss_dn4);
        let eq68_e2119_d_n5: f64 = (eq68_e2117_d_n5 - var_pdiss_dn5);
        let eq68_e2119_q: f64 = eq68_e2117_q;
        (eq68_e2119, (-var_pdiss_dn0), (-var_pdiss_dn2), (-var_pdiss_dn3), eq68_e2119_d_n4, eq68_e2119_d_n5, (-var_pdiss_dn6), (-var_pdiss_dn7), (-var_pdiss_dn8), (-var_pdiss_dn9), (-var_pdiss_dn10), (-var_pdiss_dn11), eq68_e2119_q, eq68_e2115_d_n4, eq68_e2115_d_n5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node2(
            Some(nodes[5]),
            None,
            nodes[4],
            multiplicity * (eq68_e2121_q_d_n4),
            nodes[5],
            multiplicity * (eq68_e2121_q_d_n5),
        );
        let (eq69_e2137, eq69_e2137_d_n0, eq69_e2137_d_n2, eq69_e2137_d_n3, eq69_e2137_d_n4, eq69_e2137_d_n5, eq69_e2137_d_n6, eq69_e2137_d_n7, eq69_e2137_d_n8, eq69_e2137_d_n9, eq69_e2137_d_n10, eq69_e2137_d_n11, eq69_e2137_q, eq69_e2137_q_d_n4, eq69_e2137_q_d_n5,) = {
    if ((var_guard893 != 0.0) && (var_guard896 == 0.0)) {
        let eq69_e2128: f64 = (var_deltemp1 * var_gth);
        let eq69_e2128_d_n4: f64 = (var_deltemp1_dn4 * var_gth);
        let eq69_e2128_d_n5: f64 = (var_deltemp1_dn5 * var_gth);
        let eq69_e2131: f64 = (var_deltemp1 * var_cth);
        let eq69_e2131_d_n4: f64 = (var_deltemp1_dn4 * var_cth);
        let eq69_e2131_d_n5: f64 = (var_deltemp1_dn5 * var_cth);
        let eq69_e2132_q: f64 = eq69_e2131;
        let eq69_e2133: f64 = (eq69_e2128 + eq69_e2131);
        let eq69_e2133_d_n4: f64 = (eq69_e2128_d_n4 + eq69_e2131_d_n4);
        let eq69_e2133_d_n5: f64 = (eq69_e2128_d_n5 + eq69_e2131_d_n5);
        let eq69_e2133_q: f64 = eq69_e2132_q;
        let eq69_e2135: f64 = (eq69_e2133 - var_pdiss);
        let eq69_e2135_d_n4: f64 = (eq69_e2133_d_n4 - var_pdiss_dn4);
        let eq69_e2135_d_n5: f64 = (eq69_e2133_d_n5 - var_pdiss_dn5);
        let eq69_e2135_q: f64 = eq69_e2133_q;
        (eq69_e2135, (-var_pdiss_dn0), (-var_pdiss_dn2), (-var_pdiss_dn3), eq69_e2135_d_n4, eq69_e2135_d_n5, (-var_pdiss_dn6), (-var_pdiss_dn7), (-var_pdiss_dn8), (-var_pdiss_dn9), (-var_pdiss_dn10), (-var_pdiss_dn11), eq69_e2135_q, eq69_e2131_d_n4, eq69_e2131_d_n5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node2(
            Some(nodes[5]),
            None,
            nodes[4],
            multiplicity * (eq69_e2137_q_d_n4),
            nodes[5],
            multiplicity * (eq69_e2137_q_d_n5),
        );
        let eq80_e2212_q: f64 = var_qbsj;
        let eq80_e2213: f64 = (var_devsign * var_qbsj);
        let eq80_e2213_d_n3: f64 = (var_devsign * var_qbsj_dn3);
        let eq80_e2213_d_n4: f64 = (var_devsign * var_qbsj_dn4);
        let eq80_e2213_d_n5: f64 = (var_devsign * var_qbsj_dn5);
        let eq80_e2213_d_n6: f64 = (var_devsign * var_qbsj_dn6);
        let eq80_e2213_d_n7: f64 = (var_devsign * var_qbsj_dn7);
        let eq80_e2213_d_n8: f64 = (var_devsign * var_qbsj_dn8);
        let eq80_e2213_d_n9: f64 = (var_devsign * var_qbsj_dn9);
        let eq80_e2213_d_n10: f64 = (var_devsign * var_qbsj_dn10);
        let eq80_e2213_d_n11: f64 = (var_devsign * var_qbsj_dn11);
        let eq80_e2213_q: f64 = (var_devsign * eq80_e2212_q);
        let eq80_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, eq80_e2213_d_n3, eq80_e2213_d_n4, eq80_e2213_d_n5, eq80_e2213_d_n6, eq80_e2213_d_n7, eq80_e2213_d_n8, eq80_e2213_d_n9, eq80_e2213_d_n10, eq80_e2213_d_n11, 0.0, 0.0];
        let eq80_reactive_branch_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[7]),
            nodes,
            &eq80_reactive_node_derivatives,
            branches,
            &eq80_reactive_branch_derivatives,
            multiplicity,
        );
        let eq81_e2216_q: f64 = var_qbdj;
        let eq81_e2217: f64 = (var_devsign * var_qbdj);
        let eq81_e2217_d_n3: f64 = (var_devsign * var_qbdj_dn3);
        let eq81_e2217_d_n4: f64 = (var_devsign * var_qbdj_dn4);
        let eq81_e2217_d_n5: f64 = (var_devsign * var_qbdj_dn5);
        let eq81_e2217_d_n6: f64 = (var_devsign * var_qbdj_dn6);
        let eq81_e2217_d_n7: f64 = (var_devsign * var_qbdj_dn7);
        let eq81_e2217_d_n8: f64 = (var_devsign * var_qbdj_dn8);
        let eq81_e2217_d_n9: f64 = (var_devsign * var_qbdj_dn9);
        let eq81_e2217_d_n10: f64 = (var_devsign * var_qbdj_dn10);
        let eq81_e2217_d_n11: f64 = (var_devsign * var_qbdj_dn11);
        let eq81_e2217_q: f64 = (var_devsign * eq81_e2216_q);
        let eq81_reactive_node_derivatives: [f64; 14] = [0.0, 0.0, 0.0, eq81_e2217_d_n3, eq81_e2217_d_n4, eq81_e2217_d_n5, eq81_e2217_d_n6, eq81_e2217_d_n7, eq81_e2217_d_n8, eq81_e2217_d_n9, eq81_e2217_d_n10, eq81_e2217_d_n11, 0.0, 0.0];
        let eq81_reactive_branch_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[6]),
            nodes,
            &eq81_reactive_node_derivatives,
            branches,
            &eq81_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
