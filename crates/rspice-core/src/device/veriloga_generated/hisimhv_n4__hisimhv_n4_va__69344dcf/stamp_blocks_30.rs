#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_102(
        locals: &mut StampLocals,
    ) {
        let (assign32840_e37313, assign32840_e37313_d_n0, assign32840_e37313_d_n2, assign32840_e37313_d_n4, assign32840_e37313_d_n5, assign32840_e37313_d_n6, assign32840_e37313_d_n7, assign32840_e37313_d_n8, assign32840_e37313_d_n9, assign32840_e37313_d_n10, assign32840_e37313_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard779 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_subl, locals.var_q_subl_dn0, locals.var_q_subl_dn2, locals.var_q_subl_dn4, locals.var_q_subl_dn5, locals.var_q_subl_dn6, locals.var_q_subl_dn7, locals.var_q_subl_dn8, locals.var_q_subl_dn9, locals.var_q_subl_dn10, locals.var_q_subl_dn13,)
    }
};
        locals.var_q_subl = assign32840_e37313;
        locals.var_q_subl_dn0 = assign32840_e37313_d_n0;
        locals.var_q_subl_dn2 = assign32840_e37313_d_n2;
        locals.var_q_subl_dn4 = assign32840_e37313_d_n4;
        locals.var_q_subl_dn5 = assign32840_e37313_d_n5;
        locals.var_q_subl_dn6 = assign32840_e37313_d_n6;
        locals.var_q_subl_dn7 = assign32840_e37313_d_n7;
        locals.var_q_subl_dn8 = assign32840_e37313_d_n8;
        locals.var_q_subl_dn9 = assign32840_e37313_d_n9;
        locals.var_q_subl_dn10 = assign32840_e37313_d_n10;
        locals.var_q_subl_dn13 = assign32840_e37313_d_n13;
        locals.var_q_subl_rv = 0.0;

        let (assign32850_e37327, assign32850_e37327_d_n0, assign32850_e37327_d_n2, assign32850_e37327_d_n4, assign32850_e37327_d_n5, assign32850_e37327_d_n6, assign32850_e37327_d_n7, assign32850_e37327_d_n8, assign32850_e37327_d_n9, assign32850_e37327_d_n10, assign32850_e37327_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard779 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_sl_dep, locals.var_q_sl_dep_dn0, locals.var_q_sl_dep_dn2, locals.var_q_sl_dep_dn4, locals.var_q_sl_dep_dn5, locals.var_q_sl_dep_dn6, locals.var_q_sl_dep_dn7, locals.var_q_sl_dep_dn8, locals.var_q_sl_dep_dn9, locals.var_q_sl_dep_dn10, locals.var_q_sl_dep_dn13,)
    }
};
        locals.var_q_sl_dep = assign32850_e37327;
        locals.var_q_sl_dep_dn0 = assign32850_e37327_d_n0;
        locals.var_q_sl_dep_dn2 = assign32850_e37327_d_n2;
        locals.var_q_sl_dep_dn4 = assign32850_e37327_d_n4;
        locals.var_q_sl_dep_dn5 = assign32850_e37327_d_n5;
        locals.var_q_sl_dep_dn6 = assign32850_e37327_d_n6;
        locals.var_q_sl_dep_dn7 = assign32850_e37327_d_n7;
        locals.var_q_sl_dep_dn8 = assign32850_e37327_d_n8;
        locals.var_q_sl_dep_dn9 = assign32850_e37327_d_n9;
        locals.var_q_sl_dep_dn10 = assign32850_e37327_d_n10;
        locals.var_q_sl_dep_dn13 = assign32850_e37327_d_n13;
        locals.var_q_sl_dep_rv = 0.0;

        let (assign32860_e37364, assign32860_e37364_d_n0, assign32860_e37364_d_n2, assign32860_e37364_d_n4, assign32860_e37364_d_n5, assign32860_e37364_d_n6, assign32860_e37364_d_n7, assign32860_e37364_d_n8, assign32860_e37364_d_n9, assign32860_e37364_d_n10, assign32860_e37364_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard779 == 0.0)) {
        let assign32860_e37342: f64 = (-locals.var_t1);
        let assign32860_e37345: f64 = (-locals.var_beta);
        let assign32860_e37348: f64 = (locals.var_phi_sl_dep - locals.var_vbscl__blk435);
        let assign32860_e37349: f64 = (assign32860_e37345 * assign32860_e37348);
        let assign32860_e37350: f64 = (assign32860_e37349).exp();
        let assign32860_e37352: f64 = (-locals.var_beta);
        let assign32860_e37355: f64 = (locals.var_phi_bl_dep - locals.var_vbscl__blk435);
        let assign32860_e37356: f64 = (assign32860_e37352 * assign32860_e37355);
        let assign32860_e37357: f64 = (assign32860_e37356).exp();
        let assign32860_e37358: f64 = (assign32860_e37350 - assign32860_e37357);
        let assign32860_e37359: f64 = (locals.var_cnst1 * assign32860_e37358);
        let assign32860_e37360: f64 = (assign32860_e37342 + assign32860_e37359);
        let assign32860_e37361: f64 = (assign32860_e37360).sqrt();
        let assign32860_e37362: f64 = (locals.var_cnst0 * assign32860_e37361);
        (assign32860_e37362, ((locals.var_cnst0_dn0 * assign32860_e37361) + (locals.var_cnst0 * (((-locals.var_t1_dn0) + ((locals.var_cnst1_dn0 * assign32860_e37358) + (locals.var_cnst1 * ((assign32860_e37350 * (((-locals.var_beta_dn0) * assign32860_e37348) + (assign32860_e37345 * (locals.var_phi_sl_dep_dn0 - locals.var_vbscl__blk435_dn0)))) - (assign32860_e37357 * (((-locals.var_beta_dn0) * assign32860_e37355) + (assign32860_e37352 * (locals.var_phi_bl_dep_dn0 - locals.var_vbscl__blk435_dn0)))))))) / (2.0 * assign32860_e37361)))), ((locals.var_cnst0_dn2 * assign32860_e37361) + (locals.var_cnst0 * (((-locals.var_t1_dn2) + ((locals.var_cnst1_dn2 * assign32860_e37358) + (locals.var_cnst1 * ((assign32860_e37350 * (((-locals.var_beta_dn2) * assign32860_e37348) + (assign32860_e37345 * (locals.var_phi_sl_dep_dn2 - locals.var_vbscl__blk435_dn2)))) - (assign32860_e37357 * (((-locals.var_beta_dn2) * assign32860_e37355) + (assign32860_e37352 * (locals.var_phi_bl_dep_dn2 - locals.var_vbscl__blk435_dn2)))))))) / (2.0 * assign32860_e37361)))), ((locals.var_cnst0_dn4 * assign32860_e37361) + (locals.var_cnst0 * (((-locals.var_t1_dn4) + ((locals.var_cnst1_dn4 * assign32860_e37358) + (locals.var_cnst1 * ((assign32860_e37350 * (((-locals.var_beta_dn4) * assign32860_e37348) + (assign32860_e37345 * (locals.var_phi_sl_dep_dn4 - locals.var_vbscl__blk435_dn4)))) - (assign32860_e37357 * (((-locals.var_beta_dn4) * assign32860_e37355) + (assign32860_e37352 * (locals.var_phi_bl_dep_dn4 - locals.var_vbscl__blk435_dn4)))))))) / (2.0 * assign32860_e37361)))), ((locals.var_cnst0_dn5 * assign32860_e37361) + (locals.var_cnst0 * (((-locals.var_t1_dn5) + ((locals.var_cnst1_dn5 * assign32860_e37358) + (locals.var_cnst1 * ((assign32860_e37350 * (((-locals.var_beta_dn5) * assign32860_e37348) + (assign32860_e37345 * (locals.var_phi_sl_dep_dn5 - locals.var_vbscl__blk435_dn5)))) - (assign32860_e37357 * (((-locals.var_beta_dn5) * assign32860_e37355) + (assign32860_e37352 * (locals.var_phi_bl_dep_dn5 - locals.var_vbscl__blk435_dn5)))))))) / (2.0 * assign32860_e37361)))), ((locals.var_cnst0_dn6 * assign32860_e37361) + (locals.var_cnst0 * (((-locals.var_t1_dn6) + ((locals.var_cnst1_dn6 * assign32860_e37358) + (locals.var_cnst1 * ((assign32860_e37350 * (((-locals.var_beta_dn6) * assign32860_e37348) + (assign32860_e37345 * (locals.var_phi_sl_dep_dn6 - locals.var_vbscl__blk435_dn6)))) - (assign32860_e37357 * (((-locals.var_beta_dn6) * assign32860_e37355) + (assign32860_e37352 * (locals.var_phi_bl_dep_dn6 - locals.var_vbscl__blk435_dn6)))))))) / (2.0 * assign32860_e37361)))), ((locals.var_cnst0_dn7 * assign32860_e37361) + (locals.var_cnst0 * (((-locals.var_t1_dn7) + ((locals.var_cnst1_dn7 * assign32860_e37358) + (locals.var_cnst1 * ((assign32860_e37350 * (((-locals.var_beta_dn7) * assign32860_e37348) + (assign32860_e37345 * (locals.var_phi_sl_dep_dn7 - locals.var_vbscl__blk435_dn7)))) - (assign32860_e37357 * (((-locals.var_beta_dn7) * assign32860_e37355) + (assign32860_e37352 * (locals.var_phi_bl_dep_dn7 - locals.var_vbscl__blk435_dn7)))))))) / (2.0 * assign32860_e37361)))), ((locals.var_cnst0_dn8 * assign32860_e37361) + (locals.var_cnst0 * (((-locals.var_t1_dn8) + ((locals.var_cnst1_dn8 * assign32860_e37358) + (locals.var_cnst1 * ((assign32860_e37350 * (((-locals.var_beta_dn8) * assign32860_e37348) + (assign32860_e37345 * (locals.var_phi_sl_dep_dn8 - locals.var_vbscl__blk435_dn8)))) - (assign32860_e37357 * (((-locals.var_beta_dn8) * assign32860_e37355) + (assign32860_e37352 * (locals.var_phi_bl_dep_dn8 - locals.var_vbscl__blk435_dn8)))))))) / (2.0 * assign32860_e37361)))), ((locals.var_cnst0_dn9 * assign32860_e37361) + (locals.var_cnst0 * (((-locals.var_t1_dn9) + ((locals.var_cnst1_dn9 * assign32860_e37358) + (locals.var_cnst1 * ((assign32860_e37350 * (((-locals.var_beta_dn9) * assign32860_e37348) + (assign32860_e37345 * (locals.var_phi_sl_dep_dn9 - locals.var_vbscl__blk435_dn9)))) - (assign32860_e37357 * (((-locals.var_beta_dn9) * assign32860_e37355) + (assign32860_e37352 * (locals.var_phi_bl_dep_dn9 - locals.var_vbscl__blk435_dn9)))))))) / (2.0 * assign32860_e37361)))), ((locals.var_cnst0_dn10 * assign32860_e37361) + (locals.var_cnst0 * (((-locals.var_t1_dn10) + ((locals.var_cnst1_dn10 * assign32860_e37358) + (locals.var_cnst1 * ((assign32860_e37350 * (((-locals.var_beta_dn10) * assign32860_e37348) + (assign32860_e37345 * (locals.var_phi_sl_dep_dn10 - locals.var_vbscl__blk435_dn10)))) - (assign32860_e37357 * (((-locals.var_beta_dn10) * assign32860_e37355) + (assign32860_e37352 * (locals.var_phi_bl_dep_dn10 - locals.var_vbscl__blk435_dn10)))))))) / (2.0 * assign32860_e37361)))), ((locals.var_cnst0_dn13 * assign32860_e37361) + (locals.var_cnst0 * (((-locals.var_t1_dn13) + ((locals.var_cnst1_dn13 * assign32860_e37358) + (locals.var_cnst1 * ((assign32860_e37350 * (((-locals.var_beta_dn13) * assign32860_e37348) + (assign32860_e37345 * (locals.var_phi_sl_dep_dn13 - locals.var_vbscl__blk435_dn13)))) - (assign32860_e37357 * (((-locals.var_beta_dn13) * assign32860_e37355) + (assign32860_e37352 * (locals.var_phi_bl_dep_dn13 - locals.var_vbscl__blk435_dn13)))))))) / (2.0 * assign32860_e37361)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign32860_e37364;
        locals.var_t3_dn0 = assign32860_e37364_d_n0;
        locals.var_t3_dn2 = assign32860_e37364_d_n2;
        locals.var_t3_dn4 = assign32860_e37364_d_n4;
        locals.var_t3_dn5 = assign32860_e37364_d_n5;
        locals.var_t3_dn6 = assign32860_e37364_d_n6;
        locals.var_t3_dn7 = assign32860_e37364_d_n7;
        locals.var_t3_dn8 = assign32860_e37364_d_n8;
        locals.var_t3_dn9 = assign32860_e37364_d_n9;
        locals.var_t3_dn10 = assign32860_e37364_d_n10;
        locals.var_t3_dn13 = assign32860_e37364_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign32870_e37385, assign32870_e37385_d_n0, assign32870_e37385_d_n2, assign32870_e37385_d_n4, assign32870_e37385_d_n5, assign32870_e37385_d_n6, assign32870_e37385_d_n7, assign32870_e37385_d_n8, assign32870_e37385_d_n9, assign32870_e37385_d_n10, assign32870_e37385_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard779 == 0.0)) {
        let assign32870_e37380: f64 = (-locals.var_t1);
        let assign32870_e37381: f64 = (assign32870_e37380).sqrt();
        let assign32870_e37382: f64 = (locals.var_cnst0 * assign32870_e37381);
        let assign32870_e37383: f64 = (locals.var_t3 - assign32870_e37382);
        (assign32870_e37383, (locals.var_t3_dn0 - ((locals.var_cnst0_dn0 * assign32870_e37381) + (locals.var_cnst0 * ((-locals.var_t1_dn0) / (2.0 * assign32870_e37381))))), (locals.var_t3_dn2 - ((locals.var_cnst0_dn2 * assign32870_e37381) + (locals.var_cnst0 * ((-locals.var_t1_dn2) / (2.0 * assign32870_e37381))))), (locals.var_t3_dn4 - ((locals.var_cnst0_dn4 * assign32870_e37381) + (locals.var_cnst0 * ((-locals.var_t1_dn4) / (2.0 * assign32870_e37381))))), (locals.var_t3_dn5 - ((locals.var_cnst0_dn5 * assign32870_e37381) + (locals.var_cnst0 * ((-locals.var_t1_dn5) / (2.0 * assign32870_e37381))))), (locals.var_t3_dn6 - ((locals.var_cnst0_dn6 * assign32870_e37381) + (locals.var_cnst0 * ((-locals.var_t1_dn6) / (2.0 * assign32870_e37381))))), (locals.var_t3_dn7 - ((locals.var_cnst0_dn7 * assign32870_e37381) + (locals.var_cnst0 * ((-locals.var_t1_dn7) / (2.0 * assign32870_e37381))))), (locals.var_t3_dn8 - ((locals.var_cnst0_dn8 * assign32870_e37381) + (locals.var_cnst0 * ((-locals.var_t1_dn8) / (2.0 * assign32870_e37381))))), (locals.var_t3_dn9 - ((locals.var_cnst0_dn9 * assign32870_e37381) + (locals.var_cnst0 * ((-locals.var_t1_dn9) / (2.0 * assign32870_e37381))))), (locals.var_t3_dn10 - ((locals.var_cnst0_dn10 * assign32870_e37381) + (locals.var_cnst0 * ((-locals.var_t1_dn10) / (2.0 * assign32870_e37381))))), (locals.var_t3_dn13 - ((locals.var_cnst0_dn13 * assign32870_e37381) + (locals.var_cnst0 * ((-locals.var_t1_dn13) / (2.0 * assign32870_e37381))))),)
    } else {
        (locals.var_q_subl, locals.var_q_subl_dn0, locals.var_q_subl_dn2, locals.var_q_subl_dn4, locals.var_q_subl_dn5, locals.var_q_subl_dn6, locals.var_q_subl_dn7, locals.var_q_subl_dn8, locals.var_q_subl_dn9, locals.var_q_subl_dn10, locals.var_q_subl_dn13,)
    }
};
        locals.var_q_subl = assign32870_e37385;
        locals.var_q_subl_dn0 = assign32870_e37385_d_n0;
        locals.var_q_subl_dn2 = assign32870_e37385_d_n2;
        locals.var_q_subl_dn4 = assign32870_e37385_d_n4;
        locals.var_q_subl_dn5 = assign32870_e37385_d_n5;
        locals.var_q_subl_dn6 = assign32870_e37385_d_n6;
        locals.var_q_subl_dn7 = assign32870_e37385_d_n7;
        locals.var_q_subl_dn8 = assign32870_e37385_d_n8;
        locals.var_q_subl_dn9 = assign32870_e37385_d_n9;
        locals.var_q_subl_dn10 = assign32870_e37385_d_n10;
        locals.var_q_subl_dn13 = assign32870_e37385_d_n13;
        locals.var_q_subl_rv = 0.0;

        let (assign32880_e37409, assign32880_e37409_d_n0, assign32880_e37409_d_n2, assign32880_e37409_d_n4, assign32880_e37409_d_n5, assign32880_e37409_d_n6, assign32880_e37409_d_n7, assign32880_e37409_d_n8, assign32880_e37409_d_n9, assign32880_e37409_d_n10, assign32880_e37409_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard779 == 0.0)) {
        let assign32880_e37401: f64 = (locals.var_t2 - 1.0);
        let assign32880_e37403: f64 = (assign32880_e37401 - locals.var_t1);
        let assign32880_e37405: f64 = (assign32880_e37403 + 1e-15);
        let assign32880_e37406: f64 = (assign32880_e37405).sqrt();
        let assign32880_e37407: f64 = (locals.var_cnst0 * assign32880_e37406);
        (assign32880_e37407, ((locals.var_cnst0_dn0 * assign32880_e37406) + (locals.var_cnst0 * ((locals.var_t2_dn0 - locals.var_t1_dn0) / (2.0 * assign32880_e37406)))), ((locals.var_cnst0_dn2 * assign32880_e37406) + (locals.var_cnst0 * ((locals.var_t2_dn2 - locals.var_t1_dn2) / (2.0 * assign32880_e37406)))), ((locals.var_cnst0_dn4 * assign32880_e37406) + (locals.var_cnst0 * ((locals.var_t2_dn4 - locals.var_t1_dn4) / (2.0 * assign32880_e37406)))), ((locals.var_cnst0_dn5 * assign32880_e37406) + (locals.var_cnst0 * ((locals.var_t2_dn5 - locals.var_t1_dn5) / (2.0 * assign32880_e37406)))), ((locals.var_cnst0_dn6 * assign32880_e37406) + (locals.var_cnst0 * ((locals.var_t2_dn6 - locals.var_t1_dn6) / (2.0 * assign32880_e37406)))), ((locals.var_cnst0_dn7 * assign32880_e37406) + (locals.var_cnst0 * ((locals.var_t2_dn7 - locals.var_t1_dn7) / (2.0 * assign32880_e37406)))), ((locals.var_cnst0_dn8 * assign32880_e37406) + (locals.var_cnst0 * ((locals.var_t2_dn8 - locals.var_t1_dn8) / (2.0 * assign32880_e37406)))), ((locals.var_cnst0_dn9 * assign32880_e37406) + (locals.var_cnst0 * ((locals.var_t2_dn9 - locals.var_t1_dn9) / (2.0 * assign32880_e37406)))), ((locals.var_cnst0_dn10 * assign32880_e37406) + (locals.var_cnst0 * ((locals.var_t2_dn10 - locals.var_t1_dn10) / (2.0 * assign32880_e37406)))), ((locals.var_cnst0_dn13 * assign32880_e37406) + (locals.var_cnst0 * ((locals.var_t2_dn13 - locals.var_t1_dn13) / (2.0 * assign32880_e37406)))),)
    } else {
        (locals.var_q_sl_dep, locals.var_q_sl_dep_dn0, locals.var_q_sl_dep_dn2, locals.var_q_sl_dep_dn4, locals.var_q_sl_dep_dn5, locals.var_q_sl_dep_dn6, locals.var_q_sl_dep_dn7, locals.var_q_sl_dep_dn8, locals.var_q_sl_dep_dn9, locals.var_q_sl_dep_dn10, locals.var_q_sl_dep_dn13,)
    }
};
        locals.var_q_sl_dep = assign32880_e37409;
        locals.var_q_sl_dep_dn0 = assign32880_e37409_d_n0;
        locals.var_q_sl_dep_dn2 = assign32880_e37409_d_n2;
        locals.var_q_sl_dep_dn4 = assign32880_e37409_d_n4;
        locals.var_q_sl_dep_dn5 = assign32880_e37409_d_n5;
        locals.var_q_sl_dep_dn6 = assign32880_e37409_d_n6;
        locals.var_q_sl_dep_dn7 = assign32880_e37409_d_n7;
        locals.var_q_sl_dep_dn8 = assign32880_e37409_d_n8;
        locals.var_q_sl_dep_dn9 = assign32880_e37409_d_n9;
        locals.var_q_sl_dep_dn10 = assign32880_e37409_d_n10;
        locals.var_q_sl_dep_dn13 = assign32880_e37409_d_n13;
        locals.var_q_sl_dep_rv = 0.0;

        let (assign32890_e37421, assign32890_e37421_d_n0, assign32890_e37421_d_n2, assign32890_e37421_d_n4, assign32890_e37421_d_n5, assign32890_e37421_d_n6, assign32890_e37421_d_n7, assign32890_e37421_d_n8, assign32890_e37421_d_n9, assign32890_e37421_d_n10, assign32890_e37421_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_nl, locals.var_q_nl_dn0, locals.var_q_nl_dn2, locals.var_q_nl_dn4, locals.var_q_nl_dn5, locals.var_q_nl_dn6, locals.var_q_nl_dn7, locals.var_q_nl_dn8, locals.var_q_nl_dn9, locals.var_q_nl_dn10, locals.var_q_nl_dn13,)
    }
};
        locals.var_q_nl = assign32890_e37421;
        locals.var_q_nl_dn0 = assign32890_e37421_d_n0;
        locals.var_q_nl_dn2 = assign32890_e37421_d_n2;
        locals.var_q_nl_dn4 = assign32890_e37421_d_n4;
        locals.var_q_nl_dn5 = assign32890_e37421_d_n5;
        locals.var_q_nl_dn6 = assign32890_e37421_d_n6;
        locals.var_q_nl_dn7 = assign32890_e37421_d_n7;
        locals.var_q_nl_dn8 = assign32890_e37421_d_n8;
        locals.var_q_nl_dn9 = assign32890_e37421_d_n9;
        locals.var_q_nl_dn10 = assign32890_e37421_d_n10;
        locals.var_q_nl_dn13 = assign32890_e37421_d_n13;
        locals.var_q_nl_rv = 0.0;

        let (assign32900_e37435, assign32900_e37435_d_n0, assign32900_e37435_d_n2, assign32900_e37435_d_n4, assign32900_e37435_d_n5, assign32900_e37435_d_n6, assign32900_e37435_d_n7, assign32900_e37435_d_n8, assign32900_e37435_d_n9, assign32900_e37435_d_n10, assign32900_e37435_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) {
        let assign32900_e37433: f64 = (locals.var_phi_bl_dep - locals.var_phi_jl_dep);
        (assign32900_e37433, (locals.var_phi_bl_dep_dn0 - locals.var_phi_jl_dep_dn0), (locals.var_phi_bl_dep_dn2 - locals.var_phi_jl_dep_dn2), (locals.var_phi_bl_dep_dn4 - locals.var_phi_jl_dep_dn4), (locals.var_phi_bl_dep_dn5 - locals.var_phi_jl_dep_dn5), (locals.var_phi_bl_dep_dn6 - locals.var_phi_jl_dep_dn6), (locals.var_phi_bl_dep_dn7 - locals.var_phi_jl_dep_dn7), (locals.var_phi_bl_dep_dn8 - locals.var_phi_jl_dep_dn8), (locals.var_phi_bl_dep_dn9 - locals.var_phi_jl_dep_dn9), (locals.var_phi_bl_dep_dn10 - locals.var_phi_jl_dep_dn10), (locals.var_phi_bl_dep_dn13 - locals.var_phi_jl_dep_dn13),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign32900_e37435;
        locals.var_t1_dn0 = assign32900_e37435_d_n0;
        locals.var_t1_dn2 = assign32900_e37435_d_n2;
        locals.var_t1_dn4 = assign32900_e37435_d_n4;
        locals.var_t1_dn5 = assign32900_e37435_d_n5;
        locals.var_t1_dn6 = assign32900_e37435_d_n6;
        locals.var_t1_dn7 = assign32900_e37435_d_n7;
        locals.var_t1_dn8 = assign32900_e37435_d_n8;
        locals.var_t1_dn9 = assign32900_e37435_d_n9;
        locals.var_t1_dn10 = assign32900_e37435_d_n10;
        locals.var_t1_dn13 = assign32900_e37435_d_n13;
        locals.var_t1_rv = 0.0;

        let assign32910_e37439: f64 = 0.1;
        let assign32910_e37444: f64 = if ((locals.var_t1 < assign32910_e37439) && (0.1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard780 = assign32910_e37444;
        locals.var_guard780_rv = 0.0;

        let (assign32920_e37462, assign32920_e37462_d_n0, assign32920_e37462_d_n2, assign32920_e37462_d_n4, assign32920_e37462_d_n5, assign32920_e37462_d_n6, assign32920_e37462_d_n7, assign32920_e37462_d_n8, assign32920_e37462_d_n9, assign32920_e37462_d_n10, assign32920_e37462_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard780 != 0.0)) {
        let assign32920_e37458: f64 = 0.1;
        let assign32920_e37460: f64 = (assign32920_e37458 - locals.var_t1);
        (assign32920_e37460, (-locals.var_t1_dn0), (-locals.var_t1_dn2), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign32920_e37462;
        locals.var_tmf1_dn0 = assign32920_e37462_d_n0;
        locals.var_tmf1_dn2 = assign32920_e37462_d_n2;
        locals.var_tmf1_dn4 = assign32920_e37462_d_n4;
        locals.var_tmf1_dn5 = assign32920_e37462_d_n5;
        locals.var_tmf1_dn6 = assign32920_e37462_d_n6;
        locals.var_tmf1_dn7 = assign32920_e37462_d_n7;
        locals.var_tmf1_dn8 = assign32920_e37462_d_n8;
        locals.var_tmf1_dn9 = assign32920_e37462_d_n9;
        locals.var_tmf1_dn10 = assign32920_e37462_d_n10;
        locals.var_tmf1_dn13 = assign32920_e37462_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign32930_e37478, assign32930_e37478_d_n0, assign32930_e37478_d_n2, assign32930_e37478_d_n4, assign32930_e37478_d_n5, assign32930_e37478_d_n6, assign32930_e37478_d_n7, assign32930_e37478_d_n8, assign32930_e37478_d_n9, assign32930_e37478_d_n10, assign32930_e37478_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard780 != 0.0)) {
        let assign32930_e37476: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign32930_e37476, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign32930_e37478;
        locals.var_x2_dn0 = assign32930_e37478_d_n0;
        locals.var_x2_dn2 = assign32930_e37478_d_n2;
        locals.var_x2_dn4 = assign32930_e37478_d_n4;
        locals.var_x2_dn5 = assign32930_e37478_d_n5;
        locals.var_x2_dn6 = assign32930_e37478_d_n6;
        locals.var_x2_dn7 = assign32930_e37478_d_n7;
        locals.var_x2_dn8 = assign32930_e37478_d_n8;
        locals.var_x2_dn9 = assign32930_e37478_d_n9;
        locals.var_x2_dn10 = assign32930_e37478_d_n10;
        locals.var_x2_dn13 = assign32930_e37478_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign32940_e37494, assign32940_e37494_d_n0, assign32940_e37494_d_n2, assign32940_e37494_d_n4, assign32940_e37494_d_n5, assign32940_e37494_d_n6, assign32940_e37494_d_n7, assign32940_e37494_d_n8, assign32940_e37494_d_n9, assign32940_e37494_d_n10, assign32940_e37494_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard780 != 0.0)) {
        let assign32940_e37492: f64 = (0.1 * 0.1);
        (assign32940_e37492, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign32940_e37494;
        locals.var_xmax2_dn0 = assign32940_e37494_d_n0;
        locals.var_xmax2_dn2 = assign32940_e37494_d_n2;
        locals.var_xmax2_dn4 = assign32940_e37494_d_n4;
        locals.var_xmax2_dn5 = assign32940_e37494_d_n5;
        locals.var_xmax2_dn6 = assign32940_e37494_d_n6;
        locals.var_xmax2_dn7 = assign32940_e37494_d_n7;
        locals.var_xmax2_dn8 = assign32940_e37494_d_n8;
        locals.var_xmax2_dn9 = assign32940_e37494_d_n9;
        locals.var_xmax2_dn10 = assign32940_e37494_d_n10;
        locals.var_xmax2_dn13 = assign32940_e37494_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign32950_e37508, assign32950_e37508_d_n0, assign32950_e37508_d_n2, assign32950_e37508_d_n4, assign32950_e37508_d_n5, assign32950_e37508_d_n6, assign32950_e37508_d_n7, assign32950_e37508_d_n8, assign32950_e37508_d_n9, assign32950_e37508_d_n10, assign32950_e37508_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard780 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign32950_e37508;
        locals.var_xp_dn0 = assign32950_e37508_d_n0;
        locals.var_xp_dn2 = assign32950_e37508_d_n2;
        locals.var_xp_dn4 = assign32950_e37508_d_n4;
        locals.var_xp_dn5 = assign32950_e37508_d_n5;
        locals.var_xp_dn6 = assign32950_e37508_d_n6;
        locals.var_xp_dn7 = assign32950_e37508_d_n7;
        locals.var_xp_dn8 = assign32950_e37508_d_n8;
        locals.var_xp_dn9 = assign32950_e37508_d_n9;
        locals.var_xp_dn10 = assign32950_e37508_d_n10;
        locals.var_xp_dn13 = assign32950_e37508_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign32960_e37522, assign32960_e37522_d_n0, assign32960_e37522_d_n2, assign32960_e37522_d_n4, assign32960_e37522_d_n5, assign32960_e37522_d_n6, assign32960_e37522_d_n7, assign32960_e37522_d_n8, assign32960_e37522_d_n9, assign32960_e37522_d_n10, assign32960_e37522_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard780 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign32960_e37522;
        locals.var_xmp_dn0 = assign32960_e37522_d_n0;
        locals.var_xmp_dn2 = assign32960_e37522_d_n2;
        locals.var_xmp_dn4 = assign32960_e37522_d_n4;
        locals.var_xmp_dn5 = assign32960_e37522_d_n5;
        locals.var_xmp_dn6 = assign32960_e37522_d_n6;
        locals.var_xmp_dn7 = assign32960_e37522_d_n7;
        locals.var_xmp_dn8 = assign32960_e37522_d_n8;
        locals.var_xmp_dn9 = assign32960_e37522_d_n9;
        locals.var_xmp_dn10 = assign32960_e37522_d_n10;
        locals.var_xmp_dn13 = assign32960_e37522_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign32970_e37536,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard780 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign32970_e37536;
        locals.var_m0_rv = 0.0;

        let (assign32980_e37550,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard780 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign32980_e37550;
        locals.var_mm_rv = 0.0;

        let (assign32990_e37564, assign32990_e37564_d_n0, assign32990_e37564_d_n2, assign32990_e37564_d_n4, assign32990_e37564_d_n5, assign32990_e37564_d_n6, assign32990_e37564_d_n7, assign32990_e37564_d_n8, assign32990_e37564_d_n9, assign32990_e37564_d_n10, assign32990_e37564_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard780 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign32990_e37564;
        locals.var_arg_dn0 = assign32990_e37564_d_n0;
        locals.var_arg_dn2 = assign32990_e37564_d_n2;
        locals.var_arg_dn4 = assign32990_e37564_d_n4;
        locals.var_arg_dn5 = assign32990_e37564_d_n5;
        locals.var_arg_dn6 = assign32990_e37564_d_n6;
        locals.var_arg_dn7 = assign32990_e37564_d_n7;
        locals.var_arg_dn8 = assign32990_e37564_d_n8;
        locals.var_arg_dn9 = assign32990_e37564_d_n9;
        locals.var_arg_dn10 = assign32990_e37564_d_n10;
        locals.var_arg_dn13 = assign32990_e37564_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign33000_e37578, assign33000_e37578_d_n0, assign33000_e37578_d_n2, assign33000_e37578_d_n4, assign33000_e37578_d_n5, assign33000_e37578_d_n6, assign33000_e37578_d_n7, assign33000_e37578_d_n8, assign33000_e37578_d_n9, assign33000_e37578_d_n10, assign33000_e37578_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard780 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign33000_e37578;
        locals.var_dnm_dn0 = assign33000_e37578_d_n0;
        locals.var_dnm_dn2 = assign33000_e37578_d_n2;
        locals.var_dnm_dn4 = assign33000_e37578_d_n4;
        locals.var_dnm_dn5 = assign33000_e37578_d_n5;
        locals.var_dnm_dn6 = assign33000_e37578_d_n6;
        locals.var_dnm_dn7 = assign33000_e37578_d_n7;
        locals.var_dnm_dn8 = assign33000_e37578_d_n8;
        locals.var_dnm_dn9 = assign33000_e37578_d_n9;
        locals.var_dnm_dn10 = assign33000_e37578_d_n10;
        locals.var_dnm_dn13 = assign33000_e37578_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign33010_e37594, assign33010_e37594_d_n0, assign33010_e37594_d_n2, assign33010_e37594_d_n4, assign33010_e37594_d_n5, assign33010_e37594_d_n6, assign33010_e37594_d_n7, assign33010_e37594_d_n8, assign33010_e37594_d_n9, assign33010_e37594_d_n10, assign33010_e37594_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard780 != 0.0)) {
        let assign33010_e37592: f64 = (locals.var_xp * locals.var_x2);
        (assign33010_e37592, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign33010_e37594;
        locals.var_xp_dn0 = assign33010_e37594_d_n0;
        locals.var_xp_dn2 = assign33010_e37594_d_n2;
        locals.var_xp_dn4 = assign33010_e37594_d_n4;
        locals.var_xp_dn5 = assign33010_e37594_d_n5;
        locals.var_xp_dn6 = assign33010_e37594_d_n6;
        locals.var_xp_dn7 = assign33010_e37594_d_n7;
        locals.var_xp_dn8 = assign33010_e37594_d_n8;
        locals.var_xp_dn9 = assign33010_e37594_d_n9;
        locals.var_xp_dn10 = assign33010_e37594_d_n10;
        locals.var_xp_dn13 = assign33010_e37594_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign33020_e37610, assign33020_e37610_d_n0, assign33020_e37610_d_n2, assign33020_e37610_d_n4, assign33020_e37610_d_n5, assign33020_e37610_d_n6, assign33020_e37610_d_n7, assign33020_e37610_d_n8, assign33020_e37610_d_n9, assign33020_e37610_d_n10, assign33020_e37610_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard780 != 0.0)) {
        let assign33020_e37608: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign33020_e37608, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign33020_e37610;
        locals.var_xmp_dn0 = assign33020_e37610_d_n0;
        locals.var_xmp_dn2 = assign33020_e37610_d_n2;
        locals.var_xmp_dn4 = assign33020_e37610_d_n4;
        locals.var_xmp_dn5 = assign33020_e37610_d_n5;
        locals.var_xmp_dn6 = assign33020_e37610_d_n6;
        locals.var_xmp_dn7 = assign33020_e37610_d_n7;
        locals.var_xmp_dn8 = assign33020_e37610_d_n8;
        locals.var_xmp_dn9 = assign33020_e37610_d_n9;
        locals.var_xmp_dn10 = assign33020_e37610_d_n10;
        locals.var_xmp_dn13 = assign33020_e37610_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign33030_e37626, assign33030_e37626_d_n0, assign33030_e37626_d_n2, assign33030_e37626_d_n4, assign33030_e37626_d_n5, assign33030_e37626_d_n6, assign33030_e37626_d_n7, assign33030_e37626_d_n8, assign33030_e37626_d_n9, assign33030_e37626_d_n10, assign33030_e37626_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard780 != 0.0)) {
        let assign33030_e37624: f64 = (locals.var_xp * locals.var_x2);
        (assign33030_e37624, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign33030_e37626;
        locals.var_xp_dn0 = assign33030_e37626_d_n0;
        locals.var_xp_dn2 = assign33030_e37626_d_n2;
        locals.var_xp_dn4 = assign33030_e37626_d_n4;
        locals.var_xp_dn5 = assign33030_e37626_d_n5;
        locals.var_xp_dn6 = assign33030_e37626_d_n6;
        locals.var_xp_dn7 = assign33030_e37626_d_n7;
        locals.var_xp_dn8 = assign33030_e37626_d_n8;
        locals.var_xp_dn9 = assign33030_e37626_d_n9;
        locals.var_xp_dn10 = assign33030_e37626_d_n10;
        locals.var_xp_dn13 = assign33030_e37626_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign33040_e37642, assign33040_e37642_d_n0, assign33040_e37642_d_n2, assign33040_e37642_d_n4, assign33040_e37642_d_n5, assign33040_e37642_d_n6, assign33040_e37642_d_n7, assign33040_e37642_d_n8, assign33040_e37642_d_n9, assign33040_e37642_d_n10, assign33040_e37642_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard780 != 0.0)) {
        let assign33040_e37640: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign33040_e37640, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign33040_e37642;
        locals.var_xmp_dn0 = assign33040_e37642_d_n0;
        locals.var_xmp_dn2 = assign33040_e37642_d_n2;
        locals.var_xmp_dn4 = assign33040_e37642_d_n4;
        locals.var_xmp_dn5 = assign33040_e37642_d_n5;
        locals.var_xmp_dn6 = assign33040_e37642_d_n6;
        locals.var_xmp_dn7 = assign33040_e37642_d_n7;
        locals.var_xmp_dn8 = assign33040_e37642_d_n8;
        locals.var_xmp_dn9 = assign33040_e37642_d_n9;
        locals.var_xmp_dn10 = assign33040_e37642_d_n10;
        locals.var_xmp_dn13 = assign33040_e37642_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign33050_e37658, assign33050_e37658_d_n0, assign33050_e37658_d_n2, assign33050_e37658_d_n4, assign33050_e37658_d_n5, assign33050_e37658_d_n6, assign33050_e37658_d_n7, assign33050_e37658_d_n8, assign33050_e37658_d_n9, assign33050_e37658_d_n10, assign33050_e37658_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard780 != 0.0)) {
        let assign33050_e37656: f64 = (locals.var_xp + locals.var_xmp);
        (assign33050_e37656, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign33050_e37658;
        locals.var_arg_dn0 = assign33050_e37658_d_n0;
        locals.var_arg_dn2 = assign33050_e37658_d_n2;
        locals.var_arg_dn4 = assign33050_e37658_d_n4;
        locals.var_arg_dn5 = assign33050_e37658_d_n5;
        locals.var_arg_dn6 = assign33050_e37658_d_n6;
        locals.var_arg_dn7 = assign33050_e37658_d_n7;
        locals.var_arg_dn8 = assign33050_e37658_d_n8;
        locals.var_arg_dn9 = assign33050_e37658_d_n9;
        locals.var_arg_dn10 = assign33050_e37658_d_n10;
        locals.var_arg_dn13 = assign33050_e37658_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign33060_e37672, assign33060_e37672_d_n0, assign33060_e37672_d_n2, assign33060_e37672_d_n4, assign33060_e37672_d_n5, assign33060_e37672_d_n6, assign33060_e37672_d_n7, assign33060_e37672_d_n8, assign33060_e37672_d_n9, assign33060_e37672_d_n10, assign33060_e37672_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard780 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign33060_e37672;
        locals.var_dnm_dn0 = assign33060_e37672_d_n0;
        locals.var_dnm_dn2 = assign33060_e37672_d_n2;
        locals.var_dnm_dn4 = assign33060_e37672_d_n4;
        locals.var_dnm_dn5 = assign33060_e37672_d_n5;
        locals.var_dnm_dn6 = assign33060_e37672_d_n6;
        locals.var_dnm_dn7 = assign33060_e37672_d_n7;
        locals.var_dnm_dn8 = assign33060_e37672_d_n8;
        locals.var_dnm_dn9 = assign33060_e37672_d_n9;
        locals.var_dnm_dn10 = assign33060_e37672_d_n10;
        locals.var_dnm_dn13 = assign33060_e37672_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign33070_e37687: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard781 = assign33070_e37687;
        locals.var_guard781_rv = 0.0;

        let assign33080_e37690: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard782 = assign33080_e37690;
        locals.var_guard782_rv = 0.0;

        let (assign33090_e37708,) = {
    if (((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard780 != 0.0)) && (locals.var_guard781 != 0.0)) && (locals.var_guard782 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33090_e37708;
        locals.var_mm_rv = 0.0;

        let assign33100_e37711: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard783 = assign33100_e37711;
        locals.var_guard783_rv = 0.0;

        let (assign33110_e37732,) = {
    if ((((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard780 != 0.0)) && (locals.var_guard781 != 0.0)) && (locals.var_guard782 == 0.0)) && (locals.var_guard783 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33110_e37732;
        locals.var_mm_rv = 0.0;

        let assign33120_e37735: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard784 = assign33120_e37735;
        locals.var_guard784_rv = 0.0;

        let (assign33130_e37759,) = {
    if (((((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard780 != 0.0)) && (locals.var_guard781 != 0.0)) && (locals.var_guard782 == 0.0)) && (locals.var_guard783 == 0.0)) && (locals.var_guard784 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33130_e37759;
        locals.var_mm_rv = 0.0;

        let assign33140_e37762: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard785 = assign33140_e37762;
        locals.var_guard785_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_103(
        locals: &mut StampLocals,
    ) {
        let (assign33150_e37789,) = {
    if ((((((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard780 != 0.0)) && (locals.var_guard781 != 0.0)) && (locals.var_guard782 == 0.0)) && (locals.var_guard783 == 0.0)) && (locals.var_guard784 == 0.0)) && (locals.var_guard785 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33150_e37789;
        locals.var_mm_rv = 0.0;

        let (assign33160_e37805,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard780 != 0.0)) && (locals.var_guard781 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign33160_e37805;
        locals.var_m0_rv = 0.0;

        let mut assign33170_loop_guard: usize = 0;
        while {
            let assign33170_cond_e37822: f64 = if (((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard780 != 0.0)) && (locals.var_guard781 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign33170_cond_e37822 != 0.0
        } {
            assign33170_loop_guard += 1;
            assert!(assign33170_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign33170_body0_e37839, assign33170_body0_e37839_d_n0, assign33170_body0_e37839_d_n2, assign33170_body0_e37839_d_n4, assign33170_body0_e37839_d_n5, assign33170_body0_e37839_d_n6, assign33170_body0_e37839_d_n7, assign33170_body0_e37839_d_n8, assign33170_body0_e37839_d_n9, assign33170_body0_e37839_d_n10, assign33170_body0_e37839_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard780 != 0.0)) && (locals.var_guard781 != 0.0)) {
        let assign33170_body0_e37837: f64 = (locals.var_dnm).sqrt();
        (assign33170_body0_e37837, (locals.var_dnm_dn0 / (2.0 * assign33170_body0_e37837)), (locals.var_dnm_dn2 / (2.0 * assign33170_body0_e37837)), (locals.var_dnm_dn4 / (2.0 * assign33170_body0_e37837)), (locals.var_dnm_dn5 / (2.0 * assign33170_body0_e37837)), (locals.var_dnm_dn6 / (2.0 * assign33170_body0_e37837)), (locals.var_dnm_dn7 / (2.0 * assign33170_body0_e37837)), (locals.var_dnm_dn8 / (2.0 * assign33170_body0_e37837)), (locals.var_dnm_dn9 / (2.0 * assign33170_body0_e37837)), (locals.var_dnm_dn10 / (2.0 * assign33170_body0_e37837)), (locals.var_dnm_dn13 / (2.0 * assign33170_body0_e37837)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign33170_body0_e37839;
            locals.var_dnm_dn0 = assign33170_body0_e37839_d_n0;
            locals.var_dnm_dn2 = assign33170_body0_e37839_d_n2;
            locals.var_dnm_dn4 = assign33170_body0_e37839_d_n4;
            locals.var_dnm_dn5 = assign33170_body0_e37839_d_n5;
            locals.var_dnm_dn6 = assign33170_body0_e37839_d_n6;
            locals.var_dnm_dn7 = assign33170_body0_e37839_d_n7;
            locals.var_dnm_dn8 = assign33170_body0_e37839_d_n8;
            locals.var_dnm_dn9 = assign33170_body0_e37839_d_n9;
            locals.var_dnm_dn10 = assign33170_body0_e37839_d_n10;
            locals.var_dnm_dn13 = assign33170_body0_e37839_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign33170_body1_e37857,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard780 != 0.0)) && (locals.var_guard781 != 0.0)) {
        let assign33170_body1_e37855: f64 = (locals.var_m0 + 1.0);
        (assign33170_body1_e37855,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign33170_body1_e37857;
            locals.var_m0_rv = 0.0;
        }

        let (assign33180_e37885, assign33180_e37885_d_n0, assign33180_e37885_d_n2, assign33180_e37885_d_n4, assign33180_e37885_d_n5, assign33180_e37885_d_n6, assign33180_e37885_d_n7, assign33180_e37885_d_n8, assign33180_e37885_d_n9, assign33180_e37885_d_n10, assign33180_e37885_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard780 != 0.0)) && (locals.var_guard781 == 0.0)) {
        let (assign33180_e37883, assign33180_e37883_d_n0, assign33180_e37883_d_n2, assign33180_e37883_d_n4, assign33180_e37883_d_n5, assign33180_e37883_d_n6, assign33180_e37883_d_n7, assign33180_e37883_d_n8, assign33180_e37883_d_n9, assign33180_e37883_d_n10, assign33180_e37883_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign33180_e37880: f64 = (2.0 * 2.0);
                let assign33180_e37881: f64 = (1.0 / assign33180_e37880);
                let assign33180_e37882: f64 = (locals.var_dnm).powf(assign33180_e37881);
                (assign33180_e37882, if 0.0 == 0.0 && ((assign33180_e37881) as f64).is_finite() && ((assign33180_e37881) as f64).fract() == 0.0 { if assign33180_e37881 == 0.0 { 0.0 } else { (assign33180_e37881 * ((locals.var_dnm).powf(assign33180_e37881 - 1.0) * locals.var_dnm_dn0)) } } else { (assign33180_e37882 * (assign33180_e37881 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33180_e37881) as f64).is_finite() && ((assign33180_e37881) as f64).fract() == 0.0 { if assign33180_e37881 == 0.0 { 0.0 } else { (assign33180_e37881 * ((locals.var_dnm).powf(assign33180_e37881 - 1.0) * locals.var_dnm_dn2)) } } else { (assign33180_e37882 * (assign33180_e37881 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33180_e37881) as f64).is_finite() && ((assign33180_e37881) as f64).fract() == 0.0 { if assign33180_e37881 == 0.0 { 0.0 } else { (assign33180_e37881 * ((locals.var_dnm).powf(assign33180_e37881 - 1.0) * locals.var_dnm_dn4)) } } else { (assign33180_e37882 * (assign33180_e37881 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33180_e37881) as f64).is_finite() && ((assign33180_e37881) as f64).fract() == 0.0 { if assign33180_e37881 == 0.0 { 0.0 } else { (assign33180_e37881 * ((locals.var_dnm).powf(assign33180_e37881 - 1.0) * locals.var_dnm_dn5)) } } else { (assign33180_e37882 * (assign33180_e37881 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33180_e37881) as f64).is_finite() && ((assign33180_e37881) as f64).fract() == 0.0 { if assign33180_e37881 == 0.0 { 0.0 } else { (assign33180_e37881 * ((locals.var_dnm).powf(assign33180_e37881 - 1.0) * locals.var_dnm_dn6)) } } else { (assign33180_e37882 * (assign33180_e37881 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33180_e37881) as f64).is_finite() && ((assign33180_e37881) as f64).fract() == 0.0 { if assign33180_e37881 == 0.0 { 0.0 } else { (assign33180_e37881 * ((locals.var_dnm).powf(assign33180_e37881 - 1.0) * locals.var_dnm_dn7)) } } else { (assign33180_e37882 * (assign33180_e37881 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33180_e37881) as f64).is_finite() && ((assign33180_e37881) as f64).fract() == 0.0 { if assign33180_e37881 == 0.0 { 0.0 } else { (assign33180_e37881 * ((locals.var_dnm).powf(assign33180_e37881 - 1.0) * locals.var_dnm_dn8)) } } else { (assign33180_e37882 * (assign33180_e37881 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33180_e37881) as f64).is_finite() && ((assign33180_e37881) as f64).fract() == 0.0 { if assign33180_e37881 == 0.0 { 0.0 } else { (assign33180_e37881 * ((locals.var_dnm).powf(assign33180_e37881 - 1.0) * locals.var_dnm_dn9)) } } else { (assign33180_e37882 * (assign33180_e37881 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33180_e37881) as f64).is_finite() && ((assign33180_e37881) as f64).fract() == 0.0 { if assign33180_e37881 == 0.0 { 0.0 } else { (assign33180_e37881 * ((locals.var_dnm).powf(assign33180_e37881 - 1.0) * locals.var_dnm_dn10)) } } else { (assign33180_e37882 * (assign33180_e37881 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33180_e37881) as f64).is_finite() && ((assign33180_e37881) as f64).fract() == 0.0 { if assign33180_e37881 == 0.0 { 0.0 } else { (assign33180_e37881 * ((locals.var_dnm).powf(assign33180_e37881 - 1.0) * locals.var_dnm_dn13)) } } else { (assign33180_e37882 * (assign33180_e37881 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign33180_e37883, assign33180_e37883_d_n0, assign33180_e37883_d_n2, assign33180_e37883_d_n4, assign33180_e37883_d_n5, assign33180_e37883_d_n6, assign33180_e37883_d_n7, assign33180_e37883_d_n8, assign33180_e37883_d_n9, assign33180_e37883_d_n10, assign33180_e37883_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign33180_e37885;
        locals.var_dnm_dn0 = assign33180_e37885_d_n0;
        locals.var_dnm_dn2 = assign33180_e37885_d_n2;
        locals.var_dnm_dn4 = assign33180_e37885_d_n4;
        locals.var_dnm_dn5 = assign33180_e37885_d_n5;
        locals.var_dnm_dn6 = assign33180_e37885_d_n6;
        locals.var_dnm_dn7 = assign33180_e37885_d_n7;
        locals.var_dnm_dn8 = assign33180_e37885_d_n8;
        locals.var_dnm_dn9 = assign33180_e37885_d_n9;
        locals.var_dnm_dn10 = assign33180_e37885_d_n10;
        locals.var_dnm_dn13 = assign33180_e37885_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign33190_e37901, assign33190_e37901_d_n0, assign33190_e37901_d_n2, assign33190_e37901_d_n4, assign33190_e37901_d_n5, assign33190_e37901_d_n6, assign33190_e37901_d_n7, assign33190_e37901_d_n8, assign33190_e37901_d_n9, assign33190_e37901_d_n10, assign33190_e37901_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard780 != 0.0)) {
        let assign33190_e37899: f64 = (1.0 / locals.var_dnm);
        (assign33190_e37899, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign33190_e37901;
        locals.var_dnm_dn0 = assign33190_e37901_d_n0;
        locals.var_dnm_dn2 = assign33190_e37901_d_n2;
        locals.var_dnm_dn4 = assign33190_e37901_d_n4;
        locals.var_dnm_dn5 = assign33190_e37901_d_n5;
        locals.var_dnm_dn6 = assign33190_e37901_d_n6;
        locals.var_dnm_dn7 = assign33190_e37901_d_n7;
        locals.var_dnm_dn8 = assign33190_e37901_d_n8;
        locals.var_dnm_dn9 = assign33190_e37901_d_n9;
        locals.var_dnm_dn10 = assign33190_e37901_d_n10;
        locals.var_dnm_dn13 = assign33190_e37901_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign33200_e37919, assign33200_e37919_d_n0, assign33200_e37919_d_n2, assign33200_e37919_d_n4, assign33200_e37919_d_n5, assign33200_e37919_d_n6, assign33200_e37919_d_n7, assign33200_e37919_d_n8, assign33200_e37919_d_n9, assign33200_e37919_d_n10, assign33200_e37919_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard780 != 0.0)) {
        let assign33200_e37915: f64 = (locals.var_tmf1 * 0.1);
        let assign33200_e37917: f64 = (assign33200_e37915 * locals.var_dnm);
        (assign33200_e37917, (((locals.var_tmf1_dn0 * 0.1) * locals.var_dnm) + (assign33200_e37915 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.1) * locals.var_dnm) + (assign33200_e37915 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.1) * locals.var_dnm) + (assign33200_e37915 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.1) * locals.var_dnm) + (assign33200_e37915 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.1) * locals.var_dnm) + (assign33200_e37915 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.1) * locals.var_dnm) + (assign33200_e37915 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.1) * locals.var_dnm) + (assign33200_e37915 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.1) * locals.var_dnm) + (assign33200_e37915 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.1) * locals.var_dnm) + (assign33200_e37915 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * 0.1) * locals.var_dnm) + (assign33200_e37915 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign33200_e37919;
        locals.var_tmf0_dn0 = assign33200_e37919_d_n0;
        locals.var_tmf0_dn2 = assign33200_e37919_d_n2;
        locals.var_tmf0_dn4 = assign33200_e37919_d_n4;
        locals.var_tmf0_dn5 = assign33200_e37919_d_n5;
        locals.var_tmf0_dn6 = assign33200_e37919_d_n6;
        locals.var_tmf0_dn7 = assign33200_e37919_d_n7;
        locals.var_tmf0_dn8 = assign33200_e37919_d_n8;
        locals.var_tmf0_dn9 = assign33200_e37919_d_n9;
        locals.var_tmf0_dn10 = assign33200_e37919_d_n10;
        locals.var_tmf0_dn13 = assign33200_e37919_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign33210_e37939, assign33210_e37939_d_n0, assign33210_e37939_d_n2, assign33210_e37939_d_n4, assign33210_e37939_d_n5, assign33210_e37939_d_n6, assign33210_e37939_d_n7, assign33210_e37939_d_n8, assign33210_e37939_d_n9, assign33210_e37939_d_n10, assign33210_e37939_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard780 != 0.0)) {
        let assign33210_e37933: f64 = (0.1 * locals.var_xmp);
        let assign33210_e37935: f64 = (assign33210_e37933 * locals.var_dnm);
        let assign33210_e37937: f64 = (assign33210_e37935 / locals.var_arg);
        (assign33210_e37937, ((((((0.1 * locals.var_xmp_dn0) * locals.var_dnm) + (assign33210_e37933 * locals.var_dnm_dn0)) * locals.var_arg) - (assign33210_e37935 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn2) * locals.var_dnm) + (assign33210_e37933 * locals.var_dnm_dn2)) * locals.var_arg) - (assign33210_e37935 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn4) * locals.var_dnm) + (assign33210_e37933 * locals.var_dnm_dn4)) * locals.var_arg) - (assign33210_e37935 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn5) * locals.var_dnm) + (assign33210_e37933 * locals.var_dnm_dn5)) * locals.var_arg) - (assign33210_e37935 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn6) * locals.var_dnm) + (assign33210_e37933 * locals.var_dnm_dn6)) * locals.var_arg) - (assign33210_e37935 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn7) * locals.var_dnm) + (assign33210_e37933 * locals.var_dnm_dn7)) * locals.var_arg) - (assign33210_e37935 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn8) * locals.var_dnm) + (assign33210_e37933 * locals.var_dnm_dn8)) * locals.var_arg) - (assign33210_e37935 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn9) * locals.var_dnm) + (assign33210_e37933 * locals.var_dnm_dn9)) * locals.var_arg) - (assign33210_e37935 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn10) * locals.var_dnm) + (assign33210_e37933 * locals.var_dnm_dn10)) * locals.var_arg) - (assign33210_e37935 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn13) * locals.var_dnm) + (assign33210_e37933 * locals.var_dnm_dn13)) * locals.var_arg) - (assign33210_e37935 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign33210_e37939;
        locals.var_t0_dn0 = assign33210_e37939_d_n0;
        locals.var_t0_dn2 = assign33210_e37939_d_n2;
        locals.var_t0_dn4 = assign33210_e37939_d_n4;
        locals.var_t0_dn5 = assign33210_e37939_d_n5;
        locals.var_t0_dn6 = assign33210_e37939_d_n6;
        locals.var_t0_dn7 = assign33210_e37939_d_n7;
        locals.var_t0_dn8 = assign33210_e37939_d_n8;
        locals.var_t0_dn9 = assign33210_e37939_d_n9;
        locals.var_t0_dn10 = assign33210_e37939_d_n10;
        locals.var_t0_dn13 = assign33210_e37939_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign33220_e37957, assign33220_e37957_d_n0, assign33220_e37957_d_n2, assign33220_e37957_d_n4, assign33220_e37957_d_n5, assign33220_e37957_d_n6, assign33220_e37957_d_n7, assign33220_e37957_d_n8, assign33220_e37957_d_n9, assign33220_e37957_d_n10, assign33220_e37957_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard780 != 0.0)) {
        let assign33220_e37953: f64 = 0.1;
        let assign33220_e37955: f64 = (assign33220_e37953 - locals.var_tmf0);
        (assign33220_e37955, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign33220_e37957;
        locals.var_t2_dn0 = assign33220_e37957_d_n0;
        locals.var_t2_dn2 = assign33220_e37957_d_n2;
        locals.var_t2_dn4 = assign33220_e37957_d_n4;
        locals.var_t2_dn5 = assign33220_e37957_d_n5;
        locals.var_t2_dn6 = assign33220_e37957_d_n6;
        locals.var_t2_dn7 = assign33220_e37957_d_n7;
        locals.var_t2_dn8 = assign33220_e37957_d_n8;
        locals.var_t2_dn9 = assign33220_e37957_d_n9;
        locals.var_t2_dn10 = assign33220_e37957_d_n10;
        locals.var_t2_dn13 = assign33220_e37957_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign33230_e37971, assign33230_e37971_d_n0, assign33230_e37971_d_n2, assign33230_e37971_d_n4, assign33230_e37971_d_n5, assign33230_e37971_d_n6, assign33230_e37971_d_n7, assign33230_e37971_d_n8, assign33230_e37971_d_n9, assign33230_e37971_d_n10, assign33230_e37971_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard780 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign33230_e37971;
        locals.var_t0_dn0 = assign33230_e37971_d_n0;
        locals.var_t0_dn2 = assign33230_e37971_d_n2;
        locals.var_t0_dn4 = assign33230_e37971_d_n4;
        locals.var_t0_dn5 = assign33230_e37971_d_n5;
        locals.var_t0_dn6 = assign33230_e37971_d_n6;
        locals.var_t0_dn7 = assign33230_e37971_d_n7;
        locals.var_t0_dn8 = assign33230_e37971_d_n8;
        locals.var_t0_dn9 = assign33230_e37971_d_n9;
        locals.var_t0_dn10 = assign33230_e37971_d_n10;
        locals.var_t0_dn13 = assign33230_e37971_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign33240_e37986, assign33240_e37986_d_n0, assign33240_e37986_d_n2, assign33240_e37986_d_n4, assign33240_e37986_d_n5, assign33240_e37986_d_n6, assign33240_e37986_d_n7, assign33240_e37986_d_n8, assign33240_e37986_d_n9, assign33240_e37986_d_n10, assign33240_e37986_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard780 == 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign33240_e37986;
        locals.var_t2_dn0 = assign33240_e37986_d_n0;
        locals.var_t2_dn2 = assign33240_e37986_d_n2;
        locals.var_t2_dn4 = assign33240_e37986_d_n4;
        locals.var_t2_dn5 = assign33240_e37986_d_n5;
        locals.var_t2_dn6 = assign33240_e37986_d_n6;
        locals.var_t2_dn7 = assign33240_e37986_d_n7;
        locals.var_t2_dn8 = assign33240_e37986_d_n8;
        locals.var_t2_dn9 = assign33240_e37986_d_n9;
        locals.var_t2_dn10 = assign33240_e37986_d_n10;
        locals.var_t2_dn13 = assign33240_e37986_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign33250_e38001, assign33250_e38001_d_n0, assign33250_e38001_d_n2, assign33250_e38001_d_n4, assign33250_e38001_d_n5, assign33250_e38001_d_n6, assign33250_e38001_d_n7, assign33250_e38001_d_n8, assign33250_e38001_d_n9, assign33250_e38001_d_n10, assign33250_e38001_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard780 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign33250_e38001;
        locals.var_t0_dn0 = assign33250_e38001_d_n0;
        locals.var_t0_dn2 = assign33250_e38001_d_n2;
        locals.var_t0_dn4 = assign33250_e38001_d_n4;
        locals.var_t0_dn5 = assign33250_e38001_d_n5;
        locals.var_t0_dn6 = assign33250_e38001_d_n6;
        locals.var_t0_dn7 = assign33250_e38001_d_n7;
        locals.var_t0_dn8 = assign33250_e38001_d_n8;
        locals.var_t0_dn9 = assign33250_e38001_d_n9;
        locals.var_t0_dn10 = assign33250_e38001_d_n10;
        locals.var_t0_dn13 = assign33250_e38001_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign33260_e38016, assign33260_e38016_d_n0, assign33260_e38016_d_n2, assign33260_e38016_d_n4, assign33260_e38016_d_n5, assign33260_e38016_d_n6, assign33260_e38016_d_n7, assign33260_e38016_d_n8, assign33260_e38016_d_n9, assign33260_e38016_d_n10, assign33260_e38016_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) {
        let assign33260_e38013: f64 = (locals.var_c_2esipq_ndepm * locals.var_t2);
        let assign33260_e38014: f64 = (assign33260_e38013).sqrt();
        (assign33260_e38014, (((locals.var_c_2esipq_ndepm_dn0 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn0)) / (2.0 * assign33260_e38014)), (((locals.var_c_2esipq_ndepm_dn2 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn2)) / (2.0 * assign33260_e38014)), (((locals.var_c_2esipq_ndepm_dn4 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn4)) / (2.0 * assign33260_e38014)), (((locals.var_c_2esipq_ndepm_dn5 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn5)) / (2.0 * assign33260_e38014)), (((locals.var_c_2esipq_ndepm_dn6 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn6)) / (2.0 * assign33260_e38014)), (((locals.var_c_2esipq_ndepm_dn7 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn7)) / (2.0 * assign33260_e38014)), (((locals.var_c_2esipq_ndepm_dn8 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn8)) / (2.0 * assign33260_e38014)), (((locals.var_c_2esipq_ndepm_dn9 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn9)) / (2.0 * assign33260_e38014)), (((locals.var_c_2esipq_ndepm_dn10 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn10)) / (2.0 * assign33260_e38014)), (((locals.var_c_2esipq_ndepm_dn13 * locals.var_t2) + (locals.var_c_2esipq_ndepm * locals.var_t2_dn13)) / (2.0 * assign33260_e38014)),)
    } else {
        (locals.var_w_bl, locals.var_w_bl_dn0, locals.var_w_bl_dn2, locals.var_w_bl_dn4, locals.var_w_bl_dn5, locals.var_w_bl_dn6, locals.var_w_bl_dn7, locals.var_w_bl_dn8, locals.var_w_bl_dn9, locals.var_w_bl_dn10, locals.var_w_bl_dn13,)
    }
};
        locals.var_w_bl = assign33260_e38016;
        locals.var_w_bl_dn0 = assign33260_e38016_d_n0;
        locals.var_w_bl_dn2 = assign33260_e38016_d_n2;
        locals.var_w_bl_dn4 = assign33260_e38016_d_n4;
        locals.var_w_bl_dn5 = assign33260_e38016_d_n5;
        locals.var_w_bl_dn6 = assign33260_e38016_d_n6;
        locals.var_w_bl_dn7 = assign33260_e38016_d_n7;
        locals.var_w_bl_dn8 = assign33260_e38016_d_n8;
        locals.var_w_bl_dn9 = assign33260_e38016_d_n9;
        locals.var_w_bl_dn10 = assign33260_e38016_d_n10;
        locals.var_w_bl_dn13 = assign33260_e38016_d_n13;
        locals.var_w_bl_rv = 0.0;

        let assign33270_e38020: f64 = (locals.var_uc_depthn - 1e-8);
        let assign33270_e38025: f64 = if ((locals.var_w_bl > assign33270_e38020) && (1e-8 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard786 = assign33270_e38025;
        locals.var_guard786_rv = 0.0;

        let (assign33280_e38043, assign33280_e38043_d_n0, assign33280_e38043_d_n2, assign33280_e38043_d_n4, assign33280_e38043_d_n5, assign33280_e38043_d_n6, assign33280_e38043_d_n7, assign33280_e38043_d_n8, assign33280_e38043_d_n9, assign33280_e38043_d_n10, assign33280_e38043_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard786 != 0.0)) {
        let assign33280_e38039: f64 = (locals.var_w_bl - locals.var_uc_depthn);
        let assign33280_e38041: f64 = (assign33280_e38039 + 1e-8);
        (assign33280_e38041, (locals.var_w_bl_dn0 - locals.var_uc_depthn_dn0), (locals.var_w_bl_dn2 - locals.var_uc_depthn_dn2), (locals.var_w_bl_dn4 - locals.var_uc_depthn_dn4), (locals.var_w_bl_dn5 - locals.var_uc_depthn_dn5), (locals.var_w_bl_dn6 - locals.var_uc_depthn_dn6), (locals.var_w_bl_dn7 - locals.var_uc_depthn_dn7), (locals.var_w_bl_dn8 - locals.var_uc_depthn_dn8), (locals.var_w_bl_dn9 - locals.var_uc_depthn_dn9), (locals.var_w_bl_dn10 - locals.var_uc_depthn_dn10), (locals.var_w_bl_dn13 - locals.var_uc_depthn_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign33280_e38043;
        locals.var_tmf1_dn0 = assign33280_e38043_d_n0;
        locals.var_tmf1_dn2 = assign33280_e38043_d_n2;
        locals.var_tmf1_dn4 = assign33280_e38043_d_n4;
        locals.var_tmf1_dn5 = assign33280_e38043_d_n5;
        locals.var_tmf1_dn6 = assign33280_e38043_d_n6;
        locals.var_tmf1_dn7 = assign33280_e38043_d_n7;
        locals.var_tmf1_dn8 = assign33280_e38043_d_n8;
        locals.var_tmf1_dn9 = assign33280_e38043_d_n9;
        locals.var_tmf1_dn10 = assign33280_e38043_d_n10;
        locals.var_tmf1_dn13 = assign33280_e38043_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign33290_e38059, assign33290_e38059_d_n0, assign33290_e38059_d_n2, assign33290_e38059_d_n4, assign33290_e38059_d_n5, assign33290_e38059_d_n6, assign33290_e38059_d_n7, assign33290_e38059_d_n8, assign33290_e38059_d_n9, assign33290_e38059_d_n10, assign33290_e38059_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard786 != 0.0)) {
        let assign33290_e38057: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign33290_e38057, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign33290_e38059;
        locals.var_x2_dn0 = assign33290_e38059_d_n0;
        locals.var_x2_dn2 = assign33290_e38059_d_n2;
        locals.var_x2_dn4 = assign33290_e38059_d_n4;
        locals.var_x2_dn5 = assign33290_e38059_d_n5;
        locals.var_x2_dn6 = assign33290_e38059_d_n6;
        locals.var_x2_dn7 = assign33290_e38059_d_n7;
        locals.var_x2_dn8 = assign33290_e38059_d_n8;
        locals.var_x2_dn9 = assign33290_e38059_d_n9;
        locals.var_x2_dn10 = assign33290_e38059_d_n10;
        locals.var_x2_dn13 = assign33290_e38059_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign33300_e38075, assign33300_e38075_d_n0, assign33300_e38075_d_n2, assign33300_e38075_d_n4, assign33300_e38075_d_n5, assign33300_e38075_d_n6, assign33300_e38075_d_n7, assign33300_e38075_d_n8, assign33300_e38075_d_n9, assign33300_e38075_d_n10, assign33300_e38075_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard786 != 0.0)) {
        let assign33300_e38073: f64 = (1e-8 * 1e-8);
        (assign33300_e38073, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign33300_e38075;
        locals.var_xmax2_dn0 = assign33300_e38075_d_n0;
        locals.var_xmax2_dn2 = assign33300_e38075_d_n2;
        locals.var_xmax2_dn4 = assign33300_e38075_d_n4;
        locals.var_xmax2_dn5 = assign33300_e38075_d_n5;
        locals.var_xmax2_dn6 = assign33300_e38075_d_n6;
        locals.var_xmax2_dn7 = assign33300_e38075_d_n7;
        locals.var_xmax2_dn8 = assign33300_e38075_d_n8;
        locals.var_xmax2_dn9 = assign33300_e38075_d_n9;
        locals.var_xmax2_dn10 = assign33300_e38075_d_n10;
        locals.var_xmax2_dn13 = assign33300_e38075_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign33310_e38089, assign33310_e38089_d_n0, assign33310_e38089_d_n2, assign33310_e38089_d_n4, assign33310_e38089_d_n5, assign33310_e38089_d_n6, assign33310_e38089_d_n7, assign33310_e38089_d_n8, assign33310_e38089_d_n9, assign33310_e38089_d_n10, assign33310_e38089_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard786 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign33310_e38089;
        locals.var_xp_dn0 = assign33310_e38089_d_n0;
        locals.var_xp_dn2 = assign33310_e38089_d_n2;
        locals.var_xp_dn4 = assign33310_e38089_d_n4;
        locals.var_xp_dn5 = assign33310_e38089_d_n5;
        locals.var_xp_dn6 = assign33310_e38089_d_n6;
        locals.var_xp_dn7 = assign33310_e38089_d_n7;
        locals.var_xp_dn8 = assign33310_e38089_d_n8;
        locals.var_xp_dn9 = assign33310_e38089_d_n9;
        locals.var_xp_dn10 = assign33310_e38089_d_n10;
        locals.var_xp_dn13 = assign33310_e38089_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign33320_e38103, assign33320_e38103_d_n0, assign33320_e38103_d_n2, assign33320_e38103_d_n4, assign33320_e38103_d_n5, assign33320_e38103_d_n6, assign33320_e38103_d_n7, assign33320_e38103_d_n8, assign33320_e38103_d_n9, assign33320_e38103_d_n10, assign33320_e38103_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard786 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign33320_e38103;
        locals.var_xmp_dn0 = assign33320_e38103_d_n0;
        locals.var_xmp_dn2 = assign33320_e38103_d_n2;
        locals.var_xmp_dn4 = assign33320_e38103_d_n4;
        locals.var_xmp_dn5 = assign33320_e38103_d_n5;
        locals.var_xmp_dn6 = assign33320_e38103_d_n6;
        locals.var_xmp_dn7 = assign33320_e38103_d_n7;
        locals.var_xmp_dn8 = assign33320_e38103_d_n8;
        locals.var_xmp_dn9 = assign33320_e38103_d_n9;
        locals.var_xmp_dn10 = assign33320_e38103_d_n10;
        locals.var_xmp_dn13 = assign33320_e38103_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign33330_e38117,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard786 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign33330_e38117;
        locals.var_m0_rv = 0.0;

        let (assign33340_e38131,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard786 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33340_e38131;
        locals.var_mm_rv = 0.0;

        let (assign33350_e38145, assign33350_e38145_d_n0, assign33350_e38145_d_n2, assign33350_e38145_d_n4, assign33350_e38145_d_n5, assign33350_e38145_d_n6, assign33350_e38145_d_n7, assign33350_e38145_d_n8, assign33350_e38145_d_n9, assign33350_e38145_d_n10, assign33350_e38145_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard786 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign33350_e38145;
        locals.var_arg_dn0 = assign33350_e38145_d_n0;
        locals.var_arg_dn2 = assign33350_e38145_d_n2;
        locals.var_arg_dn4 = assign33350_e38145_d_n4;
        locals.var_arg_dn5 = assign33350_e38145_d_n5;
        locals.var_arg_dn6 = assign33350_e38145_d_n6;
        locals.var_arg_dn7 = assign33350_e38145_d_n7;
        locals.var_arg_dn8 = assign33350_e38145_d_n8;
        locals.var_arg_dn9 = assign33350_e38145_d_n9;
        locals.var_arg_dn10 = assign33350_e38145_d_n10;
        locals.var_arg_dn13 = assign33350_e38145_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign33360_e38159, assign33360_e38159_d_n0, assign33360_e38159_d_n2, assign33360_e38159_d_n4, assign33360_e38159_d_n5, assign33360_e38159_d_n6, assign33360_e38159_d_n7, assign33360_e38159_d_n8, assign33360_e38159_d_n9, assign33360_e38159_d_n10, assign33360_e38159_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard786 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign33360_e38159;
        locals.var_dnm_dn0 = assign33360_e38159_d_n0;
        locals.var_dnm_dn2 = assign33360_e38159_d_n2;
        locals.var_dnm_dn4 = assign33360_e38159_d_n4;
        locals.var_dnm_dn5 = assign33360_e38159_d_n5;
        locals.var_dnm_dn6 = assign33360_e38159_d_n6;
        locals.var_dnm_dn7 = assign33360_e38159_d_n7;
        locals.var_dnm_dn8 = assign33360_e38159_d_n8;
        locals.var_dnm_dn9 = assign33360_e38159_d_n9;
        locals.var_dnm_dn10 = assign33360_e38159_d_n10;
        locals.var_dnm_dn13 = assign33360_e38159_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign33370_e38175, assign33370_e38175_d_n0, assign33370_e38175_d_n2, assign33370_e38175_d_n4, assign33370_e38175_d_n5, assign33370_e38175_d_n6, assign33370_e38175_d_n7, assign33370_e38175_d_n8, assign33370_e38175_d_n9, assign33370_e38175_d_n10, assign33370_e38175_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard786 != 0.0)) {
        let assign33370_e38173: f64 = (locals.var_xp * locals.var_x2);
        (assign33370_e38173, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign33370_e38175;
        locals.var_xp_dn0 = assign33370_e38175_d_n0;
        locals.var_xp_dn2 = assign33370_e38175_d_n2;
        locals.var_xp_dn4 = assign33370_e38175_d_n4;
        locals.var_xp_dn5 = assign33370_e38175_d_n5;
        locals.var_xp_dn6 = assign33370_e38175_d_n6;
        locals.var_xp_dn7 = assign33370_e38175_d_n7;
        locals.var_xp_dn8 = assign33370_e38175_d_n8;
        locals.var_xp_dn9 = assign33370_e38175_d_n9;
        locals.var_xp_dn10 = assign33370_e38175_d_n10;
        locals.var_xp_dn13 = assign33370_e38175_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign33380_e38191, assign33380_e38191_d_n0, assign33380_e38191_d_n2, assign33380_e38191_d_n4, assign33380_e38191_d_n5, assign33380_e38191_d_n6, assign33380_e38191_d_n7, assign33380_e38191_d_n8, assign33380_e38191_d_n9, assign33380_e38191_d_n10, assign33380_e38191_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard786 != 0.0)) {
        let assign33380_e38189: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign33380_e38189, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign33380_e38191;
        locals.var_xmp_dn0 = assign33380_e38191_d_n0;
        locals.var_xmp_dn2 = assign33380_e38191_d_n2;
        locals.var_xmp_dn4 = assign33380_e38191_d_n4;
        locals.var_xmp_dn5 = assign33380_e38191_d_n5;
        locals.var_xmp_dn6 = assign33380_e38191_d_n6;
        locals.var_xmp_dn7 = assign33380_e38191_d_n7;
        locals.var_xmp_dn8 = assign33380_e38191_d_n8;
        locals.var_xmp_dn9 = assign33380_e38191_d_n9;
        locals.var_xmp_dn10 = assign33380_e38191_d_n10;
        locals.var_xmp_dn13 = assign33380_e38191_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign33390_e38207, assign33390_e38207_d_n0, assign33390_e38207_d_n2, assign33390_e38207_d_n4, assign33390_e38207_d_n5, assign33390_e38207_d_n6, assign33390_e38207_d_n7, assign33390_e38207_d_n8, assign33390_e38207_d_n9, assign33390_e38207_d_n10, assign33390_e38207_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard786 != 0.0)) {
        let assign33390_e38205: f64 = (locals.var_xp * locals.var_x2);
        (assign33390_e38205, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign33390_e38207;
        locals.var_xp_dn0 = assign33390_e38207_d_n0;
        locals.var_xp_dn2 = assign33390_e38207_d_n2;
        locals.var_xp_dn4 = assign33390_e38207_d_n4;
        locals.var_xp_dn5 = assign33390_e38207_d_n5;
        locals.var_xp_dn6 = assign33390_e38207_d_n6;
        locals.var_xp_dn7 = assign33390_e38207_d_n7;
        locals.var_xp_dn8 = assign33390_e38207_d_n8;
        locals.var_xp_dn9 = assign33390_e38207_d_n9;
        locals.var_xp_dn10 = assign33390_e38207_d_n10;
        locals.var_xp_dn13 = assign33390_e38207_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign33400_e38223, assign33400_e38223_d_n0, assign33400_e38223_d_n2, assign33400_e38223_d_n4, assign33400_e38223_d_n5, assign33400_e38223_d_n6, assign33400_e38223_d_n7, assign33400_e38223_d_n8, assign33400_e38223_d_n9, assign33400_e38223_d_n10, assign33400_e38223_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard786 != 0.0)) {
        let assign33400_e38221: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign33400_e38221, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign33400_e38223;
        locals.var_xmp_dn0 = assign33400_e38223_d_n0;
        locals.var_xmp_dn2 = assign33400_e38223_d_n2;
        locals.var_xmp_dn4 = assign33400_e38223_d_n4;
        locals.var_xmp_dn5 = assign33400_e38223_d_n5;
        locals.var_xmp_dn6 = assign33400_e38223_d_n6;
        locals.var_xmp_dn7 = assign33400_e38223_d_n7;
        locals.var_xmp_dn8 = assign33400_e38223_d_n8;
        locals.var_xmp_dn9 = assign33400_e38223_d_n9;
        locals.var_xmp_dn10 = assign33400_e38223_d_n10;
        locals.var_xmp_dn13 = assign33400_e38223_d_n13;
        locals.var_xmp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_104(
        locals: &mut StampLocals,
    ) {
        let (assign33410_e38239, assign33410_e38239_d_n0, assign33410_e38239_d_n2, assign33410_e38239_d_n4, assign33410_e38239_d_n5, assign33410_e38239_d_n6, assign33410_e38239_d_n7, assign33410_e38239_d_n8, assign33410_e38239_d_n9, assign33410_e38239_d_n10, assign33410_e38239_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard786 != 0.0)) {
        let assign33410_e38237: f64 = (locals.var_xp + locals.var_xmp);
        (assign33410_e38237, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign33410_e38239;
        locals.var_arg_dn0 = assign33410_e38239_d_n0;
        locals.var_arg_dn2 = assign33410_e38239_d_n2;
        locals.var_arg_dn4 = assign33410_e38239_d_n4;
        locals.var_arg_dn5 = assign33410_e38239_d_n5;
        locals.var_arg_dn6 = assign33410_e38239_d_n6;
        locals.var_arg_dn7 = assign33410_e38239_d_n7;
        locals.var_arg_dn8 = assign33410_e38239_d_n8;
        locals.var_arg_dn9 = assign33410_e38239_d_n9;
        locals.var_arg_dn10 = assign33410_e38239_d_n10;
        locals.var_arg_dn13 = assign33410_e38239_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign33420_e38253, assign33420_e38253_d_n0, assign33420_e38253_d_n2, assign33420_e38253_d_n4, assign33420_e38253_d_n5, assign33420_e38253_d_n6, assign33420_e38253_d_n7, assign33420_e38253_d_n8, assign33420_e38253_d_n9, assign33420_e38253_d_n10, assign33420_e38253_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard786 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign33420_e38253;
        locals.var_dnm_dn0 = assign33420_e38253_d_n0;
        locals.var_dnm_dn2 = assign33420_e38253_d_n2;
        locals.var_dnm_dn4 = assign33420_e38253_d_n4;
        locals.var_dnm_dn5 = assign33420_e38253_d_n5;
        locals.var_dnm_dn6 = assign33420_e38253_d_n6;
        locals.var_dnm_dn7 = assign33420_e38253_d_n7;
        locals.var_dnm_dn8 = assign33420_e38253_d_n8;
        locals.var_dnm_dn9 = assign33420_e38253_d_n9;
        locals.var_dnm_dn10 = assign33420_e38253_d_n10;
        locals.var_dnm_dn13 = assign33420_e38253_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign33430_e38268: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard787 = assign33430_e38268;
        locals.var_guard787_rv = 0.0;

        let assign33440_e38271: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard788 = assign33440_e38271;
        locals.var_guard788_rv = 0.0;

        let (assign33450_e38289,) = {
    if (((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard786 != 0.0)) && (locals.var_guard787 != 0.0)) && (locals.var_guard788 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33450_e38289;
        locals.var_mm_rv = 0.0;

        let assign33460_e38292: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard789 = assign33460_e38292;
        locals.var_guard789_rv = 0.0;

        let (assign33470_e38313,) = {
    if ((((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard786 != 0.0)) && (locals.var_guard787 != 0.0)) && (locals.var_guard788 == 0.0)) && (locals.var_guard789 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33470_e38313;
        locals.var_mm_rv = 0.0;

        let assign33480_e38316: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard790 = assign33480_e38316;
        locals.var_guard790_rv = 0.0;

        let (assign33490_e38340,) = {
    if (((((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard786 != 0.0)) && (locals.var_guard787 != 0.0)) && (locals.var_guard788 == 0.0)) && (locals.var_guard789 == 0.0)) && (locals.var_guard790 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33490_e38340;
        locals.var_mm_rv = 0.0;

        let assign33500_e38343: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard791 = assign33500_e38343;
        locals.var_guard791_rv = 0.0;

        let (assign33510_e38370,) = {
    if ((((((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard786 != 0.0)) && (locals.var_guard787 != 0.0)) && (locals.var_guard788 == 0.0)) && (locals.var_guard789 == 0.0)) && (locals.var_guard790 == 0.0)) && (locals.var_guard791 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33510_e38370;
        locals.var_mm_rv = 0.0;

        let (assign33520_e38386,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard786 != 0.0)) && (locals.var_guard787 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign33520_e38386;
        locals.var_m0_rv = 0.0;

        let mut assign33530_loop_guard: usize = 0;
        while {
            let assign33530_cond_e38403: f64 = if (((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard786 != 0.0)) && (locals.var_guard787 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign33530_cond_e38403 != 0.0
        } {
            assign33530_loop_guard += 1;
            assert!(assign33530_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign33530_body0_e38420, assign33530_body0_e38420_d_n0, assign33530_body0_e38420_d_n2, assign33530_body0_e38420_d_n4, assign33530_body0_e38420_d_n5, assign33530_body0_e38420_d_n6, assign33530_body0_e38420_d_n7, assign33530_body0_e38420_d_n8, assign33530_body0_e38420_d_n9, assign33530_body0_e38420_d_n10, assign33530_body0_e38420_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard786 != 0.0)) && (locals.var_guard787 != 0.0)) {
        let assign33530_body0_e38418: f64 = (locals.var_dnm).sqrt();
        (assign33530_body0_e38418, (locals.var_dnm_dn0 / (2.0 * assign33530_body0_e38418)), (locals.var_dnm_dn2 / (2.0 * assign33530_body0_e38418)), (locals.var_dnm_dn4 / (2.0 * assign33530_body0_e38418)), (locals.var_dnm_dn5 / (2.0 * assign33530_body0_e38418)), (locals.var_dnm_dn6 / (2.0 * assign33530_body0_e38418)), (locals.var_dnm_dn7 / (2.0 * assign33530_body0_e38418)), (locals.var_dnm_dn8 / (2.0 * assign33530_body0_e38418)), (locals.var_dnm_dn9 / (2.0 * assign33530_body0_e38418)), (locals.var_dnm_dn10 / (2.0 * assign33530_body0_e38418)), (locals.var_dnm_dn13 / (2.0 * assign33530_body0_e38418)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign33530_body0_e38420;
            locals.var_dnm_dn0 = assign33530_body0_e38420_d_n0;
            locals.var_dnm_dn2 = assign33530_body0_e38420_d_n2;
            locals.var_dnm_dn4 = assign33530_body0_e38420_d_n4;
            locals.var_dnm_dn5 = assign33530_body0_e38420_d_n5;
            locals.var_dnm_dn6 = assign33530_body0_e38420_d_n6;
            locals.var_dnm_dn7 = assign33530_body0_e38420_d_n7;
            locals.var_dnm_dn8 = assign33530_body0_e38420_d_n8;
            locals.var_dnm_dn9 = assign33530_body0_e38420_d_n9;
            locals.var_dnm_dn10 = assign33530_body0_e38420_d_n10;
            locals.var_dnm_dn13 = assign33530_body0_e38420_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign33530_body1_e38438,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard786 != 0.0)) && (locals.var_guard787 != 0.0)) {
        let assign33530_body1_e38436: f64 = (locals.var_m0 + 1.0);
        (assign33530_body1_e38436,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign33530_body1_e38438;
            locals.var_m0_rv = 0.0;
        }

        let (assign33540_e38466, assign33540_e38466_d_n0, assign33540_e38466_d_n2, assign33540_e38466_d_n4, assign33540_e38466_d_n5, assign33540_e38466_d_n6, assign33540_e38466_d_n7, assign33540_e38466_d_n8, assign33540_e38466_d_n9, assign33540_e38466_d_n10, assign33540_e38466_d_n13,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard786 != 0.0)) && (locals.var_guard787 == 0.0)) {
        let (assign33540_e38464, assign33540_e38464_d_n0, assign33540_e38464_d_n2, assign33540_e38464_d_n4, assign33540_e38464_d_n5, assign33540_e38464_d_n6, assign33540_e38464_d_n7, assign33540_e38464_d_n8, assign33540_e38464_d_n9, assign33540_e38464_d_n10, assign33540_e38464_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign33540_e38461: f64 = (2.0 * 2.0);
                let assign33540_e38462: f64 = (1.0 / assign33540_e38461);
                let assign33540_e38463: f64 = (locals.var_dnm).powf(assign33540_e38462);
                (assign33540_e38463, if 0.0 == 0.0 && ((assign33540_e38462) as f64).is_finite() && ((assign33540_e38462) as f64).fract() == 0.0 { if assign33540_e38462 == 0.0 { 0.0 } else { (assign33540_e38462 * ((locals.var_dnm).powf(assign33540_e38462 - 1.0) * locals.var_dnm_dn0)) } } else { (assign33540_e38463 * (assign33540_e38462 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33540_e38462) as f64).is_finite() && ((assign33540_e38462) as f64).fract() == 0.0 { if assign33540_e38462 == 0.0 { 0.0 } else { (assign33540_e38462 * ((locals.var_dnm).powf(assign33540_e38462 - 1.0) * locals.var_dnm_dn2)) } } else { (assign33540_e38463 * (assign33540_e38462 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33540_e38462) as f64).is_finite() && ((assign33540_e38462) as f64).fract() == 0.0 { if assign33540_e38462 == 0.0 { 0.0 } else { (assign33540_e38462 * ((locals.var_dnm).powf(assign33540_e38462 - 1.0) * locals.var_dnm_dn4)) } } else { (assign33540_e38463 * (assign33540_e38462 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33540_e38462) as f64).is_finite() && ((assign33540_e38462) as f64).fract() == 0.0 { if assign33540_e38462 == 0.0 { 0.0 } else { (assign33540_e38462 * ((locals.var_dnm).powf(assign33540_e38462 - 1.0) * locals.var_dnm_dn5)) } } else { (assign33540_e38463 * (assign33540_e38462 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33540_e38462) as f64).is_finite() && ((assign33540_e38462) as f64).fract() == 0.0 { if assign33540_e38462 == 0.0 { 0.0 } else { (assign33540_e38462 * ((locals.var_dnm).powf(assign33540_e38462 - 1.0) * locals.var_dnm_dn6)) } } else { (assign33540_e38463 * (assign33540_e38462 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33540_e38462) as f64).is_finite() && ((assign33540_e38462) as f64).fract() == 0.0 { if assign33540_e38462 == 0.0 { 0.0 } else { (assign33540_e38462 * ((locals.var_dnm).powf(assign33540_e38462 - 1.0) * locals.var_dnm_dn7)) } } else { (assign33540_e38463 * (assign33540_e38462 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33540_e38462) as f64).is_finite() && ((assign33540_e38462) as f64).fract() == 0.0 { if assign33540_e38462 == 0.0 { 0.0 } else { (assign33540_e38462 * ((locals.var_dnm).powf(assign33540_e38462 - 1.0) * locals.var_dnm_dn8)) } } else { (assign33540_e38463 * (assign33540_e38462 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33540_e38462) as f64).is_finite() && ((assign33540_e38462) as f64).fract() == 0.0 { if assign33540_e38462 == 0.0 { 0.0 } else { (assign33540_e38462 * ((locals.var_dnm).powf(assign33540_e38462 - 1.0) * locals.var_dnm_dn9)) } } else { (assign33540_e38463 * (assign33540_e38462 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33540_e38462) as f64).is_finite() && ((assign33540_e38462) as f64).fract() == 0.0 { if assign33540_e38462 == 0.0 { 0.0 } else { (assign33540_e38462 * ((locals.var_dnm).powf(assign33540_e38462 - 1.0) * locals.var_dnm_dn10)) } } else { (assign33540_e38463 * (assign33540_e38462 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33540_e38462) as f64).is_finite() && ((assign33540_e38462) as f64).fract() == 0.0 { if assign33540_e38462 == 0.0 { 0.0 } else { (assign33540_e38462 * ((locals.var_dnm).powf(assign33540_e38462 - 1.0) * locals.var_dnm_dn13)) } } else { (assign33540_e38463 * (assign33540_e38462 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign33540_e38464, assign33540_e38464_d_n0, assign33540_e38464_d_n2, assign33540_e38464_d_n4, assign33540_e38464_d_n5, assign33540_e38464_d_n6, assign33540_e38464_d_n7, assign33540_e38464_d_n8, assign33540_e38464_d_n9, assign33540_e38464_d_n10, assign33540_e38464_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign33540_e38466;
        locals.var_dnm_dn0 = assign33540_e38466_d_n0;
        locals.var_dnm_dn2 = assign33540_e38466_d_n2;
        locals.var_dnm_dn4 = assign33540_e38466_d_n4;
        locals.var_dnm_dn5 = assign33540_e38466_d_n5;
        locals.var_dnm_dn6 = assign33540_e38466_d_n6;
        locals.var_dnm_dn7 = assign33540_e38466_d_n7;
        locals.var_dnm_dn8 = assign33540_e38466_d_n8;
        locals.var_dnm_dn9 = assign33540_e38466_d_n9;
        locals.var_dnm_dn10 = assign33540_e38466_d_n10;
        locals.var_dnm_dn13 = assign33540_e38466_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign33550_e38482, assign33550_e38482_d_n0, assign33550_e38482_d_n2, assign33550_e38482_d_n4, assign33550_e38482_d_n5, assign33550_e38482_d_n6, assign33550_e38482_d_n7, assign33550_e38482_d_n8, assign33550_e38482_d_n9, assign33550_e38482_d_n10, assign33550_e38482_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard786 != 0.0)) {
        let assign33550_e38480: f64 = (1.0 / locals.var_dnm);
        (assign33550_e38480, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign33550_e38482;
        locals.var_dnm_dn0 = assign33550_e38482_d_n0;
        locals.var_dnm_dn2 = assign33550_e38482_d_n2;
        locals.var_dnm_dn4 = assign33550_e38482_d_n4;
        locals.var_dnm_dn5 = assign33550_e38482_d_n5;
        locals.var_dnm_dn6 = assign33550_e38482_d_n6;
        locals.var_dnm_dn7 = assign33550_e38482_d_n7;
        locals.var_dnm_dn8 = assign33550_e38482_d_n8;
        locals.var_dnm_dn9 = assign33550_e38482_d_n9;
        locals.var_dnm_dn10 = assign33550_e38482_d_n10;
        locals.var_dnm_dn13 = assign33550_e38482_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign33560_e38500, assign33560_e38500_d_n0, assign33560_e38500_d_n2, assign33560_e38500_d_n4, assign33560_e38500_d_n5, assign33560_e38500_d_n6, assign33560_e38500_d_n7, assign33560_e38500_d_n8, assign33560_e38500_d_n9, assign33560_e38500_d_n10, assign33560_e38500_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard786 != 0.0)) {
        let assign33560_e38496: f64 = (locals.var_tmf1 * 1e-8);
        let assign33560_e38498: f64 = (assign33560_e38496 * locals.var_dnm);
        (assign33560_e38498, (((locals.var_tmf1_dn0 * 1e-8) * locals.var_dnm) + (assign33560_e38496 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 1e-8) * locals.var_dnm) + (assign33560_e38496 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 1e-8) * locals.var_dnm) + (assign33560_e38496 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 1e-8) * locals.var_dnm) + (assign33560_e38496 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 1e-8) * locals.var_dnm) + (assign33560_e38496 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 1e-8) * locals.var_dnm) + (assign33560_e38496 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 1e-8) * locals.var_dnm) + (assign33560_e38496 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 1e-8) * locals.var_dnm) + (assign33560_e38496 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 1e-8) * locals.var_dnm) + (assign33560_e38496 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * 1e-8) * locals.var_dnm) + (assign33560_e38496 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign33560_e38500;
        locals.var_tmf0_dn0 = assign33560_e38500_d_n0;
        locals.var_tmf0_dn2 = assign33560_e38500_d_n2;
        locals.var_tmf0_dn4 = assign33560_e38500_d_n4;
        locals.var_tmf0_dn5 = assign33560_e38500_d_n5;
        locals.var_tmf0_dn6 = assign33560_e38500_d_n6;
        locals.var_tmf0_dn7 = assign33560_e38500_d_n7;
        locals.var_tmf0_dn8 = assign33560_e38500_d_n8;
        locals.var_tmf0_dn9 = assign33560_e38500_d_n9;
        locals.var_tmf0_dn10 = assign33560_e38500_d_n10;
        locals.var_tmf0_dn13 = assign33560_e38500_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign33570_e38520, assign33570_e38520_d_n0, assign33570_e38520_d_n2, assign33570_e38520_d_n4, assign33570_e38520_d_n5, assign33570_e38520_d_n6, assign33570_e38520_d_n7, assign33570_e38520_d_n8, assign33570_e38520_d_n9, assign33570_e38520_d_n10, assign33570_e38520_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard786 != 0.0)) {
        let assign33570_e38514: f64 = (1e-8 * locals.var_xmp);
        let assign33570_e38516: f64 = (assign33570_e38514 * locals.var_dnm);
        let assign33570_e38518: f64 = (assign33570_e38516 / locals.var_arg);
        (assign33570_e38518, ((((((1e-8 * locals.var_xmp_dn0) * locals.var_dnm) + (assign33570_e38514 * locals.var_dnm_dn0)) * locals.var_arg) - (assign33570_e38516 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn2) * locals.var_dnm) + (assign33570_e38514 * locals.var_dnm_dn2)) * locals.var_arg) - (assign33570_e38516 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn4) * locals.var_dnm) + (assign33570_e38514 * locals.var_dnm_dn4)) * locals.var_arg) - (assign33570_e38516 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn5) * locals.var_dnm) + (assign33570_e38514 * locals.var_dnm_dn5)) * locals.var_arg) - (assign33570_e38516 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn6) * locals.var_dnm) + (assign33570_e38514 * locals.var_dnm_dn6)) * locals.var_arg) - (assign33570_e38516 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn7) * locals.var_dnm) + (assign33570_e38514 * locals.var_dnm_dn7)) * locals.var_arg) - (assign33570_e38516 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn8) * locals.var_dnm) + (assign33570_e38514 * locals.var_dnm_dn8)) * locals.var_arg) - (assign33570_e38516 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn9) * locals.var_dnm) + (assign33570_e38514 * locals.var_dnm_dn9)) * locals.var_arg) - (assign33570_e38516 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn10) * locals.var_dnm) + (assign33570_e38514 * locals.var_dnm_dn10)) * locals.var_arg) - (assign33570_e38516 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((1e-8 * locals.var_xmp_dn13) * locals.var_dnm) + (assign33570_e38514 * locals.var_dnm_dn13)) * locals.var_arg) - (assign33570_e38516 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign33570_e38520;
        locals.var_t3_dn0 = assign33570_e38520_d_n0;
        locals.var_t3_dn2 = assign33570_e38520_d_n2;
        locals.var_t3_dn4 = assign33570_e38520_d_n4;
        locals.var_t3_dn5 = assign33570_e38520_d_n5;
        locals.var_t3_dn6 = assign33570_e38520_d_n6;
        locals.var_t3_dn7 = assign33570_e38520_d_n7;
        locals.var_t3_dn8 = assign33570_e38520_d_n8;
        locals.var_t3_dn9 = assign33570_e38520_d_n9;
        locals.var_t3_dn10 = assign33570_e38520_d_n10;
        locals.var_t3_dn13 = assign33570_e38520_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign33580_e38538, assign33580_e38538_d_n0, assign33580_e38538_d_n2, assign33580_e38538_d_n4, assign33580_e38538_d_n5, assign33580_e38538_d_n6, assign33580_e38538_d_n7, assign33580_e38538_d_n8, assign33580_e38538_d_n9, assign33580_e38538_d_n10, assign33580_e38538_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard786 != 0.0)) {
        let assign33580_e38534: f64 = (locals.var_uc_depthn - 1e-8);
        let assign33580_e38536: f64 = (assign33580_e38534 + locals.var_tmf0);
        (assign33580_e38536, (locals.var_uc_depthn_dn0 + locals.var_tmf0_dn0), (locals.var_uc_depthn_dn2 + locals.var_tmf0_dn2), (locals.var_uc_depthn_dn4 + locals.var_tmf0_dn4), (locals.var_uc_depthn_dn5 + locals.var_tmf0_dn5), (locals.var_uc_depthn_dn6 + locals.var_tmf0_dn6), (locals.var_uc_depthn_dn7 + locals.var_tmf0_dn7), (locals.var_uc_depthn_dn8 + locals.var_tmf0_dn8), (locals.var_uc_depthn_dn9 + locals.var_tmf0_dn9), (locals.var_uc_depthn_dn10 + locals.var_tmf0_dn10), (locals.var_uc_depthn_dn13 + locals.var_tmf0_dn13),)
    } else {
        (locals.var_w_bl, locals.var_w_bl_dn0, locals.var_w_bl_dn2, locals.var_w_bl_dn4, locals.var_w_bl_dn5, locals.var_w_bl_dn6, locals.var_w_bl_dn7, locals.var_w_bl_dn8, locals.var_w_bl_dn9, locals.var_w_bl_dn10, locals.var_w_bl_dn13,)
    }
};
        locals.var_w_bl = assign33580_e38538;
        locals.var_w_bl_dn0 = assign33580_e38538_d_n0;
        locals.var_w_bl_dn2 = assign33580_e38538_d_n2;
        locals.var_w_bl_dn4 = assign33580_e38538_d_n4;
        locals.var_w_bl_dn5 = assign33580_e38538_d_n5;
        locals.var_w_bl_dn6 = assign33580_e38538_d_n6;
        locals.var_w_bl_dn7 = assign33580_e38538_d_n7;
        locals.var_w_bl_dn8 = assign33580_e38538_d_n8;
        locals.var_w_bl_dn9 = assign33580_e38538_d_n9;
        locals.var_w_bl_dn10 = assign33580_e38538_d_n10;
        locals.var_w_bl_dn13 = assign33580_e38538_d_n13;
        locals.var_w_bl_rv = 0.0;

        let (assign33590_e38552, assign33590_e38552_d_n0, assign33590_e38552_d_n2, assign33590_e38552_d_n4, assign33590_e38552_d_n5, assign33590_e38552_d_n6, assign33590_e38552_d_n7, assign33590_e38552_d_n8, assign33590_e38552_d_n9, assign33590_e38552_d_n10, assign33590_e38552_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard786 != 0.0)) {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign33590_e38552;
        locals.var_t3_dn0 = assign33590_e38552_d_n0;
        locals.var_t3_dn2 = assign33590_e38552_d_n2;
        locals.var_t3_dn4 = assign33590_e38552_d_n4;
        locals.var_t3_dn5 = assign33590_e38552_d_n5;
        locals.var_t3_dn6 = assign33590_e38552_d_n6;
        locals.var_t3_dn7 = assign33590_e38552_d_n7;
        locals.var_t3_dn8 = assign33590_e38552_d_n8;
        locals.var_t3_dn9 = assign33590_e38552_d_n9;
        locals.var_t3_dn10 = assign33590_e38552_d_n10;
        locals.var_t3_dn13 = assign33590_e38552_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign33600_e38567, assign33600_e38567_d_n0, assign33600_e38567_d_n2, assign33600_e38567_d_n4, assign33600_e38567_d_n5, assign33600_e38567_d_n6, assign33600_e38567_d_n7, assign33600_e38567_d_n8, assign33600_e38567_d_n9, assign33600_e38567_d_n10, assign33600_e38567_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard786 == 0.0)) {
        (locals.var_w_bl, locals.var_w_bl_dn0, locals.var_w_bl_dn2, locals.var_w_bl_dn4, locals.var_w_bl_dn5, locals.var_w_bl_dn6, locals.var_w_bl_dn7, locals.var_w_bl_dn8, locals.var_w_bl_dn9, locals.var_w_bl_dn10, locals.var_w_bl_dn13,)
    } else {
        (locals.var_w_bl, locals.var_w_bl_dn0, locals.var_w_bl_dn2, locals.var_w_bl_dn4, locals.var_w_bl_dn5, locals.var_w_bl_dn6, locals.var_w_bl_dn7, locals.var_w_bl_dn8, locals.var_w_bl_dn9, locals.var_w_bl_dn10, locals.var_w_bl_dn13,)
    }
};
        locals.var_w_bl = assign33600_e38567;
        locals.var_w_bl_dn0 = assign33600_e38567_d_n0;
        locals.var_w_bl_dn2 = assign33600_e38567_d_n2;
        locals.var_w_bl_dn4 = assign33600_e38567_d_n4;
        locals.var_w_bl_dn5 = assign33600_e38567_d_n5;
        locals.var_w_bl_dn6 = assign33600_e38567_d_n6;
        locals.var_w_bl_dn7 = assign33600_e38567_d_n7;
        locals.var_w_bl_dn8 = assign33600_e38567_d_n8;
        locals.var_w_bl_dn9 = assign33600_e38567_d_n9;
        locals.var_w_bl_dn10 = assign33600_e38567_d_n10;
        locals.var_w_bl_dn13 = assign33600_e38567_d_n13;
        locals.var_w_bl_rv = 0.0;

        let (assign33610_e38582, assign33610_e38582_d_n0, assign33610_e38582_d_n2, assign33610_e38582_d_n4, assign33610_e38582_d_n5, assign33610_e38582_d_n6, assign33610_e38582_d_n7, assign33610_e38582_d_n8, assign33610_e38582_d_n9, assign33610_e38582_d_n10, assign33610_e38582_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard786 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign33610_e38582;
        locals.var_t3_dn0 = assign33610_e38582_d_n0;
        locals.var_t3_dn2 = assign33610_e38582_d_n2;
        locals.var_t3_dn4 = assign33610_e38582_d_n4;
        locals.var_t3_dn5 = assign33610_e38582_d_n5;
        locals.var_t3_dn6 = assign33610_e38582_d_n6;
        locals.var_t3_dn7 = assign33610_e38582_d_n7;
        locals.var_t3_dn8 = assign33610_e38582_d_n8;
        locals.var_t3_dn9 = assign33610_e38582_d_n9;
        locals.var_t3_dn10 = assign33610_e38582_d_n10;
        locals.var_t3_dn13 = assign33610_e38582_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign33620_e38601, assign33620_e38601_d_n0, assign33620_e38601_d_n2, assign33620_e38601_d_n4, assign33620_e38601_d_n5, assign33620_e38601_d_n6, assign33620_e38601_d_n7, assign33620_e38601_d_n8, assign33620_e38601_d_n9, assign33620_e38601_d_n10, assign33620_e38601_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) {
        let assign33620_e38595: f64 = (locals.var_phi_jl_dep - locals.var_vbscl__blk435);
        let assign33620_e38597: f64 = (assign33620_e38595 + locals.var_vbi_dep);
        let assign33620_e38598: f64 = (locals.var_c_2esipq_nsub * assign33620_e38597);
        let assign33620_e38599: f64 = (assign33620_e38598).sqrt();
        (assign33620_e38599, (((locals.var_c_2esipq_nsub_dn0 * assign33620_e38597) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn0 - locals.var_vbscl__blk435_dn0) + locals.var_vbi_dep_dn0))) / (2.0 * assign33620_e38599)), (((locals.var_c_2esipq_nsub_dn2 * assign33620_e38597) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn2 - locals.var_vbscl__blk435_dn2) + locals.var_vbi_dep_dn2))) / (2.0 * assign33620_e38599)), (((locals.var_c_2esipq_nsub_dn4 * assign33620_e38597) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn4 - locals.var_vbscl__blk435_dn4) + locals.var_vbi_dep_dn4))) / (2.0 * assign33620_e38599)), (((locals.var_c_2esipq_nsub_dn5 * assign33620_e38597) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn5 - locals.var_vbscl__blk435_dn5) + locals.var_vbi_dep_dn5))) / (2.0 * assign33620_e38599)), (((locals.var_c_2esipq_nsub_dn6 * assign33620_e38597) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn6 - locals.var_vbscl__blk435_dn6) + locals.var_vbi_dep_dn6))) / (2.0 * assign33620_e38599)), (((locals.var_c_2esipq_nsub_dn7 * assign33620_e38597) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn7 - locals.var_vbscl__blk435_dn7) + locals.var_vbi_dep_dn7))) / (2.0 * assign33620_e38599)), (((locals.var_c_2esipq_nsub_dn8 * assign33620_e38597) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn8 - locals.var_vbscl__blk435_dn8) + locals.var_vbi_dep_dn8))) / (2.0 * assign33620_e38599)), (((locals.var_c_2esipq_nsub_dn9 * assign33620_e38597) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn9 - locals.var_vbscl__blk435_dn9) + locals.var_vbi_dep_dn9))) / (2.0 * assign33620_e38599)), (((locals.var_c_2esipq_nsub_dn10 * assign33620_e38597) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn10 - locals.var_vbscl__blk435_dn10) + locals.var_vbi_dep_dn10))) / (2.0 * assign33620_e38599)), (((locals.var_c_2esipq_nsub_dn13 * assign33620_e38597) + (locals.var_c_2esipq_nsub * ((locals.var_phi_jl_dep_dn13 - locals.var_vbscl__blk435_dn13) + locals.var_vbi_dep_dn13))) / (2.0 * assign33620_e38599)),)
    } else {
        (locals.var_w_subl, locals.var_w_subl_dn0, locals.var_w_subl_dn2, locals.var_w_subl_dn4, locals.var_w_subl_dn5, locals.var_w_subl_dn6, locals.var_w_subl_dn7, locals.var_w_subl_dn8, locals.var_w_subl_dn9, locals.var_w_subl_dn10, locals.var_w_subl_dn13,)
    }
};
        locals.var_w_subl = assign33620_e38601;
        locals.var_w_subl_dn0 = assign33620_e38601_d_n0;
        locals.var_w_subl_dn2 = assign33620_e38601_d_n2;
        locals.var_w_subl_dn4 = assign33620_e38601_d_n4;
        locals.var_w_subl_dn5 = assign33620_e38601_d_n5;
        locals.var_w_subl_dn6 = assign33620_e38601_d_n6;
        locals.var_w_subl_dn7 = assign33620_e38601_d_n7;
        locals.var_w_subl_dn8 = assign33620_e38601_d_n8;
        locals.var_w_subl_dn9 = assign33620_e38601_d_n9;
        locals.var_w_subl_dn10 = assign33620_e38601_d_n10;
        locals.var_w_subl_dn13 = assign33620_e38601_d_n13;
        locals.var_w_subl_rv = 0.0;

        let (assign33630_e38615, assign33630_e38615_d_n0, assign33630_e38615_d_n2, assign33630_e38615_d_n4, assign33630_e38615_d_n5, assign33630_e38615_d_n6, assign33630_e38615_d_n7, assign33630_e38615_d_n8, assign33630_e38615_d_n9, assign33630_e38615_d_n10, assign33630_e38615_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) {
        let assign33630_e38613: f64 = (locals.var_w_bl * locals.var_q_ndepm);
        (assign33630_e38613, ((locals.var_w_bl_dn0 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn0)), ((locals.var_w_bl_dn2 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn2)), ((locals.var_w_bl_dn4 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn4)), ((locals.var_w_bl_dn5 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn5)), ((locals.var_w_bl_dn6 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn6)), ((locals.var_w_bl_dn7 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn7)), ((locals.var_w_bl_dn8 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn8)), ((locals.var_w_bl_dn9 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn9)), ((locals.var_w_bl_dn10 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn10)), ((locals.var_w_bl_dn13 * locals.var_q_ndepm) + (locals.var_w_bl * locals.var_q_ndepm_dn13)),)
    } else {
        (locals.var_q_bl_dep, locals.var_q_bl_dep_dn0, locals.var_q_bl_dep_dn2, locals.var_q_bl_dep_dn4, locals.var_q_bl_dep_dn5, locals.var_q_bl_dep_dn6, locals.var_q_bl_dep_dn7, locals.var_q_bl_dep_dn8, locals.var_q_bl_dep_dn9, locals.var_q_bl_dep_dn10, locals.var_q_bl_dep_dn13,)
    }
};
        locals.var_q_bl_dep = assign33630_e38615;
        locals.var_q_bl_dep_dn0 = assign33630_e38615_d_n0;
        locals.var_q_bl_dep_dn2 = assign33630_e38615_d_n2;
        locals.var_q_bl_dep_dn4 = assign33630_e38615_d_n4;
        locals.var_q_bl_dep_dn5 = assign33630_e38615_d_n5;
        locals.var_q_bl_dep_dn6 = assign33630_e38615_d_n6;
        locals.var_q_bl_dep_dn7 = assign33630_e38615_d_n7;
        locals.var_q_bl_dep_dn8 = assign33630_e38615_d_n8;
        locals.var_q_bl_dep_dn9 = assign33630_e38615_d_n9;
        locals.var_q_bl_dep_dn10 = assign33630_e38615_d_n10;
        locals.var_q_bl_dep_dn13 = assign33630_e38615_d_n13;
        locals.var_q_bl_dep_rv = 0.0;

        let (assign33640_e38630, assign33640_e38630_d_n0, assign33640_e38630_d_n2, assign33640_e38630_d_n4, assign33640_e38630_d_n5, assign33640_e38630_d_n6, assign33640_e38630_d_n7, assign33640_e38630_d_n8, assign33640_e38630_d_n9, assign33640_e38630_d_n10, assign33640_e38630_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard772 == 0.0)) {
        let assign33640_e38626: f64 = (-locals.var_w_subl);
        let assign33640_e38628: f64 = (assign33640_e38626 * locals.var_q_nsub__blk544);
        (assign33640_e38628, (((-locals.var_w_subl_dn0) * locals.var_q_nsub__blk544) + (assign33640_e38626 * locals.var_q_nsub__blk544_dn0)), (((-locals.var_w_subl_dn2) * locals.var_q_nsub__blk544) + (assign33640_e38626 * locals.var_q_nsub__blk544_dn2)), (((-locals.var_w_subl_dn4) * locals.var_q_nsub__blk544) + (assign33640_e38626 * locals.var_q_nsub__blk544_dn4)), (((-locals.var_w_subl_dn5) * locals.var_q_nsub__blk544) + (assign33640_e38626 * locals.var_q_nsub__blk544_dn5)), (((-locals.var_w_subl_dn6) * locals.var_q_nsub__blk544) + (assign33640_e38626 * locals.var_q_nsub__blk544_dn6)), (((-locals.var_w_subl_dn7) * locals.var_q_nsub__blk544) + (assign33640_e38626 * locals.var_q_nsub__blk544_dn7)), (((-locals.var_w_subl_dn8) * locals.var_q_nsub__blk544) + (assign33640_e38626 * locals.var_q_nsub__blk544_dn8)), (((-locals.var_w_subl_dn9) * locals.var_q_nsub__blk544) + (assign33640_e38626 * locals.var_q_nsub__blk544_dn9)), (((-locals.var_w_subl_dn10) * locals.var_q_nsub__blk544) + (assign33640_e38626 * locals.var_q_nsub__blk544_dn10)), (((-locals.var_w_subl_dn13) * locals.var_q_nsub__blk544) + (assign33640_e38626 * locals.var_q_nsub__blk544_dn13)),)
    } else {
        (locals.var_q_subl_dep, locals.var_q_subl_dep_dn0, locals.var_q_subl_dep_dn2, locals.var_q_subl_dep_dn4, locals.var_q_subl_dep_dn5, locals.var_q_subl_dep_dn6, locals.var_q_subl_dep_dn7, locals.var_q_subl_dep_dn8, locals.var_q_subl_dep_dn9, locals.var_q_subl_dep_dn10, locals.var_q_subl_dep_dn13,)
    }
};
        locals.var_q_subl_dep = assign33640_e38630;
        locals.var_q_subl_dep_dn0 = assign33640_e38630_d_n0;
        locals.var_q_subl_dep_dn2 = assign33640_e38630_d_n2;
        locals.var_q_subl_dep_dn4 = assign33640_e38630_d_n4;
        locals.var_q_subl_dep_dn5 = assign33640_e38630_d_n5;
        locals.var_q_subl_dep_dn6 = assign33640_e38630_d_n6;
        locals.var_q_subl_dep_dn7 = assign33640_e38630_d_n7;
        locals.var_q_subl_dep_dn8 = assign33640_e38630_d_n8;
        locals.var_q_subl_dep_dn9 = assign33640_e38630_d_n9;
        locals.var_q_subl_dep_dn10 = assign33640_e38630_d_n10;
        locals.var_q_subl_dep_dn13 = assign33640_e38630_d_n13;
        locals.var_q_subl_dep_rv = 0.0;

        let assign33650_e38633: f64 = (locals.var_phi_sl_dep - locals.var_vds_maxbl);
        let assign33650_e38636: f64 = 0.06;
        let assign33650_e38641: f64 = if ((assign33650_e38633 < assign33650_e38636) && (0.06 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard792 = assign33650_e38641;
        locals.var_guard792_rv = 0.0;

        let (assign33660_e38658, assign33660_e38658_d_n0, assign33660_e38658_d_n2, assign33660_e38658_d_n4, assign33660_e38658_d_n5, assign33660_e38658_d_n6, assign33660_e38658_d_n7, assign33660_e38658_d_n8, assign33660_e38658_d_n9, assign33660_e38658_d_n10, assign33660_e38658_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard792 != 0.0)) {
        let assign33660_e38652: f64 = 0.06;
        let assign33660_e38655: f64 = (locals.var_phi_sl_dep - locals.var_vds_maxbl);
        let assign33660_e38656: f64 = (assign33660_e38652 - assign33660_e38655);
        (assign33660_e38656, (-(locals.var_phi_sl_dep_dn0 - locals.var_vds_maxbl_dn0)), (-(locals.var_phi_sl_dep_dn2 - locals.var_vds_maxbl_dn2)), (-(locals.var_phi_sl_dep_dn4 - locals.var_vds_maxbl_dn4)), (-(locals.var_phi_sl_dep_dn5 - locals.var_vds_maxbl_dn5)), (-(locals.var_phi_sl_dep_dn6 - locals.var_vds_maxbl_dn6)), (-(locals.var_phi_sl_dep_dn7 - locals.var_vds_maxbl_dn7)), (-(locals.var_phi_sl_dep_dn8 - locals.var_vds_maxbl_dn8)), (-(locals.var_phi_sl_dep_dn9 - locals.var_vds_maxbl_dn9)), (-(locals.var_phi_sl_dep_dn10 - locals.var_vds_maxbl_dn10)), (-(locals.var_phi_sl_dep_dn13 - locals.var_vds_maxbl_dn13)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign33660_e38658;
        locals.var_tmf1_dn0 = assign33660_e38658_d_n0;
        locals.var_tmf1_dn2 = assign33660_e38658_d_n2;
        locals.var_tmf1_dn4 = assign33660_e38658_d_n4;
        locals.var_tmf1_dn5 = assign33660_e38658_d_n5;
        locals.var_tmf1_dn6 = assign33660_e38658_d_n6;
        locals.var_tmf1_dn7 = assign33660_e38658_d_n7;
        locals.var_tmf1_dn8 = assign33660_e38658_d_n8;
        locals.var_tmf1_dn9 = assign33660_e38658_d_n9;
        locals.var_tmf1_dn10 = assign33660_e38658_d_n10;
        locals.var_tmf1_dn13 = assign33660_e38658_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign33670_e38671, assign33670_e38671_d_n0, assign33670_e38671_d_n2, assign33670_e38671_d_n4, assign33670_e38671_d_n5, assign33670_e38671_d_n6, assign33670_e38671_d_n7, assign33670_e38671_d_n8, assign33670_e38671_d_n9, assign33670_e38671_d_n10, assign33670_e38671_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard792 != 0.0)) {
        let assign33670_e38669: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign33670_e38669, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign33670_e38671;
        locals.var_x2_dn0 = assign33670_e38671_d_n0;
        locals.var_x2_dn2 = assign33670_e38671_d_n2;
        locals.var_x2_dn4 = assign33670_e38671_d_n4;
        locals.var_x2_dn5 = assign33670_e38671_d_n5;
        locals.var_x2_dn6 = assign33670_e38671_d_n6;
        locals.var_x2_dn7 = assign33670_e38671_d_n7;
        locals.var_x2_dn8 = assign33670_e38671_d_n8;
        locals.var_x2_dn9 = assign33670_e38671_d_n9;
        locals.var_x2_dn10 = assign33670_e38671_d_n10;
        locals.var_x2_dn13 = assign33670_e38671_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign33680_e38684, assign33680_e38684_d_n0, assign33680_e38684_d_n2, assign33680_e38684_d_n4, assign33680_e38684_d_n5, assign33680_e38684_d_n6, assign33680_e38684_d_n7, assign33680_e38684_d_n8, assign33680_e38684_d_n9, assign33680_e38684_d_n10, assign33680_e38684_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard792 != 0.0)) {
        let assign33680_e38682: f64 = (0.06 * 0.06);
        (assign33680_e38682, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign33680_e38684;
        locals.var_xmax2_dn0 = assign33680_e38684_d_n0;
        locals.var_xmax2_dn2 = assign33680_e38684_d_n2;
        locals.var_xmax2_dn4 = assign33680_e38684_d_n4;
        locals.var_xmax2_dn5 = assign33680_e38684_d_n5;
        locals.var_xmax2_dn6 = assign33680_e38684_d_n6;
        locals.var_xmax2_dn7 = assign33680_e38684_d_n7;
        locals.var_xmax2_dn8 = assign33680_e38684_d_n8;
        locals.var_xmax2_dn9 = assign33680_e38684_d_n9;
        locals.var_xmax2_dn10 = assign33680_e38684_d_n10;
        locals.var_xmax2_dn13 = assign33680_e38684_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign33690_e38695, assign33690_e38695_d_n0, assign33690_e38695_d_n2, assign33690_e38695_d_n4, assign33690_e38695_d_n5, assign33690_e38695_d_n6, assign33690_e38695_d_n7, assign33690_e38695_d_n8, assign33690_e38695_d_n9, assign33690_e38695_d_n10, assign33690_e38695_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard792 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign33690_e38695;
        locals.var_xp_dn0 = assign33690_e38695_d_n0;
        locals.var_xp_dn2 = assign33690_e38695_d_n2;
        locals.var_xp_dn4 = assign33690_e38695_d_n4;
        locals.var_xp_dn5 = assign33690_e38695_d_n5;
        locals.var_xp_dn6 = assign33690_e38695_d_n6;
        locals.var_xp_dn7 = assign33690_e38695_d_n7;
        locals.var_xp_dn8 = assign33690_e38695_d_n8;
        locals.var_xp_dn9 = assign33690_e38695_d_n9;
        locals.var_xp_dn10 = assign33690_e38695_d_n10;
        locals.var_xp_dn13 = assign33690_e38695_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign33700_e38706, assign33700_e38706_d_n0, assign33700_e38706_d_n2, assign33700_e38706_d_n4, assign33700_e38706_d_n5, assign33700_e38706_d_n6, assign33700_e38706_d_n7, assign33700_e38706_d_n8, assign33700_e38706_d_n9, assign33700_e38706_d_n10, assign33700_e38706_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard792 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign33700_e38706;
        locals.var_xmp_dn0 = assign33700_e38706_d_n0;
        locals.var_xmp_dn2 = assign33700_e38706_d_n2;
        locals.var_xmp_dn4 = assign33700_e38706_d_n4;
        locals.var_xmp_dn5 = assign33700_e38706_d_n5;
        locals.var_xmp_dn6 = assign33700_e38706_d_n6;
        locals.var_xmp_dn7 = assign33700_e38706_d_n7;
        locals.var_xmp_dn8 = assign33700_e38706_d_n8;
        locals.var_xmp_dn9 = assign33700_e38706_d_n9;
        locals.var_xmp_dn10 = assign33700_e38706_d_n10;
        locals.var_xmp_dn13 = assign33700_e38706_d_n13;
        locals.var_xmp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_105(
        locals: &mut StampLocals,
    ) {
        let (assign33710_e38717,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard792 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign33710_e38717;
        locals.var_m0_rv = 0.0;

        let (assign33720_e38728,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard792 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33720_e38728;
        locals.var_mm_rv = 0.0;

        let (assign33730_e38739, assign33730_e38739_d_n0, assign33730_e38739_d_n2, assign33730_e38739_d_n4, assign33730_e38739_d_n5, assign33730_e38739_d_n6, assign33730_e38739_d_n7, assign33730_e38739_d_n8, assign33730_e38739_d_n9, assign33730_e38739_d_n10, assign33730_e38739_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard792 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign33730_e38739;
        locals.var_arg_dn0 = assign33730_e38739_d_n0;
        locals.var_arg_dn2 = assign33730_e38739_d_n2;
        locals.var_arg_dn4 = assign33730_e38739_d_n4;
        locals.var_arg_dn5 = assign33730_e38739_d_n5;
        locals.var_arg_dn6 = assign33730_e38739_d_n6;
        locals.var_arg_dn7 = assign33730_e38739_d_n7;
        locals.var_arg_dn8 = assign33730_e38739_d_n8;
        locals.var_arg_dn9 = assign33730_e38739_d_n9;
        locals.var_arg_dn10 = assign33730_e38739_d_n10;
        locals.var_arg_dn13 = assign33730_e38739_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign33740_e38750, assign33740_e38750_d_n0, assign33740_e38750_d_n2, assign33740_e38750_d_n4, assign33740_e38750_d_n5, assign33740_e38750_d_n6, assign33740_e38750_d_n7, assign33740_e38750_d_n8, assign33740_e38750_d_n9, assign33740_e38750_d_n10, assign33740_e38750_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard792 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign33740_e38750;
        locals.var_dnm_dn0 = assign33740_e38750_d_n0;
        locals.var_dnm_dn2 = assign33740_e38750_d_n2;
        locals.var_dnm_dn4 = assign33740_e38750_d_n4;
        locals.var_dnm_dn5 = assign33740_e38750_d_n5;
        locals.var_dnm_dn6 = assign33740_e38750_d_n6;
        locals.var_dnm_dn7 = assign33740_e38750_d_n7;
        locals.var_dnm_dn8 = assign33740_e38750_d_n8;
        locals.var_dnm_dn9 = assign33740_e38750_d_n9;
        locals.var_dnm_dn10 = assign33740_e38750_d_n10;
        locals.var_dnm_dn13 = assign33740_e38750_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign33750_e38763, assign33750_e38763_d_n0, assign33750_e38763_d_n2, assign33750_e38763_d_n4, assign33750_e38763_d_n5, assign33750_e38763_d_n6, assign33750_e38763_d_n7, assign33750_e38763_d_n8, assign33750_e38763_d_n9, assign33750_e38763_d_n10, assign33750_e38763_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard792 != 0.0)) {
        let assign33750_e38761: f64 = (locals.var_xp * locals.var_x2);
        (assign33750_e38761, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign33750_e38763;
        locals.var_xp_dn0 = assign33750_e38763_d_n0;
        locals.var_xp_dn2 = assign33750_e38763_d_n2;
        locals.var_xp_dn4 = assign33750_e38763_d_n4;
        locals.var_xp_dn5 = assign33750_e38763_d_n5;
        locals.var_xp_dn6 = assign33750_e38763_d_n6;
        locals.var_xp_dn7 = assign33750_e38763_d_n7;
        locals.var_xp_dn8 = assign33750_e38763_d_n8;
        locals.var_xp_dn9 = assign33750_e38763_d_n9;
        locals.var_xp_dn10 = assign33750_e38763_d_n10;
        locals.var_xp_dn13 = assign33750_e38763_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign33760_e38776, assign33760_e38776_d_n0, assign33760_e38776_d_n2, assign33760_e38776_d_n4, assign33760_e38776_d_n5, assign33760_e38776_d_n6, assign33760_e38776_d_n7, assign33760_e38776_d_n8, assign33760_e38776_d_n9, assign33760_e38776_d_n10, assign33760_e38776_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard792 != 0.0)) {
        let assign33760_e38774: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign33760_e38774, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign33760_e38776;
        locals.var_xmp_dn0 = assign33760_e38776_d_n0;
        locals.var_xmp_dn2 = assign33760_e38776_d_n2;
        locals.var_xmp_dn4 = assign33760_e38776_d_n4;
        locals.var_xmp_dn5 = assign33760_e38776_d_n5;
        locals.var_xmp_dn6 = assign33760_e38776_d_n6;
        locals.var_xmp_dn7 = assign33760_e38776_d_n7;
        locals.var_xmp_dn8 = assign33760_e38776_d_n8;
        locals.var_xmp_dn9 = assign33760_e38776_d_n9;
        locals.var_xmp_dn10 = assign33760_e38776_d_n10;
        locals.var_xmp_dn13 = assign33760_e38776_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign33770_e38789, assign33770_e38789_d_n0, assign33770_e38789_d_n2, assign33770_e38789_d_n4, assign33770_e38789_d_n5, assign33770_e38789_d_n6, assign33770_e38789_d_n7, assign33770_e38789_d_n8, assign33770_e38789_d_n9, assign33770_e38789_d_n10, assign33770_e38789_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard792 != 0.0)) {
        let assign33770_e38787: f64 = (locals.var_xp * locals.var_x2);
        (assign33770_e38787, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign33770_e38789;
        locals.var_xp_dn0 = assign33770_e38789_d_n0;
        locals.var_xp_dn2 = assign33770_e38789_d_n2;
        locals.var_xp_dn4 = assign33770_e38789_d_n4;
        locals.var_xp_dn5 = assign33770_e38789_d_n5;
        locals.var_xp_dn6 = assign33770_e38789_d_n6;
        locals.var_xp_dn7 = assign33770_e38789_d_n7;
        locals.var_xp_dn8 = assign33770_e38789_d_n8;
        locals.var_xp_dn9 = assign33770_e38789_d_n9;
        locals.var_xp_dn10 = assign33770_e38789_d_n10;
        locals.var_xp_dn13 = assign33770_e38789_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign33780_e38802, assign33780_e38802_d_n0, assign33780_e38802_d_n2, assign33780_e38802_d_n4, assign33780_e38802_d_n5, assign33780_e38802_d_n6, assign33780_e38802_d_n7, assign33780_e38802_d_n8, assign33780_e38802_d_n9, assign33780_e38802_d_n10, assign33780_e38802_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard792 != 0.0)) {
        let assign33780_e38800: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign33780_e38800, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign33780_e38802;
        locals.var_xmp_dn0 = assign33780_e38802_d_n0;
        locals.var_xmp_dn2 = assign33780_e38802_d_n2;
        locals.var_xmp_dn4 = assign33780_e38802_d_n4;
        locals.var_xmp_dn5 = assign33780_e38802_d_n5;
        locals.var_xmp_dn6 = assign33780_e38802_d_n6;
        locals.var_xmp_dn7 = assign33780_e38802_d_n7;
        locals.var_xmp_dn8 = assign33780_e38802_d_n8;
        locals.var_xmp_dn9 = assign33780_e38802_d_n9;
        locals.var_xmp_dn10 = assign33780_e38802_d_n10;
        locals.var_xmp_dn13 = assign33780_e38802_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign33790_e38815, assign33790_e38815_d_n0, assign33790_e38815_d_n2, assign33790_e38815_d_n4, assign33790_e38815_d_n5, assign33790_e38815_d_n6, assign33790_e38815_d_n7, assign33790_e38815_d_n8, assign33790_e38815_d_n9, assign33790_e38815_d_n10, assign33790_e38815_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard792 != 0.0)) {
        let assign33790_e38813: f64 = (locals.var_xp + locals.var_xmp);
        (assign33790_e38813, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign33790_e38815;
        locals.var_arg_dn0 = assign33790_e38815_d_n0;
        locals.var_arg_dn2 = assign33790_e38815_d_n2;
        locals.var_arg_dn4 = assign33790_e38815_d_n4;
        locals.var_arg_dn5 = assign33790_e38815_d_n5;
        locals.var_arg_dn6 = assign33790_e38815_d_n6;
        locals.var_arg_dn7 = assign33790_e38815_d_n7;
        locals.var_arg_dn8 = assign33790_e38815_d_n8;
        locals.var_arg_dn9 = assign33790_e38815_d_n9;
        locals.var_arg_dn10 = assign33790_e38815_d_n10;
        locals.var_arg_dn13 = assign33790_e38815_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign33800_e38826, assign33800_e38826_d_n0, assign33800_e38826_d_n2, assign33800_e38826_d_n4, assign33800_e38826_d_n5, assign33800_e38826_d_n6, assign33800_e38826_d_n7, assign33800_e38826_d_n8, assign33800_e38826_d_n9, assign33800_e38826_d_n10, assign33800_e38826_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard792 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign33800_e38826;
        locals.var_dnm_dn0 = assign33800_e38826_d_n0;
        locals.var_dnm_dn2 = assign33800_e38826_d_n2;
        locals.var_dnm_dn4 = assign33800_e38826_d_n4;
        locals.var_dnm_dn5 = assign33800_e38826_d_n5;
        locals.var_dnm_dn6 = assign33800_e38826_d_n6;
        locals.var_dnm_dn7 = assign33800_e38826_d_n7;
        locals.var_dnm_dn8 = assign33800_e38826_d_n8;
        locals.var_dnm_dn9 = assign33800_e38826_d_n9;
        locals.var_dnm_dn10 = assign33800_e38826_d_n10;
        locals.var_dnm_dn13 = assign33800_e38826_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign33810_e38841: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard793 = assign33810_e38841;
        locals.var_guard793_rv = 0.0;

        let assign33820_e38844: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard794 = assign33820_e38844;
        locals.var_guard794_rv = 0.0;

        let (assign33830_e38859,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard792 != 0.0)) && (locals.var_guard793 != 0.0)) && (locals.var_guard794 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33830_e38859;
        locals.var_mm_rv = 0.0;

        let assign33840_e38862: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard795 = assign33840_e38862;
        locals.var_guard795_rv = 0.0;

        let (assign33850_e38880,) = {
    if (((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard792 != 0.0)) && (locals.var_guard793 != 0.0)) && (locals.var_guard794 == 0.0)) && (locals.var_guard795 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33850_e38880;
        locals.var_mm_rv = 0.0;

        let assign33860_e38883: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard796 = assign33860_e38883;
        locals.var_guard796_rv = 0.0;

        let (assign33870_e38904,) = {
    if ((((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard792 != 0.0)) && (locals.var_guard793 != 0.0)) && (locals.var_guard794 == 0.0)) && (locals.var_guard795 == 0.0)) && (locals.var_guard796 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33870_e38904;
        locals.var_mm_rv = 0.0;

        let assign33880_e38907: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard797 = assign33880_e38907;
        locals.var_guard797_rv = 0.0;

        let (assign33890_e38931,) = {
    if (((((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard792 != 0.0)) && (locals.var_guard793 != 0.0)) && (locals.var_guard794 == 0.0)) && (locals.var_guard795 == 0.0)) && (locals.var_guard796 == 0.0)) && (locals.var_guard797 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign33890_e38931;
        locals.var_mm_rv = 0.0;

        let (assign33900_e38944,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard792 != 0.0)) && (locals.var_guard793 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign33900_e38944;
        locals.var_m0_rv = 0.0;

        let mut assign33910_loop_guard: usize = 0;
        while {
            let assign33910_cond_e38958: f64 = if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard792 != 0.0)) && (locals.var_guard793 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign33910_cond_e38958 != 0.0
        } {
            assign33910_loop_guard += 1;
            assert!(assign33910_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign33910_body0_e38972, assign33910_body0_e38972_d_n0, assign33910_body0_e38972_d_n2, assign33910_body0_e38972_d_n4, assign33910_body0_e38972_d_n5, assign33910_body0_e38972_d_n6, assign33910_body0_e38972_d_n7, assign33910_body0_e38972_d_n8, assign33910_body0_e38972_d_n9, assign33910_body0_e38972_d_n10, assign33910_body0_e38972_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard792 != 0.0)) && (locals.var_guard793 != 0.0)) {
        let assign33910_body0_e38970: f64 = (locals.var_dnm).sqrt();
        (assign33910_body0_e38970, (locals.var_dnm_dn0 / (2.0 * assign33910_body0_e38970)), (locals.var_dnm_dn2 / (2.0 * assign33910_body0_e38970)), (locals.var_dnm_dn4 / (2.0 * assign33910_body0_e38970)), (locals.var_dnm_dn5 / (2.0 * assign33910_body0_e38970)), (locals.var_dnm_dn6 / (2.0 * assign33910_body0_e38970)), (locals.var_dnm_dn7 / (2.0 * assign33910_body0_e38970)), (locals.var_dnm_dn8 / (2.0 * assign33910_body0_e38970)), (locals.var_dnm_dn9 / (2.0 * assign33910_body0_e38970)), (locals.var_dnm_dn10 / (2.0 * assign33910_body0_e38970)), (locals.var_dnm_dn13 / (2.0 * assign33910_body0_e38970)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign33910_body0_e38972;
            locals.var_dnm_dn0 = assign33910_body0_e38972_d_n0;
            locals.var_dnm_dn2 = assign33910_body0_e38972_d_n2;
            locals.var_dnm_dn4 = assign33910_body0_e38972_d_n4;
            locals.var_dnm_dn5 = assign33910_body0_e38972_d_n5;
            locals.var_dnm_dn6 = assign33910_body0_e38972_d_n6;
            locals.var_dnm_dn7 = assign33910_body0_e38972_d_n7;
            locals.var_dnm_dn8 = assign33910_body0_e38972_d_n8;
            locals.var_dnm_dn9 = assign33910_body0_e38972_d_n9;
            locals.var_dnm_dn10 = assign33910_body0_e38972_d_n10;
            locals.var_dnm_dn13 = assign33910_body0_e38972_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign33910_body1_e38987,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard792 != 0.0)) && (locals.var_guard793 != 0.0)) {
        let assign33910_body1_e38985: f64 = (locals.var_m0 + 1.0);
        (assign33910_body1_e38985,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign33910_body1_e38987;
            locals.var_m0_rv = 0.0;
        }

        let (assign33920_e39012, assign33920_e39012_d_n0, assign33920_e39012_d_n2, assign33920_e39012_d_n4, assign33920_e39012_d_n5, assign33920_e39012_d_n6, assign33920_e39012_d_n7, assign33920_e39012_d_n8, assign33920_e39012_d_n9, assign33920_e39012_d_n10, assign33920_e39012_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard792 != 0.0)) && (locals.var_guard793 == 0.0)) {
        let (assign33920_e39010, assign33920_e39010_d_n0, assign33920_e39010_d_n2, assign33920_e39010_d_n4, assign33920_e39010_d_n5, assign33920_e39010_d_n6, assign33920_e39010_d_n7, assign33920_e39010_d_n8, assign33920_e39010_d_n9, assign33920_e39010_d_n10, assign33920_e39010_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign33920_e39007: f64 = (2.0 * 2.0);
                let assign33920_e39008: f64 = (1.0 / assign33920_e39007);
                let assign33920_e39009: f64 = (locals.var_dnm).powf(assign33920_e39008);
                (assign33920_e39009, if 0.0 == 0.0 && ((assign33920_e39008) as f64).is_finite() && ((assign33920_e39008) as f64).fract() == 0.0 { if assign33920_e39008 == 0.0 { 0.0 } else { (assign33920_e39008 * ((locals.var_dnm).powf(assign33920_e39008 - 1.0) * locals.var_dnm_dn0)) } } else { (assign33920_e39009 * (assign33920_e39008 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33920_e39008) as f64).is_finite() && ((assign33920_e39008) as f64).fract() == 0.0 { if assign33920_e39008 == 0.0 { 0.0 } else { (assign33920_e39008 * ((locals.var_dnm).powf(assign33920_e39008 - 1.0) * locals.var_dnm_dn2)) } } else { (assign33920_e39009 * (assign33920_e39008 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33920_e39008) as f64).is_finite() && ((assign33920_e39008) as f64).fract() == 0.0 { if assign33920_e39008 == 0.0 { 0.0 } else { (assign33920_e39008 * ((locals.var_dnm).powf(assign33920_e39008 - 1.0) * locals.var_dnm_dn4)) } } else { (assign33920_e39009 * (assign33920_e39008 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33920_e39008) as f64).is_finite() && ((assign33920_e39008) as f64).fract() == 0.0 { if assign33920_e39008 == 0.0 { 0.0 } else { (assign33920_e39008 * ((locals.var_dnm).powf(assign33920_e39008 - 1.0) * locals.var_dnm_dn5)) } } else { (assign33920_e39009 * (assign33920_e39008 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33920_e39008) as f64).is_finite() && ((assign33920_e39008) as f64).fract() == 0.0 { if assign33920_e39008 == 0.0 { 0.0 } else { (assign33920_e39008 * ((locals.var_dnm).powf(assign33920_e39008 - 1.0) * locals.var_dnm_dn6)) } } else { (assign33920_e39009 * (assign33920_e39008 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33920_e39008) as f64).is_finite() && ((assign33920_e39008) as f64).fract() == 0.0 { if assign33920_e39008 == 0.0 { 0.0 } else { (assign33920_e39008 * ((locals.var_dnm).powf(assign33920_e39008 - 1.0) * locals.var_dnm_dn7)) } } else { (assign33920_e39009 * (assign33920_e39008 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33920_e39008) as f64).is_finite() && ((assign33920_e39008) as f64).fract() == 0.0 { if assign33920_e39008 == 0.0 { 0.0 } else { (assign33920_e39008 * ((locals.var_dnm).powf(assign33920_e39008 - 1.0) * locals.var_dnm_dn8)) } } else { (assign33920_e39009 * (assign33920_e39008 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33920_e39008) as f64).is_finite() && ((assign33920_e39008) as f64).fract() == 0.0 { if assign33920_e39008 == 0.0 { 0.0 } else { (assign33920_e39008 * ((locals.var_dnm).powf(assign33920_e39008 - 1.0) * locals.var_dnm_dn9)) } } else { (assign33920_e39009 * (assign33920_e39008 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33920_e39008) as f64).is_finite() && ((assign33920_e39008) as f64).fract() == 0.0 { if assign33920_e39008 == 0.0 { 0.0 } else { (assign33920_e39008 * ((locals.var_dnm).powf(assign33920_e39008 - 1.0) * locals.var_dnm_dn10)) } } else { (assign33920_e39009 * (assign33920_e39008 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign33920_e39008) as f64).is_finite() && ((assign33920_e39008) as f64).fract() == 0.0 { if assign33920_e39008 == 0.0 { 0.0 } else { (assign33920_e39008 * ((locals.var_dnm).powf(assign33920_e39008 - 1.0) * locals.var_dnm_dn13)) } } else { (assign33920_e39009 * (assign33920_e39008 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign33920_e39010, assign33920_e39010_d_n0, assign33920_e39010_d_n2, assign33920_e39010_d_n4, assign33920_e39010_d_n5, assign33920_e39010_d_n6, assign33920_e39010_d_n7, assign33920_e39010_d_n8, assign33920_e39010_d_n9, assign33920_e39010_d_n10, assign33920_e39010_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign33920_e39012;
        locals.var_dnm_dn0 = assign33920_e39012_d_n0;
        locals.var_dnm_dn2 = assign33920_e39012_d_n2;
        locals.var_dnm_dn4 = assign33920_e39012_d_n4;
        locals.var_dnm_dn5 = assign33920_e39012_d_n5;
        locals.var_dnm_dn6 = assign33920_e39012_d_n6;
        locals.var_dnm_dn7 = assign33920_e39012_d_n7;
        locals.var_dnm_dn8 = assign33920_e39012_d_n8;
        locals.var_dnm_dn9 = assign33920_e39012_d_n9;
        locals.var_dnm_dn10 = assign33920_e39012_d_n10;
        locals.var_dnm_dn13 = assign33920_e39012_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign33930_e39025, assign33930_e39025_d_n0, assign33930_e39025_d_n2, assign33930_e39025_d_n4, assign33930_e39025_d_n5, assign33930_e39025_d_n6, assign33930_e39025_d_n7, assign33930_e39025_d_n8, assign33930_e39025_d_n9, assign33930_e39025_d_n10, assign33930_e39025_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard792 != 0.0)) {
        let assign33930_e39023: f64 = (1.0 / locals.var_dnm);
        (assign33930_e39023, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign33930_e39025;
        locals.var_dnm_dn0 = assign33930_e39025_d_n0;
        locals.var_dnm_dn2 = assign33930_e39025_d_n2;
        locals.var_dnm_dn4 = assign33930_e39025_d_n4;
        locals.var_dnm_dn5 = assign33930_e39025_d_n5;
        locals.var_dnm_dn6 = assign33930_e39025_d_n6;
        locals.var_dnm_dn7 = assign33930_e39025_d_n7;
        locals.var_dnm_dn8 = assign33930_e39025_d_n8;
        locals.var_dnm_dn9 = assign33930_e39025_d_n9;
        locals.var_dnm_dn10 = assign33930_e39025_d_n10;
        locals.var_dnm_dn13 = assign33930_e39025_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign33940_e39040, assign33940_e39040_d_n0, assign33940_e39040_d_n2, assign33940_e39040_d_n4, assign33940_e39040_d_n5, assign33940_e39040_d_n6, assign33940_e39040_d_n7, assign33940_e39040_d_n8, assign33940_e39040_d_n9, assign33940_e39040_d_n10, assign33940_e39040_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard792 != 0.0)) {
        let assign33940_e39036: f64 = (locals.var_tmf1 * 0.06);
        let assign33940_e39038: f64 = (assign33940_e39036 * locals.var_dnm);
        (assign33940_e39038, (((locals.var_tmf1_dn0 * 0.06) * locals.var_dnm) + (assign33940_e39036 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.06) * locals.var_dnm) + (assign33940_e39036 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.06) * locals.var_dnm) + (assign33940_e39036 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.06) * locals.var_dnm) + (assign33940_e39036 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.06) * locals.var_dnm) + (assign33940_e39036 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.06) * locals.var_dnm) + (assign33940_e39036 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.06) * locals.var_dnm) + (assign33940_e39036 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.06) * locals.var_dnm) + (assign33940_e39036 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.06) * locals.var_dnm) + (assign33940_e39036 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * 0.06) * locals.var_dnm) + (assign33940_e39036 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign33940_e39040;
        locals.var_tmf0_dn0 = assign33940_e39040_d_n0;
        locals.var_tmf0_dn2 = assign33940_e39040_d_n2;
        locals.var_tmf0_dn4 = assign33940_e39040_d_n4;
        locals.var_tmf0_dn5 = assign33940_e39040_d_n5;
        locals.var_tmf0_dn6 = assign33940_e39040_d_n6;
        locals.var_tmf0_dn7 = assign33940_e39040_d_n7;
        locals.var_tmf0_dn8 = assign33940_e39040_d_n8;
        locals.var_tmf0_dn9 = assign33940_e39040_d_n9;
        locals.var_tmf0_dn10 = assign33940_e39040_d_n10;
        locals.var_tmf0_dn13 = assign33940_e39040_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign33950_e39057, assign33950_e39057_d_n0, assign33950_e39057_d_n2, assign33950_e39057_d_n4, assign33950_e39057_d_n5, assign33950_e39057_d_n6, assign33950_e39057_d_n7, assign33950_e39057_d_n8, assign33950_e39057_d_n9, assign33950_e39057_d_n10, assign33950_e39057_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard792 != 0.0)) {
        let assign33950_e39051: f64 = (0.06 * locals.var_xmp);
        let assign33950_e39053: f64 = (assign33950_e39051 * locals.var_dnm);
        let assign33950_e39055: f64 = (assign33950_e39053 / locals.var_arg);
        (assign33950_e39055, ((((((0.06 * locals.var_xmp_dn0) * locals.var_dnm) + (assign33950_e39051 * locals.var_dnm_dn0)) * locals.var_arg) - (assign33950_e39053 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn2) * locals.var_dnm) + (assign33950_e39051 * locals.var_dnm_dn2)) * locals.var_arg) - (assign33950_e39053 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn4) * locals.var_dnm) + (assign33950_e39051 * locals.var_dnm_dn4)) * locals.var_arg) - (assign33950_e39053 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn5) * locals.var_dnm) + (assign33950_e39051 * locals.var_dnm_dn5)) * locals.var_arg) - (assign33950_e39053 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn6) * locals.var_dnm) + (assign33950_e39051 * locals.var_dnm_dn6)) * locals.var_arg) - (assign33950_e39053 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn7) * locals.var_dnm) + (assign33950_e39051 * locals.var_dnm_dn7)) * locals.var_arg) - (assign33950_e39053 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn8) * locals.var_dnm) + (assign33950_e39051 * locals.var_dnm_dn8)) * locals.var_arg) - (assign33950_e39053 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn9) * locals.var_dnm) + (assign33950_e39051 * locals.var_dnm_dn9)) * locals.var_arg) - (assign33950_e39053 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn10) * locals.var_dnm) + (assign33950_e39051 * locals.var_dnm_dn10)) * locals.var_arg) - (assign33950_e39053 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.06 * locals.var_xmp_dn13) * locals.var_dnm) + (assign33950_e39051 * locals.var_dnm_dn13)) * locals.var_arg) - (assign33950_e39053 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign33950_e39057;
        locals.var_t0_dn0 = assign33950_e39057_d_n0;
        locals.var_t0_dn2 = assign33950_e39057_d_n2;
        locals.var_t0_dn4 = assign33950_e39057_d_n4;
        locals.var_t0_dn5 = assign33950_e39057_d_n5;
        locals.var_t0_dn6 = assign33950_e39057_d_n6;
        locals.var_t0_dn7 = assign33950_e39057_d_n7;
        locals.var_t0_dn8 = assign33950_e39057_d_n8;
        locals.var_t0_dn9 = assign33950_e39057_d_n9;
        locals.var_t0_dn10 = assign33950_e39057_d_n10;
        locals.var_t0_dn13 = assign33950_e39057_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign33960_e39072, assign33960_e39072_d_n0, assign33960_e39072_d_n2, assign33960_e39072_d_n4, assign33960_e39072_d_n5, assign33960_e39072_d_n6, assign33960_e39072_d_n7, assign33960_e39072_d_n8, assign33960_e39072_d_n9, assign33960_e39072_d_n10, assign33960_e39072_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard792 != 0.0)) {
        let assign33960_e39068: f64 = 0.06;
        let assign33960_e39070: f64 = (assign33960_e39068 - locals.var_tmf0);
        (assign33960_e39070, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign33960_e39072;
        locals.var_t2_dn0 = assign33960_e39072_d_n0;
        locals.var_t2_dn2 = assign33960_e39072_d_n2;
        locals.var_t2_dn4 = assign33960_e39072_d_n4;
        locals.var_t2_dn5 = assign33960_e39072_d_n5;
        locals.var_t2_dn6 = assign33960_e39072_d_n6;
        locals.var_t2_dn7 = assign33960_e39072_d_n7;
        locals.var_t2_dn8 = assign33960_e39072_d_n8;
        locals.var_t2_dn9 = assign33960_e39072_d_n9;
        locals.var_t2_dn10 = assign33960_e39072_d_n10;
        locals.var_t2_dn13 = assign33960_e39072_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign33970_e39083, assign33970_e39083_d_n0, assign33970_e39083_d_n2, assign33970_e39083_d_n4, assign33970_e39083_d_n5, assign33970_e39083_d_n6, assign33970_e39083_d_n7, assign33970_e39083_d_n8, assign33970_e39083_d_n9, assign33970_e39083_d_n10, assign33970_e39083_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard792 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign33970_e39083;
        locals.var_t0_dn0 = assign33970_e39083_d_n0;
        locals.var_t0_dn2 = assign33970_e39083_d_n2;
        locals.var_t0_dn4 = assign33970_e39083_d_n4;
        locals.var_t0_dn5 = assign33970_e39083_d_n5;
        locals.var_t0_dn6 = assign33970_e39083_d_n6;
        locals.var_t0_dn7 = assign33970_e39083_d_n7;
        locals.var_t0_dn8 = assign33970_e39083_d_n8;
        locals.var_t0_dn9 = assign33970_e39083_d_n9;
        locals.var_t0_dn10 = assign33970_e39083_d_n10;
        locals.var_t0_dn13 = assign33970_e39083_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign33980_e39097, assign33980_e39097_d_n0, assign33980_e39097_d_n2, assign33980_e39097_d_n4, assign33980_e39097_d_n5, assign33980_e39097_d_n6, assign33980_e39097_d_n7, assign33980_e39097_d_n8, assign33980_e39097_d_n9, assign33980_e39097_d_n10, assign33980_e39097_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard792 == 0.0)) {
        let assign33980_e39095: f64 = (locals.var_phi_sl_dep - locals.var_vds_maxbl);
        (assign33980_e39095, (locals.var_phi_sl_dep_dn0 - locals.var_vds_maxbl_dn0), (locals.var_phi_sl_dep_dn2 - locals.var_vds_maxbl_dn2), (locals.var_phi_sl_dep_dn4 - locals.var_vds_maxbl_dn4), (locals.var_phi_sl_dep_dn5 - locals.var_vds_maxbl_dn5), (locals.var_phi_sl_dep_dn6 - locals.var_vds_maxbl_dn6), (locals.var_phi_sl_dep_dn7 - locals.var_vds_maxbl_dn7), (locals.var_phi_sl_dep_dn8 - locals.var_vds_maxbl_dn8), (locals.var_phi_sl_dep_dn9 - locals.var_vds_maxbl_dn9), (locals.var_phi_sl_dep_dn10 - locals.var_vds_maxbl_dn10), (locals.var_phi_sl_dep_dn13 - locals.var_vds_maxbl_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign33980_e39097;
        locals.var_t2_dn0 = assign33980_e39097_d_n0;
        locals.var_t2_dn2 = assign33980_e39097_d_n2;
        locals.var_t2_dn4 = assign33980_e39097_d_n4;
        locals.var_t2_dn5 = assign33980_e39097_d_n5;
        locals.var_t2_dn6 = assign33980_e39097_d_n6;
        locals.var_t2_dn7 = assign33980_e39097_d_n7;
        locals.var_t2_dn8 = assign33980_e39097_d_n8;
        locals.var_t2_dn9 = assign33980_e39097_d_n9;
        locals.var_t2_dn10 = assign33980_e39097_d_n10;
        locals.var_t2_dn13 = assign33980_e39097_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign33990_e39109, assign33990_e39109_d_n0, assign33990_e39109_d_n2, assign33990_e39109_d_n4, assign33990_e39109_d_n5, assign33990_e39109_d_n6, assign33990_e39109_d_n7, assign33990_e39109_d_n8, assign33990_e39109_d_n9, assign33990_e39109_d_n10, assign33990_e39109_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) && (locals.var_guard792 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign33990_e39109;
        locals.var_t0_dn0 = assign33990_e39109_d_n0;
        locals.var_t0_dn2 = assign33990_e39109_d_n2;
        locals.var_t0_dn4 = assign33990_e39109_d_n4;
        locals.var_t0_dn5 = assign33990_e39109_d_n5;
        locals.var_t0_dn6 = assign33990_e39109_d_n6;
        locals.var_t0_dn7 = assign33990_e39109_d_n7;
        locals.var_t0_dn8 = assign33990_e39109_d_n8;
        locals.var_t0_dn9 = assign33990_e39109_d_n9;
        locals.var_t0_dn10 = assign33990_e39109_d_n10;
        locals.var_t0_dn13 = assign33990_e39109_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign34000_e39131, assign34000_e39131_d_n0, assign34000_e39131_d_n2, assign34000_e39131_d_n4, assign34000_e39131_d_n5, assign34000_e39131_d_n6, assign34000_e39131_d_n7, assign34000_e39131_d_n8, assign34000_e39131_d_n9, assign34000_e39131_d_n10, assign34000_e39131_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) {
        let assign34000_e39118: f64 = (locals.var_beta * locals.var_t2);
        let assign34000_e39119: f64 = (assign34000_e39118).exp();
        let assign34000_e39121: f64 = (assign34000_e39119 - 1.0);
        let assign34000_e39124: f64 = (locals.var_beta * locals.var_t2);
        let assign34000_e39125: f64 = (assign34000_e39121 - assign34000_e39124);
        let assign34000_e39128: f64 = (10.0 * 2.220446049250313e-16);
        let assign34000_e39129: f64 = (assign34000_e39125 + assign34000_e39128);
        (assign34000_e39129, ((assign34000_e39119 * ((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0))) - ((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0))), ((assign34000_e39119 * ((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2))) - ((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2))), ((assign34000_e39119 * ((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4))) - ((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4))), ((assign34000_e39119 * ((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5))) - ((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5))), ((assign34000_e39119 * ((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6))) - ((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6))), ((assign34000_e39119 * ((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7))) - ((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7))), ((assign34000_e39119 * ((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8))) - ((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8))), ((assign34000_e39119 * ((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9))) - ((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9))), ((assign34000_e39119 * ((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10))) - ((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10))), ((assign34000_e39119 * ((locals.var_beta_dn13 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn13))) - ((locals.var_beta_dn13 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn13))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign34000_e39131;
        locals.var_t4_dn0 = assign34000_e39131_d_n0;
        locals.var_t4_dn2 = assign34000_e39131_d_n2;
        locals.var_t4_dn4 = assign34000_e39131_d_n4;
        locals.var_t4_dn5 = assign34000_e39131_d_n5;
        locals.var_t4_dn6 = assign34000_e39131_d_n6;
        locals.var_t4_dn7 = assign34000_e39131_d_n7;
        locals.var_t4_dn8 = assign34000_e39131_d_n8;
        locals.var_t4_dn9 = assign34000_e39131_d_n9;
        locals.var_t4_dn10 = assign34000_e39131_d_n10;
        locals.var_t4_dn13 = assign34000_e39131_d_n13;
        locals.var_t4_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_106(
        locals: &mut StampLocals,
    ) {
        let (assign34010_e39144, assign34010_e39144_d_n0, assign34010_e39144_d_n2, assign34010_e39144_d_n4, assign34010_e39144_d_n5, assign34010_e39144_d_n6, assign34010_e39144_d_n7, assign34010_e39144_d_n8, assign34010_e39144_d_n9, assign34010_e39144_d_n10, assign34010_e39144_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard711 == 0.0)) {
        let assign34010_e39139: f64 = (-locals.var_cnst0);
        let assign34010_e39141: f64 = (locals.var_t4).sqrt();
        let assign34010_e39142: f64 = (assign34010_e39139 * assign34010_e39141);
        (assign34010_e39142, (((-locals.var_cnst0_dn0) * assign34010_e39141) + (assign34010_e39139 * (locals.var_t4_dn0 / (2.0 * assign34010_e39141)))), (((-locals.var_cnst0_dn2) * assign34010_e39141) + (assign34010_e39139 * (locals.var_t4_dn2 / (2.0 * assign34010_e39141)))), (((-locals.var_cnst0_dn4) * assign34010_e39141) + (assign34010_e39139 * (locals.var_t4_dn4 / (2.0 * assign34010_e39141)))), (((-locals.var_cnst0_dn5) * assign34010_e39141) + (assign34010_e39139 * (locals.var_t4_dn5 / (2.0 * assign34010_e39141)))), (((-locals.var_cnst0_dn6) * assign34010_e39141) + (assign34010_e39139 * (locals.var_t4_dn6 / (2.0 * assign34010_e39141)))), (((-locals.var_cnst0_dn7) * assign34010_e39141) + (assign34010_e39139 * (locals.var_t4_dn7 / (2.0 * assign34010_e39141)))), (((-locals.var_cnst0_dn8) * assign34010_e39141) + (assign34010_e39139 * (locals.var_t4_dn8 / (2.0 * assign34010_e39141)))), (((-locals.var_cnst0_dn9) * assign34010_e39141) + (assign34010_e39139 * (locals.var_t4_dn9 / (2.0 * assign34010_e39141)))), (((-locals.var_cnst0_dn10) * assign34010_e39141) + (assign34010_e39139 * (locals.var_t4_dn10 / (2.0 * assign34010_e39141)))), (((-locals.var_cnst0_dn13) * assign34010_e39141) + (assign34010_e39139 * (locals.var_t4_dn13 / (2.0 * assign34010_e39141)))),)
    } else {
        (locals.var_q_nl_cur, locals.var_q_nl_cur_dn0, locals.var_q_nl_cur_dn2, locals.var_q_nl_cur_dn4, locals.var_q_nl_cur_dn5, locals.var_q_nl_cur_dn6, locals.var_q_nl_cur_dn7, locals.var_q_nl_cur_dn8, locals.var_q_nl_cur_dn9, locals.var_q_nl_cur_dn10, locals.var_q_nl_cur_dn13,)
    }
};
        locals.var_q_nl_cur = assign34010_e39144;
        locals.var_q_nl_cur_dn0 = assign34010_e39144_d_n0;
        locals.var_q_nl_cur_dn2 = assign34010_e39144_d_n2;
        locals.var_q_nl_cur_dn4 = assign34010_e39144_d_n4;
        locals.var_q_nl_cur_dn5 = assign34010_e39144_d_n5;
        locals.var_q_nl_cur_dn6 = assign34010_e39144_d_n6;
        locals.var_q_nl_cur_dn7 = assign34010_e39144_d_n7;
        locals.var_q_nl_cur_dn8 = assign34010_e39144_d_n8;
        locals.var_q_nl_cur_dn9 = assign34010_e39144_d_n9;
        locals.var_q_nl_cur_dn10 = assign34010_e39144_d_n10;
        locals.var_q_nl_cur_dn13 = assign34010_e39144_d_n13;
        locals.var_q_nl_cur_rv = 0.0;

        let (assign34020_e39150, assign34020_e39150_d_n0, assign34020_e39150_d_n2, assign34020_e39150_d_n4, assign34020_e39150_d_n5, assign34020_e39150_d_n6, assign34020_e39150_d_n7, assign34020_e39150_d_n8, assign34020_e39150_d_n9, assign34020_e39150_d_n10, assign34020_e39150_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        (locals.var_phi_s0_dep, locals.var_phi_s0_dep_dn0, locals.var_phi_s0_dep_dn2, locals.var_phi_s0_dep_dn4, locals.var_phi_s0_dep_dn5, locals.var_phi_s0_dep_dn6, locals.var_phi_s0_dep_dn7, locals.var_phi_s0_dep_dn8, locals.var_phi_s0_dep_dn9, locals.var_phi_s0_dep_dn10, locals.var_phi_s0_dep_dn13,)
    } else {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn4, locals.var_ps0_dn5, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn8, locals.var_ps0_dn9, locals.var_ps0_dn10, locals.var_ps0_dn13,)
    }
};
        locals.var_ps0 = assign34020_e39150;
        locals.var_ps0_dn0 = assign34020_e39150_d_n0;
        locals.var_ps0_dn2 = assign34020_e39150_d_n2;
        locals.var_ps0_dn4 = assign34020_e39150_d_n4;
        locals.var_ps0_dn5 = assign34020_e39150_d_n5;
        locals.var_ps0_dn6 = assign34020_e39150_d_n6;
        locals.var_ps0_dn7 = assign34020_e39150_d_n7;
        locals.var_ps0_dn8 = assign34020_e39150_d_n8;
        locals.var_ps0_dn9 = assign34020_e39150_d_n9;
        locals.var_ps0_dn10 = assign34020_e39150_d_n10;
        locals.var_ps0_dn13 = assign34020_e39150_d_n13;
        locals.var_ps0_rv = 0.0;

        let (assign34030_e39156, assign34030_e39156_d_n0, assign34030_e39156_d_n2, assign34030_e39156_d_n4, assign34030_e39156_d_n5, assign34030_e39156_d_n6, assign34030_e39156_d_n7, assign34030_e39156_d_n8, assign34030_e39156_d_n9, assign34030_e39156_d_n10, assign34030_e39156_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        (locals.var_phi_sl_dep, locals.var_phi_sl_dep_dn0, locals.var_phi_sl_dep_dn2, locals.var_phi_sl_dep_dn4, locals.var_phi_sl_dep_dn5, locals.var_phi_sl_dep_dn6, locals.var_phi_sl_dep_dn7, locals.var_phi_sl_dep_dn8, locals.var_phi_sl_dep_dn9, locals.var_phi_sl_dep_dn10, locals.var_phi_sl_dep_dn13,)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn4, locals.var_psl_dn5, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn8, locals.var_psl_dn9, locals.var_psl_dn10, locals.var_psl_dn13,)
    }
};
        locals.var_psl = assign34030_e39156;
        locals.var_psl_dn0 = assign34030_e39156_d_n0;
        locals.var_psl_dn2 = assign34030_e39156_d_n2;
        locals.var_psl_dn4 = assign34030_e39156_d_n4;
        locals.var_psl_dn5 = assign34030_e39156_d_n5;
        locals.var_psl_dn6 = assign34030_e39156_d_n6;
        locals.var_psl_dn7 = assign34030_e39156_d_n7;
        locals.var_psl_dn8 = assign34030_e39156_d_n8;
        locals.var_psl_dn9 = assign34030_e39156_d_n9;
        locals.var_psl_dn10 = assign34030_e39156_d_n10;
        locals.var_psl_dn13 = assign34030_e39156_d_n13;
        locals.var_psl_rv = 0.0;

        let (assign34040_e39164, assign34040_e39164_d_n0, assign34040_e39164_d_n2, assign34040_e39164_d_n4, assign34040_e39164_d_n5, assign34040_e39164_d_n6, assign34040_e39164_d_n7, assign34040_e39164_d_n8, assign34040_e39164_d_n9, assign34040_e39164_d_n10, assign34040_e39164_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign34040_e39162: f64 = (locals.var_phi_sl_dep - locals.var_phi_s0_dep);
        (assign34040_e39162, (locals.var_phi_sl_dep_dn0 - locals.var_phi_s0_dep_dn0), (locals.var_phi_sl_dep_dn2 - locals.var_phi_s0_dep_dn2), (locals.var_phi_sl_dep_dn4 - locals.var_phi_s0_dep_dn4), (locals.var_phi_sl_dep_dn5 - locals.var_phi_s0_dep_dn5), (locals.var_phi_sl_dep_dn6 - locals.var_phi_s0_dep_dn6), (locals.var_phi_sl_dep_dn7 - locals.var_phi_s0_dep_dn7), (locals.var_phi_sl_dep_dn8 - locals.var_phi_s0_dep_dn8), (locals.var_phi_sl_dep_dn9 - locals.var_phi_s0_dep_dn9), (locals.var_phi_sl_dep_dn10 - locals.var_phi_s0_dep_dn10), (locals.var_phi_sl_dep_dn13 - locals.var_phi_s0_dep_dn13),)
    } else {
        (locals.var_pds, locals.var_pds_dn0, locals.var_pds_dn2, locals.var_pds_dn4, locals.var_pds_dn5, locals.var_pds_dn6, locals.var_pds_dn7, locals.var_pds_dn8, locals.var_pds_dn9, locals.var_pds_dn10, locals.var_pds_dn13,)
    }
};
        locals.var_pds = assign34040_e39164;
        locals.var_pds_dn0 = assign34040_e39164_d_n0;
        locals.var_pds_dn2 = assign34040_e39164_d_n2;
        locals.var_pds_dn4 = assign34040_e39164_d_n4;
        locals.var_pds_dn5 = assign34040_e39164_d_n5;
        locals.var_pds_dn6 = assign34040_e39164_d_n6;
        locals.var_pds_dn7 = assign34040_e39164_d_n7;
        locals.var_pds_dn8 = assign34040_e39164_d_n8;
        locals.var_pds_dn9 = assign34040_e39164_d_n9;
        locals.var_pds_dn10 = assign34040_e39164_d_n10;
        locals.var_pds_dn13 = assign34040_e39164_d_n13;
        locals.var_pds_rv = 0.0;

        let (assign34050_e39173, assign34050_e39173_d_n0, assign34050_e39173_d_n2, assign34050_e39173_d_n4, assign34050_e39173_d_n5, assign34050_e39173_d_n6, assign34050_e39173_d_n7, assign34050_e39173_d_n8, assign34050_e39173_d_n9, assign34050_e39173_d_n10, assign34050_e39173_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign34050_e39170: f64 = (locals.var_q_s0 + locals.var_q_sl);
        let assign34050_e39171: f64 = (-assign34050_e39170);
        (assign34050_e39171, (-(locals.var_q_s0_dn0 + locals.var_q_sl_dn0)), (-(locals.var_q_s0_dn2 + locals.var_q_sl_dn2)), (-(locals.var_q_s0_dn4 + locals.var_q_sl_dn4)), (-(locals.var_q_s0_dn5 + locals.var_q_sl_dn5)), (-(locals.var_q_s0_dn6 + locals.var_q_sl_dn6)), (-(locals.var_q_s0_dn7 + locals.var_q_sl_dn7)), (-(locals.var_q_s0_dn8 + locals.var_q_sl_dn8)), (-(locals.var_q_s0_dn9 + locals.var_q_sl_dn9)), (-(locals.var_q_s0_dn10 + locals.var_q_sl_dn10)), (-(locals.var_q_s0_dn13 + locals.var_q_sl_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign34050_e39173;
        locals.var_t1_dn0 = assign34050_e39173_d_n0;
        locals.var_t1_dn2 = assign34050_e39173_d_n2;
        locals.var_t1_dn4 = assign34050_e39173_d_n4;
        locals.var_t1_dn5 = assign34050_e39173_d_n5;
        locals.var_t1_dn6 = assign34050_e39173_d_n6;
        locals.var_t1_dn7 = assign34050_e39173_d_n7;
        locals.var_t1_dn8 = assign34050_e39173_d_n8;
        locals.var_t1_dn9 = assign34050_e39173_d_n9;
        locals.var_t1_dn10 = assign34050_e39173_d_n10;
        locals.var_t1_dn13 = assign34050_e39173_d_n13;
        locals.var_t1_rv = 0.0;

        let assign34060_e39177: f64 = locals.var_qn_delta;
        let assign34060_e39182: f64 = if ((locals.var_t1 < assign34060_e39177) && (locals.var_qn_delta >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard798 = assign34060_e39182;
        locals.var_guard798_rv = 0.0;

        let (assign34070_e39194, assign34070_e39194_d_n0, assign34070_e39194_d_n2, assign34070_e39194_d_n4, assign34070_e39194_d_n5, assign34070_e39194_d_n6, assign34070_e39194_d_n7, assign34070_e39194_d_n8, assign34070_e39194_d_n9, assign34070_e39194_d_n10, assign34070_e39194_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard798 != 0.0)) {
        let assign34070_e39190: f64 = locals.var_qn_delta;
        let assign34070_e39192: f64 = (assign34070_e39190 - locals.var_t1);
        (assign34070_e39192, (locals.var_qn_delta_dn0 - locals.var_t1_dn0), (locals.var_qn_delta_dn2 - locals.var_t1_dn2), (locals.var_qn_delta_dn4 - locals.var_t1_dn4), (locals.var_qn_delta_dn5 - locals.var_t1_dn5), (locals.var_qn_delta_dn6 - locals.var_t1_dn6), (locals.var_qn_delta_dn7 - locals.var_t1_dn7), (locals.var_qn_delta_dn8 - locals.var_t1_dn8), (locals.var_qn_delta_dn9 - locals.var_t1_dn9), (locals.var_qn_delta_dn10 - locals.var_t1_dn10), (locals.var_qn_delta_dn13 - locals.var_t1_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign34070_e39194;
        locals.var_tmf1_dn0 = assign34070_e39194_d_n0;
        locals.var_tmf1_dn2 = assign34070_e39194_d_n2;
        locals.var_tmf1_dn4 = assign34070_e39194_d_n4;
        locals.var_tmf1_dn5 = assign34070_e39194_d_n5;
        locals.var_tmf1_dn6 = assign34070_e39194_d_n6;
        locals.var_tmf1_dn7 = assign34070_e39194_d_n7;
        locals.var_tmf1_dn8 = assign34070_e39194_d_n8;
        locals.var_tmf1_dn9 = assign34070_e39194_d_n9;
        locals.var_tmf1_dn10 = assign34070_e39194_d_n10;
        locals.var_tmf1_dn13 = assign34070_e39194_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign34080_e39204, assign34080_e39204_d_n0, assign34080_e39204_d_n2, assign34080_e39204_d_n4, assign34080_e39204_d_n5, assign34080_e39204_d_n6, assign34080_e39204_d_n7, assign34080_e39204_d_n8, assign34080_e39204_d_n9, assign34080_e39204_d_n10, assign34080_e39204_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard798 != 0.0)) {
        let assign34080_e39202: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign34080_e39202, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign34080_e39204;
        locals.var_x2_dn0 = assign34080_e39204_d_n0;
        locals.var_x2_dn2 = assign34080_e39204_d_n2;
        locals.var_x2_dn4 = assign34080_e39204_d_n4;
        locals.var_x2_dn5 = assign34080_e39204_d_n5;
        locals.var_x2_dn6 = assign34080_e39204_d_n6;
        locals.var_x2_dn7 = assign34080_e39204_d_n7;
        locals.var_x2_dn8 = assign34080_e39204_d_n8;
        locals.var_x2_dn9 = assign34080_e39204_d_n9;
        locals.var_x2_dn10 = assign34080_e39204_d_n10;
        locals.var_x2_dn13 = assign34080_e39204_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign34090_e39214, assign34090_e39214_d_n0, assign34090_e39214_d_n2, assign34090_e39214_d_n4, assign34090_e39214_d_n5, assign34090_e39214_d_n6, assign34090_e39214_d_n7, assign34090_e39214_d_n8, assign34090_e39214_d_n9, assign34090_e39214_d_n10, assign34090_e39214_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard798 != 0.0)) {
        let assign34090_e39212: f64 = (locals.var_qn_delta * locals.var_qn_delta);
        (assign34090_e39212, ((locals.var_qn_delta_dn0 * locals.var_qn_delta) + (locals.var_qn_delta * locals.var_qn_delta_dn0)), ((locals.var_qn_delta_dn2 * locals.var_qn_delta) + (locals.var_qn_delta * locals.var_qn_delta_dn2)), ((locals.var_qn_delta_dn4 * locals.var_qn_delta) + (locals.var_qn_delta * locals.var_qn_delta_dn4)), ((locals.var_qn_delta_dn5 * locals.var_qn_delta) + (locals.var_qn_delta * locals.var_qn_delta_dn5)), ((locals.var_qn_delta_dn6 * locals.var_qn_delta) + (locals.var_qn_delta * locals.var_qn_delta_dn6)), ((locals.var_qn_delta_dn7 * locals.var_qn_delta) + (locals.var_qn_delta * locals.var_qn_delta_dn7)), ((locals.var_qn_delta_dn8 * locals.var_qn_delta) + (locals.var_qn_delta * locals.var_qn_delta_dn8)), ((locals.var_qn_delta_dn9 * locals.var_qn_delta) + (locals.var_qn_delta * locals.var_qn_delta_dn9)), ((locals.var_qn_delta_dn10 * locals.var_qn_delta) + (locals.var_qn_delta * locals.var_qn_delta_dn10)), ((locals.var_qn_delta_dn13 * locals.var_qn_delta) + (locals.var_qn_delta * locals.var_qn_delta_dn13)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign34090_e39214;
        locals.var_xmax2_dn0 = assign34090_e39214_d_n0;
        locals.var_xmax2_dn2 = assign34090_e39214_d_n2;
        locals.var_xmax2_dn4 = assign34090_e39214_d_n4;
        locals.var_xmax2_dn5 = assign34090_e39214_d_n5;
        locals.var_xmax2_dn6 = assign34090_e39214_d_n6;
        locals.var_xmax2_dn7 = assign34090_e39214_d_n7;
        locals.var_xmax2_dn8 = assign34090_e39214_d_n8;
        locals.var_xmax2_dn9 = assign34090_e39214_d_n9;
        locals.var_xmax2_dn10 = assign34090_e39214_d_n10;
        locals.var_xmax2_dn13 = assign34090_e39214_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign34100_e39222, assign34100_e39222_d_n0, assign34100_e39222_d_n2, assign34100_e39222_d_n4, assign34100_e39222_d_n5, assign34100_e39222_d_n6, assign34100_e39222_d_n7, assign34100_e39222_d_n8, assign34100_e39222_d_n9, assign34100_e39222_d_n10, assign34100_e39222_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard798 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign34100_e39222;
        locals.var_xp_dn0 = assign34100_e39222_d_n0;
        locals.var_xp_dn2 = assign34100_e39222_d_n2;
        locals.var_xp_dn4 = assign34100_e39222_d_n4;
        locals.var_xp_dn5 = assign34100_e39222_d_n5;
        locals.var_xp_dn6 = assign34100_e39222_d_n6;
        locals.var_xp_dn7 = assign34100_e39222_d_n7;
        locals.var_xp_dn8 = assign34100_e39222_d_n8;
        locals.var_xp_dn9 = assign34100_e39222_d_n9;
        locals.var_xp_dn10 = assign34100_e39222_d_n10;
        locals.var_xp_dn13 = assign34100_e39222_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign34110_e39230, assign34110_e39230_d_n0, assign34110_e39230_d_n2, assign34110_e39230_d_n4, assign34110_e39230_d_n5, assign34110_e39230_d_n6, assign34110_e39230_d_n7, assign34110_e39230_d_n8, assign34110_e39230_d_n9, assign34110_e39230_d_n10, assign34110_e39230_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard798 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign34110_e39230;
        locals.var_xmp_dn0 = assign34110_e39230_d_n0;
        locals.var_xmp_dn2 = assign34110_e39230_d_n2;
        locals.var_xmp_dn4 = assign34110_e39230_d_n4;
        locals.var_xmp_dn5 = assign34110_e39230_d_n5;
        locals.var_xmp_dn6 = assign34110_e39230_d_n6;
        locals.var_xmp_dn7 = assign34110_e39230_d_n7;
        locals.var_xmp_dn8 = assign34110_e39230_d_n8;
        locals.var_xmp_dn9 = assign34110_e39230_d_n9;
        locals.var_xmp_dn10 = assign34110_e39230_d_n10;
        locals.var_xmp_dn13 = assign34110_e39230_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign34120_e39238,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard798 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign34120_e39238;
        locals.var_m0_rv = 0.0;

        let (assign34130_e39246,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard798 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign34130_e39246;
        locals.var_mm_rv = 0.0;

        let (assign34140_e39254, assign34140_e39254_d_n0, assign34140_e39254_d_n2, assign34140_e39254_d_n4, assign34140_e39254_d_n5, assign34140_e39254_d_n6, assign34140_e39254_d_n7, assign34140_e39254_d_n8, assign34140_e39254_d_n9, assign34140_e39254_d_n10, assign34140_e39254_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard798 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign34140_e39254;
        locals.var_arg_dn0 = assign34140_e39254_d_n0;
        locals.var_arg_dn2 = assign34140_e39254_d_n2;
        locals.var_arg_dn4 = assign34140_e39254_d_n4;
        locals.var_arg_dn5 = assign34140_e39254_d_n5;
        locals.var_arg_dn6 = assign34140_e39254_d_n6;
        locals.var_arg_dn7 = assign34140_e39254_d_n7;
        locals.var_arg_dn8 = assign34140_e39254_d_n8;
        locals.var_arg_dn9 = assign34140_e39254_d_n9;
        locals.var_arg_dn10 = assign34140_e39254_d_n10;
        locals.var_arg_dn13 = assign34140_e39254_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign34150_e39262, assign34150_e39262_d_n0, assign34150_e39262_d_n2, assign34150_e39262_d_n4, assign34150_e39262_d_n5, assign34150_e39262_d_n6, assign34150_e39262_d_n7, assign34150_e39262_d_n8, assign34150_e39262_d_n9, assign34150_e39262_d_n10, assign34150_e39262_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard798 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign34150_e39262;
        locals.var_dnm_dn0 = assign34150_e39262_d_n0;
        locals.var_dnm_dn2 = assign34150_e39262_d_n2;
        locals.var_dnm_dn4 = assign34150_e39262_d_n4;
        locals.var_dnm_dn5 = assign34150_e39262_d_n5;
        locals.var_dnm_dn6 = assign34150_e39262_d_n6;
        locals.var_dnm_dn7 = assign34150_e39262_d_n7;
        locals.var_dnm_dn8 = assign34150_e39262_d_n8;
        locals.var_dnm_dn9 = assign34150_e39262_d_n9;
        locals.var_dnm_dn10 = assign34150_e39262_d_n10;
        locals.var_dnm_dn13 = assign34150_e39262_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign34160_e39272, assign34160_e39272_d_n0, assign34160_e39272_d_n2, assign34160_e39272_d_n4, assign34160_e39272_d_n5, assign34160_e39272_d_n6, assign34160_e39272_d_n7, assign34160_e39272_d_n8, assign34160_e39272_d_n9, assign34160_e39272_d_n10, assign34160_e39272_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard798 != 0.0)) {
        let assign34160_e39270: f64 = (locals.var_xp * locals.var_x2);
        (assign34160_e39270, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign34160_e39272;
        locals.var_xp_dn0 = assign34160_e39272_d_n0;
        locals.var_xp_dn2 = assign34160_e39272_d_n2;
        locals.var_xp_dn4 = assign34160_e39272_d_n4;
        locals.var_xp_dn5 = assign34160_e39272_d_n5;
        locals.var_xp_dn6 = assign34160_e39272_d_n6;
        locals.var_xp_dn7 = assign34160_e39272_d_n7;
        locals.var_xp_dn8 = assign34160_e39272_d_n8;
        locals.var_xp_dn9 = assign34160_e39272_d_n9;
        locals.var_xp_dn10 = assign34160_e39272_d_n10;
        locals.var_xp_dn13 = assign34160_e39272_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign34170_e39282, assign34170_e39282_d_n0, assign34170_e39282_d_n2, assign34170_e39282_d_n4, assign34170_e39282_d_n5, assign34170_e39282_d_n6, assign34170_e39282_d_n7, assign34170_e39282_d_n8, assign34170_e39282_d_n9, assign34170_e39282_d_n10, assign34170_e39282_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard798 != 0.0)) {
        let assign34170_e39280: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign34170_e39280, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign34170_e39282;
        locals.var_xmp_dn0 = assign34170_e39282_d_n0;
        locals.var_xmp_dn2 = assign34170_e39282_d_n2;
        locals.var_xmp_dn4 = assign34170_e39282_d_n4;
        locals.var_xmp_dn5 = assign34170_e39282_d_n5;
        locals.var_xmp_dn6 = assign34170_e39282_d_n6;
        locals.var_xmp_dn7 = assign34170_e39282_d_n7;
        locals.var_xmp_dn8 = assign34170_e39282_d_n8;
        locals.var_xmp_dn9 = assign34170_e39282_d_n9;
        locals.var_xmp_dn10 = assign34170_e39282_d_n10;
        locals.var_xmp_dn13 = assign34170_e39282_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign34180_e39292, assign34180_e39292_d_n0, assign34180_e39292_d_n2, assign34180_e39292_d_n4, assign34180_e39292_d_n5, assign34180_e39292_d_n6, assign34180_e39292_d_n7, assign34180_e39292_d_n8, assign34180_e39292_d_n9, assign34180_e39292_d_n10, assign34180_e39292_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard798 != 0.0)) {
        let assign34180_e39290: f64 = (locals.var_xp * locals.var_x2);
        (assign34180_e39290, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign34180_e39292;
        locals.var_xp_dn0 = assign34180_e39292_d_n0;
        locals.var_xp_dn2 = assign34180_e39292_d_n2;
        locals.var_xp_dn4 = assign34180_e39292_d_n4;
        locals.var_xp_dn5 = assign34180_e39292_d_n5;
        locals.var_xp_dn6 = assign34180_e39292_d_n6;
        locals.var_xp_dn7 = assign34180_e39292_d_n7;
        locals.var_xp_dn8 = assign34180_e39292_d_n8;
        locals.var_xp_dn9 = assign34180_e39292_d_n9;
        locals.var_xp_dn10 = assign34180_e39292_d_n10;
        locals.var_xp_dn13 = assign34180_e39292_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign34190_e39302, assign34190_e39302_d_n0, assign34190_e39302_d_n2, assign34190_e39302_d_n4, assign34190_e39302_d_n5, assign34190_e39302_d_n6, assign34190_e39302_d_n7, assign34190_e39302_d_n8, assign34190_e39302_d_n9, assign34190_e39302_d_n10, assign34190_e39302_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard798 != 0.0)) {
        let assign34190_e39300: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign34190_e39300, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign34190_e39302;
        locals.var_xmp_dn0 = assign34190_e39302_d_n0;
        locals.var_xmp_dn2 = assign34190_e39302_d_n2;
        locals.var_xmp_dn4 = assign34190_e39302_d_n4;
        locals.var_xmp_dn5 = assign34190_e39302_d_n5;
        locals.var_xmp_dn6 = assign34190_e39302_d_n6;
        locals.var_xmp_dn7 = assign34190_e39302_d_n7;
        locals.var_xmp_dn8 = assign34190_e39302_d_n8;
        locals.var_xmp_dn9 = assign34190_e39302_d_n9;
        locals.var_xmp_dn10 = assign34190_e39302_d_n10;
        locals.var_xmp_dn13 = assign34190_e39302_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign34200_e39312, assign34200_e39312_d_n0, assign34200_e39312_d_n2, assign34200_e39312_d_n4, assign34200_e39312_d_n5, assign34200_e39312_d_n6, assign34200_e39312_d_n7, assign34200_e39312_d_n8, assign34200_e39312_d_n9, assign34200_e39312_d_n10, assign34200_e39312_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard798 != 0.0)) {
        let assign34200_e39310: f64 = (locals.var_xp + locals.var_xmp);
        (assign34200_e39310, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign34200_e39312;
        locals.var_arg_dn0 = assign34200_e39312_d_n0;
        locals.var_arg_dn2 = assign34200_e39312_d_n2;
        locals.var_arg_dn4 = assign34200_e39312_d_n4;
        locals.var_arg_dn5 = assign34200_e39312_d_n5;
        locals.var_arg_dn6 = assign34200_e39312_d_n6;
        locals.var_arg_dn7 = assign34200_e39312_d_n7;
        locals.var_arg_dn8 = assign34200_e39312_d_n8;
        locals.var_arg_dn9 = assign34200_e39312_d_n9;
        locals.var_arg_dn10 = assign34200_e39312_d_n10;
        locals.var_arg_dn13 = assign34200_e39312_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign34210_e39320, assign34210_e39320_d_n0, assign34210_e39320_d_n2, assign34210_e39320_d_n4, assign34210_e39320_d_n5, assign34210_e39320_d_n6, assign34210_e39320_d_n7, assign34210_e39320_d_n8, assign34210_e39320_d_n9, assign34210_e39320_d_n10, assign34210_e39320_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard798 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign34210_e39320;
        locals.var_dnm_dn0 = assign34210_e39320_d_n0;
        locals.var_dnm_dn2 = assign34210_e39320_d_n2;
        locals.var_dnm_dn4 = assign34210_e39320_d_n4;
        locals.var_dnm_dn5 = assign34210_e39320_d_n5;
        locals.var_dnm_dn6 = assign34210_e39320_d_n6;
        locals.var_dnm_dn7 = assign34210_e39320_d_n7;
        locals.var_dnm_dn8 = assign34210_e39320_d_n8;
        locals.var_dnm_dn9 = assign34210_e39320_d_n9;
        locals.var_dnm_dn10 = assign34210_e39320_d_n10;
        locals.var_dnm_dn13 = assign34210_e39320_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign34220_e39335: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard799 = assign34220_e39335;
        locals.var_guard799_rv = 0.0;

        let assign34230_e39338: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard800 = assign34230_e39338;
        locals.var_guard800_rv = 0.0;

        let (assign34240_e39350,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard798 != 0.0)) && (locals.var_guard799 != 0.0)) && (locals.var_guard800 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign34240_e39350;
        locals.var_mm_rv = 0.0;

        let assign34250_e39353: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard801 = assign34250_e39353;
        locals.var_guard801_rv = 0.0;

        let (assign34260_e39368,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard798 != 0.0)) && (locals.var_guard799 != 0.0)) && (locals.var_guard800 == 0.0)) && (locals.var_guard801 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign34260_e39368;
        locals.var_mm_rv = 0.0;

        let assign34270_e39371: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard802 = assign34270_e39371;
        locals.var_guard802_rv = 0.0;

        let (assign34280_e39389,) = {
    if (((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard798 != 0.0)) && (locals.var_guard799 != 0.0)) && (locals.var_guard800 == 0.0)) && (locals.var_guard801 == 0.0)) && (locals.var_guard802 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign34280_e39389;
        locals.var_mm_rv = 0.0;

        let assign34290_e39392: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard803 = assign34290_e39392;
        locals.var_guard803_rv = 0.0;

        let (assign34300_e39413,) = {
    if ((((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard798 != 0.0)) && (locals.var_guard799 != 0.0)) && (locals.var_guard800 == 0.0)) && (locals.var_guard801 == 0.0)) && (locals.var_guard802 == 0.0)) && (locals.var_guard803 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign34300_e39413;
        locals.var_mm_rv = 0.0;

        let (assign34310_e39423,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard798 != 0.0)) && (locals.var_guard799 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign34310_e39423;
        locals.var_m0_rv = 0.0;

        let mut assign34320_loop_guard: usize = 0;
        while {
            let assign34320_cond_e39434: f64 = if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard798 != 0.0)) && (locals.var_guard799 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign34320_cond_e39434 != 0.0
        } {
            assign34320_loop_guard += 1;
            assert!(assign34320_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign34320_body0_e39445, assign34320_body0_e39445_d_n0, assign34320_body0_e39445_d_n2, assign34320_body0_e39445_d_n4, assign34320_body0_e39445_d_n5, assign34320_body0_e39445_d_n6, assign34320_body0_e39445_d_n7, assign34320_body0_e39445_d_n8, assign34320_body0_e39445_d_n9, assign34320_body0_e39445_d_n10, assign34320_body0_e39445_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard798 != 0.0)) && (locals.var_guard799 != 0.0)) {
        let assign34320_body0_e39443: f64 = (locals.var_dnm).sqrt();
        (assign34320_body0_e39443, (locals.var_dnm_dn0 / (2.0 * assign34320_body0_e39443)), (locals.var_dnm_dn2 / (2.0 * assign34320_body0_e39443)), (locals.var_dnm_dn4 / (2.0 * assign34320_body0_e39443)), (locals.var_dnm_dn5 / (2.0 * assign34320_body0_e39443)), (locals.var_dnm_dn6 / (2.0 * assign34320_body0_e39443)), (locals.var_dnm_dn7 / (2.0 * assign34320_body0_e39443)), (locals.var_dnm_dn8 / (2.0 * assign34320_body0_e39443)), (locals.var_dnm_dn9 / (2.0 * assign34320_body0_e39443)), (locals.var_dnm_dn10 / (2.0 * assign34320_body0_e39443)), (locals.var_dnm_dn13 / (2.0 * assign34320_body0_e39443)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign34320_body0_e39445;
            locals.var_dnm_dn0 = assign34320_body0_e39445_d_n0;
            locals.var_dnm_dn2 = assign34320_body0_e39445_d_n2;
            locals.var_dnm_dn4 = assign34320_body0_e39445_d_n4;
            locals.var_dnm_dn5 = assign34320_body0_e39445_d_n5;
            locals.var_dnm_dn6 = assign34320_body0_e39445_d_n6;
            locals.var_dnm_dn7 = assign34320_body0_e39445_d_n7;
            locals.var_dnm_dn8 = assign34320_body0_e39445_d_n8;
            locals.var_dnm_dn9 = assign34320_body0_e39445_d_n9;
            locals.var_dnm_dn10 = assign34320_body0_e39445_d_n10;
            locals.var_dnm_dn13 = assign34320_body0_e39445_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign34320_body1_e39457,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard798 != 0.0)) && (locals.var_guard799 != 0.0)) {
        let assign34320_body1_e39455: f64 = (locals.var_m0 + 1.0);
        (assign34320_body1_e39455,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign34320_body1_e39457;
            locals.var_m0_rv = 0.0;
        }

    }

    pub(super) fn stamp_reactive_block_107(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign34330_e39479, assign34330_e39479_d_n0, assign34330_e39479_d_n2, assign34330_e39479_d_n4, assign34330_e39479_d_n5, assign34330_e39479_d_n6, assign34330_e39479_d_n7, assign34330_e39479_d_n8, assign34330_e39479_d_n9, assign34330_e39479_d_n10, assign34330_e39479_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard798 != 0.0)) && (locals.var_guard799 == 0.0)) {
        let (assign34330_e39477, assign34330_e39477_d_n0, assign34330_e39477_d_n2, assign34330_e39477_d_n4, assign34330_e39477_d_n5, assign34330_e39477_d_n6, assign34330_e39477_d_n7, assign34330_e39477_d_n8, assign34330_e39477_d_n9, assign34330_e39477_d_n10, assign34330_e39477_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign34330_e39474: f64 = (2.0 * 2.0);
                let assign34330_e39475: f64 = (1.0 / assign34330_e39474);
                let assign34330_e39476: f64 = (locals.var_dnm).powf(assign34330_e39475);
                (assign34330_e39476, if 0.0 == 0.0 && ((assign34330_e39475) as f64).is_finite() && ((assign34330_e39475) as f64).fract() == 0.0 { if assign34330_e39475 == 0.0 { 0.0 } else { (assign34330_e39475 * ((locals.var_dnm).powf(assign34330_e39475 - 1.0) * locals.var_dnm_dn0)) } } else { (assign34330_e39476 * (assign34330_e39475 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign34330_e39475) as f64).is_finite() && ((assign34330_e39475) as f64).fract() == 0.0 { if assign34330_e39475 == 0.0 { 0.0 } else { (assign34330_e39475 * ((locals.var_dnm).powf(assign34330_e39475 - 1.0) * locals.var_dnm_dn2)) } } else { (assign34330_e39476 * (assign34330_e39475 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign34330_e39475) as f64).is_finite() && ((assign34330_e39475) as f64).fract() == 0.0 { if assign34330_e39475 == 0.0 { 0.0 } else { (assign34330_e39475 * ((locals.var_dnm).powf(assign34330_e39475 - 1.0) * locals.var_dnm_dn4)) } } else { (assign34330_e39476 * (assign34330_e39475 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign34330_e39475) as f64).is_finite() && ((assign34330_e39475) as f64).fract() == 0.0 { if assign34330_e39475 == 0.0 { 0.0 } else { (assign34330_e39475 * ((locals.var_dnm).powf(assign34330_e39475 - 1.0) * locals.var_dnm_dn5)) } } else { (assign34330_e39476 * (assign34330_e39475 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign34330_e39475) as f64).is_finite() && ((assign34330_e39475) as f64).fract() == 0.0 { if assign34330_e39475 == 0.0 { 0.0 } else { (assign34330_e39475 * ((locals.var_dnm).powf(assign34330_e39475 - 1.0) * locals.var_dnm_dn6)) } } else { (assign34330_e39476 * (assign34330_e39475 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign34330_e39475) as f64).is_finite() && ((assign34330_e39475) as f64).fract() == 0.0 { if assign34330_e39475 == 0.0 { 0.0 } else { (assign34330_e39475 * ((locals.var_dnm).powf(assign34330_e39475 - 1.0) * locals.var_dnm_dn7)) } } else { (assign34330_e39476 * (assign34330_e39475 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign34330_e39475) as f64).is_finite() && ((assign34330_e39475) as f64).fract() == 0.0 { if assign34330_e39475 == 0.0 { 0.0 } else { (assign34330_e39475 * ((locals.var_dnm).powf(assign34330_e39475 - 1.0) * locals.var_dnm_dn8)) } } else { (assign34330_e39476 * (assign34330_e39475 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign34330_e39475) as f64).is_finite() && ((assign34330_e39475) as f64).fract() == 0.0 { if assign34330_e39475 == 0.0 { 0.0 } else { (assign34330_e39475 * ((locals.var_dnm).powf(assign34330_e39475 - 1.0) * locals.var_dnm_dn9)) } } else { (assign34330_e39476 * (assign34330_e39475 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign34330_e39475) as f64).is_finite() && ((assign34330_e39475) as f64).fract() == 0.0 { if assign34330_e39475 == 0.0 { 0.0 } else { (assign34330_e39475 * ((locals.var_dnm).powf(assign34330_e39475 - 1.0) * locals.var_dnm_dn10)) } } else { (assign34330_e39476 * (assign34330_e39475 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign34330_e39475) as f64).is_finite() && ((assign34330_e39475) as f64).fract() == 0.0 { if assign34330_e39475 == 0.0 { 0.0 } else { (assign34330_e39475 * ((locals.var_dnm).powf(assign34330_e39475 - 1.0) * locals.var_dnm_dn13)) } } else { (assign34330_e39476 * (assign34330_e39475 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign34330_e39477, assign34330_e39477_d_n0, assign34330_e39477_d_n2, assign34330_e39477_d_n4, assign34330_e39477_d_n5, assign34330_e39477_d_n6, assign34330_e39477_d_n7, assign34330_e39477_d_n8, assign34330_e39477_d_n9, assign34330_e39477_d_n10, assign34330_e39477_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign34330_e39479;
        locals.var_dnm_dn0 = assign34330_e39479_d_n0;
        locals.var_dnm_dn2 = assign34330_e39479_d_n2;
        locals.var_dnm_dn4 = assign34330_e39479_d_n4;
        locals.var_dnm_dn5 = assign34330_e39479_d_n5;
        locals.var_dnm_dn6 = assign34330_e39479_d_n6;
        locals.var_dnm_dn7 = assign34330_e39479_d_n7;
        locals.var_dnm_dn8 = assign34330_e39479_d_n8;
        locals.var_dnm_dn9 = assign34330_e39479_d_n9;
        locals.var_dnm_dn10 = assign34330_e39479_d_n10;
        locals.var_dnm_dn13 = assign34330_e39479_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign34340_e39489, assign34340_e39489_d_n0, assign34340_e39489_d_n2, assign34340_e39489_d_n4, assign34340_e39489_d_n5, assign34340_e39489_d_n6, assign34340_e39489_d_n7, assign34340_e39489_d_n8, assign34340_e39489_d_n9, assign34340_e39489_d_n10, assign34340_e39489_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard798 != 0.0)) {
        let assign34340_e39487: f64 = (1.0 / locals.var_dnm);
        (assign34340_e39487, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign34340_e39489;
        locals.var_dnm_dn0 = assign34340_e39489_d_n0;
        locals.var_dnm_dn2 = assign34340_e39489_d_n2;
        locals.var_dnm_dn4 = assign34340_e39489_d_n4;
        locals.var_dnm_dn5 = assign34340_e39489_d_n5;
        locals.var_dnm_dn6 = assign34340_e39489_d_n6;
        locals.var_dnm_dn7 = assign34340_e39489_d_n7;
        locals.var_dnm_dn8 = assign34340_e39489_d_n8;
        locals.var_dnm_dn9 = assign34340_e39489_d_n9;
        locals.var_dnm_dn10 = assign34340_e39489_d_n10;
        locals.var_dnm_dn13 = assign34340_e39489_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign34350_e39501, assign34350_e39501_d_n0, assign34350_e39501_d_n2, assign34350_e39501_d_n4, assign34350_e39501_d_n5, assign34350_e39501_d_n6, assign34350_e39501_d_n7, assign34350_e39501_d_n8, assign34350_e39501_d_n9, assign34350_e39501_d_n10, assign34350_e39501_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard798 != 0.0)) {
        let assign34350_e39497: f64 = (locals.var_tmf1 * locals.var_qn_delta);
        let assign34350_e39499: f64 = (assign34350_e39497 * locals.var_dnm);
        (assign34350_e39499, ((((locals.var_tmf1_dn0 * locals.var_qn_delta) + (locals.var_tmf1 * locals.var_qn_delta_dn0)) * locals.var_dnm) + (assign34350_e39497 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_qn_delta) + (locals.var_tmf1 * locals.var_qn_delta_dn2)) * locals.var_dnm) + (assign34350_e39497 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * locals.var_qn_delta) + (locals.var_tmf1 * locals.var_qn_delta_dn4)) * locals.var_dnm) + (assign34350_e39497 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * locals.var_qn_delta) + (locals.var_tmf1 * locals.var_qn_delta_dn5)) * locals.var_dnm) + (assign34350_e39497 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * locals.var_qn_delta) + (locals.var_tmf1 * locals.var_qn_delta_dn6)) * locals.var_dnm) + (assign34350_e39497 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_qn_delta) + (locals.var_tmf1 * locals.var_qn_delta_dn7)) * locals.var_dnm) + (assign34350_e39497 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * locals.var_qn_delta) + (locals.var_tmf1 * locals.var_qn_delta_dn8)) * locals.var_dnm) + (assign34350_e39497 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * locals.var_qn_delta) + (locals.var_tmf1 * locals.var_qn_delta_dn9)) * locals.var_dnm) + (assign34350_e39497 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * locals.var_qn_delta) + (locals.var_tmf1 * locals.var_qn_delta_dn10)) * locals.var_dnm) + (assign34350_e39497 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn13 * locals.var_qn_delta) + (locals.var_tmf1 * locals.var_qn_delta_dn13)) * locals.var_dnm) + (assign34350_e39497 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign34350_e39501;
        locals.var_tmf0_dn0 = assign34350_e39501_d_n0;
        locals.var_tmf0_dn2 = assign34350_e39501_d_n2;
        locals.var_tmf0_dn4 = assign34350_e39501_d_n4;
        locals.var_tmf0_dn5 = assign34350_e39501_d_n5;
        locals.var_tmf0_dn6 = assign34350_e39501_d_n6;
        locals.var_tmf0_dn7 = assign34350_e39501_d_n7;
        locals.var_tmf0_dn8 = assign34350_e39501_d_n8;
        locals.var_tmf0_dn9 = assign34350_e39501_d_n9;
        locals.var_tmf0_dn10 = assign34350_e39501_d_n10;
        locals.var_tmf0_dn13 = assign34350_e39501_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign34360_e39515, assign34360_e39515_d_n0, assign34360_e39515_d_n2, assign34360_e39515_d_n4, assign34360_e39515_d_n5, assign34360_e39515_d_n6, assign34360_e39515_d_n7, assign34360_e39515_d_n8, assign34360_e39515_d_n9, assign34360_e39515_d_n10, assign34360_e39515_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard798 != 0.0)) {
        let assign34360_e39509: f64 = (locals.var_qn_delta * locals.var_xmp);
        let assign34360_e39511: f64 = (assign34360_e39509 * locals.var_dnm);
        let assign34360_e39513: f64 = (assign34360_e39511 / locals.var_arg);
        (assign34360_e39513, (((((((locals.var_qn_delta_dn0 * locals.var_xmp) + (locals.var_qn_delta * locals.var_xmp_dn0)) * locals.var_dnm) + (assign34360_e39509 * locals.var_dnm_dn0)) * locals.var_arg) - (assign34360_e39511 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_qn_delta_dn2 * locals.var_xmp) + (locals.var_qn_delta * locals.var_xmp_dn2)) * locals.var_dnm) + (assign34360_e39509 * locals.var_dnm_dn2)) * locals.var_arg) - (assign34360_e39511 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_qn_delta_dn4 * locals.var_xmp) + (locals.var_qn_delta * locals.var_xmp_dn4)) * locals.var_dnm) + (assign34360_e39509 * locals.var_dnm_dn4)) * locals.var_arg) - (assign34360_e39511 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_qn_delta_dn5 * locals.var_xmp) + (locals.var_qn_delta * locals.var_xmp_dn5)) * locals.var_dnm) + (assign34360_e39509 * locals.var_dnm_dn5)) * locals.var_arg) - (assign34360_e39511 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_qn_delta_dn6 * locals.var_xmp) + (locals.var_qn_delta * locals.var_xmp_dn6)) * locals.var_dnm) + (assign34360_e39509 * locals.var_dnm_dn6)) * locals.var_arg) - (assign34360_e39511 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_qn_delta_dn7 * locals.var_xmp) + (locals.var_qn_delta * locals.var_xmp_dn7)) * locals.var_dnm) + (assign34360_e39509 * locals.var_dnm_dn7)) * locals.var_arg) - (assign34360_e39511 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_qn_delta_dn8 * locals.var_xmp) + (locals.var_qn_delta * locals.var_xmp_dn8)) * locals.var_dnm) + (assign34360_e39509 * locals.var_dnm_dn8)) * locals.var_arg) - (assign34360_e39511 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_qn_delta_dn9 * locals.var_xmp) + (locals.var_qn_delta * locals.var_xmp_dn9)) * locals.var_dnm) + (assign34360_e39509 * locals.var_dnm_dn9)) * locals.var_arg) - (assign34360_e39511 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_qn_delta_dn10 * locals.var_xmp) + (locals.var_qn_delta * locals.var_xmp_dn10)) * locals.var_dnm) + (assign34360_e39509 * locals.var_dnm_dn10)) * locals.var_arg) - (assign34360_e39511 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_qn_delta_dn13 * locals.var_xmp) + (locals.var_qn_delta * locals.var_xmp_dn13)) * locals.var_dnm) + (assign34360_e39509 * locals.var_dnm_dn13)) * locals.var_arg) - (assign34360_e39511 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign34360_e39515;
        locals.var_t0_dn0 = assign34360_e39515_d_n0;
        locals.var_t0_dn2 = assign34360_e39515_d_n2;
        locals.var_t0_dn4 = assign34360_e39515_d_n4;
        locals.var_t0_dn5 = assign34360_e39515_d_n5;
        locals.var_t0_dn6 = assign34360_e39515_d_n6;
        locals.var_t0_dn7 = assign34360_e39515_d_n7;
        locals.var_t0_dn8 = assign34360_e39515_d_n8;
        locals.var_t0_dn9 = assign34360_e39515_d_n9;
        locals.var_t0_dn10 = assign34360_e39515_d_n10;
        locals.var_t0_dn13 = assign34360_e39515_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign34370_e39527, assign34370_e39527_d_n0, assign34370_e39527_d_n2, assign34370_e39527_d_n4, assign34370_e39527_d_n5, assign34370_e39527_d_n6, assign34370_e39527_d_n7, assign34370_e39527_d_n8, assign34370_e39527_d_n9, assign34370_e39527_d_n10, assign34370_e39527_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard798 != 0.0)) {
        let assign34370_e39523: f64 = locals.var_qn_delta;
        let assign34370_e39525: f64 = (assign34370_e39523 - locals.var_tmf0);
        (assign34370_e39525, (locals.var_qn_delta_dn0 - locals.var_tmf0_dn0), (locals.var_qn_delta_dn2 - locals.var_tmf0_dn2), (locals.var_qn_delta_dn4 - locals.var_tmf0_dn4), (locals.var_qn_delta_dn5 - locals.var_tmf0_dn5), (locals.var_qn_delta_dn6 - locals.var_tmf0_dn6), (locals.var_qn_delta_dn7 - locals.var_tmf0_dn7), (locals.var_qn_delta_dn8 - locals.var_tmf0_dn8), (locals.var_qn_delta_dn9 - locals.var_tmf0_dn9), (locals.var_qn_delta_dn10 - locals.var_tmf0_dn10), (locals.var_qn_delta_dn13 - locals.var_tmf0_dn13),)
    } else {
        (locals.var_qn_drift, locals.var_qn_drift_dn0, locals.var_qn_drift_dn2, locals.var_qn_drift_dn4, locals.var_qn_drift_dn5, locals.var_qn_drift_dn6, locals.var_qn_drift_dn7, locals.var_qn_drift_dn8, locals.var_qn_drift_dn9, locals.var_qn_drift_dn10, locals.var_qn_drift_dn13,)
    }
};
        locals.var_qn_drift = assign34370_e39527;
        locals.var_qn_drift_dn0 = assign34370_e39527_d_n0;
        locals.var_qn_drift_dn2 = assign34370_e39527_d_n2;
        locals.var_qn_drift_dn4 = assign34370_e39527_d_n4;
        locals.var_qn_drift_dn5 = assign34370_e39527_d_n5;
        locals.var_qn_drift_dn6 = assign34370_e39527_d_n6;
        locals.var_qn_drift_dn7 = assign34370_e39527_d_n7;
        locals.var_qn_drift_dn8 = assign34370_e39527_d_n8;
        locals.var_qn_drift_dn9 = assign34370_e39527_d_n9;
        locals.var_qn_drift_dn10 = assign34370_e39527_d_n10;
        locals.var_qn_drift_dn13 = assign34370_e39527_d_n13;
        locals.var_qn_drift_rv = 0.0;

        let (assign34380_e39535, assign34380_e39535_d_n0, assign34380_e39535_d_n2, assign34380_e39535_d_n4, assign34380_e39535_d_n5, assign34380_e39535_d_n6, assign34380_e39535_d_n7, assign34380_e39535_d_n8, assign34380_e39535_d_n9, assign34380_e39535_d_n10, assign34380_e39535_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard798 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign34380_e39535;
        locals.var_t0_dn0 = assign34380_e39535_d_n0;
        locals.var_t0_dn2 = assign34380_e39535_d_n2;
        locals.var_t0_dn4 = assign34380_e39535_d_n4;
        locals.var_t0_dn5 = assign34380_e39535_d_n5;
        locals.var_t0_dn6 = assign34380_e39535_d_n6;
        locals.var_t0_dn7 = assign34380_e39535_d_n7;
        locals.var_t0_dn8 = assign34380_e39535_d_n8;
        locals.var_t0_dn9 = assign34380_e39535_d_n9;
        locals.var_t0_dn10 = assign34380_e39535_d_n10;
        locals.var_t0_dn13 = assign34380_e39535_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign34390_e39544, assign34390_e39544_d_n0, assign34390_e39544_d_n2, assign34390_e39544_d_n4, assign34390_e39544_d_n5, assign34390_e39544_d_n6, assign34390_e39544_d_n7, assign34390_e39544_d_n8, assign34390_e39544_d_n9, assign34390_e39544_d_n10, assign34390_e39544_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard798 == 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    } else {
        (locals.var_qn_drift, locals.var_qn_drift_dn0, locals.var_qn_drift_dn2, locals.var_qn_drift_dn4, locals.var_qn_drift_dn5, locals.var_qn_drift_dn6, locals.var_qn_drift_dn7, locals.var_qn_drift_dn8, locals.var_qn_drift_dn9, locals.var_qn_drift_dn10, locals.var_qn_drift_dn13,)
    }
};
        locals.var_qn_drift = assign34390_e39544;
        locals.var_qn_drift_dn0 = assign34390_e39544_d_n0;
        locals.var_qn_drift_dn2 = assign34390_e39544_d_n2;
        locals.var_qn_drift_dn4 = assign34390_e39544_d_n4;
        locals.var_qn_drift_dn5 = assign34390_e39544_d_n5;
        locals.var_qn_drift_dn6 = assign34390_e39544_d_n6;
        locals.var_qn_drift_dn7 = assign34390_e39544_d_n7;
        locals.var_qn_drift_dn8 = assign34390_e39544_d_n8;
        locals.var_qn_drift_dn9 = assign34390_e39544_d_n9;
        locals.var_qn_drift_dn10 = assign34390_e39544_d_n10;
        locals.var_qn_drift_dn13 = assign34390_e39544_d_n13;
        locals.var_qn_drift_rv = 0.0;

        let (assign34400_e39553, assign34400_e39553_d_n0, assign34400_e39553_d_n2, assign34400_e39553_d_n4, assign34400_e39553_d_n5, assign34400_e39553_d_n6, assign34400_e39553_d_n7, assign34400_e39553_d_n8, assign34400_e39553_d_n9, assign34400_e39553_d_n10, assign34400_e39553_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard798 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign34400_e39553;
        locals.var_t0_dn0 = assign34400_e39553_d_n0;
        locals.var_t0_dn2 = assign34400_e39553_d_n2;
        locals.var_t0_dn4 = assign34400_e39553_d_n4;
        locals.var_t0_dn5 = assign34400_e39553_d_n5;
        locals.var_t0_dn6 = assign34400_e39553_d_n6;
        locals.var_t0_dn7 = assign34400_e39553_d_n7;
        locals.var_t0_dn8 = assign34400_e39553_d_n8;
        locals.var_t0_dn9 = assign34400_e39553_d_n9;
        locals.var_t0_dn10 = assign34400_e39553_d_n10;
        locals.var_t0_dn13 = assign34400_e39553_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign34410_e39565, assign34410_e39565_d_n0, assign34410_e39565_d_n2, assign34410_e39565_d_n4, assign34410_e39565_d_n5, assign34410_e39565_d_n6, assign34410_e39565_d_n7, assign34410_e39565_d_n8, assign34410_e39565_d_n9, assign34410_e39565_d_n10, assign34410_e39565_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign34410_e39559: f64 = (locals.var_beta * locals.var_qn_drift);
        let assign34410_e39561: f64 = (assign34410_e39559 / 2.0);
        let assign34410_e39563: f64 = (assign34410_e39561 * locals.var_pds);
        (assign34410_e39563, (((((locals.var_beta_dn0 * locals.var_qn_drift) + (locals.var_beta * locals.var_qn_drift_dn0)) / 2.0) * locals.var_pds) + (assign34410_e39561 * locals.var_pds_dn0)), (((((locals.var_beta_dn2 * locals.var_qn_drift) + (locals.var_beta * locals.var_qn_drift_dn2)) / 2.0) * locals.var_pds) + (assign34410_e39561 * locals.var_pds_dn2)), (((((locals.var_beta_dn4 * locals.var_qn_drift) + (locals.var_beta * locals.var_qn_drift_dn4)) / 2.0) * locals.var_pds) + (assign34410_e39561 * locals.var_pds_dn4)), (((((locals.var_beta_dn5 * locals.var_qn_drift) + (locals.var_beta * locals.var_qn_drift_dn5)) / 2.0) * locals.var_pds) + (assign34410_e39561 * locals.var_pds_dn5)), (((((locals.var_beta_dn6 * locals.var_qn_drift) + (locals.var_beta * locals.var_qn_drift_dn6)) / 2.0) * locals.var_pds) + (assign34410_e39561 * locals.var_pds_dn6)), (((((locals.var_beta_dn7 * locals.var_qn_drift) + (locals.var_beta * locals.var_qn_drift_dn7)) / 2.0) * locals.var_pds) + (assign34410_e39561 * locals.var_pds_dn7)), (((((locals.var_beta_dn8 * locals.var_qn_drift) + (locals.var_beta * locals.var_qn_drift_dn8)) / 2.0) * locals.var_pds) + (assign34410_e39561 * locals.var_pds_dn8)), (((((locals.var_beta_dn9 * locals.var_qn_drift) + (locals.var_beta * locals.var_qn_drift_dn9)) / 2.0) * locals.var_pds) + (assign34410_e39561 * locals.var_pds_dn9)), (((((locals.var_beta_dn10 * locals.var_qn_drift) + (locals.var_beta * locals.var_qn_drift_dn10)) / 2.0) * locals.var_pds) + (assign34410_e39561 * locals.var_pds_dn10)), (((((locals.var_beta_dn13 * locals.var_qn_drift) + (locals.var_beta * locals.var_qn_drift_dn13)) / 2.0) * locals.var_pds) + (assign34410_e39561 * locals.var_pds_dn13)),)
    } else {
        (locals.var_idd_drift, locals.var_idd_drift_dn0, locals.var_idd_drift_dn2, locals.var_idd_drift_dn4, locals.var_idd_drift_dn5, locals.var_idd_drift_dn6, locals.var_idd_drift_dn7, locals.var_idd_drift_dn8, locals.var_idd_drift_dn9, locals.var_idd_drift_dn10, locals.var_idd_drift_dn13,)
    }
};
        locals.var_idd_drift = assign34410_e39565;
        locals.var_idd_drift_dn0 = assign34410_e39565_d_n0;
        locals.var_idd_drift_dn2 = assign34410_e39565_d_n2;
        locals.var_idd_drift_dn4 = assign34410_e39565_d_n4;
        locals.var_idd_drift_dn5 = assign34410_e39565_d_n5;
        locals.var_idd_drift_dn6 = assign34410_e39565_d_n6;
        locals.var_idd_drift_dn7 = assign34410_e39565_d_n7;
        locals.var_idd_drift_dn8 = assign34410_e39565_d_n8;
        locals.var_idd_drift_dn9 = assign34410_e39565_d_n9;
        locals.var_idd_drift_dn10 = assign34410_e39565_d_n10;
        locals.var_idd_drift_dn13 = assign34410_e39565_d_n13;
        locals.var_idd_drift_rv = 0.0;

        let (assign34420_e39575, assign34420_e39575_d_n0, assign34420_e39575_d_n2, assign34420_e39575_d_n4, assign34420_e39575_d_n5, assign34420_e39575_d_n6, assign34420_e39575_d_n7, assign34420_e39575_d_n8, assign34420_e39575_d_n9, assign34420_e39575_d_n10, assign34420_e39575_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign34420_e39570: f64 = (-locals.var_q_nl_cur);
        let assign34420_e39572: f64 = (assign34420_e39570 + locals.var_q_n0_cur);
        let assign34420_e39573: f64 = (-assign34420_e39572);
        (assign34420_e39573, (-((-locals.var_q_nl_cur_dn0) + locals.var_q_n0_cur_dn0)), (-((-locals.var_q_nl_cur_dn2) + locals.var_q_n0_cur_dn2)), (-((-locals.var_q_nl_cur_dn4) + locals.var_q_n0_cur_dn4)), (-((-locals.var_q_nl_cur_dn5) + locals.var_q_n0_cur_dn5)), (-((-locals.var_q_nl_cur_dn6) + locals.var_q_n0_cur_dn6)), (-((-locals.var_q_nl_cur_dn7) + locals.var_q_n0_cur_dn7)), (-((-locals.var_q_nl_cur_dn8) + locals.var_q_n0_cur_dn8)), (-((-locals.var_q_nl_cur_dn9) + locals.var_q_n0_cur_dn9)), (-((-locals.var_q_nl_cur_dn10) + locals.var_q_n0_cur_dn10)), (-((-locals.var_q_nl_cur_dn13) + locals.var_q_n0_cur_dn13)),)
    } else {
        (locals.var_idd_diffu, locals.var_idd_diffu_dn0, locals.var_idd_diffu_dn2, locals.var_idd_diffu_dn4, locals.var_idd_diffu_dn5, locals.var_idd_diffu_dn6, locals.var_idd_diffu_dn7, locals.var_idd_diffu_dn8, locals.var_idd_diffu_dn9, locals.var_idd_diffu_dn10, locals.var_idd_diffu_dn13,)
    }
};
        locals.var_idd_diffu = assign34420_e39575;
        locals.var_idd_diffu_dn0 = assign34420_e39575_d_n0;
        locals.var_idd_diffu_dn2 = assign34420_e39575_d_n2;
        locals.var_idd_diffu_dn4 = assign34420_e39575_d_n4;
        locals.var_idd_diffu_dn5 = assign34420_e39575_d_n5;
        locals.var_idd_diffu_dn6 = assign34420_e39575_d_n6;
        locals.var_idd_diffu_dn7 = assign34420_e39575_d_n7;
        locals.var_idd_diffu_dn8 = assign34420_e39575_d_n8;
        locals.var_idd_diffu_dn9 = assign34420_e39575_d_n9;
        locals.var_idd_diffu_dn10 = assign34420_e39575_d_n10;
        locals.var_idd_diffu_dn13 = assign34420_e39575_d_n13;
        locals.var_idd_diffu_rv = 0.0;

        let (assign34430_e39583, assign34430_e39583_d_n0, assign34430_e39583_d_n2, assign34430_e39583_d_n4, assign34430_e39583_d_n5, assign34430_e39583_d_n6, assign34430_e39583_d_n7, assign34430_e39583_d_n8, assign34430_e39583_d_n9, assign34430_e39583_d_n10, assign34430_e39583_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign34430_e39581: f64 = (locals.var_idd_drift + locals.var_idd_diffu);
        (assign34430_e39581, (locals.var_idd_drift_dn0 + locals.var_idd_diffu_dn0), (locals.var_idd_drift_dn2 + locals.var_idd_diffu_dn2), (locals.var_idd_drift_dn4 + locals.var_idd_diffu_dn4), (locals.var_idd_drift_dn5 + locals.var_idd_diffu_dn5), (locals.var_idd_drift_dn6 + locals.var_idd_diffu_dn6), (locals.var_idd_drift_dn7 + locals.var_idd_diffu_dn7), (locals.var_idd_drift_dn8 + locals.var_idd_diffu_dn8), (locals.var_idd_drift_dn9 + locals.var_idd_diffu_dn9), (locals.var_idd_drift_dn10 + locals.var_idd_diffu_dn10), (locals.var_idd_drift_dn13 + locals.var_idd_diffu_dn13),)
    } else {
        (locals.var_idd, locals.var_idd_dn0, locals.var_idd_dn2, locals.var_idd_dn4, locals.var_idd_dn5, locals.var_idd_dn6, locals.var_idd_dn7, locals.var_idd_dn8, locals.var_idd_dn9, locals.var_idd_dn10, locals.var_idd_dn13,)
    }
};
        locals.var_idd = assign34430_e39583;
        locals.var_idd_dn0 = assign34430_e39583_d_n0;
        locals.var_idd_dn2 = assign34430_e39583_d_n2;
        locals.var_idd_dn4 = assign34430_e39583_d_n4;
        locals.var_idd_dn5 = assign34430_e39583_d_n5;
        locals.var_idd_dn6 = assign34430_e39583_d_n6;
        locals.var_idd_dn7 = assign34430_e39583_d_n7;
        locals.var_idd_dn8 = assign34430_e39583_d_n8;
        locals.var_idd_dn9 = assign34430_e39583_d_n9;
        locals.var_idd_dn10 = assign34430_e39583_d_n10;
        locals.var_idd_dn13 = assign34430_e39583_d_n13;
        locals.var_idd_rv = 0.0;

        let (assign34440_e39590, assign34440_e39590_d_n0, assign34440_e39590_d_n2, assign34440_e39590_d_n4, assign34440_e39590_d_n5, assign34440_e39590_d_n6, assign34440_e39590_d_n7, assign34440_e39590_d_n8, assign34440_e39590_d_n9, assign34440_e39590_d_n10, assign34440_e39590_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign34440_e39588: f64 = (-locals.var_q_n0_cur);
        (assign34440_e39588, (-locals.var_q_n0_cur_dn0), (-locals.var_q_n0_cur_dn2), (-locals.var_q_n0_cur_dn4), (-locals.var_q_n0_cur_dn5), (-locals.var_q_n0_cur_dn6), (-locals.var_q_n0_cur_dn7), (-locals.var_q_n0_cur_dn8), (-locals.var_q_n0_cur_dn9), (-locals.var_q_n0_cur_dn10), (-locals.var_q_n0_cur_dn13),)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn4, locals.var_qiu_dn5, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn8, locals.var_qiu_dn9, locals.var_qiu_dn10, locals.var_qiu_dn13,)
    }
};
        locals.var_qiu = assign34440_e39590;
        locals.var_qiu_dn0 = assign34440_e39590_d_n0;
        locals.var_qiu_dn2 = assign34440_e39590_d_n2;
        locals.var_qiu_dn4 = assign34440_e39590_d_n4;
        locals.var_qiu_dn5 = assign34440_e39590_d_n5;
        locals.var_qiu_dn6 = assign34440_e39590_d_n6;
        locals.var_qiu_dn7 = assign34440_e39590_d_n7;
        locals.var_qiu_dn8 = assign34440_e39590_d_n8;
        locals.var_qiu_dn9 = assign34440_e39590_d_n9;
        locals.var_qiu_dn10 = assign34440_e39590_d_n10;
        locals.var_qiu_dn13 = assign34440_e39590_d_n13;
        locals.var_qiu_rv = 0.0;

        let (assign34450_e39596, assign34450_e39596_d_n0, assign34450_e39596_d_n2, assign34450_e39596_d_n4, assign34450_e39596_d_n5, assign34450_e39596_d_n6, assign34450_e39596_d_n7, assign34450_e39596_d_n8, assign34450_e39596_d_n9, assign34450_e39596_d_n10, assign34450_e39596_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        (locals.var_leff, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lch, locals.var_lch_dn0, locals.var_lch_dn2, locals.var_lch_dn4, locals.var_lch_dn5, locals.var_lch_dn6, locals.var_lch_dn7, locals.var_lch_dn8, locals.var_lch_dn9, locals.var_lch_dn10, locals.var_lch_dn13,)
    }
};
        locals.var_lch = assign34450_e39596;
        locals.var_lch_dn0 = assign34450_e39596_d_n0;
        locals.var_lch_dn2 = assign34450_e39596_d_n2;
        locals.var_lch_dn4 = assign34450_e39596_d_n4;
        locals.var_lch_dn5 = assign34450_e39596_d_n5;
        locals.var_lch_dn6 = assign34450_e39596_d_n6;
        locals.var_lch_dn7 = assign34450_e39596_d_n7;
        locals.var_lch_dn8 = assign34450_e39596_d_n8;
        locals.var_lch_dn9 = assign34450_e39596_d_n9;
        locals.var_lch_dn10 = assign34450_e39596_d_n10;
        locals.var_lch_dn13 = assign34450_e39596_d_n13;
        locals.var_lch_rv = 0.0;

        let (assign34460_e39604, assign34460_e39604_d_n0, assign34460_e39604_d_n2, assign34460_e39604_d_n4, assign34460_e39604_d_n5, assign34460_e39604_d_n6, assign34460_e39604_d_n7, assign34460_e39604_d_n8, assign34460_e39604_d_n9, assign34460_e39604_d_n10, assign34460_e39604_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign34460_e39602: f64 = (locals.var_ninv_o_esi / 100.0);
        (assign34460_e39602, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign34460_e39604;
        locals.var_t2_dn0 = assign34460_e39604_d_n0;
        locals.var_t2_dn2 = assign34460_e39604_d_n2;
        locals.var_t2_dn4 = assign34460_e39604_d_n4;
        locals.var_t2_dn5 = assign34460_e39604_d_n5;
        locals.var_t2_dn6 = assign34460_e39604_d_n6;
        locals.var_t2_dn7 = assign34460_e39604_d_n7;
        locals.var_t2_dn8 = assign34460_e39604_d_n8;
        locals.var_t2_dn9 = assign34460_e39604_d_n9;
        locals.var_t2_dn10 = assign34460_e39604_d_n10;
        locals.var_t2_dn13 = assign34460_e39604_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign34470_e39616, assign34470_e39616_d_n0, assign34470_e39616_d_n2, assign34470_e39616_d_n4, assign34470_e39616_d_n5, assign34470_e39616_d_n6, assign34470_e39616_d_n7, assign34470_e39616_d_n8, assign34470_e39616_d_n9, assign34470_e39616_d_n10, assign34470_e39616_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign34470_e39611: f64 = (locals.var_phi_sl_dep - locals.var_phi_s0_dep);
        let assign34470_e39613: f64 = (assign34470_e39611 * locals.var_ninvde);
        let assign34470_e39614: f64 = (1.0 + assign34470_e39613);
        (assign34470_e39614, (((locals.var_phi_sl_dep_dn0 - locals.var_phi_s0_dep_dn0) * locals.var_ninvde) + (assign34470_e39611 * locals.var_ninvde_dn0)), (((locals.var_phi_sl_dep_dn2 - locals.var_phi_s0_dep_dn2) * locals.var_ninvde) + (assign34470_e39611 * locals.var_ninvde_dn2)), (((locals.var_phi_sl_dep_dn4 - locals.var_phi_s0_dep_dn4) * locals.var_ninvde) + (assign34470_e39611 * locals.var_ninvde_dn4)), (((locals.var_phi_sl_dep_dn5 - locals.var_phi_s0_dep_dn5) * locals.var_ninvde) + (assign34470_e39611 * locals.var_ninvde_dn5)), (((locals.var_phi_sl_dep_dn6 - locals.var_phi_s0_dep_dn6) * locals.var_ninvde) + (assign34470_e39611 * locals.var_ninvde_dn6)), (((locals.var_phi_sl_dep_dn7 - locals.var_phi_s0_dep_dn7) * locals.var_ninvde) + (assign34470_e39611 * locals.var_ninvde_dn7)), (((locals.var_phi_sl_dep_dn8 - locals.var_phi_s0_dep_dn8) * locals.var_ninvde) + (assign34470_e39611 * locals.var_ninvde_dn8)), (((locals.var_phi_sl_dep_dn9 - locals.var_phi_s0_dep_dn9) * locals.var_ninvde) + (assign34470_e39611 * locals.var_ninvde_dn9)), (((locals.var_phi_sl_dep_dn10 - locals.var_phi_s0_dep_dn10) * locals.var_ninvde) + (assign34470_e39611 * locals.var_ninvde_dn10)), (((locals.var_phi_sl_dep_dn13 - locals.var_phi_s0_dep_dn13) * locals.var_ninvde) + (assign34470_e39611 * locals.var_ninvde_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign34470_e39616;
        locals.var_t4_dn0 = assign34470_e39616_d_n0;
        locals.var_t4_dn2 = assign34470_e39616_d_n2;
        locals.var_t4_dn4 = assign34470_e39616_d_n4;
        locals.var_t4_dn5 = assign34470_e39616_d_n5;
        locals.var_t4_dn6 = assign34470_e39616_d_n6;
        locals.var_t4_dn7 = assign34470_e39616_d_n7;
        locals.var_t4_dn8 = assign34470_e39616_d_n8;
        locals.var_t4_dn9 = assign34470_e39616_d_n9;
        locals.var_t4_dn10 = assign34470_e39616_d_n10;
        locals.var_t4_dn13 = assign34470_e39616_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign34480_e39624, assign34480_e39624_d_n0, assign34480_e39624_d_n2, assign34480_e39624_d_n4, assign34480_e39624_d_n5, assign34480_e39624_d_n6, assign34480_e39624_d_n7, assign34480_e39624_d_n8, assign34480_e39624_d_n9, assign34480_e39624_d_n10, assign34480_e39624_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign34480_e39622: f64 = (locals.var_t2 * locals.var_qiu);
        (assign34480_e39622, ((locals.var_t2_dn0 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn0)), ((locals.var_t2_dn2 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn2)), ((locals.var_t2_dn4 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn4)), ((locals.var_t2_dn5 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn5)), ((locals.var_t2_dn6 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn6)), ((locals.var_t2_dn7 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn7)), ((locals.var_t2_dn8 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn8)), ((locals.var_t2_dn9 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn9)), ((locals.var_t2_dn10 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn10)), ((locals.var_t2_dn13 * locals.var_qiu) + (locals.var_t2 * locals.var_qiu_dn13)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign34480_e39624;
        locals.var_t5_dn0 = assign34480_e39624_d_n0;
        locals.var_t5_dn2 = assign34480_e39624_d_n2;
        locals.var_t5_dn4 = assign34480_e39624_d_n4;
        locals.var_t5_dn5 = assign34480_e39624_d_n5;
        locals.var_t5_dn6 = assign34480_e39624_d_n6;
        locals.var_t5_dn7 = assign34480_e39624_d_n7;
        locals.var_t5_dn8 = assign34480_e39624_d_n8;
        locals.var_t5_dn9 = assign34480_e39624_d_n9;
        locals.var_t5_dn10 = assign34480_e39624_d_n10;
        locals.var_t5_dn13 = assign34480_e39624_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign34490_e39632, assign34490_e39632_d_n0, assign34490_e39632_d_n2, assign34490_e39632_d_n4, assign34490_e39632_d_n5, assign34490_e39632_d_n6, assign34490_e39632_d_n7, assign34490_e39632_d_n8, assign34490_e39632_d_n9, assign34490_e39632_d_n10, assign34490_e39632_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign34490_e39630: f64 = (locals.var_t5 / locals.var_t4);
        (assign34490_e39630, (((locals.var_t5_dn0 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn2 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn4 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn5 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn6 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn7 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn8 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn9 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn10 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn13 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn13)) / (locals.var_t4 * locals.var_t4)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign34490_e39632;
        locals.var_t3_dn0 = assign34490_e39632_d_n0;
        locals.var_t3_dn2 = assign34490_e39632_d_n2;
        locals.var_t3_dn4 = assign34490_e39632_d_n4;
        locals.var_t3_dn5 = assign34490_e39632_d_n5;
        locals.var_t3_dn6 = assign34490_e39632_d_n6;
        locals.var_t3_dn7 = assign34490_e39632_d_n7;
        locals.var_t3_dn8 = assign34490_e39632_d_n8;
        locals.var_t3_dn9 = assign34490_e39632_d_n9;
        locals.var_t3_dn10 = assign34490_e39632_d_n10;
        locals.var_t3_dn13 = assign34490_e39632_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign34500_e39638, assign34500_e39638_d_n0, assign34500_e39638_d_n2, assign34500_e39638_d_n4, assign34500_e39638_d_n5, assign34500_e39638_d_n6, assign34500_e39638_d_n7, assign34500_e39638_d_n8, assign34500_e39638_d_n9, assign34500_e39638_d_n10, assign34500_e39638_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    } else {
        (locals.var_eeff, locals.var_eeff_dn0, locals.var_eeff_dn2, locals.var_eeff_dn4, locals.var_eeff_dn5, locals.var_eeff_dn6, locals.var_eeff_dn7, locals.var_eeff_dn8, locals.var_eeff_dn9, locals.var_eeff_dn10, locals.var_eeff_dn13,)
    }
};
        locals.var_eeff = assign34500_e39638;
        locals.var_eeff_dn0 = assign34500_e39638_d_n0;
        locals.var_eeff_dn2 = assign34500_e39638_d_n2;
        locals.var_eeff_dn4 = assign34500_e39638_d_n4;
        locals.var_eeff_dn5 = assign34500_e39638_d_n5;
        locals.var_eeff_dn6 = assign34500_e39638_d_n6;
        locals.var_eeff_dn7 = assign34500_e39638_d_n7;
        locals.var_eeff_dn8 = assign34500_e39638_d_n8;
        locals.var_eeff_dn9 = assign34500_e39638_d_n9;
        locals.var_eeff_dn10 = assign34500_e39638_d_n10;
        locals.var_eeff_dn13 = assign34500_e39638_d_n13;
        locals.var_eeff_rv = 0.0;

        let (assign34510_e39653, assign34510_e39653_d_n0, assign34510_e39653_d_n2, assign34510_e39653_d_n4, assign34510_e39653_d_n5, assign34510_e39653_d_n6, assign34510_e39653_d_n7, assign34510_e39653_d_n8, assign34510_e39653_d_n9, assign34510_e39653_d_n10, assign34510_e39653_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let (assign34510_e39651, assign34510_e39651_d_n0, assign34510_e39651_d_n2, assign34510_e39651_d_n4, assign34510_e39651_d_n5, assign34510_e39651_d_n6, assign34510_e39651_d_n7, assign34510_e39651_d_n8, assign34510_e39651_d_n9, assign34510_e39651_d_n10, assign34510_e39651_d_n13,) = {
            if (locals.var_eeff == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign34510_e39649: f64 = (p.p160 - 1.0);
                let assign34510_e39650: f64 = (locals.var_eeff).powf(assign34510_e39649);
                (assign34510_e39650, if 0.0 == 0.0 && ((assign34510_e39649) as f64).is_finite() && ((assign34510_e39649) as f64).fract() == 0.0 { if assign34510_e39649 == 0.0 { 0.0 } else { (assign34510_e39649 * ((locals.var_eeff).powf(assign34510_e39649 - 1.0) * locals.var_eeff_dn0)) } } else { (assign34510_e39650 * (assign34510_e39649 * (locals.var_eeff_dn0 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34510_e39649) as f64).is_finite() && ((assign34510_e39649) as f64).fract() == 0.0 { if assign34510_e39649 == 0.0 { 0.0 } else { (assign34510_e39649 * ((locals.var_eeff).powf(assign34510_e39649 - 1.0) * locals.var_eeff_dn2)) } } else { (assign34510_e39650 * (assign34510_e39649 * (locals.var_eeff_dn2 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34510_e39649) as f64).is_finite() && ((assign34510_e39649) as f64).fract() == 0.0 { if assign34510_e39649 == 0.0 { 0.0 } else { (assign34510_e39649 * ((locals.var_eeff).powf(assign34510_e39649 - 1.0) * locals.var_eeff_dn4)) } } else { (assign34510_e39650 * (assign34510_e39649 * (locals.var_eeff_dn4 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34510_e39649) as f64).is_finite() && ((assign34510_e39649) as f64).fract() == 0.0 { if assign34510_e39649 == 0.0 { 0.0 } else { (assign34510_e39649 * ((locals.var_eeff).powf(assign34510_e39649 - 1.0) * locals.var_eeff_dn5)) } } else { (assign34510_e39650 * (assign34510_e39649 * (locals.var_eeff_dn5 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34510_e39649) as f64).is_finite() && ((assign34510_e39649) as f64).fract() == 0.0 { if assign34510_e39649 == 0.0 { 0.0 } else { (assign34510_e39649 * ((locals.var_eeff).powf(assign34510_e39649 - 1.0) * locals.var_eeff_dn6)) } } else { (assign34510_e39650 * (assign34510_e39649 * (locals.var_eeff_dn6 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34510_e39649) as f64).is_finite() && ((assign34510_e39649) as f64).fract() == 0.0 { if assign34510_e39649 == 0.0 { 0.0 } else { (assign34510_e39649 * ((locals.var_eeff).powf(assign34510_e39649 - 1.0) * locals.var_eeff_dn7)) } } else { (assign34510_e39650 * (assign34510_e39649 * (locals.var_eeff_dn7 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34510_e39649) as f64).is_finite() && ((assign34510_e39649) as f64).fract() == 0.0 { if assign34510_e39649 == 0.0 { 0.0 } else { (assign34510_e39649 * ((locals.var_eeff).powf(assign34510_e39649 - 1.0) * locals.var_eeff_dn8)) } } else { (assign34510_e39650 * (assign34510_e39649 * (locals.var_eeff_dn8 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34510_e39649) as f64).is_finite() && ((assign34510_e39649) as f64).fract() == 0.0 { if assign34510_e39649 == 0.0 { 0.0 } else { (assign34510_e39649 * ((locals.var_eeff).powf(assign34510_e39649 - 1.0) * locals.var_eeff_dn9)) } } else { (assign34510_e39650 * (assign34510_e39649 * (locals.var_eeff_dn9 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34510_e39649) as f64).is_finite() && ((assign34510_e39649) as f64).fract() == 0.0 { if assign34510_e39649 == 0.0 { 0.0 } else { (assign34510_e39649 * ((locals.var_eeff).powf(assign34510_e39649 - 1.0) * locals.var_eeff_dn10)) } } else { (assign34510_e39650 * (assign34510_e39649 * (locals.var_eeff_dn10 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34510_e39649) as f64).is_finite() && ((assign34510_e39649) as f64).fract() == 0.0 { if assign34510_e39649 == 0.0 { 0.0 } else { (assign34510_e39649 * ((locals.var_eeff).powf(assign34510_e39649 - 1.0) * locals.var_eeff_dn13)) } } else { (assign34510_e39650 * (assign34510_e39649 * (locals.var_eeff_dn13 / locals.var_eeff))) },)
            }
        };
        (assign34510_e39651, assign34510_e39651_d_n0, assign34510_e39651_d_n2, assign34510_e39651_d_n4, assign34510_e39651_d_n5, assign34510_e39651_d_n6, assign34510_e39651_d_n7, assign34510_e39651_d_n8, assign34510_e39651_d_n9, assign34510_e39651_d_n10, assign34510_e39651_d_n13,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign34510_e39653;
        locals.var_t5_dn0 = assign34510_e39653_d_n0;
        locals.var_t5_dn2 = assign34510_e39653_d_n2;
        locals.var_t5_dn4 = assign34510_e39653_d_n4;
        locals.var_t5_dn5 = assign34510_e39653_d_n5;
        locals.var_t5_dn6 = assign34510_e39653_d_n6;
        locals.var_t5_dn7 = assign34510_e39653_d_n7;
        locals.var_t5_dn8 = assign34510_e39653_d_n8;
        locals.var_t5_dn9 = assign34510_e39653_d_n9;
        locals.var_t5_dn10 = assign34510_e39653_d_n10;
        locals.var_t5_dn13 = assign34510_e39653_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign34520_e39661, assign34520_e39661_d_n0, assign34520_e39661_d_n2, assign34520_e39661_d_n4, assign34520_e39661_d_n5, assign34520_e39661_d_n6, assign34520_e39661_d_n7, assign34520_e39661_d_n8, assign34520_e39661_d_n9, assign34520_e39661_d_n10, assign34520_e39661_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign34520_e39659: f64 = (locals.var_t5 * locals.var_eeff);
        (assign34520_e39659, ((locals.var_t5_dn0 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn0)), ((locals.var_t5_dn2 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn2)), ((locals.var_t5_dn4 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn4)), ((locals.var_t5_dn5 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn5)), ((locals.var_t5_dn6 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn6)), ((locals.var_t5_dn7 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn7)), ((locals.var_t5_dn8 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn8)), ((locals.var_t5_dn9 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn9)), ((locals.var_t5_dn10 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn10)), ((locals.var_t5_dn13 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn13)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn13,)
    }
};
        locals.var_t8 = assign34520_e39661;
        locals.var_t8_dn0 = assign34520_e39661_d_n0;
        locals.var_t8_dn2 = assign34520_e39661_d_n2;
        locals.var_t8_dn4 = assign34520_e39661_d_n4;
        locals.var_t8_dn5 = assign34520_e39661_d_n5;
        locals.var_t8_dn6 = assign34520_e39661_d_n6;
        locals.var_t8_dn7 = assign34520_e39661_d_n7;
        locals.var_t8_dn8 = assign34520_e39661_d_n8;
        locals.var_t8_dn9 = assign34520_e39661_d_n9;
        locals.var_t8_dn10 = assign34520_e39661_d_n10;
        locals.var_t8_dn13 = assign34520_e39661_d_n13;
        locals.var_t8_rv = 0.0;

        let (assign34530_e39676, assign34530_e39676_d_n0, assign34530_e39676_d_n2, assign34530_e39676_d_n4, assign34530_e39676_d_n5, assign34530_e39676_d_n6, assign34530_e39676_d_n7, assign34530_e39676_d_n8, assign34530_e39676_d_n9, assign34530_e39676_d_n10, assign34530_e39676_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let (assign34530_e39674, assign34530_e39674_d_n0, assign34530_e39674_d_n2, assign34530_e39674_d_n4, assign34530_e39674_d_n5, assign34530_e39674_d_n6, assign34530_e39674_d_n7, assign34530_e39674_d_n8, assign34530_e39674_d_n9, assign34530_e39674_d_n10, assign34530_e39674_d_n13,) = {
            if (locals.var_eeff == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign34530_e39672: f64 = (locals.var_muesr - 1.0);
                let assign34530_e39673: f64 = (locals.var_eeff).powf(assign34530_e39672);
                (assign34530_e39673, if 0.0 == 0.0 && ((assign34530_e39672) as f64).is_finite() && ((assign34530_e39672) as f64).fract() == 0.0 { if assign34530_e39672 == 0.0 { 0.0 } else { (assign34530_e39672 * ((locals.var_eeff).powf(assign34530_e39672 - 1.0) * locals.var_eeff_dn0)) } } else { (assign34530_e39673 * (assign34530_e39672 * (locals.var_eeff_dn0 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34530_e39672) as f64).is_finite() && ((assign34530_e39672) as f64).fract() == 0.0 { if assign34530_e39672 == 0.0 { 0.0 } else { (assign34530_e39672 * ((locals.var_eeff).powf(assign34530_e39672 - 1.0) * locals.var_eeff_dn2)) } } else { (assign34530_e39673 * (assign34530_e39672 * (locals.var_eeff_dn2 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34530_e39672) as f64).is_finite() && ((assign34530_e39672) as f64).fract() == 0.0 { if assign34530_e39672 == 0.0 { 0.0 } else { (assign34530_e39672 * ((locals.var_eeff).powf(assign34530_e39672 - 1.0) * locals.var_eeff_dn4)) } } else { (assign34530_e39673 * (assign34530_e39672 * (locals.var_eeff_dn4 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34530_e39672) as f64).is_finite() && ((assign34530_e39672) as f64).fract() == 0.0 { if assign34530_e39672 == 0.0 { 0.0 } else { (assign34530_e39672 * ((locals.var_eeff).powf(assign34530_e39672 - 1.0) * locals.var_eeff_dn5)) } } else { (assign34530_e39673 * (assign34530_e39672 * (locals.var_eeff_dn5 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34530_e39672) as f64).is_finite() && ((assign34530_e39672) as f64).fract() == 0.0 { if assign34530_e39672 == 0.0 { 0.0 } else { (assign34530_e39672 * ((locals.var_eeff).powf(assign34530_e39672 - 1.0) * locals.var_eeff_dn6)) } } else { (assign34530_e39673 * (assign34530_e39672 * (locals.var_eeff_dn6 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34530_e39672) as f64).is_finite() && ((assign34530_e39672) as f64).fract() == 0.0 { if assign34530_e39672 == 0.0 { 0.0 } else { (assign34530_e39672 * ((locals.var_eeff).powf(assign34530_e39672 - 1.0) * locals.var_eeff_dn7)) } } else { (assign34530_e39673 * (assign34530_e39672 * (locals.var_eeff_dn7 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34530_e39672) as f64).is_finite() && ((assign34530_e39672) as f64).fract() == 0.0 { if assign34530_e39672 == 0.0 { 0.0 } else { (assign34530_e39672 * ((locals.var_eeff).powf(assign34530_e39672 - 1.0) * locals.var_eeff_dn8)) } } else { (assign34530_e39673 * (assign34530_e39672 * (locals.var_eeff_dn8 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34530_e39672) as f64).is_finite() && ((assign34530_e39672) as f64).fract() == 0.0 { if assign34530_e39672 == 0.0 { 0.0 } else { (assign34530_e39672 * ((locals.var_eeff).powf(assign34530_e39672 - 1.0) * locals.var_eeff_dn9)) } } else { (assign34530_e39673 * (assign34530_e39672 * (locals.var_eeff_dn9 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34530_e39672) as f64).is_finite() && ((assign34530_e39672) as f64).fract() == 0.0 { if assign34530_e39672 == 0.0 { 0.0 } else { (assign34530_e39672 * ((locals.var_eeff).powf(assign34530_e39672 - 1.0) * locals.var_eeff_dn10)) } } else { (assign34530_e39673 * (assign34530_e39672 * (locals.var_eeff_dn10 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign34530_e39672) as f64).is_finite() && ((assign34530_e39672) as f64).fract() == 0.0 { if assign34530_e39672 == 0.0 { 0.0 } else { (assign34530_e39672 * ((locals.var_eeff).powf(assign34530_e39672 - 1.0) * locals.var_eeff_dn13)) } } else { (assign34530_e39673 * (assign34530_e39672 * (locals.var_eeff_dn13 / locals.var_eeff))) },)
            }
        };
        (assign34530_e39674, assign34530_e39674_d_n0, assign34530_e39674_d_n2, assign34530_e39674_d_n4, assign34530_e39674_d_n5, assign34530_e39674_d_n6, assign34530_e39674_d_n7, assign34530_e39674_d_n8, assign34530_e39674_d_n9, assign34530_e39674_d_n10, assign34530_e39674_d_n13,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign34530_e39676;
        locals.var_t7_dn0 = assign34530_e39676_d_n0;
        locals.var_t7_dn2 = assign34530_e39676_d_n2;
        locals.var_t7_dn4 = assign34530_e39676_d_n4;
        locals.var_t7_dn5 = assign34530_e39676_d_n5;
        locals.var_t7_dn6 = assign34530_e39676_d_n6;
        locals.var_t7_dn7 = assign34530_e39676_d_n7;
        locals.var_t7_dn8 = assign34530_e39676_d_n8;
        locals.var_t7_dn9 = assign34530_e39676_d_n9;
        locals.var_t7_dn10 = assign34530_e39676_d_n10;
        locals.var_t7_dn13 = assign34530_e39676_d_n13;
        locals.var_t7_rv = 0.0;

        let (assign34540_e39684, assign34540_e39684_d_n0, assign34540_e39684_d_n2, assign34540_e39684_d_n4, assign34540_e39684_d_n5, assign34540_e39684_d_n6, assign34540_e39684_d_n7, assign34540_e39684_d_n8, assign34540_e39684_d_n9, assign34540_e39684_d_n10, assign34540_e39684_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign34540_e39682: f64 = (locals.var_t7 * locals.var_eeff);
        (assign34540_e39682, ((locals.var_t7_dn0 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn0)), ((locals.var_t7_dn2 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn2)), ((locals.var_t7_dn4 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn4)), ((locals.var_t7_dn5 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn5)), ((locals.var_t7_dn6 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn6)), ((locals.var_t7_dn7 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn7)), ((locals.var_t7_dn8 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn8)), ((locals.var_t7_dn9 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn9)), ((locals.var_t7_dn10 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn10)), ((locals.var_t7_dn13 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn13)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign34540_e39684;
        locals.var_t6_dn0 = assign34540_e39684_d_n0;
        locals.var_t6_dn2 = assign34540_e39684_d_n2;
        locals.var_t6_dn4 = assign34540_e39684_d_n4;
        locals.var_t6_dn5 = assign34540_e39684_d_n5;
        locals.var_t6_dn6 = assign34540_e39684_d_n6;
        locals.var_t6_dn7 = assign34540_e39684_d_n7;
        locals.var_t6_dn8 = assign34540_e39684_d_n8;
        locals.var_t6_dn9 = assign34540_e39684_d_n9;
        locals.var_t6_dn10 = assign34540_e39684_d_n10;
        locals.var_t6_dn13 = assign34540_e39684_d_n13;
        locals.var_t6_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_108(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign34550_e39692, assign34550_e39692_d_n0, assign34550_e39692_d_n2, assign34550_e39692_d_n4, assign34550_e39692_d_n5, assign34550_e39692_d_n6, assign34550_e39692_d_n7, assign34550_e39692_d_n8, assign34550_e39692_d_n9, assign34550_e39692_d_n10, assign34550_e39692_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign34550_e39690: f64 = (1.6021918e-19 * 10000.0);
        (assign34550_e39690, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign34550_e39692;
        locals.var_t9_dn0 = assign34550_e39692_d_n0;
        locals.var_t9_dn2 = assign34550_e39692_d_n2;
        locals.var_t9_dn4 = assign34550_e39692_d_n4;
        locals.var_t9_dn5 = assign34550_e39692_d_n5;
        locals.var_t9_dn6 = assign34550_e39692_d_n6;
        locals.var_t9_dn7 = assign34550_e39692_d_n7;
        locals.var_t9_dn8 = assign34550_e39692_d_n8;
        locals.var_t9_dn9 = assign34550_e39692_d_n9;
        locals.var_t9_dn10 = assign34550_e39692_d_n10;
        locals.var_t9_dn13 = assign34550_e39692_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign34560_e39700, assign34560_e39700_d_n0, assign34560_e39700_d_n2, assign34560_e39700_d_n4, assign34560_e39700_d_n5, assign34560_e39700_d_n6, assign34560_e39700_d_n7, assign34560_e39700_d_n8, assign34560_e39700_d_n9, assign34560_e39700_d_n10, assign34560_e39700_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign34560_e39698: f64 = (locals.var_qiu / locals.var_t9);
        (assign34560_e39698, (((locals.var_qiu_dn0 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn0)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn2 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn2)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn4 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn4)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn5 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn5)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn6 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn6)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn7 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn7)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn8 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn8)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn9 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn9)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn10 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn10)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn13 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn13)) / (locals.var_t9 * locals.var_t9)),)
    } else {
        (locals.var_rns, locals.var_rns_dn0, locals.var_rns_dn2, locals.var_rns_dn4, locals.var_rns_dn5, locals.var_rns_dn6, locals.var_rns_dn7, locals.var_rns_dn8, locals.var_rns_dn9, locals.var_rns_dn10, locals.var_rns_dn13,)
    }
};
        locals.var_rns = assign34560_e39700;
        locals.var_rns_dn0 = assign34560_e39700_d_n0;
        locals.var_rns_dn2 = assign34560_e39700_d_n2;
        locals.var_rns_dn4 = assign34560_e39700_d_n4;
        locals.var_rns_dn5 = assign34560_e39700_d_n5;
        locals.var_rns_dn6 = assign34560_e39700_d_n6;
        locals.var_rns_dn7 = assign34560_e39700_d_n7;
        locals.var_rns_dn8 = assign34560_e39700_d_n8;
        locals.var_rns_dn9 = assign34560_e39700_d_n9;
        locals.var_rns_dn10 = assign34560_e39700_d_n10;
        locals.var_rns_dn13 = assign34560_e39700_d_n13;
        locals.var_rns_rv = 0.0;

        let (assign34570_e39724, assign34570_e39724_d_n0, assign34570_e39724_d_n2, assign34570_e39724_d_n4, assign34570_e39724_d_n5, assign34570_e39724_d_n6, assign34570_e39724_d_n7, assign34570_e39724_d_n8, assign34570_e39724_d_n9, assign34570_e39724_d_n10, assign34570_e39724_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign34570_e39708: f64 = (locals.var_uc_muecb1 * locals.var_rns);
        let assign34570_e39710: f64 = (assign34570_e39708 / 100000000000.0);
        let assign34570_e39711: f64 = (locals.var_uc_muecb0 + assign34570_e39710);
        let assign34570_e39713: f64 = (assign34570_e39711 + 1e-25);
        let assign34570_e39714: f64 = (1.0 / assign34570_e39713);
        let assign34570_e39717: f64 = (locals.var_mphn0 * locals.var_t8);
        let assign34570_e39718: f64 = (assign34570_e39714 + assign34570_e39717);
        let assign34570_e39721: f64 = (locals.var_t6 / locals.var_uc_muesr1);
        let assign34570_e39722: f64 = (assign34570_e39718 + assign34570_e39721);
        (assign34570_e39722, (((-(((locals.var_uc_muecb1 * locals.var_rns_dn0) / 100000000000.0) / (assign34570_e39713 * assign34570_e39713))) + ((locals.var_mphn0_dn0 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn0))) + (locals.var_t6_dn0 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn2) / 100000000000.0) / (assign34570_e39713 * assign34570_e39713))) + ((locals.var_mphn0_dn2 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn2))) + (locals.var_t6_dn2 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn4) / 100000000000.0) / (assign34570_e39713 * assign34570_e39713))) + ((locals.var_mphn0_dn4 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn4))) + (locals.var_t6_dn4 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn5) / 100000000000.0) / (assign34570_e39713 * assign34570_e39713))) + ((locals.var_mphn0_dn5 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn5))) + (locals.var_t6_dn5 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn6) / 100000000000.0) / (assign34570_e39713 * assign34570_e39713))) + ((locals.var_mphn0_dn6 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn6))) + (locals.var_t6_dn6 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn7) / 100000000000.0) / (assign34570_e39713 * assign34570_e39713))) + ((locals.var_mphn0_dn7 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn7))) + (locals.var_t6_dn7 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn8) / 100000000000.0) / (assign34570_e39713 * assign34570_e39713))) + ((locals.var_mphn0_dn8 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn8))) + (locals.var_t6_dn8 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn9) / 100000000000.0) / (assign34570_e39713 * assign34570_e39713))) + ((locals.var_mphn0_dn9 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn9))) + (locals.var_t6_dn9 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn10) / 100000000000.0) / (assign34570_e39713 * assign34570_e39713))) + ((locals.var_mphn0_dn10 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn10))) + (locals.var_t6_dn10 / locals.var_uc_muesr1)), (((-(((locals.var_uc_muecb1 * locals.var_rns_dn13) / 100000000000.0) / (assign34570_e39713 * assign34570_e39713))) + ((locals.var_mphn0_dn13 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn13))) + (locals.var_t6_dn13 / locals.var_uc_muesr1)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign34570_e39724;
        locals.var_t1_dn0 = assign34570_e39724_d_n0;
        locals.var_t1_dn2 = assign34570_e39724_d_n2;
        locals.var_t1_dn4 = assign34570_e39724_d_n4;
        locals.var_t1_dn5 = assign34570_e39724_d_n5;
        locals.var_t1_dn6 = assign34570_e39724_d_n6;
        locals.var_t1_dn7 = assign34570_e39724_d_n7;
        locals.var_t1_dn8 = assign34570_e39724_d_n8;
        locals.var_t1_dn9 = assign34570_e39724_d_n9;
        locals.var_t1_dn10 = assign34570_e39724_d_n10;
        locals.var_t1_dn13 = assign34570_e39724_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign34580_e39732, assign34580_e39732_d_n0, assign34580_e39732_d_n2, assign34580_e39732_d_n4, assign34580_e39732_d_n5, assign34580_e39732_d_n6, assign34580_e39732_d_n7, assign34580_e39732_d_n8, assign34580_e39732_d_n9, assign34580_e39732_d_n10, assign34580_e39732_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign34580_e39730: f64 = (1.0 / locals.var_t1);
        (assign34580_e39730, (-(locals.var_t1_dn0 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn2 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn4 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn5 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn6 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn7 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn8 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn9 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn10 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn13 / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn13,)
    }
};
        locals.var_muun = assign34580_e39732;
        locals.var_muun_dn0 = assign34580_e39732_d_n0;
        locals.var_muun_dn2 = assign34580_e39732_d_n2;
        locals.var_muun_dn4 = assign34580_e39732_d_n4;
        locals.var_muun_dn5 = assign34580_e39732_d_n5;
        locals.var_muun_dn6 = assign34580_e39732_d_n6;
        locals.var_muun_dn7 = assign34580_e39732_d_n7;
        locals.var_muun_dn8 = assign34580_e39732_d_n8;
        locals.var_muun_dn9 = assign34580_e39732_d_n9;
        locals.var_muun_dn10 = assign34580_e39732_d_n10;
        locals.var_muun_dn13 = assign34580_e39732_d_n13;
        locals.var_muun_rv = 0.0;

        let (assign34590_e39740, assign34590_e39740_d_n0, assign34590_e39740_d_n2, assign34590_e39740_d_n4, assign34590_e39740_d_n5, assign34590_e39740_d_n6, assign34590_e39740_d_n7, assign34590_e39740_d_n8, assign34590_e39740_d_n9, assign34590_e39740_d_n10, assign34590_e39740_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign34590_e39738: f64 = (locals.var_muun / 10000.0);
        (assign34590_e39738, (locals.var_muun_dn0 / 10000.0), (locals.var_muun_dn2 / 10000.0), (locals.var_muun_dn4 / 10000.0), (locals.var_muun_dn5 / 10000.0), (locals.var_muun_dn6 / 10000.0), (locals.var_muun_dn7 / 10000.0), (locals.var_muun_dn8 / 10000.0), (locals.var_muun_dn9 / 10000.0), (locals.var_muun_dn10 / 10000.0), (locals.var_muun_dn13 / 10000.0),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn13,)
    }
};
        locals.var_muun = assign34590_e39740;
        locals.var_muun_dn0 = assign34590_e39740_d_n0;
        locals.var_muun_dn2 = assign34590_e39740_d_n2;
        locals.var_muun_dn4 = assign34590_e39740_d_n4;
        locals.var_muun_dn5 = assign34590_e39740_d_n5;
        locals.var_muun_dn6 = assign34590_e39740_d_n6;
        locals.var_muun_dn7 = assign34590_e39740_d_n7;
        locals.var_muun_dn8 = assign34590_e39740_d_n8;
        locals.var_muun_dn9 = assign34590_e39740_d_n9;
        locals.var_muun_dn10 = assign34590_e39740_d_n10;
        locals.var_muun_dn13 = assign34590_e39740_d_n13;
        locals.var_muun_rv = 0.0;

        let (assign34600_e39752, assign34600_e39752_d_n0, assign34600_e39752_d_n2, assign34600_e39752_d_n4, assign34600_e39752_d_n5, assign34600_e39752_d_n6, assign34600_e39752_d_n7, assign34600_e39752_d_n8, assign34600_e39752_d_n9, assign34600_e39752_d_n10, assign34600_e39752_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign34600_e39747: f64 = (locals.var_qiu + 1e-25);
        let assign34600_e39748: f64 = (locals.var_beta * assign34600_e39747);
        let assign34600_e39750: f64 = (assign34600_e39748 * locals.var_lch);
        (assign34600_e39750, ((((locals.var_beta_dn0 * assign34600_e39747) + (locals.var_beta * locals.var_qiu_dn0)) * locals.var_lch) + (assign34600_e39748 * locals.var_lch_dn0)), ((((locals.var_beta_dn2 * assign34600_e39747) + (locals.var_beta * locals.var_qiu_dn2)) * locals.var_lch) + (assign34600_e39748 * locals.var_lch_dn2)), ((((locals.var_beta_dn4 * assign34600_e39747) + (locals.var_beta * locals.var_qiu_dn4)) * locals.var_lch) + (assign34600_e39748 * locals.var_lch_dn4)), ((((locals.var_beta_dn5 * assign34600_e39747) + (locals.var_beta * locals.var_qiu_dn5)) * locals.var_lch) + (assign34600_e39748 * locals.var_lch_dn5)), ((((locals.var_beta_dn6 * assign34600_e39747) + (locals.var_beta * locals.var_qiu_dn6)) * locals.var_lch) + (assign34600_e39748 * locals.var_lch_dn6)), ((((locals.var_beta_dn7 * assign34600_e39747) + (locals.var_beta * locals.var_qiu_dn7)) * locals.var_lch) + (assign34600_e39748 * locals.var_lch_dn7)), ((((locals.var_beta_dn8 * assign34600_e39747) + (locals.var_beta * locals.var_qiu_dn8)) * locals.var_lch) + (assign34600_e39748 * locals.var_lch_dn8)), ((((locals.var_beta_dn9 * assign34600_e39747) + (locals.var_beta * locals.var_qiu_dn9)) * locals.var_lch) + (assign34600_e39748 * locals.var_lch_dn9)), ((((locals.var_beta_dn10 * assign34600_e39747) + (locals.var_beta * locals.var_qiu_dn10)) * locals.var_lch) + (assign34600_e39748 * locals.var_lch_dn10)), ((((locals.var_beta_dn13 * assign34600_e39747) + (locals.var_beta * locals.var_qiu_dn13)) * locals.var_lch) + (assign34600_e39748 * locals.var_lch_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign34600_e39752;
        locals.var_t2_dn0 = assign34600_e39752_d_n0;
        locals.var_t2_dn2 = assign34600_e39752_d_n2;
        locals.var_t2_dn4 = assign34600_e39752_d_n4;
        locals.var_t2_dn5 = assign34600_e39752_d_n5;
        locals.var_t2_dn6 = assign34600_e39752_d_n6;
        locals.var_t2_dn7 = assign34600_e39752_d_n7;
        locals.var_t2_dn8 = assign34600_e39752_d_n8;
        locals.var_t2_dn9 = assign34600_e39752_d_n9;
        locals.var_t2_dn10 = assign34600_e39752_d_n10;
        locals.var_t2_dn13 = assign34600_e39752_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign34610_e39760, assign34610_e39760_d_n0, assign34610_e39760_d_n2, assign34610_e39760_d_n4, assign34610_e39760_d_n5, assign34610_e39760_d_n6, assign34610_e39760_d_n7, assign34610_e39760_d_n8, assign34610_e39760_d_n9, assign34610_e39760_d_n10, assign34610_e39760_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign34610_e39758: f64 = (1.0 / locals.var_t2);
        (assign34610_e39758, (-(locals.var_t2_dn0 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn2 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn4 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn5 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn6 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn7 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn8 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn9 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn10 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn13 / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign34610_e39760;
        locals.var_t1_dn0 = assign34610_e39760_d_n0;
        locals.var_t1_dn2 = assign34610_e39760_d_n2;
        locals.var_t1_dn4 = assign34610_e39760_d_n4;
        locals.var_t1_dn5 = assign34610_e39760_d_n5;
        locals.var_t1_dn6 = assign34610_e39760_d_n6;
        locals.var_t1_dn7 = assign34610_e39760_d_n7;
        locals.var_t1_dn8 = assign34610_e39760_d_n8;
        locals.var_t1_dn9 = assign34610_e39760_d_n9;
        locals.var_t1_dn10 = assign34610_e39760_d_n10;
        locals.var_t1_dn13 = assign34610_e39760_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign34620_e39768, assign34620_e39768_d_n0, assign34620_e39768_d_n2, assign34620_e39768_d_n4, assign34620_e39768_d_n5, assign34620_e39768_d_n6, assign34620_e39768_d_n7, assign34620_e39768_d_n8, assign34620_e39768_d_n9, assign34620_e39768_d_n10, assign34620_e39768_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign34620_e39766: f64 = (locals.var_idd * locals.var_t1);
        (assign34620_e39766, ((locals.var_idd_dn0 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn0)), ((locals.var_idd_dn2 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn2)), ((locals.var_idd_dn4 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn4)), ((locals.var_idd_dn5 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn5)), ((locals.var_idd_dn6 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn6)), ((locals.var_idd_dn7 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn7)), ((locals.var_idd_dn8 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn8)), ((locals.var_idd_dn9 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn9)), ((locals.var_idd_dn10 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn10)), ((locals.var_idd_dn13 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn13)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn13,)
    }
};
        locals.var_ty = assign34620_e39768;
        locals.var_ty_dn0 = assign34620_e39768_d_n0;
        locals.var_ty_dn2 = assign34620_e39768_d_n2;
        locals.var_ty_dn4 = assign34620_e39768_d_n4;
        locals.var_ty_dn5 = assign34620_e39768_d_n5;
        locals.var_ty_dn6 = assign34620_e39768_d_n6;
        locals.var_ty_dn7 = assign34620_e39768_d_n7;
        locals.var_ty_dn8 = assign34620_e39768_d_n8;
        locals.var_ty_dn9 = assign34620_e39768_d_n9;
        locals.var_ty_dn10 = assign34620_e39768_d_n10;
        locals.var_ty_dn13 = assign34620_e39768_d_n13;
        locals.var_ty_rv = 0.0;

        let (assign34630_e39778, assign34630_e39778_d_n0, assign34630_e39778_d_n2, assign34630_e39778_d_n4, assign34630_e39778_d_n5, assign34630_e39778_d_n6, assign34630_e39778_d_n7, assign34630_e39778_d_n8, assign34630_e39778_d_n9, assign34630_e39778_d_n10, assign34630_e39778_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign34630_e39774: f64 = (0.2 * locals.var_vmaxe);
        let assign34630_e39776: f64 = (assign34630_e39774 / locals.var_muun);
        (assign34630_e39776, ((((0.2 * locals.var_vmaxe_dn0) * locals.var_muun) - (assign34630_e39774 * locals.var_muun_dn0)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn2) * locals.var_muun) - (assign34630_e39774 * locals.var_muun_dn2)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn4) * locals.var_muun) - (assign34630_e39774 * locals.var_muun_dn4)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn5) * locals.var_muun) - (assign34630_e39774 * locals.var_muun_dn5)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn6) * locals.var_muun) - (assign34630_e39774 * locals.var_muun_dn6)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn7) * locals.var_muun) - (assign34630_e39774 * locals.var_muun_dn7)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn8) * locals.var_muun) - (assign34630_e39774 * locals.var_muun_dn8)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn9) * locals.var_muun) - (assign34630_e39774 * locals.var_muun_dn9)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn10) * locals.var_muun) - (assign34630_e39774 * locals.var_muun_dn10)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn13) * locals.var_muun) - (assign34630_e39774 * locals.var_muun_dn13)) / (locals.var_muun * locals.var_muun)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign34630_e39778;
        locals.var_t2_dn0 = assign34630_e39778_d_n0;
        locals.var_t2_dn2 = assign34630_e39778_d_n2;
        locals.var_t2_dn4 = assign34630_e39778_d_n4;
        locals.var_t2_dn5 = assign34630_e39778_d_n5;
        locals.var_t2_dn6 = assign34630_e39778_d_n6;
        locals.var_t2_dn7 = assign34630_e39778_d_n7;
        locals.var_t2_dn8 = assign34630_e39778_d_n8;
        locals.var_t2_dn9 = assign34630_e39778_d_n9;
        locals.var_t2_dn10 = assign34630_e39778_d_n10;
        locals.var_t2_dn13 = assign34630_e39778_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign34640_e39791, assign34640_e39791_d_n0, assign34640_e39791_d_n2, assign34640_e39791_d_n4, assign34640_e39791_d_n5, assign34640_e39791_d_n6, assign34640_e39791_d_n7, assign34640_e39791_d_n8, assign34640_e39791_d_n9, assign34640_e39791_d_n10, assign34640_e39791_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign34640_e39784: f64 = (locals.var_ty * locals.var_ty);
        let assign34640_e39787: f64 = (locals.var_t2 * locals.var_t2);
        let assign34640_e39788: f64 = (assign34640_e39784 + assign34640_e39787);
        let assign34640_e39789: f64 = (assign34640_e39788).sqrt();
        (assign34640_e39789, ((((locals.var_ty_dn0 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn0)) + ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0))) / (2.0 * assign34640_e39789)), ((((locals.var_ty_dn2 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn2)) + ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2))) / (2.0 * assign34640_e39789)), ((((locals.var_ty_dn4 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn4)) + ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4))) / (2.0 * assign34640_e39789)), ((((locals.var_ty_dn5 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn5)) + ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5))) / (2.0 * assign34640_e39789)), ((((locals.var_ty_dn6 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn6)) + ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6))) / (2.0 * assign34640_e39789)), ((((locals.var_ty_dn7 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn7)) + ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7))) / (2.0 * assign34640_e39789)), ((((locals.var_ty_dn8 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn8)) + ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8))) / (2.0 * assign34640_e39789)), ((((locals.var_ty_dn9 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn9)) + ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9))) / (2.0 * assign34640_e39789)), ((((locals.var_ty_dn10 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn10)) + ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10))) / (2.0 * assign34640_e39789)), ((((locals.var_ty_dn13 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn13)) + ((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13))) / (2.0 * assign34640_e39789)),)
    } else {
        (locals.var_ey, locals.var_ey_dn0, locals.var_ey_dn2, locals.var_ey_dn4, locals.var_ey_dn5, locals.var_ey_dn6, locals.var_ey_dn7, locals.var_ey_dn8, locals.var_ey_dn9, locals.var_ey_dn10, locals.var_ey_dn13,)
    }
};
        locals.var_ey = assign34640_e39791;
        locals.var_ey_dn0 = assign34640_e39791_d_n0;
        locals.var_ey_dn2 = assign34640_e39791_d_n2;
        locals.var_ey_dn4 = assign34640_e39791_d_n4;
        locals.var_ey_dn5 = assign34640_e39791_d_n5;
        locals.var_ey_dn6 = assign34640_e39791_d_n6;
        locals.var_ey_dn7 = assign34640_e39791_d_n7;
        locals.var_ey_dn8 = assign34640_e39791_d_n8;
        locals.var_ey_dn9 = assign34640_e39791_d_n9;
        locals.var_ey_dn10 = assign34640_e39791_d_n10;
        locals.var_ey_dn13 = assign34640_e39791_d_n13;
        locals.var_ey_rv = 0.0;

        let (assign34650_e39799, assign34650_e39799_d_n0, assign34650_e39799_d_n2, assign34650_e39799_d_n4, assign34650_e39799_d_n5, assign34650_e39799_d_n6, assign34650_e39799_d_n7, assign34650_e39799_d_n8, assign34650_e39799_d_n9, assign34650_e39799_d_n10, assign34650_e39799_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign34650_e39797: f64 = (1.0 / locals.var_ey);
        (assign34650_e39797, (-(locals.var_ey_dn0 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn2 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn4 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn5 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn6 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn7 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn8 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn9 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn10 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn13 / (locals.var_ey * locals.var_ey))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign34650_e39799;
        locals.var_t4_dn0 = assign34650_e39799_d_n0;
        locals.var_t4_dn2 = assign34650_e39799_d_n2;
        locals.var_t4_dn4 = assign34650_e39799_d_n4;
        locals.var_t4_dn5 = assign34650_e39799_d_n5;
        locals.var_t4_dn6 = assign34650_e39799_d_n6;
        locals.var_t4_dn7 = assign34650_e39799_d_n7;
        locals.var_t4_dn8 = assign34650_e39799_d_n8;
        locals.var_t4_dn9 = assign34650_e39799_d_n9;
        locals.var_t4_dn10 = assign34650_e39799_d_n10;
        locals.var_t4_dn13 = assign34650_e39799_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign34660_e39807, assign34660_e39807_d_n0, assign34660_e39807_d_n2, assign34660_e39807_d_n4, assign34660_e39807_d_n5, assign34660_e39807_d_n6, assign34660_e39807_d_n7, assign34660_e39807_d_n8, assign34660_e39807_d_n9, assign34660_e39807_d_n10, assign34660_e39807_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign34660_e39805: f64 = (locals.var_muun * locals.var_ey);
        (assign34660_e39805, ((locals.var_muun_dn0 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn0)), ((locals.var_muun_dn2 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn2)), ((locals.var_muun_dn4 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn4)), ((locals.var_muun_dn5 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn5)), ((locals.var_muun_dn6 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn6)), ((locals.var_muun_dn7 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn7)), ((locals.var_muun_dn8 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn8)), ((locals.var_muun_dn9 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn9)), ((locals.var_muun_dn10 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn10)), ((locals.var_muun_dn13 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn13)),)
    } else {
        (locals.var_em, locals.var_em_dn0, locals.var_em_dn2, locals.var_em_dn4, locals.var_em_dn5, locals.var_em_dn6, locals.var_em_dn7, locals.var_em_dn8, locals.var_em_dn9, locals.var_em_dn10, locals.var_em_dn13,)
    }
};
        locals.var_em = assign34660_e39807;
        locals.var_em_dn0 = assign34660_e39807_d_n0;
        locals.var_em_dn2 = assign34660_e39807_d_n2;
        locals.var_em_dn4 = assign34660_e39807_d_n4;
        locals.var_em_dn5 = assign34660_e39807_d_n5;
        locals.var_em_dn6 = assign34660_e39807_d_n6;
        locals.var_em_dn7 = assign34660_e39807_d_n7;
        locals.var_em_dn8 = assign34660_e39807_d_n8;
        locals.var_em_dn9 = assign34660_e39807_d_n9;
        locals.var_em_dn10 = assign34660_e39807_d_n10;
        locals.var_em_dn13 = assign34660_e39807_d_n13;
        locals.var_em_rv = 0.0;

        let (assign34670_e39815, assign34670_e39815_d_n0, assign34670_e39815_d_n2, assign34670_e39815_d_n4, assign34670_e39815_d_n5, assign34670_e39815_d_n6, assign34670_e39815_d_n7, assign34670_e39815_d_n8, assign34670_e39815_d_n9, assign34670_e39815_d_n10, assign34670_e39815_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign34670_e39813: f64 = (locals.var_em / locals.var_vmaxe);
        (assign34670_e39813, (((locals.var_em_dn0 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn0)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn2 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn2)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn4 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn4)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn5 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn5)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn6 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn6)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn7 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn7)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn8 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn8)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn9 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn9)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn10 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn10)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn13 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn13)) / (locals.var_vmaxe * locals.var_vmaxe)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign34670_e39815;
        locals.var_t1_dn0 = assign34670_e39815_d_n0;
        locals.var_t1_dn2 = assign34670_e39815_d_n2;
        locals.var_t1_dn4 = assign34670_e39815_d_n4;
        locals.var_t1_dn5 = assign34670_e39815_d_n5;
        locals.var_t1_dn6 = assign34670_e39815_d_n6;
        locals.var_t1_dn7 = assign34670_e39815_d_n7;
        locals.var_t1_dn8 = assign34670_e39815_d_n8;
        locals.var_t1_dn9 = assign34670_e39815_d_n9;
        locals.var_t1_dn10 = assign34670_e39815_d_n10;
        locals.var_t1_dn13 = assign34670_e39815_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign34680_e39821, assign34680_e39821_d_n0, assign34680_e39821_d_n2, assign34680_e39821_d_n4, assign34680_e39821_d_n5, assign34680_e39821_d_n6, assign34680_e39821_d_n7, assign34680_e39821_d_n8, assign34680_e39821_d_n9, assign34680_e39821_d_n10, assign34680_e39821_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        (locals.var_ey, locals.var_ey_dn0, locals.var_ey_dn2, locals.var_ey_dn4, locals.var_ey_dn5, locals.var_ey_dn6, locals.var_ey_dn7, locals.var_ey_dn8, locals.var_ey_dn9, locals.var_ey_dn10, locals.var_ey_dn13,)
    } else {
        (locals.var_ey_suf, locals.var_ey_suf_dn0, locals.var_ey_suf_dn2, locals.var_ey_suf_dn4, locals.var_ey_suf_dn5, locals.var_ey_suf_dn6, locals.var_ey_suf_dn7, locals.var_ey_suf_dn8, locals.var_ey_suf_dn9, locals.var_ey_suf_dn10, locals.var_ey_suf_dn13,)
    }
};
        locals.var_ey_suf = assign34680_e39821;
        locals.var_ey_suf_dn0 = assign34680_e39821_d_n0;
        locals.var_ey_suf_dn2 = assign34680_e39821_d_n2;
        locals.var_ey_suf_dn4 = assign34680_e39821_d_n4;
        locals.var_ey_suf_dn5 = assign34680_e39821_d_n5;
        locals.var_ey_suf_dn6 = assign34680_e39821_d_n6;
        locals.var_ey_suf_dn7 = assign34680_e39821_d_n7;
        locals.var_ey_suf_dn8 = assign34680_e39821_d_n8;
        locals.var_ey_suf_dn9 = assign34680_e39821_d_n9;
        locals.var_ey_suf_dn10 = assign34680_e39821_d_n10;
        locals.var_ey_suf_dn13 = assign34680_e39821_d_n13;
        locals.var_ey_suf_rv = 0.0;

        let assign34690_e39825: f64 = (10.0 * 2.220446049250313e-16);
        let assign34690_e39826: f64 = (1.0 - assign34690_e39825);
        let assign34690_e39833: f64 = (10.0 * 2.220446049250313e-16);
        let assign34690_e39834: f64 = (1.0 + assign34690_e39833);
        let assign34690_e39836: f64 = if ((assign34690_e39826 <= p.p178) && (p.p178 <= assign34690_e39834)) { 1.0 } else { 0.0 };
        locals.var_guard804 = assign34690_e39836;
        locals.var_guard804_rv = 0.0;

        let (assign34700_e39844, assign34700_e39844_d_n0, assign34700_e39844_d_n2, assign34700_e39844_d_n4, assign34700_e39844_d_n5, assign34700_e39844_d_n6, assign34700_e39844_d_n7, assign34700_e39844_d_n8, assign34700_e39844_d_n9, assign34700_e39844_d_n10, assign34700_e39844_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard804 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign34700_e39844;
        locals.var_t3_dn0 = assign34700_e39844_d_n0;
        locals.var_t3_dn2 = assign34700_e39844_d_n2;
        locals.var_t3_dn4 = assign34700_e39844_d_n4;
        locals.var_t3_dn5 = assign34700_e39844_d_n5;
        locals.var_t3_dn6 = assign34700_e39844_d_n6;
        locals.var_t3_dn7 = assign34700_e39844_d_n7;
        locals.var_t3_dn8 = assign34700_e39844_d_n8;
        locals.var_t3_dn9 = assign34700_e39844_d_n9;
        locals.var_t3_dn10 = assign34700_e39844_d_n10;
        locals.var_t3_dn13 = assign34700_e39844_d_n13;
        locals.var_t3_rv = 0.0;

        let assign34710_e39848: f64 = (10.0 * 2.220446049250313e-16);
        let assign34710_e39849: f64 = (2.0 - assign34710_e39848);
        let assign34710_e39856: f64 = (10.0 * 2.220446049250313e-16);
        let assign34710_e39857: f64 = (2.0 + assign34710_e39856);
        let assign34710_e39859: f64 = if ((assign34710_e39849 <= p.p178) && (p.p178 <= assign34710_e39857)) { 1.0 } else { 0.0 };
        locals.var_guard805 = assign34710_e39859;
        locals.var_guard805_rv = 0.0;

        let (assign34720_e39870, assign34720_e39870_d_n0, assign34720_e39870_d_n2, assign34720_e39870_d_n4, assign34720_e39870_d_n5, assign34720_e39870_d_n6, assign34720_e39870_d_n7, assign34720_e39870_d_n8, assign34720_e39870_d_n9, assign34720_e39870_d_n10, assign34720_e39870_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard804 == 0.0)) && (locals.var_guard805 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign34720_e39870;
        locals.var_t3_dn0 = assign34720_e39870_d_n0;
        locals.var_t3_dn2 = assign34720_e39870_d_n2;
        locals.var_t3_dn4 = assign34720_e39870_d_n4;
        locals.var_t3_dn5 = assign34720_e39870_d_n5;
        locals.var_t3_dn6 = assign34720_e39870_d_n6;
        locals.var_t3_dn7 = assign34720_e39870_d_n7;
        locals.var_t3_dn8 = assign34720_e39870_d_n8;
        locals.var_t3_dn9 = assign34720_e39870_d_n9;
        locals.var_t3_dn10 = assign34720_e39870_d_n10;
        locals.var_t3_dn13 = assign34720_e39870_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign34730_e39891, assign34730_e39891_d_n0, assign34730_e39891_d_n2, assign34730_e39891_d_n4, assign34730_e39891_d_n5, assign34730_e39891_d_n6, assign34730_e39891_d_n7, assign34730_e39891_d_n8, assign34730_e39891_d_n9, assign34730_e39891_d_n10, assign34730_e39891_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard804 == 0.0)) && (locals.var_guard805 == 0.0)) {
        let (assign34730_e39889, assign34730_e39889_d_n0, assign34730_e39889_d_n2, assign34730_e39889_d_n4, assign34730_e39889_d_n5, assign34730_e39889_d_n6, assign34730_e39889_d_n7, assign34730_e39889_d_n8, assign34730_e39889_d_n9, assign34730_e39889_d_n10, assign34730_e39889_d_n13,) = {
            if (locals.var_t1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign34730_e39887: f64 = (p.p178 - 1.0);
                let assign34730_e39888: f64 = (locals.var_t1).powf(assign34730_e39887);
                (assign34730_e39888, if 0.0 == 0.0 && ((assign34730_e39887) as f64).is_finite() && ((assign34730_e39887) as f64).fract() == 0.0 { if assign34730_e39887 == 0.0 { 0.0 } else { (assign34730_e39887 * ((locals.var_t1).powf(assign34730_e39887 - 1.0) * locals.var_t1_dn0)) } } else { (assign34730_e39888 * (assign34730_e39887 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign34730_e39887) as f64).is_finite() && ((assign34730_e39887) as f64).fract() == 0.0 { if assign34730_e39887 == 0.0 { 0.0 } else { (assign34730_e39887 * ((locals.var_t1).powf(assign34730_e39887 - 1.0) * locals.var_t1_dn2)) } } else { (assign34730_e39888 * (assign34730_e39887 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign34730_e39887) as f64).is_finite() && ((assign34730_e39887) as f64).fract() == 0.0 { if assign34730_e39887 == 0.0 { 0.0 } else { (assign34730_e39887 * ((locals.var_t1).powf(assign34730_e39887 - 1.0) * locals.var_t1_dn4)) } } else { (assign34730_e39888 * (assign34730_e39887 * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign34730_e39887) as f64).is_finite() && ((assign34730_e39887) as f64).fract() == 0.0 { if assign34730_e39887 == 0.0 { 0.0 } else { (assign34730_e39887 * ((locals.var_t1).powf(assign34730_e39887 - 1.0) * locals.var_t1_dn5)) } } else { (assign34730_e39888 * (assign34730_e39887 * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign34730_e39887) as f64).is_finite() && ((assign34730_e39887) as f64).fract() == 0.0 { if assign34730_e39887 == 0.0 { 0.0 } else { (assign34730_e39887 * ((locals.var_t1).powf(assign34730_e39887 - 1.0) * locals.var_t1_dn6)) } } else { (assign34730_e39888 * (assign34730_e39887 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign34730_e39887) as f64).is_finite() && ((assign34730_e39887) as f64).fract() == 0.0 { if assign34730_e39887 == 0.0 { 0.0 } else { (assign34730_e39887 * ((locals.var_t1).powf(assign34730_e39887 - 1.0) * locals.var_t1_dn7)) } } else { (assign34730_e39888 * (assign34730_e39887 * (locals.var_t1_dn7 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign34730_e39887) as f64).is_finite() && ((assign34730_e39887) as f64).fract() == 0.0 { if assign34730_e39887 == 0.0 { 0.0 } else { (assign34730_e39887 * ((locals.var_t1).powf(assign34730_e39887 - 1.0) * locals.var_t1_dn8)) } } else { (assign34730_e39888 * (assign34730_e39887 * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign34730_e39887) as f64).is_finite() && ((assign34730_e39887) as f64).fract() == 0.0 { if assign34730_e39887 == 0.0 { 0.0 } else { (assign34730_e39887 * ((locals.var_t1).powf(assign34730_e39887 - 1.0) * locals.var_t1_dn9)) } } else { (assign34730_e39888 * (assign34730_e39887 * (locals.var_t1_dn9 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign34730_e39887) as f64).is_finite() && ((assign34730_e39887) as f64).fract() == 0.0 { if assign34730_e39887 == 0.0 { 0.0 } else { (assign34730_e39887 * ((locals.var_t1).powf(assign34730_e39887 - 1.0) * locals.var_t1_dn10)) } } else { (assign34730_e39888 * (assign34730_e39887 * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign34730_e39887) as f64).is_finite() && ((assign34730_e39887) as f64).fract() == 0.0 { if assign34730_e39887 == 0.0 { 0.0 } else { (assign34730_e39887 * ((locals.var_t1).powf(assign34730_e39887 - 1.0) * locals.var_t1_dn13)) } } else { (assign34730_e39888 * (assign34730_e39887 * (locals.var_t1_dn13 / locals.var_t1))) },)
            }
        };
        (assign34730_e39889, assign34730_e39889_d_n0, assign34730_e39889_d_n2, assign34730_e39889_d_n4, assign34730_e39889_d_n5, assign34730_e39889_d_n6, assign34730_e39889_d_n7, assign34730_e39889_d_n8, assign34730_e39889_d_n9, assign34730_e39889_d_n10, assign34730_e39889_d_n13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign34730_e39891;
        locals.var_t3_dn0 = assign34730_e39891_d_n0;
        locals.var_t3_dn2 = assign34730_e39891_d_n2;
        locals.var_t3_dn4 = assign34730_e39891_d_n4;
        locals.var_t3_dn5 = assign34730_e39891_d_n5;
        locals.var_t3_dn6 = assign34730_e39891_d_n6;
        locals.var_t3_dn7 = assign34730_e39891_d_n7;
        locals.var_t3_dn8 = assign34730_e39891_d_n8;
        locals.var_t3_dn9 = assign34730_e39891_d_n9;
        locals.var_t3_dn10 = assign34730_e39891_d_n10;
        locals.var_t3_dn13 = assign34730_e39891_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign34740_e39899, assign34740_e39899_d_n0, assign34740_e39899_d_n2, assign34740_e39899_d_n4, assign34740_e39899_d_n5, assign34740_e39899_d_n6, assign34740_e39899_d_n7, assign34740_e39899_d_n8, assign34740_e39899_d_n9, assign34740_e39899_d_n10, assign34740_e39899_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign34740_e39897: f64 = (locals.var_t1 * locals.var_t3);
        (assign34740_e39897, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)), ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)), ((locals.var_t1_dn9 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn9)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn13 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign34740_e39899;
        locals.var_t2_dn0 = assign34740_e39899_d_n0;
        locals.var_t2_dn2 = assign34740_e39899_d_n2;
        locals.var_t2_dn4 = assign34740_e39899_d_n4;
        locals.var_t2_dn5 = assign34740_e39899_d_n5;
        locals.var_t2_dn6 = assign34740_e39899_d_n6;
        locals.var_t2_dn7 = assign34740_e39899_d_n7;
        locals.var_t2_dn8 = assign34740_e39899_d_n8;
        locals.var_t2_dn9 = assign34740_e39899_d_n9;
        locals.var_t2_dn10 = assign34740_e39899_d_n10;
        locals.var_t2_dn13 = assign34740_e39899_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign34750_e39907, assign34750_e39907_d_n0, assign34750_e39907_d_n2, assign34750_e39907_d_n4, assign34750_e39907_d_n5, assign34750_e39907_d_n6, assign34750_e39907_d_n7, assign34750_e39907_d_n8, assign34750_e39907_d_n9, assign34750_e39907_d_n10, assign34750_e39907_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign34750_e39905: f64 = (1.0 + locals.var_t2);
        (assign34750_e39905, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign34750_e39907;
        locals.var_t4_dn0 = assign34750_e39907_d_n0;
        locals.var_t4_dn2 = assign34750_e39907_d_n2;
        locals.var_t4_dn4 = assign34750_e39907_d_n4;
        locals.var_t4_dn5 = assign34750_e39907_d_n5;
        locals.var_t4_dn6 = assign34750_e39907_d_n6;
        locals.var_t4_dn7 = assign34750_e39907_d_n7;
        locals.var_t4_dn8 = assign34750_e39907_d_n8;
        locals.var_t4_dn9 = assign34750_e39907_d_n9;
        locals.var_t4_dn10 = assign34750_e39907_d_n10;
        locals.var_t4_dn13 = assign34750_e39907_d_n13;
        locals.var_t4_rv = 0.0;

        let assign34760_e39911: f64 = (10.0 * 2.220446049250313e-16);
        let assign34760_e39912: f64 = (1.0 - assign34760_e39911);
        let assign34760_e39919: f64 = (10.0 * 2.220446049250313e-16);
        let assign34760_e39920: f64 = (1.0 + assign34760_e39919);
        let assign34760_e39922: f64 = if ((assign34760_e39912 <= p.p178) && (p.p178 <= assign34760_e39920)) { 1.0 } else { 0.0 };
        locals.var_guard806 = assign34760_e39922;
        locals.var_guard806_rv = 0.0;

        let (assign34770_e39932, assign34770_e39932_d_n0, assign34770_e39932_d_n2, assign34770_e39932_d_n4, assign34770_e39932_d_n5, assign34770_e39932_d_n6, assign34770_e39932_d_n7, assign34770_e39932_d_n8, assign34770_e39932_d_n9, assign34770_e39932_d_n10, assign34770_e39932_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard806 != 0.0)) {
        let assign34770_e39930: f64 = (1.0 / locals.var_t4);
        (assign34770_e39930, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn13 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign34770_e39932;
        locals.var_t5_dn0 = assign34770_e39932_d_n0;
        locals.var_t5_dn2 = assign34770_e39932_d_n2;
        locals.var_t5_dn4 = assign34770_e39932_d_n4;
        locals.var_t5_dn5 = assign34770_e39932_d_n5;
        locals.var_t5_dn6 = assign34770_e39932_d_n6;
        locals.var_t5_dn7 = assign34770_e39932_d_n7;
        locals.var_t5_dn8 = assign34770_e39932_d_n8;
        locals.var_t5_dn9 = assign34770_e39932_d_n9;
        locals.var_t5_dn10 = assign34770_e39932_d_n10;
        locals.var_t5_dn13 = assign34770_e39932_d_n13;
        locals.var_t5_rv = 0.0;

        let assign34780_e39936: f64 = (10.0 * 2.220446049250313e-16);
        let assign34780_e39937: f64 = (2.0 - assign34780_e39936);
        let assign34780_e39944: f64 = (10.0 * 2.220446049250313e-16);
        let assign34780_e39945: f64 = (2.0 + assign34780_e39944);
        let assign34780_e39947: f64 = if ((assign34780_e39937 <= p.p178) && (p.p178 <= assign34780_e39945)) { 1.0 } else { 0.0 };
        locals.var_guard807 = assign34780_e39947;
        locals.var_guard807_rv = 0.0;

        let (assign34790_e39961, assign34790_e39961_d_n0, assign34790_e39961_d_n2, assign34790_e39961_d_n4, assign34790_e39961_d_n5, assign34790_e39961_d_n6, assign34790_e39961_d_n7, assign34790_e39961_d_n8, assign34790_e39961_d_n9, assign34790_e39961_d_n10, assign34790_e39961_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard806 == 0.0)) && (locals.var_guard807 != 0.0)) {
        let assign34790_e39958: f64 = (locals.var_t4).sqrt();
        let assign34790_e39959: f64 = (1.0 / assign34790_e39958);
        (assign34790_e39959, (-((locals.var_t4_dn0 / (2.0 * assign34790_e39958)) / (assign34790_e39958 * assign34790_e39958))), (-((locals.var_t4_dn2 / (2.0 * assign34790_e39958)) / (assign34790_e39958 * assign34790_e39958))), (-((locals.var_t4_dn4 / (2.0 * assign34790_e39958)) / (assign34790_e39958 * assign34790_e39958))), (-((locals.var_t4_dn5 / (2.0 * assign34790_e39958)) / (assign34790_e39958 * assign34790_e39958))), (-((locals.var_t4_dn6 / (2.0 * assign34790_e39958)) / (assign34790_e39958 * assign34790_e39958))), (-((locals.var_t4_dn7 / (2.0 * assign34790_e39958)) / (assign34790_e39958 * assign34790_e39958))), (-((locals.var_t4_dn8 / (2.0 * assign34790_e39958)) / (assign34790_e39958 * assign34790_e39958))), (-((locals.var_t4_dn9 / (2.0 * assign34790_e39958)) / (assign34790_e39958 * assign34790_e39958))), (-((locals.var_t4_dn10 / (2.0 * assign34790_e39958)) / (assign34790_e39958 * assign34790_e39958))), (-((locals.var_t4_dn13 / (2.0 * assign34790_e39958)) / (assign34790_e39958 * assign34790_e39958))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign34790_e39961;
        locals.var_t5_dn0 = assign34790_e39961_d_n0;
        locals.var_t5_dn2 = assign34790_e39961_d_n2;
        locals.var_t5_dn4 = assign34790_e39961_d_n4;
        locals.var_t5_dn5 = assign34790_e39961_d_n5;
        locals.var_t5_dn6 = assign34790_e39961_d_n6;
        locals.var_t5_dn7 = assign34790_e39961_d_n7;
        locals.var_t5_dn8 = assign34790_e39961_d_n8;
        locals.var_t5_dn9 = assign34790_e39961_d_n9;
        locals.var_t5_dn10 = assign34790_e39961_d_n10;
        locals.var_t5_dn13 = assign34790_e39961_d_n13;
        locals.var_t5_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_109(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign34800_e39985, assign34800_e39985_d_n0, assign34800_e39985_d_n2, assign34800_e39985_d_n4, assign34800_e39985_d_n5, assign34800_e39985_d_n6, assign34800_e39985_d_n7, assign34800_e39985_d_n8, assign34800_e39985_d_n9, assign34800_e39985_d_n10, assign34800_e39985_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard806 == 0.0)) && (locals.var_guard807 == 0.0)) {
        let (assign34800_e39983, assign34800_e39983_d_n0, assign34800_e39983_d_n2, assign34800_e39983_d_n4, assign34800_e39983_d_n5, assign34800_e39983_d_n6, assign34800_e39983_d_n7, assign34800_e39983_d_n8, assign34800_e39983_d_n9, assign34800_e39983_d_n10, assign34800_e39983_d_n13,) = {
            if (locals.var_t4 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign34800_e39977: f64 = (-1.0);
                let assign34800_e39979: f64 = (assign34800_e39977 / p.p178);
                let assign34800_e39981: f64 = (assign34800_e39979 - 1.0);
                let assign34800_e39982: f64 = (locals.var_t4).powf(assign34800_e39981);
                (assign34800_e39982, if 0.0 == 0.0 && ((assign34800_e39981) as f64).is_finite() && ((assign34800_e39981) as f64).fract() == 0.0 { if assign34800_e39981 == 0.0 { 0.0 } else { (assign34800_e39981 * ((locals.var_t4).powf(assign34800_e39981 - 1.0) * locals.var_t4_dn0)) } } else { (assign34800_e39982 * (assign34800_e39981 * (locals.var_t4_dn0 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign34800_e39981) as f64).is_finite() && ((assign34800_e39981) as f64).fract() == 0.0 { if assign34800_e39981 == 0.0 { 0.0 } else { (assign34800_e39981 * ((locals.var_t4).powf(assign34800_e39981 - 1.0) * locals.var_t4_dn2)) } } else { (assign34800_e39982 * (assign34800_e39981 * (locals.var_t4_dn2 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign34800_e39981) as f64).is_finite() && ((assign34800_e39981) as f64).fract() == 0.0 { if assign34800_e39981 == 0.0 { 0.0 } else { (assign34800_e39981 * ((locals.var_t4).powf(assign34800_e39981 - 1.0) * locals.var_t4_dn4)) } } else { (assign34800_e39982 * (assign34800_e39981 * (locals.var_t4_dn4 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign34800_e39981) as f64).is_finite() && ((assign34800_e39981) as f64).fract() == 0.0 { if assign34800_e39981 == 0.0 { 0.0 } else { (assign34800_e39981 * ((locals.var_t4).powf(assign34800_e39981 - 1.0) * locals.var_t4_dn5)) } } else { (assign34800_e39982 * (assign34800_e39981 * (locals.var_t4_dn5 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign34800_e39981) as f64).is_finite() && ((assign34800_e39981) as f64).fract() == 0.0 { if assign34800_e39981 == 0.0 { 0.0 } else { (assign34800_e39981 * ((locals.var_t4).powf(assign34800_e39981 - 1.0) * locals.var_t4_dn6)) } } else { (assign34800_e39982 * (assign34800_e39981 * (locals.var_t4_dn6 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign34800_e39981) as f64).is_finite() && ((assign34800_e39981) as f64).fract() == 0.0 { if assign34800_e39981 == 0.0 { 0.0 } else { (assign34800_e39981 * ((locals.var_t4).powf(assign34800_e39981 - 1.0) * locals.var_t4_dn7)) } } else { (assign34800_e39982 * (assign34800_e39981 * (locals.var_t4_dn7 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign34800_e39981) as f64).is_finite() && ((assign34800_e39981) as f64).fract() == 0.0 { if assign34800_e39981 == 0.0 { 0.0 } else { (assign34800_e39981 * ((locals.var_t4).powf(assign34800_e39981 - 1.0) * locals.var_t4_dn8)) } } else { (assign34800_e39982 * (assign34800_e39981 * (locals.var_t4_dn8 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign34800_e39981) as f64).is_finite() && ((assign34800_e39981) as f64).fract() == 0.0 { if assign34800_e39981 == 0.0 { 0.0 } else { (assign34800_e39981 * ((locals.var_t4).powf(assign34800_e39981 - 1.0) * locals.var_t4_dn9)) } } else { (assign34800_e39982 * (assign34800_e39981 * (locals.var_t4_dn9 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign34800_e39981) as f64).is_finite() && ((assign34800_e39981) as f64).fract() == 0.0 { if assign34800_e39981 == 0.0 { 0.0 } else { (assign34800_e39981 * ((locals.var_t4).powf(assign34800_e39981 - 1.0) * locals.var_t4_dn10)) } } else { (assign34800_e39982 * (assign34800_e39981 * (locals.var_t4_dn10 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign34800_e39981) as f64).is_finite() && ((assign34800_e39981) as f64).fract() == 0.0 { if assign34800_e39981 == 0.0 { 0.0 } else { (assign34800_e39981 * ((locals.var_t4).powf(assign34800_e39981 - 1.0) * locals.var_t4_dn13)) } } else { (assign34800_e39982 * (assign34800_e39981 * (locals.var_t4_dn13 / locals.var_t4))) },)
            }
        };
        (assign34800_e39983, assign34800_e39983_d_n0, assign34800_e39983_d_n2, assign34800_e39983_d_n4, assign34800_e39983_d_n5, assign34800_e39983_d_n6, assign34800_e39983_d_n7, assign34800_e39983_d_n8, assign34800_e39983_d_n9, assign34800_e39983_d_n10, assign34800_e39983_d_n13,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign34800_e39985;
        locals.var_t6_dn0 = assign34800_e39985_d_n0;
        locals.var_t6_dn2 = assign34800_e39985_d_n2;
        locals.var_t6_dn4 = assign34800_e39985_d_n4;
        locals.var_t6_dn5 = assign34800_e39985_d_n5;
        locals.var_t6_dn6 = assign34800_e39985_d_n6;
        locals.var_t6_dn7 = assign34800_e39985_d_n7;
        locals.var_t6_dn8 = assign34800_e39985_d_n8;
        locals.var_t6_dn9 = assign34800_e39985_d_n9;
        locals.var_t6_dn10 = assign34800_e39985_d_n10;
        locals.var_t6_dn13 = assign34800_e39985_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign34810_e39999, assign34810_e39999_d_n0, assign34810_e39999_d_n2, assign34810_e39999_d_n4, assign34810_e39999_d_n5, assign34810_e39999_d_n6, assign34810_e39999_d_n7, assign34810_e39999_d_n8, assign34810_e39999_d_n9, assign34810_e39999_d_n10, assign34810_e39999_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard806 == 0.0)) && (locals.var_guard807 == 0.0)) {
        let assign34810_e39997: f64 = (locals.var_t4 * locals.var_t6);
        (assign34810_e39997, ((locals.var_t4_dn0 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn0)), ((locals.var_t4_dn2 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn2)), ((locals.var_t4_dn4 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn4)), ((locals.var_t4_dn5 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn5)), ((locals.var_t4_dn6 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn6)), ((locals.var_t4_dn7 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn7)), ((locals.var_t4_dn8 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn8)), ((locals.var_t4_dn9 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn9)), ((locals.var_t4_dn10 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn10)), ((locals.var_t4_dn13 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn13)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign34810_e39999;
        locals.var_t5_dn0 = assign34810_e39999_d_n0;
        locals.var_t5_dn2 = assign34810_e39999_d_n2;
        locals.var_t5_dn4 = assign34810_e39999_d_n4;
        locals.var_t5_dn5 = assign34810_e39999_d_n5;
        locals.var_t5_dn6 = assign34810_e39999_d_n6;
        locals.var_t5_dn7 = assign34810_e39999_d_n7;
        locals.var_t5_dn8 = assign34810_e39999_d_n8;
        locals.var_t5_dn9 = assign34810_e39999_d_n9;
        locals.var_t5_dn10 = assign34810_e39999_d_n10;
        locals.var_t5_dn13 = assign34810_e39999_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign34820_e40007, assign34820_e40007_d_n0, assign34820_e40007_d_n2, assign34820_e40007_d_n4, assign34820_e40007_d_n5, assign34820_e40007_d_n6, assign34820_e40007_d_n7, assign34820_e40007_d_n8, assign34820_e40007_d_n9, assign34820_e40007_d_n10, assign34820_e40007_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign34820_e40005: f64 = (locals.var_muun * locals.var_t5);
        (assign34820_e40005, ((locals.var_muun_dn0 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn0)), ((locals.var_muun_dn2 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn2)), ((locals.var_muun_dn4 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn4)), ((locals.var_muun_dn5 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn5)), ((locals.var_muun_dn6 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn6)), ((locals.var_muun_dn7 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn7)), ((locals.var_muun_dn8 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn8)), ((locals.var_muun_dn9 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn9)), ((locals.var_muun_dn10 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn10)), ((locals.var_muun_dn13 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn13)),)
    } else {
        (locals.var_mu, locals.var_mu_dn0, locals.var_mu_dn2, locals.var_mu_dn4, locals.var_mu_dn5, locals.var_mu_dn6, locals.var_mu_dn7, locals.var_mu_dn8, locals.var_mu_dn9, locals.var_mu_dn10, locals.var_mu_dn13,)
    }
};
        locals.var_mu = assign34820_e40007;
        locals.var_mu_dn0 = assign34820_e40007_d_n0;
        locals.var_mu_dn2 = assign34820_e40007_d_n2;
        locals.var_mu_dn4 = assign34820_e40007_d_n4;
        locals.var_mu_dn5 = assign34820_e40007_d_n5;
        locals.var_mu_dn6 = assign34820_e40007_d_n6;
        locals.var_mu_dn7 = assign34820_e40007_d_n7;
        locals.var_mu_dn8 = assign34820_e40007_d_n8;
        locals.var_mu_dn9 = assign34820_e40007_d_n9;
        locals.var_mu_dn10 = assign34820_e40007_d_n10;
        locals.var_mu_dn13 = assign34820_e40007_d_n13;
        locals.var_mu_rv = 0.0;

        let assign34830_e40010: f64 = if locals.var_vdsorg > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard808 = assign34830_e40010;
        locals.var_guard808_rv = 0.0;

        let (assign34840_e40022, assign34840_e40022_d_n0, assign34840_e40022_d_n2, assign34840_e40022_d_n4, assign34840_e40022_d_n5, assign34840_e40022_d_n6, assign34840_e40022_d_n7, assign34840_e40022_d_n8, assign34840_e40022_d_n9, assign34840_e40022_d_n10, assign34840_e40022_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) {
        let assign34840_e40019: f64 = (locals.var_cox * locals.var_cox);
        let assign34840_e40020: f64 = (locals.var_q_ndepm_esi / assign34840_e40019);
        (assign34840_e40020, (((locals.var_q_ndepm_esi_dn0 * assign34840_e40019) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn0 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn0)))) / (assign34840_e40019 * assign34840_e40019)), (((locals.var_q_ndepm_esi_dn2 * assign34840_e40019) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn2 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn2)))) / (assign34840_e40019 * assign34840_e40019)), (((locals.var_q_ndepm_esi_dn4 * assign34840_e40019) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn4 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn4)))) / (assign34840_e40019 * assign34840_e40019)), (((locals.var_q_ndepm_esi_dn5 * assign34840_e40019) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn5 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn5)))) / (assign34840_e40019 * assign34840_e40019)), (((locals.var_q_ndepm_esi_dn6 * assign34840_e40019) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn6 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn6)))) / (assign34840_e40019 * assign34840_e40019)), (((locals.var_q_ndepm_esi_dn7 * assign34840_e40019) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn7 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn7)))) / (assign34840_e40019 * assign34840_e40019)), (((locals.var_q_ndepm_esi_dn8 * assign34840_e40019) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn8 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn8)))) / (assign34840_e40019 * assign34840_e40019)), (((locals.var_q_ndepm_esi_dn9 * assign34840_e40019) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn9 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn9)))) / (assign34840_e40019 * assign34840_e40019)), (((locals.var_q_ndepm_esi_dn10 * assign34840_e40019) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn10 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn10)))) / (assign34840_e40019 * assign34840_e40019)), (((locals.var_q_ndepm_esi_dn13 * assign34840_e40019) - (locals.var_q_ndepm_esi * ((locals.var_cox_dn13 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn13)))) / (assign34840_e40019 * assign34840_e40019)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign34840_e40022;
        locals.var_t2_dn0 = assign34840_e40022_d_n0;
        locals.var_t2_dn2 = assign34840_e40022_d_n2;
        locals.var_t2_dn4 = assign34840_e40022_d_n4;
        locals.var_t2_dn5 = assign34840_e40022_d_n5;
        locals.var_t2_dn6 = assign34840_e40022_d_n6;
        locals.var_t2_dn7 = assign34840_e40022_d_n7;
        locals.var_t2_dn8 = assign34840_e40022_d_n8;
        locals.var_t2_dn9 = assign34840_e40022_d_n9;
        locals.var_t2_dn10 = assign34840_e40022_d_n10;
        locals.var_t2_dn13 = assign34840_e40022_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign34850_e40036, assign34850_e40036_d_n0, assign34850_e40036_d_n2, assign34850_e40036_d_n4, assign34850_e40036_d_n5, assign34850_e40036_d_n6, assign34850_e40036_d_n7, assign34850_e40036_d_n8, assign34850_e40036_d_n9, assign34850_e40036_d_n10, assign34850_e40036_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) {
        let assign34850_e40030: f64 = (locals.var_vgp + locals.var_uc_depvdsef1);
        let assign34850_e40032: f64 = (assign34850_e40030 - locals.var_beta_inv);
        let assign34850_e40034: f64 = (assign34850_e40032 - locals.var_vbsz__blk438);
        (assign34850_e40034, (((locals.var_vgp_dn0 + locals.var_uc_depvdsef1_dn0) - locals.var_beta_inv_dn0) - locals.var_vbsz__blk438_dn0), (((locals.var_vgp_dn2 + locals.var_uc_depvdsef1_dn2) - locals.var_beta_inv_dn2) - locals.var_vbsz__blk438_dn2), (((locals.var_vgp_dn4 + locals.var_uc_depvdsef1_dn4) - locals.var_beta_inv_dn4) - locals.var_vbsz__blk438_dn4), (((locals.var_vgp_dn5 + locals.var_uc_depvdsef1_dn5) - locals.var_beta_inv_dn5) - locals.var_vbsz__blk438_dn5), (((locals.var_vgp_dn6 + locals.var_uc_depvdsef1_dn6) - locals.var_beta_inv_dn6) - locals.var_vbsz__blk438_dn6), (((locals.var_vgp_dn7 + locals.var_uc_depvdsef1_dn7) - locals.var_beta_inv_dn7) - locals.var_vbsz__blk438_dn7), (((locals.var_vgp_dn8 + locals.var_uc_depvdsef1_dn8) - locals.var_beta_inv_dn8) - locals.var_vbsz__blk438_dn8), (((locals.var_vgp_dn9 + locals.var_uc_depvdsef1_dn9) - locals.var_beta_inv_dn9) - locals.var_vbsz__blk438_dn9), (((locals.var_vgp_dn10 + locals.var_uc_depvdsef1_dn10) - locals.var_beta_inv_dn10) - locals.var_vbsz__blk438_dn10), (((locals.var_vgp_dn13 + locals.var_uc_depvdsef1_dn13) - locals.var_beta_inv_dn13) - locals.var_vbsz__blk438_dn13),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign34850_e40036;
        locals.var_t0_dn0 = assign34850_e40036_d_n0;
        locals.var_t0_dn2 = assign34850_e40036_d_n2;
        locals.var_t0_dn4 = assign34850_e40036_d_n4;
        locals.var_t0_dn5 = assign34850_e40036_d_n5;
        locals.var_t0_dn6 = assign34850_e40036_d_n6;
        locals.var_t0_dn7 = assign34850_e40036_d_n7;
        locals.var_t0_dn8 = assign34850_e40036_d_n8;
        locals.var_t0_dn9 = assign34850_e40036_d_n9;
        locals.var_t0_dn10 = assign34850_e40036_d_n10;
        locals.var_t0_dn13 = assign34850_e40036_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign34860_e40050, assign34860_e40050_d_n0, assign34860_e40050_d_n2, assign34860_e40050_d_n4, assign34860_e40050_d_n5, assign34860_e40050_d_n6, assign34860_e40050_d_n7, assign34860_e40050_d_n8, assign34860_e40050_d_n9, assign34860_e40050_d_n10, assign34860_e40050_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) {
        let assign34860_e40045: f64 = (2.0 / locals.var_t2);
        let assign34860_e40047: f64 = (assign34860_e40045 * locals.var_t0);
        let assign34860_e40048: f64 = (1.0 + assign34860_e40047);
        (assign34860_e40048, (((-((2.0 * locals.var_t2_dn0) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign34860_e40045 * locals.var_t0_dn0)), (((-((2.0 * locals.var_t2_dn2) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign34860_e40045 * locals.var_t0_dn2)), (((-((2.0 * locals.var_t2_dn4) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign34860_e40045 * locals.var_t0_dn4)), (((-((2.0 * locals.var_t2_dn5) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign34860_e40045 * locals.var_t0_dn5)), (((-((2.0 * locals.var_t2_dn6) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign34860_e40045 * locals.var_t0_dn6)), (((-((2.0 * locals.var_t2_dn7) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign34860_e40045 * locals.var_t0_dn7)), (((-((2.0 * locals.var_t2_dn8) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign34860_e40045 * locals.var_t0_dn8)), (((-((2.0 * locals.var_t2_dn9) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign34860_e40045 * locals.var_t0_dn9)), (((-((2.0 * locals.var_t2_dn10) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign34860_e40045 * locals.var_t0_dn10)), (((-((2.0 * locals.var_t2_dn13) / (locals.var_t2 * locals.var_t2))) * locals.var_t0) + (assign34860_e40045 * locals.var_t0_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign34860_e40050;
        locals.var_t4_dn0 = assign34860_e40050_d_n0;
        locals.var_t4_dn2 = assign34860_e40050_d_n2;
        locals.var_t4_dn4 = assign34860_e40050_d_n4;
        locals.var_t4_dn5 = assign34860_e40050_d_n5;
        locals.var_t4_dn6 = assign34860_e40050_d_n6;
        locals.var_t4_dn7 = assign34860_e40050_d_n7;
        locals.var_t4_dn8 = assign34860_e40050_d_n8;
        locals.var_t4_dn9 = assign34860_e40050_d_n9;
        locals.var_t4_dn10 = assign34860_e40050_d_n10;
        locals.var_t4_dn13 = assign34860_e40050_d_n13;
        locals.var_t4_rv = 0.0;

        let assign34870_e40054: f64 = 2.0;
        let assign34870_e40059: f64 = if ((locals.var_t4 < assign34870_e40054) && (2.0 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard809 = assign34870_e40059;
        locals.var_guard809_rv = 0.0;

        let (assign34880_e40073, assign34880_e40073_d_n0, assign34880_e40073_d_n2, assign34880_e40073_d_n4, assign34880_e40073_d_n5, assign34880_e40073_d_n6, assign34880_e40073_d_n7, assign34880_e40073_d_n8, assign34880_e40073_d_n9, assign34880_e40073_d_n10, assign34880_e40073_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard809 != 0.0)) {
        let assign34880_e40069: f64 = 2.0;
        let assign34880_e40071: f64 = (assign34880_e40069 - locals.var_t4);
        (assign34880_e40071, (-locals.var_t4_dn0), (-locals.var_t4_dn2), (-locals.var_t4_dn4), (-locals.var_t4_dn5), (-locals.var_t4_dn6), (-locals.var_t4_dn7), (-locals.var_t4_dn8), (-locals.var_t4_dn9), (-locals.var_t4_dn10), (-locals.var_t4_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign34880_e40073;
        locals.var_tmf1_dn0 = assign34880_e40073_d_n0;
        locals.var_tmf1_dn2 = assign34880_e40073_d_n2;
        locals.var_tmf1_dn4 = assign34880_e40073_d_n4;
        locals.var_tmf1_dn5 = assign34880_e40073_d_n5;
        locals.var_tmf1_dn6 = assign34880_e40073_d_n6;
        locals.var_tmf1_dn7 = assign34880_e40073_d_n7;
        locals.var_tmf1_dn8 = assign34880_e40073_d_n8;
        locals.var_tmf1_dn9 = assign34880_e40073_d_n9;
        locals.var_tmf1_dn10 = assign34880_e40073_d_n10;
        locals.var_tmf1_dn13 = assign34880_e40073_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign34890_e40085, assign34890_e40085_d_n0, assign34890_e40085_d_n2, assign34890_e40085_d_n4, assign34890_e40085_d_n5, assign34890_e40085_d_n6, assign34890_e40085_d_n7, assign34890_e40085_d_n8, assign34890_e40085_d_n9, assign34890_e40085_d_n10, assign34890_e40085_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard809 != 0.0)) {
        let assign34890_e40083: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign34890_e40083, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign34890_e40085;
        locals.var_x2_dn0 = assign34890_e40085_d_n0;
        locals.var_x2_dn2 = assign34890_e40085_d_n2;
        locals.var_x2_dn4 = assign34890_e40085_d_n4;
        locals.var_x2_dn5 = assign34890_e40085_d_n5;
        locals.var_x2_dn6 = assign34890_e40085_d_n6;
        locals.var_x2_dn7 = assign34890_e40085_d_n7;
        locals.var_x2_dn8 = assign34890_e40085_d_n8;
        locals.var_x2_dn9 = assign34890_e40085_d_n9;
        locals.var_x2_dn10 = assign34890_e40085_d_n10;
        locals.var_x2_dn13 = assign34890_e40085_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign34900_e40097, assign34900_e40097_d_n0, assign34900_e40097_d_n2, assign34900_e40097_d_n4, assign34900_e40097_d_n5, assign34900_e40097_d_n6, assign34900_e40097_d_n7, assign34900_e40097_d_n8, assign34900_e40097_d_n9, assign34900_e40097_d_n10, assign34900_e40097_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard809 != 0.0)) {
        let assign34900_e40095: f64 = (2.0 * 2.0);
        (assign34900_e40095, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign34900_e40097;
        locals.var_xmax2_dn0 = assign34900_e40097_d_n0;
        locals.var_xmax2_dn2 = assign34900_e40097_d_n2;
        locals.var_xmax2_dn4 = assign34900_e40097_d_n4;
        locals.var_xmax2_dn5 = assign34900_e40097_d_n5;
        locals.var_xmax2_dn6 = assign34900_e40097_d_n6;
        locals.var_xmax2_dn7 = assign34900_e40097_d_n7;
        locals.var_xmax2_dn8 = assign34900_e40097_d_n8;
        locals.var_xmax2_dn9 = assign34900_e40097_d_n9;
        locals.var_xmax2_dn10 = assign34900_e40097_d_n10;
        locals.var_xmax2_dn13 = assign34900_e40097_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign34910_e40107, assign34910_e40107_d_n0, assign34910_e40107_d_n2, assign34910_e40107_d_n4, assign34910_e40107_d_n5, assign34910_e40107_d_n6, assign34910_e40107_d_n7, assign34910_e40107_d_n8, assign34910_e40107_d_n9, assign34910_e40107_d_n10, assign34910_e40107_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard809 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign34910_e40107;
        locals.var_xp_dn0 = assign34910_e40107_d_n0;
        locals.var_xp_dn2 = assign34910_e40107_d_n2;
        locals.var_xp_dn4 = assign34910_e40107_d_n4;
        locals.var_xp_dn5 = assign34910_e40107_d_n5;
        locals.var_xp_dn6 = assign34910_e40107_d_n6;
        locals.var_xp_dn7 = assign34910_e40107_d_n7;
        locals.var_xp_dn8 = assign34910_e40107_d_n8;
        locals.var_xp_dn9 = assign34910_e40107_d_n9;
        locals.var_xp_dn10 = assign34910_e40107_d_n10;
        locals.var_xp_dn13 = assign34910_e40107_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign34920_e40117, assign34920_e40117_d_n0, assign34920_e40117_d_n2, assign34920_e40117_d_n4, assign34920_e40117_d_n5, assign34920_e40117_d_n6, assign34920_e40117_d_n7, assign34920_e40117_d_n8, assign34920_e40117_d_n9, assign34920_e40117_d_n10, assign34920_e40117_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard809 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign34920_e40117;
        locals.var_xmp_dn0 = assign34920_e40117_d_n0;
        locals.var_xmp_dn2 = assign34920_e40117_d_n2;
        locals.var_xmp_dn4 = assign34920_e40117_d_n4;
        locals.var_xmp_dn5 = assign34920_e40117_d_n5;
        locals.var_xmp_dn6 = assign34920_e40117_d_n6;
        locals.var_xmp_dn7 = assign34920_e40117_d_n7;
        locals.var_xmp_dn8 = assign34920_e40117_d_n8;
        locals.var_xmp_dn9 = assign34920_e40117_d_n9;
        locals.var_xmp_dn10 = assign34920_e40117_d_n10;
        locals.var_xmp_dn13 = assign34920_e40117_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign34930_e40127,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard809 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign34930_e40127;
        locals.var_m0_rv = 0.0;

        let (assign34940_e40137,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard809 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign34940_e40137;
        locals.var_mm_rv = 0.0;

        let (assign34950_e40147, assign34950_e40147_d_n0, assign34950_e40147_d_n2, assign34950_e40147_d_n4, assign34950_e40147_d_n5, assign34950_e40147_d_n6, assign34950_e40147_d_n7, assign34950_e40147_d_n8, assign34950_e40147_d_n9, assign34950_e40147_d_n10, assign34950_e40147_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard809 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign34950_e40147;
        locals.var_arg_dn0 = assign34950_e40147_d_n0;
        locals.var_arg_dn2 = assign34950_e40147_d_n2;
        locals.var_arg_dn4 = assign34950_e40147_d_n4;
        locals.var_arg_dn5 = assign34950_e40147_d_n5;
        locals.var_arg_dn6 = assign34950_e40147_d_n6;
        locals.var_arg_dn7 = assign34950_e40147_d_n7;
        locals.var_arg_dn8 = assign34950_e40147_d_n8;
        locals.var_arg_dn9 = assign34950_e40147_d_n9;
        locals.var_arg_dn10 = assign34950_e40147_d_n10;
        locals.var_arg_dn13 = assign34950_e40147_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign34960_e40157, assign34960_e40157_d_n0, assign34960_e40157_d_n2, assign34960_e40157_d_n4, assign34960_e40157_d_n5, assign34960_e40157_d_n6, assign34960_e40157_d_n7, assign34960_e40157_d_n8, assign34960_e40157_d_n9, assign34960_e40157_d_n10, assign34960_e40157_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard809 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign34960_e40157;
        locals.var_dnm_dn0 = assign34960_e40157_d_n0;
        locals.var_dnm_dn2 = assign34960_e40157_d_n2;
        locals.var_dnm_dn4 = assign34960_e40157_d_n4;
        locals.var_dnm_dn5 = assign34960_e40157_d_n5;
        locals.var_dnm_dn6 = assign34960_e40157_d_n6;
        locals.var_dnm_dn7 = assign34960_e40157_d_n7;
        locals.var_dnm_dn8 = assign34960_e40157_d_n8;
        locals.var_dnm_dn9 = assign34960_e40157_d_n9;
        locals.var_dnm_dn10 = assign34960_e40157_d_n10;
        locals.var_dnm_dn13 = assign34960_e40157_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign34970_e40169, assign34970_e40169_d_n0, assign34970_e40169_d_n2, assign34970_e40169_d_n4, assign34970_e40169_d_n5, assign34970_e40169_d_n6, assign34970_e40169_d_n7, assign34970_e40169_d_n8, assign34970_e40169_d_n9, assign34970_e40169_d_n10, assign34970_e40169_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard809 != 0.0)) {
        let assign34970_e40167: f64 = (locals.var_xp * locals.var_x2);
        (assign34970_e40167, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign34970_e40169;
        locals.var_xp_dn0 = assign34970_e40169_d_n0;
        locals.var_xp_dn2 = assign34970_e40169_d_n2;
        locals.var_xp_dn4 = assign34970_e40169_d_n4;
        locals.var_xp_dn5 = assign34970_e40169_d_n5;
        locals.var_xp_dn6 = assign34970_e40169_d_n6;
        locals.var_xp_dn7 = assign34970_e40169_d_n7;
        locals.var_xp_dn8 = assign34970_e40169_d_n8;
        locals.var_xp_dn9 = assign34970_e40169_d_n9;
        locals.var_xp_dn10 = assign34970_e40169_d_n10;
        locals.var_xp_dn13 = assign34970_e40169_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign34980_e40181, assign34980_e40181_d_n0, assign34980_e40181_d_n2, assign34980_e40181_d_n4, assign34980_e40181_d_n5, assign34980_e40181_d_n6, assign34980_e40181_d_n7, assign34980_e40181_d_n8, assign34980_e40181_d_n9, assign34980_e40181_d_n10, assign34980_e40181_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard809 != 0.0)) {
        let assign34980_e40179: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign34980_e40179, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign34980_e40181;
        locals.var_xmp_dn0 = assign34980_e40181_d_n0;
        locals.var_xmp_dn2 = assign34980_e40181_d_n2;
        locals.var_xmp_dn4 = assign34980_e40181_d_n4;
        locals.var_xmp_dn5 = assign34980_e40181_d_n5;
        locals.var_xmp_dn6 = assign34980_e40181_d_n6;
        locals.var_xmp_dn7 = assign34980_e40181_d_n7;
        locals.var_xmp_dn8 = assign34980_e40181_d_n8;
        locals.var_xmp_dn9 = assign34980_e40181_d_n9;
        locals.var_xmp_dn10 = assign34980_e40181_d_n10;
        locals.var_xmp_dn13 = assign34980_e40181_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign34990_e40193, assign34990_e40193_d_n0, assign34990_e40193_d_n2, assign34990_e40193_d_n4, assign34990_e40193_d_n5, assign34990_e40193_d_n6, assign34990_e40193_d_n7, assign34990_e40193_d_n8, assign34990_e40193_d_n9, assign34990_e40193_d_n10, assign34990_e40193_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard809 != 0.0)) {
        let assign34990_e40191: f64 = (locals.var_xp * locals.var_x2);
        (assign34990_e40191, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign34990_e40193;
        locals.var_xp_dn0 = assign34990_e40193_d_n0;
        locals.var_xp_dn2 = assign34990_e40193_d_n2;
        locals.var_xp_dn4 = assign34990_e40193_d_n4;
        locals.var_xp_dn5 = assign34990_e40193_d_n5;
        locals.var_xp_dn6 = assign34990_e40193_d_n6;
        locals.var_xp_dn7 = assign34990_e40193_d_n7;
        locals.var_xp_dn8 = assign34990_e40193_d_n8;
        locals.var_xp_dn9 = assign34990_e40193_d_n9;
        locals.var_xp_dn10 = assign34990_e40193_d_n10;
        locals.var_xp_dn13 = assign34990_e40193_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign35000_e40205, assign35000_e40205_d_n0, assign35000_e40205_d_n2, assign35000_e40205_d_n4, assign35000_e40205_d_n5, assign35000_e40205_d_n6, assign35000_e40205_d_n7, assign35000_e40205_d_n8, assign35000_e40205_d_n9, assign35000_e40205_d_n10, assign35000_e40205_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard809 != 0.0)) {
        let assign35000_e40203: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign35000_e40203, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign35000_e40205;
        locals.var_xmp_dn0 = assign35000_e40205_d_n0;
        locals.var_xmp_dn2 = assign35000_e40205_d_n2;
        locals.var_xmp_dn4 = assign35000_e40205_d_n4;
        locals.var_xmp_dn5 = assign35000_e40205_d_n5;
        locals.var_xmp_dn6 = assign35000_e40205_d_n6;
        locals.var_xmp_dn7 = assign35000_e40205_d_n7;
        locals.var_xmp_dn8 = assign35000_e40205_d_n8;
        locals.var_xmp_dn9 = assign35000_e40205_d_n9;
        locals.var_xmp_dn10 = assign35000_e40205_d_n10;
        locals.var_xmp_dn13 = assign35000_e40205_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign35010_e40217, assign35010_e40217_d_n0, assign35010_e40217_d_n2, assign35010_e40217_d_n4, assign35010_e40217_d_n5, assign35010_e40217_d_n6, assign35010_e40217_d_n7, assign35010_e40217_d_n8, assign35010_e40217_d_n9, assign35010_e40217_d_n10, assign35010_e40217_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard809 != 0.0)) {
        let assign35010_e40215: f64 = (locals.var_xp + locals.var_xmp);
        (assign35010_e40215, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign35010_e40217;
        locals.var_arg_dn0 = assign35010_e40217_d_n0;
        locals.var_arg_dn2 = assign35010_e40217_d_n2;
        locals.var_arg_dn4 = assign35010_e40217_d_n4;
        locals.var_arg_dn5 = assign35010_e40217_d_n5;
        locals.var_arg_dn6 = assign35010_e40217_d_n6;
        locals.var_arg_dn7 = assign35010_e40217_d_n7;
        locals.var_arg_dn8 = assign35010_e40217_d_n8;
        locals.var_arg_dn9 = assign35010_e40217_d_n9;
        locals.var_arg_dn10 = assign35010_e40217_d_n10;
        locals.var_arg_dn13 = assign35010_e40217_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign35020_e40227, assign35020_e40227_d_n0, assign35020_e40227_d_n2, assign35020_e40227_d_n4, assign35020_e40227_d_n5, assign35020_e40227_d_n6, assign35020_e40227_d_n7, assign35020_e40227_d_n8, assign35020_e40227_d_n9, assign35020_e40227_d_n10, assign35020_e40227_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard809 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign35020_e40227;
        locals.var_dnm_dn0 = assign35020_e40227_d_n0;
        locals.var_dnm_dn2 = assign35020_e40227_d_n2;
        locals.var_dnm_dn4 = assign35020_e40227_d_n4;
        locals.var_dnm_dn5 = assign35020_e40227_d_n5;
        locals.var_dnm_dn6 = assign35020_e40227_d_n6;
        locals.var_dnm_dn7 = assign35020_e40227_d_n7;
        locals.var_dnm_dn8 = assign35020_e40227_d_n8;
        locals.var_dnm_dn9 = assign35020_e40227_d_n9;
        locals.var_dnm_dn10 = assign35020_e40227_d_n10;
        locals.var_dnm_dn13 = assign35020_e40227_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign35030_e40242: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard810 = assign35030_e40242;
        locals.var_guard810_rv = 0.0;

        let assign35040_e40245: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard811 = assign35040_e40245;
        locals.var_guard811_rv = 0.0;

        let (assign35050_e40259,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard809 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard811 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign35050_e40259;
        locals.var_mm_rv = 0.0;

        let assign35060_e40262: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard812 = assign35060_e40262;
        locals.var_guard812_rv = 0.0;

        let (assign35070_e40279,) = {
    if (((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard809 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard811 == 0.0)) && (locals.var_guard812 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign35070_e40279;
        locals.var_mm_rv = 0.0;

        let assign35080_e40282: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard813 = assign35080_e40282;
        locals.var_guard813_rv = 0.0;

        let (assign35090_e40302,) = {
    if ((((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard809 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard811 == 0.0)) && (locals.var_guard812 == 0.0)) && (locals.var_guard813 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign35090_e40302;
        locals.var_mm_rv = 0.0;

        let assign35100_e40305: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard814 = assign35100_e40305;
        locals.var_guard814_rv = 0.0;

        let (assign35110_e40328,) = {
    if (((((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard809 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_guard811 == 0.0)) && (locals.var_guard812 == 0.0)) && (locals.var_guard813 == 0.0)) && (locals.var_guard814 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign35110_e40328;
        locals.var_mm_rv = 0.0;

        let (assign35120_e40340,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard809 != 0.0)) && (locals.var_guard810 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign35120_e40340;
        locals.var_m0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_110(
        locals: &mut StampLocals,
    ) {
        let mut assign35130_loop_guard: usize = 0;
        while {
            let assign35130_cond_e40353: f64 = if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard809 != 0.0)) && (locals.var_guard810 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign35130_cond_e40353 != 0.0
        } {
            assign35130_loop_guard += 1;
            assert!(assign35130_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign35130_body0_e40366, assign35130_body0_e40366_d_n0, assign35130_body0_e40366_d_n2, assign35130_body0_e40366_d_n4, assign35130_body0_e40366_d_n5, assign35130_body0_e40366_d_n6, assign35130_body0_e40366_d_n7, assign35130_body0_e40366_d_n8, assign35130_body0_e40366_d_n9, assign35130_body0_e40366_d_n10, assign35130_body0_e40366_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard809 != 0.0)) && (locals.var_guard810 != 0.0)) {
        let assign35130_body0_e40364: f64 = (locals.var_dnm).sqrt();
        (assign35130_body0_e40364, (locals.var_dnm_dn0 / (2.0 * assign35130_body0_e40364)), (locals.var_dnm_dn2 / (2.0 * assign35130_body0_e40364)), (locals.var_dnm_dn4 / (2.0 * assign35130_body0_e40364)), (locals.var_dnm_dn5 / (2.0 * assign35130_body0_e40364)), (locals.var_dnm_dn6 / (2.0 * assign35130_body0_e40364)), (locals.var_dnm_dn7 / (2.0 * assign35130_body0_e40364)), (locals.var_dnm_dn8 / (2.0 * assign35130_body0_e40364)), (locals.var_dnm_dn9 / (2.0 * assign35130_body0_e40364)), (locals.var_dnm_dn10 / (2.0 * assign35130_body0_e40364)), (locals.var_dnm_dn13 / (2.0 * assign35130_body0_e40364)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign35130_body0_e40366;
            locals.var_dnm_dn0 = assign35130_body0_e40366_d_n0;
            locals.var_dnm_dn2 = assign35130_body0_e40366_d_n2;
            locals.var_dnm_dn4 = assign35130_body0_e40366_d_n4;
            locals.var_dnm_dn5 = assign35130_body0_e40366_d_n5;
            locals.var_dnm_dn6 = assign35130_body0_e40366_d_n6;
            locals.var_dnm_dn7 = assign35130_body0_e40366_d_n7;
            locals.var_dnm_dn8 = assign35130_body0_e40366_d_n8;
            locals.var_dnm_dn9 = assign35130_body0_e40366_d_n9;
            locals.var_dnm_dn10 = assign35130_body0_e40366_d_n10;
            locals.var_dnm_dn13 = assign35130_body0_e40366_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign35130_body1_e40380,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard809 != 0.0)) && (locals.var_guard810 != 0.0)) {
        let assign35130_body1_e40378: f64 = (locals.var_m0 + 1.0);
        (assign35130_body1_e40378,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign35130_body1_e40380;
            locals.var_m0_rv = 0.0;
        }

        let (assign35140_e40404, assign35140_e40404_d_n0, assign35140_e40404_d_n2, assign35140_e40404_d_n4, assign35140_e40404_d_n5, assign35140_e40404_d_n6, assign35140_e40404_d_n7, assign35140_e40404_d_n8, assign35140_e40404_d_n9, assign35140_e40404_d_n10, assign35140_e40404_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard809 != 0.0)) && (locals.var_guard810 == 0.0)) {
        let (assign35140_e40402, assign35140_e40402_d_n0, assign35140_e40402_d_n2, assign35140_e40402_d_n4, assign35140_e40402_d_n5, assign35140_e40402_d_n6, assign35140_e40402_d_n7, assign35140_e40402_d_n8, assign35140_e40402_d_n9, assign35140_e40402_d_n10, assign35140_e40402_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign35140_e40399: f64 = (2.0 * 2.0);
                let assign35140_e40400: f64 = (1.0 / assign35140_e40399);
                let assign35140_e40401: f64 = (locals.var_dnm).powf(assign35140_e40400);
                (assign35140_e40401, if 0.0 == 0.0 && ((assign35140_e40400) as f64).is_finite() && ((assign35140_e40400) as f64).fract() == 0.0 { if assign35140_e40400 == 0.0 { 0.0 } else { (assign35140_e40400 * ((locals.var_dnm).powf(assign35140_e40400 - 1.0) * locals.var_dnm_dn0)) } } else { (assign35140_e40401 * (assign35140_e40400 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35140_e40400) as f64).is_finite() && ((assign35140_e40400) as f64).fract() == 0.0 { if assign35140_e40400 == 0.0 { 0.0 } else { (assign35140_e40400 * ((locals.var_dnm).powf(assign35140_e40400 - 1.0) * locals.var_dnm_dn2)) } } else { (assign35140_e40401 * (assign35140_e40400 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35140_e40400) as f64).is_finite() && ((assign35140_e40400) as f64).fract() == 0.0 { if assign35140_e40400 == 0.0 { 0.0 } else { (assign35140_e40400 * ((locals.var_dnm).powf(assign35140_e40400 - 1.0) * locals.var_dnm_dn4)) } } else { (assign35140_e40401 * (assign35140_e40400 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35140_e40400) as f64).is_finite() && ((assign35140_e40400) as f64).fract() == 0.0 { if assign35140_e40400 == 0.0 { 0.0 } else { (assign35140_e40400 * ((locals.var_dnm).powf(assign35140_e40400 - 1.0) * locals.var_dnm_dn5)) } } else { (assign35140_e40401 * (assign35140_e40400 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35140_e40400) as f64).is_finite() && ((assign35140_e40400) as f64).fract() == 0.0 { if assign35140_e40400 == 0.0 { 0.0 } else { (assign35140_e40400 * ((locals.var_dnm).powf(assign35140_e40400 - 1.0) * locals.var_dnm_dn6)) } } else { (assign35140_e40401 * (assign35140_e40400 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35140_e40400) as f64).is_finite() && ((assign35140_e40400) as f64).fract() == 0.0 { if assign35140_e40400 == 0.0 { 0.0 } else { (assign35140_e40400 * ((locals.var_dnm).powf(assign35140_e40400 - 1.0) * locals.var_dnm_dn7)) } } else { (assign35140_e40401 * (assign35140_e40400 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35140_e40400) as f64).is_finite() && ((assign35140_e40400) as f64).fract() == 0.0 { if assign35140_e40400 == 0.0 { 0.0 } else { (assign35140_e40400 * ((locals.var_dnm).powf(assign35140_e40400 - 1.0) * locals.var_dnm_dn8)) } } else { (assign35140_e40401 * (assign35140_e40400 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35140_e40400) as f64).is_finite() && ((assign35140_e40400) as f64).fract() == 0.0 { if assign35140_e40400 == 0.0 { 0.0 } else { (assign35140_e40400 * ((locals.var_dnm).powf(assign35140_e40400 - 1.0) * locals.var_dnm_dn9)) } } else { (assign35140_e40401 * (assign35140_e40400 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35140_e40400) as f64).is_finite() && ((assign35140_e40400) as f64).fract() == 0.0 { if assign35140_e40400 == 0.0 { 0.0 } else { (assign35140_e40400 * ((locals.var_dnm).powf(assign35140_e40400 - 1.0) * locals.var_dnm_dn10)) } } else { (assign35140_e40401 * (assign35140_e40400 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35140_e40400) as f64).is_finite() && ((assign35140_e40400) as f64).fract() == 0.0 { if assign35140_e40400 == 0.0 { 0.0 } else { (assign35140_e40400 * ((locals.var_dnm).powf(assign35140_e40400 - 1.0) * locals.var_dnm_dn13)) } } else { (assign35140_e40401 * (assign35140_e40400 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign35140_e40402, assign35140_e40402_d_n0, assign35140_e40402_d_n2, assign35140_e40402_d_n4, assign35140_e40402_d_n5, assign35140_e40402_d_n6, assign35140_e40402_d_n7, assign35140_e40402_d_n8, assign35140_e40402_d_n9, assign35140_e40402_d_n10, assign35140_e40402_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign35140_e40404;
        locals.var_dnm_dn0 = assign35140_e40404_d_n0;
        locals.var_dnm_dn2 = assign35140_e40404_d_n2;
        locals.var_dnm_dn4 = assign35140_e40404_d_n4;
        locals.var_dnm_dn5 = assign35140_e40404_d_n5;
        locals.var_dnm_dn6 = assign35140_e40404_d_n6;
        locals.var_dnm_dn7 = assign35140_e40404_d_n7;
        locals.var_dnm_dn8 = assign35140_e40404_d_n8;
        locals.var_dnm_dn9 = assign35140_e40404_d_n9;
        locals.var_dnm_dn10 = assign35140_e40404_d_n10;
        locals.var_dnm_dn13 = assign35140_e40404_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign35150_e40416, assign35150_e40416_d_n0, assign35150_e40416_d_n2, assign35150_e40416_d_n4, assign35150_e40416_d_n5, assign35150_e40416_d_n6, assign35150_e40416_d_n7, assign35150_e40416_d_n8, assign35150_e40416_d_n9, assign35150_e40416_d_n10, assign35150_e40416_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard809 != 0.0)) {
        let assign35150_e40414: f64 = (1.0 / locals.var_dnm);
        (assign35150_e40414, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign35150_e40416;
        locals.var_dnm_dn0 = assign35150_e40416_d_n0;
        locals.var_dnm_dn2 = assign35150_e40416_d_n2;
        locals.var_dnm_dn4 = assign35150_e40416_d_n4;
        locals.var_dnm_dn5 = assign35150_e40416_d_n5;
        locals.var_dnm_dn6 = assign35150_e40416_d_n6;
        locals.var_dnm_dn7 = assign35150_e40416_d_n7;
        locals.var_dnm_dn8 = assign35150_e40416_d_n8;
        locals.var_dnm_dn9 = assign35150_e40416_d_n9;
        locals.var_dnm_dn10 = assign35150_e40416_d_n10;
        locals.var_dnm_dn13 = assign35150_e40416_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign35160_e40430, assign35160_e40430_d_n0, assign35160_e40430_d_n2, assign35160_e40430_d_n4, assign35160_e40430_d_n5, assign35160_e40430_d_n6, assign35160_e40430_d_n7, assign35160_e40430_d_n8, assign35160_e40430_d_n9, assign35160_e40430_d_n10, assign35160_e40430_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard809 != 0.0)) {
        let assign35160_e40426: f64 = (locals.var_tmf1 * 2.0);
        let assign35160_e40428: f64 = (assign35160_e40426 * locals.var_dnm);
        (assign35160_e40428, (((locals.var_tmf1_dn0 * 2.0) * locals.var_dnm) + (assign35160_e40426 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 2.0) * locals.var_dnm) + (assign35160_e40426 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 2.0) * locals.var_dnm) + (assign35160_e40426 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 2.0) * locals.var_dnm) + (assign35160_e40426 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 2.0) * locals.var_dnm) + (assign35160_e40426 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 2.0) * locals.var_dnm) + (assign35160_e40426 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 2.0) * locals.var_dnm) + (assign35160_e40426 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 2.0) * locals.var_dnm) + (assign35160_e40426 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 2.0) * locals.var_dnm) + (assign35160_e40426 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * 2.0) * locals.var_dnm) + (assign35160_e40426 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign35160_e40430;
        locals.var_tmf0_dn0 = assign35160_e40430_d_n0;
        locals.var_tmf0_dn2 = assign35160_e40430_d_n2;
        locals.var_tmf0_dn4 = assign35160_e40430_d_n4;
        locals.var_tmf0_dn5 = assign35160_e40430_d_n5;
        locals.var_tmf0_dn6 = assign35160_e40430_d_n6;
        locals.var_tmf0_dn7 = assign35160_e40430_d_n7;
        locals.var_tmf0_dn8 = assign35160_e40430_d_n8;
        locals.var_tmf0_dn9 = assign35160_e40430_d_n9;
        locals.var_tmf0_dn10 = assign35160_e40430_d_n10;
        locals.var_tmf0_dn13 = assign35160_e40430_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign35170_e40446, assign35170_e40446_d_n0, assign35170_e40446_d_n2, assign35170_e40446_d_n4, assign35170_e40446_d_n5, assign35170_e40446_d_n6, assign35170_e40446_d_n7, assign35170_e40446_d_n8, assign35170_e40446_d_n9, assign35170_e40446_d_n10, assign35170_e40446_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard809 != 0.0)) {
        let assign35170_e40440: f64 = (2.0 * locals.var_xmp);
        let assign35170_e40442: f64 = (assign35170_e40440 * locals.var_dnm);
        let assign35170_e40444: f64 = (assign35170_e40442 / locals.var_arg);
        (assign35170_e40444, ((((((2.0 * locals.var_xmp_dn0) * locals.var_dnm) + (assign35170_e40440 * locals.var_dnm_dn0)) * locals.var_arg) - (assign35170_e40442 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn2) * locals.var_dnm) + (assign35170_e40440 * locals.var_dnm_dn2)) * locals.var_arg) - (assign35170_e40442 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn4) * locals.var_dnm) + (assign35170_e40440 * locals.var_dnm_dn4)) * locals.var_arg) - (assign35170_e40442 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn5) * locals.var_dnm) + (assign35170_e40440 * locals.var_dnm_dn5)) * locals.var_arg) - (assign35170_e40442 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn6) * locals.var_dnm) + (assign35170_e40440 * locals.var_dnm_dn6)) * locals.var_arg) - (assign35170_e40442 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn7) * locals.var_dnm) + (assign35170_e40440 * locals.var_dnm_dn7)) * locals.var_arg) - (assign35170_e40442 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn8) * locals.var_dnm) + (assign35170_e40440 * locals.var_dnm_dn8)) * locals.var_arg) - (assign35170_e40442 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn9) * locals.var_dnm) + (assign35170_e40440 * locals.var_dnm_dn9)) * locals.var_arg) - (assign35170_e40442 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn10) * locals.var_dnm) + (assign35170_e40440 * locals.var_dnm_dn10)) * locals.var_arg) - (assign35170_e40442 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((2.0 * locals.var_xmp_dn13) * locals.var_dnm) + (assign35170_e40440 * locals.var_dnm_dn13)) * locals.var_arg) - (assign35170_e40442 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign35170_e40446;
        locals.var_t0_dn0 = assign35170_e40446_d_n0;
        locals.var_t0_dn2 = assign35170_e40446_d_n2;
        locals.var_t0_dn4 = assign35170_e40446_d_n4;
        locals.var_t0_dn5 = assign35170_e40446_d_n5;
        locals.var_t0_dn6 = assign35170_e40446_d_n6;
        locals.var_t0_dn7 = assign35170_e40446_d_n7;
        locals.var_t0_dn8 = assign35170_e40446_d_n8;
        locals.var_t0_dn9 = assign35170_e40446_d_n9;
        locals.var_t0_dn10 = assign35170_e40446_d_n10;
        locals.var_t0_dn13 = assign35170_e40446_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign35180_e40460, assign35180_e40460_d_n0, assign35180_e40460_d_n2, assign35180_e40460_d_n4, assign35180_e40460_d_n5, assign35180_e40460_d_n6, assign35180_e40460_d_n7, assign35180_e40460_d_n8, assign35180_e40460_d_n9, assign35180_e40460_d_n10, assign35180_e40460_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard809 != 0.0)) {
        let assign35180_e40456: f64 = 2.0;
        let assign35180_e40458: f64 = (assign35180_e40456 - locals.var_tmf0);
        (assign35180_e40458, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn13),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign35180_e40460;
        locals.var_t9_dn0 = assign35180_e40460_d_n0;
        locals.var_t9_dn2 = assign35180_e40460_d_n2;
        locals.var_t9_dn4 = assign35180_e40460_d_n4;
        locals.var_t9_dn5 = assign35180_e40460_d_n5;
        locals.var_t9_dn6 = assign35180_e40460_d_n6;
        locals.var_t9_dn7 = assign35180_e40460_d_n7;
        locals.var_t9_dn8 = assign35180_e40460_d_n8;
        locals.var_t9_dn9 = assign35180_e40460_d_n9;
        locals.var_t9_dn10 = assign35180_e40460_d_n10;
        locals.var_t9_dn13 = assign35180_e40460_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign35190_e40470, assign35190_e40470_d_n0, assign35190_e40470_d_n2, assign35190_e40470_d_n4, assign35190_e40470_d_n5, assign35190_e40470_d_n6, assign35190_e40470_d_n7, assign35190_e40470_d_n8, assign35190_e40470_d_n9, assign35190_e40470_d_n10, assign35190_e40470_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard809 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign35190_e40470;
        locals.var_t0_dn0 = assign35190_e40470_d_n0;
        locals.var_t0_dn2 = assign35190_e40470_d_n2;
        locals.var_t0_dn4 = assign35190_e40470_d_n4;
        locals.var_t0_dn5 = assign35190_e40470_d_n5;
        locals.var_t0_dn6 = assign35190_e40470_d_n6;
        locals.var_t0_dn7 = assign35190_e40470_d_n7;
        locals.var_t0_dn8 = assign35190_e40470_d_n8;
        locals.var_t0_dn9 = assign35190_e40470_d_n9;
        locals.var_t0_dn10 = assign35190_e40470_d_n10;
        locals.var_t0_dn13 = assign35190_e40470_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign35200_e40481, assign35200_e40481_d_n0, assign35200_e40481_d_n2, assign35200_e40481_d_n4, assign35200_e40481_d_n5, assign35200_e40481_d_n6, assign35200_e40481_d_n7, assign35200_e40481_d_n8, assign35200_e40481_d_n9, assign35200_e40481_d_n10, assign35200_e40481_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard809 == 0.0)) {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign35200_e40481;
        locals.var_t9_dn0 = assign35200_e40481_d_n0;
        locals.var_t9_dn2 = assign35200_e40481_d_n2;
        locals.var_t9_dn4 = assign35200_e40481_d_n4;
        locals.var_t9_dn5 = assign35200_e40481_d_n5;
        locals.var_t9_dn6 = assign35200_e40481_d_n6;
        locals.var_t9_dn7 = assign35200_e40481_d_n7;
        locals.var_t9_dn8 = assign35200_e40481_d_n8;
        locals.var_t9_dn9 = assign35200_e40481_d_n9;
        locals.var_t9_dn10 = assign35200_e40481_d_n10;
        locals.var_t9_dn13 = assign35200_e40481_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign35210_e40492, assign35210_e40492_d_n0, assign35210_e40492_d_n2, assign35210_e40492_d_n4, assign35210_e40492_d_n5, assign35210_e40492_d_n6, assign35210_e40492_d_n7, assign35210_e40492_d_n8, assign35210_e40492_d_n9, assign35210_e40492_d_n10, assign35210_e40492_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard809 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign35210_e40492;
        locals.var_t0_dn0 = assign35210_e40492_d_n0;
        locals.var_t0_dn2 = assign35210_e40492_d_n2;
        locals.var_t0_dn4 = assign35210_e40492_d_n4;
        locals.var_t0_dn5 = assign35210_e40492_d_n5;
        locals.var_t0_dn6 = assign35210_e40492_d_n6;
        locals.var_t0_dn7 = assign35210_e40492_d_n7;
        locals.var_t0_dn8 = assign35210_e40492_d_n8;
        locals.var_t0_dn9 = assign35210_e40492_d_n9;
        locals.var_t0_dn10 = assign35210_e40492_d_n10;
        locals.var_t0_dn13 = assign35210_e40492_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign35220_e40502, assign35220_e40502_d_n0, assign35220_e40502_d_n2, assign35220_e40502_d_n4, assign35220_e40502_d_n5, assign35220_e40502_d_n6, assign35220_e40502_d_n7, assign35220_e40502_d_n8, assign35220_e40502_d_n9, assign35220_e40502_d_n10, assign35220_e40502_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) {
        let assign35220_e40500: f64 = (locals.var_t9 + 1e-25);
        (assign35220_e40500, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign35220_e40502;
        locals.var_t9_dn0 = assign35220_e40502_d_n0;
        locals.var_t9_dn2 = assign35220_e40502_d_n2;
        locals.var_t9_dn4 = assign35220_e40502_d_n4;
        locals.var_t9_dn5 = assign35220_e40502_d_n5;
        locals.var_t9_dn6 = assign35220_e40502_d_n6;
        locals.var_t9_dn7 = assign35220_e40502_d_n7;
        locals.var_t9_dn8 = assign35220_e40502_d_n8;
        locals.var_t9_dn9 = assign35220_e40502_d_n9;
        locals.var_t9_dn10 = assign35220_e40502_d_n10;
        locals.var_t9_dn13 = assign35220_e40502_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign35230_e40511, assign35230_e40511_d_n0, assign35230_e40511_d_n2, assign35230_e40511_d_n4, assign35230_e40511_d_n5, assign35230_e40511_d_n6, assign35230_e40511_d_n7, assign35230_e40511_d_n8, assign35230_e40511_d_n9, assign35230_e40511_d_n10, assign35230_e40511_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) {
        let assign35230_e40509: f64 = (locals.var_t9).sqrt();
        (assign35230_e40509, (locals.var_t9_dn0 / (2.0 * assign35230_e40509)), (locals.var_t9_dn2 / (2.0 * assign35230_e40509)), (locals.var_t9_dn4 / (2.0 * assign35230_e40509)), (locals.var_t9_dn5 / (2.0 * assign35230_e40509)), (locals.var_t9_dn6 / (2.0 * assign35230_e40509)), (locals.var_t9_dn7 / (2.0 * assign35230_e40509)), (locals.var_t9_dn8 / (2.0 * assign35230_e40509)), (locals.var_t9_dn9 / (2.0 * assign35230_e40509)), (locals.var_t9_dn10 / (2.0 * assign35230_e40509)), (locals.var_t9_dn13 / (2.0 * assign35230_e40509)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign35230_e40511;
        locals.var_t3_dn0 = assign35230_e40511_d_n0;
        locals.var_t3_dn2 = assign35230_e40511_d_n2;
        locals.var_t3_dn4 = assign35230_e40511_d_n4;
        locals.var_t3_dn5 = assign35230_e40511_d_n5;
        locals.var_t3_dn6 = assign35230_e40511_d_n6;
        locals.var_t3_dn7 = assign35230_e40511_d_n7;
        locals.var_t3_dn8 = assign35230_e40511_d_n8;
        locals.var_t3_dn9 = assign35230_e40511_d_n9;
        locals.var_t3_dn10 = assign35230_e40511_d_n10;
        locals.var_t3_dn13 = assign35230_e40511_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign35240_e40523, assign35240_e40523_d_n0, assign35240_e40523_d_n2, assign35240_e40523_d_n4, assign35240_e40523_d_n5, assign35240_e40523_d_n6, assign35240_e40523_d_n7, assign35240_e40523_d_n8, assign35240_e40523_d_n9, assign35240_e40523_d_n10, assign35240_e40523_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) {
        let assign35240_e40520: f64 = (1.0 - locals.var_t3);
        let assign35240_e40521: f64 = (locals.var_t2 * assign35240_e40520);
        (assign35240_e40521, ((locals.var_t2_dn0 * assign35240_e40520) + (locals.var_t2 * (-locals.var_t3_dn0))), ((locals.var_t2_dn2 * assign35240_e40520) + (locals.var_t2 * (-locals.var_t3_dn2))), ((locals.var_t2_dn4 * assign35240_e40520) + (locals.var_t2 * (-locals.var_t3_dn4))), ((locals.var_t2_dn5 * assign35240_e40520) + (locals.var_t2 * (-locals.var_t3_dn5))), ((locals.var_t2_dn6 * assign35240_e40520) + (locals.var_t2 * (-locals.var_t3_dn6))), ((locals.var_t2_dn7 * assign35240_e40520) + (locals.var_t2 * (-locals.var_t3_dn7))), ((locals.var_t2_dn8 * assign35240_e40520) + (locals.var_t2 * (-locals.var_t3_dn8))), ((locals.var_t2_dn9 * assign35240_e40520) + (locals.var_t2 * (-locals.var_t3_dn9))), ((locals.var_t2_dn10 * assign35240_e40520) + (locals.var_t2 * (-locals.var_t3_dn10))), ((locals.var_t2_dn13 * assign35240_e40520) + (locals.var_t2 * (-locals.var_t3_dn13))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign35240_e40523;
        locals.var_t4_dn0 = assign35240_e40523_d_n0;
        locals.var_t4_dn2 = assign35240_e40523_d_n2;
        locals.var_t4_dn4 = assign35240_e40523_d_n4;
        locals.var_t4_dn5 = assign35240_e40523_d_n5;
        locals.var_t4_dn6 = assign35240_e40523_d_n6;
        locals.var_t4_dn7 = assign35240_e40523_d_n7;
        locals.var_t4_dn8 = assign35240_e40523_d_n8;
        locals.var_t4_dn9 = assign35240_e40523_d_n9;
        locals.var_t4_dn10 = assign35240_e40523_d_n10;
        locals.var_t4_dn13 = assign35240_e40523_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign35250_e40535, assign35250_e40535_d_n0, assign35250_e40535_d_n2, assign35250_e40535_d_n4, assign35250_e40535_d_n5, assign35250_e40535_d_n6, assign35250_e40535_d_n7, assign35250_e40535_d_n8, assign35250_e40535_d_n9, assign35250_e40535_d_n10, assign35250_e40535_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) {
        let assign35250_e40531: f64 = (locals.var_vgp + locals.var_uc_depvdsef1);
        let assign35250_e40533: f64 = (assign35250_e40531 + locals.var_t4);
        (assign35250_e40533, ((locals.var_vgp_dn0 + locals.var_uc_depvdsef1_dn0) + locals.var_t4_dn0), ((locals.var_vgp_dn2 + locals.var_uc_depvdsef1_dn2) + locals.var_t4_dn2), ((locals.var_vgp_dn4 + locals.var_uc_depvdsef1_dn4) + locals.var_t4_dn4), ((locals.var_vgp_dn5 + locals.var_uc_depvdsef1_dn5) + locals.var_t4_dn5), ((locals.var_vgp_dn6 + locals.var_uc_depvdsef1_dn6) + locals.var_t4_dn6), ((locals.var_vgp_dn7 + locals.var_uc_depvdsef1_dn7) + locals.var_t4_dn7), ((locals.var_vgp_dn8 + locals.var_uc_depvdsef1_dn8) + locals.var_t4_dn8), ((locals.var_vgp_dn9 + locals.var_uc_depvdsef1_dn9) + locals.var_t4_dn9), ((locals.var_vgp_dn10 + locals.var_uc_depvdsef1_dn10) + locals.var_t4_dn10), ((locals.var_vgp_dn13 + locals.var_uc_depvdsef1_dn13) + locals.var_t4_dn13),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign35250_e40535;
        locals.var_t10_dn0 = assign35250_e40535_d_n0;
        locals.var_t10_dn2 = assign35250_e40535_d_n2;
        locals.var_t10_dn4 = assign35250_e40535_d_n4;
        locals.var_t10_dn5 = assign35250_e40535_d_n5;
        locals.var_t10_dn6 = assign35250_e40535_d_n6;
        locals.var_t10_dn7 = assign35250_e40535_d_n7;
        locals.var_t10_dn8 = assign35250_e40535_d_n8;
        locals.var_t10_dn9 = assign35250_e40535_d_n9;
        locals.var_t10_dn10 = assign35250_e40535_d_n10;
        locals.var_t10_dn13 = assign35250_e40535_d_n13;
        locals.var_t10_rv = 0.0;

        let (assign35260_e40545, assign35260_e40545_d_n0, assign35260_e40545_d_n2, assign35260_e40545_d_n4, assign35260_e40545_d_n5, assign35260_e40545_d_n6, assign35260_e40545_d_n7, assign35260_e40545_d_n8, assign35260_e40545_d_n9, assign35260_e40545_d_n10, assign35260_e40545_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) {
        let assign35260_e40543: f64 = (locals.var_t10 * locals.var_uc_depvdsef2);
        (assign35260_e40543, ((locals.var_t10_dn0 * locals.var_uc_depvdsef2) + (locals.var_t10 * locals.var_uc_depvdsef2_dn0)), ((locals.var_t10_dn2 * locals.var_uc_depvdsef2) + (locals.var_t10 * locals.var_uc_depvdsef2_dn2)), ((locals.var_t10_dn4 * locals.var_uc_depvdsef2) + (locals.var_t10 * locals.var_uc_depvdsef2_dn4)), ((locals.var_t10_dn5 * locals.var_uc_depvdsef2) + (locals.var_t10 * locals.var_uc_depvdsef2_dn5)), ((locals.var_t10_dn6 * locals.var_uc_depvdsef2) + (locals.var_t10 * locals.var_uc_depvdsef2_dn6)), ((locals.var_t10_dn7 * locals.var_uc_depvdsef2) + (locals.var_t10 * locals.var_uc_depvdsef2_dn7)), ((locals.var_t10_dn8 * locals.var_uc_depvdsef2) + (locals.var_t10 * locals.var_uc_depvdsef2_dn8)), ((locals.var_t10_dn9 * locals.var_uc_depvdsef2) + (locals.var_t10 * locals.var_uc_depvdsef2_dn9)), ((locals.var_t10_dn10 * locals.var_uc_depvdsef2) + (locals.var_t10 * locals.var_uc_depvdsef2_dn10)), ((locals.var_t10_dn13 * locals.var_uc_depvdsef2) + (locals.var_t10 * locals.var_uc_depvdsef2_dn13)),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign35260_e40545;
        locals.var_t10_dn0 = assign35260_e40545_d_n0;
        locals.var_t10_dn2 = assign35260_e40545_d_n2;
        locals.var_t10_dn4 = assign35260_e40545_d_n4;
        locals.var_t10_dn5 = assign35260_e40545_d_n5;
        locals.var_t10_dn6 = assign35260_e40545_d_n6;
        locals.var_t10_dn7 = assign35260_e40545_d_n7;
        locals.var_t10_dn8 = assign35260_e40545_d_n8;
        locals.var_t10_dn9 = assign35260_e40545_d_n9;
        locals.var_t10_dn10 = assign35260_e40545_d_n10;
        locals.var_t10_dn13 = assign35260_e40545_d_n13;
        locals.var_t10_rv = 0.0;

        let assign35270_e40549: f64 = (locals.var_uc_depleak + 4.0);
        let assign35270_e40554: f64 = if ((locals.var_t10 < assign35270_e40549) && (4.0 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard815 = assign35270_e40554;
        locals.var_guard815_rv = 0.0;

        let (assign35280_e40568, assign35280_e40568_d_n0, assign35280_e40568_d_n2, assign35280_e40568_d_n4, assign35280_e40568_d_n5, assign35280_e40568_d_n6, assign35280_e40568_d_n7, assign35280_e40568_d_n8, assign35280_e40568_d_n9, assign35280_e40568_d_n10, assign35280_e40568_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard815 != 0.0)) {
        let assign35280_e40564: f64 = (locals.var_uc_depleak + 4.0);
        let assign35280_e40566: f64 = (assign35280_e40564 - locals.var_t10);
        (assign35280_e40566, (locals.var_uc_depleak_dn0 - locals.var_t10_dn0), (locals.var_uc_depleak_dn2 - locals.var_t10_dn2), (locals.var_uc_depleak_dn4 - locals.var_t10_dn4), (locals.var_uc_depleak_dn5 - locals.var_t10_dn5), (locals.var_uc_depleak_dn6 - locals.var_t10_dn6), (locals.var_uc_depleak_dn7 - locals.var_t10_dn7), (locals.var_uc_depleak_dn8 - locals.var_t10_dn8), (locals.var_uc_depleak_dn9 - locals.var_t10_dn9), (locals.var_uc_depleak_dn10 - locals.var_t10_dn10), (locals.var_uc_depleak_dn13 - locals.var_t10_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign35280_e40568;
        locals.var_tmf1_dn0 = assign35280_e40568_d_n0;
        locals.var_tmf1_dn2 = assign35280_e40568_d_n2;
        locals.var_tmf1_dn4 = assign35280_e40568_d_n4;
        locals.var_tmf1_dn5 = assign35280_e40568_d_n5;
        locals.var_tmf1_dn6 = assign35280_e40568_d_n6;
        locals.var_tmf1_dn7 = assign35280_e40568_d_n7;
        locals.var_tmf1_dn8 = assign35280_e40568_d_n8;
        locals.var_tmf1_dn9 = assign35280_e40568_d_n9;
        locals.var_tmf1_dn10 = assign35280_e40568_d_n10;
        locals.var_tmf1_dn13 = assign35280_e40568_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign35290_e40580, assign35290_e40580_d_n0, assign35290_e40580_d_n2, assign35290_e40580_d_n4, assign35290_e40580_d_n5, assign35290_e40580_d_n6, assign35290_e40580_d_n7, assign35290_e40580_d_n8, assign35290_e40580_d_n9, assign35290_e40580_d_n10, assign35290_e40580_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard815 != 0.0)) {
        let assign35290_e40578: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign35290_e40578, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn13,)
    }
};
        locals.var_x2 = assign35290_e40580;
        locals.var_x2_dn0 = assign35290_e40580_d_n0;
        locals.var_x2_dn2 = assign35290_e40580_d_n2;
        locals.var_x2_dn4 = assign35290_e40580_d_n4;
        locals.var_x2_dn5 = assign35290_e40580_d_n5;
        locals.var_x2_dn6 = assign35290_e40580_d_n6;
        locals.var_x2_dn7 = assign35290_e40580_d_n7;
        locals.var_x2_dn8 = assign35290_e40580_d_n8;
        locals.var_x2_dn9 = assign35290_e40580_d_n9;
        locals.var_x2_dn10 = assign35290_e40580_d_n10;
        locals.var_x2_dn13 = assign35290_e40580_d_n13;
        locals.var_x2_rv = 0.0;

        let (assign35300_e40592, assign35300_e40592_d_n0, assign35300_e40592_d_n2, assign35300_e40592_d_n4, assign35300_e40592_d_n5, assign35300_e40592_d_n6, assign35300_e40592_d_n7, assign35300_e40592_d_n8, assign35300_e40592_d_n9, assign35300_e40592_d_n10, assign35300_e40592_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard815 != 0.0)) {
        let assign35300_e40590: f64 = (4.0 * 4.0);
        (assign35300_e40590, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn13,)
    }
};
        locals.var_xmax2 = assign35300_e40592;
        locals.var_xmax2_dn0 = assign35300_e40592_d_n0;
        locals.var_xmax2_dn2 = assign35300_e40592_d_n2;
        locals.var_xmax2_dn4 = assign35300_e40592_d_n4;
        locals.var_xmax2_dn5 = assign35300_e40592_d_n5;
        locals.var_xmax2_dn6 = assign35300_e40592_d_n6;
        locals.var_xmax2_dn7 = assign35300_e40592_d_n7;
        locals.var_xmax2_dn8 = assign35300_e40592_d_n8;
        locals.var_xmax2_dn9 = assign35300_e40592_d_n9;
        locals.var_xmax2_dn10 = assign35300_e40592_d_n10;
        locals.var_xmax2_dn13 = assign35300_e40592_d_n13;
        locals.var_xmax2_rv = 0.0;

        let (assign35310_e40602, assign35310_e40602_d_n0, assign35310_e40602_d_n2, assign35310_e40602_d_n4, assign35310_e40602_d_n5, assign35310_e40602_d_n6, assign35310_e40602_d_n7, assign35310_e40602_d_n8, assign35310_e40602_d_n9, assign35310_e40602_d_n10, assign35310_e40602_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard815 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign35310_e40602;
        locals.var_xp_dn0 = assign35310_e40602_d_n0;
        locals.var_xp_dn2 = assign35310_e40602_d_n2;
        locals.var_xp_dn4 = assign35310_e40602_d_n4;
        locals.var_xp_dn5 = assign35310_e40602_d_n5;
        locals.var_xp_dn6 = assign35310_e40602_d_n6;
        locals.var_xp_dn7 = assign35310_e40602_d_n7;
        locals.var_xp_dn8 = assign35310_e40602_d_n8;
        locals.var_xp_dn9 = assign35310_e40602_d_n9;
        locals.var_xp_dn10 = assign35310_e40602_d_n10;
        locals.var_xp_dn13 = assign35310_e40602_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign35320_e40612, assign35320_e40612_d_n0, assign35320_e40612_d_n2, assign35320_e40612_d_n4, assign35320_e40612_d_n5, assign35320_e40612_d_n6, assign35320_e40612_d_n7, assign35320_e40612_d_n8, assign35320_e40612_d_n9, assign35320_e40612_d_n10, assign35320_e40612_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard815 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign35320_e40612;
        locals.var_xmp_dn0 = assign35320_e40612_d_n0;
        locals.var_xmp_dn2 = assign35320_e40612_d_n2;
        locals.var_xmp_dn4 = assign35320_e40612_d_n4;
        locals.var_xmp_dn5 = assign35320_e40612_d_n5;
        locals.var_xmp_dn6 = assign35320_e40612_d_n6;
        locals.var_xmp_dn7 = assign35320_e40612_d_n7;
        locals.var_xmp_dn8 = assign35320_e40612_d_n8;
        locals.var_xmp_dn9 = assign35320_e40612_d_n9;
        locals.var_xmp_dn10 = assign35320_e40612_d_n10;
        locals.var_xmp_dn13 = assign35320_e40612_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign35330_e40622,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard815 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign35330_e40622;
        locals.var_m0_rv = 0.0;

        let (assign35340_e40632,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard815 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign35340_e40632;
        locals.var_mm_rv = 0.0;

        let (assign35350_e40642, assign35350_e40642_d_n0, assign35350_e40642_d_n2, assign35350_e40642_d_n4, assign35350_e40642_d_n5, assign35350_e40642_d_n6, assign35350_e40642_d_n7, assign35350_e40642_d_n8, assign35350_e40642_d_n9, assign35350_e40642_d_n10, assign35350_e40642_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard815 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign35350_e40642;
        locals.var_arg_dn0 = assign35350_e40642_d_n0;
        locals.var_arg_dn2 = assign35350_e40642_d_n2;
        locals.var_arg_dn4 = assign35350_e40642_d_n4;
        locals.var_arg_dn5 = assign35350_e40642_d_n5;
        locals.var_arg_dn6 = assign35350_e40642_d_n6;
        locals.var_arg_dn7 = assign35350_e40642_d_n7;
        locals.var_arg_dn8 = assign35350_e40642_d_n8;
        locals.var_arg_dn9 = assign35350_e40642_d_n9;
        locals.var_arg_dn10 = assign35350_e40642_d_n10;
        locals.var_arg_dn13 = assign35350_e40642_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign35360_e40652, assign35360_e40652_d_n0, assign35360_e40652_d_n2, assign35360_e40652_d_n4, assign35360_e40652_d_n5, assign35360_e40652_d_n6, assign35360_e40652_d_n7, assign35360_e40652_d_n8, assign35360_e40652_d_n9, assign35360_e40652_d_n10, assign35360_e40652_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard815 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign35360_e40652;
        locals.var_dnm_dn0 = assign35360_e40652_d_n0;
        locals.var_dnm_dn2 = assign35360_e40652_d_n2;
        locals.var_dnm_dn4 = assign35360_e40652_d_n4;
        locals.var_dnm_dn5 = assign35360_e40652_d_n5;
        locals.var_dnm_dn6 = assign35360_e40652_d_n6;
        locals.var_dnm_dn7 = assign35360_e40652_d_n7;
        locals.var_dnm_dn8 = assign35360_e40652_d_n8;
        locals.var_dnm_dn9 = assign35360_e40652_d_n9;
        locals.var_dnm_dn10 = assign35360_e40652_d_n10;
        locals.var_dnm_dn13 = assign35360_e40652_d_n13;
        locals.var_dnm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_111(
        locals: &mut StampLocals,
    ) {
        let (assign35370_e40664, assign35370_e40664_d_n0, assign35370_e40664_d_n2, assign35370_e40664_d_n4, assign35370_e40664_d_n5, assign35370_e40664_d_n6, assign35370_e40664_d_n7, assign35370_e40664_d_n8, assign35370_e40664_d_n9, assign35370_e40664_d_n10, assign35370_e40664_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard815 != 0.0)) {
        let assign35370_e40662: f64 = (locals.var_xp * locals.var_x2);
        (assign35370_e40662, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign35370_e40664;
        locals.var_xp_dn0 = assign35370_e40664_d_n0;
        locals.var_xp_dn2 = assign35370_e40664_d_n2;
        locals.var_xp_dn4 = assign35370_e40664_d_n4;
        locals.var_xp_dn5 = assign35370_e40664_d_n5;
        locals.var_xp_dn6 = assign35370_e40664_d_n6;
        locals.var_xp_dn7 = assign35370_e40664_d_n7;
        locals.var_xp_dn8 = assign35370_e40664_d_n8;
        locals.var_xp_dn9 = assign35370_e40664_d_n9;
        locals.var_xp_dn10 = assign35370_e40664_d_n10;
        locals.var_xp_dn13 = assign35370_e40664_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign35380_e40676, assign35380_e40676_d_n0, assign35380_e40676_d_n2, assign35380_e40676_d_n4, assign35380_e40676_d_n5, assign35380_e40676_d_n6, assign35380_e40676_d_n7, assign35380_e40676_d_n8, assign35380_e40676_d_n9, assign35380_e40676_d_n10, assign35380_e40676_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard815 != 0.0)) {
        let assign35380_e40674: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign35380_e40674, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign35380_e40676;
        locals.var_xmp_dn0 = assign35380_e40676_d_n0;
        locals.var_xmp_dn2 = assign35380_e40676_d_n2;
        locals.var_xmp_dn4 = assign35380_e40676_d_n4;
        locals.var_xmp_dn5 = assign35380_e40676_d_n5;
        locals.var_xmp_dn6 = assign35380_e40676_d_n6;
        locals.var_xmp_dn7 = assign35380_e40676_d_n7;
        locals.var_xmp_dn8 = assign35380_e40676_d_n8;
        locals.var_xmp_dn9 = assign35380_e40676_d_n9;
        locals.var_xmp_dn10 = assign35380_e40676_d_n10;
        locals.var_xmp_dn13 = assign35380_e40676_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign35390_e40688, assign35390_e40688_d_n0, assign35390_e40688_d_n2, assign35390_e40688_d_n4, assign35390_e40688_d_n5, assign35390_e40688_d_n6, assign35390_e40688_d_n7, assign35390_e40688_d_n8, assign35390_e40688_d_n9, assign35390_e40688_d_n10, assign35390_e40688_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard815 != 0.0)) {
        let assign35390_e40686: f64 = (locals.var_xp * locals.var_x2);
        (assign35390_e40686, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign35390_e40688;
        locals.var_xp_dn0 = assign35390_e40688_d_n0;
        locals.var_xp_dn2 = assign35390_e40688_d_n2;
        locals.var_xp_dn4 = assign35390_e40688_d_n4;
        locals.var_xp_dn5 = assign35390_e40688_d_n5;
        locals.var_xp_dn6 = assign35390_e40688_d_n6;
        locals.var_xp_dn7 = assign35390_e40688_d_n7;
        locals.var_xp_dn8 = assign35390_e40688_d_n8;
        locals.var_xp_dn9 = assign35390_e40688_d_n9;
        locals.var_xp_dn10 = assign35390_e40688_d_n10;
        locals.var_xp_dn13 = assign35390_e40688_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign35400_e40700, assign35400_e40700_d_n0, assign35400_e40700_d_n2, assign35400_e40700_d_n4, assign35400_e40700_d_n5, assign35400_e40700_d_n6, assign35400_e40700_d_n7, assign35400_e40700_d_n8, assign35400_e40700_d_n9, assign35400_e40700_d_n10, assign35400_e40700_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard815 != 0.0)) {
        let assign35400_e40698: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign35400_e40698, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign35400_e40700;
        locals.var_xmp_dn0 = assign35400_e40700_d_n0;
        locals.var_xmp_dn2 = assign35400_e40700_d_n2;
        locals.var_xmp_dn4 = assign35400_e40700_d_n4;
        locals.var_xmp_dn5 = assign35400_e40700_d_n5;
        locals.var_xmp_dn6 = assign35400_e40700_d_n6;
        locals.var_xmp_dn7 = assign35400_e40700_d_n7;
        locals.var_xmp_dn8 = assign35400_e40700_d_n8;
        locals.var_xmp_dn9 = assign35400_e40700_d_n9;
        locals.var_xmp_dn10 = assign35400_e40700_d_n10;
        locals.var_xmp_dn13 = assign35400_e40700_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign35410_e40712, assign35410_e40712_d_n0, assign35410_e40712_d_n2, assign35410_e40712_d_n4, assign35410_e40712_d_n5, assign35410_e40712_d_n6, assign35410_e40712_d_n7, assign35410_e40712_d_n8, assign35410_e40712_d_n9, assign35410_e40712_d_n10, assign35410_e40712_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard815 != 0.0)) {
        let assign35410_e40710: f64 = (locals.var_xp * locals.var_x2);
        (assign35410_e40710, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign35410_e40712;
        locals.var_xp_dn0 = assign35410_e40712_d_n0;
        locals.var_xp_dn2 = assign35410_e40712_d_n2;
        locals.var_xp_dn4 = assign35410_e40712_d_n4;
        locals.var_xp_dn5 = assign35410_e40712_d_n5;
        locals.var_xp_dn6 = assign35410_e40712_d_n6;
        locals.var_xp_dn7 = assign35410_e40712_d_n7;
        locals.var_xp_dn8 = assign35410_e40712_d_n8;
        locals.var_xp_dn9 = assign35410_e40712_d_n9;
        locals.var_xp_dn10 = assign35410_e40712_d_n10;
        locals.var_xp_dn13 = assign35410_e40712_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign35420_e40724, assign35420_e40724_d_n0, assign35420_e40724_d_n2, assign35420_e40724_d_n4, assign35420_e40724_d_n5, assign35420_e40724_d_n6, assign35420_e40724_d_n7, assign35420_e40724_d_n8, assign35420_e40724_d_n9, assign35420_e40724_d_n10, assign35420_e40724_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard815 != 0.0)) {
        let assign35420_e40722: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign35420_e40722, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign35420_e40724;
        locals.var_xmp_dn0 = assign35420_e40724_d_n0;
        locals.var_xmp_dn2 = assign35420_e40724_d_n2;
        locals.var_xmp_dn4 = assign35420_e40724_d_n4;
        locals.var_xmp_dn5 = assign35420_e40724_d_n5;
        locals.var_xmp_dn6 = assign35420_e40724_d_n6;
        locals.var_xmp_dn7 = assign35420_e40724_d_n7;
        locals.var_xmp_dn8 = assign35420_e40724_d_n8;
        locals.var_xmp_dn9 = assign35420_e40724_d_n9;
        locals.var_xmp_dn10 = assign35420_e40724_d_n10;
        locals.var_xmp_dn13 = assign35420_e40724_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign35430_e40736, assign35430_e40736_d_n0, assign35430_e40736_d_n2, assign35430_e40736_d_n4, assign35430_e40736_d_n5, assign35430_e40736_d_n6, assign35430_e40736_d_n7, assign35430_e40736_d_n8, assign35430_e40736_d_n9, assign35430_e40736_d_n10, assign35430_e40736_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard815 != 0.0)) {
        let assign35430_e40734: f64 = (locals.var_xp * locals.var_x2);
        (assign35430_e40734, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn13 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn13)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn13,)
    }
};
        locals.var_xp = assign35430_e40736;
        locals.var_xp_dn0 = assign35430_e40736_d_n0;
        locals.var_xp_dn2 = assign35430_e40736_d_n2;
        locals.var_xp_dn4 = assign35430_e40736_d_n4;
        locals.var_xp_dn5 = assign35430_e40736_d_n5;
        locals.var_xp_dn6 = assign35430_e40736_d_n6;
        locals.var_xp_dn7 = assign35430_e40736_d_n7;
        locals.var_xp_dn8 = assign35430_e40736_d_n8;
        locals.var_xp_dn9 = assign35430_e40736_d_n9;
        locals.var_xp_dn10 = assign35430_e40736_d_n10;
        locals.var_xp_dn13 = assign35430_e40736_d_n13;
        locals.var_xp_rv = 0.0;

        let (assign35440_e40748, assign35440_e40748_d_n0, assign35440_e40748_d_n2, assign35440_e40748_d_n4, assign35440_e40748_d_n5, assign35440_e40748_d_n6, assign35440_e40748_d_n7, assign35440_e40748_d_n8, assign35440_e40748_d_n9, assign35440_e40748_d_n10, assign35440_e40748_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard815 != 0.0)) {
        let assign35440_e40746: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign35440_e40746, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn13 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn13)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn13,)
    }
};
        locals.var_xmp = assign35440_e40748;
        locals.var_xmp_dn0 = assign35440_e40748_d_n0;
        locals.var_xmp_dn2 = assign35440_e40748_d_n2;
        locals.var_xmp_dn4 = assign35440_e40748_d_n4;
        locals.var_xmp_dn5 = assign35440_e40748_d_n5;
        locals.var_xmp_dn6 = assign35440_e40748_d_n6;
        locals.var_xmp_dn7 = assign35440_e40748_d_n7;
        locals.var_xmp_dn8 = assign35440_e40748_d_n8;
        locals.var_xmp_dn9 = assign35440_e40748_d_n9;
        locals.var_xmp_dn10 = assign35440_e40748_d_n10;
        locals.var_xmp_dn13 = assign35440_e40748_d_n13;
        locals.var_xmp_rv = 0.0;

        let (assign35450_e40760, assign35450_e40760_d_n0, assign35450_e40760_d_n2, assign35450_e40760_d_n4, assign35450_e40760_d_n5, assign35450_e40760_d_n6, assign35450_e40760_d_n7, assign35450_e40760_d_n8, assign35450_e40760_d_n9, assign35450_e40760_d_n10, assign35450_e40760_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard815 != 0.0)) {
        let assign35450_e40758: f64 = (locals.var_xp + locals.var_xmp);
        (assign35450_e40758, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn13 + locals.var_xmp_dn13),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    }
};
        locals.var_arg = assign35450_e40760;
        locals.var_arg_dn0 = assign35450_e40760_d_n0;
        locals.var_arg_dn2 = assign35450_e40760_d_n2;
        locals.var_arg_dn4 = assign35450_e40760_d_n4;
        locals.var_arg_dn5 = assign35450_e40760_d_n5;
        locals.var_arg_dn6 = assign35450_e40760_d_n6;
        locals.var_arg_dn7 = assign35450_e40760_d_n7;
        locals.var_arg_dn8 = assign35450_e40760_d_n8;
        locals.var_arg_dn9 = assign35450_e40760_d_n9;
        locals.var_arg_dn10 = assign35450_e40760_d_n10;
        locals.var_arg_dn13 = assign35450_e40760_d_n13;
        locals.var_arg_rv = 0.0;

        let (assign35460_e40770, assign35460_e40770_d_n0, assign35460_e40770_d_n2, assign35460_e40770_d_n4, assign35460_e40770_d_n5, assign35460_e40770_d_n6, assign35460_e40770_d_n7, assign35460_e40770_d_n8, assign35460_e40770_d_n9, assign35460_e40770_d_n10, assign35460_e40770_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard815 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign35460_e40770;
        locals.var_dnm_dn0 = assign35460_e40770_d_n0;
        locals.var_dnm_dn2 = assign35460_e40770_d_n2;
        locals.var_dnm_dn4 = assign35460_e40770_d_n4;
        locals.var_dnm_dn5 = assign35460_e40770_d_n5;
        locals.var_dnm_dn6 = assign35460_e40770_d_n6;
        locals.var_dnm_dn7 = assign35460_e40770_d_n7;
        locals.var_dnm_dn8 = assign35460_e40770_d_n8;
        locals.var_dnm_dn9 = assign35460_e40770_d_n9;
        locals.var_dnm_dn10 = assign35460_e40770_d_n10;
        locals.var_dnm_dn13 = assign35460_e40770_d_n13;
        locals.var_dnm_rv = 0.0;

        let assign35470_e40785: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard816 = assign35470_e40785;
        locals.var_guard816_rv = 0.0;

        let assign35480_e40788: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard817 = assign35480_e40788;
        locals.var_guard817_rv = 0.0;

        let (assign35490_e40802,) = {
    if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard815 != 0.0)) && (locals.var_guard816 != 0.0)) && (locals.var_guard817 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign35490_e40802;
        locals.var_mm_rv = 0.0;

        let assign35500_e40805: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard818 = assign35500_e40805;
        locals.var_guard818_rv = 0.0;

        let (assign35510_e40822,) = {
    if (((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard815 != 0.0)) && (locals.var_guard816 != 0.0)) && (locals.var_guard817 == 0.0)) && (locals.var_guard818 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign35510_e40822;
        locals.var_mm_rv = 0.0;

        let assign35520_e40825: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard819 = assign35520_e40825;
        locals.var_guard819_rv = 0.0;

        let (assign35530_e40845,) = {
    if ((((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard815 != 0.0)) && (locals.var_guard816 != 0.0)) && (locals.var_guard817 == 0.0)) && (locals.var_guard818 == 0.0)) && (locals.var_guard819 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign35530_e40845;
        locals.var_mm_rv = 0.0;

        let assign35540_e40848: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard820 = assign35540_e40848;
        locals.var_guard820_rv = 0.0;

        let (assign35550_e40871,) = {
    if (((((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard815 != 0.0)) && (locals.var_guard816 != 0.0)) && (locals.var_guard817 == 0.0)) && (locals.var_guard818 == 0.0)) && (locals.var_guard819 == 0.0)) && (locals.var_guard820 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign35550_e40871;
        locals.var_mm_rv = 0.0;

        let (assign35560_e40883,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard815 != 0.0)) && (locals.var_guard816 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign35560_e40883;
        locals.var_m0_rv = 0.0;

        let mut assign35570_loop_guard: usize = 0;
        while {
            let assign35570_cond_e40896: f64 = if ((((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard815 != 0.0)) && (locals.var_guard816 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign35570_cond_e40896 != 0.0
        } {
            assign35570_loop_guard += 1;
            assert!(assign35570_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign35570_body0_e40909, assign35570_body0_e40909_d_n0, assign35570_body0_e40909_d_n2, assign35570_body0_e40909_d_n4, assign35570_body0_e40909_d_n5, assign35570_body0_e40909_d_n6, assign35570_body0_e40909_d_n7, assign35570_body0_e40909_d_n8, assign35570_body0_e40909_d_n9, assign35570_body0_e40909_d_n10, assign35570_body0_e40909_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard815 != 0.0)) && (locals.var_guard816 != 0.0)) {
        let assign35570_body0_e40907: f64 = (locals.var_dnm).sqrt();
        (assign35570_body0_e40907, (locals.var_dnm_dn0 / (2.0 * assign35570_body0_e40907)), (locals.var_dnm_dn2 / (2.0 * assign35570_body0_e40907)), (locals.var_dnm_dn4 / (2.0 * assign35570_body0_e40907)), (locals.var_dnm_dn5 / (2.0 * assign35570_body0_e40907)), (locals.var_dnm_dn6 / (2.0 * assign35570_body0_e40907)), (locals.var_dnm_dn7 / (2.0 * assign35570_body0_e40907)), (locals.var_dnm_dn8 / (2.0 * assign35570_body0_e40907)), (locals.var_dnm_dn9 / (2.0 * assign35570_body0_e40907)), (locals.var_dnm_dn10 / (2.0 * assign35570_body0_e40907)), (locals.var_dnm_dn13 / (2.0 * assign35570_body0_e40907)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
            locals.var_dnm = assign35570_body0_e40909;
            locals.var_dnm_dn0 = assign35570_body0_e40909_d_n0;
            locals.var_dnm_dn2 = assign35570_body0_e40909_d_n2;
            locals.var_dnm_dn4 = assign35570_body0_e40909_d_n4;
            locals.var_dnm_dn5 = assign35570_body0_e40909_d_n5;
            locals.var_dnm_dn6 = assign35570_body0_e40909_d_n6;
            locals.var_dnm_dn7 = assign35570_body0_e40909_d_n7;
            locals.var_dnm_dn8 = assign35570_body0_e40909_d_n8;
            locals.var_dnm_dn9 = assign35570_body0_e40909_d_n9;
            locals.var_dnm_dn10 = assign35570_body0_e40909_d_n10;
            locals.var_dnm_dn13 = assign35570_body0_e40909_d_n13;
            locals.var_dnm_rv = 0.0;
            let (assign35570_body1_e40923,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard815 != 0.0)) && (locals.var_guard816 != 0.0)) {
        let assign35570_body1_e40921: f64 = (locals.var_m0 + 1.0);
        (assign35570_body1_e40921,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign35570_body1_e40923;
            locals.var_m0_rv = 0.0;
        }

        let (assign35580_e40947, assign35580_e40947_d_n0, assign35580_e40947_d_n2, assign35580_e40947_d_n4, assign35580_e40947_d_n5, assign35580_e40947_d_n6, assign35580_e40947_d_n7, assign35580_e40947_d_n8, assign35580_e40947_d_n9, assign35580_e40947_d_n10, assign35580_e40947_d_n13,) = {
    if (((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard815 != 0.0)) && (locals.var_guard816 == 0.0)) {
        let (assign35580_e40945, assign35580_e40945_d_n0, assign35580_e40945_d_n2, assign35580_e40945_d_n4, assign35580_e40945_d_n5, assign35580_e40945_d_n6, assign35580_e40945_d_n7, assign35580_e40945_d_n8, assign35580_e40945_d_n9, assign35580_e40945_d_n10, assign35580_e40945_d_n13,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign35580_e40942: f64 = (2.0 * 4.0);
                let assign35580_e40943: f64 = (1.0 / assign35580_e40942);
                let assign35580_e40944: f64 = (locals.var_dnm).powf(assign35580_e40943);
                (assign35580_e40944, if 0.0 == 0.0 && ((assign35580_e40943) as f64).is_finite() && ((assign35580_e40943) as f64).fract() == 0.0 { if assign35580_e40943 == 0.0 { 0.0 } else { (assign35580_e40943 * ((locals.var_dnm).powf(assign35580_e40943 - 1.0) * locals.var_dnm_dn0)) } } else { (assign35580_e40944 * (assign35580_e40943 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35580_e40943) as f64).is_finite() && ((assign35580_e40943) as f64).fract() == 0.0 { if assign35580_e40943 == 0.0 { 0.0 } else { (assign35580_e40943 * ((locals.var_dnm).powf(assign35580_e40943 - 1.0) * locals.var_dnm_dn2)) } } else { (assign35580_e40944 * (assign35580_e40943 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35580_e40943) as f64).is_finite() && ((assign35580_e40943) as f64).fract() == 0.0 { if assign35580_e40943 == 0.0 { 0.0 } else { (assign35580_e40943 * ((locals.var_dnm).powf(assign35580_e40943 - 1.0) * locals.var_dnm_dn4)) } } else { (assign35580_e40944 * (assign35580_e40943 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35580_e40943) as f64).is_finite() && ((assign35580_e40943) as f64).fract() == 0.0 { if assign35580_e40943 == 0.0 { 0.0 } else { (assign35580_e40943 * ((locals.var_dnm).powf(assign35580_e40943 - 1.0) * locals.var_dnm_dn5)) } } else { (assign35580_e40944 * (assign35580_e40943 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35580_e40943) as f64).is_finite() && ((assign35580_e40943) as f64).fract() == 0.0 { if assign35580_e40943 == 0.0 { 0.0 } else { (assign35580_e40943 * ((locals.var_dnm).powf(assign35580_e40943 - 1.0) * locals.var_dnm_dn6)) } } else { (assign35580_e40944 * (assign35580_e40943 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35580_e40943) as f64).is_finite() && ((assign35580_e40943) as f64).fract() == 0.0 { if assign35580_e40943 == 0.0 { 0.0 } else { (assign35580_e40943 * ((locals.var_dnm).powf(assign35580_e40943 - 1.0) * locals.var_dnm_dn7)) } } else { (assign35580_e40944 * (assign35580_e40943 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35580_e40943) as f64).is_finite() && ((assign35580_e40943) as f64).fract() == 0.0 { if assign35580_e40943 == 0.0 { 0.0 } else { (assign35580_e40943 * ((locals.var_dnm).powf(assign35580_e40943 - 1.0) * locals.var_dnm_dn8)) } } else { (assign35580_e40944 * (assign35580_e40943 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35580_e40943) as f64).is_finite() && ((assign35580_e40943) as f64).fract() == 0.0 { if assign35580_e40943 == 0.0 { 0.0 } else { (assign35580_e40943 * ((locals.var_dnm).powf(assign35580_e40943 - 1.0) * locals.var_dnm_dn9)) } } else { (assign35580_e40944 * (assign35580_e40943 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35580_e40943) as f64).is_finite() && ((assign35580_e40943) as f64).fract() == 0.0 { if assign35580_e40943 == 0.0 { 0.0 } else { (assign35580_e40943 * ((locals.var_dnm).powf(assign35580_e40943 - 1.0) * locals.var_dnm_dn10)) } } else { (assign35580_e40944 * (assign35580_e40943 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign35580_e40943) as f64).is_finite() && ((assign35580_e40943) as f64).fract() == 0.0 { if assign35580_e40943 == 0.0 { 0.0 } else { (assign35580_e40943 * ((locals.var_dnm).powf(assign35580_e40943 - 1.0) * locals.var_dnm_dn13)) } } else { (assign35580_e40944 * (assign35580_e40943 * (locals.var_dnm_dn13 / locals.var_dnm))) },)
            }
        };
        (assign35580_e40945, assign35580_e40945_d_n0, assign35580_e40945_d_n2, assign35580_e40945_d_n4, assign35580_e40945_d_n5, assign35580_e40945_d_n6, assign35580_e40945_d_n7, assign35580_e40945_d_n8, assign35580_e40945_d_n9, assign35580_e40945_d_n10, assign35580_e40945_d_n13,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign35580_e40947;
        locals.var_dnm_dn0 = assign35580_e40947_d_n0;
        locals.var_dnm_dn2 = assign35580_e40947_d_n2;
        locals.var_dnm_dn4 = assign35580_e40947_d_n4;
        locals.var_dnm_dn5 = assign35580_e40947_d_n5;
        locals.var_dnm_dn6 = assign35580_e40947_d_n6;
        locals.var_dnm_dn7 = assign35580_e40947_d_n7;
        locals.var_dnm_dn8 = assign35580_e40947_d_n8;
        locals.var_dnm_dn9 = assign35580_e40947_d_n9;
        locals.var_dnm_dn10 = assign35580_e40947_d_n10;
        locals.var_dnm_dn13 = assign35580_e40947_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign35590_e40959, assign35590_e40959_d_n0, assign35590_e40959_d_n2, assign35590_e40959_d_n4, assign35590_e40959_d_n5, assign35590_e40959_d_n6, assign35590_e40959_d_n7, assign35590_e40959_d_n8, assign35590_e40959_d_n9, assign35590_e40959_d_n10, assign35590_e40959_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard815 != 0.0)) {
        let assign35590_e40957: f64 = (1.0 / locals.var_dnm);
        (assign35590_e40957, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn13 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn13,)
    }
};
        locals.var_dnm = assign35590_e40959;
        locals.var_dnm_dn0 = assign35590_e40959_d_n0;
        locals.var_dnm_dn2 = assign35590_e40959_d_n2;
        locals.var_dnm_dn4 = assign35590_e40959_d_n4;
        locals.var_dnm_dn5 = assign35590_e40959_d_n5;
        locals.var_dnm_dn6 = assign35590_e40959_d_n6;
        locals.var_dnm_dn7 = assign35590_e40959_d_n7;
        locals.var_dnm_dn8 = assign35590_e40959_d_n8;
        locals.var_dnm_dn9 = assign35590_e40959_d_n9;
        locals.var_dnm_dn10 = assign35590_e40959_d_n10;
        locals.var_dnm_dn13 = assign35590_e40959_d_n13;
        locals.var_dnm_rv = 0.0;

        let (assign35600_e40973, assign35600_e40973_d_n0, assign35600_e40973_d_n2, assign35600_e40973_d_n4, assign35600_e40973_d_n5, assign35600_e40973_d_n6, assign35600_e40973_d_n7, assign35600_e40973_d_n8, assign35600_e40973_d_n9, assign35600_e40973_d_n10, assign35600_e40973_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard815 != 0.0)) {
        let assign35600_e40969: f64 = (locals.var_tmf1 * 4.0);
        let assign35600_e40971: f64 = (assign35600_e40969 * locals.var_dnm);
        (assign35600_e40971, (((locals.var_tmf1_dn0 * 4.0) * locals.var_dnm) + (assign35600_e40969 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 4.0) * locals.var_dnm) + (assign35600_e40969 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 4.0) * locals.var_dnm) + (assign35600_e40969 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 4.0) * locals.var_dnm) + (assign35600_e40969 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 4.0) * locals.var_dnm) + (assign35600_e40969 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 4.0) * locals.var_dnm) + (assign35600_e40969 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 4.0) * locals.var_dnm) + (assign35600_e40969 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 4.0) * locals.var_dnm) + (assign35600_e40969 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 4.0) * locals.var_dnm) + (assign35600_e40969 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn13 * 4.0) * locals.var_dnm) + (assign35600_e40969 * locals.var_dnm_dn13)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn13,)
    }
};
        locals.var_tmf0 = assign35600_e40973;
        locals.var_tmf0_dn0 = assign35600_e40973_d_n0;
        locals.var_tmf0_dn2 = assign35600_e40973_d_n2;
        locals.var_tmf0_dn4 = assign35600_e40973_d_n4;
        locals.var_tmf0_dn5 = assign35600_e40973_d_n5;
        locals.var_tmf0_dn6 = assign35600_e40973_d_n6;
        locals.var_tmf0_dn7 = assign35600_e40973_d_n7;
        locals.var_tmf0_dn8 = assign35600_e40973_d_n8;
        locals.var_tmf0_dn9 = assign35600_e40973_d_n9;
        locals.var_tmf0_dn10 = assign35600_e40973_d_n10;
        locals.var_tmf0_dn13 = assign35600_e40973_d_n13;
        locals.var_tmf0_rv = 0.0;

        let (assign35610_e40989, assign35610_e40989_d_n0, assign35610_e40989_d_n2, assign35610_e40989_d_n4, assign35610_e40989_d_n5, assign35610_e40989_d_n6, assign35610_e40989_d_n7, assign35610_e40989_d_n8, assign35610_e40989_d_n9, assign35610_e40989_d_n10, assign35610_e40989_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard815 != 0.0)) {
        let assign35610_e40983: f64 = (4.0 * locals.var_xmp);
        let assign35610_e40985: f64 = (assign35610_e40983 * locals.var_dnm);
        let assign35610_e40987: f64 = (assign35610_e40985 / locals.var_arg);
        (assign35610_e40987, ((((((4.0 * locals.var_xmp_dn0) * locals.var_dnm) + (assign35610_e40983 * locals.var_dnm_dn0)) * locals.var_arg) - (assign35610_e40985 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((4.0 * locals.var_xmp_dn2) * locals.var_dnm) + (assign35610_e40983 * locals.var_dnm_dn2)) * locals.var_arg) - (assign35610_e40985 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((4.0 * locals.var_xmp_dn4) * locals.var_dnm) + (assign35610_e40983 * locals.var_dnm_dn4)) * locals.var_arg) - (assign35610_e40985 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((4.0 * locals.var_xmp_dn5) * locals.var_dnm) + (assign35610_e40983 * locals.var_dnm_dn5)) * locals.var_arg) - (assign35610_e40985 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((4.0 * locals.var_xmp_dn6) * locals.var_dnm) + (assign35610_e40983 * locals.var_dnm_dn6)) * locals.var_arg) - (assign35610_e40985 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((4.0 * locals.var_xmp_dn7) * locals.var_dnm) + (assign35610_e40983 * locals.var_dnm_dn7)) * locals.var_arg) - (assign35610_e40985 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((4.0 * locals.var_xmp_dn8) * locals.var_dnm) + (assign35610_e40983 * locals.var_dnm_dn8)) * locals.var_arg) - (assign35610_e40985 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((4.0 * locals.var_xmp_dn9) * locals.var_dnm) + (assign35610_e40983 * locals.var_dnm_dn9)) * locals.var_arg) - (assign35610_e40985 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((4.0 * locals.var_xmp_dn10) * locals.var_dnm) + (assign35610_e40983 * locals.var_dnm_dn10)) * locals.var_arg) - (assign35610_e40985 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((4.0 * locals.var_xmp_dn13) * locals.var_dnm) + (assign35610_e40983 * locals.var_dnm_dn13)) * locals.var_arg) - (assign35610_e40985 * locals.var_arg_dn13)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign35610_e40989;
        locals.var_t0_dn0 = assign35610_e40989_d_n0;
        locals.var_t0_dn2 = assign35610_e40989_d_n2;
        locals.var_t0_dn4 = assign35610_e40989_d_n4;
        locals.var_t0_dn5 = assign35610_e40989_d_n5;
        locals.var_t0_dn6 = assign35610_e40989_d_n6;
        locals.var_t0_dn7 = assign35610_e40989_d_n7;
        locals.var_t0_dn8 = assign35610_e40989_d_n8;
        locals.var_t0_dn9 = assign35610_e40989_d_n9;
        locals.var_t0_dn10 = assign35610_e40989_d_n10;
        locals.var_t0_dn13 = assign35610_e40989_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign35620_e41003, assign35620_e41003_d_n0, assign35620_e41003_d_n2, assign35620_e41003_d_n4, assign35620_e41003_d_n5, assign35620_e41003_d_n6, assign35620_e41003_d_n7, assign35620_e41003_d_n8, assign35620_e41003_d_n9, assign35620_e41003_d_n10, assign35620_e41003_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard815 != 0.0)) {
        let assign35620_e40999: f64 = (locals.var_uc_depleak + 4.0);
        let assign35620_e41001: f64 = (assign35620_e40999 - locals.var_tmf0);
        (assign35620_e41001, (locals.var_uc_depleak_dn0 - locals.var_tmf0_dn0), (locals.var_uc_depleak_dn2 - locals.var_tmf0_dn2), (locals.var_uc_depleak_dn4 - locals.var_tmf0_dn4), (locals.var_uc_depleak_dn5 - locals.var_tmf0_dn5), (locals.var_uc_depleak_dn6 - locals.var_tmf0_dn6), (locals.var_uc_depleak_dn7 - locals.var_tmf0_dn7), (locals.var_uc_depleak_dn8 - locals.var_tmf0_dn8), (locals.var_uc_depleak_dn9 - locals.var_tmf0_dn9), (locals.var_uc_depleak_dn10 - locals.var_tmf0_dn10), (locals.var_uc_depleak_dn13 - locals.var_tmf0_dn13),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign35620_e41003;
        locals.var_t10_dn0 = assign35620_e41003_d_n0;
        locals.var_t10_dn2 = assign35620_e41003_d_n2;
        locals.var_t10_dn4 = assign35620_e41003_d_n4;
        locals.var_t10_dn5 = assign35620_e41003_d_n5;
        locals.var_t10_dn6 = assign35620_e41003_d_n6;
        locals.var_t10_dn7 = assign35620_e41003_d_n7;
        locals.var_t10_dn8 = assign35620_e41003_d_n8;
        locals.var_t10_dn9 = assign35620_e41003_d_n9;
        locals.var_t10_dn10 = assign35620_e41003_d_n10;
        locals.var_t10_dn13 = assign35620_e41003_d_n13;
        locals.var_t10_rv = 0.0;

        let (assign35630_e41013, assign35630_e41013_d_n0, assign35630_e41013_d_n2, assign35630_e41013_d_n4, assign35630_e41013_d_n5, assign35630_e41013_d_n6, assign35630_e41013_d_n7, assign35630_e41013_d_n8, assign35630_e41013_d_n9, assign35630_e41013_d_n10, assign35630_e41013_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard815 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign35630_e41013;
        locals.var_t0_dn0 = assign35630_e41013_d_n0;
        locals.var_t0_dn2 = assign35630_e41013_d_n2;
        locals.var_t0_dn4 = assign35630_e41013_d_n4;
        locals.var_t0_dn5 = assign35630_e41013_d_n5;
        locals.var_t0_dn6 = assign35630_e41013_d_n6;
        locals.var_t0_dn7 = assign35630_e41013_d_n7;
        locals.var_t0_dn8 = assign35630_e41013_d_n8;
        locals.var_t0_dn9 = assign35630_e41013_d_n9;
        locals.var_t0_dn10 = assign35630_e41013_d_n10;
        locals.var_t0_dn13 = assign35630_e41013_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign35640_e41024, assign35640_e41024_d_n0, assign35640_e41024_d_n2, assign35640_e41024_d_n4, assign35640_e41024_d_n5, assign35640_e41024_d_n6, assign35640_e41024_d_n7, assign35640_e41024_d_n8, assign35640_e41024_d_n9, assign35640_e41024_d_n10, assign35640_e41024_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard815 == 0.0)) {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign35640_e41024;
        locals.var_t10_dn0 = assign35640_e41024_d_n0;
        locals.var_t10_dn2 = assign35640_e41024_d_n2;
        locals.var_t10_dn4 = assign35640_e41024_d_n4;
        locals.var_t10_dn5 = assign35640_e41024_d_n5;
        locals.var_t10_dn6 = assign35640_e41024_d_n6;
        locals.var_t10_dn7 = assign35640_e41024_d_n7;
        locals.var_t10_dn8 = assign35640_e41024_d_n8;
        locals.var_t10_dn9 = assign35640_e41024_d_n9;
        locals.var_t10_dn10 = assign35640_e41024_d_n10;
        locals.var_t10_dn13 = assign35640_e41024_d_n13;
        locals.var_t10_rv = 0.0;

        let (assign35650_e41035, assign35650_e41035_d_n0, assign35650_e41035_d_n2, assign35650_e41035_d_n4, assign35650_e41035_d_n5, assign35650_e41035_d_n6, assign35650_e41035_d_n7, assign35650_e41035_d_n8, assign35650_e41035_d_n9, assign35650_e41035_d_n10, assign35650_e41035_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard815 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign35650_e41035;
        locals.var_t0_dn0 = assign35650_e41035_d_n0;
        locals.var_t0_dn2 = assign35650_e41035_d_n2;
        locals.var_t0_dn4 = assign35650_e41035_d_n4;
        locals.var_t0_dn5 = assign35650_e41035_d_n5;
        locals.var_t0_dn6 = assign35650_e41035_d_n6;
        locals.var_t0_dn7 = assign35650_e41035_d_n7;
        locals.var_t0_dn8 = assign35650_e41035_d_n8;
        locals.var_t0_dn9 = assign35650_e41035_d_n9;
        locals.var_t0_dn10 = assign35650_e41035_d_n10;
        locals.var_t0_dn13 = assign35650_e41035_d_n13;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_112(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign35660_e41045, assign35660_e41045_d_n0, assign35660_e41045_d_n2, assign35660_e41045_d_n4, assign35660_e41045_d_n5, assign35660_e41045_d_n6, assign35660_e41045_d_n7, assign35660_e41045_d_n8, assign35660_e41045_d_n9, assign35660_e41045_d_n10, assign35660_e41045_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) {
        let assign35660_e41043: f64 = (locals.var_vdsorg / locals.var_t10);
        (assign35660_e41043, (((locals.var_vdsorg_dn0 * locals.var_t10) - (locals.var_vdsorg * locals.var_t10_dn0)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vdsorg_dn2 * locals.var_t10) - (locals.var_vdsorg * locals.var_t10_dn2)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vdsorg_dn4 * locals.var_t10) - (locals.var_vdsorg * locals.var_t10_dn4)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vdsorg_dn5 * locals.var_t10) - (locals.var_vdsorg * locals.var_t10_dn5)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vdsorg_dn6 * locals.var_t10) - (locals.var_vdsorg * locals.var_t10_dn6)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vdsorg_dn7 * locals.var_t10) - (locals.var_vdsorg * locals.var_t10_dn7)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vdsorg_dn8 * locals.var_t10) - (locals.var_vdsorg * locals.var_t10_dn8)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vdsorg_dn9 * locals.var_t10) - (locals.var_vdsorg * locals.var_t10_dn9)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vdsorg_dn10 * locals.var_t10) - (locals.var_vdsorg * locals.var_t10_dn10)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vdsorg_dn13 * locals.var_t10) - (locals.var_vdsorg * locals.var_t10_dn13)) / (locals.var_t10 * locals.var_t10)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign35660_e41045;
        locals.var_t1_dn0 = assign35660_e41045_d_n0;
        locals.var_t1_dn2 = assign35660_e41045_d_n2;
        locals.var_t1_dn4 = assign35660_e41045_d_n4;
        locals.var_t1_dn5 = assign35660_e41045_d_n5;
        locals.var_t1_dn6 = assign35660_e41045_d_n6;
        locals.var_t1_dn7 = assign35660_e41045_d_n7;
        locals.var_t1_dn8 = assign35660_e41045_d_n8;
        locals.var_t1_dn9 = assign35660_e41045_d_n9;
        locals.var_t1_dn10 = assign35660_e41045_d_n10;
        locals.var_t1_dn13 = assign35660_e41045_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign35670_e41062, assign35670_e41062_d_n0, assign35670_e41062_d_n2, assign35670_e41062_d_n4, assign35670_e41062_d_n5, assign35670_e41062_d_n6, assign35670_e41062_d_n7, assign35670_e41062_d_n8, assign35670_e41062_d_n9, assign35670_e41062_d_n10, assign35670_e41062_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) {
        let (assign35670_e41060, assign35670_e41060_d_n0, assign35670_e41060_d_n2, assign35670_e41060_d_n4, assign35670_e41060_d_n5, assign35670_e41060_d_n6, assign35670_e41060_d_n7, assign35670_e41060_d_n8, assign35670_e41060_d_n9, assign35670_e41060_d_n10, assign35670_e41060_d_n13,) = {
            if (locals.var_t1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign35670_e41058: f64 = (locals.var_ddlte - 1.0);
                let assign35670_e41059: f64 = (locals.var_t1).powf(assign35670_e41058);
                (assign35670_e41059, if locals.var_ddlte_dn0 == 0.0 && ((assign35670_e41058) as f64).is_finite() && ((assign35670_e41058) as f64).fract() == 0.0 { if assign35670_e41058 == 0.0 { 0.0 } else { (assign35670_e41058 * ((locals.var_t1).powf(assign35670_e41058 - 1.0) * locals.var_t1_dn0)) } } else { (assign35670_e41059 * ((locals.var_ddlte_dn0 * (locals.var_t1).ln()) + (assign35670_e41058 * (locals.var_t1_dn0 / locals.var_t1)))) }, if locals.var_ddlte_dn2 == 0.0 && ((assign35670_e41058) as f64).is_finite() && ((assign35670_e41058) as f64).fract() == 0.0 { if assign35670_e41058 == 0.0 { 0.0 } else { (assign35670_e41058 * ((locals.var_t1).powf(assign35670_e41058 - 1.0) * locals.var_t1_dn2)) } } else { (assign35670_e41059 * ((locals.var_ddlte_dn2 * (locals.var_t1).ln()) + (assign35670_e41058 * (locals.var_t1_dn2 / locals.var_t1)))) }, if locals.var_ddlte_dn4 == 0.0 && ((assign35670_e41058) as f64).is_finite() && ((assign35670_e41058) as f64).fract() == 0.0 { if assign35670_e41058 == 0.0 { 0.0 } else { (assign35670_e41058 * ((locals.var_t1).powf(assign35670_e41058 - 1.0) * locals.var_t1_dn4)) } } else { (assign35670_e41059 * ((locals.var_ddlte_dn4 * (locals.var_t1).ln()) + (assign35670_e41058 * (locals.var_t1_dn4 / locals.var_t1)))) }, if locals.var_ddlte_dn5 == 0.0 && ((assign35670_e41058) as f64).is_finite() && ((assign35670_e41058) as f64).fract() == 0.0 { if assign35670_e41058 == 0.0 { 0.0 } else { (assign35670_e41058 * ((locals.var_t1).powf(assign35670_e41058 - 1.0) * locals.var_t1_dn5)) } } else { (assign35670_e41059 * ((locals.var_ddlte_dn5 * (locals.var_t1).ln()) + (assign35670_e41058 * (locals.var_t1_dn5 / locals.var_t1)))) }, if locals.var_ddlte_dn6 == 0.0 && ((assign35670_e41058) as f64).is_finite() && ((assign35670_e41058) as f64).fract() == 0.0 { if assign35670_e41058 == 0.0 { 0.0 } else { (assign35670_e41058 * ((locals.var_t1).powf(assign35670_e41058 - 1.0) * locals.var_t1_dn6)) } } else { (assign35670_e41059 * ((locals.var_ddlte_dn6 * (locals.var_t1).ln()) + (assign35670_e41058 * (locals.var_t1_dn6 / locals.var_t1)))) }, if locals.var_ddlte_dn7 == 0.0 && ((assign35670_e41058) as f64).is_finite() && ((assign35670_e41058) as f64).fract() == 0.0 { if assign35670_e41058 == 0.0 { 0.0 } else { (assign35670_e41058 * ((locals.var_t1).powf(assign35670_e41058 - 1.0) * locals.var_t1_dn7)) } } else { (assign35670_e41059 * ((locals.var_ddlte_dn7 * (locals.var_t1).ln()) + (assign35670_e41058 * (locals.var_t1_dn7 / locals.var_t1)))) }, if locals.var_ddlte_dn8 == 0.0 && ((assign35670_e41058) as f64).is_finite() && ((assign35670_e41058) as f64).fract() == 0.0 { if assign35670_e41058 == 0.0 { 0.0 } else { (assign35670_e41058 * ((locals.var_t1).powf(assign35670_e41058 - 1.0) * locals.var_t1_dn8)) } } else { (assign35670_e41059 * ((locals.var_ddlte_dn8 * (locals.var_t1).ln()) + (assign35670_e41058 * (locals.var_t1_dn8 / locals.var_t1)))) }, if locals.var_ddlte_dn9 == 0.0 && ((assign35670_e41058) as f64).is_finite() && ((assign35670_e41058) as f64).fract() == 0.0 { if assign35670_e41058 == 0.0 { 0.0 } else { (assign35670_e41058 * ((locals.var_t1).powf(assign35670_e41058 - 1.0) * locals.var_t1_dn9)) } } else { (assign35670_e41059 * ((locals.var_ddlte_dn9 * (locals.var_t1).ln()) + (assign35670_e41058 * (locals.var_t1_dn9 / locals.var_t1)))) }, if locals.var_ddlte_dn10 == 0.0 && ((assign35670_e41058) as f64).is_finite() && ((assign35670_e41058) as f64).fract() == 0.0 { if assign35670_e41058 == 0.0 { 0.0 } else { (assign35670_e41058 * ((locals.var_t1).powf(assign35670_e41058 - 1.0) * locals.var_t1_dn10)) } } else { (assign35670_e41059 * ((locals.var_ddlte_dn10 * (locals.var_t1).ln()) + (assign35670_e41058 * (locals.var_t1_dn10 / locals.var_t1)))) }, if locals.var_ddlte_dn13 == 0.0 && ((assign35670_e41058) as f64).is_finite() && ((assign35670_e41058) as f64).fract() == 0.0 { if assign35670_e41058 == 0.0 { 0.0 } else { (assign35670_e41058 * ((locals.var_t1).powf(assign35670_e41058 - 1.0) * locals.var_t1_dn13)) } } else { (assign35670_e41059 * ((locals.var_ddlte_dn13 * (locals.var_t1).ln()) + (assign35670_e41058 * (locals.var_t1_dn13 / locals.var_t1)))) },)
            }
        };
        (assign35670_e41060, assign35670_e41060_d_n0, assign35670_e41060_d_n2, assign35670_e41060_d_n4, assign35670_e41060_d_n5, assign35670_e41060_d_n6, assign35670_e41060_d_n7, assign35670_e41060_d_n8, assign35670_e41060_d_n9, assign35670_e41060_d_n10, assign35670_e41060_d_n13,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign35670_e41062;
        locals.var_t2_dn0 = assign35670_e41062_d_n0;
        locals.var_t2_dn2 = assign35670_e41062_d_n2;
        locals.var_t2_dn4 = assign35670_e41062_d_n4;
        locals.var_t2_dn5 = assign35670_e41062_d_n5;
        locals.var_t2_dn6 = assign35670_e41062_d_n6;
        locals.var_t2_dn7 = assign35670_e41062_d_n7;
        locals.var_t2_dn8 = assign35670_e41062_d_n8;
        locals.var_t2_dn9 = assign35670_e41062_d_n9;
        locals.var_t2_dn10 = assign35670_e41062_d_n10;
        locals.var_t2_dn13 = assign35670_e41062_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign35680_e41072, assign35680_e41072_d_n0, assign35680_e41072_d_n2, assign35680_e41072_d_n4, assign35680_e41072_d_n5, assign35680_e41072_d_n6, assign35680_e41072_d_n7, assign35680_e41072_d_n8, assign35680_e41072_d_n9, assign35680_e41072_d_n10, assign35680_e41072_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) {
        let assign35680_e41070: f64 = (locals.var_t2 * locals.var_t1);
        (assign35680_e41070, ((locals.var_t2_dn0 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn0)), ((locals.var_t2_dn2 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn2)), ((locals.var_t2_dn4 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn4)), ((locals.var_t2_dn5 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn5)), ((locals.var_t2_dn6 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn6)), ((locals.var_t2_dn7 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn7)), ((locals.var_t2_dn8 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn8)), ((locals.var_t2_dn9 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn9)), ((locals.var_t2_dn10 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn10)), ((locals.var_t2_dn13 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn13)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign35680_e41072;
        locals.var_t7_dn0 = assign35680_e41072_d_n0;
        locals.var_t7_dn2 = assign35680_e41072_d_n2;
        locals.var_t7_dn4 = assign35680_e41072_d_n4;
        locals.var_t7_dn5 = assign35680_e41072_d_n5;
        locals.var_t7_dn6 = assign35680_e41072_d_n6;
        locals.var_t7_dn7 = assign35680_e41072_d_n7;
        locals.var_t7_dn8 = assign35680_e41072_d_n8;
        locals.var_t7_dn9 = assign35680_e41072_d_n9;
        locals.var_t7_dn10 = assign35680_e41072_d_n10;
        locals.var_t7_dn13 = assign35680_e41072_d_n13;
        locals.var_t7_rv = 0.0;

        let (assign35690_e41082, assign35690_e41082_d_n0, assign35690_e41082_d_n2, assign35690_e41082_d_n4, assign35690_e41082_d_n5, assign35690_e41082_d_n6, assign35690_e41082_d_n7, assign35690_e41082_d_n8, assign35690_e41082_d_n9, assign35690_e41082_d_n10, assign35690_e41082_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) {
        let assign35690_e41080: f64 = (1.0 + locals.var_t7);
        (assign35690_e41080, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign35690_e41082;
        locals.var_t3_dn0 = assign35690_e41082_d_n0;
        locals.var_t3_dn2 = assign35690_e41082_d_n2;
        locals.var_t3_dn4 = assign35690_e41082_d_n4;
        locals.var_t3_dn5 = assign35690_e41082_d_n5;
        locals.var_t3_dn6 = assign35690_e41082_d_n6;
        locals.var_t3_dn7 = assign35690_e41082_d_n7;
        locals.var_t3_dn8 = assign35690_e41082_d_n8;
        locals.var_t3_dn9 = assign35690_e41082_d_n9;
        locals.var_t3_dn10 = assign35690_e41082_d_n10;
        locals.var_t3_dn13 = assign35690_e41082_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign35700_e41101, assign35700_e41101_d_n0, assign35700_e41101_d_n2, assign35700_e41101_d_n4, assign35700_e41101_d_n5, assign35700_e41101_d_n6, assign35700_e41101_d_n7, assign35700_e41101_d_n8, assign35700_e41101_d_n9, assign35700_e41101_d_n10, assign35700_e41101_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) {
        let (assign35700_e41099, assign35700_e41099_d_n0, assign35700_e41099_d_n2, assign35700_e41099_d_n4, assign35700_e41099_d_n5, assign35700_e41099_d_n6, assign35700_e41099_d_n7, assign35700_e41099_d_n8, assign35700_e41099_d_n9, assign35700_e41099_d_n10, assign35700_e41099_d_n13,) = {
            if (locals.var_t3 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign35700_e41095: f64 = (1.0 / locals.var_ddlte);
                let assign35700_e41097: f64 = (assign35700_e41095 - 1.0);
                let assign35700_e41098: f64 = (locals.var_t3).powf(assign35700_e41097);
                (assign35700_e41098, if (-(locals.var_ddlte_dn0 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign35700_e41097) as f64).is_finite() && ((assign35700_e41097) as f64).fract() == 0.0 { if assign35700_e41097 == 0.0 { 0.0 } else { (assign35700_e41097 * ((locals.var_t3).powf(assign35700_e41097 - 1.0) * locals.var_t3_dn0)) } } else { (assign35700_e41098 * (((-(locals.var_ddlte_dn0 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign35700_e41097 * (locals.var_t3_dn0 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn2 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign35700_e41097) as f64).is_finite() && ((assign35700_e41097) as f64).fract() == 0.0 { if assign35700_e41097 == 0.0 { 0.0 } else { (assign35700_e41097 * ((locals.var_t3).powf(assign35700_e41097 - 1.0) * locals.var_t3_dn2)) } } else { (assign35700_e41098 * (((-(locals.var_ddlte_dn2 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign35700_e41097 * (locals.var_t3_dn2 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn4 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign35700_e41097) as f64).is_finite() && ((assign35700_e41097) as f64).fract() == 0.0 { if assign35700_e41097 == 0.0 { 0.0 } else { (assign35700_e41097 * ((locals.var_t3).powf(assign35700_e41097 - 1.0) * locals.var_t3_dn4)) } } else { (assign35700_e41098 * (((-(locals.var_ddlte_dn4 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign35700_e41097 * (locals.var_t3_dn4 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn5 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign35700_e41097) as f64).is_finite() && ((assign35700_e41097) as f64).fract() == 0.0 { if assign35700_e41097 == 0.0 { 0.0 } else { (assign35700_e41097 * ((locals.var_t3).powf(assign35700_e41097 - 1.0) * locals.var_t3_dn5)) } } else { (assign35700_e41098 * (((-(locals.var_ddlte_dn5 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign35700_e41097 * (locals.var_t3_dn5 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn6 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign35700_e41097) as f64).is_finite() && ((assign35700_e41097) as f64).fract() == 0.0 { if assign35700_e41097 == 0.0 { 0.0 } else { (assign35700_e41097 * ((locals.var_t3).powf(assign35700_e41097 - 1.0) * locals.var_t3_dn6)) } } else { (assign35700_e41098 * (((-(locals.var_ddlte_dn6 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign35700_e41097 * (locals.var_t3_dn6 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn7 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign35700_e41097) as f64).is_finite() && ((assign35700_e41097) as f64).fract() == 0.0 { if assign35700_e41097 == 0.0 { 0.0 } else { (assign35700_e41097 * ((locals.var_t3).powf(assign35700_e41097 - 1.0) * locals.var_t3_dn7)) } } else { (assign35700_e41098 * (((-(locals.var_ddlte_dn7 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign35700_e41097 * (locals.var_t3_dn7 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn8 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign35700_e41097) as f64).is_finite() && ((assign35700_e41097) as f64).fract() == 0.0 { if assign35700_e41097 == 0.0 { 0.0 } else { (assign35700_e41097 * ((locals.var_t3).powf(assign35700_e41097 - 1.0) * locals.var_t3_dn8)) } } else { (assign35700_e41098 * (((-(locals.var_ddlte_dn8 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign35700_e41097 * (locals.var_t3_dn8 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn9 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign35700_e41097) as f64).is_finite() && ((assign35700_e41097) as f64).fract() == 0.0 { if assign35700_e41097 == 0.0 { 0.0 } else { (assign35700_e41097 * ((locals.var_t3).powf(assign35700_e41097 - 1.0) * locals.var_t3_dn9)) } } else { (assign35700_e41098 * (((-(locals.var_ddlte_dn9 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign35700_e41097 * (locals.var_t3_dn9 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn10 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign35700_e41097) as f64).is_finite() && ((assign35700_e41097) as f64).fract() == 0.0 { if assign35700_e41097 == 0.0 { 0.0 } else { (assign35700_e41097 * ((locals.var_t3).powf(assign35700_e41097 - 1.0) * locals.var_t3_dn10)) } } else { (assign35700_e41098 * (((-(locals.var_ddlte_dn10 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign35700_e41097 * (locals.var_t3_dn10 / locals.var_t3)))) }, if (-(locals.var_ddlte_dn13 / (locals.var_ddlte * locals.var_ddlte))) == 0.0 && ((assign35700_e41097) as f64).is_finite() && ((assign35700_e41097) as f64).fract() == 0.0 { if assign35700_e41097 == 0.0 { 0.0 } else { (assign35700_e41097 * ((locals.var_t3).powf(assign35700_e41097 - 1.0) * locals.var_t3_dn13)) } } else { (assign35700_e41098 * (((-(locals.var_ddlte_dn13 / (locals.var_ddlte * locals.var_ddlte))) * (locals.var_t3).ln()) + (assign35700_e41097 * (locals.var_t3_dn13 / locals.var_t3)))) },)
            }
        };
        (assign35700_e41099, assign35700_e41099_d_n0, assign35700_e41099_d_n2, assign35700_e41099_d_n4, assign35700_e41099_d_n5, assign35700_e41099_d_n6, assign35700_e41099_d_n7, assign35700_e41099_d_n8, assign35700_e41099_d_n9, assign35700_e41099_d_n10, assign35700_e41099_d_n13,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign35700_e41101;
        locals.var_t4_dn0 = assign35700_e41101_d_n0;
        locals.var_t4_dn2 = assign35700_e41101_d_n2;
        locals.var_t4_dn4 = assign35700_e41101_d_n4;
        locals.var_t4_dn5 = assign35700_e41101_d_n5;
        locals.var_t4_dn6 = assign35700_e41101_d_n6;
        locals.var_t4_dn7 = assign35700_e41101_d_n7;
        locals.var_t4_dn8 = assign35700_e41101_d_n8;
        locals.var_t4_dn9 = assign35700_e41101_d_n9;
        locals.var_t4_dn10 = assign35700_e41101_d_n10;
        locals.var_t4_dn13 = assign35700_e41101_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign35710_e41111, assign35710_e41111_d_n0, assign35710_e41111_d_n2, assign35710_e41111_d_n4, assign35710_e41111_d_n5, assign35710_e41111_d_n6, assign35710_e41111_d_n7, assign35710_e41111_d_n8, assign35710_e41111_d_n9, assign35710_e41111_d_n10, assign35710_e41111_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) {
        let assign35710_e41109: f64 = (locals.var_t4 * locals.var_t3);
        (assign35710_e41109, ((locals.var_t4_dn0 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn0)), ((locals.var_t4_dn2 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn2)), ((locals.var_t4_dn4 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn4)), ((locals.var_t4_dn5 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn5)), ((locals.var_t4_dn6 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn6)), ((locals.var_t4_dn7 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn7)), ((locals.var_t4_dn8 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn8)), ((locals.var_t4_dn9 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn9)), ((locals.var_t4_dn10 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn10)), ((locals.var_t4_dn13 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn13)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign35710_e41111;
        locals.var_t6_dn0 = assign35710_e41111_d_n0;
        locals.var_t6_dn2 = assign35710_e41111_d_n2;
        locals.var_t6_dn4 = assign35710_e41111_d_n4;
        locals.var_t6_dn5 = assign35710_e41111_d_n5;
        locals.var_t6_dn6 = assign35710_e41111_d_n6;
        locals.var_t6_dn7 = assign35710_e41111_d_n7;
        locals.var_t6_dn8 = assign35710_e41111_d_n8;
        locals.var_t6_dn9 = assign35710_e41111_d_n9;
        locals.var_t6_dn10 = assign35710_e41111_d_n10;
        locals.var_t6_dn13 = assign35710_e41111_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign35720_e41121, assign35720_e41121_d_n0, assign35720_e41121_d_n2, assign35720_e41121_d_n4, assign35720_e41121_d_n5, assign35720_e41121_d_n6, assign35720_e41121_d_n7, assign35720_e41121_d_n8, assign35720_e41121_d_n9, assign35720_e41121_d_n10, assign35720_e41121_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 != 0.0)) {
        let assign35720_e41119: f64 = (locals.var_vdsorg / locals.var_t6);
        (assign35720_e41119, (((locals.var_vdsorg_dn0 * locals.var_t6) - (locals.var_vdsorg * locals.var_t6_dn0)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vdsorg_dn2 * locals.var_t6) - (locals.var_vdsorg * locals.var_t6_dn2)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vdsorg_dn4 * locals.var_t6) - (locals.var_vdsorg * locals.var_t6_dn4)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vdsorg_dn5 * locals.var_t6) - (locals.var_vdsorg * locals.var_t6_dn5)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vdsorg_dn6 * locals.var_t6) - (locals.var_vdsorg * locals.var_t6_dn6)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vdsorg_dn7 * locals.var_t6) - (locals.var_vdsorg * locals.var_t6_dn7)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vdsorg_dn8 * locals.var_t6) - (locals.var_vdsorg * locals.var_t6_dn8)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vdsorg_dn9 * locals.var_t6) - (locals.var_vdsorg * locals.var_t6_dn9)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vdsorg_dn10 * locals.var_t6) - (locals.var_vdsorg * locals.var_t6_dn10)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vdsorg_dn13 * locals.var_t6) - (locals.var_vdsorg * locals.var_t6_dn13)) / (locals.var_t6 * locals.var_t6)),)
    } else {
        (locals.var_vdseff0, locals.var_vdseff0_dn0, locals.var_vdseff0_dn2, locals.var_vdseff0_dn4, locals.var_vdseff0_dn5, locals.var_vdseff0_dn6, locals.var_vdseff0_dn7, locals.var_vdseff0_dn8, locals.var_vdseff0_dn9, locals.var_vdseff0_dn10, locals.var_vdseff0_dn13,)
    }
};
        locals.var_vdseff0 = assign35720_e41121;
        locals.var_vdseff0_dn0 = assign35720_e41121_d_n0;
        locals.var_vdseff0_dn2 = assign35720_e41121_d_n2;
        locals.var_vdseff0_dn4 = assign35720_e41121_d_n4;
        locals.var_vdseff0_dn5 = assign35720_e41121_d_n5;
        locals.var_vdseff0_dn6 = assign35720_e41121_d_n6;
        locals.var_vdseff0_dn7 = assign35720_e41121_d_n7;
        locals.var_vdseff0_dn8 = assign35720_e41121_d_n8;
        locals.var_vdseff0_dn9 = assign35720_e41121_d_n9;
        locals.var_vdseff0_dn10 = assign35720_e41121_d_n10;
        locals.var_vdseff0_dn13 = assign35720_e41121_d_n13;
        locals.var_vdseff0_rv = 0.0;

        let (assign35730_e41130, assign35730_e41130_d_n0, assign35730_e41130_d_n2, assign35730_e41130_d_n4, assign35730_e41130_d_n5, assign35730_e41130_d_n6, assign35730_e41130_d_n7, assign35730_e41130_d_n8, assign35730_e41130_d_n9, assign35730_e41130_d_n10, assign35730_e41130_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard808 == 0.0)) {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn4, locals.var_vdsorg_dn5, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn8, locals.var_vdsorg_dn9, locals.var_vdsorg_dn10, locals.var_vdsorg_dn13,)
    } else {
        (locals.var_vdseff0, locals.var_vdseff0_dn0, locals.var_vdseff0_dn2, locals.var_vdseff0_dn4, locals.var_vdseff0_dn5, locals.var_vdseff0_dn6, locals.var_vdseff0_dn7, locals.var_vdseff0_dn8, locals.var_vdseff0_dn9, locals.var_vdseff0_dn10, locals.var_vdseff0_dn13,)
    }
};
        locals.var_vdseff0 = assign35730_e41130;
        locals.var_vdseff0_dn0 = assign35730_e41130_d_n0;
        locals.var_vdseff0_dn2 = assign35730_e41130_d_n2;
        locals.var_vdseff0_dn4 = assign35730_e41130_d_n4;
        locals.var_vdseff0_dn5 = assign35730_e41130_d_n5;
        locals.var_vdseff0_dn6 = assign35730_e41130_d_n6;
        locals.var_vdseff0_dn7 = assign35730_e41130_d_n7;
        locals.var_vdseff0_dn8 = assign35730_e41130_d_n8;
        locals.var_vdseff0_dn9 = assign35730_e41130_d_n9;
        locals.var_vdseff0_dn10 = assign35730_e41130_d_n10;
        locals.var_vdseff0_dn13 = assign35730_e41130_d_n13;
        locals.var_vdseff0_rv = 0.0;

        let (assign35740_e41142, assign35740_e41142_d_n0, assign35740_e41142_d_n2, assign35740_e41142_d_n4, assign35740_e41142_d_n5, assign35740_e41142_d_n6, assign35740_e41142_d_n7, assign35740_e41142_d_n8, assign35740_e41142_d_n9, assign35740_e41142_d_n10, assign35740_e41142_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign35740_e41137: f64 = (locals.var_phi_sl_dep - locals.var_phi_s0_dep);
        let assign35740_e41139: f64 = (assign35740_e41137 * locals.var_ninvde);
        let assign35740_e41140: f64 = (1.0 + assign35740_e41139);
        (assign35740_e41140, (((locals.var_phi_sl_dep_dn0 - locals.var_phi_s0_dep_dn0) * locals.var_ninvde) + (assign35740_e41137 * locals.var_ninvde_dn0)), (((locals.var_phi_sl_dep_dn2 - locals.var_phi_s0_dep_dn2) * locals.var_ninvde) + (assign35740_e41137 * locals.var_ninvde_dn2)), (((locals.var_phi_sl_dep_dn4 - locals.var_phi_s0_dep_dn4) * locals.var_ninvde) + (assign35740_e41137 * locals.var_ninvde_dn4)), (((locals.var_phi_sl_dep_dn5 - locals.var_phi_s0_dep_dn5) * locals.var_ninvde) + (assign35740_e41137 * locals.var_ninvde_dn5)), (((locals.var_phi_sl_dep_dn6 - locals.var_phi_s0_dep_dn6) * locals.var_ninvde) + (assign35740_e41137 * locals.var_ninvde_dn6)), (((locals.var_phi_sl_dep_dn7 - locals.var_phi_s0_dep_dn7) * locals.var_ninvde) + (assign35740_e41137 * locals.var_ninvde_dn7)), (((locals.var_phi_sl_dep_dn8 - locals.var_phi_s0_dep_dn8) * locals.var_ninvde) + (assign35740_e41137 * locals.var_ninvde_dn8)), (((locals.var_phi_sl_dep_dn9 - locals.var_phi_s0_dep_dn9) * locals.var_ninvde) + (assign35740_e41137 * locals.var_ninvde_dn9)), (((locals.var_phi_sl_dep_dn10 - locals.var_phi_s0_dep_dn10) * locals.var_ninvde) + (assign35740_e41137 * locals.var_ninvde_dn10)), (((locals.var_phi_sl_dep_dn13 - locals.var_phi_s0_dep_dn13) * locals.var_ninvde) + (assign35740_e41137 * locals.var_ninvde_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign35740_e41142;
        locals.var_t4_dn0 = assign35740_e41142_d_n0;
        locals.var_t4_dn2 = assign35740_e41142_d_n2;
        locals.var_t4_dn4 = assign35740_e41142_d_n4;
        locals.var_t4_dn5 = assign35740_e41142_d_n5;
        locals.var_t4_dn6 = assign35740_e41142_d_n6;
        locals.var_t4_dn7 = assign35740_e41142_d_n7;
        locals.var_t4_dn8 = assign35740_e41142_d_n8;
        locals.var_t4_dn9 = assign35740_e41142_d_n9;
        locals.var_t4_dn10 = assign35740_e41142_d_n10;
        locals.var_t4_dn13 = assign35740_e41142_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign35750_e41149, assign35750_e41149_d_n0, assign35750_e41149_d_n2, assign35750_e41149_d_n4, assign35750_e41149_d_n5, assign35750_e41149_d_n6, assign35750_e41149_d_n7, assign35750_e41149_d_n8, assign35750_e41149_d_n9, assign35750_e41149_d_n10, assign35750_e41149_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign35750_e41147: f64 = (-locals.var_qn_res0);
        (assign35750_e41147, (-locals.var_qn_res0_dn0), (-locals.var_qn_res0_dn2), (-locals.var_qn_res0_dn4), (-locals.var_qn_res0_dn5), (-locals.var_qn_res0_dn6), (-locals.var_qn_res0_dn7), (-locals.var_qn_res0_dn8), (-locals.var_qn_res0_dn9), (-locals.var_qn_res0_dn10), (-locals.var_qn_res0_dn13),)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn4, locals.var_qiu_dn5, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn8, locals.var_qiu_dn9, locals.var_qiu_dn10, locals.var_qiu_dn13,)
    }
};
        locals.var_qiu = assign35750_e41149;
        locals.var_qiu_dn0 = assign35750_e41149_d_n0;
        locals.var_qiu_dn2 = assign35750_e41149_d_n2;
        locals.var_qiu_dn4 = assign35750_e41149_d_n4;
        locals.var_qiu_dn5 = assign35750_e41149_d_n5;
        locals.var_qiu_dn6 = assign35750_e41149_d_n6;
        locals.var_qiu_dn7 = assign35750_e41149_d_n7;
        locals.var_qiu_dn8 = assign35750_e41149_d_n8;
        locals.var_qiu_dn9 = assign35750_e41149_d_n9;
        locals.var_qiu_dn10 = assign35750_e41149_d_n10;
        locals.var_qiu_dn13 = assign35750_e41149_d_n13;
        locals.var_qiu_rv = 0.0;

        let (assign35760_e41155, assign35760_e41155_d_n0, assign35760_e41155_d_n2, assign35760_e41155_d_n4, assign35760_e41155_d_n5, assign35760_e41155_d_n6, assign35760_e41155_d_n7, assign35760_e41155_d_n8, assign35760_e41155_d_n9, assign35760_e41155_d_n10, assign35760_e41155_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn4, locals.var_qiu_dn5, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn8, locals.var_qiu_dn9, locals.var_qiu_dn10, locals.var_qiu_dn13,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign35760_e41155;
        locals.var_t5_dn0 = assign35760_e41155_d_n0;
        locals.var_t5_dn2 = assign35760_e41155_d_n2;
        locals.var_t5_dn4 = assign35760_e41155_d_n4;
        locals.var_t5_dn5 = assign35760_e41155_d_n5;
        locals.var_t5_dn6 = assign35760_e41155_d_n6;
        locals.var_t5_dn7 = assign35760_e41155_d_n7;
        locals.var_t5_dn8 = assign35760_e41155_d_n8;
        locals.var_t5_dn9 = assign35760_e41155_d_n9;
        locals.var_t5_dn10 = assign35760_e41155_d_n10;
        locals.var_t5_dn13 = assign35760_e41155_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign35770_e41163, assign35770_e41163_d_n0, assign35770_e41163_d_n2, assign35770_e41163_d_n4, assign35770_e41163_d_n5, assign35770_e41163_d_n6, assign35770_e41163_d_n7, assign35770_e41163_d_n8, assign35770_e41163_d_n9, assign35770_e41163_d_n10, assign35770_e41163_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign35770_e41161: f64 = (locals.var_t5 / locals.var_t4);
        (assign35770_e41161, (((locals.var_t5_dn0 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn2 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn4 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn5 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn6 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn7 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn8 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn9 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn10 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn13 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn13)) / (locals.var_t4 * locals.var_t4)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign35770_e41163;
        locals.var_t3_dn0 = assign35770_e41163_d_n0;
        locals.var_t3_dn2 = assign35770_e41163_d_n2;
        locals.var_t3_dn4 = assign35770_e41163_d_n4;
        locals.var_t3_dn5 = assign35770_e41163_d_n5;
        locals.var_t3_dn6 = assign35770_e41163_d_n6;
        locals.var_t3_dn7 = assign35770_e41163_d_n7;
        locals.var_t3_dn8 = assign35770_e41163_d_n8;
        locals.var_t3_dn9 = assign35770_e41163_d_n9;
        locals.var_t3_dn10 = assign35770_e41163_d_n10;
        locals.var_t3_dn13 = assign35770_e41163_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign35780_e41169, assign35780_e41169_d_n0, assign35780_e41169_d_n2, assign35780_e41169_d_n4, assign35780_e41169_d_n5, assign35780_e41169_d_n6, assign35780_e41169_d_n7, assign35780_e41169_d_n8, assign35780_e41169_d_n9, assign35780_e41169_d_n10, assign35780_e41169_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    } else {
        (locals.var_eeff, locals.var_eeff_dn0, locals.var_eeff_dn2, locals.var_eeff_dn4, locals.var_eeff_dn5, locals.var_eeff_dn6, locals.var_eeff_dn7, locals.var_eeff_dn8, locals.var_eeff_dn9, locals.var_eeff_dn10, locals.var_eeff_dn13,)
    }
};
        locals.var_eeff = assign35780_e41169;
        locals.var_eeff_dn0 = assign35780_e41169_d_n0;
        locals.var_eeff_dn2 = assign35780_e41169_d_n2;
        locals.var_eeff_dn4 = assign35780_e41169_d_n4;
        locals.var_eeff_dn5 = assign35780_e41169_d_n5;
        locals.var_eeff_dn6 = assign35780_e41169_d_n6;
        locals.var_eeff_dn7 = assign35780_e41169_d_n7;
        locals.var_eeff_dn8 = assign35780_e41169_d_n8;
        locals.var_eeff_dn9 = assign35780_e41169_d_n9;
        locals.var_eeff_dn10 = assign35780_e41169_d_n10;
        locals.var_eeff_dn13 = assign35780_e41169_d_n13;
        locals.var_eeff_rv = 0.0;

        let (assign35790_e41184, assign35790_e41184_d_n0, assign35790_e41184_d_n2, assign35790_e41184_d_n4, assign35790_e41184_d_n5, assign35790_e41184_d_n6, assign35790_e41184_d_n7, assign35790_e41184_d_n8, assign35790_e41184_d_n9, assign35790_e41184_d_n10, assign35790_e41184_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let (assign35790_e41182, assign35790_e41182_d_n0, assign35790_e41182_d_n2, assign35790_e41182_d_n4, assign35790_e41182_d_n5, assign35790_e41182_d_n6, assign35790_e41182_d_n7, assign35790_e41182_d_n8, assign35790_e41182_d_n9, assign35790_e41182_d_n10, assign35790_e41182_d_n13,) = {
            if (locals.var_eeff == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign35790_e41180: f64 = (p.p376 - 1.0);
                let assign35790_e41181: f64 = (locals.var_eeff).powf(assign35790_e41180);
                (assign35790_e41181, if 0.0 == 0.0 && ((assign35790_e41180) as f64).is_finite() && ((assign35790_e41180) as f64).fract() == 0.0 { if assign35790_e41180 == 0.0 { 0.0 } else { (assign35790_e41180 * ((locals.var_eeff).powf(assign35790_e41180 - 1.0) * locals.var_eeff_dn0)) } } else { (assign35790_e41181 * (assign35790_e41180 * (locals.var_eeff_dn0 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign35790_e41180) as f64).is_finite() && ((assign35790_e41180) as f64).fract() == 0.0 { if assign35790_e41180 == 0.0 { 0.0 } else { (assign35790_e41180 * ((locals.var_eeff).powf(assign35790_e41180 - 1.0) * locals.var_eeff_dn2)) } } else { (assign35790_e41181 * (assign35790_e41180 * (locals.var_eeff_dn2 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign35790_e41180) as f64).is_finite() && ((assign35790_e41180) as f64).fract() == 0.0 { if assign35790_e41180 == 0.0 { 0.0 } else { (assign35790_e41180 * ((locals.var_eeff).powf(assign35790_e41180 - 1.0) * locals.var_eeff_dn4)) } } else { (assign35790_e41181 * (assign35790_e41180 * (locals.var_eeff_dn4 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign35790_e41180) as f64).is_finite() && ((assign35790_e41180) as f64).fract() == 0.0 { if assign35790_e41180 == 0.0 { 0.0 } else { (assign35790_e41180 * ((locals.var_eeff).powf(assign35790_e41180 - 1.0) * locals.var_eeff_dn5)) } } else { (assign35790_e41181 * (assign35790_e41180 * (locals.var_eeff_dn5 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign35790_e41180) as f64).is_finite() && ((assign35790_e41180) as f64).fract() == 0.0 { if assign35790_e41180 == 0.0 { 0.0 } else { (assign35790_e41180 * ((locals.var_eeff).powf(assign35790_e41180 - 1.0) * locals.var_eeff_dn6)) } } else { (assign35790_e41181 * (assign35790_e41180 * (locals.var_eeff_dn6 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign35790_e41180) as f64).is_finite() && ((assign35790_e41180) as f64).fract() == 0.0 { if assign35790_e41180 == 0.0 { 0.0 } else { (assign35790_e41180 * ((locals.var_eeff).powf(assign35790_e41180 - 1.0) * locals.var_eeff_dn7)) } } else { (assign35790_e41181 * (assign35790_e41180 * (locals.var_eeff_dn7 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign35790_e41180) as f64).is_finite() && ((assign35790_e41180) as f64).fract() == 0.0 { if assign35790_e41180 == 0.0 { 0.0 } else { (assign35790_e41180 * ((locals.var_eeff).powf(assign35790_e41180 - 1.0) * locals.var_eeff_dn8)) } } else { (assign35790_e41181 * (assign35790_e41180 * (locals.var_eeff_dn8 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign35790_e41180) as f64).is_finite() && ((assign35790_e41180) as f64).fract() == 0.0 { if assign35790_e41180 == 0.0 { 0.0 } else { (assign35790_e41180 * ((locals.var_eeff).powf(assign35790_e41180 - 1.0) * locals.var_eeff_dn9)) } } else { (assign35790_e41181 * (assign35790_e41180 * (locals.var_eeff_dn9 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign35790_e41180) as f64).is_finite() && ((assign35790_e41180) as f64).fract() == 0.0 { if assign35790_e41180 == 0.0 { 0.0 } else { (assign35790_e41180 * ((locals.var_eeff).powf(assign35790_e41180 - 1.0) * locals.var_eeff_dn10)) } } else { (assign35790_e41181 * (assign35790_e41180 * (locals.var_eeff_dn10 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign35790_e41180) as f64).is_finite() && ((assign35790_e41180) as f64).fract() == 0.0 { if assign35790_e41180 == 0.0 { 0.0 } else { (assign35790_e41180 * ((locals.var_eeff).powf(assign35790_e41180 - 1.0) * locals.var_eeff_dn13)) } } else { (assign35790_e41181 * (assign35790_e41180 * (locals.var_eeff_dn13 / locals.var_eeff))) },)
            }
        };
        (assign35790_e41182, assign35790_e41182_d_n0, assign35790_e41182_d_n2, assign35790_e41182_d_n4, assign35790_e41182_d_n5, assign35790_e41182_d_n6, assign35790_e41182_d_n7, assign35790_e41182_d_n8, assign35790_e41182_d_n9, assign35790_e41182_d_n10, assign35790_e41182_d_n13,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign35790_e41184;
        locals.var_t5_dn0 = assign35790_e41184_d_n0;
        locals.var_t5_dn2 = assign35790_e41184_d_n2;
        locals.var_t5_dn4 = assign35790_e41184_d_n4;
        locals.var_t5_dn5 = assign35790_e41184_d_n5;
        locals.var_t5_dn6 = assign35790_e41184_d_n6;
        locals.var_t5_dn7 = assign35790_e41184_d_n7;
        locals.var_t5_dn8 = assign35790_e41184_d_n8;
        locals.var_t5_dn9 = assign35790_e41184_d_n9;
        locals.var_t5_dn10 = assign35790_e41184_d_n10;
        locals.var_t5_dn13 = assign35790_e41184_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign35800_e41192, assign35800_e41192_d_n0, assign35800_e41192_d_n2, assign35800_e41192_d_n4, assign35800_e41192_d_n5, assign35800_e41192_d_n6, assign35800_e41192_d_n7, assign35800_e41192_d_n8, assign35800_e41192_d_n9, assign35800_e41192_d_n10, assign35800_e41192_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign35800_e41190: f64 = (locals.var_t5 * locals.var_eeff);
        (assign35800_e41190, ((locals.var_t5_dn0 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn0)), ((locals.var_t5_dn2 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn2)), ((locals.var_t5_dn4 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn4)), ((locals.var_t5_dn5 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn5)), ((locals.var_t5_dn6 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn6)), ((locals.var_t5_dn7 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn7)), ((locals.var_t5_dn8 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn8)), ((locals.var_t5_dn9 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn9)), ((locals.var_t5_dn10 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn10)), ((locals.var_t5_dn13 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn13)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn13,)
    }
};
        locals.var_t8 = assign35800_e41192;
        locals.var_t8_dn0 = assign35800_e41192_d_n0;
        locals.var_t8_dn2 = assign35800_e41192_d_n2;
        locals.var_t8_dn4 = assign35800_e41192_d_n4;
        locals.var_t8_dn5 = assign35800_e41192_d_n5;
        locals.var_t8_dn6 = assign35800_e41192_d_n6;
        locals.var_t8_dn7 = assign35800_e41192_d_n7;
        locals.var_t8_dn8 = assign35800_e41192_d_n8;
        locals.var_t8_dn9 = assign35800_e41192_d_n9;
        locals.var_t8_dn10 = assign35800_e41192_d_n10;
        locals.var_t8_dn13 = assign35800_e41192_d_n13;
        locals.var_t8_rv = 0.0;

        let (assign35810_e41200, assign35810_e41200_d_n0, assign35810_e41200_d_n2, assign35810_e41200_d_n4, assign35810_e41200_d_n5, assign35810_e41200_d_n6, assign35810_e41200_d_n7, assign35810_e41200_d_n8, assign35810_e41200_d_n9, assign35810_e41200_d_n10, assign35810_e41200_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign35810_e41198: f64 = (1.6021918e-19 * 10000.0);
        (assign35810_e41198, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign35810_e41200;
        locals.var_t9_dn0 = assign35810_e41200_d_n0;
        locals.var_t9_dn2 = assign35810_e41200_d_n2;
        locals.var_t9_dn4 = assign35810_e41200_d_n4;
        locals.var_t9_dn5 = assign35810_e41200_d_n5;
        locals.var_t9_dn6 = assign35810_e41200_d_n6;
        locals.var_t9_dn7 = assign35810_e41200_d_n7;
        locals.var_t9_dn8 = assign35810_e41200_d_n8;
        locals.var_t9_dn9 = assign35810_e41200_d_n9;
        locals.var_t9_dn10 = assign35810_e41200_d_n10;
        locals.var_t9_dn13 = assign35810_e41200_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign35820_e41208, assign35820_e41208_d_n0, assign35820_e41208_d_n2, assign35820_e41208_d_n4, assign35820_e41208_d_n5, assign35820_e41208_d_n6, assign35820_e41208_d_n7, assign35820_e41208_d_n8, assign35820_e41208_d_n9, assign35820_e41208_d_n10, assign35820_e41208_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign35820_e41206: f64 = (locals.var_qiu / locals.var_t9);
        (assign35820_e41206, (((locals.var_qiu_dn0 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn0)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn2 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn2)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn4 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn4)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn5 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn5)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn6 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn6)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn7 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn7)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn8 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn8)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn9 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn9)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn10 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn10)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn13 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn13)) / (locals.var_t9 * locals.var_t9)),)
    } else {
        (locals.var_rns, locals.var_rns_dn0, locals.var_rns_dn2, locals.var_rns_dn4, locals.var_rns_dn5, locals.var_rns_dn6, locals.var_rns_dn7, locals.var_rns_dn8, locals.var_rns_dn9, locals.var_rns_dn10, locals.var_rns_dn13,)
    }
};
        locals.var_rns = assign35820_e41208;
        locals.var_rns_dn0 = assign35820_e41208_d_n0;
        locals.var_rns_dn2 = assign35820_e41208_d_n2;
        locals.var_rns_dn4 = assign35820_e41208_d_n4;
        locals.var_rns_dn5 = assign35820_e41208_d_n5;
        locals.var_rns_dn6 = assign35820_e41208_d_n6;
        locals.var_rns_dn7 = assign35820_e41208_d_n7;
        locals.var_rns_dn8 = assign35820_e41208_d_n8;
        locals.var_rns_dn9 = assign35820_e41208_d_n9;
        locals.var_rns_dn10 = assign35820_e41208_d_n10;
        locals.var_rns_dn13 = assign35820_e41208_d_n13;
        locals.var_rns_rv = 0.0;

        let (assign35830_e41228, assign35830_e41228_d_n0, assign35830_e41228_d_n2, assign35830_e41228_d_n4, assign35830_e41228_d_n5, assign35830_e41228_d_n6, assign35830_e41228_d_n7, assign35830_e41228_d_n8, assign35830_e41228_d_n9, assign35830_e41228_d_n10, assign35830_e41228_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign35830_e41216: f64 = (locals.var_uc_depmue1 * locals.var_rns);
        let assign35830_e41218: f64 = (assign35830_e41216 / 100000000000.0);
        let assign35830_e41219: f64 = (locals.var_uc_depmue0 + assign35830_e41218);
        let assign35830_e41221: f64 = (assign35830_e41219 + 1e-25);
        let assign35830_e41222: f64 = (1.0 / assign35830_e41221);
        let assign35830_e41225: f64 = (locals.var_depmphn0 * locals.var_t8);
        let assign35830_e41226: f64 = (assign35830_e41222 + assign35830_e41225);
        (assign35830_e41226, ((-((locals.var_uc_depmue0_dn0 + (((locals.var_uc_depmue1_dn0 * locals.var_rns) + (locals.var_uc_depmue1 * locals.var_rns_dn0)) / 100000000000.0)) / (assign35830_e41221 * assign35830_e41221))) + ((locals.var_depmphn0_dn0 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn0))), ((-((locals.var_uc_depmue0_dn2 + (((locals.var_uc_depmue1_dn2 * locals.var_rns) + (locals.var_uc_depmue1 * locals.var_rns_dn2)) / 100000000000.0)) / (assign35830_e41221 * assign35830_e41221))) + ((locals.var_depmphn0_dn2 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn2))), ((-((locals.var_uc_depmue0_dn4 + (((locals.var_uc_depmue1_dn4 * locals.var_rns) + (locals.var_uc_depmue1 * locals.var_rns_dn4)) / 100000000000.0)) / (assign35830_e41221 * assign35830_e41221))) + ((locals.var_depmphn0_dn4 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn4))), ((-((locals.var_uc_depmue0_dn5 + (((locals.var_uc_depmue1_dn5 * locals.var_rns) + (locals.var_uc_depmue1 * locals.var_rns_dn5)) / 100000000000.0)) / (assign35830_e41221 * assign35830_e41221))) + ((locals.var_depmphn0_dn5 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn5))), ((-((locals.var_uc_depmue0_dn6 + (((locals.var_uc_depmue1_dn6 * locals.var_rns) + (locals.var_uc_depmue1 * locals.var_rns_dn6)) / 100000000000.0)) / (assign35830_e41221 * assign35830_e41221))) + ((locals.var_depmphn0_dn6 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn6))), ((-((locals.var_uc_depmue0_dn7 + (((locals.var_uc_depmue1_dn7 * locals.var_rns) + (locals.var_uc_depmue1 * locals.var_rns_dn7)) / 100000000000.0)) / (assign35830_e41221 * assign35830_e41221))) + ((locals.var_depmphn0_dn7 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn7))), ((-((locals.var_uc_depmue0_dn8 + (((locals.var_uc_depmue1_dn8 * locals.var_rns) + (locals.var_uc_depmue1 * locals.var_rns_dn8)) / 100000000000.0)) / (assign35830_e41221 * assign35830_e41221))) + ((locals.var_depmphn0_dn8 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn8))), ((-((locals.var_uc_depmue0_dn9 + (((locals.var_uc_depmue1_dn9 * locals.var_rns) + (locals.var_uc_depmue1 * locals.var_rns_dn9)) / 100000000000.0)) / (assign35830_e41221 * assign35830_e41221))) + ((locals.var_depmphn0_dn9 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn9))), ((-((locals.var_uc_depmue0_dn10 + (((locals.var_uc_depmue1_dn10 * locals.var_rns) + (locals.var_uc_depmue1 * locals.var_rns_dn10)) / 100000000000.0)) / (assign35830_e41221 * assign35830_e41221))) + ((locals.var_depmphn0_dn10 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn10))), ((-((locals.var_uc_depmue0_dn13 + (((locals.var_uc_depmue1_dn13 * locals.var_rns) + (locals.var_uc_depmue1 * locals.var_rns_dn13)) / 100000000000.0)) / (assign35830_e41221 * assign35830_e41221))) + ((locals.var_depmphn0_dn13 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn13))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign35830_e41228;
        locals.var_t1_dn0 = assign35830_e41228_d_n0;
        locals.var_t1_dn2 = assign35830_e41228_d_n2;
        locals.var_t1_dn4 = assign35830_e41228_d_n4;
        locals.var_t1_dn5 = assign35830_e41228_d_n5;
        locals.var_t1_dn6 = assign35830_e41228_d_n6;
        locals.var_t1_dn7 = assign35830_e41228_d_n7;
        locals.var_t1_dn8 = assign35830_e41228_d_n8;
        locals.var_t1_dn9 = assign35830_e41228_d_n9;
        locals.var_t1_dn10 = assign35830_e41228_d_n10;
        locals.var_t1_dn13 = assign35830_e41228_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign35840_e41236, assign35840_e41236_d_n0, assign35840_e41236_d_n2, assign35840_e41236_d_n4, assign35840_e41236_d_n5, assign35840_e41236_d_n6, assign35840_e41236_d_n7, assign35840_e41236_d_n8, assign35840_e41236_d_n9, assign35840_e41236_d_n10, assign35840_e41236_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign35840_e41234: f64 = (1.0 / locals.var_t1);
        (assign35840_e41234, (-(locals.var_t1_dn0 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn2 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn4 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn5 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn6 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn7 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn8 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn9 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn10 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn13 / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn13,)
    }
};
        locals.var_muun = assign35840_e41236;
        locals.var_muun_dn0 = assign35840_e41236_d_n0;
        locals.var_muun_dn2 = assign35840_e41236_d_n2;
        locals.var_muun_dn4 = assign35840_e41236_d_n4;
        locals.var_muun_dn5 = assign35840_e41236_d_n5;
        locals.var_muun_dn6 = assign35840_e41236_d_n6;
        locals.var_muun_dn7 = assign35840_e41236_d_n7;
        locals.var_muun_dn8 = assign35840_e41236_d_n8;
        locals.var_muun_dn9 = assign35840_e41236_d_n9;
        locals.var_muun_dn10 = assign35840_e41236_d_n10;
        locals.var_muun_dn13 = assign35840_e41236_d_n13;
        locals.var_muun_rv = 0.0;

        let (assign35850_e41244, assign35850_e41244_d_n0, assign35850_e41244_d_n2, assign35850_e41244_d_n4, assign35850_e41244_d_n5, assign35850_e41244_d_n6, assign35850_e41244_d_n7, assign35850_e41244_d_n8, assign35850_e41244_d_n9, assign35850_e41244_d_n10, assign35850_e41244_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign35850_e41242: f64 = (locals.var_muun / 10000.0);
        (assign35850_e41242, (locals.var_muun_dn0 / 10000.0), (locals.var_muun_dn2 / 10000.0), (locals.var_muun_dn4 / 10000.0), (locals.var_muun_dn5 / 10000.0), (locals.var_muun_dn6 / 10000.0), (locals.var_muun_dn7 / 10000.0), (locals.var_muun_dn8 / 10000.0), (locals.var_muun_dn9 / 10000.0), (locals.var_muun_dn10 / 10000.0), (locals.var_muun_dn13 / 10000.0),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn13,)
    }
};
        locals.var_muun = assign35850_e41244;
        locals.var_muun_dn0 = assign35850_e41244_d_n0;
        locals.var_muun_dn2 = assign35850_e41244_d_n2;
        locals.var_muun_dn4 = assign35850_e41244_d_n4;
        locals.var_muun_dn5 = assign35850_e41244_d_n5;
        locals.var_muun_dn6 = assign35850_e41244_d_n6;
        locals.var_muun_dn7 = assign35850_e41244_d_n7;
        locals.var_muun_dn8 = assign35850_e41244_d_n8;
        locals.var_muun_dn9 = assign35850_e41244_d_n9;
        locals.var_muun_dn10 = assign35850_e41244_d_n10;
        locals.var_muun_dn13 = assign35850_e41244_d_n13;
        locals.var_muun_rv = 0.0;

        let (assign35860_e41252, assign35860_e41252_d_n0, assign35860_e41252_d_n2, assign35860_e41252_d_n4, assign35860_e41252_d_n5, assign35860_e41252_d_n6, assign35860_e41252_d_n7, assign35860_e41252_d_n8, assign35860_e41252_d_n9, assign35860_e41252_d_n10, assign35860_e41252_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign35860_e41250: f64 = (locals.var_vdseff0 / locals.var_lch);
        (assign35860_e41250, (((locals.var_vdseff0_dn0 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn2 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn4 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn5 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn6 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn7 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn8 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn9 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn9)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn10 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn13 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn13)) / (locals.var_lch * locals.var_lch)),)
    } else {
        (locals.var_edri__blk555, locals.var_edri__blk555_dn0, locals.var_edri__blk555_dn2, locals.var_edri__blk555_dn4, locals.var_edri__blk555_dn5, locals.var_edri__blk555_dn6, locals.var_edri__blk555_dn7, locals.var_edri__blk555_dn8, locals.var_edri__blk555_dn9, locals.var_edri__blk555_dn10, locals.var_edri__blk555_dn13,)
    }
};
        locals.var_edri__blk555 = assign35860_e41252;
        locals.var_edri__blk555_dn0 = assign35860_e41252_d_n0;
        locals.var_edri__blk555_dn2 = assign35860_e41252_d_n2;
        locals.var_edri__blk555_dn4 = assign35860_e41252_d_n4;
        locals.var_edri__blk555_dn5 = assign35860_e41252_d_n5;
        locals.var_edri__blk555_dn6 = assign35860_e41252_d_n6;
        locals.var_edri__blk555_dn7 = assign35860_e41252_d_n7;
        locals.var_edri__blk555_dn8 = assign35860_e41252_d_n8;
        locals.var_edri__blk555_dn9 = assign35860_e41252_d_n9;
        locals.var_edri__blk555_dn10 = assign35860_e41252_d_n10;
        locals.var_edri__blk555_dn13 = assign35860_e41252_d_n13;
        locals.var_edri__blk555_rv = 0.0;

        let (assign35870_e41262, assign35870_e41262_d_n0, assign35870_e41262_d_n2, assign35870_e41262_d_n4, assign35870_e41262_d_n5, assign35870_e41262_d_n6, assign35870_e41262_d_n7, assign35870_e41262_d_n8, assign35870_e41262_d_n9, assign35870_e41262_d_n10, assign35870_e41262_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign35870_e41258: f64 = (locals.var_muun * locals.var_edri__blk555);
        let assign35870_e41260: f64 = (assign35870_e41258 / locals.var_uc_depvmax);
        (assign35870_e41260, (((((locals.var_muun_dn0 * locals.var_edri__blk555) + (locals.var_muun * locals.var_edri__blk555_dn0)) * locals.var_uc_depvmax) - (assign35870_e41258 * locals.var_uc_depvmax_dn0)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn2 * locals.var_edri__blk555) + (locals.var_muun * locals.var_edri__blk555_dn2)) * locals.var_uc_depvmax) - (assign35870_e41258 * locals.var_uc_depvmax_dn2)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn4 * locals.var_edri__blk555) + (locals.var_muun * locals.var_edri__blk555_dn4)) * locals.var_uc_depvmax) - (assign35870_e41258 * locals.var_uc_depvmax_dn4)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn5 * locals.var_edri__blk555) + (locals.var_muun * locals.var_edri__blk555_dn5)) * locals.var_uc_depvmax) - (assign35870_e41258 * locals.var_uc_depvmax_dn5)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn6 * locals.var_edri__blk555) + (locals.var_muun * locals.var_edri__blk555_dn6)) * locals.var_uc_depvmax) - (assign35870_e41258 * locals.var_uc_depvmax_dn6)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn7 * locals.var_edri__blk555) + (locals.var_muun * locals.var_edri__blk555_dn7)) * locals.var_uc_depvmax) - (assign35870_e41258 * locals.var_uc_depvmax_dn7)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn8 * locals.var_edri__blk555) + (locals.var_muun * locals.var_edri__blk555_dn8)) * locals.var_uc_depvmax) - (assign35870_e41258 * locals.var_uc_depvmax_dn8)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn9 * locals.var_edri__blk555) + (locals.var_muun * locals.var_edri__blk555_dn9)) * locals.var_uc_depvmax) - (assign35870_e41258 * locals.var_uc_depvmax_dn9)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn10 * locals.var_edri__blk555) + (locals.var_muun * locals.var_edri__blk555_dn10)) * locals.var_uc_depvmax) - (assign35870_e41258 * locals.var_uc_depvmax_dn10)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn13 * locals.var_edri__blk555) + (locals.var_muun * locals.var_edri__blk555_dn13)) * locals.var_uc_depvmax) - (assign35870_e41258 * locals.var_uc_depvmax_dn13)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign35870_e41262;
        locals.var_t1_dn0 = assign35870_e41262_d_n0;
        locals.var_t1_dn2 = assign35870_e41262_d_n2;
        locals.var_t1_dn4 = assign35870_e41262_d_n4;
        locals.var_t1_dn5 = assign35870_e41262_d_n5;
        locals.var_t1_dn6 = assign35870_e41262_d_n6;
        locals.var_t1_dn7 = assign35870_e41262_d_n7;
        locals.var_t1_dn8 = assign35870_e41262_d_n8;
        locals.var_t1_dn9 = assign35870_e41262_d_n9;
        locals.var_t1_dn10 = assign35870_e41262_d_n10;
        locals.var_t1_dn13 = assign35870_e41262_d_n13;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_113(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign35880_e41275, assign35880_e41275_d_n0, assign35880_e41275_d_n2, assign35880_e41275_d_n4, assign35880_e41275_d_n5, assign35880_e41275_d_n6, assign35880_e41275_d_n7, assign35880_e41275_d_n8, assign35880_e41275_d_n9, assign35880_e41275_d_n10, assign35880_e41275_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let (assign35880_e41273, assign35880_e41273_d_n0, assign35880_e41273_d_n2, assign35880_e41273_d_n4, assign35880_e41273_d_n5, assign35880_e41273_d_n6, assign35880_e41273_d_n7, assign35880_e41273_d_n8, assign35880_e41273_d_n9, assign35880_e41273_d_n10, assign35880_e41273_d_n13,) = {
            if (locals.var_t1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign35880_e41272: f64 = (locals.var_t1).powf(p.p378);
                (assign35880_e41272, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn0)) } } else { (assign35880_e41272 * (p.p378 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn2)) } } else { (assign35880_e41272 * (p.p378 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn4)) } } else { (assign35880_e41272 * (p.p378 * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn5)) } } else { (assign35880_e41272 * (p.p378 * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn6)) } } else { (assign35880_e41272 * (p.p378 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn7)) } } else { (assign35880_e41272 * (p.p378 * (locals.var_t1_dn7 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn8)) } } else { (assign35880_e41272 * (p.p378 * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn9)) } } else { (assign35880_e41272 * (p.p378 * (locals.var_t1_dn9 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn10)) } } else { (assign35880_e41272 * (p.p378 * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn13)) } } else { (assign35880_e41272 * (p.p378 * (locals.var_t1_dn13 / locals.var_t1))) },)
            }
        };
        (assign35880_e41273, assign35880_e41273_d_n0, assign35880_e41273_d_n2, assign35880_e41273_d_n4, assign35880_e41273_d_n5, assign35880_e41273_d_n6, assign35880_e41273_d_n7, assign35880_e41273_d_n8, assign35880_e41273_d_n9, assign35880_e41273_d_n10, assign35880_e41273_d_n13,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign35880_e41275;
        locals.var_t2_dn0 = assign35880_e41275_d_n0;
        locals.var_t2_dn2 = assign35880_e41275_d_n2;
        locals.var_t2_dn4 = assign35880_e41275_d_n4;
        locals.var_t2_dn5 = assign35880_e41275_d_n5;
        locals.var_t2_dn6 = assign35880_e41275_d_n6;
        locals.var_t2_dn7 = assign35880_e41275_d_n7;
        locals.var_t2_dn8 = assign35880_e41275_d_n8;
        locals.var_t2_dn9 = assign35880_e41275_d_n9;
        locals.var_t2_dn10 = assign35880_e41275_d_n10;
        locals.var_t2_dn13 = assign35880_e41275_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign35890_e41283, assign35890_e41283_d_n0, assign35890_e41283_d_n2, assign35890_e41283_d_n4, assign35890_e41283_d_n5, assign35890_e41283_d_n6, assign35890_e41283_d_n7, assign35890_e41283_d_n8, assign35890_e41283_d_n9, assign35890_e41283_d_n10, assign35890_e41283_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign35890_e41281: f64 = (1.0 + locals.var_t2);
        (assign35890_e41281, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign35890_e41283;
        locals.var_t3_dn0 = assign35890_e41283_d_n0;
        locals.var_t3_dn2 = assign35890_e41283_d_n2;
        locals.var_t3_dn4 = assign35890_e41283_d_n4;
        locals.var_t3_dn5 = assign35890_e41283_d_n5;
        locals.var_t3_dn6 = assign35890_e41283_d_n6;
        locals.var_t3_dn7 = assign35890_e41283_d_n7;
        locals.var_t3_dn8 = assign35890_e41283_d_n8;
        locals.var_t3_dn9 = assign35890_e41283_d_n9;
        locals.var_t3_dn10 = assign35890_e41283_d_n10;
        locals.var_t3_dn13 = assign35890_e41283_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign35900_e41298, assign35900_e41298_d_n0, assign35900_e41298_d_n2, assign35900_e41298_d_n4, assign35900_e41298_d_n5, assign35900_e41298_d_n6, assign35900_e41298_d_n7, assign35900_e41298_d_n8, assign35900_e41298_d_n9, assign35900_e41298_d_n10, assign35900_e41298_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let (assign35900_e41296, assign35900_e41296_d_n0, assign35900_e41296_d_n2, assign35900_e41296_d_n4, assign35900_e41296_d_n5, assign35900_e41296_d_n6, assign35900_e41296_d_n7, assign35900_e41296_d_n8, assign35900_e41296_d_n9, assign35900_e41296_d_n10, assign35900_e41296_d_n13,) = {
            if (locals.var_t3 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign35900_e41294: f64 = (1.0 / p.p378);
                let assign35900_e41295: f64 = (locals.var_t3).powf(assign35900_e41294);
                (assign35900_e41295, if 0.0 == 0.0 && ((assign35900_e41294) as f64).is_finite() && ((assign35900_e41294) as f64).fract() == 0.0 { if assign35900_e41294 == 0.0 { 0.0 } else { (assign35900_e41294 * ((locals.var_t3).powf(assign35900_e41294 - 1.0) * locals.var_t3_dn0)) } } else { (assign35900_e41295 * (assign35900_e41294 * (locals.var_t3_dn0 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign35900_e41294) as f64).is_finite() && ((assign35900_e41294) as f64).fract() == 0.0 { if assign35900_e41294 == 0.0 { 0.0 } else { (assign35900_e41294 * ((locals.var_t3).powf(assign35900_e41294 - 1.0) * locals.var_t3_dn2)) } } else { (assign35900_e41295 * (assign35900_e41294 * (locals.var_t3_dn2 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign35900_e41294) as f64).is_finite() && ((assign35900_e41294) as f64).fract() == 0.0 { if assign35900_e41294 == 0.0 { 0.0 } else { (assign35900_e41294 * ((locals.var_t3).powf(assign35900_e41294 - 1.0) * locals.var_t3_dn4)) } } else { (assign35900_e41295 * (assign35900_e41294 * (locals.var_t3_dn4 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign35900_e41294) as f64).is_finite() && ((assign35900_e41294) as f64).fract() == 0.0 { if assign35900_e41294 == 0.0 { 0.0 } else { (assign35900_e41294 * ((locals.var_t3).powf(assign35900_e41294 - 1.0) * locals.var_t3_dn5)) } } else { (assign35900_e41295 * (assign35900_e41294 * (locals.var_t3_dn5 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign35900_e41294) as f64).is_finite() && ((assign35900_e41294) as f64).fract() == 0.0 { if assign35900_e41294 == 0.0 { 0.0 } else { (assign35900_e41294 * ((locals.var_t3).powf(assign35900_e41294 - 1.0) * locals.var_t3_dn6)) } } else { (assign35900_e41295 * (assign35900_e41294 * (locals.var_t3_dn6 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign35900_e41294) as f64).is_finite() && ((assign35900_e41294) as f64).fract() == 0.0 { if assign35900_e41294 == 0.0 { 0.0 } else { (assign35900_e41294 * ((locals.var_t3).powf(assign35900_e41294 - 1.0) * locals.var_t3_dn7)) } } else { (assign35900_e41295 * (assign35900_e41294 * (locals.var_t3_dn7 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign35900_e41294) as f64).is_finite() && ((assign35900_e41294) as f64).fract() == 0.0 { if assign35900_e41294 == 0.0 { 0.0 } else { (assign35900_e41294 * ((locals.var_t3).powf(assign35900_e41294 - 1.0) * locals.var_t3_dn8)) } } else { (assign35900_e41295 * (assign35900_e41294 * (locals.var_t3_dn8 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign35900_e41294) as f64).is_finite() && ((assign35900_e41294) as f64).fract() == 0.0 { if assign35900_e41294 == 0.0 { 0.0 } else { (assign35900_e41294 * ((locals.var_t3).powf(assign35900_e41294 - 1.0) * locals.var_t3_dn9)) } } else { (assign35900_e41295 * (assign35900_e41294 * (locals.var_t3_dn9 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign35900_e41294) as f64).is_finite() && ((assign35900_e41294) as f64).fract() == 0.0 { if assign35900_e41294 == 0.0 { 0.0 } else { (assign35900_e41294 * ((locals.var_t3).powf(assign35900_e41294 - 1.0) * locals.var_t3_dn10)) } } else { (assign35900_e41295 * (assign35900_e41294 * (locals.var_t3_dn10 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign35900_e41294) as f64).is_finite() && ((assign35900_e41294) as f64).fract() == 0.0 { if assign35900_e41294 == 0.0 { 0.0 } else { (assign35900_e41294 * ((locals.var_t3).powf(assign35900_e41294 - 1.0) * locals.var_t3_dn13)) } } else { (assign35900_e41295 * (assign35900_e41294 * (locals.var_t3_dn13 / locals.var_t3))) },)
            }
        };
        (assign35900_e41296, assign35900_e41296_d_n0, assign35900_e41296_d_n2, assign35900_e41296_d_n4, assign35900_e41296_d_n5, assign35900_e41296_d_n6, assign35900_e41296_d_n7, assign35900_e41296_d_n8, assign35900_e41296_d_n9, assign35900_e41296_d_n10, assign35900_e41296_d_n13,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign35900_e41298;
        locals.var_t4_dn0 = assign35900_e41298_d_n0;
        locals.var_t4_dn2 = assign35900_e41298_d_n2;
        locals.var_t4_dn4 = assign35900_e41298_d_n4;
        locals.var_t4_dn5 = assign35900_e41298_d_n5;
        locals.var_t4_dn6 = assign35900_e41298_d_n6;
        locals.var_t4_dn7 = assign35900_e41298_d_n7;
        locals.var_t4_dn8 = assign35900_e41298_d_n8;
        locals.var_t4_dn9 = assign35900_e41298_d_n9;
        locals.var_t4_dn10 = assign35900_e41298_d_n10;
        locals.var_t4_dn13 = assign35900_e41298_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign35910_e41306, assign35910_e41306_d_n0, assign35910_e41306_d_n2, assign35910_e41306_d_n4, assign35910_e41306_d_n5, assign35910_e41306_d_n6, assign35910_e41306_d_n7, assign35910_e41306_d_n8, assign35910_e41306_d_n9, assign35910_e41306_d_n10, assign35910_e41306_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign35910_e41304: f64 = (locals.var_muun / locals.var_t4);
        (assign35910_e41304, (((locals.var_muun_dn0 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn2 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn4 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn5 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn6 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn7 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn8 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn9 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn10 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn13 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn13)) / (locals.var_t4 * locals.var_t4)),)
    } else {
        (locals.var_mu_res__blk506, locals.var_mu_res__blk506_dn0, locals.var_mu_res__blk506_dn2, locals.var_mu_res__blk506_dn4, locals.var_mu_res__blk506_dn5, locals.var_mu_res__blk506_dn6, locals.var_mu_res__blk506_dn7, locals.var_mu_res__blk506_dn8, locals.var_mu_res__blk506_dn9, locals.var_mu_res__blk506_dn10, locals.var_mu_res__blk506_dn13,)
    }
};
        locals.var_mu_res__blk506 = assign35910_e41306;
        locals.var_mu_res__blk506_dn0 = assign35910_e41306_d_n0;
        locals.var_mu_res__blk506_dn2 = assign35910_e41306_d_n2;
        locals.var_mu_res__blk506_dn4 = assign35910_e41306_d_n4;
        locals.var_mu_res__blk506_dn5 = assign35910_e41306_d_n5;
        locals.var_mu_res__blk506_dn6 = assign35910_e41306_d_n6;
        locals.var_mu_res__blk506_dn7 = assign35910_e41306_d_n7;
        locals.var_mu_res__blk506_dn8 = assign35910_e41306_d_n8;
        locals.var_mu_res__blk506_dn9 = assign35910_e41306_d_n9;
        locals.var_mu_res__blk506_dn10 = assign35910_e41306_d_n10;
        locals.var_mu_res__blk506_dn13 = assign35910_e41306_d_n13;
        locals.var_mu_res__blk506_rv = 0.0;

        let (assign35920_e41319, assign35920_e41319_d_n0, assign35920_e41319_d_n2, assign35920_e41319_d_n4, assign35920_e41319_d_n5, assign35920_e41319_d_n6, assign35920_e41319_d_n7, assign35920_e41319_d_n8, assign35920_e41319_d_n9, assign35920_e41319_d_n10, assign35920_e41319_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign35920_e41312: f64 = (-locals.var_qn_res0);
        let assign35920_e41313: f64 = (locals.var_weff_nf * assign35920_e41312);
        let assign35920_e41315: f64 = (assign35920_e41313 * locals.var_mu_res__blk506);
        let assign35920_e41317: f64 = (assign35920_e41315 * locals.var_edri__blk555);
        (assign35920_e41317, (((((locals.var_weff_nf * (-locals.var_qn_res0_dn0)) * locals.var_mu_res__blk506) + (assign35920_e41313 * locals.var_mu_res__blk506_dn0)) * locals.var_edri__blk555) + (assign35920_e41315 * locals.var_edri__blk555_dn0)), (((((locals.var_weff_nf * (-locals.var_qn_res0_dn2)) * locals.var_mu_res__blk506) + (assign35920_e41313 * locals.var_mu_res__blk506_dn2)) * locals.var_edri__blk555) + (assign35920_e41315 * locals.var_edri__blk555_dn2)), (((((locals.var_weff_nf * (-locals.var_qn_res0_dn4)) * locals.var_mu_res__blk506) + (assign35920_e41313 * locals.var_mu_res__blk506_dn4)) * locals.var_edri__blk555) + (assign35920_e41315 * locals.var_edri__blk555_dn4)), (((((locals.var_weff_nf * (-locals.var_qn_res0_dn5)) * locals.var_mu_res__blk506) + (assign35920_e41313 * locals.var_mu_res__blk506_dn5)) * locals.var_edri__blk555) + (assign35920_e41315 * locals.var_edri__blk555_dn5)), (((((locals.var_weff_nf * (-locals.var_qn_res0_dn6)) * locals.var_mu_res__blk506) + (assign35920_e41313 * locals.var_mu_res__blk506_dn6)) * locals.var_edri__blk555) + (assign35920_e41315 * locals.var_edri__blk555_dn6)), (((((locals.var_weff_nf * (-locals.var_qn_res0_dn7)) * locals.var_mu_res__blk506) + (assign35920_e41313 * locals.var_mu_res__blk506_dn7)) * locals.var_edri__blk555) + (assign35920_e41315 * locals.var_edri__blk555_dn7)), (((((locals.var_weff_nf * (-locals.var_qn_res0_dn8)) * locals.var_mu_res__blk506) + (assign35920_e41313 * locals.var_mu_res__blk506_dn8)) * locals.var_edri__blk555) + (assign35920_e41315 * locals.var_edri__blk555_dn8)), (((((locals.var_weff_nf * (-locals.var_qn_res0_dn9)) * locals.var_mu_res__blk506) + (assign35920_e41313 * locals.var_mu_res__blk506_dn9)) * locals.var_edri__blk555) + (assign35920_e41315 * locals.var_edri__blk555_dn9)), (((((locals.var_weff_nf * (-locals.var_qn_res0_dn10)) * locals.var_mu_res__blk506) + (assign35920_e41313 * locals.var_mu_res__blk506_dn10)) * locals.var_edri__blk555) + (assign35920_e41315 * locals.var_edri__blk555_dn10)), (((((locals.var_weff_nf * (-locals.var_qn_res0_dn13)) * locals.var_mu_res__blk506) + (assign35920_e41313 * locals.var_mu_res__blk506_dn13)) * locals.var_edri__blk555) + (assign35920_e41315 * locals.var_edri__blk555_dn13)),)
    } else {
        (locals.var_ids_res, locals.var_ids_res_dn0, locals.var_ids_res_dn2, locals.var_ids_res_dn4, locals.var_ids_res_dn5, locals.var_ids_res_dn6, locals.var_ids_res_dn7, locals.var_ids_res_dn8, locals.var_ids_res_dn9, locals.var_ids_res_dn10, locals.var_ids_res_dn13,)
    }
};
        locals.var_ids_res = assign35920_e41319;
        locals.var_ids_res_dn0 = assign35920_e41319_d_n0;
        locals.var_ids_res_dn2 = assign35920_e41319_d_n2;
        locals.var_ids_res_dn4 = assign35920_e41319_d_n4;
        locals.var_ids_res_dn5 = assign35920_e41319_d_n5;
        locals.var_ids_res_dn6 = assign35920_e41319_d_n6;
        locals.var_ids_res_dn7 = assign35920_e41319_d_n7;
        locals.var_ids_res_dn8 = assign35920_e41319_d_n8;
        locals.var_ids_res_dn9 = assign35920_e41319_d_n9;
        locals.var_ids_res_dn10 = assign35920_e41319_d_n10;
        locals.var_ids_res_dn13 = assign35920_e41319_d_n13;
        locals.var_ids_res_rv = 0.0;

        let (assign35930_e41331, assign35930_e41331_d_n0, assign35930_e41331_d_n2, assign35930_e41331_d_n4, assign35930_e41331_d_n5, assign35930_e41331_d_n6, assign35930_e41331_d_n7, assign35930_e41331_d_n8, assign35930_e41331_d_n9, assign35930_e41331_d_n10, assign35930_e41331_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign35930_e41326: f64 = (locals.var_phi_sl_dep - locals.var_phi_s0_dep);
        let assign35930_e41328: f64 = (assign35930_e41326 * locals.var_ninvde);
        let assign35930_e41329: f64 = (1.0 + assign35930_e41328);
        (assign35930_e41329, (((locals.var_phi_sl_dep_dn0 - locals.var_phi_s0_dep_dn0) * locals.var_ninvde) + (assign35930_e41326 * locals.var_ninvde_dn0)), (((locals.var_phi_sl_dep_dn2 - locals.var_phi_s0_dep_dn2) * locals.var_ninvde) + (assign35930_e41326 * locals.var_ninvde_dn2)), (((locals.var_phi_sl_dep_dn4 - locals.var_phi_s0_dep_dn4) * locals.var_ninvde) + (assign35930_e41326 * locals.var_ninvde_dn4)), (((locals.var_phi_sl_dep_dn5 - locals.var_phi_s0_dep_dn5) * locals.var_ninvde) + (assign35930_e41326 * locals.var_ninvde_dn5)), (((locals.var_phi_sl_dep_dn6 - locals.var_phi_s0_dep_dn6) * locals.var_ninvde) + (assign35930_e41326 * locals.var_ninvde_dn6)), (((locals.var_phi_sl_dep_dn7 - locals.var_phi_s0_dep_dn7) * locals.var_ninvde) + (assign35930_e41326 * locals.var_ninvde_dn7)), (((locals.var_phi_sl_dep_dn8 - locals.var_phi_s0_dep_dn8) * locals.var_ninvde) + (assign35930_e41326 * locals.var_ninvde_dn8)), (((locals.var_phi_sl_dep_dn9 - locals.var_phi_s0_dep_dn9) * locals.var_ninvde) + (assign35930_e41326 * locals.var_ninvde_dn9)), (((locals.var_phi_sl_dep_dn10 - locals.var_phi_s0_dep_dn10) * locals.var_ninvde) + (assign35930_e41326 * locals.var_ninvde_dn10)), (((locals.var_phi_sl_dep_dn13 - locals.var_phi_s0_dep_dn13) * locals.var_ninvde) + (assign35930_e41326 * locals.var_ninvde_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign35930_e41331;
        locals.var_t4_dn0 = assign35930_e41331_d_n0;
        locals.var_t4_dn2 = assign35930_e41331_d_n2;
        locals.var_t4_dn4 = assign35930_e41331_d_n4;
        locals.var_t4_dn5 = assign35930_e41331_d_n5;
        locals.var_t4_dn6 = assign35930_e41331_d_n6;
        locals.var_t4_dn7 = assign35930_e41331_d_n7;
        locals.var_t4_dn8 = assign35930_e41331_d_n8;
        locals.var_t4_dn9 = assign35930_e41331_d_n9;
        locals.var_t4_dn10 = assign35930_e41331_d_n10;
        locals.var_t4_dn13 = assign35930_e41331_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign35940_e41338, assign35940_e41338_d_n0, assign35940_e41338_d_n2, assign35940_e41338_d_n4, assign35940_e41338_d_n5, assign35940_e41338_d_n6, assign35940_e41338_d_n7, assign35940_e41338_d_n8, assign35940_e41338_d_n9, assign35940_e41338_d_n10, assign35940_e41338_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign35940_e41336: f64 = (-locals.var_qn_bac);
        (assign35940_e41336, (-locals.var_qn_bac_dn0), (-locals.var_qn_bac_dn2), (-locals.var_qn_bac_dn4), (-locals.var_qn_bac_dn5), (-locals.var_qn_bac_dn6), (-locals.var_qn_bac_dn7), (-locals.var_qn_bac_dn8), (-locals.var_qn_bac_dn9), (-locals.var_qn_bac_dn10), (-locals.var_qn_bac_dn13),)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn4, locals.var_qiu_dn5, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn8, locals.var_qiu_dn9, locals.var_qiu_dn10, locals.var_qiu_dn13,)
    }
};
        locals.var_qiu = assign35940_e41338;
        locals.var_qiu_dn0 = assign35940_e41338_d_n0;
        locals.var_qiu_dn2 = assign35940_e41338_d_n2;
        locals.var_qiu_dn4 = assign35940_e41338_d_n4;
        locals.var_qiu_dn5 = assign35940_e41338_d_n5;
        locals.var_qiu_dn6 = assign35940_e41338_d_n6;
        locals.var_qiu_dn7 = assign35940_e41338_d_n7;
        locals.var_qiu_dn8 = assign35940_e41338_d_n8;
        locals.var_qiu_dn9 = assign35940_e41338_d_n9;
        locals.var_qiu_dn10 = assign35940_e41338_d_n10;
        locals.var_qiu_dn13 = assign35940_e41338_d_n13;
        locals.var_qiu_rv = 0.0;

        let (assign35950_e41344, assign35950_e41344_d_n0, assign35950_e41344_d_n2, assign35950_e41344_d_n4, assign35950_e41344_d_n5, assign35950_e41344_d_n6, assign35950_e41344_d_n7, assign35950_e41344_d_n8, assign35950_e41344_d_n9, assign35950_e41344_d_n10, assign35950_e41344_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn4, locals.var_qiu_dn5, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn8, locals.var_qiu_dn9, locals.var_qiu_dn10, locals.var_qiu_dn13,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign35950_e41344;
        locals.var_t5_dn0 = assign35950_e41344_d_n0;
        locals.var_t5_dn2 = assign35950_e41344_d_n2;
        locals.var_t5_dn4 = assign35950_e41344_d_n4;
        locals.var_t5_dn5 = assign35950_e41344_d_n5;
        locals.var_t5_dn6 = assign35950_e41344_d_n6;
        locals.var_t5_dn7 = assign35950_e41344_d_n7;
        locals.var_t5_dn8 = assign35950_e41344_d_n8;
        locals.var_t5_dn9 = assign35950_e41344_d_n9;
        locals.var_t5_dn10 = assign35950_e41344_d_n10;
        locals.var_t5_dn13 = assign35950_e41344_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign35960_e41352, assign35960_e41352_d_n0, assign35960_e41352_d_n2, assign35960_e41352_d_n4, assign35960_e41352_d_n5, assign35960_e41352_d_n6, assign35960_e41352_d_n7, assign35960_e41352_d_n8, assign35960_e41352_d_n9, assign35960_e41352_d_n10, assign35960_e41352_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign35960_e41350: f64 = (locals.var_t5 / locals.var_t4);
        (assign35960_e41350, (((locals.var_t5_dn0 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn2 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn4 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn5 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn6 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn7 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn8 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn9 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn10 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn13 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn13)) / (locals.var_t4 * locals.var_t4)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign35960_e41352;
        locals.var_t3_dn0 = assign35960_e41352_d_n0;
        locals.var_t3_dn2 = assign35960_e41352_d_n2;
        locals.var_t3_dn4 = assign35960_e41352_d_n4;
        locals.var_t3_dn5 = assign35960_e41352_d_n5;
        locals.var_t3_dn6 = assign35960_e41352_d_n6;
        locals.var_t3_dn7 = assign35960_e41352_d_n7;
        locals.var_t3_dn8 = assign35960_e41352_d_n8;
        locals.var_t3_dn9 = assign35960_e41352_d_n9;
        locals.var_t3_dn10 = assign35960_e41352_d_n10;
        locals.var_t3_dn13 = assign35960_e41352_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign35970_e41358, assign35970_e41358_d_n0, assign35970_e41358_d_n2, assign35970_e41358_d_n4, assign35970_e41358_d_n5, assign35970_e41358_d_n6, assign35970_e41358_d_n7, assign35970_e41358_d_n8, assign35970_e41358_d_n9, assign35970_e41358_d_n10, assign35970_e41358_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    } else {
        (locals.var_eeff, locals.var_eeff_dn0, locals.var_eeff_dn2, locals.var_eeff_dn4, locals.var_eeff_dn5, locals.var_eeff_dn6, locals.var_eeff_dn7, locals.var_eeff_dn8, locals.var_eeff_dn9, locals.var_eeff_dn10, locals.var_eeff_dn13,)
    }
};
        locals.var_eeff = assign35970_e41358;
        locals.var_eeff_dn0 = assign35970_e41358_d_n0;
        locals.var_eeff_dn2 = assign35970_e41358_d_n2;
        locals.var_eeff_dn4 = assign35970_e41358_d_n4;
        locals.var_eeff_dn5 = assign35970_e41358_d_n5;
        locals.var_eeff_dn6 = assign35970_e41358_d_n6;
        locals.var_eeff_dn7 = assign35970_e41358_d_n7;
        locals.var_eeff_dn8 = assign35970_e41358_d_n8;
        locals.var_eeff_dn9 = assign35970_e41358_d_n9;
        locals.var_eeff_dn10 = assign35970_e41358_d_n10;
        locals.var_eeff_dn13 = assign35970_e41358_d_n13;
        locals.var_eeff_rv = 0.0;

        let (assign35980_e41373, assign35980_e41373_d_n0, assign35980_e41373_d_n2, assign35980_e41373_d_n4, assign35980_e41373_d_n5, assign35980_e41373_d_n6, assign35980_e41373_d_n7, assign35980_e41373_d_n8, assign35980_e41373_d_n9, assign35980_e41373_d_n10, assign35980_e41373_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let (assign35980_e41371, assign35980_e41371_d_n0, assign35980_e41371_d_n2, assign35980_e41371_d_n4, assign35980_e41371_d_n5, assign35980_e41371_d_n6, assign35980_e41371_d_n7, assign35980_e41371_d_n8, assign35980_e41371_d_n9, assign35980_e41371_d_n10, assign35980_e41371_d_n13,) = {
            if (locals.var_eeff == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign35980_e41369: f64 = (p.p376 - 1.0);
                let assign35980_e41370: f64 = (locals.var_eeff).powf(assign35980_e41369);
                (assign35980_e41370, if 0.0 == 0.0 && ((assign35980_e41369) as f64).is_finite() && ((assign35980_e41369) as f64).fract() == 0.0 { if assign35980_e41369 == 0.0 { 0.0 } else { (assign35980_e41369 * ((locals.var_eeff).powf(assign35980_e41369 - 1.0) * locals.var_eeff_dn0)) } } else { (assign35980_e41370 * (assign35980_e41369 * (locals.var_eeff_dn0 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign35980_e41369) as f64).is_finite() && ((assign35980_e41369) as f64).fract() == 0.0 { if assign35980_e41369 == 0.0 { 0.0 } else { (assign35980_e41369 * ((locals.var_eeff).powf(assign35980_e41369 - 1.0) * locals.var_eeff_dn2)) } } else { (assign35980_e41370 * (assign35980_e41369 * (locals.var_eeff_dn2 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign35980_e41369) as f64).is_finite() && ((assign35980_e41369) as f64).fract() == 0.0 { if assign35980_e41369 == 0.0 { 0.0 } else { (assign35980_e41369 * ((locals.var_eeff).powf(assign35980_e41369 - 1.0) * locals.var_eeff_dn4)) } } else { (assign35980_e41370 * (assign35980_e41369 * (locals.var_eeff_dn4 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign35980_e41369) as f64).is_finite() && ((assign35980_e41369) as f64).fract() == 0.0 { if assign35980_e41369 == 0.0 { 0.0 } else { (assign35980_e41369 * ((locals.var_eeff).powf(assign35980_e41369 - 1.0) * locals.var_eeff_dn5)) } } else { (assign35980_e41370 * (assign35980_e41369 * (locals.var_eeff_dn5 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign35980_e41369) as f64).is_finite() && ((assign35980_e41369) as f64).fract() == 0.0 { if assign35980_e41369 == 0.0 { 0.0 } else { (assign35980_e41369 * ((locals.var_eeff).powf(assign35980_e41369 - 1.0) * locals.var_eeff_dn6)) } } else { (assign35980_e41370 * (assign35980_e41369 * (locals.var_eeff_dn6 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign35980_e41369) as f64).is_finite() && ((assign35980_e41369) as f64).fract() == 0.0 { if assign35980_e41369 == 0.0 { 0.0 } else { (assign35980_e41369 * ((locals.var_eeff).powf(assign35980_e41369 - 1.0) * locals.var_eeff_dn7)) } } else { (assign35980_e41370 * (assign35980_e41369 * (locals.var_eeff_dn7 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign35980_e41369) as f64).is_finite() && ((assign35980_e41369) as f64).fract() == 0.0 { if assign35980_e41369 == 0.0 { 0.0 } else { (assign35980_e41369 * ((locals.var_eeff).powf(assign35980_e41369 - 1.0) * locals.var_eeff_dn8)) } } else { (assign35980_e41370 * (assign35980_e41369 * (locals.var_eeff_dn8 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign35980_e41369) as f64).is_finite() && ((assign35980_e41369) as f64).fract() == 0.0 { if assign35980_e41369 == 0.0 { 0.0 } else { (assign35980_e41369 * ((locals.var_eeff).powf(assign35980_e41369 - 1.0) * locals.var_eeff_dn9)) } } else { (assign35980_e41370 * (assign35980_e41369 * (locals.var_eeff_dn9 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign35980_e41369) as f64).is_finite() && ((assign35980_e41369) as f64).fract() == 0.0 { if assign35980_e41369 == 0.0 { 0.0 } else { (assign35980_e41369 * ((locals.var_eeff).powf(assign35980_e41369 - 1.0) * locals.var_eeff_dn10)) } } else { (assign35980_e41370 * (assign35980_e41369 * (locals.var_eeff_dn10 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign35980_e41369) as f64).is_finite() && ((assign35980_e41369) as f64).fract() == 0.0 { if assign35980_e41369 == 0.0 { 0.0 } else { (assign35980_e41369 * ((locals.var_eeff).powf(assign35980_e41369 - 1.0) * locals.var_eeff_dn13)) } } else { (assign35980_e41370 * (assign35980_e41369 * (locals.var_eeff_dn13 / locals.var_eeff))) },)
            }
        };
        (assign35980_e41371, assign35980_e41371_d_n0, assign35980_e41371_d_n2, assign35980_e41371_d_n4, assign35980_e41371_d_n5, assign35980_e41371_d_n6, assign35980_e41371_d_n7, assign35980_e41371_d_n8, assign35980_e41371_d_n9, assign35980_e41371_d_n10, assign35980_e41371_d_n13,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign35980_e41373;
        locals.var_t5_dn0 = assign35980_e41373_d_n0;
        locals.var_t5_dn2 = assign35980_e41373_d_n2;
        locals.var_t5_dn4 = assign35980_e41373_d_n4;
        locals.var_t5_dn5 = assign35980_e41373_d_n5;
        locals.var_t5_dn6 = assign35980_e41373_d_n6;
        locals.var_t5_dn7 = assign35980_e41373_d_n7;
        locals.var_t5_dn8 = assign35980_e41373_d_n8;
        locals.var_t5_dn9 = assign35980_e41373_d_n9;
        locals.var_t5_dn10 = assign35980_e41373_d_n10;
        locals.var_t5_dn13 = assign35980_e41373_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign35990_e41381, assign35990_e41381_d_n0, assign35990_e41381_d_n2, assign35990_e41381_d_n4, assign35990_e41381_d_n5, assign35990_e41381_d_n6, assign35990_e41381_d_n7, assign35990_e41381_d_n8, assign35990_e41381_d_n9, assign35990_e41381_d_n10, assign35990_e41381_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign35990_e41379: f64 = (locals.var_t5 * locals.var_eeff);
        (assign35990_e41379, ((locals.var_t5_dn0 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn0)), ((locals.var_t5_dn2 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn2)), ((locals.var_t5_dn4 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn4)), ((locals.var_t5_dn5 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn5)), ((locals.var_t5_dn6 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn6)), ((locals.var_t5_dn7 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn7)), ((locals.var_t5_dn8 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn8)), ((locals.var_t5_dn9 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn9)), ((locals.var_t5_dn10 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn10)), ((locals.var_t5_dn13 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn13)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn13,)
    }
};
        locals.var_t8 = assign35990_e41381;
        locals.var_t8_dn0 = assign35990_e41381_d_n0;
        locals.var_t8_dn2 = assign35990_e41381_d_n2;
        locals.var_t8_dn4 = assign35990_e41381_d_n4;
        locals.var_t8_dn5 = assign35990_e41381_d_n5;
        locals.var_t8_dn6 = assign35990_e41381_d_n6;
        locals.var_t8_dn7 = assign35990_e41381_d_n7;
        locals.var_t8_dn8 = assign35990_e41381_d_n8;
        locals.var_t8_dn9 = assign35990_e41381_d_n9;
        locals.var_t8_dn10 = assign35990_e41381_d_n10;
        locals.var_t8_dn13 = assign35990_e41381_d_n13;
        locals.var_t8_rv = 0.0;

        let (assign36000_e41389, assign36000_e41389_d_n0, assign36000_e41389_d_n2, assign36000_e41389_d_n4, assign36000_e41389_d_n5, assign36000_e41389_d_n6, assign36000_e41389_d_n7, assign36000_e41389_d_n8, assign36000_e41389_d_n9, assign36000_e41389_d_n10, assign36000_e41389_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign36000_e41387: f64 = (1.6021918e-19 * 10000.0);
        (assign36000_e41387, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign36000_e41389;
        locals.var_t9_dn0 = assign36000_e41389_d_n0;
        locals.var_t9_dn2 = assign36000_e41389_d_n2;
        locals.var_t9_dn4 = assign36000_e41389_d_n4;
        locals.var_t9_dn5 = assign36000_e41389_d_n5;
        locals.var_t9_dn6 = assign36000_e41389_d_n6;
        locals.var_t9_dn7 = assign36000_e41389_d_n7;
        locals.var_t9_dn8 = assign36000_e41389_d_n8;
        locals.var_t9_dn9 = assign36000_e41389_d_n9;
        locals.var_t9_dn10 = assign36000_e41389_d_n10;
        locals.var_t9_dn13 = assign36000_e41389_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign36010_e41397, assign36010_e41397_d_n0, assign36010_e41397_d_n2, assign36010_e41397_d_n4, assign36010_e41397_d_n5, assign36010_e41397_d_n6, assign36010_e41397_d_n7, assign36010_e41397_d_n8, assign36010_e41397_d_n9, assign36010_e41397_d_n10, assign36010_e41397_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign36010_e41395: f64 = (locals.var_qiu / locals.var_t9);
        (assign36010_e41395, (((locals.var_qiu_dn0 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn0)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn2 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn2)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn4 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn4)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn5 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn5)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn6 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn6)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn7 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn7)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn8 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn8)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn9 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn9)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn10 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn10)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qiu_dn13 * locals.var_t9) - (locals.var_qiu * locals.var_t9_dn13)) / (locals.var_t9 * locals.var_t9)),)
    } else {
        (locals.var_rns, locals.var_rns_dn0, locals.var_rns_dn2, locals.var_rns_dn4, locals.var_rns_dn5, locals.var_rns_dn6, locals.var_rns_dn7, locals.var_rns_dn8, locals.var_rns_dn9, locals.var_rns_dn10, locals.var_rns_dn13,)
    }
};
        locals.var_rns = assign36010_e41397;
        locals.var_rns_dn0 = assign36010_e41397_d_n0;
        locals.var_rns_dn2 = assign36010_e41397_d_n2;
        locals.var_rns_dn4 = assign36010_e41397_d_n4;
        locals.var_rns_dn5 = assign36010_e41397_d_n5;
        locals.var_rns_dn6 = assign36010_e41397_d_n6;
        locals.var_rns_dn7 = assign36010_e41397_d_n7;
        locals.var_rns_dn8 = assign36010_e41397_d_n8;
        locals.var_rns_dn9 = assign36010_e41397_d_n9;
        locals.var_rns_dn10 = assign36010_e41397_d_n10;
        locals.var_rns_dn13 = assign36010_e41397_d_n13;
        locals.var_rns_rv = 0.0;

        let (assign36020_e41417, assign36020_e41417_d_n0, assign36020_e41417_d_n2, assign36020_e41417_d_n4, assign36020_e41417_d_n5, assign36020_e41417_d_n6, assign36020_e41417_d_n7, assign36020_e41417_d_n8, assign36020_e41417_d_n9, assign36020_e41417_d_n10, assign36020_e41417_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign36020_e41405: f64 = (locals.var_uc_depmueback1 * locals.var_rns);
        let assign36020_e41407: f64 = (assign36020_e41405 / 100000000000.0);
        let assign36020_e41408: f64 = (locals.var_uc_depmueback0 + assign36020_e41407);
        let assign36020_e41410: f64 = (assign36020_e41408 + 1e-25);
        let assign36020_e41411: f64 = (1.0 / assign36020_e41410);
        let assign36020_e41414: f64 = (locals.var_depmphn0 * locals.var_t8);
        let assign36020_e41415: f64 = (assign36020_e41411 + assign36020_e41414);
        (assign36020_e41415, ((-((locals.var_uc_depmueback0_dn0 + (((locals.var_uc_depmueback1_dn0 * locals.var_rns) + (locals.var_uc_depmueback1 * locals.var_rns_dn0)) / 100000000000.0)) / (assign36020_e41410 * assign36020_e41410))) + ((locals.var_depmphn0_dn0 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn0))), ((-((locals.var_uc_depmueback0_dn2 + (((locals.var_uc_depmueback1_dn2 * locals.var_rns) + (locals.var_uc_depmueback1 * locals.var_rns_dn2)) / 100000000000.0)) / (assign36020_e41410 * assign36020_e41410))) + ((locals.var_depmphn0_dn2 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn2))), ((-((locals.var_uc_depmueback0_dn4 + (((locals.var_uc_depmueback1_dn4 * locals.var_rns) + (locals.var_uc_depmueback1 * locals.var_rns_dn4)) / 100000000000.0)) / (assign36020_e41410 * assign36020_e41410))) + ((locals.var_depmphn0_dn4 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn4))), ((-((locals.var_uc_depmueback0_dn5 + (((locals.var_uc_depmueback1_dn5 * locals.var_rns) + (locals.var_uc_depmueback1 * locals.var_rns_dn5)) / 100000000000.0)) / (assign36020_e41410 * assign36020_e41410))) + ((locals.var_depmphn0_dn5 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn5))), ((-((locals.var_uc_depmueback0_dn6 + (((locals.var_uc_depmueback1_dn6 * locals.var_rns) + (locals.var_uc_depmueback1 * locals.var_rns_dn6)) / 100000000000.0)) / (assign36020_e41410 * assign36020_e41410))) + ((locals.var_depmphn0_dn6 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn6))), ((-((locals.var_uc_depmueback0_dn7 + (((locals.var_uc_depmueback1_dn7 * locals.var_rns) + (locals.var_uc_depmueback1 * locals.var_rns_dn7)) / 100000000000.0)) / (assign36020_e41410 * assign36020_e41410))) + ((locals.var_depmphn0_dn7 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn7))), ((-((locals.var_uc_depmueback0_dn8 + (((locals.var_uc_depmueback1_dn8 * locals.var_rns) + (locals.var_uc_depmueback1 * locals.var_rns_dn8)) / 100000000000.0)) / (assign36020_e41410 * assign36020_e41410))) + ((locals.var_depmphn0_dn8 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn8))), ((-((locals.var_uc_depmueback0_dn9 + (((locals.var_uc_depmueback1_dn9 * locals.var_rns) + (locals.var_uc_depmueback1 * locals.var_rns_dn9)) / 100000000000.0)) / (assign36020_e41410 * assign36020_e41410))) + ((locals.var_depmphn0_dn9 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn9))), ((-((locals.var_uc_depmueback0_dn10 + (((locals.var_uc_depmueback1_dn10 * locals.var_rns) + (locals.var_uc_depmueback1 * locals.var_rns_dn10)) / 100000000000.0)) / (assign36020_e41410 * assign36020_e41410))) + ((locals.var_depmphn0_dn10 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn10))), ((-((locals.var_uc_depmueback0_dn13 + (((locals.var_uc_depmueback1_dn13 * locals.var_rns) + (locals.var_uc_depmueback1 * locals.var_rns_dn13)) / 100000000000.0)) / (assign36020_e41410 * assign36020_e41410))) + ((locals.var_depmphn0_dn13 * locals.var_t8) + (locals.var_depmphn0 * locals.var_t8_dn13))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign36020_e41417;
        locals.var_t1_dn0 = assign36020_e41417_d_n0;
        locals.var_t1_dn2 = assign36020_e41417_d_n2;
        locals.var_t1_dn4 = assign36020_e41417_d_n4;
        locals.var_t1_dn5 = assign36020_e41417_d_n5;
        locals.var_t1_dn6 = assign36020_e41417_d_n6;
        locals.var_t1_dn7 = assign36020_e41417_d_n7;
        locals.var_t1_dn8 = assign36020_e41417_d_n8;
        locals.var_t1_dn9 = assign36020_e41417_d_n9;
        locals.var_t1_dn10 = assign36020_e41417_d_n10;
        locals.var_t1_dn13 = assign36020_e41417_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign36030_e41425, assign36030_e41425_d_n0, assign36030_e41425_d_n2, assign36030_e41425_d_n4, assign36030_e41425_d_n5, assign36030_e41425_d_n6, assign36030_e41425_d_n7, assign36030_e41425_d_n8, assign36030_e41425_d_n9, assign36030_e41425_d_n10, assign36030_e41425_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign36030_e41423: f64 = (1.0 / locals.var_t1);
        (assign36030_e41423, (-(locals.var_t1_dn0 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn2 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn4 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn5 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn6 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn7 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn8 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn9 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn10 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn13 / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn13,)
    }
};
        locals.var_muun = assign36030_e41425;
        locals.var_muun_dn0 = assign36030_e41425_d_n0;
        locals.var_muun_dn2 = assign36030_e41425_d_n2;
        locals.var_muun_dn4 = assign36030_e41425_d_n4;
        locals.var_muun_dn5 = assign36030_e41425_d_n5;
        locals.var_muun_dn6 = assign36030_e41425_d_n6;
        locals.var_muun_dn7 = assign36030_e41425_d_n7;
        locals.var_muun_dn8 = assign36030_e41425_d_n8;
        locals.var_muun_dn9 = assign36030_e41425_d_n9;
        locals.var_muun_dn10 = assign36030_e41425_d_n10;
        locals.var_muun_dn13 = assign36030_e41425_d_n13;
        locals.var_muun_rv = 0.0;

        let (assign36040_e41433, assign36040_e41433_d_n0, assign36040_e41433_d_n2, assign36040_e41433_d_n4, assign36040_e41433_d_n5, assign36040_e41433_d_n6, assign36040_e41433_d_n7, assign36040_e41433_d_n8, assign36040_e41433_d_n9, assign36040_e41433_d_n10, assign36040_e41433_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign36040_e41431: f64 = (locals.var_muun / 10000.0);
        (assign36040_e41431, (locals.var_muun_dn0 / 10000.0), (locals.var_muun_dn2 / 10000.0), (locals.var_muun_dn4 / 10000.0), (locals.var_muun_dn5 / 10000.0), (locals.var_muun_dn6 / 10000.0), (locals.var_muun_dn7 / 10000.0), (locals.var_muun_dn8 / 10000.0), (locals.var_muun_dn9 / 10000.0), (locals.var_muun_dn10 / 10000.0), (locals.var_muun_dn13 / 10000.0),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn13,)
    }
};
        locals.var_muun = assign36040_e41433;
        locals.var_muun_dn0 = assign36040_e41433_d_n0;
        locals.var_muun_dn2 = assign36040_e41433_d_n2;
        locals.var_muun_dn4 = assign36040_e41433_d_n4;
        locals.var_muun_dn5 = assign36040_e41433_d_n5;
        locals.var_muun_dn6 = assign36040_e41433_d_n6;
        locals.var_muun_dn7 = assign36040_e41433_d_n7;
        locals.var_muun_dn8 = assign36040_e41433_d_n8;
        locals.var_muun_dn9 = assign36040_e41433_d_n9;
        locals.var_muun_dn10 = assign36040_e41433_d_n10;
        locals.var_muun_dn13 = assign36040_e41433_d_n13;
        locals.var_muun_rv = 0.0;

        let (assign36050_e41441, assign36050_e41441_d_n0, assign36050_e41441_d_n2, assign36050_e41441_d_n4, assign36050_e41441_d_n5, assign36050_e41441_d_n6, assign36050_e41441_d_n7, assign36050_e41441_d_n8, assign36050_e41441_d_n9, assign36050_e41441_d_n10, assign36050_e41441_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign36050_e41439: f64 = (locals.var_vdseff0 / locals.var_lch);
        (assign36050_e41439, (((locals.var_vdseff0_dn0 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn2 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn4 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn5 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn6 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn7 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn8 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn9 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn9)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn10 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)), (((locals.var_vdseff0_dn13 * locals.var_lch) - (locals.var_vdseff0 * locals.var_lch_dn13)) / (locals.var_lch * locals.var_lch)),)
    } else {
        (locals.var_edri__blk555, locals.var_edri__blk555_dn0, locals.var_edri__blk555_dn2, locals.var_edri__blk555_dn4, locals.var_edri__blk555_dn5, locals.var_edri__blk555_dn6, locals.var_edri__blk555_dn7, locals.var_edri__blk555_dn8, locals.var_edri__blk555_dn9, locals.var_edri__blk555_dn10, locals.var_edri__blk555_dn13,)
    }
};
        locals.var_edri__blk555 = assign36050_e41441;
        locals.var_edri__blk555_dn0 = assign36050_e41441_d_n0;
        locals.var_edri__blk555_dn2 = assign36050_e41441_d_n2;
        locals.var_edri__blk555_dn4 = assign36050_e41441_d_n4;
        locals.var_edri__blk555_dn5 = assign36050_e41441_d_n5;
        locals.var_edri__blk555_dn6 = assign36050_e41441_d_n6;
        locals.var_edri__blk555_dn7 = assign36050_e41441_d_n7;
        locals.var_edri__blk555_dn8 = assign36050_e41441_d_n8;
        locals.var_edri__blk555_dn9 = assign36050_e41441_d_n9;
        locals.var_edri__blk555_dn10 = assign36050_e41441_d_n10;
        locals.var_edri__blk555_dn13 = assign36050_e41441_d_n13;
        locals.var_edri__blk555_rv = 0.0;

        let (assign36060_e41451, assign36060_e41451_d_n0, assign36060_e41451_d_n2, assign36060_e41451_d_n4, assign36060_e41451_d_n5, assign36060_e41451_d_n6, assign36060_e41451_d_n7, assign36060_e41451_d_n8, assign36060_e41451_d_n9, assign36060_e41451_d_n10, assign36060_e41451_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign36060_e41447: f64 = (locals.var_muun * locals.var_edri__blk555);
        let assign36060_e41449: f64 = (assign36060_e41447 / locals.var_uc_depvmax);
        (assign36060_e41449, (((((locals.var_muun_dn0 * locals.var_edri__blk555) + (locals.var_muun * locals.var_edri__blk555_dn0)) * locals.var_uc_depvmax) - (assign36060_e41447 * locals.var_uc_depvmax_dn0)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn2 * locals.var_edri__blk555) + (locals.var_muun * locals.var_edri__blk555_dn2)) * locals.var_uc_depvmax) - (assign36060_e41447 * locals.var_uc_depvmax_dn2)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn4 * locals.var_edri__blk555) + (locals.var_muun * locals.var_edri__blk555_dn4)) * locals.var_uc_depvmax) - (assign36060_e41447 * locals.var_uc_depvmax_dn4)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn5 * locals.var_edri__blk555) + (locals.var_muun * locals.var_edri__blk555_dn5)) * locals.var_uc_depvmax) - (assign36060_e41447 * locals.var_uc_depvmax_dn5)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn6 * locals.var_edri__blk555) + (locals.var_muun * locals.var_edri__blk555_dn6)) * locals.var_uc_depvmax) - (assign36060_e41447 * locals.var_uc_depvmax_dn6)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn7 * locals.var_edri__blk555) + (locals.var_muun * locals.var_edri__blk555_dn7)) * locals.var_uc_depvmax) - (assign36060_e41447 * locals.var_uc_depvmax_dn7)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn8 * locals.var_edri__blk555) + (locals.var_muun * locals.var_edri__blk555_dn8)) * locals.var_uc_depvmax) - (assign36060_e41447 * locals.var_uc_depvmax_dn8)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn9 * locals.var_edri__blk555) + (locals.var_muun * locals.var_edri__blk555_dn9)) * locals.var_uc_depvmax) - (assign36060_e41447 * locals.var_uc_depvmax_dn9)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn10 * locals.var_edri__blk555) + (locals.var_muun * locals.var_edri__blk555_dn10)) * locals.var_uc_depvmax) - (assign36060_e41447 * locals.var_uc_depvmax_dn10)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)), (((((locals.var_muun_dn13 * locals.var_edri__blk555) + (locals.var_muun * locals.var_edri__blk555_dn13)) * locals.var_uc_depvmax) - (assign36060_e41447 * locals.var_uc_depvmax_dn13)) / (locals.var_uc_depvmax * locals.var_uc_depvmax)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign36060_e41451;
        locals.var_t1_dn0 = assign36060_e41451_d_n0;
        locals.var_t1_dn2 = assign36060_e41451_d_n2;
        locals.var_t1_dn4 = assign36060_e41451_d_n4;
        locals.var_t1_dn5 = assign36060_e41451_d_n5;
        locals.var_t1_dn6 = assign36060_e41451_d_n6;
        locals.var_t1_dn7 = assign36060_e41451_d_n7;
        locals.var_t1_dn8 = assign36060_e41451_d_n8;
        locals.var_t1_dn9 = assign36060_e41451_d_n9;
        locals.var_t1_dn10 = assign36060_e41451_d_n10;
        locals.var_t1_dn13 = assign36060_e41451_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign36070_e41464, assign36070_e41464_d_n0, assign36070_e41464_d_n2, assign36070_e41464_d_n4, assign36070_e41464_d_n5, assign36070_e41464_d_n6, assign36070_e41464_d_n7, assign36070_e41464_d_n8, assign36070_e41464_d_n9, assign36070_e41464_d_n10, assign36070_e41464_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let (assign36070_e41462, assign36070_e41462_d_n0, assign36070_e41462_d_n2, assign36070_e41462_d_n4, assign36070_e41462_d_n5, assign36070_e41462_d_n6, assign36070_e41462_d_n7, assign36070_e41462_d_n8, assign36070_e41462_d_n9, assign36070_e41462_d_n10, assign36070_e41462_d_n13,) = {
            if (locals.var_t1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign36070_e41461: f64 = (locals.var_t1).powf(p.p378);
                (assign36070_e41461, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn0)) } } else { (assign36070_e41461 * (p.p378 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn2)) } } else { (assign36070_e41461 * (p.p378 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn4)) } } else { (assign36070_e41461 * (p.p378 * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn5)) } } else { (assign36070_e41461 * (p.p378 * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn6)) } } else { (assign36070_e41461 * (p.p378 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn7)) } } else { (assign36070_e41461 * (p.p378 * (locals.var_t1_dn7 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn8)) } } else { (assign36070_e41461 * (p.p378 * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn9)) } } else { (assign36070_e41461 * (p.p378 * (locals.var_t1_dn9 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn10)) } } else { (assign36070_e41461 * (p.p378 * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((p.p378) as f64).is_finite() && ((p.p378) as f64).fract() == 0.0 { if p.p378 == 0.0 { 0.0 } else { (p.p378 * ((locals.var_t1).powf(p.p378 - 1.0) * locals.var_t1_dn13)) } } else { (assign36070_e41461 * (p.p378 * (locals.var_t1_dn13 / locals.var_t1))) },)
            }
        };
        (assign36070_e41462, assign36070_e41462_d_n0, assign36070_e41462_d_n2, assign36070_e41462_d_n4, assign36070_e41462_d_n5, assign36070_e41462_d_n6, assign36070_e41462_d_n7, assign36070_e41462_d_n8, assign36070_e41462_d_n9, assign36070_e41462_d_n10, assign36070_e41462_d_n13,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign36070_e41464;
        locals.var_t2_dn0 = assign36070_e41464_d_n0;
        locals.var_t2_dn2 = assign36070_e41464_d_n2;
        locals.var_t2_dn4 = assign36070_e41464_d_n4;
        locals.var_t2_dn5 = assign36070_e41464_d_n5;
        locals.var_t2_dn6 = assign36070_e41464_d_n6;
        locals.var_t2_dn7 = assign36070_e41464_d_n7;
        locals.var_t2_dn8 = assign36070_e41464_d_n8;
        locals.var_t2_dn9 = assign36070_e41464_d_n9;
        locals.var_t2_dn10 = assign36070_e41464_d_n10;
        locals.var_t2_dn13 = assign36070_e41464_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign36080_e41472, assign36080_e41472_d_n0, assign36080_e41472_d_n2, assign36080_e41472_d_n4, assign36080_e41472_d_n5, assign36080_e41472_d_n6, assign36080_e41472_d_n7, assign36080_e41472_d_n8, assign36080_e41472_d_n9, assign36080_e41472_d_n10, assign36080_e41472_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign36080_e41470: f64 = (1.0 + locals.var_t2);
        (assign36080_e41470, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign36080_e41472;
        locals.var_t3_dn0 = assign36080_e41472_d_n0;
        locals.var_t3_dn2 = assign36080_e41472_d_n2;
        locals.var_t3_dn4 = assign36080_e41472_d_n4;
        locals.var_t3_dn5 = assign36080_e41472_d_n5;
        locals.var_t3_dn6 = assign36080_e41472_d_n6;
        locals.var_t3_dn7 = assign36080_e41472_d_n7;
        locals.var_t3_dn8 = assign36080_e41472_d_n8;
        locals.var_t3_dn9 = assign36080_e41472_d_n9;
        locals.var_t3_dn10 = assign36080_e41472_d_n10;
        locals.var_t3_dn13 = assign36080_e41472_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign36090_e41487, assign36090_e41487_d_n0, assign36090_e41487_d_n2, assign36090_e41487_d_n4, assign36090_e41487_d_n5, assign36090_e41487_d_n6, assign36090_e41487_d_n7, assign36090_e41487_d_n8, assign36090_e41487_d_n9, assign36090_e41487_d_n10, assign36090_e41487_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let (assign36090_e41485, assign36090_e41485_d_n0, assign36090_e41485_d_n2, assign36090_e41485_d_n4, assign36090_e41485_d_n5, assign36090_e41485_d_n6, assign36090_e41485_d_n7, assign36090_e41485_d_n8, assign36090_e41485_d_n9, assign36090_e41485_d_n10, assign36090_e41485_d_n13,) = {
            if (locals.var_t3 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign36090_e41483: f64 = (1.0 / p.p378);
                let assign36090_e41484: f64 = (locals.var_t3).powf(assign36090_e41483);
                (assign36090_e41484, if 0.0 == 0.0 && ((assign36090_e41483) as f64).is_finite() && ((assign36090_e41483) as f64).fract() == 0.0 { if assign36090_e41483 == 0.0 { 0.0 } else { (assign36090_e41483 * ((locals.var_t3).powf(assign36090_e41483 - 1.0) * locals.var_t3_dn0)) } } else { (assign36090_e41484 * (assign36090_e41483 * (locals.var_t3_dn0 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign36090_e41483) as f64).is_finite() && ((assign36090_e41483) as f64).fract() == 0.0 { if assign36090_e41483 == 0.0 { 0.0 } else { (assign36090_e41483 * ((locals.var_t3).powf(assign36090_e41483 - 1.0) * locals.var_t3_dn2)) } } else { (assign36090_e41484 * (assign36090_e41483 * (locals.var_t3_dn2 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign36090_e41483) as f64).is_finite() && ((assign36090_e41483) as f64).fract() == 0.0 { if assign36090_e41483 == 0.0 { 0.0 } else { (assign36090_e41483 * ((locals.var_t3).powf(assign36090_e41483 - 1.0) * locals.var_t3_dn4)) } } else { (assign36090_e41484 * (assign36090_e41483 * (locals.var_t3_dn4 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign36090_e41483) as f64).is_finite() && ((assign36090_e41483) as f64).fract() == 0.0 { if assign36090_e41483 == 0.0 { 0.0 } else { (assign36090_e41483 * ((locals.var_t3).powf(assign36090_e41483 - 1.0) * locals.var_t3_dn5)) } } else { (assign36090_e41484 * (assign36090_e41483 * (locals.var_t3_dn5 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign36090_e41483) as f64).is_finite() && ((assign36090_e41483) as f64).fract() == 0.0 { if assign36090_e41483 == 0.0 { 0.0 } else { (assign36090_e41483 * ((locals.var_t3).powf(assign36090_e41483 - 1.0) * locals.var_t3_dn6)) } } else { (assign36090_e41484 * (assign36090_e41483 * (locals.var_t3_dn6 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign36090_e41483) as f64).is_finite() && ((assign36090_e41483) as f64).fract() == 0.0 { if assign36090_e41483 == 0.0 { 0.0 } else { (assign36090_e41483 * ((locals.var_t3).powf(assign36090_e41483 - 1.0) * locals.var_t3_dn7)) } } else { (assign36090_e41484 * (assign36090_e41483 * (locals.var_t3_dn7 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign36090_e41483) as f64).is_finite() && ((assign36090_e41483) as f64).fract() == 0.0 { if assign36090_e41483 == 0.0 { 0.0 } else { (assign36090_e41483 * ((locals.var_t3).powf(assign36090_e41483 - 1.0) * locals.var_t3_dn8)) } } else { (assign36090_e41484 * (assign36090_e41483 * (locals.var_t3_dn8 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign36090_e41483) as f64).is_finite() && ((assign36090_e41483) as f64).fract() == 0.0 { if assign36090_e41483 == 0.0 { 0.0 } else { (assign36090_e41483 * ((locals.var_t3).powf(assign36090_e41483 - 1.0) * locals.var_t3_dn9)) } } else { (assign36090_e41484 * (assign36090_e41483 * (locals.var_t3_dn9 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign36090_e41483) as f64).is_finite() && ((assign36090_e41483) as f64).fract() == 0.0 { if assign36090_e41483 == 0.0 { 0.0 } else { (assign36090_e41483 * ((locals.var_t3).powf(assign36090_e41483 - 1.0) * locals.var_t3_dn10)) } } else { (assign36090_e41484 * (assign36090_e41483 * (locals.var_t3_dn10 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign36090_e41483) as f64).is_finite() && ((assign36090_e41483) as f64).fract() == 0.0 { if assign36090_e41483 == 0.0 { 0.0 } else { (assign36090_e41483 * ((locals.var_t3).powf(assign36090_e41483 - 1.0) * locals.var_t3_dn13)) } } else { (assign36090_e41484 * (assign36090_e41483 * (locals.var_t3_dn13 / locals.var_t3))) },)
            }
        };
        (assign36090_e41485, assign36090_e41485_d_n0, assign36090_e41485_d_n2, assign36090_e41485_d_n4, assign36090_e41485_d_n5, assign36090_e41485_d_n6, assign36090_e41485_d_n7, assign36090_e41485_d_n8, assign36090_e41485_d_n9, assign36090_e41485_d_n10, assign36090_e41485_d_n13,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign36090_e41487;
        locals.var_t4_dn0 = assign36090_e41487_d_n0;
        locals.var_t4_dn2 = assign36090_e41487_d_n2;
        locals.var_t4_dn4 = assign36090_e41487_d_n4;
        locals.var_t4_dn5 = assign36090_e41487_d_n5;
        locals.var_t4_dn6 = assign36090_e41487_d_n6;
        locals.var_t4_dn7 = assign36090_e41487_d_n7;
        locals.var_t4_dn8 = assign36090_e41487_d_n8;
        locals.var_t4_dn9 = assign36090_e41487_d_n9;
        locals.var_t4_dn10 = assign36090_e41487_d_n10;
        locals.var_t4_dn13 = assign36090_e41487_d_n13;
        locals.var_t4_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_114(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign36100_e41495, assign36100_e41495_d_n0, assign36100_e41495_d_n2, assign36100_e41495_d_n4, assign36100_e41495_d_n5, assign36100_e41495_d_n6, assign36100_e41495_d_n7, assign36100_e41495_d_n8, assign36100_e41495_d_n9, assign36100_e41495_d_n10, assign36100_e41495_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign36100_e41493: f64 = (locals.var_muun / locals.var_t4);
        (assign36100_e41493, (((locals.var_muun_dn0 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn2 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn4 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn5 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn6 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn7 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn8 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn9 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn10 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)), (((locals.var_muun_dn13 * locals.var_t4) - (locals.var_muun * locals.var_t4_dn13)) / (locals.var_t4 * locals.var_t4)),)
    } else {
        (locals.var_mu_bac, locals.var_mu_bac_dn0, locals.var_mu_bac_dn2, locals.var_mu_bac_dn4, locals.var_mu_bac_dn5, locals.var_mu_bac_dn6, locals.var_mu_bac_dn7, locals.var_mu_bac_dn8, locals.var_mu_bac_dn9, locals.var_mu_bac_dn10, locals.var_mu_bac_dn13,)
    }
};
        locals.var_mu_bac = assign36100_e41495;
        locals.var_mu_bac_dn0 = assign36100_e41495_d_n0;
        locals.var_mu_bac_dn2 = assign36100_e41495_d_n2;
        locals.var_mu_bac_dn4 = assign36100_e41495_d_n4;
        locals.var_mu_bac_dn5 = assign36100_e41495_d_n5;
        locals.var_mu_bac_dn6 = assign36100_e41495_d_n6;
        locals.var_mu_bac_dn7 = assign36100_e41495_d_n7;
        locals.var_mu_bac_dn8 = assign36100_e41495_d_n8;
        locals.var_mu_bac_dn9 = assign36100_e41495_d_n9;
        locals.var_mu_bac_dn10 = assign36100_e41495_d_n10;
        locals.var_mu_bac_dn13 = assign36100_e41495_d_n13;
        locals.var_mu_bac_rv = 0.0;

        let (assign36110_e41508, assign36110_e41508_d_n0, assign36110_e41508_d_n2, assign36110_e41508_d_n4, assign36110_e41508_d_n5, assign36110_e41508_d_n6, assign36110_e41508_d_n7, assign36110_e41508_d_n8, assign36110_e41508_d_n9, assign36110_e41508_d_n10, assign36110_e41508_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign36110_e41501: f64 = (-locals.var_qn_bac);
        let assign36110_e41502: f64 = (locals.var_weff_nf * assign36110_e41501);
        let assign36110_e41504: f64 = (assign36110_e41502 * locals.var_mu_bac);
        let assign36110_e41506: f64 = (assign36110_e41504 * locals.var_edri__blk555);
        (assign36110_e41506, (((((locals.var_weff_nf * (-locals.var_qn_bac_dn0)) * locals.var_mu_bac) + (assign36110_e41502 * locals.var_mu_bac_dn0)) * locals.var_edri__blk555) + (assign36110_e41504 * locals.var_edri__blk555_dn0)), (((((locals.var_weff_nf * (-locals.var_qn_bac_dn2)) * locals.var_mu_bac) + (assign36110_e41502 * locals.var_mu_bac_dn2)) * locals.var_edri__blk555) + (assign36110_e41504 * locals.var_edri__blk555_dn2)), (((((locals.var_weff_nf * (-locals.var_qn_bac_dn4)) * locals.var_mu_bac) + (assign36110_e41502 * locals.var_mu_bac_dn4)) * locals.var_edri__blk555) + (assign36110_e41504 * locals.var_edri__blk555_dn4)), (((((locals.var_weff_nf * (-locals.var_qn_bac_dn5)) * locals.var_mu_bac) + (assign36110_e41502 * locals.var_mu_bac_dn5)) * locals.var_edri__blk555) + (assign36110_e41504 * locals.var_edri__blk555_dn5)), (((((locals.var_weff_nf * (-locals.var_qn_bac_dn6)) * locals.var_mu_bac) + (assign36110_e41502 * locals.var_mu_bac_dn6)) * locals.var_edri__blk555) + (assign36110_e41504 * locals.var_edri__blk555_dn6)), (((((locals.var_weff_nf * (-locals.var_qn_bac_dn7)) * locals.var_mu_bac) + (assign36110_e41502 * locals.var_mu_bac_dn7)) * locals.var_edri__blk555) + (assign36110_e41504 * locals.var_edri__blk555_dn7)), (((((locals.var_weff_nf * (-locals.var_qn_bac_dn8)) * locals.var_mu_bac) + (assign36110_e41502 * locals.var_mu_bac_dn8)) * locals.var_edri__blk555) + (assign36110_e41504 * locals.var_edri__blk555_dn8)), (((((locals.var_weff_nf * (-locals.var_qn_bac_dn9)) * locals.var_mu_bac) + (assign36110_e41502 * locals.var_mu_bac_dn9)) * locals.var_edri__blk555) + (assign36110_e41504 * locals.var_edri__blk555_dn9)), (((((locals.var_weff_nf * (-locals.var_qn_bac_dn10)) * locals.var_mu_bac) + (assign36110_e41502 * locals.var_mu_bac_dn10)) * locals.var_edri__blk555) + (assign36110_e41504 * locals.var_edri__blk555_dn10)), (((((locals.var_weff_nf * (-locals.var_qn_bac_dn13)) * locals.var_mu_bac) + (assign36110_e41502 * locals.var_mu_bac_dn13)) * locals.var_edri__blk555) + (assign36110_e41504 * locals.var_edri__blk555_dn13)),)
    } else {
        (locals.var_ids_bac, locals.var_ids_bac_dn0, locals.var_ids_bac_dn2, locals.var_ids_bac_dn4, locals.var_ids_bac_dn5, locals.var_ids_bac_dn6, locals.var_ids_bac_dn7, locals.var_ids_bac_dn8, locals.var_ids_bac_dn9, locals.var_ids_bac_dn10, locals.var_ids_bac_dn13,)
    }
};
        locals.var_ids_bac = assign36110_e41508;
        locals.var_ids_bac_dn0 = assign36110_e41508_d_n0;
        locals.var_ids_bac_dn2 = assign36110_e41508_d_n2;
        locals.var_ids_bac_dn4 = assign36110_e41508_d_n4;
        locals.var_ids_bac_dn5 = assign36110_e41508_d_n5;
        locals.var_ids_bac_dn6 = assign36110_e41508_d_n6;
        locals.var_ids_bac_dn7 = assign36110_e41508_d_n7;
        locals.var_ids_bac_dn8 = assign36110_e41508_d_n8;
        locals.var_ids_bac_dn9 = assign36110_e41508_d_n9;
        locals.var_ids_bac_dn10 = assign36110_e41508_d_n10;
        locals.var_ids_bac_dn13 = assign36110_e41508_d_n13;
        locals.var_ids_bac_rv = 0.0;

        let (assign36120_e41518, assign36120_e41518_d_n0, assign36120_e41518_d_n2, assign36120_e41518_d_n4, assign36120_e41518_d_n5, assign36120_e41518_d_n6, assign36120_e41518_d_n7, assign36120_e41518_d_n8, assign36120_e41518_d_n9, assign36120_e41518_d_n10, assign36120_e41518_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign36120_e41514: f64 = (locals.var_weff_nf * locals.var_beta_inv);
        let assign36120_e41516: f64 = (assign36120_e41514 / locals.var_lch);
        (assign36120_e41516, ((((locals.var_weff_nf * locals.var_beta_inv_dn0) * locals.var_lch) - (assign36120_e41514 * locals.var_lch_dn0)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn2) * locals.var_lch) - (assign36120_e41514 * locals.var_lch_dn2)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn4) * locals.var_lch) - (assign36120_e41514 * locals.var_lch_dn4)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn5) * locals.var_lch) - (assign36120_e41514 * locals.var_lch_dn5)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn6) * locals.var_lch) - (assign36120_e41514 * locals.var_lch_dn6)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn7) * locals.var_lch) - (assign36120_e41514 * locals.var_lch_dn7)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn8) * locals.var_lch) - (assign36120_e41514 * locals.var_lch_dn8)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn9) * locals.var_lch) - (assign36120_e41514 * locals.var_lch_dn9)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn10) * locals.var_lch) - (assign36120_e41514 * locals.var_lch_dn10)) / (locals.var_lch * locals.var_lch)), ((((locals.var_weff_nf * locals.var_beta_inv_dn13) * locals.var_lch) - (assign36120_e41514 * locals.var_lch_dn13)) / (locals.var_lch * locals.var_lch)),)
    } else {
        (locals.var_betawl, locals.var_betawl_dn0, locals.var_betawl_dn2, locals.var_betawl_dn4, locals.var_betawl_dn5, locals.var_betawl_dn6, locals.var_betawl_dn7, locals.var_betawl_dn8, locals.var_betawl_dn9, locals.var_betawl_dn10, locals.var_betawl_dn13,)
    }
};
        locals.var_betawl = assign36120_e41518;
        locals.var_betawl_dn0 = assign36120_e41518_d_n0;
        locals.var_betawl_dn2 = assign36120_e41518_d_n2;
        locals.var_betawl_dn4 = assign36120_e41518_d_n4;
        locals.var_betawl_dn5 = assign36120_e41518_d_n5;
        locals.var_betawl_dn6 = assign36120_e41518_d_n6;
        locals.var_betawl_dn7 = assign36120_e41518_d_n7;
        locals.var_betawl_dn8 = assign36120_e41518_d_n8;
        locals.var_betawl_dn9 = assign36120_e41518_d_n9;
        locals.var_betawl_dn10 = assign36120_e41518_d_n10;
        locals.var_betawl_dn13 = assign36120_e41518_d_n13;
        locals.var_betawl_rv = 0.0;

        let (assign36130_e41532, assign36130_e41532_d_n0, assign36130_e41532_d_n2, assign36130_e41532_d_n4, assign36130_e41532_d_n5, assign36130_e41532_d_n6, assign36130_e41532_d_n7, assign36130_e41532_d_n8, assign36130_e41532_d_n9, assign36130_e41532_d_n10, assign36130_e41532_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign36130_e41524: f64 = (locals.var_betawl * locals.var_idd);
        let assign36130_e41526: f64 = (assign36130_e41524 * locals.var_mu);
        let assign36130_e41528: f64 = (assign36130_e41526 + locals.var_ids_res);
        let assign36130_e41530: f64 = (assign36130_e41528 + locals.var_ids_bac);
        (assign36130_e41530, ((((((locals.var_betawl_dn0 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn0)) * locals.var_mu) + (assign36130_e41524 * locals.var_mu_dn0)) + locals.var_ids_res_dn0) + locals.var_ids_bac_dn0), ((((((locals.var_betawl_dn2 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn2)) * locals.var_mu) + (assign36130_e41524 * locals.var_mu_dn2)) + locals.var_ids_res_dn2) + locals.var_ids_bac_dn2), ((((((locals.var_betawl_dn4 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn4)) * locals.var_mu) + (assign36130_e41524 * locals.var_mu_dn4)) + locals.var_ids_res_dn4) + locals.var_ids_bac_dn4), ((((((locals.var_betawl_dn5 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn5)) * locals.var_mu) + (assign36130_e41524 * locals.var_mu_dn5)) + locals.var_ids_res_dn5) + locals.var_ids_bac_dn5), ((((((locals.var_betawl_dn6 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn6)) * locals.var_mu) + (assign36130_e41524 * locals.var_mu_dn6)) + locals.var_ids_res_dn6) + locals.var_ids_bac_dn6), ((((((locals.var_betawl_dn7 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn7)) * locals.var_mu) + (assign36130_e41524 * locals.var_mu_dn7)) + locals.var_ids_res_dn7) + locals.var_ids_bac_dn7), ((((((locals.var_betawl_dn8 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn8)) * locals.var_mu) + (assign36130_e41524 * locals.var_mu_dn8)) + locals.var_ids_res_dn8) + locals.var_ids_bac_dn8), ((((((locals.var_betawl_dn9 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn9)) * locals.var_mu) + (assign36130_e41524 * locals.var_mu_dn9)) + locals.var_ids_res_dn9) + locals.var_ids_bac_dn9), ((((((locals.var_betawl_dn10 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn10)) * locals.var_mu) + (assign36130_e41524 * locals.var_mu_dn10)) + locals.var_ids_res_dn10) + locals.var_ids_bac_dn10), ((((((locals.var_betawl_dn13 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn13)) * locals.var_mu) + (assign36130_e41524 * locals.var_mu_dn13)) + locals.var_ids_res_dn13) + locals.var_ids_bac_dn13),)
    } else {
        (locals.var_ids0, locals.var_ids0_dn0, locals.var_ids0_dn2, locals.var_ids0_dn4, locals.var_ids0_dn5, locals.var_ids0_dn6, locals.var_ids0_dn7, locals.var_ids0_dn8, locals.var_ids0_dn9, locals.var_ids0_dn10, locals.var_ids0_dn13,)
    }
};
        locals.var_ids0 = assign36130_e41532;
        locals.var_ids0_dn0 = assign36130_e41532_d_n0;
        locals.var_ids0_dn2 = assign36130_e41532_d_n2;
        locals.var_ids0_dn4 = assign36130_e41532_d_n4;
        locals.var_ids0_dn5 = assign36130_e41532_d_n5;
        locals.var_ids0_dn6 = assign36130_e41532_d_n6;
        locals.var_ids0_dn7 = assign36130_e41532_d_n7;
        locals.var_ids0_dn8 = assign36130_e41532_d_n8;
        locals.var_ids0_dn9 = assign36130_e41532_d_n9;
        locals.var_ids0_dn10 = assign36130_e41532_d_n10;
        locals.var_ids0_dn13 = assign36130_e41532_d_n13;
        locals.var_ids0_rv = 0.0;

        let (assign36140_e41542, assign36140_e41542_d_n0, assign36140_e41542_d_n2, assign36140_e41542_d_n4, assign36140_e41542_d_n5, assign36140_e41542_d_n6, assign36140_e41542_d_n7, assign36140_e41542_d_n8, assign36140_e41542_d_n9, assign36140_e41542_d_n10, assign36140_e41542_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign36140_e41538: f64 = (locals.var_betawl * locals.var_idd);
        let assign36140_e41540: f64 = (assign36140_e41538 * locals.var_mu);
        (assign36140_e41540, ((((locals.var_betawl_dn0 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn0)) * locals.var_mu) + (assign36140_e41538 * locals.var_mu_dn0)), ((((locals.var_betawl_dn2 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn2)) * locals.var_mu) + (assign36140_e41538 * locals.var_mu_dn2)), ((((locals.var_betawl_dn4 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn4)) * locals.var_mu) + (assign36140_e41538 * locals.var_mu_dn4)), ((((locals.var_betawl_dn5 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn5)) * locals.var_mu) + (assign36140_e41538 * locals.var_mu_dn5)), ((((locals.var_betawl_dn6 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn6)) * locals.var_mu) + (assign36140_e41538 * locals.var_mu_dn6)), ((((locals.var_betawl_dn7 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn7)) * locals.var_mu) + (assign36140_e41538 * locals.var_mu_dn7)), ((((locals.var_betawl_dn8 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn8)) * locals.var_mu) + (assign36140_e41538 * locals.var_mu_dn8)), ((((locals.var_betawl_dn9 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn9)) * locals.var_mu) + (assign36140_e41538 * locals.var_mu_dn9)), ((((locals.var_betawl_dn10 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn10)) * locals.var_mu) + (assign36140_e41538 * locals.var_mu_dn10)), ((((locals.var_betawl_dn13 * locals.var_idd) + (locals.var_betawl * locals.var_idd_dn13)) * locals.var_mu) + (assign36140_e41538 * locals.var_mu_dn13)),)
    } else {
        (locals.var_ids_acc, locals.var_ids_acc_dn0, locals.var_ids_acc_dn2, locals.var_ids_acc_dn4, locals.var_ids_acc_dn5, locals.var_ids_acc_dn6, locals.var_ids_acc_dn7, locals.var_ids_acc_dn8, locals.var_ids_acc_dn9, locals.var_ids_acc_dn10, locals.var_ids_acc_dn13,)
    }
};
        locals.var_ids_acc = assign36140_e41542;
        locals.var_ids_acc_dn0 = assign36140_e41542_d_n0;
        locals.var_ids_acc_dn2 = assign36140_e41542_d_n2;
        locals.var_ids_acc_dn4 = assign36140_e41542_d_n4;
        locals.var_ids_acc_dn5 = assign36140_e41542_d_n5;
        locals.var_ids_acc_dn6 = assign36140_e41542_d_n6;
        locals.var_ids_acc_dn7 = assign36140_e41542_d_n7;
        locals.var_ids_acc_dn8 = assign36140_e41542_d_n8;
        locals.var_ids_acc_dn9 = assign36140_e41542_d_n9;
        locals.var_ids_acc_dn10 = assign36140_e41542_d_n10;
        locals.var_ids_acc_dn13 = assign36140_e41542_d_n13;
        locals.var_ids_acc_rv = 0.0;

        let (assign36150_e41548, assign36150_e41548_d_n0, assign36150_e41548_d_n2, assign36150_e41548_d_n4, assign36150_e41548_d_n5, assign36150_e41548_d_n6, assign36150_e41548_d_n7, assign36150_e41548_d_n8, assign36150_e41548_d_n9, assign36150_e41548_d_n10, assign36150_e41548_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        (locals.var_mu, locals.var_mu_dn0, locals.var_mu_dn2, locals.var_mu_dn4, locals.var_mu_dn5, locals.var_mu_dn6, locals.var_mu_dn7, locals.var_mu_dn8, locals.var_mu_dn9, locals.var_mu_dn10, locals.var_mu_dn13,)
    } else {
        (locals.var_mu_acc, locals.var_mu_acc_dn0, locals.var_mu_acc_dn2, locals.var_mu_acc_dn4, locals.var_mu_acc_dn5, locals.var_mu_acc_dn6, locals.var_mu_acc_dn7, locals.var_mu_acc_dn8, locals.var_mu_acc_dn9, locals.var_mu_acc_dn10, locals.var_mu_acc_dn13,)
    }
};
        locals.var_mu_acc = assign36150_e41548;
        locals.var_mu_acc_dn0 = assign36150_e41548_d_n0;
        locals.var_mu_acc_dn2 = assign36150_e41548_d_n2;
        locals.var_mu_acc_dn4 = assign36150_e41548_d_n4;
        locals.var_mu_acc_dn5 = assign36150_e41548_d_n5;
        locals.var_mu_acc_dn6 = assign36150_e41548_d_n6;
        locals.var_mu_acc_dn7 = assign36150_e41548_d_n7;
        locals.var_mu_acc_dn8 = assign36150_e41548_d_n8;
        locals.var_mu_acc_dn9 = assign36150_e41548_d_n9;
        locals.var_mu_acc_dn10 = assign36150_e41548_d_n10;
        locals.var_mu_acc_dn13 = assign36150_e41548_d_n13;
        locals.var_mu_acc_rv = 0.0;

        let (assign36160_e41554, assign36160_e41554_d_n0, assign36160_e41554_d_n2, assign36160_e41554_d_n4, assign36160_e41554_d_n5, assign36160_e41554_d_n6, assign36160_e41554_d_n7, assign36160_e41554_d_n8, assign36160_e41554_d_n9, assign36160_e41554_d_n10, assign36160_e41554_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn4, locals.var_vdsorg_dn5, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn8, locals.var_vdsorg_dn9, locals.var_vdsorg_dn10, locals.var_vdsorg_dn13,)
    } else {
        (locals.var_vds, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn13,)
    }
};
        locals.var_vds = assign36160_e41554;
        locals.var_vds_dn0 = assign36160_e41554_d_n0;
        locals.var_vds_dn2 = assign36160_e41554_d_n2;
        locals.var_vds_dn4 = assign36160_e41554_d_n4;
        locals.var_vds_dn5 = assign36160_e41554_d_n5;
        locals.var_vds_dn6 = assign36160_e41554_d_n6;
        locals.var_vds_dn7 = assign36160_e41554_d_n7;
        locals.var_vds_dn8 = assign36160_e41554_d_n8;
        locals.var_vds_dn9 = assign36160_e41554_d_n9;
        locals.var_vds_dn10 = assign36160_e41554_d_n10;
        locals.var_vds_dn13 = assign36160_e41554_d_n13;
        locals.var_vds_rv = 0.0;

        let assign36170_e41557: f64 = if p.p283 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard821 = assign36170_e41557;
        locals.var_guard821_rv = 0.0;

        let (assign36180_e41569, assign36180_e41569_d_n0, assign36180_e41569_d_n2, assign36180_e41569_d_n4, assign36180_e41569_d_n5, assign36180_e41569_d_n6, assign36180_e41569_d_n7, assign36180_e41569_d_n8, assign36180_e41569_d_n9, assign36180_e41569_d_n10, assign36180_e41569_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard821 != 0.0)) {
        let assign36180_e41566: f64 = (locals.var_vds - locals.var_pds);
        let assign36180_e41567: f64 = (0.5 * assign36180_e41566);
        (assign36180_e41567, (0.5 * (locals.var_vds_dn0 - locals.var_pds_dn0)), (0.5 * (locals.var_vds_dn2 - locals.var_pds_dn2)), (0.5 * (locals.var_vds_dn4 - locals.var_pds_dn4)), (0.5 * (locals.var_vds_dn5 - locals.var_pds_dn5)), (0.5 * (locals.var_vds_dn6 - locals.var_pds_dn6)), (0.5 * (locals.var_vds_dn7 - locals.var_pds_dn7)), (0.5 * (locals.var_vds_dn8 - locals.var_pds_dn8)), (0.5 * (locals.var_vds_dn9 - locals.var_pds_dn9)), (0.5 * (locals.var_vds_dn10 - locals.var_pds_dn10)), (0.5 * (locals.var_vds_dn13 - locals.var_pds_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign36180_e41569;
        locals.var_t1_dn0 = assign36180_e41569_d_n0;
        locals.var_t1_dn2 = assign36180_e41569_d_n2;
        locals.var_t1_dn4 = assign36180_e41569_d_n4;
        locals.var_t1_dn5 = assign36180_e41569_d_n5;
        locals.var_t1_dn6 = assign36180_e41569_d_n6;
        locals.var_t1_dn7 = assign36180_e41569_d_n7;
        locals.var_t1_dn8 = assign36180_e41569_d_n8;
        locals.var_t1_dn9 = assign36180_e41569_d_n9;
        locals.var_t1_dn10 = assign36180_e41569_d_n10;
        locals.var_t1_dn13 = assign36180_e41569_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign36190_e41581, assign36190_e41581_d_n0, assign36190_e41581_d_n2, assign36190_e41581_d_n4, assign36190_e41581_d_n5, assign36190_e41581_d_n6, assign36190_e41581_d_n7, assign36190_e41581_d_n8, assign36190_e41581_d_n9, assign36190_e41581_d_n10, assign36190_e41581_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard821 != 0.0)) {
        let assign36190_e41577: f64 = (2.0 * locals.var_t1);
        let assign36190_e41579: f64 = (assign36190_e41577 / 0.01);
        (assign36190_e41579, ((2.0 * locals.var_t1_dn0) / 0.01), ((2.0 * locals.var_t1_dn2) / 0.01), ((2.0 * locals.var_t1_dn4) / 0.01), ((2.0 * locals.var_t1_dn5) / 0.01), ((2.0 * locals.var_t1_dn6) / 0.01), ((2.0 * locals.var_t1_dn7) / 0.01), ((2.0 * locals.var_t1_dn8) / 0.01), ((2.0 * locals.var_t1_dn9) / 0.01), ((2.0 * locals.var_t1_dn10) / 0.01), ((2.0 * locals.var_t1_dn13) / 0.01),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign36190_e41581;
        locals.var_tmf1_dn0 = assign36190_e41581_d_n0;
        locals.var_tmf1_dn2 = assign36190_e41581_d_n2;
        locals.var_tmf1_dn4 = assign36190_e41581_d_n4;
        locals.var_tmf1_dn5 = assign36190_e41581_d_n5;
        locals.var_tmf1_dn6 = assign36190_e41581_d_n6;
        locals.var_tmf1_dn7 = assign36190_e41581_d_n7;
        locals.var_tmf1_dn8 = assign36190_e41581_d_n8;
        locals.var_tmf1_dn9 = assign36190_e41581_d_n9;
        locals.var_tmf1_dn10 = assign36190_e41581_d_n10;
        locals.var_tmf1_dn13 = assign36190_e41581_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign36200_e41625, assign36200_e41625_d_n0, assign36200_e41625_d_n2, assign36200_e41625_d_n4, assign36200_e41625_d_n5, assign36200_e41625_d_n6, assign36200_e41625_d_n7, assign36200_e41625_d_n8, assign36200_e41625_d_n9, assign36200_e41625_d_n10, assign36200_e41625_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard821 != 0.0)) {
        let assign36200_e41591: f64 = (1.0 / 2.0);
        let assign36200_e41595: f64 = (1.0 / 6.0);
        let assign36200_e41599: f64 = (1.0 / 24.0);
        let assign36200_e41603: f64 = (1.0 / 120.0);
        let assign36200_e41607: f64 = (1.0 / 720.0);
        let assign36200_e41611: f64 = (1.0 / 5040.0);
        let assign36200_e41612: f64 = (locals.var_tmf1 * assign36200_e41611);
        let assign36200_e41613: f64 = (assign36200_e41607 + assign36200_e41612);
        let assign36200_e41614: f64 = (locals.var_tmf1 * assign36200_e41613);
        let assign36200_e41615: f64 = (assign36200_e41603 + assign36200_e41614);
        let assign36200_e41616: f64 = (locals.var_tmf1 * assign36200_e41615);
        let assign36200_e41617: f64 = (assign36200_e41599 + assign36200_e41616);
        let assign36200_e41618: f64 = (locals.var_tmf1 * assign36200_e41617);
        let assign36200_e41619: f64 = (assign36200_e41595 + assign36200_e41618);
        let assign36200_e41620: f64 = (locals.var_tmf1 * assign36200_e41619);
        let assign36200_e41621: f64 = (assign36200_e41591 + assign36200_e41620);
        let assign36200_e41622: f64 = (locals.var_tmf1 * assign36200_e41621);
        let assign36200_e41623: f64 = (1.0 + assign36200_e41622);
        (assign36200_e41623, ((locals.var_tmf1_dn0 * assign36200_e41621) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign36200_e41619) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign36200_e41617) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign36200_e41615) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign36200_e41613) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign36200_e41611))))))))))), ((locals.var_tmf1_dn2 * assign36200_e41621) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign36200_e41619) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign36200_e41617) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign36200_e41615) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign36200_e41613) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign36200_e41611))))))))))), ((locals.var_tmf1_dn4 * assign36200_e41621) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign36200_e41619) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign36200_e41617) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign36200_e41615) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign36200_e41613) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign36200_e41611))))))))))), ((locals.var_tmf1_dn5 * assign36200_e41621) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign36200_e41619) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign36200_e41617) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign36200_e41615) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign36200_e41613) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign36200_e41611))))))))))), ((locals.var_tmf1_dn6 * assign36200_e41621) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign36200_e41619) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign36200_e41617) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign36200_e41615) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign36200_e41613) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign36200_e41611))))))))))), ((locals.var_tmf1_dn7 * assign36200_e41621) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign36200_e41619) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign36200_e41617) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign36200_e41615) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign36200_e41613) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign36200_e41611))))))))))), ((locals.var_tmf1_dn8 * assign36200_e41621) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign36200_e41619) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign36200_e41617) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign36200_e41615) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign36200_e41613) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign36200_e41611))))))))))), ((locals.var_tmf1_dn9 * assign36200_e41621) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign36200_e41619) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign36200_e41617) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign36200_e41615) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign36200_e41613) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign36200_e41611))))))))))), ((locals.var_tmf1_dn10 * assign36200_e41621) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign36200_e41619) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign36200_e41617) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign36200_e41615) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign36200_e41613) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign36200_e41611))))))))))), ((locals.var_tmf1_dn13 * assign36200_e41621) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign36200_e41619) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign36200_e41617) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign36200_e41615) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign36200_e41613) + (locals.var_tmf1 * (locals.var_tmf1_dn13 * assign36200_e41611))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign36200_e41625;
        locals.var_tmf2_dn0 = assign36200_e41625_d_n0;
        locals.var_tmf2_dn2 = assign36200_e41625_d_n2;
        locals.var_tmf2_dn4 = assign36200_e41625_d_n4;
        locals.var_tmf2_dn5 = assign36200_e41625_d_n5;
        locals.var_tmf2_dn6 = assign36200_e41625_d_n6;
        locals.var_tmf2_dn7 = assign36200_e41625_d_n7;
        locals.var_tmf2_dn8 = assign36200_e41625_d_n8;
        locals.var_tmf2_dn9 = assign36200_e41625_d_n9;
        locals.var_tmf2_dn10 = assign36200_e41625_d_n10;
        locals.var_tmf2_dn13 = assign36200_e41625_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign36210_e41665, assign36210_e41665_d_n0, assign36210_e41665_d_n2, assign36210_e41665_d_n4, assign36210_e41665_d_n5, assign36210_e41665_d_n6, assign36210_e41665_d_n7, assign36210_e41665_d_n8, assign36210_e41665_d_n9, assign36210_e41665_d_n10, assign36210_e41665_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard821 != 0.0)) {
        let assign36210_e41633: f64 = (1.0 / 2.0);
        let assign36210_e41637: f64 = (1.0 / 3.0);
        let assign36210_e41641: f64 = (1.0 / 8.0);
        let assign36210_e41645: f64 = (1.0 / 30.0);
        let assign36210_e41649: f64 = (1.0 / 144.0);
        let assign36210_e41653: f64 = (1.0 / 840.0);
        let assign36210_e41654: f64 = (locals.var_tmf1 * assign36210_e41653);
        let assign36210_e41655: f64 = (assign36210_e41649 + assign36210_e41654);
        let assign36210_e41656: f64 = (locals.var_tmf1 * assign36210_e41655);
        let assign36210_e41657: f64 = (assign36210_e41645 + assign36210_e41656);
        let assign36210_e41658: f64 = (locals.var_tmf1 * assign36210_e41657);
        let assign36210_e41659: f64 = (assign36210_e41641 + assign36210_e41658);
        let assign36210_e41660: f64 = (locals.var_tmf1 * assign36210_e41659);
        let assign36210_e41661: f64 = (assign36210_e41637 + assign36210_e41660);
        let assign36210_e41662: f64 = (locals.var_tmf1 * assign36210_e41661);
        let assign36210_e41663: f64 = (assign36210_e41633 + assign36210_e41662);
        (assign36210_e41663, ((locals.var_tmf1_dn0 * assign36210_e41661) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign36210_e41659) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign36210_e41657) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign36210_e41655) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign36210_e41653))))))))), ((locals.var_tmf1_dn2 * assign36210_e41661) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign36210_e41659) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign36210_e41657) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign36210_e41655) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign36210_e41653))))))))), ((locals.var_tmf1_dn4 * assign36210_e41661) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign36210_e41659) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign36210_e41657) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign36210_e41655) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign36210_e41653))))))))), ((locals.var_tmf1_dn5 * assign36210_e41661) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign36210_e41659) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign36210_e41657) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign36210_e41655) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign36210_e41653))))))))), ((locals.var_tmf1_dn6 * assign36210_e41661) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign36210_e41659) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign36210_e41657) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign36210_e41655) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign36210_e41653))))))))), ((locals.var_tmf1_dn7 * assign36210_e41661) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign36210_e41659) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign36210_e41657) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign36210_e41655) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign36210_e41653))))))))), ((locals.var_tmf1_dn8 * assign36210_e41661) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign36210_e41659) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign36210_e41657) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign36210_e41655) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign36210_e41653))))))))), ((locals.var_tmf1_dn9 * assign36210_e41661) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign36210_e41659) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign36210_e41657) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign36210_e41655) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign36210_e41653))))))))), ((locals.var_tmf1_dn10 * assign36210_e41661) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign36210_e41659) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign36210_e41657) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign36210_e41655) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign36210_e41653))))))))), ((locals.var_tmf1_dn13 * assign36210_e41661) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign36210_e41659) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign36210_e41657) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign36210_e41655) + (locals.var_tmf1 * (locals.var_tmf1_dn13 * assign36210_e41653))))))))),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn13,)
    }
};
        locals.var_tmf3 = assign36210_e41665;
        locals.var_tmf3_dn0 = assign36210_e41665_d_n0;
        locals.var_tmf3_dn2 = assign36210_e41665_d_n2;
        locals.var_tmf3_dn4 = assign36210_e41665_d_n4;
        locals.var_tmf3_dn5 = assign36210_e41665_d_n5;
        locals.var_tmf3_dn6 = assign36210_e41665_d_n6;
        locals.var_tmf3_dn7 = assign36210_e41665_d_n7;
        locals.var_tmf3_dn8 = assign36210_e41665_d_n8;
        locals.var_tmf3_dn9 = assign36210_e41665_d_n9;
        locals.var_tmf3_dn10 = assign36210_e41665_d_n10;
        locals.var_tmf3_dn13 = assign36210_e41665_d_n13;
        locals.var_tmf3_rv = 0.0;

        let (assign36220_e41675, assign36220_e41675_d_n0, assign36220_e41675_d_n2, assign36220_e41675_d_n4, assign36220_e41675_d_n5, assign36220_e41675_d_n6, assign36220_e41675_d_n7, assign36220_e41675_d_n8, assign36220_e41675_d_n9, assign36220_e41675_d_n10, assign36220_e41675_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard821 != 0.0)) {
        let assign36220_e41673: f64 = (0.01 / locals.var_tmf2);
        (assign36220_e41673, (-((0.01 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((0.01 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign36220_e41675;
        locals.var_t6_dn0 = assign36220_e41675_d_n0;
        locals.var_t6_dn2 = assign36220_e41675_d_n2;
        locals.var_t6_dn4 = assign36220_e41675_d_n4;
        locals.var_t6_dn5 = assign36220_e41675_d_n5;
        locals.var_t6_dn6 = assign36220_e41675_d_n6;
        locals.var_t6_dn7 = assign36220_e41675_d_n7;
        locals.var_t6_dn8 = assign36220_e41675_d_n8;
        locals.var_t6_dn9 = assign36220_e41675_d_n9;
        locals.var_t6_dn10 = assign36220_e41675_d_n10;
        locals.var_t6_dn13 = assign36220_e41675_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign36230_e41690, assign36230_e41690_d_n0, assign36230_e41690_d_n2, assign36230_e41690_d_n4, assign36230_e41690_d_n5, assign36230_e41690_d_n6, assign36230_e41690_d_n7, assign36230_e41690_d_n8, assign36230_e41690_d_n9, assign36230_e41690_d_n10, assign36230_e41690_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard821 != 0.0)) {
        let assign36230_e41682: f64 = (-2.0);
        let assign36230_e41684: f64 = (assign36230_e41682 * locals.var_tmf3);
        let assign36230_e41687: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign36230_e41688: f64 = (assign36230_e41684 / assign36230_e41687);
        (assign36230_e41688, ((((assign36230_e41682 * locals.var_tmf3_dn0) * assign36230_e41687) - (assign36230_e41684 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign36230_e41687 * assign36230_e41687)), ((((assign36230_e41682 * locals.var_tmf3_dn2) * assign36230_e41687) - (assign36230_e41684 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign36230_e41687 * assign36230_e41687)), ((((assign36230_e41682 * locals.var_tmf3_dn4) * assign36230_e41687) - (assign36230_e41684 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign36230_e41687 * assign36230_e41687)), ((((assign36230_e41682 * locals.var_tmf3_dn5) * assign36230_e41687) - (assign36230_e41684 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign36230_e41687 * assign36230_e41687)), ((((assign36230_e41682 * locals.var_tmf3_dn6) * assign36230_e41687) - (assign36230_e41684 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign36230_e41687 * assign36230_e41687)), ((((assign36230_e41682 * locals.var_tmf3_dn7) * assign36230_e41687) - (assign36230_e41684 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign36230_e41687 * assign36230_e41687)), ((((assign36230_e41682 * locals.var_tmf3_dn8) * assign36230_e41687) - (assign36230_e41684 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign36230_e41687 * assign36230_e41687)), ((((assign36230_e41682 * locals.var_tmf3_dn9) * assign36230_e41687) - (assign36230_e41684 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign36230_e41687 * assign36230_e41687)), ((((assign36230_e41682 * locals.var_tmf3_dn10) * assign36230_e41687) - (assign36230_e41684 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign36230_e41687 * assign36230_e41687)), ((((assign36230_e41682 * locals.var_tmf3_dn13) * assign36230_e41687) - (assign36230_e41684 * ((locals.var_tmf2_dn13 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn13)))) / (assign36230_e41687 * assign36230_e41687)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign36230_e41690;
        locals.var_t2_dn0 = assign36230_e41690_d_n0;
        locals.var_t2_dn2 = assign36230_e41690_d_n2;
        locals.var_t2_dn4 = assign36230_e41690_d_n4;
        locals.var_t2_dn5 = assign36230_e41690_d_n5;
        locals.var_t2_dn6 = assign36230_e41690_d_n6;
        locals.var_t2_dn7 = assign36230_e41690_d_n7;
        locals.var_t2_dn8 = assign36230_e41690_d_n8;
        locals.var_t2_dn9 = assign36230_e41690_d_n9;
        locals.var_t2_dn10 = assign36230_e41690_d_n10;
        locals.var_t2_dn13 = assign36230_e41690_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign36240_e41702, assign36240_e41702_d_n0, assign36240_e41702_d_n2, assign36240_e41702_d_n4, assign36240_e41702_d_n5, assign36240_e41702_d_n6, assign36240_e41702_d_n7, assign36240_e41702_d_n8, assign36240_e41702_d_n9, assign36240_e41702_d_n10, assign36240_e41702_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard821 != 0.0)) {
        let assign36240_e41699: f64 = (locals.var_phi_s0_dep + locals.var_t6);
        let assign36240_e41700: f64 = (1.1 - assign36240_e41699);
        (assign36240_e41700, (-(locals.var_phi_s0_dep_dn0 + locals.var_t6_dn0)), (-(locals.var_phi_s0_dep_dn2 + locals.var_t6_dn2)), (-(locals.var_phi_s0_dep_dn4 + locals.var_t6_dn4)), (-(locals.var_phi_s0_dep_dn5 + locals.var_t6_dn5)), (-(locals.var_phi_s0_dep_dn6 + locals.var_t6_dn6)), (-(locals.var_phi_s0_dep_dn7 + locals.var_t6_dn7)), (-(locals.var_phi_s0_dep_dn8 + locals.var_t6_dn8)), (-(locals.var_phi_s0_dep_dn9 + locals.var_t6_dn9)), (-(locals.var_phi_s0_dep_dn10 + locals.var_t6_dn10)), (-(locals.var_phi_s0_dep_dn13 + locals.var_t6_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign36240_e41702;
        locals.var_t1_dn0 = assign36240_e41702_d_n0;
        locals.var_t1_dn2 = assign36240_e41702_d_n2;
        locals.var_t1_dn4 = assign36240_e41702_d_n4;
        locals.var_t1_dn5 = assign36240_e41702_d_n5;
        locals.var_t1_dn6 = assign36240_e41702_d_n6;
        locals.var_t1_dn7 = assign36240_e41702_d_n7;
        locals.var_t1_dn8 = assign36240_e41702_d_n8;
        locals.var_t1_dn9 = assign36240_e41702_d_n9;
        locals.var_t1_dn10 = assign36240_e41702_d_n10;
        locals.var_t1_dn13 = assign36240_e41702_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign36250_e41719, assign36250_e41719_d_n0, assign36250_e41719_d_n2, assign36250_e41719_d_n4, assign36250_e41719_d_n5, assign36250_e41719_d_n6, assign36250_e41719_d_n7, assign36250_e41719_d_n8, assign36250_e41719_d_n9, assign36250_e41719_d_n10, assign36250_e41719_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard821 != 0.0)) {
        let assign36250_e41710: f64 = (locals.var_t1 * locals.var_t1);
        let assign36250_e41713: f64 = (4.0 * 0.05);
        let assign36250_e41715: f64 = (assign36250_e41713 * 0.05);
        let assign36250_e41716: f64 = (assign36250_e41710 + assign36250_e41715);
        let assign36250_e41717: f64 = (assign36250_e41716).sqrt();
        (assign36250_e41717, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign36250_e41717)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign36250_e41717)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign36250_e41717)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign36250_e41717)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign36250_e41717)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign36250_e41717)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign36250_e41717)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign36250_e41717)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign36250_e41717)), (((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) / (2.0 * assign36250_e41717)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign36250_e41719;
        locals.var_tmf2_dn0 = assign36250_e41719_d_n0;
        locals.var_tmf2_dn2 = assign36250_e41719_d_n2;
        locals.var_tmf2_dn4 = assign36250_e41719_d_n4;
        locals.var_tmf2_dn5 = assign36250_e41719_d_n5;
        locals.var_tmf2_dn6 = assign36250_e41719_d_n6;
        locals.var_tmf2_dn7 = assign36250_e41719_d_n7;
        locals.var_tmf2_dn8 = assign36250_e41719_d_n8;
        locals.var_tmf2_dn9 = assign36250_e41719_d_n9;
        locals.var_tmf2_dn10 = assign36250_e41719_d_n10;
        locals.var_tmf2_dn13 = assign36250_e41719_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign36260_e41733, assign36260_e41733_d_n0, assign36260_e41733_d_n2, assign36260_e41733_d_n4, assign36260_e41733_d_n5, assign36260_e41733_d_n6, assign36260_e41733_d_n7, assign36260_e41733_d_n8, assign36260_e41733_d_n9, assign36260_e41733_d_n10, assign36260_e41733_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard821 != 0.0)) {
        let assign36260_e41729: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign36260_e41730: f64 = (1.0 + assign36260_e41729);
        let assign36260_e41731: f64 = (0.5 * assign36260_e41730);
        (assign36260_e41731, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn7 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn9 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn13 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign36260_e41733;
        locals.var_t0_dn0 = assign36260_e41733_d_n0;
        locals.var_t0_dn2 = assign36260_e41733_d_n2;
        locals.var_t0_dn4 = assign36260_e41733_d_n4;
        locals.var_t0_dn5 = assign36260_e41733_d_n5;
        locals.var_t0_dn6 = assign36260_e41733_d_n6;
        locals.var_t0_dn7 = assign36260_e41733_d_n7;
        locals.var_t0_dn8 = assign36260_e41733_d_n8;
        locals.var_t0_dn9 = assign36260_e41733_d_n9;
        locals.var_t0_dn10 = assign36260_e41733_d_n10;
        locals.var_t0_dn13 = assign36260_e41733_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign36270_e41745, assign36270_e41745_d_n0, assign36270_e41745_d_n2, assign36270_e41745_d_n4, assign36270_e41745_d_n5, assign36270_e41745_d_n6, assign36270_e41745_d_n7, assign36270_e41745_d_n8, assign36270_e41745_d_n9, assign36270_e41745_d_n10, assign36270_e41745_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard821 != 0.0)) {
        let assign36270_e41742: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign36270_e41743: f64 = (0.5 * assign36270_e41742);
        (assign36270_e41743, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign36270_e41745;
        locals.var_t2_dn0 = assign36270_e41745_d_n0;
        locals.var_t2_dn2 = assign36270_e41745_d_n2;
        locals.var_t2_dn4 = assign36270_e41745_d_n4;
        locals.var_t2_dn5 = assign36270_e41745_d_n5;
        locals.var_t2_dn6 = assign36270_e41745_d_n6;
        locals.var_t2_dn7 = assign36270_e41745_d_n7;
        locals.var_t2_dn8 = assign36270_e41745_d_n8;
        locals.var_t2_dn9 = assign36270_e41745_d_n9;
        locals.var_t2_dn10 = assign36270_e41745_d_n10;
        locals.var_t2_dn13 = assign36270_e41745_d_n13;
        locals.var_t2_rv = 0.0;

        let assign36280_e41748: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard822 = assign36280_e41748;
        locals.var_guard822_rv = 0.0;

        let (assign36290_e41758, assign36290_e41758_d_n0, assign36290_e41758_d_n2, assign36290_e41758_d_n4, assign36290_e41758_d_n5, assign36290_e41758_d_n6, assign36290_e41758_d_n7, assign36290_e41758_d_n8, assign36290_e41758_d_n9, assign36290_e41758_d_n10, assign36290_e41758_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard821 != 0.0)) && (locals.var_guard822 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign36290_e41758;
        locals.var_t2_dn0 = assign36290_e41758_d_n0;
        locals.var_t2_dn2 = assign36290_e41758_d_n2;
        locals.var_t2_dn4 = assign36290_e41758_d_n4;
        locals.var_t2_dn5 = assign36290_e41758_d_n5;
        locals.var_t2_dn6 = assign36290_e41758_d_n6;
        locals.var_t2_dn7 = assign36290_e41758_d_n7;
        locals.var_t2_dn8 = assign36290_e41758_d_n8;
        locals.var_t2_dn9 = assign36290_e41758_d_n9;
        locals.var_t2_dn10 = assign36290_e41758_d_n10;
        locals.var_t2_dn13 = assign36290_e41758_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign36300_e41768, assign36300_e41768_d_n0, assign36300_e41768_d_n2, assign36300_e41768_d_n4, assign36300_e41768_d_n5, assign36300_e41768_d_n6, assign36300_e41768_d_n7, assign36300_e41768_d_n8, assign36300_e41768_d_n9, assign36300_e41768_d_n10, assign36300_e41768_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard821 != 0.0)) && (locals.var_guard822 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign36300_e41768;
        locals.var_t0_dn0 = assign36300_e41768_d_n0;
        locals.var_t0_dn2 = assign36300_e41768_d_n2;
        locals.var_t0_dn4 = assign36300_e41768_d_n4;
        locals.var_t0_dn5 = assign36300_e41768_d_n5;
        locals.var_t0_dn6 = assign36300_e41768_d_n6;
        locals.var_t0_dn7 = assign36300_e41768_d_n7;
        locals.var_t0_dn8 = assign36300_e41768_d_n8;
        locals.var_t0_dn9 = assign36300_e41768_d_n9;
        locals.var_t0_dn10 = assign36300_e41768_d_n10;
        locals.var_t0_dn13 = assign36300_e41768_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign36310_e41778, assign36310_e41778_d_n0, assign36310_e41778_d_n2, assign36310_e41778_d_n4, assign36310_e41778_d_n5, assign36310_e41778_d_n6, assign36310_e41778_d_n7, assign36310_e41778_d_n8, assign36310_e41778_d_n9, assign36310_e41778_d_n10, assign36310_e41778_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard821 != 0.0)) {
        let assign36310_e41776: f64 = (locals.var_t2 + 1e-25);
        (assign36310_e41776, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign36310_e41778;
        locals.var_t2_dn0 = assign36310_e41778_d_n0;
        locals.var_t2_dn2 = assign36310_e41778_d_n2;
        locals.var_t2_dn4 = assign36310_e41778_d_n4;
        locals.var_t2_dn5 = assign36310_e41778_d_n5;
        locals.var_t2_dn6 = assign36310_e41778_d_n6;
        locals.var_t2_dn7 = assign36310_e41778_d_n7;
        locals.var_t2_dn8 = assign36310_e41778_d_n8;
        locals.var_t2_dn9 = assign36310_e41778_d_n9;
        locals.var_t2_dn10 = assign36310_e41778_d_n10;
        locals.var_t2_dn13 = assign36310_e41778_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign36320_e41788, assign36320_e41788_d_n0, assign36320_e41788_d_n2, assign36320_e41788_d_n4, assign36320_e41788_d_n5, assign36320_e41788_d_n6, assign36320_e41788_d_n7, assign36320_e41788_d_n8, assign36320_e41788_d_n9, assign36320_e41788_d_n10, assign36320_e41788_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard821 != 0.0)) {
        let assign36320_e41786: f64 = (locals.var_beta * locals.var_ptl0);
        (assign36320_e41786, (locals.var_beta_dn0 * locals.var_ptl0), (locals.var_beta_dn2 * locals.var_ptl0), (locals.var_beta_dn4 * locals.var_ptl0), (locals.var_beta_dn5 * locals.var_ptl0), (locals.var_beta_dn6 * locals.var_ptl0), (locals.var_beta_dn7 * locals.var_ptl0), (locals.var_beta_dn8 * locals.var_ptl0), (locals.var_beta_dn9 * locals.var_ptl0), (locals.var_beta_dn10 * locals.var_ptl0), (locals.var_beta_dn13 * locals.var_ptl0),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign36320_e41788;
        locals.var_t0_dn0 = assign36320_e41788_d_n0;
        locals.var_t0_dn2 = assign36320_e41788_d_n2;
        locals.var_t0_dn4 = assign36320_e41788_d_n4;
        locals.var_t0_dn5 = assign36320_e41788_d_n5;
        locals.var_t0_dn6 = assign36320_e41788_d_n6;
        locals.var_t0_dn7 = assign36320_e41788_d_n7;
        locals.var_t0_dn8 = assign36320_e41788_d_n8;
        locals.var_t0_dn9 = assign36320_e41788_d_n9;
        locals.var_t0_dn10 = assign36320_e41788_d_n10;
        locals.var_t0_dn13 = assign36320_e41788_d_n13;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_115(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign36330_e41798, assign36330_e41798_d_n0, assign36330_e41798_d_n2, assign36330_e41798_d_n4, assign36330_e41798_d_n5, assign36330_e41798_d_n6, assign36330_e41798_d_n7, assign36330_e41798_d_n8, assign36330_e41798_d_n9, assign36330_e41798_d_n10, assign36330_e41798_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard821 != 0.0)) {
        let assign36330_e41796: f64 = (locals.var_cox * locals.var_t0);
        (assign36330_e41796, ((locals.var_cox_dn0 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn0)), ((locals.var_cox_dn2 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn2)), ((locals.var_cox_dn4 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn4)), ((locals.var_cox_dn5 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn5)), ((locals.var_cox_dn6 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn6)), ((locals.var_cox_dn7 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn7)), ((locals.var_cox_dn8 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn8)), ((locals.var_cox_dn9 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn9)), ((locals.var_cox_dn10 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn10)), ((locals.var_cox_dn13 * locals.var_t0) + (locals.var_cox * locals.var_t0_dn13)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign36330_e41798;
        locals.var_t3_dn0 = assign36330_e41798_d_n0;
        locals.var_t3_dn2 = assign36330_e41798_d_n2;
        locals.var_t3_dn4 = assign36330_e41798_d_n4;
        locals.var_t3_dn5 = assign36330_e41798_d_n5;
        locals.var_t3_dn6 = assign36330_e41798_d_n6;
        locals.var_t3_dn7 = assign36330_e41798_d_n7;
        locals.var_t3_dn8 = assign36330_e41798_d_n8;
        locals.var_t3_dn9 = assign36330_e41798_d_n9;
        locals.var_t3_dn10 = assign36330_e41798_d_n10;
        locals.var_t3_dn13 = assign36330_e41798_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign36340_e41808, assign36340_e41808_d_n0, assign36340_e41808_d_n2, assign36340_e41808_d_n4, assign36340_e41808_d_n5, assign36340_e41808_d_n6, assign36340_e41808_d_n7, assign36340_e41808_d_n8, assign36340_e41808_d_n9, assign36340_e41808_d_n10, assign36340_e41808_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard821 != 0.0)) {
        let assign36340_e41806: f64 = (locals.var_t2).powf(p.p284);
        (assign36340_e41806, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn0)) } } else { (assign36340_e41806 * (p.p284 * (locals.var_t2_dn0 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn2)) } } else { (assign36340_e41806 * (p.p284 * (locals.var_t2_dn2 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn4)) } } else { (assign36340_e41806 * (p.p284 * (locals.var_t2_dn4 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn5)) } } else { (assign36340_e41806 * (p.p284 * (locals.var_t2_dn5 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn6)) } } else { (assign36340_e41806 * (p.p284 * (locals.var_t2_dn6 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn7)) } } else { (assign36340_e41806 * (p.p284 * (locals.var_t2_dn7 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn8)) } } else { (assign36340_e41806 * (p.p284 * (locals.var_t2_dn8 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn9)) } } else { (assign36340_e41806 * (p.p284 * (locals.var_t2_dn9 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn10)) } } else { (assign36340_e41806 * (p.p284 * (locals.var_t2_dn10 / locals.var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((locals.var_t2).powf(p.p284 - 1.0) * locals.var_t2_dn13)) } } else { (assign36340_e41806 * (p.p284 * (locals.var_t2_dn13 / locals.var_t2))) },)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign36340_e41808;
        locals.var_t0_dn0 = assign36340_e41808_d_n0;
        locals.var_t0_dn2 = assign36340_e41808_d_n2;
        locals.var_t0_dn4 = assign36340_e41808_d_n4;
        locals.var_t0_dn5 = assign36340_e41808_d_n5;
        locals.var_t0_dn6 = assign36340_e41808_d_n6;
        locals.var_t0_dn7 = assign36340_e41808_d_n7;
        locals.var_t0_dn8 = assign36340_e41808_d_n8;
        locals.var_t0_dn9 = assign36340_e41808_d_n9;
        locals.var_t0_dn10 = assign36340_e41808_d_n10;
        locals.var_t0_dn13 = assign36340_e41808_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign36350_e41818, assign36350_e41818_d_n0, assign36350_e41818_d_n2, assign36350_e41818_d_n4, assign36350_e41818_d_n5, assign36350_e41818_d_n6, assign36350_e41818_d_n7, assign36350_e41818_d_n8, assign36350_e41818_d_n9, assign36350_e41818_d_n10, assign36350_e41818_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard821 != 0.0)) {
        let assign36350_e41816: f64 = (locals.var_t3 * locals.var_t0);
        (assign36350_e41816, ((locals.var_t3_dn0 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn0)), ((locals.var_t3_dn2 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn2)), ((locals.var_t3_dn4 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn4)), ((locals.var_t3_dn5 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn5)), ((locals.var_t3_dn6 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn6)), ((locals.var_t3_dn7 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn7)), ((locals.var_t3_dn8 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn8)), ((locals.var_t3_dn9 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn9)), ((locals.var_t3_dn10 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn10)), ((locals.var_t3_dn13 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn13)),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign36350_e41818;
        locals.var_t9_dn0 = assign36350_e41818_d_n0;
        locals.var_t9_dn2 = assign36350_e41818_d_n2;
        locals.var_t9_dn4 = assign36350_e41818_d_n4;
        locals.var_t9_dn5 = assign36350_e41818_d_n5;
        locals.var_t9_dn6 = assign36350_e41818_d_n6;
        locals.var_t9_dn7 = assign36350_e41818_d_n7;
        locals.var_t9_dn8 = assign36350_e41818_d_n8;
        locals.var_t9_dn9 = assign36350_e41818_d_n9;
        locals.var_t9_dn10 = assign36350_e41818_d_n10;
        locals.var_t9_dn13 = assign36350_e41818_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign36360_e41830, assign36360_e41830_d_n0, assign36360_e41830_d_n2, assign36360_e41830_d_n4, assign36360_e41830_d_n5, assign36360_e41830_d_n6, assign36360_e41830_d_n7, assign36360_e41830_d_n8, assign36360_e41830_d_n9, assign36360_e41830_d_n10, assign36360_e41830_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard821 != 0.0)) {
        let assign36360_e41827: f64 = (locals.var_vdsz__blk439 * p.p285);
        let assign36360_e41828: f64 = (1.0 + assign36360_e41827);
        (assign36360_e41828, (locals.var_vdsz__blk439_dn0 * p.p285), (locals.var_vdsz__blk439_dn2 * p.p285), (locals.var_vdsz__blk439_dn4 * p.p285), (locals.var_vdsz__blk439_dn5 * p.p285), (locals.var_vdsz__blk439_dn6 * p.p285), (locals.var_vdsz__blk439_dn7 * p.p285), (locals.var_vdsz__blk439_dn8 * p.p285), (locals.var_vdsz__blk439_dn9 * p.p285), (locals.var_vdsz__blk439_dn10 * p.p285), (locals.var_vdsz__blk439_dn13 * p.p285),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign36360_e41830;
        locals.var_t4_dn0 = assign36360_e41830_d_n0;
        locals.var_t4_dn2 = assign36360_e41830_d_n2;
        locals.var_t4_dn4 = assign36360_e41830_d_n4;
        locals.var_t4_dn5 = assign36360_e41830_d_n5;
        locals.var_t4_dn6 = assign36360_e41830_d_n6;
        locals.var_t4_dn7 = assign36360_e41830_d_n7;
        locals.var_t4_dn8 = assign36360_e41830_d_n8;
        locals.var_t4_dn9 = assign36360_e41830_d_n9;
        locals.var_t4_dn10 = assign36360_e41830_d_n10;
        locals.var_t4_dn13 = assign36360_e41830_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign36370_e41838, assign36370_e41838_d_n0, assign36370_e41838_d_n2, assign36370_e41838_d_n4, assign36370_e41838_d_n5, assign36370_e41838_d_n6, assign36370_e41838_d_n7, assign36370_e41838_d_n8, assign36370_e41838_d_n9, assign36370_e41838_d_n10, assign36370_e41838_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard821 != 0.0)) {
        (locals.var_pt40, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign36370_e41838;
        locals.var_t0_dn0 = assign36370_e41838_d_n0;
        locals.var_t0_dn2 = assign36370_e41838_d_n2;
        locals.var_t0_dn4 = assign36370_e41838_d_n4;
        locals.var_t0_dn5 = assign36370_e41838_d_n5;
        locals.var_t0_dn6 = assign36370_e41838_d_n6;
        locals.var_t0_dn7 = assign36370_e41838_d_n7;
        locals.var_t0_dn8 = assign36370_e41838_d_n8;
        locals.var_t0_dn9 = assign36370_e41838_d_n9;
        locals.var_t0_dn10 = assign36370_e41838_d_n10;
        locals.var_t0_dn13 = assign36370_e41838_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign36380_e41850, assign36380_e41850_d_n0, assign36380_e41850_d_n2, assign36380_e41850_d_n4, assign36380_e41850_d_n5, assign36380_e41850_d_n6, assign36380_e41850_d_n7, assign36380_e41850_d_n8, assign36380_e41850_d_n9, assign36380_e41850_d_n10, assign36380_e41850_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard821 != 0.0)) {
        let assign36380_e41846: f64 = (locals.var_phi_s0_dep + locals.var_t6);
        let assign36380_e41848: f64 = (assign36380_e41846 - locals.var_vbsz__blk438);
        (assign36380_e41848, ((locals.var_phi_s0_dep_dn0 + locals.var_t6_dn0) - locals.var_vbsz__blk438_dn0), ((locals.var_phi_s0_dep_dn2 + locals.var_t6_dn2) - locals.var_vbsz__blk438_dn2), ((locals.var_phi_s0_dep_dn4 + locals.var_t6_dn4) - locals.var_vbsz__blk438_dn4), ((locals.var_phi_s0_dep_dn5 + locals.var_t6_dn5) - locals.var_vbsz__blk438_dn5), ((locals.var_phi_s0_dep_dn6 + locals.var_t6_dn6) - locals.var_vbsz__blk438_dn6), ((locals.var_phi_s0_dep_dn7 + locals.var_t6_dn7) - locals.var_vbsz__blk438_dn7), ((locals.var_phi_s0_dep_dn8 + locals.var_t6_dn8) - locals.var_vbsz__blk438_dn8), ((locals.var_phi_s0_dep_dn9 + locals.var_t6_dn9) - locals.var_vbsz__blk438_dn9), ((locals.var_phi_s0_dep_dn10 + locals.var_t6_dn10) - locals.var_vbsz__blk438_dn10), ((locals.var_phi_s0_dep_dn13 + locals.var_t6_dn13) - locals.var_vbsz__blk438_dn13),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign36380_e41850;
        locals.var_t5_dn0 = assign36380_e41850_d_n0;
        locals.var_t5_dn2 = assign36380_e41850_d_n2;
        locals.var_t5_dn4 = assign36380_e41850_d_n4;
        locals.var_t5_dn5 = assign36380_e41850_d_n5;
        locals.var_t5_dn6 = assign36380_e41850_d_n6;
        locals.var_t5_dn7 = assign36380_e41850_d_n7;
        locals.var_t5_dn8 = assign36380_e41850_d_n8;
        locals.var_t5_dn9 = assign36380_e41850_d_n9;
        locals.var_t5_dn10 = assign36380_e41850_d_n10;
        locals.var_t5_dn13 = assign36380_e41850_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign36390_e41864, assign36390_e41864_d_n0, assign36390_e41864_d_n2, assign36390_e41864_d_n4, assign36390_e41864_d_n5, assign36390_e41864_d_n6, assign36390_e41864_d_n7, assign36390_e41864_d_n8, assign36390_e41864_d_n9, assign36390_e41864_d_n10, assign36390_e41864_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard821 != 0.0)) {
        let assign36390_e41859: f64 = (locals.var_vdsz__blk439 * locals.var_t0);
        let assign36390_e41861: f64 = (assign36390_e41859 * locals.var_t5);
        let assign36390_e41862: f64 = (locals.var_t4 + assign36390_e41861);
        (assign36390_e41862, (locals.var_t4_dn0 + ((((locals.var_vdsz__blk439_dn0 * locals.var_t0) + (locals.var_vdsz__blk439 * locals.var_t0_dn0)) * locals.var_t5) + (assign36390_e41859 * locals.var_t5_dn0))), (locals.var_t4_dn2 + ((((locals.var_vdsz__blk439_dn2 * locals.var_t0) + (locals.var_vdsz__blk439 * locals.var_t0_dn2)) * locals.var_t5) + (assign36390_e41859 * locals.var_t5_dn2))), (locals.var_t4_dn4 + ((((locals.var_vdsz__blk439_dn4 * locals.var_t0) + (locals.var_vdsz__blk439 * locals.var_t0_dn4)) * locals.var_t5) + (assign36390_e41859 * locals.var_t5_dn4))), (locals.var_t4_dn5 + ((((locals.var_vdsz__blk439_dn5 * locals.var_t0) + (locals.var_vdsz__blk439 * locals.var_t0_dn5)) * locals.var_t5) + (assign36390_e41859 * locals.var_t5_dn5))), (locals.var_t4_dn6 + ((((locals.var_vdsz__blk439_dn6 * locals.var_t0) + (locals.var_vdsz__blk439 * locals.var_t0_dn6)) * locals.var_t5) + (assign36390_e41859 * locals.var_t5_dn6))), (locals.var_t4_dn7 + ((((locals.var_vdsz__blk439_dn7 * locals.var_t0) + (locals.var_vdsz__blk439 * locals.var_t0_dn7)) * locals.var_t5) + (assign36390_e41859 * locals.var_t5_dn7))), (locals.var_t4_dn8 + ((((locals.var_vdsz__blk439_dn8 * locals.var_t0) + (locals.var_vdsz__blk439 * locals.var_t0_dn8)) * locals.var_t5) + (assign36390_e41859 * locals.var_t5_dn8))), (locals.var_t4_dn9 + ((((locals.var_vdsz__blk439_dn9 * locals.var_t0) + (locals.var_vdsz__blk439 * locals.var_t0_dn9)) * locals.var_t5) + (assign36390_e41859 * locals.var_t5_dn9))), (locals.var_t4_dn10 + ((((locals.var_vdsz__blk439_dn10 * locals.var_t0) + (locals.var_vdsz__blk439 * locals.var_t0_dn10)) * locals.var_t5) + (assign36390_e41859 * locals.var_t5_dn10))), (locals.var_t4_dn13 + ((((locals.var_vdsz__blk439_dn13 * locals.var_t0) + (locals.var_vdsz__blk439 * locals.var_t0_dn13)) * locals.var_t5) + (assign36390_e41859 * locals.var_t5_dn13))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign36390_e41864;
        locals.var_t4_dn0 = assign36390_e41864_d_n0;
        locals.var_t4_dn2 = assign36390_e41864_d_n2;
        locals.var_t4_dn4 = assign36390_e41864_d_n4;
        locals.var_t4_dn5 = assign36390_e41864_d_n5;
        locals.var_t4_dn6 = assign36390_e41864_d_n6;
        locals.var_t4_dn7 = assign36390_e41864_d_n7;
        locals.var_t4_dn8 = assign36390_e41864_d_n8;
        locals.var_t4_dn9 = assign36390_e41864_d_n9;
        locals.var_t4_dn10 = assign36390_e41864_d_n10;
        locals.var_t4_dn13 = assign36390_e41864_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign36400_e41874, assign36400_e41874_d_n0, assign36400_e41874_d_n2, assign36400_e41874_d_n4, assign36400_e41874_d_n5, assign36400_e41874_d_n6, assign36400_e41874_d_n7, assign36400_e41874_d_n8, assign36400_e41874_d_n9, assign36400_e41874_d_n10, assign36400_e41874_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard821 != 0.0)) {
        let assign36400_e41872: f64 = (locals.var_t9 * locals.var_t4);
        (assign36400_e41872, ((locals.var_t9_dn0 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn0)), ((locals.var_t9_dn2 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn2)), ((locals.var_t9_dn4 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn4)), ((locals.var_t9_dn5 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn5)), ((locals.var_t9_dn6 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn6)), ((locals.var_t9_dn7 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn7)), ((locals.var_t9_dn8 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn8)), ((locals.var_t9_dn9 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn9)), ((locals.var_t9_dn10 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn10)), ((locals.var_t9_dn13 * locals.var_t4) + (locals.var_t9 * locals.var_t4_dn13)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign36400_e41874;
        locals.var_t6_dn0 = assign36400_e41874_d_n0;
        locals.var_t6_dn2 = assign36400_e41874_d_n2;
        locals.var_t6_dn4 = assign36400_e41874_d_n4;
        locals.var_t6_dn5 = assign36400_e41874_d_n5;
        locals.var_t6_dn6 = assign36400_e41874_d_n6;
        locals.var_t6_dn7 = assign36400_e41874_d_n7;
        locals.var_t6_dn8 = assign36400_e41874_d_n8;
        locals.var_t6_dn9 = assign36400_e41874_d_n9;
        locals.var_t6_dn10 = assign36400_e41874_d_n10;
        locals.var_t6_dn13 = assign36400_e41874_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign36410_e41882, assign36410_e41882_d_n0, assign36410_e41882_d_n2, assign36410_e41882_d_n4, assign36410_e41882_d_n5, assign36410_e41882_d_n6, assign36410_e41882_d_n7, assign36410_e41882_d_n8, assign36410_e41882_d_n9, assign36410_e41882_d_n10, assign36410_e41882_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard821 != 0.0)) {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign36410_e41882;
        locals.var_t9_dn0 = assign36410_e41882_d_n0;
        locals.var_t9_dn2 = assign36410_e41882_d_n2;
        locals.var_t9_dn4 = assign36410_e41882_d_n4;
        locals.var_t9_dn5 = assign36410_e41882_d_n5;
        locals.var_t9_dn6 = assign36410_e41882_d_n6;
        locals.var_t9_dn7 = assign36410_e41882_d_n7;
        locals.var_t9_dn8 = assign36410_e41882_d_n8;
        locals.var_t9_dn9 = assign36410_e41882_d_n9;
        locals.var_t9_dn10 = assign36410_e41882_d_n10;
        locals.var_t9_dn13 = assign36410_e41882_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign36420_e41891, assign36420_e41891_d_n0, assign36420_e41891_d_n2, assign36420_e41891_d_n4, assign36420_e41891_d_n5, assign36420_e41891_d_n6, assign36420_e41891_d_n7, assign36420_e41891_d_n8, assign36420_e41891_d_n9, assign36420_e41891_d_n10, assign36420_e41891_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard821 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign36420_e41891;
        locals.var_t9_dn0 = assign36420_e41891_d_n0;
        locals.var_t9_dn2 = assign36420_e41891_d_n2;
        locals.var_t9_dn4 = assign36420_e41891_d_n4;
        locals.var_t9_dn5 = assign36420_e41891_d_n5;
        locals.var_t9_dn6 = assign36420_e41891_d_n6;
        locals.var_t9_dn7 = assign36420_e41891_d_n7;
        locals.var_t9_dn8 = assign36420_e41891_d_n8;
        locals.var_t9_dn9 = assign36420_e41891_d_n9;
        locals.var_t9_dn10 = assign36420_e41891_d_n10;
        locals.var_t9_dn13 = assign36420_e41891_d_n13;
        locals.var_t9_rv = 0.0;

        let assign36430_e41894: f64 = if p.p287 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard823 = assign36430_e41894;
        locals.var_guard823_rv = 0.0;

        let (assign36440_e41904, assign36440_e41904_d_n0, assign36440_e41904_d_n2, assign36440_e41904_d_n4, assign36440_e41904_d_n5, assign36440_e41904_d_n6, assign36440_e41904_d_n7, assign36440_e41904_d_n8, assign36440_e41904_d_n9, assign36440_e41904_d_n10, assign36440_e41904_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard823 != 0.0)) {
        let assign36440_e41902: f64 = (locals.var_beta * locals.var_gdl0);
        (assign36440_e41902, (locals.var_beta_dn0 * locals.var_gdl0), (locals.var_beta_dn2 * locals.var_gdl0), (locals.var_beta_dn4 * locals.var_gdl0), (locals.var_beta_dn5 * locals.var_gdl0), (locals.var_beta_dn6 * locals.var_gdl0), (locals.var_beta_dn7 * locals.var_gdl0), (locals.var_beta_dn8 * locals.var_gdl0), (locals.var_beta_dn9 * locals.var_gdl0), (locals.var_beta_dn10 * locals.var_gdl0), (locals.var_beta_dn13 * locals.var_gdl0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign36440_e41904;
        locals.var_t1_dn0 = assign36440_e41904_d_n0;
        locals.var_t1_dn2 = assign36440_e41904_d_n2;
        locals.var_t1_dn4 = assign36440_e41904_d_n4;
        locals.var_t1_dn5 = assign36440_e41904_d_n5;
        locals.var_t1_dn6 = assign36440_e41904_d_n6;
        locals.var_t1_dn7 = assign36440_e41904_d_n7;
        locals.var_t1_dn8 = assign36440_e41904_d_n8;
        locals.var_t1_dn9 = assign36440_e41904_d_n9;
        locals.var_t1_dn10 = assign36440_e41904_d_n10;
        locals.var_t1_dn13 = assign36440_e41904_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign36450_e41914, assign36450_e41914_d_n0, assign36450_e41914_d_n2, assign36450_e41914_d_n4, assign36450_e41914_d_n5, assign36450_e41914_d_n6, assign36450_e41914_d_n7, assign36450_e41914_d_n8, assign36450_e41914_d_n9, assign36450_e41914_d_n10, assign36450_e41914_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard823 != 0.0)) {
        let assign36450_e41912: f64 = (locals.var_cox * locals.var_t1);
        (assign36450_e41912, ((locals.var_cox_dn0 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn0)), ((locals.var_cox_dn2 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn2)), ((locals.var_cox_dn4 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn4)), ((locals.var_cox_dn5 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn5)), ((locals.var_cox_dn6 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn6)), ((locals.var_cox_dn7 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn7)), ((locals.var_cox_dn8 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn8)), ((locals.var_cox_dn9 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn9)), ((locals.var_cox_dn10 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn10)), ((locals.var_cox_dn13 * locals.var_t1) + (locals.var_cox * locals.var_t1_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign36450_e41914;
        locals.var_t2_dn0 = assign36450_e41914_d_n0;
        locals.var_t2_dn2 = assign36450_e41914_d_n2;
        locals.var_t2_dn4 = assign36450_e41914_d_n4;
        locals.var_t2_dn5 = assign36450_e41914_d_n5;
        locals.var_t2_dn6 = assign36450_e41914_d_n6;
        locals.var_t2_dn7 = assign36450_e41914_d_n7;
        locals.var_t2_dn8 = assign36450_e41914_d_n8;
        locals.var_t2_dn9 = assign36450_e41914_d_n9;
        locals.var_t2_dn10 = assign36450_e41914_d_n10;
        locals.var_t2_dn13 = assign36450_e41914_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign36460_e41924, assign36460_e41924_d_n0, assign36460_e41924_d_n2, assign36460_e41924_d_n4, assign36460_e41924_d_n5, assign36460_e41924_d_n6, assign36460_e41924_d_n7, assign36460_e41924_d_n8, assign36460_e41924_d_n9, assign36460_e41924_d_n10, assign36460_e41924_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard823 != 0.0)) {
        let assign36460_e41922: f64 = (locals.var_t2 * locals.var_vdsz__blk439);
        (assign36460_e41922, ((locals.var_t2_dn0 * locals.var_vdsz__blk439) + (locals.var_t2 * locals.var_vdsz__blk439_dn0)), ((locals.var_t2_dn2 * locals.var_vdsz__blk439) + (locals.var_t2 * locals.var_vdsz__blk439_dn2)), ((locals.var_t2_dn4 * locals.var_vdsz__blk439) + (locals.var_t2 * locals.var_vdsz__blk439_dn4)), ((locals.var_t2_dn5 * locals.var_vdsz__blk439) + (locals.var_t2 * locals.var_vdsz__blk439_dn5)), ((locals.var_t2_dn6 * locals.var_vdsz__blk439) + (locals.var_t2 * locals.var_vdsz__blk439_dn6)), ((locals.var_t2_dn7 * locals.var_vdsz__blk439) + (locals.var_t2 * locals.var_vdsz__blk439_dn7)), ((locals.var_t2_dn8 * locals.var_vdsz__blk439) + (locals.var_t2 * locals.var_vdsz__blk439_dn8)), ((locals.var_t2_dn9 * locals.var_vdsz__blk439) + (locals.var_t2 * locals.var_vdsz__blk439_dn9)), ((locals.var_t2_dn10 * locals.var_vdsz__blk439) + (locals.var_t2 * locals.var_vdsz__blk439_dn10)), ((locals.var_t2_dn13 * locals.var_vdsz__blk439) + (locals.var_t2 * locals.var_vdsz__blk439_dn13)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn13,)
    }
};
        locals.var_t8 = assign36460_e41924;
        locals.var_t8_dn0 = assign36460_e41924_d_n0;
        locals.var_t8_dn2 = assign36460_e41924_d_n2;
        locals.var_t8_dn4 = assign36460_e41924_d_n4;
        locals.var_t8_dn5 = assign36460_e41924_d_n5;
        locals.var_t8_dn6 = assign36460_e41924_d_n6;
        locals.var_t8_dn7 = assign36460_e41924_d_n7;
        locals.var_t8_dn8 = assign36460_e41924_d_n8;
        locals.var_t8_dn9 = assign36460_e41924_d_n9;
        locals.var_t8_dn10 = assign36460_e41924_d_n10;
        locals.var_t8_dn13 = assign36460_e41924_d_n13;
        locals.var_t8_rv = 0.0;

        let (assign36470_e41933, assign36470_e41933_d_n0, assign36470_e41933_d_n2, assign36470_e41933_d_n4, assign36470_e41933_d_n5, assign36470_e41933_d_n6, assign36470_e41933_d_n7, assign36470_e41933_d_n8, assign36470_e41933_d_n9, assign36470_e41933_d_n10, assign36470_e41933_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard823 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn13,)
    }
};
        locals.var_t8 = assign36470_e41933;
        locals.var_t8_dn0 = assign36470_e41933_d_n0;
        locals.var_t8_dn2 = assign36470_e41933_d_n2;
        locals.var_t8_dn4 = assign36470_e41933_d_n4;
        locals.var_t8_dn5 = assign36470_e41933_d_n5;
        locals.var_t8_dn6 = assign36470_e41933_d_n6;
        locals.var_t8_dn7 = assign36470_e41933_d_n7;
        locals.var_t8_dn8 = assign36470_e41933_d_n8;
        locals.var_t8_dn9 = assign36470_e41933_d_n9;
        locals.var_t8_dn10 = assign36470_e41933_d_n10;
        locals.var_t8_dn13 = assign36470_e41933_d_n13;
        locals.var_t8_rv = 0.0;

        let assign36480_e41936: f64 = (locals.var_t9 + locals.var_t8);
        let assign36480_e41938: f64 = if assign36480_e41936 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard824 = assign36480_e41938;
        locals.var_guard824_rv = 0.0;

        let (assign36490_e41950, assign36490_e41950_d_n0, assign36490_e41950_d_n2, assign36490_e41950_d_n4, assign36490_e41950_d_n5, assign36490_e41950_d_n6, assign36490_e41950_d_n7, assign36490_e41950_d_n8, assign36490_e41950_d_n9, assign36490_e41950_d_n10, assign36490_e41950_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard824 != 0.0)) {
        let assign36490_e41947: f64 = (locals.var_t9 + locals.var_t8);
        let assign36490_e41948: f64 = (locals.var_pds * assign36490_e41947);
        (assign36490_e41948, ((locals.var_pds_dn0 * assign36490_e41947) + (locals.var_pds * (locals.var_t9_dn0 + locals.var_t8_dn0))), ((locals.var_pds_dn2 * assign36490_e41947) + (locals.var_pds * (locals.var_t9_dn2 + locals.var_t8_dn2))), ((locals.var_pds_dn4 * assign36490_e41947) + (locals.var_pds * (locals.var_t9_dn4 + locals.var_t8_dn4))), ((locals.var_pds_dn5 * assign36490_e41947) + (locals.var_pds * (locals.var_t9_dn5 + locals.var_t8_dn5))), ((locals.var_pds_dn6 * assign36490_e41947) + (locals.var_pds * (locals.var_t9_dn6 + locals.var_t8_dn6))), ((locals.var_pds_dn7 * assign36490_e41947) + (locals.var_pds * (locals.var_t9_dn7 + locals.var_t8_dn7))), ((locals.var_pds_dn8 * assign36490_e41947) + (locals.var_pds * (locals.var_t9_dn8 + locals.var_t8_dn8))), ((locals.var_pds_dn9 * assign36490_e41947) + (locals.var_pds * (locals.var_t9_dn9 + locals.var_t8_dn9))), ((locals.var_pds_dn10 * assign36490_e41947) + (locals.var_pds * (locals.var_t9_dn10 + locals.var_t8_dn10))), ((locals.var_pds_dn13 * assign36490_e41947) + (locals.var_pds * (locals.var_t9_dn13 + locals.var_t8_dn13))),)
    } else {
        (locals.var_idd1, locals.var_idd1_dn0, locals.var_idd1_dn2, locals.var_idd1_dn4, locals.var_idd1_dn5, locals.var_idd1_dn6, locals.var_idd1_dn7, locals.var_idd1_dn8, locals.var_idd1_dn9, locals.var_idd1_dn10, locals.var_idd1_dn13,)
    }
};
        locals.var_idd1 = assign36490_e41950;
        locals.var_idd1_dn0 = assign36490_e41950_d_n0;
        locals.var_idd1_dn2 = assign36490_e41950_d_n2;
        locals.var_idd1_dn4 = assign36490_e41950_d_n4;
        locals.var_idd1_dn5 = assign36490_e41950_d_n5;
        locals.var_idd1_dn6 = assign36490_e41950_d_n6;
        locals.var_idd1_dn7 = assign36490_e41950_d_n7;
        locals.var_idd1_dn8 = assign36490_e41950_d_n8;
        locals.var_idd1_dn9 = assign36490_e41950_d_n9;
        locals.var_idd1_dn10 = assign36490_e41950_d_n10;
        locals.var_idd1_dn13 = assign36490_e41950_d_n13;
        locals.var_idd1_rv = 0.0;

        let (assign36500_e41964, assign36500_e41964_d_n0, assign36500_e41964_d_n2, assign36500_e41964_d_n4, assign36500_e41964_d_n5, assign36500_e41964_d_n6, assign36500_e41964_d_n7, assign36500_e41964_d_n8, assign36500_e41964_d_n9, assign36500_e41964_d_n10, assign36500_e41964_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard824 != 0.0)) {
        let assign36500_e41959: f64 = (locals.var_betawl * locals.var_idd1);
        let assign36500_e41961: f64 = (assign36500_e41959 * locals.var_mu);
        let assign36500_e41962: f64 = (locals.var_ids0 + assign36500_e41961);
        (assign36500_e41962, (locals.var_ids0_dn0 + ((((locals.var_betawl_dn0 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn0)) * locals.var_mu) + (assign36500_e41959 * locals.var_mu_dn0))), (locals.var_ids0_dn2 + ((((locals.var_betawl_dn2 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn2)) * locals.var_mu) + (assign36500_e41959 * locals.var_mu_dn2))), (locals.var_ids0_dn4 + ((((locals.var_betawl_dn4 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn4)) * locals.var_mu) + (assign36500_e41959 * locals.var_mu_dn4))), (locals.var_ids0_dn5 + ((((locals.var_betawl_dn5 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn5)) * locals.var_mu) + (assign36500_e41959 * locals.var_mu_dn5))), (locals.var_ids0_dn6 + ((((locals.var_betawl_dn6 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn6)) * locals.var_mu) + (assign36500_e41959 * locals.var_mu_dn6))), (locals.var_ids0_dn7 + ((((locals.var_betawl_dn7 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn7)) * locals.var_mu) + (assign36500_e41959 * locals.var_mu_dn7))), (locals.var_ids0_dn8 + ((((locals.var_betawl_dn8 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn8)) * locals.var_mu) + (assign36500_e41959 * locals.var_mu_dn8))), (locals.var_ids0_dn9 + ((((locals.var_betawl_dn9 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn9)) * locals.var_mu) + (assign36500_e41959 * locals.var_mu_dn9))), (locals.var_ids0_dn10 + ((((locals.var_betawl_dn10 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn10)) * locals.var_mu) + (assign36500_e41959 * locals.var_mu_dn10))), (locals.var_ids0_dn13 + ((((locals.var_betawl_dn13 * locals.var_idd1) + (locals.var_betawl * locals.var_idd1_dn13)) * locals.var_mu) + (assign36500_e41959 * locals.var_mu_dn13))),)
    } else {
        (locals.var_ids0, locals.var_ids0_dn0, locals.var_ids0_dn2, locals.var_ids0_dn4, locals.var_ids0_dn5, locals.var_ids0_dn6, locals.var_ids0_dn7, locals.var_ids0_dn8, locals.var_ids0_dn9, locals.var_ids0_dn10, locals.var_ids0_dn13,)
    }
};
        locals.var_ids0 = assign36500_e41964;
        locals.var_ids0_dn0 = assign36500_e41964_d_n0;
        locals.var_ids0_dn2 = assign36500_e41964_d_n2;
        locals.var_ids0_dn4 = assign36500_e41964_d_n4;
        locals.var_ids0_dn5 = assign36500_e41964_d_n5;
        locals.var_ids0_dn6 = assign36500_e41964_d_n6;
        locals.var_ids0_dn7 = assign36500_e41964_d_n7;
        locals.var_ids0_dn8 = assign36500_e41964_d_n8;
        locals.var_ids0_dn9 = assign36500_e41964_d_n9;
        locals.var_ids0_dn10 = assign36500_e41964_d_n10;
        locals.var_ids0_dn13 = assign36500_e41964_d_n13;
        locals.var_ids0_rv = 0.0;

        let assign36510_e41971: f64 = if ((locals.var_flg_rsrd == 2.0) || (locals.var_flg_rsrd == 3.0)) { 1.0 } else { 0.0 };
        locals.var_guard825 = assign36510_e41971;
        locals.var_guard825_rv = 0.0;

        let assign36520_e41974: f64 = if p.p296 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard826 = assign36520_e41974;
        locals.var_guard826_rv = 0.0;

        let (assign36530_e41984, assign36530_e41984_d_n0, assign36530_e41984_d_n2, assign36530_e41984_d_n4, assign36530_e41984_d_n5, assign36530_e41984_d_n6, assign36530_e41984_d_n7, assign36530_e41984_d_n8, assign36530_e41984_d_n9, assign36530_e41984_d_n10, assign36530_e41984_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard825 != 0.0)) && (locals.var_guard826 != 0.0)) {
        (locals.var_rd23e, locals.var_rd23e_dn0, locals.var_rd23e_dn2, locals.var_rd23e_dn4, locals.var_rd23e_dn5, locals.var_rd23e_dn6, locals.var_rd23e_dn7, locals.var_rd23e_dn8, locals.var_rd23e_dn9, locals.var_rd23e_dn10, locals.var_rd23e_dn13,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign36530_e41984;
        locals.var_t4_dn0 = assign36530_e41984_d_n0;
        locals.var_t4_dn2 = assign36530_e41984_d_n2;
        locals.var_t4_dn4 = assign36530_e41984_d_n4;
        locals.var_t4_dn5 = assign36530_e41984_d_n5;
        locals.var_t4_dn6 = assign36530_e41984_d_n6;
        locals.var_t4_dn7 = assign36530_e41984_d_n7;
        locals.var_t4_dn8 = assign36530_e41984_d_n8;
        locals.var_t4_dn9 = assign36530_e41984_d_n9;
        locals.var_t4_dn10 = assign36530_e41984_d_n10;
        locals.var_t4_dn13 = assign36530_e41984_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign36540_e41998, assign36540_e41998_d_n0, assign36540_e41998_d_n2, assign36540_e41998_d_n4, assign36540_e41998_d_n5, assign36540_e41998_d_n6, assign36540_e41998_d_n7, assign36540_e41998_d_n8, assign36540_e41998_d_n9, assign36540_e41998_d_n10, assign36540_e41998_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard825 != 0.0)) && (locals.var_guard826 != 0.0)) {
        let assign36540_e41995: f64 = (locals.var_vgse - p.p300);
        let assign36540_e41996: f64 = (locals.var_uc_rd24 * assign36540_e41995);
        (assign36540_e41996, (locals.var_uc_rd24 * locals.var_vgse_dn0), (locals.var_uc_rd24 * locals.var_vgse_dn2), 0.0, 0.0, (locals.var_uc_rd24 * locals.var_vgse_dn6), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign36540_e41998;
        locals.var_t1_dn0 = assign36540_e41998_d_n0;
        locals.var_t1_dn2 = assign36540_e41998_d_n2;
        locals.var_t1_dn4 = assign36540_e41998_d_n4;
        locals.var_t1_dn5 = assign36540_e41998_d_n5;
        locals.var_t1_dn6 = assign36540_e41998_d_n6;
        locals.var_t1_dn7 = assign36540_e41998_d_n7;
        locals.var_t1_dn8 = assign36540_e41998_d_n8;
        locals.var_t1_dn9 = assign36540_e41998_d_n9;
        locals.var_t1_dn10 = assign36540_e41998_d_n10;
        locals.var_t1_dn13 = assign36540_e41998_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign36550_e42014, assign36550_e42014_d_n0, assign36550_e42014_d_n2, assign36550_e42014_d_n4, assign36550_e42014_d_n5, assign36550_e42014_d_n6, assign36550_e42014_d_n7, assign36550_e42014_d_n8, assign36550_e42014_d_n9, assign36550_e42014_d_n10, assign36550_e42014_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard825 != 0.0)) && (locals.var_guard826 != 0.0)) {
        let assign36550_e42008: f64 = (locals.var_t1 - locals.var_t4);
        let assign36550_e42011: f64 = (0.01 * 0.01);
        let assign36550_e42012: f64 = (assign36550_e42008 - assign36550_e42011);
        (assign36550_e42012, (locals.var_t1_dn0 - locals.var_t4_dn0), (locals.var_t1_dn2 - locals.var_t4_dn2), (locals.var_t1_dn4 - locals.var_t4_dn4), (locals.var_t1_dn5 - locals.var_t4_dn5), (locals.var_t1_dn6 - locals.var_t4_dn6), (locals.var_t1_dn7 - locals.var_t4_dn7), (locals.var_t1_dn8 - locals.var_t4_dn8), (locals.var_t1_dn9 - locals.var_t4_dn9), (locals.var_t1_dn10 - locals.var_t4_dn10), (locals.var_t1_dn13 - locals.var_t4_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign36550_e42014;
        locals.var_tmf1_dn0 = assign36550_e42014_d_n0;
        locals.var_tmf1_dn2 = assign36550_e42014_d_n2;
        locals.var_tmf1_dn4 = assign36550_e42014_d_n4;
        locals.var_tmf1_dn5 = assign36550_e42014_d_n5;
        locals.var_tmf1_dn6 = assign36550_e42014_d_n6;
        locals.var_tmf1_dn7 = assign36550_e42014_d_n7;
        locals.var_tmf1_dn8 = assign36550_e42014_d_n8;
        locals.var_tmf1_dn9 = assign36550_e42014_d_n9;
        locals.var_tmf1_dn10 = assign36550_e42014_d_n10;
        locals.var_tmf1_dn13 = assign36550_e42014_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign36560_e42030, assign36560_e42030_d_n0, assign36560_e42030_d_n2, assign36560_e42030_d_n4, assign36560_e42030_d_n5, assign36560_e42030_d_n6, assign36560_e42030_d_n7, assign36560_e42030_d_n8, assign36560_e42030_d_n9, assign36560_e42030_d_n10, assign36560_e42030_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard825 != 0.0)) && (locals.var_guard826 != 0.0)) {
        let assign36560_e42024: f64 = (4.0 * locals.var_t4);
        let assign36560_e42027: f64 = (0.01 * 0.01);
        let assign36560_e42028: f64 = (assign36560_e42024 * assign36560_e42027);
        (assign36560_e42028, ((4.0 * locals.var_t4_dn0) * assign36560_e42027), ((4.0 * locals.var_t4_dn2) * assign36560_e42027), ((4.0 * locals.var_t4_dn4) * assign36560_e42027), ((4.0 * locals.var_t4_dn5) * assign36560_e42027), ((4.0 * locals.var_t4_dn6) * assign36560_e42027), ((4.0 * locals.var_t4_dn7) * assign36560_e42027), ((4.0 * locals.var_t4_dn8) * assign36560_e42027), ((4.0 * locals.var_t4_dn9) * assign36560_e42027), ((4.0 * locals.var_t4_dn10) * assign36560_e42027), ((4.0 * locals.var_t4_dn13) * assign36560_e42027),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign36560_e42030;
        locals.var_tmf2_dn0 = assign36560_e42030_d_n0;
        locals.var_tmf2_dn2 = assign36560_e42030_d_n2;
        locals.var_tmf2_dn4 = assign36560_e42030_d_n4;
        locals.var_tmf2_dn5 = assign36560_e42030_d_n5;
        locals.var_tmf2_dn6 = assign36560_e42030_d_n6;
        locals.var_tmf2_dn7 = assign36560_e42030_d_n7;
        locals.var_tmf2_dn8 = assign36560_e42030_d_n8;
        locals.var_tmf2_dn9 = assign36560_e42030_d_n9;
        locals.var_tmf2_dn10 = assign36560_e42030_d_n10;
        locals.var_tmf2_dn13 = assign36560_e42030_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign36570_e42046, assign36570_e42046_d_n0, assign36570_e42046_d_n2, assign36570_e42046_d_n4, assign36570_e42046_d_n5, assign36570_e42046_d_n6, assign36570_e42046_d_n7, assign36570_e42046_d_n8, assign36570_e42046_d_n9, assign36570_e42046_d_n10, assign36570_e42046_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard825 != 0.0)) && (locals.var_guard826 != 0.0)) {
        let (assign36570_e42044, assign36570_e42044_d_n0, assign36570_e42044_d_n2, assign36570_e42044_d_n4, assign36570_e42044_d_n5, assign36570_e42044_d_n6, assign36570_e42044_d_n7, assign36570_e42044_d_n8, assign36570_e42044_d_n9, assign36570_e42044_d_n10, assign36570_e42044_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign36570_e42043: f64 = (-locals.var_tmf2);
                (assign36570_e42043, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign36570_e42044, assign36570_e42044_d_n0, assign36570_e42044_d_n2, assign36570_e42044_d_n4, assign36570_e42044_d_n5, assign36570_e42044_d_n6, assign36570_e42044_d_n7, assign36570_e42044_d_n8, assign36570_e42044_d_n9, assign36570_e42044_d_n10, assign36570_e42044_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign36570_e42046;
        locals.var_tmf2_dn0 = assign36570_e42046_d_n0;
        locals.var_tmf2_dn2 = assign36570_e42046_d_n2;
        locals.var_tmf2_dn4 = assign36570_e42046_d_n4;
        locals.var_tmf2_dn5 = assign36570_e42046_d_n5;
        locals.var_tmf2_dn6 = assign36570_e42046_d_n6;
        locals.var_tmf2_dn7 = assign36570_e42046_d_n7;
        locals.var_tmf2_dn8 = assign36570_e42046_d_n8;
        locals.var_tmf2_dn9 = assign36570_e42046_d_n9;
        locals.var_tmf2_dn10 = assign36570_e42046_d_n10;
        locals.var_tmf2_dn13 = assign36570_e42046_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign36580_e42061, assign36580_e42061_d_n0, assign36580_e42061_d_n2, assign36580_e42061_d_n4, assign36580_e42061_d_n5, assign36580_e42061_d_n6, assign36580_e42061_d_n7, assign36580_e42061_d_n8, assign36580_e42061_d_n9, assign36580_e42061_d_n10, assign36580_e42061_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard825 != 0.0)) && (locals.var_guard826 != 0.0)) {
        let assign36580_e42056: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign36580_e42058: f64 = (assign36580_e42056 + locals.var_tmf2);
        let assign36580_e42059: f64 = (assign36580_e42058).sqrt();
        (assign36580_e42059, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign36580_e42059)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign36580_e42059)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign36580_e42059)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign36580_e42059)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign36580_e42059)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign36580_e42059)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign36580_e42059)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign36580_e42059)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign36580_e42059)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign36580_e42059)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign36580_e42061;
        locals.var_tmf2_dn0 = assign36580_e42061_d_n0;
        locals.var_tmf2_dn2 = assign36580_e42061_d_n2;
        locals.var_tmf2_dn4 = assign36580_e42061_d_n4;
        locals.var_tmf2_dn5 = assign36580_e42061_d_n5;
        locals.var_tmf2_dn6 = assign36580_e42061_d_n6;
        locals.var_tmf2_dn7 = assign36580_e42061_d_n7;
        locals.var_tmf2_dn8 = assign36580_e42061_d_n8;
        locals.var_tmf2_dn9 = assign36580_e42061_d_n9;
        locals.var_tmf2_dn10 = assign36580_e42061_d_n10;
        locals.var_tmf2_dn13 = assign36580_e42061_d_n13;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_116(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign36590_e42077, assign36590_e42077_d_n0, assign36590_e42077_d_n2, assign36590_e42077_d_n4, assign36590_e42077_d_n5, assign36590_e42077_d_n6, assign36590_e42077_d_n7, assign36590_e42077_d_n8, assign36590_e42077_d_n9, assign36590_e42077_d_n10, assign36590_e42077_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard825 != 0.0)) && (locals.var_guard826 != 0.0)) {
        let assign36590_e42073: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign36590_e42074: f64 = (1.0 + assign36590_e42073);
        let assign36590_e42075: f64 = (0.5 * assign36590_e42074);
        (assign36590_e42075, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign36590_e42077;
        locals.var_t0_dn0 = assign36590_e42077_d_n0;
        locals.var_t0_dn2 = assign36590_e42077_d_n2;
        locals.var_t0_dn4 = assign36590_e42077_d_n4;
        locals.var_t0_dn5 = assign36590_e42077_d_n5;
        locals.var_t0_dn6 = assign36590_e42077_d_n6;
        locals.var_t0_dn7 = assign36590_e42077_d_n7;
        locals.var_t0_dn8 = assign36590_e42077_d_n8;
        locals.var_t0_dn9 = assign36590_e42077_d_n9;
        locals.var_t0_dn10 = assign36590_e42077_d_n10;
        locals.var_t0_dn13 = assign36590_e42077_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign36600_e42093, assign36600_e42093_d_n0, assign36600_e42093_d_n2, assign36600_e42093_d_n4, assign36600_e42093_d_n5, assign36600_e42093_d_n6, assign36600_e42093_d_n7, assign36600_e42093_d_n8, assign36600_e42093_d_n9, assign36600_e42093_d_n10, assign36600_e42093_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard825 != 0.0)) && (locals.var_guard826 != 0.0)) {
        let assign36600_e42089: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign36600_e42090: f64 = (0.5 * assign36600_e42089);
        let assign36600_e42091: f64 = (locals.var_t4 + assign36600_e42090);
        (assign36600_e42091, (locals.var_t4_dn0 + (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t4_dn2 + (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t4_dn4 + (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t4_dn5 + (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t4_dn6 + (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t4_dn7 + (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t4_dn8 + (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t4_dn9 + (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t4_dn10 + (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t4_dn13 + (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign36600_e42093;
        locals.var_t2_dn0 = assign36600_e42093_d_n0;
        locals.var_t2_dn2 = assign36600_e42093_d_n2;
        locals.var_t2_dn4 = assign36600_e42093_d_n4;
        locals.var_t2_dn5 = assign36600_e42093_d_n5;
        locals.var_t2_dn6 = assign36600_e42093_d_n6;
        locals.var_t2_dn7 = assign36600_e42093_d_n7;
        locals.var_t2_dn8 = assign36600_e42093_d_n8;
        locals.var_t2_dn9 = assign36600_e42093_d_n9;
        locals.var_t2_dn10 = assign36600_e42093_d_n10;
        locals.var_t2_dn13 = assign36600_e42093_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign36610_e42107, assign36610_e42107_d_n0, assign36610_e42107_d_n2, assign36610_e42107_d_n4, assign36610_e42107_d_n5, assign36610_e42107_d_n6, assign36610_e42107_d_n7, assign36610_e42107_d_n8, assign36610_e42107_d_n9, assign36610_e42107_d_n10, assign36610_e42107_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard825 != 0.0)) && (locals.var_guard826 != 0.0)) {
        let assign36610_e42104: f64 = (p.p296 + 1.0);
        let assign36610_e42105: f64 = (locals.var_t4 * assign36610_e42104);
        (assign36610_e42105, (locals.var_t4_dn0 * assign36610_e42104), (locals.var_t4_dn2 * assign36610_e42104), (locals.var_t4_dn4 * assign36610_e42104), (locals.var_t4_dn5 * assign36610_e42104), (locals.var_t4_dn6 * assign36610_e42104), (locals.var_t4_dn7 * assign36610_e42104), (locals.var_t4_dn8 * assign36610_e42104), (locals.var_t4_dn9 * assign36610_e42104), (locals.var_t4_dn10 * assign36610_e42104), (locals.var_t4_dn13 * assign36610_e42104),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign36610_e42107;
        locals.var_t3_dn0 = assign36610_e42107_d_n0;
        locals.var_t3_dn2 = assign36610_e42107_d_n2;
        locals.var_t3_dn4 = assign36610_e42107_d_n4;
        locals.var_t3_dn5 = assign36610_e42107_d_n5;
        locals.var_t3_dn6 = assign36610_e42107_d_n6;
        locals.var_t3_dn7 = assign36610_e42107_d_n7;
        locals.var_t3_dn8 = assign36610_e42107_d_n8;
        locals.var_t3_dn9 = assign36610_e42107_d_n9;
        locals.var_t3_dn10 = assign36610_e42107_d_n10;
        locals.var_t3_dn13 = assign36610_e42107_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign36620_e42123, assign36620_e42123_d_n0, assign36620_e42123_d_n2, assign36620_e42123_d_n4, assign36620_e42123_d_n5, assign36620_e42123_d_n6, assign36620_e42123_d_n7, assign36620_e42123_d_n8, assign36620_e42123_d_n9, assign36620_e42123_d_n10, assign36620_e42123_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard825 != 0.0)) && (locals.var_guard826 != 0.0)) {
        let assign36620_e42117: f64 = (locals.var_t3 - locals.var_t2);
        let assign36620_e42120: f64 = (0.01 * 0.01);
        let assign36620_e42121: f64 = (assign36620_e42117 - assign36620_e42120);
        (assign36620_e42121, (locals.var_t3_dn0 - locals.var_t2_dn0), (locals.var_t3_dn2 - locals.var_t2_dn2), (locals.var_t3_dn4 - locals.var_t2_dn4), (locals.var_t3_dn5 - locals.var_t2_dn5), (locals.var_t3_dn6 - locals.var_t2_dn6), (locals.var_t3_dn7 - locals.var_t2_dn7), (locals.var_t3_dn8 - locals.var_t2_dn8), (locals.var_t3_dn9 - locals.var_t2_dn9), (locals.var_t3_dn10 - locals.var_t2_dn10), (locals.var_t3_dn13 - locals.var_t2_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign36620_e42123;
        locals.var_tmf1_dn0 = assign36620_e42123_d_n0;
        locals.var_tmf1_dn2 = assign36620_e42123_d_n2;
        locals.var_tmf1_dn4 = assign36620_e42123_d_n4;
        locals.var_tmf1_dn5 = assign36620_e42123_d_n5;
        locals.var_tmf1_dn6 = assign36620_e42123_d_n6;
        locals.var_tmf1_dn7 = assign36620_e42123_d_n7;
        locals.var_tmf1_dn8 = assign36620_e42123_d_n8;
        locals.var_tmf1_dn9 = assign36620_e42123_d_n9;
        locals.var_tmf1_dn10 = assign36620_e42123_d_n10;
        locals.var_tmf1_dn13 = assign36620_e42123_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign36630_e42139, assign36630_e42139_d_n0, assign36630_e42139_d_n2, assign36630_e42139_d_n4, assign36630_e42139_d_n5, assign36630_e42139_d_n6, assign36630_e42139_d_n7, assign36630_e42139_d_n8, assign36630_e42139_d_n9, assign36630_e42139_d_n10, assign36630_e42139_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard825 != 0.0)) && (locals.var_guard826 != 0.0)) {
        let assign36630_e42133: f64 = (4.0 * locals.var_t3);
        let assign36630_e42136: f64 = (0.01 * 0.01);
        let assign36630_e42137: f64 = (assign36630_e42133 * assign36630_e42136);
        (assign36630_e42137, ((4.0 * locals.var_t3_dn0) * assign36630_e42136), ((4.0 * locals.var_t3_dn2) * assign36630_e42136), ((4.0 * locals.var_t3_dn4) * assign36630_e42136), ((4.0 * locals.var_t3_dn5) * assign36630_e42136), ((4.0 * locals.var_t3_dn6) * assign36630_e42136), ((4.0 * locals.var_t3_dn7) * assign36630_e42136), ((4.0 * locals.var_t3_dn8) * assign36630_e42136), ((4.0 * locals.var_t3_dn9) * assign36630_e42136), ((4.0 * locals.var_t3_dn10) * assign36630_e42136), ((4.0 * locals.var_t3_dn13) * assign36630_e42136),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign36630_e42139;
        locals.var_tmf2_dn0 = assign36630_e42139_d_n0;
        locals.var_tmf2_dn2 = assign36630_e42139_d_n2;
        locals.var_tmf2_dn4 = assign36630_e42139_d_n4;
        locals.var_tmf2_dn5 = assign36630_e42139_d_n5;
        locals.var_tmf2_dn6 = assign36630_e42139_d_n6;
        locals.var_tmf2_dn7 = assign36630_e42139_d_n7;
        locals.var_tmf2_dn8 = assign36630_e42139_d_n8;
        locals.var_tmf2_dn9 = assign36630_e42139_d_n9;
        locals.var_tmf2_dn10 = assign36630_e42139_d_n10;
        locals.var_tmf2_dn13 = assign36630_e42139_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign36640_e42155, assign36640_e42155_d_n0, assign36640_e42155_d_n2, assign36640_e42155_d_n4, assign36640_e42155_d_n5, assign36640_e42155_d_n6, assign36640_e42155_d_n7, assign36640_e42155_d_n8, assign36640_e42155_d_n9, assign36640_e42155_d_n10, assign36640_e42155_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard825 != 0.0)) && (locals.var_guard826 != 0.0)) {
        let (assign36640_e42153, assign36640_e42153_d_n0, assign36640_e42153_d_n2, assign36640_e42153_d_n4, assign36640_e42153_d_n5, assign36640_e42153_d_n6, assign36640_e42153_d_n7, assign36640_e42153_d_n8, assign36640_e42153_d_n9, assign36640_e42153_d_n10, assign36640_e42153_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign36640_e42152: f64 = (-locals.var_tmf2);
                (assign36640_e42152, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign36640_e42153, assign36640_e42153_d_n0, assign36640_e42153_d_n2, assign36640_e42153_d_n4, assign36640_e42153_d_n5, assign36640_e42153_d_n6, assign36640_e42153_d_n7, assign36640_e42153_d_n8, assign36640_e42153_d_n9, assign36640_e42153_d_n10, assign36640_e42153_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign36640_e42155;
        locals.var_tmf2_dn0 = assign36640_e42155_d_n0;
        locals.var_tmf2_dn2 = assign36640_e42155_d_n2;
        locals.var_tmf2_dn4 = assign36640_e42155_d_n4;
        locals.var_tmf2_dn5 = assign36640_e42155_d_n5;
        locals.var_tmf2_dn6 = assign36640_e42155_d_n6;
        locals.var_tmf2_dn7 = assign36640_e42155_d_n7;
        locals.var_tmf2_dn8 = assign36640_e42155_d_n8;
        locals.var_tmf2_dn9 = assign36640_e42155_d_n9;
        locals.var_tmf2_dn10 = assign36640_e42155_d_n10;
        locals.var_tmf2_dn13 = assign36640_e42155_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign36650_e42170, assign36650_e42170_d_n0, assign36650_e42170_d_n2, assign36650_e42170_d_n4, assign36650_e42170_d_n5, assign36650_e42170_d_n6, assign36650_e42170_d_n7, assign36650_e42170_d_n8, assign36650_e42170_d_n9, assign36650_e42170_d_n10, assign36650_e42170_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard825 != 0.0)) && (locals.var_guard826 != 0.0)) {
        let assign36650_e42165: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign36650_e42167: f64 = (assign36650_e42165 + locals.var_tmf2);
        let assign36650_e42168: f64 = (assign36650_e42167).sqrt();
        (assign36650_e42168, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign36650_e42168)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign36650_e42168)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign36650_e42168)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign36650_e42168)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign36650_e42168)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign36650_e42168)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign36650_e42168)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign36650_e42168)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign36650_e42168)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign36650_e42168)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign36650_e42170;
        locals.var_tmf2_dn0 = assign36650_e42170_d_n0;
        locals.var_tmf2_dn2 = assign36650_e42170_d_n2;
        locals.var_tmf2_dn4 = assign36650_e42170_d_n4;
        locals.var_tmf2_dn5 = assign36650_e42170_d_n5;
        locals.var_tmf2_dn6 = assign36650_e42170_d_n6;
        locals.var_tmf2_dn7 = assign36650_e42170_d_n7;
        locals.var_tmf2_dn8 = assign36650_e42170_d_n8;
        locals.var_tmf2_dn9 = assign36650_e42170_d_n9;
        locals.var_tmf2_dn10 = assign36650_e42170_d_n10;
        locals.var_tmf2_dn13 = assign36650_e42170_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign36660_e42186, assign36660_e42186_d_n0, assign36660_e42186_d_n2, assign36660_e42186_d_n4, assign36660_e42186_d_n5, assign36660_e42186_d_n6, assign36660_e42186_d_n7, assign36660_e42186_d_n8, assign36660_e42186_d_n9, assign36660_e42186_d_n10, assign36660_e42186_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard825 != 0.0)) && (locals.var_guard826 != 0.0)) {
        let assign36660_e42182: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign36660_e42183: f64 = (1.0 + assign36660_e42182);
        let assign36660_e42184: f64 = (0.5 * assign36660_e42183);
        (assign36660_e42184, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign36660_e42186;
        locals.var_t0_dn0 = assign36660_e42186_d_n0;
        locals.var_t0_dn2 = assign36660_e42186_d_n2;
        locals.var_t0_dn4 = assign36660_e42186_d_n4;
        locals.var_t0_dn5 = assign36660_e42186_d_n5;
        locals.var_t0_dn6 = assign36660_e42186_d_n6;
        locals.var_t0_dn7 = assign36660_e42186_d_n7;
        locals.var_t0_dn8 = assign36660_e42186_d_n8;
        locals.var_t0_dn9 = assign36660_e42186_d_n9;
        locals.var_t0_dn10 = assign36660_e42186_d_n10;
        locals.var_t0_dn13 = assign36660_e42186_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign36670_e42202, assign36670_e42202_d_n0, assign36670_e42202_d_n2, assign36670_e42202_d_n4, assign36670_e42202_d_n5, assign36670_e42202_d_n6, assign36670_e42202_d_n7, assign36670_e42202_d_n8, assign36670_e42202_d_n9, assign36670_e42202_d_n10, assign36670_e42202_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard825 != 0.0)) && (locals.var_guard826 != 0.0)) {
        let assign36670_e42198: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign36670_e42199: f64 = (0.5 * assign36670_e42198);
        let assign36670_e42200: f64 = (locals.var_t3 - assign36670_e42199);
        (assign36670_e42200, (locals.var_t3_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t3_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t3_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t3_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t3_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t3_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t3_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t3_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t3_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t3_dn13 - (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign36670_e42202;
        locals.var_t7_dn0 = assign36670_e42202_d_n0;
        locals.var_t7_dn2 = assign36670_e42202_d_n2;
        locals.var_t7_dn4 = assign36670_e42202_d_n4;
        locals.var_t7_dn5 = assign36670_e42202_d_n5;
        locals.var_t7_dn6 = assign36670_e42202_d_n6;
        locals.var_t7_dn7 = assign36670_e42202_d_n7;
        locals.var_t7_dn8 = assign36670_e42202_d_n8;
        locals.var_t7_dn9 = assign36670_e42202_d_n9;
        locals.var_t7_dn10 = assign36670_e42202_d_n10;
        locals.var_t7_dn13 = assign36670_e42202_d_n13;
        locals.var_t7_rv = 0.0;

        let (assign36680_e42213, assign36680_e42213_d_n0, assign36680_e42213_d_n2, assign36680_e42213_d_n4, assign36680_e42213_d_n5, assign36680_e42213_d_n6, assign36680_e42213_d_n7, assign36680_e42213_d_n8, assign36680_e42213_d_n9, assign36680_e42213_d_n10, assign36680_e42213_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard825 != 0.0)) && (locals.var_guard826 == 0.0)) {
        (locals.var_rd23e, locals.var_rd23e_dn0, locals.var_rd23e_dn2, locals.var_rd23e_dn4, locals.var_rd23e_dn5, locals.var_rd23e_dn6, locals.var_rd23e_dn7, locals.var_rd23e_dn8, locals.var_rd23e_dn9, locals.var_rd23e_dn10, locals.var_rd23e_dn13,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign36680_e42213;
        locals.var_t7_dn0 = assign36680_e42213_d_n0;
        locals.var_t7_dn2 = assign36680_e42213_d_n2;
        locals.var_t7_dn4 = assign36680_e42213_d_n4;
        locals.var_t7_dn5 = assign36680_e42213_d_n5;
        locals.var_t7_dn6 = assign36680_e42213_d_n6;
        locals.var_t7_dn7 = assign36680_e42213_d_n7;
        locals.var_t7_dn8 = assign36680_e42213_d_n8;
        locals.var_t7_dn9 = assign36680_e42213_d_n9;
        locals.var_t7_dn10 = assign36680_e42213_d_n10;
        locals.var_t7_dn13 = assign36680_e42213_d_n13;
        locals.var_t7_rv = 0.0;

        let assign36690_e42216: f64 = if locals.var_vdse >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard827 = assign36690_e42216;
        locals.var_guard827_rv = 0.0;

        let (assign36700_e42226, assign36700_e42226_d_n0, assign36700_e42226_d_n2,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard825 != 0.0)) && (locals.var_guard827 != 0.0)) {
        (locals.var_vdse, locals.var_vdse_dn0, locals.var_vdse_dn2,)
    } else {
        (locals.var_vdse_eff, locals.var_vdse_eff_dn0, locals.var_vdse_eff_dn2,)
    }
};
        locals.var_vdse_eff = assign36700_e42226;
        locals.var_vdse_eff_dn0 = assign36700_e42226_d_n0;
        locals.var_vdse_eff_dn2 = assign36700_e42226_d_n2;
        locals.var_vdse_eff_rv = 0.0;

        let (assign36710_e42237, assign36710_e42237_d_n0, assign36710_e42237_d_n2,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard825 != 0.0)) && (locals.var_guard827 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_vdse_eff, locals.var_vdse_eff_dn0, locals.var_vdse_eff_dn2,)
    }
};
        locals.var_vdse_eff = assign36710_e42237;
        locals.var_vdse_eff_dn0 = assign36710_e42237_d_n0;
        locals.var_vdse_eff_dn2 = assign36710_e42237_d_n2;
        locals.var_vdse_eff_rv = 0.0;

        let assign36720_e42241: f64 = (20.0 * 1e-12);
        let assign36720_e42242: f64 = if locals.var_vdse_eff < assign36720_e42241 { 1.0 } else { 0.0 };
        locals.var_guard828 = assign36720_e42242;
        locals.var_guard828_rv = 0.0;

        let (assign36730_e42272,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard825 != 0.0)) && (locals.var_guard828 != 0.0)) {
        let assign36730_e42252: f64 = (20.0 + 1.0);
        let assign36730_e42255: f64 = (p.p297 - 1.0);
        let assign36730_e42256: f64 = (assign36730_e42252).powf(assign36730_e42255);
        let assign36730_e42259: f64 = (20.0 + 1.0);
        let assign36730_e42262: f64 = (0.5 * p.p297);
        let assign36730_e42264: f64 = (assign36730_e42262 * 20.0);
        let assign36730_e42265: f64 = (assign36730_e42259 - assign36730_e42264);
        let assign36730_e42266: f64 = (assign36730_e42256 * assign36730_e42265);
        let assign36730_e42269: f64 = (1e-12_f64).powf(p.p297);
        let assign36730_e42270: f64 = (assign36730_e42266 * assign36730_e42269);
        (assign36730_e42270,)
    } else {
        (locals.var_ra_alpha,)
    }
};
        locals.var_ra_alpha = assign36730_e42272;
        locals.var_ra_alpha_rv = 0.0;

        let (assign36740_e42300,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard825 != 0.0)) && (locals.var_guard828 != 0.0)) {
        let assign36740_e42282: f64 = (0.5 * p.p297);
        let assign36740_e42285: f64 = (20.0 + 1.0);
        let assign36740_e42288: f64 = (p.p297 - 1.0);
        let assign36740_e42289: f64 = (assign36740_e42285).powf(assign36740_e42288);
        let assign36740_e42290: f64 = (assign36740_e42282 * assign36740_e42289);
        let assign36740_e42292: f64 = (assign36740_e42290 / 20.0);
        let assign36740_e42296: f64 = (p.p297 - 2.0);
        let assign36740_e42297: f64 = (1e-12_f64).powf(assign36740_e42296);
        let assign36740_e42298: f64 = (assign36740_e42292 * assign36740_e42297);
        (assign36740_e42298,)
    } else {
        (locals.var_ra_beta,)
    }
};
        locals.var_ra_beta = assign36740_e42300;
        locals.var_ra_beta_rv = 0.0;

        let (assign36750_e42316, assign36750_e42316_d_n0, assign36750_e42316_d_n2, assign36750_e42316_d_n4, assign36750_e42316_d_n5, assign36750_e42316_d_n6, assign36750_e42316_d_n7, assign36750_e42316_d_n8, assign36750_e42316_d_n9, assign36750_e42316_d_n10, assign36750_e42316_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard825 != 0.0)) && (locals.var_guard828 != 0.0)) {
        let assign36750_e42311: f64 = (locals.var_ra_beta * locals.var_vdse_eff);
        let assign36750_e42313: f64 = (assign36750_e42311 * locals.var_vdse_eff);
        let assign36750_e42314: f64 = (locals.var_ra_alpha + assign36750_e42313);
        (assign36750_e42314, (((locals.var_ra_beta * locals.var_vdse_eff_dn0) * locals.var_vdse_eff) + (assign36750_e42311 * locals.var_vdse_eff_dn0)), (((locals.var_ra_beta * locals.var_vdse_eff_dn2) * locals.var_vdse_eff) + (assign36750_e42311 * locals.var_vdse_eff_dn2)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign36750_e42316;
        locals.var_t1_dn0 = assign36750_e42316_d_n0;
        locals.var_t1_dn2 = assign36750_e42316_d_n2;
        locals.var_t1_dn4 = assign36750_e42316_d_n4;
        locals.var_t1_dn5 = assign36750_e42316_d_n5;
        locals.var_t1_dn6 = assign36750_e42316_d_n6;
        locals.var_t1_dn7 = assign36750_e42316_d_n7;
        locals.var_t1_dn8 = assign36750_e42316_d_n8;
        locals.var_t1_dn9 = assign36750_e42316_d_n9;
        locals.var_t1_dn10 = assign36750_e42316_d_n10;
        locals.var_t1_dn13 = assign36750_e42316_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign36760_e42331, assign36760_e42331_d_n0, assign36760_e42331_d_n2, assign36760_e42331_d_n4, assign36760_e42331_d_n5, assign36760_e42331_d_n6, assign36760_e42331_d_n7, assign36760_e42331_d_n8, assign36760_e42331_d_n9, assign36760_e42331_d_n10, assign36760_e42331_d_n13,) = {
    if ((((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard825 != 0.0)) && (locals.var_guard828 == 0.0)) {
        let assign36760_e42327: f64 = (locals.var_vdse_eff + 1e-12);
        let assign36760_e42329: f64 = (assign36760_e42327).powf(p.p297);
        (assign36760_e42329, if 0.0 == 0.0 && ((p.p297) as f64).is_finite() && ((p.p297) as f64).fract() == 0.0 { if p.p297 == 0.0 { 0.0 } else { (p.p297 * ((assign36760_e42327).powf(p.p297 - 1.0) * locals.var_vdse_eff_dn0)) } } else { (assign36760_e42329 * (p.p297 * (locals.var_vdse_eff_dn0 / assign36760_e42327))) }, if 0.0 == 0.0 && ((p.p297) as f64).is_finite() && ((p.p297) as f64).fract() == 0.0 { if p.p297 == 0.0 { 0.0 } else { (p.p297 * ((assign36760_e42327).powf(p.p297 - 1.0) * locals.var_vdse_eff_dn2)) } } else { (assign36760_e42329 * (p.p297 * (locals.var_vdse_eff_dn2 / assign36760_e42327))) }, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign36760_e42331;
        locals.var_t1_dn0 = assign36760_e42331_d_n0;
        locals.var_t1_dn2 = assign36760_e42331_d_n2;
        locals.var_t1_dn4 = assign36760_e42331_d_n4;
        locals.var_t1_dn5 = assign36760_e42331_d_n5;
        locals.var_t1_dn6 = assign36760_e42331_d_n6;
        locals.var_t1_dn7 = assign36760_e42331_d_n7;
        locals.var_t1_dn8 = assign36760_e42331_d_n8;
        locals.var_t1_dn9 = assign36760_e42331_d_n9;
        locals.var_t1_dn10 = assign36760_e42331_d_n10;
        locals.var_t1_dn13 = assign36760_e42331_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign36770_e42343, assign36770_e42343_d_n0, assign36770_e42343_d_n2, assign36770_e42343_d_n4, assign36770_e42343_d_n5, assign36770_e42343_d_n6, assign36770_e42343_d_n7, assign36770_e42343_d_n8, assign36770_e42343_d_n9, assign36770_e42343_d_n10, assign36770_e42343_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard825 != 0.0)) {
        let assign36770_e42339: f64 = (locals.var_vdse_eff + 1e-12);
        let assign36770_e42341: f64 = (assign36770_e42339).powf(p.p299);
        (assign36770_e42341, if 0.0 == 0.0 && ((p.p299) as f64).is_finite() && ((p.p299) as f64).fract() == 0.0 { if p.p299 == 0.0 { 0.0 } else { (p.p299 * ((assign36770_e42339).powf(p.p299 - 1.0) * locals.var_vdse_eff_dn0)) } } else { (assign36770_e42341 * (p.p299 * (locals.var_vdse_eff_dn0 / assign36770_e42339))) }, if 0.0 == 0.0 && ((p.p299) as f64).is_finite() && ((p.p299) as f64).fract() == 0.0 { if p.p299 == 0.0 { 0.0 } else { (p.p299 * ((assign36770_e42339).powf(p.p299 - 1.0) * locals.var_vdse_eff_dn2)) } } else { (assign36770_e42341 * (p.p299 * (locals.var_vdse_eff_dn2 / assign36770_e42339))) }, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn13,)
    }
};
        locals.var_t9 = assign36770_e42343;
        locals.var_t9_dn0 = assign36770_e42343_d_n0;
        locals.var_t9_dn2 = assign36770_e42343_d_n2;
        locals.var_t9_dn4 = assign36770_e42343_d_n4;
        locals.var_t9_dn5 = assign36770_e42343_d_n5;
        locals.var_t9_dn6 = assign36770_e42343_d_n6;
        locals.var_t9_dn7 = assign36770_e42343_d_n7;
        locals.var_t9_dn8 = assign36770_e42343_d_n8;
        locals.var_t9_dn9 = assign36770_e42343_d_n9;
        locals.var_t9_dn10 = assign36770_e42343_d_n10;
        locals.var_t9_dn13 = assign36770_e42343_d_n13;
        locals.var_t9_rv = 0.0;

        let (assign36780_e42361, assign36780_e42361_d_n0, assign36780_e42361_d_n2, assign36780_e42361_d_n4, assign36780_e42361_d_n5, assign36780_e42361_d_n6, assign36780_e42361_d_n7, assign36780_e42361_d_n8, assign36780_e42361_d_n9, assign36780_e42361_d_n10, assign36780_e42361_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard825 != 0.0)) {
        let assign36780_e42351: f64 = (locals.var_t7 * locals.var_t1);
        let assign36780_e42354: f64 = (locals.var_vbse * locals.var_uc_rd22);
        let assign36780_e42356: f64 = (assign36780_e42354 * locals.var_t9);
        let assign36780_e42357: f64 = (assign36780_e42351 + assign36780_e42356);
        let assign36780_e42359: f64 = (assign36780_e42357 / locals.var_weff_nf);
        (assign36780_e42359, ((((locals.var_t7_dn0 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn0)) + (((locals.var_vbse_dn0 * locals.var_uc_rd22) * locals.var_t9) + (assign36780_e42354 * locals.var_t9_dn0))) / locals.var_weff_nf), ((((locals.var_t7_dn2 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn2)) + (((locals.var_vbse_dn2 * locals.var_uc_rd22) * locals.var_t9) + (assign36780_e42354 * locals.var_t9_dn2))) / locals.var_weff_nf), ((((locals.var_t7_dn4 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn4)) + (assign36780_e42354 * locals.var_t9_dn4)) / locals.var_weff_nf), ((((locals.var_t7_dn5 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn5)) + (assign36780_e42354 * locals.var_t9_dn5)) / locals.var_weff_nf), ((((locals.var_t7_dn6 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn6)) + (assign36780_e42354 * locals.var_t9_dn6)) / locals.var_weff_nf), ((((locals.var_t7_dn7 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn7)) + (assign36780_e42354 * locals.var_t9_dn7)) / locals.var_weff_nf), ((((locals.var_t7_dn8 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn8)) + (((locals.var_vbse_dn8 * locals.var_uc_rd22) * locals.var_t9) + (assign36780_e42354 * locals.var_t9_dn8))) / locals.var_weff_nf), ((((locals.var_t7_dn9 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn9)) + (assign36780_e42354 * locals.var_t9_dn9)) / locals.var_weff_nf), ((((locals.var_t7_dn10 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn10)) + (assign36780_e42354 * locals.var_t9_dn10)) / locals.var_weff_nf), ((((locals.var_t7_dn13 * locals.var_t1) + (locals.var_t7 * locals.var_t1_dn13)) + (assign36780_e42354 * locals.var_t9_dn13)) / locals.var_weff_nf),)
    } else {
        (locals.var_ra, locals.var_ra_dn0, locals.var_ra_dn2, locals.var_ra_dn4, locals.var_ra_dn5, locals.var_ra_dn6, locals.var_ra_dn7, locals.var_ra_dn8, locals.var_ra_dn9, locals.var_ra_dn10, locals.var_ra_dn13,)
    }
};
        locals.var_ra = assign36780_e42361;
        locals.var_ra_dn0 = assign36780_e42361_d_n0;
        locals.var_ra_dn2 = assign36780_e42361_d_n2;
        locals.var_ra_dn4 = assign36780_e42361_d_n4;
        locals.var_ra_dn5 = assign36780_e42361_d_n5;
        locals.var_ra_dn6 = assign36780_e42361_d_n6;
        locals.var_ra_dn7 = assign36780_e42361_d_n7;
        locals.var_ra_dn8 = assign36780_e42361_d_n8;
        locals.var_ra_dn9 = assign36780_e42361_d_n9;
        locals.var_ra_dn10 = assign36780_e42361_d_n10;
        locals.var_ra_dn13 = assign36780_e42361_d_n13;
        locals.var_ra_rv = 0.0;

        let (assign36790_e42371, assign36790_e42371_d_n0, assign36790_e42371_d_n2, assign36790_e42371_d_n4, assign36790_e42371_d_n5, assign36790_e42371_d_n6, assign36790_e42371_d_n7, assign36790_e42371_d_n8, assign36790_e42371_d_n9, assign36790_e42371_d_n10, assign36790_e42371_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard825 != 0.0)) {
        let assign36790_e42369: f64 = (locals.var_ra * locals.var_ids0);
        (assign36790_e42369, ((locals.var_ra_dn0 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn0)), ((locals.var_ra_dn2 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn2)), ((locals.var_ra_dn4 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn4)), ((locals.var_ra_dn5 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn5)), ((locals.var_ra_dn6 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn6)), ((locals.var_ra_dn7 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn7)), ((locals.var_ra_dn8 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn8)), ((locals.var_ra_dn9 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn9)), ((locals.var_ra_dn10 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn10)), ((locals.var_ra_dn13 * locals.var_ids0) + (locals.var_ra * locals.var_ids0_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign36790_e42371;
        locals.var_t0_dn0 = assign36790_e42371_d_n0;
        locals.var_t0_dn2 = assign36790_e42371_d_n2;
        locals.var_t0_dn4 = assign36790_e42371_d_n4;
        locals.var_t0_dn5 = assign36790_e42371_d_n5;
        locals.var_t0_dn6 = assign36790_e42371_d_n6;
        locals.var_t0_dn7 = assign36790_e42371_d_n7;
        locals.var_t0_dn8 = assign36790_e42371_d_n8;
        locals.var_t0_dn9 = assign36790_e42371_d_n9;
        locals.var_t0_dn10 = assign36790_e42371_d_n10;
        locals.var_t0_dn13 = assign36790_e42371_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign36800_e42381, assign36800_e42381_d_n0, assign36800_e42381_d_n2, assign36800_e42381_d_n4, assign36800_e42381_d_n5, assign36800_e42381_d_n6, assign36800_e42381_d_n7, assign36800_e42381_d_n8, assign36800_e42381_d_n9, assign36800_e42381_d_n10, assign36800_e42381_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard825 != 0.0)) {
        let assign36800_e42379: f64 = (locals.var_vds + 1e-12);
        (assign36800_e42379, locals.var_vds_dn0, locals.var_vds_dn2, locals.var_vds_dn4, locals.var_vds_dn5, locals.var_vds_dn6, locals.var_vds_dn7, locals.var_vds_dn8, locals.var_vds_dn9, locals.var_vds_dn10, locals.var_vds_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign36800_e42381;
        locals.var_t1_dn0 = assign36800_e42381_d_n0;
        locals.var_t1_dn2 = assign36800_e42381_d_n2;
        locals.var_t1_dn4 = assign36800_e42381_d_n4;
        locals.var_t1_dn5 = assign36800_e42381_d_n5;
        locals.var_t1_dn6 = assign36800_e42381_d_n6;
        locals.var_t1_dn7 = assign36800_e42381_d_n7;
        locals.var_t1_dn8 = assign36800_e42381_d_n8;
        locals.var_t1_dn9 = assign36800_e42381_d_n9;
        locals.var_t1_dn10 = assign36800_e42381_d_n10;
        locals.var_t1_dn13 = assign36800_e42381_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign36810_e42391, assign36810_e42391_d_n0, assign36810_e42391_d_n2, assign36810_e42391_d_n4, assign36810_e42391_d_n5, assign36810_e42391_d_n6, assign36810_e42391_d_n7, assign36810_e42391_d_n8, assign36810_e42391_d_n9, assign36810_e42391_d_n10, assign36810_e42391_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard825 != 0.0)) {
        let assign36810_e42389: f64 = (1.0 / locals.var_t1);
        (assign36810_e42389, (-(locals.var_t1_dn0 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn2 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn4 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn5 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn6 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn7 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn8 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn9 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn10 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn13 / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign36810_e42391;
        locals.var_t2_dn0 = assign36810_e42391_d_n0;
        locals.var_t2_dn2 = assign36810_e42391_d_n2;
        locals.var_t2_dn4 = assign36810_e42391_d_n4;
        locals.var_t2_dn5 = assign36810_e42391_d_n5;
        locals.var_t2_dn6 = assign36810_e42391_d_n6;
        locals.var_t2_dn7 = assign36810_e42391_d_n7;
        locals.var_t2_dn8 = assign36810_e42391_d_n8;
        locals.var_t2_dn9 = assign36810_e42391_d_n9;
        locals.var_t2_dn10 = assign36810_e42391_d_n10;
        locals.var_t2_dn13 = assign36810_e42391_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign36820_e42403, assign36820_e42403_d_n0, assign36820_e42403_d_n2, assign36820_e42403_d_n4, assign36820_e42403_d_n5, assign36820_e42403_d_n6, assign36820_e42403_d_n7, assign36820_e42403_d_n8, assign36820_e42403_d_n9, assign36820_e42403_d_n10, assign36820_e42403_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard825 != 0.0)) {
        let assign36820_e42400: f64 = (locals.var_t0 * locals.var_t2);
        let assign36820_e42401: f64 = (1.0 + assign36820_e42400);
        (assign36820_e42401, ((locals.var_t0_dn0 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn0)), ((locals.var_t0_dn2 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn2)), ((locals.var_t0_dn4 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn4)), ((locals.var_t0_dn5 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn5)), ((locals.var_t0_dn6 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn6)), ((locals.var_t0_dn7 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn7)), ((locals.var_t0_dn8 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn8)), ((locals.var_t0_dn9 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn9)), ((locals.var_t0_dn10 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn10)), ((locals.var_t0_dn13 * locals.var_t2) + (locals.var_t0 * locals.var_t2_dn13)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign36820_e42403;
        locals.var_t3_dn0 = assign36820_e42403_d_n0;
        locals.var_t3_dn2 = assign36820_e42403_d_n2;
        locals.var_t3_dn4 = assign36820_e42403_d_n4;
        locals.var_t3_dn5 = assign36820_e42403_d_n5;
        locals.var_t3_dn6 = assign36820_e42403_d_n6;
        locals.var_t3_dn7 = assign36820_e42403_d_n7;
        locals.var_t3_dn8 = assign36820_e42403_d_n8;
        locals.var_t3_dn9 = assign36820_e42403_d_n9;
        locals.var_t3_dn10 = assign36820_e42403_d_n10;
        locals.var_t3_dn13 = assign36820_e42403_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign36830_e42413, assign36830_e42413_d_n0, assign36830_e42413_d_n2, assign36830_e42413_d_n4, assign36830_e42413_d_n5, assign36830_e42413_d_n6, assign36830_e42413_d_n7, assign36830_e42413_d_n8, assign36830_e42413_d_n9, assign36830_e42413_d_n10, assign36830_e42413_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard825 != 0.0)) {
        let assign36830_e42411: f64 = (1.0 / locals.var_t3);
        (assign36830_e42411, (-(locals.var_t3_dn0 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn2 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))), (-(locals.var_t3_dn13 / (locals.var_t3 * locals.var_t3))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign36830_e42413;
        locals.var_t4_dn0 = assign36830_e42413_d_n0;
        locals.var_t4_dn2 = assign36830_e42413_d_n2;
        locals.var_t4_dn4 = assign36830_e42413_d_n4;
        locals.var_t4_dn5 = assign36830_e42413_d_n5;
        locals.var_t4_dn6 = assign36830_e42413_d_n6;
        locals.var_t4_dn7 = assign36830_e42413_d_n7;
        locals.var_t4_dn8 = assign36830_e42413_d_n8;
        locals.var_t4_dn9 = assign36830_e42413_d_n9;
        locals.var_t4_dn10 = assign36830_e42413_d_n10;
        locals.var_t4_dn13 = assign36830_e42413_d_n13;
        locals.var_t4_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_117(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign36840_e42423, assign36840_e42423_d_n0, assign36840_e42423_d_n2, assign36840_e42423_d_n4, assign36840_e42423_d_n5, assign36840_e42423_d_n6, assign36840_e42423_d_n7, assign36840_e42423_d_n8, assign36840_e42423_d_n9, assign36840_e42423_d_n10, assign36840_e42423_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard825 != 0.0)) {
        let assign36840_e42421: f64 = (locals.var_ids0 * locals.var_t4);
        (assign36840_e42421, ((locals.var_ids0_dn0 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn0)), ((locals.var_ids0_dn2 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn2)), ((locals.var_ids0_dn4 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn4)), ((locals.var_ids0_dn5 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn5)), ((locals.var_ids0_dn6 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn6)), ((locals.var_ids0_dn7 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn7)), ((locals.var_ids0_dn8 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn8)), ((locals.var_ids0_dn9 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn9)), ((locals.var_ids0_dn10 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn10)), ((locals.var_ids0_dn13 * locals.var_t4) + (locals.var_ids0 * locals.var_t4_dn13)),)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn13,)
    }
};
        locals.var_ids = assign36840_e42423;
        locals.var_ids_dn0 = assign36840_e42423_d_n0;
        locals.var_ids_dn2 = assign36840_e42423_d_n2;
        locals.var_ids_dn4 = assign36840_e42423_d_n4;
        locals.var_ids_dn5 = assign36840_e42423_d_n5;
        locals.var_ids_dn6 = assign36840_e42423_d_n6;
        locals.var_ids_dn7 = assign36840_e42423_d_n7;
        locals.var_ids_dn8 = assign36840_e42423_d_n8;
        locals.var_ids_dn9 = assign36840_e42423_d_n9;
        locals.var_ids_dn10 = assign36840_e42423_d_n10;
        locals.var_ids_dn13 = assign36840_e42423_d_n13;
        locals.var_ids_rv = 0.0;

        let (assign36850_e42432, assign36850_e42432_d_n0, assign36850_e42432_d_n2, assign36850_e42432_d_n4, assign36850_e42432_d_n5, assign36850_e42432_d_n6, assign36850_e42432_d_n7, assign36850_e42432_d_n8, assign36850_e42432_d_n9, assign36850_e42432_d_n10, assign36850_e42432_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard825 == 0.0)) {
        (locals.var_ids0, locals.var_ids0_dn0, locals.var_ids0_dn2, locals.var_ids0_dn4, locals.var_ids0_dn5, locals.var_ids0_dn6, locals.var_ids0_dn7, locals.var_ids0_dn8, locals.var_ids0_dn9, locals.var_ids0_dn10, locals.var_ids0_dn13,)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn13,)
    }
};
        locals.var_ids = assign36850_e42432;
        locals.var_ids_dn0 = assign36850_e42432_d_n0;
        locals.var_ids_dn2 = assign36850_e42432_d_n2;
        locals.var_ids_dn4 = assign36850_e42432_d_n4;
        locals.var_ids_dn5 = assign36850_e42432_d_n5;
        locals.var_ids_dn6 = assign36850_e42432_d_n6;
        locals.var_ids_dn7 = assign36850_e42432_d_n7;
        locals.var_ids_dn8 = assign36850_e42432_d_n8;
        locals.var_ids_dn9 = assign36850_e42432_d_n9;
        locals.var_ids_dn10 = assign36850_e42432_d_n10;
        locals.var_ids_dn13 = assign36850_e42432_d_n13;
        locals.var_ids_rv = 0.0;

        let (assign36860_e42441, assign36860_e42441_d_n0, assign36860_e42441_d_n2, assign36860_e42441_d_n4, assign36860_e42441_d_n5, assign36860_e42441_d_n6, assign36860_e42441_d_n7, assign36860_e42441_d_n8, assign36860_e42441_d_n9, assign36860_e42441_d_n10, assign36860_e42441_d_n13,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard825 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ra, locals.var_ra_dn0, locals.var_ra_dn2, locals.var_ra_dn4, locals.var_ra_dn5, locals.var_ra_dn6, locals.var_ra_dn7, locals.var_ra_dn8, locals.var_ra_dn9, locals.var_ra_dn10, locals.var_ra_dn13,)
    }
};
        locals.var_ra = assign36860_e42441;
        locals.var_ra_dn0 = assign36860_e42441_d_n0;
        locals.var_ra_dn2 = assign36860_e42441_d_n2;
        locals.var_ra_dn4 = assign36860_e42441_d_n4;
        locals.var_ra_dn5 = assign36860_e42441_d_n5;
        locals.var_ra_dn6 = assign36860_e42441_d_n6;
        locals.var_ra_dn7 = assign36860_e42441_d_n7;
        locals.var_ra_dn8 = assign36860_e42441_d_n8;
        locals.var_ra_dn9 = assign36860_e42441_d_n9;
        locals.var_ra_dn10 = assign36860_e42441_d_n10;
        locals.var_ra_dn13 = assign36860_e42441_d_n13;
        locals.var_ra_rv = 0.0;

        let (assign36870_e42456, assign36870_e42456_d_n0, assign36870_e42456_d_n2, assign36870_e42456_d_n4, assign36870_e42456_d_n5, assign36870_e42456_d_n6, assign36870_e42456_d_n7, assign36870_e42456_d_n8, assign36870_e42456_d_n9, assign36870_e42456_d_n10, assign36870_e42456_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign36870_e42446: f64 = (-0.5);
        let assign36870_e42449: f64 = (locals.var_q_sub0 + locals.var_q_subl);
        let assign36870_e42451: f64 = (assign36870_e42449 + locals.var_q_sub0_dep);
        let assign36870_e42453: f64 = (assign36870_e42451 + locals.var_q_subl_dep);
        let assign36870_e42454: f64 = (assign36870_e42446 * assign36870_e42453);
        (assign36870_e42454, (assign36870_e42446 * (((locals.var_q_sub0_dn0 + locals.var_q_subl_dn0) + locals.var_q_sub0_dep_dn0) + locals.var_q_subl_dep_dn0)), (assign36870_e42446 * (((locals.var_q_sub0_dn2 + locals.var_q_subl_dn2) + locals.var_q_sub0_dep_dn2) + locals.var_q_subl_dep_dn2)), (assign36870_e42446 * (((locals.var_q_sub0_dn4 + locals.var_q_subl_dn4) + locals.var_q_sub0_dep_dn4) + locals.var_q_subl_dep_dn4)), (assign36870_e42446 * (((locals.var_q_sub0_dn5 + locals.var_q_subl_dn5) + locals.var_q_sub0_dep_dn5) + locals.var_q_subl_dep_dn5)), (assign36870_e42446 * (((locals.var_q_sub0_dn6 + locals.var_q_subl_dn6) + locals.var_q_sub0_dep_dn6) + locals.var_q_subl_dep_dn6)), (assign36870_e42446 * (((locals.var_q_sub0_dn7 + locals.var_q_subl_dn7) + locals.var_q_sub0_dep_dn7) + locals.var_q_subl_dep_dn7)), (assign36870_e42446 * (((locals.var_q_sub0_dn8 + locals.var_q_subl_dn8) + locals.var_q_sub0_dep_dn8) + locals.var_q_subl_dep_dn8)), (assign36870_e42446 * (((locals.var_q_sub0_dn9 + locals.var_q_subl_dn9) + locals.var_q_sub0_dep_dn9) + locals.var_q_subl_dep_dn9)), (assign36870_e42446 * (((locals.var_q_sub0_dn10 + locals.var_q_subl_dn10) + locals.var_q_sub0_dep_dn10) + locals.var_q_subl_dep_dn10)), (assign36870_e42446 * (((locals.var_q_sub0_dn13 + locals.var_q_subl_dn13) + locals.var_q_sub0_dep_dn13) + locals.var_q_subl_dep_dn13)),)
    } else {
        (locals.var_qbu, locals.var_qbu_dn0, locals.var_qbu_dn2, locals.var_qbu_dn4, locals.var_qbu_dn5, locals.var_qbu_dn6, locals.var_qbu_dn7, locals.var_qbu_dn8, locals.var_qbu_dn9, locals.var_qbu_dn10, locals.var_qbu_dn13,)
    }
};
        locals.var_qbu = assign36870_e42456;
        locals.var_qbu_dn0 = assign36870_e42456_d_n0;
        locals.var_qbu_dn2 = assign36870_e42456_d_n2;
        locals.var_qbu_dn4 = assign36870_e42456_d_n4;
        locals.var_qbu_dn5 = assign36870_e42456_d_n5;
        locals.var_qbu_dn6 = assign36870_e42456_d_n6;
        locals.var_qbu_dn7 = assign36870_e42456_d_n7;
        locals.var_qbu_dn8 = assign36870_e42456_d_n8;
        locals.var_qbu_dn9 = assign36870_e42456_d_n9;
        locals.var_qbu_dn10 = assign36870_e42456_d_n10;
        locals.var_qbu_dn13 = assign36870_e42456_d_n13;
        locals.var_qbu_rv = 0.0;

        let (assign36880_e42475, assign36880_e42475_d_n0, assign36880_e42475_d_n2, assign36880_e42475_d_n4, assign36880_e42475_d_n5, assign36880_e42475_d_n6, assign36880_e42475_d_n7, assign36880_e42475_d_n8, assign36880_e42475_d_n9, assign36880_e42475_d_n10, assign36880_e42475_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign36880_e42461: f64 = (-0.5);
        let assign36880_e42464: f64 = (locals.var_q_n0__blk538 + locals.var_q_nl);
        let assign36880_e42466: f64 = (assign36880_e42464 + locals.var_q_s0_dep);
        let assign36880_e42468: f64 = (assign36880_e42466 + locals.var_q_sl_dep);
        let assign36880_e42470: f64 = (assign36880_e42468 + locals.var_q_b0_dep);
        let assign36880_e42472: f64 = (assign36880_e42470 + locals.var_q_bl_dep);
        let assign36880_e42473: f64 = (assign36880_e42461 * assign36880_e42472);
        (assign36880_e42473, (assign36880_e42461 * (((((locals.var_q_n0__blk538_dn0 + locals.var_q_nl_dn0) + locals.var_q_s0_dep_dn0) + locals.var_q_sl_dep_dn0) + locals.var_q_b0_dep_dn0) + locals.var_q_bl_dep_dn0)), (assign36880_e42461 * (((((locals.var_q_n0__blk538_dn2 + locals.var_q_nl_dn2) + locals.var_q_s0_dep_dn2) + locals.var_q_sl_dep_dn2) + locals.var_q_b0_dep_dn2) + locals.var_q_bl_dep_dn2)), (assign36880_e42461 * (((((locals.var_q_n0__blk538_dn4 + locals.var_q_nl_dn4) + locals.var_q_s0_dep_dn4) + locals.var_q_sl_dep_dn4) + locals.var_q_b0_dep_dn4) + locals.var_q_bl_dep_dn4)), (assign36880_e42461 * (((((locals.var_q_n0__blk538_dn5 + locals.var_q_nl_dn5) + locals.var_q_s0_dep_dn5) + locals.var_q_sl_dep_dn5) + locals.var_q_b0_dep_dn5) + locals.var_q_bl_dep_dn5)), (assign36880_e42461 * (((((locals.var_q_n0__blk538_dn6 + locals.var_q_nl_dn6) + locals.var_q_s0_dep_dn6) + locals.var_q_sl_dep_dn6) + locals.var_q_b0_dep_dn6) + locals.var_q_bl_dep_dn6)), (assign36880_e42461 * (((((locals.var_q_n0__blk538_dn7 + locals.var_q_nl_dn7) + locals.var_q_s0_dep_dn7) + locals.var_q_sl_dep_dn7) + locals.var_q_b0_dep_dn7) + locals.var_q_bl_dep_dn7)), (assign36880_e42461 * (((((locals.var_q_n0__blk538_dn8 + locals.var_q_nl_dn8) + locals.var_q_s0_dep_dn8) + locals.var_q_sl_dep_dn8) + locals.var_q_b0_dep_dn8) + locals.var_q_bl_dep_dn8)), (assign36880_e42461 * (((((locals.var_q_n0__blk538_dn9 + locals.var_q_nl_dn9) + locals.var_q_s0_dep_dn9) + locals.var_q_sl_dep_dn9) + locals.var_q_b0_dep_dn9) + locals.var_q_bl_dep_dn9)), (assign36880_e42461 * (((((locals.var_q_n0__blk538_dn10 + locals.var_q_nl_dn10) + locals.var_q_s0_dep_dn10) + locals.var_q_sl_dep_dn10) + locals.var_q_b0_dep_dn10) + locals.var_q_bl_dep_dn10)), (assign36880_e42461 * (((((locals.var_q_n0__blk538_dn13 + locals.var_q_nl_dn13) + locals.var_q_s0_dep_dn13) + locals.var_q_sl_dep_dn13) + locals.var_q_b0_dep_dn13) + locals.var_q_bl_dep_dn13)),)
    } else {
        (locals.var_qiu, locals.var_qiu_dn0, locals.var_qiu_dn2, locals.var_qiu_dn4, locals.var_qiu_dn5, locals.var_qiu_dn6, locals.var_qiu_dn7, locals.var_qiu_dn8, locals.var_qiu_dn9, locals.var_qiu_dn10, locals.var_qiu_dn13,)
    }
};
        locals.var_qiu = assign36880_e42475;
        locals.var_qiu_dn0 = assign36880_e42475_d_n0;
        locals.var_qiu_dn2 = assign36880_e42475_d_n2;
        locals.var_qiu_dn4 = assign36880_e42475_d_n4;
        locals.var_qiu_dn5 = assign36880_e42475_d_n5;
        locals.var_qiu_dn6 = assign36880_e42475_d_n6;
        locals.var_qiu_dn7 = assign36880_e42475_d_n7;
        locals.var_qiu_dn8 = assign36880_e42475_d_n8;
        locals.var_qiu_dn9 = assign36880_e42475_d_n9;
        locals.var_qiu_dn10 = assign36880_e42475_d_n10;
        locals.var_qiu_dn13 = assign36880_e42475_d_n13;
        locals.var_qiu_rv = 0.0;

        let (assign36890_e42481, assign36890_e42481_d_n0, assign36890_e42481_d_n2, assign36890_e42481_d_n4, assign36890_e42481_d_n5, assign36890_e42481_d_n6, assign36890_e42481_d_n7, assign36890_e42481_d_n8, assign36890_e42481_d_n9, assign36890_e42481_d_n10, assign36890_e42481_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        (0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn4, locals.var_qdrat_dn5, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn8, locals.var_qdrat_dn9, locals.var_qdrat_dn10, locals.var_qdrat_dn13,)
    }
};
        locals.var_qdrat = assign36890_e42481;
        locals.var_qdrat_dn0 = assign36890_e42481_d_n0;
        locals.var_qdrat_dn2 = assign36890_e42481_d_n2;
        locals.var_qdrat_dn4 = assign36890_e42481_d_n4;
        locals.var_qdrat_dn5 = assign36890_e42481_d_n5;
        locals.var_qdrat_dn6 = assign36890_e42481_d_n6;
        locals.var_qdrat_dn7 = assign36890_e42481_d_n7;
        locals.var_qdrat_dn8 = assign36890_e42481_d_n8;
        locals.var_qdrat_dn9 = assign36890_e42481_d_n9;
        locals.var_qdrat_dn10 = assign36890_e42481_d_n10;
        locals.var_qdrat_dn13 = assign36890_e42481_d_n13;
        locals.var_qdrat_rv = 0.0;

        let (assign36900_e42492, assign36900_e42492_d_n0, assign36900_e42492_d_n2, assign36900_e42492_d_n4, assign36900_e42492_d_n5, assign36900_e42492_d_n6, assign36900_e42492_d_n7, assign36900_e42492_d_n8, assign36900_e42492_d_n9, assign36900_e42492_d_n10, assign36900_e42492_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign36900_e42486: f64 = (-0.5);
        let assign36900_e42489: f64 = (locals.var_q_n0__blk538 + locals.var_q_nl);
        let assign36900_e42490: f64 = (assign36900_e42486 * assign36900_e42489);
        (assign36900_e42490, (assign36900_e42486 * (locals.var_q_n0__blk538_dn0 + locals.var_q_nl_dn0)), (assign36900_e42486 * (locals.var_q_n0__blk538_dn2 + locals.var_q_nl_dn2)), (assign36900_e42486 * (locals.var_q_n0__blk538_dn4 + locals.var_q_nl_dn4)), (assign36900_e42486 * (locals.var_q_n0__blk538_dn5 + locals.var_q_nl_dn5)), (assign36900_e42486 * (locals.var_q_n0__blk538_dn6 + locals.var_q_nl_dn6)), (assign36900_e42486 * (locals.var_q_n0__blk538_dn7 + locals.var_q_nl_dn7)), (assign36900_e42486 * (locals.var_q_n0__blk538_dn8 + locals.var_q_nl_dn8)), (assign36900_e42486 * (locals.var_q_n0__blk538_dn9 + locals.var_q_nl_dn9)), (assign36900_e42486 * (locals.var_q_n0__blk538_dn10 + locals.var_q_nl_dn10)), (assign36900_e42486 * (locals.var_q_n0__blk538_dn13 + locals.var_q_nl_dn13)),)
    } else {
        (locals.var_qiu_noi, locals.var_qiu_noi_dn0, locals.var_qiu_noi_dn2, locals.var_qiu_noi_dn4, locals.var_qiu_noi_dn5, locals.var_qiu_noi_dn6, locals.var_qiu_noi_dn7, locals.var_qiu_noi_dn8, locals.var_qiu_noi_dn9, locals.var_qiu_noi_dn10, locals.var_qiu_noi_dn13,)
    }
};
        locals.var_qiu_noi = assign36900_e42492;
        locals.var_qiu_noi_dn0 = assign36900_e42492_d_n0;
        locals.var_qiu_noi_dn2 = assign36900_e42492_d_n2;
        locals.var_qiu_noi_dn4 = assign36900_e42492_d_n4;
        locals.var_qiu_noi_dn5 = assign36900_e42492_d_n5;
        locals.var_qiu_noi_dn6 = assign36900_e42492_d_n6;
        locals.var_qiu_noi_dn7 = assign36900_e42492_d_n7;
        locals.var_qiu_noi_dn8 = assign36900_e42492_d_n8;
        locals.var_qiu_noi_dn9 = assign36900_e42492_d_n9;
        locals.var_qiu_noi_dn10 = assign36900_e42492_d_n10;
        locals.var_qiu_noi_dn13 = assign36900_e42492_d_n13;
        locals.var_qiu_noi_rv = 0.0;

        let (assign36910_e42499, assign36910_e42499_d_n0, assign36910_e42499_d_n2, assign36910_e42499_d_n4, assign36910_e42499_d_n5, assign36910_e42499_d_n6, assign36910_e42499_d_n7, assign36910_e42499_d_n8, assign36910_e42499_d_n9, assign36910_e42499_d_n10, assign36910_e42499_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        let assign36910_e42497: f64 = (-locals.var_q_n0__blk538);
        (assign36910_e42497, (-locals.var_q_n0__blk538_dn0), (-locals.var_q_n0__blk538_dn2), (-locals.var_q_n0__blk538_dn4), (-locals.var_q_n0__blk538_dn5), (-locals.var_q_n0__blk538_dn6), (-locals.var_q_n0__blk538_dn7), (-locals.var_q_n0__blk538_dn8), (-locals.var_q_n0__blk538_dn9), (-locals.var_q_n0__blk538_dn10), (-locals.var_q_n0__blk538_dn13),)
    } else {
        (locals.var_qn0, locals.var_qn0_dn0, locals.var_qn0_dn2, locals.var_qn0_dn4, locals.var_qn0_dn5, locals.var_qn0_dn6, locals.var_qn0_dn7, locals.var_qn0_dn8, locals.var_qn0_dn9, locals.var_qn0_dn10, locals.var_qn0_dn13,)
    }
};
        locals.var_qn0 = assign36910_e42499;
        locals.var_qn0_dn0 = assign36910_e42499_d_n0;
        locals.var_qn0_dn2 = assign36910_e42499_d_n2;
        locals.var_qn0_dn4 = assign36910_e42499_d_n4;
        locals.var_qn0_dn5 = assign36910_e42499_d_n5;
        locals.var_qn0_dn6 = assign36910_e42499_d_n6;
        locals.var_qn0_dn7 = assign36910_e42499_d_n7;
        locals.var_qn0_dn8 = assign36910_e42499_d_n8;
        locals.var_qn0_dn9 = assign36910_e42499_d_n9;
        locals.var_qn0_dn10 = assign36910_e42499_d_n10;
        locals.var_qn0_dn13 = assign36910_e42499_d_n13;
        locals.var_qn0_rv = 0.0;

        let (assign36920_e42505, assign36920_e42505_d_n0, assign36920_e42505_d_n2, assign36920_e42505_d_n4, assign36920_e42505_d_n5, assign36920_e42505_d_n6, assign36920_e42505_d_n7, assign36920_e42505_d_n8, assign36920_e42505_d_n9, assign36920_e42505_d_n10, assign36920_e42505_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) {
        (locals.var_ey_suf, locals.var_ey_suf_dn0, locals.var_ey_suf_dn2, locals.var_ey_suf_dn4, locals.var_ey_suf_dn5, locals.var_ey_suf_dn6, locals.var_ey_suf_dn7, locals.var_ey_suf_dn8, locals.var_ey_suf_dn9, locals.var_ey_suf_dn10, locals.var_ey_suf_dn13,)
    } else {
        (locals.var_ey, locals.var_ey_dn0, locals.var_ey_dn2, locals.var_ey_dn4, locals.var_ey_dn5, locals.var_ey_dn6, locals.var_ey_dn7, locals.var_ey_dn8, locals.var_ey_dn9, locals.var_ey_dn10, locals.var_ey_dn13,)
    }
};
        locals.var_ey = assign36920_e42505;
        locals.var_ey_dn0 = assign36920_e42505_d_n0;
        locals.var_ey_dn2 = assign36920_e42505_d_n2;
        locals.var_ey_dn4 = assign36920_e42505_d_n4;
        locals.var_ey_dn5 = assign36920_e42505_d_n5;
        locals.var_ey_dn6 = assign36920_e42505_d_n6;
        locals.var_ey_dn7 = assign36920_e42505_d_n7;
        locals.var_ey_dn8 = assign36920_e42505_d_n8;
        locals.var_ey_dn9 = assign36920_e42505_d_n9;
        locals.var_ey_dn10 = assign36920_e42505_d_n10;
        locals.var_ey_dn13 = assign36920_e42505_d_n13;
        locals.var_ey_rv = 0.0;

        let assign36930_e42512: f64 = if ((locals.var_qn0 < 1e-25) || (locals.var_qiu < 1e-25)) { 1.0 } else { 0.0 };
        locals.var_guard829 = assign36930_e42512;
        locals.var_guard829_rv = 0.0;

        let (assign36940_e42520,) = {
    if (((locals.var_guard443 != 0.0) && (locals.var_guard444 != 0.0)) && (locals.var_guard829 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_noqi,)
    }
};
        locals.var_flg_noqi = assign36940_e42520;
        locals.var_flg_noqi_rv = 0.0;

        let (assign36950_e42529, assign36950_e42529_d_n0, assign36950_e42529_d_n2, assign36950_e42529_d_n4, assign36950_e42529_d_n5, assign36950_e42529_d_n6, assign36950_e42529_d_n7, assign36950_e42529_d_n8, assign36950_e42529_d_n9, assign36950_e42529_d_n10, assign36950_e42529_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        (locals.var_vbipn, locals.var_vbipn_dn0, locals.var_vbipn_dn2, locals.var_vbipn_dn4, locals.var_vbipn_dn5, locals.var_vbipn_dn6, locals.var_vbipn_dn7, locals.var_vbipn_dn8, locals.var_vbipn_dn9, locals.var_vbipn_dn10, locals.var_vbipn_dn13,)
    } else {
        (locals.var_vbi_dep__blk855, locals.var_vbi_dep__blk855_dn0, locals.var_vbi_dep__blk855_dn2, locals.var_vbi_dep__blk855_dn4, locals.var_vbi_dep__blk855_dn5, locals.var_vbi_dep__blk855_dn6, locals.var_vbi_dep__blk855_dn7, locals.var_vbi_dep__blk855_dn8, locals.var_vbi_dep__blk855_dn9, locals.var_vbi_dep__blk855_dn10, locals.var_vbi_dep__blk855_dn13,)
    }
};
        locals.var_vbi_dep__blk855 = assign36950_e42529;
        locals.var_vbi_dep__blk855_dn0 = assign36950_e42529_d_n0;
        locals.var_vbi_dep__blk855_dn2 = assign36950_e42529_d_n2;
        locals.var_vbi_dep__blk855_dn4 = assign36950_e42529_d_n4;
        locals.var_vbi_dep__blk855_dn5 = assign36950_e42529_d_n5;
        locals.var_vbi_dep__blk855_dn6 = assign36950_e42529_d_n6;
        locals.var_vbi_dep__blk855_dn7 = assign36950_e42529_d_n7;
        locals.var_vbi_dep__blk855_dn8 = assign36950_e42529_d_n8;
        locals.var_vbi_dep__blk855_dn9 = assign36950_e42529_d_n9;
        locals.var_vbi_dep__blk855_dn10 = assign36950_e42529_d_n10;
        locals.var_vbi_dep__blk855_dn13 = assign36950_e42529_d_n13;
        locals.var_vbi_dep__blk855_rv = 0.0;

        let (assign36960_e42540, assign36960_e42540_d_n0, assign36960_e42540_d_n2, assign36960_e42540_d_n4, assign36960_e42540_d_n5, assign36960_e42540_d_n6, assign36960_e42540_d_n7, assign36960_e42540_d_n8, assign36960_e42540_d_n9, assign36960_e42540_d_n10, assign36960_e42540_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign36960_e42538: f64 = (1.6021918e-19 * locals.var_uc_ndepm);
        (assign36960_e42538, (1.6021918e-19 * locals.var_uc_ndepm_dn0), (1.6021918e-19 * locals.var_uc_ndepm_dn2), (1.6021918e-19 * locals.var_uc_ndepm_dn4), (1.6021918e-19 * locals.var_uc_ndepm_dn5), (1.6021918e-19 * locals.var_uc_ndepm_dn6), (1.6021918e-19 * locals.var_uc_ndepm_dn7), (1.6021918e-19 * locals.var_uc_ndepm_dn8), (1.6021918e-19 * locals.var_uc_ndepm_dn9), (1.6021918e-19 * locals.var_uc_ndepm_dn10), (1.6021918e-19 * locals.var_uc_ndepm_dn13),)
    } else {
        (locals.var_q_ndepm__blk905, locals.var_q_ndepm__blk905_dn0, locals.var_q_ndepm__blk905_dn2, locals.var_q_ndepm__blk905_dn4, locals.var_q_ndepm__blk905_dn5, locals.var_q_ndepm__blk905_dn6, locals.var_q_ndepm__blk905_dn7, locals.var_q_ndepm__blk905_dn8, locals.var_q_ndepm__blk905_dn9, locals.var_q_ndepm__blk905_dn10, locals.var_q_ndepm__blk905_dn13,)
    }
};
        locals.var_q_ndepm__blk905 = assign36960_e42540;
        locals.var_q_ndepm__blk905_dn0 = assign36960_e42540_d_n0;
        locals.var_q_ndepm__blk905_dn2 = assign36960_e42540_d_n2;
        locals.var_q_ndepm__blk905_dn4 = assign36960_e42540_d_n4;
        locals.var_q_ndepm__blk905_dn5 = assign36960_e42540_d_n5;
        locals.var_q_ndepm__blk905_dn6 = assign36960_e42540_d_n6;
        locals.var_q_ndepm__blk905_dn7 = assign36960_e42540_d_n7;
        locals.var_q_ndepm__blk905_dn8 = assign36960_e42540_d_n8;
        locals.var_q_ndepm__blk905_dn9 = assign36960_e42540_d_n9;
        locals.var_q_ndepm__blk905_dn10 = assign36960_e42540_d_n10;
        locals.var_q_ndepm__blk905_dn13 = assign36960_e42540_d_n13;
        locals.var_q_ndepm__blk905_rv = 0.0;

        let (assign36970_e42553, assign36970_e42553_d_n0, assign36970_e42553_d_n2, assign36970_e42553_d_n4, assign36970_e42553_d_n5, assign36970_e42553_d_n6, assign36970_e42553_d_n7, assign36970_e42553_d_n8, assign36970_e42553_d_n9, assign36970_e42553_d_n10, assign36970_e42553_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign36970_e42549: f64 = (1.6021918e-19 * locals.var_uc_ndepm);
        let assign36970_e42551: f64 = (assign36970_e42549 * 1.034943e-10);
        (assign36970_e42551, ((1.6021918e-19 * locals.var_uc_ndepm_dn0) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn2) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn4) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn5) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn6) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn7) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn8) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn9) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn10) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn13) * 1.034943e-10),)
    } else {
        (locals.var_q_ndepm_esi__blk884, locals.var_q_ndepm_esi__blk884_dn0, locals.var_q_ndepm_esi__blk884_dn2, locals.var_q_ndepm_esi__blk884_dn4, locals.var_q_ndepm_esi__blk884_dn5, locals.var_q_ndepm_esi__blk884_dn6, locals.var_q_ndepm_esi__blk884_dn7, locals.var_q_ndepm_esi__blk884_dn8, locals.var_q_ndepm_esi__blk884_dn9, locals.var_q_ndepm_esi__blk884_dn10, locals.var_q_ndepm_esi__blk884_dn13,)
    }
};
        locals.var_q_ndepm_esi__blk884 = assign36970_e42553;
        locals.var_q_ndepm_esi__blk884_dn0 = assign36970_e42553_d_n0;
        locals.var_q_ndepm_esi__blk884_dn2 = assign36970_e42553_d_n2;
        locals.var_q_ndepm_esi__blk884_dn4 = assign36970_e42553_d_n4;
        locals.var_q_ndepm_esi__blk884_dn5 = assign36970_e42553_d_n5;
        locals.var_q_ndepm_esi__blk884_dn6 = assign36970_e42553_d_n6;
        locals.var_q_ndepm_esi__blk884_dn7 = assign36970_e42553_d_n7;
        locals.var_q_ndepm_esi__blk884_dn8 = assign36970_e42553_d_n8;
        locals.var_q_ndepm_esi__blk884_dn9 = assign36970_e42553_d_n9;
        locals.var_q_ndepm_esi__blk884_dn10 = assign36970_e42553_d_n10;
        locals.var_q_ndepm_esi__blk884_dn13 = assign36970_e42553_d_n13;
        locals.var_q_ndepm_esi__blk884_rv = 0.0;

        let (assign36980_e42564, assign36980_e42564_d_n0, assign36980_e42564_d_n2, assign36980_e42564_d_n4, assign36980_e42564_d_n5, assign36980_e42564_d_n6, assign36980_e42564_d_n7, assign36980_e42564_d_n8, assign36980_e42564_d_n9, assign36980_e42564_d_n10, assign36980_e42564_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign36980_e42562: f64 = (1.6021918e-19 * locals.var_ef_nsubc);
        (assign36980_e42562, (1.6021918e-19 * locals.var_ef_nsubc_dn0), (1.6021918e-19 * locals.var_ef_nsubc_dn2), (1.6021918e-19 * locals.var_ef_nsubc_dn4), (1.6021918e-19 * locals.var_ef_nsubc_dn5), (1.6021918e-19 * locals.var_ef_nsubc_dn6), (1.6021918e-19 * locals.var_ef_nsubc_dn7), (1.6021918e-19 * locals.var_ef_nsubc_dn8), (1.6021918e-19 * locals.var_ef_nsubc_dn9), (1.6021918e-19 * locals.var_ef_nsubc_dn10), (1.6021918e-19 * locals.var_ef_nsubc_dn13),)
    } else {
        (locals.var_q_nsub__blk904, locals.var_q_nsub__blk904_dn0, locals.var_q_nsub__blk904_dn2, locals.var_q_nsub__blk904_dn4, locals.var_q_nsub__blk904_dn5, locals.var_q_nsub__blk904_dn6, locals.var_q_nsub__blk904_dn7, locals.var_q_nsub__blk904_dn8, locals.var_q_nsub__blk904_dn9, locals.var_q_nsub__blk904_dn10, locals.var_q_nsub__blk904_dn13,)
    }
};
        locals.var_q_nsub__blk904 = assign36980_e42564;
        locals.var_q_nsub__blk904_dn0 = assign36980_e42564_d_n0;
        locals.var_q_nsub__blk904_dn2 = assign36980_e42564_d_n2;
        locals.var_q_nsub__blk904_dn4 = assign36980_e42564_d_n4;
        locals.var_q_nsub__blk904_dn5 = assign36980_e42564_d_n5;
        locals.var_q_nsub__blk904_dn6 = assign36980_e42564_d_n6;
        locals.var_q_nsub__blk904_dn7 = assign36980_e42564_d_n7;
        locals.var_q_nsub__blk904_dn8 = assign36980_e42564_d_n8;
        locals.var_q_nsub__blk904_dn9 = assign36980_e42564_d_n9;
        locals.var_q_nsub__blk904_dn10 = assign36980_e42564_d_n10;
        locals.var_q_nsub__blk904_dn13 = assign36980_e42564_d_n13;
        locals.var_q_nsub__blk904_rv = 0.0;

        let (assign36990_e42575, assign36990_e42575_d_n0, assign36990_e42575_d_n2, assign36990_e42575_d_n4, assign36990_e42575_d_n5, assign36990_e42575_d_n6, assign36990_e42575_d_n7, assign36990_e42575_d_n8, assign36990_e42575_d_n9, assign36990_e42575_d_n10, assign36990_e42575_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign36990_e42573: f64 = (locals.var_uc_depthn * locals.var_uc_depthn);
        (assign36990_e42573, ((locals.var_uc_depthn_dn0 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn0)), ((locals.var_uc_depthn_dn2 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn2)), ((locals.var_uc_depthn_dn4 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn4)), ((locals.var_uc_depthn_dn5 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn5)), ((locals.var_uc_depthn_dn6 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn6)), ((locals.var_uc_depthn_dn7 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn7)), ((locals.var_uc_depthn_dn8 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn8)), ((locals.var_uc_depthn_dn9 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn9)), ((locals.var_uc_depthn_dn10 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn10)), ((locals.var_uc_depthn_dn13 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn13)),)
    } else {
        (locals.var_tn2__blk903, locals.var_tn2__blk903_dn0, locals.var_tn2__blk903_dn2, locals.var_tn2__blk903_dn4, locals.var_tn2__blk903_dn5, locals.var_tn2__blk903_dn6, locals.var_tn2__blk903_dn7, locals.var_tn2__blk903_dn8, locals.var_tn2__blk903_dn9, locals.var_tn2__blk903_dn10, locals.var_tn2__blk903_dn13,)
    }
};
        locals.var_tn2__blk903 = assign36990_e42575;
        locals.var_tn2__blk903_dn0 = assign36990_e42575_d_n0;
        locals.var_tn2__blk903_dn2 = assign36990_e42575_d_n2;
        locals.var_tn2__blk903_dn4 = assign36990_e42575_d_n4;
        locals.var_tn2__blk903_dn5 = assign36990_e42575_d_n5;
        locals.var_tn2__blk903_dn6 = assign36990_e42575_d_n6;
        locals.var_tn2__blk903_dn7 = assign36990_e42575_d_n7;
        locals.var_tn2__blk903_dn8 = assign36990_e42575_d_n8;
        locals.var_tn2__blk903_dn9 = assign36990_e42575_d_n9;
        locals.var_tn2__blk903_dn10 = assign36990_e42575_d_n10;
        locals.var_tn2__blk903_dn13 = assign36990_e42575_d_n13;
        locals.var_tn2__blk903_rv = 0.0;

        let (assign37000_e42588, assign37000_e42588_d_n0, assign37000_e42588_d_n2, assign37000_e42588_d_n4, assign37000_e42588_d_n5, assign37000_e42588_d_n6, assign37000_e42588_d_n7, assign37000_e42588_d_n8, assign37000_e42588_d_n9, assign37000_e42588_d_n10, assign37000_e42588_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign37000_e42584: f64 = (2.0 * 1.034943e-10);
        let assign37000_e42586: f64 = (assign37000_e42584 / locals.var_q_ndepm__blk905);
        (assign37000_e42586, (-((assign37000_e42584 * locals.var_q_ndepm__blk905_dn0) / (locals.var_q_ndepm__blk905 * locals.var_q_ndepm__blk905))), (-((assign37000_e42584 * locals.var_q_ndepm__blk905_dn2) / (locals.var_q_ndepm__blk905 * locals.var_q_ndepm__blk905))), (-((assign37000_e42584 * locals.var_q_ndepm__blk905_dn4) / (locals.var_q_ndepm__blk905 * locals.var_q_ndepm__blk905))), (-((assign37000_e42584 * locals.var_q_ndepm__blk905_dn5) / (locals.var_q_ndepm__blk905 * locals.var_q_ndepm__blk905))), (-((assign37000_e42584 * locals.var_q_ndepm__blk905_dn6) / (locals.var_q_ndepm__blk905 * locals.var_q_ndepm__blk905))), (-((assign37000_e42584 * locals.var_q_ndepm__blk905_dn7) / (locals.var_q_ndepm__blk905 * locals.var_q_ndepm__blk905))), (-((assign37000_e42584 * locals.var_q_ndepm__blk905_dn8) / (locals.var_q_ndepm__blk905 * locals.var_q_ndepm__blk905))), (-((assign37000_e42584 * locals.var_q_ndepm__blk905_dn9) / (locals.var_q_ndepm__blk905 * locals.var_q_ndepm__blk905))), (-((assign37000_e42584 * locals.var_q_ndepm__blk905_dn10) / (locals.var_q_ndepm__blk905 * locals.var_q_ndepm__blk905))), (-((assign37000_e42584 * locals.var_q_ndepm__blk905_dn13) / (locals.var_q_ndepm__blk905 * locals.var_q_ndepm__blk905))),)
    } else {
        (locals.var_c_2esipq_ndepm__blk908, locals.var_c_2esipq_ndepm__blk908_dn0, locals.var_c_2esipq_ndepm__blk908_dn2, locals.var_c_2esipq_ndepm__blk908_dn4, locals.var_c_2esipq_ndepm__blk908_dn5, locals.var_c_2esipq_ndepm__blk908_dn6, locals.var_c_2esipq_ndepm__blk908_dn7, locals.var_c_2esipq_ndepm__blk908_dn8, locals.var_c_2esipq_ndepm__blk908_dn9, locals.var_c_2esipq_ndepm__blk908_dn10, locals.var_c_2esipq_ndepm__blk908_dn13,)
    }
};
        locals.var_c_2esipq_ndepm__blk908 = assign37000_e42588;
        locals.var_c_2esipq_ndepm__blk908_dn0 = assign37000_e42588_d_n0;
        locals.var_c_2esipq_ndepm__blk908_dn2 = assign37000_e42588_d_n2;
        locals.var_c_2esipq_ndepm__blk908_dn4 = assign37000_e42588_d_n4;
        locals.var_c_2esipq_ndepm__blk908_dn5 = assign37000_e42588_d_n5;
        locals.var_c_2esipq_ndepm__blk908_dn6 = assign37000_e42588_d_n6;
        locals.var_c_2esipq_ndepm__blk908_dn7 = assign37000_e42588_d_n7;
        locals.var_c_2esipq_ndepm__blk908_dn8 = assign37000_e42588_d_n8;
        locals.var_c_2esipq_ndepm__blk908_dn9 = assign37000_e42588_d_n9;
        locals.var_c_2esipq_ndepm__blk908_dn10 = assign37000_e42588_d_n10;
        locals.var_c_2esipq_ndepm__blk908_dn13 = assign37000_e42588_d_n13;
        locals.var_c_2esipq_ndepm__blk908_rv = 0.0;

        let (assign37010_e42601, assign37010_e42601_d_n0, assign37010_e42601_d_n2, assign37010_e42601_d_n4, assign37010_e42601_d_n5, assign37010_e42601_d_n6, assign37010_e42601_d_n7, assign37010_e42601_d_n8, assign37010_e42601_d_n9, assign37010_e42601_d_n10, assign37010_e42601_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign37010_e42597: f64 = (2.0 * 1.034943e-10);
        let assign37010_e42599: f64 = (assign37010_e42597 / locals.var_q_nsub__blk904);
        (assign37010_e42599, (-((assign37010_e42597 * locals.var_q_nsub__blk904_dn0) / (locals.var_q_nsub__blk904 * locals.var_q_nsub__blk904))), (-((assign37010_e42597 * locals.var_q_nsub__blk904_dn2) / (locals.var_q_nsub__blk904 * locals.var_q_nsub__blk904))), (-((assign37010_e42597 * locals.var_q_nsub__blk904_dn4) / (locals.var_q_nsub__blk904 * locals.var_q_nsub__blk904))), (-((assign37010_e42597 * locals.var_q_nsub__blk904_dn5) / (locals.var_q_nsub__blk904 * locals.var_q_nsub__blk904))), (-((assign37010_e42597 * locals.var_q_nsub__blk904_dn6) / (locals.var_q_nsub__blk904 * locals.var_q_nsub__blk904))), (-((assign37010_e42597 * locals.var_q_nsub__blk904_dn7) / (locals.var_q_nsub__blk904 * locals.var_q_nsub__blk904))), (-((assign37010_e42597 * locals.var_q_nsub__blk904_dn8) / (locals.var_q_nsub__blk904 * locals.var_q_nsub__blk904))), (-((assign37010_e42597 * locals.var_q_nsub__blk904_dn9) / (locals.var_q_nsub__blk904 * locals.var_q_nsub__blk904))), (-((assign37010_e42597 * locals.var_q_nsub__blk904_dn10) / (locals.var_q_nsub__blk904 * locals.var_q_nsub__blk904))), (-((assign37010_e42597 * locals.var_q_nsub__blk904_dn13) / (locals.var_q_nsub__blk904 * locals.var_q_nsub__blk904))),)
    } else {
        (locals.var_c_2esipq_nsub__blk909, locals.var_c_2esipq_nsub__blk909_dn0, locals.var_c_2esipq_nsub__blk909_dn2, locals.var_c_2esipq_nsub__blk909_dn4, locals.var_c_2esipq_nsub__blk909_dn5, locals.var_c_2esipq_nsub__blk909_dn6, locals.var_c_2esipq_nsub__blk909_dn7, locals.var_c_2esipq_nsub__blk909_dn8, locals.var_c_2esipq_nsub__blk909_dn9, locals.var_c_2esipq_nsub__blk909_dn10, locals.var_c_2esipq_nsub__blk909_dn13,)
    }
};
        locals.var_c_2esipq_nsub__blk909 = assign37010_e42601;
        locals.var_c_2esipq_nsub__blk909_dn0 = assign37010_e42601_d_n0;
        locals.var_c_2esipq_nsub__blk909_dn2 = assign37010_e42601_d_n2;
        locals.var_c_2esipq_nsub__blk909_dn4 = assign37010_e42601_d_n4;
        locals.var_c_2esipq_nsub__blk909_dn5 = assign37010_e42601_d_n5;
        locals.var_c_2esipq_nsub__blk909_dn6 = assign37010_e42601_d_n6;
        locals.var_c_2esipq_nsub__blk909_dn7 = assign37010_e42601_d_n7;
        locals.var_c_2esipq_nsub__blk909_dn8 = assign37010_e42601_d_n8;
        locals.var_c_2esipq_nsub__blk909_dn9 = assign37010_e42601_d_n9;
        locals.var_c_2esipq_nsub__blk909_dn10 = assign37010_e42601_d_n10;
        locals.var_c_2esipq_nsub__blk909_dn13 = assign37010_e42601_d_n13;
        locals.var_c_2esipq_nsub__blk909_rv = 0.0;

        let (assign37020_e42612, assign37020_e42612_d_n0, assign37020_e42612_d_n2, assign37020_e42612_d_n4, assign37020_e42612_d_n5, assign37020_e42612_d_n6, assign37020_e42612_d_n7, assign37020_e42612_d_n8, assign37020_e42612_d_n9, assign37020_e42612_d_n10, assign37020_e42612_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign37020_e42610: f64 = (locals.var_uc_ndepm / locals.var_ef_nsubc);
        (assign37020_e42610, (((locals.var_uc_ndepm_dn0 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn0)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn2 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn2)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn4 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn4)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn5 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn5)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn6 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn6)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn7 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn7)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn8 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn8)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn9 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn9)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn10 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn10)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn13 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn13)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)),)
    } else {
        (locals.var_ndepmpnsub__blk902, locals.var_ndepmpnsub__blk902_dn0, locals.var_ndepmpnsub__blk902_dn2, locals.var_ndepmpnsub__blk902_dn4, locals.var_ndepmpnsub__blk902_dn5, locals.var_ndepmpnsub__blk902_dn6, locals.var_ndepmpnsub__blk902_dn7, locals.var_ndepmpnsub__blk902_dn8, locals.var_ndepmpnsub__blk902_dn9, locals.var_ndepmpnsub__blk902_dn10, locals.var_ndepmpnsub__blk902_dn13,)
    }
};
        locals.var_ndepmpnsub__blk902 = assign37020_e42612;
        locals.var_ndepmpnsub__blk902_dn0 = assign37020_e42612_d_n0;
        locals.var_ndepmpnsub__blk902_dn2 = assign37020_e42612_d_n2;
        locals.var_ndepmpnsub__blk902_dn4 = assign37020_e42612_d_n4;
        locals.var_ndepmpnsub__blk902_dn5 = assign37020_e42612_d_n5;
        locals.var_ndepmpnsub__blk902_dn6 = assign37020_e42612_d_n6;
        locals.var_ndepmpnsub__blk902_dn7 = assign37020_e42612_d_n7;
        locals.var_ndepmpnsub__blk902_dn8 = assign37020_e42612_d_n8;
        locals.var_ndepmpnsub__blk902_dn9 = assign37020_e42612_d_n9;
        locals.var_ndepmpnsub__blk902_dn10 = assign37020_e42612_d_n10;
        locals.var_ndepmpnsub__blk902_dn13 = assign37020_e42612_d_n13;
        locals.var_ndepmpnsub__blk902_rv = 0.0;

        let (assign37030_e42625, assign37030_e42625_d_n0, assign37030_e42625_d_n2, assign37030_e42625_d_n4, assign37030_e42625_d_n5, assign37030_e42625_d_n6, assign37030_e42625_d_n7, assign37030_e42625_d_n8, assign37030_e42625_d_n9, assign37030_e42625_d_n10, assign37030_e42625_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign37030_e42622: f64 = (1.0 + locals.var_ndepmpnsub__blk902);
        let assign37030_e42623: f64 = (1.0 / assign37030_e42622);
        (assign37030_e42623, (-(locals.var_ndepmpnsub__blk902_dn0 / (assign37030_e42622 * assign37030_e42622))), (-(locals.var_ndepmpnsub__blk902_dn2 / (assign37030_e42622 * assign37030_e42622))), (-(locals.var_ndepmpnsub__blk902_dn4 / (assign37030_e42622 * assign37030_e42622))), (-(locals.var_ndepmpnsub__blk902_dn5 / (assign37030_e42622 * assign37030_e42622))), (-(locals.var_ndepmpnsub__blk902_dn6 / (assign37030_e42622 * assign37030_e42622))), (-(locals.var_ndepmpnsub__blk902_dn7 / (assign37030_e42622 * assign37030_e42622))), (-(locals.var_ndepmpnsub__blk902_dn8 / (assign37030_e42622 * assign37030_e42622))), (-(locals.var_ndepmpnsub__blk902_dn9 / (assign37030_e42622 * assign37030_e42622))), (-(locals.var_ndepmpnsub__blk902_dn10 / (assign37030_e42622 * assign37030_e42622))), (-(locals.var_ndepmpnsub__blk902_dn13 / (assign37030_e42622 * assign37030_e42622))),)
    } else {
        (locals.var_ndepmpnsub_inv1__blk901, locals.var_ndepmpnsub_inv1__blk901_dn0, locals.var_ndepmpnsub_inv1__blk901_dn2, locals.var_ndepmpnsub_inv1__blk901_dn4, locals.var_ndepmpnsub_inv1__blk901_dn5, locals.var_ndepmpnsub_inv1__blk901_dn6, locals.var_ndepmpnsub_inv1__blk901_dn7, locals.var_ndepmpnsub_inv1__blk901_dn8, locals.var_ndepmpnsub_inv1__blk901_dn9, locals.var_ndepmpnsub_inv1__blk901_dn10, locals.var_ndepmpnsub_inv1__blk901_dn13,)
    }
};
        locals.var_ndepmpnsub_inv1__blk901 = assign37030_e42625;
        locals.var_ndepmpnsub_inv1__blk901_dn0 = assign37030_e42625_d_n0;
        locals.var_ndepmpnsub_inv1__blk901_dn2 = assign37030_e42625_d_n2;
        locals.var_ndepmpnsub_inv1__blk901_dn4 = assign37030_e42625_d_n4;
        locals.var_ndepmpnsub_inv1__blk901_dn5 = assign37030_e42625_d_n5;
        locals.var_ndepmpnsub_inv1__blk901_dn6 = assign37030_e42625_d_n6;
        locals.var_ndepmpnsub_inv1__blk901_dn7 = assign37030_e42625_d_n7;
        locals.var_ndepmpnsub_inv1__blk901_dn8 = assign37030_e42625_d_n8;
        locals.var_ndepmpnsub_inv1__blk901_dn9 = assign37030_e42625_d_n9;
        locals.var_ndepmpnsub_inv1__blk901_dn10 = assign37030_e42625_d_n10;
        locals.var_ndepmpnsub_inv1__blk901_dn13 = assign37030_e42625_d_n13;
        locals.var_ndepmpnsub_inv1__blk901_rv = 0.0;

        let (assign37040_e42638, assign37040_e42638_d_n0, assign37040_e42638_d_n2, assign37040_e42638_d_n4, assign37040_e42638_d_n5, assign37040_e42638_d_n6, assign37040_e42638_d_n7, assign37040_e42638_d_n8, assign37040_e42638_d_n9, assign37040_e42638_d_n10, assign37040_e42638_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign37040_e42635: f64 = (locals.var_cox * locals.var_cox);
        let assign37040_e42636: f64 = (locals.var_q_ndepm_esi__blk884 / assign37040_e42635);
        (assign37040_e42636, (((locals.var_q_ndepm_esi__blk884_dn0 * assign37040_e42635) - (locals.var_q_ndepm_esi__blk884 * ((locals.var_cox_dn0 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn0)))) / (assign37040_e42635 * assign37040_e42635)), (((locals.var_q_ndepm_esi__blk884_dn2 * assign37040_e42635) - (locals.var_q_ndepm_esi__blk884 * ((locals.var_cox_dn2 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn2)))) / (assign37040_e42635 * assign37040_e42635)), (((locals.var_q_ndepm_esi__blk884_dn4 * assign37040_e42635) - (locals.var_q_ndepm_esi__blk884 * ((locals.var_cox_dn4 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn4)))) / (assign37040_e42635 * assign37040_e42635)), (((locals.var_q_ndepm_esi__blk884_dn5 * assign37040_e42635) - (locals.var_q_ndepm_esi__blk884 * ((locals.var_cox_dn5 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn5)))) / (assign37040_e42635 * assign37040_e42635)), (((locals.var_q_ndepm_esi__blk884_dn6 * assign37040_e42635) - (locals.var_q_ndepm_esi__blk884 * ((locals.var_cox_dn6 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn6)))) / (assign37040_e42635 * assign37040_e42635)), (((locals.var_q_ndepm_esi__blk884_dn7 * assign37040_e42635) - (locals.var_q_ndepm_esi__blk884 * ((locals.var_cox_dn7 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn7)))) / (assign37040_e42635 * assign37040_e42635)), (((locals.var_q_ndepm_esi__blk884_dn8 * assign37040_e42635) - (locals.var_q_ndepm_esi__blk884 * ((locals.var_cox_dn8 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn8)))) / (assign37040_e42635 * assign37040_e42635)), (((locals.var_q_ndepm_esi__blk884_dn9 * assign37040_e42635) - (locals.var_q_ndepm_esi__blk884 * ((locals.var_cox_dn9 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn9)))) / (assign37040_e42635 * assign37040_e42635)), (((locals.var_q_ndepm_esi__blk884_dn10 * assign37040_e42635) - (locals.var_q_ndepm_esi__blk884 * ((locals.var_cox_dn10 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn10)))) / (assign37040_e42635 * assign37040_e42635)), (((locals.var_q_ndepm_esi__blk884_dn13 * assign37040_e42635) - (locals.var_q_ndepm_esi__blk884 * ((locals.var_cox_dn13 * locals.var_cox) + (locals.var_cox * locals.var_cox_dn13)))) / (assign37040_e42635 * assign37040_e42635)),)
    } else {
        (locals.var_q_ndepm_esi_cox_inv2, locals.var_q_ndepm_esi_cox_inv2_dn0, locals.var_q_ndepm_esi_cox_inv2_dn2, locals.var_q_ndepm_esi_cox_inv2_dn4, locals.var_q_ndepm_esi_cox_inv2_dn5, locals.var_q_ndepm_esi_cox_inv2_dn6, locals.var_q_ndepm_esi_cox_inv2_dn7, locals.var_q_ndepm_esi_cox_inv2_dn8, locals.var_q_ndepm_esi_cox_inv2_dn9, locals.var_q_ndepm_esi_cox_inv2_dn10, locals.var_q_ndepm_esi_cox_inv2_dn13,)
    }
};
        locals.var_q_ndepm_esi_cox_inv2 = assign37040_e42638;
        locals.var_q_ndepm_esi_cox_inv2_dn0 = assign37040_e42638_d_n0;
        locals.var_q_ndepm_esi_cox_inv2_dn2 = assign37040_e42638_d_n2;
        locals.var_q_ndepm_esi_cox_inv2_dn4 = assign37040_e42638_d_n4;
        locals.var_q_ndepm_esi_cox_inv2_dn5 = assign37040_e42638_d_n5;
        locals.var_q_ndepm_esi_cox_inv2_dn6 = assign37040_e42638_d_n6;
        locals.var_q_ndepm_esi_cox_inv2_dn7 = assign37040_e42638_d_n7;
        locals.var_q_ndepm_esi_cox_inv2_dn8 = assign37040_e42638_d_n8;
        locals.var_q_ndepm_esi_cox_inv2_dn9 = assign37040_e42638_d_n9;
        locals.var_q_ndepm_esi_cox_inv2_dn10 = assign37040_e42638_d_n10;
        locals.var_q_ndepm_esi_cox_inv2_dn13 = assign37040_e42638_d_n13;
        locals.var_q_ndepm_esi_cox_inv2_rv = 0.0;

        let (assign37050_e42649, assign37050_e42649_d_n0, assign37050_e42649_d_n2, assign37050_e42649_d_n4, assign37050_e42649_d_n5, assign37050_e42649_d_n6, assign37050_e42649_d_n7, assign37050_e42649_d_n8, assign37050_e42649_d_n9, assign37050_e42649_d_n10, assign37050_e42649_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign37050_e42647: f64 = (2.0 / locals.var_q_ndepm_esi_cox_inv2);
        (assign37050_e42647, (-((2.0 * locals.var_q_ndepm_esi_cox_inv2_dn0) / (locals.var_q_ndepm_esi_cox_inv2 * locals.var_q_ndepm_esi_cox_inv2))), (-((2.0 * locals.var_q_ndepm_esi_cox_inv2_dn2) / (locals.var_q_ndepm_esi_cox_inv2 * locals.var_q_ndepm_esi_cox_inv2))), (-((2.0 * locals.var_q_ndepm_esi_cox_inv2_dn4) / (locals.var_q_ndepm_esi_cox_inv2 * locals.var_q_ndepm_esi_cox_inv2))), (-((2.0 * locals.var_q_ndepm_esi_cox_inv2_dn5) / (locals.var_q_ndepm_esi_cox_inv2 * locals.var_q_ndepm_esi_cox_inv2))), (-((2.0 * locals.var_q_ndepm_esi_cox_inv2_dn6) / (locals.var_q_ndepm_esi_cox_inv2 * locals.var_q_ndepm_esi_cox_inv2))), (-((2.0 * locals.var_q_ndepm_esi_cox_inv2_dn7) / (locals.var_q_ndepm_esi_cox_inv2 * locals.var_q_ndepm_esi_cox_inv2))), (-((2.0 * locals.var_q_ndepm_esi_cox_inv2_dn8) / (locals.var_q_ndepm_esi_cox_inv2 * locals.var_q_ndepm_esi_cox_inv2))), (-((2.0 * locals.var_q_ndepm_esi_cox_inv2_dn9) / (locals.var_q_ndepm_esi_cox_inv2 * locals.var_q_ndepm_esi_cox_inv2))), (-((2.0 * locals.var_q_ndepm_esi_cox_inv2_dn10) / (locals.var_q_ndepm_esi_cox_inv2 * locals.var_q_ndepm_esi_cox_inv2))), (-((2.0 * locals.var_q_ndepm_esi_cox_inv2_dn13) / (locals.var_q_ndepm_esi_cox_inv2 * locals.var_q_ndepm_esi_cox_inv2))),)
    } else {
        (locals.var_c2_q_ndepm_esi_cox_inv2, locals.var_c2_q_ndepm_esi_cox_inv2_dn0, locals.var_c2_q_ndepm_esi_cox_inv2_dn2, locals.var_c2_q_ndepm_esi_cox_inv2_dn4, locals.var_c2_q_ndepm_esi_cox_inv2_dn5, locals.var_c2_q_ndepm_esi_cox_inv2_dn6, locals.var_c2_q_ndepm_esi_cox_inv2_dn7, locals.var_c2_q_ndepm_esi_cox_inv2_dn8, locals.var_c2_q_ndepm_esi_cox_inv2_dn9, locals.var_c2_q_ndepm_esi_cox_inv2_dn10, locals.var_c2_q_ndepm_esi_cox_inv2_dn13,)
    }
};
        locals.var_c2_q_ndepm_esi_cox_inv2 = assign37050_e42649;
        locals.var_c2_q_ndepm_esi_cox_inv2_dn0 = assign37050_e42649_d_n0;
        locals.var_c2_q_ndepm_esi_cox_inv2_dn2 = assign37050_e42649_d_n2;
        locals.var_c2_q_ndepm_esi_cox_inv2_dn4 = assign37050_e42649_d_n4;
        locals.var_c2_q_ndepm_esi_cox_inv2_dn5 = assign37050_e42649_d_n5;
        locals.var_c2_q_ndepm_esi_cox_inv2_dn6 = assign37050_e42649_d_n6;
        locals.var_c2_q_ndepm_esi_cox_inv2_dn7 = assign37050_e42649_d_n7;
        locals.var_c2_q_ndepm_esi_cox_inv2_dn8 = assign37050_e42649_d_n8;
        locals.var_c2_q_ndepm_esi_cox_inv2_dn9 = assign37050_e42649_d_n9;
        locals.var_c2_q_ndepm_esi_cox_inv2_dn10 = assign37050_e42649_d_n10;
        locals.var_c2_q_ndepm_esi_cox_inv2_dn13 = assign37050_e42649_d_n13;
        locals.var_c2_q_ndepm_esi_cox_inv2_rv = 0.0;

        let (assign37060_e42658,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        (4.0,)
    } else {
        (locals.var_depqfn_dlt,)
    }
};
        locals.var_depqfn_dlt = assign37060_e42658;
        locals.var_depqfn_dlt_rv = 0.0;

        let (assign37070_e42667,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        (0.1,)
    } else {
        (locals.var_ps_delta,)
    }
};
        locals.var_ps_delta = assign37070_e42667;
        locals.var_ps_delta_rv = 0.0;

        let (assign37080_e42676,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        (0.1,)
    } else {
        (locals.var_vfboffset,)
    }
};
        locals.var_vfboffset = assign37080_e42676;
        locals.var_vfboffset_rv = 0.0;

        let (assign37090_e42687, assign37090_e42687_d_n0, assign37090_e42687_d_n2, assign37090_e42687_d_n4, assign37090_e42687_d_n5, assign37090_e42687_d_n6, assign37090_e42687_d_n7, assign37090_e42687_d_n8, assign37090_e42687_d_n9, assign37090_e42687_d_n10, assign37090_e42687_d_n13,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        let assign37090_e42685: f64 = (locals.var_pb2n + p.p407);
        (assign37090_e42685, locals.var_pb2n_dn0, locals.var_pb2n_dn2, locals.var_pb2n_dn4, locals.var_pb2n_dn5, locals.var_pb2n_dn6, locals.var_pb2n_dn7, locals.var_pb2n_dn8, locals.var_pb2n_dn9, locals.var_pb2n_dn10, locals.var_pb2n_dn13,)
    } else {
        (locals.var_vgpdep_dlt, locals.var_vgpdep_dlt_dn0, locals.var_vgpdep_dlt_dn2, locals.var_vgpdep_dlt_dn4, locals.var_vgpdep_dlt_dn5, locals.var_vgpdep_dlt_dn6, locals.var_vgpdep_dlt_dn7, locals.var_vgpdep_dlt_dn8, locals.var_vgpdep_dlt_dn9, locals.var_vgpdep_dlt_dn10, locals.var_vgpdep_dlt_dn13,)
    }
};
        locals.var_vgpdep_dlt = assign37090_e42687;
        locals.var_vgpdep_dlt_dn0 = assign37090_e42687_d_n0;
        locals.var_vgpdep_dlt_dn2 = assign37090_e42687_d_n2;
        locals.var_vgpdep_dlt_dn4 = assign37090_e42687_d_n4;
        locals.var_vgpdep_dlt_dn5 = assign37090_e42687_d_n5;
        locals.var_vgpdep_dlt_dn6 = assign37090_e42687_d_n6;
        locals.var_vgpdep_dlt_dn7 = assign37090_e42687_d_n7;
        locals.var_vgpdep_dlt_dn8 = assign37090_e42687_d_n8;
        locals.var_vgpdep_dlt_dn9 = assign37090_e42687_d_n9;
        locals.var_vgpdep_dlt_dn10 = assign37090_e42687_d_n10;
        locals.var_vgpdep_dlt_dn13 = assign37090_e42687_d_n13;
        locals.var_vgpdep_dlt_rv = 0.0;

        let (assign37100_e42696,) = {
    if ((locals.var_guard443 != 0.0) && ((locals.var_guard445 != 0.0) && (locals.var_guard444 == 0.0))) {
        (3.0,)
    } else {
        (locals.var_vgpdep_pw,)
    }
};
        locals.var_vgpdep_pw = assign37100_e42696;
        locals.var_vgpdep_pw_rv = 0.0;

    }
}
