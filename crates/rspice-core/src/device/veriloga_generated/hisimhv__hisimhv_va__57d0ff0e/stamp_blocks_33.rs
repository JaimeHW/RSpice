#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_136(
        locals: &mut StampLocals,
    ) {
        let (assign39850_e52630, assign39850_e52630_d_n0, assign39850_e52630_d_n2, assign39850_e52630_d_n4, assign39850_e52630_d_n5, assign39850_e52630_d_n6, assign39850_e52630_d_n7, assign39850_e52630_d_n8, assign39850_e52630_d_n9, assign39850_e52630_d_n10, assign39850_e52630_d_n11, assign39850_e52630_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) {
        let assign39850_e52627: f64 = (locals.var_phi_sl_dep__blk858 - locals.var_phi_bl_dep__blk861);
        let assign39850_e52628: f64 = (locals.var_beta * assign39850_e52627);
        (assign39850_e52628, ((locals.var_beta_dn0 * assign39850_e52627) + (locals.var_beta * (locals.var_phi_sl_dep__blk858_dn0 - locals.var_phi_bl_dep__blk861_dn0))), ((locals.var_beta_dn2 * assign39850_e52627) + (locals.var_beta * (locals.var_phi_sl_dep__blk858_dn2 - locals.var_phi_bl_dep__blk861_dn2))), ((locals.var_beta_dn4 * assign39850_e52627) + (locals.var_beta * (locals.var_phi_sl_dep__blk858_dn4 - locals.var_phi_bl_dep__blk861_dn4))), ((locals.var_beta_dn5 * assign39850_e52627) + (locals.var_beta * (locals.var_phi_sl_dep__blk858_dn5 - locals.var_phi_bl_dep__blk861_dn5))), ((locals.var_beta_dn6 * assign39850_e52627) + (locals.var_beta * (locals.var_phi_sl_dep__blk858_dn6 - locals.var_phi_bl_dep__blk861_dn6))), ((locals.var_beta_dn7 * assign39850_e52627) + (locals.var_beta * (locals.var_phi_sl_dep__blk858_dn7 - locals.var_phi_bl_dep__blk861_dn7))), ((locals.var_beta_dn8 * assign39850_e52627) + (locals.var_beta * (locals.var_phi_sl_dep__blk858_dn8 - locals.var_phi_bl_dep__blk861_dn8))), ((locals.var_beta_dn9 * assign39850_e52627) + (locals.var_beta * (locals.var_phi_sl_dep__blk858_dn9 - locals.var_phi_bl_dep__blk861_dn9))), ((locals.var_beta_dn10 * assign39850_e52627) + (locals.var_beta * (locals.var_phi_sl_dep__blk858_dn10 - locals.var_phi_bl_dep__blk861_dn10))), ((locals.var_beta_dn11 * assign39850_e52627) + (locals.var_beta * (locals.var_phi_sl_dep__blk858_dn11 - locals.var_phi_bl_dep__blk861_dn11))), ((locals.var_beta_dn14 * assign39850_e52627) + (locals.var_beta * (locals.var_phi_sl_dep__blk858_dn14 - locals.var_phi_bl_dep__blk861_dn14))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign39850_e52630;
        locals.var_t1_dn0 = assign39850_e52630_d_n0;
        locals.var_t1_dn2 = assign39850_e52630_d_n2;
        locals.var_t1_dn4 = assign39850_e52630_d_n4;
        locals.var_t1_dn5 = assign39850_e52630_d_n5;
        locals.var_t1_dn6 = assign39850_e52630_d_n6;
        locals.var_t1_dn7 = assign39850_e52630_d_n7;
        locals.var_t1_dn8 = assign39850_e52630_d_n8;
        locals.var_t1_dn9 = assign39850_e52630_d_n9;
        locals.var_t1_dn10 = assign39850_e52630_d_n10;
        locals.var_t1_dn11 = assign39850_e52630_d_n11;
        locals.var_t1_dn14 = assign39850_e52630_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign39860_e52643, assign39860_e52643_d_n0, assign39860_e52643_d_n2, assign39860_e52643_d_n4, assign39860_e52643_d_n5, assign39860_e52643_d_n6, assign39860_e52643_d_n7, assign39860_e52643_d_n8, assign39860_e52643_d_n9, assign39860_e52643_d_n10, assign39860_e52643_d_n11, assign39860_e52643_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) {
        let assign39860_e52641: f64 = (locals.var_t1).exp();
        (assign39860_e52641, (assign39860_e52641 * locals.var_t1_dn0), (assign39860_e52641 * locals.var_t1_dn2), (assign39860_e52641 * locals.var_t1_dn4), (assign39860_e52641 * locals.var_t1_dn5), (assign39860_e52641 * locals.var_t1_dn6), (assign39860_e52641 * locals.var_t1_dn7), (assign39860_e52641 * locals.var_t1_dn8), (assign39860_e52641 * locals.var_t1_dn9), (assign39860_e52641 * locals.var_t1_dn10), (assign39860_e52641 * locals.var_t1_dn11), (assign39860_e52641 * locals.var_t1_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign39860_e52643;
        locals.var_t2_dn0 = assign39860_e52643_d_n0;
        locals.var_t2_dn2 = assign39860_e52643_d_n2;
        locals.var_t2_dn4 = assign39860_e52643_d_n4;
        locals.var_t2_dn5 = assign39860_e52643_d_n5;
        locals.var_t2_dn6 = assign39860_e52643_d_n6;
        locals.var_t2_dn7 = assign39860_e52643_d_n7;
        locals.var_t2_dn8 = assign39860_e52643_d_n8;
        locals.var_t2_dn9 = assign39860_e52643_d_n9;
        locals.var_t2_dn10 = assign39860_e52643_d_n10;
        locals.var_t2_dn11 = assign39860_e52643_d_n11;
        locals.var_t2_dn14 = assign39860_e52643_d_n14;
        locals.var_t2_rv = 0.0;

        let assign39870_e52646: f64 = if locals.var_phi_sl_dep__blk858 >= locals.var_phi_bl_dep__blk861 { 1.0 } else { 0.0 };
        locals.var_guard1009 = assign39870_e52646;
        locals.var_guard1009_rv = 0.0;

        let (assign39880_e52660, assign39880_e52660_d_n0, assign39880_e52660_d_n2, assign39880_e52660_d_n4, assign39880_e52660_d_n5, assign39880_e52660_d_n6, assign39880_e52660_d_n7, assign39880_e52660_d_n8, assign39880_e52660_d_n9, assign39880_e52660_d_n10, assign39880_e52660_d_n11, assign39880_e52660_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) && (locals.var_guard1009 != 0.0)) {
        (locals.var_q_sl__blk867, locals.var_q_sl__blk867_dn0, locals.var_q_sl__blk867_dn2, locals.var_q_sl__blk867_dn4, locals.var_q_sl__blk867_dn5, locals.var_q_sl__blk867_dn6, locals.var_q_sl__blk867_dn7, locals.var_q_sl__blk867_dn8, locals.var_q_sl__blk867_dn9, locals.var_q_sl__blk867_dn10, locals.var_q_sl__blk867_dn11, locals.var_q_sl__blk867_dn14,)
    } else {
        (locals.var_q_nl__blk897, locals.var_q_nl__blk897_dn0, locals.var_q_nl__blk897_dn2, locals.var_q_nl__blk897_dn4, locals.var_q_nl__blk897_dn5, locals.var_q_nl__blk897_dn6, locals.var_q_nl__blk897_dn7, locals.var_q_nl__blk897_dn8, locals.var_q_nl__blk897_dn9, locals.var_q_nl__blk897_dn10, locals.var_q_nl__blk897_dn11, locals.var_q_nl__blk897_dn14,)
    }
};
        locals.var_q_nl__blk897 = assign39880_e52660;
        locals.var_q_nl__blk897_dn0 = assign39880_e52660_d_n0;
        locals.var_q_nl__blk897_dn2 = assign39880_e52660_d_n2;
        locals.var_q_nl__blk897_dn4 = assign39880_e52660_d_n4;
        locals.var_q_nl__blk897_dn5 = assign39880_e52660_d_n5;
        locals.var_q_nl__blk897_dn6 = assign39880_e52660_d_n6;
        locals.var_q_nl__blk897_dn7 = assign39880_e52660_d_n7;
        locals.var_q_nl__blk897_dn8 = assign39880_e52660_d_n8;
        locals.var_q_nl__blk897_dn9 = assign39880_e52660_d_n9;
        locals.var_q_nl__blk897_dn10 = assign39880_e52660_d_n10;
        locals.var_q_nl__blk897_dn11 = assign39880_e52660_d_n11;
        locals.var_q_nl__blk897_dn14 = assign39880_e52660_d_n14;
        locals.var_q_nl__blk897_rv = 0.0;

        let (assign39890_e52674, assign39890_e52674_d_n0, assign39890_e52674_d_n2, assign39890_e52674_d_n4, assign39890_e52674_d_n5, assign39890_e52674_d_n6, assign39890_e52674_d_n7, assign39890_e52674_d_n8, assign39890_e52674_d_n9, assign39890_e52674_d_n10, assign39890_e52674_d_n11, assign39890_e52674_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) && (locals.var_guard1009 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_sl_dep__blk900, locals.var_q_sl_dep__blk900_dn0, locals.var_q_sl_dep__blk900_dn2, locals.var_q_sl_dep__blk900_dn4, locals.var_q_sl_dep__blk900_dn5, locals.var_q_sl_dep__blk900_dn6, locals.var_q_sl_dep__blk900_dn7, locals.var_q_sl_dep__blk900_dn8, locals.var_q_sl_dep__blk900_dn9, locals.var_q_sl_dep__blk900_dn10, locals.var_q_sl_dep__blk900_dn11, locals.var_q_sl_dep__blk900_dn14,)
    }
};
        locals.var_q_sl_dep__blk900 = assign39890_e52674;
        locals.var_q_sl_dep__blk900_dn0 = assign39890_e52674_d_n0;
        locals.var_q_sl_dep__blk900_dn2 = assign39890_e52674_d_n2;
        locals.var_q_sl_dep__blk900_dn4 = assign39890_e52674_d_n4;
        locals.var_q_sl_dep__blk900_dn5 = assign39890_e52674_d_n5;
        locals.var_q_sl_dep__blk900_dn6 = assign39890_e52674_d_n6;
        locals.var_q_sl_dep__blk900_dn7 = assign39890_e52674_d_n7;
        locals.var_q_sl_dep__blk900_dn8 = assign39890_e52674_d_n8;
        locals.var_q_sl_dep__blk900_dn9 = assign39890_e52674_d_n9;
        locals.var_q_sl_dep__blk900_dn10 = assign39890_e52674_d_n10;
        locals.var_q_sl_dep__blk900_dn11 = assign39890_e52674_d_n11;
        locals.var_q_sl_dep__blk900_dn14 = assign39890_e52674_d_n14;
        locals.var_q_sl_dep__blk900_rv = 0.0;

        let (assign39900_e52688, assign39900_e52688_d_n0, assign39900_e52688_d_n2, assign39900_e52688_d_n4, assign39900_e52688_d_n5, assign39900_e52688_d_n6, assign39900_e52688_d_n7, assign39900_e52688_d_n8, assign39900_e52688_d_n9, assign39900_e52688_d_n10, assign39900_e52688_d_n11, assign39900_e52688_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) && (locals.var_guard1009 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_subl__blk869, locals.var_q_subl__blk869_dn0, locals.var_q_subl__blk869_dn2, locals.var_q_subl__blk869_dn4, locals.var_q_subl__blk869_dn5, locals.var_q_subl__blk869_dn6, locals.var_q_subl__blk869_dn7, locals.var_q_subl__blk869_dn8, locals.var_q_subl__blk869_dn9, locals.var_q_subl__blk869_dn10, locals.var_q_subl__blk869_dn11, locals.var_q_subl__blk869_dn14,)
    }
};
        locals.var_q_subl__blk869 = assign39900_e52688;
        locals.var_q_subl__blk869_dn0 = assign39900_e52688_d_n0;
        locals.var_q_subl__blk869_dn2 = assign39900_e52688_d_n2;
        locals.var_q_subl__blk869_dn4 = assign39900_e52688_d_n4;
        locals.var_q_subl__blk869_dn5 = assign39900_e52688_d_n5;
        locals.var_q_subl__blk869_dn6 = assign39900_e52688_d_n6;
        locals.var_q_subl__blk869_dn7 = assign39900_e52688_d_n7;
        locals.var_q_subl__blk869_dn8 = assign39900_e52688_d_n8;
        locals.var_q_subl__blk869_dn9 = assign39900_e52688_d_n9;
        locals.var_q_subl__blk869_dn10 = assign39900_e52688_d_n10;
        locals.var_q_subl__blk869_dn11 = assign39900_e52688_d_n11;
        locals.var_q_subl__blk869_dn14 = assign39900_e52688_d_n14;
        locals.var_q_subl__blk869_rv = 0.0;

        let (assign39910_e52703, assign39910_e52703_d_n0, assign39910_e52703_d_n2, assign39910_e52703_d_n4, assign39910_e52703_d_n5, assign39910_e52703_d_n6, assign39910_e52703_d_n7, assign39910_e52703_d_n8, assign39910_e52703_d_n9, assign39910_e52703_d_n10, assign39910_e52703_d_n11, assign39910_e52703_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) && (locals.var_guard1009 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_nl__blk897, locals.var_q_nl__blk897_dn0, locals.var_q_nl__blk897_dn2, locals.var_q_nl__blk897_dn4, locals.var_q_nl__blk897_dn5, locals.var_q_nl__blk897_dn6, locals.var_q_nl__blk897_dn7, locals.var_q_nl__blk897_dn8, locals.var_q_nl__blk897_dn9, locals.var_q_nl__blk897_dn10, locals.var_q_nl__blk897_dn11, locals.var_q_nl__blk897_dn14,)
    }
};
        locals.var_q_nl__blk897 = assign39910_e52703;
        locals.var_q_nl__blk897_dn0 = assign39910_e52703_d_n0;
        locals.var_q_nl__blk897_dn2 = assign39910_e52703_d_n2;
        locals.var_q_nl__blk897_dn4 = assign39910_e52703_d_n4;
        locals.var_q_nl__blk897_dn5 = assign39910_e52703_d_n5;
        locals.var_q_nl__blk897_dn6 = assign39910_e52703_d_n6;
        locals.var_q_nl__blk897_dn7 = assign39910_e52703_d_n7;
        locals.var_q_nl__blk897_dn8 = assign39910_e52703_d_n8;
        locals.var_q_nl__blk897_dn9 = assign39910_e52703_d_n9;
        locals.var_q_nl__blk897_dn10 = assign39910_e52703_d_n10;
        locals.var_q_nl__blk897_dn11 = assign39910_e52703_d_n11;
        locals.var_q_nl__blk897_dn14 = assign39910_e52703_d_n14;
        locals.var_q_nl__blk897_rv = 0.0;

        let (assign39920_e52727, assign39920_e52727_d_n0, assign39920_e52727_d_n2, assign39920_e52727_d_n4, assign39920_e52727_d_n5, assign39920_e52727_d_n6, assign39920_e52727_d_n7, assign39920_e52727_d_n8, assign39920_e52727_d_n9, assign39920_e52727_d_n10, assign39920_e52727_d_n11, assign39920_e52727_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) && (locals.var_guard1009 == 0.0)) {
        let assign39920_e52719: f64 = (locals.var_t2 - 1.0);
        let assign39920_e52721: f64 = (assign39920_e52719 - locals.var_t1);
        let assign39920_e52723: f64 = (assign39920_e52721 + 1e-15);
        let assign39920_e52724: f64 = (assign39920_e52723).sqrt();
        let assign39920_e52725: f64 = (locals.var_cnst0 * assign39920_e52724);
        (assign39920_e52725, ((locals.var_cnst0_dn0 * assign39920_e52724) + (locals.var_cnst0 * ((locals.var_t2_dn0 - locals.var_t1_dn0) / (2.0 * assign39920_e52724)))), ((locals.var_cnst0_dn2 * assign39920_e52724) + (locals.var_cnst0 * ((locals.var_t2_dn2 - locals.var_t1_dn2) / (2.0 * assign39920_e52724)))), ((locals.var_cnst0_dn4 * assign39920_e52724) + (locals.var_cnst0 * ((locals.var_t2_dn4 - locals.var_t1_dn4) / (2.0 * assign39920_e52724)))), ((locals.var_cnst0_dn5 * assign39920_e52724) + (locals.var_cnst0 * ((locals.var_t2_dn5 - locals.var_t1_dn5) / (2.0 * assign39920_e52724)))), ((locals.var_cnst0_dn6 * assign39920_e52724) + (locals.var_cnst0 * ((locals.var_t2_dn6 - locals.var_t1_dn6) / (2.0 * assign39920_e52724)))), ((locals.var_cnst0_dn7 * assign39920_e52724) + (locals.var_cnst0 * ((locals.var_t2_dn7 - locals.var_t1_dn7) / (2.0 * assign39920_e52724)))), ((locals.var_cnst0_dn8 * assign39920_e52724) + (locals.var_cnst0 * ((locals.var_t2_dn8 - locals.var_t1_dn8) / (2.0 * assign39920_e52724)))), ((locals.var_cnst0_dn9 * assign39920_e52724) + (locals.var_cnst0 * ((locals.var_t2_dn9 - locals.var_t1_dn9) / (2.0 * assign39920_e52724)))), ((locals.var_cnst0_dn10 * assign39920_e52724) + (locals.var_cnst0 * ((locals.var_t2_dn10 - locals.var_t1_dn10) / (2.0 * assign39920_e52724)))), ((locals.var_cnst0_dn11 * assign39920_e52724) + (locals.var_cnst0 * ((locals.var_t2_dn11 - locals.var_t1_dn11) / (2.0 * assign39920_e52724)))), ((locals.var_cnst0_dn14 * assign39920_e52724) + (locals.var_cnst0 * ((locals.var_t2_dn14 - locals.var_t1_dn14) / (2.0 * assign39920_e52724)))),)
    } else {
        (locals.var_q_sl_dep__blk900, locals.var_q_sl_dep__blk900_dn0, locals.var_q_sl_dep__blk900_dn2, locals.var_q_sl_dep__blk900_dn4, locals.var_q_sl_dep__blk900_dn5, locals.var_q_sl_dep__blk900_dn6, locals.var_q_sl_dep__blk900_dn7, locals.var_q_sl_dep__blk900_dn8, locals.var_q_sl_dep__blk900_dn9, locals.var_q_sl_dep__blk900_dn10, locals.var_q_sl_dep__blk900_dn11, locals.var_q_sl_dep__blk900_dn14,)
    }
};
        locals.var_q_sl_dep__blk900 = assign39920_e52727;
        locals.var_q_sl_dep__blk900_dn0 = assign39920_e52727_d_n0;
        locals.var_q_sl_dep__blk900_dn2 = assign39920_e52727_d_n2;
        locals.var_q_sl_dep__blk900_dn4 = assign39920_e52727_d_n4;
        locals.var_q_sl_dep__blk900_dn5 = assign39920_e52727_d_n5;
        locals.var_q_sl_dep__blk900_dn6 = assign39920_e52727_d_n6;
        locals.var_q_sl_dep__blk900_dn7 = assign39920_e52727_d_n7;
        locals.var_q_sl_dep__blk900_dn8 = assign39920_e52727_d_n8;
        locals.var_q_sl_dep__blk900_dn9 = assign39920_e52727_d_n9;
        locals.var_q_sl_dep__blk900_dn10 = assign39920_e52727_d_n10;
        locals.var_q_sl_dep__blk900_dn11 = assign39920_e52727_d_n11;
        locals.var_q_sl_dep__blk900_dn14 = assign39920_e52727_d_n14;
        locals.var_q_sl_dep__blk900_rv = 0.0;

        let assign39930_e52730: f64 = if locals.var_w_bsubl__blk841 > locals.var_uc_depthn { 1.0 } else { 0.0 };
        locals.var_guard1010 = assign39930_e52730;
        locals.var_guard1010_rv = 0.0;

        let (assign39940_e52747, assign39940_e52747_d_n0, assign39940_e52747_d_n2, assign39940_e52747_d_n4, assign39940_e52747_d_n5, assign39940_e52747_d_n6, assign39940_e52747_d_n7, assign39940_e52747_d_n8, assign39940_e52747_d_n9, assign39940_e52747_d_n10, assign39940_e52747_d_n11, assign39940_e52747_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) && (locals.var_guard1009 == 0.0)) && (locals.var_guard1010 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_subl__blk869, locals.var_q_subl__blk869_dn0, locals.var_q_subl__blk869_dn2, locals.var_q_subl__blk869_dn4, locals.var_q_subl__blk869_dn5, locals.var_q_subl__blk869_dn6, locals.var_q_subl__blk869_dn7, locals.var_q_subl__blk869_dn8, locals.var_q_subl__blk869_dn9, locals.var_q_subl__blk869_dn10, locals.var_q_subl__blk869_dn11, locals.var_q_subl__blk869_dn14,)
    }
};
        locals.var_q_subl__blk869 = assign39940_e52747;
        locals.var_q_subl__blk869_dn0 = assign39940_e52747_d_n0;
        locals.var_q_subl__blk869_dn2 = assign39940_e52747_d_n2;
        locals.var_q_subl__blk869_dn4 = assign39940_e52747_d_n4;
        locals.var_q_subl__blk869_dn5 = assign39940_e52747_d_n5;
        locals.var_q_subl__blk869_dn6 = assign39940_e52747_d_n6;
        locals.var_q_subl__blk869_dn7 = assign39940_e52747_d_n7;
        locals.var_q_subl__blk869_dn8 = assign39940_e52747_d_n8;
        locals.var_q_subl__blk869_dn9 = assign39940_e52747_d_n9;
        locals.var_q_subl__blk869_dn10 = assign39940_e52747_d_n10;
        locals.var_q_subl__blk869_dn11 = assign39940_e52747_d_n11;
        locals.var_q_subl__blk869_dn14 = assign39940_e52747_d_n14;
        locals.var_q_subl__blk869_rv = 0.0;

        let (assign39950_e52787, assign39950_e52787_d_n0, assign39950_e52787_d_n2, assign39950_e52787_d_n4, assign39950_e52787_d_n5, assign39950_e52787_d_n6, assign39950_e52787_d_n7, assign39950_e52787_d_n8, assign39950_e52787_d_n9, assign39950_e52787_d_n10, assign39950_e52787_d_n11, assign39950_e52787_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) && (locals.var_guard1009 == 0.0)) && (locals.var_guard1010 == 0.0)) {
        let assign39950_e52765: f64 = (-locals.var_t1);
        let assign39950_e52768: f64 = (-locals.var_beta);
        let assign39950_e52771: f64 = (locals.var_phi_sl_dep__blk858 - locals.var_vbsc);
        let assign39950_e52772: f64 = (assign39950_e52768 * assign39950_e52771);
        let assign39950_e52773: f64 = (assign39950_e52772).exp();
        let assign39950_e52775: f64 = (-locals.var_beta);
        let assign39950_e52778: f64 = (locals.var_phi_bl_dep__blk861 - locals.var_vbsc);
        let assign39950_e52779: f64 = (assign39950_e52775 * assign39950_e52778);
        let assign39950_e52780: f64 = (assign39950_e52779).exp();
        let assign39950_e52781: f64 = (assign39950_e52773 - assign39950_e52780);
        let assign39950_e52782: f64 = (locals.var_cnst1 * assign39950_e52781);
        let assign39950_e52783: f64 = (assign39950_e52765 + assign39950_e52782);
        let assign39950_e52784: f64 = (assign39950_e52783).sqrt();
        let assign39950_e52785: f64 = (locals.var_cnst0 * assign39950_e52784);
        (assign39950_e52785, ((locals.var_cnst0_dn0 * assign39950_e52784) + (locals.var_cnst0 * (((-locals.var_t1_dn0) + ((locals.var_cnst1_dn0 * assign39950_e52781) + (locals.var_cnst1 * ((assign39950_e52773 * (((-locals.var_beta_dn0) * assign39950_e52771) + (assign39950_e52768 * (locals.var_phi_sl_dep__blk858_dn0 - locals.var_vbsc_dn0)))) - (assign39950_e52780 * (((-locals.var_beta_dn0) * assign39950_e52778) + (assign39950_e52775 * (locals.var_phi_bl_dep__blk861_dn0 - locals.var_vbsc_dn0)))))))) / (2.0 * assign39950_e52784)))), ((locals.var_cnst0_dn2 * assign39950_e52784) + (locals.var_cnst0 * (((-locals.var_t1_dn2) + ((locals.var_cnst1_dn2 * assign39950_e52781) + (locals.var_cnst1 * ((assign39950_e52773 * (((-locals.var_beta_dn2) * assign39950_e52771) + (assign39950_e52768 * (locals.var_phi_sl_dep__blk858_dn2 - locals.var_vbsc_dn2)))) - (assign39950_e52780 * (((-locals.var_beta_dn2) * assign39950_e52778) + (assign39950_e52775 * (locals.var_phi_bl_dep__blk861_dn2 - locals.var_vbsc_dn2)))))))) / (2.0 * assign39950_e52784)))), ((locals.var_cnst0_dn4 * assign39950_e52784) + (locals.var_cnst0 * (((-locals.var_t1_dn4) + ((locals.var_cnst1_dn4 * assign39950_e52781) + (locals.var_cnst1 * ((assign39950_e52773 * (((-locals.var_beta_dn4) * assign39950_e52771) + (assign39950_e52768 * (locals.var_phi_sl_dep__blk858_dn4 - locals.var_vbsc_dn4)))) - (assign39950_e52780 * (((-locals.var_beta_dn4) * assign39950_e52778) + (assign39950_e52775 * (locals.var_phi_bl_dep__blk861_dn4 - locals.var_vbsc_dn4)))))))) / (2.0 * assign39950_e52784)))), ((locals.var_cnst0_dn5 * assign39950_e52784) + (locals.var_cnst0 * (((-locals.var_t1_dn5) + ((locals.var_cnst1_dn5 * assign39950_e52781) + (locals.var_cnst1 * ((assign39950_e52773 * (((-locals.var_beta_dn5) * assign39950_e52771) + (assign39950_e52768 * (locals.var_phi_sl_dep__blk858_dn5 - locals.var_vbsc_dn5)))) - (assign39950_e52780 * (((-locals.var_beta_dn5) * assign39950_e52778) + (assign39950_e52775 * (locals.var_phi_bl_dep__blk861_dn5 - locals.var_vbsc_dn5)))))))) / (2.0 * assign39950_e52784)))), ((locals.var_cnst0_dn6 * assign39950_e52784) + (locals.var_cnst0 * (((-locals.var_t1_dn6) + ((locals.var_cnst1_dn6 * assign39950_e52781) + (locals.var_cnst1 * ((assign39950_e52773 * (((-locals.var_beta_dn6) * assign39950_e52771) + (assign39950_e52768 * (locals.var_phi_sl_dep__blk858_dn6 - locals.var_vbsc_dn6)))) - (assign39950_e52780 * (((-locals.var_beta_dn6) * assign39950_e52778) + (assign39950_e52775 * (locals.var_phi_bl_dep__blk861_dn6 - locals.var_vbsc_dn6)))))))) / (2.0 * assign39950_e52784)))), ((locals.var_cnst0_dn7 * assign39950_e52784) + (locals.var_cnst0 * (((-locals.var_t1_dn7) + ((locals.var_cnst1_dn7 * assign39950_e52781) + (locals.var_cnst1 * ((assign39950_e52773 * (((-locals.var_beta_dn7) * assign39950_e52771) + (assign39950_e52768 * (locals.var_phi_sl_dep__blk858_dn7 - locals.var_vbsc_dn7)))) - (assign39950_e52780 * (((-locals.var_beta_dn7) * assign39950_e52778) + (assign39950_e52775 * (locals.var_phi_bl_dep__blk861_dn7 - locals.var_vbsc_dn7)))))))) / (2.0 * assign39950_e52784)))), ((locals.var_cnst0_dn8 * assign39950_e52784) + (locals.var_cnst0 * (((-locals.var_t1_dn8) + ((locals.var_cnst1_dn8 * assign39950_e52781) + (locals.var_cnst1 * ((assign39950_e52773 * (((-locals.var_beta_dn8) * assign39950_e52771) + (assign39950_e52768 * (locals.var_phi_sl_dep__blk858_dn8 - locals.var_vbsc_dn8)))) - (assign39950_e52780 * (((-locals.var_beta_dn8) * assign39950_e52778) + (assign39950_e52775 * (locals.var_phi_bl_dep__blk861_dn8 - locals.var_vbsc_dn8)))))))) / (2.0 * assign39950_e52784)))), ((locals.var_cnst0_dn9 * assign39950_e52784) + (locals.var_cnst0 * (((-locals.var_t1_dn9) + ((locals.var_cnst1_dn9 * assign39950_e52781) + (locals.var_cnst1 * ((assign39950_e52773 * (((-locals.var_beta_dn9) * assign39950_e52771) + (assign39950_e52768 * (locals.var_phi_sl_dep__blk858_dn9 - locals.var_vbsc_dn9)))) - (assign39950_e52780 * (((-locals.var_beta_dn9) * assign39950_e52778) + (assign39950_e52775 * (locals.var_phi_bl_dep__blk861_dn9 - locals.var_vbsc_dn9)))))))) / (2.0 * assign39950_e52784)))), ((locals.var_cnst0_dn10 * assign39950_e52784) + (locals.var_cnst0 * (((-locals.var_t1_dn10) + ((locals.var_cnst1_dn10 * assign39950_e52781) + (locals.var_cnst1 * ((assign39950_e52773 * (((-locals.var_beta_dn10) * assign39950_e52771) + (assign39950_e52768 * (locals.var_phi_sl_dep__blk858_dn10 - locals.var_vbsc_dn10)))) - (assign39950_e52780 * (((-locals.var_beta_dn10) * assign39950_e52778) + (assign39950_e52775 * (locals.var_phi_bl_dep__blk861_dn10 - locals.var_vbsc_dn10)))))))) / (2.0 * assign39950_e52784)))), ((locals.var_cnst0_dn11 * assign39950_e52784) + (locals.var_cnst0 * (((-locals.var_t1_dn11) + ((locals.var_cnst1_dn11 * assign39950_e52781) + (locals.var_cnst1 * ((assign39950_e52773 * (((-locals.var_beta_dn11) * assign39950_e52771) + (assign39950_e52768 * (locals.var_phi_sl_dep__blk858_dn11 - locals.var_vbsc_dn11)))) - (assign39950_e52780 * (((-locals.var_beta_dn11) * assign39950_e52778) + (assign39950_e52775 * (locals.var_phi_bl_dep__blk861_dn11 - locals.var_vbsc_dn11)))))))) / (2.0 * assign39950_e52784)))), ((locals.var_cnst0_dn14 * assign39950_e52784) + (locals.var_cnst0 * (((-locals.var_t1_dn14) + ((locals.var_cnst1_dn14 * assign39950_e52781) + (locals.var_cnst1 * ((assign39950_e52773 * (((-locals.var_beta_dn14) * assign39950_e52771) + (assign39950_e52768 * (locals.var_phi_sl_dep__blk858_dn14 - locals.var_vbsc_dn14)))) - (assign39950_e52780 * (((-locals.var_beta_dn14) * assign39950_e52778) + (assign39950_e52775 * (locals.var_phi_bl_dep__blk861_dn14 - locals.var_vbsc_dn14)))))))) / (2.0 * assign39950_e52784)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign39950_e52787;
        locals.var_t3_dn0 = assign39950_e52787_d_n0;
        locals.var_t3_dn2 = assign39950_e52787_d_n2;
        locals.var_t3_dn4 = assign39950_e52787_d_n4;
        locals.var_t3_dn5 = assign39950_e52787_d_n5;
        locals.var_t3_dn6 = assign39950_e52787_d_n6;
        locals.var_t3_dn7 = assign39950_e52787_d_n7;
        locals.var_t3_dn8 = assign39950_e52787_d_n8;
        locals.var_t3_dn9 = assign39950_e52787_d_n9;
        locals.var_t3_dn10 = assign39950_e52787_d_n10;
        locals.var_t3_dn11 = assign39950_e52787_d_n11;
        locals.var_t3_dn14 = assign39950_e52787_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign39960_e52811, assign39960_e52811_d_n0, assign39960_e52811_d_n2, assign39960_e52811_d_n4, assign39960_e52811_d_n5, assign39960_e52811_d_n6, assign39960_e52811_d_n7, assign39960_e52811_d_n8, assign39960_e52811_d_n9, assign39960_e52811_d_n10, assign39960_e52811_d_n11, assign39960_e52811_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) && (locals.var_guard1009 == 0.0)) && (locals.var_guard1010 == 0.0)) {
        let assign39960_e52806: f64 = (-locals.var_t1);
        let assign39960_e52807: f64 = (assign39960_e52806).sqrt();
        let assign39960_e52808: f64 = (locals.var_cnst0 * assign39960_e52807);
        let assign39960_e52809: f64 = (locals.var_t3 - assign39960_e52808);
        (assign39960_e52809, (locals.var_t3_dn0 - ((locals.var_cnst0_dn0 * assign39960_e52807) + (locals.var_cnst0 * ((-locals.var_t1_dn0) / (2.0 * assign39960_e52807))))), (locals.var_t3_dn2 - ((locals.var_cnst0_dn2 * assign39960_e52807) + (locals.var_cnst0 * ((-locals.var_t1_dn2) / (2.0 * assign39960_e52807))))), (locals.var_t3_dn4 - ((locals.var_cnst0_dn4 * assign39960_e52807) + (locals.var_cnst0 * ((-locals.var_t1_dn4) / (2.0 * assign39960_e52807))))), (locals.var_t3_dn5 - ((locals.var_cnst0_dn5 * assign39960_e52807) + (locals.var_cnst0 * ((-locals.var_t1_dn5) / (2.0 * assign39960_e52807))))), (locals.var_t3_dn6 - ((locals.var_cnst0_dn6 * assign39960_e52807) + (locals.var_cnst0 * ((-locals.var_t1_dn6) / (2.0 * assign39960_e52807))))), (locals.var_t3_dn7 - ((locals.var_cnst0_dn7 * assign39960_e52807) + (locals.var_cnst0 * ((-locals.var_t1_dn7) / (2.0 * assign39960_e52807))))), (locals.var_t3_dn8 - ((locals.var_cnst0_dn8 * assign39960_e52807) + (locals.var_cnst0 * ((-locals.var_t1_dn8) / (2.0 * assign39960_e52807))))), (locals.var_t3_dn9 - ((locals.var_cnst0_dn9 * assign39960_e52807) + (locals.var_cnst0 * ((-locals.var_t1_dn9) / (2.0 * assign39960_e52807))))), (locals.var_t3_dn10 - ((locals.var_cnst0_dn10 * assign39960_e52807) + (locals.var_cnst0 * ((-locals.var_t1_dn10) / (2.0 * assign39960_e52807))))), (locals.var_t3_dn11 - ((locals.var_cnst0_dn11 * assign39960_e52807) + (locals.var_cnst0 * ((-locals.var_t1_dn11) / (2.0 * assign39960_e52807))))), (locals.var_t3_dn14 - ((locals.var_cnst0_dn14 * assign39960_e52807) + (locals.var_cnst0 * ((-locals.var_t1_dn14) / (2.0 * assign39960_e52807))))),)
    } else {
        (locals.var_q_subl__blk869, locals.var_q_subl__blk869_dn0, locals.var_q_subl__blk869_dn2, locals.var_q_subl__blk869_dn4, locals.var_q_subl__blk869_dn5, locals.var_q_subl__blk869_dn6, locals.var_q_subl__blk869_dn7, locals.var_q_subl__blk869_dn8, locals.var_q_subl__blk869_dn9, locals.var_q_subl__blk869_dn10, locals.var_q_subl__blk869_dn11, locals.var_q_subl__blk869_dn14,)
    }
};
        locals.var_q_subl__blk869 = assign39960_e52811;
        locals.var_q_subl__blk869_dn0 = assign39960_e52811_d_n0;
        locals.var_q_subl__blk869_dn2 = assign39960_e52811_d_n2;
        locals.var_q_subl__blk869_dn4 = assign39960_e52811_d_n4;
        locals.var_q_subl__blk869_dn5 = assign39960_e52811_d_n5;
        locals.var_q_subl__blk869_dn6 = assign39960_e52811_d_n6;
        locals.var_q_subl__blk869_dn7 = assign39960_e52811_d_n7;
        locals.var_q_subl__blk869_dn8 = assign39960_e52811_d_n8;
        locals.var_q_subl__blk869_dn9 = assign39960_e52811_d_n9;
        locals.var_q_subl__blk869_dn10 = assign39960_e52811_d_n10;
        locals.var_q_subl__blk869_dn11 = assign39960_e52811_d_n11;
        locals.var_q_subl__blk869_dn14 = assign39960_e52811_d_n14;
        locals.var_q_subl__blk869_rv = 0.0;

        let assign39970_e52814: f64 = (locals.var_phi_sl_dep__blk858 - locals.var_vds_maxbl__blk856);
        let assign39970_e52817: f64 = locals.var_ps_delta;
        let assign39970_e52822: f64 = if ((assign39970_e52814 < assign39970_e52817) && (locals.var_ps_delta >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1011 = assign39970_e52822;
        locals.var_guard1011_rv = 0.0;

        let (assign39980_e52842, assign39980_e52842_d_n0, assign39980_e52842_d_n2, assign39980_e52842_d_n4, assign39980_e52842_d_n5, assign39980_e52842_d_n6, assign39980_e52842_d_n7, assign39980_e52842_d_n8, assign39980_e52842_d_n9, assign39980_e52842_d_n10, assign39980_e52842_d_n11, assign39980_e52842_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign39980_e52836: f64 = locals.var_ps_delta;
        let assign39980_e52839: f64 = (locals.var_phi_sl_dep__blk858 - locals.var_vds_maxbl__blk856);
        let assign39980_e52840: f64 = (assign39980_e52836 - assign39980_e52839);
        (assign39980_e52840, (-(locals.var_phi_sl_dep__blk858_dn0 - locals.var_vds_maxbl__blk856_dn0)), (-(locals.var_phi_sl_dep__blk858_dn2 - locals.var_vds_maxbl__blk856_dn2)), (-(locals.var_phi_sl_dep__blk858_dn4 - locals.var_vds_maxbl__blk856_dn4)), (-(locals.var_phi_sl_dep__blk858_dn5 - locals.var_vds_maxbl__blk856_dn5)), (-(locals.var_phi_sl_dep__blk858_dn6 - locals.var_vds_maxbl__blk856_dn6)), (-(locals.var_phi_sl_dep__blk858_dn7 - locals.var_vds_maxbl__blk856_dn7)), (-(locals.var_phi_sl_dep__blk858_dn8 - locals.var_vds_maxbl__blk856_dn8)), (-(locals.var_phi_sl_dep__blk858_dn9 - locals.var_vds_maxbl__blk856_dn9)), (-(locals.var_phi_sl_dep__blk858_dn10 - locals.var_vds_maxbl__blk856_dn10)), (-(locals.var_phi_sl_dep__blk858_dn11 - locals.var_vds_maxbl__blk856_dn11)), (-(locals.var_phi_sl_dep__blk858_dn14 - locals.var_vds_maxbl__blk856_dn14)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign39980_e52842;
        locals.var_tmf1_dn0 = assign39980_e52842_d_n0;
        locals.var_tmf1_dn2 = assign39980_e52842_d_n2;
        locals.var_tmf1_dn4 = assign39980_e52842_d_n4;
        locals.var_tmf1_dn5 = assign39980_e52842_d_n5;
        locals.var_tmf1_dn6 = assign39980_e52842_d_n6;
        locals.var_tmf1_dn7 = assign39980_e52842_d_n7;
        locals.var_tmf1_dn8 = assign39980_e52842_d_n8;
        locals.var_tmf1_dn9 = assign39980_e52842_d_n9;
        locals.var_tmf1_dn10 = assign39980_e52842_d_n10;
        locals.var_tmf1_dn11 = assign39980_e52842_d_n11;
        locals.var_tmf1_dn14 = assign39980_e52842_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign39990_e52858, assign39990_e52858_d_n0, assign39990_e52858_d_n2, assign39990_e52858_d_n4, assign39990_e52858_d_n5, assign39990_e52858_d_n6, assign39990_e52858_d_n7, assign39990_e52858_d_n8, assign39990_e52858_d_n9, assign39990_e52858_d_n10, assign39990_e52858_d_n11, assign39990_e52858_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign39990_e52856: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign39990_e52856, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign39990_e52858;
        locals.var_x2_dn0 = assign39990_e52858_d_n0;
        locals.var_x2_dn2 = assign39990_e52858_d_n2;
        locals.var_x2_dn4 = assign39990_e52858_d_n4;
        locals.var_x2_dn5 = assign39990_e52858_d_n5;
        locals.var_x2_dn6 = assign39990_e52858_d_n6;
        locals.var_x2_dn7 = assign39990_e52858_d_n7;
        locals.var_x2_dn8 = assign39990_e52858_d_n8;
        locals.var_x2_dn9 = assign39990_e52858_d_n9;
        locals.var_x2_dn10 = assign39990_e52858_d_n10;
        locals.var_x2_dn11 = assign39990_e52858_d_n11;
        locals.var_x2_dn14 = assign39990_e52858_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign40000_e52874, assign40000_e52874_d_n0, assign40000_e52874_d_n2, assign40000_e52874_d_n4, assign40000_e52874_d_n5, assign40000_e52874_d_n6, assign40000_e52874_d_n7, assign40000_e52874_d_n8, assign40000_e52874_d_n9, assign40000_e52874_d_n10, assign40000_e52874_d_n11, assign40000_e52874_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign40000_e52872: f64 = (locals.var_ps_delta * locals.var_ps_delta);
        (assign40000_e52872, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign40000_e52874;
        locals.var_xmax2_dn0 = assign40000_e52874_d_n0;
        locals.var_xmax2_dn2 = assign40000_e52874_d_n2;
        locals.var_xmax2_dn4 = assign40000_e52874_d_n4;
        locals.var_xmax2_dn5 = assign40000_e52874_d_n5;
        locals.var_xmax2_dn6 = assign40000_e52874_d_n6;
        locals.var_xmax2_dn7 = assign40000_e52874_d_n7;
        locals.var_xmax2_dn8 = assign40000_e52874_d_n8;
        locals.var_xmax2_dn9 = assign40000_e52874_d_n9;
        locals.var_xmax2_dn10 = assign40000_e52874_d_n10;
        locals.var_xmax2_dn11 = assign40000_e52874_d_n11;
        locals.var_xmax2_dn14 = assign40000_e52874_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign40010_e52888, assign40010_e52888_d_n0, assign40010_e52888_d_n2, assign40010_e52888_d_n4, assign40010_e52888_d_n5, assign40010_e52888_d_n6, assign40010_e52888_d_n7, assign40010_e52888_d_n8, assign40010_e52888_d_n9, assign40010_e52888_d_n10, assign40010_e52888_d_n11, assign40010_e52888_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign40010_e52888;
        locals.var_xp_dn0 = assign40010_e52888_d_n0;
        locals.var_xp_dn2 = assign40010_e52888_d_n2;
        locals.var_xp_dn4 = assign40010_e52888_d_n4;
        locals.var_xp_dn5 = assign40010_e52888_d_n5;
        locals.var_xp_dn6 = assign40010_e52888_d_n6;
        locals.var_xp_dn7 = assign40010_e52888_d_n7;
        locals.var_xp_dn8 = assign40010_e52888_d_n8;
        locals.var_xp_dn9 = assign40010_e52888_d_n9;
        locals.var_xp_dn10 = assign40010_e52888_d_n10;
        locals.var_xp_dn11 = assign40010_e52888_d_n11;
        locals.var_xp_dn14 = assign40010_e52888_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign40020_e52902, assign40020_e52902_d_n0, assign40020_e52902_d_n2, assign40020_e52902_d_n4, assign40020_e52902_d_n5, assign40020_e52902_d_n6, assign40020_e52902_d_n7, assign40020_e52902_d_n8, assign40020_e52902_d_n9, assign40020_e52902_d_n10, assign40020_e52902_d_n11, assign40020_e52902_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign40020_e52902;
        locals.var_xmp_dn0 = assign40020_e52902_d_n0;
        locals.var_xmp_dn2 = assign40020_e52902_d_n2;
        locals.var_xmp_dn4 = assign40020_e52902_d_n4;
        locals.var_xmp_dn5 = assign40020_e52902_d_n5;
        locals.var_xmp_dn6 = assign40020_e52902_d_n6;
        locals.var_xmp_dn7 = assign40020_e52902_d_n7;
        locals.var_xmp_dn8 = assign40020_e52902_d_n8;
        locals.var_xmp_dn9 = assign40020_e52902_d_n9;
        locals.var_xmp_dn10 = assign40020_e52902_d_n10;
        locals.var_xmp_dn11 = assign40020_e52902_d_n11;
        locals.var_xmp_dn14 = assign40020_e52902_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign40030_e52916,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign40030_e52916;
        locals.var_m0_rv = 0.0;

        let (assign40040_e52930,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign40040_e52930;
        locals.var_mm_rv = 0.0;

        let (assign40050_e52944, assign40050_e52944_d_n0, assign40050_e52944_d_n2, assign40050_e52944_d_n4, assign40050_e52944_d_n5, assign40050_e52944_d_n6, assign40050_e52944_d_n7, assign40050_e52944_d_n8, assign40050_e52944_d_n9, assign40050_e52944_d_n10, assign40050_e52944_d_n11, assign40050_e52944_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign40050_e52944;
        locals.var_arg_dn0 = assign40050_e52944_d_n0;
        locals.var_arg_dn2 = assign40050_e52944_d_n2;
        locals.var_arg_dn4 = assign40050_e52944_d_n4;
        locals.var_arg_dn5 = assign40050_e52944_d_n5;
        locals.var_arg_dn6 = assign40050_e52944_d_n6;
        locals.var_arg_dn7 = assign40050_e52944_d_n7;
        locals.var_arg_dn8 = assign40050_e52944_d_n8;
        locals.var_arg_dn9 = assign40050_e52944_d_n9;
        locals.var_arg_dn10 = assign40050_e52944_d_n10;
        locals.var_arg_dn11 = assign40050_e52944_d_n11;
        locals.var_arg_dn14 = assign40050_e52944_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign40060_e52958, assign40060_e52958_d_n0, assign40060_e52958_d_n2, assign40060_e52958_d_n4, assign40060_e52958_d_n5, assign40060_e52958_d_n6, assign40060_e52958_d_n7, assign40060_e52958_d_n8, assign40060_e52958_d_n9, assign40060_e52958_d_n10, assign40060_e52958_d_n11, assign40060_e52958_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign40060_e52958;
        locals.var_dnm_dn0 = assign40060_e52958_d_n0;
        locals.var_dnm_dn2 = assign40060_e52958_d_n2;
        locals.var_dnm_dn4 = assign40060_e52958_d_n4;
        locals.var_dnm_dn5 = assign40060_e52958_d_n5;
        locals.var_dnm_dn6 = assign40060_e52958_d_n6;
        locals.var_dnm_dn7 = assign40060_e52958_d_n7;
        locals.var_dnm_dn8 = assign40060_e52958_d_n8;
        locals.var_dnm_dn9 = assign40060_e52958_d_n9;
        locals.var_dnm_dn10 = assign40060_e52958_d_n10;
        locals.var_dnm_dn11 = assign40060_e52958_d_n11;
        locals.var_dnm_dn14 = assign40060_e52958_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign40070_e52974, assign40070_e52974_d_n0, assign40070_e52974_d_n2, assign40070_e52974_d_n4, assign40070_e52974_d_n5, assign40070_e52974_d_n6, assign40070_e52974_d_n7, assign40070_e52974_d_n8, assign40070_e52974_d_n9, assign40070_e52974_d_n10, assign40070_e52974_d_n11, assign40070_e52974_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign40070_e52972: f64 = (locals.var_xp * locals.var_x2);
        (assign40070_e52972, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign40070_e52974;
        locals.var_xp_dn0 = assign40070_e52974_d_n0;
        locals.var_xp_dn2 = assign40070_e52974_d_n2;
        locals.var_xp_dn4 = assign40070_e52974_d_n4;
        locals.var_xp_dn5 = assign40070_e52974_d_n5;
        locals.var_xp_dn6 = assign40070_e52974_d_n6;
        locals.var_xp_dn7 = assign40070_e52974_d_n7;
        locals.var_xp_dn8 = assign40070_e52974_d_n8;
        locals.var_xp_dn9 = assign40070_e52974_d_n9;
        locals.var_xp_dn10 = assign40070_e52974_d_n10;
        locals.var_xp_dn11 = assign40070_e52974_d_n11;
        locals.var_xp_dn14 = assign40070_e52974_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign40080_e52990, assign40080_e52990_d_n0, assign40080_e52990_d_n2, assign40080_e52990_d_n4, assign40080_e52990_d_n5, assign40080_e52990_d_n6, assign40080_e52990_d_n7, assign40080_e52990_d_n8, assign40080_e52990_d_n9, assign40080_e52990_d_n10, assign40080_e52990_d_n11, assign40080_e52990_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign40080_e52988: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign40080_e52988, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign40080_e52990;
        locals.var_xmp_dn0 = assign40080_e52990_d_n0;
        locals.var_xmp_dn2 = assign40080_e52990_d_n2;
        locals.var_xmp_dn4 = assign40080_e52990_d_n4;
        locals.var_xmp_dn5 = assign40080_e52990_d_n5;
        locals.var_xmp_dn6 = assign40080_e52990_d_n6;
        locals.var_xmp_dn7 = assign40080_e52990_d_n7;
        locals.var_xmp_dn8 = assign40080_e52990_d_n8;
        locals.var_xmp_dn9 = assign40080_e52990_d_n9;
        locals.var_xmp_dn10 = assign40080_e52990_d_n10;
        locals.var_xmp_dn11 = assign40080_e52990_d_n11;
        locals.var_xmp_dn14 = assign40080_e52990_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign40090_e53006, assign40090_e53006_d_n0, assign40090_e53006_d_n2, assign40090_e53006_d_n4, assign40090_e53006_d_n5, assign40090_e53006_d_n6, assign40090_e53006_d_n7, assign40090_e53006_d_n8, assign40090_e53006_d_n9, assign40090_e53006_d_n10, assign40090_e53006_d_n11, assign40090_e53006_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign40090_e53004: f64 = (locals.var_xp * locals.var_x2);
        (assign40090_e53004, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign40090_e53006;
        locals.var_xp_dn0 = assign40090_e53006_d_n0;
        locals.var_xp_dn2 = assign40090_e53006_d_n2;
        locals.var_xp_dn4 = assign40090_e53006_d_n4;
        locals.var_xp_dn5 = assign40090_e53006_d_n5;
        locals.var_xp_dn6 = assign40090_e53006_d_n6;
        locals.var_xp_dn7 = assign40090_e53006_d_n7;
        locals.var_xp_dn8 = assign40090_e53006_d_n8;
        locals.var_xp_dn9 = assign40090_e53006_d_n9;
        locals.var_xp_dn10 = assign40090_e53006_d_n10;
        locals.var_xp_dn11 = assign40090_e53006_d_n11;
        locals.var_xp_dn14 = assign40090_e53006_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign40100_e53022, assign40100_e53022_d_n0, assign40100_e53022_d_n2, assign40100_e53022_d_n4, assign40100_e53022_d_n5, assign40100_e53022_d_n6, assign40100_e53022_d_n7, assign40100_e53022_d_n8, assign40100_e53022_d_n9, assign40100_e53022_d_n10, assign40100_e53022_d_n11, assign40100_e53022_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign40100_e53020: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign40100_e53020, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign40100_e53022;
        locals.var_xmp_dn0 = assign40100_e53022_d_n0;
        locals.var_xmp_dn2 = assign40100_e53022_d_n2;
        locals.var_xmp_dn4 = assign40100_e53022_d_n4;
        locals.var_xmp_dn5 = assign40100_e53022_d_n5;
        locals.var_xmp_dn6 = assign40100_e53022_d_n6;
        locals.var_xmp_dn7 = assign40100_e53022_d_n7;
        locals.var_xmp_dn8 = assign40100_e53022_d_n8;
        locals.var_xmp_dn9 = assign40100_e53022_d_n9;
        locals.var_xmp_dn10 = assign40100_e53022_d_n10;
        locals.var_xmp_dn11 = assign40100_e53022_d_n11;
        locals.var_xmp_dn14 = assign40100_e53022_d_n14;
        locals.var_xmp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_137(
        locals: &mut StampLocals,
    ) {
        let (assign40110_e53038, assign40110_e53038_d_n0, assign40110_e53038_d_n2, assign40110_e53038_d_n4, assign40110_e53038_d_n5, assign40110_e53038_d_n6, assign40110_e53038_d_n7, assign40110_e53038_d_n8, assign40110_e53038_d_n9, assign40110_e53038_d_n10, assign40110_e53038_d_n11, assign40110_e53038_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign40110_e53036: f64 = (locals.var_xp * locals.var_x2);
        (assign40110_e53036, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign40110_e53038;
        locals.var_xp_dn0 = assign40110_e53038_d_n0;
        locals.var_xp_dn2 = assign40110_e53038_d_n2;
        locals.var_xp_dn4 = assign40110_e53038_d_n4;
        locals.var_xp_dn5 = assign40110_e53038_d_n5;
        locals.var_xp_dn6 = assign40110_e53038_d_n6;
        locals.var_xp_dn7 = assign40110_e53038_d_n7;
        locals.var_xp_dn8 = assign40110_e53038_d_n8;
        locals.var_xp_dn9 = assign40110_e53038_d_n9;
        locals.var_xp_dn10 = assign40110_e53038_d_n10;
        locals.var_xp_dn11 = assign40110_e53038_d_n11;
        locals.var_xp_dn14 = assign40110_e53038_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign40120_e53054, assign40120_e53054_d_n0, assign40120_e53054_d_n2, assign40120_e53054_d_n4, assign40120_e53054_d_n5, assign40120_e53054_d_n6, assign40120_e53054_d_n7, assign40120_e53054_d_n8, assign40120_e53054_d_n9, assign40120_e53054_d_n10, assign40120_e53054_d_n11, assign40120_e53054_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign40120_e53052: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign40120_e53052, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign40120_e53054;
        locals.var_xmp_dn0 = assign40120_e53054_d_n0;
        locals.var_xmp_dn2 = assign40120_e53054_d_n2;
        locals.var_xmp_dn4 = assign40120_e53054_d_n4;
        locals.var_xmp_dn5 = assign40120_e53054_d_n5;
        locals.var_xmp_dn6 = assign40120_e53054_d_n6;
        locals.var_xmp_dn7 = assign40120_e53054_d_n7;
        locals.var_xmp_dn8 = assign40120_e53054_d_n8;
        locals.var_xmp_dn9 = assign40120_e53054_d_n9;
        locals.var_xmp_dn10 = assign40120_e53054_d_n10;
        locals.var_xmp_dn11 = assign40120_e53054_d_n11;
        locals.var_xmp_dn14 = assign40120_e53054_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign40130_e53070, assign40130_e53070_d_n0, assign40130_e53070_d_n2, assign40130_e53070_d_n4, assign40130_e53070_d_n5, assign40130_e53070_d_n6, assign40130_e53070_d_n7, assign40130_e53070_d_n8, assign40130_e53070_d_n9, assign40130_e53070_d_n10, assign40130_e53070_d_n11, assign40130_e53070_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign40130_e53068: f64 = (locals.var_xp * locals.var_x2);
        (assign40130_e53068, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign40130_e53070;
        locals.var_xp_dn0 = assign40130_e53070_d_n0;
        locals.var_xp_dn2 = assign40130_e53070_d_n2;
        locals.var_xp_dn4 = assign40130_e53070_d_n4;
        locals.var_xp_dn5 = assign40130_e53070_d_n5;
        locals.var_xp_dn6 = assign40130_e53070_d_n6;
        locals.var_xp_dn7 = assign40130_e53070_d_n7;
        locals.var_xp_dn8 = assign40130_e53070_d_n8;
        locals.var_xp_dn9 = assign40130_e53070_d_n9;
        locals.var_xp_dn10 = assign40130_e53070_d_n10;
        locals.var_xp_dn11 = assign40130_e53070_d_n11;
        locals.var_xp_dn14 = assign40130_e53070_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign40140_e53086, assign40140_e53086_d_n0, assign40140_e53086_d_n2, assign40140_e53086_d_n4, assign40140_e53086_d_n5, assign40140_e53086_d_n6, assign40140_e53086_d_n7, assign40140_e53086_d_n8, assign40140_e53086_d_n9, assign40140_e53086_d_n10, assign40140_e53086_d_n11, assign40140_e53086_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign40140_e53084: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign40140_e53084, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign40140_e53086;
        locals.var_xmp_dn0 = assign40140_e53086_d_n0;
        locals.var_xmp_dn2 = assign40140_e53086_d_n2;
        locals.var_xmp_dn4 = assign40140_e53086_d_n4;
        locals.var_xmp_dn5 = assign40140_e53086_d_n5;
        locals.var_xmp_dn6 = assign40140_e53086_d_n6;
        locals.var_xmp_dn7 = assign40140_e53086_d_n7;
        locals.var_xmp_dn8 = assign40140_e53086_d_n8;
        locals.var_xmp_dn9 = assign40140_e53086_d_n9;
        locals.var_xmp_dn10 = assign40140_e53086_d_n10;
        locals.var_xmp_dn11 = assign40140_e53086_d_n11;
        locals.var_xmp_dn14 = assign40140_e53086_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign40150_e53102, assign40150_e53102_d_n0, assign40150_e53102_d_n2, assign40150_e53102_d_n4, assign40150_e53102_d_n5, assign40150_e53102_d_n6, assign40150_e53102_d_n7, assign40150_e53102_d_n8, assign40150_e53102_d_n9, assign40150_e53102_d_n10, assign40150_e53102_d_n11, assign40150_e53102_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign40150_e53100: f64 = (locals.var_xp + locals.var_xmp);
        (assign40150_e53100, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign40150_e53102;
        locals.var_arg_dn0 = assign40150_e53102_d_n0;
        locals.var_arg_dn2 = assign40150_e53102_d_n2;
        locals.var_arg_dn4 = assign40150_e53102_d_n4;
        locals.var_arg_dn5 = assign40150_e53102_d_n5;
        locals.var_arg_dn6 = assign40150_e53102_d_n6;
        locals.var_arg_dn7 = assign40150_e53102_d_n7;
        locals.var_arg_dn8 = assign40150_e53102_d_n8;
        locals.var_arg_dn9 = assign40150_e53102_d_n9;
        locals.var_arg_dn10 = assign40150_e53102_d_n10;
        locals.var_arg_dn11 = assign40150_e53102_d_n11;
        locals.var_arg_dn14 = assign40150_e53102_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign40160_e53116, assign40160_e53116_d_n0, assign40160_e53116_d_n2, assign40160_e53116_d_n4, assign40160_e53116_d_n5, assign40160_e53116_d_n6, assign40160_e53116_d_n7, assign40160_e53116_d_n8, assign40160_e53116_d_n9, assign40160_e53116_d_n10, assign40160_e53116_d_n11, assign40160_e53116_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign40160_e53116;
        locals.var_dnm_dn0 = assign40160_e53116_d_n0;
        locals.var_dnm_dn2 = assign40160_e53116_d_n2;
        locals.var_dnm_dn4 = assign40160_e53116_d_n4;
        locals.var_dnm_dn5 = assign40160_e53116_d_n5;
        locals.var_dnm_dn6 = assign40160_e53116_d_n6;
        locals.var_dnm_dn7 = assign40160_e53116_d_n7;
        locals.var_dnm_dn8 = assign40160_e53116_d_n8;
        locals.var_dnm_dn9 = assign40160_e53116_d_n9;
        locals.var_dnm_dn10 = assign40160_e53116_d_n10;
        locals.var_dnm_dn11 = assign40160_e53116_d_n11;
        locals.var_dnm_dn14 = assign40160_e53116_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign40170_e53131: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1012 = assign40170_e53131;
        locals.var_guard1012_rv = 0.0;

        let assign40180_e53134: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1013 = assign40180_e53134;
        locals.var_guard1013_rv = 0.0;

        let (assign40190_e53152,) = {
    if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) && (locals.var_guard1011 != 0.0)) && (locals.var_guard1012 != 0.0)) && (locals.var_guard1013 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign40190_e53152;
        locals.var_mm_rv = 0.0;

        let assign40200_e53155: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1014 = assign40200_e53155;
        locals.var_guard1014_rv = 0.0;

        let (assign40210_e53176,) = {
    if (((((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) && (locals.var_guard1011 != 0.0)) && (locals.var_guard1012 != 0.0)) && (locals.var_guard1013 == 0.0)) && (locals.var_guard1014 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign40210_e53176;
        locals.var_mm_rv = 0.0;

        let assign40220_e53179: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1015 = assign40220_e53179;
        locals.var_guard1015_rv = 0.0;

        let (assign40230_e53203,) = {
    if ((((((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) && (locals.var_guard1011 != 0.0)) && (locals.var_guard1012 != 0.0)) && (locals.var_guard1013 == 0.0)) && (locals.var_guard1014 == 0.0)) && (locals.var_guard1015 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign40230_e53203;
        locals.var_mm_rv = 0.0;

        let assign40240_e53206: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1016 = assign40240_e53206;
        locals.var_guard1016_rv = 0.0;

        let (assign40250_e53233,) = {
    if (((((((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) && (locals.var_guard1011 != 0.0)) && (locals.var_guard1012 != 0.0)) && (locals.var_guard1013 == 0.0)) && (locals.var_guard1014 == 0.0)) && (locals.var_guard1015 == 0.0)) && (locals.var_guard1016 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign40250_e53233;
        locals.var_mm_rv = 0.0;

        let (assign40260_e53249,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) && (locals.var_guard1011 != 0.0)) && (locals.var_guard1012 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign40260_e53249;
        locals.var_m0_rv = 0.0;

        let mut assign40270_loop_guard: usize = 0;
        while {
            let assign40270_cond_e53266: f64 = if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) && (locals.var_guard1011 != 0.0)) && (locals.var_guard1012 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign40270_cond_e53266 != 0.0
        } {
            assign40270_loop_guard += 1;
            assert!(assign40270_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign40270_body0_e53283, assign40270_body0_e53283_d_n0, assign40270_body0_e53283_d_n2, assign40270_body0_e53283_d_n4, assign40270_body0_e53283_d_n5, assign40270_body0_e53283_d_n6, assign40270_body0_e53283_d_n7, assign40270_body0_e53283_d_n8, assign40270_body0_e53283_d_n9, assign40270_body0_e53283_d_n10, assign40270_body0_e53283_d_n11, assign40270_body0_e53283_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) && (locals.var_guard1011 != 0.0)) && (locals.var_guard1012 != 0.0)) {
        let assign40270_body0_e53281: f64 = (locals.var_dnm).sqrt();
        (assign40270_body0_e53281, (locals.var_dnm_dn0 / (2.0 * assign40270_body0_e53281)), (locals.var_dnm_dn2 / (2.0 * assign40270_body0_e53281)), (locals.var_dnm_dn4 / (2.0 * assign40270_body0_e53281)), (locals.var_dnm_dn5 / (2.0 * assign40270_body0_e53281)), (locals.var_dnm_dn6 / (2.0 * assign40270_body0_e53281)), (locals.var_dnm_dn7 / (2.0 * assign40270_body0_e53281)), (locals.var_dnm_dn8 / (2.0 * assign40270_body0_e53281)), (locals.var_dnm_dn9 / (2.0 * assign40270_body0_e53281)), (locals.var_dnm_dn10 / (2.0 * assign40270_body0_e53281)), (locals.var_dnm_dn11 / (2.0 * assign40270_body0_e53281)), (locals.var_dnm_dn14 / (2.0 * assign40270_body0_e53281)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign40270_body0_e53283;
            locals.var_dnm_dn0 = assign40270_body0_e53283_d_n0;
            locals.var_dnm_dn2 = assign40270_body0_e53283_d_n2;
            locals.var_dnm_dn4 = assign40270_body0_e53283_d_n4;
            locals.var_dnm_dn5 = assign40270_body0_e53283_d_n5;
            locals.var_dnm_dn6 = assign40270_body0_e53283_d_n6;
            locals.var_dnm_dn7 = assign40270_body0_e53283_d_n7;
            locals.var_dnm_dn8 = assign40270_body0_e53283_d_n8;
            locals.var_dnm_dn9 = assign40270_body0_e53283_d_n9;
            locals.var_dnm_dn10 = assign40270_body0_e53283_d_n10;
            locals.var_dnm_dn11 = assign40270_body0_e53283_d_n11;
            locals.var_dnm_dn14 = assign40270_body0_e53283_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign40270_body1_e53301,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) && (locals.var_guard1011 != 0.0)) && (locals.var_guard1012 != 0.0)) {
        let assign40270_body1_e53299: f64 = (locals.var_m0 + 1.0);
        (assign40270_body1_e53299,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign40270_body1_e53301;
            locals.var_m0_rv = 0.0;
        }

        let (assign40280_e53329, assign40280_e53329_d_n0, assign40280_e53329_d_n2, assign40280_e53329_d_n4, assign40280_e53329_d_n5, assign40280_e53329_d_n6, assign40280_e53329_d_n7, assign40280_e53329_d_n8, assign40280_e53329_d_n9, assign40280_e53329_d_n10, assign40280_e53329_d_n11, assign40280_e53329_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) && (locals.var_guard1011 != 0.0)) && (locals.var_guard1012 == 0.0)) {
        let (assign40280_e53327, assign40280_e53327_d_n0, assign40280_e53327_d_n2, assign40280_e53327_d_n4, assign40280_e53327_d_n5, assign40280_e53327_d_n6, assign40280_e53327_d_n7, assign40280_e53327_d_n8, assign40280_e53327_d_n9, assign40280_e53327_d_n10, assign40280_e53327_d_n11, assign40280_e53327_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign40280_e53324: f64 = (2.0 * 4.0);
                let assign40280_e53325: f64 = (1.0 / assign40280_e53324);
                let assign40280_e53326: f64 = (locals.var_dnm).powf(assign40280_e53325);
                (assign40280_e53326, if 0.0 == 0.0 && ((assign40280_e53325) as f64).is_finite() && ((assign40280_e53325) as f64).fract() == 0.0 { if assign40280_e53325 == 0.0 { 0.0 } else { (assign40280_e53325 * ((locals.var_dnm).powf(assign40280_e53325 - 1.0) * locals.var_dnm_dn0)) } } else { (assign40280_e53326 * (assign40280_e53325 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign40280_e53325) as f64).is_finite() && ((assign40280_e53325) as f64).fract() == 0.0 { if assign40280_e53325 == 0.0 { 0.0 } else { (assign40280_e53325 * ((locals.var_dnm).powf(assign40280_e53325 - 1.0) * locals.var_dnm_dn2)) } } else { (assign40280_e53326 * (assign40280_e53325 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign40280_e53325) as f64).is_finite() && ((assign40280_e53325) as f64).fract() == 0.0 { if assign40280_e53325 == 0.0 { 0.0 } else { (assign40280_e53325 * ((locals.var_dnm).powf(assign40280_e53325 - 1.0) * locals.var_dnm_dn4)) } } else { (assign40280_e53326 * (assign40280_e53325 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign40280_e53325) as f64).is_finite() && ((assign40280_e53325) as f64).fract() == 0.0 { if assign40280_e53325 == 0.0 { 0.0 } else { (assign40280_e53325 * ((locals.var_dnm).powf(assign40280_e53325 - 1.0) * locals.var_dnm_dn5)) } } else { (assign40280_e53326 * (assign40280_e53325 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign40280_e53325) as f64).is_finite() && ((assign40280_e53325) as f64).fract() == 0.0 { if assign40280_e53325 == 0.0 { 0.0 } else { (assign40280_e53325 * ((locals.var_dnm).powf(assign40280_e53325 - 1.0) * locals.var_dnm_dn6)) } } else { (assign40280_e53326 * (assign40280_e53325 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign40280_e53325) as f64).is_finite() && ((assign40280_e53325) as f64).fract() == 0.0 { if assign40280_e53325 == 0.0 { 0.0 } else { (assign40280_e53325 * ((locals.var_dnm).powf(assign40280_e53325 - 1.0) * locals.var_dnm_dn7)) } } else { (assign40280_e53326 * (assign40280_e53325 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign40280_e53325) as f64).is_finite() && ((assign40280_e53325) as f64).fract() == 0.0 { if assign40280_e53325 == 0.0 { 0.0 } else { (assign40280_e53325 * ((locals.var_dnm).powf(assign40280_e53325 - 1.0) * locals.var_dnm_dn8)) } } else { (assign40280_e53326 * (assign40280_e53325 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign40280_e53325) as f64).is_finite() && ((assign40280_e53325) as f64).fract() == 0.0 { if assign40280_e53325 == 0.0 { 0.0 } else { (assign40280_e53325 * ((locals.var_dnm).powf(assign40280_e53325 - 1.0) * locals.var_dnm_dn9)) } } else { (assign40280_e53326 * (assign40280_e53325 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign40280_e53325) as f64).is_finite() && ((assign40280_e53325) as f64).fract() == 0.0 { if assign40280_e53325 == 0.0 { 0.0 } else { (assign40280_e53325 * ((locals.var_dnm).powf(assign40280_e53325 - 1.0) * locals.var_dnm_dn10)) } } else { (assign40280_e53326 * (assign40280_e53325 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign40280_e53325) as f64).is_finite() && ((assign40280_e53325) as f64).fract() == 0.0 { if assign40280_e53325 == 0.0 { 0.0 } else { (assign40280_e53325 * ((locals.var_dnm).powf(assign40280_e53325 - 1.0) * locals.var_dnm_dn11)) } } else { (assign40280_e53326 * (assign40280_e53325 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign40280_e53325) as f64).is_finite() && ((assign40280_e53325) as f64).fract() == 0.0 { if assign40280_e53325 == 0.0 { 0.0 } else { (assign40280_e53325 * ((locals.var_dnm).powf(assign40280_e53325 - 1.0) * locals.var_dnm_dn14)) } } else { (assign40280_e53326 * (assign40280_e53325 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign40280_e53327, assign40280_e53327_d_n0, assign40280_e53327_d_n2, assign40280_e53327_d_n4, assign40280_e53327_d_n5, assign40280_e53327_d_n6, assign40280_e53327_d_n7, assign40280_e53327_d_n8, assign40280_e53327_d_n9, assign40280_e53327_d_n10, assign40280_e53327_d_n11, assign40280_e53327_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign40280_e53329;
        locals.var_dnm_dn0 = assign40280_e53329_d_n0;
        locals.var_dnm_dn2 = assign40280_e53329_d_n2;
        locals.var_dnm_dn4 = assign40280_e53329_d_n4;
        locals.var_dnm_dn5 = assign40280_e53329_d_n5;
        locals.var_dnm_dn6 = assign40280_e53329_d_n6;
        locals.var_dnm_dn7 = assign40280_e53329_d_n7;
        locals.var_dnm_dn8 = assign40280_e53329_d_n8;
        locals.var_dnm_dn9 = assign40280_e53329_d_n9;
        locals.var_dnm_dn10 = assign40280_e53329_d_n10;
        locals.var_dnm_dn11 = assign40280_e53329_d_n11;
        locals.var_dnm_dn14 = assign40280_e53329_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign40290_e53345, assign40290_e53345_d_n0, assign40290_e53345_d_n2, assign40290_e53345_d_n4, assign40290_e53345_d_n5, assign40290_e53345_d_n6, assign40290_e53345_d_n7, assign40290_e53345_d_n8, assign40290_e53345_d_n9, assign40290_e53345_d_n10, assign40290_e53345_d_n11, assign40290_e53345_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign40290_e53343: f64 = (1.0 / locals.var_dnm);
        (assign40290_e53343, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign40290_e53345;
        locals.var_dnm_dn0 = assign40290_e53345_d_n0;
        locals.var_dnm_dn2 = assign40290_e53345_d_n2;
        locals.var_dnm_dn4 = assign40290_e53345_d_n4;
        locals.var_dnm_dn5 = assign40290_e53345_d_n5;
        locals.var_dnm_dn6 = assign40290_e53345_d_n6;
        locals.var_dnm_dn7 = assign40290_e53345_d_n7;
        locals.var_dnm_dn8 = assign40290_e53345_d_n8;
        locals.var_dnm_dn9 = assign40290_e53345_d_n9;
        locals.var_dnm_dn10 = assign40290_e53345_d_n10;
        locals.var_dnm_dn11 = assign40290_e53345_d_n11;
        locals.var_dnm_dn14 = assign40290_e53345_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign40300_e53363, assign40300_e53363_d_n0, assign40300_e53363_d_n2, assign40300_e53363_d_n4, assign40300_e53363_d_n5, assign40300_e53363_d_n6, assign40300_e53363_d_n7, assign40300_e53363_d_n8, assign40300_e53363_d_n9, assign40300_e53363_d_n10, assign40300_e53363_d_n11, assign40300_e53363_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign40300_e53359: f64 = (locals.var_tmf1 * locals.var_ps_delta);
        let assign40300_e53361: f64 = (assign40300_e53359 * locals.var_dnm);
        (assign40300_e53361, (((locals.var_tmf1_dn0 * locals.var_ps_delta) * locals.var_dnm) + (assign40300_e53359 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * locals.var_ps_delta) * locals.var_dnm) + (assign40300_e53359 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * locals.var_ps_delta) * locals.var_dnm) + (assign40300_e53359 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * locals.var_ps_delta) * locals.var_dnm) + (assign40300_e53359 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * locals.var_ps_delta) * locals.var_dnm) + (assign40300_e53359 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * locals.var_ps_delta) * locals.var_dnm) + (assign40300_e53359 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * locals.var_ps_delta) * locals.var_dnm) + (assign40300_e53359 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * locals.var_ps_delta) * locals.var_dnm) + (assign40300_e53359 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * locals.var_ps_delta) * locals.var_dnm) + (assign40300_e53359 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * locals.var_ps_delta) * locals.var_dnm) + (assign40300_e53359 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * locals.var_ps_delta) * locals.var_dnm) + (assign40300_e53359 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign40300_e53363;
        locals.var_tmf0_dn0 = assign40300_e53363_d_n0;
        locals.var_tmf0_dn2 = assign40300_e53363_d_n2;
        locals.var_tmf0_dn4 = assign40300_e53363_d_n4;
        locals.var_tmf0_dn5 = assign40300_e53363_d_n5;
        locals.var_tmf0_dn6 = assign40300_e53363_d_n6;
        locals.var_tmf0_dn7 = assign40300_e53363_d_n7;
        locals.var_tmf0_dn8 = assign40300_e53363_d_n8;
        locals.var_tmf0_dn9 = assign40300_e53363_d_n9;
        locals.var_tmf0_dn10 = assign40300_e53363_d_n10;
        locals.var_tmf0_dn11 = assign40300_e53363_d_n11;
        locals.var_tmf0_dn14 = assign40300_e53363_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign40310_e53383, assign40310_e53383_d_n0, assign40310_e53383_d_n2, assign40310_e53383_d_n4, assign40310_e53383_d_n5, assign40310_e53383_d_n6, assign40310_e53383_d_n7, assign40310_e53383_d_n8, assign40310_e53383_d_n9, assign40310_e53383_d_n10, assign40310_e53383_d_n11, assign40310_e53383_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign40310_e53377: f64 = (locals.var_ps_delta * locals.var_xmp);
        let assign40310_e53379: f64 = (assign40310_e53377 * locals.var_dnm);
        let assign40310_e53381: f64 = (assign40310_e53379 / locals.var_arg);
        (assign40310_e53381, ((((((locals.var_ps_delta * locals.var_xmp_dn0) * locals.var_dnm) + (assign40310_e53377 * locals.var_dnm_dn0)) * locals.var_arg) - (assign40310_e53379 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_ps_delta * locals.var_xmp_dn2) * locals.var_dnm) + (assign40310_e53377 * locals.var_dnm_dn2)) * locals.var_arg) - (assign40310_e53379 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_ps_delta * locals.var_xmp_dn4) * locals.var_dnm) + (assign40310_e53377 * locals.var_dnm_dn4)) * locals.var_arg) - (assign40310_e53379 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_ps_delta * locals.var_xmp_dn5) * locals.var_dnm) + (assign40310_e53377 * locals.var_dnm_dn5)) * locals.var_arg) - (assign40310_e53379 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_ps_delta * locals.var_xmp_dn6) * locals.var_dnm) + (assign40310_e53377 * locals.var_dnm_dn6)) * locals.var_arg) - (assign40310_e53379 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_ps_delta * locals.var_xmp_dn7) * locals.var_dnm) + (assign40310_e53377 * locals.var_dnm_dn7)) * locals.var_arg) - (assign40310_e53379 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_ps_delta * locals.var_xmp_dn8) * locals.var_dnm) + (assign40310_e53377 * locals.var_dnm_dn8)) * locals.var_arg) - (assign40310_e53379 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_ps_delta * locals.var_xmp_dn9) * locals.var_dnm) + (assign40310_e53377 * locals.var_dnm_dn9)) * locals.var_arg) - (assign40310_e53379 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_ps_delta * locals.var_xmp_dn10) * locals.var_dnm) + (assign40310_e53377 * locals.var_dnm_dn10)) * locals.var_arg) - (assign40310_e53379 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_ps_delta * locals.var_xmp_dn11) * locals.var_dnm) + (assign40310_e53377 * locals.var_dnm_dn11)) * locals.var_arg) - (assign40310_e53379 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_ps_delta * locals.var_xmp_dn14) * locals.var_dnm) + (assign40310_e53377 * locals.var_dnm_dn14)) * locals.var_arg) - (assign40310_e53379 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign40310_e53383;
        locals.var_t0_dn0 = assign40310_e53383_d_n0;
        locals.var_t0_dn2 = assign40310_e53383_d_n2;
        locals.var_t0_dn4 = assign40310_e53383_d_n4;
        locals.var_t0_dn5 = assign40310_e53383_d_n5;
        locals.var_t0_dn6 = assign40310_e53383_d_n6;
        locals.var_t0_dn7 = assign40310_e53383_d_n7;
        locals.var_t0_dn8 = assign40310_e53383_d_n8;
        locals.var_t0_dn9 = assign40310_e53383_d_n9;
        locals.var_t0_dn10 = assign40310_e53383_d_n10;
        locals.var_t0_dn11 = assign40310_e53383_d_n11;
        locals.var_t0_dn14 = assign40310_e53383_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign40320_e53401, assign40320_e53401_d_n0, assign40320_e53401_d_n2, assign40320_e53401_d_n4, assign40320_e53401_d_n5, assign40320_e53401_d_n6, assign40320_e53401_d_n7, assign40320_e53401_d_n8, assign40320_e53401_d_n9, assign40320_e53401_d_n10, assign40320_e53401_d_n11, assign40320_e53401_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        let assign40320_e53397: f64 = locals.var_ps_delta;
        let assign40320_e53399: f64 = (assign40320_e53397 - locals.var_tmf0);
        (assign40320_e53399, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign40320_e53401;
        locals.var_t2_dn0 = assign40320_e53401_d_n0;
        locals.var_t2_dn2 = assign40320_e53401_d_n2;
        locals.var_t2_dn4 = assign40320_e53401_d_n4;
        locals.var_t2_dn5 = assign40320_e53401_d_n5;
        locals.var_t2_dn6 = assign40320_e53401_d_n6;
        locals.var_t2_dn7 = assign40320_e53401_d_n7;
        locals.var_t2_dn8 = assign40320_e53401_d_n8;
        locals.var_t2_dn9 = assign40320_e53401_d_n9;
        locals.var_t2_dn10 = assign40320_e53401_d_n10;
        locals.var_t2_dn11 = assign40320_e53401_d_n11;
        locals.var_t2_dn14 = assign40320_e53401_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign40330_e53415, assign40330_e53415_d_n0, assign40330_e53415_d_n2, assign40330_e53415_d_n4, assign40330_e53415_d_n5, assign40330_e53415_d_n6, assign40330_e53415_d_n7, assign40330_e53415_d_n8, assign40330_e53415_d_n9, assign40330_e53415_d_n10, assign40330_e53415_d_n11, assign40330_e53415_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) && (locals.var_guard1011 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign40330_e53415;
        locals.var_t0_dn0 = assign40330_e53415_d_n0;
        locals.var_t0_dn2 = assign40330_e53415_d_n2;
        locals.var_t0_dn4 = assign40330_e53415_d_n4;
        locals.var_t0_dn5 = assign40330_e53415_d_n5;
        locals.var_t0_dn6 = assign40330_e53415_d_n6;
        locals.var_t0_dn7 = assign40330_e53415_d_n7;
        locals.var_t0_dn8 = assign40330_e53415_d_n8;
        locals.var_t0_dn9 = assign40330_e53415_d_n9;
        locals.var_t0_dn10 = assign40330_e53415_d_n10;
        locals.var_t0_dn11 = assign40330_e53415_d_n11;
        locals.var_t0_dn14 = assign40330_e53415_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign40340_e53432, assign40340_e53432_d_n0, assign40340_e53432_d_n2, assign40340_e53432_d_n4, assign40340_e53432_d_n5, assign40340_e53432_d_n6, assign40340_e53432_d_n7, assign40340_e53432_d_n8, assign40340_e53432_d_n9, assign40340_e53432_d_n10, assign40340_e53432_d_n11, assign40340_e53432_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) && (locals.var_guard1011 == 0.0)) {
        let assign40340_e53430: f64 = (locals.var_phi_sl_dep__blk858 - locals.var_vds_maxbl__blk856);
        (assign40340_e53430, (locals.var_phi_sl_dep__blk858_dn0 - locals.var_vds_maxbl__blk856_dn0), (locals.var_phi_sl_dep__blk858_dn2 - locals.var_vds_maxbl__blk856_dn2), (locals.var_phi_sl_dep__blk858_dn4 - locals.var_vds_maxbl__blk856_dn4), (locals.var_phi_sl_dep__blk858_dn5 - locals.var_vds_maxbl__blk856_dn5), (locals.var_phi_sl_dep__blk858_dn6 - locals.var_vds_maxbl__blk856_dn6), (locals.var_phi_sl_dep__blk858_dn7 - locals.var_vds_maxbl__blk856_dn7), (locals.var_phi_sl_dep__blk858_dn8 - locals.var_vds_maxbl__blk856_dn8), (locals.var_phi_sl_dep__blk858_dn9 - locals.var_vds_maxbl__blk856_dn9), (locals.var_phi_sl_dep__blk858_dn10 - locals.var_vds_maxbl__blk856_dn10), (locals.var_phi_sl_dep__blk858_dn11 - locals.var_vds_maxbl__blk856_dn11), (locals.var_phi_sl_dep__blk858_dn14 - locals.var_vds_maxbl__blk856_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign40340_e53432;
        locals.var_t2_dn0 = assign40340_e53432_d_n0;
        locals.var_t2_dn2 = assign40340_e53432_d_n2;
        locals.var_t2_dn4 = assign40340_e53432_d_n4;
        locals.var_t2_dn5 = assign40340_e53432_d_n5;
        locals.var_t2_dn6 = assign40340_e53432_d_n6;
        locals.var_t2_dn7 = assign40340_e53432_d_n7;
        locals.var_t2_dn8 = assign40340_e53432_d_n8;
        locals.var_t2_dn9 = assign40340_e53432_d_n9;
        locals.var_t2_dn10 = assign40340_e53432_d_n10;
        locals.var_t2_dn11 = assign40340_e53432_d_n11;
        locals.var_t2_dn14 = assign40340_e53432_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign40350_e53447, assign40350_e53447_d_n0, assign40350_e53447_d_n2, assign40350_e53447_d_n4, assign40350_e53447_d_n5, assign40350_e53447_d_n6, assign40350_e53447_d_n7, assign40350_e53447_d_n8, assign40350_e53447_d_n9, assign40350_e53447_d_n10, assign40350_e53447_d_n11, assign40350_e53447_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) && (locals.var_guard1011 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign40350_e53447;
        locals.var_t0_dn0 = assign40350_e53447_d_n0;
        locals.var_t0_dn2 = assign40350_e53447_d_n2;
        locals.var_t0_dn4 = assign40350_e53447_d_n4;
        locals.var_t0_dn5 = assign40350_e53447_d_n5;
        locals.var_t0_dn6 = assign40350_e53447_d_n6;
        locals.var_t0_dn7 = assign40350_e53447_d_n7;
        locals.var_t0_dn8 = assign40350_e53447_d_n8;
        locals.var_t0_dn9 = assign40350_e53447_d_n9;
        locals.var_t0_dn10 = assign40350_e53447_d_n10;
        locals.var_t0_dn11 = assign40350_e53447_d_n11;
        locals.var_t0_dn14 = assign40350_e53447_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign40360_e53470, assign40360_e53470_d_n0, assign40360_e53470_d_n2, assign40360_e53470_d_n4, assign40360_e53470_d_n5, assign40360_e53470_d_n6, assign40360_e53470_d_n7, assign40360_e53470_d_n8, assign40360_e53470_d_n9, assign40360_e53470_d_n10, assign40360_e53470_d_n11, assign40360_e53470_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) {
        let assign40360_e53459: f64 = (locals.var_beta * locals.var_t2);
        let assign40360_e53460: f64 = (assign40360_e53459).exp();
        let assign40360_e53462: f64 = (assign40360_e53460 - 1.0);
        let assign40360_e53465: f64 = (locals.var_beta * locals.var_t2);
        let assign40360_e53466: f64 = (assign40360_e53462 - assign40360_e53465);
        let assign40360_e53468: f64 = (assign40360_e53466 + 1e-15);
        (assign40360_e53468, ((assign40360_e53460 * ((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0))) - ((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0))), ((assign40360_e53460 * ((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2))) - ((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2))), ((assign40360_e53460 * ((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4))) - ((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4))), ((assign40360_e53460 * ((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5))) - ((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5))), ((assign40360_e53460 * ((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6))) - ((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6))), ((assign40360_e53460 * ((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7))) - ((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7))), ((assign40360_e53460 * ((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8))) - ((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8))), ((assign40360_e53460 * ((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9))) - ((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9))), ((assign40360_e53460 * ((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10))) - ((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10))), ((assign40360_e53460 * ((locals.var_beta_dn11 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn11))) - ((locals.var_beta_dn11 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn11))), ((assign40360_e53460 * ((locals.var_beta_dn14 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn14))) - ((locals.var_beta_dn14 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign40360_e53470;
        locals.var_t4_dn0 = assign40360_e53470_d_n0;
        locals.var_t4_dn2 = assign40360_e53470_d_n2;
        locals.var_t4_dn4 = assign40360_e53470_d_n4;
        locals.var_t4_dn5 = assign40360_e53470_d_n5;
        locals.var_t4_dn6 = assign40360_e53470_d_n6;
        locals.var_t4_dn7 = assign40360_e53470_d_n7;
        locals.var_t4_dn8 = assign40360_e53470_d_n8;
        locals.var_t4_dn9 = assign40360_e53470_d_n9;
        locals.var_t4_dn10 = assign40360_e53470_d_n10;
        locals.var_t4_dn11 = assign40360_e53470_d_n11;
        locals.var_t4_dn14 = assign40360_e53470_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign40370_e53486, assign40370_e53486_d_n0, assign40370_e53486_d_n2, assign40370_e53486_d_n4, assign40370_e53486_d_n5, assign40370_e53486_d_n6, assign40370_e53486_d_n7, assign40370_e53486_d_n8, assign40370_e53486_d_n9, assign40370_e53486_d_n10, assign40370_e53486_d_n11, assign40370_e53486_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard979 == 0.0)) {
        let assign40370_e53481: f64 = (-locals.var_cnst0);
        let assign40370_e53483: f64 = (locals.var_t4).sqrt();
        let assign40370_e53484: f64 = (assign40370_e53481 * assign40370_e53483);
        (assign40370_e53484, (((-locals.var_cnst0_dn0) * assign40370_e53483) + (assign40370_e53481 * (locals.var_t4_dn0 / (2.0 * assign40370_e53483)))), (((-locals.var_cnst0_dn2) * assign40370_e53483) + (assign40370_e53481 * (locals.var_t4_dn2 / (2.0 * assign40370_e53483)))), (((-locals.var_cnst0_dn4) * assign40370_e53483) + (assign40370_e53481 * (locals.var_t4_dn4 / (2.0 * assign40370_e53483)))), (((-locals.var_cnst0_dn5) * assign40370_e53483) + (assign40370_e53481 * (locals.var_t4_dn5 / (2.0 * assign40370_e53483)))), (((-locals.var_cnst0_dn6) * assign40370_e53483) + (assign40370_e53481 * (locals.var_t4_dn6 / (2.0 * assign40370_e53483)))), (((-locals.var_cnst0_dn7) * assign40370_e53483) + (assign40370_e53481 * (locals.var_t4_dn7 / (2.0 * assign40370_e53483)))), (((-locals.var_cnst0_dn8) * assign40370_e53483) + (assign40370_e53481 * (locals.var_t4_dn8 / (2.0 * assign40370_e53483)))), (((-locals.var_cnst0_dn9) * assign40370_e53483) + (assign40370_e53481 * (locals.var_t4_dn9 / (2.0 * assign40370_e53483)))), (((-locals.var_cnst0_dn10) * assign40370_e53483) + (assign40370_e53481 * (locals.var_t4_dn10 / (2.0 * assign40370_e53483)))), (((-locals.var_cnst0_dn11) * assign40370_e53483) + (assign40370_e53481 * (locals.var_t4_dn11 / (2.0 * assign40370_e53483)))), (((-locals.var_cnst0_dn14) * assign40370_e53483) + (assign40370_e53481 * (locals.var_t4_dn14 / (2.0 * assign40370_e53483)))),)
    } else {
        (locals.var_q_nl_cur__blk894, locals.var_q_nl_cur__blk894_dn0, locals.var_q_nl_cur__blk894_dn2, locals.var_q_nl_cur__blk894_dn4, locals.var_q_nl_cur__blk894_dn5, locals.var_q_nl_cur__blk894_dn6, locals.var_q_nl_cur__blk894_dn7, locals.var_q_nl_cur__blk894_dn8, locals.var_q_nl_cur__blk894_dn9, locals.var_q_nl_cur__blk894_dn10, locals.var_q_nl_cur__blk894_dn11, locals.var_q_nl_cur__blk894_dn14,)
    }
};
        locals.var_q_nl_cur__blk894 = assign40370_e53486;
        locals.var_q_nl_cur__blk894_dn0 = assign40370_e53486_d_n0;
        locals.var_q_nl_cur__blk894_dn2 = assign40370_e53486_d_n2;
        locals.var_q_nl_cur__blk894_dn4 = assign40370_e53486_d_n4;
        locals.var_q_nl_cur__blk894_dn5 = assign40370_e53486_d_n5;
        locals.var_q_nl_cur__blk894_dn6 = assign40370_e53486_d_n6;
        locals.var_q_nl_cur__blk894_dn7 = assign40370_e53486_d_n7;
        locals.var_q_nl_cur__blk894_dn8 = assign40370_e53486_d_n8;
        locals.var_q_nl_cur__blk894_dn9 = assign40370_e53486_d_n9;
        locals.var_q_nl_cur__blk894_dn10 = assign40370_e53486_d_n10;
        locals.var_q_nl_cur__blk894_dn11 = assign40370_e53486_d_n11;
        locals.var_q_nl_cur__blk894_dn14 = assign40370_e53486_d_n14;
        locals.var_q_nl_cur__blk894_rv = 0.0;

        let (assign40380_e53495, assign40380_e53495_d_n0, assign40380_e53495_d_n2, assign40380_e53495_d_n4, assign40380_e53495_d_n5, assign40380_e53495_d_n6, assign40380_e53495_d_n7, assign40380_e53495_d_n8, assign40380_e53495_d_n9, assign40380_e53495_d_n10, assign40380_e53495_d_n11, assign40380_e53495_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        (locals.var_phi_s0_dep__blk857, locals.var_phi_s0_dep__blk857_dn0, locals.var_phi_s0_dep__blk857_dn2, locals.var_phi_s0_dep__blk857_dn4, locals.var_phi_s0_dep__blk857_dn5, locals.var_phi_s0_dep__blk857_dn6, locals.var_phi_s0_dep__blk857_dn7, locals.var_phi_s0_dep__blk857_dn8, locals.var_phi_s0_dep__blk857_dn9, locals.var_phi_s0_dep__blk857_dn10, locals.var_phi_s0_dep__blk857_dn11, locals.var_phi_s0_dep__blk857_dn14,)
    } else {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn4, locals.var_ps0_dn5, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn8, locals.var_ps0_dn9, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn14,)
    }
};
        locals.var_ps0 = assign40380_e53495;
        locals.var_ps0_dn0 = assign40380_e53495_d_n0;
        locals.var_ps0_dn2 = assign40380_e53495_d_n2;
        locals.var_ps0_dn4 = assign40380_e53495_d_n4;
        locals.var_ps0_dn5 = assign40380_e53495_d_n5;
        locals.var_ps0_dn6 = assign40380_e53495_d_n6;
        locals.var_ps0_dn7 = assign40380_e53495_d_n7;
        locals.var_ps0_dn8 = assign40380_e53495_d_n8;
        locals.var_ps0_dn9 = assign40380_e53495_d_n9;
        locals.var_ps0_dn10 = assign40380_e53495_d_n10;
        locals.var_ps0_dn11 = assign40380_e53495_d_n11;
        locals.var_ps0_dn14 = assign40380_e53495_d_n14;
        locals.var_ps0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_138(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign40390_e53504, assign40390_e53504_d_n0, assign40390_e53504_d_n2, assign40390_e53504_d_n4, assign40390_e53504_d_n5, assign40390_e53504_d_n6, assign40390_e53504_d_n7, assign40390_e53504_d_n8, assign40390_e53504_d_n9, assign40390_e53504_d_n10, assign40390_e53504_d_n11, assign40390_e53504_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        (locals.var_phi_sl_dep__blk858, locals.var_phi_sl_dep__blk858_dn0, locals.var_phi_sl_dep__blk858_dn2, locals.var_phi_sl_dep__blk858_dn4, locals.var_phi_sl_dep__blk858_dn5, locals.var_phi_sl_dep__blk858_dn6, locals.var_phi_sl_dep__blk858_dn7, locals.var_phi_sl_dep__blk858_dn8, locals.var_phi_sl_dep__blk858_dn9, locals.var_phi_sl_dep__blk858_dn10, locals.var_phi_sl_dep__blk858_dn11, locals.var_phi_sl_dep__blk858_dn14,)
    } else {
        (locals.var_psl, locals.var_psl_dn0, locals.var_psl_dn2, locals.var_psl_dn4, locals.var_psl_dn5, locals.var_psl_dn6, locals.var_psl_dn7, locals.var_psl_dn8, locals.var_psl_dn9, locals.var_psl_dn10, locals.var_psl_dn11, locals.var_psl_dn14,)
    }
};
        locals.var_psl = assign40390_e53504;
        locals.var_psl_dn0 = assign40390_e53504_d_n0;
        locals.var_psl_dn2 = assign40390_e53504_d_n2;
        locals.var_psl_dn4 = assign40390_e53504_d_n4;
        locals.var_psl_dn5 = assign40390_e53504_d_n5;
        locals.var_psl_dn6 = assign40390_e53504_d_n6;
        locals.var_psl_dn7 = assign40390_e53504_d_n7;
        locals.var_psl_dn8 = assign40390_e53504_d_n8;
        locals.var_psl_dn9 = assign40390_e53504_d_n9;
        locals.var_psl_dn10 = assign40390_e53504_d_n10;
        locals.var_psl_dn11 = assign40390_e53504_d_n11;
        locals.var_psl_dn14 = assign40390_e53504_d_n14;
        locals.var_psl_rv = 0.0;

        let (assign40400_e53515, assign40400_e53515_d_n0, assign40400_e53515_d_n2, assign40400_e53515_d_n4, assign40400_e53515_d_n5, assign40400_e53515_d_n6, assign40400_e53515_d_n7, assign40400_e53515_d_n8, assign40400_e53515_d_n9, assign40400_e53515_d_n10, assign40400_e53515_d_n11, assign40400_e53515_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        let assign40400_e53513: f64 = (locals.var_phi_sl_dep__blk858 - locals.var_phi_s0_dep__blk857);
        (assign40400_e53513, (locals.var_phi_sl_dep__blk858_dn0 - locals.var_phi_s0_dep__blk857_dn0), (locals.var_phi_sl_dep__blk858_dn2 - locals.var_phi_s0_dep__blk857_dn2), (locals.var_phi_sl_dep__blk858_dn4 - locals.var_phi_s0_dep__blk857_dn4), (locals.var_phi_sl_dep__blk858_dn5 - locals.var_phi_s0_dep__blk857_dn5), (locals.var_phi_sl_dep__blk858_dn6 - locals.var_phi_s0_dep__blk857_dn6), (locals.var_phi_sl_dep__blk858_dn7 - locals.var_phi_s0_dep__blk857_dn7), (locals.var_phi_sl_dep__blk858_dn8 - locals.var_phi_s0_dep__blk857_dn8), (locals.var_phi_sl_dep__blk858_dn9 - locals.var_phi_s0_dep__blk857_dn9), (locals.var_phi_sl_dep__blk858_dn10 - locals.var_phi_s0_dep__blk857_dn10), (locals.var_phi_sl_dep__blk858_dn11 - locals.var_phi_s0_dep__blk857_dn11), (locals.var_phi_sl_dep__blk858_dn14 - locals.var_phi_s0_dep__blk857_dn14),)
    } else {
        (locals.var_pds, locals.var_pds_dn0, locals.var_pds_dn2, locals.var_pds_dn4, locals.var_pds_dn5, locals.var_pds_dn6, locals.var_pds_dn7, locals.var_pds_dn8, locals.var_pds_dn9, locals.var_pds_dn10, locals.var_pds_dn11, locals.var_pds_dn14,)
    }
};
        locals.var_pds = assign40400_e53515;
        locals.var_pds_dn0 = assign40400_e53515_d_n0;
        locals.var_pds_dn2 = assign40400_e53515_d_n2;
        locals.var_pds_dn4 = assign40400_e53515_d_n4;
        locals.var_pds_dn5 = assign40400_e53515_d_n5;
        locals.var_pds_dn6 = assign40400_e53515_d_n6;
        locals.var_pds_dn7 = assign40400_e53515_d_n7;
        locals.var_pds_dn8 = assign40400_e53515_d_n8;
        locals.var_pds_dn9 = assign40400_e53515_d_n9;
        locals.var_pds_dn10 = assign40400_e53515_d_n10;
        locals.var_pds_dn11 = assign40400_e53515_d_n11;
        locals.var_pds_dn14 = assign40400_e53515_d_n14;
        locals.var_pds_rv = 0.0;

        let (assign40410_e53528, assign40410_e53528_d_n0, assign40410_e53528_d_n2, assign40410_e53528_d_n4, assign40410_e53528_d_n5, assign40410_e53528_d_n6, assign40410_e53528_d_n7, assign40410_e53528_d_n8, assign40410_e53528_d_n9, assign40410_e53528_d_n10, assign40410_e53528_d_n11, assign40410_e53528_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        let assign40410_e53524: f64 = (locals.var_vds - locals.var_pds);
        let assign40410_e53526: f64 = (assign40410_e53524 / 2.0);
        (assign40410_e53526, ((locals.var_vds_dn0 - locals.var_pds_dn0) / 2.0), ((locals.var_vds_dn2 - locals.var_pds_dn2) / 2.0), ((locals.var_vds_dn4 - locals.var_pds_dn4) / 2.0), ((locals.var_vds_dn5 - locals.var_pds_dn5) / 2.0), ((locals.var_vds_dn6 - locals.var_pds_dn6) / 2.0), ((locals.var_vds_dn7 - locals.var_pds_dn7) / 2.0), ((locals.var_vds_dn8 - locals.var_pds_dn8) / 2.0), ((locals.var_vds_dn9 - locals.var_pds_dn9) / 2.0), ((locals.var_vds_dn10 - locals.var_pds_dn10) / 2.0), ((locals.var_vds_dn11 - locals.var_pds_dn11) / 2.0), ((locals.var_vds_dn14 - locals.var_pds_dn14) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign40410_e53528;
        locals.var_t1_dn0 = assign40410_e53528_d_n0;
        locals.var_t1_dn2 = assign40410_e53528_d_n2;
        locals.var_t1_dn4 = assign40410_e53528_d_n4;
        locals.var_t1_dn5 = assign40410_e53528_d_n5;
        locals.var_t1_dn6 = assign40410_e53528_d_n6;
        locals.var_t1_dn7 = assign40410_e53528_d_n7;
        locals.var_t1_dn8 = assign40410_e53528_d_n8;
        locals.var_t1_dn9 = assign40410_e53528_d_n9;
        locals.var_t1_dn10 = assign40410_e53528_d_n10;
        locals.var_t1_dn11 = assign40410_e53528_d_n11;
        locals.var_t1_dn14 = assign40410_e53528_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign40420_e53543, assign40420_e53543_d_n0, assign40420_e53543_d_n2, assign40420_e53543_d_n4, assign40420_e53543_d_n5, assign40420_e53543_d_n6, assign40420_e53543_d_n7, assign40420_e53543_d_n8, assign40420_e53543_d_n9, assign40420_e53543_d_n10, assign40420_e53543_d_n11, assign40420_e53543_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        let assign40420_e53537: f64 = (2.0 * locals.var_t1);
        let assign40420_e53540: f64 = (p.p263 * 0.1);
        let assign40420_e53541: f64 = (assign40420_e53537 / assign40420_e53540);
        (assign40420_e53541, ((2.0 * locals.var_t1_dn0) / assign40420_e53540), ((2.0 * locals.var_t1_dn2) / assign40420_e53540), ((2.0 * locals.var_t1_dn4) / assign40420_e53540), ((2.0 * locals.var_t1_dn5) / assign40420_e53540), ((2.0 * locals.var_t1_dn6) / assign40420_e53540), ((2.0 * locals.var_t1_dn7) / assign40420_e53540), ((2.0 * locals.var_t1_dn8) / assign40420_e53540), ((2.0 * locals.var_t1_dn9) / assign40420_e53540), ((2.0 * locals.var_t1_dn10) / assign40420_e53540), ((2.0 * locals.var_t1_dn11) / assign40420_e53540), ((2.0 * locals.var_t1_dn14) / assign40420_e53540),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign40420_e53543;
        locals.var_tmf1_dn0 = assign40420_e53543_d_n0;
        locals.var_tmf1_dn2 = assign40420_e53543_d_n2;
        locals.var_tmf1_dn4 = assign40420_e53543_d_n4;
        locals.var_tmf1_dn5 = assign40420_e53543_d_n5;
        locals.var_tmf1_dn6 = assign40420_e53543_d_n6;
        locals.var_tmf1_dn7 = assign40420_e53543_d_n7;
        locals.var_tmf1_dn8 = assign40420_e53543_d_n8;
        locals.var_tmf1_dn9 = assign40420_e53543_d_n9;
        locals.var_tmf1_dn10 = assign40420_e53543_d_n10;
        locals.var_tmf1_dn11 = assign40420_e53543_d_n11;
        locals.var_tmf1_dn14 = assign40420_e53543_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign40430_e53588, assign40430_e53588_d_n0, assign40430_e53588_d_n2, assign40430_e53588_d_n4, assign40430_e53588_d_n5, assign40430_e53588_d_n6, assign40430_e53588_d_n7, assign40430_e53588_d_n8, assign40430_e53588_d_n9, assign40430_e53588_d_n10, assign40430_e53588_d_n11, assign40430_e53588_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        let assign40430_e53554: f64 = (1.0 / 2.0);
        let assign40430_e53558: f64 = (1.0 / 6.0);
        let assign40430_e53562: f64 = (1.0 / 24.0);
        let assign40430_e53566: f64 = (1.0 / 120.0);
        let assign40430_e53570: f64 = (1.0 / 720.0);
        let assign40430_e53574: f64 = (1.0 / 5040.0);
        let assign40430_e53575: f64 = (locals.var_tmf1 * assign40430_e53574);
        let assign40430_e53576: f64 = (assign40430_e53570 + assign40430_e53575);
        let assign40430_e53577: f64 = (locals.var_tmf1 * assign40430_e53576);
        let assign40430_e53578: f64 = (assign40430_e53566 + assign40430_e53577);
        let assign40430_e53579: f64 = (locals.var_tmf1 * assign40430_e53578);
        let assign40430_e53580: f64 = (assign40430_e53562 + assign40430_e53579);
        let assign40430_e53581: f64 = (locals.var_tmf1 * assign40430_e53580);
        let assign40430_e53582: f64 = (assign40430_e53558 + assign40430_e53581);
        let assign40430_e53583: f64 = (locals.var_tmf1 * assign40430_e53582);
        let assign40430_e53584: f64 = (assign40430_e53554 + assign40430_e53583);
        let assign40430_e53585: f64 = (locals.var_tmf1 * assign40430_e53584);
        let assign40430_e53586: f64 = (1.0 + assign40430_e53585);
        (assign40430_e53586, ((locals.var_tmf1_dn0 * assign40430_e53584) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign40430_e53582) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign40430_e53580) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign40430_e53578) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign40430_e53576) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign40430_e53574))))))))))), ((locals.var_tmf1_dn2 * assign40430_e53584) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign40430_e53582) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign40430_e53580) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign40430_e53578) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign40430_e53576) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign40430_e53574))))))))))), ((locals.var_tmf1_dn4 * assign40430_e53584) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign40430_e53582) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign40430_e53580) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign40430_e53578) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign40430_e53576) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign40430_e53574))))))))))), ((locals.var_tmf1_dn5 * assign40430_e53584) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign40430_e53582) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign40430_e53580) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign40430_e53578) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign40430_e53576) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign40430_e53574))))))))))), ((locals.var_tmf1_dn6 * assign40430_e53584) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign40430_e53582) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign40430_e53580) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign40430_e53578) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign40430_e53576) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign40430_e53574))))))))))), ((locals.var_tmf1_dn7 * assign40430_e53584) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign40430_e53582) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign40430_e53580) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign40430_e53578) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign40430_e53576) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign40430_e53574))))))))))), ((locals.var_tmf1_dn8 * assign40430_e53584) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign40430_e53582) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign40430_e53580) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign40430_e53578) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign40430_e53576) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign40430_e53574))))))))))), ((locals.var_tmf1_dn9 * assign40430_e53584) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign40430_e53582) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign40430_e53580) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign40430_e53578) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign40430_e53576) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign40430_e53574))))))))))), ((locals.var_tmf1_dn10 * assign40430_e53584) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign40430_e53582) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign40430_e53580) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign40430_e53578) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign40430_e53576) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign40430_e53574))))))))))), ((locals.var_tmf1_dn11 * assign40430_e53584) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign40430_e53582) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign40430_e53580) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign40430_e53578) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign40430_e53576) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign40430_e53574))))))))))), ((locals.var_tmf1_dn14 * assign40430_e53584) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign40430_e53582) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign40430_e53580) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign40430_e53578) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign40430_e53576) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign40430_e53574))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign40430_e53588;
        locals.var_tmf2_dn0 = assign40430_e53588_d_n0;
        locals.var_tmf2_dn2 = assign40430_e53588_d_n2;
        locals.var_tmf2_dn4 = assign40430_e53588_d_n4;
        locals.var_tmf2_dn5 = assign40430_e53588_d_n5;
        locals.var_tmf2_dn6 = assign40430_e53588_d_n6;
        locals.var_tmf2_dn7 = assign40430_e53588_d_n7;
        locals.var_tmf2_dn8 = assign40430_e53588_d_n8;
        locals.var_tmf2_dn9 = assign40430_e53588_d_n9;
        locals.var_tmf2_dn10 = assign40430_e53588_d_n10;
        locals.var_tmf2_dn11 = assign40430_e53588_d_n11;
        locals.var_tmf2_dn14 = assign40430_e53588_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign40440_e53629, assign40440_e53629_d_n0, assign40440_e53629_d_n2, assign40440_e53629_d_n4, assign40440_e53629_d_n5, assign40440_e53629_d_n6, assign40440_e53629_d_n7, assign40440_e53629_d_n8, assign40440_e53629_d_n9, assign40440_e53629_d_n10, assign40440_e53629_d_n11, assign40440_e53629_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        let assign40440_e53597: f64 = (1.0 / 2.0);
        let assign40440_e53601: f64 = (1.0 / 3.0);
        let assign40440_e53605: f64 = (1.0 / 8.0);
        let assign40440_e53609: f64 = (1.0 / 30.0);
        let assign40440_e53613: f64 = (1.0 / 144.0);
        let assign40440_e53617: f64 = (1.0 / 840.0);
        let assign40440_e53618: f64 = (locals.var_tmf1 * assign40440_e53617);
        let assign40440_e53619: f64 = (assign40440_e53613 + assign40440_e53618);
        let assign40440_e53620: f64 = (locals.var_tmf1 * assign40440_e53619);
        let assign40440_e53621: f64 = (assign40440_e53609 + assign40440_e53620);
        let assign40440_e53622: f64 = (locals.var_tmf1 * assign40440_e53621);
        let assign40440_e53623: f64 = (assign40440_e53605 + assign40440_e53622);
        let assign40440_e53624: f64 = (locals.var_tmf1 * assign40440_e53623);
        let assign40440_e53625: f64 = (assign40440_e53601 + assign40440_e53624);
        let assign40440_e53626: f64 = (locals.var_tmf1 * assign40440_e53625);
        let assign40440_e53627: f64 = (assign40440_e53597 + assign40440_e53626);
        (assign40440_e53627, ((locals.var_tmf1_dn0 * assign40440_e53625) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign40440_e53623) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign40440_e53621) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign40440_e53619) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign40440_e53617))))))))), ((locals.var_tmf1_dn2 * assign40440_e53625) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign40440_e53623) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign40440_e53621) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign40440_e53619) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign40440_e53617))))))))), ((locals.var_tmf1_dn4 * assign40440_e53625) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign40440_e53623) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign40440_e53621) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign40440_e53619) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign40440_e53617))))))))), ((locals.var_tmf1_dn5 * assign40440_e53625) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign40440_e53623) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign40440_e53621) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign40440_e53619) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign40440_e53617))))))))), ((locals.var_tmf1_dn6 * assign40440_e53625) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign40440_e53623) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign40440_e53621) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign40440_e53619) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign40440_e53617))))))))), ((locals.var_tmf1_dn7 * assign40440_e53625) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign40440_e53623) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign40440_e53621) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign40440_e53619) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign40440_e53617))))))))), ((locals.var_tmf1_dn8 * assign40440_e53625) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign40440_e53623) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign40440_e53621) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign40440_e53619) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign40440_e53617))))))))), ((locals.var_tmf1_dn9 * assign40440_e53625) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign40440_e53623) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign40440_e53621) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign40440_e53619) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign40440_e53617))))))))), ((locals.var_tmf1_dn10 * assign40440_e53625) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign40440_e53623) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign40440_e53621) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign40440_e53619) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign40440_e53617))))))))), ((locals.var_tmf1_dn11 * assign40440_e53625) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign40440_e53623) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign40440_e53621) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign40440_e53619) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign40440_e53617))))))))), ((locals.var_tmf1_dn14 * assign40440_e53625) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign40440_e53623) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign40440_e53621) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign40440_e53619) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign40440_e53617))))))))),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn14,)
    }
};
        locals.var_tmf3 = assign40440_e53629;
        locals.var_tmf3_dn0 = assign40440_e53629_d_n0;
        locals.var_tmf3_dn2 = assign40440_e53629_d_n2;
        locals.var_tmf3_dn4 = assign40440_e53629_d_n4;
        locals.var_tmf3_dn5 = assign40440_e53629_d_n5;
        locals.var_tmf3_dn6 = assign40440_e53629_d_n6;
        locals.var_tmf3_dn7 = assign40440_e53629_d_n7;
        locals.var_tmf3_dn8 = assign40440_e53629_d_n8;
        locals.var_tmf3_dn9 = assign40440_e53629_d_n9;
        locals.var_tmf3_dn10 = assign40440_e53629_d_n10;
        locals.var_tmf3_dn11 = assign40440_e53629_d_n11;
        locals.var_tmf3_dn14 = assign40440_e53629_d_n14;
        locals.var_tmf3_rv = 0.0;

        let (assign40450_e53642, assign40450_e53642_d_n0, assign40450_e53642_d_n2, assign40450_e53642_d_n4, assign40450_e53642_d_n5, assign40450_e53642_d_n6, assign40450_e53642_d_n7, assign40450_e53642_d_n8, assign40450_e53642_d_n9, assign40450_e53642_d_n10, assign40450_e53642_d_n11, assign40450_e53642_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        let assign40450_e53638: f64 = (p.p263 * 0.1);
        let assign40450_e53640: f64 = (assign40450_e53638 / locals.var_tmf2);
        (assign40450_e53640, (-((assign40450_e53638 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((assign40450_e53638 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((assign40450_e53638 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2))), (-((assign40450_e53638 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2))), (-((assign40450_e53638 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((assign40450_e53638 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2))), (-((assign40450_e53638 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2))), (-((assign40450_e53638 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2))), (-((assign40450_e53638 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((assign40450_e53638 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2))), (-((assign40450_e53638 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_pzadd, locals.var_pzadd_dn0, locals.var_pzadd_dn2, locals.var_pzadd_dn4, locals.var_pzadd_dn5, locals.var_pzadd_dn6, locals.var_pzadd_dn7, locals.var_pzadd_dn8, locals.var_pzadd_dn9, locals.var_pzadd_dn10, locals.var_pzadd_dn11, locals.var_pzadd_dn14,)
    }
};
        locals.var_pzadd = assign40450_e53642;
        locals.var_pzadd_dn0 = assign40450_e53642_d_n0;
        locals.var_pzadd_dn2 = assign40450_e53642_d_n2;
        locals.var_pzadd_dn4 = assign40450_e53642_d_n4;
        locals.var_pzadd_dn5 = assign40450_e53642_d_n5;
        locals.var_pzadd_dn6 = assign40450_e53642_d_n6;
        locals.var_pzadd_dn7 = assign40450_e53642_d_n7;
        locals.var_pzadd_dn8 = assign40450_e53642_d_n8;
        locals.var_pzadd_dn9 = assign40450_e53642_d_n9;
        locals.var_pzadd_dn10 = assign40450_e53642_d_n10;
        locals.var_pzadd_dn11 = assign40450_e53642_d_n11;
        locals.var_pzadd_dn14 = assign40450_e53642_d_n14;
        locals.var_pzadd_rv = 0.0;

        let (assign40460_e53658, assign40460_e53658_d_n0, assign40460_e53658_d_n2, assign40460_e53658_d_n4, assign40460_e53658_d_n5, assign40460_e53658_d_n6, assign40460_e53658_d_n7, assign40460_e53658_d_n8, assign40460_e53658_d_n9, assign40460_e53658_d_n10, assign40460_e53658_d_n11, assign40460_e53658_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        let assign40460_e53650: f64 = (-2.0);
        let assign40460_e53652: f64 = (assign40460_e53650 * locals.var_tmf3);
        let assign40460_e53655: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign40460_e53656: f64 = (assign40460_e53652 / assign40460_e53655);
        (assign40460_e53656, ((((assign40460_e53650 * locals.var_tmf3_dn0) * assign40460_e53655) - (assign40460_e53652 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign40460_e53655 * assign40460_e53655)), ((((assign40460_e53650 * locals.var_tmf3_dn2) * assign40460_e53655) - (assign40460_e53652 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign40460_e53655 * assign40460_e53655)), ((((assign40460_e53650 * locals.var_tmf3_dn4) * assign40460_e53655) - (assign40460_e53652 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign40460_e53655 * assign40460_e53655)), ((((assign40460_e53650 * locals.var_tmf3_dn5) * assign40460_e53655) - (assign40460_e53652 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign40460_e53655 * assign40460_e53655)), ((((assign40460_e53650 * locals.var_tmf3_dn6) * assign40460_e53655) - (assign40460_e53652 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign40460_e53655 * assign40460_e53655)), ((((assign40460_e53650 * locals.var_tmf3_dn7) * assign40460_e53655) - (assign40460_e53652 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign40460_e53655 * assign40460_e53655)), ((((assign40460_e53650 * locals.var_tmf3_dn8) * assign40460_e53655) - (assign40460_e53652 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign40460_e53655 * assign40460_e53655)), ((((assign40460_e53650 * locals.var_tmf3_dn9) * assign40460_e53655) - (assign40460_e53652 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign40460_e53655 * assign40460_e53655)), ((((assign40460_e53650 * locals.var_tmf3_dn10) * assign40460_e53655) - (assign40460_e53652 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign40460_e53655 * assign40460_e53655)), ((((assign40460_e53650 * locals.var_tmf3_dn11) * assign40460_e53655) - (assign40460_e53652 * ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)))) / (assign40460_e53655 * assign40460_e53655)), ((((assign40460_e53650 * locals.var_tmf3_dn14) * assign40460_e53655) - (assign40460_e53652 * ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)))) / (assign40460_e53655 * assign40460_e53655)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign40460_e53658;
        locals.var_t2_dn0 = assign40460_e53658_d_n0;
        locals.var_t2_dn2 = assign40460_e53658_d_n2;
        locals.var_t2_dn4 = assign40460_e53658_d_n4;
        locals.var_t2_dn5 = assign40460_e53658_d_n5;
        locals.var_t2_dn6 = assign40460_e53658_d_n6;
        locals.var_t2_dn7 = assign40460_e53658_d_n7;
        locals.var_t2_dn8 = assign40460_e53658_d_n8;
        locals.var_t2_dn9 = assign40460_e53658_d_n9;
        locals.var_t2_dn10 = assign40460_e53658_d_n10;
        locals.var_t2_dn11 = assign40460_e53658_d_n11;
        locals.var_t2_dn14 = assign40460_e53658_d_n14;
        locals.var_t2_rv = 0.0;

        let assign40470_e53662: f64 = (10.0 * 2.220446049250313e-16);
        let assign40470_e53665: f64 = (10.0 * 2.220446049250313e-16);
        let assign40470_e53666: f64 = (assign40470_e53662 + assign40470_e53665);
        let assign40470_e53670: f64 = (10.0 * 2.220446049250313e-16);
        let assign40470_e53673: f64 = if ((locals.var_pzadd < assign40470_e53666) && (assign40470_e53670 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1017 = assign40470_e53673;
        locals.var_guard1017_rv = 0.0;

        let (assign40480_e53692, assign40480_e53692_d_n0, assign40480_e53692_d_n2, assign40480_e53692_d_n4, assign40480_e53692_d_n5, assign40480_e53692_d_n6, assign40480_e53692_d_n7, assign40480_e53692_d_n8, assign40480_e53692_d_n9, assign40480_e53692_d_n10, assign40480_e53692_d_n11, assign40480_e53692_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1017 != 0.0)) {
        let assign40480_e53684: f64 = (10.0 * 2.220446049250313e-16);
        let assign40480_e53687: f64 = (10.0 * 2.220446049250313e-16);
        let assign40480_e53688: f64 = (assign40480_e53684 + assign40480_e53687);
        let assign40480_e53690: f64 = (assign40480_e53688 - locals.var_pzadd);
        (assign40480_e53690, (-locals.var_pzadd_dn0), (-locals.var_pzadd_dn2), (-locals.var_pzadd_dn4), (-locals.var_pzadd_dn5), (-locals.var_pzadd_dn6), (-locals.var_pzadd_dn7), (-locals.var_pzadd_dn8), (-locals.var_pzadd_dn9), (-locals.var_pzadd_dn10), (-locals.var_pzadd_dn11), (-locals.var_pzadd_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign40480_e53692;
        locals.var_tmf1_dn0 = assign40480_e53692_d_n0;
        locals.var_tmf1_dn2 = assign40480_e53692_d_n2;
        locals.var_tmf1_dn4 = assign40480_e53692_d_n4;
        locals.var_tmf1_dn5 = assign40480_e53692_d_n5;
        locals.var_tmf1_dn6 = assign40480_e53692_d_n6;
        locals.var_tmf1_dn7 = assign40480_e53692_d_n7;
        locals.var_tmf1_dn8 = assign40480_e53692_d_n8;
        locals.var_tmf1_dn9 = assign40480_e53692_d_n9;
        locals.var_tmf1_dn10 = assign40480_e53692_d_n10;
        locals.var_tmf1_dn11 = assign40480_e53692_d_n11;
        locals.var_tmf1_dn14 = assign40480_e53692_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign40490_e53705, assign40490_e53705_d_n0, assign40490_e53705_d_n2, assign40490_e53705_d_n4, assign40490_e53705_d_n5, assign40490_e53705_d_n6, assign40490_e53705_d_n7, assign40490_e53705_d_n8, assign40490_e53705_d_n9, assign40490_e53705_d_n10, assign40490_e53705_d_n11, assign40490_e53705_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1017 != 0.0)) {
        let assign40490_e53703: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign40490_e53703, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign40490_e53705;
        locals.var_x2_dn0 = assign40490_e53705_d_n0;
        locals.var_x2_dn2 = assign40490_e53705_d_n2;
        locals.var_x2_dn4 = assign40490_e53705_d_n4;
        locals.var_x2_dn5 = assign40490_e53705_d_n5;
        locals.var_x2_dn6 = assign40490_e53705_d_n6;
        locals.var_x2_dn7 = assign40490_e53705_d_n7;
        locals.var_x2_dn8 = assign40490_e53705_d_n8;
        locals.var_x2_dn9 = assign40490_e53705_d_n9;
        locals.var_x2_dn10 = assign40490_e53705_d_n10;
        locals.var_x2_dn11 = assign40490_e53705_d_n11;
        locals.var_x2_dn14 = assign40490_e53705_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign40500_e53722, assign40500_e53722_d_n0, assign40500_e53722_d_n2, assign40500_e53722_d_n4, assign40500_e53722_d_n5, assign40500_e53722_d_n6, assign40500_e53722_d_n7, assign40500_e53722_d_n8, assign40500_e53722_d_n9, assign40500_e53722_d_n10, assign40500_e53722_d_n11, assign40500_e53722_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1017 != 0.0)) {
        let assign40500_e53716: f64 = (10.0 * 2.220446049250313e-16);
        let assign40500_e53719: f64 = (10.0 * 2.220446049250313e-16);
        let assign40500_e53720: f64 = (assign40500_e53716 * assign40500_e53719);
        (assign40500_e53720, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign40500_e53722;
        locals.var_xmax2_dn0 = assign40500_e53722_d_n0;
        locals.var_xmax2_dn2 = assign40500_e53722_d_n2;
        locals.var_xmax2_dn4 = assign40500_e53722_d_n4;
        locals.var_xmax2_dn5 = assign40500_e53722_d_n5;
        locals.var_xmax2_dn6 = assign40500_e53722_d_n6;
        locals.var_xmax2_dn7 = assign40500_e53722_d_n7;
        locals.var_xmax2_dn8 = assign40500_e53722_d_n8;
        locals.var_xmax2_dn9 = assign40500_e53722_d_n9;
        locals.var_xmax2_dn10 = assign40500_e53722_d_n10;
        locals.var_xmax2_dn11 = assign40500_e53722_d_n11;
        locals.var_xmax2_dn14 = assign40500_e53722_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign40510_e53733, assign40510_e53733_d_n0, assign40510_e53733_d_n2, assign40510_e53733_d_n4, assign40510_e53733_d_n5, assign40510_e53733_d_n6, assign40510_e53733_d_n7, assign40510_e53733_d_n8, assign40510_e53733_d_n9, assign40510_e53733_d_n10, assign40510_e53733_d_n11, assign40510_e53733_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1017 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign40510_e53733;
        locals.var_xp_dn0 = assign40510_e53733_d_n0;
        locals.var_xp_dn2 = assign40510_e53733_d_n2;
        locals.var_xp_dn4 = assign40510_e53733_d_n4;
        locals.var_xp_dn5 = assign40510_e53733_d_n5;
        locals.var_xp_dn6 = assign40510_e53733_d_n6;
        locals.var_xp_dn7 = assign40510_e53733_d_n7;
        locals.var_xp_dn8 = assign40510_e53733_d_n8;
        locals.var_xp_dn9 = assign40510_e53733_d_n9;
        locals.var_xp_dn10 = assign40510_e53733_d_n10;
        locals.var_xp_dn11 = assign40510_e53733_d_n11;
        locals.var_xp_dn14 = assign40510_e53733_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign40520_e53744, assign40520_e53744_d_n0, assign40520_e53744_d_n2, assign40520_e53744_d_n4, assign40520_e53744_d_n5, assign40520_e53744_d_n6, assign40520_e53744_d_n7, assign40520_e53744_d_n8, assign40520_e53744_d_n9, assign40520_e53744_d_n10, assign40520_e53744_d_n11, assign40520_e53744_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1017 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign40520_e53744;
        locals.var_xmp_dn0 = assign40520_e53744_d_n0;
        locals.var_xmp_dn2 = assign40520_e53744_d_n2;
        locals.var_xmp_dn4 = assign40520_e53744_d_n4;
        locals.var_xmp_dn5 = assign40520_e53744_d_n5;
        locals.var_xmp_dn6 = assign40520_e53744_d_n6;
        locals.var_xmp_dn7 = assign40520_e53744_d_n7;
        locals.var_xmp_dn8 = assign40520_e53744_d_n8;
        locals.var_xmp_dn9 = assign40520_e53744_d_n9;
        locals.var_xmp_dn10 = assign40520_e53744_d_n10;
        locals.var_xmp_dn11 = assign40520_e53744_d_n11;
        locals.var_xmp_dn14 = assign40520_e53744_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign40530_e53755,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1017 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign40530_e53755;
        locals.var_m0_rv = 0.0;

        let (assign40540_e53766,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1017 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign40540_e53766;
        locals.var_mm_rv = 0.0;

        let (assign40550_e53777, assign40550_e53777_d_n0, assign40550_e53777_d_n2, assign40550_e53777_d_n4, assign40550_e53777_d_n5, assign40550_e53777_d_n6, assign40550_e53777_d_n7, assign40550_e53777_d_n8, assign40550_e53777_d_n9, assign40550_e53777_d_n10, assign40550_e53777_d_n11, assign40550_e53777_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1017 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign40550_e53777;
        locals.var_arg_dn0 = assign40550_e53777_d_n0;
        locals.var_arg_dn2 = assign40550_e53777_d_n2;
        locals.var_arg_dn4 = assign40550_e53777_d_n4;
        locals.var_arg_dn5 = assign40550_e53777_d_n5;
        locals.var_arg_dn6 = assign40550_e53777_d_n6;
        locals.var_arg_dn7 = assign40550_e53777_d_n7;
        locals.var_arg_dn8 = assign40550_e53777_d_n8;
        locals.var_arg_dn9 = assign40550_e53777_d_n9;
        locals.var_arg_dn10 = assign40550_e53777_d_n10;
        locals.var_arg_dn11 = assign40550_e53777_d_n11;
        locals.var_arg_dn14 = assign40550_e53777_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign40560_e53788, assign40560_e53788_d_n0, assign40560_e53788_d_n2, assign40560_e53788_d_n4, assign40560_e53788_d_n5, assign40560_e53788_d_n6, assign40560_e53788_d_n7, assign40560_e53788_d_n8, assign40560_e53788_d_n9, assign40560_e53788_d_n10, assign40560_e53788_d_n11, assign40560_e53788_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1017 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign40560_e53788;
        locals.var_dnm_dn0 = assign40560_e53788_d_n0;
        locals.var_dnm_dn2 = assign40560_e53788_d_n2;
        locals.var_dnm_dn4 = assign40560_e53788_d_n4;
        locals.var_dnm_dn5 = assign40560_e53788_d_n5;
        locals.var_dnm_dn6 = assign40560_e53788_d_n6;
        locals.var_dnm_dn7 = assign40560_e53788_d_n7;
        locals.var_dnm_dn8 = assign40560_e53788_d_n8;
        locals.var_dnm_dn9 = assign40560_e53788_d_n9;
        locals.var_dnm_dn10 = assign40560_e53788_d_n10;
        locals.var_dnm_dn11 = assign40560_e53788_d_n11;
        locals.var_dnm_dn14 = assign40560_e53788_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign40570_e53801, assign40570_e53801_d_n0, assign40570_e53801_d_n2, assign40570_e53801_d_n4, assign40570_e53801_d_n5, assign40570_e53801_d_n6, assign40570_e53801_d_n7, assign40570_e53801_d_n8, assign40570_e53801_d_n9, assign40570_e53801_d_n10, assign40570_e53801_d_n11, assign40570_e53801_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1017 != 0.0)) {
        let assign40570_e53799: f64 = (locals.var_xp * locals.var_x2);
        (assign40570_e53799, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign40570_e53801;
        locals.var_xp_dn0 = assign40570_e53801_d_n0;
        locals.var_xp_dn2 = assign40570_e53801_d_n2;
        locals.var_xp_dn4 = assign40570_e53801_d_n4;
        locals.var_xp_dn5 = assign40570_e53801_d_n5;
        locals.var_xp_dn6 = assign40570_e53801_d_n6;
        locals.var_xp_dn7 = assign40570_e53801_d_n7;
        locals.var_xp_dn8 = assign40570_e53801_d_n8;
        locals.var_xp_dn9 = assign40570_e53801_d_n9;
        locals.var_xp_dn10 = assign40570_e53801_d_n10;
        locals.var_xp_dn11 = assign40570_e53801_d_n11;
        locals.var_xp_dn14 = assign40570_e53801_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign40580_e53814, assign40580_e53814_d_n0, assign40580_e53814_d_n2, assign40580_e53814_d_n4, assign40580_e53814_d_n5, assign40580_e53814_d_n6, assign40580_e53814_d_n7, assign40580_e53814_d_n8, assign40580_e53814_d_n9, assign40580_e53814_d_n10, assign40580_e53814_d_n11, assign40580_e53814_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1017 != 0.0)) {
        let assign40580_e53812: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign40580_e53812, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign40580_e53814;
        locals.var_xmp_dn0 = assign40580_e53814_d_n0;
        locals.var_xmp_dn2 = assign40580_e53814_d_n2;
        locals.var_xmp_dn4 = assign40580_e53814_d_n4;
        locals.var_xmp_dn5 = assign40580_e53814_d_n5;
        locals.var_xmp_dn6 = assign40580_e53814_d_n6;
        locals.var_xmp_dn7 = assign40580_e53814_d_n7;
        locals.var_xmp_dn8 = assign40580_e53814_d_n8;
        locals.var_xmp_dn9 = assign40580_e53814_d_n9;
        locals.var_xmp_dn10 = assign40580_e53814_d_n10;
        locals.var_xmp_dn11 = assign40580_e53814_d_n11;
        locals.var_xmp_dn14 = assign40580_e53814_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign40590_e53827, assign40590_e53827_d_n0, assign40590_e53827_d_n2, assign40590_e53827_d_n4, assign40590_e53827_d_n5, assign40590_e53827_d_n6, assign40590_e53827_d_n7, assign40590_e53827_d_n8, assign40590_e53827_d_n9, assign40590_e53827_d_n10, assign40590_e53827_d_n11, assign40590_e53827_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1017 != 0.0)) {
        let assign40590_e53825: f64 = (locals.var_xp * locals.var_x2);
        (assign40590_e53825, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign40590_e53827;
        locals.var_xp_dn0 = assign40590_e53827_d_n0;
        locals.var_xp_dn2 = assign40590_e53827_d_n2;
        locals.var_xp_dn4 = assign40590_e53827_d_n4;
        locals.var_xp_dn5 = assign40590_e53827_d_n5;
        locals.var_xp_dn6 = assign40590_e53827_d_n6;
        locals.var_xp_dn7 = assign40590_e53827_d_n7;
        locals.var_xp_dn8 = assign40590_e53827_d_n8;
        locals.var_xp_dn9 = assign40590_e53827_d_n9;
        locals.var_xp_dn10 = assign40590_e53827_d_n10;
        locals.var_xp_dn11 = assign40590_e53827_d_n11;
        locals.var_xp_dn14 = assign40590_e53827_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign40600_e53840, assign40600_e53840_d_n0, assign40600_e53840_d_n2, assign40600_e53840_d_n4, assign40600_e53840_d_n5, assign40600_e53840_d_n6, assign40600_e53840_d_n7, assign40600_e53840_d_n8, assign40600_e53840_d_n9, assign40600_e53840_d_n10, assign40600_e53840_d_n11, assign40600_e53840_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1017 != 0.0)) {
        let assign40600_e53838: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign40600_e53838, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign40600_e53840;
        locals.var_xmp_dn0 = assign40600_e53840_d_n0;
        locals.var_xmp_dn2 = assign40600_e53840_d_n2;
        locals.var_xmp_dn4 = assign40600_e53840_d_n4;
        locals.var_xmp_dn5 = assign40600_e53840_d_n5;
        locals.var_xmp_dn6 = assign40600_e53840_d_n6;
        locals.var_xmp_dn7 = assign40600_e53840_d_n7;
        locals.var_xmp_dn8 = assign40600_e53840_d_n8;
        locals.var_xmp_dn9 = assign40600_e53840_d_n9;
        locals.var_xmp_dn10 = assign40600_e53840_d_n10;
        locals.var_xmp_dn11 = assign40600_e53840_d_n11;
        locals.var_xmp_dn14 = assign40600_e53840_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign40610_e53853, assign40610_e53853_d_n0, assign40610_e53853_d_n2, assign40610_e53853_d_n4, assign40610_e53853_d_n5, assign40610_e53853_d_n6, assign40610_e53853_d_n7, assign40610_e53853_d_n8, assign40610_e53853_d_n9, assign40610_e53853_d_n10, assign40610_e53853_d_n11, assign40610_e53853_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1017 != 0.0)) {
        let assign40610_e53851: f64 = (locals.var_xp + locals.var_xmp);
        (assign40610_e53851, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign40610_e53853;
        locals.var_arg_dn0 = assign40610_e53853_d_n0;
        locals.var_arg_dn2 = assign40610_e53853_d_n2;
        locals.var_arg_dn4 = assign40610_e53853_d_n4;
        locals.var_arg_dn5 = assign40610_e53853_d_n5;
        locals.var_arg_dn6 = assign40610_e53853_d_n6;
        locals.var_arg_dn7 = assign40610_e53853_d_n7;
        locals.var_arg_dn8 = assign40610_e53853_d_n8;
        locals.var_arg_dn9 = assign40610_e53853_d_n9;
        locals.var_arg_dn10 = assign40610_e53853_d_n10;
        locals.var_arg_dn11 = assign40610_e53853_d_n11;
        locals.var_arg_dn14 = assign40610_e53853_d_n14;
        locals.var_arg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_139(
        locals: &mut StampLocals,
    ) {
        let (assign40620_e53864, assign40620_e53864_d_n0, assign40620_e53864_d_n2, assign40620_e53864_d_n4, assign40620_e53864_d_n5, assign40620_e53864_d_n6, assign40620_e53864_d_n7, assign40620_e53864_d_n8, assign40620_e53864_d_n9, assign40620_e53864_d_n10, assign40620_e53864_d_n11, assign40620_e53864_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1017 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign40620_e53864;
        locals.var_dnm_dn0 = assign40620_e53864_d_n0;
        locals.var_dnm_dn2 = assign40620_e53864_d_n2;
        locals.var_dnm_dn4 = assign40620_e53864_d_n4;
        locals.var_dnm_dn5 = assign40620_e53864_d_n5;
        locals.var_dnm_dn6 = assign40620_e53864_d_n6;
        locals.var_dnm_dn7 = assign40620_e53864_d_n7;
        locals.var_dnm_dn8 = assign40620_e53864_d_n8;
        locals.var_dnm_dn9 = assign40620_e53864_d_n9;
        locals.var_dnm_dn10 = assign40620_e53864_d_n10;
        locals.var_dnm_dn11 = assign40620_e53864_d_n11;
        locals.var_dnm_dn14 = assign40620_e53864_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign40630_e53879: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1018 = assign40630_e53879;
        locals.var_guard1018_rv = 0.0;

        let assign40640_e53882: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1019 = assign40640_e53882;
        locals.var_guard1019_rv = 0.0;

        let (assign40650_e53897,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1017 != 0.0)) && (locals.var_guard1018 != 0.0)) && (locals.var_guard1019 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign40650_e53897;
        locals.var_mm_rv = 0.0;

        let assign40660_e53900: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1020 = assign40660_e53900;
        locals.var_guard1020_rv = 0.0;

        let (assign40670_e53918,) = {
    if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1017 != 0.0)) && (locals.var_guard1018 != 0.0)) && (locals.var_guard1019 == 0.0)) && (locals.var_guard1020 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign40670_e53918;
        locals.var_mm_rv = 0.0;

        let assign40680_e53921: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1021 = assign40680_e53921;
        locals.var_guard1021_rv = 0.0;

        let (assign40690_e53942,) = {
    if (((((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1017 != 0.0)) && (locals.var_guard1018 != 0.0)) && (locals.var_guard1019 == 0.0)) && (locals.var_guard1020 == 0.0)) && (locals.var_guard1021 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign40690_e53942;
        locals.var_mm_rv = 0.0;

        let assign40700_e53945: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1022 = assign40700_e53945;
        locals.var_guard1022_rv = 0.0;

        let (assign40710_e53969,) = {
    if ((((((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1017 != 0.0)) && (locals.var_guard1018 != 0.0)) && (locals.var_guard1019 == 0.0)) && (locals.var_guard1020 == 0.0)) && (locals.var_guard1021 == 0.0)) && (locals.var_guard1022 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign40710_e53969;
        locals.var_mm_rv = 0.0;

        let (assign40720_e53982,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1017 != 0.0)) && (locals.var_guard1018 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign40720_e53982;
        locals.var_m0_rv = 0.0;

        let mut assign40730_loop_guard: usize = 0;
        while {
            let assign40730_cond_e53996: f64 = if (((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1017 != 0.0)) && (locals.var_guard1018 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign40730_cond_e53996 != 0.0
        } {
            assign40730_loop_guard += 1;
            assert!(assign40730_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign40730_body0_e54010, assign40730_body0_e54010_d_n0, assign40730_body0_e54010_d_n2, assign40730_body0_e54010_d_n4, assign40730_body0_e54010_d_n5, assign40730_body0_e54010_d_n6, assign40730_body0_e54010_d_n7, assign40730_body0_e54010_d_n8, assign40730_body0_e54010_d_n9, assign40730_body0_e54010_d_n10, assign40730_body0_e54010_d_n11, assign40730_body0_e54010_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1017 != 0.0)) && (locals.var_guard1018 != 0.0)) {
        let assign40730_body0_e54008: f64 = (locals.var_dnm).sqrt();
        (assign40730_body0_e54008, (locals.var_dnm_dn0 / (2.0 * assign40730_body0_e54008)), (locals.var_dnm_dn2 / (2.0 * assign40730_body0_e54008)), (locals.var_dnm_dn4 / (2.0 * assign40730_body0_e54008)), (locals.var_dnm_dn5 / (2.0 * assign40730_body0_e54008)), (locals.var_dnm_dn6 / (2.0 * assign40730_body0_e54008)), (locals.var_dnm_dn7 / (2.0 * assign40730_body0_e54008)), (locals.var_dnm_dn8 / (2.0 * assign40730_body0_e54008)), (locals.var_dnm_dn9 / (2.0 * assign40730_body0_e54008)), (locals.var_dnm_dn10 / (2.0 * assign40730_body0_e54008)), (locals.var_dnm_dn11 / (2.0 * assign40730_body0_e54008)), (locals.var_dnm_dn14 / (2.0 * assign40730_body0_e54008)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign40730_body0_e54010;
            locals.var_dnm_dn0 = assign40730_body0_e54010_d_n0;
            locals.var_dnm_dn2 = assign40730_body0_e54010_d_n2;
            locals.var_dnm_dn4 = assign40730_body0_e54010_d_n4;
            locals.var_dnm_dn5 = assign40730_body0_e54010_d_n5;
            locals.var_dnm_dn6 = assign40730_body0_e54010_d_n6;
            locals.var_dnm_dn7 = assign40730_body0_e54010_d_n7;
            locals.var_dnm_dn8 = assign40730_body0_e54010_d_n8;
            locals.var_dnm_dn9 = assign40730_body0_e54010_d_n9;
            locals.var_dnm_dn10 = assign40730_body0_e54010_d_n10;
            locals.var_dnm_dn11 = assign40730_body0_e54010_d_n11;
            locals.var_dnm_dn14 = assign40730_body0_e54010_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign40730_body1_e54025,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1017 != 0.0)) && (locals.var_guard1018 != 0.0)) {
        let assign40730_body1_e54023: f64 = (locals.var_m0 + 1.0);
        (assign40730_body1_e54023,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign40730_body1_e54025;
            locals.var_m0_rv = 0.0;
        }

        let (assign40740_e54050, assign40740_e54050_d_n0, assign40740_e54050_d_n2, assign40740_e54050_d_n4, assign40740_e54050_d_n5, assign40740_e54050_d_n6, assign40740_e54050_d_n7, assign40740_e54050_d_n8, assign40740_e54050_d_n9, assign40740_e54050_d_n10, assign40740_e54050_d_n11, assign40740_e54050_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1017 != 0.0)) && (locals.var_guard1018 == 0.0)) {
        let (assign40740_e54048, assign40740_e54048_d_n0, assign40740_e54048_d_n2, assign40740_e54048_d_n4, assign40740_e54048_d_n5, assign40740_e54048_d_n6, assign40740_e54048_d_n7, assign40740_e54048_d_n8, assign40740_e54048_d_n9, assign40740_e54048_d_n10, assign40740_e54048_d_n11, assign40740_e54048_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign40740_e54045: f64 = (2.0 * 2.0);
                let assign40740_e54046: f64 = (1.0 / assign40740_e54045);
                let assign40740_e54047: f64 = (locals.var_dnm).powf(assign40740_e54046);
                (assign40740_e54047, if 0.0 == 0.0 && ((assign40740_e54046) as f64).is_finite() && ((assign40740_e54046) as f64).fract() == 0.0 { if assign40740_e54046 == 0.0 { 0.0 } else { (assign40740_e54046 * ((locals.var_dnm).powf(assign40740_e54046 - 1.0) * locals.var_dnm_dn0)) } } else { (assign40740_e54047 * (assign40740_e54046 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign40740_e54046) as f64).is_finite() && ((assign40740_e54046) as f64).fract() == 0.0 { if assign40740_e54046 == 0.0 { 0.0 } else { (assign40740_e54046 * ((locals.var_dnm).powf(assign40740_e54046 - 1.0) * locals.var_dnm_dn2)) } } else { (assign40740_e54047 * (assign40740_e54046 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign40740_e54046) as f64).is_finite() && ((assign40740_e54046) as f64).fract() == 0.0 { if assign40740_e54046 == 0.0 { 0.0 } else { (assign40740_e54046 * ((locals.var_dnm).powf(assign40740_e54046 - 1.0) * locals.var_dnm_dn4)) } } else { (assign40740_e54047 * (assign40740_e54046 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign40740_e54046) as f64).is_finite() && ((assign40740_e54046) as f64).fract() == 0.0 { if assign40740_e54046 == 0.0 { 0.0 } else { (assign40740_e54046 * ((locals.var_dnm).powf(assign40740_e54046 - 1.0) * locals.var_dnm_dn5)) } } else { (assign40740_e54047 * (assign40740_e54046 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign40740_e54046) as f64).is_finite() && ((assign40740_e54046) as f64).fract() == 0.0 { if assign40740_e54046 == 0.0 { 0.0 } else { (assign40740_e54046 * ((locals.var_dnm).powf(assign40740_e54046 - 1.0) * locals.var_dnm_dn6)) } } else { (assign40740_e54047 * (assign40740_e54046 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign40740_e54046) as f64).is_finite() && ((assign40740_e54046) as f64).fract() == 0.0 { if assign40740_e54046 == 0.0 { 0.0 } else { (assign40740_e54046 * ((locals.var_dnm).powf(assign40740_e54046 - 1.0) * locals.var_dnm_dn7)) } } else { (assign40740_e54047 * (assign40740_e54046 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign40740_e54046) as f64).is_finite() && ((assign40740_e54046) as f64).fract() == 0.0 { if assign40740_e54046 == 0.0 { 0.0 } else { (assign40740_e54046 * ((locals.var_dnm).powf(assign40740_e54046 - 1.0) * locals.var_dnm_dn8)) } } else { (assign40740_e54047 * (assign40740_e54046 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign40740_e54046) as f64).is_finite() && ((assign40740_e54046) as f64).fract() == 0.0 { if assign40740_e54046 == 0.0 { 0.0 } else { (assign40740_e54046 * ((locals.var_dnm).powf(assign40740_e54046 - 1.0) * locals.var_dnm_dn9)) } } else { (assign40740_e54047 * (assign40740_e54046 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign40740_e54046) as f64).is_finite() && ((assign40740_e54046) as f64).fract() == 0.0 { if assign40740_e54046 == 0.0 { 0.0 } else { (assign40740_e54046 * ((locals.var_dnm).powf(assign40740_e54046 - 1.0) * locals.var_dnm_dn10)) } } else { (assign40740_e54047 * (assign40740_e54046 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign40740_e54046) as f64).is_finite() && ((assign40740_e54046) as f64).fract() == 0.0 { if assign40740_e54046 == 0.0 { 0.0 } else { (assign40740_e54046 * ((locals.var_dnm).powf(assign40740_e54046 - 1.0) * locals.var_dnm_dn11)) } } else { (assign40740_e54047 * (assign40740_e54046 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign40740_e54046) as f64).is_finite() && ((assign40740_e54046) as f64).fract() == 0.0 { if assign40740_e54046 == 0.0 { 0.0 } else { (assign40740_e54046 * ((locals.var_dnm).powf(assign40740_e54046 - 1.0) * locals.var_dnm_dn14)) } } else { (assign40740_e54047 * (assign40740_e54046 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign40740_e54048, assign40740_e54048_d_n0, assign40740_e54048_d_n2, assign40740_e54048_d_n4, assign40740_e54048_d_n5, assign40740_e54048_d_n6, assign40740_e54048_d_n7, assign40740_e54048_d_n8, assign40740_e54048_d_n9, assign40740_e54048_d_n10, assign40740_e54048_d_n11, assign40740_e54048_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign40740_e54050;
        locals.var_dnm_dn0 = assign40740_e54050_d_n0;
        locals.var_dnm_dn2 = assign40740_e54050_d_n2;
        locals.var_dnm_dn4 = assign40740_e54050_d_n4;
        locals.var_dnm_dn5 = assign40740_e54050_d_n5;
        locals.var_dnm_dn6 = assign40740_e54050_d_n6;
        locals.var_dnm_dn7 = assign40740_e54050_d_n7;
        locals.var_dnm_dn8 = assign40740_e54050_d_n8;
        locals.var_dnm_dn9 = assign40740_e54050_d_n9;
        locals.var_dnm_dn10 = assign40740_e54050_d_n10;
        locals.var_dnm_dn11 = assign40740_e54050_d_n11;
        locals.var_dnm_dn14 = assign40740_e54050_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign40750_e54063, assign40750_e54063_d_n0, assign40750_e54063_d_n2, assign40750_e54063_d_n4, assign40750_e54063_d_n5, assign40750_e54063_d_n6, assign40750_e54063_d_n7, assign40750_e54063_d_n8, assign40750_e54063_d_n9, assign40750_e54063_d_n10, assign40750_e54063_d_n11, assign40750_e54063_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1017 != 0.0)) {
        let assign40750_e54061: f64 = (1.0 / locals.var_dnm);
        (assign40750_e54061, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign40750_e54063;
        locals.var_dnm_dn0 = assign40750_e54063_d_n0;
        locals.var_dnm_dn2 = assign40750_e54063_d_n2;
        locals.var_dnm_dn4 = assign40750_e54063_d_n4;
        locals.var_dnm_dn5 = assign40750_e54063_d_n5;
        locals.var_dnm_dn6 = assign40750_e54063_d_n6;
        locals.var_dnm_dn7 = assign40750_e54063_d_n7;
        locals.var_dnm_dn8 = assign40750_e54063_d_n8;
        locals.var_dnm_dn9 = assign40750_e54063_d_n9;
        locals.var_dnm_dn10 = assign40750_e54063_d_n10;
        locals.var_dnm_dn11 = assign40750_e54063_d_n11;
        locals.var_dnm_dn14 = assign40750_e54063_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign40760_e54080, assign40760_e54080_d_n0, assign40760_e54080_d_n2, assign40760_e54080_d_n4, assign40760_e54080_d_n5, assign40760_e54080_d_n6, assign40760_e54080_d_n7, assign40760_e54080_d_n8, assign40760_e54080_d_n9, assign40760_e54080_d_n10, assign40760_e54080_d_n11, assign40760_e54080_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1017 != 0.0)) {
        let assign40760_e54075: f64 = (10.0 * 2.220446049250313e-16);
        let assign40760_e54076: f64 = (locals.var_tmf1 * assign40760_e54075);
        let assign40760_e54078: f64 = (assign40760_e54076 * locals.var_dnm);
        (assign40760_e54078, (((locals.var_tmf1_dn0 * assign40760_e54075) * locals.var_dnm) + (assign40760_e54076 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * assign40760_e54075) * locals.var_dnm) + (assign40760_e54076 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * assign40760_e54075) * locals.var_dnm) + (assign40760_e54076 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * assign40760_e54075) * locals.var_dnm) + (assign40760_e54076 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * assign40760_e54075) * locals.var_dnm) + (assign40760_e54076 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * assign40760_e54075) * locals.var_dnm) + (assign40760_e54076 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * assign40760_e54075) * locals.var_dnm) + (assign40760_e54076 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * assign40760_e54075) * locals.var_dnm) + (assign40760_e54076 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * assign40760_e54075) * locals.var_dnm) + (assign40760_e54076 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * assign40760_e54075) * locals.var_dnm) + (assign40760_e54076 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * assign40760_e54075) * locals.var_dnm) + (assign40760_e54076 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign40760_e54080;
        locals.var_tmf0_dn0 = assign40760_e54080_d_n0;
        locals.var_tmf0_dn2 = assign40760_e54080_d_n2;
        locals.var_tmf0_dn4 = assign40760_e54080_d_n4;
        locals.var_tmf0_dn5 = assign40760_e54080_d_n5;
        locals.var_tmf0_dn6 = assign40760_e54080_d_n6;
        locals.var_tmf0_dn7 = assign40760_e54080_d_n7;
        locals.var_tmf0_dn8 = assign40760_e54080_d_n8;
        locals.var_tmf0_dn9 = assign40760_e54080_d_n9;
        locals.var_tmf0_dn10 = assign40760_e54080_d_n10;
        locals.var_tmf0_dn11 = assign40760_e54080_d_n11;
        locals.var_tmf0_dn14 = assign40760_e54080_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign40770_e54099, assign40770_e54099_d_n0, assign40770_e54099_d_n2, assign40770_e54099_d_n4, assign40770_e54099_d_n5, assign40770_e54099_d_n6, assign40770_e54099_d_n7, assign40770_e54099_d_n8, assign40770_e54099_d_n9, assign40770_e54099_d_n10, assign40770_e54099_d_n11, assign40770_e54099_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1017 != 0.0)) {
        let assign40770_e54091: f64 = (10.0 * 2.220446049250313e-16);
        let assign40770_e54093: f64 = (assign40770_e54091 * locals.var_xmp);
        let assign40770_e54095: f64 = (assign40770_e54093 * locals.var_dnm);
        let assign40770_e54097: f64 = (assign40770_e54095 / locals.var_arg);
        (assign40770_e54097, ((((((assign40770_e54091 * locals.var_xmp_dn0) * locals.var_dnm) + (assign40770_e54093 * locals.var_dnm_dn0)) * locals.var_arg) - (assign40770_e54095 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((assign40770_e54091 * locals.var_xmp_dn2) * locals.var_dnm) + (assign40770_e54093 * locals.var_dnm_dn2)) * locals.var_arg) - (assign40770_e54095 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((assign40770_e54091 * locals.var_xmp_dn4) * locals.var_dnm) + (assign40770_e54093 * locals.var_dnm_dn4)) * locals.var_arg) - (assign40770_e54095 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((assign40770_e54091 * locals.var_xmp_dn5) * locals.var_dnm) + (assign40770_e54093 * locals.var_dnm_dn5)) * locals.var_arg) - (assign40770_e54095 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((assign40770_e54091 * locals.var_xmp_dn6) * locals.var_dnm) + (assign40770_e54093 * locals.var_dnm_dn6)) * locals.var_arg) - (assign40770_e54095 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((assign40770_e54091 * locals.var_xmp_dn7) * locals.var_dnm) + (assign40770_e54093 * locals.var_dnm_dn7)) * locals.var_arg) - (assign40770_e54095 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((assign40770_e54091 * locals.var_xmp_dn8) * locals.var_dnm) + (assign40770_e54093 * locals.var_dnm_dn8)) * locals.var_arg) - (assign40770_e54095 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((assign40770_e54091 * locals.var_xmp_dn9) * locals.var_dnm) + (assign40770_e54093 * locals.var_dnm_dn9)) * locals.var_arg) - (assign40770_e54095 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((assign40770_e54091 * locals.var_xmp_dn10) * locals.var_dnm) + (assign40770_e54093 * locals.var_dnm_dn10)) * locals.var_arg) - (assign40770_e54095 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((assign40770_e54091 * locals.var_xmp_dn11) * locals.var_dnm) + (assign40770_e54093 * locals.var_dnm_dn11)) * locals.var_arg) - (assign40770_e54095 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((assign40770_e54091 * locals.var_xmp_dn14) * locals.var_dnm) + (assign40770_e54093 * locals.var_dnm_dn14)) * locals.var_arg) - (assign40770_e54095 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign40770_e54099;
        locals.var_t0_dn0 = assign40770_e54099_d_n0;
        locals.var_t0_dn2 = assign40770_e54099_d_n2;
        locals.var_t0_dn4 = assign40770_e54099_d_n4;
        locals.var_t0_dn5 = assign40770_e54099_d_n5;
        locals.var_t0_dn6 = assign40770_e54099_d_n6;
        locals.var_t0_dn7 = assign40770_e54099_d_n7;
        locals.var_t0_dn8 = assign40770_e54099_d_n8;
        locals.var_t0_dn9 = assign40770_e54099_d_n9;
        locals.var_t0_dn10 = assign40770_e54099_d_n10;
        locals.var_t0_dn11 = assign40770_e54099_d_n11;
        locals.var_t0_dn14 = assign40770_e54099_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign40780_e54118, assign40780_e54118_d_n0, assign40780_e54118_d_n2, assign40780_e54118_d_n4, assign40780_e54118_d_n5, assign40780_e54118_d_n6, assign40780_e54118_d_n7, assign40780_e54118_d_n8, assign40780_e54118_d_n9, assign40780_e54118_d_n10, assign40780_e54118_d_n11, assign40780_e54118_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1017 != 0.0)) {
        let assign40780_e54110: f64 = (10.0 * 2.220446049250313e-16);
        let assign40780_e54113: f64 = (10.0 * 2.220446049250313e-16);
        let assign40780_e54114: f64 = (assign40780_e54110 + assign40780_e54113);
        let assign40780_e54116: f64 = (assign40780_e54114 - locals.var_tmf0);
        (assign40780_e54116, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_pzadd, locals.var_pzadd_dn0, locals.var_pzadd_dn2, locals.var_pzadd_dn4, locals.var_pzadd_dn5, locals.var_pzadd_dn6, locals.var_pzadd_dn7, locals.var_pzadd_dn8, locals.var_pzadd_dn9, locals.var_pzadd_dn10, locals.var_pzadd_dn11, locals.var_pzadd_dn14,)
    }
};
        locals.var_pzadd = assign40780_e54118;
        locals.var_pzadd_dn0 = assign40780_e54118_d_n0;
        locals.var_pzadd_dn2 = assign40780_e54118_d_n2;
        locals.var_pzadd_dn4 = assign40780_e54118_d_n4;
        locals.var_pzadd_dn5 = assign40780_e54118_d_n5;
        locals.var_pzadd_dn6 = assign40780_e54118_d_n6;
        locals.var_pzadd_dn7 = assign40780_e54118_d_n7;
        locals.var_pzadd_dn8 = assign40780_e54118_d_n8;
        locals.var_pzadd_dn9 = assign40780_e54118_d_n9;
        locals.var_pzadd_dn10 = assign40780_e54118_d_n10;
        locals.var_pzadd_dn11 = assign40780_e54118_d_n11;
        locals.var_pzadd_dn14 = assign40780_e54118_d_n14;
        locals.var_pzadd_rv = 0.0;

        let (assign40790_e54129, assign40790_e54129_d_n0, assign40790_e54129_d_n2, assign40790_e54129_d_n4, assign40790_e54129_d_n5, assign40790_e54129_d_n6, assign40790_e54129_d_n7, assign40790_e54129_d_n8, assign40790_e54129_d_n9, assign40790_e54129_d_n10, assign40790_e54129_d_n11, assign40790_e54129_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1017 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign40790_e54129;
        locals.var_t0_dn0 = assign40790_e54129_d_n0;
        locals.var_t0_dn2 = assign40790_e54129_d_n2;
        locals.var_t0_dn4 = assign40790_e54129_d_n4;
        locals.var_t0_dn5 = assign40790_e54129_d_n5;
        locals.var_t0_dn6 = assign40790_e54129_d_n6;
        locals.var_t0_dn7 = assign40790_e54129_d_n7;
        locals.var_t0_dn8 = assign40790_e54129_d_n8;
        locals.var_t0_dn9 = assign40790_e54129_d_n9;
        locals.var_t0_dn10 = assign40790_e54129_d_n10;
        locals.var_t0_dn11 = assign40790_e54129_d_n11;
        locals.var_t0_dn14 = assign40790_e54129_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign40800_e54141, assign40800_e54141_d_n0, assign40800_e54141_d_n2, assign40800_e54141_d_n4, assign40800_e54141_d_n5, assign40800_e54141_d_n6, assign40800_e54141_d_n7, assign40800_e54141_d_n8, assign40800_e54141_d_n9, assign40800_e54141_d_n10, assign40800_e54141_d_n11, assign40800_e54141_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1017 == 0.0)) {
        (locals.var_pzadd, locals.var_pzadd_dn0, locals.var_pzadd_dn2, locals.var_pzadd_dn4, locals.var_pzadd_dn5, locals.var_pzadd_dn6, locals.var_pzadd_dn7, locals.var_pzadd_dn8, locals.var_pzadd_dn9, locals.var_pzadd_dn10, locals.var_pzadd_dn11, locals.var_pzadd_dn14,)
    } else {
        (locals.var_pzadd, locals.var_pzadd_dn0, locals.var_pzadd_dn2, locals.var_pzadd_dn4, locals.var_pzadd_dn5, locals.var_pzadd_dn6, locals.var_pzadd_dn7, locals.var_pzadd_dn8, locals.var_pzadd_dn9, locals.var_pzadd_dn10, locals.var_pzadd_dn11, locals.var_pzadd_dn14,)
    }
};
        locals.var_pzadd = assign40800_e54141;
        locals.var_pzadd_dn0 = assign40800_e54141_d_n0;
        locals.var_pzadd_dn2 = assign40800_e54141_d_n2;
        locals.var_pzadd_dn4 = assign40800_e54141_d_n4;
        locals.var_pzadd_dn5 = assign40800_e54141_d_n5;
        locals.var_pzadd_dn6 = assign40800_e54141_d_n6;
        locals.var_pzadd_dn7 = assign40800_e54141_d_n7;
        locals.var_pzadd_dn8 = assign40800_e54141_d_n8;
        locals.var_pzadd_dn9 = assign40800_e54141_d_n9;
        locals.var_pzadd_dn10 = assign40800_e54141_d_n10;
        locals.var_pzadd_dn11 = assign40800_e54141_d_n11;
        locals.var_pzadd_dn14 = assign40800_e54141_d_n14;
        locals.var_pzadd_rv = 0.0;

        let (assign40810_e54153, assign40810_e54153_d_n0, assign40810_e54153_d_n2, assign40810_e54153_d_n4, assign40810_e54153_d_n5, assign40810_e54153_d_n6, assign40810_e54153_d_n7, assign40810_e54153_d_n8, assign40810_e54153_d_n9, assign40810_e54153_d_n10, assign40810_e54153_d_n11, assign40810_e54153_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1017 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign40810_e54153;
        locals.var_t0_dn0 = assign40810_e54153_d_n0;
        locals.var_t0_dn2 = assign40810_e54153_d_n2;
        locals.var_t0_dn4 = assign40810_e54153_d_n4;
        locals.var_t0_dn5 = assign40810_e54153_d_n5;
        locals.var_t0_dn6 = assign40810_e54153_d_n6;
        locals.var_t0_dn7 = assign40810_e54153_d_n7;
        locals.var_t0_dn8 = assign40810_e54153_d_n8;
        locals.var_t0_dn9 = assign40810_e54153_d_n9;
        locals.var_t0_dn10 = assign40810_e54153_d_n10;
        locals.var_t0_dn11 = assign40810_e54153_d_n11;
        locals.var_t0_dn14 = assign40810_e54153_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign40820_e54164, assign40820_e54164_d_n0, assign40820_e54164_d_n2, assign40820_e54164_d_n4, assign40820_e54164_d_n5, assign40820_e54164_d_n6, assign40820_e54164_d_n7, assign40820_e54164_d_n8, assign40820_e54164_d_n9, assign40820_e54164_d_n10, assign40820_e54164_d_n11, assign40820_e54164_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        let assign40820_e54162: f64 = (locals.var_ps0 + locals.var_pzadd);
        (assign40820_e54162, (locals.var_ps0_dn0 + locals.var_pzadd_dn0), (locals.var_ps0_dn2 + locals.var_pzadd_dn2), (locals.var_ps0_dn4 + locals.var_pzadd_dn4), (locals.var_ps0_dn5 + locals.var_pzadd_dn5), (locals.var_ps0_dn6 + locals.var_pzadd_dn6), (locals.var_ps0_dn7 + locals.var_pzadd_dn7), (locals.var_ps0_dn8 + locals.var_pzadd_dn8), (locals.var_ps0_dn9 + locals.var_pzadd_dn9), (locals.var_ps0_dn10 + locals.var_pzadd_dn10), (locals.var_ps0_dn11 + locals.var_pzadd_dn11), (locals.var_ps0_dn14 + locals.var_pzadd_dn14),)
    } else {
        (locals.var_ps0z, locals.var_ps0z_dn0, locals.var_ps0z_dn2, locals.var_ps0z_dn4, locals.var_ps0z_dn5, locals.var_ps0z_dn6, locals.var_ps0z_dn7, locals.var_ps0z_dn8, locals.var_ps0z_dn9, locals.var_ps0z_dn10, locals.var_ps0z_dn11, locals.var_ps0z_dn14,)
    }
};
        locals.var_ps0z = assign40820_e54164;
        locals.var_ps0z_dn0 = assign40820_e54164_d_n0;
        locals.var_ps0z_dn2 = assign40820_e54164_d_n2;
        locals.var_ps0z_dn4 = assign40820_e54164_d_n4;
        locals.var_ps0z_dn5 = assign40820_e54164_d_n5;
        locals.var_ps0z_dn6 = assign40820_e54164_d_n6;
        locals.var_ps0z_dn7 = assign40820_e54164_d_n7;
        locals.var_ps0z_dn8 = assign40820_e54164_d_n8;
        locals.var_ps0z_dn9 = assign40820_e54164_d_n9;
        locals.var_ps0z_dn10 = assign40820_e54164_d_n10;
        locals.var_ps0z_dn11 = assign40820_e54164_d_n11;
        locals.var_ps0z_dn14 = assign40820_e54164_d_n14;
        locals.var_ps0z_rv = 0.0;

        let assign40830_e54167: f64 = (locals.var_ps0z - locals.var_vds_maxb0__blk855);
        let assign40830_e54170: f64 = locals.var_ps_delta;
        let assign40830_e54175: f64 = if ((assign40830_e54167 < assign40830_e54170) && (locals.var_ps_delta >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1023 = assign40830_e54175;
        locals.var_guard1023_rv = 0.0;

        let (assign40840_e54192, assign40840_e54192_d_n0, assign40840_e54192_d_n2, assign40840_e54192_d_n4, assign40840_e54192_d_n5, assign40840_e54192_d_n6, assign40840_e54192_d_n7, assign40840_e54192_d_n8, assign40840_e54192_d_n9, assign40840_e54192_d_n10, assign40840_e54192_d_n11, assign40840_e54192_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1023 != 0.0)) {
        let assign40840_e54186: f64 = locals.var_ps_delta;
        let assign40840_e54189: f64 = (locals.var_ps0z - locals.var_vds_maxb0__blk855);
        let assign40840_e54190: f64 = (assign40840_e54186 - assign40840_e54189);
        (assign40840_e54190, (-locals.var_ps0z_dn0), (-locals.var_ps0z_dn2), (-locals.var_ps0z_dn4), (-locals.var_ps0z_dn5), (-locals.var_ps0z_dn6), (-locals.var_ps0z_dn7), (-locals.var_ps0z_dn8), (-locals.var_ps0z_dn9), (-locals.var_ps0z_dn10), (-locals.var_ps0z_dn11), (-locals.var_ps0z_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign40840_e54192;
        locals.var_tmf1_dn0 = assign40840_e54192_d_n0;
        locals.var_tmf1_dn2 = assign40840_e54192_d_n2;
        locals.var_tmf1_dn4 = assign40840_e54192_d_n4;
        locals.var_tmf1_dn5 = assign40840_e54192_d_n5;
        locals.var_tmf1_dn6 = assign40840_e54192_d_n6;
        locals.var_tmf1_dn7 = assign40840_e54192_d_n7;
        locals.var_tmf1_dn8 = assign40840_e54192_d_n8;
        locals.var_tmf1_dn9 = assign40840_e54192_d_n9;
        locals.var_tmf1_dn10 = assign40840_e54192_d_n10;
        locals.var_tmf1_dn11 = assign40840_e54192_d_n11;
        locals.var_tmf1_dn14 = assign40840_e54192_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign40850_e54205, assign40850_e54205_d_n0, assign40850_e54205_d_n2, assign40850_e54205_d_n4, assign40850_e54205_d_n5, assign40850_e54205_d_n6, assign40850_e54205_d_n7, assign40850_e54205_d_n8, assign40850_e54205_d_n9, assign40850_e54205_d_n10, assign40850_e54205_d_n11, assign40850_e54205_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1023 != 0.0)) {
        let assign40850_e54203: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign40850_e54203, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign40850_e54205;
        locals.var_x2_dn0 = assign40850_e54205_d_n0;
        locals.var_x2_dn2 = assign40850_e54205_d_n2;
        locals.var_x2_dn4 = assign40850_e54205_d_n4;
        locals.var_x2_dn5 = assign40850_e54205_d_n5;
        locals.var_x2_dn6 = assign40850_e54205_d_n6;
        locals.var_x2_dn7 = assign40850_e54205_d_n7;
        locals.var_x2_dn8 = assign40850_e54205_d_n8;
        locals.var_x2_dn9 = assign40850_e54205_d_n9;
        locals.var_x2_dn10 = assign40850_e54205_d_n10;
        locals.var_x2_dn11 = assign40850_e54205_d_n11;
        locals.var_x2_dn14 = assign40850_e54205_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign40860_e54218, assign40860_e54218_d_n0, assign40860_e54218_d_n2, assign40860_e54218_d_n4, assign40860_e54218_d_n5, assign40860_e54218_d_n6, assign40860_e54218_d_n7, assign40860_e54218_d_n8, assign40860_e54218_d_n9, assign40860_e54218_d_n10, assign40860_e54218_d_n11, assign40860_e54218_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1023 != 0.0)) {
        let assign40860_e54216: f64 = (locals.var_ps_delta * locals.var_ps_delta);
        (assign40860_e54216, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign40860_e54218;
        locals.var_xmax2_dn0 = assign40860_e54218_d_n0;
        locals.var_xmax2_dn2 = assign40860_e54218_d_n2;
        locals.var_xmax2_dn4 = assign40860_e54218_d_n4;
        locals.var_xmax2_dn5 = assign40860_e54218_d_n5;
        locals.var_xmax2_dn6 = assign40860_e54218_d_n6;
        locals.var_xmax2_dn7 = assign40860_e54218_d_n7;
        locals.var_xmax2_dn8 = assign40860_e54218_d_n8;
        locals.var_xmax2_dn9 = assign40860_e54218_d_n9;
        locals.var_xmax2_dn10 = assign40860_e54218_d_n10;
        locals.var_xmax2_dn11 = assign40860_e54218_d_n11;
        locals.var_xmax2_dn14 = assign40860_e54218_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign40870_e54229, assign40870_e54229_d_n0, assign40870_e54229_d_n2, assign40870_e54229_d_n4, assign40870_e54229_d_n5, assign40870_e54229_d_n6, assign40870_e54229_d_n7, assign40870_e54229_d_n8, assign40870_e54229_d_n9, assign40870_e54229_d_n10, assign40870_e54229_d_n11, assign40870_e54229_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1023 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign40870_e54229;
        locals.var_xp_dn0 = assign40870_e54229_d_n0;
        locals.var_xp_dn2 = assign40870_e54229_d_n2;
        locals.var_xp_dn4 = assign40870_e54229_d_n4;
        locals.var_xp_dn5 = assign40870_e54229_d_n5;
        locals.var_xp_dn6 = assign40870_e54229_d_n6;
        locals.var_xp_dn7 = assign40870_e54229_d_n7;
        locals.var_xp_dn8 = assign40870_e54229_d_n8;
        locals.var_xp_dn9 = assign40870_e54229_d_n9;
        locals.var_xp_dn10 = assign40870_e54229_d_n10;
        locals.var_xp_dn11 = assign40870_e54229_d_n11;
        locals.var_xp_dn14 = assign40870_e54229_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign40880_e54240, assign40880_e54240_d_n0, assign40880_e54240_d_n2, assign40880_e54240_d_n4, assign40880_e54240_d_n5, assign40880_e54240_d_n6, assign40880_e54240_d_n7, assign40880_e54240_d_n8, assign40880_e54240_d_n9, assign40880_e54240_d_n10, assign40880_e54240_d_n11, assign40880_e54240_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1023 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign40880_e54240;
        locals.var_xmp_dn0 = assign40880_e54240_d_n0;
        locals.var_xmp_dn2 = assign40880_e54240_d_n2;
        locals.var_xmp_dn4 = assign40880_e54240_d_n4;
        locals.var_xmp_dn5 = assign40880_e54240_d_n5;
        locals.var_xmp_dn6 = assign40880_e54240_d_n6;
        locals.var_xmp_dn7 = assign40880_e54240_d_n7;
        locals.var_xmp_dn8 = assign40880_e54240_d_n8;
        locals.var_xmp_dn9 = assign40880_e54240_d_n9;
        locals.var_xmp_dn10 = assign40880_e54240_d_n10;
        locals.var_xmp_dn11 = assign40880_e54240_d_n11;
        locals.var_xmp_dn14 = assign40880_e54240_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign40890_e54251,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1023 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign40890_e54251;
        locals.var_m0_rv = 0.0;

        let (assign40900_e54262,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1023 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign40900_e54262;
        locals.var_mm_rv = 0.0;

        let (assign40910_e54273, assign40910_e54273_d_n0, assign40910_e54273_d_n2, assign40910_e54273_d_n4, assign40910_e54273_d_n5, assign40910_e54273_d_n6, assign40910_e54273_d_n7, assign40910_e54273_d_n8, assign40910_e54273_d_n9, assign40910_e54273_d_n10, assign40910_e54273_d_n11, assign40910_e54273_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1023 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign40910_e54273;
        locals.var_arg_dn0 = assign40910_e54273_d_n0;
        locals.var_arg_dn2 = assign40910_e54273_d_n2;
        locals.var_arg_dn4 = assign40910_e54273_d_n4;
        locals.var_arg_dn5 = assign40910_e54273_d_n5;
        locals.var_arg_dn6 = assign40910_e54273_d_n6;
        locals.var_arg_dn7 = assign40910_e54273_d_n7;
        locals.var_arg_dn8 = assign40910_e54273_d_n8;
        locals.var_arg_dn9 = assign40910_e54273_d_n9;
        locals.var_arg_dn10 = assign40910_e54273_d_n10;
        locals.var_arg_dn11 = assign40910_e54273_d_n11;
        locals.var_arg_dn14 = assign40910_e54273_d_n14;
        locals.var_arg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_140(
        locals: &mut StampLocals,
    ) {
        let (assign40920_e54284, assign40920_e54284_d_n0, assign40920_e54284_d_n2, assign40920_e54284_d_n4, assign40920_e54284_d_n5, assign40920_e54284_d_n6, assign40920_e54284_d_n7, assign40920_e54284_d_n8, assign40920_e54284_d_n9, assign40920_e54284_d_n10, assign40920_e54284_d_n11, assign40920_e54284_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1023 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign40920_e54284;
        locals.var_dnm_dn0 = assign40920_e54284_d_n0;
        locals.var_dnm_dn2 = assign40920_e54284_d_n2;
        locals.var_dnm_dn4 = assign40920_e54284_d_n4;
        locals.var_dnm_dn5 = assign40920_e54284_d_n5;
        locals.var_dnm_dn6 = assign40920_e54284_d_n6;
        locals.var_dnm_dn7 = assign40920_e54284_d_n7;
        locals.var_dnm_dn8 = assign40920_e54284_d_n8;
        locals.var_dnm_dn9 = assign40920_e54284_d_n9;
        locals.var_dnm_dn10 = assign40920_e54284_d_n10;
        locals.var_dnm_dn11 = assign40920_e54284_d_n11;
        locals.var_dnm_dn14 = assign40920_e54284_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign40930_e54297, assign40930_e54297_d_n0, assign40930_e54297_d_n2, assign40930_e54297_d_n4, assign40930_e54297_d_n5, assign40930_e54297_d_n6, assign40930_e54297_d_n7, assign40930_e54297_d_n8, assign40930_e54297_d_n9, assign40930_e54297_d_n10, assign40930_e54297_d_n11, assign40930_e54297_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1023 != 0.0)) {
        let assign40930_e54295: f64 = (locals.var_xp * locals.var_x2);
        (assign40930_e54295, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign40930_e54297;
        locals.var_xp_dn0 = assign40930_e54297_d_n0;
        locals.var_xp_dn2 = assign40930_e54297_d_n2;
        locals.var_xp_dn4 = assign40930_e54297_d_n4;
        locals.var_xp_dn5 = assign40930_e54297_d_n5;
        locals.var_xp_dn6 = assign40930_e54297_d_n6;
        locals.var_xp_dn7 = assign40930_e54297_d_n7;
        locals.var_xp_dn8 = assign40930_e54297_d_n8;
        locals.var_xp_dn9 = assign40930_e54297_d_n9;
        locals.var_xp_dn10 = assign40930_e54297_d_n10;
        locals.var_xp_dn11 = assign40930_e54297_d_n11;
        locals.var_xp_dn14 = assign40930_e54297_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign40940_e54310, assign40940_e54310_d_n0, assign40940_e54310_d_n2, assign40940_e54310_d_n4, assign40940_e54310_d_n5, assign40940_e54310_d_n6, assign40940_e54310_d_n7, assign40940_e54310_d_n8, assign40940_e54310_d_n9, assign40940_e54310_d_n10, assign40940_e54310_d_n11, assign40940_e54310_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1023 != 0.0)) {
        let assign40940_e54308: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign40940_e54308, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign40940_e54310;
        locals.var_xmp_dn0 = assign40940_e54310_d_n0;
        locals.var_xmp_dn2 = assign40940_e54310_d_n2;
        locals.var_xmp_dn4 = assign40940_e54310_d_n4;
        locals.var_xmp_dn5 = assign40940_e54310_d_n5;
        locals.var_xmp_dn6 = assign40940_e54310_d_n6;
        locals.var_xmp_dn7 = assign40940_e54310_d_n7;
        locals.var_xmp_dn8 = assign40940_e54310_d_n8;
        locals.var_xmp_dn9 = assign40940_e54310_d_n9;
        locals.var_xmp_dn10 = assign40940_e54310_d_n10;
        locals.var_xmp_dn11 = assign40940_e54310_d_n11;
        locals.var_xmp_dn14 = assign40940_e54310_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign40950_e54323, assign40950_e54323_d_n0, assign40950_e54323_d_n2, assign40950_e54323_d_n4, assign40950_e54323_d_n5, assign40950_e54323_d_n6, assign40950_e54323_d_n7, assign40950_e54323_d_n8, assign40950_e54323_d_n9, assign40950_e54323_d_n10, assign40950_e54323_d_n11, assign40950_e54323_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1023 != 0.0)) {
        let assign40950_e54321: f64 = (locals.var_xp * locals.var_x2);
        (assign40950_e54321, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign40950_e54323;
        locals.var_xp_dn0 = assign40950_e54323_d_n0;
        locals.var_xp_dn2 = assign40950_e54323_d_n2;
        locals.var_xp_dn4 = assign40950_e54323_d_n4;
        locals.var_xp_dn5 = assign40950_e54323_d_n5;
        locals.var_xp_dn6 = assign40950_e54323_d_n6;
        locals.var_xp_dn7 = assign40950_e54323_d_n7;
        locals.var_xp_dn8 = assign40950_e54323_d_n8;
        locals.var_xp_dn9 = assign40950_e54323_d_n9;
        locals.var_xp_dn10 = assign40950_e54323_d_n10;
        locals.var_xp_dn11 = assign40950_e54323_d_n11;
        locals.var_xp_dn14 = assign40950_e54323_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign40960_e54336, assign40960_e54336_d_n0, assign40960_e54336_d_n2, assign40960_e54336_d_n4, assign40960_e54336_d_n5, assign40960_e54336_d_n6, assign40960_e54336_d_n7, assign40960_e54336_d_n8, assign40960_e54336_d_n9, assign40960_e54336_d_n10, assign40960_e54336_d_n11, assign40960_e54336_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1023 != 0.0)) {
        let assign40960_e54334: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign40960_e54334, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign40960_e54336;
        locals.var_xmp_dn0 = assign40960_e54336_d_n0;
        locals.var_xmp_dn2 = assign40960_e54336_d_n2;
        locals.var_xmp_dn4 = assign40960_e54336_d_n4;
        locals.var_xmp_dn5 = assign40960_e54336_d_n5;
        locals.var_xmp_dn6 = assign40960_e54336_d_n6;
        locals.var_xmp_dn7 = assign40960_e54336_d_n7;
        locals.var_xmp_dn8 = assign40960_e54336_d_n8;
        locals.var_xmp_dn9 = assign40960_e54336_d_n9;
        locals.var_xmp_dn10 = assign40960_e54336_d_n10;
        locals.var_xmp_dn11 = assign40960_e54336_d_n11;
        locals.var_xmp_dn14 = assign40960_e54336_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign40970_e54349, assign40970_e54349_d_n0, assign40970_e54349_d_n2, assign40970_e54349_d_n4, assign40970_e54349_d_n5, assign40970_e54349_d_n6, assign40970_e54349_d_n7, assign40970_e54349_d_n8, assign40970_e54349_d_n9, assign40970_e54349_d_n10, assign40970_e54349_d_n11, assign40970_e54349_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1023 != 0.0)) {
        let assign40970_e54347: f64 = (locals.var_xp * locals.var_x2);
        (assign40970_e54347, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign40970_e54349;
        locals.var_xp_dn0 = assign40970_e54349_d_n0;
        locals.var_xp_dn2 = assign40970_e54349_d_n2;
        locals.var_xp_dn4 = assign40970_e54349_d_n4;
        locals.var_xp_dn5 = assign40970_e54349_d_n5;
        locals.var_xp_dn6 = assign40970_e54349_d_n6;
        locals.var_xp_dn7 = assign40970_e54349_d_n7;
        locals.var_xp_dn8 = assign40970_e54349_d_n8;
        locals.var_xp_dn9 = assign40970_e54349_d_n9;
        locals.var_xp_dn10 = assign40970_e54349_d_n10;
        locals.var_xp_dn11 = assign40970_e54349_d_n11;
        locals.var_xp_dn14 = assign40970_e54349_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign40980_e54362, assign40980_e54362_d_n0, assign40980_e54362_d_n2, assign40980_e54362_d_n4, assign40980_e54362_d_n5, assign40980_e54362_d_n6, assign40980_e54362_d_n7, assign40980_e54362_d_n8, assign40980_e54362_d_n9, assign40980_e54362_d_n10, assign40980_e54362_d_n11, assign40980_e54362_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1023 != 0.0)) {
        let assign40980_e54360: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign40980_e54360, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign40980_e54362;
        locals.var_xmp_dn0 = assign40980_e54362_d_n0;
        locals.var_xmp_dn2 = assign40980_e54362_d_n2;
        locals.var_xmp_dn4 = assign40980_e54362_d_n4;
        locals.var_xmp_dn5 = assign40980_e54362_d_n5;
        locals.var_xmp_dn6 = assign40980_e54362_d_n6;
        locals.var_xmp_dn7 = assign40980_e54362_d_n7;
        locals.var_xmp_dn8 = assign40980_e54362_d_n8;
        locals.var_xmp_dn9 = assign40980_e54362_d_n9;
        locals.var_xmp_dn10 = assign40980_e54362_d_n10;
        locals.var_xmp_dn11 = assign40980_e54362_d_n11;
        locals.var_xmp_dn14 = assign40980_e54362_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign40990_e54375, assign40990_e54375_d_n0, assign40990_e54375_d_n2, assign40990_e54375_d_n4, assign40990_e54375_d_n5, assign40990_e54375_d_n6, assign40990_e54375_d_n7, assign40990_e54375_d_n8, assign40990_e54375_d_n9, assign40990_e54375_d_n10, assign40990_e54375_d_n11, assign40990_e54375_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1023 != 0.0)) {
        let assign40990_e54373: f64 = (locals.var_xp * locals.var_x2);
        (assign40990_e54373, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign40990_e54375;
        locals.var_xp_dn0 = assign40990_e54375_d_n0;
        locals.var_xp_dn2 = assign40990_e54375_d_n2;
        locals.var_xp_dn4 = assign40990_e54375_d_n4;
        locals.var_xp_dn5 = assign40990_e54375_d_n5;
        locals.var_xp_dn6 = assign40990_e54375_d_n6;
        locals.var_xp_dn7 = assign40990_e54375_d_n7;
        locals.var_xp_dn8 = assign40990_e54375_d_n8;
        locals.var_xp_dn9 = assign40990_e54375_d_n9;
        locals.var_xp_dn10 = assign40990_e54375_d_n10;
        locals.var_xp_dn11 = assign40990_e54375_d_n11;
        locals.var_xp_dn14 = assign40990_e54375_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign41000_e54388, assign41000_e54388_d_n0, assign41000_e54388_d_n2, assign41000_e54388_d_n4, assign41000_e54388_d_n5, assign41000_e54388_d_n6, assign41000_e54388_d_n7, assign41000_e54388_d_n8, assign41000_e54388_d_n9, assign41000_e54388_d_n10, assign41000_e54388_d_n11, assign41000_e54388_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1023 != 0.0)) {
        let assign41000_e54386: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign41000_e54386, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign41000_e54388;
        locals.var_xmp_dn0 = assign41000_e54388_d_n0;
        locals.var_xmp_dn2 = assign41000_e54388_d_n2;
        locals.var_xmp_dn4 = assign41000_e54388_d_n4;
        locals.var_xmp_dn5 = assign41000_e54388_d_n5;
        locals.var_xmp_dn6 = assign41000_e54388_d_n6;
        locals.var_xmp_dn7 = assign41000_e54388_d_n7;
        locals.var_xmp_dn8 = assign41000_e54388_d_n8;
        locals.var_xmp_dn9 = assign41000_e54388_d_n9;
        locals.var_xmp_dn10 = assign41000_e54388_d_n10;
        locals.var_xmp_dn11 = assign41000_e54388_d_n11;
        locals.var_xmp_dn14 = assign41000_e54388_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign41010_e54401, assign41010_e54401_d_n0, assign41010_e54401_d_n2, assign41010_e54401_d_n4, assign41010_e54401_d_n5, assign41010_e54401_d_n6, assign41010_e54401_d_n7, assign41010_e54401_d_n8, assign41010_e54401_d_n9, assign41010_e54401_d_n10, assign41010_e54401_d_n11, assign41010_e54401_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1023 != 0.0)) {
        let assign41010_e54399: f64 = (locals.var_xp + locals.var_xmp);
        (assign41010_e54399, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign41010_e54401;
        locals.var_arg_dn0 = assign41010_e54401_d_n0;
        locals.var_arg_dn2 = assign41010_e54401_d_n2;
        locals.var_arg_dn4 = assign41010_e54401_d_n4;
        locals.var_arg_dn5 = assign41010_e54401_d_n5;
        locals.var_arg_dn6 = assign41010_e54401_d_n6;
        locals.var_arg_dn7 = assign41010_e54401_d_n7;
        locals.var_arg_dn8 = assign41010_e54401_d_n8;
        locals.var_arg_dn9 = assign41010_e54401_d_n9;
        locals.var_arg_dn10 = assign41010_e54401_d_n10;
        locals.var_arg_dn11 = assign41010_e54401_d_n11;
        locals.var_arg_dn14 = assign41010_e54401_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign41020_e54412, assign41020_e54412_d_n0, assign41020_e54412_d_n2, assign41020_e54412_d_n4, assign41020_e54412_d_n5, assign41020_e54412_d_n6, assign41020_e54412_d_n7, assign41020_e54412_d_n8, assign41020_e54412_d_n9, assign41020_e54412_d_n10, assign41020_e54412_d_n11, assign41020_e54412_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1023 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign41020_e54412;
        locals.var_dnm_dn0 = assign41020_e54412_d_n0;
        locals.var_dnm_dn2 = assign41020_e54412_d_n2;
        locals.var_dnm_dn4 = assign41020_e54412_d_n4;
        locals.var_dnm_dn5 = assign41020_e54412_d_n5;
        locals.var_dnm_dn6 = assign41020_e54412_d_n6;
        locals.var_dnm_dn7 = assign41020_e54412_d_n7;
        locals.var_dnm_dn8 = assign41020_e54412_d_n8;
        locals.var_dnm_dn9 = assign41020_e54412_d_n9;
        locals.var_dnm_dn10 = assign41020_e54412_d_n10;
        locals.var_dnm_dn11 = assign41020_e54412_d_n11;
        locals.var_dnm_dn14 = assign41020_e54412_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign41030_e54427: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1024 = assign41030_e54427;
        locals.var_guard1024_rv = 0.0;

        let assign41040_e54430: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1025 = assign41040_e54430;
        locals.var_guard1025_rv = 0.0;

        let (assign41050_e54445,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1023 != 0.0)) && (locals.var_guard1024 != 0.0)) && (locals.var_guard1025 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign41050_e54445;
        locals.var_mm_rv = 0.0;

        let assign41060_e54448: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1026 = assign41060_e54448;
        locals.var_guard1026_rv = 0.0;

        let (assign41070_e54466,) = {
    if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1023 != 0.0)) && (locals.var_guard1024 != 0.0)) && (locals.var_guard1025 == 0.0)) && (locals.var_guard1026 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign41070_e54466;
        locals.var_mm_rv = 0.0;

        let assign41080_e54469: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1027 = assign41080_e54469;
        locals.var_guard1027_rv = 0.0;

        let (assign41090_e54490,) = {
    if (((((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1023 != 0.0)) && (locals.var_guard1024 != 0.0)) && (locals.var_guard1025 == 0.0)) && (locals.var_guard1026 == 0.0)) && (locals.var_guard1027 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign41090_e54490;
        locals.var_mm_rv = 0.0;

        let assign41100_e54493: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1028 = assign41100_e54493;
        locals.var_guard1028_rv = 0.0;

        let (assign41110_e54517,) = {
    if ((((((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1023 != 0.0)) && (locals.var_guard1024 != 0.0)) && (locals.var_guard1025 == 0.0)) && (locals.var_guard1026 == 0.0)) && (locals.var_guard1027 == 0.0)) && (locals.var_guard1028 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign41110_e54517;
        locals.var_mm_rv = 0.0;

        let (assign41120_e54530,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1023 != 0.0)) && (locals.var_guard1024 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign41120_e54530;
        locals.var_m0_rv = 0.0;

        let mut assign41130_loop_guard: usize = 0;
        while {
            let assign41130_cond_e54544: f64 = if (((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1023 != 0.0)) && (locals.var_guard1024 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign41130_cond_e54544 != 0.0
        } {
            assign41130_loop_guard += 1;
            assert!(assign41130_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign41130_body0_e54558, assign41130_body0_e54558_d_n0, assign41130_body0_e54558_d_n2, assign41130_body0_e54558_d_n4, assign41130_body0_e54558_d_n5, assign41130_body0_e54558_d_n6, assign41130_body0_e54558_d_n7, assign41130_body0_e54558_d_n8, assign41130_body0_e54558_d_n9, assign41130_body0_e54558_d_n10, assign41130_body0_e54558_d_n11, assign41130_body0_e54558_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1023 != 0.0)) && (locals.var_guard1024 != 0.0)) {
        let assign41130_body0_e54556: f64 = (locals.var_dnm).sqrt();
        (assign41130_body0_e54556, (locals.var_dnm_dn0 / (2.0 * assign41130_body0_e54556)), (locals.var_dnm_dn2 / (2.0 * assign41130_body0_e54556)), (locals.var_dnm_dn4 / (2.0 * assign41130_body0_e54556)), (locals.var_dnm_dn5 / (2.0 * assign41130_body0_e54556)), (locals.var_dnm_dn6 / (2.0 * assign41130_body0_e54556)), (locals.var_dnm_dn7 / (2.0 * assign41130_body0_e54556)), (locals.var_dnm_dn8 / (2.0 * assign41130_body0_e54556)), (locals.var_dnm_dn9 / (2.0 * assign41130_body0_e54556)), (locals.var_dnm_dn10 / (2.0 * assign41130_body0_e54556)), (locals.var_dnm_dn11 / (2.0 * assign41130_body0_e54556)), (locals.var_dnm_dn14 / (2.0 * assign41130_body0_e54556)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign41130_body0_e54558;
            locals.var_dnm_dn0 = assign41130_body0_e54558_d_n0;
            locals.var_dnm_dn2 = assign41130_body0_e54558_d_n2;
            locals.var_dnm_dn4 = assign41130_body0_e54558_d_n4;
            locals.var_dnm_dn5 = assign41130_body0_e54558_d_n5;
            locals.var_dnm_dn6 = assign41130_body0_e54558_d_n6;
            locals.var_dnm_dn7 = assign41130_body0_e54558_d_n7;
            locals.var_dnm_dn8 = assign41130_body0_e54558_d_n8;
            locals.var_dnm_dn9 = assign41130_body0_e54558_d_n9;
            locals.var_dnm_dn10 = assign41130_body0_e54558_d_n10;
            locals.var_dnm_dn11 = assign41130_body0_e54558_d_n11;
            locals.var_dnm_dn14 = assign41130_body0_e54558_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign41130_body1_e54573,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1023 != 0.0)) && (locals.var_guard1024 != 0.0)) {
        let assign41130_body1_e54571: f64 = (locals.var_m0 + 1.0);
        (assign41130_body1_e54571,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign41130_body1_e54573;
            locals.var_m0_rv = 0.0;
        }

        let (assign41140_e54598, assign41140_e54598_d_n0, assign41140_e54598_d_n2, assign41140_e54598_d_n4, assign41140_e54598_d_n5, assign41140_e54598_d_n6, assign41140_e54598_d_n7, assign41140_e54598_d_n8, assign41140_e54598_d_n9, assign41140_e54598_d_n10, assign41140_e54598_d_n11, assign41140_e54598_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1023 != 0.0)) && (locals.var_guard1024 == 0.0)) {
        let (assign41140_e54596, assign41140_e54596_d_n0, assign41140_e54596_d_n2, assign41140_e54596_d_n4, assign41140_e54596_d_n5, assign41140_e54596_d_n6, assign41140_e54596_d_n7, assign41140_e54596_d_n8, assign41140_e54596_d_n9, assign41140_e54596_d_n10, assign41140_e54596_d_n11, assign41140_e54596_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign41140_e54593: f64 = (2.0 * 4.0);
                let assign41140_e54594: f64 = (1.0 / assign41140_e54593);
                let assign41140_e54595: f64 = (locals.var_dnm).powf(assign41140_e54594);
                (assign41140_e54595, if 0.0 == 0.0 && ((assign41140_e54594) as f64).is_finite() && ((assign41140_e54594) as f64).fract() == 0.0 { if assign41140_e54594 == 0.0 { 0.0 } else { (assign41140_e54594 * ((locals.var_dnm).powf(assign41140_e54594 - 1.0) * locals.var_dnm_dn0)) } } else { (assign41140_e54595 * (assign41140_e54594 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41140_e54594) as f64).is_finite() && ((assign41140_e54594) as f64).fract() == 0.0 { if assign41140_e54594 == 0.0 { 0.0 } else { (assign41140_e54594 * ((locals.var_dnm).powf(assign41140_e54594 - 1.0) * locals.var_dnm_dn2)) } } else { (assign41140_e54595 * (assign41140_e54594 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41140_e54594) as f64).is_finite() && ((assign41140_e54594) as f64).fract() == 0.0 { if assign41140_e54594 == 0.0 { 0.0 } else { (assign41140_e54594 * ((locals.var_dnm).powf(assign41140_e54594 - 1.0) * locals.var_dnm_dn4)) } } else { (assign41140_e54595 * (assign41140_e54594 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41140_e54594) as f64).is_finite() && ((assign41140_e54594) as f64).fract() == 0.0 { if assign41140_e54594 == 0.0 { 0.0 } else { (assign41140_e54594 * ((locals.var_dnm).powf(assign41140_e54594 - 1.0) * locals.var_dnm_dn5)) } } else { (assign41140_e54595 * (assign41140_e54594 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41140_e54594) as f64).is_finite() && ((assign41140_e54594) as f64).fract() == 0.0 { if assign41140_e54594 == 0.0 { 0.0 } else { (assign41140_e54594 * ((locals.var_dnm).powf(assign41140_e54594 - 1.0) * locals.var_dnm_dn6)) } } else { (assign41140_e54595 * (assign41140_e54594 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41140_e54594) as f64).is_finite() && ((assign41140_e54594) as f64).fract() == 0.0 { if assign41140_e54594 == 0.0 { 0.0 } else { (assign41140_e54594 * ((locals.var_dnm).powf(assign41140_e54594 - 1.0) * locals.var_dnm_dn7)) } } else { (assign41140_e54595 * (assign41140_e54594 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41140_e54594) as f64).is_finite() && ((assign41140_e54594) as f64).fract() == 0.0 { if assign41140_e54594 == 0.0 { 0.0 } else { (assign41140_e54594 * ((locals.var_dnm).powf(assign41140_e54594 - 1.0) * locals.var_dnm_dn8)) } } else { (assign41140_e54595 * (assign41140_e54594 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41140_e54594) as f64).is_finite() && ((assign41140_e54594) as f64).fract() == 0.0 { if assign41140_e54594 == 0.0 { 0.0 } else { (assign41140_e54594 * ((locals.var_dnm).powf(assign41140_e54594 - 1.0) * locals.var_dnm_dn9)) } } else { (assign41140_e54595 * (assign41140_e54594 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41140_e54594) as f64).is_finite() && ((assign41140_e54594) as f64).fract() == 0.0 { if assign41140_e54594 == 0.0 { 0.0 } else { (assign41140_e54594 * ((locals.var_dnm).powf(assign41140_e54594 - 1.0) * locals.var_dnm_dn10)) } } else { (assign41140_e54595 * (assign41140_e54594 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41140_e54594) as f64).is_finite() && ((assign41140_e54594) as f64).fract() == 0.0 { if assign41140_e54594 == 0.0 { 0.0 } else { (assign41140_e54594 * ((locals.var_dnm).powf(assign41140_e54594 - 1.0) * locals.var_dnm_dn11)) } } else { (assign41140_e54595 * (assign41140_e54594 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41140_e54594) as f64).is_finite() && ((assign41140_e54594) as f64).fract() == 0.0 { if assign41140_e54594 == 0.0 { 0.0 } else { (assign41140_e54594 * ((locals.var_dnm).powf(assign41140_e54594 - 1.0) * locals.var_dnm_dn14)) } } else { (assign41140_e54595 * (assign41140_e54594 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign41140_e54596, assign41140_e54596_d_n0, assign41140_e54596_d_n2, assign41140_e54596_d_n4, assign41140_e54596_d_n5, assign41140_e54596_d_n6, assign41140_e54596_d_n7, assign41140_e54596_d_n8, assign41140_e54596_d_n9, assign41140_e54596_d_n10, assign41140_e54596_d_n11, assign41140_e54596_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign41140_e54598;
        locals.var_dnm_dn0 = assign41140_e54598_d_n0;
        locals.var_dnm_dn2 = assign41140_e54598_d_n2;
        locals.var_dnm_dn4 = assign41140_e54598_d_n4;
        locals.var_dnm_dn5 = assign41140_e54598_d_n5;
        locals.var_dnm_dn6 = assign41140_e54598_d_n6;
        locals.var_dnm_dn7 = assign41140_e54598_d_n7;
        locals.var_dnm_dn8 = assign41140_e54598_d_n8;
        locals.var_dnm_dn9 = assign41140_e54598_d_n9;
        locals.var_dnm_dn10 = assign41140_e54598_d_n10;
        locals.var_dnm_dn11 = assign41140_e54598_d_n11;
        locals.var_dnm_dn14 = assign41140_e54598_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign41150_e54611, assign41150_e54611_d_n0, assign41150_e54611_d_n2, assign41150_e54611_d_n4, assign41150_e54611_d_n5, assign41150_e54611_d_n6, assign41150_e54611_d_n7, assign41150_e54611_d_n8, assign41150_e54611_d_n9, assign41150_e54611_d_n10, assign41150_e54611_d_n11, assign41150_e54611_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1023 != 0.0)) {
        let assign41150_e54609: f64 = (1.0 / locals.var_dnm);
        (assign41150_e54609, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign41150_e54611;
        locals.var_dnm_dn0 = assign41150_e54611_d_n0;
        locals.var_dnm_dn2 = assign41150_e54611_d_n2;
        locals.var_dnm_dn4 = assign41150_e54611_d_n4;
        locals.var_dnm_dn5 = assign41150_e54611_d_n5;
        locals.var_dnm_dn6 = assign41150_e54611_d_n6;
        locals.var_dnm_dn7 = assign41150_e54611_d_n7;
        locals.var_dnm_dn8 = assign41150_e54611_d_n8;
        locals.var_dnm_dn9 = assign41150_e54611_d_n9;
        locals.var_dnm_dn10 = assign41150_e54611_d_n10;
        locals.var_dnm_dn11 = assign41150_e54611_d_n11;
        locals.var_dnm_dn14 = assign41150_e54611_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign41160_e54626, assign41160_e54626_d_n0, assign41160_e54626_d_n2, assign41160_e54626_d_n4, assign41160_e54626_d_n5, assign41160_e54626_d_n6, assign41160_e54626_d_n7, assign41160_e54626_d_n8, assign41160_e54626_d_n9, assign41160_e54626_d_n10, assign41160_e54626_d_n11, assign41160_e54626_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1023 != 0.0)) {
        let assign41160_e54622: f64 = (locals.var_tmf1 * locals.var_ps_delta);
        let assign41160_e54624: f64 = (assign41160_e54622 * locals.var_dnm);
        (assign41160_e54624, (((locals.var_tmf1_dn0 * locals.var_ps_delta) * locals.var_dnm) + (assign41160_e54622 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * locals.var_ps_delta) * locals.var_dnm) + (assign41160_e54622 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * locals.var_ps_delta) * locals.var_dnm) + (assign41160_e54622 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * locals.var_ps_delta) * locals.var_dnm) + (assign41160_e54622 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * locals.var_ps_delta) * locals.var_dnm) + (assign41160_e54622 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * locals.var_ps_delta) * locals.var_dnm) + (assign41160_e54622 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * locals.var_ps_delta) * locals.var_dnm) + (assign41160_e54622 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * locals.var_ps_delta) * locals.var_dnm) + (assign41160_e54622 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * locals.var_ps_delta) * locals.var_dnm) + (assign41160_e54622 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * locals.var_ps_delta) * locals.var_dnm) + (assign41160_e54622 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * locals.var_ps_delta) * locals.var_dnm) + (assign41160_e54622 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign41160_e54626;
        locals.var_tmf0_dn0 = assign41160_e54626_d_n0;
        locals.var_tmf0_dn2 = assign41160_e54626_d_n2;
        locals.var_tmf0_dn4 = assign41160_e54626_d_n4;
        locals.var_tmf0_dn5 = assign41160_e54626_d_n5;
        locals.var_tmf0_dn6 = assign41160_e54626_d_n6;
        locals.var_tmf0_dn7 = assign41160_e54626_d_n7;
        locals.var_tmf0_dn8 = assign41160_e54626_d_n8;
        locals.var_tmf0_dn9 = assign41160_e54626_d_n9;
        locals.var_tmf0_dn10 = assign41160_e54626_d_n10;
        locals.var_tmf0_dn11 = assign41160_e54626_d_n11;
        locals.var_tmf0_dn14 = assign41160_e54626_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign41170_e54643, assign41170_e54643_d_n0, assign41170_e54643_d_n2, assign41170_e54643_d_n4, assign41170_e54643_d_n5, assign41170_e54643_d_n6, assign41170_e54643_d_n7, assign41170_e54643_d_n8, assign41170_e54643_d_n9, assign41170_e54643_d_n10, assign41170_e54643_d_n11, assign41170_e54643_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1023 != 0.0)) {
        let assign41170_e54637: f64 = (locals.var_ps_delta * locals.var_xmp);
        let assign41170_e54639: f64 = (assign41170_e54637 * locals.var_dnm);
        let assign41170_e54641: f64 = (assign41170_e54639 / locals.var_arg);
        (assign41170_e54641, ((((((locals.var_ps_delta * locals.var_xmp_dn0) * locals.var_dnm) + (assign41170_e54637 * locals.var_dnm_dn0)) * locals.var_arg) - (assign41170_e54639 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_ps_delta * locals.var_xmp_dn2) * locals.var_dnm) + (assign41170_e54637 * locals.var_dnm_dn2)) * locals.var_arg) - (assign41170_e54639 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_ps_delta * locals.var_xmp_dn4) * locals.var_dnm) + (assign41170_e54637 * locals.var_dnm_dn4)) * locals.var_arg) - (assign41170_e54639 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_ps_delta * locals.var_xmp_dn5) * locals.var_dnm) + (assign41170_e54637 * locals.var_dnm_dn5)) * locals.var_arg) - (assign41170_e54639 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_ps_delta * locals.var_xmp_dn6) * locals.var_dnm) + (assign41170_e54637 * locals.var_dnm_dn6)) * locals.var_arg) - (assign41170_e54639 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_ps_delta * locals.var_xmp_dn7) * locals.var_dnm) + (assign41170_e54637 * locals.var_dnm_dn7)) * locals.var_arg) - (assign41170_e54639 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_ps_delta * locals.var_xmp_dn8) * locals.var_dnm) + (assign41170_e54637 * locals.var_dnm_dn8)) * locals.var_arg) - (assign41170_e54639 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_ps_delta * locals.var_xmp_dn9) * locals.var_dnm) + (assign41170_e54637 * locals.var_dnm_dn9)) * locals.var_arg) - (assign41170_e54639 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_ps_delta * locals.var_xmp_dn10) * locals.var_dnm) + (assign41170_e54637 * locals.var_dnm_dn10)) * locals.var_arg) - (assign41170_e54639 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_ps_delta * locals.var_xmp_dn11) * locals.var_dnm) + (assign41170_e54637 * locals.var_dnm_dn11)) * locals.var_arg) - (assign41170_e54639 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_ps_delta * locals.var_xmp_dn14) * locals.var_dnm) + (assign41170_e54637 * locals.var_dnm_dn14)) * locals.var_arg) - (assign41170_e54639 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign41170_e54643;
        locals.var_t0_dn0 = assign41170_e54643_d_n0;
        locals.var_t0_dn2 = assign41170_e54643_d_n2;
        locals.var_t0_dn4 = assign41170_e54643_d_n4;
        locals.var_t0_dn5 = assign41170_e54643_d_n5;
        locals.var_t0_dn6 = assign41170_e54643_d_n6;
        locals.var_t0_dn7 = assign41170_e54643_d_n7;
        locals.var_t0_dn8 = assign41170_e54643_d_n8;
        locals.var_t0_dn9 = assign41170_e54643_d_n9;
        locals.var_t0_dn10 = assign41170_e54643_d_n10;
        locals.var_t0_dn11 = assign41170_e54643_d_n11;
        locals.var_t0_dn14 = assign41170_e54643_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign41180_e54658, assign41180_e54658_d_n0, assign41180_e54658_d_n2, assign41180_e54658_d_n4, assign41180_e54658_d_n5, assign41180_e54658_d_n6, assign41180_e54658_d_n7, assign41180_e54658_d_n8, assign41180_e54658_d_n9, assign41180_e54658_d_n10, assign41180_e54658_d_n11, assign41180_e54658_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1023 != 0.0)) {
        let assign41180_e54654: f64 = locals.var_ps_delta;
        let assign41180_e54656: f64 = (assign41180_e54654 - locals.var_tmf0);
        (assign41180_e54656, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign41180_e54658;
        locals.var_t2_dn0 = assign41180_e54658_d_n0;
        locals.var_t2_dn2 = assign41180_e54658_d_n2;
        locals.var_t2_dn4 = assign41180_e54658_d_n4;
        locals.var_t2_dn5 = assign41180_e54658_d_n5;
        locals.var_t2_dn6 = assign41180_e54658_d_n6;
        locals.var_t2_dn7 = assign41180_e54658_d_n7;
        locals.var_t2_dn8 = assign41180_e54658_d_n8;
        locals.var_t2_dn9 = assign41180_e54658_d_n9;
        locals.var_t2_dn10 = assign41180_e54658_d_n10;
        locals.var_t2_dn11 = assign41180_e54658_d_n11;
        locals.var_t2_dn14 = assign41180_e54658_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign41190_e54669, assign41190_e54669_d_n0, assign41190_e54669_d_n2, assign41190_e54669_d_n4, assign41190_e54669_d_n5, assign41190_e54669_d_n6, assign41190_e54669_d_n7, assign41190_e54669_d_n8, assign41190_e54669_d_n9, assign41190_e54669_d_n10, assign41190_e54669_d_n11, assign41190_e54669_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1023 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign41190_e54669;
        locals.var_t0_dn0 = assign41190_e54669_d_n0;
        locals.var_t0_dn2 = assign41190_e54669_d_n2;
        locals.var_t0_dn4 = assign41190_e54669_d_n4;
        locals.var_t0_dn5 = assign41190_e54669_d_n5;
        locals.var_t0_dn6 = assign41190_e54669_d_n6;
        locals.var_t0_dn7 = assign41190_e54669_d_n7;
        locals.var_t0_dn8 = assign41190_e54669_d_n8;
        locals.var_t0_dn9 = assign41190_e54669_d_n9;
        locals.var_t0_dn10 = assign41190_e54669_d_n10;
        locals.var_t0_dn11 = assign41190_e54669_d_n11;
        locals.var_t0_dn14 = assign41190_e54669_d_n14;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_141(
        locals: &mut StampLocals,
    ) {
        let (assign41200_e54683, assign41200_e54683_d_n0, assign41200_e54683_d_n2, assign41200_e54683_d_n4, assign41200_e54683_d_n5, assign41200_e54683_d_n6, assign41200_e54683_d_n7, assign41200_e54683_d_n8, assign41200_e54683_d_n9, assign41200_e54683_d_n10, assign41200_e54683_d_n11, assign41200_e54683_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1023 == 0.0)) {
        let assign41200_e54681: f64 = (locals.var_ps0z - locals.var_vds_maxb0__blk855);
        (assign41200_e54681, locals.var_ps0z_dn0, locals.var_ps0z_dn2, locals.var_ps0z_dn4, locals.var_ps0z_dn5, locals.var_ps0z_dn6, locals.var_ps0z_dn7, locals.var_ps0z_dn8, locals.var_ps0z_dn9, locals.var_ps0z_dn10, locals.var_ps0z_dn11, locals.var_ps0z_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign41200_e54683;
        locals.var_t2_dn0 = assign41200_e54683_d_n0;
        locals.var_t2_dn2 = assign41200_e54683_d_n2;
        locals.var_t2_dn4 = assign41200_e54683_d_n4;
        locals.var_t2_dn5 = assign41200_e54683_d_n5;
        locals.var_t2_dn6 = assign41200_e54683_d_n6;
        locals.var_t2_dn7 = assign41200_e54683_d_n7;
        locals.var_t2_dn8 = assign41200_e54683_d_n8;
        locals.var_t2_dn9 = assign41200_e54683_d_n9;
        locals.var_t2_dn10 = assign41200_e54683_d_n10;
        locals.var_t2_dn11 = assign41200_e54683_d_n11;
        locals.var_t2_dn14 = assign41200_e54683_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign41210_e54695, assign41210_e54695_d_n0, assign41210_e54695_d_n2, assign41210_e54695_d_n4, assign41210_e54695_d_n5, assign41210_e54695_d_n6, assign41210_e54695_d_n7, assign41210_e54695_d_n8, assign41210_e54695_d_n9, assign41210_e54695_d_n10, assign41210_e54695_d_n11, assign41210_e54695_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1023 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign41210_e54695;
        locals.var_t0_dn0 = assign41210_e54695_d_n0;
        locals.var_t0_dn2 = assign41210_e54695_d_n2;
        locals.var_t0_dn4 = assign41210_e54695_d_n4;
        locals.var_t0_dn5 = assign41210_e54695_d_n5;
        locals.var_t0_dn6 = assign41210_e54695_d_n6;
        locals.var_t0_dn7 = assign41210_e54695_d_n7;
        locals.var_t0_dn8 = assign41210_e54695_d_n8;
        locals.var_t0_dn9 = assign41210_e54695_d_n9;
        locals.var_t0_dn10 = assign41210_e54695_d_n10;
        locals.var_t0_dn11 = assign41210_e54695_d_n11;
        locals.var_t0_dn14 = assign41210_e54695_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign41220_e54715, assign41220_e54715_d_n0, assign41220_e54715_d_n2, assign41220_e54715_d_n4, assign41220_e54715_d_n5, assign41220_e54715_d_n6, assign41220_e54715_d_n7, assign41220_e54715_d_n8, assign41220_e54715_d_n9, assign41220_e54715_d_n10, assign41220_e54715_d_n11, assign41220_e54715_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        let assign41220_e54704: f64 = (locals.var_beta * locals.var_t2);
        let assign41220_e54705: f64 = (assign41220_e54704).exp();
        let assign41220_e54707: f64 = (assign41220_e54705 - 1.0);
        let assign41220_e54710: f64 = (locals.var_beta * locals.var_t2);
        let assign41220_e54711: f64 = (assign41220_e54707 - assign41220_e54710);
        let assign41220_e54713: f64 = (assign41220_e54711 + 1e-15);
        (assign41220_e54713, ((assign41220_e54705 * ((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0))) - ((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0))), ((assign41220_e54705 * ((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2))) - ((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2))), ((assign41220_e54705 * ((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4))) - ((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4))), ((assign41220_e54705 * ((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5))) - ((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5))), ((assign41220_e54705 * ((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6))) - ((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6))), ((assign41220_e54705 * ((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7))) - ((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7))), ((assign41220_e54705 * ((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8))) - ((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8))), ((assign41220_e54705 * ((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9))) - ((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9))), ((assign41220_e54705 * ((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10))) - ((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10))), ((assign41220_e54705 * ((locals.var_beta_dn11 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn11))) - ((locals.var_beta_dn11 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn11))), ((assign41220_e54705 * ((locals.var_beta_dn14 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn14))) - ((locals.var_beta_dn14 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign41220_e54715;
        locals.var_t4_dn0 = assign41220_e54715_d_n0;
        locals.var_t4_dn2 = assign41220_e54715_d_n2;
        locals.var_t4_dn4 = assign41220_e54715_d_n4;
        locals.var_t4_dn5 = assign41220_e54715_d_n5;
        locals.var_t4_dn6 = assign41220_e54715_d_n6;
        locals.var_t4_dn7 = assign41220_e54715_d_n7;
        locals.var_t4_dn8 = assign41220_e54715_d_n8;
        locals.var_t4_dn9 = assign41220_e54715_d_n9;
        locals.var_t4_dn10 = assign41220_e54715_d_n10;
        locals.var_t4_dn11 = assign41220_e54715_d_n11;
        locals.var_t4_dn14 = assign41220_e54715_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign41230_e54728, assign41230_e54728_d_n0, assign41230_e54728_d_n2, assign41230_e54728_d_n4, assign41230_e54728_d_n5, assign41230_e54728_d_n6, assign41230_e54728_d_n7, assign41230_e54728_d_n8, assign41230_e54728_d_n9, assign41230_e54728_d_n10, assign41230_e54728_d_n11, assign41230_e54728_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        let assign41230_e54723: f64 = (-locals.var_cnst0);
        let assign41230_e54725: f64 = (locals.var_t4).sqrt();
        let assign41230_e54726: f64 = (assign41230_e54723 * assign41230_e54725);
        (assign41230_e54726, (((-locals.var_cnst0_dn0) * assign41230_e54725) + (assign41230_e54723 * (locals.var_t4_dn0 / (2.0 * assign41230_e54725)))), (((-locals.var_cnst0_dn2) * assign41230_e54725) + (assign41230_e54723 * (locals.var_t4_dn2 / (2.0 * assign41230_e54725)))), (((-locals.var_cnst0_dn4) * assign41230_e54725) + (assign41230_e54723 * (locals.var_t4_dn4 / (2.0 * assign41230_e54725)))), (((-locals.var_cnst0_dn5) * assign41230_e54725) + (assign41230_e54723 * (locals.var_t4_dn5 / (2.0 * assign41230_e54725)))), (((-locals.var_cnst0_dn6) * assign41230_e54725) + (assign41230_e54723 * (locals.var_t4_dn6 / (2.0 * assign41230_e54725)))), (((-locals.var_cnst0_dn7) * assign41230_e54725) + (assign41230_e54723 * (locals.var_t4_dn7 / (2.0 * assign41230_e54725)))), (((-locals.var_cnst0_dn8) * assign41230_e54725) + (assign41230_e54723 * (locals.var_t4_dn8 / (2.0 * assign41230_e54725)))), (((-locals.var_cnst0_dn9) * assign41230_e54725) + (assign41230_e54723 * (locals.var_t4_dn9 / (2.0 * assign41230_e54725)))), (((-locals.var_cnst0_dn10) * assign41230_e54725) + (assign41230_e54723 * (locals.var_t4_dn10 / (2.0 * assign41230_e54725)))), (((-locals.var_cnst0_dn11) * assign41230_e54725) + (assign41230_e54723 * (locals.var_t4_dn11 / (2.0 * assign41230_e54725)))), (((-locals.var_cnst0_dn14) * assign41230_e54725) + (assign41230_e54723 * (locals.var_t4_dn14 / (2.0 * assign41230_e54725)))),)
    } else {
        (locals.var_q_n0_sym, locals.var_q_n0_sym_dn0, locals.var_q_n0_sym_dn2, locals.var_q_n0_sym_dn4, locals.var_q_n0_sym_dn5, locals.var_q_n0_sym_dn6, locals.var_q_n0_sym_dn7, locals.var_q_n0_sym_dn8, locals.var_q_n0_sym_dn9, locals.var_q_n0_sym_dn10, locals.var_q_n0_sym_dn11, locals.var_q_n0_sym_dn14,)
    }
};
        locals.var_q_n0_sym = assign41230_e54728;
        locals.var_q_n0_sym_dn0 = assign41230_e54728_d_n0;
        locals.var_q_n0_sym_dn2 = assign41230_e54728_d_n2;
        locals.var_q_n0_sym_dn4 = assign41230_e54728_d_n4;
        locals.var_q_n0_sym_dn5 = assign41230_e54728_d_n5;
        locals.var_q_n0_sym_dn6 = assign41230_e54728_d_n6;
        locals.var_q_n0_sym_dn7 = assign41230_e54728_d_n7;
        locals.var_q_n0_sym_dn8 = assign41230_e54728_d_n8;
        locals.var_q_n0_sym_dn9 = assign41230_e54728_d_n9;
        locals.var_q_n0_sym_dn10 = assign41230_e54728_d_n10;
        locals.var_q_n0_sym_dn11 = assign41230_e54728_d_n11;
        locals.var_q_n0_sym_dn14 = assign41230_e54728_d_n14;
        locals.var_q_n0_sym_rv = 0.0;

        let assign41240_e54731: f64 = if locals.var_w_bsub0__blk840 > locals.var_uc_depthn { 1.0 } else { 0.0 };
        locals.var_guard1034 = assign41240_e54731;
        locals.var_guard1034_rv = 0.0;

        let (assign41250_e54742, assign41250_e54742_d_n0, assign41250_e54742_d_n2, assign41250_e54742_d_n4, assign41250_e54742_d_n5, assign41250_e54742_d_n6, assign41250_e54742_d_n7, assign41250_e54742_d_n8, assign41250_e54742_d_n9, assign41250_e54742_d_n10, assign41250_e54742_d_n11, assign41250_e54742_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 != 0.0)) {
        (locals.var_tnp, locals.var_tnp_dn0, locals.var_tnp_dn2, locals.var_tnp_dn4, locals.var_tnp_dn5, locals.var_tnp_dn6, locals.var_tnp_dn7, locals.var_tnp_dn8, locals.var_tnp_dn9, locals.var_tnp_dn10, locals.var_tnp_dn11, locals.var_tnp_dn14,)
    } else {
        (locals.var_ws, locals.var_ws_dn0, locals.var_ws_dn2, locals.var_ws_dn4, locals.var_ws_dn5, locals.var_ws_dn6, locals.var_ws_dn7, locals.var_ws_dn8, locals.var_ws_dn9, locals.var_ws_dn10, locals.var_ws_dn11, locals.var_ws_dn14,)
    }
};
        locals.var_ws = assign41250_e54742;
        locals.var_ws_dn0 = assign41250_e54742_d_n0;
        locals.var_ws_dn2 = assign41250_e54742_d_n2;
        locals.var_ws_dn4 = assign41250_e54742_d_n4;
        locals.var_ws_dn5 = assign41250_e54742_d_n5;
        locals.var_ws_dn6 = assign41250_e54742_d_n6;
        locals.var_ws_dn7 = assign41250_e54742_d_n7;
        locals.var_ws_dn8 = assign41250_e54742_d_n8;
        locals.var_ws_dn9 = assign41250_e54742_d_n9;
        locals.var_ws_dn10 = assign41250_e54742_d_n10;
        locals.var_ws_dn11 = assign41250_e54742_d_n11;
        locals.var_ws_dn14 = assign41250_e54742_d_n14;
        locals.var_ws_rv = 0.0;

        let assign41260_e54746: f64 = (-0.1);
        let assign41260_e54751: f64 = if ((locals.var_ps0 > assign41260_e54746) && (0.1 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1035 = assign41260_e54751;
        locals.var_guard1035_rv = 0.0;

        let (assign41270_e54769, assign41270_e54769_d_n0, assign41270_e54769_d_n2, assign41270_e54769_d_n4, assign41270_e54769_d_n5, assign41270_e54769_d_n6, assign41270_e54769_d_n7, assign41270_e54769_d_n8, assign41270_e54769_d_n9, assign41270_e54769_d_n10, assign41270_e54769_d_n11, assign41270_e54769_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1035 != 0.0)) {
        let assign41270_e54765: f64 = locals.var_ps0;
        let assign41270_e54767: f64 = (assign41270_e54765 + 0.1);
        (assign41270_e54767, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn4, locals.var_ps0_dn5, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn8, locals.var_ps0_dn9, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign41270_e54769;
        locals.var_tmf1_dn0 = assign41270_e54769_d_n0;
        locals.var_tmf1_dn2 = assign41270_e54769_d_n2;
        locals.var_tmf1_dn4 = assign41270_e54769_d_n4;
        locals.var_tmf1_dn5 = assign41270_e54769_d_n5;
        locals.var_tmf1_dn6 = assign41270_e54769_d_n6;
        locals.var_tmf1_dn7 = assign41270_e54769_d_n7;
        locals.var_tmf1_dn8 = assign41270_e54769_d_n8;
        locals.var_tmf1_dn9 = assign41270_e54769_d_n9;
        locals.var_tmf1_dn10 = assign41270_e54769_d_n10;
        locals.var_tmf1_dn11 = assign41270_e54769_d_n11;
        locals.var_tmf1_dn14 = assign41270_e54769_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign41280_e54785, assign41280_e54785_d_n0, assign41280_e54785_d_n2, assign41280_e54785_d_n4, assign41280_e54785_d_n5, assign41280_e54785_d_n6, assign41280_e54785_d_n7, assign41280_e54785_d_n8, assign41280_e54785_d_n9, assign41280_e54785_d_n10, assign41280_e54785_d_n11, assign41280_e54785_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1035 != 0.0)) {
        let assign41280_e54783: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign41280_e54783, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign41280_e54785;
        locals.var_x2_dn0 = assign41280_e54785_d_n0;
        locals.var_x2_dn2 = assign41280_e54785_d_n2;
        locals.var_x2_dn4 = assign41280_e54785_d_n4;
        locals.var_x2_dn5 = assign41280_e54785_d_n5;
        locals.var_x2_dn6 = assign41280_e54785_d_n6;
        locals.var_x2_dn7 = assign41280_e54785_d_n7;
        locals.var_x2_dn8 = assign41280_e54785_d_n8;
        locals.var_x2_dn9 = assign41280_e54785_d_n9;
        locals.var_x2_dn10 = assign41280_e54785_d_n10;
        locals.var_x2_dn11 = assign41280_e54785_d_n11;
        locals.var_x2_dn14 = assign41280_e54785_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign41290_e54801, assign41290_e54801_d_n0, assign41290_e54801_d_n2, assign41290_e54801_d_n4, assign41290_e54801_d_n5, assign41290_e54801_d_n6, assign41290_e54801_d_n7, assign41290_e54801_d_n8, assign41290_e54801_d_n9, assign41290_e54801_d_n10, assign41290_e54801_d_n11, assign41290_e54801_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1035 != 0.0)) {
        let assign41290_e54799: f64 = (0.1 * 0.1);
        (assign41290_e54799, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign41290_e54801;
        locals.var_xmax2_dn0 = assign41290_e54801_d_n0;
        locals.var_xmax2_dn2 = assign41290_e54801_d_n2;
        locals.var_xmax2_dn4 = assign41290_e54801_d_n4;
        locals.var_xmax2_dn5 = assign41290_e54801_d_n5;
        locals.var_xmax2_dn6 = assign41290_e54801_d_n6;
        locals.var_xmax2_dn7 = assign41290_e54801_d_n7;
        locals.var_xmax2_dn8 = assign41290_e54801_d_n8;
        locals.var_xmax2_dn9 = assign41290_e54801_d_n9;
        locals.var_xmax2_dn10 = assign41290_e54801_d_n10;
        locals.var_xmax2_dn11 = assign41290_e54801_d_n11;
        locals.var_xmax2_dn14 = assign41290_e54801_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign41300_e54815, assign41300_e54815_d_n0, assign41300_e54815_d_n2, assign41300_e54815_d_n4, assign41300_e54815_d_n5, assign41300_e54815_d_n6, assign41300_e54815_d_n7, assign41300_e54815_d_n8, assign41300_e54815_d_n9, assign41300_e54815_d_n10, assign41300_e54815_d_n11, assign41300_e54815_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1035 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign41300_e54815;
        locals.var_xp_dn0 = assign41300_e54815_d_n0;
        locals.var_xp_dn2 = assign41300_e54815_d_n2;
        locals.var_xp_dn4 = assign41300_e54815_d_n4;
        locals.var_xp_dn5 = assign41300_e54815_d_n5;
        locals.var_xp_dn6 = assign41300_e54815_d_n6;
        locals.var_xp_dn7 = assign41300_e54815_d_n7;
        locals.var_xp_dn8 = assign41300_e54815_d_n8;
        locals.var_xp_dn9 = assign41300_e54815_d_n9;
        locals.var_xp_dn10 = assign41300_e54815_d_n10;
        locals.var_xp_dn11 = assign41300_e54815_d_n11;
        locals.var_xp_dn14 = assign41300_e54815_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign41310_e54829, assign41310_e54829_d_n0, assign41310_e54829_d_n2, assign41310_e54829_d_n4, assign41310_e54829_d_n5, assign41310_e54829_d_n6, assign41310_e54829_d_n7, assign41310_e54829_d_n8, assign41310_e54829_d_n9, assign41310_e54829_d_n10, assign41310_e54829_d_n11, assign41310_e54829_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1035 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign41310_e54829;
        locals.var_xmp_dn0 = assign41310_e54829_d_n0;
        locals.var_xmp_dn2 = assign41310_e54829_d_n2;
        locals.var_xmp_dn4 = assign41310_e54829_d_n4;
        locals.var_xmp_dn5 = assign41310_e54829_d_n5;
        locals.var_xmp_dn6 = assign41310_e54829_d_n6;
        locals.var_xmp_dn7 = assign41310_e54829_d_n7;
        locals.var_xmp_dn8 = assign41310_e54829_d_n8;
        locals.var_xmp_dn9 = assign41310_e54829_d_n9;
        locals.var_xmp_dn10 = assign41310_e54829_d_n10;
        locals.var_xmp_dn11 = assign41310_e54829_d_n11;
        locals.var_xmp_dn14 = assign41310_e54829_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign41320_e54843,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1035 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign41320_e54843;
        locals.var_m0_rv = 0.0;

        let (assign41330_e54857,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1035 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign41330_e54857;
        locals.var_mm_rv = 0.0;

        let (assign41340_e54871, assign41340_e54871_d_n0, assign41340_e54871_d_n2, assign41340_e54871_d_n4, assign41340_e54871_d_n5, assign41340_e54871_d_n6, assign41340_e54871_d_n7, assign41340_e54871_d_n8, assign41340_e54871_d_n9, assign41340_e54871_d_n10, assign41340_e54871_d_n11, assign41340_e54871_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1035 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign41340_e54871;
        locals.var_arg_dn0 = assign41340_e54871_d_n0;
        locals.var_arg_dn2 = assign41340_e54871_d_n2;
        locals.var_arg_dn4 = assign41340_e54871_d_n4;
        locals.var_arg_dn5 = assign41340_e54871_d_n5;
        locals.var_arg_dn6 = assign41340_e54871_d_n6;
        locals.var_arg_dn7 = assign41340_e54871_d_n7;
        locals.var_arg_dn8 = assign41340_e54871_d_n8;
        locals.var_arg_dn9 = assign41340_e54871_d_n9;
        locals.var_arg_dn10 = assign41340_e54871_d_n10;
        locals.var_arg_dn11 = assign41340_e54871_d_n11;
        locals.var_arg_dn14 = assign41340_e54871_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign41350_e54885, assign41350_e54885_d_n0, assign41350_e54885_d_n2, assign41350_e54885_d_n4, assign41350_e54885_d_n5, assign41350_e54885_d_n6, assign41350_e54885_d_n7, assign41350_e54885_d_n8, assign41350_e54885_d_n9, assign41350_e54885_d_n10, assign41350_e54885_d_n11, assign41350_e54885_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1035 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign41350_e54885;
        locals.var_dnm_dn0 = assign41350_e54885_d_n0;
        locals.var_dnm_dn2 = assign41350_e54885_d_n2;
        locals.var_dnm_dn4 = assign41350_e54885_d_n4;
        locals.var_dnm_dn5 = assign41350_e54885_d_n5;
        locals.var_dnm_dn6 = assign41350_e54885_d_n6;
        locals.var_dnm_dn7 = assign41350_e54885_d_n7;
        locals.var_dnm_dn8 = assign41350_e54885_d_n8;
        locals.var_dnm_dn9 = assign41350_e54885_d_n9;
        locals.var_dnm_dn10 = assign41350_e54885_d_n10;
        locals.var_dnm_dn11 = assign41350_e54885_d_n11;
        locals.var_dnm_dn14 = assign41350_e54885_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign41360_e54899,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1035 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign41360_e54899;
        locals.var_m0_rv = 0.0;

        let mut assign41370_loop_guard: usize = 0;
        while {
            let assign41370_cond_e54914: f64 = if (((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1035 != 0.0)) && (locals.var_m0 < locals.var_vgpdep_pw)) { 1.0 } else { 0.0 };
            assign41370_cond_e54914 != 0.0
        } {
            assign41370_loop_guard += 1;
            assert!(assign41370_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign41370_body0_e54930, assign41370_body0_e54930_d_n0, assign41370_body0_e54930_d_n2, assign41370_body0_e54930_d_n4, assign41370_body0_e54930_d_n5, assign41370_body0_e54930_d_n6, assign41370_body0_e54930_d_n7, assign41370_body0_e54930_d_n8, assign41370_body0_e54930_d_n9, assign41370_body0_e54930_d_n10, assign41370_body0_e54930_d_n11, assign41370_body0_e54930_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1035 != 0.0)) {
        let assign41370_body0_e54928: f64 = (locals.var_xp * locals.var_x2);
        (assign41370_body0_e54928, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
            locals.var_xp = assign41370_body0_e54930;
            locals.var_xp_dn0 = assign41370_body0_e54930_d_n0;
            locals.var_xp_dn2 = assign41370_body0_e54930_d_n2;
            locals.var_xp_dn4 = assign41370_body0_e54930_d_n4;
            locals.var_xp_dn5 = assign41370_body0_e54930_d_n5;
            locals.var_xp_dn6 = assign41370_body0_e54930_d_n6;
            locals.var_xp_dn7 = assign41370_body0_e54930_d_n7;
            locals.var_xp_dn8 = assign41370_body0_e54930_d_n8;
            locals.var_xp_dn9 = assign41370_body0_e54930_d_n9;
            locals.var_xp_dn10 = assign41370_body0_e54930_d_n10;
            locals.var_xp_dn11 = assign41370_body0_e54930_d_n11;
            locals.var_xp_dn14 = assign41370_body0_e54930_d_n14;
            locals.var_xp_rv = 0.0;
            let (assign41370_body1_e54946, assign41370_body1_e54946_d_n0, assign41370_body1_e54946_d_n2, assign41370_body1_e54946_d_n4, assign41370_body1_e54946_d_n5, assign41370_body1_e54946_d_n6, assign41370_body1_e54946_d_n7, assign41370_body1_e54946_d_n8, assign41370_body1_e54946_d_n9, assign41370_body1_e54946_d_n10, assign41370_body1_e54946_d_n11, assign41370_body1_e54946_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1035 != 0.0)) {
        let assign41370_body1_e54944: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign41370_body1_e54944, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
            locals.var_xmp = assign41370_body1_e54946;
            locals.var_xmp_dn0 = assign41370_body1_e54946_d_n0;
            locals.var_xmp_dn2 = assign41370_body1_e54946_d_n2;
            locals.var_xmp_dn4 = assign41370_body1_e54946_d_n4;
            locals.var_xmp_dn5 = assign41370_body1_e54946_d_n5;
            locals.var_xmp_dn6 = assign41370_body1_e54946_d_n6;
            locals.var_xmp_dn7 = assign41370_body1_e54946_d_n7;
            locals.var_xmp_dn8 = assign41370_body1_e54946_d_n8;
            locals.var_xmp_dn9 = assign41370_body1_e54946_d_n9;
            locals.var_xmp_dn10 = assign41370_body1_e54946_d_n10;
            locals.var_xmp_dn11 = assign41370_body1_e54946_d_n11;
            locals.var_xmp_dn14 = assign41370_body1_e54946_d_n14;
            locals.var_xmp_rv = 0.0;
            let (assign41370_body2_e54962,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1035 != 0.0)) {
        let assign41370_body2_e54960: f64 = (locals.var_m0 + 1.0);
        (assign41370_body2_e54960,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign41370_body2_e54962;
            locals.var_m0_rv = 0.0;
        }

        let (assign41380_e54978, assign41380_e54978_d_n0, assign41380_e54978_d_n2, assign41380_e54978_d_n4, assign41380_e54978_d_n5, assign41380_e54978_d_n6, assign41380_e54978_d_n7, assign41380_e54978_d_n8, assign41380_e54978_d_n9, assign41380_e54978_d_n10, assign41380_e54978_d_n11, assign41380_e54978_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1035 != 0.0)) {
        let assign41380_e54976: f64 = (locals.var_xp + locals.var_xmp);
        (assign41380_e54976, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign41380_e54978;
        locals.var_arg_dn0 = assign41380_e54978_d_n0;
        locals.var_arg_dn2 = assign41380_e54978_d_n2;
        locals.var_arg_dn4 = assign41380_e54978_d_n4;
        locals.var_arg_dn5 = assign41380_e54978_d_n5;
        locals.var_arg_dn6 = assign41380_e54978_d_n6;
        locals.var_arg_dn7 = assign41380_e54978_d_n7;
        locals.var_arg_dn8 = assign41380_e54978_d_n8;
        locals.var_arg_dn9 = assign41380_e54978_d_n9;
        locals.var_arg_dn10 = assign41380_e54978_d_n10;
        locals.var_arg_dn11 = assign41380_e54978_d_n11;
        locals.var_arg_dn14 = assign41380_e54978_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign41390_e54992, assign41390_e54992_d_n0, assign41390_e54992_d_n2, assign41390_e54992_d_n4, assign41390_e54992_d_n5, assign41390_e54992_d_n6, assign41390_e54992_d_n7, assign41390_e54992_d_n8, assign41390_e54992_d_n9, assign41390_e54992_d_n10, assign41390_e54992_d_n11, assign41390_e54992_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1035 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign41390_e54992;
        locals.var_dnm_dn0 = assign41390_e54992_d_n0;
        locals.var_dnm_dn2 = assign41390_e54992_d_n2;
        locals.var_dnm_dn4 = assign41390_e54992_d_n4;
        locals.var_dnm_dn5 = assign41390_e54992_d_n5;
        locals.var_dnm_dn6 = assign41390_e54992_d_n6;
        locals.var_dnm_dn7 = assign41390_e54992_d_n7;
        locals.var_dnm_dn8 = assign41390_e54992_d_n8;
        locals.var_dnm_dn9 = assign41390_e54992_d_n9;
        locals.var_dnm_dn10 = assign41390_e54992_d_n10;
        locals.var_dnm_dn11 = assign41390_e54992_d_n11;
        locals.var_dnm_dn14 = assign41390_e54992_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign41400_e55007: f64 = if ((((locals.var_vgpdep_pw == 1.0) || (locals.var_vgpdep_pw == 2.0)) || (locals.var_vgpdep_pw == 4.0)) || (locals.var_vgpdep_pw == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1036 = assign41400_e55007;
        locals.var_guard1036_rv = 0.0;

        let assign41410_e55010: f64 = if locals.var_vgpdep_pw == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1037 = assign41410_e55010;
        locals.var_guard1037_rv = 0.0;

        let (assign41420_e55028,) = {
    if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1035 != 0.0)) && (locals.var_guard1036 != 0.0)) && (locals.var_guard1037 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign41420_e55028;
        locals.var_mm_rv = 0.0;

        let assign41430_e55031: f64 = if locals.var_vgpdep_pw == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1038 = assign41430_e55031;
        locals.var_guard1038_rv = 0.0;

        let (assign41440_e55052,) = {
    if (((((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1035 != 0.0)) && (locals.var_guard1036 != 0.0)) && (locals.var_guard1037 == 0.0)) && (locals.var_guard1038 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign41440_e55052;
        locals.var_mm_rv = 0.0;

        let assign41450_e55055: f64 = if locals.var_vgpdep_pw == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1039 = assign41450_e55055;
        locals.var_guard1039_rv = 0.0;

        let (assign41460_e55079,) = {
    if ((((((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1035 != 0.0)) && (locals.var_guard1036 != 0.0)) && (locals.var_guard1037 == 0.0)) && (locals.var_guard1038 == 0.0)) && (locals.var_guard1039 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign41460_e55079;
        locals.var_mm_rv = 0.0;

        let assign41470_e55082: f64 = if locals.var_vgpdep_pw == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1040 = assign41470_e55082;
        locals.var_guard1040_rv = 0.0;

        let (assign41480_e55109,) = {
    if (((((((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1035 != 0.0)) && (locals.var_guard1036 != 0.0)) && (locals.var_guard1037 == 0.0)) && (locals.var_guard1038 == 0.0)) && (locals.var_guard1039 == 0.0)) && (locals.var_guard1040 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign41480_e55109;
        locals.var_mm_rv = 0.0;

        let (assign41490_e55125,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1035 != 0.0)) && (locals.var_guard1036 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign41490_e55125;
        locals.var_m0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_142(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign41500_loop_guard: usize = 0;
        while {
            let assign41500_cond_e55142: f64 = if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1035 != 0.0)) && (locals.var_guard1036 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign41500_cond_e55142 != 0.0
        } {
            assign41500_loop_guard += 1;
            assert!(assign41500_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign41500_body0_e55159, assign41500_body0_e55159_d_n0, assign41500_body0_e55159_d_n2, assign41500_body0_e55159_d_n4, assign41500_body0_e55159_d_n5, assign41500_body0_e55159_d_n6, assign41500_body0_e55159_d_n7, assign41500_body0_e55159_d_n8, assign41500_body0_e55159_d_n9, assign41500_body0_e55159_d_n10, assign41500_body0_e55159_d_n11, assign41500_body0_e55159_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1035 != 0.0)) && (locals.var_guard1036 != 0.0)) {
        let assign41500_body0_e55157: f64 = (locals.var_dnm).sqrt();
        (assign41500_body0_e55157, (locals.var_dnm_dn0 / (2.0 * assign41500_body0_e55157)), (locals.var_dnm_dn2 / (2.0 * assign41500_body0_e55157)), (locals.var_dnm_dn4 / (2.0 * assign41500_body0_e55157)), (locals.var_dnm_dn5 / (2.0 * assign41500_body0_e55157)), (locals.var_dnm_dn6 / (2.0 * assign41500_body0_e55157)), (locals.var_dnm_dn7 / (2.0 * assign41500_body0_e55157)), (locals.var_dnm_dn8 / (2.0 * assign41500_body0_e55157)), (locals.var_dnm_dn9 / (2.0 * assign41500_body0_e55157)), (locals.var_dnm_dn10 / (2.0 * assign41500_body0_e55157)), (locals.var_dnm_dn11 / (2.0 * assign41500_body0_e55157)), (locals.var_dnm_dn14 / (2.0 * assign41500_body0_e55157)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign41500_body0_e55159;
            locals.var_dnm_dn0 = assign41500_body0_e55159_d_n0;
            locals.var_dnm_dn2 = assign41500_body0_e55159_d_n2;
            locals.var_dnm_dn4 = assign41500_body0_e55159_d_n4;
            locals.var_dnm_dn5 = assign41500_body0_e55159_d_n5;
            locals.var_dnm_dn6 = assign41500_body0_e55159_d_n6;
            locals.var_dnm_dn7 = assign41500_body0_e55159_d_n7;
            locals.var_dnm_dn8 = assign41500_body0_e55159_d_n8;
            locals.var_dnm_dn9 = assign41500_body0_e55159_d_n9;
            locals.var_dnm_dn10 = assign41500_body0_e55159_d_n10;
            locals.var_dnm_dn11 = assign41500_body0_e55159_d_n11;
            locals.var_dnm_dn14 = assign41500_body0_e55159_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign41500_body1_e55177,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1035 != 0.0)) && (locals.var_guard1036 != 0.0)) {
        let assign41500_body1_e55175: f64 = (locals.var_m0 + 1.0);
        (assign41500_body1_e55175,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign41500_body1_e55177;
            locals.var_m0_rv = 0.0;
        }

        let (assign41510_e55205, assign41510_e55205_d_n0, assign41510_e55205_d_n2, assign41510_e55205_d_n4, assign41510_e55205_d_n5, assign41510_e55205_d_n6, assign41510_e55205_d_n7, assign41510_e55205_d_n8, assign41510_e55205_d_n9, assign41510_e55205_d_n10, assign41510_e55205_d_n11, assign41510_e55205_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1035 != 0.0)) && (locals.var_guard1036 == 0.0)) {
        let (assign41510_e55203, assign41510_e55203_d_n0, assign41510_e55203_d_n2, assign41510_e55203_d_n4, assign41510_e55203_d_n5, assign41510_e55203_d_n6, assign41510_e55203_d_n7, assign41510_e55203_d_n8, assign41510_e55203_d_n9, assign41510_e55203_d_n10, assign41510_e55203_d_n11, assign41510_e55203_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign41510_e55200: f64 = (2.0 * locals.var_vgpdep_pw);
                let assign41510_e55201: f64 = (1.0 / assign41510_e55200);
                let assign41510_e55202: f64 = (locals.var_dnm).powf(assign41510_e55201);
                (assign41510_e55202, if 0.0 == 0.0 && ((assign41510_e55201) as f64).is_finite() && ((assign41510_e55201) as f64).fract() == 0.0 { if assign41510_e55201 == 0.0 { 0.0 } else { (assign41510_e55201 * ((locals.var_dnm).powf(assign41510_e55201 - 1.0) * locals.var_dnm_dn0)) } } else { (assign41510_e55202 * (assign41510_e55201 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41510_e55201) as f64).is_finite() && ((assign41510_e55201) as f64).fract() == 0.0 { if assign41510_e55201 == 0.0 { 0.0 } else { (assign41510_e55201 * ((locals.var_dnm).powf(assign41510_e55201 - 1.0) * locals.var_dnm_dn2)) } } else { (assign41510_e55202 * (assign41510_e55201 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41510_e55201) as f64).is_finite() && ((assign41510_e55201) as f64).fract() == 0.0 { if assign41510_e55201 == 0.0 { 0.0 } else { (assign41510_e55201 * ((locals.var_dnm).powf(assign41510_e55201 - 1.0) * locals.var_dnm_dn4)) } } else { (assign41510_e55202 * (assign41510_e55201 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41510_e55201) as f64).is_finite() && ((assign41510_e55201) as f64).fract() == 0.0 { if assign41510_e55201 == 0.0 { 0.0 } else { (assign41510_e55201 * ((locals.var_dnm).powf(assign41510_e55201 - 1.0) * locals.var_dnm_dn5)) } } else { (assign41510_e55202 * (assign41510_e55201 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41510_e55201) as f64).is_finite() && ((assign41510_e55201) as f64).fract() == 0.0 { if assign41510_e55201 == 0.0 { 0.0 } else { (assign41510_e55201 * ((locals.var_dnm).powf(assign41510_e55201 - 1.0) * locals.var_dnm_dn6)) } } else { (assign41510_e55202 * (assign41510_e55201 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41510_e55201) as f64).is_finite() && ((assign41510_e55201) as f64).fract() == 0.0 { if assign41510_e55201 == 0.0 { 0.0 } else { (assign41510_e55201 * ((locals.var_dnm).powf(assign41510_e55201 - 1.0) * locals.var_dnm_dn7)) } } else { (assign41510_e55202 * (assign41510_e55201 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41510_e55201) as f64).is_finite() && ((assign41510_e55201) as f64).fract() == 0.0 { if assign41510_e55201 == 0.0 { 0.0 } else { (assign41510_e55201 * ((locals.var_dnm).powf(assign41510_e55201 - 1.0) * locals.var_dnm_dn8)) } } else { (assign41510_e55202 * (assign41510_e55201 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41510_e55201) as f64).is_finite() && ((assign41510_e55201) as f64).fract() == 0.0 { if assign41510_e55201 == 0.0 { 0.0 } else { (assign41510_e55201 * ((locals.var_dnm).powf(assign41510_e55201 - 1.0) * locals.var_dnm_dn9)) } } else { (assign41510_e55202 * (assign41510_e55201 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41510_e55201) as f64).is_finite() && ((assign41510_e55201) as f64).fract() == 0.0 { if assign41510_e55201 == 0.0 { 0.0 } else { (assign41510_e55201 * ((locals.var_dnm).powf(assign41510_e55201 - 1.0) * locals.var_dnm_dn10)) } } else { (assign41510_e55202 * (assign41510_e55201 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41510_e55201) as f64).is_finite() && ((assign41510_e55201) as f64).fract() == 0.0 { if assign41510_e55201 == 0.0 { 0.0 } else { (assign41510_e55201 * ((locals.var_dnm).powf(assign41510_e55201 - 1.0) * locals.var_dnm_dn11)) } } else { (assign41510_e55202 * (assign41510_e55201 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41510_e55201) as f64).is_finite() && ((assign41510_e55201) as f64).fract() == 0.0 { if assign41510_e55201 == 0.0 { 0.0 } else { (assign41510_e55201 * ((locals.var_dnm).powf(assign41510_e55201 - 1.0) * locals.var_dnm_dn14)) } } else { (assign41510_e55202 * (assign41510_e55201 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign41510_e55203, assign41510_e55203_d_n0, assign41510_e55203_d_n2, assign41510_e55203_d_n4, assign41510_e55203_d_n5, assign41510_e55203_d_n6, assign41510_e55203_d_n7, assign41510_e55203_d_n8, assign41510_e55203_d_n9, assign41510_e55203_d_n10, assign41510_e55203_d_n11, assign41510_e55203_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign41510_e55205;
        locals.var_dnm_dn0 = assign41510_e55205_d_n0;
        locals.var_dnm_dn2 = assign41510_e55205_d_n2;
        locals.var_dnm_dn4 = assign41510_e55205_d_n4;
        locals.var_dnm_dn5 = assign41510_e55205_d_n5;
        locals.var_dnm_dn6 = assign41510_e55205_d_n6;
        locals.var_dnm_dn7 = assign41510_e55205_d_n7;
        locals.var_dnm_dn8 = assign41510_e55205_d_n8;
        locals.var_dnm_dn9 = assign41510_e55205_d_n9;
        locals.var_dnm_dn10 = assign41510_e55205_d_n10;
        locals.var_dnm_dn11 = assign41510_e55205_d_n11;
        locals.var_dnm_dn14 = assign41510_e55205_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign41520_e55221, assign41520_e55221_d_n0, assign41520_e55221_d_n2, assign41520_e55221_d_n4, assign41520_e55221_d_n5, assign41520_e55221_d_n6, assign41520_e55221_d_n7, assign41520_e55221_d_n8, assign41520_e55221_d_n9, assign41520_e55221_d_n10, assign41520_e55221_d_n11, assign41520_e55221_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1035 != 0.0)) {
        let assign41520_e55219: f64 = (1.0 / locals.var_dnm);
        (assign41520_e55219, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign41520_e55221;
        locals.var_dnm_dn0 = assign41520_e55221_d_n0;
        locals.var_dnm_dn2 = assign41520_e55221_d_n2;
        locals.var_dnm_dn4 = assign41520_e55221_d_n4;
        locals.var_dnm_dn5 = assign41520_e55221_d_n5;
        locals.var_dnm_dn6 = assign41520_e55221_d_n6;
        locals.var_dnm_dn7 = assign41520_e55221_d_n7;
        locals.var_dnm_dn8 = assign41520_e55221_d_n8;
        locals.var_dnm_dn9 = assign41520_e55221_d_n9;
        locals.var_dnm_dn10 = assign41520_e55221_d_n10;
        locals.var_dnm_dn11 = assign41520_e55221_d_n11;
        locals.var_dnm_dn14 = assign41520_e55221_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign41530_e55239, assign41530_e55239_d_n0, assign41530_e55239_d_n2, assign41530_e55239_d_n4, assign41530_e55239_d_n5, assign41530_e55239_d_n6, assign41530_e55239_d_n7, assign41530_e55239_d_n8, assign41530_e55239_d_n9, assign41530_e55239_d_n10, assign41530_e55239_d_n11, assign41530_e55239_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1035 != 0.0)) {
        let assign41530_e55235: f64 = (locals.var_tmf1 * 0.1);
        let assign41530_e55237: f64 = (assign41530_e55235 * locals.var_dnm);
        (assign41530_e55237, (((locals.var_tmf1_dn0 * 0.1) * locals.var_dnm) + (assign41530_e55235 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * 0.1) * locals.var_dnm) + (assign41530_e55235 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * 0.1) * locals.var_dnm) + (assign41530_e55235 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * 0.1) * locals.var_dnm) + (assign41530_e55235 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * 0.1) * locals.var_dnm) + (assign41530_e55235 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * 0.1) * locals.var_dnm) + (assign41530_e55235 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * 0.1) * locals.var_dnm) + (assign41530_e55235 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * 0.1) * locals.var_dnm) + (assign41530_e55235 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * 0.1) * locals.var_dnm) + (assign41530_e55235 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * 0.1) * locals.var_dnm) + (assign41530_e55235 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * 0.1) * locals.var_dnm) + (assign41530_e55235 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign41530_e55239;
        locals.var_tmf0_dn0 = assign41530_e55239_d_n0;
        locals.var_tmf0_dn2 = assign41530_e55239_d_n2;
        locals.var_tmf0_dn4 = assign41530_e55239_d_n4;
        locals.var_tmf0_dn5 = assign41530_e55239_d_n5;
        locals.var_tmf0_dn6 = assign41530_e55239_d_n6;
        locals.var_tmf0_dn7 = assign41530_e55239_d_n7;
        locals.var_tmf0_dn8 = assign41530_e55239_d_n8;
        locals.var_tmf0_dn9 = assign41530_e55239_d_n9;
        locals.var_tmf0_dn10 = assign41530_e55239_d_n10;
        locals.var_tmf0_dn11 = assign41530_e55239_d_n11;
        locals.var_tmf0_dn14 = assign41530_e55239_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign41540_e55259, assign41540_e55259_d_n0, assign41540_e55259_d_n2, assign41540_e55259_d_n4, assign41540_e55259_d_n5, assign41540_e55259_d_n6, assign41540_e55259_d_n7, assign41540_e55259_d_n8, assign41540_e55259_d_n9, assign41540_e55259_d_n10, assign41540_e55259_d_n11, assign41540_e55259_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1035 != 0.0)) {
        let assign41540_e55253: f64 = (0.1 * locals.var_xmp);
        let assign41540_e55255: f64 = (assign41540_e55253 * locals.var_dnm);
        let assign41540_e55257: f64 = (assign41540_e55255 / locals.var_arg);
        (assign41540_e55257, ((((((0.1 * locals.var_xmp_dn0) * locals.var_dnm) + (assign41540_e55253 * locals.var_dnm_dn0)) * locals.var_arg) - (assign41540_e55255 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn2) * locals.var_dnm) + (assign41540_e55253 * locals.var_dnm_dn2)) * locals.var_arg) - (assign41540_e55255 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn4) * locals.var_dnm) + (assign41540_e55253 * locals.var_dnm_dn4)) * locals.var_arg) - (assign41540_e55255 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn5) * locals.var_dnm) + (assign41540_e55253 * locals.var_dnm_dn5)) * locals.var_arg) - (assign41540_e55255 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn6) * locals.var_dnm) + (assign41540_e55253 * locals.var_dnm_dn6)) * locals.var_arg) - (assign41540_e55255 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn7) * locals.var_dnm) + (assign41540_e55253 * locals.var_dnm_dn7)) * locals.var_arg) - (assign41540_e55255 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn8) * locals.var_dnm) + (assign41540_e55253 * locals.var_dnm_dn8)) * locals.var_arg) - (assign41540_e55255 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn9) * locals.var_dnm) + (assign41540_e55253 * locals.var_dnm_dn9)) * locals.var_arg) - (assign41540_e55255 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn10) * locals.var_dnm) + (assign41540_e55253 * locals.var_dnm_dn10)) * locals.var_arg) - (assign41540_e55255 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn11) * locals.var_dnm) + (assign41540_e55253 * locals.var_dnm_dn11)) * locals.var_arg) - (assign41540_e55255 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((0.1 * locals.var_xmp_dn14) * locals.var_dnm) + (assign41540_e55253 * locals.var_dnm_dn14)) * locals.var_arg) - (assign41540_e55255 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign41540_e55259;
        locals.var_t0_dn0 = assign41540_e55259_d_n0;
        locals.var_t0_dn2 = assign41540_e55259_d_n2;
        locals.var_t0_dn4 = assign41540_e55259_d_n4;
        locals.var_t0_dn5 = assign41540_e55259_d_n5;
        locals.var_t0_dn6 = assign41540_e55259_d_n6;
        locals.var_t0_dn7 = assign41540_e55259_d_n7;
        locals.var_t0_dn8 = assign41540_e55259_d_n8;
        locals.var_t0_dn9 = assign41540_e55259_d_n9;
        locals.var_t0_dn10 = assign41540_e55259_d_n10;
        locals.var_t0_dn11 = assign41540_e55259_d_n11;
        locals.var_t0_dn14 = assign41540_e55259_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign41550_e55277, assign41550_e55277_d_n0, assign41550_e55277_d_n2, assign41550_e55277_d_n4, assign41550_e55277_d_n5, assign41550_e55277_d_n6, assign41550_e55277_d_n7, assign41550_e55277_d_n8, assign41550_e55277_d_n9, assign41550_e55277_d_n10, assign41550_e55277_d_n11, assign41550_e55277_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1035 != 0.0)) {
        let assign41550_e55273: f64 = (-0.1);
        let assign41550_e55275: f64 = (assign41550_e55273 + locals.var_tmf0);
        (assign41550_e55275, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    }
};
        locals.var_ps0dep = assign41550_e55277;
        locals.var_ps0dep_dn0 = assign41550_e55277_d_n0;
        locals.var_ps0dep_dn2 = assign41550_e55277_d_n2;
        locals.var_ps0dep_dn4 = assign41550_e55277_d_n4;
        locals.var_ps0dep_dn5 = assign41550_e55277_d_n5;
        locals.var_ps0dep_dn6 = assign41550_e55277_d_n6;
        locals.var_ps0dep_dn7 = assign41550_e55277_d_n7;
        locals.var_ps0dep_dn8 = assign41550_e55277_d_n8;
        locals.var_ps0dep_dn9 = assign41550_e55277_d_n9;
        locals.var_ps0dep_dn10 = assign41550_e55277_d_n10;
        locals.var_ps0dep_dn11 = assign41550_e55277_d_n11;
        locals.var_ps0dep_dn14 = assign41550_e55277_d_n14;
        locals.var_ps0dep_rv = 0.0;

        let (assign41560_e55291, assign41560_e55291_d_n0, assign41560_e55291_d_n2, assign41560_e55291_d_n4, assign41560_e55291_d_n5, assign41560_e55291_d_n6, assign41560_e55291_d_n7, assign41560_e55291_d_n8, assign41560_e55291_d_n9, assign41560_e55291_d_n10, assign41560_e55291_d_n11, assign41560_e55291_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1035 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign41560_e55291;
        locals.var_t0_dn0 = assign41560_e55291_d_n0;
        locals.var_t0_dn2 = assign41560_e55291_d_n2;
        locals.var_t0_dn4 = assign41560_e55291_d_n4;
        locals.var_t0_dn5 = assign41560_e55291_d_n5;
        locals.var_t0_dn6 = assign41560_e55291_d_n6;
        locals.var_t0_dn7 = assign41560_e55291_d_n7;
        locals.var_t0_dn8 = assign41560_e55291_d_n8;
        locals.var_t0_dn9 = assign41560_e55291_d_n9;
        locals.var_t0_dn10 = assign41560_e55291_d_n10;
        locals.var_t0_dn11 = assign41560_e55291_d_n11;
        locals.var_t0_dn14 = assign41560_e55291_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign41570_e55306, assign41570_e55306_d_n0, assign41570_e55306_d_n2, assign41570_e55306_d_n4, assign41570_e55306_d_n5, assign41570_e55306_d_n6, assign41570_e55306_d_n7, assign41570_e55306_d_n8, assign41570_e55306_d_n9, assign41570_e55306_d_n10, assign41570_e55306_d_n11, assign41570_e55306_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1035 == 0.0)) {
        (locals.var_ps0, locals.var_ps0_dn0, locals.var_ps0_dn2, locals.var_ps0_dn4, locals.var_ps0_dn5, locals.var_ps0_dn6, locals.var_ps0_dn7, locals.var_ps0_dn8, locals.var_ps0_dn9, locals.var_ps0_dn10, locals.var_ps0_dn11, locals.var_ps0_dn14,)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    }
};
        locals.var_ps0dep = assign41570_e55306;
        locals.var_ps0dep_dn0 = assign41570_e55306_d_n0;
        locals.var_ps0dep_dn2 = assign41570_e55306_d_n2;
        locals.var_ps0dep_dn4 = assign41570_e55306_d_n4;
        locals.var_ps0dep_dn5 = assign41570_e55306_d_n5;
        locals.var_ps0dep_dn6 = assign41570_e55306_d_n6;
        locals.var_ps0dep_dn7 = assign41570_e55306_d_n7;
        locals.var_ps0dep_dn8 = assign41570_e55306_d_n8;
        locals.var_ps0dep_dn9 = assign41570_e55306_d_n9;
        locals.var_ps0dep_dn10 = assign41570_e55306_d_n10;
        locals.var_ps0dep_dn11 = assign41570_e55306_d_n11;
        locals.var_ps0dep_dn14 = assign41570_e55306_d_n14;
        locals.var_ps0dep_rv = 0.0;

        let (assign41580_e55321, assign41580_e55321_d_n0, assign41580_e55321_d_n2, assign41580_e55321_d_n4, assign41580_e55321_d_n5, assign41580_e55321_d_n6, assign41580_e55321_d_n7, assign41580_e55321_d_n8, assign41580_e55321_d_n9, assign41580_e55321_d_n10, assign41580_e55321_d_n11, assign41580_e55321_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1035 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign41580_e55321;
        locals.var_t0_dn0 = assign41580_e55321_d_n0;
        locals.var_t0_dn2 = assign41580_e55321_d_n2;
        locals.var_t0_dn4 = assign41580_e55321_d_n4;
        locals.var_t0_dn5 = assign41580_e55321_d_n5;
        locals.var_t0_dn6 = assign41580_e55321_d_n6;
        locals.var_t0_dn7 = assign41580_e55321_d_n7;
        locals.var_t0_dn8 = assign41580_e55321_d_n8;
        locals.var_t0_dn9 = assign41580_e55321_d_n9;
        locals.var_t0_dn10 = assign41580_e55321_d_n10;
        locals.var_t0_dn11 = assign41580_e55321_d_n11;
        locals.var_t0_dn14 = assign41580_e55321_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign41590_e55341, assign41590_e55341_d_n0, assign41590_e55341_d_n2, assign41590_e55341_d_n4, assign41590_e55341_d_n5, assign41590_e55341_d_n6, assign41590_e55341_d_n7, assign41590_e55341_d_n8, assign41590_e55341_d_n9, assign41590_e55341_d_n10, assign41590_e55341_d_n11, assign41590_e55341_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) {
        let assign41590_e55333: f64 = (locals.var_vgs - locals.var_vgp);
        let assign41590_e55336: f64 = (locals.var_uc_vfbc - p.p392);
        let assign41590_e55338: f64 = (assign41590_e55336 - locals.var_vfboffset);
        let assign41590_e55339: f64 = (assign41590_e55333 - assign41590_e55338);
        (assign41590_e55339, (-locals.var_vgp_dn0), (-locals.var_vgp_dn2), (-locals.var_vgp_dn4), (-locals.var_vgp_dn5), (locals.var_vgs_dn6 - locals.var_vgp_dn6), (locals.var_vgs_dn7 - locals.var_vgp_dn7), (locals.var_vgs_dn8 - locals.var_vgp_dn8), (-locals.var_vgp_dn9), (-locals.var_vgp_dn10), (-locals.var_vgp_dn11), (-locals.var_vgp_dn14),)
    } else {
        (locals.var_vfb_res, locals.var_vfb_res_dn0, locals.var_vfb_res_dn2, locals.var_vfb_res_dn4, locals.var_vfb_res_dn5, locals.var_vfb_res_dn6, locals.var_vfb_res_dn7, locals.var_vfb_res_dn8, locals.var_vfb_res_dn9, locals.var_vfb_res_dn10, locals.var_vfb_res_dn11, locals.var_vfb_res_dn14,)
    }
};
        locals.var_vfb_res = assign41590_e55341;
        locals.var_vfb_res_dn0 = assign41590_e55341_d_n0;
        locals.var_vfb_res_dn2 = assign41590_e55341_d_n2;
        locals.var_vfb_res_dn4 = assign41590_e55341_d_n4;
        locals.var_vfb_res_dn5 = assign41590_e55341_d_n5;
        locals.var_vfb_res_dn6 = assign41590_e55341_d_n6;
        locals.var_vfb_res_dn7 = assign41590_e55341_d_n7;
        locals.var_vfb_res_dn8 = assign41590_e55341_d_n8;
        locals.var_vfb_res_dn9 = assign41590_e55341_d_n9;
        locals.var_vfb_res_dn10 = assign41590_e55341_d_n10;
        locals.var_vfb_res_dn11 = assign41590_e55341_d_n11;
        locals.var_vfb_res_dn14 = assign41590_e55341_d_n14;
        locals.var_vfb_res_rv = 0.0;

        let (assign41600_e55355, assign41600_e55355_d_n0, assign41600_e55355_d_n2, assign41600_e55355_d_n4, assign41600_e55355_d_n5, assign41600_e55355_d_n6, assign41600_e55355_d_n7, assign41600_e55355_d_n8, assign41600_e55355_d_n9, assign41600_e55355_d_n10, assign41600_e55355_d_n11, assign41600_e55355_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) {
        let assign41600_e55353: f64 = (locals.var_vgs - locals.var_vfb_res);
        (assign41600_e55353, (-locals.var_vfb_res_dn0), (-locals.var_vfb_res_dn2), (-locals.var_vfb_res_dn4), (-locals.var_vfb_res_dn5), (locals.var_vgs_dn6 - locals.var_vfb_res_dn6), (locals.var_vgs_dn7 - locals.var_vfb_res_dn7), (locals.var_vgs_dn8 - locals.var_vfb_res_dn8), (-locals.var_vfb_res_dn9), (-locals.var_vfb_res_dn10), (-locals.var_vfb_res_dn11), (-locals.var_vfb_res_dn14),)
    } else {
        (locals.var_vgp_res, locals.var_vgp_res_dn0, locals.var_vgp_res_dn2, locals.var_vgp_res_dn4, locals.var_vgp_res_dn5, locals.var_vgp_res_dn6, locals.var_vgp_res_dn7, locals.var_vgp_res_dn8, locals.var_vgp_res_dn9, locals.var_vgp_res_dn10, locals.var_vgp_res_dn11, locals.var_vgp_res_dn14,)
    }
};
        locals.var_vgp_res = assign41600_e55355;
        locals.var_vgp_res_dn0 = assign41600_e55355_d_n0;
        locals.var_vgp_res_dn2 = assign41600_e55355_d_n2;
        locals.var_vgp_res_dn4 = assign41600_e55355_d_n4;
        locals.var_vgp_res_dn5 = assign41600_e55355_d_n5;
        locals.var_vgp_res_dn6 = assign41600_e55355_d_n6;
        locals.var_vgp_res_dn7 = assign41600_e55355_d_n7;
        locals.var_vgp_res_dn8 = assign41600_e55355_d_n8;
        locals.var_vgp_res_dn9 = assign41600_e55355_d_n9;
        locals.var_vgp_res_dn10 = assign41600_e55355_d_n10;
        locals.var_vgp_res_dn11 = assign41600_e55355_d_n11;
        locals.var_vgp_res_dn14 = assign41600_e55355_d_n14;
        locals.var_vgp_res_rv = 0.0;

        let assign41610_e55359: f64 = (-locals.var_vgpdep_dlt);
        let assign41610_e55364: f64 = if ((locals.var_vgp_res > assign41610_e55359) && (locals.var_vgpdep_dlt >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1041 = assign41610_e55364;
        locals.var_guard1041_rv = 0.0;

        let (assign41620_e55382, assign41620_e55382_d_n0, assign41620_e55382_d_n2, assign41620_e55382_d_n4, assign41620_e55382_d_n5, assign41620_e55382_d_n6, assign41620_e55382_d_n7, assign41620_e55382_d_n8, assign41620_e55382_d_n9, assign41620_e55382_d_n10, assign41620_e55382_d_n11, assign41620_e55382_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1041 != 0.0)) {
        let assign41620_e55378: f64 = locals.var_vgp_res;
        let assign41620_e55380: f64 = (assign41620_e55378 + locals.var_vgpdep_dlt);
        (assign41620_e55380, (locals.var_vgp_res_dn0 + locals.var_vgpdep_dlt_dn0), (locals.var_vgp_res_dn2 + locals.var_vgpdep_dlt_dn2), (locals.var_vgp_res_dn4 + locals.var_vgpdep_dlt_dn4), (locals.var_vgp_res_dn5 + locals.var_vgpdep_dlt_dn5), (locals.var_vgp_res_dn6 + locals.var_vgpdep_dlt_dn6), (locals.var_vgp_res_dn7 + locals.var_vgpdep_dlt_dn7), (locals.var_vgp_res_dn8 + locals.var_vgpdep_dlt_dn8), (locals.var_vgp_res_dn9 + locals.var_vgpdep_dlt_dn9), (locals.var_vgp_res_dn10 + locals.var_vgpdep_dlt_dn10), (locals.var_vgp_res_dn11 + locals.var_vgpdep_dlt_dn11), (locals.var_vgp_res_dn14 + locals.var_vgpdep_dlt_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign41620_e55382;
        locals.var_tmf1_dn0 = assign41620_e55382_d_n0;
        locals.var_tmf1_dn2 = assign41620_e55382_d_n2;
        locals.var_tmf1_dn4 = assign41620_e55382_d_n4;
        locals.var_tmf1_dn5 = assign41620_e55382_d_n5;
        locals.var_tmf1_dn6 = assign41620_e55382_d_n6;
        locals.var_tmf1_dn7 = assign41620_e55382_d_n7;
        locals.var_tmf1_dn8 = assign41620_e55382_d_n8;
        locals.var_tmf1_dn9 = assign41620_e55382_d_n9;
        locals.var_tmf1_dn10 = assign41620_e55382_d_n10;
        locals.var_tmf1_dn11 = assign41620_e55382_d_n11;
        locals.var_tmf1_dn14 = assign41620_e55382_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign41630_e55398, assign41630_e55398_d_n0, assign41630_e55398_d_n2, assign41630_e55398_d_n4, assign41630_e55398_d_n5, assign41630_e55398_d_n6, assign41630_e55398_d_n7, assign41630_e55398_d_n8, assign41630_e55398_d_n9, assign41630_e55398_d_n10, assign41630_e55398_d_n11, assign41630_e55398_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1041 != 0.0)) {
        let assign41630_e55396: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign41630_e55396, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign41630_e55398;
        locals.var_x2_dn0 = assign41630_e55398_d_n0;
        locals.var_x2_dn2 = assign41630_e55398_d_n2;
        locals.var_x2_dn4 = assign41630_e55398_d_n4;
        locals.var_x2_dn5 = assign41630_e55398_d_n5;
        locals.var_x2_dn6 = assign41630_e55398_d_n6;
        locals.var_x2_dn7 = assign41630_e55398_d_n7;
        locals.var_x2_dn8 = assign41630_e55398_d_n8;
        locals.var_x2_dn9 = assign41630_e55398_d_n9;
        locals.var_x2_dn10 = assign41630_e55398_d_n10;
        locals.var_x2_dn11 = assign41630_e55398_d_n11;
        locals.var_x2_dn14 = assign41630_e55398_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign41640_e55414, assign41640_e55414_d_n0, assign41640_e55414_d_n2, assign41640_e55414_d_n4, assign41640_e55414_d_n5, assign41640_e55414_d_n6, assign41640_e55414_d_n7, assign41640_e55414_d_n8, assign41640_e55414_d_n9, assign41640_e55414_d_n10, assign41640_e55414_d_n11, assign41640_e55414_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1041 != 0.0)) {
        let assign41640_e55412: f64 = (locals.var_vgpdep_dlt * locals.var_vgpdep_dlt);
        (assign41640_e55412, ((locals.var_vgpdep_dlt_dn0 * locals.var_vgpdep_dlt) + (locals.var_vgpdep_dlt * locals.var_vgpdep_dlt_dn0)), ((locals.var_vgpdep_dlt_dn2 * locals.var_vgpdep_dlt) + (locals.var_vgpdep_dlt * locals.var_vgpdep_dlt_dn2)), ((locals.var_vgpdep_dlt_dn4 * locals.var_vgpdep_dlt) + (locals.var_vgpdep_dlt * locals.var_vgpdep_dlt_dn4)), ((locals.var_vgpdep_dlt_dn5 * locals.var_vgpdep_dlt) + (locals.var_vgpdep_dlt * locals.var_vgpdep_dlt_dn5)), ((locals.var_vgpdep_dlt_dn6 * locals.var_vgpdep_dlt) + (locals.var_vgpdep_dlt * locals.var_vgpdep_dlt_dn6)), ((locals.var_vgpdep_dlt_dn7 * locals.var_vgpdep_dlt) + (locals.var_vgpdep_dlt * locals.var_vgpdep_dlt_dn7)), ((locals.var_vgpdep_dlt_dn8 * locals.var_vgpdep_dlt) + (locals.var_vgpdep_dlt * locals.var_vgpdep_dlt_dn8)), ((locals.var_vgpdep_dlt_dn9 * locals.var_vgpdep_dlt) + (locals.var_vgpdep_dlt * locals.var_vgpdep_dlt_dn9)), ((locals.var_vgpdep_dlt_dn10 * locals.var_vgpdep_dlt) + (locals.var_vgpdep_dlt * locals.var_vgpdep_dlt_dn10)), ((locals.var_vgpdep_dlt_dn11 * locals.var_vgpdep_dlt) + (locals.var_vgpdep_dlt * locals.var_vgpdep_dlt_dn11)), ((locals.var_vgpdep_dlt_dn14 * locals.var_vgpdep_dlt) + (locals.var_vgpdep_dlt * locals.var_vgpdep_dlt_dn14)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign41640_e55414;
        locals.var_xmax2_dn0 = assign41640_e55414_d_n0;
        locals.var_xmax2_dn2 = assign41640_e55414_d_n2;
        locals.var_xmax2_dn4 = assign41640_e55414_d_n4;
        locals.var_xmax2_dn5 = assign41640_e55414_d_n5;
        locals.var_xmax2_dn6 = assign41640_e55414_d_n6;
        locals.var_xmax2_dn7 = assign41640_e55414_d_n7;
        locals.var_xmax2_dn8 = assign41640_e55414_d_n8;
        locals.var_xmax2_dn9 = assign41640_e55414_d_n9;
        locals.var_xmax2_dn10 = assign41640_e55414_d_n10;
        locals.var_xmax2_dn11 = assign41640_e55414_d_n11;
        locals.var_xmax2_dn14 = assign41640_e55414_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign41650_e55428, assign41650_e55428_d_n0, assign41650_e55428_d_n2, assign41650_e55428_d_n4, assign41650_e55428_d_n5, assign41650_e55428_d_n6, assign41650_e55428_d_n7, assign41650_e55428_d_n8, assign41650_e55428_d_n9, assign41650_e55428_d_n10, assign41650_e55428_d_n11, assign41650_e55428_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1041 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign41650_e55428;
        locals.var_xp_dn0 = assign41650_e55428_d_n0;
        locals.var_xp_dn2 = assign41650_e55428_d_n2;
        locals.var_xp_dn4 = assign41650_e55428_d_n4;
        locals.var_xp_dn5 = assign41650_e55428_d_n5;
        locals.var_xp_dn6 = assign41650_e55428_d_n6;
        locals.var_xp_dn7 = assign41650_e55428_d_n7;
        locals.var_xp_dn8 = assign41650_e55428_d_n8;
        locals.var_xp_dn9 = assign41650_e55428_d_n9;
        locals.var_xp_dn10 = assign41650_e55428_d_n10;
        locals.var_xp_dn11 = assign41650_e55428_d_n11;
        locals.var_xp_dn14 = assign41650_e55428_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign41660_e55442, assign41660_e55442_d_n0, assign41660_e55442_d_n2, assign41660_e55442_d_n4, assign41660_e55442_d_n5, assign41660_e55442_d_n6, assign41660_e55442_d_n7, assign41660_e55442_d_n8, assign41660_e55442_d_n9, assign41660_e55442_d_n10, assign41660_e55442_d_n11, assign41660_e55442_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1041 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign41660_e55442;
        locals.var_xmp_dn0 = assign41660_e55442_d_n0;
        locals.var_xmp_dn2 = assign41660_e55442_d_n2;
        locals.var_xmp_dn4 = assign41660_e55442_d_n4;
        locals.var_xmp_dn5 = assign41660_e55442_d_n5;
        locals.var_xmp_dn6 = assign41660_e55442_d_n6;
        locals.var_xmp_dn7 = assign41660_e55442_d_n7;
        locals.var_xmp_dn8 = assign41660_e55442_d_n8;
        locals.var_xmp_dn9 = assign41660_e55442_d_n9;
        locals.var_xmp_dn10 = assign41660_e55442_d_n10;
        locals.var_xmp_dn11 = assign41660_e55442_d_n11;
        locals.var_xmp_dn14 = assign41660_e55442_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign41670_e55456,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1041 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign41670_e55456;
        locals.var_m0_rv = 0.0;

        let (assign41680_e55470,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1041 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign41680_e55470;
        locals.var_mm_rv = 0.0;

        let (assign41690_e55484, assign41690_e55484_d_n0, assign41690_e55484_d_n2, assign41690_e55484_d_n4, assign41690_e55484_d_n5, assign41690_e55484_d_n6, assign41690_e55484_d_n7, assign41690_e55484_d_n8, assign41690_e55484_d_n9, assign41690_e55484_d_n10, assign41690_e55484_d_n11, assign41690_e55484_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1041 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign41690_e55484;
        locals.var_arg_dn0 = assign41690_e55484_d_n0;
        locals.var_arg_dn2 = assign41690_e55484_d_n2;
        locals.var_arg_dn4 = assign41690_e55484_d_n4;
        locals.var_arg_dn5 = assign41690_e55484_d_n5;
        locals.var_arg_dn6 = assign41690_e55484_d_n6;
        locals.var_arg_dn7 = assign41690_e55484_d_n7;
        locals.var_arg_dn8 = assign41690_e55484_d_n8;
        locals.var_arg_dn9 = assign41690_e55484_d_n9;
        locals.var_arg_dn10 = assign41690_e55484_d_n10;
        locals.var_arg_dn11 = assign41690_e55484_d_n11;
        locals.var_arg_dn14 = assign41690_e55484_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign41700_e55498, assign41700_e55498_d_n0, assign41700_e55498_d_n2, assign41700_e55498_d_n4, assign41700_e55498_d_n5, assign41700_e55498_d_n6, assign41700_e55498_d_n7, assign41700_e55498_d_n8, assign41700_e55498_d_n9, assign41700_e55498_d_n10, assign41700_e55498_d_n11, assign41700_e55498_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1041 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign41700_e55498;
        locals.var_dnm_dn0 = assign41700_e55498_d_n0;
        locals.var_dnm_dn2 = assign41700_e55498_d_n2;
        locals.var_dnm_dn4 = assign41700_e55498_d_n4;
        locals.var_dnm_dn5 = assign41700_e55498_d_n5;
        locals.var_dnm_dn6 = assign41700_e55498_d_n6;
        locals.var_dnm_dn7 = assign41700_e55498_d_n7;
        locals.var_dnm_dn8 = assign41700_e55498_d_n8;
        locals.var_dnm_dn9 = assign41700_e55498_d_n9;
        locals.var_dnm_dn10 = assign41700_e55498_d_n10;
        locals.var_dnm_dn11 = assign41700_e55498_d_n11;
        locals.var_dnm_dn14 = assign41700_e55498_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign41710_e55512,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1041 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign41710_e55512;
        locals.var_m0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_143(
        locals: &mut StampLocals,
    ) {
        let mut assign41720_loop_guard: usize = 0;
        while {
            let assign41720_cond_e55527: f64 = if (((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1041 != 0.0)) && (locals.var_m0 < locals.var_vgpdep_pw)) { 1.0 } else { 0.0 };
            assign41720_cond_e55527 != 0.0
        } {
            assign41720_loop_guard += 1;
            assert!(assign41720_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign41720_body0_e55543, assign41720_body0_e55543_d_n0, assign41720_body0_e55543_d_n2, assign41720_body0_e55543_d_n4, assign41720_body0_e55543_d_n5, assign41720_body0_e55543_d_n6, assign41720_body0_e55543_d_n7, assign41720_body0_e55543_d_n8, assign41720_body0_e55543_d_n9, assign41720_body0_e55543_d_n10, assign41720_body0_e55543_d_n11, assign41720_body0_e55543_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1041 != 0.0)) {
        let assign41720_body0_e55541: f64 = (locals.var_xp * locals.var_x2);
        (assign41720_body0_e55541, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
            locals.var_xp = assign41720_body0_e55543;
            locals.var_xp_dn0 = assign41720_body0_e55543_d_n0;
            locals.var_xp_dn2 = assign41720_body0_e55543_d_n2;
            locals.var_xp_dn4 = assign41720_body0_e55543_d_n4;
            locals.var_xp_dn5 = assign41720_body0_e55543_d_n5;
            locals.var_xp_dn6 = assign41720_body0_e55543_d_n6;
            locals.var_xp_dn7 = assign41720_body0_e55543_d_n7;
            locals.var_xp_dn8 = assign41720_body0_e55543_d_n8;
            locals.var_xp_dn9 = assign41720_body0_e55543_d_n9;
            locals.var_xp_dn10 = assign41720_body0_e55543_d_n10;
            locals.var_xp_dn11 = assign41720_body0_e55543_d_n11;
            locals.var_xp_dn14 = assign41720_body0_e55543_d_n14;
            locals.var_xp_rv = 0.0;
            let (assign41720_body1_e55559, assign41720_body1_e55559_d_n0, assign41720_body1_e55559_d_n2, assign41720_body1_e55559_d_n4, assign41720_body1_e55559_d_n5, assign41720_body1_e55559_d_n6, assign41720_body1_e55559_d_n7, assign41720_body1_e55559_d_n8, assign41720_body1_e55559_d_n9, assign41720_body1_e55559_d_n10, assign41720_body1_e55559_d_n11, assign41720_body1_e55559_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1041 != 0.0)) {
        let assign41720_body1_e55557: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign41720_body1_e55557, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
            locals.var_xmp = assign41720_body1_e55559;
            locals.var_xmp_dn0 = assign41720_body1_e55559_d_n0;
            locals.var_xmp_dn2 = assign41720_body1_e55559_d_n2;
            locals.var_xmp_dn4 = assign41720_body1_e55559_d_n4;
            locals.var_xmp_dn5 = assign41720_body1_e55559_d_n5;
            locals.var_xmp_dn6 = assign41720_body1_e55559_d_n6;
            locals.var_xmp_dn7 = assign41720_body1_e55559_d_n7;
            locals.var_xmp_dn8 = assign41720_body1_e55559_d_n8;
            locals.var_xmp_dn9 = assign41720_body1_e55559_d_n9;
            locals.var_xmp_dn10 = assign41720_body1_e55559_d_n10;
            locals.var_xmp_dn11 = assign41720_body1_e55559_d_n11;
            locals.var_xmp_dn14 = assign41720_body1_e55559_d_n14;
            locals.var_xmp_rv = 0.0;
            let (assign41720_body2_e55575,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1041 != 0.0)) {
        let assign41720_body2_e55573: f64 = (locals.var_m0 + 1.0);
        (assign41720_body2_e55573,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign41720_body2_e55575;
            locals.var_m0_rv = 0.0;
        }

        let (assign41730_e55591, assign41730_e55591_d_n0, assign41730_e55591_d_n2, assign41730_e55591_d_n4, assign41730_e55591_d_n5, assign41730_e55591_d_n6, assign41730_e55591_d_n7, assign41730_e55591_d_n8, assign41730_e55591_d_n9, assign41730_e55591_d_n10, assign41730_e55591_d_n11, assign41730_e55591_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1041 != 0.0)) {
        let assign41730_e55589: f64 = (locals.var_xp + locals.var_xmp);
        (assign41730_e55589, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign41730_e55591;
        locals.var_arg_dn0 = assign41730_e55591_d_n0;
        locals.var_arg_dn2 = assign41730_e55591_d_n2;
        locals.var_arg_dn4 = assign41730_e55591_d_n4;
        locals.var_arg_dn5 = assign41730_e55591_d_n5;
        locals.var_arg_dn6 = assign41730_e55591_d_n6;
        locals.var_arg_dn7 = assign41730_e55591_d_n7;
        locals.var_arg_dn8 = assign41730_e55591_d_n8;
        locals.var_arg_dn9 = assign41730_e55591_d_n9;
        locals.var_arg_dn10 = assign41730_e55591_d_n10;
        locals.var_arg_dn11 = assign41730_e55591_d_n11;
        locals.var_arg_dn14 = assign41730_e55591_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign41740_e55605, assign41740_e55605_d_n0, assign41740_e55605_d_n2, assign41740_e55605_d_n4, assign41740_e55605_d_n5, assign41740_e55605_d_n6, assign41740_e55605_d_n7, assign41740_e55605_d_n8, assign41740_e55605_d_n9, assign41740_e55605_d_n10, assign41740_e55605_d_n11, assign41740_e55605_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1041 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign41740_e55605;
        locals.var_dnm_dn0 = assign41740_e55605_d_n0;
        locals.var_dnm_dn2 = assign41740_e55605_d_n2;
        locals.var_dnm_dn4 = assign41740_e55605_d_n4;
        locals.var_dnm_dn5 = assign41740_e55605_d_n5;
        locals.var_dnm_dn6 = assign41740_e55605_d_n6;
        locals.var_dnm_dn7 = assign41740_e55605_d_n7;
        locals.var_dnm_dn8 = assign41740_e55605_d_n8;
        locals.var_dnm_dn9 = assign41740_e55605_d_n9;
        locals.var_dnm_dn10 = assign41740_e55605_d_n10;
        locals.var_dnm_dn11 = assign41740_e55605_d_n11;
        locals.var_dnm_dn14 = assign41740_e55605_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign41750_e55620: f64 = if ((((locals.var_vgpdep_pw == 1.0) || (locals.var_vgpdep_pw == 2.0)) || (locals.var_vgpdep_pw == 4.0)) || (locals.var_vgpdep_pw == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1042 = assign41750_e55620;
        locals.var_guard1042_rv = 0.0;

        let assign41760_e55623: f64 = if locals.var_vgpdep_pw == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1043 = assign41760_e55623;
        locals.var_guard1043_rv = 0.0;

        let (assign41770_e55641,) = {
    if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1041 != 0.0)) && (locals.var_guard1042 != 0.0)) && (locals.var_guard1043 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign41770_e55641;
        locals.var_mm_rv = 0.0;

        let assign41780_e55644: f64 = if locals.var_vgpdep_pw == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1044 = assign41780_e55644;
        locals.var_guard1044_rv = 0.0;

        let (assign41790_e55665,) = {
    if (((((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1041 != 0.0)) && (locals.var_guard1042 != 0.0)) && (locals.var_guard1043 == 0.0)) && (locals.var_guard1044 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign41790_e55665;
        locals.var_mm_rv = 0.0;

        let assign41800_e55668: f64 = if locals.var_vgpdep_pw == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1045 = assign41800_e55668;
        locals.var_guard1045_rv = 0.0;

        let (assign41810_e55692,) = {
    if ((((((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1041 != 0.0)) && (locals.var_guard1042 != 0.0)) && (locals.var_guard1043 == 0.0)) && (locals.var_guard1044 == 0.0)) && (locals.var_guard1045 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign41810_e55692;
        locals.var_mm_rv = 0.0;

        let assign41820_e55695: f64 = if locals.var_vgpdep_pw == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1046 = assign41820_e55695;
        locals.var_guard1046_rv = 0.0;

        let (assign41830_e55722,) = {
    if (((((((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1041 != 0.0)) && (locals.var_guard1042 != 0.0)) && (locals.var_guard1043 == 0.0)) && (locals.var_guard1044 == 0.0)) && (locals.var_guard1045 == 0.0)) && (locals.var_guard1046 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign41830_e55722;
        locals.var_mm_rv = 0.0;

        let (assign41840_e55738,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1041 != 0.0)) && (locals.var_guard1042 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign41840_e55738;
        locals.var_m0_rv = 0.0;

        let mut assign41850_loop_guard: usize = 0;
        while {
            let assign41850_cond_e55755: f64 = if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1041 != 0.0)) && (locals.var_guard1042 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign41850_cond_e55755 != 0.0
        } {
            assign41850_loop_guard += 1;
            assert!(assign41850_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign41850_body0_e55772, assign41850_body0_e55772_d_n0, assign41850_body0_e55772_d_n2, assign41850_body0_e55772_d_n4, assign41850_body0_e55772_d_n5, assign41850_body0_e55772_d_n6, assign41850_body0_e55772_d_n7, assign41850_body0_e55772_d_n8, assign41850_body0_e55772_d_n9, assign41850_body0_e55772_d_n10, assign41850_body0_e55772_d_n11, assign41850_body0_e55772_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1041 != 0.0)) && (locals.var_guard1042 != 0.0)) {
        let assign41850_body0_e55770: f64 = (locals.var_dnm).sqrt();
        (assign41850_body0_e55770, (locals.var_dnm_dn0 / (2.0 * assign41850_body0_e55770)), (locals.var_dnm_dn2 / (2.0 * assign41850_body0_e55770)), (locals.var_dnm_dn4 / (2.0 * assign41850_body0_e55770)), (locals.var_dnm_dn5 / (2.0 * assign41850_body0_e55770)), (locals.var_dnm_dn6 / (2.0 * assign41850_body0_e55770)), (locals.var_dnm_dn7 / (2.0 * assign41850_body0_e55770)), (locals.var_dnm_dn8 / (2.0 * assign41850_body0_e55770)), (locals.var_dnm_dn9 / (2.0 * assign41850_body0_e55770)), (locals.var_dnm_dn10 / (2.0 * assign41850_body0_e55770)), (locals.var_dnm_dn11 / (2.0 * assign41850_body0_e55770)), (locals.var_dnm_dn14 / (2.0 * assign41850_body0_e55770)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign41850_body0_e55772;
            locals.var_dnm_dn0 = assign41850_body0_e55772_d_n0;
            locals.var_dnm_dn2 = assign41850_body0_e55772_d_n2;
            locals.var_dnm_dn4 = assign41850_body0_e55772_d_n4;
            locals.var_dnm_dn5 = assign41850_body0_e55772_d_n5;
            locals.var_dnm_dn6 = assign41850_body0_e55772_d_n6;
            locals.var_dnm_dn7 = assign41850_body0_e55772_d_n7;
            locals.var_dnm_dn8 = assign41850_body0_e55772_d_n8;
            locals.var_dnm_dn9 = assign41850_body0_e55772_d_n9;
            locals.var_dnm_dn10 = assign41850_body0_e55772_d_n10;
            locals.var_dnm_dn11 = assign41850_body0_e55772_d_n11;
            locals.var_dnm_dn14 = assign41850_body0_e55772_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign41850_body1_e55790,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1041 != 0.0)) && (locals.var_guard1042 != 0.0)) {
        let assign41850_body1_e55788: f64 = (locals.var_m0 + 1.0);
        (assign41850_body1_e55788,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign41850_body1_e55790;
            locals.var_m0_rv = 0.0;
        }

        let (assign41860_e55818, assign41860_e55818_d_n0, assign41860_e55818_d_n2, assign41860_e55818_d_n4, assign41860_e55818_d_n5, assign41860_e55818_d_n6, assign41860_e55818_d_n7, assign41860_e55818_d_n8, assign41860_e55818_d_n9, assign41860_e55818_d_n10, assign41860_e55818_d_n11, assign41860_e55818_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1041 != 0.0)) && (locals.var_guard1042 == 0.0)) {
        let (assign41860_e55816, assign41860_e55816_d_n0, assign41860_e55816_d_n2, assign41860_e55816_d_n4, assign41860_e55816_d_n5, assign41860_e55816_d_n6, assign41860_e55816_d_n7, assign41860_e55816_d_n8, assign41860_e55816_d_n9, assign41860_e55816_d_n10, assign41860_e55816_d_n11, assign41860_e55816_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign41860_e55813: f64 = (2.0 * locals.var_vgpdep_pw);
                let assign41860_e55814: f64 = (1.0 / assign41860_e55813);
                let assign41860_e55815: f64 = (locals.var_dnm).powf(assign41860_e55814);
                (assign41860_e55815, if 0.0 == 0.0 && ((assign41860_e55814) as f64).is_finite() && ((assign41860_e55814) as f64).fract() == 0.0 { if assign41860_e55814 == 0.0 { 0.0 } else { (assign41860_e55814 * ((locals.var_dnm).powf(assign41860_e55814 - 1.0) * locals.var_dnm_dn0)) } } else { (assign41860_e55815 * (assign41860_e55814 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41860_e55814) as f64).is_finite() && ((assign41860_e55814) as f64).fract() == 0.0 { if assign41860_e55814 == 0.0 { 0.0 } else { (assign41860_e55814 * ((locals.var_dnm).powf(assign41860_e55814 - 1.0) * locals.var_dnm_dn2)) } } else { (assign41860_e55815 * (assign41860_e55814 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41860_e55814) as f64).is_finite() && ((assign41860_e55814) as f64).fract() == 0.0 { if assign41860_e55814 == 0.0 { 0.0 } else { (assign41860_e55814 * ((locals.var_dnm).powf(assign41860_e55814 - 1.0) * locals.var_dnm_dn4)) } } else { (assign41860_e55815 * (assign41860_e55814 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41860_e55814) as f64).is_finite() && ((assign41860_e55814) as f64).fract() == 0.0 { if assign41860_e55814 == 0.0 { 0.0 } else { (assign41860_e55814 * ((locals.var_dnm).powf(assign41860_e55814 - 1.0) * locals.var_dnm_dn5)) } } else { (assign41860_e55815 * (assign41860_e55814 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41860_e55814) as f64).is_finite() && ((assign41860_e55814) as f64).fract() == 0.0 { if assign41860_e55814 == 0.0 { 0.0 } else { (assign41860_e55814 * ((locals.var_dnm).powf(assign41860_e55814 - 1.0) * locals.var_dnm_dn6)) } } else { (assign41860_e55815 * (assign41860_e55814 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41860_e55814) as f64).is_finite() && ((assign41860_e55814) as f64).fract() == 0.0 { if assign41860_e55814 == 0.0 { 0.0 } else { (assign41860_e55814 * ((locals.var_dnm).powf(assign41860_e55814 - 1.0) * locals.var_dnm_dn7)) } } else { (assign41860_e55815 * (assign41860_e55814 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41860_e55814) as f64).is_finite() && ((assign41860_e55814) as f64).fract() == 0.0 { if assign41860_e55814 == 0.0 { 0.0 } else { (assign41860_e55814 * ((locals.var_dnm).powf(assign41860_e55814 - 1.0) * locals.var_dnm_dn8)) } } else { (assign41860_e55815 * (assign41860_e55814 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41860_e55814) as f64).is_finite() && ((assign41860_e55814) as f64).fract() == 0.0 { if assign41860_e55814 == 0.0 { 0.0 } else { (assign41860_e55814 * ((locals.var_dnm).powf(assign41860_e55814 - 1.0) * locals.var_dnm_dn9)) } } else { (assign41860_e55815 * (assign41860_e55814 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41860_e55814) as f64).is_finite() && ((assign41860_e55814) as f64).fract() == 0.0 { if assign41860_e55814 == 0.0 { 0.0 } else { (assign41860_e55814 * ((locals.var_dnm).powf(assign41860_e55814 - 1.0) * locals.var_dnm_dn10)) } } else { (assign41860_e55815 * (assign41860_e55814 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41860_e55814) as f64).is_finite() && ((assign41860_e55814) as f64).fract() == 0.0 { if assign41860_e55814 == 0.0 { 0.0 } else { (assign41860_e55814 * ((locals.var_dnm).powf(assign41860_e55814 - 1.0) * locals.var_dnm_dn11)) } } else { (assign41860_e55815 * (assign41860_e55814 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign41860_e55814) as f64).is_finite() && ((assign41860_e55814) as f64).fract() == 0.0 { if assign41860_e55814 == 0.0 { 0.0 } else { (assign41860_e55814 * ((locals.var_dnm).powf(assign41860_e55814 - 1.0) * locals.var_dnm_dn14)) } } else { (assign41860_e55815 * (assign41860_e55814 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign41860_e55816, assign41860_e55816_d_n0, assign41860_e55816_d_n2, assign41860_e55816_d_n4, assign41860_e55816_d_n5, assign41860_e55816_d_n6, assign41860_e55816_d_n7, assign41860_e55816_d_n8, assign41860_e55816_d_n9, assign41860_e55816_d_n10, assign41860_e55816_d_n11, assign41860_e55816_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign41860_e55818;
        locals.var_dnm_dn0 = assign41860_e55818_d_n0;
        locals.var_dnm_dn2 = assign41860_e55818_d_n2;
        locals.var_dnm_dn4 = assign41860_e55818_d_n4;
        locals.var_dnm_dn5 = assign41860_e55818_d_n5;
        locals.var_dnm_dn6 = assign41860_e55818_d_n6;
        locals.var_dnm_dn7 = assign41860_e55818_d_n7;
        locals.var_dnm_dn8 = assign41860_e55818_d_n8;
        locals.var_dnm_dn9 = assign41860_e55818_d_n9;
        locals.var_dnm_dn10 = assign41860_e55818_d_n10;
        locals.var_dnm_dn11 = assign41860_e55818_d_n11;
        locals.var_dnm_dn14 = assign41860_e55818_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign41870_e55834, assign41870_e55834_d_n0, assign41870_e55834_d_n2, assign41870_e55834_d_n4, assign41870_e55834_d_n5, assign41870_e55834_d_n6, assign41870_e55834_d_n7, assign41870_e55834_d_n8, assign41870_e55834_d_n9, assign41870_e55834_d_n10, assign41870_e55834_d_n11, assign41870_e55834_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1041 != 0.0)) {
        let assign41870_e55832: f64 = (1.0 / locals.var_dnm);
        (assign41870_e55832, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign41870_e55834;
        locals.var_dnm_dn0 = assign41870_e55834_d_n0;
        locals.var_dnm_dn2 = assign41870_e55834_d_n2;
        locals.var_dnm_dn4 = assign41870_e55834_d_n4;
        locals.var_dnm_dn5 = assign41870_e55834_d_n5;
        locals.var_dnm_dn6 = assign41870_e55834_d_n6;
        locals.var_dnm_dn7 = assign41870_e55834_d_n7;
        locals.var_dnm_dn8 = assign41870_e55834_d_n8;
        locals.var_dnm_dn9 = assign41870_e55834_d_n9;
        locals.var_dnm_dn10 = assign41870_e55834_d_n10;
        locals.var_dnm_dn11 = assign41870_e55834_d_n11;
        locals.var_dnm_dn14 = assign41870_e55834_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign41880_e55852, assign41880_e55852_d_n0, assign41880_e55852_d_n2, assign41880_e55852_d_n4, assign41880_e55852_d_n5, assign41880_e55852_d_n6, assign41880_e55852_d_n7, assign41880_e55852_d_n8, assign41880_e55852_d_n9, assign41880_e55852_d_n10, assign41880_e55852_d_n11, assign41880_e55852_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1041 != 0.0)) {
        let assign41880_e55848: f64 = (locals.var_tmf1 * locals.var_vgpdep_dlt);
        let assign41880_e55850: f64 = (assign41880_e55848 * locals.var_dnm);
        (assign41880_e55850, ((((locals.var_tmf1_dn0 * locals.var_vgpdep_dlt) + (locals.var_tmf1 * locals.var_vgpdep_dlt_dn0)) * locals.var_dnm) + (assign41880_e55848 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_vgpdep_dlt) + (locals.var_tmf1 * locals.var_vgpdep_dlt_dn2)) * locals.var_dnm) + (assign41880_e55848 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * locals.var_vgpdep_dlt) + (locals.var_tmf1 * locals.var_vgpdep_dlt_dn4)) * locals.var_dnm) + (assign41880_e55848 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * locals.var_vgpdep_dlt) + (locals.var_tmf1 * locals.var_vgpdep_dlt_dn5)) * locals.var_dnm) + (assign41880_e55848 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * locals.var_vgpdep_dlt) + (locals.var_tmf1 * locals.var_vgpdep_dlt_dn6)) * locals.var_dnm) + (assign41880_e55848 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_vgpdep_dlt) + (locals.var_tmf1 * locals.var_vgpdep_dlt_dn7)) * locals.var_dnm) + (assign41880_e55848 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * locals.var_vgpdep_dlt) + (locals.var_tmf1 * locals.var_vgpdep_dlt_dn8)) * locals.var_dnm) + (assign41880_e55848 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * locals.var_vgpdep_dlt) + (locals.var_tmf1 * locals.var_vgpdep_dlt_dn9)) * locals.var_dnm) + (assign41880_e55848 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * locals.var_vgpdep_dlt) + (locals.var_tmf1 * locals.var_vgpdep_dlt_dn10)) * locals.var_dnm) + (assign41880_e55848 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * locals.var_vgpdep_dlt) + (locals.var_tmf1 * locals.var_vgpdep_dlt_dn11)) * locals.var_dnm) + (assign41880_e55848 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * locals.var_vgpdep_dlt) + (locals.var_tmf1 * locals.var_vgpdep_dlt_dn14)) * locals.var_dnm) + (assign41880_e55848 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign41880_e55852;
        locals.var_tmf0_dn0 = assign41880_e55852_d_n0;
        locals.var_tmf0_dn2 = assign41880_e55852_d_n2;
        locals.var_tmf0_dn4 = assign41880_e55852_d_n4;
        locals.var_tmf0_dn5 = assign41880_e55852_d_n5;
        locals.var_tmf0_dn6 = assign41880_e55852_d_n6;
        locals.var_tmf0_dn7 = assign41880_e55852_d_n7;
        locals.var_tmf0_dn8 = assign41880_e55852_d_n8;
        locals.var_tmf0_dn9 = assign41880_e55852_d_n9;
        locals.var_tmf0_dn10 = assign41880_e55852_d_n10;
        locals.var_tmf0_dn11 = assign41880_e55852_d_n11;
        locals.var_tmf0_dn14 = assign41880_e55852_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign41890_e55872, assign41890_e55872_d_n0, assign41890_e55872_d_n2, assign41890_e55872_d_n4, assign41890_e55872_d_n5, assign41890_e55872_d_n6, assign41890_e55872_d_n7, assign41890_e55872_d_n8, assign41890_e55872_d_n9, assign41890_e55872_d_n10, assign41890_e55872_d_n11, assign41890_e55872_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1041 != 0.0)) {
        let assign41890_e55866: f64 = (locals.var_vgpdep_dlt * locals.var_xmp);
        let assign41890_e55868: f64 = (assign41890_e55866 * locals.var_dnm);
        let assign41890_e55870: f64 = (assign41890_e55868 / locals.var_arg);
        (assign41890_e55870, (((((((locals.var_vgpdep_dlt_dn0 * locals.var_xmp) + (locals.var_vgpdep_dlt * locals.var_xmp_dn0)) * locals.var_dnm) + (assign41890_e55866 * locals.var_dnm_dn0)) * locals.var_arg) - (assign41890_e55868 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_vgpdep_dlt_dn2 * locals.var_xmp) + (locals.var_vgpdep_dlt * locals.var_xmp_dn2)) * locals.var_dnm) + (assign41890_e55866 * locals.var_dnm_dn2)) * locals.var_arg) - (assign41890_e55868 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_vgpdep_dlt_dn4 * locals.var_xmp) + (locals.var_vgpdep_dlt * locals.var_xmp_dn4)) * locals.var_dnm) + (assign41890_e55866 * locals.var_dnm_dn4)) * locals.var_arg) - (assign41890_e55868 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_vgpdep_dlt_dn5 * locals.var_xmp) + (locals.var_vgpdep_dlt * locals.var_xmp_dn5)) * locals.var_dnm) + (assign41890_e55866 * locals.var_dnm_dn5)) * locals.var_arg) - (assign41890_e55868 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_vgpdep_dlt_dn6 * locals.var_xmp) + (locals.var_vgpdep_dlt * locals.var_xmp_dn6)) * locals.var_dnm) + (assign41890_e55866 * locals.var_dnm_dn6)) * locals.var_arg) - (assign41890_e55868 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_vgpdep_dlt_dn7 * locals.var_xmp) + (locals.var_vgpdep_dlt * locals.var_xmp_dn7)) * locals.var_dnm) + (assign41890_e55866 * locals.var_dnm_dn7)) * locals.var_arg) - (assign41890_e55868 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_vgpdep_dlt_dn8 * locals.var_xmp) + (locals.var_vgpdep_dlt * locals.var_xmp_dn8)) * locals.var_dnm) + (assign41890_e55866 * locals.var_dnm_dn8)) * locals.var_arg) - (assign41890_e55868 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_vgpdep_dlt_dn9 * locals.var_xmp) + (locals.var_vgpdep_dlt * locals.var_xmp_dn9)) * locals.var_dnm) + (assign41890_e55866 * locals.var_dnm_dn9)) * locals.var_arg) - (assign41890_e55868 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_vgpdep_dlt_dn10 * locals.var_xmp) + (locals.var_vgpdep_dlt * locals.var_xmp_dn10)) * locals.var_dnm) + (assign41890_e55866 * locals.var_dnm_dn10)) * locals.var_arg) - (assign41890_e55868 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_vgpdep_dlt_dn11 * locals.var_xmp) + (locals.var_vgpdep_dlt * locals.var_xmp_dn11)) * locals.var_dnm) + (assign41890_e55866 * locals.var_dnm_dn11)) * locals.var_arg) - (assign41890_e55868 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_vgpdep_dlt_dn14 * locals.var_xmp) + (locals.var_vgpdep_dlt * locals.var_xmp_dn14)) * locals.var_dnm) + (assign41890_e55866 * locals.var_dnm_dn14)) * locals.var_arg) - (assign41890_e55868 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign41890_e55872;
        locals.var_t0_dn0 = assign41890_e55872_d_n0;
        locals.var_t0_dn2 = assign41890_e55872_d_n2;
        locals.var_t0_dn4 = assign41890_e55872_d_n4;
        locals.var_t0_dn5 = assign41890_e55872_d_n5;
        locals.var_t0_dn6 = assign41890_e55872_d_n6;
        locals.var_t0_dn7 = assign41890_e55872_d_n7;
        locals.var_t0_dn8 = assign41890_e55872_d_n8;
        locals.var_t0_dn9 = assign41890_e55872_d_n9;
        locals.var_t0_dn10 = assign41890_e55872_d_n10;
        locals.var_t0_dn11 = assign41890_e55872_d_n11;
        locals.var_t0_dn14 = assign41890_e55872_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign41900_e55890, assign41900_e55890_d_n0, assign41900_e55890_d_n2, assign41900_e55890_d_n4, assign41900_e55890_d_n5, assign41900_e55890_d_n6, assign41900_e55890_d_n7, assign41900_e55890_d_n8, assign41900_e55890_d_n9, assign41900_e55890_d_n10, assign41900_e55890_d_n11, assign41900_e55890_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1041 != 0.0)) {
        let assign41900_e55886: f64 = (-locals.var_vgpdep_dlt);
        let assign41900_e55888: f64 = (assign41900_e55886 + locals.var_tmf0);
        (assign41900_e55888, ((-locals.var_vgpdep_dlt_dn0) + locals.var_tmf0_dn0), ((-locals.var_vgpdep_dlt_dn2) + locals.var_tmf0_dn2), ((-locals.var_vgpdep_dlt_dn4) + locals.var_tmf0_dn4), ((-locals.var_vgpdep_dlt_dn5) + locals.var_tmf0_dn5), ((-locals.var_vgpdep_dlt_dn6) + locals.var_tmf0_dn6), ((-locals.var_vgpdep_dlt_dn7) + locals.var_tmf0_dn7), ((-locals.var_vgpdep_dlt_dn8) + locals.var_tmf0_dn8), ((-locals.var_vgpdep_dlt_dn9) + locals.var_tmf0_dn9), ((-locals.var_vgpdep_dlt_dn10) + locals.var_tmf0_dn10), ((-locals.var_vgpdep_dlt_dn11) + locals.var_tmf0_dn11), ((-locals.var_vgpdep_dlt_dn14) + locals.var_tmf0_dn14),)
    } else {
        (locals.var_vgp_res, locals.var_vgp_res_dn0, locals.var_vgp_res_dn2, locals.var_vgp_res_dn4, locals.var_vgp_res_dn5, locals.var_vgp_res_dn6, locals.var_vgp_res_dn7, locals.var_vgp_res_dn8, locals.var_vgp_res_dn9, locals.var_vgp_res_dn10, locals.var_vgp_res_dn11, locals.var_vgp_res_dn14,)
    }
};
        locals.var_vgp_res = assign41900_e55890;
        locals.var_vgp_res_dn0 = assign41900_e55890_d_n0;
        locals.var_vgp_res_dn2 = assign41900_e55890_d_n2;
        locals.var_vgp_res_dn4 = assign41900_e55890_d_n4;
        locals.var_vgp_res_dn5 = assign41900_e55890_d_n5;
        locals.var_vgp_res_dn6 = assign41900_e55890_d_n6;
        locals.var_vgp_res_dn7 = assign41900_e55890_d_n7;
        locals.var_vgp_res_dn8 = assign41900_e55890_d_n8;
        locals.var_vgp_res_dn9 = assign41900_e55890_d_n9;
        locals.var_vgp_res_dn10 = assign41900_e55890_d_n10;
        locals.var_vgp_res_dn11 = assign41900_e55890_d_n11;
        locals.var_vgp_res_dn14 = assign41900_e55890_d_n14;
        locals.var_vgp_res_rv = 0.0;

        let (assign41910_e55904, assign41910_e55904_d_n0, assign41910_e55904_d_n2, assign41910_e55904_d_n4, assign41910_e55904_d_n5, assign41910_e55904_d_n6, assign41910_e55904_d_n7, assign41910_e55904_d_n8, assign41910_e55904_d_n9, assign41910_e55904_d_n10, assign41910_e55904_d_n11, assign41910_e55904_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1041 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign41910_e55904;
        locals.var_t0_dn0 = assign41910_e55904_d_n0;
        locals.var_t0_dn2 = assign41910_e55904_d_n2;
        locals.var_t0_dn4 = assign41910_e55904_d_n4;
        locals.var_t0_dn5 = assign41910_e55904_d_n5;
        locals.var_t0_dn6 = assign41910_e55904_d_n6;
        locals.var_t0_dn7 = assign41910_e55904_d_n7;
        locals.var_t0_dn8 = assign41910_e55904_d_n8;
        locals.var_t0_dn9 = assign41910_e55904_d_n9;
        locals.var_t0_dn10 = assign41910_e55904_d_n10;
        locals.var_t0_dn11 = assign41910_e55904_d_n11;
        locals.var_t0_dn14 = assign41910_e55904_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign41920_e55919, assign41920_e55919_d_n0, assign41920_e55919_d_n2, assign41920_e55919_d_n4, assign41920_e55919_d_n5, assign41920_e55919_d_n6, assign41920_e55919_d_n7, assign41920_e55919_d_n8, assign41920_e55919_d_n9, assign41920_e55919_d_n10, assign41920_e55919_d_n11, assign41920_e55919_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1041 == 0.0)) {
        (locals.var_vgp_res, locals.var_vgp_res_dn0, locals.var_vgp_res_dn2, locals.var_vgp_res_dn4, locals.var_vgp_res_dn5, locals.var_vgp_res_dn6, locals.var_vgp_res_dn7, locals.var_vgp_res_dn8, locals.var_vgp_res_dn9, locals.var_vgp_res_dn10, locals.var_vgp_res_dn11, locals.var_vgp_res_dn14,)
    } else {
        (locals.var_vgp_res, locals.var_vgp_res_dn0, locals.var_vgp_res_dn2, locals.var_vgp_res_dn4, locals.var_vgp_res_dn5, locals.var_vgp_res_dn6, locals.var_vgp_res_dn7, locals.var_vgp_res_dn8, locals.var_vgp_res_dn9, locals.var_vgp_res_dn10, locals.var_vgp_res_dn11, locals.var_vgp_res_dn14,)
    }
};
        locals.var_vgp_res = assign41920_e55919;
        locals.var_vgp_res_dn0 = assign41920_e55919_d_n0;
        locals.var_vgp_res_dn2 = assign41920_e55919_d_n2;
        locals.var_vgp_res_dn4 = assign41920_e55919_d_n4;
        locals.var_vgp_res_dn5 = assign41920_e55919_d_n5;
        locals.var_vgp_res_dn6 = assign41920_e55919_d_n6;
        locals.var_vgp_res_dn7 = assign41920_e55919_d_n7;
        locals.var_vgp_res_dn8 = assign41920_e55919_d_n8;
        locals.var_vgp_res_dn9 = assign41920_e55919_d_n9;
        locals.var_vgp_res_dn10 = assign41920_e55919_d_n10;
        locals.var_vgp_res_dn11 = assign41920_e55919_d_n11;
        locals.var_vgp_res_dn14 = assign41920_e55919_d_n14;
        locals.var_vgp_res_rv = 0.0;

        let (assign41930_e55934, assign41930_e55934_d_n0, assign41930_e55934_d_n2, assign41930_e55934_d_n4, assign41930_e55934_d_n5, assign41930_e55934_d_n6, assign41930_e55934_d_n7, assign41930_e55934_d_n8, assign41930_e55934_d_n9, assign41930_e55934_d_n10, assign41930_e55934_d_n11, assign41930_e55934_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1041 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign41930_e55934;
        locals.var_t0_dn0 = assign41930_e55934_d_n0;
        locals.var_t0_dn2 = assign41930_e55934_d_n2;
        locals.var_t0_dn4 = assign41930_e55934_d_n4;
        locals.var_t0_dn5 = assign41930_e55934_d_n5;
        locals.var_t0_dn6 = assign41930_e55934_d_n6;
        locals.var_t0_dn7 = assign41930_e55934_d_n7;
        locals.var_t0_dn8 = assign41930_e55934_d_n8;
        locals.var_t0_dn9 = assign41930_e55934_d_n9;
        locals.var_t0_dn10 = assign41930_e55934_d_n10;
        locals.var_t0_dn11 = assign41930_e55934_d_n11;
        locals.var_t0_dn14 = assign41930_e55934_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign41940_e55946,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
        locals.var_flg_conv = assign41940_e55946;
        locals.var_flg_conv_rv = 0.0;

        let (assign41950_e55958,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign41950_e55958;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_144(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign41960_loop_guard: usize = 0;
        while {
            let assign41960_cond_e55971: f64 = if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_lp_s0 <= 150.0)) { 1.0 } else { 0.0 };
            assign41960_cond_e55971 != 0.0
        } {
            assign41960_loop_guard += 1;
            assert!(assign41960_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign41960_body0_e55985, assign41960_body0_e55985_d_n0, assign41960_body0_e55985_d_n2, assign41960_body0_e55985_d_n4, assign41960_body0_e55985_d_n5, assign41960_body0_e55985_d_n6, assign41960_body0_e55985_d_n7, assign41960_body0_e55985_d_n8, assign41960_body0_e55985_d_n9, assign41960_body0_e55985_d_n10, assign41960_body0_e55985_d_n11, assign41960_body0_e55985_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) {
        let assign41960_body0_e55983: f64 = (locals.var_beta * locals.var_ps0dep);
        (assign41960_body0_e55983, ((locals.var_beta_dn0 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn0)), ((locals.var_beta_dn2 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn2)), ((locals.var_beta_dn4 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn4)), ((locals.var_beta_dn5 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn5)), ((locals.var_beta_dn6 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn6)), ((locals.var_beta_dn7 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn7)), ((locals.var_beta_dn8 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn8)), ((locals.var_beta_dn9 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn9)), ((locals.var_beta_dn10 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn10)), ((locals.var_beta_dn11 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn11)), ((locals.var_beta_dn14 * locals.var_ps0dep) + (locals.var_beta * locals.var_ps0dep_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign41960_body0_e55985;
            locals.var_t1_dn0 = assign41960_body0_e55985_d_n0;
            locals.var_t1_dn2 = assign41960_body0_e55985_d_n2;
            locals.var_t1_dn4 = assign41960_body0_e55985_d_n4;
            locals.var_t1_dn5 = assign41960_body0_e55985_d_n5;
            locals.var_t1_dn6 = assign41960_body0_e55985_d_n6;
            locals.var_t1_dn7 = assign41960_body0_e55985_d_n7;
            locals.var_t1_dn8 = assign41960_body0_e55985_d_n8;
            locals.var_t1_dn9 = assign41960_body0_e55985_d_n9;
            locals.var_t1_dn10 = assign41960_body0_e55985_d_n10;
            locals.var_t1_dn11 = assign41960_body0_e55985_d_n11;
            locals.var_t1_dn14 = assign41960_body0_e55985_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign41960_body1_e55998, assign41960_body1_e55998_d_n0, assign41960_body1_e55998_d_n2, assign41960_body1_e55998_d_n4, assign41960_body1_e55998_d_n5, assign41960_body1_e55998_d_n6, assign41960_body1_e55998_d_n7, assign41960_body1_e55998_d_n8, assign41960_body1_e55998_d_n9, assign41960_body1_e55998_d_n10, assign41960_body1_e55998_d_n11, assign41960_body1_e55998_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) {
        let assign41960_body1_e55996: f64 = (locals.var_t1).exp();
        (assign41960_body1_e55996, (assign41960_body1_e55996 * locals.var_t1_dn0), (assign41960_body1_e55996 * locals.var_t1_dn2), (assign41960_body1_e55996 * locals.var_t1_dn4), (assign41960_body1_e55996 * locals.var_t1_dn5), (assign41960_body1_e55996 * locals.var_t1_dn6), (assign41960_body1_e55996 * locals.var_t1_dn7), (assign41960_body1_e55996 * locals.var_t1_dn8), (assign41960_body1_e55996 * locals.var_t1_dn9), (assign41960_body1_e55996 * locals.var_t1_dn10), (assign41960_body1_e55996 * locals.var_t1_dn11), (assign41960_body1_e55996 * locals.var_t1_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
            locals.var_t2 = assign41960_body1_e55998;
            locals.var_t2_dn0 = assign41960_body1_e55998_d_n0;
            locals.var_t2_dn2 = assign41960_body1_e55998_d_n2;
            locals.var_t2_dn4 = assign41960_body1_e55998_d_n4;
            locals.var_t2_dn5 = assign41960_body1_e55998_d_n5;
            locals.var_t2_dn6 = assign41960_body1_e55998_d_n6;
            locals.var_t2_dn7 = assign41960_body1_e55998_d_n7;
            locals.var_t2_dn8 = assign41960_body1_e55998_d_n8;
            locals.var_t2_dn9 = assign41960_body1_e55998_d_n9;
            locals.var_t2_dn10 = assign41960_body1_e55998_d_n10;
            locals.var_t2_dn11 = assign41960_body1_e55998_d_n11;
            locals.var_t2_dn14 = assign41960_body1_e55998_d_n14;
            locals.var_t2_rv = 0.0;
            let assign41960_body2_e56001: f64 = if locals.var_ps0dep >= 0.0 { 1.0 } else { 0.0 };
            locals.var_guard1047 = assign41960_body2_e56001;
            locals.var_guard1047_rv = 0.0;
            let (assign41960_body3_e56025, assign41960_body3_e56025_d_n0, assign41960_body3_e56025_d_n2, assign41960_body3_e56025_d_n4, assign41960_body3_e56025_d_n5, assign41960_body3_e56025_d_n6, assign41960_body3_e56025_d_n7, assign41960_body3_e56025_d_n8, assign41960_body3_e56025_d_n9, assign41960_body3_e56025_d_n10, assign41960_body3_e56025_d_n11, assign41960_body3_e56025_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1047 != 0.0)) {
        let assign41960_body3_e56014: f64 = (-locals.var_cnst0);
        let assign41960_body3_e56017: f64 = (locals.var_t2 - 1.0);
        let assign41960_body3_e56019: f64 = (assign41960_body3_e56017 - locals.var_t1);
        let assign41960_body3_e56021: f64 = (assign41960_body3_e56019 + 1e-15);
        let assign41960_body3_e56022: f64 = (assign41960_body3_e56021).sqrt();
        let assign41960_body3_e56023: f64 = (assign41960_body3_e56014 * assign41960_body3_e56022);
        (assign41960_body3_e56023, (((-locals.var_cnst0_dn0) * assign41960_body3_e56022) + (assign41960_body3_e56014 * ((locals.var_t2_dn0 - locals.var_t1_dn0) / (2.0 * assign41960_body3_e56022)))), (((-locals.var_cnst0_dn2) * assign41960_body3_e56022) + (assign41960_body3_e56014 * ((locals.var_t2_dn2 - locals.var_t1_dn2) / (2.0 * assign41960_body3_e56022)))), (((-locals.var_cnst0_dn4) * assign41960_body3_e56022) + (assign41960_body3_e56014 * ((locals.var_t2_dn4 - locals.var_t1_dn4) / (2.0 * assign41960_body3_e56022)))), (((-locals.var_cnst0_dn5) * assign41960_body3_e56022) + (assign41960_body3_e56014 * ((locals.var_t2_dn5 - locals.var_t1_dn5) / (2.0 * assign41960_body3_e56022)))), (((-locals.var_cnst0_dn6) * assign41960_body3_e56022) + (assign41960_body3_e56014 * ((locals.var_t2_dn6 - locals.var_t1_dn6) / (2.0 * assign41960_body3_e56022)))), (((-locals.var_cnst0_dn7) * assign41960_body3_e56022) + (assign41960_body3_e56014 * ((locals.var_t2_dn7 - locals.var_t1_dn7) / (2.0 * assign41960_body3_e56022)))), (((-locals.var_cnst0_dn8) * assign41960_body3_e56022) + (assign41960_body3_e56014 * ((locals.var_t2_dn8 - locals.var_t1_dn8) / (2.0 * assign41960_body3_e56022)))), (((-locals.var_cnst0_dn9) * assign41960_body3_e56022) + (assign41960_body3_e56014 * ((locals.var_t2_dn9 - locals.var_t1_dn9) / (2.0 * assign41960_body3_e56022)))), (((-locals.var_cnst0_dn10) * assign41960_body3_e56022) + (assign41960_body3_e56014 * ((locals.var_t2_dn10 - locals.var_t1_dn10) / (2.0 * assign41960_body3_e56022)))), (((-locals.var_cnst0_dn11) * assign41960_body3_e56022) + (assign41960_body3_e56014 * ((locals.var_t2_dn11 - locals.var_t1_dn11) / (2.0 * assign41960_body3_e56022)))), (((-locals.var_cnst0_dn14) * assign41960_body3_e56022) + (assign41960_body3_e56014 * ((locals.var_t2_dn14 - locals.var_t1_dn14) / (2.0 * assign41960_body3_e56022)))),)
    } else {
        (locals.var_q_s0__blk1032, locals.var_q_s0__blk1032_dn0, locals.var_q_s0__blk1032_dn2, locals.var_q_s0__blk1032_dn4, locals.var_q_s0__blk1032_dn5, locals.var_q_s0__blk1032_dn6, locals.var_q_s0__blk1032_dn7, locals.var_q_s0__blk1032_dn8, locals.var_q_s0__blk1032_dn9, locals.var_q_s0__blk1032_dn10, locals.var_q_s0__blk1032_dn11, locals.var_q_s0__blk1032_dn14,)
    }
};
            locals.var_q_s0__blk1032 = assign41960_body3_e56025;
            locals.var_q_s0__blk1032_dn0 = assign41960_body3_e56025_d_n0;
            locals.var_q_s0__blk1032_dn2 = assign41960_body3_e56025_d_n2;
            locals.var_q_s0__blk1032_dn4 = assign41960_body3_e56025_d_n4;
            locals.var_q_s0__blk1032_dn5 = assign41960_body3_e56025_d_n5;
            locals.var_q_s0__blk1032_dn6 = assign41960_body3_e56025_d_n6;
            locals.var_q_s0__blk1032_dn7 = assign41960_body3_e56025_d_n7;
            locals.var_q_s0__blk1032_dn8 = assign41960_body3_e56025_d_n8;
            locals.var_q_s0__blk1032_dn9 = assign41960_body3_e56025_d_n9;
            locals.var_q_s0__blk1032_dn10 = assign41960_body3_e56025_d_n10;
            locals.var_q_s0__blk1032_dn11 = assign41960_body3_e56025_d_n11;
            locals.var_q_s0__blk1032_dn14 = assign41960_body3_e56025_d_n14;
            locals.var_q_s0__blk1032_rv = 0.0;
            let (assign41960_body4_e56051, assign41960_body4_e56051_d_n0, assign41960_body4_e56051_d_n2, assign41960_body4_e56051_d_n4, assign41960_body4_e56051_d_n5, assign41960_body4_e56051_d_n6, assign41960_body4_e56051_d_n7, assign41960_body4_e56051_d_n8, assign41960_body4_e56051_d_n9, assign41960_body4_e56051_d_n10, assign41960_body4_e56051_d_n11, assign41960_body4_e56051_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1047 != 0.0)) {
        let assign41960_body4_e56039: f64 = (0.5 * locals.var_cnst0);
        let assign41960_body4_e56041: f64 = (assign41960_body4_e56039 * locals.var_cnst0);
        let assign41960_body4_e56043: f64 = (assign41960_body4_e56041 / locals.var_q_s0__blk1032);
        let assign41960_body4_e56046: f64 = (locals.var_beta * locals.var_t2);
        let assign41960_body4_e56048: f64 = (assign41960_body4_e56046 - locals.var_beta);
        let assign41960_body4_e56049: f64 = (assign41960_body4_e56043 * assign41960_body4_e56048);
        (assign41960_body4_e56049, ((((((((0.5 * locals.var_cnst0_dn0) * locals.var_cnst0) + (assign41960_body4_e56039 * locals.var_cnst0_dn0)) * locals.var_q_s0__blk1032) - (assign41960_body4_e56041 * locals.var_q_s0__blk1032_dn0)) / (locals.var_q_s0__blk1032 * locals.var_q_s0__blk1032)) * assign41960_body4_e56048) + (assign41960_body4_e56043 * (((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0)) - locals.var_beta_dn0))), ((((((((0.5 * locals.var_cnst0_dn2) * locals.var_cnst0) + (assign41960_body4_e56039 * locals.var_cnst0_dn2)) * locals.var_q_s0__blk1032) - (assign41960_body4_e56041 * locals.var_q_s0__blk1032_dn2)) / (locals.var_q_s0__blk1032 * locals.var_q_s0__blk1032)) * assign41960_body4_e56048) + (assign41960_body4_e56043 * (((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2)) - locals.var_beta_dn2))), ((((((((0.5 * locals.var_cnst0_dn4) * locals.var_cnst0) + (assign41960_body4_e56039 * locals.var_cnst0_dn4)) * locals.var_q_s0__blk1032) - (assign41960_body4_e56041 * locals.var_q_s0__blk1032_dn4)) / (locals.var_q_s0__blk1032 * locals.var_q_s0__blk1032)) * assign41960_body4_e56048) + (assign41960_body4_e56043 * (((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4)) - locals.var_beta_dn4))), ((((((((0.5 * locals.var_cnst0_dn5) * locals.var_cnst0) + (assign41960_body4_e56039 * locals.var_cnst0_dn5)) * locals.var_q_s0__blk1032) - (assign41960_body4_e56041 * locals.var_q_s0__blk1032_dn5)) / (locals.var_q_s0__blk1032 * locals.var_q_s0__blk1032)) * assign41960_body4_e56048) + (assign41960_body4_e56043 * (((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5)) - locals.var_beta_dn5))), ((((((((0.5 * locals.var_cnst0_dn6) * locals.var_cnst0) + (assign41960_body4_e56039 * locals.var_cnst0_dn6)) * locals.var_q_s0__blk1032) - (assign41960_body4_e56041 * locals.var_q_s0__blk1032_dn6)) / (locals.var_q_s0__blk1032 * locals.var_q_s0__blk1032)) * assign41960_body4_e56048) + (assign41960_body4_e56043 * (((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6)) - locals.var_beta_dn6))), ((((((((0.5 * locals.var_cnst0_dn7) * locals.var_cnst0) + (assign41960_body4_e56039 * locals.var_cnst0_dn7)) * locals.var_q_s0__blk1032) - (assign41960_body4_e56041 * locals.var_q_s0__blk1032_dn7)) / (locals.var_q_s0__blk1032 * locals.var_q_s0__blk1032)) * assign41960_body4_e56048) + (assign41960_body4_e56043 * (((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7)) - locals.var_beta_dn7))), ((((((((0.5 * locals.var_cnst0_dn8) * locals.var_cnst0) + (assign41960_body4_e56039 * locals.var_cnst0_dn8)) * locals.var_q_s0__blk1032) - (assign41960_body4_e56041 * locals.var_q_s0__blk1032_dn8)) / (locals.var_q_s0__blk1032 * locals.var_q_s0__blk1032)) * assign41960_body4_e56048) + (assign41960_body4_e56043 * (((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8)) - locals.var_beta_dn8))), ((((((((0.5 * locals.var_cnst0_dn9) * locals.var_cnst0) + (assign41960_body4_e56039 * locals.var_cnst0_dn9)) * locals.var_q_s0__blk1032) - (assign41960_body4_e56041 * locals.var_q_s0__blk1032_dn9)) / (locals.var_q_s0__blk1032 * locals.var_q_s0__blk1032)) * assign41960_body4_e56048) + (assign41960_body4_e56043 * (((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9)) - locals.var_beta_dn9))), ((((((((0.5 * locals.var_cnst0_dn10) * locals.var_cnst0) + (assign41960_body4_e56039 * locals.var_cnst0_dn10)) * locals.var_q_s0__blk1032) - (assign41960_body4_e56041 * locals.var_q_s0__blk1032_dn10)) / (locals.var_q_s0__blk1032 * locals.var_q_s0__blk1032)) * assign41960_body4_e56048) + (assign41960_body4_e56043 * (((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10)) - locals.var_beta_dn10))), ((((((((0.5 * locals.var_cnst0_dn11) * locals.var_cnst0) + (assign41960_body4_e56039 * locals.var_cnst0_dn11)) * locals.var_q_s0__blk1032) - (assign41960_body4_e56041 * locals.var_q_s0__blk1032_dn11)) / (locals.var_q_s0__blk1032 * locals.var_q_s0__blk1032)) * assign41960_body4_e56048) + (assign41960_body4_e56043 * (((locals.var_beta_dn11 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn11)) - locals.var_beta_dn11))), ((((((((0.5 * locals.var_cnst0_dn14) * locals.var_cnst0) + (assign41960_body4_e56039 * locals.var_cnst0_dn14)) * locals.var_q_s0__blk1032) - (assign41960_body4_e56041 * locals.var_q_s0__blk1032_dn14)) / (locals.var_q_s0__blk1032 * locals.var_q_s0__blk1032)) * assign41960_body4_e56048) + (assign41960_body4_e56043 * (((locals.var_beta_dn14 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn14)) - locals.var_beta_dn14))),)
    } else {
        (locals.var_q_s0_dps__blk1033, locals.var_q_s0_dps__blk1033_dn0, locals.var_q_s0_dps__blk1033_dn2, locals.var_q_s0_dps__blk1033_dn4, locals.var_q_s0_dps__blk1033_dn5, locals.var_q_s0_dps__blk1033_dn6, locals.var_q_s0_dps__blk1033_dn7, locals.var_q_s0_dps__blk1033_dn8, locals.var_q_s0_dps__blk1033_dn9, locals.var_q_s0_dps__blk1033_dn10, locals.var_q_s0_dps__blk1033_dn11, locals.var_q_s0_dps__blk1033_dn14,)
    }
};
            locals.var_q_s0_dps__blk1033 = assign41960_body4_e56051;
            locals.var_q_s0_dps__blk1033_dn0 = assign41960_body4_e56051_d_n0;
            locals.var_q_s0_dps__blk1033_dn2 = assign41960_body4_e56051_d_n2;
            locals.var_q_s0_dps__blk1033_dn4 = assign41960_body4_e56051_d_n4;
            locals.var_q_s0_dps__blk1033_dn5 = assign41960_body4_e56051_d_n5;
            locals.var_q_s0_dps__blk1033_dn6 = assign41960_body4_e56051_d_n6;
            locals.var_q_s0_dps__blk1033_dn7 = assign41960_body4_e56051_d_n7;
            locals.var_q_s0_dps__blk1033_dn8 = assign41960_body4_e56051_d_n8;
            locals.var_q_s0_dps__blk1033_dn9 = assign41960_body4_e56051_d_n9;
            locals.var_q_s0_dps__blk1033_dn10 = assign41960_body4_e56051_d_n10;
            locals.var_q_s0_dps__blk1033_dn11 = assign41960_body4_e56051_d_n11;
            locals.var_q_s0_dps__blk1033_dn14 = assign41960_body4_e56051_d_n14;
            locals.var_q_s0_dps__blk1033_rv = 0.0;
            let (assign41960_body5_e56072, assign41960_body5_e56072_d_n0, assign41960_body5_e56072_d_n2, assign41960_body5_e56072_d_n4, assign41960_body5_e56072_d_n5, assign41960_body5_e56072_d_n6, assign41960_body5_e56072_d_n7, assign41960_body5_e56072_d_n8, assign41960_body5_e56072_d_n9, assign41960_body5_e56072_d_n10, assign41960_body5_e56072_d_n11, assign41960_body5_e56072_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1047 == 0.0)) {
        let assign41960_body5_e56065: f64 = (-locals.var_beta);
        let assign41960_body5_e56068: f64 = (locals.var_ps0dep - locals.var_vbsc);
        let assign41960_body5_e56069: f64 = (assign41960_body5_e56065 * assign41960_body5_e56068);
        let assign41960_body5_e56070: f64 = (assign41960_body5_e56069).exp();
        (assign41960_body5_e56070, (assign41960_body5_e56070 * (((-locals.var_beta_dn0) * assign41960_body5_e56068) + (assign41960_body5_e56065 * (locals.var_ps0dep_dn0 - locals.var_vbsc_dn0)))), (assign41960_body5_e56070 * (((-locals.var_beta_dn2) * assign41960_body5_e56068) + (assign41960_body5_e56065 * (locals.var_ps0dep_dn2 - locals.var_vbsc_dn2)))), (assign41960_body5_e56070 * (((-locals.var_beta_dn4) * assign41960_body5_e56068) + (assign41960_body5_e56065 * (locals.var_ps0dep_dn4 - locals.var_vbsc_dn4)))), (assign41960_body5_e56070 * (((-locals.var_beta_dn5) * assign41960_body5_e56068) + (assign41960_body5_e56065 * (locals.var_ps0dep_dn5 - locals.var_vbsc_dn5)))), (assign41960_body5_e56070 * (((-locals.var_beta_dn6) * assign41960_body5_e56068) + (assign41960_body5_e56065 * (locals.var_ps0dep_dn6 - locals.var_vbsc_dn6)))), (assign41960_body5_e56070 * (((-locals.var_beta_dn7) * assign41960_body5_e56068) + (assign41960_body5_e56065 * (locals.var_ps0dep_dn7 - locals.var_vbsc_dn7)))), (assign41960_body5_e56070 * (((-locals.var_beta_dn8) * assign41960_body5_e56068) + (assign41960_body5_e56065 * (locals.var_ps0dep_dn8 - locals.var_vbsc_dn8)))), (assign41960_body5_e56070 * (((-locals.var_beta_dn9) * assign41960_body5_e56068) + (assign41960_body5_e56065 * (locals.var_ps0dep_dn9 - locals.var_vbsc_dn9)))), (assign41960_body5_e56070 * (((-locals.var_beta_dn10) * assign41960_body5_e56068) + (assign41960_body5_e56065 * (locals.var_ps0dep_dn10 - locals.var_vbsc_dn10)))), (assign41960_body5_e56070 * (((-locals.var_beta_dn11) * assign41960_body5_e56068) + (assign41960_body5_e56065 * (locals.var_ps0dep_dn11 - locals.var_vbsc_dn11)))), (assign41960_body5_e56070 * (((-locals.var_beta_dn14) * assign41960_body5_e56068) + (assign41960_body5_e56065 * (locals.var_ps0dep_dn14 - locals.var_vbsc_dn14)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
            locals.var_t3 = assign41960_body5_e56072;
            locals.var_t3_dn0 = assign41960_body5_e56072_d_n0;
            locals.var_t3_dn2 = assign41960_body5_e56072_d_n2;
            locals.var_t3_dn4 = assign41960_body5_e56072_d_n4;
            locals.var_t3_dn5 = assign41960_body5_e56072_d_n5;
            locals.var_t3_dn6 = assign41960_body5_e56072_d_n6;
            locals.var_t3_dn7 = assign41960_body5_e56072_d_n7;
            locals.var_t3_dn8 = assign41960_body5_e56072_d_n8;
            locals.var_t3_dn9 = assign41960_body5_e56072_d_n9;
            locals.var_t3_dn10 = assign41960_body5_e56072_d_n10;
            locals.var_t3_dn11 = assign41960_body5_e56072_d_n11;
            locals.var_t3_dn14 = assign41960_body5_e56072_d_n14;
            locals.var_t3_rv = 0.0;
            let (assign41960_body6_e56092, assign41960_body6_e56092_d_n0, assign41960_body6_e56092_d_n2, assign41960_body6_e56092_d_n4, assign41960_body6_e56092_d_n5, assign41960_body6_e56092_d_n6, assign41960_body6_e56092_d_n7, assign41960_body6_e56092_d_n8, assign41960_body6_e56092_d_n9, assign41960_body6_e56092_d_n10, assign41960_body6_e56092_d_n11, assign41960_body6_e56092_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1047 == 0.0)) {
        let assign41960_body6_e56086: f64 = (-locals.var_beta);
        let assign41960_body6_e56088: f64 = (-locals.var_vbsc);
        let assign41960_body6_e56089: f64 = (assign41960_body6_e56086 * assign41960_body6_e56088);
        let assign41960_body6_e56090: f64 = (assign41960_body6_e56089).exp();
        (assign41960_body6_e56090, (assign41960_body6_e56090 * (((-locals.var_beta_dn0) * assign41960_body6_e56088) + (assign41960_body6_e56086 * (-locals.var_vbsc_dn0)))), (assign41960_body6_e56090 * (((-locals.var_beta_dn2) * assign41960_body6_e56088) + (assign41960_body6_e56086 * (-locals.var_vbsc_dn2)))), (assign41960_body6_e56090 * (((-locals.var_beta_dn4) * assign41960_body6_e56088) + (assign41960_body6_e56086 * (-locals.var_vbsc_dn4)))), (assign41960_body6_e56090 * (((-locals.var_beta_dn5) * assign41960_body6_e56088) + (assign41960_body6_e56086 * (-locals.var_vbsc_dn5)))), (assign41960_body6_e56090 * (((-locals.var_beta_dn6) * assign41960_body6_e56088) + (assign41960_body6_e56086 * (-locals.var_vbsc_dn6)))), (assign41960_body6_e56090 * (((-locals.var_beta_dn7) * assign41960_body6_e56088) + (assign41960_body6_e56086 * (-locals.var_vbsc_dn7)))), (assign41960_body6_e56090 * (((-locals.var_beta_dn8) * assign41960_body6_e56088) + (assign41960_body6_e56086 * (-locals.var_vbsc_dn8)))), (assign41960_body6_e56090 * (((-locals.var_beta_dn9) * assign41960_body6_e56088) + (assign41960_body6_e56086 * (-locals.var_vbsc_dn9)))), (assign41960_body6_e56090 * (((-locals.var_beta_dn10) * assign41960_body6_e56088) + (assign41960_body6_e56086 * (-locals.var_vbsc_dn10)))), (assign41960_body6_e56090 * (((-locals.var_beta_dn11) * assign41960_body6_e56088) + (assign41960_body6_e56086 * (-locals.var_vbsc_dn11)))), (assign41960_body6_e56090 * (((-locals.var_beta_dn14) * assign41960_body6_e56088) + (assign41960_body6_e56086 * (-locals.var_vbsc_dn14)))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
            locals.var_t4 = assign41960_body6_e56092;
            locals.var_t4_dn0 = assign41960_body6_e56092_d_n0;
            locals.var_t4_dn2 = assign41960_body6_e56092_d_n2;
            locals.var_t4_dn4 = assign41960_body6_e56092_d_n4;
            locals.var_t4_dn5 = assign41960_body6_e56092_d_n5;
            locals.var_t4_dn6 = assign41960_body6_e56092_d_n6;
            locals.var_t4_dn7 = assign41960_body6_e56092_d_n7;
            locals.var_t4_dn8 = assign41960_body6_e56092_d_n8;
            locals.var_t4_dn9 = assign41960_body6_e56092_d_n9;
            locals.var_t4_dn10 = assign41960_body6_e56092_d_n10;
            locals.var_t4_dn11 = assign41960_body6_e56092_d_n11;
            locals.var_t4_dn14 = assign41960_body6_e56092_d_n14;
            locals.var_t4_rv = 0.0;
            let (assign41960_body7_e56122, assign41960_body7_e56122_d_n0, assign41960_body7_e56122_d_n2, assign41960_body7_e56122_d_n4, assign41960_body7_e56122_d_n5, assign41960_body7_e56122_d_n6, assign41960_body7_e56122_d_n7, assign41960_body7_e56122_d_n8, assign41960_body7_e56122_d_n9, assign41960_body7_e56122_d_n10, assign41960_body7_e56122_d_n11, assign41960_body7_e56122_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1047 == 0.0)) {
        let assign41960_body7_e56108: f64 = (locals.var_t2 - 1.0);
        let assign41960_body7_e56110: f64 = (assign41960_body7_e56108 - locals.var_t1);
        let assign41960_body7_e56114: f64 = (locals.var_t3 - locals.var_t4);
        let assign41960_body7_e56115: f64 = (locals.var_cnst1 * assign41960_body7_e56114);
        let assign41960_body7_e56116: f64 = (assign41960_body7_e56110 + assign41960_body7_e56115);
        let assign41960_body7_e56118: f64 = (assign41960_body7_e56116 + 1e-15);
        let assign41960_body7_e56119: f64 = (assign41960_body7_e56118).sqrt();
        let assign41960_body7_e56120: f64 = (locals.var_cnst0 * assign41960_body7_e56119);
        (assign41960_body7_e56120, ((locals.var_cnst0_dn0 * assign41960_body7_e56119) + (locals.var_cnst0 * (((locals.var_t2_dn0 - locals.var_t1_dn0) + ((locals.var_cnst1_dn0 * assign41960_body7_e56114) + (locals.var_cnst1 * (locals.var_t3_dn0 - locals.var_t4_dn0)))) / (2.0 * assign41960_body7_e56119)))), ((locals.var_cnst0_dn2 * assign41960_body7_e56119) + (locals.var_cnst0 * (((locals.var_t2_dn2 - locals.var_t1_dn2) + ((locals.var_cnst1_dn2 * assign41960_body7_e56114) + (locals.var_cnst1 * (locals.var_t3_dn2 - locals.var_t4_dn2)))) / (2.0 * assign41960_body7_e56119)))), ((locals.var_cnst0_dn4 * assign41960_body7_e56119) + (locals.var_cnst0 * (((locals.var_t2_dn4 - locals.var_t1_dn4) + ((locals.var_cnst1_dn4 * assign41960_body7_e56114) + (locals.var_cnst1 * (locals.var_t3_dn4 - locals.var_t4_dn4)))) / (2.0 * assign41960_body7_e56119)))), ((locals.var_cnst0_dn5 * assign41960_body7_e56119) + (locals.var_cnst0 * (((locals.var_t2_dn5 - locals.var_t1_dn5) + ((locals.var_cnst1_dn5 * assign41960_body7_e56114) + (locals.var_cnst1 * (locals.var_t3_dn5 - locals.var_t4_dn5)))) / (2.0 * assign41960_body7_e56119)))), ((locals.var_cnst0_dn6 * assign41960_body7_e56119) + (locals.var_cnst0 * (((locals.var_t2_dn6 - locals.var_t1_dn6) + ((locals.var_cnst1_dn6 * assign41960_body7_e56114) + (locals.var_cnst1 * (locals.var_t3_dn6 - locals.var_t4_dn6)))) / (2.0 * assign41960_body7_e56119)))), ((locals.var_cnst0_dn7 * assign41960_body7_e56119) + (locals.var_cnst0 * (((locals.var_t2_dn7 - locals.var_t1_dn7) + ((locals.var_cnst1_dn7 * assign41960_body7_e56114) + (locals.var_cnst1 * (locals.var_t3_dn7 - locals.var_t4_dn7)))) / (2.0 * assign41960_body7_e56119)))), ((locals.var_cnst0_dn8 * assign41960_body7_e56119) + (locals.var_cnst0 * (((locals.var_t2_dn8 - locals.var_t1_dn8) + ((locals.var_cnst1_dn8 * assign41960_body7_e56114) + (locals.var_cnst1 * (locals.var_t3_dn8 - locals.var_t4_dn8)))) / (2.0 * assign41960_body7_e56119)))), ((locals.var_cnst0_dn9 * assign41960_body7_e56119) + (locals.var_cnst0 * (((locals.var_t2_dn9 - locals.var_t1_dn9) + ((locals.var_cnst1_dn9 * assign41960_body7_e56114) + (locals.var_cnst1 * (locals.var_t3_dn9 - locals.var_t4_dn9)))) / (2.0 * assign41960_body7_e56119)))), ((locals.var_cnst0_dn10 * assign41960_body7_e56119) + (locals.var_cnst0 * (((locals.var_t2_dn10 - locals.var_t1_dn10) + ((locals.var_cnst1_dn10 * assign41960_body7_e56114) + (locals.var_cnst1 * (locals.var_t3_dn10 - locals.var_t4_dn10)))) / (2.0 * assign41960_body7_e56119)))), ((locals.var_cnst0_dn11 * assign41960_body7_e56119) + (locals.var_cnst0 * (((locals.var_t2_dn11 - locals.var_t1_dn11) + ((locals.var_cnst1_dn11 * assign41960_body7_e56114) + (locals.var_cnst1 * (locals.var_t3_dn11 - locals.var_t4_dn11)))) / (2.0 * assign41960_body7_e56119)))), ((locals.var_cnst0_dn14 * assign41960_body7_e56119) + (locals.var_cnst0 * (((locals.var_t2_dn14 - locals.var_t1_dn14) + ((locals.var_cnst1_dn14 * assign41960_body7_e56114) + (locals.var_cnst1 * (locals.var_t3_dn14 - locals.var_t4_dn14)))) / (2.0 * assign41960_body7_e56119)))),)
    } else {
        (locals.var_q_s0__blk1032, locals.var_q_s0__blk1032_dn0, locals.var_q_s0__blk1032_dn2, locals.var_q_s0__blk1032_dn4, locals.var_q_s0__blk1032_dn5, locals.var_q_s0__blk1032_dn6, locals.var_q_s0__blk1032_dn7, locals.var_q_s0__blk1032_dn8, locals.var_q_s0__blk1032_dn9, locals.var_q_s0__blk1032_dn10, locals.var_q_s0__blk1032_dn11, locals.var_q_s0__blk1032_dn14,)
    }
};
            locals.var_q_s0__blk1032 = assign41960_body7_e56122;
            locals.var_q_s0__blk1032_dn0 = assign41960_body7_e56122_d_n0;
            locals.var_q_s0__blk1032_dn2 = assign41960_body7_e56122_d_n2;
            locals.var_q_s0__blk1032_dn4 = assign41960_body7_e56122_d_n4;
            locals.var_q_s0__blk1032_dn5 = assign41960_body7_e56122_d_n5;
            locals.var_q_s0__blk1032_dn6 = assign41960_body7_e56122_d_n6;
            locals.var_q_s0__blk1032_dn7 = assign41960_body7_e56122_d_n7;
            locals.var_q_s0__blk1032_dn8 = assign41960_body7_e56122_d_n8;
            locals.var_q_s0__blk1032_dn9 = assign41960_body7_e56122_d_n9;
            locals.var_q_s0__blk1032_dn10 = assign41960_body7_e56122_d_n10;
            locals.var_q_s0__blk1032_dn11 = assign41960_body7_e56122_d_n11;
            locals.var_q_s0__blk1032_dn14 = assign41960_body7_e56122_d_n14;
            locals.var_q_s0__blk1032_rv = 0.0;
            let (assign41960_body8_e56143, assign41960_body8_e56143_d_n0, assign41960_body8_e56143_d_n2, assign41960_body8_e56143_d_n4, assign41960_body8_e56143_d_n5, assign41960_body8_e56143_d_n6, assign41960_body8_e56143_d_n7, assign41960_body8_e56143_d_n8, assign41960_body8_e56143_d_n9, assign41960_body8_e56143_d_n10, assign41960_body8_e56143_d_n11, assign41960_body8_e56143_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1047 == 0.0)) {
        let assign41960_body8_e56137: f64 = (0.5 * locals.var_cnst0);
        let assign41960_body8_e56139: f64 = (assign41960_body8_e56137 * locals.var_cnst0);
        let assign41960_body8_e56141: f64 = (assign41960_body8_e56139 / locals.var_q_s0__blk1032);
        (assign41960_body8_e56141, ((((((0.5 * locals.var_cnst0_dn0) * locals.var_cnst0) + (assign41960_body8_e56137 * locals.var_cnst0_dn0)) * locals.var_q_s0__blk1032) - (assign41960_body8_e56139 * locals.var_q_s0__blk1032_dn0)) / (locals.var_q_s0__blk1032 * locals.var_q_s0__blk1032)), ((((((0.5 * locals.var_cnst0_dn2) * locals.var_cnst0) + (assign41960_body8_e56137 * locals.var_cnst0_dn2)) * locals.var_q_s0__blk1032) - (assign41960_body8_e56139 * locals.var_q_s0__blk1032_dn2)) / (locals.var_q_s0__blk1032 * locals.var_q_s0__blk1032)), ((((((0.5 * locals.var_cnst0_dn4) * locals.var_cnst0) + (assign41960_body8_e56137 * locals.var_cnst0_dn4)) * locals.var_q_s0__blk1032) - (assign41960_body8_e56139 * locals.var_q_s0__blk1032_dn4)) / (locals.var_q_s0__blk1032 * locals.var_q_s0__blk1032)), ((((((0.5 * locals.var_cnst0_dn5) * locals.var_cnst0) + (assign41960_body8_e56137 * locals.var_cnst0_dn5)) * locals.var_q_s0__blk1032) - (assign41960_body8_e56139 * locals.var_q_s0__blk1032_dn5)) / (locals.var_q_s0__blk1032 * locals.var_q_s0__blk1032)), ((((((0.5 * locals.var_cnst0_dn6) * locals.var_cnst0) + (assign41960_body8_e56137 * locals.var_cnst0_dn6)) * locals.var_q_s0__blk1032) - (assign41960_body8_e56139 * locals.var_q_s0__blk1032_dn6)) / (locals.var_q_s0__blk1032 * locals.var_q_s0__blk1032)), ((((((0.5 * locals.var_cnst0_dn7) * locals.var_cnst0) + (assign41960_body8_e56137 * locals.var_cnst0_dn7)) * locals.var_q_s0__blk1032) - (assign41960_body8_e56139 * locals.var_q_s0__blk1032_dn7)) / (locals.var_q_s0__blk1032 * locals.var_q_s0__blk1032)), ((((((0.5 * locals.var_cnst0_dn8) * locals.var_cnst0) + (assign41960_body8_e56137 * locals.var_cnst0_dn8)) * locals.var_q_s0__blk1032) - (assign41960_body8_e56139 * locals.var_q_s0__blk1032_dn8)) / (locals.var_q_s0__blk1032 * locals.var_q_s0__blk1032)), ((((((0.5 * locals.var_cnst0_dn9) * locals.var_cnst0) + (assign41960_body8_e56137 * locals.var_cnst0_dn9)) * locals.var_q_s0__blk1032) - (assign41960_body8_e56139 * locals.var_q_s0__blk1032_dn9)) / (locals.var_q_s0__blk1032 * locals.var_q_s0__blk1032)), ((((((0.5 * locals.var_cnst0_dn10) * locals.var_cnst0) + (assign41960_body8_e56137 * locals.var_cnst0_dn10)) * locals.var_q_s0__blk1032) - (assign41960_body8_e56139 * locals.var_q_s0__blk1032_dn10)) / (locals.var_q_s0__blk1032 * locals.var_q_s0__blk1032)), ((((((0.5 * locals.var_cnst0_dn11) * locals.var_cnst0) + (assign41960_body8_e56137 * locals.var_cnst0_dn11)) * locals.var_q_s0__blk1032) - (assign41960_body8_e56139 * locals.var_q_s0__blk1032_dn11)) / (locals.var_q_s0__blk1032 * locals.var_q_s0__blk1032)), ((((((0.5 * locals.var_cnst0_dn14) * locals.var_cnst0) + (assign41960_body8_e56137 * locals.var_cnst0_dn14)) * locals.var_q_s0__blk1032) - (assign41960_body8_e56139 * locals.var_q_s0__blk1032_dn14)) / (locals.var_q_s0__blk1032 * locals.var_q_s0__blk1032)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
            locals.var_t5 = assign41960_body8_e56143;
            locals.var_t5_dn0 = assign41960_body8_e56143_d_n0;
            locals.var_t5_dn2 = assign41960_body8_e56143_d_n2;
            locals.var_t5_dn4 = assign41960_body8_e56143_d_n4;
            locals.var_t5_dn5 = assign41960_body8_e56143_d_n5;
            locals.var_t5_dn6 = assign41960_body8_e56143_d_n6;
            locals.var_t5_dn7 = assign41960_body8_e56143_d_n7;
            locals.var_t5_dn8 = assign41960_body8_e56143_d_n8;
            locals.var_t5_dn9 = assign41960_body8_e56143_d_n9;
            locals.var_t5_dn10 = assign41960_body8_e56143_d_n10;
            locals.var_t5_dn11 = assign41960_body8_e56143_d_n11;
            locals.var_t5_dn14 = assign41960_body8_e56143_d_n14;
            locals.var_t5_rv = 0.0;
            let (assign41960_body9_e56171, assign41960_body9_e56171_d_n0, assign41960_body9_e56171_d_n2, assign41960_body9_e56171_d_n4, assign41960_body9_e56171_d_n5, assign41960_body9_e56171_d_n6, assign41960_body9_e56171_d_n7, assign41960_body9_e56171_d_n8, assign41960_body9_e56171_d_n9, assign41960_body9_e56171_d_n10, assign41960_body9_e56171_d_n11, assign41960_body9_e56171_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1047 == 0.0)) {
        let assign41960_body9_e56159: f64 = (locals.var_beta * locals.var_t2);
        let assign41960_body9_e56161: f64 = (assign41960_body9_e56159 - locals.var_beta);
        let assign41960_body9_e56164: f64 = (-locals.var_beta);
        let assign41960_body9_e56166: f64 = (assign41960_body9_e56164 * locals.var_t3);
        let assign41960_body9_e56167: f64 = (locals.var_cnst1 * assign41960_body9_e56166);
        let assign41960_body9_e56168: f64 = (assign41960_body9_e56161 + assign41960_body9_e56167);
        let assign41960_body9_e56169: f64 = (locals.var_t5 * assign41960_body9_e56168);
        (assign41960_body9_e56169, ((locals.var_t5_dn0 * assign41960_body9_e56168) + (locals.var_t5 * ((((locals.var_beta_dn0 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn0)) - locals.var_beta_dn0) + ((locals.var_cnst1_dn0 * assign41960_body9_e56166) + (locals.var_cnst1 * (((-locals.var_beta_dn0) * locals.var_t3) + (assign41960_body9_e56164 * locals.var_t3_dn0))))))), ((locals.var_t5_dn2 * assign41960_body9_e56168) + (locals.var_t5 * ((((locals.var_beta_dn2 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn2)) - locals.var_beta_dn2) + ((locals.var_cnst1_dn2 * assign41960_body9_e56166) + (locals.var_cnst1 * (((-locals.var_beta_dn2) * locals.var_t3) + (assign41960_body9_e56164 * locals.var_t3_dn2))))))), ((locals.var_t5_dn4 * assign41960_body9_e56168) + (locals.var_t5 * ((((locals.var_beta_dn4 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn4)) - locals.var_beta_dn4) + ((locals.var_cnst1_dn4 * assign41960_body9_e56166) + (locals.var_cnst1 * (((-locals.var_beta_dn4) * locals.var_t3) + (assign41960_body9_e56164 * locals.var_t3_dn4))))))), ((locals.var_t5_dn5 * assign41960_body9_e56168) + (locals.var_t5 * ((((locals.var_beta_dn5 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn5)) - locals.var_beta_dn5) + ((locals.var_cnst1_dn5 * assign41960_body9_e56166) + (locals.var_cnst1 * (((-locals.var_beta_dn5) * locals.var_t3) + (assign41960_body9_e56164 * locals.var_t3_dn5))))))), ((locals.var_t5_dn6 * assign41960_body9_e56168) + (locals.var_t5 * ((((locals.var_beta_dn6 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn6)) - locals.var_beta_dn6) + ((locals.var_cnst1_dn6 * assign41960_body9_e56166) + (locals.var_cnst1 * (((-locals.var_beta_dn6) * locals.var_t3) + (assign41960_body9_e56164 * locals.var_t3_dn6))))))), ((locals.var_t5_dn7 * assign41960_body9_e56168) + (locals.var_t5 * ((((locals.var_beta_dn7 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn7)) - locals.var_beta_dn7) + ((locals.var_cnst1_dn7 * assign41960_body9_e56166) + (locals.var_cnst1 * (((-locals.var_beta_dn7) * locals.var_t3) + (assign41960_body9_e56164 * locals.var_t3_dn7))))))), ((locals.var_t5_dn8 * assign41960_body9_e56168) + (locals.var_t5 * ((((locals.var_beta_dn8 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn8)) - locals.var_beta_dn8) + ((locals.var_cnst1_dn8 * assign41960_body9_e56166) + (locals.var_cnst1 * (((-locals.var_beta_dn8) * locals.var_t3) + (assign41960_body9_e56164 * locals.var_t3_dn8))))))), ((locals.var_t5_dn9 * assign41960_body9_e56168) + (locals.var_t5 * ((((locals.var_beta_dn9 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn9)) - locals.var_beta_dn9) + ((locals.var_cnst1_dn9 * assign41960_body9_e56166) + (locals.var_cnst1 * (((-locals.var_beta_dn9) * locals.var_t3) + (assign41960_body9_e56164 * locals.var_t3_dn9))))))), ((locals.var_t5_dn10 * assign41960_body9_e56168) + (locals.var_t5 * ((((locals.var_beta_dn10 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn10)) - locals.var_beta_dn10) + ((locals.var_cnst1_dn10 * assign41960_body9_e56166) + (locals.var_cnst1 * (((-locals.var_beta_dn10) * locals.var_t3) + (assign41960_body9_e56164 * locals.var_t3_dn10))))))), ((locals.var_t5_dn11 * assign41960_body9_e56168) + (locals.var_t5 * ((((locals.var_beta_dn11 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn11)) - locals.var_beta_dn11) + ((locals.var_cnst1_dn11 * assign41960_body9_e56166) + (locals.var_cnst1 * (((-locals.var_beta_dn11) * locals.var_t3) + (assign41960_body9_e56164 * locals.var_t3_dn11))))))), ((locals.var_t5_dn14 * assign41960_body9_e56168) + (locals.var_t5 * ((((locals.var_beta_dn14 * locals.var_t2) + (locals.var_beta * locals.var_t2_dn14)) - locals.var_beta_dn14) + ((locals.var_cnst1_dn14 * assign41960_body9_e56166) + (locals.var_cnst1 * (((-locals.var_beta_dn14) * locals.var_t3) + (assign41960_body9_e56164 * locals.var_t3_dn14))))))),)
    } else {
        (locals.var_q_s0_dps__blk1033, locals.var_q_s0_dps__blk1033_dn0, locals.var_q_s0_dps__blk1033_dn2, locals.var_q_s0_dps__blk1033_dn4, locals.var_q_s0_dps__blk1033_dn5, locals.var_q_s0_dps__blk1033_dn6, locals.var_q_s0_dps__blk1033_dn7, locals.var_q_s0_dps__blk1033_dn8, locals.var_q_s0_dps__blk1033_dn9, locals.var_q_s0_dps__blk1033_dn10, locals.var_q_s0_dps__blk1033_dn11, locals.var_q_s0_dps__blk1033_dn14,)
    }
};
            locals.var_q_s0_dps__blk1033 = assign41960_body9_e56171;
            locals.var_q_s0_dps__blk1033_dn0 = assign41960_body9_e56171_d_n0;
            locals.var_q_s0_dps__blk1033_dn2 = assign41960_body9_e56171_d_n2;
            locals.var_q_s0_dps__blk1033_dn4 = assign41960_body9_e56171_d_n4;
            locals.var_q_s0_dps__blk1033_dn5 = assign41960_body9_e56171_d_n5;
            locals.var_q_s0_dps__blk1033_dn6 = assign41960_body9_e56171_d_n6;
            locals.var_q_s0_dps__blk1033_dn7 = assign41960_body9_e56171_d_n7;
            locals.var_q_s0_dps__blk1033_dn8 = assign41960_body9_e56171_d_n8;
            locals.var_q_s0_dps__blk1033_dn9 = assign41960_body9_e56171_d_n9;
            locals.var_q_s0_dps__blk1033_dn10 = assign41960_body9_e56171_d_n10;
            locals.var_q_s0_dps__blk1033_dn11 = assign41960_body9_e56171_d_n11;
            locals.var_q_s0_dps__blk1033_dn14 = assign41960_body9_e56171_d_n14;
            locals.var_q_s0_dps__blk1033_rv = 0.0;
            let (assign41960_body10_e56187,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_flg_conv != 0.0)) {
        let assign41960_body10_e56185: f64 = (150.0 + 1.0);
        (assign41960_body10_e56185,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign41960_body10_e56187;
            locals.var_lp_s0_rv = 0.0;
            let (assign41960_body11_e56208, assign41960_body11_e56208_d_n0, assign41960_body11_e56208_d_n2, assign41960_body11_e56208_d_n4, assign41960_body11_e56208_d_n5, assign41960_body11_e56208_d_n6, assign41960_body11_e56208_d_n7, assign41960_body11_e56208_d_n8, assign41960_body11_e56208_d_n9, assign41960_body11_e56208_d_n10, assign41960_body11_e56208_d_n11, assign41960_body11_e56208_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign41960_body11_e56203: f64 = (locals.var_vgp_res - locals.var_ps0dep);
        let assign41960_body11_e56204: f64 = (locals.var_cox * assign41960_body11_e56203);
        let assign41960_body11_e56206: f64 = (assign41960_body11_e56204 + locals.var_q_s0__blk1032);
        (assign41960_body11_e56206, (((locals.var_cox_dn0 * assign41960_body11_e56203) + (locals.var_cox * (locals.var_vgp_res_dn0 - locals.var_ps0dep_dn0))) + locals.var_q_s0__blk1032_dn0), (((locals.var_cox_dn2 * assign41960_body11_e56203) + (locals.var_cox * (locals.var_vgp_res_dn2 - locals.var_ps0dep_dn2))) + locals.var_q_s0__blk1032_dn2), (((locals.var_cox_dn4 * assign41960_body11_e56203) + (locals.var_cox * (locals.var_vgp_res_dn4 - locals.var_ps0dep_dn4))) + locals.var_q_s0__blk1032_dn4), (((locals.var_cox_dn5 * assign41960_body11_e56203) + (locals.var_cox * (locals.var_vgp_res_dn5 - locals.var_ps0dep_dn5))) + locals.var_q_s0__blk1032_dn5), (((locals.var_cox_dn6 * assign41960_body11_e56203) + (locals.var_cox * (locals.var_vgp_res_dn6 - locals.var_ps0dep_dn6))) + locals.var_q_s0__blk1032_dn6), (((locals.var_cox_dn7 * assign41960_body11_e56203) + (locals.var_cox * (locals.var_vgp_res_dn7 - locals.var_ps0dep_dn7))) + locals.var_q_s0__blk1032_dn7), (((locals.var_cox_dn8 * assign41960_body11_e56203) + (locals.var_cox * (locals.var_vgp_res_dn8 - locals.var_ps0dep_dn8))) + locals.var_q_s0__blk1032_dn8), (((locals.var_cox_dn9 * assign41960_body11_e56203) + (locals.var_cox * (locals.var_vgp_res_dn9 - locals.var_ps0dep_dn9))) + locals.var_q_s0__blk1032_dn9), (((locals.var_cox_dn10 * assign41960_body11_e56203) + (locals.var_cox * (locals.var_vgp_res_dn10 - locals.var_ps0dep_dn10))) + locals.var_q_s0__blk1032_dn10), (((locals.var_cox_dn11 * assign41960_body11_e56203) + (locals.var_cox * (locals.var_vgp_res_dn11 - locals.var_ps0dep_dn11))) + locals.var_q_s0__blk1032_dn11), (((locals.var_cox_dn14 * assign41960_body11_e56203) + (locals.var_cox * (locals.var_vgp_res_dn14 - locals.var_ps0dep_dn14))) + locals.var_q_s0__blk1032_dn14),)
    } else {
        (locals.var_pf1, locals.var_pf1_dn0, locals.var_pf1_dn2, locals.var_pf1_dn4, locals.var_pf1_dn5, locals.var_pf1_dn6, locals.var_pf1_dn7, locals.var_pf1_dn8, locals.var_pf1_dn9, locals.var_pf1_dn10, locals.var_pf1_dn11, locals.var_pf1_dn14,)
    }
};
            locals.var_pf1 = assign41960_body11_e56208;
            locals.var_pf1_dn0 = assign41960_body11_e56208_d_n0;
            locals.var_pf1_dn2 = assign41960_body11_e56208_d_n2;
            locals.var_pf1_dn4 = assign41960_body11_e56208_d_n4;
            locals.var_pf1_dn5 = assign41960_body11_e56208_d_n5;
            locals.var_pf1_dn6 = assign41960_body11_e56208_d_n6;
            locals.var_pf1_dn7 = assign41960_body11_e56208_d_n7;
            locals.var_pf1_dn8 = assign41960_body11_e56208_d_n8;
            locals.var_pf1_dn9 = assign41960_body11_e56208_d_n9;
            locals.var_pf1_dn10 = assign41960_body11_e56208_d_n10;
            locals.var_pf1_dn11 = assign41960_body11_e56208_d_n11;
            locals.var_pf1_dn14 = assign41960_body11_e56208_d_n14;
            locals.var_pf1_rv = 0.0;
            let (assign41960_body12_e56226, assign41960_body12_e56226_d_n0, assign41960_body12_e56226_d_n2, assign41960_body12_e56226_d_n4, assign41960_body12_e56226_d_n5, assign41960_body12_e56226_d_n6, assign41960_body12_e56226_d_n7, assign41960_body12_e56226_d_n8, assign41960_body12_e56226_d_n9, assign41960_body12_e56226_d_n10, assign41960_body12_e56226_d_n11, assign41960_body12_e56226_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign41960_body12_e56222: f64 = (-locals.var_cox);
        let assign41960_body12_e56224: f64 = (assign41960_body12_e56222 + locals.var_q_s0_dps__blk1033);
        (assign41960_body12_e56224, ((-locals.var_cox_dn0) + locals.var_q_s0_dps__blk1033_dn0), ((-locals.var_cox_dn2) + locals.var_q_s0_dps__blk1033_dn2), ((-locals.var_cox_dn4) + locals.var_q_s0_dps__blk1033_dn4), ((-locals.var_cox_dn5) + locals.var_q_s0_dps__blk1033_dn5), ((-locals.var_cox_dn6) + locals.var_q_s0_dps__blk1033_dn6), ((-locals.var_cox_dn7) + locals.var_q_s0_dps__blk1033_dn7), ((-locals.var_cox_dn8) + locals.var_q_s0_dps__blk1033_dn8), ((-locals.var_cox_dn9) + locals.var_q_s0_dps__blk1033_dn9), ((-locals.var_cox_dn10) + locals.var_q_s0_dps__blk1033_dn10), ((-locals.var_cox_dn11) + locals.var_q_s0_dps__blk1033_dn11), ((-locals.var_cox_dn14) + locals.var_q_s0_dps__blk1033_dn14),)
    } else {
        (locals.var_pf11, locals.var_pf11_dn0, locals.var_pf11_dn2, locals.var_pf11_dn4, locals.var_pf11_dn5, locals.var_pf11_dn6, locals.var_pf11_dn7, locals.var_pf11_dn8, locals.var_pf11_dn9, locals.var_pf11_dn10, locals.var_pf11_dn11, locals.var_pf11_dn14,)
    }
};
            locals.var_pf11 = assign41960_body12_e56226;
            locals.var_pf11_dn0 = assign41960_body12_e56226_d_n0;
            locals.var_pf11_dn2 = assign41960_body12_e56226_d_n2;
            locals.var_pf11_dn4 = assign41960_body12_e56226_d_n4;
            locals.var_pf11_dn5 = assign41960_body12_e56226_d_n5;
            locals.var_pf11_dn6 = assign41960_body12_e56226_d_n6;
            locals.var_pf11_dn7 = assign41960_body12_e56226_d_n7;
            locals.var_pf11_dn8 = assign41960_body12_e56226_d_n8;
            locals.var_pf11_dn9 = assign41960_body12_e56226_d_n9;
            locals.var_pf11_dn10 = assign41960_body12_e56226_d_n10;
            locals.var_pf11_dn11 = assign41960_body12_e56226_d_n11;
            locals.var_pf11_dn14 = assign41960_body12_e56226_d_n14;
            locals.var_pf11_rv = 0.0;
            let (assign41960_body13_e56244, assign41960_body13_e56244_d_n0, assign41960_body13_e56244_d_n2, assign41960_body13_e56244_d_n4, assign41960_body13_e56244_d_n5, assign41960_body13_e56244_d_n6, assign41960_body13_e56244_d_n7, assign41960_body13_e56244_d_n8, assign41960_body13_e56244_d_n9, assign41960_body13_e56244_d_n10, assign41960_body13_e56244_d_n11, assign41960_body13_e56244_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign41960_body13_e56240: f64 = (-locals.var_pf1);
        let assign41960_body13_e56242: f64 = (assign41960_body13_e56240 / locals.var_pf11);
        (assign41960_body13_e56242, ((((-locals.var_pf1_dn0) * locals.var_pf11) - (assign41960_body13_e56240 * locals.var_pf11_dn0)) / (locals.var_pf11 * locals.var_pf11)), ((((-locals.var_pf1_dn2) * locals.var_pf11) - (assign41960_body13_e56240 * locals.var_pf11_dn2)) / (locals.var_pf11 * locals.var_pf11)), ((((-locals.var_pf1_dn4) * locals.var_pf11) - (assign41960_body13_e56240 * locals.var_pf11_dn4)) / (locals.var_pf11 * locals.var_pf11)), ((((-locals.var_pf1_dn5) * locals.var_pf11) - (assign41960_body13_e56240 * locals.var_pf11_dn5)) / (locals.var_pf11 * locals.var_pf11)), ((((-locals.var_pf1_dn6) * locals.var_pf11) - (assign41960_body13_e56240 * locals.var_pf11_dn6)) / (locals.var_pf11 * locals.var_pf11)), ((((-locals.var_pf1_dn7) * locals.var_pf11) - (assign41960_body13_e56240 * locals.var_pf11_dn7)) / (locals.var_pf11 * locals.var_pf11)), ((((-locals.var_pf1_dn8) * locals.var_pf11) - (assign41960_body13_e56240 * locals.var_pf11_dn8)) / (locals.var_pf11 * locals.var_pf11)), ((((-locals.var_pf1_dn9) * locals.var_pf11) - (assign41960_body13_e56240 * locals.var_pf11_dn9)) / (locals.var_pf11 * locals.var_pf11)), ((((-locals.var_pf1_dn10) * locals.var_pf11) - (assign41960_body13_e56240 * locals.var_pf11_dn10)) / (locals.var_pf11 * locals.var_pf11)), ((((-locals.var_pf1_dn11) * locals.var_pf11) - (assign41960_body13_e56240 * locals.var_pf11_dn11)) / (locals.var_pf11 * locals.var_pf11)), ((((-locals.var_pf1_dn14) * locals.var_pf11) - (assign41960_body13_e56240 * locals.var_pf11_dn14)) / (locals.var_pf11 * locals.var_pf11)),)
    } else {
        (locals.var_dps, locals.var_dps_dn0, locals.var_dps_dn2, locals.var_dps_dn4, locals.var_dps_dn5, locals.var_dps_dn6, locals.var_dps_dn7, locals.var_dps_dn8, locals.var_dps_dn9, locals.var_dps_dn10, locals.var_dps_dn11, locals.var_dps_dn14,)
    }
};
            locals.var_dps = assign41960_body13_e56244;
            locals.var_dps_dn0 = assign41960_body13_e56244_d_n0;
            locals.var_dps_dn2 = assign41960_body13_e56244_d_n2;
            locals.var_dps_dn4 = assign41960_body13_e56244_d_n4;
            locals.var_dps_dn5 = assign41960_body13_e56244_d_n5;
            locals.var_dps_dn6 = assign41960_body13_e56244_d_n6;
            locals.var_dps_dn7 = assign41960_body13_e56244_d_n7;
            locals.var_dps_dn8 = assign41960_body13_e56244_d_n8;
            locals.var_dps_dn9 = assign41960_body13_e56244_d_n9;
            locals.var_dps_dn10 = assign41960_body13_e56244_d_n10;
            locals.var_dps_dn11 = assign41960_body13_e56244_d_n11;
            locals.var_dps_dn14 = assign41960_body13_e56244_d_n14;
            locals.var_dps_rv = 0.0;
            let assign41960_body14_e56246: f64 = (locals.var_dps).abs();
            let assign41960_body14_e56249: f64 = (1e-10 * 100.0);
            let assign41960_body14_e56250: f64 = if assign41960_body14_e56246 < assign41960_body14_e56249 { 1.0 } else { 0.0 };
            locals.var_guard1048 = assign41960_body14_e56250;
            locals.var_guard1048_rv = 0.0;
            let (assign41960_body15_e56267,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_flg_conv == 0.0)) && (locals.var_guard1048 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_conv,)
    }
};
            locals.var_flg_conv = assign41960_body15_e56267;
            locals.var_flg_conv_rv = 0.0;
            let assign41960_body16_e56270: f64 = if locals.var_dps > 0.1 { 1.0 } else { 0.0 };
            locals.var_guard1049 = assign41960_body16_e56270;
            locals.var_guard1049_rv = 0.0;
            let (assign41960_body17_e56290, assign41960_body17_e56290_d_n0, assign41960_body17_e56290_d_n2, assign41960_body17_e56290_d_n4, assign41960_body17_e56290_d_n5, assign41960_body17_e56290_d_n6, assign41960_body17_e56290_d_n7, assign41960_body17_e56290_d_n8, assign41960_body17_e56290_d_n9, assign41960_body17_e56290_d_n10, assign41960_body17_e56290_d_n11, assign41960_body17_e56290_d_n14,) = {
    if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_flg_conv == 0.0)) && (locals.var_guard1048 == 0.0)) && (locals.var_guard1049 != 0.0)) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dps, locals.var_dps_dn0, locals.var_dps_dn2, locals.var_dps_dn4, locals.var_dps_dn5, locals.var_dps_dn6, locals.var_dps_dn7, locals.var_dps_dn8, locals.var_dps_dn9, locals.var_dps_dn10, locals.var_dps_dn11, locals.var_dps_dn14,)
    }
};
            locals.var_dps = assign41960_body17_e56290;
            locals.var_dps_dn0 = assign41960_body17_e56290_d_n0;
            locals.var_dps_dn2 = assign41960_body17_e56290_d_n2;
            locals.var_dps_dn4 = assign41960_body17_e56290_d_n4;
            locals.var_dps_dn5 = assign41960_body17_e56290_d_n5;
            locals.var_dps_dn6 = assign41960_body17_e56290_d_n6;
            locals.var_dps_dn7 = assign41960_body17_e56290_d_n7;
            locals.var_dps_dn8 = assign41960_body17_e56290_d_n8;
            locals.var_dps_dn9 = assign41960_body17_e56290_d_n9;
            locals.var_dps_dn10 = assign41960_body17_e56290_d_n10;
            locals.var_dps_dn11 = assign41960_body17_e56290_d_n11;
            locals.var_dps_dn14 = assign41960_body17_e56290_d_n14;
            locals.var_dps_rv = 0.0;
            let assign41960_body18_e56293: f64 = (-0.1);
            let assign41960_body18_e56294: f64 = if locals.var_dps < assign41960_body18_e56293 { 1.0 } else { 0.0 };
            locals.var_guard1050 = assign41960_body18_e56294;
            locals.var_guard1050_rv = 0.0;
            let (assign41960_body19_e56318, assign41960_body19_e56318_d_n0, assign41960_body19_e56318_d_n2, assign41960_body19_e56318_d_n4, assign41960_body19_e56318_d_n5, assign41960_body19_e56318_d_n6, assign41960_body19_e56318_d_n7, assign41960_body19_e56318_d_n8, assign41960_body19_e56318_d_n9, assign41960_body19_e56318_d_n10, assign41960_body19_e56318_d_n11, assign41960_body19_e56318_d_n14,) = {
    if (((((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_flg_conv == 0.0)) && (locals.var_guard1048 == 0.0)) && (locals.var_guard1049 == 0.0)) && (locals.var_guard1050 != 0.0)) {
        let assign41960_body19_e56316: f64 = (-0.1);
        (assign41960_body19_e56316, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dps, locals.var_dps_dn0, locals.var_dps_dn2, locals.var_dps_dn4, locals.var_dps_dn5, locals.var_dps_dn6, locals.var_dps_dn7, locals.var_dps_dn8, locals.var_dps_dn9, locals.var_dps_dn10, locals.var_dps_dn11, locals.var_dps_dn14,)
    }
};
            locals.var_dps = assign41960_body19_e56318;
            locals.var_dps_dn0 = assign41960_body19_e56318_d_n0;
            locals.var_dps_dn2 = assign41960_body19_e56318_d_n2;
            locals.var_dps_dn4 = assign41960_body19_e56318_d_n4;
            locals.var_dps_dn5 = assign41960_body19_e56318_d_n5;
            locals.var_dps_dn6 = assign41960_body19_e56318_d_n6;
            locals.var_dps_dn7 = assign41960_body19_e56318_d_n7;
            locals.var_dps_dn8 = assign41960_body19_e56318_d_n8;
            locals.var_dps_dn9 = assign41960_body19_e56318_d_n9;
            locals.var_dps_dn10 = assign41960_body19_e56318_d_n10;
            locals.var_dps_dn11 = assign41960_body19_e56318_d_n11;
            locals.var_dps_dn14 = assign41960_body19_e56318_d_n14;
            locals.var_dps_rv = 0.0;
            let (assign41960_body20_e56335, assign41960_body20_e56335_d_n0, assign41960_body20_e56335_d_n2, assign41960_body20_e56335_d_n4, assign41960_body20_e56335_d_n5, assign41960_body20_e56335_d_n6, assign41960_body20_e56335_d_n7, assign41960_body20_e56335_d_n8, assign41960_body20_e56335_d_n9, assign41960_body20_e56335_d_n10, assign41960_body20_e56335_d_n11, assign41960_body20_e56335_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_flg_conv == 0.0)) {
        let assign41960_body20_e56333: f64 = (locals.var_ps0dep + locals.var_dps);
        (assign41960_body20_e56333, (locals.var_ps0dep_dn0 + locals.var_dps_dn0), (locals.var_ps0dep_dn2 + locals.var_dps_dn2), (locals.var_ps0dep_dn4 + locals.var_dps_dn4), (locals.var_ps0dep_dn5 + locals.var_dps_dn5), (locals.var_ps0dep_dn6 + locals.var_dps_dn6), (locals.var_ps0dep_dn7 + locals.var_dps_dn7), (locals.var_ps0dep_dn8 + locals.var_dps_dn8), (locals.var_ps0dep_dn9 + locals.var_dps_dn9), (locals.var_ps0dep_dn10 + locals.var_dps_dn10), (locals.var_ps0dep_dn11 + locals.var_dps_dn11), (locals.var_ps0dep_dn14 + locals.var_dps_dn14),)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    }
};
            locals.var_ps0dep = assign41960_body20_e56335;
            locals.var_ps0dep_dn0 = assign41960_body20_e56335_d_n0;
            locals.var_ps0dep_dn2 = assign41960_body20_e56335_d_n2;
            locals.var_ps0dep_dn4 = assign41960_body20_e56335_d_n4;
            locals.var_ps0dep_dn5 = assign41960_body20_e56335_d_n5;
            locals.var_ps0dep_dn6 = assign41960_body20_e56335_d_n6;
            locals.var_ps0dep_dn7 = assign41960_body20_e56335_d_n7;
            locals.var_ps0dep_dn8 = assign41960_body20_e56335_d_n8;
            locals.var_ps0dep_dn9 = assign41960_body20_e56335_d_n9;
            locals.var_ps0dep_dn10 = assign41960_body20_e56335_d_n10;
            locals.var_ps0dep_dn11 = assign41960_body20_e56335_d_n11;
            locals.var_ps0dep_dn14 = assign41960_body20_e56335_d_n14;
            locals.var_ps0dep_rv = 0.0;
            let (assign41960_body21_e56349,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) {
        let assign41960_body21_e56347: f64 = (locals.var_lp_s0 + 1.0);
        (assign41960_body21_e56347,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign41960_body21_e56349;
            locals.var_lp_s0_rv = 0.0;
        }

        let (assign41980_e56365, assign41980_e56365_d_n0, assign41980_e56365_d_n2, assign41980_e56365_d_n4, assign41980_e56365_d_n5, assign41980_e56365_d_n6, assign41980_e56365_d_n7, assign41980_e56365_d_n8, assign41980_e56365_d_n9, assign41980_e56365_d_n10, assign41980_e56365_d_n11, assign41980_e56365_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) {
        let assign41980_e56363: f64 = (-locals.var_ps0dep);
        (assign41980_e56363, (-locals.var_ps0dep_dn0), (-locals.var_ps0dep_dn2), (-locals.var_ps0dep_dn4), (-locals.var_ps0dep_dn5), (-locals.var_ps0dep_dn6), (-locals.var_ps0dep_dn7), (-locals.var_ps0dep_dn8), (-locals.var_ps0dep_dn9), (-locals.var_ps0dep_dn10), (-locals.var_ps0dep_dn11), (-locals.var_ps0dep_dn14),)
    } else {
        (locals.var_ps0dep, locals.var_ps0dep_dn0, locals.var_ps0dep_dn2, locals.var_ps0dep_dn4, locals.var_ps0dep_dn5, locals.var_ps0dep_dn6, locals.var_ps0dep_dn7, locals.var_ps0dep_dn8, locals.var_ps0dep_dn9, locals.var_ps0dep_dn10, locals.var_ps0dep_dn11, locals.var_ps0dep_dn14,)
    }
};
        locals.var_ps0dep = assign41980_e56365;
        locals.var_ps0dep_dn0 = assign41980_e56365_d_n0;
        locals.var_ps0dep_dn2 = assign41980_e56365_d_n2;
        locals.var_ps0dep_dn4 = assign41980_e56365_d_n4;
        locals.var_ps0dep_dn5 = assign41980_e56365_d_n5;
        locals.var_ps0dep_dn6 = assign41980_e56365_d_n6;
        locals.var_ps0dep_dn7 = assign41980_e56365_d_n7;
        locals.var_ps0dep_dn8 = assign41980_e56365_d_n8;
        locals.var_ps0dep_dn9 = assign41980_e56365_d_n9;
        locals.var_ps0dep_dn10 = assign41980_e56365_d_n10;
        locals.var_ps0dep_dn11 = assign41980_e56365_d_n11;
        locals.var_ps0dep_dn14 = assign41980_e56365_d_n14;
        locals.var_ps0dep_rv = 0.0;

        let (assign41990_e56385, assign41990_e56385_d_n0, assign41990_e56385_d_n2, assign41990_e56385_d_n4, assign41990_e56385_d_n5, assign41990_e56385_d_n6, assign41990_e56385_d_n7, assign41990_e56385_d_n8, assign41990_e56385_d_n9, assign41990_e56385_d_n10, assign41990_e56385_d_n11, assign41990_e56385_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) {
        let assign41990_e56377: f64 = (locals.var_q_ndepm__blk909 * locals.var_tnp);
        let assign41990_e56379: f64 = (assign41990_e56377 * locals.var_tnp);
        let assign41990_e56381: f64 = (assign41990_e56379 / 2.0);
        let assign41990_e56383: f64 = (assign41990_e56381 / 1.034943e-10);
        (assign41990_e56383, ((((((locals.var_q_ndepm__blk909_dn0 * locals.var_tnp) + (locals.var_q_ndepm__blk909 * locals.var_tnp_dn0)) * locals.var_tnp) + (assign41990_e56377 * locals.var_tnp_dn0)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk909_dn2 * locals.var_tnp) + (locals.var_q_ndepm__blk909 * locals.var_tnp_dn2)) * locals.var_tnp) + (assign41990_e56377 * locals.var_tnp_dn2)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk909_dn4 * locals.var_tnp) + (locals.var_q_ndepm__blk909 * locals.var_tnp_dn4)) * locals.var_tnp) + (assign41990_e56377 * locals.var_tnp_dn4)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk909_dn5 * locals.var_tnp) + (locals.var_q_ndepm__blk909 * locals.var_tnp_dn5)) * locals.var_tnp) + (assign41990_e56377 * locals.var_tnp_dn5)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk909_dn6 * locals.var_tnp) + (locals.var_q_ndepm__blk909 * locals.var_tnp_dn6)) * locals.var_tnp) + (assign41990_e56377 * locals.var_tnp_dn6)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk909_dn7 * locals.var_tnp) + (locals.var_q_ndepm__blk909 * locals.var_tnp_dn7)) * locals.var_tnp) + (assign41990_e56377 * locals.var_tnp_dn7)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk909_dn8 * locals.var_tnp) + (locals.var_q_ndepm__blk909 * locals.var_tnp_dn8)) * locals.var_tnp) + (assign41990_e56377 * locals.var_tnp_dn8)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk909_dn9 * locals.var_tnp) + (locals.var_q_ndepm__blk909 * locals.var_tnp_dn9)) * locals.var_tnp) + (assign41990_e56377 * locals.var_tnp_dn9)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk909_dn10 * locals.var_tnp) + (locals.var_q_ndepm__blk909 * locals.var_tnp_dn10)) * locals.var_tnp) + (assign41990_e56377 * locals.var_tnp_dn10)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk909_dn11 * locals.var_tnp) + (locals.var_q_ndepm__blk909 * locals.var_tnp_dn11)) * locals.var_tnp) + (assign41990_e56377 * locals.var_tnp_dn11)) / 2.0) / 1.034943e-10), ((((((locals.var_q_ndepm__blk909_dn14 * locals.var_tnp) + (locals.var_q_ndepm__blk909 * locals.var_tnp_dn14)) * locals.var_tnp) + (assign41990_e56377 * locals.var_tnp_dn14)) / 2.0) / 1.034943e-10),)
    } else {
        (locals.var_dphi_sb__blk1030, locals.var_dphi_sb__blk1030_dn0, locals.var_dphi_sb__blk1030_dn2, locals.var_dphi_sb__blk1030_dn4, locals.var_dphi_sb__blk1030_dn5, locals.var_dphi_sb__blk1030_dn6, locals.var_dphi_sb__blk1030_dn7, locals.var_dphi_sb__blk1030_dn8, locals.var_dphi_sb__blk1030_dn9, locals.var_dphi_sb__blk1030_dn10, locals.var_dphi_sb__blk1030_dn11, locals.var_dphi_sb__blk1030_dn14,)
    }
};
        locals.var_dphi_sb__blk1030 = assign41990_e56385;
        locals.var_dphi_sb__blk1030_dn0 = assign41990_e56385_d_n0;
        locals.var_dphi_sb__blk1030_dn2 = assign41990_e56385_d_n2;
        locals.var_dphi_sb__blk1030_dn4 = assign41990_e56385_d_n4;
        locals.var_dphi_sb__blk1030_dn5 = assign41990_e56385_d_n5;
        locals.var_dphi_sb__blk1030_dn6 = assign41990_e56385_d_n6;
        locals.var_dphi_sb__blk1030_dn7 = assign41990_e56385_d_n7;
        locals.var_dphi_sb__blk1030_dn8 = assign41990_e56385_d_n8;
        locals.var_dphi_sb__blk1030_dn9 = assign41990_e56385_d_n9;
        locals.var_dphi_sb__blk1030_dn10 = assign41990_e56385_d_n10;
        locals.var_dphi_sb__blk1030_dn11 = assign41990_e56385_d_n11;
        locals.var_dphi_sb__blk1030_dn14 = assign41990_e56385_d_n14;
        locals.var_dphi_sb__blk1030_rv = 0.0;

        let (assign42000_e56404, assign42000_e56404_d_n0, assign42000_e56404_d_n2, assign42000_e56404_d_n4, assign42000_e56404_d_n5, assign42000_e56404_d_n6, assign42000_e56404_d_n7, assign42000_e56404_d_n8, assign42000_e56404_d_n9, assign42000_e56404_d_n10, assign42000_e56404_d_n11, assign42000_e56404_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) {
        let assign42000_e56398: f64 = (2.0 * locals.var_beta);
        let assign42000_e56400: f64 = (assign42000_e56398 * locals.var_dphi_sb__blk1030);
        let assign42000_e56401: f64 = (assign42000_e56400).sqrt();
        let assign42000_e56402: f64 = (p.p394 * assign42000_e56401);
        (assign42000_e56402, (p.p394 * ((((2.0 * locals.var_beta_dn0) * locals.var_dphi_sb__blk1030) + (assign42000_e56398 * locals.var_dphi_sb__blk1030_dn0)) / (2.0 * assign42000_e56401))), (p.p394 * ((((2.0 * locals.var_beta_dn2) * locals.var_dphi_sb__blk1030) + (assign42000_e56398 * locals.var_dphi_sb__blk1030_dn2)) / (2.0 * assign42000_e56401))), (p.p394 * ((((2.0 * locals.var_beta_dn4) * locals.var_dphi_sb__blk1030) + (assign42000_e56398 * locals.var_dphi_sb__blk1030_dn4)) / (2.0 * assign42000_e56401))), (p.p394 * ((((2.0 * locals.var_beta_dn5) * locals.var_dphi_sb__blk1030) + (assign42000_e56398 * locals.var_dphi_sb__blk1030_dn5)) / (2.0 * assign42000_e56401))), (p.p394 * ((((2.0 * locals.var_beta_dn6) * locals.var_dphi_sb__blk1030) + (assign42000_e56398 * locals.var_dphi_sb__blk1030_dn6)) / (2.0 * assign42000_e56401))), (p.p394 * ((((2.0 * locals.var_beta_dn7) * locals.var_dphi_sb__blk1030) + (assign42000_e56398 * locals.var_dphi_sb__blk1030_dn7)) / (2.0 * assign42000_e56401))), (p.p394 * ((((2.0 * locals.var_beta_dn8) * locals.var_dphi_sb__blk1030) + (assign42000_e56398 * locals.var_dphi_sb__blk1030_dn8)) / (2.0 * assign42000_e56401))), (p.p394 * ((((2.0 * locals.var_beta_dn9) * locals.var_dphi_sb__blk1030) + (assign42000_e56398 * locals.var_dphi_sb__blk1030_dn9)) / (2.0 * assign42000_e56401))), (p.p394 * ((((2.0 * locals.var_beta_dn10) * locals.var_dphi_sb__blk1030) + (assign42000_e56398 * locals.var_dphi_sb__blk1030_dn10)) / (2.0 * assign42000_e56401))), (p.p394 * ((((2.0 * locals.var_beta_dn11) * locals.var_dphi_sb__blk1030) + (assign42000_e56398 * locals.var_dphi_sb__blk1030_dn11)) / (2.0 * assign42000_e56401))), (p.p394 * ((((2.0 * locals.var_beta_dn14) * locals.var_dphi_sb__blk1030) + (assign42000_e56398 * locals.var_dphi_sb__blk1030_dn14)) / (2.0 * assign42000_e56401))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign42000_e56404;
        locals.var_t0_dn0 = assign42000_e56404_d_n0;
        locals.var_t0_dn2 = assign42000_e56404_d_n2;
        locals.var_t0_dn4 = assign42000_e56404_d_n4;
        locals.var_t0_dn5 = assign42000_e56404_d_n5;
        locals.var_t0_dn6 = assign42000_e56404_d_n6;
        locals.var_t0_dn7 = assign42000_e56404_d_n7;
        locals.var_t0_dn8 = assign42000_e56404_d_n8;
        locals.var_t0_dn9 = assign42000_e56404_d_n9;
        locals.var_t0_dn10 = assign42000_e56404_d_n10;
        locals.var_t0_dn11 = assign42000_e56404_d_n11;
        locals.var_t0_dn14 = assign42000_e56404_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign42010_e56423, assign42010_e56423_d_n0, assign42010_e56423_d_n2, assign42010_e56423_d_n4, assign42010_e56423_d_n5, assign42010_e56423_d_n6, assign42010_e56423_d_n7, assign42010_e56423_d_n8, assign42010_e56423_d_n9, assign42010_e56423_d_n10, assign42010_e56423_d_n11, assign42010_e56423_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) {
        let assign42010_e56415: f64 = (locals.var_t0).exp();
        let assign42010_e56417: f64 = (-locals.var_t0);
        let assign42010_e56418: f64 = (assign42010_e56417).exp();
        let assign42010_e56419: f64 = (assign42010_e56415 + assign42010_e56418);
        let assign42010_e56421: f64 = (assign42010_e56419 / 2.0);
        (assign42010_e56421, (((assign42010_e56415 * locals.var_t0_dn0) + (assign42010_e56418 * (-locals.var_t0_dn0))) / 2.0), (((assign42010_e56415 * locals.var_t0_dn2) + (assign42010_e56418 * (-locals.var_t0_dn2))) / 2.0), (((assign42010_e56415 * locals.var_t0_dn4) + (assign42010_e56418 * (-locals.var_t0_dn4))) / 2.0), (((assign42010_e56415 * locals.var_t0_dn5) + (assign42010_e56418 * (-locals.var_t0_dn5))) / 2.0), (((assign42010_e56415 * locals.var_t0_dn6) + (assign42010_e56418 * (-locals.var_t0_dn6))) / 2.0), (((assign42010_e56415 * locals.var_t0_dn7) + (assign42010_e56418 * (-locals.var_t0_dn7))) / 2.0), (((assign42010_e56415 * locals.var_t0_dn8) + (assign42010_e56418 * (-locals.var_t0_dn8))) / 2.0), (((assign42010_e56415 * locals.var_t0_dn9) + (assign42010_e56418 * (-locals.var_t0_dn9))) / 2.0), (((assign42010_e56415 * locals.var_t0_dn10) + (assign42010_e56418 * (-locals.var_t0_dn10))) / 2.0), (((assign42010_e56415 * locals.var_t0_dn11) + (assign42010_e56418 * (-locals.var_t0_dn11))) / 2.0), (((assign42010_e56415 * locals.var_t0_dn14) + (assign42010_e56418 * (-locals.var_t0_dn14))) / 2.0),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign42010_e56423;
        locals.var_t1_dn0 = assign42010_e56423_d_n0;
        locals.var_t1_dn2 = assign42010_e56423_d_n2;
        locals.var_t1_dn4 = assign42010_e56423_d_n4;
        locals.var_t1_dn5 = assign42010_e56423_d_n5;
        locals.var_t1_dn6 = assign42010_e56423_d_n6;
        locals.var_t1_dn7 = assign42010_e56423_d_n7;
        locals.var_t1_dn8 = assign42010_e56423_d_n8;
        locals.var_t1_dn9 = assign42010_e56423_d_n9;
        locals.var_t1_dn10 = assign42010_e56423_d_n10;
        locals.var_t1_dn11 = assign42010_e56423_d_n11;
        locals.var_t1_dn14 = assign42010_e56423_d_n14;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_145(
        locals: &mut StampLocals,
    ) {
        let (assign42020_e56438, assign42020_e56438_d_n0, assign42020_e56438_d_n2, assign42020_e56438_d_n4, assign42020_e56438_d_n5, assign42020_e56438_d_n6, assign42020_e56438_d_n7, assign42020_e56438_d_n8, assign42020_e56438_d_n9, assign42020_e56438_d_n10, assign42020_e56438_d_n11, assign42020_e56438_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) {
        let assign42020_e56434: f64 = (locals.var_t1).ln();
        let assign42020_e56436: f64 = (assign42020_e56434 / locals.var_dphi_sb__blk1030);
        (assign42020_e56436, ((((locals.var_t1_dn0 / locals.var_t1) * locals.var_dphi_sb__blk1030) - (assign42020_e56434 * locals.var_dphi_sb__blk1030_dn0)) / (locals.var_dphi_sb__blk1030 * locals.var_dphi_sb__blk1030)), ((((locals.var_t1_dn2 / locals.var_t1) * locals.var_dphi_sb__blk1030) - (assign42020_e56434 * locals.var_dphi_sb__blk1030_dn2)) / (locals.var_dphi_sb__blk1030 * locals.var_dphi_sb__blk1030)), ((((locals.var_t1_dn4 / locals.var_t1) * locals.var_dphi_sb__blk1030) - (assign42020_e56434 * locals.var_dphi_sb__blk1030_dn4)) / (locals.var_dphi_sb__blk1030 * locals.var_dphi_sb__blk1030)), ((((locals.var_t1_dn5 / locals.var_t1) * locals.var_dphi_sb__blk1030) - (assign42020_e56434 * locals.var_dphi_sb__blk1030_dn5)) / (locals.var_dphi_sb__blk1030 * locals.var_dphi_sb__blk1030)), ((((locals.var_t1_dn6 / locals.var_t1) * locals.var_dphi_sb__blk1030) - (assign42020_e56434 * locals.var_dphi_sb__blk1030_dn6)) / (locals.var_dphi_sb__blk1030 * locals.var_dphi_sb__blk1030)), ((((locals.var_t1_dn7 / locals.var_t1) * locals.var_dphi_sb__blk1030) - (assign42020_e56434 * locals.var_dphi_sb__blk1030_dn7)) / (locals.var_dphi_sb__blk1030 * locals.var_dphi_sb__blk1030)), ((((locals.var_t1_dn8 / locals.var_t1) * locals.var_dphi_sb__blk1030) - (assign42020_e56434 * locals.var_dphi_sb__blk1030_dn8)) / (locals.var_dphi_sb__blk1030 * locals.var_dphi_sb__blk1030)), ((((locals.var_t1_dn9 / locals.var_t1) * locals.var_dphi_sb__blk1030) - (assign42020_e56434 * locals.var_dphi_sb__blk1030_dn9)) / (locals.var_dphi_sb__blk1030 * locals.var_dphi_sb__blk1030)), ((((locals.var_t1_dn10 / locals.var_t1) * locals.var_dphi_sb__blk1030) - (assign42020_e56434 * locals.var_dphi_sb__blk1030_dn10)) / (locals.var_dphi_sb__blk1030 * locals.var_dphi_sb__blk1030)), ((((locals.var_t1_dn11 / locals.var_t1) * locals.var_dphi_sb__blk1030) - (assign42020_e56434 * locals.var_dphi_sb__blk1030_dn11)) / (locals.var_dphi_sb__blk1030 * locals.var_dphi_sb__blk1030)), ((((locals.var_t1_dn14 / locals.var_t1) * locals.var_dphi_sb__blk1030) - (assign42020_e56434 * locals.var_dphi_sb__blk1030_dn14)) / (locals.var_dphi_sb__blk1030 * locals.var_dphi_sb__blk1030)),)
    } else {
        (locals.var_c_sb__blk1031, locals.var_c_sb__blk1031_dn0, locals.var_c_sb__blk1031_dn2, locals.var_c_sb__blk1031_dn4, locals.var_c_sb__blk1031_dn5, locals.var_c_sb__blk1031_dn6, locals.var_c_sb__blk1031_dn7, locals.var_c_sb__blk1031_dn8, locals.var_c_sb__blk1031_dn9, locals.var_c_sb__blk1031_dn10, locals.var_c_sb__blk1031_dn11, locals.var_c_sb__blk1031_dn14,)
    }
};
        locals.var_c_sb__blk1031 = assign42020_e56438;
        locals.var_c_sb__blk1031_dn0 = assign42020_e56438_d_n0;
        locals.var_c_sb__blk1031_dn2 = assign42020_e56438_d_n2;
        locals.var_c_sb__blk1031_dn4 = assign42020_e56438_d_n4;
        locals.var_c_sb__blk1031_dn5 = assign42020_e56438_d_n5;
        locals.var_c_sb__blk1031_dn6 = assign42020_e56438_d_n6;
        locals.var_c_sb__blk1031_dn7 = assign42020_e56438_d_n7;
        locals.var_c_sb__blk1031_dn8 = assign42020_e56438_d_n8;
        locals.var_c_sb__blk1031_dn9 = assign42020_e56438_d_n9;
        locals.var_c_sb__blk1031_dn10 = assign42020_e56438_d_n10;
        locals.var_c_sb__blk1031_dn11 = assign42020_e56438_d_n11;
        locals.var_c_sb__blk1031_dn14 = assign42020_e56438_d_n14;
        locals.var_c_sb__blk1031_rv = 0.0;

        let (assign42030_e56452, assign42030_e56452_d_n0, assign42030_e56452_d_n2, assign42030_e56452_d_n4, assign42030_e56452_d_n5, assign42030_e56452_d_n6, assign42030_e56452_d_n7, assign42030_e56452_d_n8, assign42030_e56452_d_n9, assign42030_e56452_d_n10, assign42030_e56452_d_n11, assign42030_e56452_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) {
        let assign42030_e56450: f64 = (locals.var_c_sb__blk1031 * locals.var_ps0dep);
        (assign42030_e56450, ((locals.var_c_sb__blk1031_dn0 * locals.var_ps0dep) + (locals.var_c_sb__blk1031 * locals.var_ps0dep_dn0)), ((locals.var_c_sb__blk1031_dn2 * locals.var_ps0dep) + (locals.var_c_sb__blk1031 * locals.var_ps0dep_dn2)), ((locals.var_c_sb__blk1031_dn4 * locals.var_ps0dep) + (locals.var_c_sb__blk1031 * locals.var_ps0dep_dn4)), ((locals.var_c_sb__blk1031_dn5 * locals.var_ps0dep) + (locals.var_c_sb__blk1031 * locals.var_ps0dep_dn5)), ((locals.var_c_sb__blk1031_dn6 * locals.var_ps0dep) + (locals.var_c_sb__blk1031 * locals.var_ps0dep_dn6)), ((locals.var_c_sb__blk1031_dn7 * locals.var_ps0dep) + (locals.var_c_sb__blk1031 * locals.var_ps0dep_dn7)), ((locals.var_c_sb__blk1031_dn8 * locals.var_ps0dep) + (locals.var_c_sb__blk1031 * locals.var_ps0dep_dn8)), ((locals.var_c_sb__blk1031_dn9 * locals.var_ps0dep) + (locals.var_c_sb__blk1031 * locals.var_ps0dep_dn9)), ((locals.var_c_sb__blk1031_dn10 * locals.var_ps0dep) + (locals.var_c_sb__blk1031 * locals.var_ps0dep_dn10)), ((locals.var_c_sb__blk1031_dn11 * locals.var_ps0dep) + (locals.var_c_sb__blk1031 * locals.var_ps0dep_dn11)), ((locals.var_c_sb__blk1031_dn14 * locals.var_ps0dep) + (locals.var_c_sb__blk1031 * locals.var_ps0dep_dn14)),)
    } else {
        (locals.var_tx, locals.var_tx_dn0, locals.var_tx_dn2, locals.var_tx_dn4, locals.var_tx_dn5, locals.var_tx_dn6, locals.var_tx_dn7, locals.var_tx_dn8, locals.var_tx_dn9, locals.var_tx_dn10, locals.var_tx_dn11, locals.var_tx_dn14,)
    }
};
        locals.var_tx = assign42030_e56452;
        locals.var_tx_dn0 = assign42030_e56452_d_n0;
        locals.var_tx_dn2 = assign42030_e56452_d_n2;
        locals.var_tx_dn4 = assign42030_e56452_d_n4;
        locals.var_tx_dn5 = assign42030_e56452_d_n5;
        locals.var_tx_dn6 = assign42030_e56452_d_n6;
        locals.var_tx_dn7 = assign42030_e56452_d_n7;
        locals.var_tx_dn8 = assign42030_e56452_d_n8;
        locals.var_tx_dn9 = assign42030_e56452_d_n9;
        locals.var_tx_dn10 = assign42030_e56452_d_n10;
        locals.var_tx_dn11 = assign42030_e56452_d_n11;
        locals.var_tx_dn14 = assign42030_e56452_d_n14;
        locals.var_tx_rv = 0.0;

        let (assign42040_e56468, assign42040_e56468_d_n0, assign42040_e56468_d_n2, assign42040_e56468_d_n4, assign42040_e56468_d_n5, assign42040_e56468_d_n6, assign42040_e56468_d_n7, assign42040_e56468_d_n8, assign42040_e56468_d_n9, assign42040_e56468_d_n10, assign42040_e56468_d_n11, assign42040_e56468_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) {
        let assign42040_e56463: f64 = (-locals.var_c_sb__blk1031);
        let assign42040_e56465: f64 = (assign42040_e56463 * locals.var_dphi_sb__blk1030);
        let assign42040_e56466: f64 = (assign42040_e56465).exp();
        (assign42040_e56466, (assign42040_e56466 * (((-locals.var_c_sb__blk1031_dn0) * locals.var_dphi_sb__blk1030) + (assign42040_e56463 * locals.var_dphi_sb__blk1030_dn0))), (assign42040_e56466 * (((-locals.var_c_sb__blk1031_dn2) * locals.var_dphi_sb__blk1030) + (assign42040_e56463 * locals.var_dphi_sb__blk1030_dn2))), (assign42040_e56466 * (((-locals.var_c_sb__blk1031_dn4) * locals.var_dphi_sb__blk1030) + (assign42040_e56463 * locals.var_dphi_sb__blk1030_dn4))), (assign42040_e56466 * (((-locals.var_c_sb__blk1031_dn5) * locals.var_dphi_sb__blk1030) + (assign42040_e56463 * locals.var_dphi_sb__blk1030_dn5))), (assign42040_e56466 * (((-locals.var_c_sb__blk1031_dn6) * locals.var_dphi_sb__blk1030) + (assign42040_e56463 * locals.var_dphi_sb__blk1030_dn6))), (assign42040_e56466 * (((-locals.var_c_sb__blk1031_dn7) * locals.var_dphi_sb__blk1030) + (assign42040_e56463 * locals.var_dphi_sb__blk1030_dn7))), (assign42040_e56466 * (((-locals.var_c_sb__blk1031_dn8) * locals.var_dphi_sb__blk1030) + (assign42040_e56463 * locals.var_dphi_sb__blk1030_dn8))), (assign42040_e56466 * (((-locals.var_c_sb__blk1031_dn9) * locals.var_dphi_sb__blk1030) + (assign42040_e56463 * locals.var_dphi_sb__blk1030_dn9))), (assign42040_e56466 * (((-locals.var_c_sb__blk1031_dn10) * locals.var_dphi_sb__blk1030) + (assign42040_e56463 * locals.var_dphi_sb__blk1030_dn10))), (assign42040_e56466 * (((-locals.var_c_sb__blk1031_dn11) * locals.var_dphi_sb__blk1030) + (assign42040_e56463 * locals.var_dphi_sb__blk1030_dn11))), (assign42040_e56466 * (((-locals.var_c_sb__blk1031_dn14) * locals.var_dphi_sb__blk1030) + (assign42040_e56463 * locals.var_dphi_sb__blk1030_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign42040_e56468;
        locals.var_t0_dn0 = assign42040_e56468_d_n0;
        locals.var_t0_dn2 = assign42040_e56468_d_n2;
        locals.var_t0_dn4 = assign42040_e56468_d_n4;
        locals.var_t0_dn5 = assign42040_e56468_d_n5;
        locals.var_t0_dn6 = assign42040_e56468_d_n6;
        locals.var_t0_dn7 = assign42040_e56468_d_n7;
        locals.var_t0_dn8 = assign42040_e56468_d_n8;
        locals.var_t0_dn9 = assign42040_e56468_d_n9;
        locals.var_t0_dn10 = assign42040_e56468_d_n10;
        locals.var_t0_dn11 = assign42040_e56468_d_n11;
        locals.var_t0_dn14 = assign42040_e56468_d_n14;
        locals.var_t0_rv = 0.0;

        let assign42050_e56470: f64 = (locals.var_tx).abs();
        let assign42050_e56472: f64 = if assign42050_e56470 > 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard1052 = assign42050_e56472;
        locals.var_guard1052_rv = 0.0;

        let (assign42060_e56489, assign42060_e56489_d_n0, assign42060_e56489_d_n2, assign42060_e56489_d_n4, assign42060_e56489_d_n5, assign42060_e56489_d_n6, assign42060_e56489_d_n7, assign42060_e56489_d_n8, assign42060_e56489_d_n9, assign42060_e56489_d_n10, assign42060_e56489_d_n11, assign42060_e56489_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1052 != 0.0)) {
        let assign42060_e56485: f64 = (locals.var_tx).exp();
        let assign42060_e56487: f64 = (assign42060_e56485 * locals.var_t0);
        (assign42060_e56487, (((assign42060_e56485 * locals.var_tx_dn0) * locals.var_t0) + (assign42060_e56485 * locals.var_t0_dn0)), (((assign42060_e56485 * locals.var_tx_dn2) * locals.var_t0) + (assign42060_e56485 * locals.var_t0_dn2)), (((assign42060_e56485 * locals.var_tx_dn4) * locals.var_t0) + (assign42060_e56485 * locals.var_t0_dn4)), (((assign42060_e56485 * locals.var_tx_dn5) * locals.var_t0) + (assign42060_e56485 * locals.var_t0_dn5)), (((assign42060_e56485 * locals.var_tx_dn6) * locals.var_t0) + (assign42060_e56485 * locals.var_t0_dn6)), (((assign42060_e56485 * locals.var_tx_dn7) * locals.var_t0) + (assign42060_e56485 * locals.var_t0_dn7)), (((assign42060_e56485 * locals.var_tx_dn8) * locals.var_t0) + (assign42060_e56485 * locals.var_t0_dn8)), (((assign42060_e56485 * locals.var_tx_dn9) * locals.var_t0) + (assign42060_e56485 * locals.var_t0_dn9)), (((assign42060_e56485 * locals.var_tx_dn10) * locals.var_t0) + (assign42060_e56485 * locals.var_t0_dn10)), (((assign42060_e56485 * locals.var_tx_dn11) * locals.var_t0) + (assign42060_e56485 * locals.var_t0_dn11)), (((assign42060_e56485 * locals.var_tx_dn14) * locals.var_t0) + (assign42060_e56485 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign42060_e56489;
        locals.var_t1_dn0 = assign42060_e56489_d_n0;
        locals.var_t1_dn2 = assign42060_e56489_d_n2;
        locals.var_t1_dn4 = assign42060_e56489_d_n4;
        locals.var_t1_dn5 = assign42060_e56489_d_n5;
        locals.var_t1_dn6 = assign42060_e56489_d_n6;
        locals.var_t1_dn7 = assign42060_e56489_d_n7;
        locals.var_t1_dn8 = assign42060_e56489_d_n8;
        locals.var_t1_dn9 = assign42060_e56489_d_n9;
        locals.var_t1_dn10 = assign42060_e56489_d_n10;
        locals.var_t1_dn11 = assign42060_e56489_d_n11;
        locals.var_t1_dn14 = assign42060_e56489_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign42070_e56505, assign42070_e56505_d_n0, assign42070_e56505_d_n2, assign42070_e56505_d_n4, assign42070_e56505_d_n5, assign42070_e56505_d_n6, assign42070_e56505_d_n7, assign42070_e56505_d_n8, assign42070_e56505_d_n9, assign42070_e56505_d_n10, assign42070_e56505_d_n11, assign42070_e56505_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1052 != 0.0)) {
        let assign42070_e56503: f64 = (locals.var_t1 - locals.var_t0);
        (assign42070_e56503, (locals.var_t1_dn0 - locals.var_t0_dn0), (locals.var_t1_dn2 - locals.var_t0_dn2), (locals.var_t1_dn4 - locals.var_t0_dn4), (locals.var_t1_dn5 - locals.var_t0_dn5), (locals.var_t1_dn6 - locals.var_t0_dn6), (locals.var_t1_dn7 - locals.var_t0_dn7), (locals.var_t1_dn8 - locals.var_t0_dn8), (locals.var_t1_dn9 - locals.var_t0_dn9), (locals.var_t1_dn10 - locals.var_t0_dn10), (locals.var_t1_dn11 - locals.var_t0_dn11), (locals.var_t1_dn14 - locals.var_t0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign42070_e56505;
        locals.var_t2_dn0 = assign42070_e56505_d_n0;
        locals.var_t2_dn2 = assign42070_e56505_d_n2;
        locals.var_t2_dn4 = assign42070_e56505_d_n4;
        locals.var_t2_dn5 = assign42070_e56505_d_n5;
        locals.var_t2_dn6 = assign42070_e56505_d_n6;
        locals.var_t2_dn7 = assign42070_e56505_d_n7;
        locals.var_t2_dn8 = assign42070_e56505_d_n8;
        locals.var_t2_dn9 = assign42070_e56505_d_n9;
        locals.var_t2_dn10 = assign42070_e56505_d_n10;
        locals.var_t2_dn11 = assign42070_e56505_d_n11;
        locals.var_t2_dn14 = assign42070_e56505_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign42080_e56524, assign42080_e56524_d_n0, assign42080_e56524_d_n2, assign42080_e56524_d_n4, assign42080_e56524_d_n5, assign42080_e56524_d_n6, assign42080_e56524_d_n7, assign42080_e56524_d_n8, assign42080_e56524_d_n9, assign42080_e56524_d_n10, assign42080_e56524_d_n11, assign42080_e56524_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1052 == 0.0)) {
        let assign42080_e56520: f64 = (1.0 + locals.var_tx);
        let assign42080_e56522: f64 = (assign42080_e56520 * locals.var_t0);
        (assign42080_e56522, ((locals.var_tx_dn0 * locals.var_t0) + (assign42080_e56520 * locals.var_t0_dn0)), ((locals.var_tx_dn2 * locals.var_t0) + (assign42080_e56520 * locals.var_t0_dn2)), ((locals.var_tx_dn4 * locals.var_t0) + (assign42080_e56520 * locals.var_t0_dn4)), ((locals.var_tx_dn5 * locals.var_t0) + (assign42080_e56520 * locals.var_t0_dn5)), ((locals.var_tx_dn6 * locals.var_t0) + (assign42080_e56520 * locals.var_t0_dn6)), ((locals.var_tx_dn7 * locals.var_t0) + (assign42080_e56520 * locals.var_t0_dn7)), ((locals.var_tx_dn8 * locals.var_t0) + (assign42080_e56520 * locals.var_t0_dn8)), ((locals.var_tx_dn9 * locals.var_t0) + (assign42080_e56520 * locals.var_t0_dn9)), ((locals.var_tx_dn10 * locals.var_t0) + (assign42080_e56520 * locals.var_t0_dn10)), ((locals.var_tx_dn11 * locals.var_t0) + (assign42080_e56520 * locals.var_t0_dn11)), ((locals.var_tx_dn14 * locals.var_t0) + (assign42080_e56520 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign42080_e56524;
        locals.var_t1_dn0 = assign42080_e56524_d_n0;
        locals.var_t1_dn2 = assign42080_e56524_d_n2;
        locals.var_t1_dn4 = assign42080_e56524_d_n4;
        locals.var_t1_dn5 = assign42080_e56524_d_n5;
        locals.var_t1_dn6 = assign42080_e56524_d_n6;
        locals.var_t1_dn7 = assign42080_e56524_d_n7;
        locals.var_t1_dn8 = assign42080_e56524_d_n8;
        locals.var_t1_dn9 = assign42080_e56524_d_n9;
        locals.var_t1_dn10 = assign42080_e56524_d_n10;
        locals.var_t1_dn11 = assign42080_e56524_d_n11;
        locals.var_t1_dn14 = assign42080_e56524_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign42090_e56547, assign42090_e56547_d_n0, assign42090_e56547_d_n2, assign42090_e56547_d_n4, assign42090_e56547_d_n5, assign42090_e56547_d_n6, assign42090_e56547_d_n7, assign42090_e56547_d_n8, assign42090_e56547_d_n9, assign42090_e56547_d_n10, assign42090_e56547_d_n11, assign42090_e56547_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1052 == 0.0)) {
        let assign42090_e56541: f64 = (locals.var_tx / 2.0);
        let assign42090_e56542: f64 = (1.0 + assign42090_e56541);
        let assign42090_e56543: f64 = (locals.var_tx * assign42090_e56542);
        let assign42090_e56545: f64 = (assign42090_e56543 * locals.var_t0);
        (assign42090_e56545, ((((locals.var_tx_dn0 * assign42090_e56542) + (locals.var_tx * (locals.var_tx_dn0 / 2.0))) * locals.var_t0) + (assign42090_e56543 * locals.var_t0_dn0)), ((((locals.var_tx_dn2 * assign42090_e56542) + (locals.var_tx * (locals.var_tx_dn2 / 2.0))) * locals.var_t0) + (assign42090_e56543 * locals.var_t0_dn2)), ((((locals.var_tx_dn4 * assign42090_e56542) + (locals.var_tx * (locals.var_tx_dn4 / 2.0))) * locals.var_t0) + (assign42090_e56543 * locals.var_t0_dn4)), ((((locals.var_tx_dn5 * assign42090_e56542) + (locals.var_tx * (locals.var_tx_dn5 / 2.0))) * locals.var_t0) + (assign42090_e56543 * locals.var_t0_dn5)), ((((locals.var_tx_dn6 * assign42090_e56542) + (locals.var_tx * (locals.var_tx_dn6 / 2.0))) * locals.var_t0) + (assign42090_e56543 * locals.var_t0_dn6)), ((((locals.var_tx_dn7 * assign42090_e56542) + (locals.var_tx * (locals.var_tx_dn7 / 2.0))) * locals.var_t0) + (assign42090_e56543 * locals.var_t0_dn7)), ((((locals.var_tx_dn8 * assign42090_e56542) + (locals.var_tx * (locals.var_tx_dn8 / 2.0))) * locals.var_t0) + (assign42090_e56543 * locals.var_t0_dn8)), ((((locals.var_tx_dn9 * assign42090_e56542) + (locals.var_tx * (locals.var_tx_dn9 / 2.0))) * locals.var_t0) + (assign42090_e56543 * locals.var_t0_dn9)), ((((locals.var_tx_dn10 * assign42090_e56542) + (locals.var_tx * (locals.var_tx_dn10 / 2.0))) * locals.var_t0) + (assign42090_e56543 * locals.var_t0_dn10)), ((((locals.var_tx_dn11 * assign42090_e56542) + (locals.var_tx * (locals.var_tx_dn11 / 2.0))) * locals.var_t0) + (assign42090_e56543 * locals.var_t0_dn11)), ((((locals.var_tx_dn14 * assign42090_e56542) + (locals.var_tx * (locals.var_tx_dn14 / 2.0))) * locals.var_t0) + (assign42090_e56543 * locals.var_t0_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign42090_e56547;
        locals.var_t2_dn0 = assign42090_e56547_d_n0;
        locals.var_t2_dn2 = assign42090_e56547_d_n2;
        locals.var_t2_dn4 = assign42090_e56547_d_n4;
        locals.var_t2_dn5 = assign42090_e56547_d_n5;
        locals.var_t2_dn6 = assign42090_e56547_d_n6;
        locals.var_t2_dn7 = assign42090_e56547_d_n7;
        locals.var_t2_dn8 = assign42090_e56547_d_n8;
        locals.var_t2_dn9 = assign42090_e56547_d_n9;
        locals.var_t2_dn10 = assign42090_e56547_d_n10;
        locals.var_t2_dn11 = assign42090_e56547_d_n11;
        locals.var_t2_dn14 = assign42090_e56547_d_n14;
        locals.var_t2_rv = 0.0;

        let assign42100_e56549: f64 = (locals.var_t2).abs();
        let assign42100_e56551: f64 = if assign42100_e56549 > 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard1053 = assign42100_e56551;
        locals.var_guard1053_rv = 0.0;

        let (assign42110_e56570, assign42110_e56570_d_n0, assign42110_e56570_d_n2, assign42110_e56570_d_n4, assign42110_e56570_d_n5, assign42110_e56570_d_n6, assign42110_e56570_d_n7, assign42110_e56570_d_n8, assign42110_e56570_d_n9, assign42110_e56570_d_n10, assign42110_e56570_d_n11, assign42110_e56570_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1053 != 0.0)) {
        let assign42110_e56565: f64 = (1.0 + locals.var_t2);
        let assign42110_e56566: f64 = (assign42110_e56565).ln();
        let assign42110_e56568: f64 = (assign42110_e56566 / locals.var_c_sb__blk1031);
        (assign42110_e56568, ((((locals.var_t2_dn0 / assign42110_e56565) * locals.var_c_sb__blk1031) - (assign42110_e56566 * locals.var_c_sb__blk1031_dn0)) / (locals.var_c_sb__blk1031 * locals.var_c_sb__blk1031)), ((((locals.var_t2_dn2 / assign42110_e56565) * locals.var_c_sb__blk1031) - (assign42110_e56566 * locals.var_c_sb__blk1031_dn2)) / (locals.var_c_sb__blk1031 * locals.var_c_sb__blk1031)), ((((locals.var_t2_dn4 / assign42110_e56565) * locals.var_c_sb__blk1031) - (assign42110_e56566 * locals.var_c_sb__blk1031_dn4)) / (locals.var_c_sb__blk1031 * locals.var_c_sb__blk1031)), ((((locals.var_t2_dn5 / assign42110_e56565) * locals.var_c_sb__blk1031) - (assign42110_e56566 * locals.var_c_sb__blk1031_dn5)) / (locals.var_c_sb__blk1031 * locals.var_c_sb__blk1031)), ((((locals.var_t2_dn6 / assign42110_e56565) * locals.var_c_sb__blk1031) - (assign42110_e56566 * locals.var_c_sb__blk1031_dn6)) / (locals.var_c_sb__blk1031 * locals.var_c_sb__blk1031)), ((((locals.var_t2_dn7 / assign42110_e56565) * locals.var_c_sb__blk1031) - (assign42110_e56566 * locals.var_c_sb__blk1031_dn7)) / (locals.var_c_sb__blk1031 * locals.var_c_sb__blk1031)), ((((locals.var_t2_dn8 / assign42110_e56565) * locals.var_c_sb__blk1031) - (assign42110_e56566 * locals.var_c_sb__blk1031_dn8)) / (locals.var_c_sb__blk1031 * locals.var_c_sb__blk1031)), ((((locals.var_t2_dn9 / assign42110_e56565) * locals.var_c_sb__blk1031) - (assign42110_e56566 * locals.var_c_sb__blk1031_dn9)) / (locals.var_c_sb__blk1031 * locals.var_c_sb__blk1031)), ((((locals.var_t2_dn10 / assign42110_e56565) * locals.var_c_sb__blk1031) - (assign42110_e56566 * locals.var_c_sb__blk1031_dn10)) / (locals.var_c_sb__blk1031 * locals.var_c_sb__blk1031)), ((((locals.var_t2_dn11 / assign42110_e56565) * locals.var_c_sb__blk1031) - (assign42110_e56566 * locals.var_c_sb__blk1031_dn11)) / (locals.var_c_sb__blk1031 * locals.var_c_sb__blk1031)), ((((locals.var_t2_dn14 / assign42110_e56565) * locals.var_c_sb__blk1031) - (assign42110_e56566 * locals.var_c_sb__blk1031_dn14)) / (locals.var_c_sb__blk1031 * locals.var_c_sb__blk1031)),)
    } else {
        (locals.var_pb0dep, locals.var_pb0dep_dn0, locals.var_pb0dep_dn2, locals.var_pb0dep_dn4, locals.var_pb0dep_dn5, locals.var_pb0dep_dn6, locals.var_pb0dep_dn7, locals.var_pb0dep_dn8, locals.var_pb0dep_dn9, locals.var_pb0dep_dn10, locals.var_pb0dep_dn11, locals.var_pb0dep_dn14,)
    }
};
        locals.var_pb0dep = assign42110_e56570;
        locals.var_pb0dep_dn0 = assign42110_e56570_d_n0;
        locals.var_pb0dep_dn2 = assign42110_e56570_d_n2;
        locals.var_pb0dep_dn4 = assign42110_e56570_d_n4;
        locals.var_pb0dep_dn5 = assign42110_e56570_d_n5;
        locals.var_pb0dep_dn6 = assign42110_e56570_d_n6;
        locals.var_pb0dep_dn7 = assign42110_e56570_d_n7;
        locals.var_pb0dep_dn8 = assign42110_e56570_d_n8;
        locals.var_pb0dep_dn9 = assign42110_e56570_d_n9;
        locals.var_pb0dep_dn10 = assign42110_e56570_d_n10;
        locals.var_pb0dep_dn11 = assign42110_e56570_d_n11;
        locals.var_pb0dep_dn14 = assign42110_e56570_d_n14;
        locals.var_pb0dep_rv = 0.0;

        let (assign42120_e56587, assign42120_e56587_d_n0, assign42120_e56587_d_n2, assign42120_e56587_d_n4, assign42120_e56587_d_n5, assign42120_e56587_d_n6, assign42120_e56587_d_n7, assign42120_e56587_d_n8, assign42120_e56587_d_n9, assign42120_e56587_d_n10, assign42120_e56587_d_n11, assign42120_e56587_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1053 == 0.0)) {
        let assign42120_e56585: f64 = (locals.var_t2 / locals.var_c_sb__blk1031);
        (assign42120_e56585, (((locals.var_t2_dn0 * locals.var_c_sb__blk1031) - (locals.var_t2 * locals.var_c_sb__blk1031_dn0)) / (locals.var_c_sb__blk1031 * locals.var_c_sb__blk1031)), (((locals.var_t2_dn2 * locals.var_c_sb__blk1031) - (locals.var_t2 * locals.var_c_sb__blk1031_dn2)) / (locals.var_c_sb__blk1031 * locals.var_c_sb__blk1031)), (((locals.var_t2_dn4 * locals.var_c_sb__blk1031) - (locals.var_t2 * locals.var_c_sb__blk1031_dn4)) / (locals.var_c_sb__blk1031 * locals.var_c_sb__blk1031)), (((locals.var_t2_dn5 * locals.var_c_sb__blk1031) - (locals.var_t2 * locals.var_c_sb__blk1031_dn5)) / (locals.var_c_sb__blk1031 * locals.var_c_sb__blk1031)), (((locals.var_t2_dn6 * locals.var_c_sb__blk1031) - (locals.var_t2 * locals.var_c_sb__blk1031_dn6)) / (locals.var_c_sb__blk1031 * locals.var_c_sb__blk1031)), (((locals.var_t2_dn7 * locals.var_c_sb__blk1031) - (locals.var_t2 * locals.var_c_sb__blk1031_dn7)) / (locals.var_c_sb__blk1031 * locals.var_c_sb__blk1031)), (((locals.var_t2_dn8 * locals.var_c_sb__blk1031) - (locals.var_t2 * locals.var_c_sb__blk1031_dn8)) / (locals.var_c_sb__blk1031 * locals.var_c_sb__blk1031)), (((locals.var_t2_dn9 * locals.var_c_sb__blk1031) - (locals.var_t2 * locals.var_c_sb__blk1031_dn9)) / (locals.var_c_sb__blk1031 * locals.var_c_sb__blk1031)), (((locals.var_t2_dn10 * locals.var_c_sb__blk1031) - (locals.var_t2 * locals.var_c_sb__blk1031_dn10)) / (locals.var_c_sb__blk1031 * locals.var_c_sb__blk1031)), (((locals.var_t2_dn11 * locals.var_c_sb__blk1031) - (locals.var_t2 * locals.var_c_sb__blk1031_dn11)) / (locals.var_c_sb__blk1031 * locals.var_c_sb__blk1031)), (((locals.var_t2_dn14 * locals.var_c_sb__blk1031) - (locals.var_t2 * locals.var_c_sb__blk1031_dn14)) / (locals.var_c_sb__blk1031 * locals.var_c_sb__blk1031)),)
    } else {
        (locals.var_pb0dep, locals.var_pb0dep_dn0, locals.var_pb0dep_dn2, locals.var_pb0dep_dn4, locals.var_pb0dep_dn5, locals.var_pb0dep_dn6, locals.var_pb0dep_dn7, locals.var_pb0dep_dn8, locals.var_pb0dep_dn9, locals.var_pb0dep_dn10, locals.var_pb0dep_dn11, locals.var_pb0dep_dn14,)
    }
};
        locals.var_pb0dep = assign42120_e56587;
        locals.var_pb0dep_dn0 = assign42120_e56587_d_n0;
        locals.var_pb0dep_dn2 = assign42120_e56587_d_n2;
        locals.var_pb0dep_dn4 = assign42120_e56587_d_n4;
        locals.var_pb0dep_dn5 = assign42120_e56587_d_n5;
        locals.var_pb0dep_dn6 = assign42120_e56587_d_n6;
        locals.var_pb0dep_dn7 = assign42120_e56587_d_n7;
        locals.var_pb0dep_dn8 = assign42120_e56587_d_n8;
        locals.var_pb0dep_dn9 = assign42120_e56587_d_n9;
        locals.var_pb0dep_dn10 = assign42120_e56587_d_n10;
        locals.var_pb0dep_dn11 = assign42120_e56587_d_n11;
        locals.var_pb0dep_dn14 = assign42120_e56587_d_n14;
        locals.var_pb0dep_rv = 0.0;

        let assign42130_e56590: f64 = (2.0 * 1.034943e-10);
        let assign42130_e56593: f64 = (locals.var_ps0dep - locals.var_pb0dep);
        let assign42130_e56594: f64 = (assign42130_e56590 * assign42130_e56593);
        let assign42130_e56596: f64 = (assign42130_e56594 / locals.var_q_ndepm__blk909);
        let assign42130_e56598: f64 = if assign42130_e56596 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1054 = assign42130_e56598;
        locals.var_guard1054_rv = 0.0;

        let (assign42140_e56612, assign42140_e56612_d_n0, assign42140_e56612_d_n2, assign42140_e56612_d_n4, assign42140_e56612_d_n5, assign42140_e56612_d_n6, assign42140_e56612_d_n7, assign42140_e56612_d_n8, assign42140_e56612_d_n9, assign42140_e56612_d_n10, assign42140_e56612_d_n11, assign42140_e56612_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1054 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ws, locals.var_ws_dn0, locals.var_ws_dn2, locals.var_ws_dn4, locals.var_ws_dn5, locals.var_ws_dn6, locals.var_ws_dn7, locals.var_ws_dn8, locals.var_ws_dn9, locals.var_ws_dn10, locals.var_ws_dn11, locals.var_ws_dn14,)
    }
};
        locals.var_ws = assign42140_e56612;
        locals.var_ws_dn0 = assign42140_e56612_d_n0;
        locals.var_ws_dn2 = assign42140_e56612_d_n2;
        locals.var_ws_dn4 = assign42140_e56612_d_n4;
        locals.var_ws_dn5 = assign42140_e56612_d_n5;
        locals.var_ws_dn6 = assign42140_e56612_d_n6;
        locals.var_ws_dn7 = assign42140_e56612_d_n7;
        locals.var_ws_dn8 = assign42140_e56612_d_n8;
        locals.var_ws_dn9 = assign42140_e56612_d_n9;
        locals.var_ws_dn10 = assign42140_e56612_d_n10;
        locals.var_ws_dn11 = assign42140_e56612_d_n11;
        locals.var_ws_dn14 = assign42140_e56612_d_n14;
        locals.var_ws_rv = 0.0;

        let (assign42150_e56636, assign42150_e56636_d_n0, assign42150_e56636_d_n2, assign42150_e56636_d_n4, assign42150_e56636_d_n5, assign42150_e56636_d_n6, assign42150_e56636_d_n7, assign42150_e56636_d_n8, assign42150_e56636_d_n9, assign42150_e56636_d_n10, assign42150_e56636_d_n11, assign42150_e56636_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) && (locals.var_guard1054 == 0.0)) {
        let assign42150_e56627: f64 = (2.0 * 1.034943e-10);
        let assign42150_e56630: f64 = (locals.var_ps0dep - locals.var_pb0dep);
        let assign42150_e56631: f64 = (assign42150_e56627 * assign42150_e56630);
        let assign42150_e56633: f64 = (assign42150_e56631 / locals.var_q_ndepm__blk909);
        let assign42150_e56634: f64 = (assign42150_e56633).sqrt();
        (assign42150_e56634, (((((assign42150_e56627 * (locals.var_ps0dep_dn0 - locals.var_pb0dep_dn0)) * locals.var_q_ndepm__blk909) - (assign42150_e56631 * locals.var_q_ndepm__blk909_dn0)) / (locals.var_q_ndepm__blk909 * locals.var_q_ndepm__blk909)) / (2.0 * assign42150_e56634)), (((((assign42150_e56627 * (locals.var_ps0dep_dn2 - locals.var_pb0dep_dn2)) * locals.var_q_ndepm__blk909) - (assign42150_e56631 * locals.var_q_ndepm__blk909_dn2)) / (locals.var_q_ndepm__blk909 * locals.var_q_ndepm__blk909)) / (2.0 * assign42150_e56634)), (((((assign42150_e56627 * (locals.var_ps0dep_dn4 - locals.var_pb0dep_dn4)) * locals.var_q_ndepm__blk909) - (assign42150_e56631 * locals.var_q_ndepm__blk909_dn4)) / (locals.var_q_ndepm__blk909 * locals.var_q_ndepm__blk909)) / (2.0 * assign42150_e56634)), (((((assign42150_e56627 * (locals.var_ps0dep_dn5 - locals.var_pb0dep_dn5)) * locals.var_q_ndepm__blk909) - (assign42150_e56631 * locals.var_q_ndepm__blk909_dn5)) / (locals.var_q_ndepm__blk909 * locals.var_q_ndepm__blk909)) / (2.0 * assign42150_e56634)), (((((assign42150_e56627 * (locals.var_ps0dep_dn6 - locals.var_pb0dep_dn6)) * locals.var_q_ndepm__blk909) - (assign42150_e56631 * locals.var_q_ndepm__blk909_dn6)) / (locals.var_q_ndepm__blk909 * locals.var_q_ndepm__blk909)) / (2.0 * assign42150_e56634)), (((((assign42150_e56627 * (locals.var_ps0dep_dn7 - locals.var_pb0dep_dn7)) * locals.var_q_ndepm__blk909) - (assign42150_e56631 * locals.var_q_ndepm__blk909_dn7)) / (locals.var_q_ndepm__blk909 * locals.var_q_ndepm__blk909)) / (2.0 * assign42150_e56634)), (((((assign42150_e56627 * (locals.var_ps0dep_dn8 - locals.var_pb0dep_dn8)) * locals.var_q_ndepm__blk909) - (assign42150_e56631 * locals.var_q_ndepm__blk909_dn8)) / (locals.var_q_ndepm__blk909 * locals.var_q_ndepm__blk909)) / (2.0 * assign42150_e56634)), (((((assign42150_e56627 * (locals.var_ps0dep_dn9 - locals.var_pb0dep_dn9)) * locals.var_q_ndepm__blk909) - (assign42150_e56631 * locals.var_q_ndepm__blk909_dn9)) / (locals.var_q_ndepm__blk909 * locals.var_q_ndepm__blk909)) / (2.0 * assign42150_e56634)), (((((assign42150_e56627 * (locals.var_ps0dep_dn10 - locals.var_pb0dep_dn10)) * locals.var_q_ndepm__blk909) - (assign42150_e56631 * locals.var_q_ndepm__blk909_dn10)) / (locals.var_q_ndepm__blk909 * locals.var_q_ndepm__blk909)) / (2.0 * assign42150_e56634)), (((((assign42150_e56627 * (locals.var_ps0dep_dn11 - locals.var_pb0dep_dn11)) * locals.var_q_ndepm__blk909) - (assign42150_e56631 * locals.var_q_ndepm__blk909_dn11)) / (locals.var_q_ndepm__blk909 * locals.var_q_ndepm__blk909)) / (2.0 * assign42150_e56634)), (((((assign42150_e56627 * (locals.var_ps0dep_dn14 - locals.var_pb0dep_dn14)) * locals.var_q_ndepm__blk909) - (assign42150_e56631 * locals.var_q_ndepm__blk909_dn14)) / (locals.var_q_ndepm__blk909 * locals.var_q_ndepm__blk909)) / (2.0 * assign42150_e56634)),)
    } else {
        (locals.var_ws, locals.var_ws_dn0, locals.var_ws_dn2, locals.var_ws_dn4, locals.var_ws_dn5, locals.var_ws_dn6, locals.var_ws_dn7, locals.var_ws_dn8, locals.var_ws_dn9, locals.var_ws_dn10, locals.var_ws_dn11, locals.var_ws_dn14,)
    }
};
        locals.var_ws = assign42150_e56636;
        locals.var_ws_dn0 = assign42150_e56636_d_n0;
        locals.var_ws_dn2 = assign42150_e56636_d_n2;
        locals.var_ws_dn4 = assign42150_e56636_d_n4;
        locals.var_ws_dn5 = assign42150_e56636_d_n5;
        locals.var_ws_dn6 = assign42150_e56636_d_n6;
        locals.var_ws_dn7 = assign42150_e56636_d_n7;
        locals.var_ws_dn8 = assign42150_e56636_d_n8;
        locals.var_ws_dn9 = assign42150_e56636_d_n9;
        locals.var_ws_dn10 = assign42150_e56636_d_n10;
        locals.var_ws_dn11 = assign42150_e56636_d_n11;
        locals.var_ws_dn14 = assign42150_e56636_d_n14;
        locals.var_ws_rv = 0.0;

        let (assign42160_e56653, assign42160_e56653_d_n0, assign42160_e56653_d_n2, assign42160_e56653_d_n4, assign42160_e56653_d_n5, assign42160_e56653_d_n6, assign42160_e56653_d_n7, assign42160_e56653_d_n8, assign42160_e56653_d_n9, assign42160_e56653_d_n10, assign42160_e56653_d_n11, assign42160_e56653_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1034 == 0.0)) {
        let (assign42160_e56651, assign42160_e56651_d_n0, assign42160_e56651_d_n2, assign42160_e56651_d_n4, assign42160_e56651_d_n5, assign42160_e56651_d_n6, assign42160_e56651_d_n7, assign42160_e56651_d_n8, assign42160_e56651_d_n9, assign42160_e56651_d_n10, assign42160_e56651_d_n11, assign42160_e56651_d_n14,) = {
            if (locals.var_ws > locals.var_tnp) {
                (locals.var_tnp, locals.var_tnp_dn0, locals.var_tnp_dn2, locals.var_tnp_dn4, locals.var_tnp_dn5, locals.var_tnp_dn6, locals.var_tnp_dn7, locals.var_tnp_dn8, locals.var_tnp_dn9, locals.var_tnp_dn10, locals.var_tnp_dn11, locals.var_tnp_dn14,)
            } else {
                (locals.var_ws, locals.var_ws_dn0, locals.var_ws_dn2, locals.var_ws_dn4, locals.var_ws_dn5, locals.var_ws_dn6, locals.var_ws_dn7, locals.var_ws_dn8, locals.var_ws_dn9, locals.var_ws_dn10, locals.var_ws_dn11, locals.var_ws_dn14,)
            }
        };
        (assign42160_e56651, assign42160_e56651_d_n0, assign42160_e56651_d_n2, assign42160_e56651_d_n4, assign42160_e56651_d_n5, assign42160_e56651_d_n6, assign42160_e56651_d_n7, assign42160_e56651_d_n8, assign42160_e56651_d_n9, assign42160_e56651_d_n10, assign42160_e56651_d_n11, assign42160_e56651_d_n14,)
    } else {
        (locals.var_ws, locals.var_ws_dn0, locals.var_ws_dn2, locals.var_ws_dn4, locals.var_ws_dn5, locals.var_ws_dn6, locals.var_ws_dn7, locals.var_ws_dn8, locals.var_ws_dn9, locals.var_ws_dn10, locals.var_ws_dn11, locals.var_ws_dn14,)
    }
};
        locals.var_ws = assign42160_e56653;
        locals.var_ws_dn0 = assign42160_e56653_d_n0;
        locals.var_ws_dn2 = assign42160_e56653_d_n2;
        locals.var_ws_dn4 = assign42160_e56653_d_n4;
        locals.var_ws_dn5 = assign42160_e56653_d_n5;
        locals.var_ws_dn6 = assign42160_e56653_d_n6;
        locals.var_ws_dn7 = assign42160_e56653_d_n7;
        locals.var_ws_dn8 = assign42160_e56653_d_n8;
        locals.var_ws_dn9 = assign42160_e56653_d_n9;
        locals.var_ws_dn10 = assign42160_e56653_d_n10;
        locals.var_ws_dn11 = assign42160_e56653_d_n11;
        locals.var_ws_dn14 = assign42160_e56653_d_n14;
        locals.var_ws_rv = 0.0;

        let assign42170_e56656: f64 = if locals.var_ws < locals.var_tnp { 1.0 } else { 0.0 };
        locals.var_guard1055 = assign42170_e56656;
        locals.var_guard1055_rv = 0.0;

        let (assign42180_e56669, assign42180_e56669_d_n0, assign42180_e56669_d_n2, assign42180_e56669_d_n4, assign42180_e56669_d_n5, assign42180_e56669_d_n6, assign42180_e56669_d_n7, assign42180_e56669_d_n8, assign42180_e56669_d_n9, assign42180_e56669_d_n10, assign42180_e56669_d_n11, assign42180_e56669_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1055 != 0.0)) {
        let assign42180_e56667: f64 = (locals.var_tnp - locals.var_ws);
        (assign42180_e56667, (locals.var_tnp_dn0 - locals.var_ws_dn0), (locals.var_tnp_dn2 - locals.var_ws_dn2), (locals.var_tnp_dn4 - locals.var_ws_dn4), (locals.var_tnp_dn5 - locals.var_ws_dn5), (locals.var_tnp_dn6 - locals.var_ws_dn6), (locals.var_tnp_dn7 - locals.var_ws_dn7), (locals.var_tnp_dn8 - locals.var_ws_dn8), (locals.var_tnp_dn9 - locals.var_ws_dn9), (locals.var_tnp_dn10 - locals.var_ws_dn10), (locals.var_tnp_dn11 - locals.var_ws_dn11), (locals.var_tnp_dn14 - locals.var_ws_dn14),)
    } else {
        (locals.var_w_res, locals.var_w_res_dn0, locals.var_w_res_dn2, locals.var_w_res_dn4, locals.var_w_res_dn5, locals.var_w_res_dn6, locals.var_w_res_dn7, locals.var_w_res_dn8, locals.var_w_res_dn9, locals.var_w_res_dn10, locals.var_w_res_dn11, locals.var_w_res_dn14,)
    }
};
        locals.var_w_res = assign42180_e56669;
        locals.var_w_res_dn0 = assign42180_e56669_d_n0;
        locals.var_w_res_dn2 = assign42180_e56669_d_n2;
        locals.var_w_res_dn4 = assign42180_e56669_d_n4;
        locals.var_w_res_dn5 = assign42180_e56669_d_n5;
        locals.var_w_res_dn6 = assign42180_e56669_d_n6;
        locals.var_w_res_dn7 = assign42180_e56669_d_n7;
        locals.var_w_res_dn8 = assign42180_e56669_d_n8;
        locals.var_w_res_dn9 = assign42180_e56669_d_n9;
        locals.var_w_res_dn10 = assign42180_e56669_d_n10;
        locals.var_w_res_dn11 = assign42180_e56669_d_n11;
        locals.var_w_res_dn14 = assign42180_e56669_d_n14;
        locals.var_w_res_rv = 0.0;

        let (assign42190_e56681, assign42190_e56681_d_n0, assign42190_e56681_d_n2, assign42190_e56681_d_n4, assign42190_e56681_d_n5, assign42190_e56681_d_n6, assign42190_e56681_d_n7, assign42190_e56681_d_n8, assign42190_e56681_d_n9, assign42190_e56681_d_n10, assign42190_e56681_d_n11, assign42190_e56681_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1055 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_w_res, locals.var_w_res_dn0, locals.var_w_res_dn2, locals.var_w_res_dn4, locals.var_w_res_dn5, locals.var_w_res_dn6, locals.var_w_res_dn7, locals.var_w_res_dn8, locals.var_w_res_dn9, locals.var_w_res_dn10, locals.var_w_res_dn11, locals.var_w_res_dn14,)
    }
};
        locals.var_w_res = assign42190_e56681;
        locals.var_w_res_dn0 = assign42190_e56681_d_n0;
        locals.var_w_res_dn2 = assign42190_e56681_d_n2;
        locals.var_w_res_dn4 = assign42190_e56681_d_n4;
        locals.var_w_res_dn5 = assign42190_e56681_d_n5;
        locals.var_w_res_dn6 = assign42190_e56681_d_n6;
        locals.var_w_res_dn7 = assign42190_e56681_d_n7;
        locals.var_w_res_dn8 = assign42190_e56681_d_n8;
        locals.var_w_res_dn9 = assign42190_e56681_d_n9;
        locals.var_w_res_dn10 = assign42190_e56681_d_n10;
        locals.var_w_res_dn11 = assign42190_e56681_d_n11;
        locals.var_w_res_dn14 = assign42190_e56681_d_n14;
        locals.var_w_res_rv = 0.0;

        let (assign42200_e56693, assign42200_e56693_d_n0, assign42200_e56693_d_n2, assign42200_e56693_d_n4, assign42200_e56693_d_n5, assign42200_e56693_d_n6, assign42200_e56693_d_n7, assign42200_e56693_d_n8, assign42200_e56693_d_n9, assign42200_e56693_d_n10, assign42200_e56693_d_n11, assign42200_e56693_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        let assign42200_e56690: f64 = (locals.var_q_n0_cur__blk893 + locals.var_q_nl_cur__blk894);
        let assign42200_e56691: f64 = (-assign42200_e56690);
        (assign42200_e56691, (-(locals.var_q_n0_cur__blk893_dn0 + locals.var_q_nl_cur__blk894_dn0)), (-(locals.var_q_n0_cur__blk893_dn2 + locals.var_q_nl_cur__blk894_dn2)), (-(locals.var_q_n0_cur__blk893_dn4 + locals.var_q_nl_cur__blk894_dn4)), (-(locals.var_q_n0_cur__blk893_dn5 + locals.var_q_nl_cur__blk894_dn5)), (-(locals.var_q_n0_cur__blk893_dn6 + locals.var_q_nl_cur__blk894_dn6)), (-(locals.var_q_n0_cur__blk893_dn7 + locals.var_q_nl_cur__blk894_dn7)), (-(locals.var_q_n0_cur__blk893_dn8 + locals.var_q_nl_cur__blk894_dn8)), (-(locals.var_q_n0_cur__blk893_dn9 + locals.var_q_nl_cur__blk894_dn9)), (-(locals.var_q_n0_cur__blk893_dn10 + locals.var_q_nl_cur__blk894_dn10)), (-(locals.var_q_n0_cur__blk893_dn11 + locals.var_q_nl_cur__blk894_dn11)), (-(locals.var_q_n0_cur__blk893_dn14 + locals.var_q_nl_cur__blk894_dn14)),)
    } else {
        (locals.var_qn_drift__blk898, locals.var_qn_drift__blk898_dn0, locals.var_qn_drift__blk898_dn2, locals.var_qn_drift__blk898_dn4, locals.var_qn_drift__blk898_dn5, locals.var_qn_drift__blk898_dn6, locals.var_qn_drift__blk898_dn7, locals.var_qn_drift__blk898_dn8, locals.var_qn_drift__blk898_dn9, locals.var_qn_drift__blk898_dn10, locals.var_qn_drift__blk898_dn11, locals.var_qn_drift__blk898_dn14,)
    }
};
        locals.var_qn_drift__blk898 = assign42200_e56693;
        locals.var_qn_drift__blk898_dn0 = assign42200_e56693_d_n0;
        locals.var_qn_drift__blk898_dn2 = assign42200_e56693_d_n2;
        locals.var_qn_drift__blk898_dn4 = assign42200_e56693_d_n4;
        locals.var_qn_drift__blk898_dn5 = assign42200_e56693_d_n5;
        locals.var_qn_drift__blk898_dn6 = assign42200_e56693_d_n6;
        locals.var_qn_drift__blk898_dn7 = assign42200_e56693_d_n7;
        locals.var_qn_drift__blk898_dn8 = assign42200_e56693_d_n8;
        locals.var_qn_drift__blk898_dn9 = assign42200_e56693_d_n9;
        locals.var_qn_drift__blk898_dn10 = assign42200_e56693_d_n10;
        locals.var_qn_drift__blk898_dn11 = assign42200_e56693_d_n11;
        locals.var_qn_drift__blk898_dn14 = assign42200_e56693_d_n14;
        locals.var_qn_drift__blk898_rv = 0.0;

        let assign42210_e56696: f64 = if locals.var_pds < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1056 = assign42210_e56696;
        locals.var_guard1056_rv = 0.0;

        let (assign42220_e56707, assign42220_e56707_d_n0, assign42220_e56707_d_n2, assign42220_e56707_d_n4, assign42220_e56707_d_n5, assign42220_e56707_d_n6, assign42220_e56707_d_n7, assign42220_e56707_d_n8, assign42220_e56707_d_n9, assign42220_e56707_d_n10, assign42220_e56707_d_n11, assign42220_e56707_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1056 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pds, locals.var_pds_dn0, locals.var_pds_dn2, locals.var_pds_dn4, locals.var_pds_dn5, locals.var_pds_dn6, locals.var_pds_dn7, locals.var_pds_dn8, locals.var_pds_dn9, locals.var_pds_dn10, locals.var_pds_dn11, locals.var_pds_dn14,)
    }
};
        locals.var_pds = assign42220_e56707;
        locals.var_pds_dn0 = assign42220_e56707_d_n0;
        locals.var_pds_dn2 = assign42220_e56707_d_n2;
        locals.var_pds_dn4 = assign42220_e56707_d_n4;
        locals.var_pds_dn5 = assign42220_e56707_d_n5;
        locals.var_pds_dn6 = assign42220_e56707_d_n6;
        locals.var_pds_dn7 = assign42220_e56707_d_n7;
        locals.var_pds_dn8 = assign42220_e56707_d_n8;
        locals.var_pds_dn9 = assign42220_e56707_d_n9;
        locals.var_pds_dn10 = assign42220_e56707_d_n10;
        locals.var_pds_dn11 = assign42220_e56707_d_n11;
        locals.var_pds_dn14 = assign42220_e56707_d_n14;
        locals.var_pds_rv = 0.0;

        let (assign42230_e56718, assign42230_e56718_d_n0, assign42230_e56718_d_n2, assign42230_e56718_d_n4, assign42230_e56718_d_n5, assign42230_e56718_d_n6, assign42230_e56718_d_n7, assign42230_e56718_d_n8, assign42230_e56718_d_n9, assign42230_e56718_d_n10, assign42230_e56718_d_n11, assign42230_e56718_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1056 != 0.0)) {
        (locals.var_phi_s0_dep__blk857, locals.var_phi_s0_dep__blk857_dn0, locals.var_phi_s0_dep__blk857_dn2, locals.var_phi_s0_dep__blk857_dn4, locals.var_phi_s0_dep__blk857_dn5, locals.var_phi_s0_dep__blk857_dn6, locals.var_phi_s0_dep__blk857_dn7, locals.var_phi_s0_dep__blk857_dn8, locals.var_phi_s0_dep__blk857_dn9, locals.var_phi_s0_dep__blk857_dn10, locals.var_phi_s0_dep__blk857_dn11, locals.var_phi_s0_dep__blk857_dn14,)
    } else {
        (locals.var_phi_sl_dep__blk858, locals.var_phi_sl_dep__blk858_dn0, locals.var_phi_sl_dep__blk858_dn2, locals.var_phi_sl_dep__blk858_dn4, locals.var_phi_sl_dep__blk858_dn5, locals.var_phi_sl_dep__blk858_dn6, locals.var_phi_sl_dep__blk858_dn7, locals.var_phi_sl_dep__blk858_dn8, locals.var_phi_sl_dep__blk858_dn9, locals.var_phi_sl_dep__blk858_dn10, locals.var_phi_sl_dep__blk858_dn11, locals.var_phi_sl_dep__blk858_dn14,)
    }
};
        locals.var_phi_sl_dep__blk858 = assign42230_e56718;
        locals.var_phi_sl_dep__blk858_dn0 = assign42230_e56718_d_n0;
        locals.var_phi_sl_dep__blk858_dn2 = assign42230_e56718_d_n2;
        locals.var_phi_sl_dep__blk858_dn4 = assign42230_e56718_d_n4;
        locals.var_phi_sl_dep__blk858_dn5 = assign42230_e56718_d_n5;
        locals.var_phi_sl_dep__blk858_dn6 = assign42230_e56718_d_n6;
        locals.var_phi_sl_dep__blk858_dn7 = assign42230_e56718_d_n7;
        locals.var_phi_sl_dep__blk858_dn8 = assign42230_e56718_d_n8;
        locals.var_phi_sl_dep__blk858_dn9 = assign42230_e56718_d_n9;
        locals.var_phi_sl_dep__blk858_dn10 = assign42230_e56718_d_n10;
        locals.var_phi_sl_dep__blk858_dn11 = assign42230_e56718_d_n11;
        locals.var_phi_sl_dep__blk858_dn14 = assign42230_e56718_d_n14;
        locals.var_phi_sl_dep__blk858_rv = 0.0;

        let (assign42240_e56729, assign42240_e56729_d_n0, assign42240_e56729_d_n2, assign42240_e56729_d_n4, assign42240_e56729_d_n5, assign42240_e56729_d_n6, assign42240_e56729_d_n7, assign42240_e56729_d_n8, assign42240_e56729_d_n9, assign42240_e56729_d_n10, assign42240_e56729_d_n11, assign42240_e56729_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1056 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_idd, locals.var_idd_dn0, locals.var_idd_dn2, locals.var_idd_dn4, locals.var_idd_dn5, locals.var_idd_dn6, locals.var_idd_dn7, locals.var_idd_dn8, locals.var_idd_dn9, locals.var_idd_dn10, locals.var_idd_dn11, locals.var_idd_dn14,)
    }
};
        locals.var_idd = assign42240_e56729;
        locals.var_idd_dn0 = assign42240_e56729_d_n0;
        locals.var_idd_dn2 = assign42240_e56729_d_n2;
        locals.var_idd_dn4 = assign42240_e56729_d_n4;
        locals.var_idd_dn5 = assign42240_e56729_d_n5;
        locals.var_idd_dn6 = assign42240_e56729_d_n6;
        locals.var_idd_dn7 = assign42240_e56729_d_n7;
        locals.var_idd_dn8 = assign42240_e56729_d_n8;
        locals.var_idd_dn9 = assign42240_e56729_d_n9;
        locals.var_idd_dn10 = assign42240_e56729_d_n10;
        locals.var_idd_dn11 = assign42240_e56729_d_n11;
        locals.var_idd_dn14 = assign42240_e56729_d_n14;
        locals.var_idd_rv = 0.0;

        let (assign42250_e56747, assign42250_e56747_d_n0, assign42250_e56747_d_n2, assign42250_e56747_d_n4, assign42250_e56747_d_n5, assign42250_e56747_d_n6, assign42250_e56747_d_n7, assign42250_e56747_d_n8, assign42250_e56747_d_n9, assign42250_e56747_d_n10, assign42250_e56747_d_n11, assign42250_e56747_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1056 == 0.0)) {
        let assign42250_e56741: f64 = (locals.var_beta * locals.var_qn_drift__blk898);
        let assign42250_e56743: f64 = (assign42250_e56741 / 2.0);
        let assign42250_e56745: f64 = (assign42250_e56743 * locals.var_pds);
        (assign42250_e56745, (((((locals.var_beta_dn0 * locals.var_qn_drift__blk898) + (locals.var_beta * locals.var_qn_drift__blk898_dn0)) / 2.0) * locals.var_pds) + (assign42250_e56743 * locals.var_pds_dn0)), (((((locals.var_beta_dn2 * locals.var_qn_drift__blk898) + (locals.var_beta * locals.var_qn_drift__blk898_dn2)) / 2.0) * locals.var_pds) + (assign42250_e56743 * locals.var_pds_dn2)), (((((locals.var_beta_dn4 * locals.var_qn_drift__blk898) + (locals.var_beta * locals.var_qn_drift__blk898_dn4)) / 2.0) * locals.var_pds) + (assign42250_e56743 * locals.var_pds_dn4)), (((((locals.var_beta_dn5 * locals.var_qn_drift__blk898) + (locals.var_beta * locals.var_qn_drift__blk898_dn5)) / 2.0) * locals.var_pds) + (assign42250_e56743 * locals.var_pds_dn5)), (((((locals.var_beta_dn6 * locals.var_qn_drift__blk898) + (locals.var_beta * locals.var_qn_drift__blk898_dn6)) / 2.0) * locals.var_pds) + (assign42250_e56743 * locals.var_pds_dn6)), (((((locals.var_beta_dn7 * locals.var_qn_drift__blk898) + (locals.var_beta * locals.var_qn_drift__blk898_dn7)) / 2.0) * locals.var_pds) + (assign42250_e56743 * locals.var_pds_dn7)), (((((locals.var_beta_dn8 * locals.var_qn_drift__blk898) + (locals.var_beta * locals.var_qn_drift__blk898_dn8)) / 2.0) * locals.var_pds) + (assign42250_e56743 * locals.var_pds_dn8)), (((((locals.var_beta_dn9 * locals.var_qn_drift__blk898) + (locals.var_beta * locals.var_qn_drift__blk898_dn9)) / 2.0) * locals.var_pds) + (assign42250_e56743 * locals.var_pds_dn9)), (((((locals.var_beta_dn10 * locals.var_qn_drift__blk898) + (locals.var_beta * locals.var_qn_drift__blk898_dn10)) / 2.0) * locals.var_pds) + (assign42250_e56743 * locals.var_pds_dn10)), (((((locals.var_beta_dn11 * locals.var_qn_drift__blk898) + (locals.var_beta * locals.var_qn_drift__blk898_dn11)) / 2.0) * locals.var_pds) + (assign42250_e56743 * locals.var_pds_dn11)), (((((locals.var_beta_dn14 * locals.var_qn_drift__blk898) + (locals.var_beta * locals.var_qn_drift__blk898_dn14)) / 2.0) * locals.var_pds) + (assign42250_e56743 * locals.var_pds_dn14)),)
    } else {
        (locals.var_idd, locals.var_idd_dn0, locals.var_idd_dn2, locals.var_idd_dn4, locals.var_idd_dn5, locals.var_idd_dn6, locals.var_idd_dn7, locals.var_idd_dn8, locals.var_idd_dn9, locals.var_idd_dn10, locals.var_idd_dn11, locals.var_idd_dn14,)
    }
};
        locals.var_idd = assign42250_e56747;
        locals.var_idd_dn0 = assign42250_e56747_d_n0;
        locals.var_idd_dn2 = assign42250_e56747_d_n2;
        locals.var_idd_dn4 = assign42250_e56747_d_n4;
        locals.var_idd_dn5 = assign42250_e56747_d_n5;
        locals.var_idd_dn6 = assign42250_e56747_d_n6;
        locals.var_idd_dn7 = assign42250_e56747_d_n7;
        locals.var_idd_dn8 = assign42250_e56747_d_n8;
        locals.var_idd_dn9 = assign42250_e56747_d_n9;
        locals.var_idd_dn10 = assign42250_e56747_d_n10;
        locals.var_idd_dn11 = assign42250_e56747_d_n11;
        locals.var_idd_dn14 = assign42250_e56747_d_n14;
        locals.var_idd_rv = 0.0;

        let (assign42260_e56764, assign42260_e56764_d_n0, assign42260_e56764_d_n2, assign42260_e56764_d_n4, assign42260_e56764_d_n5, assign42260_e56764_d_n6, assign42260_e56764_d_n7, assign42260_e56764_d_n8, assign42260_e56764_d_n9, assign42260_e56764_d_n10, assign42260_e56764_d_n11, assign42260_e56764_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1056 == 0.0)) {
        let (assign42260_e56762, assign42260_e56762_d_n0, assign42260_e56762_d_n2, assign42260_e56762_d_n4, assign42260_e56762_d_n5, assign42260_e56762_d_n6, assign42260_e56762_d_n7, assign42260_e56762_d_n8, assign42260_e56762_d_n9, assign42260_e56762_d_n10, assign42260_e56762_d_n11, assign42260_e56762_d_n14,) = {
            if (locals.var_idd < 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                (locals.var_idd, locals.var_idd_dn0, locals.var_idd_dn2, locals.var_idd_dn4, locals.var_idd_dn5, locals.var_idd_dn6, locals.var_idd_dn7, locals.var_idd_dn8, locals.var_idd_dn9, locals.var_idd_dn10, locals.var_idd_dn11, locals.var_idd_dn14,)
            }
        };
        (assign42260_e56762, assign42260_e56762_d_n0, assign42260_e56762_d_n2, assign42260_e56762_d_n4, assign42260_e56762_d_n5, assign42260_e56762_d_n6, assign42260_e56762_d_n7, assign42260_e56762_d_n8, assign42260_e56762_d_n9, assign42260_e56762_d_n10, assign42260_e56762_d_n11, assign42260_e56762_d_n14,)
    } else {
        (locals.var_idd, locals.var_idd_dn0, locals.var_idd_dn2, locals.var_idd_dn4, locals.var_idd_dn5, locals.var_idd_dn6, locals.var_idd_dn7, locals.var_idd_dn8, locals.var_idd_dn9, locals.var_idd_dn10, locals.var_idd_dn11, locals.var_idd_dn14,)
    }
};
        locals.var_idd = assign42260_e56764;
        locals.var_idd_dn0 = assign42260_e56764_d_n0;
        locals.var_idd_dn2 = assign42260_e56764_d_n2;
        locals.var_idd_dn4 = assign42260_e56764_d_n4;
        locals.var_idd_dn5 = assign42260_e56764_d_n5;
        locals.var_idd_dn6 = assign42260_e56764_d_n6;
        locals.var_idd_dn7 = assign42260_e56764_d_n7;
        locals.var_idd_dn8 = assign42260_e56764_d_n8;
        locals.var_idd_dn9 = assign42260_e56764_d_n9;
        locals.var_idd_dn10 = assign42260_e56764_d_n10;
        locals.var_idd_dn11 = assign42260_e56764_d_n11;
        locals.var_idd_dn14 = assign42260_e56764_d_n14;
        locals.var_idd_rv = 0.0;

        let (assign42270_e56774, assign42270_e56774_d_n0, assign42270_e56774_d_n2, assign42270_e56774_d_n4, assign42270_e56774_d_n5, assign42270_e56774_d_n6, assign42270_e56774_d_n7, assign42270_e56774_d_n8, assign42270_e56774_d_n9, assign42270_e56774_d_n10, assign42270_e56774_d_n11, assign42270_e56774_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        let assign42270_e56772: f64 = (-locals.var_q_n0_sym);
        (assign42270_e56772, (-locals.var_q_n0_sym_dn0), (-locals.var_q_n0_sym_dn2), (-locals.var_q_n0_sym_dn4), (-locals.var_q_n0_sym_dn5), (-locals.var_q_n0_sym_dn6), (-locals.var_q_n0_sym_dn7), (-locals.var_q_n0_sym_dn8), (-locals.var_q_n0_sym_dn9), (-locals.var_q_n0_sym_dn10), (-locals.var_q_n0_sym_dn11), (-locals.var_q_n0_sym_dn14),)
    } else {
        (locals.var_qn0, locals.var_qn0_dn0, locals.var_qn0_dn2, locals.var_qn0_dn4, locals.var_qn0_dn5, locals.var_qn0_dn6, locals.var_qn0_dn7, locals.var_qn0_dn8, locals.var_qn0_dn9, locals.var_qn0_dn10, locals.var_qn0_dn11, locals.var_qn0_dn14,)
    }
};
        locals.var_qn0 = assign42270_e56774;
        locals.var_qn0_dn0 = assign42270_e56774_d_n0;
        locals.var_qn0_dn2 = assign42270_e56774_d_n2;
        locals.var_qn0_dn4 = assign42270_e56774_d_n4;
        locals.var_qn0_dn5 = assign42270_e56774_d_n5;
        locals.var_qn0_dn6 = assign42270_e56774_d_n6;
        locals.var_qn0_dn7 = assign42270_e56774_d_n7;
        locals.var_qn0_dn8 = assign42270_e56774_d_n8;
        locals.var_qn0_dn9 = assign42270_e56774_d_n9;
        locals.var_qn0_dn10 = assign42270_e56774_d_n10;
        locals.var_qn0_dn11 = assign42270_e56774_d_n11;
        locals.var_qn0_dn14 = assign42270_e56774_d_n14;
        locals.var_qn0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_146(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign42280_e56783, assign42280_e56783_d_n0, assign42280_e56783_d_n2, assign42280_e56783_d_n4, assign42280_e56783_d_n5, assign42280_e56783_d_n6, assign42280_e56783_d_n7, assign42280_e56783_d_n8, assign42280_e56783_d_n9, assign42280_e56783_d_n10, assign42280_e56783_d_n11, assign42280_e56783_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        (locals.var_leff, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lch, locals.var_lch_dn0, locals.var_lch_dn2, locals.var_lch_dn4, locals.var_lch_dn5, locals.var_lch_dn6, locals.var_lch_dn7, locals.var_lch_dn8, locals.var_lch_dn9, locals.var_lch_dn10, locals.var_lch_dn11, locals.var_lch_dn14,)
    }
};
        locals.var_lch = assign42280_e56783;
        locals.var_lch_dn0 = assign42280_e56783_d_n0;
        locals.var_lch_dn2 = assign42280_e56783_d_n2;
        locals.var_lch_dn4 = assign42280_e56783_d_n4;
        locals.var_lch_dn5 = assign42280_e56783_d_n5;
        locals.var_lch_dn6 = assign42280_e56783_d_n6;
        locals.var_lch_dn7 = assign42280_e56783_d_n7;
        locals.var_lch_dn8 = assign42280_e56783_d_n8;
        locals.var_lch_dn9 = assign42280_e56783_d_n9;
        locals.var_lch_dn10 = assign42280_e56783_d_n10;
        locals.var_lch_dn11 = assign42280_e56783_d_n11;
        locals.var_lch_dn14 = assign42280_e56783_d_n14;
        locals.var_lch_rv = 0.0;

        let (assign42290_e56794, assign42290_e56794_d_n0, assign42290_e56794_d_n2, assign42290_e56794_d_n4, assign42290_e56794_d_n5, assign42290_e56794_d_n6, assign42290_e56794_d_n7, assign42290_e56794_d_n8, assign42290_e56794_d_n9, assign42290_e56794_d_n10, assign42290_e56794_d_n11, assign42290_e56794_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        let assign42290_e56792: f64 = (locals.var_ninv_o_esi / 100.0);
        (assign42290_e56792, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign42290_e56794;
        locals.var_t2_dn0 = assign42290_e56794_d_n0;
        locals.var_t2_dn2 = assign42290_e56794_d_n2;
        locals.var_t2_dn4 = assign42290_e56794_d_n4;
        locals.var_t2_dn5 = assign42290_e56794_d_n5;
        locals.var_t2_dn6 = assign42290_e56794_d_n6;
        locals.var_t2_dn7 = assign42290_e56794_d_n7;
        locals.var_t2_dn8 = assign42290_e56794_d_n8;
        locals.var_t2_dn9 = assign42290_e56794_d_n9;
        locals.var_t2_dn10 = assign42290_e56794_d_n10;
        locals.var_t2_dn11 = assign42290_e56794_d_n11;
        locals.var_t2_dn14 = assign42290_e56794_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign42300_e56803, assign42300_e56803_d_n0, assign42300_e56803_d_n2, assign42300_e56803_d_n4, assign42300_e56803_d_n5, assign42300_e56803_d_n6, assign42300_e56803_d_n7, assign42300_e56803_d_n8, assign42300_e56803_d_n9, assign42300_e56803_d_n10, assign42300_e56803_d_n11, assign42300_e56803_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        (locals.var_ninvde, locals.var_ninvde_dn0, locals.var_ninvde_dn2, locals.var_ninvde_dn4, locals.var_ninvde_dn5, locals.var_ninvde_dn6, locals.var_ninvde_dn7, locals.var_ninvde_dn8, locals.var_ninvde_dn9, locals.var_ninvde_dn10, locals.var_ninvde_dn11, locals.var_ninvde_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign42300_e56803;
        locals.var_t0_dn0 = assign42300_e56803_d_n0;
        locals.var_t0_dn2 = assign42300_e56803_d_n2;
        locals.var_t0_dn4 = assign42300_e56803_d_n4;
        locals.var_t0_dn5 = assign42300_e56803_d_n5;
        locals.var_t0_dn6 = assign42300_e56803_d_n6;
        locals.var_t0_dn7 = assign42300_e56803_d_n7;
        locals.var_t0_dn8 = assign42300_e56803_d_n8;
        locals.var_t0_dn9 = assign42300_e56803_d_n9;
        locals.var_t0_dn10 = assign42300_e56803_d_n10;
        locals.var_t0_dn11 = assign42300_e56803_d_n11;
        locals.var_t0_dn14 = assign42300_e56803_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign42310_e56820, assign42310_e56820_d_n0, assign42310_e56820_d_n2, assign42310_e56820_d_n4, assign42310_e56820_d_n5, assign42310_e56820_d_n6, assign42310_e56820_d_n7, assign42310_e56820_d_n8, assign42310_e56820_d_n9, assign42310_e56820_d_n10, assign42310_e56820_d_n11, assign42310_e56820_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        let assign42310_e56812: f64 = (locals.var_pds * locals.var_pds);
        let assign42310_e56814: f64 = (assign42310_e56812 + p.p262);
        let assign42310_e56815: f64 = (assign42310_e56814).sqrt();
        let assign42310_e56817: f64 = (p.p262).sqrt();
        let assign42310_e56818: f64 = (assign42310_e56815 - assign42310_e56817);
        (assign42310_e56818, (((locals.var_pds_dn0 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn0)) / (2.0 * assign42310_e56815)), (((locals.var_pds_dn2 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn2)) / (2.0 * assign42310_e56815)), (((locals.var_pds_dn4 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn4)) / (2.0 * assign42310_e56815)), (((locals.var_pds_dn5 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn5)) / (2.0 * assign42310_e56815)), (((locals.var_pds_dn6 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn6)) / (2.0 * assign42310_e56815)), (((locals.var_pds_dn7 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn7)) / (2.0 * assign42310_e56815)), (((locals.var_pds_dn8 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn8)) / (2.0 * assign42310_e56815)), (((locals.var_pds_dn9 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn9)) / (2.0 * assign42310_e56815)), (((locals.var_pds_dn10 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn10)) / (2.0 * assign42310_e56815)), (((locals.var_pds_dn11 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn11)) / (2.0 * assign42310_e56815)), (((locals.var_pds_dn14 * locals.var_pds) + (locals.var_pds * locals.var_pds_dn14)) / (2.0 * assign42310_e56815)),)
    } else {
        (locals.var_pdsz, locals.var_pdsz_dn0, locals.var_pdsz_dn2, locals.var_pdsz_dn4, locals.var_pdsz_dn5, locals.var_pdsz_dn6, locals.var_pdsz_dn7, locals.var_pdsz_dn8, locals.var_pdsz_dn9, locals.var_pdsz_dn10, locals.var_pdsz_dn11, locals.var_pdsz_dn14,)
    }
};
        locals.var_pdsz = assign42310_e56820;
        locals.var_pdsz_dn0 = assign42310_e56820_d_n0;
        locals.var_pdsz_dn2 = assign42310_e56820_d_n2;
        locals.var_pdsz_dn4 = assign42310_e56820_d_n4;
        locals.var_pdsz_dn5 = assign42310_e56820_d_n5;
        locals.var_pdsz_dn6 = assign42310_e56820_d_n6;
        locals.var_pdsz_dn7 = assign42310_e56820_d_n7;
        locals.var_pdsz_dn8 = assign42310_e56820_d_n8;
        locals.var_pdsz_dn9 = assign42310_e56820_d_n9;
        locals.var_pdsz_dn10 = assign42310_e56820_d_n10;
        locals.var_pdsz_dn11 = assign42310_e56820_d_n11;
        locals.var_pdsz_dn14 = assign42310_e56820_d_n14;
        locals.var_pdsz_rv = 0.0;

        let (assign42320_e56833, assign42320_e56833_d_n0, assign42320_e56833_d_n2, assign42320_e56833_d_n4, assign42320_e56833_d_n5, assign42320_e56833_d_n6, assign42320_e56833_d_n7, assign42320_e56833_d_n8, assign42320_e56833_d_n9, assign42320_e56833_d_n10, assign42320_e56833_d_n11, assign42320_e56833_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        let assign42320_e56830: f64 = (locals.var_pdsz * locals.var_t0);
        let assign42320_e56831: f64 = (1.0 + assign42320_e56830);
        (assign42320_e56831, ((locals.var_pdsz_dn0 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn0)), ((locals.var_pdsz_dn2 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn2)), ((locals.var_pdsz_dn4 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn4)), ((locals.var_pdsz_dn5 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn5)), ((locals.var_pdsz_dn6 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn6)), ((locals.var_pdsz_dn7 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn7)), ((locals.var_pdsz_dn8 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn8)), ((locals.var_pdsz_dn9 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn9)), ((locals.var_pdsz_dn10 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn10)), ((locals.var_pdsz_dn11 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn11)), ((locals.var_pdsz_dn14 * locals.var_t0) + (locals.var_pdsz * locals.var_t0_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign42320_e56833;
        locals.var_t4_dn0 = assign42320_e56833_d_n0;
        locals.var_t4_dn2 = assign42320_e56833_d_n2;
        locals.var_t4_dn4 = assign42320_e56833_d_n4;
        locals.var_t4_dn5 = assign42320_e56833_d_n5;
        locals.var_t4_dn6 = assign42320_e56833_d_n6;
        locals.var_t4_dn7 = assign42320_e56833_d_n7;
        locals.var_t4_dn8 = assign42320_e56833_d_n8;
        locals.var_t4_dn9 = assign42320_e56833_d_n9;
        locals.var_t4_dn10 = assign42320_e56833_d_n10;
        locals.var_t4_dn11 = assign42320_e56833_d_n11;
        locals.var_t4_dn14 = assign42320_e56833_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign42330_e56844, assign42330_e56844_d_n0, assign42330_e56844_d_n2, assign42330_e56844_d_n4, assign42330_e56844_d_n5, assign42330_e56844_d_n6, assign42330_e56844_d_n7, assign42330_e56844_d_n8, assign42330_e56844_d_n9, assign42330_e56844_d_n10, assign42330_e56844_d_n11, assign42330_e56844_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        let assign42330_e56842: f64 = (locals.var_t2 * locals.var_qn0);
        (assign42330_e56842, ((locals.var_t2_dn0 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn0)), ((locals.var_t2_dn2 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn2)), ((locals.var_t2_dn4 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn4)), ((locals.var_t2_dn5 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn5)), ((locals.var_t2_dn6 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn6)), ((locals.var_t2_dn7 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn7)), ((locals.var_t2_dn8 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn8)), ((locals.var_t2_dn9 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn9)), ((locals.var_t2_dn10 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn10)), ((locals.var_t2_dn11 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn11)), ((locals.var_t2_dn14 * locals.var_qn0) + (locals.var_t2 * locals.var_qn0_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign42330_e56844;
        locals.var_t5_dn0 = assign42330_e56844_d_n0;
        locals.var_t5_dn2 = assign42330_e56844_d_n2;
        locals.var_t5_dn4 = assign42330_e56844_d_n4;
        locals.var_t5_dn5 = assign42330_e56844_d_n5;
        locals.var_t5_dn6 = assign42330_e56844_d_n6;
        locals.var_t5_dn7 = assign42330_e56844_d_n7;
        locals.var_t5_dn8 = assign42330_e56844_d_n8;
        locals.var_t5_dn9 = assign42330_e56844_d_n9;
        locals.var_t5_dn10 = assign42330_e56844_d_n10;
        locals.var_t5_dn11 = assign42330_e56844_d_n11;
        locals.var_t5_dn14 = assign42330_e56844_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign42340_e56855, assign42340_e56855_d_n0, assign42340_e56855_d_n2, assign42340_e56855_d_n4, assign42340_e56855_d_n5, assign42340_e56855_d_n6, assign42340_e56855_d_n7, assign42340_e56855_d_n8, assign42340_e56855_d_n9, assign42340_e56855_d_n10, assign42340_e56855_d_n11, assign42340_e56855_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        let assign42340_e56853: f64 = (locals.var_t5 / locals.var_t4);
        (assign42340_e56853, (((locals.var_t5_dn0 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn2 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn4 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn5 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn6 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn7 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn8 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn9 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn10 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn11 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn14 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn14)) / (locals.var_t4 * locals.var_t4)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign42340_e56855;
        locals.var_t3_dn0 = assign42340_e56855_d_n0;
        locals.var_t3_dn2 = assign42340_e56855_d_n2;
        locals.var_t3_dn4 = assign42340_e56855_d_n4;
        locals.var_t3_dn5 = assign42340_e56855_d_n5;
        locals.var_t3_dn6 = assign42340_e56855_d_n6;
        locals.var_t3_dn7 = assign42340_e56855_d_n7;
        locals.var_t3_dn8 = assign42340_e56855_d_n8;
        locals.var_t3_dn9 = assign42340_e56855_d_n9;
        locals.var_t3_dn10 = assign42340_e56855_d_n10;
        locals.var_t3_dn11 = assign42340_e56855_d_n11;
        locals.var_t3_dn14 = assign42340_e56855_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign42350_e56864, assign42350_e56864_d_n0, assign42350_e56864_d_n2, assign42350_e56864_d_n4, assign42350_e56864_d_n5, assign42350_e56864_d_n6, assign42350_e56864_d_n7, assign42350_e56864_d_n8, assign42350_e56864_d_n9, assign42350_e56864_d_n10, assign42350_e56864_d_n11, assign42350_e56864_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    } else {
        (locals.var_eeff, locals.var_eeff_dn0, locals.var_eeff_dn2, locals.var_eeff_dn4, locals.var_eeff_dn5, locals.var_eeff_dn6, locals.var_eeff_dn7, locals.var_eeff_dn8, locals.var_eeff_dn9, locals.var_eeff_dn10, locals.var_eeff_dn11, locals.var_eeff_dn14,)
    }
};
        locals.var_eeff = assign42350_e56864;
        locals.var_eeff_dn0 = assign42350_e56864_d_n0;
        locals.var_eeff_dn2 = assign42350_e56864_d_n2;
        locals.var_eeff_dn4 = assign42350_e56864_d_n4;
        locals.var_eeff_dn5 = assign42350_e56864_d_n5;
        locals.var_eeff_dn6 = assign42350_e56864_d_n6;
        locals.var_eeff_dn7 = assign42350_e56864_d_n7;
        locals.var_eeff_dn8 = assign42350_e56864_d_n8;
        locals.var_eeff_dn9 = assign42350_e56864_d_n9;
        locals.var_eeff_dn10 = assign42350_e56864_d_n10;
        locals.var_eeff_dn11 = assign42350_e56864_d_n11;
        locals.var_eeff_dn14 = assign42350_e56864_d_n14;
        locals.var_eeff_rv = 0.0;

        let (assign42360_e56882, assign42360_e56882_d_n0, assign42360_e56882_d_n2, assign42360_e56882_d_n4, assign42360_e56882_d_n5, assign42360_e56882_d_n6, assign42360_e56882_d_n7, assign42360_e56882_d_n8, assign42360_e56882_d_n9, assign42360_e56882_d_n10, assign42360_e56882_d_n11, assign42360_e56882_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        let (assign42360_e56880, assign42360_e56880_d_n0, assign42360_e56880_d_n2, assign42360_e56880_d_n4, assign42360_e56880_d_n5, assign42360_e56880_d_n6, assign42360_e56880_d_n7, assign42360_e56880_d_n8, assign42360_e56880_d_n9, assign42360_e56880_d_n10, assign42360_e56880_d_n11, assign42360_e56880_d_n14,) = {
            if (locals.var_eeff == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign42360_e56878: f64 = (p.p160 - 1.0);
                let assign42360_e56879: f64 = (locals.var_eeff).powf(assign42360_e56878);
                (assign42360_e56879, if 0.0 == 0.0 && ((assign42360_e56878) as f64).is_finite() && ((assign42360_e56878) as f64).fract() == 0.0 { if assign42360_e56878 == 0.0 { 0.0 } else { (assign42360_e56878 * ((locals.var_eeff).powf(assign42360_e56878 - 1.0) * locals.var_eeff_dn0)) } } else { (assign42360_e56879 * (assign42360_e56878 * (locals.var_eeff_dn0 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42360_e56878) as f64).is_finite() && ((assign42360_e56878) as f64).fract() == 0.0 { if assign42360_e56878 == 0.0 { 0.0 } else { (assign42360_e56878 * ((locals.var_eeff).powf(assign42360_e56878 - 1.0) * locals.var_eeff_dn2)) } } else { (assign42360_e56879 * (assign42360_e56878 * (locals.var_eeff_dn2 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42360_e56878) as f64).is_finite() && ((assign42360_e56878) as f64).fract() == 0.0 { if assign42360_e56878 == 0.0 { 0.0 } else { (assign42360_e56878 * ((locals.var_eeff).powf(assign42360_e56878 - 1.0) * locals.var_eeff_dn4)) } } else { (assign42360_e56879 * (assign42360_e56878 * (locals.var_eeff_dn4 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42360_e56878) as f64).is_finite() && ((assign42360_e56878) as f64).fract() == 0.0 { if assign42360_e56878 == 0.0 { 0.0 } else { (assign42360_e56878 * ((locals.var_eeff).powf(assign42360_e56878 - 1.0) * locals.var_eeff_dn5)) } } else { (assign42360_e56879 * (assign42360_e56878 * (locals.var_eeff_dn5 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42360_e56878) as f64).is_finite() && ((assign42360_e56878) as f64).fract() == 0.0 { if assign42360_e56878 == 0.0 { 0.0 } else { (assign42360_e56878 * ((locals.var_eeff).powf(assign42360_e56878 - 1.0) * locals.var_eeff_dn6)) } } else { (assign42360_e56879 * (assign42360_e56878 * (locals.var_eeff_dn6 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42360_e56878) as f64).is_finite() && ((assign42360_e56878) as f64).fract() == 0.0 { if assign42360_e56878 == 0.0 { 0.0 } else { (assign42360_e56878 * ((locals.var_eeff).powf(assign42360_e56878 - 1.0) * locals.var_eeff_dn7)) } } else { (assign42360_e56879 * (assign42360_e56878 * (locals.var_eeff_dn7 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42360_e56878) as f64).is_finite() && ((assign42360_e56878) as f64).fract() == 0.0 { if assign42360_e56878 == 0.0 { 0.0 } else { (assign42360_e56878 * ((locals.var_eeff).powf(assign42360_e56878 - 1.0) * locals.var_eeff_dn8)) } } else { (assign42360_e56879 * (assign42360_e56878 * (locals.var_eeff_dn8 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42360_e56878) as f64).is_finite() && ((assign42360_e56878) as f64).fract() == 0.0 { if assign42360_e56878 == 0.0 { 0.0 } else { (assign42360_e56878 * ((locals.var_eeff).powf(assign42360_e56878 - 1.0) * locals.var_eeff_dn9)) } } else { (assign42360_e56879 * (assign42360_e56878 * (locals.var_eeff_dn9 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42360_e56878) as f64).is_finite() && ((assign42360_e56878) as f64).fract() == 0.0 { if assign42360_e56878 == 0.0 { 0.0 } else { (assign42360_e56878 * ((locals.var_eeff).powf(assign42360_e56878 - 1.0) * locals.var_eeff_dn10)) } } else { (assign42360_e56879 * (assign42360_e56878 * (locals.var_eeff_dn10 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42360_e56878) as f64).is_finite() && ((assign42360_e56878) as f64).fract() == 0.0 { if assign42360_e56878 == 0.0 { 0.0 } else { (assign42360_e56878 * ((locals.var_eeff).powf(assign42360_e56878 - 1.0) * locals.var_eeff_dn11)) } } else { (assign42360_e56879 * (assign42360_e56878 * (locals.var_eeff_dn11 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42360_e56878) as f64).is_finite() && ((assign42360_e56878) as f64).fract() == 0.0 { if assign42360_e56878 == 0.0 { 0.0 } else { (assign42360_e56878 * ((locals.var_eeff).powf(assign42360_e56878 - 1.0) * locals.var_eeff_dn14)) } } else { (assign42360_e56879 * (assign42360_e56878 * (locals.var_eeff_dn14 / locals.var_eeff))) },)
            }
        };
        (assign42360_e56880, assign42360_e56880_d_n0, assign42360_e56880_d_n2, assign42360_e56880_d_n4, assign42360_e56880_d_n5, assign42360_e56880_d_n6, assign42360_e56880_d_n7, assign42360_e56880_d_n8, assign42360_e56880_d_n9, assign42360_e56880_d_n10, assign42360_e56880_d_n11, assign42360_e56880_d_n14,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign42360_e56882;
        locals.var_t5_dn0 = assign42360_e56882_d_n0;
        locals.var_t5_dn2 = assign42360_e56882_d_n2;
        locals.var_t5_dn4 = assign42360_e56882_d_n4;
        locals.var_t5_dn5 = assign42360_e56882_d_n5;
        locals.var_t5_dn6 = assign42360_e56882_d_n6;
        locals.var_t5_dn7 = assign42360_e56882_d_n7;
        locals.var_t5_dn8 = assign42360_e56882_d_n8;
        locals.var_t5_dn9 = assign42360_e56882_d_n9;
        locals.var_t5_dn10 = assign42360_e56882_d_n10;
        locals.var_t5_dn11 = assign42360_e56882_d_n11;
        locals.var_t5_dn14 = assign42360_e56882_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign42370_e56893, assign42370_e56893_d_n0, assign42370_e56893_d_n2, assign42370_e56893_d_n4, assign42370_e56893_d_n5, assign42370_e56893_d_n6, assign42370_e56893_d_n7, assign42370_e56893_d_n8, assign42370_e56893_d_n9, assign42370_e56893_d_n10, assign42370_e56893_d_n11, assign42370_e56893_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        let assign42370_e56891: f64 = (locals.var_t5 * locals.var_eeff);
        (assign42370_e56891, ((locals.var_t5_dn0 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn0)), ((locals.var_t5_dn2 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn2)), ((locals.var_t5_dn4 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn4)), ((locals.var_t5_dn5 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn5)), ((locals.var_t5_dn6 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn6)), ((locals.var_t5_dn7 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn7)), ((locals.var_t5_dn8 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn8)), ((locals.var_t5_dn9 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn9)), ((locals.var_t5_dn10 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn10)), ((locals.var_t5_dn11 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn11)), ((locals.var_t5_dn14 * locals.var_eeff) + (locals.var_t5 * locals.var_eeff_dn14)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign42370_e56893;
        locals.var_t8_dn0 = assign42370_e56893_d_n0;
        locals.var_t8_dn2 = assign42370_e56893_d_n2;
        locals.var_t8_dn4 = assign42370_e56893_d_n4;
        locals.var_t8_dn5 = assign42370_e56893_d_n5;
        locals.var_t8_dn6 = assign42370_e56893_d_n6;
        locals.var_t8_dn7 = assign42370_e56893_d_n7;
        locals.var_t8_dn8 = assign42370_e56893_d_n8;
        locals.var_t8_dn9 = assign42370_e56893_d_n9;
        locals.var_t8_dn10 = assign42370_e56893_d_n10;
        locals.var_t8_dn11 = assign42370_e56893_d_n11;
        locals.var_t8_dn14 = assign42370_e56893_d_n14;
        locals.var_t8_rv = 0.0;

        let (assign42380_e56911, assign42380_e56911_d_n0, assign42380_e56911_d_n2, assign42380_e56911_d_n4, assign42380_e56911_d_n5, assign42380_e56911_d_n6, assign42380_e56911_d_n7, assign42380_e56911_d_n8, assign42380_e56911_d_n9, assign42380_e56911_d_n10, assign42380_e56911_d_n11, assign42380_e56911_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        let (assign42380_e56909, assign42380_e56909_d_n0, assign42380_e56909_d_n2, assign42380_e56909_d_n4, assign42380_e56909_d_n5, assign42380_e56909_d_n6, assign42380_e56909_d_n7, assign42380_e56909_d_n8, assign42380_e56909_d_n9, assign42380_e56909_d_n10, assign42380_e56909_d_n11, assign42380_e56909_d_n14,) = {
            if (locals.var_eeff == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign42380_e56907: f64 = (locals.var_muesr - 1.0);
                let assign42380_e56908: f64 = (locals.var_eeff).powf(assign42380_e56907);
                (assign42380_e56908, if 0.0 == 0.0 && ((assign42380_e56907) as f64).is_finite() && ((assign42380_e56907) as f64).fract() == 0.0 { if assign42380_e56907 == 0.0 { 0.0 } else { (assign42380_e56907 * ((locals.var_eeff).powf(assign42380_e56907 - 1.0) * locals.var_eeff_dn0)) } } else { (assign42380_e56908 * (assign42380_e56907 * (locals.var_eeff_dn0 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42380_e56907) as f64).is_finite() && ((assign42380_e56907) as f64).fract() == 0.0 { if assign42380_e56907 == 0.0 { 0.0 } else { (assign42380_e56907 * ((locals.var_eeff).powf(assign42380_e56907 - 1.0) * locals.var_eeff_dn2)) } } else { (assign42380_e56908 * (assign42380_e56907 * (locals.var_eeff_dn2 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42380_e56907) as f64).is_finite() && ((assign42380_e56907) as f64).fract() == 0.0 { if assign42380_e56907 == 0.0 { 0.0 } else { (assign42380_e56907 * ((locals.var_eeff).powf(assign42380_e56907 - 1.0) * locals.var_eeff_dn4)) } } else { (assign42380_e56908 * (assign42380_e56907 * (locals.var_eeff_dn4 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42380_e56907) as f64).is_finite() && ((assign42380_e56907) as f64).fract() == 0.0 { if assign42380_e56907 == 0.0 { 0.0 } else { (assign42380_e56907 * ((locals.var_eeff).powf(assign42380_e56907 - 1.0) * locals.var_eeff_dn5)) } } else { (assign42380_e56908 * (assign42380_e56907 * (locals.var_eeff_dn5 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42380_e56907) as f64).is_finite() && ((assign42380_e56907) as f64).fract() == 0.0 { if assign42380_e56907 == 0.0 { 0.0 } else { (assign42380_e56907 * ((locals.var_eeff).powf(assign42380_e56907 - 1.0) * locals.var_eeff_dn6)) } } else { (assign42380_e56908 * (assign42380_e56907 * (locals.var_eeff_dn6 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42380_e56907) as f64).is_finite() && ((assign42380_e56907) as f64).fract() == 0.0 { if assign42380_e56907 == 0.0 { 0.0 } else { (assign42380_e56907 * ((locals.var_eeff).powf(assign42380_e56907 - 1.0) * locals.var_eeff_dn7)) } } else { (assign42380_e56908 * (assign42380_e56907 * (locals.var_eeff_dn7 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42380_e56907) as f64).is_finite() && ((assign42380_e56907) as f64).fract() == 0.0 { if assign42380_e56907 == 0.0 { 0.0 } else { (assign42380_e56907 * ((locals.var_eeff).powf(assign42380_e56907 - 1.0) * locals.var_eeff_dn8)) } } else { (assign42380_e56908 * (assign42380_e56907 * (locals.var_eeff_dn8 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42380_e56907) as f64).is_finite() && ((assign42380_e56907) as f64).fract() == 0.0 { if assign42380_e56907 == 0.0 { 0.0 } else { (assign42380_e56907 * ((locals.var_eeff).powf(assign42380_e56907 - 1.0) * locals.var_eeff_dn9)) } } else { (assign42380_e56908 * (assign42380_e56907 * (locals.var_eeff_dn9 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42380_e56907) as f64).is_finite() && ((assign42380_e56907) as f64).fract() == 0.0 { if assign42380_e56907 == 0.0 { 0.0 } else { (assign42380_e56907 * ((locals.var_eeff).powf(assign42380_e56907 - 1.0) * locals.var_eeff_dn10)) } } else { (assign42380_e56908 * (assign42380_e56907 * (locals.var_eeff_dn10 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42380_e56907) as f64).is_finite() && ((assign42380_e56907) as f64).fract() == 0.0 { if assign42380_e56907 == 0.0 { 0.0 } else { (assign42380_e56907 * ((locals.var_eeff).powf(assign42380_e56907 - 1.0) * locals.var_eeff_dn11)) } } else { (assign42380_e56908 * (assign42380_e56907 * (locals.var_eeff_dn11 / locals.var_eeff))) }, if 0.0 == 0.0 && ((assign42380_e56907) as f64).is_finite() && ((assign42380_e56907) as f64).fract() == 0.0 { if assign42380_e56907 == 0.0 { 0.0 } else { (assign42380_e56907 * ((locals.var_eeff).powf(assign42380_e56907 - 1.0) * locals.var_eeff_dn14)) } } else { (assign42380_e56908 * (assign42380_e56907 * (locals.var_eeff_dn14 / locals.var_eeff))) },)
            }
        };
        (assign42380_e56909, assign42380_e56909_d_n0, assign42380_e56909_d_n2, assign42380_e56909_d_n4, assign42380_e56909_d_n5, assign42380_e56909_d_n6, assign42380_e56909_d_n7, assign42380_e56909_d_n8, assign42380_e56909_d_n9, assign42380_e56909_d_n10, assign42380_e56909_d_n11, assign42380_e56909_d_n14,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign42380_e56911;
        locals.var_t7_dn0 = assign42380_e56911_d_n0;
        locals.var_t7_dn2 = assign42380_e56911_d_n2;
        locals.var_t7_dn4 = assign42380_e56911_d_n4;
        locals.var_t7_dn5 = assign42380_e56911_d_n5;
        locals.var_t7_dn6 = assign42380_e56911_d_n6;
        locals.var_t7_dn7 = assign42380_e56911_d_n7;
        locals.var_t7_dn8 = assign42380_e56911_d_n8;
        locals.var_t7_dn9 = assign42380_e56911_d_n9;
        locals.var_t7_dn10 = assign42380_e56911_d_n10;
        locals.var_t7_dn11 = assign42380_e56911_d_n11;
        locals.var_t7_dn14 = assign42380_e56911_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign42390_e56922, assign42390_e56922_d_n0, assign42390_e56922_d_n2, assign42390_e56922_d_n4, assign42390_e56922_d_n5, assign42390_e56922_d_n6, assign42390_e56922_d_n7, assign42390_e56922_d_n8, assign42390_e56922_d_n9, assign42390_e56922_d_n10, assign42390_e56922_d_n11, assign42390_e56922_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        let assign42390_e56920: f64 = (locals.var_t7 * locals.var_eeff);
        (assign42390_e56920, ((locals.var_t7_dn0 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn0)), ((locals.var_t7_dn2 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn2)), ((locals.var_t7_dn4 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn4)), ((locals.var_t7_dn5 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn5)), ((locals.var_t7_dn6 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn6)), ((locals.var_t7_dn7 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn7)), ((locals.var_t7_dn8 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn8)), ((locals.var_t7_dn9 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn9)), ((locals.var_t7_dn10 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn10)), ((locals.var_t7_dn11 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn11)), ((locals.var_t7_dn14 * locals.var_eeff) + (locals.var_t7 * locals.var_eeff_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign42390_e56922;
        locals.var_t6_dn0 = assign42390_e56922_d_n0;
        locals.var_t6_dn2 = assign42390_e56922_d_n2;
        locals.var_t6_dn4 = assign42390_e56922_d_n4;
        locals.var_t6_dn5 = assign42390_e56922_d_n5;
        locals.var_t6_dn6 = assign42390_e56922_d_n6;
        locals.var_t6_dn7 = assign42390_e56922_d_n7;
        locals.var_t6_dn8 = assign42390_e56922_d_n8;
        locals.var_t6_dn9 = assign42390_e56922_d_n9;
        locals.var_t6_dn10 = assign42390_e56922_d_n10;
        locals.var_t6_dn11 = assign42390_e56922_d_n11;
        locals.var_t6_dn14 = assign42390_e56922_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign42400_e56933, assign42400_e56933_d_n0, assign42400_e56933_d_n2, assign42400_e56933_d_n4, assign42400_e56933_d_n5, assign42400_e56933_d_n6, assign42400_e56933_d_n7, assign42400_e56933_d_n8, assign42400_e56933_d_n9, assign42400_e56933_d_n10, assign42400_e56933_d_n11, assign42400_e56933_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        let assign42400_e56931: f64 = (1.6021918e-19 * 10000.0);
        (assign42400_e56931, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign42400_e56933;
        locals.var_t9_dn0 = assign42400_e56933_d_n0;
        locals.var_t9_dn2 = assign42400_e56933_d_n2;
        locals.var_t9_dn4 = assign42400_e56933_d_n4;
        locals.var_t9_dn5 = assign42400_e56933_d_n5;
        locals.var_t9_dn6 = assign42400_e56933_d_n6;
        locals.var_t9_dn7 = assign42400_e56933_d_n7;
        locals.var_t9_dn8 = assign42400_e56933_d_n8;
        locals.var_t9_dn9 = assign42400_e56933_d_n9;
        locals.var_t9_dn10 = assign42400_e56933_d_n10;
        locals.var_t9_dn11 = assign42400_e56933_d_n11;
        locals.var_t9_dn14 = assign42400_e56933_d_n14;
        locals.var_t9_rv = 0.0;

        let (assign42410_e56944, assign42410_e56944_d_n0, assign42410_e56944_d_n2, assign42410_e56944_d_n4, assign42410_e56944_d_n5, assign42410_e56944_d_n6, assign42410_e56944_d_n7, assign42410_e56944_d_n8, assign42410_e56944_d_n9, assign42410_e56944_d_n10, assign42410_e56944_d_n11, assign42410_e56944_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        let assign42410_e56942: f64 = (locals.var_qn0 / locals.var_t9);
        (assign42410_e56942, (((locals.var_qn0_dn0 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn0)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn2 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn2)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn4 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn4)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn5 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn5)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn6 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn6)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn7 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn7)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn8 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn8)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn9 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn9)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn10 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn10)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn11 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn11)) / (locals.var_t9 * locals.var_t9)), (((locals.var_qn0_dn14 * locals.var_t9) - (locals.var_qn0 * locals.var_t9_dn14)) / (locals.var_t9 * locals.var_t9)),)
    } else {
        (locals.var_rns, locals.var_rns_dn0, locals.var_rns_dn2, locals.var_rns_dn4, locals.var_rns_dn5, locals.var_rns_dn6, locals.var_rns_dn7, locals.var_rns_dn8, locals.var_rns_dn9, locals.var_rns_dn10, locals.var_rns_dn11, locals.var_rns_dn14,)
    }
};
        locals.var_rns = assign42410_e56944;
        locals.var_rns_dn0 = assign42410_e56944_d_n0;
        locals.var_rns_dn2 = assign42410_e56944_d_n2;
        locals.var_rns_dn4 = assign42410_e56944_d_n4;
        locals.var_rns_dn5 = assign42410_e56944_d_n5;
        locals.var_rns_dn6 = assign42410_e56944_d_n6;
        locals.var_rns_dn7 = assign42410_e56944_d_n7;
        locals.var_rns_dn8 = assign42410_e56944_d_n8;
        locals.var_rns_dn9 = assign42410_e56944_d_n9;
        locals.var_rns_dn10 = assign42410_e56944_d_n10;
        locals.var_rns_dn11 = assign42410_e56944_d_n11;
        locals.var_rns_dn14 = assign42410_e56944_d_n14;
        locals.var_rns_rv = 0.0;

        let (assign42420_e56953, assign42420_e56953_d_n0, assign42420_e56953_d_n2, assign42420_e56953_d_n4, assign42420_e56953_d_n5, assign42420_e56953_d_n6, assign42420_e56953_d_n7, assign42420_e56953_d_n8, assign42420_e56953_d_n9, assign42420_e56953_d_n10, assign42420_e56953_d_n11, assign42420_e56953_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        (locals.var_uc_muecb0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign42420_e56953;
        locals.var_t2_dn0 = assign42420_e56953_d_n0;
        locals.var_t2_dn2 = assign42420_e56953_d_n2;
        locals.var_t2_dn4 = assign42420_e56953_d_n4;
        locals.var_t2_dn5 = assign42420_e56953_d_n5;
        locals.var_t2_dn6 = assign42420_e56953_d_n6;
        locals.var_t2_dn7 = assign42420_e56953_d_n7;
        locals.var_t2_dn8 = assign42420_e56953_d_n8;
        locals.var_t2_dn9 = assign42420_e56953_d_n9;
        locals.var_t2_dn10 = assign42420_e56953_d_n10;
        locals.var_t2_dn11 = assign42420_e56953_d_n11;
        locals.var_t2_dn14 = assign42420_e56953_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign42430_e56978, assign42430_e56978_d_n0, assign42430_e56978_d_n2, assign42430_e56978_d_n4, assign42430_e56978_d_n5, assign42430_e56978_d_n6, assign42430_e56978_d_n7, assign42430_e56978_d_n8, assign42430_e56978_d_n9, assign42430_e56978_d_n10, assign42430_e56978_d_n11, assign42430_e56978_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        let assign42430_e56964: f64 = (locals.var_uc_muecb1 * locals.var_rns);
        let assign42430_e56966: f64 = (assign42430_e56964 / 100000000000.0);
        let assign42430_e56967: f64 = (locals.var_t2 + assign42430_e56966);
        let assign42430_e56968: f64 = (1.0 / assign42430_e56967);
        let assign42430_e56971: f64 = (locals.var_mphn0 * locals.var_t8);
        let assign42430_e56972: f64 = (assign42430_e56968 + assign42430_e56971);
        let assign42430_e56975: f64 = (locals.var_t6 / locals.var_uc_muesr1);
        let assign42430_e56976: f64 = (assign42430_e56972 + assign42430_e56975);
        (assign42430_e56976, (((-((locals.var_t2_dn0 + ((locals.var_uc_muecb1 * locals.var_rns_dn0) / 100000000000.0)) / (assign42430_e56967 * assign42430_e56967))) + ((locals.var_mphn0_dn0 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn0))) + (locals.var_t6_dn0 / locals.var_uc_muesr1)), (((-((locals.var_t2_dn2 + ((locals.var_uc_muecb1 * locals.var_rns_dn2) / 100000000000.0)) / (assign42430_e56967 * assign42430_e56967))) + ((locals.var_mphn0_dn2 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn2))) + (locals.var_t6_dn2 / locals.var_uc_muesr1)), (((-((locals.var_t2_dn4 + ((locals.var_uc_muecb1 * locals.var_rns_dn4) / 100000000000.0)) / (assign42430_e56967 * assign42430_e56967))) + ((locals.var_mphn0_dn4 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn4))) + (locals.var_t6_dn4 / locals.var_uc_muesr1)), (((-((locals.var_t2_dn5 + ((locals.var_uc_muecb1 * locals.var_rns_dn5) / 100000000000.0)) / (assign42430_e56967 * assign42430_e56967))) + ((locals.var_mphn0_dn5 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn5))) + (locals.var_t6_dn5 / locals.var_uc_muesr1)), (((-((locals.var_t2_dn6 + ((locals.var_uc_muecb1 * locals.var_rns_dn6) / 100000000000.0)) / (assign42430_e56967 * assign42430_e56967))) + ((locals.var_mphn0_dn6 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn6))) + (locals.var_t6_dn6 / locals.var_uc_muesr1)), (((-((locals.var_t2_dn7 + ((locals.var_uc_muecb1 * locals.var_rns_dn7) / 100000000000.0)) / (assign42430_e56967 * assign42430_e56967))) + ((locals.var_mphn0_dn7 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn7))) + (locals.var_t6_dn7 / locals.var_uc_muesr1)), (((-((locals.var_t2_dn8 + ((locals.var_uc_muecb1 * locals.var_rns_dn8) / 100000000000.0)) / (assign42430_e56967 * assign42430_e56967))) + ((locals.var_mphn0_dn8 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn8))) + (locals.var_t6_dn8 / locals.var_uc_muesr1)), (((-((locals.var_t2_dn9 + ((locals.var_uc_muecb1 * locals.var_rns_dn9) / 100000000000.0)) / (assign42430_e56967 * assign42430_e56967))) + ((locals.var_mphn0_dn9 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn9))) + (locals.var_t6_dn9 / locals.var_uc_muesr1)), (((-((locals.var_t2_dn10 + ((locals.var_uc_muecb1 * locals.var_rns_dn10) / 100000000000.0)) / (assign42430_e56967 * assign42430_e56967))) + ((locals.var_mphn0_dn10 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn10))) + (locals.var_t6_dn10 / locals.var_uc_muesr1)), (((-((locals.var_t2_dn11 + ((locals.var_uc_muecb1 * locals.var_rns_dn11) / 100000000000.0)) / (assign42430_e56967 * assign42430_e56967))) + ((locals.var_mphn0_dn11 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn11))) + (locals.var_t6_dn11 / locals.var_uc_muesr1)), (((-((locals.var_t2_dn14 + ((locals.var_uc_muecb1 * locals.var_rns_dn14) / 100000000000.0)) / (assign42430_e56967 * assign42430_e56967))) + ((locals.var_mphn0_dn14 * locals.var_t8) + (locals.var_mphn0 * locals.var_t8_dn14))) + (locals.var_t6_dn14 / locals.var_uc_muesr1)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign42430_e56978;
        locals.var_t1_dn0 = assign42430_e56978_d_n0;
        locals.var_t1_dn2 = assign42430_e56978_d_n2;
        locals.var_t1_dn4 = assign42430_e56978_d_n4;
        locals.var_t1_dn5 = assign42430_e56978_d_n5;
        locals.var_t1_dn6 = assign42430_e56978_d_n6;
        locals.var_t1_dn7 = assign42430_e56978_d_n7;
        locals.var_t1_dn8 = assign42430_e56978_d_n8;
        locals.var_t1_dn9 = assign42430_e56978_d_n9;
        locals.var_t1_dn10 = assign42430_e56978_d_n10;
        locals.var_t1_dn11 = assign42430_e56978_d_n11;
        locals.var_t1_dn14 = assign42430_e56978_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign42440_e56989, assign42440_e56989_d_n0, assign42440_e56989_d_n2, assign42440_e56989_d_n4, assign42440_e56989_d_n5, assign42440_e56989_d_n6, assign42440_e56989_d_n7, assign42440_e56989_d_n8, assign42440_e56989_d_n9, assign42440_e56989_d_n10, assign42440_e56989_d_n11, assign42440_e56989_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        let assign42440_e56987: f64 = (1.0 / locals.var_t1);
        (assign42440_e56987, (-(locals.var_t1_dn0 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn2 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn4 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn5 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn6 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn7 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn8 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn9 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn10 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn11 / (locals.var_t1 * locals.var_t1))), (-(locals.var_t1_dn14 / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn11, locals.var_muun_dn14,)
    }
};
        locals.var_muun = assign42440_e56989;
        locals.var_muun_dn0 = assign42440_e56989_d_n0;
        locals.var_muun_dn2 = assign42440_e56989_d_n2;
        locals.var_muun_dn4 = assign42440_e56989_d_n4;
        locals.var_muun_dn5 = assign42440_e56989_d_n5;
        locals.var_muun_dn6 = assign42440_e56989_d_n6;
        locals.var_muun_dn7 = assign42440_e56989_d_n7;
        locals.var_muun_dn8 = assign42440_e56989_d_n8;
        locals.var_muun_dn9 = assign42440_e56989_d_n9;
        locals.var_muun_dn10 = assign42440_e56989_d_n10;
        locals.var_muun_dn11 = assign42440_e56989_d_n11;
        locals.var_muun_dn14 = assign42440_e56989_d_n14;
        locals.var_muun_rv = 0.0;

        let (assign42450_e57000, assign42450_e57000_d_n0, assign42450_e57000_d_n2, assign42450_e57000_d_n4, assign42450_e57000_d_n5, assign42450_e57000_d_n6, assign42450_e57000_d_n7, assign42450_e57000_d_n8, assign42450_e57000_d_n9, assign42450_e57000_d_n10, assign42450_e57000_d_n11, assign42450_e57000_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        let assign42450_e56998: f64 = (locals.var_muun / 10000.0);
        (assign42450_e56998, (locals.var_muun_dn0 / 10000.0), (locals.var_muun_dn2 / 10000.0), (locals.var_muun_dn4 / 10000.0), (locals.var_muun_dn5 / 10000.0), (locals.var_muun_dn6 / 10000.0), (locals.var_muun_dn7 / 10000.0), (locals.var_muun_dn8 / 10000.0), (locals.var_muun_dn9 / 10000.0), (locals.var_muun_dn10 / 10000.0), (locals.var_muun_dn11 / 10000.0), (locals.var_muun_dn14 / 10000.0),)
    } else {
        (locals.var_muun, locals.var_muun_dn0, locals.var_muun_dn2, locals.var_muun_dn4, locals.var_muun_dn5, locals.var_muun_dn6, locals.var_muun_dn7, locals.var_muun_dn8, locals.var_muun_dn9, locals.var_muun_dn10, locals.var_muun_dn11, locals.var_muun_dn14,)
    }
};
        locals.var_muun = assign42450_e57000;
        locals.var_muun_dn0 = assign42450_e57000_d_n0;
        locals.var_muun_dn2 = assign42450_e57000_d_n2;
        locals.var_muun_dn4 = assign42450_e57000_d_n4;
        locals.var_muun_dn5 = assign42450_e57000_d_n5;
        locals.var_muun_dn6 = assign42450_e57000_d_n6;
        locals.var_muun_dn7 = assign42450_e57000_d_n7;
        locals.var_muun_dn8 = assign42450_e57000_d_n8;
        locals.var_muun_dn9 = assign42450_e57000_d_n9;
        locals.var_muun_dn10 = assign42450_e57000_d_n10;
        locals.var_muun_dn11 = assign42450_e57000_d_n11;
        locals.var_muun_dn14 = assign42450_e57000_d_n14;
        locals.var_muun_rv = 0.0;

        let (assign42460_e57015, assign42460_e57015_d_n0, assign42460_e57015_d_n2, assign42460_e57015_d_n4, assign42460_e57015_d_n5, assign42460_e57015_d_n6, assign42460_e57015_d_n7, assign42460_e57015_d_n8, assign42460_e57015_d_n9, assign42460_e57015_d_n10, assign42460_e57015_d_n11, assign42460_e57015_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        let assign42460_e57010: f64 = (locals.var_qn0 + 1e-25);
        let assign42460_e57011: f64 = (locals.var_beta * assign42460_e57010);
        let assign42460_e57013: f64 = (assign42460_e57011 * locals.var_lch);
        (assign42460_e57013, ((((locals.var_beta_dn0 * assign42460_e57010) + (locals.var_beta * locals.var_qn0_dn0)) * locals.var_lch) + (assign42460_e57011 * locals.var_lch_dn0)), ((((locals.var_beta_dn2 * assign42460_e57010) + (locals.var_beta * locals.var_qn0_dn2)) * locals.var_lch) + (assign42460_e57011 * locals.var_lch_dn2)), ((((locals.var_beta_dn4 * assign42460_e57010) + (locals.var_beta * locals.var_qn0_dn4)) * locals.var_lch) + (assign42460_e57011 * locals.var_lch_dn4)), ((((locals.var_beta_dn5 * assign42460_e57010) + (locals.var_beta * locals.var_qn0_dn5)) * locals.var_lch) + (assign42460_e57011 * locals.var_lch_dn5)), ((((locals.var_beta_dn6 * assign42460_e57010) + (locals.var_beta * locals.var_qn0_dn6)) * locals.var_lch) + (assign42460_e57011 * locals.var_lch_dn6)), ((((locals.var_beta_dn7 * assign42460_e57010) + (locals.var_beta * locals.var_qn0_dn7)) * locals.var_lch) + (assign42460_e57011 * locals.var_lch_dn7)), ((((locals.var_beta_dn8 * assign42460_e57010) + (locals.var_beta * locals.var_qn0_dn8)) * locals.var_lch) + (assign42460_e57011 * locals.var_lch_dn8)), ((((locals.var_beta_dn9 * assign42460_e57010) + (locals.var_beta * locals.var_qn0_dn9)) * locals.var_lch) + (assign42460_e57011 * locals.var_lch_dn9)), ((((locals.var_beta_dn10 * assign42460_e57010) + (locals.var_beta * locals.var_qn0_dn10)) * locals.var_lch) + (assign42460_e57011 * locals.var_lch_dn10)), ((((locals.var_beta_dn11 * assign42460_e57010) + (locals.var_beta * locals.var_qn0_dn11)) * locals.var_lch) + (assign42460_e57011 * locals.var_lch_dn11)), ((((locals.var_beta_dn14 * assign42460_e57010) + (locals.var_beta * locals.var_qn0_dn14)) * locals.var_lch) + (assign42460_e57011 * locals.var_lch_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign42460_e57015;
        locals.var_t2_dn0 = assign42460_e57015_d_n0;
        locals.var_t2_dn2 = assign42460_e57015_d_n2;
        locals.var_t2_dn4 = assign42460_e57015_d_n4;
        locals.var_t2_dn5 = assign42460_e57015_d_n5;
        locals.var_t2_dn6 = assign42460_e57015_d_n6;
        locals.var_t2_dn7 = assign42460_e57015_d_n7;
        locals.var_t2_dn8 = assign42460_e57015_d_n8;
        locals.var_t2_dn9 = assign42460_e57015_d_n9;
        locals.var_t2_dn10 = assign42460_e57015_d_n10;
        locals.var_t2_dn11 = assign42460_e57015_d_n11;
        locals.var_t2_dn14 = assign42460_e57015_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign42470_e57026, assign42470_e57026_d_n0, assign42470_e57026_d_n2, assign42470_e57026_d_n4, assign42470_e57026_d_n5, assign42470_e57026_d_n6, assign42470_e57026_d_n7, assign42470_e57026_d_n8, assign42470_e57026_d_n9, assign42470_e57026_d_n10, assign42470_e57026_d_n11, assign42470_e57026_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        let assign42470_e57024: f64 = (1.0 / locals.var_t2);
        (assign42470_e57024, (-(locals.var_t2_dn0 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn2 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn4 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn5 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn6 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn7 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn8 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn9 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn10 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn11 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn14 / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign42470_e57026;
        locals.var_t1_dn0 = assign42470_e57026_d_n0;
        locals.var_t1_dn2 = assign42470_e57026_d_n2;
        locals.var_t1_dn4 = assign42470_e57026_d_n4;
        locals.var_t1_dn5 = assign42470_e57026_d_n5;
        locals.var_t1_dn6 = assign42470_e57026_d_n6;
        locals.var_t1_dn7 = assign42470_e57026_d_n7;
        locals.var_t1_dn8 = assign42470_e57026_d_n8;
        locals.var_t1_dn9 = assign42470_e57026_d_n9;
        locals.var_t1_dn10 = assign42470_e57026_d_n10;
        locals.var_t1_dn11 = assign42470_e57026_d_n11;
        locals.var_t1_dn14 = assign42470_e57026_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign42480_e57037, assign42480_e57037_d_n0, assign42480_e57037_d_n2, assign42480_e57037_d_n4, assign42480_e57037_d_n5, assign42480_e57037_d_n6, assign42480_e57037_d_n7, assign42480_e57037_d_n8, assign42480_e57037_d_n9, assign42480_e57037_d_n10, assign42480_e57037_d_n11, assign42480_e57037_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        let assign42480_e57035: f64 = (locals.var_idd * locals.var_t1);
        (assign42480_e57035, ((locals.var_idd_dn0 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn0)), ((locals.var_idd_dn2 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn2)), ((locals.var_idd_dn4 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn4)), ((locals.var_idd_dn5 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn5)), ((locals.var_idd_dn6 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn6)), ((locals.var_idd_dn7 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn7)), ((locals.var_idd_dn8 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn8)), ((locals.var_idd_dn9 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn9)), ((locals.var_idd_dn10 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn10)), ((locals.var_idd_dn11 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn11)), ((locals.var_idd_dn14 * locals.var_t1) + (locals.var_idd * locals.var_t1_dn14)),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign42480_e57037;
        locals.var_ty_dn0 = assign42480_e57037_d_n0;
        locals.var_ty_dn2 = assign42480_e57037_d_n2;
        locals.var_ty_dn4 = assign42480_e57037_d_n4;
        locals.var_ty_dn5 = assign42480_e57037_d_n5;
        locals.var_ty_dn6 = assign42480_e57037_d_n6;
        locals.var_ty_dn7 = assign42480_e57037_d_n7;
        locals.var_ty_dn8 = assign42480_e57037_d_n8;
        locals.var_ty_dn9 = assign42480_e57037_d_n9;
        locals.var_ty_dn10 = assign42480_e57037_d_n10;
        locals.var_ty_dn11 = assign42480_e57037_d_n11;
        locals.var_ty_dn14 = assign42480_e57037_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign42490_e57050, assign42490_e57050_d_n0, assign42490_e57050_d_n2, assign42490_e57050_d_n4, assign42490_e57050_d_n5, assign42490_e57050_d_n6, assign42490_e57050_d_n7, assign42490_e57050_d_n8, assign42490_e57050_d_n9, assign42490_e57050_d_n10, assign42490_e57050_d_n11, assign42490_e57050_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        let assign42490_e57046: f64 = (0.2 * locals.var_vmaxe);
        let assign42490_e57048: f64 = (assign42490_e57046 / locals.var_muun);
        (assign42490_e57048, ((((0.2 * locals.var_vmaxe_dn0) * locals.var_muun) - (assign42490_e57046 * locals.var_muun_dn0)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn2) * locals.var_muun) - (assign42490_e57046 * locals.var_muun_dn2)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn4) * locals.var_muun) - (assign42490_e57046 * locals.var_muun_dn4)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn5) * locals.var_muun) - (assign42490_e57046 * locals.var_muun_dn5)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn6) * locals.var_muun) - (assign42490_e57046 * locals.var_muun_dn6)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn7) * locals.var_muun) - (assign42490_e57046 * locals.var_muun_dn7)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn8) * locals.var_muun) - (assign42490_e57046 * locals.var_muun_dn8)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn9) * locals.var_muun) - (assign42490_e57046 * locals.var_muun_dn9)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn10) * locals.var_muun) - (assign42490_e57046 * locals.var_muun_dn10)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn11) * locals.var_muun) - (assign42490_e57046 * locals.var_muun_dn11)) / (locals.var_muun * locals.var_muun)), ((((0.2 * locals.var_vmaxe_dn14) * locals.var_muun) - (assign42490_e57046 * locals.var_muun_dn14)) / (locals.var_muun * locals.var_muun)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign42490_e57050;
        locals.var_t2_dn0 = assign42490_e57050_d_n0;
        locals.var_t2_dn2 = assign42490_e57050_d_n2;
        locals.var_t2_dn4 = assign42490_e57050_d_n4;
        locals.var_t2_dn5 = assign42490_e57050_d_n5;
        locals.var_t2_dn6 = assign42490_e57050_d_n6;
        locals.var_t2_dn7 = assign42490_e57050_d_n7;
        locals.var_t2_dn8 = assign42490_e57050_d_n8;
        locals.var_t2_dn9 = assign42490_e57050_d_n9;
        locals.var_t2_dn10 = assign42490_e57050_d_n10;
        locals.var_t2_dn11 = assign42490_e57050_d_n11;
        locals.var_t2_dn14 = assign42490_e57050_d_n14;
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_147(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign42500_e57066, assign42500_e57066_d_n0, assign42500_e57066_d_n2, assign42500_e57066_d_n4, assign42500_e57066_d_n5, assign42500_e57066_d_n6, assign42500_e57066_d_n7, assign42500_e57066_d_n8, assign42500_e57066_d_n9, assign42500_e57066_d_n10, assign42500_e57066_d_n11, assign42500_e57066_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        let assign42500_e57059: f64 = (locals.var_ty * locals.var_ty);
        let assign42500_e57062: f64 = (locals.var_t2 * locals.var_t2);
        let assign42500_e57063: f64 = (assign42500_e57059 + assign42500_e57062);
        let assign42500_e57064: f64 = (assign42500_e57063).sqrt();
        (assign42500_e57064, ((((locals.var_ty_dn0 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn0)) + ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0))) / (2.0 * assign42500_e57064)), ((((locals.var_ty_dn2 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn2)) + ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2))) / (2.0 * assign42500_e57064)), ((((locals.var_ty_dn4 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn4)) + ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4))) / (2.0 * assign42500_e57064)), ((((locals.var_ty_dn5 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn5)) + ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5))) / (2.0 * assign42500_e57064)), ((((locals.var_ty_dn6 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn6)) + ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6))) / (2.0 * assign42500_e57064)), ((((locals.var_ty_dn7 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn7)) + ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7))) / (2.0 * assign42500_e57064)), ((((locals.var_ty_dn8 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn8)) + ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8))) / (2.0 * assign42500_e57064)), ((((locals.var_ty_dn9 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn9)) + ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9))) / (2.0 * assign42500_e57064)), ((((locals.var_ty_dn10 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn10)) + ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10))) / (2.0 * assign42500_e57064)), ((((locals.var_ty_dn11 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn11)) + ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11))) / (2.0 * assign42500_e57064)), ((((locals.var_ty_dn14 * locals.var_ty) + (locals.var_ty * locals.var_ty_dn14)) + ((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14))) / (2.0 * assign42500_e57064)),)
    } else {
        (locals.var_ey, locals.var_ey_dn0, locals.var_ey_dn2, locals.var_ey_dn4, locals.var_ey_dn5, locals.var_ey_dn6, locals.var_ey_dn7, locals.var_ey_dn8, locals.var_ey_dn9, locals.var_ey_dn10, locals.var_ey_dn11, locals.var_ey_dn14,)
    }
};
        locals.var_ey = assign42500_e57066;
        locals.var_ey_dn0 = assign42500_e57066_d_n0;
        locals.var_ey_dn2 = assign42500_e57066_d_n2;
        locals.var_ey_dn4 = assign42500_e57066_d_n4;
        locals.var_ey_dn5 = assign42500_e57066_d_n5;
        locals.var_ey_dn6 = assign42500_e57066_d_n6;
        locals.var_ey_dn7 = assign42500_e57066_d_n7;
        locals.var_ey_dn8 = assign42500_e57066_d_n8;
        locals.var_ey_dn9 = assign42500_e57066_d_n9;
        locals.var_ey_dn10 = assign42500_e57066_d_n10;
        locals.var_ey_dn11 = assign42500_e57066_d_n11;
        locals.var_ey_dn14 = assign42500_e57066_d_n14;
        locals.var_ey_rv = 0.0;

        let (assign42510_e57077, assign42510_e57077_d_n0, assign42510_e57077_d_n2, assign42510_e57077_d_n4, assign42510_e57077_d_n5, assign42510_e57077_d_n6, assign42510_e57077_d_n7, assign42510_e57077_d_n8, assign42510_e57077_d_n9, assign42510_e57077_d_n10, assign42510_e57077_d_n11, assign42510_e57077_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        let assign42510_e57075: f64 = (1.0 / locals.var_ey);
        (assign42510_e57075, (-(locals.var_ey_dn0 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn2 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn4 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn5 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn6 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn7 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn8 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn9 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn10 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn11 / (locals.var_ey * locals.var_ey))), (-(locals.var_ey_dn14 / (locals.var_ey * locals.var_ey))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign42510_e57077;
        locals.var_t4_dn0 = assign42510_e57077_d_n0;
        locals.var_t4_dn2 = assign42510_e57077_d_n2;
        locals.var_t4_dn4 = assign42510_e57077_d_n4;
        locals.var_t4_dn5 = assign42510_e57077_d_n5;
        locals.var_t4_dn6 = assign42510_e57077_d_n6;
        locals.var_t4_dn7 = assign42510_e57077_d_n7;
        locals.var_t4_dn8 = assign42510_e57077_d_n8;
        locals.var_t4_dn9 = assign42510_e57077_d_n9;
        locals.var_t4_dn10 = assign42510_e57077_d_n10;
        locals.var_t4_dn11 = assign42510_e57077_d_n11;
        locals.var_t4_dn14 = assign42510_e57077_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign42520_e57088, assign42520_e57088_d_n0, assign42520_e57088_d_n2, assign42520_e57088_d_n4, assign42520_e57088_d_n5, assign42520_e57088_d_n6, assign42520_e57088_d_n7, assign42520_e57088_d_n8, assign42520_e57088_d_n9, assign42520_e57088_d_n10, assign42520_e57088_d_n11, assign42520_e57088_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        let assign42520_e57086: f64 = (locals.var_muun * locals.var_ey);
        (assign42520_e57086, ((locals.var_muun_dn0 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn0)), ((locals.var_muun_dn2 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn2)), ((locals.var_muun_dn4 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn4)), ((locals.var_muun_dn5 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn5)), ((locals.var_muun_dn6 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn6)), ((locals.var_muun_dn7 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn7)), ((locals.var_muun_dn8 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn8)), ((locals.var_muun_dn9 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn9)), ((locals.var_muun_dn10 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn10)), ((locals.var_muun_dn11 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn11)), ((locals.var_muun_dn14 * locals.var_ey) + (locals.var_muun * locals.var_ey_dn14)),)
    } else {
        (locals.var_em, locals.var_em_dn0, locals.var_em_dn2, locals.var_em_dn4, locals.var_em_dn5, locals.var_em_dn6, locals.var_em_dn7, locals.var_em_dn8, locals.var_em_dn9, locals.var_em_dn10, locals.var_em_dn11, locals.var_em_dn14,)
    }
};
        locals.var_em = assign42520_e57088;
        locals.var_em_dn0 = assign42520_e57088_d_n0;
        locals.var_em_dn2 = assign42520_e57088_d_n2;
        locals.var_em_dn4 = assign42520_e57088_d_n4;
        locals.var_em_dn5 = assign42520_e57088_d_n5;
        locals.var_em_dn6 = assign42520_e57088_d_n6;
        locals.var_em_dn7 = assign42520_e57088_d_n7;
        locals.var_em_dn8 = assign42520_e57088_d_n8;
        locals.var_em_dn9 = assign42520_e57088_d_n9;
        locals.var_em_dn10 = assign42520_e57088_d_n10;
        locals.var_em_dn11 = assign42520_e57088_d_n11;
        locals.var_em_dn14 = assign42520_e57088_d_n14;
        locals.var_em_rv = 0.0;

        let (assign42530_e57099, assign42530_e57099_d_n0, assign42530_e57099_d_n2, assign42530_e57099_d_n4, assign42530_e57099_d_n5, assign42530_e57099_d_n6, assign42530_e57099_d_n7, assign42530_e57099_d_n8, assign42530_e57099_d_n9, assign42530_e57099_d_n10, assign42530_e57099_d_n11, assign42530_e57099_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        let assign42530_e57097: f64 = (locals.var_em / locals.var_vmaxe);
        (assign42530_e57097, (((locals.var_em_dn0 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn0)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn2 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn2)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn4 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn4)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn5 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn5)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn6 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn6)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn7 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn7)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn8 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn8)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn9 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn9)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn10 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn10)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn11 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn11)) / (locals.var_vmaxe * locals.var_vmaxe)), (((locals.var_em_dn14 * locals.var_vmaxe) - (locals.var_em * locals.var_vmaxe_dn14)) / (locals.var_vmaxe * locals.var_vmaxe)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign42530_e57099;
        locals.var_t1_dn0 = assign42530_e57099_d_n0;
        locals.var_t1_dn2 = assign42530_e57099_d_n2;
        locals.var_t1_dn4 = assign42530_e57099_d_n4;
        locals.var_t1_dn5 = assign42530_e57099_d_n5;
        locals.var_t1_dn6 = assign42530_e57099_d_n6;
        locals.var_t1_dn7 = assign42530_e57099_d_n7;
        locals.var_t1_dn8 = assign42530_e57099_d_n8;
        locals.var_t1_dn9 = assign42530_e57099_d_n9;
        locals.var_t1_dn10 = assign42530_e57099_d_n10;
        locals.var_t1_dn11 = assign42530_e57099_d_n11;
        locals.var_t1_dn14 = assign42530_e57099_d_n14;
        locals.var_t1_rv = 0.0;

        let assign42540_e57103: f64 = (10.0 * 2.220446049250313e-16);
        let assign42540_e57104: f64 = (1.0 - assign42540_e57103);
        let assign42540_e57111: f64 = (10.0 * 2.220446049250313e-16);
        let assign42540_e57112: f64 = (1.0 + assign42540_e57111);
        let assign42540_e57114: f64 = if ((assign42540_e57104 <= p.p178) && (p.p178 <= assign42540_e57112)) { 1.0 } else { 0.0 };
        locals.var_guard1057 = assign42540_e57114;
        locals.var_guard1057_rv = 0.0;

        let (assign42550_e57125, assign42550_e57125_d_n0, assign42550_e57125_d_n2, assign42550_e57125_d_n4, assign42550_e57125_d_n5, assign42550_e57125_d_n6, assign42550_e57125_d_n7, assign42550_e57125_d_n8, assign42550_e57125_d_n9, assign42550_e57125_d_n10, assign42550_e57125_d_n11, assign42550_e57125_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1057 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign42550_e57125;
        locals.var_t3_dn0 = assign42550_e57125_d_n0;
        locals.var_t3_dn2 = assign42550_e57125_d_n2;
        locals.var_t3_dn4 = assign42550_e57125_d_n4;
        locals.var_t3_dn5 = assign42550_e57125_d_n5;
        locals.var_t3_dn6 = assign42550_e57125_d_n6;
        locals.var_t3_dn7 = assign42550_e57125_d_n7;
        locals.var_t3_dn8 = assign42550_e57125_d_n8;
        locals.var_t3_dn9 = assign42550_e57125_d_n9;
        locals.var_t3_dn10 = assign42550_e57125_d_n10;
        locals.var_t3_dn11 = assign42550_e57125_d_n11;
        locals.var_t3_dn14 = assign42550_e57125_d_n14;
        locals.var_t3_rv = 0.0;

        let assign42560_e57129: f64 = (10.0 * 2.220446049250313e-16);
        let assign42560_e57130: f64 = (2.0 - assign42560_e57129);
        let assign42560_e57137: f64 = (10.0 * 2.220446049250313e-16);
        let assign42560_e57138: f64 = (2.0 + assign42560_e57137);
        let assign42560_e57140: f64 = if ((assign42560_e57130 <= p.p178) && (p.p178 <= assign42560_e57138)) { 1.0 } else { 0.0 };
        locals.var_guard1058 = assign42560_e57140;
        locals.var_guard1058_rv = 0.0;

        let (assign42570_e57154, assign42570_e57154_d_n0, assign42570_e57154_d_n2, assign42570_e57154_d_n4, assign42570_e57154_d_n5, assign42570_e57154_d_n6, assign42570_e57154_d_n7, assign42570_e57154_d_n8, assign42570_e57154_d_n9, assign42570_e57154_d_n10, assign42570_e57154_d_n11, assign42570_e57154_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1057 == 0.0)) && (locals.var_guard1058 != 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign42570_e57154;
        locals.var_t3_dn0 = assign42570_e57154_d_n0;
        locals.var_t3_dn2 = assign42570_e57154_d_n2;
        locals.var_t3_dn4 = assign42570_e57154_d_n4;
        locals.var_t3_dn5 = assign42570_e57154_d_n5;
        locals.var_t3_dn6 = assign42570_e57154_d_n6;
        locals.var_t3_dn7 = assign42570_e57154_d_n7;
        locals.var_t3_dn8 = assign42570_e57154_d_n8;
        locals.var_t3_dn9 = assign42570_e57154_d_n9;
        locals.var_t3_dn10 = assign42570_e57154_d_n10;
        locals.var_t3_dn11 = assign42570_e57154_d_n11;
        locals.var_t3_dn14 = assign42570_e57154_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign42580_e57178, assign42580_e57178_d_n0, assign42580_e57178_d_n2, assign42580_e57178_d_n4, assign42580_e57178_d_n5, assign42580_e57178_d_n6, assign42580_e57178_d_n7, assign42580_e57178_d_n8, assign42580_e57178_d_n9, assign42580_e57178_d_n10, assign42580_e57178_d_n11, assign42580_e57178_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1057 == 0.0)) && (locals.var_guard1058 == 0.0)) {
        let (assign42580_e57176, assign42580_e57176_d_n0, assign42580_e57176_d_n2, assign42580_e57176_d_n4, assign42580_e57176_d_n5, assign42580_e57176_d_n6, assign42580_e57176_d_n7, assign42580_e57176_d_n8, assign42580_e57176_d_n9, assign42580_e57176_d_n10, assign42580_e57176_d_n11, assign42580_e57176_d_n14,) = {
            if (locals.var_t1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign42580_e57174: f64 = (p.p178 - 1.0);
                let assign42580_e57175: f64 = (locals.var_t1).powf(assign42580_e57174);
                (assign42580_e57175, if 0.0 == 0.0 && ((assign42580_e57174) as f64).is_finite() && ((assign42580_e57174) as f64).fract() == 0.0 { if assign42580_e57174 == 0.0 { 0.0 } else { (assign42580_e57174 * ((locals.var_t1).powf(assign42580_e57174 - 1.0) * locals.var_t1_dn0)) } } else { (assign42580_e57175 * (assign42580_e57174 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign42580_e57174) as f64).is_finite() && ((assign42580_e57174) as f64).fract() == 0.0 { if assign42580_e57174 == 0.0 { 0.0 } else { (assign42580_e57174 * ((locals.var_t1).powf(assign42580_e57174 - 1.0) * locals.var_t1_dn2)) } } else { (assign42580_e57175 * (assign42580_e57174 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign42580_e57174) as f64).is_finite() && ((assign42580_e57174) as f64).fract() == 0.0 { if assign42580_e57174 == 0.0 { 0.0 } else { (assign42580_e57174 * ((locals.var_t1).powf(assign42580_e57174 - 1.0) * locals.var_t1_dn4)) } } else { (assign42580_e57175 * (assign42580_e57174 * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign42580_e57174) as f64).is_finite() && ((assign42580_e57174) as f64).fract() == 0.0 { if assign42580_e57174 == 0.0 { 0.0 } else { (assign42580_e57174 * ((locals.var_t1).powf(assign42580_e57174 - 1.0) * locals.var_t1_dn5)) } } else { (assign42580_e57175 * (assign42580_e57174 * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign42580_e57174) as f64).is_finite() && ((assign42580_e57174) as f64).fract() == 0.0 { if assign42580_e57174 == 0.0 { 0.0 } else { (assign42580_e57174 * ((locals.var_t1).powf(assign42580_e57174 - 1.0) * locals.var_t1_dn6)) } } else { (assign42580_e57175 * (assign42580_e57174 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign42580_e57174) as f64).is_finite() && ((assign42580_e57174) as f64).fract() == 0.0 { if assign42580_e57174 == 0.0 { 0.0 } else { (assign42580_e57174 * ((locals.var_t1).powf(assign42580_e57174 - 1.0) * locals.var_t1_dn7)) } } else { (assign42580_e57175 * (assign42580_e57174 * (locals.var_t1_dn7 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign42580_e57174) as f64).is_finite() && ((assign42580_e57174) as f64).fract() == 0.0 { if assign42580_e57174 == 0.0 { 0.0 } else { (assign42580_e57174 * ((locals.var_t1).powf(assign42580_e57174 - 1.0) * locals.var_t1_dn8)) } } else { (assign42580_e57175 * (assign42580_e57174 * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign42580_e57174) as f64).is_finite() && ((assign42580_e57174) as f64).fract() == 0.0 { if assign42580_e57174 == 0.0 { 0.0 } else { (assign42580_e57174 * ((locals.var_t1).powf(assign42580_e57174 - 1.0) * locals.var_t1_dn9)) } } else { (assign42580_e57175 * (assign42580_e57174 * (locals.var_t1_dn9 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign42580_e57174) as f64).is_finite() && ((assign42580_e57174) as f64).fract() == 0.0 { if assign42580_e57174 == 0.0 { 0.0 } else { (assign42580_e57174 * ((locals.var_t1).powf(assign42580_e57174 - 1.0) * locals.var_t1_dn10)) } } else { (assign42580_e57175 * (assign42580_e57174 * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign42580_e57174) as f64).is_finite() && ((assign42580_e57174) as f64).fract() == 0.0 { if assign42580_e57174 == 0.0 { 0.0 } else { (assign42580_e57174 * ((locals.var_t1).powf(assign42580_e57174 - 1.0) * locals.var_t1_dn11)) } } else { (assign42580_e57175 * (assign42580_e57174 * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign42580_e57174) as f64).is_finite() && ((assign42580_e57174) as f64).fract() == 0.0 { if assign42580_e57174 == 0.0 { 0.0 } else { (assign42580_e57174 * ((locals.var_t1).powf(assign42580_e57174 - 1.0) * locals.var_t1_dn14)) } } else { (assign42580_e57175 * (assign42580_e57174 * (locals.var_t1_dn14 / locals.var_t1))) },)
            }
        };
        (assign42580_e57176, assign42580_e57176_d_n0, assign42580_e57176_d_n2, assign42580_e57176_d_n4, assign42580_e57176_d_n5, assign42580_e57176_d_n6, assign42580_e57176_d_n7, assign42580_e57176_d_n8, assign42580_e57176_d_n9, assign42580_e57176_d_n10, assign42580_e57176_d_n11, assign42580_e57176_d_n14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign42580_e57178;
        locals.var_t3_dn0 = assign42580_e57178_d_n0;
        locals.var_t3_dn2 = assign42580_e57178_d_n2;
        locals.var_t3_dn4 = assign42580_e57178_d_n4;
        locals.var_t3_dn5 = assign42580_e57178_d_n5;
        locals.var_t3_dn6 = assign42580_e57178_d_n6;
        locals.var_t3_dn7 = assign42580_e57178_d_n7;
        locals.var_t3_dn8 = assign42580_e57178_d_n8;
        locals.var_t3_dn9 = assign42580_e57178_d_n9;
        locals.var_t3_dn10 = assign42580_e57178_d_n10;
        locals.var_t3_dn11 = assign42580_e57178_d_n11;
        locals.var_t3_dn14 = assign42580_e57178_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign42590_e57189, assign42590_e57189_d_n0, assign42590_e57189_d_n2, assign42590_e57189_d_n4, assign42590_e57189_d_n5, assign42590_e57189_d_n6, assign42590_e57189_d_n7, assign42590_e57189_d_n8, assign42590_e57189_d_n9, assign42590_e57189_d_n10, assign42590_e57189_d_n11, assign42590_e57189_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        let assign42590_e57187: f64 = (locals.var_t1 * locals.var_t3);
        (assign42590_e57187, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)), ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)), ((locals.var_t1_dn9 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn9)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)), ((locals.var_t1_dn14 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign42590_e57189;
        locals.var_t2_dn0 = assign42590_e57189_d_n0;
        locals.var_t2_dn2 = assign42590_e57189_d_n2;
        locals.var_t2_dn4 = assign42590_e57189_d_n4;
        locals.var_t2_dn5 = assign42590_e57189_d_n5;
        locals.var_t2_dn6 = assign42590_e57189_d_n6;
        locals.var_t2_dn7 = assign42590_e57189_d_n7;
        locals.var_t2_dn8 = assign42590_e57189_d_n8;
        locals.var_t2_dn9 = assign42590_e57189_d_n9;
        locals.var_t2_dn10 = assign42590_e57189_d_n10;
        locals.var_t2_dn11 = assign42590_e57189_d_n11;
        locals.var_t2_dn14 = assign42590_e57189_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign42600_e57200, assign42600_e57200_d_n0, assign42600_e57200_d_n2, assign42600_e57200_d_n4, assign42600_e57200_d_n5, assign42600_e57200_d_n6, assign42600_e57200_d_n7, assign42600_e57200_d_n8, assign42600_e57200_d_n9, assign42600_e57200_d_n10, assign42600_e57200_d_n11, assign42600_e57200_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        let assign42600_e57198: f64 = (1.0 + locals.var_t2);
        (assign42600_e57198, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign42600_e57200;
        locals.var_t4_dn0 = assign42600_e57200_d_n0;
        locals.var_t4_dn2 = assign42600_e57200_d_n2;
        locals.var_t4_dn4 = assign42600_e57200_d_n4;
        locals.var_t4_dn5 = assign42600_e57200_d_n5;
        locals.var_t4_dn6 = assign42600_e57200_d_n6;
        locals.var_t4_dn7 = assign42600_e57200_d_n7;
        locals.var_t4_dn8 = assign42600_e57200_d_n8;
        locals.var_t4_dn9 = assign42600_e57200_d_n9;
        locals.var_t4_dn10 = assign42600_e57200_d_n10;
        locals.var_t4_dn11 = assign42600_e57200_d_n11;
        locals.var_t4_dn14 = assign42600_e57200_d_n14;
        locals.var_t4_rv = 0.0;

        let assign42610_e57204: f64 = (10.0 * 2.220446049250313e-16);
        let assign42610_e57205: f64 = (1.0 - assign42610_e57204);
        let assign42610_e57212: f64 = (10.0 * 2.220446049250313e-16);
        let assign42610_e57213: f64 = (1.0 + assign42610_e57212);
        let assign42610_e57215: f64 = if ((assign42610_e57205 <= p.p178) && (p.p178 <= assign42610_e57213)) { 1.0 } else { 0.0 };
        locals.var_guard1059 = assign42610_e57215;
        locals.var_guard1059_rv = 0.0;

        let (assign42620_e57228, assign42620_e57228_d_n0, assign42620_e57228_d_n2, assign42620_e57228_d_n4, assign42620_e57228_d_n5, assign42620_e57228_d_n6, assign42620_e57228_d_n7, assign42620_e57228_d_n8, assign42620_e57228_d_n9, assign42620_e57228_d_n10, assign42620_e57228_d_n11, assign42620_e57228_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1059 != 0.0)) {
        let assign42620_e57226: f64 = (1.0 / locals.var_t4);
        (assign42620_e57226, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn14 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign42620_e57228;
        locals.var_t5_dn0 = assign42620_e57228_d_n0;
        locals.var_t5_dn2 = assign42620_e57228_d_n2;
        locals.var_t5_dn4 = assign42620_e57228_d_n4;
        locals.var_t5_dn5 = assign42620_e57228_d_n5;
        locals.var_t5_dn6 = assign42620_e57228_d_n6;
        locals.var_t5_dn7 = assign42620_e57228_d_n7;
        locals.var_t5_dn8 = assign42620_e57228_d_n8;
        locals.var_t5_dn9 = assign42620_e57228_d_n9;
        locals.var_t5_dn10 = assign42620_e57228_d_n10;
        locals.var_t5_dn11 = assign42620_e57228_d_n11;
        locals.var_t5_dn14 = assign42620_e57228_d_n14;
        locals.var_t5_rv = 0.0;

        let assign42630_e57232: f64 = (10.0 * 2.220446049250313e-16);
        let assign42630_e57233: f64 = (2.0 - assign42630_e57232);
        let assign42630_e57240: f64 = (10.0 * 2.220446049250313e-16);
        let assign42630_e57241: f64 = (2.0 + assign42630_e57240);
        let assign42630_e57243: f64 = if ((assign42630_e57233 <= p.p178) && (p.p178 <= assign42630_e57241)) { 1.0 } else { 0.0 };
        locals.var_guard1060 = assign42630_e57243;
        locals.var_guard1060_rv = 0.0;

        let (assign42640_e57260, assign42640_e57260_d_n0, assign42640_e57260_d_n2, assign42640_e57260_d_n4, assign42640_e57260_d_n5, assign42640_e57260_d_n6, assign42640_e57260_d_n7, assign42640_e57260_d_n8, assign42640_e57260_d_n9, assign42640_e57260_d_n10, assign42640_e57260_d_n11, assign42640_e57260_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1059 == 0.0)) && (locals.var_guard1060 != 0.0)) {
        let assign42640_e57257: f64 = (locals.var_t4).sqrt();
        let assign42640_e57258: f64 = (1.0 / assign42640_e57257);
        (assign42640_e57258, (-((locals.var_t4_dn0 / (2.0 * assign42640_e57257)) / (assign42640_e57257 * assign42640_e57257))), (-((locals.var_t4_dn2 / (2.0 * assign42640_e57257)) / (assign42640_e57257 * assign42640_e57257))), (-((locals.var_t4_dn4 / (2.0 * assign42640_e57257)) / (assign42640_e57257 * assign42640_e57257))), (-((locals.var_t4_dn5 / (2.0 * assign42640_e57257)) / (assign42640_e57257 * assign42640_e57257))), (-((locals.var_t4_dn6 / (2.0 * assign42640_e57257)) / (assign42640_e57257 * assign42640_e57257))), (-((locals.var_t4_dn7 / (2.0 * assign42640_e57257)) / (assign42640_e57257 * assign42640_e57257))), (-((locals.var_t4_dn8 / (2.0 * assign42640_e57257)) / (assign42640_e57257 * assign42640_e57257))), (-((locals.var_t4_dn9 / (2.0 * assign42640_e57257)) / (assign42640_e57257 * assign42640_e57257))), (-((locals.var_t4_dn10 / (2.0 * assign42640_e57257)) / (assign42640_e57257 * assign42640_e57257))), (-((locals.var_t4_dn11 / (2.0 * assign42640_e57257)) / (assign42640_e57257 * assign42640_e57257))), (-((locals.var_t4_dn14 / (2.0 * assign42640_e57257)) / (assign42640_e57257 * assign42640_e57257))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign42640_e57260;
        locals.var_t5_dn0 = assign42640_e57260_d_n0;
        locals.var_t5_dn2 = assign42640_e57260_d_n2;
        locals.var_t5_dn4 = assign42640_e57260_d_n4;
        locals.var_t5_dn5 = assign42640_e57260_d_n5;
        locals.var_t5_dn6 = assign42640_e57260_d_n6;
        locals.var_t5_dn7 = assign42640_e57260_d_n7;
        locals.var_t5_dn8 = assign42640_e57260_d_n8;
        locals.var_t5_dn9 = assign42640_e57260_d_n9;
        locals.var_t5_dn10 = assign42640_e57260_d_n10;
        locals.var_t5_dn11 = assign42640_e57260_d_n11;
        locals.var_t5_dn14 = assign42640_e57260_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign42650_e57287, assign42650_e57287_d_n0, assign42650_e57287_d_n2, assign42650_e57287_d_n4, assign42650_e57287_d_n5, assign42650_e57287_d_n6, assign42650_e57287_d_n7, assign42650_e57287_d_n8, assign42650_e57287_d_n9, assign42650_e57287_d_n10, assign42650_e57287_d_n11, assign42650_e57287_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1059 == 0.0)) && (locals.var_guard1060 == 0.0)) {
        let (assign42650_e57285, assign42650_e57285_d_n0, assign42650_e57285_d_n2, assign42650_e57285_d_n4, assign42650_e57285_d_n5, assign42650_e57285_d_n6, assign42650_e57285_d_n7, assign42650_e57285_d_n8, assign42650_e57285_d_n9, assign42650_e57285_d_n10, assign42650_e57285_d_n11, assign42650_e57285_d_n14,) = {
            if (locals.var_t4 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign42650_e57279: f64 = (-1.0);
                let assign42650_e57281: f64 = (assign42650_e57279 / p.p178);
                let assign42650_e57283: f64 = (assign42650_e57281 - 1.0);
                let assign42650_e57284: f64 = (locals.var_t4).powf(assign42650_e57283);
                (assign42650_e57284, if 0.0 == 0.0 && ((assign42650_e57283) as f64).is_finite() && ((assign42650_e57283) as f64).fract() == 0.0 { if assign42650_e57283 == 0.0 { 0.0 } else { (assign42650_e57283 * ((locals.var_t4).powf(assign42650_e57283 - 1.0) * locals.var_t4_dn0)) } } else { (assign42650_e57284 * (assign42650_e57283 * (locals.var_t4_dn0 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign42650_e57283) as f64).is_finite() && ((assign42650_e57283) as f64).fract() == 0.0 { if assign42650_e57283 == 0.0 { 0.0 } else { (assign42650_e57283 * ((locals.var_t4).powf(assign42650_e57283 - 1.0) * locals.var_t4_dn2)) } } else { (assign42650_e57284 * (assign42650_e57283 * (locals.var_t4_dn2 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign42650_e57283) as f64).is_finite() && ((assign42650_e57283) as f64).fract() == 0.0 { if assign42650_e57283 == 0.0 { 0.0 } else { (assign42650_e57283 * ((locals.var_t4).powf(assign42650_e57283 - 1.0) * locals.var_t4_dn4)) } } else { (assign42650_e57284 * (assign42650_e57283 * (locals.var_t4_dn4 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign42650_e57283) as f64).is_finite() && ((assign42650_e57283) as f64).fract() == 0.0 { if assign42650_e57283 == 0.0 { 0.0 } else { (assign42650_e57283 * ((locals.var_t4).powf(assign42650_e57283 - 1.0) * locals.var_t4_dn5)) } } else { (assign42650_e57284 * (assign42650_e57283 * (locals.var_t4_dn5 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign42650_e57283) as f64).is_finite() && ((assign42650_e57283) as f64).fract() == 0.0 { if assign42650_e57283 == 0.0 { 0.0 } else { (assign42650_e57283 * ((locals.var_t4).powf(assign42650_e57283 - 1.0) * locals.var_t4_dn6)) } } else { (assign42650_e57284 * (assign42650_e57283 * (locals.var_t4_dn6 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign42650_e57283) as f64).is_finite() && ((assign42650_e57283) as f64).fract() == 0.0 { if assign42650_e57283 == 0.0 { 0.0 } else { (assign42650_e57283 * ((locals.var_t4).powf(assign42650_e57283 - 1.0) * locals.var_t4_dn7)) } } else { (assign42650_e57284 * (assign42650_e57283 * (locals.var_t4_dn7 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign42650_e57283) as f64).is_finite() && ((assign42650_e57283) as f64).fract() == 0.0 { if assign42650_e57283 == 0.0 { 0.0 } else { (assign42650_e57283 * ((locals.var_t4).powf(assign42650_e57283 - 1.0) * locals.var_t4_dn8)) } } else { (assign42650_e57284 * (assign42650_e57283 * (locals.var_t4_dn8 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign42650_e57283) as f64).is_finite() && ((assign42650_e57283) as f64).fract() == 0.0 { if assign42650_e57283 == 0.0 { 0.0 } else { (assign42650_e57283 * ((locals.var_t4).powf(assign42650_e57283 - 1.0) * locals.var_t4_dn9)) } } else { (assign42650_e57284 * (assign42650_e57283 * (locals.var_t4_dn9 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign42650_e57283) as f64).is_finite() && ((assign42650_e57283) as f64).fract() == 0.0 { if assign42650_e57283 == 0.0 { 0.0 } else { (assign42650_e57283 * ((locals.var_t4).powf(assign42650_e57283 - 1.0) * locals.var_t4_dn10)) } } else { (assign42650_e57284 * (assign42650_e57283 * (locals.var_t4_dn10 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign42650_e57283) as f64).is_finite() && ((assign42650_e57283) as f64).fract() == 0.0 { if assign42650_e57283 == 0.0 { 0.0 } else { (assign42650_e57283 * ((locals.var_t4).powf(assign42650_e57283 - 1.0) * locals.var_t4_dn11)) } } else { (assign42650_e57284 * (assign42650_e57283 * (locals.var_t4_dn11 / locals.var_t4))) }, if 0.0 == 0.0 && ((assign42650_e57283) as f64).is_finite() && ((assign42650_e57283) as f64).fract() == 0.0 { if assign42650_e57283 == 0.0 { 0.0 } else { (assign42650_e57283 * ((locals.var_t4).powf(assign42650_e57283 - 1.0) * locals.var_t4_dn14)) } } else { (assign42650_e57284 * (assign42650_e57283 * (locals.var_t4_dn14 / locals.var_t4))) },)
            }
        };
        (assign42650_e57285, assign42650_e57285_d_n0, assign42650_e57285_d_n2, assign42650_e57285_d_n4, assign42650_e57285_d_n5, assign42650_e57285_d_n6, assign42650_e57285_d_n7, assign42650_e57285_d_n8, assign42650_e57285_d_n9, assign42650_e57285_d_n10, assign42650_e57285_d_n11, assign42650_e57285_d_n14,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign42650_e57287;
        locals.var_t6_dn0 = assign42650_e57287_d_n0;
        locals.var_t6_dn2 = assign42650_e57287_d_n2;
        locals.var_t6_dn4 = assign42650_e57287_d_n4;
        locals.var_t6_dn5 = assign42650_e57287_d_n5;
        locals.var_t6_dn6 = assign42650_e57287_d_n6;
        locals.var_t6_dn7 = assign42650_e57287_d_n7;
        locals.var_t6_dn8 = assign42650_e57287_d_n8;
        locals.var_t6_dn9 = assign42650_e57287_d_n9;
        locals.var_t6_dn10 = assign42650_e57287_d_n10;
        locals.var_t6_dn11 = assign42650_e57287_d_n11;
        locals.var_t6_dn14 = assign42650_e57287_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign42660_e57304, assign42660_e57304_d_n0, assign42660_e57304_d_n2, assign42660_e57304_d_n4, assign42660_e57304_d_n5, assign42660_e57304_d_n6, assign42660_e57304_d_n7, assign42660_e57304_d_n8, assign42660_e57304_d_n9, assign42660_e57304_d_n10, assign42660_e57304_d_n11, assign42660_e57304_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1059 == 0.0)) && (locals.var_guard1060 == 0.0)) {
        let assign42660_e57302: f64 = (locals.var_t4 * locals.var_t6);
        (assign42660_e57302, ((locals.var_t4_dn0 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn0)), ((locals.var_t4_dn2 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn2)), ((locals.var_t4_dn4 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn4)), ((locals.var_t4_dn5 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn5)), ((locals.var_t4_dn6 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn6)), ((locals.var_t4_dn7 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn7)), ((locals.var_t4_dn8 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn8)), ((locals.var_t4_dn9 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn9)), ((locals.var_t4_dn10 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn10)), ((locals.var_t4_dn11 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn11)), ((locals.var_t4_dn14 * locals.var_t6) + (locals.var_t4 * locals.var_t6_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign42660_e57304;
        locals.var_t5_dn0 = assign42660_e57304_d_n0;
        locals.var_t5_dn2 = assign42660_e57304_d_n2;
        locals.var_t5_dn4 = assign42660_e57304_d_n4;
        locals.var_t5_dn5 = assign42660_e57304_d_n5;
        locals.var_t5_dn6 = assign42660_e57304_d_n6;
        locals.var_t5_dn7 = assign42660_e57304_d_n7;
        locals.var_t5_dn8 = assign42660_e57304_d_n8;
        locals.var_t5_dn9 = assign42660_e57304_d_n9;
        locals.var_t5_dn10 = assign42660_e57304_d_n10;
        locals.var_t5_dn11 = assign42660_e57304_d_n11;
        locals.var_t5_dn14 = assign42660_e57304_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign42670_e57315, assign42670_e57315_d_n0, assign42670_e57315_d_n2, assign42670_e57315_d_n4, assign42670_e57315_d_n5, assign42670_e57315_d_n6, assign42670_e57315_d_n7, assign42670_e57315_d_n8, assign42670_e57315_d_n9, assign42670_e57315_d_n10, assign42670_e57315_d_n11, assign42670_e57315_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        let assign42670_e57313: f64 = (locals.var_muun * locals.var_t5);
        (assign42670_e57313, ((locals.var_muun_dn0 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn0)), ((locals.var_muun_dn2 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn2)), ((locals.var_muun_dn4 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn4)), ((locals.var_muun_dn5 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn5)), ((locals.var_muun_dn6 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn6)), ((locals.var_muun_dn7 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn7)), ((locals.var_muun_dn8 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn8)), ((locals.var_muun_dn9 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn9)), ((locals.var_muun_dn10 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn10)), ((locals.var_muun_dn11 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn11)), ((locals.var_muun_dn14 * locals.var_t5) + (locals.var_muun * locals.var_t5_dn14)),)
    } else {
        (locals.var_mu, locals.var_mu_dn0, locals.var_mu_dn2, locals.var_mu_dn4, locals.var_mu_dn5, locals.var_mu_dn6, locals.var_mu_dn7, locals.var_mu_dn8, locals.var_mu_dn9, locals.var_mu_dn10, locals.var_mu_dn11, locals.var_mu_dn14,)
    }
};
        locals.var_mu = assign42670_e57315;
        locals.var_mu_dn0 = assign42670_e57315_d_n0;
        locals.var_mu_dn2 = assign42670_e57315_d_n2;
        locals.var_mu_dn4 = assign42670_e57315_d_n4;
        locals.var_mu_dn5 = assign42670_e57315_d_n5;
        locals.var_mu_dn6 = assign42670_e57315_d_n6;
        locals.var_mu_dn7 = assign42670_e57315_d_n7;
        locals.var_mu_dn8 = assign42670_e57315_d_n8;
        locals.var_mu_dn9 = assign42670_e57315_d_n9;
        locals.var_mu_dn10 = assign42670_e57315_d_n10;
        locals.var_mu_dn11 = assign42670_e57315_d_n11;
        locals.var_mu_dn14 = assign42670_e57315_d_n14;
        locals.var_mu_rv = 0.0;

        let (assign42680_e57324, assign42680_e57324_d_n0, assign42680_e57324_d_n2, assign42680_e57324_d_n4, assign42680_e57324_d_n5, assign42680_e57324_d_n6, assign42680_e57324_d_n7, assign42680_e57324_d_n8, assign42680_e57324_d_n9, assign42680_e57324_d_n10, assign42680_e57324_d_n11, assign42680_e57324_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        (locals.var_mu, locals.var_mu_dn0, locals.var_mu_dn2, locals.var_mu_dn4, locals.var_mu_dn5, locals.var_mu_dn6, locals.var_mu_dn7, locals.var_mu_dn8, locals.var_mu_dn9, locals.var_mu_dn10, locals.var_mu_dn11, locals.var_mu_dn14,)
    } else {
        (locals.var_mu_acc, locals.var_mu_acc_dn0, locals.var_mu_acc_dn2, locals.var_mu_acc_dn4, locals.var_mu_acc_dn5, locals.var_mu_acc_dn6, locals.var_mu_acc_dn7, locals.var_mu_acc_dn8, locals.var_mu_acc_dn9, locals.var_mu_acc_dn10, locals.var_mu_acc_dn11, locals.var_mu_acc_dn14,)
    }
};
        locals.var_mu_acc = assign42680_e57324;
        locals.var_mu_acc_dn0 = assign42680_e57324_d_n0;
        locals.var_mu_acc_dn2 = assign42680_e57324_d_n2;
        locals.var_mu_acc_dn4 = assign42680_e57324_d_n4;
        locals.var_mu_acc_dn5 = assign42680_e57324_d_n5;
        locals.var_mu_acc_dn6 = assign42680_e57324_d_n6;
        locals.var_mu_acc_dn7 = assign42680_e57324_d_n7;
        locals.var_mu_acc_dn8 = assign42680_e57324_d_n8;
        locals.var_mu_acc_dn9 = assign42680_e57324_d_n9;
        locals.var_mu_acc_dn10 = assign42680_e57324_d_n10;
        locals.var_mu_acc_dn11 = assign42680_e57324_d_n11;
        locals.var_mu_acc_dn14 = assign42680_e57324_d_n14;
        locals.var_mu_acc_rv = 0.0;

        let (assign42690_e57333, assign42690_e57333_d_n0, assign42690_e57333_d_n2, assign42690_e57333_d_n4, assign42690_e57333_d_n5, assign42690_e57333_d_n6, assign42690_e57333_d_n7, assign42690_e57333_d_n8, assign42690_e57333_d_n9, assign42690_e57333_d_n10, assign42690_e57333_d_n11, assign42690_e57333_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        (locals.var_ey, locals.var_ey_dn0, locals.var_ey_dn2, locals.var_ey_dn4, locals.var_ey_dn5, locals.var_ey_dn6, locals.var_ey_dn7, locals.var_ey_dn8, locals.var_ey_dn9, locals.var_ey_dn10, locals.var_ey_dn11, locals.var_ey_dn14,)
    } else {
        (locals.var_ey_acc, locals.var_ey_acc_dn0, locals.var_ey_acc_dn2, locals.var_ey_acc_dn4, locals.var_ey_acc_dn5, locals.var_ey_acc_dn6, locals.var_ey_acc_dn7, locals.var_ey_acc_dn8, locals.var_ey_acc_dn9, locals.var_ey_acc_dn10, locals.var_ey_acc_dn11, locals.var_ey_acc_dn14,)
    }
};
        locals.var_ey_acc = assign42690_e57333;
        locals.var_ey_acc_dn0 = assign42690_e57333_d_n0;
        locals.var_ey_acc_dn2 = assign42690_e57333_d_n2;
        locals.var_ey_acc_dn4 = assign42690_e57333_d_n4;
        locals.var_ey_acc_dn5 = assign42690_e57333_d_n5;
        locals.var_ey_acc_dn6 = assign42690_e57333_d_n6;
        locals.var_ey_acc_dn7 = assign42690_e57333_d_n7;
        locals.var_ey_acc_dn8 = assign42690_e57333_d_n8;
        locals.var_ey_acc_dn9 = assign42690_e57333_d_n9;
        locals.var_ey_acc_dn10 = assign42690_e57333_d_n10;
        locals.var_ey_acc_dn11 = assign42690_e57333_d_n11;
        locals.var_ey_acc_dn14 = assign42690_e57333_d_n14;
        locals.var_ey_acc_rv = 0.0;

        let (assign42700_e57342, assign42700_e57342_d_n0, assign42700_e57342_d_n2, assign42700_e57342_d_n4, assign42700_e57342_d_n5, assign42700_e57342_d_n6, assign42700_e57342_d_n7, assign42700_e57342_d_n8, assign42700_e57342_d_n9, assign42700_e57342_d_n10, assign42700_e57342_d_n11, assign42700_e57342_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        (locals.var_vdsorg, locals.var_vdsorg_dn0, locals.var_vdsorg_dn2, locals.var_vdsorg_dn4, locals.var_vdsorg_dn5, locals.var_vdsorg_dn6, locals.var_vdsorg_dn7, locals.var_vdsorg_dn8, locals.var_vdsorg_dn9, locals.var_vdsorg_dn10, locals.var_vdsorg_dn11, locals.var_vdsorg_dn14,)
    } else {
        (locals.var_vds_res, locals.var_vds_res_dn0, locals.var_vds_res_dn2, locals.var_vds_res_dn4, locals.var_vds_res_dn5, locals.var_vds_res_dn6, locals.var_vds_res_dn7, locals.var_vds_res_dn8, locals.var_vds_res_dn9, locals.var_vds_res_dn10, locals.var_vds_res_dn11, locals.var_vds_res_dn14,)
    }
};
        locals.var_vds_res = assign42700_e57342;
        locals.var_vds_res_dn0 = assign42700_e57342_d_n0;
        locals.var_vds_res_dn2 = assign42700_e57342_d_n2;
        locals.var_vds_res_dn4 = assign42700_e57342_d_n4;
        locals.var_vds_res_dn5 = assign42700_e57342_d_n5;
        locals.var_vds_res_dn6 = assign42700_e57342_d_n6;
        locals.var_vds_res_dn7 = assign42700_e57342_d_n7;
        locals.var_vds_res_dn8 = assign42700_e57342_d_n8;
        locals.var_vds_res_dn9 = assign42700_e57342_d_n9;
        locals.var_vds_res_dn10 = assign42700_e57342_d_n10;
        locals.var_vds_res_dn11 = assign42700_e57342_d_n11;
        locals.var_vds_res_dn14 = assign42700_e57342_d_n14;
        locals.var_vds_res_rv = 0.0;

        let assign42710_e57345: f64 = if locals.var_vdsorg > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard1061 = assign42710_e57345;
        locals.var_guard1061_rv = 0.0;

        let (assign42720_e57360, assign42720_e57360_d_n0, assign42720_e57360_d_n2, assign42720_e57360_d_n4, assign42720_e57360_d_n5, assign42720_e57360_d_n6, assign42720_e57360_d_n7, assign42720_e57360_d_n8, assign42720_e57360_d_n9, assign42720_e57360_d_n10, assign42720_e57360_d_n11, assign42720_e57360_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) {
        let assign42720_e57356: f64 = (locals.var_vbsc + locals.var_beta_inv);
        let assign42720_e57358: f64 = (assign42720_e57356 * p.p396);
        (assign42720_e57358, ((locals.var_vbsc_dn0 + locals.var_beta_inv_dn0) * p.p396), ((locals.var_vbsc_dn2 + locals.var_beta_inv_dn2) * p.p396), ((locals.var_vbsc_dn4 + locals.var_beta_inv_dn4) * p.p396), ((locals.var_vbsc_dn5 + locals.var_beta_inv_dn5) * p.p396), ((locals.var_vbsc_dn6 + locals.var_beta_inv_dn6) * p.p396), ((locals.var_vbsc_dn7 + locals.var_beta_inv_dn7) * p.p396), ((locals.var_vbsc_dn8 + locals.var_beta_inv_dn8) * p.p396), ((locals.var_vbsc_dn9 + locals.var_beta_inv_dn9) * p.p396), ((locals.var_vbsc_dn10 + locals.var_beta_inv_dn10) * p.p396), ((locals.var_vbsc_dn11 + locals.var_beta_inv_dn11) * p.p396), ((locals.var_vbsc_dn14 + locals.var_beta_inv_dn14) * p.p396),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign42720_e57360;
        locals.var_t10_dn0 = assign42720_e57360_d_n0;
        locals.var_t10_dn2 = assign42720_e57360_d_n2;
        locals.var_t10_dn4 = assign42720_e57360_d_n4;
        locals.var_t10_dn5 = assign42720_e57360_d_n5;
        locals.var_t10_dn6 = assign42720_e57360_d_n6;
        locals.var_t10_dn7 = assign42720_e57360_d_n7;
        locals.var_t10_dn8 = assign42720_e57360_d_n8;
        locals.var_t10_dn9 = assign42720_e57360_d_n9;
        locals.var_t10_dn10 = assign42720_e57360_d_n10;
        locals.var_t10_dn11 = assign42720_e57360_d_n11;
        locals.var_t10_dn14 = assign42720_e57360_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign42730_e57377, assign42730_e57377_d_n0, assign42730_e57377_d_n2, assign42730_e57377_d_n4, assign42730_e57377_d_n5, assign42730_e57377_d_n6, assign42730_e57377_d_n7, assign42730_e57377_d_n8, assign42730_e57377_d_n9, assign42730_e57377_d_n10, assign42730_e57377_d_n11, assign42730_e57377_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) {
        let assign42730_e57373: f64 = (locals.var_vgp - locals.var_t10);
        let assign42730_e57374: f64 = (locals.var_c2_q_ndepm_esi_cox_inv2 * assign42730_e57373);
        let assign42730_e57375: f64 = (1.0 + assign42730_e57374);
        (assign42730_e57375, ((locals.var_c2_q_ndepm_esi_cox_inv2_dn0 * assign42730_e57373) + (locals.var_c2_q_ndepm_esi_cox_inv2 * (locals.var_vgp_dn0 - locals.var_t10_dn0))), ((locals.var_c2_q_ndepm_esi_cox_inv2_dn2 * assign42730_e57373) + (locals.var_c2_q_ndepm_esi_cox_inv2 * (locals.var_vgp_dn2 - locals.var_t10_dn2))), ((locals.var_c2_q_ndepm_esi_cox_inv2_dn4 * assign42730_e57373) + (locals.var_c2_q_ndepm_esi_cox_inv2 * (locals.var_vgp_dn4 - locals.var_t10_dn4))), ((locals.var_c2_q_ndepm_esi_cox_inv2_dn5 * assign42730_e57373) + (locals.var_c2_q_ndepm_esi_cox_inv2 * (locals.var_vgp_dn5 - locals.var_t10_dn5))), ((locals.var_c2_q_ndepm_esi_cox_inv2_dn6 * assign42730_e57373) + (locals.var_c2_q_ndepm_esi_cox_inv2 * (locals.var_vgp_dn6 - locals.var_t10_dn6))), ((locals.var_c2_q_ndepm_esi_cox_inv2_dn7 * assign42730_e57373) + (locals.var_c2_q_ndepm_esi_cox_inv2 * (locals.var_vgp_dn7 - locals.var_t10_dn7))), ((locals.var_c2_q_ndepm_esi_cox_inv2_dn8 * assign42730_e57373) + (locals.var_c2_q_ndepm_esi_cox_inv2 * (locals.var_vgp_dn8 - locals.var_t10_dn8))), ((locals.var_c2_q_ndepm_esi_cox_inv2_dn9 * assign42730_e57373) + (locals.var_c2_q_ndepm_esi_cox_inv2 * (locals.var_vgp_dn9 - locals.var_t10_dn9))), ((locals.var_c2_q_ndepm_esi_cox_inv2_dn10 * assign42730_e57373) + (locals.var_c2_q_ndepm_esi_cox_inv2 * (locals.var_vgp_dn10 - locals.var_t10_dn10))), ((locals.var_c2_q_ndepm_esi_cox_inv2_dn11 * assign42730_e57373) + (locals.var_c2_q_ndepm_esi_cox_inv2 * (locals.var_vgp_dn11 - locals.var_t10_dn11))), ((locals.var_c2_q_ndepm_esi_cox_inv2_dn14 * assign42730_e57373) + (locals.var_c2_q_ndepm_esi_cox_inv2 * (locals.var_vgp_dn14 - locals.var_t10_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign42730_e57377;
        locals.var_t4_dn0 = assign42730_e57377_d_n0;
        locals.var_t4_dn2 = assign42730_e57377_d_n2;
        locals.var_t4_dn4 = assign42730_e57377_d_n4;
        locals.var_t4_dn5 = assign42730_e57377_d_n5;
        locals.var_t4_dn6 = assign42730_e57377_d_n6;
        locals.var_t4_dn7 = assign42730_e57377_d_n7;
        locals.var_t4_dn8 = assign42730_e57377_d_n8;
        locals.var_t4_dn9 = assign42730_e57377_d_n9;
        locals.var_t4_dn10 = assign42730_e57377_d_n10;
        locals.var_t4_dn11 = assign42730_e57377_d_n11;
        locals.var_t4_dn14 = assign42730_e57377_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign42740_e57390, assign42740_e57390_d_n0, assign42740_e57390_d_n2, assign42740_e57390_d_n4, assign42740_e57390_d_n5, assign42740_e57390_d_n6, assign42740_e57390_d_n7, assign42740_e57390_d_n8, assign42740_e57390_d_n9, assign42740_e57390_d_n10, assign42740_e57390_d_n11, assign42740_e57390_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) {
        let assign42740_e57388: f64 = (1.0 + locals.var_c2_q_ndepm_esi_cox_inv2);
        (assign42740_e57388, locals.var_c2_q_ndepm_esi_cox_inv2_dn0, locals.var_c2_q_ndepm_esi_cox_inv2_dn2, locals.var_c2_q_ndepm_esi_cox_inv2_dn4, locals.var_c2_q_ndepm_esi_cox_inv2_dn5, locals.var_c2_q_ndepm_esi_cox_inv2_dn6, locals.var_c2_q_ndepm_esi_cox_inv2_dn7, locals.var_c2_q_ndepm_esi_cox_inv2_dn8, locals.var_c2_q_ndepm_esi_cox_inv2_dn9, locals.var_c2_q_ndepm_esi_cox_inv2_dn10, locals.var_c2_q_ndepm_esi_cox_inv2_dn11, locals.var_c2_q_ndepm_esi_cox_inv2_dn14,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign42740_e57390;
        locals.var_t5_dn0 = assign42740_e57390_d_n0;
        locals.var_t5_dn2 = assign42740_e57390_d_n2;
        locals.var_t5_dn4 = assign42740_e57390_d_n4;
        locals.var_t5_dn5 = assign42740_e57390_d_n5;
        locals.var_t5_dn6 = assign42740_e57390_d_n6;
        locals.var_t5_dn7 = assign42740_e57390_d_n7;
        locals.var_t5_dn8 = assign42740_e57390_d_n8;
        locals.var_t5_dn9 = assign42740_e57390_d_n9;
        locals.var_t5_dn10 = assign42740_e57390_d_n10;
        locals.var_t5_dn11 = assign42740_e57390_d_n11;
        locals.var_t5_dn14 = assign42740_e57390_d_n14;
        locals.var_t5_rv = 0.0;

        let assign42750_e57394: f64 = locals.var_t5;
        let assign42750_e57399: f64 = if ((locals.var_t4 < assign42750_e57394) && (locals.var_t5 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1062 = assign42750_e57399;
        locals.var_guard1062_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_148(
        locals: &mut StampLocals,
    ) {
        let (assign42760_e57416, assign42760_e57416_d_n0, assign42760_e57416_d_n2, assign42760_e57416_d_n4, assign42760_e57416_d_n5, assign42760_e57416_d_n6, assign42760_e57416_d_n7, assign42760_e57416_d_n8, assign42760_e57416_d_n9, assign42760_e57416_d_n10, assign42760_e57416_d_n11, assign42760_e57416_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) {
        let assign42760_e57412: f64 = locals.var_t5;
        let assign42760_e57414: f64 = (assign42760_e57412 - locals.var_t4);
        (assign42760_e57414, (locals.var_t5_dn0 - locals.var_t4_dn0), (locals.var_t5_dn2 - locals.var_t4_dn2), (locals.var_t5_dn4 - locals.var_t4_dn4), (locals.var_t5_dn5 - locals.var_t4_dn5), (locals.var_t5_dn6 - locals.var_t4_dn6), (locals.var_t5_dn7 - locals.var_t4_dn7), (locals.var_t5_dn8 - locals.var_t4_dn8), (locals.var_t5_dn9 - locals.var_t4_dn9), (locals.var_t5_dn10 - locals.var_t4_dn10), (locals.var_t5_dn11 - locals.var_t4_dn11), (locals.var_t5_dn14 - locals.var_t4_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign42760_e57416;
        locals.var_tmf1_dn0 = assign42760_e57416_d_n0;
        locals.var_tmf1_dn2 = assign42760_e57416_d_n2;
        locals.var_tmf1_dn4 = assign42760_e57416_d_n4;
        locals.var_tmf1_dn5 = assign42760_e57416_d_n5;
        locals.var_tmf1_dn6 = assign42760_e57416_d_n6;
        locals.var_tmf1_dn7 = assign42760_e57416_d_n7;
        locals.var_tmf1_dn8 = assign42760_e57416_d_n8;
        locals.var_tmf1_dn9 = assign42760_e57416_d_n9;
        locals.var_tmf1_dn10 = assign42760_e57416_d_n10;
        locals.var_tmf1_dn11 = assign42760_e57416_d_n11;
        locals.var_tmf1_dn14 = assign42760_e57416_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign42770_e57431, assign42770_e57431_d_n0, assign42770_e57431_d_n2, assign42770_e57431_d_n4, assign42770_e57431_d_n5, assign42770_e57431_d_n6, assign42770_e57431_d_n7, assign42770_e57431_d_n8, assign42770_e57431_d_n9, assign42770_e57431_d_n10, assign42770_e57431_d_n11, assign42770_e57431_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) {
        let assign42770_e57429: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign42770_e57429, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign42770_e57431;
        locals.var_x2_dn0 = assign42770_e57431_d_n0;
        locals.var_x2_dn2 = assign42770_e57431_d_n2;
        locals.var_x2_dn4 = assign42770_e57431_d_n4;
        locals.var_x2_dn5 = assign42770_e57431_d_n5;
        locals.var_x2_dn6 = assign42770_e57431_d_n6;
        locals.var_x2_dn7 = assign42770_e57431_d_n7;
        locals.var_x2_dn8 = assign42770_e57431_d_n8;
        locals.var_x2_dn9 = assign42770_e57431_d_n9;
        locals.var_x2_dn10 = assign42770_e57431_d_n10;
        locals.var_x2_dn11 = assign42770_e57431_d_n11;
        locals.var_x2_dn14 = assign42770_e57431_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign42780_e57446, assign42780_e57446_d_n0, assign42780_e57446_d_n2, assign42780_e57446_d_n4, assign42780_e57446_d_n5, assign42780_e57446_d_n6, assign42780_e57446_d_n7, assign42780_e57446_d_n8, assign42780_e57446_d_n9, assign42780_e57446_d_n10, assign42780_e57446_d_n11, assign42780_e57446_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) {
        let assign42780_e57444: f64 = (locals.var_t5 * locals.var_t5);
        (assign42780_e57444, ((locals.var_t5_dn0 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn0)), ((locals.var_t5_dn2 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn2)), ((locals.var_t5_dn4 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn4)), ((locals.var_t5_dn5 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn5)), ((locals.var_t5_dn6 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn6)), ((locals.var_t5_dn7 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn7)), ((locals.var_t5_dn8 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn8)), ((locals.var_t5_dn9 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn9)), ((locals.var_t5_dn10 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn10)), ((locals.var_t5_dn11 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn11)), ((locals.var_t5_dn14 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn14)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign42780_e57446;
        locals.var_xmax2_dn0 = assign42780_e57446_d_n0;
        locals.var_xmax2_dn2 = assign42780_e57446_d_n2;
        locals.var_xmax2_dn4 = assign42780_e57446_d_n4;
        locals.var_xmax2_dn5 = assign42780_e57446_d_n5;
        locals.var_xmax2_dn6 = assign42780_e57446_d_n6;
        locals.var_xmax2_dn7 = assign42780_e57446_d_n7;
        locals.var_xmax2_dn8 = assign42780_e57446_d_n8;
        locals.var_xmax2_dn9 = assign42780_e57446_d_n9;
        locals.var_xmax2_dn10 = assign42780_e57446_d_n10;
        locals.var_xmax2_dn11 = assign42780_e57446_d_n11;
        locals.var_xmax2_dn14 = assign42780_e57446_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign42790_e57459, assign42790_e57459_d_n0, assign42790_e57459_d_n2, assign42790_e57459_d_n4, assign42790_e57459_d_n5, assign42790_e57459_d_n6, assign42790_e57459_d_n7, assign42790_e57459_d_n8, assign42790_e57459_d_n9, assign42790_e57459_d_n10, assign42790_e57459_d_n11, assign42790_e57459_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign42790_e57459;
        locals.var_xp_dn0 = assign42790_e57459_d_n0;
        locals.var_xp_dn2 = assign42790_e57459_d_n2;
        locals.var_xp_dn4 = assign42790_e57459_d_n4;
        locals.var_xp_dn5 = assign42790_e57459_d_n5;
        locals.var_xp_dn6 = assign42790_e57459_d_n6;
        locals.var_xp_dn7 = assign42790_e57459_d_n7;
        locals.var_xp_dn8 = assign42790_e57459_d_n8;
        locals.var_xp_dn9 = assign42790_e57459_d_n9;
        locals.var_xp_dn10 = assign42790_e57459_d_n10;
        locals.var_xp_dn11 = assign42790_e57459_d_n11;
        locals.var_xp_dn14 = assign42790_e57459_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign42800_e57472, assign42800_e57472_d_n0, assign42800_e57472_d_n2, assign42800_e57472_d_n4, assign42800_e57472_d_n5, assign42800_e57472_d_n6, assign42800_e57472_d_n7, assign42800_e57472_d_n8, assign42800_e57472_d_n9, assign42800_e57472_d_n10, assign42800_e57472_d_n11, assign42800_e57472_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign42800_e57472;
        locals.var_xmp_dn0 = assign42800_e57472_d_n0;
        locals.var_xmp_dn2 = assign42800_e57472_d_n2;
        locals.var_xmp_dn4 = assign42800_e57472_d_n4;
        locals.var_xmp_dn5 = assign42800_e57472_d_n5;
        locals.var_xmp_dn6 = assign42800_e57472_d_n6;
        locals.var_xmp_dn7 = assign42800_e57472_d_n7;
        locals.var_xmp_dn8 = assign42800_e57472_d_n8;
        locals.var_xmp_dn9 = assign42800_e57472_d_n9;
        locals.var_xmp_dn10 = assign42800_e57472_d_n10;
        locals.var_xmp_dn11 = assign42800_e57472_d_n11;
        locals.var_xmp_dn14 = assign42800_e57472_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign42810_e57485,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign42810_e57485;
        locals.var_m0_rv = 0.0;

        let (assign42820_e57498,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign42820_e57498;
        locals.var_mm_rv = 0.0;

        let (assign42830_e57511, assign42830_e57511_d_n0, assign42830_e57511_d_n2, assign42830_e57511_d_n4, assign42830_e57511_d_n5, assign42830_e57511_d_n6, assign42830_e57511_d_n7, assign42830_e57511_d_n8, assign42830_e57511_d_n9, assign42830_e57511_d_n10, assign42830_e57511_d_n11, assign42830_e57511_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign42830_e57511;
        locals.var_arg_dn0 = assign42830_e57511_d_n0;
        locals.var_arg_dn2 = assign42830_e57511_d_n2;
        locals.var_arg_dn4 = assign42830_e57511_d_n4;
        locals.var_arg_dn5 = assign42830_e57511_d_n5;
        locals.var_arg_dn6 = assign42830_e57511_d_n6;
        locals.var_arg_dn7 = assign42830_e57511_d_n7;
        locals.var_arg_dn8 = assign42830_e57511_d_n8;
        locals.var_arg_dn9 = assign42830_e57511_d_n9;
        locals.var_arg_dn10 = assign42830_e57511_d_n10;
        locals.var_arg_dn11 = assign42830_e57511_d_n11;
        locals.var_arg_dn14 = assign42830_e57511_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign42840_e57524, assign42840_e57524_d_n0, assign42840_e57524_d_n2, assign42840_e57524_d_n4, assign42840_e57524_d_n5, assign42840_e57524_d_n6, assign42840_e57524_d_n7, assign42840_e57524_d_n8, assign42840_e57524_d_n9, assign42840_e57524_d_n10, assign42840_e57524_d_n11, assign42840_e57524_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign42840_e57524;
        locals.var_dnm_dn0 = assign42840_e57524_d_n0;
        locals.var_dnm_dn2 = assign42840_e57524_d_n2;
        locals.var_dnm_dn4 = assign42840_e57524_d_n4;
        locals.var_dnm_dn5 = assign42840_e57524_d_n5;
        locals.var_dnm_dn6 = assign42840_e57524_d_n6;
        locals.var_dnm_dn7 = assign42840_e57524_d_n7;
        locals.var_dnm_dn8 = assign42840_e57524_d_n8;
        locals.var_dnm_dn9 = assign42840_e57524_d_n9;
        locals.var_dnm_dn10 = assign42840_e57524_d_n10;
        locals.var_dnm_dn11 = assign42840_e57524_d_n11;
        locals.var_dnm_dn14 = assign42840_e57524_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign42850_e57539, assign42850_e57539_d_n0, assign42850_e57539_d_n2, assign42850_e57539_d_n4, assign42850_e57539_d_n5, assign42850_e57539_d_n6, assign42850_e57539_d_n7, assign42850_e57539_d_n8, assign42850_e57539_d_n9, assign42850_e57539_d_n10, assign42850_e57539_d_n11, assign42850_e57539_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) {
        let assign42850_e57537: f64 = (locals.var_xp * locals.var_x2);
        (assign42850_e57537, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign42850_e57539;
        locals.var_xp_dn0 = assign42850_e57539_d_n0;
        locals.var_xp_dn2 = assign42850_e57539_d_n2;
        locals.var_xp_dn4 = assign42850_e57539_d_n4;
        locals.var_xp_dn5 = assign42850_e57539_d_n5;
        locals.var_xp_dn6 = assign42850_e57539_d_n6;
        locals.var_xp_dn7 = assign42850_e57539_d_n7;
        locals.var_xp_dn8 = assign42850_e57539_d_n8;
        locals.var_xp_dn9 = assign42850_e57539_d_n9;
        locals.var_xp_dn10 = assign42850_e57539_d_n10;
        locals.var_xp_dn11 = assign42850_e57539_d_n11;
        locals.var_xp_dn14 = assign42850_e57539_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign42860_e57554, assign42860_e57554_d_n0, assign42860_e57554_d_n2, assign42860_e57554_d_n4, assign42860_e57554_d_n5, assign42860_e57554_d_n6, assign42860_e57554_d_n7, assign42860_e57554_d_n8, assign42860_e57554_d_n9, assign42860_e57554_d_n10, assign42860_e57554_d_n11, assign42860_e57554_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) {
        let assign42860_e57552: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign42860_e57552, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign42860_e57554;
        locals.var_xmp_dn0 = assign42860_e57554_d_n0;
        locals.var_xmp_dn2 = assign42860_e57554_d_n2;
        locals.var_xmp_dn4 = assign42860_e57554_d_n4;
        locals.var_xmp_dn5 = assign42860_e57554_d_n5;
        locals.var_xmp_dn6 = assign42860_e57554_d_n6;
        locals.var_xmp_dn7 = assign42860_e57554_d_n7;
        locals.var_xmp_dn8 = assign42860_e57554_d_n8;
        locals.var_xmp_dn9 = assign42860_e57554_d_n9;
        locals.var_xmp_dn10 = assign42860_e57554_d_n10;
        locals.var_xmp_dn11 = assign42860_e57554_d_n11;
        locals.var_xmp_dn14 = assign42860_e57554_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign42870_e57569, assign42870_e57569_d_n0, assign42870_e57569_d_n2, assign42870_e57569_d_n4, assign42870_e57569_d_n5, assign42870_e57569_d_n6, assign42870_e57569_d_n7, assign42870_e57569_d_n8, assign42870_e57569_d_n9, assign42870_e57569_d_n10, assign42870_e57569_d_n11, assign42870_e57569_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) {
        let assign42870_e57567: f64 = (locals.var_xp * locals.var_x2);
        (assign42870_e57567, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign42870_e57569;
        locals.var_xp_dn0 = assign42870_e57569_d_n0;
        locals.var_xp_dn2 = assign42870_e57569_d_n2;
        locals.var_xp_dn4 = assign42870_e57569_d_n4;
        locals.var_xp_dn5 = assign42870_e57569_d_n5;
        locals.var_xp_dn6 = assign42870_e57569_d_n6;
        locals.var_xp_dn7 = assign42870_e57569_d_n7;
        locals.var_xp_dn8 = assign42870_e57569_d_n8;
        locals.var_xp_dn9 = assign42870_e57569_d_n9;
        locals.var_xp_dn10 = assign42870_e57569_d_n10;
        locals.var_xp_dn11 = assign42870_e57569_d_n11;
        locals.var_xp_dn14 = assign42870_e57569_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign42880_e57584, assign42880_e57584_d_n0, assign42880_e57584_d_n2, assign42880_e57584_d_n4, assign42880_e57584_d_n5, assign42880_e57584_d_n6, assign42880_e57584_d_n7, assign42880_e57584_d_n8, assign42880_e57584_d_n9, assign42880_e57584_d_n10, assign42880_e57584_d_n11, assign42880_e57584_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) {
        let assign42880_e57582: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign42880_e57582, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign42880_e57584;
        locals.var_xmp_dn0 = assign42880_e57584_d_n0;
        locals.var_xmp_dn2 = assign42880_e57584_d_n2;
        locals.var_xmp_dn4 = assign42880_e57584_d_n4;
        locals.var_xmp_dn5 = assign42880_e57584_d_n5;
        locals.var_xmp_dn6 = assign42880_e57584_d_n6;
        locals.var_xmp_dn7 = assign42880_e57584_d_n7;
        locals.var_xmp_dn8 = assign42880_e57584_d_n8;
        locals.var_xmp_dn9 = assign42880_e57584_d_n9;
        locals.var_xmp_dn10 = assign42880_e57584_d_n10;
        locals.var_xmp_dn11 = assign42880_e57584_d_n11;
        locals.var_xmp_dn14 = assign42880_e57584_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign42890_e57599, assign42890_e57599_d_n0, assign42890_e57599_d_n2, assign42890_e57599_d_n4, assign42890_e57599_d_n5, assign42890_e57599_d_n6, assign42890_e57599_d_n7, assign42890_e57599_d_n8, assign42890_e57599_d_n9, assign42890_e57599_d_n10, assign42890_e57599_d_n11, assign42890_e57599_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) {
        let assign42890_e57597: f64 = (locals.var_xp + locals.var_xmp);
        (assign42890_e57597, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign42890_e57599;
        locals.var_arg_dn0 = assign42890_e57599_d_n0;
        locals.var_arg_dn2 = assign42890_e57599_d_n2;
        locals.var_arg_dn4 = assign42890_e57599_d_n4;
        locals.var_arg_dn5 = assign42890_e57599_d_n5;
        locals.var_arg_dn6 = assign42890_e57599_d_n6;
        locals.var_arg_dn7 = assign42890_e57599_d_n7;
        locals.var_arg_dn8 = assign42890_e57599_d_n8;
        locals.var_arg_dn9 = assign42890_e57599_d_n9;
        locals.var_arg_dn10 = assign42890_e57599_d_n10;
        locals.var_arg_dn11 = assign42890_e57599_d_n11;
        locals.var_arg_dn14 = assign42890_e57599_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign42900_e57612, assign42900_e57612_d_n0, assign42900_e57612_d_n2, assign42900_e57612_d_n4, assign42900_e57612_d_n5, assign42900_e57612_d_n6, assign42900_e57612_d_n7, assign42900_e57612_d_n8, assign42900_e57612_d_n9, assign42900_e57612_d_n10, assign42900_e57612_d_n11, assign42900_e57612_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign42900_e57612;
        locals.var_dnm_dn0 = assign42900_e57612_d_n0;
        locals.var_dnm_dn2 = assign42900_e57612_d_n2;
        locals.var_dnm_dn4 = assign42900_e57612_d_n4;
        locals.var_dnm_dn5 = assign42900_e57612_d_n5;
        locals.var_dnm_dn6 = assign42900_e57612_d_n6;
        locals.var_dnm_dn7 = assign42900_e57612_d_n7;
        locals.var_dnm_dn8 = assign42900_e57612_d_n8;
        locals.var_dnm_dn9 = assign42900_e57612_d_n9;
        locals.var_dnm_dn10 = assign42900_e57612_d_n10;
        locals.var_dnm_dn11 = assign42900_e57612_d_n11;
        locals.var_dnm_dn14 = assign42900_e57612_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign42910_e57627: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1063 = assign42910_e57627;
        locals.var_guard1063_rv = 0.0;

        let assign42920_e57630: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1064 = assign42920_e57630;
        locals.var_guard1064_rv = 0.0;

        let (assign42930_e57647,) = {
    if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 != 0.0)) && (locals.var_guard1064 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign42930_e57647;
        locals.var_mm_rv = 0.0;

        let assign42940_e57650: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1065 = assign42940_e57650;
        locals.var_guard1065_rv = 0.0;

        let (assign42950_e57670,) = {
    if (((((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 != 0.0)) && (locals.var_guard1064 == 0.0)) && (locals.var_guard1065 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign42950_e57670;
        locals.var_mm_rv = 0.0;

        let assign42960_e57673: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1066 = assign42960_e57673;
        locals.var_guard1066_rv = 0.0;

        let (assign42970_e57696,) = {
    if ((((((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 != 0.0)) && (locals.var_guard1064 == 0.0)) && (locals.var_guard1065 == 0.0)) && (locals.var_guard1066 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign42970_e57696;
        locals.var_mm_rv = 0.0;

        let assign42980_e57699: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1067 = assign42980_e57699;
        locals.var_guard1067_rv = 0.0;

        let (assign42990_e57725,) = {
    if (((((((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 != 0.0)) && (locals.var_guard1064 == 0.0)) && (locals.var_guard1065 == 0.0)) && (locals.var_guard1066 == 0.0)) && (locals.var_guard1067 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign42990_e57725;
        locals.var_mm_rv = 0.0;

        let (assign43000_e57740,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign43000_e57740;
        locals.var_m0_rv = 0.0;

        let mut assign43010_loop_guard: usize = 0;
        while {
            let assign43010_cond_e57756: f64 = if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign43010_cond_e57756 != 0.0
        } {
            assign43010_loop_guard += 1;
            assert!(assign43010_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign43010_body0_e57772, assign43010_body0_e57772_d_n0, assign43010_body0_e57772_d_n2, assign43010_body0_e57772_d_n4, assign43010_body0_e57772_d_n5, assign43010_body0_e57772_d_n6, assign43010_body0_e57772_d_n7, assign43010_body0_e57772_d_n8, assign43010_body0_e57772_d_n9, assign43010_body0_e57772_d_n10, assign43010_body0_e57772_d_n11, assign43010_body0_e57772_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 != 0.0)) {
        let assign43010_body0_e57770: f64 = (locals.var_dnm).sqrt();
        (assign43010_body0_e57770, (locals.var_dnm_dn0 / (2.0 * assign43010_body0_e57770)), (locals.var_dnm_dn2 / (2.0 * assign43010_body0_e57770)), (locals.var_dnm_dn4 / (2.0 * assign43010_body0_e57770)), (locals.var_dnm_dn5 / (2.0 * assign43010_body0_e57770)), (locals.var_dnm_dn6 / (2.0 * assign43010_body0_e57770)), (locals.var_dnm_dn7 / (2.0 * assign43010_body0_e57770)), (locals.var_dnm_dn8 / (2.0 * assign43010_body0_e57770)), (locals.var_dnm_dn9 / (2.0 * assign43010_body0_e57770)), (locals.var_dnm_dn10 / (2.0 * assign43010_body0_e57770)), (locals.var_dnm_dn11 / (2.0 * assign43010_body0_e57770)), (locals.var_dnm_dn14 / (2.0 * assign43010_body0_e57770)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign43010_body0_e57772;
            locals.var_dnm_dn0 = assign43010_body0_e57772_d_n0;
            locals.var_dnm_dn2 = assign43010_body0_e57772_d_n2;
            locals.var_dnm_dn4 = assign43010_body0_e57772_d_n4;
            locals.var_dnm_dn5 = assign43010_body0_e57772_d_n5;
            locals.var_dnm_dn6 = assign43010_body0_e57772_d_n6;
            locals.var_dnm_dn7 = assign43010_body0_e57772_d_n7;
            locals.var_dnm_dn8 = assign43010_body0_e57772_d_n8;
            locals.var_dnm_dn9 = assign43010_body0_e57772_d_n9;
            locals.var_dnm_dn10 = assign43010_body0_e57772_d_n10;
            locals.var_dnm_dn11 = assign43010_body0_e57772_d_n11;
            locals.var_dnm_dn14 = assign43010_body0_e57772_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign43010_body1_e57789,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 != 0.0)) {
        let assign43010_body1_e57787: f64 = (locals.var_m0 + 1.0);
        (assign43010_body1_e57787,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign43010_body1_e57789;
            locals.var_m0_rv = 0.0;
        }

        let (assign43020_e57816, assign43020_e57816_d_n0, assign43020_e57816_d_n2, assign43020_e57816_d_n4, assign43020_e57816_d_n5, assign43020_e57816_d_n6, assign43020_e57816_d_n7, assign43020_e57816_d_n8, assign43020_e57816_d_n9, assign43020_e57816_d_n10, assign43020_e57816_d_n11, assign43020_e57816_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) && (locals.var_guard1063 == 0.0)) {
        let (assign43020_e57814, assign43020_e57814_d_n0, assign43020_e57814_d_n2, assign43020_e57814_d_n4, assign43020_e57814_d_n5, assign43020_e57814_d_n6, assign43020_e57814_d_n7, assign43020_e57814_d_n8, assign43020_e57814_d_n9, assign43020_e57814_d_n10, assign43020_e57814_d_n11, assign43020_e57814_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign43020_e57811: f64 = (2.0 * 2.0);
                let assign43020_e57812: f64 = (1.0 / assign43020_e57811);
                let assign43020_e57813: f64 = (locals.var_dnm).powf(assign43020_e57812);
                (assign43020_e57813, if 0.0 == 0.0 && ((assign43020_e57812) as f64).is_finite() && ((assign43020_e57812) as f64).fract() == 0.0 { if assign43020_e57812 == 0.0 { 0.0 } else { (assign43020_e57812 * ((locals.var_dnm).powf(assign43020_e57812 - 1.0) * locals.var_dnm_dn0)) } } else { (assign43020_e57813 * (assign43020_e57812 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43020_e57812) as f64).is_finite() && ((assign43020_e57812) as f64).fract() == 0.0 { if assign43020_e57812 == 0.0 { 0.0 } else { (assign43020_e57812 * ((locals.var_dnm).powf(assign43020_e57812 - 1.0) * locals.var_dnm_dn2)) } } else { (assign43020_e57813 * (assign43020_e57812 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43020_e57812) as f64).is_finite() && ((assign43020_e57812) as f64).fract() == 0.0 { if assign43020_e57812 == 0.0 { 0.0 } else { (assign43020_e57812 * ((locals.var_dnm).powf(assign43020_e57812 - 1.0) * locals.var_dnm_dn4)) } } else { (assign43020_e57813 * (assign43020_e57812 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43020_e57812) as f64).is_finite() && ((assign43020_e57812) as f64).fract() == 0.0 { if assign43020_e57812 == 0.0 { 0.0 } else { (assign43020_e57812 * ((locals.var_dnm).powf(assign43020_e57812 - 1.0) * locals.var_dnm_dn5)) } } else { (assign43020_e57813 * (assign43020_e57812 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43020_e57812) as f64).is_finite() && ((assign43020_e57812) as f64).fract() == 0.0 { if assign43020_e57812 == 0.0 { 0.0 } else { (assign43020_e57812 * ((locals.var_dnm).powf(assign43020_e57812 - 1.0) * locals.var_dnm_dn6)) } } else { (assign43020_e57813 * (assign43020_e57812 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43020_e57812) as f64).is_finite() && ((assign43020_e57812) as f64).fract() == 0.0 { if assign43020_e57812 == 0.0 { 0.0 } else { (assign43020_e57812 * ((locals.var_dnm).powf(assign43020_e57812 - 1.0) * locals.var_dnm_dn7)) } } else { (assign43020_e57813 * (assign43020_e57812 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43020_e57812) as f64).is_finite() && ((assign43020_e57812) as f64).fract() == 0.0 { if assign43020_e57812 == 0.0 { 0.0 } else { (assign43020_e57812 * ((locals.var_dnm).powf(assign43020_e57812 - 1.0) * locals.var_dnm_dn8)) } } else { (assign43020_e57813 * (assign43020_e57812 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43020_e57812) as f64).is_finite() && ((assign43020_e57812) as f64).fract() == 0.0 { if assign43020_e57812 == 0.0 { 0.0 } else { (assign43020_e57812 * ((locals.var_dnm).powf(assign43020_e57812 - 1.0) * locals.var_dnm_dn9)) } } else { (assign43020_e57813 * (assign43020_e57812 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43020_e57812) as f64).is_finite() && ((assign43020_e57812) as f64).fract() == 0.0 { if assign43020_e57812 == 0.0 { 0.0 } else { (assign43020_e57812 * ((locals.var_dnm).powf(assign43020_e57812 - 1.0) * locals.var_dnm_dn10)) } } else { (assign43020_e57813 * (assign43020_e57812 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43020_e57812) as f64).is_finite() && ((assign43020_e57812) as f64).fract() == 0.0 { if assign43020_e57812 == 0.0 { 0.0 } else { (assign43020_e57812 * ((locals.var_dnm).powf(assign43020_e57812 - 1.0) * locals.var_dnm_dn11)) } } else { (assign43020_e57813 * (assign43020_e57812 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43020_e57812) as f64).is_finite() && ((assign43020_e57812) as f64).fract() == 0.0 { if assign43020_e57812 == 0.0 { 0.0 } else { (assign43020_e57812 * ((locals.var_dnm).powf(assign43020_e57812 - 1.0) * locals.var_dnm_dn14)) } } else { (assign43020_e57813 * (assign43020_e57812 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign43020_e57814, assign43020_e57814_d_n0, assign43020_e57814_d_n2, assign43020_e57814_d_n4, assign43020_e57814_d_n5, assign43020_e57814_d_n6, assign43020_e57814_d_n7, assign43020_e57814_d_n8, assign43020_e57814_d_n9, assign43020_e57814_d_n10, assign43020_e57814_d_n11, assign43020_e57814_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign43020_e57816;
        locals.var_dnm_dn0 = assign43020_e57816_d_n0;
        locals.var_dnm_dn2 = assign43020_e57816_d_n2;
        locals.var_dnm_dn4 = assign43020_e57816_d_n4;
        locals.var_dnm_dn5 = assign43020_e57816_d_n5;
        locals.var_dnm_dn6 = assign43020_e57816_d_n6;
        locals.var_dnm_dn7 = assign43020_e57816_d_n7;
        locals.var_dnm_dn8 = assign43020_e57816_d_n8;
        locals.var_dnm_dn9 = assign43020_e57816_d_n9;
        locals.var_dnm_dn10 = assign43020_e57816_d_n10;
        locals.var_dnm_dn11 = assign43020_e57816_d_n11;
        locals.var_dnm_dn14 = assign43020_e57816_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign43030_e57831, assign43030_e57831_d_n0, assign43030_e57831_d_n2, assign43030_e57831_d_n4, assign43030_e57831_d_n5, assign43030_e57831_d_n6, assign43030_e57831_d_n7, assign43030_e57831_d_n8, assign43030_e57831_d_n9, assign43030_e57831_d_n10, assign43030_e57831_d_n11, assign43030_e57831_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) {
        let assign43030_e57829: f64 = (1.0 / locals.var_dnm);
        (assign43030_e57829, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign43030_e57831;
        locals.var_dnm_dn0 = assign43030_e57831_d_n0;
        locals.var_dnm_dn2 = assign43030_e57831_d_n2;
        locals.var_dnm_dn4 = assign43030_e57831_d_n4;
        locals.var_dnm_dn5 = assign43030_e57831_d_n5;
        locals.var_dnm_dn6 = assign43030_e57831_d_n6;
        locals.var_dnm_dn7 = assign43030_e57831_d_n7;
        locals.var_dnm_dn8 = assign43030_e57831_d_n8;
        locals.var_dnm_dn9 = assign43030_e57831_d_n9;
        locals.var_dnm_dn10 = assign43030_e57831_d_n10;
        locals.var_dnm_dn11 = assign43030_e57831_d_n11;
        locals.var_dnm_dn14 = assign43030_e57831_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign43040_e57848, assign43040_e57848_d_n0, assign43040_e57848_d_n2, assign43040_e57848_d_n4, assign43040_e57848_d_n5, assign43040_e57848_d_n6, assign43040_e57848_d_n7, assign43040_e57848_d_n8, assign43040_e57848_d_n9, assign43040_e57848_d_n10, assign43040_e57848_d_n11, assign43040_e57848_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) {
        let assign43040_e57844: f64 = (locals.var_tmf1 * locals.var_t5);
        let assign43040_e57846: f64 = (assign43040_e57844 * locals.var_dnm);
        (assign43040_e57846, ((((locals.var_tmf1_dn0 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn0)) * locals.var_dnm) + (assign43040_e57844 * locals.var_dnm_dn0)), ((((locals.var_tmf1_dn2 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn2)) * locals.var_dnm) + (assign43040_e57844 * locals.var_dnm_dn2)), ((((locals.var_tmf1_dn4 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn4)) * locals.var_dnm) + (assign43040_e57844 * locals.var_dnm_dn4)), ((((locals.var_tmf1_dn5 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn5)) * locals.var_dnm) + (assign43040_e57844 * locals.var_dnm_dn5)), ((((locals.var_tmf1_dn6 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn6)) * locals.var_dnm) + (assign43040_e57844 * locals.var_dnm_dn6)), ((((locals.var_tmf1_dn7 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn7)) * locals.var_dnm) + (assign43040_e57844 * locals.var_dnm_dn7)), ((((locals.var_tmf1_dn8 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn8)) * locals.var_dnm) + (assign43040_e57844 * locals.var_dnm_dn8)), ((((locals.var_tmf1_dn9 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn9)) * locals.var_dnm) + (assign43040_e57844 * locals.var_dnm_dn9)), ((((locals.var_tmf1_dn10 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn10)) * locals.var_dnm) + (assign43040_e57844 * locals.var_dnm_dn10)), ((((locals.var_tmf1_dn11 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn11)) * locals.var_dnm) + (assign43040_e57844 * locals.var_dnm_dn11)), ((((locals.var_tmf1_dn14 * locals.var_t5) + (locals.var_tmf1 * locals.var_t5_dn14)) * locals.var_dnm) + (assign43040_e57844 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign43040_e57848;
        locals.var_tmf0_dn0 = assign43040_e57848_d_n0;
        locals.var_tmf0_dn2 = assign43040_e57848_d_n2;
        locals.var_tmf0_dn4 = assign43040_e57848_d_n4;
        locals.var_tmf0_dn5 = assign43040_e57848_d_n5;
        locals.var_tmf0_dn6 = assign43040_e57848_d_n6;
        locals.var_tmf0_dn7 = assign43040_e57848_d_n7;
        locals.var_tmf0_dn8 = assign43040_e57848_d_n8;
        locals.var_tmf0_dn9 = assign43040_e57848_d_n9;
        locals.var_tmf0_dn10 = assign43040_e57848_d_n10;
        locals.var_tmf0_dn11 = assign43040_e57848_d_n11;
        locals.var_tmf0_dn14 = assign43040_e57848_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign43050_e57867, assign43050_e57867_d_n0, assign43050_e57867_d_n2, assign43050_e57867_d_n4, assign43050_e57867_d_n5, assign43050_e57867_d_n6, assign43050_e57867_d_n7, assign43050_e57867_d_n8, assign43050_e57867_d_n9, assign43050_e57867_d_n10, assign43050_e57867_d_n11, assign43050_e57867_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) {
        let assign43050_e57861: f64 = (locals.var_t5 * locals.var_xmp);
        let assign43050_e57863: f64 = (assign43050_e57861 * locals.var_dnm);
        let assign43050_e57865: f64 = (assign43050_e57863 / locals.var_arg);
        (assign43050_e57865, (((((((locals.var_t5_dn0 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign43050_e57861 * locals.var_dnm_dn0)) * locals.var_arg) - (assign43050_e57863 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn2 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign43050_e57861 * locals.var_dnm_dn2)) * locals.var_arg) - (assign43050_e57863 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn4 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign43050_e57861 * locals.var_dnm_dn4)) * locals.var_arg) - (assign43050_e57863 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn5 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign43050_e57861 * locals.var_dnm_dn5)) * locals.var_arg) - (assign43050_e57863 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn6 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign43050_e57861 * locals.var_dnm_dn6)) * locals.var_arg) - (assign43050_e57863 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn7 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn7)) * locals.var_dnm) + (assign43050_e57861 * locals.var_dnm_dn7)) * locals.var_arg) - (assign43050_e57863 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn8 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign43050_e57861 * locals.var_dnm_dn8)) * locals.var_arg) - (assign43050_e57863 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn9 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn9)) * locals.var_dnm) + (assign43050_e57861 * locals.var_dnm_dn9)) * locals.var_arg) - (assign43050_e57863 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn10 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign43050_e57861 * locals.var_dnm_dn10)) * locals.var_arg) - (assign43050_e57863 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn11 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign43050_e57861 * locals.var_dnm_dn11)) * locals.var_arg) - (assign43050_e57863 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), (((((((locals.var_t5_dn14 * locals.var_xmp) + (locals.var_t5 * locals.var_xmp_dn14)) * locals.var_dnm) + (assign43050_e57861 * locals.var_dnm_dn14)) * locals.var_arg) - (assign43050_e57863 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign43050_e57867;
        locals.var_t0_dn0 = assign43050_e57867_d_n0;
        locals.var_t0_dn2 = assign43050_e57867_d_n2;
        locals.var_t0_dn4 = assign43050_e57867_d_n4;
        locals.var_t0_dn5 = assign43050_e57867_d_n5;
        locals.var_t0_dn6 = assign43050_e57867_d_n6;
        locals.var_t0_dn7 = assign43050_e57867_d_n7;
        locals.var_t0_dn8 = assign43050_e57867_d_n8;
        locals.var_t0_dn9 = assign43050_e57867_d_n9;
        locals.var_t0_dn10 = assign43050_e57867_d_n10;
        locals.var_t0_dn11 = assign43050_e57867_d_n11;
        locals.var_t0_dn14 = assign43050_e57867_d_n14;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_149(
        locals: &mut StampLocals,
    ) {
        let (assign43060_e57884, assign43060_e57884_d_n0, assign43060_e57884_d_n2, assign43060_e57884_d_n4, assign43060_e57884_d_n5, assign43060_e57884_d_n6, assign43060_e57884_d_n7, assign43060_e57884_d_n8, assign43060_e57884_d_n9, assign43060_e57884_d_n10, assign43060_e57884_d_n11, assign43060_e57884_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) {
        let assign43060_e57880: f64 = locals.var_t5;
        let assign43060_e57882: f64 = (assign43060_e57880 - locals.var_tmf0);
        (assign43060_e57882, (locals.var_t5_dn0 - locals.var_tmf0_dn0), (locals.var_t5_dn2 - locals.var_tmf0_dn2), (locals.var_t5_dn4 - locals.var_tmf0_dn4), (locals.var_t5_dn5 - locals.var_tmf0_dn5), (locals.var_t5_dn6 - locals.var_tmf0_dn6), (locals.var_t5_dn7 - locals.var_tmf0_dn7), (locals.var_t5_dn8 - locals.var_tmf0_dn8), (locals.var_t5_dn9 - locals.var_tmf0_dn9), (locals.var_t5_dn10 - locals.var_tmf0_dn10), (locals.var_t5_dn11 - locals.var_tmf0_dn11), (locals.var_t5_dn14 - locals.var_tmf0_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign43060_e57884;
        locals.var_t4_dn0 = assign43060_e57884_d_n0;
        locals.var_t4_dn2 = assign43060_e57884_d_n2;
        locals.var_t4_dn4 = assign43060_e57884_d_n4;
        locals.var_t4_dn5 = assign43060_e57884_d_n5;
        locals.var_t4_dn6 = assign43060_e57884_d_n6;
        locals.var_t4_dn7 = assign43060_e57884_d_n7;
        locals.var_t4_dn8 = assign43060_e57884_d_n8;
        locals.var_t4_dn9 = assign43060_e57884_d_n9;
        locals.var_t4_dn10 = assign43060_e57884_d_n10;
        locals.var_t4_dn11 = assign43060_e57884_d_n11;
        locals.var_t4_dn14 = assign43060_e57884_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign43070_e57897, assign43070_e57897_d_n0, assign43070_e57897_d_n2, assign43070_e57897_d_n4, assign43070_e57897_d_n5, assign43070_e57897_d_n6, assign43070_e57897_d_n7, assign43070_e57897_d_n8, assign43070_e57897_d_n9, assign43070_e57897_d_n10, assign43070_e57897_d_n11, assign43070_e57897_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign43070_e57897;
        locals.var_t0_dn0 = assign43070_e57897_d_n0;
        locals.var_t0_dn2 = assign43070_e57897_d_n2;
        locals.var_t0_dn4 = assign43070_e57897_d_n4;
        locals.var_t0_dn5 = assign43070_e57897_d_n5;
        locals.var_t0_dn6 = assign43070_e57897_d_n6;
        locals.var_t0_dn7 = assign43070_e57897_d_n7;
        locals.var_t0_dn8 = assign43070_e57897_d_n8;
        locals.var_t0_dn9 = assign43070_e57897_d_n9;
        locals.var_t0_dn10 = assign43070_e57897_d_n10;
        locals.var_t0_dn11 = assign43070_e57897_d_n11;
        locals.var_t0_dn14 = assign43070_e57897_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign43080_e57911, assign43080_e57911_d_n0, assign43080_e57911_d_n2, assign43080_e57911_d_n4, assign43080_e57911_d_n5, assign43080_e57911_d_n6, assign43080_e57911_d_n7, assign43080_e57911_d_n8, assign43080_e57911_d_n9, assign43080_e57911_d_n10, assign43080_e57911_d_n11, assign43080_e57911_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 == 0.0)) {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign43080_e57911;
        locals.var_t4_dn0 = assign43080_e57911_d_n0;
        locals.var_t4_dn2 = assign43080_e57911_d_n2;
        locals.var_t4_dn4 = assign43080_e57911_d_n4;
        locals.var_t4_dn5 = assign43080_e57911_d_n5;
        locals.var_t4_dn6 = assign43080_e57911_d_n6;
        locals.var_t4_dn7 = assign43080_e57911_d_n7;
        locals.var_t4_dn8 = assign43080_e57911_d_n8;
        locals.var_t4_dn9 = assign43080_e57911_d_n9;
        locals.var_t4_dn10 = assign43080_e57911_d_n10;
        locals.var_t4_dn11 = assign43080_e57911_d_n11;
        locals.var_t4_dn14 = assign43080_e57911_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign43090_e57925, assign43090_e57925_d_n0, assign43090_e57925_d_n2, assign43090_e57925_d_n4, assign43090_e57925_d_n5, assign43090_e57925_d_n6, assign43090_e57925_d_n7, assign43090_e57925_d_n8, assign43090_e57925_d_n9, assign43090_e57925_d_n10, assign43090_e57925_d_n11, assign43090_e57925_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1062 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign43090_e57925;
        locals.var_t0_dn0 = assign43090_e57925_d_n0;
        locals.var_t0_dn2 = assign43090_e57925_d_n2;
        locals.var_t0_dn4 = assign43090_e57925_d_n4;
        locals.var_t0_dn5 = assign43090_e57925_d_n5;
        locals.var_t0_dn6 = assign43090_e57925_d_n6;
        locals.var_t0_dn7 = assign43090_e57925_d_n7;
        locals.var_t0_dn8 = assign43090_e57925_d_n8;
        locals.var_t0_dn9 = assign43090_e57925_d_n9;
        locals.var_t0_dn10 = assign43090_e57925_d_n10;
        locals.var_t0_dn11 = assign43090_e57925_d_n11;
        locals.var_t0_dn14 = assign43090_e57925_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign43100_e57937, assign43100_e57937_d_n0, assign43100_e57937_d_n2, assign43100_e57937_d_n4, assign43100_e57937_d_n5, assign43100_e57937_d_n6, assign43100_e57937_d_n7, assign43100_e57937_d_n8, assign43100_e57937_d_n9, assign43100_e57937_d_n10, assign43100_e57937_d_n11, assign43100_e57937_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) {
        let assign43100_e57935: f64 = (locals.var_t4).sqrt();
        (assign43100_e57935, (locals.var_t4_dn0 / (2.0 * assign43100_e57935)), (locals.var_t4_dn2 / (2.0 * assign43100_e57935)), (locals.var_t4_dn4 / (2.0 * assign43100_e57935)), (locals.var_t4_dn5 / (2.0 * assign43100_e57935)), (locals.var_t4_dn6 / (2.0 * assign43100_e57935)), (locals.var_t4_dn7 / (2.0 * assign43100_e57935)), (locals.var_t4_dn8 / (2.0 * assign43100_e57935)), (locals.var_t4_dn9 / (2.0 * assign43100_e57935)), (locals.var_t4_dn10 / (2.0 * assign43100_e57935)), (locals.var_t4_dn11 / (2.0 * assign43100_e57935)), (locals.var_t4_dn14 / (2.0 * assign43100_e57935)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign43100_e57937;
        locals.var_t3_dn0 = assign43100_e57937_d_n0;
        locals.var_t3_dn2 = assign43100_e57937_d_n2;
        locals.var_t3_dn4 = assign43100_e57937_d_n4;
        locals.var_t3_dn5 = assign43100_e57937_d_n5;
        locals.var_t3_dn6 = assign43100_e57937_d_n6;
        locals.var_t3_dn7 = assign43100_e57937_d_n7;
        locals.var_t3_dn8 = assign43100_e57937_d_n8;
        locals.var_t3_dn9 = assign43100_e57937_d_n9;
        locals.var_t3_dn10 = assign43100_e57937_d_n10;
        locals.var_t3_dn11 = assign43100_e57937_d_n11;
        locals.var_t3_dn14 = assign43100_e57937_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign43110_e57954, assign43110_e57954_d_n0, assign43110_e57954_d_n2, assign43110_e57954_d_n4, assign43110_e57954_d_n5, assign43110_e57954_d_n6, assign43110_e57954_d_n7, assign43110_e57954_d_n8, assign43110_e57954_d_n9, assign43110_e57954_d_n10, assign43110_e57954_d_n11, assign43110_e57954_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) {
        let assign43110_e57950: f64 = (1.0 - locals.var_t3);
        let assign43110_e57951: f64 = (locals.var_q_ndepm_esi_cox_inv2 * assign43110_e57950);
        let assign43110_e57952: f64 = (locals.var_vgp + assign43110_e57951);
        (assign43110_e57952, (locals.var_vgp_dn0 + ((locals.var_q_ndepm_esi_cox_inv2_dn0 * assign43110_e57950) + (locals.var_q_ndepm_esi_cox_inv2 * (-locals.var_t3_dn0)))), (locals.var_vgp_dn2 + ((locals.var_q_ndepm_esi_cox_inv2_dn2 * assign43110_e57950) + (locals.var_q_ndepm_esi_cox_inv2 * (-locals.var_t3_dn2)))), (locals.var_vgp_dn4 + ((locals.var_q_ndepm_esi_cox_inv2_dn4 * assign43110_e57950) + (locals.var_q_ndepm_esi_cox_inv2 * (-locals.var_t3_dn4)))), (locals.var_vgp_dn5 + ((locals.var_q_ndepm_esi_cox_inv2_dn5 * assign43110_e57950) + (locals.var_q_ndepm_esi_cox_inv2 * (-locals.var_t3_dn5)))), (locals.var_vgp_dn6 + ((locals.var_q_ndepm_esi_cox_inv2_dn6 * assign43110_e57950) + (locals.var_q_ndepm_esi_cox_inv2 * (-locals.var_t3_dn6)))), (locals.var_vgp_dn7 + ((locals.var_q_ndepm_esi_cox_inv2_dn7 * assign43110_e57950) + (locals.var_q_ndepm_esi_cox_inv2 * (-locals.var_t3_dn7)))), (locals.var_vgp_dn8 + ((locals.var_q_ndepm_esi_cox_inv2_dn8 * assign43110_e57950) + (locals.var_q_ndepm_esi_cox_inv2 * (-locals.var_t3_dn8)))), (locals.var_vgp_dn9 + ((locals.var_q_ndepm_esi_cox_inv2_dn9 * assign43110_e57950) + (locals.var_q_ndepm_esi_cox_inv2 * (-locals.var_t3_dn9)))), (locals.var_vgp_dn10 + ((locals.var_q_ndepm_esi_cox_inv2_dn10 * assign43110_e57950) + (locals.var_q_ndepm_esi_cox_inv2 * (-locals.var_t3_dn10)))), (locals.var_vgp_dn11 + ((locals.var_q_ndepm_esi_cox_inv2_dn11 * assign43110_e57950) + (locals.var_q_ndepm_esi_cox_inv2 * (-locals.var_t3_dn11)))), (locals.var_vgp_dn14 + ((locals.var_q_ndepm_esi_cox_inv2_dn14 * assign43110_e57950) + (locals.var_q_ndepm_esi_cox_inv2 * (-locals.var_t3_dn14)))),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign43110_e57954;
        locals.var_t10_dn0 = assign43110_e57954_d_n0;
        locals.var_t10_dn2 = assign43110_e57954_d_n2;
        locals.var_t10_dn4 = assign43110_e57954_d_n4;
        locals.var_t10_dn5 = assign43110_e57954_d_n5;
        locals.var_t10_dn6 = assign43110_e57954_d_n6;
        locals.var_t10_dn7 = assign43110_e57954_d_n7;
        locals.var_t10_dn8 = assign43110_e57954_d_n8;
        locals.var_t10_dn9 = assign43110_e57954_d_n9;
        locals.var_t10_dn10 = assign43110_e57954_d_n10;
        locals.var_t10_dn11 = assign43110_e57954_d_n11;
        locals.var_t10_dn14 = assign43110_e57954_d_n14;
        locals.var_t10_rv = 0.0;

        let assign43120_e57958: f64 = (locals.var_uc_depleak + locals.var_depqfn_dlt);
        let assign43120_e57963: f64 = if ((locals.var_t10 < assign43120_e57958) && (locals.var_depqfn_dlt >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1068 = assign43120_e57963;
        locals.var_guard1068_rv = 0.0;

        let (assign43130_e57980, assign43130_e57980_d_n0, assign43130_e57980_d_n2, assign43130_e57980_d_n4, assign43130_e57980_d_n5, assign43130_e57980_d_n6, assign43130_e57980_d_n7, assign43130_e57980_d_n8, assign43130_e57980_d_n9, assign43130_e57980_d_n10, assign43130_e57980_d_n11, assign43130_e57980_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1068 != 0.0)) {
        let assign43130_e57976: f64 = (locals.var_uc_depleak + locals.var_depqfn_dlt);
        let assign43130_e57978: f64 = (assign43130_e57976 - locals.var_t10);
        (assign43130_e57978, (locals.var_uc_depleak_dn0 - locals.var_t10_dn0), (locals.var_uc_depleak_dn2 - locals.var_t10_dn2), (locals.var_uc_depleak_dn4 - locals.var_t10_dn4), (locals.var_uc_depleak_dn5 - locals.var_t10_dn5), (locals.var_uc_depleak_dn6 - locals.var_t10_dn6), (locals.var_uc_depleak_dn7 - locals.var_t10_dn7), (locals.var_uc_depleak_dn8 - locals.var_t10_dn8), (locals.var_uc_depleak_dn9 - locals.var_t10_dn9), (locals.var_uc_depleak_dn10 - locals.var_t10_dn10), (locals.var_uc_depleak_dn11 - locals.var_t10_dn11), (locals.var_uc_depleak_dn14 - locals.var_t10_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign43130_e57980;
        locals.var_tmf1_dn0 = assign43130_e57980_d_n0;
        locals.var_tmf1_dn2 = assign43130_e57980_d_n2;
        locals.var_tmf1_dn4 = assign43130_e57980_d_n4;
        locals.var_tmf1_dn5 = assign43130_e57980_d_n5;
        locals.var_tmf1_dn6 = assign43130_e57980_d_n6;
        locals.var_tmf1_dn7 = assign43130_e57980_d_n7;
        locals.var_tmf1_dn8 = assign43130_e57980_d_n8;
        locals.var_tmf1_dn9 = assign43130_e57980_d_n9;
        locals.var_tmf1_dn10 = assign43130_e57980_d_n10;
        locals.var_tmf1_dn11 = assign43130_e57980_d_n11;
        locals.var_tmf1_dn14 = assign43130_e57980_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign43140_e57995, assign43140_e57995_d_n0, assign43140_e57995_d_n2, assign43140_e57995_d_n4, assign43140_e57995_d_n5, assign43140_e57995_d_n6, assign43140_e57995_d_n7, assign43140_e57995_d_n8, assign43140_e57995_d_n9, assign43140_e57995_d_n10, assign43140_e57995_d_n11, assign43140_e57995_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1068 != 0.0)) {
        let assign43140_e57993: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign43140_e57993, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign43140_e57995;
        locals.var_x2_dn0 = assign43140_e57995_d_n0;
        locals.var_x2_dn2 = assign43140_e57995_d_n2;
        locals.var_x2_dn4 = assign43140_e57995_d_n4;
        locals.var_x2_dn5 = assign43140_e57995_d_n5;
        locals.var_x2_dn6 = assign43140_e57995_d_n6;
        locals.var_x2_dn7 = assign43140_e57995_d_n7;
        locals.var_x2_dn8 = assign43140_e57995_d_n8;
        locals.var_x2_dn9 = assign43140_e57995_d_n9;
        locals.var_x2_dn10 = assign43140_e57995_d_n10;
        locals.var_x2_dn11 = assign43140_e57995_d_n11;
        locals.var_x2_dn14 = assign43140_e57995_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign43150_e58010, assign43150_e58010_d_n0, assign43150_e58010_d_n2, assign43150_e58010_d_n4, assign43150_e58010_d_n5, assign43150_e58010_d_n6, assign43150_e58010_d_n7, assign43150_e58010_d_n8, assign43150_e58010_d_n9, assign43150_e58010_d_n10, assign43150_e58010_d_n11, assign43150_e58010_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1068 != 0.0)) {
        let assign43150_e58008: f64 = (locals.var_depqfn_dlt * locals.var_depqfn_dlt);
        (assign43150_e58008, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign43150_e58010;
        locals.var_xmax2_dn0 = assign43150_e58010_d_n0;
        locals.var_xmax2_dn2 = assign43150_e58010_d_n2;
        locals.var_xmax2_dn4 = assign43150_e58010_d_n4;
        locals.var_xmax2_dn5 = assign43150_e58010_d_n5;
        locals.var_xmax2_dn6 = assign43150_e58010_d_n6;
        locals.var_xmax2_dn7 = assign43150_e58010_d_n7;
        locals.var_xmax2_dn8 = assign43150_e58010_d_n8;
        locals.var_xmax2_dn9 = assign43150_e58010_d_n9;
        locals.var_xmax2_dn10 = assign43150_e58010_d_n10;
        locals.var_xmax2_dn11 = assign43150_e58010_d_n11;
        locals.var_xmax2_dn14 = assign43150_e58010_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign43160_e58023, assign43160_e58023_d_n0, assign43160_e58023_d_n2, assign43160_e58023_d_n4, assign43160_e58023_d_n5, assign43160_e58023_d_n6, assign43160_e58023_d_n7, assign43160_e58023_d_n8, assign43160_e58023_d_n9, assign43160_e58023_d_n10, assign43160_e58023_d_n11, assign43160_e58023_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1068 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign43160_e58023;
        locals.var_xp_dn0 = assign43160_e58023_d_n0;
        locals.var_xp_dn2 = assign43160_e58023_d_n2;
        locals.var_xp_dn4 = assign43160_e58023_d_n4;
        locals.var_xp_dn5 = assign43160_e58023_d_n5;
        locals.var_xp_dn6 = assign43160_e58023_d_n6;
        locals.var_xp_dn7 = assign43160_e58023_d_n7;
        locals.var_xp_dn8 = assign43160_e58023_d_n8;
        locals.var_xp_dn9 = assign43160_e58023_d_n9;
        locals.var_xp_dn10 = assign43160_e58023_d_n10;
        locals.var_xp_dn11 = assign43160_e58023_d_n11;
        locals.var_xp_dn14 = assign43160_e58023_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign43170_e58036, assign43170_e58036_d_n0, assign43170_e58036_d_n2, assign43170_e58036_d_n4, assign43170_e58036_d_n5, assign43170_e58036_d_n6, assign43170_e58036_d_n7, assign43170_e58036_d_n8, assign43170_e58036_d_n9, assign43170_e58036_d_n10, assign43170_e58036_d_n11, assign43170_e58036_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1068 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign43170_e58036;
        locals.var_xmp_dn0 = assign43170_e58036_d_n0;
        locals.var_xmp_dn2 = assign43170_e58036_d_n2;
        locals.var_xmp_dn4 = assign43170_e58036_d_n4;
        locals.var_xmp_dn5 = assign43170_e58036_d_n5;
        locals.var_xmp_dn6 = assign43170_e58036_d_n6;
        locals.var_xmp_dn7 = assign43170_e58036_d_n7;
        locals.var_xmp_dn8 = assign43170_e58036_d_n8;
        locals.var_xmp_dn9 = assign43170_e58036_d_n9;
        locals.var_xmp_dn10 = assign43170_e58036_d_n10;
        locals.var_xmp_dn11 = assign43170_e58036_d_n11;
        locals.var_xmp_dn14 = assign43170_e58036_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign43180_e58049,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1068 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign43180_e58049;
        locals.var_m0_rv = 0.0;

        let (assign43190_e58062,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1068 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign43190_e58062;
        locals.var_mm_rv = 0.0;

        let (assign43200_e58075, assign43200_e58075_d_n0, assign43200_e58075_d_n2, assign43200_e58075_d_n4, assign43200_e58075_d_n5, assign43200_e58075_d_n6, assign43200_e58075_d_n7, assign43200_e58075_d_n8, assign43200_e58075_d_n9, assign43200_e58075_d_n10, assign43200_e58075_d_n11, assign43200_e58075_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1068 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign43200_e58075;
        locals.var_arg_dn0 = assign43200_e58075_d_n0;
        locals.var_arg_dn2 = assign43200_e58075_d_n2;
        locals.var_arg_dn4 = assign43200_e58075_d_n4;
        locals.var_arg_dn5 = assign43200_e58075_d_n5;
        locals.var_arg_dn6 = assign43200_e58075_d_n6;
        locals.var_arg_dn7 = assign43200_e58075_d_n7;
        locals.var_arg_dn8 = assign43200_e58075_d_n8;
        locals.var_arg_dn9 = assign43200_e58075_d_n9;
        locals.var_arg_dn10 = assign43200_e58075_d_n10;
        locals.var_arg_dn11 = assign43200_e58075_d_n11;
        locals.var_arg_dn14 = assign43200_e58075_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign43210_e58088, assign43210_e58088_d_n0, assign43210_e58088_d_n2, assign43210_e58088_d_n4, assign43210_e58088_d_n5, assign43210_e58088_d_n6, assign43210_e58088_d_n7, assign43210_e58088_d_n8, assign43210_e58088_d_n9, assign43210_e58088_d_n10, assign43210_e58088_d_n11, assign43210_e58088_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1068 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign43210_e58088;
        locals.var_dnm_dn0 = assign43210_e58088_d_n0;
        locals.var_dnm_dn2 = assign43210_e58088_d_n2;
        locals.var_dnm_dn4 = assign43210_e58088_d_n4;
        locals.var_dnm_dn5 = assign43210_e58088_d_n5;
        locals.var_dnm_dn6 = assign43210_e58088_d_n6;
        locals.var_dnm_dn7 = assign43210_e58088_d_n7;
        locals.var_dnm_dn8 = assign43210_e58088_d_n8;
        locals.var_dnm_dn9 = assign43210_e58088_d_n9;
        locals.var_dnm_dn10 = assign43210_e58088_d_n10;
        locals.var_dnm_dn11 = assign43210_e58088_d_n11;
        locals.var_dnm_dn14 = assign43210_e58088_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign43220_e58103, assign43220_e58103_d_n0, assign43220_e58103_d_n2, assign43220_e58103_d_n4, assign43220_e58103_d_n5, assign43220_e58103_d_n6, assign43220_e58103_d_n7, assign43220_e58103_d_n8, assign43220_e58103_d_n9, assign43220_e58103_d_n10, assign43220_e58103_d_n11, assign43220_e58103_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1068 != 0.0)) {
        let assign43220_e58101: f64 = (locals.var_xp * locals.var_x2);
        (assign43220_e58101, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign43220_e58103;
        locals.var_xp_dn0 = assign43220_e58103_d_n0;
        locals.var_xp_dn2 = assign43220_e58103_d_n2;
        locals.var_xp_dn4 = assign43220_e58103_d_n4;
        locals.var_xp_dn5 = assign43220_e58103_d_n5;
        locals.var_xp_dn6 = assign43220_e58103_d_n6;
        locals.var_xp_dn7 = assign43220_e58103_d_n7;
        locals.var_xp_dn8 = assign43220_e58103_d_n8;
        locals.var_xp_dn9 = assign43220_e58103_d_n9;
        locals.var_xp_dn10 = assign43220_e58103_d_n10;
        locals.var_xp_dn11 = assign43220_e58103_d_n11;
        locals.var_xp_dn14 = assign43220_e58103_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign43230_e58118, assign43230_e58118_d_n0, assign43230_e58118_d_n2, assign43230_e58118_d_n4, assign43230_e58118_d_n5, assign43230_e58118_d_n6, assign43230_e58118_d_n7, assign43230_e58118_d_n8, assign43230_e58118_d_n9, assign43230_e58118_d_n10, assign43230_e58118_d_n11, assign43230_e58118_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1068 != 0.0)) {
        let assign43230_e58116: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign43230_e58116, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign43230_e58118;
        locals.var_xmp_dn0 = assign43230_e58118_d_n0;
        locals.var_xmp_dn2 = assign43230_e58118_d_n2;
        locals.var_xmp_dn4 = assign43230_e58118_d_n4;
        locals.var_xmp_dn5 = assign43230_e58118_d_n5;
        locals.var_xmp_dn6 = assign43230_e58118_d_n6;
        locals.var_xmp_dn7 = assign43230_e58118_d_n7;
        locals.var_xmp_dn8 = assign43230_e58118_d_n8;
        locals.var_xmp_dn9 = assign43230_e58118_d_n9;
        locals.var_xmp_dn10 = assign43230_e58118_d_n10;
        locals.var_xmp_dn11 = assign43230_e58118_d_n11;
        locals.var_xmp_dn14 = assign43230_e58118_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign43240_e58133, assign43240_e58133_d_n0, assign43240_e58133_d_n2, assign43240_e58133_d_n4, assign43240_e58133_d_n5, assign43240_e58133_d_n6, assign43240_e58133_d_n7, assign43240_e58133_d_n8, assign43240_e58133_d_n9, assign43240_e58133_d_n10, assign43240_e58133_d_n11, assign43240_e58133_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1068 != 0.0)) {
        let assign43240_e58131: f64 = (locals.var_xp * locals.var_x2);
        (assign43240_e58131, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign43240_e58133;
        locals.var_xp_dn0 = assign43240_e58133_d_n0;
        locals.var_xp_dn2 = assign43240_e58133_d_n2;
        locals.var_xp_dn4 = assign43240_e58133_d_n4;
        locals.var_xp_dn5 = assign43240_e58133_d_n5;
        locals.var_xp_dn6 = assign43240_e58133_d_n6;
        locals.var_xp_dn7 = assign43240_e58133_d_n7;
        locals.var_xp_dn8 = assign43240_e58133_d_n8;
        locals.var_xp_dn9 = assign43240_e58133_d_n9;
        locals.var_xp_dn10 = assign43240_e58133_d_n10;
        locals.var_xp_dn11 = assign43240_e58133_d_n11;
        locals.var_xp_dn14 = assign43240_e58133_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign43250_e58148, assign43250_e58148_d_n0, assign43250_e58148_d_n2, assign43250_e58148_d_n4, assign43250_e58148_d_n5, assign43250_e58148_d_n6, assign43250_e58148_d_n7, assign43250_e58148_d_n8, assign43250_e58148_d_n9, assign43250_e58148_d_n10, assign43250_e58148_d_n11, assign43250_e58148_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1068 != 0.0)) {
        let assign43250_e58146: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign43250_e58146, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign43250_e58148;
        locals.var_xmp_dn0 = assign43250_e58148_d_n0;
        locals.var_xmp_dn2 = assign43250_e58148_d_n2;
        locals.var_xmp_dn4 = assign43250_e58148_d_n4;
        locals.var_xmp_dn5 = assign43250_e58148_d_n5;
        locals.var_xmp_dn6 = assign43250_e58148_d_n6;
        locals.var_xmp_dn7 = assign43250_e58148_d_n7;
        locals.var_xmp_dn8 = assign43250_e58148_d_n8;
        locals.var_xmp_dn9 = assign43250_e58148_d_n9;
        locals.var_xmp_dn10 = assign43250_e58148_d_n10;
        locals.var_xmp_dn11 = assign43250_e58148_d_n11;
        locals.var_xmp_dn14 = assign43250_e58148_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign43260_e58163, assign43260_e58163_d_n0, assign43260_e58163_d_n2, assign43260_e58163_d_n4, assign43260_e58163_d_n5, assign43260_e58163_d_n6, assign43260_e58163_d_n7, assign43260_e58163_d_n8, assign43260_e58163_d_n9, assign43260_e58163_d_n10, assign43260_e58163_d_n11, assign43260_e58163_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1068 != 0.0)) {
        let assign43260_e58161: f64 = (locals.var_xp + locals.var_xmp);
        (assign43260_e58161, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign43260_e58163;
        locals.var_arg_dn0 = assign43260_e58163_d_n0;
        locals.var_arg_dn2 = assign43260_e58163_d_n2;
        locals.var_arg_dn4 = assign43260_e58163_d_n4;
        locals.var_arg_dn5 = assign43260_e58163_d_n5;
        locals.var_arg_dn6 = assign43260_e58163_d_n6;
        locals.var_arg_dn7 = assign43260_e58163_d_n7;
        locals.var_arg_dn8 = assign43260_e58163_d_n8;
        locals.var_arg_dn9 = assign43260_e58163_d_n9;
        locals.var_arg_dn10 = assign43260_e58163_d_n10;
        locals.var_arg_dn11 = assign43260_e58163_d_n11;
        locals.var_arg_dn14 = assign43260_e58163_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign43270_e58176, assign43270_e58176_d_n0, assign43270_e58176_d_n2, assign43270_e58176_d_n4, assign43270_e58176_d_n5, assign43270_e58176_d_n6, assign43270_e58176_d_n7, assign43270_e58176_d_n8, assign43270_e58176_d_n9, assign43270_e58176_d_n10, assign43270_e58176_d_n11, assign43270_e58176_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1068 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign43270_e58176;
        locals.var_dnm_dn0 = assign43270_e58176_d_n0;
        locals.var_dnm_dn2 = assign43270_e58176_d_n2;
        locals.var_dnm_dn4 = assign43270_e58176_d_n4;
        locals.var_dnm_dn5 = assign43270_e58176_d_n5;
        locals.var_dnm_dn6 = assign43270_e58176_d_n6;
        locals.var_dnm_dn7 = assign43270_e58176_d_n7;
        locals.var_dnm_dn8 = assign43270_e58176_d_n8;
        locals.var_dnm_dn9 = assign43270_e58176_d_n9;
        locals.var_dnm_dn10 = assign43270_e58176_d_n10;
        locals.var_dnm_dn11 = assign43270_e58176_d_n11;
        locals.var_dnm_dn14 = assign43270_e58176_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign43280_e58191: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1069 = assign43280_e58191;
        locals.var_guard1069_rv = 0.0;

        let assign43290_e58194: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1070 = assign43290_e58194;
        locals.var_guard1070_rv = 0.0;

        let (assign43300_e58211,) = {
    if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1069 != 0.0)) && (locals.var_guard1070 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign43300_e58211;
        locals.var_mm_rv = 0.0;

        let assign43310_e58214: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1071 = assign43310_e58214;
        locals.var_guard1071_rv = 0.0;

        let (assign43320_e58234,) = {
    if (((((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1069 != 0.0)) && (locals.var_guard1070 == 0.0)) && (locals.var_guard1071 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign43320_e58234;
        locals.var_mm_rv = 0.0;

        let assign43330_e58237: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1072 = assign43330_e58237;
        locals.var_guard1072_rv = 0.0;

        let (assign43340_e58260,) = {
    if ((((((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1069 != 0.0)) && (locals.var_guard1070 == 0.0)) && (locals.var_guard1071 == 0.0)) && (locals.var_guard1072 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign43340_e58260;
        locals.var_mm_rv = 0.0;

        let assign43350_e58263: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1073 = assign43350_e58263;
        locals.var_guard1073_rv = 0.0;

        let (assign43360_e58289,) = {
    if (((((((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1069 != 0.0)) && (locals.var_guard1070 == 0.0)) && (locals.var_guard1071 == 0.0)) && (locals.var_guard1072 == 0.0)) && (locals.var_guard1073 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign43360_e58289;
        locals.var_mm_rv = 0.0;

        let (assign43370_e58304,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1069 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign43370_e58304;
        locals.var_m0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_150(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign43380_loop_guard: usize = 0;
        while {
            let assign43380_cond_e58320: f64 = if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1069 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign43380_cond_e58320 != 0.0
        } {
            assign43380_loop_guard += 1;
            assert!(assign43380_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign43380_body0_e58336, assign43380_body0_e58336_d_n0, assign43380_body0_e58336_d_n2, assign43380_body0_e58336_d_n4, assign43380_body0_e58336_d_n5, assign43380_body0_e58336_d_n6, assign43380_body0_e58336_d_n7, assign43380_body0_e58336_d_n8, assign43380_body0_e58336_d_n9, assign43380_body0_e58336_d_n10, assign43380_body0_e58336_d_n11, assign43380_body0_e58336_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1069 != 0.0)) {
        let assign43380_body0_e58334: f64 = (locals.var_dnm).sqrt();
        (assign43380_body0_e58334, (locals.var_dnm_dn0 / (2.0 * assign43380_body0_e58334)), (locals.var_dnm_dn2 / (2.0 * assign43380_body0_e58334)), (locals.var_dnm_dn4 / (2.0 * assign43380_body0_e58334)), (locals.var_dnm_dn5 / (2.0 * assign43380_body0_e58334)), (locals.var_dnm_dn6 / (2.0 * assign43380_body0_e58334)), (locals.var_dnm_dn7 / (2.0 * assign43380_body0_e58334)), (locals.var_dnm_dn8 / (2.0 * assign43380_body0_e58334)), (locals.var_dnm_dn9 / (2.0 * assign43380_body0_e58334)), (locals.var_dnm_dn10 / (2.0 * assign43380_body0_e58334)), (locals.var_dnm_dn11 / (2.0 * assign43380_body0_e58334)), (locals.var_dnm_dn14 / (2.0 * assign43380_body0_e58334)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign43380_body0_e58336;
            locals.var_dnm_dn0 = assign43380_body0_e58336_d_n0;
            locals.var_dnm_dn2 = assign43380_body0_e58336_d_n2;
            locals.var_dnm_dn4 = assign43380_body0_e58336_d_n4;
            locals.var_dnm_dn5 = assign43380_body0_e58336_d_n5;
            locals.var_dnm_dn6 = assign43380_body0_e58336_d_n6;
            locals.var_dnm_dn7 = assign43380_body0_e58336_d_n7;
            locals.var_dnm_dn8 = assign43380_body0_e58336_d_n8;
            locals.var_dnm_dn9 = assign43380_body0_e58336_d_n9;
            locals.var_dnm_dn10 = assign43380_body0_e58336_d_n10;
            locals.var_dnm_dn11 = assign43380_body0_e58336_d_n11;
            locals.var_dnm_dn14 = assign43380_body0_e58336_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign43380_body1_e58353,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1069 != 0.0)) {
        let assign43380_body1_e58351: f64 = (locals.var_m0 + 1.0);
        (assign43380_body1_e58351,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign43380_body1_e58353;
            locals.var_m0_rv = 0.0;
        }

        let (assign43390_e58380, assign43390_e58380_d_n0, assign43390_e58380_d_n2, assign43390_e58380_d_n4, assign43390_e58380_d_n5, assign43390_e58380_d_n6, assign43390_e58380_d_n7, assign43390_e58380_d_n8, assign43390_e58380_d_n9, assign43390_e58380_d_n10, assign43390_e58380_d_n11, assign43390_e58380_d_n14,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1068 != 0.0)) && (locals.var_guard1069 == 0.0)) {
        let (assign43390_e58378, assign43390_e58378_d_n0, assign43390_e58378_d_n2, assign43390_e58378_d_n4, assign43390_e58378_d_n5, assign43390_e58378_d_n6, assign43390_e58378_d_n7, assign43390_e58378_d_n8, assign43390_e58378_d_n9, assign43390_e58378_d_n10, assign43390_e58378_d_n11, assign43390_e58378_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign43390_e58375: f64 = (2.0 * 2.0);
                let assign43390_e58376: f64 = (1.0 / assign43390_e58375);
                let assign43390_e58377: f64 = (locals.var_dnm).powf(assign43390_e58376);
                (assign43390_e58377, if 0.0 == 0.0 && ((assign43390_e58376) as f64).is_finite() && ((assign43390_e58376) as f64).fract() == 0.0 { if assign43390_e58376 == 0.0 { 0.0 } else { (assign43390_e58376 * ((locals.var_dnm).powf(assign43390_e58376 - 1.0) * locals.var_dnm_dn0)) } } else { (assign43390_e58377 * (assign43390_e58376 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43390_e58376) as f64).is_finite() && ((assign43390_e58376) as f64).fract() == 0.0 { if assign43390_e58376 == 0.0 { 0.0 } else { (assign43390_e58376 * ((locals.var_dnm).powf(assign43390_e58376 - 1.0) * locals.var_dnm_dn2)) } } else { (assign43390_e58377 * (assign43390_e58376 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43390_e58376) as f64).is_finite() && ((assign43390_e58376) as f64).fract() == 0.0 { if assign43390_e58376 == 0.0 { 0.0 } else { (assign43390_e58376 * ((locals.var_dnm).powf(assign43390_e58376 - 1.0) * locals.var_dnm_dn4)) } } else { (assign43390_e58377 * (assign43390_e58376 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43390_e58376) as f64).is_finite() && ((assign43390_e58376) as f64).fract() == 0.0 { if assign43390_e58376 == 0.0 { 0.0 } else { (assign43390_e58376 * ((locals.var_dnm).powf(assign43390_e58376 - 1.0) * locals.var_dnm_dn5)) } } else { (assign43390_e58377 * (assign43390_e58376 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43390_e58376) as f64).is_finite() && ((assign43390_e58376) as f64).fract() == 0.0 { if assign43390_e58376 == 0.0 { 0.0 } else { (assign43390_e58376 * ((locals.var_dnm).powf(assign43390_e58376 - 1.0) * locals.var_dnm_dn6)) } } else { (assign43390_e58377 * (assign43390_e58376 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43390_e58376) as f64).is_finite() && ((assign43390_e58376) as f64).fract() == 0.0 { if assign43390_e58376 == 0.0 { 0.0 } else { (assign43390_e58376 * ((locals.var_dnm).powf(assign43390_e58376 - 1.0) * locals.var_dnm_dn7)) } } else { (assign43390_e58377 * (assign43390_e58376 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43390_e58376) as f64).is_finite() && ((assign43390_e58376) as f64).fract() == 0.0 { if assign43390_e58376 == 0.0 { 0.0 } else { (assign43390_e58376 * ((locals.var_dnm).powf(assign43390_e58376 - 1.0) * locals.var_dnm_dn8)) } } else { (assign43390_e58377 * (assign43390_e58376 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43390_e58376) as f64).is_finite() && ((assign43390_e58376) as f64).fract() == 0.0 { if assign43390_e58376 == 0.0 { 0.0 } else { (assign43390_e58376 * ((locals.var_dnm).powf(assign43390_e58376 - 1.0) * locals.var_dnm_dn9)) } } else { (assign43390_e58377 * (assign43390_e58376 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43390_e58376) as f64).is_finite() && ((assign43390_e58376) as f64).fract() == 0.0 { if assign43390_e58376 == 0.0 { 0.0 } else { (assign43390_e58376 * ((locals.var_dnm).powf(assign43390_e58376 - 1.0) * locals.var_dnm_dn10)) } } else { (assign43390_e58377 * (assign43390_e58376 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43390_e58376) as f64).is_finite() && ((assign43390_e58376) as f64).fract() == 0.0 { if assign43390_e58376 == 0.0 { 0.0 } else { (assign43390_e58376 * ((locals.var_dnm).powf(assign43390_e58376 - 1.0) * locals.var_dnm_dn11)) } } else { (assign43390_e58377 * (assign43390_e58376 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43390_e58376) as f64).is_finite() && ((assign43390_e58376) as f64).fract() == 0.0 { if assign43390_e58376 == 0.0 { 0.0 } else { (assign43390_e58376 * ((locals.var_dnm).powf(assign43390_e58376 - 1.0) * locals.var_dnm_dn14)) } } else { (assign43390_e58377 * (assign43390_e58376 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign43390_e58378, assign43390_e58378_d_n0, assign43390_e58378_d_n2, assign43390_e58378_d_n4, assign43390_e58378_d_n5, assign43390_e58378_d_n6, assign43390_e58378_d_n7, assign43390_e58378_d_n8, assign43390_e58378_d_n9, assign43390_e58378_d_n10, assign43390_e58378_d_n11, assign43390_e58378_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign43390_e58380;
        locals.var_dnm_dn0 = assign43390_e58380_d_n0;
        locals.var_dnm_dn2 = assign43390_e58380_d_n2;
        locals.var_dnm_dn4 = assign43390_e58380_d_n4;
        locals.var_dnm_dn5 = assign43390_e58380_d_n5;
        locals.var_dnm_dn6 = assign43390_e58380_d_n6;
        locals.var_dnm_dn7 = assign43390_e58380_d_n7;
        locals.var_dnm_dn8 = assign43390_e58380_d_n8;
        locals.var_dnm_dn9 = assign43390_e58380_d_n9;
        locals.var_dnm_dn10 = assign43390_e58380_d_n10;
        locals.var_dnm_dn11 = assign43390_e58380_d_n11;
        locals.var_dnm_dn14 = assign43390_e58380_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign43400_e58395, assign43400_e58395_d_n0, assign43400_e58395_d_n2, assign43400_e58395_d_n4, assign43400_e58395_d_n5, assign43400_e58395_d_n6, assign43400_e58395_d_n7, assign43400_e58395_d_n8, assign43400_e58395_d_n9, assign43400_e58395_d_n10, assign43400_e58395_d_n11, assign43400_e58395_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1068 != 0.0)) {
        let assign43400_e58393: f64 = (1.0 / locals.var_dnm);
        (assign43400_e58393, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign43400_e58395;
        locals.var_dnm_dn0 = assign43400_e58395_d_n0;
        locals.var_dnm_dn2 = assign43400_e58395_d_n2;
        locals.var_dnm_dn4 = assign43400_e58395_d_n4;
        locals.var_dnm_dn5 = assign43400_e58395_d_n5;
        locals.var_dnm_dn6 = assign43400_e58395_d_n6;
        locals.var_dnm_dn7 = assign43400_e58395_d_n7;
        locals.var_dnm_dn8 = assign43400_e58395_d_n8;
        locals.var_dnm_dn9 = assign43400_e58395_d_n9;
        locals.var_dnm_dn10 = assign43400_e58395_d_n10;
        locals.var_dnm_dn11 = assign43400_e58395_d_n11;
        locals.var_dnm_dn14 = assign43400_e58395_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign43410_e58412, assign43410_e58412_d_n0, assign43410_e58412_d_n2, assign43410_e58412_d_n4, assign43410_e58412_d_n5, assign43410_e58412_d_n6, assign43410_e58412_d_n7, assign43410_e58412_d_n8, assign43410_e58412_d_n9, assign43410_e58412_d_n10, assign43410_e58412_d_n11, assign43410_e58412_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1068 != 0.0)) {
        let assign43410_e58408: f64 = (locals.var_tmf1 * locals.var_depqfn_dlt);
        let assign43410_e58410: f64 = (assign43410_e58408 * locals.var_dnm);
        (assign43410_e58410, (((locals.var_tmf1_dn0 * locals.var_depqfn_dlt) * locals.var_dnm) + (assign43410_e58408 * locals.var_dnm_dn0)), (((locals.var_tmf1_dn2 * locals.var_depqfn_dlt) * locals.var_dnm) + (assign43410_e58408 * locals.var_dnm_dn2)), (((locals.var_tmf1_dn4 * locals.var_depqfn_dlt) * locals.var_dnm) + (assign43410_e58408 * locals.var_dnm_dn4)), (((locals.var_tmf1_dn5 * locals.var_depqfn_dlt) * locals.var_dnm) + (assign43410_e58408 * locals.var_dnm_dn5)), (((locals.var_tmf1_dn6 * locals.var_depqfn_dlt) * locals.var_dnm) + (assign43410_e58408 * locals.var_dnm_dn6)), (((locals.var_tmf1_dn7 * locals.var_depqfn_dlt) * locals.var_dnm) + (assign43410_e58408 * locals.var_dnm_dn7)), (((locals.var_tmf1_dn8 * locals.var_depqfn_dlt) * locals.var_dnm) + (assign43410_e58408 * locals.var_dnm_dn8)), (((locals.var_tmf1_dn9 * locals.var_depqfn_dlt) * locals.var_dnm) + (assign43410_e58408 * locals.var_dnm_dn9)), (((locals.var_tmf1_dn10 * locals.var_depqfn_dlt) * locals.var_dnm) + (assign43410_e58408 * locals.var_dnm_dn10)), (((locals.var_tmf1_dn11 * locals.var_depqfn_dlt) * locals.var_dnm) + (assign43410_e58408 * locals.var_dnm_dn11)), (((locals.var_tmf1_dn14 * locals.var_depqfn_dlt) * locals.var_dnm) + (assign43410_e58408 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign43410_e58412;
        locals.var_tmf0_dn0 = assign43410_e58412_d_n0;
        locals.var_tmf0_dn2 = assign43410_e58412_d_n2;
        locals.var_tmf0_dn4 = assign43410_e58412_d_n4;
        locals.var_tmf0_dn5 = assign43410_e58412_d_n5;
        locals.var_tmf0_dn6 = assign43410_e58412_d_n6;
        locals.var_tmf0_dn7 = assign43410_e58412_d_n7;
        locals.var_tmf0_dn8 = assign43410_e58412_d_n8;
        locals.var_tmf0_dn9 = assign43410_e58412_d_n9;
        locals.var_tmf0_dn10 = assign43410_e58412_d_n10;
        locals.var_tmf0_dn11 = assign43410_e58412_d_n11;
        locals.var_tmf0_dn14 = assign43410_e58412_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign43420_e58431, assign43420_e58431_d_n0, assign43420_e58431_d_n2, assign43420_e58431_d_n4, assign43420_e58431_d_n5, assign43420_e58431_d_n6, assign43420_e58431_d_n7, assign43420_e58431_d_n8, assign43420_e58431_d_n9, assign43420_e58431_d_n10, assign43420_e58431_d_n11, assign43420_e58431_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1068 != 0.0)) {
        let assign43420_e58425: f64 = (locals.var_depqfn_dlt * locals.var_xmp);
        let assign43420_e58427: f64 = (assign43420_e58425 * locals.var_dnm);
        let assign43420_e58429: f64 = (assign43420_e58427 / locals.var_arg);
        (assign43420_e58429, ((((((locals.var_depqfn_dlt * locals.var_xmp_dn0) * locals.var_dnm) + (assign43420_e58425 * locals.var_dnm_dn0)) * locals.var_arg) - (assign43420_e58427 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_depqfn_dlt * locals.var_xmp_dn2) * locals.var_dnm) + (assign43420_e58425 * locals.var_dnm_dn2)) * locals.var_arg) - (assign43420_e58427 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_depqfn_dlt * locals.var_xmp_dn4) * locals.var_dnm) + (assign43420_e58425 * locals.var_dnm_dn4)) * locals.var_arg) - (assign43420_e58427 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_depqfn_dlt * locals.var_xmp_dn5) * locals.var_dnm) + (assign43420_e58425 * locals.var_dnm_dn5)) * locals.var_arg) - (assign43420_e58427 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_depqfn_dlt * locals.var_xmp_dn6) * locals.var_dnm) + (assign43420_e58425 * locals.var_dnm_dn6)) * locals.var_arg) - (assign43420_e58427 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_depqfn_dlt * locals.var_xmp_dn7) * locals.var_dnm) + (assign43420_e58425 * locals.var_dnm_dn7)) * locals.var_arg) - (assign43420_e58427 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_depqfn_dlt * locals.var_xmp_dn8) * locals.var_dnm) + (assign43420_e58425 * locals.var_dnm_dn8)) * locals.var_arg) - (assign43420_e58427 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_depqfn_dlt * locals.var_xmp_dn9) * locals.var_dnm) + (assign43420_e58425 * locals.var_dnm_dn9)) * locals.var_arg) - (assign43420_e58427 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_depqfn_dlt * locals.var_xmp_dn10) * locals.var_dnm) + (assign43420_e58425 * locals.var_dnm_dn10)) * locals.var_arg) - (assign43420_e58427 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_depqfn_dlt * locals.var_xmp_dn11) * locals.var_dnm) + (assign43420_e58425 * locals.var_dnm_dn11)) * locals.var_arg) - (assign43420_e58427 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), ((((((locals.var_depqfn_dlt * locals.var_xmp_dn14) * locals.var_dnm) + (assign43420_e58425 * locals.var_dnm_dn14)) * locals.var_arg) - (assign43420_e58427 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign43420_e58431;
        locals.var_t0_dn0 = assign43420_e58431_d_n0;
        locals.var_t0_dn2 = assign43420_e58431_d_n2;
        locals.var_t0_dn4 = assign43420_e58431_d_n4;
        locals.var_t0_dn5 = assign43420_e58431_d_n5;
        locals.var_t0_dn6 = assign43420_e58431_d_n6;
        locals.var_t0_dn7 = assign43420_e58431_d_n7;
        locals.var_t0_dn8 = assign43420_e58431_d_n8;
        locals.var_t0_dn9 = assign43420_e58431_d_n9;
        locals.var_t0_dn10 = assign43420_e58431_d_n10;
        locals.var_t0_dn11 = assign43420_e58431_d_n11;
        locals.var_t0_dn14 = assign43420_e58431_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign43430_e58448, assign43430_e58448_d_n0, assign43430_e58448_d_n2, assign43430_e58448_d_n4, assign43430_e58448_d_n5, assign43430_e58448_d_n6, assign43430_e58448_d_n7, assign43430_e58448_d_n8, assign43430_e58448_d_n9, assign43430_e58448_d_n10, assign43430_e58448_d_n11, assign43430_e58448_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1068 != 0.0)) {
        let assign43430_e58444: f64 = (locals.var_uc_depleak + locals.var_depqfn_dlt);
        let assign43430_e58446: f64 = (assign43430_e58444 - locals.var_tmf0);
        (assign43430_e58446, (locals.var_uc_depleak_dn0 - locals.var_tmf0_dn0), (locals.var_uc_depleak_dn2 - locals.var_tmf0_dn2), (locals.var_uc_depleak_dn4 - locals.var_tmf0_dn4), (locals.var_uc_depleak_dn5 - locals.var_tmf0_dn5), (locals.var_uc_depleak_dn6 - locals.var_tmf0_dn6), (locals.var_uc_depleak_dn7 - locals.var_tmf0_dn7), (locals.var_uc_depleak_dn8 - locals.var_tmf0_dn8), (locals.var_uc_depleak_dn9 - locals.var_tmf0_dn9), (locals.var_uc_depleak_dn10 - locals.var_tmf0_dn10), (locals.var_uc_depleak_dn11 - locals.var_tmf0_dn11), (locals.var_uc_depleak_dn14 - locals.var_tmf0_dn14),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign43430_e58448;
        locals.var_t10_dn0 = assign43430_e58448_d_n0;
        locals.var_t10_dn2 = assign43430_e58448_d_n2;
        locals.var_t10_dn4 = assign43430_e58448_d_n4;
        locals.var_t10_dn5 = assign43430_e58448_d_n5;
        locals.var_t10_dn6 = assign43430_e58448_d_n6;
        locals.var_t10_dn7 = assign43430_e58448_d_n7;
        locals.var_t10_dn8 = assign43430_e58448_d_n8;
        locals.var_t10_dn9 = assign43430_e58448_d_n9;
        locals.var_t10_dn10 = assign43430_e58448_d_n10;
        locals.var_t10_dn11 = assign43430_e58448_d_n11;
        locals.var_t10_dn14 = assign43430_e58448_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign43440_e58461, assign43440_e58461_d_n0, assign43440_e58461_d_n2, assign43440_e58461_d_n4, assign43440_e58461_d_n5, assign43440_e58461_d_n6, assign43440_e58461_d_n7, assign43440_e58461_d_n8, assign43440_e58461_d_n9, assign43440_e58461_d_n10, assign43440_e58461_d_n11, assign43440_e58461_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1068 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign43440_e58461;
        locals.var_t0_dn0 = assign43440_e58461_d_n0;
        locals.var_t0_dn2 = assign43440_e58461_d_n2;
        locals.var_t0_dn4 = assign43440_e58461_d_n4;
        locals.var_t0_dn5 = assign43440_e58461_d_n5;
        locals.var_t0_dn6 = assign43440_e58461_d_n6;
        locals.var_t0_dn7 = assign43440_e58461_d_n7;
        locals.var_t0_dn8 = assign43440_e58461_d_n8;
        locals.var_t0_dn9 = assign43440_e58461_d_n9;
        locals.var_t0_dn10 = assign43440_e58461_d_n10;
        locals.var_t0_dn11 = assign43440_e58461_d_n11;
        locals.var_t0_dn14 = assign43440_e58461_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign43450_e58475, assign43450_e58475_d_n0, assign43450_e58475_d_n2, assign43450_e58475_d_n4, assign43450_e58475_d_n5, assign43450_e58475_d_n6, assign43450_e58475_d_n7, assign43450_e58475_d_n8, assign43450_e58475_d_n9, assign43450_e58475_d_n10, assign43450_e58475_d_n11, assign43450_e58475_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1068 == 0.0)) {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign43450_e58475;
        locals.var_t10_dn0 = assign43450_e58475_d_n0;
        locals.var_t10_dn2 = assign43450_e58475_d_n2;
        locals.var_t10_dn4 = assign43450_e58475_d_n4;
        locals.var_t10_dn5 = assign43450_e58475_d_n5;
        locals.var_t10_dn6 = assign43450_e58475_d_n6;
        locals.var_t10_dn7 = assign43450_e58475_d_n7;
        locals.var_t10_dn8 = assign43450_e58475_d_n8;
        locals.var_t10_dn9 = assign43450_e58475_d_n9;
        locals.var_t10_dn10 = assign43450_e58475_d_n10;
        locals.var_t10_dn11 = assign43450_e58475_d_n11;
        locals.var_t10_dn14 = assign43450_e58475_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign43460_e58489, assign43460_e58489_d_n0, assign43460_e58489_d_n2, assign43460_e58489_d_n4, assign43460_e58489_d_n5, assign43460_e58489_d_n6, assign43460_e58489_d_n7, assign43460_e58489_d_n8, assign43460_e58489_d_n9, assign43460_e58489_d_n10, assign43460_e58489_d_n11, assign43460_e58489_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) && (locals.var_guard1068 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign43460_e58489;
        locals.var_t0_dn0 = assign43460_e58489_d_n0;
        locals.var_t0_dn2 = assign43460_e58489_d_n2;
        locals.var_t0_dn4 = assign43460_e58489_d_n4;
        locals.var_t0_dn5 = assign43460_e58489_d_n5;
        locals.var_t0_dn6 = assign43460_e58489_d_n6;
        locals.var_t0_dn7 = assign43460_e58489_d_n7;
        locals.var_t0_dn8 = assign43460_e58489_d_n8;
        locals.var_t0_dn9 = assign43460_e58489_d_n9;
        locals.var_t0_dn10 = assign43460_e58489_d_n10;
        locals.var_t0_dn11 = assign43460_e58489_d_n11;
        locals.var_t0_dn14 = assign43460_e58489_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign43470_e58502, assign43470_e58502_d_n0, assign43470_e58502_d_n2, assign43470_e58502_d_n4, assign43470_e58502_d_n5, assign43470_e58502_d_n6, assign43470_e58502_d_n7, assign43470_e58502_d_n8, assign43470_e58502_d_n9, assign43470_e58502_d_n10, assign43470_e58502_d_n11, assign43470_e58502_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) {
        let assign43470_e58500: f64 = (locals.var_vds_res / locals.var_t10);
        (assign43470_e58500, (((locals.var_vds_res_dn0 * locals.var_t10) - (locals.var_vds_res * locals.var_t10_dn0)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_res_dn2 * locals.var_t10) - (locals.var_vds_res * locals.var_t10_dn2)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_res_dn4 * locals.var_t10) - (locals.var_vds_res * locals.var_t10_dn4)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_res_dn5 * locals.var_t10) - (locals.var_vds_res * locals.var_t10_dn5)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_res_dn6 * locals.var_t10) - (locals.var_vds_res * locals.var_t10_dn6)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_res_dn7 * locals.var_t10) - (locals.var_vds_res * locals.var_t10_dn7)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_res_dn8 * locals.var_t10) - (locals.var_vds_res * locals.var_t10_dn8)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_res_dn9 * locals.var_t10) - (locals.var_vds_res * locals.var_t10_dn9)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_res_dn10 * locals.var_t10) - (locals.var_vds_res * locals.var_t10_dn10)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_res_dn11 * locals.var_t10) - (locals.var_vds_res * locals.var_t10_dn11)) / (locals.var_t10 * locals.var_t10)), (((locals.var_vds_res_dn14 * locals.var_t10) - (locals.var_vds_res * locals.var_t10_dn14)) / (locals.var_t10 * locals.var_t10)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign43470_e58502;
        locals.var_t1_dn0 = assign43470_e58502_d_n0;
        locals.var_t1_dn2 = assign43470_e58502_d_n2;
        locals.var_t1_dn4 = assign43470_e58502_d_n4;
        locals.var_t1_dn5 = assign43470_e58502_d_n5;
        locals.var_t1_dn6 = assign43470_e58502_d_n6;
        locals.var_t1_dn7 = assign43470_e58502_d_n7;
        locals.var_t1_dn8 = assign43470_e58502_d_n8;
        locals.var_t1_dn9 = assign43470_e58502_d_n9;
        locals.var_t1_dn10 = assign43470_e58502_d_n10;
        locals.var_t1_dn11 = assign43470_e58502_d_n11;
        locals.var_t1_dn14 = assign43470_e58502_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign43480_e58522, assign43480_e58522_d_n0, assign43480_e58522_d_n2, assign43480_e58522_d_n4, assign43480_e58522_d_n5, assign43480_e58522_d_n6, assign43480_e58522_d_n7, assign43480_e58522_d_n8, assign43480_e58522_d_n9, assign43480_e58522_d_n10, assign43480_e58522_d_n11, assign43480_e58522_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) {
        let (assign43480_e58520, assign43480_e58520_d_n0, assign43480_e58520_d_n2, assign43480_e58520_d_n4, assign43480_e58520_d_n5, assign43480_e58520_d_n6, assign43480_e58520_d_n7, assign43480_e58520_d_n8, assign43480_e58520_d_n9, assign43480_e58520_d_n10, assign43480_e58520_d_n11, assign43480_e58520_d_n14,) = {
            if (locals.var_t1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign43480_e58518: f64 = (p.p383 - 1.0);
                let assign43480_e58519: f64 = (locals.var_t1).powf(assign43480_e58518);
                (assign43480_e58519, if 0.0 == 0.0 && ((assign43480_e58518) as f64).is_finite() && ((assign43480_e58518) as f64).fract() == 0.0 { if assign43480_e58518 == 0.0 { 0.0 } else { (assign43480_e58518 * ((locals.var_t1).powf(assign43480_e58518 - 1.0) * locals.var_t1_dn0)) } } else { (assign43480_e58519 * (assign43480_e58518 * (locals.var_t1_dn0 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign43480_e58518) as f64).is_finite() && ((assign43480_e58518) as f64).fract() == 0.0 { if assign43480_e58518 == 0.0 { 0.0 } else { (assign43480_e58518 * ((locals.var_t1).powf(assign43480_e58518 - 1.0) * locals.var_t1_dn2)) } } else { (assign43480_e58519 * (assign43480_e58518 * (locals.var_t1_dn2 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign43480_e58518) as f64).is_finite() && ((assign43480_e58518) as f64).fract() == 0.0 { if assign43480_e58518 == 0.0 { 0.0 } else { (assign43480_e58518 * ((locals.var_t1).powf(assign43480_e58518 - 1.0) * locals.var_t1_dn4)) } } else { (assign43480_e58519 * (assign43480_e58518 * (locals.var_t1_dn4 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign43480_e58518) as f64).is_finite() && ((assign43480_e58518) as f64).fract() == 0.0 { if assign43480_e58518 == 0.0 { 0.0 } else { (assign43480_e58518 * ((locals.var_t1).powf(assign43480_e58518 - 1.0) * locals.var_t1_dn5)) } } else { (assign43480_e58519 * (assign43480_e58518 * (locals.var_t1_dn5 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign43480_e58518) as f64).is_finite() && ((assign43480_e58518) as f64).fract() == 0.0 { if assign43480_e58518 == 0.0 { 0.0 } else { (assign43480_e58518 * ((locals.var_t1).powf(assign43480_e58518 - 1.0) * locals.var_t1_dn6)) } } else { (assign43480_e58519 * (assign43480_e58518 * (locals.var_t1_dn6 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign43480_e58518) as f64).is_finite() && ((assign43480_e58518) as f64).fract() == 0.0 { if assign43480_e58518 == 0.0 { 0.0 } else { (assign43480_e58518 * ((locals.var_t1).powf(assign43480_e58518 - 1.0) * locals.var_t1_dn7)) } } else { (assign43480_e58519 * (assign43480_e58518 * (locals.var_t1_dn7 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign43480_e58518) as f64).is_finite() && ((assign43480_e58518) as f64).fract() == 0.0 { if assign43480_e58518 == 0.0 { 0.0 } else { (assign43480_e58518 * ((locals.var_t1).powf(assign43480_e58518 - 1.0) * locals.var_t1_dn8)) } } else { (assign43480_e58519 * (assign43480_e58518 * (locals.var_t1_dn8 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign43480_e58518) as f64).is_finite() && ((assign43480_e58518) as f64).fract() == 0.0 { if assign43480_e58518 == 0.0 { 0.0 } else { (assign43480_e58518 * ((locals.var_t1).powf(assign43480_e58518 - 1.0) * locals.var_t1_dn9)) } } else { (assign43480_e58519 * (assign43480_e58518 * (locals.var_t1_dn9 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign43480_e58518) as f64).is_finite() && ((assign43480_e58518) as f64).fract() == 0.0 { if assign43480_e58518 == 0.0 { 0.0 } else { (assign43480_e58518 * ((locals.var_t1).powf(assign43480_e58518 - 1.0) * locals.var_t1_dn10)) } } else { (assign43480_e58519 * (assign43480_e58518 * (locals.var_t1_dn10 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign43480_e58518) as f64).is_finite() && ((assign43480_e58518) as f64).fract() == 0.0 { if assign43480_e58518 == 0.0 { 0.0 } else { (assign43480_e58518 * ((locals.var_t1).powf(assign43480_e58518 - 1.0) * locals.var_t1_dn11)) } } else { (assign43480_e58519 * (assign43480_e58518 * (locals.var_t1_dn11 / locals.var_t1))) }, if 0.0 == 0.0 && ((assign43480_e58518) as f64).is_finite() && ((assign43480_e58518) as f64).fract() == 0.0 { if assign43480_e58518 == 0.0 { 0.0 } else { (assign43480_e58518 * ((locals.var_t1).powf(assign43480_e58518 - 1.0) * locals.var_t1_dn14)) } } else { (assign43480_e58519 * (assign43480_e58518 * (locals.var_t1_dn14 / locals.var_t1))) },)
            }
        };
        (assign43480_e58520, assign43480_e58520_d_n0, assign43480_e58520_d_n2, assign43480_e58520_d_n4, assign43480_e58520_d_n5, assign43480_e58520_d_n6, assign43480_e58520_d_n7, assign43480_e58520_d_n8, assign43480_e58520_d_n9, assign43480_e58520_d_n10, assign43480_e58520_d_n11, assign43480_e58520_d_n14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign43480_e58522;
        locals.var_t2_dn0 = assign43480_e58522_d_n0;
        locals.var_t2_dn2 = assign43480_e58522_d_n2;
        locals.var_t2_dn4 = assign43480_e58522_d_n4;
        locals.var_t2_dn5 = assign43480_e58522_d_n5;
        locals.var_t2_dn6 = assign43480_e58522_d_n6;
        locals.var_t2_dn7 = assign43480_e58522_d_n7;
        locals.var_t2_dn8 = assign43480_e58522_d_n8;
        locals.var_t2_dn9 = assign43480_e58522_d_n9;
        locals.var_t2_dn10 = assign43480_e58522_d_n10;
        locals.var_t2_dn11 = assign43480_e58522_d_n11;
        locals.var_t2_dn14 = assign43480_e58522_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign43490_e58537, assign43490_e58537_d_n0, assign43490_e58537_d_n2, assign43490_e58537_d_n4, assign43490_e58537_d_n5, assign43490_e58537_d_n6, assign43490_e58537_d_n7, assign43490_e58537_d_n8, assign43490_e58537_d_n9, assign43490_e58537_d_n10, assign43490_e58537_d_n11, assign43490_e58537_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) {
        let assign43490_e58534: f64 = (locals.var_t2 * locals.var_t1);
        let assign43490_e58535: f64 = (1.0 + assign43490_e58534);
        (assign43490_e58535, ((locals.var_t2_dn0 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn0)), ((locals.var_t2_dn2 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn2)), ((locals.var_t2_dn4 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn4)), ((locals.var_t2_dn5 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn5)), ((locals.var_t2_dn6 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn6)), ((locals.var_t2_dn7 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn7)), ((locals.var_t2_dn8 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn8)), ((locals.var_t2_dn9 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn9)), ((locals.var_t2_dn10 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn10)), ((locals.var_t2_dn11 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn11)), ((locals.var_t2_dn14 * locals.var_t1) + (locals.var_t2 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign43490_e58537;
        locals.var_t3_dn0 = assign43490_e58537_d_n0;
        locals.var_t3_dn2 = assign43490_e58537_d_n2;
        locals.var_t3_dn4 = assign43490_e58537_d_n4;
        locals.var_t3_dn5 = assign43490_e58537_d_n5;
        locals.var_t3_dn6 = assign43490_e58537_d_n6;
        locals.var_t3_dn7 = assign43490_e58537_d_n7;
        locals.var_t3_dn8 = assign43490_e58537_d_n8;
        locals.var_t3_dn9 = assign43490_e58537_d_n9;
        locals.var_t3_dn10 = assign43490_e58537_d_n10;
        locals.var_t3_dn11 = assign43490_e58537_d_n11;
        locals.var_t3_dn14 = assign43490_e58537_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign43500_e58559, assign43500_e58559_d_n0, assign43500_e58559_d_n2, assign43500_e58559_d_n4, assign43500_e58559_d_n5, assign43500_e58559_d_n6, assign43500_e58559_d_n7, assign43500_e58559_d_n8, assign43500_e58559_d_n9, assign43500_e58559_d_n10, assign43500_e58559_d_n11, assign43500_e58559_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) {
        let (assign43500_e58557, assign43500_e58557_d_n0, assign43500_e58557_d_n2, assign43500_e58557_d_n4, assign43500_e58557_d_n5, assign43500_e58557_d_n6, assign43500_e58557_d_n7, assign43500_e58557_d_n8, assign43500_e58557_d_n9, assign43500_e58557_d_n10, assign43500_e58557_d_n11, assign43500_e58557_d_n14,) = {
            if (locals.var_t3 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign43500_e58553: f64 = (1.0 / p.p383);
                let assign43500_e58555: f64 = (assign43500_e58553 - 1.0);
                let assign43500_e58556: f64 = (locals.var_t3).powf(assign43500_e58555);
                (assign43500_e58556, if 0.0 == 0.0 && ((assign43500_e58555) as f64).is_finite() && ((assign43500_e58555) as f64).fract() == 0.0 { if assign43500_e58555 == 0.0 { 0.0 } else { (assign43500_e58555 * ((locals.var_t3).powf(assign43500_e58555 - 1.0) * locals.var_t3_dn0)) } } else { (assign43500_e58556 * (assign43500_e58555 * (locals.var_t3_dn0 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign43500_e58555) as f64).is_finite() && ((assign43500_e58555) as f64).fract() == 0.0 { if assign43500_e58555 == 0.0 { 0.0 } else { (assign43500_e58555 * ((locals.var_t3).powf(assign43500_e58555 - 1.0) * locals.var_t3_dn2)) } } else { (assign43500_e58556 * (assign43500_e58555 * (locals.var_t3_dn2 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign43500_e58555) as f64).is_finite() && ((assign43500_e58555) as f64).fract() == 0.0 { if assign43500_e58555 == 0.0 { 0.0 } else { (assign43500_e58555 * ((locals.var_t3).powf(assign43500_e58555 - 1.0) * locals.var_t3_dn4)) } } else { (assign43500_e58556 * (assign43500_e58555 * (locals.var_t3_dn4 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign43500_e58555) as f64).is_finite() && ((assign43500_e58555) as f64).fract() == 0.0 { if assign43500_e58555 == 0.0 { 0.0 } else { (assign43500_e58555 * ((locals.var_t3).powf(assign43500_e58555 - 1.0) * locals.var_t3_dn5)) } } else { (assign43500_e58556 * (assign43500_e58555 * (locals.var_t3_dn5 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign43500_e58555) as f64).is_finite() && ((assign43500_e58555) as f64).fract() == 0.0 { if assign43500_e58555 == 0.0 { 0.0 } else { (assign43500_e58555 * ((locals.var_t3).powf(assign43500_e58555 - 1.0) * locals.var_t3_dn6)) } } else { (assign43500_e58556 * (assign43500_e58555 * (locals.var_t3_dn6 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign43500_e58555) as f64).is_finite() && ((assign43500_e58555) as f64).fract() == 0.0 { if assign43500_e58555 == 0.0 { 0.0 } else { (assign43500_e58555 * ((locals.var_t3).powf(assign43500_e58555 - 1.0) * locals.var_t3_dn7)) } } else { (assign43500_e58556 * (assign43500_e58555 * (locals.var_t3_dn7 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign43500_e58555) as f64).is_finite() && ((assign43500_e58555) as f64).fract() == 0.0 { if assign43500_e58555 == 0.0 { 0.0 } else { (assign43500_e58555 * ((locals.var_t3).powf(assign43500_e58555 - 1.0) * locals.var_t3_dn8)) } } else { (assign43500_e58556 * (assign43500_e58555 * (locals.var_t3_dn8 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign43500_e58555) as f64).is_finite() && ((assign43500_e58555) as f64).fract() == 0.0 { if assign43500_e58555 == 0.0 { 0.0 } else { (assign43500_e58555 * ((locals.var_t3).powf(assign43500_e58555 - 1.0) * locals.var_t3_dn9)) } } else { (assign43500_e58556 * (assign43500_e58555 * (locals.var_t3_dn9 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign43500_e58555) as f64).is_finite() && ((assign43500_e58555) as f64).fract() == 0.0 { if assign43500_e58555 == 0.0 { 0.0 } else { (assign43500_e58555 * ((locals.var_t3).powf(assign43500_e58555 - 1.0) * locals.var_t3_dn10)) } } else { (assign43500_e58556 * (assign43500_e58555 * (locals.var_t3_dn10 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign43500_e58555) as f64).is_finite() && ((assign43500_e58555) as f64).fract() == 0.0 { if assign43500_e58555 == 0.0 { 0.0 } else { (assign43500_e58555 * ((locals.var_t3).powf(assign43500_e58555 - 1.0) * locals.var_t3_dn11)) } } else { (assign43500_e58556 * (assign43500_e58555 * (locals.var_t3_dn11 / locals.var_t3))) }, if 0.0 == 0.0 && ((assign43500_e58555) as f64).is_finite() && ((assign43500_e58555) as f64).fract() == 0.0 { if assign43500_e58555 == 0.0 { 0.0 } else { (assign43500_e58555 * ((locals.var_t3).powf(assign43500_e58555 - 1.0) * locals.var_t3_dn14)) } } else { (assign43500_e58556 * (assign43500_e58555 * (locals.var_t3_dn14 / locals.var_t3))) },)
            }
        };
        (assign43500_e58557, assign43500_e58557_d_n0, assign43500_e58557_d_n2, assign43500_e58557_d_n4, assign43500_e58557_d_n5, assign43500_e58557_d_n6, assign43500_e58557_d_n7, assign43500_e58557_d_n8, assign43500_e58557_d_n9, assign43500_e58557_d_n10, assign43500_e58557_d_n11, assign43500_e58557_d_n14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign43500_e58559;
        locals.var_t4_dn0 = assign43500_e58559_d_n0;
        locals.var_t4_dn2 = assign43500_e58559_d_n2;
        locals.var_t4_dn4 = assign43500_e58559_d_n4;
        locals.var_t4_dn5 = assign43500_e58559_d_n5;
        locals.var_t4_dn6 = assign43500_e58559_d_n6;
        locals.var_t4_dn7 = assign43500_e58559_d_n7;
        locals.var_t4_dn8 = assign43500_e58559_d_n8;
        locals.var_t4_dn9 = assign43500_e58559_d_n9;
        locals.var_t4_dn10 = assign43500_e58559_d_n10;
        locals.var_t4_dn11 = assign43500_e58559_d_n11;
        locals.var_t4_dn14 = assign43500_e58559_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign43510_e58572, assign43510_e58572_d_n0, assign43510_e58572_d_n2, assign43510_e58572_d_n4, assign43510_e58572_d_n5, assign43510_e58572_d_n6, assign43510_e58572_d_n7, assign43510_e58572_d_n8, assign43510_e58572_d_n9, assign43510_e58572_d_n10, assign43510_e58572_d_n11, assign43510_e58572_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) {
        let assign43510_e58570: f64 = (locals.var_t4 * locals.var_t3);
        (assign43510_e58570, ((locals.var_t4_dn0 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn0)), ((locals.var_t4_dn2 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn2)), ((locals.var_t4_dn4 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn4)), ((locals.var_t4_dn5 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn5)), ((locals.var_t4_dn6 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn6)), ((locals.var_t4_dn7 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn7)), ((locals.var_t4_dn8 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn8)), ((locals.var_t4_dn9 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn9)), ((locals.var_t4_dn10 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn10)), ((locals.var_t4_dn11 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn11)), ((locals.var_t4_dn14 * locals.var_t3) + (locals.var_t4 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign43510_e58572;
        locals.var_t6_dn0 = assign43510_e58572_d_n0;
        locals.var_t6_dn2 = assign43510_e58572_d_n2;
        locals.var_t6_dn4 = assign43510_e58572_d_n4;
        locals.var_t6_dn5 = assign43510_e58572_d_n5;
        locals.var_t6_dn6 = assign43510_e58572_d_n6;
        locals.var_t6_dn7 = assign43510_e58572_d_n7;
        locals.var_t6_dn8 = assign43510_e58572_d_n8;
        locals.var_t6_dn9 = assign43510_e58572_d_n9;
        locals.var_t6_dn10 = assign43510_e58572_d_n10;
        locals.var_t6_dn11 = assign43510_e58572_d_n11;
        locals.var_t6_dn14 = assign43510_e58572_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign43520_e58585, assign43520_e58585_d_n0, assign43520_e58585_d_n2, assign43520_e58585_d_n4, assign43520_e58585_d_n5, assign43520_e58585_d_n6, assign43520_e58585_d_n7, assign43520_e58585_d_n8, assign43520_e58585_d_n9, assign43520_e58585_d_n10, assign43520_e58585_d_n11, assign43520_e58585_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1061 != 0.0)) {
        let assign43520_e58583: f64 = (locals.var_vds_res / locals.var_t6);
        (assign43520_e58583, (((locals.var_vds_res_dn0 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn0)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_res_dn2 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn2)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_res_dn4 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn4)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_res_dn5 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn5)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_res_dn6 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn6)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_res_dn7 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn7)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_res_dn8 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn8)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_res_dn9 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn9)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_res_dn10 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn10)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_res_dn11 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn11)) / (locals.var_t6 * locals.var_t6)), (((locals.var_vds_res_dn14 * locals.var_t6) - (locals.var_vds_res * locals.var_t6_dn14)) / (locals.var_t6 * locals.var_t6)),)
    } else {
        (locals.var_vds_res, locals.var_vds_res_dn0, locals.var_vds_res_dn2, locals.var_vds_res_dn4, locals.var_vds_res_dn5, locals.var_vds_res_dn6, locals.var_vds_res_dn7, locals.var_vds_res_dn8, locals.var_vds_res_dn9, locals.var_vds_res_dn10, locals.var_vds_res_dn11, locals.var_vds_res_dn14,)
    }
};
        locals.var_vds_res = assign43520_e58585;
        locals.var_vds_res_dn0 = assign43520_e58585_d_n0;
        locals.var_vds_res_dn2 = assign43520_e58585_d_n2;
        locals.var_vds_res_dn4 = assign43520_e58585_d_n4;
        locals.var_vds_res_dn5 = assign43520_e58585_d_n5;
        locals.var_vds_res_dn6 = assign43520_e58585_d_n6;
        locals.var_vds_res_dn7 = assign43520_e58585_d_n7;
        locals.var_vds_res_dn8 = assign43520_e58585_d_n8;
        locals.var_vds_res_dn9 = assign43520_e58585_d_n9;
        locals.var_vds_res_dn10 = assign43520_e58585_d_n10;
        locals.var_vds_res_dn11 = assign43520_e58585_d_n11;
        locals.var_vds_res_dn14 = assign43520_e58585_d_n14;
        locals.var_vds_res_rv = 0.0;

        let (assign43530_e58596, assign43530_e58596_d_n0, assign43530_e58596_d_n2, assign43530_e58596_d_n4, assign43530_e58596_d_n5, assign43530_e58596_d_n6, assign43530_e58596_d_n7, assign43530_e58596_d_n8, assign43530_e58596_d_n9, assign43530_e58596_d_n10, assign43530_e58596_d_n11, assign43530_e58596_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        let assign43530_e58594: f64 = (locals.var_vgs - locals.var_vbsc);
        (assign43530_e58594, (-locals.var_vbsc_dn0), (-locals.var_vbsc_dn2), (-locals.var_vbsc_dn4), (-locals.var_vbsc_dn5), (locals.var_vgs_dn6 - locals.var_vbsc_dn6), (locals.var_vgs_dn7 - locals.var_vbsc_dn7), (locals.var_vgs_dn8 - locals.var_vbsc_dn8), (-locals.var_vbsc_dn9), (-locals.var_vbsc_dn10), (-locals.var_vbsc_dn11), (-locals.var_vbsc_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign43530_e58596;
        locals.var_t1_dn0 = assign43530_e58596_d_n0;
        locals.var_t1_dn2 = assign43530_e58596_d_n2;
        locals.var_t1_dn4 = assign43530_e58596_d_n4;
        locals.var_t1_dn5 = assign43530_e58596_d_n5;
        locals.var_t1_dn6 = assign43530_e58596_d_n6;
        locals.var_t1_dn7 = assign43530_e58596_d_n7;
        locals.var_t1_dn8 = assign43530_e58596_d_n8;
        locals.var_t1_dn9 = assign43530_e58596_d_n9;
        locals.var_t1_dn10 = assign43530_e58596_d_n10;
        locals.var_t1_dn11 = assign43530_e58596_d_n11;
        locals.var_t1_dn14 = assign43530_e58596_d_n14;
        locals.var_t1_rv = 0.0;

        let assign43540_e58600: f64 = 1.0;
        let assign43540_e58605: f64 = if ((locals.var_t1 < assign43540_e58600) && (1.0 >= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1074 = assign43540_e58605;
        locals.var_guard1074_rv = 0.0;

        let (assign43550_e58620, assign43550_e58620_d_n0, assign43550_e58620_d_n2, assign43550_e58620_d_n4, assign43550_e58620_d_n5, assign43550_e58620_d_n6, assign43550_e58620_d_n7, assign43550_e58620_d_n8, assign43550_e58620_d_n9, assign43550_e58620_d_n10, assign43550_e58620_d_n11, assign43550_e58620_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1074 != 0.0)) {
        let assign43550_e58616: f64 = 1.0;
        let assign43550_e58618: f64 = (assign43550_e58616 - locals.var_t1);
        (assign43550_e58618, (-locals.var_t1_dn0), (-locals.var_t1_dn2), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11), (-locals.var_t1_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign43550_e58620;
        locals.var_tmf1_dn0 = assign43550_e58620_d_n0;
        locals.var_tmf1_dn2 = assign43550_e58620_d_n2;
        locals.var_tmf1_dn4 = assign43550_e58620_d_n4;
        locals.var_tmf1_dn5 = assign43550_e58620_d_n5;
        locals.var_tmf1_dn6 = assign43550_e58620_d_n6;
        locals.var_tmf1_dn7 = assign43550_e58620_d_n7;
        locals.var_tmf1_dn8 = assign43550_e58620_d_n8;
        locals.var_tmf1_dn9 = assign43550_e58620_d_n9;
        locals.var_tmf1_dn10 = assign43550_e58620_d_n10;
        locals.var_tmf1_dn11 = assign43550_e58620_d_n11;
        locals.var_tmf1_dn14 = assign43550_e58620_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign43560_e58633, assign43560_e58633_d_n0, assign43560_e58633_d_n2, assign43560_e58633_d_n4, assign43560_e58633_d_n5, assign43560_e58633_d_n6, assign43560_e58633_d_n7, assign43560_e58633_d_n8, assign43560_e58633_d_n9, assign43560_e58633_d_n10, assign43560_e58633_d_n11, assign43560_e58633_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1074 != 0.0)) {
        let assign43560_e58631: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign43560_e58631, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn7, locals.var_x2_dn8, locals.var_x2_dn9, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn14,)
    }
};
        locals.var_x2 = assign43560_e58633;
        locals.var_x2_dn0 = assign43560_e58633_d_n0;
        locals.var_x2_dn2 = assign43560_e58633_d_n2;
        locals.var_x2_dn4 = assign43560_e58633_d_n4;
        locals.var_x2_dn5 = assign43560_e58633_d_n5;
        locals.var_x2_dn6 = assign43560_e58633_d_n6;
        locals.var_x2_dn7 = assign43560_e58633_d_n7;
        locals.var_x2_dn8 = assign43560_e58633_d_n8;
        locals.var_x2_dn9 = assign43560_e58633_d_n9;
        locals.var_x2_dn10 = assign43560_e58633_d_n10;
        locals.var_x2_dn11 = assign43560_e58633_d_n11;
        locals.var_x2_dn14 = assign43560_e58633_d_n14;
        locals.var_x2_rv = 0.0;

        let (assign43570_e58646, assign43570_e58646_d_n0, assign43570_e58646_d_n2, assign43570_e58646_d_n4, assign43570_e58646_d_n5, assign43570_e58646_d_n6, assign43570_e58646_d_n7, assign43570_e58646_d_n8, assign43570_e58646_d_n9, assign43570_e58646_d_n10, assign43570_e58646_d_n11, assign43570_e58646_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1074 != 0.0)) {
        let assign43570_e58644: f64 = 1.0;
        (assign43570_e58644, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn7, locals.var_xmax2_dn8, locals.var_xmax2_dn9, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn14,)
    }
};
        locals.var_xmax2 = assign43570_e58646;
        locals.var_xmax2_dn0 = assign43570_e58646_d_n0;
        locals.var_xmax2_dn2 = assign43570_e58646_d_n2;
        locals.var_xmax2_dn4 = assign43570_e58646_d_n4;
        locals.var_xmax2_dn5 = assign43570_e58646_d_n5;
        locals.var_xmax2_dn6 = assign43570_e58646_d_n6;
        locals.var_xmax2_dn7 = assign43570_e58646_d_n7;
        locals.var_xmax2_dn8 = assign43570_e58646_d_n8;
        locals.var_xmax2_dn9 = assign43570_e58646_d_n9;
        locals.var_xmax2_dn10 = assign43570_e58646_d_n10;
        locals.var_xmax2_dn11 = assign43570_e58646_d_n11;
        locals.var_xmax2_dn14 = assign43570_e58646_d_n14;
        locals.var_xmax2_rv = 0.0;

        let (assign43580_e58657, assign43580_e58657_d_n0, assign43580_e58657_d_n2, assign43580_e58657_d_n4, assign43580_e58657_d_n5, assign43580_e58657_d_n6, assign43580_e58657_d_n7, assign43580_e58657_d_n8, assign43580_e58657_d_n9, assign43580_e58657_d_n10, assign43580_e58657_d_n11, assign43580_e58657_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1074 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign43580_e58657;
        locals.var_xp_dn0 = assign43580_e58657_d_n0;
        locals.var_xp_dn2 = assign43580_e58657_d_n2;
        locals.var_xp_dn4 = assign43580_e58657_d_n4;
        locals.var_xp_dn5 = assign43580_e58657_d_n5;
        locals.var_xp_dn6 = assign43580_e58657_d_n6;
        locals.var_xp_dn7 = assign43580_e58657_d_n7;
        locals.var_xp_dn8 = assign43580_e58657_d_n8;
        locals.var_xp_dn9 = assign43580_e58657_d_n9;
        locals.var_xp_dn10 = assign43580_e58657_d_n10;
        locals.var_xp_dn11 = assign43580_e58657_d_n11;
        locals.var_xp_dn14 = assign43580_e58657_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign43590_e58668, assign43590_e58668_d_n0, assign43590_e58668_d_n2, assign43590_e58668_d_n4, assign43590_e58668_d_n5, assign43590_e58668_d_n6, assign43590_e58668_d_n7, assign43590_e58668_d_n8, assign43590_e58668_d_n9, assign43590_e58668_d_n10, assign43590_e58668_d_n11, assign43590_e58668_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1074 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign43590_e58668;
        locals.var_xmp_dn0 = assign43590_e58668_d_n0;
        locals.var_xmp_dn2 = assign43590_e58668_d_n2;
        locals.var_xmp_dn4 = assign43590_e58668_d_n4;
        locals.var_xmp_dn5 = assign43590_e58668_d_n5;
        locals.var_xmp_dn6 = assign43590_e58668_d_n6;
        locals.var_xmp_dn7 = assign43590_e58668_d_n7;
        locals.var_xmp_dn8 = assign43590_e58668_d_n8;
        locals.var_xmp_dn9 = assign43590_e58668_d_n9;
        locals.var_xmp_dn10 = assign43590_e58668_d_n10;
        locals.var_xmp_dn11 = assign43590_e58668_d_n11;
        locals.var_xmp_dn14 = assign43590_e58668_d_n14;
        locals.var_xmp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_151(
        locals: &mut StampLocals,
    ) {
        let (assign43600_e58679,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1074 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign43600_e58679;
        locals.var_m0_rv = 0.0;

        let (assign43610_e58690,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1074 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign43610_e58690;
        locals.var_mm_rv = 0.0;

        let (assign43620_e58701, assign43620_e58701_d_n0, assign43620_e58701_d_n2, assign43620_e58701_d_n4, assign43620_e58701_d_n5, assign43620_e58701_d_n6, assign43620_e58701_d_n7, assign43620_e58701_d_n8, assign43620_e58701_d_n9, assign43620_e58701_d_n10, assign43620_e58701_d_n11, assign43620_e58701_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1074 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign43620_e58701;
        locals.var_arg_dn0 = assign43620_e58701_d_n0;
        locals.var_arg_dn2 = assign43620_e58701_d_n2;
        locals.var_arg_dn4 = assign43620_e58701_d_n4;
        locals.var_arg_dn5 = assign43620_e58701_d_n5;
        locals.var_arg_dn6 = assign43620_e58701_d_n6;
        locals.var_arg_dn7 = assign43620_e58701_d_n7;
        locals.var_arg_dn8 = assign43620_e58701_d_n8;
        locals.var_arg_dn9 = assign43620_e58701_d_n9;
        locals.var_arg_dn10 = assign43620_e58701_d_n10;
        locals.var_arg_dn11 = assign43620_e58701_d_n11;
        locals.var_arg_dn14 = assign43620_e58701_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign43630_e58712, assign43630_e58712_d_n0, assign43630_e58712_d_n2, assign43630_e58712_d_n4, assign43630_e58712_d_n5, assign43630_e58712_d_n6, assign43630_e58712_d_n7, assign43630_e58712_d_n8, assign43630_e58712_d_n9, assign43630_e58712_d_n10, assign43630_e58712_d_n11, assign43630_e58712_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1074 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign43630_e58712;
        locals.var_dnm_dn0 = assign43630_e58712_d_n0;
        locals.var_dnm_dn2 = assign43630_e58712_d_n2;
        locals.var_dnm_dn4 = assign43630_e58712_d_n4;
        locals.var_dnm_dn5 = assign43630_e58712_d_n5;
        locals.var_dnm_dn6 = assign43630_e58712_d_n6;
        locals.var_dnm_dn7 = assign43630_e58712_d_n7;
        locals.var_dnm_dn8 = assign43630_e58712_d_n8;
        locals.var_dnm_dn9 = assign43630_e58712_d_n9;
        locals.var_dnm_dn10 = assign43630_e58712_d_n10;
        locals.var_dnm_dn11 = assign43630_e58712_d_n11;
        locals.var_dnm_dn14 = assign43630_e58712_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign43640_e58725, assign43640_e58725_d_n0, assign43640_e58725_d_n2, assign43640_e58725_d_n4, assign43640_e58725_d_n5, assign43640_e58725_d_n6, assign43640_e58725_d_n7, assign43640_e58725_d_n8, assign43640_e58725_d_n9, assign43640_e58725_d_n10, assign43640_e58725_d_n11, assign43640_e58725_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1074 != 0.0)) {
        let assign43640_e58723: f64 = (locals.var_xp * locals.var_x2);
        (assign43640_e58723, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign43640_e58725;
        locals.var_xp_dn0 = assign43640_e58725_d_n0;
        locals.var_xp_dn2 = assign43640_e58725_d_n2;
        locals.var_xp_dn4 = assign43640_e58725_d_n4;
        locals.var_xp_dn5 = assign43640_e58725_d_n5;
        locals.var_xp_dn6 = assign43640_e58725_d_n6;
        locals.var_xp_dn7 = assign43640_e58725_d_n7;
        locals.var_xp_dn8 = assign43640_e58725_d_n8;
        locals.var_xp_dn9 = assign43640_e58725_d_n9;
        locals.var_xp_dn10 = assign43640_e58725_d_n10;
        locals.var_xp_dn11 = assign43640_e58725_d_n11;
        locals.var_xp_dn14 = assign43640_e58725_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign43650_e58738, assign43650_e58738_d_n0, assign43650_e58738_d_n2, assign43650_e58738_d_n4, assign43650_e58738_d_n5, assign43650_e58738_d_n6, assign43650_e58738_d_n7, assign43650_e58738_d_n8, assign43650_e58738_d_n9, assign43650_e58738_d_n10, assign43650_e58738_d_n11, assign43650_e58738_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1074 != 0.0)) {
        let assign43650_e58736: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign43650_e58736, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign43650_e58738;
        locals.var_xmp_dn0 = assign43650_e58738_d_n0;
        locals.var_xmp_dn2 = assign43650_e58738_d_n2;
        locals.var_xmp_dn4 = assign43650_e58738_d_n4;
        locals.var_xmp_dn5 = assign43650_e58738_d_n5;
        locals.var_xmp_dn6 = assign43650_e58738_d_n6;
        locals.var_xmp_dn7 = assign43650_e58738_d_n7;
        locals.var_xmp_dn8 = assign43650_e58738_d_n8;
        locals.var_xmp_dn9 = assign43650_e58738_d_n9;
        locals.var_xmp_dn10 = assign43650_e58738_d_n10;
        locals.var_xmp_dn11 = assign43650_e58738_d_n11;
        locals.var_xmp_dn14 = assign43650_e58738_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign43660_e58751, assign43660_e58751_d_n0, assign43660_e58751_d_n2, assign43660_e58751_d_n4, assign43660_e58751_d_n5, assign43660_e58751_d_n6, assign43660_e58751_d_n7, assign43660_e58751_d_n8, assign43660_e58751_d_n9, assign43660_e58751_d_n10, assign43660_e58751_d_n11, assign43660_e58751_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1074 != 0.0)) {
        let assign43660_e58749: f64 = (locals.var_xp * locals.var_x2);
        (assign43660_e58749, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn7 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn7)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn9 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn9)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn14 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn14)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn7, locals.var_xp_dn8, locals.var_xp_dn9, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn14,)
    }
};
        locals.var_xp = assign43660_e58751;
        locals.var_xp_dn0 = assign43660_e58751_d_n0;
        locals.var_xp_dn2 = assign43660_e58751_d_n2;
        locals.var_xp_dn4 = assign43660_e58751_d_n4;
        locals.var_xp_dn5 = assign43660_e58751_d_n5;
        locals.var_xp_dn6 = assign43660_e58751_d_n6;
        locals.var_xp_dn7 = assign43660_e58751_d_n7;
        locals.var_xp_dn8 = assign43660_e58751_d_n8;
        locals.var_xp_dn9 = assign43660_e58751_d_n9;
        locals.var_xp_dn10 = assign43660_e58751_d_n10;
        locals.var_xp_dn11 = assign43660_e58751_d_n11;
        locals.var_xp_dn14 = assign43660_e58751_d_n14;
        locals.var_xp_rv = 0.0;

        let (assign43670_e58764, assign43670_e58764_d_n0, assign43670_e58764_d_n2, assign43670_e58764_d_n4, assign43670_e58764_d_n5, assign43670_e58764_d_n6, assign43670_e58764_d_n7, assign43670_e58764_d_n8, assign43670_e58764_d_n9, assign43670_e58764_d_n10, assign43670_e58764_d_n11, assign43670_e58764_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1074 != 0.0)) {
        let assign43670_e58762: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign43670_e58762, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn7 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn7)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn9 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn9)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn14 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn14)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn7, locals.var_xmp_dn8, locals.var_xmp_dn9, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn14,)
    }
};
        locals.var_xmp = assign43670_e58764;
        locals.var_xmp_dn0 = assign43670_e58764_d_n0;
        locals.var_xmp_dn2 = assign43670_e58764_d_n2;
        locals.var_xmp_dn4 = assign43670_e58764_d_n4;
        locals.var_xmp_dn5 = assign43670_e58764_d_n5;
        locals.var_xmp_dn6 = assign43670_e58764_d_n6;
        locals.var_xmp_dn7 = assign43670_e58764_d_n7;
        locals.var_xmp_dn8 = assign43670_e58764_d_n8;
        locals.var_xmp_dn9 = assign43670_e58764_d_n9;
        locals.var_xmp_dn10 = assign43670_e58764_d_n10;
        locals.var_xmp_dn11 = assign43670_e58764_d_n11;
        locals.var_xmp_dn14 = assign43670_e58764_d_n14;
        locals.var_xmp_rv = 0.0;

        let (assign43680_e58777, assign43680_e58777_d_n0, assign43680_e58777_d_n2, assign43680_e58777_d_n4, assign43680_e58777_d_n5, assign43680_e58777_d_n6, assign43680_e58777_d_n7, assign43680_e58777_d_n8, assign43680_e58777_d_n9, assign43680_e58777_d_n10, assign43680_e58777_d_n11, assign43680_e58777_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1074 != 0.0)) {
        let assign43680_e58775: f64 = (locals.var_xp + locals.var_xmp);
        (assign43680_e58775, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn7 + locals.var_xmp_dn7), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn9 + locals.var_xmp_dn9), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn14 + locals.var_xmp_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign43680_e58777;
        locals.var_arg_dn0 = assign43680_e58777_d_n0;
        locals.var_arg_dn2 = assign43680_e58777_d_n2;
        locals.var_arg_dn4 = assign43680_e58777_d_n4;
        locals.var_arg_dn5 = assign43680_e58777_d_n5;
        locals.var_arg_dn6 = assign43680_e58777_d_n6;
        locals.var_arg_dn7 = assign43680_e58777_d_n7;
        locals.var_arg_dn8 = assign43680_e58777_d_n8;
        locals.var_arg_dn9 = assign43680_e58777_d_n9;
        locals.var_arg_dn10 = assign43680_e58777_d_n10;
        locals.var_arg_dn11 = assign43680_e58777_d_n11;
        locals.var_arg_dn14 = assign43680_e58777_d_n14;
        locals.var_arg_rv = 0.0;

        let (assign43690_e58788, assign43690_e58788_d_n0, assign43690_e58788_d_n2, assign43690_e58788_d_n4, assign43690_e58788_d_n5, assign43690_e58788_d_n6, assign43690_e58788_d_n7, assign43690_e58788_d_n8, assign43690_e58788_d_n9, assign43690_e58788_d_n10, assign43690_e58788_d_n11, assign43690_e58788_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1074 != 0.0)) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign43690_e58788;
        locals.var_dnm_dn0 = assign43690_e58788_d_n0;
        locals.var_dnm_dn2 = assign43690_e58788_d_n2;
        locals.var_dnm_dn4 = assign43690_e58788_d_n4;
        locals.var_dnm_dn5 = assign43690_e58788_d_n5;
        locals.var_dnm_dn6 = assign43690_e58788_d_n6;
        locals.var_dnm_dn7 = assign43690_e58788_d_n7;
        locals.var_dnm_dn8 = assign43690_e58788_d_n8;
        locals.var_dnm_dn9 = assign43690_e58788_d_n9;
        locals.var_dnm_dn10 = assign43690_e58788_d_n10;
        locals.var_dnm_dn11 = assign43690_e58788_d_n11;
        locals.var_dnm_dn14 = assign43690_e58788_d_n14;
        locals.var_dnm_rv = 0.0;

        let assign43700_e58803: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard1075 = assign43700_e58803;
        locals.var_guard1075_rv = 0.0;

        let assign43710_e58806: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1076 = assign43710_e58806;
        locals.var_guard1076_rv = 0.0;

        let (assign43720_e58821,) = {
    if (((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign43720_e58821;
        locals.var_mm_rv = 0.0;

        let assign43730_e58824: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1077 = assign43730_e58824;
        locals.var_guard1077_rv = 0.0;

        let (assign43740_e58842,) = {
    if ((((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 == 0.0)) && (locals.var_guard1077 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign43740_e58842;
        locals.var_mm_rv = 0.0;

        let assign43750_e58845: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard1078 = assign43750_e58845;
        locals.var_guard1078_rv = 0.0;

        let (assign43760_e58866,) = {
    if (((((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 == 0.0)) && (locals.var_guard1077 == 0.0)) && (locals.var_guard1078 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign43760_e58866;
        locals.var_mm_rv = 0.0;

        let assign43770_e58869: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard1079 = assign43770_e58869;
        locals.var_guard1079_rv = 0.0;

        let (assign43780_e58893,) = {
    if ((((((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1075 != 0.0)) && (locals.var_guard1076 == 0.0)) && (locals.var_guard1077 == 0.0)) && (locals.var_guard1078 == 0.0)) && (locals.var_guard1079 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign43780_e58893;
        locals.var_mm_rv = 0.0;

        let (assign43790_e58906,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1075 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign43790_e58906;
        locals.var_m0_rv = 0.0;

        let mut assign43800_loop_guard: usize = 0;
        while {
            let assign43800_cond_e58920: f64 = if (((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1075 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign43800_cond_e58920 != 0.0
        } {
            assign43800_loop_guard += 1;
            assert!(assign43800_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign43800_body0_e58934, assign43800_body0_e58934_d_n0, assign43800_body0_e58934_d_n2, assign43800_body0_e58934_d_n4, assign43800_body0_e58934_d_n5, assign43800_body0_e58934_d_n6, assign43800_body0_e58934_d_n7, assign43800_body0_e58934_d_n8, assign43800_body0_e58934_d_n9, assign43800_body0_e58934_d_n10, assign43800_body0_e58934_d_n11, assign43800_body0_e58934_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1075 != 0.0)) {
        let assign43800_body0_e58932: f64 = (locals.var_dnm).sqrt();
        (assign43800_body0_e58932, (locals.var_dnm_dn0 / (2.0 * assign43800_body0_e58932)), (locals.var_dnm_dn2 / (2.0 * assign43800_body0_e58932)), (locals.var_dnm_dn4 / (2.0 * assign43800_body0_e58932)), (locals.var_dnm_dn5 / (2.0 * assign43800_body0_e58932)), (locals.var_dnm_dn6 / (2.0 * assign43800_body0_e58932)), (locals.var_dnm_dn7 / (2.0 * assign43800_body0_e58932)), (locals.var_dnm_dn8 / (2.0 * assign43800_body0_e58932)), (locals.var_dnm_dn9 / (2.0 * assign43800_body0_e58932)), (locals.var_dnm_dn10 / (2.0 * assign43800_body0_e58932)), (locals.var_dnm_dn11 / (2.0 * assign43800_body0_e58932)), (locals.var_dnm_dn14 / (2.0 * assign43800_body0_e58932)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
            locals.var_dnm = assign43800_body0_e58934;
            locals.var_dnm_dn0 = assign43800_body0_e58934_d_n0;
            locals.var_dnm_dn2 = assign43800_body0_e58934_d_n2;
            locals.var_dnm_dn4 = assign43800_body0_e58934_d_n4;
            locals.var_dnm_dn5 = assign43800_body0_e58934_d_n5;
            locals.var_dnm_dn6 = assign43800_body0_e58934_d_n6;
            locals.var_dnm_dn7 = assign43800_body0_e58934_d_n7;
            locals.var_dnm_dn8 = assign43800_body0_e58934_d_n8;
            locals.var_dnm_dn9 = assign43800_body0_e58934_d_n9;
            locals.var_dnm_dn10 = assign43800_body0_e58934_d_n10;
            locals.var_dnm_dn11 = assign43800_body0_e58934_d_n11;
            locals.var_dnm_dn14 = assign43800_body0_e58934_d_n14;
            locals.var_dnm_rv = 0.0;
            let (assign43800_body1_e58949,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1075 != 0.0)) {
        let assign43800_body1_e58947: f64 = (locals.var_m0 + 1.0);
        (assign43800_body1_e58947,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign43800_body1_e58949;
            locals.var_m0_rv = 0.0;
        }

        let (assign43810_e58974, assign43810_e58974_d_n0, assign43810_e58974_d_n2, assign43810_e58974_d_n4, assign43810_e58974_d_n5, assign43810_e58974_d_n6, assign43810_e58974_d_n7, assign43810_e58974_d_n8, assign43810_e58974_d_n9, assign43810_e58974_d_n10, assign43810_e58974_d_n11, assign43810_e58974_d_n14,) = {
    if ((((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1074 != 0.0)) && (locals.var_guard1075 == 0.0)) {
        let (assign43810_e58972, assign43810_e58972_d_n0, assign43810_e58972_d_n2, assign43810_e58972_d_n4, assign43810_e58972_d_n5, assign43810_e58972_d_n6, assign43810_e58972_d_n7, assign43810_e58972_d_n8, assign43810_e58972_d_n9, assign43810_e58972_d_n10, assign43810_e58972_d_n11, assign43810_e58972_d_n14,) = {
            if (locals.var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign43810_e58969: f64 = (2.0 * 2.0);
                let assign43810_e58970: f64 = (1.0 / assign43810_e58969);
                let assign43810_e58971: f64 = (locals.var_dnm).powf(assign43810_e58970);
                (assign43810_e58971, if 0.0 == 0.0 && ((assign43810_e58970) as f64).is_finite() && ((assign43810_e58970) as f64).fract() == 0.0 { if assign43810_e58970 == 0.0 { 0.0 } else { (assign43810_e58970 * ((locals.var_dnm).powf(assign43810_e58970 - 1.0) * locals.var_dnm_dn0)) } } else { (assign43810_e58971 * (assign43810_e58970 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43810_e58970) as f64).is_finite() && ((assign43810_e58970) as f64).fract() == 0.0 { if assign43810_e58970 == 0.0 { 0.0 } else { (assign43810_e58970 * ((locals.var_dnm).powf(assign43810_e58970 - 1.0) * locals.var_dnm_dn2)) } } else { (assign43810_e58971 * (assign43810_e58970 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43810_e58970) as f64).is_finite() && ((assign43810_e58970) as f64).fract() == 0.0 { if assign43810_e58970 == 0.0 { 0.0 } else { (assign43810_e58970 * ((locals.var_dnm).powf(assign43810_e58970 - 1.0) * locals.var_dnm_dn4)) } } else { (assign43810_e58971 * (assign43810_e58970 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43810_e58970) as f64).is_finite() && ((assign43810_e58970) as f64).fract() == 0.0 { if assign43810_e58970 == 0.0 { 0.0 } else { (assign43810_e58970 * ((locals.var_dnm).powf(assign43810_e58970 - 1.0) * locals.var_dnm_dn5)) } } else { (assign43810_e58971 * (assign43810_e58970 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43810_e58970) as f64).is_finite() && ((assign43810_e58970) as f64).fract() == 0.0 { if assign43810_e58970 == 0.0 { 0.0 } else { (assign43810_e58970 * ((locals.var_dnm).powf(assign43810_e58970 - 1.0) * locals.var_dnm_dn6)) } } else { (assign43810_e58971 * (assign43810_e58970 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43810_e58970) as f64).is_finite() && ((assign43810_e58970) as f64).fract() == 0.0 { if assign43810_e58970 == 0.0 { 0.0 } else { (assign43810_e58970 * ((locals.var_dnm).powf(assign43810_e58970 - 1.0) * locals.var_dnm_dn7)) } } else { (assign43810_e58971 * (assign43810_e58970 * (locals.var_dnm_dn7 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43810_e58970) as f64).is_finite() && ((assign43810_e58970) as f64).fract() == 0.0 { if assign43810_e58970 == 0.0 { 0.0 } else { (assign43810_e58970 * ((locals.var_dnm).powf(assign43810_e58970 - 1.0) * locals.var_dnm_dn8)) } } else { (assign43810_e58971 * (assign43810_e58970 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43810_e58970) as f64).is_finite() && ((assign43810_e58970) as f64).fract() == 0.0 { if assign43810_e58970 == 0.0 { 0.0 } else { (assign43810_e58970 * ((locals.var_dnm).powf(assign43810_e58970 - 1.0) * locals.var_dnm_dn9)) } } else { (assign43810_e58971 * (assign43810_e58970 * (locals.var_dnm_dn9 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43810_e58970) as f64).is_finite() && ((assign43810_e58970) as f64).fract() == 0.0 { if assign43810_e58970 == 0.0 { 0.0 } else { (assign43810_e58970 * ((locals.var_dnm).powf(assign43810_e58970 - 1.0) * locals.var_dnm_dn10)) } } else { (assign43810_e58971 * (assign43810_e58970 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43810_e58970) as f64).is_finite() && ((assign43810_e58970) as f64).fract() == 0.0 { if assign43810_e58970 == 0.0 { 0.0 } else { (assign43810_e58970 * ((locals.var_dnm).powf(assign43810_e58970 - 1.0) * locals.var_dnm_dn11)) } } else { (assign43810_e58971 * (assign43810_e58970 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign43810_e58970) as f64).is_finite() && ((assign43810_e58970) as f64).fract() == 0.0 { if assign43810_e58970 == 0.0 { 0.0 } else { (assign43810_e58970 * ((locals.var_dnm).powf(assign43810_e58970 - 1.0) * locals.var_dnm_dn14)) } } else { (assign43810_e58971 * (assign43810_e58970 * (locals.var_dnm_dn14 / locals.var_dnm))) },)
            }
        };
        (assign43810_e58972, assign43810_e58972_d_n0, assign43810_e58972_d_n2, assign43810_e58972_d_n4, assign43810_e58972_d_n5, assign43810_e58972_d_n6, assign43810_e58972_d_n7, assign43810_e58972_d_n8, assign43810_e58972_d_n9, assign43810_e58972_d_n10, assign43810_e58972_d_n11, assign43810_e58972_d_n14,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign43810_e58974;
        locals.var_dnm_dn0 = assign43810_e58974_d_n0;
        locals.var_dnm_dn2 = assign43810_e58974_d_n2;
        locals.var_dnm_dn4 = assign43810_e58974_d_n4;
        locals.var_dnm_dn5 = assign43810_e58974_d_n5;
        locals.var_dnm_dn6 = assign43810_e58974_d_n6;
        locals.var_dnm_dn7 = assign43810_e58974_d_n7;
        locals.var_dnm_dn8 = assign43810_e58974_d_n8;
        locals.var_dnm_dn9 = assign43810_e58974_d_n9;
        locals.var_dnm_dn10 = assign43810_e58974_d_n10;
        locals.var_dnm_dn11 = assign43810_e58974_d_n11;
        locals.var_dnm_dn14 = assign43810_e58974_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign43820_e58987, assign43820_e58987_d_n0, assign43820_e58987_d_n2, assign43820_e58987_d_n4, assign43820_e58987_d_n5, assign43820_e58987_d_n6, assign43820_e58987_d_n7, assign43820_e58987_d_n8, assign43820_e58987_d_n9, assign43820_e58987_d_n10, assign43820_e58987_d_n11, assign43820_e58987_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1074 != 0.0)) {
        let assign43820_e58985: f64 = (1.0 / locals.var_dnm);
        (assign43820_e58985, (-(locals.var_dnm_dn0 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn2 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn4 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn5 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn6 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn7 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn8 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn9 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn10 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn11 / (locals.var_dnm * locals.var_dnm))), (-(locals.var_dnm_dn14 / (locals.var_dnm * locals.var_dnm))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn7, locals.var_dnm_dn8, locals.var_dnm_dn9, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn14,)
    }
};
        locals.var_dnm = assign43820_e58987;
        locals.var_dnm_dn0 = assign43820_e58987_d_n0;
        locals.var_dnm_dn2 = assign43820_e58987_d_n2;
        locals.var_dnm_dn4 = assign43820_e58987_d_n4;
        locals.var_dnm_dn5 = assign43820_e58987_d_n5;
        locals.var_dnm_dn6 = assign43820_e58987_d_n6;
        locals.var_dnm_dn7 = assign43820_e58987_d_n7;
        locals.var_dnm_dn8 = assign43820_e58987_d_n8;
        locals.var_dnm_dn9 = assign43820_e58987_d_n9;
        locals.var_dnm_dn10 = assign43820_e58987_d_n10;
        locals.var_dnm_dn11 = assign43820_e58987_d_n11;
        locals.var_dnm_dn14 = assign43820_e58987_d_n14;
        locals.var_dnm_rv = 0.0;

        let (assign43830_e59002, assign43830_e59002_d_n0, assign43830_e59002_d_n2, assign43830_e59002_d_n4, assign43830_e59002_d_n5, assign43830_e59002_d_n6, assign43830_e59002_d_n7, assign43830_e59002_d_n8, assign43830_e59002_d_n9, assign43830_e59002_d_n10, assign43830_e59002_d_n11, assign43830_e59002_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1074 != 0.0)) {
        let assign43830_e58998: f64 = locals.var_tmf1;
        let assign43830_e59000: f64 = (assign43830_e58998 * locals.var_dnm);
        (assign43830_e59000, ((locals.var_tmf1_dn0 * locals.var_dnm) + (assign43830_e58998 * locals.var_dnm_dn0)), ((locals.var_tmf1_dn2 * locals.var_dnm) + (assign43830_e58998 * locals.var_dnm_dn2)), ((locals.var_tmf1_dn4 * locals.var_dnm) + (assign43830_e58998 * locals.var_dnm_dn4)), ((locals.var_tmf1_dn5 * locals.var_dnm) + (assign43830_e58998 * locals.var_dnm_dn5)), ((locals.var_tmf1_dn6 * locals.var_dnm) + (assign43830_e58998 * locals.var_dnm_dn6)), ((locals.var_tmf1_dn7 * locals.var_dnm) + (assign43830_e58998 * locals.var_dnm_dn7)), ((locals.var_tmf1_dn8 * locals.var_dnm) + (assign43830_e58998 * locals.var_dnm_dn8)), ((locals.var_tmf1_dn9 * locals.var_dnm) + (assign43830_e58998 * locals.var_dnm_dn9)), ((locals.var_tmf1_dn10 * locals.var_dnm) + (assign43830_e58998 * locals.var_dnm_dn10)), ((locals.var_tmf1_dn11 * locals.var_dnm) + (assign43830_e58998 * locals.var_dnm_dn11)), ((locals.var_tmf1_dn14 * locals.var_dnm) + (assign43830_e58998 * locals.var_dnm_dn14)),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign43830_e59002;
        locals.var_tmf0_dn0 = assign43830_e59002_d_n0;
        locals.var_tmf0_dn2 = assign43830_e59002_d_n2;
        locals.var_tmf0_dn4 = assign43830_e59002_d_n4;
        locals.var_tmf0_dn5 = assign43830_e59002_d_n5;
        locals.var_tmf0_dn6 = assign43830_e59002_d_n6;
        locals.var_tmf0_dn7 = assign43830_e59002_d_n7;
        locals.var_tmf0_dn8 = assign43830_e59002_d_n8;
        locals.var_tmf0_dn9 = assign43830_e59002_d_n9;
        locals.var_tmf0_dn10 = assign43830_e59002_d_n10;
        locals.var_tmf0_dn11 = assign43830_e59002_d_n11;
        locals.var_tmf0_dn14 = assign43830_e59002_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign43840_e59019, assign43840_e59019_d_n0, assign43840_e59019_d_n2, assign43840_e59019_d_n4, assign43840_e59019_d_n5, assign43840_e59019_d_n6, assign43840_e59019_d_n7, assign43840_e59019_d_n8, assign43840_e59019_d_n9, assign43840_e59019_d_n10, assign43840_e59019_d_n11, assign43840_e59019_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1074 != 0.0)) {
        let assign43840_e59013: f64 = locals.var_xmp;
        let assign43840_e59015: f64 = (assign43840_e59013 * locals.var_dnm);
        let assign43840_e59017: f64 = (assign43840_e59015 / locals.var_arg);
        (assign43840_e59017, (((((locals.var_xmp_dn0 * locals.var_dnm) + (assign43840_e59013 * locals.var_dnm_dn0)) * locals.var_arg) - (assign43840_e59015 * locals.var_arg_dn0)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn2 * locals.var_dnm) + (assign43840_e59013 * locals.var_dnm_dn2)) * locals.var_arg) - (assign43840_e59015 * locals.var_arg_dn2)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn4 * locals.var_dnm) + (assign43840_e59013 * locals.var_dnm_dn4)) * locals.var_arg) - (assign43840_e59015 * locals.var_arg_dn4)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn5 * locals.var_dnm) + (assign43840_e59013 * locals.var_dnm_dn5)) * locals.var_arg) - (assign43840_e59015 * locals.var_arg_dn5)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn6 * locals.var_dnm) + (assign43840_e59013 * locals.var_dnm_dn6)) * locals.var_arg) - (assign43840_e59015 * locals.var_arg_dn6)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn7 * locals.var_dnm) + (assign43840_e59013 * locals.var_dnm_dn7)) * locals.var_arg) - (assign43840_e59015 * locals.var_arg_dn7)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn8 * locals.var_dnm) + (assign43840_e59013 * locals.var_dnm_dn8)) * locals.var_arg) - (assign43840_e59015 * locals.var_arg_dn8)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn9 * locals.var_dnm) + (assign43840_e59013 * locals.var_dnm_dn9)) * locals.var_arg) - (assign43840_e59015 * locals.var_arg_dn9)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn10 * locals.var_dnm) + (assign43840_e59013 * locals.var_dnm_dn10)) * locals.var_arg) - (assign43840_e59015 * locals.var_arg_dn10)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn11 * locals.var_dnm) + (assign43840_e59013 * locals.var_dnm_dn11)) * locals.var_arg) - (assign43840_e59015 * locals.var_arg_dn11)) / (locals.var_arg * locals.var_arg)), (((((locals.var_xmp_dn14 * locals.var_dnm) + (assign43840_e59013 * locals.var_dnm_dn14)) * locals.var_arg) - (assign43840_e59015 * locals.var_arg_dn14)) / (locals.var_arg * locals.var_arg)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign43840_e59019;
        locals.var_t0_dn0 = assign43840_e59019_d_n0;
        locals.var_t0_dn2 = assign43840_e59019_d_n2;
        locals.var_t0_dn4 = assign43840_e59019_d_n4;
        locals.var_t0_dn5 = assign43840_e59019_d_n5;
        locals.var_t0_dn6 = assign43840_e59019_d_n6;
        locals.var_t0_dn7 = assign43840_e59019_d_n7;
        locals.var_t0_dn8 = assign43840_e59019_d_n8;
        locals.var_t0_dn9 = assign43840_e59019_d_n9;
        locals.var_t0_dn10 = assign43840_e59019_d_n10;
        locals.var_t0_dn11 = assign43840_e59019_d_n11;
        locals.var_t0_dn14 = assign43840_e59019_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign43850_e59034, assign43850_e59034_d_n0, assign43850_e59034_d_n2, assign43850_e59034_d_n4, assign43850_e59034_d_n5, assign43850_e59034_d_n6, assign43850_e59034_d_n7, assign43850_e59034_d_n8, assign43850_e59034_d_n9, assign43850_e59034_d_n10, assign43850_e59034_d_n11, assign43850_e59034_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1074 != 0.0)) {
        let assign43850_e59030: f64 = 1.0;
        let assign43850_e59032: f64 = (assign43850_e59030 - locals.var_tmf0);
        (assign43850_e59032, (-locals.var_tmf0_dn0), (-locals.var_tmf0_dn2), (-locals.var_tmf0_dn4), (-locals.var_tmf0_dn5), (-locals.var_tmf0_dn6), (-locals.var_tmf0_dn7), (-locals.var_tmf0_dn8), (-locals.var_tmf0_dn9), (-locals.var_tmf0_dn10), (-locals.var_tmf0_dn11), (-locals.var_tmf0_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign43850_e59034;
        locals.var_t1_dn0 = assign43850_e59034_d_n0;
        locals.var_t1_dn2 = assign43850_e59034_d_n2;
        locals.var_t1_dn4 = assign43850_e59034_d_n4;
        locals.var_t1_dn5 = assign43850_e59034_d_n5;
        locals.var_t1_dn6 = assign43850_e59034_d_n6;
        locals.var_t1_dn7 = assign43850_e59034_d_n7;
        locals.var_t1_dn8 = assign43850_e59034_d_n8;
        locals.var_t1_dn9 = assign43850_e59034_d_n9;
        locals.var_t1_dn10 = assign43850_e59034_d_n10;
        locals.var_t1_dn11 = assign43850_e59034_d_n11;
        locals.var_t1_dn14 = assign43850_e59034_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign43860_e59045, assign43860_e59045_d_n0, assign43860_e59045_d_n2, assign43860_e59045_d_n4, assign43860_e59045_d_n5, assign43860_e59045_d_n6, assign43860_e59045_d_n7, assign43860_e59045_d_n8, assign43860_e59045_d_n9, assign43860_e59045_d_n10, assign43860_e59045_d_n11, assign43860_e59045_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1074 != 0.0)) {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign43860_e59045;
        locals.var_t0_dn0 = assign43860_e59045_d_n0;
        locals.var_t0_dn2 = assign43860_e59045_d_n2;
        locals.var_t0_dn4 = assign43860_e59045_d_n4;
        locals.var_t0_dn5 = assign43860_e59045_d_n5;
        locals.var_t0_dn6 = assign43860_e59045_d_n6;
        locals.var_t0_dn7 = assign43860_e59045_d_n7;
        locals.var_t0_dn8 = assign43860_e59045_d_n8;
        locals.var_t0_dn9 = assign43860_e59045_d_n9;
        locals.var_t0_dn10 = assign43860_e59045_d_n10;
        locals.var_t0_dn11 = assign43860_e59045_d_n11;
        locals.var_t0_dn14 = assign43860_e59045_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign43870_e59057, assign43870_e59057_d_n0, assign43870_e59057_d_n2, assign43870_e59057_d_n4, assign43870_e59057_d_n5, assign43870_e59057_d_n6, assign43870_e59057_d_n7, assign43870_e59057_d_n8, assign43870_e59057_d_n9, assign43870_e59057_d_n10, assign43870_e59057_d_n11, assign43870_e59057_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1074 == 0.0)) {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign43870_e59057;
        locals.var_t1_dn0 = assign43870_e59057_d_n0;
        locals.var_t1_dn2 = assign43870_e59057_d_n2;
        locals.var_t1_dn4 = assign43870_e59057_d_n4;
        locals.var_t1_dn5 = assign43870_e59057_d_n5;
        locals.var_t1_dn6 = assign43870_e59057_d_n6;
        locals.var_t1_dn7 = assign43870_e59057_d_n7;
        locals.var_t1_dn8 = assign43870_e59057_d_n8;
        locals.var_t1_dn9 = assign43870_e59057_d_n9;
        locals.var_t1_dn10 = assign43870_e59057_d_n10;
        locals.var_t1_dn11 = assign43870_e59057_d_n11;
        locals.var_t1_dn14 = assign43870_e59057_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign43880_e59069, assign43880_e59069_d_n0, assign43880_e59069_d_n2, assign43880_e59069_d_n4, assign43880_e59069_d_n5, assign43880_e59069_d_n6, assign43880_e59069_d_n7, assign43880_e59069_d_n8, assign43880_e59069_d_n9, assign43880_e59069_d_n10, assign43880_e59069_d_n11, assign43880_e59069_d_n14,) = {
    if (((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) && (locals.var_guard1074 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign43880_e59069;
        locals.var_t0_dn0 = assign43880_e59069_d_n0;
        locals.var_t0_dn2 = assign43880_e59069_d_n2;
        locals.var_t0_dn4 = assign43880_e59069_d_n4;
        locals.var_t0_dn5 = assign43880_e59069_d_n5;
        locals.var_t0_dn6 = assign43880_e59069_d_n6;
        locals.var_t0_dn7 = assign43880_e59069_d_n7;
        locals.var_t0_dn8 = assign43880_e59069_d_n8;
        locals.var_t0_dn9 = assign43880_e59069_d_n9;
        locals.var_t0_dn10 = assign43880_e59069_d_n10;
        locals.var_t0_dn11 = assign43880_e59069_d_n11;
        locals.var_t0_dn14 = assign43880_e59069_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign43890_e59080, assign43890_e59080_d_n0, assign43890_e59080_d_n2, assign43890_e59080_d_n4, assign43890_e59080_d_n5, assign43890_e59080_d_n6, assign43890_e59080_d_n7, assign43890_e59080_d_n8, assign43890_e59080_d_n9, assign43890_e59080_d_n10, assign43890_e59080_d_n11, assign43890_e59080_d_n14,) = {
    if ((locals.var_guard447 != 0.0) && ((locals.var_guard449 != 0.0) && (locals.var_guard448 == 0.0))) {
        let assign43890_e59078: f64 = (locals.var_t1 / locals.var_uc_depthn);
        (assign43890_e59078, (((locals.var_t1_dn0 * locals.var_uc_depthn) - (locals.var_t1 * locals.var_uc_depthn_dn0)) / (locals.var_uc_depthn * locals.var_uc_depthn)), (((locals.var_t1_dn2 * locals.var_uc_depthn) - (locals.var_t1 * locals.var_uc_depthn_dn2)) / (locals.var_uc_depthn * locals.var_uc_depthn)), (((locals.var_t1_dn4 * locals.var_uc_depthn) - (locals.var_t1 * locals.var_uc_depthn_dn4)) / (locals.var_uc_depthn * locals.var_uc_depthn)), (((locals.var_t1_dn5 * locals.var_uc_depthn) - (locals.var_t1 * locals.var_uc_depthn_dn5)) / (locals.var_uc_depthn * locals.var_uc_depthn)), (((locals.var_t1_dn6 * locals.var_uc_depthn) - (locals.var_t1 * locals.var_uc_depthn_dn6)) / (locals.var_uc_depthn * locals.var_uc_depthn)), (((locals.var_t1_dn7 * locals.var_uc_depthn) - (locals.var_t1 * locals.var_uc_depthn_dn7)) / (locals.var_uc_depthn * locals.var_uc_depthn)), (((locals.var_t1_dn8 * locals.var_uc_depthn) - (locals.var_t1 * locals.var_uc_depthn_dn8)) / (locals.var_uc_depthn * locals.var_uc_depthn)), (((locals.var_t1_dn9 * locals.var_uc_depthn) - (locals.var_t1 * locals.var_uc_depthn_dn9)) / (locals.var_uc_depthn * locals.var_uc_depthn)), (((locals.var_t1_dn10 * locals.var_uc_depthn) - (locals.var_t1 * locals.var_uc_depthn_dn10)) / (locals.var_uc_depthn * locals.var_uc_depthn)), (((locals.var_t1_dn11 * locals.var_uc_depthn) - (locals.var_t1 * locals.var_uc_depthn_dn11)) / (locals.var_uc_depthn * locals.var_uc_depthn)), (((locals.var_t1_dn14 * locals.var_uc_depthn) - (locals.var_t1 * locals.var_uc_depthn_dn14)) / (locals.var_uc_depthn * locals.var_uc_depthn)),)
    } else {
        (locals.var_eeff, locals.var_eeff_dn0, locals.var_eeff_dn2, locals.var_eeff_dn4, locals.var_eeff_dn5, locals.var_eeff_dn6, locals.var_eeff_dn7, locals.var_eeff_dn8, locals.var_eeff_dn9, locals.var_eeff_dn10, locals.var_eeff_dn11, locals.var_eeff_dn14,)
    }
};
        locals.var_eeff = assign43890_e59080;
        locals.var_eeff_dn0 = assign43890_e59080_d_n0;
        locals.var_eeff_dn2 = assign43890_e59080_d_n2;
        locals.var_eeff_dn4 = assign43890_e59080_d_n4;
        locals.var_eeff_dn5 = assign43890_e59080_d_n5;
        locals.var_eeff_dn6 = assign43890_e59080_d_n6;
        locals.var_eeff_dn7 = assign43890_e59080_d_n7;
        locals.var_eeff_dn8 = assign43890_e59080_d_n8;
        locals.var_eeff_dn9 = assign43890_e59080_d_n9;
        locals.var_eeff_dn10 = assign43890_e59080_d_n10;
        locals.var_eeff_dn11 = assign43890_e59080_d_n11;
        locals.var_eeff_dn14 = assign43890_e59080_d_n14;
        locals.var_eeff_rv = 0.0;

    }
}
