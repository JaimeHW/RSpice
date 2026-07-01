#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_38(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign17210_e11729, assign17210_e11729_d_n0, assign17210_e11729_d_n2, assign17210_e11729_d_n4, assign17210_e11729_d_n5, assign17210_e11729_d_n6, assign17210_e11729_d_n7, assign17210_e11729_d_n8, assign17210_e11729_d_n9, assign17210_e11729_d_n10, assign17210_e11729_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17210_e11727: f64 = (locals.var_tratio).ln();
        (assign17210_e11727, (locals.var_tratio_dn0 / locals.var_tratio), (locals.var_tratio_dn2 / locals.var_tratio), (locals.var_tratio_dn4 / locals.var_tratio), (locals.var_tratio_dn5 / locals.var_tratio), (locals.var_tratio_dn6 / locals.var_tratio), (locals.var_tratio_dn7 / locals.var_tratio), (locals.var_tratio_dn8 / locals.var_tratio), (locals.var_tratio_dn9 / locals.var_tratio), (locals.var_tratio_dn10 / locals.var_tratio), (locals.var_tratio_dn13 / locals.var_tratio),)
    } else {
        (locals.var_log_tratio, locals.var_log_tratio_dn0, locals.var_log_tratio_dn2, locals.var_log_tratio_dn4, locals.var_log_tratio_dn5, locals.var_log_tratio_dn6, locals.var_log_tratio_dn7, locals.var_log_tratio_dn8, locals.var_log_tratio_dn9, locals.var_log_tratio_dn10, locals.var_log_tratio_dn13,)
    }
};
        locals.var_log_tratio = assign17210_e11729;
        locals.var_log_tratio_dn0 = assign17210_e11729_d_n0;
        locals.var_log_tratio_dn2 = assign17210_e11729_d_n2;
        locals.var_log_tratio_dn4 = assign17210_e11729_d_n4;
        locals.var_log_tratio_dn5 = assign17210_e11729_d_n5;
        locals.var_log_tratio_dn6 = assign17210_e11729_d_n6;
        locals.var_log_tratio_dn7 = assign17210_e11729_d_n7;
        locals.var_log_tratio_dn8 = assign17210_e11729_d_n8;
        locals.var_log_tratio_dn9 = assign17210_e11729_d_n9;
        locals.var_log_tratio_dn10 = assign17210_e11729_d_n10;
        locals.var_log_tratio_dn13 = assign17210_e11729_d_n13;
        locals.var_log_tratio_rv = 0.0;

        let (assign17220_e11741, assign17220_e11741_d_n0, assign17220_e11741_d_n2, assign17220_e11741_d_n4, assign17220_e11741_d_n5, assign17220_e11741_d_n6, assign17220_e11741_d_n7, assign17220_e11741_d_n8, assign17220_e11741_d_n9, assign17220_e11741_d_n10, assign17220_e11741_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17220_e11734: f64 = (locals.var_uc_bgtmp1 * locals.var_tdiff);
        let assign17220_e11735: f64 = (locals.var_egtnom - assign17220_e11734);
        let assign17220_e11738: f64 = (locals.var_uc_bgtmp2 * locals.var_tdiff_2);
        let assign17220_e11739: f64 = (assign17220_e11735 - assign17220_e11738);
        (assign17220_e11739, ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn0)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn0)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn2)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn2)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn4)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn4)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn5)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn5)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn6)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn6)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn7)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn7)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn8)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn8)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn9)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn9)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn10)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn10)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn13)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn13)),)
    } else {
        (locals.var_eg, locals.var_eg_dn0, locals.var_eg_dn2, locals.var_eg_dn4, locals.var_eg_dn5, locals.var_eg_dn6, locals.var_eg_dn7, locals.var_eg_dn8, locals.var_eg_dn9, locals.var_eg_dn10, locals.var_eg_dn13,)
    }
};
        locals.var_eg = assign17220_e11741;
        locals.var_eg_dn0 = assign17220_e11741_d_n0;
        locals.var_eg_dn2 = assign17220_e11741_d_n2;
        locals.var_eg_dn4 = assign17220_e11741_d_n4;
        locals.var_eg_dn5 = assign17220_e11741_d_n5;
        locals.var_eg_dn6 = assign17220_e11741_d_n6;
        locals.var_eg_dn7 = assign17220_e11741_d_n7;
        locals.var_eg_dn8 = assign17220_e11741_d_n8;
        locals.var_eg_dn9 = assign17220_e11741_d_n9;
        locals.var_eg_dn10 = assign17220_e11741_d_n10;
        locals.var_eg_dn13 = assign17220_e11741_d_n13;
        locals.var_eg_rv = 0.0;

        let (assign17230_e11746, assign17230_e11746_d_n0, assign17230_e11746_d_n2, assign17230_e11746_d_n4, assign17230_e11746_d_n5, assign17230_e11746_d_n6, assign17230_e11746_d_n7, assign17230_e11746_d_n8, assign17230_e11746_d_n9, assign17230_e11746_d_n10, assign17230_e11746_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17230_e11744: f64 = (locals.var_eg).sqrt();
        (assign17230_e11744, (locals.var_eg_dn0 / (2.0 * assign17230_e11744)), (locals.var_eg_dn2 / (2.0 * assign17230_e11744)), (locals.var_eg_dn4 / (2.0 * assign17230_e11744)), (locals.var_eg_dn5 / (2.0 * assign17230_e11744)), (locals.var_eg_dn6 / (2.0 * assign17230_e11744)), (locals.var_eg_dn7 / (2.0 * assign17230_e11744)), (locals.var_eg_dn8 / (2.0 * assign17230_e11744)), (locals.var_eg_dn9 / (2.0 * assign17230_e11744)), (locals.var_eg_dn10 / (2.0 * assign17230_e11744)), (locals.var_eg_dn13 / (2.0 * assign17230_e11744)),)
    } else {
        (locals.var_sqrt_eg, locals.var_sqrt_eg_dn0, locals.var_sqrt_eg_dn2, locals.var_sqrt_eg_dn4, locals.var_sqrt_eg_dn5, locals.var_sqrt_eg_dn6, locals.var_sqrt_eg_dn7, locals.var_sqrt_eg_dn8, locals.var_sqrt_eg_dn9, locals.var_sqrt_eg_dn10, locals.var_sqrt_eg_dn13,)
    }
};
        locals.var_sqrt_eg = assign17230_e11746;
        locals.var_sqrt_eg_dn0 = assign17230_e11746_d_n0;
        locals.var_sqrt_eg_dn2 = assign17230_e11746_d_n2;
        locals.var_sqrt_eg_dn4 = assign17230_e11746_d_n4;
        locals.var_sqrt_eg_dn5 = assign17230_e11746_d_n5;
        locals.var_sqrt_eg_dn6 = assign17230_e11746_d_n6;
        locals.var_sqrt_eg_dn7 = assign17230_e11746_d_n7;
        locals.var_sqrt_eg_dn8 = assign17230_e11746_d_n8;
        locals.var_sqrt_eg_dn9 = assign17230_e11746_d_n9;
        locals.var_sqrt_eg_dn10 = assign17230_e11746_d_n10;
        locals.var_sqrt_eg_dn13 = assign17230_e11746_d_n13;
        locals.var_sqrt_eg_rv = 0.0;

        let (assign17240_e11752, assign17240_e11752_d_n0, assign17240_e11752_d_n2, assign17240_e11752_d_n4, assign17240_e11752_d_n5, assign17240_e11752_d_n6, assign17240_e11752_d_n7, assign17240_e11752_d_n8, assign17240_e11752_d_n9, assign17240_e11752_d_n10, assign17240_e11752_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17240_e11750: f64 = (1.0 / locals.var_ttemp);
        (assign17240_e11750, (-(locals.var_ttemp_dn0 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn2 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn4 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn5 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn6 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn7 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn8 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn9 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn10 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn13 / (locals.var_ttemp * locals.var_ttemp))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign17240_e11752;
        locals.var_t1_dn0 = assign17240_e11752_d_n0;
        locals.var_t1_dn2 = assign17240_e11752_d_n2;
        locals.var_t1_dn4 = assign17240_e11752_d_n4;
        locals.var_t1_dn5 = assign17240_e11752_d_n5;
        locals.var_t1_dn6 = assign17240_e11752_d_n6;
        locals.var_t1_dn7 = assign17240_e11752_d_n7;
        locals.var_t1_dn8 = assign17240_e11752_d_n8;
        locals.var_t1_dn9 = assign17240_e11752_d_n9;
        locals.var_t1_dn10 = assign17240_e11752_d_n10;
        locals.var_t1_dn13 = assign17240_e11752_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign17250_e11758, assign17250_e11758_d_n0, assign17250_e11758_d_n2, assign17250_e11758_d_n4, assign17250_e11758_d_n5, assign17250_e11758_d_n6, assign17250_e11758_d_n7, assign17250_e11758_d_n8, assign17250_e11758_d_n9, assign17250_e11758_d_n10, assign17250_e11758_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17250_e11756: f64 = (1.0 / locals.var_ktnom);
        (assign17250_e11756, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign17250_e11758;
        locals.var_t2_dn0 = assign17250_e11758_d_n0;
        locals.var_t2_dn2 = assign17250_e11758_d_n2;
        locals.var_t2_dn4 = assign17250_e11758_d_n4;
        locals.var_t2_dn5 = assign17250_e11758_d_n5;
        locals.var_t2_dn6 = assign17250_e11758_d_n6;
        locals.var_t2_dn7 = assign17250_e11758_d_n7;
        locals.var_t2_dn8 = assign17250_e11758_d_n8;
        locals.var_t2_dn9 = assign17250_e11758_d_n9;
        locals.var_t2_dn10 = assign17250_e11758_d_n10;
        locals.var_t2_dn13 = assign17250_e11758_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign17260_e11780, assign17260_e11780_d_n0, assign17260_e11780_d_n2, assign17260_e11780_d_n4, assign17260_e11780_d_n5, assign17260_e11780_d_n6, assign17260_e11780_d_n7, assign17260_e11780_d_n8, assign17260_e11780_d_n9, assign17260_e11780_d_n10, assign17260_e11780_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17260_e11762: f64 = (locals.var_egtnom + p.p259);
        let assign17260_e11766: f64 = (locals.var_t1 - locals.var_t2);
        let assign17260_e11767: f64 = (p.p260 * assign17260_e11766);
        let assign17260_e11768: f64 = (assign17260_e11762 + assign17260_e11767);
        let assign17260_e11772: f64 = (locals.var_t1 * locals.var_t1);
        let assign17260_e11775: f64 = (locals.var_t2 * locals.var_t2);
        let assign17260_e11776: f64 = (assign17260_e11772 - assign17260_e11775);
        let assign17260_e11777: f64 = (p.p261 * assign17260_e11776);
        let assign17260_e11778: f64 = (assign17260_e11768 + assign17260_e11777);
        (assign17260_e11778, ((p.p260 * (locals.var_t1_dn0 - locals.var_t2_dn0)) + (p.p261 * (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) - ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0))))), ((p.p260 * (locals.var_t1_dn2 - locals.var_t2_dn2)) + (p.p261 * (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) - ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2))))), ((p.p260 * (locals.var_t1_dn4 - locals.var_t2_dn4)) + (p.p261 * (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) - ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4))))), ((p.p260 * (locals.var_t1_dn5 - locals.var_t2_dn5)) + (p.p261 * (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) - ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5))))), ((p.p260 * (locals.var_t1_dn6 - locals.var_t2_dn6)) + (p.p261 * (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) - ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6))))), ((p.p260 * (locals.var_t1_dn7 - locals.var_t2_dn7)) + (p.p261 * (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) - ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7))))), ((p.p260 * (locals.var_t1_dn8 - locals.var_t2_dn8)) + (p.p261 * (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) - ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8))))), ((p.p260 * (locals.var_t1_dn9 - locals.var_t2_dn9)) + (p.p261 * (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) - ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9))))), ((p.p260 * (locals.var_t1_dn10 - locals.var_t2_dn10)) + (p.p261 * (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) - ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10))))), ((p.p260 * (locals.var_t1_dn13 - locals.var_t2_dn13)) + (p.p261 * (((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) - ((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign17260_e11780;
        locals.var_t3_dn0 = assign17260_e11780_d_n0;
        locals.var_t3_dn2 = assign17260_e11780_d_n2;
        locals.var_t3_dn4 = assign17260_e11780_d_n4;
        locals.var_t3_dn5 = assign17260_e11780_d_n5;
        locals.var_t3_dn6 = assign17260_e11780_d_n6;
        locals.var_t3_dn7 = assign17260_e11780_d_n7;
        locals.var_t3_dn8 = assign17260_e11780_d_n8;
        locals.var_t3_dn9 = assign17260_e11780_d_n9;
        locals.var_t3_dn10 = assign17260_e11780_d_n10;
        locals.var_t3_dn13 = assign17260_e11780_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign17270_e11785, assign17270_e11785_d_n0, assign17270_e11785_d_n2, assign17270_e11785_d_n4, assign17270_e11785_d_n5, assign17270_e11785_d_n6, assign17270_e11785_d_n7, assign17270_e11785_d_n8, assign17270_e11785_d_n9, assign17270_e11785_d_n10, assign17270_e11785_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17270_e11783: f64 = (locals.var_t3).sqrt();
        (assign17270_e11783, (locals.var_t3_dn0 / (2.0 * assign17270_e11783)), (locals.var_t3_dn2 / (2.0 * assign17270_e11783)), (locals.var_t3_dn4 / (2.0 * assign17270_e11783)), (locals.var_t3_dn5 / (2.0 * assign17270_e11783)), (locals.var_t3_dn6 / (2.0 * assign17270_e11783)), (locals.var_t3_dn7 / (2.0 * assign17270_e11783)), (locals.var_t3_dn8 / (2.0 * assign17270_e11783)), (locals.var_t3_dn9 / (2.0 * assign17270_e11783)), (locals.var_t3_dn10 / (2.0 * assign17270_e11783)), (locals.var_t3_dn13 / (2.0 * assign17270_e11783)),)
    } else {
        (locals.var_egp12, locals.var_egp12_dn0, locals.var_egp12_dn2, locals.var_egp12_dn4, locals.var_egp12_dn5, locals.var_egp12_dn6, locals.var_egp12_dn7, locals.var_egp12_dn8, locals.var_egp12_dn9, locals.var_egp12_dn10, locals.var_egp12_dn13,)
    }
};
        locals.var_egp12 = assign17270_e11785;
        locals.var_egp12_dn0 = assign17270_e11785_d_n0;
        locals.var_egp12_dn2 = assign17270_e11785_d_n2;
        locals.var_egp12_dn4 = assign17270_e11785_d_n4;
        locals.var_egp12_dn5 = assign17270_e11785_d_n5;
        locals.var_egp12_dn6 = assign17270_e11785_d_n6;
        locals.var_egp12_dn7 = assign17270_e11785_d_n7;
        locals.var_egp12_dn8 = assign17270_e11785_d_n8;
        locals.var_egp12_dn9 = assign17270_e11785_d_n9;
        locals.var_egp12_dn10 = assign17270_e11785_d_n10;
        locals.var_egp12_dn13 = assign17270_e11785_d_n13;
        locals.var_egp12_rv = 0.0;

        let (assign17280_e11791, assign17280_e11791_d_n0, assign17280_e11791_d_n2, assign17280_e11791_d_n4, assign17280_e11791_d_n5, assign17280_e11791_d_n6, assign17280_e11791_d_n7, assign17280_e11791_d_n8, assign17280_e11791_d_n9, assign17280_e11791_d_n10, assign17280_e11791_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17280_e11789: f64 = (locals.var_t3 * locals.var_egp12);
        (assign17280_e11789, ((locals.var_t3_dn0 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn0)), ((locals.var_t3_dn2 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn2)), ((locals.var_t3_dn4 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn4)), ((locals.var_t3_dn5 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn5)), ((locals.var_t3_dn6 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn6)), ((locals.var_t3_dn7 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn7)), ((locals.var_t3_dn8 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn8)), ((locals.var_t3_dn9 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn9)), ((locals.var_t3_dn10 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn10)), ((locals.var_t3_dn13 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn13)),)
    } else {
        (locals.var_egp32, locals.var_egp32_dn0, locals.var_egp32_dn2, locals.var_egp32_dn4, locals.var_egp32_dn5, locals.var_egp32_dn6, locals.var_egp32_dn7, locals.var_egp32_dn8, locals.var_egp32_dn9, locals.var_egp32_dn10, locals.var_egp32_dn13,)
    }
};
        locals.var_egp32 = assign17280_e11791;
        locals.var_egp32_dn0 = assign17280_e11791_d_n0;
        locals.var_egp32_dn2 = assign17280_e11791_d_n2;
        locals.var_egp32_dn4 = assign17280_e11791_d_n4;
        locals.var_egp32_dn5 = assign17280_e11791_d_n5;
        locals.var_egp32_dn6 = assign17280_e11791_d_n6;
        locals.var_egp32_dn7 = assign17280_e11791_d_n7;
        locals.var_egp32_dn8 = assign17280_e11791_d_n8;
        locals.var_egp32_dn9 = assign17280_e11791_d_n9;
        locals.var_egp32_dn10 = assign17280_e11791_d_n10;
        locals.var_egp32_dn13 = assign17280_e11791_d_n13;
        locals.var_egp32_rv = 0.0;

        let (assign17290_e11799, assign17290_e11799_d_n0, assign17290_e11799_d_n2, assign17290_e11799_d_n4, assign17290_e11799_d_n5, assign17290_e11799_d_n6, assign17290_e11799_d_n7, assign17290_e11799_d_n8, assign17290_e11799_d_n9, assign17290_e11799_d_n10, assign17290_e11799_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17290_e11796: f64 = (1.3806226e-23 * locals.var_ttemp);
        let assign17290_e11797: f64 = (1.6021918e-19 / assign17290_e11796);
        (assign17290_e11797, (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn0)) / (assign17290_e11796 * assign17290_e11796))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn2)) / (assign17290_e11796 * assign17290_e11796))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn4)) / (assign17290_e11796 * assign17290_e11796))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn5)) / (assign17290_e11796 * assign17290_e11796))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn6)) / (assign17290_e11796 * assign17290_e11796))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn7)) / (assign17290_e11796 * assign17290_e11796))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn8)) / (assign17290_e11796 * assign17290_e11796))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn9)) / (assign17290_e11796 * assign17290_e11796))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn10)) / (assign17290_e11796 * assign17290_e11796))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn13)) / (assign17290_e11796 * assign17290_e11796))),)
    } else {
        (locals.var_beta, locals.var_beta_dn0, locals.var_beta_dn2, locals.var_beta_dn4, locals.var_beta_dn5, locals.var_beta_dn6, locals.var_beta_dn7, locals.var_beta_dn8, locals.var_beta_dn9, locals.var_beta_dn10, locals.var_beta_dn13,)
    }
};
        locals.var_beta = assign17290_e11799;
        locals.var_beta_dn0 = assign17290_e11799_d_n0;
        locals.var_beta_dn2 = assign17290_e11799_d_n2;
        locals.var_beta_dn4 = assign17290_e11799_d_n4;
        locals.var_beta_dn5 = assign17290_e11799_d_n5;
        locals.var_beta_dn6 = assign17290_e11799_d_n6;
        locals.var_beta_dn7 = assign17290_e11799_d_n7;
        locals.var_beta_dn8 = assign17290_e11799_d_n8;
        locals.var_beta_dn9 = assign17290_e11799_d_n9;
        locals.var_beta_dn10 = assign17290_e11799_d_n10;
        locals.var_beta_dn13 = assign17290_e11799_d_n13;
        locals.var_beta_rv = 0.0;

        let (assign17300_e11805, assign17300_e11805_d_n0, assign17300_e11805_d_n2, assign17300_e11805_d_n4, assign17300_e11805_d_n5, assign17300_e11805_d_n6, assign17300_e11805_d_n7, assign17300_e11805_d_n8, assign17300_e11805_d_n9, assign17300_e11805_d_n10, assign17300_e11805_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17300_e11803: f64 = (1.0 / locals.var_beta);
        (assign17300_e11803, (-(locals.var_beta_dn0 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn2 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn4 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn5 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn6 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn7 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn8 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn9 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn10 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn13 / (locals.var_beta * locals.var_beta))),)
    } else {
        (locals.var_beta_inv, locals.var_beta_inv_dn0, locals.var_beta_inv_dn2, locals.var_beta_inv_dn4, locals.var_beta_inv_dn5, locals.var_beta_inv_dn6, locals.var_beta_inv_dn7, locals.var_beta_inv_dn8, locals.var_beta_inv_dn9, locals.var_beta_inv_dn10, locals.var_beta_inv_dn13,)
    }
};
        locals.var_beta_inv = assign17300_e11805;
        locals.var_beta_inv_dn0 = assign17300_e11805_d_n0;
        locals.var_beta_inv_dn2 = assign17300_e11805_d_n2;
        locals.var_beta_inv_dn4 = assign17300_e11805_d_n4;
        locals.var_beta_inv_dn5 = assign17300_e11805_d_n5;
        locals.var_beta_inv_dn6 = assign17300_e11805_d_n6;
        locals.var_beta_inv_dn7 = assign17300_e11805_d_n7;
        locals.var_beta_inv_dn8 = assign17300_e11805_d_n8;
        locals.var_beta_inv_dn9 = assign17300_e11805_d_n9;
        locals.var_beta_inv_dn10 = assign17300_e11805_d_n10;
        locals.var_beta_inv_dn13 = assign17300_e11805_d_n13;
        locals.var_beta_inv_rv = 0.0;

        let (assign17310_e11811, assign17310_e11811_d_n0, assign17310_e11811_d_n2, assign17310_e11811_d_n4, assign17310_e11811_d_n5, assign17310_e11811_d_n6, assign17310_e11811_d_n7, assign17310_e11811_d_n8, assign17310_e11811_d_n9, assign17310_e11811_d_n10, assign17310_e11811_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17310_e11809: f64 = (locals.var_beta * locals.var_beta);
        (assign17310_e11809, ((locals.var_beta_dn0 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn0)), ((locals.var_beta_dn2 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn2)), ((locals.var_beta_dn4 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn4)), ((locals.var_beta_dn5 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn5)), ((locals.var_beta_dn6 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn6)), ((locals.var_beta_dn7 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn7)), ((locals.var_beta_dn8 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn8)), ((locals.var_beta_dn9 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn9)), ((locals.var_beta_dn10 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn10)), ((locals.var_beta_dn13 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn13)),)
    } else {
        (locals.var_beta2, locals.var_beta2_dn0, locals.var_beta2_dn2, locals.var_beta2_dn4, locals.var_beta2_dn5, locals.var_beta2_dn6, locals.var_beta2_dn7, locals.var_beta2_dn8, locals.var_beta2_dn9, locals.var_beta2_dn10, locals.var_beta2_dn13,)
    }
};
        locals.var_beta2 = assign17310_e11811;
        locals.var_beta2_dn0 = assign17310_e11811_d_n0;
        locals.var_beta2_dn2 = assign17310_e11811_d_n2;
        locals.var_beta2_dn4 = assign17310_e11811_d_n4;
        locals.var_beta2_dn5 = assign17310_e11811_d_n5;
        locals.var_beta2_dn6 = assign17310_e11811_d_n6;
        locals.var_beta2_dn7 = assign17310_e11811_d_n7;
        locals.var_beta2_dn8 = assign17310_e11811_d_n8;
        locals.var_beta2_dn9 = assign17310_e11811_d_n9;
        locals.var_beta2_dn10 = assign17310_e11811_d_n10;
        locals.var_beta2_dn13 = assign17310_e11811_d_n13;
        locals.var_beta2_rv = 0.0;

        let (assign17320_e11819,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17320_e11816: f64 = (1.3806226e-23 * locals.var_ktnom);
        let assign17320_e11817: f64 = (1.6021918e-19 / assign17320_e11816);
        (assign17320_e11817,)
    } else {
        (locals.var_betatnom,)
    }
};
        locals.var_betatnom = assign17320_e11819;
        locals.var_betatnom_rv = 0.0;

        let (assign17330_e11842, assign17330_e11842_d_n0, assign17330_e11842_d_n2, assign17330_e11842_d_n4, assign17330_e11842_d_n5, assign17330_e11842_d_n6, assign17330_e11842_d_n7, assign17330_e11842_d_n8, assign17330_e11842_d_n9, assign17330_e11842_d_n10, assign17330_e11842_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17330_e11824: f64 = (locals.var_log_tratio * 1.5);
        let assign17330_e11825: f64 = (assign17330_e11824).exp();
        let assign17330_e11826: f64 = (1.04e16 * assign17330_e11825);
        let assign17330_e11828: f64 = (-locals.var_eg);
        let assign17330_e11830: f64 = (assign17330_e11828 / 2.0);
        let assign17330_e11832: f64 = (assign17330_e11830 * locals.var_beta);
        let assign17330_e11835: f64 = (locals.var_egtnom / 2.0);
        let assign17330_e11837: f64 = (assign17330_e11835 * locals.var_betatnom);
        let assign17330_e11838: f64 = (assign17330_e11832 + assign17330_e11837);
        let assign17330_e11839: f64 = (assign17330_e11838).exp();
        let assign17330_e11840: f64 = (assign17330_e11826 * assign17330_e11839);
        (assign17330_e11840, (((1.04e16 * (assign17330_e11825 * (locals.var_log_tratio_dn0 * 1.5))) * assign17330_e11839) + (assign17330_e11826 * (assign17330_e11839 * ((((-locals.var_eg_dn0) / 2.0) * locals.var_beta) + (assign17330_e11830 * locals.var_beta_dn0))))), (((1.04e16 * (assign17330_e11825 * (locals.var_log_tratio_dn2 * 1.5))) * assign17330_e11839) + (assign17330_e11826 * (assign17330_e11839 * ((((-locals.var_eg_dn2) / 2.0) * locals.var_beta) + (assign17330_e11830 * locals.var_beta_dn2))))), (((1.04e16 * (assign17330_e11825 * (locals.var_log_tratio_dn4 * 1.5))) * assign17330_e11839) + (assign17330_e11826 * (assign17330_e11839 * ((((-locals.var_eg_dn4) / 2.0) * locals.var_beta) + (assign17330_e11830 * locals.var_beta_dn4))))), (((1.04e16 * (assign17330_e11825 * (locals.var_log_tratio_dn5 * 1.5))) * assign17330_e11839) + (assign17330_e11826 * (assign17330_e11839 * ((((-locals.var_eg_dn5) / 2.0) * locals.var_beta) + (assign17330_e11830 * locals.var_beta_dn5))))), (((1.04e16 * (assign17330_e11825 * (locals.var_log_tratio_dn6 * 1.5))) * assign17330_e11839) + (assign17330_e11826 * (assign17330_e11839 * ((((-locals.var_eg_dn6) / 2.0) * locals.var_beta) + (assign17330_e11830 * locals.var_beta_dn6))))), (((1.04e16 * (assign17330_e11825 * (locals.var_log_tratio_dn7 * 1.5))) * assign17330_e11839) + (assign17330_e11826 * (assign17330_e11839 * ((((-locals.var_eg_dn7) / 2.0) * locals.var_beta) + (assign17330_e11830 * locals.var_beta_dn7))))), (((1.04e16 * (assign17330_e11825 * (locals.var_log_tratio_dn8 * 1.5))) * assign17330_e11839) + (assign17330_e11826 * (assign17330_e11839 * ((((-locals.var_eg_dn8) / 2.0) * locals.var_beta) + (assign17330_e11830 * locals.var_beta_dn8))))), (((1.04e16 * (assign17330_e11825 * (locals.var_log_tratio_dn9 * 1.5))) * assign17330_e11839) + (assign17330_e11826 * (assign17330_e11839 * ((((-locals.var_eg_dn9) / 2.0) * locals.var_beta) + (assign17330_e11830 * locals.var_beta_dn9))))), (((1.04e16 * (assign17330_e11825 * (locals.var_log_tratio_dn10 * 1.5))) * assign17330_e11839) + (assign17330_e11826 * (assign17330_e11839 * ((((-locals.var_eg_dn10) / 2.0) * locals.var_beta) + (assign17330_e11830 * locals.var_beta_dn10))))), (((1.04e16 * (assign17330_e11825 * (locals.var_log_tratio_dn13 * 1.5))) * assign17330_e11839) + (assign17330_e11826 * (assign17330_e11839 * ((((-locals.var_eg_dn13) / 2.0) * locals.var_beta) + (assign17330_e11830 * locals.var_beta_dn13))))),)
    } else {
        (locals.var_nin, locals.var_nin_dn0, locals.var_nin_dn2, locals.var_nin_dn4, locals.var_nin_dn5, locals.var_nin_dn6, locals.var_nin_dn7, locals.var_nin_dn8, locals.var_nin_dn9, locals.var_nin_dn10, locals.var_nin_dn13,)
    }
};
        locals.var_nin = assign17330_e11842;
        locals.var_nin_dn0 = assign17330_e11842_d_n0;
        locals.var_nin_dn2 = assign17330_e11842_d_n2;
        locals.var_nin_dn4 = assign17330_e11842_d_n4;
        locals.var_nin_dn5 = assign17330_e11842_d_n5;
        locals.var_nin_dn6 = assign17330_e11842_d_n6;
        locals.var_nin_dn7 = assign17330_e11842_d_n7;
        locals.var_nin_dn8 = assign17330_e11842_d_n8;
        locals.var_nin_dn9 = assign17330_e11842_d_n9;
        locals.var_nin_dn10 = assign17330_e11842_d_n10;
        locals.var_nin_dn13 = assign17330_e11842_d_n13;
        locals.var_nin_rv = 0.0;

        let (assign17340_e11849, assign17340_e11849_d_n0, assign17340_e11849_d_n2, assign17340_e11849_d_n4, assign17340_e11849_d_n5, assign17340_e11849_d_n6, assign17340_e11849_d_n7, assign17340_e11849_d_n8, assign17340_e11849_d_n9, assign17340_e11849_d_n10, assign17340_e11849_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17340_e11846: f64 = (locals.var_log_tratio * locals.var_uc_muetmp);
        let assign17340_e11847: f64 = (assign17340_e11846).exp();
        (assign17340_e11847, (assign17340_e11847 * (locals.var_log_tratio_dn0 * locals.var_uc_muetmp)), (assign17340_e11847 * (locals.var_log_tratio_dn2 * locals.var_uc_muetmp)), (assign17340_e11847 * (locals.var_log_tratio_dn4 * locals.var_uc_muetmp)), (assign17340_e11847 * (locals.var_log_tratio_dn5 * locals.var_uc_muetmp)), (assign17340_e11847 * (locals.var_log_tratio_dn6 * locals.var_uc_muetmp)), (assign17340_e11847 * (locals.var_log_tratio_dn7 * locals.var_uc_muetmp)), (assign17340_e11847 * (locals.var_log_tratio_dn8 * locals.var_uc_muetmp)), (assign17340_e11847 * (locals.var_log_tratio_dn9 * locals.var_uc_muetmp)), (assign17340_e11847 * (locals.var_log_tratio_dn10 * locals.var_uc_muetmp)), (assign17340_e11847 * (locals.var_log_tratio_dn13 * locals.var_uc_muetmp)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign17340_e11849;
        locals.var_t1_dn0 = assign17340_e11849_d_n0;
        locals.var_t1_dn2 = assign17340_e11849_d_n2;
        locals.var_t1_dn4 = assign17340_e11849_d_n4;
        locals.var_t1_dn5 = assign17340_e11849_d_n5;
        locals.var_t1_dn6 = assign17340_e11849_d_n6;
        locals.var_t1_dn7 = assign17340_e11849_d_n7;
        locals.var_t1_dn8 = assign17340_e11849_d_n8;
        locals.var_t1_dn9 = assign17340_e11849_d_n9;
        locals.var_t1_dn10 = assign17340_e11849_d_n10;
        locals.var_t1_dn13 = assign17340_e11849_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign17350_e11855, assign17350_e11855_d_n0, assign17350_e11855_d_n2, assign17350_e11855_d_n4, assign17350_e11855_d_n5, assign17350_e11855_d_n6, assign17350_e11855_d_n7, assign17350_e11855_d_n8, assign17350_e11855_d_n9, assign17350_e11855_d_n10, assign17350_e11855_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17350_e11853: f64 = (locals.var_t1 / locals.var_mueph);
        (assign17350_e11853, (((locals.var_t1_dn0 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn0)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn2 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn2)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn4 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn4)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn5 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn5)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn6 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn6)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn7 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn7)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn8 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn8)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn9 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn9)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn10 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn10)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn13 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn13)) / (locals.var_mueph * locals.var_mueph)),)
    } else {
        (locals.var_mphn0, locals.var_mphn0_dn0, locals.var_mphn0_dn2, locals.var_mphn0_dn4, locals.var_mphn0_dn5, locals.var_mphn0_dn6, locals.var_mphn0_dn7, locals.var_mphn0_dn8, locals.var_mphn0_dn9, locals.var_mphn0_dn10, locals.var_mphn0_dn13,)
    }
};
        locals.var_mphn0 = assign17350_e11855;
        locals.var_mphn0_dn0 = assign17350_e11855_d_n0;
        locals.var_mphn0_dn2 = assign17350_e11855_d_n2;
        locals.var_mphn0_dn4 = assign17350_e11855_d_n4;
        locals.var_mphn0_dn5 = assign17350_e11855_d_n5;
        locals.var_mphn0_dn6 = assign17350_e11855_d_n6;
        locals.var_mphn0_dn7 = assign17350_e11855_d_n7;
        locals.var_mphn0_dn8 = assign17350_e11855_d_n8;
        locals.var_mphn0_dn9 = assign17350_e11855_d_n9;
        locals.var_mphn0_dn10 = assign17350_e11855_d_n10;
        locals.var_mphn0_dn13 = assign17350_e11855_d_n13;
        locals.var_mphn0_rv = 0.0;

        let assign17360_e11862: f64 = if ((locals.var_uc_codep != 0.0) && (locals.var_uc_codep < 3.0)) { 1.0 } else { 0.0 };
        locals.var_guard355 = assign17360_e11862;
        locals.var_guard355_rv = 0.0;

        let (assign17370_e11877, assign17370_e11877_d_n0, assign17370_e11877_d_n2, assign17370_e11877_d_n4, assign17370_e11877_d_n5, assign17370_e11877_d_n6, assign17370_e11877_d_n7, assign17370_e11877_d_n8, assign17370_e11877_d_n9, assign17370_e11877_d_n10, assign17370_e11877_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard355 != 0.0)) {
        let assign17370_e11868: f64 = (2.0 * 1.034943e-10);
        let assign17370_e11870: f64 = (assign17370_e11868 * 1.6021918e-19);
        let assign17370_e11872: f64 = (assign17370_e11870 * locals.var_uc_ndepm);
        let assign17370_e11874: f64 = (assign17370_e11872 * locals.var_beta_inv);
        let assign17370_e11875: f64 = (assign17370_e11874).sqrt();
        (assign17370_e11875, ((((assign17370_e11870 * locals.var_uc_ndepm_dn0) * locals.var_beta_inv) + (assign17370_e11872 * locals.var_beta_inv_dn0)) / (2.0 * assign17370_e11875)), ((((assign17370_e11870 * locals.var_uc_ndepm_dn2) * locals.var_beta_inv) + (assign17370_e11872 * locals.var_beta_inv_dn2)) / (2.0 * assign17370_e11875)), ((((assign17370_e11870 * locals.var_uc_ndepm_dn4) * locals.var_beta_inv) + (assign17370_e11872 * locals.var_beta_inv_dn4)) / (2.0 * assign17370_e11875)), ((((assign17370_e11870 * locals.var_uc_ndepm_dn5) * locals.var_beta_inv) + (assign17370_e11872 * locals.var_beta_inv_dn5)) / (2.0 * assign17370_e11875)), ((((assign17370_e11870 * locals.var_uc_ndepm_dn6) * locals.var_beta_inv) + (assign17370_e11872 * locals.var_beta_inv_dn6)) / (2.0 * assign17370_e11875)), ((((assign17370_e11870 * locals.var_uc_ndepm_dn7) * locals.var_beta_inv) + (assign17370_e11872 * locals.var_beta_inv_dn7)) / (2.0 * assign17370_e11875)), ((((assign17370_e11870 * locals.var_uc_ndepm_dn8) * locals.var_beta_inv) + (assign17370_e11872 * locals.var_beta_inv_dn8)) / (2.0 * assign17370_e11875)), ((((assign17370_e11870 * locals.var_uc_ndepm_dn9) * locals.var_beta_inv) + (assign17370_e11872 * locals.var_beta_inv_dn9)) / (2.0 * assign17370_e11875)), ((((assign17370_e11870 * locals.var_uc_ndepm_dn10) * locals.var_beta_inv) + (assign17370_e11872 * locals.var_beta_inv_dn10)) / (2.0 * assign17370_e11875)), ((((assign17370_e11870 * locals.var_uc_ndepm_dn13) * locals.var_beta_inv) + (assign17370_e11872 * locals.var_beta_inv_dn13)) / (2.0 * assign17370_e11875)),)
    } else {
        (locals.var_cnst0, locals.var_cnst0_dn0, locals.var_cnst0_dn2, locals.var_cnst0_dn4, locals.var_cnst0_dn5, locals.var_cnst0_dn6, locals.var_cnst0_dn7, locals.var_cnst0_dn8, locals.var_cnst0_dn9, locals.var_cnst0_dn10, locals.var_cnst0_dn13,)
    }
};
        locals.var_cnst0 = assign17370_e11877;
        locals.var_cnst0_dn0 = assign17370_e11877_d_n0;
        locals.var_cnst0_dn2 = assign17370_e11877_d_n2;
        locals.var_cnst0_dn4 = assign17370_e11877_d_n4;
        locals.var_cnst0_dn5 = assign17370_e11877_d_n5;
        locals.var_cnst0_dn6 = assign17370_e11877_d_n6;
        locals.var_cnst0_dn7 = assign17370_e11877_d_n7;
        locals.var_cnst0_dn8 = assign17370_e11877_d_n8;
        locals.var_cnst0_dn9 = assign17370_e11877_d_n9;
        locals.var_cnst0_dn10 = assign17370_e11877_d_n10;
        locals.var_cnst0_dn13 = assign17370_e11877_d_n13;
        locals.var_cnst0_rv = 0.0;

        let (assign17380_e11889, assign17380_e11889_d_n0, assign17380_e11889_d_n2, assign17380_e11889_d_n4, assign17380_e11889_d_n5, assign17380_e11889_d_n6, assign17380_e11889_d_n7, assign17380_e11889_d_n8, assign17380_e11889_d_n9, assign17380_e11889_d_n10, assign17380_e11889_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard355 != 0.0)) {
        let assign17380_e11883: f64 = (locals.var_nin * locals.var_nin);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_uc_ndepm;
        let assign17380_e11885: f64 = (assign17380_e11883 * __rspice_inv_cse_0);
        let assign17380_e11887: f64 = (assign17380_e11885 * __rspice_inv_cse_0);
        (assign17380_e11887, ((((((((locals.var_nin_dn0 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn0)) * locals.var_uc_ndepm) - (assign17380_e11883 * locals.var_uc_ndepm_dn0)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17380_e11885 * locals.var_uc_ndepm_dn0)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn2 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn2)) * locals.var_uc_ndepm) - (assign17380_e11883 * locals.var_uc_ndepm_dn2)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17380_e11885 * locals.var_uc_ndepm_dn2)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn4 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn4)) * locals.var_uc_ndepm) - (assign17380_e11883 * locals.var_uc_ndepm_dn4)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17380_e11885 * locals.var_uc_ndepm_dn4)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn5 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn5)) * locals.var_uc_ndepm) - (assign17380_e11883 * locals.var_uc_ndepm_dn5)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17380_e11885 * locals.var_uc_ndepm_dn5)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn6 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn6)) * locals.var_uc_ndepm) - (assign17380_e11883 * locals.var_uc_ndepm_dn6)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17380_e11885 * locals.var_uc_ndepm_dn6)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn7 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn7)) * locals.var_uc_ndepm) - (assign17380_e11883 * locals.var_uc_ndepm_dn7)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17380_e11885 * locals.var_uc_ndepm_dn7)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn8 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn8)) * locals.var_uc_ndepm) - (assign17380_e11883 * locals.var_uc_ndepm_dn8)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17380_e11885 * locals.var_uc_ndepm_dn8)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn9 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn9)) * locals.var_uc_ndepm) - (assign17380_e11883 * locals.var_uc_ndepm_dn9)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17380_e11885 * locals.var_uc_ndepm_dn9)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn10 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn10)) * locals.var_uc_ndepm) - (assign17380_e11883 * locals.var_uc_ndepm_dn10)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17380_e11885 * locals.var_uc_ndepm_dn10)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn13 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn13)) * locals.var_uc_ndepm) - (assign17380_e11883 * locals.var_uc_ndepm_dn13)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17380_e11885 * locals.var_uc_ndepm_dn13)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)),)
    } else {
        (locals.var_cnst1, locals.var_cnst1_dn0, locals.var_cnst1_dn2, locals.var_cnst1_dn4, locals.var_cnst1_dn5, locals.var_cnst1_dn6, locals.var_cnst1_dn7, locals.var_cnst1_dn8, locals.var_cnst1_dn9, locals.var_cnst1_dn10, locals.var_cnst1_dn13,)
    }
};
        locals.var_cnst1 = assign17380_e11889;
        locals.var_cnst1_dn0 = assign17380_e11889_d_n0;
        locals.var_cnst1_dn2 = assign17380_e11889_d_n2;
        locals.var_cnst1_dn4 = assign17380_e11889_d_n4;
        locals.var_cnst1_dn5 = assign17380_e11889_d_n5;
        locals.var_cnst1_dn6 = assign17380_e11889_d_n6;
        locals.var_cnst1_dn7 = assign17380_e11889_d_n7;
        locals.var_cnst1_dn8 = assign17380_e11889_d_n8;
        locals.var_cnst1_dn9 = assign17380_e11889_d_n9;
        locals.var_cnst1_dn10 = assign17380_e11889_d_n10;
        locals.var_cnst1_dn13 = assign17380_e11889_d_n13;
        locals.var_cnst1_rv = 0.0;

        let (assign17390_e11902, assign17390_e11902_d_n0, assign17390_e11902_d_n2, assign17390_e11902_d_n4, assign17390_e11902_d_n5, assign17390_e11902_d_n6, assign17390_e11902_d_n7, assign17390_e11902_d_n8, assign17390_e11902_d_n9, assign17390_e11902_d_n10, assign17390_e11902_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard355 != 0.0)) {
        let assign17390_e11895: f64 = (2.0 * locals.var_beta_inv);
        let assign17390_e11898: f64 = (locals.var_uc_ndepm / locals.var_nin);
        let assign17390_e11899: f64 = (assign17390_e11898).ln();
        let assign17390_e11900: f64 = (assign17390_e11895 * assign17390_e11899);
        (assign17390_e11900, (((2.0 * locals.var_beta_inv_dn0) * assign17390_e11899) + (assign17390_e11895 * ((((locals.var_uc_ndepm_dn0 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign17390_e11898))), (((2.0 * locals.var_beta_inv_dn2) * assign17390_e11899) + (assign17390_e11895 * ((((locals.var_uc_ndepm_dn2 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign17390_e11898))), (((2.0 * locals.var_beta_inv_dn4) * assign17390_e11899) + (assign17390_e11895 * ((((locals.var_uc_ndepm_dn4 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign17390_e11898))), (((2.0 * locals.var_beta_inv_dn5) * assign17390_e11899) + (assign17390_e11895 * ((((locals.var_uc_ndepm_dn5 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign17390_e11898))), (((2.0 * locals.var_beta_inv_dn6) * assign17390_e11899) + (assign17390_e11895 * ((((locals.var_uc_ndepm_dn6 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign17390_e11898))), (((2.0 * locals.var_beta_inv_dn7) * assign17390_e11899) + (assign17390_e11895 * ((((locals.var_uc_ndepm_dn7 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign17390_e11898))), (((2.0 * locals.var_beta_inv_dn8) * assign17390_e11899) + (assign17390_e11895 * ((((locals.var_uc_ndepm_dn8 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign17390_e11898))), (((2.0 * locals.var_beta_inv_dn9) * assign17390_e11899) + (assign17390_e11895 * ((((locals.var_uc_ndepm_dn9 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign17390_e11898))), (((2.0 * locals.var_beta_inv_dn10) * assign17390_e11899) + (assign17390_e11895 * ((((locals.var_uc_ndepm_dn10 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign17390_e11898))), (((2.0 * locals.var_beta_inv_dn13) * assign17390_e11899) + (assign17390_e11895 * ((((locals.var_uc_ndepm_dn13 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn13)) / (locals.var_nin * locals.var_nin)) / assign17390_e11898))),)
    } else {
        (locals.var_pb2n, locals.var_pb2n_dn0, locals.var_pb2n_dn2, locals.var_pb2n_dn4, locals.var_pb2n_dn5, locals.var_pb2n_dn6, locals.var_pb2n_dn7, locals.var_pb2n_dn8, locals.var_pb2n_dn9, locals.var_pb2n_dn10, locals.var_pb2n_dn13,)
    }
};
        locals.var_pb2n = assign17390_e11902;
        locals.var_pb2n_dn0 = assign17390_e11902_d_n0;
        locals.var_pb2n_dn2 = assign17390_e11902_d_n2;
        locals.var_pb2n_dn4 = assign17390_e11902_d_n4;
        locals.var_pb2n_dn5 = assign17390_e11902_d_n5;
        locals.var_pb2n_dn6 = assign17390_e11902_d_n6;
        locals.var_pb2n_dn7 = assign17390_e11902_d_n7;
        locals.var_pb2n_dn8 = assign17390_e11902_d_n8;
        locals.var_pb2n_dn9 = assign17390_e11902_d_n9;
        locals.var_pb2n_dn10 = assign17390_e11902_d_n10;
        locals.var_pb2n_dn13 = assign17390_e11902_d_n13;
        locals.var_pb2n_rv = 0.0;

        let (assign17400_e11917, assign17400_e11917_d_n0, assign17400_e11917_d_n2, assign17400_e11917_d_n4, assign17400_e11917_d_n5, assign17400_e11917_d_n6, assign17400_e11917_d_n7, assign17400_e11917_d_n8, assign17400_e11917_d_n9, assign17400_e11917_d_n10, assign17400_e11917_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard355 != 0.0)) {
        let assign17400_e11909: f64 = (locals.var_uc_ndepm * locals.var_ef_nsubc);
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_nin;
        let assign17400_e11911: f64 = (assign17400_e11909 * __rspice_inv_cse_1);
        let assign17400_e11913: f64 = (assign17400_e11911 * __rspice_inv_cse_1);
        let assign17400_e11914: f64 = (assign17400_e11913).ln();
        let assign17400_e11915: f64 = (locals.var_beta_inv * assign17400_e11914);
        (assign17400_e11915, ((locals.var_beta_inv_dn0 * assign17400_e11914) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn0 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn0)) * locals.var_nin) - (assign17400_e11909 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17400_e11911 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign17400_e11913))), ((locals.var_beta_inv_dn2 * assign17400_e11914) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn2 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn2)) * locals.var_nin) - (assign17400_e11909 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17400_e11911 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign17400_e11913))), ((locals.var_beta_inv_dn4 * assign17400_e11914) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn4 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn4)) * locals.var_nin) - (assign17400_e11909 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17400_e11911 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign17400_e11913))), ((locals.var_beta_inv_dn5 * assign17400_e11914) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn5 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn5)) * locals.var_nin) - (assign17400_e11909 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17400_e11911 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign17400_e11913))), ((locals.var_beta_inv_dn6 * assign17400_e11914) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn6 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn6)) * locals.var_nin) - (assign17400_e11909 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17400_e11911 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign17400_e11913))), ((locals.var_beta_inv_dn7 * assign17400_e11914) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn7 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn7)) * locals.var_nin) - (assign17400_e11909 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17400_e11911 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign17400_e11913))), ((locals.var_beta_inv_dn8 * assign17400_e11914) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn8 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn8)) * locals.var_nin) - (assign17400_e11909 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17400_e11911 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign17400_e11913))), ((locals.var_beta_inv_dn9 * assign17400_e11914) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn9 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn9)) * locals.var_nin) - (assign17400_e11909 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17400_e11911 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign17400_e11913))), ((locals.var_beta_inv_dn10 * assign17400_e11914) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn10 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn10)) * locals.var_nin) - (assign17400_e11909 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17400_e11911 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign17400_e11913))), ((locals.var_beta_inv_dn13 * assign17400_e11914) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn13 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn13)) * locals.var_nin) - (assign17400_e11909 * locals.var_nin_dn13)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17400_e11911 * locals.var_nin_dn13)) / (locals.var_nin * locals.var_nin)) / assign17400_e11913))),)
    } else {
        (locals.var_vbipn, locals.var_vbipn_dn0, locals.var_vbipn_dn2, locals.var_vbipn_dn4, locals.var_vbipn_dn5, locals.var_vbipn_dn6, locals.var_vbipn_dn7, locals.var_vbipn_dn8, locals.var_vbipn_dn9, locals.var_vbipn_dn10, locals.var_vbipn_dn13,)
    }
};
        locals.var_vbipn = assign17400_e11917;
        locals.var_vbipn_dn0 = assign17400_e11917_d_n0;
        locals.var_vbipn_dn2 = assign17400_e11917_d_n2;
        locals.var_vbipn_dn4 = assign17400_e11917_d_n4;
        locals.var_vbipn_dn5 = assign17400_e11917_d_n5;
        locals.var_vbipn_dn6 = assign17400_e11917_d_n6;
        locals.var_vbipn_dn7 = assign17400_e11917_d_n7;
        locals.var_vbipn_dn8 = assign17400_e11917_d_n8;
        locals.var_vbipn_dn9 = assign17400_e11917_d_n9;
        locals.var_vbipn_dn10 = assign17400_e11917_d_n10;
        locals.var_vbipn_dn13 = assign17400_e11917_d_n13;
        locals.var_vbipn_rv = 0.0;

        let (assign17410_e11926, assign17410_e11926_d_n0, assign17410_e11926_d_n2, assign17410_e11926_d_n4, assign17410_e11926_d_n5, assign17410_e11926_d_n6, assign17410_e11926_d_n7, assign17410_e11926_d_n8, assign17410_e11926_d_n9, assign17410_e11926_d_n10, assign17410_e11926_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard355 != 0.0)) {
        let assign17410_e11923: f64 = (locals.var_log_tratio * p.p380);
        let assign17410_e11924: f64 = (assign17410_e11923).exp();
        (assign17410_e11924, (assign17410_e11924 * (locals.var_log_tratio_dn0 * p.p380)), (assign17410_e11924 * (locals.var_log_tratio_dn2 * p.p380)), (assign17410_e11924 * (locals.var_log_tratio_dn4 * p.p380)), (assign17410_e11924 * (locals.var_log_tratio_dn5 * p.p380)), (assign17410_e11924 * (locals.var_log_tratio_dn6 * p.p380)), (assign17410_e11924 * (locals.var_log_tratio_dn7 * p.p380)), (assign17410_e11924 * (locals.var_log_tratio_dn8 * p.p380)), (assign17410_e11924 * (locals.var_log_tratio_dn9 * p.p380)), (assign17410_e11924 * (locals.var_log_tratio_dn10 * p.p380)), (assign17410_e11924 * (locals.var_log_tratio_dn13 * p.p380)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign17410_e11926;
        locals.var_t1_dn0 = assign17410_e11926_d_n0;
        locals.var_t1_dn2 = assign17410_e11926_d_n2;
        locals.var_t1_dn4 = assign17410_e11926_d_n4;
        locals.var_t1_dn5 = assign17410_e11926_d_n5;
        locals.var_t1_dn6 = assign17410_e11926_d_n6;
        locals.var_t1_dn7 = assign17410_e11926_d_n7;
        locals.var_t1_dn8 = assign17410_e11926_d_n8;
        locals.var_t1_dn9 = assign17410_e11926_d_n9;
        locals.var_t1_dn10 = assign17410_e11926_d_n10;
        locals.var_t1_dn13 = assign17410_e11926_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign17420_e11934, assign17420_e11934_d_n0, assign17420_e11934_d_n2, assign17420_e11934_d_n4, assign17420_e11934_d_n5, assign17420_e11934_d_n6, assign17420_e11934_d_n7, assign17420_e11934_d_n8, assign17420_e11934_d_n9, assign17420_e11934_d_n10, assign17420_e11934_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard355 != 0.0)) {
        let assign17420_e11932: f64 = (locals.var_t1 / locals.var_uc_depmueph1);
        (assign17420_e11932, (locals.var_t1_dn0 / locals.var_uc_depmueph1), (locals.var_t1_dn2 / locals.var_uc_depmueph1), (locals.var_t1_dn4 / locals.var_uc_depmueph1), (locals.var_t1_dn5 / locals.var_uc_depmueph1), (locals.var_t1_dn6 / locals.var_uc_depmueph1), (locals.var_t1_dn7 / locals.var_uc_depmueph1), (locals.var_t1_dn8 / locals.var_uc_depmueph1), (locals.var_t1_dn9 / locals.var_uc_depmueph1), (locals.var_t1_dn10 / locals.var_uc_depmueph1), (locals.var_t1_dn13 / locals.var_uc_depmueph1),)
    } else {
        (locals.var_depmphn0, locals.var_depmphn0_dn0, locals.var_depmphn0_dn2, locals.var_depmphn0_dn4, locals.var_depmphn0_dn5, locals.var_depmphn0_dn6, locals.var_depmphn0_dn7, locals.var_depmphn0_dn8, locals.var_depmphn0_dn9, locals.var_depmphn0_dn10, locals.var_depmphn0_dn13,)
    }
};
        locals.var_depmphn0 = assign17420_e11934;
        locals.var_depmphn0_dn0 = assign17420_e11934_d_n0;
        locals.var_depmphn0_dn2 = assign17420_e11934_d_n2;
        locals.var_depmphn0_dn4 = assign17420_e11934_d_n4;
        locals.var_depmphn0_dn5 = assign17420_e11934_d_n5;
        locals.var_depmphn0_dn6 = assign17420_e11934_d_n6;
        locals.var_depmphn0_dn7 = assign17420_e11934_d_n7;
        locals.var_depmphn0_dn8 = assign17420_e11934_d_n8;
        locals.var_depmphn0_dn9 = assign17420_e11934_d_n9;
        locals.var_depmphn0_dn10 = assign17420_e11934_d_n10;
        locals.var_depmphn0_dn13 = assign17420_e11934_d_n13;
        locals.var_depmphn0_rv = 0.0;

        let (assign17430_e11956, assign17430_e11956_d_n0, assign17430_e11956_d_n2, assign17430_e11956_d_n4, assign17430_e11956_d_n5, assign17430_e11956_d_n6, assign17430_e11956_d_n7, assign17430_e11956_d_n8, assign17430_e11956_d_n9, assign17430_e11956_d_n10, assign17430_e11956_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard355 != 0.0)) {
        let assign17430_e11941: f64 = (0.4 * locals.var_tratio);
        let assign17430_e11942: f64 = (1.8 + assign17430_e11941);
        let assign17430_e11945: f64 = (0.1 * locals.var_tratio);
        let assign17430_e11947: f64 = (assign17430_e11945 * locals.var_tratio);
        let assign17430_e11948: f64 = (assign17430_e11942 + assign17430_e11947);
        let assign17430_e11952: f64 = (1.0 - locals.var_tratio);
        let assign17430_e11953: f64 = (p.p379 * assign17430_e11952);
        let assign17430_e11954: f64 = (assign17430_e11948 - assign17430_e11953);
        (assign17430_e11954, (((0.4 * locals.var_tratio_dn0) + (((0.1 * locals.var_tratio_dn0) * locals.var_tratio) + (assign17430_e11945 * locals.var_tratio_dn0))) - (p.p379 * (-locals.var_tratio_dn0))), (((0.4 * locals.var_tratio_dn2) + (((0.1 * locals.var_tratio_dn2) * locals.var_tratio) + (assign17430_e11945 * locals.var_tratio_dn2))) - (p.p379 * (-locals.var_tratio_dn2))), (((0.4 * locals.var_tratio_dn4) + (((0.1 * locals.var_tratio_dn4) * locals.var_tratio) + (assign17430_e11945 * locals.var_tratio_dn4))) - (p.p379 * (-locals.var_tratio_dn4))), (((0.4 * locals.var_tratio_dn5) + (((0.1 * locals.var_tratio_dn5) * locals.var_tratio) + (assign17430_e11945 * locals.var_tratio_dn5))) - (p.p379 * (-locals.var_tratio_dn5))), (((0.4 * locals.var_tratio_dn6) + (((0.1 * locals.var_tratio_dn6) * locals.var_tratio) + (assign17430_e11945 * locals.var_tratio_dn6))) - (p.p379 * (-locals.var_tratio_dn6))), (((0.4 * locals.var_tratio_dn7) + (((0.1 * locals.var_tratio_dn7) * locals.var_tratio) + (assign17430_e11945 * locals.var_tratio_dn7))) - (p.p379 * (-locals.var_tratio_dn7))), (((0.4 * locals.var_tratio_dn8) + (((0.1 * locals.var_tratio_dn8) * locals.var_tratio) + (assign17430_e11945 * locals.var_tratio_dn8))) - (p.p379 * (-locals.var_tratio_dn8))), (((0.4 * locals.var_tratio_dn9) + (((0.1 * locals.var_tratio_dn9) * locals.var_tratio) + (assign17430_e11945 * locals.var_tratio_dn9))) - (p.p379 * (-locals.var_tratio_dn9))), (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign17430_e11945 * locals.var_tratio_dn10))) - (p.p379 * (-locals.var_tratio_dn10))), (((0.4 * locals.var_tratio_dn13) + (((0.1 * locals.var_tratio_dn13) * locals.var_tratio) + (assign17430_e11945 * locals.var_tratio_dn13))) - (p.p379 * (-locals.var_tratio_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign17430_e11956;
        locals.var_t0_dn0 = assign17430_e11956_d_n0;
        locals.var_t0_dn2 = assign17430_e11956_d_n2;
        locals.var_t0_dn4 = assign17430_e11956_d_n4;
        locals.var_t0_dn5 = assign17430_e11956_d_n5;
        locals.var_t0_dn6 = assign17430_e11956_d_n6;
        locals.var_t0_dn7 = assign17430_e11956_d_n7;
        locals.var_t0_dn8 = assign17430_e11956_d_n8;
        locals.var_t0_dn9 = assign17430_e11956_d_n9;
        locals.var_t0_dn10 = assign17430_e11956_d_n10;
        locals.var_t0_dn13 = assign17430_e11956_d_n13;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_39(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign17440_e11964, assign17440_e11964_d_n0, assign17440_e11964_d_n2, assign17440_e11964_d_n4, assign17440_e11964_d_n5, assign17440_e11964_d_n6, assign17440_e11964_d_n7, assign17440_e11964_d_n8, assign17440_e11964_d_n9, assign17440_e11964_d_n10, assign17440_e11964_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard355 != 0.0)) {
        let assign17440_e11962: f64 = (locals.var_uc_depvmax / locals.var_t0);
        (assign17440_e11962, (((locals.var_uc_depvmax_dn0 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn0)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn2 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn2)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn4 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn5 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn6 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn7 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn8 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn9 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn10 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn13 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn13)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn13,)
    }
};
        locals.var_uc_depvmax = assign17440_e11964;
        locals.var_uc_depvmax_dn0 = assign17440_e11964_d_n0;
        locals.var_uc_depvmax_dn2 = assign17440_e11964_d_n2;
        locals.var_uc_depvmax_dn4 = assign17440_e11964_d_n4;
        locals.var_uc_depvmax_dn5 = assign17440_e11964_d_n5;
        locals.var_uc_depvmax_dn6 = assign17440_e11964_d_n6;
        locals.var_uc_depvmax_dn7 = assign17440_e11964_d_n7;
        locals.var_uc_depvmax_dn8 = assign17440_e11964_d_n8;
        locals.var_uc_depvmax_dn9 = assign17440_e11964_d_n9;
        locals.var_uc_depvmax_dn10 = assign17440_e11964_d_n10;
        locals.var_uc_depvmax_dn13 = assign17440_e11964_d_n13;
        locals.var_uc_depvmax_rv = 0.0;

        let assign17460_e11972: f64 = if locals.var_uc_depvmax < 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard357 = assign17460_e11972;
        locals.var_guard357_rv = 0.0;

        let (assign17470_e11980, assign17470_e11980_d_n0, assign17470_e11980_d_n2, assign17470_e11980_d_n4, assign17470_e11980_d_n5, assign17470_e11980_d_n6, assign17470_e11980_d_n7, assign17470_e11980_d_n8, assign17470_e11980_d_n9, assign17470_e11980_d_n10, assign17470_e11980_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard355 != 0.0)) && (locals.var_guard357 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn13,)
    }
};
        locals.var_uc_depvmax = assign17470_e11980;
        locals.var_uc_depvmax_dn0 = assign17470_e11980_d_n0;
        locals.var_uc_depvmax_dn2 = assign17470_e11980_d_n2;
        locals.var_uc_depvmax_dn4 = assign17470_e11980_d_n4;
        locals.var_uc_depvmax_dn5 = assign17470_e11980_d_n5;
        locals.var_uc_depvmax_dn6 = assign17470_e11980_d_n6;
        locals.var_uc_depvmax_dn7 = assign17470_e11980_d_n7;
        locals.var_uc_depvmax_dn8 = assign17470_e11980_d_n8;
        locals.var_uc_depvmax_dn9 = assign17470_e11980_d_n9;
        locals.var_uc_depvmax_dn10 = assign17470_e11980_d_n10;
        locals.var_uc_depvmax_dn13 = assign17470_e11980_d_n13;
        locals.var_uc_depvmax_rv = 0.0;

        let (assign17480_e11990, assign17480_e11990_d_n0, assign17480_e11990_d_n2, assign17480_e11990_d_n4, assign17480_e11990_d_n5, assign17480_e11990_d_n6, assign17480_e11990_d_n7, assign17480_e11990_d_n8, assign17480_e11990_d_n9, assign17480_e11990_d_n10, assign17480_e11990_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard355 != 0.0)) {
        let assign17480_e11987: f64 = (locals.var_tratio).powf(p.p381);
        let assign17480_e11988: f64 = (locals.var_uc_depmue0 / assign17480_e11987);
        (assign17480_e11988, (((locals.var_uc_depmue0_dn0 * assign17480_e11987) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn0)) } } else { (assign17480_e11987 * (p.p381 * (locals.var_tratio_dn0 / locals.var_tratio))) })) / (assign17480_e11987 * assign17480_e11987)), (((locals.var_uc_depmue0_dn2 * assign17480_e11987) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn2)) } } else { (assign17480_e11987 * (p.p381 * (locals.var_tratio_dn2 / locals.var_tratio))) })) / (assign17480_e11987 * assign17480_e11987)), (((locals.var_uc_depmue0_dn4 * assign17480_e11987) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn4)) } } else { (assign17480_e11987 * (p.p381 * (locals.var_tratio_dn4 / locals.var_tratio))) })) / (assign17480_e11987 * assign17480_e11987)), (((locals.var_uc_depmue0_dn5 * assign17480_e11987) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn5)) } } else { (assign17480_e11987 * (p.p381 * (locals.var_tratio_dn5 / locals.var_tratio))) })) / (assign17480_e11987 * assign17480_e11987)), (((locals.var_uc_depmue0_dn6 * assign17480_e11987) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn6)) } } else { (assign17480_e11987 * (p.p381 * (locals.var_tratio_dn6 / locals.var_tratio))) })) / (assign17480_e11987 * assign17480_e11987)), (((locals.var_uc_depmue0_dn7 * assign17480_e11987) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn7)) } } else { (assign17480_e11987 * (p.p381 * (locals.var_tratio_dn7 / locals.var_tratio))) })) / (assign17480_e11987 * assign17480_e11987)), (((locals.var_uc_depmue0_dn8 * assign17480_e11987) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn8)) } } else { (assign17480_e11987 * (p.p381 * (locals.var_tratio_dn8 / locals.var_tratio))) })) / (assign17480_e11987 * assign17480_e11987)), (((locals.var_uc_depmue0_dn9 * assign17480_e11987) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn9)) } } else { (assign17480_e11987 * (p.p381 * (locals.var_tratio_dn9 / locals.var_tratio))) })) / (assign17480_e11987 * assign17480_e11987)), (((locals.var_uc_depmue0_dn10 * assign17480_e11987) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn10)) } } else { (assign17480_e11987 * (p.p381 * (locals.var_tratio_dn10 / locals.var_tratio))) })) / (assign17480_e11987 * assign17480_e11987)), (((locals.var_uc_depmue0_dn13 * assign17480_e11987) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn13)) } } else { (assign17480_e11987 * (p.p381 * (locals.var_tratio_dn13 / locals.var_tratio))) })) / (assign17480_e11987 * assign17480_e11987)),)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn13,)
    }
};
        locals.var_uc_depmue0 = assign17480_e11990;
        locals.var_uc_depmue0_dn0 = assign17480_e11990_d_n0;
        locals.var_uc_depmue0_dn2 = assign17480_e11990_d_n2;
        locals.var_uc_depmue0_dn4 = assign17480_e11990_d_n4;
        locals.var_uc_depmue0_dn5 = assign17480_e11990_d_n5;
        locals.var_uc_depmue0_dn6 = assign17480_e11990_d_n6;
        locals.var_uc_depmue0_dn7 = assign17480_e11990_d_n7;
        locals.var_uc_depmue0_dn8 = assign17480_e11990_d_n8;
        locals.var_uc_depmue0_dn9 = assign17480_e11990_d_n9;
        locals.var_uc_depmue0_dn10 = assign17480_e11990_d_n10;
        locals.var_uc_depmue0_dn13 = assign17480_e11990_d_n13;
        locals.var_uc_depmue0_rv = 0.0;

        let (assign17490_e12000, assign17490_e12000_d_n0, assign17490_e12000_d_n2, assign17490_e12000_d_n4, assign17490_e12000_d_n5, assign17490_e12000_d_n6, assign17490_e12000_d_n7, assign17490_e12000_d_n8, assign17490_e12000_d_n9, assign17490_e12000_d_n10, assign17490_e12000_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard355 != 0.0)) {
        let assign17490_e11997: f64 = (locals.var_tratio).powf(p.p382);
        let assign17490_e11998: f64 = (locals.var_uc_depmue2 / assign17490_e11997);
        (assign17490_e11998, (((locals.var_uc_depmue2_dn0 * assign17490_e11997) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn0)) } } else { (assign17490_e11997 * (p.p382 * (locals.var_tratio_dn0 / locals.var_tratio))) })) / (assign17490_e11997 * assign17490_e11997)), (((locals.var_uc_depmue2_dn2 * assign17490_e11997) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn2)) } } else { (assign17490_e11997 * (p.p382 * (locals.var_tratio_dn2 / locals.var_tratio))) })) / (assign17490_e11997 * assign17490_e11997)), (((locals.var_uc_depmue2_dn4 * assign17490_e11997) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn4)) } } else { (assign17490_e11997 * (p.p382 * (locals.var_tratio_dn4 / locals.var_tratio))) })) / (assign17490_e11997 * assign17490_e11997)), (((locals.var_uc_depmue2_dn5 * assign17490_e11997) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn5)) } } else { (assign17490_e11997 * (p.p382 * (locals.var_tratio_dn5 / locals.var_tratio))) })) / (assign17490_e11997 * assign17490_e11997)), (((locals.var_uc_depmue2_dn6 * assign17490_e11997) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn6)) } } else { (assign17490_e11997 * (p.p382 * (locals.var_tratio_dn6 / locals.var_tratio))) })) / (assign17490_e11997 * assign17490_e11997)), (((locals.var_uc_depmue2_dn7 * assign17490_e11997) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn7)) } } else { (assign17490_e11997 * (p.p382 * (locals.var_tratio_dn7 / locals.var_tratio))) })) / (assign17490_e11997 * assign17490_e11997)), (((locals.var_uc_depmue2_dn8 * assign17490_e11997) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn8)) } } else { (assign17490_e11997 * (p.p382 * (locals.var_tratio_dn8 / locals.var_tratio))) })) / (assign17490_e11997 * assign17490_e11997)), (((locals.var_uc_depmue2_dn9 * assign17490_e11997) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn9)) } } else { (assign17490_e11997 * (p.p382 * (locals.var_tratio_dn9 / locals.var_tratio))) })) / (assign17490_e11997 * assign17490_e11997)), (((locals.var_uc_depmue2_dn10 * assign17490_e11997) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn10)) } } else { (assign17490_e11997 * (p.p382 * (locals.var_tratio_dn10 / locals.var_tratio))) })) / (assign17490_e11997 * assign17490_e11997)), (((locals.var_uc_depmue2_dn13 * assign17490_e11997) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn13)) } } else { (assign17490_e11997 * (p.p382 * (locals.var_tratio_dn13 / locals.var_tratio))) })) / (assign17490_e11997 * assign17490_e11997)),)
    } else {
        (locals.var_uc_depmue2, locals.var_uc_depmue2_dn0, locals.var_uc_depmue2_dn2, locals.var_uc_depmue2_dn4, locals.var_uc_depmue2_dn5, locals.var_uc_depmue2_dn6, locals.var_uc_depmue2_dn7, locals.var_uc_depmue2_dn8, locals.var_uc_depmue2_dn9, locals.var_uc_depmue2_dn10, locals.var_uc_depmue2_dn13,)
    }
};
        locals.var_uc_depmue2 = assign17490_e12000;
        locals.var_uc_depmue2_dn0 = assign17490_e12000_d_n0;
        locals.var_uc_depmue2_dn2 = assign17490_e12000_d_n2;
        locals.var_uc_depmue2_dn4 = assign17490_e12000_d_n4;
        locals.var_uc_depmue2_dn5 = assign17490_e12000_d_n5;
        locals.var_uc_depmue2_dn6 = assign17490_e12000_d_n6;
        locals.var_uc_depmue2_dn7 = assign17490_e12000_d_n7;
        locals.var_uc_depmue2_dn8 = assign17490_e12000_d_n8;
        locals.var_uc_depmue2_dn9 = assign17490_e12000_d_n9;
        locals.var_uc_depmue2_dn10 = assign17490_e12000_d_n10;
        locals.var_uc_depmue2_dn13 = assign17490_e12000_d_n13;
        locals.var_uc_depmue2_rv = 0.0;

        let assign17500_e12003: f64 = if locals.var_uc_codep == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard358 = assign17500_e12003;
        locals.var_guard358_rv = 0.0;

        let (assign17510_e12021, assign17510_e12021_d_n0, assign17510_e12021_d_n2, assign17510_e12021_d_n4, assign17510_e12021_d_n5, assign17510_e12021_d_n6, assign17510_e12021_d_n7, assign17510_e12021_d_n8, assign17510_e12021_d_n9, assign17510_e12021_d_n10, assign17510_e12021_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard355 == 0.0)) && (locals.var_guard358 != 0.0)) {
        let assign17510_e12012: f64 = (2.0 * 1.034943e-10);
        let assign17510_e12014: f64 = (assign17510_e12012 * 1.6021918e-19);
        let assign17510_e12016: f64 = (assign17510_e12014 * locals.var_uc_ndepm);
        let assign17510_e12018: f64 = (assign17510_e12016 * locals.var_beta_inv);
        let assign17510_e12019: f64 = (assign17510_e12018).sqrt();
        (assign17510_e12019, ((((assign17510_e12014 * locals.var_uc_ndepm_dn0) * locals.var_beta_inv) + (assign17510_e12016 * locals.var_beta_inv_dn0)) / (2.0 * assign17510_e12019)), ((((assign17510_e12014 * locals.var_uc_ndepm_dn2) * locals.var_beta_inv) + (assign17510_e12016 * locals.var_beta_inv_dn2)) / (2.0 * assign17510_e12019)), ((((assign17510_e12014 * locals.var_uc_ndepm_dn4) * locals.var_beta_inv) + (assign17510_e12016 * locals.var_beta_inv_dn4)) / (2.0 * assign17510_e12019)), ((((assign17510_e12014 * locals.var_uc_ndepm_dn5) * locals.var_beta_inv) + (assign17510_e12016 * locals.var_beta_inv_dn5)) / (2.0 * assign17510_e12019)), ((((assign17510_e12014 * locals.var_uc_ndepm_dn6) * locals.var_beta_inv) + (assign17510_e12016 * locals.var_beta_inv_dn6)) / (2.0 * assign17510_e12019)), ((((assign17510_e12014 * locals.var_uc_ndepm_dn7) * locals.var_beta_inv) + (assign17510_e12016 * locals.var_beta_inv_dn7)) / (2.0 * assign17510_e12019)), ((((assign17510_e12014 * locals.var_uc_ndepm_dn8) * locals.var_beta_inv) + (assign17510_e12016 * locals.var_beta_inv_dn8)) / (2.0 * assign17510_e12019)), ((((assign17510_e12014 * locals.var_uc_ndepm_dn9) * locals.var_beta_inv) + (assign17510_e12016 * locals.var_beta_inv_dn9)) / (2.0 * assign17510_e12019)), ((((assign17510_e12014 * locals.var_uc_ndepm_dn10) * locals.var_beta_inv) + (assign17510_e12016 * locals.var_beta_inv_dn10)) / (2.0 * assign17510_e12019)), ((((assign17510_e12014 * locals.var_uc_ndepm_dn13) * locals.var_beta_inv) + (assign17510_e12016 * locals.var_beta_inv_dn13)) / (2.0 * assign17510_e12019)),)
    } else {
        (locals.var_cnst0, locals.var_cnst0_dn0, locals.var_cnst0_dn2, locals.var_cnst0_dn4, locals.var_cnst0_dn5, locals.var_cnst0_dn6, locals.var_cnst0_dn7, locals.var_cnst0_dn8, locals.var_cnst0_dn9, locals.var_cnst0_dn10, locals.var_cnst0_dn13,)
    }
};
        locals.var_cnst0 = assign17510_e12021;
        locals.var_cnst0_dn0 = assign17510_e12021_d_n0;
        locals.var_cnst0_dn2 = assign17510_e12021_d_n2;
        locals.var_cnst0_dn4 = assign17510_e12021_d_n4;
        locals.var_cnst0_dn5 = assign17510_e12021_d_n5;
        locals.var_cnst0_dn6 = assign17510_e12021_d_n6;
        locals.var_cnst0_dn7 = assign17510_e12021_d_n7;
        locals.var_cnst0_dn8 = assign17510_e12021_d_n8;
        locals.var_cnst0_dn9 = assign17510_e12021_d_n9;
        locals.var_cnst0_dn10 = assign17510_e12021_d_n10;
        locals.var_cnst0_dn13 = assign17510_e12021_d_n13;
        locals.var_cnst0_rv = 0.0;

        let (assign17520_e12036, assign17520_e12036_d_n0, assign17520_e12036_d_n2, assign17520_e12036_d_n4, assign17520_e12036_d_n5, assign17520_e12036_d_n6, assign17520_e12036_d_n7, assign17520_e12036_d_n8, assign17520_e12036_d_n9, assign17520_e12036_d_n10, assign17520_e12036_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard355 == 0.0)) && (locals.var_guard358 != 0.0)) {
        let assign17520_e12030: f64 = (locals.var_nin * locals.var_nin);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_uc_ndepm;
        let assign17520_e12032: f64 = (assign17520_e12030 * __rspice_inv_cse_0);
        let assign17520_e12034: f64 = (assign17520_e12032 * __rspice_inv_cse_0);
        (assign17520_e12034, ((((((((locals.var_nin_dn0 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn0)) * locals.var_uc_ndepm) - (assign17520_e12030 * locals.var_uc_ndepm_dn0)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17520_e12032 * locals.var_uc_ndepm_dn0)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn2 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn2)) * locals.var_uc_ndepm) - (assign17520_e12030 * locals.var_uc_ndepm_dn2)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17520_e12032 * locals.var_uc_ndepm_dn2)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn4 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn4)) * locals.var_uc_ndepm) - (assign17520_e12030 * locals.var_uc_ndepm_dn4)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17520_e12032 * locals.var_uc_ndepm_dn4)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn5 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn5)) * locals.var_uc_ndepm) - (assign17520_e12030 * locals.var_uc_ndepm_dn5)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17520_e12032 * locals.var_uc_ndepm_dn5)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn6 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn6)) * locals.var_uc_ndepm) - (assign17520_e12030 * locals.var_uc_ndepm_dn6)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17520_e12032 * locals.var_uc_ndepm_dn6)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn7 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn7)) * locals.var_uc_ndepm) - (assign17520_e12030 * locals.var_uc_ndepm_dn7)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17520_e12032 * locals.var_uc_ndepm_dn7)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn8 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn8)) * locals.var_uc_ndepm) - (assign17520_e12030 * locals.var_uc_ndepm_dn8)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17520_e12032 * locals.var_uc_ndepm_dn8)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn9 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn9)) * locals.var_uc_ndepm) - (assign17520_e12030 * locals.var_uc_ndepm_dn9)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17520_e12032 * locals.var_uc_ndepm_dn9)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn10 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn10)) * locals.var_uc_ndepm) - (assign17520_e12030 * locals.var_uc_ndepm_dn10)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17520_e12032 * locals.var_uc_ndepm_dn10)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn13 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn13)) * locals.var_uc_ndepm) - (assign17520_e12030 * locals.var_uc_ndepm_dn13)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign17520_e12032 * locals.var_uc_ndepm_dn13)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)),)
    } else {
        (locals.var_cnst1, locals.var_cnst1_dn0, locals.var_cnst1_dn2, locals.var_cnst1_dn4, locals.var_cnst1_dn5, locals.var_cnst1_dn6, locals.var_cnst1_dn7, locals.var_cnst1_dn8, locals.var_cnst1_dn9, locals.var_cnst1_dn10, locals.var_cnst1_dn13,)
    }
};
        locals.var_cnst1 = assign17520_e12036;
        locals.var_cnst1_dn0 = assign17520_e12036_d_n0;
        locals.var_cnst1_dn2 = assign17520_e12036_d_n2;
        locals.var_cnst1_dn4 = assign17520_e12036_d_n4;
        locals.var_cnst1_dn5 = assign17520_e12036_d_n5;
        locals.var_cnst1_dn6 = assign17520_e12036_d_n6;
        locals.var_cnst1_dn7 = assign17520_e12036_d_n7;
        locals.var_cnst1_dn8 = assign17520_e12036_d_n8;
        locals.var_cnst1_dn9 = assign17520_e12036_d_n9;
        locals.var_cnst1_dn10 = assign17520_e12036_d_n10;
        locals.var_cnst1_dn13 = assign17520_e12036_d_n13;
        locals.var_cnst1_rv = 0.0;

        let (assign17530_e12052, assign17530_e12052_d_n0, assign17530_e12052_d_n2, assign17530_e12052_d_n4, assign17530_e12052_d_n5, assign17530_e12052_d_n6, assign17530_e12052_d_n7, assign17530_e12052_d_n8, assign17530_e12052_d_n9, assign17530_e12052_d_n10, assign17530_e12052_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard355 == 0.0)) && (locals.var_guard358 != 0.0)) {
        let assign17530_e12045: f64 = (2.0 * locals.var_beta_inv);
        let assign17530_e12048: f64 = (locals.var_uc_ndepm / locals.var_nin);
        let assign17530_e12049: f64 = (assign17530_e12048).ln();
        let assign17530_e12050: f64 = (assign17530_e12045 * assign17530_e12049);
        (assign17530_e12050, (((2.0 * locals.var_beta_inv_dn0) * assign17530_e12049) + (assign17530_e12045 * ((((locals.var_uc_ndepm_dn0 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign17530_e12048))), (((2.0 * locals.var_beta_inv_dn2) * assign17530_e12049) + (assign17530_e12045 * ((((locals.var_uc_ndepm_dn2 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign17530_e12048))), (((2.0 * locals.var_beta_inv_dn4) * assign17530_e12049) + (assign17530_e12045 * ((((locals.var_uc_ndepm_dn4 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign17530_e12048))), (((2.0 * locals.var_beta_inv_dn5) * assign17530_e12049) + (assign17530_e12045 * ((((locals.var_uc_ndepm_dn5 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign17530_e12048))), (((2.0 * locals.var_beta_inv_dn6) * assign17530_e12049) + (assign17530_e12045 * ((((locals.var_uc_ndepm_dn6 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign17530_e12048))), (((2.0 * locals.var_beta_inv_dn7) * assign17530_e12049) + (assign17530_e12045 * ((((locals.var_uc_ndepm_dn7 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign17530_e12048))), (((2.0 * locals.var_beta_inv_dn8) * assign17530_e12049) + (assign17530_e12045 * ((((locals.var_uc_ndepm_dn8 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign17530_e12048))), (((2.0 * locals.var_beta_inv_dn9) * assign17530_e12049) + (assign17530_e12045 * ((((locals.var_uc_ndepm_dn9 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign17530_e12048))), (((2.0 * locals.var_beta_inv_dn10) * assign17530_e12049) + (assign17530_e12045 * ((((locals.var_uc_ndepm_dn10 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign17530_e12048))), (((2.0 * locals.var_beta_inv_dn13) * assign17530_e12049) + (assign17530_e12045 * ((((locals.var_uc_ndepm_dn13 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn13)) / (locals.var_nin * locals.var_nin)) / assign17530_e12048))),)
    } else {
        (locals.var_pb2n, locals.var_pb2n_dn0, locals.var_pb2n_dn2, locals.var_pb2n_dn4, locals.var_pb2n_dn5, locals.var_pb2n_dn6, locals.var_pb2n_dn7, locals.var_pb2n_dn8, locals.var_pb2n_dn9, locals.var_pb2n_dn10, locals.var_pb2n_dn13,)
    }
};
        locals.var_pb2n = assign17530_e12052;
        locals.var_pb2n_dn0 = assign17530_e12052_d_n0;
        locals.var_pb2n_dn2 = assign17530_e12052_d_n2;
        locals.var_pb2n_dn4 = assign17530_e12052_d_n4;
        locals.var_pb2n_dn5 = assign17530_e12052_d_n5;
        locals.var_pb2n_dn6 = assign17530_e12052_d_n6;
        locals.var_pb2n_dn7 = assign17530_e12052_d_n7;
        locals.var_pb2n_dn8 = assign17530_e12052_d_n8;
        locals.var_pb2n_dn9 = assign17530_e12052_d_n9;
        locals.var_pb2n_dn10 = assign17530_e12052_d_n10;
        locals.var_pb2n_dn13 = assign17530_e12052_d_n13;
        locals.var_pb2n_rv = 0.0;

        let (assign17540_e12070, assign17540_e12070_d_n0, assign17540_e12070_d_n2, assign17540_e12070_d_n4, assign17540_e12070_d_n5, assign17540_e12070_d_n6, assign17540_e12070_d_n7, assign17540_e12070_d_n8, assign17540_e12070_d_n9, assign17540_e12070_d_n10, assign17540_e12070_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard355 == 0.0)) && (locals.var_guard358 != 0.0)) {
        let assign17540_e12062: f64 = (locals.var_uc_ndepm * locals.var_ef_nsubc);
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_nin;
        let assign17540_e12064: f64 = (assign17540_e12062 * __rspice_inv_cse_1);
        let assign17540_e12066: f64 = (assign17540_e12064 * __rspice_inv_cse_1);
        let assign17540_e12067: f64 = (assign17540_e12066).ln();
        let assign17540_e12068: f64 = (locals.var_beta_inv * assign17540_e12067);
        (assign17540_e12068, ((locals.var_beta_inv_dn0 * assign17540_e12067) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn0 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn0)) * locals.var_nin) - (assign17540_e12062 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17540_e12064 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign17540_e12066))), ((locals.var_beta_inv_dn2 * assign17540_e12067) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn2 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn2)) * locals.var_nin) - (assign17540_e12062 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17540_e12064 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign17540_e12066))), ((locals.var_beta_inv_dn4 * assign17540_e12067) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn4 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn4)) * locals.var_nin) - (assign17540_e12062 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17540_e12064 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign17540_e12066))), ((locals.var_beta_inv_dn5 * assign17540_e12067) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn5 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn5)) * locals.var_nin) - (assign17540_e12062 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17540_e12064 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign17540_e12066))), ((locals.var_beta_inv_dn6 * assign17540_e12067) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn6 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn6)) * locals.var_nin) - (assign17540_e12062 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17540_e12064 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign17540_e12066))), ((locals.var_beta_inv_dn7 * assign17540_e12067) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn7 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn7)) * locals.var_nin) - (assign17540_e12062 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17540_e12064 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign17540_e12066))), ((locals.var_beta_inv_dn8 * assign17540_e12067) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn8 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn8)) * locals.var_nin) - (assign17540_e12062 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17540_e12064 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign17540_e12066))), ((locals.var_beta_inv_dn9 * assign17540_e12067) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn9 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn9)) * locals.var_nin) - (assign17540_e12062 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17540_e12064 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign17540_e12066))), ((locals.var_beta_inv_dn10 * assign17540_e12067) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn10 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn10)) * locals.var_nin) - (assign17540_e12062 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17540_e12064 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign17540_e12066))), ((locals.var_beta_inv_dn13 * assign17540_e12067) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn13 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn13)) * locals.var_nin) - (assign17540_e12062 * locals.var_nin_dn13)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign17540_e12064 * locals.var_nin_dn13)) / (locals.var_nin * locals.var_nin)) / assign17540_e12066))),)
    } else {
        (locals.var_vbipn, locals.var_vbipn_dn0, locals.var_vbipn_dn2, locals.var_vbipn_dn4, locals.var_vbipn_dn5, locals.var_vbipn_dn6, locals.var_vbipn_dn7, locals.var_vbipn_dn8, locals.var_vbipn_dn9, locals.var_vbipn_dn10, locals.var_vbipn_dn13,)
    }
};
        locals.var_vbipn = assign17540_e12070;
        locals.var_vbipn_dn0 = assign17540_e12070_d_n0;
        locals.var_vbipn_dn2 = assign17540_e12070_d_n2;
        locals.var_vbipn_dn4 = assign17540_e12070_d_n4;
        locals.var_vbipn_dn5 = assign17540_e12070_d_n5;
        locals.var_vbipn_dn6 = assign17540_e12070_d_n6;
        locals.var_vbipn_dn7 = assign17540_e12070_d_n7;
        locals.var_vbipn_dn8 = assign17540_e12070_d_n8;
        locals.var_vbipn_dn9 = assign17540_e12070_d_n9;
        locals.var_vbipn_dn10 = assign17540_e12070_d_n10;
        locals.var_vbipn_dn13 = assign17540_e12070_d_n13;
        locals.var_vbipn_rv = 0.0;

        let (assign17550_e12082, assign17550_e12082_d_n0, assign17550_e12082_d_n2, assign17550_e12082_d_n4, assign17550_e12082_d_n5, assign17550_e12082_d_n6, assign17550_e12082_d_n7, assign17550_e12082_d_n8, assign17550_e12082_d_n9, assign17550_e12082_d_n10, assign17550_e12082_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard355 == 0.0)) && (locals.var_guard358 != 0.0)) {
        let assign17550_e12079: f64 = (locals.var_log_tratio * p.p380);
        let assign17550_e12080: f64 = (assign17550_e12079).exp();
        (assign17550_e12080, (assign17550_e12080 * (locals.var_log_tratio_dn0 * p.p380)), (assign17550_e12080 * (locals.var_log_tratio_dn2 * p.p380)), (assign17550_e12080 * (locals.var_log_tratio_dn4 * p.p380)), (assign17550_e12080 * (locals.var_log_tratio_dn5 * p.p380)), (assign17550_e12080 * (locals.var_log_tratio_dn6 * p.p380)), (assign17550_e12080 * (locals.var_log_tratio_dn7 * p.p380)), (assign17550_e12080 * (locals.var_log_tratio_dn8 * p.p380)), (assign17550_e12080 * (locals.var_log_tratio_dn9 * p.p380)), (assign17550_e12080 * (locals.var_log_tratio_dn10 * p.p380)), (assign17550_e12080 * (locals.var_log_tratio_dn13 * p.p380)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign17550_e12082;
        locals.var_t1_dn0 = assign17550_e12082_d_n0;
        locals.var_t1_dn2 = assign17550_e12082_d_n2;
        locals.var_t1_dn4 = assign17550_e12082_d_n4;
        locals.var_t1_dn5 = assign17550_e12082_d_n5;
        locals.var_t1_dn6 = assign17550_e12082_d_n6;
        locals.var_t1_dn7 = assign17550_e12082_d_n7;
        locals.var_t1_dn8 = assign17550_e12082_d_n8;
        locals.var_t1_dn9 = assign17550_e12082_d_n9;
        locals.var_t1_dn10 = assign17550_e12082_d_n10;
        locals.var_t1_dn13 = assign17550_e12082_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign17560_e12093, assign17560_e12093_d_n0, assign17560_e12093_d_n2, assign17560_e12093_d_n4, assign17560_e12093_d_n5, assign17560_e12093_d_n6, assign17560_e12093_d_n7, assign17560_e12093_d_n8, assign17560_e12093_d_n9, assign17560_e12093_d_n10, assign17560_e12093_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard355 == 0.0)) && (locals.var_guard358 != 0.0)) {
        let assign17560_e12091: f64 = (locals.var_t1 / locals.var_uc_depmueph1);
        (assign17560_e12091, (locals.var_t1_dn0 / locals.var_uc_depmueph1), (locals.var_t1_dn2 / locals.var_uc_depmueph1), (locals.var_t1_dn4 / locals.var_uc_depmueph1), (locals.var_t1_dn5 / locals.var_uc_depmueph1), (locals.var_t1_dn6 / locals.var_uc_depmueph1), (locals.var_t1_dn7 / locals.var_uc_depmueph1), (locals.var_t1_dn8 / locals.var_uc_depmueph1), (locals.var_t1_dn9 / locals.var_uc_depmueph1), (locals.var_t1_dn10 / locals.var_uc_depmueph1), (locals.var_t1_dn13 / locals.var_uc_depmueph1),)
    } else {
        (locals.var_depmphn0, locals.var_depmphn0_dn0, locals.var_depmphn0_dn2, locals.var_depmphn0_dn4, locals.var_depmphn0_dn5, locals.var_depmphn0_dn6, locals.var_depmphn0_dn7, locals.var_depmphn0_dn8, locals.var_depmphn0_dn9, locals.var_depmphn0_dn10, locals.var_depmphn0_dn13,)
    }
};
        locals.var_depmphn0 = assign17560_e12093;
        locals.var_depmphn0_dn0 = assign17560_e12093_d_n0;
        locals.var_depmphn0_dn2 = assign17560_e12093_d_n2;
        locals.var_depmphn0_dn4 = assign17560_e12093_d_n4;
        locals.var_depmphn0_dn5 = assign17560_e12093_d_n5;
        locals.var_depmphn0_dn6 = assign17560_e12093_d_n6;
        locals.var_depmphn0_dn7 = assign17560_e12093_d_n7;
        locals.var_depmphn0_dn8 = assign17560_e12093_d_n8;
        locals.var_depmphn0_dn9 = assign17560_e12093_d_n9;
        locals.var_depmphn0_dn10 = assign17560_e12093_d_n10;
        locals.var_depmphn0_dn13 = assign17560_e12093_d_n13;
        locals.var_depmphn0_rv = 0.0;

        let (assign17570_e12118, assign17570_e12118_d_n0, assign17570_e12118_d_n2, assign17570_e12118_d_n4, assign17570_e12118_d_n5, assign17570_e12118_d_n6, assign17570_e12118_d_n7, assign17570_e12118_d_n8, assign17570_e12118_d_n9, assign17570_e12118_d_n10, assign17570_e12118_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard355 == 0.0)) && (locals.var_guard358 != 0.0)) {
        let assign17570_e12103: f64 = (0.4 * locals.var_tratio);
        let assign17570_e12104: f64 = (1.8 + assign17570_e12103);
        let assign17570_e12107: f64 = (0.1 * locals.var_tratio);
        let assign17570_e12109: f64 = (assign17570_e12107 * locals.var_tratio);
        let assign17570_e12110: f64 = (assign17570_e12104 + assign17570_e12109);
        let assign17570_e12114: f64 = (1.0 - locals.var_tratio);
        let assign17570_e12115: f64 = (p.p379 * assign17570_e12114);
        let assign17570_e12116: f64 = (assign17570_e12110 - assign17570_e12115);
        (assign17570_e12116, (((0.4 * locals.var_tratio_dn0) + (((0.1 * locals.var_tratio_dn0) * locals.var_tratio) + (assign17570_e12107 * locals.var_tratio_dn0))) - (p.p379 * (-locals.var_tratio_dn0))), (((0.4 * locals.var_tratio_dn2) + (((0.1 * locals.var_tratio_dn2) * locals.var_tratio) + (assign17570_e12107 * locals.var_tratio_dn2))) - (p.p379 * (-locals.var_tratio_dn2))), (((0.4 * locals.var_tratio_dn4) + (((0.1 * locals.var_tratio_dn4) * locals.var_tratio) + (assign17570_e12107 * locals.var_tratio_dn4))) - (p.p379 * (-locals.var_tratio_dn4))), (((0.4 * locals.var_tratio_dn5) + (((0.1 * locals.var_tratio_dn5) * locals.var_tratio) + (assign17570_e12107 * locals.var_tratio_dn5))) - (p.p379 * (-locals.var_tratio_dn5))), (((0.4 * locals.var_tratio_dn6) + (((0.1 * locals.var_tratio_dn6) * locals.var_tratio) + (assign17570_e12107 * locals.var_tratio_dn6))) - (p.p379 * (-locals.var_tratio_dn6))), (((0.4 * locals.var_tratio_dn7) + (((0.1 * locals.var_tratio_dn7) * locals.var_tratio) + (assign17570_e12107 * locals.var_tratio_dn7))) - (p.p379 * (-locals.var_tratio_dn7))), (((0.4 * locals.var_tratio_dn8) + (((0.1 * locals.var_tratio_dn8) * locals.var_tratio) + (assign17570_e12107 * locals.var_tratio_dn8))) - (p.p379 * (-locals.var_tratio_dn8))), (((0.4 * locals.var_tratio_dn9) + (((0.1 * locals.var_tratio_dn9) * locals.var_tratio) + (assign17570_e12107 * locals.var_tratio_dn9))) - (p.p379 * (-locals.var_tratio_dn9))), (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign17570_e12107 * locals.var_tratio_dn10))) - (p.p379 * (-locals.var_tratio_dn10))), (((0.4 * locals.var_tratio_dn13) + (((0.1 * locals.var_tratio_dn13) * locals.var_tratio) + (assign17570_e12107 * locals.var_tratio_dn13))) - (p.p379 * (-locals.var_tratio_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign17570_e12118;
        locals.var_t0_dn0 = assign17570_e12118_d_n0;
        locals.var_t0_dn2 = assign17570_e12118_d_n2;
        locals.var_t0_dn4 = assign17570_e12118_d_n4;
        locals.var_t0_dn5 = assign17570_e12118_d_n5;
        locals.var_t0_dn6 = assign17570_e12118_d_n6;
        locals.var_t0_dn7 = assign17570_e12118_d_n7;
        locals.var_t0_dn8 = assign17570_e12118_d_n8;
        locals.var_t0_dn9 = assign17570_e12118_d_n9;
        locals.var_t0_dn10 = assign17570_e12118_d_n10;
        locals.var_t0_dn13 = assign17570_e12118_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign17580_e12129, assign17580_e12129_d_n0, assign17580_e12129_d_n2, assign17580_e12129_d_n4, assign17580_e12129_d_n5, assign17580_e12129_d_n6, assign17580_e12129_d_n7, assign17580_e12129_d_n8, assign17580_e12129_d_n9, assign17580_e12129_d_n10, assign17580_e12129_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard355 == 0.0)) && (locals.var_guard358 != 0.0)) {
        let assign17580_e12127: f64 = (locals.var_uc_depvmax / locals.var_t0);
        (assign17580_e12127, (((locals.var_uc_depvmax_dn0 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn0)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn2 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn2)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn4 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn5 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn6 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn7 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn8 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn9 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn10 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn13 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn13)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn13,)
    }
};
        locals.var_uc_depvmax = assign17580_e12129;
        locals.var_uc_depvmax_dn0 = assign17580_e12129_d_n0;
        locals.var_uc_depvmax_dn2 = assign17580_e12129_d_n2;
        locals.var_uc_depvmax_dn4 = assign17580_e12129_d_n4;
        locals.var_uc_depvmax_dn5 = assign17580_e12129_d_n5;
        locals.var_uc_depvmax_dn6 = assign17580_e12129_d_n6;
        locals.var_uc_depvmax_dn7 = assign17580_e12129_d_n7;
        locals.var_uc_depvmax_dn8 = assign17580_e12129_d_n8;
        locals.var_uc_depvmax_dn9 = assign17580_e12129_d_n9;
        locals.var_uc_depvmax_dn10 = assign17580_e12129_d_n10;
        locals.var_uc_depvmax_dn13 = assign17580_e12129_d_n13;
        locals.var_uc_depvmax_rv = 0.0;

        let assign17600_e12137: f64 = if locals.var_uc_depvmax < 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard360 = assign17600_e12137;
        locals.var_guard360_rv = 0.0;

        let (assign17610_e12148, assign17610_e12148_d_n0, assign17610_e12148_d_n2, assign17610_e12148_d_n4, assign17610_e12148_d_n5, assign17610_e12148_d_n6, assign17610_e12148_d_n7, assign17610_e12148_d_n8, assign17610_e12148_d_n9, assign17610_e12148_d_n10, assign17610_e12148_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard355 == 0.0)) && (locals.var_guard358 != 0.0)) && (locals.var_guard360 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn13,)
    }
};
        locals.var_uc_depvmax = assign17610_e12148;
        locals.var_uc_depvmax_dn0 = assign17610_e12148_d_n0;
        locals.var_uc_depvmax_dn2 = assign17610_e12148_d_n2;
        locals.var_uc_depvmax_dn4 = assign17610_e12148_d_n4;
        locals.var_uc_depvmax_dn5 = assign17610_e12148_d_n5;
        locals.var_uc_depvmax_dn6 = assign17610_e12148_d_n6;
        locals.var_uc_depvmax_dn7 = assign17610_e12148_d_n7;
        locals.var_uc_depvmax_dn8 = assign17610_e12148_d_n8;
        locals.var_uc_depvmax_dn9 = assign17610_e12148_d_n9;
        locals.var_uc_depvmax_dn10 = assign17610_e12148_d_n10;
        locals.var_uc_depvmax_dn13 = assign17610_e12148_d_n13;
        locals.var_uc_depvmax_rv = 0.0;

        let (assign17620_e12161, assign17620_e12161_d_n0, assign17620_e12161_d_n2, assign17620_e12161_d_n4, assign17620_e12161_d_n5, assign17620_e12161_d_n6, assign17620_e12161_d_n7, assign17620_e12161_d_n8, assign17620_e12161_d_n9, assign17620_e12161_d_n10, assign17620_e12161_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard355 == 0.0)) && (locals.var_guard358 != 0.0)) {
        let assign17620_e12158: f64 = (locals.var_tratio).powf(p.p381);
        let assign17620_e12159: f64 = (locals.var_uc_depmue0 / assign17620_e12158);
        (assign17620_e12159, (((locals.var_uc_depmue0_dn0 * assign17620_e12158) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn0)) } } else { (assign17620_e12158 * (p.p381 * (locals.var_tratio_dn0 / locals.var_tratio))) })) / (assign17620_e12158 * assign17620_e12158)), (((locals.var_uc_depmue0_dn2 * assign17620_e12158) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn2)) } } else { (assign17620_e12158 * (p.p381 * (locals.var_tratio_dn2 / locals.var_tratio))) })) / (assign17620_e12158 * assign17620_e12158)), (((locals.var_uc_depmue0_dn4 * assign17620_e12158) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn4)) } } else { (assign17620_e12158 * (p.p381 * (locals.var_tratio_dn4 / locals.var_tratio))) })) / (assign17620_e12158 * assign17620_e12158)), (((locals.var_uc_depmue0_dn5 * assign17620_e12158) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn5)) } } else { (assign17620_e12158 * (p.p381 * (locals.var_tratio_dn5 / locals.var_tratio))) })) / (assign17620_e12158 * assign17620_e12158)), (((locals.var_uc_depmue0_dn6 * assign17620_e12158) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn6)) } } else { (assign17620_e12158 * (p.p381 * (locals.var_tratio_dn6 / locals.var_tratio))) })) / (assign17620_e12158 * assign17620_e12158)), (((locals.var_uc_depmue0_dn7 * assign17620_e12158) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn7)) } } else { (assign17620_e12158 * (p.p381 * (locals.var_tratio_dn7 / locals.var_tratio))) })) / (assign17620_e12158 * assign17620_e12158)), (((locals.var_uc_depmue0_dn8 * assign17620_e12158) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn8)) } } else { (assign17620_e12158 * (p.p381 * (locals.var_tratio_dn8 / locals.var_tratio))) })) / (assign17620_e12158 * assign17620_e12158)), (((locals.var_uc_depmue0_dn9 * assign17620_e12158) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn9)) } } else { (assign17620_e12158 * (p.p381 * (locals.var_tratio_dn9 / locals.var_tratio))) })) / (assign17620_e12158 * assign17620_e12158)), (((locals.var_uc_depmue0_dn10 * assign17620_e12158) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn10)) } } else { (assign17620_e12158 * (p.p381 * (locals.var_tratio_dn10 / locals.var_tratio))) })) / (assign17620_e12158 * assign17620_e12158)), (((locals.var_uc_depmue0_dn13 * assign17620_e12158) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn13)) } } else { (assign17620_e12158 * (p.p381 * (locals.var_tratio_dn13 / locals.var_tratio))) })) / (assign17620_e12158 * assign17620_e12158)),)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn13,)
    }
};
        locals.var_uc_depmue0 = assign17620_e12161;
        locals.var_uc_depmue0_dn0 = assign17620_e12161_d_n0;
        locals.var_uc_depmue0_dn2 = assign17620_e12161_d_n2;
        locals.var_uc_depmue0_dn4 = assign17620_e12161_d_n4;
        locals.var_uc_depmue0_dn5 = assign17620_e12161_d_n5;
        locals.var_uc_depmue0_dn6 = assign17620_e12161_d_n6;
        locals.var_uc_depmue0_dn7 = assign17620_e12161_d_n7;
        locals.var_uc_depmue0_dn8 = assign17620_e12161_d_n8;
        locals.var_uc_depmue0_dn9 = assign17620_e12161_d_n9;
        locals.var_uc_depmue0_dn10 = assign17620_e12161_d_n10;
        locals.var_uc_depmue0_dn13 = assign17620_e12161_d_n13;
        locals.var_uc_depmue0_rv = 0.0;

        let (assign17630_e12176, assign17630_e12176_d_n0, assign17630_e12176_d_n2, assign17630_e12176_d_n4, assign17630_e12176_d_n5, assign17630_e12176_d_n6, assign17630_e12176_d_n7, assign17630_e12176_d_n8, assign17630_e12176_d_n9, assign17630_e12176_d_n10, assign17630_e12176_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard355 == 0.0)) && (locals.var_guard358 != 0.0)) {
        let assign17630_e12172: f64 = (locals.var_tratio - 1.0);
        let assign17630_e12173: f64 = (p.p365 * assign17630_e12172);
        let assign17630_e12174: f64 = (p.p364 + assign17630_e12173);
        (assign17630_e12174, (p.p365 * locals.var_tratio_dn0), (p.p365 * locals.var_tratio_dn2), (p.p365 * locals.var_tratio_dn4), (p.p365 * locals.var_tratio_dn5), (p.p365 * locals.var_tratio_dn6), (p.p365 * locals.var_tratio_dn7), (p.p365 * locals.var_tratio_dn8), (p.p365 * locals.var_tratio_dn9), (p.p365 * locals.var_tratio_dn10), (p.p365 * locals.var_tratio_dn13),)
    } else {
        (locals.var_uc_depwlp, locals.var_uc_depwlp_dn0, locals.var_uc_depwlp_dn2, locals.var_uc_depwlp_dn4, locals.var_uc_depwlp_dn5, locals.var_uc_depwlp_dn6, locals.var_uc_depwlp_dn7, locals.var_uc_depwlp_dn8, locals.var_uc_depwlp_dn9, locals.var_uc_depwlp_dn10, locals.var_uc_depwlp_dn13,)
    }
};
        locals.var_uc_depwlp = assign17630_e12176;
        locals.var_uc_depwlp_dn0 = assign17630_e12176_d_n0;
        locals.var_uc_depwlp_dn2 = assign17630_e12176_d_n2;
        locals.var_uc_depwlp_dn4 = assign17630_e12176_d_n4;
        locals.var_uc_depwlp_dn5 = assign17630_e12176_d_n5;
        locals.var_uc_depwlp_dn6 = assign17630_e12176_d_n6;
        locals.var_uc_depwlp_dn7 = assign17630_e12176_d_n7;
        locals.var_uc_depwlp_dn8 = assign17630_e12176_d_n8;
        locals.var_uc_depwlp_dn9 = assign17630_e12176_d_n9;
        locals.var_uc_depwlp_dn10 = assign17630_e12176_d_n10;
        locals.var_uc_depwlp_dn13 = assign17630_e12176_d_n13;
        locals.var_uc_depwlp_rv = 0.0;

        let (assign17640_e12186, assign17640_e12186_d_n0, assign17640_e12186_d_n2, assign17640_e12186_d_n4, assign17640_e12186_d_n5, assign17640_e12186_d_n6, assign17640_e12186_d_n7, assign17640_e12186_d_n8, assign17640_e12186_d_n9, assign17640_e12186_d_n10, assign17640_e12186_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard355 == 0.0)) && (locals.var_guard358 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pb2n, locals.var_pb2n_dn0, locals.var_pb2n_dn2, locals.var_pb2n_dn4, locals.var_pb2n_dn5, locals.var_pb2n_dn6, locals.var_pb2n_dn7, locals.var_pb2n_dn8, locals.var_pb2n_dn9, locals.var_pb2n_dn10, locals.var_pb2n_dn13,)
    }
};
        locals.var_pb2n = assign17640_e12186;
        locals.var_pb2n_dn0 = assign17640_e12186_d_n0;
        locals.var_pb2n_dn2 = assign17640_e12186_d_n2;
        locals.var_pb2n_dn4 = assign17640_e12186_d_n4;
        locals.var_pb2n_dn5 = assign17640_e12186_d_n5;
        locals.var_pb2n_dn6 = assign17640_e12186_d_n6;
        locals.var_pb2n_dn7 = assign17640_e12186_d_n7;
        locals.var_pb2n_dn8 = assign17640_e12186_d_n8;
        locals.var_pb2n_dn9 = assign17640_e12186_d_n9;
        locals.var_pb2n_dn10 = assign17640_e12186_d_n10;
        locals.var_pb2n_dn13 = assign17640_e12186_d_n13;
        locals.var_pb2n_rv = 0.0;

        let (assign17650_e12205, assign17650_e12205_d_n0, assign17650_e12205_d_n2, assign17650_e12205_d_n4, assign17650_e12205_d_n5, assign17650_e12205_d_n6, assign17650_e12205_d_n7, assign17650_e12205_d_n8, assign17650_e12205_d_n9, assign17650_e12205_d_n10, assign17650_e12205_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard355 == 0.0)) && (locals.var_guard358 == 0.0)) {
        let assign17650_e12197: f64 = (locals.var_uc_njunc / locals.var_nin);
        let assign17650_e12199: f64 = (assign17650_e12197 * locals.var_nsub);
        let assign17650_e12201: f64 = (assign17650_e12199 / locals.var_nin);
        let assign17650_e12202: f64 = (assign17650_e12201).ln();
        let assign17650_e12203: f64 = (locals.var_beta_inv * assign17650_e12202);
        (assign17650_e12203, ((locals.var_beta_inv_dn0 * assign17650_e12202) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn0) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17650_e12197 * locals.var_nsub_dn0)) * locals.var_nin) - (assign17650_e12199 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign17650_e12201))), ((locals.var_beta_inv_dn2 * assign17650_e12202) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn2) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17650_e12197 * locals.var_nsub_dn2)) * locals.var_nin) - (assign17650_e12199 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign17650_e12201))), ((locals.var_beta_inv_dn4 * assign17650_e12202) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn4) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17650_e12197 * locals.var_nsub_dn4)) * locals.var_nin) - (assign17650_e12199 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign17650_e12201))), ((locals.var_beta_inv_dn5 * assign17650_e12202) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn5) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17650_e12197 * locals.var_nsub_dn5)) * locals.var_nin) - (assign17650_e12199 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign17650_e12201))), ((locals.var_beta_inv_dn6 * assign17650_e12202) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn6) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17650_e12197 * locals.var_nsub_dn6)) * locals.var_nin) - (assign17650_e12199 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign17650_e12201))), ((locals.var_beta_inv_dn7 * assign17650_e12202) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn7) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17650_e12197 * locals.var_nsub_dn7)) * locals.var_nin) - (assign17650_e12199 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign17650_e12201))), ((locals.var_beta_inv_dn8 * assign17650_e12202) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn8) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17650_e12197 * locals.var_nsub_dn8)) * locals.var_nin) - (assign17650_e12199 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign17650_e12201))), ((locals.var_beta_inv_dn9 * assign17650_e12202) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn9) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17650_e12197 * locals.var_nsub_dn9)) * locals.var_nin) - (assign17650_e12199 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign17650_e12201))), ((locals.var_beta_inv_dn10 * assign17650_e12202) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn10) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17650_e12197 * locals.var_nsub_dn10)) * locals.var_nin) - (assign17650_e12199 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign17650_e12201))), ((locals.var_beta_inv_dn13 * assign17650_e12202) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn13) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign17650_e12197 * locals.var_nsub_dn13)) * locals.var_nin) - (assign17650_e12199 * locals.var_nin_dn13)) / (locals.var_nin * locals.var_nin)) / assign17650_e12201))),)
    } else {
        (locals.var_vbipn, locals.var_vbipn_dn0, locals.var_vbipn_dn2, locals.var_vbipn_dn4, locals.var_vbipn_dn5, locals.var_vbipn_dn6, locals.var_vbipn_dn7, locals.var_vbipn_dn8, locals.var_vbipn_dn9, locals.var_vbipn_dn10, locals.var_vbipn_dn13,)
    }
};
        locals.var_vbipn = assign17650_e12205;
        locals.var_vbipn_dn0 = assign17650_e12205_d_n0;
        locals.var_vbipn_dn2 = assign17650_e12205_d_n2;
        locals.var_vbipn_dn4 = assign17650_e12205_d_n4;
        locals.var_vbipn_dn5 = assign17650_e12205_d_n5;
        locals.var_vbipn_dn6 = assign17650_e12205_d_n6;
        locals.var_vbipn_dn7 = assign17650_e12205_d_n7;
        locals.var_vbipn_dn8 = assign17650_e12205_d_n8;
        locals.var_vbipn_dn9 = assign17650_e12205_d_n9;
        locals.var_vbipn_dn10 = assign17650_e12205_d_n10;
        locals.var_vbipn_dn13 = assign17650_e12205_d_n13;
        locals.var_vbipn_rv = 0.0;

        let (assign17660_e12215, assign17660_e12215_d_n0, assign17660_e12215_d_n2, assign17660_e12215_d_n4, assign17660_e12215_d_n5, assign17660_e12215_d_n6, assign17660_e12215_d_n7, assign17660_e12215_d_n8, assign17660_e12215_d_n9, assign17660_e12215_d_n10, assign17660_e12215_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard355 == 0.0)) && (locals.var_guard358 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_depmphn0, locals.var_depmphn0_dn0, locals.var_depmphn0_dn2, locals.var_depmphn0_dn4, locals.var_depmphn0_dn5, locals.var_depmphn0_dn6, locals.var_depmphn0_dn7, locals.var_depmphn0_dn8, locals.var_depmphn0_dn9, locals.var_depmphn0_dn10, locals.var_depmphn0_dn13,)
    }
};
        locals.var_depmphn0 = assign17660_e12215;
        locals.var_depmphn0_dn0 = assign17660_e12215_d_n0;
        locals.var_depmphn0_dn2 = assign17660_e12215_d_n2;
        locals.var_depmphn0_dn4 = assign17660_e12215_d_n4;
        locals.var_depmphn0_dn5 = assign17660_e12215_d_n5;
        locals.var_depmphn0_dn6 = assign17660_e12215_d_n6;
        locals.var_depmphn0_dn7 = assign17660_e12215_d_n7;
        locals.var_depmphn0_dn8 = assign17660_e12215_d_n8;
        locals.var_depmphn0_dn9 = assign17660_e12215_d_n9;
        locals.var_depmphn0_dn10 = assign17660_e12215_d_n10;
        locals.var_depmphn0_dn13 = assign17660_e12215_d_n13;
        locals.var_depmphn0_rv = 0.0;

        let (assign17670_e12221, assign17670_e12221_d_n0, assign17670_e12221_d_n2, assign17670_e12221_d_n4, assign17670_e12221_d_n5, assign17670_e12221_d_n6, assign17670_e12221_d_n7, assign17670_e12221_d_n8, assign17670_e12221_d_n9, assign17670_e12221_d_n10, assign17670_e12221_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17670_e12219: f64 = (locals.var_ptovr0 * locals.var_beta_inv);
        (assign17670_e12219, ((locals.var_ptovr0_dn0 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn0)), ((locals.var_ptovr0_dn2 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn2)), ((locals.var_ptovr0_dn4 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn4)), ((locals.var_ptovr0_dn5 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn5)), ((locals.var_ptovr0_dn6 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn6)), ((locals.var_ptovr0_dn7 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn7)), ((locals.var_ptovr0_dn8 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn8)), ((locals.var_ptovr0_dn9 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn9)), ((locals.var_ptovr0_dn10 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn10)), ((locals.var_ptovr0_dn13 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn13)),)
    } else {
        (locals.var_ptovr, locals.var_ptovr_dn0, locals.var_ptovr_dn2, locals.var_ptovr_dn4, locals.var_ptovr_dn5, locals.var_ptovr_dn6, locals.var_ptovr_dn7, locals.var_ptovr_dn8, locals.var_ptovr_dn9, locals.var_ptovr_dn10, locals.var_ptovr_dn13,)
    }
};
        locals.var_ptovr = assign17670_e12221;
        locals.var_ptovr_dn0 = assign17670_e12221_d_n0;
        locals.var_ptovr_dn2 = assign17670_e12221_d_n2;
        locals.var_ptovr_dn4 = assign17670_e12221_d_n4;
        locals.var_ptovr_dn5 = assign17670_e12221_d_n5;
        locals.var_ptovr_dn6 = assign17670_e12221_d_n6;
        locals.var_ptovr_dn7 = assign17670_e12221_d_n7;
        locals.var_ptovr_dn8 = assign17670_e12221_d_n8;
        locals.var_ptovr_dn9 = assign17670_e12221_d_n9;
        locals.var_ptovr_dn10 = assign17670_e12221_d_n10;
        locals.var_ptovr_dn13 = assign17670_e12221_d_n13;
        locals.var_ptovr_rv = 0.0;

        let (assign17680_e12227, assign17680_e12227_d_n0, assign17680_e12227_d_n2, assign17680_e12227_d_n4, assign17680_e12227_d_n5, assign17680_e12227_d_n6, assign17680_e12227_d_n7, assign17680_e12227_d_n8, assign17680_e12227_d_n9, assign17680_e12227_d_n10, assign17680_e12227_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17680_e12225: f64 = (locals.var_ttemp / locals.var_ktnom);
        (assign17680_e12225, (locals.var_ttemp_dn0 / locals.var_ktnom), (locals.var_ttemp_dn2 / locals.var_ktnom), (locals.var_ttemp_dn4 / locals.var_ktnom), (locals.var_ttemp_dn5 / locals.var_ktnom), (locals.var_ttemp_dn6 / locals.var_ktnom), (locals.var_ttemp_dn7 / locals.var_ktnom), (locals.var_ttemp_dn8 / locals.var_ktnom), (locals.var_ttemp_dn9 / locals.var_ktnom), (locals.var_ttemp_dn10 / locals.var_ktnom), (locals.var_ttemp_dn13 / locals.var_ktnom),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign17680_e12227;
        locals.var_t1_dn0 = assign17680_e12227_d_n0;
        locals.var_t1_dn2 = assign17680_e12227_d_n2;
        locals.var_t1_dn4 = assign17680_e12227_d_n4;
        locals.var_t1_dn5 = assign17680_e12227_d_n5;
        locals.var_t1_dn6 = assign17680_e12227_d_n6;
        locals.var_t1_dn7 = assign17680_e12227_d_n7;
        locals.var_t1_dn8 = assign17680_e12227_d_n8;
        locals.var_t1_dn9 = assign17680_e12227_d_n9;
        locals.var_t1_dn10 = assign17680_e12227_d_n10;
        locals.var_t1_dn13 = assign17680_e12227_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign17690_e12247, assign17690_e12247_d_n0, assign17690_e12247_d_n2, assign17690_e12247_d_n4, assign17690_e12247_d_n5, assign17690_e12247_d_n6, assign17690_e12247_d_n7, assign17690_e12247_d_n8, assign17690_e12247_d_n9, assign17690_e12247_d_n10, assign17690_e12247_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17690_e12232: f64 = (0.4 * locals.var_t1);
        let assign17690_e12233: f64 = (1.8 + assign17690_e12232);
        let assign17690_e12236: f64 = (0.1 * locals.var_t1);
        let assign17690_e12238: f64 = (assign17690_e12236 * locals.var_t1);
        let assign17690_e12239: f64 = (assign17690_e12233 + assign17690_e12238);
        let assign17690_e12243: f64 = (1.0 - locals.var_t1);
        let assign17690_e12244: f64 = (locals.var_uc_vtmp * assign17690_e12243);
        let assign17690_e12245: f64 = (assign17690_e12239 - assign17690_e12244);
        (assign17690_e12245, (((0.4 * locals.var_t1_dn0) + (((0.1 * locals.var_t1_dn0) * locals.var_t1) + (assign17690_e12236 * locals.var_t1_dn0))) - (locals.var_uc_vtmp * (-locals.var_t1_dn0))), (((0.4 * locals.var_t1_dn2) + (((0.1 * locals.var_t1_dn2) * locals.var_t1) + (assign17690_e12236 * locals.var_t1_dn2))) - (locals.var_uc_vtmp * (-locals.var_t1_dn2))), (((0.4 * locals.var_t1_dn4) + (((0.1 * locals.var_t1_dn4) * locals.var_t1) + (assign17690_e12236 * locals.var_t1_dn4))) - (locals.var_uc_vtmp * (-locals.var_t1_dn4))), (((0.4 * locals.var_t1_dn5) + (((0.1 * locals.var_t1_dn5) * locals.var_t1) + (assign17690_e12236 * locals.var_t1_dn5))) - (locals.var_uc_vtmp * (-locals.var_t1_dn5))), (((0.4 * locals.var_t1_dn6) + (((0.1 * locals.var_t1_dn6) * locals.var_t1) + (assign17690_e12236 * locals.var_t1_dn6))) - (locals.var_uc_vtmp * (-locals.var_t1_dn6))), (((0.4 * locals.var_t1_dn7) + (((0.1 * locals.var_t1_dn7) * locals.var_t1) + (assign17690_e12236 * locals.var_t1_dn7))) - (locals.var_uc_vtmp * (-locals.var_t1_dn7))), (((0.4 * locals.var_t1_dn8) + (((0.1 * locals.var_t1_dn8) * locals.var_t1) + (assign17690_e12236 * locals.var_t1_dn8))) - (locals.var_uc_vtmp * (-locals.var_t1_dn8))), (((0.4 * locals.var_t1_dn9) + (((0.1 * locals.var_t1_dn9) * locals.var_t1) + (assign17690_e12236 * locals.var_t1_dn9))) - (locals.var_uc_vtmp * (-locals.var_t1_dn9))), (((0.4 * locals.var_t1_dn10) + (((0.1 * locals.var_t1_dn10) * locals.var_t1) + (assign17690_e12236 * locals.var_t1_dn10))) - (locals.var_uc_vtmp * (-locals.var_t1_dn10))), (((0.4 * locals.var_t1_dn13) + (((0.1 * locals.var_t1_dn13) * locals.var_t1) + (assign17690_e12236 * locals.var_t1_dn13))) - (locals.var_uc_vtmp * (-locals.var_t1_dn13))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign17690_e12247;
        locals.var_t0_dn0 = assign17690_e12247_d_n0;
        locals.var_t0_dn2 = assign17690_e12247_d_n2;
        locals.var_t0_dn4 = assign17690_e12247_d_n4;
        locals.var_t0_dn5 = assign17690_e12247_d_n5;
        locals.var_t0_dn6 = assign17690_e12247_d_n6;
        locals.var_t0_dn7 = assign17690_e12247_d_n7;
        locals.var_t0_dn8 = assign17690_e12247_d_n8;
        locals.var_t0_dn9 = assign17690_e12247_d_n9;
        locals.var_t0_dn10 = assign17690_e12247_d_n10;
        locals.var_t0_dn13 = assign17690_e12247_d_n13;
        locals.var_t0_rv = 0.0;

        let assign17700_e12250: f64 = if p.p39 != 2.0 { 1.0 } else { 0.0 };
        locals.var_guard361 = assign17700_e12250;
        locals.var_guard361_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_40(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign17710_e12270, assign17710_e12270_d_n0, assign17710_e12270_d_n2, assign17710_e12270_d_n4, assign17710_e12270_d_n5, assign17710_e12270_d_n6, assign17710_e12270_d_n7, assign17710_e12270_d_n8, assign17710_e12270_d_n9, assign17710_e12270_d_n10, assign17710_e12270_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard361 != 0.0)) {
        let assign17710_e12256: f64 = (locals.var_vmax0 * locals.var_uc_vmax);
        let assign17710_e12258: f64 = (assign17710_e12256 / locals.var_t0);
        let assign17710_e12262: f64 = (p.p90 * locals.var_tdiff0);
        let assign17710_e12263: f64 = (1.0 + assign17710_e12262);
        let assign17710_e12266: f64 = (p.p91 * locals.var_tdiff0_2);
        let assign17710_e12267: f64 = (assign17710_e12263 + assign17710_e12266);
        let assign17710_e12268: f64 = (assign17710_e12258 * assign17710_e12267);
        (assign17710_e12268, (((-((assign17710_e12256 * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))) * assign17710_e12267) + (assign17710_e12258 * ((p.p90 * locals.var_tdiff0_dn0) + (p.p91 * locals.var_tdiff0_2_dn0)))), (((-((assign17710_e12256 * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))) * assign17710_e12267) + (assign17710_e12258 * ((p.p90 * locals.var_tdiff0_dn2) + (p.p91 * locals.var_tdiff0_2_dn2)))), (((-((assign17710_e12256 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))) * assign17710_e12267) + (assign17710_e12258 * ((p.p90 * locals.var_tdiff0_dn4) + (p.p91 * locals.var_tdiff0_2_dn4)))), (((-((assign17710_e12256 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))) * assign17710_e12267) + (assign17710_e12258 * ((p.p90 * locals.var_tdiff0_dn5) + (p.p91 * locals.var_tdiff0_2_dn5)))), (((-((assign17710_e12256 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))) * assign17710_e12267) + (assign17710_e12258 * ((p.p90 * locals.var_tdiff0_dn6) + (p.p91 * locals.var_tdiff0_2_dn6)))), (((-((assign17710_e12256 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))) * assign17710_e12267) + (assign17710_e12258 * ((p.p90 * locals.var_tdiff0_dn7) + (p.p91 * locals.var_tdiff0_2_dn7)))), (((-((assign17710_e12256 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))) * assign17710_e12267) + (assign17710_e12258 * ((p.p90 * locals.var_tdiff0_dn8) + (p.p91 * locals.var_tdiff0_2_dn8)))), (((-((assign17710_e12256 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))) * assign17710_e12267) + (assign17710_e12258 * ((p.p90 * locals.var_tdiff0_dn9) + (p.p91 * locals.var_tdiff0_2_dn9)))), (((-((assign17710_e12256 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))) * assign17710_e12267) + (assign17710_e12258 * ((p.p90 * locals.var_tdiff0_dn10) + (p.p91 * locals.var_tdiff0_2_dn10)))), (((-((assign17710_e12256 * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0))) * assign17710_e12267) + (assign17710_e12258 * ((p.p90 * locals.var_tdiff0_dn13) + (p.p91 * locals.var_tdiff0_2_dn13)))),)
    } else {
        (locals.var_vmaxeff, locals.var_vmaxeff_dn0, locals.var_vmaxeff_dn2, locals.var_vmaxeff_dn4, locals.var_vmaxeff_dn5, locals.var_vmaxeff_dn6, locals.var_vmaxeff_dn7, locals.var_vmaxeff_dn8, locals.var_vmaxeff_dn9, locals.var_vmaxeff_dn10, locals.var_vmaxeff_dn13,)
    }
};
        locals.var_vmaxeff = assign17710_e12270;
        locals.var_vmaxeff_dn0 = assign17710_e12270_d_n0;
        locals.var_vmaxeff_dn2 = assign17710_e12270_d_n2;
        locals.var_vmaxeff_dn4 = assign17710_e12270_d_n4;
        locals.var_vmaxeff_dn5 = assign17710_e12270_d_n5;
        locals.var_vmaxeff_dn6 = assign17710_e12270_d_n6;
        locals.var_vmaxeff_dn7 = assign17710_e12270_d_n7;
        locals.var_vmaxeff_dn8 = assign17710_e12270_d_n8;
        locals.var_vmaxeff_dn9 = assign17710_e12270_d_n9;
        locals.var_vmaxeff_dn10 = assign17710_e12270_d_n10;
        locals.var_vmaxeff_dn13 = assign17710_e12270_d_n13;
        locals.var_vmaxeff_rv = 0.0;

        let (assign17720_e12291, assign17720_e12291_d_n0, assign17720_e12291_d_n2, assign17720_e12291_d_n4, assign17720_e12291_d_n5, assign17720_e12291_d_n6, assign17720_e12291_d_n7, assign17720_e12291_d_n8, assign17720_e12291_d_n9, assign17720_e12291_d_n10, assign17720_e12291_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard361 == 0.0)) {
        let assign17720_e12277: f64 = (locals.var_vmax0 * locals.var_uc_vmax);
        let assign17720_e12279: f64 = (assign17720_e12277 / locals.var_t0);
        let assign17720_e12283: f64 = (p.p90 * locals.var_tdiff);
        let assign17720_e12284: f64 = (1.0 + assign17720_e12283);
        let assign17720_e12287: f64 = (p.p91 * locals.var_tdiff_2);
        let assign17720_e12288: f64 = (assign17720_e12284 + assign17720_e12287);
        let assign17720_e12289: f64 = (assign17720_e12279 * assign17720_e12288);
        (assign17720_e12289, (((-((assign17720_e12277 * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))) * assign17720_e12288) + (assign17720_e12279 * ((p.p90 * locals.var_tdiff_dn0) + (p.p91 * locals.var_tdiff_2_dn0)))), (((-((assign17720_e12277 * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))) * assign17720_e12288) + (assign17720_e12279 * ((p.p90 * locals.var_tdiff_dn2) + (p.p91 * locals.var_tdiff_2_dn2)))), (((-((assign17720_e12277 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))) * assign17720_e12288) + (assign17720_e12279 * ((p.p90 * locals.var_tdiff_dn4) + (p.p91 * locals.var_tdiff_2_dn4)))), (((-((assign17720_e12277 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))) * assign17720_e12288) + (assign17720_e12279 * ((p.p90 * locals.var_tdiff_dn5) + (p.p91 * locals.var_tdiff_2_dn5)))), (((-((assign17720_e12277 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))) * assign17720_e12288) + (assign17720_e12279 * ((p.p90 * locals.var_tdiff_dn6) + (p.p91 * locals.var_tdiff_2_dn6)))), (((-((assign17720_e12277 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))) * assign17720_e12288) + (assign17720_e12279 * ((p.p90 * locals.var_tdiff_dn7) + (p.p91 * locals.var_tdiff_2_dn7)))), (((-((assign17720_e12277 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))) * assign17720_e12288) + (assign17720_e12279 * ((p.p90 * locals.var_tdiff_dn8) + (p.p91 * locals.var_tdiff_2_dn8)))), (((-((assign17720_e12277 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))) * assign17720_e12288) + (assign17720_e12279 * ((p.p90 * locals.var_tdiff_dn9) + (p.p91 * locals.var_tdiff_2_dn9)))), (((-((assign17720_e12277 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))) * assign17720_e12288) + (assign17720_e12279 * ((p.p90 * locals.var_tdiff_dn10) + (p.p91 * locals.var_tdiff_2_dn10)))), (((-((assign17720_e12277 * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0))) * assign17720_e12288) + (assign17720_e12279 * ((p.p90 * locals.var_tdiff_dn13) + (p.p91 * locals.var_tdiff_2_dn13)))),)
    } else {
        (locals.var_vmaxeff, locals.var_vmaxeff_dn0, locals.var_vmaxeff_dn2, locals.var_vmaxeff_dn4, locals.var_vmaxeff_dn5, locals.var_vmaxeff_dn6, locals.var_vmaxeff_dn7, locals.var_vmaxeff_dn8, locals.var_vmaxeff_dn9, locals.var_vmaxeff_dn10, locals.var_vmaxeff_dn13,)
    }
};
        locals.var_vmaxeff = assign17720_e12291;
        locals.var_vmaxeff_dn0 = assign17720_e12291_d_n0;
        locals.var_vmaxeff_dn2 = assign17720_e12291_d_n2;
        locals.var_vmaxeff_dn4 = assign17720_e12291_d_n4;
        locals.var_vmaxeff_dn5 = assign17720_e12291_d_n5;
        locals.var_vmaxeff_dn6 = assign17720_e12291_d_n6;
        locals.var_vmaxeff_dn7 = assign17720_e12291_d_n7;
        locals.var_vmaxeff_dn8 = assign17720_e12291_d_n8;
        locals.var_vmaxeff_dn9 = assign17720_e12291_d_n9;
        locals.var_vmaxeff_dn10 = assign17720_e12291_d_n10;
        locals.var_vmaxeff_dn13 = assign17720_e12291_d_n13;
        locals.var_vmaxeff_rv = 0.0;

        let assign17740_e12299: f64 = if p.p39 != 2.0 { 1.0 } else { 0.0 };
        locals.var_guard363 = assign17740_e12299;
        locals.var_guard363_rv = 0.0;

        let (assign17750_e12315, assign17750_e12315_d_n0, assign17750_e12315_d_n2, assign17750_e12315_d_n4, assign17750_e12315_d_n5, assign17750_e12315_d_n6, assign17750_e12315_d_n7, assign17750_e12315_d_n8, assign17750_e12315_d_n9, assign17750_e12315_d_n10, assign17750_e12315_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard363 != 0.0)) {
        let assign17750_e12307: f64 = (p.p324 * locals.var_tdiff0);
        let assign17750_e12308: f64 = (1.0 + assign17750_e12307);
        let assign17750_e12311: f64 = (p.p325 * locals.var_tdiff0_2);
        let assign17750_e12312: f64 = (assign17750_e12308 + assign17750_e12311);
        let assign17750_e12313: f64 = (locals.var_ninvd0 * assign17750_e12312);
        (assign17750_e12313, (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn0) + (p.p325 * locals.var_tdiff0_2_dn0))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn2) + (p.p325 * locals.var_tdiff0_2_dn2))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn4) + (p.p325 * locals.var_tdiff0_2_dn4))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn5) + (p.p325 * locals.var_tdiff0_2_dn5))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn6) + (p.p325 * locals.var_tdiff0_2_dn6))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn7) + (p.p325 * locals.var_tdiff0_2_dn7))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn8) + (p.p325 * locals.var_tdiff0_2_dn8))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn9) + (p.p325 * locals.var_tdiff0_2_dn9))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn10) + (p.p325 * locals.var_tdiff0_2_dn10))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn13) + (p.p325 * locals.var_tdiff0_2_dn13))),)
    } else {
        (locals.var_ninvde, locals.var_ninvde_dn0, locals.var_ninvde_dn2, locals.var_ninvde_dn4, locals.var_ninvde_dn5, locals.var_ninvde_dn6, locals.var_ninvde_dn7, locals.var_ninvde_dn8, locals.var_ninvde_dn9, locals.var_ninvde_dn10, locals.var_ninvde_dn13,)
    }
};
        locals.var_ninvde = assign17750_e12315;
        locals.var_ninvde_dn0 = assign17750_e12315_d_n0;
        locals.var_ninvde_dn2 = assign17750_e12315_d_n2;
        locals.var_ninvde_dn4 = assign17750_e12315_d_n4;
        locals.var_ninvde_dn5 = assign17750_e12315_d_n5;
        locals.var_ninvde_dn6 = assign17750_e12315_d_n6;
        locals.var_ninvde_dn7 = assign17750_e12315_d_n7;
        locals.var_ninvde_dn8 = assign17750_e12315_d_n8;
        locals.var_ninvde_dn9 = assign17750_e12315_d_n9;
        locals.var_ninvde_dn10 = assign17750_e12315_d_n10;
        locals.var_ninvde_dn13 = assign17750_e12315_d_n13;
        locals.var_ninvde_rv = 0.0;

        let (assign17760_e12329, assign17760_e12329_d_n0, assign17760_e12329_d_n2, assign17760_e12329_d_n4, assign17760_e12329_d_n5, assign17760_e12329_d_n6, assign17760_e12329_d_n7, assign17760_e12329_d_n8, assign17760_e12329_d_n9, assign17760_e12329_d_n10, assign17760_e12329_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard363 != 0.0)) {
        let assign17760_e12322: f64 = (p.p390 * locals.var_tdiff0);
        let assign17760_e12323: f64 = (1.0 + assign17760_e12322);
        let assign17760_e12326: f64 = (p.p391 * locals.var_tdiff0_2);
        let assign17760_e12327: f64 = (assign17760_e12323 + assign17760_e12326);
        (assign17760_e12327, ((p.p390 * locals.var_tdiff0_dn0) + (p.p391 * locals.var_tdiff0_2_dn0)), ((p.p390 * locals.var_tdiff0_dn2) + (p.p391 * locals.var_tdiff0_2_dn2)), ((p.p390 * locals.var_tdiff0_dn4) + (p.p391 * locals.var_tdiff0_2_dn4)), ((p.p390 * locals.var_tdiff0_dn5) + (p.p391 * locals.var_tdiff0_2_dn5)), ((p.p390 * locals.var_tdiff0_dn6) + (p.p391 * locals.var_tdiff0_2_dn6)), ((p.p390 * locals.var_tdiff0_dn7) + (p.p391 * locals.var_tdiff0_2_dn7)), ((p.p390 * locals.var_tdiff0_dn8) + (p.p391 * locals.var_tdiff0_2_dn8)), ((p.p390 * locals.var_tdiff0_dn9) + (p.p391 * locals.var_tdiff0_2_dn9)), ((p.p390 * locals.var_tdiff0_dn10) + (p.p391 * locals.var_tdiff0_2_dn10)), ((p.p390 * locals.var_tdiff0_dn13) + (p.p391 * locals.var_tdiff0_2_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign17760_e12329;
        locals.var_t1_dn0 = assign17760_e12329_d_n0;
        locals.var_t1_dn2 = assign17760_e12329_d_n2;
        locals.var_t1_dn4 = assign17760_e12329_d_n4;
        locals.var_t1_dn5 = assign17760_e12329_d_n5;
        locals.var_t1_dn6 = assign17760_e12329_d_n6;
        locals.var_t1_dn7 = assign17760_e12329_d_n7;
        locals.var_t1_dn8 = assign17760_e12329_d_n8;
        locals.var_t1_dn9 = assign17760_e12329_d_n9;
        locals.var_t1_dn10 = assign17760_e12329_d_n10;
        locals.var_t1_dn13 = assign17760_e12329_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign17770_e12337, assign17770_e12337_d_n0, assign17770_e12337_d_n2, assign17770_e12337_d_n4, assign17770_e12337_d_n5, assign17770_e12337_d_n6, assign17770_e12337_d_n7, assign17770_e12337_d_n8, assign17770_e12337_d_n9, assign17770_e12337_d_n10, assign17770_e12337_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard363 != 0.0)) {
        let assign17770_e12335: f64 = (locals.var_ninvd0cres * locals.var_t1);
        (assign17770_e12335, ((locals.var_ninvd0cres_dn0 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn0)), ((locals.var_ninvd0cres_dn2 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn2)), ((locals.var_ninvd0cres_dn4 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn4)), ((locals.var_ninvd0cres_dn5 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn5)), ((locals.var_ninvd0cres_dn6 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn6)), ((locals.var_ninvd0cres_dn7 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn7)), ((locals.var_ninvd0cres_dn8 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn8)), ((locals.var_ninvd0cres_dn9 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn9)), ((locals.var_ninvd0cres_dn10 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn10)), ((locals.var_ninvd0cres_dn13 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn13)),)
    } else {
        (locals.var_ninvdecres, locals.var_ninvdecres_dn0, locals.var_ninvdecres_dn2, locals.var_ninvdecres_dn4, locals.var_ninvdecres_dn5, locals.var_ninvdecres_dn6, locals.var_ninvdecres_dn7, locals.var_ninvdecres_dn8, locals.var_ninvdecres_dn9, locals.var_ninvdecres_dn10, locals.var_ninvdecres_dn13,)
    }
};
        locals.var_ninvdecres = assign17770_e12337;
        locals.var_ninvdecres_dn0 = assign17770_e12337_d_n0;
        locals.var_ninvdecres_dn2 = assign17770_e12337_d_n2;
        locals.var_ninvdecres_dn4 = assign17770_e12337_d_n4;
        locals.var_ninvdecres_dn5 = assign17770_e12337_d_n5;
        locals.var_ninvdecres_dn6 = assign17770_e12337_d_n6;
        locals.var_ninvdecres_dn7 = assign17770_e12337_d_n7;
        locals.var_ninvdecres_dn8 = assign17770_e12337_d_n8;
        locals.var_ninvdecres_dn9 = assign17770_e12337_d_n9;
        locals.var_ninvdecres_dn10 = assign17770_e12337_d_n10;
        locals.var_ninvdecres_dn13 = assign17770_e12337_d_n13;
        locals.var_ninvdecres_rv = 0.0;

        let (assign17780_e12345, assign17780_e12345_d_n0, assign17780_e12345_d_n2, assign17780_e12345_d_n4, assign17780_e12345_d_n5, assign17780_e12345_d_n6, assign17780_e12345_d_n7, assign17780_e12345_d_n8, assign17780_e12345_d_n9, assign17780_e12345_d_n10, assign17780_e12345_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard363 != 0.0)) {
        let assign17780_e12343: f64 = (locals.var_ninvd0hres * locals.var_t1);
        (assign17780_e12343, ((locals.var_ninvd0hres_dn0 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn0)), ((locals.var_ninvd0hres_dn2 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn2)), ((locals.var_ninvd0hres_dn4 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn4)), ((locals.var_ninvd0hres_dn5 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn5)), ((locals.var_ninvd0hres_dn6 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn6)), ((locals.var_ninvd0hres_dn7 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn7)), ((locals.var_ninvd0hres_dn8 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn8)), ((locals.var_ninvd0hres_dn9 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn9)), ((locals.var_ninvd0hres_dn10 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn10)), ((locals.var_ninvd0hres_dn13 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn13)),)
    } else {
        (locals.var_ninvdehres, locals.var_ninvdehres_dn0, locals.var_ninvdehres_dn2, locals.var_ninvdehres_dn4, locals.var_ninvdehres_dn5, locals.var_ninvdehres_dn6, locals.var_ninvdehres_dn7, locals.var_ninvdehres_dn8, locals.var_ninvdehres_dn9, locals.var_ninvdehres_dn10, locals.var_ninvdehres_dn13,)
    }
};
        locals.var_ninvdehres = assign17780_e12345;
        locals.var_ninvdehres_dn0 = assign17780_e12345_d_n0;
        locals.var_ninvdehres_dn2 = assign17780_e12345_d_n2;
        locals.var_ninvdehres_dn4 = assign17780_e12345_d_n4;
        locals.var_ninvdehres_dn5 = assign17780_e12345_d_n5;
        locals.var_ninvdehres_dn6 = assign17780_e12345_d_n6;
        locals.var_ninvdehres_dn7 = assign17780_e12345_d_n7;
        locals.var_ninvdehres_dn8 = assign17780_e12345_d_n8;
        locals.var_ninvdehres_dn9 = assign17780_e12345_d_n9;
        locals.var_ninvdehres_dn10 = assign17780_e12345_d_n10;
        locals.var_ninvdehres_dn13 = assign17780_e12345_d_n13;
        locals.var_ninvdehres_rv = 0.0;

        let (assign17790_e12362, assign17790_e12362_d_n0, assign17790_e12362_d_n2, assign17790_e12362_d_n4, assign17790_e12362_d_n5, assign17790_e12362_d_n6, assign17790_e12362_d_n7, assign17790_e12362_d_n8, assign17790_e12362_d_n9, assign17790_e12362_d_n10, assign17790_e12362_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard363 == 0.0)) {
        let assign17790_e12354: f64 = (p.p324 * locals.var_tdiff);
        let assign17790_e12355: f64 = (1.0 + assign17790_e12354);
        let assign17790_e12358: f64 = (p.p325 * locals.var_tdiff_2);
        let assign17790_e12359: f64 = (assign17790_e12355 + assign17790_e12358);
        let assign17790_e12360: f64 = (locals.var_ninvd0 * assign17790_e12359);
        (assign17790_e12360, (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn0) + (p.p325 * locals.var_tdiff_2_dn0))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn2) + (p.p325 * locals.var_tdiff_2_dn2))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn4) + (p.p325 * locals.var_tdiff_2_dn4))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn5) + (p.p325 * locals.var_tdiff_2_dn5))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn6) + (p.p325 * locals.var_tdiff_2_dn6))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn7) + (p.p325 * locals.var_tdiff_2_dn7))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn8) + (p.p325 * locals.var_tdiff_2_dn8))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn9) + (p.p325 * locals.var_tdiff_2_dn9))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn10) + (p.p325 * locals.var_tdiff_2_dn10))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn13) + (p.p325 * locals.var_tdiff_2_dn13))),)
    } else {
        (locals.var_ninvde, locals.var_ninvde_dn0, locals.var_ninvde_dn2, locals.var_ninvde_dn4, locals.var_ninvde_dn5, locals.var_ninvde_dn6, locals.var_ninvde_dn7, locals.var_ninvde_dn8, locals.var_ninvde_dn9, locals.var_ninvde_dn10, locals.var_ninvde_dn13,)
    }
};
        locals.var_ninvde = assign17790_e12362;
        locals.var_ninvde_dn0 = assign17790_e12362_d_n0;
        locals.var_ninvde_dn2 = assign17790_e12362_d_n2;
        locals.var_ninvde_dn4 = assign17790_e12362_d_n4;
        locals.var_ninvde_dn5 = assign17790_e12362_d_n5;
        locals.var_ninvde_dn6 = assign17790_e12362_d_n6;
        locals.var_ninvde_dn7 = assign17790_e12362_d_n7;
        locals.var_ninvde_dn8 = assign17790_e12362_d_n8;
        locals.var_ninvde_dn9 = assign17790_e12362_d_n9;
        locals.var_ninvde_dn10 = assign17790_e12362_d_n10;
        locals.var_ninvde_dn13 = assign17790_e12362_d_n13;
        locals.var_ninvde_rv = 0.0;

        let (assign17800_e12377, assign17800_e12377_d_n0, assign17800_e12377_d_n2, assign17800_e12377_d_n4, assign17800_e12377_d_n5, assign17800_e12377_d_n6, assign17800_e12377_d_n7, assign17800_e12377_d_n8, assign17800_e12377_d_n9, assign17800_e12377_d_n10, assign17800_e12377_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard363 == 0.0)) {
        let assign17800_e12370: f64 = (p.p390 * locals.var_tdiff);
        let assign17800_e12371: f64 = (1.0 + assign17800_e12370);
        let assign17800_e12374: f64 = (p.p391 * locals.var_tdiff_2);
        let assign17800_e12375: f64 = (assign17800_e12371 + assign17800_e12374);
        (assign17800_e12375, ((p.p390 * locals.var_tdiff_dn0) + (p.p391 * locals.var_tdiff_2_dn0)), ((p.p390 * locals.var_tdiff_dn2) + (p.p391 * locals.var_tdiff_2_dn2)), ((p.p390 * locals.var_tdiff_dn4) + (p.p391 * locals.var_tdiff_2_dn4)), ((p.p390 * locals.var_tdiff_dn5) + (p.p391 * locals.var_tdiff_2_dn5)), ((p.p390 * locals.var_tdiff_dn6) + (p.p391 * locals.var_tdiff_2_dn6)), ((p.p390 * locals.var_tdiff_dn7) + (p.p391 * locals.var_tdiff_2_dn7)), ((p.p390 * locals.var_tdiff_dn8) + (p.p391 * locals.var_tdiff_2_dn8)), ((p.p390 * locals.var_tdiff_dn9) + (p.p391 * locals.var_tdiff_2_dn9)), ((p.p390 * locals.var_tdiff_dn10) + (p.p391 * locals.var_tdiff_2_dn10)), ((p.p390 * locals.var_tdiff_dn13) + (p.p391 * locals.var_tdiff_2_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign17800_e12377;
        locals.var_t1_dn0 = assign17800_e12377_d_n0;
        locals.var_t1_dn2 = assign17800_e12377_d_n2;
        locals.var_t1_dn4 = assign17800_e12377_d_n4;
        locals.var_t1_dn5 = assign17800_e12377_d_n5;
        locals.var_t1_dn6 = assign17800_e12377_d_n6;
        locals.var_t1_dn7 = assign17800_e12377_d_n7;
        locals.var_t1_dn8 = assign17800_e12377_d_n8;
        locals.var_t1_dn9 = assign17800_e12377_d_n9;
        locals.var_t1_dn10 = assign17800_e12377_d_n10;
        locals.var_t1_dn13 = assign17800_e12377_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign17810_e12386, assign17810_e12386_d_n0, assign17810_e12386_d_n2, assign17810_e12386_d_n4, assign17810_e12386_d_n5, assign17810_e12386_d_n6, assign17810_e12386_d_n7, assign17810_e12386_d_n8, assign17810_e12386_d_n9, assign17810_e12386_d_n10, assign17810_e12386_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard363 == 0.0)) {
        let assign17810_e12384: f64 = (locals.var_ninvd0cres * locals.var_t1);
        (assign17810_e12384, ((locals.var_ninvd0cres_dn0 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn0)), ((locals.var_ninvd0cres_dn2 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn2)), ((locals.var_ninvd0cres_dn4 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn4)), ((locals.var_ninvd0cres_dn5 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn5)), ((locals.var_ninvd0cres_dn6 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn6)), ((locals.var_ninvd0cres_dn7 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn7)), ((locals.var_ninvd0cres_dn8 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn8)), ((locals.var_ninvd0cres_dn9 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn9)), ((locals.var_ninvd0cres_dn10 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn10)), ((locals.var_ninvd0cres_dn13 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn13)),)
    } else {
        (locals.var_ninvdecres, locals.var_ninvdecres_dn0, locals.var_ninvdecres_dn2, locals.var_ninvdecres_dn4, locals.var_ninvdecres_dn5, locals.var_ninvdecres_dn6, locals.var_ninvdecres_dn7, locals.var_ninvdecres_dn8, locals.var_ninvdecres_dn9, locals.var_ninvdecres_dn10, locals.var_ninvdecres_dn13,)
    }
};
        locals.var_ninvdecres = assign17810_e12386;
        locals.var_ninvdecres_dn0 = assign17810_e12386_d_n0;
        locals.var_ninvdecres_dn2 = assign17810_e12386_d_n2;
        locals.var_ninvdecres_dn4 = assign17810_e12386_d_n4;
        locals.var_ninvdecres_dn5 = assign17810_e12386_d_n5;
        locals.var_ninvdecres_dn6 = assign17810_e12386_d_n6;
        locals.var_ninvdecres_dn7 = assign17810_e12386_d_n7;
        locals.var_ninvdecres_dn8 = assign17810_e12386_d_n8;
        locals.var_ninvdecres_dn9 = assign17810_e12386_d_n9;
        locals.var_ninvdecres_dn10 = assign17810_e12386_d_n10;
        locals.var_ninvdecres_dn13 = assign17810_e12386_d_n13;
        locals.var_ninvdecres_rv = 0.0;

        let (assign17820_e12395, assign17820_e12395_d_n0, assign17820_e12395_d_n2, assign17820_e12395_d_n4, assign17820_e12395_d_n5, assign17820_e12395_d_n6, assign17820_e12395_d_n7, assign17820_e12395_d_n8, assign17820_e12395_d_n9, assign17820_e12395_d_n10, assign17820_e12395_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard363 == 0.0)) {
        let assign17820_e12393: f64 = (locals.var_ninvd0hres * locals.var_t1);
        (assign17820_e12393, ((locals.var_ninvd0hres_dn0 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn0)), ((locals.var_ninvd0hres_dn2 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn2)), ((locals.var_ninvd0hres_dn4 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn4)), ((locals.var_ninvd0hres_dn5 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn5)), ((locals.var_ninvd0hres_dn6 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn6)), ((locals.var_ninvd0hres_dn7 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn7)), ((locals.var_ninvd0hres_dn8 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn8)), ((locals.var_ninvd0hres_dn9 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn9)), ((locals.var_ninvd0hres_dn10 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn10)), ((locals.var_ninvd0hres_dn13 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn13)),)
    } else {
        (locals.var_ninvdehres, locals.var_ninvdehres_dn0, locals.var_ninvdehres_dn2, locals.var_ninvdehres_dn4, locals.var_ninvdehres_dn5, locals.var_ninvdehres_dn6, locals.var_ninvdehres_dn7, locals.var_ninvdehres_dn8, locals.var_ninvdehres_dn9, locals.var_ninvdehres_dn10, locals.var_ninvdehres_dn13,)
    }
};
        locals.var_ninvdehres = assign17820_e12395;
        locals.var_ninvdehres_dn0 = assign17820_e12395_d_n0;
        locals.var_ninvdehres_dn2 = assign17820_e12395_d_n2;
        locals.var_ninvdehres_dn4 = assign17820_e12395_d_n4;
        locals.var_ninvdehres_dn5 = assign17820_e12395_d_n5;
        locals.var_ninvdehres_dn6 = assign17820_e12395_d_n6;
        locals.var_ninvdehres_dn7 = assign17820_e12395_d_n7;
        locals.var_ninvdehres_dn8 = assign17820_e12395_d_n8;
        locals.var_ninvdehres_dn9 = assign17820_e12395_d_n9;
        locals.var_ninvdehres_dn10 = assign17820_e12395_d_n10;
        locals.var_ninvdehres_dn13 = assign17820_e12395_d_n13;
        locals.var_ninvdehres_rv = 0.0;

        let assign17840_e12403: f64 = if locals.var_ninvde < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard365 = assign17840_e12403;
        locals.var_guard365_rv = 0.0;

        let (assign17850_e12409, assign17850_e12409_d_n0, assign17850_e12409_d_n2, assign17850_e12409_d_n4, assign17850_e12409_d_n5, assign17850_e12409_d_n6, assign17850_e12409_d_n7, assign17850_e12409_d_n8, assign17850_e12409_d_n9, assign17850_e12409_d_n10, assign17850_e12409_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard365 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ninvde, locals.var_ninvde_dn0, locals.var_ninvde_dn2, locals.var_ninvde_dn4, locals.var_ninvde_dn5, locals.var_ninvde_dn6, locals.var_ninvde_dn7, locals.var_ninvde_dn8, locals.var_ninvde_dn9, locals.var_ninvde_dn10, locals.var_ninvde_dn13,)
    }
};
        locals.var_ninvde = assign17850_e12409;
        locals.var_ninvde_dn0 = assign17850_e12409_d_n0;
        locals.var_ninvde_dn2 = assign17850_e12409_d_n2;
        locals.var_ninvde_dn4 = assign17850_e12409_d_n4;
        locals.var_ninvde_dn5 = assign17850_e12409_d_n5;
        locals.var_ninvde_dn6 = assign17850_e12409_d_n6;
        locals.var_ninvde_dn7 = assign17850_e12409_d_n7;
        locals.var_ninvde_dn8 = assign17850_e12409_d_n8;
        locals.var_ninvde_dn9 = assign17850_e12409_d_n9;
        locals.var_ninvde_dn10 = assign17850_e12409_d_n10;
        locals.var_ninvde_dn13 = assign17850_e12409_d_n13;
        locals.var_ninvde_rv = 0.0;

        let assign17870_e12417: f64 = if locals.var_ninvdecres < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard367 = assign17870_e12417;
        locals.var_guard367_rv = 0.0;

        let (assign17880_e12423, assign17880_e12423_d_n0, assign17880_e12423_d_n2, assign17880_e12423_d_n4, assign17880_e12423_d_n5, assign17880_e12423_d_n6, assign17880_e12423_d_n7, assign17880_e12423_d_n8, assign17880_e12423_d_n9, assign17880_e12423_d_n10, assign17880_e12423_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard367 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ninvdecres, locals.var_ninvdecres_dn0, locals.var_ninvdecres_dn2, locals.var_ninvdecres_dn4, locals.var_ninvdecres_dn5, locals.var_ninvdecres_dn6, locals.var_ninvdecres_dn7, locals.var_ninvdecres_dn8, locals.var_ninvdecres_dn9, locals.var_ninvdecres_dn10, locals.var_ninvdecres_dn13,)
    }
};
        locals.var_ninvdecres = assign17880_e12423;
        locals.var_ninvdecres_dn0 = assign17880_e12423_d_n0;
        locals.var_ninvdecres_dn2 = assign17880_e12423_d_n2;
        locals.var_ninvdecres_dn4 = assign17880_e12423_d_n4;
        locals.var_ninvdecres_dn5 = assign17880_e12423_d_n5;
        locals.var_ninvdecres_dn6 = assign17880_e12423_d_n6;
        locals.var_ninvdecres_dn7 = assign17880_e12423_d_n7;
        locals.var_ninvdecres_dn8 = assign17880_e12423_d_n8;
        locals.var_ninvdecres_dn9 = assign17880_e12423_d_n9;
        locals.var_ninvdecres_dn10 = assign17880_e12423_d_n10;
        locals.var_ninvdecres_dn13 = assign17880_e12423_d_n13;
        locals.var_ninvdecres_rv = 0.0;

        let assign17900_e12431: f64 = if locals.var_ninvdehres < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard369 = assign17900_e12431;
        locals.var_guard369_rv = 0.0;

        let (assign17910_e12437, assign17910_e12437_d_n0, assign17910_e12437_d_n2, assign17910_e12437_d_n4, assign17910_e12437_d_n5, assign17910_e12437_d_n6, assign17910_e12437_d_n7, assign17910_e12437_d_n8, assign17910_e12437_d_n9, assign17910_e12437_d_n10, assign17910_e12437_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard369 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ninvdehres, locals.var_ninvdehres_dn0, locals.var_ninvdehres_dn2, locals.var_ninvdehres_dn4, locals.var_ninvdehres_dn5, locals.var_ninvdehres_dn6, locals.var_ninvdehres_dn7, locals.var_ninvdehres_dn8, locals.var_ninvdehres_dn9, locals.var_ninvdehres_dn10, locals.var_ninvdehres_dn13,)
    }
};
        locals.var_ninvdehres = assign17910_e12437;
        locals.var_ninvdehres_dn0 = assign17910_e12437_d_n0;
        locals.var_ninvdehres_dn2 = assign17910_e12437_d_n2;
        locals.var_ninvdehres_dn4 = assign17910_e12437_d_n4;
        locals.var_ninvdehres_dn5 = assign17910_e12437_d_n5;
        locals.var_ninvdehres_dn6 = assign17910_e12437_d_n6;
        locals.var_ninvdehres_dn7 = assign17910_e12437_d_n7;
        locals.var_ninvdehres_dn8 = assign17910_e12437_d_n8;
        locals.var_ninvdehres_dn9 = assign17910_e12437_d_n9;
        locals.var_ninvdehres_dn10 = assign17910_e12437_d_n10;
        locals.var_ninvdehres_dn13 = assign17910_e12437_d_n13;
        locals.var_ninvdehres_rv = 0.0;

        let (assign17920_e12453, assign17920_e12453_d_n0, assign17920_e12453_d_n2, assign17920_e12453_d_n4, assign17920_e12453_d_n5, assign17920_e12453_d_n6, assign17920_e12453_d_n7, assign17920_e12453_d_n8, assign17920_e12453_d_n9, assign17920_e12453_d_n10, assign17920_e12453_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (p.p53 != 0.0)) {
        let assign17920_e12444: f64 = (p.p328 * locals.var_tdiff0);
        let assign17920_e12445: f64 = (locals.var_uc_rth0 + assign17920_e12444);
        let assign17920_e12448: f64 = (p.p329 * locals.var_tdiff0_2);
        let assign17920_e12449: f64 = (assign17920_e12445 + assign17920_e12448);
        let assign17920_e12451: f64 = (assign17920_e12449 * locals.var_rthtemp0);
        (assign17920_e12451, (((p.p328 * locals.var_tdiff0_dn0) + (p.p329 * locals.var_tdiff0_2_dn0)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn2) + (p.p329 * locals.var_tdiff0_2_dn2)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn4) + (p.p329 * locals.var_tdiff0_2_dn4)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn5) + (p.p329 * locals.var_tdiff0_2_dn5)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn6) + (p.p329 * locals.var_tdiff0_2_dn6)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn7) + (p.p329 * locals.var_tdiff0_2_dn7)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn8) + (p.p329 * locals.var_tdiff0_2_dn8)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn9) + (p.p329 * locals.var_tdiff0_2_dn9)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn10) + (p.p329 * locals.var_tdiff0_2_dn10)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn13) + (p.p329 * locals.var_tdiff0_2_dn13)) * locals.var_rthtemp0),)
    } else {
        (locals.var_rth, locals.var_rth_dn0, locals.var_rth_dn2, locals.var_rth_dn4, locals.var_rth_dn5, locals.var_rth_dn6, locals.var_rth_dn7, locals.var_rth_dn8, locals.var_rth_dn9, locals.var_rth_dn10, locals.var_rth_dn13,)
    }
};
        locals.var_rth = assign17920_e12453;
        locals.var_rth_dn0 = assign17920_e12453_d_n0;
        locals.var_rth_dn2 = assign17920_e12453_d_n2;
        locals.var_rth_dn4 = assign17920_e12453_d_n4;
        locals.var_rth_dn5 = assign17920_e12453_d_n5;
        locals.var_rth_dn6 = assign17920_e12453_d_n6;
        locals.var_rth_dn7 = assign17920_e12453_d_n7;
        locals.var_rth_dn8 = assign17920_e12453_d_n8;
        locals.var_rth_dn9 = assign17920_e12453_d_n9;
        locals.var_rth_dn10 = assign17920_e12453_d_n10;
        locals.var_rth_dn13 = assign17920_e12453_d_n13;
        locals.var_rth_rv = 0.0;

        let assign17940_e12461: f64 = if locals.var_rth < 0.0001 { 1.0 } else { 0.0 };
        locals.var_guard371 = assign17940_e12461;
        locals.var_guard371_rv = 0.0;

        let (assign17950_e12469, assign17950_e12469_d_n0, assign17950_e12469_d_n2, assign17950_e12469_d_n4, assign17950_e12469_d_n5, assign17950_e12469_d_n6, assign17950_e12469_d_n7, assign17950_e12469_d_n8, assign17950_e12469_d_n9, assign17950_e12469_d_n10, assign17950_e12469_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (p.p53 != 0.0)) && (locals.var_guard371 != 0.0)) {
        (0.0001, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rth, locals.var_rth_dn0, locals.var_rth_dn2, locals.var_rth_dn4, locals.var_rth_dn5, locals.var_rth_dn6, locals.var_rth_dn7, locals.var_rth_dn8, locals.var_rth_dn9, locals.var_rth_dn10, locals.var_rth_dn13,)
    }
};
        locals.var_rth = assign17950_e12469;
        locals.var_rth_dn0 = assign17950_e12469_d_n0;
        locals.var_rth_dn2 = assign17950_e12469_d_n2;
        locals.var_rth_dn4 = assign17950_e12469_d_n4;
        locals.var_rth_dn5 = assign17950_e12469_d_n5;
        locals.var_rth_dn6 = assign17950_e12469_d_n6;
        locals.var_rth_dn7 = assign17950_e12469_d_n7;
        locals.var_rth_dn8 = assign17950_e12469_d_n8;
        locals.var_rth_dn9 = assign17950_e12469_d_n9;
        locals.var_rth_dn10 = assign17950_e12469_d_n10;
        locals.var_rth_dn13 = assign17950_e12469_d_n13;
        locals.var_rth_rv = 0.0;

        let (assign17960_e12481, assign17960_e12481_d_n0, assign17960_e12481_d_n2, assign17960_e12481_d_n4, assign17960_e12481_d_n5, assign17960_e12481_d_n6, assign17960_e12481_d_n7, assign17960_e12481_d_n8, assign17960_e12481_d_n9, assign17960_e12481_d_n10, assign17960_e12481_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17960_e12474: f64 = (p.p330 * locals.var_tdiff0);
        let assign17960_e12475: f64 = (locals.var_uc_powrat + assign17960_e12474);
        let assign17960_e12478: f64 = (p.p331 * locals.var_tdiff0_2);
        let assign17960_e12479: f64 = (assign17960_e12475 + assign17960_e12478);
        (assign17960_e12479, ((p.p330 * locals.var_tdiff0_dn0) + (p.p331 * locals.var_tdiff0_2_dn0)), ((p.p330 * locals.var_tdiff0_dn2) + (p.p331 * locals.var_tdiff0_2_dn2)), ((p.p330 * locals.var_tdiff0_dn4) + (p.p331 * locals.var_tdiff0_2_dn4)), ((p.p330 * locals.var_tdiff0_dn5) + (p.p331 * locals.var_tdiff0_2_dn5)), ((p.p330 * locals.var_tdiff0_dn6) + (p.p331 * locals.var_tdiff0_2_dn6)), ((p.p330 * locals.var_tdiff0_dn7) + (p.p331 * locals.var_tdiff0_2_dn7)), ((p.p330 * locals.var_tdiff0_dn8) + (p.p331 * locals.var_tdiff0_2_dn8)), ((p.p330 * locals.var_tdiff0_dn9) + (p.p331 * locals.var_tdiff0_2_dn9)), ((p.p330 * locals.var_tdiff0_dn10) + (p.p331 * locals.var_tdiff0_2_dn10)), ((p.p330 * locals.var_tdiff0_dn13) + (p.p331 * locals.var_tdiff0_2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign17960_e12481;
        locals.var_t2_dn0 = assign17960_e12481_d_n0;
        locals.var_t2_dn2 = assign17960_e12481_d_n2;
        locals.var_t2_dn4 = assign17960_e12481_d_n4;
        locals.var_t2_dn5 = assign17960_e12481_d_n5;
        locals.var_t2_dn6 = assign17960_e12481_d_n6;
        locals.var_t2_dn7 = assign17960_e12481_d_n7;
        locals.var_t2_dn8 = assign17960_e12481_d_n8;
        locals.var_t2_dn9 = assign17960_e12481_d_n9;
        locals.var_t2_dn10 = assign17960_e12481_d_n10;
        locals.var_t2_dn13 = assign17960_e12481_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign17970_e12489, assign17970_e12489_d_n0, assign17970_e12489_d_n2, assign17970_e12489_d_n4, assign17970_e12489_d_n5, assign17970_e12489_d_n6, assign17970_e12489_d_n7, assign17970_e12489_d_n8, assign17970_e12489_d_n9, assign17970_e12489_d_n10, assign17970_e12489_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign17970_e12485: f64 = locals.var_t2;
        let assign17970_e12487: f64 = (assign17970_e12485 - 0.05);
        (assign17970_e12487, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign17970_e12489;
        locals.var_tmf1_dn0 = assign17970_e12489_d_n0;
        locals.var_tmf1_dn2 = assign17970_e12489_d_n2;
        locals.var_tmf1_dn4 = assign17970_e12489_d_n4;
        locals.var_tmf1_dn5 = assign17970_e12489_d_n5;
        locals.var_tmf1_dn6 = assign17970_e12489_d_n6;
        locals.var_tmf1_dn7 = assign17970_e12489_d_n7;
        locals.var_tmf1_dn8 = assign17970_e12489_d_n8;
        locals.var_tmf1_dn9 = assign17970_e12489_d_n9;
        locals.var_tmf1_dn10 = assign17970_e12489_d_n10;
        locals.var_tmf1_dn13 = assign17970_e12489_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign17980_e12497, assign17980_e12497_d_n0, assign17980_e12497_d_n2, assign17980_e12497_d_n4, assign17980_e12497_d_n5, assign17980_e12497_d_n6, assign17980_e12497_d_n7, assign17980_e12497_d_n8, assign17980_e12497_d_n9, assign17980_e12497_d_n10, assign17980_e12497_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign17980_e12497;
        locals.var_tmf2_dn0 = assign17980_e12497_d_n0;
        locals.var_tmf2_dn2 = assign17980_e12497_d_n2;
        locals.var_tmf2_dn4 = assign17980_e12497_d_n4;
        locals.var_tmf2_dn5 = assign17980_e12497_d_n5;
        locals.var_tmf2_dn6 = assign17980_e12497_d_n6;
        locals.var_tmf2_dn7 = assign17980_e12497_d_n7;
        locals.var_tmf2_dn8 = assign17980_e12497_d_n8;
        locals.var_tmf2_dn9 = assign17980_e12497_d_n9;
        locals.var_tmf2_dn10 = assign17980_e12497_d_n10;
        locals.var_tmf2_dn13 = assign17980_e12497_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign17990_e12507, assign17990_e12507_d_n0, assign17990_e12507_d_n2, assign17990_e12507_d_n4, assign17990_e12507_d_n5, assign17990_e12507_d_n6, assign17990_e12507_d_n7, assign17990_e12507_d_n8, assign17990_e12507_d_n9, assign17990_e12507_d_n10, assign17990_e12507_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let (assign17990_e12505, assign17990_e12505_d_n0, assign17990_e12505_d_n2, assign17990_e12505_d_n4, assign17990_e12505_d_n5, assign17990_e12505_d_n6, assign17990_e12505_d_n7, assign17990_e12505_d_n8, assign17990_e12505_d_n9, assign17990_e12505_d_n10, assign17990_e12505_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign17990_e12504: f64 = (-locals.var_tmf2);
                (assign17990_e12504, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign17990_e12505, assign17990_e12505_d_n0, assign17990_e12505_d_n2, assign17990_e12505_d_n4, assign17990_e12505_d_n5, assign17990_e12505_d_n6, assign17990_e12505_d_n7, assign17990_e12505_d_n8, assign17990_e12505_d_n9, assign17990_e12505_d_n10, assign17990_e12505_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign17990_e12507;
        locals.var_tmf2_dn0 = assign17990_e12507_d_n0;
        locals.var_tmf2_dn2 = assign17990_e12507_d_n2;
        locals.var_tmf2_dn4 = assign17990_e12507_d_n4;
        locals.var_tmf2_dn5 = assign17990_e12507_d_n5;
        locals.var_tmf2_dn6 = assign17990_e12507_d_n6;
        locals.var_tmf2_dn7 = assign17990_e12507_d_n7;
        locals.var_tmf2_dn8 = assign17990_e12507_d_n8;
        locals.var_tmf2_dn9 = assign17990_e12507_d_n9;
        locals.var_tmf2_dn10 = assign17990_e12507_d_n10;
        locals.var_tmf2_dn13 = assign17990_e12507_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign18000_e12516, assign18000_e12516_d_n0, assign18000_e12516_d_n2, assign18000_e12516_d_n4, assign18000_e12516_d_n5, assign18000_e12516_d_n6, assign18000_e12516_d_n7, assign18000_e12516_d_n8, assign18000_e12516_d_n9, assign18000_e12516_d_n10, assign18000_e12516_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign18000_e12511: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18000_e12513: f64 = (assign18000_e12511 + locals.var_tmf2);
        let assign18000_e12514: f64 = (assign18000_e12513).sqrt();
        (assign18000_e12514, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18000_e12514)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18000_e12514)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18000_e12514)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18000_e12514)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18000_e12514)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18000_e12514)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18000_e12514)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18000_e12514)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18000_e12514)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign18000_e12514)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18000_e12516;
        locals.var_tmf2_dn0 = assign18000_e12516_d_n0;
        locals.var_tmf2_dn2 = assign18000_e12516_d_n2;
        locals.var_tmf2_dn4 = assign18000_e12516_d_n4;
        locals.var_tmf2_dn5 = assign18000_e12516_d_n5;
        locals.var_tmf2_dn6 = assign18000_e12516_d_n6;
        locals.var_tmf2_dn7 = assign18000_e12516_d_n7;
        locals.var_tmf2_dn8 = assign18000_e12516_d_n8;
        locals.var_tmf2_dn9 = assign18000_e12516_d_n9;
        locals.var_tmf2_dn10 = assign18000_e12516_d_n10;
        locals.var_tmf2_dn13 = assign18000_e12516_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign18010_e12526, assign18010_e12526_d_n0, assign18010_e12526_d_n2, assign18010_e12526_d_n4, assign18010_e12526_d_n5, assign18010_e12526_d_n6, assign18010_e12526_d_n7, assign18010_e12526_d_n8, assign18010_e12526_d_n9, assign18010_e12526_d_n10, assign18010_e12526_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign18010_e12522: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18010_e12523: f64 = (1.0 + assign18010_e12522);
        let assign18010_e12524: f64 = (0.5 * assign18010_e12523);
        (assign18010_e12524, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign18010_e12526;
        locals.var_t0_dn0 = assign18010_e12526_d_n0;
        locals.var_t0_dn2 = assign18010_e12526_d_n2;
        locals.var_t0_dn4 = assign18010_e12526_d_n4;
        locals.var_t0_dn5 = assign18010_e12526_d_n5;
        locals.var_t0_dn6 = assign18010_e12526_d_n6;
        locals.var_t0_dn7 = assign18010_e12526_d_n7;
        locals.var_t0_dn8 = assign18010_e12526_d_n8;
        locals.var_t0_dn9 = assign18010_e12526_d_n9;
        locals.var_t0_dn10 = assign18010_e12526_d_n10;
        locals.var_t0_dn13 = assign18010_e12526_d_n13;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_41(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign18020_e12536, assign18020_e12536_d_n0, assign18020_e12536_d_n2, assign18020_e12536_d_n4, assign18020_e12536_d_n5, assign18020_e12536_d_n6, assign18020_e12536_d_n7, assign18020_e12536_d_n8, assign18020_e12536_d_n9, assign18020_e12536_d_n10, assign18020_e12536_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign18020_e12532: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18020_e12533: f64 = (0.5 * assign18020_e12532);
        let assign18020_e12534: f64 = assign18020_e12533;
        (assign18020_e12534, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign18020_e12536;
        locals.var_t2_dn0 = assign18020_e12536_d_n0;
        locals.var_t2_dn2 = assign18020_e12536_d_n2;
        locals.var_t2_dn4 = assign18020_e12536_d_n4;
        locals.var_t2_dn5 = assign18020_e12536_d_n5;
        locals.var_t2_dn6 = assign18020_e12536_d_n6;
        locals.var_t2_dn7 = assign18020_e12536_d_n7;
        locals.var_t2_dn8 = assign18020_e12536_d_n8;
        locals.var_t2_dn9 = assign18020_e12536_d_n9;
        locals.var_t2_dn10 = assign18020_e12536_d_n10;
        locals.var_t2_dn13 = assign18020_e12536_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign18030_e12544, assign18030_e12544_d_n0, assign18030_e12544_d_n2, assign18030_e12544_d_n4, assign18030_e12544_d_n5, assign18030_e12544_d_n6, assign18030_e12544_d_n7, assign18030_e12544_d_n8, assign18030_e12544_d_n9, assign18030_e12544_d_n10, assign18030_e12544_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign18030_e12540: f64 = (1.0 - locals.var_t2);
        let assign18030_e12542: f64 = (assign18030_e12540 - 0.05);
        (assign18030_e12542, (-locals.var_t2_dn0), (-locals.var_t2_dn2), (-locals.var_t2_dn4), (-locals.var_t2_dn5), (-locals.var_t2_dn6), (-locals.var_t2_dn7), (-locals.var_t2_dn8), (-locals.var_t2_dn9), (-locals.var_t2_dn10), (-locals.var_t2_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign18030_e12544;
        locals.var_tmf1_dn0 = assign18030_e12544_d_n0;
        locals.var_tmf1_dn2 = assign18030_e12544_d_n2;
        locals.var_tmf1_dn4 = assign18030_e12544_d_n4;
        locals.var_tmf1_dn5 = assign18030_e12544_d_n5;
        locals.var_tmf1_dn6 = assign18030_e12544_d_n6;
        locals.var_tmf1_dn7 = assign18030_e12544_d_n7;
        locals.var_tmf1_dn8 = assign18030_e12544_d_n8;
        locals.var_tmf1_dn9 = assign18030_e12544_d_n9;
        locals.var_tmf1_dn10 = assign18030_e12544_d_n10;
        locals.var_tmf1_dn13 = assign18030_e12544_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign18040_e12552, assign18040_e12552_d_n0, assign18040_e12552_d_n2, assign18040_e12552_d_n4, assign18040_e12552_d_n5, assign18040_e12552_d_n6, assign18040_e12552_d_n7, assign18040_e12552_d_n8, assign18040_e12552_d_n9, assign18040_e12552_d_n10, assign18040_e12552_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign18040_e12548: f64 = 4.0;
        let assign18040_e12550: f64 = (assign18040_e12548 * 0.05);
        (assign18040_e12550, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18040_e12552;
        locals.var_tmf2_dn0 = assign18040_e12552_d_n0;
        locals.var_tmf2_dn2 = assign18040_e12552_d_n2;
        locals.var_tmf2_dn4 = assign18040_e12552_d_n4;
        locals.var_tmf2_dn5 = assign18040_e12552_d_n5;
        locals.var_tmf2_dn6 = assign18040_e12552_d_n6;
        locals.var_tmf2_dn7 = assign18040_e12552_d_n7;
        locals.var_tmf2_dn8 = assign18040_e12552_d_n8;
        locals.var_tmf2_dn9 = assign18040_e12552_d_n9;
        locals.var_tmf2_dn10 = assign18040_e12552_d_n10;
        locals.var_tmf2_dn13 = assign18040_e12552_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign18050_e12562, assign18050_e12562_d_n0, assign18050_e12562_d_n2, assign18050_e12562_d_n4, assign18050_e12562_d_n5, assign18050_e12562_d_n6, assign18050_e12562_d_n7, assign18050_e12562_d_n8, assign18050_e12562_d_n9, assign18050_e12562_d_n10, assign18050_e12562_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let (assign18050_e12560, assign18050_e12560_d_n0, assign18050_e12560_d_n2, assign18050_e12560_d_n4, assign18050_e12560_d_n5, assign18050_e12560_d_n6, assign18050_e12560_d_n7, assign18050_e12560_d_n8, assign18050_e12560_d_n9, assign18050_e12560_d_n10, assign18050_e12560_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign18050_e12559: f64 = (-locals.var_tmf2);
                (assign18050_e12559, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign18050_e12560, assign18050_e12560_d_n0, assign18050_e12560_d_n2, assign18050_e12560_d_n4, assign18050_e12560_d_n5, assign18050_e12560_d_n6, assign18050_e12560_d_n7, assign18050_e12560_d_n8, assign18050_e12560_d_n9, assign18050_e12560_d_n10, assign18050_e12560_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18050_e12562;
        locals.var_tmf2_dn0 = assign18050_e12562_d_n0;
        locals.var_tmf2_dn2 = assign18050_e12562_d_n2;
        locals.var_tmf2_dn4 = assign18050_e12562_d_n4;
        locals.var_tmf2_dn5 = assign18050_e12562_d_n5;
        locals.var_tmf2_dn6 = assign18050_e12562_d_n6;
        locals.var_tmf2_dn7 = assign18050_e12562_d_n7;
        locals.var_tmf2_dn8 = assign18050_e12562_d_n8;
        locals.var_tmf2_dn9 = assign18050_e12562_d_n9;
        locals.var_tmf2_dn10 = assign18050_e12562_d_n10;
        locals.var_tmf2_dn13 = assign18050_e12562_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign18060_e12571, assign18060_e12571_d_n0, assign18060_e12571_d_n2, assign18060_e12571_d_n4, assign18060_e12571_d_n5, assign18060_e12571_d_n6, assign18060_e12571_d_n7, assign18060_e12571_d_n8, assign18060_e12571_d_n9, assign18060_e12571_d_n10, assign18060_e12571_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign18060_e12566: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18060_e12568: f64 = (assign18060_e12566 + locals.var_tmf2);
        let assign18060_e12569: f64 = (assign18060_e12568).sqrt();
        (assign18060_e12569, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18060_e12569)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18060_e12569)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18060_e12569)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18060_e12569)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18060_e12569)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18060_e12569)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18060_e12569)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18060_e12569)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18060_e12569)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign18060_e12569)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18060_e12571;
        locals.var_tmf2_dn0 = assign18060_e12571_d_n0;
        locals.var_tmf2_dn2 = assign18060_e12571_d_n2;
        locals.var_tmf2_dn4 = assign18060_e12571_d_n4;
        locals.var_tmf2_dn5 = assign18060_e12571_d_n5;
        locals.var_tmf2_dn6 = assign18060_e12571_d_n6;
        locals.var_tmf2_dn7 = assign18060_e12571_d_n7;
        locals.var_tmf2_dn8 = assign18060_e12571_d_n8;
        locals.var_tmf2_dn9 = assign18060_e12571_d_n9;
        locals.var_tmf2_dn10 = assign18060_e12571_d_n10;
        locals.var_tmf2_dn13 = assign18060_e12571_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign18070_e12581, assign18070_e12581_d_n0, assign18070_e12581_d_n2, assign18070_e12581_d_n4, assign18070_e12581_d_n5, assign18070_e12581_d_n6, assign18070_e12581_d_n7, assign18070_e12581_d_n8, assign18070_e12581_d_n9, assign18070_e12581_d_n10, assign18070_e12581_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign18070_e12577: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18070_e12578: f64 = (1.0 + assign18070_e12577);
        let assign18070_e12579: f64 = (0.5 * assign18070_e12578);
        (assign18070_e12579, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign18070_e12581;
        locals.var_t0_dn0 = assign18070_e12581_d_n0;
        locals.var_t0_dn2 = assign18070_e12581_d_n2;
        locals.var_t0_dn4 = assign18070_e12581_d_n4;
        locals.var_t0_dn5 = assign18070_e12581_d_n5;
        locals.var_t0_dn6 = assign18070_e12581_d_n6;
        locals.var_t0_dn7 = assign18070_e12581_d_n7;
        locals.var_t0_dn8 = assign18070_e12581_d_n8;
        locals.var_t0_dn9 = assign18070_e12581_d_n9;
        locals.var_t0_dn10 = assign18070_e12581_d_n10;
        locals.var_t0_dn13 = assign18070_e12581_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign18080_e12591, assign18080_e12591_d_n0, assign18080_e12591_d_n2, assign18080_e12591_d_n4, assign18080_e12591_d_n5, assign18080_e12591_d_n6, assign18080_e12591_d_n7, assign18080_e12591_d_n8, assign18080_e12591_d_n9, assign18080_e12591_d_n10, assign18080_e12591_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign18080_e12587: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18080_e12588: f64 = (0.5 * assign18080_e12587);
        let assign18080_e12589: f64 = (1.0 - assign18080_e12588);
        (assign18080_e12589, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_powratio, locals.var_powratio_dn0, locals.var_powratio_dn2, locals.var_powratio_dn4, locals.var_powratio_dn5, locals.var_powratio_dn6, locals.var_powratio_dn7, locals.var_powratio_dn8, locals.var_powratio_dn9, locals.var_powratio_dn10, locals.var_powratio_dn13,)
    }
};
        locals.var_powratio = assign18080_e12591;
        locals.var_powratio_dn0 = assign18080_e12591_d_n0;
        locals.var_powratio_dn2 = assign18080_e12591_d_n2;
        locals.var_powratio_dn4 = assign18080_e12591_d_n4;
        locals.var_powratio_dn5 = assign18080_e12591_d_n5;
        locals.var_powratio_dn6 = assign18080_e12591_d_n6;
        locals.var_powratio_dn7 = assign18080_e12591_d_n7;
        locals.var_powratio_dn8 = assign18080_e12591_d_n8;
        locals.var_powratio_dn9 = assign18080_e12591_d_n9;
        locals.var_powratio_dn10 = assign18080_e12591_d_n10;
        locals.var_powratio_dn13 = assign18080_e12591_d_n13;
        locals.var_powratio_rv = 0.0;

        let (assign18090_e12602, assign18090_e12602_d_n0, assign18090_e12602_d_n2, assign18090_e12602_d_n4, assign18090_e12602_d_n5, assign18090_e12602_d_n6, assign18090_e12602_d_n7, assign18090_e12602_d_n8, assign18090_e12602_d_n9, assign18090_e12602_d_n10, assign18090_e12602_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign18090_e12595: f64 = (2.0 * locals.var_beta_inv);
        let assign18090_e12598: f64 = (locals.var_nsub / locals.var_nin);
        let assign18090_e12599: f64 = (assign18090_e12598).ln();
        let assign18090_e12600: f64 = (assign18090_e12595 * assign18090_e12599);
        (assign18090_e12600, (((2.0 * locals.var_beta_inv_dn0) * assign18090_e12599) + (assign18090_e12595 * ((((locals.var_nsub_dn0 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign18090_e12598))), (((2.0 * locals.var_beta_inv_dn2) * assign18090_e12599) + (assign18090_e12595 * ((((locals.var_nsub_dn2 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign18090_e12598))), (((2.0 * locals.var_beta_inv_dn4) * assign18090_e12599) + (assign18090_e12595 * ((((locals.var_nsub_dn4 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign18090_e12598))), (((2.0 * locals.var_beta_inv_dn5) * assign18090_e12599) + (assign18090_e12595 * ((((locals.var_nsub_dn5 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign18090_e12598))), (((2.0 * locals.var_beta_inv_dn6) * assign18090_e12599) + (assign18090_e12595 * ((((locals.var_nsub_dn6 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign18090_e12598))), (((2.0 * locals.var_beta_inv_dn7) * assign18090_e12599) + (assign18090_e12595 * ((((locals.var_nsub_dn7 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign18090_e12598))), (((2.0 * locals.var_beta_inv_dn8) * assign18090_e12599) + (assign18090_e12595 * ((((locals.var_nsub_dn8 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign18090_e12598))), (((2.0 * locals.var_beta_inv_dn9) * assign18090_e12599) + (assign18090_e12595 * ((((locals.var_nsub_dn9 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign18090_e12598))), (((2.0 * locals.var_beta_inv_dn10) * assign18090_e12599) + (assign18090_e12595 * ((((locals.var_nsub_dn10 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign18090_e12598))), (((2.0 * locals.var_beta_inv_dn13) * assign18090_e12599) + (assign18090_e12595 * ((((locals.var_nsub_dn13 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn13)) / (locals.var_nin * locals.var_nin)) / assign18090_e12598))),)
    } else {
        (locals.var_pb2, locals.var_pb2_dn0, locals.var_pb2_dn2, locals.var_pb2_dn4, locals.var_pb2_dn5, locals.var_pb2_dn6, locals.var_pb2_dn7, locals.var_pb2_dn8, locals.var_pb2_dn9, locals.var_pb2_dn10, locals.var_pb2_dn13,)
    }
};
        locals.var_pb2 = assign18090_e12602;
        locals.var_pb2_dn0 = assign18090_e12602_d_n0;
        locals.var_pb2_dn2 = assign18090_e12602_d_n2;
        locals.var_pb2_dn4 = assign18090_e12602_d_n4;
        locals.var_pb2_dn5 = assign18090_e12602_d_n5;
        locals.var_pb2_dn6 = assign18090_e12602_d_n6;
        locals.var_pb2_dn7 = assign18090_e12602_d_n7;
        locals.var_pb2_dn8 = assign18090_e12602_d_n8;
        locals.var_pb2_dn9 = assign18090_e12602_d_n9;
        locals.var_pb2_dn10 = assign18090_e12602_d_n10;
        locals.var_pb2_dn13 = assign18090_e12602_d_n13;
        locals.var_pb2_rv = 0.0;

        let (assign18100_e12610, assign18100_e12610_d_n0, assign18100_e12610_d_n2, assign18100_e12610_d_n4, assign18100_e12610_d_n5, assign18100_e12610_d_n6, assign18100_e12610_d_n7, assign18100_e12610_d_n8, assign18100_e12610_d_n9, assign18100_e12610_d_n10, assign18100_e12610_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign18100_e12606: f64 = (2.0 * 1.034943e-10);
        let assign18100_e12608: f64 = (assign18100_e12606 / 1.6021918e-19);
        (assign18100_e12608, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign18100_e12610;
        locals.var_t1_dn0 = assign18100_e12610_d_n0;
        locals.var_t1_dn2 = assign18100_e12610_d_n2;
        locals.var_t1_dn4 = assign18100_e12610_d_n4;
        locals.var_t1_dn5 = assign18100_e12610_d_n5;
        locals.var_t1_dn6 = assign18100_e12610_d_n6;
        locals.var_t1_dn7 = assign18100_e12610_d_n7;
        locals.var_t1_dn8 = assign18100_e12610_d_n8;
        locals.var_t1_dn9 = assign18100_e12610_d_n9;
        locals.var_t1_dn10 = assign18100_e12610_d_n10;
        locals.var_t1_dn13 = assign18100_e12610_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign18110_e12617, assign18110_e12617_d_n0, assign18110_e12617_d_n2, assign18110_e12617_d_n4, assign18110_e12617_d_n5, assign18110_e12617_d_n6, assign18110_e12617_d_n7, assign18110_e12617_d_n8, assign18110_e12617_d_n9, assign18110_e12617_d_n10, assign18110_e12617_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign18110_e12614: f64 = (locals.var_t1 / locals.var_nsub);
        let assign18110_e12615: f64 = (assign18110_e12614).sqrt();
        (assign18110_e12615, ((((locals.var_t1_dn0 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn0)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18110_e12615)), ((((locals.var_t1_dn2 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn2)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18110_e12615)), ((((locals.var_t1_dn4 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn4)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18110_e12615)), ((((locals.var_t1_dn5 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn5)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18110_e12615)), ((((locals.var_t1_dn6 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn6)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18110_e12615)), ((((locals.var_t1_dn7 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn7)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18110_e12615)), ((((locals.var_t1_dn8 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn8)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18110_e12615)), ((((locals.var_t1_dn9 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn9)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18110_e12615)), ((((locals.var_t1_dn10 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn10)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18110_e12615)), ((((locals.var_t1_dn13 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn13)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18110_e12615)),)
    } else {
        (locals.var_wdpl, locals.var_wdpl_dn0, locals.var_wdpl_dn2, locals.var_wdpl_dn4, locals.var_wdpl_dn5, locals.var_wdpl_dn6, locals.var_wdpl_dn7, locals.var_wdpl_dn8, locals.var_wdpl_dn9, locals.var_wdpl_dn10, locals.var_wdpl_dn13,)
    }
};
        locals.var_wdpl = assign18110_e12617;
        locals.var_wdpl_dn0 = assign18110_e12617_d_n0;
        locals.var_wdpl_dn2 = assign18110_e12617_d_n2;
        locals.var_wdpl_dn4 = assign18110_e12617_d_n4;
        locals.var_wdpl_dn5 = assign18110_e12617_d_n5;
        locals.var_wdpl_dn6 = assign18110_e12617_d_n6;
        locals.var_wdpl_dn7 = assign18110_e12617_d_n7;
        locals.var_wdpl_dn8 = assign18110_e12617_d_n8;
        locals.var_wdpl_dn9 = assign18110_e12617_d_n9;
        locals.var_wdpl_dn10 = assign18110_e12617_d_n10;
        locals.var_wdpl_dn13 = assign18110_e12617_d_n13;
        locals.var_wdpl_rv = 0.0;

        let (assign18120_e12624, assign18120_e12624_d_n0, assign18120_e12624_d_n2, assign18120_e12624_d_n4, assign18120_e12624_d_n5, assign18120_e12624_d_n6, assign18120_e12624_d_n7, assign18120_e12624_d_n8, assign18120_e12624_d_n9, assign18120_e12624_d_n10, assign18120_e12624_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign18120_e12621: f64 = (locals.var_t1 / locals.var_ef_nsubp);
        let assign18120_e12622: f64 = (assign18120_e12621).sqrt();
        (assign18120_e12622, ((((locals.var_t1_dn0 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn0)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18120_e12622)), ((((locals.var_t1_dn2 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn2)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18120_e12622)), ((((locals.var_t1_dn4 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn4)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18120_e12622)), ((((locals.var_t1_dn5 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn5)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18120_e12622)), ((((locals.var_t1_dn6 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn6)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18120_e12622)), ((((locals.var_t1_dn7 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn7)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18120_e12622)), ((((locals.var_t1_dn8 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn8)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18120_e12622)), ((((locals.var_t1_dn9 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn9)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18120_e12622)), ((((locals.var_t1_dn10 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn10)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18120_e12622)), ((((locals.var_t1_dn13 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn13)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18120_e12622)),)
    } else {
        (locals.var_wdplp, locals.var_wdplp_dn0, locals.var_wdplp_dn2, locals.var_wdplp_dn4, locals.var_wdplp_dn5, locals.var_wdplp_dn6, locals.var_wdplp_dn7, locals.var_wdplp_dn8, locals.var_wdplp_dn9, locals.var_wdplp_dn10, locals.var_wdplp_dn13,)
    }
};
        locals.var_wdplp = assign18120_e12624;
        locals.var_wdplp_dn0 = assign18120_e12624_d_n0;
        locals.var_wdplp_dn2 = assign18120_e12624_d_n2;
        locals.var_wdplp_dn4 = assign18120_e12624_d_n4;
        locals.var_wdplp_dn5 = assign18120_e12624_d_n5;
        locals.var_wdplp_dn6 = assign18120_e12624_d_n6;
        locals.var_wdplp_dn7 = assign18120_e12624_d_n7;
        locals.var_wdplp_dn8 = assign18120_e12624_d_n8;
        locals.var_wdplp_dn9 = assign18120_e12624_d_n9;
        locals.var_wdplp_dn10 = assign18120_e12624_d_n10;
        locals.var_wdplp_dn13 = assign18120_e12624_d_n13;
        locals.var_wdplp_rv = 0.0;

        let assign18130_e12627: f64 = if locals.var_uc_codep == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard372 = assign18130_e12627;
        locals.var_guard372_rv = 0.0;

        let (assign18140_e12642, assign18140_e12642_d_n0, assign18140_e12642_d_n2, assign18140_e12642_d_n4, assign18140_e12642_d_n5, assign18140_e12642_d_n6, assign18140_e12642_d_n7, assign18140_e12642_d_n8, assign18140_e12642_d_n9, assign18140_e12642_d_n10, assign18140_e12642_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard372 != 0.0)) {
        let assign18140_e12633: f64 = (2.0 * 1.034943e-10);
        let assign18140_e12635: f64 = (assign18140_e12633 * 1.6021918e-19);
        let assign18140_e12637: f64 = (assign18140_e12635 * locals.var_nsub);
        let assign18140_e12639: f64 = (assign18140_e12637 * locals.var_beta_inv);
        let assign18140_e12640: f64 = (assign18140_e12639).sqrt();
        (assign18140_e12640, ((((assign18140_e12635 * locals.var_nsub_dn0) * locals.var_beta_inv) + (assign18140_e12637 * locals.var_beta_inv_dn0)) / (2.0 * assign18140_e12640)), ((((assign18140_e12635 * locals.var_nsub_dn2) * locals.var_beta_inv) + (assign18140_e12637 * locals.var_beta_inv_dn2)) / (2.0 * assign18140_e12640)), ((((assign18140_e12635 * locals.var_nsub_dn4) * locals.var_beta_inv) + (assign18140_e12637 * locals.var_beta_inv_dn4)) / (2.0 * assign18140_e12640)), ((((assign18140_e12635 * locals.var_nsub_dn5) * locals.var_beta_inv) + (assign18140_e12637 * locals.var_beta_inv_dn5)) / (2.0 * assign18140_e12640)), ((((assign18140_e12635 * locals.var_nsub_dn6) * locals.var_beta_inv) + (assign18140_e12637 * locals.var_beta_inv_dn6)) / (2.0 * assign18140_e12640)), ((((assign18140_e12635 * locals.var_nsub_dn7) * locals.var_beta_inv) + (assign18140_e12637 * locals.var_beta_inv_dn7)) / (2.0 * assign18140_e12640)), ((((assign18140_e12635 * locals.var_nsub_dn8) * locals.var_beta_inv) + (assign18140_e12637 * locals.var_beta_inv_dn8)) / (2.0 * assign18140_e12640)), ((((assign18140_e12635 * locals.var_nsub_dn9) * locals.var_beta_inv) + (assign18140_e12637 * locals.var_beta_inv_dn9)) / (2.0 * assign18140_e12640)), ((((assign18140_e12635 * locals.var_nsub_dn10) * locals.var_beta_inv) + (assign18140_e12637 * locals.var_beta_inv_dn10)) / (2.0 * assign18140_e12640)), ((((assign18140_e12635 * locals.var_nsub_dn13) * locals.var_beta_inv) + (assign18140_e12637 * locals.var_beta_inv_dn13)) / (2.0 * assign18140_e12640)),)
    } else {
        (locals.var_cnst0, locals.var_cnst0_dn0, locals.var_cnst0_dn2, locals.var_cnst0_dn4, locals.var_cnst0_dn5, locals.var_cnst0_dn6, locals.var_cnst0_dn7, locals.var_cnst0_dn8, locals.var_cnst0_dn9, locals.var_cnst0_dn10, locals.var_cnst0_dn13,)
    }
};
        locals.var_cnst0 = assign18140_e12642;
        locals.var_cnst0_dn0 = assign18140_e12642_d_n0;
        locals.var_cnst0_dn2 = assign18140_e12642_d_n2;
        locals.var_cnst0_dn4 = assign18140_e12642_d_n4;
        locals.var_cnst0_dn5 = assign18140_e12642_d_n5;
        locals.var_cnst0_dn6 = assign18140_e12642_d_n6;
        locals.var_cnst0_dn7 = assign18140_e12642_d_n7;
        locals.var_cnst0_dn8 = assign18140_e12642_d_n8;
        locals.var_cnst0_dn9 = assign18140_e12642_d_n9;
        locals.var_cnst0_dn10 = assign18140_e12642_d_n10;
        locals.var_cnst0_dn13 = assign18140_e12642_d_n13;
        locals.var_cnst0_rv = 0.0;

        let (assign18150_e12650, assign18150_e12650_d_n0, assign18150_e12650_d_n2, assign18150_e12650_d_n4, assign18150_e12650_d_n5, assign18150_e12650_d_n6, assign18150_e12650_d_n7, assign18150_e12650_d_n8, assign18150_e12650_d_n9, assign18150_e12650_d_n10, assign18150_e12650_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard372 != 0.0)) {
        let assign18150_e12648: f64 = (locals.var_nin / locals.var_nsub);
        (assign18150_e12648, (((locals.var_nin_dn0 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn0)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn2 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn2)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn4 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn4)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn5 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn5)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn6 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn6)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn7 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn7)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn8 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn8)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn9 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn9)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn10 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn10)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn13 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn13)) / (locals.var_nsub * locals.var_nsub)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign18150_e12650;
        locals.var_t1_dn0 = assign18150_e12650_d_n0;
        locals.var_t1_dn2 = assign18150_e12650_d_n2;
        locals.var_t1_dn4 = assign18150_e12650_d_n4;
        locals.var_t1_dn5 = assign18150_e12650_d_n5;
        locals.var_t1_dn6 = assign18150_e12650_d_n6;
        locals.var_t1_dn7 = assign18150_e12650_d_n7;
        locals.var_t1_dn8 = assign18150_e12650_d_n8;
        locals.var_t1_dn9 = assign18150_e12650_d_n9;
        locals.var_t1_dn10 = assign18150_e12650_d_n10;
        locals.var_t1_dn13 = assign18150_e12650_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign18160_e12658, assign18160_e12658_d_n0, assign18160_e12658_d_n2, assign18160_e12658_d_n4, assign18160_e12658_d_n5, assign18160_e12658_d_n6, assign18160_e12658_d_n7, assign18160_e12658_d_n8, assign18160_e12658_d_n9, assign18160_e12658_d_n10, assign18160_e12658_d_n13,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard372 != 0.0)) {
        let assign18160_e12656: f64 = (locals.var_t1 * locals.var_t1);
        (assign18160_e12656, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)),)
    } else {
        (locals.var_cnst1, locals.var_cnst1_dn0, locals.var_cnst1_dn2, locals.var_cnst1_dn4, locals.var_cnst1_dn5, locals.var_cnst1_dn6, locals.var_cnst1_dn7, locals.var_cnst1_dn8, locals.var_cnst1_dn9, locals.var_cnst1_dn10, locals.var_cnst1_dn13,)
    }
};
        locals.var_cnst1 = assign18160_e12658;
        locals.var_cnst1_dn0 = assign18160_e12658_d_n0;
        locals.var_cnst1_dn2 = assign18160_e12658_d_n2;
        locals.var_cnst1_dn4 = assign18160_e12658_d_n4;
        locals.var_cnst1_dn5 = assign18160_e12658_d_n5;
        locals.var_cnst1_dn6 = assign18160_e12658_d_n6;
        locals.var_cnst1_dn7 = assign18160_e12658_d_n7;
        locals.var_cnst1_dn8 = assign18160_e12658_d_n8;
        locals.var_cnst1_dn9 = assign18160_e12658_d_n9;
        locals.var_cnst1_dn10 = assign18160_e12658_d_n10;
        locals.var_cnst1_dn13 = assign18160_e12658_d_n13;
        locals.var_cnst1_rv = 0.0;

        let assign18170_e12661: f64 = if locals.var_uc_codep == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard373 = assign18170_e12661;
        locals.var_guard373_rv = 0.0;

        let assign18180_e12664: f64 = if locals.var_uc_nover != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard374 = assign18180_e12664;
        locals.var_guard374_rv = 0.0;

        let (assign18190_e12677, assign18190_e12677_d_n0, assign18190_e12677_d_n2, assign18190_e12677_d_n4, assign18190_e12677_d_n5, assign18190_e12677_d_n6, assign18190_e12677_d_n7, assign18190_e12677_d_n8, assign18190_e12677_d_n9, assign18190_e12677_d_n10, assign18190_e12677_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard373 != 0.0)) && (locals.var_guard374 != 0.0)) {
        let assign18190_e12673: f64 = (locals.var_uc_nover / locals.var_nsub);
        let assign18190_e12674: f64 = (assign18190_e12673).sqrt();
        let assign18190_e12675: f64 = (locals.var_cnst0 * assign18190_e12674);
        (assign18190_e12675, ((locals.var_cnst0_dn0 * assign18190_e12674) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn0) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18190_e12674)))), ((locals.var_cnst0_dn2 * assign18190_e12674) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn2) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18190_e12674)))), ((locals.var_cnst0_dn4 * assign18190_e12674) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn4) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18190_e12674)))), ((locals.var_cnst0_dn5 * assign18190_e12674) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn5) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18190_e12674)))), ((locals.var_cnst0_dn6 * assign18190_e12674) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn6) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18190_e12674)))), ((locals.var_cnst0_dn7 * assign18190_e12674) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn7) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18190_e12674)))), ((locals.var_cnst0_dn8 * assign18190_e12674) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn8) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18190_e12674)))), ((locals.var_cnst0_dn9 * assign18190_e12674) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn9) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18190_e12674)))), ((locals.var_cnst0_dn10 * assign18190_e12674) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn10) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18190_e12674)))), ((locals.var_cnst0_dn13 * assign18190_e12674) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn13) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18190_e12674)))),)
    } else {
        (locals.var_cnst0over, locals.var_cnst0over_dn0, locals.var_cnst0over_dn2, locals.var_cnst0over_dn4, locals.var_cnst0over_dn5, locals.var_cnst0over_dn6, locals.var_cnst0over_dn7, locals.var_cnst0over_dn8, locals.var_cnst0over_dn9, locals.var_cnst0over_dn10, locals.var_cnst0over_dn13,)
    }
};
        locals.var_cnst0over = assign18190_e12677;
        locals.var_cnst0over_dn0 = assign18190_e12677_d_n0;
        locals.var_cnst0over_dn2 = assign18190_e12677_d_n2;
        locals.var_cnst0over_dn4 = assign18190_e12677_d_n4;
        locals.var_cnst0over_dn5 = assign18190_e12677_d_n5;
        locals.var_cnst0over_dn6 = assign18190_e12677_d_n6;
        locals.var_cnst0over_dn7 = assign18190_e12677_d_n7;
        locals.var_cnst0over_dn8 = assign18190_e12677_d_n8;
        locals.var_cnst0over_dn9 = assign18190_e12677_d_n9;
        locals.var_cnst0over_dn10 = assign18190_e12677_d_n10;
        locals.var_cnst0over_dn13 = assign18190_e12677_d_n13;
        locals.var_cnst0over_rv = 0.0;

        let assign18200_e12680: f64 = if locals.var_uc_novers != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard375 = assign18200_e12680;
        locals.var_guard375_rv = 0.0;

        let (assign18210_e12693, assign18210_e12693_d_n0, assign18210_e12693_d_n2, assign18210_e12693_d_n4, assign18210_e12693_d_n5, assign18210_e12693_d_n6, assign18210_e12693_d_n7, assign18210_e12693_d_n8, assign18210_e12693_d_n9, assign18210_e12693_d_n10, assign18210_e12693_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard373 != 0.0)) && (locals.var_guard375 != 0.0)) {
        let assign18210_e12689: f64 = (locals.var_uc_novers / locals.var_nsub);
        let assign18210_e12690: f64 = (assign18210_e12689).sqrt();
        let assign18210_e12691: f64 = (locals.var_cnst0 * assign18210_e12690);
        (assign18210_e12691, ((locals.var_cnst0_dn0 * assign18210_e12690) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn0) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18210_e12690)))), ((locals.var_cnst0_dn2 * assign18210_e12690) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn2) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18210_e12690)))), ((locals.var_cnst0_dn4 * assign18210_e12690) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn4) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18210_e12690)))), ((locals.var_cnst0_dn5 * assign18210_e12690) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn5) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18210_e12690)))), ((locals.var_cnst0_dn6 * assign18210_e12690) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn6) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18210_e12690)))), ((locals.var_cnst0_dn7 * assign18210_e12690) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn7) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18210_e12690)))), ((locals.var_cnst0_dn8 * assign18210_e12690) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn8) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18210_e12690)))), ((locals.var_cnst0_dn9 * assign18210_e12690) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn9) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18210_e12690)))), ((locals.var_cnst0_dn10 * assign18210_e12690) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn10) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18210_e12690)))), ((locals.var_cnst0_dn13 * assign18210_e12690) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn13) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18210_e12690)))),)
    } else {
        (locals.var_cnst0overs, locals.var_cnst0overs_dn0, locals.var_cnst0overs_dn2, locals.var_cnst0overs_dn4, locals.var_cnst0overs_dn5, locals.var_cnst0overs_dn6, locals.var_cnst0overs_dn7, locals.var_cnst0overs_dn8, locals.var_cnst0overs_dn9, locals.var_cnst0overs_dn10, locals.var_cnst0overs_dn13,)
    }
};
        locals.var_cnst0overs = assign18210_e12693;
        locals.var_cnst0overs_dn0 = assign18210_e12693_d_n0;
        locals.var_cnst0overs_dn2 = assign18210_e12693_d_n2;
        locals.var_cnst0overs_dn4 = assign18210_e12693_d_n4;
        locals.var_cnst0overs_dn5 = assign18210_e12693_d_n5;
        locals.var_cnst0overs_dn6 = assign18210_e12693_d_n6;
        locals.var_cnst0overs_dn7 = assign18210_e12693_d_n7;
        locals.var_cnst0overs_dn8 = assign18210_e12693_d_n8;
        locals.var_cnst0overs_dn9 = assign18210_e12693_d_n9;
        locals.var_cnst0overs_dn10 = assign18210_e12693_d_n10;
        locals.var_cnst0overs_dn13 = assign18210_e12693_d_n13;
        locals.var_cnst0overs_rv = 0.0;

        let assign18220_e12696: f64 = if locals.var_uc_nover != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard376 = assign18220_e12696;
        locals.var_guard376_rv = 0.0;

        let (assign18230_e12710, assign18230_e12710_d_n0, assign18230_e12710_d_n2, assign18230_e12710_d_n4, assign18230_e12710_d_n5, assign18230_e12710_d_n6, assign18230_e12710_d_n7, assign18230_e12710_d_n8, assign18230_e12710_d_n9, assign18230_e12710_d_n10, assign18230_e12710_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard373 == 0.0)) && (locals.var_guard376 != 0.0)) {
        let assign18230_e12706: f64 = (locals.var_uc_nover / locals.var_uc_ndepm);
        let assign18230_e12707: f64 = (assign18230_e12706).sqrt();
        let assign18230_e12708: f64 = (locals.var_cnst0 * assign18230_e12707);
        (assign18230_e12708, ((locals.var_cnst0_dn0 * assign18230_e12707) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn0) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18230_e12707)))), ((locals.var_cnst0_dn2 * assign18230_e12707) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn2) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18230_e12707)))), ((locals.var_cnst0_dn4 * assign18230_e12707) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn4) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18230_e12707)))), ((locals.var_cnst0_dn5 * assign18230_e12707) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn5) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18230_e12707)))), ((locals.var_cnst0_dn6 * assign18230_e12707) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn6) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18230_e12707)))), ((locals.var_cnst0_dn7 * assign18230_e12707) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn7) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18230_e12707)))), ((locals.var_cnst0_dn8 * assign18230_e12707) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn8) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18230_e12707)))), ((locals.var_cnst0_dn9 * assign18230_e12707) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn9) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18230_e12707)))), ((locals.var_cnst0_dn10 * assign18230_e12707) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn10) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18230_e12707)))), ((locals.var_cnst0_dn13 * assign18230_e12707) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn13) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18230_e12707)))),)
    } else {
        (locals.var_cnst0over, locals.var_cnst0over_dn0, locals.var_cnst0over_dn2, locals.var_cnst0over_dn4, locals.var_cnst0over_dn5, locals.var_cnst0over_dn6, locals.var_cnst0over_dn7, locals.var_cnst0over_dn8, locals.var_cnst0over_dn9, locals.var_cnst0over_dn10, locals.var_cnst0over_dn13,)
    }
};
        locals.var_cnst0over = assign18230_e12710;
        locals.var_cnst0over_dn0 = assign18230_e12710_d_n0;
        locals.var_cnst0over_dn2 = assign18230_e12710_d_n2;
        locals.var_cnst0over_dn4 = assign18230_e12710_d_n4;
        locals.var_cnst0over_dn5 = assign18230_e12710_d_n5;
        locals.var_cnst0over_dn6 = assign18230_e12710_d_n6;
        locals.var_cnst0over_dn7 = assign18230_e12710_d_n7;
        locals.var_cnst0over_dn8 = assign18230_e12710_d_n8;
        locals.var_cnst0over_dn9 = assign18230_e12710_d_n9;
        locals.var_cnst0over_dn10 = assign18230_e12710_d_n10;
        locals.var_cnst0over_dn13 = assign18230_e12710_d_n13;
        locals.var_cnst0over_rv = 0.0;

        let assign18240_e12713: f64 = if locals.var_uc_novers != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard377 = assign18240_e12713;
        locals.var_guard377_rv = 0.0;

        let (assign18250_e12727, assign18250_e12727_d_n0, assign18250_e12727_d_n2, assign18250_e12727_d_n4, assign18250_e12727_d_n5, assign18250_e12727_d_n6, assign18250_e12727_d_n7, assign18250_e12727_d_n8, assign18250_e12727_d_n9, assign18250_e12727_d_n10, assign18250_e12727_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard373 == 0.0)) && (locals.var_guard377 != 0.0)) {
        let assign18250_e12723: f64 = (locals.var_uc_novers / locals.var_uc_ndepm);
        let assign18250_e12724: f64 = (assign18250_e12723).sqrt();
        let assign18250_e12725: f64 = (locals.var_cnst0 * assign18250_e12724);
        (assign18250_e12725, ((locals.var_cnst0_dn0 * assign18250_e12724) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn0) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18250_e12724)))), ((locals.var_cnst0_dn2 * assign18250_e12724) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn2) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18250_e12724)))), ((locals.var_cnst0_dn4 * assign18250_e12724) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn4) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18250_e12724)))), ((locals.var_cnst0_dn5 * assign18250_e12724) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn5) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18250_e12724)))), ((locals.var_cnst0_dn6 * assign18250_e12724) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn6) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18250_e12724)))), ((locals.var_cnst0_dn7 * assign18250_e12724) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn7) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18250_e12724)))), ((locals.var_cnst0_dn8 * assign18250_e12724) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn8) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18250_e12724)))), ((locals.var_cnst0_dn9 * assign18250_e12724) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn9) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18250_e12724)))), ((locals.var_cnst0_dn10 * assign18250_e12724) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn10) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18250_e12724)))), ((locals.var_cnst0_dn13 * assign18250_e12724) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn13) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18250_e12724)))),)
    } else {
        (locals.var_cnst0overs, locals.var_cnst0overs_dn0, locals.var_cnst0overs_dn2, locals.var_cnst0overs_dn4, locals.var_cnst0overs_dn5, locals.var_cnst0overs_dn6, locals.var_cnst0overs_dn7, locals.var_cnst0overs_dn8, locals.var_cnst0overs_dn9, locals.var_cnst0overs_dn10, locals.var_cnst0overs_dn13,)
    }
};
        locals.var_cnst0overs = assign18250_e12727;
        locals.var_cnst0overs_dn0 = assign18250_e12727_d_n0;
        locals.var_cnst0overs_dn2 = assign18250_e12727_d_n2;
        locals.var_cnst0overs_dn4 = assign18250_e12727_d_n4;
        locals.var_cnst0overs_dn5 = assign18250_e12727_d_n5;
        locals.var_cnst0overs_dn6 = assign18250_e12727_d_n6;
        locals.var_cnst0overs_dn7 = assign18250_e12727_d_n7;
        locals.var_cnst0overs_dn8 = assign18250_e12727_d_n8;
        locals.var_cnst0overs_dn9 = assign18250_e12727_d_n9;
        locals.var_cnst0overs_dn10 = assign18250_e12727_d_n10;
        locals.var_cnst0overs_dn13 = assign18250_e12727_d_n13;
        locals.var_cnst0overs_rv = 0.0;

        let assign18260_e12730: f64 = if locals.var_uc_cordrift == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard378 = assign18260_e12730;
        locals.var_guard378_rv = 0.0;

        let assign18270_e12733: f64 = if locals.var_uc_rd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard379 = assign18270_e12733;
        locals.var_guard379_rv = 0.0;

        let (assign18280_e12757, assign18280_e12757_d_n0, assign18280_e12757_d_n2, assign18280_e12757_d_n4, assign18280_e12757_d_n5, assign18280_e12757_d_n6, assign18280_e12757_d_n7, assign18280_e12757_d_n8, assign18280_e12757_d_n9, assign18280_e12757_d_n10, assign18280_e12757_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard379 != 0.0)) {
        let assign18280_e12742: f64 = (p.p67 * locals.var_uc_rdslp1);
        let assign18280_e12744: f64 = (assign18280_e12742 * 1000000.0);
        let assign18280_e12746: f64 = (assign18280_e12744 + locals.var_uc_rdict1);
        let assign18280_e12747: f64 = (locals.var_rdtemp0 * assign18280_e12746);
        let assign18280_e12750: f64 = (p.p68 * p.p100);
        let assign18280_e12752: f64 = (assign18280_e12750 * 1000000.0);
        let assign18280_e12754: f64 = (assign18280_e12752 + p.p101);
        let assign18280_e12755: f64 = (assign18280_e12747 * assign18280_e12754);
        (assign18280_e12755, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign18280_e12757;
        locals.var_t2_dn0 = assign18280_e12757_d_n0;
        locals.var_t2_dn2 = assign18280_e12757_d_n2;
        locals.var_t2_dn4 = assign18280_e12757_d_n4;
        locals.var_t2_dn5 = assign18280_e12757_d_n5;
        locals.var_t2_dn6 = assign18280_e12757_d_n6;
        locals.var_t2_dn7 = assign18280_e12757_d_n7;
        locals.var_t2_dn8 = assign18280_e12757_d_n8;
        locals.var_t2_dn9 = assign18280_e12757_d_n9;
        locals.var_t2_dn10 = assign18280_e12757_d_n10;
        locals.var_t2_dn13 = assign18280_e12757_d_n13;
        locals.var_t2_rv = 0.0;

        let assign18290_e12760: f64 = if p.p39 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard380 = assign18290_e12760;
        locals.var_guard380_rv = 0.0;

        let (assign18300_e12780, assign18300_e12780_d_n0, assign18300_e12780_d_n2, assign18300_e12780_d_n4, assign18300_e12780_d_n5, assign18300_e12780_d_n6, assign18300_e12780_d_n7, assign18300_e12780_d_n8, assign18300_e12780_d_n9, assign18300_e12780_d_n10, assign18300_e12780_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard379 != 0.0)) && (locals.var_guard380 != 0.0)) {
        let assign18300_e12771: f64 = (locals.var_mks_rdtemp1 * locals.var_tdiff0);
        let assign18300_e12772: f64 = (locals.var_uc_rd + assign18300_e12771);
        let assign18300_e12775: f64 = (locals.var_mks_rdtemp2 * locals.var_tdiff0_2);
        let assign18300_e12776: f64 = (assign18300_e12772 + assign18300_e12775);
        let assign18300_e12778: f64 = (assign18300_e12776 * locals.var_t2);
        (assign18300_e12778, ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn0) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn0)) * locals.var_t2) + (assign18300_e12776 * locals.var_t2_dn0)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn2) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn2)) * locals.var_t2) + (assign18300_e12776 * locals.var_t2_dn2)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn4) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn4)) * locals.var_t2) + (assign18300_e12776 * locals.var_t2_dn4)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn5) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn5)) * locals.var_t2) + (assign18300_e12776 * locals.var_t2_dn5)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn6) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn6)) * locals.var_t2) + (assign18300_e12776 * locals.var_t2_dn6)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn7) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn7)) * locals.var_t2) + (assign18300_e12776 * locals.var_t2_dn7)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn8) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn8)) * locals.var_t2) + (assign18300_e12776 * locals.var_t2_dn8)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn9) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn9)) * locals.var_t2) + (assign18300_e12776 * locals.var_t2_dn9)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn10) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn10)) * locals.var_t2) + (assign18300_e12776 * locals.var_t2_dn10)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn13) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn13)) * locals.var_t2) + (assign18300_e12776 * locals.var_t2_dn13)),)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn13,)
    }
};
        locals.var_rde = assign18300_e12780;
        locals.var_rde_dn0 = assign18300_e12780_d_n0;
        locals.var_rde_dn2 = assign18300_e12780_d_n2;
        locals.var_rde_dn4 = assign18300_e12780_d_n4;
        locals.var_rde_dn5 = assign18300_e12780_d_n5;
        locals.var_rde_dn6 = assign18300_e12780_d_n6;
        locals.var_rde_dn7 = assign18300_e12780_d_n7;
        locals.var_rde_dn8 = assign18300_e12780_d_n8;
        locals.var_rde_dn9 = assign18300_e12780_d_n9;
        locals.var_rde_dn10 = assign18300_e12780_d_n10;
        locals.var_rde_dn13 = assign18300_e12780_d_n13;
        locals.var_rde_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_42(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign18310_e12798, assign18310_e12798_d_n0, assign18310_e12798_d_n2, assign18310_e12798_d_n4, assign18310_e12798_d_n5, assign18310_e12798_d_n6, assign18310_e12798_d_n7, assign18310_e12798_d_n8, assign18310_e12798_d_n9, assign18310_e12798_d_n10, assign18310_e12798_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard379 != 0.0)) && (locals.var_guard380 != 0.0)) {
        let assign18310_e12791: f64 = (0.005 * locals.var_uc_rd);
        let assign18310_e12792: f64 = (locals.var_rde - assign18310_e12791);
        let assign18310_e12795: f64 = (0.01 * locals.var_uc_rd);
        let assign18310_e12796: f64 = (assign18310_e12792 - assign18310_e12795);
        (assign18310_e12796, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign18310_e12798;
        locals.var_tmf1_dn0 = assign18310_e12798_d_n0;
        locals.var_tmf1_dn2 = assign18310_e12798_d_n2;
        locals.var_tmf1_dn4 = assign18310_e12798_d_n4;
        locals.var_tmf1_dn5 = assign18310_e12798_d_n5;
        locals.var_tmf1_dn6 = assign18310_e12798_d_n6;
        locals.var_tmf1_dn7 = assign18310_e12798_d_n7;
        locals.var_tmf1_dn8 = assign18310_e12798_d_n8;
        locals.var_tmf1_dn9 = assign18310_e12798_d_n9;
        locals.var_tmf1_dn10 = assign18310_e12798_d_n10;
        locals.var_tmf1_dn13 = assign18310_e12798_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign18320_e12816, assign18320_e12816_d_n0, assign18320_e12816_d_n2, assign18320_e12816_d_n4, assign18320_e12816_d_n5, assign18320_e12816_d_n6, assign18320_e12816_d_n7, assign18320_e12816_d_n8, assign18320_e12816_d_n9, assign18320_e12816_d_n10, assign18320_e12816_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard379 != 0.0)) && (locals.var_guard380 != 0.0)) {
        let assign18320_e12809: f64 = (0.005 * locals.var_uc_rd);
        let assign18320_e12810: f64 = (4.0 * assign18320_e12809);
        let assign18320_e12813: f64 = (0.01 * locals.var_uc_rd);
        let assign18320_e12814: f64 = (assign18320_e12810 * assign18320_e12813);
        (assign18320_e12814, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18320_e12816;
        locals.var_tmf2_dn0 = assign18320_e12816_d_n0;
        locals.var_tmf2_dn2 = assign18320_e12816_d_n2;
        locals.var_tmf2_dn4 = assign18320_e12816_d_n4;
        locals.var_tmf2_dn5 = assign18320_e12816_d_n5;
        locals.var_tmf2_dn6 = assign18320_e12816_d_n6;
        locals.var_tmf2_dn7 = assign18320_e12816_d_n7;
        locals.var_tmf2_dn8 = assign18320_e12816_d_n8;
        locals.var_tmf2_dn9 = assign18320_e12816_d_n9;
        locals.var_tmf2_dn10 = assign18320_e12816_d_n10;
        locals.var_tmf2_dn13 = assign18320_e12816_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign18330_e12832, assign18330_e12832_d_n0, assign18330_e12832_d_n2, assign18330_e12832_d_n4, assign18330_e12832_d_n5, assign18330_e12832_d_n6, assign18330_e12832_d_n7, assign18330_e12832_d_n8, assign18330_e12832_d_n9, assign18330_e12832_d_n10, assign18330_e12832_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard379 != 0.0)) && (locals.var_guard380 != 0.0)) {
        let (assign18330_e12830, assign18330_e12830_d_n0, assign18330_e12830_d_n2, assign18330_e12830_d_n4, assign18330_e12830_d_n5, assign18330_e12830_d_n6, assign18330_e12830_d_n7, assign18330_e12830_d_n8, assign18330_e12830_d_n9, assign18330_e12830_d_n10, assign18330_e12830_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign18330_e12829: f64 = (-locals.var_tmf2);
                (assign18330_e12829, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign18330_e12830, assign18330_e12830_d_n0, assign18330_e12830_d_n2, assign18330_e12830_d_n4, assign18330_e12830_d_n5, assign18330_e12830_d_n6, assign18330_e12830_d_n7, assign18330_e12830_d_n8, assign18330_e12830_d_n9, assign18330_e12830_d_n10, assign18330_e12830_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18330_e12832;
        locals.var_tmf2_dn0 = assign18330_e12832_d_n0;
        locals.var_tmf2_dn2 = assign18330_e12832_d_n2;
        locals.var_tmf2_dn4 = assign18330_e12832_d_n4;
        locals.var_tmf2_dn5 = assign18330_e12832_d_n5;
        locals.var_tmf2_dn6 = assign18330_e12832_d_n6;
        locals.var_tmf2_dn7 = assign18330_e12832_d_n7;
        locals.var_tmf2_dn8 = assign18330_e12832_d_n8;
        locals.var_tmf2_dn9 = assign18330_e12832_d_n9;
        locals.var_tmf2_dn10 = assign18330_e12832_d_n10;
        locals.var_tmf2_dn13 = assign18330_e12832_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign18340_e12847, assign18340_e12847_d_n0, assign18340_e12847_d_n2, assign18340_e12847_d_n4, assign18340_e12847_d_n5, assign18340_e12847_d_n6, assign18340_e12847_d_n7, assign18340_e12847_d_n8, assign18340_e12847_d_n9, assign18340_e12847_d_n10, assign18340_e12847_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard379 != 0.0)) && (locals.var_guard380 != 0.0)) {
        let assign18340_e12842: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18340_e12844: f64 = (assign18340_e12842 + locals.var_tmf2);
        let assign18340_e12845: f64 = (assign18340_e12844).sqrt();
        (assign18340_e12845, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18340_e12845)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18340_e12845)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18340_e12845)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18340_e12845)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18340_e12845)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18340_e12845)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18340_e12845)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18340_e12845)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18340_e12845)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign18340_e12845)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18340_e12847;
        locals.var_tmf2_dn0 = assign18340_e12847_d_n0;
        locals.var_tmf2_dn2 = assign18340_e12847_d_n2;
        locals.var_tmf2_dn4 = assign18340_e12847_d_n4;
        locals.var_tmf2_dn5 = assign18340_e12847_d_n5;
        locals.var_tmf2_dn6 = assign18340_e12847_d_n6;
        locals.var_tmf2_dn7 = assign18340_e12847_d_n7;
        locals.var_tmf2_dn8 = assign18340_e12847_d_n8;
        locals.var_tmf2_dn9 = assign18340_e12847_d_n9;
        locals.var_tmf2_dn10 = assign18340_e12847_d_n10;
        locals.var_tmf2_dn13 = assign18340_e12847_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign18350_e12863, assign18350_e12863_d_n0, assign18350_e12863_d_n2, assign18350_e12863_d_n4, assign18350_e12863_d_n5, assign18350_e12863_d_n6, assign18350_e12863_d_n7, assign18350_e12863_d_n8, assign18350_e12863_d_n9, assign18350_e12863_d_n10, assign18350_e12863_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard379 != 0.0)) && (locals.var_guard380 != 0.0)) {
        let assign18350_e12859: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18350_e12860: f64 = (1.0 + assign18350_e12859);
        let assign18350_e12861: f64 = (0.5 * assign18350_e12860);
        (assign18350_e12861, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign18350_e12863;
        locals.var_t0_dn0 = assign18350_e12863_d_n0;
        locals.var_t0_dn2 = assign18350_e12863_d_n2;
        locals.var_t0_dn4 = assign18350_e12863_d_n4;
        locals.var_t0_dn5 = assign18350_e12863_d_n5;
        locals.var_t0_dn6 = assign18350_e12863_d_n6;
        locals.var_t0_dn7 = assign18350_e12863_d_n7;
        locals.var_t0_dn8 = assign18350_e12863_d_n8;
        locals.var_t0_dn9 = assign18350_e12863_d_n9;
        locals.var_t0_dn10 = assign18350_e12863_d_n10;
        locals.var_t0_dn13 = assign18350_e12863_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign18360_e12881, assign18360_e12881_d_n0, assign18360_e12881_d_n2, assign18360_e12881_d_n4, assign18360_e12881_d_n5, assign18360_e12881_d_n6, assign18360_e12881_d_n7, assign18360_e12881_d_n8, assign18360_e12881_d_n9, assign18360_e12881_d_n10, assign18360_e12881_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard379 != 0.0)) && (locals.var_guard380 != 0.0)) {
        let assign18360_e12873: f64 = (0.005 * locals.var_uc_rd);
        let assign18360_e12877: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18360_e12878: f64 = (0.5 * assign18360_e12877);
        let assign18360_e12879: f64 = (assign18360_e12873 + assign18360_e12878);
        (assign18360_e12879, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn13,)
    }
};
        locals.var_rde = assign18360_e12881;
        locals.var_rde_dn0 = assign18360_e12881_d_n0;
        locals.var_rde_dn2 = assign18360_e12881_d_n2;
        locals.var_rde_dn4 = assign18360_e12881_d_n4;
        locals.var_rde_dn5 = assign18360_e12881_d_n5;
        locals.var_rde_dn6 = assign18360_e12881_d_n6;
        locals.var_rde_dn7 = assign18360_e12881_d_n7;
        locals.var_rde_dn8 = assign18360_e12881_d_n8;
        locals.var_rde_dn9 = assign18360_e12881_d_n9;
        locals.var_rde_dn10 = assign18360_e12881_d_n10;
        locals.var_rde_dn13 = assign18360_e12881_d_n13;
        locals.var_rde_rv = 0.0;

        let (assign18370_e12902, assign18370_e12902_d_n0, assign18370_e12902_d_n2, assign18370_e12902_d_n4, assign18370_e12902_d_n5, assign18370_e12902_d_n6, assign18370_e12902_d_n7, assign18370_e12902_d_n8, assign18370_e12902_d_n9, assign18370_e12902_d_n10, assign18370_e12902_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard379 != 0.0)) && (locals.var_guard380 == 0.0)) {
        let assign18370_e12893: f64 = (locals.var_mks_rdtemp1 * locals.var_tdiff);
        let assign18370_e12894: f64 = (locals.var_uc_rd + assign18370_e12893);
        let assign18370_e12897: f64 = (locals.var_mks_rdtemp2 * locals.var_tdiff_2);
        let assign18370_e12898: f64 = (assign18370_e12894 + assign18370_e12897);
        let assign18370_e12900: f64 = (assign18370_e12898 * locals.var_t2);
        (assign18370_e12900, ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn0) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn0)) * locals.var_t2) + (assign18370_e12898 * locals.var_t2_dn0)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn2) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn2)) * locals.var_t2) + (assign18370_e12898 * locals.var_t2_dn2)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn4) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn4)) * locals.var_t2) + (assign18370_e12898 * locals.var_t2_dn4)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn5) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn5)) * locals.var_t2) + (assign18370_e12898 * locals.var_t2_dn5)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn6) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn6)) * locals.var_t2) + (assign18370_e12898 * locals.var_t2_dn6)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn7) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn7)) * locals.var_t2) + (assign18370_e12898 * locals.var_t2_dn7)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn8) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn8)) * locals.var_t2) + (assign18370_e12898 * locals.var_t2_dn8)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn9) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn9)) * locals.var_t2) + (assign18370_e12898 * locals.var_t2_dn9)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn10) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn10)) * locals.var_t2) + (assign18370_e12898 * locals.var_t2_dn10)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn13) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn13)) * locals.var_t2) + (assign18370_e12898 * locals.var_t2_dn13)),)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn13,)
    }
};
        locals.var_rde = assign18370_e12902;
        locals.var_rde_dn0 = assign18370_e12902_d_n0;
        locals.var_rde_dn2 = assign18370_e12902_d_n2;
        locals.var_rde_dn4 = assign18370_e12902_d_n4;
        locals.var_rde_dn5 = assign18370_e12902_d_n5;
        locals.var_rde_dn6 = assign18370_e12902_d_n6;
        locals.var_rde_dn7 = assign18370_e12902_d_n7;
        locals.var_rde_dn8 = assign18370_e12902_d_n8;
        locals.var_rde_dn9 = assign18370_e12902_d_n9;
        locals.var_rde_dn10 = assign18370_e12902_d_n10;
        locals.var_rde_dn13 = assign18370_e12902_d_n13;
        locals.var_rde_rv = 0.0;

        let (assign18380_e12921, assign18380_e12921_d_n0, assign18380_e12921_d_n2, assign18380_e12921_d_n4, assign18380_e12921_d_n5, assign18380_e12921_d_n6, assign18380_e12921_d_n7, assign18380_e12921_d_n8, assign18380_e12921_d_n9, assign18380_e12921_d_n10, assign18380_e12921_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard379 != 0.0)) && (locals.var_guard380 == 0.0)) {
        let assign18380_e12914: f64 = (0.005 * locals.var_uc_rd);
        let assign18380_e12915: f64 = (locals.var_rde - assign18380_e12914);
        let assign18380_e12918: f64 = (0.01 * locals.var_uc_rd);
        let assign18380_e12919: f64 = (assign18380_e12915 - assign18380_e12918);
        (assign18380_e12919, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign18380_e12921;
        locals.var_tmf1_dn0 = assign18380_e12921_d_n0;
        locals.var_tmf1_dn2 = assign18380_e12921_d_n2;
        locals.var_tmf1_dn4 = assign18380_e12921_d_n4;
        locals.var_tmf1_dn5 = assign18380_e12921_d_n5;
        locals.var_tmf1_dn6 = assign18380_e12921_d_n6;
        locals.var_tmf1_dn7 = assign18380_e12921_d_n7;
        locals.var_tmf1_dn8 = assign18380_e12921_d_n8;
        locals.var_tmf1_dn9 = assign18380_e12921_d_n9;
        locals.var_tmf1_dn10 = assign18380_e12921_d_n10;
        locals.var_tmf1_dn13 = assign18380_e12921_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign18390_e12940, assign18390_e12940_d_n0, assign18390_e12940_d_n2, assign18390_e12940_d_n4, assign18390_e12940_d_n5, assign18390_e12940_d_n6, assign18390_e12940_d_n7, assign18390_e12940_d_n8, assign18390_e12940_d_n9, assign18390_e12940_d_n10, assign18390_e12940_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard379 != 0.0)) && (locals.var_guard380 == 0.0)) {
        let assign18390_e12933: f64 = (0.005 * locals.var_uc_rd);
        let assign18390_e12934: f64 = (4.0 * assign18390_e12933);
        let assign18390_e12937: f64 = (0.01 * locals.var_uc_rd);
        let assign18390_e12938: f64 = (assign18390_e12934 * assign18390_e12937);
        (assign18390_e12938, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18390_e12940;
        locals.var_tmf2_dn0 = assign18390_e12940_d_n0;
        locals.var_tmf2_dn2 = assign18390_e12940_d_n2;
        locals.var_tmf2_dn4 = assign18390_e12940_d_n4;
        locals.var_tmf2_dn5 = assign18390_e12940_d_n5;
        locals.var_tmf2_dn6 = assign18390_e12940_d_n6;
        locals.var_tmf2_dn7 = assign18390_e12940_d_n7;
        locals.var_tmf2_dn8 = assign18390_e12940_d_n8;
        locals.var_tmf2_dn9 = assign18390_e12940_d_n9;
        locals.var_tmf2_dn10 = assign18390_e12940_d_n10;
        locals.var_tmf2_dn13 = assign18390_e12940_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign18400_e12957, assign18400_e12957_d_n0, assign18400_e12957_d_n2, assign18400_e12957_d_n4, assign18400_e12957_d_n5, assign18400_e12957_d_n6, assign18400_e12957_d_n7, assign18400_e12957_d_n8, assign18400_e12957_d_n9, assign18400_e12957_d_n10, assign18400_e12957_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard379 != 0.0)) && (locals.var_guard380 == 0.0)) {
        let (assign18400_e12955, assign18400_e12955_d_n0, assign18400_e12955_d_n2, assign18400_e12955_d_n4, assign18400_e12955_d_n5, assign18400_e12955_d_n6, assign18400_e12955_d_n7, assign18400_e12955_d_n8, assign18400_e12955_d_n9, assign18400_e12955_d_n10, assign18400_e12955_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign18400_e12954: f64 = (-locals.var_tmf2);
                (assign18400_e12954, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign18400_e12955, assign18400_e12955_d_n0, assign18400_e12955_d_n2, assign18400_e12955_d_n4, assign18400_e12955_d_n5, assign18400_e12955_d_n6, assign18400_e12955_d_n7, assign18400_e12955_d_n8, assign18400_e12955_d_n9, assign18400_e12955_d_n10, assign18400_e12955_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18400_e12957;
        locals.var_tmf2_dn0 = assign18400_e12957_d_n0;
        locals.var_tmf2_dn2 = assign18400_e12957_d_n2;
        locals.var_tmf2_dn4 = assign18400_e12957_d_n4;
        locals.var_tmf2_dn5 = assign18400_e12957_d_n5;
        locals.var_tmf2_dn6 = assign18400_e12957_d_n6;
        locals.var_tmf2_dn7 = assign18400_e12957_d_n7;
        locals.var_tmf2_dn8 = assign18400_e12957_d_n8;
        locals.var_tmf2_dn9 = assign18400_e12957_d_n9;
        locals.var_tmf2_dn10 = assign18400_e12957_d_n10;
        locals.var_tmf2_dn13 = assign18400_e12957_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign18410_e12973, assign18410_e12973_d_n0, assign18410_e12973_d_n2, assign18410_e12973_d_n4, assign18410_e12973_d_n5, assign18410_e12973_d_n6, assign18410_e12973_d_n7, assign18410_e12973_d_n8, assign18410_e12973_d_n9, assign18410_e12973_d_n10, assign18410_e12973_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard379 != 0.0)) && (locals.var_guard380 == 0.0)) {
        let assign18410_e12968: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18410_e12970: f64 = (assign18410_e12968 + locals.var_tmf2);
        let assign18410_e12971: f64 = (assign18410_e12970).sqrt();
        (assign18410_e12971, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18410_e12971)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18410_e12971)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18410_e12971)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18410_e12971)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18410_e12971)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18410_e12971)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18410_e12971)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18410_e12971)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18410_e12971)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign18410_e12971)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18410_e12973;
        locals.var_tmf2_dn0 = assign18410_e12973_d_n0;
        locals.var_tmf2_dn2 = assign18410_e12973_d_n2;
        locals.var_tmf2_dn4 = assign18410_e12973_d_n4;
        locals.var_tmf2_dn5 = assign18410_e12973_d_n5;
        locals.var_tmf2_dn6 = assign18410_e12973_d_n6;
        locals.var_tmf2_dn7 = assign18410_e12973_d_n7;
        locals.var_tmf2_dn8 = assign18410_e12973_d_n8;
        locals.var_tmf2_dn9 = assign18410_e12973_d_n9;
        locals.var_tmf2_dn10 = assign18410_e12973_d_n10;
        locals.var_tmf2_dn13 = assign18410_e12973_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign18420_e12990, assign18420_e12990_d_n0, assign18420_e12990_d_n2, assign18420_e12990_d_n4, assign18420_e12990_d_n5, assign18420_e12990_d_n6, assign18420_e12990_d_n7, assign18420_e12990_d_n8, assign18420_e12990_d_n9, assign18420_e12990_d_n10, assign18420_e12990_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard379 != 0.0)) && (locals.var_guard380 == 0.0)) {
        let assign18420_e12986: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18420_e12987: f64 = (1.0 + assign18420_e12986);
        let assign18420_e12988: f64 = (0.5 * assign18420_e12987);
        (assign18420_e12988, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign18420_e12990;
        locals.var_t0_dn0 = assign18420_e12990_d_n0;
        locals.var_t0_dn2 = assign18420_e12990_d_n2;
        locals.var_t0_dn4 = assign18420_e12990_d_n4;
        locals.var_t0_dn5 = assign18420_e12990_d_n5;
        locals.var_t0_dn6 = assign18420_e12990_d_n6;
        locals.var_t0_dn7 = assign18420_e12990_d_n7;
        locals.var_t0_dn8 = assign18420_e12990_d_n8;
        locals.var_t0_dn9 = assign18420_e12990_d_n9;
        locals.var_t0_dn10 = assign18420_e12990_d_n10;
        locals.var_t0_dn13 = assign18420_e12990_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign18430_e13009, assign18430_e13009_d_n0, assign18430_e13009_d_n2, assign18430_e13009_d_n4, assign18430_e13009_d_n5, assign18430_e13009_d_n6, assign18430_e13009_d_n7, assign18430_e13009_d_n8, assign18430_e13009_d_n9, assign18430_e13009_d_n10, assign18430_e13009_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard379 != 0.0)) && (locals.var_guard380 == 0.0)) {
        let assign18430_e13001: f64 = (0.005 * locals.var_uc_rd);
        let assign18430_e13005: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18430_e13006: f64 = (0.5 * assign18430_e13005);
        let assign18430_e13007: f64 = (assign18430_e13001 + assign18430_e13006);
        (assign18430_e13007, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn13,)
    }
};
        locals.var_rde = assign18430_e13009;
        locals.var_rde_dn0 = assign18430_e13009_d_n0;
        locals.var_rde_dn2 = assign18430_e13009_d_n2;
        locals.var_rde_dn4 = assign18430_e13009_d_n4;
        locals.var_rde_dn5 = assign18430_e13009_d_n5;
        locals.var_rde_dn6 = assign18430_e13009_d_n6;
        locals.var_rde_dn7 = assign18430_e13009_d_n7;
        locals.var_rde_dn8 = assign18430_e13009_d_n8;
        locals.var_rde_dn9 = assign18430_e13009_d_n9;
        locals.var_rde_dn10 = assign18430_e13009_d_n10;
        locals.var_rde_dn13 = assign18430_e13009_d_n13;
        locals.var_rde_rv = 0.0;

        let (assign18440_e13018, assign18440_e13018_d_n0, assign18440_e13018_d_n2, assign18440_e13018_d_n4, assign18440_e13018_d_n5, assign18440_e13018_d_n6, assign18440_e13018_d_n7, assign18440_e13018_d_n8, assign18440_e13018_d_n9, assign18440_e13018_d_n10, assign18440_e13018_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard379 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn13,)
    }
};
        locals.var_rde = assign18440_e13018;
        locals.var_rde_dn0 = assign18440_e13018_d_n0;
        locals.var_rde_dn2 = assign18440_e13018_d_n2;
        locals.var_rde_dn4 = assign18440_e13018_d_n4;
        locals.var_rde_dn5 = assign18440_e13018_d_n5;
        locals.var_rde_dn6 = assign18440_e13018_d_n6;
        locals.var_rde_dn7 = assign18440_e13018_d_n7;
        locals.var_rde_dn8 = assign18440_e13018_d_n8;
        locals.var_rde_dn9 = assign18440_e13018_d_n9;
        locals.var_rde_dn10 = assign18440_e13018_d_n10;
        locals.var_rde_dn13 = assign18440_e13018_d_n13;
        locals.var_rde_rv = 0.0;

        let assign18450_e13021: f64 = if locals.var_uc_rs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard381 = assign18450_e13021;
        locals.var_guard381_rv = 0.0;

        let (assign18460_e13045, assign18460_e13045_d_n0, assign18460_e13045_d_n2, assign18460_e13045_d_n4, assign18460_e13045_d_n5, assign18460_e13045_d_n6, assign18460_e13045_d_n7, assign18460_e13045_d_n8, assign18460_e13045_d_n9, assign18460_e13045_d_n10, assign18460_e13045_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard381 != 0.0)) {
        let assign18460_e13030: f64 = (p.p69 * locals.var_uc_rdslp1);
        let assign18460_e13032: f64 = (assign18460_e13030 * 1000000.0);
        let assign18460_e13034: f64 = (assign18460_e13032 + locals.var_uc_rdict1);
        let assign18460_e13035: f64 = (locals.var_rdtemp0 * assign18460_e13034);
        let assign18460_e13038: f64 = (p.p70 * p.p100);
        let assign18460_e13040: f64 = (assign18460_e13038 * 1000000.0);
        let assign18460_e13042: f64 = (assign18460_e13040 + p.p101);
        let assign18460_e13043: f64 = (assign18460_e13035 * assign18460_e13042);
        (assign18460_e13043, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign18460_e13045;
        locals.var_t2_dn0 = assign18460_e13045_d_n0;
        locals.var_t2_dn2 = assign18460_e13045_d_n2;
        locals.var_t2_dn4 = assign18460_e13045_d_n4;
        locals.var_t2_dn5 = assign18460_e13045_d_n5;
        locals.var_t2_dn6 = assign18460_e13045_d_n6;
        locals.var_t2_dn7 = assign18460_e13045_d_n7;
        locals.var_t2_dn8 = assign18460_e13045_d_n8;
        locals.var_t2_dn9 = assign18460_e13045_d_n9;
        locals.var_t2_dn10 = assign18460_e13045_d_n10;
        locals.var_t2_dn13 = assign18460_e13045_d_n13;
        locals.var_t2_rv = 0.0;

        let assign18470_e13048: f64 = if p.p39 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard382 = assign18470_e13048;
        locals.var_guard382_rv = 0.0;

        let (assign18480_e13068, assign18480_e13068_d_n0, assign18480_e13068_d_n2, assign18480_e13068_d_n4, assign18480_e13068_d_n5, assign18480_e13068_d_n6, assign18480_e13068_d_n7, assign18480_e13068_d_n8, assign18480_e13068_d_n9, assign18480_e13068_d_n10, assign18480_e13068_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard381 != 0.0)) && (locals.var_guard382 != 0.0)) {
        let assign18480_e13059: f64 = (locals.var_mks_rdtemp1 * locals.var_tdiff0);
        let assign18480_e13060: f64 = (locals.var_uc_rs + assign18480_e13059);
        let assign18480_e13063: f64 = (locals.var_mks_rdtemp2 * locals.var_tdiff0_2);
        let assign18480_e13064: f64 = (assign18480_e13060 + assign18480_e13063);
        let assign18480_e13066: f64 = (assign18480_e13064 * locals.var_t2);
        (assign18480_e13066, ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn0) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn0)) * locals.var_t2) + (assign18480_e13064 * locals.var_t2_dn0)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn2) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn2)) * locals.var_t2) + (assign18480_e13064 * locals.var_t2_dn2)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn4) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn4)) * locals.var_t2) + (assign18480_e13064 * locals.var_t2_dn4)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn5) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn5)) * locals.var_t2) + (assign18480_e13064 * locals.var_t2_dn5)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn6) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn6)) * locals.var_t2) + (assign18480_e13064 * locals.var_t2_dn6)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn7) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn7)) * locals.var_t2) + (assign18480_e13064 * locals.var_t2_dn7)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn8) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn8)) * locals.var_t2) + (assign18480_e13064 * locals.var_t2_dn8)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn9) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn9)) * locals.var_t2) + (assign18480_e13064 * locals.var_t2_dn9)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn10) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn10)) * locals.var_t2) + (assign18480_e13064 * locals.var_t2_dn10)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn13) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn13)) * locals.var_t2) + (assign18480_e13064 * locals.var_t2_dn13)),)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn13,)
    }
};
        locals.var_rse = assign18480_e13068;
        locals.var_rse_dn0 = assign18480_e13068_d_n0;
        locals.var_rse_dn2 = assign18480_e13068_d_n2;
        locals.var_rse_dn4 = assign18480_e13068_d_n4;
        locals.var_rse_dn5 = assign18480_e13068_d_n5;
        locals.var_rse_dn6 = assign18480_e13068_d_n6;
        locals.var_rse_dn7 = assign18480_e13068_d_n7;
        locals.var_rse_dn8 = assign18480_e13068_d_n8;
        locals.var_rse_dn9 = assign18480_e13068_d_n9;
        locals.var_rse_dn10 = assign18480_e13068_d_n10;
        locals.var_rse_dn13 = assign18480_e13068_d_n13;
        locals.var_rse_rv = 0.0;

        let (assign18490_e13086, assign18490_e13086_d_n0, assign18490_e13086_d_n2, assign18490_e13086_d_n4, assign18490_e13086_d_n5, assign18490_e13086_d_n6, assign18490_e13086_d_n7, assign18490_e13086_d_n8, assign18490_e13086_d_n9, assign18490_e13086_d_n10, assign18490_e13086_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard381 != 0.0)) && (locals.var_guard382 != 0.0)) {
        let assign18490_e13079: f64 = (0.005 * locals.var_uc_rs);
        let assign18490_e13080: f64 = (locals.var_rse - assign18490_e13079);
        let assign18490_e13083: f64 = (0.01 * locals.var_uc_rs);
        let assign18490_e13084: f64 = (assign18490_e13080 - assign18490_e13083);
        (assign18490_e13084, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign18490_e13086;
        locals.var_tmf1_dn0 = assign18490_e13086_d_n0;
        locals.var_tmf1_dn2 = assign18490_e13086_d_n2;
        locals.var_tmf1_dn4 = assign18490_e13086_d_n4;
        locals.var_tmf1_dn5 = assign18490_e13086_d_n5;
        locals.var_tmf1_dn6 = assign18490_e13086_d_n6;
        locals.var_tmf1_dn7 = assign18490_e13086_d_n7;
        locals.var_tmf1_dn8 = assign18490_e13086_d_n8;
        locals.var_tmf1_dn9 = assign18490_e13086_d_n9;
        locals.var_tmf1_dn10 = assign18490_e13086_d_n10;
        locals.var_tmf1_dn13 = assign18490_e13086_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign18500_e13104, assign18500_e13104_d_n0, assign18500_e13104_d_n2, assign18500_e13104_d_n4, assign18500_e13104_d_n5, assign18500_e13104_d_n6, assign18500_e13104_d_n7, assign18500_e13104_d_n8, assign18500_e13104_d_n9, assign18500_e13104_d_n10, assign18500_e13104_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard381 != 0.0)) && (locals.var_guard382 != 0.0)) {
        let assign18500_e13097: f64 = (0.005 * locals.var_uc_rs);
        let assign18500_e13098: f64 = (4.0 * assign18500_e13097);
        let assign18500_e13101: f64 = (0.01 * locals.var_uc_rs);
        let assign18500_e13102: f64 = (assign18500_e13098 * assign18500_e13101);
        (assign18500_e13102, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18500_e13104;
        locals.var_tmf2_dn0 = assign18500_e13104_d_n0;
        locals.var_tmf2_dn2 = assign18500_e13104_d_n2;
        locals.var_tmf2_dn4 = assign18500_e13104_d_n4;
        locals.var_tmf2_dn5 = assign18500_e13104_d_n5;
        locals.var_tmf2_dn6 = assign18500_e13104_d_n6;
        locals.var_tmf2_dn7 = assign18500_e13104_d_n7;
        locals.var_tmf2_dn8 = assign18500_e13104_d_n8;
        locals.var_tmf2_dn9 = assign18500_e13104_d_n9;
        locals.var_tmf2_dn10 = assign18500_e13104_d_n10;
        locals.var_tmf2_dn13 = assign18500_e13104_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign18510_e13120, assign18510_e13120_d_n0, assign18510_e13120_d_n2, assign18510_e13120_d_n4, assign18510_e13120_d_n5, assign18510_e13120_d_n6, assign18510_e13120_d_n7, assign18510_e13120_d_n8, assign18510_e13120_d_n9, assign18510_e13120_d_n10, assign18510_e13120_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard381 != 0.0)) && (locals.var_guard382 != 0.0)) {
        let (assign18510_e13118, assign18510_e13118_d_n0, assign18510_e13118_d_n2, assign18510_e13118_d_n4, assign18510_e13118_d_n5, assign18510_e13118_d_n6, assign18510_e13118_d_n7, assign18510_e13118_d_n8, assign18510_e13118_d_n9, assign18510_e13118_d_n10, assign18510_e13118_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign18510_e13117: f64 = (-locals.var_tmf2);
                (assign18510_e13117, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign18510_e13118, assign18510_e13118_d_n0, assign18510_e13118_d_n2, assign18510_e13118_d_n4, assign18510_e13118_d_n5, assign18510_e13118_d_n6, assign18510_e13118_d_n7, assign18510_e13118_d_n8, assign18510_e13118_d_n9, assign18510_e13118_d_n10, assign18510_e13118_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18510_e13120;
        locals.var_tmf2_dn0 = assign18510_e13120_d_n0;
        locals.var_tmf2_dn2 = assign18510_e13120_d_n2;
        locals.var_tmf2_dn4 = assign18510_e13120_d_n4;
        locals.var_tmf2_dn5 = assign18510_e13120_d_n5;
        locals.var_tmf2_dn6 = assign18510_e13120_d_n6;
        locals.var_tmf2_dn7 = assign18510_e13120_d_n7;
        locals.var_tmf2_dn8 = assign18510_e13120_d_n8;
        locals.var_tmf2_dn9 = assign18510_e13120_d_n9;
        locals.var_tmf2_dn10 = assign18510_e13120_d_n10;
        locals.var_tmf2_dn13 = assign18510_e13120_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign18520_e13135, assign18520_e13135_d_n0, assign18520_e13135_d_n2, assign18520_e13135_d_n4, assign18520_e13135_d_n5, assign18520_e13135_d_n6, assign18520_e13135_d_n7, assign18520_e13135_d_n8, assign18520_e13135_d_n9, assign18520_e13135_d_n10, assign18520_e13135_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard381 != 0.0)) && (locals.var_guard382 != 0.0)) {
        let assign18520_e13130: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18520_e13132: f64 = (assign18520_e13130 + locals.var_tmf2);
        let assign18520_e13133: f64 = (assign18520_e13132).sqrt();
        (assign18520_e13133, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18520_e13133)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18520_e13133)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18520_e13133)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18520_e13133)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18520_e13133)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18520_e13133)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18520_e13133)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18520_e13133)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18520_e13133)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign18520_e13133)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18520_e13135;
        locals.var_tmf2_dn0 = assign18520_e13135_d_n0;
        locals.var_tmf2_dn2 = assign18520_e13135_d_n2;
        locals.var_tmf2_dn4 = assign18520_e13135_d_n4;
        locals.var_tmf2_dn5 = assign18520_e13135_d_n5;
        locals.var_tmf2_dn6 = assign18520_e13135_d_n6;
        locals.var_tmf2_dn7 = assign18520_e13135_d_n7;
        locals.var_tmf2_dn8 = assign18520_e13135_d_n8;
        locals.var_tmf2_dn9 = assign18520_e13135_d_n9;
        locals.var_tmf2_dn10 = assign18520_e13135_d_n10;
        locals.var_tmf2_dn13 = assign18520_e13135_d_n13;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_43(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign18530_e13151, assign18530_e13151_d_n0, assign18530_e13151_d_n2, assign18530_e13151_d_n4, assign18530_e13151_d_n5, assign18530_e13151_d_n6, assign18530_e13151_d_n7, assign18530_e13151_d_n8, assign18530_e13151_d_n9, assign18530_e13151_d_n10, assign18530_e13151_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard381 != 0.0)) && (locals.var_guard382 != 0.0)) {
        let assign18530_e13147: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18530_e13148: f64 = (1.0 + assign18530_e13147);
        let assign18530_e13149: f64 = (0.5 * assign18530_e13148);
        (assign18530_e13149, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign18530_e13151;
        locals.var_t0_dn0 = assign18530_e13151_d_n0;
        locals.var_t0_dn2 = assign18530_e13151_d_n2;
        locals.var_t0_dn4 = assign18530_e13151_d_n4;
        locals.var_t0_dn5 = assign18530_e13151_d_n5;
        locals.var_t0_dn6 = assign18530_e13151_d_n6;
        locals.var_t0_dn7 = assign18530_e13151_d_n7;
        locals.var_t0_dn8 = assign18530_e13151_d_n8;
        locals.var_t0_dn9 = assign18530_e13151_d_n9;
        locals.var_t0_dn10 = assign18530_e13151_d_n10;
        locals.var_t0_dn13 = assign18530_e13151_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign18540_e13169, assign18540_e13169_d_n0, assign18540_e13169_d_n2, assign18540_e13169_d_n4, assign18540_e13169_d_n5, assign18540_e13169_d_n6, assign18540_e13169_d_n7, assign18540_e13169_d_n8, assign18540_e13169_d_n9, assign18540_e13169_d_n10, assign18540_e13169_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard381 != 0.0)) && (locals.var_guard382 != 0.0)) {
        let assign18540_e13161: f64 = (0.005 * locals.var_uc_rs);
        let assign18540_e13165: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18540_e13166: f64 = (0.5 * assign18540_e13165);
        let assign18540_e13167: f64 = (assign18540_e13161 + assign18540_e13166);
        (assign18540_e13167, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn13,)
    }
};
        locals.var_rse = assign18540_e13169;
        locals.var_rse_dn0 = assign18540_e13169_d_n0;
        locals.var_rse_dn2 = assign18540_e13169_d_n2;
        locals.var_rse_dn4 = assign18540_e13169_d_n4;
        locals.var_rse_dn5 = assign18540_e13169_d_n5;
        locals.var_rse_dn6 = assign18540_e13169_d_n6;
        locals.var_rse_dn7 = assign18540_e13169_d_n7;
        locals.var_rse_dn8 = assign18540_e13169_d_n8;
        locals.var_rse_dn9 = assign18540_e13169_d_n9;
        locals.var_rse_dn10 = assign18540_e13169_d_n10;
        locals.var_rse_dn13 = assign18540_e13169_d_n13;
        locals.var_rse_rv = 0.0;

        let (assign18550_e13190, assign18550_e13190_d_n0, assign18550_e13190_d_n2, assign18550_e13190_d_n4, assign18550_e13190_d_n5, assign18550_e13190_d_n6, assign18550_e13190_d_n7, assign18550_e13190_d_n8, assign18550_e13190_d_n9, assign18550_e13190_d_n10, assign18550_e13190_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard381 != 0.0)) && (locals.var_guard382 == 0.0)) {
        let assign18550_e13181: f64 = (locals.var_mks_rdtemp1 * locals.var_tdiff);
        let assign18550_e13182: f64 = (locals.var_uc_rs + assign18550_e13181);
        let assign18550_e13185: f64 = (locals.var_mks_rdtemp2 * locals.var_tdiff_2);
        let assign18550_e13186: f64 = (assign18550_e13182 + assign18550_e13185);
        let assign18550_e13188: f64 = (assign18550_e13186 * locals.var_t2);
        (assign18550_e13188, ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn0) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn0)) * locals.var_t2) + (assign18550_e13186 * locals.var_t2_dn0)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn2) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn2)) * locals.var_t2) + (assign18550_e13186 * locals.var_t2_dn2)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn4) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn4)) * locals.var_t2) + (assign18550_e13186 * locals.var_t2_dn4)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn5) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn5)) * locals.var_t2) + (assign18550_e13186 * locals.var_t2_dn5)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn6) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn6)) * locals.var_t2) + (assign18550_e13186 * locals.var_t2_dn6)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn7) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn7)) * locals.var_t2) + (assign18550_e13186 * locals.var_t2_dn7)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn8) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn8)) * locals.var_t2) + (assign18550_e13186 * locals.var_t2_dn8)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn9) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn9)) * locals.var_t2) + (assign18550_e13186 * locals.var_t2_dn9)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn10) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn10)) * locals.var_t2) + (assign18550_e13186 * locals.var_t2_dn10)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn13) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn13)) * locals.var_t2) + (assign18550_e13186 * locals.var_t2_dn13)),)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn13,)
    }
};
        locals.var_rse = assign18550_e13190;
        locals.var_rse_dn0 = assign18550_e13190_d_n0;
        locals.var_rse_dn2 = assign18550_e13190_d_n2;
        locals.var_rse_dn4 = assign18550_e13190_d_n4;
        locals.var_rse_dn5 = assign18550_e13190_d_n5;
        locals.var_rse_dn6 = assign18550_e13190_d_n6;
        locals.var_rse_dn7 = assign18550_e13190_d_n7;
        locals.var_rse_dn8 = assign18550_e13190_d_n8;
        locals.var_rse_dn9 = assign18550_e13190_d_n9;
        locals.var_rse_dn10 = assign18550_e13190_d_n10;
        locals.var_rse_dn13 = assign18550_e13190_d_n13;
        locals.var_rse_rv = 0.0;

        let (assign18560_e13209, assign18560_e13209_d_n0, assign18560_e13209_d_n2, assign18560_e13209_d_n4, assign18560_e13209_d_n5, assign18560_e13209_d_n6, assign18560_e13209_d_n7, assign18560_e13209_d_n8, assign18560_e13209_d_n9, assign18560_e13209_d_n10, assign18560_e13209_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard381 != 0.0)) && (locals.var_guard382 == 0.0)) {
        let assign18560_e13202: f64 = (0.005 * locals.var_uc_rs);
        let assign18560_e13203: f64 = (locals.var_rse - assign18560_e13202);
        let assign18560_e13206: f64 = (0.01 * locals.var_uc_rs);
        let assign18560_e13207: f64 = (assign18560_e13203 - assign18560_e13206);
        (assign18560_e13207, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign18560_e13209;
        locals.var_tmf1_dn0 = assign18560_e13209_d_n0;
        locals.var_tmf1_dn2 = assign18560_e13209_d_n2;
        locals.var_tmf1_dn4 = assign18560_e13209_d_n4;
        locals.var_tmf1_dn5 = assign18560_e13209_d_n5;
        locals.var_tmf1_dn6 = assign18560_e13209_d_n6;
        locals.var_tmf1_dn7 = assign18560_e13209_d_n7;
        locals.var_tmf1_dn8 = assign18560_e13209_d_n8;
        locals.var_tmf1_dn9 = assign18560_e13209_d_n9;
        locals.var_tmf1_dn10 = assign18560_e13209_d_n10;
        locals.var_tmf1_dn13 = assign18560_e13209_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign18570_e13228, assign18570_e13228_d_n0, assign18570_e13228_d_n2, assign18570_e13228_d_n4, assign18570_e13228_d_n5, assign18570_e13228_d_n6, assign18570_e13228_d_n7, assign18570_e13228_d_n8, assign18570_e13228_d_n9, assign18570_e13228_d_n10, assign18570_e13228_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard381 != 0.0)) && (locals.var_guard382 == 0.0)) {
        let assign18570_e13221: f64 = (0.005 * locals.var_uc_rs);
        let assign18570_e13222: f64 = (4.0 * assign18570_e13221);
        let assign18570_e13225: f64 = (0.01 * locals.var_uc_rs);
        let assign18570_e13226: f64 = (assign18570_e13222 * assign18570_e13225);
        (assign18570_e13226, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18570_e13228;
        locals.var_tmf2_dn0 = assign18570_e13228_d_n0;
        locals.var_tmf2_dn2 = assign18570_e13228_d_n2;
        locals.var_tmf2_dn4 = assign18570_e13228_d_n4;
        locals.var_tmf2_dn5 = assign18570_e13228_d_n5;
        locals.var_tmf2_dn6 = assign18570_e13228_d_n6;
        locals.var_tmf2_dn7 = assign18570_e13228_d_n7;
        locals.var_tmf2_dn8 = assign18570_e13228_d_n8;
        locals.var_tmf2_dn9 = assign18570_e13228_d_n9;
        locals.var_tmf2_dn10 = assign18570_e13228_d_n10;
        locals.var_tmf2_dn13 = assign18570_e13228_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign18580_e13245, assign18580_e13245_d_n0, assign18580_e13245_d_n2, assign18580_e13245_d_n4, assign18580_e13245_d_n5, assign18580_e13245_d_n6, assign18580_e13245_d_n7, assign18580_e13245_d_n8, assign18580_e13245_d_n9, assign18580_e13245_d_n10, assign18580_e13245_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard381 != 0.0)) && (locals.var_guard382 == 0.0)) {
        let (assign18580_e13243, assign18580_e13243_d_n0, assign18580_e13243_d_n2, assign18580_e13243_d_n4, assign18580_e13243_d_n5, assign18580_e13243_d_n6, assign18580_e13243_d_n7, assign18580_e13243_d_n8, assign18580_e13243_d_n9, assign18580_e13243_d_n10, assign18580_e13243_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign18580_e13242: f64 = (-locals.var_tmf2);
                (assign18580_e13242, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign18580_e13243, assign18580_e13243_d_n0, assign18580_e13243_d_n2, assign18580_e13243_d_n4, assign18580_e13243_d_n5, assign18580_e13243_d_n6, assign18580_e13243_d_n7, assign18580_e13243_d_n8, assign18580_e13243_d_n9, assign18580_e13243_d_n10, assign18580_e13243_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18580_e13245;
        locals.var_tmf2_dn0 = assign18580_e13245_d_n0;
        locals.var_tmf2_dn2 = assign18580_e13245_d_n2;
        locals.var_tmf2_dn4 = assign18580_e13245_d_n4;
        locals.var_tmf2_dn5 = assign18580_e13245_d_n5;
        locals.var_tmf2_dn6 = assign18580_e13245_d_n6;
        locals.var_tmf2_dn7 = assign18580_e13245_d_n7;
        locals.var_tmf2_dn8 = assign18580_e13245_d_n8;
        locals.var_tmf2_dn9 = assign18580_e13245_d_n9;
        locals.var_tmf2_dn10 = assign18580_e13245_d_n10;
        locals.var_tmf2_dn13 = assign18580_e13245_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign18590_e13261, assign18590_e13261_d_n0, assign18590_e13261_d_n2, assign18590_e13261_d_n4, assign18590_e13261_d_n5, assign18590_e13261_d_n6, assign18590_e13261_d_n7, assign18590_e13261_d_n8, assign18590_e13261_d_n9, assign18590_e13261_d_n10, assign18590_e13261_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard381 != 0.0)) && (locals.var_guard382 == 0.0)) {
        let assign18590_e13256: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18590_e13258: f64 = (assign18590_e13256 + locals.var_tmf2);
        let assign18590_e13259: f64 = (assign18590_e13258).sqrt();
        (assign18590_e13259, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18590_e13259)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18590_e13259)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18590_e13259)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18590_e13259)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18590_e13259)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18590_e13259)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18590_e13259)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18590_e13259)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18590_e13259)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign18590_e13259)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18590_e13261;
        locals.var_tmf2_dn0 = assign18590_e13261_d_n0;
        locals.var_tmf2_dn2 = assign18590_e13261_d_n2;
        locals.var_tmf2_dn4 = assign18590_e13261_d_n4;
        locals.var_tmf2_dn5 = assign18590_e13261_d_n5;
        locals.var_tmf2_dn6 = assign18590_e13261_d_n6;
        locals.var_tmf2_dn7 = assign18590_e13261_d_n7;
        locals.var_tmf2_dn8 = assign18590_e13261_d_n8;
        locals.var_tmf2_dn9 = assign18590_e13261_d_n9;
        locals.var_tmf2_dn10 = assign18590_e13261_d_n10;
        locals.var_tmf2_dn13 = assign18590_e13261_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign18600_e13278, assign18600_e13278_d_n0, assign18600_e13278_d_n2, assign18600_e13278_d_n4, assign18600_e13278_d_n5, assign18600_e13278_d_n6, assign18600_e13278_d_n7, assign18600_e13278_d_n8, assign18600_e13278_d_n9, assign18600_e13278_d_n10, assign18600_e13278_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard381 != 0.0)) && (locals.var_guard382 == 0.0)) {
        let assign18600_e13274: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18600_e13275: f64 = (1.0 + assign18600_e13274);
        let assign18600_e13276: f64 = (0.5 * assign18600_e13275);
        (assign18600_e13276, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign18600_e13278;
        locals.var_t0_dn0 = assign18600_e13278_d_n0;
        locals.var_t0_dn2 = assign18600_e13278_d_n2;
        locals.var_t0_dn4 = assign18600_e13278_d_n4;
        locals.var_t0_dn5 = assign18600_e13278_d_n5;
        locals.var_t0_dn6 = assign18600_e13278_d_n6;
        locals.var_t0_dn7 = assign18600_e13278_d_n7;
        locals.var_t0_dn8 = assign18600_e13278_d_n8;
        locals.var_t0_dn9 = assign18600_e13278_d_n9;
        locals.var_t0_dn10 = assign18600_e13278_d_n10;
        locals.var_t0_dn13 = assign18600_e13278_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign18610_e13297, assign18610_e13297_d_n0, assign18610_e13297_d_n2, assign18610_e13297_d_n4, assign18610_e13297_d_n5, assign18610_e13297_d_n6, assign18610_e13297_d_n7, assign18610_e13297_d_n8, assign18610_e13297_d_n9, assign18610_e13297_d_n10, assign18610_e13297_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard381 != 0.0)) && (locals.var_guard382 == 0.0)) {
        let assign18610_e13289: f64 = (0.005 * locals.var_uc_rs);
        let assign18610_e13293: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18610_e13294: f64 = (0.5 * assign18610_e13293);
        let assign18610_e13295: f64 = (assign18610_e13289 + assign18610_e13294);
        (assign18610_e13295, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn13,)
    }
};
        locals.var_rse = assign18610_e13297;
        locals.var_rse_dn0 = assign18610_e13297_d_n0;
        locals.var_rse_dn2 = assign18610_e13297_d_n2;
        locals.var_rse_dn4 = assign18610_e13297_d_n4;
        locals.var_rse_dn5 = assign18610_e13297_d_n5;
        locals.var_rse_dn6 = assign18610_e13297_d_n6;
        locals.var_rse_dn7 = assign18610_e13297_d_n7;
        locals.var_rse_dn8 = assign18610_e13297_d_n8;
        locals.var_rse_dn9 = assign18610_e13297_d_n9;
        locals.var_rse_dn10 = assign18610_e13297_d_n10;
        locals.var_rse_dn13 = assign18610_e13297_d_n13;
        locals.var_rse_rv = 0.0;

        let (assign18620_e13306, assign18620_e13306_d_n0, assign18620_e13306_d_n2, assign18620_e13306_d_n4, assign18620_e13306_d_n5, assign18620_e13306_d_n6, assign18620_e13306_d_n7, assign18620_e13306_d_n8, assign18620_e13306_d_n9, assign18620_e13306_d_n10, assign18620_e13306_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard381 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn13,)
    }
};
        locals.var_rse = assign18620_e13306;
        locals.var_rse_dn0 = assign18620_e13306_d_n0;
        locals.var_rse_dn2 = assign18620_e13306_d_n2;
        locals.var_rse_dn4 = assign18620_e13306_d_n4;
        locals.var_rse_dn5 = assign18620_e13306_d_n5;
        locals.var_rse_dn6 = assign18620_e13306_d_n6;
        locals.var_rse_dn7 = assign18620_e13306_d_n7;
        locals.var_rse_dn8 = assign18620_e13306_d_n8;
        locals.var_rse_dn9 = assign18620_e13306_d_n9;
        locals.var_rse_dn10 = assign18620_e13306_d_n10;
        locals.var_rse_dn13 = assign18620_e13306_d_n13;
        locals.var_rse_rv = 0.0;

        let assign18630_e13309: f64 = if locals.var_uc_rdvd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard383 = assign18630_e13309;
        locals.var_guard383_rv = 0.0;

        let (assign18640_e13333, assign18640_e13333_d_n0, assign18640_e13333_d_n2, assign18640_e13333_d_n4, assign18640_e13333_d_n5, assign18640_e13333_d_n6, assign18640_e13333_d_n7, assign18640_e13333_d_n8, assign18640_e13333_d_n9, assign18640_e13333_d_n10, assign18640_e13333_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18640_e13318: f64 = (p.p67 * locals.var_uc_rdslp1);
        let assign18640_e13320: f64 = (assign18640_e13318 * 1000000.0);
        let assign18640_e13322: f64 = (assign18640_e13320 + locals.var_uc_rdict1);
        let assign18640_e13323: f64 = (locals.var_rdvdtemp0 * assign18640_e13322);
        let assign18640_e13326: f64 = (p.p68 * p.p100);
        let assign18640_e13328: f64 = (assign18640_e13326 * 1000000.0);
        let assign18640_e13330: f64 = (assign18640_e13328 + p.p101);
        let assign18640_e13331: f64 = (assign18640_e13323 * assign18640_e13330);
        (assign18640_e13331, ((locals.var_rdvdtemp0_dn0 * assign18640_e13322) * assign18640_e13330), ((locals.var_rdvdtemp0_dn2 * assign18640_e13322) * assign18640_e13330), ((locals.var_rdvdtemp0_dn4 * assign18640_e13322) * assign18640_e13330), ((locals.var_rdvdtemp0_dn5 * assign18640_e13322) * assign18640_e13330), ((locals.var_rdvdtemp0_dn6 * assign18640_e13322) * assign18640_e13330), ((locals.var_rdvdtemp0_dn7 * assign18640_e13322) * assign18640_e13330), ((locals.var_rdvdtemp0_dn8 * assign18640_e13322) * assign18640_e13330), ((locals.var_rdvdtemp0_dn9 * assign18640_e13322) * assign18640_e13330), ((locals.var_rdvdtemp0_dn10 * assign18640_e13322) * assign18640_e13330), ((locals.var_rdvdtemp0_dn13 * assign18640_e13322) * assign18640_e13330),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign18640_e13333;
        locals.var_t4_dn0 = assign18640_e13333_d_n0;
        locals.var_t4_dn2 = assign18640_e13333_d_n2;
        locals.var_t4_dn4 = assign18640_e13333_d_n4;
        locals.var_t4_dn5 = assign18640_e13333_d_n5;
        locals.var_t4_dn6 = assign18640_e13333_d_n6;
        locals.var_t4_dn7 = assign18640_e13333_d_n7;
        locals.var_t4_dn8 = assign18640_e13333_d_n8;
        locals.var_t4_dn9 = assign18640_e13333_d_n9;
        locals.var_t4_dn10 = assign18640_e13333_d_n10;
        locals.var_t4_dn13 = assign18640_e13333_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign18650_e13347, assign18650_e13347_d_n0, assign18650_e13347_d_n2, assign18650_e13347_d_n4, assign18650_e13347_d_n5, assign18650_e13347_d_n6, assign18650_e13347_d_n7, assign18650_e13347_d_n8, assign18650_e13347_d_n9, assign18650_e13347_d_n10, assign18650_e13347_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18650_e13341: f64 = (1.0 - locals.var_uc_rdov13);
        let assign18650_e13343: f64 = (assign18650_e13341 * p.p63);
        let assign18650_e13345: f64 = (assign18650_e13343 * 1000000.0);
        (assign18650_e13345, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign18650_e13347;
        locals.var_t1_dn0 = assign18650_e13347_d_n0;
        locals.var_t1_dn2 = assign18650_e13347_d_n2;
        locals.var_t1_dn4 = assign18650_e13347_d_n4;
        locals.var_t1_dn5 = assign18650_e13347_d_n5;
        locals.var_t1_dn6 = assign18650_e13347_d_n6;
        locals.var_t1_dn7 = assign18650_e13347_d_n7;
        locals.var_t1_dn8 = assign18650_e13347_d_n8;
        locals.var_t1_dn9 = assign18650_e13347_d_n9;
        locals.var_t1_dn10 = assign18650_e13347_d_n10;
        locals.var_t1_dn13 = assign18650_e13347_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign18660_e13368, assign18660_e13368_d_n0, assign18660_e13368_d_n2, assign18660_e13368_d_n4, assign18660_e13368_d_n5, assign18660_e13368_d_n6, assign18660_e13368_d_n7, assign18660_e13368_d_n8, assign18660_e13368_d_n9, assign18660_e13368_d_n10, assign18660_e13368_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18660_e13355: f64 = (p.p99 * p.p99);
        let assign18660_e13359: f64 = (0.0001 * 0.01);
        let assign18660_e13360: f64 = (4.0 * assign18660_e13359);
        let assign18660_e13363: f64 = (0.0001 * 0.01);
        let assign18660_e13364: f64 = (assign18660_e13360 * assign18660_e13363);
        let assign18660_e13365: f64 = (assign18660_e13355 + assign18660_e13364);
        let assign18660_e13366: f64 = (assign18660_e13365).sqrt();
        (assign18660_e13366, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18660_e13368;
        locals.var_tmf2_dn0 = assign18660_e13368_d_n0;
        locals.var_tmf2_dn2 = assign18660_e13368_d_n2;
        locals.var_tmf2_dn4 = assign18660_e13368_d_n4;
        locals.var_tmf2_dn5 = assign18660_e13368_d_n5;
        locals.var_tmf2_dn6 = assign18660_e13368_d_n6;
        locals.var_tmf2_dn7 = assign18660_e13368_d_n7;
        locals.var_tmf2_dn8 = assign18660_e13368_d_n8;
        locals.var_tmf2_dn9 = assign18660_e13368_d_n9;
        locals.var_tmf2_dn10 = assign18660_e13368_d_n10;
        locals.var_tmf2_dn13 = assign18660_e13368_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign18670_e13382, assign18670_e13382_d_n0, assign18670_e13382_d_n2, assign18670_e13382_d_n4, assign18670_e13382_d_n5, assign18670_e13382_d_n6, assign18670_e13382_d_n7, assign18670_e13382_d_n8, assign18670_e13382_d_n9, assign18670_e13382_d_n10, assign18670_e13382_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18670_e13378: f64 = (p.p99 / locals.var_tmf2);
        let assign18670_e13379: f64 = (1.0 + assign18670_e13378);
        let assign18670_e13380: f64 = (0.5 * assign18670_e13379);
        (assign18670_e13380, (0.5 * (-((p.p99 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign18670_e13382;
        locals.var_t0_dn0 = assign18670_e13382_d_n0;
        locals.var_t0_dn2 = assign18670_e13382_d_n2;
        locals.var_t0_dn4 = assign18670_e13382_d_n4;
        locals.var_t0_dn5 = assign18670_e13382_d_n5;
        locals.var_t0_dn6 = assign18670_e13382_d_n6;
        locals.var_t0_dn7 = assign18670_e13382_d_n7;
        locals.var_t0_dn8 = assign18670_e13382_d_n8;
        locals.var_t0_dn9 = assign18670_e13382_d_n9;
        locals.var_t0_dn10 = assign18670_e13382_d_n10;
        locals.var_t0_dn13 = assign18670_e13382_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign18680_e13394, assign18680_e13394_d_n0, assign18680_e13394_d_n2, assign18680_e13394_d_n4, assign18680_e13394_d_n5, assign18680_e13394_d_n6, assign18680_e13394_d_n7, assign18680_e13394_d_n8, assign18680_e13394_d_n9, assign18680_e13394_d_n10, assign18680_e13394_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18680_e13391: f64 = (p.p99 + locals.var_tmf2);
        let assign18680_e13392: f64 = (0.5 * assign18680_e13391);
        (assign18680_e13392, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * locals.var_tmf2_dn5), (0.5 * locals.var_tmf2_dn6), (0.5 * locals.var_tmf2_dn7), (0.5 * locals.var_tmf2_dn8), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn13),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign18680_e13394;
        locals.var_t2_dn0 = assign18680_e13394_d_n0;
        locals.var_t2_dn2 = assign18680_e13394_d_n2;
        locals.var_t2_dn4 = assign18680_e13394_d_n4;
        locals.var_t2_dn5 = assign18680_e13394_d_n5;
        locals.var_t2_dn6 = assign18680_e13394_d_n6;
        locals.var_t2_dn7 = assign18680_e13394_d_n7;
        locals.var_t2_dn8 = assign18680_e13394_d_n8;
        locals.var_t2_dn9 = assign18680_e13394_d_n9;
        locals.var_t2_dn10 = assign18680_e13394_d_n10;
        locals.var_t2_dn13 = assign18680_e13394_d_n13;
        locals.var_t2_rv = 0.0;

        let assign18690_e13397: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard384 = assign18690_e13397;
        locals.var_guard384_rv = 0.0;

        let (assign18700_e13407, assign18700_e13407_d_n0, assign18700_e13407_d_n2, assign18700_e13407_d_n4, assign18700_e13407_d_n5, assign18700_e13407_d_n6, assign18700_e13407_d_n7, assign18700_e13407_d_n8, assign18700_e13407_d_n9, assign18700_e13407_d_n10, assign18700_e13407_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign18700_e13407;
        locals.var_t2_dn0 = assign18700_e13407_d_n0;
        locals.var_t2_dn2 = assign18700_e13407_d_n2;
        locals.var_t2_dn4 = assign18700_e13407_d_n4;
        locals.var_t2_dn5 = assign18700_e13407_d_n5;
        locals.var_t2_dn6 = assign18700_e13407_d_n6;
        locals.var_t2_dn7 = assign18700_e13407_d_n7;
        locals.var_t2_dn8 = assign18700_e13407_d_n8;
        locals.var_t2_dn9 = assign18700_e13407_d_n9;
        locals.var_t2_dn10 = assign18700_e13407_d_n10;
        locals.var_t2_dn13 = assign18700_e13407_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign18710_e13417, assign18710_e13417_d_n0, assign18710_e13417_d_n2, assign18710_e13417_d_n4, assign18710_e13417_d_n5, assign18710_e13417_d_n6, assign18710_e13417_d_n7, assign18710_e13417_d_n8, assign18710_e13417_d_n9, assign18710_e13417_d_n10, assign18710_e13417_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign18710_e13417;
        locals.var_t0_dn0 = assign18710_e13417_d_n0;
        locals.var_t0_dn2 = assign18710_e13417_d_n2;
        locals.var_t0_dn4 = assign18710_e13417_d_n4;
        locals.var_t0_dn5 = assign18710_e13417_d_n5;
        locals.var_t0_dn6 = assign18710_e13417_d_n6;
        locals.var_t0_dn7 = assign18710_e13417_d_n7;
        locals.var_t0_dn8 = assign18710_e13417_d_n8;
        locals.var_t0_dn9 = assign18710_e13417_d_n9;
        locals.var_t0_dn10 = assign18710_e13417_d_n10;
        locals.var_t0_dn13 = assign18710_e13417_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign18720_e13428, assign18720_e13428_d_n0, assign18720_e13428_d_n2, assign18720_e13428_d_n4, assign18720_e13428_d_n5, assign18720_e13428_d_n6, assign18720_e13428_d_n7, assign18720_e13428_d_n8, assign18720_e13428_d_n9, assign18720_e13428_d_n10, assign18720_e13428_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18720_e13424: f64 = (-p.p98);
        let assign18720_e13426: f64 = (assign18720_e13424 / locals.var_t2);
        (assign18720_e13426, (-((assign18720_e13424 * locals.var_t2_dn0) / (locals.var_t2 * locals.var_t2))), (-((assign18720_e13424 * locals.var_t2_dn2) / (locals.var_t2 * locals.var_t2))), (-((assign18720_e13424 * locals.var_t2_dn4) / (locals.var_t2 * locals.var_t2))), (-((assign18720_e13424 * locals.var_t2_dn5) / (locals.var_t2 * locals.var_t2))), (-((assign18720_e13424 * locals.var_t2_dn6) / (locals.var_t2 * locals.var_t2))), (-((assign18720_e13424 * locals.var_t2_dn7) / (locals.var_t2 * locals.var_t2))), (-((assign18720_e13424 * locals.var_t2_dn8) / (locals.var_t2 * locals.var_t2))), (-((assign18720_e13424 * locals.var_t2_dn9) / (locals.var_t2 * locals.var_t2))), (-((assign18720_e13424 * locals.var_t2_dn10) / (locals.var_t2 * locals.var_t2))), (-((assign18720_e13424 * locals.var_t2_dn13) / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn13,)
    }
};
        locals.var_t8 = assign18720_e13428;
        locals.var_t8_dn0 = assign18720_e13428_d_n0;
        locals.var_t8_dn2 = assign18720_e13428_d_n2;
        locals.var_t8_dn4 = assign18720_e13428_d_n4;
        locals.var_t8_dn5 = assign18720_e13428_d_n5;
        locals.var_t8_dn6 = assign18720_e13428_d_n6;
        locals.var_t8_dn7 = assign18720_e13428_d_n7;
        locals.var_t8_dn8 = assign18720_e13428_d_n8;
        locals.var_t8_dn9 = assign18720_e13428_d_n9;
        locals.var_t8_dn10 = assign18720_e13428_d_n10;
        locals.var_t8_dn13 = assign18720_e13428_d_n13;
        locals.var_t8_rv = 0.0;

        let (assign18730_e13444, assign18730_e13444_d_n0, assign18730_e13444_d_n2, assign18730_e13444_d_n4, assign18730_e13444_d_n5, assign18730_e13444_d_n6, assign18730_e13444_d_n7, assign18730_e13444_d_n8, assign18730_e13444_d_n9, assign18730_e13444_d_n10, assign18730_e13444_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18730_e13436: f64 = (locals.var_t8 * p.p63);
        let assign18730_e13438: f64 = (assign18730_e13436 * 1000000.0);
        let assign18730_e13440: f64 = (assign18730_e13438 + 1.0);
        let assign18730_e13442: f64 = (assign18730_e13440 + p.p98);
        (assign18730_e13442, ((locals.var_t8_dn0 * p.p63) * 1000000.0), ((locals.var_t8_dn2 * p.p63) * 1000000.0), ((locals.var_t8_dn4 * p.p63) * 1000000.0), ((locals.var_t8_dn5 * p.p63) * 1000000.0), ((locals.var_t8_dn6 * p.p63) * 1000000.0), ((locals.var_t8_dn7 * p.p63) * 1000000.0), ((locals.var_t8_dn8 * p.p63) * 1000000.0), ((locals.var_t8_dn9 * p.p63) * 1000000.0), ((locals.var_t8_dn10 * p.p63) * 1000000.0), ((locals.var_t8_dn13 * p.p63) * 1000000.0),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign18730_e13444;
        locals.var_t3_dn0 = assign18730_e13444_d_n0;
        locals.var_t3_dn2 = assign18730_e13444_d_n2;
        locals.var_t3_dn4 = assign18730_e13444_d_n4;
        locals.var_t3_dn5 = assign18730_e13444_d_n5;
        locals.var_t3_dn6 = assign18730_e13444_d_n6;
        locals.var_t3_dn7 = assign18730_e13444_d_n7;
        locals.var_t3_dn8 = assign18730_e13444_d_n8;
        locals.var_t3_dn9 = assign18730_e13444_d_n9;
        locals.var_t3_dn10 = assign18730_e13444_d_n10;
        locals.var_t3_dn13 = assign18730_e13444_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign18740_e13458, assign18740_e13458_d_n0, assign18740_e13458_d_n2, assign18740_e13458_d_n4, assign18740_e13458_d_n5, assign18740_e13458_d_n6, assign18740_e13458_d_n7, assign18740_e13458_d_n8, assign18740_e13458_d_n9, assign18740_e13458_d_n10, assign18740_e13458_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18740_e13452: f64 = (locals.var_t3 * locals.var_t4);
        let assign18740_e13454: f64 = (assign18740_e13452 - locals.var_t4);
        let assign18740_e13456: f64 = (assign18740_e13454 - 0.01);
        (assign18740_e13456, (((locals.var_t3_dn0 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn0)) - locals.var_t4_dn0), (((locals.var_t3_dn2 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn2)) - locals.var_t4_dn2), (((locals.var_t3_dn4 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn4)) - locals.var_t4_dn4), (((locals.var_t3_dn5 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn5)) - locals.var_t4_dn5), (((locals.var_t3_dn6 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn6)) - locals.var_t4_dn6), (((locals.var_t3_dn7 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn7)) - locals.var_t4_dn7), (((locals.var_t3_dn8 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn8)) - locals.var_t4_dn8), (((locals.var_t3_dn9 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn9)) - locals.var_t4_dn9), (((locals.var_t3_dn10 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn10)) - locals.var_t4_dn10), (((locals.var_t3_dn13 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn13)) - locals.var_t4_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign18740_e13458;
        locals.var_tmf1_dn0 = assign18740_e13458_d_n0;
        locals.var_tmf1_dn2 = assign18740_e13458_d_n2;
        locals.var_tmf1_dn4 = assign18740_e13458_d_n4;
        locals.var_tmf1_dn5 = assign18740_e13458_d_n5;
        locals.var_tmf1_dn6 = assign18740_e13458_d_n6;
        locals.var_tmf1_dn7 = assign18740_e13458_d_n7;
        locals.var_tmf1_dn8 = assign18740_e13458_d_n8;
        locals.var_tmf1_dn9 = assign18740_e13458_d_n9;
        locals.var_tmf1_dn10 = assign18740_e13458_d_n10;
        locals.var_tmf1_dn13 = assign18740_e13458_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign18750_e13470, assign18750_e13470_d_n0, assign18750_e13470_d_n2, assign18750_e13470_d_n4, assign18750_e13470_d_n5, assign18750_e13470_d_n6, assign18750_e13470_d_n7, assign18750_e13470_d_n8, assign18750_e13470_d_n9, assign18750_e13470_d_n10, assign18750_e13470_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18750_e13466: f64 = (4.0 * locals.var_t4);
        let assign18750_e13468: f64 = (assign18750_e13466 * 0.01);
        (assign18750_e13468, ((4.0 * locals.var_t4_dn0) * 0.01), ((4.0 * locals.var_t4_dn2) * 0.01), ((4.0 * locals.var_t4_dn4) * 0.01), ((4.0 * locals.var_t4_dn5) * 0.01), ((4.0 * locals.var_t4_dn6) * 0.01), ((4.0 * locals.var_t4_dn7) * 0.01), ((4.0 * locals.var_t4_dn8) * 0.01), ((4.0 * locals.var_t4_dn9) * 0.01), ((4.0 * locals.var_t4_dn10) * 0.01), ((4.0 * locals.var_t4_dn13) * 0.01),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18750_e13470;
        locals.var_tmf2_dn0 = assign18750_e13470_d_n0;
        locals.var_tmf2_dn2 = assign18750_e13470_d_n2;
        locals.var_tmf2_dn4 = assign18750_e13470_d_n4;
        locals.var_tmf2_dn5 = assign18750_e13470_d_n5;
        locals.var_tmf2_dn6 = assign18750_e13470_d_n6;
        locals.var_tmf2_dn7 = assign18750_e13470_d_n7;
        locals.var_tmf2_dn8 = assign18750_e13470_d_n8;
        locals.var_tmf2_dn9 = assign18750_e13470_d_n9;
        locals.var_tmf2_dn10 = assign18750_e13470_d_n10;
        locals.var_tmf2_dn13 = assign18750_e13470_d_n13;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_44(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign18760_e13484, assign18760_e13484_d_n0, assign18760_e13484_d_n2, assign18760_e13484_d_n4, assign18760_e13484_d_n5, assign18760_e13484_d_n6, assign18760_e13484_d_n7, assign18760_e13484_d_n8, assign18760_e13484_d_n9, assign18760_e13484_d_n10, assign18760_e13484_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let (assign18760_e13482, assign18760_e13482_d_n0, assign18760_e13482_d_n2, assign18760_e13482_d_n4, assign18760_e13482_d_n5, assign18760_e13482_d_n6, assign18760_e13482_d_n7, assign18760_e13482_d_n8, assign18760_e13482_d_n9, assign18760_e13482_d_n10, assign18760_e13482_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign18760_e13481: f64 = (-locals.var_tmf2);
                (assign18760_e13481, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign18760_e13482, assign18760_e13482_d_n0, assign18760_e13482_d_n2, assign18760_e13482_d_n4, assign18760_e13482_d_n5, assign18760_e13482_d_n6, assign18760_e13482_d_n7, assign18760_e13482_d_n8, assign18760_e13482_d_n9, assign18760_e13482_d_n10, assign18760_e13482_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18760_e13484;
        locals.var_tmf2_dn0 = assign18760_e13484_d_n0;
        locals.var_tmf2_dn2 = assign18760_e13484_d_n2;
        locals.var_tmf2_dn4 = assign18760_e13484_d_n4;
        locals.var_tmf2_dn5 = assign18760_e13484_d_n5;
        locals.var_tmf2_dn6 = assign18760_e13484_d_n6;
        locals.var_tmf2_dn7 = assign18760_e13484_d_n7;
        locals.var_tmf2_dn8 = assign18760_e13484_d_n8;
        locals.var_tmf2_dn9 = assign18760_e13484_d_n9;
        locals.var_tmf2_dn10 = assign18760_e13484_d_n10;
        locals.var_tmf2_dn13 = assign18760_e13484_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign18770_e13497, assign18770_e13497_d_n0, assign18770_e13497_d_n2, assign18770_e13497_d_n4, assign18770_e13497_d_n5, assign18770_e13497_d_n6, assign18770_e13497_d_n7, assign18770_e13497_d_n8, assign18770_e13497_d_n9, assign18770_e13497_d_n10, assign18770_e13497_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18770_e13492: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18770_e13494: f64 = (assign18770_e13492 + locals.var_tmf2);
        let assign18770_e13495: f64 = (assign18770_e13494).sqrt();
        (assign18770_e13495, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18770_e13495)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18770_e13495)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18770_e13495)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18770_e13495)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18770_e13495)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18770_e13495)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18770_e13495)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18770_e13495)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18770_e13495)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign18770_e13495)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18770_e13497;
        locals.var_tmf2_dn0 = assign18770_e13497_d_n0;
        locals.var_tmf2_dn2 = assign18770_e13497_d_n2;
        locals.var_tmf2_dn4 = assign18770_e13497_d_n4;
        locals.var_tmf2_dn5 = assign18770_e13497_d_n5;
        locals.var_tmf2_dn6 = assign18770_e13497_d_n6;
        locals.var_tmf2_dn7 = assign18770_e13497_d_n7;
        locals.var_tmf2_dn8 = assign18770_e13497_d_n8;
        locals.var_tmf2_dn9 = assign18770_e13497_d_n9;
        locals.var_tmf2_dn10 = assign18770_e13497_d_n10;
        locals.var_tmf2_dn13 = assign18770_e13497_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign18780_e13511, assign18780_e13511_d_n0, assign18780_e13511_d_n2, assign18780_e13511_d_n4, assign18780_e13511_d_n5, assign18780_e13511_d_n6, assign18780_e13511_d_n7, assign18780_e13511_d_n8, assign18780_e13511_d_n9, assign18780_e13511_d_n10, assign18780_e13511_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18780_e13507: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18780_e13508: f64 = (1.0 + assign18780_e13507);
        let assign18780_e13509: f64 = (0.5 * assign18780_e13508);
        (assign18780_e13509, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign18780_e13511;
        locals.var_t6_dn0 = assign18780_e13511_d_n0;
        locals.var_t6_dn2 = assign18780_e13511_d_n2;
        locals.var_t6_dn4 = assign18780_e13511_d_n4;
        locals.var_t6_dn5 = assign18780_e13511_d_n5;
        locals.var_t6_dn6 = assign18780_e13511_d_n6;
        locals.var_t6_dn7 = assign18780_e13511_d_n7;
        locals.var_t6_dn8 = assign18780_e13511_d_n8;
        locals.var_t6_dn9 = assign18780_e13511_d_n9;
        locals.var_t6_dn10 = assign18780_e13511_d_n10;
        locals.var_t6_dn13 = assign18780_e13511_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign18790_e13525, assign18790_e13525_d_n0, assign18790_e13525_d_n2, assign18790_e13525_d_n4, assign18790_e13525_d_n5, assign18790_e13525_d_n6, assign18790_e13525_d_n7, assign18790_e13525_d_n8, assign18790_e13525_d_n9, assign18790_e13525_d_n10, assign18790_e13525_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18790_e13521: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18790_e13522: f64 = (0.5 * assign18790_e13521);
        let assign18790_e13523: f64 = (locals.var_t4 + assign18790_e13522);
        (assign18790_e13523, (locals.var_t4_dn0 + (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t4_dn2 + (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t4_dn4 + (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t4_dn5 + (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t4_dn6 + (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t4_dn7 + (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t4_dn8 + (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t4_dn9 + (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t4_dn10 + (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t4_dn13 + (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign18790_e13525;
        locals.var_t5_dn0 = assign18790_e13525_d_n0;
        locals.var_t5_dn2 = assign18790_e13525_d_n2;
        locals.var_t5_dn4 = assign18790_e13525_d_n4;
        locals.var_t5_dn5 = assign18790_e13525_d_n5;
        locals.var_t5_dn6 = assign18790_e13525_d_n6;
        locals.var_t5_dn7 = assign18790_e13525_d_n7;
        locals.var_t5_dn8 = assign18790_e13525_d_n8;
        locals.var_t5_dn9 = assign18790_e13525_d_n9;
        locals.var_t5_dn10 = assign18790_e13525_d_n10;
        locals.var_t5_dn13 = assign18790_e13525_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign18800_e13541, assign18800_e13541_d_n0, assign18800_e13541_d_n2, assign18800_e13541_d_n4, assign18800_e13541_d_n5, assign18800_e13541_d_n6, assign18800_e13541_d_n7, assign18800_e13541_d_n8, assign18800_e13541_d_n9, assign18800_e13541_d_n10, assign18800_e13541_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18800_e13534: f64 = (p.p98 + 1.0);
        let assign18800_e13535: f64 = (locals.var_t4 * assign18800_e13534);
        let assign18800_e13537: f64 = (assign18800_e13535 - locals.var_t5);
        let assign18800_e13539: f64 = (assign18800_e13537 - 5e-5);
        (assign18800_e13539, ((locals.var_t4_dn0 * assign18800_e13534) - locals.var_t5_dn0), ((locals.var_t4_dn2 * assign18800_e13534) - locals.var_t5_dn2), ((locals.var_t4_dn4 * assign18800_e13534) - locals.var_t5_dn4), ((locals.var_t4_dn5 * assign18800_e13534) - locals.var_t5_dn5), ((locals.var_t4_dn6 * assign18800_e13534) - locals.var_t5_dn6), ((locals.var_t4_dn7 * assign18800_e13534) - locals.var_t5_dn7), ((locals.var_t4_dn8 * assign18800_e13534) - locals.var_t5_dn8), ((locals.var_t4_dn9 * assign18800_e13534) - locals.var_t5_dn9), ((locals.var_t4_dn10 * assign18800_e13534) - locals.var_t5_dn10), ((locals.var_t4_dn13 * assign18800_e13534) - locals.var_t5_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign18800_e13541;
        locals.var_tmf1_dn0 = assign18800_e13541_d_n0;
        locals.var_tmf1_dn2 = assign18800_e13541_d_n2;
        locals.var_tmf1_dn4 = assign18800_e13541_d_n4;
        locals.var_tmf1_dn5 = assign18800_e13541_d_n5;
        locals.var_tmf1_dn6 = assign18800_e13541_d_n6;
        locals.var_tmf1_dn7 = assign18800_e13541_d_n7;
        locals.var_tmf1_dn8 = assign18800_e13541_d_n8;
        locals.var_tmf1_dn9 = assign18800_e13541_d_n9;
        locals.var_tmf1_dn10 = assign18800_e13541_d_n10;
        locals.var_tmf1_dn13 = assign18800_e13541_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign18810_e13557, assign18810_e13557_d_n0, assign18810_e13557_d_n2, assign18810_e13557_d_n4, assign18810_e13557_d_n5, assign18810_e13557_d_n6, assign18810_e13557_d_n7, assign18810_e13557_d_n8, assign18810_e13557_d_n9, assign18810_e13557_d_n10, assign18810_e13557_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18810_e13551: f64 = (p.p98 + 1.0);
        let assign18810_e13552: f64 = (locals.var_t4 * assign18810_e13551);
        let assign18810_e13553: f64 = (4.0 * assign18810_e13552);
        let assign18810_e13555: f64 = (assign18810_e13553 * 5e-5);
        (assign18810_e13555, ((4.0 * (locals.var_t4_dn0 * assign18810_e13551)) * 5e-5), ((4.0 * (locals.var_t4_dn2 * assign18810_e13551)) * 5e-5), ((4.0 * (locals.var_t4_dn4 * assign18810_e13551)) * 5e-5), ((4.0 * (locals.var_t4_dn5 * assign18810_e13551)) * 5e-5), ((4.0 * (locals.var_t4_dn6 * assign18810_e13551)) * 5e-5), ((4.0 * (locals.var_t4_dn7 * assign18810_e13551)) * 5e-5), ((4.0 * (locals.var_t4_dn8 * assign18810_e13551)) * 5e-5), ((4.0 * (locals.var_t4_dn9 * assign18810_e13551)) * 5e-5), ((4.0 * (locals.var_t4_dn10 * assign18810_e13551)) * 5e-5), ((4.0 * (locals.var_t4_dn13 * assign18810_e13551)) * 5e-5),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18810_e13557;
        locals.var_tmf2_dn0 = assign18810_e13557_d_n0;
        locals.var_tmf2_dn2 = assign18810_e13557_d_n2;
        locals.var_tmf2_dn4 = assign18810_e13557_d_n4;
        locals.var_tmf2_dn5 = assign18810_e13557_d_n5;
        locals.var_tmf2_dn6 = assign18810_e13557_d_n6;
        locals.var_tmf2_dn7 = assign18810_e13557_d_n7;
        locals.var_tmf2_dn8 = assign18810_e13557_d_n8;
        locals.var_tmf2_dn9 = assign18810_e13557_d_n9;
        locals.var_tmf2_dn10 = assign18810_e13557_d_n10;
        locals.var_tmf2_dn13 = assign18810_e13557_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign18820_e13571, assign18820_e13571_d_n0, assign18820_e13571_d_n2, assign18820_e13571_d_n4, assign18820_e13571_d_n5, assign18820_e13571_d_n6, assign18820_e13571_d_n7, assign18820_e13571_d_n8, assign18820_e13571_d_n9, assign18820_e13571_d_n10, assign18820_e13571_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let (assign18820_e13569, assign18820_e13569_d_n0, assign18820_e13569_d_n2, assign18820_e13569_d_n4, assign18820_e13569_d_n5, assign18820_e13569_d_n6, assign18820_e13569_d_n7, assign18820_e13569_d_n8, assign18820_e13569_d_n9, assign18820_e13569_d_n10, assign18820_e13569_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign18820_e13568: f64 = (-locals.var_tmf2);
                (assign18820_e13568, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign18820_e13569, assign18820_e13569_d_n0, assign18820_e13569_d_n2, assign18820_e13569_d_n4, assign18820_e13569_d_n5, assign18820_e13569_d_n6, assign18820_e13569_d_n7, assign18820_e13569_d_n8, assign18820_e13569_d_n9, assign18820_e13569_d_n10, assign18820_e13569_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18820_e13571;
        locals.var_tmf2_dn0 = assign18820_e13571_d_n0;
        locals.var_tmf2_dn2 = assign18820_e13571_d_n2;
        locals.var_tmf2_dn4 = assign18820_e13571_d_n4;
        locals.var_tmf2_dn5 = assign18820_e13571_d_n5;
        locals.var_tmf2_dn6 = assign18820_e13571_d_n6;
        locals.var_tmf2_dn7 = assign18820_e13571_d_n7;
        locals.var_tmf2_dn8 = assign18820_e13571_d_n8;
        locals.var_tmf2_dn9 = assign18820_e13571_d_n9;
        locals.var_tmf2_dn10 = assign18820_e13571_d_n10;
        locals.var_tmf2_dn13 = assign18820_e13571_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign18830_e13584, assign18830_e13584_d_n0, assign18830_e13584_d_n2, assign18830_e13584_d_n4, assign18830_e13584_d_n5, assign18830_e13584_d_n6, assign18830_e13584_d_n7, assign18830_e13584_d_n8, assign18830_e13584_d_n9, assign18830_e13584_d_n10, assign18830_e13584_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18830_e13579: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18830_e13581: f64 = (assign18830_e13579 + locals.var_tmf2);
        let assign18830_e13582: f64 = (assign18830_e13581).sqrt();
        (assign18830_e13582, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18830_e13582)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18830_e13582)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18830_e13582)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18830_e13582)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18830_e13582)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18830_e13582)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18830_e13582)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18830_e13582)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18830_e13582)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign18830_e13582)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18830_e13584;
        locals.var_tmf2_dn0 = assign18830_e13584_d_n0;
        locals.var_tmf2_dn2 = assign18830_e13584_d_n2;
        locals.var_tmf2_dn4 = assign18830_e13584_d_n4;
        locals.var_tmf2_dn5 = assign18830_e13584_d_n5;
        locals.var_tmf2_dn6 = assign18830_e13584_d_n6;
        locals.var_tmf2_dn7 = assign18830_e13584_d_n7;
        locals.var_tmf2_dn8 = assign18830_e13584_d_n8;
        locals.var_tmf2_dn9 = assign18830_e13584_d_n9;
        locals.var_tmf2_dn10 = assign18830_e13584_d_n10;
        locals.var_tmf2_dn13 = assign18830_e13584_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign18840_e13598, assign18840_e13598_d_n0, assign18840_e13598_d_n2, assign18840_e13598_d_n4, assign18840_e13598_d_n5, assign18840_e13598_d_n6, assign18840_e13598_d_n7, assign18840_e13598_d_n8, assign18840_e13598_d_n9, assign18840_e13598_d_n10, assign18840_e13598_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18840_e13594: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18840_e13595: f64 = (1.0 + assign18840_e13594);
        let assign18840_e13596: f64 = (0.5 * assign18840_e13595);
        (assign18840_e13596, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign18840_e13598;
        locals.var_t6_dn0 = assign18840_e13598_d_n0;
        locals.var_t6_dn2 = assign18840_e13598_d_n2;
        locals.var_t6_dn4 = assign18840_e13598_d_n4;
        locals.var_t6_dn5 = assign18840_e13598_d_n5;
        locals.var_t6_dn6 = assign18840_e13598_d_n6;
        locals.var_t6_dn7 = assign18840_e13598_d_n7;
        locals.var_t6_dn8 = assign18840_e13598_d_n8;
        locals.var_t6_dn9 = assign18840_e13598_d_n9;
        locals.var_t6_dn10 = assign18840_e13598_d_n10;
        locals.var_t6_dn13 = assign18840_e13598_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign18850_e13616, assign18850_e13616_d_n0, assign18850_e13616_d_n2, assign18850_e13616_d_n4, assign18850_e13616_d_n5, assign18850_e13616_d_n6, assign18850_e13616_d_n7, assign18850_e13616_d_n8, assign18850_e13616_d_n9, assign18850_e13616_d_n10, assign18850_e13616_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18850_e13607: f64 = (p.p98 + 1.0);
        let assign18850_e13608: f64 = (locals.var_t4 * assign18850_e13607);
        let assign18850_e13612: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18850_e13613: f64 = (0.5 * assign18850_e13612);
        let assign18850_e13614: f64 = (assign18850_e13608 - assign18850_e13613);
        (assign18850_e13614, ((locals.var_t4_dn0 * assign18850_e13607) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((locals.var_t4_dn2 * assign18850_e13607) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((locals.var_t4_dn4 * assign18850_e13607) - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), ((locals.var_t4_dn5 * assign18850_e13607) - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), ((locals.var_t4_dn6 * assign18850_e13607) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((locals.var_t4_dn7 * assign18850_e13607) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((locals.var_t4_dn8 * assign18850_e13607) - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), ((locals.var_t4_dn9 * assign18850_e13607) - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), ((locals.var_t4_dn10 * assign18850_e13607) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((locals.var_t4_dn13 * assign18850_e13607) - (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign18850_e13616;
        locals.var_t7_dn0 = assign18850_e13616_d_n0;
        locals.var_t7_dn2 = assign18850_e13616_d_n2;
        locals.var_t7_dn4 = assign18850_e13616_d_n4;
        locals.var_t7_dn5 = assign18850_e13616_d_n5;
        locals.var_t7_dn6 = assign18850_e13616_d_n6;
        locals.var_t7_dn7 = assign18850_e13616_d_n7;
        locals.var_t7_dn8 = assign18850_e13616_d_n8;
        locals.var_t7_dn9 = assign18850_e13616_d_n9;
        locals.var_t7_dn10 = assign18850_e13616_d_n10;
        locals.var_t7_dn13 = assign18850_e13616_d_n13;
        locals.var_t7_rv = 0.0;

        let (assign18860_e13632, assign18860_e13632_d_n0, assign18860_e13632_d_n2, assign18860_e13632_d_n4, assign18860_e13632_d_n5, assign18860_e13632_d_n6, assign18860_e13632_d_n7, assign18860_e13632_d_n8, assign18860_e13632_d_n9, assign18860_e13632_d_n10, assign18860_e13632_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18860_e13625: f64 = (locals.var_t1 * locals.var_t4);
        let assign18860_e13626: f64 = (locals.var_t7 + assign18860_e13625);
        let assign18860_e13628: f64 = assign18860_e13626;
        let assign18860_e13630: f64 = (assign18860_e13628 - 5e-5);
        (assign18860_e13630, (locals.var_t7_dn0 + ((locals.var_t1_dn0 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn0))), (locals.var_t7_dn2 + ((locals.var_t1_dn2 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn2))), (locals.var_t7_dn4 + ((locals.var_t1_dn4 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn4))), (locals.var_t7_dn5 + ((locals.var_t1_dn5 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn5))), (locals.var_t7_dn6 + ((locals.var_t1_dn6 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn6))), (locals.var_t7_dn7 + ((locals.var_t1_dn7 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn7))), (locals.var_t7_dn8 + ((locals.var_t1_dn8 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn8))), (locals.var_t7_dn9 + ((locals.var_t1_dn9 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn9))), (locals.var_t7_dn10 + ((locals.var_t1_dn10 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn10))), (locals.var_t7_dn13 + ((locals.var_t1_dn13 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn13))),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign18860_e13632;
        locals.var_tmf1_dn0 = assign18860_e13632_d_n0;
        locals.var_tmf1_dn2 = assign18860_e13632_d_n2;
        locals.var_tmf1_dn4 = assign18860_e13632_d_n4;
        locals.var_tmf1_dn5 = assign18860_e13632_d_n5;
        locals.var_tmf1_dn6 = assign18860_e13632_d_n6;
        locals.var_tmf1_dn7 = assign18860_e13632_d_n7;
        locals.var_tmf1_dn8 = assign18860_e13632_d_n8;
        locals.var_tmf1_dn9 = assign18860_e13632_d_n9;
        locals.var_tmf1_dn10 = assign18860_e13632_d_n10;
        locals.var_tmf1_dn13 = assign18860_e13632_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign18870_e13644, assign18870_e13644_d_n0, assign18870_e13644_d_n2, assign18870_e13644_d_n4, assign18870_e13644_d_n5, assign18870_e13644_d_n6, assign18870_e13644_d_n7, assign18870_e13644_d_n8, assign18870_e13644_d_n9, assign18870_e13644_d_n10, assign18870_e13644_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18870_e13644;
        locals.var_tmf2_dn0 = assign18870_e13644_d_n0;
        locals.var_tmf2_dn2 = assign18870_e13644_d_n2;
        locals.var_tmf2_dn4 = assign18870_e13644_d_n4;
        locals.var_tmf2_dn5 = assign18870_e13644_d_n5;
        locals.var_tmf2_dn6 = assign18870_e13644_d_n6;
        locals.var_tmf2_dn7 = assign18870_e13644_d_n7;
        locals.var_tmf2_dn8 = assign18870_e13644_d_n8;
        locals.var_tmf2_dn9 = assign18870_e13644_d_n9;
        locals.var_tmf2_dn10 = assign18870_e13644_d_n10;
        locals.var_tmf2_dn13 = assign18870_e13644_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign18880_e13658, assign18880_e13658_d_n0, assign18880_e13658_d_n2, assign18880_e13658_d_n4, assign18880_e13658_d_n5, assign18880_e13658_d_n6, assign18880_e13658_d_n7, assign18880_e13658_d_n8, assign18880_e13658_d_n9, assign18880_e13658_d_n10, assign18880_e13658_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let (assign18880_e13656, assign18880_e13656_d_n0, assign18880_e13656_d_n2, assign18880_e13656_d_n4, assign18880_e13656_d_n5, assign18880_e13656_d_n6, assign18880_e13656_d_n7, assign18880_e13656_d_n8, assign18880_e13656_d_n9, assign18880_e13656_d_n10, assign18880_e13656_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign18880_e13655: f64 = (-locals.var_tmf2);
                (assign18880_e13655, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign18880_e13656, assign18880_e13656_d_n0, assign18880_e13656_d_n2, assign18880_e13656_d_n4, assign18880_e13656_d_n5, assign18880_e13656_d_n6, assign18880_e13656_d_n7, assign18880_e13656_d_n8, assign18880_e13656_d_n9, assign18880_e13656_d_n10, assign18880_e13656_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18880_e13658;
        locals.var_tmf2_dn0 = assign18880_e13658_d_n0;
        locals.var_tmf2_dn2 = assign18880_e13658_d_n2;
        locals.var_tmf2_dn4 = assign18880_e13658_d_n4;
        locals.var_tmf2_dn5 = assign18880_e13658_d_n5;
        locals.var_tmf2_dn6 = assign18880_e13658_d_n6;
        locals.var_tmf2_dn7 = assign18880_e13658_d_n7;
        locals.var_tmf2_dn8 = assign18880_e13658_d_n8;
        locals.var_tmf2_dn9 = assign18880_e13658_d_n9;
        locals.var_tmf2_dn10 = assign18880_e13658_d_n10;
        locals.var_tmf2_dn13 = assign18880_e13658_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign18890_e13671, assign18890_e13671_d_n0, assign18890_e13671_d_n2, assign18890_e13671_d_n4, assign18890_e13671_d_n5, assign18890_e13671_d_n6, assign18890_e13671_d_n7, assign18890_e13671_d_n8, assign18890_e13671_d_n9, assign18890_e13671_d_n10, assign18890_e13671_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18890_e13666: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18890_e13668: f64 = (assign18890_e13666 + locals.var_tmf2);
        let assign18890_e13669: f64 = (assign18890_e13668).sqrt();
        (assign18890_e13669, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18890_e13669)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18890_e13669)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18890_e13669)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18890_e13669)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18890_e13669)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18890_e13669)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18890_e13669)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18890_e13669)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18890_e13669)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign18890_e13669)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18890_e13671;
        locals.var_tmf2_dn0 = assign18890_e13671_d_n0;
        locals.var_tmf2_dn2 = assign18890_e13671_d_n2;
        locals.var_tmf2_dn4 = assign18890_e13671_d_n4;
        locals.var_tmf2_dn5 = assign18890_e13671_d_n5;
        locals.var_tmf2_dn6 = assign18890_e13671_d_n6;
        locals.var_tmf2_dn7 = assign18890_e13671_d_n7;
        locals.var_tmf2_dn8 = assign18890_e13671_d_n8;
        locals.var_tmf2_dn9 = assign18890_e13671_d_n9;
        locals.var_tmf2_dn10 = assign18890_e13671_d_n10;
        locals.var_tmf2_dn13 = assign18890_e13671_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign18900_e13685, assign18900_e13685_d_n0, assign18900_e13685_d_n2, assign18900_e13685_d_n4, assign18900_e13685_d_n5, assign18900_e13685_d_n6, assign18900_e13685_d_n7, assign18900_e13685_d_n8, assign18900_e13685_d_n9, assign18900_e13685_d_n10, assign18900_e13685_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18900_e13681: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18900_e13682: f64 = (1.0 + assign18900_e13681);
        let assign18900_e13683: f64 = (0.5 * assign18900_e13682);
        (assign18900_e13683, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign18900_e13685;
        locals.var_t6_dn0 = assign18900_e13685_d_n0;
        locals.var_t6_dn2 = assign18900_e13685_d_n2;
        locals.var_t6_dn4 = assign18900_e13685_d_n4;
        locals.var_t6_dn5 = assign18900_e13685_d_n5;
        locals.var_t6_dn6 = assign18900_e13685_d_n6;
        locals.var_t6_dn7 = assign18900_e13685_d_n7;
        locals.var_t6_dn8 = assign18900_e13685_d_n8;
        locals.var_t6_dn9 = assign18900_e13685_d_n9;
        locals.var_t6_dn10 = assign18900_e13685_d_n10;
        locals.var_t6_dn13 = assign18900_e13685_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign18910_e13699, assign18910_e13699_d_n0, assign18910_e13699_d_n2, assign18910_e13699_d_n4, assign18910_e13699_d_n5, assign18910_e13699_d_n6, assign18910_e13699_d_n7, assign18910_e13699_d_n8, assign18910_e13699_d_n9, assign18910_e13699_d_n10, assign18910_e13699_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18910_e13695: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18910_e13696: f64 = (0.5 * assign18910_e13695);
        let assign18910_e13697: f64 = assign18910_e13696;
        (assign18910_e13697, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign18910_e13699;
        locals.var_t2_dn0 = assign18910_e13699_d_n0;
        locals.var_t2_dn2 = assign18910_e13699_d_n2;
        locals.var_t2_dn4 = assign18910_e13699_d_n4;
        locals.var_t2_dn5 = assign18910_e13699_d_n5;
        locals.var_t2_dn6 = assign18910_e13699_d_n6;
        locals.var_t2_dn7 = assign18910_e13699_d_n7;
        locals.var_t2_dn8 = assign18910_e13699_d_n8;
        locals.var_t2_dn9 = assign18910_e13699_d_n9;
        locals.var_t2_dn10 = assign18910_e13699_d_n10;
        locals.var_t2_dn13 = assign18910_e13699_d_n13;
        locals.var_t2_rv = 0.0;

        let assign18920_e13706: f64 = if ((p.p39 == 0.0) || (p.p39 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard385 = assign18920_e13706;
        locals.var_guard385_rv = 0.0;

        let (assign18930_e13726, assign18930_e13726_d_n0, assign18930_e13726_d_n2, assign18930_e13726_d_n4, assign18930_e13726_d_n5, assign18930_e13726_d_n6, assign18930_e13726_d_n7, assign18930_e13726_d_n8, assign18930_e13726_d_n9, assign18930_e13726_d_n10, assign18930_e13726_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign18930_e13717: f64 = (locals.var_mks_rdvdtemp1 * locals.var_tdiff0);
        let assign18930_e13718: f64 = (locals.var_uc_rdvd + assign18930_e13717);
        let assign18930_e13721: f64 = (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2);
        let assign18930_e13722: f64 = (assign18930_e13718 + assign18930_e13721);
        let assign18930_e13724: f64 = (assign18930_e13722 * locals.var_t2);
        (assign18930_e13724, ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn0) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn0)) * locals.var_t2) + (assign18930_e13722 * locals.var_t2_dn0)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn2) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn2)) * locals.var_t2) + (assign18930_e13722 * locals.var_t2_dn2)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn4) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn4)) * locals.var_t2) + (assign18930_e13722 * locals.var_t2_dn4)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn5) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn5)) * locals.var_t2) + (assign18930_e13722 * locals.var_t2_dn5)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn6) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn6)) * locals.var_t2) + (assign18930_e13722 * locals.var_t2_dn6)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn7) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn7)) * locals.var_t2) + (assign18930_e13722 * locals.var_t2_dn7)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn8) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn8)) * locals.var_t2) + (assign18930_e13722 * locals.var_t2_dn8)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn9) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn9)) * locals.var_t2) + (assign18930_e13722 * locals.var_t2_dn9)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn10) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn10)) * locals.var_t2) + (assign18930_e13722 * locals.var_t2_dn10)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn13) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn13)) * locals.var_t2) + (assign18930_e13722 * locals.var_t2_dn13)),)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn13,)
    }
};
        locals.var_rdvde = assign18930_e13726;
        locals.var_rdvde_dn0 = assign18930_e13726_d_n0;
        locals.var_rdvde_dn2 = assign18930_e13726_d_n2;
        locals.var_rdvde_dn4 = assign18930_e13726_d_n4;
        locals.var_rdvde_dn5 = assign18930_e13726_d_n5;
        locals.var_rdvde_dn6 = assign18930_e13726_d_n6;
        locals.var_rdvde_dn7 = assign18930_e13726_d_n7;
        locals.var_rdvde_dn8 = assign18930_e13726_d_n8;
        locals.var_rdvde_dn9 = assign18930_e13726_d_n9;
        locals.var_rdvde_dn10 = assign18930_e13726_d_n10;
        locals.var_rdvde_dn13 = assign18930_e13726_d_n13;
        locals.var_rdvde_rv = 0.0;

        let (assign18940_e13744, assign18940_e13744_d_n0, assign18940_e13744_d_n2, assign18940_e13744_d_n4, assign18940_e13744_d_n5, assign18940_e13744_d_n6, assign18940_e13744_d_n7, assign18940_e13744_d_n8, assign18940_e13744_d_n9, assign18940_e13744_d_n10, assign18940_e13744_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign18940_e13737: f64 = (0.005 * locals.var_uc_rdvd);
        let assign18940_e13738: f64 = (locals.var_rdvde - assign18940_e13737);
        let assign18940_e13741: f64 = (0.01 * locals.var_uc_rdvd);
        let assign18940_e13742: f64 = (assign18940_e13738 - assign18940_e13741);
        (assign18940_e13742, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign18940_e13744;
        locals.var_tmf1_dn0 = assign18940_e13744_d_n0;
        locals.var_tmf1_dn2 = assign18940_e13744_d_n2;
        locals.var_tmf1_dn4 = assign18940_e13744_d_n4;
        locals.var_tmf1_dn5 = assign18940_e13744_d_n5;
        locals.var_tmf1_dn6 = assign18940_e13744_d_n6;
        locals.var_tmf1_dn7 = assign18940_e13744_d_n7;
        locals.var_tmf1_dn8 = assign18940_e13744_d_n8;
        locals.var_tmf1_dn9 = assign18940_e13744_d_n9;
        locals.var_tmf1_dn10 = assign18940_e13744_d_n10;
        locals.var_tmf1_dn13 = assign18940_e13744_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign18950_e13762, assign18950_e13762_d_n0, assign18950_e13762_d_n2, assign18950_e13762_d_n4, assign18950_e13762_d_n5, assign18950_e13762_d_n6, assign18950_e13762_d_n7, assign18950_e13762_d_n8, assign18950_e13762_d_n9, assign18950_e13762_d_n10, assign18950_e13762_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign18950_e13755: f64 = (0.005 * locals.var_uc_rdvd);
        let assign18950_e13756: f64 = (4.0 * assign18950_e13755);
        let assign18950_e13759: f64 = (0.01 * locals.var_uc_rdvd);
        let assign18950_e13760: f64 = (assign18950_e13756 * assign18950_e13759);
        (assign18950_e13760, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18950_e13762;
        locals.var_tmf2_dn0 = assign18950_e13762_d_n0;
        locals.var_tmf2_dn2 = assign18950_e13762_d_n2;
        locals.var_tmf2_dn4 = assign18950_e13762_d_n4;
        locals.var_tmf2_dn5 = assign18950_e13762_d_n5;
        locals.var_tmf2_dn6 = assign18950_e13762_d_n6;
        locals.var_tmf2_dn7 = assign18950_e13762_d_n7;
        locals.var_tmf2_dn8 = assign18950_e13762_d_n8;
        locals.var_tmf2_dn9 = assign18950_e13762_d_n9;
        locals.var_tmf2_dn10 = assign18950_e13762_d_n10;
        locals.var_tmf2_dn13 = assign18950_e13762_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign18960_e13778, assign18960_e13778_d_n0, assign18960_e13778_d_n2, assign18960_e13778_d_n4, assign18960_e13778_d_n5, assign18960_e13778_d_n6, assign18960_e13778_d_n7, assign18960_e13778_d_n8, assign18960_e13778_d_n9, assign18960_e13778_d_n10, assign18960_e13778_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let (assign18960_e13776, assign18960_e13776_d_n0, assign18960_e13776_d_n2, assign18960_e13776_d_n4, assign18960_e13776_d_n5, assign18960_e13776_d_n6, assign18960_e13776_d_n7, assign18960_e13776_d_n8, assign18960_e13776_d_n9, assign18960_e13776_d_n10, assign18960_e13776_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign18960_e13775: f64 = (-locals.var_tmf2);
                (assign18960_e13775, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign18960_e13776, assign18960_e13776_d_n0, assign18960_e13776_d_n2, assign18960_e13776_d_n4, assign18960_e13776_d_n5, assign18960_e13776_d_n6, assign18960_e13776_d_n7, assign18960_e13776_d_n8, assign18960_e13776_d_n9, assign18960_e13776_d_n10, assign18960_e13776_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18960_e13778;
        locals.var_tmf2_dn0 = assign18960_e13778_d_n0;
        locals.var_tmf2_dn2 = assign18960_e13778_d_n2;
        locals.var_tmf2_dn4 = assign18960_e13778_d_n4;
        locals.var_tmf2_dn5 = assign18960_e13778_d_n5;
        locals.var_tmf2_dn6 = assign18960_e13778_d_n6;
        locals.var_tmf2_dn7 = assign18960_e13778_d_n7;
        locals.var_tmf2_dn8 = assign18960_e13778_d_n8;
        locals.var_tmf2_dn9 = assign18960_e13778_d_n9;
        locals.var_tmf2_dn10 = assign18960_e13778_d_n10;
        locals.var_tmf2_dn13 = assign18960_e13778_d_n13;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_45(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign18970_e13793, assign18970_e13793_d_n0, assign18970_e13793_d_n2, assign18970_e13793_d_n4, assign18970_e13793_d_n5, assign18970_e13793_d_n6, assign18970_e13793_d_n7, assign18970_e13793_d_n8, assign18970_e13793_d_n9, assign18970_e13793_d_n10, assign18970_e13793_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign18970_e13788: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18970_e13790: f64 = (assign18970_e13788 + locals.var_tmf2);
        let assign18970_e13791: f64 = (assign18970_e13790).sqrt();
        (assign18970_e13791, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18970_e13791)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18970_e13791)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18970_e13791)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18970_e13791)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18970_e13791)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18970_e13791)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18970_e13791)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18970_e13791)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18970_e13791)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign18970_e13791)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign18970_e13793;
        locals.var_tmf2_dn0 = assign18970_e13793_d_n0;
        locals.var_tmf2_dn2 = assign18970_e13793_d_n2;
        locals.var_tmf2_dn4 = assign18970_e13793_d_n4;
        locals.var_tmf2_dn5 = assign18970_e13793_d_n5;
        locals.var_tmf2_dn6 = assign18970_e13793_d_n6;
        locals.var_tmf2_dn7 = assign18970_e13793_d_n7;
        locals.var_tmf2_dn8 = assign18970_e13793_d_n8;
        locals.var_tmf2_dn9 = assign18970_e13793_d_n9;
        locals.var_tmf2_dn10 = assign18970_e13793_d_n10;
        locals.var_tmf2_dn13 = assign18970_e13793_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign18980_e13809, assign18980_e13809_d_n0, assign18980_e13809_d_n2, assign18980_e13809_d_n4, assign18980_e13809_d_n5, assign18980_e13809_d_n6, assign18980_e13809_d_n7, assign18980_e13809_d_n8, assign18980_e13809_d_n9, assign18980_e13809_d_n10, assign18980_e13809_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign18980_e13805: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18980_e13806: f64 = (1.0 + assign18980_e13805);
        let assign18980_e13807: f64 = (0.5 * assign18980_e13806);
        (assign18980_e13807, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign18980_e13809;
        locals.var_t0_dn0 = assign18980_e13809_d_n0;
        locals.var_t0_dn2 = assign18980_e13809_d_n2;
        locals.var_t0_dn4 = assign18980_e13809_d_n4;
        locals.var_t0_dn5 = assign18980_e13809_d_n5;
        locals.var_t0_dn6 = assign18980_e13809_d_n6;
        locals.var_t0_dn7 = assign18980_e13809_d_n7;
        locals.var_t0_dn8 = assign18980_e13809_d_n8;
        locals.var_t0_dn9 = assign18980_e13809_d_n9;
        locals.var_t0_dn10 = assign18980_e13809_d_n10;
        locals.var_t0_dn13 = assign18980_e13809_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign18990_e13827, assign18990_e13827_d_n0, assign18990_e13827_d_n2, assign18990_e13827_d_n4, assign18990_e13827_d_n5, assign18990_e13827_d_n6, assign18990_e13827_d_n7, assign18990_e13827_d_n8, assign18990_e13827_d_n9, assign18990_e13827_d_n10, assign18990_e13827_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign18990_e13819: f64 = (0.005 * locals.var_uc_rdvd);
        let assign18990_e13823: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18990_e13824: f64 = (0.5 * assign18990_e13823);
        let assign18990_e13825: f64 = (assign18990_e13819 + assign18990_e13824);
        (assign18990_e13825, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn13,)
    }
};
        locals.var_rdvde = assign18990_e13827;
        locals.var_rdvde_dn0 = assign18990_e13827_d_n0;
        locals.var_rdvde_dn2 = assign18990_e13827_d_n2;
        locals.var_rdvde_dn4 = assign18990_e13827_d_n4;
        locals.var_rdvde_dn5 = assign18990_e13827_d_n5;
        locals.var_rdvde_dn6 = assign18990_e13827_d_n6;
        locals.var_rdvde_dn7 = assign18990_e13827_d_n7;
        locals.var_rdvde_dn8 = assign18990_e13827_d_n8;
        locals.var_rdvde_dn9 = assign18990_e13827_d_n9;
        locals.var_rdvde_dn10 = assign18990_e13827_d_n10;
        locals.var_rdvde_dn13 = assign18990_e13827_d_n13;
        locals.var_rdvde_rv = 0.0;

        let (assign19000_e13848, assign19000_e13848_d_n0, assign19000_e13848_d_n2, assign19000_e13848_d_n4, assign19000_e13848_d_n5, assign19000_e13848_d_n6, assign19000_e13848_d_n7, assign19000_e13848_d_n8, assign19000_e13848_d_n9, assign19000_e13848_d_n10, assign19000_e13848_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard385 == 0.0)) {
        let assign19000_e13839: f64 = (locals.var_mks_rdvdtemp1 * locals.var_tdiff);
        let assign19000_e13840: f64 = (locals.var_uc_rdvd + assign19000_e13839);
        let assign19000_e13843: f64 = (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2);
        let assign19000_e13844: f64 = (assign19000_e13840 + assign19000_e13843);
        let assign19000_e13846: f64 = (assign19000_e13844 * locals.var_t2);
        (assign19000_e13846, ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn0) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn0)) * locals.var_t2) + (assign19000_e13844 * locals.var_t2_dn0)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn2) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn2)) * locals.var_t2) + (assign19000_e13844 * locals.var_t2_dn2)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn4) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn4)) * locals.var_t2) + (assign19000_e13844 * locals.var_t2_dn4)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn5) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn5)) * locals.var_t2) + (assign19000_e13844 * locals.var_t2_dn5)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn6) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn6)) * locals.var_t2) + (assign19000_e13844 * locals.var_t2_dn6)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn7) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn7)) * locals.var_t2) + (assign19000_e13844 * locals.var_t2_dn7)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn8) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn8)) * locals.var_t2) + (assign19000_e13844 * locals.var_t2_dn8)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn9) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn9)) * locals.var_t2) + (assign19000_e13844 * locals.var_t2_dn9)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn10) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn10)) * locals.var_t2) + (assign19000_e13844 * locals.var_t2_dn10)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn13) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn13)) * locals.var_t2) + (assign19000_e13844 * locals.var_t2_dn13)),)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn13,)
    }
};
        locals.var_rdvde = assign19000_e13848;
        locals.var_rdvde_dn0 = assign19000_e13848_d_n0;
        locals.var_rdvde_dn2 = assign19000_e13848_d_n2;
        locals.var_rdvde_dn4 = assign19000_e13848_d_n4;
        locals.var_rdvde_dn5 = assign19000_e13848_d_n5;
        locals.var_rdvde_dn6 = assign19000_e13848_d_n6;
        locals.var_rdvde_dn7 = assign19000_e13848_d_n7;
        locals.var_rdvde_dn8 = assign19000_e13848_d_n8;
        locals.var_rdvde_dn9 = assign19000_e13848_d_n9;
        locals.var_rdvde_dn10 = assign19000_e13848_d_n10;
        locals.var_rdvde_dn13 = assign19000_e13848_d_n13;
        locals.var_rdvde_rv = 0.0;

        let (assign19010_e13867, assign19010_e13867_d_n0, assign19010_e13867_d_n2, assign19010_e13867_d_n4, assign19010_e13867_d_n5, assign19010_e13867_d_n6, assign19010_e13867_d_n7, assign19010_e13867_d_n8, assign19010_e13867_d_n9, assign19010_e13867_d_n10, assign19010_e13867_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard385 == 0.0)) {
        let assign19010_e13860: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19010_e13861: f64 = (locals.var_rdvde - assign19010_e13860);
        let assign19010_e13864: f64 = (0.01 * locals.var_uc_rdvd);
        let assign19010_e13865: f64 = (assign19010_e13861 - assign19010_e13864);
        (assign19010_e13865, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign19010_e13867;
        locals.var_tmf1_dn0 = assign19010_e13867_d_n0;
        locals.var_tmf1_dn2 = assign19010_e13867_d_n2;
        locals.var_tmf1_dn4 = assign19010_e13867_d_n4;
        locals.var_tmf1_dn5 = assign19010_e13867_d_n5;
        locals.var_tmf1_dn6 = assign19010_e13867_d_n6;
        locals.var_tmf1_dn7 = assign19010_e13867_d_n7;
        locals.var_tmf1_dn8 = assign19010_e13867_d_n8;
        locals.var_tmf1_dn9 = assign19010_e13867_d_n9;
        locals.var_tmf1_dn10 = assign19010_e13867_d_n10;
        locals.var_tmf1_dn13 = assign19010_e13867_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign19020_e13886, assign19020_e13886_d_n0, assign19020_e13886_d_n2, assign19020_e13886_d_n4, assign19020_e13886_d_n5, assign19020_e13886_d_n6, assign19020_e13886_d_n7, assign19020_e13886_d_n8, assign19020_e13886_d_n9, assign19020_e13886_d_n10, assign19020_e13886_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard385 == 0.0)) {
        let assign19020_e13879: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19020_e13880: f64 = (4.0 * assign19020_e13879);
        let assign19020_e13883: f64 = (0.01 * locals.var_uc_rdvd);
        let assign19020_e13884: f64 = (assign19020_e13880 * assign19020_e13883);
        (assign19020_e13884, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign19020_e13886;
        locals.var_tmf2_dn0 = assign19020_e13886_d_n0;
        locals.var_tmf2_dn2 = assign19020_e13886_d_n2;
        locals.var_tmf2_dn4 = assign19020_e13886_d_n4;
        locals.var_tmf2_dn5 = assign19020_e13886_d_n5;
        locals.var_tmf2_dn6 = assign19020_e13886_d_n6;
        locals.var_tmf2_dn7 = assign19020_e13886_d_n7;
        locals.var_tmf2_dn8 = assign19020_e13886_d_n8;
        locals.var_tmf2_dn9 = assign19020_e13886_d_n9;
        locals.var_tmf2_dn10 = assign19020_e13886_d_n10;
        locals.var_tmf2_dn13 = assign19020_e13886_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign19030_e13903, assign19030_e13903_d_n0, assign19030_e13903_d_n2, assign19030_e13903_d_n4, assign19030_e13903_d_n5, assign19030_e13903_d_n6, assign19030_e13903_d_n7, assign19030_e13903_d_n8, assign19030_e13903_d_n9, assign19030_e13903_d_n10, assign19030_e13903_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard385 == 0.0)) {
        let (assign19030_e13901, assign19030_e13901_d_n0, assign19030_e13901_d_n2, assign19030_e13901_d_n4, assign19030_e13901_d_n5, assign19030_e13901_d_n6, assign19030_e13901_d_n7, assign19030_e13901_d_n8, assign19030_e13901_d_n9, assign19030_e13901_d_n10, assign19030_e13901_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign19030_e13900: f64 = (-locals.var_tmf2);
                (assign19030_e13900, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign19030_e13901, assign19030_e13901_d_n0, assign19030_e13901_d_n2, assign19030_e13901_d_n4, assign19030_e13901_d_n5, assign19030_e13901_d_n6, assign19030_e13901_d_n7, assign19030_e13901_d_n8, assign19030_e13901_d_n9, assign19030_e13901_d_n10, assign19030_e13901_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign19030_e13903;
        locals.var_tmf2_dn0 = assign19030_e13903_d_n0;
        locals.var_tmf2_dn2 = assign19030_e13903_d_n2;
        locals.var_tmf2_dn4 = assign19030_e13903_d_n4;
        locals.var_tmf2_dn5 = assign19030_e13903_d_n5;
        locals.var_tmf2_dn6 = assign19030_e13903_d_n6;
        locals.var_tmf2_dn7 = assign19030_e13903_d_n7;
        locals.var_tmf2_dn8 = assign19030_e13903_d_n8;
        locals.var_tmf2_dn9 = assign19030_e13903_d_n9;
        locals.var_tmf2_dn10 = assign19030_e13903_d_n10;
        locals.var_tmf2_dn13 = assign19030_e13903_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign19040_e13919, assign19040_e13919_d_n0, assign19040_e13919_d_n2, assign19040_e13919_d_n4, assign19040_e13919_d_n5, assign19040_e13919_d_n6, assign19040_e13919_d_n7, assign19040_e13919_d_n8, assign19040_e13919_d_n9, assign19040_e13919_d_n10, assign19040_e13919_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard385 == 0.0)) {
        let assign19040_e13914: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19040_e13916: f64 = (assign19040_e13914 + locals.var_tmf2);
        let assign19040_e13917: f64 = (assign19040_e13916).sqrt();
        (assign19040_e13917, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19040_e13917)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19040_e13917)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign19040_e13917)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign19040_e13917)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign19040_e13917)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign19040_e13917)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign19040_e13917)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign19040_e13917)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign19040_e13917)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign19040_e13917)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign19040_e13919;
        locals.var_tmf2_dn0 = assign19040_e13919_d_n0;
        locals.var_tmf2_dn2 = assign19040_e13919_d_n2;
        locals.var_tmf2_dn4 = assign19040_e13919_d_n4;
        locals.var_tmf2_dn5 = assign19040_e13919_d_n5;
        locals.var_tmf2_dn6 = assign19040_e13919_d_n6;
        locals.var_tmf2_dn7 = assign19040_e13919_d_n7;
        locals.var_tmf2_dn8 = assign19040_e13919_d_n8;
        locals.var_tmf2_dn9 = assign19040_e13919_d_n9;
        locals.var_tmf2_dn10 = assign19040_e13919_d_n10;
        locals.var_tmf2_dn13 = assign19040_e13919_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign19050_e13936, assign19050_e13936_d_n0, assign19050_e13936_d_n2, assign19050_e13936_d_n4, assign19050_e13936_d_n5, assign19050_e13936_d_n6, assign19050_e13936_d_n7, assign19050_e13936_d_n8, assign19050_e13936_d_n9, assign19050_e13936_d_n10, assign19050_e13936_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard385 == 0.0)) {
        let assign19050_e13932: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign19050_e13933: f64 = (1.0 + assign19050_e13932);
        let assign19050_e13934: f64 = (0.5 * assign19050_e13933);
        (assign19050_e13934, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign19050_e13936;
        locals.var_t0_dn0 = assign19050_e13936_d_n0;
        locals.var_t0_dn2 = assign19050_e13936_d_n2;
        locals.var_t0_dn4 = assign19050_e13936_d_n4;
        locals.var_t0_dn5 = assign19050_e13936_d_n5;
        locals.var_t0_dn6 = assign19050_e13936_d_n6;
        locals.var_t0_dn7 = assign19050_e13936_d_n7;
        locals.var_t0_dn8 = assign19050_e13936_d_n8;
        locals.var_t0_dn9 = assign19050_e13936_d_n9;
        locals.var_t0_dn10 = assign19050_e13936_d_n10;
        locals.var_t0_dn13 = assign19050_e13936_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign19060_e13955, assign19060_e13955_d_n0, assign19060_e13955_d_n2, assign19060_e13955_d_n4, assign19060_e13955_d_n5, assign19060_e13955_d_n6, assign19060_e13955_d_n7, assign19060_e13955_d_n8, assign19060_e13955_d_n9, assign19060_e13955_d_n10, assign19060_e13955_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard385 == 0.0)) {
        let assign19060_e13947: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19060_e13951: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19060_e13952: f64 = (0.5 * assign19060_e13951);
        let assign19060_e13953: f64 = (assign19060_e13947 + assign19060_e13952);
        (assign19060_e13953, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn13,)
    }
};
        locals.var_rdvde = assign19060_e13955;
        locals.var_rdvde_dn0 = assign19060_e13955_d_n0;
        locals.var_rdvde_dn2 = assign19060_e13955_d_n2;
        locals.var_rdvde_dn4 = assign19060_e13955_d_n4;
        locals.var_rdvde_dn5 = assign19060_e13955_d_n5;
        locals.var_rdvde_dn6 = assign19060_e13955_d_n6;
        locals.var_rdvde_dn7 = assign19060_e13955_d_n7;
        locals.var_rdvde_dn8 = assign19060_e13955_d_n8;
        locals.var_rdvde_dn9 = assign19060_e13955_d_n9;
        locals.var_rdvde_dn10 = assign19060_e13955_d_n10;
        locals.var_rdvde_dn13 = assign19060_e13955_d_n13;
        locals.var_rdvde_rv = 0.0;

        let (assign19070_e13979, assign19070_e13979_d_n0, assign19070_e13979_d_n2, assign19070_e13979_d_n4, assign19070_e13979_d_n5, assign19070_e13979_d_n6, assign19070_e13979_d_n7, assign19070_e13979_d_n8, assign19070_e13979_d_n9, assign19070_e13979_d_n10, assign19070_e13979_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign19070_e13964: f64 = (p.p69 * locals.var_uc_rdslp1);
        let assign19070_e13966: f64 = (assign19070_e13964 * 1000000.0);
        let assign19070_e13968: f64 = (assign19070_e13966 + locals.var_uc_rdict1);
        let assign19070_e13969: f64 = (locals.var_rdvdtemp0 * assign19070_e13968);
        let assign19070_e13972: f64 = (p.p70 * p.p100);
        let assign19070_e13974: f64 = (assign19070_e13972 * 1000000.0);
        let assign19070_e13976: f64 = (assign19070_e13974 + p.p101);
        let assign19070_e13977: f64 = (assign19070_e13969 * assign19070_e13976);
        (assign19070_e13977, ((locals.var_rdvdtemp0_dn0 * assign19070_e13968) * assign19070_e13976), ((locals.var_rdvdtemp0_dn2 * assign19070_e13968) * assign19070_e13976), ((locals.var_rdvdtemp0_dn4 * assign19070_e13968) * assign19070_e13976), ((locals.var_rdvdtemp0_dn5 * assign19070_e13968) * assign19070_e13976), ((locals.var_rdvdtemp0_dn6 * assign19070_e13968) * assign19070_e13976), ((locals.var_rdvdtemp0_dn7 * assign19070_e13968) * assign19070_e13976), ((locals.var_rdvdtemp0_dn8 * assign19070_e13968) * assign19070_e13976), ((locals.var_rdvdtemp0_dn9 * assign19070_e13968) * assign19070_e13976), ((locals.var_rdvdtemp0_dn10 * assign19070_e13968) * assign19070_e13976), ((locals.var_rdvdtemp0_dn13 * assign19070_e13968) * assign19070_e13976),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign19070_e13979;
        locals.var_t4_dn0 = assign19070_e13979_d_n0;
        locals.var_t4_dn2 = assign19070_e13979_d_n2;
        locals.var_t4_dn4 = assign19070_e13979_d_n4;
        locals.var_t4_dn5 = assign19070_e13979_d_n5;
        locals.var_t4_dn6 = assign19070_e13979_d_n6;
        locals.var_t4_dn7 = assign19070_e13979_d_n7;
        locals.var_t4_dn8 = assign19070_e13979_d_n8;
        locals.var_t4_dn9 = assign19070_e13979_d_n9;
        locals.var_t4_dn10 = assign19070_e13979_d_n10;
        locals.var_t4_dn13 = assign19070_e13979_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign19080_e13993, assign19080_e13993_d_n0, assign19080_e13993_d_n2, assign19080_e13993_d_n4, assign19080_e13993_d_n5, assign19080_e13993_d_n6, assign19080_e13993_d_n7, assign19080_e13993_d_n8, assign19080_e13993_d_n9, assign19080_e13993_d_n10, assign19080_e13993_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign19080_e13987: f64 = (1.0 - locals.var_uc_rdov13);
        let assign19080_e13989: f64 = (assign19080_e13987 * p.p66);
        let assign19080_e13991: f64 = (assign19080_e13989 * 1000000.0);
        (assign19080_e13991, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign19080_e13993;
        locals.var_t1_dn0 = assign19080_e13993_d_n0;
        locals.var_t1_dn2 = assign19080_e13993_d_n2;
        locals.var_t1_dn4 = assign19080_e13993_d_n4;
        locals.var_t1_dn5 = assign19080_e13993_d_n5;
        locals.var_t1_dn6 = assign19080_e13993_d_n6;
        locals.var_t1_dn7 = assign19080_e13993_d_n7;
        locals.var_t1_dn8 = assign19080_e13993_d_n8;
        locals.var_t1_dn9 = assign19080_e13993_d_n9;
        locals.var_t1_dn10 = assign19080_e13993_d_n10;
        locals.var_t1_dn13 = assign19080_e13993_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign19090_e14009, assign19090_e14009_d_n0, assign19090_e14009_d_n2, assign19090_e14009_d_n4, assign19090_e14009_d_n5, assign19090_e14009_d_n6, assign19090_e14009_d_n7, assign19090_e14009_d_n8, assign19090_e14009_d_n9, assign19090_e14009_d_n10, assign19090_e14009_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign19090_e14001: f64 = (locals.var_t8 * p.p66);
        let assign19090_e14003: f64 = (assign19090_e14001 * 1000000.0);
        let assign19090_e14005: f64 = (assign19090_e14003 + 1.0);
        let assign19090_e14007: f64 = (assign19090_e14005 + p.p98);
        (assign19090_e14007, ((locals.var_t8_dn0 * p.p66) * 1000000.0), ((locals.var_t8_dn2 * p.p66) * 1000000.0), ((locals.var_t8_dn4 * p.p66) * 1000000.0), ((locals.var_t8_dn5 * p.p66) * 1000000.0), ((locals.var_t8_dn6 * p.p66) * 1000000.0), ((locals.var_t8_dn7 * p.p66) * 1000000.0), ((locals.var_t8_dn8 * p.p66) * 1000000.0), ((locals.var_t8_dn9 * p.p66) * 1000000.0), ((locals.var_t8_dn10 * p.p66) * 1000000.0), ((locals.var_t8_dn13 * p.p66) * 1000000.0),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign19090_e14009;
        locals.var_t3_dn0 = assign19090_e14009_d_n0;
        locals.var_t3_dn2 = assign19090_e14009_d_n2;
        locals.var_t3_dn4 = assign19090_e14009_d_n4;
        locals.var_t3_dn5 = assign19090_e14009_d_n5;
        locals.var_t3_dn6 = assign19090_e14009_d_n6;
        locals.var_t3_dn7 = assign19090_e14009_d_n7;
        locals.var_t3_dn8 = assign19090_e14009_d_n8;
        locals.var_t3_dn9 = assign19090_e14009_d_n9;
        locals.var_t3_dn10 = assign19090_e14009_d_n10;
        locals.var_t3_dn13 = assign19090_e14009_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign19100_e14023, assign19100_e14023_d_n0, assign19100_e14023_d_n2, assign19100_e14023_d_n4, assign19100_e14023_d_n5, assign19100_e14023_d_n6, assign19100_e14023_d_n7, assign19100_e14023_d_n8, assign19100_e14023_d_n9, assign19100_e14023_d_n10, assign19100_e14023_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign19100_e14017: f64 = (locals.var_t3 * locals.var_t4);
        let assign19100_e14019: f64 = (assign19100_e14017 - locals.var_t4);
        let assign19100_e14021: f64 = (assign19100_e14019 - 0.01);
        (assign19100_e14021, (((locals.var_t3_dn0 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn0)) - locals.var_t4_dn0), (((locals.var_t3_dn2 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn2)) - locals.var_t4_dn2), (((locals.var_t3_dn4 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn4)) - locals.var_t4_dn4), (((locals.var_t3_dn5 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn5)) - locals.var_t4_dn5), (((locals.var_t3_dn6 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn6)) - locals.var_t4_dn6), (((locals.var_t3_dn7 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn7)) - locals.var_t4_dn7), (((locals.var_t3_dn8 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn8)) - locals.var_t4_dn8), (((locals.var_t3_dn9 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn9)) - locals.var_t4_dn9), (((locals.var_t3_dn10 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn10)) - locals.var_t4_dn10), (((locals.var_t3_dn13 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn13)) - locals.var_t4_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign19100_e14023;
        locals.var_tmf1_dn0 = assign19100_e14023_d_n0;
        locals.var_tmf1_dn2 = assign19100_e14023_d_n2;
        locals.var_tmf1_dn4 = assign19100_e14023_d_n4;
        locals.var_tmf1_dn5 = assign19100_e14023_d_n5;
        locals.var_tmf1_dn6 = assign19100_e14023_d_n6;
        locals.var_tmf1_dn7 = assign19100_e14023_d_n7;
        locals.var_tmf1_dn8 = assign19100_e14023_d_n8;
        locals.var_tmf1_dn9 = assign19100_e14023_d_n9;
        locals.var_tmf1_dn10 = assign19100_e14023_d_n10;
        locals.var_tmf1_dn13 = assign19100_e14023_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign19110_e14035, assign19110_e14035_d_n0, assign19110_e14035_d_n2, assign19110_e14035_d_n4, assign19110_e14035_d_n5, assign19110_e14035_d_n6, assign19110_e14035_d_n7, assign19110_e14035_d_n8, assign19110_e14035_d_n9, assign19110_e14035_d_n10, assign19110_e14035_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign19110_e14031: f64 = (4.0 * locals.var_t4);
        let assign19110_e14033: f64 = (assign19110_e14031 * 0.01);
        (assign19110_e14033, ((4.0 * locals.var_t4_dn0) * 0.01), ((4.0 * locals.var_t4_dn2) * 0.01), ((4.0 * locals.var_t4_dn4) * 0.01), ((4.0 * locals.var_t4_dn5) * 0.01), ((4.0 * locals.var_t4_dn6) * 0.01), ((4.0 * locals.var_t4_dn7) * 0.01), ((4.0 * locals.var_t4_dn8) * 0.01), ((4.0 * locals.var_t4_dn9) * 0.01), ((4.0 * locals.var_t4_dn10) * 0.01), ((4.0 * locals.var_t4_dn13) * 0.01),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign19110_e14035;
        locals.var_tmf2_dn0 = assign19110_e14035_d_n0;
        locals.var_tmf2_dn2 = assign19110_e14035_d_n2;
        locals.var_tmf2_dn4 = assign19110_e14035_d_n4;
        locals.var_tmf2_dn5 = assign19110_e14035_d_n5;
        locals.var_tmf2_dn6 = assign19110_e14035_d_n6;
        locals.var_tmf2_dn7 = assign19110_e14035_d_n7;
        locals.var_tmf2_dn8 = assign19110_e14035_d_n8;
        locals.var_tmf2_dn9 = assign19110_e14035_d_n9;
        locals.var_tmf2_dn10 = assign19110_e14035_d_n10;
        locals.var_tmf2_dn13 = assign19110_e14035_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign19120_e14049, assign19120_e14049_d_n0, assign19120_e14049_d_n2, assign19120_e14049_d_n4, assign19120_e14049_d_n5, assign19120_e14049_d_n6, assign19120_e14049_d_n7, assign19120_e14049_d_n8, assign19120_e14049_d_n9, assign19120_e14049_d_n10, assign19120_e14049_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let (assign19120_e14047, assign19120_e14047_d_n0, assign19120_e14047_d_n2, assign19120_e14047_d_n4, assign19120_e14047_d_n5, assign19120_e14047_d_n6, assign19120_e14047_d_n7, assign19120_e14047_d_n8, assign19120_e14047_d_n9, assign19120_e14047_d_n10, assign19120_e14047_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign19120_e14046: f64 = (-locals.var_tmf2);
                (assign19120_e14046, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign19120_e14047, assign19120_e14047_d_n0, assign19120_e14047_d_n2, assign19120_e14047_d_n4, assign19120_e14047_d_n5, assign19120_e14047_d_n6, assign19120_e14047_d_n7, assign19120_e14047_d_n8, assign19120_e14047_d_n9, assign19120_e14047_d_n10, assign19120_e14047_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign19120_e14049;
        locals.var_tmf2_dn0 = assign19120_e14049_d_n0;
        locals.var_tmf2_dn2 = assign19120_e14049_d_n2;
        locals.var_tmf2_dn4 = assign19120_e14049_d_n4;
        locals.var_tmf2_dn5 = assign19120_e14049_d_n5;
        locals.var_tmf2_dn6 = assign19120_e14049_d_n6;
        locals.var_tmf2_dn7 = assign19120_e14049_d_n7;
        locals.var_tmf2_dn8 = assign19120_e14049_d_n8;
        locals.var_tmf2_dn9 = assign19120_e14049_d_n9;
        locals.var_tmf2_dn10 = assign19120_e14049_d_n10;
        locals.var_tmf2_dn13 = assign19120_e14049_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign19130_e14062, assign19130_e14062_d_n0, assign19130_e14062_d_n2, assign19130_e14062_d_n4, assign19130_e14062_d_n5, assign19130_e14062_d_n6, assign19130_e14062_d_n7, assign19130_e14062_d_n8, assign19130_e14062_d_n9, assign19130_e14062_d_n10, assign19130_e14062_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign19130_e14057: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19130_e14059: f64 = (assign19130_e14057 + locals.var_tmf2);
        let assign19130_e14060: f64 = (assign19130_e14059).sqrt();
        (assign19130_e14060, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19130_e14060)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19130_e14060)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign19130_e14060)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign19130_e14060)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign19130_e14060)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign19130_e14060)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign19130_e14060)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign19130_e14060)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign19130_e14060)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign19130_e14060)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign19130_e14062;
        locals.var_tmf2_dn0 = assign19130_e14062_d_n0;
        locals.var_tmf2_dn2 = assign19130_e14062_d_n2;
        locals.var_tmf2_dn4 = assign19130_e14062_d_n4;
        locals.var_tmf2_dn5 = assign19130_e14062_d_n5;
        locals.var_tmf2_dn6 = assign19130_e14062_d_n6;
        locals.var_tmf2_dn7 = assign19130_e14062_d_n7;
        locals.var_tmf2_dn8 = assign19130_e14062_d_n8;
        locals.var_tmf2_dn9 = assign19130_e14062_d_n9;
        locals.var_tmf2_dn10 = assign19130_e14062_d_n10;
        locals.var_tmf2_dn13 = assign19130_e14062_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign19140_e14076, assign19140_e14076_d_n0, assign19140_e14076_d_n2, assign19140_e14076_d_n4, assign19140_e14076_d_n5, assign19140_e14076_d_n6, assign19140_e14076_d_n7, assign19140_e14076_d_n8, assign19140_e14076_d_n9, assign19140_e14076_d_n10, assign19140_e14076_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign19140_e14072: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign19140_e14073: f64 = (1.0 + assign19140_e14072);
        let assign19140_e14074: f64 = (0.5 * assign19140_e14073);
        (assign19140_e14074, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign19140_e14076;
        locals.var_t6_dn0 = assign19140_e14076_d_n0;
        locals.var_t6_dn2 = assign19140_e14076_d_n2;
        locals.var_t6_dn4 = assign19140_e14076_d_n4;
        locals.var_t6_dn5 = assign19140_e14076_d_n5;
        locals.var_t6_dn6 = assign19140_e14076_d_n6;
        locals.var_t6_dn7 = assign19140_e14076_d_n7;
        locals.var_t6_dn8 = assign19140_e14076_d_n8;
        locals.var_t6_dn9 = assign19140_e14076_d_n9;
        locals.var_t6_dn10 = assign19140_e14076_d_n10;
        locals.var_t6_dn13 = assign19140_e14076_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign19150_e14090, assign19150_e14090_d_n0, assign19150_e14090_d_n2, assign19150_e14090_d_n4, assign19150_e14090_d_n5, assign19150_e14090_d_n6, assign19150_e14090_d_n7, assign19150_e14090_d_n8, assign19150_e14090_d_n9, assign19150_e14090_d_n10, assign19150_e14090_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign19150_e14086: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19150_e14087: f64 = (0.5 * assign19150_e14086);
        let assign19150_e14088: f64 = (locals.var_t4 + assign19150_e14087);
        (assign19150_e14088, (locals.var_t4_dn0 + (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t4_dn2 + (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t4_dn4 + (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t4_dn5 + (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t4_dn6 + (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t4_dn7 + (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t4_dn8 + (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t4_dn9 + (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t4_dn10 + (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t4_dn13 + (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign19150_e14090;
        locals.var_t5_dn0 = assign19150_e14090_d_n0;
        locals.var_t5_dn2 = assign19150_e14090_d_n2;
        locals.var_t5_dn4 = assign19150_e14090_d_n4;
        locals.var_t5_dn5 = assign19150_e14090_d_n5;
        locals.var_t5_dn6 = assign19150_e14090_d_n6;
        locals.var_t5_dn7 = assign19150_e14090_d_n7;
        locals.var_t5_dn8 = assign19150_e14090_d_n8;
        locals.var_t5_dn9 = assign19150_e14090_d_n9;
        locals.var_t5_dn10 = assign19150_e14090_d_n10;
        locals.var_t5_dn13 = assign19150_e14090_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign19160_e14106, assign19160_e14106_d_n0, assign19160_e14106_d_n2, assign19160_e14106_d_n4, assign19160_e14106_d_n5, assign19160_e14106_d_n6, assign19160_e14106_d_n7, assign19160_e14106_d_n8, assign19160_e14106_d_n9, assign19160_e14106_d_n10, assign19160_e14106_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign19160_e14099: f64 = (p.p98 + 1.0);
        let assign19160_e14100: f64 = (locals.var_t4 * assign19160_e14099);
        let assign19160_e14102: f64 = (assign19160_e14100 - locals.var_t5);
        let assign19160_e14104: f64 = (assign19160_e14102 - 5e-5);
        (assign19160_e14104, ((locals.var_t4_dn0 * assign19160_e14099) - locals.var_t5_dn0), ((locals.var_t4_dn2 * assign19160_e14099) - locals.var_t5_dn2), ((locals.var_t4_dn4 * assign19160_e14099) - locals.var_t5_dn4), ((locals.var_t4_dn5 * assign19160_e14099) - locals.var_t5_dn5), ((locals.var_t4_dn6 * assign19160_e14099) - locals.var_t5_dn6), ((locals.var_t4_dn7 * assign19160_e14099) - locals.var_t5_dn7), ((locals.var_t4_dn8 * assign19160_e14099) - locals.var_t5_dn8), ((locals.var_t4_dn9 * assign19160_e14099) - locals.var_t5_dn9), ((locals.var_t4_dn10 * assign19160_e14099) - locals.var_t5_dn10), ((locals.var_t4_dn13 * assign19160_e14099) - locals.var_t5_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign19160_e14106;
        locals.var_tmf1_dn0 = assign19160_e14106_d_n0;
        locals.var_tmf1_dn2 = assign19160_e14106_d_n2;
        locals.var_tmf1_dn4 = assign19160_e14106_d_n4;
        locals.var_tmf1_dn5 = assign19160_e14106_d_n5;
        locals.var_tmf1_dn6 = assign19160_e14106_d_n6;
        locals.var_tmf1_dn7 = assign19160_e14106_d_n7;
        locals.var_tmf1_dn8 = assign19160_e14106_d_n8;
        locals.var_tmf1_dn9 = assign19160_e14106_d_n9;
        locals.var_tmf1_dn10 = assign19160_e14106_d_n10;
        locals.var_tmf1_dn13 = assign19160_e14106_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign19170_e14122, assign19170_e14122_d_n0, assign19170_e14122_d_n2, assign19170_e14122_d_n4, assign19170_e14122_d_n5, assign19170_e14122_d_n6, assign19170_e14122_d_n7, assign19170_e14122_d_n8, assign19170_e14122_d_n9, assign19170_e14122_d_n10, assign19170_e14122_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign19170_e14116: f64 = (p.p98 + 1.0);
        let assign19170_e14117: f64 = (locals.var_t4 * assign19170_e14116);
        let assign19170_e14118: f64 = (4.0 * assign19170_e14117);
        let assign19170_e14120: f64 = (assign19170_e14118 * 5e-5);
        (assign19170_e14120, ((4.0 * (locals.var_t4_dn0 * assign19170_e14116)) * 5e-5), ((4.0 * (locals.var_t4_dn2 * assign19170_e14116)) * 5e-5), ((4.0 * (locals.var_t4_dn4 * assign19170_e14116)) * 5e-5), ((4.0 * (locals.var_t4_dn5 * assign19170_e14116)) * 5e-5), ((4.0 * (locals.var_t4_dn6 * assign19170_e14116)) * 5e-5), ((4.0 * (locals.var_t4_dn7 * assign19170_e14116)) * 5e-5), ((4.0 * (locals.var_t4_dn8 * assign19170_e14116)) * 5e-5), ((4.0 * (locals.var_t4_dn9 * assign19170_e14116)) * 5e-5), ((4.0 * (locals.var_t4_dn10 * assign19170_e14116)) * 5e-5), ((4.0 * (locals.var_t4_dn13 * assign19170_e14116)) * 5e-5),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign19170_e14122;
        locals.var_tmf2_dn0 = assign19170_e14122_d_n0;
        locals.var_tmf2_dn2 = assign19170_e14122_d_n2;
        locals.var_tmf2_dn4 = assign19170_e14122_d_n4;
        locals.var_tmf2_dn5 = assign19170_e14122_d_n5;
        locals.var_tmf2_dn6 = assign19170_e14122_d_n6;
        locals.var_tmf2_dn7 = assign19170_e14122_d_n7;
        locals.var_tmf2_dn8 = assign19170_e14122_d_n8;
        locals.var_tmf2_dn9 = assign19170_e14122_d_n9;
        locals.var_tmf2_dn10 = assign19170_e14122_d_n10;
        locals.var_tmf2_dn13 = assign19170_e14122_d_n13;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_46(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign19180_e14136, assign19180_e14136_d_n0, assign19180_e14136_d_n2, assign19180_e14136_d_n4, assign19180_e14136_d_n5, assign19180_e14136_d_n6, assign19180_e14136_d_n7, assign19180_e14136_d_n8, assign19180_e14136_d_n9, assign19180_e14136_d_n10, assign19180_e14136_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let (assign19180_e14134, assign19180_e14134_d_n0, assign19180_e14134_d_n2, assign19180_e14134_d_n4, assign19180_e14134_d_n5, assign19180_e14134_d_n6, assign19180_e14134_d_n7, assign19180_e14134_d_n8, assign19180_e14134_d_n9, assign19180_e14134_d_n10, assign19180_e14134_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign19180_e14133: f64 = (-locals.var_tmf2);
                (assign19180_e14133, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign19180_e14134, assign19180_e14134_d_n0, assign19180_e14134_d_n2, assign19180_e14134_d_n4, assign19180_e14134_d_n5, assign19180_e14134_d_n6, assign19180_e14134_d_n7, assign19180_e14134_d_n8, assign19180_e14134_d_n9, assign19180_e14134_d_n10, assign19180_e14134_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign19180_e14136;
        locals.var_tmf2_dn0 = assign19180_e14136_d_n0;
        locals.var_tmf2_dn2 = assign19180_e14136_d_n2;
        locals.var_tmf2_dn4 = assign19180_e14136_d_n4;
        locals.var_tmf2_dn5 = assign19180_e14136_d_n5;
        locals.var_tmf2_dn6 = assign19180_e14136_d_n6;
        locals.var_tmf2_dn7 = assign19180_e14136_d_n7;
        locals.var_tmf2_dn8 = assign19180_e14136_d_n8;
        locals.var_tmf2_dn9 = assign19180_e14136_d_n9;
        locals.var_tmf2_dn10 = assign19180_e14136_d_n10;
        locals.var_tmf2_dn13 = assign19180_e14136_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign19190_e14149, assign19190_e14149_d_n0, assign19190_e14149_d_n2, assign19190_e14149_d_n4, assign19190_e14149_d_n5, assign19190_e14149_d_n6, assign19190_e14149_d_n7, assign19190_e14149_d_n8, assign19190_e14149_d_n9, assign19190_e14149_d_n10, assign19190_e14149_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign19190_e14144: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19190_e14146: f64 = (assign19190_e14144 + locals.var_tmf2);
        let assign19190_e14147: f64 = (assign19190_e14146).sqrt();
        (assign19190_e14147, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19190_e14147)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19190_e14147)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign19190_e14147)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign19190_e14147)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign19190_e14147)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign19190_e14147)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign19190_e14147)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign19190_e14147)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign19190_e14147)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign19190_e14147)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign19190_e14149;
        locals.var_tmf2_dn0 = assign19190_e14149_d_n0;
        locals.var_tmf2_dn2 = assign19190_e14149_d_n2;
        locals.var_tmf2_dn4 = assign19190_e14149_d_n4;
        locals.var_tmf2_dn5 = assign19190_e14149_d_n5;
        locals.var_tmf2_dn6 = assign19190_e14149_d_n6;
        locals.var_tmf2_dn7 = assign19190_e14149_d_n7;
        locals.var_tmf2_dn8 = assign19190_e14149_d_n8;
        locals.var_tmf2_dn9 = assign19190_e14149_d_n9;
        locals.var_tmf2_dn10 = assign19190_e14149_d_n10;
        locals.var_tmf2_dn13 = assign19190_e14149_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign19200_e14163, assign19200_e14163_d_n0, assign19200_e14163_d_n2, assign19200_e14163_d_n4, assign19200_e14163_d_n5, assign19200_e14163_d_n6, assign19200_e14163_d_n7, assign19200_e14163_d_n8, assign19200_e14163_d_n9, assign19200_e14163_d_n10, assign19200_e14163_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign19200_e14159: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign19200_e14160: f64 = (1.0 + assign19200_e14159);
        let assign19200_e14161: f64 = (0.5 * assign19200_e14160);
        (assign19200_e14161, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign19200_e14163;
        locals.var_t6_dn0 = assign19200_e14163_d_n0;
        locals.var_t6_dn2 = assign19200_e14163_d_n2;
        locals.var_t6_dn4 = assign19200_e14163_d_n4;
        locals.var_t6_dn5 = assign19200_e14163_d_n5;
        locals.var_t6_dn6 = assign19200_e14163_d_n6;
        locals.var_t6_dn7 = assign19200_e14163_d_n7;
        locals.var_t6_dn8 = assign19200_e14163_d_n8;
        locals.var_t6_dn9 = assign19200_e14163_d_n9;
        locals.var_t6_dn10 = assign19200_e14163_d_n10;
        locals.var_t6_dn13 = assign19200_e14163_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign19210_e14181, assign19210_e14181_d_n0, assign19210_e14181_d_n2, assign19210_e14181_d_n4, assign19210_e14181_d_n5, assign19210_e14181_d_n6, assign19210_e14181_d_n7, assign19210_e14181_d_n8, assign19210_e14181_d_n9, assign19210_e14181_d_n10, assign19210_e14181_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign19210_e14172: f64 = (p.p98 + 1.0);
        let assign19210_e14173: f64 = (locals.var_t4 * assign19210_e14172);
        let assign19210_e14177: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19210_e14178: f64 = (0.5 * assign19210_e14177);
        let assign19210_e14179: f64 = (assign19210_e14173 - assign19210_e14178);
        (assign19210_e14179, ((locals.var_t4_dn0 * assign19210_e14172) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((locals.var_t4_dn2 * assign19210_e14172) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((locals.var_t4_dn4 * assign19210_e14172) - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), ((locals.var_t4_dn5 * assign19210_e14172) - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), ((locals.var_t4_dn6 * assign19210_e14172) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((locals.var_t4_dn7 * assign19210_e14172) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((locals.var_t4_dn8 * assign19210_e14172) - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), ((locals.var_t4_dn9 * assign19210_e14172) - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), ((locals.var_t4_dn10 * assign19210_e14172) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((locals.var_t4_dn13 * assign19210_e14172) - (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn13,)
    }
};
        locals.var_t7 = assign19210_e14181;
        locals.var_t7_dn0 = assign19210_e14181_d_n0;
        locals.var_t7_dn2 = assign19210_e14181_d_n2;
        locals.var_t7_dn4 = assign19210_e14181_d_n4;
        locals.var_t7_dn5 = assign19210_e14181_d_n5;
        locals.var_t7_dn6 = assign19210_e14181_d_n6;
        locals.var_t7_dn7 = assign19210_e14181_d_n7;
        locals.var_t7_dn8 = assign19210_e14181_d_n8;
        locals.var_t7_dn9 = assign19210_e14181_d_n9;
        locals.var_t7_dn10 = assign19210_e14181_d_n10;
        locals.var_t7_dn13 = assign19210_e14181_d_n13;
        locals.var_t7_rv = 0.0;

        let (assign19220_e14197, assign19220_e14197_d_n0, assign19220_e14197_d_n2, assign19220_e14197_d_n4, assign19220_e14197_d_n5, assign19220_e14197_d_n6, assign19220_e14197_d_n7, assign19220_e14197_d_n8, assign19220_e14197_d_n9, assign19220_e14197_d_n10, assign19220_e14197_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign19220_e14190: f64 = (locals.var_t1 * locals.var_t4);
        let assign19220_e14191: f64 = (locals.var_t7 + assign19220_e14190);
        let assign19220_e14193: f64 = assign19220_e14191;
        let assign19220_e14195: f64 = (assign19220_e14193 - 5e-5);
        (assign19220_e14195, (locals.var_t7_dn0 + ((locals.var_t1_dn0 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn0))), (locals.var_t7_dn2 + ((locals.var_t1_dn2 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn2))), (locals.var_t7_dn4 + ((locals.var_t1_dn4 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn4))), (locals.var_t7_dn5 + ((locals.var_t1_dn5 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn5))), (locals.var_t7_dn6 + ((locals.var_t1_dn6 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn6))), (locals.var_t7_dn7 + ((locals.var_t1_dn7 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn7))), (locals.var_t7_dn8 + ((locals.var_t1_dn8 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn8))), (locals.var_t7_dn9 + ((locals.var_t1_dn9 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn9))), (locals.var_t7_dn10 + ((locals.var_t1_dn10 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn10))), (locals.var_t7_dn13 + ((locals.var_t1_dn13 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn13))),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign19220_e14197;
        locals.var_tmf1_dn0 = assign19220_e14197_d_n0;
        locals.var_tmf1_dn2 = assign19220_e14197_d_n2;
        locals.var_tmf1_dn4 = assign19220_e14197_d_n4;
        locals.var_tmf1_dn5 = assign19220_e14197_d_n5;
        locals.var_tmf1_dn6 = assign19220_e14197_d_n6;
        locals.var_tmf1_dn7 = assign19220_e14197_d_n7;
        locals.var_tmf1_dn8 = assign19220_e14197_d_n8;
        locals.var_tmf1_dn9 = assign19220_e14197_d_n9;
        locals.var_tmf1_dn10 = assign19220_e14197_d_n10;
        locals.var_tmf1_dn13 = assign19220_e14197_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign19230_e14209, assign19230_e14209_d_n0, assign19230_e14209_d_n2, assign19230_e14209_d_n4, assign19230_e14209_d_n5, assign19230_e14209_d_n6, assign19230_e14209_d_n7, assign19230_e14209_d_n8, assign19230_e14209_d_n9, assign19230_e14209_d_n10, assign19230_e14209_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign19230_e14209;
        locals.var_tmf2_dn0 = assign19230_e14209_d_n0;
        locals.var_tmf2_dn2 = assign19230_e14209_d_n2;
        locals.var_tmf2_dn4 = assign19230_e14209_d_n4;
        locals.var_tmf2_dn5 = assign19230_e14209_d_n5;
        locals.var_tmf2_dn6 = assign19230_e14209_d_n6;
        locals.var_tmf2_dn7 = assign19230_e14209_d_n7;
        locals.var_tmf2_dn8 = assign19230_e14209_d_n8;
        locals.var_tmf2_dn9 = assign19230_e14209_d_n9;
        locals.var_tmf2_dn10 = assign19230_e14209_d_n10;
        locals.var_tmf2_dn13 = assign19230_e14209_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign19240_e14223, assign19240_e14223_d_n0, assign19240_e14223_d_n2, assign19240_e14223_d_n4, assign19240_e14223_d_n5, assign19240_e14223_d_n6, assign19240_e14223_d_n7, assign19240_e14223_d_n8, assign19240_e14223_d_n9, assign19240_e14223_d_n10, assign19240_e14223_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let (assign19240_e14221, assign19240_e14221_d_n0, assign19240_e14221_d_n2, assign19240_e14221_d_n4, assign19240_e14221_d_n5, assign19240_e14221_d_n6, assign19240_e14221_d_n7, assign19240_e14221_d_n8, assign19240_e14221_d_n9, assign19240_e14221_d_n10, assign19240_e14221_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign19240_e14220: f64 = (-locals.var_tmf2);
                (assign19240_e14220, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign19240_e14221, assign19240_e14221_d_n0, assign19240_e14221_d_n2, assign19240_e14221_d_n4, assign19240_e14221_d_n5, assign19240_e14221_d_n6, assign19240_e14221_d_n7, assign19240_e14221_d_n8, assign19240_e14221_d_n9, assign19240_e14221_d_n10, assign19240_e14221_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign19240_e14223;
        locals.var_tmf2_dn0 = assign19240_e14223_d_n0;
        locals.var_tmf2_dn2 = assign19240_e14223_d_n2;
        locals.var_tmf2_dn4 = assign19240_e14223_d_n4;
        locals.var_tmf2_dn5 = assign19240_e14223_d_n5;
        locals.var_tmf2_dn6 = assign19240_e14223_d_n6;
        locals.var_tmf2_dn7 = assign19240_e14223_d_n7;
        locals.var_tmf2_dn8 = assign19240_e14223_d_n8;
        locals.var_tmf2_dn9 = assign19240_e14223_d_n9;
        locals.var_tmf2_dn10 = assign19240_e14223_d_n10;
        locals.var_tmf2_dn13 = assign19240_e14223_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign19250_e14236, assign19250_e14236_d_n0, assign19250_e14236_d_n2, assign19250_e14236_d_n4, assign19250_e14236_d_n5, assign19250_e14236_d_n6, assign19250_e14236_d_n7, assign19250_e14236_d_n8, assign19250_e14236_d_n9, assign19250_e14236_d_n10, assign19250_e14236_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign19250_e14231: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19250_e14233: f64 = (assign19250_e14231 + locals.var_tmf2);
        let assign19250_e14234: f64 = (assign19250_e14233).sqrt();
        (assign19250_e14234, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19250_e14234)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19250_e14234)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign19250_e14234)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign19250_e14234)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign19250_e14234)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign19250_e14234)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign19250_e14234)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign19250_e14234)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign19250_e14234)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign19250_e14234)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign19250_e14236;
        locals.var_tmf2_dn0 = assign19250_e14236_d_n0;
        locals.var_tmf2_dn2 = assign19250_e14236_d_n2;
        locals.var_tmf2_dn4 = assign19250_e14236_d_n4;
        locals.var_tmf2_dn5 = assign19250_e14236_d_n5;
        locals.var_tmf2_dn6 = assign19250_e14236_d_n6;
        locals.var_tmf2_dn7 = assign19250_e14236_d_n7;
        locals.var_tmf2_dn8 = assign19250_e14236_d_n8;
        locals.var_tmf2_dn9 = assign19250_e14236_d_n9;
        locals.var_tmf2_dn10 = assign19250_e14236_d_n10;
        locals.var_tmf2_dn13 = assign19250_e14236_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign19260_e14250, assign19260_e14250_d_n0, assign19260_e14250_d_n2, assign19260_e14250_d_n4, assign19260_e14250_d_n5, assign19260_e14250_d_n6, assign19260_e14250_d_n7, assign19260_e14250_d_n8, assign19260_e14250_d_n9, assign19260_e14250_d_n10, assign19260_e14250_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign19260_e14246: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign19260_e14247: f64 = (1.0 + assign19260_e14246);
        let assign19260_e14248: f64 = (0.5 * assign19260_e14247);
        (assign19260_e14248, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn13,)
    }
};
        locals.var_t6 = assign19260_e14250;
        locals.var_t6_dn0 = assign19260_e14250_d_n0;
        locals.var_t6_dn2 = assign19260_e14250_d_n2;
        locals.var_t6_dn4 = assign19260_e14250_d_n4;
        locals.var_t6_dn5 = assign19260_e14250_d_n5;
        locals.var_t6_dn6 = assign19260_e14250_d_n6;
        locals.var_t6_dn7 = assign19260_e14250_d_n7;
        locals.var_t6_dn8 = assign19260_e14250_d_n8;
        locals.var_t6_dn9 = assign19260_e14250_d_n9;
        locals.var_t6_dn10 = assign19260_e14250_d_n10;
        locals.var_t6_dn13 = assign19260_e14250_d_n13;
        locals.var_t6_rv = 0.0;

        let (assign19270_e14264, assign19270_e14264_d_n0, assign19270_e14264_d_n2, assign19270_e14264_d_n4, assign19270_e14264_d_n5, assign19270_e14264_d_n6, assign19270_e14264_d_n7, assign19270_e14264_d_n8, assign19270_e14264_d_n9, assign19270_e14264_d_n10, assign19270_e14264_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign19270_e14260: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19270_e14261: f64 = (0.5 * assign19270_e14260);
        let assign19270_e14262: f64 = assign19270_e14261;
        (assign19270_e14262, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign19270_e14264;
        locals.var_t2_dn0 = assign19270_e14264_d_n0;
        locals.var_t2_dn2 = assign19270_e14264_d_n2;
        locals.var_t2_dn4 = assign19270_e14264_d_n4;
        locals.var_t2_dn5 = assign19270_e14264_d_n5;
        locals.var_t2_dn6 = assign19270_e14264_d_n6;
        locals.var_t2_dn7 = assign19270_e14264_d_n7;
        locals.var_t2_dn8 = assign19270_e14264_d_n8;
        locals.var_t2_dn9 = assign19270_e14264_d_n9;
        locals.var_t2_dn10 = assign19270_e14264_d_n10;
        locals.var_t2_dn13 = assign19270_e14264_d_n13;
        locals.var_t2_rv = 0.0;

        let assign19280_e14271: f64 = if ((p.p39 == 0.0) || (p.p39 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard386 = assign19280_e14271;
        locals.var_guard386_rv = 0.0;

        let (assign19290_e14291, assign19290_e14291_d_n0, assign19290_e14291_d_n2, assign19290_e14291_d_n4, assign19290_e14291_d_n5, assign19290_e14291_d_n6, assign19290_e14291_d_n7, assign19290_e14291_d_n8, assign19290_e14291_d_n9, assign19290_e14291_d_n10, assign19290_e14291_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard386 != 0.0)) {
        let assign19290_e14282: f64 = (locals.var_mks_rdvdtemp1 * locals.var_tdiff0);
        let assign19290_e14283: f64 = (locals.var_uc_rdvd + assign19290_e14282);
        let assign19290_e14286: f64 = (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2);
        let assign19290_e14287: f64 = (assign19290_e14283 + assign19290_e14286);
        let assign19290_e14289: f64 = (assign19290_e14287 * locals.var_t2);
        (assign19290_e14289, ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn0) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn0)) * locals.var_t2) + (assign19290_e14287 * locals.var_t2_dn0)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn2) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn2)) * locals.var_t2) + (assign19290_e14287 * locals.var_t2_dn2)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn4) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn4)) * locals.var_t2) + (assign19290_e14287 * locals.var_t2_dn4)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn5) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn5)) * locals.var_t2) + (assign19290_e14287 * locals.var_t2_dn5)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn6) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn6)) * locals.var_t2) + (assign19290_e14287 * locals.var_t2_dn6)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn7) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn7)) * locals.var_t2) + (assign19290_e14287 * locals.var_t2_dn7)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn8) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn8)) * locals.var_t2) + (assign19290_e14287 * locals.var_t2_dn8)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn9) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn9)) * locals.var_t2) + (assign19290_e14287 * locals.var_t2_dn9)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn10) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn10)) * locals.var_t2) + (assign19290_e14287 * locals.var_t2_dn10)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn13) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn13)) * locals.var_t2) + (assign19290_e14287 * locals.var_t2_dn13)),)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn13,)
    }
};
        locals.var_rsvde = assign19290_e14291;
        locals.var_rsvde_dn0 = assign19290_e14291_d_n0;
        locals.var_rsvde_dn2 = assign19290_e14291_d_n2;
        locals.var_rsvde_dn4 = assign19290_e14291_d_n4;
        locals.var_rsvde_dn5 = assign19290_e14291_d_n5;
        locals.var_rsvde_dn6 = assign19290_e14291_d_n6;
        locals.var_rsvde_dn7 = assign19290_e14291_d_n7;
        locals.var_rsvde_dn8 = assign19290_e14291_d_n8;
        locals.var_rsvde_dn9 = assign19290_e14291_d_n9;
        locals.var_rsvde_dn10 = assign19290_e14291_d_n10;
        locals.var_rsvde_dn13 = assign19290_e14291_d_n13;
        locals.var_rsvde_rv = 0.0;

        let (assign19300_e14309, assign19300_e14309_d_n0, assign19300_e14309_d_n2, assign19300_e14309_d_n4, assign19300_e14309_d_n5, assign19300_e14309_d_n6, assign19300_e14309_d_n7, assign19300_e14309_d_n8, assign19300_e14309_d_n9, assign19300_e14309_d_n10, assign19300_e14309_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard386 != 0.0)) {
        let assign19300_e14302: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19300_e14303: f64 = (locals.var_rsvde - assign19300_e14302);
        let assign19300_e14306: f64 = (0.01 * locals.var_uc_rdvd);
        let assign19300_e14307: f64 = (assign19300_e14303 - assign19300_e14306);
        (assign19300_e14307, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign19300_e14309;
        locals.var_tmf1_dn0 = assign19300_e14309_d_n0;
        locals.var_tmf1_dn2 = assign19300_e14309_d_n2;
        locals.var_tmf1_dn4 = assign19300_e14309_d_n4;
        locals.var_tmf1_dn5 = assign19300_e14309_d_n5;
        locals.var_tmf1_dn6 = assign19300_e14309_d_n6;
        locals.var_tmf1_dn7 = assign19300_e14309_d_n7;
        locals.var_tmf1_dn8 = assign19300_e14309_d_n8;
        locals.var_tmf1_dn9 = assign19300_e14309_d_n9;
        locals.var_tmf1_dn10 = assign19300_e14309_d_n10;
        locals.var_tmf1_dn13 = assign19300_e14309_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign19310_e14327, assign19310_e14327_d_n0, assign19310_e14327_d_n2, assign19310_e14327_d_n4, assign19310_e14327_d_n5, assign19310_e14327_d_n6, assign19310_e14327_d_n7, assign19310_e14327_d_n8, assign19310_e14327_d_n9, assign19310_e14327_d_n10, assign19310_e14327_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard386 != 0.0)) {
        let assign19310_e14320: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19310_e14321: f64 = (4.0 * assign19310_e14320);
        let assign19310_e14324: f64 = (0.01 * locals.var_uc_rdvd);
        let assign19310_e14325: f64 = (assign19310_e14321 * assign19310_e14324);
        (assign19310_e14325, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign19310_e14327;
        locals.var_tmf2_dn0 = assign19310_e14327_d_n0;
        locals.var_tmf2_dn2 = assign19310_e14327_d_n2;
        locals.var_tmf2_dn4 = assign19310_e14327_d_n4;
        locals.var_tmf2_dn5 = assign19310_e14327_d_n5;
        locals.var_tmf2_dn6 = assign19310_e14327_d_n6;
        locals.var_tmf2_dn7 = assign19310_e14327_d_n7;
        locals.var_tmf2_dn8 = assign19310_e14327_d_n8;
        locals.var_tmf2_dn9 = assign19310_e14327_d_n9;
        locals.var_tmf2_dn10 = assign19310_e14327_d_n10;
        locals.var_tmf2_dn13 = assign19310_e14327_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign19320_e14343, assign19320_e14343_d_n0, assign19320_e14343_d_n2, assign19320_e14343_d_n4, assign19320_e14343_d_n5, assign19320_e14343_d_n6, assign19320_e14343_d_n7, assign19320_e14343_d_n8, assign19320_e14343_d_n9, assign19320_e14343_d_n10, assign19320_e14343_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard386 != 0.0)) {
        let (assign19320_e14341, assign19320_e14341_d_n0, assign19320_e14341_d_n2, assign19320_e14341_d_n4, assign19320_e14341_d_n5, assign19320_e14341_d_n6, assign19320_e14341_d_n7, assign19320_e14341_d_n8, assign19320_e14341_d_n9, assign19320_e14341_d_n10, assign19320_e14341_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign19320_e14340: f64 = (-locals.var_tmf2);
                (assign19320_e14340, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign19320_e14341, assign19320_e14341_d_n0, assign19320_e14341_d_n2, assign19320_e14341_d_n4, assign19320_e14341_d_n5, assign19320_e14341_d_n6, assign19320_e14341_d_n7, assign19320_e14341_d_n8, assign19320_e14341_d_n9, assign19320_e14341_d_n10, assign19320_e14341_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign19320_e14343;
        locals.var_tmf2_dn0 = assign19320_e14343_d_n0;
        locals.var_tmf2_dn2 = assign19320_e14343_d_n2;
        locals.var_tmf2_dn4 = assign19320_e14343_d_n4;
        locals.var_tmf2_dn5 = assign19320_e14343_d_n5;
        locals.var_tmf2_dn6 = assign19320_e14343_d_n6;
        locals.var_tmf2_dn7 = assign19320_e14343_d_n7;
        locals.var_tmf2_dn8 = assign19320_e14343_d_n8;
        locals.var_tmf2_dn9 = assign19320_e14343_d_n9;
        locals.var_tmf2_dn10 = assign19320_e14343_d_n10;
        locals.var_tmf2_dn13 = assign19320_e14343_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign19330_e14358, assign19330_e14358_d_n0, assign19330_e14358_d_n2, assign19330_e14358_d_n4, assign19330_e14358_d_n5, assign19330_e14358_d_n6, assign19330_e14358_d_n7, assign19330_e14358_d_n8, assign19330_e14358_d_n9, assign19330_e14358_d_n10, assign19330_e14358_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard386 != 0.0)) {
        let assign19330_e14353: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19330_e14355: f64 = (assign19330_e14353 + locals.var_tmf2);
        let assign19330_e14356: f64 = (assign19330_e14355).sqrt();
        (assign19330_e14356, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19330_e14356)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19330_e14356)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign19330_e14356)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign19330_e14356)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign19330_e14356)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign19330_e14356)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign19330_e14356)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign19330_e14356)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign19330_e14356)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign19330_e14356)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign19330_e14358;
        locals.var_tmf2_dn0 = assign19330_e14358_d_n0;
        locals.var_tmf2_dn2 = assign19330_e14358_d_n2;
        locals.var_tmf2_dn4 = assign19330_e14358_d_n4;
        locals.var_tmf2_dn5 = assign19330_e14358_d_n5;
        locals.var_tmf2_dn6 = assign19330_e14358_d_n6;
        locals.var_tmf2_dn7 = assign19330_e14358_d_n7;
        locals.var_tmf2_dn8 = assign19330_e14358_d_n8;
        locals.var_tmf2_dn9 = assign19330_e14358_d_n9;
        locals.var_tmf2_dn10 = assign19330_e14358_d_n10;
        locals.var_tmf2_dn13 = assign19330_e14358_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign19340_e14374, assign19340_e14374_d_n0, assign19340_e14374_d_n2, assign19340_e14374_d_n4, assign19340_e14374_d_n5, assign19340_e14374_d_n6, assign19340_e14374_d_n7, assign19340_e14374_d_n8, assign19340_e14374_d_n9, assign19340_e14374_d_n10, assign19340_e14374_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard386 != 0.0)) {
        let assign19340_e14370: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign19340_e14371: f64 = (1.0 + assign19340_e14370);
        let assign19340_e14372: f64 = (0.5 * assign19340_e14371);
        (assign19340_e14372, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign19340_e14374;
        locals.var_t0_dn0 = assign19340_e14374_d_n0;
        locals.var_t0_dn2 = assign19340_e14374_d_n2;
        locals.var_t0_dn4 = assign19340_e14374_d_n4;
        locals.var_t0_dn5 = assign19340_e14374_d_n5;
        locals.var_t0_dn6 = assign19340_e14374_d_n6;
        locals.var_t0_dn7 = assign19340_e14374_d_n7;
        locals.var_t0_dn8 = assign19340_e14374_d_n8;
        locals.var_t0_dn9 = assign19340_e14374_d_n9;
        locals.var_t0_dn10 = assign19340_e14374_d_n10;
        locals.var_t0_dn13 = assign19340_e14374_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign19350_e14392, assign19350_e14392_d_n0, assign19350_e14392_d_n2, assign19350_e14392_d_n4, assign19350_e14392_d_n5, assign19350_e14392_d_n6, assign19350_e14392_d_n7, assign19350_e14392_d_n8, assign19350_e14392_d_n9, assign19350_e14392_d_n10, assign19350_e14392_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard386 != 0.0)) {
        let assign19350_e14384: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19350_e14388: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19350_e14389: f64 = (0.5 * assign19350_e14388);
        let assign19350_e14390: f64 = (assign19350_e14384 + assign19350_e14389);
        (assign19350_e14390, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn13,)
    }
};
        locals.var_rsvde = assign19350_e14392;
        locals.var_rsvde_dn0 = assign19350_e14392_d_n0;
        locals.var_rsvde_dn2 = assign19350_e14392_d_n2;
        locals.var_rsvde_dn4 = assign19350_e14392_d_n4;
        locals.var_rsvde_dn5 = assign19350_e14392_d_n5;
        locals.var_rsvde_dn6 = assign19350_e14392_d_n6;
        locals.var_rsvde_dn7 = assign19350_e14392_d_n7;
        locals.var_rsvde_dn8 = assign19350_e14392_d_n8;
        locals.var_rsvde_dn9 = assign19350_e14392_d_n9;
        locals.var_rsvde_dn10 = assign19350_e14392_d_n10;
        locals.var_rsvde_dn13 = assign19350_e14392_d_n13;
        locals.var_rsvde_rv = 0.0;

        let (assign19360_e14413, assign19360_e14413_d_n0, assign19360_e14413_d_n2, assign19360_e14413_d_n4, assign19360_e14413_d_n5, assign19360_e14413_d_n6, assign19360_e14413_d_n7, assign19360_e14413_d_n8, assign19360_e14413_d_n9, assign19360_e14413_d_n10, assign19360_e14413_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard386 == 0.0)) {
        let assign19360_e14404: f64 = (locals.var_mks_rdvdtemp1 * locals.var_tdiff);
        let assign19360_e14405: f64 = (locals.var_uc_rdvd + assign19360_e14404);
        let assign19360_e14408: f64 = (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2);
        let assign19360_e14409: f64 = (assign19360_e14405 + assign19360_e14408);
        let assign19360_e14411: f64 = (assign19360_e14409 * locals.var_t2);
        (assign19360_e14411, ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn0) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn0)) * locals.var_t2) + (assign19360_e14409 * locals.var_t2_dn0)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn2) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn2)) * locals.var_t2) + (assign19360_e14409 * locals.var_t2_dn2)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn4) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn4)) * locals.var_t2) + (assign19360_e14409 * locals.var_t2_dn4)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn5) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn5)) * locals.var_t2) + (assign19360_e14409 * locals.var_t2_dn5)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn6) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn6)) * locals.var_t2) + (assign19360_e14409 * locals.var_t2_dn6)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn7) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn7)) * locals.var_t2) + (assign19360_e14409 * locals.var_t2_dn7)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn8) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn8)) * locals.var_t2) + (assign19360_e14409 * locals.var_t2_dn8)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn9) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn9)) * locals.var_t2) + (assign19360_e14409 * locals.var_t2_dn9)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn10) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn10)) * locals.var_t2) + (assign19360_e14409 * locals.var_t2_dn10)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn13) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn13)) * locals.var_t2) + (assign19360_e14409 * locals.var_t2_dn13)),)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn13,)
    }
};
        locals.var_rsvde = assign19360_e14413;
        locals.var_rsvde_dn0 = assign19360_e14413_d_n0;
        locals.var_rsvde_dn2 = assign19360_e14413_d_n2;
        locals.var_rsvde_dn4 = assign19360_e14413_d_n4;
        locals.var_rsvde_dn5 = assign19360_e14413_d_n5;
        locals.var_rsvde_dn6 = assign19360_e14413_d_n6;
        locals.var_rsvde_dn7 = assign19360_e14413_d_n7;
        locals.var_rsvde_dn8 = assign19360_e14413_d_n8;
        locals.var_rsvde_dn9 = assign19360_e14413_d_n9;
        locals.var_rsvde_dn10 = assign19360_e14413_d_n10;
        locals.var_rsvde_dn13 = assign19360_e14413_d_n13;
        locals.var_rsvde_rv = 0.0;

        let (assign19370_e14432, assign19370_e14432_d_n0, assign19370_e14432_d_n2, assign19370_e14432_d_n4, assign19370_e14432_d_n5, assign19370_e14432_d_n6, assign19370_e14432_d_n7, assign19370_e14432_d_n8, assign19370_e14432_d_n9, assign19370_e14432_d_n10, assign19370_e14432_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard386 == 0.0)) {
        let assign19370_e14425: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19370_e14426: f64 = (locals.var_rsvde - assign19370_e14425);
        let assign19370_e14429: f64 = (0.01 * locals.var_uc_rdvd);
        let assign19370_e14430: f64 = (assign19370_e14426 - assign19370_e14429);
        (assign19370_e14430, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn13,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign19370_e14432;
        locals.var_tmf1_dn0 = assign19370_e14432_d_n0;
        locals.var_tmf1_dn2 = assign19370_e14432_d_n2;
        locals.var_tmf1_dn4 = assign19370_e14432_d_n4;
        locals.var_tmf1_dn5 = assign19370_e14432_d_n5;
        locals.var_tmf1_dn6 = assign19370_e14432_d_n6;
        locals.var_tmf1_dn7 = assign19370_e14432_d_n7;
        locals.var_tmf1_dn8 = assign19370_e14432_d_n8;
        locals.var_tmf1_dn9 = assign19370_e14432_d_n9;
        locals.var_tmf1_dn10 = assign19370_e14432_d_n10;
        locals.var_tmf1_dn13 = assign19370_e14432_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign19380_e14451, assign19380_e14451_d_n0, assign19380_e14451_d_n2, assign19380_e14451_d_n4, assign19380_e14451_d_n5, assign19380_e14451_d_n6, assign19380_e14451_d_n7, assign19380_e14451_d_n8, assign19380_e14451_d_n9, assign19380_e14451_d_n10, assign19380_e14451_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard386 == 0.0)) {
        let assign19380_e14444: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19380_e14445: f64 = (4.0 * assign19380_e14444);
        let assign19380_e14448: f64 = (0.01 * locals.var_uc_rdvd);
        let assign19380_e14449: f64 = (assign19380_e14445 * assign19380_e14448);
        (assign19380_e14449, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign19380_e14451;
        locals.var_tmf2_dn0 = assign19380_e14451_d_n0;
        locals.var_tmf2_dn2 = assign19380_e14451_d_n2;
        locals.var_tmf2_dn4 = assign19380_e14451_d_n4;
        locals.var_tmf2_dn5 = assign19380_e14451_d_n5;
        locals.var_tmf2_dn6 = assign19380_e14451_d_n6;
        locals.var_tmf2_dn7 = assign19380_e14451_d_n7;
        locals.var_tmf2_dn8 = assign19380_e14451_d_n8;
        locals.var_tmf2_dn9 = assign19380_e14451_d_n9;
        locals.var_tmf2_dn10 = assign19380_e14451_d_n10;
        locals.var_tmf2_dn13 = assign19380_e14451_d_n13;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_47(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let (assign19390_e14468, assign19390_e14468_d_n0, assign19390_e14468_d_n2, assign19390_e14468_d_n4, assign19390_e14468_d_n5, assign19390_e14468_d_n6, assign19390_e14468_d_n7, assign19390_e14468_d_n8, assign19390_e14468_d_n9, assign19390_e14468_d_n10, assign19390_e14468_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard386 == 0.0)) {
        let (assign19390_e14466, assign19390_e14466_d_n0, assign19390_e14466_d_n2, assign19390_e14466_d_n4, assign19390_e14466_d_n5, assign19390_e14466_d_n6, assign19390_e14466_d_n7, assign19390_e14466_d_n8, assign19390_e14466_d_n9, assign19390_e14466_d_n10, assign19390_e14466_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign19390_e14465: f64 = (-locals.var_tmf2);
                (assign19390_e14465, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign19390_e14466, assign19390_e14466_d_n0, assign19390_e14466_d_n2, assign19390_e14466_d_n4, assign19390_e14466_d_n5, assign19390_e14466_d_n6, assign19390_e14466_d_n7, assign19390_e14466_d_n8, assign19390_e14466_d_n9, assign19390_e14466_d_n10, assign19390_e14466_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign19390_e14468;
        locals.var_tmf2_dn0 = assign19390_e14468_d_n0;
        locals.var_tmf2_dn2 = assign19390_e14468_d_n2;
        locals.var_tmf2_dn4 = assign19390_e14468_d_n4;
        locals.var_tmf2_dn5 = assign19390_e14468_d_n5;
        locals.var_tmf2_dn6 = assign19390_e14468_d_n6;
        locals.var_tmf2_dn7 = assign19390_e14468_d_n7;
        locals.var_tmf2_dn8 = assign19390_e14468_d_n8;
        locals.var_tmf2_dn9 = assign19390_e14468_d_n9;
        locals.var_tmf2_dn10 = assign19390_e14468_d_n10;
        locals.var_tmf2_dn13 = assign19390_e14468_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign19400_e14484, assign19400_e14484_d_n0, assign19400_e14484_d_n2, assign19400_e14484_d_n4, assign19400_e14484_d_n5, assign19400_e14484_d_n6, assign19400_e14484_d_n7, assign19400_e14484_d_n8, assign19400_e14484_d_n9, assign19400_e14484_d_n10, assign19400_e14484_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard386 == 0.0)) {
        let assign19400_e14479: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19400_e14481: f64 = (assign19400_e14479 + locals.var_tmf2);
        let assign19400_e14482: f64 = (assign19400_e14481).sqrt();
        (assign19400_e14482, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19400_e14482)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19400_e14482)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign19400_e14482)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign19400_e14482)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign19400_e14482)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign19400_e14482)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign19400_e14482)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign19400_e14482)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign19400_e14482)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign19400_e14482)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign19400_e14484;
        locals.var_tmf2_dn0 = assign19400_e14484_d_n0;
        locals.var_tmf2_dn2 = assign19400_e14484_d_n2;
        locals.var_tmf2_dn4 = assign19400_e14484_d_n4;
        locals.var_tmf2_dn5 = assign19400_e14484_d_n5;
        locals.var_tmf2_dn6 = assign19400_e14484_d_n6;
        locals.var_tmf2_dn7 = assign19400_e14484_d_n7;
        locals.var_tmf2_dn8 = assign19400_e14484_d_n8;
        locals.var_tmf2_dn9 = assign19400_e14484_d_n9;
        locals.var_tmf2_dn10 = assign19400_e14484_d_n10;
        locals.var_tmf2_dn13 = assign19400_e14484_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign19410_e14501, assign19410_e14501_d_n0, assign19410_e14501_d_n2, assign19410_e14501_d_n4, assign19410_e14501_d_n5, assign19410_e14501_d_n6, assign19410_e14501_d_n7, assign19410_e14501_d_n8, assign19410_e14501_d_n9, assign19410_e14501_d_n10, assign19410_e14501_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard386 == 0.0)) {
        let assign19410_e14497: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign19410_e14498: f64 = (1.0 + assign19410_e14497);
        let assign19410_e14499: f64 = (0.5 * assign19410_e14498);
        (assign19410_e14499, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign19410_e14501;
        locals.var_t0_dn0 = assign19410_e14501_d_n0;
        locals.var_t0_dn2 = assign19410_e14501_d_n2;
        locals.var_t0_dn4 = assign19410_e14501_d_n4;
        locals.var_t0_dn5 = assign19410_e14501_d_n5;
        locals.var_t0_dn6 = assign19410_e14501_d_n6;
        locals.var_t0_dn7 = assign19410_e14501_d_n7;
        locals.var_t0_dn8 = assign19410_e14501_d_n8;
        locals.var_t0_dn9 = assign19410_e14501_d_n9;
        locals.var_t0_dn10 = assign19410_e14501_d_n10;
        locals.var_t0_dn13 = assign19410_e14501_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign19420_e14520, assign19420_e14520_d_n0, assign19420_e14520_d_n2, assign19420_e14520_d_n4, assign19420_e14520_d_n5, assign19420_e14520_d_n6, assign19420_e14520_d_n7, assign19420_e14520_d_n8, assign19420_e14520_d_n9, assign19420_e14520_d_n10, assign19420_e14520_d_n13,) = {
    if ((((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard386 == 0.0)) {
        let assign19420_e14512: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19420_e14516: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19420_e14517: f64 = (0.5 * assign19420_e14516);
        let assign19420_e14518: f64 = (assign19420_e14512 + assign19420_e14517);
        (assign19420_e14518, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn13,)
    }
};
        locals.var_rsvde = assign19420_e14520;
        locals.var_rsvde_dn0 = assign19420_e14520_d_n0;
        locals.var_rsvde_dn2 = assign19420_e14520_d_n2;
        locals.var_rsvde_dn4 = assign19420_e14520_d_n4;
        locals.var_rsvde_dn5 = assign19420_e14520_d_n5;
        locals.var_rsvde_dn6 = assign19420_e14520_d_n6;
        locals.var_rsvde_dn7 = assign19420_e14520_d_n7;
        locals.var_rsvde_dn8 = assign19420_e14520_d_n8;
        locals.var_rsvde_dn9 = assign19420_e14520_d_n9;
        locals.var_rsvde_dn10 = assign19420_e14520_d_n10;
        locals.var_rsvde_dn13 = assign19420_e14520_d_n13;
        locals.var_rsvde_rv = 0.0;

        let (assign19430_e14529, assign19430_e14529_d_n0, assign19430_e14529_d_n2, assign19430_e14529_d_n4, assign19430_e14529_d_n5, assign19430_e14529_d_n6, assign19430_e14529_d_n7, assign19430_e14529_d_n8, assign19430_e14529_d_n9, assign19430_e14529_d_n10, assign19430_e14529_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn13,)
    }
};
        locals.var_rdvde = assign19430_e14529;
        locals.var_rdvde_dn0 = assign19430_e14529_d_n0;
        locals.var_rdvde_dn2 = assign19430_e14529_d_n2;
        locals.var_rdvde_dn4 = assign19430_e14529_d_n4;
        locals.var_rdvde_dn5 = assign19430_e14529_d_n5;
        locals.var_rdvde_dn6 = assign19430_e14529_d_n6;
        locals.var_rdvde_dn7 = assign19430_e14529_d_n7;
        locals.var_rdvde_dn8 = assign19430_e14529_d_n8;
        locals.var_rdvde_dn9 = assign19430_e14529_d_n9;
        locals.var_rdvde_dn10 = assign19430_e14529_d_n10;
        locals.var_rdvde_dn13 = assign19430_e14529_d_n13;
        locals.var_rdvde_rv = 0.0;

        let (assign19440_e14538, assign19440_e14538_d_n0, assign19440_e14538_d_n2, assign19440_e14538_d_n4, assign19440_e14538_d_n5, assign19440_e14538_d_n6, assign19440_e14538_d_n7, assign19440_e14538_d_n8, assign19440_e14538_d_n9, assign19440_e14538_d_n10, assign19440_e14538_d_n13,) = {
    if (((locals.var_guard352 != 0.0) && (locals.var_guard378 != 0.0)) && (locals.var_guard383 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn13,)
    }
};
        locals.var_rsvde = assign19440_e14538;
        locals.var_rsvde_dn0 = assign19440_e14538_d_n0;
        locals.var_rsvde_dn2 = assign19440_e14538_d_n2;
        locals.var_rsvde_dn4 = assign19440_e14538_d_n4;
        locals.var_rsvde_dn5 = assign19440_e14538_d_n5;
        locals.var_rsvde_dn6 = assign19440_e14538_d_n6;
        locals.var_rsvde_dn7 = assign19440_e14538_d_n7;
        locals.var_rsvde_dn8 = assign19440_e14538_d_n8;
        locals.var_rsvde_dn9 = assign19440_e14538_d_n9;
        locals.var_rsvde_dn10 = assign19440_e14538_d_n10;
        locals.var_rsvde_dn13 = assign19440_e14538_d_n13;
        locals.var_rsvde_rv = 0.0;

        let (assign19450_e14545, assign19450_e14545_d_n0, assign19450_e14545_d_n2, assign19450_e14545_d_n4, assign19450_e14545_d_n5, assign19450_e14545_d_n6, assign19450_e14545_d_n7, assign19450_e14545_d_n8, assign19450_e14545_d_n9, assign19450_e14545_d_n10, assign19450_e14545_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign19450_e14542: f64 = (locals.var_beta_inv).sqrt();
        let assign19450_e14543: f64 = (locals.var_costi00 * assign19450_e14542);
        (assign19450_e14543, (locals.var_costi00 * (locals.var_beta_inv_dn0 / (2.0 * assign19450_e14542))), (locals.var_costi00 * (locals.var_beta_inv_dn2 / (2.0 * assign19450_e14542))), (locals.var_costi00 * (locals.var_beta_inv_dn4 / (2.0 * assign19450_e14542))), (locals.var_costi00 * (locals.var_beta_inv_dn5 / (2.0 * assign19450_e14542))), (locals.var_costi00 * (locals.var_beta_inv_dn6 / (2.0 * assign19450_e14542))), (locals.var_costi00 * (locals.var_beta_inv_dn7 / (2.0 * assign19450_e14542))), (locals.var_costi00 * (locals.var_beta_inv_dn8 / (2.0 * assign19450_e14542))), (locals.var_costi00 * (locals.var_beta_inv_dn9 / (2.0 * assign19450_e14542))), (locals.var_costi00 * (locals.var_beta_inv_dn10 / (2.0 * assign19450_e14542))), (locals.var_costi00 * (locals.var_beta_inv_dn13 / (2.0 * assign19450_e14542))),)
    } else {
        (locals.var_costi0, locals.var_costi0_dn0, locals.var_costi0_dn2, locals.var_costi0_dn4, locals.var_costi0_dn5, locals.var_costi0_dn6, locals.var_costi0_dn7, locals.var_costi0_dn8, locals.var_costi0_dn9, locals.var_costi0_dn10, locals.var_costi0_dn13,)
    }
};
        locals.var_costi0 = assign19450_e14545;
        locals.var_costi0_dn0 = assign19450_e14545_d_n0;
        locals.var_costi0_dn2 = assign19450_e14545_d_n2;
        locals.var_costi0_dn4 = assign19450_e14545_d_n4;
        locals.var_costi0_dn5 = assign19450_e14545_d_n5;
        locals.var_costi0_dn6 = assign19450_e14545_d_n6;
        locals.var_costi0_dn7 = assign19450_e14545_d_n7;
        locals.var_costi0_dn8 = assign19450_e14545_d_n8;
        locals.var_costi0_dn9 = assign19450_e14545_d_n9;
        locals.var_costi0_dn10 = assign19450_e14545_d_n10;
        locals.var_costi0_dn13 = assign19450_e14545_d_n13;
        locals.var_costi0_rv = 0.0;

        let (assign19460_e14551, assign19460_e14551_d_n0, assign19460_e14551_d_n2, assign19460_e14551_d_n4, assign19460_e14551_d_n5, assign19460_e14551_d_n6, assign19460_e14551_d_n7, assign19460_e14551_d_n8, assign19460_e14551_d_n9, assign19460_e14551_d_n10, assign19460_e14551_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign19460_e14549: f64 = (locals.var_costi0 * locals.var_costi0);
        (assign19460_e14549, ((locals.var_costi0_dn0 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn0)), ((locals.var_costi0_dn2 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn2)), ((locals.var_costi0_dn4 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn4)), ((locals.var_costi0_dn5 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn5)), ((locals.var_costi0_dn6 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn6)), ((locals.var_costi0_dn7 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn7)), ((locals.var_costi0_dn8 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn8)), ((locals.var_costi0_dn9 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn9)), ((locals.var_costi0_dn10 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn10)), ((locals.var_costi0_dn13 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn13)),)
    } else {
        (locals.var_costi0_p2, locals.var_costi0_p2_dn0, locals.var_costi0_p2_dn2, locals.var_costi0_p2_dn4, locals.var_costi0_p2_dn5, locals.var_costi0_p2_dn6, locals.var_costi0_p2_dn7, locals.var_costi0_p2_dn8, locals.var_costi0_p2_dn9, locals.var_costi0_p2_dn10, locals.var_costi0_p2_dn13,)
    }
};
        locals.var_costi0_p2 = assign19460_e14551;
        locals.var_costi0_p2_dn0 = assign19460_e14551_d_n0;
        locals.var_costi0_p2_dn2 = assign19460_e14551_d_n2;
        locals.var_costi0_p2_dn4 = assign19460_e14551_d_n4;
        locals.var_costi0_p2_dn5 = assign19460_e14551_d_n5;
        locals.var_costi0_p2_dn6 = assign19460_e14551_d_n6;
        locals.var_costi0_p2_dn7 = assign19460_e14551_d_n7;
        locals.var_costi0_p2_dn8 = assign19460_e14551_d_n8;
        locals.var_costi0_p2_dn9 = assign19460_e14551_d_n9;
        locals.var_costi0_p2_dn10 = assign19460_e14551_d_n10;
        locals.var_costi0_p2_dn13 = assign19460_e14551_d_n13;
        locals.var_costi0_p2_rv = 0.0;

        let (assign19470_e14559, assign19470_e14559_d_n0, assign19470_e14559_d_n2, assign19470_e14559_d_n4, assign19470_e14559_d_n5, assign19470_e14559_d_n6, assign19470_e14559_d_n7, assign19470_e14559_d_n8, assign19470_e14559_d_n9, assign19470_e14559_d_n10, assign19470_e14559_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign19470_e14555: f64 = (locals.var_nin * locals.var_nin);
        let assign19470_e14557: f64 = (assign19470_e14555 * locals.var_nsti_p2);
        (assign19470_e14557, (((locals.var_nin_dn0 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn0)) * locals.var_nsti_p2), (((locals.var_nin_dn2 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn2)) * locals.var_nsti_p2), (((locals.var_nin_dn4 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn4)) * locals.var_nsti_p2), (((locals.var_nin_dn5 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn5)) * locals.var_nsti_p2), (((locals.var_nin_dn6 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn6)) * locals.var_nsti_p2), (((locals.var_nin_dn7 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn7)) * locals.var_nsti_p2), (((locals.var_nin_dn8 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn8)) * locals.var_nsti_p2), (((locals.var_nin_dn9 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn9)) * locals.var_nsti_p2), (((locals.var_nin_dn10 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn10)) * locals.var_nsti_p2), (((locals.var_nin_dn13 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn13)) * locals.var_nsti_p2),)
    } else {
        (locals.var_costi1, locals.var_costi1_dn0, locals.var_costi1_dn2, locals.var_costi1_dn4, locals.var_costi1_dn5, locals.var_costi1_dn6, locals.var_costi1_dn7, locals.var_costi1_dn8, locals.var_costi1_dn9, locals.var_costi1_dn10, locals.var_costi1_dn13,)
    }
};
        locals.var_costi1 = assign19470_e14559;
        locals.var_costi1_dn0 = assign19470_e14559_d_n0;
        locals.var_costi1_dn2 = assign19470_e14559_d_n2;
        locals.var_costi1_dn4 = assign19470_e14559_d_n4;
        locals.var_costi1_dn5 = assign19470_e14559_d_n5;
        locals.var_costi1_dn6 = assign19470_e14559_d_n6;
        locals.var_costi1_dn7 = assign19470_e14559_d_n7;
        locals.var_costi1_dn8 = assign19470_e14559_d_n8;
        locals.var_costi1_dn9 = assign19470_e14559_d_n9;
        locals.var_costi1_dn10 = assign19470_e14559_d_n10;
        locals.var_costi1_dn13 = assign19470_e14559_d_n13;
        locals.var_costi1_rv = 0.0;

        let (assign19480_e14567, assign19480_e14567_d_n0, assign19480_e14567_d_n2, assign19480_e14567_d_n4, assign19480_e14567_d_n5, assign19480_e14567_d_n6, assign19480_e14567_d_n7, assign19480_e14567_d_n8, assign19480_e14567_d_n9, assign19480_e14567_d_n10, assign19480_e14567_d_n13,) = {
    if (locals.var_guard352 != 0.0) {
        let assign19480_e14564: f64 = (p.p448 * locals.var_tdiff);
        let assign19480_e14565: f64 = (p.p447 + assign19480_e14564);
        (assign19480_e14565, (p.p448 * locals.var_tdiff_dn0), (p.p448 * locals.var_tdiff_dn2), (p.p448 * locals.var_tdiff_dn4), (p.p448 * locals.var_tdiff_dn5), (p.p448 * locals.var_tdiff_dn6), (p.p448 * locals.var_tdiff_dn7), (p.p448 * locals.var_tdiff_dn8), (p.p448 * locals.var_tdiff_dn9), (p.p448 * locals.var_tdiff_dn10), (p.p448 * locals.var_tdiff_dn13),)
    } else {
        (locals.var_hbdceff, locals.var_hbdceff_dn0, locals.var_hbdceff_dn2, locals.var_hbdceff_dn4, locals.var_hbdceff_dn5, locals.var_hbdceff_dn6, locals.var_hbdceff_dn7, locals.var_hbdceff_dn8, locals.var_hbdceff_dn9, locals.var_hbdceff_dn10, locals.var_hbdceff_dn13,)
    }
};
        locals.var_hbdceff = assign19480_e14567;
        locals.var_hbdceff_dn0 = assign19480_e14567_d_n0;
        locals.var_hbdceff_dn2 = assign19480_e14567_d_n2;
        locals.var_hbdceff_dn4 = assign19480_e14567_d_n4;
        locals.var_hbdceff_dn5 = assign19480_e14567_d_n5;
        locals.var_hbdceff_dn6 = assign19480_e14567_d_n6;
        locals.var_hbdceff_dn7 = assign19480_e14567_d_n7;
        locals.var_hbdceff_dn8 = assign19480_e14567_d_n8;
        locals.var_hbdceff_dn9 = assign19480_e14567_d_n9;
        locals.var_hbdceff_dn10 = assign19480_e14567_d_n10;
        locals.var_hbdceff_dn13 = assign19480_e14567_d_n13;
        locals.var_hbdceff_rv = 0.0;

        let (assign19490_e14571,) = {
    if (locals.var_guard352 != 0.0) {
        (p.p193,)
    } else {
        (locals.var_uc_subtmp,)
    }
};
        locals.var_uc_subtmp = assign19490_e14571;
        locals.var_uc_subtmp_rv = 0.0;

        let assign19520_e14584: f64 = if locals.var_uc_subtmp < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard389 = assign19520_e14584;
        locals.var_guard389_rv = 0.0;

        let (assign19530_e14590,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard389 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_uc_subtmp,)
    }
};
        locals.var_uc_subtmp = assign19530_e14590;
        locals.var_uc_subtmp_rv = 0.0;

        let assign19540_e14593: f64 = if locals.var_uc_subtmp > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard390 = assign19540_e14593;
        locals.var_guard390_rv = 0.0;

        let (assign19550_e14599,) = {
    if ((locals.var_guard352 != 0.0) && (locals.var_guard390 != 0.0)) {
        (0.005,)
    } else {
        (locals.var_uc_subtmp,)
    }
};
        locals.var_uc_subtmp = assign19550_e14599;
        locals.var_uc_subtmp_rv = 0.0;

        let (assign19560_e14606, assign19560_e14606_d_n0, assign19560_e14606_d_n2, assign19560_e14606_d_n4, assign19560_e14606_d_n5, assign19560_e14606_d_n6, assign19560_e14606_d_n7, assign19560_e14606_d_n8, assign19560_e14606_d_n9, assign19560_e14606_d_n10, assign19560_e14606_d_n13,) = {
    if (locals.var_guard352 == 0.0) {
        let assign19560_e14602: f64 = ctx_temp;
        let assign19560_e14604: f64 = (assign19560_e14602 + p.p11);
        (assign19560_e14604, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn13,)
    }
};
        locals.var_ttemp = assign19560_e14606;
        locals.var_ttemp_dn0 = assign19560_e14606_d_n0;
        locals.var_ttemp_dn2 = assign19560_e14606_d_n2;
        locals.var_ttemp_dn4 = assign19560_e14606_d_n4;
        locals.var_ttemp_dn5 = assign19560_e14606_d_n5;
        locals.var_ttemp_dn6 = assign19560_e14606_d_n6;
        locals.var_ttemp_dn7 = assign19560_e14606_d_n7;
        locals.var_ttemp_dn8 = assign19560_e14606_d_n8;
        locals.var_ttemp_dn9 = assign19560_e14606_d_n9;
        locals.var_ttemp_dn10 = assign19560_e14606_d_n10;
        locals.var_ttemp_dn13 = assign19560_e14606_d_n13;
        locals.var_ttemp_rv = 0.0;

        let assign19570_e14609: f64 = (locals.var_weff_ld * p.p7);
        locals.var_weffld_nf = assign19570_e14609;
        locals.var_weffld_nf_rv = 0.0;

        let assign19580_e14612: f64 = (p.p67 + p.p68);
        locals.var_ldrift0 = assign19580_e14612;
        locals.var_ldrift0_rv = 0.0;

        locals.var_vfb = locals.var_uc_vfbc;
        locals.var_vfb_rv = 0.0;

        locals.var_vmaxe = locals.var_vmaxeff;
        locals.var_vmaxe_dn0 = locals.var_vmaxeff_dn0;
        locals.var_vmaxe_dn2 = locals.var_vmaxeff_dn2;
        locals.var_vmaxe_dn4 = locals.var_vmaxeff_dn4;
        locals.var_vmaxe_dn5 = locals.var_vmaxeff_dn5;
        locals.var_vmaxe_dn6 = locals.var_vmaxeff_dn6;
        locals.var_vmaxe_dn7 = locals.var_vmaxeff_dn7;
        locals.var_vmaxe_dn8 = locals.var_vmaxeff_dn8;
        locals.var_vmaxe_dn9 = locals.var_vmaxeff_dn9;
        locals.var_vmaxe_dn10 = locals.var_vmaxeff_dn10;
        locals.var_vmaxe_dn13 = locals.var_vmaxeff_dn13;
        locals.var_vmaxe_rv = 0.0;

        locals.var_c_eox = locals.var_cecox;
        locals.var_c_eox_rv = 0.0;

        locals.var_tox0 = p.p95;
        locals.var_tox0_rv = 0.0;

        let assign19630_e14619: f64 = (locals.var_c_eox / locals.var_tox0);
        locals.var_cox0 = assign19630_e14619;
        locals.var_cox0_rv = 0.0;

        let assign19640_e14622: f64 = (1.0 / locals.var_cox0);
        locals.var_cox0_inv = assign19640_e14622;
        locals.var_cox0_inv_rv = 0.0;

        let assign19650_e14625: f64 = (locals.var_c_eox / locals.var_uc_toxb);
        locals.var_coxb0 = assign19650_e14625;
        locals.var_coxb0_rv = 0.0;

        let assign19660_e14628: f64 = (p.p87 * p.p434);
        locals.var_vgs_min = assign19660_e14628;
        locals.var_vgs_min_rv = 0.0;

        let assign19670_e14632: f64 = (locals.var_pb2 - p.p262);
        let assign19670_e14633: f64 = (0.8 - assign19670_e14632);
        let assign19670_e14635: f64 = (assign19670_e14633 - 0.1);
        locals.var_tmf1 = assign19670_e14635;
        locals.var_tmf1_dn0 = (-locals.var_pb2_dn0);
        locals.var_tmf1_dn2 = (-locals.var_pb2_dn2);
        locals.var_tmf1_dn4 = (-locals.var_pb2_dn4);
        locals.var_tmf1_dn5 = (-locals.var_pb2_dn5);
        locals.var_tmf1_dn6 = (-locals.var_pb2_dn6);
        locals.var_tmf1_dn7 = (-locals.var_pb2_dn7);
        locals.var_tmf1_dn8 = (-locals.var_pb2_dn8);
        locals.var_tmf1_dn9 = (-locals.var_pb2_dn9);
        locals.var_tmf1_dn10 = (-locals.var_pb2_dn10);
        locals.var_tmf1_dn13 = (-locals.var_pb2_dn13);
        locals.var_tmf1_rv = 0.0;

        let assign19680_e14638: f64 = (4.0 * 0.8);
        let assign19680_e14640: f64 = (assign19680_e14638 * 0.1);
        locals.var_tmf2 = assign19680_e14640;
        locals.var_tmf2_dn0 = 0.0;
        locals.var_tmf2_dn2 = 0.0;
        locals.var_tmf2_dn4 = 0.0;
        locals.var_tmf2_dn5 = 0.0;
        locals.var_tmf2_dn6 = 0.0;
        locals.var_tmf2_dn7 = 0.0;
        locals.var_tmf2_dn8 = 0.0;
        locals.var_tmf2_dn9 = 0.0;
        locals.var_tmf2_dn10 = 0.0;
        locals.var_tmf2_dn13 = 0.0;
        locals.var_tmf2_rv = 0.0;

        let (assign19690_e14647, assign19690_e14647_d_n0, assign19690_e14647_d_n2, assign19690_e14647_d_n4, assign19690_e14647_d_n5, assign19690_e14647_d_n6, assign19690_e14647_d_n7, assign19690_e14647_d_n8, assign19690_e14647_d_n9, assign19690_e14647_d_n10, assign19690_e14647_d_n13,) = {
    if (locals.var_tmf2 > 0.0) {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    } else {
        let assign19690_e14646: f64 = (-locals.var_tmf2);
        (assign19690_e14646, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
    }
};
        locals.var_tmf2 = assign19690_e14647;
        locals.var_tmf2_dn0 = assign19690_e14647_d_n0;
        locals.var_tmf2_dn2 = assign19690_e14647_d_n2;
        locals.var_tmf2_dn4 = assign19690_e14647_d_n4;
        locals.var_tmf2_dn5 = assign19690_e14647_d_n5;
        locals.var_tmf2_dn6 = assign19690_e14647_d_n6;
        locals.var_tmf2_dn7 = assign19690_e14647_d_n7;
        locals.var_tmf2_dn8 = assign19690_e14647_d_n8;
        locals.var_tmf2_dn9 = assign19690_e14647_d_n9;
        locals.var_tmf2_dn10 = assign19690_e14647_d_n10;
        locals.var_tmf2_dn13 = assign19690_e14647_d_n13;
        locals.var_tmf2_rv = 0.0;

        let assign19700_e14650: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19700_e14652: f64 = (assign19700_e14650 + locals.var_tmf2);
        let assign19700_e14653: f64 = (assign19700_e14652).sqrt();
        locals.var_tmf2 = assign19700_e14653;
        locals.var_tmf2_dn0 = ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19700_e14653));
        locals.var_tmf2_dn2 = ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19700_e14653));
        locals.var_tmf2_dn4 = ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign19700_e14653));
        locals.var_tmf2_dn5 = ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign19700_e14653));
        locals.var_tmf2_dn6 = ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign19700_e14653));
        locals.var_tmf2_dn7 = ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign19700_e14653));
        locals.var_tmf2_dn8 = ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign19700_e14653));
        locals.var_tmf2_dn9 = ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign19700_e14653));
        locals.var_tmf2_dn10 = ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign19700_e14653));
        locals.var_tmf2_dn13 = ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign19700_e14653));
        locals.var_tmf2_rv = 0.0;

        let assign19710_e14658: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign19710_e14659: f64 = (1.0 + assign19710_e14658);
        let assign19710_e14660: f64 = (0.5 * assign19710_e14659);
        locals.var_t0 = assign19710_e14660;
        locals.var_t0_dn0 = (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn2 = (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn4 = (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn5 = (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn6 = (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn7 = (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn8 = (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn9 = (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn10 = (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn13 = (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_rv = 0.0;

        let assign19720_e14665: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19720_e14666: f64 = (0.5 * assign19720_e14665);
        let assign19720_e14667: f64 = (0.8 - assign19720_e14666);
        locals.var_t1 = assign19720_e14667;
        locals.var_t1_dn0 = (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)));
        locals.var_t1_dn2 = (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)));
        locals.var_t1_dn4 = (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)));
        locals.var_t1_dn5 = (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)));
        locals.var_t1_dn6 = (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)));
        locals.var_t1_dn7 = (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)));
        locals.var_t1_dn8 = (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)));
        locals.var_t1_dn9 = (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)));
        locals.var_t1_dn10 = (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)));
        locals.var_t1_dn13 = (-(0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13)));
        locals.var_t1_rv = 0.0;

        locals.var_vbs_max = locals.var_t1;
        locals.var_vbs_max_dn0 = locals.var_t1_dn0;
        locals.var_vbs_max_dn2 = locals.var_t1_dn2;
        locals.var_vbs_max_dn4 = locals.var_t1_dn4;
        locals.var_vbs_max_dn5 = locals.var_t1_dn5;
        locals.var_vbs_max_dn6 = locals.var_t1_dn6;
        locals.var_vbs_max_dn7 = locals.var_t1_dn7;
        locals.var_vbs_max_dn8 = locals.var_t1_dn8;
        locals.var_vbs_max_dn9 = locals.var_t1_dn9;
        locals.var_vbs_max_dn10 = locals.var_t1_dn10;
        locals.var_vbs_max_dn13 = locals.var_t1_dn13;
        locals.var_vbs_max_rv = 0.0;

        let assign19740_e14671: f64 = (locals.var_pb20 - p.p262);
        let assign19740_e14673: f64 = if assign19740_e14671 < locals.var_vbs_max { 1.0 } else { 0.0 };
        locals.var_guard391 = assign19740_e14673;
        locals.var_guard391_rv = 0.0;

        let (assign19750_e14679, assign19750_e14679_d_n0, assign19750_e14679_d_n2, assign19750_e14679_d_n4, assign19750_e14679_d_n5, assign19750_e14679_d_n6, assign19750_e14679_d_n7, assign19750_e14679_d_n8, assign19750_e14679_d_n9, assign19750_e14679_d_n10, assign19750_e14679_d_n13,) = {
    if (locals.var_guard391 != 0.0) {
        let assign19750_e14677: f64 = (locals.var_pb20 - p.p262);
        (assign19750_e14677, locals.var_pb20_dn0, locals.var_pb20_dn2, locals.var_pb20_dn4, locals.var_pb20_dn5, locals.var_pb20_dn6, locals.var_pb20_dn7, locals.var_pb20_dn8, locals.var_pb20_dn9, locals.var_pb20_dn10, locals.var_pb20_dn13,)
    } else {
        (locals.var_vbs_max, locals.var_vbs_max_dn0, locals.var_vbs_max_dn2, locals.var_vbs_max_dn4, locals.var_vbs_max_dn5, locals.var_vbs_max_dn6, locals.var_vbs_max_dn7, locals.var_vbs_max_dn8, locals.var_vbs_max_dn9, locals.var_vbs_max_dn10, locals.var_vbs_max_dn13,)
    }
};
        locals.var_vbs_max = assign19750_e14679;
        locals.var_vbs_max_dn0 = assign19750_e14679_d_n0;
        locals.var_vbs_max_dn2 = assign19750_e14679_d_n2;
        locals.var_vbs_max_dn4 = assign19750_e14679_d_n4;
        locals.var_vbs_max_dn5 = assign19750_e14679_d_n5;
        locals.var_vbs_max_dn6 = assign19750_e14679_d_n6;
        locals.var_vbs_max_dn7 = assign19750_e14679_d_n7;
        locals.var_vbs_max_dn8 = assign19750_e14679_d_n8;
        locals.var_vbs_max_dn9 = assign19750_e14679_d_n9;
        locals.var_vbs_max_dn10 = assign19750_e14679_d_n10;
        locals.var_vbs_max_dn13 = assign19750_e14679_d_n13;
        locals.var_vbs_max_rv = 0.0;

        let assign19760_e14682: f64 = (locals.var_pb2c - p.p262);
        let assign19760_e14684: f64 = if assign19760_e14682 < locals.var_vbs_max { 1.0 } else { 0.0 };
        locals.var_guard392 = assign19760_e14684;
        locals.var_guard392_rv = 0.0;

        let (assign19770_e14690, assign19770_e14690_d_n0, assign19770_e14690_d_n2, assign19770_e14690_d_n4, assign19770_e14690_d_n5, assign19770_e14690_d_n6, assign19770_e14690_d_n7, assign19770_e14690_d_n8, assign19770_e14690_d_n9, assign19770_e14690_d_n10, assign19770_e14690_d_n13,) = {
    if (locals.var_guard392 != 0.0) {
        let assign19770_e14688: f64 = (locals.var_pb2c - p.p262);
        (assign19770_e14688, locals.var_pb2c_dn0, locals.var_pb2c_dn2, locals.var_pb2c_dn4, locals.var_pb2c_dn5, locals.var_pb2c_dn6, locals.var_pb2c_dn7, locals.var_pb2c_dn8, locals.var_pb2c_dn9, locals.var_pb2c_dn10, locals.var_pb2c_dn13,)
    } else {
        (locals.var_vbs_max, locals.var_vbs_max_dn0, locals.var_vbs_max_dn2, locals.var_vbs_max_dn4, locals.var_vbs_max_dn5, locals.var_vbs_max_dn6, locals.var_vbs_max_dn7, locals.var_vbs_max_dn8, locals.var_vbs_max_dn9, locals.var_vbs_max_dn10, locals.var_vbs_max_dn13,)
    }
};
        locals.var_vbs_max = assign19770_e14690;
        locals.var_vbs_max_dn0 = assign19770_e14690_d_n0;
        locals.var_vbs_max_dn2 = assign19770_e14690_d_n2;
        locals.var_vbs_max_dn4 = assign19770_e14690_d_n4;
        locals.var_vbs_max_dn5 = assign19770_e14690_d_n5;
        locals.var_vbs_max_dn6 = assign19770_e14690_d_n6;
        locals.var_vbs_max_dn7 = assign19770_e14690_d_n7;
        locals.var_vbs_max_dn8 = assign19770_e14690_d_n8;
        locals.var_vbs_max_dn9 = assign19770_e14690_d_n9;
        locals.var_vbs_max_dn10 = assign19770_e14690_d_n10;
        locals.var_vbs_max_dn13 = assign19770_e14690_d_n13;
        locals.var_vbs_max_rv = 0.0;

        let assign19780_e14697: f64 = if ((locals.var_uc_codep > 0.0) && (locals.var_uc_codep <= 3.0)) { 1.0 } else { 0.0 };
        locals.var_guard393 = assign19780_e14697;
        locals.var_guard393_rv = 0.0;

        let assign19790_e14700: f64 = (locals.var_pb2n - p.p262);
        let assign19790_e14702: f64 = if assign19790_e14700 < locals.var_vbs_max { 1.0 } else { 0.0 };
        locals.var_guard394 = assign19790_e14702;
        locals.var_guard394_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_48(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign19800_e14710, assign19800_e14710_d_n0, assign19800_e14710_d_n2, assign19800_e14710_d_n4, assign19800_e14710_d_n5, assign19800_e14710_d_n6, assign19800_e14710_d_n7, assign19800_e14710_d_n8, assign19800_e14710_d_n9, assign19800_e14710_d_n10, assign19800_e14710_d_n13,) = {
    if ((locals.var_guard393 != 0.0) && (locals.var_guard394 != 0.0)) {
        let assign19800_e14708: f64 = (locals.var_pb2n - p.p262);
        (assign19800_e14708, locals.var_pb2n_dn0, locals.var_pb2n_dn2, locals.var_pb2n_dn4, locals.var_pb2n_dn5, locals.var_pb2n_dn6, locals.var_pb2n_dn7, locals.var_pb2n_dn8, locals.var_pb2n_dn9, locals.var_pb2n_dn10, locals.var_pb2n_dn13,)
    } else {
        (locals.var_vbs_max, locals.var_vbs_max_dn0, locals.var_vbs_max_dn2, locals.var_vbs_max_dn4, locals.var_vbs_max_dn5, locals.var_vbs_max_dn6, locals.var_vbs_max_dn7, locals.var_vbs_max_dn8, locals.var_vbs_max_dn9, locals.var_vbs_max_dn10, locals.var_vbs_max_dn13,)
    }
};
        locals.var_vbs_max = assign19800_e14710;
        locals.var_vbs_max_dn0 = assign19800_e14710_d_n0;
        locals.var_vbs_max_dn2 = assign19800_e14710_d_n2;
        locals.var_vbs_max_dn4 = assign19800_e14710_d_n4;
        locals.var_vbs_max_dn5 = assign19800_e14710_d_n5;
        locals.var_vbs_max_dn6 = assign19800_e14710_d_n6;
        locals.var_vbs_max_dn7 = assign19800_e14710_d_n7;
        locals.var_vbs_max_dn8 = assign19800_e14710_d_n8;
        locals.var_vbs_max_dn9 = assign19800_e14710_d_n9;
        locals.var_vbs_max_dn10 = assign19800_e14710_d_n10;
        locals.var_vbs_max_dn13 = assign19800_e14710_d_n13;
        locals.var_vbs_max_rv = 0.0;

        let assign19810_e14713: f64 = (locals.var_vbipn - p.p262);
        let assign19810_e14715: f64 = if assign19810_e14713 < locals.var_vbs_max { 1.0 } else { 0.0 };
        locals.var_guard395 = assign19810_e14715;
        locals.var_guard395_rv = 0.0;

        let (assign19820_e14723, assign19820_e14723_d_n0, assign19820_e14723_d_n2, assign19820_e14723_d_n4, assign19820_e14723_d_n5, assign19820_e14723_d_n6, assign19820_e14723_d_n7, assign19820_e14723_d_n8, assign19820_e14723_d_n9, assign19820_e14723_d_n10, assign19820_e14723_d_n13,) = {
    if ((locals.var_guard393 != 0.0) && (locals.var_guard395 != 0.0)) {
        let assign19820_e14721: f64 = (locals.var_vbipn - p.p262);
        (assign19820_e14721, locals.var_vbipn_dn0, locals.var_vbipn_dn2, locals.var_vbipn_dn4, locals.var_vbipn_dn5, locals.var_vbipn_dn6, locals.var_vbipn_dn7, locals.var_vbipn_dn8, locals.var_vbipn_dn9, locals.var_vbipn_dn10, locals.var_vbipn_dn13,)
    } else {
        (locals.var_vbs_max, locals.var_vbs_max_dn0, locals.var_vbs_max_dn2, locals.var_vbs_max_dn4, locals.var_vbs_max_dn5, locals.var_vbs_max_dn6, locals.var_vbs_max_dn7, locals.var_vbs_max_dn8, locals.var_vbs_max_dn9, locals.var_vbs_max_dn10, locals.var_vbs_max_dn13,)
    }
};
        locals.var_vbs_max = assign19820_e14723;
        locals.var_vbs_max_dn0 = assign19820_e14723_d_n0;
        locals.var_vbs_max_dn2 = assign19820_e14723_d_n2;
        locals.var_vbs_max_dn4 = assign19820_e14723_d_n4;
        locals.var_vbs_max_dn5 = assign19820_e14723_d_n5;
        locals.var_vbs_max_dn6 = assign19820_e14723_d_n6;
        locals.var_vbs_max_dn7 = assign19820_e14723_d_n7;
        locals.var_vbs_max_dn8 = assign19820_e14723_d_n8;
        locals.var_vbs_max_dn9 = assign19820_e14723_d_n9;
        locals.var_vbs_max_dn10 = assign19820_e14723_d_n10;
        locals.var_vbs_max_dn13 = assign19820_e14723_d_n13;
        locals.var_vbs_max_rv = 0.0;

        let assign19830_e14727: f64 = (locals.var_vbs_max * 0.5);
        let assign19830_e14728: f64 = if locals.var_vbs_bnd > assign19830_e14727 { 1.0 } else { 0.0 };
        locals.var_guard396 = assign19830_e14728;
        locals.var_guard396_rv = 0.0;

        let (assign19840_e14734, assign19840_e14734_d_n0, assign19840_e14734_d_n2, assign19840_e14734_d_n4, assign19840_e14734_d_n5, assign19840_e14734_d_n6, assign19840_e14734_d_n7, assign19840_e14734_d_n8, assign19840_e14734_d_n9, assign19840_e14734_d_n10, assign19840_e14734_d_n13,) = {
    if (locals.var_guard396 != 0.0) {
        let assign19840_e14732: f64 = (0.5 * locals.var_vbs_max);
        (assign19840_e14732, (0.5 * locals.var_vbs_max_dn0), (0.5 * locals.var_vbs_max_dn2), (0.5 * locals.var_vbs_max_dn4), (0.5 * locals.var_vbs_max_dn5), (0.5 * locals.var_vbs_max_dn6), (0.5 * locals.var_vbs_max_dn7), (0.5 * locals.var_vbs_max_dn8), (0.5 * locals.var_vbs_max_dn9), (0.5 * locals.var_vbs_max_dn10), (0.5 * locals.var_vbs_max_dn13),)
    } else {
        (locals.var_vbs_bnd, locals.var_vbs_bnd_dn0, locals.var_vbs_bnd_dn2, locals.var_vbs_bnd_dn4, locals.var_vbs_bnd_dn5, locals.var_vbs_bnd_dn6, locals.var_vbs_bnd_dn7, locals.var_vbs_bnd_dn8, locals.var_vbs_bnd_dn9, locals.var_vbs_bnd_dn10, locals.var_vbs_bnd_dn13,)
    }
};
        locals.var_vbs_bnd = assign19840_e14734;
        locals.var_vbs_bnd_dn0 = assign19840_e14734_d_n0;
        locals.var_vbs_bnd_dn2 = assign19840_e14734_d_n2;
        locals.var_vbs_bnd_dn4 = assign19840_e14734_d_n4;
        locals.var_vbs_bnd_dn5 = assign19840_e14734_d_n5;
        locals.var_vbs_bnd_dn6 = assign19840_e14734_d_n6;
        locals.var_vbs_bnd_dn7 = assign19840_e14734_d_n7;
        locals.var_vbs_bnd_dn8 = assign19840_e14734_d_n8;
        locals.var_vbs_bnd_dn9 = assign19840_e14734_d_n9;
        locals.var_vbs_bnd_dn10 = assign19840_e14734_d_n10;
        locals.var_vbs_bnd_dn13 = assign19840_e14734_d_n13;
        locals.var_vbs_bnd_rv = 0.0;

        let assign19850_e14736: f64 = if param_given[338] { 1.0 } else { 0.0 };
        locals.var_guard397 = assign19850_e14736;
        locals.var_guard397_rv = 0.0;

        let (assign19860_e14740, assign19860_e14740_d_n0, assign19860_e14740_d_n2, assign19860_e14740_d_n4, assign19860_e14740_d_n5, assign19860_e14740_d_n6, assign19860_e14740_d_n7, assign19860_e14740_d_n8, assign19860_e14740_d_n9, assign19860_e14740_d_n10, assign19860_e14740_d_n13,) = {
    if (locals.var_guard397 != 0.0) {
        (p.p338, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_max_local, locals.var_vbs_max_local_dn0, locals.var_vbs_max_local_dn2, locals.var_vbs_max_local_dn4, locals.var_vbs_max_local_dn5, locals.var_vbs_max_local_dn6, locals.var_vbs_max_local_dn7, locals.var_vbs_max_local_dn8, locals.var_vbs_max_local_dn9, locals.var_vbs_max_local_dn10, locals.var_vbs_max_local_dn13,)
    }
};
        locals.var_vbs_max_local = assign19860_e14740;
        locals.var_vbs_max_local_dn0 = assign19860_e14740_d_n0;
        locals.var_vbs_max_local_dn2 = assign19860_e14740_d_n2;
        locals.var_vbs_max_local_dn4 = assign19860_e14740_d_n4;
        locals.var_vbs_max_local_dn5 = assign19860_e14740_d_n5;
        locals.var_vbs_max_local_dn6 = assign19860_e14740_d_n6;
        locals.var_vbs_max_local_dn7 = assign19860_e14740_d_n7;
        locals.var_vbs_max_local_dn8 = assign19860_e14740_d_n8;
        locals.var_vbs_max_local_dn9 = assign19860_e14740_d_n9;
        locals.var_vbs_max_local_dn10 = assign19860_e14740_d_n10;
        locals.var_vbs_max_local_dn13 = assign19860_e14740_d_n13;
        locals.var_vbs_max_local_rv = 0.0;

        let (assign19870_e14745, assign19870_e14745_d_n0, assign19870_e14745_d_n2, assign19870_e14745_d_n4, assign19870_e14745_d_n5, assign19870_e14745_d_n6, assign19870_e14745_d_n7, assign19870_e14745_d_n8, assign19870_e14745_d_n9, assign19870_e14745_d_n10, assign19870_e14745_d_n13,) = {
    if (locals.var_guard397 == 0.0) {
        (locals.var_vbs_max, locals.var_vbs_max_dn0, locals.var_vbs_max_dn2, locals.var_vbs_max_dn4, locals.var_vbs_max_dn5, locals.var_vbs_max_dn6, locals.var_vbs_max_dn7, locals.var_vbs_max_dn8, locals.var_vbs_max_dn9, locals.var_vbs_max_dn10, locals.var_vbs_max_dn13,)
    } else {
        (locals.var_vbs_max_local, locals.var_vbs_max_local_dn0, locals.var_vbs_max_local_dn2, locals.var_vbs_max_local_dn4, locals.var_vbs_max_local_dn5, locals.var_vbs_max_local_dn6, locals.var_vbs_max_local_dn7, locals.var_vbs_max_local_dn8, locals.var_vbs_max_local_dn9, locals.var_vbs_max_local_dn10, locals.var_vbs_max_local_dn13,)
    }
};
        locals.var_vbs_max_local = assign19870_e14745;
        locals.var_vbs_max_local_dn0 = assign19870_e14745_d_n0;
        locals.var_vbs_max_local_dn2 = assign19870_e14745_d_n2;
        locals.var_vbs_max_local_dn4 = assign19870_e14745_d_n4;
        locals.var_vbs_max_local_dn5 = assign19870_e14745_d_n5;
        locals.var_vbs_max_local_dn6 = assign19870_e14745_d_n6;
        locals.var_vbs_max_local_dn7 = assign19870_e14745_d_n7;
        locals.var_vbs_max_local_dn8 = assign19870_e14745_d_n8;
        locals.var_vbs_max_local_dn9 = assign19870_e14745_d_n9;
        locals.var_vbs_max_local_dn10 = assign19870_e14745_d_n10;
        locals.var_vbs_max_local_dn13 = assign19870_e14745_d_n13;
        locals.var_vbs_max_local_rv = 0.0;

        let assign19880_e14747: f64 = if param_given[339] { 1.0 } else { 0.0 };
        locals.var_guard398 = assign19880_e14747;
        locals.var_guard398_rv = 0.0;

        let (assign19890_e14751, assign19890_e14751_d_n0, assign19890_e14751_d_n2, assign19890_e14751_d_n4, assign19890_e14751_d_n5, assign19890_e14751_d_n6, assign19890_e14751_d_n7, assign19890_e14751_d_n8, assign19890_e14751_d_n9, assign19890_e14751_d_n10, assign19890_e14751_d_n13,) = {
    if (locals.var_guard398 != 0.0) {
        (p.p339, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_bnd_local, locals.var_vbs_bnd_local_dn0, locals.var_vbs_bnd_local_dn2, locals.var_vbs_bnd_local_dn4, locals.var_vbs_bnd_local_dn5, locals.var_vbs_bnd_local_dn6, locals.var_vbs_bnd_local_dn7, locals.var_vbs_bnd_local_dn8, locals.var_vbs_bnd_local_dn9, locals.var_vbs_bnd_local_dn10, locals.var_vbs_bnd_local_dn13,)
    }
};
        locals.var_vbs_bnd_local = assign19890_e14751;
        locals.var_vbs_bnd_local_dn0 = assign19890_e14751_d_n0;
        locals.var_vbs_bnd_local_dn2 = assign19890_e14751_d_n2;
        locals.var_vbs_bnd_local_dn4 = assign19890_e14751_d_n4;
        locals.var_vbs_bnd_local_dn5 = assign19890_e14751_d_n5;
        locals.var_vbs_bnd_local_dn6 = assign19890_e14751_d_n6;
        locals.var_vbs_bnd_local_dn7 = assign19890_e14751_d_n7;
        locals.var_vbs_bnd_local_dn8 = assign19890_e14751_d_n8;
        locals.var_vbs_bnd_local_dn9 = assign19890_e14751_d_n9;
        locals.var_vbs_bnd_local_dn10 = assign19890_e14751_d_n10;
        locals.var_vbs_bnd_local_dn13 = assign19890_e14751_d_n13;
        locals.var_vbs_bnd_local_rv = 0.0;

        let assign19900_e14753: f64 = if param_given[338] { 1.0 } else { 0.0 };
        locals.var_guard399 = assign19900_e14753;
        locals.var_guard399_rv = 0.0;

        let (assign19910_e14762, assign19910_e14762_d_n0, assign19910_e14762_d_n2, assign19910_e14762_d_n4, assign19910_e14762_d_n5, assign19910_e14762_d_n6, assign19910_e14762_d_n7, assign19910_e14762_d_n8, assign19910_e14762_d_n9, assign19910_e14762_d_n10, assign19910_e14762_d_n13,) = {
    if ((locals.var_guard398 == 0.0) && (locals.var_guard399 != 0.0)) {
        let assign19910_e14760: f64 = (0.5 * locals.var_vbs_max_local);
        (assign19910_e14760, (0.5 * locals.var_vbs_max_local_dn0), (0.5 * locals.var_vbs_max_local_dn2), (0.5 * locals.var_vbs_max_local_dn4), (0.5 * locals.var_vbs_max_local_dn5), (0.5 * locals.var_vbs_max_local_dn6), (0.5 * locals.var_vbs_max_local_dn7), (0.5 * locals.var_vbs_max_local_dn8), (0.5 * locals.var_vbs_max_local_dn9), (0.5 * locals.var_vbs_max_local_dn10), (0.5 * locals.var_vbs_max_local_dn13),)
    } else {
        (locals.var_vbs_bnd_local, locals.var_vbs_bnd_local_dn0, locals.var_vbs_bnd_local_dn2, locals.var_vbs_bnd_local_dn4, locals.var_vbs_bnd_local_dn5, locals.var_vbs_bnd_local_dn6, locals.var_vbs_bnd_local_dn7, locals.var_vbs_bnd_local_dn8, locals.var_vbs_bnd_local_dn9, locals.var_vbs_bnd_local_dn10, locals.var_vbs_bnd_local_dn13,)
    }
};
        locals.var_vbs_bnd_local = assign19910_e14762;
        locals.var_vbs_bnd_local_dn0 = assign19910_e14762_d_n0;
        locals.var_vbs_bnd_local_dn2 = assign19910_e14762_d_n2;
        locals.var_vbs_bnd_local_dn4 = assign19910_e14762_d_n4;
        locals.var_vbs_bnd_local_dn5 = assign19910_e14762_d_n5;
        locals.var_vbs_bnd_local_dn6 = assign19910_e14762_d_n6;
        locals.var_vbs_bnd_local_dn7 = assign19910_e14762_d_n7;
        locals.var_vbs_bnd_local_dn8 = assign19910_e14762_d_n8;
        locals.var_vbs_bnd_local_dn9 = assign19910_e14762_d_n9;
        locals.var_vbs_bnd_local_dn10 = assign19910_e14762_d_n10;
        locals.var_vbs_bnd_local_dn13 = assign19910_e14762_d_n13;
        locals.var_vbs_bnd_local_rv = 0.0;

        let (assign19920_e14770, assign19920_e14770_d_n0, assign19920_e14770_d_n2, assign19920_e14770_d_n4, assign19920_e14770_d_n5, assign19920_e14770_d_n6, assign19920_e14770_d_n7, assign19920_e14770_d_n8, assign19920_e14770_d_n9, assign19920_e14770_d_n10, assign19920_e14770_d_n13,) = {
    if ((locals.var_guard398 == 0.0) && (locals.var_guard399 == 0.0)) {
        (locals.var_vbs_bnd, locals.var_vbs_bnd_dn0, locals.var_vbs_bnd_dn2, locals.var_vbs_bnd_dn4, locals.var_vbs_bnd_dn5, locals.var_vbs_bnd_dn6, locals.var_vbs_bnd_dn7, locals.var_vbs_bnd_dn8, locals.var_vbs_bnd_dn9, locals.var_vbs_bnd_dn10, locals.var_vbs_bnd_dn13,)
    } else {
        (locals.var_vbs_bnd_local, locals.var_vbs_bnd_local_dn0, locals.var_vbs_bnd_local_dn2, locals.var_vbs_bnd_local_dn4, locals.var_vbs_bnd_local_dn5, locals.var_vbs_bnd_local_dn6, locals.var_vbs_bnd_local_dn7, locals.var_vbs_bnd_local_dn8, locals.var_vbs_bnd_local_dn9, locals.var_vbs_bnd_local_dn10, locals.var_vbs_bnd_local_dn13,)
    }
};
        locals.var_vbs_bnd_local = assign19920_e14770;
        locals.var_vbs_bnd_local_dn0 = assign19920_e14770_d_n0;
        locals.var_vbs_bnd_local_dn2 = assign19920_e14770_d_n2;
        locals.var_vbs_bnd_local_dn4 = assign19920_e14770_d_n4;
        locals.var_vbs_bnd_local_dn5 = assign19920_e14770_d_n5;
        locals.var_vbs_bnd_local_dn6 = assign19920_e14770_d_n6;
        locals.var_vbs_bnd_local_dn7 = assign19920_e14770_d_n7;
        locals.var_vbs_bnd_local_dn8 = assign19920_e14770_d_n8;
        locals.var_vbs_bnd_local_dn9 = assign19920_e14770_d_n9;
        locals.var_vbs_bnd_local_dn10 = assign19920_e14770_d_n10;
        locals.var_vbs_bnd_local_dn13 = assign19920_e14770_d_n13;
        locals.var_vbs_bnd_local_rv = 0.0;

        let assign19930_e14774: f64 = (locals.var_vbs_max_local * 0.5);
        let assign19930_e14775: f64 = if locals.var_vbs_bnd_local > assign19930_e14774 { 1.0 } else { 0.0 };
        locals.var_guard400 = assign19930_e14775;
        locals.var_guard400_rv = 0.0;

        let (assign19940_e14781, assign19940_e14781_d_n0, assign19940_e14781_d_n2, assign19940_e14781_d_n4, assign19940_e14781_d_n5, assign19940_e14781_d_n6, assign19940_e14781_d_n7, assign19940_e14781_d_n8, assign19940_e14781_d_n9, assign19940_e14781_d_n10, assign19940_e14781_d_n13,) = {
    if (locals.var_guard400 != 0.0) {
        let assign19940_e14779: f64 = (0.5 * locals.var_vbs_max_local);
        (assign19940_e14779, (0.5 * locals.var_vbs_max_local_dn0), (0.5 * locals.var_vbs_max_local_dn2), (0.5 * locals.var_vbs_max_local_dn4), (0.5 * locals.var_vbs_max_local_dn5), (0.5 * locals.var_vbs_max_local_dn6), (0.5 * locals.var_vbs_max_local_dn7), (0.5 * locals.var_vbs_max_local_dn8), (0.5 * locals.var_vbs_max_local_dn9), (0.5 * locals.var_vbs_max_local_dn10), (0.5 * locals.var_vbs_max_local_dn13),)
    } else {
        (locals.var_vbs_bnd_local, locals.var_vbs_bnd_local_dn0, locals.var_vbs_bnd_local_dn2, locals.var_vbs_bnd_local_dn4, locals.var_vbs_bnd_local_dn5, locals.var_vbs_bnd_local_dn6, locals.var_vbs_bnd_local_dn7, locals.var_vbs_bnd_local_dn8, locals.var_vbs_bnd_local_dn9, locals.var_vbs_bnd_local_dn10, locals.var_vbs_bnd_local_dn13,)
    }
};
        locals.var_vbs_bnd_local = assign19940_e14781;
        locals.var_vbs_bnd_local_dn0 = assign19940_e14781_d_n0;
        locals.var_vbs_bnd_local_dn2 = assign19940_e14781_d_n2;
        locals.var_vbs_bnd_local_dn4 = assign19940_e14781_d_n4;
        locals.var_vbs_bnd_local_dn5 = assign19940_e14781_d_n5;
        locals.var_vbs_bnd_local_dn6 = assign19940_e14781_d_n6;
        locals.var_vbs_bnd_local_dn7 = assign19940_e14781_d_n7;
        locals.var_vbs_bnd_local_dn8 = assign19940_e14781_d_n8;
        locals.var_vbs_bnd_local_dn9 = assign19940_e14781_d_n9;
        locals.var_vbs_bnd_local_dn10 = assign19940_e14781_d_n10;
        locals.var_vbs_bnd_local_dn13 = assign19940_e14781_d_n13;
        locals.var_vbs_bnd_local_rv = 0.0;

        let assign19950_e14788: f64 = if ((locals.var_rse > 0.0) || (locals.var_rde > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard401 = assign19950_e14788;
        locals.var_guard401_rv = 0.0;

        let assign19960_e14791: f64 = if locals.var_uc_corsrd == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard402 = assign19960_e14791;
        locals.var_guard402_rv = 0.0;

        let (assign19970_e14797,) = {
    if ((locals.var_guard401 != 0.0) && (locals.var_guard402 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_rsrd,)
    }
};
        locals.var_flg_rsrd = assign19970_e14797;
        locals.var_flg_rsrd_rv = 0.0;

        let assign19980_e14800: f64 = if locals.var_uc_corsrd == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard403 = assign19980_e14800;
        locals.var_guard403_rv = 0.0;

        let (assign19990_e14806,) = {
    if ((locals.var_guard401 != 0.0) && (locals.var_guard403 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_flg_rsrd,)
    }
};
        locals.var_flg_rsrd = assign19990_e14806;
        locals.var_flg_rsrd_rv = 0.0;

        let assign20000_e14809: f64 = if locals.var_uc_corsrd == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard404 = assign20000_e14809;
        locals.var_guard404_rv = 0.0;

        let (assign20010_e14815,) = {
    if ((locals.var_guard401 != 0.0) && (locals.var_guard404 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_flg_rsrd,)
    }
};
        locals.var_flg_rsrd = assign20010_e14815;
        locals.var_flg_rsrd_rv = 0.0;

        locals.var_flg_pprv = 0.0;
        locals.var_flg_pprv_rv = 0.0;

        let assign20030_e14827: f64 = (locals.var_mks_nsubsub + locals.var_uc_nover);
        let assign20030_e14828: f64 = (locals.var_uc_nover * assign20030_e14827);
        let assign20030_e14831: f64 = if (((locals.var_uc_cordrift == 1.0) && (p.p54 == 1.0)) && (assign20030_e14828 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard405 = assign20030_e14831;
        locals.var_guard405_rv = 0.0;

        let (assign20040_e14835, assign20040_e14835_d_n0, assign20040_e14835_d_n2,) = {
    if (locals.var_guard405 != 0.0) {
        (locals.var_vdsei, locals.var_vdsei_dn0, locals.var_vdsei_dn2,)
    } else {
        (locals.var_vdsegmt, locals.var_vdsegmt_dn0, locals.var_vdsegmt_dn2,)
    }
};
        locals.var_vdsegmt = assign20040_e14835;
        locals.var_vdsegmt_dn0 = assign20040_e14835_d_n0;
        locals.var_vdsegmt_dn2 = assign20040_e14835_d_n2;
        locals.var_vdsegmt_rv = 0.0;

        let assign20050_e14838: f64 = if locals.var_vdsegmt >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard406 = assign20050_e14838;
        locals.var_guard406_rv = 0.0;

        let (assign20060_e14844, assign20060_e14844_d_n0, assign20060_e14844_d_n2,) = {
    if ((locals.var_guard405 != 0.0) && (locals.var_guard406 != 0.0)) {
        (locals.var_vdsegmt, locals.var_vdsegmt_dn0, locals.var_vdsegmt_dn2,)
    } else {
        (locals.var_vdserev, locals.var_vdserev_dn0, locals.var_vdserev_dn2,)
    }
};
        locals.var_vdserev = assign20060_e14844;
        locals.var_vdserev_dn0 = assign20060_e14844_d_n0;
        locals.var_vdserev_dn2 = assign20060_e14844_d_n2;
        locals.var_vdserev_rv = 0.0;

        let (assign20070_e14850, assign20070_e14850_d_n0, assign20070_e14850_d_n2,) = {
    if ((locals.var_guard405 != 0.0) && (locals.var_guard406 != 0.0)) {
        (locals.var_vsubs, 0.0, 0.0,)
    } else {
        (locals.var_vsubsrev, locals.var_vsubsrev_dn0, locals.var_vsubsrev_dn2,)
    }
};
        locals.var_vsubsrev = assign20070_e14850;
        locals.var_vsubsrev_dn0 = assign20070_e14850_d_n0;
        locals.var_vsubsrev_dn2 = assign20070_e14850_d_n2;
        locals.var_vsubsrev_rv = 0.0;

        let (assign20080_e14858, assign20080_e14858_d_n0, assign20080_e14858_d_n2,) = {
    if ((locals.var_guard405 != 0.0) && (locals.var_guard406 == 0.0)) {
        let assign20080_e14856: f64 = (-locals.var_vdsegmt);
        (assign20080_e14856, (-locals.var_vdsegmt_dn0), (-locals.var_vdsegmt_dn2),)
    } else {
        (locals.var_vdserev, locals.var_vdserev_dn0, locals.var_vdserev_dn2,)
    }
};
        locals.var_vdserev = assign20080_e14858;
        locals.var_vdserev_dn0 = assign20080_e14858_d_n0;
        locals.var_vdserev_dn2 = assign20080_e14858_d_n2;
        locals.var_vdserev_rv = 0.0;

        let (assign20090_e14867, assign20090_e14867_d_n0, assign20090_e14867_d_n2,) = {
    if ((locals.var_guard405 != 0.0) && (locals.var_guard406 == 0.0)) {
        let assign20090_e14865: f64 = (locals.var_vsubs - locals.var_vdsegmt);
        (assign20090_e14865, (-locals.var_vdsegmt_dn0), (-locals.var_vdsegmt_dn2),)
    } else {
        (locals.var_vsubsrev, locals.var_vsubsrev_dn0, locals.var_vsubsrev_dn2,)
    }
};
        locals.var_vsubsrev = assign20090_e14867;
        locals.var_vsubsrev_dn0 = assign20090_e14867_d_n0;
        locals.var_vsubsrev_dn2 = assign20090_e14867_d_n2;
        locals.var_vsubsrev_rv = 0.0;

        let (assign20100_e14877, assign20100_e14877_d_n0, assign20100_e14877_d_n2, assign20100_e14877_d_n4, assign20100_e14877_d_n5, assign20100_e14877_d_n6, assign20100_e14877_d_n7, assign20100_e14877_d_n8, assign20100_e14877_d_n9, assign20100_e14877_d_n10, assign20100_e14877_d_n13,) = {
    if (locals.var_guard405 != 0.0) {
        let assign20100_e14872: f64 = (locals.var_vdserev / 2.0);
        let assign20100_e14873: f64 = (2.0 * assign20100_e14872);
        let assign20100_e14875: f64 = (assign20100_e14873 / p.p262);
        (assign20100_e14875, ((2.0 * (locals.var_vdserev_dn0 / 2.0)) / p.p262), ((2.0 * (locals.var_vdserev_dn2 / 2.0)) / p.p262), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign20100_e14877;
        locals.var_tmf1_dn0 = assign20100_e14877_d_n0;
        locals.var_tmf1_dn2 = assign20100_e14877_d_n2;
        locals.var_tmf1_dn4 = assign20100_e14877_d_n4;
        locals.var_tmf1_dn5 = assign20100_e14877_d_n5;
        locals.var_tmf1_dn6 = assign20100_e14877_d_n6;
        locals.var_tmf1_dn7 = assign20100_e14877_d_n7;
        locals.var_tmf1_dn8 = assign20100_e14877_d_n8;
        locals.var_tmf1_dn9 = assign20100_e14877_d_n9;
        locals.var_tmf1_dn10 = assign20100_e14877_d_n10;
        locals.var_tmf1_dn13 = assign20100_e14877_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign20110_e14917, assign20110_e14917_d_n0, assign20110_e14917_d_n2, assign20110_e14917_d_n4, assign20110_e14917_d_n5, assign20110_e14917_d_n6, assign20110_e14917_d_n7, assign20110_e14917_d_n8, assign20110_e14917_d_n9, assign20110_e14917_d_n10, assign20110_e14917_d_n13,) = {
    if (locals.var_guard405 != 0.0) {
        let assign20110_e14883: f64 = (1.0 / 2.0);
        let assign20110_e14887: f64 = (1.0 / 6.0);
        let assign20110_e14891: f64 = (1.0 / 24.0);
        let assign20110_e14895: f64 = (1.0 / 120.0);
        let assign20110_e14899: f64 = (1.0 / 720.0);
        let assign20110_e14903: f64 = (1.0 / 5040.0);
        let assign20110_e14904: f64 = (locals.var_tmf1 * assign20110_e14903);
        let assign20110_e14905: f64 = (assign20110_e14899 + assign20110_e14904);
        let assign20110_e14906: f64 = (locals.var_tmf1 * assign20110_e14905);
        let assign20110_e14907: f64 = (assign20110_e14895 + assign20110_e14906);
        let assign20110_e14908: f64 = (locals.var_tmf1 * assign20110_e14907);
        let assign20110_e14909: f64 = (assign20110_e14891 + assign20110_e14908);
        let assign20110_e14910: f64 = (locals.var_tmf1 * assign20110_e14909);
        let assign20110_e14911: f64 = (assign20110_e14887 + assign20110_e14910);
        let assign20110_e14912: f64 = (locals.var_tmf1 * assign20110_e14911);
        let assign20110_e14913: f64 = (assign20110_e14883 + assign20110_e14912);
        let assign20110_e14914: f64 = (locals.var_tmf1 * assign20110_e14913);
        let assign20110_e14915: f64 = (1.0 + assign20110_e14914);
        (assign20110_e14915, ((locals.var_tmf1_dn0 * assign20110_e14913) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20110_e14911) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20110_e14909) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20110_e14907) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20110_e14905) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign20110_e14903))))))))))), ((locals.var_tmf1_dn2 * assign20110_e14913) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20110_e14911) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20110_e14909) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20110_e14907) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20110_e14905) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign20110_e14903))))))))))), ((locals.var_tmf1_dn4 * assign20110_e14913) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20110_e14911) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20110_e14909) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20110_e14907) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20110_e14905) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign20110_e14903))))))))))), ((locals.var_tmf1_dn5 * assign20110_e14913) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20110_e14911) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20110_e14909) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20110_e14907) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20110_e14905) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign20110_e14903))))))))))), ((locals.var_tmf1_dn6 * assign20110_e14913) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20110_e14911) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20110_e14909) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20110_e14907) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20110_e14905) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign20110_e14903))))))))))), ((locals.var_tmf1_dn7 * assign20110_e14913) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20110_e14911) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20110_e14909) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20110_e14907) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20110_e14905) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign20110_e14903))))))))))), ((locals.var_tmf1_dn8 * assign20110_e14913) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20110_e14911) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20110_e14909) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20110_e14907) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20110_e14905) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign20110_e14903))))))))))), ((locals.var_tmf1_dn9 * assign20110_e14913) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20110_e14911) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20110_e14909) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20110_e14907) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20110_e14905) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign20110_e14903))))))))))), ((locals.var_tmf1_dn10 * assign20110_e14913) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20110_e14911) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20110_e14909) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20110_e14907) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20110_e14905) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign20110_e14903))))))))))), ((locals.var_tmf1_dn13 * assign20110_e14913) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign20110_e14911) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign20110_e14909) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign20110_e14907) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign20110_e14905) + (locals.var_tmf1 * (locals.var_tmf1_dn13 * assign20110_e14903))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign20110_e14917;
        locals.var_tmf2_dn0 = assign20110_e14917_d_n0;
        locals.var_tmf2_dn2 = assign20110_e14917_d_n2;
        locals.var_tmf2_dn4 = assign20110_e14917_d_n4;
        locals.var_tmf2_dn5 = assign20110_e14917_d_n5;
        locals.var_tmf2_dn6 = assign20110_e14917_d_n6;
        locals.var_tmf2_dn7 = assign20110_e14917_d_n7;
        locals.var_tmf2_dn8 = assign20110_e14917_d_n8;
        locals.var_tmf2_dn9 = assign20110_e14917_d_n9;
        locals.var_tmf2_dn10 = assign20110_e14917_d_n10;
        locals.var_tmf2_dn13 = assign20110_e14917_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign20120_e14953, assign20120_e14953_d_n0, assign20120_e14953_d_n2, assign20120_e14953_d_n4, assign20120_e14953_d_n5, assign20120_e14953_d_n6, assign20120_e14953_d_n7, assign20120_e14953_d_n8, assign20120_e14953_d_n9, assign20120_e14953_d_n10, assign20120_e14953_d_n13,) = {
    if (locals.var_guard405 != 0.0) {
        let assign20120_e14921: f64 = (1.0 / 2.0);
        let assign20120_e14925: f64 = (1.0 / 3.0);
        let assign20120_e14929: f64 = (1.0 / 8.0);
        let assign20120_e14933: f64 = (1.0 / 30.0);
        let assign20120_e14937: f64 = (1.0 / 144.0);
        let assign20120_e14941: f64 = (1.0 / 840.0);
        let assign20120_e14942: f64 = (locals.var_tmf1 * assign20120_e14941);
        let assign20120_e14943: f64 = (assign20120_e14937 + assign20120_e14942);
        let assign20120_e14944: f64 = (locals.var_tmf1 * assign20120_e14943);
        let assign20120_e14945: f64 = (assign20120_e14933 + assign20120_e14944);
        let assign20120_e14946: f64 = (locals.var_tmf1 * assign20120_e14945);
        let assign20120_e14947: f64 = (assign20120_e14929 + assign20120_e14946);
        let assign20120_e14948: f64 = (locals.var_tmf1 * assign20120_e14947);
        let assign20120_e14949: f64 = (assign20120_e14925 + assign20120_e14948);
        let assign20120_e14950: f64 = (locals.var_tmf1 * assign20120_e14949);
        let assign20120_e14951: f64 = (assign20120_e14921 + assign20120_e14950);
        (assign20120_e14951, ((locals.var_tmf1_dn0 * assign20120_e14949) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20120_e14947) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20120_e14945) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20120_e14943) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign20120_e14941))))))))), ((locals.var_tmf1_dn2 * assign20120_e14949) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20120_e14947) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20120_e14945) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20120_e14943) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign20120_e14941))))))))), ((locals.var_tmf1_dn4 * assign20120_e14949) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20120_e14947) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20120_e14945) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20120_e14943) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign20120_e14941))))))))), ((locals.var_tmf1_dn5 * assign20120_e14949) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20120_e14947) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20120_e14945) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20120_e14943) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign20120_e14941))))))))), ((locals.var_tmf1_dn6 * assign20120_e14949) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20120_e14947) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20120_e14945) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20120_e14943) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign20120_e14941))))))))), ((locals.var_tmf1_dn7 * assign20120_e14949) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20120_e14947) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20120_e14945) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20120_e14943) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign20120_e14941))))))))), ((locals.var_tmf1_dn8 * assign20120_e14949) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20120_e14947) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20120_e14945) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20120_e14943) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign20120_e14941))))))))), ((locals.var_tmf1_dn9 * assign20120_e14949) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20120_e14947) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20120_e14945) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20120_e14943) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign20120_e14941))))))))), ((locals.var_tmf1_dn10 * assign20120_e14949) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20120_e14947) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20120_e14945) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20120_e14943) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign20120_e14941))))))))), ((locals.var_tmf1_dn13 * assign20120_e14949) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign20120_e14947) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign20120_e14945) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign20120_e14943) + (locals.var_tmf1 * (locals.var_tmf1_dn13 * assign20120_e14941))))))))),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn13,)
    }
};
        locals.var_tmf3 = assign20120_e14953;
        locals.var_tmf3_dn0 = assign20120_e14953_d_n0;
        locals.var_tmf3_dn2 = assign20120_e14953_d_n2;
        locals.var_tmf3_dn4 = assign20120_e14953_d_n4;
        locals.var_tmf3_dn5 = assign20120_e14953_d_n5;
        locals.var_tmf3_dn6 = assign20120_e14953_d_n6;
        locals.var_tmf3_dn7 = assign20120_e14953_d_n7;
        locals.var_tmf3_dn8 = assign20120_e14953_d_n8;
        locals.var_tmf3_dn9 = assign20120_e14953_d_n9;
        locals.var_tmf3_dn10 = assign20120_e14953_d_n10;
        locals.var_tmf3_dn13 = assign20120_e14953_d_n13;
        locals.var_tmf3_rv = 0.0;

        let (assign20130_e14959, assign20130_e14959_d_n0, assign20130_e14959_d_n2, assign20130_e14959_d_n4, assign20130_e14959_d_n5, assign20130_e14959_d_n6, assign20130_e14959_d_n7, assign20130_e14959_d_n8, assign20130_e14959_d_n9, assign20130_e14959_d_n10, assign20130_e14959_d_n13,) = {
    if (locals.var_guard405 != 0.0) {
        let assign20130_e14957: f64 = (p.p262 / locals.var_tmf2);
        (assign20130_e14957, (-((p.p262 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn13,)
    }
};
        locals.var_vzadd = assign20130_e14959;
        locals.var_vzadd_dn0 = assign20130_e14959_d_n0;
        locals.var_vzadd_dn2 = assign20130_e14959_d_n2;
        locals.var_vzadd_dn4 = assign20130_e14959_d_n4;
        locals.var_vzadd_dn5 = assign20130_e14959_d_n5;
        locals.var_vzadd_dn6 = assign20130_e14959_d_n6;
        locals.var_vzadd_dn7 = assign20130_e14959_d_n7;
        locals.var_vzadd_dn8 = assign20130_e14959_d_n8;
        locals.var_vzadd_dn9 = assign20130_e14959_d_n9;
        locals.var_vzadd_dn10 = assign20130_e14959_d_n10;
        locals.var_vzadd_dn13 = assign20130_e14959_d_n13;
        locals.var_vzadd_rv = 0.0;

        let (assign20140_e14970, assign20140_e14970_d_n0, assign20140_e14970_d_n2, assign20140_e14970_d_n4, assign20140_e14970_d_n5, assign20140_e14970_d_n6, assign20140_e14970_d_n7, assign20140_e14970_d_n8, assign20140_e14970_d_n9, assign20140_e14970_d_n10, assign20140_e14970_d_n13,) = {
    if (locals.var_guard405 != 0.0) {
        let assign20140_e14962: f64 = (-2.0);
        let assign20140_e14964: f64 = (assign20140_e14962 * locals.var_tmf3);
        let assign20140_e14967: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign20140_e14968: f64 = (assign20140_e14964 / assign20140_e14967);
        (assign20140_e14968, ((((assign20140_e14962 * locals.var_tmf3_dn0) * assign20140_e14967) - (assign20140_e14964 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign20140_e14967 * assign20140_e14967)), ((((assign20140_e14962 * locals.var_tmf3_dn2) * assign20140_e14967) - (assign20140_e14964 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign20140_e14967 * assign20140_e14967)), ((((assign20140_e14962 * locals.var_tmf3_dn4) * assign20140_e14967) - (assign20140_e14964 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign20140_e14967 * assign20140_e14967)), ((((assign20140_e14962 * locals.var_tmf3_dn5) * assign20140_e14967) - (assign20140_e14964 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign20140_e14967 * assign20140_e14967)), ((((assign20140_e14962 * locals.var_tmf3_dn6) * assign20140_e14967) - (assign20140_e14964 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign20140_e14967 * assign20140_e14967)), ((((assign20140_e14962 * locals.var_tmf3_dn7) * assign20140_e14967) - (assign20140_e14964 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign20140_e14967 * assign20140_e14967)), ((((assign20140_e14962 * locals.var_tmf3_dn8) * assign20140_e14967) - (assign20140_e14964 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign20140_e14967 * assign20140_e14967)), ((((assign20140_e14962 * locals.var_tmf3_dn9) * assign20140_e14967) - (assign20140_e14964 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign20140_e14967 * assign20140_e14967)), ((((assign20140_e14962 * locals.var_tmf3_dn10) * assign20140_e14967) - (assign20140_e14964 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign20140_e14967 * assign20140_e14967)), ((((assign20140_e14962 * locals.var_tmf3_dn13) * assign20140_e14967) - (assign20140_e14964 * ((locals.var_tmf2_dn13 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn13)))) / (assign20140_e14967 * assign20140_e14967)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign20140_e14970;
        locals.var_t2_dn0 = assign20140_e14970_d_n0;
        locals.var_t2_dn2 = assign20140_e14970_d_n2;
        locals.var_t2_dn4 = assign20140_e14970_d_n4;
        locals.var_t2_dn5 = assign20140_e14970_d_n5;
        locals.var_t2_dn6 = assign20140_e14970_d_n6;
        locals.var_t2_dn7 = assign20140_e14970_d_n7;
        locals.var_t2_dn8 = assign20140_e14970_d_n8;
        locals.var_t2_dn9 = assign20140_e14970_d_n9;
        locals.var_t2_dn10 = assign20140_e14970_d_n10;
        locals.var_t2_dn13 = assign20140_e14970_d_n13;
        locals.var_t2_rv = 0.0;

        let assign20150_e14973: f64 = if locals.var_vzadd < 1e-12 { 1.0 } else { 0.0 };
        locals.var_guard407 = assign20150_e14973;
        locals.var_guard407_rv = 0.0;

        let (assign20160_e14979, assign20160_e14979_d_n0, assign20160_e14979_d_n2, assign20160_e14979_d_n4, assign20160_e14979_d_n5, assign20160_e14979_d_n6, assign20160_e14979_d_n7, assign20160_e14979_d_n8, assign20160_e14979_d_n9, assign20160_e14979_d_n10, assign20160_e14979_d_n13,) = {
    if ((locals.var_guard405 != 0.0) && (locals.var_guard407 != 0.0)) {
        (1e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn13,)
    }
};
        locals.var_vzadd = assign20160_e14979;
        locals.var_vzadd_dn0 = assign20160_e14979_d_n0;
        locals.var_vzadd_dn2 = assign20160_e14979_d_n2;
        locals.var_vzadd_dn4 = assign20160_e14979_d_n4;
        locals.var_vzadd_dn5 = assign20160_e14979_d_n5;
        locals.var_vzadd_dn6 = assign20160_e14979_d_n6;
        locals.var_vzadd_dn7 = assign20160_e14979_d_n7;
        locals.var_vzadd_dn8 = assign20160_e14979_d_n8;
        locals.var_vzadd_dn9 = assign20160_e14979_d_n9;
        locals.var_vzadd_dn10 = assign20160_e14979_d_n10;
        locals.var_vzadd_dn13 = assign20160_e14979_d_n13;
        locals.var_vzadd_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_49(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20170_e14987, assign20170_e14987_d_n0, assign20170_e14987_d_n2, assign20170_e14987_d_n4, assign20170_e14987_d_n5, assign20170_e14987_d_n6, assign20170_e14987_d_n7, assign20170_e14987_d_n8, assign20170_e14987_d_n9, assign20170_e14987_d_n10, assign20170_e14987_d_n13,) = {
    if (locals.var_guard405 != 0.0) {
        let assign20170_e14984: f64 = (2.0 * locals.var_vzadd);
        let assign20170_e14985: f64 = (locals.var_vdserev + assign20170_e14984);
        (assign20170_e14985, (locals.var_vdserev_dn0 + (2.0 * locals.var_vzadd_dn0)), (locals.var_vdserev_dn2 + (2.0 * locals.var_vzadd_dn2)), (2.0 * locals.var_vzadd_dn4), (2.0 * locals.var_vzadd_dn5), (2.0 * locals.var_vzadd_dn6), (2.0 * locals.var_vzadd_dn7), (2.0 * locals.var_vzadd_dn8), (2.0 * locals.var_vzadd_dn9), (2.0 * locals.var_vzadd_dn10), (2.0 * locals.var_vzadd_dn13),)
    } else {
        (locals.var_vdserevz, locals.var_vdserevz_dn0, locals.var_vdserevz_dn2, locals.var_vdserevz_dn4, locals.var_vdserevz_dn5, locals.var_vdserevz_dn6, locals.var_vdserevz_dn7, locals.var_vdserevz_dn8, locals.var_vdserevz_dn9, locals.var_vdserevz_dn10, locals.var_vdserevz_dn13,)
    }
};
        locals.var_vdserevz = assign20170_e14987;
        locals.var_vdserevz_dn0 = assign20170_e14987_d_n0;
        locals.var_vdserevz_dn2 = assign20170_e14987_d_n2;
        locals.var_vdserevz_dn4 = assign20170_e14987_d_n4;
        locals.var_vdserevz_dn5 = assign20170_e14987_d_n5;
        locals.var_vdserevz_dn6 = assign20170_e14987_d_n6;
        locals.var_vdserevz_dn7 = assign20170_e14987_d_n7;
        locals.var_vdserevz_dn8 = assign20170_e14987_d_n8;
        locals.var_vdserevz_dn9 = assign20170_e14987_d_n9;
        locals.var_vdserevz_dn10 = assign20170_e14987_d_n10;
        locals.var_vdserevz_dn13 = assign20170_e14987_d_n13;
        locals.var_vdserevz_rv = 0.0;

        let (assign20180_e14999, assign20180_e14999_d_n0, assign20180_e14999_d_n2, assign20180_e14999_d_n4, assign20180_e14999_d_n5, assign20180_e14999_d_n6, assign20180_e14999_d_n7, assign20180_e14999_d_n8, assign20180_e14999_d_n9, assign20180_e14999_d_n10, assign20180_e14999_d_n13,) = {
    if (locals.var_guard405 != 0.0) {
        let assign20180_e14992: f64 = (p.p333 * locals.var_vdserevz);
        let assign20180_e14993: f64 = (p.p335 - assign20180_e14992);
        let assign20180_e14996: f64 = (p.p332 * locals.var_vsubsrev);
        let assign20180_e14997: f64 = (assign20180_e14993 - assign20180_e14996);
        (assign20180_e14997, ((-(p.p333 * locals.var_vdserevz_dn0)) - (p.p332 * locals.var_vsubsrev_dn0)), ((-(p.p333 * locals.var_vdserevz_dn2)) - (p.p332 * locals.var_vsubsrev_dn2)), (-(p.p333 * locals.var_vdserevz_dn4)), (-(p.p333 * locals.var_vdserevz_dn5)), (-(p.p333 * locals.var_vdserevz_dn6)), (-(p.p333 * locals.var_vdserevz_dn7)), (-(p.p333 * locals.var_vdserevz_dn8)), (-(p.p333 * locals.var_vdserevz_dn9)), (-(p.p333 * locals.var_vdserevz_dn10)), (-(p.p333 * locals.var_vdserevz_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign20180_e14999;
        locals.var_t0_dn0 = assign20180_e14999_d_n0;
        locals.var_t0_dn2 = assign20180_e14999_d_n2;
        locals.var_t0_dn4 = assign20180_e14999_d_n4;
        locals.var_t0_dn5 = assign20180_e14999_d_n5;
        locals.var_t0_dn6 = assign20180_e14999_d_n6;
        locals.var_t0_dn7 = assign20180_e14999_d_n7;
        locals.var_t0_dn8 = assign20180_e14999_d_n8;
        locals.var_t0_dn9 = assign20180_e14999_d_n9;
        locals.var_t0_dn10 = assign20180_e14999_d_n10;
        locals.var_t0_dn13 = assign20180_e14999_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign20190_e15012, assign20190_e15012_d_n0, assign20190_e15012_d_n2, assign20190_e15012_d_n4, assign20190_e15012_d_n5, assign20190_e15012_d_n6, assign20190_e15012_d_n7, assign20190_e15012_d_n8, assign20190_e15012_d_n9, assign20190_e15012_d_n10, assign20190_e15012_d_n13,) = {
    if (locals.var_guard405 != 0.0) {
        let assign20190_e15003: f64 = (locals.var_t0 * locals.var_t0);
        let assign20190_e15006: f64 = (4.0 * 10.0);
        let assign20190_e15008: f64 = (assign20190_e15006 * 10.0);
        let assign20190_e15009: f64 = (assign20190_e15003 + assign20190_e15008);
        let assign20190_e15010: f64 = (assign20190_e15009).sqrt();
        (assign20190_e15010, (((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)) / (2.0 * assign20190_e15010)), (((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)) / (2.0 * assign20190_e15010)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign20190_e15010)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign20190_e15010)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign20190_e15010)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign20190_e15010)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign20190_e15010)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign20190_e15010)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign20190_e15010)), (((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)) / (2.0 * assign20190_e15010)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign20190_e15012;
        locals.var_tmf2_dn0 = assign20190_e15012_d_n0;
        locals.var_tmf2_dn2 = assign20190_e15012_d_n2;
        locals.var_tmf2_dn4 = assign20190_e15012_d_n4;
        locals.var_tmf2_dn5 = assign20190_e15012_d_n5;
        locals.var_tmf2_dn6 = assign20190_e15012_d_n6;
        locals.var_tmf2_dn7 = assign20190_e15012_d_n7;
        locals.var_tmf2_dn8 = assign20190_e15012_d_n8;
        locals.var_tmf2_dn9 = assign20190_e15012_d_n9;
        locals.var_tmf2_dn10 = assign20190_e15012_d_n10;
        locals.var_tmf2_dn13 = assign20190_e15012_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign20200_e15022, assign20200_e15022_d_n0, assign20200_e15022_d_n2, assign20200_e15022_d_n4, assign20200_e15022_d_n5, assign20200_e15022_d_n6, assign20200_e15022_d_n7, assign20200_e15022_d_n8, assign20200_e15022_d_n9, assign20200_e15022_d_n10, assign20200_e15022_d_n13,) = {
    if (locals.var_guard405 != 0.0) {
        let assign20200_e15018: f64 = (locals.var_t0 / locals.var_tmf2);
        let assign20200_e15019: f64 = (1.0 + assign20200_e15018);
        let assign20200_e15020: f64 = (0.5 * assign20200_e15019);
        (assign20200_e15020, (0.5 * (((locals.var_t0_dn0 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn2 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn4 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn5 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn6 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn7 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn8 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn9 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn10 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn13 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign20200_e15022;
        locals.var_t2_dn0 = assign20200_e15022_d_n0;
        locals.var_t2_dn2 = assign20200_e15022_d_n2;
        locals.var_t2_dn4 = assign20200_e15022_d_n4;
        locals.var_t2_dn5 = assign20200_e15022_d_n5;
        locals.var_t2_dn6 = assign20200_e15022_d_n6;
        locals.var_t2_dn7 = assign20200_e15022_d_n7;
        locals.var_t2_dn8 = assign20200_e15022_d_n8;
        locals.var_t2_dn9 = assign20200_e15022_d_n9;
        locals.var_t2_dn10 = assign20200_e15022_d_n10;
        locals.var_t2_dn13 = assign20200_e15022_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign20210_e15030, assign20210_e15030_d_n0, assign20210_e15030_d_n2, assign20210_e15030_d_n4, assign20210_e15030_d_n5, assign20210_e15030_d_n6, assign20210_e15030_d_n7, assign20210_e15030_d_n8, assign20210_e15030_d_n9, assign20210_e15030_d_n10, assign20210_e15030_d_n13,) = {
    if (locals.var_guard405 != 0.0) {
        let assign20210_e15027: f64 = (locals.var_t0 + locals.var_tmf2);
        let assign20210_e15028: f64 = (0.5 * assign20210_e15027);
        (assign20210_e15028, (0.5 * (locals.var_t0_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t0_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t0_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t0_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t0_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t0_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t0_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t0_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t0_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t0_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign20210_e15030;
        locals.var_t1_dn0 = assign20210_e15030_d_n0;
        locals.var_t1_dn2 = assign20210_e15030_d_n2;
        locals.var_t1_dn4 = assign20210_e15030_d_n4;
        locals.var_t1_dn5 = assign20210_e15030_d_n5;
        locals.var_t1_dn6 = assign20210_e15030_d_n6;
        locals.var_t1_dn7 = assign20210_e15030_d_n7;
        locals.var_t1_dn8 = assign20210_e15030_d_n8;
        locals.var_t1_dn9 = assign20210_e15030_d_n9;
        locals.var_t1_dn10 = assign20210_e15030_d_n10;
        locals.var_t1_dn13 = assign20210_e15030_d_n13;
        locals.var_t1_rv = 0.0;

        let assign20220_e15033: f64 = if locals.var_t1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard408 = assign20220_e15033;
        locals.var_guard408_rv = 0.0;

        let (assign20230_e15039, assign20230_e15039_d_n0, assign20230_e15039_d_n2, assign20230_e15039_d_n4, assign20230_e15039_d_n5, assign20230_e15039_d_n6, assign20230_e15039_d_n7, assign20230_e15039_d_n8, assign20230_e15039_d_n9, assign20230_e15039_d_n10, assign20230_e15039_d_n13,) = {
    if ((locals.var_guard405 != 0.0) && (locals.var_guard408 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign20230_e15039;
        locals.var_t1_dn0 = assign20230_e15039_d_n0;
        locals.var_t1_dn2 = assign20230_e15039_d_n2;
        locals.var_t1_dn4 = assign20230_e15039_d_n4;
        locals.var_t1_dn5 = assign20230_e15039_d_n5;
        locals.var_t1_dn6 = assign20230_e15039_d_n6;
        locals.var_t1_dn7 = assign20230_e15039_d_n7;
        locals.var_t1_dn8 = assign20230_e15039_d_n8;
        locals.var_t1_dn9 = assign20230_e15039_d_n9;
        locals.var_t1_dn10 = assign20230_e15039_d_n10;
        locals.var_t1_dn13 = assign20230_e15039_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign20240_e15045, assign20240_e15045_d_n0, assign20240_e15045_d_n2, assign20240_e15045_d_n4, assign20240_e15045_d_n5, assign20240_e15045_d_n6, assign20240_e15045_d_n7, assign20240_e15045_d_n8, assign20240_e15045_d_n9, assign20240_e15045_d_n10, assign20240_e15045_d_n13,) = {
    if ((locals.var_guard405 != 0.0) && (locals.var_guard408 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign20240_e15045;
        locals.var_t2_dn0 = assign20240_e15045_d_n0;
        locals.var_t2_dn2 = assign20240_e15045_d_n2;
        locals.var_t2_dn4 = assign20240_e15045_d_n4;
        locals.var_t2_dn5 = assign20240_e15045_d_n5;
        locals.var_t2_dn6 = assign20240_e15045_d_n6;
        locals.var_t2_dn7 = assign20240_e15045_d_n7;
        locals.var_t2_dn8 = assign20240_e15045_d_n8;
        locals.var_t2_dn9 = assign20240_e15045_d_n9;
        locals.var_t2_dn10 = assign20240_e15045_d_n10;
        locals.var_t2_dn13 = assign20240_e15045_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign20250_e15053, assign20250_e15053_d_n0, assign20250_e15053_d_n2, assign20250_e15053_d_n4, assign20250_e15053_d_n5, assign20250_e15053_d_n6, assign20250_e15053_d_n7, assign20250_e15053_d_n8, assign20250_e15053_d_n9, assign20250_e15053_d_n10, assign20250_e15053_d_n13,) = {
    if (locals.var_guard405 != 0.0) {
        let assign20250_e15050: f64 = (10.0 * 2.220446049250313e-16);
        let assign20250_e15051: f64 = (locals.var_t1 + assign20250_e15050);
        (assign20250_e15051, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign20250_e15053;
        locals.var_t1_dn0 = assign20250_e15053_d_n0;
        locals.var_t1_dn2 = assign20250_e15053_d_n2;
        locals.var_t1_dn4 = assign20250_e15053_d_n4;
        locals.var_t1_dn5 = assign20250_e15053_d_n5;
        locals.var_t1_dn6 = assign20250_e15053_d_n6;
        locals.var_t1_dn7 = assign20250_e15053_d_n7;
        locals.var_t1_dn8 = assign20250_e15053_d_n8;
        locals.var_t1_dn9 = assign20250_e15053_d_n9;
        locals.var_t1_dn10 = assign20250_e15053_d_n10;
        locals.var_t1_dn13 = assign20250_e15053_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign20260_e15063, assign20260_e15063_d_n0, assign20260_e15063_d_n2, assign20260_e15063_d_n4, assign20260_e15063_d_n5, assign20260_e15063_d_n6, assign20260_e15063_d_n7, assign20260_e15063_d_n8, assign20260_e15063_d_n9, assign20260_e15063_d_n10, assign20260_e15063_d_n13,) = {
    if (locals.var_guard405 != 0.0) {
        let assign20260_e15059: f64 = (locals.var_mks_nsubsub + locals.var_uc_nover);
        let assign20260_e15060: f64 = (locals.var_uc_nover * assign20260_e15059);
        let assign20260_e15061: f64 = (locals.var_mks_nsubsub / assign20260_e15060);
        (assign20260_e15061, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign20260_e15063;
        locals.var_t0_dn0 = assign20260_e15063_d_n0;
        locals.var_t0_dn2 = assign20260_e15063_d_n2;
        locals.var_t0_dn4 = assign20260_e15063_d_n4;
        locals.var_t0_dn5 = assign20260_e15063_d_n5;
        locals.var_t0_dn6 = assign20260_e15063_d_n6;
        locals.var_t0_dn7 = assign20260_e15063_d_n7;
        locals.var_t0_dn8 = assign20260_e15063_d_n8;
        locals.var_t0_dn9 = assign20260_e15063_d_n9;
        locals.var_t0_dn10 = assign20260_e15063_d_n10;
        locals.var_t0_dn13 = assign20260_e15063_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign20270_e15073, assign20270_e15073_d_n0, assign20270_e15073_d_n2, assign20270_e15073_d_n4, assign20270_e15073_d_n5, assign20270_e15073_d_n6, assign20270_e15073_d_n7, assign20270_e15073_d_n8, assign20270_e15073_d_n9, assign20270_e15073_d_n10, assign20270_e15073_d_n13,) = {
    if (locals.var_guard405 != 0.0) {
        let assign20270_e15067: f64 = (2.0 * 1.034943e-10);
        let assign20270_e15069: f64 = (assign20270_e15067 / 1.6021918e-19);
        let assign20270_e15071: f64 = (assign20270_e15069 * locals.var_t0);
        (assign20270_e15071, (assign20270_e15069 * locals.var_t0_dn0), (assign20270_e15069 * locals.var_t0_dn2), (assign20270_e15069 * locals.var_t0_dn4), (assign20270_e15069 * locals.var_t0_dn5), (assign20270_e15069 * locals.var_t0_dn6), (assign20270_e15069 * locals.var_t0_dn7), (assign20270_e15069 * locals.var_t0_dn8), (assign20270_e15069 * locals.var_t0_dn9), (assign20270_e15069 * locals.var_t0_dn10), (assign20270_e15069 * locals.var_t0_dn13),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign20270_e15073;
        locals.var_t4_dn0 = assign20270_e15073_d_n0;
        locals.var_t4_dn2 = assign20270_e15073_d_n2;
        locals.var_t4_dn4 = assign20270_e15073_d_n4;
        locals.var_t4_dn5 = assign20270_e15073_d_n5;
        locals.var_t4_dn6 = assign20270_e15073_d_n6;
        locals.var_t4_dn7 = assign20270_e15073_d_n7;
        locals.var_t4_dn8 = assign20270_e15073_d_n8;
        locals.var_t4_dn9 = assign20270_e15073_d_n9;
        locals.var_t4_dn10 = assign20270_e15073_d_n10;
        locals.var_t4_dn13 = assign20270_e15073_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign20280_e15082, assign20280_e15082_d_n0, assign20280_e15082_d_n2, assign20280_e15082_d_n4, assign20280_e15082_d_n5, assign20280_e15082_d_n6, assign20280_e15082_d_n7, assign20280_e15082_d_n8, assign20280_e15082_d_n9, assign20280_e15082_d_n10, assign20280_e15082_d_n13,) = {
    if (locals.var_guard405 != 0.0) {
        let assign20280_e15077: f64 = (locals.var_t4 * locals.var_t1);
        let assign20280_e15078: f64 = (assign20280_e15077).sqrt();
        let assign20280_e15080: f64 = (assign20280_e15078 + 1e-25);
        (assign20280_e15080, (((locals.var_t4_dn0 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn0)) / (2.0 * assign20280_e15078)), (((locals.var_t4_dn2 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn2)) / (2.0 * assign20280_e15078)), (((locals.var_t4_dn4 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn4)) / (2.0 * assign20280_e15078)), (((locals.var_t4_dn5 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn5)) / (2.0 * assign20280_e15078)), (((locals.var_t4_dn6 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn6)) / (2.0 * assign20280_e15078)), (((locals.var_t4_dn7 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn7)) / (2.0 * assign20280_e15078)), (((locals.var_t4_dn8 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn8)) / (2.0 * assign20280_e15078)), (((locals.var_t4_dn9 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn9)) / (2.0 * assign20280_e15078)), (((locals.var_t4_dn10 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn10)) / (2.0 * assign20280_e15078)), (((locals.var_t4_dn13 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn13)) / (2.0 * assign20280_e15078)),)
    } else {
        (locals.var_wdep, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn4, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, locals.var_wdep_dn9, locals.var_wdep_dn10, locals.var_wdep_dn13,)
    }
};
        locals.var_wdep = assign20280_e15082;
        locals.var_wdep_dn0 = assign20280_e15082_d_n0;
        locals.var_wdep_dn2 = assign20280_e15082_d_n2;
        locals.var_wdep_dn4 = assign20280_e15082_d_n4;
        locals.var_wdep_dn5 = assign20280_e15082_d_n5;
        locals.var_wdep_dn6 = assign20280_e15082_d_n6;
        locals.var_wdep_dn7 = assign20280_e15082_d_n7;
        locals.var_wdep_dn8 = assign20280_e15082_d_n8;
        locals.var_wdep_dn9 = assign20280_e15082_d_n9;
        locals.var_wdep_dn10 = assign20280_e15082_d_n10;
        locals.var_wdep_dn13 = assign20280_e15082_d_n13;
        locals.var_wdep_rv = 0.0;

        let (assign20290_e15092, assign20290_e15092_d_n0, assign20290_e15092_d_n2, assign20290_e15092_d_n4, assign20290_e15092_d_n5, assign20290_e15092_d_n6, assign20290_e15092_d_n7, assign20290_e15092_d_n8, assign20290_e15092_d_n9, assign20290_e15092_d_n10, assign20290_e15092_d_n13,) = {
    if (locals.var_guard405 != 0.0) {
        let assign20290_e15086: f64 = (p.p334 - locals.var_wdep);
        let assign20290_e15089: f64 = (0.1 * p.p334);
        let assign20290_e15090: f64 = (assign20290_e15086 - assign20290_e15089);
        (assign20290_e15090, (-locals.var_wdep_dn0), (-locals.var_wdep_dn2), (-locals.var_wdep_dn4), (-locals.var_wdep_dn5), (-locals.var_wdep_dn6), (-locals.var_wdep_dn7), (-locals.var_wdep_dn8), (-locals.var_wdep_dn9), (-locals.var_wdep_dn10), (-locals.var_wdep_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign20290_e15092;
        locals.var_tmf1_dn0 = assign20290_e15092_d_n0;
        locals.var_tmf1_dn2 = assign20290_e15092_d_n2;
        locals.var_tmf1_dn4 = assign20290_e15092_d_n4;
        locals.var_tmf1_dn5 = assign20290_e15092_d_n5;
        locals.var_tmf1_dn6 = assign20290_e15092_d_n6;
        locals.var_tmf1_dn7 = assign20290_e15092_d_n7;
        locals.var_tmf1_dn8 = assign20290_e15092_d_n8;
        locals.var_tmf1_dn9 = assign20290_e15092_d_n9;
        locals.var_tmf1_dn10 = assign20290_e15092_d_n10;
        locals.var_tmf1_dn13 = assign20290_e15092_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign20300_e15102, assign20300_e15102_d_n0, assign20300_e15102_d_n2, assign20300_e15102_d_n4, assign20300_e15102_d_n5, assign20300_e15102_d_n6, assign20300_e15102_d_n7, assign20300_e15102_d_n8, assign20300_e15102_d_n9, assign20300_e15102_d_n10, assign20300_e15102_d_n13,) = {
    if (locals.var_guard405 != 0.0) {
        let assign20300_e15096: f64 = (4.0 * p.p334);
        let assign20300_e15099: f64 = (0.1 * p.p334);
        let assign20300_e15100: f64 = (assign20300_e15096 * assign20300_e15099);
        (assign20300_e15100, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign20300_e15102;
        locals.var_tmf2_dn0 = assign20300_e15102_d_n0;
        locals.var_tmf2_dn2 = assign20300_e15102_d_n2;
        locals.var_tmf2_dn4 = assign20300_e15102_d_n4;
        locals.var_tmf2_dn5 = assign20300_e15102_d_n5;
        locals.var_tmf2_dn6 = assign20300_e15102_d_n6;
        locals.var_tmf2_dn7 = assign20300_e15102_d_n7;
        locals.var_tmf2_dn8 = assign20300_e15102_d_n8;
        locals.var_tmf2_dn9 = assign20300_e15102_d_n9;
        locals.var_tmf2_dn10 = assign20300_e15102_d_n10;
        locals.var_tmf2_dn13 = assign20300_e15102_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign20310_e15112, assign20310_e15112_d_n0, assign20310_e15112_d_n2, assign20310_e15112_d_n4, assign20310_e15112_d_n5, assign20310_e15112_d_n6, assign20310_e15112_d_n7, assign20310_e15112_d_n8, assign20310_e15112_d_n9, assign20310_e15112_d_n10, assign20310_e15112_d_n13,) = {
    if (locals.var_guard405 != 0.0) {
        let (assign20310_e15110, assign20310_e15110_d_n0, assign20310_e15110_d_n2, assign20310_e15110_d_n4, assign20310_e15110_d_n5, assign20310_e15110_d_n6, assign20310_e15110_d_n7, assign20310_e15110_d_n8, assign20310_e15110_d_n9, assign20310_e15110_d_n10, assign20310_e15110_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign20310_e15109: f64 = (-locals.var_tmf2);
                (assign20310_e15109, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign20310_e15110, assign20310_e15110_d_n0, assign20310_e15110_d_n2, assign20310_e15110_d_n4, assign20310_e15110_d_n5, assign20310_e15110_d_n6, assign20310_e15110_d_n7, assign20310_e15110_d_n8, assign20310_e15110_d_n9, assign20310_e15110_d_n10, assign20310_e15110_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign20310_e15112;
        locals.var_tmf2_dn0 = assign20310_e15112_d_n0;
        locals.var_tmf2_dn2 = assign20310_e15112_d_n2;
        locals.var_tmf2_dn4 = assign20310_e15112_d_n4;
        locals.var_tmf2_dn5 = assign20310_e15112_d_n5;
        locals.var_tmf2_dn6 = assign20310_e15112_d_n6;
        locals.var_tmf2_dn7 = assign20310_e15112_d_n7;
        locals.var_tmf2_dn8 = assign20310_e15112_d_n8;
        locals.var_tmf2_dn9 = assign20310_e15112_d_n9;
        locals.var_tmf2_dn10 = assign20310_e15112_d_n10;
        locals.var_tmf2_dn13 = assign20310_e15112_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign20320_e15121, assign20320_e15121_d_n0, assign20320_e15121_d_n2, assign20320_e15121_d_n4, assign20320_e15121_d_n5, assign20320_e15121_d_n6, assign20320_e15121_d_n7, assign20320_e15121_d_n8, assign20320_e15121_d_n9, assign20320_e15121_d_n10, assign20320_e15121_d_n13,) = {
    if (locals.var_guard405 != 0.0) {
        let assign20320_e15116: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign20320_e15118: f64 = (assign20320_e15116 + locals.var_tmf2);
        let assign20320_e15119: f64 = (assign20320_e15118).sqrt();
        (assign20320_e15119, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign20320_e15119)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign20320_e15119)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign20320_e15119)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign20320_e15119)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign20320_e15119)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign20320_e15119)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign20320_e15119)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign20320_e15119)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign20320_e15119)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign20320_e15119)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign20320_e15121;
        locals.var_tmf2_dn0 = assign20320_e15121_d_n0;
        locals.var_tmf2_dn2 = assign20320_e15121_d_n2;
        locals.var_tmf2_dn4 = assign20320_e15121_d_n4;
        locals.var_tmf2_dn5 = assign20320_e15121_d_n5;
        locals.var_tmf2_dn6 = assign20320_e15121_d_n6;
        locals.var_tmf2_dn7 = assign20320_e15121_d_n7;
        locals.var_tmf2_dn8 = assign20320_e15121_d_n8;
        locals.var_tmf2_dn9 = assign20320_e15121_d_n9;
        locals.var_tmf2_dn10 = assign20320_e15121_d_n10;
        locals.var_tmf2_dn13 = assign20320_e15121_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign20330_e15131, assign20330_e15131_d_n0, assign20330_e15131_d_n2, assign20330_e15131_d_n4, assign20330_e15131_d_n5, assign20330_e15131_d_n6, assign20330_e15131_d_n7, assign20330_e15131_d_n8, assign20330_e15131_d_n9, assign20330_e15131_d_n10, assign20330_e15131_d_n13,) = {
    if (locals.var_guard405 != 0.0) {
        let assign20330_e15127: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign20330_e15128: f64 = (1.0 + assign20330_e15127);
        let assign20330_e15129: f64 = (0.5 * assign20330_e15128);
        (assign20330_e15129, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign20330_e15131;
        locals.var_t0_dn0 = assign20330_e15131_d_n0;
        locals.var_t0_dn2 = assign20330_e15131_d_n2;
        locals.var_t0_dn4 = assign20330_e15131_d_n4;
        locals.var_t0_dn5 = assign20330_e15131_d_n5;
        locals.var_t0_dn6 = assign20330_e15131_d_n6;
        locals.var_t0_dn7 = assign20330_e15131_d_n7;
        locals.var_t0_dn8 = assign20330_e15131_d_n8;
        locals.var_t0_dn9 = assign20330_e15131_d_n9;
        locals.var_t0_dn10 = assign20330_e15131_d_n10;
        locals.var_t0_dn13 = assign20330_e15131_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign20340_e15141, assign20340_e15141_d_n0, assign20340_e15141_d_n2, assign20340_e15141_d_n4, assign20340_e15141_d_n5, assign20340_e15141_d_n6, assign20340_e15141_d_n7, assign20340_e15141_d_n8, assign20340_e15141_d_n9, assign20340_e15141_d_n10, assign20340_e15141_d_n13,) = {
    if (locals.var_guard405 != 0.0) {
        let assign20340_e15137: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign20340_e15138: f64 = (0.5 * assign20340_e15137);
        let assign20340_e15139: f64 = (p.p334 - assign20340_e15138);
        (assign20340_e15139, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_wdep, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn4, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, locals.var_wdep_dn9, locals.var_wdep_dn10, locals.var_wdep_dn13,)
    }
};
        locals.var_wdep = assign20340_e15141;
        locals.var_wdep_dn0 = assign20340_e15141_d_n0;
        locals.var_wdep_dn2 = assign20340_e15141_d_n2;
        locals.var_wdep_dn4 = assign20340_e15141_d_n4;
        locals.var_wdep_dn5 = assign20340_e15141_d_n5;
        locals.var_wdep_dn6 = assign20340_e15141_d_n6;
        locals.var_wdep_dn7 = assign20340_e15141_d_n7;
        locals.var_wdep_dn8 = assign20340_e15141_d_n8;
        locals.var_wdep_dn9 = assign20340_e15141_d_n9;
        locals.var_wdep_dn10 = assign20340_e15141_d_n10;
        locals.var_wdep_dn13 = assign20340_e15141_d_n13;
        locals.var_wdep_rv = 0.0;

        let (assign20350_e15146, assign20350_e15146_d_n0, assign20350_e15146_d_n2, assign20350_e15146_d_n4, assign20350_e15146_d_n5, assign20350_e15146_d_n6, assign20350_e15146_d_n7, assign20350_e15146_d_n8, assign20350_e15146_d_n9, assign20350_e15146_d_n10, assign20350_e15146_d_n13,) = {
    if (locals.var_guard405 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_wdep, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn4, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, locals.var_wdep_dn9, locals.var_wdep_dn10, locals.var_wdep_dn13,)
    }
};
        locals.var_wdep = assign20350_e15146;
        locals.var_wdep_dn0 = assign20350_e15146_d_n0;
        locals.var_wdep_dn2 = assign20350_e15146_d_n2;
        locals.var_wdep_dn4 = assign20350_e15146_d_n4;
        locals.var_wdep_dn5 = assign20350_e15146_d_n5;
        locals.var_wdep_dn6 = assign20350_e15146_d_n6;
        locals.var_wdep_dn7 = assign20350_e15146_d_n7;
        locals.var_wdep_dn8 = assign20350_e15146_d_n8;
        locals.var_wdep_dn9 = assign20350_e15146_d_n9;
        locals.var_wdep_dn10 = assign20350_e15146_d_n10;
        locals.var_wdep_dn13 = assign20350_e15146_d_n13;
        locals.var_wdep_rv = 0.0;

        let assign20360_e15153: f64 = if ((locals.var_flg_rsrd == 1.0) || (locals.var_flg_rsrd == 3.0)) { 1.0 } else { 0.0 };
        locals.var_guard409 = assign20360_e15153;
        locals.var_guard409_rv = 0.0;

        let (assign20370_e15157, assign20370_e15157_d_n0, assign20370_e15157_d_n2,) = {
    if (locals.var_guard409 != 0.0) {
        (locals.var_vdsei, locals.var_vdsei_dn0, locals.var_vdsei_dn2,)
    } else {
        (locals.var_vdsegmt, locals.var_vdsegmt_dn0, locals.var_vdsegmt_dn2,)
    }
};
        locals.var_vdsegmt = assign20370_e15157;
        locals.var_vdsegmt_dn0 = assign20370_e15157_d_n0;
        locals.var_vdsegmt_dn2 = assign20370_e15157_d_n2;
        locals.var_vdsegmt_rv = 0.0;

        let (assign20380_e15161, assign20380_e15161_d_n2, assign20380_e15161_d_n6,) = {
    if (locals.var_guard409 != 0.0) {
        (locals.var_vgsei, locals.var_vgsei_dn2, locals.var_vgsei_dn6,)
    } else {
        (locals.var_vgsegmt, locals.var_vgsegmt_dn2, locals.var_vgsegmt_dn6,)
    }
};
        locals.var_vgsegmt = assign20380_e15161;
        locals.var_vgsegmt_dn2 = assign20380_e15161_d_n2;
        locals.var_vgsegmt_dn6 = assign20380_e15161_d_n6;
        locals.var_vgsegmt_rv = 0.0;

        let (assign20390_e15165, assign20390_e15165_d_n2, assign20390_e15165_d_n8,) = {
    if (locals.var_guard409 != 0.0) {
        (locals.var_vbsei, locals.var_vbsei_dn2, locals.var_vbsei_dn8,)
    } else {
        (locals.var_vbsegmt, locals.var_vbsegmt_dn2, locals.var_vbsegmt_dn8,)
    }
};
        locals.var_vbsegmt = assign20390_e15165;
        locals.var_vbsegmt_dn2 = assign20390_e15165_d_n2;
        locals.var_vbsegmt_dn8 = assign20390_e15165_d_n8;
        locals.var_vbsegmt_rv = 0.0;

        let assign20400_e15168: f64 = if locals.var_vdsegmt >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard410 = assign20400_e15168;
        locals.var_guard410_rv = 0.0;

        let (assign20410_e15174,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard410 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_vdsemodenml,)
    }
};
        locals.var_vdsemodenml = assign20410_e15174;
        locals.var_vdsemodenml_rv = 0.0;

        let (assign20420_e15180,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard410 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_vdsemodervs,)
    }
};
        locals.var_vdsemodervs = assign20420_e15180;
        locals.var_vdsemodervs_rv = 0.0;

        let (assign20430_e15186, assign20430_e15186_d_n0, assign20430_e15186_d_n2,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard410 != 0.0)) {
        (locals.var_vdsegmt, locals.var_vdsegmt_dn0, locals.var_vdsegmt_dn2,)
    } else {
        (locals.var_vdserev, locals.var_vdserev_dn0, locals.var_vdserev_dn2,)
    }
};
        locals.var_vdserev = assign20430_e15186;
        locals.var_vdserev_dn0 = assign20430_e15186_d_n0;
        locals.var_vdserev_dn2 = assign20430_e15186_d_n2;
        locals.var_vdserev_rv = 0.0;

        let (assign20440_e15192, assign20440_e15192_d_n0, assign20440_e15192_d_n2, assign20440_e15192_d_n6,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard410 != 0.0)) {
        (locals.var_vgsegmt, 0.0, locals.var_vgsegmt_dn2, locals.var_vgsegmt_dn6,)
    } else {
        (locals.var_vgserev, locals.var_vgserev_dn0, locals.var_vgserev_dn2, locals.var_vgserev_dn6,)
    }
};
        locals.var_vgserev = assign20440_e15192;
        locals.var_vgserev_dn0 = assign20440_e15192_d_n0;
        locals.var_vgserev_dn2 = assign20440_e15192_d_n2;
        locals.var_vgserev_dn6 = assign20440_e15192_d_n6;
        locals.var_vgserev_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_50(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20450_e15198, assign20450_e15198_d_n0, assign20450_e15198_d_n2, assign20450_e15198_d_n8,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard410 != 0.0)) {
        (locals.var_vbsegmt, 0.0, locals.var_vbsegmt_dn2, locals.var_vbsegmt_dn8,)
    } else {
        (locals.var_vbserev, locals.var_vbserev_dn0, locals.var_vbserev_dn2, locals.var_vbserev_dn8,)
    }
};
        locals.var_vbserev = assign20450_e15198;
        locals.var_vbserev_dn0 = assign20450_e15198_d_n0;
        locals.var_vbserev_dn2 = assign20450_e15198_d_n2;
        locals.var_vbserev_dn8 = assign20450_e15198_d_n8;
        locals.var_vbserev_rv = 0.0;

        let (assign20460_e15204, assign20460_e15204_d_n0, assign20460_e15204_d_n2,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard410 != 0.0)) {
        (locals.var_vsubs, 0.0, 0.0,)
    } else {
        (locals.var_vsubsrev, locals.var_vsubsrev_dn0, locals.var_vsubsrev_dn2,)
    }
};
        locals.var_vsubsrev = assign20460_e15204;
        locals.var_vsubsrev_dn0 = assign20460_e15204_d_n0;
        locals.var_vsubsrev_dn2 = assign20460_e15204_d_n2;
        locals.var_vsubsrev_rv = 0.0;

        let (assign20470_e15211,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard410 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_vdsemodenml,)
    }
};
        locals.var_vdsemodenml = assign20470_e15211;
        locals.var_vdsemodenml_rv = 0.0;

        let (assign20480_e15218,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard410 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_vdsemodervs,)
    }
};
        locals.var_vdsemodervs = assign20480_e15218;
        locals.var_vdsemodervs_rv = 0.0;

        let (assign20490_e15226, assign20490_e15226_d_n0, assign20490_e15226_d_n2,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard410 == 0.0)) {
        let assign20490_e15224: f64 = (-locals.var_vdsegmt);
        (assign20490_e15224, (-locals.var_vdsegmt_dn0), (-locals.var_vdsegmt_dn2),)
    } else {
        (locals.var_vdserev, locals.var_vdserev_dn0, locals.var_vdserev_dn2,)
    }
};
        locals.var_vdserev = assign20490_e15226;
        locals.var_vdserev_dn0 = assign20490_e15226_d_n0;
        locals.var_vdserev_dn2 = assign20490_e15226_d_n2;
        locals.var_vdserev_rv = 0.0;

        let (assign20500_e15235, assign20500_e15235_d_n0, assign20500_e15235_d_n2, assign20500_e15235_d_n6,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard410 == 0.0)) {
        let assign20500_e15233: f64 = (locals.var_vgsegmt - locals.var_vdsegmt);
        (assign20500_e15233, (-locals.var_vdsegmt_dn0), (locals.var_vgsegmt_dn2 - locals.var_vdsegmt_dn2), locals.var_vgsegmt_dn6,)
    } else {
        (locals.var_vgserev, locals.var_vgserev_dn0, locals.var_vgserev_dn2, locals.var_vgserev_dn6,)
    }
};
        locals.var_vgserev = assign20500_e15235;
        locals.var_vgserev_dn0 = assign20500_e15235_d_n0;
        locals.var_vgserev_dn2 = assign20500_e15235_d_n2;
        locals.var_vgserev_dn6 = assign20500_e15235_d_n6;
        locals.var_vgserev_rv = 0.0;

        let (assign20510_e15244, assign20510_e15244_d_n0, assign20510_e15244_d_n2, assign20510_e15244_d_n8,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard410 == 0.0)) {
        let assign20510_e15242: f64 = (locals.var_vbsegmt - locals.var_vdsegmt);
        (assign20510_e15242, (-locals.var_vdsegmt_dn0), (locals.var_vbsegmt_dn2 - locals.var_vdsegmt_dn2), locals.var_vbsegmt_dn8,)
    } else {
        (locals.var_vbserev, locals.var_vbserev_dn0, locals.var_vbserev_dn2, locals.var_vbserev_dn8,)
    }
};
        locals.var_vbserev = assign20510_e15244;
        locals.var_vbserev_dn0 = assign20510_e15244_d_n0;
        locals.var_vbserev_dn2 = assign20510_e15244_d_n2;
        locals.var_vbserev_dn8 = assign20510_e15244_d_n8;
        locals.var_vbserev_rv = 0.0;

        let (assign20520_e15253, assign20520_e15253_d_n0, assign20520_e15253_d_n2,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard410 == 0.0)) {
        let assign20520_e15251: f64 = (locals.var_vsubs - locals.var_vdsegmt);
        (assign20520_e15251, (-locals.var_vdsegmt_dn0), (-locals.var_vdsegmt_dn2),)
    } else {
        (locals.var_vsubsrev, locals.var_vsubsrev_dn0, locals.var_vsubsrev_dn2,)
    }
};
        locals.var_vsubsrev = assign20520_e15253;
        locals.var_vsubsrev_dn0 = assign20520_e15253_d_n0;
        locals.var_vsubsrev_dn2 = assign20520_e15253_d_n2;
        locals.var_vsubsrev_rv = 0.0;

        let assign20530_e15272: f64 = if (((((locals.var_rdvde > 0.0) || (locals.var_rsvde > 0.0)) || (locals.var_uc_rdvg11 > 0.0)) || (locals.var_uc_rdvb > 0.0)) || (p.p54 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard411 = assign20530_e15272;
        locals.var_guard411_rv = 0.0;

        let (assign20540_e15284, assign20540_e15284_d_n0, assign20540_e15284_d_n2, assign20540_e15284_d_n4, assign20540_e15284_d_n5, assign20540_e15284_d_n6, assign20540_e15284_d_n7, assign20540_e15284_d_n8, assign20540_e15284_d_n9, assign20540_e15284_d_n10, assign20540_e15284_d_n13,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) {
        let assign20540_e15279: f64 = (locals.var_vdserev / 2.0);
        let assign20540_e15280: f64 = (2.0 * assign20540_e15279);
        let assign20540_e15282: f64 = (assign20540_e15280 / p.p262);
        (assign20540_e15282, ((2.0 * (locals.var_vdserev_dn0 / 2.0)) / p.p262), ((2.0 * (locals.var_vdserev_dn2 / 2.0)) / p.p262), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign20540_e15284;
        locals.var_tmf1_dn0 = assign20540_e15284_d_n0;
        locals.var_tmf1_dn2 = assign20540_e15284_d_n2;
        locals.var_tmf1_dn4 = assign20540_e15284_d_n4;
        locals.var_tmf1_dn5 = assign20540_e15284_d_n5;
        locals.var_tmf1_dn6 = assign20540_e15284_d_n6;
        locals.var_tmf1_dn7 = assign20540_e15284_d_n7;
        locals.var_tmf1_dn8 = assign20540_e15284_d_n8;
        locals.var_tmf1_dn9 = assign20540_e15284_d_n9;
        locals.var_tmf1_dn10 = assign20540_e15284_d_n10;
        locals.var_tmf1_dn13 = assign20540_e15284_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign20550_e15326, assign20550_e15326_d_n0, assign20550_e15326_d_n2, assign20550_e15326_d_n4, assign20550_e15326_d_n5, assign20550_e15326_d_n6, assign20550_e15326_d_n7, assign20550_e15326_d_n8, assign20550_e15326_d_n9, assign20550_e15326_d_n10, assign20550_e15326_d_n13,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) {
        let assign20550_e15292: f64 = (1.0 / 2.0);
        let assign20550_e15296: f64 = (1.0 / 6.0);
        let assign20550_e15300: f64 = (1.0 / 24.0);
        let assign20550_e15304: f64 = (1.0 / 120.0);
        let assign20550_e15308: f64 = (1.0 / 720.0);
        let assign20550_e15312: f64 = (1.0 / 5040.0);
        let assign20550_e15313: f64 = (locals.var_tmf1 * assign20550_e15312);
        let assign20550_e15314: f64 = (assign20550_e15308 + assign20550_e15313);
        let assign20550_e15315: f64 = (locals.var_tmf1 * assign20550_e15314);
        let assign20550_e15316: f64 = (assign20550_e15304 + assign20550_e15315);
        let assign20550_e15317: f64 = (locals.var_tmf1 * assign20550_e15316);
        let assign20550_e15318: f64 = (assign20550_e15300 + assign20550_e15317);
        let assign20550_e15319: f64 = (locals.var_tmf1 * assign20550_e15318);
        let assign20550_e15320: f64 = (assign20550_e15296 + assign20550_e15319);
        let assign20550_e15321: f64 = (locals.var_tmf1 * assign20550_e15320);
        let assign20550_e15322: f64 = (assign20550_e15292 + assign20550_e15321);
        let assign20550_e15323: f64 = (locals.var_tmf1 * assign20550_e15322);
        let assign20550_e15324: f64 = (1.0 + assign20550_e15323);
        (assign20550_e15324, ((locals.var_tmf1_dn0 * assign20550_e15322) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20550_e15320) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20550_e15318) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20550_e15316) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20550_e15314) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign20550_e15312))))))))))), ((locals.var_tmf1_dn2 * assign20550_e15322) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20550_e15320) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20550_e15318) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20550_e15316) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20550_e15314) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign20550_e15312))))))))))), ((locals.var_tmf1_dn4 * assign20550_e15322) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20550_e15320) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20550_e15318) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20550_e15316) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20550_e15314) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign20550_e15312))))))))))), ((locals.var_tmf1_dn5 * assign20550_e15322) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20550_e15320) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20550_e15318) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20550_e15316) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20550_e15314) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign20550_e15312))))))))))), ((locals.var_tmf1_dn6 * assign20550_e15322) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20550_e15320) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20550_e15318) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20550_e15316) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20550_e15314) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign20550_e15312))))))))))), ((locals.var_tmf1_dn7 * assign20550_e15322) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20550_e15320) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20550_e15318) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20550_e15316) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20550_e15314) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign20550_e15312))))))))))), ((locals.var_tmf1_dn8 * assign20550_e15322) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20550_e15320) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20550_e15318) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20550_e15316) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20550_e15314) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign20550_e15312))))))))))), ((locals.var_tmf1_dn9 * assign20550_e15322) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20550_e15320) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20550_e15318) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20550_e15316) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20550_e15314) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign20550_e15312))))))))))), ((locals.var_tmf1_dn10 * assign20550_e15322) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20550_e15320) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20550_e15318) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20550_e15316) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20550_e15314) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign20550_e15312))))))))))), ((locals.var_tmf1_dn13 * assign20550_e15322) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign20550_e15320) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign20550_e15318) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign20550_e15316) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign20550_e15314) + (locals.var_tmf1 * (locals.var_tmf1_dn13 * assign20550_e15312))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign20550_e15326;
        locals.var_tmf2_dn0 = assign20550_e15326_d_n0;
        locals.var_tmf2_dn2 = assign20550_e15326_d_n2;
        locals.var_tmf2_dn4 = assign20550_e15326_d_n4;
        locals.var_tmf2_dn5 = assign20550_e15326_d_n5;
        locals.var_tmf2_dn6 = assign20550_e15326_d_n6;
        locals.var_tmf2_dn7 = assign20550_e15326_d_n7;
        locals.var_tmf2_dn8 = assign20550_e15326_d_n8;
        locals.var_tmf2_dn9 = assign20550_e15326_d_n9;
        locals.var_tmf2_dn10 = assign20550_e15326_d_n10;
        locals.var_tmf2_dn13 = assign20550_e15326_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign20560_e15364, assign20560_e15364_d_n0, assign20560_e15364_d_n2, assign20560_e15364_d_n4, assign20560_e15364_d_n5, assign20560_e15364_d_n6, assign20560_e15364_d_n7, assign20560_e15364_d_n8, assign20560_e15364_d_n9, assign20560_e15364_d_n10, assign20560_e15364_d_n13,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) {
        let assign20560_e15332: f64 = (1.0 / 2.0);
        let assign20560_e15336: f64 = (1.0 / 3.0);
        let assign20560_e15340: f64 = (1.0 / 8.0);
        let assign20560_e15344: f64 = (1.0 / 30.0);
        let assign20560_e15348: f64 = (1.0 / 144.0);
        let assign20560_e15352: f64 = (1.0 / 840.0);
        let assign20560_e15353: f64 = (locals.var_tmf1 * assign20560_e15352);
        let assign20560_e15354: f64 = (assign20560_e15348 + assign20560_e15353);
        let assign20560_e15355: f64 = (locals.var_tmf1 * assign20560_e15354);
        let assign20560_e15356: f64 = (assign20560_e15344 + assign20560_e15355);
        let assign20560_e15357: f64 = (locals.var_tmf1 * assign20560_e15356);
        let assign20560_e15358: f64 = (assign20560_e15340 + assign20560_e15357);
        let assign20560_e15359: f64 = (locals.var_tmf1 * assign20560_e15358);
        let assign20560_e15360: f64 = (assign20560_e15336 + assign20560_e15359);
        let assign20560_e15361: f64 = (locals.var_tmf1 * assign20560_e15360);
        let assign20560_e15362: f64 = (assign20560_e15332 + assign20560_e15361);
        (assign20560_e15362, ((locals.var_tmf1_dn0 * assign20560_e15360) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20560_e15358) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20560_e15356) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20560_e15354) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign20560_e15352))))))))), ((locals.var_tmf1_dn2 * assign20560_e15360) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20560_e15358) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20560_e15356) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20560_e15354) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign20560_e15352))))))))), ((locals.var_tmf1_dn4 * assign20560_e15360) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20560_e15358) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20560_e15356) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20560_e15354) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign20560_e15352))))))))), ((locals.var_tmf1_dn5 * assign20560_e15360) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20560_e15358) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20560_e15356) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20560_e15354) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign20560_e15352))))))))), ((locals.var_tmf1_dn6 * assign20560_e15360) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20560_e15358) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20560_e15356) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20560_e15354) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign20560_e15352))))))))), ((locals.var_tmf1_dn7 * assign20560_e15360) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20560_e15358) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20560_e15356) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20560_e15354) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign20560_e15352))))))))), ((locals.var_tmf1_dn8 * assign20560_e15360) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20560_e15358) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20560_e15356) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20560_e15354) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign20560_e15352))))))))), ((locals.var_tmf1_dn9 * assign20560_e15360) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20560_e15358) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20560_e15356) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20560_e15354) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign20560_e15352))))))))), ((locals.var_tmf1_dn10 * assign20560_e15360) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20560_e15358) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20560_e15356) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20560_e15354) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign20560_e15352))))))))), ((locals.var_tmf1_dn13 * assign20560_e15360) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign20560_e15358) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign20560_e15356) + (locals.var_tmf1 * ((locals.var_tmf1_dn13 * assign20560_e15354) + (locals.var_tmf1 * (locals.var_tmf1_dn13 * assign20560_e15352))))))))),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn13,)
    }
};
        locals.var_tmf3 = assign20560_e15364;
        locals.var_tmf3_dn0 = assign20560_e15364_d_n0;
        locals.var_tmf3_dn2 = assign20560_e15364_d_n2;
        locals.var_tmf3_dn4 = assign20560_e15364_d_n4;
        locals.var_tmf3_dn5 = assign20560_e15364_d_n5;
        locals.var_tmf3_dn6 = assign20560_e15364_d_n6;
        locals.var_tmf3_dn7 = assign20560_e15364_d_n7;
        locals.var_tmf3_dn8 = assign20560_e15364_d_n8;
        locals.var_tmf3_dn9 = assign20560_e15364_d_n9;
        locals.var_tmf3_dn10 = assign20560_e15364_d_n10;
        locals.var_tmf3_dn13 = assign20560_e15364_d_n13;
        locals.var_tmf3_rv = 0.0;

        let (assign20570_e15372, assign20570_e15372_d_n0, assign20570_e15372_d_n2, assign20570_e15372_d_n4, assign20570_e15372_d_n5, assign20570_e15372_d_n6, assign20570_e15372_d_n7, assign20570_e15372_d_n8, assign20570_e15372_d_n9, assign20570_e15372_d_n10, assign20570_e15372_d_n13,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) {
        let assign20570_e15370: f64 = (p.p262 / locals.var_tmf2);
        (assign20570_e15370, (-((p.p262 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn13,)
    }
};
        locals.var_vzadd = assign20570_e15372;
        locals.var_vzadd_dn0 = assign20570_e15372_d_n0;
        locals.var_vzadd_dn2 = assign20570_e15372_d_n2;
        locals.var_vzadd_dn4 = assign20570_e15372_d_n4;
        locals.var_vzadd_dn5 = assign20570_e15372_d_n5;
        locals.var_vzadd_dn6 = assign20570_e15372_d_n6;
        locals.var_vzadd_dn7 = assign20570_e15372_d_n7;
        locals.var_vzadd_dn8 = assign20570_e15372_d_n8;
        locals.var_vzadd_dn9 = assign20570_e15372_d_n9;
        locals.var_vzadd_dn10 = assign20570_e15372_d_n10;
        locals.var_vzadd_dn13 = assign20570_e15372_d_n13;
        locals.var_vzadd_rv = 0.0;

        let (assign20580_e15385, assign20580_e15385_d_n0, assign20580_e15385_d_n2, assign20580_e15385_d_n4, assign20580_e15385_d_n5, assign20580_e15385_d_n6, assign20580_e15385_d_n7, assign20580_e15385_d_n8, assign20580_e15385_d_n9, assign20580_e15385_d_n10, assign20580_e15385_d_n13,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) {
        let assign20580_e15377: f64 = (-2.0);
        let assign20580_e15379: f64 = (assign20580_e15377 * locals.var_tmf3);
        let assign20580_e15382: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign20580_e15383: f64 = (assign20580_e15379 / assign20580_e15382);
        (assign20580_e15383, ((((assign20580_e15377 * locals.var_tmf3_dn0) * assign20580_e15382) - (assign20580_e15379 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign20580_e15382 * assign20580_e15382)), ((((assign20580_e15377 * locals.var_tmf3_dn2) * assign20580_e15382) - (assign20580_e15379 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign20580_e15382 * assign20580_e15382)), ((((assign20580_e15377 * locals.var_tmf3_dn4) * assign20580_e15382) - (assign20580_e15379 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign20580_e15382 * assign20580_e15382)), ((((assign20580_e15377 * locals.var_tmf3_dn5) * assign20580_e15382) - (assign20580_e15379 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign20580_e15382 * assign20580_e15382)), ((((assign20580_e15377 * locals.var_tmf3_dn6) * assign20580_e15382) - (assign20580_e15379 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign20580_e15382 * assign20580_e15382)), ((((assign20580_e15377 * locals.var_tmf3_dn7) * assign20580_e15382) - (assign20580_e15379 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign20580_e15382 * assign20580_e15382)), ((((assign20580_e15377 * locals.var_tmf3_dn8) * assign20580_e15382) - (assign20580_e15379 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign20580_e15382 * assign20580_e15382)), ((((assign20580_e15377 * locals.var_tmf3_dn9) * assign20580_e15382) - (assign20580_e15379 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign20580_e15382 * assign20580_e15382)), ((((assign20580_e15377 * locals.var_tmf3_dn10) * assign20580_e15382) - (assign20580_e15379 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign20580_e15382 * assign20580_e15382)), ((((assign20580_e15377 * locals.var_tmf3_dn13) * assign20580_e15382) - (assign20580_e15379 * ((locals.var_tmf2_dn13 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn13)))) / (assign20580_e15382 * assign20580_e15382)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign20580_e15385;
        locals.var_t2_dn0 = assign20580_e15385_d_n0;
        locals.var_t2_dn2 = assign20580_e15385_d_n2;
        locals.var_t2_dn4 = assign20580_e15385_d_n4;
        locals.var_t2_dn5 = assign20580_e15385_d_n5;
        locals.var_t2_dn6 = assign20580_e15385_d_n6;
        locals.var_t2_dn7 = assign20580_e15385_d_n7;
        locals.var_t2_dn8 = assign20580_e15385_d_n8;
        locals.var_t2_dn9 = assign20580_e15385_d_n9;
        locals.var_t2_dn10 = assign20580_e15385_d_n10;
        locals.var_t2_dn13 = assign20580_e15385_d_n13;
        locals.var_t2_rv = 0.0;

        let assign20590_e15388: f64 = if locals.var_vzadd < 1e-12 { 1.0 } else { 0.0 };
        locals.var_guard412 = assign20590_e15388;
        locals.var_guard412_rv = 0.0;

        let (assign20600_e15396, assign20600_e15396_d_n0, assign20600_e15396_d_n2, assign20600_e15396_d_n4, assign20600_e15396_d_n5, assign20600_e15396_d_n6, assign20600_e15396_d_n7, assign20600_e15396_d_n8, assign20600_e15396_d_n9, assign20600_e15396_d_n10, assign20600_e15396_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard412 != 0.0)) {
        (1e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn13,)
    }
};
        locals.var_vzadd = assign20600_e15396;
        locals.var_vzadd_dn0 = assign20600_e15396_d_n0;
        locals.var_vzadd_dn2 = assign20600_e15396_d_n2;
        locals.var_vzadd_dn4 = assign20600_e15396_d_n4;
        locals.var_vzadd_dn5 = assign20600_e15396_d_n5;
        locals.var_vzadd_dn6 = assign20600_e15396_d_n6;
        locals.var_vzadd_dn7 = assign20600_e15396_d_n7;
        locals.var_vzadd_dn8 = assign20600_e15396_d_n8;
        locals.var_vzadd_dn9 = assign20600_e15396_d_n9;
        locals.var_vzadd_dn10 = assign20600_e15396_d_n10;
        locals.var_vzadd_dn13 = assign20600_e15396_d_n13;
        locals.var_vzadd_rv = 0.0;

        let (assign20610_e15406, assign20610_e15406_d_n0, assign20610_e15406_d_n2, assign20610_e15406_d_n4, assign20610_e15406_d_n5, assign20610_e15406_d_n6, assign20610_e15406_d_n7, assign20610_e15406_d_n8, assign20610_e15406_d_n9, assign20610_e15406_d_n10, assign20610_e15406_d_n13,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) {
        let assign20610_e15403: f64 = (2.0 * locals.var_vzadd);
        let assign20610_e15404: f64 = (locals.var_vdserev + assign20610_e15403);
        (assign20610_e15404, (locals.var_vdserev_dn0 + (2.0 * locals.var_vzadd_dn0)), (locals.var_vdserev_dn2 + (2.0 * locals.var_vzadd_dn2)), (2.0 * locals.var_vzadd_dn4), (2.0 * locals.var_vzadd_dn5), (2.0 * locals.var_vzadd_dn6), (2.0 * locals.var_vzadd_dn7), (2.0 * locals.var_vzadd_dn8), (2.0 * locals.var_vzadd_dn9), (2.0 * locals.var_vzadd_dn10), (2.0 * locals.var_vzadd_dn13),)
    } else {
        (locals.var_vdserevz, locals.var_vdserevz_dn0, locals.var_vdserevz_dn2, locals.var_vdserevz_dn4, locals.var_vdserevz_dn5, locals.var_vdserevz_dn6, locals.var_vdserevz_dn7, locals.var_vdserevz_dn8, locals.var_vdserevz_dn9, locals.var_vdserevz_dn10, locals.var_vdserevz_dn13,)
    }
};
        locals.var_vdserevz = assign20610_e15406;
        locals.var_vdserevz_dn0 = assign20610_e15406_d_n0;
        locals.var_vdserevz_dn2 = assign20610_e15406_d_n2;
        locals.var_vdserevz_dn4 = assign20610_e15406_d_n4;
        locals.var_vdserevz_dn5 = assign20610_e15406_d_n5;
        locals.var_vdserevz_dn6 = assign20610_e15406_d_n6;
        locals.var_vdserevz_dn7 = assign20610_e15406_d_n7;
        locals.var_vdserevz_dn8 = assign20610_e15406_d_n8;
        locals.var_vdserevz_dn9 = assign20610_e15406_d_n9;
        locals.var_vdserevz_dn10 = assign20610_e15406_d_n10;
        locals.var_vdserevz_dn13 = assign20610_e15406_d_n13;
        locals.var_vdserevz_rv = 0.0;

        let (assign20620_e15414, assign20620_e15414_d_n0, assign20620_e15414_d_n2, assign20620_e15414_d_n4, assign20620_e15414_d_n5, assign20620_e15414_d_n6, assign20620_e15414_d_n7, assign20620_e15414_d_n8, assign20620_e15414_d_n9, assign20620_e15414_d_n10, assign20620_e15414_d_n13,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) {
        let assign20620_e15412: f64 = (locals.var_vgserev + locals.var_vzadd);
        (assign20620_e15412, (locals.var_vgserev_dn0 + locals.var_vzadd_dn0), (locals.var_vgserev_dn2 + locals.var_vzadd_dn2), locals.var_vzadd_dn4, locals.var_vzadd_dn5, (locals.var_vgserev_dn6 + locals.var_vzadd_dn6), locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn13,)
    } else {
        (locals.var_vgserevz, locals.var_vgserevz_dn0, locals.var_vgserevz_dn2, locals.var_vgserevz_dn4, locals.var_vgserevz_dn5, locals.var_vgserevz_dn6, locals.var_vgserevz_dn7, locals.var_vgserevz_dn8, locals.var_vgserevz_dn9, locals.var_vgserevz_dn10, locals.var_vgserevz_dn13,)
    }
};
        locals.var_vgserevz = assign20620_e15414;
        locals.var_vgserevz_dn0 = assign20620_e15414_d_n0;
        locals.var_vgserevz_dn2 = assign20620_e15414_d_n2;
        locals.var_vgserevz_dn4 = assign20620_e15414_d_n4;
        locals.var_vgserevz_dn5 = assign20620_e15414_d_n5;
        locals.var_vgserevz_dn6 = assign20620_e15414_d_n6;
        locals.var_vgserevz_dn7 = assign20620_e15414_d_n7;
        locals.var_vgserevz_dn8 = assign20620_e15414_d_n8;
        locals.var_vgserevz_dn9 = assign20620_e15414_d_n9;
        locals.var_vgserevz_dn10 = assign20620_e15414_d_n10;
        locals.var_vgserevz_dn13 = assign20620_e15414_d_n13;
        locals.var_vgserevz_rv = 0.0;

        let (assign20630_e15422, assign20630_e15422_d_n0, assign20630_e15422_d_n2, assign20630_e15422_d_n4, assign20630_e15422_d_n5, assign20630_e15422_d_n6, assign20630_e15422_d_n7, assign20630_e15422_d_n8, assign20630_e15422_d_n9, assign20630_e15422_d_n10, assign20630_e15422_d_n13,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) {
        let assign20630_e15420: f64 = (locals.var_vbserev + locals.var_vzadd);
        (assign20630_e15420, (locals.var_vbserev_dn0 + locals.var_vzadd_dn0), (locals.var_vbserev_dn2 + locals.var_vzadd_dn2), locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, (locals.var_vbserev_dn8 + locals.var_vzadd_dn8), locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn13,)
    } else {
        (locals.var_vbserevz, locals.var_vbserevz_dn0, locals.var_vbserevz_dn2, locals.var_vbserevz_dn4, locals.var_vbserevz_dn5, locals.var_vbserevz_dn6, locals.var_vbserevz_dn7, locals.var_vbserevz_dn8, locals.var_vbserevz_dn9, locals.var_vbserevz_dn10, locals.var_vbserevz_dn13,)
    }
};
        locals.var_vbserevz = assign20630_e15422;
        locals.var_vbserevz_dn0 = assign20630_e15422_d_n0;
        locals.var_vbserevz_dn2 = assign20630_e15422_d_n2;
        locals.var_vbserevz_dn4 = assign20630_e15422_d_n4;
        locals.var_vbserevz_dn5 = assign20630_e15422_d_n5;
        locals.var_vbserevz_dn6 = assign20630_e15422_d_n6;
        locals.var_vbserevz_dn7 = assign20630_e15422_d_n7;
        locals.var_vbserevz_dn8 = assign20630_e15422_d_n8;
        locals.var_vbserevz_dn9 = assign20630_e15422_d_n9;
        locals.var_vbserevz_dn10 = assign20630_e15422_d_n10;
        locals.var_vbserevz_dn13 = assign20630_e15422_d_n13;
        locals.var_vbserevz_rv = 0.0;

        let assign20640_e15429: f64 = if ((p.p34 == 1.0) || (locals.var_vdsemodenml == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard413 = assign20640_e15429;
        locals.var_guard413_rv = 0.0;

        let (assign20650_e15443, assign20650_e15443_d_n0, assign20650_e15443_d_n2, assign20650_e15443_d_n4, assign20650_e15443_d_n5, assign20650_e15443_d_n6, assign20650_e15443_d_n7, assign20650_e15443_d_n8, assign20650_e15443_d_n9, assign20650_e15443_d_n10, assign20650_e15443_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) {
        let assign20650_e15437: f64 = (locals.var_vdsemodenml * locals.var_rde);
        let assign20650_e15440: f64 = (locals.var_vdsemodervs * locals.var_rse);
        let assign20650_e15441: f64 = (assign20650_e15437 + assign20650_e15440);
        (assign20650_e15441, ((locals.var_vdsemodenml * locals.var_rde_dn0) + (locals.var_vdsemodervs * locals.var_rse_dn0)), ((locals.var_vdsemodenml * locals.var_rde_dn2) + (locals.var_vdsemodervs * locals.var_rse_dn2)), ((locals.var_vdsemodenml * locals.var_rde_dn4) + (locals.var_vdsemodervs * locals.var_rse_dn4)), ((locals.var_vdsemodenml * locals.var_rde_dn5) + (locals.var_vdsemodervs * locals.var_rse_dn5)), ((locals.var_vdsemodenml * locals.var_rde_dn6) + (locals.var_vdsemodervs * locals.var_rse_dn6)), ((locals.var_vdsemodenml * locals.var_rde_dn7) + (locals.var_vdsemodervs * locals.var_rse_dn7)), ((locals.var_vdsemodenml * locals.var_rde_dn8) + (locals.var_vdsemodervs * locals.var_rse_dn8)), ((locals.var_vdsemodenml * locals.var_rde_dn9) + (locals.var_vdsemodervs * locals.var_rse_dn9)), ((locals.var_vdsemodenml * locals.var_rde_dn10) + (locals.var_vdsemodervs * locals.var_rse_dn10)), ((locals.var_vdsemodenml * locals.var_rde_dn13) + (locals.var_vdsemodervs * locals.var_rse_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign20650_e15443;
        locals.var_t1_dn0 = assign20650_e15443_d_n0;
        locals.var_t1_dn2 = assign20650_e15443_d_n2;
        locals.var_t1_dn4 = assign20650_e15443_d_n4;
        locals.var_t1_dn5 = assign20650_e15443_d_n5;
        locals.var_t1_dn6 = assign20650_e15443_d_n6;
        locals.var_t1_dn7 = assign20650_e15443_d_n7;
        locals.var_t1_dn8 = assign20650_e15443_d_n8;
        locals.var_t1_dn9 = assign20650_e15443_d_n9;
        locals.var_t1_dn10 = assign20650_e15443_d_n10;
        locals.var_t1_dn13 = assign20650_e15443_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign20660_e15457, assign20660_e15457_d_n0, assign20660_e15457_d_n2, assign20660_e15457_d_n4, assign20660_e15457_d_n5, assign20660_e15457_d_n6, assign20660_e15457_d_n7, assign20660_e15457_d_n8, assign20660_e15457_d_n9, assign20660_e15457_d_n10, assign20660_e15457_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) {
        let assign20660_e15451: f64 = (locals.var_vdsemodenml * locals.var_rdvde);
        let assign20660_e15454: f64 = (locals.var_vdsemodervs * locals.var_rsvde);
        let assign20660_e15455: f64 = (assign20660_e15451 + assign20660_e15454);
        (assign20660_e15455, ((locals.var_vdsemodenml * locals.var_rdvde_dn0) + (locals.var_vdsemodervs * locals.var_rsvde_dn0)), ((locals.var_vdsemodenml * locals.var_rdvde_dn2) + (locals.var_vdsemodervs * locals.var_rsvde_dn2)), ((locals.var_vdsemodenml * locals.var_rdvde_dn4) + (locals.var_vdsemodervs * locals.var_rsvde_dn4)), ((locals.var_vdsemodenml * locals.var_rdvde_dn5) + (locals.var_vdsemodervs * locals.var_rsvde_dn5)), ((locals.var_vdsemodenml * locals.var_rdvde_dn6) + (locals.var_vdsemodervs * locals.var_rsvde_dn6)), ((locals.var_vdsemodenml * locals.var_rdvde_dn7) + (locals.var_vdsemodervs * locals.var_rsvde_dn7)), ((locals.var_vdsemodenml * locals.var_rdvde_dn8) + (locals.var_vdsemodervs * locals.var_rsvde_dn8)), ((locals.var_vdsemodenml * locals.var_rdvde_dn9) + (locals.var_vdsemodervs * locals.var_rsvde_dn9)), ((locals.var_vdsemodenml * locals.var_rdvde_dn10) + (locals.var_vdsemodervs * locals.var_rsvde_dn10)), ((locals.var_vdsemodenml * locals.var_rdvde_dn13) + (locals.var_vdsemodervs * locals.var_rsvde_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign20660_e15457;
        locals.var_t0_dn0 = assign20660_e15457_d_n0;
        locals.var_t0_dn2 = assign20660_e15457_d_n2;
        locals.var_t0_dn4 = assign20660_e15457_d_n4;
        locals.var_t0_dn5 = assign20660_e15457_d_n5;
        locals.var_t0_dn6 = assign20660_e15457_d_n6;
        locals.var_t0_dn7 = assign20660_e15457_d_n7;
        locals.var_t0_dn8 = assign20660_e15457_d_n8;
        locals.var_t0_dn9 = assign20660_e15457_d_n9;
        locals.var_t0_dn10 = assign20660_e15457_d_n10;
        locals.var_t0_dn13 = assign20660_e15457_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign20670_e15469, assign20670_e15469_d_n0, assign20670_e15469_d_n2, assign20670_e15469_d_n4, assign20670_e15469_d_n5, assign20670_e15469_d_n6, assign20670_e15469_d_n7, assign20670_e15469_d_n8, assign20670_e15469_d_n9, assign20670_e15469_d_n10, assign20670_e15469_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) {
        let assign20670_e15466: f64 = (locals.var_t0 * locals.var_vdserevz);
        let assign20670_e15467: f64 = (locals.var_t1 + assign20670_e15466);
        (assign20670_e15467, (locals.var_t1_dn0 + ((locals.var_t0_dn0 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn0))), (locals.var_t1_dn2 + ((locals.var_t0_dn2 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn2))), (locals.var_t1_dn4 + ((locals.var_t0_dn4 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn4))), (locals.var_t1_dn5 + ((locals.var_t0_dn5 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn5))), (locals.var_t1_dn6 + ((locals.var_t0_dn6 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn6))), (locals.var_t1_dn7 + ((locals.var_t0_dn7 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn7))), (locals.var_t1_dn8 + ((locals.var_t0_dn8 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn8))), (locals.var_t1_dn9 + ((locals.var_t0_dn9 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn9))), (locals.var_t1_dn10 + ((locals.var_t0_dn10 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn10))), (locals.var_t1_dn13 + ((locals.var_t0_dn13 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn13))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign20670_e15469;
        locals.var_t4_dn0 = assign20670_e15469_d_n0;
        locals.var_t4_dn2 = assign20670_e15469_d_n2;
        locals.var_t4_dn4 = assign20670_e15469_d_n4;
        locals.var_t4_dn5 = assign20670_e15469_d_n5;
        locals.var_t4_dn6 = assign20670_e15469_d_n6;
        locals.var_t4_dn7 = assign20670_e15469_d_n7;
        locals.var_t4_dn8 = assign20670_e15469_d_n8;
        locals.var_t4_dn9 = assign20670_e15469_d_n9;
        locals.var_t4_dn10 = assign20670_e15469_d_n10;
        locals.var_t4_dn13 = assign20670_e15469_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign20680_e15490, assign20680_e15490_d_n0, assign20680_e15490_d_n2, assign20680_e15490_d_n4, assign20680_e15490_d_n5, assign20680_e15490_d_n6, assign20680_e15490_d_n7, assign20680_e15490_d_n8, assign20680_e15490_d_n9, assign20680_e15490_d_n10, assign20680_e15490_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) {
        let assign20680_e15477: f64 = (p.p292 * p.p292);
        let assign20680_e15481: f64 = (0.0001 * 0.01);
        let assign20680_e15482: f64 = (4.0 * assign20680_e15481);
        let assign20680_e15485: f64 = (0.0001 * 0.01);
        let assign20680_e15486: f64 = (assign20680_e15482 * assign20680_e15485);
        let assign20680_e15487: f64 = (assign20680_e15477 + assign20680_e15486);
        let assign20680_e15488: f64 = (assign20680_e15487).sqrt();
        (assign20680_e15488, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign20680_e15490;
        locals.var_tmf2_dn0 = assign20680_e15490_d_n0;
        locals.var_tmf2_dn2 = assign20680_e15490_d_n2;
        locals.var_tmf2_dn4 = assign20680_e15490_d_n4;
        locals.var_tmf2_dn5 = assign20680_e15490_d_n5;
        locals.var_tmf2_dn6 = assign20680_e15490_d_n6;
        locals.var_tmf2_dn7 = assign20680_e15490_d_n7;
        locals.var_tmf2_dn8 = assign20680_e15490_d_n8;
        locals.var_tmf2_dn9 = assign20680_e15490_d_n9;
        locals.var_tmf2_dn10 = assign20680_e15490_d_n10;
        locals.var_tmf2_dn13 = assign20680_e15490_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign20690_e15504, assign20690_e15504_d_n0, assign20690_e15504_d_n2, assign20690_e15504_d_n4, assign20690_e15504_d_n5, assign20690_e15504_d_n6, assign20690_e15504_d_n7, assign20690_e15504_d_n8, assign20690_e15504_d_n9, assign20690_e15504_d_n10, assign20690_e15504_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) {
        let assign20690_e15500: f64 = (p.p292 / locals.var_tmf2);
        let assign20690_e15501: f64 = (1.0 + assign20690_e15500);
        let assign20690_e15502: f64 = (0.5 * assign20690_e15501);
        (assign20690_e15502, (0.5 * (-((p.p292 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn13) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign20690_e15504;
        locals.var_t0_dn0 = assign20690_e15504_d_n0;
        locals.var_t0_dn2 = assign20690_e15504_d_n2;
        locals.var_t0_dn4 = assign20690_e15504_d_n4;
        locals.var_t0_dn5 = assign20690_e15504_d_n5;
        locals.var_t0_dn6 = assign20690_e15504_d_n6;
        locals.var_t0_dn7 = assign20690_e15504_d_n7;
        locals.var_t0_dn8 = assign20690_e15504_d_n8;
        locals.var_t0_dn9 = assign20690_e15504_d_n9;
        locals.var_t0_dn10 = assign20690_e15504_d_n10;
        locals.var_t0_dn13 = assign20690_e15504_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign20700_e15516, assign20700_e15516_d_n0, assign20700_e15516_d_n2, assign20700_e15516_d_n4, assign20700_e15516_d_n5, assign20700_e15516_d_n6, assign20700_e15516_d_n7, assign20700_e15516_d_n8, assign20700_e15516_d_n9, assign20700_e15516_d_n10, assign20700_e15516_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) {
        let assign20700_e15513: f64 = (p.p292 + locals.var_tmf2);
        let assign20700_e15514: f64 = (0.5 * assign20700_e15513);
        (assign20700_e15514, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * locals.var_tmf2_dn5), (0.5 * locals.var_tmf2_dn6), (0.5 * locals.var_tmf2_dn7), (0.5 * locals.var_tmf2_dn8), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn13),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign20700_e15516;
        locals.var_t10_dn0 = assign20700_e15516_d_n0;
        locals.var_t10_dn2 = assign20700_e15516_d_n2;
        locals.var_t10_dn4 = assign20700_e15516_d_n4;
        locals.var_t10_dn5 = assign20700_e15516_d_n5;
        locals.var_t10_dn6 = assign20700_e15516_d_n6;
        locals.var_t10_dn7 = assign20700_e15516_d_n7;
        locals.var_t10_dn8 = assign20700_e15516_d_n8;
        locals.var_t10_dn9 = assign20700_e15516_d_n9;
        locals.var_t10_dn10 = assign20700_e15516_d_n10;
        locals.var_t10_dn13 = assign20700_e15516_d_n13;
        locals.var_t10_rv = 0.0;

        let assign20710_e15519: f64 = if locals.var_t10 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard414 = assign20710_e15519;
        locals.var_guard414_rv = 0.0;

        let (assign20720_e15529, assign20720_e15529_d_n0, assign20720_e15529_d_n2, assign20720_e15529_d_n4, assign20720_e15529_d_n5, assign20720_e15529_d_n6, assign20720_e15529_d_n7, assign20720_e15529_d_n8, assign20720_e15529_d_n9, assign20720_e15529_d_n10, assign20720_e15529_d_n13,) = {
    if ((((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) && (locals.var_guard414 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign20720_e15529;
        locals.var_t10_dn0 = assign20720_e15529_d_n0;
        locals.var_t10_dn2 = assign20720_e15529_d_n2;
        locals.var_t10_dn4 = assign20720_e15529_d_n4;
        locals.var_t10_dn5 = assign20720_e15529_d_n5;
        locals.var_t10_dn6 = assign20720_e15529_d_n6;
        locals.var_t10_dn7 = assign20720_e15529_d_n7;
        locals.var_t10_dn8 = assign20720_e15529_d_n8;
        locals.var_t10_dn9 = assign20720_e15529_d_n9;
        locals.var_t10_dn10 = assign20720_e15529_d_n10;
        locals.var_t10_dn13 = assign20720_e15529_d_n13;
        locals.var_t10_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_51(
        locals: &mut StampLocals,
    ) {
        let (assign20730_e15539, assign20730_e15539_d_n0, assign20730_e15539_d_n2, assign20730_e15539_d_n4, assign20730_e15539_d_n5, assign20730_e15539_d_n6, assign20730_e15539_d_n7, assign20730_e15539_d_n8, assign20730_e15539_d_n9, assign20730_e15539_d_n10, assign20730_e15539_d_n13,) = {
    if ((((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) && (locals.var_guard414 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign20730_e15539;
        locals.var_t0_dn0 = assign20730_e15539_d_n0;
        locals.var_t0_dn2 = assign20730_e15539_d_n2;
        locals.var_t0_dn4 = assign20730_e15539_d_n4;
        locals.var_t0_dn5 = assign20730_e15539_d_n5;
        locals.var_t0_dn6 = assign20730_e15539_d_n6;
        locals.var_t0_dn7 = assign20730_e15539_d_n7;
        locals.var_t0_dn8 = assign20730_e15539_d_n8;
        locals.var_t0_dn9 = assign20730_e15539_d_n9;
        locals.var_t0_dn10 = assign20730_e15539_d_n10;
        locals.var_t0_dn13 = assign20730_e15539_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign20740_e15557, assign20740_e15557_d_n0, assign20740_e15557_d_n2, assign20740_e15557_d_n4, assign20740_e15557_d_n5, assign20740_e15557_d_n6, assign20740_e15557_d_n7, assign20740_e15557_d_n8, assign20740_e15557_d_n9, assign20740_e15557_d_n10, assign20740_e15557_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) {
        let assign20740_e15551: f64 = (locals.var_vgserevz / locals.var_t10);
        let assign20740_e15552: f64 = (1.0 - assign20740_e15551);
        let assign20740_e15553: f64 = (locals.var_uc_rdvg11 * assign20740_e15552);
        let assign20740_e15554: f64 = (1.0 + assign20740_e15553);
        let assign20740_e15555: f64 = (locals.var_t4 * assign20740_e15554);
        (assign20740_e15555, ((locals.var_t4_dn0 * assign20740_e15554) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn0 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn0)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn2 * assign20740_e15554) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn2 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn2)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn4 * assign20740_e15554) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn4 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn4)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn5 * assign20740_e15554) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn5 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn5)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn6 * assign20740_e15554) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn6 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn6)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn7 * assign20740_e15554) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn7 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn7)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn8 * assign20740_e15554) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn8 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn8)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn9 * assign20740_e15554) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn9 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn9)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn10 * assign20740_e15554) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn10 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn10)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn13 * assign20740_e15554) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn13 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn13)) / (locals.var_t10 * locals.var_t10)))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign20740_e15557;
        locals.var_t1_dn0 = assign20740_e15557_d_n0;
        locals.var_t1_dn2 = assign20740_e15557_d_n2;
        locals.var_t1_dn4 = assign20740_e15557_d_n4;
        locals.var_t1_dn5 = assign20740_e15557_d_n5;
        locals.var_t1_dn6 = assign20740_e15557_d_n6;
        locals.var_t1_dn7 = assign20740_e15557_d_n7;
        locals.var_t1_dn8 = assign20740_e15557_d_n8;
        locals.var_t1_dn9 = assign20740_e15557_d_n9;
        locals.var_t1_dn10 = assign20740_e15557_d_n10;
        locals.var_t1_dn13 = assign20740_e15557_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign20750_e15571, assign20750_e15571_d_n0, assign20750_e15571_d_n2, assign20750_e15571_d_n4, assign20750_e15571_d_n5, assign20750_e15571_d_n6, assign20750_e15571_d_n7, assign20750_e15571_d_n8, assign20750_e15571_d_n9, assign20750_e15571_d_n10, assign20750_e15571_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) {
        let assign20750_e15565: f64 = (locals.var_t1 - locals.var_t4);
        let assign20750_e15568: f64 = (0.01 * 0.01);
        let assign20750_e15569: f64 = (assign20750_e15565 - assign20750_e15568);
        (assign20750_e15569, (locals.var_t1_dn0 - locals.var_t4_dn0), (locals.var_t1_dn2 - locals.var_t4_dn2), (locals.var_t1_dn4 - locals.var_t4_dn4), (locals.var_t1_dn5 - locals.var_t4_dn5), (locals.var_t1_dn6 - locals.var_t4_dn6), (locals.var_t1_dn7 - locals.var_t4_dn7), (locals.var_t1_dn8 - locals.var_t4_dn8), (locals.var_t1_dn9 - locals.var_t4_dn9), (locals.var_t1_dn10 - locals.var_t4_dn10), (locals.var_t1_dn13 - locals.var_t4_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign20750_e15571;
        locals.var_tmf1_dn0 = assign20750_e15571_d_n0;
        locals.var_tmf1_dn2 = assign20750_e15571_d_n2;
        locals.var_tmf1_dn4 = assign20750_e15571_d_n4;
        locals.var_tmf1_dn5 = assign20750_e15571_d_n5;
        locals.var_tmf1_dn6 = assign20750_e15571_d_n6;
        locals.var_tmf1_dn7 = assign20750_e15571_d_n7;
        locals.var_tmf1_dn8 = assign20750_e15571_d_n8;
        locals.var_tmf1_dn9 = assign20750_e15571_d_n9;
        locals.var_tmf1_dn10 = assign20750_e15571_d_n10;
        locals.var_tmf1_dn13 = assign20750_e15571_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign20760_e15585, assign20760_e15585_d_n0, assign20760_e15585_d_n2, assign20760_e15585_d_n4, assign20760_e15585_d_n5, assign20760_e15585_d_n6, assign20760_e15585_d_n7, assign20760_e15585_d_n8, assign20760_e15585_d_n9, assign20760_e15585_d_n10, assign20760_e15585_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) {
        let assign20760_e15579: f64 = (4.0 * locals.var_t4);
        let assign20760_e15582: f64 = (0.01 * 0.01);
        let assign20760_e15583: f64 = (assign20760_e15579 * assign20760_e15582);
        (assign20760_e15583, ((4.0 * locals.var_t4_dn0) * assign20760_e15582), ((4.0 * locals.var_t4_dn2) * assign20760_e15582), ((4.0 * locals.var_t4_dn4) * assign20760_e15582), ((4.0 * locals.var_t4_dn5) * assign20760_e15582), ((4.0 * locals.var_t4_dn6) * assign20760_e15582), ((4.0 * locals.var_t4_dn7) * assign20760_e15582), ((4.0 * locals.var_t4_dn8) * assign20760_e15582), ((4.0 * locals.var_t4_dn9) * assign20760_e15582), ((4.0 * locals.var_t4_dn10) * assign20760_e15582), ((4.0 * locals.var_t4_dn13) * assign20760_e15582),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign20760_e15585;
        locals.var_tmf2_dn0 = assign20760_e15585_d_n0;
        locals.var_tmf2_dn2 = assign20760_e15585_d_n2;
        locals.var_tmf2_dn4 = assign20760_e15585_d_n4;
        locals.var_tmf2_dn5 = assign20760_e15585_d_n5;
        locals.var_tmf2_dn6 = assign20760_e15585_d_n6;
        locals.var_tmf2_dn7 = assign20760_e15585_d_n7;
        locals.var_tmf2_dn8 = assign20760_e15585_d_n8;
        locals.var_tmf2_dn9 = assign20760_e15585_d_n9;
        locals.var_tmf2_dn10 = assign20760_e15585_d_n10;
        locals.var_tmf2_dn13 = assign20760_e15585_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign20770_e15599, assign20770_e15599_d_n0, assign20770_e15599_d_n2, assign20770_e15599_d_n4, assign20770_e15599_d_n5, assign20770_e15599_d_n6, assign20770_e15599_d_n7, assign20770_e15599_d_n8, assign20770_e15599_d_n9, assign20770_e15599_d_n10, assign20770_e15599_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) {
        let (assign20770_e15597, assign20770_e15597_d_n0, assign20770_e15597_d_n2, assign20770_e15597_d_n4, assign20770_e15597_d_n5, assign20770_e15597_d_n6, assign20770_e15597_d_n7, assign20770_e15597_d_n8, assign20770_e15597_d_n9, assign20770_e15597_d_n10, assign20770_e15597_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign20770_e15596: f64 = (-locals.var_tmf2);
                (assign20770_e15596, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign20770_e15597, assign20770_e15597_d_n0, assign20770_e15597_d_n2, assign20770_e15597_d_n4, assign20770_e15597_d_n5, assign20770_e15597_d_n6, assign20770_e15597_d_n7, assign20770_e15597_d_n8, assign20770_e15597_d_n9, assign20770_e15597_d_n10, assign20770_e15597_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign20770_e15599;
        locals.var_tmf2_dn0 = assign20770_e15599_d_n0;
        locals.var_tmf2_dn2 = assign20770_e15599_d_n2;
        locals.var_tmf2_dn4 = assign20770_e15599_d_n4;
        locals.var_tmf2_dn5 = assign20770_e15599_d_n5;
        locals.var_tmf2_dn6 = assign20770_e15599_d_n6;
        locals.var_tmf2_dn7 = assign20770_e15599_d_n7;
        locals.var_tmf2_dn8 = assign20770_e15599_d_n8;
        locals.var_tmf2_dn9 = assign20770_e15599_d_n9;
        locals.var_tmf2_dn10 = assign20770_e15599_d_n10;
        locals.var_tmf2_dn13 = assign20770_e15599_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign20780_e15612, assign20780_e15612_d_n0, assign20780_e15612_d_n2, assign20780_e15612_d_n4, assign20780_e15612_d_n5, assign20780_e15612_d_n6, assign20780_e15612_d_n7, assign20780_e15612_d_n8, assign20780_e15612_d_n9, assign20780_e15612_d_n10, assign20780_e15612_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) {
        let assign20780_e15607: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign20780_e15609: f64 = (assign20780_e15607 + locals.var_tmf2);
        let assign20780_e15610: f64 = (assign20780_e15609).sqrt();
        (assign20780_e15610, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign20780_e15610)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign20780_e15610)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign20780_e15610)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign20780_e15610)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign20780_e15610)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign20780_e15610)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign20780_e15610)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign20780_e15610)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign20780_e15610)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign20780_e15610)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign20780_e15612;
        locals.var_tmf2_dn0 = assign20780_e15612_d_n0;
        locals.var_tmf2_dn2 = assign20780_e15612_d_n2;
        locals.var_tmf2_dn4 = assign20780_e15612_d_n4;
        locals.var_tmf2_dn5 = assign20780_e15612_d_n5;
        locals.var_tmf2_dn6 = assign20780_e15612_d_n6;
        locals.var_tmf2_dn7 = assign20780_e15612_d_n7;
        locals.var_tmf2_dn8 = assign20780_e15612_d_n8;
        locals.var_tmf2_dn9 = assign20780_e15612_d_n9;
        locals.var_tmf2_dn10 = assign20780_e15612_d_n10;
        locals.var_tmf2_dn13 = assign20780_e15612_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign20790_e15626, assign20790_e15626_d_n0, assign20790_e15626_d_n2, assign20790_e15626_d_n4, assign20790_e15626_d_n5, assign20790_e15626_d_n6, assign20790_e15626_d_n7, assign20790_e15626_d_n8, assign20790_e15626_d_n9, assign20790_e15626_d_n10, assign20790_e15626_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) {
        let assign20790_e15622: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign20790_e15623: f64 = (1.0 + assign20790_e15622);
        let assign20790_e15624: f64 = (0.5 * assign20790_e15623);
        (assign20790_e15624, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign20790_e15626;
        locals.var_t0_dn0 = assign20790_e15626_d_n0;
        locals.var_t0_dn2 = assign20790_e15626_d_n2;
        locals.var_t0_dn4 = assign20790_e15626_d_n4;
        locals.var_t0_dn5 = assign20790_e15626_d_n5;
        locals.var_t0_dn6 = assign20790_e15626_d_n6;
        locals.var_t0_dn7 = assign20790_e15626_d_n7;
        locals.var_t0_dn8 = assign20790_e15626_d_n8;
        locals.var_t0_dn9 = assign20790_e15626_d_n9;
        locals.var_t0_dn10 = assign20790_e15626_d_n10;
        locals.var_t0_dn13 = assign20790_e15626_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign20800_e15646, assign20800_e15646_d_n0, assign20800_e15646_d_n2, assign20800_e15646_d_n4, assign20800_e15646_d_n5, assign20800_e15646_d_n6, assign20800_e15646_d_n7, assign20800_e15646_d_n8, assign20800_e15646_d_n9, assign20800_e15646_d_n10, assign20800_e15646_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) {
        let assign20800_e15637: f64 = (2.0 * 0.01);
        let assign20800_e15639: f64 = (assign20800_e15637 * 0.01);
        let assign20800_e15640: f64 = (locals.var_tmf1 - assign20800_e15639);
        let assign20800_e15642: f64 = (assign20800_e15640 / locals.var_tmf2);
        let assign20800_e15643: f64 = (1.0 - assign20800_e15642);
        let assign20800_e15644: f64 = (0.5 * assign20800_e15643);
        (assign20800_e15644, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign20800_e15640 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign20800_e15640 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn4 * locals.var_tmf2) - (assign20800_e15640 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn5 * locals.var_tmf2) - (assign20800_e15640 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign20800_e15640 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign20800_e15640 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn8 * locals.var_tmf2) - (assign20800_e15640 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn9 * locals.var_tmf2) - (assign20800_e15640 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign20800_e15640 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn13 * locals.var_tmf2) - (assign20800_e15640 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign20800_e15646;
        locals.var_t5_dn0 = assign20800_e15646_d_n0;
        locals.var_t5_dn2 = assign20800_e15646_d_n2;
        locals.var_t5_dn4 = assign20800_e15646_d_n4;
        locals.var_t5_dn5 = assign20800_e15646_d_n5;
        locals.var_t5_dn6 = assign20800_e15646_d_n6;
        locals.var_t5_dn7 = assign20800_e15646_d_n7;
        locals.var_t5_dn8 = assign20800_e15646_d_n8;
        locals.var_t5_dn9 = assign20800_e15646_d_n9;
        locals.var_t5_dn10 = assign20800_e15646_d_n10;
        locals.var_t5_dn13 = assign20800_e15646_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign20810_e15660, assign20810_e15660_d_n0, assign20810_e15660_d_n2, assign20810_e15660_d_n4, assign20810_e15660_d_n5, assign20810_e15660_d_n6, assign20810_e15660_d_n7, assign20810_e15660_d_n8, assign20810_e15660_d_n9, assign20810_e15660_d_n10, assign20810_e15660_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) {
        let assign20810_e15656: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign20810_e15657: f64 = (0.5 * assign20810_e15656);
        let assign20810_e15658: f64 = (locals.var_t4 + assign20810_e15657);
        (assign20810_e15658, (locals.var_t4_dn0 + (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t4_dn2 + (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t4_dn4 + (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t4_dn5 + (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t4_dn6 + (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t4_dn7 + (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t4_dn8 + (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t4_dn9 + (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t4_dn10 + (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t4_dn13 + (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign20810_e15660;
        locals.var_t2_dn0 = assign20810_e15660_d_n0;
        locals.var_t2_dn2 = assign20810_e15660_d_n2;
        locals.var_t2_dn4 = assign20810_e15660_d_n4;
        locals.var_t2_dn5 = assign20810_e15660_d_n5;
        locals.var_t2_dn6 = assign20810_e15660_d_n6;
        locals.var_t2_dn7 = assign20810_e15660_d_n7;
        locals.var_t2_dn8 = assign20810_e15660_d_n8;
        locals.var_t2_dn9 = assign20810_e15660_d_n9;
        locals.var_t2_dn10 = assign20810_e15660_d_n10;
        locals.var_t2_dn13 = assign20810_e15660_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign20820_e15672, assign20820_e15672_d_n0, assign20820_e15672_d_n2, assign20820_e15672_d_n4, assign20820_e15672_d_n5, assign20820_e15672_d_n6, assign20820_e15672_d_n7, assign20820_e15672_d_n8, assign20820_e15672_d_n9, assign20820_e15672_d_n10, assign20820_e15672_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) {
        let assign20820_e15669: f64 = (1.0 + locals.var_uc_rdvg11);
        let assign20820_e15670: f64 = (locals.var_t4 * assign20820_e15669);
        (assign20820_e15670, (locals.var_t4_dn0 * assign20820_e15669), (locals.var_t4_dn2 * assign20820_e15669), (locals.var_t4_dn4 * assign20820_e15669), (locals.var_t4_dn5 * assign20820_e15669), (locals.var_t4_dn6 * assign20820_e15669), (locals.var_t4_dn7 * assign20820_e15669), (locals.var_t4_dn8 * assign20820_e15669), (locals.var_t4_dn9 * assign20820_e15669), (locals.var_t4_dn10 * assign20820_e15669), (locals.var_t4_dn13 * assign20820_e15669),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign20820_e15672;
        locals.var_t3_dn0 = assign20820_e15672_d_n0;
        locals.var_t3_dn2 = assign20820_e15672_d_n2;
        locals.var_t3_dn4 = assign20820_e15672_d_n4;
        locals.var_t3_dn5 = assign20820_e15672_d_n5;
        locals.var_t3_dn6 = assign20820_e15672_d_n6;
        locals.var_t3_dn7 = assign20820_e15672_d_n7;
        locals.var_t3_dn8 = assign20820_e15672_d_n8;
        locals.var_t3_dn9 = assign20820_e15672_d_n9;
        locals.var_t3_dn10 = assign20820_e15672_d_n10;
        locals.var_t3_dn13 = assign20820_e15672_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign20830_e15686, assign20830_e15686_d_n0, assign20830_e15686_d_n2, assign20830_e15686_d_n4, assign20830_e15686_d_n5, assign20830_e15686_d_n6, assign20830_e15686_d_n7, assign20830_e15686_d_n8, assign20830_e15686_d_n9, assign20830_e15686_d_n10, assign20830_e15686_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) {
        let assign20830_e15680: f64 = (locals.var_t3 - locals.var_t2);
        let assign20830_e15683: f64 = (5e-5 * 0.01);
        let assign20830_e15684: f64 = (assign20830_e15680 - assign20830_e15683);
        (assign20830_e15684, (locals.var_t3_dn0 - locals.var_t2_dn0), (locals.var_t3_dn2 - locals.var_t2_dn2), (locals.var_t3_dn4 - locals.var_t2_dn4), (locals.var_t3_dn5 - locals.var_t2_dn5), (locals.var_t3_dn6 - locals.var_t2_dn6), (locals.var_t3_dn7 - locals.var_t2_dn7), (locals.var_t3_dn8 - locals.var_t2_dn8), (locals.var_t3_dn9 - locals.var_t2_dn9), (locals.var_t3_dn10 - locals.var_t2_dn10), (locals.var_t3_dn13 - locals.var_t2_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign20830_e15686;
        locals.var_tmf1_dn0 = assign20830_e15686_d_n0;
        locals.var_tmf1_dn2 = assign20830_e15686_d_n2;
        locals.var_tmf1_dn4 = assign20830_e15686_d_n4;
        locals.var_tmf1_dn5 = assign20830_e15686_d_n5;
        locals.var_tmf1_dn6 = assign20830_e15686_d_n6;
        locals.var_tmf1_dn7 = assign20830_e15686_d_n7;
        locals.var_tmf1_dn8 = assign20830_e15686_d_n8;
        locals.var_tmf1_dn9 = assign20830_e15686_d_n9;
        locals.var_tmf1_dn10 = assign20830_e15686_d_n10;
        locals.var_tmf1_dn13 = assign20830_e15686_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign20840_e15700, assign20840_e15700_d_n0, assign20840_e15700_d_n2, assign20840_e15700_d_n4, assign20840_e15700_d_n5, assign20840_e15700_d_n6, assign20840_e15700_d_n7, assign20840_e15700_d_n8, assign20840_e15700_d_n9, assign20840_e15700_d_n10, assign20840_e15700_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) {
        let assign20840_e15694: f64 = (4.0 * locals.var_t3);
        let assign20840_e15697: f64 = (5e-5 * 0.01);
        let assign20840_e15698: f64 = (assign20840_e15694 * assign20840_e15697);
        (assign20840_e15698, ((4.0 * locals.var_t3_dn0) * assign20840_e15697), ((4.0 * locals.var_t3_dn2) * assign20840_e15697), ((4.0 * locals.var_t3_dn4) * assign20840_e15697), ((4.0 * locals.var_t3_dn5) * assign20840_e15697), ((4.0 * locals.var_t3_dn6) * assign20840_e15697), ((4.0 * locals.var_t3_dn7) * assign20840_e15697), ((4.0 * locals.var_t3_dn8) * assign20840_e15697), ((4.0 * locals.var_t3_dn9) * assign20840_e15697), ((4.0 * locals.var_t3_dn10) * assign20840_e15697), ((4.0 * locals.var_t3_dn13) * assign20840_e15697),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign20840_e15700;
        locals.var_tmf2_dn0 = assign20840_e15700_d_n0;
        locals.var_tmf2_dn2 = assign20840_e15700_d_n2;
        locals.var_tmf2_dn4 = assign20840_e15700_d_n4;
        locals.var_tmf2_dn5 = assign20840_e15700_d_n5;
        locals.var_tmf2_dn6 = assign20840_e15700_d_n6;
        locals.var_tmf2_dn7 = assign20840_e15700_d_n7;
        locals.var_tmf2_dn8 = assign20840_e15700_d_n8;
        locals.var_tmf2_dn9 = assign20840_e15700_d_n9;
        locals.var_tmf2_dn10 = assign20840_e15700_d_n10;
        locals.var_tmf2_dn13 = assign20840_e15700_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign20850_e15714, assign20850_e15714_d_n0, assign20850_e15714_d_n2, assign20850_e15714_d_n4, assign20850_e15714_d_n5, assign20850_e15714_d_n6, assign20850_e15714_d_n7, assign20850_e15714_d_n8, assign20850_e15714_d_n9, assign20850_e15714_d_n10, assign20850_e15714_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) {
        let (assign20850_e15712, assign20850_e15712_d_n0, assign20850_e15712_d_n2, assign20850_e15712_d_n4, assign20850_e15712_d_n5, assign20850_e15712_d_n6, assign20850_e15712_d_n7, assign20850_e15712_d_n8, assign20850_e15712_d_n9, assign20850_e15712_d_n10, assign20850_e15712_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign20850_e15711: f64 = (-locals.var_tmf2);
                (assign20850_e15711, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign20850_e15712, assign20850_e15712_d_n0, assign20850_e15712_d_n2, assign20850_e15712_d_n4, assign20850_e15712_d_n5, assign20850_e15712_d_n6, assign20850_e15712_d_n7, assign20850_e15712_d_n8, assign20850_e15712_d_n9, assign20850_e15712_d_n10, assign20850_e15712_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign20850_e15714;
        locals.var_tmf2_dn0 = assign20850_e15714_d_n0;
        locals.var_tmf2_dn2 = assign20850_e15714_d_n2;
        locals.var_tmf2_dn4 = assign20850_e15714_d_n4;
        locals.var_tmf2_dn5 = assign20850_e15714_d_n5;
        locals.var_tmf2_dn6 = assign20850_e15714_d_n6;
        locals.var_tmf2_dn7 = assign20850_e15714_d_n7;
        locals.var_tmf2_dn8 = assign20850_e15714_d_n8;
        locals.var_tmf2_dn9 = assign20850_e15714_d_n9;
        locals.var_tmf2_dn10 = assign20850_e15714_d_n10;
        locals.var_tmf2_dn13 = assign20850_e15714_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign20860_e15727, assign20860_e15727_d_n0, assign20860_e15727_d_n2, assign20860_e15727_d_n4, assign20860_e15727_d_n5, assign20860_e15727_d_n6, assign20860_e15727_d_n7, assign20860_e15727_d_n8, assign20860_e15727_d_n9, assign20860_e15727_d_n10, assign20860_e15727_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) {
        let assign20860_e15722: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign20860_e15724: f64 = (assign20860_e15722 + locals.var_tmf2);
        let assign20860_e15725: f64 = (assign20860_e15724).sqrt();
        (assign20860_e15725, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign20860_e15725)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign20860_e15725)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign20860_e15725)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign20860_e15725)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign20860_e15725)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign20860_e15725)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign20860_e15725)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign20860_e15725)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign20860_e15725)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign20860_e15725)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign20860_e15727;
        locals.var_tmf2_dn0 = assign20860_e15727_d_n0;
        locals.var_tmf2_dn2 = assign20860_e15727_d_n2;
        locals.var_tmf2_dn4 = assign20860_e15727_d_n4;
        locals.var_tmf2_dn5 = assign20860_e15727_d_n5;
        locals.var_tmf2_dn6 = assign20860_e15727_d_n6;
        locals.var_tmf2_dn7 = assign20860_e15727_d_n7;
        locals.var_tmf2_dn8 = assign20860_e15727_d_n8;
        locals.var_tmf2_dn9 = assign20860_e15727_d_n9;
        locals.var_tmf2_dn10 = assign20860_e15727_d_n10;
        locals.var_tmf2_dn13 = assign20860_e15727_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign20870_e15741, assign20870_e15741_d_n0, assign20870_e15741_d_n2, assign20870_e15741_d_n4, assign20870_e15741_d_n5, assign20870_e15741_d_n6, assign20870_e15741_d_n7, assign20870_e15741_d_n8, assign20870_e15741_d_n9, assign20870_e15741_d_n10, assign20870_e15741_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) {
        let assign20870_e15737: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign20870_e15738: f64 = (1.0 + assign20870_e15737);
        let assign20870_e15739: f64 = (0.5 * assign20870_e15738);
        (assign20870_e15739, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign20870_e15741;
        locals.var_t0_dn0 = assign20870_e15741_d_n0;
        locals.var_t0_dn2 = assign20870_e15741_d_n2;
        locals.var_t0_dn4 = assign20870_e15741_d_n4;
        locals.var_t0_dn5 = assign20870_e15741_d_n5;
        locals.var_t0_dn6 = assign20870_e15741_d_n6;
        locals.var_t0_dn7 = assign20870_e15741_d_n7;
        locals.var_t0_dn8 = assign20870_e15741_d_n8;
        locals.var_t0_dn9 = assign20870_e15741_d_n9;
        locals.var_t0_dn10 = assign20870_e15741_d_n10;
        locals.var_t0_dn13 = assign20870_e15741_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign20880_e15761, assign20880_e15761_d_n0, assign20880_e15761_d_n2, assign20880_e15761_d_n4, assign20880_e15761_d_n5, assign20880_e15761_d_n6, assign20880_e15761_d_n7, assign20880_e15761_d_n8, assign20880_e15761_d_n9, assign20880_e15761_d_n10, assign20880_e15761_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) {
        let assign20880_e15752: f64 = (2.0 * 5e-5);
        let assign20880_e15754: f64 = (assign20880_e15752 * 0.01);
        let assign20880_e15755: f64 = (locals.var_tmf1 + assign20880_e15754);
        let assign20880_e15757: f64 = (assign20880_e15755 / locals.var_tmf2);
        let assign20880_e15758: f64 = (1.0 - assign20880_e15757);
        let assign20880_e15759: f64 = (0.5 * assign20880_e15758);
        (assign20880_e15759, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign20880_e15755 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign20880_e15755 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn4 * locals.var_tmf2) - (assign20880_e15755 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn5 * locals.var_tmf2) - (assign20880_e15755 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign20880_e15755 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign20880_e15755 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn8 * locals.var_tmf2) - (assign20880_e15755 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn9 * locals.var_tmf2) - (assign20880_e15755 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign20880_e15755 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn13 * locals.var_tmf2) - (assign20880_e15755 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign20880_e15761;
        locals.var_t5_dn0 = assign20880_e15761_d_n0;
        locals.var_t5_dn2 = assign20880_e15761_d_n2;
        locals.var_t5_dn4 = assign20880_e15761_d_n4;
        locals.var_t5_dn5 = assign20880_e15761_d_n5;
        locals.var_t5_dn6 = assign20880_e15761_d_n6;
        locals.var_t5_dn7 = assign20880_e15761_d_n7;
        locals.var_t5_dn8 = assign20880_e15761_d_n8;
        locals.var_t5_dn9 = assign20880_e15761_d_n9;
        locals.var_t5_dn10 = assign20880_e15761_d_n10;
        locals.var_t5_dn13 = assign20880_e15761_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign20890_e15775, assign20890_e15775_d_n0, assign20890_e15775_d_n2, assign20890_e15775_d_n4, assign20890_e15775_d_n5, assign20890_e15775_d_n6, assign20890_e15775_d_n7, assign20890_e15775_d_n8, assign20890_e15775_d_n9, assign20890_e15775_d_n10, assign20890_e15775_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) {
        let assign20890_e15771: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign20890_e15772: f64 = (0.5 * assign20890_e15771);
        let assign20890_e15773: f64 = (locals.var_t3 - assign20890_e15772);
        (assign20890_e15773, (locals.var_t3_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t3_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t3_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t3_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t3_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t3_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t3_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t3_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t3_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t3_dn13 - (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_rdrift, locals.var_rdrift_dn0, locals.var_rdrift_dn2, locals.var_rdrift_dn4, locals.var_rdrift_dn5, locals.var_rdrift_dn6, locals.var_rdrift_dn7, locals.var_rdrift_dn8, locals.var_rdrift_dn9, locals.var_rdrift_dn10, locals.var_rdrift_dn13,)
    }
};
        locals.var_rdrift = assign20890_e15775;
        locals.var_rdrift_dn0 = assign20890_e15775_d_n0;
        locals.var_rdrift_dn2 = assign20890_e15775_d_n2;
        locals.var_rdrift_dn4 = assign20890_e15775_d_n4;
        locals.var_rdrift_dn5 = assign20890_e15775_d_n5;
        locals.var_rdrift_dn6 = assign20890_e15775_d_n6;
        locals.var_rdrift_dn7 = assign20890_e15775_d_n7;
        locals.var_rdrift_dn8 = assign20890_e15775_d_n8;
        locals.var_rdrift_dn9 = assign20890_e15775_d_n9;
        locals.var_rdrift_dn10 = assign20890_e15775_d_n10;
        locals.var_rdrift_dn13 = assign20890_e15775_d_n13;
        locals.var_rdrift_rv = 0.0;

        let (assign20900_e15787, assign20900_e15787_d_n0, assign20900_e15787_d_n2, assign20900_e15787_d_n4, assign20900_e15787_d_n5, assign20900_e15787_d_n6, assign20900_e15787_d_n7, assign20900_e15787_d_n8, assign20900_e15787_d_n9, assign20900_e15787_d_n10, assign20900_e15787_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) {
        let assign20900_e15784: f64 = (locals.var_uc_rdvb * locals.var_vbserevz);
        let assign20900_e15785: f64 = (1.0 - assign20900_e15784);
        (assign20900_e15785, (-(locals.var_uc_rdvb * locals.var_vbserevz_dn0)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn2)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn4)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn5)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn6)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn7)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn8)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn9)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn10)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign20900_e15787;
        locals.var_t1_dn0 = assign20900_e15787_d_n0;
        locals.var_t1_dn2 = assign20900_e15787_d_n2;
        locals.var_t1_dn4 = assign20900_e15787_d_n4;
        locals.var_t1_dn5 = assign20900_e15787_d_n5;
        locals.var_t1_dn6 = assign20900_e15787_d_n6;
        locals.var_t1_dn7 = assign20900_e15787_d_n7;
        locals.var_t1_dn8 = assign20900_e15787_d_n8;
        locals.var_t1_dn9 = assign20900_e15787_d_n9;
        locals.var_t1_dn10 = assign20900_e15787_d_n10;
        locals.var_t1_dn13 = assign20900_e15787_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign20910_e15808, assign20910_e15808_d_n0, assign20910_e15808_d_n2, assign20910_e15808_d_n4, assign20910_e15808_d_n5, assign20910_e15808_d_n6, assign20910_e15808_d_n7, assign20910_e15808_d_n8, assign20910_e15808_d_n9, assign20910_e15808_d_n10, assign20910_e15808_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) {
        let assign20910_e15795: f64 = (locals.var_t1 * locals.var_t1);
        let assign20910_e15799: f64 = (0.0001 * 0.01);
        let assign20910_e15800: f64 = (4.0 * assign20910_e15799);
        let assign20910_e15803: f64 = (0.0001 * 0.01);
        let assign20910_e15804: f64 = (assign20910_e15800 * assign20910_e15803);
        let assign20910_e15805: f64 = (assign20910_e15795 + assign20910_e15804);
        let assign20910_e15806: f64 = (assign20910_e15805).sqrt();
        (assign20910_e15806, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign20910_e15806)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign20910_e15806)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign20910_e15806)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign20910_e15806)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign20910_e15806)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign20910_e15806)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign20910_e15806)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign20910_e15806)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign20910_e15806)), (((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) / (2.0 * assign20910_e15806)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign20910_e15808;
        locals.var_tmf2_dn0 = assign20910_e15808_d_n0;
        locals.var_tmf2_dn2 = assign20910_e15808_d_n2;
        locals.var_tmf2_dn4 = assign20910_e15808_d_n4;
        locals.var_tmf2_dn5 = assign20910_e15808_d_n5;
        locals.var_tmf2_dn6 = assign20910_e15808_d_n6;
        locals.var_tmf2_dn7 = assign20910_e15808_d_n7;
        locals.var_tmf2_dn8 = assign20910_e15808_d_n8;
        locals.var_tmf2_dn9 = assign20910_e15808_d_n9;
        locals.var_tmf2_dn10 = assign20910_e15808_d_n10;
        locals.var_tmf2_dn13 = assign20910_e15808_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign20920_e15822, assign20920_e15822_d_n0, assign20920_e15822_d_n2, assign20920_e15822_d_n4, assign20920_e15822_d_n5, assign20920_e15822_d_n6, assign20920_e15822_d_n7, assign20920_e15822_d_n8, assign20920_e15822_d_n9, assign20920_e15822_d_n10, assign20920_e15822_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) {
        let assign20920_e15818: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign20920_e15819: f64 = (1.0 + assign20920_e15818);
        let assign20920_e15820: f64 = (0.5 * assign20920_e15819);
        (assign20920_e15820, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn7 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn9 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn13 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign20920_e15822;
        locals.var_t4_dn0 = assign20920_e15822_d_n0;
        locals.var_t4_dn2 = assign20920_e15822_d_n2;
        locals.var_t4_dn4 = assign20920_e15822_d_n4;
        locals.var_t4_dn5 = assign20920_e15822_d_n5;
        locals.var_t4_dn6 = assign20920_e15822_d_n6;
        locals.var_t4_dn7 = assign20920_e15822_d_n7;
        locals.var_t4_dn8 = assign20920_e15822_d_n8;
        locals.var_t4_dn9 = assign20920_e15822_d_n9;
        locals.var_t4_dn10 = assign20920_e15822_d_n10;
        locals.var_t4_dn13 = assign20920_e15822_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign20930_e15834, assign20930_e15834_d_n0, assign20930_e15834_d_n2, assign20930_e15834_d_n4, assign20930_e15834_d_n5, assign20930_e15834_d_n6, assign20930_e15834_d_n7, assign20930_e15834_d_n8, assign20930_e15834_d_n9, assign20930_e15834_d_n10, assign20930_e15834_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) {
        let assign20930_e15831: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign20930_e15832: f64 = (0.5 * assign20930_e15831);
        (assign20930_e15832, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign20930_e15834;
        locals.var_t3_dn0 = assign20930_e15834_d_n0;
        locals.var_t3_dn2 = assign20930_e15834_d_n2;
        locals.var_t3_dn4 = assign20930_e15834_d_n4;
        locals.var_t3_dn5 = assign20930_e15834_d_n5;
        locals.var_t3_dn6 = assign20930_e15834_d_n6;
        locals.var_t3_dn7 = assign20930_e15834_d_n7;
        locals.var_t3_dn8 = assign20930_e15834_d_n8;
        locals.var_t3_dn9 = assign20930_e15834_d_n9;
        locals.var_t3_dn10 = assign20930_e15834_d_n10;
        locals.var_t3_dn13 = assign20930_e15834_d_n13;
        locals.var_t3_rv = 0.0;

        let assign20940_e15837: f64 = if locals.var_t3 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard415 = assign20940_e15837;
        locals.var_guard415_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_52(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20950_e15847, assign20950_e15847_d_n0, assign20950_e15847_d_n2, assign20950_e15847_d_n4, assign20950_e15847_d_n5, assign20950_e15847_d_n6, assign20950_e15847_d_n7, assign20950_e15847_d_n8, assign20950_e15847_d_n9, assign20950_e15847_d_n10, assign20950_e15847_d_n13,) = {
    if ((((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) && (locals.var_guard415 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign20950_e15847;
        locals.var_t3_dn0 = assign20950_e15847_d_n0;
        locals.var_t3_dn2 = assign20950_e15847_d_n2;
        locals.var_t3_dn4 = assign20950_e15847_d_n4;
        locals.var_t3_dn5 = assign20950_e15847_d_n5;
        locals.var_t3_dn6 = assign20950_e15847_d_n6;
        locals.var_t3_dn7 = assign20950_e15847_d_n7;
        locals.var_t3_dn8 = assign20950_e15847_d_n8;
        locals.var_t3_dn9 = assign20950_e15847_d_n9;
        locals.var_t3_dn10 = assign20950_e15847_d_n10;
        locals.var_t3_dn13 = assign20950_e15847_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign20960_e15857, assign20960_e15857_d_n0, assign20960_e15857_d_n2, assign20960_e15857_d_n4, assign20960_e15857_d_n5, assign20960_e15857_d_n6, assign20960_e15857_d_n7, assign20960_e15857_d_n8, assign20960_e15857_d_n9, assign20960_e15857_d_n10, assign20960_e15857_d_n13,) = {
    if ((((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) && (locals.var_guard415 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign20960_e15857;
        locals.var_t4_dn0 = assign20960_e15857_d_n0;
        locals.var_t4_dn2 = assign20960_e15857_d_n2;
        locals.var_t4_dn4 = assign20960_e15857_d_n4;
        locals.var_t4_dn5 = assign20960_e15857_d_n5;
        locals.var_t4_dn6 = assign20960_e15857_d_n6;
        locals.var_t4_dn7 = assign20960_e15857_d_n7;
        locals.var_t4_dn8 = assign20960_e15857_d_n8;
        locals.var_t4_dn9 = assign20960_e15857_d_n9;
        locals.var_t4_dn10 = assign20960_e15857_d_n10;
        locals.var_t4_dn13 = assign20960_e15857_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign20970_e15867, assign20970_e15867_d_n0, assign20970_e15867_d_n2, assign20970_e15867_d_n4, assign20970_e15867_d_n5, assign20970_e15867_d_n6, assign20970_e15867_d_n7, assign20970_e15867_d_n8, assign20970_e15867_d_n9, assign20970_e15867_d_n10, assign20970_e15867_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) {
        let assign20970_e15865: f64 = (locals.var_t3 + 1e-25);
        (assign20970_e15865, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign20970_e15867;
        locals.var_t3_dn0 = assign20970_e15867_d_n0;
        locals.var_t3_dn2 = assign20970_e15867_d_n2;
        locals.var_t3_dn4 = assign20970_e15867_d_n4;
        locals.var_t3_dn5 = assign20970_e15867_d_n5;
        locals.var_t3_dn6 = assign20970_e15867_d_n6;
        locals.var_t3_dn7 = assign20970_e15867_d_n7;
        locals.var_t3_dn8 = assign20970_e15867_d_n8;
        locals.var_t3_dn9 = assign20970_e15867_d_n9;
        locals.var_t3_dn10 = assign20970_e15867_d_n10;
        locals.var_t3_dn13 = assign20970_e15867_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign20980_e15875, assign20980_e15875_d_n0, assign20980_e15875_d_n2, assign20980_e15875_d_n4, assign20980_e15875_d_n5, assign20980_e15875_d_n6, assign20980_e15875_d_n7, assign20980_e15875_d_n8, assign20980_e15875_d_n9, assign20980_e15875_d_n10, assign20980_e15875_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) {
        (locals.var_rdrift, locals.var_rdrift_dn0, locals.var_rdrift_dn2, locals.var_rdrift_dn4, locals.var_rdrift_dn5, locals.var_rdrift_dn6, locals.var_rdrift_dn7, locals.var_rdrift_dn8, locals.var_rdrift_dn9, locals.var_rdrift_dn10, locals.var_rdrift_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign20980_e15875;
        locals.var_t0_dn0 = assign20980_e15875_d_n0;
        locals.var_t0_dn2 = assign20980_e15875_d_n2;
        locals.var_t0_dn4 = assign20980_e15875_d_n4;
        locals.var_t0_dn5 = assign20980_e15875_d_n5;
        locals.var_t0_dn6 = assign20980_e15875_d_n6;
        locals.var_t0_dn7 = assign20980_e15875_d_n7;
        locals.var_t0_dn8 = assign20980_e15875_d_n8;
        locals.var_t0_dn9 = assign20980_e15875_d_n9;
        locals.var_t0_dn10 = assign20980_e15875_d_n10;
        locals.var_t0_dn13 = assign20980_e15875_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign20990_e15885, assign20990_e15885_d_n0, assign20990_e15885_d_n2, assign20990_e15885_d_n4, assign20990_e15885_d_n5, assign20990_e15885_d_n6, assign20990_e15885_d_n7, assign20990_e15885_d_n8, assign20990_e15885_d_n9, assign20990_e15885_d_n10, assign20990_e15885_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 != 0.0)) {
        let assign20990_e15883: f64 = (locals.var_rdrift * locals.var_t3);
        (assign20990_e15883, ((locals.var_rdrift_dn0 * locals.var_t3) + (locals.var_rdrift * locals.var_t3_dn0)), ((locals.var_rdrift_dn2 * locals.var_t3) + (locals.var_rdrift * locals.var_t3_dn2)), ((locals.var_rdrift_dn4 * locals.var_t3) + (locals.var_rdrift * locals.var_t3_dn4)), ((locals.var_rdrift_dn5 * locals.var_t3) + (locals.var_rdrift * locals.var_t3_dn5)), ((locals.var_rdrift_dn6 * locals.var_t3) + (locals.var_rdrift * locals.var_t3_dn6)), ((locals.var_rdrift_dn7 * locals.var_t3) + (locals.var_rdrift * locals.var_t3_dn7)), ((locals.var_rdrift_dn8 * locals.var_t3) + (locals.var_rdrift * locals.var_t3_dn8)), ((locals.var_rdrift_dn9 * locals.var_t3) + (locals.var_rdrift * locals.var_t3_dn9)), ((locals.var_rdrift_dn10 * locals.var_t3) + (locals.var_rdrift * locals.var_t3_dn10)), ((locals.var_rdrift_dn13 * locals.var_t3) + (locals.var_rdrift * locals.var_t3_dn13)),)
    } else {
        (locals.var_rdrift, locals.var_rdrift_dn0, locals.var_rdrift_dn2, locals.var_rdrift_dn4, locals.var_rdrift_dn5, locals.var_rdrift_dn6, locals.var_rdrift_dn7, locals.var_rdrift_dn8, locals.var_rdrift_dn9, locals.var_rdrift_dn10, locals.var_rdrift_dn13,)
    }
};
        locals.var_rdrift = assign20990_e15885;
        locals.var_rdrift_dn0 = assign20990_e15885_d_n0;
        locals.var_rdrift_dn2 = assign20990_e15885_d_n2;
        locals.var_rdrift_dn4 = assign20990_e15885_d_n4;
        locals.var_rdrift_dn5 = assign20990_e15885_d_n5;
        locals.var_rdrift_dn6 = assign20990_e15885_d_n6;
        locals.var_rdrift_dn7 = assign20990_e15885_d_n7;
        locals.var_rdrift_dn8 = assign20990_e15885_d_n8;
        locals.var_rdrift_dn9 = assign20990_e15885_d_n9;
        locals.var_rdrift_dn10 = assign20990_e15885_d_n10;
        locals.var_rdrift_dn13 = assign20990_e15885_d_n13;
        locals.var_rdrift_rv = 0.0;

        let (assign21000_e15894, assign21000_e15894_d_n0, assign21000_e15894_d_n2, assign21000_e15894_d_n4, assign21000_e15894_d_n5, assign21000_e15894_d_n6, assign21000_e15894_d_n7, assign21000_e15894_d_n8, assign21000_e15894_d_n9, assign21000_e15894_d_n10, assign21000_e15894_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard413 == 0.0)) {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn13,)
    } else {
        (locals.var_rdrift, locals.var_rdrift_dn0, locals.var_rdrift_dn2, locals.var_rdrift_dn4, locals.var_rdrift_dn5, locals.var_rdrift_dn6, locals.var_rdrift_dn7, locals.var_rdrift_dn8, locals.var_rdrift_dn9, locals.var_rdrift_dn10, locals.var_rdrift_dn13,)
    }
};
        locals.var_rdrift = assign21000_e15894;
        locals.var_rdrift_dn0 = assign21000_e15894_d_n0;
        locals.var_rdrift_dn2 = assign21000_e15894_d_n2;
        locals.var_rdrift_dn4 = assign21000_e15894_d_n4;
        locals.var_rdrift_dn5 = assign21000_e15894_d_n5;
        locals.var_rdrift_dn6 = assign21000_e15894_d_n6;
        locals.var_rdrift_dn7 = assign21000_e15894_d_n7;
        locals.var_rdrift_dn8 = assign21000_e15894_d_n8;
        locals.var_rdrift_dn9 = assign21000_e15894_d_n9;
        locals.var_rdrift_dn10 = assign21000_e15894_d_n10;
        locals.var_rdrift_dn13 = assign21000_e15894_d_n13;
        locals.var_rdrift_rv = 0.0;

        let (assign21010_e15906, assign21010_e15906_d_n0, assign21010_e15906_d_n2, assign21010_e15906_d_n4, assign21010_e15906_d_n5, assign21010_e15906_d_n6, assign21010_e15906_d_n7, assign21010_e15906_d_n8, assign21010_e15906_d_n9, assign21010_e15906_d_n10, assign21010_e15906_d_n13,) = {
    if ((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) {
        let assign21010_e15900: f64 = (locals.var_vdsemodenml * locals.var_rse);
        let assign21010_e15903: f64 = (locals.var_vdsemodervs * locals.var_rde);
        let assign21010_e15904: f64 = (assign21010_e15900 + assign21010_e15903);
        (assign21010_e15904, ((locals.var_vdsemodenml * locals.var_rse_dn0) + (locals.var_vdsemodervs * locals.var_rde_dn0)), ((locals.var_vdsemodenml * locals.var_rse_dn2) + (locals.var_vdsemodervs * locals.var_rde_dn2)), ((locals.var_vdsemodenml * locals.var_rse_dn4) + (locals.var_vdsemodervs * locals.var_rde_dn4)), ((locals.var_vdsemodenml * locals.var_rse_dn5) + (locals.var_vdsemodervs * locals.var_rde_dn5)), ((locals.var_vdsemodenml * locals.var_rse_dn6) + (locals.var_vdsemodervs * locals.var_rde_dn6)), ((locals.var_vdsemodenml * locals.var_rse_dn7) + (locals.var_vdsemodervs * locals.var_rde_dn7)), ((locals.var_vdsemodenml * locals.var_rse_dn8) + (locals.var_vdsemodervs * locals.var_rde_dn8)), ((locals.var_vdsemodenml * locals.var_rse_dn9) + (locals.var_vdsemodervs * locals.var_rde_dn9)), ((locals.var_vdsemodenml * locals.var_rse_dn10) + (locals.var_vdsemodervs * locals.var_rde_dn10)), ((locals.var_vdsemodenml * locals.var_rse_dn13) + (locals.var_vdsemodervs * locals.var_rde_dn13)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign21010_e15906;
        locals.var_t4_dn0 = assign21010_e15906_d_n0;
        locals.var_t4_dn2 = assign21010_e15906_d_n2;
        locals.var_t4_dn4 = assign21010_e15906_d_n4;
        locals.var_t4_dn5 = assign21010_e15906_d_n5;
        locals.var_t4_dn6 = assign21010_e15906_d_n6;
        locals.var_t4_dn7 = assign21010_e15906_d_n7;
        locals.var_t4_dn8 = assign21010_e15906_d_n8;
        locals.var_t4_dn9 = assign21010_e15906_d_n9;
        locals.var_t4_dn10 = assign21010_e15906_d_n10;
        locals.var_t4_dn13 = assign21010_e15906_d_n13;
        locals.var_t4_rv = 0.0;

        let assign21020_e15913: f64 = if ((p.p34 == 1.0) || (locals.var_vdsemodervs == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard416 = assign21020_e15913;
        locals.var_guard416_rv = 0.0;

        let (assign21030_e15927, assign21030_e15927_d_n0, assign21030_e15927_d_n2, assign21030_e15927_d_n4, assign21030_e15927_d_n5, assign21030_e15927_d_n6, assign21030_e15927_d_n7, assign21030_e15927_d_n8, assign21030_e15927_d_n9, assign21030_e15927_d_n10, assign21030_e15927_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard416 != 0.0)) {
        let assign21030_e15921: f64 = (locals.var_vdsemodenml * locals.var_rsvde);
        let assign21030_e15924: f64 = (locals.var_vdsemodervs * locals.var_rdvde);
        let assign21030_e15925: f64 = (assign21030_e15921 + assign21030_e15924);
        (assign21030_e15925, ((locals.var_vdsemodenml * locals.var_rsvde_dn0) + (locals.var_vdsemodervs * locals.var_rdvde_dn0)), ((locals.var_vdsemodenml * locals.var_rsvde_dn2) + (locals.var_vdsemodervs * locals.var_rdvde_dn2)), ((locals.var_vdsemodenml * locals.var_rsvde_dn4) + (locals.var_vdsemodervs * locals.var_rdvde_dn4)), ((locals.var_vdsemodenml * locals.var_rsvde_dn5) + (locals.var_vdsemodervs * locals.var_rdvde_dn5)), ((locals.var_vdsemodenml * locals.var_rsvde_dn6) + (locals.var_vdsemodervs * locals.var_rdvde_dn6)), ((locals.var_vdsemodenml * locals.var_rsvde_dn7) + (locals.var_vdsemodervs * locals.var_rdvde_dn7)), ((locals.var_vdsemodenml * locals.var_rsvde_dn8) + (locals.var_vdsemodervs * locals.var_rdvde_dn8)), ((locals.var_vdsemodenml * locals.var_rsvde_dn9) + (locals.var_vdsemodervs * locals.var_rdvde_dn9)), ((locals.var_vdsemodenml * locals.var_rsvde_dn10) + (locals.var_vdsemodervs * locals.var_rdvde_dn10)), ((locals.var_vdsemodenml * locals.var_rsvde_dn13) + (locals.var_vdsemodervs * locals.var_rdvde_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign21030_e15927;
        locals.var_t0_dn0 = assign21030_e15927_d_n0;
        locals.var_t0_dn2 = assign21030_e15927_d_n2;
        locals.var_t0_dn4 = assign21030_e15927_d_n4;
        locals.var_t0_dn5 = assign21030_e15927_d_n5;
        locals.var_t0_dn6 = assign21030_e15927_d_n6;
        locals.var_t0_dn7 = assign21030_e15927_d_n7;
        locals.var_t0_dn8 = assign21030_e15927_d_n8;
        locals.var_t0_dn9 = assign21030_e15927_d_n9;
        locals.var_t0_dn10 = assign21030_e15927_d_n10;
        locals.var_t0_dn13 = assign21030_e15927_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign21040_e15941, assign21040_e15941_d_n0, assign21040_e15941_d_n2, assign21040_e15941_d_n4, assign21040_e15941_d_n5, assign21040_e15941_d_n6, assign21040_e15941_d_n7, assign21040_e15941_d_n8, assign21040_e15941_d_n9, assign21040_e15941_d_n10, assign21040_e15941_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard416 != 0.0)) {
        let assign21040_e15937: f64 = (2.0 * p.p262);
        let assign21040_e15938: f64 = (locals.var_t0 * assign21040_e15937);
        let assign21040_e15939: f64 = (locals.var_t4 + assign21040_e15938);
        (assign21040_e15939, (locals.var_t4_dn0 + (locals.var_t0_dn0 * assign21040_e15937)), (locals.var_t4_dn2 + (locals.var_t0_dn2 * assign21040_e15937)), (locals.var_t4_dn4 + (locals.var_t0_dn4 * assign21040_e15937)), (locals.var_t4_dn5 + (locals.var_t0_dn5 * assign21040_e15937)), (locals.var_t4_dn6 + (locals.var_t0_dn6 * assign21040_e15937)), (locals.var_t4_dn7 + (locals.var_t0_dn7 * assign21040_e15937)), (locals.var_t4_dn8 + (locals.var_t0_dn8 * assign21040_e15937)), (locals.var_t4_dn9 + (locals.var_t0_dn9 * assign21040_e15937)), (locals.var_t4_dn10 + (locals.var_t0_dn10 * assign21040_e15937)), (locals.var_t4_dn13 + (locals.var_t0_dn13 * assign21040_e15937)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign21040_e15941;
        locals.var_t4_dn0 = assign21040_e15941_d_n0;
        locals.var_t4_dn2 = assign21040_e15941_d_n2;
        locals.var_t4_dn4 = assign21040_e15941_d_n4;
        locals.var_t4_dn5 = assign21040_e15941_d_n5;
        locals.var_t4_dn6 = assign21040_e15941_d_n6;
        locals.var_t4_dn7 = assign21040_e15941_d_n7;
        locals.var_t4_dn8 = assign21040_e15941_d_n8;
        locals.var_t4_dn9 = assign21040_e15941_d_n9;
        locals.var_t4_dn10 = assign21040_e15941_d_n10;
        locals.var_t4_dn13 = assign21040_e15941_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign21050_e15951, assign21050_e15951_d_n0, assign21050_e15951_d_n2, assign21050_e15951_d_n4, assign21050_e15951_d_n5, assign21050_e15951_d_n6, assign21050_e15951_d_n7, assign21050_e15951_d_n8, assign21050_e15951_d_n9, assign21050_e15951_d_n10, assign21050_e15951_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard416 != 0.0)) {
        let assign21050_e15949: f64 = (p.p292 + 1e-25);
        (assign21050_e15949, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn13,)
    }
};
        locals.var_t10 = assign21050_e15951;
        locals.var_t10_dn0 = assign21050_e15951_d_n0;
        locals.var_t10_dn2 = assign21050_e15951_d_n2;
        locals.var_t10_dn4 = assign21050_e15951_d_n4;
        locals.var_t10_dn5 = assign21050_e15951_d_n5;
        locals.var_t10_dn6 = assign21050_e15951_d_n6;
        locals.var_t10_dn7 = assign21050_e15951_d_n7;
        locals.var_t10_dn8 = assign21050_e15951_d_n8;
        locals.var_t10_dn9 = assign21050_e15951_d_n9;
        locals.var_t10_dn10 = assign21050_e15951_d_n10;
        locals.var_t10_dn13 = assign21050_e15951_d_n13;
        locals.var_t10_rv = 0.0;

        let (assign21060_e15969, assign21060_e15969_d_n0, assign21060_e15969_d_n2, assign21060_e15969_d_n4, assign21060_e15969_d_n5, assign21060_e15969_d_n6, assign21060_e15969_d_n7, assign21060_e15969_d_n8, assign21060_e15969_d_n9, assign21060_e15969_d_n10, assign21060_e15969_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard416 != 0.0)) {
        let assign21060_e15963: f64 = (locals.var_vgserevz / locals.var_t10);
        let assign21060_e15964: f64 = (1.0 - assign21060_e15963);
        let assign21060_e15965: f64 = (locals.var_uc_rdvg11 * assign21060_e15964);
        let assign21060_e15966: f64 = (1.0 + assign21060_e15965);
        let assign21060_e15967: f64 = (locals.var_t4 * assign21060_e15966);
        (assign21060_e15967, ((locals.var_t4_dn0 * assign21060_e15966) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn0 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn0)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn2 * assign21060_e15966) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn2 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn2)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn4 * assign21060_e15966) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn4 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn4)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn5 * assign21060_e15966) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn5 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn5)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn6 * assign21060_e15966) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn6 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn6)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn7 * assign21060_e15966) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn7 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn7)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn8 * assign21060_e15966) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn8 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn8)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn9 * assign21060_e15966) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn9 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn9)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn10 * assign21060_e15966) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn10 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn10)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn13 * assign21060_e15966) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn13 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn13)) / (locals.var_t10 * locals.var_t10)))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign21060_e15969;
        locals.var_t1_dn0 = assign21060_e15969_d_n0;
        locals.var_t1_dn2 = assign21060_e15969_d_n2;
        locals.var_t1_dn4 = assign21060_e15969_d_n4;
        locals.var_t1_dn5 = assign21060_e15969_d_n5;
        locals.var_t1_dn6 = assign21060_e15969_d_n6;
        locals.var_t1_dn7 = assign21060_e15969_d_n7;
        locals.var_t1_dn8 = assign21060_e15969_d_n8;
        locals.var_t1_dn9 = assign21060_e15969_d_n9;
        locals.var_t1_dn10 = assign21060_e15969_d_n10;
        locals.var_t1_dn13 = assign21060_e15969_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign21070_e15983, assign21070_e15983_d_n0, assign21070_e15983_d_n2, assign21070_e15983_d_n4, assign21070_e15983_d_n5, assign21070_e15983_d_n6, assign21070_e15983_d_n7, assign21070_e15983_d_n8, assign21070_e15983_d_n9, assign21070_e15983_d_n10, assign21070_e15983_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard416 != 0.0)) {
        let assign21070_e15977: f64 = (locals.var_t1 - locals.var_t4);
        let assign21070_e15980: f64 = (0.01 * 0.01);
        let assign21070_e15981: f64 = (assign21070_e15977 - assign21070_e15980);
        (assign21070_e15981, (locals.var_t1_dn0 - locals.var_t4_dn0), (locals.var_t1_dn2 - locals.var_t4_dn2), (locals.var_t1_dn4 - locals.var_t4_dn4), (locals.var_t1_dn5 - locals.var_t4_dn5), (locals.var_t1_dn6 - locals.var_t4_dn6), (locals.var_t1_dn7 - locals.var_t4_dn7), (locals.var_t1_dn8 - locals.var_t4_dn8), (locals.var_t1_dn9 - locals.var_t4_dn9), (locals.var_t1_dn10 - locals.var_t4_dn10), (locals.var_t1_dn13 - locals.var_t4_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign21070_e15983;
        locals.var_tmf1_dn0 = assign21070_e15983_d_n0;
        locals.var_tmf1_dn2 = assign21070_e15983_d_n2;
        locals.var_tmf1_dn4 = assign21070_e15983_d_n4;
        locals.var_tmf1_dn5 = assign21070_e15983_d_n5;
        locals.var_tmf1_dn6 = assign21070_e15983_d_n6;
        locals.var_tmf1_dn7 = assign21070_e15983_d_n7;
        locals.var_tmf1_dn8 = assign21070_e15983_d_n8;
        locals.var_tmf1_dn9 = assign21070_e15983_d_n9;
        locals.var_tmf1_dn10 = assign21070_e15983_d_n10;
        locals.var_tmf1_dn13 = assign21070_e15983_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign21080_e15997, assign21080_e15997_d_n0, assign21080_e15997_d_n2, assign21080_e15997_d_n4, assign21080_e15997_d_n5, assign21080_e15997_d_n6, assign21080_e15997_d_n7, assign21080_e15997_d_n8, assign21080_e15997_d_n9, assign21080_e15997_d_n10, assign21080_e15997_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard416 != 0.0)) {
        let assign21080_e15991: f64 = (4.0 * locals.var_t4);
        let assign21080_e15994: f64 = (0.01 * 0.01);
        let assign21080_e15995: f64 = (assign21080_e15991 * assign21080_e15994);
        (assign21080_e15995, ((4.0 * locals.var_t4_dn0) * assign21080_e15994), ((4.0 * locals.var_t4_dn2) * assign21080_e15994), ((4.0 * locals.var_t4_dn4) * assign21080_e15994), ((4.0 * locals.var_t4_dn5) * assign21080_e15994), ((4.0 * locals.var_t4_dn6) * assign21080_e15994), ((4.0 * locals.var_t4_dn7) * assign21080_e15994), ((4.0 * locals.var_t4_dn8) * assign21080_e15994), ((4.0 * locals.var_t4_dn9) * assign21080_e15994), ((4.0 * locals.var_t4_dn10) * assign21080_e15994), ((4.0 * locals.var_t4_dn13) * assign21080_e15994),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign21080_e15997;
        locals.var_tmf2_dn0 = assign21080_e15997_d_n0;
        locals.var_tmf2_dn2 = assign21080_e15997_d_n2;
        locals.var_tmf2_dn4 = assign21080_e15997_d_n4;
        locals.var_tmf2_dn5 = assign21080_e15997_d_n5;
        locals.var_tmf2_dn6 = assign21080_e15997_d_n6;
        locals.var_tmf2_dn7 = assign21080_e15997_d_n7;
        locals.var_tmf2_dn8 = assign21080_e15997_d_n8;
        locals.var_tmf2_dn9 = assign21080_e15997_d_n9;
        locals.var_tmf2_dn10 = assign21080_e15997_d_n10;
        locals.var_tmf2_dn13 = assign21080_e15997_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign21090_e16011, assign21090_e16011_d_n0, assign21090_e16011_d_n2, assign21090_e16011_d_n4, assign21090_e16011_d_n5, assign21090_e16011_d_n6, assign21090_e16011_d_n7, assign21090_e16011_d_n8, assign21090_e16011_d_n9, assign21090_e16011_d_n10, assign21090_e16011_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard416 != 0.0)) {
        let (assign21090_e16009, assign21090_e16009_d_n0, assign21090_e16009_d_n2, assign21090_e16009_d_n4, assign21090_e16009_d_n5, assign21090_e16009_d_n6, assign21090_e16009_d_n7, assign21090_e16009_d_n8, assign21090_e16009_d_n9, assign21090_e16009_d_n10, assign21090_e16009_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign21090_e16008: f64 = (-locals.var_tmf2);
                (assign21090_e16008, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign21090_e16009, assign21090_e16009_d_n0, assign21090_e16009_d_n2, assign21090_e16009_d_n4, assign21090_e16009_d_n5, assign21090_e16009_d_n6, assign21090_e16009_d_n7, assign21090_e16009_d_n8, assign21090_e16009_d_n9, assign21090_e16009_d_n10, assign21090_e16009_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign21090_e16011;
        locals.var_tmf2_dn0 = assign21090_e16011_d_n0;
        locals.var_tmf2_dn2 = assign21090_e16011_d_n2;
        locals.var_tmf2_dn4 = assign21090_e16011_d_n4;
        locals.var_tmf2_dn5 = assign21090_e16011_d_n5;
        locals.var_tmf2_dn6 = assign21090_e16011_d_n6;
        locals.var_tmf2_dn7 = assign21090_e16011_d_n7;
        locals.var_tmf2_dn8 = assign21090_e16011_d_n8;
        locals.var_tmf2_dn9 = assign21090_e16011_d_n9;
        locals.var_tmf2_dn10 = assign21090_e16011_d_n10;
        locals.var_tmf2_dn13 = assign21090_e16011_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign21100_e16024, assign21100_e16024_d_n0, assign21100_e16024_d_n2, assign21100_e16024_d_n4, assign21100_e16024_d_n5, assign21100_e16024_d_n6, assign21100_e16024_d_n7, assign21100_e16024_d_n8, assign21100_e16024_d_n9, assign21100_e16024_d_n10, assign21100_e16024_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard416 != 0.0)) {
        let assign21100_e16019: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign21100_e16021: f64 = (assign21100_e16019 + locals.var_tmf2);
        let assign21100_e16022: f64 = (assign21100_e16021).sqrt();
        (assign21100_e16022, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign21100_e16022)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign21100_e16022)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign21100_e16022)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign21100_e16022)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign21100_e16022)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign21100_e16022)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign21100_e16022)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign21100_e16022)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign21100_e16022)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign21100_e16022)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign21100_e16024;
        locals.var_tmf2_dn0 = assign21100_e16024_d_n0;
        locals.var_tmf2_dn2 = assign21100_e16024_d_n2;
        locals.var_tmf2_dn4 = assign21100_e16024_d_n4;
        locals.var_tmf2_dn5 = assign21100_e16024_d_n5;
        locals.var_tmf2_dn6 = assign21100_e16024_d_n6;
        locals.var_tmf2_dn7 = assign21100_e16024_d_n7;
        locals.var_tmf2_dn8 = assign21100_e16024_d_n8;
        locals.var_tmf2_dn9 = assign21100_e16024_d_n9;
        locals.var_tmf2_dn10 = assign21100_e16024_d_n10;
        locals.var_tmf2_dn13 = assign21100_e16024_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign21110_e16038, assign21110_e16038_d_n0, assign21110_e16038_d_n2, assign21110_e16038_d_n4, assign21110_e16038_d_n5, assign21110_e16038_d_n6, assign21110_e16038_d_n7, assign21110_e16038_d_n8, assign21110_e16038_d_n9, assign21110_e16038_d_n10, assign21110_e16038_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard416 != 0.0)) {
        let assign21110_e16034: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign21110_e16035: f64 = (1.0 + assign21110_e16034);
        let assign21110_e16036: f64 = (0.5 * assign21110_e16035);
        (assign21110_e16036, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign21110_e16038;
        locals.var_t0_dn0 = assign21110_e16038_d_n0;
        locals.var_t0_dn2 = assign21110_e16038_d_n2;
        locals.var_t0_dn4 = assign21110_e16038_d_n4;
        locals.var_t0_dn5 = assign21110_e16038_d_n5;
        locals.var_t0_dn6 = assign21110_e16038_d_n6;
        locals.var_t0_dn7 = assign21110_e16038_d_n7;
        locals.var_t0_dn8 = assign21110_e16038_d_n8;
        locals.var_t0_dn9 = assign21110_e16038_d_n9;
        locals.var_t0_dn10 = assign21110_e16038_d_n10;
        locals.var_t0_dn13 = assign21110_e16038_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign21120_e16058, assign21120_e16058_d_n0, assign21120_e16058_d_n2, assign21120_e16058_d_n4, assign21120_e16058_d_n5, assign21120_e16058_d_n6, assign21120_e16058_d_n7, assign21120_e16058_d_n8, assign21120_e16058_d_n9, assign21120_e16058_d_n10, assign21120_e16058_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard416 != 0.0)) {
        let assign21120_e16049: f64 = (2.0 * 0.01);
        let assign21120_e16051: f64 = (assign21120_e16049 * 0.01);
        let assign21120_e16052: f64 = (locals.var_tmf1 - assign21120_e16051);
        let assign21120_e16054: f64 = (assign21120_e16052 / locals.var_tmf2);
        let assign21120_e16055: f64 = (1.0 - assign21120_e16054);
        let assign21120_e16056: f64 = (0.5 * assign21120_e16055);
        (assign21120_e16056, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign21120_e16052 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign21120_e16052 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn4 * locals.var_tmf2) - (assign21120_e16052 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn5 * locals.var_tmf2) - (assign21120_e16052 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign21120_e16052 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign21120_e16052 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn8 * locals.var_tmf2) - (assign21120_e16052 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn9 * locals.var_tmf2) - (assign21120_e16052 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign21120_e16052 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn13 * locals.var_tmf2) - (assign21120_e16052 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign21120_e16058;
        locals.var_t5_dn0 = assign21120_e16058_d_n0;
        locals.var_t5_dn2 = assign21120_e16058_d_n2;
        locals.var_t5_dn4 = assign21120_e16058_d_n4;
        locals.var_t5_dn5 = assign21120_e16058_d_n5;
        locals.var_t5_dn6 = assign21120_e16058_d_n6;
        locals.var_t5_dn7 = assign21120_e16058_d_n7;
        locals.var_t5_dn8 = assign21120_e16058_d_n8;
        locals.var_t5_dn9 = assign21120_e16058_d_n9;
        locals.var_t5_dn10 = assign21120_e16058_d_n10;
        locals.var_t5_dn13 = assign21120_e16058_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign21130_e16072, assign21130_e16072_d_n0, assign21130_e16072_d_n2, assign21130_e16072_d_n4, assign21130_e16072_d_n5, assign21130_e16072_d_n6, assign21130_e16072_d_n7, assign21130_e16072_d_n8, assign21130_e16072_d_n9, assign21130_e16072_d_n10, assign21130_e16072_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard416 != 0.0)) {
        let assign21130_e16068: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign21130_e16069: f64 = (0.5 * assign21130_e16068);
        let assign21130_e16070: f64 = (locals.var_t4 + assign21130_e16069);
        (assign21130_e16070, (locals.var_t4_dn0 + (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t4_dn2 + (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t4_dn4 + (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t4_dn5 + (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t4_dn6 + (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t4_dn7 + (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t4_dn8 + (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t4_dn9 + (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t4_dn10 + (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t4_dn13 + (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign21130_e16072;
        locals.var_t2_dn0 = assign21130_e16072_d_n0;
        locals.var_t2_dn2 = assign21130_e16072_d_n2;
        locals.var_t2_dn4 = assign21130_e16072_d_n4;
        locals.var_t2_dn5 = assign21130_e16072_d_n5;
        locals.var_t2_dn6 = assign21130_e16072_d_n6;
        locals.var_t2_dn7 = assign21130_e16072_d_n7;
        locals.var_t2_dn8 = assign21130_e16072_d_n8;
        locals.var_t2_dn9 = assign21130_e16072_d_n9;
        locals.var_t2_dn10 = assign21130_e16072_d_n10;
        locals.var_t2_dn13 = assign21130_e16072_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign21140_e16084, assign21140_e16084_d_n0, assign21140_e16084_d_n2, assign21140_e16084_d_n4, assign21140_e16084_d_n5, assign21140_e16084_d_n6, assign21140_e16084_d_n7, assign21140_e16084_d_n8, assign21140_e16084_d_n9, assign21140_e16084_d_n10, assign21140_e16084_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard416 != 0.0)) {
        let assign21140_e16081: f64 = (1.0 + locals.var_uc_rdvg11);
        let assign21140_e16082: f64 = (locals.var_t4 * assign21140_e16081);
        (assign21140_e16082, (locals.var_t4_dn0 * assign21140_e16081), (locals.var_t4_dn2 * assign21140_e16081), (locals.var_t4_dn4 * assign21140_e16081), (locals.var_t4_dn5 * assign21140_e16081), (locals.var_t4_dn6 * assign21140_e16081), (locals.var_t4_dn7 * assign21140_e16081), (locals.var_t4_dn8 * assign21140_e16081), (locals.var_t4_dn9 * assign21140_e16081), (locals.var_t4_dn10 * assign21140_e16081), (locals.var_t4_dn13 * assign21140_e16081),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign21140_e16084;
        locals.var_t3_dn0 = assign21140_e16084_d_n0;
        locals.var_t3_dn2 = assign21140_e16084_d_n2;
        locals.var_t3_dn4 = assign21140_e16084_d_n4;
        locals.var_t3_dn5 = assign21140_e16084_d_n5;
        locals.var_t3_dn6 = assign21140_e16084_d_n6;
        locals.var_t3_dn7 = assign21140_e16084_d_n7;
        locals.var_t3_dn8 = assign21140_e16084_d_n8;
        locals.var_t3_dn9 = assign21140_e16084_d_n9;
        locals.var_t3_dn10 = assign21140_e16084_d_n10;
        locals.var_t3_dn13 = assign21140_e16084_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign21150_e16098, assign21150_e16098_d_n0, assign21150_e16098_d_n2, assign21150_e16098_d_n4, assign21150_e16098_d_n5, assign21150_e16098_d_n6, assign21150_e16098_d_n7, assign21150_e16098_d_n8, assign21150_e16098_d_n9, assign21150_e16098_d_n10, assign21150_e16098_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard416 != 0.0)) {
        let assign21150_e16092: f64 = (locals.var_t3 - locals.var_t2);
        let assign21150_e16095: f64 = (5e-5 * 0.01);
        let assign21150_e16096: f64 = (assign21150_e16092 - assign21150_e16095);
        (assign21150_e16096, (locals.var_t3_dn0 - locals.var_t2_dn0), (locals.var_t3_dn2 - locals.var_t2_dn2), (locals.var_t3_dn4 - locals.var_t2_dn4), (locals.var_t3_dn5 - locals.var_t2_dn5), (locals.var_t3_dn6 - locals.var_t2_dn6), (locals.var_t3_dn7 - locals.var_t2_dn7), (locals.var_t3_dn8 - locals.var_t2_dn8), (locals.var_t3_dn9 - locals.var_t2_dn9), (locals.var_t3_dn10 - locals.var_t2_dn10), (locals.var_t3_dn13 - locals.var_t2_dn13),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn13,)
    }
};
        locals.var_tmf1 = assign21150_e16098;
        locals.var_tmf1_dn0 = assign21150_e16098_d_n0;
        locals.var_tmf1_dn2 = assign21150_e16098_d_n2;
        locals.var_tmf1_dn4 = assign21150_e16098_d_n4;
        locals.var_tmf1_dn5 = assign21150_e16098_d_n5;
        locals.var_tmf1_dn6 = assign21150_e16098_d_n6;
        locals.var_tmf1_dn7 = assign21150_e16098_d_n7;
        locals.var_tmf1_dn8 = assign21150_e16098_d_n8;
        locals.var_tmf1_dn9 = assign21150_e16098_d_n9;
        locals.var_tmf1_dn10 = assign21150_e16098_d_n10;
        locals.var_tmf1_dn13 = assign21150_e16098_d_n13;
        locals.var_tmf1_rv = 0.0;

        let (assign21160_e16112, assign21160_e16112_d_n0, assign21160_e16112_d_n2, assign21160_e16112_d_n4, assign21160_e16112_d_n5, assign21160_e16112_d_n6, assign21160_e16112_d_n7, assign21160_e16112_d_n8, assign21160_e16112_d_n9, assign21160_e16112_d_n10, assign21160_e16112_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard416 != 0.0)) {
        let assign21160_e16106: f64 = (4.0 * locals.var_t3);
        let assign21160_e16109: f64 = (5e-5 * 0.01);
        let assign21160_e16110: f64 = (assign21160_e16106 * assign21160_e16109);
        (assign21160_e16110, ((4.0 * locals.var_t3_dn0) * assign21160_e16109), ((4.0 * locals.var_t3_dn2) * assign21160_e16109), ((4.0 * locals.var_t3_dn4) * assign21160_e16109), ((4.0 * locals.var_t3_dn5) * assign21160_e16109), ((4.0 * locals.var_t3_dn6) * assign21160_e16109), ((4.0 * locals.var_t3_dn7) * assign21160_e16109), ((4.0 * locals.var_t3_dn8) * assign21160_e16109), ((4.0 * locals.var_t3_dn9) * assign21160_e16109), ((4.0 * locals.var_t3_dn10) * assign21160_e16109), ((4.0 * locals.var_t3_dn13) * assign21160_e16109),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign21160_e16112;
        locals.var_tmf2_dn0 = assign21160_e16112_d_n0;
        locals.var_tmf2_dn2 = assign21160_e16112_d_n2;
        locals.var_tmf2_dn4 = assign21160_e16112_d_n4;
        locals.var_tmf2_dn5 = assign21160_e16112_d_n5;
        locals.var_tmf2_dn6 = assign21160_e16112_d_n6;
        locals.var_tmf2_dn7 = assign21160_e16112_d_n7;
        locals.var_tmf2_dn8 = assign21160_e16112_d_n8;
        locals.var_tmf2_dn9 = assign21160_e16112_d_n9;
        locals.var_tmf2_dn10 = assign21160_e16112_d_n10;
        locals.var_tmf2_dn13 = assign21160_e16112_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign21170_e16126, assign21170_e16126_d_n0, assign21170_e16126_d_n2, assign21170_e16126_d_n4, assign21170_e16126_d_n5, assign21170_e16126_d_n6, assign21170_e16126_d_n7, assign21170_e16126_d_n8, assign21170_e16126_d_n9, assign21170_e16126_d_n10, assign21170_e16126_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard416 != 0.0)) {
        let (assign21170_e16124, assign21170_e16124_d_n0, assign21170_e16124_d_n2, assign21170_e16124_d_n4, assign21170_e16124_d_n5, assign21170_e16124_d_n6, assign21170_e16124_d_n7, assign21170_e16124_d_n8, assign21170_e16124_d_n9, assign21170_e16124_d_n10, assign21170_e16124_d_n13,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
            } else {
                let assign21170_e16123: f64 = (-locals.var_tmf2);
                (assign21170_e16123, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn13),)
            }
        };
        (assign21170_e16124, assign21170_e16124_d_n0, assign21170_e16124_d_n2, assign21170_e16124_d_n4, assign21170_e16124_d_n5, assign21170_e16124_d_n6, assign21170_e16124_d_n7, assign21170_e16124_d_n8, assign21170_e16124_d_n9, assign21170_e16124_d_n10, assign21170_e16124_d_n13,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign21170_e16126;
        locals.var_tmf2_dn0 = assign21170_e16126_d_n0;
        locals.var_tmf2_dn2 = assign21170_e16126_d_n2;
        locals.var_tmf2_dn4 = assign21170_e16126_d_n4;
        locals.var_tmf2_dn5 = assign21170_e16126_d_n5;
        locals.var_tmf2_dn6 = assign21170_e16126_d_n6;
        locals.var_tmf2_dn7 = assign21170_e16126_d_n7;
        locals.var_tmf2_dn8 = assign21170_e16126_d_n8;
        locals.var_tmf2_dn9 = assign21170_e16126_d_n9;
        locals.var_tmf2_dn10 = assign21170_e16126_d_n10;
        locals.var_tmf2_dn13 = assign21170_e16126_d_n13;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_53(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign21180_e16139, assign21180_e16139_d_n0, assign21180_e16139_d_n2, assign21180_e16139_d_n4, assign21180_e16139_d_n5, assign21180_e16139_d_n6, assign21180_e16139_d_n7, assign21180_e16139_d_n8, assign21180_e16139_d_n9, assign21180_e16139_d_n10, assign21180_e16139_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard416 != 0.0)) {
        let assign21180_e16134: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign21180_e16136: f64 = (assign21180_e16134 + locals.var_tmf2);
        let assign21180_e16137: f64 = (assign21180_e16136).sqrt();
        (assign21180_e16137, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign21180_e16137)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign21180_e16137)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign21180_e16137)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign21180_e16137)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign21180_e16137)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign21180_e16137)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign21180_e16137)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign21180_e16137)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign21180_e16137)), ((((locals.var_tmf1_dn13 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn13)) + locals.var_tmf2_dn13) / (2.0 * assign21180_e16137)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign21180_e16139;
        locals.var_tmf2_dn0 = assign21180_e16139_d_n0;
        locals.var_tmf2_dn2 = assign21180_e16139_d_n2;
        locals.var_tmf2_dn4 = assign21180_e16139_d_n4;
        locals.var_tmf2_dn5 = assign21180_e16139_d_n5;
        locals.var_tmf2_dn6 = assign21180_e16139_d_n6;
        locals.var_tmf2_dn7 = assign21180_e16139_d_n7;
        locals.var_tmf2_dn8 = assign21180_e16139_d_n8;
        locals.var_tmf2_dn9 = assign21180_e16139_d_n9;
        locals.var_tmf2_dn10 = assign21180_e16139_d_n10;
        locals.var_tmf2_dn13 = assign21180_e16139_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign21190_e16153, assign21190_e16153_d_n0, assign21190_e16153_d_n2, assign21190_e16153_d_n4, assign21190_e16153_d_n5, assign21190_e16153_d_n6, assign21190_e16153_d_n7, assign21190_e16153_d_n8, assign21190_e16153_d_n9, assign21190_e16153_d_n10, assign21190_e16153_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard416 != 0.0)) {
        let assign21190_e16149: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign21190_e16150: f64 = (1.0 + assign21190_e16149);
        let assign21190_e16151: f64 = (0.5 * assign21190_e16150);
        (assign21190_e16151, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn13 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign21190_e16153;
        locals.var_t0_dn0 = assign21190_e16153_d_n0;
        locals.var_t0_dn2 = assign21190_e16153_d_n2;
        locals.var_t0_dn4 = assign21190_e16153_d_n4;
        locals.var_t0_dn5 = assign21190_e16153_d_n5;
        locals.var_t0_dn6 = assign21190_e16153_d_n6;
        locals.var_t0_dn7 = assign21190_e16153_d_n7;
        locals.var_t0_dn8 = assign21190_e16153_d_n8;
        locals.var_t0_dn9 = assign21190_e16153_d_n9;
        locals.var_t0_dn10 = assign21190_e16153_d_n10;
        locals.var_t0_dn13 = assign21190_e16153_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign21200_e16173, assign21200_e16173_d_n0, assign21200_e16173_d_n2, assign21200_e16173_d_n4, assign21200_e16173_d_n5, assign21200_e16173_d_n6, assign21200_e16173_d_n7, assign21200_e16173_d_n8, assign21200_e16173_d_n9, assign21200_e16173_d_n10, assign21200_e16173_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard416 != 0.0)) {
        let assign21200_e16164: f64 = (2.0 * 5e-5);
        let assign21200_e16166: f64 = (assign21200_e16164 * 0.01);
        let assign21200_e16167: f64 = (locals.var_tmf1 + assign21200_e16166);
        let assign21200_e16169: f64 = (assign21200_e16167 / locals.var_tmf2);
        let assign21200_e16170: f64 = (1.0 - assign21200_e16169);
        let assign21200_e16171: f64 = (0.5 * assign21200_e16170);
        (assign21200_e16171, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign21200_e16167 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign21200_e16167 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn4 * locals.var_tmf2) - (assign21200_e16167 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn5 * locals.var_tmf2) - (assign21200_e16167 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign21200_e16167 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign21200_e16167 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn8 * locals.var_tmf2) - (assign21200_e16167 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn9 * locals.var_tmf2) - (assign21200_e16167 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign21200_e16167 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn13 * locals.var_tmf2) - (assign21200_e16167 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn13,)
    }
};
        locals.var_t5 = assign21200_e16173;
        locals.var_t5_dn0 = assign21200_e16173_d_n0;
        locals.var_t5_dn2 = assign21200_e16173_d_n2;
        locals.var_t5_dn4 = assign21200_e16173_d_n4;
        locals.var_t5_dn5 = assign21200_e16173_d_n5;
        locals.var_t5_dn6 = assign21200_e16173_d_n6;
        locals.var_t5_dn7 = assign21200_e16173_d_n7;
        locals.var_t5_dn8 = assign21200_e16173_d_n8;
        locals.var_t5_dn9 = assign21200_e16173_d_n9;
        locals.var_t5_dn10 = assign21200_e16173_d_n10;
        locals.var_t5_dn13 = assign21200_e16173_d_n13;
        locals.var_t5_rv = 0.0;

        let (assign21210_e16187, assign21210_e16187_d_n0, assign21210_e16187_d_n2, assign21210_e16187_d_n4, assign21210_e16187_d_n5, assign21210_e16187_d_n6, assign21210_e16187_d_n7, assign21210_e16187_d_n8, assign21210_e16187_d_n9, assign21210_e16187_d_n10, assign21210_e16187_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard416 != 0.0)) {
        let assign21210_e16183: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign21210_e16184: f64 = (0.5 * assign21210_e16183);
        let assign21210_e16185: f64 = (locals.var_t3 - assign21210_e16184);
        (assign21210_e16185, (locals.var_t3_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t3_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t3_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t3_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t3_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t3_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t3_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t3_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t3_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t3_dn13 - (0.5 * (locals.var_tmf1_dn13 + locals.var_tmf2_dn13))),)
    } else {
        (locals.var_rsdrift, locals.var_rsdrift_dn0, locals.var_rsdrift_dn2, locals.var_rsdrift_dn4, locals.var_rsdrift_dn5, locals.var_rsdrift_dn6, locals.var_rsdrift_dn7, locals.var_rsdrift_dn8, locals.var_rsdrift_dn9, locals.var_rsdrift_dn10, locals.var_rsdrift_dn13,)
    }
};
        locals.var_rsdrift = assign21210_e16187;
        locals.var_rsdrift_dn0 = assign21210_e16187_d_n0;
        locals.var_rsdrift_dn2 = assign21210_e16187_d_n2;
        locals.var_rsdrift_dn4 = assign21210_e16187_d_n4;
        locals.var_rsdrift_dn5 = assign21210_e16187_d_n5;
        locals.var_rsdrift_dn6 = assign21210_e16187_d_n6;
        locals.var_rsdrift_dn7 = assign21210_e16187_d_n7;
        locals.var_rsdrift_dn8 = assign21210_e16187_d_n8;
        locals.var_rsdrift_dn9 = assign21210_e16187_d_n9;
        locals.var_rsdrift_dn10 = assign21210_e16187_d_n10;
        locals.var_rsdrift_dn13 = assign21210_e16187_d_n13;
        locals.var_rsdrift_rv = 0.0;

        let (assign21220_e16199, assign21220_e16199_d_n0, assign21220_e16199_d_n2, assign21220_e16199_d_n4, assign21220_e16199_d_n5, assign21220_e16199_d_n6, assign21220_e16199_d_n7, assign21220_e16199_d_n8, assign21220_e16199_d_n9, assign21220_e16199_d_n10, assign21220_e16199_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard416 != 0.0)) {
        let assign21220_e16196: f64 = (locals.var_uc_rdvb * locals.var_vbserevz);
        let assign21220_e16197: f64 = (1.0 - assign21220_e16196);
        (assign21220_e16197, (-(locals.var_uc_rdvb * locals.var_vbserevz_dn0)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn2)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn4)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn5)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn6)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn7)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn8)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn9)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn10)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign21220_e16199;
        locals.var_t1_dn0 = assign21220_e16199_d_n0;
        locals.var_t1_dn2 = assign21220_e16199_d_n2;
        locals.var_t1_dn4 = assign21220_e16199_d_n4;
        locals.var_t1_dn5 = assign21220_e16199_d_n5;
        locals.var_t1_dn6 = assign21220_e16199_d_n6;
        locals.var_t1_dn7 = assign21220_e16199_d_n7;
        locals.var_t1_dn8 = assign21220_e16199_d_n8;
        locals.var_t1_dn9 = assign21220_e16199_d_n9;
        locals.var_t1_dn10 = assign21220_e16199_d_n10;
        locals.var_t1_dn13 = assign21220_e16199_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign21230_e16220, assign21230_e16220_d_n0, assign21230_e16220_d_n2, assign21230_e16220_d_n4, assign21230_e16220_d_n5, assign21230_e16220_d_n6, assign21230_e16220_d_n7, assign21230_e16220_d_n8, assign21230_e16220_d_n9, assign21230_e16220_d_n10, assign21230_e16220_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard416 != 0.0)) {
        let assign21230_e16207: f64 = (locals.var_t1 * locals.var_t1);
        let assign21230_e16211: f64 = (0.0001 * 0.01);
        let assign21230_e16212: f64 = (4.0 * assign21230_e16211);
        let assign21230_e16215: f64 = (0.0001 * 0.01);
        let assign21230_e16216: f64 = (assign21230_e16212 * assign21230_e16215);
        let assign21230_e16217: f64 = (assign21230_e16207 + assign21230_e16216);
        let assign21230_e16218: f64 = (assign21230_e16217).sqrt();
        (assign21230_e16218, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign21230_e16218)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign21230_e16218)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign21230_e16218)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign21230_e16218)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign21230_e16218)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign21230_e16218)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign21230_e16218)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign21230_e16218)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign21230_e16218)), (((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) / (2.0 * assign21230_e16218)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign21230_e16220;
        locals.var_tmf2_dn0 = assign21230_e16220_d_n0;
        locals.var_tmf2_dn2 = assign21230_e16220_d_n2;
        locals.var_tmf2_dn4 = assign21230_e16220_d_n4;
        locals.var_tmf2_dn5 = assign21230_e16220_d_n5;
        locals.var_tmf2_dn6 = assign21230_e16220_d_n6;
        locals.var_tmf2_dn7 = assign21230_e16220_d_n7;
        locals.var_tmf2_dn8 = assign21230_e16220_d_n8;
        locals.var_tmf2_dn9 = assign21230_e16220_d_n9;
        locals.var_tmf2_dn10 = assign21230_e16220_d_n10;
        locals.var_tmf2_dn13 = assign21230_e16220_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign21240_e16234, assign21240_e16234_d_n0, assign21240_e16234_d_n2, assign21240_e16234_d_n4, assign21240_e16234_d_n5, assign21240_e16234_d_n6, assign21240_e16234_d_n7, assign21240_e16234_d_n8, assign21240_e16234_d_n9, assign21240_e16234_d_n10, assign21240_e16234_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard416 != 0.0)) {
        let assign21240_e16230: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign21240_e16231: f64 = (1.0 + assign21240_e16230);
        let assign21240_e16232: f64 = (0.5 * assign21240_e16231);
        (assign21240_e16232, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn7 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn9 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn13 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign21240_e16234;
        locals.var_t4_dn0 = assign21240_e16234_d_n0;
        locals.var_t4_dn2 = assign21240_e16234_d_n2;
        locals.var_t4_dn4 = assign21240_e16234_d_n4;
        locals.var_t4_dn5 = assign21240_e16234_d_n5;
        locals.var_t4_dn6 = assign21240_e16234_d_n6;
        locals.var_t4_dn7 = assign21240_e16234_d_n7;
        locals.var_t4_dn8 = assign21240_e16234_d_n8;
        locals.var_t4_dn9 = assign21240_e16234_d_n9;
        locals.var_t4_dn10 = assign21240_e16234_d_n10;
        locals.var_t4_dn13 = assign21240_e16234_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign21250_e16246, assign21250_e16246_d_n0, assign21250_e16246_d_n2, assign21250_e16246_d_n4, assign21250_e16246_d_n5, assign21250_e16246_d_n6, assign21250_e16246_d_n7, assign21250_e16246_d_n8, assign21250_e16246_d_n9, assign21250_e16246_d_n10, assign21250_e16246_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard416 != 0.0)) {
        let assign21250_e16243: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign21250_e16244: f64 = (0.5 * assign21250_e16243);
        (assign21250_e16244, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign21250_e16246;
        locals.var_t3_dn0 = assign21250_e16246_d_n0;
        locals.var_t3_dn2 = assign21250_e16246_d_n2;
        locals.var_t3_dn4 = assign21250_e16246_d_n4;
        locals.var_t3_dn5 = assign21250_e16246_d_n5;
        locals.var_t3_dn6 = assign21250_e16246_d_n6;
        locals.var_t3_dn7 = assign21250_e16246_d_n7;
        locals.var_t3_dn8 = assign21250_e16246_d_n8;
        locals.var_t3_dn9 = assign21250_e16246_d_n9;
        locals.var_t3_dn10 = assign21250_e16246_d_n10;
        locals.var_t3_dn13 = assign21250_e16246_d_n13;
        locals.var_t3_rv = 0.0;

        let assign21260_e16249: f64 = if locals.var_t3 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard417 = assign21260_e16249;
        locals.var_guard417_rv = 0.0;

        let (assign21270_e16259, assign21270_e16259_d_n0, assign21270_e16259_d_n2, assign21270_e16259_d_n4, assign21270_e16259_d_n5, assign21270_e16259_d_n6, assign21270_e16259_d_n7, assign21270_e16259_d_n8, assign21270_e16259_d_n9, assign21270_e16259_d_n10, assign21270_e16259_d_n13,) = {
    if ((((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard416 != 0.0)) && (locals.var_guard417 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign21270_e16259;
        locals.var_t3_dn0 = assign21270_e16259_d_n0;
        locals.var_t3_dn2 = assign21270_e16259_d_n2;
        locals.var_t3_dn4 = assign21270_e16259_d_n4;
        locals.var_t3_dn5 = assign21270_e16259_d_n5;
        locals.var_t3_dn6 = assign21270_e16259_d_n6;
        locals.var_t3_dn7 = assign21270_e16259_d_n7;
        locals.var_t3_dn8 = assign21270_e16259_d_n8;
        locals.var_t3_dn9 = assign21270_e16259_d_n9;
        locals.var_t3_dn10 = assign21270_e16259_d_n10;
        locals.var_t3_dn13 = assign21270_e16259_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign21280_e16269, assign21280_e16269_d_n0, assign21280_e16269_d_n2, assign21280_e16269_d_n4, assign21280_e16269_d_n5, assign21280_e16269_d_n6, assign21280_e16269_d_n7, assign21280_e16269_d_n8, assign21280_e16269_d_n9, assign21280_e16269_d_n10, assign21280_e16269_d_n13,) = {
    if ((((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard416 != 0.0)) && (locals.var_guard417 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn13,)
    }
};
        locals.var_t4 = assign21280_e16269;
        locals.var_t4_dn0 = assign21280_e16269_d_n0;
        locals.var_t4_dn2 = assign21280_e16269_d_n2;
        locals.var_t4_dn4 = assign21280_e16269_d_n4;
        locals.var_t4_dn5 = assign21280_e16269_d_n5;
        locals.var_t4_dn6 = assign21280_e16269_d_n6;
        locals.var_t4_dn7 = assign21280_e16269_d_n7;
        locals.var_t4_dn8 = assign21280_e16269_d_n8;
        locals.var_t4_dn9 = assign21280_e16269_d_n9;
        locals.var_t4_dn10 = assign21280_e16269_d_n10;
        locals.var_t4_dn13 = assign21280_e16269_d_n13;
        locals.var_t4_rv = 0.0;

        let (assign21290_e16279, assign21290_e16279_d_n0, assign21290_e16279_d_n2, assign21290_e16279_d_n4, assign21290_e16279_d_n5, assign21290_e16279_d_n6, assign21290_e16279_d_n7, assign21290_e16279_d_n8, assign21290_e16279_d_n9, assign21290_e16279_d_n10, assign21290_e16279_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard416 != 0.0)) {
        let assign21290_e16277: f64 = (locals.var_t3 + 1e-25);
        (assign21290_e16277, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign21290_e16279;
        locals.var_t3_dn0 = assign21290_e16279_d_n0;
        locals.var_t3_dn2 = assign21290_e16279_d_n2;
        locals.var_t3_dn4 = assign21290_e16279_d_n4;
        locals.var_t3_dn5 = assign21290_e16279_d_n5;
        locals.var_t3_dn6 = assign21290_e16279_d_n6;
        locals.var_t3_dn7 = assign21290_e16279_d_n7;
        locals.var_t3_dn8 = assign21290_e16279_d_n8;
        locals.var_t3_dn9 = assign21290_e16279_d_n9;
        locals.var_t3_dn10 = assign21290_e16279_d_n10;
        locals.var_t3_dn13 = assign21290_e16279_d_n13;
        locals.var_t3_rv = 0.0;

        let (assign21300_e16287, assign21300_e16287_d_n0, assign21300_e16287_d_n2, assign21300_e16287_d_n4, assign21300_e16287_d_n5, assign21300_e16287_d_n6, assign21300_e16287_d_n7, assign21300_e16287_d_n8, assign21300_e16287_d_n9, assign21300_e16287_d_n10, assign21300_e16287_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard416 != 0.0)) {
        (locals.var_rsdrift, locals.var_rsdrift_dn0, locals.var_rsdrift_dn2, locals.var_rsdrift_dn4, locals.var_rsdrift_dn5, locals.var_rsdrift_dn6, locals.var_rsdrift_dn7, locals.var_rsdrift_dn8, locals.var_rsdrift_dn9, locals.var_rsdrift_dn10, locals.var_rsdrift_dn13,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign21300_e16287;
        locals.var_t0_dn0 = assign21300_e16287_d_n0;
        locals.var_t0_dn2 = assign21300_e16287_d_n2;
        locals.var_t0_dn4 = assign21300_e16287_d_n4;
        locals.var_t0_dn5 = assign21300_e16287_d_n5;
        locals.var_t0_dn6 = assign21300_e16287_d_n6;
        locals.var_t0_dn7 = assign21300_e16287_d_n7;
        locals.var_t0_dn8 = assign21300_e16287_d_n8;
        locals.var_t0_dn9 = assign21300_e16287_d_n9;
        locals.var_t0_dn10 = assign21300_e16287_d_n10;
        locals.var_t0_dn13 = assign21300_e16287_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign21310_e16297, assign21310_e16297_d_n0, assign21310_e16297_d_n2, assign21310_e16297_d_n4, assign21310_e16297_d_n5, assign21310_e16297_d_n6, assign21310_e16297_d_n7, assign21310_e16297_d_n8, assign21310_e16297_d_n9, assign21310_e16297_d_n10, assign21310_e16297_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard416 != 0.0)) {
        let assign21310_e16295: f64 = (locals.var_rsdrift * locals.var_t3);
        (assign21310_e16295, ((locals.var_rsdrift_dn0 * locals.var_t3) + (locals.var_rsdrift * locals.var_t3_dn0)), ((locals.var_rsdrift_dn2 * locals.var_t3) + (locals.var_rsdrift * locals.var_t3_dn2)), ((locals.var_rsdrift_dn4 * locals.var_t3) + (locals.var_rsdrift * locals.var_t3_dn4)), ((locals.var_rsdrift_dn5 * locals.var_t3) + (locals.var_rsdrift * locals.var_t3_dn5)), ((locals.var_rsdrift_dn6 * locals.var_t3) + (locals.var_rsdrift * locals.var_t3_dn6)), ((locals.var_rsdrift_dn7 * locals.var_t3) + (locals.var_rsdrift * locals.var_t3_dn7)), ((locals.var_rsdrift_dn8 * locals.var_t3) + (locals.var_rsdrift * locals.var_t3_dn8)), ((locals.var_rsdrift_dn9 * locals.var_t3) + (locals.var_rsdrift * locals.var_t3_dn9)), ((locals.var_rsdrift_dn10 * locals.var_t3) + (locals.var_rsdrift * locals.var_t3_dn10)), ((locals.var_rsdrift_dn13 * locals.var_t3) + (locals.var_rsdrift * locals.var_t3_dn13)),)
    } else {
        (locals.var_rsdrift, locals.var_rsdrift_dn0, locals.var_rsdrift_dn2, locals.var_rsdrift_dn4, locals.var_rsdrift_dn5, locals.var_rsdrift_dn6, locals.var_rsdrift_dn7, locals.var_rsdrift_dn8, locals.var_rsdrift_dn9, locals.var_rsdrift_dn10, locals.var_rsdrift_dn13,)
    }
};
        locals.var_rsdrift = assign21310_e16297;
        locals.var_rsdrift_dn0 = assign21310_e16297_d_n0;
        locals.var_rsdrift_dn2 = assign21310_e16297_d_n2;
        locals.var_rsdrift_dn4 = assign21310_e16297_d_n4;
        locals.var_rsdrift_dn5 = assign21310_e16297_d_n5;
        locals.var_rsdrift_dn6 = assign21310_e16297_d_n6;
        locals.var_rsdrift_dn7 = assign21310_e16297_d_n7;
        locals.var_rsdrift_dn8 = assign21310_e16297_d_n8;
        locals.var_rsdrift_dn9 = assign21310_e16297_d_n9;
        locals.var_rsdrift_dn10 = assign21310_e16297_d_n10;
        locals.var_rsdrift_dn13 = assign21310_e16297_d_n13;
        locals.var_rsdrift_rv = 0.0;

        let (assign21320_e16306, assign21320_e16306_d_n0, assign21320_e16306_d_n2, assign21320_e16306_d_n4, assign21320_e16306_d_n5, assign21320_e16306_d_n6, assign21320_e16306_d_n7, assign21320_e16306_d_n8, assign21320_e16306_d_n9, assign21320_e16306_d_n10, assign21320_e16306_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard416 == 0.0)) {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn13,)
    } else {
        (locals.var_rsdrift, locals.var_rsdrift_dn0, locals.var_rsdrift_dn2, locals.var_rsdrift_dn4, locals.var_rsdrift_dn5, locals.var_rsdrift_dn6, locals.var_rsdrift_dn7, locals.var_rsdrift_dn8, locals.var_rsdrift_dn9, locals.var_rsdrift_dn10, locals.var_rsdrift_dn13,)
    }
};
        locals.var_rsdrift = assign21320_e16306;
        locals.var_rsdrift_dn0 = assign21320_e16306_d_n0;
        locals.var_rsdrift_dn2 = assign21320_e16306_d_n2;
        locals.var_rsdrift_dn4 = assign21320_e16306_d_n4;
        locals.var_rsdrift_dn5 = assign21320_e16306_d_n5;
        locals.var_rsdrift_dn6 = assign21320_e16306_d_n6;
        locals.var_rsdrift_dn7 = assign21320_e16306_d_n7;
        locals.var_rsdrift_dn8 = assign21320_e16306_d_n8;
        locals.var_rsdrift_dn9 = assign21320_e16306_d_n9;
        locals.var_rsdrift_dn10 = assign21320_e16306_d_n10;
        locals.var_rsdrift_dn13 = assign21320_e16306_d_n13;
        locals.var_rsdrift_rv = 0.0;

        let assign21330_e16317: f64 = (locals.var_mks_nsubsub + locals.var_uc_nover);
        let assign21330_e16318: f64 = (locals.var_uc_nover * assign21330_e16317);
        let assign21330_e16321: f64 = if (((p.p54 == 1.0) && (p.p34 == 0.0)) && (assign21330_e16318 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard418 = assign21330_e16321;
        locals.var_guard418_rv = 0.0;

        let (assign21340_e16337, assign21340_e16337_d_n0, assign21340_e16337_d_n2, assign21340_e16337_d_n4, assign21340_e16337_d_n5, assign21340_e16337_d_n6, assign21340_e16337_d_n7, assign21340_e16337_d_n8, assign21340_e16337_d_n9, assign21340_e16337_d_n10, assign21340_e16337_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard418 != 0.0)) {
        let assign21340_e16330: f64 = (p.p333 * locals.var_vdserevz);
        let assign21340_e16331: f64 = (p.p335 - assign21340_e16330);
        let assign21340_e16334: f64 = (p.p332 * locals.var_vsubsrev);
        let assign21340_e16335: f64 = (assign21340_e16331 - assign21340_e16334);
        (assign21340_e16335, ((-(p.p333 * locals.var_vdserevz_dn0)) - (p.p332 * locals.var_vsubsrev_dn0)), ((-(p.p333 * locals.var_vdserevz_dn2)) - (p.p332 * locals.var_vsubsrev_dn2)), (-(p.p333 * locals.var_vdserevz_dn4)), (-(p.p333 * locals.var_vdserevz_dn5)), (-(p.p333 * locals.var_vdserevz_dn6)), (-(p.p333 * locals.var_vdserevz_dn7)), (-(p.p333 * locals.var_vdserevz_dn8)), (-(p.p333 * locals.var_vdserevz_dn9)), (-(p.p333 * locals.var_vdserevz_dn10)), (-(p.p333 * locals.var_vdserevz_dn13)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign21340_e16337;
        locals.var_t0_dn0 = assign21340_e16337_d_n0;
        locals.var_t0_dn2 = assign21340_e16337_d_n2;
        locals.var_t0_dn4 = assign21340_e16337_d_n4;
        locals.var_t0_dn5 = assign21340_e16337_d_n5;
        locals.var_t0_dn6 = assign21340_e16337_d_n6;
        locals.var_t0_dn7 = assign21340_e16337_d_n7;
        locals.var_t0_dn8 = assign21340_e16337_d_n8;
        locals.var_t0_dn9 = assign21340_e16337_d_n9;
        locals.var_t0_dn10 = assign21340_e16337_d_n10;
        locals.var_t0_dn13 = assign21340_e16337_d_n13;
        locals.var_t0_rv = 0.0;

        let (assign21350_e16354, assign21350_e16354_d_n0, assign21350_e16354_d_n2, assign21350_e16354_d_n4, assign21350_e16354_d_n5, assign21350_e16354_d_n6, assign21350_e16354_d_n7, assign21350_e16354_d_n8, assign21350_e16354_d_n9, assign21350_e16354_d_n10, assign21350_e16354_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard418 != 0.0)) {
        let assign21350_e16345: f64 = (locals.var_t0 * locals.var_t0);
        let assign21350_e16348: f64 = (4.0 * 10.0);
        let assign21350_e16350: f64 = (assign21350_e16348 * 10.0);
        let assign21350_e16351: f64 = (assign21350_e16345 + assign21350_e16350);
        let assign21350_e16352: f64 = (assign21350_e16351).sqrt();
        (assign21350_e16352, (((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)) / (2.0 * assign21350_e16352)), (((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)) / (2.0 * assign21350_e16352)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign21350_e16352)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign21350_e16352)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign21350_e16352)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign21350_e16352)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign21350_e16352)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign21350_e16352)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign21350_e16352)), (((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)) / (2.0 * assign21350_e16352)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn13,)
    }
};
        locals.var_tmf2 = assign21350_e16354;
        locals.var_tmf2_dn0 = assign21350_e16354_d_n0;
        locals.var_tmf2_dn2 = assign21350_e16354_d_n2;
        locals.var_tmf2_dn4 = assign21350_e16354_d_n4;
        locals.var_tmf2_dn5 = assign21350_e16354_d_n5;
        locals.var_tmf2_dn6 = assign21350_e16354_d_n6;
        locals.var_tmf2_dn7 = assign21350_e16354_d_n7;
        locals.var_tmf2_dn8 = assign21350_e16354_d_n8;
        locals.var_tmf2_dn9 = assign21350_e16354_d_n9;
        locals.var_tmf2_dn10 = assign21350_e16354_d_n10;
        locals.var_tmf2_dn13 = assign21350_e16354_d_n13;
        locals.var_tmf2_rv = 0.0;

        let (assign21360_e16368, assign21360_e16368_d_n0, assign21360_e16368_d_n2, assign21360_e16368_d_n4, assign21360_e16368_d_n5, assign21360_e16368_d_n6, assign21360_e16368_d_n7, assign21360_e16368_d_n8, assign21360_e16368_d_n9, assign21360_e16368_d_n10, assign21360_e16368_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard418 != 0.0)) {
        let assign21360_e16364: f64 = (locals.var_t0 / locals.var_tmf2);
        let assign21360_e16365: f64 = (1.0 + assign21360_e16364);
        let assign21360_e16366: f64 = (0.5 * assign21360_e16365);
        (assign21360_e16366, (0.5 * (((locals.var_t0_dn0 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn2 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn4 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn5 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn6 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn7 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn8 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn9 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn10 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn13 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn13)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign21360_e16368;
        locals.var_t2_dn0 = assign21360_e16368_d_n0;
        locals.var_t2_dn2 = assign21360_e16368_d_n2;
        locals.var_t2_dn4 = assign21360_e16368_d_n4;
        locals.var_t2_dn5 = assign21360_e16368_d_n5;
        locals.var_t2_dn6 = assign21360_e16368_d_n6;
        locals.var_t2_dn7 = assign21360_e16368_d_n7;
        locals.var_t2_dn8 = assign21360_e16368_d_n8;
        locals.var_t2_dn9 = assign21360_e16368_d_n9;
        locals.var_t2_dn10 = assign21360_e16368_d_n10;
        locals.var_t2_dn13 = assign21360_e16368_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign21370_e16380, assign21370_e16380_d_n0, assign21370_e16380_d_n2, assign21370_e16380_d_n4, assign21370_e16380_d_n5, assign21370_e16380_d_n6, assign21370_e16380_d_n7, assign21370_e16380_d_n8, assign21370_e16380_d_n9, assign21370_e16380_d_n10, assign21370_e16380_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard418 != 0.0)) {
        let assign21370_e16377: f64 = (locals.var_t0 + locals.var_tmf2);
        let assign21370_e16378: f64 = (0.5 * assign21370_e16377);
        (assign21370_e16378, (0.5 * (locals.var_t0_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t0_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t0_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t0_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t0_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t0_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t0_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t0_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t0_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t0_dn13 + locals.var_tmf2_dn13)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign21370_e16380;
        locals.var_t1_dn0 = assign21370_e16380_d_n0;
        locals.var_t1_dn2 = assign21370_e16380_d_n2;
        locals.var_t1_dn4 = assign21370_e16380_d_n4;
        locals.var_t1_dn5 = assign21370_e16380_d_n5;
        locals.var_t1_dn6 = assign21370_e16380_d_n6;
        locals.var_t1_dn7 = assign21370_e16380_d_n7;
        locals.var_t1_dn8 = assign21370_e16380_d_n8;
        locals.var_t1_dn9 = assign21370_e16380_d_n9;
        locals.var_t1_dn10 = assign21370_e16380_d_n10;
        locals.var_t1_dn13 = assign21370_e16380_d_n13;
        locals.var_t1_rv = 0.0;

        let assign21380_e16383: f64 = if locals.var_t1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard419 = assign21380_e16383;
        locals.var_guard419_rv = 0.0;

        let (assign21390_e16393, assign21390_e16393_d_n0, assign21390_e16393_d_n2, assign21390_e16393_d_n4, assign21390_e16393_d_n5, assign21390_e16393_d_n6, assign21390_e16393_d_n7, assign21390_e16393_d_n8, assign21390_e16393_d_n9, assign21390_e16393_d_n10, assign21390_e16393_d_n13,) = {
    if ((((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard418 != 0.0)) && (locals.var_guard419 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign21390_e16393;
        locals.var_t1_dn0 = assign21390_e16393_d_n0;
        locals.var_t1_dn2 = assign21390_e16393_d_n2;
        locals.var_t1_dn4 = assign21390_e16393_d_n4;
        locals.var_t1_dn5 = assign21390_e16393_d_n5;
        locals.var_t1_dn6 = assign21390_e16393_d_n6;
        locals.var_t1_dn7 = assign21390_e16393_d_n7;
        locals.var_t1_dn8 = assign21390_e16393_d_n8;
        locals.var_t1_dn9 = assign21390_e16393_d_n9;
        locals.var_t1_dn10 = assign21390_e16393_d_n10;
        locals.var_t1_dn13 = assign21390_e16393_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign21400_e16403, assign21400_e16403_d_n0, assign21400_e16403_d_n2, assign21400_e16403_d_n4, assign21400_e16403_d_n5, assign21400_e16403_d_n6, assign21400_e16403_d_n7, assign21400_e16403_d_n8, assign21400_e16403_d_n9, assign21400_e16403_d_n10, assign21400_e16403_d_n13,) = {
    if ((((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard418 != 0.0)) && (locals.var_guard419 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign21400_e16403;
        locals.var_t2_dn0 = assign21400_e16403_d_n0;
        locals.var_t2_dn2 = assign21400_e16403_d_n2;
        locals.var_t2_dn4 = assign21400_e16403_d_n4;
        locals.var_t2_dn5 = assign21400_e16403_d_n5;
        locals.var_t2_dn6 = assign21400_e16403_d_n6;
        locals.var_t2_dn7 = assign21400_e16403_d_n7;
        locals.var_t2_dn8 = assign21400_e16403_d_n8;
        locals.var_t2_dn9 = assign21400_e16403_d_n9;
        locals.var_t2_dn10 = assign21400_e16403_d_n10;
        locals.var_t2_dn13 = assign21400_e16403_d_n13;
        locals.var_t2_rv = 0.0;

        let (assign21410_e16415, assign21410_e16415_d_n0, assign21410_e16415_d_n2, assign21410_e16415_d_n4, assign21410_e16415_d_n5, assign21410_e16415_d_n6, assign21410_e16415_d_n7, assign21410_e16415_d_n8, assign21410_e16415_d_n9, assign21410_e16415_d_n10, assign21410_e16415_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard418 != 0.0)) {
        let assign21410_e16412: f64 = (10.0 * 2.220446049250313e-16);
        let assign21410_e16413: f64 = (locals.var_t1 + assign21410_e16412);
        (assign21410_e16413, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign21410_e16415;
        locals.var_t1_dn0 = assign21410_e16415_d_n0;
        locals.var_t1_dn2 = assign21410_e16415_d_n2;
        locals.var_t1_dn4 = assign21410_e16415_d_n4;
        locals.var_t1_dn5 = assign21410_e16415_d_n5;
        locals.var_t1_dn6 = assign21410_e16415_d_n6;
        locals.var_t1_dn7 = assign21410_e16415_d_n7;
        locals.var_t1_dn8 = assign21410_e16415_d_n8;
        locals.var_t1_dn9 = assign21410_e16415_d_n9;
        locals.var_t1_dn10 = assign21410_e16415_d_n10;
        locals.var_t1_dn13 = assign21410_e16415_d_n13;
        locals.var_t1_rv = 0.0;

        let (assign21420_e16429, assign21420_e16429_d_n0, assign21420_e16429_d_n2, assign21420_e16429_d_n4, assign21420_e16429_d_n5, assign21420_e16429_d_n6, assign21420_e16429_d_n7, assign21420_e16429_d_n8, assign21420_e16429_d_n9, assign21420_e16429_d_n10, assign21420_e16429_d_n13,) = {
    if (((locals.var_guard409 != 0.0) && (locals.var_guard411 != 0.0)) && (locals.var_guard418 != 0.0)) {
        let assign21420_e16425: f64 = (locals.var_mks_nsubsub + locals.var_uc_nover);
        let assign21420_e16426: f64 = (locals.var_uc_nover * assign21420_e16425);
        let assign21420_e16427: f64 = (locals.var_mks_nsubsub / assign21420_e16426);
        (assign21420_e16427, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn13,)
    }
};
        locals.var_t0 = assign21420_e16429;
        locals.var_t0_dn0 = assign21420_e16429_d_n0;
        locals.var_t0_dn2 = assign21420_e16429_d_n2;
        locals.var_t0_dn4 = assign21420_e16429_d_n4;
        locals.var_t0_dn5 = assign21420_e16429_d_n5;
        locals.var_t0_dn6 = assign21420_e16429_d_n6;
        locals.var_t0_dn7 = assign21420_e16429_d_n7;
        locals.var_t0_dn8 = assign21420_e16429_d_n8;
        locals.var_t0_dn9 = assign21420_e16429_d_n9;
        locals.var_t0_dn10 = assign21420_e16429_d_n10;
        locals.var_t0_dn13 = assign21420_e16429_d_n13;
        locals.var_t0_rv = 0.0;

    }
}
